p='docs/plan/phase-22.md'; s=open(p).read()
R=[]
def r(a,b,n=1): R.append((a,b,n))
# 1 branch scope
r("`branch_of_party_account(conn, party_id, branch_id) -> bool` (`canon.account` with `org_branch_id = $2` and a role on `party_id` through `canon.account_party_role`).",
  "`branch_of_party_account(conn, party_id, branch_id) -> bool` (`canon.account a JOIN canon.branch b ON b.id = a.branch_id` with `b.org_branch_id = $2` and a role on `party_id` through `canon.account_party_role`; phase 10: `canon.account` has `branch_id`, and `canon.branch.org_branch_id` maps it to `admin.branches`); `recipient_in_branch(rfi.recipient_ref, branch_id)` (`canon.staff s JOIN canon.branch b ON b.id = s.branch_id WHERE s.id = rfi.recipient_ref AND b.org_branch_id = $2`: the RFI's addressed liaison works at the caller's branch).")
r("AND `casework.effective_tier(rfi.party_id) <> 'suspicion'` AND `branch_of_party_account(rfi.party_id, branch.id)`. The predicate",
  "AND `casework.effective_tier(rfi.party_id) <> 'suspicion'` AND `recipient_in_branch(rfi.recipient_ref, branch.id)` (the branch of the addressed liaison, phase 19 section 3.1 step 4 stores that liaison's `canon.staff` row in `recipient_ref`; never the branches where the party holds accounts, which would show an RFI addressed to one branch to every branch of the customer: M-43, lead review). The predicate")
r("a branch item's branch is the branch of the party's account at the bank (`canon.account.org_branch_id`), which phase 19 defines as \"the branch of the subject's account\" (section 3.2 step 4).",
  "a branch item's branch is the branch of the liaison it is addressed to (`rfi.recipient_ref`, a `canon.staff` row, through `canon.staff.branch_id` and `canon.branch.org_branch_id`), which phase 19 chose as a liaison of the subject's account branch (its section 3.1 step 4); a party with accounts at two branches does not make the item visible at the second.")
r("This phase reads neither: the branch is the party's account branch and the liaison is `admin.branches` (DR-205). Gap pass 1 decides whether phase 19 stores the `admin.branches.id` in `recipient_ref` and changes step 4 to say so.",
  "This phase matches the item to the caller's branch through the addressed staff row (`canon.staff.branch_id` to `canon.branch.org_branch_id`, DR-205, lead review); the caller is identified through `admin.branches`. Until phase 19's liaison role code exists, the desk picks the staff row by hand; gap pass 1 decides whether phase 19 resolves the recipient from `admin.branches.liaison_user_id` and `admin.users.staff_id` (phase 15) instead.")
r("`canon.account` (`org_branch_id`), `canon.account_party_role`,","`canon.account` and `canon.branch` (`org_branch_id`), `canon.staff`, `canon.account_party_role`,")
# 3 document answers
r("`documents_on_file` to `corroboration` (documented requires an evidence object)",
  "`documents_on_file` to a `document` answer whose sealed value lists the staged evidence ids (the claim is written `corroboration = none`; the desk raises it, section 3.2)")
r("A `document` answer needs at least one evidence id; `corroboration = documented` in the claim follows from it.",
  "A `document` answer needs at least one evidence id (at most 5); the ids go into the answer's sealed value, never into `evidence_object_id`. Every branch claim is written with `corroboration = 'none'` and `evidence_object_id` NULL: phase 19's CHECKs tie a corroboration to `corroborated_by` and `documented` to one evidence object, and a claim corroborated by the person who gave it would pass the M-45 closure gate on the giver's word. The desk raises the corroboration through phase 19's `corroborates` path, naming one of the staged `rfi_input` objects (section 4.5 item 2; lead review).")
r("6. Call the phase 19 port `RfiService::record_answers(tx, ctx, rfi_id, answers)` with",
  "6. Call the phase 19 inner method `RfiService::record_answers_in_tx(tx, actor, rfi_id, answers, source_kind)` (new in phase 19 by section 4.5 item 2: the body of `record_answers` without its `rfi.answer` Cedar check, which a liaison outside the case team would fail; this route authorises with `branch.item_answer`) with")
