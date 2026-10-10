# Verification pass, phase 03 (Haiku draft for lead review)

Scope: 12 items. Header items (7): SP-19, SP-21, SP-27, SP-35, SP-14, TS-26, M-43. The five spread over the "Requirements carried" list
(positions 1, 3, 8, 12, 16 of 17 entries incl. B-04) are SP-19, SP-21, SP-14, TS-26, M-43. The two extra header items
(SP-27 at position 5, SP-35 at position 7) fill the adversarial slot, because the file has only one "Adversarial pass" block
(7.14, run 5) and the rule asks for three. Gap blocks (not adversarial, four different blocks): 7.2, 7.7, 7.10, 7.12.
Adversarial: 7.14.
Evidence paths are docs/plan/phase-03.md unless another file is named. Line numbers were taken with grep -n.

### V-03-01 Authorisation gate, bundle lifecycle and Cedar actions (source: header SP-19; result: GAPS 1)
- T1 SQL: PASS - admin.policy_bundles at phase-03.md:51-90 (table, indexes, guard trigger); grants at :155-158.
- T2 Route: PASS - route rows name scope and the action in section 3.12, e.g. `GET /admin/policies` admin.policies (:511), action `policy.read` (:1067). The route table has no action column; section 3.12 is the mapping, and every route action name found there.
- T3 Repository and types: GAP (LOW) - the trait `Authorizer` (:355-358) declares `authorize` and `invalidate_resource`. Step 6 of 3.2 (:699) says "Evaluate the active bundle (`Authorizer::is_authorized`)", a name that no section defines for our trait (it is the cedar-policy crate method). `AuthzRequest` (:363), `Decision` (:374) and `PolicyBundleRepository` (:426) are PASS.
- T4 Logic: PASS - `loader::load` steps 1-5 (:642 ff.), `BundleWatcher` steps 1-4 (:667-675), 3.2 steps 1-9 (:683-711) with 403 E_SCOPE, 422 E_PURPOSE_REQUIRED, 503 E_UNAVAILABLE on a missing decision row.
- T5 Audit: PASS - `policy_bundle.installed` (:1032), `policy_bundle.load_failed` (:1035), `policy_bundle.activated`, `policy_bundle.shadow_changed`, `authz.allowed` (:1028), `authz.denied` (:1029).
- T6 Cross-references: PASS - `AuthContext` (phase-01.md, 4 mentions), `RequestCtx` and `emit_standalone` (phase-02.md:580), `fresh_until` (phase-01.md:741), `E_INTERNAL` (phase-00.md:483), `Tier` and `PurposeCode` (phase-02.md). Five names checked; all defined.
- T7 Consistency: PASS - 3.1 says no role-level SoD pair for change requests (phase-03.md:744 and research.md:65, DR-54), consistent with 3.4 (:733-745).
- Gaps:
GAP V-03-01-a | LOW | T3 | `Authorizer::is_authorized` (step 6 of 3.2) is not a method of the section 2 trait `Authorizer`, which has `authorize` | section 3.2 step 6 (phase-03.md:699) or the trait at :355-358 | greps: `grep -n "is_authorized" docs/plan/phase-03.md` (:699 only); `grep -n "fn authorize" docs/plan/phase-03.md` (:358)

