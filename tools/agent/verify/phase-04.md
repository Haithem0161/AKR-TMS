# Verification pass, phase 04 (Haiku draft for lead review)

Scope note: phase 04 has no "Implemented here" list, so the header items come from "Requirements carried". Only two adversarial blocks exist (run 1: 7.13 and 7.14), so the sample uses six header items (SP-22, GV-01, GV-26, S-39, UX-14, UX-60) in place of the three extra adversarial items. Gap-pass items: 7.1, 7.2, 7.8, 7.10 (four different 7.x blocks). Structural note (not counted as an item gap): phase-04.md has no "## Decisions" and no "## Open doubts" section, which the phase template requires.

Greps used for every GAP: phase-04.md (whole file), the owning phase file (cited), and docs/plan/research.md where relevant. Note: docs/plan/research.md was not needed for any GAP below.

### V-04-01 SP-22 emergency and approval path, approve route and activation   (source: header SP-22; result: GAPS 2)
- T1 SQL: PASS - admin.change_requests holds checker_id, stepup_id, activated_at, review_due_at, review_outcome (phase-04.md:121-168); lifecycle columns granted at phase-04.md:206-208; settings change.emergency_review_h, change.emergency_review_working_days, change.post_review_days, change.route_map seeded at phase-04.md:562-600.
- T2 Route: PASS - POST /change-requests/{id}/approve, scope self, SU change.approve, If-Match (phase-04.md:781); reject at line 782; Cedar route aliases named at phase-04.md:1452.
- T3 Repository and types: GAP - decide(ctx, id, d: Decision, if_match, a: StepUpAssertionId) (phase-04.md:860) and activate(tx, id, actor: ActorRef) (line 862) use types nobody defines. ActorRef and StepUpAssertionId occur only in phase-04.md (grep -rln over docs/ returns only this file). Decision is defined in phase-03.md:374 as the authorization decision record (Effect, DenyReason, policy_ids), a different type. Activated (line 652) is also undefined. The approve/reject payload (DecideRequest, DTO table in section 2) is not mapped to a Rust type.
- T4 Logic: PASS - act steps 1-7 with codes (phase-04.md:1051-1064: 409 E_NOT_ALLOWED_IN_STATE, 403 E_SELF_APPROVAL, 401 E_STEPUP_REQUIRED, 422 E_EVIDENCE_STALE); activate steps 1-5 with lock and stale-base rule (3.2, lines 910-925; outcome handling 900-909); emergency review rules (lines 927-940); instance lock I1 (line 976).
- T5 Audit: PASS - change_request.effective, .failed, .reverted, .review_confirmed, .review_escalated, task.approved, task.rejected named (phase-04.md:1373-1400); activation target events named at line 915 (setting.updated, role_grant.created, policy_bundle.activated).
- T6 Cross-references: PASS - admin.timers and TimerService.schedule (phase-00.md:138, 561); StepUpService::consume signature (phase-01.md:731); SettingsService::apply with ChangeSource (phase-00.md:574); invalidate_user (phase-01.md:605); StepUpAction classes (phase-01.md:716-720). Checked 5 names.
- T7 Consistency: GAP (LOW) - the emergency review escalation is written as event `escalated` (phase-04.md:939) while the engine event list names `task_escalated` (phase-04.md:985). No 7.x block resolves it.
- Gaps:
GAP V-04-01-a | MEDIUM | T3 | ActorRef, StepUpAssertionId, Decision (for approve/reject) and Activated are used in signatures but defined nowhere in phase 04 or phases 00-03; Decision clashes with the phase 03 authz Decision struct (phase-03.md:374) | section 2 value objects, next to ChangeRequestService (line 851) | grep -rln "ActorRef\|StepUpAssertionId" docs/ (only phase-04.md); grep "struct Decision\|enum Decision" phase-04.md (none); grep "Activated" other phases (none)
GAP V-04-01-b | LOW | T7 | emergency-review escalation event named `escalated` in 3.2 but `task_escalated` in the 3.4 event list | section 3.2 emergency review (line 939) or section 3.4 event list (line 985) | grep -n "escalated" phase-04.md

