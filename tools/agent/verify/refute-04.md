# Refuter, phase 04 (Haiku draft for lead review)

### V-04-01-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM for two names, LOW for two)
- Claim: ActorRef, StepUpAssertionId, Decision (approve and reject) and Activated are used in signatures but defined nowhere in phases 00-04; Decision also clashes with the phase 03 authz Decision struct.
- Evidence:
  - Refuted part (Decision is "defined nowhere"): the name is defined in phase 03 as another type, phase-03.md:374 "pub struct Decision {". Phase 04 gives the two values the argument takes: phase-04.md:895 "engine.act(task, Approve | Reject, if_match, assertion)" and phase-04.md:1040 "TaskAction::{Approve{comment}, Reject{comment}, Sign{comment}, SubmitForm{form}, Review{comment}}".
  - Activated: the only mention is phase-04.md:652 "-> Result<Activated, ChangeError>;". No fields are named anywhere (grep over docs/plan). The step at phase-04.md:915 "target.activate(tx, cr, actor) -> writes the new object version" uses no returned value.
  - StepUpAssertionId: undefined (grep over docs/ finds only phase-04.md). Its referent is clear from phase-01.md:420 "StepUpAssertion { id, user_id, session_id, action_class" and phase-04.md:1061 "the assertion id goes to `tasks.stepup_id`".
  - ActorRef: undefined. The nearest type is phase-02.md:556 "Actor { User { id, on_behalf_of: Option<Uuid> } | System | Anonymous }".
- Stands: Activated (MEDIUM: the return type has no fields and no step reads it); StepUpAssertionId (MEDIUM: the id type is inferable from phase 01 but not named); ActorRef (LOW: a near name of phase 02 Actor); the Decision name clash with the phase 03 struct (LOW).
- Fix: Phase 04 section 2 next to the ChangeTarget trait (line 642), and the decide signature at line 860: one sentence that says what Activated holds (or that it is unit), which phase 01 and phase 02 types StepUpAssertionId and ActorRef are, and that decide's Decision is the Approve or Reject task action, not the phase 03 struct.

### V-04-01-b  STANDS   (first agent: LOW; refuter: LOW)
- Claim: the emergency review escalation event is named `escalated` in 3.2 but `task_escalated` in the 3.4 event list.
- Evidence: phase-04.md:939 "CH is added to the review instance (`escalated` event)"; phase-04.md:985 "AKR `instance_withdrawn`, `task_reminded`, `task_escalated`, `review_recorded`"; phase-04.md:1117 "event `task_escalated {task_id, escalation_task_id, to}`". No 7.x block names the kind (grep "escalated" in phase 04 7.x: none).
- Fix: Phase 04 section 3.2 emergency review (line 939): use the kind that section 3.4 lists (line 985), or add the review escalation kind to that list.

### V-04-02-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Create in 3.9 has no numbered steps and no order of checks, and no refusal code for an owner or validator who is not an active human user.
- Evidence: phase-04.md:1340 "Owner and validator are active human users"; the only check with a code in that item is phase-04.md:1341 "(422 `E_VALIDATION`, `validator_is_owner`; the CHECK backs it)". The reason list at phase-04.md:1370 "`role_not_grantable`, `taken`, `unknown_resource_kind`, `validator_is_owner`, `self_reference`" holds no inactive-owner reason. Grep "inactive|not active" in phase 04 finds no refusal in 3.9 (the hits at lines 232, 1234 and 1699 concern other objects). The pattern exists in phase 12: phase-12.md:758 "`owner_id` must be an active human holding the data steward role (else 422 `E_VALIDATION`, reason `owner_not_active`)". The general authz rule (phase-03.md:594, "An account that is not active can do nothing") covers actors, not owner or validator references.
- Fix: Phase 04 section 3.9 Create (lines 1339-1346): name the 422 reason for an owner or validator that is disabled or not human, and state in one sentence the order in which the Create checks run.

