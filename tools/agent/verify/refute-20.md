# Refuter, phase 20 (Haiku draft for lead review)

### V-20-01-a  STANDS  (first agent: LOW; refuter: LOW)
- Claim: NewOpsTask, the input of OpsTaskRepository::insert, has no field list; the first agent calls this a plan-wide convention.
- Evidence: phase-20.md:480 "insert(tx, NewOpsTask) -> OpsTask" is the only mention of NewOpsTask (grep -n 'NewOpsTask\b'; no "struct NewOpsTask" in docs/plan). The row content is spread over phase-20.md:536 "OpsTaskService::create(tx, NewOpsTaskSpec {" and phase-20.md:540 "Insert the row (`state = open`, `version = 1`, `created_by` NULL for the system)." The convention is not uniform: phase-00.md:535 names the fields of NewJob inline.
- Fix: phase 20, section 2, after the repository traits (line 480): one sentence tying NewOpsTask to NewOpsTaskSpec and to the columns that section 3.1 derives (task_ref, neutral code, state, version).

### V-20-01-b  PARTLY  (first agent: MEDIUM; refuter: LOW)
- Claim: RefrainService::on_task_completed is called at step 7 (line 552) but no section defines it; section 3.7 has no completion hook and the refrain row has no completion column to record into.
- Evidence: Refuted in part. phase-20.md:552 "both only record that the task is done; the level changes only through the check" states the behaviour of both hooks, and phase-20.md:664 "records nothing and changes no state" states the freeze hook. Completion is recorded on the task row (phase-20.md:550, "Set `state = completed`, `completed_by = actor`"), so a no-op refrain hook is buildable. Standing part: grep -i complet over the refrain DDL (phase-20.md:168-196) finds nothing, and no signature is given (grep "on_task_completed": lines 552 and 664 only).
- Fix: phase 20, section 3.7: one sentence stating what RefrainService::on_task_completed does, and reconcile "only record" (line 552) with "records nothing" (line 664).

### V-20-01-c  PARTLY  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: cannot-complete (line 555) names no refusal for a task already completed, a task claimed by another user, or an inactive reason code.
- Evidence: Refuted for two of the three cases. phase-20.md:555 "state not `completed`; `reason_code` is `CLS_OPS_TASK_CANNOT_COMPLETE`" plus phase-20.md:766 "`NotAllowed { sub }` | `E_NOT_ALLOWED_IN_STATE` | 409" give the status for a completed task. An inactive code takes the phase 18 status (phase-18.md:840: 422 E_VALIDATION, sub-code reason_code_inactive), reused per phase-20.md:775.
- Stands: a task claimed by another user. phase-20.md:555 says only "lock as in complete"; the E_CLAIMED refusal is written for complete at phase-20.md:548 and is not repeated for cannot-complete.
- Fix: phase 20, section 3.1, cannot-complete paragraph (line 555): one sentence naming the E_CLAIMED refusal for a task claimed by another user.

### V-20-01-d  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the due time of an operations task. Phase 09 registers the clock ops.task.due, and phase 24 maps it to casework.ops_tasks.due_at; phase 20 never reads it and sets due_at from ops.freeze_task_hours or the refrain clock.
- Evidence: phase-09.md:771-772 "`ops.task.due` (working days, bounds" / "1 to 10)"; phase-24.md:741 "| `ops.task.due` | `casework.ops_tasks.due_at` (20) |"; phase-20.md:705 "`due_at = now() + ops.freeze_task_hours`". grep -n "ops.task.due" docs/plan/phase-20.md finds nothing, and no 7.x block or decision says which clock sets due_at or that ops.task.due is unused.
- Fix: phase 20, section 3.1 creation step 1 or section 3.14: one sentence saying which clock sets due_at for each task source, or that ops.task.due is unused (phase 24 row to match).

### V-20-01-e  REFUTED  (first agent: LOW; refuter: none)
- Claim: create_ops_task is said to be called by phase 15's exit_decision definition, but phase-15.md never names it; the call is in DSL.md.
- Evidence: phase-15.md:1799 "-> `create_ops_task` (phase 20; a no-op until phase 20 is live)." sits in the exit_decision definition (phase-15.md:1798 "`exit_decision`: `approve` per the authority matrix (phase 09) -> `record_decision`"). phase-15.md:2031 "### 7.2 exit_decision and create_ops_task (phase 20, DSL E.4)" names it again. The first agent's grep "none" is wrong: grep -n "create_ops_task" docs/plan/phase-15.md returns lines 1799, 2031 and 2033.

