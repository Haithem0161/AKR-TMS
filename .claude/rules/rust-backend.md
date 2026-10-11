---
paths:
  - "backend/**/*.rs"
  - "backend/**/Cargo.toml"
  - "backend/Cargo.lock"
---

# Rust Backend Rules (Axum 0.8 + SQLx 0.8)

Ported from the digital-banking-system repository (`.claude/rules/rust-backend.md`, read 2026-10-11) and adapted to
AKR-TMS. Differences from the DBS rule are marked "AKR".

## Project structure

A cargo workspace, layered by crate; the layout and the layer rules are in `ddd.md` and in `docs/plan/phase-00.md`
section 2. Every binary is lib + bin, so integration tests build the REAL router and the real loops.

AKR workspace rules:

- **Every version is pinned once**, in `[workspace.dependencies]` of `backend/Cargo.toml`, and recorded under RQ-01
  in `docs/plan/research.md`. A crate names a dependency with `workspace = true` and adds only the features it needs.
  The toolchain is an exact version in `rust-toolchain.toml`, never "stable".
- **Workspace lints**: `unsafe_code = forbid`; `unwrap_used`, `expect_used`, `todo` and `dbg_macro` are denied. Tests
  may unwrap. Production code returns the error.
- **Cargo runs with `--locked`** (the Makefile does). A changed `Cargo.lock` is reviewed like code.
- **`cargo deny check`** must pass: licences inside the allow-list, no advisory, crates from crates.io only, and the
  CBS drivers and `akr-sources` reachable from `akr-ingestd` alone.
- **Comments say what the code cannot**: a constraint, a reason, a trap. Never how the code came to be written, and
  never a restatement of the next line.

## Handler pattern (MANDATORY)

```rust
/// Short description for Swagger (the operation summary)
///
/// Longer description (optional)
#[utoipa::path(
    get,
    path = "/{id}",                       // relative: the router is nested under /api/v1/<resources>
    tag = "<resources>",
    params(("id" = Uuid, Path, description = "Resource id")),
    responses(
        (status = 200, description = "The resource", body = Resource),
        (status = 404, response = ProblemResponse),
    ),
    security(("session_cookie" = []))
)]
pub async fn get_resource(
    State(state): State<AppState>,        // State ALWAYS first
    principal: Principal,                 // who is asking (phase 01)
    Path(id): Path<Uuid>,
) -> Result<Json<Resource>, AppError> {
    let resource = state.resource_repo.get(id).await?.ok_or(AppError::NotFound)?;
    Ok(Json(resource))
}
```

For a handler with a request body, the body extractor is LAST, and a create returns `201`:

```rust
pub async fn create_resource(
    State(state): State<AppState>,
    principal: Principal,
    Json(input): Json<CreateResource>,
) -> Result<(StatusCode, Json<Resource>), AppError> {
    let resource = state.resource_service.create(&principal, input).await?;
    Ok((StatusCode::CREATED, Json(resource)))
}
```

AKR: a handler maps HTTP to the domain and back. Authorisation, the audit event, the transaction and any job or
outbox row belong to the service it calls, so they cannot be skipped by a second handler that reaches the same
operation.

## Router pattern: `OpenApiRouter` + `routes!`

Each route module exposes a function returning an `OpenApiRouter`, so the axum route and the OpenAPI path both come
from the SAME `#[utoipa::path]` annotation and cannot drift:

```rust
pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        // ONE routes!() call per URL PATH, grouping that path's methods.
        .routes(routes!(list_resources, create_resource))
        .routes(routes!(get_resource, update_resource))
}
```

Rules that bite:

- Axum 0.8 path params are `{id}`, NOT `:id`. The old form PANICS at router construction, so the process dies at
  start. Wildcards are `{*rest}`.
- Mixing different paths in one `routes!()` call misroutes.
- `route`, `route_service`, `nest_service` and `fallback` on `OpenApiRouter` are plain axum pass-throughs and add
  NOTHING to the document.
