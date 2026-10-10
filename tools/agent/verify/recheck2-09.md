# Second re-check, phase 09 (Haiku draft for lead review)

### S-09-1  MEDIUM   (fix F5, [R-09-4])
- Problem: The first sentence of the Keys paragraph still says that each file of `backend/packs/reference/` carries the rows of one table. The F5 text in the same paragraph says the default calendar file carries the rows of two tables (its one `ref.calendars` row and its `ref.calendar_days` rows), so the two statements cannot both hold for that file. The replacement left the general sentence unchanged.
- Proof: docs/plan/phase-09.md:1353 "each file of `backend/packs/reference/` carries its `release_id` (1 to 64 characters) and the rows of one table." and docs/plan/phase-09.md:1353 "The fourth release file, the default calendar, carries `release_id`, the columns of its one `ref.calendars` row and its `ref.calendar_days` rows (section 1)".
- Smallest fix: In docs/plan/phase-09.md line 1353, change "each file of `backend/packs/reference/`" to "each of the three table files of `backend/packs/reference/`".

### S-09-2  MEDIUM   (fix F3, [R-09-2b])
- Problem: The row now ends with the code `calendar_fixed`. No check of this phase names that code, and no other file of docs/plan uses it. Step 4 of section 3.5 says only "is not a fixed clock", and "fixed" is defined for clocks and switches (`E_INVARIANT_LOCKED`), not for calendars. Of the codes in that row, this is the only one without a defining check.
- Proof: docs/plan/phase-09.md:927 "`role_lacks_scope`, `calendar_fixed` |" and docs/plan/phase-09.md:805 "`calendar_id` exists and is not a fixed clock; every queue and pool named exists".
- Smallest fix: In docs/plan/phase-09.md line 927, delete "`calendar_fixed`" from the row, unless step 4 of section 3.5 (line 805) is given the sentence that names the case and returns that code.

## Summary
- Entries checked: 5 (F1 to F5).
- Findings: HIGH 0, MEDIUM 2, LOW 0.
- CHECKED: F1 ok, F2 ok, F3 S-09-2, F4 ok, F5 S-09-1