### V-20-02-a  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: FreezeError is the port's return type (used at line 641, NotFreezeList), section 3.9 calls it "phase 14's port error, a variant set of this type", but phase 14 defines no FreezeError and the variant set is never listed.
- Evidence: phase-14.md:773 "Result<FreezeStart, FreezeError>" names the type in the trait and nowhere defines it (grep -rn "enum FreezeError\|FreezeError {" docs/plan finds nothing; phase-14.md mentions it at line 773 only). phase-20.md:758 "`FreezeError` is phase 14's port error, a variant set of this type". The only variant marked as port-only is phase-20.md:773 "(the port only; the decision of phase 14 never sends a non-freeze list)".
- Fix: phase 20, section 3.9 (or section 3.4 Start): list the FreezeError variants, or say that the port returns OpsError.

### V-20-02-b  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: freezes.listing_decided_by is NOT NULL (line 127), but no start step names its value; the port candidate carries decided_by (phase-14.md:776) and the phase never says to use it.
- Evidence: phase-20.md:127 "listing_decided_by UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT"; phase-14.md:776 "pub decided_by: Uuid, pub first_decider: Uuid, pub decided_at"; phase-20.md:678, which names the value only for the backfill: "`listing_decided_by` = the latest `alert.screening_decisions.decider_id` with outcome `true`". Start steps 1-10 (phase-20.md:640-650) and the clock table (lines 630-636) never name it. The candidate carries two deciders (decided_by and first_decider), so the choice matters.
- Fix: phase 20, section 3.4 Start (step 1 or step 4): one sentence naming the source of listing_decided_by and which of the two candidate fields it takes.

### V-20-02-c  STANDS  (first agent: LOW; refuter: LOW)
- Claim: NewFreeze, the input of FreezeRepository::insert_if_absent (line 482), has no field list; same convention as V-20-01-a.
- Evidence: phase-20.md:482 "insert_if_absent(tx, NewFreeze) -> Option<Freeze>"; the insert statement is at phase-20.md:644 "INSERT ... ON CONFLICT (screening_alert_id) DO NOTHING RETURNING *". The columns are the freezes DDL (phase-20.md:115-161) and the values are in steps 1-6 and the clock table. grep -n "NewFreeze" docs/plan/phase-20.md: line 482 only.
- Fix: phase 20, section 2, repository traits: one sentence tying NewFreeze to the freezes columns that Start sets.

### V-20-03-a  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: section 3.14 says the phase 04 setting change target refuses a value outside 1 to 8, but phase 04 checks bounds only from its registry; phase 20 adds no registry entry in section 4.5 and the settings row has floor NULL.
- Evidence: phase-20.md:921 "the phase 04 `setting` change target refuses a value outside them" (the refusal the section promises); phase-04.md:956 "value type, registry bounds, floor (`E_BELOW_FLOOR`)"; research.md:45 (DR-34) "The settings registry declares hard bounds for product limits". The setting row has floor NULL (phase-20.md:361) and phase-20.md:359 says the service checks the bounds instead. grep -c freeze_task_hours docs/plan/phase-04.md: 0. Section 4.5 (lines 968-980) adds no bounds; item 8 lists the setting only as a phase 00 extension.
- Fix: phase 20, section 4.5 item 8: add the 1 to 8 bounds to the settings registry entry (DR-34), or in section 3.14 say that only FreezeService enforces them.

### V-20-04-a  STANDS  (first agent: MEDIUM; refuter: HIGH)
- Claim: curl step 6.m expects role A2 to register an Office instruction (200), but the Cedar policy permits refrain.register for role M only; the route row has scope case.decide, which A2 holds.
- Evidence: phase-20.md:896 "Akr::Action::"refrain.register"], resource)" and phase-20.md:897 "when { principal.roles.contains("M") };" permit refrain.register for role M only; phase-20.md:900 gives A2 read only. phase-20.md:1041 "`a2@` `POST /api/v1/refrain-instructions` with the step-up header and `received_at` on a Monday: 200" is the curl expectation. API.md:86 gives A2 the case.decide scope. No Open doubt covers A2 registering (Open doubts 5, 18 and 19 concern other things).
- Fix: phase 20, section 3.12 (lines 894-897) or section 6 step 6.m (line 1041): make one of them the rule; if A2 may register, add an Open doubt for the bank.

