# Verification pass, phase 19 (Haiku draft for lead review)


## Items selected (12)

Header items (5), taken from the start, middle and end of the "Implemented here" list (phase-19.md:10):
- V-19-01 M-35 watch registry (position 5)
- V-19-02 M-40 bulk reopen (position 8)
- V-19-03 M-44 correspondent RFI desk (position 11)
- V-19-04 M-48 QA workspace (position 15)
- V-19-05 M-53 alarm service (position 18)

Gap-pass 7.x blocks not headed "Adversarial pass" (4, from four different blocks):
- V-19-06 7.1 Branch answers and the release event (phase 22)
- V-19-07 7.2 Assistant QA populations (phase 28)
- V-19-08 7.7 Screening hits and suppression records (DR-375)
- V-19-09 7.9 The outreach.failed topic (TR-20; DR-405)

Adversarial blocks (3, one per run, phase-19.md:1901, 1913, 1925):
- V-19-10 7.10 RFIs of a closed or merged case (run 4)
- V-19-11 7.11 Breach alarms follow the visibility of their subject (run 5)
- V-19-12 7.12 A watch trigger counts once per event (run 6)

Reference notes used by several items: "phase-NN.md:LINE" means the file in docs/plan. Route rows are in section 2 (phase-19.md:1018-1050). Cedar actions are in section 3.13 (phase-19.md:1357-1600). Section 6 curl checks are at phase-19.md:1800-1830.

## Items checked

### V-19-01 M-35 watch registry   (source: header ID M-35, "Implemented here", position 5; result: GAPS 3)
- T1 SQL: PASS - table `casework.watches` at phase-19.md:213-246 holds every column the start, triggers and review use; setting `casework.watch_days` at phase-19.md:801; grant of trigger columns at phase-19.md:783-784 ("trigger_count, last_trigger_at, last_trigger_key"). Decision type `utr` at phase-18.md:142.
- T2 Route: GAP (see V-19-01-a). The start route `POST /api/v1/cases/{id}/watches` (phase-19.md:1024, `queue.work`, `watch.write`) and the review route (phase-19.md:1025, `case.decide`, `watch.review`) are named, and both actions are in the Cedar list at phase-19.md:1372.
- T3 Repository and types: GAP (see V-19-01-b). `WatchService::start_for_decision(tx, decision, case)` is named (phase-19.md:1157). The repository methods `record_trigger`, `set_review`, `due_expiry` are named (phase-19.md:1001). Request type `ReviewWatch` at phase-19.md:986.
- T4 Logic: PASS - start steps 1-6 are numbered with the 409 refusal and sub-code `watch_needs_utr` (phase-19.md:1158, "The decision type is `utr` (else 409 ..."); trigger steps (i) to (vi) with lock order (phase-19.md:1165); expiry (phase-19.md:1167); review refusal 409 sub-code `not_due` and lock order case then watch (phase-19.md:1169).
- T5 Audit: PASS - `watch.started` (phase-19.md:1163), `watch.triggered` (phase-19.md:1165), `watch.expired` (phase-19.md:1167), `watch.reviewed` (phase-19.md:1169); all four are in the 3.12 row at phase-19.md:1348.
- T6 Cross-references: GAP for the canon.touched mapping (see V-19-01-c). PASS for the rest: `casework.decisions` table at phase-18.md:137 with `utr` at phase-18.md:142; `RecordDecisionEffect` step f at phase-18.md:760; `WorkItemService` section 3.6 at phase-18.md:718 with kind `review_evidence` at phase-18.md:191; `BusinessUnit` with Days and WorkingDays at phase-09.md:434; topic `alert.created` at phase-17.md:1543.
- T7 Consistency: PASS - the trigger rules are changed only by 7.12 (phase-19.md:1928-1933), which names the words it replaces (phase-19.md:1932). The 90-day period in DR-152 matches the setting at phase-19.md:801.
- Gaps:
GAP V-19-01-a | LOW | T2 Route | Route `GET /api/v1/watches` names Cedar action `watch.read`, which section 3.13 never defines; the list action in the schema is `watch.list` (System), permitted to A1, A2 and M | section 2 route row (phase-19.md:1023, "(Owned; `watch.read`)") and section 3.13 (phase-19.md:1376, 1408, 1427, 1461) | `grep -n "watch.read" phase-19.md` (hits at 1023 and 1889 only); `grep -c "watch.read" phase-03.md research.md` (0 and 0)
GAP V-19-01-b | MEDIUM | T3 Repository and types | The review, the expiry handler and the trigger dispatch name no service method; only `WatchService::start_for_decision` is named, and the trigger consumer appears only as a type in section 4.3 | section 3.5 (phase-19.md:1165, 1167, 1169) and section 4.3 (phase-19.md:1686) | `grep -n "WatchService::" phase-19.md` (1157, 1646, 1707 are all start_for_decision); `grep -n "WatchTriggerDispatcher" phase-19.md` (1686 only)
GAP V-19-01-c | MEDIUM | T6 Cross-references | The `profile_change` trigger needs the parties of a `canon.touched` batch, but the phase 11 payload carries no party ids and no lookup from batch to party rows is named; phase 10 lineage has `batch_id` (phase-10.md:49) | section 3.5 Triggers (phase-19.md:1165) | `grep -n "party_ids" phase-11.md` (0 hits); the payload at phase-11.md:833-834 is `{batch_id, source_id, source_table, entity_kinds, business_date}`

