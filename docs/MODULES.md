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

## M2. Organisation, Users & Roles [P0]

**Requirements:** SP role and segregation rows (file 30 s.7.2), UX-53, M-20 (delegated-authority matrix), M-43
(liaison officers).

**Features by phase**
- P0: branches and regions imported from the CBS reference tables (seed, then admin-managed), analyst pools, users
  mirror, role grants with a validity window and vetting status, delegations with windows, the liaison officer and
  alternate per branch.
- P1: delegated-authority matrix (who may decide what, up to which class) signed by the MLRO; leave ledger that
  re-routes work; quarterly access review.

**Flows:** grant a role -> change request (maker ADM, checker SEC) -> effective at time -> audited; leave -> open work
re-routed by policy -> returned on return if unclaimed.

**API sketch:** `GET/POST /api/v1/org/branches`, `GET/POST /org/pools`, `GET/POST /admin/users`,
`POST /admin/users/{id}/grants`, `POST /org/delegations`, `GET/PUT /org/authority-matrix`.

**Screens:** A1 users, roles and leave.

**Edge cases:** a user holding both maker and checker roles (allowed, but never on the same change: M3 `forbid`);
liaison officer on leave (alternate receives branch items); branch merged or closed in the CBS (mapping drift ->
admin decision, history kept).

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
(403) or mask (404) -> decision audited. Break-glass: request with reason -> two approvers (SEC + M) -> time-boxed
grant -> every action flagged -> automatic expiry -> review.

**API sketch:** `GET/PUT /api/v1/admin/policies`, `POST /admin/policies/simulate`, `POST /access/break-glass`,
`POST /access/break-glass/{id}/approve`, `GET /records/{kind}/{id}/access-history`.

**Screens:** X6 not found / no access, G5 access and insider controls.

**Edge cases:** a customer enters the suspicion tier while an analyst has the page open (next fetch masks; live
update never reveals why); counts on queues must not reveal masked items; an export started before a tier change
(re-checked at download); policy bundle with a syntax error (refused, previous bundle stays).

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

## M5. Sources, Discovery & Ingestion [P0 discovery, P1 production]

**Requirements:** C-01..C-24, C-35, C-36, C-38, C-39, C-41..C-44, C-54, C-61, C-62, C-64, TS-04, TS-07..TS-09.

**Features by phase**
- P0 (discovery, early piece): connect to a bank's CBS copy through one of CT1-CT4; grant check on connect (fail
  closed); schema catalogue and data profiling (row counts, null rates, value distributions, character-set round
  trip for Arabic and Kurdish, name data-quality profile); field availability against the canonical attributes that
  scenarios need (feeds M7 and M11 for the data-readiness grades and the CBI coverage gap report).
- P1: production ingestion: `SourceReader` adapters (PostgreSQL, SQL Server, Oracle with an ODBC second backend,
  ODBC for Db2 and Sybase); keyset batches on watermarks; signed load budget and kill switch; "paused by bank" state;
  raw landing with lineage hashes and batch receipts; payment messages (ISO 20022 and MT) with the raw message kept;
  restore and failover detection that halts watermarks; driver conformance suite; ICS BANKS on Oracle as the first
  adapter pack.
- P1 (where agreed): CT3 change data capture through a Debezium Server sidecar.

**Flows:** connect -> read grants -> refuse on any write privilege -> probe capabilities -> start pollers. Poll ->
keyset batch within budget -> raw rows + receipt (same transaction enqueues mapping) -> watermark advance. DBA locks
the account -> "paused by bank" -> back-off -> coverage gap opened -> resume -> automatic backfill.

**API sketch:** `GET/POST /api/v1/sources`, `POST /sources/{id}/probe`, `GET /sources/{id}/grants`,
`PUT /sources/{id}/load-budget` (change request), `POST /sources/{id}/kill-switch`, `GET /sources/{id}/batches`,
`GET /discovery/profiles`, `GET /discovery/field-availability`.

**Screens:** T1 data connection (mockup 08).

**Edge cases:** the bank grants a write privilege by mistake (refused, incident to DS and SEC); a statement exceeding
the budget (cancelled, back-off); a CBS restored to an earlier point (watermarks halt, alarm, blind period); a source
row deleted in the CBS (monitored copy kept); clock drift between CBS and AKR-TMS (watermarks use source values).

## M6. Mapping & Canonical Model [P0 draft, P1]

**Requirements:** C-10, C-22..C-24, C-36, C-46, C-47, C-50..C-52, SP-48, SP-51, N-12.

