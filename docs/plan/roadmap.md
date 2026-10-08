# AKR-TMS Implementation Roadmap

**Start date:** 2026-10-07 (planning)
**Target:** AML and KYC risk and transaction-monitoring system for Iraqi banks, one installation per bank on its own
servers, read-only toward the core banking system. Arabic-first right-to-left web app on Rust/Axum and React 19.
Delivers: platform foundation and release bundles; standalone identity (Kratos and Hydra, mandatory MFA, single-use
step-up); Cedar authorisation with sensitivity tiers and existence masking; a hash-chained, anchored audit trail and
evidence vault; governed change control on the DBS workflow engine; read-only CBS discovery, the CBI coverage gap
report and the goAML checker (the two early pieces); mapping to a canonical model; production ingestion with
freshness and reconciliation; reference data, calendars and the policy pack; features and segments; screening with
Arabic name matching; customer risk rating and KYC; the scenario engine with the CBI pack, the Iraq library and the
studio; alerts, cases, decisions, RFIs, watches, network view and QA; operations tasks and freezes; goAML reporting;
the branch window; retention; governance, registers, command centre and evidence packs; and in P2 models, the
assistant, the assessment API and high availability. Full product plan in [`docs/`](../README.md).

**Scope (planning numbers, 2026-10-07):** about 180 domain tables (DATA-MODEL.md) · about 366 routes (API.md
section 3 and 4) · 32 phases (00-31) over macro-phases P0-P2 · 4 application binaries (`akr-api`, `akr-worker`,
`akr-ingestd`, `akr-assess`) and 2 support binaries (`akr-s3guard`, `akr-ctl`; DR-24) over shared crates · 5 declarative document kinds (DSL.md: scenario, mapping, rating model, change
request, workflow) · 61 screens (DESIGN-SYSTEM.md 8.1) · 1,208 research requirement IDs as the coverage baseline.
Job kinds, services and exact table counts are filled in from the phase files.

**Locked architecture decisions** (rationale in ARCHITECTURE.md and research.md):
- ARCHITECTURE.md D1-D22 are frozen; phase files implement them and never re-litigate.
- Durability is PostgreSQL only (`jobs`, outbox, timers); Valkey is cache, rate limits and SSE fan-out; no broker.
- `akr-ingestd` is the only process that reaches the CBS; every CBS statement is a generated single SELECT proven by
  `sqlparser`; the connection fails closed on any write privilege.
- Cedar embedded in process; OpenBao never in the request hot path.
- Every mutation: Idempotency-Key + If-Match + audit event. UUIDv7 ids generated in the application (DR-06).

## Phase Overview