### V-19-02 M-40 bulk reopen   (source: header ID M-40, "Implemented here", position 8; result: GAPS 3)
- T1 SQL: PASS - table `casework.qa_findings` at phase-19.md:417-432 (severity CHECK includes `critical`; `created_at` for the window); setting `qa.bulk_reopen_max` at phase-19.md:849. Case columns `closed_at` (phase-17.md:303), `assignee_id` (phase-17.md:310); decision column `decider_id` (phase-18.md:145).
- T2 Route: PASS - `POST /api/v1/qa/bulk-reopen` at phase-19.md:1048 (`qa.review`, SU class `decision`, M only). Cedar action `qa.bulk_reopen` is in the System list (phase-19.md:1378) and permitted to M alone (phase-19.md:1461-1466).
- T3 Repository and types: GAP (see V-19-02-a). Request type `BulkReopen` is defined (phase-19.md:991). `CaseService::reopen_under_batch` is named in section 4.5 item 5 (phase-19.md:1706-1712).
- T4 Logic: GAP (see V-19-02-b and V-19-02-c). Lock order and ascending id order are stated (phase-19.md:1282 and the lock order at phase-19.md:1625). The per-case results are stated ("reopened or refused with the sub-code").
- T5 Audit: PASS - `qa.bulk_reopen` is in the 3.12 table (phase-19.md:1351); each reopen writes `case.reopened` and audit `case.reopened` (phase-18.md:862).
- T6 Cross-references: PASS - phase-18.md:854-862 (section 3.13 reopen rules); `StepUpAction` constant `qa.bulk_reopen` in section 4.5 item 11 (phase-19.md:1719); `qa_review` purpose at phase-03.md:141 is not used by this item.
- T7 Consistency: PASS - "M only" at phase-19.md:1282 matches the Cedar permit (phase-19.md:1461-1466) and the late-reopen rule of phase 18 (phase-18.md:858, `case.reopen_late` is M only).
- Gaps:
GAP V-19-02-a | MEDIUM | T3 Repository and types | No repository method reads the finding (severity, created_at) or lists the analyst's closed cases; `QaFindingRepository` has `insert_many`, `set_coaching`, `open_coaching_due` only | section 2 repository traits (phase-19.md:1009) and section 3.9 (phase-19.md:1282) | `grep -n "QaFindingRepository" phase-19.md` (1009 only, no read method); no repository method for the analyst's closed cases (the section 2 repository lines at phase-19.md:995-1011 name none)
GAP V-19-02-b | MEDIUM | T4 Logic | "Closed in the window of the finding" names no window: no start, no length and no setting; DR-155 gives none either | section 3.9 bulk reopen (phase-19.md:1282) | `grep -n "window of the finding" phase-19.md` (1282 only); `grep "| DR-155 " research.md` (no window defined)
GAP V-19-02-c | MEDIUM | T4 Logic | The refusal when `finding_id` is not a critical finding (or does not exist) gives no HTTP status and no problem code; the 3.11 table has no row for it | section 3.9 (phase-19.md:1282) and section 3.11 (phase-19.md:1319-1338) | `grep -n -i "not_critical\|not a critical" phase-19.md` (none); the phrase "critical finding" appears at 3, 10, 848, 1048, 1250, 1282, 1765, 1820 and 1953, and none of those lines gives a refusal code

