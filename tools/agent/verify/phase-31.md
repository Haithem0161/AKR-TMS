# Verification pass, phase 31 (Haiku draft for lead review)

Phase 31 has no "Adversarial pass" blocks (grep -i "adversarial" finds none). Its 7.x blocks are 7.1, 7.2, 7.3 and 7.4 (gap pass 1), so all four are sampled and three more header items are taken. Header "Implemented here" items are numbered 1 to 14 in the order written.

Items sampled (12):
- V-31-01 UX-41 practice cases and attempts (header item 1; sections 3.1, 3.2, 5.1; routes 1 to 4)
- V-31-02 M-52 capacity inputs and forecast (header item 3; sections 3.3, 3.4; routes 5 to 7)
- V-31-03 W-06 correspondent and validator packs (header item 7; section 3.7)
- V-31-04 W8 correspondent requests and pack attachment (header item 10; section 3.9; routes 12 and 13)
- V-31-05 G-12 SCIM deactivation disables, never offboards (header item 13; sections 3.11, 3.12)
- V-31-06 M-76 simulation harness (header item 6; section 3.5; routes 8 and 9; job capacity_simulation)
- V-31-07 S-31 typology exposure index (header item 9; section 3.8; route 10; job exposure_compute)
- V-31-08 SP-14 SCIM create writes a pending row, link by an administrator (header item 12; section 3.11; routes 14 and 15)
- V-31-09 7.1 KV paths (gap block 7.1)
- V-31-10 7.2 directory link port (gap block 7.2)
- V-31-11 7.3 retention DELETE grants (gap block 7.3)
- V-31-12 7.4 QA sampler citation and surge sampling rule (gap block 7.4)

Evidence keys: "phase-31.md" is this phase; other files are cited as phase-NN.md, API.md, research.md. Quotes are short fragments.

### V-31-01 UX-41 synthetic practice cases and attempts   (source: header "Implemented here" item 1; sections 3.1, 3.2, 5.1; routes 1 to 4; result: GAPS 1)
- T1 SQL: PASS - tables train.practice_cases (phase-31.md:36-50), train.practice_attempts with the choice and reason_codes CHECKs (:55-67), view train.practice_scores (:75-91); setting train.pass_pct (:315). Owned objects used by the logic: admin.idempotency_keys (phase-00.md:188), ref.reason_codes with family 'decision' (phase-04.md:58-60).
- T2 Route: PASS - routes at phase-31.md:452-455 with scope queue.work and Cedar train.practice.read / attempt / own; API.md:85 gives queue.work to A1, A2, M; API.md:739-742 match; Cedar at phase-31.md:782-795.
- T3 Repository and types: PASS - PracticeRepository (phase-31.md:429: active_cases, case, insert_attempt(tx, &PracticeAttempt), own_attempts, course_scores); TrainingService.attempt(ctx, case_id, body, idem) -> AttemptResult (:436); DTOs AttemptBody and AttemptResult (:425).
- T4 Logic: PASS - steps 1 to 7 at phase-31.md:485-491: 404 E_NOT_FOUND, 409 case_retired, 422 decision_or_choice / answer_kind / reason_unknown / reason_limit, scoring, 201. No lock is needed: the table is insert-only and the first-attempt rule orders by attempted_at, id (:79).
- T5 Audit: PASS - train.practice_attempted {case_code, attempt_id, passed} at phase-31.md:490 and in the table at :726; train.cases_seeded at :480 and :727.
- T6 Cross-references: PASS - decision words no_report, utr, str, tfr: phase-18.md:10 and the CHECK at phase-18.md:112; ref.reason_codes family (phase-04.md:60); admin.training_records evidence_ref and CHECK user_id <> recorded_by (phase-24.md:172, :178); scn.scenarios.typologies (phase-06.md:82). Five names checked.
- T7 Consistency: PASS - 7.3 (phase-31.md:1035) keeps practice attempts at SELECT only for akr_retention, which agrees with the "leave only through the retention run" rule of 3.2 rule 6 (:505). No contradiction.
- Gaps:
  GAP V-31-01-a | LOW | T6 and T4 | `passed_first` is bool_or over the first attempts, so it is TRUE when any one case passed; section 5.5 shows it as "passed on first attempt" beside the counts, and no text defines the course-level flag (:503 names the column only) | section 3.2 rule 4 (phase-31.md:503) | greps: `grep -n "passed_first\|passed on first" phase-31.md` gives :83, :84, :503, :952; none of them defines the course-level flag.

