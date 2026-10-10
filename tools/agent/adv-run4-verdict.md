# Adversarial pass run 4: lead verdict (2026-10-10; decisions final, applied by apply-adv-run4.py)

Haiku draft tools/agent/adv-run4.md (clocks and timers on closed, merged or reopened items): 11 candidates; FLAW 9
(HIGH 4, MEDIUM 4, LOW 1), DOUBT 2, GUARDED 17. About 216k tokens.
Lead result: 10 confirmed in 9 fixes (HIGH 3, MEDIUM 4, LOW 2), 1 guarded on recheck (ADV-4-11), 17 guarded accepted.

## Confirmed
- ADV-4-06 HIGH (phases 17, 18, 19). Merge validation (phase 17 section 3.6) checks status, family, tier, Cedar and the
  seal only. A source with open work items, a running proposal, an outcome or a released RFI becomes `merged` while its
  clocks keep running on a case nobody can reach, and their handlers write to its chain. Fix: from phase 18 on, merge
  refuses a source or target that is `in_review` or has an outcome, and a source with an open work item; from phase 19
  on, a source with a released RFI. The phase 18 timer handlers do nothing on a `merged` or `closed` case.
- ADV-4-07 HIGH (phase 23), the doubt resolved (reading A). A case hold holds "the alerts of the case" read at the run;
  a merge moves the alerts to the target, so a held case loses its alerts' hold at the next run. Fix: a case hold on a
  merged case follows `merged_into` to its end and also holds that case's parties and alerts.
- ADV-4-09 HIGH (phases 20, 21). `committee_report_due_at` is listing plus 48 hours and the handler is satisfied by the
  request (`committee_report_at`); an unsigned or undelivered letter raises nothing. Phase 21 Open doubts 4 left the
  meaning open. Fix: `committee_report_at` means requested (it starts the second-report clock); the deadline handler,
  when the letter of `committee_report_letter_id` is not `delivered`, raises `report_missed` and audits
  `freeze.report_undelivered`. Closes phase 21 Open doubts 4.
- ADV-4-02 MEDIUM (phase 15). Rule 5 absorbs a new event into an open event review, and the trigger action computes a
  new `due_at`, but no timer is re-armed; the clock fires on the old instant. Fix: an absorb keeps the earlier `due_at`;
  when it moves earlier, the clock is cancelled and scheduled again; the handler re-arms itself when `due_at` is later
  than now.
- ADV-4-10 MEDIUM (phase 00). The cancel statement is not written, and `CHECK (fired_at IS NULL OR cancelled_at IS
  NULL)` turns a cancel of a fired timer into a failure of the owner's transaction (freeze release, filing, work item
  completion). Haiku said HIGH; the error is loud and the fix is one predicate, so MEDIUM. Fix: cancel touches pending
  timers only and returns the count.
- ADV-4-01 MEDIUM (phase 18). `timer_case_acl_expiry` writes a chain event and bumps the version on a closed, sealed
  case, so the chain runs past the seal. Fix: on a closed case the handler revokes the grant and writes the audit event
  only.
- ADV-4-03 and ADV-4-04 MEDIUM (phase 19). A case may close with a released RFI (the closure gate tests corroboration
  only). Answers then write claims to a closed case (phase 18 I7: read-only), and `timer_rfi_clock` sends
  `rfi.reminder` and `rfi.overdue` for it. The chain write itself is already suppressed by the section 3 preamble. Fix:
  the answer route refuses a sealed case (409 `E_CASE_SEALED`); the RFI clock handler does nothing when the case is
  closed.
- ADV-4-05 LOW (phase 20), the doubt resolved. `timer_committee_report_due` names no state test, unlike the second-report
  handler. Fix: "state not `released`" added, as the general handler rule already implies.
- ADV-4-08 LOW (phases 15, 18). `timer_work_item_due` and `timer_kyc_review_clock` repeat their chain event or audit on
  an at-least-once re-run. Fix: each writes only when its marker is unset (no `case.work_item_overdue` event yet for the
  item; `escalated_at IS NULL`).

## Guarded on recheck
- ADV-4-11 (watch review on a sealed case appends a chain event): phase 19 section 3 preamble: the follow-up records of a
  closed case, "its watch, its hibernation ledger and its QA grant, write their audit events and no case event". The
  review step's chain write is under that rule.

## Guarded (accepted as checked)
The 17 one-line items of the draft: close refuses open work items; alert touch and age handlers test the case; watch
expiry on a closed case; RFI release on a sealed case; second-report and freeze clocks test the state; freeze start
replay; release gate under lock; grant visibility needs `expires_at > now()`; purge re-reads holds under the party lock;
KYC clock on a closed review; breach alarm unique index; report filing target alarm only (DR-178); periodic pack overdue
read-only (DR-178); merge with a sealed case refused; timer sweep SKIP LOCKED.

DR: DR-410 (the run 4 fixes).