### V-20-04-b  PARTLY  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: close (line 746) names no refusal for a refrain already closed and no If-Match; a second close writes a second lift task and overwrites closed_by and closed_at.
- Evidence: Refuted for If-Match: API.md:43 "Mutations on versioned resources require `If-Match`" is the general rule, and the refrain row has a version column (phase-20.md:188).
- Stands: no state rule for close. phase-20.md:746 gives a rule for refer only: "`refer` requires state `open` or `confirmed` (else 409 `E_NOT_ALLOWED_IN_STATE`)". The guard at phase-20.md:310 "IF OLD.state = 'closed' AND NEW.state <> 'closed' THEN" refuses only a move out of closed. The word "terminal" (phase-20.md:1027 "a closed refrain or incident is terminal") names a test of that guard and does not refuse a second close.
- Fix: phase 20, section 3.7 close paragraph (line 746): one sentence stating that close on a closed refrain is refused (409 E_NOT_ALLOWED_IN_STATE).

### V-20-04-c  STANDS  (first agent: LOW; refuter: LOW)
- Claim: RefrainService::register, close and refer and the refrain on_level are named (lines 536, 742, 746) without signatures; the inputs follow from RegisterRefrain (line 466) and CloseRefrain (line 467).
- Evidence: phase-20.md:536 "by `RefrainService::register` and `RefrainService::close` (section 3.7)"; phase-20.md:746 "**Refer, close** (section 3.6 for the letter)". The steps of section 3.7 (lines 734-746) give the behaviour; no signature is given (grep -n RefrainService: lines 536, 552, 732 and 957 only). The refrain's on_level behaviour is stated at phase-20.md:608 and 742.
- Fix: phase 20, section 3.7: one sentence per method naming its input command type.

### V-20-04-d  STANDS  (first agent: LOW; refuter: LOW)
- Claim: NewRefrain, the input of RefrainRepository::insert (line 483), has no field list; same convention as V-20-01-a.
- Evidence: phase-20.md:483 "`RefrainRepository`: `insert(tx, NewRefrain)`"; the register step 4 names the inserted values: phase-20.md:738 "Insert `refrain_instructions` (state `open`, `refrain_ref` from the sequence, `registered_by = actor`)". grep -n "NewRefrain": line 483 only.
- Fix: phase 20, section 2, repository traits: one sentence tying NewRefrain to RegisterRefrain and the values of register step 4.

### V-20-06-a  PARTLY  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the freeze referral (section 3.6, line 730; route row 507) has no state rule, no refusal code for a released or already referred freeze, and no If-Match; a second call overwrites referral_letter_id and referred_at.
- Evidence: Refuted for If-Match: API.md:43 "Mutations on versioned resources require `If-Match`" is the general rule and covers the freeze referral route too.
- Stands: phase-20.md:730 "then `set_referred` (freeze: `referred_at`, `referral_letter_id`; refrain: state `referred`)" names no state rule. The refrain referral has one at phase-20.md:746 ("`refer` requires state `open` or `confirmed`"). The freezes guard (phase-20.md:264-293) refuses a second Committee report and a second report ("the Committee report is requested once", line 287) but nothing for referral.
- Fix: phase 20, section 3.6 (line 730): one sentence stating the state rule and the refusal for a freeze referral, as section 3.7 does for the refrain.

### V-20-06-b  STANDS  (first agent: LOW; refuter: LOW)
- Claim: no service method is named for the freeze referral or the refrain referral route.
- Evidence: phase-20.md:730 names LetterRequester, set_referred (a repository method, line 482) and the audit events, but no service method. Route rows phase-20.md:507 and 511 name none. grep -n "refer(" docs/plan/phase-20.md: none.
- Fix: phase 20, section 3.6: one sentence naming the service method behind each referral route.

