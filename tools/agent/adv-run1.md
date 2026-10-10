# Adversarial pass, run 1 (four-eyes and self-approval; Haiku draft for lead review)

### ADV-1-01 Emergency route takes access-control settings away from SEC   (verdict: FLAW; phase 04)
- Rule: "Access changes never escalate to business roles" (phase-04.md:1274). The route map sends setting keys `authz.`, `identity.`, `stepup.`, `session.`, `delegation.`, `access.` to `cr_access` (phase-04.md:569).
- Attack:
  1. Maker X holds CFG (config.author). X proposes `UpdateSetting` on an access-control key (for example `stepup.max_failures` or a `delegation.*` key) with `emergency: true`, reason `CHG_SECURITY`, and any evidence object that resolves (phase-04.md:831).
  2. The classifier sets class `emergency`. The first route-map rule is `{"class": "emergency", "route": "cr_emergency"}` (phase-04.md:566), so it wins over the access rule at phase-04.md:569.
  3. Route `cr_emergency` needs one approval: `approve` by `named_officer` `mlro` (phase-04.md:1267). Any active M holder other than X approves. No SEC approval is needed.
  4. The change activates in the approval transaction (`activate_change`, phase-04.md:1267). The access-control setting is live at once.
  5. Only a loosening change is reverted after 24 hours without review. A tightening change is never reverted (phase-04.md:936-938). Review of a non-revert emergency still needs two further people (phase-04.md:928-930), but the change is live in the meantime.
- Guard search: route-map order (phase-04.md:566 before 569). Classify step 5 (phase-04.md:831) checks only that the target allows emergency, and the setting target allows it (its Revert column, phase-04.md:956). `grep -n emergency phase-04.md` shows no key-prefix exclusion for emergency. The change_requests CHECKs (phase-04.md:161-174) do not link emergency to the route. No Cedar rule is involved. No guard found.
- Severity: HIGH. A security-policy setting changes on one business approval, which defeats the separation the phase text states at phase-04.md:1274. Four-eyes still holds (two people), so this is a segregation bypass, not a self-approval.
- Proposed 7.x text: "### 7.N Emergency changes never use the access keys". In section 3.1 step 5, add: an emergency `setting` whose key starts with `authz.`, `identity.`, `stepup.`, `session.`, `delegation.` or `access.` is refused with `E_VALIDATION`, `emergency_not_allowed`. The route-map rule `{"class": "emergency", "route": "cr_emergency"}` then applies only to other keys.

### ADV-1-02 Report drafter signs the report built from their own draft   (verdict: FLAW; phase 21)
- Rule: "Four-eyes: the proposer of the instance, the submitter of the report and the earlier deciders are excluded" (phase-21.md:850, step 3 of section 3.6). The DDL CHECK says "maker is not checker (RP-34)" (phase-21.md:107). The DDL compares only `signed_by` with `submitted_by`.
- Attack:
  1. M1 creates the report draft. `created_by = actor` (phase-21.md:778). Scope `report.prepare` is held by A2 and M (tools/agent/b3-conventions.md, scope table).
  2. A2 on the case team edits the draft and submits it. The PATCH is allowed to A2 on the case team (Cedar `report.edit_drafts`, phase-21.md:1166). `submitted_by` = the A2 (phase-21.md:826, step 3).
  3. M1 holds role M and is not the case proposer (an M holder decides and does not propose, phase-18.md:730), not the submitter, and not an earlier decider.
  4. M1 signs the sign task. The engine excludes only the proposer, the submitter and earlier deciders. The DDL CHECK passes because `signed_by` (M1) differs from `submitted_by` (A2). The Cedar permit `report.filers` has no `created_by` rule (phase-21.md:1158-1162). The only forbid on sign is for CTR reports (phase-21.md:1170-1172).
  5. Result: the person who drafted the report approves it after another person submitted it.