### V-19-03 M-44 correspondent RFI desk   (source: header ID M-44, "Implemented here", position 11; result: GAPS 2)
- T1 SQL: PASS - table `casework.rfis` at phase-19.md:92-137 has `kind` with `correspondent`, `direction`, `recipient_kind`, `contact_allowed` (DEFAULT FALSE), `clock_unit` with `rfi.response`, `deadline_at`, `extensions`, `answered_at`; `casework.rfi_questions` state at phase-19.md:143-157; setting keys used are in phase-19.md:800-850.
- T2 Route: GAP (see V-19-03-a). Routes used by the correspondent path are named: `POST /cases/{id}/rfis` (phase-19.md:1028, `queue.work`), `POST /rfis/{id}/submit` (phase-19.md:1030), `POST /rfis/{id}/release` (phase-19.md:1033, `rfi.desk`), `POST /rfis/{id}/answers` (phase-19.md:1035, `rfi.desk`). External release for correspondents is M only (phase-19.md:1540).
- T3 Repository and types: PASS - `RfiRepository` at phase-19.md:996; `RfiService::draft`, `submit`, `release`, `record_answers`, `kpis` named at phase-19.md:1060, 1078, 1115, 1131; request types `DraftRfi` and `RecordAnswers` (phase-19.md:980, 983) and view `RfiDeskView` (phase-19.md:993).
- T4 Logic: GAP (see V-19-03-b). Drafting steps 1-7 (phase-19.md:1064-1076) and submit steps 1-6 (phase-19.md:1090-1099) are numbered with errors; the clock and the 50 and 80 percent timers are at phase-19.md:1095-1098.
- T5 Audit: PASS - `rfi.drafted`, `rfi.submitted`, `rfi.released`, `rfi.answer_recorded` appear in the 3.12 table (phase-19.md:1344-1345); each mutation step names its audit event (phase-19.md:1076, 1099, 1123).
- T6 Cross-references: PASS - phase-09.md:771 (`rfi.response`, default 10, bounds 5 to 20); phase-09.md:777 (`rfi.pre_fill`); phase-09.md:797 (`PolicyPackService::scan`); phase-09.md:851 (`rfi.release` decision type); phase-04.md:988 (`WorkflowEngine::start`); phase-22.md:603 (consumer of `rfi.reminder` and `rfi.overdue`).
- T7 Consistency: PASS - section 3.1 step 4 keeps `contact_allowed` FALSE for a correspondent (phase-19.md:1073); section 4.5 item 3 says the pre-fill switch is false in the seed (phase-19.md:1699), which matches phase-09.md:777.
- Gaps:
GAP V-19-03-a | LOW | T2 Route | Route `GET /api/v1/rfis` (desk list) names Cedar action `rfi.read`, which exists only on the Rfi resource; the list action in the schema is `rfi.list` (System), permitted to the RFI role and M | route row phase-19.md:1031 against phase-19.md:1376, 1461, 1494-1495 | `grep -n "rfi.read" phase-19.md` (1374, 1452, 1487, 1527, 1533: all Rfi-resource uses); `grep -n "rfi.list" phase-19.md` (1376, 1461, 1494-1495)
GAP V-19-03-b | MEDIUM | T4 Logic | The pre-fill that M-44 lists as implemented ("pre-fill when `rfi.pre_fill` is on") is never described: the switch is named (phase-19.md:5, 10, 1635, 1699) but no step says what is pre-filled, from where or when | section 3.1 or 3.2 (drafting and the desk), next to section 3.16 (phase-19.md:1635) | `grep -n -i "pre_fill\|pre-fill\|prefill" phase-19.md` (5, 10, 1635, 1699 only); the same grep in phase-09.md, phase-22.md, phase-28.md and research.md finds only the switch at phase-09.md:777

### V-19-04 M-48 QA workspace   (source: header ID M-48, "Implemented here", position 15; result: GAPS 1)
- T1 SQL: PASS - `casework.qa_reviews` at phase-19.md:403-416 (scores, overall, comment_enc, completed_at); `casework.qa_findings` at phase-19.md:417-432 (criterion, severity, target_kind, coaching_state, coaching_due_at, re_review_sample_ids); `casework.qa_samples` state, reviewer_id, grant_id, version (phase-19.md:365-402); `casework.case_acl` revoked_by and revoked_at (phase-18.md:95-99) and reason `qa_review` (phase-19.md:52-53); settings `qa.review_working_days`, `qa.coaching_working_days`, `qa.min_reviewer_items` (phase-19.md:835-850).
- T2 Route: PASS - `POST /api/v1/qa/samples/{id}/review` at phase-19.md:1045 (`qa.review`, Cedar `qa.review.write`); Cedar `followup.reviewer.write` (phase-19.md:1519-1521) admits only the sample's reviewer while the sample is open; `GET /qa/rates` (phase-19.md:1284, M only) and `GET /qa/calibration` (phase-19.md:1286).
- T3 Repository and types: GAP (see V-19-04-a). Named and sufficient: `QaReviewRepository.insert`, `QaFindingRepository.insert_many`, `QaSampleRepository.get_for_update` and `set_state` (phase-19.md:1007-1009); request types `ReviewQaSample` and `FindingInput` (phase-19.md:990); `QaService::review` named in the 3.10 table (phase-19.md:1303-1308).
- T4 Logic: PASS - steps 1-9 are numbered (phase-19.md:1269-1278): lock order (1270); refusals 409 `sample_closed`, 403 `not_sample_reviewer` and the closer refusal (step 2, 1271; the closer refusal maps to E_SELF_APPROVAL in 3.11); 422 `score_mismatch` (step 3, 1272); 422 `finding_criterion_passed` (step 4).
- T5 Audit: PASS - `qa.reviewed` and `qa.finding_recorded` (step 9), `qa.coaching_opened` (step 6), `alarm.raised` for `qa_critical` (step 7, phase-19.md:1276), `qa.grant_revoked` (step 8, phase-19.md:1277); all listed in the 3.12 row at phase-19.md:1351-1352.
- T6 Cross-references: PASS - `casework.case_acl` revoke columns (phase-18.md:95-99, grant at phase-18.md:428); `alert.alerts` `claimed_by` and `triaged_by` (phase-17.md:146, 148); `sod_conflict` user attribute (phase-03.md:572, 600); purpose code `qa_review` (phase-03.md:141).
- T7 Consistency: PASS - the reviewer rule of step 2 (phase-19.md:1271) matches Cedar `followup.reviewer.write` and the review guard; the targeted repeat of at most 10 items (phase-19.md:1250, 1276) is the same rule in 3.8.
- Gaps:
GAP V-19-04-a | MEDIUM | T3 Repository and types | The QA grant revocation names no path. Phase 18's revoke refuses a closed case (E_CASE_SEALED) and most sampled cases are closed, so the QA revoke must not use that path, but phase 19 says only "revoked by QaService" and names no repository method (phase 18 has `CaseAccessRepository::revoke`) | section 3.9 step 8 and the grant expiry paragraph (phase-19.md:1277, 1280), with the closed-case note at phase-19.md:1261 | `grep -n -i "revoke" phase-19.md`: the QA revocation is written only at 1277 and 1280, and no line names a repository method; `grep -n "revoke" phase-18.md` finds phase-18.md:573 (CaseAccessRepository::revoke) and phase-18.md:655 (closed-case refusal) and no QA path

