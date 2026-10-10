# API Plan — Conventions and Endpoint Catalogue

Status: planning draft (2026-10-07). The OpenAPI document generated from utoipa annotations is the binding
contract: every handler is annotated and registered, and Orval regenerates the frontend client from it (DBS rule).
This document is the design intent the spec must realise. Module numbers refer to [MODULES.md](MODULES.md); tables
to [DATA-MODEL.md](DATA-MODEL.md); document bodies to [DSL.md](DSL.md).

Sources: DBS `docs/API.md` (conventions, scopes, catalogue shape), MODULES API sketches and s.22, research 28 s.16.4
(case API, affordances), 25 s.15 (assessment contract), 30 (security), 31 (UX budgets).

## 1. Conventions

- **Base paths.** `/api/v1` on `akr-api` (the browser, through the BFF, and automation). `/assess/v1` on
  `akr-assess` (phase 2, the bank's payment gateway only, mTLS). `akr-worker` and `akr-ingestd` expose no business
  API: only `/healthz`, `/readyz` and `/metrics`, reachable from the operations zone. Breaking changes mint `/v2`;
  additive changes do not.
- **Authentication.**
  - Browser: the DBS BFF. Kratos session cookie (httpOnly, SameSite=Strict) plus a CSRF token bound to the session.
    MFA is mandatory for every user (ARCHITECTURE s.13).
  - Automation: `Authorization: Bearer <token>`, a Hydra-issued JWT (client credentials) or a personal access token
    (`admin.api_tokens`, hash lookup). Tokens carry a subset of scopes (s.2).
  - Payment gateway (phase 2): mutual TLS with a bank-issued client certificate, mapped to a Cedar principal with the
    single scope `assess.call`.
- **Step-up.** Actions marked **SU** in the catalogue need `X-StepUp-Assertion: <id>`, a single-use AAL2 assertion
  from `POST /auth/step-up/verify`, valid 5 minutes and bound to the action class it was issued for. It is consumed
  on use; a replay of the same assertion is refused (`E_STEPUP_REQUIRED`). Opening a suspicion-tier record, every
  decision, every approval and every export need one (ARCHITECTURE s.13).
- **Authorisation.** Two gates: the token scope (coarse), then Cedar with (principal, action, resource, context),
  deny by default. Every decision is audited. Segregation-of-duties pairs are `forbid` rules: a maker never checks,
  a proposer never approves, an analyst never QA-reviews their own case (ARCHITECTURE s.11).
- **Existence masking.** A suspicion-tier resource the caller may not see answers **404 `E_NOT_FOUND` with the same
  body, headers and timing class as a resource that does not exist**, never 403. Lists and searches are filtered
  after Cedar; counts are computed after the filter; exports, error messages and SSE events never reveal a masked
  resource. Inference tests fail the build when a count, search, export or error leaks one.
- **Errors.** RFC 7807 `application/problem+json`: `{ type, title, status, code, errors?, correlation_id }`. `code`
  is a stable machine string from s.6; the frontend owns the Arabic and English text. Validation errors carry
  `errors[]` with JSON-pointer paths, the DSL.md s.7 shape, used API-wide.
- **Idempotency.** Every POST, PUT, PATCH and DELETE requires `Idempotency-Key` (8-255 printable ASCII; missing ->
  400 `E_IDEMPOTENCY_REQUIRED`). A replay returns the stored response byte for byte plus `x-idempotent-replay: true`;
  the same key with a different body -> 422 `E_IDEMPOTENCY_REUSE`; the same key while the first attempt still runs ->
  409 `E_IDEMPOTENCY_IN_FLIGHT`. Only successful responses are stored, so a transient failure stays retryable. The
  request hash covers method, full path, actor and raw body bytes (DBS migration 0003 pattern; `admin.idempotency_keys`).
- **Concurrency.** Mutations on versioned resources require `If-Match: "<version>"`; a missing header -> 428
  `E_PRECONDITION_REQUIRED`; a mismatch -> 409 `E_VERSION_CONFLICT` with the current version in the body. Responses
  carry `ETag`.
- **Pagination.** Cursor only: `?limit=50&cursor=<opaque>` (maximum 200); responses carry `next_cursor`. No offset
  pagination and no total counts on lists that can contain restricted or suspicion-tier rows.
- **Data freshness.** Every response that carries data derived from the CBS includes `data_as_of` (the source business
  date of the newest batch behind it, per branch where the CBS keeps one, never the AKR clock, DR-39) in the body and as
  `X-Data-As-Of`, with a `stale` flag when the source has stopped moving; the UI shows it on every screen and export (C-29).
- **Never the CBS on demand.** No endpoint causes a query to the CBS by customer, account or case; everything is
  served from AKR-TMS's own store, and an automated test enforces it (C-35).
- **Affordances.** Case, alert, report, task and change-request responses include `affordances`: the actions this
  user may take now, computed by Cedar, so the browser never re-derives permissions (research 28 s.16.4).
- **Correlation.** `X-Correlation-Id` accepted or generated; echoed in responses, logs, traces and audit entries.
- **Values.** Timestamps UTC ISO 8601; business dates `YYYY-MM-DD` in the bank's zone, named `business_date` or
  `booking_date`; money `{ "amount": "<decimal string>", "currency": "IQD" }`; ids UUIDv7; Western digits in data.
  `Accept-Language: ar | en` selects server-rendered text only (reason-code templates, generated plain language,
  PDFs); codes never change with language.
- **Rate limits.** Per-session and per-token buckets in Valkey; 429 with `Retry-After`. Name search and exports have
  their own stricter buckets.
- **Caching.** Restricted and suspicion-tier responses and every export set `Cache-Control: no-store`.
- **Purpose.** A read of a restricted or suspicion-tier record states its purpose in `X-Access-Purpose` (a purpose
  code) with optional free text in `X-Access-Reason` (percent-encoded UTF-8, at most 500 characters); without it the
  answer is 422 `E_PURPOSE_REQUIRED`. Every sensitive read writes an access event before the body is sent; such
  operations carry the OpenAPI extension `x-akr-access` (DR-46).
- **Exports.** Exports are jobs: `POST` returns a job id; the result is fetched through a short-lived presigned link
  served by the S3 guard; every export needs **SU**, states its purpose, and is audited with its row count.
- **SSE.** `GET /api/v1/events` (session auth). Event types: `queue.counts`, `task.created`, `task.updated`,
  `case.updated`, `clock.warning`, `notification.new`, `job.updated`, `ingest.status` (operations roles only),
  `sse.reset`. Events carry ids only and go to the per-user channel of users who can see the resource; clients
  refetch through TanStack Query invalidation. Reconnect with `Last-Event-ID`. As planned, the one hub is phase 17's
  `CountsHub` on `GET /events/counts` (3.12); `notification.new {notification_id}` travels on it (phase 22, DR-214);
  the path is settled in gap pass 1.
- **Performance budgets** (tech-stack.md, UX-16): first content within 1 s at 50 concurrent analysts; type-ahead
  p95 300 ms; list endpoints p95 under 300 ms at the S profile.

## 2. Scopes

Coarse gates carried by sessions (from role grants) and tokens. Cedar remains the fine-grained authority.

| Scope | Gates | Roles (MODULES s.0) |
|---|---|---|
| `self` | Own profile, preferences, sessions, tokens, notifications, inbox | Everyone |
| `queue.work` | Queues, alerts, cases (read and work), party pages, transactions, network, RFI drafts, proposals | A1, A2, M |
| `case.decide` | Case decisions and approvals, reopen, access-list changes | A2, M |
| `screening.work` | Screening alerts and decisions, name search, pair memory and good-guy proposals | SS, M |
| `kyc.work` | Ratings, overrides, KYC reviews, PEP register, ownership, exit register | KYC, M |
| `rfi.desk` | RFI desk | RFI, M |
| `qa.review` | QA samples and reviews | QA, M, CH, IA (CH and IA review MLRO decisions; phase 19) |
| `ops.tasks` | Neutral operations tasks and their completion | OPS |
| `branch.window` | Branch items, observations, confidential reports, liaison monthly report | L |
| `report.prepare` | Report drafts, validation, CTR batches, the goAML checker | A2, M |
| `report.file` | Report packages, filing records, official letters | M |
| `config.author` | Drafts of scenarios, parameter sets, mappings, code maps, rating models, segments, reference data, workflow definitions; creating change requests | CFG, DS (mappings and code maps), MRO (models) |
| `config.approve` | Decisions on change requests | CHK, M, CH, MRO (as validator) |
| `data.steward` | Sources, discovery, batches, quarantine, reconciliation, data quality, drift | DS |
| `model.risk` | Model registry, validation, inventory | MRO |
| `dashboard.read` | Command centre, metrics, board packs | M, CH, IA |
| `audit.read` | Audit events, chain status, access history | IA, SEC; M for their own scope |
| `assessor.read` | Assessor workspace (time-boxed grant) | AS, IA |
| `admin.users` | Users, grants, delegations, offboarding, directory links | ADM |
| `admin.policies` | Cedar bundles and simulation | ADM, SEC |
| `admin.system` | Settings, flags, jobs, releases, ops cockpit | ADM |
| `security.ops` | Break-glass approvals, access reviews, credential-reset confirmation, access matrix, support-session approval | SEC |
| `support.readonly` | A support session opened by the bank (time-boxed); no route requires it: `VS` reads only its own profile (`self`; phase 25, DR-237) | VS |
| `siem.pull` | `GET /siem/events` only; the route is not mounted for a client without it (phase 25, DR-245) | Service principals (`ClientCredentials`) only |
| `assess.call` | `/assess/v1` only | Gateway client certificate |
| `capacity.plan` | Capacity inputs and simulation runs (phase 31, DR-353) | M |
| `directory.sync` | `POST /directory/events` only; HMAC-verified webhook (phase 31, DR-352) | Service principal `polis-directory` only |

Personal access tokens are created with **SU**, expire within 90 days and may carry only read scopes and
`data.steward`; they can never hold `case.decide`, `config.approve`, `report.file`, `screening.work`, `kyc.work` or
any `admin.*` scope, because decisions are a person's interactive act with step-up (D7).

## 3. Endpoint catalogue

Legend in Description: **SU** step-up assertion required; **CR** the call creates or updates a change request
(DSL.md Part D) instead of changing the live object; **4E** four-eyes enforced (Cedar `forbid` plus a database
check); **[P0]** and **[P2]** mark endpoints outside P1. Paths are relative to `/api/v1`. Every mutation carries
`Idempotency-Key`, `If-Match` where the resource is versioned, and writes the audit event named in its phase file.

### 3.1 Identity and self (M1) [P0]

Normative list: docs/plan/phase-01.md s.2 (22 routes). Login, second factor, logout, heartbeat, step-up, password
change, TOTP start and token creation are exempt from `Idempotency-Key` because their bodies or responses carry
secrets the ledger must not hold (DR-28). Recovery is an administrator-set temporary password (s.3.2), not a
self-service flow (DR-30).

| Method | Path | Scope | Description |
|---|---|---|---|
| POST | /auth/login | none (pre-auth) | Password step; returns `mfa_required` or `setup_required` |
| POST | /auth/mfa | none (flow) | Second factor (WebAuthn or TOTP); mints the CSRF cookie |
| GET | /auth/session | self | Current session: AAL, idle and absolute expiry, pending steps, roles, branch scope |
| POST | /auth/session/heartbeat | self | Record user activity for the idle clock (rate-capped) |
| POST | /auth/logout | self | End the session |
| POST | /auth/step-up | self | Start a step-up challenge for an action class |
| POST | /auth/step-up/verify | self | Complete it; returns a single-use assertion id (5 minutes) |
| GET | /me | self | Profile, roles with delegations, scopes, pools, factors, pending steps |
| PATCH | /me/preferences | self | Language, digit display, density, default queue, shortcuts, theme |
| POST | /me/policy-acknowledgement | self | Acknowledge the current confidentiality undertaking (SP-101) |
| GET | /me/sessions | self | Own sessions |
| DELETE | /me/sessions/{id} | self | Revoke a session |
| GET | /me/credentials | self | Own second factors |
| POST | /me/credentials/password | self | Change password (first login, recovery, expiry or voluntary) |
| POST | /me/credentials/totp | self | Start TOTP enrolment (secret and QR, `no-store`) |
| POST | /me/credentials/totp/confirm | self | Confirm TOTP with a code on the same flow |
| POST | /me/credentials/webauthn | self | Start security-key registration |
| POST | /me/credentials/webauthn/confirm | self | Finish security-key registration with a label |
| DELETE | /me/credentials/{id} | self | Remove a factor; **SU**; 409 `E_MIN_FACTORS` if none would remain |
| GET | /me/tokens | self | Own personal access tokens |
| POST | /me/tokens | self | Create a token with a scope subset; **SU** |
| DELETE | /me/tokens/{id} | self | Revoke a token |
| GET | /me/notifications | self | In-app notifications, newest first, cursor, `unread` filter [P1, phase 22] |
| POST | /me/notifications/{id}/read | self | Mark one of my notices read; another user's id is 404 [P1, phase 22] |
| GET | /me/notifications/unread-count | self | My unread count [P1, phase 22, added] |
| GET | /me/notification-preferences | self | My e-mail mode (`immediate` or `digest`) [P1, phase 22, added] |
| PUT | /me/notification-preferences | self | Set my e-mail mode; `If-Match` [P1, phase 22, added] |

### 3.2 Organisation, users and roles (M2) [P0]

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /org/branches | self | Branches with governorate, region, liaison and alternate |
| POST | /org/branches | admin.users | Create a branch (normally imported from the CBS) |
| PATCH | /org/branches/{id} | admin.users | Assign liaison officer and alternate |
| GET | /org/pools | self | Work pools |
| POST | /org/pools | admin.users | Create a pool (skills, branch scope, tier) |
| PATCH | /org/pools/{id} | admin.users | Change members or routing attributes |
| GET | /admin/users | admin.users | Users (cursor, filters) |
| GET | /admin/users/{id} | admin.users | User detail: account state, grants, delegations, token register, factor count, pending reset |
| POST | /admin/users | admin.users | Create a person's account with a temporary password shown once; **SU** |
| PATCH | /admin/users/{id} | admin.users | Edit profile, branch scope, clearance |
| POST | /admin/users/{id}/grants | admin.users | Propose a role grant; **CR** (`cr_access`: ADM proposes, SEC approves); 202 |
| DELETE | /admin/users/{id}/grants/{grant_id} | admin.users | Revoke a grant at once (revocation needs no second approver) |
| POST | /admin/users/{id}/unlock | admin.users | Unlock an account locked after failed sign-ins |
| POST | /admin/users/{id}/disable | admin.users | Suspend an account with a reason code (grants kept) |
| POST | /admin/users/{id}/enable | admin.users | Re-enable after dormancy, expiry or suspension; **SU** |
| POST | /admin/users/{id}/reset-credentials | admin.users | Request a temporary password and optional second-factor reset (recovery, DR-30, DR-49); **SU**; **4E**: ADM requests, SEC confirms via /admin/credential-resets/{id}/confirm |
| GET | /admin/credential-resets | security.ops | Pending credential resets and those of the last 30 days |
| POST | /admin/credential-resets/{id}/confirm | security.ops | Confirm a reset; returns the temporary password once; **SU**, **4E** (confirmer is not the requester) |
| POST | /admin/users/{id}/offboard | admin.users | Disable identity, revoke sessions and tokens, end grants, sweep open tasks; **SU** |
| GET | /org/delegations | self | Delegations to and from me; all for admin |
| POST | /org/delegations | self | Delegate own roles during leave within the authority matrix; **SU** |
| DELETE | /org/delegations/{id} | self | End a delegation |
| GET | /org/authority-matrix | self | Who may decide what: grants by decision type and priority class, delegation rules (phase 09, DR-83) |
| PUT | /org/authority-matrix | admin.users | Propose a new matrix; `If-Match`; **CR** on `cr_authority` (MLRO signs, CHK approves) |
| GET | /admin/access-reviews | security.ops, dashboard.read | Quarterly access reviews, newest first; `dashboard.read` admits M, who decides business-role items (phase 25) |
| GET | /admin/access-reviews/{id} | security.ops, dashboard.read | Review detail: state, snapshot identity, items with their decisions, paged (phase 25, added) |
| POST | /admin/access-reviews | security.ops | Start a review; 202, the snapshot of `AccessMatrixBuilder::build(at)` is built by job `access_review_snapshot` as an evidence object (phase 25) |
| POST | /admin/access-reviews/{id}/items/{item_id} | security.ops, dashboard.read | Confirm, flag or propose the revocation of one grant; never one's own grant; a revocation is a **CR** on `cr_access_adm` (`role_grant` op `revoke`); `If-Match` (phase 25, added) |
| POST | /admin/access-reviews/{id}/sign | security.ops | Sign once every item is decided; signer is not the starter; **SU** `approval` (phase 25) |
| POST | /directory/events | directory.sync | Directory event from Ory Polis (create, update, deactivate, activate); HMAC-verified webhook, not reachable through the edge; a deactivation disables, never offboards; flag `pack.polis` (phase 31, added) |
| GET | /directory/users | admin.users | Directory users with link state; names to ADM only, each read an access event (phase 31, added) |
| POST | /directory/users/{id}/link | admin.users | Link a pending directory user to an active account with the same e-mail; **SU** `identity_admin`; `If-Match` (phase 31, added) |

### 3.3 Authorisation, tiers and access lists (M3) [P0]

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /admin/policies | admin.policies | Active Cedar bundle, schema and version |
| PUT | /admin/policies | admin.policies | Propose a bundle (a staged release or an authored one); **CR** (`cr_access`, or `cr_access_adm` when proposed by SEC); runs as the shadow bundle while the request is open and cannot be approved before `change.policy_shadow_min_h`; `If-Match` (active version); 202 |
| POST | /admin/policies/simulate | admin.policies | Evaluate (principal, action, resource) against the active or a staged bundle, with the attribute trace; real ids of masked kinds refused (DR-48) |
| GET | /admin/access-matrix | security.ops, audit.read | Generated access matrix: role to actions from the active bundle, and every principal's grants |
| GET | /access/purpose-codes | self | Active purpose codes and the tiers they apply to (DR-52) |
| GET | /access/break-glass | self, security.ops, dashboard.read | Requests and grants with status and review state; a requester sees own rows only; no case id outside the requester's own rows (phase 25, added) |
| POST | /access/break-glass | self | Request read-only access to one case with a reason code (A2 only); **SU** `break_glass`; the same 202 whether or not the case exists (phase 25) |
| POST | /access/break-glass/{id}/approve | security.ops, dashboard.read | Approve as CH or SEC (one of each, neither the requester; at most four hours); **SU** `break_glass`; **4E** (phase 25) |
| POST | /access/break-glass/{id}/end | self | End early (the requester or SEC); `If-Match` (phase 25) |
| POST | /access/break-glass/{id}/review | security.ops | Record the review within 24 hours of the end, outcome code (phase 25, added) |
| GET | /records/{kind}/{id}/access-history | audit.read | Who read or changed a record, with purpose; masked 404 for a suspicion-tier record outside M (phase 24) |
| GET | /access/control-report | dashboard.read, security.ops | Monthly tipping-off control report, counts only, small counts suppressed (phase 24, added) |

### 3.4 Audit, evidence vault and retention (M4) [P0 capture, P1 packs]

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /audit/events | audit.read | Audit events (filters, cursor, `purpose` required); every search is logged as `audit.searched`; suspicion-tier rows are left out for IA and SEC (phase 24) |
| GET | /audit/events/{id} | audit.read | One event with its chain position; masked 404 for a suspicion-tier row; the key carries `occurred_at` (phase 24, added) |
| GET | /audit/chain-status | audit.read | Hash-chain and anchor status per partition and stream (phase 24) |
| GET | /audit/anchors | audit.read | Signed Merkle anchors, cursor (phase 24) |
| POST | /audit/verify-runs | audit.read | Queue a verify run over a range; `Idempotency-Key`; 202 (phase 24) |
| GET | /audit/verify-runs/{id} | audit.read | Status and result, counts and finding kinds only (phase 24) |
| GET | /retention/holds | audit.read | Legal holds, cursor page; for IA and SEC case holds are omitted and party holds show scope `restricted` (phase 23) |
| POST | /retention/holds | report.file | Place a legal hold (party, case or range); **SU** class `decision`; `Idempotency-Key` (phase 23) |
| GET | /retention/holds/{id} | audit.read | One hold; a case hold is `E_NOT_FOUND` for everyone but M (phase 23, added) |
| POST | /retention/holds/{id}/release | report.file | Start the workflow `legal_hold_release`; `If-Match`; the approval is the phase 04 task action with **SU** and **4E** (phase 23) |
| GET | /retention/report | audit.read, admin.system | What is due by class and what is held, from the latest completed run; `held` is null for IA, SEC and ADM (phase 23) |
| POST | /retention/dry-runs | admin.system | Simulate a retention run; 202 with the run id; nothing deleted (phase 23) |
| GET | /retention/runs | audit.read, admin.system | Runs, newest first, dry and real (phase 23, added) |
| GET | /retention/runs/{id} | audit.read, admin.system | One run with its items; `held` is null for IA, SEC and ADM (phase 23, added) |
| GET | /retention/certificates | audit.read | Deletion certificates: run, class, kind, counts, digest, signature; never a clear key or value (phase 23) |

Phase 23 owns the rows marked "phase 23" (docs/plan/phase-23.md s.2; 9 routes, DR-184); the `/audit` rows above are
phase 24's. Evidence content is served by its owner (for example
`GET /cases/{id}/evidence/{evidence_id}/content`, phase 18); there is no generic evidence route (DR-380).