| # | Phase Name | Scope | Size | Macro | Depends On | Status |
|-|-|-|-|-|-|-|
| 00 | Platform Foundation | Workspace with four binary stubs and shared crates; compose stack (PostgreSQL 17 + standby, OpenBao, Silo MinIO with WORM buckets, Valkey, Kratos/Hydra, Gotenberg, clamd, observability); config; AppError and problem+json catalogue; correlation and idempotency middleware; `jobs`/outbox/timers harness; settings and flags; column classification and retention assignment; OpenAPI shell; CI and the signed release bundle; DBS rules ported | L | P0 | — | Spec ready |
| 01 | Identity & Sessions | Kratos/Hydra wiring, BFF sessions and CSRF, ar/en auth flows, MFA (WebAuthn first), forced first login, `/me`, step-up AAL2 single-use ledger, PATs, users mirror, pools, delegations, idle time-out; bootstrap CLI (first ADM and SEC), client-credential service principals, password history and CBI floors | L | P0 | 00 | Spec ready |
| 02 | Audit Trail & Evidence Core | Partitioned `audit_log` and `access_event` with grants, per-partition hash chain, OpenBao-signed anchors to WORM, audit write crate, evidence objects and packs (write path), Gotenberg rendering service | M | P0 | 00 | Planned |
| 03 | Authorisation, Users & Roles | Cedar bundle load and hot reload, two gates (scope, policy), attribute resolution with Valkey cache, existence masking middleware, SoD `forbid` pairs, decision logging, simulator; users and roles admin (A1), administrator unlock and credential reset, branches (`admin.branches`, DR-27), one-click offboarding, access review stub; admin settings read, flags, jobs and retry, ops cockpit v1 (data for A2 and A3) | L | P0 | 01, 02 | Planned |
| 04 | Change Control & Approval Workflows | `change_requests` with computed class, volume effect and route (DSL.md Part D); DBS workflow engine ported (definitions, versions, instances, events, tasks, actions, claim, four-eyes CHECK), inbox and task endpoints (API.md 3.21), the shipped definitions (DSL.md Part E), emergency route timer; inventory items; role grants and settings now change by CR | XL | P0 | 03 | Planned |
| 05 | Sources, Read-only Connection & Discovery | `akr-ingestd`: `SourceReader` port and drivers (PostgreSQL, SQL Server, Oracle, Db2, Sybase), connection tiers CT1-CT4, grant check on connect, `sqlparser` SELECT proof, signed load budget, paused-by-bank, the read-only conformance suite; discovery profiling and field availability (M5) | XL | P0 | 04 | Planned |
| 06 | CBI Catalogue, Readiness Grades & Gap Report | The 173 CBI items register, scenario register stubs and catalogue links, data-readiness grades from field availability, draft field mapping candidates, the CBI coverage gap report and its signed PDF pack (M7, M11, M16); screen E2 | M | P0 | 05, 08 | Planned |
| 07 | goAML Checker | goAML profile registry (Office XSD, never hard-coded), schema watcher, validation layers and quality grade, check runs and findings, regression corpus, rejection parser stub (M13 P0); screen E1; installable without a CBS connection | M | P0 | 03, 08 | Planned |
| 08 | Frontend Foundation | Tokens (`tokens.json` → `akr.css`) and contrast check; Untitled UI vendored and RTL-migrated; one Tailwind entry and lint; app shell and reduced shell; i18n kit (bidi, datetime, fold, plurals, digits, AriaLocale); fonts with `latin-ext` and glyph test; shared components for P0; `screens.ts` and route-to-policy test; X1, X6, A1-A3 minimal, X4 security section; RTL and graph spikes | M | P0 | 01 | Planned |
| 09 | Reference Data, Calendars & Policy Pack | `ref` schema: countries, risk overrides, governorates, currencies, FX rates, calendars and days, seasonal calendar, reason codes; the policy pack (routing, clocks, templates, switches, forbidden-word list) and the delegated-authority matrix, both by CR | L | P1 | 04 | Planned |
| 10 | Mapping & Canonical Model | Mappings and versions, code maps with gates, drift events, xref, attributes (DSL.md Part B: transform library, card guard, survivorship, generated SELECT); the `canon` schema with partitions and lineage; mapping workbench (T3) | XL | P1 | 05, 09 | Planned |
| 11 | Production Ingestion & Freshness | Watermark-driven incremental and sweep loads within the budget, raw landing partitions as the replay log, batch receipts, mapping jobs, freshness classes F0-F5 and `data_as_of`, CBS restore and failover detection, blind periods, first adapter pack (ICS BANKS) | L | P1 | 10 | Planned |
| 12 | Reconciliation, Coverage & Data Quality | Recon runs and results (ladder), coverage ledger and certificates, exclusions, dark units, DQ issues and quarantine queue; screens T1, T2, T4 | L | P1 | 11 | Planned |
| 13 | Features & Segments | Feature definitions and values, segments (static, cluster, risk overlay) and segment statistics, peer groups, expected profiles | M | P1 | 11 | Planned |
| 14 | Screening Engine & Lists | Normalisation library and SQL twin, lists and versions (UN, national, internal) with reconciliation, immutable index, scorer, screening runs, alerts and decisions (Office states), pair memory, good-guy rules, known-answer harness; screens S1-S4 | XL | P1 | 09, 11 | Planned |
| 15 | Risk Rating & KYC | Rating model RM-IQ engine (DSL.md Part C), ratings, overrides, rating feed, KYC reviews, PEP register and relatives, ownership results; screens K1-K4 | XL | P1 | 13, 14 | Planned |
| 16 | Scenario Engine, Packs & Studio | Scenario versions and parameter sets (DSL.md Part A), compiler (aggregate SQL, CEL events, profiles), runs, replays, shadow results, seeded tests; CBI baseline pack and Iraq library; screens D1, D2, D3 | XL | P1 | 13 | Planned |
| 17 | Alerts, Triage & Queues | Alerts with fingerprint, evidence snapshot and producing versions, alert quarantine, one open case per party and family with merge, priority classes by rule, queues and saved views, SSE counts; screens W1, W2 | L | P1 | 14, 16 | Planned |
| 18 | Cases, Decisions & Investigation | Case aggregate (events, parties, ACLs and tiers), decisions via workflow tasks with coded reasons and the generic-text check, evidence-opened gate, notes, work items, evidence items, entity links; screens W3, W4, W5, W7 | XL | P1 | 17 | Planned |
| 19 | RFIs, Watches, Network & QA | RFIs (six kinds, templates, clocks, four-eyes release), internal reports and authority requests, watches and hibernations, network snapshots and network cases, QA samples, reviews and findings; screens W6, W8, W10, W11, G3 | L | P1 | 18 | Planned |
| 20 | Operations Tasks & Freezes | Ops tasks with neutral text codes, verification V0-V3, freezes with clocks and the 48-hour Committee report, failure incidents; screens W9, S5 | M | P1 | 14, 18 | Planned |
| 21 | Reporting to the Office | Reports and versions (UTR, STR, TFR), goAML packages validated by the phase 07 engine, filings and acknowledgements, FIU feedback, CTR batches, letters, periodic reports; screens R1, R2, R3, R5 | XL | P1 | 07, 18 | Planned |
| 22 | Branch Window & Notifications | Branch inputs, observations, confidential reports, monthly liaison reports; notifications, templates and preferences (generic text only); the branch bundle (B1-B6) and X3 | L | P1 | 19, 20 | Planned |
| 23 | Retention & Legal Holds | Retention engine over classes R1-R13, retention runs and dry runs, legal holds, deletion certificates; screen G7 | M | P1 | 18 | Planned |
| 24 | Governance, Registers, Command Centre & Packs | Validation packs, registers (decisions, exclusions, findings, warnings), fairness metrics, model inventory views; command centre (G1), registers (G4), access and insider controls (G5), audit viewer and chain verifier (G6), assessor workspace (G8), training register (G9), board and evidence pack builder (R4) | XL | P1 | 18-23 | Planned |
| 25 | Platform Growth & Releases | OIDC upstream, SIEM forwarding, quarterly access review, break-glass with second approver, release and update screen (A4), support sessions, ops cockpit v1 | M | P1 | 04 | Planned |
| 26 | Islamic Pack (optional) | Islamic deals, deal parties, possession evidence; deal-centric scenarios and mapping extensions | S | P1 | 10, 16 | Planned |
| 27 | Models & Model Governance | Models and versions, model scores, ONNX serving through `ort`, TreeSHAP, shadow then ranking-only promotion through `cr_material_validated`, drift and kill switch; screen D4 | L | P2 | 17, 24 | Planned |
| 28 | Assistant | Assistant drafts for case summaries and report narratives with sentence-level sources, evaluation gate, per-bank switch | M | P2 | 18, 21, 27 | Planned |
| 29 | Assessment API | `akr-assess` in the gateway zone: mTLS per gateway, `/assess/v1` pass/hold/reject, Ed25519-signed expiring decisions, fail policies and modes, holds, inline screening on the shared index | L | P2 | 14, 16, 20 | Planned |
| 30 | Profile M/L & High Availability | K3s with Helm charts, default-deny NetworkPolicies, automated PostgreSQL failover (Patroni or CloudNativePG by proof of concept), failover drills | M | P2 | 29 | Planned |
| 31 | Training, Capacity & Correspondent Packs | Training workspace with synthetic cases (W12), capacity workbench (G2), correspondent and validator packs, typology exposure index, RFIs to correspondents, SAML and SCIM through Ory Polis | L | P2 | 19, 24 | Planned |

