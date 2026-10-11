# AKR-TMS Implementation — Status

Living tracker for the 32-phase build (phases 00-31, macro-phases P0-P2; P3 phases written later). Updated as phases
move Planned, Spec ready, In Progress, Done. Counts are plan-time targets until a phase is implemented.

Planning state on 2026-10-07: the eight system docs are written (docs/README.md); the phase table, dependency graph,
entities, services and migration chain are in roadmap.md. Phase files: 00 and 01 written 2026-10-07, 02, 03, 04, 05, 06, 07 and 08 on 2026-10-09. The owner chose (2026-10-07) that the
lead writes P0 (00-08) inline, one by one, then asks again before 09-31. For 09-31 the owner chose (2026-10-09) Haiku agent
drafts in batches, each reviewed, corrected and aligned by the lead; phases 09 to 31 (batches B1 to B8; phases 19, 20, 21 and 23 are B6, phases 22, 24, 25 and 26 are B7, phases 27 to 31 are B8) reviewed 2026-10-09.

## Phase Status

| # | Phase | Status | Started | Completed | Tables + | Tables ~ | Routes + | Services + |
|-|-|-|-|-|-|-|-|-|
| 00 | Platform Foundation | Spec ready | | | 8 | 0 | 0 (infra only) | 5 + crypto, store and S3 guard ports |
| 01 | Identity & Sessions | Spec ready | | | 7 | 3 (idempotency_keys, settings, feature_flags FKs) | 22 | 4 + RateLimiter, PasswordPolicy, ClientCredentials |
| 02 | Audit Trail & Evidence Core | Spec ready | | | 5 | 0 | 0 (no business routes) | 8 (PgAuditSink, PgAccessLog, AnchorService, Verifier, PartitionMaintenance, EvidenceStore, RenderService, PackService) |
| 03 | Authorisation, Users & Roles | Spec ready | | | 4 | 1 (users.enabled_at, branch FK) | 32 | 11 (AuthzService, MaskingLayer, Simulator, AccessMatrixBuilder, UserAdminService, CredentialResetService, OffboardingService, BranchService, PoolAdminService, DelegationService, OpsCockpit) |
| 04 | Change Control & Approval Workflows | Spec ready | | | 8 | 2 (role_grants and policy_bundles FKs) | 32 | 10 (ChangeRequestService, RouteSelector, ChangeTargetRegistry, WorkflowEngine, TaskProjector, DslValidator, DefinitionService, InboxService, InventoryService, ChangeTimers) |
| 05 | Sources, Read-only Connection & Discovery | Spec ready | | | 8 | 0 (route-map setting row only) | 21 | 11 (SourceRegistry, ConnectionGuard, GrantPolicy, SelectProof, LoadGovernor, SourceReader adapters, DiscoveryRunService, DiscoveryProfiler, FieldAvailabilityGrader, IntegrationProfileBuilder, change targets load_budget and source_access) |
| 06 | CBI Catalogue, Readiness Grades & Gap Report | Spec ready | | | 5 | 1 (admin.packs kind CHECK) | 14 | 8 (CatalogueInstaller, CatalogueQuery, ReadinessGrader, CoverageGradeJob, DeclarationService, GapReportService, PackRequestService, FieldCandidateFinder) |
| 07 | goAML Checker | Spec ready | | | 5 (the fifth is `rpt.goaml_corpus_cases`, DR-415) | 0 | 12 (9 of section 2, 3 of section 7.6) | 15 (ProfileLoader, PatchApplier, SchemaDiff, PreScreen, SandboxedXsdValidator, RuleCatalogue, Scorer, RejectionParser, CheckService, SchemaWatcher, CorpusRunner, change target goaml_profile; CorpusCaseService, CaseMasker and change target goaml_corpus_case of section 7.6) |
| 08 | Frontend Foundation | Spec ready | | | 0 | 0 | 0 (two read scopes widened in 05 and 06) | frontend: kit, component layer, catalogue, shell, 11 screens; `akr-ctl policy matrix` |
| 09 | Reference Data, Calendars & Policy Pack | Spec ready | | | 10 (8 `ref` incl. `cbs_calendar_captures`, `policy_packs`, `authority_matrix`) | 2 (`timers.calendar_id` FK, `reason_codes` grants and `template` class) | 21 | 4 + FxRateService, CalendarReconciler, BankCalendarClock, PolicyValidator, ForbiddenWordScanner, PolicyDryRun, AuthorityDelegation, 6 change targets |
| 10 | Mapping & Canonical Model | Spec ready | | | 38 (9 `admin` incl. `mapping_runs`, 29 `canon`) | 0 (phase 05 `SelectProof` moves to crate `sqlproof`) | 20 | 5 + MappingSampler, MappingExecutor, CanonicalCatalogue, ViewDdlBuilder, AttributeService, 2 change targets |
| 11 | Production Ingestion & Freshness | Spec ready | | | 5 (`watermarks`, `batches`, `quarantine_items` from 05 per DR-63, `blind_periods`, `mapping_replays`) + runtime `raw` landing tables | 1 (`sources`: CT3, `delivery`, `inbox_key`) | 12 (+ resume extended) | 14 (IngestScheduler, UnitPlanner, LoadUnitRunner, LandingWriter, ChangeDetector, WatermarkService, RestoreDetector, ResumeHandler, BlindPeriodService, FreshnessService, ReplayService, ReplayRunner, FileScanner, ChangeCaptureConsumer) |
| 12 | Reconciliation, Coverage & Data Quality | Spec ready | | | 6 | 0 (phase 11 quarantine list and exclude extended) | 16 | 8 (ReconService, RungEvaluator, ControlTotalsReader, CoverageService, CertificateBuilder, DataQualityService, DataQualityRecorder, 2 change targets) |
| 13 | Features & Segments | Spec ready | | | 11 (8 of section 1 and 3 of section 7+: `counterparty_links`, `source_tags`, `customer_source_tags`) | 0 (phase 02 partition registry, phase 04 change targets, phase 06 attribute catalogue extended) | 13 (11 of section 2, 2 of section 7.17) | 7 (FeatureValidator, FeatureCompiler, FeatureEngine, SegmentService, PeerBuilder, ProfileService, SourceTagService of section 7.17, 3 ports, 4 change targets with `source_tag`) |
| 14 | Screening Engine & Lists | Spec ready | | | 11 (the eleventh is `alert.harness_cases`, DR-415) | 0 (phase 11 emits `canon.touched`; phase 09 reason codes, phase 03 masked kinds, phase 04 change targets extended) | 27 (25 of section 2.8, 2 of section 7.19) | 13 (names crate, ListService, ListFetchService, BankListService, ScreeningIndexRegistry, Scorer, ScreeningService, AlertService, PairMemoryService, GoodGuyService, KnownAnswerHarness, FreezeTrigger port, HarnessCaseService of section 7.19, 4 change targets with `harness_case`) |
| 15 | Risk Rating & KYC | Spec ready | | | 16 (+ view `kyc.party_pep_state`) | 1 (`admin.users.staff_id`; phase 03 customer adapter and masked kinds, phase 04 targets and three shipped workflows, phase 06 rating attributes, phase 14 pair-memory cap extended) | 34 | 14 (RatingEngine, RatingService, RatingFanout, RatingModelService, RatingShadowService, OverrideService, KycReviewService, ReviewClock, PepService, PepRules, OwnershipService, OwnershipCalculator, RatingFeedExporter, RelationshipConflictCheck; 3 ports implemented, 3 change targets) |
| 16 | Scenario Engine, Packs & Studio | Spec ready | | | 11 | indexes on `canon.txn` and `canon.txn_event` (batch id); phase 06 `GET /scenarios` and `GET /scenarios/{id}` extended; phase 13 compiler entry points public; phase 04 three change targets and evidence resolvers | 23 (+2 extended) | 20 (ScenarioValidator, ScenarioCompiler, ScenarioVersionService, GoLiveGate, ParameterService, ScenarioFanout, ScenarioRunner, RecordingAlertSink, LinkedPartyResolver, ReplayService, SampleBuilder, ShadowService, SeededSuite, KillSwitchService, ScenarioReaderCatalogue, CatalogueImportService, ScenarioAdminService, ScenarioCardRenderer, ScenarioMetrics, CoverageMatrixQuery) |
| 17 | Alerts, Triage & Queues | Spec ready | | | 12 | `alert.screening_alerts` (FK on `case_id`); phase 16 runner handles `AlertSinkError` and the port comment names `AlertIntakeService`; phase 01 `default_queue_id` rule lifted; phase 02 owner kind `alert` and the `case_event` anchor payload (gap pass 1); phase 14 calls `ScreeningCaseLinker` (gap pass 1) | 24 (+1 dev) | 13 (AlertIntakeService, AlertValidator, QuarantineService, PriorityClassifier, AlertScorer, CaseAttachPolicy, CaseMerger, CaseChain, AssignmentService, QueueService, CountsHub, AlertClockHandler, VisibilityBuilder) |
| 18 | Cases, Decisions & Investigation | Spec ready | | | 9 | `casework.cases` (`escalation_level`, outcome CHECKs), `alert.alerts` (`contribution_inherited`, CHECK); phase 04 `WorkflowSubject::check_before_decision` and `Holders` rule 5 (`case_acl`, role M for reports); phase 17 `AssignmentService` writes grants, visibility clause; phase 02 owner kind `case`; phase 15 `customer` tier via `effective_tier` (gap pass 1) | 28 | 15 (CaseAggregate, CaseService, CaseSealer, CaseAccessService, NoteService, GenericTextGuard, EvidenceService, EntityLinkService, EntityMask, WorkItemService, DecisionService, ViewGate, ShelvedRegister, PartyService, PartyTransactionService) |
| 19 | RFIs, Watches, Network & QA | Spec ready | | | 15 (the fifteenth is `casework.rfi_prefill_map`, DR-417) | `casework.case_acl` reason CHECK gains `qa_review`, `casework.case_families` row `network`; phase 18 `RecordDecisionEffect` and escalation write outbox `case.decision`; phase 09 `qa.review` gains CH, IA; phase 17 intake step 4b and `attach_released`, phase 18 close gate and reopen hook, phase 02 owner kind `network_snapshot`, phase 03 `MASKED_KINDS` `rfi`, DSL E.4 rows (gap pass 1) | 35 (three of them from section 7.21) | 19 (RfiService, RfiWorkflowSubject, ReleaseRfiEffect, InternalReportService, AuthorityRequestService, WatchService, WatchTriggerDispatcher, HibernationService, HibernationGate, HibernationHitService, NetworkService, EdgeBuilder, NetworkSnapshotSealer, QaService, QaSampler, BreachService, AlarmService, FlatLineDetector; RfiPrefillService and change target `rfi_prefill_map` of section 7.21) |
| 20 | Operations Tasks & Freezes | Spec ready | | | 5 (+ sequence `casework.ops_ref_seq`) | `casework.alarms` kind and subject CHECKs widened; phase 14 `RecordingFreezeTrigger` replaced by `FreezeService` in AppState; phase 09 ClockRegistry four fixed entries, phase 02 owner kind `ops_task`, DSL E.3/E.4 rows (gap pass 1); API.md `POST /exits` superseded by phase 15 | 22 | 12 (OpsTaskService, VerificationService, FreezeService, RefrainService, IncidentService, CanonOpsReader, FreezeWorkflowSubject, OpsTaskWorkflowSubject, LetterRequester, FreezeClockHandlers, RefrainClockHandler, FreezeBackfill) |
| 21 | Reporting to the Office | Spec ready | | | 12 (+ sequences `rpt.report_ref_seq`, `rpt.letter_no_seq`) | `casework.alarms` kind and subject CHECKs widened; phase 20 `RecordingLetterRequester` replaced by `LetterService` in AppState; phase 18 `release_for_filing` body replaced, close gate reads the acknowledgement (`filing_not_acknowledged`), report sign route refusal `report_sign_required`; phase 04 `ReportWorkflowSubject` and `release_correction`, phase 07 rule codes and L0, phase 09 ClockRegistry two report entries, phase 02 owner kinds (gap pass 1); API.md liaison review rows moved to phase 22 | 32 | 18 (ReportService, ReportApprovalService, ReportWorkflowSubject, ReleaseForFilingEffect, ReportCorrectionEffect, PackageService, DownloadService, FilingService, RejectionService, FeedbackService, CtrService, LetterService, LetterBackfill, PeriodicReportService, AnalyticsService, ReportClockHandlers, ReportQuery, GoamlPackager) |
| 22 | Branch Window & Notifications | Spec ready | | | 6 (`branch_inputs` and `confidential_reports` dropped for phase 19 claims and internal reports, DR-201, DR-202) | `casework.alarms` kind and subject CHECKs; phase 00 outbox registry becomes a handler list per topic; phase 19 `record_answers_in_tx` and owner kind `rfi_input`; phase 03 one branch per liaison, `UserRoleQuery`; phase 04 `notify` effect and `notification_template` change target (gap pass 1) | 17 | 13 (BranchWindowService, BranchStatusMapper, ObservationService, HybridObservationJob, LiaisonReportService, NotificationDispatcher, NotificationService, TemplateService, NotifyEffect, NotificationTemplateTarget, SmtpRelaySender, RfiBranchRepository, UserRoleQuery) |
| 23 | Retention & Legal Holds | Spec ready | | | 7 (+ sequence `admin.hold_ref_seq`) | `admin.evidence_objects.object_deleted_at`; new role `akr_retention` (phase 00 roles and provisioning); phase 18 seal enqueues `retention_lock_extend`; phase 04 `legal_hold_release`, `release_legal_hold`, `LegalHoldWorkflowSubject`; phase 10 and 11 canonical writers read the tombstones; phase 03 bundle gains `61_retention.cedar` (gap pass 1) | 9 | 15 (RetentionEngine, DisposalRegistry, RelationshipEndResolver, LegalHoldService, LegalHoldWorkflowSubject, ReleaseLegalHoldEffect, RetentionReportService, CertificateService, CertificateSigner, WormWriter, ReplayService, EvidenceReconciler, LockExtensionService, CaseLookup, EvidenceShredder) |
| 24 | Governance, Registers, Command Centre & Packs | Spec ready | | | 7 (models, scores and assistant drafts stay P2) | `admin.packs` kind `certification`; functions `audit.tier_rank`, `audit.resource_tier`; phase 06 pack routes accept `assessor.read` and download step-up `export`; phase 00 AppRole akr-api KV `db/akr_readonly`; phase 02 evidence owner kinds `validation_pack`, `training_record`; phase 03 MASKED_KINDS; phase 04 ChangeTarget `protected_attribute`; phase 16 `check_before_decision` calls `ProtectedAttributeCheck` (gap pass 1) | 36 (+ 3 phase 06 pack routes extended) | 10 (AuditViewerService, AccessHistoryService, GovernanceRegisterService, FairnessService, ValidationPackService, CommandCentreService, ClockBoardService, AssessorService, TrainingRegisterService, ProtectedAttributeCheck) |
| 25 | Platform Growth & Releases | Spec ready | | | 11 | `admin.role_grants` (`support_session_id`, source `support_session`); functions `audit.siem_page` and `audit.siem_access_page` (DR-414); phase 04 `role_grant` op `revoke` and target `platform_release`; phase 22 `NoticeCode` 15 to 17; phase 24 control report gains two counts; phase 00 zones `z2_bank`, `z3_idp`; phase 03 `OpsCockpit` tiles; phase 17/18 case read path writes `break_glass.used` (gap pass 1) | 26 (12 owned, 14 added; four by sections 7.23 and 7.24) | 9 (AccessReviewService, BreakGlassService, ReleaseService, SupportSessionService, HelpCentreService, SiemForwarder, OidcUpstream, AnchorTimestamper; SiemDeadLetterService of section 7.24) |
| 26 | Islamic Pack (optional) | Spec ready | | | 4 (the fourth is `canon.facility_instalments`, DR-419) | `canon.product.islamic_contract_type` CHECK (+4 values); phase 20 `FreezeService::start` groups by account class; phase 16 pack seed and G9 read `pack: islamic`; phase 03 flag `pack.islamic` refused off while a version is live; phase 10 `CanonicalEntity` +2; phase 02 owner kind `islamic_deal` (gap pass 1); `canon.facility.granted_on` (DR-419) | 6 (4 owned, 2 added) | 3 (IslamicDealService, PossessionEvidenceService, ShariaAggregateQuery) |
| 27 | Models & Model Governance | Spec ready | | | 6 | phase 24 `registers.source` (+`model_monitor`), `fairness_metrics.metric` (+`score_top_band_rate`), closure codes (+5) and `fairness_monthly`; phase 17 queue ORDER BY gains the model rank; phase 04 change targets `model_version` and `model_switch`; phase 25 release import sets `registered` only; phase 00 volume `model-store` and key purpose `keys/model-artifact`; phase 02 owner kind `model_version` (gap pass 1) | 16 (6 owned, 10 added) | 10 (ModelRegistry, ModelImportVerifier, ModelScorer, ExplanationService, ModelMonitor, ModelLabelGateService, ModelPromotion, ModelSwitch, ModelKillSwitchService, ModelReads) |
| 28 | Assistant | Spec ready | | | 4 | phase 19 weekly draw of `assistant_assisted`, source kind `assistant_draft` and the QA item view of an evaluation draft; phase 18 `RecordDecisionEffect` call for `ai_involvement` and the W3 `assistant_summary` affordance; phase 21 `ReportService::update_draft`; phase 23 purge of drafts before the case row and owner kind; phase 24 review-calendar row `assistant_kill_drill` and the `ValidationPackService` assistant branch; phase 25 SIEM and support-bundle exclusions and release import `registered` only; phase 03 flag service refuses `pack.assistant` off while a version is live; phase 00 zone `z2_model`, volumes and the `llama` and `llama-eval` images (gap pass 1) | 12 (3 owned, 9 added) | 9 (AssistantService, AssistantReads, AssistantKillSwitchService, AssistantVersionService, AssistantGenerator, AssistantMonitor, GroundingVerifier, PromptBuilder, AssistantEvaluationService) |
| 29 | Assessment API | Spec ready | | | 19 | phase 24 `registers.source` CHECK widened by `assess_control`; phase 14 `ScreeningService::open_hold_alert`, the hold-alert subject view from `assess.holds.party_names_enc`, `screening_payment` skipping an entry already alerted on a hold, and `list.feed_unchanged` renewing freshness; phase 02 owner kind `assess_extract` and the PartitionRegistry entry; phase 03 `Ctx.via` value `gateway_cert`; phase 23 decision partitions dropped only after their holds; phase 24 review-calendar row `assess_control_test` and finding closure closing release exceptions; phase 25 SIEM and support-bundle exclusions; phase 00 `akr-assess` in `z1_bao`, `akr-sign-decision` reserved and unused, AppRole transit decrypt on `akr-kek` (gap pass 1) | 18 (12 owned, 2 added, 4 gateway) | 17 (AssessmentService, MessageParser, PolicyCache, DecisionSigner, GatewayAuthenticator, GatewayRateLimiter, HoldService, HoldSweepService, FailPolicyService, RailModeService, ProhibitedRuleService, ReleaseControlService, ControlTestService, RollupService, OutageService, FallbackExtractService, AssessReadService) |
| 30 | Profile M/L & High Availability | Spec ready | | | 0 | phase 25 `admin.releases.refusal_code` CHECK widened by `chart_signature_invalid` (0095, with five settings); phase 00 job harness (lease heartbeat, cluster-wide concurrency, database-clock slots), listener reload on reconnect, `akr-assess` readiness without Valkey; phase 02 owner kind `ops_drill`; phase 16 `cost` keys | 0 | 7 (DeployRenderer, ClusterZoneCheck, RestoreVerifier, FailoverDrillRecorder, HaProbe, StandbyPool, LimiterFallback) |
| 31 | Training, Capacity & Correspondent Packs | Spec ready | | | 9 (and the view `train.practice_scores`) | phase 19 `casework.alarms` kind and subject CHECKs widened (`capacity_forecast`, `pool`); phase 03 `UserAdminService::disable_from_directory`, `user.disabled` key `source`, the enable check `directory_inactive`, `Ctx.via` `webhook_hmac`; phase 19 `RfiService` and the QA sampler surge rate; phase 24 register practice column; phase 06 pack builders; phase 00 zones and the edge exclusion of `/api/v1/directory/events` | 15 (13 added, 2 owned) | 9 (TrainingService, CapacityService, CapacitySnapshotter, CapacityWatcher, SimulationHarness, ExposureService, CorrespondentPackService, DirectoryService, DirectoryWebhookVerifier) |