r("2. **Phase 19, `RfiService::record_answers` step 4.** Accept, for `corroboration = documented`, an evidence object whose owner kind is `rfi_input` and whose owner id is the RFI itself (the branch's uploads), as well as an object of the case (owner kind `case`). The branch path has no case access, so this is the only way a branch document can corroborate a claim (DR-201).",
  "2. **Phase 19, `RfiService::record_answers`.** (a) The body is split into `record_answers_in_tx(tx, actor, rfi_id, answers, source_kind)`, called by the desk route after its `rfi.answer` check and by the branch route after `branch.item_answer`; the branch path always writes `corroboration = 'none'`. (b) Step 3's `corroborates` path (the desk raising a claim to `documented`) accepts an evidence object whose owner kind is `rfi_input` and whose owner id is the RFI itself (the branch's uploads), as well as an object of the case (owner kind `case`) (DR-201, lead review).")
# 4 observation party nullable
r("    party_id              UUID NOT NULL,                       -- canonical party with an account at the branch (no foreign key)",
  "    party_id              UUID,                                -- canonical party with an account at the branch (no foreign key); NULL: no customer named (a walk-in, a third party; described in notes_enc)")
r("       OR NEW.party_id <> OLD.party_id OR NEW.indicator_groups <> OLD.indicator_groups",
  "       OR NEW.party_id IS DISTINCT FROM OLD.party_id OR NEW.indicator_groups <> OLD.indicator_groups")
r("`ObservationCreate { party_id: Uuid (the branch is the caller's, never sent),","`ObservationCreate { party_id: Option<Uuid> (NULL when the person is not a customer of the branch; the branch is the caller's, never sent),")
r("3. Party: `branch_of_party_account(party_id, branch.id)`; false:","3. Party (when given; a walk-in or a third party is recorded without one, described in the notes): `branch_of_party_account(party_id, branch.id)`; false:")
r("1. Read the observation (no lock). If `hybrid_checked_at IS NOT NULL`: no-op.","1. Read the observation (no lock). If `hybrid_checked_at IS NOT NULL`: no-op. If `party_id IS NULL`: set `hybrid_checked_at = now()` and stop (no party, no pattern).")
# 5 confidential counts
r("items released in the period by neutral status; observations and confidential reports recorded in the period (counts only).",
  "items released in the period by neutral status and observations recorded in the period (counts only; confidential reports are never counted on a branch surface, because the alternate would learn that the liaison reported, M-68; M sees them on R5 and in the register).")
# 6 floor
r("('branch.liaison_due_day', '10', 'integer', '{\"min\": 1, \"basis\": \"CBI ch. 11 gives the 10th of the month; the bank confirms\"}'::jsonb, TRUE,",
  "('branch.liaison_due_day', '10', 'integer', '{\"max\": 10, \"basis\": \"CBI ch. 11: by the 10th of the month; a bank may only set an earlier day\"}'::jsonb, TRUE,")
r("`branch.liaison_due_day` (10; governed; floor 1)","`branch.liaison_due_day` (10; governed; floor max 10)")
# 7 names
for a in ["BranchService::items","BranchService::item(","BranchService::item`","BranchService::answer","BranchService::stage_evidence","BranchService::neutral_counts"]:
    pass