### V-04-02 GV-01 register an inventory item before use (POST /inventory)   (source: header GV-01; result: GAPS 1)
- T1 SQL: PASS - admin.inventory_items with kind CHECK, object_id, owner_id, validator_id, status, last_validated_at and uq_inventory_items_object (phase-04.md:512-531); INSERT grant at line 544.
- T2 Route: PASS - POST /inventory, scope model.risk, 201 (phase-04.md:785); Cedar inventory.create in the 3.10 table (lines 1401-1420).
- T3 Repository and types: PASS - InventoryService::create(ctx, CreateInventoryItem) (phase-04.md:1337); InventoryRepository insert (line 736); DTOs CreateInventoryItem and InventoryItemView named (line 769).
- T4 Logic: GAP - section 3.9 Create (lines 1339-1346) is a bullet list of fields with refusal codes, not numbered steps, and gives no order of checks. "Owner and validator are active human users" (line 1340) has no refusal code and no HTTP status for an inactive or non-human user. grep -n -i "inactive\|not active" phase-04.md finds no refusal for it in 3.9.
- T5 Audit: PASS - inventory_item.created {kind, object_id, tier, owner_id, validator_id} (phase-04.md:1394 table, 1339-1346 text).
- T6 Cross-references: PASS - admin.users (phase-01.md:52) for owner_id and validator_id; registry objects arrive in phases 14-16 as stated at line 1347.
- T7 Consistency: PASS - section 7.6 rewrites only the validation-date sentence; no conflict with the create rules.
- Gaps:
GAP V-04-02-a | MEDIUM | T4 | Create has no numbered steps and no order of checks; no refusal code for an owner or validator who is not an active human user | section 3.9 Create (line 1340) | grep -n -i "inactive\|not active" phase-04.md (no match in 3.9); grep "active human" docs/plan/phase-01.md (no refusal code found)

### V-04-03 GV-26 post-change review at 30 and 90 days   (source: header GV-26; result: GAPS 1)
- T1 SQL: PASS - workflow_instances.purpose CHECK includes post_review (phase-04.md:334); context JSONB frozen at start (carries the day, line 338); change_requests.activated_at and review columns (phase-04.md:121-168).
- T2 Route: PASS - POST /tasks/{id}/actions/submit-form, scope self, Cedar task.submit_form (phase-04.md:795 and 1401-1420 table).
- T3 Repository and types: PASS - ChangeRequestService::start_post_review(id, day) (phase-04.md:864); TerminalEffect record_post_review (line 693 trait and 3.2 text line 941-947).
- T4 Logic: PASS - post-review paragraph (lines 941-947): purpose, form step with outcome and notes, effect, the unless-reverted rule; timer re-check row in the 3.6 table (lines 1097-1104) with the no-instance-yet-for-that-day check.
- T5 Audit: PASS - change_request.post_review_started and change_request.post_review_recorded named (line 945 and table line 1386).
- T6 Cross-references: PASS - phase 24 sync_post_review reads admin.instance_events kind post_review_recorded (phase-24.md:665); phase 00 timer owner list includes cr_post_review for change_request (phase-00.md:865).
- T7 Consistency: GAP (LOW) - section 3.2 says record_post_review writes the event post_review_recorded (phase-04.md:944), but the 3.4 list of engine events (line 983-985) does not include it, although phase 24 reads that kind from admin.instance_events.
- Gaps:
GAP V-04-03-a | LOW | T7 | post_review_recorded is written as an instance event (3.2) and read by phase 24, but is absent from the 3.4 event list | section 3.4 event list (line 983-985) | grep -n "post_review_recorded" phase-04.md (lines 944-945, 1386 only)

### V-04-04 S-39 capacity reason makes a change material   (source: header S-39; result: PASS)
- T1 SQL: PASS - reason code CHG_OPERATIONAL_CAPACITY tagged capacity (phase-04.md:95); reason_tag CHECK ('risk','capacity') (line 133).
- T2 Route: PASS - rides on POST /change-requests, scope config.author, Cedar change_request.create (phase-04.md:778 and 3.10 table).
- T3 Repository and types: PASS - classification step in ChangeRequestService::create (3.2 step 4, line 875) and class rule in 3.1 (lines 822-830).
- T4 Logic: PASS - 3.1 class steps 1-5 with codes: reason_tag capacity -> material (line 828); E_CAPACITY_NO_BTL when btl_sample is missing (line 829); E_CLASS_LOWERED (line 830).
- T5 Audit: PASS - change_request.created carries class and reason code (phase-04.md:1378 table).
- T6 Cross-references: PASS - phase-16.md:1101 (capacity reason needs a below-the-line sample, E_CAPACITY_NO_BTL) and phase-16.md:1794 (btl_sample resolves a replay); approver outside operations is the CHK second approval in cr_material (phase-04.md:1305-1308, label line 1307); DSL.md:907 gives the same rule.
- T7 Consistency: PASS - no 7.x block changes the capacity rule; 7.13 narrows only emergency handling.
- Gaps: none.