### V-20-07-a  STANDS  (first agent: MEDIUM; refuter: HIGH)
- Claim: step 5 says start_context freezes freeze.reason_code, but the FreezeWorkflowSubject attributes are state, subject_kind, tier and has_case, and "the attributes above" does not include it.
- Evidence: phase-20.md:689 "is frozen by `FreezeWorkflowSubject::start_context` as `freeze.reason_code`". phase-20.md:694 "attributes `state` (text), `subject_kind` (text), `tier` (text, always `suspicion` for a freeze), `has_case` (boolean)". phase-20.md:695 "`start_context(conn, freeze_id)`: the attributes above". grep -n "freeze.reason_code" docs/plan/phase-20.md: line 689 only. Impact is small: release_freeze writes the constant DEC_FREEZE_DELISTED (phase-20.md:703), so no effect reads the attribute. The contradiction itself is what makes it HIGH.
- Fix: phase 20, section 3.5 FreezeWorkflowSubject (line 694): add the attribute to the list, or remove it from step 5 (line 689).

### V-20-07-b  PARTLY  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: release of a freeze with no target accounts. Step 5 (line 705) takes the accounts from the freeze task, which a counterparty or payment-party freeze never has; OpsTaskService::create refuses zero accounts, so the approval rolls back every time; no rule covers it.
- Evidence: Refuted for the account mapping, which is a designed deferral: phase-20.md:1121 "No account set is derivable from these subjects" and phase-20.md:647 "the freeze stays `listed` until a person acts (Open doubts 6)".
- Stands: phase-20.md:705 takes "the freeze's target accounts (from the freeze task)"; phase-20.md:537 "`target_accounts` has 1 to 500 ids (else `E_VALIDATION`, sub-code `no_account`)"; phase-20.md:702 "failure rolls the approval back (the phase 04 rule for effects)". Propose accepts a listed freeze (phase-20.md:685). No line says what a release does for a freeze with no task.
- Fix: phase 20, section 3.5 step 5 (line 705): one sentence saying whether a release of a freeze without a task is refused at propose or records the decision without a lift task.

### V-20-07-c  PARTLY  (first agent: MEDIUM; refuter: LOW)
- Claim: propose step 4 (line 688) requires DEC_FREEZE_DELISTED but states no refusal status or code for any other reason code.
- Evidence: Refuted for a wrong-family or inactive code. phase-20.md:775 lists `reason_family_mismatch` and `reason_code_inactive` as reused phase 18 sub-codes and says "They ride in `errors[].code` or in `sub` of a 403 or 409 body". phase-18.md:840 "(422 `E_VALIDATION` at `/reason_code`, sub-code `reason_code_inactive`)" gives the status.
- Stands: a valid active decision code other than DEC_FREEZE_DELISTED fits neither sub-code, and the closed list (phase-20.md:775) has no other sub-code for it. phase-20.md:688 "`reason_code` must be `DEC_FREEZE_DELISTED`" is silent on it.
- Fix: phase 20, section 3.5 propose step 4 (line 688): name the sub-code for that case, from the closed list of section 3.9.

### V-20-07-d  STANDS  (first agent: LOW; refuter: LOW)
- Claim: FreezeService::propose_release has no signature; the inputs are the freeze id and ProposeFreezeRelease (line 464).
- Evidence: phase-20.md:680 heading "### 3.5 Release after delisting (`FreezeService::propose_release`, `FreezeWorkflowSubject`, `release_freeze`; M-34, RP-59)"; the steps at phase-20.md:684-690 give the behaviour; phase-20.md:464 "`ProposeFreezeRelease { reason_code: String }`: only `DEC_FREEZE_DELISTED`". No signature (grep -n propose_release: lines 680, 800 and 1061 only).
- Fix: phase 20, section 3.5: one sentence stating the signature (freeze id and ProposeFreezeRelease).

