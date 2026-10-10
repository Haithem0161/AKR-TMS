# Phase 00: Platform Foundation

**Goal:** A deployable workspace (four application binaries and two support binaries over shared crates) with the
profile S compose stack and its network zones, the schemas and database roles, config, the problem+json catalogue,
correlation and idempotency middleware, the PostgreSQL jobs, outbox and timers harness, settings and flags, column
classification and retention assignment, the crypto and object-store ports, the OpenAPI shell, CI with the signed
release bundle, and the DBS rules ported: the chassis every later phase bolts onto.

**Dependencies:** None
**Macro-phase:** P0
**Complexity:** L (no business routes; the size is the platform: compose stack, harness, CI and bundle)
**Binaries touched:** akr-api | akr-worker | akr-ingestd | akr-assess (all four as stubs with health, readiness and
metrics); support binaries `akr-s3guard` and `akr-ctl` (new, DR-24)
**Requirements carried:** TS-01, TS-02, TS-03, TS-09, TS-10 (harness, cockpit counters), TS-12 (schemas and roles;
partitions from phase 02), TS-13, TS-22 (buckets and the S3 port; evidence writes in phase 02), TS-23, TS-24, TS-27,
TS-28 (platform metrics), TS-29 (compose half; Helm in phase 30), TS-30, TS-31, TS-37, TS-38, TS-39 (tooling; the
audit record from phase 02), TS-40, TS-41, TS-42, TS-43, SP-04, SP-05, SP-06, SP-08, SP-09, SP-11, SP-12, SP-13,
SP-66, SP-70, SP-72, SP-73, SP-74, SP-75, SP-84, VO-07
- Gap pass 1 (coverage), implemented here: C-33 (the CBS egress rule: only akr-ingestd reaches the CBS listeners, host-enforced).
- Gap pass 1 (coverage), partly implemented: SP-10 (field encryption by class and keyed match hashes), SP-45 (the column classification registry and its coverage test); VA-21 (profile S: one server plus a backup, one runtime and one scheduler); VB-25 (the profile S compose stack on premises, no big-data cluster), VB-28 (SBOMs with maintainer and country); VC-36 (the licence allow-list in cargo deny).
- Gap pass 1 (coverage), not carried: SP-69 (vendor development practice, DR-396), SP-102 (the bank privacy office's records, DR-396).

## 1. SQLx Migration Changes

Four migrations, `backend/migrations/0001` to `0004` (canonical chain in roadmap.md, DR-23). All objects are
schema-qualified; no table has a database default for `id` (UUIDv7 generated in the application, DR-06); no
identifier is a PostgreSQL reserved word (DR-20). Every table created here inserts its own rows into
`admin.column_classification` and `admin.retention_assignment` (0004), the pattern every later migration follows.

**Provisioning before the first migration** (`akr-ctl db provision`, not a migration, because roles, passwords and
role settings are cluster-level, DR-22): create the database with `ENCODING 'UTF8'`, `LOCALE_PROVIDER builtin`,
`BUILTIN_LOCALE 'C.UTF-8'` (code-point order, stable across ICU and glibc upgrades, so no index is ever corrupted by
an OS update, TS-13, DR-26); create `akr_migrate` with LOGIN (password from OpenBao) owning the database and the `admin`
schema; `ALTER ROLE akr_migrate SET search_path = admin` so `_sqlx_migrations` lives in `admin`; after the
migrations, grant LOGIN with OpenBao-sourced passwords to the application roles and set their role settings
(`statement_timeout`, `idle_in_transaction_session_timeout`; `default_transaction_read_only = on` for
`akr_readonly`). The installer then runs the Arabic round-trip check (insert and read back a fixed Arabic, Kurdish
and Persian string set through each role; sort the fixture with `akr_ar` and compare with the expected order).

```sql
-- migration 0001_platform_schemas_roles.sql
-- Phase 00. Schemas, NOLOGIN roles and schema usage. No table grants here: every table is granted in the
-- migration that creates it (no ALTER DEFAULT PRIVILEGES, DATA-MODEL.md section 21).
-- Roles: EXCEPTION WHEN duplicate_object, not IF NOT EXISTS: two concurrent migrations (sqlx::test gives every
-- test its own database) can both pass an existence check. LOGIN, passwords and role settings are provisioning
-- (DR-22; DBS migrations 0019/0020 lesson: concurrent ALTER ROLE fails with "tuple concurrently updated").

DO $$ BEGIN CREATE ROLE akr_app NOLOGIN;          EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE ROLE akr_ingest NOLOGIN;       EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE ROLE akr_assess NOLOGIN;       EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE ROLE akr_audit_writer NOLOGIN; EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE ROLE akr_readonly NOLOGIN;     EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE SCHEMA IF NOT EXISTS admin;
CREATE SCHEMA IF NOT EXISTS audit;
CREATE SCHEMA IF NOT EXISTS raw;
CREATE SCHEMA IF NOT EXISTS canon;
CREATE SCHEMA IF NOT EXISTS feat;
CREATE SCHEMA IF NOT EXISTS scn;
CREATE SCHEMA IF NOT EXISTS alert;
CREATE SCHEMA IF NOT EXISTS casework;   -- not `case`: CASE is reserved (DR-20)
CREATE SCHEMA IF NOT EXISTS kyc;
CREATE SCHEMA IF NOT EXISTS ref;
CREATE SCHEMA IF NOT EXISTS rpt;
CREATE SCHEMA IF NOT EXISTS assess;

REVOKE ALL ON SCHEMA public FROM PUBLIC;

GRANT USAGE ON SCHEMA admin, ref, feat, scn, alert, casework, kyc, rpt, canon TO akr_app;
GRANT USAGE ON SCHEMA admin, raw, canon                                      TO akr_ingest;
GRANT USAGE ON SCHEMA admin, assess, ref, feat                               TO akr_assess;
GRANT USAGE ON SCHEMA audit                                                  TO akr_audit_writer;
GRANT USAGE ON SCHEMA admin, audit, casework, alert, rpt, kyc, scn, ref      TO akr_readonly;

-- Readiness compares the embedded migrator with the applied chain, so every service role reads the ledger.
-- Guarded: under sqlx::test the ledger is created in the test database's default schema instead.
DO $$ BEGIN
    IF to_regclass('admin._sqlx_migrations') IS NOT NULL THEN
        GRANT SELECT ON admin._sqlx_migrations TO akr_app, akr_ingest, akr_assess;
    END IF;
END $$;

-- Display collations only. No index may use them (ICU version changes then cost nothing); the database default
-- stays builtin C.UTF-8. Used as ORDER BY ... COLLATE akr_ar on name lists (UX parity test with Intl.Collator,
-- research.md RQ-10).
CREATE COLLATION IF NOT EXISTS admin.akr_ar (provider = icu, locale = 'ar-u-kn-true');
CREATE COLLATION IF NOT EXISTS admin.akr_en (provider = icu, locale = 'en-u-kn-true');
```

```sql
-- migration 0002_platform_jobs_outbox_timers.sql
-- Phase 00. The DBS harness: durable jobs (FOR UPDATE SKIP LOCKED), the transactional outbox and timers.
-- Enqueued ONLY inside the transaction that caused them (.claude/rules/jobs.md golden rule).
-- Payloads carry ids and codes only: never a name, a reason or free text (tipping-off, section 3).

DO $$ BEGIN
    CREATE TYPE admin.job_status AS ENUM ('queued', 'running', 'done', 'failed', 'dead');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS admin.jobs (
    id            UUID PRIMARY KEY,                       -- UUIDv7, app-generated
    kind          TEXT NOT NULL CHECK (kind ~ '^[a-z][a-z0-9_]*$'),
    payload       JSONB NOT NULL DEFAULT '{}'::jsonb,
    status        admin.job_status NOT NULL DEFAULT 'queued',
    run_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    attempts      INT NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    max_attempts  INT NOT NULL DEFAULT 5 CHECK (max_attempts > 0),
    last_error    TEXT,                                   -- redacted (section 3, telemetry)
    locked_at     TIMESTAMPTZ,
    locked_by     TEXT,                                   -- worker instance id
    coalesce_key  TEXT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    finished_at   TIMESTAMPTZ,
    CHECK ((status IN ('done', 'dead')) = (finished_at IS NOT NULL))
);
-- The poll: WHERE status = 'queued' AND run_at <= now() ORDER BY run_at.
CREATE INDEX IF NOT EXISTS idx_jobs_poll        ON admin.jobs (status, run_at);
CREATE INDEX IF NOT EXISTS idx_jobs_kind_status ON admin.jobs (kind, status);
CREATE INDEX IF NOT EXISTS idx_jobs_finished    ON admin.jobs (finished_at) WHERE finished_at IS NOT NULL;
-- One queued job per (kind, coalesce_key): repeated enqueues collapse; a job already running does not block the
-- next one (a change after it started needs a fresh run).
CREATE UNIQUE INDEX IF NOT EXISTS uq_jobs_coalesce_queued
    ON admin.jobs (kind, coalesce_key) WHERE status = 'queued' AND coalesce_key IS NOT NULL;

CREATE TABLE IF NOT EXISTS admin.outbox (
    id             UUID PRIMARY KEY,
    topic          TEXT NOT NULL CHECK (topic ~ '^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$'),
    payload        JSONB NOT NULL,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    available_at   TIMESTAMPTZ NOT NULL DEFAULT now(),   -- back-off moves it forward
    dispatched_at  TIMESTAMPTZ,
    attempts       INT NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    last_error     TEXT
);
CREATE INDEX IF NOT EXISTS idx_outbox_pending    ON admin.outbox (available_at) WHERE dispatched_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_outbox_dispatched ON admin.outbox (dispatched_at) WHERE dispatched_at IS NOT NULL;

CREATE TABLE IF NOT EXISTS admin.timers (
    id            UUID PRIMARY KEY,
    owner_kind    TEXT NOT NULL CHECK (owner_kind ~ '^[a-z][a-z0-9_]*$'),
    owner_id      UUID NOT NULL,
    kind          TEXT NOT NULL CHECK (kind ~ '^[a-z][a-z0-9_]*$'),
    fire_at       TIMESTAMPTZ NOT NULL,
    calendar_id   UUID,              -- FK to ref.calendars added by phase 09 (deferred FK)
    payload       JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    fired_at      TIMESTAMPTZ,
    cancelled_at  TIMESTAMPTZ,
    CHECK (fired_at IS NULL OR cancelled_at IS NULL)
);
CREATE INDEX IF NOT EXISTS idx_timers_due   ON admin.timers (fire_at) WHERE fired_at IS NULL AND cancelled_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_timers_owner ON admin.timers (owner_kind, owner_id);

GRANT SELECT, INSERT, UPDATE, DELETE ON admin.jobs   TO akr_app;   -- DELETE: maintenance_sweep prunes finished rows
GRANT SELECT, INSERT, UPDATE, DELETE ON admin.outbox TO akr_app;   -- DELETE: dispatched rows only
GRANT SELECT, INSERT, UPDATE, DELETE ON admin.timers TO akr_app;   -- DELETE: fired or cancelled rows only

-- akr-ingestd runs its own job loop over its own kinds and enqueues mapping jobs for akr-worker. Row-level
-- security keeps it to those kinds (zone Z1 hardening, SP-05); the policy applies to akr_ingest only.
GRANT SELECT, INSERT, UPDATE ON admin.jobs TO akr_ingest;
ALTER TABLE admin.jobs ENABLE ROW LEVEL SECURITY;
DO $$ BEGIN
    CREATE POLICY jobs_app_all ON admin.jobs FOR ALL TO akr_app USING (true) WITH CHECK (true);
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    CREATE POLICY jobs_ingest_own ON admin.jobs FOR SELECT TO akr_ingest
        USING (kind LIKE 'ingest\_%' OR kind LIKE 'discovery\_%');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    CREATE POLICY jobs_ingest_update ON admin.jobs FOR UPDATE TO akr_ingest
        USING (kind LIKE 'ingest\_%' OR kind LIKE 'discovery\_%')
        WITH CHECK (kind LIKE 'ingest\_%' OR kind LIKE 'discovery\_%');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    CREATE POLICY jobs_ingest_insert ON admin.jobs FOR INSERT TO akr_ingest
        WITH CHECK (kind LIKE 'ingest\_%' OR kind LIKE 'discovery\_%' OR kind LIKE 'map\_%');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
```

The DELETE grants are used only by `maintenance_sweep` on terminal rows (section 4). The table owner
(`akr_migrate`) bypasses RLS; no application role is the owner.

```sql
-- migration 0003_platform_idempotency_settings_flags.sql
-- Phase 00. Idempotency ledger (DBS migrations 0001, 0003 and 0024 folded in from the start, DR-21), runtime
-- settings and feature flags.

CREATE TABLE IF NOT EXISTS admin.idempotency_keys (
    principal_id          UUID NOT NULL,       -- FK to admin.users added by phase 01 (deferred FK)
    key                   TEXT NOT NULL CHECK (key ~ '^[ -~]{8,255}$'),   -- 8-255 printable ASCII (API.md s.1)
    endpoint              TEXT NOT NULL,       -- method + route template, for the cockpit and the TTL report
    request_hash          BYTEA NOT NULL CHECK (octet_length(request_hash) = 32),   -- raw SHA-256 bytes
    response_status       INT,
    response_content_type TEXT,                -- NULL for an empty body (204)
    response_body         BYTEA,               -- AES-256-GCM sealed under the `idempotency` key family
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at          TIMESTAMPTZ,         -- NULL = in flight
    PRIMARY KEY (principal_id, key),
    CHECK ((completed_at IS NULL) = (response_status IS NULL))
);
CREATE INDEX IF NOT EXISTS idx_idem_created ON admin.idempotency_keys (created_at);

CREATE TABLE IF NOT EXISTS admin.settings (
    key         TEXT PRIMARY KEY CHECK (key ~ '^[a-z][a-z0-9_]*(\.[a-z0-9_]+)+$'),
    value       JSONB NOT NULL,
    value_type  TEXT NOT NULL CHECK (value_type IN ('integer', 'decimal', 'boolean', 'string', 'duration_s',
                                                    'cron', 'timestamp', 'json')),
    floor       JSONB,                 -- regulatory limit {"min": n} | {"max": n} | both, plus "basis"; NULL = none
    governed    BOOLEAN NOT NULL DEFAULT TRUE,   -- TRUE: changes only through a change request (phase 04)
    label       JSONB NOT NULL CHECK (label ? 'ar' AND label ? 'en'),
    updated_by  UUID,                  -- FK to admin.users added by phase 01 (deferred FK); NULL = seeded
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    version     INT NOT NULL DEFAULT 1,
    CHECK (floor IS NULL OR governed)
);

CREATE TABLE IF NOT EXISTS admin.feature_flags (
    key         TEXT PRIMARY KEY CHECK (key ~ '^[a-z][a-z0-9_]*(\.[a-z0-9_]+)+$'),
    enabled     BOOLEAN NOT NULL DEFAULT FALSE,
    scope       JSONB NOT NULL DEFAULT '{}'::jsonb,   -- {"branches": [uuid], "roles": [code]}; {} = everyone
    label       JSONB NOT NULL CHECK (label ? 'ar' AND label ? 'en'),
    updated_by  UUID,                  -- FK to admin.users added by phase 01 (deferred FK)
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    version     INT NOT NULL DEFAULT 1
);

-- The idempotency ledger holds copies of mutation responses: application role only, never akr_readonly
-- (DBS migration 0019 lesson). Settings and flags are read by every service role.
GRANT SELECT, INSERT, UPDATE, DELETE ON admin.idempotency_keys TO akr_app;
GRANT SELECT, UPDATE ON admin.settings, admin.feature_flags TO akr_app;   -- rows are seeded by migrations only
GRANT SELECT ON admin.settings, admin.feature_flags TO akr_ingest;

-- Phase 00 keys. Every key the code reads is declared once in common::settings_registry; a test asserts that the
-- registry and these rows agree. Values are the planning defaults; the bank changes them through A2 (phase 03/04).
INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('locale.default',                 '"ar"',             'string',     NULL, FALSE,
  '{"ar": "اللغة الافتراضية للواجهة", "en": "Default interface language"}'),
 ('bank.timezone',                  '"Asia/Baghdad"',   'string',     NULL, TRUE,
  '{"ar": "المنطقة الزمنية للمصرف", "en": "Bank time zone"}'),
 ('idempotency.ttl_days',           '7',                'integer',    NULL, FALSE,
  '{"ar": "مدة الاحتفاظ بمفاتيح منع التكرار (أيام)", "en": "Idempotency key retention (days)"}'),
 ('idempotency.in_flight_lease_s',  '120',              'duration_s', NULL, FALSE,
  '{"ar": "مهلة الطلب قيد التنفيذ (ثوانٍ)", "en": "In-flight request lease (seconds)"}'),
 ('jobs.lease_s',                   '600',              'duration_s', NULL, FALSE,
  '{"ar": "مهلة حجز المهمة الخلفية (ثوانٍ)", "en": "Background job lease (seconds)"}'),
 ('jobs.done_retention_days',       '30',               'integer',    NULL, FALSE,
  '{"ar": "مدة الاحتفاظ بالمهام المنجزة (أيام)", "en": "Finished job retention (days)"}'),
 ('outbox.retention_days',          '30',               'integer',    NULL, FALSE,
  '{"ar": "مدة الاحتفاظ برسائل الصادر المرسلة (أيام)", "en": "Dispatched outbox retention (days)"}'),
 ('outbox.max_attempts',            '10',               'integer',    NULL, FALSE,
  '{"ar": "الحد الأقصى لمحاولات إرسال رسالة الصادر", "en": "Outbox delivery attempts"}'),
 ('timers.sweep_interval_s',        '30',               'duration_s', NULL, FALSE,
  '{"ar": "فاصل فحص المؤقتات (ثوانٍ)", "en": "Timer sweep interval (seconds)"}'),
 ('timers.terminal_retention_days', '90',               'integer',    NULL, FALSE,
  '{"ar": "مدة الاحتفاظ بالمؤقتات المنتهية (أيام)", "en": "Fired or cancelled timer retention (days)"}'),
 ('schedule.maintenance_sweep',     '"0 30 2 * * *"',   'cron',       NULL, FALSE,
  '{"ar": "موعد الصيانة اليومية", "en": "Daily maintenance time"}'),
 ('schedule.backup_check',          '"0 0 6 * * *"',    'cron',       NULL, FALSE,
  '{"ar": "موعد فحص النسخ الاحتياطي", "en": "Backup check time"}'),
 ('ops.backup_marker_path',         '"/var/lib/akr/backup/heartbeat"', 'string', NULL, TRUE,
  '{"ar": "مسار علامة النسخ الاحتياطي", "en": "Backup heartbeat marker path"}'),
 ('ops.backup_max_age_h',           '26',               'integer',    NULL, TRUE,
  '{"ar": "أقصى عمر مقبول لآخر نسخة احتياطية (ساعات)", "en": "Maximum backup age (hours)"}'),
 ('ops.last_restore_drill_at',      'null',             'timestamp',  NULL, TRUE,
  '{"ar": "تاريخ آخر تمرين استعادة ناجح", "en": "Last successful restore drill"}'),
 ('ops.restore_drill_max_age_days', '31',               'integer',    NULL, TRUE,
  '{"ar": "أقصى مدة بين تمارين الاستعادة (أيام)", "en": "Maximum days between restore drills"}')
ON CONFLICT (key) DO NOTHING;

-- A test fixture for FlagsRepository. NOT the security boundary of the dev-only /_echo route: that route is
-- mounted only when AKR_ENV != production (DBS lesson), so flipping this row in production exposes nothing.
INSERT INTO admin.feature_flags (key, enabled, label) VALUES
 ('dev.echo_route', FALSE, '{"ar": "مسار الاختبار للمطورين", "en": "Developer echo route"}')
ON CONFLICT (key) DO NOTHING;
```

`ops.last_restore_drill_at` is written only by `akr-ctl ops record-restore-drill` (section 3) until the A2 screen
can change governed settings (phase 04); this closes the DBS gap G-OPS-BACKUPDRILL-01 at the start.

```sql
-- migration 0004_platform_classification_retention.sql
-- Phase 00. Column classification (drives masking, field encryption and policy) and retention assignment (read by
-- maintenance_sweep now and the retention engine in phase 23). Every later migration inserts rows for the tables
-- it creates; the coverage tests in section 6 fail the build otherwise.

DO $$ BEGIN
    CREATE TYPE admin.data_classification AS ENUM
        ('public', 'internal', 'personal', 'financial', 'detection-logic', 'secret-str', 'card');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS admin.column_classification (
    schema_name    TEXT NOT NULL,
    table_name     TEXT NOT NULL,
    column_name    TEXT NOT NULL,
    classification admin.data_classification NOT NULL,
    encrypted      BOOLEAN NOT NULL DEFAULT FALSE,   -- `enc`: AES-256-GCM ciphertext under an OpenBao-wrapped DEK
    keyed_hash     BOOLEAN NOT NULL DEFAULT FALSE,   -- `hk`: HMAC-SHA256 twin under the match key
    note           TEXT,
    PRIMARY KEY (schema_name, table_name, column_name),
    CHECK (NOT encrypted OR classification IN ('personal', 'financial', 'secret-str', 'card'))
);

CREATE TABLE IF NOT EXISTS admin.retention_assignment (
    schema_name     TEXT NOT NULL,
    table_name      TEXT NOT NULL,
    retention_class TEXT NOT NULL CHECK (retention_class ~ '^R([1-9]|1[0-3])$'),
    clock           TEXT NOT NULL CHECK (clock IN ('partition', 'age', 'relationship_end', 'closure', 'decision',
                                                   'retirement', 'owner', 'never')),
    clock_column    TEXT,            -- the column the clock reads; NULL for retirement, owner and never
    note            TEXT,
    PRIMARY KEY (schema_name, table_name),
    CHECK ((retention_class = 'R10') = (clock = 'never')),
    CHECK ((clock IN ('partition', 'age', 'relationship_end', 'closure', 'decision')) = (clock_column IS NOT NULL))
);

GRANT SELECT ON admin.column_classification, admin.retention_assignment TO akr_app, akr_ingest, akr_readonly;

-- The one non-internal column of this phase first, then the rest as internal.
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification, encrypted, note)
VALUES ('admin', 'idempotency_keys', 'response_body', 'secret-str', TRUE,
        'Copy of a mutation response; may carry suspicion-tier content')
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', v.t, c, 'internal'
FROM (VALUES
  ('jobs',                  ARRAY['id','kind','payload','status','run_at','attempts','max_attempts','last_error',
                                  'locked_at','locked_by','coalesce_key','created_at','finished_at']),
  ('outbox',                ARRAY['id','topic','payload','created_at','available_at','dispatched_at','attempts',
                                  'last_error']),
  ('timers',                ARRAY['id','owner_kind','owner_id','kind','fire_at','calendar_id','payload',
                                  'created_at','fired_at','cancelled_at']),
  ('idempotency_keys',      ARRAY['principal_id','key','endpoint','request_hash','response_status',
                                  'response_content_type','created_at','completed_at']),
  ('settings',              ARRAY['key','value','value_type','floor','governed','label','updated_by','updated_at',
                                  'version']),
  ('feature_flags',         ARRAY['key','enabled','scope','label','updated_by','updated_at','version']),
  ('column_classification', ARRAY['schema_name','table_name','column_name','classification','encrypted',
                                  'keyed_hash','note']),
  ('retention_assignment',  ARRAY['schema_name','table_name','retention_class','clock','clock_column','note'])
) AS v(t, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'jobs',                  'R12', 'age',        'finished_at',  'Only done and dead rows; jobs.done_retention_days'),
 ('admin', 'outbox',                'R12', 'age',        'dispatched_at','Only dispatched rows; an undispatched row is evidence that someone was not told'),
 ('admin', 'timers',                'R12', 'age',        'fired_at',     'Only fired or cancelled rows (cancelled_at for cancelled); timers.terminal_retention_days'),
 ('admin', 'idempotency_keys',      'R12', 'age',        'created_at',   'idempotency.ttl_days'),
 ('admin', 'settings',              'R9',  'retirement', NULL,           'Current values; change history is in the audit trail (R10)'),
 ('admin', 'feature_flags',         'R9',  'retirement', NULL,           'As settings'),
 ('admin', 'column_classification', 'R9',  'retirement', NULL,           NULL),
 ('admin', 'retention_assignment',  'R9',  'retirement', NULL,           NULL)
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

Deferred FKs: `idempotency_keys.principal_id`, `settings.updated_by` and `feature_flags.updated_by` gain their FK to
`admin.users(id)` in phase 01 (`ALTER TABLE ... ADD CONSTRAINT ... REFERENCES admin.users(id) ON DELETE RESTRICT`
plus an index on each, conventions); `timers.calendar_id` gains its FK to `ref.calendars(id)` in phase 09.

## 2. DDD Implementation (Rust)

**Workspace layout (pinned here; every later phase follows it).** Layers are separated by crate boundaries so the
compiler enforces `ddd.md`: `akr-domain` cannot import axum or sqlx because it does not depend on them.

```
backend/
├── Cargo.toml              # [workspace] resolver 3, edition 2024; members = crates/*, api, worker, ingestd,
│                           #   assess, s3guard, ctl; [workspace.lints] unsafe_code = forbid,
│                           #   unwrap_used / expect_used / todo / dbg_macro = deny in CI (SP-70)
├── rust-toolchain.toml     # exact toolchain version, not "stable" (SP-72, RQ-01)
├── deny.toml               # licence allow-list, advisories, bans (driver wrappers below), sources = crates.io only
├── migrations/             # NNNN_name.sql, shared, applied only by akr-ctl as akr_migrate
├── policies/               # Cedar schema and policies (phase 03)
├── schemas/                # JSON Schemas for DSL documents (phases 04, 10, 15, 16)
├── fixtures/               # synthetic data only (SP-06); bank facts never live here (admin-managed.md)
├── crates/
│   ├── common/     # Config, AppError + problem+json, correlation, idempotency, cursor, If-Match/ETag, data_as_of,
│   │               #   ids (UUIDv7), time (UTC, bank zone), settings_registry, redaction
│   ├── domain/     # entities, value objects, repository and service traits (no infrastructure dependencies)
│   ├── db/         # Pg* repository impls, tx helpers, one pool per database role
│   ├── jobs/       # JobQueue (enqueue takes &mut Tx), worker harness, schedules, OutboxDispatcher, TimerService
│   ├── crypto/     # one trait (SP-13): seal/open AES-256-GCM envelopes, keyed hash, sign/verify;
│   │               #   OpenBao implementation (transit, KV) + in-memory test implementation
│   ├── objectstore/# S3 port: put with lock mode, get, head, presign GET; Silo, SeaweedFS and RustFS behind it
│   ├── telemetry/  # tracing subscriber (JSON), OTLP exporter, Prometheus registry, redaction layer (TS-27)
│   ├── authz/      # Cedar facade: pass-through slot until phase 03
│   ├── audit/      # AuditSink trait + TracingSink stub (PostgreSQL writer in phase 02)
│   ├── dsl/        # document validators (phase 04 onwards)
│   └── sources/    # SourceReader port and CBS adapters (phase 05); only akr-ingestd may depend on it
├── api/            # akr-api: lib + bin; AppState, routers (utoipa-axum OpenApiRouter + routes!), BFF (phase 01)
├── worker/         # akr-worker: lib + bin; job loop, kind registry, schedules, outbox, timers
├── ingestd/        # akr-ingestd: lib + bin; health only until phase 05
├── assess/         # akr-assess: lib + bin; health only until phase 29
├── s3guard/        # akr-s3guard: allow-list proxy in front of the object store (TS-41)
└── ctl/            # akr-ctl: operator CLI (db provision, migrate, bundle verify/import, ops record-restore-drill)
```

- Every binary is **lib + bin** so integration tests drive the real router and loops (a DBS implementation decision); integration tests
  live in `<bin>/tests/`.
- **Driver isolation (TS-02):** `deny.toml` lists `tiberius`, `oracle`, `odbc-api` and `akr-sources` under
  `[[bans.deny]]` with `wrappers = ["akr-sources", "akr-ingestd"]`; `cargo deny check bans` fails the build if any
  other crate depends on a CBS driver. A second check asserts that the dependency trees of `akr-api`, `akr-worker`
  and `akr-assess` do not contain `akr-sources`.
- **Port inventory (closes the phase 00 part of RQ-04):**

| DBS source (read-only) | AKR target | Change |
|-|-|-|
| `crates/common/src/{config,errors,correlation,idempotency,settings,settings_registry}.rs` | `common` | Schemas in SQL; principal-scoped keys from the start; sealed response bodies; `AKR_` env prefix |
| `crates/common/src/jobs.rs`, `worker/src/poll.rs` | `jobs`, `worker` | Per-kind permits kept; lease reaper and scheduler added; outbox and timer loops moved into `jobs` |
| `crates/common/src/vault.rs` | `crypto` | Same HTTP calls against OpenBao (RQ-02); trait boundary and test double |
| `crates/common/src/{objectstore,storage}.rs` | `objectstore` | Requests shaped to pass the S3 guard (no unsigned-trailer uploads, no presigned writes) |
| `crates/common/src/telemetry.rs`, `api/src/telemetry.rs` | `telemetry` | OTLP export implemented (the DBS deferred it) |
| `crates/db/src/{pool,cursor,settings,flags,jobs_admin}.rs` | `db` | One pool per role; signed cursors |
| `api/src/middleware/{correlation,idempotency,metrics}.rs` | `api` | Same order; idempotency exemption list starts empty |

- **Domain entities** (`domain`, derives `Debug, Serialize, Deserialize, FromRow, ToSchema`):
  `Setting { key, value, value_type, floor, governed, label, updated_by, updated_at, version }`,
  `FeatureFlag { key, enabled, scope, label, updated_by, updated_at, version }`,
  `ColumnClassification`, `RetentionAssignment`. Internal to `jobs` (not exposed in this phase): `Job`,
  `OutboxMessage`, `Timer`. Value objects: `SettingFloor { min: Option<Value>, max: Option<Value>, basis: String }`,
  `FlagScope { branches: Vec<Uuid>, roles: Vec<RoleCode> }`, `Classification` (the enum), `RetentionClass` (R1-R13),
  `RetentionClock`.
- **Update structs** (used by phases 03 and 04, defined here): `UpdateSetting { value: serde_json::Value }`,
  `UpdateFlag { enabled: Option<bool>, scope: Option<FlagScope> }`.
- **Repository traits** (`domain::repositories`):
  - `SettingsRepository { get(&self, key) -> Option<Setting>; list(&self) -> Vec<Setting>;
    update(&self, tx, key, value, expected_version, actor) -> Setting }`
  - `FlagsRepository { list(&self) -> Vec<FeatureFlag>; get(&self, key) -> Option<FeatureFlag>;
    update(&self, tx, key, UpdateFlag, expected_version, actor) -> FeatureFlag }`
  - `ClassificationRepository { for_table(&self, schema, table) -> Vec<ColumnClassification>;
    uncovered_columns(&self) -> Vec<(String, String, String)> }`
  - `RetentionAssignmentRepository { list(&self) -> Vec<RetentionAssignment>;
    uncovered_tables(&self) -> Vec<(String, String)> }`
  - In `jobs`: `JobStore { enqueue(tx, NewJob) -> Option<Uuid> (None when coalesced); claim(kinds, n, worker_id);
    complete(id); fail(id, error, backoff); reap_expired(lease) }`, `OutboxStore { append(tx, topic, payload);
    claim(n); mark_dispatched(id); defer(id, error, backoff) }`, `TimerStore { schedule(tx, NewTimer) -> Uuid;
    cancel(tx, id); cancel_for_owner(tx, owner_kind, owner_id, kind); claim_due(n) }`
- **Repository impls** (`db`): `PgSettingsRepository`, `PgFlagsRepository` (`UPDATE ... WHERE key = $1 AND version =
  $2 RETURNING *`; zero rows -> `VersionConflict` with the current version), `PgClassificationRepository` and
  `PgRetentionAssignmentRepository` (`uncovered_*` are anti-joins against `information_schema.columns` and
  `information_schema.tables` for the twelve AKR schemas), `PgJobStore` (claim = `UPDATE admin.jobs SET status =
  'running', locked_at = now(), locked_by = $3, attempts = attempts + 1 WHERE id IN (SELECT id FROM admin.jobs WHERE
  status = 'queued' AND run_at <= now() AND kind = ANY($1) ORDER BY run_at FOR UPDATE SKIP LOCKED LIMIT $2)
  RETURNING *`), `PgOutboxStore`, `PgTimerStore`.
- **utoipa schemas**: `Problem` (RFC 7807 body with `code`, `errors[]`, `correlation_id`), `ValidationError {
  pointer, code, params }`, `Health { status }`, `Readiness { status, checks: [{ name, ok }] }`, `EchoRequest`,
  `EchoResponse` (dev only). The `X-StepUp-Assertion`, cookie and bearer security schemes are declared now so later
  phases only reference them (API.md section 7).
- **Route table** (infrastructure only; no business routes in this phase):

| Method | Path | Scope | Description |
|-|-|-|-|
| GET | `/healthz` | none (infra) | Liveness on every binary: process up; body `{"status":"ok"}` |
| GET | `/readyz` | none (infra) | Readiness on every binary: its own dependencies (below); 200 or 503; names and booleans only |
| GET | `/metrics` | none (infra; management listener only) | Prometheus exposition on a separate port bound to the management network, never routed by the edge |
| POST | `/api/v1/_echo` | none (dev only) | Idempotency test route; mounted only when `AKR_ENV != production`; echoes the body |

  Readiness checks per binary: `akr-api` database (`SELECT 1` through its pool), migrations current, Valkey `PING`;
  `akr-worker` database, migrations current, OpenBao unsealed and its token valid, object store reachable through the
  guard (`HEAD` on `akr-evidence`); `akr-ingestd` database, migrations current; `akr-assess` database, Valkey;
  `akr-s3guard` upstream store reachable. "Migrations current" = the newest version in the binary's embedded
  migrator equals the newest applied in `admin._sqlx_migrations`; behind -> not ready; ahead -> ready with a warning
  log (a rollback inside the bundle's compatibility window).
- **Router**: `api::routes::infra::router() -> OpenApiRouter<AppState>` merged in `build_app`; the `/api/v1` nest
  carries the middleware stack in this order: body limit -> correlation -> trace -> metrics -> (auth slot, phase 01)
  -> (authz slot, phase 03) -> idempotency -> handlers. The same `infra` router (without `/api/v1`) is mounted by
  `worker`, `ingestd`, `assess` and `s3guard`.

## 3. Business Logic

**`Config::from_env()`** (`common::config`), one struct per binary over a shared core:
1. Read `AKR_ENV` (`development | test | staging | production`), `AKR_BIND`, `AKR_METRICS_BIND`, `BAO_ADDR`,
   `BAO_ROLE_ID`, `BAO_SECRET_ID_FILE`, `BAO_CACERT`, `VALKEY_URL`, `S3_ENDPOINT` (the guard), `OTLP_ENDPOINT`
   (optional), `AKR_INSTANCE_ID` (defaults to the host name).
2. Log in to OpenBao with the binary's AppRole, read its database credentials and the keys it needs from KV (below),
   build `DATABASE_URL` with `sslmode=verify-full` (SP-08), then drop the OpenBao token's KV capability for the rest of
   the run (the token stays only for transit and renewal).
3. `validate()` refuses to start in `production` when: the dev principal stub is enabled; `sslmode` is weaker than
   `verify-full`; `BAO_ADDR` is not `https`; `S3_ENDPOINT` points at the store instead of the guard; core dumps are not
   disabled (the process calls `prctl(PR_SET_DUMPABLE, 0)` at start and checks `RLIMIT_CORE = 0`).

**Error catalogue** (`common::errors::AppError` -> problem+json; codes from API.md section 6, which this phase
completes with `E_INTERNAL` and `E_PAYLOAD_TOO_LARGE`):

| Variant | HTTP | `code` |
|-|-|-|
| `BadRequest` | 400 | `E_BAD_REQUEST` |
| `IdempotencyRequired` | 400 | `E_IDEMPOTENCY_REQUIRED` |
| `Unauthenticated` | 401 | `E_UNAUTHENTICATED` |
| `StepUpRequired` | 401 | `E_STEPUP_REQUIRED` |
| `Scope` | 403 | `E_SCOPE` |
| `Forbidden` | 403 | `E_FORBIDDEN` |
| `SelfApproval` | 403 | `E_SELF_APPROVAL` |
| `NotFound` | 404 | `E_NOT_FOUND` (one body for missing and masked, phase 03) |
| `VersionConflict { current }` | 409 | `E_VERSION_CONFLICT` |
| `IdempotencyInFlight` | 409 | `E_IDEMPOTENCY_IN_FLIGHT` |
| `NotAllowedInState` | 409 | `E_NOT_ALLOWED_IN_STATE` |
| `PayloadTooLarge` | 413 | `E_PAYLOAD_TOO_LARGE` |
| `Validation(Vec<ValidationError>)` | 422 | `E_VALIDATION` (`errors[]` with JSON pointers) |
| `IdempotencyReuse` | 422 | `E_IDEMPOTENCY_REUSE` |
| `BelowFloor` | 422 | `E_BELOW_FLOOR` |
| `PreconditionRequired` | 428 | `E_PRECONDITION_REQUIRED` |
| `RateLimited { retry_after }` | 429 | `E_RATE_LIMITED` |
| `Internal` | 500 | `E_INTERNAL` (correlation id only, no detail) |
| `Unavailable` | 503 | `E_UNAVAILABLE` |

Later phases add their variants to this enum with the codes API.md section 6 already lists. The `title` is a fixed
English string per code; the frontend owns the Arabic and English text.

**Correlation** (`common::correlation`): accept `X-Correlation-Id` if it is a UUID, else generate a UUIDv7; put it in
the tracing span, the response header, every problem body, every job and outbox payload it causes, and (phase 02)
every audit entry.

**`IdempotencyService`** (`common::idempotency`, middleware on every POST, PUT, PATCH and DELETE under `/api/v1`):
1. No principal on the request -> the route must be on the exemption list (empty in this phase; phase 01 adds the
   pre-authentication auth routes); otherwise fail closed with 500 `E_INTERNAL` and an error log naming the route.
2. Read `Idempotency-Key`; missing -> 400 `E_IDEMPOTENCY_REQUIRED`; outside `^[ -~]{8,255}$` -> 400 `E_BAD_REQUEST`.
3. `request_hash` = SHA-256 over method, the full original path and query (`OriginalUri`, not the nest-stripped
   path), the principal id and the raw body bytes.
4. Claim: `INSERT INTO admin.idempotency_keys (principal_id, key, endpoint, request_hash) VALUES (...) ON CONFLICT
   DO NOTHING`. Inserted -> run the handler. Not inserted -> read the row:
   - different `request_hash` -> 422 `E_IDEMPOTENCY_REUSE`;
   - same hash, `completed_at` set -> open the sealed body and replay status, content type and body byte for byte,
     plus `x-idempotent-replay: true`;
   - same hash, in flight and younger than `idempotency.in_flight_lease_s` -> 409 `E_IDEMPOTENCY_IN_FLIGHT`;
   - same hash, in flight and older (the first attempt's process died) -> take the claim over with
     `UPDATE ... SET created_at = now() WHERE ... AND completed_at IS NULL AND created_at = $old` and run the handler.
5. Handler success (2xx) -> seal the body (`crypto.seal("idempotency", aad = principal_id || key, body)`) and store
   status, content type and sealed body with `completed_at = now()`. Any other outcome -> delete the claim, so a
   transient failure stays retryable.
6. `maintenance_sweep` deletes rows older than `idempotency.ttl_days`.

**`JobQueue` and the worker harness** (`jobs`, run by `akr-worker`; `akr-ingestd` runs the same harness over its own
kinds from phase 05):
1. `JobQueue::enqueue(tx: &mut Transaction<'_, Postgres>, NewJob { kind, payload, run_at, max_attempts,
   coalesce_key })` only accepts a transaction (golden rule enforced by signature); with a `coalesce_key` it inserts
   `ON CONFLICT (kind, coalesce_key) WHERE status = 'queued' AND coalesce_key IS NOT NULL DO NOTHING`.
2. Poll loop (every 500 ms, or at once after a `NOTIFY akr_jobs` sent by enqueue): claim up to the free permits per
   kind (one semaphore per kind, sized by `JobKind::concurrency`) with the claim query of section 2.
3. Dispatch to the registered handler (`trait JobHandler { const KIND: &'static str; async fn run(&self, ctx:
   JobContext, payload: Value) -> Result<(), JobError> }`). `JobContext` carries the job id, attempt, correlation id
   and the system principal; handlers have no session.
4. Ok -> `status = 'done', finished_at = now()`. `JobError::Retryable` -> `status = 'queued'`, `run_at = now() +
   2^attempts * 2 s` (capped at 1 h), `last_error` redacted; `attempts >= max_attempts` or `JobError::Permanent` ->
   `status = 'dead', finished_at = now()` and the counter `akr_jobs_dead_total{kind}` increments.
5. Lease reaper (same loop, every 60 s): rows `running` with `locked_at < now() - jobs.lease_s` (per-kind override in
   the registry) go back to `queued` with `last_error = 'lease expired'`; handlers are idempotent (at-least-once).
6. Scheduler (same loop, every 30 s): for each periodic kind, compute the next slot from its `schedule.<kind>` cron in
   `bank.timezone` and enqueue it with `coalesce_key = '<kind>:<slot ISO>'`, so several worker replicas enqueue it
   once.
7. A job kind not in the registry is never claimed by a binary that does not know it (claims filter by known kinds);
   the cockpit (phase 03) lists queued jobs of unknown kinds as a release error.

**`OutboxDispatcher`** (`jobs`, loop in `akr-worker`): claim undispatched rows with `available_at <= now() AND
attempts < outbox.max_attempts` (`FOR UPDATE SKIP LOCKED`), route by topic to the registered `TopicHandler`, mark
`dispatched_at` on success, or `attempts + 1`, `last_error`, `available_at = now() + back-off` on failure. Topics
registered in this phase: none in production (`test.ping` under `cfg(test)`); `canon.touched` (phase 11),
`alert.created` (17), `case.event` (17), `notify` (22), `siem.export` (25) register later. Rows at the attempt limit
stay undispatched and are counted by `akr_outbox_stuck` for the cockpit; `maintenance_sweep` never deletes them.

**`TimerService`** (`jobs`): `schedule(tx, NewTimer { owner_kind, owner_id, kind, fire_at, calendar_id, payload })`,
`cancel(tx, id)`, `cancel_for_owner(tx, owner_kind, owner_id, kind)`. Sweep loop every `timers.sweep_interval_s`:
claim due timers (`fire_at <= now()`, `FOR UPDATE SKIP LOCKED`), set `fired_at = now()` and enqueue the job kind the
timer kind maps to (`timer_<kind>`) in the same transaction. **Rule for every timer handler:** it re-reads its owner
and does nothing if the owner's state no longer calls for it (closed case, completed task, released freeze);
cancelling is an optimisation, the re-check is the guarantee (adversarial target "clocks firing on a closed case").
Business-day arithmetic goes through `trait BusinessClock { fn add_working(&self, from, n, unit) -> DateTime<Utc>
}`; this phase ships only `CalendarDaysClock`, named so nobody mistakes it for working days; phase 09 adds
`BankCalendarClock` and every clock defined in working days uses it.

**`SettingsService`** (`common` + `domain`):
1. Typed getters generated from `settings_registry` (`fn idempotency_ttl_days(&self) -> i64`, ...); a 30 s
   in-process cache, invalidated early by a Valkey message on `settings.changed` (best effort; the TTL is the bound).
2. `apply(tx, key, value, expected_version, actor, source: ChangeSource) -> Setting` where `ChangeSource` is
   `Direct` or `ChangeRequest(id)`: validate the value against `value_type` (cron parsed, durations positive, the
   timezone in the IANA list); check `floor` (`min`/`max`) -> 422 `E_BELOW_FLOOR`; refuse `Direct` on a governed key
   (phase 04 calls it with `ChangeRequest`); update with the version check; emit `setting.updated` with before and
   after; publish `settings.changed` after commit.
3. Secrets never live in settings (paths and thresholds only); a test fails if a seeded key ends in `password`,
   `secret`, `token` or `key` unless it is on the registry's allow-list of non-secret names.

**`FlagsService`**: `is_enabled(key, FlagContext { branch_id, roles })` = `enabled` and (scope empty, or the branch
or one of the roles listed); `update(tx, key, UpdateFlag, expected_version, actor)` emits `flag.updated`. Flags gate
features, never security boundaries (environment and Cedar do that).

**Crypto port** (`crypto`, SP-13; key hierarchy TS-24, SP-11 defined here, used from phase 01 on):
- `trait Crypto { seal(family, aad, plaintext) -> Sealed; open(family, aad, &Sealed) -> Vec<u8>;
  keyed_hash(purpose, data) -> [u8; 32]; sign(key, msg) -> Signature; verify(key, msg, sig) -> bool }`.
- OpenBao layout: Shamir 3-of-5 unseal (ceremony runbook, SP-12) or auto-unseal by a second OpenBao or the bank's
  HSM; transit keys `akr-kek` (aes256-gcm96, wraps DEKs), `akr-sign-anchor`, `akr-sign-package`,
  `akr-sign-certificate`, `akr-sign-decision` (ed25519; used from phases 02, 06, 12 and 29); KV v2 mount `akr/` with
  `db/<role>`, `s3/writer`, `s3/presign`, `keys/cursor-hmac`, `keys/match-key`, `sources/<id>` (phase 05).
- DEK families: `personal`, `financial`, `secret-str`, `card`, `idempotency`, `evidence`; one active DEK per family
  and month, created with `transit/datakey/wrapped/akr-kek`, unwrapped once (`transit/decrypt`) and cached in process
  memory; OpenBao is never called per request (DBS rule). `Sealed` carries family, DEK id and key version so old data
  stays readable after rotation; `min_decryption_version` is never raised past live data (SP-11).
- AppRole policies (least privilege): `akr-api` transit decrypt on `akr-kek`, KV read `db/akr_app`, `s3/presign`,
  `keys/*`; `akr-worker` transit encrypt and decrypt on `akr-kek`, sign on the four signing keys, KV read
  `db/akr_app`, `s3/writer`, `keys/*`; `akr-ingestd` KV read `db/akr_ingest` and `sources/*` only (DR-25; grows with
  the `raw` family in phase 11); `akr-assess` KV read `db/akr_assess` (grows in phase 29); `akr-ctl` operator
  policies issued per ceremony, never stored.
- `InMemoryCrypto` (tests and the dev profile) implements the same trait; the RQ-02 parity suite runs the DBS client
  calls against a real OpenBao in CI.

**Object-store port and S3 guard** (`objectstore`, `s3guard`; TS-22, TS-41, TS-42):
- `trait ObjectStore { put(bucket, key, bytes, sha256, lock: LockMode) -> VersionId; get(bucket, key) -> Bytes;
  head(...); presign_get(bucket, key, ttl: 60..=300 s) -> Url }`; uploads are single-part or signed multipart, never
  `STREAMING-UNSIGNED-PAYLOAD-TRAILER`; every write carries `x-amz-checksum-sha256`.
- `akr-s3guard` passes only: PUT and copy object, GET, HEAD, list object versions, multipart (create, upload part,
  complete, abort), DELETE in `akr-quarantine` only, get and put retention and legal hold, and bucket creation from
  the installer's network only. It rejects with 403 (and a counter per rule): the `select` query parameter;
  `X-Amz-Meta-Snowball-Auto-Extract`; `STREAMING-UNSIGNED-PAYLOAD-TRAILER`; query-string credentials on any method
  other than GET and HEAD; any client header beginning `X-Minio-` and `X-Amz-Bucket-Replication-Status`; `x-amz-*`
  headers absent from `SignedHeaders`; admin, STS, console and internode paths. It checks no signatures (the store
  does). The rule-to-advisory map ships as `s3guard/rules.toml`, versioned with the store build.
- Buckets created by `akr-ctl store init` through the S3 API (no `mc` at run time): `akr-evidence` (object lock,
  COMPLIANCE, default retention per class set by phase 02), `akr-anchors` (COMPLIANCE 10 years), `akr-quarantine`,
  `akr-models`, `akr-exports`. Three credential kinds (TS-42): writer (no delete except quarantine, no bucket-level
  writes), presign (GET only on served prefixes), installer and break-glass admin (OpenBao only). Policies name the
  bare bucket ARN and `bucket/*` and explicitly deny `s3:DeleteObjectVersion`, `s3:BypassGovernanceRetention` and
  every bucket-level write.

**`akr-ctl`** (operator CLI, runs in zone Z5 on the management host):
- `db provision` (section 1 provisioning), `db migrate` (runs `sqlx migrate` as `akr_migrate`; refuses when the
  target is production and no approved release id is given), `db check` (Arabic round trip, collations, roles hold no
  unexpected grants).
- `store init`, `store conformance` (the WORM conformance suite of research file 32 section 12.6 item 9 against the
  running store, at install and at every upgrade).
- `bundle verify <dir> --pubkey <file> --fingerprint <hex>` (section 4), `bundle import <dir> --registry <url>`.
- `ops record-restore-drill --result passed|failed --evidence <file>`: writes `ops.last_restore_drill_at` through
  `SettingsService::apply` with `ChangeSource::Direct` under a CLI-only exemption for this one governed key, and
  emits `restore_drill.recorded` (persisted from phase 02).
- Every command that changes an installation logs who ran it (the OS user and the PAM session id from the
  environment) into the audit sink.

**Telemetry** (`telemetry`, TS-27, TS-28): JSON logs to stdout; OTLP traces to the collector; Prometheus metrics. The
**redaction layer** runs before any sink: fields named in the classification table as `personal`, `financial`,
`secret-str` or `card`, any string matching an IBAN, a 13-19 digit card-like run or an Iraqi national-ID pattern,
and every request or response body are dropped or replaced by a length marker; a golden test feeds known values
through each sink and asserts none survives. Metric labels never carry ids of parties, accounts or cases. Platform
metrics from this phase: `akr_http_requests_total` and duration histograms per route template, `akr_jobs_queued`,
`akr_jobs_running`, `akr_jobs_dead_total`, `akr_job_duration_seconds{kind}`, `akr_outbox_pending`,
`akr_outbox_stuck`, `akr_timers_due`, `akr_loop_heartbeat_seconds{loop}`, `akr_idempotency_replays_total`,
`akr_s3guard_rejected_total{rule}`, `akr_backup_age_seconds`, `akr_restore_drill_age_days`. SLO and alert rules live
as code in `deploy/observability/` next to these metrics.

**Audit events emitted:**

| Event kind | On | Resource |
|-|-|-|
| `setting.updated` | `SettingsService::apply` (before, after, source) | setting key |
| `flag.updated` | `FlagsService::update` (before, after) | flag key |
| `restore_drill.recorded` | `akr-ctl ops record-restore-drill` | setting `ops.last_restore_drill_at` |
| `job.dead` | a job reaching `dead` (kind, attempts; no payload) | job id |

They go to the `TracingSink` stub in this phase; phase 02 swaps in the PostgreSQL writer and from then they persist.
No HTTP mutation exists in this phase except the dev-only echo, which writes nothing.

**Cedar actions:** none. The `authz` slot is `AllowAllAuthorizer`, whose constructor refuses `production` (DBS
lesson), so the stub cannot reach a deployed bank. Phase 03 fills the slot.

**Auth slot stub:** outside production the principal comes from an `X-Dev-Principal` header holding a UUID, so the
idempotency ledger (principal-scoped) can be exercised before identity exists; the stub's constructor refuses
`production`, and phase 01 replaces it with the BFF session and bearer-token extractors. A null setting value is
allowed only for keys the registry marks nullable (`ops.last_restore_drill_at` until the first drill).

**Tipping-off review.** This phase adds no screen, task, notice, export or CBS-facing artefact. What it adds that a
person outside a case's access list could read: job and outbox payloads and `last_error` (cockpit, phase 03), logs,
traces, metrics, readiness bodies and problem bodies. Rules that make the answer "nothing": payloads carry ids and
codes only (a unit test per registered kind and topic serialises a sample and rejects any field outside the allowed
id and code names); error text is redacted before it is stored; the redaction layer covers every sink; readiness and
problem bodies carry names and booleans or a code and correlation id, never resource detail; idempotency bodies are
sealed and the table is granted to `akr_app` only.

**Settings and configuration used:** the sixteen keys seeded in migration 0003, all read through the registry.
Environment variables only for bootstrap (`AKR_ENV`, binds, OpenBao address and AppRole, Valkey, S3 guard, OTLP).

## 4. Infrastructure Updates

- **Job kinds registered:** `maintenance_sweep` (daily, `schedule.maintenance_sweep`; concurrency 1; deletes finished
  jobs older than `jobs.done_retention_days`, dispatched outbox rows older than `outbox.retention_days`, idempotency
  rows older than `idempotency.ttl_days`, and fired or cancelled timers older than `timers.terminal_retention_days`;
  each in batches of 5,000 with a pause; reports counts to metrics) and `backup_check` (daily, `schedule.backup_check`;
  concurrency 1; stats `ops.backup_marker_path`, sets `akr_backup_age_seconds`, logs an error when older than
  `ops.backup_max_age_h`; computes `akr_restore_drill_age_days` from `ops.last_restore_drill_at`). Loops (not jobs):
  job poll, lease reaper, scheduler, outbox dispatch, timer sweep, each with a heartbeat metric.
- **Outbox topics:** none in production (section 3). **Timers:** none.
- **Compose (profile S bundle `deploy/compose/` and the dev `docker-compose.yml`, same images):**

| Service | Image | Networks | Notes |
|-|-|-|-|
| `postgres` | PostgreSQL 17 (pinned digest) | `z1_db`, `z2_app`, `z3_ory`, `z4_db` | builtin `C.UTF-8`; TLS; `wal_level=replica`; pgBackRest `archive_command` |
| `postgres-standby` | same | `z2_app` | streaming replica (production bundle; dev profile `standby`) |
| `pgbackrest` | pgBackRest (pinned) | `z2_app` | WAL every 60 s, weekly full, nightly differential, four fulls; repository on the backup server plus offsite, both encrypted by pgBackRest (`repo-cipher-type = aes-256-cbc`) with the passphrase held off the server (SP-84) |
| `restic` | restic (pinned) | `z2_app` | every 15 min: store drive directories (consistent snapshot) and OpenBao snapshots; repository passphrase held off the server |
| `openbao` | OpenBao (pinned) | `z1_bao`, `z2_app` | raft storage; TLS listener; Shamir 3-of-5 in production; `make bao-dev` initialises 1-of-1 for development only |
| `objectstore` | AKR Silo build (pinned tag, built and signed by our pipeline) | `z2_store` (internal) | `MINIO_BROWSER=off`; non-root; read-only root fs; no egress; reachable only from the guard |
| `s3guard` | `akr-s3guard` | `z2_store`, `z2_app`, `z3_edge` | `z3_edge` only for presigned GET through the edge |
| `valkey` | Valkey 9 (pinned) | `z2_app`, `z4_cache` | cache, rate limits, SSE fan-out; no persistence required |
| `kratos`, `kratos-migrate` | Ory Kratos (pinned) | `z3_ory` (only the BFF in `akr-api` and `akr-ctl` reach it; phase 01 section 4) | containers up and healthy; flows and config in phase 01 |
| `hydra`, `hydra-migrate` | Ory Hydra (pinned) | `z3_ory`, `z3_edge` | as Kratos |
| `gotenberg` | AKR Gotenberg image with IBM Plex fonts (RQ-11) | `z2_app` | used from phase 02 |
| `clamd` | ClamAV (pinned) | `z2_app` | signatures from the bank's mirror or the bundle; used from phase 02 |
| `caddy` | Caddy (pinned) | `z3_edge` | TLS from the bank's CA; `/api/*` to `akr-api`; `/` static frontend; `/files/*` to the guard (GET only) |
| `akr-api` | `akr-api` | `z2_app`, `z3_edge`, `z3_ory`, `z5_mgmt` | `ulimits.core = 0`, swap off |
| `akr-worker` | `akr-worker` | `z2_app`, `z5_mgmt` | as above |
| `akr-ingestd` | `akr-ingestd` | `z1_cbs`, `z1_db`, `z1_bao`, `z5_mgmt` | the only container on `z1_cbs` |
| `akr-assess` | `akr-assess` | `z4_gateway`, `z4_db`, `z4_cache`, `z5_mgmt` | compose profile `phase2`; not started by default |
| `otel-collector`, `jaeger`, `prometheus`, `alertmanager` | pinned | `z5_mgmt` | scrape the metrics listeners; receive OTLP |
| `log-forwarder` | Fluent Bit (pinned) | `z5_mgmt` | JSON stdout to the bank's SIEM over syslog or CEF |

  Networks `z1_db`, `z1_bao`, `z2_app`, `z2_store`, `z3_ory`, `z4_db`, `z4_cache` and `z5_mgmt` are `internal: true`
  (no egress). Egress to the CBS (zone Z0) is enforced on the host: the bundle ships nftables rules rendered by
  `akr-ctl net render` from `deploy/zones.yaml` that let only `akr-ingestd`'s container address reach the CBS
  listener addresses declared at install. Every container runs non-root with a read-only root file system and
  dropped capabilities.
- **Zone rules (SP-04):** `deploy/zones.yaml` is the single statement of who reaches whom (ARCHITECTURE.md section 4,
  with the DR-25 edge: `akr-ingestd` reaches OpenBao for KV reads only). `make zone-check` starts a probe container
  attached to exactly the networks of each service and tries every service port in the stack; the result must equal
  the matrix (for example: from `akr-ingestd`'s position, `postgres:5432` and `openbao:8200` open; `objectstore`,
  `valkey`, `gotenberg`, `kratos` closed; from `akr-api`'s position, `objectstore:9000` closed and `s3guard` open).
- **AppState** (`api`): `pool: PgPool` (as `akr_app`), `valkey: fred::Pool`, `settings: SettingsService`, `flags:
  FlagsService`, `jobs: JobQueue`, `crypto: Arc<dyn Crypto>`, `store: Arc<dyn ObjectStore>`, `audit: Arc<dyn
  AuditSink>`, `authz: Arc<dyn Authorizer>`, `config: Arc<ApiConfig>`. `worker` adds the job registry, the outbox topic
  registry and the timer kind registry. `ingestd` holds a pool as `akr_ingest` and nothing else in this phase.
- **OpenAPI shell:** `ApiDoc` in `api` with `info` (title "AKR-TMS API", version from the crate), servers, the
  `Problem` and `ValidationError` schemas, the three security schemes, and tags `infra` (this phase) plus the module
  tags reserved for later phases; Swagger UI at `/swagger-ui` outside production only; the spec served at
  `/api/v1/openapi.json`. Every later phase registers its paths through `OpenApiRouter` (`routes!`), so an
  unregistered handler cannot exist.
- **CI (Gitea Actions, TS-31, SP-72, SP-75):**
  1. `cargo fmt --check`; `cargo clippy --all-targets -- -D warnings` (with the workspace lints, SP-70); release
     profile with `overflow-checks = true`.
  2. `cargo deny check` (licences allow-list, advisories, bans with the driver wrappers); `cargo audit`; the waiver
     register `deploy/waivers.toml` (owner, reason, expiry; an expired waiver fails the build).
  3. `cargo test` against a throwaway PostgreSQL 17 service container (builtin `C.UTF-8`); `cargo sqlx prepare
     --check`; the schema checks of section 6; the redaction golden test; the OpenBao parity suite (RQ-02) against a
     pinned OpenBao container.
  4. `cargo build --release` with `cargo-auditable`; images on a pinned distroless base, non-root.
  5. SBOMs: `cargo-cyclonedx` per binary, the frontend packages from the pnpm lock file, and Syft per image (CycloneDX), with maintainer and country per component
     (VO-07, TS-37); Trivy scan (HIGH and CRITICAL fail unless waived); cosign signature with the vendor key held in
     the vendor's KMS under two-person control (SP-73); push to the vendor Harbor.
  6. Object store: build the Silo tag from source, compare with Silo's published checksums and provenance, run the
     WORM conformance suite and the security regression suite with and without the guard (TS-22); the advisory watch
     (TS-43) is a scheduled vendor job outside this pipeline that files findings against the store certification
     report.
  7. Bundle (release branches): `akr-ctl bundle build` writes an OCI image layout with every image, the migrations,
     the compose bundle, adapter packs (from phase 05), docs, SBOMs and their diff against the previous release,
     notices (third-party notices file; the store's Corresponding Source, LICENSE, NOTICE, CREDITS), test evidence and
     the rollback plan; a manifest of SHA-256 digests; cosign signatures over the manifest and every image (TS-30,
     SP-66, SP-74). Protected branches and a second approver on release branches.
- **Bundle verification at the bank:** `akr-ctl bundle verify` checks the public key against the fingerprint the
  bank received on a separate channel, verifies the manifest signature and every image signature, checks every digest
  in the manifest, confirms the SBOM, notices and rollback plan are present, and prints the SBOM diff against the
  installed release; any failure refuses before anything changes. `bundle import` copies the images into the bank's
  Harbor with content trust enforced. The release and update screen (A4) comes in phase 25.
- **Frontend pipeline:** `frontend/` scaffold (Vite, React 19, TypeScript, TanStack Router and Query, Orval) with
  `make generate-api` against the OpenAPI shell and a CI drift check (the committed client must equal a fresh
  generation).
- **DBS rules ported into `.claude/rules/`** (A2; adapted as planning.md was, differences marked "AKR"):
  `implementation-flow.md`, `admin-managed.md`, `ddd.md`, `jobs.md` (AKR: four binaries, ingestd's own kinds and RLS,
  loops versus jobs), `migrations.md` (AKR: UUIDv7 with no database default, which also fixes the DBS rule file's
  stale `gen_random_uuid()` template; `NNNN_` numbering; schema-qualified names; per-table grants; classification and
  retention rows; reserved-word ban), `rust-backend.md`, `api-design.md`, `auth.md`, `testing.md`, `docker.md` (AKR:
  never `docker rm`, `compose rm` or `prune`; zone networks), `file-edits.md`. `frontend.md` and `form-templates.md`
  are ported in phase 08.
- **Existing entity, schema or route updates:** none (first phase).

## 5. Frontend Impact

Scaffolding only: the `frontend/` project, the Orval pipeline proven against `/healthz` in the OpenAPI shell, and the
drift check in CI. No tokens, shell or screens (phase 08). Backend and platform phase otherwise.

## 6. Verification

1. `make db && make migrate`: provisioning then 0001-0004 apply; running `make migrate` again is a no-op; `akr-ctl db
   check` passes (Arabic, Kurdish and Persian round trip through every role; the fixture sorts in the expected
   `akr_ar` order).
2. `cargo check`: compile-time SQL passes; `cargo sqlx prepare` writes `.sqlx/`, committed.
3. `cargo clippy --all-targets -- -D warnings`: clean; `cargo deny check`: clean, and a scratch crate that adds
   `tiberius` to `akr-worker` makes `cargo deny check bans` fail.
4. `cargo test`, including:
   - idempotency: a replay returns a byte-identical body with `x-idempotent-replay: true`; the same key with a
     different body -> 422 `E_IDEMPOTENCY_REUSE`; the same key from a different principal runs independently; 12
     concurrent identical requests -> exactly one execution and one ledger row, the others 409 or a replay; a handler
     error leaves no row; an abandoned in-flight row older than the lease is taken over; the stored body is
     ciphertext (it does not contain the plaintext bytes);
   - jobs: enqueue inside a rolled-back transaction leaves no row; a handler error backs off and reaches `dead`
     after `max_attempts`; a killed worker's job is reclaimed after the lease and completes once; two enqueues with
     one coalesce key while queued -> one row; the scheduler run by two worker instances enqueues one job per slot;
   - outbox: a failing topic handler backs off and stops at `outbox.max_attempts`; `maintenance_sweep` deletes
     dispatched rows past retention and never an undispatched one;
   - timers: a due timer fires once under two concurrent sweeps; a cancelled timer never fires; the handler of a
     timer whose owner is closed does nothing;
   - settings: a value outside its `floor` -> `E_BELOW_FLOOR`; `Direct` on a governed key is refused; a version
     mismatch -> `VersionConflict` with the current version; the registry and the seeded rows agree;
   - schema checks (run after migrations, against `information_schema` and `pg_get_keywords()`): every column of
     every table in the twelve AKR schemas has a `column_classification` row; every table has a
     `retention_assignment` row; no column is `timestamp without time zone`; no `id` column has a default; no schema,
     table or column name is a reserved keyword; `akr_readonly` holds no privilege on `admin.idempotency_keys`,
     `admin.jobs`, `admin.outbox` or `admin.settings`;
   - RLS: connected as `akr_ingest`, selecting `admin.jobs` returns only `ingest_` and `discovery_` rows and an
     insert of kind `maintenance_sweep` is refused;
   - redaction: the golden values (a name, an IBAN, a card-like number, a national-ID pattern) never appear in log,
     trace or metric output.
5. curl, against the dev stack (`AKR_ENV=development`):
   - `curl -s localhost:8080/healthz` -> `{"status":"ok"}`; `curl -s localhost:8080/readyz` -> 200 with
     `{"status":"ready","checks":[{"name":"database","ok":true},{"name":"migrations","ok":true},{"name":"valkey",
     "ok":true}]}`; stop Valkey -> 503 with `"valkey","ok":false`.
   - `curl -s -X POST localhost:8080/api/v1/_echo -H 'X-Dev-Principal: 0190c000-0000-7000-8000-000000000001' -H
     'Idempotency-Key: echo-key-0001' -d '{"a":1}'` twice -> byte-identical bodies, the second with
     `x-idempotent-replay: true`; the same key with `-d '{"a":2}'` -> 422 `{"code":"E_IDEMPOTENCY_REUSE",...}`; no
     `Idempotency-Key` -> 400 `E_IDEMPOTENCY_REQUIRED`.
   - With `AKR_ENV=production` the binary refuses to start while `X-Dev-Principal` support is enabled, and
     `/api/v1/_echo` returns 404 when it is not.
   - `curl -s localhost:9101/metrics` (management listener) shows `akr_http_requests_total`; the same path on the
     public port returns 404.
   - No Cedar 403 or existence-masking 404 case exists yet (no protected resource; phase 03).
6. Swagger UI loads (development); `make generate-api` regenerates the frontend client and a second run shows zero
   diff.
7. Audit: `akr-ctl ops record-restore-drill --result passed --evidence drill.pdf` updates the setting and the
   `restore_drill.recorded` event appears on the tracing sink (persisted rows from phase 02).
8. Platform: `make zone-check` matches `deploy/zones.yaml`; `akr-ctl store conformance` passes (COMPLIANCE delete,
   overwrite, retention shortening and bucket deletion all refused, presigned URL expiry, clock comparison); a
   request with `?select&select-type=2` or an `X-Minio-` header through the guard -> 403 and
   `akr_s3guard_rejected_total` increments; `backup_check` sets `akr_backup_age_seconds` from a touched marker; a
   bundle built by CI passes `akr-ctl bundle verify` and imports into the test Harbor, and the same bundle with one
   byte changed in an image layer is refused.
9. Existing tests: none before this phase.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 1, from tools/agent/gap1-run1.md; lead-verified (tools/agent/gap1-run1-verdict.md).

### 7.1 Outbox handler lists and topic names
- `TopicHandler` registrations are a list per topic. `register(topic, handler)` appends. `OutboxDispatcher` runs the
  list for a row in order, inside one transaction, and sets `dispatched_at` only when every handler succeeded. If any
  handler fails, the row stays undispatched and the back-off of this section applies. Every handler is idempotent (the
  owner re-check rule of section 3).
- The reserved topic `notify` (phase 22) is named `notify.request`, because the `admin.outbox` CHECK requires a dotted
  name. Replace "`notify` (22)" in the reservation sentence with "`notify.request` (22)".
- Topic `siem.export` (phase 25) is released: no phase uses it. Remove it from the reservation sentence.
- Each topic is registered by the phase that produces or consumes it, through `register`. The registry is per topic,
  not per phase.

### 7.2 Job harness: lease heartbeat, cluster-wide concurrency, database clock, listener reload
- Lease heartbeat: `JobStore::heartbeat(id, locked_by)` refreshes `locked_at` every third of the kind's lease
  (`jobs.lease_s` or the per-kind override) while a handler runs; the update is guarded by `status = 'running'` and `locked_by` (the reaper leaves `locked_by` as it was, so the status test is what stops a late heartbeat). A heartbeat that matches no row (the reaper re-queued the job) changes nothing: the runner lets the handler finish, since handlers are idempotent (at-least-once, section 3), and a handler that uses `complete_in` (7.18) then rolls back. The lease
  reaper re-queues only rows whose `locked_at` is older than the lease.
- Cluster-wide concurrency: the claim enforces `JobKind::concurrency` across all nodes. Inside the claim transaction it
  takes `pg_advisory_xact_lock(hashtext(kind))` and counts the kind's `running` rows. No new field on `JobKind`. The
  per-process semaphore stays as a local bound only.
- Scheduler: schedule slots are computed from the database clock (`now()`), never from the process clock.
- Listeners: a LISTEN connection reloads its state in full after every reconnect; the notification is a hint only.
  Channels in the system: `akr_jobs` (this section), `akr_policy` (phase 03; polls `active_id()`), `akr_source`
  (phase 05), `akr_assess_policy` and `akr_assess_gateway` (phase 29).

### 7.3 Timer kinds registered by later phases
The timer kind registry is a list of kind and owner kind pairs. The sweep enqueues the job `timer_<kind>`; a kind that
is not in the registry is never claimed (the job rule of this section). Kinds registered by later phases (each in its
own section 4.2):

| Timer kind | Owner kind | Registered by |
|-|-|-|
| `cr_activate`, `cr_emergency_review`, `cr_post_review` | `change_request` | phase 04 |
| `task_timeout` | `task` | phase 04 |
| `screening_alert_clock` | `screening_alert` | phase 14 |
| `kyc_review_clock` | `kyc_review` | phase 15 |
| `alert_touch`, `alert_age` | `alert` | phase 17 |
| `work_item_due` | `work_item` | phase 18 |
| `case_acl_expiry` | `case_acl` | phase 18 |
| `rfi_clock` | `rfi` | phase 19 |
| `breach_stage` | `alert`, `work_item` | phase 19 |
| `watch_expiry` | `watch` | phase 19 |
| `hibernation_expiry` | `hibernation` | phase 19 |
| `qa_grant_expiry` | `qa_sample` | phase 19 |
| `intake_clock` | `internal_report`, `authority_request` | phase 19 |
| `freeze_due`, `freeze_limit`, `committee_report_due`, `second_report_due` | `freeze` | phase 20 |
| `refrain_due` | `refrain` | phase 20 |
| `report_filing_target` | `report` | phase 21 |
| `report_rework_due` | `report_task` | phase 21 |
| `liaison_due` | `liaison_monthly_report` | phase 22 |

### 7.4 Monitoring and retention roles
Migration 0001 is unchanged. The role list of section 1 is completed with the two roles that later phases create
(DR-22: login facts are provisioning):
- `akr_monitor`: a LOGIN role created by `akr-ctl db provision` (password in OpenBao `db/akr_monitor`), member of
  `pg_monitor` only, with no privilege on any AKR schema (phase 03 section 4.4; phase 01 7.2).
- `akr_retention`: NOLOGIN, created by phase 23 migration 0072 with its table grants; its LOGIN role
  `akr_retention_login` (member of `akr_retention` and of `akr_audit_writer`, password `db/akr_retention_login`) is
  created by `akr-ctl db provision` (phase 23 sections 1.2 and 4.4; phase 01 7.2).

### 7.5 KV mount rule, AppRole policy additions and KV paths
- **One KV v2 mount, relative paths (DR-356).** OpenBao has one KV v2 mount, `akr/` (section 3). Every KV path in every
  phase is relative to it and is written without a mount prefix: `db/<role>`, `s3/<name>`, `keys/<name>`,
  `sources/<id>`, `ory/<x>`, `polis/<x>`, `release/trust`, `kratos/oidc/<provider>`, `assistant/<x>`. The spellings
  `kv/...` (phases 25 and 31) and `secret/...` (phase 28) mean the same paths under `akr/`; those phases' own 7.x
  record the correction.
- AppRole policies (section 3), in addition to the current lines:
  - `akr-api`: KV read `db/akr_readonly` (read-only pool, phase 24), `s3/stager` (stager credential, phase 02) and
    `polis/directory` (webhook secret, phase 31).
  - `akr-worker`: KV read `db/akr_readonly` (verifier pool, phase 02), `db/akr_retention_login` (phase 23),
    `assistant/llama` and `assistant/golden-hmac` (phase 28) and `release/trust` (phase 25); read on
    `sys/seal-status` and `sys/health` (phase 03); sign on the signing keys of 7.6.
  - `akr-ingestd`: unchanged (`db/akr_ingest`, `sources/*`).
  - `akr-assess`: transit encrypt and decrypt on `akr-kek`; KV read `db/akr_assess`, `keys/assess-decision/*`,
    `keys/assess-extract/*` (phase 29 section 4.4).
  - New AppRoles: `akr-monitor` (KV read `db/akr_monitor`, for the postgres exporter, phase 03); `akr-migrate` (KV
    read `db/akr_migrate` only, issued per release, phase 30); `akr-polis` (KV read `ory/polis` and `polis/keys`
    only, phase 31).
- KV items added: `s3/stager` (phase 02), `keys/model-artifact/v<N>.pub` (phase 27, verify-only), `db/akr_monitor`,
  `db/akr_retention_login`, `db/akr_migrate`, `db/akr_readonly`, `polis/directory`, `polis/keys`, `ory/polis`,
  `release/trust`, `kratos/oidc/<provider>`, `assistant/llama`, `assistant/golden-hmac`.

### 7.6 Signing keys: inventory (DR-357)
The transit key list of section 3 (OpenBao layout) becomes:
- `akr-kek` (aes256-gcm96, wraps DEKs);
- ed25519 signing keys used by `akr-worker`: `akr-sign-anchor` (audit anchors, phase 02), `akr-sign-package`
  (as section 3 names it), `akr-sign-certificate` (coverage certificates, phases 05 and 12) and `akr-sign-deletion`
  (deletion certificates and batch manifests, phase 23; a separate key with its own rotation and custody, DR-194;
  phase 23 wrote it `akr-sign-cert`, corrected in its 7.1);
- `akr-sign-bundle` (ed25519; support bundles, phase 25): signed by `akr-ctl support-bundle create` on the bank host
  under an operator policy issued for that act, never by a service AppRole;
- `akr-sign-decision`: reserved and unused. Decision signing uses a seed wrapped under `akr-kek`, unwrapped once per
  node at start-up (DR-294, phase 29 section 3.4).
In the `akr-worker` AppRole line, "sign on the four signing keys" becomes "sign on `akr-sign-anchor`,
`akr-sign-package`, `akr-sign-certificate` and `akr-sign-deletion`".

### 7.7 Zone matrix rows, networks and zones.yaml keys
- Row `akr-assess` (line 705): networks `z4_gateway`, `z4_db`, `z4_cache`, `z5_mgmt` and `z1_bao` (phase 29 section 4.4).
- New networks: `z2_egress` (not internal; attached to `akr-worker` only; created only when `egress.proxy` is declared;
  phase 07 section 4). `z2_bank` (not internal; `akr-worker` only; egress only to the SIEM collector and TSA addresses
  declared with `akr-ctl siem configure`; phase 25 section 4.4). `z3_idp` (not internal; `kratos` only; egress to the
  OIDC issuer; phase 25 section 4.4). `z2_model` (`internal: true`; members `akr-worker`, `llama`, `llama-eval`; no
  egress; phase 28 section 4.4).
- New rows: `node-exporter` (`z5_mgmt`), `postgres-exporter` (`z2_app`, `z5_mgmt`), `blackbox-exporter` (`z5_mgmt`,
  `z3_edge`, `z1_bao`, `z2_store`; phase 03 section 4.4); `polis` (`z3_ory`, `z3_edge`; phase 31 section 4.4).
- Zone rules in `deploy/zones.yaml` (add to the rule list at line 713): `akr-worker` -> `prometheus:9090`; `prometheus`
  -> the three exporters; `blackbox-exporter` -> `caddy:443`, `openbao:8200`, `s3guard:9000`; `postgres-exporter` ->
  `postgres:5432` (phase 03); `akr-worker` -> `gotenberg:3000` and `clamd:3310` (phase 02 section 4); `akr-worker` ->
  `llama:8080` (phase 28); `akr-worker` -> the declared egress proxy (phase 07); `akr-worker` -> the bank's SMTP relay
  (`notify.smtp_host`, `notify.smtp_port`; phase 22 section 4.4); `akr-assess` -> `openbao:8200`.
- Keys read by the renderer, added to `deploy/zones.yaml`: `cluster.apiserver`, `cbs.listeners`, `gateway.cidrs` and
  `backup.repository` (phase 30 sections 4.4 and 7.5) and `egress.proxy` (phase 07 section 4).

### 7.8 Compose services, volumes, profiles and the image lock
- Services added to the table of section 4: `node-exporter`, `postgres-exporter`, `blackbox-exporter` (phase 03,
  section 4.4, pinned, networks as in 7.7); `llama` (`akr/llama-server`, pinned and signed, `z2_model`, volume
  `assistant-store` read-only, bearer secret from a read-only file); `llama-eval` (same image, profile `assistant-eval`);
  `llama-stub` (test only, profile `test`, never in the production bundle) (phase 28 section 4.4).
- Volumes: the installation file `deploy/branding.json`, a read-only bind mount into `akr-worker` at
  `/etc/akr/branding.json` (phase 06 section 4); `model-store`, read-only, mounted into `akr-worker` at `/var/akr/models` with `AKR_MODEL_STORE` set
  (phase 27 section 4.4); `assistant-store`, read-only, mounted into `llama` and `akr-worker` at `/var/akr/assistant`
  (phase 28 section 4.4); the inbox host path `AKR_INGEST_INBOX_ROOT` (default `/var/akr/inbox`), mounted into
  `akr-ingestd` only (phase 11 section 4.2).
- Profiles: `cdc` (opt-in, `akr-cdc`, not started unless `ingest.cdc_enabled`; phase 11 section 4.2); dev profiles
  `fixtures`, `fixtures-oracle` and `fixtures-mssql` (phase 05 section 4; images pinned by digest).
- `deploy/images.lock`: the file that pins every image by digest, one line per image (name, digest, source). Phases 05
  and 28 read it; section 4 must define it.

### 7.9 Readiness: akr-assess and akr-ingestd
- Replace "`akr-assess` database, Valkey" with: "`akr-assess` database, the decision key loaded, every enabled list
  index built, and the policy snapshot loaded. Valkey is not a readiness condition (DR-325)."
- `akr-ingestd` adds: `source_listener` (the `LISTEN akr_source` connection is up) and `adapters` (every compiled
  adapter's client library loads; a missing Oracle client makes that adapter `unavailable`, not the binary unready).

### 7.10 Caddy edge headers, exclusions and the web pipeline
- Caddy adds, for `/`: `Content-Security-Policy: default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:;
  font-src 'self'; connect-src 'self'; frame-ancestors 'none'; base-uri 'none'; form-action 'self'; object-src 'none'`;
  `Strict-Transport-Security`; `X-Content-Type-Options: nosniff`; `Referrer-Policy: no-referrer`; `Permissions-Policy`
  denying camera, microphone, geolocation, payment and USB. `Cache-Control: no-store` on `index.html`; hashed assets
  immutable for one year. `/branding.json` is served with `no-cache`. (Phase 08 section 4.)
- Caddy answers 404 for `/api/v1/directory/events` in every profile, before the `/api/*` rule (phase 31 section 4.4).
- Web pipeline steps (section 6): `pnpm install --frozen-lockfile`; `pnpm licenses list` check; `tsc -b`; `eslint`;
  `scripts/lint-design.mjs`; `scripts/gen-tokens.mjs` and `scripts/check-contrast.mjs`; the Orval, contract and
  policy-matrix drift check; `vitest run`; `vite build` with the bundle budget; Playwright with axe (phase 08 section 4).

### 7.11 maintenance_sweep and backup_check additions
`maintenance_sweep` also deletes:
(a) `admin.stepup_assertions` rows older than `stepup.retention_days` (phase 01);
(b) objects under `staging/` in `akr-quarantine` older than `evidence.staging_ttl_h` that have no evidence row or whose row is
    clean; it never deletes an infected or rejected object (phase 02 section 3);
(c) goAML `transient/` objects older than `evidence.staging_ttl_h` whose run is final or missing; the run is flagged
    `transient_deleted` and audited as `goaml_check.transient_deleted` with `by: sweep` (phase 07 section 3.2).
`backup_check` also sets the gauge `akr_failover_drill_age_days` from `ops.last_failover_drill_at` (phase 30 section 3.2).

### 7.12 Release bundle: charts and K3s artifacts
- `akr-ctl bundle build` (step 7 of section 4) adds `chart/` (the Helm chart `chart/akr` and the third-party charts pinned by
  digest) and `k3s/` (the K3s binary, the air-gap images tarball of the pinned release, the install script). The manifest
  lists them with SHA-256 digests, as the other files do.
- `akr-ctl bundle verify` also verifies the chart signatures. Nothing else in the verify path changes (phase 30 section 3.12).

### 7.13 Schema list: train
Migration 0001 is unchanged. Phase 31 creates the schema `train` in its own migration (`CREATE SCHEMA IF NOT EXISTS
train AUTHORIZATION akr_migrate`, with its grants; DR-338). The schema list of section 1 names it as a later addition,
as it does for the schemas of later phases.

### 7.14 Config and coverage tests for the audit partitions
- `Config::validate` refuses `TracingSink` when `AKR_ENV = production` (phase 02 section 2).
- The classification and retention coverage tests of section 6 skip partitions (`relispartition = true`) and run over the
  `audit` schema as well (phase 02 section 4).

### 7.15 Keyed-hash purposes
Every call of `keyed_hash(purpose, data)` names a purpose, declared as a constant of `common::crypto::Purpose` by the
phase that uses it; a test asserts that the names are unique. Purposes declared so far: `csrf`, `token` and
`login_identifier` (phase 01), and `islamic.document_ref` (phase 26; phase 26 wrote "phase 02 purpose", corrected in
its 7.1). The phase 10 mapping transform `KeyedHash(purpose)` computes `_hk` values with the match key
`keys/match-key` and declares its purposes the same way.

Adversarial pass (2026-10-10), run 4 (clocks and timers on closed, merged or reopened items), from tools/agent/adv-run4.md; lead-verified (tools/agent/adv-run4-verdict.md).

### 7.16 Cancel touches pending timers only (ADV-4-10, MEDIUM; DR-410)
`TimerService::cancel` and `cancel_for_owner` run `UPDATE admin.timers SET cancelled_at = now() WHERE <id, or owner_kind,
owner_id and kind> AND fired_at IS NULL AND cancelled_at IS NULL` and return the number of rows changed. A timer that
already fired or was cancelled matches nothing, so the call succeeds with 0; without the predicate the CHECK `fired_at IS
NULL OR cancelled_at IS NULL` would fail the owner's transaction (a freeze release, a filing, a work item completion).
Test: a fired `freeze_due` timer, then `cancel_for_owner` on the freeze: 0 rows, and the release commits.

Adversarial pass (2026-10-10), run 6 (idempotency, audit chain and outbox replay), from tools/agent/adv-run6.md; lead-verified (tools/agent/adv-run6-verdict.md).

### 7.17 One transaction for the claim, the handler and the stored response (ADV-6-01, ADV-6-02, HIGH; DR-412)
`IdempotencyService` (section 3) changes as follows; DR-412 replaces the takeover clause of DR-21.
1. After step 3 the middleware opens the request transaction `RequestTx` and runs `SET LOCAL lock_timeout` to
   `idempotency.in_flight_lease_s`. Step 4's claim INSERT runs on it.
2. The handler receives `RequestTx` as its business transaction (an extractor) and never commits it. A step that a
   phase names "in its own transaction" (the decision row of phase 03, access events of phase 02, the opened-view rows of
   phase 18) keeps its own transaction: those rows record an attempt, and a retry may write them again.
3. Handler success (2xx): the business audit emit is still the last business statement (phase 02 rule); the middleware
   then seals the body, writes step 5's columns on the claim row and commits once. Any other outcome rolls the whole
   transaction back, which replaces "delete the claim"; the transient failure stays retryable.
4. A second request with the same key and hash blocks on the uncommitted claim (READ COMMITTED: `INSERT ... ON CONFLICT
   DO NOTHING` waits for the other transaction). When the first commits it reads `completed_at` and replays; when the
   first rolls back its own INSERT succeeds and it runs the handler; when the wait passes `lock_timeout` it answers 409
   `E_IDEMPOTENCY_IN_FLIGHT`.
5. The takeover branch of step 4 is removed: a dead process leaves no committed claim.
Tests: (a) a fault hook that kills the request after the handler returns and before commit: a retry runs the handler
once and the case has one note; (b) a handler that sleeps 3 s with a 1 s lease and two requests with one key: one
handler run, the second gets 409 or the replay.

### 7.18 Jobs complete in their own transaction (ADV-6-05, MEDIUM; DR-412)
`JobContext::complete_in(tx)` runs `UPDATE admin.jobs SET status = 'done', finished_at = now() WHERE id = $1 AND status
= 'running' AND attempts = $2` in the handler's transaction (`$2` is the attempt number `JobContext` carries; every claim adds one to `attempts`, so it names this worker's claim, and `locked_at` cannot, because the heartbeat of 7.2 moves it). One
row: the effects and the completion commit together, and the harness's step 4 write is skipped. Zero rows (the lease
reaper re-queued the job or another worker holds it): the handler returns `JobError::Retryable` and its transaction
rolls back, so only one attempt's effects commit. A handler whose work is one transaction uses it; from phase 15 on
`rating_compute` does. Handlers that run one transaction per item make each item's own state change the marker, so a second attempt skips the items already done (phase 19 `hibernation_release` sets `released_at` per hit).

### 7.19 Lost outbox rows and dead jobs page the operator; the key window (ADV-6-04 MEDIUM, ADV-6-06 LOW; DR-412)
1. `deploy/observability/` holds two alert rules: `akr_outbox_stuck > 0` for 5 minutes, and an increase of
   `akr_jobs_dead_total` over 15 minutes; each is a P2 page to the operator, in addition to the phase 03 cockpit tile. A
   row at `outbox.max_attempts` is lost to its consumers (a KYC trigger, a notice, an alarm) until someone acts, so it
   must not wait for someone to open the cockpit.
2. An `Idempotency-Key` is valid for `idempotency.ttl_days` (DR-21: a storage window). A retry after the sweep is a new
   request; the generated client retries within minutes and makes a new key for a new user action.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.20 SP-74: the VEX statement, and where models are listed (V-00-02-b, V-00-02-c, MEDIUM; DR-413)
- CI step 7 (section 4): the bundle also holds one VEX statement per release (CycloneDX), beside the SBOMs and under the
  manifest digests. It lists the known vulnerabilities of the release's components that are not exploitable in AKR-TMS,
  each with its reason and owner, taken from the waiver register `deploy/waivers.toml` (step 2), which also holds the
  waived Trivy findings of step 5.
- `akr-ctl bundle verify` treats the VEX statement as part of the SBOM set: a bundle without it is refused as a bundle
  without an SBOM is (phase 25 refusal code `sbom_missing`).
- A model is not a release component: a release imports no model (DR-390). The signed manifest of a model version is
  its bill of materials (phase 27 section 3.3; phase 28 section 3.9), so the release SBOM lists no model.

### 7.21 A failing WAL archive pages the operator (V-00-03-a, MEDIUM; DR-413)
From phase 03 on (the postgres exporter), `deploy/observability/` holds a third alert rule beside the two of 7.19: the
WAL archiver reports a failure newer than its last success for 5 minutes (the exporter's `pg_stat_archiver` series). It
is a P2 page to the operator (SP-84, the loud WAL-gap alarm). `backup_check` and its marker are unchanged: they cover the
age of the last backup, not a gap in the archive.

### 7.22 Crypto port errors (V-00-05-a, MEDIUM; DR-413)
Section 2 crypto port: the trait line is shorthand. Every method returns `Result<_, CryptoError>` (OpenBao or the key
cannot be reached, or the tag and AAD check of `open` fails). `CryptoError` is the type phase 12's
`CertificateSigner::sign` already names. A caller maps it to 503 `E_UNAVAILABLE`, as phase 02 section 3 does for its
`Crypto` case, unless its own error section says otherwise.

### 7.23 The TopicHandler trait (V-00-06-a, MEDIUM; DR-413)
Section 3 `OutboxDispatcher`, with 7.1: a topic handler implements
`trait TopicHandler { async fn handle(&self, tx: &mut Transaction<'_, Postgres>, row: &OutboxRow) -> Result<(), JobError>; }`.
`OutboxRow` is the claimed `admin.outbox` row that `OutboxStore::claim` returns; `tx` is the row's transaction of 7.1.
Any `Err` leaves the row undispatched under the back-off of that section; `JobError::Permanent` is treated like
`Retryable` here, because a row leaves the queue only by success or at `outbox.max_attempts`.

### 7.24 Harness rows and retention sweeps emit no audit event (V-00-09-a, V-00-10-a, LOW; DR-413)
Rows of `admin.jobs`, `admin.outbox`, `admin.timers` and `admin.idempotency_keys` are operational state, not business
data. Their inserts, claims, fires, cancels, retries and retention deletes emit no audit event: the owner's own event
records the business effect, and `job.dead` is the one harness event (section 3, the audit-event table). The same holds for the
`maintenance_sweep` deletes of 7.11 (a) and (b): the issue and the use of a step-up assertion are audit events already
(phase 01), and a staging object that never became evidence has no record to keep. `maintenance_sweep` reports every
delete count to metrics. Item (c) of 7.11 stays audited, because it changes a run's row.

### 7.25 Setting change hooks (V-06-04-b, MEDIUM; DR-413)
`SettingsService::apply` consults a registry of `SettingChangeHook`s, one list per key, empty in this phase and registered by the consuming phase (the pattern of phase 03's `FlagChangeHook`). A hook runs in the caller's transaction after the write, for every `ChangeSource` and for a revert. From phase 06 on, phase 06 registers the hook that enqueues `coverage_grade {reason: "setting"}` when `coverage.maintained_fields` changes; that is the enqueue phase 06 sections 3.3 and 4 give to "the phase 04 activation of the setting".

### 7.26 Keyset cursor and Page<T> (V-06-06-c, LOW; DR-413)
The cursor of the `common` module (section 2) is the type `Keyset`: an opaque position on a list's sort columns, sent as the `cursor` query parameter and returned as `next_cursor`, as the pagination rule of docs/API.md states. `Page<T> { items: Vec<T>, next: Option<Keyset> }` is what every keyset list method returns; `next` is `None` on the last page. `Cursor` in the repository signatures of phases 03 and 04 is the same type.

### 7.27 Who creates the transit keys (V-23-09-a, MEDIUM; DR-413)
The transit keys of the OpenBao layout (section 3 and 7.6) are created once per install by the OpenBao ceremony runbook (SP-12), and by `make bao-dev` in development. No `akr-ctl` command creates a key; phase 23 named an `akr-ctl ops provision` that does not exist.

### 7.28 Help articles and release notes in the bundle (V-25-08-b, MEDIUM; DR-413)
From phase 25 on, `akr-ctl bundle build` (section 4, item 7) also writes `help/*.json` (the help articles and "what changed" entries of phase 25 section 3.9) and `notes.json` (the bilingual release notes with the policy bundle hash, which phase 25 section 3.6 reads). The manifest lists both with their SHA-256 digests, like every other file.

### 7.29 The failover drill key: CLI exemption and null seed (V-30-06-a, V-30-06-b, MEDIUM; DR-328; DR-413)
From phase 30 on, the CLI-only exemption of section 3 covers two governed keys: `ops.last_restore_drill_at`, written by `akr-ctl ops record-restore-drill`, and `ops.last_failover_drill_at`, written by `akr-ctl ops record-failover-drill` (phase 30 section 3.9). Both writes go through `SettingsService::apply` with `ChangeSource::Direct`, and no other governed key is covered. The settings registry marks `ops.last_failover_drill_at` nullable until the first passed drill, as it marks `ops.last_restore_drill_at`, so the null seed of migration `0095` is allowed.

## What This Phase Does NOT Touch

- No authentication or users (phase 01); the dev principal header exists only outside production.
- No audit persistence, partitions or anchors (phase 02); `TracingSink` only, and `partition_maintenance` arrives
  with the first partitioned table.
- No Cedar policies or existence masking (phase 03); `AllowAllAuthorizer` only, refused in production.
- No admin routes: settings read, flags, jobs list and retry, and the ops cockpit v1 land in phase 03; governed
  setting changes through change requests in phase 04.
- No change requests or workflow engine (phase 04).
- No CBS connection, drivers or fixtures (phase 05); `z1_cbs` exists empty.
- No evidence writes, Gotenberg rendering or antivirus use (phase 02); the containers only run.
- No business calendar (phase 09); `CalendarDaysClock` only.
- No design tokens, shell or screens (phase 08).
- No Helm chart, K3s or automated failover (phase 30); no release and update screen (phase 25); no support bundle
  or support sessions (phase 25).
