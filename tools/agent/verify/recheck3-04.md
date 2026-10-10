# Third re-check, phase 04 (Haiku draft for lead review)

### T-04-1  MEDIUM   (fix F1, [S-04-1])
- Problem: The OpenAPI registration bullet still gives the `change-control` tag a count of 8, but the replacement says the evidence route is a ninth operation of that tag. The listed tag counts (8, 10, 8, 3, 1, 1) still sum to 31 against the stated total of 32, which is the fault the verdict named.
- Proof: docs/plan/phase-04.md:1501 "tags `change-control` (8), `inbox` (10), `workflow-admin` (8, including reassign)," and docs/plan/phase-04.md:1503 "a ninth operation of the tag `change-control`, mounted in `change_control::router()`".
- Smallest fix: On line 1501 change "tags `change-control` (8)" to "tags `change-control` (9, the eight of section 2 and the evidence route)".

## Summary
- Entries checked: 2 (F1, F2)
- Findings by severity: HIGH 0, MEDIUM 1, LOW 0
CHECKED: F1 T-04-1, F2 ok