### V-20-07-e  REFUTED  (first agent: LOW; refuter: none)
- Claim: the 3.12 comment says check_before_decision is in phase 18 section 3.19 (line 861); 3.19 is the Cedar section; the hook is in phase 18 section 3.8 and section 4.5 item 1.
- Evidence: phase-18.md:1129 "extended with the method `check_before_decision(&self, tx: &mut Tx, ...)`" (phase 18 section 4.5 item 1, the definition). phase-18.md:758 "d. `CaseWorkflowSubject::check_before_decision` (extends phase 04 `WorkflowSubject`, see section 4.5)" sits in section 3.8 (heading at phase-18.md:750). The name exists; only the section number in the phase 20 comment (phase-20.md:861) is wrong, which the rules do not count as a gap. phase-18.md:962 is "### 3.19 Cedar actions and the file `91_cases.cedar`".

### V-20-07-f  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the 7.1 block makes propose_release create one release task per group, but base step 5 creates the lift task inside release_freeze at approval; the two rules put the task at different moments.
- Evidence: phase-20.md:705 "5. Create the lift task: `OpsTaskService::create` with action `release`" (inside release_freeze, which runs on approve, phase-20.md:700). phase-20.md:1061 "- Section 3.5: `propose_release` creates one release task per group." under the 7.1 heading, scoped by phase-20.md:1054 "From phase 26 on, and only while `pack.islamic` is on". The 7.1 text does not say that it replaces step 5, so the timing is still two rules.
- Fix: phase 20, section 7.1 bullet 3 (line 1061): one sentence saying whether the release task is created at proposal or at approval, and that the rule replaces step 5 for phase 26 on.

### V-20-08-a  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the check reads the task and its owner without a lock (line 588), and no step of the check takes them before the writes at steps 7-10; the route check (line 500) and the sweep job (line 623) can run together and raise the level twice.
- Evidence: phase-20.md:588 "1. Read the task and its owner without a lock" conflicts with phase-20.md:917 "each job takes its locks in the order above" (section 3.13), and no 7.x block resolves it. Overlap is allowed: phase-20.md:934 gives ops_canon_check concurrency 2, and phase-00.md:120-121 says "a job already running does not block the next one". phase-00.md:547 (handlers are idempotent) does not cover two runs at once, because the new-debit rule (phase-20.md:593) reads the earlier rows.
- Fix: phase 20, section 3.3 check step 1: say that the check takes the owner and then the task FOR UPDATE before step 2, as section 3.13 requires; and section 3.13: name the check path.

### V-20-08-b  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: n in "holds_matched = n" (lines 592, 597, 606) is not defined; holds_matched counts holds (line 589), so an account with two matching holds (same reference, two types) never reaches V1.
- Evidence: phase-20.md:589 "counted as `holds_matched`" (holds are counted, not accounts) and phase-20.md:592 "`L* = v2` when `holds_matched = n`"; n is also used at phase-20.md:597 and 606. No sentence defines n, and grep finds no definition in section 3.3. The DDL (phase-20.md:102) gives holds_matched no unit.
- Fix: phase 20, section 3.3 steps 2 and 5: define n, and say whether holds_matched counts holds or accounts.

### V-20-08-c  STANDS  (first agent: LOW; refuter: LOW)
- Claim: NewVerification, the input of OpsVerificationRepository::insert (line 481), has no field list; same convention as V-20-01-a.
- Evidence: phase-20.md:481 "`OpsVerificationRepository`: `insert(tx, NewVerification)`"; check step 7 names the inserted values at phase-20.md:594 "insert `ops_verifications` (kind `canonical_check`, level `new_level`, `post_order_debits = new_debits`, counts, `canon_as_of`)". grep -n "NewVerification": line 481 only.
- Fix: phase 20, section 2, repository traits: one sentence tying NewVerification to the values of check step 7.

### V-20-08-d  REFUTED  (first agent: LOW; refuter: none)
- Claim: check step 7 calls "the owner's on_level" (line 594), but on_level is defined for freezes only (line 660); the refrain's V1 transition is in section 3.7 and not linked from step 7.
- Evidence: phase-20.md:608 "5. Owner: `on_level(v0)` (freeze: section 3.4; refrain: no change, a refrain is confirmed only at V1)." states the refrain branch of on_level for V0 and V1. phase-20.md:742 "The check of section 3.3 reaching V1 sets the refrain `confirmed`" states the V1 transition.

