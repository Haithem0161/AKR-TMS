# Verification pass, phase 01 (Haiku draft for lead review)

Sample (12 items, chosen before checking):
- Header "Requirements carried" (the file has no "Implemented here" list): SP-14 (start), SP-16 (early), SP-71 (middle), UX-35 (late), VA-46 (end), SP-101, SP-18. Header order: SP-14, SP-15, SP-16, SP-17, SP-18, SP-71, SP-101, TS-25, UX-35, UX-36, VA-46.
- Gap pass blocks (run 1, not adversarial): 7.1 identity_sweep, 7.2 db provision, 7.4 dev seed, 7.6 OpenAPI extensions.
- Adversarial: the file has one adversarial block, 7.8 (run 1, four-eyes). The other two adversarial picks are replaced by header items SP-101 and SP-18.

Notes on the sample: phase 01 has no "Implemented here" list, so the five header picks come from "Requirements carried"
(start, middle, end plus two more). The gap pass has eight non-adversarial 7.x blocks; four were picked. Route rows in
section 2 carry no Cedar column; the Cedar names are pinned in section 3 ("Cedar actions", phase-01.md:858). Those names
are used as the T2 evidence for each route.

### V-01-01 SP-14 CBI password, lockout and session minima as settings floors   (source: header SP-14; result: GAPS 1)
- T1 SQL: PASS - floors on the password and lockout keys in migration 0007 (phase-01.md:326 `auth.password_min_length`
  with floor `{"min": 8, ...}`); phase 00 CHECK `floor IS NULL OR governed` (phase-00.md:214).
- T2 Route: N/A - settings are read and written by phase 03 routes (`GET /admin/settings`, phase-03.md:515); phase 01 adds none.
- T3 Repository and types: N/A - the typed getters are named as "Settings registry additions" (phase-01.md:section 3 end), no new repository.
- T4 Logic: PASS - floor refusal is phase 00's rule: "a value outside its floor -> E_BELOW_FLOOR" (phase-00.md:576).
- T5 Audit: N/A - the settings change audit (`setting.updated`) is phase 03 (phase-03.md:955).
- T6 Cross-references: PASS - `E_BELOW_FLOOR` in phase 03 (phase-03.md:955) and phase 00 (phase-00.md:576); floor accepts `min` or `max` (phase-00.md:208).
- T7 Consistency: GAP - the header says "session minima as floors" (phase-01.md:3), but every session key has floor NULL
  (phase-01.md:357 `session.idle_timeout_s`, :359 `session.absolute_lifetime_s`), and the section 3 settings paragraph
  makes them hard bounds. DR-34 (research.md:45) says session limits are hard bounds and `floor` is for regulatory minima.
  The DR resolves the rule, so the header wording is the only defect.
- Gaps:
  `GAP V-01-01-a | LOW | T7 | header says session minima are floors; body and DR-34 make them hard bounds | header Goal line (phase-01.md:3) | grep "session" in phase-00/03/04 floor rows: none; research.md DR-34 read`

### V-01-02 SP-16 service principals and client-credential clients   (source: header SP-16; result: GAPS 1)
- T1 SQL: PASS - `admin.users` CHECKs: `kind = 'service'` needs `owner_user_id` (phase-01.md:82), `expires_at` required (:83), `hydra_client_id` only for service (:84); unique index on `hydra_client_id`.
- T2 Route: PASS - the only route for a service principal is `GET /api/v1/me` (scope self, session or token; phase-01.md route table); the creation path is CLI, see T4.
- T3 Repository and types: PASS - `ClientCredentials::authenticate(jwt)` (phase-01.md:773 section); `UserRepository::insert(tx, NewUser)` and `touch_service_use(id, at)` (phase-01.md:444, :448). `NewUser` has no field list; the fields follow the admin.users DDL (phase-01.md:52).
- T4 Logic: GAP - `identity client create`, `client rotate` and `client disable` are one bullet of prose (phase-01.md:793-796) with no numbered steps, no write order between the Hydra client and the principal row, no transaction, and no compensation when one side fails. The bootstrap command has all of these (phase-01.md:782); client create does not. `authenticate` itself has numbered-style steps and is PASS.
- T5 Audit: PASS - `service_client.created`, `service_client.rotated`, `service_client.disabled` are in the audit table (phase-01.md:852).
- T6 Cross-references: PASS - Hydra `strategies.access_token: jwt`, 15 minutes (phase-01.md section 4, Hydra bullet); `HYDRA_ADMIN_URL` in section 3 environment list.
- T7 Consistency: PASS - no 7.x block changes the service principal rules.
- Gaps:
  `GAP V-01-02-a | MEDIUM | T4 | client create, rotate and disable have no numbered steps, no write order, no transaction and no compensation | section 3, the akr-ctl identity bullets (phase-01.md:793-796) | grep "client create" across docs/plan: phase-01.md only (lines 774, 793, 1026)`