### V-03-02 Segregation of duties (source: header SP-21; result: GAPS 1)
- T1 SQL: PASS - `sod` JSONB CHECK on admin.policy_bundles (:~58, `sod ? 'exclusive_roles' AND sod ? 'action_pairs'`); `credential_resets` CHECKs `initiated_by <> user_id`, `confirmed_by <> initiated_by` (phase-03.md:~232-236).
- T2 Route: PASS - `POST /org/delegations` (:497) names `delegation.create` (:1062) and the SoD refusal in 3.9 step 3; `POST /admin/credential-resets/{id}/confirm` (:510) is 4E with `credential_reset.confirm`.
- T3 Repository and types: PASS - `SodRules { exclusive_roles, action_pairs }` (:347) and `SodRules::check_roles(&BTreeSet<RoleCode>)` (:414).
- T4 Logic: PASS - 3.4 role level, action pairs, resource level (:735-744); DelegationService create step 3 (:934-950 block) gives 422 `/to_user` `sod_conflict`; the bundle test is in section 6 (:1198).
- T5 Audit: N/A - a refusal. The delegation mutation names `delegation.created` (3.9 step 4, :1049).
- T6 Cross-references: GAP (LOW) - phase 04 says the sod.json pair stays and cites "(section 3.5, DR-54)" at phase-04.md:1521. The rule is in phase 03 section 3.1 and 3.4; section 3.5 is the simulator. The DR is right (research.md:65). Other names checked: `sod_conflict` in phase-04.md:1369, exists.
- T7 Consistency: PASS - exclusive roles (ADM, SEC, IA, AS, VS) in sod.json equal the phase 01 delegation CHECK (`scope ?| ARRAY['ADM', 'SEC', 'IA', 'AS', 'VS']`, phase-01.md:~155) and the 3.9 rule.
- Gaps:
GAP V-03-02-a | LOW | T6 | Phase 04 cites phase 03 "section 3.5" for the single sod.json pair; the text is in section 3.1 (and 3.4); 3.5 is the simulator | phase-04.md:1521 should cite phase 03 section 3.1 | greps: `grep -n "^### 3.5\|^### 3.1\|^### 3.4" docs/plan/phase-03.md`; `grep -n "section 3.5" docs/plan/phase-04.md`

### V-03-03 Existence masking (source: header SP-27, extra to fill the adversarial slot; result: GAPS 1)
- T1 SQL: N/A - no table. The setting `authz.mask_floor_ms` is seeded in migration 0013 (phase-03.md:~196).
- T2 Route: GAP (MEDIUM) - the development route `GET /api/v1/_dev/masked/{id}` is in 3.11 (:1002-1003) and in the curl and masking tests (:1238). It is in no route table, and no scope or Cedar action is named for it; the 3.12 action table has no `_dev` row.
- T3 Repository and types: PASS - `DenyReason::masks` (:385-394 block), `MASKED_KINDS: &[MaskedKindProbes]` (:800).
- T4 Logic: PASS - 3.6 steps 1-4 (:782-789) numbered, 404 body and headers, the latency floor (:787), and the no-purpose rule (:789).
- T5 Audit: PASS - a masked refusal writes a decision row and, for a masked read, an access event (3.2 steps 7 and 8, :706-711); a missing record writes no access event (:711, stated).
- T6 Cross-references: PASS - `AccessLog::record` (phase-02.md, 8 mentions), `authz.mask_floor_ms` (0013), masked kinds registered by phase 15 (7.1 table, :1263-1276). Three names checked.
- T7 Consistency: PASS - 7.1 resolves section 2's list of masked kinds (reports, evidence and freezes go through the case rule); 7.14 narrows 3.6 step 4 to refused callers and says so.
- Gaps:
GAP V-03-03-a | MEDIUM | T2 | The development route `GET /_dev/masked/{id}` has no route-table row, no scope and no Cedar action | section 2 route table (dev rows) or section 3.11 with an action row in section 3.12 | greps: `grep -n "_dev" docs/plan/phase-03.md` (3.11, 6 and 7.13 only); `grep -n "_dev/masked\|_dev_masked" docs/plan/phase-0[0-2].md` (none); `grep -n "dev" docs/plan/phase-03.md` in the 3.12 table (none)