### 3.5 Sources, discovery and ingestion (M5) [P0 discovery, P1 production]

Phase 05 builds the source and discovery routes (docs/plan/phase-05.md s.2, DR-59 to DR-63); phase 11 builds the
watermark, batch, quarantine and freshness routes and extends resume (docs/plan/phase-11.md s.2, DR-89 to DR-93); the
re-read window (DR-37) and the automatic catch-up (DR-90) cover gaps, and no phase builds an operator-planned
backfill (DR-380). **CR** routes answer 202 with a change request on `cr_access`.

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /sources | data.steward, security.ops | Registered sources with tier CT1, CT2 or CT4 (CT3 from phase 11), proved read-only level, state, last contact, open pause |
| POST | /sources | data.steward | Register a source (engine, tier, adapter pack, CBS schemas); the credential is set out of band by `akr-ctl` into OpenBao (DR-59); budget version 0 = the discovery default |
| GET | /sources/{id} | data.steward, security.ops | Detail: budget in force, latest check summary, open pause, documents, credential status; freshness and last batch from phase 11 |
| PATCH | /sources/{id} | data.steward | Edit non-governed attributes (name, product, adapter pack, CBS schemas, hints) |
| POST | /sources/{id}/probe | data.steward | Queue in akr-ingestd (202 with the check id): connect, check grants from the session's effective privileges and active roles (fail closed on any write privilege or DBA-class role or privilege, each named in the result), list every database link the account can use (refused unless a bank-signed exception exists, DR-40), TLS handshake; the character-set round trip (DR-36) runs in the catalogue run |
| GET | /sources/{id}/grants | data.steward | Latest grant-check result and history; `?detail=true` for every privilege |
| GET | /sources/{id}/load-budget | data.steward | The signed load budget in force and its history |
| PUT | /sources/{id}/load-budget | data.steward | Propose a new budget signed by the bank's DBA (signed document required); **CR** `load_budget` |
| PUT | /sources/{id}/access | data.steward | Propose the approved object list and bank-signed exceptions; **CR** `source_access` |
| POST | /sources/{id}/pause | data.steward, security.ops | Pause now (operator, bank request, incident): running statements cancelled, sessions closed; **SU** |
| POST | /sources/{id}/resume | data.steward, security.ops | Resume after a fresh passing grant check (202); a halted source (restore, failover or delivery gap) needs a restore decision (`false_alarm`, `restored` with the point per table and branch, `gap_resolved`) and **SU** `backfill`; catch-up is automatic (phase 11, DR-90) |
| POST | /sources/{id}/documents | data.steward | Upload a signed document (budget, access approval, exception); staged and scanned as evidence |
| GET | /sources/{id}/documents/{doc_id} | data.steward, security.ops | Stream a source document |
| GET | /sources/{id}/watermarks | data.steward | Watermark registry with lookback windows [phase 11] |
| GET | /sources/{id}/batches | data.steward | Batches (as-of marker, rows, mapping versions, status) [phase 11] |
| GET | /batches/{id} | data.steward | Batch detail with reconciliation results (null until phase 12) [phase 11] |
| GET | /freshness | dashboard.read, data.steward | Freshness per source, table and branch: class F1-F5, `data_as_of`, lag, stale and halted state [phase 11, DR-93] |
| GET | /quarantine | data.steward | Quarantined rows (reason, source, status) [phase 11] |
| POST | /quarantine/{id}/retry | data.steward | Retry a `row_failed` row after a fix; `If-Match`; 202 (a `row_conflict` is reviewed in phase 12) [phase 11] |
| POST | /quarantine/{id}/exclude | data.steward | Exclude with reason; appears in the coverage statement; `If-Match`; **SU** `decision`; refused for unmapped transaction types and card reasons (phase 12) [phase 11] |
| POST | /quarantine/{id}/assign | data.steward | Set the owner (an active steward); `If-Match` [phase 12, DR-97] |
| GET | /discovery/runs | data.steward | Runs per source, newest first [P0] |
| POST | /discovery/runs | data.steward | Queue a catalogue run (dictionary only) or a profile run (approved objects only) [P0] |
| GET | /discovery/runs/{id} | data.steward | Progress, findings and the read-only proof summary [P0] |
| POST | /discovery/runs/{id}/cancel | data.steward | Cancel; the run stops after the current statement [P0] |
| GET | /discovery/profiles | data.steward | Profiles per object (counts and method, liveness, candidate class, null rates, lengths; no personal values) [P0] |
| GET | /discovery/field-availability | data.steward | Canonical attribute availability and quality per source [P0] |
| GET | /discovery/code-lists | data.steward | The CBS's own code lists captured from its dictionaries [P0] |
| GET | /discovery/integration-profile | data.steward | Exportable integration profile and discovery checklist (JSON; the PDF is a pack, 3.16) [P0] |
| GET | /discovery/field-candidates | data.steward | Ranked candidate columns for attributes discovery could not place, with reasons; accepted as a hint by `PATCH /sources/{id}` (phase 06) [P0] |

