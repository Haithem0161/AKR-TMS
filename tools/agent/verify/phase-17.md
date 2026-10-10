# Verification pass, phase 17 (Haiku draft for lead review)

Items sampled (12): header M-07 (start of Implemented here), M-18 (middle of Implemented here), M-15 (middle of Partly implemented), UX-13 (end of Partly implemented), VB-33 (end of the Requirements list); 7.1, 7.2, 7.5, 7.8 (gap pass blocks); 7.9, 7.12 (adversarial run 2) and 7.13 (adversarial run 5).

### V-17-01 Quarantine of payloads that cannot form an alert   (source: header M-07; result: GAPS 6)
- T1 SQL: GAP - table at phase-17.md:206 with the 8-value reason CHECK; the retry job writes `reason_detail.valid_now = true` (phase-17.md:963) but the only UPDATE grant on `alert.quarantine` omits the column: "GRANT UPDATE (status, resolution_code, resolved_by, resolved_at, released_alert_id, revalidated_at, updated_at, version)" (phase-17.md:551-552). The guard at 475-489 does not forbid it.
- T2 Route: PASS - phase-17.md:845-847: GET /api/v1/alert-quarantine (`queue.work`, `alert.quarantine.read`), POST .../release (`alert.quarantine.resolve`, step-up `decision`), POST .../reject (`alert.quarantine.resolve`).
- T3 Repository and types: GAP - `QuarantineRequest` (phase-17.md:765), `QuarantineRow` (783) and `record_standalone(request)` (938) are named. The DTO at line 795 names `reason_code?` while reject needs `resolution_code` (958-959). No repository trait for `alert.quarantine` exists: the traits at 800-830 are AlertRepository and CaseRepository only. No signatures for QuarantineService release, reject or retry (grep "QuarantineService" hits only 759, 914, 938, 1551, 1556, 1573).
- T4 Logic: GAP - record_standalone (938-944) and release (946-956) are numbered, with 409, 422 and 202 named. Reject (958-960) and the retry job (962-965) are prose with no lock and no refusal codes. Release step 2 (948) writes `revalidated_at` and an audit event and then answers 422, but no commit rule says the write survives the error (grep "commit" finds nothing in 3.2).
- T5 Audit: PASS - 3.16 table: `alert.quarantined` (phase-17.md:1348), `alert.quarantine_revalidated` (1349), `alert.quarantine_release_requested` (1350), `alert.quarantine_released` (1351), `alert.quarantine_rejected` (1352).
- T6 Cross-references: PASS - `scn.detection_keys` keyed by fingerprint (phase-16.md:332-333); `AlertSink` and `EmitOutcome` (phase-16.md:835-838); `DetectionKeyRepository::insert_or_get` (phase-16.md:807); `DetectionPayload` (phase-16.md:1206); `canon.party` (phase-10.md:451); reason-code families `detection` and `closure`, prefix `CLS_` (phase-09.md:737-741); `StepUpService::consume` (phase-01.md:731) and class `decision` (phase-01.md:709); `put_generated` (phase-02.md:876); `E_NOT_ALLOWED_IN_STATE` 409 and `E_VALIDATION` 422 (docs/API.md:966-990).
- T7 Consistency: PASS - no 7.x block changes alert.quarantine (grep "quarantin" from line 1774 finds only Decisions, 1929-2060); DR-120 (phase-17.md:1943) agrees with the base text.
- Gaps:
GAP V-17-01-a | MEDIUM | T1 | UPDATE grant omits `reason_detail`, which the retry job writes (`valid_now`) | section 1.5, the GRANT block of 0061 (phase-17.md:551-552) | grep reason_detail: only lines 215, 648, 940, 963; no GRANT line
GAP V-17-01-b | MEDIUM | T3 | No repository trait for alert.quarantine (insert, lock, update) and no QuarantineService signatures for release, reject, retry | section 2, after CaseRepository (phase-17.md:814-830), and section 3.2 | grep "QuarantineRepository" (none); grep "release(" and "reject(" (none)
GAP V-17-01-c | LOW | T3 | DTO field is `reason_code` but reject requires `resolution_code` | section 2 DTO list (phase-17.md:795) | grep "reason_code" in the quarantine text finds no other use
GAP V-17-01-d | MEDIUM | T4 | Reject has no row lock, no 409 for a row that is not open, no 422 for a missing or non-CLS_ code | section 3.2 Reject (phase-17.md:958-960) | grep "FOR UPDATE" in 958-965 (none); grep "E_NOT_ALLOWED_IN_STATE" in 958-960 (none)
GAP V-17-01-e | MEDIUM | T4 | Retry job writes the rows the release job locks, but names no lock and no status re-check | section 3.2 Retry job (phase-17.md:962-965) | grep "FOR UPDATE" in 962-965 (none); grep "quarantine_retry" (lines 623, 962, 1527, 1772; none names a lock)
GAP V-17-01-f | MEDIUM | T4 | The 422 path of release writes revalidated_at and an audit event; the spec does not say that this transaction commits | section 3.2 Release step 2 (phase-17.md:948) | grep "commit" and "rollback" in 3.2 (none)

