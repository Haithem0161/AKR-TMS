# Re-check, phase 31 (Haiku draft for lead review)

### R-31-1  HIGH   (fix F21)
- Problem: F21 adds `email_count` to the audit of reconciliation item 3, which runs only for offboarded linked accounts with `directory_active = TRUE`, while `email_count` covers every linked row. A linked, active account whose directory e-mail differs is never counted or audited, although the Conflicts list says such a mismatch is recorded as `directory.mismatch`; F20's pointer "the reconciliation counts the difference (item 3)" rests on the same gap.
- Proof: docs/plan/phase-31.md:683 "3. Linked, account offboarded, `directory_active = TRUE`: audit `directory.mismatch {count, email_count}`" and, conflicting, docs/plan/phase-31.md:692 "Directory e-mail against a linked account: recorded as `directory.mismatch` and not applied." (F20 pointer: docs/plan/phase-31.md:662 "and the reconciliation counts the difference (item 3)").
- Smallest fix: In item 3 of section 3.11 (line 683), drop the "account offboarded" condition from the `email_count` part, so that the job audits `directory.mismatch {count, email_count}` on every run for all linked rows whose e-mail differs, and keep the offboarded `count` under its own trigger.

## Summary
- Fixes checked: 33 (F1 to F33; F3 and F7 are the 7.x blocks, the rest are REPs).
- Findings by severity: HIGH 1, MEDIUM 0, LOW 0.
- CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok, F8 ok, F9 ok, F10 ok, F11 ok, F12 ok, F13 ok, F14 ok, F15 ok, F16 ok, F17 ok, F18 ok, F19 ok, F20 R-31-1, F21 R-31-1, F22 ok, F23 ok, F24 ok, F25 ok, F26 ok, F27 ok, F28 ok, F29 ok, F30 ok, F31 ok, F32 ok, F33 ok
