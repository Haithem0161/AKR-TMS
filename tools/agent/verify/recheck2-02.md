# Second re-check, phase 02 (Haiku draft for lead review)

## Summary
Entries checked: 7 (F1 to F7). Each NEW text is present in docs/plan/phase-02.md (F1 at lines 1333-1334, F2 at 1306-1308, F3 at 905, F4 at 1061, F5 at 1297-1298, F6 at 1185-1187, F7 at 1289-1292). Each reads as correct English. The names it uses exist in the owning phase (phase 17 grant at line 568, phase 23 section 7.8, phase 24 lines 704 and 1348, phase 30 lines 280 and 407). The counts agree (four columns, three kinds, three rows).
Findings by severity: HIGH 0, MEDIUM 0, LOW 0.
FINDINGS: 0
CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok

Not counted (borderline, not findings; for the lead to decide):
- F7: docs/plan/phase-02.md:978 still reads "SIEM when infected". The 7.13 block says the flag is one boolean per kind, and the EventKind struct at line 550 has `siem: bool`, so 7.13 overrides the row. Optional clean-up: change the row to "SIEM", or add "see 7.13".
- F6: docs/plan/phase-02.md:1187 keeps "-" for `training_record`. docs/plan/phase-24.md:513 gives R9 to the table `admin.training_records` ("Training evidence of staff"), but phase 24 states no class for the owner kind's certificate (docs/plan/phase-24.md:788). The table class is not the owner kind's class, so the dash is read as "states none".