### V-04-05 UX-14 My inbox (task half)   (source: header UX-14; result: GAPS 1)
- T1 SQL: PASS - reads admin.tasks (phase-04.md:377-447); the inbox indexes idx_tasks_role_open, idx_tasks_pool_open, idx_tasks_claimed, idx_tasks_completed (migration 0015 index block after line 377) are the ones section 3.7 cites (line 1160).
- T2 Route: PASS - GET /inbox (self, line 787), GET /inbox/counts (line 788), GET /tasks/{id} (line 789), claim and release (790-791).
- T3 Repository and types: PASS - InboxService list and counts (line 1148); InboxQuery, TaskRow, InboxCounts DTOs (DTO table, lines 745-768); TaskRepository inbox and counts (section 2 repository list, lines 716-735).
- T4 Logic: PASS - views mine, pool, waiting, history with the membership rules (lines 1150-1156), one query per view, keyset order, visibility and masked 404 (lines 1158-1181). Prose, but each view and filter is specified.
- T5 Audit: N/A - read-only views.
- T6 Cross-references: GAP (LOW) - section 3.7 says the client polls GET /inbox/counts every 60 seconds (phase-04.md:1178); the phase 08 X3 screen polls every 30 seconds (phase-08.md:396).
- T7 Consistency: PASS - 7.4 changes only the case_acl rule, which the inbox reads from the same place.
- Gaps:
GAP V-04-05-a | LOW | T6 | polling interval for /inbox/counts differs between phase 04 (60 s) and phase 08 (30 s) | section 3.7 (line 1178) or phase-08.md:396 | grep -n "60 seconds\|every 60\|30 s" docs/plan/phase-04.md docs/plan/phase-08.md

### V-04-06 UX-60 risk or capacity tag shown on every volume-reducing change   (source: header UX-60; result: GAPS 1)
- T1 SQL: PASS - reason_tag TEXT NOT NULL CHECK ('risk','capacity') copied at creation (phase-04.md:133).
- T2 Route: PASS - GET /change-requests/{id}, scope as the list (phase-04.md:779).
- T3 Repository and types: PASS - ChangeRequestView carries all columns except proposed and impact, so reason_tag is included (phase-04.md:748).
- T4 Logic: N/A - display requirement; the tag is always stored (NOT NULL), so no refusal or step is needed.
- T5 Audit: N/A - no mutation.
- T6 Cross-references: GAP (MEDIUM) - the requirement is carried in the header, but section 5 (phase-04.md:1528-1547) names no element that shows the tag, and phase 08 names no reason tag on the change-request list or detail (grep -n "capacity\|reason tag\|risk tag" docs/plan/phase-08.md: no match).
- T7 Consistency: PASS - no conflicting rule.
- Gaps:
GAP V-04-06-a | MEDIUM | T6 | the tag shown on volume-reducing change requests is not placed on any screen (phase 04 section 5, phase 08 change-request list and detail) | section 5 of phase 04 and the change-request screens of phase 08 | grep -n "capacity\|reason tag\|risk tag\|tag" docs/plan/phase-08.md (no screen element); grep -n "UX-60" docs/plan/phase-04.md (header and section 1 comment only)

### V-04-07 7.1 WorkflowSubject::check_before_decision   (source: 7.1 block; result: GAPS 1)
- T1 SQL: N/A - a method on a trait, no table.
- T2 Route: N/A - no route of its own; it runs inside the engine act envelope (phase-04.md:1062).
- T3 Repository and types: PASS - signature given in the 7.1 block (phase-04.md:1642-1644): check_before_decision(&self, tx, inst, actor, task) -> Result<(), WorkflowError>, default Ok(()).
- T4 Logic: PASS - act step 6 and the 7.1 text (phase-04.md:1646-1650): runs in the same transaction before the action applies; a refusal rolls back and the task stays open.
- T5 Audit: N/A - a check, no mutation of its own.
- T6 Cross-references: PASS - phase-18.md:1129 (phase 18 section 4.5 item 1) states the same signature for CaseWorkflowSubject; phase 20 implementers are named in 7.1 and not checked.
- T7 Consistency: GAP (MEDIUM) - section 3.2 says the change target's failures "are returned as 409 E_STALE_BASE, 422 E_EVIDENCE_STALE or 422 E_VALIDATION" (phase-04.md:898, decide paragraph), but ChangeTarget::check_before_decision returns Result<Vec<ValidationIssue>, ChangeError> (phase-04.md:651) and the subject method returns WorkflowError (7.1). The base text does not say which issues in the Vec refuse (warnings and errors are both ValidationIssue, line 660), and no ChangeError to WorkflowError mapping is given.
- Gaps:
GAP V-04-07-a | MEDIUM | T7 | the change subject's check returns a Vec of ValidationIssue, but the text does not say which issues refuse, and no ChangeError to WorkflowError mapping exists | section 2 ChangeTarget trait (line 651) and section 3.2 decide paragraph (line 898) | grep -n "ValidationIssue" phase-04.md (lines 651, 660, 768, 875); grep -n "WorkflowError" phase-04.md (no From or mapping for ChangeError)

