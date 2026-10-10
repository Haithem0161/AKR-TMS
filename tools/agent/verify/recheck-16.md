# Re-check, phase 16 (Haiku draft for lead review)

### R-16-1  LOW   (fix F1)
- Problem: F1 says this phase adds "the four entry points of section 3.2", but section 3.2 names only three of them (`FeatureCompiler::compile_measure`, `FeatureCompiler::compile_filter` and `ProfileComparator::plan`). `ProfileComparator::evaluate` is named in section 3.6 step 8 and not in section 3.2.
- Proof: docs/plan/phase-16.md:864 "except that this phase adds the four entry points of section 3.2 to phase 13's module" and docs/plan/phase-16.md:1162 "`ProfileComparator::evaluate` with `combine`".
- Smallest fix: in line 864 change "the four entry points of section 3.2" to "the four entry points of sections 3.2 and 3.6".

### R-16-2  LOW   (fix F8)
- Problem: The first clause of block 7.15 makes any set inside the band that does not loosen "minor", replay only. Phase 04 raises every capacity reason to at least material, and the same block requires a below-the-line sample for every capacity reason, so a capacity set inside the band is not replay-only. The clause is attributed to DSL D.3, whose minor row names no capacity case.
- Proof: docs/plan/phase-16.md:2045 "DSL D.3 holds: a set inside the band that does not loosen is minor and needs a replay" and docs/plan/phase-04.md:828 "`reason_tag = capacity` -> at least `material` (S-39)".
- Smallest fix: in line 2045 change "a set inside the band that does not loosen is minor" to "a set inside the band that does not loosen and has a risk reason is minor".

### R-16-3  MEDIUM   (fix F10)
- Problem: Block 7.17 narrows the test route to unit cases (step 4 runs only `tests.unit`, and step 5 sets `tests_passed_at` on unit passes), but two base-text lines still say the route runs unit and seeded cases, and block 7.17 does not say that it replaces either line. Section 3.12 (schedules and triggers) runs seeded cases only on release, schedule, manual request and gate G3, so the route row is also out of step with section 3.12.
- Proof: docs/plan/phase-16.md:890 "Unit and seeded cases of the draft; `draft` to `testing` on pass" and docs/plan/phase-16.md:80 "set when the unit and seeded cases pass (DSL A.9)"; the new rule is docs/plan/phase-16.md:2062 "for each entry of `tests.unit`".
- Smallest fix: in line 890 change "Unit and seeded cases of the draft" to "Unit cases of the draft", and in line 80 change "when the unit and seeded cases pass" to "when the unit cases pass".

### R-16-4  MEDIUM   (fix F12)
- Problem: F12 classifies `touch_id` on `scn.scenario_runs`, but the CREATE TABLE of migration 0055 in section 1.2 has no `touch_id` column, and its batch index still keys on `batch_id`. The column exists only in the prose of block 7.9 ("Migration `0055` adds `touch_id UUID` ..."), with no SQL. The migration text as written therefore does not create the column that F12 classifies, nor the column that the run ledger of block 7.10 uses.
- Proof: docs/plan/phase-16.md:207 "CREATE UNIQUE INDEX IF NOT EXISTS uq_scenario_runs_batch ON scn.scenario_runs (scenario_version_id, mode, batch_id)" and docs/plan/phase-16.md:1986 "Migration `0055` adds `touch_id UUID` to `scn.scenario_runs`".
- Smallest fix: add `touch_id UUID,` after line 185 (`batch_id`), add `CHECK (touch_id IS NULL OR batch_id IS NOT NULL)` to the CHECK list at line 202, and replace the index at line 207 with the touch_id index of block 7.9.

## Summary
Fixes checked: 18 (F1 to F18).
Findings: HIGH 0, MEDIUM 2, LOW 2 (4 in total).
CHECKED: F1 R-16-1, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok, F8 R-16-2, F9 ok, F10 R-16-3, F11 ok, F12 R-16-4, F13 ok, F14 ok, F15 ok, F16 ok, F17 ok, F18 ok
