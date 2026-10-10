# Refuter, phase 25 (Haiku draft for lead review)

### V-25-01-a  STANDS  (first agent: MEDIUM; refuter: HIGH)
- Claim: Line 768 says the CaseExistence port is called by `activate`, but step 8 says activation does not look at the case.
- Evidence: phase-25.md:768 "is called only by `activate` (step 6 below)"; phase-25.md:786 "The activation does not look at the case."; phase-25.md:1429 (DR-233) "activation does not look at the case". Step 6 is the first approval, not activation. `grep -n CaseExistence docs/plan/phase-25.md` gives :756 and :768 only, so no step calls the port and no 7.x block settles it. Two texts contradict: HIGH by the rubric.
- Fix: Phase 25, section 3.2, line 768: make the sentence agree with step 8 and DR-233 (activation does not call the port), or name the step that calls it.

### V-25-01-b  PARTLY  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: End and Review give no HTTP status or problem code for their refusals; End has no numbered steps.
- Evidence (refuted parts): the Review state check has a closed sub-code: phase-25.md:983 "`not_reviewable`" (under `E_NOT_ALLOWED_IN_STATE`, 409), matching API.md:982 "Action not allowed in the resource's status". A caller who is neither requester nor SEC is a Cedar denial: phase-25.md:985 "Cedar denial on a resource the caller may know exists" (`E_FORBIDDEN`), and phase-03.md:716 "other denies -> 403 `E_FORBIDDEN`" (general rule of the authorize layer).
  Stands: End's refusal for a grant that is not `active` or `requested` names no code and no sub-code (phase-25.md:796 gives only the effects; the closed list at :983 has no End entry). The End paragraph is one prose paragraph, which is acceptable only when the check and the error are named, and the state check is not.
- Fix: Phase 25, section 3.2, End paragraph (line 796): name the state refusal and its code (`E_NOT_ALLOWED_IN_STATE`, with a sub-code if one is wanted) in the same sentence.

### V-25-01-c  PARTLY  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Approve step 5 has two refusals with no code: an `as_role` the approver does not hold, and an approver who holds both CH and SEC and names no role.
- Evidence: phase-25.md:783 "An approver holding both must name the role." No code is given, and the closed list at phase-25.md:982 has `approver_role_mismatch` only, which step 7 uses (phase-25.md:785). The first refusal is not a refusal in the text: phase-25.md:783 reads "when given and held (CH or SEC), else the one role of the approver among CH and SEC", so a named role not held falls back to the single role. `grep -n "as_role\|must name the role" docs/plan/phase-25.md` gives the signature (:760) and step 5 (:783) only.
  Stands: the case of an approver who holds both roles and names none has no code in any list (phase-25.md:982-983).
- Fix: Phase 25, section 3.2, Approve step 5 (line 783): give the code for an approver who holds both roles and names none, and say whether a named role not held is a refusal (with `approver_role_mismatch`) or a fallback.

### V-25-01-d  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The requester ends a `requested` grant (status and log `lapsed`) and no audit event is named; the catalogue row says "(sweep)" only.
- Evidence: phase-25.md:796 "A request in `requested` may be ended by the requester (status `lapsed`, log `lapsed`)."; phase-25.md:1152 "`break_glass.lapsed` | request not approved within its lifetime (sweep)"; phase-25.md:1136 "Each mutation writes one event of the phase 02 catalogue". This is a user-driven status change, not housekeeping; phase-02.md:777 exempts only anchoring. `grep -n "break_glass.lapsed" docs/plan/phase-25.md` gives :801 and :1152 only.
- Fix: Phase 25, section 3.2 End paragraph (line 796) and section 3.13 row `break_glass.lapsed` (line 1152): name the event the requester's end writes and add the requester to the On column.

### V-25-01-e  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Step-up action constants are not declared; `StepUpAction::BREAK_GLASS_REQUEST` and `break_glass.approve` are used, but no `StepUpAction { name, class }` is declared.
- Evidence: phase-01.md:711 "Each phase declares its actions as `StepUpAction { name, class }` constants"; phase-25.md:771 "`StepUpService::consume(tx, ctx, header, StepUpAction::BREAK_GLASS_REQUEST, grant_id)`"; phase-22.md:548 declares its own constant. `grep -n StepUpAction docs/plan/phase-25.md` gives one hit (:771); `grep -n "class: " docs/plan/phase-25.md` gives none. Most names and classes are in prose (phase-25.md:778 "step-up class `break_glass` with action `break_glass.approve`"), but the request action's name is given nowhere, and phase 01 says a test checks that every SU operation names a declared action.
- Fix: Phase 25, section 3.2 before Request (or a constant list in section 3.12): declare the `StepUpAction` constants of sections 3.1, 3.2, 3.6 and 3.8 with their names and classes.

### V-25-01-f  STANDS  (first agent: LOW; refuter: LOW)
- Claim: `OidcUpstreamConfig` (BreakGlassService field `cfg`) is defined nowhere, no step uses it, and section 3.4 calls the configuration `OidcUpstream`.
- Evidence: phase-25.md:757 "cfg: Arc<OidcUpstreamConfig>"; phase-25.md:820 "`OidcUpstream` is configuration, not a service"; the steps of section 3.2 (phase-25.md:771-796) never read an upstream setting. `grep -rln OidcUpstreamConfig docs/plan docs research` gives phase 25 only (:757 and :1245).
- Fix: Phase 25, section 3.2 struct (line 757): delete the `cfg` field, since break-glass reads no upstream setting.

### V-25-01-g  PARTLY  (first agent: LOW; refuter: LOW)
- Claim: Types named in the 3.2 signatures are not declared: `ReviewOutcome`, `BreakGlassAccepted`, `SweepCounts`, `IdemKey`, `StepUpHeader`.
- Evidence (refuted parts): `ReviewOutcome`'s values are named: phase-25.md:798 "the outcome is one of `appropriate`, `escalate`, `no_action`". `BreakGlassAccepted`'s body is named: phase-25.md:776 "Answer 202 with `{ status: "requested" }`". `SweepCounts`'s fields are named once: phase-25.md:749 "Return counts `{ superseded, overdue_opened }`" (access review sweep only). `IdemKey` is described by the idempotency middleware: phase-00.md:517 "Read `Idempotency-Key`; missing -> 400". `StepUpHeader` is described: phase-01.md:239 "the value sent as `X-StepUp-Assertion`; UUIDv7".
  Stands: the break-glass sweep (phase-25.md:764, steps at :800-803) and the support sweeps (:925-926) name no returned counts; no file declares these names; `ReviewOutcome` is also a name for other types in phase-04.md:719 and phase-22.md:396.
