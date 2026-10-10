# Re-check, phase 29 (Haiku draft for lead review)

### R-29-1  LOW   (fix F1)
- Problem: F1 says every later change of a hold has its kind in the table below, but F16 says the alert link set by `HoldRepository::set_alert` needs no kind of its own, and no row of the section 3.17 table covers it. Both statements cannot be read literally.
- Proof: docs/plan/phase-29.md:1180 "every later change of a hold has its kind below" and docs/plan/phase-29.md:1017 "The link needs no kind of its own".
- Smallest fix: in F1 change "every later change of a hold has its kind below" to "every later change of a hold's state has its kind below, and the alert link is audited by phase 14's `screening.alert_created` (section 3.5 step 2)".

### R-29-2  MEDIUM   (fix F2)
- Problem: F2 lets a rail be assessed as soon as its first mode is activated, but it requires no fail-policy row for the rail. Step 7 of section 3.1 needs the most specific row, the signed decision carries `fail_policy`, and no text says what happens when no row matches. The wildcard rule of F28 covers only matrix proposals.
- Proof: docs/plan/phase-29.md:1510 "Until that activation the rail is not assessed" and docs/plan/phase-29.md:922 "the most specific fail policy row for the rail, corridor, currency and amount band" (the signed object at docs/plan/phase-29.md:977 lists `"fail_policy"`).
- Smallest fix: add to F2 that the first mode proposal is refused with 422 `E_VALIDATION` (`errors[].code = wildcard_row_missing`) unless the active matrix already has the rail's `*`, `*`, `*` row.

### R-29-3  LOW   (fix F2)
- Problem: F2 adds a kind to the table of section 3.17, but section 4.5 item 2 still says 17 event kinds. The table has 18 rows before F2 and 19 after it.
- Proof: docs/plan/phase-29.md:1350 "17 event kinds of section 3.17 added to the catalogue" and docs/plan/phase-29.md:1509 "a new kind in the table of section 3.17".
- Smallest fix: in section 4.5 item 2 change "17 event kinds" to "19 event kinds".

### R-29-4  LOW   (fix F4)
- Problem: F4 says the PUT is refused "as API section 5 says", but docs/API.md section 5 is the OpenAPI and codegen flow and does not state that refusal. The refusal is in the docs/API.md section 3.19 row for PUT /assess/modes/{rail}.
- Proof: docs/plan/phase-29.md:1051 "so the PUT is refused as API section 5 says" and docs/API.md:801 "refused when the entry criteria fail".
- Smallest fix: in F4 change "as API section 5 says" to "as API section 3.19 says".

### R-29-5  HIGH   (fix F6)
- Problem: F6 finds the bands a decision used by its `policy_version`, which section 3.1 step 7 calls the global `set_version` and section 1.2 bumps on every mode and rule activation too. Section 3.6 numbers each matrix and band set `previous + 1`, a separate counter, so the two cannot both hold after a mode change: policy set 1 at global 1, a mode change to global 2, a decision stored with `policy_version` 2, then the next policy activation creates set 2, and a later reconstruction of that decision picks set 2, which it did not use.
- Proof: docs/plan/phase-29.md:1513 "the bands a decision used are the rows with the greatest `set_version` at or below its `policy_version`" and docs/plan/phase-29.md:1027 "inserts the new rows with `set_version = previous + 1`" (the global counter is named at docs/plan/phase-29.md:922 "the global `set_version` (stored as `policy_version`)").
- Smallest fix: in section 3.6 change "`set_version = previous + 1`" to "`set_version` equal to `policy_state.set_version` after the activation's increment (section 3.11)".

## Summary
- Fixes checked: 34 (F1 to F34).
- Findings by severity: HIGH 1, MEDIUM 1, LOW 3.
- CHECKED: F1 R-29-1, F2 R-29-2 R-29-3, F3 ok, F4 R-29-4, F5 ok, F6 R-29-5, F7 ok, F8 ok, F9 ok, F10 ok, F11 ok, F12 ok, F13 ok, F14 ok, F15 ok, F16 ok, F17 ok, F18 ok, F19 ok, F20 ok, F21 ok, F22 ok, F23 ok, F24 ok, F25 ok, F26 ok, F27 ok, F28 ok, F29 ok, F30 ok, F31 ok, F32 ok, F33 ok, F34 ok.