### V-04-08 7.2 evidence upload route for change requests   (source: 7.2 block; result: GAPS 4)
- T1 SQL: GAP (HIGH) - the steps append to change_requests.evidence and bump version (phase-04.md:1660-1665), but the only UPDATE grant on admin.change_requests (phase-04.md:206-208) lists route, instance_id, status, submitted_at, decided_at, checker_id, stepup_id, activated_at, failure_reason, review_instance_id, review_due_at, review_outcome, reverted_at, updated_at, version. The column evidence is not in it, so akr_app cannot write the append. The column itself exists (line 135).
- T2 Route: GAP (MEDIUM) - POST /change-requests/{id}/evidence has scope and status rules in the 7.2 route row (phase-04.md:1658), but no Cedar action. The 3.10 Cedar table (lines 1401-1420) has no evidence action, and the 3.10 text names only change_request.approve and change_request.reject as aliases.
- T3 Repository and types: GAP (MEDIUM) - the append needs a repository write, but ChangeRequestRepository (phase-04.md:716-721) has no method for it. EvidenceStore::stage is named and exists (phase-02.md:839). The request and response types (the evidence id result, 201) are given only as prose.
- T4 Logic: GAP (MEDIUM) - the refusal sub-code evidence_closed depends on "no approve or reject event in the current pass" (phase-04.md:1660-1661). The engine writes one event kind for both, action_taken (phase-04.md:983), so the condition as written cannot be checked. Steps are numbered in the 7.2 text only as one line; the order of the checks is given.
- T5 Audit: PASS - change_request.evidence_added {evidence_id, kind} is named in the step (phase-04.md:1663).
- T6 Cross-references: PASS - EvidenceStore::stage (phase-02.md:839) and the change_request owner kind (phase-02.md:1260-1261, 7.10 of phase 02).
- T7 Consistency: PASS - the freshness rule of 3.2 is restated and not changed (phase-04.md:1664-1666).
- Gaps:
GAP V-04-08-a | HIGH | T1 | the append to change_requests.evidence has no UPDATE grant for the evidence column | section 1, migration 0014 GRANT UPDATE on admin.change_requests (phase-04.md:206-208) | grep -n "evidence" phase-04.md (the grant at 206-208 lists no evidence; evidence appears only in the table DDL at 135 and the classification list at 218-222)
GAP V-04-08-b | MEDIUM | T2 | the new route has no Cedar action named | 7.2 route row (phase-04.md:1658) and the 3.10 Cedar table (lines 1401-1420) | grep -n "evidence" phase-04.md (no Cedar action for evidence)
GAP V-04-08-c | MEDIUM | T3 | no repository method appends the evidence item and bumps the version | section 2 ChangeRequestRepository (phase-04.md:716-721) | grep -n "append\|add_evidence" phase-04.md (no match in section 2)
GAP V-04-08-d | MEDIUM | T4 | the evidence_closed condition refers to "approve or reject" events, but the engine event list has only action_taken | 7.2 steps (phase-04.md:1660-1661) and section 3.4 event list (line 983) | grep -n "action_taken\|approve\` or \`reject\` event" phase-04.md

