# Re-check, phase 05 (Haiku draft for lead review)

### R-05-1  LOW   (fix F1)
- Problem: Fix F1 moves the run report write to `put_generated`, which emits `evidence.stored` (phase 02). The "Reused" line of section 3.10 still lists `evidence.staged` and `evidence.sealed` for this phase, does not name `evidence.stored`, and `evidence.sealed` is defined in no phase file.
- Proof: docs/plan/phase-05.md:1264 "Reused: `change_request.*` (phase 04) for the two targets; `evidence.staged`, `evidence.sealed` (phase 02)." Second quote: docs/plan/phase-02.md:879 "and emitting `evidence.stored`" (the event of `put_generated`; the report now uses it, see docs/plan/phase-05.md:1141).
- Smallest fix: In docs/plan/phase-05.md:1264 replace "`evidence.staged`, `evidence.sealed` (phase 02)" with "`evidence.staged` (source documents), `evidence.stored` (the run report) (phase 02)".

### R-05-2  LOW   (fix F3)
- Problem: Fix F3 gives VD-02 a "(partly: ...)" qualifier but leaves it in the "implemented here" list. The header's "partly implemented" list, on the next line, is where partial items sit in this phase's gap ledger, so the item is in the wrong list.
- Proof: docs/plan/phase-05.md:36 "VD-02 (partly: this phase reads the inputs to each mapping" and docs/plan/phase-05.md:37 "Gap pass 1 (coverage), partly implemented: C-31".
- Smallest fix: Move the item "VD-02 (partly: ... published under `cr_mapping`)" from line 36 to the start of the "partly implemented" list on line 37, keeping its text.

### R-05-3  MEDIUM   (fix F4)
- Problem: The 7.12 block says it replaces "each base sentence" that says the pause row feeds a coverage gap or a backfill, but it names four places only. The retention seed row for `admin.source_pauses` still calls the table "Coverage-gap evidence", which is the old rule and contradicts the block's own statement that the table is "the record of who paused a source, when and why".
- Proof: docs/plan/phase-05.md:241 "'Coverage-gap evidence (C-15); as its source'" and docs/plan/phase-05.md:1590 "stays the record of who paused a source, when and why."
- Smallest fix: In docs/plan/phase-05.md:241 change the note to 'Record of who paused a source, when and why (C-15); as its source'.

### R-05-4  LOW   (fix F4)
- Problem: The "What This Phase Does NOT Touch" bullet still places a backfill planner in phase 11. Fix F4 says phase 11 builds no backfill from the pause row (its 7.4), and phase 11 section 3.6 says no backfill plan is needed for a gap, so the pointer is a leftover of the old rule.
- Proof: docs/plan/phase-05.md:1596 "no backfill planner (phase 11, DR-63):" and docs/plan/phase-11.md:963 "plan is needed for the gap; the operator-planned backfill of API.md 3.5 (phase 12) is for explicit slices only."
- Smallest fix: In docs/plan/phase-05.md:1596 delete "no backfill planner (phase 11, DR-63)," from the bullet, or replace it with "no backfill planner (none is built; section 7.12)".

## Summary
- Fixes checked: 5 (F1 to F5).
- Findings: HIGH 0, MEDIUM 1, LOW 3.
- CHECKED: F1 R-05-1, F2 ok, F3 R-05-2, F4 R-05-3 and R-05-4, F5 ok

