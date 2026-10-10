# Gap pass 1, run 4a (targets 17 to 24; Haiku draft for lead review)


## Target phase 17

### G1-17-01 Hibernation gate, breach clock and open-hit clause on the alert pipeline
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-17.md section 3.1 (steps 4b, 6, 9), section 3.9 (queue query), section 3.11 (counts), section 3.10 (visibility), section 3.12 (alert_touch handler)
- Sources: phase-19.md:1697 "a new step 4b, between steps 4 and 5, calls HibernationGate::check"; phase-19.md:1698 (steps 5, 7, 9, 10, 12 skipped on suppression; attach_released); phase-19.md:1699 (queue, counts and visibility clause); phase-19.md:1700 (timer_alert_touch calls BreachService::breach); phase-19.md:1185 (HibernationGate::check port); phase-19.md:275 (casework.hibernation_hits with alert_id, alert_created_at)
- Check: grep -n -i hibernation docs/plan/phase-17.md -> only lines 1954 and 1968 (Open doubts and "does not touch"); grep -c BreachService docs/plan/phase-17.md -> 0; grep -c HibernationGate docs/plan/phase-17.md -> 0; section 3.1 has steps 1 to 13 with no suppression branch; section 3.12 handler writes alert.touch_breached with no breach call.
- Proposed 7.x text:

### 7.1 Hibernation gate, breach clock and open-hit clause (from phase 19 on)
From phase 19 on, the tables `casework.hibernation_hits` and the services `HibernationGate` and `BreachService` exist; before phase 19 this section runs as written. Phase 19 section 4.5 item 4 is the owner of these changes; this note states them where the pipeline is described.
- Section 3.1: a new step 4b, between steps 4 and 5, calls `HibernationGate::check(conn, party_id, scenario_id, measures, now)`. On `Suppress(h)`, steps 6, 8 and 11 run and the hit row (`casework.hibernation_hits`) is inserted after step 6; steps 5, 7, 9, 10 and 12 are skipped (no case is opened or joined, no clock runs, no `case.alert_attached`, no `alert.created`). Otherwise the steps run as written. Step 9 also calls `BreachService::arm(tx, "alert", id, created_at, touch_hours)`.
- New method `attach_released(tx, alert_id, alert_created_at)` (used by the `hibernation_release` job) runs steps 5, 7, 9, 10 and 12 for an existing alert; the clocks start at the release, not at the original `created_at`.
- Sections 3.9 (queue query), 3.10 (visibility predicate) and 3.11 (counts) add the clause `AND NOT EXISTS (SELECT 1 FROM casework.hibernation_hits h WHERE h.alert_id = a.id AND h.alert_created_at = a.created_at AND h.released_at IS NULL)`.
- Section 3.12: the `alert_touch` handler also calls `BreachService::breach(tx, "alert", id, 100)` with its breach write. `BreachService::breach` skips an alert that has an open hit.

### G1-17-02 Phase 18 extensions of the case surfaces (text note)
- Severity: LOW
- Category: Missing Logic
- Target: phase-17.md section 2 (CaseRepository), section 3.8 (Assign), section 1.1 (casework.cases columns, and the migration summary in 1.7), route row of GET /api/v1/cases/{id} (section 4.x route table, line 846)
- Sources: phase-18.md:1123 "Phase 17 AssignmentService: writes the access-list grants ... revokes the previous assignee's grant on reassignment"; phase-18.md:1124 "GET /cases/{id} gains the fields escalation_level, outcome, sealed, open_proposal_id, affordances"; phase-18.md:1126 "CaseRepository: extended with the methods of section 2"; phase-18.md:28 "casework.cases gains escalation_level and the outcome CHECK"; phase-18.md:640 "Assignment (phase 17 AssignmentService, extended here)"
- Check: grep -c "escalation_level" docs/plan/phase-17.md -> 0; grep -c "affordances" -> 0; grep -n "case_acl" docs/plan/phase-17.md -> lines 1227 and 1926 only (visibility, no grant write); section 3.8 Assign writes no grant row.
- Proposed 7.x text:

### 7.2 Phase 18 extensions of the case surfaces (text note)
Phase 18 owns these; this section only names them so that the phase 17 text is not read as final.
- `casework.cases` gains `escalation_level` and the `outcome` CHECK (phase 18 section 1.1, migration 0062). The `case` row written here has neither column set.
- `AssignmentService` (section 3.8, Assign step 4 and Claim) also writes the `casework.case_acl` grants of phase 18 section 3.2 in the same transaction: the assignee gets `work` (reason `assignment`) or `decide` (A2, reason `assignment`), the second worker gets `decide` (reason `second_review`), and a reassignment revokes the previous assignee's grant (`revoked_by` is the actor).
- `CaseRepository` gains the methods of phase 18 section 2 (`set_closure`, `set_reopened`, `set_status`, `set_escalation_level`, `set_workflow_version`).
- `GET /api/v1/cases/{id}` gains the fields `escalation_level`, `outcome`, `sealed`, `open_proposal_id` and `affordances`, and the reporter row for M, as phase 18 section 4.5 item 5 says. The route and its scope are unchanged.

### G1-17-03 Conflict declared on assignment raises an alarm (from phase 19 on)
- Severity: LOW
- Category: Missing Event
- Target: phase-17.md section 3.8 (Assign step 2 and the conflict check of Claim step 7)
- Sources: phase-19.md:1701 "AssignmentService (section 3.8) raises conflict_declared through AlarmService::raise on a declared conflict"; phase-19.md:1303 (alarm kind row `conflict_declared`, raised by phase 17 AssignmentService)
- Check: grep -c "conflict_declared" docs/plan/phase-17.md -> 0; section 3.8 Claim step 7 writes audit `alert.conflict_refused` only; Assign has no alarm step.
- Proposed 7.x text:

### 7.3 Declared conflict on assignment raises an alarm (from phase 19 on)
From phase 19 on, a declared conflict found by the `AssignmentService` conflict check (Claim step 7, and Assign step 2) also calls `AlarmService::raise` with kind `conflict_declared` (subject `case`, supervisor M). A detected conflict that refuses a claim stays audit-only (`alert.conflict_refused`), as written above. Phase 19 section 3.10 owns the alarm row.

### G1-17-04 Break-glass admitting clause in visibility rule 5 and the Cedar assignees (from phase 25 on)
- Severity: MEDIUM
- Category: Missing Policy
- Target: phase-17.md section 3.10 (rule 5 and the closing paragraph), section 3.17 (Cedar `Case.assignees` adapter comment), section 6 check 6 (predicate against Cedar)
- Sources: phase-25.md:786 "services::visibility::build ... gains one admitting clause in the team of rule 5"; phase-25.md:787 "the requester is added to Case.assignees only when the request carries purpose code break_glass"; phase-25.md:1261 (4.5 item 1, "the purpose-keyed break-glass clause"); phase-25.md:1340 ("the phase 17 visibility-against-Cedar test ... extended")
- Check: grep -c -i "break_glass" docs/plan/phase-17.md -> 0; rule 5 (section 3.10) lists the team as assignees, first/second worker and pool grant only; section 6 check 6 has no purpose-keyed case.
- Proposed 7.x text:

### 7.4 Break-glass grant clause in rule 5 (from phase 25 on)
From phase 25 on, section 3.10 rule 5 gains one admitting clause in the team (the phase 18 pattern), and only when the request purpose is `break_glass`: `OR ($purpose = 'break_glass' AND EXISTS (SELECT 1 FROM admin.break_glass_grants g WHERE g.case_id = c.id AND g.requester_id = $1 AND g.status = 'active' AND g.expires_at > now()))`. The analyst network, the purpose code and step-up `record_open` still apply. The Cedar entity adapter adds the requester to `Case.assignees` under the same condition (phase 25 section 3.2 "Use" step 2), so `Case.assignees` in section 3.17 is purpose-keyed. Check 6 of section 6 is extended by phase 25 with a purpose-keyed case.

