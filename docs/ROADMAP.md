# Build Roadmap

Status: **planning draft 2026-10-07**. Expands ARCHITECTURE.md section 22 into workstreams, task checklists, exit
criteria, cross-phase invariants, risks and external dependencies. The implementation phases (phase-00 onwards) that
carry these tasks are in [plan/roadmap.md](plan/roadmap.md).

Macro-phases follow the leadership plan (plan section 11): **P0** discovery and foundations, **P1** core system for the
first bank, **P2** checks before payments are released and the smarter features, **P3** scale to further banks.
Durations are the plan's indicative figures for the first bank; they depend most on how fast the bank approves
read-only database access. **Order and exit criteria are the commitments, not dates.** Tasks inside a phase run in
parallel where their dependencies allow.

Two clocks run from the start:

- **The reform window.** Banks that filed their CBI self-assessment by 30 September 2026 have about six months, to
  about the end of March 2027, to fix gaps. The two early pieces (the CBI coverage gap report and the goAML checker)
  must reach banks inside that window, so they are built first inside P0 (owner decision 2026-10-07; research file 33
  section 5 item 15).
- **The first bank.** Live in about nine months, ending with a two-to-three-month parallel run.

Requirement IDs in brackets are research IDs (research/01-33). Module numbers are MODULES.md. Screen IDs are
DESIGN-SYSTEM.md section 8.1.

## Phase P0 — Discovery and foundations

Goal: a deployable, observable, secured platform skeleton with identity, authorisation and the audit write path; the
read-only discovery tooling; and the two early pieces in banks' hands inside the reform window. Discovery at a bank
takes six to eight weeks once access is approved.

P0 runs as two tracks. **Track A** is vendor-side build and starts at once. **Track B** is per bank and starts when a
bank signs: it needs Track A's discovery tooling and nothing else.

### A1. Platform and infrastructure (Track A)
- [ ] Profile S compose bundle: one server plus a backup server, Caddy edge, bootstrap, release and rollback scripts
      (tech-stack.md); network zones Z0-Z7 default deny, `akr-ingestd` alone in the CBS-facing zone (ARCHITECTURE.md)
- [ ] PostgreSQL 17 primary and standby; ICU collation fixed at `initdb` and verified for Arabic; schemas and roles of
      DATA-MODEL.md; pgBackRest WAL archiving every minute plus restic for buckets and OpenBao snapshots, **with an
      offsite target**
- [ ] OpenBao: Shamir unseal ceremony documented and rehearsed; transit KEK; DEK families; Ed25519 keys for anchors,
      packages and decisions; parity with the DBS Vault client calls proven (tech-stack open item); never in the
      request hot path
- [ ] Object store: MinIO built from the pinned Silo release behind the S3 guard; COMPLIANCE object lock buckets for
      audit anchors and evidence; WORM and security regression suites run on every build; SeaweedFS exit tested under
      the same suites (TS-41 to TS-43)
- [ ] Valkey (cache, rate limits, SSE fan-out only)
- [ ] Gitea and Actions runners, Harbor with Trivy and content trust; cosign with the vendor key in KMS; CycloneDX
      SBOMs; `cargo-auditable`; the signed OCI-layout release bundle imported offline into a test "bank" Harbor
- [ ] Observability: Prometheus and Alertmanager, OpenTelemetry Collector and Jaeger, JSON logs with a syslog or CEF
      forwarder for the bank's SIEM; `/metrics`, `/healthz`, `/readyz` on every binary
- [ ] Dev environment: docker compose mirroring the stack; `make` targets
- [ ] Pin every version (cargo workspace, images, frontend packages); third-party notices file; SBOM with maintainer
      and country for every component, for the legal screen (VO-07, TS-37)

### A2. Application skeleton (Track A)
- [ ] Cargo workspace: four binaries `akr-api`, `akr-worker`, `akr-ingestd`, `akr-assess` (stub) over shared crates;
      Axum with utoipa and Swagger; problem+json errors; idempotency with byte-exact replay; `If-Match`; cursor
      pagination; `data_as_of` plumbing (API.md section 1)
- [ ] Orval pipeline proven end to end on `/healthz` and `/me`
- [ ] Postgres `jobs` with `FOR UPDATE SKIP LOCKED`, transactional outbox and timers (the DBS harness); dead-job
      handling; ops cockpit stub
