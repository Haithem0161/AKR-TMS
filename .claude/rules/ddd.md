---
paths:
  - "backend/crates/**"
  - "backend/api/**"
  - "backend/worker/**"
  - "backend/ingestd/**"
  - "backend/assess/**"
---

# Domain-Driven Design Rules (Rust/Axum)

Ported from the digital-banking-system repository (`.claude/rules/ddd.md`, read 2026-10-11) and adapted to AKR-TMS.
Differences from the DBS rule are marked "AKR".

We use **DDD with Hexagonal Architecture**: code organised by business domain, with clear layer separation.

## Backend structure: a CARGO WORKSPACE, layered by crate

AKR-TMS is a modular monolith. AKR: four long-running binaries (`akr-api`, `akr-worker`, `akr-ingestd`,
`akr-assess`), the object-store guard `akr-s3guard` and the operator tool `akr-ctl`, over a set of library crates.
The layout is pinned in `docs/plan/phase-00.md` section 2 and every later phase follows it. **Layers are separated by
CRATE boundaries, not only module boundaries**: the compiler enforces the layering, because a crate cannot use a
dependency it does not declare. `akr-domain` cannot import `axum`.

```
backend/
├── Cargo.toml          # [workspace]; every version pinned once in [workspace.dependencies]
├── migrations/         # NNNN_name.sql, one shared chain, applied only by akr-ctl as akr_migrate
├── .sqlx/              # committed offline query cache (make prepare)
├── crates/
│   ├── common/         # akr-common: config, AppError + problem+json, correlation, idempotency, cursor, ETag, ids
│   ├── domain/         # akr-domain: entities, value objects, repository and service TRAITS (ports)
│   ├── db/             # akr-db: Pg* repository IMPLS (sqlx), tx helpers, one pool per database role
│   ├── jobs/           # akr-jobs: JobQueue, worker harness, schedules, OutboxDispatcher, TimerService
│   ├── crypto/         # akr-crypto: seal/open, keyed hash, sign/verify; OpenBao and in-memory implementations
│   ├── objectstore/    # akr-objectstore: the S3 port
│   ├── telemetry/      # akr-telemetry: logs, traces, metrics, the redaction layer
│   ├── authz/          # akr-authz: the Cedar facade
│   ├── audit/          # akr-audit: AuditSink and its sinks
│   ├── dsl/            # akr-dsl: document validators
│   └── sources/        # akr-sources: SourceReader and the CBS adapters; ONLY akr-ingestd may depend on it
├── api/                # akr-api: AppState, routers, OpenAPI, middleware
├── worker/             # akr-worker: job loop, kind registry, schedules, outbox, timers
├── ingestd/            # akr-ingestd: the only binary that talks to the CBS
├── assess/             # akr-assess: the decision gateway
├── s3guard/            # akr-s3guard
└── ctl/                # akr-ctl
```

Inside each layer crate, code is organised by **bounded context**: `crates/domain/src/<context>/`, the matching
adapter in `crates/db/src/<context>/` and the handlers in `api/src/routes/<context>/`. So the axes are **crate =
layer** and **module = bounded context**.

AKR: every binary is **lib + bin**, so integration tests drive the real router and the real loops; they live in
`<binary>/tests/`.

## DDD layer rules

| Layer | Crate | Purpose | May depend on | Allowed crates |
|-------|-------|---------|---------------|----------------|
| **Domain** | `akr-domain` | Entities, repository and service traits, business rules | nothing in the workspace | `serde`, `utoipa::ToSchema`, `sqlx::FromRow` and `sqlx::Type` (derives only), `uuid`, `chrono`, `async-trait`, `thiserror` |
| **Infrastructure** | `akr-db`, `akr-crypto`, `akr-objectstore`, `akr-authz`, `akr-audit`, `akr-sources` | SQLx queries and external clients; implements the domain's ports | `akr-domain`, `akr-common` | `sqlx` (queries), `reqwest`, `async-trait` |
| **Presentation** | `akr-api`, `akr-assess` (its HTTP surface) | Handlers, request and response mapping, middleware | all of the above | `axum`, `utoipa`, `utoipa-axum` |
| **Shared** | `akr-common`, `akr-jobs`, `akr-telemetry` | Config, errors and problem+json, correlation, idempotency, the jobs harness, telemetry | `akr-domain` | `axum` in `akr-common` only for `IntoResponse` on `AppError` and the shared infra router; `sqlx` where the thing is database-backed |
| **Workers** | `akr-worker`, `akr-ingestd` | Job loop, handlers, ingestion loops | all of the above | `tokio` |

**Key principles:**

- The domain layer has **zero infrastructure dependencies**: no sqlx *queries*, no axum types. The `FromRow` and
  `Type` derives are permitted (they are inert trait impls), so a repository impl maps rows straight into a domain
  entity.
- AKR: a port returns `DomainError`, never `sqlx::Error`. The adapter maps the database error (a unique violation
  becomes `Conflict`, a missing row `NotFound`), so no caller above `akr-db` matches on a driver's error type.
