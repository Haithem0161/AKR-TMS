# Verification pass, phase 00 (Haiku draft for lead review)

Scope: docs/plan/phase-00.md (1068 lines). Lines cited as phase-00.md:LINE unless another file is named. Greps run over
docs/plan/phase-*.md and docs/plan/research.md where a name was checked.

Items sampled (12): header "Requirements carried" TS-01 (start), SP-74 (middle), SP-84 (end); header "Gap pass 1" lines
C-33 (implemented here) and SP-10 (partly implemented); gap-pass blocks 7.1, 7.2, 7.3, 7.11 (four blocks, not adversarial);
adversarial blocks 7.16 (run 4), 7.17 and 7.18 (run 6). Phase 00 has no route in its business sense, so most T2 checks are N/A.

### V-00-01 TS-01 DBS application architecture convention (source: header Requirements carried, start of list; result: PASS)
- T1 SQL: N/A - architecture convention, no table.
- T2 Route: N/A - no business route; infrastructure table only (phase-00.md:449-463: health, readiness, metrics).
- T3 Repository and types: PASS - "Layers are separated by crate boundaries" (phase-00.md:362-363); utoipa-axum OpenApiRouter (phase-00.md:390); Problem schema (phase-00.md:445).
- T4 Logic: N/A - no service step.
- T5 Audit: N/A - no mutation.
- T6 Cross-references: PASS - Cedar deferred to phase 03 (phase-00.md:373 "Cedar schema and policies (phase 03)"); BFF in phase 01 (phase-00.md:390 "BFF (phase 01)"); Orval pipeline in phase 00 (phase-00.md:757).
- T7 Consistency: PASS - no 7.x block changes this item.

### V-00-02 SP-74 signed CycloneDX SBOM per release (source: header Requirements carried, middle of list; result: GAPS 3)
- T1 SQL: N/A - build artefact; the bundle SBOM diff is stored in phase 25 (phase-25.md:184 `sbom_diff JSONB`), checked under T6.
- T2 Route: N/A - CLI only (`akr-ctl bundle build`, phase-00.md:747; `akr-ctl bundle verify`, phase-00.md:752-753).
- T3 Repository and types: PASS - commands and their inputs named (phase-00.md:740-754).
- T4 Logic: GAP - SBOM steps are numbered (phase-00.md:740-751) and refusal is stated (phase-00.md:755 "any failure refuses before anything changes"), but the SBOM scope covers only crates (`cargo-cyclonedx`) and images (Syft). Frontend packages, models and VEX are not in the steps (see GAP lines a to c).
- T5 Audit: N/A - pipeline and CLI, no audit event named in the item; release audit is phase 25's.
- T6 Cross-references: PASS - `sbom_missing` refusal code at phase-25.md:180; `sbom_diff` column at phase-25.md:184; phase 25 runs the phase 00 verifier at phase-25.md:893.
- T7 Consistency: PASS - no 7.x block changes SBOM text.
- Gaps:
GAP V-00-02-a | MEDIUM | T4 | SBOM for the frontend packages (npm/Vite dependency tree) is not generated; only cargo-cyclonedx and Syft are named | section 6 step 5 (phase-00.md:740) | grep "cyclonedx|sbom" over docs/plan/phase-*.md: none for frontend, npm or pnpm
GAP V-00-02-b | MEDIUM | T4 | SBOM for the model files (GGUF served by llama, phase 28) is not covered | section 6 step 5 (phase-00.md:740); phase 28 section 4.4 (phase-28.md:753) | grep "sbom|cyclonedx" phase-28.md: none; grep "GGUF.*sbom" docs: none
GAP V-00-02-c | MEDIUM | T4 | no VEX statement is produced or verified with the signed SBOM (SP-74 names it) | section 6 step 7 (bundle, phase-00.md:747-751) | grep -i "vex" phase-00.md, phase-25.md, phase-30.md: none