### V-31-02 M-52 capacity inputs and forecast   (source: header "Implemented here" item 3; sections 3.3, 3.4; routes 5 to 7; result: GAPS 4)
- T1 SQL: PASS - own tables casework.capacity_inputs (phase-31.md:100-115) and casework.capacity_snapshots (:120-133). Names the logic uses exist: alert.alerts priority_class (phase-17.md:142), triaged_at (:149), touch_due_at (:151), state CHECK (:169); casework.cases family (phase-17.md:293), opened_at (:302), closed_at (:303), assignee_pool_id (:311); casework.queues pool_id, filter, active (phase-17.md:49, :51, :53); report.small_number_threshold (phase-21.md:518); bank.timezone (phase-00.md:238).
- T2 Route: PASS - GET /capacity/inputs (phase-31.md:456, dashboard.read), POST /capacity/inputs (:457, capacity.plan), GET /capacity/forecast (:458, dashboard.read); API.md:743-745 match; Cedar at phase-31.md:797-810 (M edits; CH and IA read).
- T3 Repository and types: GAP (see V-31-02-a). Otherwise PASS: CapacityRepository (phase-31.md:430: live_input, insert_input, supersede_input, upsert_snapshots, insert_run, finish_run); CapacityService (:437: inputs, record_input(ctx, body, if_match, idem), forecast, request_simulation, simulation); DTOs at :419 and :425.
- T4 Logic: PASS - input steps 1 to 5 at phase-31.md:512-516 (428 E_PRECONDITION_REQUIRED, 409 E_VERSION_CONFLICT, 422 occupancy_above_max, 201); measured handling steps 1 to 4 at :519-527; forecast steps 1 to 10 at :533-543 (insufficient_data, suppression); alarm steps 1 to 3 at :546-548; lock order at :514 and :858.
- T5 Audit: PASS for the input write: capacity.input_recorded at phase-31.md:516 and :728. The alarm uses alarm.raised (:742). The hourly snapshot upsert names no audit event: GAP V-31-02-b.
- T6 Cross-references: PASS - AlarmService::raise (phase-19.md:1299); live index uq_alarms_live on COALESCE(stage, 0) (phase-19.md:325); supervisor.alarm notice (phase-22.md:607); alarm kind and subject CHECK lists in 0097 (phase-31.md:163-171) contain all the earlier values of phase-20.md:39-45, phase-21.md:377-383 and phase-22.md:42-50; working days Sunday to Thursday match the weekend [5, 6] of phase-09.md:108. The NewAlarm subject shape differs from phase 22: GAP V-31-02-d.
- T7 Consistency: PASS - the forecast window and W (phase-31.md:533-534) match the Sunday bank week of 3.3 (:103); capacity.min_working_days (:324) agrees with 3.4 step 5 (:538); the 60-day ageing rule in 3.3 (:525) agrees with Open doubts 4 (:1088). The ForecastQuery signature contradicts 3.4 step 7: GAP V-31-02-c.
- Gaps:
  GAP V-31-02-a | MEDIUM | T3 | `ClassAggregate`, the return type of `CapacityRepository::window_aggregate`, is named but never defined; its fields are not given | section 2.1 (DTO list, phase-31.md:425) or section 2.2 (:430) | greps: `grep -rn "ClassAggregate" docs/plan research docs/*.md` finds only phase-31.md:430.
  GAP V-31-02-b | MEDIUM | T5 | the hourly upsert of casework.capacity_snapshots writes a table and names no audit event; 3.3 says a re-run "does nothing else" (:526) and the 3.14 table has no snapshot kind (:724-740) | section 3.14 (phase-31.md:724-740) or 3.3 step 3 (:526) | greps: `grep -n "capacity_snapshot\|snapshot.*audit" phase-31.md` finds no audit kind for snapshots.
  GAP V-31-02-c | MEDIUM | T7 | 2.3 gives `CapacityService::forecast(ctx, query: ForecastQuery { handling_pct, arrivals_pct })` (phase-31.md:437), but 3.4 step 7 says the sensitivity grid is "fixed grid, not a user input" (:540) and no step reads those two fields; route 458 names no query parameter | section 3.4 step 7 (phase-31.md:540) or section 2.3 (:437) | greps: `grep -rn "ForecastQuery\|handling_pct" docs/plan research docs/*.md` finds only phase-31.md:437.
  GAP V-31-02-d | LOW | T6 | step 3 calls `AlarmService::raise` with `NewAlarm { ..., subject: ('pool', pool_id), ... }` (phase-31.md:548), but the alarm table and phase 22 use subject_kind and subject_id (phase-19.md:303-304; phase-22.md:564) | section 3.4 alarm step 3 (phase-31.md:548) | greps: `grep -rn "NewAlarm {" docs/plan` finds only phase-22.md:564 (subject_kind, subject_id) and phase-31.md:548; phase 19 gives no field list.

### V-31-03 W-06 correspondent and validator packs   (source: header "Implemented here" item 7; section 3.7; result: GAPS 3)
- T1 SQL: PASS - facts read existing objects: admin.releases release_version (phase-25.md:170-172); casework.qa_samples is_calibration (phase-19.md:387); alert.harness_runs kind, passed_gate, recall_clean, ran_at (phase-14.md:326-341); ref.list_versions loaded_at, activated_at (phase-14.md:92, :115-116); casework.qa_reviews (phase-19.md:403); admin.coverage_certificates (phase-12.md:312); admin.validation_packs verdict and sha256 (phase-24.md:103-113); admin.dq_issues severity (phase-12.md:171-179); scn.readiness_grades (phase-06.md:186). Fact F4 names ref.lists instead: GAP V-31-03-a.
- T2 Route: PASS - POST /packs (phase-06.md:488, pack.create) and GET /packs/{id}/download (phase-06.md:490, pack.download with step-up class export by phase-06.md:975-979). The request body for the two new kinds is not written: GAP V-31-03-b.
- T3 Repository and types: PASS - CorrespondentPackService with CorrespondentFacts::build(period) and ValidatorFacts::build(period) (phase-31.md:440); RenderService (phase-02.md:897) and PackService::publish (phase-02.md:915) exist.
- T4 Logic: PASS - facts F1 to F11, each with a source and data_as_of (phase-31.md:583-595); guards 1 to 3 (:598-600). The refusals are job failures (pack.build_failed), not HTTP refusals.
- T5 Audit: PASS - pack.created with the SHA-256 (phase-31.md:581; phase-02.md:920, :980); pack.build_failed (phase-06.md:691, :738); the download audit is phase 06's pack.downloaded (phase-06.md:696).
- T6 Cross-references: PASS - packs_kind_check lists correspondent and validator (phase-06.md:263-265); phase-24.md:240-243 adds certification; the case_reference refusal rule (phase-24.md:805); access_event action download (phase-02.md:151); report.small_number_threshold default 5, floor 3 (phase-21.md:518).
- T7 Consistency: PASS for the facts, the guards and the download class (agrees with phase-06.md:975-979 and research.md:241, DR-230). Wrong section pointer for the Cedar file: GAP V-31-03-c.
- Gaps:
  GAP V-31-03-a | LOW | T6 | fact F4 reads "per list in `ref.lists`" for the versions activated in the period, but loaded_at and activated_at are columns of ref.list_versions (phase-14.md:92, :115-116) | section 3.7 F4 (phase-31.md:587) | greps: `grep -n "ref.lists\|ref.list_versions" phase-31.md` finds :587 and the header; ref.lists is a different table (phase-14.md:65).
  GAP V-31-03-b | LOW | T2 | `POST /packs` for kinds correspondent and validator has no written request body (period, lang) or 202 response; phase 06 documents only integration_profile, cbi_gap and vendor_request (phase-06.md:488); section 4.1 gives only the job payload (phase-31.md:873) | section 3.7 kinds and build (phase-31.md:581) or 4.5 item 5 (:902) | greps: `grep -n "POST /packs\|period.*lang" phase-31.md` finds no body definition.
  GAP V-31-03-c | LOW | T7 | "Cedar: create for M only (section 3.16, E2)" points to section 3.16, which is the settings list; the Cedar file is section 3.17 (phase-31.md:761) | section 3.7 kinds and build (phase-31.md:581) | greps: `grep -n "section 3.16" phase-31.md` gives :457, :468, :910 (all three name API.md section 3.16) and :581; so :581 is the only pointer to this phase's 3.16, which is the settings list (:755), while the Cedar text is in 3.17 (:761).