P3 phases (further adapter packs, regime packs, Islamic pack updates, Kurdish interface, multi-installation support)
are written when P2 nears its exit, per docs/ROADMAP.md.

## Dependency Graph

```
P0
Phase 00 ──▶ Phase 01 ──▶ Phase 03 ──▶ Phase 04 ──▶ Phase 05 ──▶ Phase 06 (gap report, early piece 1)
   │            │            ▲   │                                   ▲
   └──▶ Phase 02 ────────────┘   └──▶ Phase 07 (goAML checker, early piece 2)
                │                                ▲                   │
                └──▶ Phase 08 (frontend) ────────┴───────────────────┘

P1                      ┌──▶ Phase 12 (recon)
Phase 04 ──▶ Phase 09 ──┤
Phase 05 ───────────────┴──▶ Phase 10 ──▶ Phase 11 ──┬──▶ Phase 13 ──┬──▶ Phase 16 ──┐
                                                    │              └──▶ Phase 15 ◀─┐ │
                                                    └──▶ Phase 14 ─────────────────┘ │
                                                              └──────────┬───────────┘
                                                                         ▼
                                       Phase 17 ──▶ Phase 18 ──┬──▶ Phase 19 ──┐
                                                               ├──▶ Phase 20 ──┼──▶ Phase 22
                                                               ├──▶ Phase 21 (also 07)
                                                               └──▶ Phase 23
                                                     Phases 18-23 ──▶ Phase 24
Phase 04 ──▶ Phase 25 (any time in P1)          Phases 10 + 16 ──▶ Phase 26 (optional)

P2
Phases 17 + 24 ──▶ Phase 27 ──▶ Phase 28 (also 18, 21)
Phases 14 + 16 + 20 ──▶ Phase 29 ──▶ Phase 30
Phases 19 + 24 ──▶ Phase 31
```