### V-19-05 M-53 alarm service   (source: header ID M-53, "Implemented here", position 18; result: GAPS 3)
- T1 SQL: PASS - table `casework.alarms` at phase-19.md:298-327 holds every column the raise, notice, acknowledge and close steps use; `kind` and `subject_kind` CHECKs match the six kinds and the subjects (phase-19.md:300-303); `closure_reason_code` references `ref.reason_codes(code)` (phase-09.md:34 defines the table); unique live index `uq_alarms_live` (phase-19.md:325); grants at phase-19.md:787-788.
- T2 Route: GAP (see V-19-05-a). `POST /api/v1/alarms/{id}/acknowledge` (phase-19.md:1050, `case.decide` and `audit.read`, Cedar `alarm.acknowledge`) is consistent with the permits at phase-19.md:1442 and 1478.
- T3 Repository and types: GAP (see V-19-05-b). `AlarmRepository` has `insert_live`, `list_visible`, `mark_notified`, `acknowledge`, `close` (phase-19.md:1004); `AlarmService::raise` is named (phase-19.md:1299); `CloseAlarm` and `AlarmView` are defined in section 2.
- T4 Logic: GAP (see V-19-05-c). Breach stages 1-7 are numbered (phase-19.md:1290-1297); the repeat rule of `insert_live` is stated (phase-19.md:1299, "a repeat returns None and writes nothing").
- T5 Audit: PASS - `alarm.raised` (3.12 row at phase-19.md:1352; outbox at phase-19.md:1676), `alarm.notified` (phase-19.md:1310), `alarm.acknowledged` and `alarm.closed` (phase-19.md:1311, 1352).
- T6 Cross-references: PASS - `case_chain_check` job (phase-17.md:1526) and `chain_broken` note (phase-17.md:1733); `timer_alert_touch` (phase-17.md:1524); `AssignmentService` (phase-17.md:1124) and declared-conflict alarm (phase-17.md:1802); `timer_work_item_due` (phase-18.md:726); notice consumer `alarm.raised` (phase-22.md:607); closure family (phase-09.md:738).
- T7 Consistency: PASS - the A2 read limit by kind in Cedar (phase-19.md:1442, `touch_breach` or `work_item_breach`) matches the 3.10 table (phase-19.md:1303-1308); the IA and SEC reads of chain and conflict alarms (phase-19.md:1582-1587) match the table.
- Gaps:
GAP V-19-05-a | MEDIUM | T2 Route | `GET /api/v1/alarms` names Cedar action `alarm.read` (on Alarm). Its list action `alarm.list` (System) is permitted to A2 and M only, so IA and SEC cannot list the chain or conflict alarms that section 6 check (p) expects them to see | route row phase-19.md:1049 against phase-19.md:1378, 1429, 1465 and the IA and SEC permits at phase-19.md:1582-1587 | `grep -n "alarm.list" phase-19.md` (1378, 1429, 1465 only; no permit for IA or SEC); the check (p) at phase-19.md:1823
GAP V-19-05-b | MEDIUM | T3 Repository and types | The acknowledge and close route has no repository method that loads one alarm under FOR UPDATE and applies the 7.11 visibility; `AlarmRepository` has `list_visible` (lists) and the two writes only | section 2 `AlarmRepository` (phase-19.md:1004) and section 3.10 (phase-19.md:1311) | `grep -n "AlarmRepository" phase-19.md` (1004 and 1916, neither with a get or lock method for alarms)
GAP V-19-05-c | MEDIUM | T4 Logic | No refusal is stated for acknowledging an alarm that is not open, or closing one that is already closed; no status or problem code is given | section 3.10 acknowledge bullet (phase-19.md:1311) | `grep -n "alarm" phase-19.md` near 1310-1314 shows no 409 or E_NOT_ALLOWED_IN_STATE; section 3.11 (phase-19.md:1319-1338) has no alarm row