### V-04-09 7.8 notice requests from reminders, escalations and the notify effect   (source: 7.8 block; result: PASS)
- T1 SQL: N/A - writes an outbox row (phase 00 outbox), no new table in phase 04.
- T2 Route: N/A - no route.
- T3 Repository and types: PASS - TerminalEffect trait (phase-04.md:693); NotifyEffect params and the outbox payload defined in phase 22 (phase-22.md:821).
- T4 Logic: PASS - 7.8 text (phase-04.md:1711-1716): remind and escalate write the outbox row in their own transaction; the notify effect writes its row; no row before phase 22.
- T5 Audit: PASS - the mutations on the task (reminded_at, escalated_at) have their events task_reminded and task_escalated (phase-04.md:1112-1117, 3.6 timeouts; event list at line 985).
- T6 Cross-references: PASS - phase-22.md:821 (NotifyEffect params, notify.request row, remind and escalate codes task.overdue and task.waiting, to holders or escalation_target); phase-22.md:579 (the closed codes of section 1.2); phase-22.md:193-194 (the two codes exist as templates).
- T7 Consistency: PASS - 3.6 says the notice arrives with phase 22 (phase-04.md:1113) and 3.4 delta 9 says notify is deferred until phase 22 (line 1020), both consistent with 7.8.
- Gaps: none.

### V-04-10 7.10 one setting caps an assessor grant   (source: 7.10 block; result: GAPS 1)
- T1 SQL: PASS - the change is a stated column change in the block (default 30, floor {"max": 30, "basis": ...}) (phase-04.md:1725-1729). The copy-paste seed row still reads '90' with floor NULL (phase-04.md:590).
- T2 Route: PASS - the setting key uses route cr_access through the access. prefix rule of the route map (phase-04.md:571).
- T3 Repository and types: N/A - a setting read by the existing SettingsService (phase-00.md:571).
- T4 Logic: PASS - AS grant rule in 3.3 (valid_to within the cap) with the 7.10 cap of 1 to 30 (overrides the 3.10 bounds of 1 to 365).
- T5 Audit: N/A - the key is changed through the setting path, whose audit setting.updated already exists (phase-00.md:651).
- T6 Cross-references: PASS - phase-24.md:1260 (the assessor.grant_max_days key is not created; resolved by the 7.x block of phase 24) and phase-25.md:1359 (reads access.assessor_max_days). Note: phase-24.md:464 still seeds assessor.grant_max_days in its base text; the phase 24 block resolves it.
- T7 Consistency: GAP (LOW) - the block overrides the seed in prose only; the copy-paste DDL in section 1 still seeds 90 with floor NULL (phase-04.md:590-591), so the two disagree until someone edits the seed.
- Gaps:
GAP V-04-10-a | LOW | T7 | the seed row in section 1 still reads 90 with floor NULL, but 7.10 changes the default to 30 with a max floor in prose only | section 1 settings seed (phase-04.md:590-591) | grep -n "assessor_max_days" phase-04.md (lines 590, 1480, 1725-1729)

### V-04-11 7.13 emergency changes never touch access (ADV-1-01)   (source: adversarial run 1; result: PASS)
- T1 SQL: N/A - no table; the example key stepup.max_failures exists (phase-01.md:375).
- T2 Route: N/A - no route; the refusal happens in ChangeRequestService::classify (3.1 step 5, phase-04.md:826-828).
- T3 Repository and types: PASS - classify is named in the 3.1 heading and the refusal sits in its step 5.
- T4 Logic: PASS - 7.13 (phase-04.md:1749-1756) gives the kinds and key prefixes, the 422 E_VALIDATION code with reason emergency_not_allowed, and the order (before any route). The access route stays cr_access or cr_access_adm.
- T5 Audit: N/A - a refusal writes no mutation.
- T6 Cross-references: PASS - the key prefixes match the route map (phase-04.md:571); stepup.max_failures exists (phase-01.md:375); the error code and the reason name exist (3.10 at 1367).
- T7 Consistency: PASS - 3.1 step 5 allows emergency only when the target allows it (phase-04.md:826-828); the route map seed keeps its first emergency rule (line 566), and the block says the rule then applies only to other kinds. Not a contradiction because the block names what it changes.
- Gaps: none.