- [ ] Settings store (admin-managed data, never constants), with CBI minima as floors
- [ ] P0 migrations (DATA-MODEL.md migration order, P0 set); `.sqlx` offline data in CI; UUIDv7 generated app-side
- [ ] CI: fmt, clippy `-D warnings`, tests on throwaway PostgreSQL, `sqlx prepare --check`, cargo deny and audit,
      image build, Trivy gate, SBOM diff, deploy to staging; protected branches and a second approver on releases
- [ ] Port the DBS rules into `.claude/rules/` (implementation-flow, admin-managed, ddd, jobs, migrations,
      rust-backend, api-design, auth, frontend, testing, docker, file-edits), adapted as planning.md was

### A3. Identity, authorisation and audit (Track A; M1-M4 core)
- [ ] Kratos and Hydra; custom Arabic and English auth UI (X1): mandatory MFA, WebAuthn first, TOTP minimum, paste
      allowed in code fields; forced first-login change; recovery by admin reset; Hydra as OIDC provider for Gitea
      and Grafana
- [ ] BFF sessions in `akr-api` with CSRF bound to the session; step-up AAL2 as a single-use ledger row
      (`X-StepUp-Assertion`); idle time-out with the warning and extend action (UX-35)
- [ ] Organisation, users, roles (MODULES.md role codes), one-click offboarding; quarterly access review stub
- [ ] Cedar embedded, deny by default: two gates (scope, then policy); first policy set (self, admin, settings);
      decision logging; existence masking returning the same 404 as a missing resource; segregation-of-duties
      `forbid` pairs
- [ ] Audit write path: partitioned audit tables with no UPDATE or DELETE grants, per-partition hash chain, anchor
      worker signing to the WORM bucket; separate from operational logs; evidence vault write path
- [ ] Every request traced and every mutation audited from the first endpoint
- [ ] Change control and the DBS workflow engine (DSL.md Parts D and E): role grants and settings change by change
      request from the first administrative action; the bootstrap administrator grant is a recorded CLI command (DR-15)

### A4. Discovery and the CBI coverage gap report (Track A builds, Track B runs; early piece 1)
- [ ] `akr-ingestd` connection layer: `SourceReader` port; drivers for PostgreSQL, SQL Server, Oracle (with the ODBC
      second backend), Db2 and Sybase over ODBC; connection tiers CT1-CT4 including the bank-delivered extract
- [ ] **Read-only conformance suite**: grants checked on every connect and the connection refused on any write
      privilege; every statement parsed with `sqlparser` and proven a single `SELECT` over approved objects; the
      signed load budget enforced (bounded pool, statement time-out, back-off); "paused by bank" state (C-01, C-13,
      C-15, SP-87)
- [ ] Discovery profiling (M5): schema inventory, row counts, field availability, value profiles, code-table capture;
      `GET /discovery/profiles`, `GET /discovery/field-availability`
- [ ] Draft mapping (M6) for the first CBS family (ICS BANKS on Oracle, to be confirmed with the first bank)
- [ ] CBI catalogue register: the 173 items with their data needs (M11)
- [ ] Data-readiness grades per scenario and the CBI coverage gap report (M7; S-02, S-03, C-48): each item against
      the bank's data, the gap, an interim route and an owner, the vendor requests written out
- [ ] Gap report PDF pack through Gotenberg (M16), signed and hashed, for the bank's remediation file
- [ ] Screen E2 in the reduced shell

### A5. The goAML checker (Track A; early piece 2)
- [ ] goAML profile registry holding the Office's published XSD (never hard-coded) and the Office's answered rules
      (research file 23 section 9)
- [ ] Validation layers: schema, Office business rules, code tables, Iraqi field rules (mother's name, entity-name
      pattern), cross-field rules; quality grade against the Office's grading criteria (RP-27, RP-29)
- [ ] Regression corpus of valid and invalid reports; rejection-message parser stub for when the Office's list
      arrives (RP-42, RP-84)
- [ ] Screen E1 in the reduced shell: upload or paste, faults linked to fields with bilingual explanations, printable
      result; nothing filed from here
- [ ] Runs standalone at a bank without a CBS connection (no bank data needed to install it)

### A6. Frontend foundation (Track A)
- [ ] Design tokens: `tokens.json` → `akr.css`, contrast check in CI, Derived tokens confirmed by the owner (DS-O2)
- [ ] Untitled UI React vendored and RTL-migrated; one Tailwind entry; lint rules of DESIGN-SYSTEM.md section 6.4
- [ ] App shell and reduced shell; i18n kit (direction, bidi, datetime, fold with Kurdish and Persian additions,
      plurals, digits, AriaLocale); fonts with `latin-ext` and the glyph-coverage test (UX-26)