Status values: Planned | Spec ready | In Progress | Done | Blocked.

> "Tables ~" = tables a phase modifies (ALTER, new columns, new indexes) that it does not create. Route counts are
> filled in from each phase's route table, which is normative.

## Cumulative Totals

| Metric | Before | Current | Target |
|-|-|-|-|
| Domain tables (specified) | 0 | 284 (the sum of the Tables + column: 277 before the owner decisions of 2026-10-10, six tables added by them, and `feat.counterparty_links` of gap pass 1, which the column had not counted) | about 180 (DATA-MODEL.md) |
| Routes | 0 | 595 (the sum of the Routes + column; it equals the 595 rows of API.md: 581 after the verification pass and 14 added by the owner decisions of 2026-10-10) | about 366 (API.md at the start of planning; it lists 595 rows now) |
| Phases specified | 0 | 32 | 32 |
| Screens | 0 | 62 (X1, X3, X4 security, X6, G10, A1-A3, T1, E1, E2, D3, T2, T3, T4, S1, S2, S3, S4, K1, K2, K3, K4, D1, D2, W1, W2, W3, W4, W5, W7, W6, W8, W10, W11, G3, W9, S5, R1, R2, R3, G7, B1, B2, B3, B4, B5, B6, R5, G1, G4, G5, G6, G8, G9, R4, A4, X5, I1 (proposed by phase 26; not yet in DESIGN-SYSTEM 8.1), D4, W12, G2) | 62 (DESIGN-SYSTEM.md 8.1) |
| Research requirement IDs carried by a phase | 0 | 600 (some partial; `python3 tools/idcount.py 31`, phases 00-31: header lists, ranges expanded, "Not carried" lines excluded; the earlier 295 did not expand ranges) | 1,208 (coverage baseline) |