### V-17-02 Alert and case statuses as events   (source: header M-18; result: GAPS 3)
- T1 SQL: PASS - `alert.alert_events` at phase-17.md:190-204 (kind CHECK `^alert\.[a-z_]+$`, `UNIQUE (alert_id, seq)`), append-only trigger at 421; `casework.case_events` at 335-348 (kind CHECK `^case\.[a-z_]+$`); grants at 545 (`SELECT, INSERT ... alert.alert_events`), 546-550 (state and claim columns), 558 (`casework.case_events`).
- T2 Route: N/A - the event writes are made by the claim, triage, assign and merge routes, which have their own rows. The case trail is read by GET /api/v1/cases/{id}/timeline (phase-17.md:850).
- T3 Repository and types: GAP - `AlertRepository` (phase-17.md:804-812) has no method that appends an alert event, and no `NewAlertEvent` type exists. `CaseRepository::append_event` (line 820) takes `NewCaseEvent`, and `CaseEvent` (line 741) has a field of type `CaseEventKind`; neither type is defined anywhere in docs/plan/phase-*.md.
- T4 Logic: GAP - the case append is numbered and locked: "seq = last_seq + 1" under the case row lock (phase-17.md:1111-1113). The alert trail has no sequence rule: `alert.alerts` has no `last_seq` column (DDL at 106-171), and claim step 8 (line 1139) writes `alert_events` with no seq or lock rule.
- T5 Audit: PASS - 3.16 table: `alert.recorded` (1344), `alert.no_longer_matches` (1346), `alert.matches_again` (1347), `alert.claimed` and `alert.unclaimed` (1353), `alert.triaged` and `alert.marked_duplicate` (1354), `alert.reclassified` (1355), `alert.assigned` (1356), `alert.touch_breached` (1357); `case.alert_triaged` is named in step 3.8 Triage (1151), and the 3.16 note (line 1372) says every `alert.*` and `case.*` event is also written to its trail.
- T6 Cross-references: PASS - phase-02.md:265 and 530 reserve `case_event` as a chained stream for phase 17; the canonical encoding `cv = 1` is at phase-02.md:688.
- T7 Consistency: PASS - 7.9 (line 1865) and 7.10 (line 1871) name `case.alert_amended_after_decision` and `case.alert_no_longer_matches`, which the base text already uses (lines 1298 and 1303). The `case_events.kind` CHECK is a pattern (line 341), so the new kinds fit.
- Gaps:
GAP V-17-02-a | MEDIUM | T3 | No method or type appends alert events: no append method on AlertRepository and no `NewAlertEvent` | section 2, AlertRepository (phase-17.md:804-812) | grep "NewAlertEvent" and "AlertEventRepository" (none); grep "append" in 804-812 (none)
GAP V-17-02-b | MEDIUM | T4 | `alert_events.seq` is not allocated: `alert.alerts` has no `last_seq`, and no lock or max-plus-one rule is stated for the alert trail | section 3.8 Claim step 8 (phase-17.md:1139), or a rule next to section 3.7 (1111) | grep "last_seq" in 106-171 (none); grep "seq" for alert_events: none; the only sequence rules are for cases (phase-17.md:432-438, 1110-1111)
GAP V-17-02-c | MEDIUM | T3 | `NewCaseEvent` (argument of `append_event`) and `CaseEventKind` (field of `CaseEvent`) are never defined | section 2, domain types (phase-17.md:741) and CaseRepository (820) | grep "NewCaseEvent|CaseEventKind" across docs/plan/phase-*.md (only phase-17.md:741 and 820)

