# Owner-decision round, phase 08 (Haiku draft for lead review)

### O-08-1  HIGH   (section 7.11 items 3 and 4, check 2 against the G10 row)
- Problem: After 202, the add and retire forms link the bank user to the change request on G10. The G10 role list has no A2, and a route the user's roles do not list renders X6. So an A2 proposer gets a dead link and cannot reach the withdraw action that phase 07 requires for a case whose intake failed.
- Proof: docs/plan/phase-08.md:371 "CFG, DS, MRO, CHK, M, CH, ADM, SEC, IA" (the G10 roles, no A2); docs/plan/phase-08.md:719 "on 202 the page links to the change request on G10"; docs/plan/phase-08.md:262 "a route the user's roles do not list renders X6"; docs/plan/phase-07.md:1191 "its maker withdraws the request and the case becomes `rejected`" (G10 offers withdraw to the maker at docs/plan/phase-08.md:411). Also docs/DESIGN-SYSTEM.md:759 "| A1, A2 | My work;" (no change-request entry for A2).
- Smallest fix: in items 3 and 4 replace "on 202 the page links to the change request on G10" and "on 202 the link to the change request" with what A2 sees (the request reference as text, no link), or add A2 to the G10 roles in the row at line 371 and check rule 3 of section 3.5 for it.

### O-08-2  MEDIUM   (section 7.11 item 3, check 3 against phase 07 section 7.6)
- Problem: The form's key rule drops the requirement that a key starts with a lower-case letter. Phase 07 requires it in the column check and in the Prepare step. Prepare answers any pattern failure with 422 case_key_taken, so a key such as 1-ab passes the form and is refused with a misleading message.
- Proof: docs/plan/phase-08.md:719 "the key (lower-case letters, digits and hyphens, 3 to 61 characters)"; docs/plan/phase-07.md:1025 "CHECK (case_key ~ '^[a-z][a-z0-9-]{2,60}$')"; docs/plan/phase-07.md:1183 "`case_key` matches its pattern and no `proposed` or `active` case of the code has it (else 422 `case_key_taken`)".
- Smallest fix: in item 3 replace the key rule with "the key (starts with a lower-case letter, then lower-case letters, digits or hyphens; 3 to 61 characters in all)".

### O-08-3  MEDIUM   (section 7.11 items 2, 3 and 5, check 1 and the i18n rule)
- Problem: The two expected-outcome words, "Would be rejected" and "Accepted", are given in English only and have no row in the words table of item 5. Every other stored value the block shows (states, intake results, origin) has both languages there. The stored values are would_reject and graded, so a developer must invent the Arabic for both.
- Proof: docs/plan/phase-08.md:718 "the expected result in words ("Would be rejected" with its rule codes, or "Accepted")"; docs/plan/phase-08.md:721 "5. **Words** (`src/i18n/locales/{ar,en}/enums.json`):" (rows 725 to 736 hold only corpus_case.state and corpus_case.intake keys); docs/plan/phase-07.md:1026 "CHECK (expect_outcome IN ('would_reject', 'graded'))".
- Smallest fix: add two rows to the item 5 table, `corpus_case.expect.would_reject` ("Would be rejected") and `corpus_case.expect.graded` ("Accepted"), each with its Arabic word.

### O-08-4  LOW   (section 7.9, base paragraph changed in this round, check 1)
- Problem: The replacement pointer in 7.9 is followed by the words "The text of the verification pass was:" and then the whole old paragraph. The old paragraph still says a Latin name's source label is typed and that the card, passport and generated labels are not used. The line starts by pointing to 7.10, so check 2 does not fire, but the leftover words and the overturned rule remain in the phase.
- Proof: docs/plan/phase-08.md:693 "The text of the verification pass was: The `Name` component shows what the source gives."; docs/plan/phase-08.md:693 "the labels `card`, `passport` and `generated` of DESIGN-SYSTEM 7.6 are not used in this plan".
- Smallest fix: delete the words "The text of the verification pass was:" and the two old label clauses (the "its source label is the word `typed`" clause and the "so the labels `card`, `passport` and `generated` ... are not used in this plan" clause), keeping the row rules that 7.10 says stand.

### O-08-5  LOW   (section 7.10 item 2, check 3 against DESIGN-SYSTEM 7.6)
- Problem: "the muted small style of DESIGN-SYSTEM 7.6" names a style that 7.6 does not define; 7.6 says only "smaller". "Muted" occurs in DESIGN-SYSTEM only for the two rail tokens, so the colour is not fixed.
- Proof: docs/plan/phase-08.md:701 "in the muted small style of DESIGN-SYSTEM 7.6"; docs/DESIGN-SYSTEM.md:525 "the Latin transliteration beneath, smaller, labelled with its source".
- Smallest fix: replace "in the muted small style of DESIGN-SYSTEM 7.6" with "smaller, as DESIGN-SYSTEM 7.6 says, in the Stone secondary-text token (`--color-stone`)".

