# Verification pass, phase 16 (Haiku draft for lead review)

Items selected before checking (line numbers are in docs/plan/phase-16.md):
- Header "Requirements carried" (start, middle, late, end of the list): S-05 (line 13), S-49 (line 28), L-18 (line 39), L-61 (line 42).
- Header "Implemented here" (line 44): VB-23.
- Gap pass blocks, run 3b (header at line 1944), blocks 7.2, 7.3, 7.4, 7.6 (7.1 to 7.8 are under this header).
- Adversarial pass blocks, run 2 (header at line 1982), blocks 7.9, 7.10, 7.11.

### V-16-01 Bank scenario creation, POST /scenarios   (source: header "Requirements carried", S-05, line 13; result: GAPS 3)
- T1 SQL: PASS - every field of `NewScenario` is a column of phase 06's `scn.scenarios` (phase-06.md:72-93: code, family, pack, name, summary, typologies, needs, observational, research_grade, visibility, owner_id, register_version, status). `pack = 'bank'` is allowed and bank codes must end in -9NN (phase-06.md:79, 97-98).
- T2 Route: PASS - `POST /api/v1/scenarios`, scope config.author, action `scenario.create` (phase-16.md:884). Cedar permits CFG only (phase-16.md:1631-1636).
- T3 Repository and types: GAP (see a, b). `NewScenario` fields are listed (phase-16.md:728). No repository method inserts one bank scenario; `ScenarioAdminService` appears only in a heading (phase-16.md:1471).
- T4 Logic: GAP (see a, c). Steps 1-2 are numbered (phase-16.md:1493-1497), but the duplicate-code refusal is not written and the `owner_not_active` refusal has no code.
- T5 Audit: PASS - "and audit `scenario.created`" (phase-16.md:1497); listed at phase-16.md:1590.
- T6 Cross-references: PASS - phase 06 `scn.scenarios` (phase-06.md:72), the 18 families (phase-06.md:366), DSL code `E_NAMESPACE` (DSL.md:1111). The owner check follows phase 06's pattern (phase-06.md:611), see c.
- T7 Consistency: PASS - no 7.x block changes S-05 (blocks 7.1 to 7.12 checked).
GAP V-16-01-a | MEDIUM | T3 | No named insert for one bank scenario. Phase 06's `ScenarioRepository::insert_missing` (phase-06.md:402) is a bulk insert of absent codes, so a duplicate POST would be skipped silently. `scn.scenarios.code` is UNIQUE (phase-06.md:74), and no duplicate-code refusal is written. | 3.15 Bank scenarios step 2 (phase-16.md:1496-1497) and the section 2 repository list (phase-16.md:734-827) | greps: `ScenarioRepository` and `ScenarioAdminService` in phase-16.md (heading 1471 and AppState 1781 only); `duplicate`, `409`, `23505` in 3.15 (none).
GAP V-16-01-b | MEDIUM | T3 | The POST body is named without fields: `ScenarioCreate` (phase-16.md:868). `NewScenario` (phase-16.md:728) has no document field, yet step 2 installs "the given document" as draft 1.0.0 (phase-16.md:1497). | section 2 utoipa schemas (phase-16.md:868) | greps: `ScenarioCreate` in phase-16.md (line 868 only); `document` in the NewScenario line (728, none).
GAP V-16-01-c | MEDIUM | T4 | The refusal `owner_not_active` has no HTTP status and no problem code (phase-16.md:1494). Phase 06 gives the same refusal as 422 `E_VALIDATION` with reason `owner_not_active` (phase-06.md:611). | 3.15 Bank scenarios step 1 (phase-16.md:1494) and the 3.17 list (phase-16.md:1575-1578) | greps: `owner_not_active` in phase-16.md (line 1494 only); 3.17 list (none).

