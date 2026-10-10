# Second re-check, phase 24 (Haiku draft for lead review)

### S-24-1  MEDIUM   (fix F4, [R-24-4])
- Problem: The F4 text says G5 has no entry and G6 is never exported, but the first sentence of section 7.12 still says every REGISTER screen of the catalogue offers one export. G5 and G6 are REGISTER screens (sections 5.3 and 5.4) owned by this phase (sections 3.2 and 3.1), and the owner rule in the same 7.23 bullet, "the phase that owns a REGISTER screen adds its entry", also requires entries for them, so the two statements cannot both hold.
- Proof: docs/plan/phase-24.md:1331 "Each REGISTER screen of the catalogue offers one export, built as a job." and docs/plan/phase-24.md:1478 "G6 is never exported (section 3.1 step 9)".
- Smallest fix: In line 1331 insert "that has an entry in section 7.23" after "Each REGISTER screen of the catalogue", and in line 1478 insert "except G5 and G6," after "the phase that owns a REGISTER screen adds its entry,".

## Summary
Entries checked: 9 (F1 to F9).
Findings: HIGH 0, MEDIUM 1, LOW 0.
CHECKED: F1 ok, F2 ok, F3 ok, F4 S-24-1, F5 ok, F6 ok, F7 ok, F8 ok, F9 ok
