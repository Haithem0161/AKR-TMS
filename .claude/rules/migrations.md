---
paths:
  - "backend/migrations/**"
  - "backend/ctl/src/db*"
  - "backend/crates/db/src/provision.rs"
---

# SQLx Migration Rules

Ported from the digital-banking-system repository (`.claude/rules/migrations.md`, read 2026-10-11) and adapted to
AKR-TMS. Differences from the DBS rule are marked "AKR". The DBS file's templates used `gen_random_uuid()` and a
three-digit file number; both were stale there and are corrected here.

The conventions of `docs/DATA-MODEL.md` section 0 are the source. This file is the checklist for writing a migration.

## File format

- Location: `backend/migrations/`, one shared chain for every binary.
- AKR naming: `NNNN_short_description.sql`, four digits (`0005_identity_principals.sql`). The numbers are assigned in
  phase order; a phase file's section 1 names its files.
- AKR: migrations are applied only by `akr-ctl db migrate`, connected as `akr_migrate` (`make migrate`). No binary
  migrates at start, and no other role can create anything.
- The SQL comes from the phase file's section 1. If the SQL has to differ, the phase file changes in the same change.

## Writing migrations

### MUST follow

- `IF NOT EXISTS` on `CREATE SCHEMA`, `CREATE TABLE` and `CREATE INDEX`; enum types and policies inside a
  `DO $$ BEGIN ... EXCEPTION WHEN duplicate_object THEN NULL; END $$;` block. A migration can be applied twice by hand
  without an error.
- **AKR ids: `id UUID PRIMARY KEY` with NO database default.** Ids are UUIDv7 made by the application
  (`akr_common::ids`). Never `gen_random_uuid()`, never `uuid_generate_v4()`, never a serial.
- **AKR names are schema-qualified**, always: `admin.jobs`, never `jobs`. The schemas are `raw`, `canon`, `feat`,
  `scn`, `alert`, `casework`, `kyc`, `ref`, `rpt`, `assess`, `audit`, `admin`. Nothing is created in `public`.
- **AKR: no PostgreSQL reserved word as an identifier**: no schema, table or column named `case`, `default`, `grant`,
  `limit`, `references`, `window`, `user`, `order` and the like (DR-20; the case schema is `casework`).
- `TIMESTAMPTZ` always, never `TIMESTAMP`. `created_at TIMESTAMPTZ NOT NULL DEFAULT now()` on every table;
  `updated_at` on mutable tables only, maintained by the application.
- `version INT NOT NULL DEFAULT 1` on every mutable aggregate (compared through `If-Match`).
- Money is `NUMERIC(28,6)`, positive, with its currency; never a float.
- Foreign keys are indexed and `ON DELETE RESTRICT`. Nothing cascades in a bank.
- A closed set is a PostgreSQL enum; a set that may grow or belongs to the bank is `TEXT` with a `CHECK` or a
  reference table.
- **AKR: grants are written table by table, in the migration that creates the table.** Never `ALTER DEFAULT
  PRIVILEGES`, never a grant to `PUBLIC`, never `GRANT ALL`. Each role gets exactly the verbs the phase file names.
  No role has UPDATE or DELETE on an `audit` table. A role that reads a table at start (the classification register)
  needs its grant in the same migration, or the binary cannot start.
- **AKR: every table inserts its own classification and retention rows**: one row in
  `admin.column_classification` for every column, and one row in `admin.retention_assignment` for the table, both
  with `ON CONFLICT ... DO NOTHING`. `akr-ctl db check` fails when a table or a column has none.
- **AKR: a personal column marked for encryption** is `BYTEA` holding a sealed envelope, with a keyed-hash twin
  where it must be matched; the classification row says `encrypted = TRUE`.
- **AKR: row-level security** where the phase file says so, with the policy written per role. The table owner
  (`akr_migrate`) bypasses it; no application role owns a table.
- High-volume facts are range-partitioned by month; partitions are made ahead by the `partition_maintenance` job, not
  by `pg_partman`.
- A seed row (`admin.settings`, a reference code) uses `ON CONFLICT ... DO NOTHING`, so applying it again never
  overwrites the bank's value (`admin-managed.md`).

### NEVER do

- NEVER modify a migration that has been applied anywhere outside your own machine, and never one that is in a
  release: SQLx records the checksum. AKR: before the first release, a migration of the phase being built may be
  corrected in place together with the phase file; the developer's database is then repaired or rebuilt.