r("`case.assign` gives `task.waiting` to the assignee;","`case.assigned` gives `task.waiting` to the assignee;")
r("### 3.9.1 The e-mail relay","### 3.8.3 The e-mail relay")
r("(section 3.9.1)","(section 3.8.3)",2)
r("the only implementation is `SmtpRelaySender` (section 3.9).","the only implementation is `SmtpRelaySender` (section 3.8.3).")
r("entity Notice = { owner: User };\n\nAction declarations","entity Notice = { owner: User };\n```\n\nAction declarations")
# 2 outbox multi-handler
r("Topics not consumed here: `internal_report.recorded` and `authority_request.recorded` (phase 15 reads them), `authority_request.overdue` is consumed (table above); `hibernation.*`, `network.snapshot_requested` and `graph.link` belong to phases 19 and 15; `case.decision` is read by phase 15 for TR-15 as well (each consumer keeps its own cursor).",
  "Topics not consumed here: `internal_report.recorded` and `authority_request.recorded` (phase 15 reads them), `authority_request.overdue` is consumed (table above); `hibernation.*`, `network.snapshot_requested` and `graph.link` belong to phases 19 and 15. **Several handlers per topic (lead review).** Phase 00's outbox row has one `dispatched_at` and its registry routes a topic to one `TopicHandler`, while `case.event` is already handled by phase 17 (counts) and `case.decision` by phase 15 (TR-15). Phase 00's registry therefore maps a topic to an ordered list of handlers (section 4.5 item 10): they run in registration order in the dispatch transaction, the row is marked dispatched only when every handler succeeded, and a failure retries the row for all of them, which is safe because every handler is idempotent (this dispatcher by `dedupe_key`, phase 17 by recomputing counts, phase 15 by its coalesce key). The dispatcher is appended after the earlier handlers of each topic.")
r("10. **Phase 00.** The outbox topic `notify.request` (phase 00 reserves the name `notify (22)`; its CHECK requires a dotted name) gets its handler; the `OutboxDispatcher` routes the other topics of section 3.8 to the same dispatcher.",
  "10. **Phase 00.** The outbox topic `notify.request` (phase 00 reserves the name `notify (22)`; its CHECK requires a dotted name) gets its handler. The `TopicHandler` registry becomes a list per topic (section 3.8, lead review): `register(topic, handler)` appends; `OutboxDispatcher` runs the list in order in one transaction and marks the row dispatched only when all succeed. The dispatcher is appended to `case.event` (after phase 17), `case.decision` (after phase 15) and every other topic of section 3.8.")
for a,b,n in R:
    c=s.count(a); assert c==n,(c,n,a[:70]); s=s.replace(a,b)
import re
# rename phase 22's BranchService (phase 03's BranchService::update and the dependency mention stay)
s2=re.sub(r'BranchService::(?!update)','BranchWindowService::',s)
s2=s2.replace('Arc<BranchService>','Arc<BranchWindowService>')
s2=s2.replace('`BranchService`, `ObservationService`','`BranchWindowService`, `ObservationService`')
n=s2.count('BranchWindowService'); assert n>=8,n
s=s2
# lead corrections paragraph before Open doubts
lc=("**Lead corrections (review 2026-10-09).** A branch item is listed to the branch of the liaison it is addressed to "
"(`rfi.recipient_ref`, a `canon.staff` row, through `canon.branch.org_branch_id`), not to every branch where the party "
"holds an account; the draft's `canon.account.org_branch_id` does not exist (`canon.account.branch_id` joins `canon.branch`). "
"Phase 00's outbox routes a topic to one handler and `case.event` and `case.decision` already have one (phases 17 and 15), so the "
"registry becomes a list of handlers per topic (section 3.8, section 4.5 item 10). Branch claims are written with "
"`corroboration = 'none'` and no `evidence_object_id` (phase 19's CHECKs; a claim corroborated by its own maker would pass the "
"M-45 closure gate); the document ids sit in the sealed value and the desk raises the corroboration; the branch calls "
"`record_answers_in_tx`, the phase 19 body without its `rfi.answer` check. An observation may name no customer (`party_id` "
"nullable): S-22's counter observations are often about walk-ins and third parties. Confidential reports are not counted on "
"any branch surface (the alternate would learn that the liaison reported). The due-day floor is `{\"max\": 10}`. This phase's "
"service is `BranchWindowService` (phase 03 owns `BranchService`); the case event is `case.assigned`.\n\n")
a="## Open doubts (for gap pass 1 and the bank)"
assert s.count(a)==1; s=s.replace(a,lc+a)
open(p,'w').write(s); print('ok',len(R))