### V-19-06 7.1 Branch answers and the release event (phase 22)   (source: 7.1 block, phase-19.md:1840-1847; result: PASS)
- T1 SQL: PASS - `casework.rfi_answers.evidence_object_id` references `admin.evidence_objects` (phase-19.md:172); the corroboration CHECK (phase-19.md:178) matches the 7.1 rule that the branch path always writes `corroboration = 'none'` (phase-19.md:1842-1845).
- T2 Route: PASS - the desk route `POST /api/v1/rfis/{id}/answers` (phase-19.md:1035) and phase 22's branch route `POST /api/v1/branch/items/{id}/input` (phase-22.md:427, Cedar `branch.item_answer` at phase-22.md:650).
- T3 Repository and types: PASS - `record_answers_in_tx(tx, actor, rfi_id, answers, source_kind)` is named with its parameters (phase-19.md:1843-1844) and phase 22 calls it with the same signature (phase-22.md:496); `AnswerInput` is defined (phase-19.md:983).
- T4 Logic: PASS - the branch path's corroboration is fixed to `none` (phase-19.md:1842-1845); the documented-claim refusal stays at 3.3 step 4 (phase-19.md:1121, 422 `E_VALIDATION` at the evidence pointer).
- T5 Audit: PASS - 3.3 step 6 writes `rfi.answer_recorded` (phase-19.md:1123); the branch path's own audit `branch.item_answered` is in phase 22 (phase-22.md:497).
- T6 Cross-references: PASS - `branch.item_answer` (phase-22.md:427, 650); owner kind `rfi_input` registered by phase 22 section 4.5 item 3 (phase-22.md:819) and used at phase-22.md:486; DR-201 (research.md row: branch answer is a phase 19 claim through `record_answers_in_tx`); DR-385 (research.md row) and phase 22 section 7.2 observation fields (phase-22.md:929).
- T7 Consistency: PASS - 7.1 extends the base step 3 payload of `case.rfi_released` (phase-19.md:1104, `{rfi_id, kind}`) with `recipient_kind` and does not contradict it; the corroboration rule is consistent with the CHECK at phase-19.md:178.

### V-19-07 7.2 Assistant QA populations (phase 28)   (source: 7.2 block, phase-19.md:1850-1855; result: GAPS 4)
- T1 SQL: PASS - phase 28 migration 0089 widens the population and source_kind CHECKs by DROP and ADD of `qa_samples_population_check` and `qa_samples_source_kind_check` (phase-28.md:181-190), which are the default names of the inline CHECKs at phase-19.md:368-370 and 371-372; `ai_involvement` is a column of `casework.decisions` (phase-18.md:158); setting `assistant.qa_sample_percent` is seeded at phase-28.md:335.
- T2 Route: PASS - `POST /api/v1/qa/samples` (phase-19.md:1043) is the draw used by phase 28 (phase-28.md:544).
- T3 Repository and types: GAP (see V-19-07-a). The `population` read itself is covered by `QaSampleRepository.population` (phase-19.md:1007).
- T4 Logic: GAP (see V-19-07-b). The `assistant_golden` rows are defined by phase 28 (phase-28.md:544: drafts whose `eval_run_id` is the run). The assisted rule (7.2 lines 1852-1854) gives rows, rate and seed, but no trigger.
- T5 Audit: PASS - each drawn item writes `qa.sample_drawn` (phase-19.md:1262) and the draw writes the summary; 7.2 adds no new mutation.
- T6 Cross-references: GAP (see V-19-07-c). PASS for `casework.assistant_drafts` (phase-28.md:130) and the draft id as `source_id` (phase-28.md:544).
- T7 Consistency: GAP (see V-19-07-d).
- Gaps:
GAP V-19-07-a | MEDIUM | T3 Repository and types | The closed Rust enums `SamplePopulation` and `SampleSource` do not gain the new values, so `DrawQaSample` cannot carry them; section 2 says the enums match the SQL CHECK lists | section 2 Enums (phase-19.md:977) and `DrawQaSample` (phase-19.md:989) | `grep -n "AssistantGolden\|AssistantAssisted\|AssistantDraft" phase-19.md` (none); `grep -n "assistant_golden" phase-19.md` (1851, 1854, 1855 only, all in 7.2)
GAP V-19-07-b | MEDIUM | T4 Logic | "Drawn by the weekly draw of section 3.8" has no job or schedule: the section 4.1 job table and the seven schedules of section 4.5 item 14 contain no assisted draw | section 4.1 (phase-19.md:1640-1660) and section 4.5 item 14 (phase-19.md:1722) | `grep -n "schedule\." phase-19.md` (seven keys only, 1653-1660 and 852-864); `grep -n "assisted" phase-19.md` (1852 only)
GAP V-19-07-c | MEDIUM | T6 Cross-references | Phase 28 says the item view shows "the verified output with its cited records" and names phase 19 section 4.5 for it; section 4.5 has no such item and 7.2 asks only for the draft text | section 4.5 (phase-19.md:1700-1724) or 7.2 (phase-19.md:1855) | `grep -n "assistant" phase-19.md` (1850-1855 in 7.2, and 2004, a scope line); phase-28.md:544 names "phase 19, section 4.5"
GAP V-19-07-d | MEDIUM | T7 Consistency | Section 2 says the QA item view has "no text except the decision text for an analyst network reader" (phase-19.md:993); 7.2 has the screen show the text of an `assistant_golden` draft (phase-19.md:1855) and the `QaItemView` shape is not changed | section 2 Views (phase-19.md:993) | `grep -n "QaItemView" phase-19.md` (993 only); no draft-text field in the struct