- [ ] Shared components needed by P0: StatusBadge, values, step-up dialog, session warning, forms renderer, table,
      empty and not-found states (X6); admin screens A1-A3 minimal
- [ ] Screen catalogue `screens.ts` with the route-to-policy coverage test (UX-53)

### A7. Spikes and early decisions (Track A)
- [ ] RTL spike: chart time-axis direction, Recharts or in-house SVG only, Arabic leading values (DS-O3, DS-O7)
- [ ] Graph spike: `@xyflow/react` ego graph at 100 nodes within 2 s with grouping (IR-82)
- [ ] Name-matching core: the normalisation library and its SQL twin with the golden file, started early because
      screening and the goAML checker's name rules both need it
- [ ] Usability round 1 with Iraqi compliance staff on paper and clickable prototypes, Arabic first (UX-03); range
      order question (DS-O4)

### B. Discovery at each bank (Track B, six to eight weeks)
- [ ] Inventory of the core banking and payment systems; CBS vendor and version; read-only access approved at a
      connection tier; signed load budget; named DBA contact
- [ ] Discovery run, data-quality baseline, data-readiness grades, CBI coverage gap report delivered
- [ ] Alignment with the bank's own risk assessment and policy (thresholds, segments, doubt policy Q7, UTR adoption
      Q6, retention floor Q5)
- [ ] Hardware sizing from discovery data; second site confirmed; branch link quality measured (DS-O6)
- [ ] goAML checker installed for the MLRO's team

**Exit criteria**
- The goAML checker validates against the Office's XSD and rules with its regression corpus green, and is installed
  at at least one bank inside the reform window.
- A bank can run discovery read-only and receive a signed CBI coverage gap report with grades, gaps, interim routes
  and owners.
- Login with MFA on staging through the pipeline; every request traced and audited; the audit chain verifies and
  anchors land in the WORM bucket.
- The read-only conformance suite passes for every driver family against fixtures and refuses a write-granted
  account.
- A release bundle is built, signed, carried offline and imported into a test bank's Harbor.
- A restore drill from the offsite backup into staging has passed and is recorded.

## Phase P1 — Core system, first bank

Goal: the first bank runs AKR-TMS for screening, risk rating, monitoring, cases and reporting, and goes live after a
two-to-three-month parallel run in which every alert is compared with the bank's current system. About nine months
from the start of discovery.

### Data (M5-M8)
- [ ] Production ingestion: watermarks, keyset batches within the load budget, raw landing as the replay log,
      freshness classes F0-F5 with `data_as_of` on every response, CBS restore and failover detection that halts
      watermarks and records blind periods; first adapter pack (C-29)
- [ ] Mapping complete (DSL.md Part B): closed transform library, card guard, survivorship, code maps with the
      quarantine gate for transaction types, drift bulkheads, lifecycle through change requests
- [ ] Reconciliation ladder, coverage certificates, dark units, data-quality and quarantine queue (M7)
- [ ] Reference data and calendars: currencies, countries, code tables, business calendars per bank, seasonal
      calendar (M8)

### Screening (M9)
- [ ] Normalisation library and SQL twin in parity; immutable in-memory index; Rust scorer with the mother's name and
      chain-aware matching (N-26, N-29)
- [ ] Lists: UN, national (Excel stop-gap reconciled by ID until a bulk feed exists, N-45), internal; list
      reconciliation; full re-screen within one hour of a list update
- [ ] Daily known-answer harness in Arabic, Kurdish and Latin (S3); pair memory with fingerprints and expiry (S4)
- [ ] Screening queue and review (S1, S2) with the Office's three match states; freeze board with the freeze clock
      and the 48-hour Committee report (S5)

### Risk rating and KYC (M10)
- [ ] Rating model RM-IQ (DSL.md Part C): factors, unknown scores 60, peak term, floors, completeness cap, triggers,
      overrides down one level with maker-checker
- [ ] KYC review queue and workspace, EDD checklists, ownership and control with the 20 percent test, PEP register
      with relatives to the second degree, evidence vault viewer (K1-K4)

### Monitoring (M11)
- [ ] Scenario engine (DSL.md Part A): aggregate logic compiled to parameterised SQL, event logic in CEL, profile
      logic over features with peer fallback marked degraded; incremental, sweep, replay and shadow modes
- [ ] CBI baseline pack (never retired) and the Iraq library; extension scenarios in replay and shadow until agreed
      with the Office (Q2)
