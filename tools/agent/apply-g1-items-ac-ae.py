#!/usr/bin/env python3
"""Apply the fixes of gap items (ac), (ad) and (ae) (tools/agent/gap-items.md), found in gap pass 1 runs 7b and 8c.

Lead work, no Haiku draft. (ac): the trigger evaluator of phase 15 and the producers of the trigger topics no phase
wrote (DR-405). (ad): the PEP list source of phase 15 and the meaning of the `regime` pack (DR-406). (ae): the subject
kinds of the phase 14 look-back (DR-406).
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'
DOCS = ROOT / 'docs'

HDR = ('Gap pass 1 (2026-10-10), gap items (ac), (ad) and (ae) of tools/agent/gap-items.md (found in runs 7b and 8c); '
       'lead work, no Haiku draft.')

P15 = [
    """### 7.N Trigger evaluation and the account events (K-11, DSL.md C.5; gap item (ac); DR-405)
Section 3.3 lists the trigger topics, but no step said how a trigger's `on`, `when` and `action` are applied, and five
topics had no producer. This block is that path. It replaces the last row of the section 3.3 table ("not wired here").
1. **Job.** `rating_compute` gains the reason `trigger` and two payload fields: `event` (a DSL.md C.5 `on` name) and
   `event_ref` (the id the producer gave: an alert, case, RFI, account or report id). A trigger job uses the coalesce key
   `rating_trigger:<party_id>:<event>:<event_ref>`, so two events for one party are never merged into one job; the
   handler still takes the party's advisory lock (3.2 step 1). `event_ref` is never stored on a rating or a review.
