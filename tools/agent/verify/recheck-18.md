# Re-check, phase 18 (Haiku draft for lead review)

### R-18-1  MEDIUM   (fix F4)
- Problem: Fix F4 says the opened-views route does not take evidence items; the row of an evidence item is written only by the content read (section 3.4 step 6). The W7 decision dialog text still says that each opening of a panel or item posts opened-views, so a developer following it would POST view_kind evidence and get 422.
- Proof: docs/plan/phase-18.md:1180 "the gate report as a checklist of the panels and items with the links that open them (each opening posts `opened-views`)" versus docs/plan/phase-18.md:1487 "The route does not take `evidence`: the row of an evidence item is written only by the content read".
- Smallest fix: in section 5.4 (line 1180) replace "(each opening posts `opened-views`)" with "(a panel opening posts `opened-views`; an evidence item's link writes its own row through the content read, section 3.4 step 6)".

### R-18-2  MEDIUM   (fix F8)
- Problem: The call form of GenericTextGuard::check takes conn first (section 3.3, and fix F8). The proposal text rule in section 3.7 still calls it with two arguments and no conn, so the call there differs from the one defined in 3.3 and used in 3.8.
- Proof: docs/plan/phase-18.md:737 "`text`: `GenericTextGuard::check(text, casework.decision_text_min_chars)` (section 3.3)." versus docs/plan/phase-18.md:678 "`GenericTextGuard::check(conn, body, min_chars)`".
- Smallest fix: in the section 3.7 step 3 bullet at line 737 change the call to `GenericTextGuard::check(conn, text, casework.decision_text_min_chars)`.

### R-18-3  MEDIUM   (fix F8)
- Problem: Step 4 of section 3.3 makes min_chars depend on whether the note belongs to a manual grant, but grant step 5 sends that note through NoteService::post, whose input is CreateNote with only body and amends. Nothing says how post learns that the note is a grant's note, so a developer must invent the mechanism.
- Proof: docs/plan/phase-18.md:559 "`CreateNote { body: String (1 to casework.note_max_chars), amends: Option<Uuid> }`" and docs/plan/phase-18.md:658 "`note` is required and goes through `NoteService::post` (section 3.3)".
- Smallest fix: in section 3.3 step 4 say that post takes an internal min_chars argument (the public route passes 1, and grant step 5 passes casework.decision_text_min_chars), or name the flag the grant sets.

### R-18-4  LOW   (fix F11)
- Problem: The fix says section 4.5 item 13 covers the timer kind, the two job kinds and the setting `schedule.case_acl_expiry_scan`. Item 13 names only the work_item kind and its setting; the job kinds are covered only by the words "the job kinds of section 4.1". The pointer is wrong, though the kind and the setting are defined elsewhere (section 4.2, the settings row of the migration, and 7.10 item 5).
- Proof: docs/plan/phase-18.md:1141 "13. **Phase 00**: the timer kind `work_item_due`, the job kinds of section 4.1 and the setting `schedule.work_item_due_scan`." against docs/plan/phase-18.md:1495 "Section 4.5 item 13 covers the kind, the two job kinds and the setting `schedule.case_acl_expiry_scan`."
- Smallest fix: in item 13 (line 1141) add `case_acl_expiry` to the timer kinds and `schedule.case_acl_expiry_scan` to the settings, or delete the item 13 sentence from the 7.25 block.

## Summary
- Fixes checked: 19 (F1 to F19).
- Findings by severity: HIGH 0, MEDIUM 3, LOW 1 (4 in total).
- CHECKED: F1 ok, F2 ok, F3 ok, F4 R-18-1, F5 ok, F6 ok, F7 ok, F8 R-18-2, R-18-3, F9 ok, F10 ok, F11 R-18-4, F12 ok, F13 ok, F14 ok, F15 ok, F16 ok, F17 ok, F18 ok, F19 ok