### V-16-02 Known-answer seeded suite   (source: header "Requirements carried", S-49, line 28; result: PASS)
- T1 SQL: PASS - table `scn.seeded_tests` (phase-16.md:354-373) with `catalogue_item_id`, `scenario_id`, `case_code`, `polarity`, `fixture`, `expected`, `last_run_version_id`, `last_result`, `consecutive_failures`; UPDATE grant on the run columns (phase-16.md:553-554); schedule `schedule.seeded_suite_daily` (phase-16.md:610-611).
- T2 Route: PASS - `GET /api/v1/seeded-tests` (config.author, config.approve; `scenario.seeded.read`) and `POST /api/v1/seeded-tests/runs` (config.author; `scenario.seeded.run`) (phase-16.md:905-906). Cedar: CFG and MRO may run (phase-16.md:1638-1640); CFG, MRO, CHK, M, CH may read (phase-16.md:1651-1654).
- T3 Repository and types: PASS - `SeededRepository` with `active_for`, `record(tx, id, rec: &SeededRecord)`, `list` (phase-16.md:816-820). The fields of `SeededRecord` are the columns written in 3.12 step 4 (phase-16.md:1399-1400), so not flagged.
- T4 Logic: PASS - steps 1-4 are numbered (phase-16.md:1391-1401): one transaction with rollback, pass, fail and error rules, consecutive-failure count. Job concurrency is 1 (phase-16.md:1755).
- T5 Audit: PASS - `seeded_test.result` on fail, error and the first pass after failures; `seeded_suite.completed` once per run (phase-16.md:1400-1401); both listed at phase-16.md:1601.
- T6 Cross-references: PASS - `scn.cbi_catalogue_items` (phase-06.md:48); `coalesce_key` on `admin.jobs` (phase-00.md:111, 122-123); the pack seed enqueues the suite (phase-16.md:1530).
- T7 Consistency: PASS - no 7.x block changes the suite; 7.9 touch keys cover incremental, settled and shadow runs only (phase-16.md:1988).

### V-16-03 Threshold change governance for parameter sets   (source: header "Requirements carried", L-18, line 39; result: GAPS 4)
- T1 SQL: PASS - `scn.parameter_sets` (phase-16.md:113-135): `set_version`, `values`, `basis`, `band_status`, `reason_code` (FK to `ref.reason_codes`, phase-04.md:58), `reason_tag`, `reason_text`, `status`, `change_request_id`. Values, basis and reason are immutable once proposed (phase-16.md:470-474).
- T2 Route: PASS - `GET` and `POST /api/v1/scenarios/{id}/parameter-sets` (config.author; `scenario.parameter_set.read` and `.write`) (phase-16.md:891-892). The proposal goes through phase 04's generic `POST /change-requests` (phase-04.md:645, 778).
- T3 Repository and types: PASS - `ParameterSetRepository` (insert, next_version, active, last_superseded, move_status, list) (phase-16.md:754-762); `NewParameterSet` and `ParameterSetCreate` are named (phase-16.md:731, 871).
- T4 Logic: GAP (see a, b). Drafting steps 1-6 are numbered (phase-16.md:1094-1107) and the change class is at phase-16.md:1110-1116. The reason-code refusal has no code, and no lock is stated for the `next_version` step.
- T5 Audit: PASS - `parameter_set.drafted` (phase-16.md:1104) and `parameter_set.activated` (phase-16.md:1119); both in the event table (phase-16.md:1597). The proposal's status move is audited by phase 04 (`change_request.*`, phase-16.md:1608).
- T6 Cross-references: PASS - `E_CAPACITY_NO_BTL` (phase-04.md:828-829); `cr_minor` and `cr_material` (phase-04.md:1264-1265); `TUNE_RECALIBRATION_PCT` seeded in phase 04 (phase-04.md:107); `ref.reason_codes` (phase-04.md:58). See c for the BTL wiring.
- T7 Consistency: PASS - no 7.x block changes parameter sets; 3.5 and DR-113 (research.md:124) both make a volume decrease material.
GAP V-16-03-a | MEDIUM | T4 | A reason code that is inactive or outside family `tuning` or `change` has no refusal code at drafting. Phase 04 refuses that case with 422 `reason_not_allowed` (phase-04.md:872-873), but only at change-request create, after drafting. | 3.5 Drafting step 4 (phase-16.md:1100) and 3.17 (phase-16.md:1575-1578) | greps: `reason_not_allowed` in phase-16.md (none); the E_ codes in 3.5 cover only bounds, DP and capacity (phase-16.md:1095-1101).
GAP V-16-03-b | MEDIUM | T4 | Drafting reads `next_version` and then inserts, with no transaction or lock stated. Two drafts on one version meet at `uq_parameter_set_version` (phase-16.md:132), and the unique violation has no mapped response. | 3.5 Drafting steps 1-5 (phase-16.md:1094-1104) | greps: `lock`, `FOR UPDATE`, `transaction` in phase-16.md lines 1093-1125 (none).
GAP V-16-03-c | MEDIUM | T6 | The capacity BTL refusal applies only to kinds whose `required_evidence` lists `btl_sample` (phase-04.md:828-829). `Prepared.required_evidence` is set by each target's prepare (phase-04.md:661). The parameter_set target row and its prepare steps do not name it. | 3.14 target table, parameter_set row (phase-16.md:1461), and 3.5 change class (phase-16.md:1110-1116) | greps: `required_evidence` in phase-16.md (none); `btl_sample` in 3.5 and 3.14 (none).
GAP V-16-03-d | MEDIUM | T7 | L-18 names below-the-line evidence for threshold change governance in general (phase-16.md:39). S-39 names it for capacity changes (phase-16.md:21), and 3.5 step 4 applies it to capacity only. No Decision and no DR records the narrower scope for risk-tagged sets. | 3.5 step 4 (phase-16.md:1100-1102) or a Decisions entry | greps: `below-the-line` in DR-113 (research.md:124, none); `L-18` in research.md (DR-113 line only).

