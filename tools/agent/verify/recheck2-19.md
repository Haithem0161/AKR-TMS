# Second re-check, phase 19 (Haiku draft for lead review)

### S-19-1  LOW   (fix F10, [R-19-4b])
- Problem: Section 5.5 (line 1767, F10) lets accounts and contacts through, as well as party names, when they sit inside the verified output of an `assistant_draft` item. Section 3.14 (line 1616, F9) and section 2 (line 993, F11) let only parties of its case through. The same exception now has two scopes in one phase. Phase 28 allows only amount, date and name placeholders in that output and removes any digit outside a span (docs/plan/phase-28.md:480 and :491), so no account number or phone number can reach it. The mismatch is wording, not a build fault.
- Proof: docs/plan/phase-19.md:1767 "no party name, account or contact, except inside the verified output of an `assistant_draft` item"; and docs/plan/phase-19.md:1616 "may name parties of its case, section 7+".
- Smallest fix: In line 1767, replace "except inside the verified output of an `assistant_draft` item" with "except the parties of its case named inside the verified output of an `assistant_draft` item".

## Summary
Entries checked: 11 (F1 to F11).
Findings by severity: HIGH 0, MEDIUM 0, LOW 1 (FINDINGS: 1).
CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok, F8 ok, F9 ok, F10 S-19-1, F11 ok