- Fix: Phase 25, section 3.2 sweep (after line 803): name the counts the break-glass sweep returns, and do the same for the support sweeps in section 3.8.

### V-25-01-h  STANDS  (first agent: LOW; refuter: LOW)
- Claim: Step 8 cites "(Open doubts 14)" for the requester learning that a case exists; Open doubts 14 is "The apply is a record", and the right item is 6.
- Evidence: phase-25.md:786 "(Open doubts 14)"; phase-25.md:1472 "14. **The apply is a record.**"; phase-25.md:1464 "6. **What the approvers and the requester learn.**". `grep -n "Open doubts 14" docs/plan/phase-25.md` gives :786, :1403 (7.8, correct) and :1472.
- Fix: Phase 25, section 3.2 step 8 (line 786): point the cross-reference to Open doubts 6.

### V-25-02-a  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: A missing `cef_forwarder` cursor row starts at 1970-01-01, but no step inserts it, so the cursor is never stored and each run re-sends the first batch.
- Evidence: phase-25.md:854 "a missing row starts at `1970-01-01`, nil id"; phase-25.md:858 "Advance the cursor (`last_occurred_at`, `last_audit_id`, `forwarded_total`, `version + 1`)", with no insert named. The grant at phase-25.md:539-540 gives `INSERT` on `admin.siem_cursors` to `akr_app`, so the insert is allowed but no step uses it. An upsert reading of "advance" is possible, so the gap is inferable.
- Fix: Phase 25, section 3.5 forwarder step 2 or 6 (lines 854 and 858): state that the first advance inserts the cursor row.

### V-25-02-b  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Step 1 takes a session-level advisory lock and no step releases it, so a pooled connection that returns keeps the lock and blocks later runs.
- Evidence: phase-25.md:853 "`pg_try_advisory_lock(hashtext('siem_forward'))`; if not acquired, return". The other locks in the plan are transaction-scoped (phase-25.md:780 "`pg_advisory_xact_lock(hashtext('break_glass:' || id))`"). `grep -n -i "advisory\|unlock" docs/plan/phase-25.md` gives :715, :780, :853, :1216 and :1231, and no unlock. No job rule in phase 00 releases a session lock (phase-00.md:851 uses `pg_advisory_xact_lock`).
- Fix: Phase 25, section 3.5 forwarder step 1 (line 853): name the unlock on every path, or take the transaction-scoped lock as section 3.2 does.

### V-25-02-c  PARTLY  (first agent: MEDIUM; refuter: LOW)
- Claim: Pull route refusals have no status or code: a `limit` above 500, and a malformed `after_at` or `after_id`.
- Evidence (refuted part): a malformed cursor has a general code: API.md:965 "| `E_BAD_REQUEST` | 400 | Malformed request |", and API.md:985 "Field or document validation". Stands: the outcome for `limit` above 500 is not stated (phase-25.md:862 "`limit` ≤ 500"; no refusal and no clamp). Note for the lead: API.md:46 sets the cursor-list maximum at 200 ("(maximum 200)"), so the 500 cap differs from the general convention, and the text does not say so.
- Fix: Phase 25, section 3.5 pull step 1 (line 862): state whether a limit above 500 is refused (with its code) or clamped, and reconcile the cap with API.md:46.

### V-25-02-d  STANDS  (first agent: LOW; refuter: LOW)
- Claim: `SiemForwarder` owns the job but has no struct and no method signature.
- Evidence: phase-25.md:841 "`SiemForwarder` owns the push to the bank's SIEM (akr-worker, job `siem_forward`)"; phase-25.md:1246 "`siem: Arc<SiemForwarder>`". The algorithm is numbered (phase-25.md:853-859), so the behaviour is stated, but no fields and no entry point are named. `grep -n SiemForwarder docs/plan/phase-25.md` gives :9, :839, :841, :1246 and no signature.
- Fix: Phase 25, section 3.5 before the forwarder steps (line 852): name the struct's fields and the job entry point with its parameters.

### V-25-02-e  STANDS  (first agent: MEDIUM; refuter: HIGH)
- Claim: `SiemLine` has `severity` and is "the output of `audit.siem_page`", but the function returns no severity column, while the CEF and JSON lines need one.
- Evidence: phase-25.md:653 "(the output of `audit.siem_page`, never a `case_id` field)" with `severity: SiemSeverity`; the RETURNS TABLE at phase-25.md:350-358 lists occurred_at, audit_id, action, actor_id, on_behalf_of, resource_type, resource_id, resource_key and details, and no severity; phase-25.md:846 "Severity: Info 3, Notice 5, Warning 7, Critical 9". The severity lives in the phase 02 catalogue (phase-02.md:549 `severity: Severity`). A column the CEF line needs is missing from the function the logic calls: HIGH.
- Fix: Phase 25, section 1.3 `audit.siem_page` RETURNS TABLE (lines 350-358): add the severity column, or state in section 3.5 that the forwarder reads it from the phase 02 catalogue.

### V-25-02-f  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The cursor advance (step 6) writes `admin.siem_cursors` on every batch and writes no audit event; section 3.13 has no kind and no exemption.
- Evidence: phase-25.md:858 "Advance the cursor (`last_occurred_at`, `last_audit_id`, `forwarded_total`, `version + 1`)"; phase-25.md:1136 "Each mutation writes one event of the phase 02 catalogue". Neither section 3.5 nor 3.13 says the move is exempt, and phase-02.md:777 exempts only anchoring. Auditing every ten-second batch would flood the log, so the lead may want the exemption; the text must still say it.
- Fix: Phase 25, section 3.13 (after line 1136): state that a cursor move writes no event, or name a kind for it.

### V-25-02-g  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Dead-letter resolution has columns, an index and a grant, but no route, no repository method and no audit kind.
- Evidence: phase-25.md:328-329 (`resolved_at`, `resolved_by`); phase-25.md:542 "GRANT UPDATE (resolved_at, resolved_by) ON admin.siem_dead_letters TO akr_app;"; phase-25.md:663 "`dead_letter(tx, &DeadLetter)`, `status() -> SiemStatus`" (no resolve method); the route table at phase-25.md:668-690 has no dead-letter row; section 3.13 has no kind. The open-dead-letter tile (phase-25.md:967) depends on the resolve operation. `grep -n "resolv" docs/plan/phase-25.md` gives :328-332, :542, :620 (a column list), :967 and no operation.
- Fix: Phase 25, section 2 (line 663) and section 3.13: add the resolve method, its route and its audit kind, or remove the grant and state that dead letters stay open.