### G1-17-05 Model rank in the queue order and the model-score route (from phase 27 on)
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-17.md section 3.9 (queue items query step 3), section 3.17 (OpenAPI tag `alerts` and router), section 6 check 9 (MASKED_KINDS probe set of kind `alert`)
- Sources: phase-27.md:498 "The phase 17 items query ORDER BY ... becomes ORDER BY priority_class, model_rank DESC NULLS LAST, score DESC NULLS LAST, created_at, id"; phase-27.md:777 "queue row gains model_band for A1, A2 and M"; phase-27.md:773 "alerts (route 16, added to the phase 17 tag)"; phase-27.md:777 (probe set of kind alert gains route 16)
- Check: grep -n "ORDER BY priority_class, score" docs/plan/phase-17.md -> line 1189 (section 3.9 step 3) only; grep -c "model_band" docs/plan/phase-17.md -> 0; grep -c "model-score" -> 0.
- Proposed 7.x text:

### 7.5 Model rank in the queue order (from phase 27 on)
From phase 27 on, the items query of section 3.9 step 3 reads `ORDER BY priority_class, model_rank DESC NULLS LAST, score DESC NULLS LAST, created_at, id` (`class_score_age`), where `model_rank` is the live score of `alert.model_scores` for the alert (a `LEFT JOIN LATERAL` on `alert_id` and `alert_created_at`, role `live`, latest `scored_at`), and the kill condition of phase 27 section 3.4 applies. The queue row gains `model_band` for A1, A2 and M. The route `GET /alerts/{id}/model-score` (phase 27, route 16) is added to the `alerts` router and the `alerts` OpenAPI tag, and it joins the probe set of kind `alert` in check 9. Phase 27 owns these changes; `alert.alerts` is not altered.


## Target phase 18

### G1-18-01 Watch start, closure gate, network hook and bulk reopen (decision, close and reopen paths)
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-18.md section 3.8 (RecordDecisionEffect, step f), section 3.12 (Close, after steps 2 and 4), section 3.13 (Reopen: new entry point)
- Sources: phase-19.md:1154 "WatchService::start_for_decision(tx, decision, case), called by RecordDecisionEffect of phase 18 section 3.8 step f for utr"; phase-19.md:1124 "the closure gate ... sub-code uncorroborated_explanation"; phase-19.md:1229 "CaseService::close of a child (phase 18 section 3.12, extended) locks the child and its parent in ascending id order"; phase-19.md:1279 "Bulk reopen (POST /qa/bulk-reopen)"; phase-19.md:1704-1706 (4.5 item 5: RecordDecisionEffect, close, reopen_under_batch)
- Check: grep -c "start_for_decision" docs/plan/phase-18.md -> 0; grep -c "uncorroborated_explanation" -> 0; grep -c "reopen_under_batch" -> 0; grep -n "network" in section 3.12 (lines 828 to 844) -> none; section 3.12 steps 1 to 11 have no parent or child lock.
- Proposed 7.x text:

### 7.1 Watch start, closure gate, network hook and bulk reopen (from phase 19 on)
From phase 19 on (its `WatchService`, the `casework.network_edges` rows and the bulk reopen route exist then), this phase gets these calls:
- Section 3.8, `RecordDecisionEffect`, after step 8 (the tier): for a decision of type `utr`, `WatchService::start_for_decision(tx, decision, case)` in the same transaction (the case row is already locked by the effect).
- Section 3.12, `CaseService::close`, after step 4: the closure gate of phase 19 section 3.3. For a case of priority `P0` or `P1` that has at least one current RFI claim, no evidence item and no current claim with `corroboration <> 'none'`, refuse with 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `uncorroborated_explanation`.
- Section 3.12, `CaseService::close`, the network hook: a child case of a network case and its parent are locked in ascending `id` order (phase 19 section 3.7 step 7); when no open child remains, `case.network_children_done` is written on the parent. Closing the parent stays a person's act.
- Section 3.13, new entry point `CaseService::reopen_under_batch(tx, case_id, reason_code, actor, finding_id)`: the reopen rules of this section with the batch assertion of `POST /qa/bulk-reopen` (phase 19 section 3.9) in place of a second assertion. The route consumes the step-up once.

### G1-18-02 Work-item breach clock (from phase 19 on)
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-18.md section 3.6 (Open step; Timer `work_item_due`)
- Sources: phase-19.md:1707 "WorkItemService::open (section 3.6) calls BreachService::arm(tx, "work_item", id, now, casework.work_item_due_hours)"; phase-19.md:1707 (4.5 item 5, the timer handler calls BreachService::breach(tx, "work_item", id, 100)); phase-19.md:1285-1300 (section 3.10 step 1 and the work item alarm row `work_item_breach`)
- Check: grep -c "BreachService" docs/plan/phase-18.md -> 0; section 3.6 Open lists no timer; Timer `work_item_due` text (line 719 onward in 3.6) says "Breach actions and supervisor tasks are phase 19" (no call); `casework.work_item_due_hours` exists in phase 18 (grep count 2) so the setting is present.
- Proposed 7.x text:

### 7.2 Work-item breach clock (from phase 19 on)
From phase 19 on, `WorkItemService::open` (section 3.6, Open) calls `BreachService::arm(tx, "work_item", id, now, casework.work_item_due_hours)` in its transaction. The `timer_work_item_due` handler calls `BreachService::breach(tx, "work_item", id, 100)` with its overdue write (`case.work_item_overdue`). The breach stages and the alarm `work_item_breach` are phase 19 section 3.10. The items' states do not change.

### G1-18-03 Report checks, report sign refusal and the step-up class of the sign step (from phase 21 on)
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-18.md section 3.8 (`CaseWorkflowSubject::check_before_decision`, step-up class of the sign step, c), section 3.8 step g (`ReleaseForFilingEffect`), the decision route row (`POST /cases/{id}/decisions`, section 3.8), Open doubts 3
- Sources: phase-21.md:1253 "CaseWorkflowSubject::check_before_decision gains, for case_report_approval, the report checks of section 3.6 step 3"; phase-21.md:1254 "the route refuses 409 E_NOT_ALLOWED_IN_STATE, sub-code report_sign_required, unless via = report"; phase-21.md:1256 "ReleaseForFilingEffect: the body of section 3.7 of this phase"; phase-21.md:1257 "the step-up class of the sign step is signing (DR-170), where phase 18 section 3.8 c declares decision"; research.md:181 DR-170 "step-up class signing; the phase 18 route ... refuse a report sign task (sub-code report_sign_required)"
- Check: grep -c "report_sign_required" docs/plan/phase-18.md -> 0; grep -c "via" -> 0; phase-18.md:1286 "The sign step of case_report_approval uses step-up class decision"; `ReleaseForFilingEffect` body is named as a stub in section 3.8 step g (grep: 7 hits of release_for_filing, no body).
- Proposed 7.x text:

### 7.3 Report checks on the case report approval (from phase 21 on)
From phase 21 on (the report tables and the release exist then):
- The sign step of `case_report_approval` uses the step-up class `signing` (DR-170), not `decision`. Open doubts 3 is closed by this rule. The Cedar action of the sign step is unchanged.
- `CaseWorkflowSubject::check_before_decision` (section 3.8 d), for an instance of `case_report_approval`, runs the report checks of phase 21 section 3.6 step 3: the report exists, `status = validated`, `frozen_version = current_version`, no block in the latest run, and the act carries `via = "report"`. Otherwise 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `report_sign_required`.
- The decision route (`POST /cases/{id}/decisions`, action `sign` on `case_report_approval`) and the generic task route of phase 04 for a `sign` task of `case_report_approval` or `report_correction` refuse the same way unless `via = "report"`.
- `ReleaseForFilingEffect` (section 3.8 step g) takes the body of phase 21 section 3.7. The kind `release_for_filing` and the resource kind `case` are unchanged. The close gate of section 3.12 step 5 is replaced by phase 21 section 3.10a (`filing_not_acknowledged`), as Open doubts 10 already says.

