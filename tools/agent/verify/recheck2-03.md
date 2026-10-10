# Second re-check, phase 03 (Haiku draft for lead review)

### S-03-1  HIGH   (fix F1, [R-03-1])
- Problem: The new sentence "So the inventory of section 3.1 step 3 holds in both builds" is false for the dev-routes build. Step 3 requires an `@role` code from the nineteen on every permit, section 6 refuses a permit without one, and the dev permits of the 7.15 table name no role, so `loader::load` refuses the dev bundle.
- Proof: docs/plan/phase-03.md:646 "carries `@role("<code>")` with a code from the nineteen (or `"*"` for the `self` group, which yields no"; docs/plan/phase-03.md:1185 "a permit without `@role`, an action missing from the schema"; docs/plan/phase-03.md:1397 "| `POST /api/v1/_echo` (phase 00) | `dev.echo` | System | every active principal |"; docs/plan/phase-03.md:1393 "So the inventory of section 3.1 step 3 holds in both builds."
- Smallest fix: Replace the sentence "So the inventory of section 3.1 step 3 holds in both builds." (line 1393) by one that gives every `dev.cedar` permit a named `@role` code, or exempts `dev.cedar` from the `@role` rule of step 3.

### S-03-2  HIGH   (fix F1, [R-03-1])
- Problem: In a dev-routes build run with `AKR_ENV = production`, the three actions are members of `Action::ALL` (the feature alone decides that), but the fragment is not appended (7.15 ties the append to `AKR_ENV != production`). Step 3 then finds three actions in `Action::ALL` that the schema lacks, the bundle is refused and the instance is not ready, so "holds in both builds" is false for that build.
- Proof: docs/plan/phase-03.md:1388 "which `loader::load` appends to the active bundle only when the feature is on and" with docs/plan/phase-03.md:1389 "`AKR_ENV != production`."; docs/plan/phase-03.md:1390 "With the feature on, the three actions are members of `Action::ALL`"; docs/plan/phase-03.md:645 "Inventory: the schema's actions equal `Action::ALL` both ways;"; docs/plan/phase-03.md:1368 "in any build is mounted only when `AKR_ENV != production`."
- Smallest fix: In 7.15 change "appends to the active bundle only when the feature is on and `AKR_ENV != production`" to "appends to the active bundle whenever the feature is on (`AKR_ENV` controls only the mount of 7.13)".

### S-03-3  MEDIUM   (fix F1, [R-03-1])
- Problem: Section 2 still says a test compares the shipped `.cedarschema` text with `Action::ALL`. The replacement declares the three dev actions in the fragment `policies/dev.cedarschema`, not in the shipped text, so that test fails in a dev-routes build, and nothing says it now reads the loaded schema.
- Proof: docs/plan/phase-03.md:399 "`schema`: the shipped `.cedarschema` text and its parsed form; a test asserts that the schema's actions equal"; docs/plan/phase-03.md:400 "`Action::ALL` both ways."; docs/plan/phase-03.md:1391 "are declared in `policies/dev.cedarschema`, a schema fragment that"
- Smallest fix: In section 2 (line 399) change "the shipped `.cedarschema` text and its parsed form" to "the loaded schema (the shipped text plus the fragments that `loader::load` appends under the feature, 7.15) and its parsed form".

### S-03-4  MEDIUM   (fix F2, [R-03-2])
- Problem: The new list shape of `akr-ctl policy matrix` contradicts phase 08 section 2, which describes the same command with the same derivation (phase 03's `AccessMatrixBuilder`) but prints `roles` as a map keyed by role, so the two phases now disagree on one output.
- Proof: docs/plan/phase-03.md:1361 "printing `{bundle_hash, roles: [{role, actions}]}`"; docs/plan/phase-08.md:70 "`{bundle_hash, roles: {<role>: [<action>, ...]}}` from the permits' `@role` annotations evaluated against"
- Smallest fix: In docs/plan/phase-08.md line 70 change `{bundle_hash, roles: {<role>: [<action>, ...]}}` to `{bundle_hash, roles: [{role, actions}]}` (the `RoleActions` list of phase 03 7.16).

### S-03-5  MEDIUM   (fix F4, [R-03-4])
- Problem: The new sentence says a miss and a refusal on these kinds "cannot be told apart by an event", but the section it cites (3.2 step 8) writes an access event for a masked read refusal and none for a missing record. For a read, the event therefore separates the two cases.
- Proof: docs/plan/phase-03.md:711 "8. Masked read refusals (a masked kind and a read action): also an access event (`deny`, or `conflict`)"; docs/plan/phase-03.md:713 "no access event (there is no record to name)."; docs/plan/phase-03.md:1416 "so a miss and a refusal on them cannot be told apart by an event or by timing."
- Smallest fix: In 7.17 (line 1416) change "cannot be told apart by an event or by timing" to "cannot be told apart by timing", since section 3.2 step 8 writes the access event for a refusal and none for a miss.

## Summary
- Entries checked: 5 (F1 to F5).
- Findings: HIGH 2, MEDIUM 3, LOW 0 (5 in total).
- CHECKED: F1 S-03-1, S-03-2, S-03-3, F2 S-03-4, F3 ok, F4 S-03-5, F5 ok