### 3.6 Mapping and canonical model (M6) [P0 draft, P1]

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /canonical/catalogue | config.author, config.approve, dashboard.read | The 29 canonical entities with columns, classification, nullability, required and monitored flags (phase 10) |
| GET | /mappings | config.author, config.approve, dashboard.read | Mappings per source and entity with the published and open version |
| POST | /mappings | config.author | Create a mapping family (DS in Cedar, DR-86) |
| GET | /mappings/{id} | config.author, config.approve, dashboard.read | Detail, version list and drift affecting it |
| POST | /mappings/{id}/versions | config.author | Open a draft version |
| PUT | /mappings/{id}/versions/{v} | config.author | Replace the draft document (DSL.md Part B); `If-Match` on the revision |
| POST | /mappings/{id}/versions/{v}/validate | config.author | Schema, semantic and compile validation; draft -> testing |
| POST | /mappings/{id}/versions/{v}/test | config.author | Sample run on read-only rows (counts and shapes only, DR-86); 202 |
| POST | /mappings/{id}/versions/{v}/preview | config.author | Shape rows beside source shapes; 202; preview masked except DS and M |
| GET | /mappings/{id}/versions/{v}/impact | config.author, config.approve, dashboard.read | Rows changing value and monitored attributes touched (scenarios and coverage filled by phases 16 and 06) |
| GET | /mapping-runs/{id} | config.author, config.approve, dashboard.read | One test or preview run; preview rows for DS and M only, else masked 404 |
| POST | /mappings/{id}/replays | data.steward | Replay over the raw landing layer for a range; produces a diff (phase 11) |
| GET | /mapping-replays/{id} | data.steward, config.approve | Diff counts and shapes per table and monitored attribute, volume effect; an unpublished version's replay is masked 404 except to its requester, M and a checker with an open CR on it (phase 11, DR-92) |
| POST | /mapping-replays/{id}/apply | data.steward | Apply a completed replay to the published version; `If-Match`; 202; **SU** `backfill` (phase 11) |
| GET | /code-maps | config.author, config.approve, dashboard.read | Code maps by kind and source |
| POST | /code-maps | config.author | Create a code map for a CBS code list; optional prefill (never for currency) |
| GET | /code-maps/{id}/entries | config.author, config.approve, dashboard.read | Effective-dated entries; unmapped values listed first, unverified prefills next |
| POST | /code-maps/{id}/entries | config.author | Map, reclassify or verify codes; **CR** `code_map` |
| GET | /drift-events | data.steward | Drift events with paused tables |
| POST | /drift-events/{id}/resolve | data.steward | Resolve after the mapping change is published (or a notify kind); `If-Match` |
| GET | /sources/{id}/view-ddl-pack | data.steward | Generate the read-only view DDL pack for the bank's DBA (phase 10) |
| GET | /attributes | config.author, config.approve, dashboard.read | Attribute registry by entity (phase 10) |
| POST | /attributes | config.author | Define a bank-specific attribute with its classification (DS in Cedar) |

### 3.7 Reconciliation, coverage and data quality (M7) [P0 gap report, P1]

Phase 06 builds readiness, declarations and the gap report (docs/plan/phase-06.md, DR-64 to DR-67), with the
catalogue and register routes of 3.11 and the pack routes of 3.16; phase 12 builds reconciliation, coverage and data
quality (docs/plan/phase-12.md s.2, DR-94 to DR-97); the blind-period routes are phase 11.

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /coverage/cbi-gap-report | dashboard.read, data.steward | The latest sealed CBI coverage gap report (or `?run_id=`): 173 items, readiness grade, missing fields, declarations, vendor annex; `stale` flag [P0] |
| POST | /coverage/cbi-gap-report/runs | dashboard.read, data.steward | Regrade and build the report: sealed JSON and four PDF packs (report and vendor request, Arabic and English); 202 [P0] |
| GET | /coverage/cbi-gap-report/runs | dashboard.read, data.steward | Report runs, newest first [P0] |
| GET | /coverage/cbi-gap-report/runs/{id} | dashboard.read, data.steward | One run: status, summary, pack ids, error code [P0] |
| GET | /coverage/readiness | config.author, dashboard.read | Readiness grade A-D per scenario, bank or source scope, with reasons and missing fields [P0] |
| PUT | /coverage/readiness/{id}/declaration | config.approve | Not-covered declaration on a bank-scope C or D grade (interim route, owner, reason, review date); MLRO only; **SU** [P0] |
| POST | /recon/runs | data.steward | Queue a ladder run for a source and business date, or one batch; 202 |
| GET | /recon/runs | data.steward, dashboard.read | Reconciliation runs per batch and day |
| GET | /recon/runs/{id} | data.steward, dashboard.read | Rung results (receipt, counts, control totals, GL tie-out) with the four C-25 outcomes |
| GET | /recon/certificates | dashboard.read, data.steward | Signed daily coverage certificates, current and superseded revisions |
| GET | /recon/certificates/{business_date} | dashboard.read, data.steward | Certificates of one date with signature and vault reference |
| POST | /recon/certificates/{business_date}/attest | dashboard.read | MLRO attests a certificate with declared gaps; **SU** `signing`; `If-Match` (DR-95) |
| GET | /coverage/statement | dashboard.read | MLRO coverage statement: accounts and transactions covered, data as of, blind periods |
| GET | /coverage/ledger | data.steward, dashboard.read | Coverage per branch, product, transaction type, channel and feed; dark units with reason and since |
| GET | /coverage/exclusions | data.steward, dashboard.read | Exclusions with owner, approver, reason, review date and expiry |
| POST | /coverage/exclusions | data.steward | Propose an exclusion; **CR** `exclusion` (material, DR-96) |
| GET | /coverage/blind-periods | dashboard.read, data.steward | Blind periods with cause and statement [phase 11] |
| POST | /coverage/blind-periods | data.steward | Declare a system or rail blind period; source periods open and close automatically (phase 11, DR-90) |
| GET | /dq/issues | data.steward | Data-quality issues with owners, severities, ages and due dates |
| POST | /dq/issues/{id}/assign | data.steward | Set the owner; `If-Match` |
| POST | /dq/issues/{id}/resolve | data.steward | Resolve with a resolution and a note; closes the issue's conflict quarantine rows; `If-Match` |
| GET | /dq/known-adjustments | data.steward, dashboard.read | Bank-signed, expiring known adjustments (accepted issues) |
| POST | /dq/known-adjustments | data.steward | Accept one open issue until a bank-signed expiry; **CR** `known_adjustment` (material) |

### 3.8 Reference data and calendars (M8)

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /ref/countries | self | Countries with base level, effective level, open override and basis (phase 09) |
| PUT | /ref/countries/{iso2}/override | config.author | Bank override with reason and expiry (`level` null ends it); `If-Match`; **CR** `country_override` (raise `cr_minor`, lower `cr_material`; DR-84) |
| GET | /ref/fx-rates | self | CBI and manual rates by date (DS, CFG, CHK, M, CH, IA in Cedar) |
| POST | /ref/fx-rates | data.steward | Record a rate when the feed is missing (`upload` evidence); never replaces a CBI rate (DR-80) |
| GET | /ref/fx-rates/gaps | data.steward | Business dates with no rate for a currency |
| GET | /ref/currencies | self | ISO currencies with minor unit |
| GET | /ref/governorates | self | Iraqi governorates with region |
| GET | /ref/calendars | self | Calendars with default flag, weekend per currency and version |
| POST | /ref/calendars | config.author | Propose a calendar; **CR** `calendar` |
| PUT | /ref/calendars/{id}/week | config.author | Propose a weekend change per currency; `If-Match`; **CR** `calendar` |
| GET | /ref/calendars/{id}/days | self | Holidays, Ramadan hours, Eid, Arbaeen, pay days, Hijri labels (display only) |
| POST | /ref/calendars/{id}/days | config.author | Add or change up to 400 days; `If-Match`; **CR** `calendar_day` |
| GET | /ref/calendars/{id}/business-days | self | Business-clock preview: date reached after `n` units (DR-79) |
| GET | /ref/calendars/{id}/reconciliation | data.steward | Differences against the latest captured CBS calendar (DR-79) |
| GET | /ref/reason-codes | self | Reason-code library by family; templates and inactive codes to config readers only |
| POST | /ref/reason-codes | config.author | Add or revise up to 50 codes of one family; **CR** `reason_code` (DR-82) |
| GET | /policy-packs | config.author, config.approve, dashboard.read | Policy pack versions (phase 09, DR-81) |
| GET | /policy-packs/{id} | config.author, config.approve, dashboard.read | One version with its document; masked 404 for other readers |
| PUT | /policy-packs/active | config.author | Propose the next version with the dry run; `If-Match`; **CR** `policy_pack` |
| GET | /ref/segments | config.author | Segment register with sizes and merges; no member lists (phase 13) |
| POST | /ref/segments | config.author | Propose a static or risk-overlay segment (or a new version with `base_version`); **CR** `segment` (`cr_material`, volume effect measured, DR-99) |
| GET | /ref/segments/{id} | config.author | Rule, merge chain, version, change request, last assignment run |
| GET | /ref/segments/{id}/stats | config.author | Robust statistics (n, p50, p95, p99, MAD) per feature, currency and period; populations below 200 suppressed and counted only (S-12) |
| GET | /ref/typologies | self | Shared typology set: number, tag and label (phase 06, DR-381) |

### 3.9 Screening and name matching (M9)

Phase 14 (DR-100 to DR-102). Screening alerts and pair memory are masked kinds: any caller outside the policy gets the
404 of a missing id.

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /lists | screening.work, config.author, config.approve, audit.read, assessor.read | Lists with kind, enabled, refresh interval, active version, age and stale flag |
| GET | /lists/{id} | screening.work, config.author, config.approve, audit.read, assessor.read | Detail: thresholds, script plan, feed description, next due fetch |
| GET | /lists/{id}/versions | screening.work, config.author, config.approve, audit.read, assessor.read | Versions with status, counts, checksums, failure code, origin |
| POST | /lists/{id}/versions | screening.work | Upload a national or bank list file when no feed exists (multipart); **SU**; 202 |
| GET | /lists/{id}/versions/{v}/diff | screening.work, config.author, config.approve, audit.read, assessor.read | Additions, removals, amendments by count; entry rows to SS and M only |
| POST | /lists/{id}/versions/{v}/activate | screening.work | Activate a validated version after the known-answer gate; **SU**; **4E** for uploads and bank snapshots; `If-Match` |
| POST | /lists/{id}/fetches | screening.work | Fetch the feed now, or read the bank snapshot now (akr-ingestd); 202 |
| PUT | /lists/{id}/settings | config.author | Propose thresholds, refresh, script plan, enablement or feed source; **CR** `list_setting` (dry run); `If-Match` |
| POST | /screening/search | screening.work | Name search with purpose (audited with a keyed hash, rate-limited) |
| GET | /screening/runs | screening.work | Daily base, look-back, onboarding, payment and ad hoc runs with counts and degraded flags |
| POST | /screening/runs | screening.work | Ad hoc run for one subject; purpose required; 202 |
| POST | /screening/lookbacks | screening.work | Start a look-back for a list version |
| GET | /screening/alerts | screening.work | Screening alerts (priority class, clock); no total |
| GET | /screening/alerts/{id} | screening.work | Alert with aligned names, identifiers, evidence bits, folds |
| POST | /screening/alerts/{id}/claim | screening.work | Claim; `If-Match` |
| POST | /screening/alerts/{id}/decision | screening.work | True, potential or incorrect with reasons; **SU**; **4E** for true on a freeze list (a prior potential by another person); a true freeze match reaches the freeze workflow (M14, phase 20) |
| POST | /screening/alerts/bulk-decision | screening.work | Incorrect on up to 50 alerts of identical evidence, never freeze lists; **SU** |
| GET | /screening/pair-memory | screening.work | Remembered pairs with status and expiry |
| POST | /screening/pair-memory | screening.work | Propose a remembered pair from an incorrect alert |
| POST | /screening/pair-memory/{id}/approve | screening.work | Approve a proposal; **SU**; **4E** |
| DELETE | /screening/pair-memory/{id} | screening.work | Invalidate (row kept) |
| GET | /screening/good-guys | screening.work, config.approve, audit.read | Good-guy rules with expiry and suppression counts |
| POST | /screening/good-guys | screening.work | Propose a rule (never on a freeze list); **CR** `good_guy_rule` |
| GET | /screening/harness-runs | screening.work, config.author, config.approve, audit.read, assessor.read | Known-answer harness results per list version |
| POST | /screening/harness-runs | screening.work | Run the harness (gate, oracle or canary) |