### V-25-02-h  REFUTED  (first agent: LOW; refuter: none)
- Claim: `DeadLetter` is declared nowhere; its fields follow the table.
- Evidence: the table names every field: phase-25.md:320 "CREATE TABLE IF NOT EXISTS admin.siem_dead_letters (", with audit_id (:322), occurred_at (:323), action (:324), reason_code (:325) and attempts (:326). A type whose fields are named in a table of the plan needs no separate struct (refutation rule 1); the method at phase-25.md:663 takes `&DeadLetter`, and the table gives its fields.

### V-25-02-i  STANDS  (first agent: LOW; refuter: MEDIUM)
- Claim: The SIEM client certificate comes from OpenBao `pki/akr-worker-siem`, but no phase defines a PKI mount or role.
- Evidence: phase-25.md:848 "client certificate from OpenBao `pki/akr-worker-siem`". `grep -rn -i pki docs/plan/*.md docs/*.md` finds no PKI engine or role in phase 00, 01 or 02; phase-00.md:383 names OpenBao "(transit, KV)" only; docs/plan/research.md:433 mentions PKI only in a parity row. The transport needs a certificate source that the plan never defines: MEDIUM, because the text names the source and only its definition is missing.
- Fix: Phase 00 section 7.x (the OpenBao engines and roles) or phase 25 section 3.5 transport (line 848): name the PKI mount and role that issue the forwarder's client certificate.

### V-25-02-j  STANDS  (first agent: LOW; refuter: LOW)
- Claim: The header says the pull handler reads through `SiemPageReader`, a name declared nowhere; sections 2 and 4.3 use `SiemRepository`.
- Evidence: phase-25.md:9 "the pull handler of `SiemForwarder` reads through `SiemPageReader`"; phase-25.md:663 "`SiemRepository`: `page(after: (DateTime<Utc>, Uuid), limit, kinds: &[&str])`"; phase-25.md:1245 "`siem_pages: Arc<dyn SiemRepository>`". `grep -rln SiemPageReader docs/plan/*.md` gives phase 25 and roadmap.md:176, with no declaration.
- Fix: Phase 25, header line 9: replace `SiemPageReader` with `SiemRepository`, so the plan uses one name.

### V-25-03-a  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Request step 1 refuses a vendor user who is not `human`, not `active`, holds another role or has an open session, with no status or code.
- Evidence: phase-25.md:931 "holds no other role" and "has no open session (unique index `uq_support_sessions_open`)". The closed sub-code lists at phase-25.md:982-983 contain `hours_over_limit` (for step 2) and `session_not_requested` (for Approve) only. `grep -n "hours_over_limit" docs/plan/phase-25.md` gives :932 and :982. Phase 04 uses `role_not_grantable` for the same kind of user precondition on a role-grant change request (phase-04.md:955), but phase 25 does not say it reuses that code.
- Fix: Phase 25, section 3.8 Request step 1 (line 931): give the code (and sub-code) for each of the four refusals, reusing the 3.11 list or phase 04's `role_not_grantable` if that is intended.

### V-25-03-b  REFUTED  (first agent: MEDIUM; refuter: none)
- Claim: Approve has no step-up consumption, although the route row says SU `access_change`.
- Evidence: phase-01.md:707 "(normative: every SU route of every later phase calls `consume`; DR-29)". Phase 25 marks the route SU (phase-25.md:686 "**SU** `access_change`") and its sign and break-glass approve steps name the consume (phase-25.md:738 and :779). The general rule covers the missing step. The action constant is still undeclared; that is V-25-01-e.

### V-25-03-c  PARTLY  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: End and expiry is prose with no numbered steps and no status code for ending a session that is not `active` or `requested`; the revocation order is stated only as "in the same transaction".
- Evidence (refuted part): the order is stated in one sentence at phase-25.md:944: "revoke the vendor's sessions and tokens through the phase 01 `SessionRevoker` in the same transaction, set status `ended` or `expired`, `ended_at`, `ended_by`". Stands: no code for ending a session that is not active or requested. The closed sub-code list gives `session_not_requested` for Approve only (phase-25.md:983). `grep -n E_NOT_ALLOWED_IN_STATE docs/plan/phase-25.md` gives :715, :739, :781, :899, :936 and :983, and nothing for End.
- Fix: Phase 25, section 3.8 End and expiry (line 944): name `E_NOT_ALLOWED_IN_STATE` (with its sub-code) for a session that is not `active` or `requested`.

### V-25-03-d  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: A session still `requested` after one hour is set `refused_timeout` with no audit event; section 3.13 has no kind for it.
- Evidence: phase-25.md:944 "A session still `requested` after one hour is set `refused_timeout` (`ended_at` set, no grant)." The four support-session kinds at phase-25.md:1160-1163 are requested, started, ended and expired; `support_session.expired` is "time box passed (sweep; grant revoked)" (:1163), which does not describe this move. `grep -n refused_timeout docs/plan/phase-25.md` gives :236, :249, :496 and :944, and no audit kind.
- Fix: Phase 25, section 3.13 (after line 1163): name a kind for the refused timeout with its SIEM flag, or state which existing kind it uses.

### V-25-03-e  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The VS grant insert (Approve step 3) has no repository method; `RoleGrantWriter` is named in the struct and declared nowhere.
- Evidence: phase-25.md:918 "roles: Arc<dyn RoleGrantWriter>"; step 3 at phase-25.md:938 "Insert `admin.role_grants` with `user_id = vendor`". `SupportSessionRepository` (phase-25.md:662) has no grant insert. Phase 03 `GrantRepository` (phase-03.md:440-441) lists "revoke(tx, grant_id, user_id, actor)" and "revoke_all_for_user(tx, user_id, actor)" and no insert; phase 01 (phase-01.md:450) names only "insert_bootstrap". `grep -rln RoleGrantWriter docs/plan` gives phase 25 only. The columns are complete in step 3, so the method is inferable.
- Fix: Phase 25, section 2 `SupportSessionRepository` (line 662): name the grant insert method with its parameters, or add a trait for it in section 2.