### V-01-03 SP-71 unauthenticated routes limited to the two public ones   (source: header SP-71; result: PASS)
- T1 SQL: N/A - no table.
- T2 Route: N/A - the auth layer applies to every `/api/v1` route; the two public routes are in the route table (phase-01.md:516-517, public rows).
- T3 Repository and types: PASS - `PUBLIC_ROUTES` named as the list of the two routes (phase-01.md:561).
- T4 Logic: PASS - auth layer steps 1-9 are numbered with errors: 403 `E_CSRF_INVALID`, 400 `E_BAD_REQUEST`, 401 `E_UNAUTHENTICATED`, 403 `E_CREDENTIAL_CHANGE_REQUIRED`, 403 `E_SCOPE`, 429 `E_RATE_LIMITED` (phase-01.md:556-577).
- T5 Audit: N/A - the layer writes no audit event; the session expiry it triggers is `auth.session_expired` (named in `resolve`).
- T6 Cross-references: PASS - auth slot and `E_UNAUTHENTICATED` in phase 00 (phase-00.md:465, :489, :662); coverage test SP-71 in phase 03 (phase-03.md:1187). `API.md` section 6 (line 961) does not yet list `E_CSRF_INVALID`, `E_CREDENTIAL_CHANGE_REQUIRED` or `E_MIN_FACTORS`; phase 01 says API.md gains them (phase-01.md:821-823), so this is an owed doc edit, not a plan gap.
- T7 Consistency: PASS - phase 00 says the `X-Dev-Principal` stub is the auth slot (phase-00.md:662); phase 01 deletes it (phase-01.md section 4, "The phase 00 `X-Dev-Principal` stub is deleted"). No conflict.

### V-01-04 UX-35 session warning and heartbeat   (source: header UX-35; result: GAPS 1)
- T1 SQL: N/A - the idle clock is a Valkey key (`idle:{sid}`, phase-01.md:604); no SQL.
- T2 Route: GAP - `POST /api/v1/auth/session/heartbeat` (scope `self (session)`, phase-01.md:519) has no Cedar action. The pinned list at phase-01.md:858-860 names 14 actions and omits heartbeat and logout.
- T3 Repository and types: PASS - `SessionService::heartbeat(ctx)` returns `SessionView` (phase-01.md:604); `SessionView` fields include `idle_expires_at` and `warning_s` (utoipa schemas).
- T4 Logic: PASS - touch when last touch is older than `session.heartbeat_min_interval_s`, else no-op; the step-up and GET rules are in auth layer step 8 (phase-01.md:577). The method is one sentence but has one condition and one return.
- T5 Audit: N/A - Valkey touch, no audit by design (DR-31, research.md:42; "never extends the idle clock").
- T6 Cross-references: PASS - phase 08 SessionWarning reads `idle_expires_at` and `warning_s` and sends the heartbeat at most once per `session.heartbeat_min_interval_s` (phase-08.md:315-318); UX-35 server half named in phase 08 (phase-08.md:34).
- T7 Consistency: PASS - DR-31 and the base text agree (heartbeat only, GETs never touch).
- Gaps:
  `GAP V-01-04-a | MEDIUM | T2 | POST /auth/session/heartbeat (and POST /auth/logout) has no Cedar action | section 3 "Cedar actions" list (phase-01.md:858-860) or the route row (phase-01.md:519) | grep "heartbeat" and "logout" in the Cedar list: none`

