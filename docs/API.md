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
- **Data freshness.** Every response that carries data derived from the CBS includes `data_as_of` (the as-of of the
  newest batch behind it) in the body and as `X-Data-As-Of`; the UI shows it on every screen and export (C-29).
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
- **Exports.** Exports are jobs: `POST` returns a job id; the result is fetched through a short-lived presigned link
  served by the S3 guard; every export needs **SU**, states its purpose, and is audited with its row count.
- **SSE.** `GET /api/v1/events` (session auth). Event types: `queue.counts`, `task.created`, `task.updated`,
  `case.updated`, `clock.warning`, `notification.new`, `job.updated`, `ingest.status` (operations roles only),
  `sse.reset`. Events carry ids only and go to the per-user channel of users who can see the resource; clients
  refetch through TanStack Query invalidation. Reconnect with `Last-Event-ID`.
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
| `qa.review` | QA samples and reviews | QA, M |
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
| `admin.users` | Users, grants, delegations, offboarding | ADM |
| `admin.policies` | Cedar bundles and simulation | ADM, SEC |
| `admin.system` | Settings, flags, jobs, releases, ops cockpit | ADM |
| `security.ops` | Break-glass approvals, access reviews, support-session approval | SEC |
| `support.readonly` | A support session opened by the bank (time-boxed, redacted) | VS |
| `assess.call` | `/assess/v1` only | Gateway client certificate |

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
| GET | /me/notifications | self | In-app notifications (cursor) [P1, phase 22] |
| POST | /me/notifications/{id}/read | self | Mark read [P1, phase 22] |

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
| POST | /admin/users | admin.users | Create or invite a user |
| PATCH | /admin/users/{id} | admin.users | Edit profile, branch scope, clearance |
| POST | /admin/users/{id}/grants | admin.users | Propose a role grant; **CR** (`cr_access`) |
| DELETE | /admin/users/{id}/grants/{grant_id} | admin.users | Revoke a grant at once (revocation needs no second approver) |
| POST | /admin/users/{id}/unlock | admin.users | Unlock an account locked after failed sign-ins |
| POST | /admin/users/{id}/reset-credentials | admin.users | Temporary password and second-factor reset (recovery, DR-30); **SU**; **4E** (ADM initiates, SEC confirms) |
| POST | /admin/users/{id}/offboard | admin.users | Disable identity, revoke sessions and tokens, end grants, sweep open tasks; **SU** |
| GET | /org/delegations | self | Delegations to and from me; all for admin |
| POST | /org/delegations | self | Delegate during leave within the authority matrix |
| DELETE | /org/delegations/{id} | self | End a delegation |
| GET | /org/authority-matrix | self | Who may decide what, by amount, tier and action |
| PUT | /org/authority-matrix | admin.users | Replace the matrix; **CR** |
| GET | /admin/access-reviews | security.ops | Quarterly access reviews |
| POST | /admin/access-reviews | security.ops | Start a review (snapshot of grants) |
| POST | /admin/access-reviews/{id}/sign | security.ops | Sign the review; **SU** |

### 3.3 Authorisation, tiers and access lists (M3) [P0]

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /admin/policies | admin.policies | Active Cedar bundle, schema and version |
| PUT | /admin/policies | admin.policies | Propose a new bundle; **CR** |
| POST | /admin/policies/simulate | admin.policies | Evaluate (principal, action, resource) against the active or a draft bundle |
| GET | /cases/{id}/acl | case.decide | Case access list (M and the case owner) |
| POST | /cases/{id}/acl | case.decide | Grant read, work or decide on a case; **SU** |
| DELETE | /cases/{id}/acl/{principal_id} | case.decide | Revoke; **SU** |
| POST | /access/break-glass | self | Request break-glass with reason and scope; **SU** |
| POST | /access/break-glass/{id}/approve | security.ops | Approve (two approvers, at most four hours); **SU**, **4E** |
| POST | /access/break-glass/{id}/end | self | End early |
| GET | /records/{kind}/{id}/access-history | audit.read | Who read or changed a record, with purpose |

### 3.4 Audit, evidence vault and retention (M4) [P0 capture, P1 packs]

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /audit/events | audit.read | Audit events (filters, cursor); the auditor's own search is logged |
| GET | /audit/chain-status | audit.read | Hash-chain and anchor status per partition |
| GET | /audit/anchors | audit.read | Signed Merkle anchors |
| POST | /audit/verify-runs | audit.read | Verify chains and anchors over a range |
| GET | /audit/verify-runs/{id} | audit.read | Result |
| GET | /evidence/{id} | queue.work | Presigned link through the S3 guard; hash checked on serve; audited; **SU** for suspicion tier |
| GET | /retention/holds | audit.read | Legal holds |
| POST | /retention/holds | report.file | Place a legal hold (party, case or range); **SU** |
| POST | /retention/holds/{id}/release | report.file | Release a hold; **SU**, **4E** |
| GET | /retention/report | audit.read | What is due for deletion, by class, and what is held |
| POST | /retention/dry-runs | admin.system | Simulate a retention run |
| GET | /retention/certificates | audit.read | Deletion certificates |