### V-25-03-f  STANDS  (first agent: LOW; refuter: LOW)
- Claim: `SessionRevoker` (a struct field and the End step) is a type no plan file defines; phase 01 names its session type `SessionService`.
- Evidence: phase-25.md:919 "sessions_cache: Arc<dyn SessionRevoker>"; phase-25.md:944 "through the phase 01 `SessionRevoker`"; phase-01.md:582 "**`SessionService`** (`domain::identity::session`, over Kratos and Valkey)"; phase-01.md:457 "revoke(tx, id, user_id, actor, reason) -> bool; revoke_all_for_user(tx, user_id,". `grep -rln SessionRevoker docs/plan docs research` gives phase 25 only.
- Fix: Phase 25, section 3.8 struct (line 919) and End (line 944): use the phase 01 name for the session revoke port, or define the port in section 2.

### V-25-04-a  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `policy_bundle_hash_mismatch` is in the CHECK and the closed list, but no verify step raises it; the policy hash is computed without a comparison.
- Evidence: phase-25.md:181 "'version_not_newer', 'policy_bundle_hash_mismatch'" (the only line naming the code). Step 3 at phase-25.md:895 "Compute the policy bundle hash (SHA-256 over the canonical bundle, phase 03)" gives no expected value, and the checks at :893 are (a) to (f). The expected hash is available in principle: phase-03.md:546 "the release notes carry it" (the canonical bundle hash).
- Fix: Phase 25, section 3.6 verify step 3 (line 895): name the expected hash the bundle declares, and add the comparison with its place in the order (a) to (f).

### V-25-04-b  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Import step 1 has no status or code when `bundle_ref` names no directory, or the directory has no `manifest.json`.
- Evidence: phase-25.md:888 "`bundle_ref` must match the pattern of `0081` and name a directory in the import volume"; phase-25.md:889 reads "the manifest digest read from `manifest.json`" in the same request, before the 202. The general table has `E_BAD_REQUEST` (API.md:965, "Malformed request") and `E_VALIDATION` (API.md:985), but the step names neither, and a missing directory is not a malformed request. `grep -n "manifest.json" docs/plan/phase-25.md` gives :889 only.
- Fix: Phase 25, section 3.6 Import step 1 (line 888): name the code for a missing directory and for a directory without `manifest.json`.

### V-25-04-c  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The verification report is written as evidence in verify step 5, but no method is named; phase 02 gives the worker `put_generated`.
- Evidence: phase-25.md:897 "write the verification report as evidence (kind `package`, owner kind `release`, R10, classification internal)". Phase 02 names the worker method: phase-02.md:876 "`put_generated(tx, GeneratedEvidence, actor) -> EvidenceObject` (akr-worker; render output, snapshots, certificates," and :877 "packages". Section 3.1 step 4 names the call (phase-25.md:723); section 3.6 does not. Inferable from phase 02, hence MEDIUM.
- Fix: Phase 25, section 3.6 verify step 5 (line 897): name `EvidenceStore::put_generated` with its GeneratedEvidence fields, as section 3.1 step 4 does.

### V-25-04-d  PARTLY  (first agent: LOW; refuter: LOW)
- Claim: `VerificationResult`, `ReleaseCurrency` and `BundleTrustRoots` are named and declared nowhere.
- Evidence (refuted parts): `VerificationResult`'s fields are named in step 5, phase-25.md:897: "fill `cosign_result` (...), `notes` (...), `rollback_plan_sha256`, `policy_bundle_hash`, `policy_matrix_diff`, `evidence_id`, `verified_at`". `BundleTrustRoots`'s source is named, phase-25.md:1246: "`bundle_trust: Arc<BundleTrustRoots>` (the phase 00 verifier keys, read from OpenBao path `kv/release/trust`)".
  Stands: `ReleaseCurrency` has no method named (phase-25.md:877 "currency: Arc<dyn ReleaseCurrency>"; the tile at :903 and :968 describes its result, not a method). None of the three is declared as a Rust type.
- Fix: Phase 25, section 2 (line 660) or section 3.6 struct (line 877): name the one method of `ReleaseCurrency` and what it returns.

### V-25-04-e  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Verify step 1 moves the release to `verifying` and writes no audit event.
- Evidence: phase-25.md:893 "1. Status `verifying`."; the release kinds in section 3.13 (phase-25.md:1155-1159: imported, verified, refused, proposed, applied) have no kind for the move; phase-25.md:1136 "Each mutation writes one event of the phase 02 catalogue". `grep -n verifying docs/plan/phase-25.md` gives :176 (the CHECK), :893 (the step) and :1279 (a hook name), and no audit kind.
- Fix: Phase 25, section 3.13 (after line 1159) or section 3.6 step 1 (line 893): name a kind for the move to `verifying`, or state that the transient move writes none.

### V-25-04-f  PARTLY  (first agent: LOW; refuter: LOW)
- Claim: The route rows for propose and applied have no IF marker, while their service signatures take `expected`.
- Evidence (refuted part): the general rule requires If-Match on every versioned mutation: API.md:43 "Mutations on versioned resources require `If-Match: "<version>"`". The `expected` parameter is that rule, not a conflict with it, and the release row has a version (phase-25.md:649 `version`).
  Stands: the two rows omit the marker. phase-25.md:682 ends "202; IDEM (added)" and :683 "**SU** `approval`; IDEM (added)", while the access-review rows carry IF (:673, :674).
- Fix: Phase 25, section 2 route table (lines 682-683): add the IF marker to both rows.

### V-25-05-a  STANDS  (first agent: MEDIUM; refuter: HIGH)
- Claim: Point 1 ties the Kratos OIDC flag to `upstream_mode` in a deployment file, but the closing paragraph says the mode switch needs no redeploy.
- Evidence: phase-25.md:823 "`selfservice.methods.oidc.enabled: true` only when the bank runs `upstream_mode = oidc`"; phase-25.md:837 "the mode switch needs no redeploy". The flag sits in `deploy/kratos/kratos.yml` (phase-25.md:822), a static file, so the two texts give different rules for the same switch. `grep -n "selfservice.methods.oidc.enabled" docs/plan/phase-25.md` gives :823 only, and no 7.x block settles it. HIGH by the rubric (contradiction).
- Fix: Phase 25, section 3.4 point 1 (line 823) or the closing paragraph (line 837): say whether the flag is always on and only the BFF button depends on the mode, or whether a mode switch restarts Kratos.