### V-31-04 W8 correspondent requests and pack attachment   (source: header "Implemented here" item 10; section 3.9; routes 12 and 13; result: GAPS 5)
- T1 SQL: GAP for step 4's "status built" (V-31-04-a). Otherwise PASS: casework.rfis columns kind (phase-19.md:95), direction (:97), recipient_ref (:104), status with draft (:111), version (:127); casework.rfi_packs (own, phase-31.md:153-159); admin.packs kind and the period in scope (phase-02.md:425-427).
- T2 Route: PASS - GET /rfis/correspondents (phase-31.md:462, rfi.desk) and POST /rfis/{id}/packs (:463, rfi.desk); API.md:89 gives rfi.desk to RFI and M; API.md:556-557 match; Cedar at phase-31.md:845-851.
- T3 Repository and types: GAP (V-31-04-b). The DTOs AttachPackBody and CorrespondentRfiRow are named (phase-31.md:425).
- T4 Logic: PASS for the attach steps 1 to 6 at phase-31.md:632-637 (404 masked, 409 not draft, 422 kind_not_correspondent / direction_not_out / pack_not_ready, 200 repeat, version bump, audit). Lock order at :858 (RFI row, then the rfi_packs insert). GAP for missing or stale If-Match (V-31-04-c) and for a pack of another kind (V-31-04-d).
- T5 Audit: PASS - rfi.pack_attached {rfi_id, pack_id} at phase-31.md:636 and in the table at :732.
- T6 Cross-references: PASS - rfi_release_external (phase-19.md:1085); casework.rfis kinds and statuses (phase-19.md:92-127); RfiService (phase-19.md:1060, the RFI desk); the tipping-off rule of phase 19 section 3.14 (phase-19.md:1597). The recipient_ref meaning differs from phase 19 (V-31-04-e).
- T7 Consistency: PASS - the list rule (phase-31.md:629) and 6 item 19 (:1003) agree on the 404 for a suspicion-tier request; 5.3 (:938-942) matches the list and attach rules.
- Gaps:
  GAP V-31-04-a | MEDIUM | T1 | step 4 requires "status built" for the pack, but admin.packs has no status column in any migration: its DDL has no status (phase-02.md:423-438) and no phase adds one | section 3.9 step 4 (phase-31.md:635) | greps: `grep -rln "ADD COLUMN status" docs/plan` finds nothing; `grep -n "ALTER TABLE admin.packs" docs/plan/*.md` finds only the two kind-CHECK rewrites (phase-06.md:262-263, phase-24.md:240-241).
  GAP V-31-04-b | MEDIUM | T3 | `RfiService::attach_pack` and `RfiService::correspondent_list` are named without parameters (phase-31.md:444); no repository method inserts casework.rfi_packs or lists correspondent RFIs (section 2.2, :427-432); phase 19 RfiRepository has insert, get, get_for_update, by_case and desk only (phase-19.md:996) | section 2.2 and 2.3 (phase-31.md:427-444) | greps: `grep -n "rfi_packs\|correspondent_list\|attach_pack" docs/plan/phase-31.md` gives :153, :293, :372, :402, :444, :463, :631, :636, :846, :850, :858, :905, :1007 and no repository method in section 2.2 (:427-432); `grep -n "RfiRepository" docs/plan/phase-19.md` gives :996 only; phase-19.md:1865-1869 (7.4) names the two methods without parameters.
  GAP V-31-04-c | MEDIUM | T4 | `If-Match` is required on POST /rfis/{id}/packs, but no refusal is given for a missing one (428 E_PRECONDITION_REQUIRED) or a stale one (409 E_VERSION_CONFLICT); section 3.13 has no precondition or version row for the pack attach (:718) | section 3.9 attach (phase-31.md:631) and 3.13 (:712-718) | greps: `grep -n "428\|E_PRECONDITION\|E_VERSION_CONFLICT" docs/plan/phase-31.md` gives :512, :712, :1005 only.
  GAP V-31-04-d | LOW | T4 | step 4 covers only pack_not_ready; no refusal is given for a pack whose kind is neither correspondent nor validator (for example board) | section 3.9 step 4 (phase-31.md:635) | greps: `grep -n "pack_not_ready\|kind_not_correspondent" docs/plan/phase-31.md` gives :634 (step 3), :635 (step 4) and :718 (the error table); no row covers a wrong pack kind.
  GAP V-31-04-e | LOW | T6 | recipient_ref is described as the canon.account id of the correspondent (phase-31.md:629), but phase 19's column comment reads "branch, staff or unit id" (phase-19.md:104) | section 3.9 list (phase-31.md:629) or phase 19 column comment | greps: `grep -n "recipient_ref" docs/plan/phase-31.md` gives :629 only as the account meaning.

