# Second re-check, phase 21 (Haiku draft for lead review)

### S-21-1  HIGH   (fix F5, [R-21-2])
- Problem: The new trailer line says this phase "only implements" the phase 20 port method `LetterRequester::delivered`, but section 3.15 says this phase also implements `LetterRequester::request` (the `LetterService` replaces the recording stub as the port). The two statements cannot both hold, so the scope fence contradicts the port section.
- Proof: docs/plan/phase-21.md:1503 "this phase only implements the phase 20 port method `LetterRequester::delivered`" and docs/plan/phase-21.md:970 "The port (phase 20 `LetterRequester::request`, implemented here)."
- Smallest fix: In docs/plan/phase-21.md:1503, replace "this phase only implements the phase 20 port method `LetterRequester::delivered` (section 4.5 item 9)" by "this phase implements the phase 20 port `LetterRequester` (`request` in section 3.15, `delivered` in section 4.5 item 9)".

## Summary
- Entries checked: 8
- Findings: HIGH 1, MEDIUM 0, LOW 0
CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 S-21-1, F6 ok, F7 ok, F8 ok
