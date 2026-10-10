# Second re-check, phase 29 (Haiku draft for lead review)

### S-29-1  LOW   (fix F3, [R-29-3])
- Problem: Section 7.7 still places `assess.rail_added` "in the table of section 3.17", while the F3 text says that table holds 18 rows and that `assess.rail_added` belongs to section 7+. The table has 18 rows and no `assess.rail_added` row, so the two lines disagree on where the kind is listed. The total of 19 kinds is right either way.
- Proof: docs/plan/phase-29.md:1509 "a new kind in the table of section 3.17" and docs/plan/phase-29.md:1350 "19 event kinds (the 18 rows of section 3.17 and `assess.rail_added` of section 7+)".
- Smallest fix: In docs/plan/phase-29.md line 1509, replace "a new kind in the table of section 3.17" with "a new kind of this block, counted with the 18 rows of section 3.17 in section 4.5 item 2".

## Summary
- Entries checked: 5 (F1 to F5).
- Findings: HIGH 0, MEDIUM 0, LOW 1.
- CHECKED: F1 ok, F2 ok, F3 S-29-1, F4 ok, F5 ok