### 3.10 Customer risk rating and KYC (M10)

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /parties/{id}/profiles | queue.work, kyc.work | Monthly actual and declared figures per customer record (phase 13); `X-Access-Purpose` required; never deviations, peer or segment values (DR-99) |
| GET | /parties/{id}/rating | kyc.work | Current rating, CBI label, floors, completeness, controls |
| GET | /parties/{id}/rating/history | kyc.work | Rating history with model versions |
| GET | /parties/{id}/rating/explanation | kyc.work | Explanation panel data (drivers, gaps, triggers, overrides) |
| POST | /parties/{id}/overrides | kyc.work | Propose an override; downward starts `kyc_override_down`; **SU** |
| GET | /overrides | kyc.work | Overrides with expiry and analytics filters |
| GET | /kyc/reviews | kyc.work | Review queue (periodic, event, light yearly, EDD) |
| GET | /kyc/reviews/{id} | kyc.work | Review workspace pre-filled with new and changed facts |
| POST | /kyc/reviews/{id}/claim | kyc.work | Claim |
| PATCH | /kyc/reviews/{id} | kyc.work | Save checklist progress and evidence slots |
| POST | /kyc/reviews/{id}/queries | kyc.work | Raise a query on the review (phase 15) |
| PATCH | /kyc/reviews/{id}/queries/{qid} | kyc.work | Answer or close a query (phase 15) |
| POST | /kyc/reviews/{id}/evidence | kyc.work | Upload evidence, staged and scanned, owner kind `kyc_review` (phase 15) |
| GET | /kyc/reviews/{id}/evidence/{evidence_id} | kyc.work | Stream one evidence object, hash-checked on serve (phase 15) |
| POST | /kyc/reviews/{id}/complete | kyc.work | Complete; no sign-off with open queries; **SU** |
| POST | /kyc/reviews/{id}/reassign | kyc.work | Reassign an open review (M) (phase 15) |
| GET | /pep/register | kyc.work | PEP register with tiers and lifecycle |
| POST | /pep/register | kyc.work | Add an entry |
| PATCH | /pep/register/{id} | kyc.work | Change tier, office dates, persistence |
| POST | /pep/register/{id}/relatives | kyc.work | Confirm a relative or associate link (human confirmation) |
| GET | /parties/{id}/ownership | kyc.work | Ownership chain, effective percentages, loops |
| PUT | /parties/{id}/ownership/inputs | kyc.work | Replace the owner set the bank enters where the CBS holds no percentage (phase 15) |
| GET | /exit-register | kyc.work | Customers exited for financial-crime reasons (M only, DR-105) |
| POST | /exit-register | kyc.work | Add an entry from a completed `exit_decision` instance, one per instance (M); **SU** |
| GET | /rating/feed | kyc.work | One-way rating-change feed for the bank (cursor) |
| GET | /rating/divergence | kyc.work | Divergence against the CBS CDD level (`canon.customer.cdd_level`); not populated is never a divergence |
| GET | /rating/distribution | dashboard.read | Distribution, sanity bands, escapes |
| GET | /ownership/coverage | dashboard.read | Beneficial-ownership coverage by level, counts only (phase 15 7+) |
| GET | /rating/overrides/analytics | dashboard.read | Override counts by proposer and branch, downward share, expiries, follow-on alerts and STRs (phase 15 7+) |
| GET | /rating-models | config.author, config.approve | Model versions |
| POST | /rating-models/versions | config.author | Open a draft (DSL.md Part C) |
| PUT | /rating-models/versions/{v} | config.author | Replace the draft |
| POST | /rating-models/versions/{v}/validate | config.author | Validation |
| POST | /rating-models/versions/{v}/shadow | config.author | Shadow run on production data |
| GET | /rating-models/versions/{v}/shadow | config.author, config.approve | Disagreement list, tier migration, sensitivity, volume effect |

### 3.11 Transaction monitoring and scenario studio (M11) [P0 register, P1]

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /cbi-catalogue | self | The 173 CBI items with their implementing scenario codes; no grades and no restricted codes (DR-64) [P0] |
| POST | /cbi-catalogue/imports | config.author | Import a CBI or Office issuance as draft items with data requirements |
| GET | /coverage/matrix | dashboard.read | Coverage matrix: CBI item to scenario to readiness (phase 16); the alert, case and STR columns join in phases 17, 18 and 21 |
| GET | /scenarios | config.author | Scenario register (pack, family, status, readiness; live version from phase 16) [P0 register] |
| POST | /scenarios | config.author | Create a bank scenario (901-999) |
| GET | /scenarios/{id} | config.author, config.approve, dashboard.read | Detail: needs, links to CBI items, grades [P0]; versions, parameter sets and tests from phase 16 where `scenario.version.read` allows |
| POST | /scenarios/{id}/versions | config.author | Open a draft version |
| GET | /feature-definitions | config.author, config.approve | Feature versions (active and retired) with window and expression kind (phase 13, DR-98) |
| POST | /feature-definitions | config.author | Propose a feature or a new version; **CR** `feature_definition` (`cr_material`) |
| GET | /feature-definitions/{id} | config.author, config.approve | One version: expression, requires, recent run states |
| GET | /peer-groups | config.author, dashboard.read | Peer builds per currency: state, k, members, WCSS, stability (phase 13) |
| POST | /peer-groups | config.author | Request a candidate build for one currency (202); activation by **CR** `peer_build` |
| GET | /peer-groups/{id} | config.author, dashboard.read | Centroids in scaled space and cluster sizes; no member lists |
| PUT | /scenarios/{id}/versions/{v} | config.author | Replace the draft (DSL.md Part A) |
| POST | /scenarios/{id}/versions/{v}/validate | config.author | Schema and semantic validation |
| POST | /scenarios/{id}/versions/{v}/test | config.author | Unit and seeded cases |
| GET | /scenarios/{id}/parameter-sets | config.author, config.approve | Parameter-set versions with basis, band and reason |
| POST | /scenarios/{id}/parameter-sets | config.author | Draft a parameter set (then a change request) |
| POST | /scenarios/{id}/replays | config.author | Replay a version and parameter set over an as-of period; cost-guarded |
| GET | /replays/{id} | config.author, config.approve | Replay summary (by segment, branch, month; reach; overlap; look-ahead flag; effect); cells under five withheld (phase 16) |
| GET | /replays/{id}/results | config.author, config.approve | Per-entity alert and near-miss rows (cursor); `X-Access-Purpose`; suspicion-tier rows to M only |
| POST | /replays/{id}/samples | config.author | Generate the above- and below-the-line sampling plan |
| POST | /samples/{id}/labels | queue.work | Record a blind reviewer's label on a sampled item; never the scenario's author |
| GET | /samples/{id}/packet | queue.work | The blind packet: transaction and profile facts, no outcome, threshold, parameter or scenario identity; `X-Access-Purpose` (phase 16) |
| POST | /scenarios/{id}/shadow | config.author | Start a shadow run of a version |
| GET | /scenarios/{id}/shadow | config.author, config.approve | Shadow comparison with the live version by day and outcome |
| POST | /scenarios/{id}/kill-switch | config.approve | Fall back to the previous approved version (CHK, M; not a change request, DR-108); **SU** |
| POST | /scenarios/{id}/retire-requests | config.author | Propose the retirement of a bank or AKR scenario (**CR** `scenario`), or withdraw a version that is not live (phase 16) |
| GET | /scenarios/{id}/card | config.author, config.approve, dashboard.read | The generated scenario card |
| GET | /scenarios/{id}/metrics | config.author, config.approve, dashboard.read | Operational metrics with red and amber status (phase 16); outcome measures phase 24 |
| GET | /seeded-tests | config.author, config.approve | Known-answer suite with last results |
| POST | /seeded-tests/runs | config.author | Run the suite now |