### V-17-03 Assignment (push) of an alert with pool membership, WIP limit and conflict check   (source: header M-15; result: GAPS 4)
- T1 SQL: PASS - `alert.alerts` columns `state`, `claimed_by`, `claimed_at`, `version` (DDL at phase-17.md:106-171) with UPDATE grant at 546-550; `casework.wip_limit` = 10, governed (phase-17.md:605); pool kind `alerts` CHECK (phase-01.md:106) and role-grant scope with `pools` (phase-01.md:120).
- T2 Route: PASS - phase-17.md:844: POST /api/v1/alerts/{id}/assign, `queue.work`, `alert.assign`, `If-Match`.
- T3 Repository and types: GAP - `AssignAlert { to: { user_id } | { pool_id }, reason_code? }` (line 793) and `AlertRepository` lock, update and count_wip (807-811) are named. `AssignmentService` (section 3.8, line 1126) has no method signature for alert or case assignment. `ConflictCheck` is the phase 03 port (phase-03.md:461, `check(principal, subject_party)`), so it is named correctly.
- T4 Logic: GAP - steps 1 to 3 (1161-1164) are numbered, with 422 at `/to` and the WIP and conflict checks by reference to claim steps 6 and 7. Missing: (i) the alert states the push may act on (step 3 sets `claimed` with no state check); (ii) the pool that step 1 checks, since "the queue's pool" has no queue in the route and `alert.alerts` has no pool column.
- T5 Audit: PASS - `alert.assigned` with `{from, to}` in the 3.16 table (phase-17.md:1356); the conflict refusal audit `alert.conflict_refused` (3.16 row 1358) and the claim step 7 (1136) apply to the push by reference of step 2.
- T6 Cross-references: PASS - phase 19 `AlarmService::raise` and `conflict_declared` (phase-19.md:1299, 1306) name phase 17 section 3.8 correctly; phase 01 pool and scope (see T1); `ConflictCheck` (phase-03.md:461).
- T7 Consistency: GAP - the route row says "Assign to a user or pool" (phase-17.md:844), while step 3 (1164) says "the target is a user only (a pool target is 422)" and Decisions say "An alert is assigned to a user only" (phase-17.md:2029). 7.3 (1802-1806) and 7.12 (1882-1891) do not change this, so the contradiction is in the base text.
- Gaps:
GAP V-17-03-a | MEDIUM | T3 | No method signature for `AssignmentService` (alert and case assign), so the caller, target and version parameters are not named | section 3.8 (phase-17.md:1126-1170) or section 2 | grep "fn assign" and "assign(" (none); grep "AssignmentService" (only 1124, 1551, 1795, 1803)
GAP V-17-03-b | MEDIUM | T4 | Assign does not say which alert states it accepts; a `triaged` or `duplicate` alert would become `claimed` and break the 0059 CHECKs (`(state = 'duplicate') = (duplicate_of IS NOT NULL)` and the triaged check), and a `no_longer_matches` alert would be revived | section 3.8 Assign step 3 (phase-17.md:1164) | grep "E_NOT_ALLOWED_IN_STATE" in 1160-1170 (none); 7.12 (1882-1891) names no state
GAP V-17-03-c | MEDIUM | T4 | Step 1 checks membership of "the queue's pool", but POST /alerts/{id}/assign has no queue and `alert.alerts` has no pool column | section 3.8 Assign step 1 (phase-17.md:1161) and the route row (844) | grep "pool" in the `alert.alerts` DDL (none); grep "queue" in the route row (none)
GAP V-17-03-d | MEDIUM | T7 | Route row says "Assign to a user or pool"; step 3 and Decisions say a pool target is refused (422) and an alert goes to a user only | route table row (phase-17.md:844) against 3.8 step 3 (1164) and Decisions (2029) | grep "pool" in the route row (matches "user or pool") and in step 3 (matches the 422 refusal)