### V-19-08 7.7 Screening hits and suppression records (DR-375)   (source: 7.7 block, phase-19.md:1880-1886; result: GAPS 1)
- T1 SQL: PASS - `alert.pair_memory` (phase-14.md:356) with `last_suppressed_at` (phase-14.md:378); the grant that lets the sweep update it (phase-14.md:444) lists `last_suppressed_at`.
- T2 Route: N/A - an outbox handler and a scheduled job; the item has no route.
- T3 Repository and types: PASS - the sample reads through `QaSampleRepository.population` (phase-19.md:1007); the `screening.hit` consumer is the `watch_trigger_dispatch` job (phase-19.md:1650), whose mapping is at phase-19.md:1165.
- T4 Logic: PASS - the `screening.hit` mapping to `list_hit` is stated (phase-19.md:1165); the population rule is stated in 7.7 (phase-19.md:1881-1885).
- T5 Audit: PASS - a trigger writes `watch.triggered` (phase-19.md:1165, step vi); the draw writes `qa.sample_drawn` (phase-19.md:1262, step 8).
- T6 Cross-references: PASS - `screening.hit` is written by phase 14 from phase 15 on (phase-14.md:1311-1314); phase 15 registers the first handler (phase-15.md:2038-2041); the topic handler registrations form a list that appends (phase-00.md:835-837); DR-375 row in research.md.
- T7 Consistency: GAP (see V-19-08-a).
- Gaps:
GAP V-19-08-a | MEDIUM | T7 Consistency | The sample rule differs by phase. Section 3.8 draws 5 per cent of the week's suppressions at random (phase-19.md:1247). Section 7.7 gives only the population. Phase 14 says 5 per cent "plus every row on a freeze-list entry" (phase-14.md:1318). Phase 19 never states the freeze-list rule | section 3.8 population table (phase-19.md:1247) and section 7.7 (phase-19.md:1881-1885) | `grep -n -i "freeze" phase-19.md` (1859 and 2000 only; neither concerns the suppression sample); `grep -n "freeze" phase-14.md` confirms the rule at phase-14.md:1318

### V-19-09 7.9 The outreach.failed topic (TR-20; DR-405)   (source: 7.9 block, phase-19.md:1893-1901; result: GAPS 2)
- T1 SQL: PASS - `casework.rfis` columns `kind`, `party_id`, `deadline_at`, `answered_at`, `status` (phase-19.md:92-137); `casework.rfi_questions.state` (phase-19.md:143-157) for "still has open questions".
- T2 Route: N/A - an outbox topic written inside a timer handler; the item has no route.
- T3 Repository and types: GAP (see V-19-09-a). The timer handler `timer_rfi_clock` is named (phase-19.md:1644).
- T4 Logic: PASS - the stage, the released and customer_outreach condition, the same-transaction write with `rfi.overdue` and the definition of `prior_failures` are stated (phase-19.md:1894-1896).
- T5 Audit: PASS - the item adds an outbox write only and no domain mutation; the outbox event is named (phase-19.md:1895).
- T6 Cross-references: PASS - phase 15 names `outreach.failed` as a later producer (phase-15.md:2119-2120, "TR-20, phase 19 section 7+"); phase 15 rule 3 makes it a suspicion-linked topic with `TRG_REVIEW_REQUESTED` (phase-15.md:1459-1461); DR-405 row in research.md matches the due-stage write.
- T7 Consistency: GAP (see V-19-09-b).
- Gaps:
GAP V-19-09-a | MEDIUM | T3 Repository and types | The count of earlier failed outreach RFIs of a party needs a query by party and kind over `casework.rfis`; `RfiRepository` reads are `get`, `by_case`, `desk` and `kpi_rows`; none is by party | section 2 `RfiRepository` (phase-19.md:996) and section 7.9 (phase-19.md:1896) | the RfiRepository line (phase-19.md:996) names no by-party method; the by-party reads are AuthorityRepository.for_party (999) and WatchRepository.open_for_party (1001); `grep -n "prior_failures" phase-19.md` (1893-1896 only)
GAP V-19-09-b | LOW | T7 Consistency | 7.9 says "Section 4.2 gains the row", but the section 4.2 outbox table has no `outreach.failed` row | section 4.2 outbox table (phase-19.md:1671-1678) | `grep -n "outreach.failed" phase-19.md` (1893 and 1895 only)

