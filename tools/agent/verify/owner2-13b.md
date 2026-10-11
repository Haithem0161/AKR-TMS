# Owner-decision check, round 2, key 13b (Haiku draft for lead review)

### P-13b-1  HIGH   (fix F6, [O-13-9])
- Problem: The F6 zone line limits the API's reads of `canon.account` and `canon.counterparty` to the proposal of item 6. Item 7 runs the decision check and the activation inside the approval transaction, which is in akr-api, and both read those two tables again. The zone line and item 7 cannot both hold.
- Proof: docs/plan/phase-13.md:934 "for the source-tag proposal of section 7.17 item 6 only." and docs/plan/phase-13.md:1377 "`check_before_decision` runs steps 2 and 3 again" (approval in akr-api: docs/plan/phase-04.md:897 "Inside the engine envelope, before the action applies, the change subject runs"; the approve route is docs/plan/phase-04.md:782).
- Smallest fix: In line 934 replace "for the source-tag proposal of section 7.17 item 6 only" with "for the source-tag proposal, its decision check and its activation (section 7.17 items 6 and 7)".

### P-13b-2  MEDIUM   (fix F2 and F4, [O-13-8])
- Problem: The F2 rationale says the hourly run does not take months already computed, and the F4 fixture uses a valid_from two months back. Base section 3.2 step 1 takes every month-end within `features.catchup_months` (3) back from the newest on every run, and step 3 computes each passing pair. A change inside that window is therefore recomputed by the hourly run; the reopen is needed only for older months. The reopen is still correct, but the stated reason is false and the fixture lies inside the window.
- Proof: docs/plan/phase-13.md:1374 "and the hourly run does not take those months again"; docs/plan/phase-13.md:605 "within `features.catchup_months` back from the newest, plus every month"; docs/plan/phase-13.md:391 "('features.catchup_months', '3', 'integer', NULL, FALSE,"; docs/plan/phase-13.md:1431 "lies two months back".
- Smallest fix: In F2 replace "and the hourly run does not take those months again" with "and the hourly run takes only the months within `features.catchup_months` (3) back from the newest (step 1), so an older month is taken again only by the reopen"; in F4 use a valid_from older than that window (for example "four months back") and the matching count in place of "the two months".

### P-13b-3  LOW   (fix F2 and F13, [O-13-8])
- Problem: F2 says a queued hourly run holds the plain key `feature_compute` and would swallow the reopen. Phase 00 enqueues each periodic kind with the slot key `<kind>:<slot ISO>`, so a scheduled run never holds the plain key. The separate key is still right, because a queued catchup or replay job (plain key, section 4 row of phase 13) would swallow a plain-key reopen. F13 has the same gap: it says "another feature_compute job" without naming which one.
- Proof: docs/plan/phase-13.md:1374 "under the key `feature_compute` a queued hourly run would swallow the enqueue"; docs/plan/phase-00.md:549 "enqueue it with `coalesce_key = '<kind>:<slot ISO>'`"; docs/plan/phase-13.md:1492 "is dropped while another `feature_compute` job is queued".
- Smallest fix: In F2 replace "a queued hourly run would swallow the enqueue" with "a queued catchup or replay job, which carries that plain key, would swallow the enqueue", and in line 1492 replace "another `feature_compute` job is queued" with "another `feature_compute` job with the plain key is queued".

### P-13b-4  LOW   (fix F2, [O-13-8])
- Problem: The parenthesis says a row ended on day D still holds on D, so the month of D is the first that can change. The tag holds through `valid_to` inclusive, so the first date that can change is the day after D, and for a month-end D the reopen starts one month early. This is harmless (step 5 writes only changed rows), but the stated reason is false.
- Proof: docs/plan/phase-13.md:1374 "(a row ended on day D still holds on D, so the month of D is the first that can change)"; docs/plan/phase-13.md:1326 "AND t.booking_date >= g.valid_from AND (g.valid_to IS NULL OR t.booking_date <= g.valid_to))".
- Smallest fix: In F2 replace "so the month of D is the first that can change" with "so the first date that can change is the day after D (for a month-end D the reopen starts one month early, which is harmless)".

## Summary
- Entries checked: 15 (F1 to F15).
- Findings by severity: HIGH 1, MEDIUM 1, LOW 2 (4 in all).
- CHECKED: F1 ok, F2 P-13b-2/P-13b-3/P-13b-4, F3 ok, F4 P-13b-2, F5 ok, F6 P-13b-1, F7 ok, F8 ok, F9 ok, F10 ok, F11 ok, F12 ok, F13 P-13b-3, F14 ok, F15 ok