### G1-18-04 Break-glass grant clause in the case rows (from phase 25 on)
- Severity: MEDIUM
- Category: Missing Policy
- Target: phase-18.md section 3.10 (Case rows, the rule-5 clause and its SQL block)
- Sources: phase-25.md:1261 "phase 18 case rows (phase 18 section 3.10, the rule-5 clause): one admitting clause for an active break-glass grant of the requester"; phase-25.md:786 (the clause text: `OR ($purpose = 'break_glass' AND EXISTS (SELECT 1 FROM admin.break_glass_grants g ...))`); phase-25.md:787 (Cedar Case.assignees adapter adds the requester only for purpose break_glass)
- Check: grep -c "break_glass" docs/plan/phase-18.md -> 0; section 3.10 "Case rows" lists the phase 17 rules 1 to 5 plus the case access list clause only.
- Proposed 7.x text:

### 7.4 Break-glass clause in the case rows (from phase 25 on)
From phase 25 on, the rule-5 clause of section 3.10 (Case rows) gains the same admitting clause as phase 17 section 3.10: `OR ($purpose = 'break_glass' AND EXISTS (SELECT 1 FROM admin.break_glass_grants g WHERE g.case_id = c.id AND g.requester_id = $1 AND g.status = 'active' AND g.expires_at > now()))`. The Cedar `assignees` of `Case` (section 3.19) takes the requester under purpose `break_glass` only (phase 25 section 3.2). The masked-404 rule of section 3.10 is unchanged. Phase 25 owns the table and the grants.

### G1-18-05 AI involvement call and the assistant summary affordance (from phase 28 on)
- Severity: LOW
- Category: Missing Logic
- Target: phase-18.md section 3.8 (`RecordDecisionEffect`), the affordances object of `GET /cases/{id}` (listed in section 4.5 item 5), section 5.1 (W3 affordances)
- Sources: phase-28.md:771 "RecordDecisionEffect calls AssistantInvolvement::for_case once in the decision transaction (section 3.4)"; phase-28.md:771 "GET /cases/{id} affordances gain assistant_summary"
- Check: grep -c "AssistantInvolvement" docs/plan/phase-18.md -> 0; grep -c "assistant_summary" -> 0; `casework.decisions.ai_involvement` exists (phase-18.md:153, default `{"used": false}`), so the column is not missing.
- Proposed 7.x text:

### 7.5 AI involvement on decisions and the assistant summary (from phase 28 on)
From phase 28 on, `RecordDecisionEffect` calls `AssistantInvolvement::for_case` once in the decision transaction (phase 28 section 3.4), and writes the shape it returns into `casework.decisions.ai_involvement`. The default `{"used": false}` is unchanged; before phase 28 the value stays that default. The `affordances` object of `GET /cases/{id}` gains `assistant_summary` (computed on the server, phase 28 Open doubts 20). The W3 panel shows it only when true.

### G1-18-06 Retention lock on case seal (from phase 23 on)
- Severity: LOW
- Category: Missing Integration
- Target: phase-18.md section 3.12 (Seal, step 8 and step 9)
- Sources: phase-23.md:928 "The closing sequence's seal transaction (phase 18 section 3, the case.sealed event) enqueues retention_lock_extend with { case_id } in the same transaction"
- Check: grep -c "retention_lock_extend" docs/plan/phase-18.md -> 0; section 3.12 steps 7 to 10 enqueue nothing.
- Proposed 7.x text:

### 7.6 Retention lock extension on seal (from phase 23 on)
From phase 23 on, the seal transaction of section 3.12 (steps 8 to 10) also enqueues the job `retention_lock_extend` with `{ case_id }` in the same transaction. No route, state or event of this phase changes (phase 23 Open doubts 14 closed; DR-193).

### G1-18-07 Owner of the case review date (clock case.review)
- Severity: MEDIUM
- Category: Missing Fields
- Target: phase-18.md section 1.1 or 1.2 (casework.cases, the review date) and section 3.12 (Closure)
- Sources: phase-24.md:1314 "The clock case.review. Phase 09 names the clock; no phase names the column that holds the case review date. Until phase 18 names it, the board shows no rows"; phase-24.md:738 (register row `case.review`); phase-09.md:770 "case.review (days, bounds 30 to 365)"
- Check: grep -n -i "case.review\|review_due\|review date" docs/plan/phase-18.md -> none; the phase 09 clock gives bounds only, no anchor.
- Proposed 7.x text: UNCLEAR. No source gives the anchor of the clock (the date it starts from) or the column that holds its due date; phase 09 gives only the bounds (30 to 365 days). Two options for the lead: (a) an `admin`-free column on `casework.cases` set when the case opens (name to choose), with the clock days from the registry; (b) the clock anchored on `casework.cases.closed_at` for closed cases only. The lead picks the anchor and the column name; phase 18 then gets the column in section 1.1 and a note in 3.12. Until then the board shows no rows, as phase 24 Open doubts 19 says.

### G1-18-08 Phase 19 text notes: lock ranks, closed sub-code list, parties router
- Severity: LOW
- Category: Incomplete Coverage
- Target: phase-18.md section 3.16 (Lock order), section 3.17 (Errors, closed sub-code list), section 4.x route note of the `parties` router (section 3.14 party page)
- Sources: phase-19.md:1619 "Lock order (phase 18 section 3.16 extended)" with ranks 3 and 4 (casework.rfis ... casework.network_snapshots); phase-19.md:1335 (sub-codes of phase 19 added to the closed list of phase 18 section 3.17; the list ends `forbidden_term`); phase-19.md:1708-1710 (4.5 item 5: lock order, sub-codes, phase 18 party routes gain the graph and network-case routes)
- Check: grep -n "rank" docs/plan/phase-18.md -> lock order uses steps 1 and 2 only; grep -c "watch_needs_utr" docs/plan/phase-18.md -> 0; grep -c "no_network" -> 0.
- Proposed 7.x text:

### 7.8 Phase 19 additions to the lock order, the sub-code list and the parties router (from phase 19 on)
- Section 3.16: phase 19 section 3.15 extends the order with ranks 3 and 4 (rank 3: `casework.rfis`, `casework.rfi_questions`, `casework.rfi_answers`, `casework.authority_requests`, `casework.internal_reports`, `casework.watches`, `casework.hibernations`, `casework.hibernation_hits`, `casework.alarms`, `casework.qa_samples`; rank 4: `casework.case_acl` grants, `casework.work_items`, `casework.case_events`, `casework.network_edges`, `casework.network_snapshots`). Phase 19 section 3.15 is the text.
- Section 3.17: the 24 sub-codes of phase 19 section 3.11 join this closed list: `recipient_not_liaison`, `tier_forbids_rm`, `contact_not_allowed`, `template_unknown`, `purpose_not_in_pack`, `already_registered`, `watch_needs_utr`, `already_hibernating`, `hibernation_not_allowed`, `no_network`, `not_due`, `uncorroborated_explanation`, `no_eligible_reviewer`, `reviewer_is_closer`, `not_sample_reviewer`, `sample_closed`, `finding_criterion_passed`, `score_mismatch`, `not_bulk_case`, `no_linked_case`, `role_not_permitted`, `not_on_case`, `unknown_party`, `forbidden_term`.
- Section 3.14 (party routes): the graph and network-case routes of phase 19 are added to `api::routes::parties`. The phase 18 party routes are unchanged.


