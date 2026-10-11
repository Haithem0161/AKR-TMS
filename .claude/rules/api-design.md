---
paths:
  - "backend/api/**"
  - "backend/assess/**"
  - "backend/crates/domain/**"
  - "frontend/orval.config.ts"
  - "frontend/openapi.json"
  - "docs/API.md"
---

# API Design and OpenAPI Documentation Rules

Ported from the digital-banking-system repository (`.claude/rules/api-design.md`, read 2026-10-11) and adapted to
AKR-TMS. Differences from the DBS rule are marked "AKR". The DBS file still described registering handlers by hand
in `#[openapi(paths(...))]`; AKR uses `OpenApiRouter` only, as the DBS code does.

`docs/API.md` section 1 is the source of the conventions and section 3 the catalogue of endpoints. A route that is
not in the catalogue is added there, and in the phase file's route table, in the same change.

## utoipa v5 requirements

### Every handler MUST have

A `#[utoipa::path(...)]` attribute with the method, the path, the tag, every parameter, the request body, every
response it can give (each error as `response = ProblemResponse`) and its security scheme:

```rust
#[utoipa::path(
    post,
    path = "",                                  // relative to the nest: /api/v1/resources
    tag = "resources",
    request_body = CreateResource,
    params(("Idempotency-Key" = String, Header, description = "8 to 255 printable ASCII characters")),
    responses(
        (status = 201, description = "Created", body = Resource),
        (status = 400, response = ProblemResponse),
        (status = 403, response = ProblemResponse),
        (status = 422, response = ProblemResponse),
    ),
    security(("session_cookie" = []), ("bearer_token" = []))
)]
```

### Registration (the most common mistake)

A handler is registered ONCE, through `routes!` on an `OpenApiRouter` (see `rust-backend.md`). That one call adds
the axum route and the OpenAPI path. A handler added with plain `.route(...)` works over HTTP and is INVISIBLE to
the document, so Orval generates nothing for it and the frontend has no typed access. Only infrastructure that must
not be in the document is added that way (`/metrics`, the document itself, Swagger UI).

Do NOT list handlers or schemas in `#[openapi(paths(...), components(schemas(...)))]`: utoipa 5 collects every schema
reachable from a registered path. Reusable `ToResponse` types still need `components(responses(...))`, and a tag
needs its entry in `#[openapi(tags(...))]`.

### Schema NAMES are one flat, workspace-wide namespace (DBS decision 62)

utoipa keys every component by the type's last path segment. Two `ToSchema` types with the same name in different
modules are a SILENT collision: one shape wins, the document is well-formed, the backend compiles, Orval generates
one TypeScript type for both, and the damage lands in whichever screen belongs to the loser.

- Before adding a `ToSchema` type, grep the workspace for the name.
- If the natural name is taken, qualify it (`CaseNoteBody`) or rename in the schema only with
  `#[schema(as = ...)]`. Never leave two types sharing one component name.
- `tsc` catches this only when the two shapes are incompatible AND some code reads a field the survivor lacks.

### Every model used in the API MUST have

- `#[derive(ToSchema)]`; `Serialize` for responses, `Deserialize` for requests.
- `#[serde(deny_unknown_fields)]` on request bodies (AKR): a field the server does not know is a 422, not a silent
  drop.
- Doc comments on the struct and its fields; they become the descriptions in the document.

## Conventions every endpoint follows (AKR, from `docs/API.md` section 1)

- **Base path** `/api/v1` on `akr-api`; `/assess/v1` on `akr-assess`, which publishes its own document. A breaking
  change mints `/v2`; an additive change does not.
- **Idempotency.** Every POST, PUT, PATCH and DELETE requires `Idempotency-Key`. Missing is 400
  `E_IDEMPOTENCY_REQUIRED`; a replay returns the stored response with `x-idempotent-replay: true`; the same key with
  a different body is 422 `E_IDEMPOTENCY_REUSE`; the same key while the first attempt runs is 409
  `E_IDEMPOTENCY_IN_FLIGHT`. An exemption is written in the phase file with its reason and added to the exemption
  list in code; there is no other way out.
- **Concurrency.** A mutation of a versioned resource requires `If-Match: "<version>"`. Missing is 428
  `E_PRECONDITION_REQUIRED`; a mismatch is 409 `E_VERSION_CONFLICT` with the current version. Responses carry `ETag`.