### V-31-05 G-12 SCIM deactivation disables, never offboards   (source: header "Implemented here" item 13; sections 3.11, 3.12; result: GAPS 3)
- T1 SQL: PASS - admin.directory_users state, linked_user_id, linked_by, linked_at, directory_active (phase-31.md:205-221, 0099); admin.directory_events outcome CHECK with 'deferred' (:233) and reason_code (:234-235); admin.users email, status, disabled_reason (phase-01.md:58-60); admin.user_status values active and disabled (phase-01.md:49).
- T2 Route: PASS - POST /directory/events (phase-31.md:464, scope directory.sync; API.md:110 and :193); Cedar directory.event and user.disable for the service principal (phase-31.md:770-772).
- T3 Repository and types: GAP for DirectoryCtx (V-31-05-b). Otherwise PASS: DirectoryService.event(verified: VerifiedWebhook, body, idem) (phase-31.md:441); DirectoryWebhookVerifier.verify(headers, body, now, key) (:442); DirectoryRepository methods (:432); DirectoryEvent and its outcome and reason types (:423); UserAdminService.disable_from_directory signature (:696).
- T4 Logic: GAP for the deferred outcome (V-31-05-a). Otherwise PASS: event steps 1 to 7 at phase-31.md:656-667 (401, 400 E_IDEMPOTENCY_REQUIRED, 404 with the flag off, lock, apply by op, audit, 200); disable steps 1 to 4 at :697-700; lock order at :858.
- T5 Audit: PASS - directory.event_applied (phase-31.md:666, :734), directory.deactivate_refused (:666, :735), user.disabled with source "directory" (:698; phase-03.md:1042 and :1351).
- T6 Cross-references: PASS - offboard_finalize (phase-03.md:905, :1099); last_holder (phase-03.md:891, :1017); invalidate_user, revoke_all_sessions and set_state(inactive) (phase-03.md:843); Ctx.via value webhook_hmac (phase-03.md:1309-1311); DirectoryLinkQuery and NoDirectory (phase-03.md:1349-1350); identity_admin step-up class (phase-01.md:709).
- T7 Consistency: GAP for the outcome name in the lead correction (V-31-05-c). Otherwise PASS: 3.12 step 2 (no step-up consumed, :698) matches DR-350 (:1069) and the test in 6 item 14 (:998).
- Gaps:
  GAP V-31-05-a | MEDIUM | T4 and T7 | the outcome `deferred` is decided after the commit (3.12 step 3: "After commit ... the outcome is deferred", phase-31.md:699), but the event row is inserted in the same transaction with its outcome (3.11 step 6, :666) and is never updated (0099 comment, :225); the two rules cannot both hold | section 3.11 step 6 (phase-31.md:666) or section 3.12 step 3 (:699) | greps: `grep -n "deferred" docs/plan/phase-31.md` gives :233 and :235 (the 0099 CHECKs), :663, :699, :1081; `grep -n "never updated" docs/plan/phase-31.md` gives :225 only.
  GAP V-31-05-b | MEDIUM | T3 | `DirectoryCtx`, the first parameter of `disable_from_directory`, is named but never defined (phase-31.md:696); the caller is described only as "the directory" (:697) | section 2.1 or section 3.12 (phase-31.md:696) | greps: `grep -rn "DirectoryCtx" docs/plan research docs/*.md` finds only phase-31.md:696.
  GAP V-31-05-c | LOW | T7 | the lead correction says the outcome is `disable_deferred` (phase-31.md:1081), but the event outcome CHECK admits only applied, ignored, refused and deferred (:233), and section 3.11 uses outcome `deferred` with reason `disable_deferred` (:663) | section lead corrections (phase-31.md:1081) | greps: `grep -n "disable_deferred" docs/plan/phase-31.md` gives :235 (the reason CHECK), :663 and :699 (as reasons), :1081 (as an outcome).