### V-16-04 Champion and challenger in shadow (VB-23)   (source: header "Implemented here", line 44; result: GAPS 1)
- T1 SQL: PASS - `scn.shadow_results` (phase-16.md:309-328), partitioned by `created_at`; `scenario_versions.shadow_started_at` and `shadow_days_required` (phase-16.md:81-82; CHECK at 96). `live_alert_id` is written by phase 17 (phase-17.md:1313-1314) and granted to akr_app (phase-16.md:549).
- T2 Route: PASS - `POST /api/v1/scenarios/{id}/shadow` (config.author; `scenario.shadow.run`) (phase-16.md:899); `GET` (config.author, config.approve; `scenario.shadow.read`) (phase-16.md:900). Cedar: CFG runs (phase-16.md:1634); CFG, MRO, CHK, M, CH read (phase-16.md:1651-1654).
- T3 Repository and types: PASS - `ShadowRepository` (append_chunk, compare_day, counts) (phase-16.md:795-802); `ShadowStart` and `ShadowView` (phase-16.md:874, 1376-1378).
- T4 Logic: GAP (see a) for the start refusals. Start steps 1-3 (phase-16.md:1357-1362) and comparison steps 1-4 (phase-16.md:1369-1374) are numbered; run step 9 writes shadow rows and no sink (phase-16.md:1166).
- T5 Audit: PASS - `scenario.shadow_started` (phase-16.md:1361-1362; table 1593); `shadow.compared` (phase-16.md:1373; table 1600). Shadow row writes are counted in `scenario.run_completed` (phase-16.md:1171-1172; table 1602).
- T6 Cross-references: PASS - `schedule.shadow_compare` (phase-16.md:608); phase 17 writes `live_alert_id` (phase-17.md:1313-1314), and its section 3.14 cites phase 16 section 3.11 (phase-17.md:1310).
- T7 Consistency: PASS - 7.9 adds touch keys to shadow runs (phase-16.md:1988); 7.11 keeps the fold on live open buckets (phase-16.md:2010-2020). No contradiction.
GAP V-16-04-a | MEDIUM | T4 | Two start refusals name no code: the version is not `testing`, and its unit cases did not pass (`tests_passed_at` NULL). The 3.17 reason list has no such reason (phase-16.md:1577-1578). | 3.11 Start step 1 (phase-16.md:1357-1358) and 3.17 (phase-16.md:1575-1578) | greps: `not_testing`, `tests_missing`, `must be testing` in phase-16.md (none).