### V-17-04 Bulk assign and bulk mark duplicate   (source: header UX-13; result: GAPS 3)
- T1 SQL: PASS - `casework.bulk_cap` = 25 (phase-17.md:613); `alert.alerts` UPDATE grant for `state`, `duplicate_of`, `claimed_by`, `claimed_at` (546-550); `casework.case_alerts` UPDATE grant for `(case_id, duplicate_of)` with the comment "merges and duplicate marks" (560).
- T2 Route: PASS - phase-17.md:838: POST /api/v1/queues/{id}/bulk, `queue.work`, `queue.bulk`, `Idempotency-Key`, `dry_run` preview, cap `casework.bulk_cap`.
- T3 Repository and types: GAP - `BulkRequest { ids, action, to?, duplicate_of?, dry_run }` (line 794) and `BulkResult` (named in the OpenAPI list, line 865) exist. No service method is named for the bulk dry run or the real run: `QueueService` has only `queues`, `items` and `counts` (1186-1248), and `AssignmentService` has no signatures (see V-17-03-a).
- T4 Logic: GAP - the bulk rules are one prose paragraph (phase-17.md:1207-1211), not numbered steps. The order of the item checks is not given. The per-item reasons do not include the conflict check (M-67) or the pool membership of 3.8 step 1, and no sentence says that a bulk assign item runs the single-assign steps of 3.8.
- T5 Audit: PASS - `queue.bulk_applied` per applied item (phase-17.md:1370, and 1210); the item's own event (`alert.assigned` or `alert.marked_duplicate`) is in the 3.16 table (1354, 1356).
- T6 Cross-references: PASS - `casework.bulk_cap` (613); `p0_or_p1_duplicate` uses the priority classes of section 2 (`Priority`, phase 14), which are not redefined; the conflict check would be phase 03 `ConflictCheck` (phase-03.md:461) if added.
- T7 Consistency: GAP - 7.11 (phase-17.md:1876-1878) makes each bulk mark_duplicate item refuse a duplicate target with reason `target_is_duplicate`. 7.14 (1902-1906) says the bulk reasons "become" a closed list of five (`not_found`, `claimed_by_other`, `p0_or_p1_duplicate`, `wip_limit`, `lower_refused`) with no `target_is_duplicate`. Two 7.x blocks give different bulk rules.
- Gaps:
GAP V-17-04-a | MEDIUM | T3 | No service method for the bulk dry run or the real run, with the caller, the ids and the dry_run flag named | section 3.9 (phase-17.md:1207) or section 2 (QueueService, 1186-1248) | grep "bulk(" and "fn bulk" (none); grep "QueueService::" (only queues, items, counts)
GAP V-17-04-b | MEDIUM | T4 | Bulk checks are prose with no order and no pool or conflict reason; a bulk assign skips the conflict check (M-67) and the pool membership check unless the single-assign steps are said to apply | section 3.9 Bulk (phase-17.md:1207-1211) against 3.8 Assign (1161-1164) and claim step 7 (1136) | grep "conflict" in 1207-1211 (none); grep "bulk" in 3.8 (none)
GAP V-17-04-c | MEDIUM | T7 | 7.11 adds reason `target_is_duplicate` for each bulk mark_duplicate item; 7.14 replaces the bulk reason list with a closed set that does not include it | 7.14 (phase-17.md:1902-1906) against 7.11 (1876-1878) | grep "target_is_duplicate" (only 1878); grep "bulk reasons" in 7.14 (line 1904)

### V-17-05 Evidence snapshot with producing versions, and the alert decision record   (source: header VB-33; result: GAPS 2)
- T1 SQL: PASS - `snapshot_object_id` FK to `admin.evidence_objects` and `snapshot_sha256` CHECK (phase-17.md:154-155); producing versions `scenario_version_id`, `logic_version`, `reference_versions`, `register_version` (112-119); the decision record is the triage columns `triaged_by`, `triaged_at`, `duplicate_of`, `state` (grant 546-550), and `contribution_outcome` written by phase 18 (phase-18.md:425).
- T2 Route: PASS - phase-17.md:840: GET /api/v1/alerts/{id}/snapshot, `queue.work`, `alert.snapshot.read`; the access event is written (route text).
- T3 Repository and types: GAP - `SnapshotService` (heading at phase-17.md:966) has no method signature. `TxRow` is returned by `ReferenceReader::tx_rows` (line 774) and named in step 1 (line 970), but no plan file defines the struct (grep "struct TxRow" and "TxRow {" over docs/plan/phase-*.md: none).
- T4 Logic: PASS - steps 1 to 5 of 3.3 (966-983) are numbered; the refusal (`Infected`) maps to `AlertSinkError::Storage` with a retry (977); the order is fixed by 3.1 step 4 (890), which stores the snapshot before the alert row.
- T5 Audit: PASS - `put_generated` emits `evidence.stored` (phase-02.md:870 and the audit table at 977); the alert-level writes have their own events (`detection.recorded` and `alert.recorded`, 3.1 step 11; `alert.triaged` and `alert.marked_duplicate`, 3.16 row 1354).
- T6 Cross-references: PASS - `put_generated` (phase-02.md:876) and `GeneratedEvidence` (phase-02.md:611-613, the `NewEvidence` fields minus `file_name`); `owner_kind` regex admits `alert` (phase-02.md:356); `lang` CHECK admits `und` (phase-02.md:367); `akr_evidence_orphans_total` (phase-02.md:873); `canon.txn` (phase-10.md:951); `evidence.fields` in the scenario (phase-16.md:967); phase 18 grant (phase-18.md:425).
- T7 Consistency: PASS - no 7.x block names the snapshot (grep "snapshot" in lines 1774-1914 finds nothing); 3.3 step 5 (new snapshot on amendment, phase-17.md:981) is not contradicted.
- Gaps:
GAP V-17-05-a | MEDIUM | T3 | `SnapshotService` has no method signature (the store call, its tx and its inputs) | section 3.3 (phase-17.md:966) or section 2 | grep "SnapshotService" (only 966); grep "fn store" and "snapshot(" (none)
GAP V-17-05-b | MEDIUM | T3 | `TxRow` (the rows returned by `tx_rows` and used for the snapshot body) is not defined | section 2 next to ReferenceReader (phase-17.md:774) or section 3.3 step 1 (970) | grep "TxRow" in docs/plan/phase-*.md (only phase-17.md:774 and 970); grep "canon.txn" columns in phase-10.md:951 (no Rust struct)