**Parallel tracks.** In P0, phase 02 runs beside phase 01; phase 08 starts once phase 01 lands; phase 07 needs only
phases 03 and 08, so **the goAML checker can ship before discovery is finished**. In P1, phases 12, 13 and 14 run in
parallel after phase 11; phases 19, 20, 21 and 23 run in parallel after phase 18; phase 25 can run at any point. The
parallel run of ROADMAP.md (P1 exit) is operational and has no phase file.

## New Entities by Phase

| Phase | New tables |
|-|-|
| 00 | `jobs`, `outbox`, `timers`, `idempotency_keys`, `settings`, `feature_flags`, `column_classification`, `retention_assignment` |
| 01 | `users`, `role_grants` (bootstrap rows only until 04), `delegations`, `pools`, `stepup_assertions`, `api_tokens`, `password_history` |
| 02 | `audit_log` (partitioned), `access_event` (partitioned), `audit_anchors`, `evidence_objects`, `packs` |
| 03 | `policy_bundles`, `branches` |
| 04 | `change_requests`, `inventory_items`, `workflow_definitions`, `workflow_definition_versions`, `workflow_instances`, `instance_events`, `tasks` |
| 05 | `sources`, `source_grant_checks`, `load_budgets`, `watermarks`, `batches`, `quarantine_items`, `discovery_profiles`, `field_availability` |
| 06 | `cbi_catalogue_items`, `scenarios` (register only), `scenario_catalogue_links`, `readiness_grades` |
| 07 | `goaml_profiles`, `goaml_watch_runs`, `goaml_check_runs`, `goaml_check_findings` |
| 08 | _(none — frontend only)_ |
| 09 | `countries`, `country_risk_overrides`, `governorates`, `currencies`, `fx_rates`, `calendars`, `calendar_days`, `reason_codes`, `policy_packs`, `authority_matrix` |
| 10 | `mappings`, `mapping_versions`, `code_maps`, `code_map_entries`, `drift_events`, `xref`, `attribute_definitions`, `attribute_values`; the `canon` tables of DATA-MODEL.md section 5 (party, names, identifiers, contacts, relationships, customer, account, roles, holds, balances, txn, txn events, counterparty, product, payment messages, cards, cash events, trade, FX, facility, locker, branch, channel, staff, CBS user activity) |
| 11 | `raw.<source_table>` (per mapped table, partitioned by extraction day), `blind_periods` |
| 12 | `recon_runs`, `recon_results`, `coverage_ledger`, `coverage_certificates`, `exclusions`, `dq_issues` |
| 13 | `feature_definitions`, `feature_values`, `segments`, `segment_stats`, `peer_groups`, `profiles` |
| 14 | `ref.lists`, `ref.list_versions`, `ref.list_entries`, `ref.list_entry_names`, `alert.screening_runs`, `alert.screening_alerts`, `alert.screening_decisions`, `alert.pair_memory`, `alert.good_guy_rules`, `alert.harness_runs` |
| 15 | `rating_models`, `ratings`, `overrides`, `rating_feed`, `kyc_reviews`, `kyc_document`, `pep_register`, `pep_relatives`, `ownership_results` |
| 16 | `scenario_versions`, `parameter_sets`, `scenario_runs`, `replays`, `replay_results`, `shadow_results`, `seeded_tests` |
| 17 | `alert.alerts`, `alert.quarantine`, `casework.cases`, `casework.case_events`, `casework.case_alerts`, `casework.case_parties` |
| 18 | `casework.case_acl`, `casework.decisions`, `casework.notes`, `casework.work_items`, `casework.evidence_items`, `casework.entity_links` |
| 19 | `casework.rfis`, `casework.rfi_questions`, `casework.rfi_answers`, `casework.internal_reports`, `casework.authority_requests`, `casework.watches`, `casework.hibernations`, `casework.network_snapshots`, `casework.qa_samples`, `casework.qa_reviews`, `casework.qa_findings` |
| 20 | `ops_tasks`, `ops_verifications`, `freezes` |
| 21 | `reports`, `report_versions`, `report_packages`, `filings`, `fiu_feedback`, `ctr_batches`, `ctr_items`, `letters`, `periodic_reports` |
| 22 | `observations`, `confidential_reports`, `branch_inputs`, `liaison_monthly_reports`, `notifications`, `notification_templates`, `user_notification_prefs` |
| 23 | `legal_holds`, `deletion_certificates`, `retention_runs` |
| 24 | `validation_packs`, `registers`, `fairness_metrics` |
| 25 | _(none expected — settings, audit and existing tables; confirmed when the phase file is written)_ |
| 26 | `islamic_deals`, `deal_parties`, `possession_evidence` |
| 27 | `models`, `model_versions`, `model_scores` |
| 28 | `assistant_drafts` |
| 29 | `assess.fail_policies`, `assess.decisions`, `assess.holds` |
| 30 | _(none — infrastructure)_ |
| 31 | _(confirmed when the phase file is written; training and capacity data reuse cases and settings where possible)_ |

