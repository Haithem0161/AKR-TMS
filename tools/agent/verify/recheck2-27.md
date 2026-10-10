# Second re-check, phase 27 (Haiku draft for lead review)

### S-27-1  MEDIUM   (fix F5, [R-27-5])
- Problem: The lock-order paragraph still states the old transaction unit for the sweep, "each batch is one transaction". Fix F5 makes step 6 write one transaction per chunk of step 2, so both lines cannot hold.
- Proof: docs/plan/phase-27.md:741 "The sweep and the monitor never hold a lock across the batch: each batch is one transaction." and docs/plan/phase-27.md:500 "Insert `alert.model_scores` in one transaction per chunk of step 2".
- Smallest fix: In docs/plan/phase-27.md line 741 change "each batch is one transaction" to "each sweep chunk of step 2 is one transaction".

### S-27-2  HIGH   (fix F11, [R-27-7e]; with fix F7 and the row at line 602)
- Problem: Test l still answers 422 `E_PURPOSE_REQUIRED` to a team member of a suspicion-tier alert who sends no purpose, but fix F7 says a row the caller may not see answers the masked 404 "whatever header is missing", rule 5 of phase 17 (the visibility build that route 16 uses) admits a suspicion row only with a purpose code present, and phase 03 section 7.14 counts the purpose code among the suspicion refusals. The E_PURPOSE_REQUIRED row at line 602 gives the same 422 with no caller condition, so the two answers cannot both hold.
- Proof: docs/plan/phase-27.md:505 "A row the caller may not see answers the masked 404 `E_NOT_FOUND`, whatever header is missing"; docs/plan/phase-27.md:844 "on the team with step-up and a missing purpose: 422 `E_PURPOSE_REQUIRED`"; docs/plan/phase-27.md:602 "on a restricted or suspicion-tier alert without the purpose phase 17 requires"; docs/plan/phase-27.md:714 "under the alert's visibility and the alert's own purpose rules". Basis: docs/plan/phase-17.md:1229 "a purpose code is present" (rule 5), and docs/plan/phase-03.md:1376 "never 401 or 422, whatever header is missing".
- Smallest fix: In line 844 change "on the team with step-up and a missing purpose: 422 `E_PURPOSE_REQUIRED`" to "on the team with step-up and a missing purpose: 404 `E_NOT_FOUND`, the masked answer (phase 03 section 7.14)", and in line 602 limit the 422 to "a caller the visibility rule admits, on a restricted alert without the purpose" (the other way out, keeping 422 for a team member, is to change "whatever header is missing" in line 505 to "whatever step-up header is missing" and state that the purpose check runs before visibility).

### S-27-3  LOW   (fix F11, [R-27-7e])
- Problem: The latency clause of test l cites phase 03 section 7.14, which states no latency floor. The floor is phase 03 section 3.6 step 3.
- Proof: docs/plan/phase-27.md:844 "and the latency within the phase 03 floor (phase 03 section 7.14)"; docs/plan/phase-03.md:788 "3. Latency floor: a 404 on a masked kind is released no earlier than".
- Smallest fix: In line 844 change "(phase 03 section 7.14)" to "(phase 03 section 3.6 step 3; the outside-team 404 is phase 03 section 7.14)".

## Summary
- Entries checked: 11 (F1 to F11).
- Findings: HIGH 1, MEDIUM 1, LOW 1 (3 in total).
CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 S-27-1, F6 ok, F7 S-27-2, F8 ok, F9 ok, F10 ok, F11 S-27-2, S-27-3