- Infrastructure implements the domain's repository traits (dependency inversion).
- Presentation uses domain types for request and response and calls repositories through `AppState`, which holds
  the PORT (`Arc<dyn ResourceRepository>`), not the impl. That is what lets a test inject a fake.
- Each bounded context is self-contained and independently testable.
- `akr-common` is deliberately allowed a narrow `axum` dependency so `AppError` owns its problem+json mapping in one
  place. Nothing else in `common` touches HTTP types.

## Binary and role boundaries (AKR)

- **`akr-sources` is reachable from `akr-ingestd` only.** `backend/deny.toml` bans the CBS drivers and `akr-sources`
  everywhere else, and `tools/ci/check_driver_isolation.sh` checks the dependency trees of the other binaries. A
  handler in `akr-api` that needs source data reads what `akr-ingestd` landed in PostgreSQL.
- **One database role per binary.** `akr-api` and `akr-worker` connect as `akr_app`, `akr-ingestd` as `akr_ingest`,
  `akr-assess` as `akr_assess`; migrations run as `akr_migrate` through `akr-ctl` only. A repository impl works only
  where its role holds the grant, and the migration that creates a table writes its grants.
- **Network zones follow the same lines** (`deploy/zones.yaml`): a binary that has no edge to a service cannot use
  the crate that talks to it, whatever its Cargo.toml says.

## Adding a new bounded context (checklist)

Create one module per layer crate (`backend/crates/domain/src/<context>/`, `backend/crates/db/src/<context>/`,
`backend/api/src/routes/<context>/`) and register each in its crate's `lib.rs` or `mod.rs`. Then, in this order:

1. **Migration**: `backend/migrations/NNNN_<change>.sql` (see `migrations.md`).
2. **Entities** (`akr-domain`):
   - `Resource`: derives `Debug, Serialize, Deserialize, FromRow, ToSchema`
   - `CreateResource`: derives `Debug, Deserialize, ToSchema`
   - `UpdateResource`: derives `Debug, Deserialize, ToSchema` (all fields `Option<T>`)
3. **Repository trait** (`akr-domain`):
   ```rust
   #[async_trait::async_trait]
   pub trait ResourceRepository: Send + Sync {
       async fn list(&self, page: PageRequest) -> Result<Page<Resource>, DomainError>;
       async fn get(&self, id: Uuid) -> Result<Option<Resource>, DomainError>;
       async fn create(&self, tx: &mut PgTx<'_>, input: &CreateResource) -> Result<Resource, DomainError>;
       async fn update(&self, tx: &mut PgTx<'_>, id: Uuid, input: &UpdateResource)
           -> Result<Option<Resource>, DomainError>;
   }
   ```
   AKR: a method that writes takes the caller's transaction, because the audit event and any job or outbox row
   commit with the change (see `jobs.md`). There is no `delete` unless the phase file gives the table one; most
   AKR tables are append-only or retired by the retention job.
4. **Repository impl** (`akr-db`): `PgResourceRepository { pool: sqlx::PgPool }` with `query_as!` implementations.
5. **Handlers** (`akr-api`): each with `#[utoipa::path]`; `pub fn router() -> OpenApiRouter<AppState>`.
6. **Wire into `AppState`** (`api/src/state.rs`): add `pub resource_repo: Arc<dyn ResourceRepository>` and build it
   in `main.rs`.
7. **Register the routes and the OpenAPI together**: `OpenApiRouter::new().routes(routes!(list, create))`, ONE
   `routes!()` call per URL path, grouping that path's methods. The axum route and the OpenAPI path both come from
   the same `#[utoipa::path]`, so they cannot drift. Do NOT list handlers in `#[openapi(paths(...))]`. Add the tag
   to `#[openapi(tags(...))]`.
8. **Verify**: `cargo check`, `make lint`, `make prepare`, `make test`.
9. **Regenerate the frontend client**: `make generate-api`.

## Layer dependency violations (NEVER DO)

- **NEVER** add `axum` to `akr-domain`'s Cargo.toml: the domain is framework-agnostic.
- **NEVER** write a `sqlx::query!` or `query_as!` call in `akr-domain`: that is infrastructure (the derives are
  fine; executing SQL is not).
- **NEVER** import one bounded context's module into another's. Cross-context needs go through a trait in
  `akr-domain` or `akr-common`, wired in `main.rs`.
- **NEVER** put business logic in `api/src/routes/`: handlers map HTTP to domain and back, and nothing else.
- **NEVER** put HTTP concerns (`StatusCode`, `Json`) in `akr-db`.
- **NEVER** let a layer depend "upward" (`akr-db` on `akr-api`, `akr-domain` on anything in the workspace).
- **NEVER** (AKR) add `akr-sources` or a CBS driver to any crate other than `akr-ingestd`.

## Cross-context communication

If two bounded contexts need to interact:

1. Define a shared trait in `akr-domain` (a port) or an outbox event (`jobs.md`).
2. Wire them together in `main.rs` through `AppState`.
3. Do NOT create direct imports between `crates/domain/src/<a>/` and `crates/domain/src/<b>/`.