### V-17-06 Hibernation gate in the intake, with the breach clock and the open-hit clause   (source: 7.1, gap pass run 4a; result: GAPS 1)
- T1 SQL: PASS - `casework.hibernation_hits` (phase-19.md:278-291) has `alert_id`, `alert_created_at`, `released_at` and `UNIQUE (alert_id, alert_created_at)`; the intake INSERT is granted: "GRANT SELECT, INSERT ON ... casework.hibernation_hits, ... TO akr_app" (phase-19.md:772-774); the UPDATE columns at 786.
- T2 Route: N/A - 7.1 adds no route. Its clauses change the behaviour of GET /api/v1/queues (line 832), GET /api/v1/queues/{id}/items (line 833) and the counts stream (line 854).
- T3 Repository and types: PASS - `HibernationGate::check(conn, party_id, scenario_id, measures, now)` is named in 7.1 (phase-17.md:1780-1781); `attach_released(tx, alert_id, alert_created_at)` is new and named there; `BreachService::arm(tx, owner_kind, owner_id, start_at, window_hours)` and `breach(tx, owner, stage)` are in phase-19.md:1291 and 1294; the job `hibernation_release` is in phase-19.md:1651.
- T4 Logic: PASS - 7.1 gives the step order (4b between 4 and 5; on `Suppress`, steps 6, 8 and 11 run and the hit is inserted after step 6; 5, 7, 9, 10 and 12 are skipped); the release path runs 5, 7, 9, 10 and 12 for the existing alert. The breach handler's skip rule for an open hit is given (phase-17.md:1789-1790).
- T5 Audit: PASS - the hit insert names its audit event in the owning phase: `hibernation.hit_recorded {hibernation_id, alert_id}` (phase-19.md:1191). The block itself does not name it, so the reader must go to phase 19.
- T6 Cross-references: GAP - one LOW name mismatch (see V-17-06-a). The rest PASS: `HibernationGate` (phase-19.md:1188), `casework.hibernations` and `casework.hibernation_hits` (phase-19.md:278), `BreachService` (phase-19.md:1288-1296), `hibernation_release` (phase-19.md:1651), `BusinessClock::add` (phase-09.md:459), the window setting `casework.touch_by_hours` (phase-17.md:1278).
- T7 Consistency: PASS - base step 9 still schedules `alert_touch` and `alert_age` through TimerService (phase-17.md:902), and phase 19 says `arm` schedules only the 50 and 150 stages (phase-19.md:1291), with the 100 stage on `timer_alert_touch` (phase-19.md:1292). The two rules fit together. No contradiction.
- Gaps:
GAP V-17-06-a | LOW | T6 | The block passes `touch_hours` to `BreachService::arm`, but the phase defines a per-class array `casework.touch_by_hours[class]`, and `touch_hours` is defined nowhere | 7.1, step 9 (phase-17.md:1783) | grep "touch_hours" in docs/plan/phase-17.md (only 1783); grep "touch_hours" in phase-19.md (none; phase-31 uses `touch_hours_c`, a different concept)

### V-17-07 Case surfaces extended by phase 18   (source: 7.2; result: GAPS 1)
- T1 SQL: PASS - phase 18 adds `casework.case_acl` (phase-18.md:85-96) with `reason IN ('assignment', 'escalation', 'second_review', 'mlro_desk', 'manual')` (line 90), `revoked_by` and `revoked_at` (95-96); grants at phase-18.md:427-428; `escalation_level` added to `casework.cases` (phase-18.md:40-41) and the outcome CHECK (line 30).
- T2 Route: PASS - the route is unchanged: "GET /api/v1/cases/{id} | `queue.work` | Case header" (phase-17.md:849); 7.2 adds response fields only.
- T3 Repository and types: PASS - the CaseRepository methods are phase 18 section 2 (phase-18.md:581 "extended with set_closure"; 1135); `CaseAccessService::grant(ctx, case_id, GrantAccess, if_match)` and `revoke(...)` are named with parameters (phase-18.md:653, 661).
- T4 Logic: GAP - 7.2 says AssignmentService "writes the `casework.case_acl` grants of phase 18 section 3.2 in the assignment transaction" (phase-17.md:1795-1797) but not how. `grant` needs Cedar `case.access.change` for A2 and M only (phase-18.md:653), while phase 17 lets A1 run `case.assign` (phase-17.md:1402) and the assign route needs only `case.assign` (line 852). The block does not say whether the rows are written directly or through `grant`, under which actor, or with which If-Match.
- T5 Audit: PASS - phase 18 names `case.acl_granted` in grant step 6 (phase-18.md:659) and `case.acl_granted, case.acl_revoked` in its audit table (phase-18.md:930).
- T6 Cross-references: PASS - the phase 18 names used exist: `casework.case_acl` (85), `escalation_level` (40-41), the reporter row in 3.10 (phase-18.md:811), the open-proposal and affordance fields in section 4.5 item 5 (phase-18.md:1133).
- T7 Consistency: PASS - the reason values and the assignee rule match phase 18 (phase-18.md:649 "the assignee gets `work` when the holder is A1 and `decide` when the holder is A2"); base 3.8 step 5 (`case.assigned`, If-Match) is not contradicted.
- Gaps:
GAP V-17-07-a | MEDIUM | T4 | The assignment grant has no stated write path or authorization: `grant()` needs `case.access.change` (A2, M) and If-Match, but A1 may assign; the block does not say whether the rows are written directly or through `grant()`, or by which actor | 7.2 second bullet (phase-17.md:1795-1797) against phase-18.md:653 and phase-17.md:1402 | grep "case.access.change" in docs/plan/phase-17.md (none); grep "grant(" in phase-17.md (none)

