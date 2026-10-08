# 33. Synthesis: design principles, decisions, conflicts and open questions

Status: done (2026-10-07)

Scope: this file closes the research. The lead wrote it from files 01 to 32 and `_minio-notes.md`, without new web research and without agents. It adds **no requirement IDs**; it points to the 1,208 requirements in the other files (unique PREFIX-NN across all files, excluding FM, FL, TR, G and IR, which are failure modes, rating floors, rating triggers, stack gaps and interaction rules). It is the bridge between the research and the build: the principles every module follows, the decisions taken per module with the requirements that carry them, the conflicts found between files and how each was resolved, the decisions that remain open and who must take them, and the main risks. The plan (`plan/plan.md`, Arabic edition `plan/plan-ar.md`) tells the same story for bank leadership; where the two differ in detail, this file and the research files it cites govern.

## Contents

1. What the research found: twelve findings that shape the product
2. Design principles
3. Decisions, module by module
4. Conflicts between files and how they were resolved
5. Open decisions
6. Risks
7. How the research feeds the plan
8. Sources

## 1. What the research found: twelve findings that shape the product

1. **The regulator has written most of the specification.** The CBI's AML/CFT controls of 2025 contain 86 numbered scenarios and 87 product and behaviour indicators (173 items). The banking reform's Detailed Assessment Guide (D2) tells the CBI-approved auditor to test rule logic, alert thresholds, user permissions, audit-trail integrity, the links between KYC, monitoring and screening, and whether the data behind each report can be traced; it also asks for seven years of AML data. The AML/CFT Office publishes its goAML schema and its reporting guides. An Iraqi bank is judged against these texts, not against vendor feature lists (files 01, 18, 23, 27, 28).
2. **The reform makes the AML system a test of survival.** Every private bank that chose to continue had to file an external AML assessment by 30 September 2026, with six months to fix the gaps and a merger or liquidation path for banks that cannot. Iraq is back on the FATF grey list, and dollar access now depends on each bank's own controls (files 01, 02, 18; plan section 2).
3. **Iraqi banks detect very little today.** The 2024 mutual evaluation counted 628 money-laundering STRs from 74 banks over five years, about 1.7 per bank per year, and falling. Some audited annual reports still describe the CBI minimum as 25 scenarios, and no bank interviewed had built its own (files 01, 18). Labels for machine learning are therefore scarce: rules, data completeness and explanations come first, models later (files 21, 29).
4. **The CBI floor needs more than the ledger.** Only 47 of the 173 CBI items (27 percent) run from ledger, account and KYC fields. 80 need a CBS module or payment-message data, 26 need data from outside the CBS, and 20 are observational, so only a person at the counter can supply them. A data-readiness grade per scenario is therefore a core feature, and the branch observation log is part of monitoring (file 27 B22; file 25).
5. **The CBI catalogue misses the threat ranked first.** Corruption and the embezzlement of public funds lead Iraq's national risk assessment, yet no CBI scenario addresses them. File 27 proposes a public-funds and payroll family (SCN-PUB-01 to -07). Like every scenario beyond the CBI floor, it is to be added in coordination with the AML/CFT Office, as the CBI controls require (files 02, 27).
6. **Arabic names need Iraqi identity data, not better fuzzy matching.** An Iraqi identity is a name chain plus the mother's name plus a birth year. At a 2 percent alert budget, recall on noisy variants of listed persons rose from 0.44 (name only) to 0.84 (with birth year and mother's name) in a prototype on public data. The Office's national-list web service is unreliable when paged: 6,280 rows walked gave only 4,467 distinct IDs (file 22).
7. **The AML/CFT Office changed reporting in 2026.** Its unusual transaction report (UTR, first edition 2026) is filed only through goAML, needs no suspicion, and adds a third outcome beside an STR and no report, followed by a watch that can end in an STR. The Office publishes its goAML XSD 5.0.2 with Iraqi tightenings (mother's name required on every person, city and state required in every address, unknown lookup codes fail the whole file) and one defect (the entity-name pattern). It grades every STR (good, moderate, acceptable, poor) and returns poor ones. Nobody submits to goAML by machine: a named person uploads (files 23, 28).
8. **Freeze duties run on clocks and outside goAML.** Terrorist Funds Freezing Regulation No. 6 of 2023 defines "without delay" as within 24 hours of the listing decision (the Office's March 2023 manual says 8 hours from publication). The first report to the Committee for Freezing Terrorists' Funds (copy to the Office) is due within 48 hours of the freeze, the second within two months, and the channel is not published. Because AKR-TMS never writes to the CBS, a freeze is a task for bank operations that AKR-TMS verifies from its copy of the data (files 23, 25, 28, 30).
9. **Read-only is achievable and provable, at a price.** Reading from a replica or bank-owned views gives post-event monitoring, needs a signed load budget, and must treat a DBA closing the account as a normal "paused by bank" state. Checking payments before release needs a separate, optional phase-2 API called by the bank's payment gateway (file 25).
10. **Suspicion data need their own protection, on the bank's own servers.** Tipping-off is a crime under Law 39 of 2015 (articles 12 and 39), so suspicion-tier records need case-level access lists and existence masking. The CBI's classification rule keeps confidential data on the institution's own assets, so release 1 is one installation per bank, not a hosted multi-tenant service (file 30).
11. **No supplier documents the whole Iraqi combination.** Of the 55 suppliers reviewed, global suites bring strong engines and regional suppliers bring local presence (Pio-Tech reports 29 banks in Iraq; Azentio and Capital Banking AML run at Iraqi banks on their own cores). None documents the CBI pack with data-readiness grades, chain-aware Arabic matching with the mother's name, a tested Iraqi goAML profile and read-only integration together. Israeli-headquartered vendors are blocked pending Iraqi counsel (files 07 to 18).
12. **The stack follows the DBS, with three changes.** Valkey replaces Redis and OpenBao replaces Vault (licences and the legal screen). MinIO stays at the owner's decision, as a pinned, maintained and renamed build (Silo, with Chainguard's fork as a second source), because the upstream repository is archived and seven 2026 advisories are fixed only in the proprietary AIStor and in maintained forks. No workflow engine is needed: a Rust case aggregate plus the DBS workflow DSL (files 28, 32; `_minio-notes.md`).

## 2. Design principles

Principles 1 to 10 are the plan's (section 6), in the same order; each row adds what the principle means for the build and the requirements that carry it. Principles 11 to 15 are build rules that the plan states in other sections.

| # | Principle | What it means in the build | Key requirements |
|---|---|---|---|
| 1 | **Iraq first** | Iraqi law, the CBI catalogue, Iraqi lists, Arabic names and dinar amounts are the base. The Iraq jurisdiction profile is always on; foreign rules come as switchable regime packs (profile, clocks, templates, output adapter) | S-01, RP-01, RP-71, SP-53, K-37, R-24 |
| 2 | **Read-only, and provably so** | No writes, triggers, jobs or stored code in the CBS. Grants are checked on every connect and the connection fails closed on any write privilege. A signed per-bank load budget applies. The CBS is never queried on demand by customer or case. Actions become tasks for bank staff | C-02, C-03, C-04, C-13, C-15, C-35, C-54, M-32 |
| 3 | **Complete, and able to show it** | A reconciliation ladder from batch receipt to GL tie-out, a signed daily coverage certificate, a data-readiness grade per scenario, a quarantine queue instead of silent rejection, an MLRO coverage statement | C-25, C-26, C-27, S-03, S-54, L-01, L-04, L-05 |
| 4 | **Arabic is the original** | The normalised Arabic form is the system of record for matching. Chains are aligned, the mother's chain is compared, and Latin keys serve only as blocking aids. The interface is right to left first, with Western digits in data and Hijri dates for display only | N-01, N-07, N-13, N-16, N-20, UX-18, UX-19, UX-22, UX-25 |
| 5 | **No black boxes** | Declarative scenarios with generated plain-language text, reason codes tied to facts, rating explanations, model and rule cards. Threshold values are hidden only from branch roles | S-51, T-44, K-27, GV-30, GV-31, GV-41, GV-54, UX-42 |
| 6 | **People decide** | A score alone never rejects: it can produce at most a hold. STRs, restrictions and exits need a named human decision. Four-eyes is built in. GenAI drafts but never decides | C-65, SP-100, GV-43, M-19, M-20, M-70, GV-48 |
| 7 | **Every decision is recorded** | A no-report decision carries coded reasons and sits in a shelved-decision register. Posted notes are immutable and cases are sealed with hashes. Retention follows the strictest rule (Iraq floor 7 years) | M-04, M-05, M-26, M-27, M-59, M-61, RP-65, SP-53 |
| 8 | **Evidence is a product** | An assessor workspace mapped to the D2 tests, the CBI coverage report, evidence packs per report and per case, and a board pack, all generated from system facts | S-02, S-56, S-57, M-62, M-63, RP-63, RP-64, UX-49, UX-50, GV-35 |
| 9 | **Change is governed** | One maker-checker change request covers scenarios, thresholds, mappings, code maps, fail policies and roles. The studio workflow runs from unit test to replay, shadow, approval and monitoring. Every alert carries all its versions | TS-15, S-50, S-52, C-51, GV-24, GV-26, M-75 |
| 10 | **Proportionate** | The rating-to-controls matrix sets effort by risk, an inclusion lane serves low-risk customers, a light yearly check covers everyone, and thresholds are segmented from go-live | K-08, K-10, K-36, L-21 |
| 11 | **Never tuned to staff capacity** | No parameter caps alerts. A change that reduces volume carries a reason code (risk or capacity) and below-the-line evidence. Capacity is solved by ranking, grouping, staffing and surge mode, never by thresholds | L-16, L-18, S-39, M-11, M-52, M-55, UX-60 |
| 12 | **Tipping-off safe by construction** | A suspicion tier with per-case access lists and existence masking, neutral wording for every task and notice, inference tests that fail the build, and no AML reason ever written to the CBS | SP-26, SP-27, SP-31, SP-33, RP-62, M-65, M-69, F-16 |
| 13 | **One installation per bank** | Single tenant on the bank's own assets. Only content services (lists, typology packs, release bundles) are hosted, and they carry no bank data. Updates arrive as signed bundles | SP-89, SP-90, TS-34, TS-30, SP-66 |
| 14 | **Rules first; models earn their place** | The CBI pack and rules come first. A model ranks rule alerts only and never suppresses them in phase 1; it goes live only after shadow running and validation, beats a transparent baseline, and has enough labels. AI runs on premises and no suspicion content goes to external APIs | T-13, T-14, T-15, GV-14, GV-24, T-51, GV-52, TS-17 |
| 15 | **Unknown is never low** | A missing country, counterparty, address, rating or KYC value scores high or falls back to the segment percentile with a "degraded" flag. Gaps are shown before a deadline, not discovered at it | L-10, L-43, K-04, S-10 |

## 3. Decisions, module by module

Each bullet is a decision taken in the research, with the requirements that carry it. The modules follow the plan's sections 7 to 9; the detailed designs are in the cited files.

### 3.1 Connection to the core banking system (files 25, 32)

- Four connection tiers: a replica read (R1), read-only views on the primary (R2), log-based change data capture (R3) and a bank-delivered extract (R4). Ingestion polls first, behind a change-source interface, so a bank can move between tiers by configuration (C-01, C-06, C-39).
- Each deployment declares and proves which of three read-only levels it delivers (data plane, footprint-free, load-free); grants are checked on every connect (C-03, C-04).
- A signed per-bank load budget (sessions, statement time, rows per minute, end-of-day blackout, lag ceiling) and a kill switch; a DBA who locks the account puts the source in a normal "paused by bank" state (C-13, C-15, C-16).
- Change data capture only where the bank agrees: Debezium Server is the only JVM sidecar, for Oracle or Db2, sinking over HTTP or NATS JetStream; no Kafka and no Redis queue, because the pipeline runs on the PostgreSQL jobs and outbox harness (C-11, C-12, TS-08, TS-09).
- One `SourceReader` port with pure-Rust drivers where they exist and ODBC for Db2 and Sybase, proved by a driver conformance suite in CI (TS-04, TS-07, C-61).
- Adapter packs follow market evidence: ICS BANKS on Oracle first, then iMAL, BML ICBS, FLEXCUBE and Temenos; Finacle only on demand (C-38, C-62).
- A restore of the CBS to an earlier point, or a failover with lost commits, halts the watermarks and raises an alarm (C-64).
- Payments: native ISO 20022 and MT messages with the raw message kept, payment-gateway database adapters as the preferred source for domestic payments, and a link table from messages to ledger postings (C-41, C-42, C-43, C-44).

### 3.2 Canonical model, mapping and data quality (files 25, 20)

- One canonical model with lineage on every row, bi-temporal entities and UUIDv7 identifiers (C-46, C-47).
- Mappings are admin-managed, schema-validated data built from a closed transform library, published through maker-checker, with drift detection that pauses only the affected tables (C-50, C-51, C-52).
- Monitored data are never deleted because the source deleted them; loads are idempotent and the raw landing layer is kept (C-23, C-24).
- Scenarios evaluate at a settled time; late data recompute the affected windows and amend alerts with an explanation (C-22, S-09).
- Freshness classes, with `data_as_of` on every analyst screen (C-29).
- A coverage ledger, an exclusion register, a gate for new transaction codes and products, and drift detection on reference values (L-01, L-02, L-03, L-06).
- An Arabic and Kurdish character-set round trip at install, and a name data-quality profile per bank (C-10, N-12).
- Data are minimised at the source: no full card numbers, no PIN or authentication data, a card guard before landing, and no PCI DSS scope (C-36, SP-48, SP-51).

### 3.3 Customer risk rating and KYC (file 26)

- A hybrid rating engine (floors, weighted factor blocks and governed overrides) with five internal levels mapped to the CBI's High, Medium and Low (K-01, K-02).
- 27 starting factors in six blocks; an unknown applicable factor scores 60 and a mandatory unknown caps the best level (K-03, K-04).
- Rating floors FL-01 to FL-15 cannot be overridden per customer; a governed trigger library TR-01 to TR-26 re-rates customers on events (K-07, K-11).
- Two clocks per customer (classification review and KYC refresh) and a light automated yearly check for everyone (K-09, K-10).
- Overrides: upward easily, downward only by compliance, one level per decision and expiring within 12 months (K-24, K-25).
- A PEP register with tiers, kinship degrees and the CBI's PEP indicator pack (K-21, K-22, K-23).
- Beneficial ownership by chain calculation, with hidden-ownership scenarios (K-18, K-20).
- Ratings and KYC tasks live in AKR-TMS; the bank applies a one-way rating-change feed and sees a divergence view against the CBS risk field (K-35).
- No protected attributes or name-derived proxies as factors (K-33, GV-37).

### 3.4 Screening and Arabic name matching (files 22, 14, 28, 31)

- Names are stored as parsed chains with typed parts; Iraqi list formats and aliases are parsed; lexicons cover tribes, families, compounds and titles (N-01, N-02, N-03).
- One Rust normalisation library serves ingestion, batch screening and the phase-2 API, with a golden test file and a parity test against the browser (N-06, N-07, TS-19, TS-36).
- Scoring aligns tokens one to one, needs an evidence floor, never alerts on a family or tribe name alone, and returns "needs more data" when a query is too thin (N-25, N-26, N-29).
- Identifiers decide: the mother's chain, a year-only birth date with tolerance, and document numbers matched exactly (N-18, N-19, N-20, N-21).
- Thresholds per list are set against an alert budget agreed with the MLRO and a recall floor, tested on an evaluation harness and a synthetic Iraqi name corpus (N-31, N-32, N-37, N-38).
- Matching runs on immutable in-memory snapshots (p99 6 ms at 1.5 million names in the prototype, against 2.7 s for `pg_trgm`); `pg_trgm` serves analyst search only (N-49, N-53).
- Pair-keyed memory is offered only for incorrect matches and is invalidated when either record changes; good-guy rules are separate and expire (N-35, N-36, UX-44).
- Match states use the Office's terms (true, potential, incorrect) with the CBI's terms as aliases and a per-bank doubt policy; a potential match freezes nothing (M-24, UX-44, SP-98, RP-54).
- A screening alert is a (party, list entry) pair with its own queue and clocks; it becomes a case only when escalated or confirmed (M-12).
- The national list comes from a bulk or API-key feed negotiated with the Office, with the Excel export as a stop-gap and a reconciliation of IDs (N-45).
- The whole customer base is screened daily; lists are refreshed at least every 24 hours (12 hours under the 2025 controls), and each list change triggers a designation look-back (C-63, L-41, S-32).

### 3.5 Transaction monitoring and the scenario studio (files 27, 21)

- The CBI baseline pack is a versioned register of all 173 items that is never pruned, with a coverage report from item to scenario, alert, case and STR (S-01, S-02, T-02).
- A data-readiness grade (A to D) per scenario and bank names the missing field, writes out the request to the CBS vendor, and records "not covered" with an owner and an interim route (S-03, C-48).
- The AKR library (161 scenarios in 18 families) ships as an extension pack with typology tags, a basis tag on every parameter and an evidence label; bank scenarios use IDs 901 to 999 (S-04, S-05, S-59).
- Scenarios are declarative documents from a closed operator set, compiled to SQL, with no free SQL (S-51, TS-14).
- Every aggregable scenario aggregates across linked parties, and free-text counterparties are resolved (S-08, S-11, T-29).
- The branch observation log carries the circular's eight indicator groups; hybrid alerts combine an observation with a pattern; the seven SAR-escalation criteria are a checklist in every case (S-22, S-23, UX-67).
- First thresholds come from a bootstrap ladder, then replay on as-of history, sampling above and below the line, champion and challenger runs, and a go-live gate (S-40, S-41, S-42, S-44, S-47).
- A known-answer seeded suite runs daily for every CBI item, and an independent re-implementation diffs alerts against production (S-48, S-49, GV-10).
- Windows and baselines follow a calendar service with Hijri dates, Ramadan, Eid, Arbaeen and pay days (S-13, T-04).
- The public-funds family has restricted visibility; a front-company score opens a review and never an automatic exit; a typology exposure index covers the bank, its branches and correspondents (S-29, S-30, S-31).
- Replay history, results and parameter history are kept at least seven years (S-58).

### 3.6 Alerts, cases and investigation (file 28)

- One vocabulary throughout: signal, alert, internal report, authority request, RFI, case, task, decision, report, control incident (M-06).
- Every alert carries an evidence snapshot and a stable fingerprint; records that cannot form an alert go to a quarantine queue (M-07).
- One open case per party and case family: alerts are merged, never hidden, and each keeps its contribution outcome for scenario statistics (M-08, M-09, L-17).
- Priority class comes before score (P0: list match, terrorist-financing or proliferation indicator, authority order); the score shows its components; age raises priority; a daily sample of low scores is reviewed (M-03, M-13, M-16).
- Statuses are events; four-eyes is enforced by Cedar `forbid` rules and a database constraint; a delegated-authority matrix is signed by the MLRO (M-18, M-19, M-20).
- Outcomes are no report, UTR or STR (with TFR for terrorist financing), each with a named decider and coded reasons; the CBI's seven escalation criteria and a UTR-versus-STR aid sit in the decision panel (M-01, M-27, M-28, M-29).
- A UTR starts a watch (default 90 days) that ends in extend, close or an STR (M-35).
- No screen ranks individuals by speed; work in progress is capped at 10 per analyst (M-17, M-57).
- Quality assurance samples closures with an independent reviewer and a calibration kit (M-47, M-48, M-49, M-50).
- Branch requests go to the liaison officer only; answers are stored as claims with source and corroboration (M-43, M-45, L-47).
- No workflow engine: a Rust case aggregate with property tests, the DBS workflow DSL for approvals and tasks, and a per-bank policy pack pinned per case (M-74, M-75, TS-20).
- Network cases have a parent with a subgraph snapshot and one child per subject; the graph uses PostgreSQL edge tables and `petgraph`, with no graph database (M-10, T-21, T-22, TS-18).

### 3.7 Reporting to the AML/CFT Office (file 23)

- A goAML profile registry (XSD, patches, lookups, business rules, clocks, limits) read from the Office's XSD and never hard-coded, with a daily watcher on the Office's attachment list (RP-01, RP-02, RP-03, RP-82).
- Report types STR, SAR, TFR, UTR, CTR and AIF, plus the freeze and match reports; the MLRO has four outcomes (RP-11, RP-12, RP-16).
- Six validation layers, an Iraqi business-rule catalogue, a quality score with the Office's four grades, and a consistency checker (RP-27, RP-28, RP-29, RP-30).
- Mapping handles Iraqi name splitting, the mother's name, year-only birth dates and governorate to `state` (RP-17).
- Each report becomes a package (XML, attachments, bilingual PDF, SHA-256 manifest). No goAML credentials are stored and the portal is not automated: a named officer uploads and AKR-TMS records the reference, and portal content is captured before goAML purges it (RP-38, RP-39, RP-41, RP-79).
- Rejections are parsed into field findings; a "poor" grade creates a returned-for-completion task; a feedback register records the Office's sessions and the changes made (RP-42, RP-43, RP-32, RP-84).
- A daily CTR job tests the USD equivalent at the CBI rate of the day and ties control totals to the cash general ledger (RP-50, RP-51, RP-52).
- AI drafting works only from the evidence snapshot, with sentence-level sources and every number recomputed (RP-23).
- Foreign regime packs (US, UK, EU, Gulf) are a profile plus clocks, templates and an output adapter (RP-71 to RP-75).
- A pilot letter to the Office and the CBI is built from the open-questions register, and the answers become configuration (RP-09, RP-78).

### 3.8 Acting without writing: freezes, holds, restrictions and exits (files 23, 25, 28)

- Every action is a neutral task for bank operations with a clock; completion carries the CBS reference; AKR-TMS verifies it from its copy of the data and stores the verification level reached, V0 to V3 (M-32, M-33).
- A true match creates a freeze task (default 8 hours, never beyond 24 hours from the listing decision), the first report to the Committee within 48 hours (copy to the Office) and the second at two months; AKR-TMS then checks that the block is in place and that no debit, fees included, followed the order (M-34, RP-55, RP-56, C-63).
- An instruction to refrain from the Office runs a 7-working-day clock with confirmation and a post-check (RP-45).
- Release after delisting happens only on a recorded decision (RP-59); matches the bank cannot resolve go to the Committee's technical committee as a referral (RP-58).
- Exits need senior approval and an individual assessment; there are no group exits (M-36).

### 3.9 Islamic banking (file 24)

- A canonical Islamic product taxonomy and a deal-centric model (deal, parties by role, asset, parallel contracts, evidence status) (I-01, I-02).
- Due diligence and screening extend to suppliers, sellers, brokers, developers and partners (I-03); hard rules cover the third-party seller, prohibited structures, qard hasan caps and the real-estate two-year rule (I-04, I-06, I-07, I-08).
- A possession-evidence tracker, and a freeze workflow that involves Finance and the Sharia board for pooled funds (I-05, I-10).
- The Sharia board sees aggregate dashboards only, never cases or STRs (I-16).
- The Islamic pack is optional: file 24's 57 scenarios become 8 library scenarios with basis labels, and Islamic contracts are never presented as higher risk in themselves (S-37, I-20, I-30).
- A book flag for Islamic windows and an alert on transfers between books (I-17).

### 3.10 Governance, models and AI (files 29, 21)

- An inventory record before use for every rule, parameter set, model, matcher, reference list and language-model component, with a tier, a named owner, a validator and a committee (GV-01, GV-03, GV-04).
- A seven-part validation pack, a per-bank delta validation, and an independent re-implementation of every rule (GV-09, GV-15, GV-10).
- Shadow mode for every new or changed Tier 1 or 2 object, a permanent challenger, and a kill switch back to rules only (GV-24, GV-25, GV-20, SP-95).
- Starter models are flagged "externally calibrated, temporary"; label gates of about 50, 200 and 500 positives decide when local models may train (GV-05, T-15, T-63).
- Gradient-boosted trees with monotonic constraints are the default model; models are served as ONNX inside the bank; no external AI API (T-14, TS-16, TS-17).
- GenAI is limited to an enforced list of permitted uses, checked by a grounding verifier, accepted by a person before save, and gated by at least 200 QA-confirmed cases in Arabic and English (GV-45, GV-47, GV-48, M-71, M-73, T-49).
- Fairness: a protected-attribute register, a proxy scan, monthly outcome metrics, and kinship links hidden by default (GV-36, GV-37, GV-38, T-24).

### 3.11 Security, privacy and records (file 30)

- Trust zones Z0 to Z7 with default-deny policies and mutual TLS (SP-04, SP-08).
- A key hierarchy with Shamir unseal, a transit key-encryption key and data keys per partition-month; OpenBao stays out of the hot path; object-store data are encrypted before upload with OpenBao-wrapped keys, on encrypted volumes, without MinIO's KES (SP-09, SP-11, TS-24; file 32 section 12.6).
- MFA for every user and a fresh step-up to open suspicion-tier records (SP-14, SP-15, TS-25).
- Deny-by-default Cedar authorisation with roles and attributes (branch scope, tier, case access list), segregation-of-duties pairs, and break-glass with two approvers and a four-hour limit (SP-19, SP-21, SP-24, TS-26).
- An append-only, hash-chained audit trail kept without limit, exported to the bank's SIEM, with the auditors' own searches logged (SP-39, SP-42, SP-43, SP-44).
- One retention engine (classes R1 to R13, jurisdiction profiles, Iraq floor of seven years for AML data, crypto-shredding of partitions, legal holds, deletion certificates replayed after a restore) (SP-53, SP-56, SP-59, SP-60, M-05).
- No standing vendor access, redacted support bundles, signed updates with an SBOM, ISO/IEC 27001 before the first bank, and an independent penetration test (SP-03, SP-64, SP-65, SP-66, SP-74, SP-78).
- AKR-TMS is registered as a regulatory-critical system; outages are recorded as blind periods and alerts are never back-dated; restores are drilled monthly (SP-83, SP-85, SP-87, TS-39).

### 3.12 Analyst experience and the Arabic-first interface (file 31)

- 59 screens are catalogued (8 have mockups); the catalogue is machine-readable and drives the role-aware rail and the access matrix (UX-01, UX-53).
- The case workspace fits one page; the alert page shows evidence first, with at most three headline reasons, each beside its fact (UX-06, UX-08).
- Actions are separated from decisions; decisions need coded reasons; P0 and P1 items cannot be closed in bulk (UX-64, UX-65, UX-13).
- Western digits in data and tables, numeric expressions isolated for bidirectional text, ICU plurals with all six Arabic forms, and Hijri dates for display and entry only (UX-18, UX-19, UX-22, UX-24).
- WCAG 2.2 AA in both themes: darker on-tint text tokens, a control-border token of at least 3:1, and gold never used on light surfaces (UX-32, UX-33, UX-34).
- The branch window is a separate, light bundle (about 300 KB) that shows neutral statuses only (UX-02, UX-66).
- Usability rounds with Iraqi compliance staff, in Arabic first; response budgets are tested in CI (UX-03, UX-16).
- IBM Plex Sans Arabic, Plex Sans and Plex Mono with the extended Latin subsets, checked by a glyph-coverage test (UX-26).

### 3.13 Platform and stack (file 32)

- The DBS conventions (layered crates, bounded contexts, Axum, utoipa, Orval, problem+json) and four binaries: `akr-api`, `akr-worker`, `akr-ingestd`, `akr-assess` (TS-01, TS-02).
- PostgreSQL schemas and roles with monthly partitions; collation and ICU locale fixed at `initdb` and verified for Arabic (TS-12, TS-13).
- Valkey instead of Redis and OpenBao instead of Vault, with a notices file, an SBOM per release and legal sign-off (TS-23, TS-37).
- MinIO as a pinned Silo build from source (Chainguard's fork as second source), behind an S3 guard, with three kinds of credential, COMPLIANCE object lock, a WORM conformance suite and an advisory watch; SeaweedFS and RustFS are tested exits only (TS-22, TS-41, TS-42, TS-43; file 32 section 12).
- A three-server K3s profile for high availability, Helm and compose from one source, and signed OCI bundles imported into the bank's Harbor (TS-29, TS-30).
- Prometheus, the OpenTelemetry Collector and Jaeger, with Grafana and Loki optional, and SLOs as code (TS-27, TS-28).
- Notifications only in-app and through the bank's SMTP relay, never with subject names (TS-21).
- The DBS platform layer is treated as unbuilt and planned as real work (TS-03).

### 3.14 Phase 2: checks before payments leave (files 25, 21, 29, 30)

- A synchronous assess endpoint for the payment gateway, with mutual TLS, signed decisions and byte-exact replay (C-55, T-43).
- Service objectives of p95 150 ms and p99 400 ms, to be proven by load test; inline name matching under 25 ms at p99 (C-56, N-50).
- The fail-open or fail-closed policy is bank data per rail, corridor, currency and amount; each rail goes live through shadow and advisory modes (C-56, C-57).
- A score produces at most a hold; a reject without a person is allowed only for deterministic, bank-configured prohibited conditions; the API returns decisions and reason codes, never scores (C-65, SP-93).
- A release-without-decision control, a local fallback screen beside the gateway, and recorded blind periods (C-58, C-59).
- Tier 1 governance by default (GV-53).

## 4. Conflicts between files and how they were resolved

The research found conflicts of two kinds: between Iraqi texts (resolved by taking the binding or the stricter text and keeping the other visible), and between research files written at different times (resolved by aligning the older file). Every file named in the last column now says the same thing.

| # | Topic | What the texts or files said | Resolution | Recorded in |
|---|---|---|---|---|
| 1 | **Freeze clock** | CBI sanctions guidelines 2023: freeze "immediately" on a confirmed match. The Office's March 2023 implementation manual: within 8 hours of the lists' publication. Regulation No. 6 of 2023: "without delay", defined as within 24 hours of the listing decision; the Office's September 2026 guide says the same | Product default 8 hours (the strictest text); legal outer limit 24 hours from the listing decision; a bank that relaxes the default records the legal basis | RP-47, RP-55, M-34, C-63; file 28 section 3.4; file 30 section 16.2 (aligned 2026-10-07) |
| 2 | **Report to the freezing committee** | The CBI's 2023 guidelines: inform the Committee within three days, quoting Regulation No. 5 of 2016 (repealed); files 25 and 30 had copied the three days. Regulation No. 6 of 2023: first report within 48 hours of the freeze, copy to the Office, second within two months | 48 hours and two months everywhere; the three days are superseded | Files 06 section 8, 23 section 3.4, 25 section 15.7 and C-63, 28 M-34, 30 section 16.2 |
| 3 | **Freeze-report channel** | File 01 described the IQTFS forms (match and frozen-asset volume); the Office's guide says only "approved channels and forms", and the Iraqi goAML schema has no freeze-report type | AKR-TMS produces the controlled report and the data for the IQTFS forms; the channel is a bank setting until the Office answers (section 5, item 1) | RP-56, RP-57; file 23 section 3.4 and question 7 |
| 4 | **Report outcomes** | Files 01 and 06 and the first plan knew two outcomes: STR or no report. The Office's 2026 guide adds the UTR: goAML only, no suspicion needed | Three case outcomes (no report, UTR, STR with TFR); four MLRO outcomes counting "gather more evidence"; a watch after a UTR; UTR-to-STR links; the UTR tier is a per-bank switch, on by default | M-01, M-29, M-35, RP-11, RP-12, RP-19; file 06 (updated); plan reporting section (both editions) |
| 5 | **STR deadline** | Law 39 article 12 (Fifth) is rendered "immediately" in file 23 and "without delay" in file 30; no Iraqi text gives a number of days | No numeric deadline is claimed. Internal target: filing the same day as the decision, with escalation at 24 hours; a suspicion-formed date is kept separately | RP-47, RP-48, M-31 |
| 6 | **Match terms and the doubt rule** | CBI 2023: potential, confirmed, false positive, and a match is confirmed "when there is any suspicion or doubt". Office 2026: true, potential, incorrect; a match is true only "when sufficient indicators confirm" identity, and similarity alone is a potential match | One internal state machine. The Office's terms are the labels and the CBI's are aliases in help and exports. A per-bank doubt policy decides how unresolved doubt is handled. A potential match freezes nothing; unresolved pairs can be referred to the Committee's technical committee | M-24, UX-44, SP-98, RP-54, RP-58; file 28 section 3.4; file 30 section 19; mockup 03 relabelled |
| 7 | **Pair memory** | File 22 designed pair-keyed memory for cleared alerts; mockup 03 showed "remember this pair" ticked while a match was being escalated | Memory only for incorrect matches, invalidated when either record changes; the mockup is patched (unticked, "incorrect matches only") | N-35, UX-44; file 31 section 12.2; plan figure 3 caption |
| 8 | **Retention** | Law 39 article 11 and the CBI controls: at least 5 years. The CBI's reform assessment guide (D2.1(c)(1)): 7 years of AML data, including STRs, CDD files and audit logs. OFAC (2025): 10 years for sanctions records where US rules reach. EU AMLR: 5 years, then delete. File 30 had 5-year floors, file 28 a 7-year floor, file 18 a 7-year default | Iraq profile floor of 7 years for the AML data classes (R2, R3, R5, R6, R7, R8); default 10 years for alerts, cases, STRs and sanctions records; OFAC and EU profiles switchable with a recorded basis. Whether the CBI means 7 years outside the assessment stays open (section 5) | M-05, VD-19, SP-53, R-24; file 30 section 12.3 and file 28 section 12 (aligned 2026-10-07) |
| 9 | **The CBI scenario count** | Auditors' paragraphs in two banks' annual reports: a CBI minimum of "25 scenarios". The 2024 MER: 34 to 56 scenarios after the CBI's sectoral assessment. The CBI controls of 2025: 86 numbered scenarios plus 87 indicators, 173 items | The 2025 controls are the floor: the baseline pack holds all 173 items and the plan cites the 86 numbered scenarios; 25 and 56 are older baselines, to be named as such with banks | S-01; file 01 section 2 (note); file 27 Part A and B22; file 18 |
| 10 | **Object store** | The DBS uses MinIO, whose upstream repository was archived in April 2026, with images no longer pullable and seven 2026 advisories fixed only in the proprietary AIStor and in forks. File 28 had named SeaweedFS as the default candidate, and file 30 relied on MinIO's KES for keys | The owner's decision (2026-10-07): keep MinIO as a pinned, maintained, renamed build (Silo; Chainguard's fork as second source) behind an S3 guard, with an advisory watch. SeaweedFS and RustFS are tested exits only. Data are encrypted before upload with OpenBao-wrapped keys; no KES | File 32 section 12, TS-22, TS-41 to TS-43; `_minio-notes.md`; files 28 and 30 (aligned) |
| 11 | **Secrets and cache** | The DBS uses Vault and Redis | OpenBao (MPL-2.0) and Valkey (BSD), for licence reasons and the legal screen; descriptions of the DBS keep its names | TS-23, TS-37, VO-07; files 07, 08, 12, 21, 22, 25, 30 (aligned) |
| 12 | **Alert volume and staff capacity** | Mockup 05's example change request gave a capacity reason ("too many alerts on normal multi-branch use"), against the plan's own lesson | A change that reduces volume carries a reason code and below-the-line evidence; the mockup reason now reads "Risk-based: 3-branch deposits found low risk" | L-18, S-39, M-55, UX-60; plan figure 4 caption |
| 13 | **Liaison monthly report** | Mockup 01 sent the liaison officers' monthly report to the CBI; the CBI controls send it to the MLRO by the 10th | Mockup text patched | File 01 section 2d; RP-13, M-25; `plan/images/finish.py` |
| 14 | **Digits** | The plan says "both styles of digits". File 31 counted 16,197 Western against 944 Arabic-Indic digits in Iraqi regulator texts, and found that Plex Sans Arabic's Arabic-Indic digits are proportional, which breaks table columns | Both styles are supported: Western digits by default in data and tables, Arabic-Indic optional in prose, and every input accepts Western, Arabic-Indic and Persian digits. The plan's wording stands | UX-18 |
| 15 | **Requirement count** | Adding up the per-file counts gave too many, because file 11 repeats VA-01 to VA-56 as an index and some prefixes are not requirements | Count unique PREFIX-NN across all files, excluding FM, FL, TR, G and IR: **1,208** | `_progress.md`; plan Appendix B (both editions) |

## 5. Open decisions

None of these blocks the start of phase 0. Each has a working default, so the build can proceed; the default becomes configuration once the decision is taken.

| # | Decision | Who decides | Working default | Source |
|---|---|---|---|---|
| 1 | **The twelve questions to the AML/CFT Office and the CBI**: any numeric STR or UTR deadline; whether the server accepts the schema's newer elements; how it treats the entity-name pattern and which characters it forbids; a pilot environment with the XML validator, rejection messages and purge periods; attachment rules; duplicate transaction numbers when a UTR becomes an STR; the channel for freeze and match reports and whether a TFR is expected for a true match; replies to information requests and holds; CTR aggregation and corrections; the meaning of Iraqi code-table additions; the format of the MLRO's annual report; CRS and FATCA duties | The Office and the CBI, through the pilot bank's letter (RP-78) | **Owner answered 11 of 12 on 2026-10-07** (file 23 section 9): no fixed STR deadline; newer fields accepted; schema name rules as written; no test environment; no set attachment rules; an STR may reuse a UTR's transaction numbers; freeze report by letter or email; replies to requests by letter; CTR aggregated per day; no set MLRO report format; FATCA and CRS both apply. Still open: code-table meanings, the report-date element, rejection messages and purge period, and whether a TFR follows a true match | File 23 section 9 |
| 2 | **The 62 extension scenarios**, including the public-funds family: the CBI controls let a bank add scenarios "in coordination with" the AML/CFT Office | Each bank's MLRO with the Office | Extension scenarios run in replay and shadow, and become the bank's own set only after agreement, using the Office engagement pack | S-04, S-60; file 27 Part A legal note |
| 3 | **The legal screen of suppliers and components**: Iraq's 2022 anti-normalisation law (Israeli-headquartered vendors such as NICE Actimize, and ThetaRay with Screena), any component, data feed, model or subcontractor from a barred party, and the OFAC exposure of US vendors | Iraqi counsel | Excluded pending counsel; an SBOM with maintainer and country for every component | VO-07, TS-37; file 07 section 7 |
| 4 | **CBS data readiness for each bank**: which fields exist (mother's name, teller events and depositor IDs, card and trade modules, payment messages, hold and freeze status) | Each bank and its CBS vendor, in phase 0 | Grades shown per scenario, gaps declared with an interim route and an owner, vendor requests written out | S-03, C-40, N-56; file 28 section 16.8 (question 6) |
| 5 | **Contrast fixes to the approved identity for the product interface**: gold on light surfaces (2.4:1), amber text (4.24:1 on Paper), sand control borders (1.44:1) and green text on Clay (4.44:1) fail WCAG 2.2 AA. **Decided 2026-10-07:** the owner locked the design together with these token changes | The product owner (decided) | Locked in `plan/identity.md` (screen contrast tokens: teal text `#177272`, amber text `#9C5D00`, green text `#2D794C`, control border `#7F8695`, gold only on the dark rail); the brand colours are unchanged | UX-33, UX-34; file 31 section 5.3 |
| 6 | **The seven-year floor outside the assessment**: does the CBI mean D2's seven years to replace the controls' five for all records? | The CBI; each bank's counsel | Seven-year floor in the Iraq profile | M-05; file 28 section 16.8 (question 8) |
| 7 | **UTR adoption and the watch**: whether each bank adopts the UTR tier at once and treats "enhanced monitoring" as the watch | Each bank | UTR tier on; 90-day watch | M-01, M-35; file 28 section 16.8 (question 1) |
| 8 | **The doubt policy for screening**: whether unresolved doubt counts as a true match (the CBI's 2023 reading) or stays a potential match (the Office's 2026 reading) | Each bank's MLRO, ideally confirmed by the Office | Chosen at installation; a potential match must be resolved within a deadline equal to the freeze clock; every freeze based on doubt gets a second review within a day | M-24, SP-98; file 28 section 3.4; file 30 section 19 |
| 9 | **Vendor-operated hosting**: may a supplier run AKR-TMS for a bank? | The CBI (Information Technology and Payments Department) | One installation per bank on its own assets; only content services hosted | SP-89, SP-90; file 30 sections 17 and 20 |
| 10 | **The national list feed**: a bulk or API-key feed from the Office instead of the unreliable paged service | The Office | Excel export as a stop-gap, reconciled by ID | N-45; file 22 |
| 11 | **Phase 2 adoption**: whether a bank uses the pre-release API, on which rails, and with which fail-open or fail-closed policy | Each bank | Off; each rail goes through shadow and advisory modes first | C-56, C-57, GV-53; file 25 |
| 12 | **Licences**: AGPL source obligations when distributing the Silo build; OpenBao (MPL-2.0) and Valkey (BSD) notices; OpenSanctions data (CC BY-NC, so a commercial licence is needed); PyArabic (GPLv3, not linkable) | Counsel | A notices file and an SBOM per release; components with unclear terms excluded | TS-37, N-48, C-12; files 22, 25, 32 |
| 13 | **Third-party risk data** (World-Check is named by the CBI; PEP, ownership and adverse-media data) | Each bank | A pilot on Iraqi samples (200 companies, 100 officials) before purchase | K-41; files 15, 26 |
| 14 | **GenAI features**: which of the nine use cases each bank switches on | Each bank, after the evaluation gate | Off; drafts only, never decisions | T-48, M-70, M-73; files 21, 28, 29 |
| 15 | **Early modules inside the reform window**: banks that filed their assessment by 30 September 2026 have six months, to about the end of March 2027, to fix gaps, while a first bank goes live in about nine months | The product owner (decided) | **Decided 2026-10-07:** offer both early deliverables that help a bank's remediation file: the phase 0 discovery with the CBI coverage gap report and data-readiness grades (S-02, S-03, C-48), and the goAML validator and quality grade against the Office's published schema (RP-27, RP-29). The plan PDF stays as it is (owner's choice); the roadmap there is unchanged | Plan sections 2 and 11; files 18, 23, 27 |

## 6. Risks

| # | Risk | What would happen | How the design limits it | What remains |
|---|---|---|---|---|
| 1 | **CBS data gaps** | Only 47 of the 173 CBI items run on ledger fields; the mother's name, teller events and depositor IDs are often missing. Scenarios run degraded, matching weakens, and goAML files fail because the Office's schema requires the mother's name | Data-readiness grades, vendor requests written out, the branch observation log, optional connectors, and gaps shown before the MLRO's deadline (S-03, S-19, S-22, N-56, RP-26) | High at first: each bank's CBS vendor must deliver fields |
| 2 | **Read access refused, slowed or withdrawn** | Approval delays the start; load complaints close the connection | Four connection tiers including a bank-delivered extract, a signed load budget, a "paused by bank" state, and recorded blind periods (C-01, C-13, C-15, SP-87) | The approval time drives the roadmap, as plan section 11 says |
| 3 | **Regulatory change and conflicting texts** | The Office's texts disagree (freeze clocks); the schema and circulars change | A profile watcher on the Office's schema, a catalogue importer for new CBI items, a clock engine with recorded bases, an open-questions register (RP-03, S-06, RP-46, RP-09) | Each new text needs a reading by a person |
| 4 | **goAML rejections** | Unpublished business rules, the entity-name pattern defect and no known test environment lead to rejected files | Six validation layers, a compatibility layer, a regression corpus, parsing of rejections and a feedback register (RP-04, RP-06, RP-27, RP-42, RP-84) | The server's behaviour stays unverified until the pilot |
| 5 | **Tipping-off and insider leaks** | Branch staff or a colluding employee learns of a case | Suspicion tier, existence masking, conflict checks, canary records, inference tests in CI, and user-behaviour rules on AKR-TMS's own audit trail (SP-26 to SP-36, M-67) | People; the controls make misuse visible, not impossible |
| 6 | **Object-store supply chain** | Silo is maintained by a small team without a service-level agreement | A pinned build from source, a second source, an S3 guard, a WORM conformance suite, an advisory watch and tested exits (TS-41 to TS-43; file 32 section 12) | Medium; review yearly |
| 7 | **Alert volume above staff capacity** | Pressure to cap alerts or raise thresholds, a failure that regulators have punished repeatedly (file 20) | No caps; ranking, grouping, surge mode, a capacity workbench, analyst hours shown before a rule set is activated, and staged activation of scenarios (M-52, M-54, M-55, N-54) | Banks must staff the work; the product makes the gap visible |
| 8 | **Screening misses or floods** | Common Iraqi names, thin identifiers and an incomplete list feed | Chain-aware matching with the mother's name, an alert budget with a recall floor, "needs more data", list reconciliation and a known-answer harness (N-26, N-29, N-32, N-45, L-32) | Depends on identifier quality in each CBS |
| 9 | **Too few labels for machine learning** | About 1.7 money-laundering STRs per bank per year cannot train local models | Rules first, label gates, weak labels, synthetic data validated against partner aggregates, leave-one-bank-out evaluation, models in shadow (T-15, T-57, T-58, T-62) | Machine-learning benefits come late |
| 10 | **Legal and sanctions exposure of suppliers** | A barred or sanctioned supplier, component or data feed | The legal screen, an SBOM with maintainer and country, open components run on premises (VO-07, TS-37) | Counsel's opinion is still needed (section 5, item 3) |
| 11 | **Unproven typologies** | Scenarios labelled [N] (real estate, Captagon, zakat, Kurdistan) rest on thin evidence | Evidence labels on every scenario; validation on the first bank's own cases before they are called proven (S-59) | Until validated, they are hypotheses |
| 12 | **Evidence weight in court** | Law 78 of 2012 gives electronic records paper-equivalent weight under three conditions; court practice is unverified | Case sealing, hash chains, chain-of-custody logs, and a per-bank statement for counsel (M-61, M-63, M-64) | Needs counsel and, in time, practice |
| 13 | **Many single-tenant installations** | Support cost and version drift across banks | One product, adapter packs, per-bank policy packs, signed bundles, and pinned versions with advance notice (C-38, M-75, TS-30, GV-29) | Grows with the number of banks |
| 14 | **Timing against the reform** | The remediation window closes before a first bank is live | Two early deliverables, decided 2026-10-07 (section 5, item 15) | Selling them in time |
| 15 | **Accessibility and Arabic rendering** | Palette contrast failures; Arabic screen-reader behaviour unverified; Latin font subsets missing some letters | Token fixes, a glyph-coverage test, and accessibility testing in Arabic (UX-26, UX-33, UX-39) | Arabic screen-reader behaviour is unverified (file 31) |

## 7. How the research feeds the plan

| Plan section | Research files |
|---|---|
| 1. Summary | All, through this file |
| 2. Why now | 01, 02, 18, 20 |
| 3. What the rules require | 01, 03, 04, 05, 06 |
| 4. What the market offers | 07 to 18 |
| 5. Lessons from enforcement | 20 |
| 6. Design principles | Section 2 of this file |
| 7. The system, module by module | 21 to 28, 31 |
| 8. How it connects to the bank | 25, 30, 32 |
| 9. AI, used responsibly | 21, 29 |
| 10. One product for many Iraqi banks | 05, 24, 30, 32 |
| 11. Roadmap | 25, 28, 32 |
| 12. What success looks like | 20, 21, 27, 28 |
| Appendix A. Glossary | 06 |
| Appendix B. The research behind this plan | All; the requirement count (1,208) |

## 8. Sources

This file cites only the research files in this folder; the primary sources (laws, regulations, CBI and AML/CFT Office texts, standards, vendor documents, enforcement actions) are listed in the Sources section of each file.

- Files 01 to 32 in `research/`, as cited by number and section.
- `research/_minio-notes.md` (the MinIO decision, forks and advisories, 2026-10-07).
- `research/_progress.md` (the log, the requirement count method and the alignment record).
- `plan/plan.md` and `plan/plan-ar.md` (the reader-facing plan, both editions), `plan/identity.md` (the approved identity).