### O-08-6  LOW   (section 7.10 item 4, check 3 against phase 14)
- Problem: The cited phase 14 does not say that its back-transliteration serves matching only or is never shown under a name. Phase 14 says the lexicon is "used for back-transliteration and as a test oracle", and its S1 screen shows transliteration beside each name through the Name component.
- Proof: docs/plan/phase-08.md:711 "Phase 14's back-transliteration serves matching only and is never shown under a name."; docs/plan/phase-14.md:925 "used for back-transliteration and as a test oracle"; docs/plan/phase-14.md:1257 "Customer and list entry side by side, Arabic first with transliteration".
- Smallest fix: delete "and is never shown under a name" unless phase 14 gains a sentence that says so (for example in its S1 row at line 1257), and cite that sentence.

### O-08-7  LOW   (section 7.11 item 3, check 6, field list)
- Problem: The add form lists its fields but not profile_code, which the POST takes with the default IQ. Item 1 reads the panel for the profile code shown, so a case added from another profile would be stored under IQ. Only IQ exists in P0, so the defect is latent.
- Proof: docs/plan/phase-08.md:717 "read with `getGoamlCorpusCases` for the profile code shown"; docs/plan/phase-07.md:1134 "`profile_code?` default `IQ`".
- Smallest fix: add "the profile code shown, sent as `profile_code`" to the field list of item 3.

### O-08-8  LOW   (section 7.11 item 3, check 6, limit source)
- Problem: FileDrop checks size before upload (section 3.7), but item 3 gives no size limit and no source for one. The limit is the setting goaml.corpus_case_max_bytes, and no phase 07 response carries it (it appears only in its settings row and in the Prepare step). E1 gives its limits from the run settings, so the comparable case is stated there.
- Proof: docs/plan/phase-08.md:344 "`FileDrop` checks type and size before upload"; docs/plan/phase-07.md:1156 "at most `goaml.corpus_case_max_bytes` (else 422 `file_too_large`)"; docs/plan/phase-08.md:463 "the limits from the run settings shown".
- Smallest fix: in item 3 name the response that carries goaml.corpus_case_max_bytes (phase 07 would have to add it), or say that the client checks only the type and leaves size to the 422 file_too_large.

### O-08-9  LOW   (section 7.11 item 1, check 1, route pointer)
- Problem: Item 1 puts the panel on the list route /goaml-check/profiles. The version list and the profile code shown belong to the profile page /goaml-check/profiles/$id, which phase 07 section 5 calls the profile page.
- Proof: docs/plan/phase-08.md:717 "E1p (`/goaml-check/profiles`) gains the panel **Bank cases** under the version list"; docs/plan/phase-07.md:897 "**Profile page** (CFG, CHK, M, A2 read): versions with status, source and dates"; docs/plan/phase-08.md:378 gives `/goaml-check/profiles/$id` as the detail route.
- Smallest fix: write `/goaml-check/profiles/$id` in place of `/goaml-check/profiles` in item 1.

## Summary

Blocks checked: 7.10 and 7.11 (new). Changed base places: line 472 (the E1p paragraph pointer; correct, section 7.11 exists and names the panel) and line 693 (the 7.9 paragraph; see O-08-4). Also checked against the new text: the Name row (line 338), the DataTable row (335), the FileDrop row (344), the E1, E1p and G10 rows (377, 378, 371) and the i18n rule (line 180 to 186, and line 360). Phase 08 has no "## Decisions" or "## Open doubts" heading.

Findings: HIGH 1, MEDIUM 2, LOW 6 (total 9).

- 7.9 (base, line 693): O-08-4 (check 1). Its "typed" sentence points to 7.10 on the same line, so no finding on the extra "always typed" check. No other line of phase 08 says a Latin name is always typed without pointing to 7.10.
- 7.10: O-08-5 and O-08-6 (check 3); checks 1, 2, 5 and 6 ok.
- 7.11: O-08-1 (check 2), O-08-2 (check 3), O-08-3 (check 1 and i18n), O-08-7 and O-08-8 (check 6), O-08-9 (check 1).

Checked and ok: the three routes, their scopes and Cedar roles (GET for A2, M, CFG and CHK; both POSTs for A2 and M); the operation names getGoamlCorpusCases, postGoamlCorpusCases and postGoamlCorpusCasesIdRetire follow the routes and the hooks of phase 07 section 7.6 item 13; all six 422 reasons the form names are defined in phase 07 section 7.6 item 5; the four states and the eight intake words (pending, met, not met and five refusals) match phase 07 section 7.6 items 4 and 5, so the 12-row table is complete; the four labels and their default typed match phase 10 section 7.26 and DESIGN-SYSTEM 7.6; bank_cases and the bank/ entries match phase 07 section 7.6 item 8; both POSTs answer 202 as the form says; the state filter and the no-file rule match the GET route. The notice text and the quoted English labels in 7.11 are not findings: the base text quotes English labels the same way and keys them for translation.
