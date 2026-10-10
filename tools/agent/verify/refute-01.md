# Refuter, phase 01 (Haiku draft for lead review)

### V-01-01-a  PARTLY   (first agent: LOW; refuter: LOW)
- Claim: the Goal line says the session minima are floors, but the body and DR-34 make session limits hard bounds (T7).
- Evidence: the design part is settled. research.md:45 (DR-34): "`floor` stays for regulatory minima with their basis". phase-01.md:322-323: "Product limits are hard bounds in the settings registry, not floors (DR-34)." The session keys carry floor NULL (phase-01.md:357, :359). research/30-security-privacy.md:339 makes the CBI session time-out "per policy" and "bank-configurable", so there is no CBI session minimum for a floor to carry. The wording part stands: the Goal sentence at phase-01.md:8 reads "the CBI password, lockout and session minima as floors". SP-14 (research/30-security-privacy.md:1387) also says "session minima a floor", so the header copies the requirement that DR-34 later narrowed.
- Fix: phase 01, Goal line in the header (phase-01.md:8): name the password and lockout minima as the floors and say that session limits are hard bounds under DR-34.

### V-01-02-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `identity client create`, `client rotate` and `client disable` have no numbered steps, write order, transaction or compensation (T4).
- Evidence: the create part stands. phase-01.md:794 "the Hydra client and the service principal row; prints the secret once." names two writes and no order, transaction or failure path. The bootstrap bullet has all of these (phase-01.md:785-787: "in one transaction the mirror row ... a failed transaction deletes the Kratos identity"). `grep "client create"` over docs/plan hits phase-01.md only. The disable part is refuted in part: phase-03.md:897-898 "disable every service principal the user owns (`admin`), revoking their Hydra clients after commit" and phase-03.md:904 "3. After commit: ... Hydra client revocation for disabled service principals" give the order and the failure path for the same act: phase-03.md:1053 lists `service_client.disabled` as "(phase 01, now also from offboarding)", one audit kind for both. The CLI bullet itself stays unspecified. The rotate part stands: phase-01.md:795 names `identity client rotate` and nothing else in docs/plan says what it replaces; the new secret can be inferred from "prints the secret once" (phase-01.md:794), so MEDIUM, but the old secret's fate is written nowhere and the lead may rate it HIGH.
- Fix: phase 01, section 3, the `akr-ctl identity` bullets (phase-01.md:793-796): give `client create` the write order, transaction and failure path that the bootstrap bullet has, say what `client rotate` replaces, and point `client disable` to the phase 03 offboarding order (phase-03.md:897-904).

### V-01-04-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `POST /auth/session/heartbeat` and `POST /auth/logout` have no Cedar action (T2).
- Evidence: the pinned list says "The two public routes have no action" (phase-01.md:862), and the two public routes are login and mfa (phase-01.md:561), so these two session routes are not exempt. The pinned names (phase-01.md:858-862) are 14 and contain neither heartbeat nor logout. phase-03.md:995-997 repeats the same 14 for the retrofit and omits both. phase-03.md:485-486 says "every route has the Cedar action", and the coverage test at phase-03.md:1187-1188 requires "every operation ... has a `RouteAction`". `grep -i "heartbeat\|logout"` over docs/plan hits no action name, and phase-03 has no text on either route. Nearest candidates, not settling: `session.revoke` is pinned for `DELETE /me/sessions/{id}` (phase-01.md:527, "the current one = logout"), but `POST /auth/logout` is its own operation with audit `auth.logout` (phase-01.md:656), not `session.revoked`.
- Fix: phase 01 section 3 "Cedar actions" paragraph (phase-01.md:858-862): add one pinned name for each of the two routes, and repeat both names in the phase 03 retrofit list (phase-03.md:995-997).

