# Third re-check, phase 01 (Haiku draft for lead review)

### T-01-1  HIGH   (fix F1, [S-01-1])
- Problem: F1 adds `status = 'active'` to the lookup that `disable` uses, but step 3 of the same block commits `status = 'disabled'` before the Hydra delete and then says the command can be run again. After a failed Hydra call a rerun finds no active principal, so the Hydra client cannot be removed by any command. The two statements cannot both hold.
- Proof: docs/plan/phase-01.md:1124 "find the principal by it (`kind = 'service'`, `status = 'active'` and" and docs/plan/phase-01.md:1132 "so a failed Hydra call leaves no access and the command can be run again." (step 3, line 1129, commits `status = 'disabled'` before the Hydra delete).
- Smallest fix: Delete "and the command can be run again" from line 1132, or, if a rerun must work, give `disable` a lookup rule that does not depend on `status = 'active'` (the lead chooses).

## Summary
- Entries checked: 2 (F1, F2).
- Findings: HIGH 1, MEDIUM 0, LOW 0.
- CHECKED: F1 T-01-1, F2 ok