### V-31-06 M-76 simulation harness   (source: header "Implemented here" item 6; section 3.5; routes 8 and 9; job capacity_simulation; result: GAPS 5)
- T1 SQL: PASS - casework.capacity_runs (0097, phase-31.md:137-150: status values, result, completed_at, replay_id REFERENCES scn.replays); scn.replays period_from, period_to, status (phase-16.md:215-226); scn.replay_results business_date, would_alert, bucket, segment_ref, detection_key, entity_id (phase-16.md:252-264); casework.touch_by_hours setting (phase-17.md:599).
- T2 Route: PASS - POST /capacity/simulations (phase-31.md:459, capacity.plan, Cedar capacity.simulation.run) and GET /capacity/simulations/{id} (:460, dashboard.read); API.md:746-747 match; Cedar at :797-810.
- T3 Repository and types: GAP for the replay reads (V-31-06-a) and for SimulationInputs and SimulationResult (V-31-06-e). PASS for CapacityService.request_simulation and simulation (phase-31.md:437), SimulationBody and SimulationView (:425), CapacityRepository insert_run and finish_run (:430).
- T4 Logic: PASS for the request steps 1 to 3 (phase-31.md:555-557) and the job steps 1 to 8 (:560-567): 409 replay_not_completed, 422 period_too_long, 202 with Location, failed status on error. GAP for an absent replay (V-31-06-b) and for the running mark (V-31-06-c).
- T5 Audit: PASS for capacity.simulation_requested (phase-31.md:557, :729) and capacity.simulation_completed (:567, :730, with status failed on error). GAP for the running transition (V-31-06-c).
- T6 Cross-references: PASS - POST /api/v1/scenarios/{id}/replays with the cost guard (phase-16.md:893; section 3.9 at :1257); scn.replay_results columns (phase-16.md:252-264); touch_by_hours (phase-17.md:599); report.small_number_threshold (phase-21.md:518); P0 to P3 priority with priority_rule_version (phase-17.md:142, :889).
- T7 Consistency: GAP for the class_mix default (V-31-06-d). Otherwise PASS: the weeks limit of 1 to 8 (phase-31.md:555) matches the 202 for eight weeks or less (6 item 11, :995); the request check (:556) and the job re-check (:560) agree.
- Gaps:
  GAP V-31-06-a | MEDIUM | T3 | the job reads scn.replays (re-read at step 1) and the daily would-alert counts of scn.replay_results (step 2, phase-31.md:560-561), but no repository or port method is named; phase 16's ReplayRepository::results returns entity-level rows, which this section forbids (phase-16.md:774-783) | section 2.2 and section 3.5 job steps 1 and 2 (phase-31.md:560-561) | greps: `grep -n "ReplayRepository\|replay_results" docs/plan/phase-31.md` gives :561 (job step 2) and :1089 (Open doubts 5) and names no repository method.
  GAP V-31-06-b | MEDIUM | T4 | step 2 refuses a replay that is not completed (409 replay_not_completed), but no code is given for a replay id that does not exist | section 3.5 step 2 (phase-31.md:556) and section 3.13 (:711) | greps: `awk 'NR>=554 && NR<=568' docs/plan/phase-31.md | grep -c "404\|E_NOT_FOUND"` gives 0.
  GAP V-31-06-c | MEDIUM | T5 | job step 1 sets the run to running, a status mutation of casework.capacity_runs, and no audit event is named for it (only completed and failed are audited) | section 3.5 job step 1 (phase-31.md:560) and section 3.14 (:730) | greps: `grep -n "running" docs/plan/phase-31.md` gives no running audit kind in the 3.14 table (:724-740).
  GAP V-31-06-d | LOW | T7 | the default class_mix is "the measured mix of section 3.4" (phase-31.md:555), but section 3.4 computes per-class rates only and has no measured mix output (:533-543) | section 3.5 step 1 (phase-31.md:555) or section 3.4 step 10 (:543) | greps: `awk 'NR>=531 && NR<=548' docs/plan/phase-31.md | grep -ci "mix"` gives 0.
  GAP V-31-06-e | MEDIUM | T3 | `SimulationInputs` and `SimulationResult` are used as field types of CapacityRun (phase-31.md:421) but never defined; their fields exist only as prose in steps 1 and 7 (:555, :566) | section 2.1 (phase-31.md:421) | greps: `grep -rn "SimulationInputs\|SimulationResult" docs/plan research docs/*.md` finds only phase-31.md:421.

### V-31-07 S-31 typology exposure index   (source: header "Implemented here" item 9; section 3.8; route 10; job exposure_compute; result: GAPS 2)
- T1 SQL: PASS - own table rpt.typology_exposure (0098, phase-31.md:179-196); canon.txn account_id, business_date, amount_lcy, branch_id (phase-10.md:953, :966, :959, :973); canon.account branch_id and correspondent (phase-10.md:639, :647); canon.balance_snapshot business_date and balance_lcy (phase-10.md:903, :906); alert.alerts matched_txn_ids (phase-17.md:547) and scenario_id (:110); scn.scenarios typologies and visibility with 'normal' and 'restricted' (phase-06.md:82, :90); exposure.trend_months and exposure.trend_band_pp (phase-31.md:332, :334).
- T2 Route: PASS - GET /typology-exposure (phase-31.md:461, dashboard.read, Cedar exposure.read); API.md:748 matches; Cedar exposure.read for M, CH and IA at phase-31.md:817-827.
- T3 Repository and types: GAP for ExposureRunSummary (V-31-07-b). PASS for ExposureService (phase-31.md:439), ExposureRepository upsert_rows and rows (:431), ExposureQuery and ExposureView (:425), TypologyExposureRow and ExposureScope (:422).
- T4 Logic: PASS - definitions and trend (phase-31.md:606-614); suppression (:616); computation steps 1 to 3 (:619-621); read with 422 scope_required and period_range (:623).
- T5 Audit: PASS - exposure.computed {period_month, rows, suppressed_rows} at phase-31.md:621 and in the table at :731. The read writes no audit row, which the section does not require.
- T6 Cross-references: GAP for the partition key (V-31-07-a). Otherwise PASS: the names listed under T1 exist in phase 10, 17, 6 and 21.
- T7 Consistency: PASS - the respondent view (phase-31.md:625) matches 3.7 F10 (:593) and DR-346 (:1061); the threshold of 5 in 6 item 6 (:990) matches the phase 21 default (phase-21.md:518); restricted scenarios excluded (:609) match the visibility values (phase-06.md:90).
- Gaps:
  GAP V-31-07-a | LOW | T6 | step 2 says canon.txn is read "partition pruning by business date", but canon.txn is partitioned by booking_date, so a business_date filter does not prune partitions | section 3.8 computation step 2 (phase-31.md:620) | greps: `grep -n "PARTITION BY" docs/plan/phase-10.md` gives :998 for canon.txn (booking_date; the canon.txn block runs :951-998); only canon.balance_snapshot is partitioned by business_date (:922).
  GAP V-31-07-b | MEDIUM | T3 | `ExposureService::compute(period_month) -> ExposureRunSummary` names a return type that is never defined; its fields are not given beyond the audit detail keys (phase-31.md:621) | section 2.3 (phase-31.md:439) or section 2.1 | greps: `grep -rn "ExposureRunSummary" docs/plan research docs/*.md` finds only phase-31.md:439.

