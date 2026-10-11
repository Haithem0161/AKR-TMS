# Owner-decision check, round 2, key 14b (Haiku draft for lead review)

### P-14b-1  LOW   (fix F10, [O-07-10])
- Problem: The pointer "phase 04 section 3.2" is wrong for the rollback rule: section 3.2 (lines 852 to 951) has no rollback wording, and that rule sits in section 3.4 item 9. The scheduled failure is in the section 3.6 timer table, which cites section 3.2 only for the failed state.
- Proof: docs/plan/phase-14.md:1581 "the target refuses (phase 04 section 3.2: an immediate approval is rolled back with the error, and a scheduled activation ends `failed`)"; docs/plan/phase-04.md:1023 "An effect error aborts the whole action (the approval is rolled back and answered" (section 3.4, which runs from line 974); docs/plan/phase-04.md:1103 "a stale base or a target refusal -> `failed` (section 3.2)" (section 3.6).
- Smallest fix: In docs/plan/phase-14.md line 1581 (the Activate row), replace "phase 04 section 3.2:" with "phase 04 section 3.4 item 9 for the rollback and section 3.6 for the scheduled failure:"; F11 needs no change of its own.

## Summary
- Entries checked: 13 (F1 to F13), all in docs/plan/phase-14.md.
- Findings by severity: HIGH 0, MEDIUM 0, LOW 1.
- The L-1 entries (F6, F7, F8) were checked against the GRANT lines of phase 14 (lines 421 to 447, 1501 and 1502), the guard triggers (phase 14 lines 453, 487, 504 to 505, 1488 and 1491), and the owner role `akr_migrate` (phase 00 line 181). Each NEW text matches what `akr_app` lacks and what the trigger raises.
- F13: the sentence's correction (`results`, one element per case) is carried by item 6 of the same 7.19 block.

CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok, F8 ok, F9 ok, F10 P-14b-1, F11 ok, F12 ok, F13 ok