### V-19-10 7.10 RFIs of a closed or merged case (ADV-4-03, ADV-4-04, ADV-4-06)   (source: adversarial run 4, phase-19.md:1901-1911; result: GAPS 2)
- T1 SQL: PASS - `casework.cases.sealed_at` (phase-17.md:304, "phase 18 (M-38, M-61)"); case `status` includes `closed` and `merged` (phase-17.md:298); `casework.rfis.status` includes `released` (phase-19.md:92-137).
- T2 Route: PASS - the refusal is on `POST /api/v1/rfis/{id}/answers` (phase-19.md:1035); the timer handler is `timer_rfi_clock` (phase-19.md:1644).
- T3 Repository and types: PASS - `RfiRepository.by_case` (phase-19.md:996) supplies the RFIs of a merge source; the case lock is the phase 17 `CaseRepository::lock`; `RfiService::record_answers` is named at phase-19.md:1117.
- T4 Logic: GAP (see V-19-10-a). The three steps are numbered and each gives its refusal code (phase-19.md:1904, 409 E_CASE_SEALED; 1906; 1908, 409 E_NOT_ALLOWED_IN_STATE with sub-code `rfi_open`).
- T5 Audit: PASS - a refused answer writes nothing ("No claim is written", phase-19.md:1905), and a suppressed handler writes no `rfi.reminder` or `rfi.overdue`.
- T6 Cross-references: PASS - merge validation of phase 17 section 3.6 (phase-17.md:1076-1090, step 1 with `E_CASE_SEALED` at phase-17.md:1082); invariant I7 (phase-18.md:852); `E_CASE_SEALED` is in API.md at line 981.
- T7 Consistency: GAP (see V-19-10-b).
- Gaps:
GAP V-19-10-a | LOW | T4 Logic | The new sub-code `rfi_open` is not in the closed sub-code list of section 3.11 (phase-19.md:1338-1343). Phase 17 section 7.13 also names it (phase-17.md:1898) | section 3.11 closed list (phase-19.md:1338) | `grep -n "rfi_open" docs/plan/*.md` (phase-19.md:1909 and phase-17.md:1898 only; not in the 3.11 list)
GAP V-19-10-b | LOW | T7 Consistency | Section 4.5 item 4 lists the phase 17 extensions and omits the new merge refusal of phase 17 section 3.6 that 7.10 step 3 adds | section 4.5 item 4 (phase-19.md:1700-1705) | `grep -n -i "merge" phase-19.md` within 1700-1726 (none)

### V-19-11 7.11 Breach alarms follow the visibility of their subject (ADV-5-05)   (source: adversarial run 5, phase-19.md:1913-1923; result: GAPS 1)
- T1 SQL: PASS - `casework.alarms` `subject_kind`, `subject_id`, `case_id` (phase-19.md:298-327); `casework.work_items.case_id` (phase-18.md:190); `casework.case_acl` (phase-18.md:85-100); phase 17 worker and assignee columns (phase-17.md:308-310).
- T2 Route: PASS - `GET /api/v1/alarms` (phase-19.md:1049) and `POST /api/v1/alarms/{id}/acknowledge` (phase-19.md:1050); the 404 `E_NOT_FOUND` answer is in the 3.11 table (phase-19.md:1319-1338).
- T3 Repository and types: GAP (see V-19-11-a). `AlarmRepository.list_visible` is named (phase-19.md:1004) and covers lists only.
- T4 Logic: PASS - the visibility rule for each kind, the 404 answer for other callers and the predicate sources are stated (phase-19.md:1916-1923).
- T5 Audit: PASS - a refused read writes nothing; acknowledge writes `alarm.acknowledged` (phase-19.md:1311, 1352).
- T6 Cross-references: PASS - phase 17 section 3.10 predicate (phase-17.md:1216-1231) and `services::visibility::build` (phase-17.md:1232); phase 18 case predicate with `casework.case_acl` (phase-18.md:799-812); phase 22 section 3.8 step 4 recipients (phase-22.md:587); Cedar `followup.a2.alarm` (phase-19.md:1442).
- T7 Consistency: PASS - 7.11 narrows the base 3.10 table ("A2 for touch breaches", phase-19.md:1303) to the A2 who can see the alert; the block states the rule, and DR-411 (research.md row) says the same.
- Gaps:
GAP V-19-11-a | MEDIUM | T3 Repository and types | The 404 rule for one alarm needs a read or lock of one alarm under the predicate (acknowledge loads it first). `AlarmRepository` has `list_visible` for lists only | section 2 `AlarmRepository` (phase-19.md:1004) and section 7.11 (phase-19.md:1916-1919) | `grep -n "AlarmRepository" phase-19.md` (1004 and 1916, neither with a get or lock method for alarms)