- [ ] Scenario library and coverage matrix (D1), scenario studio with test on history and below-the-line sample
      (D2), policy studio (D3)
- [ ] Feature tables, segments and segment statistics

### Alerts, cases and investigation (M12)
- [ ] Alerts with evidence snapshot, fingerprint and every producing version; one open case per party and family
      with merge; priority classes by rule before score (M-03, M-07, M-08)
- [ ] Case workspace (W3) with the decision panel: actions separate from decisions, coded reasons, generic-text check,
      seven-criterion checklist, evidence-opened gate, four-eyes by Cedar `forbid` and a database constraint
- [ ] Queues with saved views and queue-run mode (W1, W2); party page, transaction explorer and network view
      (W4-W6); RFIs to branches (W8); watches and hibernations (W10); internal reports and authority requests (W11)

### Reporting (M13)
- [ ] goAML package generation for UTR, STR and TFR from the case, validated by the P0 checker engine; filing record
      with the goAML reference and acknowledgement; FIU feedback register (R1, R2)
- [ ] CTR batches aggregated per day, monthly movements, transfers register, correspondent activity, scheduling and
      watermarks (R3); board pack and evidence pack builder (R4); liaison report review (R5)

### Acting without writing (M14)
- [ ] Operations tasks with neutral text codes, clocks and verification levels V0-V3; failure incidents; the minimal
      operations inbox (W9)

### Governance, command centre, branch window (M15-M17)
- [ ] Change control for every governed object (DSL.md Part D) with the shipped workflow definitions (Part E);
      emergency route with review within five working days; model inventory
- [ ] Command centre (G1), QA workspace (G3), registers (G4), access and insider controls (G5), audit viewer and chain
      verifier (G6), retention and legal holds (G7), assessor workspace (G8), training register (G9)
- [ ] Branch window as its own bundle (B1-B6): neutral statuses, branch review form, observation form, confidential
      report, monthly liaison report signed by the 10th

### Platform growth (M1-M4, M18, M20)
- [ ] OIDC upstream (M1); authority matrix, leave and access review (M2); sensitivity tiers, case access lists and
      break-glass (M3); audit viewer, SIEM forwarding and the retention engine with classes R1-R13 (M4)
- [ ] Islamic banking pack where the bank needs it (M18)
- [ ] Release and update screen with bundle verification and rollback (A4); vendor support sessions (M20)

### Quality gates
- [ ] Accessibility pack: axe on every route in both languages, keyboard walkthroughs of the six core tasks, 400
      percent zoom in Arabic (UX-32, UX-39)
- [ ] Response budgets under 50 concurrent analysts on a synthetic data set (UX-16)
- [ ] Tipping-off inference tests in CI: counts, search, exports and notifications reveal nothing about masked
      records (SP-26 to SP-36)
- [ ] Collation and fold parity between browser and server (UX-28)
- [ ] Load test on the bank's target hardware

### Parallel run (two to three months)
- [ ] AKR-TMS runs beside the bank's current system on live data; every alert compared both ways; every
      disagreement explained in a log reviewed weekly with the MLRO
- [ ] First goAML filings made from AKR-TMS by the bank's named officer and acknowledged by the Office
- [ ] Go-live decision signed by the MLRO