Reconciles with DATA-MODEL.md (sections 1-20). Where DATA-MODEL.md names a table this list misses, the gap pass
records it against the owning phase.

## New Services by Phase

| Phase | New domain services |
|-|-|
| 00 | `SettingsService`, `IdempotencyService`, `JobQueue` and worker harness, `OutboxDispatcher`, `TimerService` |
| 01 | `IdentityService` (Kratos/Hydra facade), `SessionService` (BFF), `StepUpService`, `TokenService`; supporting `ClientCredentials` (Hydra JWT), `PasswordPolicy`, `RateLimiter` |
| 02 | `AuditWriter`, `AnchorService`, `EvidenceStore`, `RenderService` (Gotenberg) |
| 03 | `AuthzService` (Cedar facade and attribute cache), `MaskingLayer`, `UserAdminService`, `OffboardingService` |
| 04 | `ChangeRequestService` (class, volume effect, route), `WorkflowEngine`, `DslValidator` (workflow), `TaskProjector`, `InboxService` |
| 05 | `SourceRegistry`, `ConnectionGuard` (grant check, SELECT proof, budget), `SourceReader` adapters, `DiscoveryProfiler` |
| 06 | `CatalogueService`, `ReadinessGrader`, `GapReportService` |
| 07 | `GoamlProfileRegistry`, `GoamlValidator`, `QualityGrader`, `SchemaWatcher` |
| 08 | _(frontend: shell, i18n kit, shared components)_ |
| 09 | `ReferenceService`, `CalendarService`, `PolicyPackService`, `AuthorityMatrixService` |
| 10 | `MappingService`, `MappingCompiler`, `CodeMapService`, `DriftDetector`, `CanonicalWriter` |
| 11 | `IngestScheduler`, `WatermarkService`, `FreshnessService`, `RestoreDetector` |
| 12 | `ReconService`, `CoverageService`, `DataQualityService` |
| 13 | `FeatureEngine`, `SegmentService`, `ProfileService` |
| 14 | `NameNormaliser`, `ListService`, `ScreeningIndex`, `Scorer`, `ScreeningService`, `KnownAnswerHarness` |
| 15 | `RatingEngine`, `KycReviewService`, `PepService`, `OwnershipService` |
| 16 | `ScenarioValidator`, `ScenarioCompiler`, `ScenarioRunner`, `ReplayService`, `ShadowService` |
| 17 | `AlertService`, `CaseMerger`, `PriorityClassifier`, `QueueService` |
| 18 | `CaseService`, `DecisionService`, `GenericTextGuard`, `EvidenceService` |
| 19 | `RfiService`, `WatchService`, `NetworkService`, `QaService` |
| 20 | `OpsTaskService`, `VerificationService`, `FreezeService` |
| 21 | `ReportService`, `GoamlPackager`, `FilingService`, `CtrService`, `PeriodicReportService` |
| 22 | `BranchService`, `ObservationService`, `NotificationDispatcher`, `SseHub` |
| 23 | `RetentionEngine`, `LegalHoldService` |
| 24 | `RegisterService`, `CommandCentreService`, `PackBuilder`, `AssessorService` |
| 25 | `OidcUpstream`, `SiemForwarder`, `AccessReviewService`, `BreakGlassService`, `ReleaseService` |
| 26 | `IslamicDealService` |
| 27 | `ModelRegistry`, `ModelScorer`, `ExplanationService` |
| 28 | `AssistantService`, `EvaluationService` |
| 29 | `AssessmentService`, `FailPolicyService`, `DecisionSigner` |
| 30 | _(infrastructure)_ |
| 31 | `TrainingService`, `CapacityService`, `CorrespondentPackService` |

