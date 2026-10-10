# Re-check, phase 08 (Haiku draft for lead review)

### R-08-1  MEDIUM   (fix F2)
- Problem: The new 7.6 block says G10 shows the reason tag "beside the reason code, on the list". The G10 list has no reason-code column (its columns are reference, kind word, object, class and volume-effect badges, maker, status and age). The reason code appears on the detail only, so where the tag goes on the list is left undefined.
- Proof: docs/plan/phase-08.md:407 "with columns reference, kind word, object, class and volume-effect badges with words," and docs/plan/phase-08.md:409 "reason code with label and text" (detail only).
- Smallest fix: In the 7.6 block, replace "beside the reason code, on the list and on the detail" with "beside the reason code on the detail and beside the class badge on the list", or add a reason-code column to the G10 list columns at line 407.

### R-08-2  LOW   (fix F6)
- Problem: "The dialog asks for a new code" is right only for the TOTP field. The same dialog also offers WebAuthn, which has no code, so a refused WebAuthn proof needs a fresh security-key touch, not a code.
- Proof: docs/plan/phase-08.md:203 "A refused proof on `POST /auth/step-up/verify` is not resent: the dialog asks for a new code." and docs/plan/phase-08.md:310 "then WebAuthn (`navigator.credentials.get` with the returned options) or the TOTP field".
- Smallest fix: In line 203, change "the dialog asks for a new code" to "the dialog asks for a new proof (a new TOTP code, or a fresh security-key touch)".

### R-08-3  MEDIUM   (fix F9)
- Problem: The new sentence says this phase writes `shortcuts` and `theme` through PATCH /me/preferences. The X4 paragraph says the profile writes language, density and prose digits and hides the theme choice while DS-O1 is open, and line 355 says the shortcut layer is P1. The two statements of what the phase writes cannot both hold.
- Proof: docs/plan/phase-08.md:49-50 "The preferences it writes (`density`, `prose_digits`, `shortcuts`, `theme`) are the phase 01 `PreferencesV1` fields through `PATCH /me/preferences`." and docs/plan/phase-08.md:388-389 "Language, density and prose digits (`PATCH /me/preferences`, `If-Match`); the theme choice is hidden while DS-O1 is open."
- Smallest fix: In line 50, change "(`density`, `prose_digits`, `shortcuts`, `theme`)" to "(`density`, `prose_digits`)", which matches line 388 and leaves language to the sentence that follows.

### R-08-4  LOW   (fix F12)
- Problem: Section 7.2 (base text) says RailGroup has "one value per rail entry". The new 7.8 block maps a second CH rail entry, board pack, to the existing value `reports` and adds no value, so one value covers two entries. The block does not say it replaces the 7.2 sentence, so the base text still stands.
- Proof: docs/plan/phase-08.md:651 "`RailGroup` is a closed union, one value per rail entry of DESIGN-SYSTEM 8.2" and docs/plan/phase-08.md:689 "It takes `RailGroup` `reports`, the group of the R series".
- Smallest fix: In line 651, change "one value per rail entry of DESIGN-SYSTEM 8.2" to "one value per rail group of DESIGN-SYSTEM 8.2 (several entries may share a group, see section 7.8)".

### R-08-5  MEDIUM   (fix F13)
- Problem: The new 7.9 block says the transliteration's source label "is the label of that row's source_field (the mapped source column, phase 10)". Phase 10 gives source_field only as a bare TEXT column and defines no label for any mapped column; its uses of "label" are for settings, code lists and code maps. The words the component shows (card, passport, typed, generated, per DESIGN-SYSTEM 7.6) therefore have no source in the plan, and a developer must guess the mapping.
- Proof: docs/plan/phase-08.md:692 "its source label is the label of that row's `source_field` (the mapped source column, phase 10)" and docs/plan/phase-10.md:505 "source_field        TEXT NOT NULL,".
- Smallest fix: In the 7.9 block, replace "its source label is the label of that row's `source_field` (the mapped source column, phase 10)" with a rule that gives the label word for each source (for example, `typed` for every CBS-sourced row), or name a phase 10 column that holds the label and add it to the mapping.

## Summary
- Fixes checked: 13 (F1 to F13).
- Findings: HIGH 0, MEDIUM 3 (R-08-1, R-08-3, R-08-5), LOW 2 (R-08-2, R-08-4).
CHECKED: F1 ok, F2 R-08-1, F3 ok, F4 ok, F5 ok, F6 R-08-2, F7 ok, F8 ok, F9 R-08-3, F10 ok, F11 ok, F12 R-08-4, F13 R-08-5