- **Pagination.** Cursor only: `?limit=50&cursor=<opaque>`, at most 200, with `next_cursor`. No offset, and no total
  count on a list that can contain restricted or suspicion-tier rows.
- **Existence masking.** A suspicion-tier resource the caller may not see answers 404 `E_NOT_FOUND` with the same
  body, headers and timing class as a resource that does not exist. Never 403. Lists are filtered after the policy
  decision and counts are computed after the filter.
- **Authorisation.** Two gates: the scope (coarse), then Cedar (deny by default). Every endpoint names its scope in
  the route table and its Cedar action in the phase file.
- **Step-up.** An operation marked **SU** needs `X-StepUp-Assertion`, single-use and bound to its action class.
- **Purpose.** A read of a restricted or suspicion-tier record carries `X-Access-Purpose`; without it the answer is
  422 `E_PURPOSE_REQUIRED`, and the access event is written before the body is sent.
- **Data freshness.** A response that carries data derived from the CBS includes `data_as_of` and `X-Data-As-Of`.
- **Never the CBS on demand.** No endpoint causes a query to the CBS. Everything is served from AKR's own store.
- **Affordances.** Case, alert, report, task and change-request responses include `affordances`: what this user may
  do now, computed by the policy engine. The browser never re-derives permissions.
- **Exports are jobs.** `POST` returns a job id; the result is fetched through a short-lived presigned link served by
  the S3 guard; every export needs step-up, states its purpose and is audited with its row count.
- **Caching.** Restricted and suspicion-tier responses and every export set `Cache-Control: no-store`.
- **Errors.** `application/problem+json` with a stable `code` (`rust-backend.md`). The frontend owns the Arabic and
  English text; the server sends codes.
- **Contract extensions (DR-76).** What the browser must know about an operation travels in the document, never in
  a copy: `x-akr-stepup`, `x-akr-idempotency`, `x-akr-access`, and the scopes. `make generate-api` turns them into
  `frontend/src/api/contract.gen.ts` (phase 08).

## REST conventions

| Action | Method | Path | Handler name | Returns |
|--------|--------|------|--------------|---------|
| List | GET | `/api/v1/<resources>` | `list_<resources>` | `Json<Page<Resource>>` (`items`, `next_cursor`) |
| Get one | GET | `/api/v1/<resources>/{id}` | `get_<resource>` | `Json<Resource>` with `ETag` |
| Create | POST | `/api/v1/<resources>` | `create_<resource>` | `(StatusCode::CREATED, Json<Resource>)` |
| Update | PUT or PATCH | `/api/v1/<resources>/{id}` | `update_<resource>` | `Json<Resource>` with the new `ETag` |
| A domain action | POST | `/api/v1/<resources>/{id}/<verb>` | `<verb>_<resource>` | the resource, or 202 with a job id |

- AKR: there is almost no DELETE. A record is retired, closed, withdrawn or superseded by a named action, and
  retention removes data by policy. A DELETE route exists only where the phase file gives it one.
- AKR: a change that needs a second person does not mutate the resource; it creates a change request and answers
  202 with the request's id (phase 04).
- Ids are UUIDv7. Timestamps are UTC ISO 8601; business dates are `YYYY-MM-DD` and named `business_date` or
  `booking_date`. Money is `{ "amount": "<decimal string>", "currency": "IQD" }`, never a JSON number.
- URLs: plural nouns, kebab-case for several words (`/api/v1/change-requests`).
- Handlers: snake_case, the verb first. Tags: the resource, plural, lowercase.
- Path parameters are `{id}`, not `:id`.

## Swagger UI and the document

- The document: `GET /api/v1/openapi.json`, in every environment. `akr-api openapi` prints the same document
  without starting the server.
- Swagger UI: `/swagger-ui`, outside production only.
- `make generate-api` writes `frontend/openapi.json` from the binary and regenerates the client. Generated files are
  never edited, and both are committed. CI runs it and fails on any difference.
- A phase that changes the API regenerates the client in the same phase, never batched.
- After adding an endpoint: check that it appears in the document with its parameters, its bodies and every error
  response, then regenerate.