### V-04-03-a  STANDS   (first agent: LOW; refuter: LOW)
- Claim: post_review_recorded is written as an instance event (3.2) and read by phase 24, but it is absent from the 3.4 event list.
- Evidence: phase-04.md:944 "`record_post_review` writes the event `post_review_recorded {day, outcome}`"; the 3.4 list "Events written" (phase-04.md:981-985) ends with "`review_recorded`, `effect_ran`, `effect_deferred`." Phase 24 reads the kind: phase-24.md:665 "`kind = 'post_review_recorded'` (phase 04)". No 7.x block adds it to the list (grep: lines 944-945, 1386 only).
- Fix: Phase 04 section 3.4 event list (line 985): add post_review_recorded to the list of events written.

### V-04-05-a  STANDS   (first agent: LOW; refuter: LOW)
- Claim: the polling interval for /inbox/counts is 60 s in phase 04 and 30 s in phase 08.
- Evidence: phase-04.md:1178 "The client polls `GET /inbox/counts` every 60 seconds"; phase-08.md:396 "(polled every 30 s while visible; the rail badge reads the same query)", in the X3 inbox screen spec (line 395). phase-17.md:1267 also takes the /inbox/counts cadence from phase 04 as 60 seconds. No 7.x block in phase 04 or phase 08 settles the interval (phase 08 7.1 to 7.5 checked). LOW because both specs can be built as written and the difference is one polling constant; the texts do disagree, so the lead may raise it.
- Fix: Phase 08 screen X3 (line 396) or phase 04 section 3.7 (line 1178): keep one interval and state it in both places.

### V-04-06-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM for the stands part)
- Claim: the risk or capacity tag that UX-60 requires on volume-reducing changes is shown on no screen (phase 04 section 5, phase 08 change-request list and detail).
- Evidence:
  - Refuted part (no screen at all): the wording flag is placed on the change views in docs/DESIGN-SYSTEM.md:641 "the reason code is required and capacity wording is flagged (UX-60)" (section 7.17, change and diff views), and on the D2 scenario studio at phase-16.md:1865 "risk reason, and capacity wording is flagged".
  - Stands: phase-08.md:406-409 (G10) names "class and volume-effect badges with words" and "reason code with label and text", with no risk or capacity tag and no wording flag. The DiffView row at phase-08.md:342 has the same badges only. Section 5 of phase 04 (phase-04.md:1543) names no tag element. Grep "capacity|reason tag|risk tag" in phase 08 finds no screen element.
- Stands: the risk or capacity tag itself is not shown on the phase 08 change-request list or detail (MEDIUM: inferable from DESIGN-SYSTEM section 7.17, which the DiffView row already follows).
- Fix: Phase 08 screen G10 (lines 406-409): name the risk or capacity tag of the reason beside the reason code on the list and the detail, with the capacity-wording flag of DESIGN-SYSTEM section 7.17.

### V-04-07-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM for the mapping)
- Claim: the change subject's check returns a Vec of ValidationIssue, but the text does not say which issues refuse, and no ChangeError to WorkflowError mapping exists.
- Evidence:
  - Refuted part (which issues refuse): phase-04.md:897-898 "failures are returned as 409 `E_STALE_BASE`, 422 `E_EVIDENCE_STALE` or 422 `E_VALIDATION`, and the task stays open." The returned issues are the failures, and the shadow refusal is stated at phase-04.md:968 "`check_before_decision` refuses approval while the bundle has been in shadow".
  - Stands (mapping): the target's signature is phase-04.md:651 "-> Result<Vec<ValidationIssue>, ChangeError>;", while the subject's 7.1 signature is phase-04.md:1644 "-> Result<(), WorkflowError>;". phase-04.md:1370-1371 "`ChangeError` and `WorkflowError` (`thiserror`) carry these and map in `api::error`" maps both to problem codes, not one type to the other. Grep "WorkflowError" in phase 04 finds no conversion and no variant that carries a subject refusal.
- Stands: the conversion from the target's ChangeError to the subject's WorkflowError (MEDIUM: the codes are listed at phase-04.md:897-898, so a developer can infer the mapping, but the carrying variant is not named).
- Fix: Phase 04 section 7.1 (line 1644): name the WorkflowError variant that carries the target's refusal and the codes it maps to, in one sentence.

