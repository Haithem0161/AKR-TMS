# Re-check, phase 03 (Haiku draft for lead review)

### R-03-1  MEDIUM   (fix F5)
- Problem: 7.15 adds three Cedar actions (`dev.echo`, `dev.masked_read`, `dev.detection_emit`) and the policy file `policies/dev.cedar`, but declares none of them: not in the schema, not in `Action::ALL`, and no schema entity exists for the `_dev_masked` resource whose access list the suspicion permit reads. Section 3.1 requires the schema's actions to equal `Action::ALL`, and strict validation refuses a policy that names an undeclared action, so a `dev-routes` build cannot load its bundle or pass the inventory check until these exist.
- Proof: docs/plan/phase-03.md:1386 "cargo feature `dev-routes` (7.13) adds them to `ROUTES` with actions of the namespace `dev.*` and compiles in the" and docs/plan/phase-03.md:645 "3. Inventory: the schema's actions equal `Action::ALL` both ways; every action used by `ROUTES` exists". The access list comes from docs/plan/phase-03.md:1003 "over an in-memory fixture of two records (one `normal` tier, one `suspicion` tier with an".
- Smallest fix: Add one sentence to 7.15: the three actions are declared in `schema.cedarschema` and in `Action::ALL` under `cfg(feature = "dev-routes")`, and `_dev_masked` gets a schema entity with its tier and access list as attributes.

### R-03-2  MEDIUM   (fix F6)
- Problem: 7.12 says the file prints `roles` as a map keyed by role, `{<role>: [<action>, ...]}`. 7.16 says the file prints the output of `roles`, which is a `Vec<RoleActions>` (a list of `{role, actions}` objects, the same shape step 1 gives on line 772). The two shapes differ, so the claim that the file and the endpoint's `roles` part "are therefore equal" fails on any JSON comparison that section 6 is meant to run.
- Proof: docs/plan/phase-03.md:1360 "printing `{bundle_hash, roles: {<role>: [<action>," and docs/plan/phase-03.md:1406 "prints the output of `roles`".
- Smallest fix: In 7.12 replace `roles: {<role>: [<action>, ...]}` with `roles: [{role, actions}]` (the `RoleActions` list), or state in 7.16 that the file keys that list by role.

### R-03-3  MEDIUM   (fix F6)
- Problem: 7.16 makes step 1 of `build` the one derivation of the role part and gives it to `roles`, which returns only `RoleActions`, and says `build` adds the step 2 principals. Step 1 also says its result carries the `sod.json` rules ("also the `sod.json`"), phase 25 takes the `sod.json` rules from `build`, and the output list of step 3 (docs/plan/phase-03.md:776 "`{generated_at, bundle_version, bundle_sha256, roles, principals}`") has no key for them. After 7.16 the sod rules have no producer and no output key. `LoadedBundle.sod` (line 411) is the natural source.
- Proof: docs/plan/phase-03.md:772 "Result: `{role, actions: [action]}`; also the `sod.json`" and docs/plan/phase-25.md:721 "roles to actions and the `sod.json` rules".
- Smallest fix: In 7.16 add that `build` also returns the bundle's `sod.json` rules (`LoadedBundle.sod`) as a `sod` key of step 3's output, or delete "also the `sod.json` rules" from step 1 and name where phase 25 gets them.

### R-03-4  MEDIUM   (fix F7)
- Problem: 7.17 says a `Masked` refusal writes the `deny` access event and that its kind "need not be in `MASKED_KINDS`". Section 3.2 step 8 writes an access event only for "a masked kind and a read action", and section 3.6 steps 2 and 3 (same work as a miss; latency floor) apply only "on a masked kind". 7.17 does not say that it replaces either rule. So a refusal of a kind outside `MASKED_KINDS` writes an access event that a miss does not, and is not held to the floor, which is the timing difference section 3.6 exists to prevent.
- Proof: docs/plan/phase-03.md:711 "Masked read refusals (a masked kind and a read action): also an access event" and docs/plan/phase-03.md:1412 "The kind need not be in `MASKED_KINDS`." The floor rule is docs/plan/phase-03.md:787 "3. Latency floor: a 404 on a masked kind is released no earlier than".
- Smallest fix: In 7.17 add one sentence: step 8's access event and section 3.6 steps 2 and 3 apply to every refusal whose reason is `Masked`, whatever the kind, and this replaces the "masked kind" condition in those steps.

### R-03-5  LOW   (fix F7)
- Problem: The access event of a `Masked` refusal is named as "the `deny` of section 3.6", but section 3.6 does not define the `deny` event. Section 3.2 step 8 and section 3.11 do.
- Proof: docs/plan/phase-03.md:1412 "and the access event is the `deny` of section 3.6." and docs/plan/phase-03.md:994 "The policy layer writes the `deny` and `conflict` access events (section 3.2 step 8)."
- Smallest fix: In 7.17 change "the `deny` of section 3.6" to "the `deny` of section 3.2 step 8".

## Summary
- Fixes checked: 7 (F1 to F7).
- Findings: HIGH 0, MEDIUM 4, LOW 1.
- CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 R-03-1, F6 R-03-2, R-03-3, F7 R-03-4, R-03-5