### V-00-03 SP-84 backup repositories, heartbeat and WAL alarm (source: header Requirements carried, end of list; result: GAPS 1)
- T1 SQL: N/A - backup tools are compose services; the marker path is a setting (phase-00.md:260 `ops.backup_marker_path`).
- T2 Route: N/A - no route.
- T3 Repository and types: N/A - external tools, no service type.
- T4 Logic: PASS - pgBackRest schedule "WAL every 60 s, weekly full, nightly differential, four fulls" (phase-00.md:694); restic every 15 min (phase-00.md:695); `backup_check` daily with heartbeat marker (phase-00.md:683-684).
- T5 Audit: N/A - no business mutation.
- T6 Cross-references: PASS - second repository and offsite copy named at phase-30.md:259; the immutable copy is deferred to phase 30 section 3.9 (phase-30.md:264 "SP-88") and Open doubts item 22 (phase-30.md:636 "not object-locked in this design").
- T7 Consistency: PASS - 7.11 extends backup_check (phase-00.md:976-983) without contradiction.
- Gaps:
GAP V-00-03-a | MEDIUM | T4 | no loud WAL-gap alarm: SP-84 requires one and phase 00 names only `wal_level=replica` and the archive command | phase 00 section 4 alert rules (7.19, phase-00.md:1046-1049) or phase 30 section 3.13 (phase-30.md:331) | grep -i "wal" phase-00.md (only lines 692 and 692-694); grep -i "wal.*alarm|wal gap|wal-gap" phase-00.md phase-30.md: none

### V-00-04 C-33 CBS egress only from akr-ingestd (source: header Gap pass 1 "implemented here"; result: PASS)
- T1 SQL: N/A - network rule, no table.
- T2 Route: N/A - no route.
- T3 Repository and types: PASS - rule rendered by `akr-ctl net render` from `deploy/zones.yaml` (phase-00.md:713-714).
- T4 Logic: PASS - "let only `akr-ingestd`'s container address reach the CBS listener addresses declared at install" (phase-00.md:713-714); verification by `make zone-check` (phase-00.md:718-721).
- T5 Audit: N/A - no mutation.
- T6 Cross-references: PASS - `cbs.listeners` key defined in phase 30 section 4.4 (phase-30.md:471) and rule text at phase-30.md:132; `z1_cbs` carries CBS traffic in phase 05 (phase-05.md:1354).
- T7 Consistency: PASS - 7.7 adds the `cbs.listeners` key (phase-00.md:943) and does not contradict 713-714.

### V-00-05 SP-10 field encryption by class and keyed match hashes (source: header Gap pass 1 "partly implemented"; result: GAPS 1)
- T1 SQL: PASS - `admin.column_classification` with `encrypted` and `keyed_hash` columns and CHECK (phase-00.md:291-300).
- T2 Route: N/A - no route.
- T3 Repository and types: PASS - `trait Crypto { seal; open; keyed_hash(purpose, data); ... }` (phase-00.md:587-588); `common::crypto::Purpose` constants (phase-00.md:1001-1004, 7.15).
- T4 Logic: GAP - the port has no failure path for `open` (wrong DEK version or AAD mismatch) and names no problem code. Family and DEK rules are stated (phase-00.md:593-598).
- T5 Audit: N/A - no business mutation in the port.
- T6 Cross-references: PASS - phase 10 `KeyedHash(purpose)` (phase-10.md:1585); phase 01 purposes `csrf`, `token`, `login_identifier` (phase-01.md:407); phase 26 `islamic.document_ref` (phase-26.md:396).
- T7 Consistency: PASS - 7.15 extends the purpose list without contradiction.
- Gaps:
GAP V-00-05-a | LOW | T4 | `Crypto::open` has no error for a decryption or AAD failure | section 2 crypto port (phase-00.md:587-588) | grep -i "decrypt|AAD|open(" phase-00.md: no error mapping

### V-00-06 7.1 outbox handler lists and topic names (source: 7.1; result: GAPS 1)
- T1 SQL: PASS - `admin.outbox` with the dotted-name CHECK the block relies on (phase-00.md:125-134, CHECK at line 128).
- T2 Route: N/A.
- T3 Repository and types: GAP - `TopicHandler` and `OutboxDispatcher` have no signature anywhere in phase 00. The block gives `register(topic, handler)` (phase-00.md:836), but the handler type and its method are not defined. `OutboxStore` (phase-00.md:436-437) lists no handler type. Phase 22 calls `NotificationDispatcher::handle(tx, outbox_row)` (phase-22.md:583) with no shared trait.
- T4 Logic: PASS - dispatched_at only when every handler succeeds (phase-00.md:836-838); failure keeps the row undispatched with back-off (phase-00.md:558-560).
- T5 Audit: N/A - dispatch is harness state.
- T6 Cross-references: PASS - `canon.touched` (phase-11.md:833); `alert.created` and `case.event` (phase-17.md:1539); `notify.request` (phase-22.md:599); `siem.export` released (phase-25.md:869).
- T7 Consistency: PASS - the block replaces `notify` and `siem.export` in the base reservation sentence (phase-00.md:840-842 over 557-558), which it states it does.
- Gaps:
GAP V-00-06-a | MEDIUM | T3 | the `TopicHandler` trait (handler method, arguments, error type) is not defined; the block names `register(topic, handler)` only | section 2 jobs module (phase-00.md:436-437) or 7.1 | grep "TopicHandler" docs/plan/phase-*.md: only phase-00.md:555, 836 and phase-14.md:1224; grep "trait TopicHandler|fn handle" phase-00.md: none

