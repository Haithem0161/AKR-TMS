# Phase NN: <Name>

**Goal:** <One sentence describing what this phase delivers>

**Dependencies:** Phase X, Phase Y (or "None")
**Macro-phase:** P0 | P1 | P2 | P3  (docs/ROADMAP.md)
**Complexity:** S | M | L | XL
**Binaries touched:** akr-api | akr-worker | akr-ingestd | akr-assess
**Requirements carried:** <research IDs, e.g. C-02, C-03, S-01>

## 1. SQLx Migration Changes

New and modified tables, enum types, indexes: copy-paste-ready SQL, all `IF NOT EXISTS`, `TIMESTAMPTZ` timestamps,
`id UUID PRIMARY KEY` with **no DB default** (UUIDv7 generated in the application, DR-06). Every table states its
schema, owning role, retention class (R1-R13) and grants (DATA-MODEL.md conventions and section 21). Audit tables
follow DATA-MODEL.md section 2 (partitioning, grants, no UPDATE or DELETE). Migration numbers follow the canonical
chain in roadmap.md.

```sql
-- migration NNNN_<name>.sql
```

## 2. DDD Implementation (Rust)

- **Crate placement**: per the workspace layout pinned in phase 00
- **Domain entity**: `<Entity>` (`Debug, Serialize, Deserialize, FromRow, ToSchema`)
- **Create/Update DTOs**: `Create<Entity>`, `Update<Entity>` (Update fields all `Option<T>`)
- **Repository trait** (domain layer): method signatures
- **Repository impl** (infrastructure layer): `Pg<Entity>Repository` notes (joins, RETURNING)
- **utoipa schemas**: request and response DTOs
- **Route table** (scopes from API.md section 2; markers SU, CR, 4E as in API.md section 3):

| Method | Path | Scope | Description |
|-|-|-|-|
| GET | `/api/v1/<resources>` | `<scope>` | List (cursor) |

- **Router**: `pub fn router() -> Router<AppState>`

## 3. Business Logic

- Service structs and method signatures with trait bounds
- `thiserror` error variants and their problem+json `code` values (API.md section 6)
- Numbered step-by-step logic per method
- **Audit events emitted**: `| Event kind | On | Resource |`; every mutation appears here
- **Cedar actions**: actions touched and the attributes evaluated (tier, case access list, branch scope, SoD pairs)
- **Tipping-off review**: for every screen, task, notice, export or CBS-facing artefact added, what a person outside
  the case's access list can learn from it (the answer must be "nothing")
- Configuration and settings keys used (admin-managed data, never constants)

## 4. Infrastructure Updates

- Postgres `jobs` kinds, outbox events and timers (or "No new jobs")
- Docker compose changes (or "No infrastructure changes needed")
- AppState additions (new repositories and services)
- OpenAPI registration (paths, schemas, tag)
- Network zone rules touched (which binary may reach which service, Z0-Z7)
- Existing entity, schema or route updates required by this phase

## 5. Frontend Impact

- Screens and components added or changed (screen IDs and specs from DESIGN-SYSTEM.md section 8)
- Generated-client hooks consumed
- Or: "Backend-only phase — no frontend impact"

## 6. Verification

1. `make migrate`: migrations apply cleanly, twice (idempotent)
2. `cargo check`: compile-time SQL passes
3. `cargo clippy -- -D warnings`
4. `cargo test`
5. curl-test each endpoint (specific examples and expected responses, including a Cedar-deny 403, a 404 existence
   masking case for suspicion-tier resources and an idempotency-replay case where applicable)
6. Swagger shows the new endpoints; `make generate-api` regenerates the client cleanly
7. The audit trail contains the expected events for the curl session
8. For phases touching the CBS connection: the read-only conformance suite passes (write-granted account refused at
   connect; only single SELECT statements reach the source; load budget enforced)
9. Existing tests still pass

## 7+. Gap Additions

<appended by gap-analysis passes>

## What This Phase Does NOT Touch

- <explicit list of what is left alone>