### V-01-05-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the `pat.max_per_user` count runs in step 2, outside the step 3 transaction and under no lock, so two concurrent creates can exceed the limit (T4).
- Evidence: phase-01.md:754 "fewer than `pat.max_per_user` active tokens -> else 422 (`limit_reached`)" is a validation check in step 2; phase-01.md:756 "3. Transaction: `consume` (`token.create`); insert the row" holds no count. The only row lock in the section is on the step-up assertion (phase-01.md:737 "The row lock settles concurrent consumers: exactly one wins"), which serialises requests that use the same assertion, not two creates with two different assertions. No general rule closes it. phase-01.md:758 "The route is outside the idempotency" ledger, so the idempotency serialisation does not apply. Phase 00 takes an advisory lock only for job kinds (phase-00.md:851). Phase 17 locks a user's work count (phase-17.md:1130, `pg_advisory_xact_lock(hashtext('wip:' || user_id))`), a precedent for alert claims that phases 00, 02 and 03 do not carry. `grep -n "FOR UPDATE\|advisory\|lock"` in phase-01.md finds no lock on tokens or on the owner row.
- Fix: phase 01 section 3, `TokenService::create` steps 2-3 (phase-01.md:752-756): move the active-token count inside the step 3 transaction under a lock on the owner's row, as the phase 17 claim does (phase-17.md:1130).

### V-01-07-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the sign-in anomaly rules that the SP-18 fields serve are named in phase 25 as phase 24's SP-36 pack, but phase 24 and the plan do not define SP-36 (T6).
- Evidence: phase-25.md:13 "the anomaly rules themselves are phase 24's SP-36 pack". `grep -n "SP-36" docs/plan/phase-*.md` hits phase-25.md:13 and phase-25.md:1180 only, and phase-25.md:1180 cites SP-36 only for the SOC seeing refusals. phase-24.md has no SP-36 and no anomaly text. The mapping is plausible, because SP-36 lists "off-hours or off-network access" (research/30-security-privacy.md:1409), which overlaps the sign-in rules of SP-18 (research/30-security-privacy.md:1391, "Send sign-in anomaly rules ... to the SIEM"). But no phase carries SP-36. `grep -n -i "unusual hour\|workstation\|anomal" docs/plan/*.md`, leaving out outcome names such as "source anomaly", finds the sign-in rules only at phase-01.md:18 and :855 (what the fields are for) and phase-25.md:13 (the pointer), so no phase defines them. The roadmap puts SP-26 to SP-36 at P0-P1 (docs/ROADMAP.md:333).
- Fix: lead decision on the owner. Phase 24, header Requirements carried: add SP-36 (or record in its Open doubts where it moves), then make phase-25.md:13 and the phase 01 header (phase-01.md:18-19) name that owner.

### V-01-08-a  REFUTED   (first agent: LOW; refuter: none)
- Claim: 7.1 step 6 cites "phase 03 section 4.5", which is not a heading; the consistency check is an unnumbered bullet in phase 03 section 4 (T6).
- Evidence: the thing exists in the cited phase. phase-03.md:1096 "## 4. Infrastructure Updates" has no "### 4.x" subheadings (`grep -c "^### 4\."` gives 0). phase-03.md:1101-1102 "gains two steps: expire pending credential resets, and the consistency check (pool branch lists". The wrong number is a locator error only, which the NOT-a-gap list covers (a name that differs only by a section number). The cite should read "section 4"; the lead may fix it in passing.

### V-01-09-a  REFUTED   (first agent: LOW; refuter: none)
- Claim: 7.2(a) cites "phase 03 section 4.4" for `akr_monitor`, which is not a heading (T6).
- Evidence: phase-03.md:1116 "- **Provisioning:** `akr-ctl db provision` creates the LOGIN role `akr_monitor`" sits in phase 03 section 4 (phase-03.md:1096 onward), which has no subheadings (`grep -c "^### 4\."` gives 0). Locator error only; not a gap. The lead may point the cite at "section 4" in passing.