### V-04-08-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: the append to change_requests.evidence in 7.2 has no UPDATE grant on the evidence column, so akr_app cannot write it.
- Evidence: phase-04.md:206-208 "GRANT UPDATE (route, instance_id, status, submitted_at, decided_at, checker_id, stepup_id, activated_at," with no evidence column in the list. phase-04.md:1552-1553 says akr_app has "UPDATE on the lifecycle columns only", and evidence is not one. phase-04.md:1662 "append `{kind: evidence_object, id}` to `change_requests.evidence`". DATA-MODEL.md:64 "Grants are listed table by table; never `ALTER DEFAULT PRIVILEGES`", so no default grant covers it. Greps: no GRANT in the 7.2 block (phase-04.md:1652-1666); no other phase grants UPDATE on admin.change_requests.
- Fix: Phase 04 section 1, migration 0014 GRANT (lines 206-208): add the evidence column to the UPDATE column list.

### V-04-08-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the new route POST /change-requests/{id}/evidence names no Cedar action.
- Evidence: phase-04.md:1658 route row, whose Scope column lists API scopes only ("config.author, admin.users, admin.policies, admin.system"). The 3.10 action table has no evidence row; the nearest maker-only row is phase-04.md:1419 "`change_request.submit`, `change_request.withdraw`". Phase 04 names only two route actions, phase-04.md:1452 "`change_request.approve` and `change_request.reject`". The general rule does not name it: phase-03.md:485-486 "every route has the Cedar action in the last column of section 3's action table", and phase-03.md:1187 "every operation ... has a `RouteAction` and an `x-akr-action`". Grep "evidence" in the 3.10 region finds no action.
- Fix: Phase 04 section 3.10 action table (lines 1412-1425) and the 7.2 route row (line 1658): name one Cedar action for the evidence route, with its maker and status rule.

### V-04-08-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: no repository method appends the evidence item and bumps the version.
- Evidence: phase-04.md:716-721 ChangeRequestRepository lists insert, get, get_for_update, list, open_for, set_submitted, set_decided, set_effective, set_failed, set_withdrawn, set_review, set_reverted and due_drafts. None appends evidence. The 7.2 step names no method: phase-04.md:1662 "append `{kind: evidence_object, id}` to `change_requests.evidence`; version + 1". The only append in section 2 is EventRepository::append at phase-04.md:730, which writes instance events, not evidence.
- Fix: Phase 04 section 2 ChangeRequestRepository (lines 716-721): add the method that appends one evidence item and bumps the version under the If-Match check, and name it in the 7.2 step.

### V-04-08-d  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: evidence_closed refers to "approve or reject" events, but the engine writes action_taken only.
- Evidence: phase-04.md:1660 "`submitted` with no `approve` or `reject` event in" (7.2 steps). The engine event list at phase-04.md:982 has `action_taken` and no approve or reject kind. phase-04.md:1014 "then `action_taken`" is written for every action, so the event kind does not tell approve from reject. No payload field of action_taken is named as the action (grep: none). The data exists in the tasks table (phase-04.md:468 grants UPDATE on "outcome" for admin.tasks), so the 7.2 check can be made, but the text names no source.
- Fix: Phase 04 7.2 steps (lines 1660-1661): name the field of action_taken, or the tasks outcome of the current pass, that marks an approve or a reject.

### V-04-10-a  REFUTED   (first agent: LOW; refuter: none)
- Claim: the section 1 seed row still reads 90 with floor NULL, but 7.10 changes the default to 30 with a max floor in prose only.
- Evidence: phase-04.md:1726-1727 "Its seed becomes default `30` with the floor `{"max": 30, "basis": ...}`". The 7.10 block names the seed it replaces, so under the 7.x rule the block wins over the base text. DR-361 (docs/plan/research.md:372) records "default 30, maximum 30" and cites phase 04 7.10.
- Note for the lead (not a rule conflict): the copy-paste seed at phase-04.md:590 still reads "'90', 'integer', NULL, TRUE". An SQL edit there would remove the copy-paste mismatch.