- Inside `OpenApiRouter::nest("/prefix", ..)` use relative paths (`path = ""`, `path = "/{id}"`); `context_path`
  would apply the prefix twice.
- `with_state` must be the LAST call.
- AKR: product routes live under `/api/v1`. `/healthz` and `/readyz` are outside it, on every binary. `/metrics` is
  on the management listener only and is not in the document. The document is served at `/api/v1/openapi.json`;
  Swagger UI exists outside production only.

## Model pattern (MANDATORY)

```rust
/// Read model: database rows and API responses
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Resource {
    pub id: Uuid,
    pub code: String,
    pub note: Option<String>,             // nullable column -> Option<T>
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create body: writable fields only, no id and no timestamps
#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateResource {
    pub code: String,
    pub note: Option<String>,
}

/// Update body: every field optional
#[derive(Debug, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct UpdateResource {
    pub note: Option<String>,
}
```

AKR: the id is made by the application (`akr_common::ids::new_id()`, UUIDv7), never by the database. A secret, a
key or a sealed value never derives `Debug`, `Serialize` or `ToSchema`; it is a type that prints as `[redacted]` and
zeroises on drop.

## SQLx query rules

- ALWAYS `sqlx::query_as!` for rows, `sqlx::query!` for statements with no row to map, `sqlx::query_scalar!` for one
  column. They are checked at compile time against `backend/.sqlx` (`make prepare` after changing SQL).
- ALWAYS list columns in SELECT; never `SELECT *`.
- `RETURNING` on INSERT and UPDATE to get the row back.
- A nullable column is `Option<T>` in the struct.
- AKR: table names are schema-qualified in every query (`admin.jobs`).
- AKR: an update of a mutable aggregate compares the version: `WHERE id = $1 AND version = $2`, and zero rows is
  `AppError::VersionConflict`.
- AKR: a query built at run time (`sqlx::query`) is allowed only where the SQL cannot be known at compile time (the
  provisioner, the database checks), and never with a value spliced into the text: identifiers are validated and
  quoted, values are bound.
- AKR: `akr-db` holds one pool per database role. A repository is built on the pool of the role that holds its
  grants; nothing borrows a wider role's pool.

## Error handling

`AppError` lives ONCE, in `akr_common::errors`, and owns its `IntoResponse`. Do NOT define per-domain HTTP error
enums: a domain has a `thiserror` enum of its own, and a `From` impl maps it to an `AppError` variant. A variant's
`code()` is part of the API contract.

Every error is RFC 9457 **`application/problem+json`** with a stable machine `code` that clients and tests branch
on (never the human text) and the `correlation_id`. The catalogue (AKR):

| Variant | HTTP | code |
|-|-|-|
| BadRequest | 400 | `E_BAD_REQUEST` |
| IdempotencyRequired | 400 | `E_IDEMPOTENCY_REQUIRED` |
| Unauthenticated | 401 | `E_UNAUTHENTICATED` |
| StepUpRequired | 401 | `E_STEPUP_REQUIRED` |
| Scope | 403 | `E_SCOPE` |
| Forbidden | 403 | `E_FORBIDDEN` |
| SelfApproval | 403 | `E_SELF_APPROVAL` |
| NotFound | 404 | `E_NOT_FOUND` (also the answer for a resource the caller may not know exists) |
| VersionConflict | 409 | `E_VERSION_CONFLICT` (the body carries `current_version`) |
| IdempotencyInFlight | 409 | `E_IDEMPOTENCY_IN_FLIGHT` |
| NotAllowedInState | 409 | `E_NOT_ALLOWED_IN_STATE` |
| PayloadTooLarge | 413 | `E_PAYLOAD_TOO_LARGE` |
| Validation | 422 | `E_VALIDATION` (with `errors[]`, each a JSON pointer and a code) |
| IdempotencyReuse | 422 | `E_IDEMPOTENCY_REUSE` |
| BelowFloor | 422 | `E_BELOW_FLOOR` |
| PreconditionRequired | 428 | `E_PRECONDITION_REQUIRED` |
| RateLimited | 429 | `E_RATE_LIMITED` (with `Retry-After`) |
| Internal | 500 | `E_INTERNAL` (the correlation id only, NEVER any detail) |
| Unavailable | 503 | `E_UNAVAILABLE` |