### V-20-09-a  PARTLY  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the OPS Confirm button (section 5.1, line 991) cannot be decided from OpsTaskNeutralView (line 472): no completed_by, no completed_by_me, no verification level, no second_person_only flag.
- Evidence: Refuted for the completer rule. phase-20.md:548 "an unclaimed task is claimed by the completer in the same step", so a completed task always carries its completer as the claimer, and the view field "claimed_by_me: bool" (phase-20.md:472) can tell the screen who completed the item.
- Stands: the copy-shows-the-hold condition. OpsTaskNeutralView (phase-20.md:472) has no verification level or hold flag, while phase-20.md:991 says Confirm shows "when the copy does not show the hold". Only the server answer decides it (phase-20.md:606, 409 copy_matches).
- Fix: phase 20, section 5.1 (line 991): one sentence saying that Confirm is offered on items completed by another operator and that the 409 copy_matches answer decides.

### V-20-09-b  STANDS  (first agent: MEDIUM; refuter: LOW)
- Claim: "a completed item stays visible for the day" (line 990) has no rule in ListOpsTasks (line 469), which takes state and due_before filters only, nor in the Reads rule of section 3.1 (line 557).
- Evidence: phase-20.md:990 "A completed item stays visible for the day with its reference"; phase-20.md:469 "`ListOpsTasks { state?, due_before?, cursor }`"; phase-20.md:557 "return the view of the caller's scope". The list has no completed-since filter; the day window can only be a screen filter on completed_at (phase-20.md:472). Severity is LOW because the screen can apply the window itself.
- Fix: phase 20, section 2 (ListOpsTasks) or section 3.1 Reads: one sentence saying whether the day window is applied by the server or by the screen.

### V-20-10-a  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the release tasks are created at proposal in the block (line 1061) and on approval in the base text (line 705); same contradiction as V-20-07-f.
- Evidence: the same as V-20-07-f: phase-20.md:705 "5. Create the lift task: `OpsTaskService::create` with action `release`" and phase-20.md:1061 "- Section 3.5: `propose_release` creates one release task per group."
- Fix: as V-20-07-f: section 7.1 bullet 3 (line 1061), one sentence saying which moment the release task is created at, and that it replaces step 5.

### V-20-10-b  STANDS  (first agent: LOW; refuter: LOW)
- Claim: the order of the groups is not stated, so "the first group's" task (line 1058) is undefined.
- Evidence: phase-20.md:1058 "`freezes.ops_task_id` holds the unrestricted task when there is one, else the first group's"; phase-20.md:1057 lists the groups in the order "`freeze` (unrestricted), `restrict` (restricted) or `hold` (pooled)". No sentence gives an order rule; grep -n "first group" finds line 1058 only.
- Fix: phase 20, section 7.1 bullet 1 (line 1058): one sentence giving the order of the groups.

### V-20-12-a  STANDS  (first agent: HIGH; refuter: HIGH)
- Claim: the timer handler cannot learn whether the letter is delivered. Section 3.6 offers only LetterRequester::request, the worker AppState carries only that port (line 958), and no read or callback is named in phase 20; phase 21 proposes FreezeService::letter_delivered but phase 20 does not add it.
- Evidence: phase-20.md:719 "async fn request(&self, tx: &mut Tx, req: &LetterRequest) -> Result<Uuid, LetterError>;" is the only port method, and the worker AppState carries only that port (phase-20.md:958). The 7.3 rule needs a delivery read: phase-20.md:1079 "`committee_report_letter_id` not in state `delivered`". phase-21.md:1388 "`FreezeService::letter_delivered` (section" points to a phase 20 section 4.5 item 9, which is about phase 10 (phase-20.md:978). phase-21.md:1264 proposes the method only if gap pass 1 decides, and phase-21.md:1409 says no phase 20 method changes.
- Fix: phase 20, section 3.6 (add the read or a letter_delivered callback to the port) and section 4.3 (wire it into the worker).

## Summary

- REFUTED: 3 (V-20-01-e, V-20-07-e, V-20-08-d)
- STANDS: 20
- PARTLY: 7