**Features by phase**
- P0: draft mapping of the discovery data to the canonical model, enough to grade data readiness.
- P1: mapping specifications as admin-managed documents (DSL.md part B) built from a closed transform library; code
  maps (transaction codes, products, countries, ID types); publish by change request; drift detection that pauses only
  the affected tables; bi-temporal canonical entities with lineage; card guard before landing; settled time and late
  data that recompute windows.

**Flows:** DS drafts a mapping -> validate against the schema and sample rows -> preview canonical output -> change
request -> CHK approves -> new version applies from a batch boundary. Drift detected (new column, type change, new
code) -> affected table paused -> DS task -> mapping change -> resume and backfill.

**API sketch:** `GET/POST /api/v1/mappings`, `POST /mappings/{id}/versions`, `POST /mappings/{id}/validate`,
`POST /mappings/{id}/preview`, `GET/POST /code-maps`, `GET /drift-events`.

**Screens:** T3 mapping workbench.

**Edge cases:** a new transaction code arrives unmapped (gated into quarantine, never monitored as "other"); a code
reclassified (new version effective forward; history keeps the old classification); a party merged in the CBS
(canonical merge with lineage, cases follow the surviving party).

## M7. Reconciliation, Coverage & Data Quality [P0 gap report, P1]

**Requirements:** C-25..C-29, C-48, L-01..L-06, S-02, S-03, S-54.

**Features by phase**
- P0 (early piece): the CBI coverage gap report: each of the 173 CBI items against the bank's data, with a
  data-readiness grade A-D per scenario, the missing fields named, the request to the CBS vendor written out, and an
  interim route with an owner.
- P1: reconciliation ladder (batch receipt, row counts, control totals, GL tie-out) per batch and per day; signed
  daily coverage certificate; coverage ledger and exclusion register (with expiry); quarantine and data-quality queue
  with owners and age; freshness classes F0-F5 with ingest-lag histograms; `data_as_of` everywhere; MLRO coverage
  statement.

**Flows:** end of day -> ladder runs -> breaks open data-quality issues -> certificate signed when all rungs pass or
issued with declared gaps -> visible to M and AS.

**API sketch:** `GET /api/v1/recon/runs`, `GET /recon/certificates/{date}`, `GET /coverage/ledger`,
`GET/POST /coverage/exclusions`, `GET /dq/issues`, `POST /dq/issues/{id}/resolve`, `GET /coverage/cbi-gap-report`.

**Screens:** T2 reconciliation and coverage ledger, T4 data-quality and quarantine queue.

**Edge cases:** GL tie-out off by rounding (tolerance is bank data, change-controlled); a branch with no postings for
days ("dark unit" flagged, never assumed quiet); a certificate for a day with a blind period (declares it).

## M8. Reference Data & Calendars [P1]

**Requirements:** S-13, T-04, RP-50 (CBI rate of the day), the country-risk and code-list rows of files 04 and 26.