- NEVER delete a migration file.
- NEVER `DROP TABLE` or `DROP COLUMN` without the owner's explicit approval.
- NEVER `CASCADE`.
- NEVER create a login role or set a password in a migration. Migrations create the NOLOGIN group roles; the login
  roles and their passwords come from `akr-ctl db provision` (`make db`).
- NEVER put bank data, a real name or a real account in a migration. A seed is a product default.

## Templates (AKR)

### New table

```sql
CREATE TABLE IF NOT EXISTS admin.resources (
    id          UUID PRIMARY KEY,                       -- UUIDv7, app-generated
    code        TEXT NOT NULL,
    label       JSONB NOT NULL,                         -- {ar, en}
    status      TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'retired')),
    version     INT NOT NULL DEFAULT 1,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (code)
);

CREATE INDEX IF NOT EXISTS resources_status_idx ON admin.resources (status);

GRANT SELECT, INSERT, UPDATE ON admin.resources TO akr_app;
GRANT SELECT ON admin.resources TO akr_readonly;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', 'resources', c, 'internal'
FROM unnest(ARRAY['id', 'code', 'label', 'status', 'version', 'created_at', 'updated_at']) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note)
VALUES ('admin', 'resources', 'R9', 'retirement', NULL, NULL)
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

### Adding a column

```sql
ALTER TABLE admin.resources
    ADD COLUMN IF NOT EXISTS category_id UUID REFERENCES admin.categories (id) ON DELETE RESTRICT;

CREATE INDEX IF NOT EXISTS resources_category_id_idx ON admin.resources (category_id);

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
VALUES ('admin', 'resources', 'category_id', 'internal')
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;
```

### Adding an enum type

```sql
DO $$ BEGIN
    CREATE TYPE admin.resource_kind AS ENUM ('one', 'two');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
```

Adding a value to an existing enum is `ALTER TYPE ... ADD VALUE IF NOT EXISTS`, in a migration of its own: the new
value cannot be used in the transaction that adds it.

## Running migrations

```bash
make db             # the database and the login roles (akr-ctl db provision)
make migrate        # akr-ctl db migrate, then db provision again (a login role joins roles the migrations create)
make check          # akr-ctl db check: grants, ownership, classification and retention coverage, PUBLIC holds nothing
make prepare        # write backend/.sqlx from the migrated database
make prepare-check  # fail when backend/.sqlx is not what the code needs
```

## After a migration change

1. `make migrate`, then `make migrate` again: the second run applies nothing.
2. `make check` passes.
3. `cargo check`: every `sqlx::query!` and `query_as!` still compiles. Fix the SQL or the struct to match.
4. `make prepare`, and commit `backend/.sqlx` with the migration.
5. `make test`: the database tests apply every migration to a fresh database (`#[sqlx::test(migrations = ...)]`).

## Rollback

SQLx has no automatic rollback. To undo a released migration, write a new one that reverses it. A release's rollback
plan (phase 00 section 4, the bundle) says whether its migrations are reversible, and an irreversible one is called
out before the bank applies it.

## Mapping SQL types to Rust types

| PostgreSQL type | Rust type | Notes |
|----------------|-----------|-------|
| `UUID` | `uuid::Uuid` | |
| `TEXT` | `String` | |
| `INTEGER` | `i32` | |
| `BIGINT` | `i64` | |
| `BOOLEAN` | `bool` | |
| `TIMESTAMPTZ` | `chrono::DateTime<Utc>` | |
| `DATE` | `chrono::NaiveDate` | |
| `JSONB` | `serde_json::Value` or `sqlx::types::Json<T>` | |
| `NUMERIC` | `rust_decimal::Decimal` | never `f64` for money |
| `BYTEA` | `Vec<u8>` | sealed envelopes, hashes |
| Nullable column | `Option<T>` | MUST be `Option` in the struct |
| Enum type | a Rust enum with `#[derive(sqlx::Type)]` and `#[sqlx(type_name = "...")]` | cast in the query: `status AS "status: JobStatus"` |

## `RESTRICT` cannot be deferred (DBS decision 66)

PostgreSQL checks `ON DELETE RESTRICT` inside the statement, and `SET CONSTRAINTS ... DEFERRED` does not reach it
even when the constraint is declared `DEFERRABLE`. Only `NO ACTION` defers. They refuse the same things; the
difference is only when they look. Where a legitimate delete-and-reinsert must happen inside one transaction, the
referencing key is `ON DELETE NO ACTION DEFERRABLE INITIALLY IMMEDIATE`, and the one code path that needs it sets it
deferred for its own transaction. Adding such a key is a deliberate decision written in the phase file, not a
convenience.