## Migration Chain (canonical)

File numbers are assigned in phase order as the phase files are written (DR-23); DATA-MODEL.md section 23 keeps the
group order. An applied migration is never edited.

| Phase | Files (canonical) | Group (DATA-MODEL.md s.23) |
|-|-|-|
| 00 | `0001_platform_schemas_roles`, `0002_platform_jobs_outbox_timers`, `0003_platform_idempotency_settings_flags`, `0004_platform_classification_retention` | 1 |
| 01 | `0005_identity_principals_grants`, `0006_identity_stepup_tokens_passwords`, `0007_identity_settings` | 2 |
| 02 | next numbers | 3 |
| 03 | next numbers | 4 |
| 04 | next numbers | 5 |
| 05 | next numbers | 6 |
| 06 | next numbers | 7 |
| 07 | next numbers | 8 |
| 09 | next numbers | 9 |
| 10 | next numbers | 10 |
| 11 | next numbers | 11 |
| 12 | next numbers | 12 |
| 13 | next numbers | 13 |
| 14 | next numbers | 14 |
| 15 | next numbers | 15 |
| 16 | next numbers | 16 |
| 17-19 | next numbers, split by phase | 17 |
| 20 | next numbers | 18 |
| 21 | next numbers | 19 |
| 22 | next numbers | 20 |
| 23 | next numbers | 21 |
| 24 | next numbers | 22 |
| 26 | next numbers | 23 |
| 27-28 | next numbers | 24 |
| 29 | next numbers | 25 |

## Gap Analysis Additions

_Running log, updated after each pass. No passes run yet (phase files not yet written)._

| Pass | Date | Gaps | CRITICAL | HIGH | MEDIUM | LOW | Notes |
|-|-|-|-|-|-|-|-|
| 0 (authoring) | 2026-10-07 | 1 authority-doc delta | 0 | 1 | 0 | 0 | DATA-MODEL.md lacked the workflow engine tables that DSL.md Part E and API.md 3.21 use; added with DBS names, `casework.tasks` renamed `casework.work_items` (DR-08) |
| 0 (phase 00 authoring) | 2026-10-07 | 3 authority-doc deltas | 1 | 0 | 1 | 1 | CRITICAL: the schema `case` and five columns were PostgreSQL reserved words and would not migrate (renamed, DR-20). MEDIUM: platform table shapes updated from DBS lessons (idempotency ledger DR-21; settings value type, governed flag, label and version; jobs `finished_at`; outbox back-off columns; timer `created_at`; classification and retention columns). LOW: API.md problem codes gained `E_INTERNAL` and `E_PAYLOAD_TOO_LARGE`. Also DR-22 to DR-26 (roles, migration numbering, support binaries, ingestd reads OpenBao, collation) |
| 0 (phase 01 authoring) | 2026-10-07 | 6 authority-doc deltas | 0 | 1 | 3 | 2 | HIGH: no organisational branch table existed in P0 although user, grant and pool branch scopes need one (`admin.branches` in phase 03, DR-27). MEDIUM: API.md 3.1 lacked the login, second-factor, heartbeat, credential and undertaking routes (22 routes now); the PAT hash moved from argon2id to HMAC-SHA-256 and secret-carrying routes left the idempotency ledger (DR-28); the delegation reason became a code, not free text (tipping-off). LOW: three problem codes (`E_CSRF_INVALID`, `E_CREDENTIAL_CHANGE_REQUIRED`, `E_MIN_FACTORS`); the bootstrap CLI moved from phase 03 to 01 (DR-33). Also DR-29 to DR-32 and DR-34 |