### V-00-07 7.2 job harness: heartbeat, cluster-wide concurrency, database clock, listeners (source: 7.2; result: GAPS 2)
- T1 SQL: PASS - `admin.jobs.locked_at` and `locked_by` (phase-00.md:107-108); `running` enum value (phase-00.md:97).
- T2 Route: N/A.
- T3 Repository and types: GAP - `JobKind` (with `concurrency` and the per-kind lease override) is used at phase-00.md:539 and 850-851 but never defined as a type or registry shape in phase 00. The kind registry is prose only (phase-00.md:680-684). `JobStore::heartbeat(id, locked_by)` is named by the block (phase-00.md:847) and is PASS.
- T4 Logic: GAP (LOW) - the heartbeat is guarded by `locked_by`, but the result when the update matches zero rows (lease lost) is not stated. Claim lock order is stated (phase-00.md:850-851).
- T5 Audit: N/A - harness state, no business mutation.
- T6 Cross-references: PASS - `akr_policy` (phase-03.md:430 "NOTIFY akr_policy"); `akr_source` (phase-05.md:705); `akr_assess_policy` (phase-29.md:1029); `akr_assess_gateway` (phase-29.md:1074); `jobs.lease_s` setting (phase-00.md:244).
- T7 Consistency: PASS - scheduler clock (phase-00.md:548 cron in bank.timezone; 7.2 names database clock for slot computation) does not contradict; reaper rule (phase-00.md:546) matches 7.2.
- Gaps:
GAP V-00-07-a | MEDIUM | T3 | `JobKind` is named but has no definition (fields: kind name, concurrency, lease override, max attempts) | section 2 jobs module or section 3 kind registry (phase-00.md:680-684) | grep "JobKind" docs/plan/*.md: only phase-00.md:539, 850-851 and phase-00.md:851 "No new field on JobKind"; grep "struct JobKind|trait JobKind" docs/plan: none
GAP V-00-07-b | LOW | T4 | heartbeat result when zero rows match (lease already reaped or `locked_by` changed) is not stated | 7.2 first bullet (phase-00.md:847-849) | grep -i "heartbeat" phase-00.md: only 847-848

### V-00-08 7.3 timer kinds registered by later phases (source: 7.3; result: PASS)
- T1 SQL: PASS - `admin.timers` with owner_kind, owner_id, kind, fire_at, fired_at, cancelled_at (phase-00.md:138-149).
- T2 Route: N/A.
- T3 Repository and types: PASS - `TimerService` schedule, cancel, cancel_for_owner (phase-00.md:561-562); `NewTimer` fields (phase-00.md:561).
- T4 Logic: PASS - sweep claims due timers and enqueues `timer_<kind>` in the same transaction (phase-00.md:562-564); unregistered kind never claimed (phase-00.md:859-861).
- T5 Audit: N/A - registry block; the sweep's `fired_at` write is covered by the gap in V-00-10 T5.
- T6 Cross-references: PASS - all 18 kinds in the table were grepped in the named owner phase: cr_activate and task_timeout (phase-04.md), screening_alert_clock (phase-14.md), kyc_review_clock (phase-15.md), alert_touch (phase-17.md), work_item_due (phase-18.md), rfi_clock, breach_stage, watch_expiry, hibernation_expiry, qa_grant_expiry, intake_clock (phase-19.md), freeze_due, refrain_due (phase-20.md; section 4.2 at phase-20.md:941 "gains five kinds"), freeze_limit and the two report kinds (phase-20.md:656-658), report_filing_target and report_rework_due (phase-21.md), liaison_due (phase-22.md:789).
- T7 Consistency: PASS - base timer rule (phase-00.md:564-567) matches 7.3.

### V-00-09 7.11 maintenance_sweep and backup_check additions (source: 7.11; result: GAPS 1)
- T1 SQL: PASS - `admin.stepup_assertions` (phase-01.md:238); `stepup.retention_days` (phase-01.md:377); bucket `akr-quarantine` (phase-02.md:359); `evidence.staging_ttl_h` (phase-02.md:518); `transient_deleted` column (phase-07.md:219); `ops.last_failover_drill_at` (phase-30.md:41).
- T2 Route: N/A.
- T3 Repository and types: PASS - sweep job exists (phase-00.md:680 "maintenance_sweep"); no new method named.
- T4 Logic: PASS - three delete rules with age, prefix and exclusions (phase-00.md:977-981; infected or rejected never deleted, phase-02.md:1021-1022).
- T5 Audit: GAP - deletes (a) step-up rows and (b) staging objects name no audit event. Only (c) names one (`goaml_check.transient_deleted`, phase-00.md:982; phase-07.md:717). phase-01.md:886-887 and phase-02.md:1021-1023 name none either.
- T6 Cross-references: PASS - phase-07.md:717 audit kind; phase-07.md:564 "maintenance_sweep (phase 00, extended)".
- T7 Consistency: PASS - phase-07.md:806-807 says the same as 7.11 (c).
- Gaps:
GAP V-00-09-a | MEDIUM | T5 | the sweep's deletes of `admin.stepup_assertions` rows (a) and of `akr-quarantine` staging objects (b) name no audit event; the base sweep deletes (idempotency, jobs, outbox) name none either (phase-00.md:524-531) | 7.11 items (a) and (b), phase-00.md:976-981 | grep -i "audit" phase-01.md:884-890, phase-02.md:1019-1023, phase-00.md:524-531: no audit kind for these deletes

### V-00-10 7.16 timer cancel touches pending timers only (adversarial run 4; result: GAPS 1)
- T1 SQL: PASS - `admin.timers` columns and CHECK `fired_at IS NULL OR cancelled_at IS NULL` (phase-00.md:138-149, CHECK at 148).
- T2 Route: N/A.
- T3 Repository and types: PASS - `cancel(tx, id)` and `cancel_for_owner(tx, owner_kind, owner_id, kind)` (phase-00.md:561-562); returns the row count (phase-00.md:1012-1013).
- T4 Logic: PASS - one UPDATE with the pending predicate; zero rows is a success (phase-00.md:1011-1014); test given (phase-00.md:1015).
- T5 Audit: GAP - the UPDATE of `admin.timers` names no audit event. The harness tables (jobs, outbox, timers) name none anywhere in phase 00 (grep -i audit phase-00.md: no harness row). The lead must decide whether the harness is exempt and say so in section 2.
- T6 Cross-references: PASS - `cancel_for_owner` used by phase 20 (phase-20.md:704), phase 15 (phase-15.md:2150) and phase 14 (phase-14.md:1030).
- T7 Consistency: PASS - base text calls cancel an optimisation (phase-00.md:565-566), 7.16 agrees.
- Gaps:
GAP V-00-10-a | MEDIUM | T5 | the timer cancel UPDATE names no audit event; neither do the timer fire and the sweep's fired_at write (phase-00.md:562-564) | 7.16 or section 2 TimerService (phase-00.md:561) | grep -i "audit" phase-00.md lines 555-570 and 1009-1016: none

### V-00-11 7.17 one transaction for the claim, the handler and the stored response (adversarial run 6; result: PASS)
- T1 SQL: PASS - `admin.idempotency_keys` with `completed_at`, `response_status`, `request_hash` and CHECK (phase-00.md:188-200).
- T2 Route: N/A - middleware applies to mutating routes; no route named.
- T3 Repository and types: PASS - `RequestTx` extractor named (phase-00.md:1021-1024); `IdempotencyService` in section 3 (phase-00.md:1020).
- T4 Logic: PASS - steps 1 to 5 numbered, with 409 `E_IDEMPOTENCY_IN_FLIGHT` for the in-flight case (phase-00.md:1021-1033); `lock_timeout` value named (phase-00.md:1021-1022).
- T5 Audit: N/A - the claim table is infrastructure; the business audit stays with the owner handler, "the audit emit is the last statement before commit" (phase-02.md:680).
- T6 Cross-references: PASS - `idempotency.in_flight_lease_s` (phase-00.md:242); `E_IDEMPOTENCY_IN_FLIGHT` in docs/API.md:41 and 923; DR-412 exists (docs/plan/research.md, "| DR-412 |"); phase 03 decision row in its own short transaction (phase-03.md:706-708); phase 02 access events in their own transaction (phase-02.md:715); phase 18 opened views (phase-18.md:703).
- T7 Consistency: PASS - 7.17 items 4 and 5 remove the takeover clause (phase-00.md:526-527) and replace "delete the claim" (phase-00.md:529); the block says what it replaces.

### V-00-12 7.18 jobs complete in their own transaction (adversarial run 6; result: GAPS 2)
- T1 SQL: PASS - `admin.jobs` columns status, finished_at, locked_by, locked_at (phase-00.md:100-116).
- T2 Route: N/A.
- T3 Repository and types: GAP - `complete_in(tx)` must use "the values of this worker's claim" (locked_by, locked_at), but `JobContext` carries only "job id, attempt, correlation id and the system principal" (phase-00.md:541-542). The block does not say where the claim values reach the handler.
- T4 Logic: PASS - UPDATE guarded by status, locked_by and locked_at; zero rows gives `JobError::Retryable` with rollback (phase-00.md:1039-1043).
- T5 Audit: N/A - job row completion is harness state; handler effects belong to the handler's phase (phase-15.md:2158-2159 names `rating.trigger_applied`).
- T6 Cross-references: PASS - phase 15 uses `complete_in` for `rating_compute` (phase-15.md:2158). GAP (LOW) - the block points to "phase 19 section 7+" for the per-item marker; phase 19 section 7+ (phase-19.md:1836 onward) names no marker.
- T7 Consistency: PASS - base step 4 (phase-00.md:543) is skipped for complete_in handlers, which the block states (phase-00.md:1040-1041).
- Gaps:
GAP V-00-12-a | MEDIUM | T3 | the source of `locked_by` and `locked_at` for `JobContext::complete_in` is not stated (JobContext fields omit them) | section 2 JobContext or 7.18 (phase-00.md:541-542, 1039) | grep -i "locked_by" phase-00.md: 107-108 and 1039-1040 only; grep "JobContext" phase-00.md: 541, 1039 only
GAP V-00-12-b | LOW | T6 | "phase 19 section 7+" is named as the home of the per-item marker, but it is not there | phase-00.md:1040-1043 or phase-19.md section 7+ | grep -i "marker|per item" phase-19.md from line 1836: none

## Summary
- Items checked: 12 (5 header, 4 gap-pass blocks 7.1 / 7.2 / 7.3 / 7.11, 3 adversarial blocks 7.16 / 7.17 / 7.18).
- Items fully PASS: 4 (V-00-01, V-00-04, V-00-08, V-00-11).
- GAP lines by severity: HIGH 0; MEDIUM 9; LOW 3 (total 12).
- Structure note (not a GAP line): phase-00.md has no "## Decisions" and no "## Open doubts" section (headings at lines 23, 360, 469, 678, 769, 774, 831, 1054 only), although planning.md and the conventions require both before "What This Phase Does NOT Touch".

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-00-02-a | MEDIUM | T4 | frontend package SBOM | section 6 step 5 (phase-00.md:740) |
| V-00-02-b | MEDIUM | T4 | model file SBOM | section 6 step 5 (phase-00.md:740) |
| V-00-02-c | MEDIUM | T4 | VEX statement | section 6 step 7 (phase-00.md:747-751) |
| V-00-03-a | MEDIUM | T4 | WAL-gap alarm | phase 00 7.19 (phase-00.md:1046-1049) or phase 30 section 3.13 (phase-30.md:331) |
| V-00-05-a | LOW | T4 | `Crypto::open` failure error | section 2 crypto port (phase-00.md:587-588) |
| V-00-06-a | MEDIUM | T3 | `TopicHandler` trait signature | section 2 jobs module (phase-00.md:436-437) or 7.1 |
| V-00-07-a | MEDIUM | T3 | `JobKind` definition | section 2 or section 3 kind registry (phase-00.md:680-684) |
| V-00-07-b | LOW | T4 | heartbeat zero-row result | 7.2 (phase-00.md:847-849) |
| V-00-09-a | MEDIUM | T5 | audit events for sweep deletes (a) and (b) | 7.11 (phase-00.md:976-981) |
| V-00-10-a | MEDIUM | T5 | audit for timer cancel and fire | section 2 TimerService (phase-00.md:561-564) or 7.16 |
| V-00-12-a | MEDIUM | T3 | source of locked_by and locked_at for complete_in | JobContext, section 2 (phase-00.md:541-542) or 7.18 |
| V-00-12-b | LOW | T6 | per-item marker named in phase 19 section 7+ but absent | phase-19.md section 7+ (line 1836 onward) |

Summary note: the 12 GAP lines above count 9 MEDIUM and 3 LOW; the lead should grep each one before acting, as instructed.