## Jobs by Phase

_Filled in from each phase's Section 4._

| Job kind | Phase | Schedule / trigger |
|-|-|-|
| `maintenance_sweep` | 00 | Daily, `schedule.maintenance_sweep` (02:30 bank time) |
| `backup_check` | 00 | Daily, `schedule.backup_check` (06:00 bank time) |
| `identity_sweep` | 01 | Daily, `schedule.identity_sweep` (03:00 bank time): dormancy and account expiry |
| `anchor_audit` | 02 | Every 5 minutes, `schedule.anchor_audit`: signed Merkle anchors to `akr-anchors` |
| `partition_maintenance` | 02 | Daily, `schedule.partition_maintenance` (01:15): partitions ahead for registered tables |
| `audit_verify` | 02 | Weekly, `schedule.audit_verify` (Saturday 04:00): chain, anchors, signatures, WORM copies |
| `audit_lock_extend` | 02 | Monthly, `schedule.audit_lock_extend`: extends anchor object locks |
| `evidence_scan` | 02 | On demand, enqueued by `EvidenceStore::stage`: antivirus, seal, lock |
| `ops_probe` | 03 | Every minute, `schedule.ops_probe`: seal state, disk, certificates, backups, exporters snapshot to Valkey for the cockpit |
| `offboard_finalize` | 03 | On demand, enqueued by `OffboardingService` when Kratos or Hydra was unreachable |
| `identity_sweep` (extended) | 03 | Same schedule: also expires credential resets, checks branch consistency, dormancy from `enabled_at` |
| `timer_cr_activate` | 04 | Timer `cr_activate` at a change request's `effective_at` |
| `timer_cr_emergency_review` | 04 | Timer `cr_emergency_review` at `review_due_at`: revert a loosening change, escalate others (DR-55) |
| `timer_cr_post_review` | 04 | Timer `cr_post_review` at 30 and 90 days after a material change: starts `cr_post_review` |
| `timer_task_timeout` | 04 | Timer `task_timeout` at a task's `due_at`: remind, escalate or reassign |
| `change_sweep` | 04 | Hourly at minute 15, `schedule.change_sweep`: expire drafts, release claims of ineligible holders, park and unpark tasks |
| `discovery_probe` | 05 | On demand (akr-ingestd), enqueued by probe and resume: connect and grant check |
| `discovery_run` | 05 | On demand (akr-ingestd), enqueued by `POST /discovery/runs`: catalogue or profile run, one per source |
| `ingest_source_health` | 05 | Every 15 minutes (akr-ingestd), `schedule.source_health`: grant check, business date, paused-by-bank back-off |
| `discovery_seal_report` | 05 | On demand (akr-worker), after a run: seals the run report as evidence; from phase 06 enqueues `coverage_grade` after a profile run |
| `coverage_grade` | 06 | On demand, coalesced: after `packs seed`, a profile run, a change of `branch.observations_live` or `pack.islamic`, or of `coverage.maintained_fields` |
| `gap_report_build` | 06 | On demand, `POST /coverage/cbi-gap-report/runs`: regrade, sealed JSON, four PDF packs |
| `pack_build` | 06 | On demand, `POST /packs`: integration profile PDFs and rebuilds of `cbi_gap` and `vendor_request` |
| `coverage_sweep` | 06 | Daily, `schedule.coverage_sweep` (02:40 bank time): expires declarations past their review date |
| `goaml_check` | 07 | On demand, `POST /goaml/check`: pre-screen, sandboxed XSD, rules, estimated grade; deletes the uploaded files |
| `goaml_watch` | 07 | Daily, `schedule.goaml_watch` (05:10 bank time) only while `goaml.watch_enabled`; manual `POST /goaml/watch-runs` |
| `goaml_corpus` | 07 | On demand: `packs seed` and every new profile candidate (watcher, import, amendment) |
| `fx_rate_import` | 09 | Sunday to Thursday 09:30, `schedule.fx_rate_import`; completes `skipped` while `fx.cbi_feed_enabled` is off |
| `ref_expiry_sweep` | 09 | Daily 00:10, `schedule.ref_expiry_sweep`: ends expired country overrides, calendar horizon gauge |
| calendar capture (step of `discovery_run`) | 09 | akr-ingestd, inside each phase 05 discovery run |
| `ingest_mapping_sample` | 10 | akr-ingestd, on demand: mapping test and preview runs (one attempt) |
| `mapping_compile` | 10 | On demand: validate fallback and recompile after drift |
| `drift_scan` | 10 | Enqueued by phase 05 `discovery_seal_report` after each sealed profile run |
| `code_map_prefill` | 10 | On demand: `POST /code-maps` with `prefill: true` (never currency) |
| `mapping_preview_purge` | 10 | Daily 01:50, `schedule.mapping_preview_purge` |
| `ingest_tick` | 11 | Every minute, `schedule.ingest_tick`: due tables, partitions ahead, pending mapping |
| `ingest_load` | 11 | Enqueued by the tick per (source, table); reschedules itself at the next window |
| `ingest_file_scan` | 11 | Every 30 seconds, `schedule.ingest_file_scan` (CT4 files, flag `ingest.files_enabled`) |
| `ingest_map_pending` | 11 | The tick (landed receipts after a publication); the quarantine retry route |
| `ingest_freshness` | 11 | Every 5 minutes, `schedule.ingest_freshness`: stale states, blind-period closes |
| `ingest_resume` | 11 | The resume route of a halted source |
| `ingest_mapping_replay`, `ingest_mapping_replay_apply` | 11 | The replay and apply routes |
| `recon_sweep` | 12 | Daily 03:20, `schedule.recon_daily`: day runs for late or unreconciled dates |
| `recon_run` | 12 | `POST /recon/runs`, the sweep, and `ingest_control_totals` on completion |
| `ingest_control_totals` | 12 | `recon_run` (source counts and grouped sums, akr-ingestd) |
| `coverage_ledger_refresh` | 12 | `recon_run` completion, exclusion activation, expiry sweep |
| `coverage_certificate` | 12 | `recon_run` completion of a day run |
| `dq_expiry_sweep` | 12 | Daily 03:05, `schedule.dq_expiry_sweep`: known adjustments and exclusions |
| `feature_compute` | 13 | Hourly at :10, `schedule.feature_compute`; a `feature_definition` activation; a phase 11 replay into a closed month |
| `segment_assign` | 13 | Daily 02:40, `schedule.segment_assign`; a `segment` or `peer_build` activation |
| `segment_stats_refresh` | 13 | Day 2 of the month 03:30, `schedule.segment_stats_refresh`; a membership change |
| `peer_group_build` | 13 | Day 5 of the month 04:00 for IQD and USD, `schedule.peer_group_build`; `POST /peer-groups` |
| `profile_refresh` | 13 | Day 3 of the month 03:30, `schedule.profile_refresh`; `segment_stats_refresh` completion |
| `partition_maintenance` (extended) | 02 | Same schedule: the three monthly `feat` tables through `feat.create_partition` |
| `list_stale_sweep` | 14 | Every 10 minutes, `schedule.list_stale_sweep`: stale alarms and due feed fetches |
| `list_fetch`, `list_parse`, `harness_run`, `list_activate` | 14 | Sweep, upload or `POST /lists/{id}/fetches`; chained per version |
| `screening_lookback` | 14 | `list_activate`, `POST /screening/lookbacks` |
| `screening_daily_base` | 14 | Daily 03:30, `schedule.screening_daily_base` |
| `screening_onboarding` | 14 | Outbox `canon.touched` (phase 11) |
| `screening_payment` | 14 | Every 15 minutes, `schedule.screening_payment` |
| `screening_adhoc` | 14 | `POST /screening/runs` |
| `pair_memory_sweep` | 14 | Daily 03:10, `schedule.pair_memory_sweep`; Monday QA sample |
| `timer_screening_alert_clock` | 14 | Timer kind `screening_alert_clock` (phase 00 sweep) |
| `ingest_bank_list` | 14 | akr-ingestd; `POST /lists/{bank}/fetches` |
| `maintenance_sweep` (extended) | 07 | Same schedule: deletes transient goAML uploads past `evidence.staging_ttl_h` |
| `rating_compute` | 15 | Trigger consumers (canon, country, PEP, review, override, ownership), after `rating.trigger_debounce_s`; `rating_nightly` |
| `rating_touch_fanout` | 15 | Outbox `canon.touched` (phase 11) |
| `rating_country_fanout` | 15 | Outbox `ref.country_level_changed` (phase 09; TR-10) |
| `rating_nightly` | 15 | Daily 01:30, `schedule.rating_nightly`; a model activation |
| `rating_shadow_run` | 15 | `POST /rating-models/versions/{v}/shadow` |
| `kyc_review_sweep` | 15 | Daily 02:00, `schedule.kyc_review_sweep`: due reviews, expired overrides |
| `timer_kyc_review_clock` | 15 | Timer kind `kyc_review_clock` at a review's due date |
| `pep_list_sweep` | 15 | Every 10 minutes, `schedule.pep_list_sweep`: decided `pep` alerts and the CBS class setting |
| `pep_persistence_sweep` | 15 | Daily 02:15, `schedule.pep_persistence_sweep` |
| `pep_relative_suggest` | 15 | A `pep_entry` activation |
| `ownership_compute` | 15 | `rating_touch_fanout`; `PUT /parties/{id}/ownership/inputs` |
| `rating_feed_export` | 15 | Daily 06:00, `schedule.rating_feed_export`, gated by `kyc.feed_export_enabled` |
| `scenario_touch_fanout` | 16 | Outbox `canon.touched` (phase 11): enqueues incremental, settled and shadow runs |
| `scenario_run` | 16 | The fan-out (settled runs with `run_at` after the settle delay) and the sweeps |
| `scenario_sweep` | 16 | Daily 03:30 (L2) and monthly 04:00 on the 1st (L3), `schedule.scenario_sweep_daily` and `_monthly` |
| `replay_run` | 16 | `POST /scenarios/{id}/replays`; resumable at the committed cursor |
| `replay_sample_plan` | 16 | `POST /replays/{id}/samples` |
| `shadow_compare` | 16 | Daily 05:45, `schedule.shadow_compare`, one job per shadow version |
| `seeded_suite` | 16 | Daily 04:15, `schedule.seeded_suite_daily`; the pack seed; `POST /seeded-tests/runs`; gate G3 |
| `scenario_metrics` | 16 | Daily 06:30, `schedule.scenario_metrics`: silence and flat-line transitions |
| `reader_replay` | 16 | `ReaderCatalogue::measure` when a reader has no replay for a feature, segment or peer proposal |
| `partition_maintenance` (extended) | 02 | Same schedule: `alert.alerts` through `alert.create_partition` |
| `timer_alert_touch` | 17 | Timer `alert_touch` (must-touch-by per class, bank calendar): records one breach |
| `timer_alert_age` | 17 | Timer `alert_age` (`casework.age_raise_hours`): raises the class one step, never above P0 |
| `case_chain_check` | 17 | Daily 02:40, `schedule.case_chain_check`: verifies case event chains |
| `quarantine_retry` | 17 | Hourly at :15, `schedule.quarantine_retry`: re-validates open quarantine rows, never releases |
| `quarantine_release` | 17 | Enqueued by `POST /alert-quarantine/{id}/release`: emits through the sink in the worker |
| `dev_detection_emit` | 17 | Dev builds only: `POST /_dev/detections` fixture emit |
| `timer_work_item_due` | 18 | Timer `work_item_due` at the work item's due time (bank calendar applied at open): one overdue event |
| `work_item_due_scan` | 18 | `schedule.work_item_due_scan` hourly: backstop for open overdue items whose timer did not fire |
| `timer_rfi_clock` | 19 | Timer `rfi_clock` at each RFI clock stage: reminders, overdue notice, expiry |
| `timer_breach_stage` | 19 | Timer `breach_stage` at 50, 100 and 150 percent of an alert or work-item clock: warning or alarm |
| `timer_watch_expiry` | 19 | Timer `watch_expiry` at the watch's expiry: review due |
| `timer_hibernation_expiry` | 19 | Timer `hibernation_expiry` at the hibernation's expiry: end with `expired` |
| `timer_qa_grant_expiry` | 19 | Timer `qa_grant_expiry` at the end of the QA read grant: revoke and void |
| `timer_intake_clock` | 19 | Timer `intake_clock` at an internal report's ack time or an authority request's deadline: overdue outbox event |
| `watch_trigger_dispatch` | 19 | Outbox `screening.hit`, `authority_request.recorded`, `canon.touched`, `alert.created`: watch and hibernation triggers |
| `hibernation_release` | 19 | Outbox `hibernation.ended`, one job per open hit: attach the alert to its case |
| `hibernation_envelope_check` | 19 | Outbox `hibernation.envelope_exceeded`: end the hibernation |
| `hibernation_regret` | 19 | Daily 04:45, `schedule.hibernation_regret`: marks `report_after` |
| `rfi_clock_scan` | 19 | Hourly at minute 35, `schedule.rfi_clock_scan`: expires claims, re-arms missing RFI timers |
| `network_edge_build` | 19 | Outbox `canon.touched` per batch; daily 02:20 sweep `schedule.network_edge_build` |
| `network_snapshot_seal` | 19 | Outbox `network.snapshot_requested` from the network-case route: sealed snapshot via `put_generated` |
| `flat_line_check` | 19 | Daily 04:30, `schedule.flat_line_check`: flat-line alarms |
| `qa_forced_sample` | 19 | Daily 03:10, `schedule.qa_forced_sample`: low-score forced sample (M-16) |
| `qa_suppression_sample` | 19 | Monday 06:00, `schedule.qa_suppression_sample`: 5 percent of screening suppressions |
| `qa_grant_sweep` | 19 | Hourly at minute 50, `schedule.qa_grant_sweep`: revokes QA grants whose timer did not fire |
| `ops_canon_check` | 20 | Enqueued by a task completion and by the sweep: `VerificationService::check` from the canonical copy |
| `ops_check_sweep` | 20 | Every 15 minutes, `schedule.ops_check_sweep`: enqueues checks for tasks needing one, oldest check first |
| `ops_clock_scan` | 20 | Hourly at minute 7, `schedule.ops_clock_scan`: backstop for the freeze and refrain timers |
| `freeze_backfill` | 20 | One-time, `akr-ctl ops backfill-freezes` at install: late freezes for phase 14 recorded true decisions |
| `timer_freeze_due` | 20 | Timer `freeze_due` at the freeze task due time: overdue alarm while `listed` |
| `timer_freeze_limit` | 20 | Timer `freeze_limit` at listing + 24 hours: incident `limit_missed` while `listed` |
| `timer_committee_report_due` | 20 | Timer `committee_report_due` at listing + 48 hours: incident `report_missed` and the letter request if not yet made |
| `timer_second_report_due` | 20 | Timer `second_report_due` two calendar months after the Committee request: second report request |
| `timer_refrain_due` | 20 | Timer `refrain_due` at 7 working days: overdue alarm while the refrain is `open` |
| `report_validate` | 21 | `POST /reports/{id}/validate`: phase 07 engine plus L0 on the frozen content |
| `report_package` | 21 | `POST /reports/{id}/package`: ZIP, SHA-256 manifest and bilingual PDF |
| `ctr_daily` | 21 | `schedule.ctr_daily` after the business-date close, and `POST /ctr/batches`: the daily CTR run |
| `ctr_fx_wait` | 21 | Hourly, `schedule.ctr_fx_wait`: re-runs batches waiting for a rate or a source certificate |
| `ctr_package` | 21 | `POST /ctr/batches/{id}/package`: the CTR files after confirmation |
| `letter_render` | 21 | Enqueued by the `LetterRequester` port and the manual letter route: renders the PDF from the owner row |
| `periodic_build` | 21 | `POST /reports/periodic`: builds a pack with small-number suppression |
| `report_clock_scan` | 21 | Hourly at minute 9, `schedule.report_clock_scan`: backstop for the two report timers |
| `report_calibration` | 21 | Monthly, first day 06:00, `schedule.report_calibration`: RP-29 calibration sample |
| `letter_backfill` | 21 | One-time, `akr-ctl letters backfill` at install: letters under the phase 20 stub ids |
| `timer_report_filing_target` | 21 | Timer `report_filing_target` at signature + `report.filing_target` hours: alarm `report_filing_overdue` while signed or packaged |
| `timer_report_rework_due` | 21 | Timer `report_rework_due` at the task due time: alarm `report_rework_overdue` while the task is open |
| `retention_run` | 23 | `schedule.retention_run` (nightly 03:45, real run); `POST /retention/dry-runs` (dry run); `akr-ctl retention run --now` and `retention replay` |
| `retention_lock_extend` | 23 | Enqueued by the phase 18 case seal transaction: extends the case's evidence locks, never shortens |
| `evidence_reconcile` | 23 | `schedule.evidence_reconcile` (daily 04:10): lists evidence orphans and deletes them after their lock |
| `branch_monthly_draft` | 22 | `schedule.branch_monthly_draft` (first day of the month, 00:30): drafts each branch's report for the month before |
| `timer_liaison_due` | 22 | Timer kind `liaison_due` at `due_on` plus one day: alarm `liaison_report_overdue` while the report is a draft |
| `observation_hybrid` | 22 | Enqueued by the observation transaction: S-23 hybrid alert check (off until `branch.hybrid_scenario_code` is set) |
| `notification_email` | 22 | Enqueued by the dispatcher for an immediate recipient when `notify.email_enabled`: one generic e-mail through the bank relay |
| `notification_digest` | 22 | `schedule.notification_digest` (hourly): one digest e-mail per recipient |
| `notification_sweep` | 22 | `schedule.notification_sweep` (daily 02:10): deletes notices older than `notify.retention_days` |
| `audit_verify_run` | 24 | Enqueued by `POST /audit/verify-runs`: phase 02 `Verifier::run` over the range through the read-only pool; stores counts and finding kinds only |
| `register_post_review_sync` | 24 | `schedule.register_post_review_sync` (every 15 minutes): phase 04 post-change review concerns become findings (unique `source_key`) |
| `review_calendar_sweep` | 24 | `schedule.review_calendar_sweep` (06:00): one warning for each review-calendar row that turns red |
| `fairness_monthly` | 24 | `schedule.fairness_monthly` (day 2, 04:30): group rates of the month before with suppression; a ratio above `fairness.review_ratio` opens a review |
| `access_review_snapshot` | 25 | Enqueued by `POST /admin/access-reviews`: `AccessMatrixBuilder::build(at)` and the grants in force as one evidence object, one item per grant |
| `access_review_sweep` | 25 | `schedule.access_review_sweep` (06:10): one overdue warning in the phase 24 register (fixed source key) |
| `break_glass_sweep` | 25 | `schedule.break_glass_sweep` (every minute): lapses old requests, records expiries, flags reviews overdue after `break_glass.review_hours` |
| `siem_forward` | 25 | `schedule.siem_forward` (every 10 seconds): reads `audit.siem_page` after the cursor, sends CEF or JSON over TLS syslog, dead-letters refused lines |
| `support_session_sweep` | 25 | `schedule.support_session_sweep` (every minute): ends sessions at `ends_at`, revokes the `VS` grant, refuses requests not approved in time |
| `support_bundle_purge` | 25 | `schedule.support_bundle_purge` (03:40): deletes archives older than `support.bundle_retention_days`; the record stays |
| `release_verify` | 25 | Enqueued by `POST /admin/releases/verify`: phase 00 `bundle verify` code path; writes the result or the first failing check; help articles written once |
| `anchor_timestamp` | 25 | Enqueued by the phase 02 anchor job when `audit.rfc3161_enabled`: one RFC 3161 token per anchor |
| `islamic_evidence_sweep` | 26 | `schedule.islamic_evidence_sweep` (04:30): no-op unless `pack.islamic` and `islamic.evidence_tracking`; one counts-only phase 24 register warning (source `scan`, key `islamic_evidence_gap:<date>`) |
| `model_import_verify` | 27 | Enqueued by `POST /models/{id}/versions`: artifact sha, operator allow-list, catalogue features, protected attributes, golden rows; sets `registered` or `refused` |
| `model_score_sweep` | 27 | `schedule.model_score_sweep` (every minute): scores new alerts with the live, challenger and shadow versions; skips everything while a kill switch is engaged |
| `model_monitor` | 27 | `schedule.model_monitor` (05:00): score and feature PSI, override rate, scored count for the previous bank day; a red point opens one phase 24 finding (source `model_monitor`) |
| `model_label_gate` | 27 | `schedule.model_label_gate` (Monday 04:00): QA-confirmed label counts over every tier against the 50/200/500 gates; one `label_gate` evaluation |
| `assistant_generate` | 28 | Enqueued by `POST /assistant/drafts`: re-checks switches and visibility, calls the live model, verifies, scans and seals the draft |
| `assistant_evaluate` | 28 | Enqueued by `POST /assistant/evaluations`: golden drafts on `llama-eval` as the system with purpose `qa_review`; counts only; the human review stage finalises the run |
| `assistant_version_verify` | 28 | Enqueued by `POST /assistant/versions` for a model: manifest, signature and artifact SHA-256 checked on `llama-eval`; sets `registered` or `refused` |
| `assistant_monitor` | 28 | `schedule.assistant_monitor` (05:20): 30-day counts per task on live drafts, suppressed; a red breach opens one phase 24 finding per task and month (source `assistant_monitor`) |
| `assess_release_match` | 29 | `schedule.assess_release_match` (every 15 minutes): released outgoing payments of advisory and enforcing rails from `canon.payment_message` matched to decisions; exceptions `no_decision`, `no_match_key`, `pass_expired`, `decision_not_pass` to the phase 24 register (source `assess_control`); idle while `pack.assess` is off |
| `assess_hold_sweep` | 29 | `schedule.assess_hold_sweep` (every minute): expires holds, opens the phase 14 alert of a list hold as a system `payment` run, syncs alert outcomes, enqueues the extract when the freeze-list versions change; idle while `pack.assess` is off |
| `assess_blind_watch` | 29 | `schedule.assess_blind_watch` (every minute): opens and closes outage periods from node heartbeats; idle while `pack.assess` is off or no rail has an open mode |
| `assess_rollup_watch` | 29 | `schedule.assess_rollup_watch` (00:40): daily counts per rail and mode (`coalesce(would_be, decision)`), hold counts and watch warnings (key age, certificate expiry, extract age, open outage, control test overdue, repeats); idle while `pack.assess` is off |
| `assess_fallback_extract` | 29 | `schedule.assess_fallback_extract` (05:30) and enqueued by the hold sweep: the signed freeze-list extract for the bank's fallback, valid at most 12 hours; idle while `pack.assess` is off |
| `assess_ledger_sweep` | 29 | `schedule.assess_ledger_sweep` (hourly): deletes replay-ledger rows older than `assess.idempotency_hours`; idle while `pack.assess` is off |
| `capacity_snapshot` | 31 | `schedule.capacity_snapshot` (hourly, minute 5): measured arrivals, decisions, handling minutes and backlog per pool, work kind and class, no person column; alerts no queue admits go to the unassigned row; idle while `pack.capacity` is off |
| `capacity_watch` | 31 | `schedule.capacity_watch` (06:20): raises `capacity_forecast` through phase 19 `AlarmService` when the horizon backlog grows and required analysts exceed staffed |
| `capacity_simulation` | 31 | Enqueued by `POST /capacity/simulations`: fluid queue over a completed phase 16 replay; estimates only |
| `exposure_compute` | 31 | `schedule.exposure_compute` (2nd of the month, 01:30): typology exposure of the previous month, small numbers suppressed; idle while `pack.exposure` is off |
| `directory_reconcile` | 31 | `schedule.directory_reconcile` (03:15): repeats a failed directory disable, counts stale pending rows, directory-disabled accounts awaiting an ADM decision and mismatches; never offboards; idle while `pack.polis` is off |
| `pack_build` (extended) | 31 | `POST /packs` kinds `correspondent` and `validator` from system facts |
| `partition_maintenance` (extended) | 02 | Same schedule: `kyc.ratings` through `kyc.create_partition` |

