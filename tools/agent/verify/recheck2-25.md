# Second re-check, phase 25 (Haiku draft for lead review)

### S-25-1  MEDIUM   (fix F14 and F15, [R-25-10b, R-25-10c])
- Problem: The grep for route rows returns 23 lines (lines 670 to 691 and line 1483), while F14 and F15 state 22. The 23rd line repeats the evidence upload route of section 2 (line 684) inside section 7.22, which F12 rewrote, so the file lists one route twice, although the 22 section 2 rows are 22 distinct routes.
- Proof: docs/plan/phase-25.md:694 "- **Route count**: 22 rows, of which 12 are owned (API.md 3.2, 3.3, 3.20 and the help row) and 10 are added"; docs/plan/phase-25.md:684 "| POST | `/api/v1/admin/releases/{id}/evidence` | `admin.system` | Upload the `rollout.json` of the apply as evidence of owner kind `release`;"; docs/plan/phase-25.md:1483 "| POST | `/api/v1/admin/releases/{id}/evidence` | `admin.system` | Upload the `rollout.json` of the apply (multipart,"
- Smallest fix: Replace the table line at docs/plan/phase-25.md:1483 by a sentence that points to the section 2 row at line 684, and move its upload rules (multipart, `EvidenceStore::stage`, declared content type `text/plain`, kind `package`, retention R10, classification `internal`) into that section 2 row, so that the grep count is 22.

## Summary
Entries checked: 15 (F1 to F15).
Findings by severity: HIGH 0, MEDIUM 1, LOW 0.
CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok, F8 ok, F9 ok, F10 ok, F11 ok, F12 ok, F13 ok, F14 S-25-1, F15 S-25-1
