# Re-check, phase 06 (Haiku draft for lead review)

### R-06-1  LOW   (fix F4)
- Problem: 7.10 makes `code` and `name` of a linked entry optional but not `restricted`, which step 4 names only inside a restricted entry. Under the block's own rule every named key not listed is required, so every normal linked entry would need a `restricted` key that its shape does not carry.
- Proof: docs/plan/phase-06.md:1051 "and the `code` and `name` of a `linked` entry (absent for a restricted scenario, which carries `restricted: true`)" and docs/plan/phase-06.md:645 "code and name, coverage, grade}] (a restricted scenario appears as `{restricted: true, coverage, grade}`".
- Smallest fix: In 7.10, after "and the `code` and `name` of a `linked` entry", add ", and `restricted` (present only, as true, on a restricted entry)".

### R-06-2  HIGH   (fix F5)
- Problem: The rule F5 keeps in step 1, "a run that is not queued -> done", contradicts section 4's retry rule. After a failed render the retry finds the run already set to running by the first attempt, exits at step 1 and leaves the run running, which blocks new runs through `uq_gap_report_runs_active` (docs/plan/phase-06.md:254).
- Proof: docs/plan/phase-06.md:633 "a run that is not `queued` -> done." and docs/plan/phase-06.md:845 "a retry resumes from the step the run reached".
- Smallest fix: In step 1 of 3.5, replace "a run that is not `queued` -> done." with "a `completed` or `failed` run -> done; a `running` run resumes", and let `GapReportRepository::start` return the run in both states.

### R-06-3  HIGH   (fix F6)
- Problem: No compose volume gives akr-worker the installation file. Phase 00 section 7.8 lists the worker's volumes without it (docs/plan/phase-00.md:951), phase 08 says only that Caddy serves it (docs/plan/phase-08.md:517), and phase 06 section 4 says compose is unchanged. Without that mount every run ends in `fail(internal)` and the section 6 curl check that polls a run to completed fails.
- Proof: docs/plan/phase-06.md:638 "which the akr-worker service mounts read-only" and docs/plan/phase-06.md:850 "**Docker compose:** no change."
- Smallest fix: Add one read-only bind mount of `deploy/branding.json` into akr-worker to section 4 (in place of "**Docker compose:** no change.") and to the phase 00 section 7.8 volume list, or name a bank-name source that the worker already reads.

### R-06-4  LOW   (fix F6)
- Problem: The file `deploy/branding.json` is named in phase 08 section 3.10 (docs/plan/phase-08.md:517, under the heading at line 490), not in section 4 (heading at line 521), which names only the served path `/branding.json`.
- Proof: docs/plan/phase-06.md:638 "from the installation file `deploy/branding.json` of phase 08 section 4".
- Smallest fix: In that line change "of phase 08 section 4" to "of phase 08 section 3.10".

### R-06-5  MEDIUM   (fix F8)
- Problem: `AttributeKey`, the element type of `ItemGrade.missing_fields` that F8 adds, is defined nowhere in docs/plan or the planning docs, while the other types of that struct (`Grade`, `Reason`, `Term`, `GradeUpsert`) are defined in section 2. The name appears only in phase 06 base text (section 2 `Need`, section 3.2 `maintained`) with no definition.
- Proof: docs/plan/phase-06.md:1055 "`ItemGrade { catalogue_item_id: Uuid, grade: Grade, reasons: Vec<Reason>, missing_fields: Vec<AttributeKey> }`".
- Smallest fix: Add to the section 2 value objects: "`AttributeKey` (a newtype over the canonical key `<namespace>.<name>` of section 7.3)".

## Summary
- Fixes checked: 23 (F1 to F23).
- Findings: HIGH 2, MEDIUM 1, LOW 2 (5 in all).
- CHECKED: F1 ok, F2 ok, F3 ok, F4 R-06-1, F5 R-06-2, F6 R-06-3 and R-06-4, F7 ok, F8 R-06-5, F9 ok, F10 ok, F11 ok, F12 ok, F13 ok, F14 ok, F15 ok, F16 ok, F17 ok, F18 ok, F19 ok, F20 ok, F21 ok, F22 ok, F23 ok
