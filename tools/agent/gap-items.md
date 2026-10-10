# Known items for gap pass 1 (collected during batch reviews; lead, 2026-10-09)
(a) DONE 2026-10-10 (tools/agent/fix-g1-cedar.py; also the aal string test in 06, 12). HIGH: phases 05, 10, 11, 12 use Cedar `principal in Akr::Role::"X"`: rewrite to `when { principal.roles.contains("X") }`.
(b) API.md 3.8 `/ref/registers/{kind}` and `/ref/typologies` have no owner phase.
(c) API.md 3.5 backfill routes have no owner (DR-90).
(d) DONE 2026-10-10 (phase 04 7.2, DR-363). Phase 04 has no change-request evidence upload route.
(e) Phase 04 approver-ids port for DR-92 masking.
(f) Migration smoke test of the whole chain in a throwaway postgres:17.
(g) Phase 10 name keys vs phase 14 `names` crate.
(h) Freeze-list alert retention R7 vs R5.
(i) Masked readers of phases 13-16 must use casework.effective_tier.
(j) Rating feed cursor.
(k) K-12, K-19, K-20, K-23, K-25, K-31, K-32, K-34, K-36, K-38..41 unassigned.
(l) Phase 16 items.
(m) Phase 17 items: ScreeningCaseLinker, case_event anchor, owner kind alert, SSE 503.
(n) Phase 18 items: phase 04 check_before_decision + Holders rule 5 text; phase 02 owner kind case; phase 01 StepUpAction
    list; M-36 -> phase 15; home of the sub-codes.
(o) Phase 19: phase 17 file needs intake step 4b + attach_released + queue/count clause + alarm calls; phase 18 file needs
    close gate (uncorroborated_explanation), network hook, reopen_under_batch, WorkItemService arm/breach; DSL E.4 rows
    rfi_release_internal/external; phase 02 owner kind network_snapshot; phase 03 MASKED_KINDS `rfi`; M-51..55, M-76, T-22
    target phases; API.md /cases/{id}/acl vs phase 18 /cases/{id}/access path.
(p) Phase 20: phase 09 ClockRegistry gains 4 fixed entries (freeze limit 24h, Committee report 48h, refrain 7 working days,
    second report 2 calendar months) and the calendar-month function; phase 19 section 3.10 alarm table rows for the 3 new
    kinds; DSL E.3/E.4 rows freeze_release, ops_attestation and effects release_freeze, record_verification,
    create_ops_task; phase 02 owner kind ops_task; phase 14 delisted_on set on entries omitted from a new list version;
    phase 04 `setting` change target bound (1..8) for ops.freeze_task_hours; phase 15 exit_decision definition calls
    create_ops_task.