### V-16-05 Dated passing-test gate (L-61)   (source: header "Requirements carried", L-61, line 42; result: GAPS 1)
- T1 SQL: PASS - `tests_passed_at` (phase-16.md:80) with the CHECK that testing, shadow and live rows carry it (phase-16.md:97); `fixtures` (phase-16.md:73); both in the UPDATE grant (phase-16.md:530-533).
- T2 Route: PASS - `POST /api/v1/scenarios/{id}/versions/{v}/test` (config.author; `scenario.version.write`) (phase-16.md:890). Cedar CFG (phase-16.md:1632-1635).
- T3 Repository and types: PASS - `ScenarioVersionRepository::move_status(tx, id, from, to, patch: &StatusPatch)` (phase-16.md:749-750). The `StatusPatch` fields are not listed; they are inferable from 3.3 and 1.1, so not flagged.
- T4 Logic: GAP (see a). Gate G2 (phase-16.md:1071) and the transition row (phase-16.md:1027) give the outcome; no step runs the unit cases.
- T5 Audit: PASS - `scenario.version_tested` (phase-16.md:1591); the edit that clears the pass is `scenario.version_saved` (phase-16.md:1591; clear step at 1028).
- T6 Cross-references: PASS - `E_TESTS_MISSING` (DSL.md:1126); the unit-case format is in DSL A.9 (DSL.md:418-435); the route runs unit and seeded cases (DSL.md:1209).
- T7 Consistency: PASS - 3.3 "testing to draft clears tests_passed_at" (phase-16.md:1028) matches G2 (phase-16.md:1071); no 7.x block changes the gate.
GAP V-16-05-a | MEDIUM | T4 | `POST .../test` has no run steps. Section 3.12 runs seeded cases only, and no unit-case run, pass criterion or failure response is written. DSL A.9 gives the case format (`fixture`, `expect`) and DSL.md:1209 says the route runs the cases. DSL.md:431 also adds studio-generated null, currency, reversal and late-data cases, which the phase does not carry. | 3.3 transition row (phase-16.md:1027) and a route step list beside 3.12 (phase-16.md:1384-1410) | greps: `unit` in phase-16.md lines 1384-1410 (none); `automatic` and `null, currency` in phase-16.md (only "written automatically", line 1861).

### V-16-06 Islamic pack in gate G9 (block 7.2)   (source: gap-pass block 7.2, run 3b header at line 1944; result: GAPS 1)
- T1 SQL: PASS - `pack` CHECK includes `islamic` (phase-06.md:79); `requires_flag` column (phase-06.md:89); the flag `pack.islamic` is seeded (phase-06.md:318).
- T2 Route: N/A - the block adds no route (go-live is a `scenario` change request, phase-16.md:1460).
- T3 Repository and types: N/A - the block adds no method or type.
- T4 Logic: GAP (see a). The G9 extension (phase-16.md:1950-1952) and the seed skip (phase-26.md:493-494) are stated; the refusal has no status or code.
- T5 Audit: N/A - the block names no mutation.
- T6 Cross-references: PASS - phase 26 section 3.7 (phase-26.md:488-498) and its verify-data rule `islamic_sole_trigger` (phase-26.md:555-556).
- T7 Consistency: PASS - the block extends G9 (phase-16.md:1078) and says phase 26 owns the extension (phase-16.md:1951); 3.3 step 2 re-runs the gate, so activation is covered.
GAP V-16-06-a | MEDIUM | T4 | The refusal `pack_not_enabled` (phase-16.md:1952; phase-26.md:498) has no HTTP status and no problem code, and it is not in the 3.17 reason list. | 3.17 reason list (phase-16.md:1577-1578) with 3.4 G9 (phase-16.md:1078) | greps: `pack_not_enabled` in phase-16.md (line 1952 only); 3.17 reason list (none).

### V-16-07 Protected-attribute check on a scenario change (block 7.3)   (source: gap-pass block 7.3, run 3b header at line 1944; result: GAPS 1)
- T1 SQL: N/A - the block adds no table or column.
- T2 Route: N/A - the block adds no route.
- T3 Repository and types: PASS - `check_before_decision` is the phase 04 `ChangeTarget` method (phase-04.md:651), re-run by the scenario target per 3.14 (phase-16.md:1464-1466).
- T4 Logic: GAP (see a). The block states the call and that a finding refuses the decision (phase-16.md:1956-1957); no status or code is given.
- T5 Audit: N/A - the block names no mutation; a refused decision keeps the phase 04 audit (phase-16.md:1608).
- T6 Cross-references: PASS - `ProtectedAttributeCheck` is phase 24 section 3.5 (phase-24.md:667-671); phase 24 Open doubts 13 defers the gate to phase 16 (phase-24.md:1489).
- T7 Consistency: PASS - the block says the check is not part of GoLiveGate (phase-16.md:1957), which matches the gate table (phase-16.md:1068-1080).
GAP V-16-07-a | MEDIUM | T4 | "a finding refuses the decision" names no status and no problem code. Phase 24 section 3.5 defines the output as a list of refusals with no code (phase-24.md:671). | block 7.3 (phase-16.md:1956-1957) and the common rules of 3.14 (phase-16.md:1464-1466) | greps: `ProtectedAttributeCheck` in phase-16.md (block only); a problem code for the check in phase-24.md section 3.5 (none).

