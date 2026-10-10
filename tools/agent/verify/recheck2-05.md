# Second re-check, phase 05 (Haiku draft for lead review)

### S-05-1  MEDIUM   (fix F5, [R-05-4])
- Problem: The F5 text says that no phase builds a backfill planner (DBS decision 380), but base text in section 3, which no 7.x block replaces, still says the per-year counts show the volume curve "the backfill planner of phase 12 uses". Phase 12 builds no such planner (docs/plan/phase-12.md:1381), so the two statements cannot both hold.
- Proof: docs/plan/phase-05.md:1095 "they show the volume curve the backfill planner of" (continued on line 1096 "phase 12 uses);") and docs/plan/phase-05.md:1596 "no phase builds a backfill planner".
- Smallest fix: On lines 1095-1096, replace "they show the volume curve the backfill planner of phase 12 uses" with "they show the volume curve of the object".

## Summary
Entries checked: 6 (F1 to F6)
Findings: HIGH 0, MEDIUM 1, LOW 0
CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 S-05-1, F6 ok