### V-01-05 VA-46 personal access tokens with the user's roles and read scopes   (source: header VA-46; result: GAPS 1)
- T1 SQL: PASS - `admin.api_tokens` (phase-01.md:257) with `scopes <@ '{self,dashboard.read,audit.read,assessor.read,data.steward}'` (:263), expiry at most 90 days; settings `pat.max_ttl_days` (phase-01.md:379) and `pat.max_per_user` (:381). Grants to `akr_app` (phase-01.md section 1, phase-01.md:288-289).
- T2 Route: PASS - `GET`, `POST` and `DELETE /api/v1/me/tokens[/{id}]` (phase-01.md:536-537, scope `self (session)`, SU on create). Cedar names pinned: `token.list`, `token.create`, `token.revoke` (phase-01.md:858-860).
- T3 Repository and types: PASS - `TokenService` create, authenticate, list, revoke (phase-01.md:745-780); `TokenRepository` methods (phase-01.md:457); `CreateTokenRequest`, `TokenCreated`, `TokenView` (utoipa schemas).
- T4 Logic: GAP - `create` step 2 counts active tokens against `pat.max_per_user` (phase-01.md:752) before step 3, the transaction (phase-01.md:756). The count has no lock and no `FOR UPDATE`; two creates with two different valid step-up assertions can both pass the count. The spec names no lock where two writers meet. `authenticate` steps 1-5 are PASS.
- T5 Audit: PASS - `token.created` and `token.revoked` in the audit table (phase-01.md:846-847).
- T6 Cross-references: PASS - scopes allow-list is API.md section 2 (`assessor.read` at API.md:101, `data.steward` at :97); idempotency exemption for `POST /me/tokens` is in phase 00 list (phase-01.md section 2 exemptions). `E_SCOPE` and `E_STEPUP_REQUIRED` are in API.md section 6 (API.md:968-969).
- T7 Consistency: PASS - no 7.x block changes the token rules.
- Gaps:
  `GAP V-01-05-a | MEDIUM | T4 | the pat.max_per_user count is checked outside the transaction and under no lock, so two creates can exceed the limit | TokenService::create, steps 2-3 (phase-01.md:752-756) | grep "FOR UPDATE\|advisory" in phase-01.md: no lock for tokens`

### V-01-06 SP-101 acknowledgement of the confidentiality undertaking at first login   (source: header SP-101; result: PASS)
- T1 SQL: PASS - `policy_ack_version` and `policy_ack_at` with a pair CHECK (phase-01.md:admin.users, phase-01.md:87); setting `auth.policy_version` (phase-01.md:351).
- T2 Route: PASS - `POST /api/v1/me/policy-acknowledgement` (phase-01.md:525, scope `self (session)`); Cedar `policy.acknowledge` pinned (phase-01.md:858-860); allowed while pending (auth layer step 5, phase-01.md:569).
- T3 Repository and types: PASS - `acknowledge_policy(ctx, version)` (phase-01.md:696), `UserRepository::acknowledge_policy(tx, id, version, at)` (phase-01.md:446), `PolicyAckRequest { version }` (phase-01.md:442).
- T4 Logic: PASS - version check first (422 `E_VALIDATION` at `/version`, `stale`), then the transaction, then the audit and `invalidate_user` (phase-01.md:696-698).
- T5 Audit: PASS - `auth.policy_acknowledged {version}` in the audit table.
- T6 Cross-references: PASS - raising `auth.policy_version` goes through the governed-setting change path of phase 04 (phase-04.md:754 "UpdateSetting ... required when the key is governed"); phase 03 shows the undertaking text in `MeView` (phase-01.md section 2 `MeView`).
- T7 Consistency: PASS - `MeView.policy.text` only while pending (phase-01.md section 2) agrees with the step 5 list.

