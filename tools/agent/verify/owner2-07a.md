# Owner-decision check, round 2, key 07a (Haiku draft for lead review)

### P-07a-1  HIGH   (fix F9, [O-07-4])
- Problem: Step 1 still ends a non-proposed case with "done" before the delete clause, so a rejected case with no `intake_result` keeps its file (the O-07-4 fault, and check F13 at line 1232 would fail as written). The delete clause is also not limited to non-proposed states, so a proposed first run, which has no `intake_result` either, matches it and deletes the staged file before step 3 opens it.
- Proof: docs/plan/phase-07.md:1172 "A state other than `proposed` -> done; when the case has no `intake_result` (its request was withdrawn or rejected before the first run, so the uploaded file is still staged), first `delete_transient("goaml", case_id)`" and docs/plan/phase-07.md:1174 "First run (`evidence_id` is NULL): `open_transient("goaml", case_id, 1)`; no object -> refusal `file_gone`."
- Smallest fix: In item 6 step 1 (docs/plan/phase-07.md line 1172), replace the text from "A state other than `proposed` -> done;" up to "`delete_transient("goaml", case_id)`" by "When the state is not `proposed`: when the case has no `intake_result` (its request was withdrawn or rejected before the first run, so the uploaded file is still staged), first `delete_transient("goaml", case_id)` and the audit event, then done; when the state is `proposed`, go on with step 2", so that the delete applies only to non-proposed cases and comes before the end.

## Summary
- Entries checked: 16 (F1 to F16, all in docs/plan/phase-07.md).
- FINDINGS: 1 (HIGH 1, MEDIUM 0, LOW 0).
- CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok, F8 ok, F9 P-07a-1, F10 ok, F11 ok, F12 ok, F13 ok, F14 ok, F15 ok, F16 ok
