# Module Plan — Features, Flows, Frontend

Status: draft for review (2026-10-07). The modules of AKR-TMS, what each does in which macro-phase (P0-P3, see
[ROADMAP.md](ROADMAP.md)), its main flows, an API sketch (full catalogue in [API.md](API.md)), its screens (IDs from
the screen catalogue in research file 31 section 10) and its edge cases. Requirement IDs in brackets are research
requirements; every module names the prefixes it owns so gap passes can check coverage.

---

## 0. Module Map

| # | Module | Macro-phase | Binaries | Owns requirement prefixes (primary) |
|---|---|---|---|---|
| M1 | Identity & Sessions | P0 | api | SP (identity rows) |
| M2 | Organisation, Users & Roles | P0 | api | SP (roles), UX (A1) |
| M3 | Authorisation, Sensitivity Tiers & Access Lists | P0 | api, all | SP (authorisation, tipping-off) |
| M4 | Audit Trail, Evidence Vault & Retention | P0 capture, P1 packs | all | SP (audit, retention), TS (vault) |
| M5 | Sources, Discovery & Ingestion | P0 discovery, P1 production | ingestd, worker | C (connection, ingestion) |
| M6 | Mapping & Canonical Model | P0 draft, P1 | worker, api | C (mapping, canonical) |
| M7 | Reconciliation, Coverage & Data Quality | P0 gap report, P1 | worker, api | C (reconciliation), L (coverage) |
| M8 | Reference Data & Calendars | P1 | worker, api | T (calendar), S (calendar) |
| M9 | Screening & Name Matching | P1 | worker, api, assess | N |
| M10 | Customer Risk Rating & KYC | P1 | worker, api | K, FL, TR |
| M11 | Transaction Monitoring & Scenario Studio | P0 register, P1 | worker, api | S, T (detection) |
| M12 | Alerts, Cases & Investigation | P1 | api, worker | M (alerts, cases, decisions, QA, RFI) |
| M13 | Reporting to the AML/CFT Office | P0 checker, P1 | worker, api | RP, R (regime packs) |
| M14 | Operations Tasks & Freezes | P1 | api, worker | M (M-32..M-36), C (C-60..C-64) |
| M15 | Governance, Change Control, Models & AI | P1 change control, P2 models and assistant | api, worker | GV, T (models, GenAI) |
| M16 | Command Centre, Registers & Evidence Packs | P0 gap report, P1, P2 packs | api, worker | UX (dashboards), S (evidence) |
| M17 | Branch Window & Liaison | P1 | api | M (M-43..M-45), S (S-22, S-23) |
| M18 | Islamic Banking Pack | P1 (optional) | worker, api | I |
| M19 | Assessment API (phase 2) | P2 | assess | C (C-55..C-59, C-65), T (T-43) |
| M20 | Admin, Settings, Releases & Ops Cockpit | P0 minimal, grows every phase | api | TS (platform), SP (operations) |

Cross-cutting prefixes: F (FATF, file 03), B, E, U, W (Basel, Egmont, UN, Wolfsberg, file 04), L (enforcement
lessons, file 20), UX (file 31), TS (file 32), and the market prefixes VO, VA, VB, VC, VD (files 07-18). Each of their
IDs is assigned to one module during gap pass 1 (docs/plan).

Role codes (research file 31 section 2.1): **A1** level-1 analyst, **A2** level-2 analyst, **SS** screening
specialist, **M** MLRO and deputy, **KYC** KYC reviewer, **RFI** RFI desk officer, **QA** QA reviewer, **OPS** bank
operations officer, **CFG** compliance configuration maker, **CHK** compliance approver (checker), **DS** data
steward, **CH** compliance head and board readers, **MRO** model-risk officer, **IA** internal auditor, **AS**
external assessor or CBI inspector, **L** branch liaison officer, **ADM** platform administrator, **SEC** security
officer, **VS** vendor support (nothing standing).

Screen IDs (X, W, S, K, D, R, T, G, B, A followed by a number) are file 31's and appear only on "Screens:" lines and
in section 21.2. There, R1 is the report builder (not retention class R1), D1 the scenario library (not decision D1)
and A1 the users screen (not a role).

---

## M1. Identity & Sessions [P0]

**Requirements:** SP-14, SP-15, TS-25 and the identity rows of file 30 s.6.

**Features by phase**
- P0: Kratos browser flows through the BFF in Arabic and English; mandatory MFA (WebAuthn first, TOTP minimum);
  forced first login; `/me` and session list; step-up AAL2 as a single-use ledger row; personal access tokens for
  scripted clients (scoped, AAL1 only); one-action offboarding (disable, revoke sessions and tokens, end grants,
  sweep open tasks).
- P1: generic OIDC upstream (Entra ID, ADFS, Keycloak) as configuration; session policy floors from the CBI minima.
- P2: Ory Polis for SAML and SCIM (optional).

**Flows:** sign in -> MFA -> session; sensitive action -> step-up dialog -> single-use assertion consumed by the
action; offboarding -> identity disabled -> sessions and tokens revoked -> open tasks returned to their queues.

**API sketch:** `POST /api/v1/auth/login/*`, `POST /auth/mfa/*`, `GET /me`, `GET/DELETE /me/sessions`,
`POST /auth/step-up`, `POST/DELETE /me/tokens`, `POST /admin/users/{id}/offboard`.

**Screens:** X1 sign-in and step-up dialog, X4 profile and preferences, A1 users (offboarding action).

**Edge cases:** step-up assertion replayed (refused, single use); federated user disabled upstream but holding a
local session (session revoked at next check); MFA device lost (recovery only through ADM + SEC, audited); clock skew
on TOTP (window of one step).
Phase file: docs/plan/phase-25.md (DR-239): the generic OIDC upstream (Entra ID, ADFS, Keycloak) as Kratos
configuration; a federated person is linked to a local identity only by an audited admin command, never by e-mail;
roles never come from the upstream and its `amr` never satisfies AAL2, step-up or `record_open`. Stated limit: an
upstream disable reaches the platform only through unlink or offboarding, so a live session ends at the idle timeout
or the 8 h lifetime (phase 25 Open doubts 4). SAML and SCIM: phase 31.
Phase file: docs/plan/phase-31.md (DR-348 to DR-353): SAML through Ory Polis (flag `pack.polis`) as the phase 25
generic OIDC upstream, no authority from the assertion; the edge exposes only the pinned Polis paths. SCIM through an
HMAC-verified webhook (`POST /directory/events`, service principal `polis-directory`): a create writes a pending
directory row and never a sign-in account; the link is an ADM act with step-up; a deactivation disables the linked
account (phase 03, reversible) and never offboards; groups never grant.

## M2. Organisation, Users & Roles [P0]

**Requirements:** SP role and segregation rows (file 30 s.7.2), UX-53, M-20 (delegated-authority matrix), M-43
(liaison officers).

**Features by phase**
- P0: branches and regions imported from the CBS reference tables (seed, then admin-managed), analyst pools, users
  mirror, role grants with a validity window and vetting status, delegations with windows, the liaison officer and
  alternate per branch.
- P1: delegated-authority matrix (who may decide what, up to which class) signed by the MLRO (phase 09, DR-83: `cr_authority`, delegations refused until the first matrix is signed); leave ledger that
  re-routes work; quarterly access review.

**Flows:** grant a role -> change request (maker ADM, checker SEC) -> effective at time -> audited; leave -> open work
re-routed by policy -> returned on return if unclaimed.

**API sketch:** `GET/POST /api/v1/org/branches`, `GET/POST /org/pools`, `GET/POST /admin/users`,
`POST /admin/users/{id}/grants`, `POST /org/delegations`, `GET/PUT /org/authority-matrix`.

**Screens:** A1 users, roles and leave.

**Edge cases:** a user holding both maker and checker roles (allowed, but never on the same change: M3 `forbid`);
liaison officer on leave (alternate receives branch items); branch merged or closed in the CBS (mapping drift ->
admin decision, history kept).
Phase file: docs/plan/phase-25.md (DR-234): the quarterly access review snapshots `AccessMatrixBuilder::build(at)`
and the grants in force as one evidence object; SEC decides every item and M the business-role items, nobody their
own grant; a revocation is a phase 04 change request (`role_grant` op `revoke`, route `cr_access_adm`); the signer is
not the starter (step-up `approval`); overdue opens one phase 24 register warning. Inspector accounts reuse role `AS`
with `valid_to` (DR-241).

## M3. Authorisation, Sensitivity Tiers & Access Lists [P0]

**Requirements:** SP-19, SP-21, SP-24, SP-26, SP-27, SP-31, SP-33, TS-26, M-65, M-69, RP-62, F-16.

**Features by phase**
- P0: Cedar policy bundle in the database with hot reload; two gates (scope, policy); attribute resolution with a
  Valkey cache and invalidation; deny by default; decision logging into audit; simulator and dry run; existence
  masking (404) for denied suspicion-tier resources; segregation-of-duties `forbid` rules.