### V-03-04 Purpose codes and the access layer (source: header SP-35, extra to fill the adversarial slot; result: PASS)
- T1 SQL: PASS - admin.purpose_codes at :116-127; seed INSERT at :130 (ten rows); `report_filing` row from 7.2 at :1286.
- T2 Route: PASS - `GET /access/purpose-codes` (:514), scope self, action `purpose_code.list` [self] (:1060).
- T3 Repository and types: PASS - `PurposeCodeRepository { list(active_only); get(code) }` (:448), `PurposeCodeDef` (:340), `PurposeCodeView` (:465 block).
- T4 Logic: PASS - 3.11 (:990): tier check and `reason_required` check, both 422 E_PURPOSE_REQUIRED.
- T5 Audit: N/A - a read. The access events `deny` and `conflict` are named in 3.2 step 8 (:711).
- T6 Cross-references: PASS - `AccessScope`, `AccessLog`, `X-Access-Reason` (phase-02.md:724), `PurposeCode`, `Tier` in phase-02.md. Four names checked.
- T7 Consistency: PASS - 7.2 adds `report_filing` with tiers `{restricted,suspicion}`, consistent with the tier rule of 3.11; base says ten seeded codes (research.md:63, DR-52), and 7.2 adds one under DR-362 (research.md:373).
- Gaps: none.

### V-03-05 Administrator provisioning and recovery (source: header SP-14; result: GAPS 1)
- T1 SQL: PASS - `admin.users` columns `forced_change`, `temp_password_expires_at`, `failed_logins`, `locked_until` (phase-01.md:52-90); `admin.credential_resets` (phase-03.md:220-242).
- T2 Route: PASS - `POST /admin/users` (:501), `POST /admin/users/{id}/unlock` (:504), `POST /admin/users/{id}/reset-credentials` (:508), `POST /admin/credential-resets/{id}/confirm` (:510).
- T3 Repository and types: GAP (HIGH) - the e-mail change in `UserAdminService::update` (:825-829) calls Kratos `update_traits`, which no section defines. The phase 01 Kratos admin port (phase-01.md:466-468) lists `create_identity`, `identity_with_credentials`, `set_state`, `set_password`, `remove_second_factors`, `list_sessions`, `revoke_session`, `revoke_all_sessions`, `delete_identity`, with no traits update. The create path (:812-820) uses `create_identity`, which exists. Reported here because it is the same administration service and the only Kratos write the user records need beyond create; it sits in the update step, not in provisioning itself.
- T4 Logic: PASS - `create` steps 1-5 (:813-823) with 422 `/email` taken and the compensation; `request` steps 1-3 (:860-866) with 409 `reset_pending`; `confirm` steps 1-4 (:869-882) with 403 E_SELF_APPROVAL and the zero-row race rule.
- T5 Audit: PASS - `user.created` (:1039), `credential_reset.requested` (:1046), `credential_reset.confirmed` (:1047), `credential_reset.expired` (:1048).
- T6 Cross-references: PASS - `StepUpService::consume` (phase-01.md:731), `PasswordPolicy` (phase-01.md, 2 mentions), `create_identity` and `delete_identity` (phase-01.md:466-468), `auth.temp_password_ttl_h` (phase-01.md:345), `identity_sweep` (phase-01.md, 8 mentions). Five names checked.
- T7 Consistency: PASS - the reset four-eyes CHECK (phase-03.md:~236) and the Cedar `reset.four_eyes` forbid (:622-625) agree.
- Gaps:
GAP V-03-05-a | HIGH | T3 | Kratos admin method `update_traits` used by the e-mail change of `UserAdminService::update` is in neither the phase 01 Kratos admin port nor phase 03 | phase 01 section 2 Kratos admin API list (phase-01.md:466-468) or phase 03 section 2 ports | greps: `grep -n "update_traits" docs/plan/*.md research` (only phase-03.md:826); `grep -n "create_identity" docs/plan/phase-01.md` (:466 only)

