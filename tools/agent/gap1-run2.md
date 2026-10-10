# Gap pass 1, run 2 (targets 03, 04; Haiku draft for lead review)

Scope: obligations that phases 04 to 31 place on phase 03 and phase 04. Run 1 (phases 00, 01, 02, section 7+) was read
first; no finding below contradicts it. Findings are grouped by target phase: phase 04 first, then phase 03. Line
numbers are in the Sources and Check lines only.

## Target phase 04

### G1-P04-01 WorkflowSubject has no check_before_decision method
- Severity: HIGH
- Category: Missing Logic
- Target: phase-04.md section 2 (trait `WorkflowSubject`, line 669) and section 3.4 step 6 (line 1062)
- Sources:
  - phase-18.md:1120 "Phase 04 `WorkflowSubject`**: extended with the method `check_before_decision`"
  - phase-19.md:1694 "the `check_before_decision` hook of phase 18 section 4.5 item 1" (RfiWorkflowSubject)
  - phase-20.md:968 "`FreezeWorkflowSubject` and `OpsTaskWorkflowSubject` implement `check_before_decision`"
  - phase-21.md:1254 report sign refused on the "phase 04 generic task route" (same hook family)
  - phase-04.md:1062 "other subjects register theirs" (the envelope promises the hook; the trait does not declare it)
- Check: grep -n "check_before_decision" docs/plan/phase-04.md -> lines 651 (ChangeTarget trait only), 896, 968 and 1062 (prose). The WorkflowSubject trait (lines 669-682) declares no such method.

Proposed 7.N text (to append to phase-04.md section 7+ as "### 7.1 WorkflowSubject: check_before_decision"):

    Section 2, trait WorkflowSubject, gains one method:
    async fn check_before_decision(&self, tx: &mut Tx, inst: &WorkflowInstance, actor: &AuthContext,
        task: &Task) -> Result<(), WorkflowError>;
    Default body: Ok(()). Subjects with no extra rule need no code.
    Section 3.4 step 6 reads: "Subject check before decision: the subject's `check_before_decision` runs inside the
    same transaction, before the action applies; a change request runs its `ChangeTarget::check_before_decision`
    through this step. A refusal rolls the action back." The phase 21 report route refusal (sub-code
    `report_sign_required`, phase 21 section 4.5 item 2) is one such check and is listed under this hook.

### G1-P04-02 No route or owner kind for change-request evidence
- Severity: HIGH
- Category: Missing Endpoint
- Target: phase-04.md section 2, route table (lines 776-806), and section 3.2 "Evidence" (line 842); owner kind in the phase 02 evidence owner registry
- Sources:
  - gap-items.md item (d): "Phase 04 has no change-request evidence upload route."
  - phase-04.md:833 "an `evidence_object` (the order or the list reference) is attached (`E_EVIDENCE_MISSING`)"
  - phase-04.md:842 "each item resolves through the `EvidenceResolver` registry (`evidence_object` phase 02"
  - phase-02.md:844 "`owner_kind` must be in the evidence owner registry ... every owning phase adds its kind"
  - phase-00.md:862 timers already use owner kind `change_request`, which the phase 02 registry does not list
  - tools/agent/b3-conventions.md (phase 20 review): "There is NO generic `POST /evidence`: uploads are per owner route"
- Check: grep -n "/evidence" docs/plan/phase-04.md -> none. grep -n "change_request" docs/plan/phase-02.md -> none. docs/API.md rows 655-661 (change requests) list no evidence route.

Proposed 7.N text ("### 7.2 Evidence upload for change requests"):

    Owner kind `change_request` is registered in the phase 02 evidence owner registry (owner id = change request id).
    Route: POST /change-requests/{id}/evidence, scopes as POST /change-requests/{id}/submit (config.author, admin.users,
    admin.policies, admin.system), Idempotency-Key, If-Match (request version). Body: a file with `kind` and
    `file_name`, staged through EvidenceStore::stage with owner_kind `change_request`. 201 with the evidence id.
    Allowed only while the request is `draft` (409 E_NOT_ALLOWED_IN_STATE after submit), so the evidence list is fixed
    at submit. Audit: evidence.staged (phase 02) only. The path follows the phase 20 pattern (POST /cases/{id}/evidence);
    the lead confirms it against API.md.

### G1-P04-03 Engine step 5 calls the step-up service with a bare class
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-04.md section 3.4 step 5 (line 1060)
- Sources:
  - phase-04.md:1060 "`StepUpService::consume(tx, action, class, resource_ref = "task:<id>")`"
  - phase-01.md:730 "`consume(tx, ctx, header, action: StepUpAction, resource_ref: Option<&str>)`"
  - phase-18.md:1133 "This phase declares its `StepUpAction` constants" (case.decide, decision)
  - tools/agent/b3-conventions.md (phase 18 review): consume takes a declared action constant, never a bare class
