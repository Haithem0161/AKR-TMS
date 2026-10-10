# Second re-check, phase 22 (Haiku draft for lead review)

### S-22-1  MEDIUM   (fix F1, [R-22-1])
- Problem: The NEW clause says an approver learns of an open deciding task from `task.overdue` once the task is overdue. Phase 04 ties `task.overdue` to the `remind` timeout only, and most shipped deciding steps carry only an `escalate` timeout, so their approver gets no overdue notice at all: cr_minor, cr_mapping, cr_workflow, cr_material_validated, and the MLRO and CHK approvals of cr_material.
- Proof: docs/plan/phase-04.md:1265 "| `cr_minor` | `approve` role CHK | esc CH 3bd | `activate_change` |" (the same escalate-only timeout at lines 1267, 1269 and 1272, and for the MLRO and CHK approvals at line 1266); docs/plan/phase-04.md:1713 "`remind` writes, in its transaction, the outbox row `notify.request {task_id, template_code: "task.overdue", to:"; docs/plan/phase-22.md:1000 "once the task is overdue, from `task.overdue`".
- Smallest fix: In docs/plan/phase-22.md:1000 change "and, once the task is overdue, from `task.overdue`" to "and, for a step with a `remind` timeout, once the task is overdue, from `task.overdue`; a step with only an `escalate` timeout sends its approver no overdue notice (phase 04 section 3.8, shipped definitions)".

## Summary
- Entries checked: 3 (F1, F2, F3).
- Findings: HIGH 0, MEDIUM 1, LOW 0.
- CHECKED: F1 S-22-1, F2 ok, F3 ok