### 3.5 Sources, discovery and ingestion (M5) [P0 discovery, P1 production]

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /sources | data.steward | Registered sources with tier CT1-CT4, read-only level, status |
| POST | /sources | data.steward | Register a source (engine, connection reference in OpenBao, tier) |
| GET | /sources/{id} | data.steward | Detail, freshness class, last batch, paused state |
| PATCH | /sources/{id} | data.steward | Edit non-governed attributes |
| POST | /sources/{id}/probe | data.steward | Connect, check grants (fail closed on any write privilege), TLS handshake, character-set round trip |
| GET | /sources/{id}/grants | data.steward | Latest grant-check result and history |
| GET | /sources/{id}/load-budget | data.steward | The signed load budget in force |
| PUT | /sources/{id}/load-budget | data.steward | Propose a new budget signed by the bank's DBA; **CR** |
| POST | /sources/{id}/pause | data.steward | Pause extraction (bank request or incident) |
| POST | /sources/{id}/resume | data.steward | Resume; backfill is planned automatically |
| GET | /sources/{id}/watermarks | data.steward | Watermark registry with lookback windows |
| GET | /sources/{id}/batches | data.steward | Batches (as-of marker, rows, mapping versions, status) |
| GET | /batches/{id} | data.steward | Batch detail with reconciliation results |
| POST | /sources/{id}/backfills | data.steward | Plan a backfill under the load budget (slices, windows, days) |
| GET | /backfills/{id} | data.steward | Plan and progress with checkpoints |
| GET | /quarantine | data.steward | Quarantined rows (reason, source, status) |
| POST | /quarantine/{id}/retry | data.steward | Retry after a fix |
| POST | /quarantine/{id}/exclude | data.steward | Exclude with reason; appears in the coverage statement |
| POST | /discovery/runs | data.steward | Start a read-only discovery run [P0] |
| GET | /discovery/runs/{id} | data.steward | Progress and findings [P0] |
| GET | /discovery/profiles | data.steward | Profiles per object (row counts, null rates, distributions, charset) [P0] |
| GET | /discovery/field-availability | data.steward | Canonical attribute availability and quality per source [P0] |
| GET | /discovery/integration-profile | data.steward | Exportable integration profile and discovery checklist [P0] |

### 3.6 Mapping and canonical model (M6) [P0 draft, P1]

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /canonical/catalogue | config.author | Canonical entities and attributes with classification |
| GET | /mappings | config.author | Mappings per source and entity with the published version |
| POST | /mappings | config.author | Create a mapping family |
| GET | /mappings/{id} | config.author | Detail and version list |
| POST | /mappings/{id}/versions | config.author | Open a draft version |
| PUT | /mappings/{id}/versions/{v} | config.author | Replace the draft document (DSL.md Part B) |
| POST | /mappings/{id}/versions/{v}/validate | config.author | Schema and semantic validation |
| POST | /mappings/{id}/versions/{v}/test | config.author | Run on read-only sample rows and golden data |
| POST | /mappings/{id}/versions/{v}/preview | config.author | Canonical output beside the source rows |
| GET | /mappings/{id}/versions/{v}/impact | config.author | Rows changing value, scenarios touched, coverage change |
| POST | /mappings/{id}/replays | data.steward | Replay over the raw landing layer for a range; produces a diff |
| GET | /mapping-replays/{id} | data.steward | Diff summary and samples |
| POST | /mapping-replays/{id}/apply | data.steward | Apply a reviewed replay; **SU** |
| GET | /code-maps | config.author | Code maps by kind and source |
| GET | /code-maps/{id}/entries | config.author | Effective-dated entries; unmapped values listed first |
| POST | /code-maps/{id}/entries | config.author | Map or reclassify codes; **CR** |
| GET | /drift-events | data.steward | Drift events with paused tables |
| POST | /drift-events/{id}/resolve | data.steward | Resolve after the mapping change is published |
| GET | /sources/{id}/view-ddl-pack | data.steward | Generate the read-only view DDL pack for the bank's DBA |