## Gap Analysis Summary

Gap pass 1 coverage complete 2026-10-10 (plan: tools/agent/gap1-plan.md, runs 1 to 8c; Haiku drafts, lead verifies and applies; DR-356 to DR-406). Gap items (ac) to (ae) fixed (DR-405, DR-406). Adversarial pass complete (runs 1 to 6 applied, DR-407 to DR-412). Verification pass applied 2026-10-10 (DR-413). Re-check of its fixes done 2026-10-10: 161 more gaps fixed. Second re-check done the same day: 29 more gaps fixed (2 HIGH). Third re-check done the same day: 5 more gaps fixed (0 HIGH). Fourth re-check done the same day: the agents found nothing in the 6 replacements, and the lead fixed 1 more gap (HIGH, a check of phase 27 that could not pass as written). Fifth re-check done the same day: 0 gaps, a clean round. The verification pass is closed and its report has status complete (837 gaps fixed in all, 49 HIGH); the nine scope decisions of the report wait for the user's word.
- Run 1 (targets 00, 01, 02), 2026-10-10: 32 findings (0 CRITICAL, 8 HIGH, 20 MEDIUM, 4 LOW); section 7 of phases 00,
  01 and 02 filled; correction blocks in phases 06, 08, 13, 23, 25, 26, 28 and 31; DR-356 to DR-359.
