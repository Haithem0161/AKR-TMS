# Re-check, phase 07 (Haiku draft for lead review)

### R-07-1  HIGH   (fix F1)
- Problem: F1 sets the total to 129, but the route tables give 130. Phases 01, 03, 04, 05 and 06 hold 22, 32, 32, 21 and 14 operations (no method and path pair repeats across them), and phase 07 adds 9. Phase 04's section 2 table has 31 rows (docs/plan/phase-04.md lines 776 to 806), but its section 7.2 adds a 32nd route, `POST /change-requests/{id}/evidence`, which the declared count leaves out; F1 copied that stale total. The same stale count sits in phase 04 (lines 771 and 1502) and in phase 06 (line 961 says 120, which should be 121).
- Proof: docs/plan/phase-07.md:954 "lists 9 new operations (129 with phases 01 and 03 to 06)" and docs/plan/phase-04.md:1654 "New route (gap item (d))" (phase 04 declares "(31 operations; scope = API.md s.2" at docs/plan/phase-04.md:771).
- Smallest fix: In docs/plan/phase-07.md line 954 change "(129 with phases 01 and 03 to 06)" to "(130 with phases 01 and 03 to 06)", and correct the same stale totals in phase 04 (31 to 32 at lines 771 and 1502) and phase 06 (120 to 121 at line 961).

## Summary
Fixes checked: 31 (F1 to F31). Findings by severity: HIGH 1, MEDIUM 0, LOW 0.
CHECKED: F1 R-07-1, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok, F8 ok, F9 ok, F10 ok, F11 ok, F12 ok, F13 ok, F14 ok, F15 ok, F16 ok, F17 ok, F18 ok, F19 ok, F20 ok, F21 ok, F22 ok, F23 ok, F24 ok, F25 ok, F26 ok, F27 ok, F28 ok, F29 ok, F30 ok, F31 ok
