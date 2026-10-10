# Re-check, phase 09 (Haiku draft for lead review)

### R-09-1  LOW   (fix F4)
- Problem: The F4 block says section 4's phase 00 and phase 04 bullets include the rename to `add` and the handling of the `Result`. They do not: the phase 00 bullet (lines 1079-1081) covers only the production port and the test clock, and the phase 04 bullets (lines 1090-1103) name neither the rename nor the two calls.
- Proof: docs/plan/phase-09.md:1335 "Section 4's phase 00 and phase 04 bullets include this." and docs/plan/phase-09.md:1080 "the `BusinessClock` port's production implementation is `BankCalendarClock` from this phase (phase 00 keeps".
- Smallest fix: Change the sentence at line 1335 to "Section 4 does not repeat this rename.", or add the rename to the phase 00 and phase 04 bullets at lines 1079 and 1090.

### R-09-2  MEDIUM   (fix F6)
- Problem: The block says the codes `unknown_queue` and `pool_archived` are raised by the authority matrix only. The matrix has no queue or pool field and no step raises either code, so both stay listed in the section 3.8 row with no raising step.
- Proof: docs/plan/phase-09.md:1341 "the codes `unknown_queue` and `pool_archived` of section 3.8 are raised by the authority matrix only" and docs/plan/phase-09.md:860 "`grants: [{decision_type, role, max_priority, second_review}]` and".
- Smallest fix: In line 1341 replace "are raised by the authority matrix only" with "are raised by no check of this phase", and delete `unknown_queue` and `pool_archived` from the row at line 926.

### R-09-3  MEDIUM   (fix F11)
- Problem: F11 makes the save-time scan use the active pack version whatever version the case pinned, but the Activation paragraph (not touched by the fix) still says the new version is used only by cases opened after `effective_from`, so the two lines disagree for a scan of an older case.
- Proof: docs/plan/phase-09.md:797 "always against the active pack version, whatever version the case pinned" and docs/plan/phase-09.md:831-832 "the new version is used only by cases opened after `effective_from`, so nothing migrates (M-75)".
- Smallest fix: Add to the sentence at lines 831-832, after "(M-75)", the words "except the save-time forbidden-word scan of section 3.5, which always uses the active version".

### R-09-4  MEDIUM   (fix F12)
- Problem: The Keys paragraph gives row keys for countries, governorates and currencies only, then calls `reference-release.v1.json` closed over exactly these keys. The tree also puts a default calendar in `packs/reference` and says that schema validates every release file there, but a default calendar's weekend sits in `ref.calendars` and its closed days in `ref.calendar_days`, so the fix's "rows of one table" rule cannot describe it, and no key list for it exists in the phase.
- Proof: docs/plan/phase-09.md:1352 "`reference-release.v1.json` is closed and validates exactly these keys." and docs/plan/phase-09.md:396 "default calendar, reason-codes.v1.json (inactive until activated)" and docs/plan/phase-09.md:399 "validates the release files of packs/reference".
- Smallest fix: In line 1352 add the default calendar file's keys (the `ref.calendars` fields, the weekend per currency, the `ref.calendar_days` rows and `release_id`), or replace "validates exactly these keys" with "validates the three table files named above".

## Summary
- Fixes checked: 13 (F1 to F13).
- Findings: HIGH 0, MEDIUM 3, LOW 1.
- CHECKED: F1 ok, F2 ok, F3 ok, F4 R-09-1, F5 ok, F6 R-09-2, F7 ok, F8 ok, F9 ok, F10 ok, F11 R-09-3, F12 R-09-4, F13 ok