### 3.12 Alerts, cases and investigation (M12)

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /queues | queue.work | Queues visible to me with counts (post-filter) |
| GET | /queues/{id}/items | queue.work | Items by priority class then score; saved views; cursor |
| GET | /alerts/{id} | queue.work | Alert with evidence snapshot, reason codes, versions, `data_as_of` |
| POST | /alerts/{id}/claim | queue.work | Claim (409 `E_CLAIMED` on a race; work in progress capped at 10) |
| POST | /alerts/{id}/unclaim | queue.work | Return to the queue |
| GET | /cases | queue.work | Case search (post-filter; no totals) |
| GET | /search | queue.work | Global search over parties and cases; kind detection; suspicion rows absent for non-members; never reports or screening (phase 18 7+) |
| GET | /cases/{id} | queue.work | Case with affordances; **SU** for suspicion tier |
| GET | /cases/{id}/timeline | queue.work | Event trail |
| POST | /cases/{id}/assign | queue.work | Assign within the pool rules |
| POST | /cases/{id}/notes | queue.work | Add a note (immutable once posted; amendments link to it) |
| POST | /cases/{id}/evidence | queue.work | Upload evidence (antivirus, hash, encryption before the vault) |
| POST | /cases/{id}/proposals | queue.work | Propose no report, UTR, STR or TFR with coded reasons, checked text, evidence and the CBI checklist; starts the approval workflow; `dry_run`; close and escalate are their own routes (phase 18, DR-135) |
| GET | /cases/{id}/decisions | queue.work | Decisions with versions and AI involvement |
| POST | /cases/{id}/decisions | case.decide | Decide on the open proposal (acts on my approval task); **SU**, **4E** |
| POST | /cases/{id}/escalate | queue.work | Escalate to the next tier |
| POST | /cases/{id}/merge | queue.work | Merge into another open case (never hides; both trails kept) |
| GET | /work | queue.work | W1 My work: my claimed alerts, my open cases, items due today (phase 17, added) |
| GET | /queues/{id}/views | queue.work | Saved views of the queue with counts (phase 17, added) |
| POST | /queues/{id}/views | queue.work | Save a personal or pool-shared view (phase 17, added) |
| DELETE | /queues/{id}/views/{view_id} | queue.work | Delete my view (phase 17, added) |
| POST | /queues/{id}/next | queue.work | Claim the next visible item of the view (SKIP LOCKED; 204 when none) (phase 17, added) |
| POST | /queues/{id}/bulk | queue.work | Bulk assign or mark duplicate, with dry run; never closure (phase 17, added) |
| GET | /alerts/{id}/snapshot | queue.work | The sealed evidence snapshot (access event) (phase 17, added) |
| GET | /alerts/{id}/model-score | queue.work | Live score, top-band flag and up to three reason codes of one alert; never shadow or challenger; the alert's own visibility, purpose and step-up rules (phase 27, added) |
| POST | /alerts/{id}/triage | queue.work | Accept, mark duplicate within the case, or raise the class (phase 17, added) |
| POST | /alerts/{id}/assign | queue.work | Assign to a user in the queue's pool (conflict check) (phase 17, added) |
| GET | /alert-quarantine | queue.work | Payloads that could not form an alert (payload to A2 and M only) (phase 17, added) |
| POST | /alert-quarantine/{id}/release | queue.work | Re-validate and release into the sink (202; worker job); **SU** (phase 17, added) |
| POST | /alert-quarantine/{id}/reject | queue.work | Reject with a closure code (phase 17, added) |
| GET | /cases/{id}/chain | audit.read | Chain verification of one case, result only (phase 17, added) |
| GET | /events/counts | self | SSE `queue.counts`: the caller's own masked counts (phase 17, added) |
| GET | /cases/{id}/notes | queue.work | Notes of the case, oldest first (cursor) (phase 18, added) |
| GET | /cases/{id}/evidence | queue.work | Evidence items with scan status (phase 18, added) |
| GET | /cases/{id}/evidence/{evidence_id}/content | queue.work | Verified bytes; access event first; **SU** (`record_open`) on a suspicion case (phase 18, added) |
| POST | /cases/{id}/opened-views | queue.work | Record that a panel was opened (an evidence item is recorded by its content read) (the decision gate, DR-136) (phase 18, added) |
| GET | /cases/{id}/proposals | queue.work | Running or last proposal, the caller's gate report and task, instance version as ETag (phase 18, added) |
| POST | /cases/{id}/close | case.decide | Closing sequence: outcome fixed, work items done or waived, seal; **SU** (phase 18, added) |
| GET | /cases/{id}/access | queue.work | Case access list with reasons (phase 18, added) |
| POST | /cases/{id}/access | case.decide | Grant read, work or decide to a person; a manual grant on a suspicion-tier case is M only and time-boxed (phase 18, added; 7+) |
| DELETE | /cases/{id}/access/{principal_id} | case.decide | Revoke a grant (not the current assignee) (phase 18, added) |
| GET | /cases/{id}/work-items | queue.work | Work items of the case (phase 18, added) |
| POST | /cases/{id}/work-items | queue.work | Open a work item with a due time (phase 18, added) |
| POST | /cases/{id}/work-items/{work_item_id}/complete | queue.work | Mark a work item done with a result (phase 18, added) |
| POST | /cases/{id}/work-items/{work_item_id}/waive | case.decide | Waive by a second person with a closure code (phase 18, added) |
| GET | /cases/{id}/links | queue.work | Entity links of the case, masked at read time (phase 18, added) |
| POST | /cases/{id}/links | queue.work | Link a party, account, transaction or external identifier (phase 18, added) |
| POST | /cases/{id}/counterparty-links | queue.work | Confirm or reject the party of a counterparty on a linked transaction (phase 18 7+) |
| GET | /shelved-decisions | case.decide | Shelved-decision register: no-report decisions with codes and decider, no text (M-04) (phase 18, added) |
| POST | /cases/{id}/reopen | case.decide | Reopen a closed case with reason; **SU** |
| POST | /cases/{id}/hibernations | case.decide | Hibernate a scenario for the party after a not-suspicious decision (envelope, expiry) (phase 19) |
| GET | /hibernations | queue.work | Hibernation ledger with regret metric (phase 19) |
| GET | /watches | queue.work | UTR watches with triggers and expiry (phase 19) |
| GET | /parties/{id} | queue.work | Party page (profile, accounts, rating, alerts and cases I may see) |
| GET | /parties/{id}/transactions | queue.work | Transactions from the canonical store (filters, cursor) |
| GET | /parties/{id}/graph | queue.work | Network around the party (kinship hidden by default; showing it is logged) (phase 19) |
| GET | /parties/{id}/timeline | queue.work | Entity timeline of the party, each source through its own visibility (phase 18 7+) |
| POST | /cases/{id}/rfis | queue.work | Draft an RFI (branch, customer via branch, correspondent) (phase 19) |
| GET | /rfis | rfi.desk | RFI desk (phase 19) |
| GET | /rfis/{id} | rfi.desk | RFI with deadline clock and answers (phase 19) |
| POST | /rfis/{id}/release | rfi.desk | Release after the second signature; **SU**, **4E** (phase 19) |
| POST | /rfis/{id}/answers | rfi.desk | Record answers as claims (source, corroboration, expiry) (phase 19) |
| GET | /authority-requests | case.decide | FIU, court and prosecutor requests (suspicion tier) (phase 19) |
| POST | /authority-requests | case.decide | Register a request with deadline; triggers re-rating (TR-17); **SU** (phase 19) |
| GET | /authority-requests/{id} | case.decide | Detail with linked cases and customers (phase 19) |
| GET | /internal-reports | case.decide | Internal reports from staff and liaison officers (phase 19) |
| POST | /internal-reports/{id}/acknowledge | case.decide | Acknowledge receipt (clock) (phase 19) |
| GET | /qa/samples | qa.review | QA samples (phase 19) |
| POST | /qa/samples | qa.review | Draw a sample (independent reviewer, never one's own cases) (phase 19) |
| POST | /qa/samples/{id}/review | qa.review | Score with findings (phase 19) |
| POST | /qa/bulk-reopen | qa.review | Reopen a reviewer's items after a critical finding; **SU** (phase 19) |
| GET | /qa/findings/{id}/reopen-candidates | qa.review | Cases the closer of a critical finding closed in its 30-day window, for the bulk reopen dialog; M only (phase 19, added) |
| GET | /qa/calibration | qa.review | Calibration kit and agreement (phase 19) |
| GET | /cases/{id}/rfis | queue.work | The case's RFIs for the case team (phase 19, added) |
| POST | /rfis/{id}/submit | queue.work | Submit a draft for release; starts `rfi_release_internal` or `rfi_release_external` (phase 19, added) |
| POST | /rfis/{id}/extend | rfi.desk | One extension by working days, two at most (phase 19, added) |
| GET | /rfis/correspondents | rfi.desk | Correspondent requests in and out: deadline, progress, recipient account id; no case detail; flag `pack.correspondent` (phase 31, added) |
| POST | /rfis/{id}/packs | rfi.desk | Attach a correspondent or validator pack to an outbound correspondent request in draft; `If-Match` (phase 31, added) |
| POST | /cases/{id}/watches | queue.work | Start a UTR watch by hand on a UTR decision of the case (phase 19, added) |
| POST | /watches/{id}/review | case.decide | Review at expiry: extend, close or upgrade; **SU** (phase 19, added) |
| POST | /hibernations/{id}/end | case.decide | End a hibernation by a person with a reason (phase 19, added) |
| POST | /internal-reports | case.decide | Register a report received outside the branch window (phase 19, added) |
| POST | /parties/{id}/network-cases | case.decide | Open a network case: parent of family `network`, sealed snapshot, one child per visible subject (phase 19, added) |
| GET | /qa/samples/{id}/item | qa.review | The sampled item for its reviewer under the time-boxed grant; **SU** on a suspicion item (phase 19, added) |
| GET | /qa/rates | qa.review | Reviewer rates with Wilson intervals (M only) (phase 19, added) |
| GET | /alarms | case.decide, audit.read | Alarm register: breach, chain, conflict, flat-line and critical QA alarms (phase 19, added) |
| POST | /alarms/{id}/acknowledge | case.decide, audit.read | Acknowledge or close an alarm (phase 19, added) |

Phase 17 ships /queues, /queues/{id}/items, /alerts/{id}, claim, unclaim, /cases, /cases/{id}, timeline, assign and merge as written, and the rows marked "phase 17, added". Phase 18 owns notes, evidence, proposals, decisions, escalate, reopen and the party pages, and ships them with the rows marked "phase 18, added" (25 routes, DR-146); phase 19 owns `/parties/{id}/graph`, hibernations, watches, RFIs, authority requests, internal reports and QA (the rows marked "phase 19") and ships them with the rows marked "phase 19, added" (31 routes, DR-157). Screening queues stay at `/screening/alerts` (3.9, phase 14).

### 3.13 Reporting to the AML/CFT Office (M13) [P0 checker, P1]

| Method | Path | Scope | Description |
|---|---|---|---|
| POST | /goaml/check | report.prepare | Upload goAML XML or ZIP files (or paste one) with an optional Office rejection message; checked against the active profile, then deleted (DR-69); 202 with the run id [P0] |
| GET | /goaml/check-runs | report.prepare | Checker runs: the caller's own, all for M [P0] |
| GET | /goaml/check-runs/{id} | report.prepare | Run status, per-file outcome, estimated score and grade (`grade_basis: estimated`); another preparer's run is masked (404) [P0] |
| GET | /goaml/check-runs/{id}/findings | report.prepare | Findings with rule, element path, line, explanation and fix; never a value from the file [P0] |
| GET | /goaml/profiles | report.prepare, config.author, config.approve | goAML profile versions (XSD version, source, status) [P0] |
| GET | /goaml/profiles/{id} | report.prepare, config.author, config.approve | Version detail: patches, lookups, rules, quality, limits, emit flags, open questions, diff from base, corpus result [P0] |
| PUT | /goaml/profiles/{id} | config.author | Promote a candidate or amend a version; **CR** `goaml_profile` (corpus result required) [P0] |
| GET | /goaml/watch-runs | report.prepare, config.author | Office schema watcher runs and the candidates they created [P0] |
| POST | /goaml/watch-runs | report.prepare, config.author | Run the watcher now; refused while it is disabled (DR-71) [P0] |
| POST | /cases/{id}/reports | report.prepare | Draft from the running proposal of type utr, str or tfr; 201, or 200 for the existing draft (phase 21) |
| GET | /reports | report.prepare | List, suspicion tier post-filtered; cursor, no total (phase 21) |
| GET | /reports/{id} | report.prepare | Draft or final with validation state and quality; **SU** (phase 21) |
| PATCH | /reports/{id} | report.prepare | Edit the draft; `If-Match` (phase 21) |
| GET | /reports/{id}/versions | report.prepare | Versions with frozen and validated times; `?from=&to=` compares two versions (phase 21, added) |
| GET | /reports/{id}/findings | report.prepare | Findings of the latest run of the version, with `field_key` for the panel, `?layer=&severity=` (phase 21, added) |
| POST | /reports/{id}/attachments | report.prepare | Stage an attachment (multipart; `EvidenceStore::stage`) (phase 21, added) |
| POST | /reports/{id}/validate | report.prepare | 202, job `report_validate`; `If-Match` (phase 21) |
| POST | /reports/{id}/submit | report.prepare | Freeze the validated version; 422 `E_REPORT_INVALID` with blocks; `If-Match` (phase 21) |
| POST | /reports/{id}/revise | report.prepare | New draft version from a rejected or returned report; `If-Match` (phase 21, added) |
| POST | /reports/{id}/sign | report.file | Sign: acts on the `sign` task of `case_report_approval` or `report_correction`; **SU** (`signing`), **4E**; `If-Match` = instance version (phase 21) |
| POST | /reports/{id}/package | report.file | 202, job `report_package` (phase 21) |
| GET | /reports/{id}/package | report.file | Download the ZIP; **SU** (`export`); `X-Access-Purpose` (phase 21) |
| POST | /reports/{id}/filing | report.file | Record the goAML reference and the portal capture (multipart); **SU** (`filing`) (phase 21) |
| POST | /filings/{id}/acknowledgement | report.file | Record the Office receipt; **SU** (`filing`) (phase 21, added) |
| POST | /reports/{id}/rejections | report.file | Record an Office rejection; findings; status `rejected` (phase 21) |
| POST | /reports/{id}/feedback | report.file | Record a grade; a poor grade opens the returned task (phase 21) |
| GET | /fiu-feedback | report.prepare | The feedback register (R2), filters by kind, grade, period; suspicion post-filter (phase 21, added) |
| GET | /report-tasks | report.prepare | Open returned, FX and control tasks; cursor (phase 21, added) |
| GET | /ctr/batches | report.prepare | Daily CTR batches; counts and totals, no customer identity below M (phase 21) |
| GET | /ctr/batches/{id} | report.prepare | Batch with control totals and run numbers; no GL tie-out (the canonical copy has no general ledger; DR-174) (phase 21) |
| POST | /ctr/batches | report.file | Run a business date (manual run; the daily job runs it automatically); 202 (phase 21, added) |
| POST | /ctr/batches/{id}/confirm | report.file | The checker confirms the control totals; **SU** (`approval`); not the creator (phase 21, added) |
| POST | /ctr/batches/{id}/package | report.file | Build the CTR files: 202, job `ctr_package` (phase 21) |
| POST | /letters | report.file | A manual `office_reply` or `hold_reply` (phase 21) |
| GET | /letters/{id} | report.file | Letter with state, number, PDF reference and delivery record (phase 21) |
| POST | /letters/{id}/sign | report.file | Sign the letter; **SU** (`signing`) (phase 21, added) |
| POST | /letters/{id}/delivery | report.file | Record the delivery: channel, date and reference; no sending (phase 21, added) |
| GET | /reports/periodic | dashboard.read | Periodic packs and their status (R3) (phase 21) |
| POST | /reports/periodic | report.file | Generate a pack for a kind and period; 202, job `periodic_build` (phase 21, added) |
| GET | /reports/periodic/{id} | dashboard.read | Pack detail: aggregates, suppressed cell count (phase 21, added) |
| POST | /reports/periodic/{id}/sign | report.file | Sign and finalise the pack; **SU** (`signing`) (phase 21, added) |

Phase 21 owns the rows marked "phase 21" and ships them with the rows marked "phase 21, added" (32 routes, DR-183); phase 07 owns the `/goaml` rows above. The liaison review rows of 3.16 are phase 22's.

### 3.14 Operations tasks and freezes (M14)

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /ops-tasks | ops.tasks, case.decide | Neutral task list (authority, instruction, clock); no AML reason; A2 and M get the team view (phase 20) |
| GET | /ops-tasks/{id} | ops.tasks, case.decide | Task detail in the caller's view (phase 20) |
| POST | /ops-tasks/{id}/claim | ops.tasks | Claim (phase 20) |
| POST | /ops-tasks/{id}/complete | ops.tasks | Complete with the CBS reference (phase 20) |
| GET | /ops-tasks/{id}/verification | case.decide | Verification level V0-V3 reached from the canonical copy (phase 20) |
| POST | /ops-tasks/{id}/attest | case.decide | Record a V3 attestation with evidence; **SU** (phase 20) |
| GET | /freezes | case.decide | Freeze board (freeze, Committee report, second report clocks; post-order debits) (phase 20) |
| GET | /freezes/{id} | case.decide | Freeze detail with letters (phase 20) |
| POST | /freezes/{id}/release-decision | case.decide | Release after delisting on a recorded decision; **SU**, **4E** (phase 20) |
| GET | /refrain-instructions | case.decide | Office refrain instructions with the 7-working-day clock (phase 20) |
| POST | /refrain-instructions | case.decide | Register an instruction; creates the operations task; **SU** (phase 20) |
| POST | /ops-tasks/{id}/cannot-complete | ops.tasks | Record that the action cannot be done, with a closure code; raises an incident (phase 20, added) |
| POST | /ops-tasks/{id}/confirm | ops.tasks | Second-person confirmation for V0; refused to the completer (phase 20, added) |
| POST | /ops-tasks/{id}/verification/check | case.decide | Run the canonical-copy check now (phase 20, added) |
| POST | /ops-tasks/{id}/evidence | case.decide | Upload the V3 attestation screenshot (owner kind `ops_task`) (phase 20, added) |
| POST | /freezes/{id}/release-proposals | case.decide | Propose the release of a delisted freeze; starts `freeze_release` (phase 20, added) |
| POST | /freezes/{id}/referral | case.decide | Refer an unresolved match to the Committee's technical committee (letter request); **SU** (phase 20, added) |
| GET | /refrain-instructions/{id} | case.decide | Instruction detail with its task (phase 20, added) |
| POST | /refrain-instructions/{id}/referral | case.decide | Refer to the technical committee (letter request); **SU** (phase 20, added) |
| POST | /refrain-instructions/{id}/close | case.decide | Close on the Office's withdrawal; creates the release task; **SU** (phase 20, added) |
| GET | /incidents | case.decide | Failure incidents (M only) (phase 20, added) |
| POST | /incidents/{id}/close | case.decide | Close an incident with a closure code (M only) (phase 20, added) |

Phase 20 owns the rows marked "phase 20" and ships them with the rows marked "phase 20, added" (22 routes, DR-168). Exit proposals are phase 15's exit register (`/exit-register`, resource kind `exit_decision`).

### 3.15 Governance, change control, models and AI (M15) [P1 change control, P2 models and assistant]

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /dsl/{kind}/schema | config.author, admin.system | JSON Schema for `change_request` and `workflow` (phase 04), `mapping` (10), `rating_model` (15), `scenario` (16); other kinds 404 until their phase |
| GET | /change-requests | config.author, config.approve, admin.users, admin.policies, admin.system, security.ops, audit.read | Change requests readable by the caller (filters: kind, class, status, mine, awaiting_me, emergency, review_pending) |
| POST | /change-requests | config.author | Create a draft, or create and submit (class, volume effect and route computed; DSL.md Part D); kinds with a dedicated route are refused here |
| GET | /change-requests/{id} | as the list | Detail with diff, evidence, impact (detection logic only with clearance), route, tasks and timeline |
| POST | /change-requests/{id}/submit | config.author, admin.users, admin.policies, admin.system | The maker submits a draft; `If-Match` |
| POST | /change-requests/{id}/approve | self | Approve on the caller's task in the route (Cedar and the engine decide who); **SU**, **4E**; `If-Match` (instance version) |
| POST | /change-requests/{id}/reject | self | Reject with comment on the caller's task; **SU**; `If-Match` (instance version) |
| POST | /change-requests/{id}/withdraw | self | The maker withdraws before a decision; `If-Match` |
| POST | /change-requests/{id}/evidence | config.author, admin.users, admin.policies, admin.system | The maker attaches a file while the request is a draft, or submitted with no decision yet; `Idempotency-Key`, `If-Match`; 201 (phase 04 7.2) |
| GET | /inventory | model.risk, audit.read | Inventory of rules, models and tools (tier, owner, validator, committee) |
| POST | /inventory | model.risk | Register an item before use |
| PATCH | /inventory/{id} | model.risk | Update owner, validator, tier, status, validation dates; **SU** when approving for use (never by the owner); `If-Match` |
| GET | /registers/{kind} | dashboard.read, model.risk | Registers: `decisions` and `exclusions` (read views through their owners' predicates), `findings`, `warnings`, `fairness`; cursor (phase 24) |
| POST | /registers/{kind}/entries | config.author, config.approve | Add a finding (M, CH, MRO); any other kind is 422; `Idempotency-Key` (phase 24) |
| GET | /registers/{kind}/entries/{id} | dashboard.read, model.risk | One finding or warning with its history (phase 24, added) |
| PATCH | /registers/{kind}/entries/{id} | config.author, config.approve | Owner, due date, severity, status, closure code; `If-Match`; **SU** class `approval` for `accepted_risk`, never by the finding's owner (phase 24, added) |
| GET | /registers/review-calendar | dashboard.read, model.risk | Review dates of rules, exclusions, change requests, readiness and scenarios, amber and red; derived, writes nothing (phase 24, added) |
| POST | /fairness/metrics/{id}/review | config.author, config.approve | Justify or change a group rate in review status; `If-Match` (phase 24, added) |
| GET | /fairness/attributes | dashboard.read, model.risk | Protected-attribute register and allowed-use matrix (phase 24, added) |
| POST | /inventory/{id}/validation-packs | model.risk | Issue a validation pack (seven parts, computed verdict); the validator is the caller, never the item owner; `Idempotency-Key` (phase 24, added) |
| GET | /inventory/{id}/validation-packs | model.risk, audit.read | Versions of an item's validation packs, cursor (phase 24, added) |
| GET | /validation-packs/{id} | model.risk, audit.read | One validation pack with its parts and evidence hash (phase 24, added) |
| GET | /models | model.risk, audit.read, dashboard.read | Families with live version, starter flag, next drill and pending version notices, cursor (phase 27) |
| POST | /models | model.risk | Create a family and its inventory item (kind `model`); 201; **IDEM** (phase 27; meaning narrowed, DR-271) |
| GET | /models/kill-switch | model.risk, audit.read, dashboard.read | State of the global switch and of each family's switch (phase 27, added) |
| GET | /models/ladder | model.risk, audit.read, dashboard.read | Model ladder per family: enabled tier, label counts and next gate, suppressed counts only (phase 27, added) |
| GET | /models/{id} | model.risk, audit.read, dashboard.read | Family with live, shadow, challenger and pinned versions, gates, next drill (phase 27) |
| POST | /models/{id}/versions | model.risk | Import a version (manifest, Ed25519 signature, card); 202 `importing`, worker job `model_import_verify`; bad signature or manifest 422 `E_VALIDATION`; **IDEM** (phase 27, added) |
| GET | /models/{id}/versions/{vid} | model.risk, audit.read, dashboard.read | Version, card (ten sections), refusal code; `?view=operating_point` adds thresholds and top-band cut, MRO and M only (phase 27, added) |
| POST | /models/{id}/validation | model.risk | Record a benchmark, out-of-time, calibration or sensitivity evaluation of a version; the pack is `POST /inventory/{id}/validation-packs`; **IDEM** (phase 27; meaning changed, DR-271) |
| POST | /models/{id}/shadow | model.risk | Start a shadow or challenger run with frozen success criteria (version `registered` or `retired`, item `approved_for_use`, shadow gate met); **IDEM** (phase 27) |
| POST | /models/{id}/versions/{vid}/promote | model.risk | Open change request on route `cr_material_validated` for the version; `If-Match`; **IDEM** (phase 27, added) |
| POST | /models/{id}/pin | model.risk | Pin or unpin the live version; **SU** `approval`; `If-Match`; **IDEM** (phase 27, added) |
| POST | /models/kill-switch | model.risk | Engage the global switch or one family's switch (scope); immediate, rules-only order; **SU** `approval`; `If-Match` on the scope row; **IDEM** (phase 27) |
| POST | /models/kill-switch/release-requests | model.risk | Open change request on route `cr_material` that releases a switch; **IDEM** (phase 27, added) |
| POST | /models/{id}/kill-drills | model.risk | Record a quarterly kill-switch drill and move `next_kill_drill_due`; **IDEM** (phase 27, added) |
| GET | /models/{id}/monitoring | model.risk, audit.read, dashboard.read | Stored monitoring points of the family's versions: bands, values, suppressed window counts, no ids (phase 27, added) |

Phase 27 owns the `/models` rows (docs/plan/phase-27.md s.2; 16 rows with `GET /alerts/{id}/model-score` in 3.12, 10
added; DR-257 to DR-273). Cedar permits the GET rows to MRO, M, IA and CH; every POST is MRO. The `/assistant` rows below are phase 28's.

| Method | Path | Scope | Description |
|---|---|---|---|
| POST | /assistant/drafts | queue.work | Request a draft of one task (`case_summary`, `report_narrative`) for one case; 202 `requested`, job `assistant_generate`; no text in the body; `X-Access-Purpose`; **IDEM** (phase 28) |
| GET | /assistant/drafts/{id} | queue.work | The draft: state, sentences with source labels, removed and marked counts, `not_considered` line, scan count; text only to the case team after the cited panels are opened; `X-Access-Purpose` (phase 28, added) |
| POST | /assistant/drafts/{id}/accept | queue.work | Accept one draft as is or edited: writes a case note or the phase 21 narrative; recorded in `ai_involvement`; no accept-all; `If-Match`; **IDEM** (phase 28) |
| POST | /assistant/drafts/{id}/discard | queue.work | Discard a draft with the useful-or-not answer; `If-Match`; **IDEM** (phase 28, added) |
| POST | /assistant/evaluations | model.risk | Run the golden evaluation of one (task, template, model) triple; 202, job `assistant_evaluate`; **IDEM** (phase 28, added) |
| GET | /assistant/evaluations | model.risk | Gate results (at least 200 QA-confirmed cases, 50 per language): counts, rates, verdicts, golden-set hash; no ids, no text; cursor (phase 28) |
| GET | /assistant/tasks | model.risk, audit.read, dashboard.read | Task registry: tier, autonomy, switch and kill state, live template and model, gate status, next drill; no text (phase 28, added) |
| POST | /assistant/versions | model.risk | Import a model (202, job `assistant_version_verify`) or a prompt template (201); bad signature or manifest 422 `E_VALIDATION`; **IDEM** (phase 28, added) |
| POST | /assistant/versions/{id}/promote | model.risk | Open change request on route `cr_material_validated` for one model and one template per enabled task; nothing changed 409 `nothing_to_promote`; `If-Match`; **IDEM** (phase 28, added) |
| POST | /assistant/kill-switch | model.risk | Engage the global switch or one task's switch; immediate; **SU** `approval`; `If-Match` on the scope row; **IDEM** (phase 28, added) |
| POST | /assistant/kill-switch/release-requests | model.risk | Open change request on route `cr_material` that releases a switch; **IDEM** (phase 28, added) |
| GET | /assistant/monitoring | model.risk, audit.read, dashboard.read | Counts per task over 30 days on live drafts, every tier, suppressed below `report.small_number_threshold`, no split (phase 28, added) |

Phase 28 owns the `/assistant` rows (docs/plan/phase-28.md s.2; 12 rows, 9 added; DR-274 to DR-291). Every route is 404
`E_NOT_FOUND` while flag `pack.assistant` is off. Cedar permits the `/assistant/drafts` rows to the case team (A1, A2, M;
the narrative accept also needs phase 21 `report.edit`), the GET registry and monitoring rows to MRO, M, IA and CH, and
every other row to MRO.

### 3.16 Command centre, registers and evidence packs (M16) [P0 gap report, P1, P2]

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /dashboard/command-centre | dashboard.read | Tiles with definition, formula, data-as-of and the list each opens; live, `no-store`, counts 1 to 4 suppressed (phase 24) |
| GET | /dashboard/clocks | dashboard.read | All clock instances from their owners' due columns, keyset by due instant; read-only (phase 24) |
| POST | /packs | dashboard.read, data.steward, assessor.read | Build a pack, a job (kinds `cbi_gap`, `vendor_request` and `integration_profile` [P0]; `board`, `case`, `report`, `assessor`, `certification` [phase 24]; `correspondent`, `validator` [P2]); AS builds kind `assessor` only |
| GET | /packs/{id} | dashboard.read, data.steward, assessor.read | Pack metadata and build status [P0; `assessor.read` phase 24] |
| GET | /packs/{id}/download | dashboard.read, data.steward, assessor.read | Download (access event); **SU** class `export` [P0; `assessor.read` phase 24] |
| POST | /exports | list route's scope | Start a CSV or XLSX export of one register screen (phase 24 7+) |
| GET | /exports/{id} | list route's scope | Status of the caller's own export (phase 24 7+) |
| GET | /exports/{id}/file | list route's scope | The export file; **SU** class `export` (phase 24 7+) |
| POST | /parallel-run/imports | case.decide | Upload the current system's alert extract for the parallel run (phase 24 7+) |
| GET | /parallel-run/imports/{id} | dashboard.read | Import status and outcome counts (phase 24 7+) |
| GET | /parallel-run/rows | dashboard.read | Comparison rows by import, outcome and explained; M and CH; party visibility (phase 24 7+) |
| POST | /parallel-run/rows/{id}/explanation | case.decide | Explain one disagreement; M only (phase 24 7+) |
| GET | /parallel-run/summary | dashboard.read | Counts by week and outcome with the unexplained count (phase 24 7+) |
| POST | /packs/verify | self | Verify a pack or document hash against the vault and the audit chain; `verified: false` alike for unknown and unreadable (phase 24) |
| GET | /assessor/d2-matrix | assessor.read | D2 clause map (signed data file) with live evidence links and a status per clause (phase 24) |
| POST | /assessor/samples | assessor.read | Draw a seeded, recorded random sample from a frame; `Idempotency-Key` (phase 24) |
| GET | /assessor/samples/{id} | assessor.read | A recorded sample with its seed, frame hash and items (phase 24, added) |
| GET | /assessor/trace/{alert_id} | assessor.read | End-to-end trace, metadata only: CBS row, transaction, alert, case, decision type, report; a suspicion-tier alert only as an item of a sample drawn under the grant in force (phase 24) |
| GET | /liaison-reports | case.decide | Liaison monthly reports for review, cursor; filters branch, status, period; Cedar restricts to M (phase 22) |
| POST | /liaison-reports/{id}/review | case.decide | Accept or return to draft with a comment; `If-Match` (phase 22) |
| GET | /training/register | dashboard.read | Training register, cursor, filter by user and course (phase 24) |
| POST | /training/register | admin.users | Record a training completion; `Idempotency-Key`; never for oneself (phase 24) |
| POST | /training/register/{id}/void | admin.users | Void a record with a reason code (phase 24, added) |
| GET | /training/practice/cases | queue.work | Active synthetic practice cases of the caller's paths, no answers; flag `pack.training` (phase 31, added) |
| GET | /training/practice/cases/{id} | queue.work | One practice case: facts, question, reason list; no known answer (phase 31, added) |
| POST | /training/practice/cases/{id}/attempts | queue.work | Submit a decision or choice; scored; returns the known answer and expert path; `Idempotency-Key` (phase 31, added) |
| GET | /training/practice/attempts | queue.work | The caller's own practice attempts (phase 31, added) |
| GET | /capacity/inputs | dashboard.read | Staffing inputs by pool and bank week (phase 31, added) |
| POST | /capacity/inputs | capacity.plan | Record or supersede the input of a pool and week; `If-Match`; `Idempotency-Key` (phase 31, added) |
| GET | /capacity/forecast | dashboard.read | Workload, backlog projection, sensitivity, surge progress; team-level only, `no-store` [P2] (phase 31) |
| POST | /capacity/simulations | capacity.plan | Simulation harness on a completed phase 16 replay; 202 (phase 31, added) |
| GET | /capacity/simulations/{id} | dashboard.read | Simulation status and result, estimates labelled (phase 31, added) |
| GET | /typology-exposure | dashboard.read | Typology exposure index by bank, branch, correspondent, with trend; suppressed cells null [P2] (phase 31) |

Phase 24 owns the rows marked "phase 24" here and in 3.3, 3.4 and 3.15 (docs/plan/phase-24.md s.2; 31 rows, 28 new,
the three pack routes are phase 06's extended; DR-218 to DR-231).

Phase 31 owns the `/training/practice`, `/capacity` and `/typology-exposure` rows here, the two correspondent rows in
3.12 and the three `/directory` rows in 3.2 (docs/plan/phase-31.md s.2; 15 rows, 13 added; DR-338 to DR-355). The
`correspondent` and `validator` kinds of `POST /packs` are built by phase 31 (flag `pack.correspondent`). Every phase 31
route is 404 `E_NOT_FOUND` while its flag is off. Capacity output is advice and never caps, raises or suppresses an
alert (D21).

### 3.17 Branch window and liaison (M17)

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /branch/home | branch.window | My branch, my role, the due date, neutral counts (phase 22, added) |
| GET | /branch/items | branch.window | Items for my branch with neutral statuses only (phase 22) |
| GET | /branch/items/{id} | branch.window | Item with the neutral question and deadline (phase 22) |
| POST | /branch/items/{id}/evidence | branch.window | Stage one document for the item (multipart) (phase 22, added) |
| POST | /branch/items/{id}/input | branch.window | Answer as phase 19 claims, never self-corroborated (phase 22) |
| POST | /branch/observations | branch.window | Record an observation (eight groups, companion ID, written acknowledgment) (phase 22) |
| POST | /branch/confidential-reports | branch.window | Confidential report to the MLRO; branch management cannot see it (phase 22) |
| GET | /branch/monthly-report | branch.window | Draft monthly liaison report (phase 22) |
| POST | /branch/monthly-report/sign | branch.window | Sign and send; **SU** (phase 22) |

Phase 22 owns the rows marked "phase 22" here, in 3.1, 3.16 and 3.20 (docs/plan/phase-22.md s.2; 17 routes, DR-201 to
DR-217). Every `/branch/*` route answers 404, not 403 `E_SCOPE`, to a caller without `branch.window` (DR-206).

### 3.18 Islamic banking pack (M18) [optional per bank]

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /islamic/deals | queue.work | Deals of one customer (`?customer_id=`), cursor; only deals the caller may read under the phase 18 party rule (phase 26) |
| GET | /islamic/deals/{id} | queue.work | Deal view: readable parties by role, asset, evidence status and rows; **SU** `record_open` on a suspicion-tier deal; a refused deal is the unknown-id 404 (phase 26) |
| POST | /islamic/deals/{id}/evidence | queue.work | Attach possession evidence (multipart, staged through `EvidenceStore`); **IDEM** (phase 26) |
| GET | /islamic/deals/{id}/evidence/{evidence_id}/content | queue.work | The bytes of one attachment through `EvidenceStore::open`; access event first; **SU** `record_open` on a suspicion-tier deal (phase 26, added) |
| POST | /islamic/deals/{id}/evidence/{evidence_id}/verify | queue.work | Verify or reject an attachment made by another person (A2, M); **SU** `decision`; `If-Match`; **IDEM** (phase 26, added) |
| GET | /islamic/sharia-board/aggregates | dashboard.read | Counts for a bank month by contract type, evidence status and gap rate, suppressed below `report.small_number_threshold`; no identifiers (phase 26) |

Phase 26 owns these rows (docs/plan/phase-26.md s.2; 6 rows, 2 added; DR-246 to DR-256). While flag `pack.islamic` is
off every `/islamic` route answers 404 `E_NOT_FOUND` with the unknown-id body.

### 3.19 Assessment administration (M19) [P2]

The decision endpoint itself is on `akr-assess` (s.4). These are the analyst and configuration endpoints on
`akr-api`.

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /assess/fail-policies | config.author, config.approve, dashboard.read | Fail policy matrix per rail, corridor, currency and amount band, with the amount bands |
| PUT | /assess/fail-policies | config.author | Propose a new policy set; **CR** `cr_material` (object `assess_policy`) |
| GET | /assess/modes | config.author, config.approve, dashboard.read | Mode per rail (shadow, advisory, enforcing) with effective-from and readiness snapshot |
| PUT | /assess/modes/{rail} | config.author | Propose a mode change; **CR** `cr_material` (object `assess_mode`); refused when the entry criteria fail |
| GET | /assess/modes/{rail}/readiness | config.author, config.approve | Entry criteria for the next mode, computed from the roll-ups (phase 29, added) |
| GET | /assess/holds | screening.work | Hold queue: held, advisory and shadow review rows |
| GET | /assess/holds/{id} | screening.work | One hold with its candidates, names for SS and M, and the proposal |
| POST | /assess/holds/{id}/claim | screening.work | Claim; `If-Match` |
| POST | /assess/holds/{id}/decision | screening.work | Release or reject: proposal, then approval by a second person; **SU** class `decision`, **4E** |
| GET | /assess/decisions | audit.read | Search of signed decisions; purpose required; party-linked rows masked |
| GET | /assess/release-without-decision | dashboard.read | Open release exceptions: no decision, no match key, pass expired, decision not a pass (C-59) |
| GET | /assess/blind-periods | dashboard.read | Outage periods of the assessment service with duration (C-58; phase 29, added) |
| GET | /assess/health | admin.system | Nodes, mode per rail, list freshness, degraded counters |
| POST | /assess/control-tests | admin.system | Record the weekly control-test payment result |

Phase 29 owns the `/assess` rows (docs/plan/phase-29.md s.2; 14 rows, 2 added; DR-292 to DR-316). Every route is 404
`E_NOT_FOUND` while flag `pack.assess` is off. Fail policies with bands, modes and prohibited rules are change targets
on `cr_material` with no emergency change (DR-300); prohibited rules have no route and are proposed through phase 04.

### 3.20 Admin, settings, releases and ops cockpit (M20) [P0 minimal]

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /admin/settings | admin.system | Settings with CBI floors shown |
| PUT | /admin/settings/{key} | admin.system | Change a setting: governed keys (those with a CBI floor and the change, access and authz families) answer 202 with a **CR**; other keys change at once (200) |
| GET | /admin/notification-templates | admin.system | Notice templates with their current version; edits by change request (phase 22, added) |
| GET | /admin/flags | admin.system | Feature flags |
| PUT | /admin/flags/{key} | admin.system | Toggle a flag (audited) |
| GET | /admin/ops | admin.system | Ops cockpit: queue depth, dead jobs, backups, anchors, disk, certificates, OpenBao seal state |
| GET | /admin/jobs | admin.system | Jobs by kind and state |
| POST | /admin/jobs/{id}/retry | admin.system | Retry a dead job |
| GET | /admin/releases | admin.system, security.ops | Imported release bundles with verification result, approval and applied record (phase 25) |
| POST | /admin/releases/verify | admin.system | Import and verify a bundle (phase 00 `bundle verify` code path: signature, SBOM diff, notes, rollback plan, policy matrix diff); 202 with the job id (phase 25) |
| POST | /admin/releases/{id}/propose | admin.system | Propose a verified release for approval: **CR** target `platform_release` on `cr_access` (maker ADM, checker SEC); `If-Match`; 202 (phase 25, added) |
| POST | /admin/releases/{id}/applied | admin.system | Record that the bank operator applied the approved release; no call-home, no remote push; `If-Match`; **SU** `approval` (phase 25, added) |
| POST | /admin/releases/{id}/evidence | admin.system | Upload the `rollout.json` of the apply as evidence of owner kind `release`; **IDEM** (phase 25, added) |
| GET | /admin/support-sessions | admin.system, security.ops | Vendor support sessions with state and times (phase 25, added) |
| POST | /admin/support-sessions | admin.system | Request a time-boxed vendor support session (at most `support.session_max_hours`) (phase 25) |
| POST | /admin/support-sessions/{id}/approve | security.ops | Approve; the `VS` grant starts with the session; approver is not the requester; **SU** `access_change`; **4E** (phase 25) |
| DELETE | /admin/support-sessions/{id} | admin.system | End it now; the `VS` grant and the vendor's sessions and tokens are revoked (phase 25) |
| POST | /admin/support-sessions/{id}/bundles | admin.system | Register a support bundle built and confirmed with `akr-ctl support-bundle` (hashes, size, key version); **SU** `export` (phase 25, added) |
| GET | /help/articles | self | Bilingual help articles and "what changed" entries of the installed release, shipped in the signed bundle; `?kind=` (phase 25) |
| GET | /siem/events | siem.pull | SIEM pull API for a service principal: `siem`-flagged events after a cursor, allow-listed fields, no case id, no tier (phase 25, added) |

Phase 25 owns the rows marked "phase 25" here and in 3.2 and 3.3 (docs/plan/phase-25.md s.2; 21 rows, 9 added;
DR-232 to DR-245). `/admin/ops` gains tiles, not routes.

### 3.21 Workflow tasks and definitions (DSL.md Part E; DBS engine ported in P0, docs/plan phase 04)

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /inbox | self | My tasks (views: mine, pool, waiting on others, history) |
| GET | /inbox/counts | self | Badge counts (post-filter) |
| GET | /tasks/{id} | self | Task with the resource summary and affordances |
| POST | /tasks/{id}/claim | self | Claim a pool task |
| POST | /tasks/{id}/release | self | Release a claim |
| POST | /tasks/{id}/actions/approve | self | Approve; **SU** on deciding steps, **4E** |
| POST | /tasks/{id}/actions/reject | self | Reject with comment; **SU** |
| POST | /tasks/{id}/actions/sign | self | Sign; **SU** always |
| POST | /tasks/{id}/actions/submit-form | self | Submit a form step |
| POST | /tasks/{id}/actions/review | self | Record a review on a non-blocking review step (also escalation notices) |
| POST | /admin/tasks/{id}/reassign | admin.system | Propose a reassignment of a stuck or parked task; **CR** (`task_assignment`, `cr_access`); **SU**; 202 |
| GET | /admin/workflow-definitions | admin.system | Definitions with versions |
| GET | /admin/workflow-definitions/{id} | admin.system | Family detail with every version's DSL and validation report |
| POST | /admin/workflow-definitions | admin.system | Create a family from a DSL body |
| PUT | /admin/workflow-definitions/{id}/draft | admin.system | Replace the draft |
| POST | /admin/workflow-definitions/{id}/validate | admin.system | Validation (DBS catalogue plus AKR codes) |
| POST | /admin/workflow-definitions/{id}/test-run | admin.system | Dry run with simulated actors |
| POST | /admin/workflow-definitions/{id}/publish | admin.system | Propose publication of the draft; **CR** (`cr_workflow`: ADM proposes, MLRO approves); 202 |

Resource-centric decision endpoints (`/cases/{id}/decisions`, `/reports/{id}/sign`, `/change-requests/{id}/approve`
and similar) act on the caller's open workflow task for that resource. They are equivalent to the task actions and
run the same Cedar checks; they exist so screens and automation do not need task ids.

## 4. The assessment API (`/assess/v1`, phase 2)

On `akr-assess`, a separate binary in its own zone, reachable only from the bank's payment gateway over mTLS
(research 25 s.15; C-55..C-58). It reads its local list index and pre-computed features, never the CBS.

### 4.1 Endpoints

| Method | Path | Scope | Description |
|---|---|---|---|
| POST | /assess/v1/payments | assess.call | Assess one payment; returns a signed decision |
| GET | /assess/v1/decisions/{id} | assess.call | Fetch a signed decision again (gateway reconciliation) |
| GET | /assess/v1/holds/{hold_ref} | assess.call | Status of a held payment: `held`, `released` (with a new signed `pass`), `rejected` |
| GET | /assess/v1/health | assess.call | Mode per rail, list freshness, degraded flags |

Phase 29 owns these rows (DR-292 to DR-316). The gateway is a service principal bound to a pinned certificate
(DR-292); the routes answer 404 `E_NOT_FOUND` while flag `pack.assess` is off; `GET /assess/v1/decisions/{id}` and
`GET /assess/v1/holds/{hold_ref}` answer only for the calling gateway.

### 4.2 Request and response

```text
POST /assess/v1/payments                         (mTLS, bank gateway certificate)
Idempotency-Key: <bank_id>:<rail>:<UETR or internal reference>:<attempt>
Content-Type: application/json
{
  "format": "mt103" | "pacs008" | "canonical",     // rails rtgs and ach send canonical until samples exist
  "raw": "<message text>",                     // or "canonical": { ... }
  "context": { "rail": "rtgs", "direction": "out", "branch": "...", "channel": "...",
               "operator_ref": "...", "debit_account_ref": "...", "value_date": "2027-03-01" }
}

200 OK
{
  "decision_id": "<uuidv7>",
  "decision": "pass" | "hold" | "reject",
  "mode": "shadow" | "advisory" | "enforcing",
  "reasons": [ { "code": "LIST_MATCH", "list": "iq_national", "field": "59a" },
               { "code": "HIGH_RISK_CORRIDOR" } ],
  "rule_id": "PROHIBITED_DESTINATION_CBI_ORDER_12",   // present only on a deterministic reject
  "hold_ref": "<opaque>",                            // present on hold
  "degraded": [],                                    // e.g. ["lists_stale", "screening_partial"]
  "expires_at": "2027-03-01T09:12:00Z",
  "signature": "<key_version>||<ed25519 over the canonical decision bytes>",
  "customer_message_code": "PENDING_REVIEW",         // generic, never the reason
  "fail_policy": "open" | "closed",                 // the matching fail-policy row, for the gateway's own copy
  "policy_version": 12                              // the policy set the decision used
}
```

Rules:

- **Idempotency.** Keys are scoped to the calling gateway. The same key and payload return the stored signed response
  byte for byte (original `expires_at`); the same key with a different
  payload -> 422 `E_IDEMPOTENCY_REUSE`; the same key while the first call runs -> 409 `E_IDEMPOTENCY_IN_FLIGHT`. After the
  replay window (`assess.idempotency_hours`) the gateway reconciles with `GET /assess/v1/decisions/{id}`.
- **People decide.** No score is returned and a score alone never rejects: rules and models produce at most `hold`.
  `reject` without a person only for deterministic, bank-configured prohibited conditions, with `rule_id`; every
  other reject is an analyst's decision on a hold (C-65).
- **Expiry.** A `pass` is valid only until `expires_at` (default 10 minutes); a gateway releasing later must call
  again. The release-without-decision control raises `pass_expired` for a release after `expires_at` plus
  `assess.release_skew_s`.
- **Signature.** Ed25519 with a key unwrapped from OpenBao at start-up and rotated by version prefix; OpenBao is
  never in the request path. The gateway refuses an unsigned or expired `pass`.
- **Modes.** In `shadow` the response is always `pass` with `mode: shadow` and the would-be decision is recorded; in
  `advisory` the gateway shows the decision without holding; in `enforcing` it holds. Mode changes per rail are change
  requests.
- **Degraded.** Lists stale by the phase 14 freshness rule turn cross-border decisions into `hold` with
  `lists_stale`; if the decision cannot be persisted, no `pass` is issued (503 `E_UNAVAILABLE`) and the gateway's fail
  policy applies. Every degraded mode is flagged in the response and counted.
- **Tipping-off.** `customer_message_code` is generic; reasons go to the gateway's operations view, never to the
  customer.
- **Targets.** Service side p95 150 ms, p99 400 ms, to be proven by load test; 50 requests a second per bank with 500
  burst on one node (C-56, N-50).

### 4.3 Fail policy

The fail policy (open or closed per rail, corridor, currency and amount band) is bank data in `assess.fail_policies`,
changed by change request; AKR-TMS publishes the recommended matrix (research 25 s.15.5) and the gateway enforces it,
because only the gateway can hold a message. Releases that have no matching signed decision are found from the
replica and listed (C-59).

## 5. OpenAPI and codegen flow (DBS)

1. Handler with `#[utoipa::path]` and registration in the `#[openapi(...)]` of its binary; an unregistered handler is
   a review blocker.
2. `cargo check` verifies SQLx queries against the live schema; `cargo sqlx prepare` refreshes `.sqlx/`. CBS reads
   are the one written exception: generated and parsed by sqlparser (tech-stack.md).
3. Backend up -> `make generate-api` -> Orval regenerates the client and models. Generated files are never edited.
4. CI fails if the committed client drifts from the spec.
5. `akr-assess` publishes its own OpenAPI document for the bank's gateway integration.

## 6. Problem codes (stable)

| Code | Status | Meaning |
|---|---|---|
| `E_BAD_REQUEST` | 400 | Malformed request |
| `E_IDEMPOTENCY_REQUIRED` | 400 | Mutation without `Idempotency-Key` |
| `E_UNAUTHENTICATED` | 401 | No valid session or token |
| `E_STEPUP_REQUIRED` | 401 | Missing, expired, reused or wrong-class step-up assertion |
| `E_SCOPE` | 403 | Token or session lacks the scope (the `/branch/*` routes answer 404 instead, DR-206) |
| `E_FORBIDDEN` | 403 | Cedar denied on a resource the caller may know exists |
| `E_SELF_APPROVAL` | 403 | Maker, proposer or earlier decider tried to decide |
| `E_CSRF_INVALID` | 403 | Missing or wrong CSRF token, or a foreign `Origin` (phase 01) |
| `E_CREDENTIAL_CHANGE_REQUIRED` | 403 | Confined session (password change, second-factor enrolment or undertaking pending) outside the setup routes (phase 01) |
| `E_NOT_FOUND` | 404 | Missing, or masked suspicion-tier resource (identical body) |
| `E_CLAIMED` | 409 | Item claimed by someone else |
| `E_WIP_LIMIT` | 409 | Work-in-progress limit reached |
| `E_VERSION_CONFLICT` | 409 | `If-Match` does not match; current version in the body |
| `E_IDEMPOTENCY_IN_FLIGHT` | 409 | Same key still executing |
| `E_STALE_ASSIGNEE` | 409 | Task assignment no longer valid at action time (DBS) |
| `E_STALE_BASE` | 409 | Change request base version moved |
| `E_CASE_SEALED` | 409 | Closed and sealed case; reopen first |
| `E_NOT_ALLOWED_IN_STATE` | 409 | Action not allowed in the resource's status |
| `E_FX_RATE_MISSING` | 409 | No CBI or manual rate for the currency and business date; nothing falls back (phase 09) |
| `E_MIN_FACTORS` | 409 | Removing the last second factor (phase 01) |
| `E_VALIDATION` | 422 | Field or document validation; `errors[]` (DSL.md s.7 codes) |
| `E_IDEMPOTENCY_REUSE` | 422 | Same key, different body |
| `E_EVIDENCE_STALE` | 422 | Change-request evidence older than allowed |
| `E_OPEN_QUERIES` | 422 | KYC review completion with open queries |
| `E_BELOW_FLOOR` | 422 | Override below a floor or the completeness cap |
| `E_PURPOSE_REQUIRED` | 422 | Search or export without a stated purpose |
| `E_REPORT_INVALID` | 422 | Report fails the goAML profile validation |
| `E_COST_GUARD` | 422 | Replay or query estimate above the cost guard (estimate in the body) |
| `E_INVARIANT_LOCKED` | 422 | A fixed clock or switch of the policy pack would change (phase 09) |
| `E_PRECONDITION_REQUIRED` | 428 | Versioned mutation without `If-Match` |
| `E_RATE_LIMITED` | 429 | Bucket empty; `Retry-After` |
| `E_UPLOAD_REJECTED` | 422 | Antivirus or type check failed |
| `E_PAYLOAD_TOO_LARGE` | 413 | Body above the route's size limit |
| `E_INTERNAL` | 500 | Unexpected failure; correlation id only, no detail |
| `E_HASH_MISMATCH` | 500 | Evidence failed its hash on serve (incident opened) |
| `E_SOURCE_REFUSED` | 409 | The source's latest grant check refused it (a write, DBA-class, ownership or uncovered link finding); finding codes in `errors[]` |
| `E_OBJECT_NOT_APPROVED` | 422 | A discovery or read names an object outside the source's approved list |
| `E_UNAVAILABLE` | 503 | Dependency down; for assess, no decision could be recorded |

## 7. Cross-cutting behaviours the spec must encode

- `X-StepUp-Assertion` is a documented securityScheme so Swagger shows it on every **SU** operation.
- Every list endpoint documents its Cedar filtering semantics (results post-filter; counts never include masked
  rows; no totals where restricted rows can appear).
- Every mutating endpoint documents its audit event kind; reads of customers, cases, reports and evidence, and every
  export, are audited too.
- Every response with CBS-derived data documents `data_as_of`.
- Every case, alert, report, task and change-request schema includes `affordances`.
- Suspicion-tier endpoints are listed in an inference test suite that checks 404 parity, count parity and search
  parity for a user outside the access list.
- Branch-window and operations-task schemas are reviewed for neutral wording: no field can carry an AML reason.