Rules that are easy to get wrong:

- **5xx never leaks detail.** A database error must not expose schema or query text; the correlation id is how it
  is found. Log the full source chain server-side.
- **Logs never carry classified values** (AKR). The telemetry layer redacts fields by the classification register,
  and an error text built with `format!` from a customer's data defeats it. Log ids.
- **Build the response explicitly**: set the status, then `insert` the `application/problem+json` content type
  (`insert` replaces; `append` would duplicate).
- For utoipa, reference the shared error as `(status = 4xx, response = ProblemResponse)` with no `description`
  beside it. `ProblemResponse` is a newtype so the document has one `$ref` and Orval generates one shared type.
- A fallback and a method-not-allowed fallback answer in problem+json too; no route returns an empty 404.

`thiserror` v2: `#[error("...")]` sets Display; `#[from]` generates `From` and enables `?`; `#[source]` keeps the
chain without showing it to the client.

## Axum extractor ordering (CRITICAL)

```
1. State(state): State<AppState>
2. principal: Principal                 (phase 01)
3. Path(id): Path<Uuid>
4. Query(params): Query<ListParams>
5. headers / typed headers (If-Match, Idempotency-Key)
6. Json(body): Json<CreateInput>        -- the body, ALWAYS LAST
```

## Async and runtime rules

- NEVER hold a `Mutex` or `RwLock` guard across an `.await`.
- `AppState` derives `Clone` and holds `Arc<dyn Port>` values.
- NEVER open a connection outside the pools.
- `tokio::spawn`, not `std::thread::spawn`; `spawn_blocking` for CPU-heavy work.
- AKR: every long-running binary stops on one cancellation token (`infra::shutdown_signal`), finishes what it
  holds, flushes telemetry and exits 0. A loop that ignores the token turns a restart into a kill.
- AKR: a binary reads its configuration once, at start, through `CoreConfig::from_env`, validates it and refuses to
  start on a value it does not understand. It reads its secrets from OpenBao, never from the environment.
- AKR: time comes from the clock port and ids from `akr_common::ids`, so a test can fix both.

## Middleware stack (the live definition is `api/src/app.rs`)

**Ordering is the trap.** Chained `.layer()` calls WRAP what was added before, so the LAST `.layer()` is the
OUTERMOST. AKR uses chained calls everywhere, with a comment that names the order; never mix in a layer tuple
(whose FIRST element is outermost).

Outermost to innermost, as built: body limit, sensitive headers (`Authorization`, `Cookie`, `X-StepUp-Assertion`),
correlation, trace span, metrics, panic guard, then on the `/api/v1` routes only: principal, then idempotency, then
the handler.

- Every tower-http middleware is feature-gated; enable the feature or it does not compile.
- Use `tower_http::timeout::TimeoutLayer`, not tower's (tower's changes the error type).
- Prefer axum's `DefaultBodyLimit` over `RequestBodyLimitLayer` (the latter wraps the response body type too).
- Auth and authz wrap the routes they protect, so a path that matches nothing is a 404 from the fallback and never a
  401.
- Label spans and metrics by the route template (`MatchedPath`, `/cases/{id}`), never the raw URI: a raw path makes
  one time series per id, and it puts an id into the traces.
- `CompressionLayer`'s default predicate skips `text/event-stream`; do not override it, or SSE buffers.
- A URI rewrite must wrap the FINISHED router; a layer added through `Router::layer` runs after routing.
- axum 0.8 has no automatic trailing-slash redirect, and `nest` is trailing-slash-exact.
- Inside a nested router `req.uri().path()` is the nest-STRIPPED path; use `OriginalUri` when the full path matters
  (the idempotency key's scope depends on it).