### 3.7 Reconciliation, coverage and data quality (M7) [P0 gap report, P1]

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /coverage/cbi-gap-report | dashboard.read | The CBI coverage gap report: 173 items, readiness grade, missing fields, vendor request, interim route [P0] |
| POST | /coverage/cbi-gap-report/runs | data.steward | Regenerate the report and its PDF pack [P0] |
| GET | /coverage/readiness | config.author | Readiness grade A-D per scenario and source [P0] |
| GET | /recon/runs | data.steward | Reconciliation runs per batch and day |
| GET | /recon/runs/{id} | data.steward | Rung results (receipt, counts, control totals, GL tie-out) |
| GET | /recon/certificates | dashboard.read | Signed daily coverage certificates |
| GET | /recon/certificates/{business_date} | dashboard.read | One certificate with its signature and vault reference |
| GET | /coverage/statement | dashboard.read | MLRO coverage statement: accounts and transactions covered, data as of, blind periods |
| GET | /coverage/ledger | data.steward | Coverage per branch, product, transaction type and channel |
| GET | /coverage/exclusions | data.steward | Exclusions with owner, reason and expiry |
| POST | /coverage/exclusions | data.steward | Propose an exclusion; **CR** |
| GET | /coverage/blind-periods | dashboard.read | Blind periods with cause and statement |
| POST | /coverage/blind-periods | data.steward | Declare a blind period (also opened automatically) |
| GET | /dq/issues | data.steward | Data-quality issues with owners and thresholds |
| POST | /dq/issues/{id}/resolve | data.steward | Resolve with a note |
| GET | /dq/known-adjustments | data.steward | Bank-signed, expiring known-adjustments register |
| POST | /dq/known-adjustments | data.steward | Add an entry; **CR** |

### 3.8 Reference data and calendars (M8)

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /ref/countries | self | Countries with risk level, basis and table version |
| PUT | /ref/countries/{iso2}/override | config.author | Bank override with reason and expiry; **CR** |
| GET | /ref/fx-rates | self | CBI rates by date |
| POST | /ref/fx-rates | data.steward | Record a rate when the feed is missing (source and evidence) |
| GET | /ref/calendars | self | Calendars |
| GET | /ref/calendars/{id}/days | self | Holidays, Ramadan hours, Eid, Arbaeen, pay days, Hijri dates |
| POST | /ref/calendars/{id}/days | config.author | Add or change days; **CR** |
| GET | /ref/reason-codes | self | Reason-code library by family |
| POST | /ref/reason-codes | config.author | Add or revise a code; **CR** |
| GET | /ref/segments | config.author | Segment register with sizes and merges |
| POST | /ref/segments | config.author | Propose a segment definition; **CR** |
| GET | /ref/registers/{kind} | config.author | Governed registers (dual-use goods, vessels, sector-goods map, flagged districts, crypto counterparties) |
| POST | /ref/registers/{kind}/entries | config.author | Add or change entries; **CR** |
| GET | /ref/typologies | self | Shared typology tag set |

### 3.9 Screening and name matching (M9)

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /lists | screening.work | Lists with refresh interval and active version |
| GET | /lists/{id}/versions | screening.work | Versions with counts, checksums, status |
| POST | /lists/{id}/versions | screening.work | Upload a national list file when no feed exists; **SU** |
| GET | /lists/{id}/versions/{v}/diff | screening.work | Additions, removals, amendments against the previous version |
| POST | /screening/search | screening.work | Name search with purpose (audited, rate-limited) |
| GET | /screening/runs | screening.work | Daily base, look-back and onboarding runs |
| POST | /screening/lookbacks | screening.work | Start a look-back for a list change |
| GET | /screening/alerts | screening.work | Screening alerts (priority class, clock) |
| GET | /screening/alerts/{id} | screening.work | Alert with aligned names, identifiers, evidence bits |
| POST | /screening/alerts/{id}/claim | screening.work | Claim |
| POST | /screening/alerts/{id}/decision | screening.work | True, potential or incorrect with reasons; **SU**; a true match on a freeze list starts the freeze workflow (M14) |
| GET | /screening/pair-memory | screening.work | Remembered pairs with expiry |
| POST | /screening/pair-memory | screening.work | Propose a remembered pair; **4E** |
| DELETE | /screening/pair-memory/{id} | screening.work | Invalidate |
| GET | /screening/good-guys | screening.work | Good-guy rules |
| POST | /screening/good-guys | screening.work | Propose a rule; **CR** |
| GET | /screening/harness-runs | screening.work | Known-answer harness results per list version |
| POST | /screening/harness-runs | screening.work | Run the harness |