### V-03-06 Ops cockpit tiles (source: header TS-26; result: PASS)
- T1 SQL: PASS - admin.jobs, admin.outbox, admin.timers (phase-00.md:100-150); audit anchors (phase-02.md:263); admin.evidence_objects (phase-02.md:353). Keys used by the tiles all exist: `ops.queue_age_warn_s`, `ops.disk_warn_pct`, `ops.disk_critical_pct`, `ops.cert_warn_days`, `ops.probe_stale_s` (0013, phase-03.md:~196-204); `ops.backup_max_age_h` and `ops.restore_drill_max_age_days` (phase-00.md:262 and :266); `audit.anchor_lag_alert_s` (phase-02.md:490); `evidence.av_max_signature_age_h` (phase-02.md:516).
- T2 Route: PASS - `GET /admin/ops` (:521), scope admin.system, action `ops.read` (:1069).
- T3 Repository and types: PASS - `OpsRepository` with six named methods (:451) and `OpsCockpit::snapshot()` (:962). The tile types (AnchorTile, EvidenceTile, VerifySummary) are named but not defined; they are inferable from the table, so not reported.
- T4 Logic: PASS - tile table with the threshold per tile (:965-980), stale-probe rule (:981), `ops_probe` steps (:982-985).
- T5 Audit: N/A - read only.
- T6 Cross-references: PASS - `sys/seal-status` (phase-00.md, 1 mention), `akr_backup_age_seconds` (phase-00.md, 3 mentions), `PING` (phase-00.md, 1 mention), anchors and evidence in phase 02 as above. Five names checked.
- T7 Consistency: PASS - 7.3 adds tiles from later phases through the same `snapshot()` (:1292-1297) and does not change the base table.
- Gaps: none.

### V-03-07 Liaison and alternate per branch (source: header M-43; result: GAPS 1)
- T1 SQL: PASS - admin.branches with `liaison_user_id`, `alternate_user_id` and CHECKs (phase-03.md:187-202); users FK and `enabled_at` (:212-218).
- T2 Route: PASS - `PATCH /org/branches/{id}` (:492) names liaison and alternate, scope admin.users, action `branch.update` (:1061).
- T3 Repository and types: PASS - `BranchRepository` with `covering_liaison`, `clear_liaison`, `update(tx, id, BranchPatch, expected_version)`, `usage(id)` (:431); `UpdateBranch` with `Option<Option<Uuid>>` (:417).
- T4 Logic: GAP (MEDIUM) - `BranchService::update` (:918-920) is bullet prose, not numbered steps. The order of the checks and the transaction are not given. "The alternate needs a liaison" has no HTTP status or problem code (the 0012 CHECK at :~200 backs it, no error is named). The archive refusal says "409 with the three counts" with no sub-code; `in_use` exists in the section 3.12 mapping (:1015) but is not linked to this refusal. The lock comes from 7.7 (:1321).
- T5 Audit: PASS - `branch.updated {changed, liaison, alternate}` (:918-920) and the cleared assignment on revocation (3.7 `revoke_grant`, :831-836).
- T6 Cross-references: PASS - grant role `L` in phase-01.md:118; `not_liaison_for_branch` (:918); `already_liaison_elsewhere` (7.7) used by phase-22.md:999 and :1044.
- T7 Consistency: PASS - 7.7 adds the one-branch rule that DR-205 states (research.md:216).
- Gaps:
GAP V-03-07-a | MEDIUM | T4 | `BranchService::update` is prose, not numbered steps. The order of checks and the transaction are not given. The "alternate needs a liaison" refusal has no status or problem code. The archive refusal has no sub-code linked (`in_use`) | section 3.9, `BranchService::update` (phase-03.md:918-920) | greps: `grep -n "alternate needs a liaison" docs/plan/phase-03.md`; `grep -n "in_use" docs/plan/phase-03.md` (:1015 only)

### V-03-08 Purpose code for report filing (source: gap block 7.2; result: PASS)
- T1 SQL: PASS - the row at phase-03.md:1286 has all the columns of admin.purpose_codes in the base order; its code matches the format CHECK (`^[a-z][a-z0-9_]{1,63}$`, :119).
- T2 Route: N/A - the route is phase 21's report package download (phase-21.md, section 3.9, row `GET /reports/{id}/package`).
- T3 Repository and types: N/A - the row is read through the base `PurposeCodeRepository`.
- T4 Logic: N/A - a seed row; the tier rule is 3.11.
- T5 Audit: N/A.
- T6 Cross-references: PASS - phase-21.md:888 says the package download takes "a code of the phase 03 purpose list"; phase-21.md:1441 (Open doubts 13) is the gap this row closes; "### 3.9 Download" is the heading of that section in phase 21.
- T7 Consistency: PASS - base says ten seeded codes (research.md:63); 7.2 adds one and cites DR-362 (research.md:373).
- Gaps: none.

