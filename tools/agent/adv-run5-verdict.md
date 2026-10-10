# Adversarial pass run 5: lead verdict (2026-10-10; decisions final, applied by apply-adv-run5.py)

Haiku draft tools/agent/adv-run5.md (existence masking leaks through counts, search, exports, events, notices and error
codes): 11 candidates; FLAW 7 (CRITICAL 4, CRITICAL or HIGH 1, HIGH 2), DOUBT 4, GUARDED 17. About 232k tokens.
Lead result: 5 confirmed (HIGH 5), 4 LOW wording or consistency fixes, 2 rejected as designed, 17 guarded
accepted. Haiku again rated by mechanism: every CRITICAL needed an id the caller already had, or was a designed reader.

## Confirmed
- ADV-5-07 HIGH (phase 22). The liaison who was shown an open branch request sees it vanish from `GET /branch/items`
  when its case or the subject party becomes suspicion tier (the `c.tier` and `effective_tier` clauses of
  `released_items`); no ordinary event removes an open item, so the vanishing tells branch staff the customer moved to
  suspicion. The section 3.12 table has no column for that liaison. Haiku said CRITICAL; the liaison is the bank's AML
  contact at the branch, not branch management, and learns a change, not a case, so HIGH. Fix: such an item is listed
  with the neutral status `closed` (the word of a withdrawn request), no due date, input refused as for a closed item;
  it ages out `branch.item_closed_days` after phase 19 closes or withdraws the RFI, like any closed item. An RFI created
  at suspicion tier (`rfi.tier`) is still never listed. Notices to the liaison use the open form of the predicate, so no
  reminder reaches the liaison for an item shown closed. Haiku's draft fix is taken in substance.
- ADV-5-03 HIGH (phase 18). `POST /cases/{id}/links` checks existence only. A suspicion party the caller may not see
  gives 201 (and then a masked link) where a random id gives 422 `unknown_party`. Fix: after the existence read the
  `EntityMask` rule (section 3.5 steps 1 to 3) runs on the target's owner party; a target the caller may not see answers
  exactly as a missing one (`unknown_party`, `unknown_account`, `unknown_txn`), with a `deny` access event.
- ADV-5-10 HIGH (phase 17, with phases 18 and 19). Merge validation answers sealed (409), `family_differs` and
  `tier_lower` (422) before step 4, the Cedar check on the target taken from the body. A caller who knows a hidden
  case's id learns that it exists, whether it is sealed, and its family. Haiku said CRITICAL; the id must be known, so
  HIGH. Fix: step 4 runs first, after the two locks; a target the caller may not see answers the masked 404 and no
  other check runs; the refusals added by runs 4 (phases 18, 19) also come after it.
- ADV-5-11 HIGH (phase 17). The bulk dry run reports `not_visible` for a hidden id while a missing id is `not_found`
  (the real run already says `not_found`). Fix: the dry run reports `not_found` for both; `not_visible` leaves the list;
  the other reasons are only given for visible ids.
- ADV-5-05 HIGH (phase 19), the doubt resolved. `followup.a2.alarm` admits every A2 to every `touch_breach` and
  `work_item_breach`, and `AlarmRepository::list_visible` has no written predicate; the row carries the subject id, so an
  A2 outside the pool learns that a hidden alert or work item exists (its own read gives 404). Fix: a breach alarm is
  visible to M and to an A2 who can see its subject (phase 17 section 3.10 for the alert; the phase 18 case predicate for
  the work item's case); others do not get it in the list, in the counts by kind, or on read and acknowledge (404 as a
  missing alarm). The phase 22 alarm audience follows.

## LOW (wording or consistency; no leak)
- ADV-5-01 (phases 03, 17), main claim GUARDED. The authorize layer runs before the handler (phase 03 section 3.2 steps
  7 to 9) and the phase 17 forbids `alerts.forbid.suspicion.alert` and `alerts.forbid.suspicion.case` (with the network
  and purpose forbids) give the masked 404 to a caller outside the team before any step-up. The 401 reaches admitted
  callers only. Residual: DR-48 and phase 03 section 3.6 step 4 say a masked read never gets 401 or 422, while phases 17,
  18 and 19 answer 401 (and phases 18, 21 answer 422) to admitted callers. Fix: phase 03 7+ states that step 4 is for a
  caller the policy refuses; an admitted caller may get 401 `E_STEPUP_REQUIRED` or 422 `E_PURPOSE_REQUIRED`, which
  confirms nothing it may not know. Phase 17's sentence "before any tier answer" is read as "after the Cedar decision".
- ADV-5-02 (phase 18), the doubt resolved. Section 3.14 step 1 refuses on the section 3.10 party rule, whose rule 4
  needs a purpose, while step 2 answers 422 for a restricted party without purpose (DR-46 and the test). Fix: step 1
  refuses on rules 1 and 5 (the masked 404); the purpose of rule 4 is answered by step 2 (422).
- ADV-5-06 (phase 18), the doubt resolved. Phase 18 section 3.10 "Lists" (filtered before the page is cut) and phase 03
  section 3.6 already cover the search; the 7.12 text does not say it and its test lacks the over-limit case. Fix: one
  sentence (`has_more` from the filtered query) and the test.
- ADV-5-08 (phase 17), the attack REJECTED, a trigger gap found. A pool member keeps seeing a case that becomes suspicion
  (phase 17 section 3.10 rule 5 admits holders of a grant on `assignee_pool_id`), so nothing vanishes for them. But the
  counts trigger for a suspicion row names "only the case assignees and M", so those pool members, and from phase 18 the
  `casework.case_acl` members, keep stale counts. Fix: the trigger resolves the principals of rule 5.

## Rejected
- ADV-5-04 (internal-report register to every A2): designed. Phase 19 section 3.14 gives the register to its readers
  (the Cedar permit `followup.a2.report`; "the suspicion-tier register for A2 and M" for authority requests; "the body
  sealed to the register readers"). The body is open to those readers, so hiding the subject alone would change
  nothing. Phase 22 Open doubts 27 says the subject is "an M-only fact"; that sentence is wrong and is closed by a LOW
  note in phase 22 (the register readers are A2 and M). If the bank wants the register M-only, it is a later choice.
- ADV-5-09 (STR counts on the regulatory-deadline tile and the `cbi_monthly` pack for CH and IA): designed. Phase 21
  section 3.21 ("Periodic packs (R3). Aggregates only, with cells below the threshold suppressed") and phase 24 section
  3.7 Rules (counts below `report.small_number_threshold` suppressed, no id, name or amount). An aggregate at or above
  the threshold names no case.

## Guarded (accepted as checked)
The 17 one-line items of the draft: masked 404 body, headers and floor; lists filtered in SQL with no total; claim
checks visibility before `E_CLAIMED`; the counts stream for non-members; the branch list, detail and counts share one
predicate; branch monthly counts suppressed; notices drop recipients who cannot read and use `task.waiting` for
suspicion; e-mail generic; entity links masked on read; profile masking from phase 15; R2 masked 404 and counts; pack
cells suppressed; tiles suppressed; RFI desk masked 404; chain and conflict alarm readers; reporter identity M only.

DR: DR-411 (the run 5 fixes).