- Run 2 (targets 03, 04), 2026-10-10: 24 findings (0 CRITICAL, 1 HIGH, 21 MEDIUM, 2 LOW); section 7 of phases 03 and 04
  filled; correction blocks in phases 02, 24, 25, 27; one API.md row (POST /change-requests/{id}/evidence); DR-360 to DR-364.
- Run 3a (targets 05 to 10), 2026-10-10: 35 findings (0 CRITICAL, 6 HIGH, 22 MEDIUM, 7 LOW); section 7 of phases 05 to
  10 filled; correction blocks in phases 14 and 21; DR-365 to DR-374.
- Run 3b (targets 11 to 16 and the unowned API.md rows), 2026-10-10: 41 findings (0 CRITICAL, 2 HIGH, 25 MEDIUM, 14
  LOW); section 7 of phases 11 to 16 filled, one block in phase 06; 10 API.md rows removed; DR-375 to DR-383.
- Run 4a (targets 17 to 24), 2026-10-10: 31 findings (0 CRITICAL, 0 HIGH, 19 MEDIUM, 12 LOW); section 7 of phases 17
  to 24 filled; carried items from runs 3a and 3b closed in phases 19, 20, 22, 24 and a correction in phase 10; DR-384 to
  DR-388.
- Run 4b (targets 25 to 31 and the system documents), 2026-10-10: 26 findings (0 CRITICAL, 1 HIGH lowered to
  MEDIUM, 17 MEDIUM, 8 LOW; 1 rejected); section 7 of phases 25 to 31 filled; carried items from run 4a closed
  (DELETE grants of phases 27, 28, 29, 31; phase 30 key name; citations in phases 09, 26, 31); DESIGN-SYSTEM gains I1,
  S2 and D3 tabs and the G1 tile; DR-389 to DR-392.