- Check: grep -n "consume(" docs/plan/phase-04.md -> line 1060 only, in the bare-class form. The step-up action names are already in the route table (lines 781-794: change.approve, change.reject, task.approve, task.reject, task.sign).

Proposed 7.N text ("### 7.3 Engine step 5 passes the declared step-up action"):

    Section 3.4 step 5 reads: "Deciding steps and every `sign`: `StepUpService::consume(tx, ctx, header, action,
    resource_ref = "task:<id>")`, where `action` is the declared StepUpAction constant of the act: `task.approve` and
    `task.reject` (class approval), `task.sign` (class signing); on the change routes `change.approve` and
    `change.reject` (class approval). A subject's own constants are passed the same way. 401 E_STEPUP_REQUIRED as
    before; the assertion id goes to tasks.stepup_id."

### G1-P04-04 Holders rule 5 (case_acl) is a placeholder
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-04.md section 3.5, item 5 (line 1079)
- Sources:
  - phase-04.md:1079 "5. `case_acl`: phase 18 (refused at publish until then)."
  - phase-18.md:1122 "`Holders::eligible` rule 5 becomes 'an active `casework.case_acl` row with `access = decide` on the instance's ..."
  - phase-18.md:1280 "Lead corrections (review 2026-10-09). ... `Holders` rule 5 adds role `M` for `case_report_approval`"
- Check: grep -n "case_acl" docs/plan/phase-04.md -> line 1079 (placeholder) and line 1244 (E_ASSIGNEE_UNAVAILABLE before phase 18). No role M condition anywhere in phase 04.

Proposed 7.N text ("### 7.4 Holders rule 5 (case_acl)"):

    Rule 5 reads: "`case_acl`: an active `casework.case_acl` row with `access = decide` on the instance's case; when the
    instance's definition is `case_report_approval`, the principal also holds role `M`. The inbox, the Task adapter and
    the engine read this one rule. Until phase 18 ships the case subject, a definition that names `case_acl` is refused
    at publish (E_ASSIGNEE_UNAVAILABLE, section 2 error table)."

### G1-P04-05 Approver ids for the DR-92 masking have no port
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-04.md section 2 (ChangeRequestService, line 854) and section 3.2
- Sources:
  - phase-11.md:1099 "The API computes `approver_ids` from the open change requests of the version"
  - phase-11.md:1490 "DR-92 needs the change-request service of phase 04 to expose the approver ids ... that ..."
  - phase-04.md:1405 "`ChangeRequest { ..., parties: Set<User> }` (`parties` = maker, deciders and the current holders of ..."
- Check: grep -n -i "approver" docs/plan/phase-04.md -> no read method; grep -n "approver_ids" docs/plan/phase-04.md -> none.

Proposed 7.N text ("### 7.5 Approver ids of open change requests (DR-92)"):

    ChangeRequestService gains one read method (name UNCLEAR: phase 11 names none) that returns the user ids of the
    `parties` of every open change request whose object_key names a given object (for a mapping replay, the mapping
    version). Read only; no step-up; no audit row. Phase 11 uses the result for the Cedar attribute
    resource.approver_ids. UNCLEAR: whether the maker is in the returned list. Phase 11 asks for "approver ids"; `parties`
    includes the maker. The lead decides.

### G1-P04-06 Validation pass sets inventory dates; phase 04 says a person enters them
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-04.md section 3.9 "Update" (line 1347) and the validation-date sentence (line 1355)
- Sources:
  - phase-24.md:702 "When the verdict is `pass`, `InventoryService::update` (phase 04) sets `last_validated_at = now()` and `next_validation_due`"
  - phase-24.md:1299 "A `pass` calls `InventoryService::update` ... Phase 04 is asked to accept the call."
  - phase-27.md:783 "`next_validation_due` of the family is set at creation and moved by the review calendar"
  - phase-04.md:1355 "Next validation dates are entered by the model-risk officer here"
- Check: grep -n "next_validation_due" docs/plan/phase-04.md -> DDL, create (line 1339), update fields, and line 1355 (manual entry). No system or validation-pass path is described.

Proposed 7.N text ("### 7.6 Validation pass updates inventory dates"):

    Section 3.9 "Update" gains: "A validation pass (phase 24 section 3.6 step 8) may call Update in its own transaction
    with the validator's context. It sets last_validated_at and next_validation_due and nothing else; the status does
    not change, so no step-up applies to that call. The pass locks the item row first (phase 24 lock order). Audit
    inventory_item.updated {fields, status_from, status_to} with status_from = status_to." Replace the sentence with: "The model-risk officer enters a date by hand, or a validation pass sets it from the review interval."

### G1-P04-07 role_grant has no revoke operation; platform_release target and route rule missing
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-04.md section 3.3 (the role_grant row, line 955; a new row for platform_release) and the route map (lines 565-577, rules at 567-568)
- Sources:
  - phase-25.md:732 "Extension of phase 04 (`role_grant` change target; DR-234): one operation `revoke` is added to the ..."
  - phase-25.md:1260 "(b) a new target kind `platform_release` (route `cr_access`, key `release:<release_version>`, `volume_effect = neutral`, evidence kind ..."
  - phase-25.md:1260 "the route map of phase 04 section 3 gains `platform_release` beside `role_grant`"
  - gap-items.md item (u)
- Check: grep -n "revoke\|platform_release" docs/plan/phase-04.md -> no revoke operation and no platform_release row. The route map (lines 567-568) has no platform_release rule. phase-25.md has no migration or seed row that writes the rule.

Proposed 7.N text ("### 7.7 role_grant revoke and the platform_release target"):

    (a) Section 3.3, role_grant gains the operation `revoke`: maker SEC, route cr_access_adm, checker ADM. Apply:
        UPDATE admin.role_grants SET revoked_at = now(), revoked_by = <checker>
        WHERE id = $grant AND revoked_at IS NULL; then invalidate_user; the apply writes role_grant.revoked (phase 03
        kind). A revoke of a grant that is not active: 422 grant_not_active, no request created (phase 25 section 3.1).
    (b) New target kind platform_release: route cr_access; object_key release:<release_version>; volume_effect neutral;
        evidence kind package with owner kind release; its EvidenceResolver is registered with the target.
    (c) Route map: add {"object_kind": ["platform_release"], "route": "cr_access"} after the role_grant rules.
        UNCLEAR: which migration writes this row. Phase 25 says the map "gains" the rule but names no migration. Without
        the rule a release falls to the class rules (cr_material), which phase 25 does not intend.

### G1-P04-08 Remind and escalate need a notify outbox row; notify effect has no params
- Severity: MEDIUM
- Category: Missing Event
- Target: phase-04.md section 3.6, remind (line 1112) and escalate (line 1115); the terminal effect `notify` (deferrable; line 1021 and the trait comment at line 690)
- Sources:
  - phase-22.md:819 "(a) ... `NotifyEffect` ... params `{ template_code, to }` ... (b) The phase 04 `remind` ..."
  - phase-22.md:971 "Phase 04 and DSL E.3 name the effect and its deferral but give no params."
  - phase-04.md:1113 "The notice itself arrives with phase 22 (`notify` is deferrable); until then the inbox shows ..."
  - phase-00.md:7.1 (run 1): topic `notify.request` is reserved for phase 22
- Check: grep -n "notify" docs/plan/phase-04.md -> lines 690, 1021, 1113 only. remind and escalate write no outbox row; no params shape for notify.

Proposed 7.N text ("### 7.8 Notice requests from reminders and escalations"):

    Section 3.6: in the transaction of remind, write the outbox row notify.request {task_id, template_code:
    "task.overdue", to: "holders"}; in the transaction of escalate, write notify.request {task_id, template_code:
    "task.waiting", to: "escalation_target"} (codes and recipients as phase 22 section 4.5 item 5(b)).
    Terminal effect notify: registered by phase 22 as NotifyEffect (all resource kinds) with params
    {"template_code": "<closed code of phase 22 section 1.2>", "to": "initiator" | "subject_team"} (the shape phase 22
    defines; its Open doubts 12). It stays deferrable until phase 22 registers it.

### G1-P04-09 Volume effect cannot be re-measured at decision time
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-04.md section 3.1 "Volume effect" (line 834) and the ChangeTarget trait (lines 642-653)
- Sources:
  - phase-16.md:2061 "phase 04 needs a recompute entry point for the volume effect of a request that ..."
  - phase-16.md:1077 "G11 volume ... never `unknown` for a material route"
  - phase-16.md:2060 "Phase 13 must call `ReaderCatalogue::measure` again in its `check_before_decision`" (the other half of the same open doubt)
  - phase-04.md:651 "check_before_decision ... -> Result<Vec<ValidationIssue>, ChangeError>" (returns issues only)
- Check: grep -n -i "recompute\|re-measure\|refresh" docs/plan/phase-04.md -> none.

Proposed 7.N text: UNCLEAR. Two designs; the sources do not choose.
    (a) ChangeTarget::check_before_decision may also return a refreshed volume_effect and base_class. When the refreshed
        class rises, the engine re-runs ChangeRequestService::classify (section 3.1 steps 1 to 5) and refuses the act
        with a sub-code (name UNCLEAR) so that approvers see the new class.
    (b) A request whose volume effect is unknown stays unapprovable as material; the maker resubmits with a new replay.
    The lead chooses. Phase 16 Open doubts 12 is the owner of the decision.

### G1-P04-10 Two settings cap an assessor grant (90 days in phase 04, 30 days in phase 24)
- Severity: MEDIUM
- Category: Incomplete Coverage
- Target: phase-04.md section 1 settings seed (line 590) and section 3.3 role_grant row (line 955)
- Sources:
  - phase-04.md:590 "('access.assessor_max_days', '90', 'integer', NULL, TRUE" (governed)
  - phase-04.md:955 "`AS` needs `valid_to` within `access.assessor_max_days`" (role_grant row)
  - phase-24.md:461 "('assessor.grant_max_days', '30', 'integer', ... 'M-62 time-boxed read-only assessor grant'" (governed, max 30)
  - phase-24.md:756 "`valid_to` no more than `assessor.grant_max_days` after `valid_from`"
  - phase-25.md:1206 and 1403 use `assessor.grant_max_days` (30 days)
- Check: grep -n "assessor" docs/plan/phase-04.md -> access.assessor_max_days only. grep -n "assessor.grant_max_days" docs/plan/*.md -> phases 24 and 25 only.

Proposed 7.N text: UNCLEAR. The sources disagree on the key and the limit (phase 04: 1 to 365 days, default 90; phase 24 and 25: maximum 30). The lead chooses one key for the AS grant. No text is proposed until then. Phase 04 section 1 and section 3.3 must use the chosen key.

### G1-P04-11 ChangeKind families, route rules and shipped definitions named by later phases
- Severity: LOW
- Category: Incomplete Coverage
- Target: phase-04.md section 3.1 / section 2 ChangeKind entity (line 1403), route map (lines 565-577), shipped definitions table (line 1258)
- Sources:
  - phase-05.md:1265 "the `ChangeKind` family `source` (kinds `load_budget`, `source_access`)" and phase-05.md:1381 "Phase 04: the route-map rule (migration 0019), the `ChangeKind` family `source`"
  - phase-07.md:727 "the `ChangeKind` family `reporting` (kind `goaml_profile`)"
  - phase-09.md:1090-1092 `{"object_kind": ["authority_matrix"], "route": "cr_authority"}` and the new shipped file `cr_authority.json`
  - phase-15.md:1797 "The phase 04 shipped set grows from nine to twelve"
  - phase-18.md:1122, phase-20.md:968, phase-19.md:1694 (further shipped definitions and effects named by those phases)
- Check: grep -n "reporting\|cr_authority" docs/plan/phase-04.md -> none. The ChangeKind line (1403) names families access, setting, workflow, dev and says detection, mapping, kyc_model and reference "arrive later". The shipped table (line 1258) has nine rows.

Proposed 7.N text ("### 7.11 Families and routes added by later phases"):

    Section 3.1 family list: phase 04 families are access, setting, workflow and dev. Later families are added by the
    phase that owns them: detection, mapping, kyc_model, reference (as already written), source (phase 05), reporting
    (phase 07). Route rules named by later phases (phase 05 migration 0019, phase 09 cr_authority) are written by those
    phases; this section needs only this note. The nine rows of the shipped table are the release 1 set; definitions
    shipped by phases 09, 15, 18 to 21 and 23 are listed in those phases.

## Target phase 03

### G1-P03-01 Masked kinds: the registry is open, but phase 03 says "from phase 15"
- Severity: MEDIUM
- Category: Incomplete Coverage
- Target: phase-03.md section 3.6, "Lists" paragraph end (line 795) and "Inference harness" paragraph (lines 797-801); SP-27 wording in the header (line 25)
- Sources:
  - phase-03.md:795 "Phase 03's lists contain no masked kind."
  - phase-03.md:25 "the tiered resources arrive from phase 15"; phase-03.md:404 "phase 15 onwards registers masked kinds (customer, alert, case, report, evidence, RFI, freeze)"
  - phase-14.md:1189 "Masked kinds registered with the phase 03 masking registry: `screening_alert` and `pair_memory`"
  - phase-14.md:1329 "Masked kinds registered before phase 15 deviate from phase 03's text; confirm."
  - phase-15.md:1821 "`customer` becomes masked; `kyc_review`, `override` and `pep_entry` are registered masked kinds"
  - phase-16.md:1796 "`replay_sample` is a masked kind"; phase-17.md:1239 "Masked kinds `alert` ... and `case`"
  - phase-19.md:1715 "`MASKED_KINDS` gains the masked kind `rfi`"; phase-22.md:821 "Gains `BranchItem` ... and `LiaisonReport`"
  - phase-24.md:1165 "`MASKED_KINDS` gains `audit_event` ... and `audit_target`"; phase-04.md:1523 "`MASKED_KINDS` gains `task_restricted`"
- Check: grep -n "MASKED_KINDS" docs/plan/phase-03.md -> line 797 only, and it names only the development kind `_dev_masked`.

Proposed 7.N text ("### 7.1 Masked kinds register by phase"):

    The registry MASKED_KINDS is open. A phase registers each masked kind it introduces, with its complete probe set
    (section 3.6), in the phase that first stores the record. The kinds named so far:
      phase 03: _dev_masked
      phase 04: task_restricted
      phase 14: screening_alert, pair_memory
      phase 15: customer, kyc_review, override, pep_entry
      phase 16: replay_sample
      phase 17: alert, case
      phase 19: rfi
      phase 22: BranchItem, LiaisonReport
      phase 24: audit_event, audit_target
    Replace "Phase 03's lists contain no masked kind" with: "Phase 03's own lists contain no masked kind." Replace
    "the tiered resources arrive from phase 15" (header) with "the tiered resources arrive from the phases that own them
    (section 3.6)." Phase 14's own deviation note is closed by this rule.

### G1-P03-02 Reporting purpose codes for package downloads are not seeded
- Severity: MEDIUM
- Category: Missing Policy
- Target: phase-03.md section 1, migration 0011, purpose code seed (lines 127-146)
- Sources:
  - phase-21.md:885 "The purpose is a code of the phase 03 purpose list; the reporting purposes are ..."
  - phase-21.md:1387 "The phase 03 purpose list does not name the reporting purposes ... the lead adds ..."
  - gap-items.md item (q): "phase 03 purpose codes for report download."
- Check: grep -n "filing\|'review'" docs/plan/phase-03.md -> none in the seed. The seed (lines 127-146) holds assigned_work, investigation, kyc_review, report_preparation, authority_request, qa_review, internal_audit, supervisory_inspection, break_glass and other.

Proposed 7.N text: UNCLEAR. Names, labels, tiers and reason_required of the reporting codes are an open decision (phase 21 Open doubts 13; "filing" and "review" are given as examples only). When the bank decides, add the rows to the migration 0011 seed with the same column list as the existing rows. The tiers of each new row are also UNCLEAR.

### G1-P03-03 Ops cockpit tiles and ops_probe inputs from later phases are not in the table
- Severity: MEDIUM
- Category: Incomplete Coverage
- Target: phase-03.md section 3.10, tile table (lines 960-972: 13 rows) and the ops_probe bullet (line 979)
- Sources:
  - phase-04.md:1134 "The cockpit (phase 03 A3) gains the tile 'parked tasks' with their ages"
  - phase-05.md:1375 "Ops cockpit: a `Sources` tile on `GET /admin/ops`"
  - phase-07.md:840 "a goAML line (phase 03 `GET /admin/ops`)"
  - phase-09.md:1065 "three tiles on A3: the age of the newest rate per currency, the calendar horizon, ..."
  - phase-25.md:959 "Ops cockpit tiles (extends phase 03 `OpsCockpit::snapshot()`; TS-28)"; phase-25.md:963 siem_lag and the other tile names
  - phase-30.md:348 "Added to the table of phase 03 section 3.10"; phase-30.md:486 "`ops_probe` gains the metrics and the outage ..."
- Check: grep -n -i "parked\|Sources\|goAML\|rate\|siem" docs/plan/phase-03.md -> none in the tile table. The table has jobs, unknown job kinds, outbox, timers, backups, audit, evidence, disk, certificates, OpenBao, Valkey, replication lag and authorisation bundle.

Proposed 7.N text ("### 7.3 Tiles and probe inputs added by later phases"):

    The tile table of section 3.10 is extended by the tiles each phase registers through OpsCockpit::snapshot():
    parked tasks (phase 04 section 3.6 step 3, ages); Sources (phase 05 Sources tile, state per source, last contact, open
    pause and its age); goAML (phase 07 goAML line); newest rate age per currency, calendar horizon, overrides expiring
    within 30 days (phase 09 A3 tiles); the tiles of phase 25 section 3.10 (break-glass active, break-glass review
    overdue, access review overdue, release currency, support sessions open, SIEM lag, SIEM dead letters, anchor timestamp
    unverified); the tiles of phase 30 section 3.13 (db_role, db_sync, db_failovers, etcd, certificates; replication_lag
    covers the DR replica). Thresholds and states are as each phase gives them. A tile whose probe snapshot is stale shows
    unknown (existing rule).
    ops_probe (section 3.10 bullet) also computes the phase 30 section 3.13 inputs: akr_db_writable_primary,
    akr_db_sync_standbys, akr_db_failovers_total, and opens or closes the outage record, as phase 30 section 3.13 defines.

### G1-P03-04 Flag service has no guard for pack flags and no hook for later jobs
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-03.md section 3.10, "FlagsAdmin::update" (lines 953-954)
- Sources:
  - phase-26.md:497 "The phase 03 flag service refuses to set `pack.islamic` to FALSE while any `live` islamic ..."
  - phase-26.md:752 "Phase 03: the flag service refuses `pack.islamic` FALSE while a live islamic version exists"
  - phase-28.md:318 "Pack flag (the phase 26 pattern for `pack.islamic`; phase 03 flag service)"; phase-28.md:768 "the feature flag `pack.assistant` is seeded by `0090`"
  - phase-06.md:587 "the phase 03 flag service when `branch.observations_live` or `pack.islamic` changes" (enqueues `coverage_grade`, phase-06.md:871)
  - gap-items.md items (v) and (x)
- Check: grep -n "pack\.\|coverage_grade\|refuses" docs/plan/phase-03.md -> none. FlagsService::update (line 954) writes flag.updated only.

Proposed 7.N text ("### 7.4 Flag guards and post-change hooks"):

    FlagsService::update checks a per-key guard before it writes (guard trait name UNCLEAR; any name the lead picks):
      pack.islamic and pack.assistant: setting FALSE while a live version of the matching pack exists is refused
      (409 E_NOT_ALLOWED_IN_STATE; sub-code UNCLEAR, phase 26 gives none beyond the code).
    After commit, a change of branch.observations_live or pack.islamic enqueues the job coverage_grade with payload
    {reason: "flag"} (phase 06 payload). Audit flag.updated unchanged.

### G1-P03-05 Ctx.via value list lacks gateway_cert and webhook_hmac
- Severity: LOW
- Category: Missing Policy
- Target: phase-03.md section 3.1, schema excerpt, type Ctx, field via (line 550)
- Sources:
  - phase-29.md:1232 "Phase 03 `Ctx.via` lists `session`, `pat` and `client_credentials`. This phase adds the value `gateway_cert`"
  - phase-29.md:1343 "Phase 03: `Ctx.via` gains the value `gateway_cert`"
  - phase-31.md:766 "`Ctx.via = "webhook_hmac"` (the value added to the phase 03 list by section 4.5)"
  - phase-31.md:1088 item 24: the `Ctx.via` value `webhook_hmac` is an addition phase 03 must accept
- Check: grep -n "gateway_cert\|webhook_hmac" docs/plan/phase-03.md -> none. Line 550 lists three values.

Proposed 7.N text ("### 7.5 Ctx.via values"):

    The comment on the via field reads: // "session" | "pat" | "client_credentials" | "gateway_cert" | "webhook_hmac"
    "gateway_cert": a gateway listener with a pinned certificate (phase 29). "webhook_hmac": the directory webhook
    (phase 31). The type of via is unchanged; no policy depends on the comment.

### G1-P03-06 Delegation window is not capped by the authority matrix
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-03.md section 2, port DelegationAuthority (lines 459-460), and section 3.9 DelegationService steps 2 and 3 (lines 931-938)
- Sources:
  - phase-09.md:874-877 "`window_limit_days(roles)`: the smallest `max_days` of the roles. Phase 03's `DelegationService` caps the window at `min(delegation.max_days, ..."
  - phase-09.md:1082-1083 "`DelegationService` step 2 caps the window at `min(delegation.max_days, window_limit_days)`"
  - phase-09.md:1268-1269 "the phase 03 fixture gains a matrix seed; a phase 03 test that expected `NoMatrix` ..."
- Check: grep -n "window" docs/plan/phase-03.md -> none. Step 2 (line 933) caps only at delegation.max_days.

Proposed 7.N text ("### 7.6 Window limit from the authority matrix"):

    Port DelegationAuthority gains window_limit_days(roles: &[RoleCode]) -> Option<u32> (name from phase 09). The
    NoMatrix implementation returns None. DelegationService step 2 reads: "length at most delegation.max_days and, when
    the port returns Some(n), at most n days." Phase 03's delegation fixture installs a matrix seed (phase 09 verification list);
    the NoMatrix test asserts the None path.

### G1-P03-07 One branch per liaison or alternate is not enforced
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-03.md section 1, migration 0012, admin.branches DDL (lines 184-206); section 3.9 BranchService::update
- Sources:
  - phase-22.md:820 "(b) `BranchService::update` refuses a user who is already liaison or alternate of another branch (422 ..."
  - phase-22.md:975 "One branch per liaison, and the alternate's grant. DR-205 and section 4.5 item 6(b) refuse ..."
  - phase-22.md:930 "DR-205 (a branch is identified by `admin.branches`; one branch per liaison)"
- Check: grep -n "already_liaison\|UNIQUE" docs/plan/phase-03.md -> the branches indexes (lines 200-203) are non-unique on liaison_user_id and alternate_user_id; no service rule.

Proposed 7.N text ("### 7.7 One branch per liaison or alternate"):

    CREATE UNIQUE INDEX IF NOT EXISTS uq_branches_liaison_user   ON admin.branches (liaison_user_id)
        WHERE liaison_user_id IS NOT NULL;
    CREATE UNIQUE INDEX IF NOT EXISTS uq_branches_alternate_user ON admin.branches (alternate_user_id)
        WHERE alternate_user_id IS NOT NULL;
    (The archive CHECK already clears both columns, so no archived predicate is needed.)
    Across the two columns (a user who is liaison of one branch and alternate of another) the indexes do not apply:
    BranchService::update refuses it with 422 E_VALIDATION at /liaison_user_id or /alternate_user_id, sub-code
    already_liaison_elsewhere (phase 22 name).

### G1-P03-08 UserRoleQuery read port is missing
- Severity: MEDIUM
- Category: Missing Integration
- Target: phase-03.md section 2, Ports (line 458)
- Sources:
  - phase-22.md:585 "roles from phase 03's effective-role read port"
  - phase-22.md:820 "(c) A read port `UserRoleQuery::holders(role, scope) -> Vec<UserId>` over the effective roles that phase 03 ..."
- Check: grep -n "UserRoleQuery\|holders(" docs/plan/phase-03.md -> none. Phase 03 names the effective roles (line 563) but no port.

Proposed 7.N text ("### 7.8 UserRoleQuery port"):

    Ports gains: UserRoleQuery { holders(role: RoleCode, scope: <type UNCLEAR>) -> Vec<UserId> }, a read port over the
    effective roles (direct grants and active delegations, the same source as User.roles in the bundle). Read only; no
    audit row; implemented in the api and worker binaries. The scope type is UNCLEAR (phase 22 gives none).

### G1-P03-09 credential_reset.requested outbox row is not written
- Severity: MEDIUM
- Category: Missing Event
- Target: phase-03.md section 3.8, CredentialResetService::request (lines 854-860), and section 4 "Outbox topics" (line 1100)
- Sources:
  - phase-22.md:820 "(a) `CredentialResetService::request` writes the outbox row `credential_reset.requested {reset_id}` in its transaction"
  - phase-22.md:609 "`credential_reset.requested` (phase 03, section 4.5 item 6) ... `security.reset_pending`"
  - phase-00.md:840 "Each topic is registered by the phase that produces or consumes it"
- Check: grep -n "outbox" docs/plan/phase-03.md -> section 4 "Outbox topics: none." (line 1100). The request steps (854-860) write an audit event only.

Proposed 7.N text ("### 7.9 Outbox topic for credential resets"):

    CredentialResetService::request writes, in its transaction, the outbox row credential_reset.requested {reset_id}
    (topic produced here; phase 22 is the handler). Section 4, "Outbox topics", reads: credential_reset.requested
    (produced in section 3.8; handled by phase 22). No other outbox row is written by this phase.

### G1-P03-10 Directory extensions to user administration are missing
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-03.md section 3.7 (disable, lines 838-843; enable, line 844) and the audit table of section 3.12 (user.disabled row)
- Sources:
  - phase-31.md:442 "`UserAdminService::disable_from_directory` (extends phase 03 `UserAdminService::disable`, sections 3.12 and 4.5)"
  - phase-31.md:697 "One transaction, the phase 03 `disable` body: status `disabled`, reason `admin`"
  - phase-31.md:686 "Phase 03 `POST /admin/users/{id}/enable` on a linked account whose `directory_active` is `FALSE` is refused 409 ..."
  - phase-31.md:741 "`user.disabled` (phase 03; payload gains `source`)"
  - phase-31.md:1088 item 24
- Check: grep -n "disable_from_directory\|directory" docs/plan/phase-03.md -> none. The enable steps (line 844) have no directory check. The user.disabled detail is {reason: admin, reason_code} (section 3.12).

Proposed 7.N text ("### 7.10 Directory extensions to user administration"):

    (a) Section 3.7 gains UserAdminService::disable_from_directory beside disable: the same body as disable (status
        disabled, reason admin, tokens revoked, after commit invalidate_user, Kratos revoke_all_sessions and
        set_state(inactive), a Kratos failure enqueues offboard_finalize), with the target and last-holder refusals of
        phase 31 section 3.12. Grants, delegations and liaison assignments stay (a suspension, not an offboarding).
    (b) enable refuses a linked account whose directory row has directory_active = FALSE: 409 E_NOT_ALLOWED_IN_STATE,
        sub-code directory_inactive. The check reads the phase 31 link through a port (name UNCLEAR; phase 31 names none).
    (c) user.disabled gains the detail key source (phase 31 names the key; its value is UNCLEAR, the directory path is
        the expected value).

### G1-P03-11 staff_id is not in UpdateUser or the PATCH row
- Severity: MEDIUM
- Category: Missing Fields
- Target: phase-03.md section 3.7, UpdateUser (line 418), and the PATCH /admin/users/{id} row (line 499)
- Sources:
  - phase-15.md:501-502 "administration of phase 03 (`PATCH /admin/users/{id}`, field `staff_id`, admin.users scope, ADM; extended, section 4)" and "ALTER TABLE admin.users ADD COLUMN IF NOT EXISTS staff_id"
  - phase-15.md:1851 "the user form gains the field `staff_id` (ADM, `admin.users` scope; the change is a phase ..."
- Check: grep -n "staff_id" docs/plan/phase-03.md -> none. UpdateUser (line 418) lists email, display_name, locale, branch_id, vetting_status, vetting_expires_on, expires_at.

Proposed 7.N text (text note only; phase 15 owns the column, index and grant):

    UpdateUser gains staff_id? (ADM only). The column, its unique index and the UPDATE grant are created by phase 15
    (its migration); this phase only accepts the field. The PATCH /admin/users/{id} row keeps the admin.users scope and
    names the field as ADM-only.

### G1-P03-12 akr-ctl policy matrix is not in the command list
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-03.md section 3.1, akr-ctl policy list (lines 648-660)
- Sources:
  - phase-08.md:66-67 "akr-ctl policy matrix --bundle <dir> --json (phase 03 AccessMatrixBuilder over a bundle on disk, no ..."
  - phase-08.md:548 "Phase 03: `akr-ctl policy matrix --json`."
  - phase-25.md:892 "run `akr-ctl policy matrix --bundle <dir> --json` (phase 08 section on the release check)"
- Check: grep -n "policy matrix" docs/plan/phase-03.md -> none. The list at lines 648-660 holds validate, install, activate and shadow.

Proposed 7.N text ("### 7.12 akr-ctl policy matrix"):

    akr-ctl policy matrix --bundle <dir> [--json] (zone Z5): runs AccessMatrixBuilder over the bundle directory without a
    database and prints {bundle_hash, roles: {<role>: [<action>, ...]}} (the shape phase 08 section 3 names). It writes no
    audit row, because it has no database; the release check (phase 25) records the hash it reads.

### G1-P03-13 Development routes: build gate and release-build absence test
- Severity: MEDIUM
- Category: Incomplete Coverage
- Target: phase-03.md section 3.11, development kind bullet (line 999), and section 6 verification
- Sources:
  - phase-17.md:852 "`POST /api/v1/_dev/detections` | dev only | ... compiled out of release builds"
  - phase-17.md:1943-1944 "exists only in non-release builds (phase 03 `_dev` rule). The release build's test that the ..."
  - phase-03.md:999 "Development kind `_dev_masked` (mounted only when `AKR_ENV != production`, like `/_echo`)"
  - phase-04.md:666 "Phase 04 registers change_request (and _dev_masked in dev builds)"; phase-04.md:960 "`_dev_object` (dev builds)"
- Check: grep -n -i "release build\|feature\|compiled" docs/plan/phase-03.md -> none. The only gate in phase 03 is the runtime AKR_ENV test.

Proposed 7.N text ("### 7.13 Development routes"):

    Every development path (phase 03 /_dev/masked, phase 17 /_dev/detections, and the phase 04 _dev_object and
    _dev_masked subjects) is compiled only into non-release builds (a build feature; name UNCLEAR) and, in any build,
    answers 404 when AKR_ENV is production. Section 6 gets a release-build check: the inventory lists every _dev path,
    and the release build returns 404 for each. Later phases add their _dev paths to that inventory.

## Summary

- Findings by severity: HIGH 2 (G1-P04-01, G1-P04-02); MEDIUM 20 (G1-P04-03 to G1-P04-10; G1-P03-01 to G1-P03-04, G1-P03-06 to G1-P03-13); LOW 2 (G1-P04-11, G1-P03-05). Total 24.
- Target phase 04: 11 findings. Target phase 03: 13 findings.
- Obligations already present and not written up: about 45 (checked one by one and found in place; not listed one by one). Examples: the cr_mapping and cr_material_validated routes and the mapping/code_map route rule; ChangeTarget methods generic_create, check_before_decision, activate, revert; WorkflowSubject fields attribute_prefix, party_members, on_ended, visibility_sql; TerminalEffect and EffectRegistry; the seven reason-code families; grants on ref.reason_codes; inventory kinds rule, parameter_set, model, llm_component, prompt_template; inventory UPDATE grants for last_validated_at and next_validation_due; the emergency route; ConflictCheck and DelegationAuthority ports; the purpose codes internal_audit, supervisory_inspection, investigation, report_preparation, break_glass, other; the _dev_masked route; x-akr-action and authz_route; the offboarding step registry.
- UNCLEAR (lead decision needed): G1-P04-05 (method name; whether the maker is in the list), G1-P04-07 (which migration writes the platform_release route rule), G1-P04-09 (design of the volume re-measure), G1-P04-10 (which assessor key), G1-P03-02 (reporting purpose code names and tiers), G1-P03-08 (scope type), G1-P03-10 (port name and the source value), G1-P03-13 (build feature name).
- Source-side notes for the lead (no finding): phase-27.md:782 says "Phase 03: ChangeTargetRegistry gains the two targets"; the registry belongs to phase 04 (the targets are phase 27's, registered in phase 04's registry). phase-04.md:666 and 960 gate dev routes by build, phase-03.md:999 by AKR_ENV (see G1-P03-13).