### V-25-05-b  REFUTED  (first agent: MEDIUM; refuter: none)
- Claim: An upstream login whose subject is not linked is refused "with a generic error", with no HTTP status or code.
- Evidence: phase-01.md:825 "`E_UNAUTHENTICATED` (one body for every failed sign-in stage)"; phase-01.md:865 "Sign-in answers: one 401 body for a wrong password, an unknown identifier, a disabled, expired or on-leave account". The generic error of phase-25.md:825 ("refused with a generic error") is that one 401 body.

### V-25-05-c  STANDS  (first agent: LOW; refuter: LOW)
- Claim: The header lists only `support-bundle` and `oidc upstream check` as new `akr-ctl` subcommands; section 4.5 adds `identity link-upstream` and `unlink-upstream`, which section 3.4 uses.
- Evidence: phase-25.md:9 "new subcommands `support-bundle` and `oidc upstream check`"; phase-25.md:1259 "`akr-ctl identity link-upstream` and `unlink-upstream` are new subcommands". `grep -n "link-upstream" docs/plan/phase-25.md` gives :825, :833 and :1259, and no header line.
- Fix: Phase 25, header "Binaries touched" (line 9): add the two identity subcommands.

### V-25-05-d  PARTLY  (first agent: LOW; refuter: LOW)
- Claim: `OidcUpstreamConfig` (the AppState field and the BreakGlassService field) is defined in no plan file; section 3.4 describes a configuration with no type.
- Evidence (refuted part): section 3.4 states the fields in prose: phase-25.md:823 "the provider list has exactly one entry with `provider: generic`" and phase-25.md:824 "`issuer_url`, `client_id` and the client secret". Stands: the type name is never declared, and it differs from the section name `OidcUpstream` (phase-25.md:818 heading and :820). `grep -rln OidcUpstreamConfig docs/plan docs research` gives phase 25 only. The AppState field is at phase-25.md:1245, "`oidc_config: Arc<OidcUpstreamConfig>`".
- Fix: Phase 25, section 3.4 (after line 820): name the type that section 3.4 describes, and use that name in section 4.3 (line 1245).

### V-25-06-a  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The tile `break_glass_review_overdue` counts "grants with `review_overdue`", but a grant has no such status; `review_overdue` is a log kind.
- Evidence: phase-25.md:971 "count of grants with `review_overdue` and no review"; phase-25.md:150 "'lapsed', 'reviewed', 'review_overdue'" (the log kinds); the grant status values are closed at phase-25.md:655 (`BreakGlassStatus { Requested, Active, Ended, Expired, Lapsed }`). The sweep writes the log kind (phase-25.md:803). A reader can build the query on the log, so MEDIUM, not HIGH.
- Fix: Phase 25, section 3.10 row `break_glass_review_overdue` (line 971): state the query over `admin.break_glass_log` rows of kind `review_overdue` whose grant has `reviewed_at IS NULL`.

### V-25-06-b  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Lag is `now() - last_occurred_at` of the cursor, so on a quiet system the lag grows with idle time and the tile turns critical with no backlog.
- Evidence: phase-25.md:859 "The lag is `now() - last_occurred_at` of the cursor"; phase-25.md:966 "`now() - siem_cursors.last_occurred_at`". The cursor moves only past forwarded rows (phase-25.md:857 "The cursor moves only past lines the collector accepted or that were dead-lettered"), and the critical line is `siem.max_lag_s` (phase-25.md:570, default 900). No term for pending rows exists in either text.
- Fix: Phase 25, section 3.10 row `siem_lag` (line 966) and section 3.5 step 7 (line 859): define lag as the age of the oldest audit row after the cursor, or zero when none is pending.

### V-25-06-c  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The critical condition of `release_currency` cannot occur: a release is refused only in verify, and `superseded` is set only by an apply, so it always has an applied successor.
- Evidence: phase-25.md:968 "applied release refused or superseded with no applied successor for 30 days"; phase-25.md:896 "On any failure: status `refused`"; phase-25.md:901 "move the previously `applied` row to `superseded`". The guards at phase-25.md:466-476 allow an applied row to move only to `superseded` ("an applied release moves only to superseded") and keep a refused row refused. Neither state arises for an applied release.
- Fix: Phase 25, section 3.10 row `release_currency` (line 968): replace the critical condition with one that can occur, or add the transition that produces the state.

### V-25-06-d  STANDS  (first agent: LOW; refuter: LOW)
- Claim: Four tiles name no count method or query; the anchor tile has no repository at all.
- Evidence: phase-25.md:969 "count of sessions `active` (and `requested` older than 1 hour)" and :970 "count of grants `active`". Section 2 has no count method on `SupportSessionRepository` (phase-25.md:662), `BreakGlassRepository` (:659) or `AccessReviewRepository` (:658), and no repository for `anchor_timestamps`. The Source column states each count in words, so only the method name is missing.
- Fix: Phase 25, section 2 (lines 658-662): name a count method on each repository the tiles use, and a repository method for the anchor tile.

### V-25-07-a  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Sweep step 2 uses `ON CONFLICT (source_key) DO NOTHING` against a partial unique index, so PostgreSQL refuses the statement as written.
- Evidence: phase-25.md:748 "ON CONFLICT (source_key) DO NOTHING"; phase-24.md:75 "CREATE UNIQUE INDEX IF NOT EXISTS uq_registers_source_key ON admin.registers (source_key) WHERE source_key IS NOT NULL;". The same form appears in phase 26 (phase-26.md:277 and :444), which the lead should check too. `grep -n "ON CONFLICT" docs/plan/phase-25.md` finds the admin.registers conflict at :748 only (the rest are seeds).
- Fix: Phase 25, section 3.1 sweep step 2 (line 748): add the index predicate to the conflict target, and check phase 26 lines 277 and 444.

### V-25-07-b  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Snapshot step 5 inserts one item per grant and step 6 sets the snapshot columns, but section 2 names no method for either.
- Evidence: phase-25.md:724 "Insert one `admin.access_review_items` row per grant"; phase-25.md:725 "Update the review: status `open`, `evidence_id`, `evidence_sha256`, `item_count`"; phase-25.md:658 lists "`set_status(tx, id, ReviewState, expected_version)`" (status only) and no item insert or snapshot update.
- Fix: Phase 25, section 2 `AccessReviewRepository` (line 658): name the item insert and the snapshot update with their parameters.