### V-16-08 DetectionPayload source kind (block 7.4)   (source: gap-pass block 7.4, run 3b header at line 1944; result: PASS)
- T1 SQL: PASS - `observational BOOLEAN` is phase 06's column (phase-06.md:86); the family rule `family ~ '^[A-Z]{2,5}$'` and the 18 families are in phase 06 (phase-06.md:75, 366).
- T2 Route: N/A - the block adds no route.
- T3 Repository and types: PASS - `DetectionPayload` is a section 2 domain type (phase-16.md:699); `source_kind` is named in the block (phase-16.md:1960) and in phase 22 (phase-22.md:820).
- T4 Logic: PASS - the runner writes `scenario` (phase-16.md:1960-1961); the family rule (`^[A-Z]{2,5}$`, one of the 18) matches the register (phase-06.md:75, 366).
- T5 Audit: N/A - the block names no mutation.
- T6 Cross-references: PASS - phase 22 owns the field (phase-22.md:820); `branch.hybrid_scenario_code` is phase 22's setting (phase-22.md:313); DR-382 (research.md:393) matches the block.
- T7 Consistency: PASS - the base payload list (phase-16.md:1208-1219) has no `source_kind`, and the block starts it from phase 22; no contradiction.

### V-16-09 Activation instant (block 7.6)   (source: gap-pass block 7.6, run 3b header at line 1944; result: PASS)
- T1 SQL: PASS - `effective_from TIMESTAMPTZ` on `scn.scenario_versions` (phase-16.md:84); in the UPDATE grant (phase-16.md:531); classified internal (phase-16.md:628).
- T2 Route: N/A - activation runs inside the approval transaction (phase-16.md:1051-1059).
- T3 Repository and types: PASS - activation sets `effective_from` through `ScenarioVersionRepository::move_status` (phase-16.md:749-750, 1056).
- T4 Logic: PASS - activation sets it (phase-16.md:1056); kill-switch reinstatement sets it too (phase-16.md:1424). The card reads "live since" (phase-16.md:1548).
- T5 Audit: PASS - `scenario.version_activated` (phase-16.md:1058; table 1594) and `scenario.kill_switched` (phase-16.md:1595).
- T6 Cross-references: PASS - phase 24 Open doubts 30 asks for this column and says phase 16 names it in its DDL (phase-24.md:1506).
- T7 Consistency: PASS - the block states what base 3.3 step 4 and 3.13 step 6 already write (phase-16.md:1056, 1424); no conflict.

### V-16-10 One run per touch (block 7.9)   (source: adversarial pass run 2, block 7.9, header at line 1982; result: GAPS 2)
- T1 SQL: GAP (see a). `touch_id UUID` with its CHECK and the replaced unique index are stated (phase-16.md:1986-1987), so the column and index pass. The column has no classification row (see a).
- T2 Route: N/A - the block adds no route; the fan-out and run payloads are covered under T4 (see b).
- T3 Repository and types: PASS - `RunRepository::start_once(tx, r: &NewRun)` (phase-16.md:767) carries the key through `NewRun`; the stated index (phase-16.md:1987) is the conflict target.
- T4 Logic: GAP (see b). The rules are stated: a second delivery is a no-op, a new touch starts its own run, and a late touch takes the settled path (phase-16.md:1990-1994; late data at phase-16.md:1176-1178). The fan-out input does not name `touch_id`.
- T5 Audit: N/A - no new mutation; run events are in 3.6 steps 11-12 (phase-16.md:1170-1174).
- T6 Cross-references: PASS - phase 11 section 7+ adds `touch_id` to `canon.touched` (phase-11.md:1543-1544) and says phase 16 keys its run ledger on it (phase-11.md:1552); DR-408 (research.md:419).
- T7 Consistency: PASS - the block replaces the 1.2 batch key (phase-16.md:205-208) and the 4.1 coalesce keys (phase-16.md:1749-1750), and says so (phase-16.md:1987-1989).
GAP V-16-10-a | MEDIUM | T1 | `touch_id` has no `admin.column_classification` row. The 0057 list for `scn.scenario_runs` (phase-16.md:633-636) classifies the other columns and not `touch_id`. Phase 00 requires a row for every column of the AKR schemas (phase-00.md:797-798); DATA-MODEL tags every column (DATA-MODEL.md:48). | 1.4 migration 0057, scenario_runs rows (phase-16.md:633-636) | greps: `touch_id` in phase-16.md lines 621-668 (none).
GAP V-16-10-b | MEDIUM | T4 | The fan-out input does not name `touch_id`. The 3.6 fan-out step 1 (phase-16.md:1130) and the 4.1 payload of `scenario_touch_fanout` (phase-16.md:1749) list `batch_id, source_id, entity_kinds, business_date`. The `scenario_run` payload (phase-16.md:1750) has no `touch_id`. The block says the fan-out passes it on (phase-16.md:1988), and phase 11 names the field (phase-11.md:1544). | 3.6 fan-out step 1 and table 4.1 (phase-16.md:1130, 1749-1750) | greps: `touch_id` in phase-16.md outside the block (none; the only hits are lines 1985-1999).

