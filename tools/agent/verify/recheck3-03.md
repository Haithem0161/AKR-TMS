# Third re-check, phase 03 (Haiku draft for lead review)

### T-03-1  HIGH   (fix F2, [S-03-1])
- Problem: The replacement says step 3 of section 3.1 accepts `@role("*")` on the `dev.*` permits "as it does for the `self` group", but step 3 allows `"*"` only for the `self` group, and no line of the phase puts the `dev.*` actions in that group. As written, step 3 and the replacement cannot both hold, so the stated inventory check fails in a `dev-routes` build.
- Proof: docs/plan/phase-03.md:646 "carries `@role("<code>")` with a code from the nineteen (or `"*"` for the `self` group, which yields no" and docs/plan/phase-03.md:1394 "accepts for the `dev.*` actions as it does for the `self` group (no `actor_role`)".
- Smallest fix: In block 7.15, replace "which step 3 of section 3.1 accepts for the `dev.*` actions as it does for the `self` group (no `actor_role`)" (docs/plan/phase-03.md:1393-1394) by "and the `dev.*` actions are members of the `self` group (declared `in ["self"]` in `dev.cedarschema`), which step 3 of section 3.1 accepts (no `actor_role`)".

## Summary
Entries checked: 3 (F1, F2, F3). Findings: HIGH 1, MEDIUM 0, LOW 0.
CHECKED: F1 ok, F2 T-03-1, F3 ok
