# Owner-decision check, round 4, key 18 (Haiku draft for lead review)

## Summary
- Entries checked: 1 (F1, finding Q-18-1, docs/plan/phase-18.md).
- Findings: HIGH 0, MEDIUM 0, LOW 0.
- Evidence for F1: the replaced clause stands at docs/plan/phase-18.md:1246 (section 6, item 9) and reads as correct English, with balanced code spans. Its two guard messages match the RAISE lines of the trigger word for word (docs/plan/phase-18.md:343 and :347; the row test is at :342). The trigger is FOR EACH ROW (:352-353). Role akr_app holds SELECT and INSERT, and UPDATE on revoked_by and revoked_at only (docs/plan/phase-18.md:427-428), so the permission denied for an UPDATE of access is right. No other GRANT on casework.case_acl exists in docs/plan/phase-*.md, and docs/plan/phase-00.md:181 says no application role is the owner. No other line of phase 18 states the guard message for a row.
- CHECKED: F1 ok
- FINDINGS: 0

