#!/usr/bin/env python3
"""Apply adversarial pass run 5 (existence masking leaks) after the lead's checks.

Source: tools/agent/adv-run5.md (Haiku draft); decisions: tools/agent/adv-run5-verdict.md.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'

HDR = ('Adversarial pass (2026-10-10), run 5 (existence masking leaks), from tools/agent/adv-run5.md; lead-verified '
       '(tools/agent/adv-run5-verdict.md).')

BLOCKS = {
    '03': [
        """### 7.N Step-up and purpose answers to admitted callers (ADV-5-01, LOW; DR-411)
Section 3.6 step 4 (and DR-48's last clause) applies to a caller the policy refuses: such a caller gets the masked 404,
never 401 or 422, whatever header is missing. A caller the policy admits (from phase 17 on, the suspicion forbids already
include the analyst network and the purpose code, so a refused caller never reaches the handler) may get 401
`E_STEPUP_REQUIRED` for a missing fresh step-up, and on a restricted record 422 `E_PURPOSE_REQUIRED` (DR-46), as phases
17, 18, 19 and 21 state: those answers confirm nothing the caller may not know. The inference harness probes the refused
caller only; one more probe per masked kind checks that a caller outside the team without step-up gets the 404, not the
401.""",
    ],
    '17': [
        """### 7.N Merge checks visibility before any other answer (ADV-5-10, HIGH; DR-411)
Section 3.6 validation: after the two `FOR UPDATE` locks, step 4 runs first. The source is decided by the authorize
layer on the path; the target, read from the body, is decided by `authorize` from the handler (phase 03 section 3.2) on
Cedar `case.merge`. A target the caller may not see answers the masked 404 (one body, one latency floor), and no state,
seal, family or tier check runs for it. Steps 1 to 3, and the refusals added from phase 18 on (`proposal_running`,
`decided`, `work_items_open`) and phase 19 on (`rfi_open`), run only when both cases are visible. Test: a merge into a
sealed suspicion case of another family, by a caller outside its team, gives the same body and headers as a merge into a
random id.""",
        """### 7.N The bulk dry run says not_found for a hidden id (ADV-5-11, HIGH; DR-411)
Section 3.9 bulk: with `dry_run = true` an id the caller cannot see is reported `allowed: false, reason: "not_found"`, the
answer for a missing id, as the real run already does. `not_visible` leaves the list of bulk reasons, which becomes
`not_found`, `claimed_by_other`, `p0_or_p1_duplicate`, `wip_limit`, `lower_refused`; the last four are given only for
visible ids. Test: per id, the dry-run result for a suspicion alert outside the caller's access equals that of a random
id.""",
        """### 7.N The counts trigger reaches every member of a suspicion row's team (ADV-5-08, LOW; DR-411)
Section 3.11 trigger: for a suspicion row the handler resolves the principals of section 3.10 rule 5 (M, the case
assignees, the holders of a grant on the case's `assignee_pool_id`, and from phase 18 on the active rows of
`casework.case_acl`), from the network `analyst`. The pool members and access-list members who may see the row then get
fresh counts too; a non-member still gets no trigger.""",
    ],
    '18': [
        """### 7.N A link target the caller may not see answers as a missing one (ADV-5-03, HIGH; DR-411)
Section 3.5 Add: after the existence read and before the `link_exists` check, the `EntityMask` rule of steps 1 to 3 runs on
the target's owner party with the caller's principal, network and purpose. A target the caller may not see is refused
exactly as a missing target: 422 `E_VALIDATION` with sub-code `unknown_party`, `unknown_account` or `unknown_txn`, the
same body, released no earlier than `authz.mask_floor_ms` after the request began, with a `deny` access event. No link is
written. An external link has no owner party and is unchanged. Test: adding a suspicion party outside the caller's
access and adding a random id give the same body; the case's link count is unchanged.""",
        """### 7.N Party read order and the search page flag (ADV-5-02, ADV-5-06; LOW; DR-411)
1. Section 3.14 step 1 refuses on rules 1 and 5 of section 3.10 (party rows) with the masked 404. The purpose of rule 4
   (restricted) is answered by step 2 (422 `E_PURPOSE_REQUIRED`, DR-46): A1, A2 and M may know a restricted party
   exists, and the test of section 6 keeps its 422.
2. Section 7.12 step 3: the party predicate is in the SQL `WHERE` of each party query before `LIMIT limit + 1`, as
   section 3.10 "Lists" requires, so `has_more` comes from the filtered query. Test: with `limit` 1, two matches of
   which one is a hidden suspicion party give `has_more: false`, the same as one match.""",
    ],
    '19': [
        """### 7.N Breach alarms follow the visibility of their subject (ADV-5-05, HIGH; DR-411)
`AlarmRepository::list_visible` and the read and acknowledge of a single alarm: a `touch_breach` alarm is visible to M
and to an A2 who can see its alert (phase 17 section 3.10 predicate, generated by `services::visibility::build`); a
`work_item_breach` alarm to M and to an A2 who can see the work item's case (the phase 18 case predicate with
`casework.case_acl`). For any other caller the alarm is absent from the list and from the counts by kind, and its read
and acknowledge answer 404 `E_NOT_FOUND` as a missing alarm. The other kinds keep their named readers. The Cedar permit
`followup.a2.alarm` stays the role gate; the service applies the predicate in SQL. The audience of an alarm notice (phase
22 section 3.8 step 4) is the same set. Test: an A2 outside the pool of a suspicion alert with a breach does not see the
alarm, and `POST /alarms/{id}/acknowledge` gives the body of a random id.""",
    ],
    '22': [
        """### 7.N A released item that moves to suspicion reads as closed (ADV-5-07, HIGH; DR-411)
1. `RfiBranchRepository::released_items`: the clauses `c.tier <> 'suspicion'` and `casework.effective_tier(rfi.party_id)
   <> 'suspicion'` no longer remove an item. When either tier is `suspicion`, the item is listed with the neutral status
   `closed` (the word of a withdrawn request), with no `due_at`: while the RFI is `released` or `answered`, and after phase
   19 closes or withdraws it, for `branch.item_closed_days` from `closed_at` like any closed item. `rfi.tier <> 'suspicion'`
   stays: an RFI created at suspicion tier is never listed. The other conditions (kind, recipient, branch) are unchanged.
2. `BranchStatusMapper` and `neutral_counts` follow step 1, so a count does not fall either. `GET /branch/items/{id}`
   returns the closed view; input and evidence on such an item answer 409 `E_NOT_ALLOWED_IN_STATE` sub-code
   `not_released`, as for a closed item.
3. The dispatcher (section 3.8 step 4) tests a liaison recipient with the open form of the predicate (the suspicion
   clauses kept), so no reminder or overdue notice reaches the liaison for an item shown as closed.
4. Section 3.12, row "An item that moves to suspicion": the item's own liaison sees it become `closed`, which a withdrawn
   request also shows; test e of section 6 expects status `closed` and a 409 on input, and a liaison of another branch
   still gets the 404.""",
        """### 7.N Who reads the internal-report register (Open doubts 27; ADV-5-04; DR-411)
Open doubts 27 is closed. Phase 19's register readers are A2 and M (phase 19 section 3.14, Cedar `followup.a2.report`):
they read the subject party and the body; the reporter's identity is M's only (M-68). A party named by a branch
observation or a confidential report is therefore known to the MLRO desk (A2 and M), never to branch staff or branch
management. The sentence "an M-only fact" in Open doubts 27 is replaced by this rule.""",
    ],
}

