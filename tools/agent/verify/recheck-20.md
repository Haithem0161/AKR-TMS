# Re-check, phase 20 (Haiku draft for lead review)

### R-20-1  HIGH   (fix F1)
- Problem: Phase 04 leaves an existing family alone (`exit_decision` already exists from phase 15) and says a release never changes a bank's published definition by itself, so the install cannot publish version 2 and no exit task is created. The two rules cannot both hold, and the "three definitions" count of 7.4 depends on this path.
- Proof: docs/plan/phase-20.md:1088 "This phase seeds version 2 of the shipped definition `exit_decision` through `akr-ctl workflow seed`"; docs/plan/phase-04.md:1253 "the family exists -> left alone and reported"; docs/plan/phase-04.md:1255 "a release never changes a bank's published definition by itself".
- Smallest fix: In line 1088 replace "seeds version 2 of the shipped definition `exit_decision` through `akr-ctl workflow seed`" with "ships version 2 of `exit_decision` as a draft that the bank publishes by workflow change request, because the phase 04 seed loader leaves an existing family alone".

### R-20-2  LOW   (fix F1)
- Problem: The pointer "(section 3.2)" is wrong. Section 3.2 is the neutral text table and does not mention the exit instance; the rule that `source_id` holds the exit instance is in the column comment of section 1.1.
- Proof: docs/plan/phase-20.md:1088 "the instance as `source_id` (section 3.2)"; docs/plan/phase-20.md:559 "### 3.2 Neutral instruction texts and the scan"; docs/plan/phase-20.md:56 "freezes.id, refrain_instructions.id or the exit_decision instance; no FK".
- Smallest fix: In line 1088 change "(section 3.2)" to "(section 1.1)".

### R-20-3  MEDIUM   (fix F2)
- Problem: The `NewVerification` list omits `ops_task_id`, a NOT NULL column of `casework.ops_verifications` with no default, so the list does not give a complete insert row. The other input rows of the same block list their foreign keys.
- Proof: docs/plan/phase-20.md:1094 "`NewVerification` holds the values of check step 7 (section 3.3)"; docs/plan/phase-20.md:95 "UUID NOT NULL REFERENCES casework.ops_tasks(id) ON DELETE RESTRICT".
- Smallest fix: In line 1094 add "`ops_task_id` (the task checked)" to the list of values.

### R-20-4  MEDIUM   (fix F2)
- Problem: The derived values listed for `NewOpsTask` omit `authority_ref`, which the note on step 6 says `OpsTaskService::create` writes as the task's own `task_ref` in the same row. The input row for the freeze path, where the caller passes that marker, is therefore still undefined.
- Proof: docs/plan/phase-20.md:1091 "plus the values creation derives: `task_ref` (step 3), `neutral_text_code` (step 2), `state = open` and `version = 1` (step 4)"; docs/plan/phase-20.md:652 "writes it as `authority_ref` in the same row, so no second write is needed".
- Smallest fix: In line 1091 add "`authority_ref` (the task's own `task_ref` when the caller passes the marker of section 3.4 step 6)" to the derived values.

### R-20-5  LOW   (fix F15)
- Problem: The sentence that F15 edits contains the doubled phrase "the reference and reference equal to", which does not read as correct English.
- Proof: docs/plan/phase-20.md:589 "the reference and reference equal to `cbs_reference`, open at `now()`".
- Smallest fix: In line 589 change "the reference and reference equal to" to "the reference equal to".

### R-20-6  LOW   (fix F16)
- Problem: The REP puts the server answer between "Confirm" (V0) and the clause "with the note that the confirmation is flagged", which now hangs after a semicolon and reads as if it describes the screen answer.
- Proof: docs/plan/phase-20.md:991 "the screen shows that answer; with the note that the confirmation is flagged;".
- Smallest fix: In line 991 move "with the note that the confirmation is flagged" to directly follow `shows "Confirm" (V0) after completion`, before the first semicolon.

### R-20-7  LOW   (fix F17)
- Problem: The fix names the inbox "W-OPS", an ID that occurs nowhere else in the plan. The catalogue ID of this screen is W9, as the heading of section 5.1 and the sentence above it say.
- Proof: docs/plan/phase-20.md:1118 "This is the day window of the W-OPS screen (section 5.1)"; docs/plan/phase-20.md:988 "### 5.1 W9 Operations-task inbox"; docs/plan/phase-20.md:986 "catalogue IDs W9 and S5".
- Smallest fix: In line 1118 change "the W-OPS screen" to "the W9 screen".

## Summary
- Fixes checked: 20 (F1 to F20).
- Findings by severity: HIGH 1, MEDIUM 2, LOW 4 (7 in total).
- CHECKED: F1 R-20-1 R-20-2, F2 R-20-3 R-20-4, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok, F8 ok, F9 ok, F10 ok, F11 ok, F12 ok, F13 ok, F14 ok, F15 R-20-5, F16 R-20-6, F17 R-20-7, F18 ok, F19 ok, F20 ok