### V-03-09 One branch per liaison or alternate (source: gap block 7.7; result: PASS)
- T1 SQL: PASS - `uq_branches_liaison` and `uq_branches_alternate` (phase-03.md:1323-1328) are exact DDL with the partial WHERE. The block states it replaces the two plain indexes of 0012 (:205-206), so the block wins (T7).
- T2 Route: N/A - the rule lives inside `PATCH /org/branches/{id}` (:492).
- T3 Repository and types: PASS - `pg_advisory_xact_lock` on each assigned user (:1321); `BranchRepository::existing` and `covering_liaison` (:431).
- T4 Logic: PASS - the error (422 E_VALIDATION at /liaison_user_id or /alternate_user_id, sub-code `already_liaison_elsewhere`) and the lock order (:1321).
- T5 Audit: PASS - `branch.updated` (:1037) for the change.
- T6 Cross-references: PASS - phase-22.md:999 and :1044 use the rule as phase 03 defines it; DR-205 at research.md:216.
- T7 Consistency: PASS - the block states the base DDL it replaces; "the case of one user as liaison of one branch and alternate of another is caught by the service rule only" is consistent with the 0012 CHECKs.
- Gaps: none.

### V-03-10 Directory extensions to user administration (source: gap block 7.10; result: PASS)
- T1 SQL: N/A - `directory_active` belongs to phase 31 (phase-31.md:211); the block adds no table.
- T2 Route: N/A - `disable_from_directory` is a service method; `POST /admin/users/{id}/enable` exists (:506).
- T3 Repository and types: PASS - `DirectoryLinkQuery { is_inactive(user_id) -> bool }` (7.10, :1349) and its real implementation `PgDirectoryLinkQuery` (phase-31.md:1027); `disable_from_directory` body (phase-31.md:694-698).
- T4 Logic: PASS - `enable` refusal 409 `E_NOT_ALLOWED_IN_STATE` sub-code `directory_inactive` (7.10; phase-31.md:715 lists the same sub-code).
- T5 Audit: PASS - `user.disabled` gains `source` (7.10, :1351; phase-31.md:742 and :901 agree).
- T6 Cross-references: PASS - phase-31.md:443, :663, :694-698, :1027, :1108 name the block's items. Four names checked.
- T7 Consistency: PASS - "reversible disable, grants stay" (7.10) matches phase-31.md:698 ("a suspension, not an offboarding"); both use reason `admin`.
- Gaps: none.