out = {}


def load(p):
    if isinstance(p, str):
        p = PLAN / f'phase-{p}.md'
    return p, out.get(p, p.read_text())


for pp, blocks in BLOCKS.items():
    p, s = load(pp)
    st = s.index('## 7+. Gap Additions')
    nx = s.find('\n## ', st + 5)
    assert 'Adversarial pass (2026-10-10), run 5' not in s[st:nx if nx != -1 else len(s)], pp
    out[p] = add_section7(s, HDR, blocks)

r = PLAN / 'research.md'
p, rs = load(r)
anchor = [l for l in rs.split('\n') if l.startswith('| DR-410 |')]
assert len(anchor) == 1
DR = ("| DR-411 | 2026-10-10 | Adversarial run 5 (existence masking): a link target, a merge target and a bulk dry-run "
      "id the caller may not see answer exactly as a missing one (phases 17, 18); breach alarms follow the visibility "
      "of their subject (phase 19); a released branch item whose case or party moves to suspicion reads as closed to its "
      "liaison instead of vanishing (phase 22); DR-48's no-401/422 clause is for refused callers, admitted callers may "
      "get 401 or 422 (phase 03); party read order, search has_more and the suspicion counts trigger made explicit; "
      "the internal-report register readers are A2 and M (phase 22 Open doubts 27) | Each closes a channel where an "
      "answer, a code or a list change differed between a hidden and a missing record; the A2 register and aggregate "
      "pack counts are designed (tools/agent/adv-run5-verdict.md) |")
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + DR + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