### V-16-11 Retried run takes over its row (block 7.10)   (source: adversarial pass run 2, block 7.10, header at line 1982; result: GAPS 2)
- T1 SQL: PASS - the upsert names `status`, `error_code`, `started_at`, `finished_at`, `entities`, `detections`, `amendments`, `version` (phase-16.md:1999-2001), all columns of `scn.scenario_runs` (phase-16.md:181-204) and in the UPDATE grant (phase-16.md:539-540). The conflict target matches the 7.9 index (phase-16.md:1987).
- T2 Route: N/A - the retry is the job; no route.
- T3 Repository and types: PASS - `RunRepository::start_once(tx, r: &NewRun) -> Result<Option<Uuid>, RepoError>` (phase-16.md:767); the block keeps the signature and changes what `None` means (phase-16.md:2003-2004).
- T4 Logic: PASS - takeover condition, lease source, restart at chunk 1, detections stop at the sink as Duplicate, and the earlier failure stays audited (phase-16.md:1996-2008).
- T5 Audit: GAP (see a). The takeover changes the run status with no event.
- T6 Cross-references: PASS for the lease and the reaper: `jobs.lease_s` (phase-00.md:244) and the reaper's `locked_at < now() - jobs.lease_s` (phase-00.md:546-547). GAP for the display rule (see b).
- T7 Consistency: PASS - the block replaces base step 2 "stop" (phase-16.md:1145-1147), and says it does ("None now means", phase-16.md:2003). Base "five attempts" (phase-16.md:1173) and "after the last attempt" (phase-16.md:2006) agree.
GAP V-16-11-a | MEDIUM | T5 | The takeover moves the run from `failed` or an expired `running` back to `running` through an UPDATE, and it names no audit event. 3.17 says "one per mutation" (phase-16.md:1585) and lists no run-start event. | 3.17 table (phase-16.md:1602) or block 7.10 (phase-16.md:2004-2006) | greps: `run_started`, `run_begun`, `run_start` in phase-16.md (none); `scenario.run_` rows in 3.17 (completed, failed, refused_cost only).
GAP V-16-11-b | LOW | T6 | "the scenario shows 'not operating as intended' (section 3.16)": section 3.16 has no such rule (phase-16.md:1536-1571). The phrase is used at phase-16.md:1084, 1158 and 1239, and DSL A.3 is its source. | block 7.10 (phase-16.md:2006-2007) or 3.16 | greps: `not operating` in phase-16.md lines 1536-1571 (none).