### V-25-07-c  STANDS  (first agent: LOW; refuter: LOW)
- Claim: Names differ: `RoleGrantReader` is undefined, `DecideItem` is `DecideAccessReviewItem` in section 2, and `ReviewState` is `ReviewStatus`.
- Evidence: phase-25.md:702 "grants: Arc<dyn RoleGrantReader>"; phase-25.md:707 "cmd: DecideItem"; phase-25.md:656 "`DecideAccessReviewItem { decision: ReviewDecision"; phase-25.md:645 "status: ReviewStatus"; phase-25.md:658 "`set_status(tx, id, ReviewState, expected_version)`". `grep -rln RoleGrantReader docs/plan` gives phase 25 only; phase 03 has `GrantRepository` (phase-03.md:440).
- Fix: Phase 25, sections 2 and 3.1: use one name each, for example phase 03's `GrantRepository` for the grant reader.

### V-25-07-d  STANDS  (first agent: MEDIUM; refuter: HIGH)
- Claim: Sweep step 2 writes the overdue audit "and no `access_review.overdue` event yet", which needs a read of the audit trail that the application role cannot make.
- Evidence: phase-25.md:748 "and no `access_review.overdue` event yet"; phase-25.md:664 "the code has no `SELECT` on `audit.audit_log`". The application role inherits only four columns of `audit.audit_log` (phase-02.md:308 "GRANT SELECT (id, occurred_at, prev_hash, entry_hash) ON audit.audit_log", inherited through the membership at phase-02.md:44), and none of them is the action or the resource, so the check cannot run. The guard the text needs does not exist; the register insert's unique source_key is the only guard available. A step that needs a read the plan forbids is a contradiction: HIGH.
- Fix: Phase 25, section 3.1 sweep step 2 (line 748): take the audit write from the register insert's returned row, not from a read of the audit trail.

### V-25-07-e  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Items move to `superseded` in decide step 4 and in sweep step 1 with no audit event; section 3.13 has no kind for it.
- Evidence: phase-25.md:731 "set the item to `superseded` (no decider)"; phase-25.md:747 "set `superseded` (once per item)". The access-review kinds in section 3.13 (phase-25.md:1140-1145: started, snapshotted, item_decided, revoke_proposed, signed, overdue) name no superseded move, and phase-25.md:1136 says each mutation writes one event.
- Fix: Phase 25, section 3.13 (after line 1145): name a kind for the item move with its SIEM flag, or state that it writes none.

### V-25-07-f  STANDS  (first agent: MEDIUM; refuter: HIGH)
- Claim: The snapshot evidence has owner kind `access_review`, which no evidence owner registry lists; phase 25 does not add it, although it adds `release` in phase 02.
- Evidence: phase-25.md:723 "owner kind `access_review`"; phase-02.md:847 "`owner_kind` must be in the evidence owner registry (`pack` now; every owning phase adds its kind)". Phase 02 `put_generated` runs "steps 1-3 of `stage`" (phase-02.md:877 wording: "steps 1-3 of `stage` with the generated-type list"), so the snapshot is refused at the registry check. phase-02.md:1262 registers `release` for this phase, and `grep -n access_review docs/plan/phase-02.md` gives no line. HIGH: the snapshot cannot be stored as written.
- Fix: Phase 02 section 7.1 (evidence owner registry), or phase 25 section 4.5: add the `access_review` owner row, as phase 02 line 1262 does for `release`.

### V-25-08-a  STANDS  (first agent: MEDIUM; refuter: HIGH)
- Claim: A malformed article (schema failure or a bidirectional control character) has no defined outcome; none of the nine refusal codes covers help content.
- Evidence: phase-25.md:960 "refuses bidirectional-control characters outside the Arabic and Latin ranges (SP-70)"; phase-25.md:995 "A refused release bundle is not an HTTP error of the verify route (the route answers 202; the job records `refusal_code`, section 3.6)". Section 6 says otherwise: phase-25.md:1338 "is refused with `E_VALIDATION` `article_text_invalid`", a code that the closed list at phase-25.md:982 does not contain and that contradicts line 995. The release outcome (refuse the release or skip the article) is not stated. HIGH: two texts contradict and the code is undefined.
- Fix: Phase 25, section 3.9 (line 960) and section 3.6 step 5 (line 897): state whether a malformed article refuses the release (with a code added to the 0081 CHECK) or is skipped, and make section 6 (line 1338) agree.

### V-25-08-b  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Phase 00 lists the bundle contents in prose and names no file or folder; the help folder and `notes.json` are not in it.
- Evidence: phase-00.md:749 "notices (third-party notices file; the store's Corresponding Source, LICENSE, NOTICE, CREDITS), test evidence and the rollback plan; a manifest of SHA-256 digests". No `help/` folder and no `notes.json` appear in phase 00 (`grep -n "help/\|notes.json" docs/plan/phase-00.md` gives none). Phase 25 reads `help/*.json` (phase-25.md:960) and `notes.json` (phase-25.md:897).
- Fix: Phase 00 bundle build item (phase-00.md:747-751): add the help folder and the release notes file, or name both in phase 25 section 3.6.

### V-25-08-c  STANDS  (first agent: LOW; refuter: LOW)
- Claim: `(SP-70)` names the wrong requirement; SP-70 is the coding rules, and the bidirectional scan is UX-20.
- Evidence: research/30-security-privacy.md:1443 "| SP-70 | Apply the stack coding rules: `unsafe_code` forbidden"; research/31-analyst-ux.md:920 "| UX-20 | Scan incoming text for bidirectional controls" with source "File 30 SP-70". The research row itself cites SP-70 as the source, so the wrong citation is in the research too. The plan's citation is phase-25.md:960.
- Fix: Phase 25, section 3.9 (line 960): cite UX-20; the lead may also correct the source column of research/31-analyst-ux.md:920.

### V-25-08-d  STANDS  (first agent: LOW; refuter: LOW)
- Claim: "Route 19 (`GET /help/articles?kind=`)" refers to a route number that exists nowhere.
- Evidence: phase-25.md:960 "Route 19 (`GET /help/articles?kind=`)". The phrase "route 19" appears elsewhere only in phase-24.md:711, as a label for a different route. The phase 25 route table (phase-25.md:668-690) has no numbering.
- Fix: Phase 25, section 3.9 (line 960): name the route by its path only.