### V-31-08 SP-14 SCIM create writes a pending row; link by an administrator   (source: header "Implemented here" item 12; section 3.11; routes 14 and 15; result: GAPS 3)
- T1 SQL: PASS - admin.directory_users state (phase-31.md:210), linked_user_id (:212), linked_by (:213), linked_at (:214), directory_active (:211); admin.users email (phase-01.md:58), kind (:56), status (:59).
- T2 Route: PASS - GET /directory/users (phase-31.md:465, admin.users; Cedar directory.read for ADM at :774-776); POST /directory/users/{id}/link (:466, admin.users; Cedar directory.link; step-up identity_admin); API.md:194-195 match; API.md:102 gives admin.users to ADM only.
- T3 Repository and types: PASS - DirectoryRepository.link(tx, id, user_id, by, at, version) (phase-31.md:432); DirectoryService.link(ctx, id, body, assertion, if_match, idem) and list (:441); LinkBody and DirectoryUserView (:425).
- T4 Logic: PASS for the link steps 1 to 5 (phase-31.md:670-674): 401 E_STEPUP_REQUIRED, 409 not_pending / already_linked / directory_inactive, 422 user_not_active / email_mismatch, 403 E_SELF_APPROVAL, then the update. The create step for an existing row (:661) is covered. GAP for If-Match refusals (V-31-08-a).
- T5 Audit: PASS - directory.linked {directory_user_id, user_id} at phase-31.md:674 and in the table at :736.
- T6 Cross-references: PASS - admin.users email (phase-01.md:58); principal kind 'human' (phase-01.md:45); identity_admin class (phase-01.md:709); POST /admin/users (phase-03.md:501); E_SELF_APPROVAL and E_STEPUP_REQUIRED (API.md:968, :971). The grant route is attributed to the wrong phase: GAP V-31-08-c.
- T7 Consistency: GAP for the mismatch mechanism (V-31-08-b). Otherwise PASS: DR-349 (phase-31.md:1067) and the create rule in 2.3 (:441) agree; 6 item 12 (:996) matches the create-as-pending rule.
- Gaps:
  GAP V-31-08-a | MEDIUM | T4 | POST /directory/users/{id}/link takes If-Match (the directory row version, phase-31.md:669), but no 428 or 409 refusal is given: section 3.13 has no precondition or version row for DirectoryError (:714-717) | section 3.11 link step 1 or section 3.13 (phase-31.md:669, :714-717) | greps: `grep -n "428\|E_PRECONDITION\|E_VERSION_CONFLICT" docs/plan/phase-31.md` gives :512, :712, :1005 only.
  GAP V-31-08-b | MEDIUM | T7 | a directory e-mail that differs from a linked account "is recorded as directory.mismatch" (phase-31.md:692; DR-351 at :1071; Open doubts 18 at :1102), but the update step records only directory.event_applied with outcome applied (:662, :666), and directory.mismatch is written only by the reconciliation for offboarded accounts (:683) | section 3.11 update step (phase-31.md:662) or conflicts (:692) | greps: `grep -n "mismatch" docs/plan/phase-31.md` gives :683, :692, :738 for the directory mismatch audit (:673 and :716 are the link e-mail check; :1071 and :1102 repeat the rule); only :683 and :738 tie the audit to a job count, and no line gives a per-event write.
  GAP V-31-08-c | LOW | T6 | the grant route is attributed to phase 03 ("phase 03 POST /admin/users/{id}/grants", phase-31.md:676), but phase 04 owns that route (phase-04.md:806) and phase 03 points to phase 04 for it (phase-03.md:524) | section 3.11 "state of a directory-created person" (phase-31.md:676) | greps: `grep -n "admin/users/{id}/grants" docs/plan/phase-03.md` gives :503 (DELETE only), :524 (the POST is phase 04's route) and :1385; no POST grant row exists in phase 03's route table.

### V-31-09 7.1 KV paths   (source: gap block 7.1, phase-31.md:1018-1020; result: PASS)
- T1 SQL: N/A - a path mapping; the block has no table, column or migration.
- T2 Route: N/A - no route in the block.
- T3 Repository and types: N/A - no type or method in the block.
- T4 Logic: PASS - the block states the rule "KV paths are relative to the one mount akr/" (phase-31.md:1019-1020); the webhook key is read from kv/polis/directory at start (:883) and the akr-api AppRole reads it (:892).
- T5 Audit: N/A - no mutation.
- T6 Cross-references: PASS - phase-00.md:893-896 (7.5 one KV mount, DR-356); phase-00.md:900 (akr-api reads polis/directory); phase-00.md:908 (akr-polis reads ory/polis and polis/keys); phase-25.md:822 (kv/kratos/oidc/<provider>); research.md:367 (DR-356).
- T7 Consistency: PASS - the block's mapping matches phase 00 section 7.5 in substance ("the spellings kv/... (phases 25 and 31) ... mean the same paths under akr/"); the base text (3.10 at phase-31.md:647, 4.4 at :892) keeps the kv/ spelling that 7.5 explains; no contradiction.
- Gaps: none.

### V-31-10 7.2 directory link port   (source: gap block 7.2, phase-31.md:1026-1030; result: GAPS 2)
- T1 SQL: PASS - the query reads admin.directory_users linked_user_id and directory_active (phase-31.md:211-212, 0099).
- T2 Route: N/A - a port, not a route.
- T3 Repository and types: PASS - DirectoryLinkQuery::is_inactive(user_id) -> bool is named with its parameter (phase-31.md:1027) and matches phase 03 (phase-03.md:1349-1350). The wiring into AppState is missing from section 4.3: GAP V-31-10-b.
- T4 Logic: PASS - the rule is one sentence: TRUE when a row has linked_user_id = user_id and directory_active = FALSE, else FALSE (phase-31.md:1027-1028); the test is at :1029-1030.
- T5 Audit: N/A - a read-only query.
- T6 Cross-references: PASS - phase-03.md:1349-1350 names DirectoryLinkQuery and NoDirectory (which returns false); phase-31.md:1029 names NoDirectory as the phase 03 fallback.
- T7 Consistency: PASS for the enable refusal (phase-31.md:901 in 4.5 item 4 agrees with the block). GAP V-31-10-a: the block says section 3.12 implements the port, but section 3.12 (:694-702) does not say so.
- Gaps:
  GAP V-31-10-a | LOW | T7 | the block says "Section 3.12 implements phase 03's DirectoryLinkQuery ... as PgDirectoryLinkQuery", but section 3.12 has no text about the port | section 3.12 (phase-31.md:694-702) or the block itself (:1027) | greps: `grep -n "PgDirectoryLinkQuery\|DirectoryLinkQuery" docs/plan/phase-31.md` gives :1027 only.
  GAP V-31-10-b | LOW | T3 | the block says the port is "wired into AppState when pack.polis is on", but the AppState additions list (section 4.3) does not name it (phase-31.md:883-885) | section 4.3 (phase-31.md:883-885) | greps: `grep -n "AppState" docs/plan/phase-31.md` gives :883-885 and :1029; the akr-api list at :883 names training, capacity, exposure, directory and directory_verifier, and not the port.

### V-31-11 7.3 retention DELETE grants   (source: gap block 7.3, phase-31.md:1034-1038; result: GAPS 1)
- T1 SQL: PASS - the grants in 0100 match the block: akr_retention SELECT only on train.practice_attempts (phase-31.md:283) and rpt.typology_exposure (:297); SELECT and DELETE on casework.capacity_snapshots (:289), casework.capacity_runs (:292) and admin.directory_events (:303). Retention classes in 0100 at :396-405.
- T2 Route: N/A - no route.
- T3 Repository and types: N/A - no type or method.
- T4 Logic: PASS - the rule is explicit: SELECT only for the R9 and R7 tables; DELETE for the R12 age tables with no owner sweep, deleted by the engine's record_purge (phase-31.md:1035-1038; phase-23.md:1013-1014).
- T5 Audit: N/A - deletions are written by the phase 23 engine (retention runs and items), not by this block.
- T6 Cross-references: PASS - akr_retention role (phase-23.md:193-197); R7 report_only (phase-23.md:597); R12 age tables and record_purge (phase-23.md:602, :1013-1014); DR-386 (research.md:397).
- T7 Consistency: GAP V-31-11-a (section 6 item 23 expects a deletion path that 7.3 removes). Otherwise PASS: 3.2 rule 6 (phase-31.md:505) and the comment of 0100 (:249) agree with SELECT-only for now.
- Gaps:
  GAP V-31-11-a | LOW | T7 | section 6 item 23 expects "akr_retention deletes practice attempts only through the retention run", but 7.3 and the 0100 grant give akr_retention SELECT only on train.practice_attempts (phase-31.md:283, :1035-1036) | section 6 item 23 (phase-31.md:1007) | greps: `grep -n "akr_retention" docs/plan/phase-31.md` gives :249, :255, :279, :283, :289, :292, :297, :303, :505, :985, :1007, :1035; only :1007 states a deletion path, and :283 and :1035 give SELECT only.

### V-31-12 7.4 QA sampler citation and the surge sampling rule   (source: gap block 7.4, phase-31.md:1040-1041; section 3.6 at :574; section 4.5 item 9 at :906; result: GAPS 1)
- T1 SQL: PASS - capacity.surge_active and capacity.surge_qc_percent (0100, phase-31.md:327-331); casework.qa_samples (phase-19.md:365).
- T2 Route: N/A - no route.
- T3 Repository and types: GAP (V-31-12-a): the input the surge rule replaces is not a named value. The sampler's method is QaSampler::sample_size(C, p, N) (phase-19.md:1236), which has no rate parameter.
- T4 Logic: GAP (V-31-12-a): the rule takes max(its rate, capacity.surge_qc_percent) (phase-31.md:906), but "its rate" has no defined value, and the two operands are not on the same scale.
- T5 Audit: N/A - sample rows and their audit belong to phase 19.
- T6 Cross-references: PASS for the citation: phase-19.md:1234 is section 3.8 (QA sampling, QaSampler), as the block says. The settings that exist are qa.confidence (phase-19.md:827), qa.expected_defect_rate (:829) and qa.suppression_sample_percent (:835); none is named as the surge base.
- T7 Consistency: PASS for the citation: section 3.6 (phase-31.md:574), section 4.5 item 9 (:906), section 3.16 (:759) and the block (:1041) all point to phase 19 section 3.8. The "its rate" wording repeats phase 19's own 7.5 (phase-19.md:1870), so the gap is in both files.
- Gaps:
  GAP V-31-12-a | MEDIUM | T4 and T6 | the surge rule raises the sampler's "rate" to at least capacity.surge_qc_percent, but phase 19 has no single rate: the sample size comes from qa.confidence and qa.expected_defect_rate through QaSampler::sample_size(C, p, N), and the screening population uses qa.suppression_sample_percent; the base named in "its rate" is not defined in either file | section 4.5 item 9 (phase-31.md:906) and section 3.16 (:759); phase 19 section 3.8 | greps: `grep -rn "QA rate\|qa rate\|its rate" docs/plan/*.md` hits the surge wording at phase-31.md:574, :759, :906, :1055 and phase-19.md:1870 only for the sampler; other hits are unrelated (phase-21.md:945 "when its rate arrives" type text, phase-30.md:595 and :620, research.md:336); no line defines a sampler rate setting.

## Summary

- Items checked: 12 (V-31-01 to V-31-12).
- Items fully PASS: 1 (V-31-09; the other items have at least one GAP line).
- GAP lines by severity: HIGH 0; MEDIUM 16; LOW 14; total 30.
- Sampling: header "Implemented here" items 1, 3, 6, 7, 9, 10, 12, 13 (five chosen from the start, middle and end of the list plus three more; the header has no "Requirements carried" item sampled); 7.x blocks 7.1 to 7.4 (all four; there are no "Adversarial pass" blocks in phase 31).

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-31-01-a | LOW | T6 and T4 | passed_first flag is "any case passed" but is labelled "passed on first attempt"; no definition | section 3.2 rule 4 (phase-31.md:503) |
| V-31-02-a | MEDIUM | T3 | ClassAggregate (window_aggregate return type) is never defined | section 2.1 or 2.2 (phase-31.md:425, :430) |
| V-31-02-b | MEDIUM | T5 | hourly capacity_snapshots upsert names no audit event | section 3.3 step 3 and 3.14 (phase-31.md:526, :724-740) |
| V-31-02-c | MEDIUM | T7 | ForecastQuery(handling_pct, arrivals_pct) contradicts the fixed sensitivity grid | section 3.4 step 7 (phase-31.md:540) or 2.3 (:437) |
| V-31-02-d | LOW | T6 | NewAlarm written with subject tuple; phase 22 and the alarm table use subject_kind and subject_id | section 3.4 alarm step 3 (phase-31.md:548) |
| V-31-03-a | LOW | T6 | fact F4 names ref.lists; the version columns are in ref.list_versions | section 3.7 F4 (phase-31.md:587) |
| V-31-03-b | LOW | T2 | POST /packs request body and response for correspondent and validator are not written | section 3.7 (phase-31.md:581) or 4.5 item 5 (:902) |
| V-31-03-c | LOW | T7 | Cedar file cited as section 3.16; it is section 3.17 | section 3.7 (phase-31.md:581) |
| V-31-04-a | MEDIUM | T1 | "status built" for admin.packs; the table has no status column | section 3.9 step 4 (phase-31.md:635) |
| V-31-04-b | MEDIUM | T3 | RfiService attach_pack and correspondent_list without parameters; no repository method for casework.rfi_packs or the list | section 2.2 and 2.3 (phase-31.md:427-444) |
| V-31-04-c | MEDIUM | T4 | If-Match on POST /rfis/{id}/packs: no 428 or 409 refusal | section 3.9 (phase-31.md:631) and 3.13 (:712-718) |
| V-31-04-d | LOW | T4 | no refusal for a pack of a kind other than correspondent or validator | section 3.9 step 4 (phase-31.md:635) |
| V-31-04-e | LOW | T6 | recipient_ref meaning (canon.account id) differs from phase 19's column comment | section 3.9 list (phase-31.md:629) or phase-19.md:104 |
| V-31-05-a | MEDIUM | T4 and T7 | outcome deferred is decided after commit but the event row is written in the same transaction and never updated | section 3.11 step 6 (phase-31.md:666) and 3.12 step 3 (:699) |
| V-31-05-b | MEDIUM | T3 | DirectoryCtx, the first parameter of disable_from_directory, is never defined | section 2.1 or 3.12 (phase-31.md:696) |
| V-31-05-c | LOW | T7 | lead correction names outcome disable_deferred; the event CHECK has deferred with reason disable_deferred | lead corrections (phase-31.md:1081) |
| V-31-06-a | MEDIUM | T3 | no repository or port named for the scn.replays and scn.replay_results reads in the job | section 2.2 and 3.5 job steps 1 and 2 (phase-31.md:560-561) |
| V-31-06-b | MEDIUM | T4 | no code for a replay id that does not exist | section 3.5 step 2 (phase-31.md:556) and 3.13 (:711) |
| V-31-06-c | MEDIUM | T5 | the running status mutation names no audit event | section 3.5 job step 1 (phase-31.md:560) and 3.14 (:730) |
| V-31-06-d | LOW | T7 | default class_mix refers to a "measured mix" that section 3.4 does not produce | section 3.5 step 1 (phase-31.md:555) or 3.4 (:543) |
| V-31-06-e | MEDIUM | T3 | SimulationInputs and SimulationResult are used as field types but never defined | section 2.1 (phase-31.md:421) |
| V-31-07-a | LOW | T6 | canon.txn "partition pruning by business date"; the table is partitioned by booking_date | section 3.8 step 2 (phase-31.md:620) |
| V-31-07-b | MEDIUM | T3 | ExposureRunSummary (compute return type) is never defined | section 2.3 (phase-31.md:439) |
| V-31-08-a | MEDIUM | T4 | If-Match on POST /directory/users/{id}/link: no 428 or 409 refusal | section 3.11 link (phase-31.md:669) and 3.13 (:714-717) |
| V-31-08-b | MEDIUM | T7 | directory e-mail mismatch "recorded as directory.mismatch" has no per-event mechanism | section 3.11 update step (phase-31.md:662) or conflicts (:692) |
| V-31-08-c | LOW | T6 | grant route attributed to phase 03; phase 04 owns it | section 3.11 (phase-31.md:676) |
| V-31-10-a | LOW | T7 | 7.2 says section 3.12 implements PgDirectoryLinkQuery; section 3.12 does not | section 3.12 (phase-31.md:694-702) |
| V-31-10-b | LOW | T3 | 7.2 says the port is wired into AppState; section 4.3 does not list it | section 4.3 (phase-31.md:883-885) |
| V-31-11-a | LOW | T7 | section 6 item 23 expects akr_retention to delete practice attempts; 7.3 gives SELECT only | section 6 item 23 (phase-31.md:1007) |
| V-31-12-a | MEDIUM | T4 and T6 | "its rate" of the phase 19 sampler is not a defined value; sample size and percentage differ in scale | section 4.5 item 9 (phase-31.md:906) and 3.16 (:759); phase-19.md:1236 |
