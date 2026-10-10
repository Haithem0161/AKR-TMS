# Re-check, phase 01 (Haiku draft for lead review)

### R-01-1  MEDIUM   (fix F8)
- Problem: The step 1 insert lists four columns and omits display_name, which is NOT NULL with no default and must carry both 'ar' and 'en' keys (the system principal insert on line 99 sets it). The block also never uses `--name` in create, so `client rotate --name` and `client disable --name` have no stored value to find the principal row by; the implementer has to guess whether it is display_name, hydra_client_id or a Hydra client name.
- Proof: docs/plan/phase-01.md:1119 "Then in one transaction insert the `admin.users` row (`kind = 'service'`," and docs/plan/phase-01.md:57 "JSONB NOT NULL CHECK (display_name ? 'ar' AND display_name ? 'en')" and docs/plan/phase-01.md:1123 "`client rotate --name`: Hydra replaces the client's secret with a new generated one."
- Smallest fix: In step 1 add `display_name` (both keys set to the `--name` value) to the insert, and state in step 2 and step 3 that the principal is found by that value (or by `hydra_client_id`).

### R-01-2  MEDIUM   (fix F9)
- Problem: The fix says the pinned list covers every route "but two" and names heartbeat and logout as those two. The same section says the two routes with no action are the public ones (login and second factor). The two statements give different sets of uncovered routes for the same list.
- Proof: docs/plan/phase-01.md:1132 "The pinned list of section 3 covers every route of this phase but two." and docs/plan/phase-01.md:862 "The two public routes have no action."
- Smallest fix: Replace "covers every route of this phase but two." with "covers every route of this phase except the two public routes (`POST /auth/login`, `POST /auth/mfa`), which have no action."

## Summary
- Fixes checked: 12 (F1 to F7 are REP, F8 to F12 are BLOCK).
- Findings: HIGH 0, MEDIUM 2, LOW 0.
- CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok, F8 R-01-1, F9 R-01-2, F10 ok, F11 ok, F12 ok