### V-25-09-a  STANDS  (first agent: LOW; refuter: LOW)
- Claim: Block 7.6 writes the KV entry as `akr/keys/auth-hmac`, with the mount prefix that 7.1 and phase 00 forbid.
- Evidence: phase-25.md:1384 "The KV entry `akr/keys/auth-hmac` (DR-356)"; phase-25.md:1351 "KV paths are relative to the one mount `akr/`"; phase-00.md:894 "is written without a mount prefix". `grep -n "akr/keys\|kv/" docs/plan/phase-25.md` finds 822, 1246, 1351, 1352, 1384 and 1386. The first four are the spellings 7.1 corrects; the last two are in 7.6.
- Fix: Phase 25, section 7.6 (lines 1384 and 1386): write the path relative to the mount, as 7.1 requires, and say that the operator types the mount prefix when using the CLI.

### V-25-10-a  STANDS  (first agent: LOW; refuter: LOW)
- Claim: Block 7.3 cites a test in section 6 for the row-count exclusion, and section 6 has no such test.
- Evidence: phase-25.md:1369 "Test (section 6): the bundle fixture's row-count list contains none of the seven names." Section 6 (phase-25.md:1308-1345) has no row-count assertion. `grep -n -i "row-count\|row count\|row_count" docs/plan/phase-25.md` gives :947, :1365 and :1369 only.
- Fix: Phase 25, section 6 (add the assertion to the database or bundle test list), or section 7.3 (line 1369): drop the reference.

### V-25-12-a  STANDS  (first agent: HIGH; refuter: HIGH)
- Claim: No route uploads the `rollout.json` evidence for owner kind `release`, so the operator cannot create the object the apply checks.
- Evidence: phase-25.md:1400-1401 "The operator first uploads the `rollout.json` that phase 30's upgrade script writes"; the route table at phase-25.md:668-690 has no evidence row (`grep -n -i evidence` finds none in the rows); API.md:236 "there is no generic evidence route (DR-380)"; phase-02.md:839 says `stage` serves "the upload routes of phases 17 and 18". A route the logic depends on is in no route table: HIGH.
- Fix: Phase 25, section 2 route table (after line 683): add the upload route for owner kind `release` with its scope, Cedar action and markers, or name the existing route that accepts it.

### V-25-12-b  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `rollout_evidence_id` is not carried by the DTO, the service signature or the owner read.
- Evidence: phase-25.md:1400 "`POST /admin/releases/{id}/applied` takes an optional `rollout_evidence_id`"; phase-25.md:656 "`RecordReleaseApplied { }`"; phase-25.md:882 "record_applied(ctx, idem, release, expected, header)" (no parameter for it). The owner read is `EvidenceRepository::get` (phase-02.md:616 "get(id) -> Option<EvidenceObject>"), which the block does not name. The field is named in 7.8, so the gap is inferable.
- Fix: Phase 25, section 2 DTO (line 656) and section 3.6 signature (line 882): carry the optional `rollout_evidence_id`, and name the evidence read that the owner check uses.

### V-25-12-c  STANDS  (first agent: LOW; refuter: LOW)
- Claim: The order of the owner check in the apply is not stated: before or after the status test and the step-up, and inside or outside the transaction.
- Evidence: phase-25.md:901 "the status must be `approved`" (the Applied paragraph has no numbered steps); phase-25.md:1402 "The route checks that the evidence has that owner (else `E_VALIDATION`)".
- Fix: Phase 25, section 3.6 Applied (line 901): place the owner check among the status and step-up checks and say whether it runs inside the transaction.

## Summary