**Features by phase**
- P1: country risk table with expiring overrides; CBI FX rates of the day; business calendar per bank (Iraqi public
  holidays, Ramadan hours, Eid, Arbaeen, pay days) with Hijri conversion; reference code lists (goAML lookups come
  from M13's profile registry).

**API sketch:** `GET/POST /api/v1/ref/countries`, `GET/POST /ref/fx-rates`, `GET/POST /ref/calendars/{id}/days`.

**Screens:** D3 policy studio (calendars), A2 settings.

**Edge cases:** a missing FX rate for a day (CTR job waits and raises a task; never uses a stale rate silently);
Hijri date ambiguity at month start (stored as Gregorian; Hijri is display only).

## M9. Screening & Name Matching [P1]

**Requirements:** N-01..N-59 (owner), M-12, M-24, SP-98, RP-54, UX-44, C-63, L-41, S-32, TS-19, TS-36.

**Features by phase**
- P1: list ingestion (national list from the Office, UN, OFAC, EU, UK and bank lists) with versions, checksums and a
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
alone (never alerts); a query too thin (returns "needs more data", not "clear"); bulk "incorrect" only where the pair
evidence is identical and never on freeze lists; a list feed that fails (lists stay on the last good version, the
freshness breach is visible, never silent).

## M10. Customer Risk Rating & KYC [P1]

**Requirements:** K-01..K-42 (owner), FL-01..FL-15, TR-01..TR-26, L-10, L-43, GV-37.

**Features by phase**
- P1: rating engine (floors, weighted factor blocks, governed overrides) with five internal levels mapped to the
  CBI's three; 27 starting factors in six blocks; unknown applicable factor scores 60; mandatory unknown caps the best
  level; trigger library re-rating on events; two clocks per customer and a light yearly check for all; overrides up
  easily, down only by compliance, one level, expiring within 12 months; PEP register with tiers and kinship to the
  second degree; beneficial ownership by chain calculation with the 20 percent test; rating-change feed for the bank
  (one way) and divergence view against the CBS risk field; KYC review queue and EDD checklists; exit-candidate
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
is open (rating returns to the computed level, review continues); missing country (scores high, flagged degraded);
an ownership loop in the chain (detected, flagged, never infinite).

## M11. Transaction Monitoring & Scenario Studio [P0 register, P1]

**Requirements:** S-01..S-60 (owner), T-02, T-04, T-29, L-16, L-17, L-18, TS-14, TS-15.

**Features by phase**
- P0: the CBI catalogue register (173 items) used by the gap report.
- P1: scenario documents (DSL.md part A); compiler to parameterised SQL and CEL; incremental, sweep, replay and shadow
  execution; the CBI baseline pack always on; the AKR library extension pack (161 scenarios, 18 families) in replay
  and shadow until agreed with the Office; bank scenarios 901-999; aggregation across linked parties; counterparty
  resolution; calendar-aware windows; known-answer seeded suite daily per CBI item; independent re-implementation
  diff; data-readiness grade per scenario; bootstrap thresholds, replay, above and below-the-line sampling, champion
  and challenger, go-live gate; scenario studio with plain-language text generated from the document; volume-lowering
  changes carry a reason code and evidence; parameter history kept at least seven years.
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
- P0 (early piece): goAML checker: upload a bank's existing goAML XML files; validate against the profile registry
  (the Office's XSD 5.0.2 plus Iraqi business rules: mother's name, city and state, name character rules, lookup
  codes); grade with the Office's four grades; list field findings with fixes; no bank system connection needed.
- P1: profile registry with a daily watcher on the Office's attachment list; report builder for STR, SAR, TFR, UTR,
  AIF (six steps: reason and indicators, subjects, transactions, narrative, validation, package); Iraqi name splitting,
  mother's name, year-only birth dates, governorate to `state`; six validation layers, quality score, consistency
  checker; packages (XML, attachments, bilingual PDF, SHA-256 manifest); filing by a named officer with the reference
  recorded and portal content captured; rejection parsing into field findings; "poor" grade -> returned-for-completion
  task; FIU feedback register; CTR daily job (per customer per day, USD equivalent at the CBI rate, GL tie-out); freeze
  and match reports and replies to Office requests as official letters (templates, numbering, PDF); CBI periodic
  reports; FATCA, and CRS as an optional module per bank; MLRO annual report (no set format: a template the bank
  edits).
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

**Edge cases:** a debit after the freeze effective time (alarm, incident, report); the CBS does not expose hold status
(V0: second-person confirmation, flagged); the freeze clock fires on a released freeze (timer cancelled with the
release event); an OPS user who sees only the neutral instruction (never the reason).

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

## M16. Command Centre, Registers & Evidence Packs [P0 gap report, P1, P2]

**Requirements:** S-02, S-56, S-57, M-62, M-63, RP-63, RP-64, GV-35, UX-49, UX-50, UX-60.

**Features by phase**
- P0: CBI coverage gap report rendered as a PDF pack (from M7) for the bank's remediation file.
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

## M18. Islamic Banking Pack [P1, optional]

**Requirements:** I-01..I-30 (owner), S-37.

**Features by phase**
- P1 (switched on per bank): Islamic product taxonomy; deal-centric model (deal, parties by role, asset, parallel
  contracts, evidence status); due diligence and screening for suppliers, sellers, brokers, developers and partners;
  hard rules (third-party seller, prohibited structures, qard hasan caps, real-estate two-year rule); possession
  evidence tracker; freeze workflow involving Finance and the Sharia board for pooled funds; Sharia board aggregate
  dashboards only; book flag for Islamic windows and an alert on transfers between books; 8 library scenarios with
  basis labels.
- P3: pack updates.

**Screens:** additions to W3, W4 and G1; a Sharia board aggregate view.

**Edge cases:** an Islamic contract scoring higher only because it is Islamic (forbidden by design; test in the
fairness suite); a pooled-fund freeze (Finance and Sharia board tasks added).

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

## M20. Admin, Settings, Releases & Ops Cockpit [P0 minimal, grows every phase]

**Requirements:** TS-01..TS-43 (platform, owner), SP-03, SP-64..SP-66, SP-83, SP-85, SP-87.

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
`/ops-tasks` (W9), `/watches` (W10), `/requests` (W11), `/rfi` (W8), `/admin/*` (A1-A4), `/help` (X5).
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