### V-01-07 SP-18 sign-in and step-up events carry the anomaly fields   (source: header SP-18; result: GAPS 1)
- T1 SQL: PASS - `audit.audit_log.request` JSONB holds ip, session_id, token_id, ua (phase-02.md:124).
- T2 Route: N/A - events, no route.
- T3 Repository and types: PASS - `RequestCtx { ip, session_id, token_id, job_id, correlation_id, ua }` and `AuditEvent::new(&kind, actor, request)` (phase-02.md:554-559).
- T4 Logic: N/A - the events are emitted by the steps of sections 3 (login, mfa, step-up); each names its kind.
- T5 Audit: PASS - `auth.login_succeeded`, `auth.login_failed`, `stepup.issued`, `stepup.denied` (phase-01.md:833-847).
- T6 Cross-references: GAP - the header says "the SIEM rules in phase 25" (phase-01.md:section header, Requirements carried). Phase 25 says the anomaly rules "are phase 24's SP-36 pack" (phase-25.md:13). Phase 24 defines no anomaly rule (grep "anomal" in phase-24.md: no hit), and SP-36 appears in no docs/plan file other than phase-25.md:13. The rules that the sign-in fields serve have no owner the grep can find.
- T7 Consistency: PASS - phase-01.md:833-834 (sign-in and step-up rows) agrees with phase-02.md:124 request block.
- Gaps:
  `GAP V-01-07-a | MEDIUM | T6 | the anomaly rules that the SP-18 fields serve are named in phase 25 as phase 24's SP-36 pack; phase 24 and the plan define no SP-36 | header Requirements carried (phase-01.md header) vs phase-25.md:13 | grep "SP-36" docs/plan/*.md: phase-25.md:13 only; grep "anomal" phase-24.md: none`

### V-01-08 7.1 identity_sweep: dormancy basis, pending credential resets, branch consistency   (source: 7.1; result: GAPS 1)
- T1 SQL: PASS - `enabled_at` added to `admin.users` by phase 03 (phase-03.md:218); `admin.credential_resets` with `status` in ('pending', 'completed', 'expired') and `expires_at` (phase-03.md:226, index :246).
- T2 Route: N/A - job, no route.
- T3 Repository and types: PASS - `UserRepository::dormant_candidates(cutoff, limit)` (phase-01.md:448); the new dormancy expression is written out in step 1 (phase-01.md:1050). The expiry of resets is phase 03's sweep step (phase-03.md:884-887).
- T4 Logic: PASS - step 1 is the replaced expression; step 5 is one condition (`expires_at <= now()` -> `expired`); step 6 is a check with a log and a count, no write (phase-01.md:1050-1056). Batching and the pause are base step 4 (phase-01.md:805).
- T5 Audit: PASS - step 5 names `credential_reset.expired` (phase-03.md:887; audit table phase-03.md:1048); base step 3 names `user.disabled` (phase-01.md:806).
- T6 Cross-references: GAP (LOW) - 7.1 step 5 cites "phase 03 section 3.8" (correct: phase-03.md:855 "### 3.8") and step 1 cites "phase 03 section 3.7" (correct: phase-03.md:806 "### 3.7"). Step 6 cites "phase 03 section 4.5", but phase 03 has no 4.5 heading; the consistency check is an unnumbered bullet inside section 4 (phase-03.md:1103). Other names checked: `enabled_at` (phase-03.md:218), `credential_resets` (phase-03.md:220).
- T7 Consistency: PASS - step 1 says "instead of" the base expression (phase-01.md:1050), so the block wins by its own words; phase 03 uses the same expression (phase-03.md:852).
- Gaps:
  `GAP V-01-08-a | LOW | T6 | step 6 cites "phase 03 section 4.5", which is not a heading; the consistency check is an unnumbered bullet in phase 03 section 4 | 7.1 step 6 (phase-01.md:1055) | grep "^### 4\." phase-03.md: no hit; phase-03.md:1103 is the bullet`