- REFUTED: 3 (V-25-02-h, V-25-03-b, V-25-05-b)
- STANDS: 42
- PARTLY: 8 (V-25-01-b, V-25-01-c, V-25-01-g, V-25-02-c, V-25-03-c, V-25-04-d, V-25-04-f, V-25-05-d)
- Total: 53 unique Gap IDs. Each ID appears twice in the first agent's file (under its item and in its summary table); each was handled once.
- Severity basis: HIGH for a missing table, column, route, grant, method or step the logic needs, and for two texts that contradict; MEDIUM for a part missing but inferable from nearby text, and for a business status move with no audit event; LOW for names, numbers, cross-references and edge cases with a stated default. Raised to HIGH: V-25-01-a, V-25-02-e, V-25-05-a, V-25-07-d, V-25-07-f, V-25-08-a. Raised to MEDIUM: V-25-02-i.

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| GAP V-25-01-a | STANDS | HIGH | Line 768 says activate calls CaseExistence; step 8 and DR-233 say activation does not | Section 3.2, line 768 |
| GAP V-25-01-b | PARTLY | MEDIUM | End's refusal for a grant not active or requested has no code; Review's state check is covered by `not_reviewable` | Section 3.2, End (line 796) |
| GAP V-25-01-c | PARTLY | MEDIUM | An approver holding both roles who names none has no code; a named role not held reads as a fallback | Section 3.2, Approve step 5 (line 783) |
| GAP V-25-01-d | STANDS | MEDIUM | The requester's end of a `requested` grant (lapsed) writes no audit event | Section 3.2, End (line 796); section 3.13 (line 1152) |
| GAP V-25-01-e | STANDS | MEDIUM | StepUpAction constants are not declared; the request action's name is given nowhere | Section 3.2 before Request, or section 3.12 |
| GAP V-25-01-f | STANDS | LOW | Unused `cfg` field of type `OidcUpstreamConfig`, which is defined nowhere | Section 3.2 struct (line 757) |
| GAP V-25-01-g | PARTLY | LOW | Sweep counts are named for the access-review sweep only; the break-glass and support sweeps name none | Section 3.2 sweep (after line 803); section 3.8 |
| GAP V-25-01-h | STANDS | LOW | Cross-reference points to Open doubts 14; the right item is 6 | Section 3.2 step 8 (line 786) |
| GAP V-25-02-a | STANDS | MEDIUM | No step inserts the first `cef_forwarder` cursor row | Section 3.5, steps 2 and 6 (lines 854, 858) |
| GAP V-25-02-b | STANDS | MEDIUM | Session-level advisory lock is never released | Section 3.5, step 1 (line 853) |
| GAP V-25-02-c | PARTLY | LOW | Outcome for a limit above 500 is not stated; a malformed cursor is covered by E_BAD_REQUEST | Section 3.5, pull step 1 (line 862) |
| GAP V-25-02-d | STANDS | LOW | SiemForwarder has no struct fields and no entry point signature | Section 3.5, before the forwarder steps (line 852) |
| GAP V-25-02-e | STANDS | HIGH | The CEF line needs severity; `audit.siem_page` returns no severity column | Section 1.3 `audit.siem_page` (lines 350-358), or section 3.5 |
| GAP V-25-02-f | STANDS | MEDIUM | The cursor advance writes no audit event, and section 3.13 says each mutation writes one | Section 3.13 (after line 1136) |
| GAP V-25-02-g | STANDS | MEDIUM | Dead-letter resolution has a grant and columns, but no route, method or audit kind | Section 2 (line 663); section 3.13 |
| GAP V-25-02-i | STANDS | MEDIUM | The PKI mount and role for the SIEM client certificate are defined nowhere | Phase 00 OpenBao section, or section 3.5 transport (line 848) |
| GAP V-25-02-j | STANDS | LOW | Header names `SiemPageReader`; the plan uses `SiemRepository` | Header, line 9 |
| GAP V-25-03-a | STANDS | MEDIUM | Request step 1 refusals (not human, not active, another role, open session) have no code | Section 3.8, Request step 1 (line 931) |
| GAP V-25-03-c | PARTLY | MEDIUM | Ending a session that is not active or requested has no code | Section 3.8, End and expiry (line 944) |
| GAP V-25-03-d | STANDS | MEDIUM | The move to `refused_timeout` writes no audit event | Section 3.13 (after line 1163) |
| GAP V-25-03-e | STANDS | MEDIUM | The VS grant insert has no repository method; `RoleGrantWriter` is undeclared | Section 2, `SupportSessionRepository` (line 662) |
| GAP V-25-03-f | STANDS | LOW | `SessionRevoker` is undefined; phase 01 names `SessionService` | Section 3.8, struct (line 919) and End (line 944) |
| GAP V-25-04-a | STANDS | MEDIUM | `policy_bundle_hash_mismatch` is never raised; no expected-hash comparison | Section 3.6, verify step 3 (line 895) |
| GAP V-25-04-b | STANDS | MEDIUM | A missing directory or manifest on import has no code | Section 3.6, Import step 1 (line 888) |
| GAP V-25-04-c | STANDS | MEDIUM | The verification report's evidence write names no method | Section 3.6, verify step 5 (line 897) |
| GAP V-25-04-d | PARTLY | LOW | `ReleaseCurrency` names no method; the other two types are described in the text | Section 2 (line 660) or section 3.6 struct (line 877) |
| GAP V-25-04-e | STANDS | MEDIUM | The move to `verifying` writes no audit event | Section 3.13 (after line 1159), or section 3.6 step 1 |
| GAP V-25-04-f | PARTLY | LOW | The propose and applied route rows lack the IF marker that API.md:43 requires | Section 2 route table (lines 682-683) |
| GAP V-25-05-a | STANDS | HIGH | The Kratos OIDC flag is tied to the mode, yet the mode switch is said to need no redeploy | Section 3.4, point 1 (line 823) or line 837 |
| GAP V-25-05-c | STANDS | LOW | Header omits `identity link-upstream` and `unlink-upstream` | Header, line 9 |
| GAP V-25-05-d | PARTLY | LOW | The type name `OidcUpstreamConfig` is never declared and differs from `OidcUpstream` | Section 3.4 (after line 820); section 4.3 (line 1245) |
| GAP V-25-06-a | STANDS | MEDIUM | The tile counts grants with a `review_overdue` status that no grant has | Section 3.10, row (line 971) |
| GAP V-25-06-b | STANDS | MEDIUM | Lag grows on a quiet system and turns critical with no backlog | Section 3.10 `siem_lag` (line 966); section 3.5 step 7 (line 859) |
| GAP V-25-06-c | STANDS | MEDIUM | The critical condition of `release_currency` cannot occur | Section 3.10, row (line 968) |
| GAP V-25-06-d | STANDS | LOW | Four tiles name no count method | Section 2 (lines 658-662) |
| GAP V-25-07-a | STANDS | MEDIUM | `ON CONFLICT (source_key)` lacks the partial index predicate, so the statement is refused | Section 3.1, sweep step 2 (line 748); phase 26 lines 277 and 444 |
| GAP V-25-07-b | STANDS | MEDIUM | Item insert and snapshot update have no repository method | Section 2, `AccessReviewRepository` (line 658) |
| GAP V-25-07-c | STANDS | LOW | `RoleGrantReader` undefined; `DecideItem` and `ReviewState` differ from section 2 names | Sections 2 and 3.1 (lines 645, 656, 658, 702, 707) |
| GAP V-25-07-d | STANDS | HIGH | The overdue guard needs an audit read that the plan forbids | Section 3.1, sweep step 2 (line 748) |
| GAP V-25-07-e | STANDS | MEDIUM | Item moves to `superseded` write no audit event | Section 3.13 (after line 1145) |
| GAP V-25-07-f | STANDS | HIGH | Owner kind `access_review` is not in the evidence owner registry, so the snapshot is refused | Phase 02 section 7.1; or phase 25 section 4.5 |
| GAP V-25-08-a | STANDS | HIGH | A malformed article has no outcome; section 6 contradicts line 995 and uses an undefined code | Section 3.9 (line 960) and section 3.6 step 5 (line 897) |
| GAP V-25-08-b | STANDS | MEDIUM | The help folder and `notes.json` are not in phase 00's bundle contents | Phase 00 bundle build item (lines 747-751) |
| GAP V-25-08-c | STANDS | LOW | SP-70 is cited for the bidirectional scan; the requirement is UX-20 | Section 3.9 (line 960) |
| GAP V-25-08-d | STANDS | LOW | "Route 19" has no referent in phase 25 | Section 3.9 (line 960) |
| GAP V-25-09-a | STANDS | LOW | Block 7.6 writes `akr/keys/auth-hmac`; 7.1 and phase 00 use paths relative to the mount | Section 7.6 (lines 1384, 1386) |
| GAP V-25-10-a | STANDS | LOW | Block 7.3 cites a section 6 test that does not exist | Section 6, or section 7.3 (line 1369) |
| GAP V-25-12-a | STANDS | HIGH | No route uploads the `rollout.json` evidence for owner kind `release` | Section 2 route table (after line 683) |
| GAP V-25-12-b | STANDS | MEDIUM | `rollout_evidence_id` is not carried by the DTO, the signature or the owner read | Section 2 DTO (line 656); section 3.6 signature (line 882) |
| GAP V-25-12-c | STANDS | LOW | The order and transaction of the owner check are not stated | Section 3.6, Applied (line 901) |