**Exit criteria** (from the plan's "what success looks like")
- 100 percent of in-scope CBS tables reconciled every day, every gap explained.
- At most one hour from a list update to a full re-screen.
- Every CBI catalogue item either running or declared with its grade, interim route and owner.
- No alert older than 30 days without a recorded reason; no overdue high-risk review at go-live.
- A goAML filing made from the case and acknowledged by the Office.
- A dry run of the assessor's D2 trace passes end to end (CBS row to alert, case, decision, report, evidence pack).
- Response budgets and WCAG 2.2 AA checks pass; the read-only conformance suite passes against the bank's CBS.
- A restore drill on the bank's servers has passed and is recorded.
- The parallel-run disagreement log is closed and the MLRO has signed the go-live.

## Phase P2 — Before release, and smarter

Goal: checks before payments leave, models that earn their place, and the assistant for drafting, each switched on
per bank and per rail. About six months after P1.

### Assessment API (M19)
- [ ] `akr-assess` in the gateway zone; mTLS client certificates per gateway; `/assess/v1` returning pass, hold or
      reject (never a score), signed with Ed25519, expiring (API.md section 4)
- [ ] Inline screening on the shared index under 25 ms at p99; p95 150 ms and p99 400 ms end to end (C-56, N-50)
- [ ] Fail policies per rail with modes shadow, advisory and enforcing; a reject without a person only on conditions
      the bank set as prohibited (C-57, GV-53)
- [ ] Profile M/L: K3s with three nodes, automated PostgreSQL failover (Patroni with pgBackRest or CloudNativePG,
      chosen by proof of concept)

### Models and the assistant (M15, M13)
- [ ] Model registry and AI governance (D4): inventory, tiers, cards, validation by the model-risk officer,
      approval through `cr_material_validated`, drift, kill switch
- [ ] Models served as ONNX inside the bank; shadow first; ranking rule alerts only, never suppressing one (T-13,
      T-14, T-15)
- [ ] Assistant for case summaries and report narratives: drafts only, sentence-level sources, evaluation gate in
      Arabic, switched on per bank (T-48, M-70, M-73)

### Further features
- [ ] Training workspace with synthetic cases (W12); capacity workbench (G2); correspondent and validator packs;
      typology exposure index (M11, M16)
- [ ] RFIs to correspondents (W8)
- [ ] SAML and SCIM through Ory Polis where a bank needs them (M1)

**Exit criteria**
- Each enabled rail has run in shadow and then advisory mode with its agreement statistics reviewed by the MLRO
  before enforcing.
- Assessment SLOs met under load on profile M; a failover drill passed.
- Every live model validated independently and approved through the governed change route.
- The assistant passes its evaluation gate in Arabic with Iraqi analysts (UX-46).

## Phase P3 — Scale

Goal: further banks in three to five months each, more regimes, and the Kurdish interface.

- [ ] Adapter packs for further CBS families; mapping a new bank in weeks, not months
- [ ] Regime packs (US, UK, EU) with their scenarios and report packs (M11, M13; D17)
- [ ] Islamic pack updates (M18)
- [ ] Kurdish (Sorani) interface (D16)
- [ ] Support model for many single-tenant installations: release cadence, per-bank policy packs, pinned versions
      with advance notice, version-drift report (C-38, M-75, TS-30, GV-29)

**Exit criteria**
- A second bank live within five months of its discovery start.
- Every installation within one release of the current version, or a recorded reason.

## Cross-phase invariants

- **Read-only toward the CBS, provably.** Any change touching `akr-ingestd` runs the read-only conformance suite.
- **Every endpoint** annotated, registered in OpenAPI and mapped to a Cedar action; the Orval client regenerated in
  the same change as the API.
- **Every mutation** carries an Idempotency-Key, `If-Match` where it updates, and an audit event.
- **Every schema change** is a new idempotent migration with `TIMESTAMPTZ` and app-side UUIDv7 ids, then
  `cargo check` and `sqlx prepare`.
- **Every phase** names the research requirement IDs it carries and passes a tipping-off review
  (.claude/rules/planning.md).
- **Never tuned to staff capacity; unknown is never low.** No configuration caps alerts.
- **Every data response** carries `data_as_of`.
- **Design**: tokens only through `akr.css`; the contrast check and the interaction-rule checklist on every screen
  change (DESIGN-SYSTEM.md).
- **Library documentation** is looked up before implementing against a crate or package.
- No emojis anywhere. Commits appear human-authored; never Claude authorship or co-authorship.
- Monthly restore drill from P0 onward, recorded in the audit trail.

## Risk register

From research file 33 section 6, plus the build risks the DBS recorded.

| # | Risk | Impact | Mitigation | Phase |
|---|---|---|---|---|
| 1 | CBS data gaps (only 47 of 173 CBI items run on ledger fields; mother's name, teller events, depositor IDs often missing) | Degraded scenarios, weaker matching, goAML files failing the mother's-name rule | Readiness grades, vendor requests written out, branch observation log, gaps shown before the MLRO's deadline (S-03, S-19, S-22, N-56, RP-26) | P0-P1 |
| 2 | Read access refused, slowed or withdrawn | Late start; load complaints close the connection | Four connection tiers including a bank-delivered extract, signed load budget, "paused by bank", recorded blind periods (C-01, C-13, C-15, SP-87) | P0 |
| 3 | Regulatory change and conflicting texts | Clocks and schemas change under the build | Profile watcher, catalogue importer, clock engine with recorded bases, open-questions register (RP-03, S-06, RP-46, RP-09) | All |
| 4 | goAML rejections (unpublished rules, entity-name pattern defect, no test environment) | Rejected filings | Six validation layers, compatibility layer, regression corpus, rejection parsing, feedback register (RP-04, RP-06, RP-27, RP-42, RP-84) | P0-P1 |
| 5 | Tipping-off and insider leaks | A customer learns of a case | Suspicion tier, existence masking, conflict checks, canary records, inference tests, behaviour rules on AKR-TMS's own audit (SP-26 to SP-36, M-67) | P0-P1 |
| 6 | Object-store supply chain (Silo by a small team, no SLA) | Unpatched or abandoned storage layer | Pinned build from source, second source, S3 guard, WORM conformance suite, advisory watch, tested exits (TS-41 to TS-43) | P0, yearly |
| 7 | Alert volume above staff capacity | Pressure to cap alerts or raise thresholds | No caps; ranking, grouping, surge mode, capacity workbench, analyst hours shown before activation, staged scenario activation (M-52, M-54, M-55, N-54) | P1-P2 |
| 8 | Screening misses or floods on common Iraqi names | Missed designations or swamped queues | Chain-aware matching with the mother's name, alert budget with a recall floor, "needs more data", list reconciliation, known-answer harness (N-26, N-29, N-32, N-45, L-32) | P1 |
| 9 | Too few labels for machine learning | Models arrive late | Rules first, label gates, weak labels, validated synthetic data, leave-one-bank-out evaluation, shadow mode (T-15, T-57, T-58, T-62) | P2 |
| 10 | Legal and sanctions exposure of suppliers | A barred component or feed | Legal screen, SBOM with maintainer and country, open components on premises (VO-07, TS-37) | P0 |
| 11 | Unproven typologies ([N] scenarios) | Hypotheses presented as proven | Evidence labels on every scenario; validation on the first bank's cases (S-59) | P1 |
| 12 | Evidence weight in court (Law 78 of 2012) | Records challenged | Case sealing, hash chains, chain-of-custody logs, per-bank statement for counsel (M-61, M-63, M-64) | P1 |
| 13 | Many single-tenant installations | Support cost, version drift | One product, adapter packs, policy packs, signed bundles, pinned versions (C-38, M-75, TS-30, GV-29) | P3 |
| 14 | Timing against the reform window | Window closes before a first bank is live | The two early pieces first in P0 | P0 |
| 15 | Accessibility and Arabic rendering | Contrast failures, unverified Arabic screen readers, missing Latin glyphs | Locked contrast tokens, glyph-coverage test, Arabic accessibility testing (UX-26, UX-33, UX-39) | P0-P1 |
| 16 | Small team (bus factor) | Operations and delivery exposure | Everything runbooked in the repository; break-glass on paper; second approver on releases | All |
| 17 | Scope creep before P1 exit | First bank never goes live | Exit criteria are the gate; new ideas land in P2 or P3 of this file | P1 |
| 18 | SQLx compile-time coupling during migrations | Broken deploys | Expand-contract migrations; migrate-then-deploy; `.sqlx` offline data in CI | All |

## External dependencies (blockers by phase)

| Item | Who | Blocks | Status |
|---|---|---|---|
| Legal screen of suppliers and components (Q3) | Iraqi counsel | P0 version pinning; first delivery | Open |
| AGPL reading for the Silo build and optional Grafana family; licences (Q11) | Counsel | First delivery to a bank | Open |
| Office open points: code-table meanings, report-date element, rejection messages and purge period, TFR after a true match (Q1) | The Office, through the pilot letter | P1 filing rules (working defaults in place) | Open; 11 of 12 answered 2026-10-07 |
| National list feed in bulk or by API key (Q9) | The Office | Better P1 screening (Excel stop-gap works) | Open |
| Read-only access approval and signed load budget | First bank | P0 Track B, then P1 | Not started |
| CBS vendor field deliveries (mother's name, teller events, depositor IDs) | Each bank and its CBS vendor | P1 scenario and goAML coverage | Per bank |
| Hardware, second site, change-board slots | First bank | P1 install | Per bank |
| Third-party risk data (Q12) and OpenSanctions commercial licence | Each bank; vendor | P1 PEP and adverse-media depth | Open |
| Doubt policy (Q7), UTR adoption (Q6), retention floor (Q5) | Each bank | P1 configuration (defaults in place) | Per bank |
| Dark palette approval (DS-O1) | Owner | Dark theme only (light ships regardless) | Open |
| Phase 2 adoption per rail (Q10); GenAI features (Q13) | Each bank | P2 switch-on | Per bank |
| Vendor-operated hosting (Q8) | CBI | Nothing in the default design (one installation per bank) | Open |

Decision numbers Q1-Q13 are MODULES.md section 25.