- Run 5a (coverage of C, E, RP, S, TS: 59 IDs), 2026-10-10: all 59 named on phase headers (13 implemented, 37
  partly, 9 not carried); 3 additions to phase 11 and 1 to phase 12 (1 HIGH, 2 MEDIUM); DR-393, DR-394. coverage.py:
  595 carried, 493 named nowhere.
- Run 5b (coverage of SP and K: 51 IDs), 2026-10-10: all 51 named on phase headers (9 implemented, 27 partly, 15
  not carried); additions to phases 18, 15, 21 and 24 (4 MEDIUM); 2 drafted gaps rejected, 8 UNCLEAR decided; DR-395,
  DR-396. coverage.py: 618 carried, 135 Not carried, 455 named nowhere.
- Run 6a (coverage of L: 56 IDs), 2026-10-10: all 56 named on phase headers (3 implemented, 47 partly, 6 not
  carried); 7 drafted gaps rejected; DR-397. coverage.py: 668 carried, 141 Not carried, 399 named nowhere.
- Run 6b (coverage of T: 50 IDs), 2026-10-10: all 50 named on phase headers (11 implemented, 23 partly, 16 not
  carried); counterparty resolver added to phases 13 and 18 (T-29, HIGH); T-09 rejected; branch-first switch fixed at
  false (phase 09); SP-30 placed on the phase 25 header; DR-398, DR-399. coverage.py: 702 carried, 158 Not carried,
  348 named nowhere.