### V-03-11 akr-ctl policy matrix (source: gap block 7.12; result: GAPS 2)
- T1 SQL: N/A.
- T2 Route: N/A - a CLI command. The API route `GET /admin/access-matrix` (:513) is separate, scope security.ops, audit.read, action `access_matrix.read` (:1068): PASS on its own.
- T3 Repository and types: GAP (MEDIUM) - the no-database entry point of `AccessMatrixBuilder` ("over the bundle directory with no database", :1360) has no name and no parameters. The only signature is `AccessMatrixBuilder::build(at)` (:769), which reads the database.
- T4 Logic: PASS - the output shape `{bundle_hash, roles: {<role>: [<action>, ...]}}` is given (:1360); "No audit row (no database)" is stated.
- T5 Audit: PASS - the no-audit rule is stated in 7.12.
- T6 Cross-references: PASS - phase-08.md:69-71 uses the same command and file (`frontend/src/app/policy-matrix.json`); phase-25.md:721 calls `AccessMatrixBuilder::build(snapshot_at)`.
- T7 Consistency: GAP (MEDIUM) - base 3.5 step 1 (:770-772) derives each role's actions by evaluating every action against a synthetic principal, so forbids apply. Block 7.12 (:1360) derives them "from the permits' @role annotations". These are two rules for one matrix. The block does not say that the database endpoint and the file must agree, so an action removed by a forbid can appear in one and not the other, while the phase 08 drift check and the phase 25 hash compare them.
- Gaps:
GAP V-03-11-a | MEDIUM | T3 | No named no-database entry point, and no parameters, for the matrix build from a bundle directory | section 3.5 `AccessMatrixBuilder` or block 7.12 (phase-03.md:1360) | greps: `grep -n "AccessMatrixBuilder" docs/plan/phase-03.md` (:413, :769, :1126, :1360 only)
GAP V-03-11-b | MEDIUM | T7 | Block 7.12 derives the matrix from @role annotations; base 3.5 step 1 evaluates each action with a synthetic principal, which respects forbids. The two can disagree and the block does not say which one the file and the endpoint must match | phase-03.md:1360 against section 3.5 step 1 (:770-772) | greps: `grep -n "@role" docs/plan/phase-03.md`; `grep -n "synthetic principal" docs/plan/phase-03.md`

### V-03-12 Step-up and purpose answers to admitted callers (source: adversarial run 5 block 7.14; result: PASS)
- T1 SQL: N/A.
- T2 Route: N/A - no new route; it changes the answers of existing masked routes.
- T3 Repository and types: N/A - the added probe is named; the harness registry `MASKED_KINDS` is at :800.
- T4 Logic: PASS - the block splits refused callers (404 always, 3.6 step 4 at :789) from admitted callers (401 E_STEPUP_REQUIRED, 422 E_PURPOSE_REQUIRED), and gives the probe rule for the refused caller.
- T5 Audit: N/A - no mutation.
- T6 Cross-references: PASS - 401 and 422 in phase-17.md:1240 and :1324; phase-18.md:910-911; phase-19.md:1326-1327; phase-21.md:1037-1038. The network and purpose forbids that make "admitted" true: phase-17.md:1441-1449 and phase-18.md:1053-1084. DR-411 (research.md:422), DR-46 (research.md:57), DR-48 (research.md:59).
- T7 Consistency: PASS - 7.14 applies to "a caller the policy refuses" and narrows 3.6 step 4, which it names; the block says what it replaces.
- Gaps: none.

## Summary
Items checked: 12. Items fully PASS: 6 (V-03-04, V-03-06, V-03-08, V-03-09, V-03-10, V-03-12).
GAP lines: 7 (HIGH 1, MEDIUM 4, LOW 2).

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-03-01-a | LOW | T3 | `Authorizer::is_authorized` in step 6 is not a method of the section 2 trait (`authorize`) | phase-03.md:699 or :355-358 |
| V-03-02-a | LOW | T6 | Phase 04 cites phase 03 "section 3.5" for the sod.json pair; it is section 3.1 | phase-04.md:1521 |
| V-03-03-a | MEDIUM | T2 | `GET /_dev/masked/{id}` has no route-table row, scope or Cedar action | section 2 route table or 3.11 with an action row in 3.12 |
| V-03-05-a | HIGH | T3 | Kratos `update_traits` (e-mail change in `UserAdminService::update`) is not in the phase 01 Kratos admin port | phase-01.md:466-468 |
| V-03-07-a | MEDIUM | T4 | `BranchService::update` is prose; check order and transaction not given; "alternate needs a liaison" has no code; archive refusal sub-code not linked | phase-03.md:918-920 |
| V-03-11-a | MEDIUM | T3 | No named no-database entry point or parameters for the matrix from a bundle directory | phase-03.md:1360 (or section 3.5) |
| V-03-11-b | MEDIUM | T7 | Block 7.12 derives the matrix from @role annotations; base 3.5 step 1 evaluates actions with synthetic principals (respects forbids) | phase-03.md:1360 vs :770-772 |
