# Second re-check, phase 16 (Haiku draft for lead review)

### S-16-1  LOW   (fix F2, [R-16-2])
- Problem: F2 narrows the clause to "has no capacity reason", but it still sits behind the lead-in "DSL D.3 holds:". DSL D.3's Minor row has no capacity exception, so the cited source no longer supports the clause. The capacity exclusion comes from S-39 (phase 04 line 29: a capacity reason makes a change material), which is the lead's chosen rule. Before this replacement the cited sentence matched the DSL row exactly.
- Proof: docs/plan/phase-16.md:2047 "DSL D.3 holds: a set inside the band that does not loosen and has no `capacity` reason is minor and needs a replay"; second quote, docs/DSL.md:927 "| Parameter value inside the approved band, not loosening | Minor | Replay |".
- Smallest fix: In docs/plan/phase-16.md line 2047, change "DSL D.3 holds:" to "DSL D.3 holds, and S-39 (phase 04 line 29) makes a capacity reason material:".

## Summary
Entries checked: 8 (F1 to F8). Findings: HIGH 0, MEDIUM 0, LOW 1.
CHECKED: F1 ok, F2 S-16-1, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok, F8 ok