### V-17-08 Model rank in the queue order   (source: 7.5; result: GAPS 1)
- T1 SQL: PASS - `alert.model_scores` has `alert_id`, `alert_created_at`, `role IN ('live', 'challenger', 'shadow')` and `score` (phase-27.md:178-186); the grant `GRANT SELECT, INSERT ON alert.model_scores TO akr_app` is at phase-27.md:291. The 7.5 block adds no column.
- T2 Route: PASS - the order is on `GET /api/v1/queues/{id}/items` (phase-17.md:833); `GET /alerts/{id}/model-score` is phase 27 route 16 (phase-27.md:421), scope `queue.work`, Cedar `alert.model_score.read`.
- T3 Repository and types: PASS - `QueueService::items(queue, view, principal, network, cursor)` is named (phase-17.md:1189); the join is written in full at phase-27.md:502.
- T4 Logic: GAP - the order and the kill condition are stated (phase-17.md:1815-1817; phase-27.md:502 puts the kill clause in the join's ON). The items query has a keyset cursor (phase-17.md:1192, "Keyset cursor"), and neither block says that the cursor now carries `model_rank` and the NULLS LAST position. Without it the next page cannot follow the new order.
- T5 Audit: N/A - the block writes no rows; the score rows are written by the phase 27 scorer.
- T6 Cross-references: PASS - phase 27 section 3.4 kill state (phase-27.md:495) and queue-order text (502); route 16 (421); the probe set of kind `alert` is check 9 of phase 17 section 6 (phase-17.md:1720), which phase 27 section 7.3 extends (phase-27.md:871-873).
- T7 Consistency: PASS - phase 27 repeats the same ORDER BY as 7.5 (phase-27.md:502); the base order in 3.9 item 3 (phase-17.md:1192) is replaced from phase 27 on, as 7.5 says.
- Gaps:
GAP V-17-08-a | MEDIUM | T4 | The keyset cursor of the items query does not carry `model_rank` or the NULLS LAST position, so the next page of the new order is undefined | section 3.9 item 3 (phase-17.md:1192) and 7.5 (1815-1817) | grep "cursor" in phase-17.md lines 1814-1821 (none); grep "cursor" in phase-27.md (only line 406, the API 3.15 row)

### V-17-09 The profile.breach topic   (source: 7.8; result: PASS)
- T1 SQL: PASS - the neutral code fits the `trigger_code` CHECK `^(TR-[0-9]{2}|TRG_[A-Z_]{3,40})$` (phase-15.md:279), so `TRG_REVIEW_REQUESTED` is storable; the outbox row needs no DDL here.
- T2 Route: N/A - the topic is an outbox message with no route.
- T3 Repository and types: PASS - the handler is phase 15's trigger logic, named in phase 15 section 7.6 (phase-15.md:2080-2081); the payload `{party_id}` is given in 7.8 step 2.
- T4 Logic: PASS - steps 1 to 5 are numbered (phase-17.md:1846-1857): the outcome rule (`New` only), the same transaction, the payload, the enqueue-only handler, the 4.2 row and the tests.
- T5 Audit: PASS - outbox writes carry no audit event in the base either: step 12 writes `alert.created` and `case.event` with none (phase-17.md:908), so the convention holds.
- T6 Cross-references: PASS - phase 15 section 3.3 lists `profile.breach` in its topic table (phase-15.md:1278) and maps the trigger to `TRG_REVIEW_REQUESTED` (3.6); that code is defined in section 3.6 (phase-15.md:1461); phase 16 section 3.1 `logic.kind` and `expected[]` with `source` `declared` (phase-16.md:948-957); DR-375 (research.md:386) and DR-401 (research.md:412); DSL.md C.5 says the shipped library is TR-01 to TR-26 (DSL.md:813).
- T7 Consistency: PASS - base 3.1 step 12 and base 4.2 name the two topics this phase registers (phase-17.md:908 and 1534-1547); 7.8 adds the third topic and says so.

### V-17-10 Amendments take the case lock   (source: 7.9, adversarial run 2; result: PASS)
- T1 SQL: PASS - `casework.case_alerts` has `alert_id` and `alert_created_at` (phase-17.md:352-360) and the FK to `alert.alerts(id, created_at)`; `casework.cases.outcome` exists (phase-17.md:301).
- T2 Route: N/A - the amendment comes from the sink (3.13, phase-17.md:1292), not a route.
- T3 Repository and types: PASS - `AlertRepository::lock` (phase-17.md:807) and `CaseRepository::lock` (818) are named.
- T4 Logic: PASS - the order is stated: "resolve the alert's case through `casework.case_alerts`, lock the case row `FOR UPDATE`, then lock the alert" (phase-17.md:1862-1864); outcome and decision events are read after both locks (1864).
- T5 Audit: PASS - `case.alert_amended_after_decision` is named in the base (phase-17.md:1298) and in 7.9 (1865-1866); `alert.amended` is in the 3.16 table (1345).
- T6 Cross-references: PASS - the bucket fold's advisory lock (phase-16.md:1193, `pg_advisory_xact_lock`); the decision outbox `case.decision` (phase-18.md:769); DR-408 (research.md:419).
- T7 Consistency: PASS - base 3.13 (phase-17.md:1297-1299) gives the same rule for decided cases; 7.9 adds only the lock order, which the base does not state.

### V-17-11 Assignment locks and a live claim   (source: 7.12, adversarial run 2; result: PASS)
- T1 SQL: PASS - no new DDL; the columns it writes (`state`, `claimed_by`, `claimed_at`, `version`) are in the UPDATE grant at phase-17.md:546-550.
- T2 Route: PASS - `POST /api/v1/alerts/{id}/assign` (phase-17.md:844) and `POST /api/v1/queues/{id}/bulk` (838).
- T3 Repository and types: PASS - `AlertRepository::lock`, `update` and `count_wip` (phase-17.md:807-811) are named; the lock key is written out ("pg_advisory_xact_lock(hashtext('wip:' || target_user_id))", line 1883).
- T4 Logic: PASS - the order is stated in one sentence: WIP lock, then the alert row lock, then If-Match, then the WIP and conflict checks (1883-1884). The state rule that base 3.8 Assign lacks is not added here (see V-17-03-b).
- T5 Audit: PASS - `alert.assigned {from, to}` (phase-17.md:1356) and `queue.bulk_applied` (1370).
- T6 Cross-references: PASS - the `wip:` key of claim step 3 (phase-17.md:1130); `E_WIP_LIMIT` 409 (docs/API.md:976); `claimed_by_other` of 3.9 (1208).
- T7 Consistency: PASS - no contradiction with base 3.8 (1126-1166) or base 3.9 bulk (1207-1211); the bulk lock matches the single-item lock.

### V-17-12 Merge checks visibility before any other answer   (source: 7.13, adversarial run 5; result: PASS)
- T1 SQL: PASS - no new DDL; the merge columns `merged_into`, `status` and `version` are in the cases UPDATE grant (phase-17.md:554-556).
- T2 Route: PASS - `POST /api/v1/cases/{id}/merge` (phase-17.md:853), scope `queue.work`, Cedar `case.merge`, `If-Match`.
- T3 Repository and types: PASS - `MergeCase { target_case_id, reason_code }` (phase-17.md:796); `CaseMerger::plan(source, target)` and `execute(tx, plan, actor)` (1078-1079); the authorize call is phase 03 section 3.2 (phase-03.md:678).
- T4 Logic: PASS - base steps 1 to 4 (phase-17.md:1082-1086) and the new order of 7.13 (1894-1899): "after the two `FOR UPDATE` locks, step 4 runs first"; a masked 404 for an invisible target; steps 1 to 3 only when both cases are visible.
- T5 Audit: PASS - `case.merged` (3.6 step 7, line 1098) and the ledger events `case.merged_out` and `case.merged_in` (1094-1095).
- T6 Cross-references: PASS - phase 18 refusals `proposal_running` and `decided` and `work_items_open` (phase-18.md:733, 921, 214); phase 19 `rfi_open` (phase-19.md:1909); DR-411 (research.md:422).
- T7 Consistency: PASS - 7.13 reorders base step 4 (phase-17.md:1086) ahead of steps 1 to 3 and says that step 4 runs first; no other block changes merge.


## Summary

- Items checked: 12 (5 header items, 4 gap-pass 7.x blocks, 3 adversarial-block items under the run 2 and run 5 headers).
- Items fully PASS: 4 (V-17-09 profile.breach topic; V-17-10 amendments take the case lock; V-17-11 assignment locks and a live claim; V-17-12 merge checks visibility).
- GAP lines by severity: HIGH 0; MEDIUM 19; LOW 2 (21 in total).

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-17-01-a | MEDIUM | T1 | UPDATE grant on alert.quarantine omits `reason_detail`, which the retry job writes | section 1.5 GRANT block (phase-17.md:551-552) |
| V-17-01-b | MEDIUM | T3 | No repository trait for alert.quarantine; no QuarantineService signatures for release, reject, retry | section 2 after CaseRepository (phase-17.md:814-830); section 3.2 |
| V-17-01-c | LOW | T3 | DTO field `reason_code` where reject needs `resolution_code` | section 2 DTO list (phase-17.md:795) |
| V-17-01-d | MEDIUM | T4 | Reject has no row lock, no 409 for a row not open, no 422 for a missing or non-CLS_ code | section 3.2 Reject (phase-17.md:958-960) |
| V-17-01-e | MEDIUM | T4 | Retry job writes rows the release job locks; no lock and no status re-check | section 3.2 Retry job (phase-17.md:962-965) |
| V-17-01-f | MEDIUM | T4 | The 422 path of release writes and audits, with no commit rule | section 3.2 Release step 2 (phase-17.md:948) |
| V-17-02-a | MEDIUM | T3 | No append method or NewAlertEvent type for alert events | section 2 AlertRepository (phase-17.md:804-812) |
| V-17-02-b | MEDIUM | T4 | alert_events.seq is not allocated (no last_seq on alert.alerts; no lock or max-plus-one rule) | section 3.8 claim step 8 (phase-17.md:1139); near section 3.7 (1111) |
| V-17-02-c | MEDIUM | T3 | NewCaseEvent and CaseEventKind are never defined | section 2 (phase-17.md:741, 820) |
| V-17-03-a | MEDIUM | T3 | No method signatures for AssignmentService (alert and case assign) | section 3.8 (phase-17.md:1126-1170) or section 2 |
| V-17-03-b | MEDIUM | T4 | Assign does not say which alert states it accepts; triaged and duplicate would break the 0059 CHECKs | section 3.8 Assign step 3 (phase-17.md:1164) |
| V-17-03-c | MEDIUM | T4 | "The queue's pool" check has no queue in the route and no pool column on alert.alerts | section 3.8 Assign step 1 (phase-17.md:1161); route row (844) |
| V-17-03-d | MEDIUM | T7 | Route row says "user or pool"; step 3 and Decisions say user only | route row (phase-17.md:844) against 3.8 step 3 (1164) and Decisions (2029) |
| V-17-04-a | MEDIUM | T3 | No service method for the bulk dry run or the real run | section 3.9 (phase-17.md:1207); section 2 QueueService (1186-1248) |
| V-17-04-b | MEDIUM | T4 | Bulk checks are prose with no order, no pool and no conflict reason | section 3.9 Bulk (phase-17.md:1207-1211) |
| V-17-04-c | MEDIUM | T7 | 7.11 adds reason `target_is_duplicate`; 7.14 closed bulk list omits it | 7.14 (phase-17.md:1902-1906) against 7.11 (1876-1878) |
| V-17-05-a | MEDIUM | T3 | SnapshotService has no method signature | section 3.3 (phase-17.md:966) |
| V-17-05-b | MEDIUM | T3 | TxRow (rows returned by tx_rows, used for the snapshot) is not defined | section 2 (phase-17.md:774) or 3.3 step 1 (970) |
| V-17-06-a | LOW | T6 | Block 7.1 passes `touch_hours`; the phase defines `casework.touch_by_hours[class]` | 7.1 step 9 (phase-17.md:1783) |
| V-17-07-a | MEDIUM | T4 | Assignment grant has no stated write path or authorization (grant() needs case.access.change, A2 and M; A1 may assign) | 7.2 second bullet (phase-17.md:1795-1797) against phase-18.md:653 and phase-17.md:1402 |
| V-17-08-a | MEDIUM | T4 | Keyset cursor of the items query does not carry model_rank or the NULLS LAST position | section 3.9 item 3 (phase-17.md:1192); 7.5 (1815-1817) |