(q) Phase 21: phase 18 file needs check_before_decision report checks, the route refusal `report_sign_required` on the phase
    18 and generic task routes, the close gate on an acknowledged filing (`filing_not_acknowledged`) and the replaced
    `release_for_filing` body; phase 04 registries `ReportWorkflowSubject` (resource kind report) and `release_correction`;
    DSL E.3/E.4 rows report_correction, release_correction; phase 07 `validate_built`, `RejectionParser` completion, rule
    codes CONS-04 NAME-01 DATE-01 ID-01 PKG-07 CTR-01..03, profile clocks; phase 09 ClockRegistry report.filing_target,
    report.rework; phase 02 owner kinds report, report_package, filing, letter, periodic_report, fiu_feedback; phase 00 timer
    kinds report_filing_target, report_rework_due; phase 22 owns /liaison-reports + review + R5 (periodic `liaison` kind reads
    phase 22's table); phase 03 purpose codes for report download.
(r) Phase 23: phase 18 seal transaction enqueues `retention_lock_extend` (case_id); phase 02 EvidenceStore gains shred,
    delete_object, extend_lock and owner kind legal_hold; phase 10/11 canonical writers read `admin.retention_tombstones`
    before inserting a customer or account identity; phase 04 registries `LegalHoldWorkflowSubject` (resource kind
    legal_hold), effect `release_legal_hold`, DSL E.4 `legal_hold_release`; phase 00 role akr_retention + LOGIN
    akr_retention_login provisioning + restore runbook (verify-certificates, replay) and readiness gate; phase 03 bundle
    manifest gains 61_retention.cedar and StepUpAction legal_hold.place / legal_hold.release; M-60 cited by phase 18 is
    not in MODULES.md (add or re-cite SP-58); R5-R7 purge beyond partitions is report-only (DR-191: bank decision on the
    phase 17/18 append-only guards); case holds hidden from IA (Open doubt 24, bank).
(s) Phase 22: phase 00 outbox registry maps a topic to an ordered handler list (row dispatched when all succeed; all
    idempotent); phase 19 `RfiService::record_answers_in_tx` (no `rfi.answer` check), `corroborates` accepting owner kind
    `rfi_input`, release chain event `case.rfi_released {rfi_id, recipient_kind}`, register detail shows observation fields
    to M; phase 02 owner kind `rfi_input`; phase 03 one branch per liaison or alternate, `UserRoleQuery` port, MASKED_KINDS,
    outbox `credential_reset.requested`; phase 04 `notify` effect params (DSL E.3), remind and escalate write
    `notify.request`, ChangeTargetRegistry `notification_template`; phases 16/17 `DetectionPayload.source_kind`
    `observation`; phase 23 disposal handlers for the six new tables (notifications swept by phase 22's own job: reconcile
    with the R12 handler and `akr_retention`); API.md SSE path (`/events` vs phase 17 `/events/counts`); B6 content and
    the seven-criteria checklist inside every case (bank); the 404 branch gate is an exception to API.md s.6 (DR-206).
(t) Phase 24: phase 06 step-up class `pack_download` is not in phase 01's closed list (use `export`); phase 06 pack routes
    accept `assessor.read` and its Cedar keeps AS to kind `assessor`; phase 02 evidence owner kinds `validation_pack`,
    `training_record`; phase 03 MASKED_KINDS `audit_event`, `audit_target`; phase 00 AppRole akr-api KV `db/akr_readonly`
    and the read-only pool provisioning; phase 04 ChangeTarget `protected_attribute`, `InventoryService::update` called by a
    validation pass; phase 16 `check_before_decision` calls `ProtectedAttributeCheck` and the scenario activation column;
    phase 18 case review column for clock `case.review`; canary records (research 30 s.8.6, 8.8) built by no phase; dev
    user `as@bank.test`; API.md `GET /audit/events/{id}` key needs `occurred_at`; phase 23 disposal handlers for the seven
    tables; phase 09 holds no CBI assessment-guide clause map (phase 24 ships it as a signed data file, DR-229).
(u) Phase 25: phase 17/18 case read path writes `break_glass.used` and the dedicated-log `used` row and takes the
    purpose-keyed break-glass clause; phase 19 `Rfi.assignees` must not take the break-glass requester; phase 01 dev users
    `sec2@bank.test`, `vs@bank.test` and key inventory entry `akr-sign-bundle` (SP-11); API.md scope `support.readonly` is
    held by no route (kept for VS, DR-237); phase 22 `NoticeCode` 15 -> 17 and the recipient rule "active holders of a
    role"; phase 04 `role_grant` operation `revoke`, target `platform_release` with its `EvidenceResolver`; phase 00 zones
    `z2_bank` (akr-worker egress to SIEM and TSA) and `z3_idp` (kratos egress to the issuer); phase 24 control report
    gains two counts; phase 23 disposal handlers for the eleven tables (help_articles and break_glass_log guards refuse
    DELETE); MODULES M3 approvers changed to CH + SEC (DR-242; bank to confirm, phase 25 Open doubts 2); `audit.siem_page` re-tested
    2026-10-09 in postgres:17 (strips non-normal and unknown tiers, akr_app has no direct read); the rest of the phase 25
    SQL not re-run after the lead fixes (run in the verification pass).
(v) Phase 26: DESIGN-SYSTEM 8.1 has no Sharia view screen (phase 26 proposes `I1`; status counts it); phase 02 owner
    kind `islamic_deal` and key purpose `islamic.document_ref`; API.md s.6 lacks the phase 02 codes E_UPLOAD_REJECTED,
    E_PAYLOAD_TOO_LARGE, E_UNAVAILABLE (earlier phases use them too); phase 16 pack seed skips flagged documents and G9
    reads `pack: islamic`; phase 03 flag service refuses `pack.islamic` off while an islamic version is live; phase 10
    `CanonicalEntity` +2 and `canonical-model.v1.json`; phase 20 `FreezeService::start` grouping by account class (and a
    two-person rule for pooled holds, phase 26 Open doubts 9); hard-rule scenarios are `restricted` (suspicion tier, M
    or team only): the bank may prefer `normal` for product-rule breaches; retention of superseded deal versions (closure
    clock on rows whose `closed_on` is NULL); I-09 and the other not-carried I- IDs; phase 26 SQL 1.1-1.3 tested
    2026-10-09 in postgres:17 with stubs after the lead fixes.
(w) Phase 27: phase 24 changes (`registers.source` + `model_monitor`, `fairness_metrics.metric` + `score_top_band_rate`,
five closure codes, review-calendar rows `model_kill_drill` and starter recalibration, command-centre tile
`model_governance`, `ValidationPackService` model branch); phase 23 purge of `alert.model_scores` before an alert
partition drop; phase 25 SIEM and support bundle exclude scores, release import sets `registered` only; phase 02 owner
kind `model_version`; phase 00 volume `model-store` and key purpose `keys/model-artifact`; phase 17 queue ORDER BY and
`model_band`; kill switch owner versus MRO scope (phase 27 Open doubts 1); differencing of daily rolling-window points
(an MRO can learn that a report decision happened, never which case).
(x) Phase 28: phase 19 weekly draw of `assistant_assisted`, source kind `assistant_draft` and the QA item view of an
evaluation draft; phase 18 `RecordDecisionEffect` call and the W3 `assistant_summary` affordance; phase 21 names
`ReportService::update_draft`; phase 23 purge of drafts before the case row and owner kind; phase 24 review-calendar
row `assistant_kill_drill` and `ValidationPackService` assistant branch; phase 25 SIEM and support bundle exclusions,
release import `registered` only; phase 03 flag service refuses `pack.assistant` off while a version is live; phase 00
zone `z2_model`, volumes, `llama` and `llama-eval` images; whether MRO sees the `report_narrative` monitoring row
(phase 24 shows report rates only to `dashboard.read`); kill switch engage by one MRO versus the two-person reading of
SP-95 (phase 28 Open doubts 8).
(y) Phase 29: phase 14 `ScreeningService::open_hold_alert` (system `payment` run, `trigger_ref` = hold id), the
hold-alert subject view from `assess.holds.party_names_enc`, the `subject_id` comment of `alert.screening_runs`
widened, `screening_payment` skipping an entry already alerted on a hold with the same match key, and
`list.feed_unchanged` renewing the freshness clock (phase 29 Open doubts 29); phase 00 `akr-assess` in `z1_bao`,
line 588 `akr-sign-decision` reserved and unused, AppRole transit decrypt on `akr-kek` and KV read `keys/assess-*`
(Open doubts 1-2); phase 02 owner kind `assess_extract` and the PartitionRegistry entry for `assess.decisions`; phase
03 `Ctx.via` value `gateway_cert`; phase 23 drop of a decision partition only after its holds are gone; phase 24
review-calendar row `assess_control_test` and finding closure closing release exceptions; phase 25 SIEM and support
bundle exclusions; phase 01 service-principal expiry versus certificate life (Open doubts 24); DESIGN-SYSTEM S2 tab
Holds, D3 tab Assessment, G1 tile Assessment.
(z) Phase 30: phase 00 job harness (lease heartbeat every third of the kind's lease, cluster-wide concurrency under
`pg_advisory_xact_lock(hashtext(kind))`, database-clock scheduler slots; DR-337), listener reload on reconnect for
`akr_assess_policy`, `akr_assess_gateway`, `akr_source`; phase 00 and phase 29 readiness of `akr-assess` lose Valkey;
phase 01 limiter fails open with `akr_rate_limit_unavailable_total`; phase 02 owner kind `ops_drill` and five audit
kinds; phase 03 eight A3 tiles; phase 25 bundle `chart/` and `k3s/`, refusal code, support bundle excludes cluster
objects; phase 16 `cost` keys `read_source`, `replay_lag_s`; phase 00 AppRole `akr-migrate` and `zones.yaml` keys
`cluster.apiserver`, `cbs.listeners`, `gateway.cidrs`; ARCHITECTURE D19 and research 32 16.6 two-site split rejected
(DR-332, Open doubts 3); DR workspace not carried (Open doubts 31); MODULES M20 requirements now SP-83..SP-88.
(aa) Phase 31: phase 03 `UserAdminService::disable_from_directory`, `user.disabled` key `source`, the enable check
`directory_inactive`, `Ctx.via` `webhook_hmac`, creation of the service principal `polis-directory`; phase 19 QA sampler
surge rate and the `AlarmService` kind row `capacity_forecast`; phase 24 `TrainingRegisterService::list` practice column
(first attempt only); phase 06 pack builders `correspondent` and `validator`; phase 00 zones (Polis rows), the Caddy
exclusion of `/api/v1/directory/events`, AppRoles `akr-polis` and `kv/polis/directory` for akr-api; phase 01 `db
provision` database `polis`; phase 00 schema list gains `train`; DESIGN-SYSTEM W12, G2 detail and the W8, R4, G1, G9,
A1 panels; the alert-to-pool attribution rule (phase 31 section 3.3, Open doubts 4); the Polis paths and webhook header
format from the pinned version (Open doubts 13).
(ab) [closed by run 7a: phase 18 section 7+ GET /search, DR-400] Found in run 6a (2026-10-10): DESIGN-SYSTEM X2 (global search and command palette, line 682) is a P1 screen; phase 08 leaves it to P1 and no phase builds it (no search route in API.md). Run 7 (UX coverage) places it; SP-27 (suspicion tier out of global search and type-ahead) and RP-61 (report register out of global search) bind it.
(ac) DONE 2026-10-10 (tools/agent/apply-g1-items-ac-ae.py, DR-405). Found in run 7b (2026-10-10): trigger topics of DSL C.5 that no phase writes, while phase 15 claims K-11 (the
trigger library): `txn.new_corridor` (TR-12), `account.reactivated` (TR-14), `account.opened` (TR-24),
`outreach.failed` (TR-20), `visit.recorded` (TR-19); `media.recorded` (TR-09) and `staff_link.recorded` (TR-25) have
P2 sources. `profile.breach` (TR-11) is fixed by run 7b (phase 17 section 7+, DR-401). For pass 2.
(ad) DONE 2026-10-10 (tools/agent/apply-g1-items-ac-ae.py, DR-406). Found in run 7b (2026-10-10): phase 15 `pep_list_sweep` source 1 reads screening alerts with `list_kind = 'pep'`,
but `ref.lists.code` is closed to six codes and none is seeded as kind `pep` (DR-100 "`pep` a kind only"), so that
source never fires. Also DSL A names a `regime` pack "PF-Iran" while ROADMAP P3 names US, UK and EU regime packs
(run 7b U-08). For pass 2.
(ae) DONE 2026-10-10 (tools/agent/apply-g1-items-ac-ae.py, DR-406). Found in run 8c (2026-10-10): phase 14 section 3.5 run kind 2 (`lookback`) screens the version's added and amended
entries against "all current subjects" but does not say whether counterparties and payment parties are in that subject
index (daily_base is a keyset over party ids only; the payment run takes only parties first seen since its checkpoint).
VC-06 asks for recent counterparties in the delta re-screen. For pass 2: name the subject kinds of the look-back.