- P1: sensitivity tiers (normal, restricted, suspicion) computed from customer and case facts; case access lists;
  purpose-of-access capture; break-glass with two approvers and a four-hour limit; access history per record;
  inference tests (counts, search, exports, errors) in CI.

**Flows:** request -> scope check -> Cedar evaluation with principal, action, resource attributes -> allow, deny
(403) or mask (404) -> decision audited. Break-glass: request with reason -> two approvers (CH + SEC; M notified; phase 25 DR-242) -> time-boxed
grant -> every action flagged -> automatic expiry -> review.

**API sketch:** `GET/PUT /api/v1/admin/policies`, `POST /admin/policies/simulate`, `POST /access/break-glass`,
`POST /access/break-glass/{id}/approve`, `GET /records/{kind}/{id}/access-history`.

**Screens:** X6 not found / no access, G5 access and insider controls.

**Edge cases:** a customer enters the suspicion tier while an analyst has the page open (next fetch masks; live
update never reveals why); counts on queues must not reveal masked items; an export started before a tier change
(re-checked at download); policy bundle with a syntax error (refused, previous bundle stays).
Phase file: docs/plan/phase-25.md (DR-232, DR-233, DR-242, DR-243): break-glass is its own table, not a `case_acl`
row; the requester is A2, the approvers one CH and one SEC (never the requester; step-up `break_glass`); the requester
reaches the case through the team clause only under purpose `break_glass`, read-only (Cedar allow-list of 15 read
actions), at most four hours (database CHECK, predicate stops at `expires_at`), every use in the audit trail and the
dedicated log, review within 24 hours, two count-only notice codes (waiting: CH and SEC; opened: SEC and M); every request answers the same
202, so nothing tells whether a case exists. Screen G5 gains the break-glass list and the access-review tiles.

## M4. Audit Trail, Evidence Vault & Retention [P0 capture, P1 packs]

**Requirements:** SP-39, SP-42, SP-43, SP-44, SP-53, SP-56, SP-59, SP-60, M-04, M-05, M-59, M-61, TS-22, TS-41..TS-43.

**Features by phase**
- P0: audit write API (crate) used by every mutation and every sensitive read; monthly partitions with a per-partition
  hash chain; Merkle anchors every five minutes signed by OpenBao transit and copied to a COMPLIANCE-locked bucket;
  partition maintenance job; MinIO (Silo build) behind the S3 guard with the WORM conformance suite; encryption before
  upload with wrapped keys.
- P1: audit viewer and chain verifier; SIEM export (syslog, CEF); evidence objects with SHA-256 on write and serve,
  antivirus, language tag, lock status; case sealing; retention engine (classes R1-R13, jurisdiction profiles, legal
  holds, partition drop with hold copy, record purge on relationship-end clocks, crypto-shredding, deletion
  certificates replayed after restore); retention dry run.

**Flows:** mutation -> domain write + audit row in the same transaction -> anchor job batches the chain heads ->
signed anchor -> WORM copy. Retention: nightly job -> eligible partitions and records -> hold check -> copy held rows
-> drop or purge -> deletion certificate.

**API sketch:** `GET /api/v1/audit/events`, `GET /audit/chain-status`, `POST /audit/verify-runs`,
`GET /evidence/{id}` (audited, presigned), `GET/POST /retention/holds`, `GET /retention/report`,
`POST /retention/dry-runs`.

**Screens:** G6 audit viewer and chain verifier, G7 retention and legal holds, K3 evidence vault viewer.

**Edge cases:** anchor signing while OpenBao is sealed (anchors queue, gap visible in the cockpit, chain still
verifiable); a hold placed after a partition drop was scheduled (drop blocked); restore from backup (deletion
certificates replayed before reopening); an auditor searching the audit trail (their searches are audited).
Phase file: docs/plan/phase-23.md (DR-184 to DR-200): one nightly engine planned from the phase 00 assignment table
(the dry run is the same plan without writes), governed per-class periods with the Iraq floors, partition drops only
through the guarded `admin.retention_drop_partition` with held rows copied first into a sealed evidence object, record
purge of R3 and R8 by party on the relationship-end clock with keyed tombstones, crypto-shredding of owner-following
evidence, the evidence lock extension at case closure, signed deletion certificates and manifests in WORM replayed
after a restore, legal holds (party, case, range) placed by M under step-up and released by the four-eyes workflow
`legal_hold_release`, case holds hidden from IA and SEC, evidence orphan reconciliation; the new role `akr_retention`;
screen G7. Also carries SP-58 (lock extension, assigned by phase 02). Not carried: the audit viewer and verifier
routes and G6 (phase 24), the purge of R5-R7 records outside partitions and of R9 and R11 history (reported as due,
DR-191), the jurisdiction profiles (Open doubt 9).
Phase file: docs/plan/phase-24.md (DR-218 to DR-231): the audit viewer (every search written as `audit.searched`,
`purpose` required), one-event view, chain and anchor status, verify runs as a job (counts and finding kinds only)
and the per-record access history, all read through a new akr-api read-only pool; suspicion-tier rows are left out
for IA and SEC through the SECURITY DEFINER `audit.resource_tier`, which mirrors phase 17 tier rule 2; screen G6.
Phase file: docs/plan/phase-25.md (DR-235, DR-241, DR-245): SIEM forwarding (CEF or JSON over TLS syslog from
akr-worker, zone `z2_bank`) and the pull API `GET /siem/events` (scope `siem.pull`, service principals), both through
SECURITY DEFINER `audit.siem_page`: `siem`-flagged kinds only, no case id, no tier value, resource fields dropped for
every non-normal row; a cursor that survives restarts, dead letters by id; the RFC 3161 time stamp per anchor (off by
default).

## M5. Sources, Discovery & Ingestion [P0 discovery, P1 production]

**Requirements:** C-01..C-24, C-35, C-36, C-38, C-39, C-41..C-44, C-54, C-61, C-62, C-64, TS-04, TS-07..TS-09.

**Features by phase**
- P0 (discovery, early piece; phase 05): connect to a bank's CBS copy through CT1, CT2 or CT4 as a bank landing
  database (CT3 and CT4 files in P1, DR-63); the credential reaches only `akr-ingestd` (DR-59); grant check on
  connect against an allow-list per engine (fail closed, DR-60); every statement proven a bounded single-table
  `SELECT` inside the engine's read-only mode (DR-61); schema catalogue (a dictionary-only first run; data is read
  only from objects the bank approved by change request, under a signed load budget, DR-62) (tables matched by exact name only; candidates ranked by liveness: recent rows, indexes,
  use in code; copies, backups and scratch tables reported as drift candidates and never used as sources; the CBS's
  own field and code dictionaries read where they exist) and data profiling (row counts by exact count on small tables
  and bounded index ranges or block samples inside the load budget on large ones, never optimizer statistics
  (DR-40); null rates, value distributions, character-set round trip for Arabic and Kurdish, name data-quality
  profile); field availability against the canonical attributes that scenarios need, where a column present but
  constant, sentinel-only or nearly all null is graded D as "present but unpopulated", never available (DR-38) (feeds
  M7 and M11 for the data-readiness grades and the CBI coverage gap report).
- P0 grant detail (DR-40): the connect-time grant check expands roles through nested roles and refuses the DBA role
  and any DBA-class privilege, including one that arrives through a role, and any ANY system privilege outside the
  SELECT ANY family; it lists every database link the account can use and refuses the connection when one is usable,
  unless the bank signs an exception; the read-only proof states that no link was used.
- P1: production ingestion: `SourceReader` adapters (PostgreSQL, SQL Server, Oracle with an ODBC second backend,
  ODBC for Db2 and Sybase); a change strategy per table (DR-37): business-date ranges with a trailing re-read window
  (one watermark per branch where the CBS keeps a business date per branch), nightly snapshots with a row-hash diff for
  tables with no change date, and a periodic full snapshot diff where rows can be deleted, never a time-of-day column;
  every source read is a single-table key range with no join on the source; signed load budget and kill switch;
  "paused by bank" state; raw landing with lineage hashes and batch receipts, with columns that carry authentication
  or card data excluded before landing by a deny list in each adapter pack; payment messages (ISO 20022 and MT) with
  the raw message kept where the source holds it, otherwise from the payment-gateway database (ARCHITECTURE 5.2) as a
  second source with its own freshness state (the ICS BANKS installation studied held no ISO 20022 and its SWIFT input
  had stopped);
  restore and failover detection that halts watermarks; driver conformance suite; ICS BANKS on Oracle as the first
  adapter pack. Phase 11 (DR-89 to DR-93): closed business days only, landing from a `testing` mapping before any
  publication, freshness classes F1-F5 with `data_as_of` from mapped receipts and a `not_moving` stale state, the
  resume decision after a halt, automatic blind periods, mapping replays over raw, writer conflicts held as
  quarantine rows for phase 12.
- P1 (where agreed): CT3 change data capture through a Debezium Server sidecar that writes a sequenced spool (DR-91);
  CT4 extract files with a contiguous manifest sequence.