### V-16-12 Fold window and closed-case skip (block 7.11)   (source: adversarial pass run 2, block 7.11, header at line 1982; result: GAPS 1)
- T1 SQL: PASS - `scn.detection_keys` has `scenario_id`, `anchor_hk`, `bucket_start` and `fingerprint` (phase-16.md:332-348) with the fold index (phase-16.md:349-350). The closed-case join uses `alert.alert_index` (phase-17.md:181), `alert.alerts` (phase-17.md:106), `casework.case_alerts` (phase-17.md:352) and `casework.cases.status` with value `closed` (phase-17.md:298).
- T2 Route: N/A - the block adds no route.
- T3 Repository and types: PASS - `DetectionKeyRepository::open_bucket(tx, scenario, anchor_hk, since)` (phase-16.md:811-812) gets the `until` bound the block states (phase-16.md:2011-2012).
- T4 Logic: PASS - the bounded window, the closed-case skip, the `New` outcome and the late-row path (phase-16.md:2010-2020); base fold at phase-16.md:1193-1197 and outcomes at 1200-1204.
- T5 Audit: PASS - `detection.recorded` for a new bucket (phase-16.md:1201-1202; table 1603); `case.linked_prior` (phase-17.md:1035; table 1361); `case.alert_amended_after_decision` (phase-17.md:1298).
- T6 Cross-references: GAP (see a). The block names phase 17 as the owner of the skip (phase-16.md:2014-2016). Phase 17 defines the join tables but no step adds the skip. The other names hold: `case.linked_prior` in 3.5 (phase-17.md:1025-1035), DR-140 (research.md:151), and the amendment rule in 3.13 (phase-17.md:1290-1298).
- T7 Consistency: PASS - the block adds an upper bound to the base "starts within the span" (phase-16.md:1193-1197); the `New` and amended outcomes match 3.7 and the late-data rule (phase-16.md:1176-1178, 1200-1204).
GAP V-16-12-a | MEDIUM | T6 | The closed-case skip is assigned to phase 17 "From phase 17 on" (phase-16.md:2014-2016), but phase 17 has no `open_bucket` text and no fold or intake step that skips a closed case. Without it, a repeat after closure folds into the closed alert. | phase 17 section 3.1 intake (phase-17.md:875-912) or section 3.13 (phase-17.md:1290), the owner of the fold | greps: `open_bucket` in phase-17.md (none); the bucket lines of phase-17.md (883, 920, 978, 1337, 1574, 1862) carry no closed-case skip.

## Summary
- Items checked: 12 (5 header items: V-16-01 to V-16-05; 4 gap-pass blocks: V-16-06 to V-16-09, from 7.2, 7.3, 7.4, 7.6; 3 adversarial run 2 blocks: V-16-10 to V-16-12, from 7.9, 7.10, 7.11).
- Items fully PASS: 3 (V-16-02, V-16-08, V-16-09).
- GAP lines by severity: HIGH 0, MEDIUM 15, LOW 1 (16 in total).

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-16-01-a | MEDIUM | T3 | No named insert for one bank scenario; no duplicate-code refusal | 3.15 step 2; section 2 repository list |
| V-16-01-b | MEDIUM | T3 | `ScenarioCreate` body has no fields; no field for the first document | section 2 utoipa schemas (line 868) |
| V-16-01-c | MEDIUM | T4 | `owner_not_active` has no HTTP status or problem code | 3.15 step 1; 3.17 |
| V-16-03-a | MEDIUM | T4 | Reason-code refusal at drafting has no code | 3.5 drafting step 4 |
| V-16-03-b | MEDIUM | T4 | No transaction or lock for `next_version`; unique violation unmapped | 3.5 drafting steps 1-5 |
| V-16-03-c | MEDIUM | T6 | parameter_set target does not declare `required_evidence` (btl_sample) | 3.14 target table; 3.5 change class |
| V-16-03-d | MEDIUM | T7 | BTL evidence scoped to capacity with no Decision or DR for the narrowing | 3.5 step 4 or Decisions |
| V-16-04-a | MEDIUM | T4 | Shadow-start refusals (not testing, tests not passed) have no code | 3.11 step 1; 3.17 |
| V-16-05-a | MEDIUM | T4 | Test route has no run steps, pass criteria or failure response; DSL automatic cases not carried | 3.3 row; beside 3.12 |
| V-16-06-a | MEDIUM | T4 | `pack_not_enabled` refusal has no status or code and is not in the reason list | 3.17 reason list; 3.4 G9 |
| V-16-07-a | MEDIUM | T4 | Protected-attribute refusal has no status or code | block 7.3; 3.14 common rules |
| V-16-10-a | MEDIUM | T1 | `touch_id` has no column classification row | 1.4 migration 0057 (scenario_runs) |
| V-16-10-b | MEDIUM | T4 | Fan-out input and job payloads do not name `touch_id` | 3.6 fan-out step 1; table 4.1 |
| V-16-11-a | MEDIUM | T5 | Takeover (run back to running) names no audit event | 3.17 table; block 7.10 |
| V-16-11-b | LOW | T6 | "not operating as intended" cited to 3.16, which does not define it | block 7.10; 3.16 |
| V-16-12-a | MEDIUM | T6 | Phase 17 does not carry the closed-case skip assigned to it | phase 17 section 3.1 or 3.13 |