Gap IDs with their verdicts: 30 claims in total, each Gap ID handled once. V-20-07-f and V-20-10-a state the same contradiction and get the same verdict and fix.

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-20-01-a | STANDS | LOW | NewOpsTask has no field list and is not tied to NewOpsTaskSpec | phase 20 section 2, line 480 |
| V-20-01-b | PARTLY | LOW | RefrainService::on_task_completed has no signature; "record" names no refrain column | phase 20 section 3.7 |
| V-20-01-c | PARTLY | MEDIUM | cannot-complete names no refusal for a task claimed by another user | phase 20 section 3.1, line 555 |
| V-20-01-d | STANDS | MEDIUM | which clock sets due_at: ops.task.due (phases 09 and 24) is unused by phase 20 | phase 20 section 3.1 step 1 or 3.14 |
| V-20-02-a | STANDS | MEDIUM | FreezeError variants are not listed; the type is defined in no plan file | phase 20 section 3.9 or 3.4 |
| V-20-02-b | STANDS | MEDIUM | the value of listing_decided_by is not named in Start | phase 20 section 3.4 Start |
| V-20-02-c | STANDS | LOW | NewFreeze is not tied to the freezes columns | phase 20 section 2 |
| V-20-03-a | STANDS | MEDIUM | the 1 to 8 bounds are not declared in the settings registry, so the refusal has no data | phase 20 section 4.5 item 8 or 3.14 |
| V-20-04-a | STANDS | HIGH | curl step 6.m (A2 registers, 200) contradicts Cedar (refrain.register for M only) | phase 20 section 3.12 (lines 894-897) or section 6 step 6.m |
| V-20-04-b | PARTLY | MEDIUM | close on an already closed refrain has no refusal rule and writes a second lift task | phase 20 section 3.7, line 746 |
| V-20-04-c | STANDS | LOW | RefrainService register, close and refer have no signatures | phase 20 section 3.7 |
| V-20-04-d | STANDS | LOW | NewRefrain is not tied to RegisterRefrain | phase 20 section 2 |
| V-20-06-a | PARTLY | MEDIUM | freeze referral has no state rule and a second referral overwrites the letter id | phase 20 section 3.6, line 730 |
| V-20-06-b | STANDS | LOW | no service method is named for the referral routes | phase 20 section 3.6 |
| V-20-07-a | STANDS | HIGH | freeze.reason_code is frozen by start_context but is not in the attribute list (no effect reads it) | phase 20 section 3.5, line 694 |
| V-20-07-b | PARTLY | MEDIUM | no rule for approving a release of a freeze that has no task | phase 20 section 3.5 step 5, line 705 |
| V-20-07-c | PARTLY | LOW | no sub-code for a valid active decision code other than DEC_FREEZE_DELISTED | phase 20 section 3.5 propose step 4, line 688 |
| V-20-07-d | STANDS | LOW | FreezeService::propose_release has no signature | phase 20 section 3.5 |
| V-20-07-f | STANDS | MEDIUM | release task created at proposal (7.1) or at approval (step 5); 7.1 does not say it replaces step 5 | phase 20 section 7.1 bullet 3, line 1061 |
| V-20-08-a | STANDS | MEDIUM | the check reads without a lock, while section 3.13 says every job takes its locks | phase 20 section 3.3 step 1 and section 3.13 |
| V-20-08-b | STANDS | MEDIUM | n is undefined; holds_matched counts holds, not accounts | phase 20 section 3.3 steps 2 and 5 |
| V-20-08-c | STANDS | LOW | NewVerification is not tied to the check's step 7 values | phase 20 section 2 |
| V-20-09-a | PARTLY | MEDIUM | the copy-shows-the-hold condition for Confirm cannot be read from the neutral view | phase 20 section 5.1, line 991 |
| V-20-09-b | STANDS | LOW | the day window for completed items is in no list rule (the screen can apply it) | phase 20 section 2 (ListOpsTasks) or 3.1 Reads |
| V-20-10-a | STANDS | MEDIUM | same contradiction as V-20-07-f | phase 20 section 7.1 bullet 3, line 1061 |
| V-20-10-b | STANDS | LOW | the order of the groups is not stated | phase 20 section 7.1 bullet 1, line 1058 |
| V-20-12-a | STANDS | HIGH | the timer handler cannot learn letter delivery; phase 21 names a phase 20 method that phase 20 does not define | phase 20 section 3.6 and section 4.3 |