2. **Evaluation** (`RatingService::rescore` with reason `trigger`): read the triggers of the live model whose `on`
   equals `event`, in id order. Evaluate each `when` (the shared Expr, CEL, as the `expr` anchors of 3.2 step 4) over
   `event` (the producer's payload fields) and the party's facts of this computation. No trigger holds: an ordinary
   re-score, and the job ends.
3. **Action** of the first trigger that holds (later ones that hold are listed in the audit payload by id):
   - `rescore`: the re-score of section 3.2, with `trigger_code` set (3.2 step 16).
   - `rescore_and_review`, `refresh_task`, `reclassify`, `pep_workflow`: the re-score, then an event review (3.4 rule 5:
     an open event review absorbs it) with `due_basis = trigger_sla` and `due_at = BusinessClock::add(now,
     review_within)` on the bank calendar. `pep_workflow` uses the PEP checklist of the level's controls row.
   - `hold_and_escalate`, `block_and_escalate`: the same review with `escalated_at` set at once. AKR-TMS never writes
     to the CBS, so the hold or block is the bank's act, recorded in the review's outcome.
   Audit `rating.trigger_applied {party_id, trigger_code, action, review_id?}` (ids and codes only). The stored
   `trigger_code` is the `TR-nn` id, or `TRG_REVIEW_REQUESTED` for the suspicion-linked topics of 3.6 rule 3 and for a
   trigger whose `when` reads `party.on_exit_register`.
4. **New fact.** The facts of a computation gain `party.on_exit_register` (TRUE when the party has a `kyc.exit_register`
   row with status `recorded`). It is a suspicion-tier fact (the register is M-only, 3.6): it is never a factor, and it
   is read only by a trigger's `when`.
5. **Events this phase derives** (`rating_touch_fanout`, which already reads the batch's changed `account` rows; no
   outbox topic):
   - `account.reactivated` (TR-14): an account row of the batch with status `active` whose previous version (the row
     whose `recorded_to` equals its `recorded_from`) has status `dormant`.
   - `account.opened` (TR-24): an account whose first version is in the batch and whose `opened_on` is on or after the
     business date of the previous mapped batch of the same source table. The first batch of a source table derives
     none (the initial load is not a wave of new accounts).
   Each enqueues `rating_compute {reason: "trigger", event, event_ref: account id}` for the account's customer party.
   TR-24's `when` reads `party.on_exit_register`, so its reviews carry the neutral code.
6. **Producers in later phases** (the DR-375 pattern: the later phase writes the topic and registers this phase's
   handler, which enqueues the job of step 1): `screening.hit` (7.3), `profile.breach` (7.6), `outreach.failed`
   (TR-20, phase 19 section 7+), `visit.recorded` (TR-19, phase 22 section 7+). The other topics of the row keep the
   producers named in section 3.3 and in phases 17 to 19 and 26.
7. **Pending triggers.** `media.recorded` (TR-09; adverse media needs a source, ROADMAP Q12), `txn.new_corridor`
   (TR-12; the corridor feature is pending, as factor F2 is in 3.2) and `staff_link.recorded` (TR-25; no HR link is on
   a P0/P1 line) have no producer in this release. They stay in the shipped library and never fire. The model page
   lists them as pending triggers, the way it lists pending factors (DR-104).
8. Section 4.1: the `rating_compute` row gains the reason `trigger`, the fields `event` and `event_ref`, and the
   trigger coalesce key. Section 3.11 gains `rating.trigger_applied`.""",
    """### 7.N The PEP list source in this release (gap item (ad); DR-406)
Source 1 of section 3.8 reads screening alerts with `list_kind = 'pep'`. In this release no list has kind `pep`: the
list codes are closed to six (DR-100, `pep` is a kind only), and a PEP data list needs a licensed source (ROADMAP Q12).
Source 1 therefore finds nothing, and `pep_list_sweep` runs only its second step (the CBS class) until a later change
adds a list code of kind `pep`. Candidates in this release come from the CBS class, manual entry and declarations. The
known-answer case of section 6 that expects a `source: pep_list` candidate is run only when such a list exists. Open
doubts 13 (latency of the PEP list path) is moot until then.""",
]

P14 = [
    """### 7.N Subjects of the look-back (VC-06; gap item (ae); DR-406)
Section 3.5 run kind 2 (`lookback`) builds its in-memory index from "all current subjects". The subjects are:
1. every current party (`canon.party`, types `person` and `entity`), with the names of section 3.5;
2. every counterparty (`canon.counterparty`) whose `party_id` is null, from its `names` (a counterparty linked to a
   party is screened as that party).
Payment-message parties are not in the look-back: each is screened once by the payment run (run kind 4) when first
seen. A later list change reaches the sender or beneficiary through the counterparty row, when the mapping links one.
The daily base run (run kind 1) is unchanged (parties only).""",
]

P19 = [
    """### 7.N The outreach.failed topic (TR-20; DR-405)
`timer_rfi_clock` at stage `due` on a `released` RFI of kind `customer_outreach` that still has open questions also
writes outbox `outreach.failed {party_id, rfi_id, prior_failures}` in the same transaction as `rfi.overdue`.
`prior_failures` is the number of earlier `customer_outreach` RFIs of the party whose `deadline_at` passed with
`answered_at` null, so TR-20's `when` can escalate on a second failure. This phase registers phase 15's trigger
handler for the topic (phase 15 section 7+). The topic is suspicion-linked (phase 15 section 3.6 rule 3), so the
review carries `TRG_REVIEW_REQUESTED`. Section 4.2 gains the row.""",
]

P22 = [
    """### 7.N The visit.recorded topic (TR-19; DR-405)
The branch RFI templates of the shipped pack carry the question key `visit_result` (answer format `yes_no_unknown`:
"the visit found the customer as declared"). When `RfiService::record_answers_in_tx` records `no` for `visit_result`
from a branch answer, the same transaction writes outbox `visit.recorded {party_id, rfi_id, result: "negative"}`. A
`yes` or `unknown` writes nothing. This phase registers phase 15's trigger handler for the topic (phase 15 section
7+). The topic is suspicion-linked (phase 15 section 3.6 rule 3), so the review carries `TRG_REVIEW_REQUESTED`, and
the branch view (section 3) shows nothing of it.""",
]

out = {}


def load(p):
    if isinstance(p, str):
        p = PLAN / f'phase-{p}.md'
    return p, out.get(p, p.read_text())


def edit(p, pairs):
    p, s = load(p)
    for a, b in pairs:
        s = rep(s, a, b)
    out[p] = s


for pp, blocks in (('15', P15), ('14', P14), ('19', P19), ('22', P22)):
    p, s = load(pp)
    st = s.index('## 7+. Gap Additions')
    nx = s.find('\n## ', st + 5)
    assert 'gap items (ac)' not in s[st:nx if nx != -1 else len(s)], pp
    out[p] = add_section7(s, HDR, blocks)

# VC-06 is now fully carried by phase 14.
edit('14', [
    ('VC-06 (the look-back on every list change and changed parties against every active list), ', ''),
    ('VC-31 (every closed alert a labelled pair; thresholds change only by change request).',
     'VC-31 (every closed alert a labelled pair; thresholds change only by change request); VC-06 (the look-back of '
     'parties and unlinked counterparties on every list change, and changed parties against every active list, '
     'section 7+).'),
])

# DSL `regime` pack: the slot of the P3 foreign regime packs; PUB, PF and TF are families of the cbi and akr packs.
p = DOCS / 'DSL.md'
edit(p, [('`regime` (PUB, PF-Iran, TF district), `bank`',
          '`regime` (a foreign regime\'s pack, ROADMAP P3: US, UK, EU; none ships before P3; the PUB, PF and TF families '
          'are in the `cbi` and `akr` packs, DR-406), `bank`')])
edit('06', [('-- cbi: implements at least one CBI item (the always-on baseline); akr: extension; islamic and regime packs are\n'
             '    -- optional (phases 26 and later); bank: the bank\'s own, codes 901-999 (phase 16).',
             '-- cbi: implements at least one CBI item (the always-on baseline); akr: extension; islamic: optional (phase\n'
             '    -- 26); regime: a foreign regime\'s pack (ROADMAP P3, no phase here, DR-406); bank: the bank\'s own, codes\n'
             '    -- 901-999 (phase 16).')])

g = ROOT / 'tools' / 'agent' / 'gap-items.md'
edit(g, [('(ac) Found in run 7b', '(ac) DONE 2026-10-10 (tools/agent/apply-g1-items-ac-ae.py, DR-405). Found in run 7b'),
         ('(ad) Found in run 7b', '(ad) DONE 2026-10-10 (tools/agent/apply-g1-items-ac-ae.py, DR-406). Found in run 7b'),
         ('(ae) Found in run 8c', '(ae) DONE 2026-10-10 (tools/agent/apply-g1-items-ac-ae.py, DR-406). Found in run 8c')])

r = PLAN / 'research.md'
p, rs = load(r)
anchor = [l for l in rs.split('\n') if l.startswith('| DR-404 |')]
assert len(anchor) == 1
DR = ("| DR-405 | 2026-10-10 | Triggers (phase 15 section 7+, gap item (ac)): `rating_compute` gains the reason "
      "`trigger` with `event` and `event_ref` and its own coalesce key; the live model's triggers whose `on` matches "
      "are tested by their `when` (CEL) and the first that holds acts (`rescore`, or an event review with due basis "
      "`trigger_sla`, escalated at once for hold and block actions, which the bank carries out); the fact "
      "`party.on_exit_register` is read only by a `when` and gives the neutral code. Phase 15 derives `account.reactivated` "
      "(dormant to active between versions) and `account.opened` (first version, opened on or after the previous "
      "batch's business date, none from a first batch) from the batch it already reads. Phase 19 writes "
      "`outreach.failed` at the due stage of an unanswered customer outreach, with `prior_failures`; phase 22 writes "
      "`visit.recorded` on a `no` to `visit_result`; each registers phase 15's handler. TR-09, TR-12 and TR-25 are "
      "pending triggers | Phase 15 claimed the trigger library (K-11) with no evaluation path and five topics without a "
      "producer; deriving account events from the batch avoids a new topic, and the later phases own the RFI facts "
      "(DR-375 pattern) |")
DR2 = ("| DR-406 | 2026-10-10 | No list has kind `pep` in this release, so phase 15's PEP list source finds nothing until "
       "a later change adds one (ROADMAP Q12); the DSL `regime` pack is the slot of the P3 foreign regime packs (US, UK, "
       "EU), and PUB, PF and TF are families of the `cbi` and `akr` packs; the phase 14 look-back screens current "
       "parties and counterparties not linked to a party, and payment-message parties only through their counterparty "
       "row | Gap items (ad) and (ae): a dead source and two meanings of `regime` were stated as if they worked; VC-06 "
       "asks for counterparties in the delta re-screen (gap pass 1, runs 7b and 8c) |")
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + DR + '\n' + DR2 + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