### V-04-12-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM for the return target)
- Claim: the rework return target (the step or task the maker receives) and the reject mode shape are not specified; DSL.md has no rework entry.
- Evidence:
  - Refuted part (the reject mode shape): the reject object is written as phase-04.md:1302 "reject": { "comment_required": true, "mode": "end" }, and 7.14 says the mode is the field of a definition's reject: phase-04.md:1759-1760 "it is the `mode` of a definition's reject, not a task action". phase-04.md:1018 "defaults to `mode: end` with `comment_required: true`; `rework` is allowed". A rework value of the same field is inferable.
  - Stands (the return target): phase-04.md:1760 "`rework` returns the instance to the maker" names no step or task. phase-04.md:1019 "increments `pass` on every task created after the `rework_returned` event" does not say which task is created. DR-407 (docs/plan/research.md:418) decides only "rework is a reject mode". Grep "rework" across docs/plan and docs/DSL.md: no step named.
  - DSL.md (LOW, silence and no conflict): docs/DSL.md:1542 uses only "mode": "end" and has no rework entry.
- Stands: the return target (MEDIUM); the DSL silence (LOW).
- Fix: Phase 04 section 7.14 (line 1760): name the step or task the maker receives after a rework, and add the rework value to the DSL reject example at docs/DSL.md:1542 in the same change.

## Summary

- REFUTED: 1 (V-04-10-a)
- STANDS: 8 (V-04-01-b, V-04-02-a, V-04-03-a, V-04-05-a, V-04-08-a, V-04-08-b, V-04-08-c, V-04-08-d)
- PARTLY: 4 (V-04-01-a, V-04-06-a, V-04-07-a, V-04-12-a)

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-04-01-a | PARTLY | MEDIUM (Activated, StepUpAssertionId); LOW (ActorRef, Decision name clash) | Activated and StepUpAssertionId are never defined; ActorRef is a near name of phase 02 Actor; decide's Decision clashes with the phase 03 struct | Phase 04 section 2 next to the ChangeTarget trait (line 642) and the decide signature (line 860) |
| V-04-01-b | STANDS | LOW | `escalated` event (line 939) against `task_escalated` in the 3.4 list (line 985) | Phase 04 section 3.2 (line 939) or the section 3.4 list (line 985) |
| V-04-02-a | STANDS | MEDIUM | No refusal code for an owner or validator who is not an active human user; no order of checks in Create | Phase 04 section 3.9 Create (lines 1339-1346) |
| V-04-03-a | STANDS | LOW | post_review_recorded is missing from the 3.4 event list | Phase 04 section 3.4 event list (line 985) |
| V-04-05-a | STANDS | LOW | /inbox/counts polled every 60 s in phase 04 and every 30 s in phase 08 | Phase 08 screen X3 (line 396) or phase 04 section 3.7 (line 1178) |
| V-04-06-a | PARTLY | MEDIUM | The risk or capacity tag is not on the phase 08 change-request list or detail (G10); a wording flag exists in DESIGN-SYSTEM 7.17 and phase 16 D2 | Phase 08 screen G10 (lines 406-409) |
| V-04-07-a | PARTLY | MEDIUM | No conversion from the target's ChangeError to the subject's WorkflowError, and no variant that carries the refusal | Phase 04 section 7.1 (line 1644) |
| V-04-08-a | STANDS | HIGH | akr_app has no UPDATE grant on change_requests.evidence | Phase 04 section 1, migration 0014 GRANT (lines 206-208) |
| V-04-08-b | STANDS | MEDIUM | POST /change-requests/{id}/evidence names no Cedar action | Phase 04 section 3.10 action table (lines 1412-1425) and the 7.2 route row (line 1658) |
| V-04-08-c | STANDS | MEDIUM | No repository method appends the evidence item and bumps the version | Phase 04 section 2 ChangeRequestRepository (lines 716-721) |
| V-04-08-d | STANDS | MEDIUM | evidence_closed tests approve or reject events that the engine never writes; the source of the check is not named | Phase 04 7.2 steps (lines 1660-1661) |
| V-04-12-a | PARTLY | MEDIUM | The rework return target (the step or task the maker receives) is unnamed; the reject shape is inferable; DSL.md is silent (LOW) | Phase 04 section 7.14 (line 1760) |