### V-01-09-b  REFUTED   (first agent: LOW; refuter: none)
- Claim: 7.2(b) cites "phase 23 sections 1.2 and 4.4" for the audit-writer membership; the membership is in section 4.5 item 4 (T6).
- Evidence: the membership is granted in section 1.2: phase-23.md:201 "GRANT akr_audit_writer TO akr_retention;", and the login is a member of `akr_retention`, which holds that grant. phase-23.md:934 (section 4.5 item 4) says the LOGIN role is created "a member of `akr_retention`, with the audit writer membership of section 1.2". So the cite to 1.2 is right, and "4.4" leads to the provisioning text through phase-23.md:922 "(provisioning, section 4.5 item 4)". The first agent misreads the grant: section 4.5 item 4 states the login's role, and the audit-writer grant itself is in section 1.2. Not a gap. The lead may repoint "4.4" to "4.5 item 4" in passing.

### V-01-10-a  REFUTED   (first agent: LOW; refuter: none)
- Claim: 7.4 cites "phase 07 section 4.5" for `a2b@`, which does not exist (T6).
- Evidence: phase-07.md:848 "Phase 01: `fixtures/dev-users.yaml` gains a second A2 user (`a2b@bank.test`)" is an unnumbered bullet in phase 07 section 4 (`## 4. Infrastructure Updates` at phase-07.md:795; `grep -c "^### 4"` gives 0). Locator error only; not a gap. The lead may point the cite at "section 4" in passing.

### V-01-10-b  REFUTED   (first agent: LOW; refuter: none)
- Claim: 7.4 cites "phase 24 section 4.5" for `as@`, which does not mention `as@` (T6).
- Evidence: the name is defined in phase 24 under other sections: phase-24.md:1229 "plus `as@bank.test`, role `AS`, grant valid for one day" (section 6, item 5) and phase-24.md:1503 (Open doubt 27) "Phase 01's dev users do not include it; the implementation adds it to the dev seed." Section 4.5 (phase-24.md:1165) does not mention it. Locator error only, which the NOT-a-gap list covers. Open doubt 27 agrees with 7.4. The lead may repoint the cite to section 6 item 5 and Open doubt 27 in passing.

### V-01-11-a  REFUTED   (first agent: LOW; refuter: none)
- Claim: the pointer to `scopes_for_roles` at line 436 is one line off; the function bullet is at line 437 (T6).
- Evidence: phase-01.md:1092 "the role-to-scope table of `common::scopes::scopes_for_roles` (line 436)"; the bullet "**Scope mapping** (`common::scopes::scopes_for_roles`)" is at phase-01.md:437. The name is correct and the locator is off by one line, which is not a gap. The lead may correct the line number in passing.

## Summary

Counts: REFUTED 6, STANDS 3, PARTLY 2 (11 Gap IDs, each handled once).

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-01-01-a | PARTLY | LOW | The Goal sentence (phase-01.md:8) says session minima are floors; DR-34 and the body make session limits hard bounds | Phase 01, Goal line in the header |
| V-01-02-a | PARTLY | MEDIUM | `client create` has no write order, transaction or failure path; `client rotate` has no behaviour text anywhere; `client disable` is covered by the phase 03 offboarding order | Phase 01 section 3, `akr-ctl identity` bullets (phase-01.md:793-796) |
| V-01-04-a | STANDS | MEDIUM | Heartbeat and logout have no pinned Cedar action; the phase 03 coverage test needs one per operation | Phase 01 section 3 Cedar paragraph (phase-01.md:858-862); phase 03 section 3.11 retrofit list (phase-03.md:995-997) |
| V-01-05-a | STANDS | MEDIUM | The `pat.max_per_user` count runs outside the step 3 transaction under no lock | Phase 01 section 3, `TokenService::create` steps 2-3 (phase-01.md:752-756) |
| V-01-07-a | STANDS | MEDIUM | The sign-in anomaly rules have no owner: phase 25 line 13 points to phase 24's SP-36, which no phase carries | Phase 24 header (Requirements carried or Open doubts); phase-25.md:13 and phase-01.md:18-19 to match |