**Flows:** connect -> read grants -> refuse on any write privilege -> probe capabilities -> start pollers. Poll ->
keyset batch within budget -> raw rows, receipt, canonical writes and watermark advance in one transaction (DR-89;
mapping runs in the same transaction once a version is published). DBA locks
the account -> "paused by bank" -> back-off -> coverage gap opened -> resume -> automatic backfill.

**API sketch:** `GET/POST /api/v1/sources`, `POST /sources/{id}/probe`, `GET /sources/{id}/grants`,
`PUT /sources/{id}/load-budget` and `PUT /sources/{id}/access` (change requests), `POST /sources/{id}/pause` (the
kill switch), `GET /sources/{id}/batches`, `GET /sources/{id}/watermarks`, `GET /freshness`, `GET /quarantine`, `POST /discovery/runs`, `GET /discovery/profiles`,
`GET /discovery/field-availability`, `GET /discovery/integration-profile` (API.md 3.5).

**Screens:** T1 data connection (mockup 08).

**Edge cases:** the bank grants a write privilege by mistake (refused, incident to DS and SEC); a statement exceeding
the budget (cancelled, back-off); a CBS restored to an earlier point (watermarks halt, alarm, blind period); a source
row deleted in the CBS (monitored copy kept); clock drift between CBS and AKR-TMS (watermarks use source values).

## M6. Mapping & Canonical Model [P0 draft, P1]

**Requirements:** C-10, C-22..C-24, C-36, C-46, C-47, C-50..C-52, SP-48, SP-51, N-12. Phase file: docs/plan/phase-10.md
(DR-85 to DR-88); settled time, late data and replays are phase 11.

**Features by phase**
- P0: draft mapping of the discovery data to the canonical model, enough to grade data readiness; for an Islamic bank
  the readiness grade covers the Islamic financing tables and profit-accrual postings, while M18 stays a pack
  switched on per installation (D22, DR-36).
- P1: mapping specifications as admin-managed documents (DSL.md part B) built from a closed transform library; code
  maps (transaction codes, products, countries, ID types, currencies, statuses), pre-filled from the CBS's own code
  dictionary where one exists, with the list for each column confirmed from data and every pre-filled entry confirmed
  by the bank before use; currency ISO codes are never copied from the CBS; publish by change request; drift detection that pauses only
  the affected tables; bi-temporal canonical entities with lineage; card guard before landing; settled time and late
  data that recompute windows.

**Flows:** DS drafts a mapping -> validate against the schema and sample rows -> preview canonical output -> change
request -> CHK approves -> new version applies from a batch boundary. Drift detected (new column, type change, new
code) -> affected table paused -> DS task -> mapping change -> resume and backfill.

**API sketch:** API.md 3.6 (20 phase 10 operations; the three replay routes are phase 11). The canonical writer runs in
akr-ingestd (DR-85); test and preview runs return counts and shapes, never personal or financial values (DR-86).

**Screens:** T3 mapping workbench.

**Edge cases:** a new transaction code arrives unmapped (gated into quarantine, never monitored as "other"); a code
reclassified (new version effective forward; history keeps the old classification); a party merged in the CBS
(canonical merge with lineage, cases follow the surviving party; a national number is not a unique key, DR-41); a
signer list kept per customer rather than per account is mapped to every account of that customer with `derived`
set and flagged, until the bank or vendor confirms the rule.

## M7. Reconciliation, Coverage & Data Quality [P0 gap report, P1]

**Requirements:** C-25..C-29, C-48, L-01..L-06, S-02, S-03, S-54.

**Features by phase**
- P0 (early piece): the CBI coverage gap report: each of the 173 CBI items against the bank's data, with a
  data-readiness grade A-D per scenario, the missing fields named (a present-but-unpopulated field is graded D with
  the reason, DR-38), the request to the bank written out for the KYC data it must refresh (expected activity, source
  of funds, PEP status), the request to the CBS vendor written out for the other gaps, and an
  interim route with an owner. Built in phase 06 (DR-64 to DR-67): grades per scenario, per source and bank-wide and
  per CBI item (link caps: partial at best B, proxy at best C); not-covered declarations by the MLRO with step-up,
  lapsing and expiring; the report as a sealed JSON document plus four PDF packs, the vendor annex naming fields by
  CBS module without item or scenario; field-mapping candidates for the steward.
- P1: reconciliation ladder (batch receipt, row counts, control totals, GL tie-out) per batch and per day; signed
  daily coverage certificate; coverage ledger and exclusion register (with expiry); quarantine and data-quality queue
  with owners and age; freshness classes F0-F5 with ingest-lag histograms; `data_as_of` everywhere; MLRO coverage
  statement.

**Flows:** end of day -> ladder runs -> breaks open data-quality issues -> certificate signed when all rungs pass or
issued with declared gaps -> visible to M and AS.

**API sketch:** `GET /api/v1/recon/runs`, `GET /recon/certificates/{date}`, `GET /coverage/ledger`,
`GET/POST /coverage/exclusions`, `GET /dq/issues`, `POST /dq/issues/{id}/resolve`, `GET /coverage/cbi-gap-report`.

**Screens:** T2 reconciliation and coverage ledger, T4 data-quality and quarantine queue.

**Edge cases:** GL tie-out off by rounding (tolerance is bank data, change-controlled); a posting group whose legs do
not balance, or that has a single leg, opens a data-quality issue and is never netted off or closed silently; a branch with no postings for
days ("dark unit" flagged, never assumed quiet); a certificate for a day with a blind period (declares it).

## M8. Reference Data & Calendars [P1]

**Requirements:** S-13, T-04, T-10, RP-50 (CBI rate of the day), RP-46, TS-40, UX-22, W-19 and TR-10 (country risk, files 04 and 26), K-24, M-23, M-27, T-44 (code lists and reason codes); with M15: M-14, M-20, M-32, M-42, M-44, M-75, RP-62, GV-44, SP-31, IR-31, IR-56, UX-65, UX-68 (policy pack and authority matrix). Phase file: docs/plan/phase-09.md (DR-78 to DR-84).