### V-01-09 7.2 akr-ctl db provision: monitor, retention and audit-writer logins, polis database, grants   (source: 7.2; result: GAPS 2)
- T1 SQL: N/A - provisioning (roles and grants, no AKR table).
- T2 Route: N/A - CLI.
- T3 Repository and types: N/A - no type named.
- T4 Logic: PASS - items (a) to (e) each name a role or database, the password source in OpenBao and the target phase (phase-01.md:1059-1067). No failure path is stated, but each step is a single statement.
- T5 Audit: PASS - the akr-ctl commands log the OS user into the audit sink (phase-01.md:781, section 3 akr-ctl header).
- T6 Cross-references: GAP x2 (LOW) - (a) `akr_monitor` is defined at phase-00.md:883-886 (PASS), but the cited "phase 03 section 4.4" does not exist; the provisioning bullet is unnumbered at phase-03.md:1116. (b) `akr_retention_login` and the audit-writer membership are cited to "phase 23 sections 1.2 and 4.4"; 1.2 has `GRANT akr_audit_writer TO akr_retention` (phase-23.md:201) and the LOGIN role plus its membership are at phase-23.md:934 (section 4.5 item 4); section 4.4 only points there (phase-23.md:922). (c) `polis` database: phase-31.md:899 (PASS). (d) `GRANT akr_audit_writer TO akr_app, akr_ingest, akr_assess`: phase-02.md:1042 (PASS). (e) `akr_readonly` LOGIN: phase-00.md:35-37 (PASS). `akr_assess` role exists (phase-00.md:50).
- T7 Consistency: PASS - base section 3 says every akr-ctl command logs to the audit sink, and 7.2 adds no contrary rule.
- Gaps:
  `GAP V-01-09-a | LOW | T6 | (a) cites "phase 03 section 4.4" for akr_monitor; no such heading | 7.2(a) (phase-01.md:1060) | grep "^### 4\." phase-03.md: no hit; provisioning bullet at phase-03.md:1116`
  `GAP V-01-09-b | LOW | T6 | (b) cites "phase 23 sections 1.2 and 4.4" for the audit-writer membership; the membership is in section 4.5 item 4 | 7.2(b) (phase-01.md:1061-1062) | phase-23.md:922 points to 4.5; phase-23.md:934 holds the membership`

### V-01-10 7.4 identity seed-dev: extra fixed users   (source: 7.4; result: GAPS 2)
- T1 SQL: N/A - seed data.
- T2 Route: N/A.
- T3 Repository and types: N/A.
- T4 Logic: N/A - a fixture list, no steps.
- T5 Audit: N/A.
- T6 Cross-references: GAP x2 (LOW) - (a) `a2b@` is cited to "phase 07 section 4.5"; phase 07 has no 4.5 heading. The a2b extension is an unnumbered bullet in phase 07 section 4 (phase-07.md:848) and the curl use is at phase-07.md:924. (b) `as@` is cited to "phase 24 section 4.5"; that section does not mention as@. The text is in phase 24 section 6 item 5 (phase-24.md:1229) and open doubt 27 (phase-24.md:1503). `sec2@` and `vs@` are cited to "phase 25 section 6" (PASS: phase-25.md:1321 names both users and their roles).
- T7 Consistency: PASS - base "one per role" (phase-01.md:797) vs 7.4 "vs has no role": 7.4 is the later rule and says so; phase 25 agrees (phase-25.md:1321).
- Gaps:
  `GAP V-01-10-a | LOW | T6 | a2b@ cited to "phase 07 section 4.5", which does not exist | 7.4 (phase-01.md:1078) | grep "^### 4\." phase-07.md: no hit; bullet at phase-07.md:848`
  `GAP V-01-10-b | LOW | T6 | as@ cited to "phase 24 section 4.5", which does not mention as@ | 7.4 (phase-01.md:1083) | grep "as@" in phase-24.md: lines 1229, 1239-1240, 1503 only`

### V-01-11 7.6 OpenAPI extensions for the generated client   (source: 7.6; result: GAPS 1)
- T1 SQL: N/A.
- T2 Route: PASS - the step-up operations are `DELETE /me/credentials/{id}` (SU, phase-01.md:534) and `POST /me/tokens` (SU, phase-01.md:536); `x-akr-stepup` applies to both.
- T3 Repository and types: PASS - schemas `ActionClass` and `ProblemCode` named (7.6, phase-01.md:1091-1093); `x-akr-role-scopes` source named.
- T4 Logic: N/A - a document change, no steps.
- T5 Audit: N/A.
- T6 Cross-references: GAP (LOW) - the pointer "scopes_for_roles (line 436)" is one line off: the bullet is at phase-01.md:437. Consumers are PASS: phase 08 reads `x-akr-stepup` (phase-08.md:58) and section 4 (phase-08.md:548); phase 03 adds `x-akr-action` (phase-03.md:398).
- T7 Consistency: PASS - base "Every SU operation names ... declared action" (phase-01.md:707-712) and 7.6 agree.
- Gaps:
  `GAP V-01-11-a | LOW | T6 | pointer to scopes_for_roles line 436 is one line off; the function bullet is at line 437 | 7.6 (phase-01.md:1092) | sed -n 436,437p phase-01.md`