### G1-18-09 Name of the party read rule (PartyReadPredicate) used by phase 26
- Severity: LOW
- Category: Incomplete Coverage
- Target: phase-18.md section 3.10 (Visibility and masking for the party surfaces; name the port)
- Sources: phase-26.md:273 "PartyReadPredicate::allows(principal, party_id, context) (phase 18 section 3.10 party rule, built by phase 17 services::visibility::build for party rows)"; phase-26.md:368 "PartyReadPredicate::allows(ctx, party_id) (phase 18 party rule)"; phase-26.md:761 "the deal read calls its party rule through PartyReadPredicate"
- Check: grep -n "PartyReadPredicate" docs/plan/phase-18.md -> none; grep -c "PartyReadPredicate" docs/plan/phase-17.md -> 0; section 3.10 of phase 18 states the party rule in prose and SQL but names no port; phase 17 has no party rows (its visibility is for alerts and cases), so the attribution to phase 17 in phase-26.md:273 is not supported.
- Proposed 7.x text:

### 7.9 Name of the party read port (from phase 26 on)
The party read rule of section 3.10 (the tier and team rule for `/parties/{id}` and the transactions of a party) is the port `PartyReadPredicate::allows(principal, party_id, context) -> bool`, implemented in phase 18 and called by the party routes of this phase and by phase 26 (deal reads, which take the customer's party rule). It is built from `casework.effective_tier(party_id)` (phase 17) and the team and access-list clauses of this section. Phase 17's `services::visibility::build` covers alerts and cases only; phase 26's sentence that attributes the party build to phase 17 is for the owner of phase 26 to correct.

## Target phase 19

### G1-19-01 RFI release payload, branch answers path and observation register fields (phase 22)
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-19.md section 3.2 (release step 3, chain payload), section 3.3 (`RfiService::record_answers`, steps 1, 3 and 4), the register detail of `GET /internal-reports` (section 3.4, internal reports)
- Sources: phase-22.md:815 "The case event case.rfi_released ... carries {rfi_id, recipient_kind}"; phase-22.md:816 "(a) The body is split into record_answers_in_tx(tx, actor, rfi_id, answers, source_kind) ... (b) ... an evidence object whose owner kind is rfi_input"; phase-22.md:968 "the register detail gains them for M only (a read extension, not built here)"; phase-19.md:1101 "chain case.rfi_released {rfi_id, kind}"
- Check: grep -c "record_answers_in_tx" docs/plan/phase-19.md -> 0; grep -c "rfi_input" -> 0; grep -c "recipient_kind" in section 3.2 step 3 -> none (the field exists on the RFI row, phase-19.md:990 RfiDeskView); grep -n -i "observation" docs/plan/phase-19.md -> none; phase-19.md:1118 step 4 accepts owner kind `case` only.
- Proposed 7.x text:

### 7.1 Branch answers and the release event (from phase 22 on)
From phase 22 on:
- Section 3.2, `RfiService::release` step 3: the chain event is `case.rfi_released {rfi_id, kind, recipient_kind}` (phase 22 section 4.5 item 1). The dispatcher of phase 22 reads the RFI by `rfi_id`; `recipient_kind` is the RFI column of section 1.
- Section 3.3, `RfiService::record_answers`: the body is split into `record_answers_in_tx(tx, actor, rfi_id, answers, source_kind)`. The desk route calls it after its `rfi.answer` check (the desk's own check, unchanged); the branch route (phase 22 section 3.3) calls it after `branch.item_answer`. The branch path always writes `corroboration = 'none'`.
- Section 3.3 step 4: an evidence object whose owner kind is `rfi_input` and whose owner id is the RFI itself is accepted as well as an owner kind `case` object (the branch's uploads, phase 22 section 4.5 item 3). The owner kind `rfi_input` is registered in phase 02 by phase 22.
- Register detail of `GET /internal-reports`: the observation fields (groups, criteria, notes) are shown to M only. Phase 22 (its item 9, reading observations) says this read extension is "not built here" and no phase names it as its owner; UNCLEAR for the lead to assign (phase 22 or this phase).

### G1-19-02 Assisted and golden QA populations and the weekly draw (phase 28)
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-19.md section 1.x (`casework.qa_samples` CHECKs on `population` and `source_kind`, lines 365 and 368), section 3.8 (population table and draw steps), section 4.1 (job `qa_suppression_sample` and the weekly draw schedule row), section 5 (QA review screen)
- Sources: phase-28.md:772 "casework.qa_samples.population gains assistant_golden and assistant_assisted and source_kind gains assistant_draft (0089)"; phase-28.md:898 "Decisions whose ai_involvement.used is true form the phase 19 population assistant_assisted, drawn weekly at assistant.qa_sample_percent"; phase-28.md:924 "The weekly draw must include assistant_assisted (requirement; phase 19 owner to confirm the draw seed)"; phase-28.md:925 "The QA review screen must show the text of an assistant_golden evaluation draft"; phase-28.md:542 (golden population drawn by POST /qa/samples)
- Check: grep -c "assistant_golden" docs/plan/phase-19.md -> 0; grep -c "assistant_assisted" -> 0; grep -c "assistant_draft" -> 0; population CHECK (phase-19.md:365) lists nine values and none is an assistant value; the weekly schedule row (phase-19.md:1656) is `qa_suppression_sample` only for the weekly job.
- Proposed 7.x text:

### 7.2 Assistant QA populations (from phase 28 on)
From phase 28 on, migration 0089 widens the CHECK constraints of `casework.qa_samples`: `population` gains `assistant_golden` and `assistant_assisted`; `source_kind` gains `assistant_draft`. The DDL of this phase is not edited.
- Section 3.8, population table: `assistant_assisted` = decisions whose `ai_involvement.used` is true (`casework.decisions`, phase 18), drawn weekly at the governed `assistant.qa_sample_percent` (phase 28 settings); `assistant_golden` = the drafts of an evaluation run (`eval_run_id`), drawn by `POST /qa/samples` with `n = assistant.gate_min_items` (phase 28, the evaluation gate).
- Section 4.1: the weekly draw job includes the `assistant_assisted` draw at `assistant.qa_sample_percent`. The draw seed is the phase 19 owner's decision (phase 28 item 13).
- Section 5 (QA review screen): the text of an `assistant_golden` draft is shown to the QA roles who review that sample (phase 28 item 14).

### G1-19-03 Alarm kinds and subject kinds added by phases 20, 21, 22 and 31
- Severity: LOW
- Category: Incomplete Coverage
- Target: phase-19.md section 1.x (`casework.alarms` kind CHECK, line 297; subject kind CHECK, line 300), section 3.10 (alarm kinds table)
- Sources: phase-20.md:970 "the AlarmKind enum gains PostOrderDebit, FreezeOverdue, RefrainOverdue; the CHECKs of casework.alarms are widened by 0066; the table of phase 19 section 3.10 gains three rows"; phase-21.md:372-379 (migration 0070: `report_filing_overdue`, `report_rework_overdue`; subject kinds `report`, `report_task`); phase-22.md:40-47 (widened: `liaison_report_overdue`; subject kind `liaison_report`); phase-31.md:160-166 (migration 0097: `capacity_forecast`, subject kind `pool`)
- Check: grep -c "post_order_debit\|freeze_overdue\|report_filing_overdue\|liaison_report_overdue\|capacity_forecast" docs/plan/phase-19.md -> 0; the section 3.10 table (phase-19.md:1300 to 1310) has six rows (touch_breach, work_item_breach, chain_broken, conflict_declared, flat_line, qa_critical).
- Proposed 7.x text:

### 7.3 Alarm kinds and subject kinds of later phases (owner notes)
The kind and subject lists of section 1.x are widened by later migrations, each owned by the phase that adds the kind. This phase's DDL is not edited.
- Kinds: `post_order_debit`, `freeze_overdue`, `refrain_overdue` (phase 20, migration 0066; subjects `freeze` and `refrain`; supervisor M); `report_filing_overdue`, `report_rework_overdue` (phase 21, migration 0070; subjects `report` and `report_task`); `liaison_report_overdue` (phase 22; subject `liaison_report`); `capacity_forecast` (phase 31, migration 0097; subject `pool`, stage NULL).
- Subject kinds also added: `freeze`, `refrain`, `ops_task`, `report`, `report_task`, `liaison_report`, `pool`.
- Section 3.10 table: one row per new kind, raised by the owning phase's service through `AlarmService::raise`. The rule that `stage` is NULL for every kind other than the two breach kinds applies to all of them.

### G1-19-04 RFI attach_pack, correspondent_list and two route rows (phase 31)
- Severity: LOW
- Category: Missing Endpoint
- Target: phase-19.md section 3.1 to 3.3 (RfiService), the route table of this phase (the `rfis` router rows)
- Sources: phase-31.md:443 "RfiService::attach_pack and RfiService::correspondent_list (extend phase 19 RfiService, section 4.5)"; phase-31.md:447 "rfis::router() (phase 19; two routes added here)"; phase-31.md:937 "A Correspondents filter on the existing desk list (phase 19 section 5.2)"
- Check: grep -c "attach_pack\|correspondent_list" docs/plan/phase-19.md -> 0; the route table of phase 19 (rows at phase-19.md:1027 to 1031 and after) has no pack attach row.
- Proposed 7.x text:

### 7.4 Pack attach and correspondent list (from phase 31 on)
From phase 31 on, `RfiService` gains `attach_pack` (attach a built pack to an outbound correspondent RFI) and `correspondent_list` (the correspondent desk list). The `rfis` router gains the two routes phase 31 names in its route table; the correspondent filter of the desk list is phase 31's (section 5). The rest of this phase is unchanged.

### G1-19-05 Surge quality sampling (phase 31)
- Severity: LOW
- Category: Missing Logic
- Target: phase-19.md section 3.8 (QA sampling, the rate used by the draw, step 2), section 3.8 "Sample size"
- Sources: phase-31.md:905 "While capacity.surge_active is TRUE, the sampler of phase 19 section 3.9 takes max(its rate, capacity.surge_qc_percent); otherwise it is unchanged"; phase-19.md:1874 (Open doubts 10: surge mode not built here)
- Check: grep -c "surge" docs/plan/phase-19.md -> 3 (Open doubts and "not built" lines only); section 3.8 has no surge term. Note: phase 31 cites "section 3.9" for the sampler; in phase 19 the sampler is section 3.8 (3.9 is QA reviews). The section number is taken from the sampler, not from the citation.
- Proposed 7.x text:

### 7.5 Surge quality sampling (from phase 31 on)
From phase 31 on, while the setting `capacity.surge_active` is TRUE (phase 31 settings), the rate used by the draw of section 3.8 is `max(its rate, capacity.surge_qc_percent)`. Otherwise the rate is unchanged. A surge removes no sample. Open doubts 10 stays for the rest of M-54.

### G1-19-06 Break-glass requester and Rfi.assignees (phase 25)
- Severity: LOW
- Category: Missing Policy
- Target: phase-19.md section 3.13 (Cedar, the `Rfi.assignees` comment, line 1361) and section 3.2 (RFI visibility, `RfiWorkflowSubject::visible_to`, line 1107)
- Sources: phase-25.md:1083 "team membership also feeds later team checks (phase 19 Rfi.assignees)"; phase-25.md:788 "file 51_platform.cedar forbids every action outside a list of fifteen read actions when context.purpose == break_glass ... so writes, decisions, access changes, RFIs ... are refused"
- Check: grep -c "break_glass" docs/plan/phase-19.md -> 0; the Cedar comment (phase-19.md:1361) says Rfi.assignees is the case team (assignees, pool members and access list) with no purpose condition.
- Proposed 7.x text:

### 7.6 Break-glass purpose and the RFI team (from phase 25 on)
From phase 25 on, `Rfi.assignees` is the case team as this section states, and a break-glass grantee enters it only under purpose `break_glass` (phase 25 section 3.2 step 2). Under that purpose file `51_platform.cedar` refuses every RFI action (phase 25 section 3.12), so the grantee has read access only where that file allows it. No change to the `RfiWorkflowSubject` rules of section 3.2.


## Target phase 20

### G1-20-01 Account-class groups for freeze tasks (phase 26, flag pack.islamic)
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-20.md section 3.4 (Start, steps 6 and 9), section 3.5 (propose_release step 5, lift task)
- Sources: phase-26.md:762 "FreezeService::start step 6 groups target accounts by class (3.8); freeze.created gains the group counts"; phase-26.md:504 "Pooled-fund freeze (I-10; FreezeService::start step 6 of phase 20)"; phase-26.md:512 "Target accounts are grouped by class. One ops task per non-empty group"; phase-26.md:524 (step 7 audit groups); phase-26.md:525 (step 8 release tasks per group); phase-26.md:358 (the flag `pack.islamic`)
- Check: grep -n "class\|group" docs/plan/phase-20.md in section 3.4 -> none for account class or group; step 6 creates one task (`OpsTaskService::create` with `action: freeze`, all target accounts); step 9 payload has no groups; section 3.5 step 5 creates one `release` task for all target accounts.
- Proposed 7.x text:

### 7.1 Account-class groups for freeze tasks (from phase 26 on)
From phase 26 on, and only while the flag `pack.islamic` is on (with the flag off every account is `unrestricted`, so the single task below is the shipped behaviour):
- Section 3.4 step 6: the target accounts are grouped by `AccountClass` (phase 26 section 3.8 step 1, through the setting `islamic.account_class_map`). One `OpsTaskService::create` per non-empty group, with action `freeze` (unrestricted), `restrict` (restricted) or `hold` (pooled), and the neutral text code of that action (phase 26 section 3.8 step 2). `freezes.ops_task_id` holds the unrestricted task when there is one, otherwise the first group's task; the other group tasks are found by `casework.ops_tasks.source_id = freeze.id`.
- Section 3.4 step 9: the audit `freeze.created` gains `groups: {unrestricted: n, restricted: n, pooled: n}` (counts only).
- Section 3.5 step 5: the lift task is created once per group of the freeze's target accounts (action `release`), as phase 26 section 3.8 step 8 says.
- A two-person rule for pooled holds and the routing to Finance and the Sharia board stay open (phase 26 Open doubts 9); this phase creates no task for either.

### Structure note (not a gap finding)
Phase 20 has no "## 7+. Gap Additions" heading (its sections run 6, then Decisions). The lead must add the heading before the 7.1 text above is appended.


## Target phase 21

### G1-21-01 Service method for the report draft PATCH
- Severity: LOW
- Category: Missing Logic
- Target: phase-21.md section 3.1 (Drafts; the PATCH route row of the section 2 route table)
- Sources: phase-28.md:523 "ReportService::update_draft (phase 21 section 3.2, the PATCH of /reports/{id}) with narrative.ar and narrative.en"; phase-28.md:773 "Phase 21: ReportService::update_draft is called by the narrative accept without change"; phase-28.md:9 (binaries touched: ReportService::update_draft)
- Check: grep -c "update_draft" docs/plan/phase-21.md -> 0; section 3.1 names only `ReportService::create_draft` (phase-21.md:766); the PATCH row (phase-21.md:732) names no service method; section 3.2 step 4 (narrative length) belongs to the builder, not to a named method.
- Proposed 7.x text:

### 7.1 Name of the draft update method (PATCH /reports/{id})
The PATCH `/reports/{id}` route (section 2 route table, scope `report.prepare`, `If-Match` on the report version, Cedar `report.edit`) is served by `ReportService::update_draft(tx, report_id, UpdateReportDraft, actor, if_match)`. Its rules are those of this phase (the draft only, the narrative limit of section 3.2 step 4, and `validated` to `draft` clears `validated_run_id`, section 3.5). Phase 28 calls the same method for an accepted narrative (the `report_narrative` branch of the phase 28 accept route), so the name is fixed here.


## Target phase 22

### G1-22-01 Two break-glass notice codes and the "active holders of the role" recipient rule (phase 25)
- Severity: MEDIUM
- Category: Missing Event
- Target: phase-22.md section 3.8 (notice codes and the recipient table of the dispatcher), section 1.2 (the closed template codes and the seed rows of `admin.notification_templates` or the table that holds `NoticeCode`)
- Sources: phase-25.md:806-813 (section 3.3: "Notice code (new, phase 22 NoticeCode extended by two variants and two seed rows in 0082)"; `security.break_glass_waiting` to CH and SEC; `security.break_glass_opened` to SEC and M; "The recipient rule 'active holders of the role' is a new rule of phase 22's dispatcher"); phase-25.md:1262 (4.5 item 17: "NoticeCode gains SecurityBreakGlassWaiting and SecurityBreakGlassOpened ... The fifteen-code list of phase 22 becomes seventeen codes"); phase-22.md:381 (NoticeCode: "the fifteen closed codes of section 3.8"); phase-22.md:577 (a code outside the enum cannot be written)
- Check: grep -c "security.break_glass" docs/plan/phase-22.md -> 0; grep -c "SecurityBreakGlass" -> 0; grep -c "active holders" -> 0 (phase 22 recipient rules are named by role and scope only); phase-22.md:381 and :577 say "fifteen".
- Proposed 7.x text:

### 7.1 Break-glass notice codes and recipient rule (from phase 25 on)
From phase 25 on (migration 0082 seeds the two rows; this phase's DDL is not edited), the closed `NoticeCode` enum of section 1.2 and section 3.8 grows from fifteen to seventeen codes:
- `security.break_glass_waiting` (Rust `SecurityBreakGlassWaiting`): produced by `break_glass.request` (phase 25 step 5) and the sweep of waiting requests; recipients: active holders of CH and of SEC; parameter `count`.
- `security.break_glass_opened` (Rust `SecurityBreakGlassOpened`): produced by `break_glass.approve` (phase 25 step 7); recipients: active holders of SEC and of M; parameter `count`.
- Recipient rule of the dispatcher (section 3.8): a new rule "active holders of the role" (the holders of the role at send time, from `UserRoleQuery::holders`), used by these two codes only. The existing role and scope rules are unchanged.
- Neither text names a case, a party, a requester or a reason. The notice carries no link to the request.


## Target phase 23

### G1-23-01 Disposal handlers for the tables that phases 22, 24 to 29 and 31 add to the retention assignment
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-23.md section 3.2 (disposal registry, the list of `(schema, table)` handlers, line 579 onward), section 3.2 clock list (line 567: `closure` names three owner tables only), section 3.1 (run, ordering rule of the record-purge list)
- Sources: phase-22.md:362-367 (six tables, classes R6, R9, R12); phase-24.md:506-512 (seven tables); phase-25.md:624-634 (eleven tables); phase-26.md:142-143, 213 (three tables, R3); phase-27.md:355-360 (six tables; phase-27.md:360 "the phase 23 engine deletes these rows before it drops an alert partition"; phase-27.md:784 "record_purge, class R5"); phase-28.md:380-383 (four tables; phase-28.md:777 "a draft is purged only with its case ... record_purge"); phase-29.md:790-808 (19 tables; phase-29.md:1512 "a decision partition is dropped only when no hold references it"); phase-31.md:396-404 (nine tables; phase-31.md:504 "leave only through the retention run of phase 23")
- Check: grep -c "assess\.\|train\.\|model_scores\|assistant_drafts\|practice_attempts" docs/plan/phase-23.md -> 0 for each owner table; grep -n "notification" docs/plan/phase-23.md -> none (the R12 notices are swept by phase 22's job, phase-22.md:1000); section 3.2 "R12 ... other R12 tables by maintenance_sweep (phase 00)" (phase-23.md:599) names no table of phases 25, 29 or 31.
- Proposed 7.x text:

### 7.1 Disposal handlers for later tables (each owner phase's retention rows)
Each row of `admin.retention_assignment` that a later phase inserts needs exactly one `DisposalHandler` in this phase's registry (section 3.2 coverage test). The handlers below are added to the registry from the phase that creates the table. Clock and class are the owner's row; the handler uses the disposal rule of section 3.2.
- Phase 22: `casework.observations` (R6, owner), `casework.liaison_monthly_reports` (R6, age `created_at`), `admin.notification_templates` (R9, retirement; never deleted), `admin.notification_template_versions` (R9, owner), `admin.user_notification_prefs` (R9, owner). `admin.notifications` (R12) is not in this engine: it is swept by phase 22's notice sweep (section 4.1 of phase 22, `notify.retention_days`); section 3.2 names only `maintenance_sweep` for R12 tables; its R12 note must name this job too.
- Phase 24: `admin.registers` (R9, closure `closed_at`), `admin.protected_attributes` (R9, retirement), `admin.validation_packs` (R11, age `issued_at`), `admin.fairness_metrics` (R9, age `computed_at`), `admin.training_records` (R9, age `recorded_at`), `admin.assessor_samples` (R9, age `drawn_at`), `admin.verify_runs` (R10, never).
- Phase 25: `admin.access_reviews` (R9, closure `closed_at`), `admin.access_review_items` (R9, owner), `admin.break_glass_grants` (R10, closure `reviewed_at`), `admin.break_glass_log` (R10, owner; the guard refuses DELETE), `admin.releases` (R10, never), `admin.help_articles` (R9, retirement; the guard refuses DELETE), `admin.support_sessions` (R9, closure `ended_at`), `admin.support_bundles` (R9, age `created_at`), `admin.anchor_timestamps` (R10, owner). `admin.siem_cursors` (R12, retirement) and `admin.siem_dead_letters` (R12, age `created_at`) are R12: swept by `maintenance_sweep` (phase 00), not by this engine (see the R12 rule of section 3.2).
- Phase 26: `canon.islamic_deals` (R3, closure `closed_on`), `canon.deal_parties` (R3, owner), `kyc.possession_evidence` (R3, owner).
- Phase 27: `admin.models` (R11, retirement), `admin.model_versions` (R11, retirement), `admin.model_evaluations` (R11, age `created_at`), `admin.model_monitor_points` (R9, age `computed_at`), `admin.model_kill_state` (R10, never), `alert.model_scores` (R5, owner): the handler `record_purge` deletes a partition's score rows before the partition drop of section 3.2 (the composite foreign key refuses the drop otherwise).
- Phase 28: `admin.assistant_versions` (R11, retirement), `admin.assistant_evaluations` (R11, age `created_at`), `admin.assistant_kill_state` (R10, never), `casework.assistant_drafts` (R6, owner): purged in `record_purge` before the case row (the foreign key is RESTRICT).
- Phase 29: `assess.rails`, `assess.gateways`, `assess.signing_keys`, `assess.amount_bands`, `assess.fail_policies`, `assess.rail_modes`, `assess.policy_state`, `assess.prohibited_rules`, `assess.control_state` (R9, retirement); `assess.decisions` (R7, partition `received_at`: a partition is dropped only when no `assess.holds` row references it, and only through `admin.retention_drop_partition`); `assess.idempotency` (R12, age `created_at`; swept by `assess_ledger_sweep`, not by this engine); `assess.node_status` (R12, age `last_seen_at`; same rule); `assess.blind_periods` (R10, never); `assess.holds` (R7, closure `resolved_at`); `assess.hold_candidates` (R7, owner); `assess.release_exceptions` (R7, closure `closed_at`); `assess.control_tests` (R11, age `recorded_at`); `assess.fallback_extracts` (R7, age `built_at`); `assess.mode_rollups` (R7, age `day`).
- Phase 31: `train.practice_cases` (R9, retirement); `train.practice_attempts` (R9, age `attempted_at`; the guard admits only `akr_retention`); `casework.capacity_inputs` (R9, retirement); `casework.capacity_snapshots` (R12, age `computed_at`) and `casework.capacity_runs` (R12, age `requested_at`), both swept by `maintenance_sweep`; `casework.rfi_packs` (R6, owner); `rpt.typology_exposure` (R7, age `computed_at`); `admin.directory_users` (R9, retirement); `admin.directory_events` (R12, age `received_at`; swept by `maintenance_sweep`).
- Clock resolvers: the `closure` rows above name owner columns of their own table (`closed_at`, `ended_at`, `resolved_at`, `reviewed_at`, `closed_on`), not the three owners named in section 3.2 (`casework.cases`, `casework.freezes`, `rpt.report_tasks`). Section 3.2 names the resolver for each of them in the registry.
- R12 rows of phases 25, 27, 29 and 31 are swept by phase 00's `maintenance_sweep` or by the owner's own job; phase 00 must list them in its sweep (owner: phase 00, not this phase).

### G1-23-02 Scope of akr_retention DELETE (DR-198) against the later grants
- Severity: MEDIUM
- Category: Missing Policy
- Target: phase-23.md DR-198 (line 1036) and section 1.2 (grants of `akr_retention`)
- Sources: phase-23.md:1036 "It has DELETE only on the tables of the party tree and the KYC family"; phase-29.md:429 and 632 "akr_retention gets SELECT and DELETE on every new table (phase 23)"; phase-27.md:290 and phase-28.md:314 "Retention: own SELECT and DELETE on every new table"; phase-31.md:282 "GRANT SELECT, DELETE ON train.practice_attempts TO akr_retention"; phase-26.md:116 "DELETE: akr_retention only, as phase 23 gives it for canon"
- Check: DR-198 names two groups only; phases 27, 28, 29 and 31 give DELETE on tables outside the party tree and the KYC family (admin.models, casework.assistant_drafts, assess.*, train.practice_attempts); phase 26 gives DELETE on canon.islamic_deals, which is not named as party tree in phase 23.
- Proposed 7.x text: UNCLEAR. Two sources disagree: DR-198 limits DELETE to the party tree and the KYC family; the later phases grant DELETE on every table of their retention rows. The lead decides which: (a) DELETE for `akr_retention` on every table of the assignment table with a clock other than `never` (then DR-198 is amended and the coverage test checks the grant), or (b) DELETE only on the party tree and KYC family, with the other owners' grants removed. Until decided, no grant is proposed here.


## Target phase 24

### G1-24-01 Register sources, fairness metric values and closure codes added by phases 27, 28 and 29
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-24.md section 1.1 (`admin.registers`, `source` CHECK), section 1.2 (`admin.fairness_metrics`, `metric` CHECK), section 3.4 (closure codes of the register update)
- Sources: phase-27.md:780 "the two CHECKs of section 1.1; the fairness_monthly metric score_top_band_rate; the closure-code list ... gains RETRAINED, RECALIBRATED, RESTRICTED_USE, RULES_ONLY and JUSTIFIED_NO_ACTION"; phase-27.md:161-165 (ALTER of registers_source_check and fairness_metrics_metric_check, owner phase 27); phase-28.md:115-116 (ALTER of registers_source_check) and phase-28.md:774 "admin.registers.source gains assistant_monitor (0088)"; phase-29.md:653-656 (ALTER adds assess_control)
- Check: grep -n "CHECK (source IN" docs/plan/phase-24.md -> line 42, values manual, post_review, fairness, review_calendar, scan only; grep -c "model_monitor\|assistant_monitor\|assess_control" docs/plan/phase-24.md -> 0; grep -n "CHECK (metric IN" -> line 137, values alert_rate, case_rate, str_rate, hold_rate only; grep -c "score_top_band_rate" -> 0; grep -c "RETRAINED" -> 0.
- Proposed 7.x text:

### 7.1 Register sources, fairness metric and closure codes of later phases (owner ALTERs)
The CHECK constraints named here are widened by the owners' own migrations (phase 27 `registers_source_check` and `fairness_metrics_metric_check`; phase 28 `registers_source_check` in 0088; phase 29 `registers_source_check`). This section's DDL is not edited.
- `admin.registers.source` after phases 27, 28 and 29: `manual`, `post_review`, `fairness`, `review_calendar`, `scan`, `model_monitor` (phase 27), `assistant_monitor` (phase 28), `assess_control` (phase 29, DR-307). For source `model_monitor` the register update refuses a closure code outside the five of phase 27 with 422 `E_VALIDATION`, sub-code `closure_code_not_allowed`. The closure-code list of section 3.4 is the union of its own codes (`EFFECTIVE`, `RULE_CHANGED`, `COMMITTEE_ACCEPTED`, `NOT_REPRODUCED`, `DUPLICATE`) and the five of phase 27 (`RETRAINED`, `RECALIBRATED`, `RESTRICTED_USE`, `RULES_ONLY`, `JUSTIFIED_NO_ACTION`).
- `admin.fairness_metrics.metric` after phase 27: `alert_rate`, `case_rate`, `str_rate`, `hold_rate`, `score_top_band_rate` (value defined by phase 27 section 3.5).

### G1-24-02 Review calendar rows and command-centre tiles added by phases 26, 27, 28 and 29
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-24.md review calendar subsection (`review_calendar(ctx, horizon_days)`, the row kinds), the finding closure rule of section 3.4, section 5.3 (G1 command centre tiles)
- Sources: phase-27.md:780 "the review calendar gains the derived row kind model_kill_drill ... and the derived row of a starter family's recalibration_due" and "command-centre tile model_governance"; phase-28.md:774 "the review calendar gains row assistant_kill_drill"; phase-29.md:1349 "assess_control_test (due assess.control_test_max_days after the last caught test per rail); the G1 command centre gains the Assessment tile (section 3.15); the register route of finding closure closes release exceptions"; phase-26.md:763 "the G1 tiles of section 5.3 (counts only)"; phase-26.md:790 (phase 26 section 5.3 G1 command centre, two tiles, no route)
- Check: grep -c "model_kill_drill\|assistant_kill_drill\|assess_control_test" docs/plan/phase-24.md -> 0; grep -c "model_governance" -> 0; the review calendar row kinds listed at phase-24.md:653 have no model, assistant or assess row; section 5.3 of phase 24 lists its own tiles only.
- Proposed 7.x text:

### 7.2 Review calendar rows and command-centre tiles of later phases
- Review calendar (`review_calendar(ctx, horizon_days)`), derived rows added without a new table, with the states `ok`, `amber` and `red` of this section: `model_kill_drill` (phase 27; due from `admin.models.next_kill_drill_due`; amber 30 days before, red on the due date); the derived row of a starter family's `recalibration_due` (phase 27); `assistant_kill_drill` (phase 28); `assess_control_test` (phase 29; due `assess.control_test_max_days` after the last caught test per rail).
- Finding closure (section 3.4): the register route of a finding whose source is `assess_control` also closes the release exceptions it names (phase 29).
- Command centre (section 5.3): the tiles `model_governance` (phase 27, counts only), `Assessment` (phase 29, section 3.15 there) and the two tiles of phase 26 (counts only; `/command`, no route). The tile definitions, their suppression and their visibility follow this section.

### G1-24-03 Validation pack branches for models and assistant (phases 27 and 28)
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-24.md section 3.6 (`ValidationPackService::issue`, the branch list)
- Sources: phase-27.md:780 "the ValidationPackService::issue model branch (section 3.6)"; phase-28.md:774 "ValidationPackService::issue gains the assistant branch of section 3.9"; phase-28.md:587 "ValidationPackService::issue takes the assistant branch for kinds llm_component and prompt_template, with the seven parts of phase 24 section 3.6"
- Check: grep -n "branch" docs/plan/phase-24.md -> the pack section names no model or assistant branch; grep -c "llm_component\|model" in section 3.6 -> 0.
- Proposed 7.x text:

### 7.3 Model and assistant branches of the validation pack (from phases 27 and 28)
From phase 27 on, `ValidationPackService::issue` (section 3.6) has the model branch that phase 27 defines for the model kinds of its registry. From phase 28 on it has the assistant branch for the kinds `llm_component` and `prompt_template`, with the seven parts of section 3.6 (phase 28 section 3.9). The branches only read; the pack's signature, its verifier and the kinds of `admin.packs` are unchanged by this section.

### G1-24-04 Control report counts for break-glass and overdue access reviews (phase 25)
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-24.md section 3.2 (Access history and the tipping-off control report; the count list of `GET /access/control-report`)
- Sources: phase-25.md:1263 "GET /access/control-report (phase 24) gains two counts with the small-number suppression of phase 24: break-glass grants activated in the month, and access reviews overdue"; phase-25.md:1340 "the phase 24 control report suppression test"
- Check: grep -n -i "break" docs/plan/phase-24.md -> no line in section 3.2 or the control report; the route row at phase-24.md:551 says "counts only" with no break-glass or review count.
- Proposed 7.x text:

### 7.4 Two counts in the control report (from phase 25 on)
From phase 25 on, `GET /api/v1/access/control-report` gains two counts, with the small-number suppression of this section: the number of break-glass grants activated in the month (`admin.break_glass_grants`, phase 25 section 1.1 table), and the number of access reviews overdue at the period end (`admin.access_reviews`, phase 25 section 3.1). Both are counts only, with no names and no request ids.

### G1-24-05 Practice column in the training register (phase 31)
- Severity: LOW
- Category: Missing Logic
- Target: phase-24.md section 3.10 (training register, `TrainingRegisterService::list`, display columns)
- Sources: phase-31.md:502 "The training register (GET /training/register, phase 24 section 3.10) gets one display column practice"; phase-31.md:906 "TrainingRegisterService::list gains the display column practice (section 3.2 rule 4; the column is TrainingService::practice_column). admin.training_records is unchanged"; phase-31 gap note: first attempt only (phase-31.md section 3.2 rule 4)
- Check: grep -n -i "practice" docs/plan/phase-24.md -> none; section 3.10 has no practice column.
- Proposed 7.x text:

### 7.5 Practice column in the training register (from phase 31 on)
From phase 31 on, `TrainingRegisterService::list` (section 3.10) gains the display column `practice`, computed by `TrainingService::practice_column` (phase 31 section 3.2, rule 4; the first attempt only). `admin.training_records` is unchanged and the column is not stored.

### G1-24-06 Warning writer used by phases 26 and 29 (name and signature)
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-24.md the service section (`GovernanceRegisterService`, the register writes), section 1.1 (warnings are opened only by a job, line 38)
- Sources: phase-26.md:274 "RegisterWriter::warn(source, source_key, title_key, detail) (phase 24, upsert with ON CONFLICT (source_key) DO NOTHING)"; phase-26.md:441-443 (sweep warning, source `scan`, `source_key = 'islamic_evidence_gap:' || bank_date`); phase-29.md:1097, 1112, 1132 (warnings in the phase 24 register, source `assess_control`, source keys `assess:blind:` and `assess:release:`)
- Check: grep -n "RegisterWriter\|warn(" docs/plan/phase-24.md -> none; grep -n "GovernanceRegisterService" docs/plan/phase-24.md -> lines 9, 630, 1152 (the service exists); no method of it is named for writing a warning; line 38 says a warning is opened only by a job.
- Proposed 7.x text:

### 7.6 Warning writer of the register (from phases 26 and 29 on)
`GovernanceRegisterService::warn(source, source_key, title_key, detail) -> Option<Uuid>` (the name phase 26 calls `RegisterWriter::warn`; the lead picks one name and phase 26 and phase 29 follow it) inserts one row of kind `warning` with `ON CONFLICT (source_key) DO NOTHING` (the insert returns `None` when the key exists). `source` must be a value of the CHECK of section 1.1 (the values of section 7.1 of this phase). It is called only from a job (section 1.1 rule: a warning is opened only by a job). The `detail` holds counts or ids only, and `title_key` is a key of the bilingual title catalogue.


## Summary

Findings by severity (31 in total, one per target phase and subject):
- HIGH: 0
- MEDIUM: 19
- LOW: 12

Per target: phase 17: 5; phase 18: 9; phase 19: 6; phase 20: 1; phase 21: 1; phase 22: 1; phase 23: 2; phase 24: 6.

Obligations already present (skipped, not written as findings): about 40. Examples checked and found present: the ScreeningCaseLinker port and the casework.effective_tier function (phase 17); the observation source kind and the alert.alerts CHECK (phase 17); the phase 17 chain_broken alarm call (phase 17 section 3.7); the check_before_decision hook and the ai_involvement column (phase 18); the rfi.answer and rfi.released chain event (phase 19); the freeze check_before_decision and the RecordingLetterRequester stub (phase 20); the single_officer_exception setting, the report_filing_overdue alarm and the report.small_number_threshold setting (phase 21); the supervisor.alarm code and the liaison monthly table (phase 22); the retention_lock_extend job, legal_hold_release and record_purge (phase 23).

UNCLEAR (for the lead; no proposal written):
1. G1-18-07: no source names the anchor or the column of the `case.review` clock (phase 09 gives bounds only; phase 24 Open doubts 19 asks phase 18 to name it).
2. G1-19-01 (last paragraph): the observation fields in the register detail are "not built here" in phase 22 and no phase owns them.
3. G1-23-02: DR-198 (DELETE for akr_retention only on the party tree and KYC family) conflicts with the DELETE grants of phases 27, 28, 29 and 31 and with phase 26's grant on canon.islamic_deals.
4. Phase 27 Open item 9 (advance notice to MRO) needs a notification that is not tied to a task; phase 27 says "decide with phase 22's owner". Not a finding; owner decision.
5. Gap-items (q) says the phase 21 liaison periodic kind reads phase 22's table; phase 22 (its note that phase 21's liaison count is unchanged) and phase 21 section 3.16 say it reads the phase 19 internal register. The text of phases 21 and 22 was used; no finding written.
6. Structure: phase 20 has no "## 7+. Gap Additions" heading. The lead adds it before "## Decisions" when G1-20-01 is applied.
7. Outside run 4a (noted only): phase 30 (line 283 of that file) still names the deletion key `akr-sign-cert`, which phase 23 section 7.1 renames to `akr-sign-deletion`. Phase 26 attributes the party read predicate to phase 17; covered in G1-18-09.
