#!/usr/bin/env python3
"""Apply adversarial pass run 4 (clocks and timers on closed, merged or reopened items) after the lead's checks.

Source: tools/agent/adv-run4.md (Haiku draft); decisions: tools/agent/adv-run4-verdict.md.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'

HDR = ('Adversarial pass (2026-10-10), run 4 (clocks and timers on closed, merged or reopened items), from '
       'tools/agent/adv-run4.md; lead-verified (tools/agent/adv-run4-verdict.md).')

BLOCKS = {
    '00': [
        """### 7.N Cancel touches pending timers only (ADV-4-10, MEDIUM; DR-410)
`TimerService::cancel` and `cancel_for_owner` run `UPDATE admin.timers SET cancelled_at = now() WHERE <id, or owner_kind,
owner_id and kind> AND fired_at IS NULL AND cancelled_at IS NULL` and return the number of rows changed. A timer that
already fired or was cancelled matches nothing, so the call succeeds with 0; without the predicate the CHECK `fired_at IS
NULL OR cancelled_at IS NULL` would fail the owner's transaction (a freeze release, a filing, a work item completion).
Test: a fired `freeze_due` timer, then `cancel_for_owner` on the freeze: 0 rows, and the release commits.""",
    ],
    '15': [
        """### 7.N An absorbed event review keeps the earlier deadline and its clock (ADV-4-02 MEDIUM, ADV-4-08 LOW; DR-410)
Section 3.4 rule 5 and the trigger actions: when an open event review absorbs a new event, `due_at` becomes the earlier
of the current `due_at` and the new event's due date (an absorb never extends a deadline). When it moves earlier, the
same transaction runs `TimerService::cancel_for_owner(tx, 'kyc_review', review_id, 'kyc_review_clock')` and schedules
`kyc_review_clock` at the new `due_at`. The handler `timer_kyc_review_clock` re-arms itself at `due_at` when `due_at` is
still later than now, and writes `escalated_at` and the audit `kyc_review.overdue` only when `escalated_at IS NULL`, so
an at-least-once re-run writes once. Test: an absorb that moves `due_at` from day 10 to day 3 escalates on day 3, once.""",
    ],
    '18': [
        """### 7.N Grant expiry on a closed case (ADV-4-01, MEDIUM; DR-410)
Handler `timer_case_acl_expiry` (section 4.1): after the lock, a case with status `closed` gets the revocation
(`revoked_at = now()`, `revoked_by` NULL) and the audit `case.acl_expired` only: no chain event and no case `version`
bump, so the seal stays the hash of the chain at closure (DR-139). An open case is handled as written. Test: a grant
expiring the day after closure is revoked and the case's `last_seq` equals the seal's.""",
        """### 7.N Merges and the work of this phase (ADV-4-06 HIGH, ADV-4-08 LOW; DR-410)
From this phase on, the merge validation of phase 17 section 3.6 also refuses (409 `E_NOT_ALLOWED_IN_STATE`) a source or
target that is `in_review` (sub-code `proposal_running`) or has an `outcome` (sub-code `decided`), and a source with an
open work item (`WorkItemRepository::open_count(source) > 0`, sub-code `work_items_open`, the test of the close in
section 3.12). A person completes or waives the items first. The timer handlers of this phase (`timer_work_item_due`,
`timer_case_acl_expiry`) do nothing on a `merged` case, and `timer_work_item_due` writes `case.work_item_overdue` only
when no such event exists for the item (the phase 17 `alert_touch` pattern), so a re-run writes once. Test: a merge of a
source with an open work item is 409 `work_items_open`.""",
    ],
    '19': [
        """### 7.N RFIs of a closed or merged case (ADV-4-03, ADV-4-04, ADV-4-06; MEDIUM; DR-410)
1. `RfiService::record_answers` step 1: after the locks, a case with `sealed_at` set refuses with 409 `E_CASE_SEALED`
   (phase 18 I7: a closed case is read-only). No claim is written.
2. The handler `timer_rfi_clock` re-reads the case as well as the RFI: a `closed` or `merged` case writes no
   `rfi.reminder` and no `rfi.overdue`.
3. From this phase on, the merge validation of phase 17 section 3.6 also refuses a source with a `released` RFI (409
   `E_NOT_ALLOWED_IN_STATE`, sub-code `rfi_open`).
The chain write of a closed case is already suppressed by the rule at the head of section 3. Test: release an RFI,
close the case: its 50 percent and deadline stages send nothing, and an answer is 409 `E_CASE_SEALED`.""",
    ],
    '20': [
        """### 7.N The Committee report deadline watches the letter (ADV-4-09 HIGH, ADV-4-05 LOW; DR-410)
`committee_report_at` means the letter was requested; it starts the second-report clock. `timer_committee_report_due`
(section 3.4) becomes: state `released`: no-op. `committee_report_at` NULL: as written (`report_missed`, then
`request_committee_report(reason = deadline)`). `committee_report_at` set and, from phase 21 on, the letter
`committee_report_letter_id` not in state `delivered`: `IncidentService::raise(report_missed, freeze)` and audit
`freeze.report_undelivered {freeze_id, letter_id}`. A delivered letter: no-op. Phase 21 Open doubts 4 is answered by
this rule. Test: a letter requested at the first level and not delivered by `committee_report_due_at` raises one
`report_missed` incident.""",
    ],
    '21': [
        """### 7.N Freeze letters and the freeze timers (Open doubts 4; ADV-4-09; DR-410)
Phase 20's `committee_report_at` means requested, and the phase 20 deadline handler raises `report_missed` when the
letter is not `delivered` by `committee_report_due_at` (phase 20 section 7+). `FreezeService::letter_delivered` (section
4.5 item 9) records the delivery on the freeze. Open doubts 4 is closed.""",
    ],
    '23': [
        """### 7.N A case hold follows a merge (ADV-4-07, HIGH; DR-410)
A merge moves the alerts of the source case to the target (phase 17 section 3.6), so a hold read from the source alone
would lose them at the next run. The matching of a case hold (section 3.8) therefore follows `merged_into` from the held
case to its end and holds, at the run, the parties of `casework.case_parties` and the alerts of `casework.case_alerts` of
every case on that path. A hold may cover more than the original file; it never covers less. Test: hold case S, merge S
into T: the next run still holds S's alerts in T.""",
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
    assert 'Adversarial pass (2026-10-10), run 4' not in s[st:nx if nx != -1 else len(s)], pp
    out[p] = add_section7(s, HDR, blocks)

r = PLAN / 'research.md'
p, rs = load(r)
anchor = [l for l in rs.split('\n') if l.startswith('| DR-409 |')]
assert len(anchor) == 1
DR = ("| DR-410 | 2026-10-10 | Adversarial run 4 (clocks): timer cancel touches pending timers only (phase 00); an "
      "absorbed event review keeps the earlier deadline and re-arms its clock, and the overdue handler writes once "
      "(phase 15); grant expiry on a closed case writes audit only, merges refuse a decided, in-review or open-work "
      "source (phase 18) or one with a released RFI (phase 19); RFI answers and clocks stop on a closed case (phase "
      "19); the Committee report deadline raises report_missed until the letter is delivered (phases 20, 21); a case "
      "hold follows merged_into (phase 23) | Each closes a path where a clock or deadline was lost, fired on an item "
      "nobody owns, or wrote past a seal; the watch review was already guarded (tools/agent/adv-run4-verdict.md) |")
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + DR + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
