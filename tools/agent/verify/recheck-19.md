# Re-check, phase 19 (Haiku draft for lead review)

### R-19-1  MEDIUM   (fix F5)
- Problem: The route table of section 2 now has 32 rows (19 owned, 13 New), and F5 calls the reopen-candidates route the thirty-second, but the count sentence, the 4.5 item 16 count and section 6 check 7 still say 31 routes with 12 added. DR-157 (line 1999) also still says 31 routes, so the DR-157 citation in 7.14 item 4 is wrong too.
- Proof: docs/plan/phase-19.md:1053 "Count: 31 product routes (19 owned rows of API.md 3.12, 12 added rows)." and docs/plan/phase-19.md:1831 "the OpenAPI document lists the 31 paths of section 2 (the 19 owned and the 12 added)".
- Smallest fix: Change the count at line 1053 to 32 product routes with 13 added rows, change "12 added rows" at line 1727 and "the 12 added" at line 1831 to 13, change "31 paths" at line 1831 to 32, and cite the reopen route's own decision in 7.14 item 4 instead of DR-157.

### R-19-2  MEDIUM   (fix F5)
- Problem: 7.14 item 1 says the sub-code `not_critical_finding` joins the closed sub-code list of section 3.11, but that list still ends at `rfi_open` without it, so the closed list is incomplete.
- Proof: docs/plan/phase-19.md:1945 "sub-code `not_critical_finding`, which joins the closed list of section 3.11." and docs/plan/phase-19.md:1339 "`not_bulk_case`, `no_linked_case`, `role_not_permitted`, `not_on_case`, `unknown_party`, `forbidden_term`, `rfi_open`."
- Smallest fix: Append `not_critical_finding` after `rfi_open` in the closed sub-code list of section 3.11 at line 1339.

### R-19-3  MEDIUM   (fix F5)
- Problem: Section 5.5 says the bulk-reopen dialog lists at most 25 cases, while 7.14 item 4 makes the route that feeds the dialog return up to 100 rows, so a developer cannot tell which limit the dialog shows.
- Proof: docs/plan/phase-19.md:1771 "a dialog lists the cases closed by the analyst in the window (at most 25)" and docs/plan/phase-19.md:1948 "returns at most 100 rows of `ClosedCaseRow`, with no total."
- Smallest fix: Replace "(at most 25)" at line 1771 with "(up to 100 candidates from the route, of which at most 25 may be selected)".

### R-19-4  MEDIUM   (fix F13)
- Problem: F13 and F14 let the QA item show the verified output of an assistant_draft item, and that output can name parties (phase 28 section 3.3 step 2), but the view rule at line 993, the 3.14 row and the 5.5 item panel still say no party name is shown. The reviewer holds a time-boxed case grant, so outsiders are not exposed, but the stated rules cannot all hold.
- Proof: docs/plan/phase-19.md:993 "no party name, no account, no text except the decision text for an analyst network reader and, from phase 28 on, the verified output of an `assistant_draft` item"; docs/plan/phase-19.md:1616 "never a party name, an account or a contact"; docs/plan/phase-19.md:1767 "no party name, account or contact"; docs/plan/phase-28.md:490 "`{name:Rk.f}` becomes the record's name in the sentence's language".
- Smallest fix: Add "except names inside the verified output of an assistant_draft item (section 7+)" to the no-party-name clause at line 993 and to the rows at lines 1616 and 1767.

## Summary
- Fixes checked: 23 (F1 to F23 of applied-19.txt).
- Findings by severity: HIGH 0, MEDIUM 4, LOW 0.
- CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 R-19-1 R-19-2 R-19-3, F6 ok, F7 ok, F8 ok, F9 ok, F10 ok, F11 ok, F12 ok, F13 R-19-4, F14 R-19-4, F15 ok, F16 ok, F17 ok, F18 ok, F19 ok, F20 ok, F21 ok, F22 ok, F23 ok.