### V-04-12 7.14 rework is a reject mode (ADV-1-10)   (source: adversarial run 1; result: GAPS 1)
- T1 SQL: PASS - tasks.pass SMALLINT DEFAULT 1 (phase-04.md:383) and the unique deciders per pass indexes (lines 429-430).
- T2 Route: N/A - rework is a reject mode, not a route; the reject route is at phase-04.md:782.
- T3 Repository and types: N/A - no new method.
- T4 Logic: GAP (MEDIUM) - the block says rework "returns the instance to the maker", but it does not say which step or task the maker receives, what the reject mode field looks like in the definition, or which step the instance goes back to. The DSL reference has no rework entry (grep -rn "rework" docs/DSL.md returns nothing). The base text (3.4 delta 8, phase-04.md:1018-1019) says only that pass increments.
- T5 Audit: PASS - rework_returned is an engine event (phase-04.md:983); the reject audit task.rejected covers the action (3.10 at 1390).
- T6 Cross-references: GAP is included in T4 (DSL.md has no rework shape). phase-18.md:1437-1438 and 1529 say the case definitions use mode end and that the phase 04 task actions have no rework: consistent with 7.14, not a contradiction.
- T7 Consistency: PASS - pass numbering and the maker exclusion agree with the unique indexes (lines 429-430) and the excluded rule (3.5, lines 1070-1094).
- Gaps:
GAP V-04-12-a | MEDIUM | T4 | rework return target (the step or task the maker receives) and the reject mode shape in the definition are not specified; DSL.md has no rework entry | section 3.4 delta 8 (phase-04.md:1018-1019) or DSL.md reject section | grep -rn "rework" docs/DSL.md (no match); grep -n "rework" docs/plan/phase-04.md (lines 383, 983, 1018-1019, 1222)

## Summary

- Items checked: 12 (six header items, four gap-pass blocks 7.1, 7.2, 7.8, 7.10, two adversarial run 1 blocks 7.13 and 7.14).
- Items fully PASS: 3 (V-04-04 S-39 capacity, V-04-09 7.8 notice requests, V-04-11 7.13 emergency changes).
- GAP lines by severity: HIGH 1, MEDIUM 8, LOW 4 (13 in total).

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-04-01-a | MEDIUM | T3 | ActorRef, StepUpAssertionId, Decision (approve and reject payload; phase 03 already uses the name for its authz decision record) and Activated are used but not defined | section 2 value objects, before ChangeRequestService (phase-04.md:851-866) |
| V-04-01-b | LOW | T7 | emergency-review escalation event is `escalated` (3.2) but `task_escalated` (3.4) | phase-04.md:939 or 985 |
| V-04-02-a | MEDIUM | T4 | inventory Create has no numbered steps, no order of checks, and no refusal for an owner or validator who is not an active human user | section 3.9 Create (phase-04.md:1339-1346) |
| V-04-03-a | LOW | T7 | post_review_recorded is written as an instance event (3.2) and read by phase 24, but is missing from the 3.4 event list | section 3.4 event list (phase-04.md:983-985) |
| V-04-05-a | LOW | T6 | /inbox/counts polling interval: 60 s in phase 04, 30 s in phase 08 | phase-04.md:1178 and phase-08.md:396 |
| V-04-06-a | MEDIUM | T6 | the risk or capacity tag required by UX-60 is shown on no screen (phase 04 section 5, phase 08 change-request screens) | phase-04.md:1528-1547; phase-08.md change-request screens |
| V-04-07-a | MEDIUM | T7 | the change subject's check returns a Vec of ValidationIssue without saying which issues refuse, and no ChangeError to WorkflowError mapping exists | phase-04.md:651, 660, 898 and the 7.1 block (1642-1650) |
| V-04-08-a | HIGH | T1 | the append to change_requests.evidence has no UPDATE grant on the evidence column | section 1 migration 0014 GRANT (phase-04.md:206-208) |
| V-04-08-b | MEDIUM | T2 | POST /change-requests/{id}/evidence names no Cedar action | 7.2 route row (phase-04.md:1658) and the 3.10 Cedar table (lines 1401-1420) |
| V-04-08-c | MEDIUM | T3 | no repository method appends the evidence item and bumps the version | section 2 ChangeRequestRepository (phase-04.md:716-721) |
| V-04-08-d | MEDIUM | T4 | evidence_closed refers to "approve or reject" events, but the engine writes action_taken only | 7.2 steps (phase-04.md:1660-1661) and event list (line 983) |
| V-04-10-a | LOW | T7 | the section 1 seed still reads 90 with floor NULL; 7.10 changes it in prose only | phase-04.md:590-591 |
| V-04-12-a | MEDIUM | T4 | rework return target and the reject-mode shape are not specified; DSL.md has no rework entry | section 3.4 delta 8 (phase-04.md:1018-1019) or DSL.md reject section |

Structural note outside the item gaps: phase-04.md has no "## Decisions" and no "## Open doubts" section (the phase template requires both, and b3-conventions orders them before "What This Phase Does NOT Touch").