### 3.10 Customer risk rating and KYC (M10)

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /parties/{id}/rating | kyc.work | Current rating, CBI label, floors, completeness, controls |
| GET | /parties/{id}/rating/history | kyc.work | Rating history with model versions |
| GET | /parties/{id}/rating/explanation | kyc.work | Explanation panel data (drivers, gaps, triggers, overrides) |
| POST | /parties/{id}/overrides | kyc.work | Propose an override; downward starts `kyc_override_down`; **SU** |
| GET | /overrides | kyc.work | Overrides with expiry and analytics filters |
| GET | /kyc/reviews | kyc.work | Review queue (periodic, event, light yearly, EDD) |
| GET | /kyc/reviews/{id} | kyc.work | Review workspace pre-filled with new and changed facts |
| POST | /kyc/reviews/{id}/claim | kyc.work | Claim |
| PATCH | /kyc/reviews/{id} | kyc.work | Save checklist progress and evidence slots |
| POST | /kyc/reviews/{id}/complete | kyc.work | Complete; no sign-off with open queries; **SU** |
| GET | /pep/register | kyc.work | PEP register with tiers and lifecycle |
| POST | /pep/register | kyc.work | Add an entry |
| PATCH | /pep/register/{id} | kyc.work | Change tier, office dates, persistence |
| POST | /pep/register/{id}/relatives | kyc.work | Confirm a relative or associate link (human confirmation) |
| GET | /parties/{id}/ownership | kyc.work | Ownership chain, effective percentages, loops |
| GET | /exit-register | kyc.work | Customers exited for financial-crime reasons |
| POST | /exit-register | kyc.work | Add an entry (from an exit decision) |
| GET | /rating/feed | kyc.work | One-way rating-change feed for the bank (cursor) |
| GET | /rating/divergence | kyc.work | Divergence against the CBS risk field |
| GET | /rating/distribution | dashboard.read | Distribution, sanity bands, escapes |
| GET | /rating-models | config.author | Model versions |
| POST | /rating-models/versions | config.author | Open a draft (DSL.md Part C) |
| PUT | /rating-models/versions/{v} | config.author | Replace the draft |
| POST | /rating-models/versions/{v}/validate | config.author | Validation |
| POST | /rating-models/versions/{v}/shadow | config.author | Shadow run on production data |
| GET | /rating-models/versions/{v}/shadow | config.author | Disagreement list, tier migration, sensitivity |