- Run 7a (coverage of UX and W: 54 IDs, plus gap item (ab) X2), 2026-10-10: all 54 named on phase headers (18
  implemented, 24 partly, 12 not carried); 8 gaps accepted (1 HIGH, 5 MEDIUM, 2 LOW): global search, party timeline,
  K1 strip, shortcut layer, response budgets, usability round, accessibility gate, register exports; DR-400.
  coverage.py: 744 carried, 170 Not carried, 294 named nowhere.
- Run 7b (coverage of F, B and U: 46 IDs), 2026-10-10: all 46 named on phase headers (18 implemented, 22 partly, 6
  not carried); 1 gap accepted (MEDIUM): the `profile.breach` topic in phase 17 with phase 15's TR-11 handler; the UN
  feed schema check rejected; DR-401. coverage.py: 784 carried, 176 Not carried, 248 named nowhere (market IDs only).
- Run 8a (coverage of VA and VO: 69 market IDs), 2026-10-10: all 69 named on phase headers (13 implemented, 36
  partly, 20 not carried); 1 gap accepted (HIGH): the parallel run beside the bank's current system in phase 24
  (`alert.parallel_run_rows`, five routes); DR-402. coverage.py: 833 carried, 196 Not carried, 179 named nowhere.
- Run 8b (coverage of VB and VD: 93 market IDs), 2026-10-10: all 93 named on phase headers (15 implemented, 38
  partly, 40 not carried); no gap; DR-403. coverage.py: 886 carried, 236 Not carried, 86 named nowhere (VC).
- Run 8c (coverage of VC: 86 market IDs), 2026-10-10: all 86 named on phase headers (11 implemented, 43 partly, 32
  not carried); 1 gap accepted (MEDIUM): the one-hour re-screen measure and alarm after a list activation in phase 14
  (section 7.10, no new timer, table or route); DR-404. coverage.py: 940 carried, 268 Not carried, 0 named nowhere.
  Gap pass 1 coverage is complete.
- Gap items (ac) to (ae), 2026-10-10 (lead work, apply-g1-items-ac-ae.py): the phase 15 trigger evaluator and the
  producers of the account, outreach and visit events (HIGH, DR-405); the PEP list source, the `regime` pack meaning
  and the look-back subjects (MEDIUM, DR-406). VC-06 moved to implemented.

