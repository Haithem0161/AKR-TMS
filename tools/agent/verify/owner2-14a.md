# Owner-decision check, round 2, key 14a (Haiku draft for lead review)

## Checks run
- F1 (O-14-1): route rows in docs/plan/phase-14.md = 27 (`grep -c` on the route pattern): 25 in section 2.8 (lines 818 to 842) and 2 in section 7.19 (lines 1535, 1536). Tag `lists` = 8 rows (lines 818 to 825); tag `screening` = 17 + 2 = 19. No line still says 25 operations for the whole phase.
- F2 (O-14-2): CREATE TABLE count = 10 in section 1 (lines 65, 92, 139, 166, 223, 251, 302, 326, 356, 391) plus alert.harness_cases (line 1457) = 11. Section 7.19 gives 14 classification rows (lines 1505 to 1517: 12 internal, 2 personal, covering all 14 columns) and one retention row (line 1520, clock `closure`, which phase 00 line 307 allows).
- F3 (O-14-3): section 1.5 ungoverned keys (lines 529 to 556) match section 4.3 once `screening.harness_cases_max` is added; the key is defined at line 1524 with governed FALSE.
- F4 (O-14-4): four targets = list_setting, screening_policy, good_guy_rule (section 3.10) plus harness_case (section 7.19 item 5). Phase 04 has an open registry (phase-04.md line 123), so phase 14 registering the fourth target is consistent.
- F5, F6 (O-14-4): section 1 has ten tables (see F2); crate tree lines 655 and 656 read as the replacements state.
- F7 (O-14-4): HarnessCaseService and HarnessCaseTarget are introduced in section 7.19 item 4 (line 1557); line 806 matches.
- F8 (O-14-5): section 7.19 item 3 (line 1540) holds `CaseQuery.identifiers: Option<IdentifierInput>`. The other "never stored" texts (lines 682 and 829) concern a list record and the search query text, not identifiers.
- F9 (O-14-6): no `unmet_ids` left in the file; `results` appears in item 6 (line 1588); the generated hook named in section 5 (line 1263) exists.
- F10 (O-14-6): `useGetScreeningHarnessRuns` (line 1263), `trial.met` (trial JSON, line 1464) and the `gate` and `oracle` run kinds (section 3.9) all exist.
- F11 (O-14-6): docs/API.md line 96 gives `config.approve` to CHK, M, CH and MRO; the S3 reader list (line 1262) includes CH.
- F12 (L-4): scorer rule 9 (line 981) gives Hit, BelowLine, NeedsMoreData and Clear, which match the snake-case list; item 4 step 3 (line 1562) refuses a key that the active version lacks; no other trial text says `absent`.
- F13 (O-14-7): item 4 step 3 needs an active `iq_national` version; the Lists bullet (line 1282) uploads and activates one before the curl step; the body fields (`list_code`, `entry_key`, `name`, `expect`) match the proposal DTO (line 1540).

## Summary
Entries checked: 13 (F1 to F13).
Findings by severity: HIGH 0, MEDIUM 0, LOW 0.
FINDINGS: 0
CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok, F8 ok, F9 ok, F10 ok, F11 ok, F12 ok, F13 ok
