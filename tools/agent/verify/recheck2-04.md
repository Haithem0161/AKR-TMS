# Second re-check, phase 04 (Haiku draft for lead review)

### S-04-1  MEDIUM   (fix F7, [R-07-1b])
- Problem: The new total of 32 operations is right, but the tag counts in the same OpenAPI bullet add up to 31 (8 + 10 + 8 + 3 + 1 + 1), so the evidence route is in no tag. The evidence route has the same scope list as the change-control submit route, so `change-control` should count 9, and the Routers bullet still says 8.
- Proof: docs/plan/phase-04.md:1501 "tags `change-control` (8), `inbox` (10), `workflow-admin` (8, including reassign)," and docs/plan/phase-04.md:1503 "32 operations (the 31 of section 2 and the evidence route of section 7.2), each with `x-akr-action`". Lines 781 (submit) and 1659 (evidence) both carry the scope "config.author, admin.users, admin.policies, admin.system". The Routers bullet is docs/plan/phase-04.md:812 "`api::routes::change_control::router()` (8 + the three phase 03 CR branches)".
- Smallest fix: In the OpenAPI bullet at line 1501 change `change-control` (8) to (9) and count the evidence route in it, and change the Routers bullet at line 812 to `change_control::router()` (9 + the three phase 03 CR branches); or name another tag for the evidence route and its count.

## Summary
- Entries checked: 9 (F1 to F9).
- Findings by severity: HIGH 0, MEDIUM 1, LOW 0. FINDINGS: 1.
- CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 S-04-1, F8 ok, F9 ok