- Guard search: phase-21.md:107 (CHECK on `signed_by` and `submitted_by` only). phase-21.md:850 (exclusion list has no drafter). phase-21.md:1158-1172 (Cedar: no `created_by` forbid). `grep -n created_by phase-21.md` finds the attribute in the Cedar entity (phase-21.md:1113) and no rule that uses it. phase-04.md:1083 (the self-approval forbid uses the engine's `excluded` set, which holds maker and deciders only). No guard found.
- Severity: HIGH. A four-eyes rule on report sign-off is bypassed by an M who prepared the content, with no race needed. The phase text says "maker is not checker" (phase-21.md:107), and the drafter is the natural maker.
- Proposed 7.x text: "### 7.N Drafter never signs the report". Add `resource.created_by` to the sign exclusion: Cedar forbid `report.sign` when `resource.created_by == principal` (the attribute already exists, phase-21.md:1113), and the engine's `excluded` set for the sign task includes the drafter. Add DDL CHECK `signed_by IS NULL OR created_by IS NULL OR signed_by <> created_by` on `rpt.reports`, next to the existing submitter CHECK.

### ADV-1-03 Freeze released while a re-listing is activated   (verdict: DOUBT; phase 20, phase 14)
- Rule: "A freeze is released only when delisted" (phase-20.md:682). The gate is re-run in the effect (phase-20.md:702).
- Attack:
  1. Version v2 omits entry P, so the freeze is delisted (phase-20.md:682).
  2. M1 proposes the release and M2 approves it (phase-20.md:689, 3.5 check_before_decision).
  3. The list office activates v3, which lists P again. Activation runs in one transaction and writes `ref.lists.active_version_id` (phase-14.md:957).
  4. If the release reads `v_active` (v2) before v3 commits, the release commits. In serial order the release comes first, so the release is valid when it commits. The freeze is then released while v3 lists P.
  5. Activation queues `screening_lookback` (phase-14.md:957). A new alert for P is raised, and the freeze returns only after that alert is decided true by two people.
- Guard search: no `FOR SHARE`, `FOR UPDATE` or advisory lock on `ref.list_versions` or `ref.lists` in phase-20.md or phase-14.md (grep). Activation writes `ref.lists` (phase-14.md:957), so a shared lock there would order the two. The lookback (phase-14.md:957) is the only later guard.
- Severity: none assigned (DOUBT).
- Doubt: the text does not say whether a freeze may lapse while a re-listing is pending. Reading A: the serial order is valid and the lookback covers the gap. Reading B: release must lock the list row (`FOR SHARE` on `ref.lists` in `release_freeze` step 2) so the freeze never lapses on a list that is changing. The lead chooses. The smaller fix is the lock in B.

### ADV-1-04 Step-up assertion started without a resource is valid for any decision   (verdict: FLAW; phase 01, phase 04)
- Rule: step-up is single-use and "bound to an action class" (phase-01.md:6). The engine consumes the assertion for the task: `resource_ref = "task:<id>"` (phase-04.md section 3.4, act step 5).
- Attack:
  1. Approver P (an M holder) calls `POST /auth/step-up` with `action_class` `decision` and no `resource_ref`. The field is optional (phase-01.md:500).
  2. P gets assertion A, valid for at most five minutes (phase-01.md:250 CHECK).
  3. P uses A on another decision: a screening alert decision (phase-14.md:1024 uses `screening_alert:<id>`), or a deciding task of another request. P only needs to be eligible and not excluded for that task.
  4. The consume predicate matches because the stored `resource_ref` is NULL: `(resource_ref IS NULL OR resource_ref = $6)` (phase-01.md:735-736). The act proceeds, and the audit trail records a `stepup_id` for a different resource than the one P confirmed.
- Guard search: phase-01.md:735-736 (NULL accepted). phase-01.md:238-252 (no CHECK requires `resource_ref` for any class). phase-01.md:500 (start field optional). Eligibility, four-eyes and the single-use rule still apply, so no four-eyes bypass follows.
- Severity: LOW. Step-up stops proving a deliberate act on this task. No separation rule is defeated.
- Proposed 7.x text: "### 7.N Decision step-ups name their resource". `StepUpService::start` refuses a missing `resource_ref` with `E_VALIDATION` when the action class is `decision`, `approval`, `signing` or `filing`. The consume predicate then needs an exact `resource_ref` match for those classes.

### ADV-1-05 A single person dismisses a freeze-list hit   (verdict: DOUBT; phase 14)
- Rule: "On a freeze list the decider must not be a prior decider" applies to `true` only (phase-14.md:1014). The forbid is for outcome `true` only (phase-14.md:1138-1140).
- Attack:
  1. SS1 decides `potential` on a freeze-list alert. The alert is now `potential`.
  2. SS1, or any single SS, decides `incorrect` with reason codes. The transition `potential` to `incorrect` has no four-eyes rule (phase-14.md:1015). The forbid does not apply to `incorrect`.
  3. The alert ends `incorrect`. The freeze trigger runs only for `true` (phase-14.md:1028, step 8).
  4. A true match on a freeze list is closed by one person.
- Guard search: transition table (phase-14.md:1011-1015): no four-eyes row for `incorrect`. Cedar forbid (phase-14.md:1138-1140): `true` only. Bulk decisions exclude freeze lists (phase-14.md line 10, L-36). No guard for single-person dismissal found.
- Verdict reason: the text states four-eyes for `true` only. Reading A: the spec intends that only confirming a freeze needs two people, as written. Reading B: clearing a freeze hit by one person is the false negative the four-eyes rule exists to stop. The lead decides. This is not a new flaw: it is a choice the text leaves open, and it is not in the phase 14 open doubts.

### ADV-1-06 A tightening emergency change stays live with one approver   (verdict: DOUBT; phase 04)
- Rule: "Review ... two further people" (phase-04.md:928-930). "A tightening or neutral change is never reverted by the clock" (phase-04.md:938).
- Attack:
  1. Maker X proposes a tightening setting change with `emergency: true`, reason `CHG_SECURITY` and a resolvable evidence object (phase-04.md:831).
  2. One M holder approves it (phase-04.md:1267). The change is effective at once.
  3. The review is due after the window in phase-04.md:922-923. Nobody completes it.
  4. The clock sets `escalated`, and a CH review task is added (phase-04.md:938). The change stays effective.
  5. The change now has one approval from the MLRO and the maker. The review never completes.
- Guard search: phase-04.md:938 (escalation only, no revert). change_requests CHECK on `review_outcome` (phase-04.md:173) requires only `emergency` and `activated_at`. No guard found that makes the change expire or need a second approval.
- Verdict reason: the four-eyes rule on approval is met (maker and MLRO are two people). The doubt is whether an unreviewed tightening change may stay live. Reading A: a designed emergency exception with a later review. Reading B: an unreviewed tightening needs a deadline or a second approval. This combines with ADV-1-01: an emergency tightening on access keys is live on one approval.

### ADV-1-07 A scheduled CTR batch is confirmed by one officer   (verdict: DOUBT; phase 21)
- Rule: "a manual run is confirmed by a second person" (phase-21.md:958). The DDL check passes when `created_by` is NULL (phase-21.md:67).
- Attack:
  1. The daily job creates the CTR batch with `created_by` NULL. The DDL comment says "NULL = the daily job" (phase-21.md:59).
  2. Any M holder with `ctr.confirm` (phase-21.md:1158-1162) confirms the control totals alone. The CHECK passes because `created_by` is NULL (phase-21.md:67), so no second person is needed.
  3. The batch goes to `packaged` and its files are filed (phase-21.md:964).
- Guard search: phase-21.md:958 (second person only for manual runs), phase-21.md:67 (CHECK skipped for NULL), phase-21.md:1170-1172 (forbid for sign only). No guard for scheduled runs found.
- Verdict reason: the text limits four-eyes to manual runs, so as written this is a designed rule. The lead decides whether daily CTR filings need a second confirmer too. If yes, the fix is a `confirmed_by` check against the batch's job run record and the submitter.

### ADV-1-08 A2 reopens a decided case and unwinds a filed outcome alone   (verdict: DOUBT; phase 18)
- Rule: "a different outcome needs a reopen, which clears `outcome`" (phase-18.md:733, step 2 of section 3.7). Reopen is allowed to A2 (phase-18.md:858).
- Attack:
  1. Case C is decided `str` by M1 and the report is filed to the Office (phase 21).
  2. Within 30 days (`casework.reopen_window_days`), A2 calls `POST /cases/{id}/reopen`. The case goes to `open` and `outcome` is cleared (phase-18.md:861).
  3. The decision row and the filing remain, but the case shows no outcome.
  4. Closing needs an outcome (phase-18.md section 3.12, step 3). A new decision needs M approval, so the case cannot be closed without a second person. The reopen itself needs only A2.
- Guard search: phase-18.md:859-861 (status and version checks only, no check for a filed outcome). No reopen guard in phase 21 (`grep -n reopen phase-21.md` shows metrics only, phase-21.md:1012 and 1435). Closure checks the filing (phase-18.md section 3.12, step 5), reopen does not.
- Verdict reason: reopen by A2 is a designed act (DR-140), and the text does not say what happens to an outcome already filed. The lead decides whether reopen must be refused once a report is filed, or limited to M after filing. Not listed in the phase 18 open doubts.

### ADV-1-09 Downward override approver written as "CHK or M" in one step   (verdict: DOUBT; phase 15)
- Rule: "one step, `approve`, by role CHK or M, not the proposer" (phase-15.md:1388).
- Attack: none on four-eyes. The proposer is always excluded (phase-15.md:1388) and the engine checks the maker (phase-04.md:1058).
- Doubt: a step has one assignee role (phase-04.md section 3.4, delta 6), so "CHK or M" is not one role. Reading A: two parallel approve steps, both needed. Reading B: a pool. Reading C: whichever one is available. The four-eyes result is the same under all three, but the approver set is not. Severity LOW (wording).

### ADV-1-10 Rework text contradicts itself, and pass-scoped exclusion would allow re-approval   (verdict: DOUBT; phase 04, phase 18)
- Rule: "`rework` is allowed ... and increments `pass`" (phase-04.md section 3.4, delta 8). Phase 18 open doubt 16 says "The phase 04 task actions have no rework".
- Attack (if rework is added): approver P approves pass 1 and the maker reworks. In pass 2, P is not excluded, because the exclusion set counts only the same pass (phase-04.md:1080-1081). P may approve pass 2 again. Two people are still required in each pass, because the maker is always excluded and the unique index is per pass (phase-04.md:429-431). So the four-eyes count holds, but the fresh-approval reading is not stated.
- Guard search: phase-04.md:429-431 (per-pass unique indexes), phase-04.md:1080-1081 (per-pass exclusion). phase-18.md (open doubt 16) says rework is absent. No guard for re-approval across passes.
- Verdict reason: the text contradicts itself on whether rework exists. The four-eyes effect is low. The lead decides whether a rework pass needs approval from a new person, and records the choice.

### ADV-1-11 Change-request maker and body cannot be changed after creation   (verdict: GUARDED; phase 04)
- Rule: "the person completing a deciding step is never the maker" (phase-04.md:422). A maker-approved change is not allowed.
- Attack: withdraw and re-submit under a new maker, or edit the proposal after a checker has reviewed it.
- Guard found: `GRANT UPDATE (route, instance_id, status, submitted_at, decided_at, checker_id, stepup_id, activated_at, failure_reason, review_instance_id, review_due_at, review_outcome, reverted_at, updated_at, version) ON admin.change_requests TO akr_app` (phase-04.md:206-208). `maker_id`, `object_key`, `proposed`, `class` and `volume_effect` are not in the list. Withdraw is terminal (phase-04.md:821-822). Submit is maker-only (section 3.2 `submit`, step 1). The open-request unique index on `object_key` (phase-04.md section 3.2 `create`, step 5) allows one open request per object, so a second maker cannot open a parallel copy.

### ADV-1-12 One person decides two parallel branches or a second pass   (verdict: GUARDED; phase 04)
- Rule: "Nobody decides twice in one pass of one instance, in person or through a delegate (DR-57)" (phase-04.md:428).
- Attack: a person eligible for both the MLRO and the CHK branch of `cr_material` decides both. Or a delegate acts for an approver who already decided.
- Guard found: `uq_tasks_decision_person` on `(instance_id, pass, completed_by)` and `uq_tasks_decision_decider` on `(instance_id, pass, decider)` (phase-04.md:429-431). `decider` is `COALESCE(acting_for, completed_by)` (phase-04.md:399). The exclusion of a person who already decided in the same pass (phase-04.md:1080-1081). The instance row lock I1 serialises concurrent approvals (phase-04.md section 3.4). A concurrent second approval is refused by the unique index.

### ADV-1-13 Delegation cannot make the maker or an earlier decider approve   (verdict: GUARDED; phase 04, phase 03)
- Rule: "never acts for the maker by delegation" (phase-04.md:422-425).
- Attack: the maker holds the approver role through a delegation and acts as the delegator. Or an approver delegates to the maker.
- Guard found: CHECK `acting_for IS NULL OR maker_id IS NULL OR acting_for <> maker_id` (phase-04.md:425). Eligibility sets `excluded` when the principal or `acting_for` is the maker (phase-04.md:1080). The decider unique index counts the delegator (phase-04.md:431). Delegation creation runs `SodRules::check_roles` (phase-03.md:940). An exclusive role cannot sit with another (phase-03.md:637). The phase 18 case route refuses any act under a delegation (phase-18.md section 3.8 step d, item iv).

### ADV-1-14 One person holds two roles and both sides of a change   (verdict: GUARDED; phase 03, phase 04)
- Rule: "A role-level SoD pair ... is not added (DR-54) ... separation on change requests is per-request" (phase-04.md:1094-1098).
- Attack: P holds ADM and SEC. P proposes a grant as ADM (route `cr_access`, checker SEC), then approves it as SEC. Or P proposes as SEC (route `cr_access_adm`, checker ADM) and approves as ADM.
- Guard found: CHECK `checker_id IS NULL OR checker_id <> maker_id` compares person ids, not roles (phase-04.md:161). The engine's `excluded` set holds the principal whatever the role (phase-04.md:1080). Phase 03 refuses an exclusive role with any other role (phase-03.md:637 and 735-737).
- Note: holding both roles is allowed by the text. Four-eyes still needs a second person in each direction, so the attack needs a colleague.

### ADV-1-15 One person sets a freeze-list hit to true   (verdict: GUARDED; phase 14)
- Rule: "On a freeze list refused (422 `four_eyes_required`): a `potential` decision comes first" (phase-14.md:1013). "The decider must not be a prior decider" (phase-14.md:1014).
- Attack: SS1 decides `potential` then `true` on the same alert. Or SS1 decides `true` on an open alert directly. Or two SS decide `true` at once.
- Guard found: Cedar forbid `screening.freeze_true_second_person` on `resource.excluded` for outcome `true` (phase-14.md:1138-1140). CHECK `NOT (decider_id = ANY (prior_decider_ids))` (phase-14.md:319). CHECK `outcome <> 'true' OR list_kind <> 'freeze' OR cardinality(prior_decider_ids) >= 1` (phase-14.md:321). The transition refusal for open to true on a freeze list (phase-14.md:1013). The decide step takes `get_for_update` and checks `If-Match` (phase-14.md section 3.6 decide, step 2). Two concurrent decisions: one wins, the other gets 409.
- Note: the CHECK counts prior deciders and does not require that the prior decision was `potential`. The transition table forces `potential` first, so the gap is closed by the table, not by the CHECK. Guarded as written.

### ADV-1-16 A pair-memory rule is approved by its proposer   (verdict: GUARDED; phase 14)
- Rule: "the approver is not the proposer (CHECK and Cedar `screening.no_self_approve_pair`)" (phase-14.md:1038).
- Attack: SS1 proposes a remembered pair from an incorrect alert and approves it, suppressing future matches.
- Guard found: Cedar forbid `screening.pair.approve` when `resource.proposed_by_me` (phase-14.md:1135-1136). The CHECK named in phase-14.md:1038. Approval is under the `decision` step-up class (phase-14.md:837).

### ADV-1-17 Case decision by the proposer, a delegate or an M-proposer   (verdict: GUARDED; phase 18)
- Rule: "four-eyes, second layer under Cedar and the workflow (M-19)" (phase-18.md:164). "the proposer (instance maker) and earlier deciders of the pass are excluded" (phase-18.md section 3.8, step b).
- Attack: A2 proposes and then approves its own proposal. Or an M holder proposes and decides. Or a delegate of the proposer approves.
- Guard found: CHECK `decider_id <> proposer_id` (phase-18.md:164). Engine exclusion of the maker (phase-18.md section 3.8, step b). Delegated acts refused (phase-18.md section 3.8, step d item iv; open doubt 17). Cedar: an M holder decides and does not propose (phase-18.md:730). One running proposal per case under the case lock (phase-18.md section 3.7, step 2).
- Note: on a reopened case, a decider of the earlier decision is not excluded from a later one. That is covered by ADV-1-08, not a new flaw here.

### ADV-1-18 RFI release by the drafter or the submitter   (verdict: GUARDED; phase 19)
- Rule: "released_by <> proposer_id" (phase-19.md:131). "the proposer is excluded (4E: 403 `E_SELF_APPROVAL`)" (phase-19.md:1104, step 1).
- Attack: the drafter D (`proposer_id`) releases an RFI that the submitter S submitted. Or S releases it.
- Guard found: CHECK `released_by IS NULL OR released_by <> proposer_id` (phase-19.md:131). The engine maker is set to the submitter at submit (phase-19.md:1097, `maker: actor`), so S is excluded by the engine. D is excluded by the CHECK. Both are excluded in every combination.

### ADV-1-19 Other sign-off and approval checks in phases 14, 20 and 21   (verdict: GUARDED; phases 14, 20, 21)
- Periodic pack: CHECK `signed_by IS NULL OR generated_by IS NULL OR signed_by <> generated_by` (phase-21.md:363), and the 403 `E_SELF_APPROVAL` on the sign route (phase-21.md:1000, step 3).
- Manual list activation: CHECK `origin = 'feed' OR activated_by IS NULL OR requested_by IS NULL OR activated_by <> requested_by` (phase-14.md:127). Feed versions activate with no person, by design (phase-14.md:523). The auto-activation default is an open doubt (phase-14.md:1437).
- Attestation: `check_before_decision` refuses an approver who is the `completed_by` (phase-20.md:619), and the Cedar forbid `ops.task.confirm_not_completer` (phase-20.md:868).
- Freeze release: the instance maker is the proposer (phase-20.md:689), the approver must be role M (phase-20.md:692), and `check_before_decision` re-checks the freeze state and delegation (phase-20.md section 3.5).

## Summary
- Counts by verdict: FLAW 3; DOUBT 7; GUARDED 9. Total 19 candidates.
- FLAW items by severity: HIGH 2 (ADV-1-01 emergency route takes access settings off the SEC route; ADV-1-02 report drafter may sign). LOW 1 (ADV-1-04 step-up not bound to a resource).
- DOUBT items, one line each:
  - ADV-1-03: freeze release and a concurrent re-listing; does a freeze need a list lock or may it lapse?
  - ADV-1-05: one person may dismiss a freeze-list hit (`incorrect`); the text requires four-eyes only for `true`.
  - ADV-1-06: an unreviewed tightening emergency change stays live on one approval.
  - ADV-1-07: a scheduled CTR batch is confirmed by one M; the text covers manual runs only.
  - ADV-1-08: reopen by A2 clears an outcome that may already be filed; no guard.
  - ADV-1-09: downward override approver is "CHK or M" in one step; wording only.
  - ADV-1-10: rework is both allowed and absent in the text; a rework pass lets a pass-1 approver approve again.