### V-01-12 7.8 decision step-ups name their resource (ADV-1-04)   (source: adversarial run 1, 7.8; result: PASS)
- T1 SQL: PASS - `resource_ref` CHECK regex accepts `task:<uuid>` and `assess_hold:<uuid>` (phase-01.md:238-250, stepup_assertions).
- T2 Route: N/A - the rule is inside `consume`; the routes that call it are in phases 04 and 18.
- T3 Repository and types: PASS - `consume(tx, ctx, header, action, resource_ref)` (phase-01.md:731); `StepUpAction { name, class }` (phase-01.md:427).
- T4 Logic: PASS - step 2 is replaced for the four classes (phase-01.md:1107-1108); zero rows -> 401 `E_STEPUP_REQUIRED` (base step 2); the test is stated (phase-01.md:1110-1111).
- T5 Audit: N/A - no new mutation; the consuming action's own audit event records `stepup_id` (base step 3).
- T6 Cross-references: PASS - DR-407 exists and names this rule (research.md:418). Callers pass a resource for decision and signing: phase 04 engine passes `resource_ref = "task:<id>"` (phase-04.md:1060-1061, 7.3 at phase-04.md:1667-1669); phase 18 names the engine's step 5 as the consumer (phase-18.md:757); phase 29 passes `assess_hold:<id>` (phase-29.md:1002). Grep for callers of class `filing`: none found, so no caller passes None for it.
- T7 Consistency: PASS - the block states what it replaces (base step 2 predicate) and the classes not listed keep the base predicate.

## Summary

Items checked: 12 (7 from "Requirements carried" in the header, 4 from gap-pass 7.x blocks, 1 from the adversarial block 7.8).
Items fully PASS: 3 (V-01-03 SP-71, V-01-06 SP-101, V-01-12 7.8).
Items with at least one GAP: 9 (V-01-01, 02, 04, 05, 07, 08, 09, 10, 11).
GAP lines by severity: HIGH 0, MEDIUM 4, LOW 7. Total 11.

Sample deviation: the file has one adversarial block (7.8), not three, so two adversarial picks were replaced by header items SP-101 and SP-18. The header has no "Implemented here" list, so the header picks come from "Requirements carried".

Other observations (not gap lines): docs/API.md section 6 does not yet list `E_CSRF_INVALID`, `E_CREDENTIAL_CHANGE_REQUIRED` or `E_MIN_FACTORS` (phase 01 says it will add them). The pinned Cedar list (phase-01.md:858-860) covers 14 actions and omits heartbeat and logout (V-01-04-a).

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-01-01-a | LOW | T7 | header says session minima are floors; body and DR-34 make them hard bounds | header Goal (phase-01.md:3) |
| V-01-02-a | MEDIUM | T4 | client create, rotate and disable: no numbered steps, write order, transaction or compensation | section 3, akr-ctl bullets (phase-01.md:793-796) |
| V-01-04-a | MEDIUM | T2 | POST /auth/session/heartbeat and POST /auth/logout have no Cedar action | Cedar list (phase-01.md:858-860) |
| V-01-05-a | MEDIUM | T4 | pat.max_per_user checked outside the transaction and under no lock | TokenService::create steps 2-3 (phase-01.md:752-756) |
| V-01-07-a | MEDIUM | T6 | anomaly rules named as phase 24 SP-36, which phase 24 and the plan do not define | header (phase-01.md header) vs phase-25.md:13 |
| V-01-08-a | LOW | T6 | cites "phase 03 section 4.5", no such heading | phase-01.md:1055 |
| V-01-09-a | LOW | T6 | cites "phase 03 section 4.4", no such heading | phase-01.md:1060 |
| V-01-09-b | LOW | T6 | cites "phase 23 section 4.4" for audit-writer membership; it is 4.5 item 4 | phase-01.md:1061-1062 |
| V-01-10-a | LOW | T6 | cites "phase 07 section 4.5", no such heading | phase-01.md:1078 |
| V-01-10-b | LOW | T6 | cites "phase 24 section 4.5" for as@; not there | phase-01.md:1083 |
| V-01-11-a | LOW | T6 | scopes_for_roles pointer off by one line (436 vs 437) | phase-01.md:1092 |