### V-19-12 7.12 A watch trigger counts once per event (ADV-6-08)   (source: adversarial run 6, phase-19.md:1925-1933; result: PASS)
- T1 SQL: PASS - `casework.watches` `trigger_count`, `last_trigger_at`, `last_trigger_key` (phase-19.md:231-238) and the CHECK at phase-19.md:241; grants at phase-19.md:783-784; `admin.outbox.created_at` (phase-00.md:125-135, column list at phase-00.md:329).
- T2 Route: N/A - a worker job; the item has no route.
- T3 Repository and types: PASS - `WatchRepository.get_for_update` and `record_trigger(tx, id, key, at)` are named (phase-19.md:1001); the block passes `event_at` as `at`.
- T4 Logic: PASS - steps (iii) and (iv) are stated with the skip test (phase-19.md:1928-1931); the lock order of the trigger steps is at phase-19.md:1165.
- T5 Audit: PASS - skipped events write no `trigger_count`, work item or audit (phase-19.md:1929-1931); a counted trigger keeps `watch.triggered` (phase-19.md:1165, 3.12 row at phase-19.md:1348).
- T6 Cross-references: PASS - `admin.outbox.created_at` (phase-00.md:329); DR-412 row in research.md ("a watch trigger counts once per event by `event_at`").
- T7 Consistency: PASS - the block states that its words replace "within the same second" (phase-19.md:1932), which is the wording of the section 4.2 consumer bullet (phase-19.md:1680); the payload change in 7.12 matches the job row at phase-19.md:1650.

## Summary

- Items checked: 12.
- Items fully PASS: 2 (V-19-06, V-19-12).
- GAP lines by severity: HIGH 0, MEDIUM 17, LOW 5 (22 in total).

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-19-01-a | LOW | T2 | Route GET /watches names `watch.read`, which the Cedar section never defines (list action is `watch.list`) | phase-19.md:1023; section 3.13 (1376) |
| V-19-01-b | MEDIUM | T3 | Service method names for the review, the expiry handler and the trigger dispatch | section 3.5 (1165, 1167, 1169); section 4.3 (1686) |
| V-19-01-c | MEDIUM | T6 | The canon.touched payload has no party ids and no batch-to-party lookup is named | section 3.5 Triggers (1165); phase-11.md:833-834 |
| V-19-02-a | MEDIUM | T3 | Repository reads for the finding and the analyst's closed cases | section 2 (1009); section 3.9 (1282) |
| V-19-02-b | MEDIUM | T4 | The window for "closed in the window of the finding" | section 3.9 (1282) |
| V-19-02-c | MEDIUM | T4 | Refusal status and code when the finding is not critical | section 3.9 (1282); section 3.11 (1319-1338) |
| V-19-03-a | LOW | T2 | Route GET /rfis names `rfi.read` (Rfi resource); the list action is `rfi.list` | phase-19.md:1031; section 3.13 (1376, 1461, 1494) |
| V-19-03-b | MEDIUM | T4 | The pre-fill that M-44 lists is never described (only the switch is named) | header (10); section 3.16 (1635); section 4.5 item 3 (1699) |
| V-19-04-a | MEDIUM | T3 | QA grant revocation path not stated (phase 18 revoke refuses closed cases) | section 3.9 step 8 (1277, 1280) |
| V-19-05-a | MEDIUM | T2 | GET /alarms list action: `alarm.list` has no IA or SEC permit | route row (1049); section 3.13 (1378, 1429, 1465) |
| V-19-05-b | MEDIUM | T3 | No repository method loads one alarm under lock for acknowledge or close | section 2 AlarmRepository (1004); section 3.10 (1311) |
| V-19-05-c | MEDIUM | T4 | No refusal stated for acknowledging or closing an alarm in the wrong state | section 3.10 (1311); section 3.11 |
| V-19-07-a | MEDIUM | T3 | Enums SamplePopulation and SampleSource not extended with the assistant values | section 2 Enums (977); DrawQaSample (989) |
| V-19-07-b | MEDIUM | T4 | The weekly assisted draw has no job or schedule | section 4.1 (1640-1660); section 4.5 item 14 (1722) |
| V-19-07-c | MEDIUM | T6 | Phase 28 requires the item view to show cited records; phase 19 section 4.5 does not | section 4.5 (1700-1724); 7.2 (1855); phase-28.md:544 |
| V-19-07-d | MEDIUM | T7 | Item view "no text" rule contradicts 7.2 draft-text display | section 2 Views (993); 7.2 (1855) |
| V-19-08-a | MEDIUM | T7 | Suppression sample rule: 5 per cent in 3.8 versus freeze-list rows in phase 14 | section 3.8 (1247); section 7.7 (1881-1885); phase-14.md:1318 |
| V-19-09-a | MEDIUM | T3 | No repository method for the prior_failures count by party | section 2 RfiRepository (996); 7.9 (1896) |
| V-19-09-b | LOW | T7 | Section 4.2 lacks the outreach.failed row the block says it adds | section 4.2 outbox table (1671-1678) |
| V-19-10-a | LOW | T4 | Sub-code rfi_open missing from the section 3.11 closed list | section 3.11 (1338) |
| V-19-10-b | LOW | T7 | Section 4.5 item 4 omits the phase 17 merge change from 7.10 | section 4.5 item 4 (1700-1705) |
| V-19-11-a | MEDIUM | T3 | No single-alarm read under the visibility predicate | section 2 AlarmRepository (1004); 7.11 (1916-1919) |