Adversarial pass (plan: tools/agent/adv-plan.md, 6 runs; Haiku drafts, lead verifies and applies):
- Run 1 (four-eyes and self-approval), 2026-10-10: 19 candidates; 6 confirmed (HIGH 2, MEDIUM 1, LOW 3), 4 rejected, 9
  guarded; DR-407.
- Run 2 (alerts under concurrency, replay and late data), 2026-10-10: 13 candidates; 12 confirmed in 9 fixes (CRITICAL 2,
  HIGH 3, MEDIUM 3, LOW 1), 1 left open, 14 guarded; DR-408.
- Run 3 (watermarks, restores, failover, late batches, source pauses), 2026-10-10: 22 candidates; 9 confirmed (HIGH 6,
  MEDIUM 1, LOW 2), 2 rejected, 11 guarded; DR-409.
- Run 4 (clocks and timers on closed, merged or reopened items), 2026-10-10: 11 candidates; 10 confirmed in 9 fixes (HIGH
  3, MEDIUM 4, LOW 2), 1 guarded on recheck, 17 guarded; DR-410.
- Run 5 (existence masking leaks), 2026-10-10: 11 candidates; 5 confirmed (HIGH 5), 4 LOW consistency fixes, 2 rejected
  as designed, 17 guarded; DR-411.
- Run 6 (idempotency, audit chain, outbox replay), 2026-10-10: 17 candidates; 6 confirmed in 5 fixes (HIGH 2, MEDIUM
  4), 1 LOW, 1 rejected as designed, 9 guarded; DR-412. Adversarial pass complete.

Verification pass (final; three Haiku agents per phase, the lead decides each claim: tools/agent/verify-verdict.md):
- 2026-10-10: 384 items sampled (12 per phase); 94 passed with no gap; 641 gaps confirmed (0 CRITICAL, 39 HIGH, 289
  MEDIUM, 313 LOW), 8 duplicates; all fixed in the phase files (549 replacements, 218 blocks) and by ten hand edits;
  report docs/plan/PHASES-00-31-VERIFICATION.md (status gaps_found); DR-413.
- 2026-10-10, re-check of the 767 fixes (one Haiku agent per phase; lead verdict tools/agent/recheck-verdict.md):
  153 findings raised, all confirmed, and 8 lead finds; 161 gaps (0 CRITICAL, 7 HIGH, 43 MEDIUM, 111 LOW), all fixed
  by 221 replacements (tools/agent/recheck-fixes.txt). The route counts of phases 04, 06, 15, 18, 19, 24 and 25 in
  the table above were corrected; the Routes total is 581, the number of API.md rows. The three checks are green.
- 2026-10-10, second re-check, of the 221 replacements (one Haiku agent per phase; lead verdict
  tools/agent/recheck2-verdict.md): 32 findings raised, 27 confirmed, 5 rejected, and 2 lead finds; 29 gaps
  (0 CRITICAL, 2 HIGH, 7 MEDIUM, 20 LOW), all fixed by 29 replacements. The three checks are green.
- 2026-10-10, third re-check, of the 29 replacements (one Haiku agent per touched phase file, 17 files; lead verdict
  tools/agent/recheck3-verdict.md): 7 findings raised, 3 confirmed, 4 rejected, and 2 lead finds; 5 gaps
  (0 CRITICAL, 0 HIGH, 1 MEDIUM, 4 LOW), all fixed by 6 replacements. The three checks are green.
- 2026-10-10, fourth re-check, of the 6 replacements (one Haiku agent per touched phase file, 5 files; lead verdict
  tools/agent/recheck4-verdict.md): 0 findings raised, and 1 lead find; 1 gap (0 CRITICAL, 1 HIGH: check 6 of phase 27
  compared the visibility predicate with role-only Cedar permits), fixed by 1 replacement. The three checks are green.
- 2026-10-10, fifth re-check, of the 1 replacement (one Haiku agent, phase 27; lead verdict
  tools/agent/recheck5-verdict.md): 0 findings raised, no lead find; 0 gaps. A clean round.
- The plan is ready for implementation when a pass finds 0 true gaps. The fifth re-check did (641, then 161, then 29,
  then 5, then 1, then 0), so the report status is complete. The first pass was a sample of 384 items and the later
  rounds read only changed text, so a gap met while building is fixed in the phase file in the same change. The nine
  scope decisions of the report were put to the owner on 2026-10-10 (next entry).

Owner decisions (2026-10-10; DR-414 to DR-421; block sources in tools/agent/owner/):
- The owner answered twelve questions. Seven answers overturn a scope decision of the verification pass and are now
  in the phase files as section 7+ blocks with the base text corrected: allowed reads go to the SIEM and the SP-36
  rules ship (DR-414; phase 25 section 7.23); banks add their own test cases through change control (DR-415; phase 07
  section 7.6, phase 08 section 7.11, phase 14 section 7.19); the bank keeps payroll, pension and government-source
  tags by hand (DR-416; phase 13 section 7.17, phase 18 section 7.29); RFI pre-fill is built (DR-417; phase 19
  section 7.21); SIEM dead letters can be resent or resolved (DR-418; phase 25 section 7.24); schedule tables are
  mapped (DR-419; phase 26 section 7.13, phase 10 section 7.27); the four Latin-name labels are kept (DR-420; phase
  10 section 7.26, phase 08 section 7.10, phase 18 section 7.28). Three answers confirm the plan (DR-421).
- Counts changed: 14 routes (phase 07 12, phase 13 13, phase 14 27, phase 19 35, phase 25 26; API.md 595 rows) and 6
  tables (phases 07, 13 twice, 14, 19, 26). Phase 25 is XL by the template rule (26 routes; it was M); phases 07 and
  13 stay M.
- Check of the changed text: see docs/plan/PHASES-00-31-VERIFICATION.md, section "Owner decisions (2026-10-10)".
- 2026-10-11, the check is closed after four rounds (one Haiku agent per changed phase, every finding decided by
  the lead; tools/agent/owner-verdict.md): 101, then 31, then 6, then 0 true gaps, 138 in all, 12 of them high,
  all fixed by 258 replacements (tools/agent/owner-fixes.txt). DR-422 and DR-423 lock the design corrections;
  the two rules of DR-423 are phase 13 section 7.18 (a recompute clears the feature rows it no longer produces)
  and phase 04 section 7.21 (a rejection of a change request is not held by the subject check). The three checks
  are green: 423 decisions with 0 problems; 595 API.md rows, all owned; 940 requirements carried, 268 named as
  not carried, 0 named nowhere. The plan is ready for implementation; phase 00 is next.

See roadmap.md "Gap Analysis Additions" for the authoring delta of 2026-10-07 and the CBS
discovery delta of 2026-10-09 (102 items, DR-35 to DR-42).

## Blockers & Notes

- External items and owners: docs/ROADMAP.md "External dependencies". None blocks writing the phase files.
- The two early pieces (phases 06 and 07) are the P0 priority after the foundation they need; phase 07 can ship
  before discovery (phase 05) is finished.
- Dark palette (DS-O1): the owner answered on 2026-10-10 "light only for now" and decides when the phase 08 shell is
  on screen (DR-421); light ships regardless.
- Agents for phase files and gap passes: ask the user first (project rule).
- Gap-pass candidate (found while writing phase 05): phase 04 has no upload route for change-request evidence objects
  (the emergency evidence); phase 05 adds source documents only.
- CBS discovery folded in 2026-10-09 (cbs-discovery/, DR-35 to DR-42). Phases 05, 06, 10 and 11 must use it: change
  strategy per table, per-branch business date, unpopulated fields, composite keys, system-generated postings.