**Features by phase**
- P1: country risk table with expiring overrides; CBI FX rates of the day; business calendar per bank, with the working week and weekend read from the CBS calendar
  at discovery and kept per currency where the CBS sets the weekend per currency (DR-39) (Iraqi public
  holidays, Ramadan hours, Eid, Arbaeen, pay days) with Hijri conversion; reference code lists (goAML lookups come
  from M13's profile registry). Phase 09 also captures the CBS holiday and currency-week objects in the discovery run and
  shows the reconciliation on request (DR-79); the working-day business clock pins its calendar version per timer; the
  policy pack (closed registries, forbidden-word list) and the delegated-authority matrix (`cr_authority`, MLRO signs)
  change by change request (DR-81, DR-83).

**API sketch:** API.md 3.8 (21 phase 09 operations with `/policy-packs` and `/org/authority-matrix`).

**Screens:** D3 policy studio (calendars, countries, FX, reason codes, policy pack, authority matrix; built in phase 09), A2 settings.

**Edge cases:** a missing FX rate for a day (CTR job waits and raises a task; never uses a stale rate silently);
Hijri date ambiguity at month start (stored as Gregorian; Hijri is display only).

## M9. Screening & Name Matching [P1]

**Requirements:** N-01..N-59 (owner), M-12, M-24, SP-98, RP-54, UX-44, C-63, L-41, S-32, TS-19, TS-36.
Phase file: docs/plan/phase-14.md (DR-100 to DR-102): one `names` crate with a SQL twin, the gated list lifecycle,
fingerprinted alerts, four-eyes freeze `true`, pair memory and good-guy rules as governed suppression, screens S1-S4;
the freeze workflow behind the `FreezeTrigger` port is phase 20, inline payment screening phase 29.

**Features by phase**
- P1: list ingestion (national list from the Office, UN, OFAC, EU, UK and bank lists; a bank-internal list held in the CBS is
  imported only after bank review, with its source and load date recorded as unknown until the bank confirms them and
  a stale-list alarm when it has no feed) with versions, checksums and a
  daily known-answer harness; parsed name chains and lexicons; one Rust normalisation library with a SQL twin and a
  browser parity test; in-memory index snapshots; scorer with one-to-one alignment, evidence floor and "needs more
  data"; secondary identifiers (mother's chain, birth year with tolerance, documents exact); screening alerts as
  (party, list entry) pairs with their own queue and clocks; Office match terms with CBI aliases and a per-bank doubt
  policy; pair memory for incorrect matches only, invalidated on change; good-guy rules that expire; daily whole-base
  screening; designation look-back on every list change; thresholds per list against an agreed alert budget and a
  recall floor on the evaluation harness.
- P2: the same index serves `akr-assess` inline (M19).

**Flows:** list update -> new version -> index snapshot rebuilt -> look-back screens the whole base for new entries ->
hits queued by priority. Hit -> SS compares (S1) -> incorrect (pair memory offered) / potential (escalate, freezes
nothing) / true (case + M14 freeze task + clocks).

**API sketch:** `GET /api/v1/lists`, `GET /lists/{id}/versions`, `POST /screening/search`, `GET /screening/alerts`,
`POST /screening/alerts/{id}/decision`, `GET/POST /screening/pair-memory`, `GET/POST /screening/good-guys`,
`GET /screening/harness-runs`.

**Screens:** S1 screening review (mockup 03), S2 screening queue, S3 list status and known-answer tests, S4 pair
memory and exemption register.

**Edge cases:** a pair-memory entry now matching a current designation (must be zero; alarm); a family or tribe name
alone (never alerts); a name at the source's byte cap is scored with a truncation flag, never rejected, and a short
name field is not screened when the full name lines exist; a query too thin (returns "needs more data", not "clear"); bulk "incorrect" only where the pair
evidence is identical and never on freeze lists; a list feed that fails (lists stay on the last good version, the
freshness breach is visible, never silent).

## M10. Customer Risk Rating & KYC [P1]

**Requirements:** K-01..K-42 (owner), FL-01..FL-15, TR-01..TR-26, L-10, L-43, GV-37.
Phase file: docs/plan/phase-15.md (DR-103 to DR-106): RM-IQ engine with the normalised inherent score, change-only
history behind one current pointer, suspicion-derived factors withheld (risk reaches the rating only by a compliance
override), downward overrides by change request, the review clocks and EDD sign-off, PEP candidates confirmed by change
request, chain ownership with unknown never clear, `customer` masked and the relationship-conflict check; screens
K1-K4. Gap pass 1 placed the rest (DR-395, DR-396): K-12 phase 19; K-19, K-25, K-34 and K-36
phase 15; K-20 phase 16; K-23 phase 21; K-31 phase 12; K-32 phase 10; K-38 phase 24; K-39 to K-41 are not carried.

**Features by phase**
- P1: rating engine (floors, weighted factor blocks, governed overrides) with five internal levels mapped to the
  CBI's three; 27 starting factors in six blocks; unknown applicable factor scores 60; mandatory unknown caps the best
  level; trigger library re-rating on events; two clocks per customer and a light yearly check for all; overrides up
  easily, down only by compliance, one level, expiring within 12 months; PEP register with tiers (candidates may be seeded from a CBS customer class the bank designates as political, as
  unverified entries of unknown tier, never as a status; a CBS PEP flag is read only if the bank declares it
  maintained, DR-38) and kinship (confirmed by a person; the CBS studied stored no relatives) to the
  second degree; beneficial ownership by chain calculation with a configurable threshold (default 20 percent) on
  ownership percentages the bank enters when the CBS holds none; a missing percentage gives an unknown result under
  the unknown-factor rule, never clear; rating-change feed for the bank
  (one way) and divergence view against the CBS risk field (a CBS value that is unpopulated shows as "not populated" and is never
  read as low or as a divergence); KYC review queue and EDD checklists; exit-candidate
  queue.

**Flows:** canonical change or trigger -> re-rate -> explanation (drivers with refresh dates, missing-input share)
-> if level changes: review task by clock -> KYC reviewer completes -> override if justified (change-controlled
downward) -> feed row for the bank.

**API sketch:** `GET /api/v1/parties/{id}/rating`, `GET /parties/{id}/rating/history`, `POST /parties/{id}/overrides`,
`GET /kyc/reviews`, `POST /kyc/reviews/{id}/complete`, `GET/POST /pep/register`, `GET /parties/{id}/ownership`,
`GET /rating/feed`.

**Screens:** K1 customer risk profile (mockup 04), K2 KYC review queue and workspace, K3 ownership and control, K4 PEP
register and relatives, W4 party page.

**Edge cases:** a downward override proposed by an analyst (refused: compliance only); override expiry while a review
is open (rating returns to the computed level, review continues); missing country (scores high, flagged degraded); an ID expiry that is a sentinel or impossible date is treated as
missing (scores high, flagged degraded), never as valid or expired; a review date that is a sentinel or empty is
unknown, never on time;
an ownership loop in the chain (detected, flagged, never infinite).

## M11. Transaction Monitoring & Scenario Studio [P0 register, P1]

**Requirements:** S-01..S-60 (owner), T-02, T-04, T-07, T-29, L-16, L-17, L-18, TS-14, TS-15.
Phase file: docs/plan/phase-16.md (DR-107 to DR-116): one compiler shared with phase 13, one live version with a
step-up fallback kill switch, the go-live gate, version-free detection fingerprints behind the `AlertSink` port (alerts
are phase 17), replays with reproducible blind samples, seeded known-answer cases, the CBI importer; S-48 and the
outcome measures of S-53 are phase 24.
Phase file: docs/plan/phase-31.md (DR-346): the typology exposure index per month, bank, branch and correspondent
account (matched posted flows and LCY value against totals and balances; restricted scenarios excluded; small numbers
suppressed; flag `pack.exposure`).

**Features by phase**
- P0: the CBI catalogue register (173 items) and the scenario register (161 scenarios, 18 families; 99 in the `cbi`
  pack, 62 in `akr`) with their links, installed from signed pack data by `akr-ctl packs seed`, never deleted (phase
  06, DR-65); `GET /cbi-catalogue` shows the public catalogue without grades.
- P1: scenario documents (DSL.md part A); compiler to parameterised SQL and CEL; incremental, sweep, replay and shadow
  execution; the CBI baseline pack always on; the AKR library extension pack (161 scenarios, 18 families) in replay
  and shadow until agreed with the Office; bank scenarios 901-999; aggregation across linked parties; counterparty
  resolution; calendar-aware windows; known-answer seeded suite daily per CBI item; independent re-implementation
  diff; data-readiness grade per scenario; bootstrap thresholds, replay, above and below-the-line sampling, champion
  and challenger, go-live gate; scenario studio with plain-language text generated from the document; volume-lowering
  changes carry a reason code and evidence; parameter history kept at least seven years.
- P1 (phase 13, DR-98, DR-99): the feature catalogue (nine closed kinds, month-end values gated on freshness and
  coverage), the three-level segment register (static and risk overlay by change request, behavioural clusters from
  deterministic peer builds), robust segment statistics with the 200 floor (S-12), and expected profiles whose
  comparators stay detection logic (S-10, T-07); `/ref/segments`, `/feature-definitions`, `/peer-groups`,
  `/parties/{id}/profiles`.
- P2: typology exposure index across the bank, branches and correspondents (S-31).

**Flows:** batch mapped -> outbox names accounts touched -> event scenarios at once, window scenarios for those
accounts -> settled-time re-evaluation -> alerts (M12). Studio: CFG drafts -> unit tests -> replay on history ->
sample above and below the line -> change request with evidence -> CHK approves -> shadow -> live.

**API sketch:** `GET /api/v1/scenarios`, `POST /scenarios/{id}/versions`, `POST /scenarios/{id}/validate`,
`POST /scenarios/{id}/replays`, `GET /replays/{id}`, `POST /scenarios/{id}/shadow`, `GET /cbi-catalogue`,
`GET /coverage/matrix`.

**Screens:** D1 scenario library and coverage matrix, D2 scenario studio (mockup 05).

**Edge cases:** late data after an alert was decided (alert amended with an explanation, decision kept and flagged
for review); a replay that would scan beyond the cost guard (refused with the estimate); a scenario whose data grade
drops to D (declared "not covered" with owner and interim route, never silently off); two versions live at once
(forbidden: one live, others shadow).

## M12. Alerts, Cases & Investigation [P1]

**Requirements:** M-01..M-31, M-37..M-42, M-46..M-76 (owner; M-32..M-36 in M14, M-43..M-45 in M17), T-21, T-22,
L-17, UX-06, UX-08, UX-13, UX-64, UX-65.

Phase file: docs/plan/phase-17.md (DR-117 to DR-132): alerts from the phase 16 `AlertSink` through `AlertIntakeService`
(identity kept in `scn.detection_keys`, `alert.alert_index`, snapshot through `put_generated`), the never-lost quarantine, one
open case per party and family with merge and per-case hash chains, governed priority classes, queues on the phase 01 pools,
the generated visibility predicate and masked SSE counts; screens W1, W2.
Phase file: docs/plan/phase-18.md (DR-133 to DR-147): the case access list as a visibility clause and the `case_acl`
assignee kind; notes and decision text under the generic-text guard; evidence uploads and opened-view gate; proposals and
decisions through the phase 04 workflows (`case_close_no_report`, `case_report_approval` signed by M), four-eyes CHECK,
authority matrix, contribution inheritance; escalation levels, closure with a hash seal, reopen by a person; work items,
entity links, the shelved-decision register; screens W3, W4, W5, W7. M-36 (exit proposals) is proposed for phase 15.
Phase file: docs/plan/phase-19.md (DR-148 to DR-157): RFIs of six kinds from pack templates, released by the phase 04
workflows `rfi_release_internal` (RFI desk) and `rfi_release_external` (MLRO), answers as claims; internal report and authority
request registers (TR-17 through outbox); UTR watches; hibernations with the intake gate, hit ledger and regret metric;
evidenced network edges with hidden kinship, snapshots and network cases; QA samples with a `qa_review` read grant, rubric,
findings and bulk reopen; clock breach stages and the alarm register; screens W6, W8, W10, W11, G3. M-51 to M-55, M-76 and
T-22 are partly carried or open (phase-19 Open doubts).
RFIs, watches, hibernations, network and QA are phase 19.
Phase file: docs/plan/phase-31.md (DR-338 to DR-344, DR-347): the training workspace in schema `train` (synthetic
cases, scored on the first attempt, screen W12); the capacity workbench by team only (measured handling times, fluid
forecast, sensitivity, `capacity_forecast` alarm, simulation on phase 16 replays, surge QC rate; screen G2; advice
only, D21); the correspondent list and pack attachment on the RFI desk (W8). Not carried: M-50, M-55, the surge
pre-clear, the growth, latency and silence alarms.

**Features by phase**
- P1: alert generation with evidence snapshot, fingerprint and all versions; quarantine for records that cannot form
  an alert; one open case per party and family with merge (never hide); priority classes P0-P3 before score; score
  components; age raises priority; daily low-score sample; case aggregate (statuses as events, property tests);
  four-eyes by Cedar and a database constraint; decisions (no report, UTR, STR, TFR) with coded reasons, the CBI's
  seven escalation criteria and the UTR-versus-STR aid; shelved-decision register; UTR watch (default 90 days);
  immutable notes; case sealing; WIP cap of 10 per analyst; no speed ranking; surge mode; QA sampling with an
  independent reviewer and calibration kit; RFI composer and desk (six kinds, deadline clocks, four-eyes release,
  forbidden-word scan, answers as claims); internal reports and authority requests register; transaction explorer;
  network view and network cases (parent with subgraph snapshot, one child per subject).
- P2: seeded cases and training workspace (W12); AI case summaries (M15 gate).

**Flows:** alert -> merged into the party's open case or a new case -> A1 triages -> A2 investigates -> proposes
outcome -> M decides (four-eyes) -> no report (shelved register) / UTR (watch) / STR or TFR (M13 report builder) ->
case sealed at closure. RFI: compose -> four-eyes release -> deadline clock -> answer stored as claim.

**API sketch:** `GET /api/v1/queues/{id}/items`, `POST /alerts/{id}/claim`, `GET /cases/{id}`,
`POST /cases/{id}/notes`, `POST /cases/{id}/proposals`, `POST /cases/{id}/decisions`, `POST /cases/{id}/merge`,
`GET /cases/{id}/timeline`, `GET /parties/{id}/transactions`, `GET /parties/{id}/graph`, `POST /rfis`,
`POST /rfis/{id}/release`, `GET/POST /authority-requests`, `GET /qa/samples`, `POST /qa/samples/{id}/review`.

**Screens:** W1 my work, W2 queue, W3 case workspace (mockup 02), W4 party page, W5 transaction explorer, W6 network
view, W7 decision dialogs, W8 RFI composer and desk, W10 watches, W11 internal reports and authority requests, G3 QA
workspace, X2 global search, X3 notifications and task inbox.

**Edge cases:** two analysts claim the same alert (one wins; the other gets a conflict); a decision submitted by the
proposer (refused by `forbid` and by the constraint); a P0 item in a bulk close (refused); an alert arriving for a
party whose case is sealed (opens a new case linked to the old one); a watch expiring on a holiday (business
calendar); late data amending an alert inside a decided case (flagged for the decider).

## M13. Reporting to the AML/CFT Office [P0 checker, P1]

**Requirements:** RP-01..RP-84 (owner), R-01..R-61 (regime packs).

**Features by phase**
- P0 (early piece): goAML checker: upload a bank's existing goAML XML or ZIP files (or paste one, with an optional
  Office rejection message); validate against the profile registry (the Office's XSD 5.0.2 with patch P-01 for the
  entity-name facet, plus Iraqi business rules: mother's name, city and state, name character rules, lookup codes);
  an estimated grade in the Office's four levels, always labelled estimated (DR-72); field findings with path, line
  and fix, never a value; files deleted after the run (DR-69); no bank system connection needed. Phase 07 also
  delivers the profile registry as versioned data changed by change request (DR-68), the rule catalogue, the
  regression corpus, a pre-screen plus sandboxed XSD validation (DR-70) and the Office schema watcher through the
  bank's proxy, off by default, with `akr-ctl goaml import-xsd` for air-gapped banks (DR-71).
- P1: report builder for STR, SAR, TFR, UTR, AIF (six steps: reason and indicators, subjects, transactions, narrative,
  validation, package); Iraqi name splitting, mother's name, year-only birth dates, governorate to `state`; six
  validation layers, quality score, consistency checker; packages (XML, attachments, bilingual PDF, SHA-256 manifest);
  filing by a named officer with the reference recorded and portal content captured; rejection parsing into field
  findings; "poor" grade -> returned-for-completion task; FIU feedback register; CTR daily job (per customer per day,
  USD equivalent at the CBI rate, GL tie-out); freeze and match reports and replies to Office requests as official
  letters (templates, numbering, PDF); CBI periodic reports; FATCA, and CRS as an optional module per bank; MLRO
  annual report (no set format: a template the bank edits).
- P2: AI narrative drafting from the evidence snapshot only, with sentence-level sources and recomputed numbers.
- P3: foreign regime packs (US, UK, EU, Gulf): profile, clocks, templates, output adapter.

**Flows:** STR decision -> report builder pre-filled from the case -> validation panel with linked fixes -> M signs
with step-up -> package built and locked -> named officer uploads to goAML -> enters the reference -> AKR-TMS captures
the acknowledgement -> Office grade recorded -> "poor" -> rework task.

**API sketch:** `POST /api/v1/goaml/check` (P0), `GET /goaml/profiles`, `GET/POST /reports`, `GET /reports/{id}`,
`POST /reports/{id}/validate`, `POST /reports/{id}/sign`, `POST /reports/{id}/package`, `POST /reports/{id}/filing`,
`POST /reports/{id}/feedback`, `GET /ctr/batches`, `POST /letters`, `GET /reports/periodic`.

**Screens:** R1 report builder (mockup 07), R2 report list and FIU feedback register, R3 regulatory and periodic
reports.

**Edge cases:** the Office publishes a new XSD (watcher raises a task; new profile version through change request;
reports in flight stay on their profile); a filed STR reusing a UTR's transaction numbers (allowed, owner's answer);
the portal purges a filed report (content captured at filing); a CTR day missing its FX rate (waits, task raised);
a report with an unknown lookup code (blocked at validation: the Office fails the whole file).
Phase file: docs/plan/phase-21.md (DR-169 to DR-183): reports drafted from the running proposal, the MLRO sign of
`case_report_approval` as the decision, validation through the phase 07 engine plus L0, frozen and sealed versions,
packages with a SHA-256 manifest, filing and acknowledgement by the MLRO (the case close gate reads the
acknowledgement), rejections and Office grades with corrections under `report_correction` on resource kind `report`,
the daily CTR run (fixed USD 10,000 threshold, CBI rate of the day, second-person confirmation), letters implementing
phase 20's `LetterRequester` with a backfill under the stub ids, periodic packs with small-number suppression,
analytics, two report clocks; screens R1, R2, R3 (R5 and the liaison review rows are phase 22's). Not carried: RP-23
(phases 27 and 28), RP-33, RP-45, RP-55, RP-58, RP-59 (phase 20), RP-54 (phase 14), RP-71 to RP-75, RP-77, RP-78,
RP-81, R-01 to R-61 (P3), M-27 (phase 18).

## M14. Operations Tasks & Freezes [P1]

**Requirements:** M-32..M-36, C-60..C-64, RP-45, RP-55, RP-56, RP-58, RP-59.

**Features by phase**
- P1: neutral task contract (authority, instruction, clock, completion with CBS reference); verification from the
  canonical copy with levels V0-V3 and the achieved level stored and reported per bank; failure incident to the MLRO;
  freeze board for true matches (freeze task default 8 hours within 24 hours of the listing decision; Committee report
  within 48 hours, copy to the Office; second report at two months; post-order debit alarm, fees included); refrain
  instruction with a 7-working-day clock; release after delisting only on a recorded decision; referral to the
  Committee's technical committee; exits with senior approval and individual assessment.

**Flows:** true match -> freeze task to OPS (neutral text) -> OPS acts in the CBS and enters the reference -> next
batches -> verification reaches V1..V3 -> Committee letter generated (M13) -> two-month timer -> second letter.

**API sketch:** `GET /api/v1/ops-tasks`, `POST /ops-tasks/{id}/complete`, `GET /ops-tasks/{id}/verification`,
`GET /freezes`, `POST /freezes/{id}/release-decision`.

**Screens:** W9 operations tasks and verification, S5 freeze board.

**Edge cases:** a debit after the freeze effective time (alarm, incident, report); the CBS holds hold rows but no order type or
authority, so a hold row shows that a block exists, not that a freeze order was issued (V0: second-person
confirmation, flagged); the freeze clock fires on a released freeze (timer cancelled with the
release event); an OPS user who sees only the neutral instruction (never the reason).
Phase file: docs/plan/phase-20.md (DR-158 to DR-168): neutral operations tasks (three instruction texts, scope-chosen
view, closed field list); verification V0 to V3 from the canonical copy with debits watched until release; freezes created
through phase 14's `FreezeTrigger` with fixed 24-hour, 48-hour and 7-working-day clocks and the governed 8-hour task clock;
release after delisting by the `freeze_release` workflow (M, four-eyes) with a lift task; refrain instructions; failure
incidents and three new phase 19 alarm kinds; letters requested through the `LetterRequester` port (phase 21 implements);
backfill of phase 14 true decisions; screens W9, S5. Not carried: M-35 (phase 19), M-36 (phase 15 exit register), C-61,
C-62 (phases 05 and 11), C-64 (phases 10 and 12).

## M15. Governance, Change Control, Models & AI [P1 change control, P2 models and assistant]

**Requirements:** GV-01..GV-54 (owner), T-13..T-15, T-48..T-51, T-63, TS-16, TS-17, M-70..M-73.

**Features by phase**
- P1: generic change request (maker, checker, reason code, evidence, effective time) used by every governed object;
  inventory record before use (tier, owner, validator, committee); validation pack and per-bank delta validation for
  rules; registers of decisions, exclusions, findings and warnings; fairness register and proxy scan.
- P2: model registry (signed ONNX, cards, validation, drift, label gates, starter models flagged), shadow and
  challenger, kill switch to rules only; scoring of rule alerts; GenAI assistant limited to the enforced list of
  permitted uses, grounding verifier, acceptance by a person, evaluation gate of at least 200 QA-confirmed cases in
  Arabic and English.

**Flows:** any governed change -> change request -> evidence attached (replay, sample, validation) -> checker approves
(never the maker) -> effective at time -> versions recorded on every later alert.

**API sketch:** `GET/POST /api/v1/change-requests`, `POST /change-requests/{id}/approve`, `POST /change-requests/{id}/reject`,
`GET/POST /inventory`, `GET /models`, `POST /models/{id}/shadow`, `POST /models/kill-switch`, `POST /assistant/drafts`.

**Screens:** D3 policy studio, D4 model registry and AI governance, G4 registers.

**Edge cases:** a change request approved after its evidence went stale (approval requires evidence younger than a
bank-set age); the kill switch pressed during scoring (in-flight alerts keep their rule ranking); a model artifact
with a bad signature (refused at load).
Phase file: docs/plan/phase-24.md (DR-218 to DR-231): validation packs as written-once versions with seven parts and
a computed verdict (the validator never the item owner; a `pass` moves the dates through phase 04); the findings and
warnings register (append-and-close, fed by phase 04 post-change reviews, the review calendar and fairness reviews);
decisions and exclusions as read views; the protected-attribute register and allowed-use matrix (never the sole
trigger); monthly fairness metrics with suppression and a review ratio; the derived review calendar; screens G4 and
G5. Not carried: models, scores and the assistant (P2, phases 27 and 28).
Phase file: docs/plan/phase-27.md (DR-257 to DR-273): a model family and each version are inventory items of kind
`model`, and only an `approved_for_use` version scores (shadow included); import in two steps (manifest hash and
Ed25519 signature in the API, artifact checks in the worker; a bank-trained artifact enters the store only by
`akr-ctl model artifact add`); `ort` with a `tract` fallback in the worker only; the score is a separate row in
`alert.model_scores` that ranks inside the priority class and never sets the class, hides, closes or creates an alert;
label gates 50, 200 and 500 from QA-confirmed outcomes over every tier; frozen shadow criteria; validation through the
phase 24 pack; PSI and override monitoring with red points as phase 24 findings (source `model_monitor`, five closure
codes); kill switch immediate by an MRO, release by change request `cr_material`; pins, one live family; TreeSHAP
reason codes (three shown). The kill switch is pressed by an MRO only (scope `model.risk`), not by a family owner who
is not an MRO (Open doubts 1). Not carried: GV-02, GV-06..08, GV-10..13, GV-17, GV-23, GV-26..28, GV-31, GV-33,
GV-42..53, T-48..51, M-70..73, TS-17 (the assistant, phase 28, and other phases).
Phase file: docs/plan/phase-28.md (DR-274 to DR-291): an optional pack (flag `pack.assistant`, default off, 404 on
every route) with two tasks only, case summary and report narrative, both drafts; the llama.cpp server with one GGUF
model on premises in zone `z2_model` without egress, called by the worker only, and a second server `llama-eval` for
verification and evaluation; generation is a job, the browser polls; figures, dates and names are placeholders filled by
code and the `GroundingVerifier` removes any sentence it cannot match; phase 09 tipping-off scan; nothing is saved
without one person accepting one draft, recorded in `casework.decisions.ai_involvement`; golden evaluation on
QA-confirmed closed cases (200, 50 per language) with a human factual review; go-live of the model and template set
by change request `cr_material_validated`; monitoring counts suppressed; QA sample of assisted decisions never below
10 per cent; no agent, tools or function calling. The kill switch is engaged by one MRO and released by two people
through `cr_material`; research 28 asks for two people on both, so the bank decides (Open doubts 8). Not carried:
disposition recommendation (needs GV-42), the other candidate tasks of research 21, the GV-51 security tests and exit
plan (procurement records), model selection and GPU hardware.

## M16. Command Centre, Registers & Evidence Packs [P0 gap report, P1, P2]

**Requirements:** S-02, S-56, S-57, M-62, M-63, RP-63, RP-64, GV-35, UX-49, UX-50, UX-60.

**Features by phase**
- P0: CBI coverage gap report rendered as PDF packs (from M7) for the bank's remediation file: the report and the
  vendor request in Arabic and English, plus the integration profile PDF; `POST /packs` limited to these three kinds
  until phase 24; audited download with step-up (phase 06, DR-67).
- P1: command centre (mockup 01: what is late, decisions waiting, clocks, arrivals against throughput, verification
  failures, QA, data incidents, tipping-off summary); assessor workspace mapped to the D2 tests (random sample,
  end-to-end trace, verifier, packs); evidence packs per report and per case; board pack builder; training register;
  liaison report review (with M17).
- P2: capacity workbench; correspondent pack (CBDDQ evidence) and validator pack; typology exposure index views.

**API sketch:** `GET /api/v1/dashboard/command-centre`, `POST /packs` (kind: board, case, report, assessor,
correspondent), `GET /packs/{id}`, `GET /assessor/d2-matrix`, `POST /assessor/samples`, `GET /training/register`.

**Screens:** G1 command centre (mockup 01), G2 capacity workbench (P2), G8 assessor workspace, G9 training register,
R4 board pack and evidence pack builder, R5 liaison report review.

**Edge cases:** a board reader opening case content (refused: aggregates only); an assessor sample including a
suspicion-tier case (allowed by the time-boxed grant, every view logged); a pack regenerated after data changed (new
version; the old one stays with its hash).
Phase file: docs/plan/phase-24.md (DR-218 to DR-231): the live command centre (counts through each list's predicate,
1 to 4 suppressed, the tipping-off tile for M only), the read-only clocks board, the assessor workspace (role `AS`
with `valid_to`, the D2 map as a signed data file, seeded recorded samples, a metadata-only trace that reaches a
suspicion alert only through the assessor's own sample), pack kinds `board`, `case`, `report`, `assessor` and
`certification` on phase 06's builder (download step-up `export`, uniform verify answer), the training register
(recorded by ADM, never for oneself); screens G1, G8, G9, R4. Not carried: the capacity workbench, correspondent and
validator packs, the typology exposure index (P2), redaction and the requester watermark (Open doubt 11).
Phase file: docs/plan/phase-31.md (DR-339, DR-345, DR-346, DR-355): correspondent and validator packs from system
facts (flag `pack.correspondent`); the typology exposure panel on G1 (not a tile); the practice column on the G9
register (display only, never a record or a count); the capacity forecast route of 3.16.

## M17. Branch Window & Liaison [P1]

**Requirements:** M-43..M-45, S-22, S-23, UX-02, UX-66, L-47.

**Features by phase**
- P1: separate light bundle (about 300 KB) for liaison officers; branch home with neutral statuses; branch review
  form (explanation as a claim, documents on file, visit result); observation form with the CBI's eight indicator
  groups; confidential report straight to the MLRO that branch management cannot see; monthly liaison report
  auto-drafted, signed with step-up by the 10th; branch transactions and documents check.

**Flows:** observation -> MLRO queue -> hybrid alert when it matches a pattern. Month end -> draft report -> liaison
signs by the 10th -> MLRO reviews (R5).

**API sketch:** `GET /api/v1/branch/items`, `POST /branch/items/{id}/input`, `POST /branch/observations`,
`POST /branch/confidential-reports`, `GET /branch/monthly-report`, `POST /branch/monthly-report/sign`.

**Screens:** B1 branch home, B2 branch review, B3 observation form, B4 confidential report, B5 monthly liaison report,
B6 branch transactions and documents check.

**Edge cases:** an item becomes suspicion tier (disappears from the branch view without explanation, by design);
the branch manager asks for the liaison's reports (no route exists); the 10th falls on a holiday (business calendar).
Phase file: docs/plan/phase-22.md (DR-201 to DR-217): the branch window reuses phase 19 (a branch answer is an RFI
claim from source `branch_officer`, never self-corroborated; confidential reports and observations are internal
reports with channel `branch_window`), so no `branch_inputs` or `confidential_reports` table; an item is listed only to
the branch of the liaison it is addressed to; a closed three-word neutral status map; branch routes answer 404 without
the scope; the observation log with eight closed groups, the seven criteria, an optional customer and the code
acknowledgment; the S-23 hybrid alert on an existing scenario alert (off until a scenario is named); the monthly report
drafted with suppressed counts, signed with step-up `signing` (one register entry per period), reviewed or returned by
M on R5, due on the 10th or the next working day with the `liaison_report_overdue` alarm (M-53). The same phase ships
the notification half of X3 (M1 `/me` rows): one dispatcher on the outbox (phase 00's registry becomes a handler list
per topic), recipients computed at dispatch through the screens' visibility, fifteen closed templates changed by change
request with versions, e-mail through the bank relay only (off by default, digest default), notices swept after
`notify.retention_days`; `notification.new` on phase 17's `CountsHub`. Carries M-43, M-44 (branch side), M-45, L-47,
UX-02, UX-66, UX-67, S-22, S-23, M-53, M-66, M-68; B6 content and the checklist inside every case are open doubts.

## M18. Islamic Banking Pack [P1, optional]

**Requirements:** I-01..I-30 (owner), S-37.

**Features by phase**
- P1 (switched on per bank): Islamic product taxonomy; deal-centric model (deal, parties by role, asset, parallel
  contracts, evidence status); due diligence and screening for suppliers, sellers, brokers, developers and partners;
  hard rules (third-party seller, prohibited structures, qard hasan caps, real-estate two-year rule); possession
  evidence tracker; pooled-fund freeze by account class (phase 26 DR-252: no Finance or Sharia board task, since neither
  role exists); Sharia board aggregate
  dashboards only; book flag for Islamic windows (the contract type is derived from the product or ledger code or from the agreement
  tables through the product code map, and that map is tested, since the CBS may keep no contract type on the
  account) and an alert on transfers between books; 8 library scenarios with
  basis labels.
- P3: pack updates.

**Screens:** additions to W3, W4 and G1; a Sharia board aggregate view.

**Edge cases:** an Islamic contract scoring higher only because it is Islamic (forbidden by design; test in the
fairness suite); a pooled-fund freeze (the customer's pooled investment account gets an ordinary hold task, the pool and
the other holders are never in the instruction).
Phase file: docs/plan/phase-26.md (DR-246 to DR-256): deals and deal parties in `canon` (migration 0083, which also
widens the product contract-type list by four values), analyst-written possession evidence in `kyc` with the view
`kyc.deal_evidence_status` (0084); six routes (API.md 3.18); the deal read follows the phase 18 party rule
(`88_islamic.cedar`); missing evidence is a counts-only phase 24 register warning from job `islamic_evidence_sweep`, not
an alert (DR-248); no Islamic factor in any score (DR-249); library SCN-ISL-01..08 in shadow, live through G9
(DR-251); the Sharia board is not a user and reads counts through M, CH and IA (`dashboard.read`, DR-250). The Sharia
view's screen id is `I1` in the DESIGN-SYSTEM catalogue (gap pass 1). Not carried by phase 26: I-09,
I-11..I-15, I-18, I-19, I-21..I-23, I-25, I-27..I-29 (phase 26 header; gap pass 1).

## M19. Assessment API (phase 2) [P2]

**Requirements:** C-55..C-59, C-65, T-43, N-50, SP-93, GV-53.

**Features by phase**
- P2: synchronous assess endpoint over mTLS; signed decisions with byte-exact replay; local list index and
  pre-computed features, never the CBS; fail policy per rail, corridor, currency and amount (bank data); shadow and
  advisory modes per rail; decisions and reason codes, never scores; holds go to an analyst queue; deterministic
  bank-configured rejects only; release-without-decision control; local fallback screen beside the gateway; blind
  periods recorded.

**API sketch:** `POST /assess/v1/payments` (mTLS, idempotent), `GET /assess/v1/decisions/{id}`,
`GET/PUT /api/v1/assess/fail-policies` (change request), `GET /assess/holds`.

**Screens:** additions to S2 (hold queue), D3 (fail policies), G1 (assess health).

**Edge cases:** OpenBao sealed after a node restart (auto-unseal; keys cached); a replayed request with a different
body (refused, idempotency); the gateway times out and releases (release-without-decision control raises an item).
Phase file: docs/plan/phase-29.md (DR-292 to DR-316): an optional pack (flag `pack.assess`, default off, 404 on every
route); gateways bound by a pinned certificate and a service principal; decisions signed in process with an Ed25519
seed wrapped under `akr-kek`, persisted with their response bytes before the answer, replayed byte for byte from the
service's own ledger, and valid for ten minutes; MT103, pacs.008 and canonical input; the phase 14 index and freshness
rule inline, no model and no pair memory inline; freeze-list hits hold (PEP and internal by setting); modes shadow,
advisory and enforcing per rail with governed entry criteria; fail policies with bands, modes and prohibited rules
(closed grammar) as `cr_material` change targets with no emergency change; holds decided by two people, list holds as
phase 14 alerts and the freeze through phases 14 and 20; the release-without-decision control on the canonical copy,
including releases after a pass expired; outage periods, weekly control tests and a signed freeze-list extract for the
bank's own fallback (no matcher of this product beside the gateway). Left open: domestic payments on stale lists
(C-57 reading), raw RTGS and ACH formats, and pair memory inline (phase 29 Open doubts 4, 6 and 12).

## M20. Admin, Settings, Releases & Ops Cockpit [P0 minimal, grows every phase]

**Requirements:** TS-01..TS-43 (platform, owner), SP-03, SP-64..SP-66, SP-83..SP-88.

**Features by phase**
- P0: workspace and compose stack; settings and feature flags; jobs queue with dead-job retry; ops cockpit v1 (queue
  depth, dead jobs, backups, anchors, disk, certificates, OpenBao seal state); OTel and `/metrics`.
- P1: release bundle import and verification (signature, SBOM diff, release notes, rollback plan); support session
  opened by the bank (time-boxed, `support-readonly`, redacted bundles); blind-period register; platform health;
  help centre with bilingual articles and "what changed".
- P2: K3s and Helm profile; automated PostgreSQL failover.

**API sketch:** `GET/PUT /api/v1/admin/settings`, `GET/PUT /admin/flags`, `GET /admin/ops`, `POST /admin/jobs/{id}/retry`,
`POST /admin/releases/verify`, `POST /admin/support-sessions`, `GET /admin/blind-periods`.

**Screens:** A2 settings, A3 platform health, A4 release and updates, X5 help centre.

**Edge cases:** a bundle with a bad signature (refused before any change); a setting below a CBI minimum (refused:
floors); vendor support asking for case data (no route).
Phase file: docs/plan/phase-25.md (DR-236 to DR-240, DR-244): release bundles imported and verified by the phase 00
`bundle verify` code path, approved by change request `platform_release` on `cr_access`, applied by the operator and
recorded (no call-home); vendor support sessions (at most 8 h, SEC approval with step-up `access_change`, the `VS`
grant can only read its own profile, automatic end); support bundles built by `akr-ctl support-bundle` (allow-list,
per-bundle pseudonym key, signed, deleted after 30 days, disclosure record kept); help articles and "what changed"
shipped in the signed bundle; ops tiles for SIEM lag and dead letters, release currency, open support sessions and
active break-glass grants; screens A4 and X5.
Phase file: docs/plan/phase-30.md (DR-317 to DR-337; migration 0095, no table, no route): profile M/L on three K3s
servers from one catalogue that also renders the profile S compose bundle; zones as NetworkPolicies rendered from
`deploy/zones.yaml` with a cluster zone check; PostgreSQL failover with strict synchronous commit (Patroni with
pgBackRest provisional, proof of concept decides); OpenBao automatic unseal; job harness lease heartbeat and
cluster-wide concurrency; database outages recorded as phase 11 blind periods; quarterly failover drill, `akr-ctl
restore verify`, RTO/RPO per profile; signed charts and Kyverno admission; eight A3 tiles. The read-only DR workspace
is not carried (Open doubts 31).

---

## 21. Frontend Architecture

### 21.1 App shell

Lapis navigation rail on the start side (right in Arabic) with the gold active marker; the rail shows only what the
role may open, generated from the machine-readable screen catalogue (UX-01, UX-53); header with global search and
command palette (X2), notifications (X3), language and profile (X4). Clay background, white data cards. The branch
window is a separate bundle with its own minimal shell.

### 21.2 Route map

`/work` (W1), `/queues/:id` (W2), `/cases/:id` (W3), `/parties/:id` (W4, with tabs for transactions W5, network W6,
risk K1, ownership K3), `/screening` (S1, S2), `/screening/lists` (S3), `/screening/memory` (S4), `/freezes` (S5),
`/kyc` (K2), `/pep` (K4), `/scenarios` (D1), `/scenarios/:id/studio` (D2), `/policy` (D3), `/models` (D4),
`/reports` (R1, R2), `/reports/periodic` (R3), `/packs` (R4), `/liaison` (R5), `/data/connection` (T1),
`/data/recon` (T2), `/data/mapping` (T3), `/data/quality` (T4), `/command` (G1), `/capacity` (G2), `/qa` (G3),
`/registers` (G4), `/access` (G5), `/audit` (G6), `/retention` (G7), `/assessor` (G8), `/training` (G9),
`/ops-tasks` (W9), `/watches` (W10), `/requests` (W11), `/rfi` (W8), `/admin/*` (A1-A4), `/help` (X5), `/me` (X4),
`/inbox` (X3), `/changes` (G10).
Branch bundle: `/branch` (B1-B6). Early pieces: `/goaml-check` (E1) and `/discovery` (E2, with the CBI coverage gap report) run in a reduced shell; screen IDs and layout patterns in DESIGN-SYSTEM.md section 8.1.

### 21.3 State rules

Server state only through TanStack Query hooks generated by Orval; no hand-written API types; URL holds filters and
selection; SSE events invalidate queries (typed event enum, ignored events listed); no client-side caching of
suspicion-tier data beyond the page.

### 21.4 Schema-driven editors

Mapping, scenario, policy and change-request editors render from the JSON Schemas served by the API (DBS dynamic form
pattern, react-hook-form + Zod), so a new field in DSL.md needs no new screen code.

### 21.5 RTL and i18n rules

Every string in Arabic and English (i18next); logical CSS only (`ms-*`, `me-*`, `ps-*`, `text-start`); directional
icons flip; Western digits in data and tables; numbers, dates, IBANs, phone numbers and amounts isolated with `<bdi>`;
ICU plurals with all six Arabic forms; `YYYY-MM-DD HH:MM`; Hijri shown beside Gregorian where the bank wants it; no
`toLocaleString`, no `input type=date`; fonts vendored (IBM Plex Sans Arabic, Plex Sans, Plex Mono), glyph coverage
tested.

### 21.6 Feedback and failure

A calm failure message with the correlation id; no error reveals a masked resource (X6); `data_as_of` on every data
screen; empty states explain why (no data, not covered, not permitted is never distinguishable for suspicion tier).

### 21.7 Theming

Light theme by default; dark theme from the same tokens; WCAG 2.2 AA in both (DESIGN-SYSTEM.md).

## 22. API Conventions (cross-cutting)

DBS conventions, detailed in [API.md](API.md): `/api/v1` prefix; RFC 7807 problem+json with stable codes;
Idempotency-Key on every mutation with byte-exact replay; If-Match with ETags on every update; cursor pagination;
SSE for live updates; `data_as_of` in every data response; existence masking returns the same 404 body as a missing
resource; the assessment API lives under `/assess/v1` on its own binary.

## 23. End-to-End Flows (definitive walkthroughs)

1. **Discovery for a new bank (P0).** DS connects through CT2 -> grants checked -> profiling -> field availability
   -> data-readiness grades -> CBI coverage gap report pack -> bank adds it to its remediation file.
2. **goAML checker (P0).** M uploads last quarter's XML files -> validation and grades -> findings with fixes -> the
   bank corrects its process before the full system arrives.
3. **Cash structuring to an STR (P1).** Batch -> CBI cash scenario fires for a customer across linked accounts ->
   alert with evidence snapshot -> merged into the open case -> A1 triage -> A2 investigation (transactions, network,
   RFI to the branch) -> proposal STR -> M decides -> report builder -> validation -> M signs -> package -> officer
   uploads -> reference recorded -> grade "good".
4. **List update to a verified freeze (P1).** National list version -> look-back finds a customer -> SS true match ->
   freeze task (8-hour default) -> OPS acts -> verification V2 -> Committee letter within 48 hours -> two-month timer
   -> second letter.
5. **Threshold change (P1).** CFG proposes a lower volume -> replay shows fewer alerts -> below-the-line sample shows
   no productive alerts lost -> reason code "risk" -> change request -> CHK approves -> shadow 30 days -> live.
6. **Re-rating by trigger (P1).** A customer becomes a PEP relative -> TR fires -> level rises -> EDD review task ->
   KYC completes -> rating feed row -> bank updates the CBS risk field; divergence view clears.
7. **UTR then STR (P1).** M decides UTR -> watch 90 days -> new alert during the watch -> STR, linked to the UTR.
8. **Branch observation (P1).** Teller sees an indicator -> liaison records it (B3) -> hybrid alert with a
   structuring pattern -> case.
9. **CTR day (P1).** End of day -> per-customer daily cash -> USD equivalent at the CBI rate -> batch tied to the cash
   GL -> package -> filed.
10. **CBS restored to an earlier point (P1).** Watermarks halt -> alarm -> blind period opened -> DS resolves with the
    bank -> backfill -> alerts produced, never back-dated -> blind period in the coverage statement.
11. **Assessor D2 test (P1).** AS draws a random sample -> end-to-end trace from transaction to alert to case to
    report -> verifier checks hashes -> evidence pack.
12. **Pre-release hold (P2).** Gateway calls assess -> list hit above threshold -> hold -> SS decides incorrect ->
    release signal -> decision signed and kept.

## 24. Module x Phase Matrix

| Module | P0 | P1 | P2 | P3 |
|---|---|---|---|---|
| M1 Identity | Core | OIDC upstream | Polis (SAML, SCIM) | |
| M2 Organisation | Core | Authority matrix, leave, access review | | |
| M3 Authorisation | Cedar core, masking | Tiers, case ACLs, break-glass | | |
| M4 Audit, vault, retention | Audit + anchors + vault | Viewer, SIEM, retention engine | | |
| M5 Sources | Discovery profiling | Production ingestion, first adapter pack | More adapter packs | More adapter packs |
| M6 Mapping | Draft | Full | | |
| M7 Reconciliation | CBI gap report | Ladder, certificates, DQ | | |
| M8 Reference | | Full | | |
| M9 Screening | | Full | Inline for assess | |
| M10 Rating and KYC | | Full | | |
| M11 Monitoring | CBI register | Engine, packs, studio | Exposure index | Regime scenarios |
| M12 Cases | | Full | Training workspace | |
| M13 Reporting | goAML checker | Full | AI drafting | Regime packs |
| M14 Operations tasks | | Full | | |
| M15 Governance and AI | | Change control, inventory | Models, assistant | |
| M16 Command centre and packs | Gap report pack | Command centre, assessor, board | Capacity, correspondent, validator packs | |
| M17 Branch window | | Full | | |
| M18 Islamic pack | | Optional | | Updates |
| M19 Assessment API | | | Full | |
| M20 Admin and ops | Minimal | Releases, support sessions | K3s profile | Kurdish interface |

## 25. Decision Queue

Open decisions carried from research file 33 section 5. None blocks phase 0; each has a working default that becomes
configuration once decided.

| # | Decision | Who | Working default |
|---|---|---|---|
| Q1 | Office open points: code-table meanings, report-date element, rejection messages and purge period, TFR after a true match | The Office (pilot letter) | Report-date element as in the XSD; TFR offered as an option after a true match |
| Q2 | The 62 extension scenarios, including public funds | Each MLRO with the Office | Replay and shadow until agreed |
| Q3 | Legal screen of suppliers and components | Iraqi counsel | Barred parties excluded; SBOM with maintainer and country |
| Q4 | CBS data readiness per bank | Each bank and CBS vendor (P0) | Grades per scenario, declared gaps with owners |
| Q5 | Seven-year floor outside the assessment | CBI, counsel | Seven years in the Iraq profile |
| Q6 | UTR adoption and the watch | Each bank | UTR on, 90-day watch |
| Q7 | Doubt policy for screening | Each MLRO, ideally the Office | Chosen at installation; potential matches resolved within the freeze clock |
| Q8 | Vendor-operated hosting | CBI | One installation per bank |
| Q9 | National list feed | The Office | Excel export, reconciled by ID |
| Q10 | Phase 2 adoption per rail | Each bank | Off; shadow then advisory first |
| Q11 | Licences (AGPL reading, OpenSanctions commercial licence, GPL libraries) | Counsel | Notices + SBOM; unclear terms excluded |
| Q12 | Third-party risk data (World-Check and others) | Each bank | Pilot on Iraqi samples before purchase |
| Q13 | GenAI features per bank | Each bank after the evaluation gate | Off; drafts only |

Decided and recorded: the design (locked 2026-10-07 with the screen contrast tokens), the early pieces (both, in P0),
the Office questions the owner answered (research file 23 s.9).