### 3.11 Transaction monitoring and scenario studio (M11) [P0 register, P1]

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /cbi-catalogue | self | The 173 CBI items with their implementing scenarios [P0] |
| POST | /cbi-catalogue/imports | config.author | Import a CBI or Office issuance as draft items with data requirements |
| GET | /coverage/matrix | dashboard.read | Coverage matrix: CBI item to scenario to readiness to alerts to cases to STRs |
| GET | /scenarios | config.author | Scenario register (pack, family, live version, status, readiness) |
| POST | /scenarios | config.author | Create a bank scenario (901-999) |
| GET | /scenarios/{id} | config.author | Detail with versions and parameter sets |
| POST | /scenarios/{id}/versions | config.author | Open a draft version |
| PUT | /scenarios/{id}/versions/{v} | config.author | Replace the draft (DSL.md Part A) |
| POST | /scenarios/{id}/versions/{v}/validate | config.author | Schema and semantic validation |
| POST | /scenarios/{id}/versions/{v}/test | config.author | Unit and seeded cases |
| GET | /scenarios/{id}/parameter-sets | config.author | Parameter-set versions with basis and reason |
| POST | /scenarios/{id}/parameter-sets | config.author | Draft a parameter set (then a change request) |
| POST | /scenarios/{id}/replays | config.author | Replay a version and parameter set over an as-of period; cost-guarded |
| GET | /replays/{id} | config.author | Replay summary (by segment, branch, month; reach; overlap; look-ahead flag) |
| GET | /replays/{id}/results | config.author | Per-entity results (cursor) |
| POST | /replays/{id}/samples | config.author | Generate the above- and below-the-line sampling plan |
| POST | /samples/{id}/labels | queue.work | Record a blind reviewer's label on a sampled item |
| POST | /scenarios/{id}/shadow | config.author | Start a shadow run of a version |
| GET | /scenarios/{id}/shadow | config.author | Shadow comparison with the live version |
| POST | /scenarios/{id}/kill-switch | config.approve | Fall back to the previous approved version; **SU** |
| GET | /scenarios/{id}/card | dashboard.read | The generated scenario card |
| GET | /scenarios/{id}/metrics | dashboard.read | Performance metrics with red and amber status |
| GET | /scenarios/{id}/harness-diff | model.risk | Independent re-implementation diff (simulated versus deployed) |
| GET | /seeded-tests | config.author | Known-answer suite with last results |
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
| GET | /cases/{id} | queue.work | Case with affordances; **SU** for suspicion tier |
| GET | /cases/{id}/timeline | queue.work | Event trail |
| POST | /cases/{id}/assign | queue.work | Assign within the pool rules |
| POST | /cases/{id}/notes | queue.work | Add a note (immutable once posted; amendments link to it) |
| POST | /cases/{id}/evidence | queue.work | Upload evidence (antivirus, hash, encryption before the vault) |
| POST | /cases/{id}/proposals | queue.work | Propose no report, UTR, STR, TFR, close or escalate with reasons and checklist; starts the approval workflow |
| GET | /cases/{id}/decisions | queue.work | Decisions with versions and AI involvement |
| POST | /cases/{id}/decisions | case.decide | Decide on the open proposal (acts on my approval task); **SU**, **4E** |
| POST | /cases/{id}/escalate | queue.work | Escalate to the next tier |
| POST | /cases/{id}/merge | queue.work | Merge into another open case (never hides; both trails kept) |
| POST | /cases/{id}/reopen | case.decide | Reopen a closed case with reason; **SU** |
| POST | /cases/{id}/hibernations | case.decide | Hibernate a scenario for the party after a not-suspicious decision (envelope, expiry) |
| GET | /hibernations | queue.work | Hibernation ledger with regret metric |
| GET | /watches | queue.work | UTR watches with triggers and expiry |
| GET | /parties/{id} | queue.work | Party page (profile, accounts, rating, alerts and cases I may see) |
| GET | /parties/{id}/transactions | queue.work | Transactions from the canonical store (filters, cursor) |
| GET | /parties/{id}/graph | queue.work | Network around the party (kinship hidden by default; showing it is logged) |
| POST | /cases/{id}/rfis | queue.work | Draft an RFI (branch, customer via branch, correspondent) |
| GET | /rfis | rfi.desk | RFI desk |
| GET | /rfis/{id} | rfi.desk | RFI with deadline clock and answers |
| POST | /rfis/{id}/release | rfi.desk | Release after the second signature; **SU**, **4E** |
| POST | /rfis/{id}/answers | rfi.desk | Record answers as claims (source, corroboration, expiry) |
| GET | /authority-requests | case.decide | FIU, court and prosecutor requests (suspicion tier) |
| POST | /authority-requests | case.decide | Register a request with deadline; triggers re-rating (TR-17); **SU** |
| GET | /authority-requests/{id} | case.decide | Detail with linked cases and customers |
| GET | /internal-reports | case.decide | Internal reports from staff and liaison officers |
| POST | /internal-reports/{id}/acknowledge | case.decide | Acknowledge receipt (clock) |
| GET | /qa/samples | qa.review | QA samples |
| POST | /qa/samples | qa.review | Draw a sample (independent reviewer, never one's own cases) |
| POST | /qa/samples/{id}/review | qa.review | Score with findings |
| POST | /qa/bulk-reopen | qa.review | Reopen a reviewer's items after a critical finding; **SU** |
| GET | /qa/calibration | qa.review | Calibration kit and agreement |

### 3.13 Reporting to the AML/CFT Office (M13) [P0 checker, P1]

| Method | Path | Scope | Description |
|---|---|---|---|
| POST | /goaml/check | report.prepare | Upload goAML XML files for validation and grading; returns a run id [P0] |
| GET | /goaml/check-runs | report.prepare | Checker runs [P0] |
| GET | /goaml/check-runs/{id} | report.prepare | Run summary with grades [P0] |
| GET | /goaml/check-runs/{id}/findings | report.prepare | Findings with the fix for each [P0] |
| GET | /goaml/profiles | report.prepare | goAML profiles (XSD version, patches, lookups, rules, clocks) [P0] |
| GET | /goaml/profiles/{id} | report.prepare | Profile detail [P0] |
| PUT | /goaml/profiles/{id} | config.author | Propose a profile change; **CR** |
| GET | /goaml/watch-runs | report.prepare | XSD and attachment watcher results |
| POST | /cases/{id}/reports | report.prepare | Create a report draft (STR, SAR, TFR, UTR, AIF) from the case |
| GET | /reports | report.prepare | Reports (suspicion tier; post-filter) |
| GET | /reports/{id} | report.prepare | Draft or final with validation state; **SU** |
| PATCH | /reports/{id} | report.prepare | Edit the draft |
| POST | /reports/{id}/validate | report.prepare | Validate against the profile (XSD and business rules) |
| POST | /reports/{id}/submit | report.prepare | Submit for approval (`case_report_approval`) |
| POST | /reports/{id}/sign | report.file | Sign as the named officer (acts on the `sign` task); **SU**, **4E** |
| POST | /reports/{id}/package | report.file | Build the XML package and stamped PDF after signature |
| GET | /reports/{id}/package | report.file | Download the package (audited); **SU** |
| POST | /reports/{id}/filing | report.file | Record the goAML reference and acknowledgment after the officer uploads; **SU** |
| POST | /reports/{id}/rejections | report.file | Record an Office rejection and open the correction |
| POST | /reports/{id}/feedback | report.file | Record the FIU grade |
| GET | /ctr/batches | report.prepare | Daily CTR batches (per customer per day, USD equivalent at the CBI rate) |
| GET | /ctr/batches/{id} | report.prepare | Batch with GL tie-out |
| POST | /ctr/batches/{id}/package | report.file | Package for filing |
| POST | /letters | report.file | Generate an official letter (freeze report, reply to a request, Committee referral) |
| GET | /letters/{id} | report.file | Letter with PDF and delivery record |
| GET | /reports/periodic | dashboard.read | Periodic statistics for the Office and the board |

### 3.14 Operations tasks and freezes (M14)

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /ops-tasks | ops.tasks | Neutral task list (authority, instruction, clock); no AML reason |
| GET | /ops-tasks/{id} | ops.tasks | Task detail |
| POST | /ops-tasks/{id}/claim | ops.tasks | Claim |
| POST | /ops-tasks/{id}/complete | ops.tasks | Complete with the CBS reference |
| GET | /ops-tasks/{id}/verification | case.decide | Verification level V0-V3 reached from the canonical copy |
| POST | /ops-tasks/{id}/attest | case.decide | Record a V3 attestation with evidence; **SU** |
| GET | /freezes | case.decide | Freeze board (freeze, Committee report, second report clocks; post-order debits) |
| GET | /freezes/{id} | case.decide | Freeze detail with letters |
| POST | /freezes/{id}/release-decision | case.decide | Release after delisting on a recorded decision; **SU**, **4E** |
| GET | /refrain-instructions | case.decide | Office refrain instructions with the 7-working-day clock |
| POST | /refrain-instructions | case.decide | Register an instruction; creates the operations task; **SU** |
| POST | /exits | case.decide | Propose an exit with individual assessment; starts `exit_decision` |

### 3.15 Governance, change control, models and AI (M15) [P1 change control, P2 models and assistant]

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /dsl/{kind}/schema | config.author | JSON Schema for `scenario`, `mapping`, `rating_model`, `change_request` |
| GET | /change-requests | config.author | Change requests (filters: kind, class, status, mine) |
| POST | /change-requests | config.author | Create (class, volume effect and route computed; DSL.md Part D) |
| GET | /change-requests/{id} | config.author | Detail with diff, evidence, impact, route |
| POST | /change-requests/{id}/submit | config.author | Submit for approval |
| POST | /change-requests/{id}/approve | config.approve | Approve (acts on my task in the route); **SU**, **4E** |
| POST | /change-requests/{id}/reject | config.approve | Reject with comment; **SU** |
| POST | /change-requests/{id}/withdraw | config.author | Withdraw before a decision |
| GET | /inventory | model.risk | Inventory of rules, models and tools (tier, owner, validator, committee) |
| POST | /inventory | model.risk | Register an item before use |
| PATCH | /inventory/{id} | model.risk | Update owner, tier, validation dates |
| GET | /registers/{kind} | dashboard.read | Registers: decisions, exclusions, findings, warnings, fairness |
| POST | /registers/{kind}/entries | config.author | Add an entry |
| GET | /models | model.risk | Model registry [P2] |
| POST | /models | model.risk | Register a signed ONNX artifact with its card [P2] |
| GET | /models/{id} | model.risk | Card, validation, drift, label gates [P2] |
| POST | /models/{id}/validation | model.risk | Record validation results [P2] |
| POST | /models/{id}/shadow | model.risk | Start shadow or challenger [P2] |
| POST | /models/kill-switch | model.risk | Fall back to rules only; **SU** [P2] |
| POST | /assistant/drafts | queue.work | Ask the on-premises assistant for a draft within the permitted uses [P2] |
| POST | /assistant/drafts/{id}/accept | queue.work | A person accepts or edits the draft; logged as AI involvement [P2] |
| GET | /assistant/evaluations | model.risk | Evaluation gate results (at least 200 QA-confirmed cases, Arabic and English) [P2] |

### 3.16 Command centre, registers and evidence packs (M16) [P0 gap report, P1, P2]

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /dashboard/command-centre | dashboard.read | What is late, decisions waiting, clocks, arrivals against throughput, incidents |
| GET | /dashboard/clocks | dashboard.read | All regulatory and internal clocks with status |
| POST | /packs | dashboard.read | Build a pack (kind: gap_report [P0], board, case, report, assessor, correspondent [P2]); a job |
| GET | /packs/{id} | dashboard.read | Pack status and contents list |
| GET | /packs/{id}/download | dashboard.read | Download (audited); **SU** |
| POST | /packs/verify | self | Verify a pack or document hash against the vault |
| GET | /assessor/d2-matrix | assessor.read | D2 test-script map with live evidence links |
| POST | /assessor/samples | assessor.read | Draw a random sample (recorded) |
| GET | /assessor/trace/{alert_id} | assessor.read | End-to-end trace: transaction, alert, case, report |
| GET | /liaison-reports | case.decide | Liaison monthly reports for review |
| POST | /liaison-reports/{id}/review | case.decide | Review with comments |
| GET | /training/register | dashboard.read | Training register |
| POST | /training/register | admin.users | Record training |
| GET | /capacity/forecast | dashboard.read | Capacity workbench (arrivals, throughput, staffing) [P2] |
| GET | /typology-exposure | dashboard.read | Typology exposure index by bank, branch, correspondent [P2] |

### 3.17 Branch window and liaison (M17)

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /branch/items | branch.window | Items for my branch with neutral statuses only |
| GET | /branch/items/{id} | branch.window | Item with the neutral question and deadline |
| POST | /branch/items/{id}/input | branch.window | Answer with evidence |
| POST | /branch/observations | branch.window | Record an observation (eight groups, companion ID, written acknowledgment) |
| POST | /branch/confidential-reports | branch.window | Confidential report to the MLRO; branch management cannot see it |
| GET | /branch/monthly-report | branch.window | Draft monthly liaison report |
| POST | /branch/monthly-report/sign | branch.window | Sign and send; **SU** |

### 3.18 Islamic banking pack (M18) [optional per bank]

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /islamic/deals | queue.work | Deals for a party (contract type, stage, evidence status) |
| GET | /islamic/deals/{id} | queue.work | Deal view: parties by role, asset, parallel contracts, possession evidence |
| POST | /islamic/deals/{id}/evidence | queue.work | Attach possession or supplier evidence |
| GET | /islamic/sharia-board/aggregates | dashboard.read | Aggregate figures only for the Sharia board |

### 3.19 Assessment administration (M19) [P2]

The decision endpoint itself is on `akr-assess` (s.4). These are the analyst and configuration endpoints on
`akr-api`.

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /assess/fail-policies | config.author | Fail policy per rail, corridor, currency and amount band |
| PUT | /assess/fail-policies | config.author | Propose a change; **CR** |
| GET | /assess/modes | config.author | Mode per rail (shadow, advisory, enforcing) |
| PUT | /assess/modes/{rail} | config.author | Propose a mode change; **CR** |
| GET | /assess/holds | screening.work | Held payments queue |
| GET | /assess/holds/{id} | screening.work | Hold with the decision, reasons and payment parties |
| POST | /assess/holds/{id}/claim | screening.work | Claim |
| POST | /assess/holds/{id}/decision | screening.work | Release or reject with a recorded reason; **SU**, **4E** |
| GET | /assess/decisions | audit.read | Signed decisions (search) |
| GET | /assess/release-without-decision | dashboard.read | Payments released without a matching signed decision (C-59) |
| GET | /assess/health | admin.system | Latency, list freshness, degraded modes, node status |
| POST | /assess/control-tests | admin.system | Record the weekly control-test payment result |

### 3.20 Admin, settings, releases and ops cockpit (M20) [P0 minimal]

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /admin/settings | admin.system | Settings with CBI floors shown |
| PUT | /admin/settings/{key} | admin.system | Change a setting; **CR** when the key carries a CBI floor |
| GET | /admin/flags | admin.system | Feature flags |
| PUT | /admin/flags/{key} | admin.system | Toggle a flag (audited) |
| GET | /admin/ops | admin.system | Ops cockpit: queue depth, dead jobs, backups, anchors, disk, certificates, OpenBao seal state |
| GET | /admin/jobs | admin.system | Jobs by kind and state |
| POST | /admin/jobs/{id}/retry | admin.system | Retry a dead job |
| GET | /admin/releases | admin.system | Imported release bundles [P1] |
| POST | /admin/releases/verify | admin.system | Verify a bundle (signature, SBOM diff, notes, rollback plan) [P1] |
| POST | /admin/support-sessions | admin.system | Open a time-boxed support session for vendor support [P1] |
| POST | /admin/support-sessions/{id}/approve | security.ops | Approve it; **SU** [P1] |
| DELETE | /admin/support-sessions/{id} | admin.system | End it [P1] |
| GET | /help/articles | self | Bilingual help articles and "what changed" [P1] |

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
| POST | /admin/tasks/{id}/reassign | admin.system | Reassign a stuck task with reason and second approver; **SU**, **4E** |
| GET | /admin/workflow-definitions | admin.system | Definitions with versions |
| POST | /admin/workflow-definitions | admin.system | Create a family from a DSL body |
| PUT | /admin/workflow-definitions/{id}/draft | admin.system | Replace the draft |
| POST | /admin/workflow-definitions/{id}/validate | admin.system | Validation (DBS catalogue plus AKR codes) |
| POST | /admin/workflow-definitions/{id}/test-run | admin.system | Dry run with simulated actors |
| POST | /admin/workflow-definitions/{id}/publish | admin.system | Publish; **CR** (two administrators) |

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

### 4.2 Request and response

```text
POST /assess/v1/payments                         (mTLS, bank gateway certificate)
Idempotency-Key: <bank_id>:<rail>:<UETR or internal reference>:<attempt>
Content-Type: application/json
{
  "format": "mt103" | "pacs008" | "rtgs" | "ach" | "canonical",
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
  "degraded": [],                                    // e.g. ["ml_unavailable", "lists_stale"]
  "expires_at": "2027-03-01T09:12:00Z",
  "signature": "<key_version>||<ed25519 over the canonical decision bytes>",
  "customer_message_code": "PENDING_REVIEW"          // generic, never the reason
}
```

Rules:

- **Idempotency.** The same key and payload return the stored response byte for byte; the same key with a different
  payload -> 422 `E_IDEMPOTENCY_REUSE`; the same key while the first call runs -> 409 `E_IDEMPOTENCY_IN_FLIGHT`.
- **People decide.** No score is returned and a score alone never rejects: rules and models produce at most `hold`.
  `reject` without a person only for deterministic, bank-configured prohibited conditions, with `rule_id`; every
  other reject is an analyst's decision on a hold (C-65).
- **Expiry.** A `pass` is valid only until `expires_at` (default 10 minutes); a gateway releasing later must call
  again.
- **Signature.** Ed25519 with a key unwrapped from OpenBao at start-up and rotated by version prefix; OpenBao is
  never in the request path. The gateway refuses an unsigned or expired `pass`.
- **Modes.** In `shadow` the response is always `pass` with `mode: shadow` and the would-be decision is recorded; in
  `advisory` the gateway shows the decision without holding; in `enforcing` it holds. Mode changes per rail are change
  requests.
- **Degraded.** Lists older than their refresh threshold turn cross-border decisions into `hold` with
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
| `E_SCOPE` | 403 | Token or session lacks the scope |
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
| `E_MIN_FACTORS` | 409 | Removing the last second factor (phase 01) |
| `E_VALIDATION` | 422 | Field or document validation; `errors[]` (DSL.md s.7 codes) |
| `E_IDEMPOTENCY_REUSE` | 422 | Same key, different body |
| `E_EVIDENCE_STALE` | 422 | Change-request evidence older than allowed |
| `E_OPEN_QUERIES` | 422 | KYC review completion with open queries |
| `E_BELOW_FLOOR` | 422 | Override below a floor or the completeness cap |
| `E_PURPOSE_REQUIRED` | 422 | Search or export without a stated purpose |
| `E_REPORT_INVALID` | 422 | Report fails the goAML profile validation |
| `E_COST_GUARD` | 422 | Replay or query estimate above the cost guard (estimate in the body) |
| `E_PRECONDITION_REQUIRED` | 428 | Versioned mutation without `If-Match` |
| `E_RATE_LIMITED` | 429 | Bucket empty; `Retry-After` |
| `E_UPLOAD_REJECTED` | 422 | Antivirus or type check failed |
| `E_PAYLOAD_TOO_LARGE` | 413 | Body above the route's size limit |
| `E_INTERNAL` | 500 | Unexpected failure; correlation id only, no detail |
| `E_HASH_MISMATCH` | 500 | Evidence failed its hash on serve (incident opened) |
| `E_SOURCE_WRITE_PRIVILEGE` | 409 | Probe refused: the source account holds a write-class privilege |
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
