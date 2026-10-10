# Third re-check, phase 22 (Haiku draft for lead review)

### T-22-1  HIGH   (fix F1, [S-22-1])
- Problem: The F1 clause gives the approver `task.overdue` at a `remind` timeout and says `task.waiting` goes to the escalation target only, with no tier condition; but dispatcher step 5 turns every notice about a suspicion-tier resource into `task.waiting` for its recipients, so a suspicion-tier RFI's MLRO approval (`rfi_release_external`, which has a `remind` timeout, phase 19 lines 1088 and 1092) gets `task.waiting` at the remind and the two rules cannot both hold. The OLD text had the same gap, so the replacement carries it over rather than creating it.
- Proof: docs/plan/phase-22.md:1000 "where the step has a `remind` timeout, from `task.overdue` once the task is overdue" and docs/plan/phase-22.md:588 "when the tier is `suspicion` and the template is not `task.waiting`, the template is `task.waiting`" (also docs/plan/phase-22.md:750 "a suspicion-tier resource gives `task.waiting`").
- Smallest fix: In the F1 text, qualify the remind clause with "and the resource is not suspicion-tier", and add after it "a suspicion-tier resource gives its holders `task.waiting` at a remind (section 3.8 step 5)".

## Summary
- Entries checked: 1 (F1, [S-22-1]).
- Findings: HIGH 1, MEDIUM 0, LOW 0.
- FINDINGS: 1
CHECKED: F1 T-22-1
