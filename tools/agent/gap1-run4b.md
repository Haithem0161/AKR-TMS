# Gap pass 1, run 4b (targets 25 to 31 and the system documents; Haiku draft for lead review)

## Target phase 25 (Platform growth and releases)

### G1-25-01 Tables excluded from the support bundle and the SIEM
- Severity: MEDIUM
- Category: Missing Policy
- Target: phase-25.md section 3.8 (support bundle, step 1) and section 3.5 (SIEM, stripping rule)
- Sources: phase-27.md:785 "the support bundle and the SIEM export exclude `alert.model_scores` by table name"; phase-28.md:775 "exclude `casework.assistant_drafts` and `admin.assistant_evaluations` by table name"; phase-29.md:1350 "exclude `assess.decisions`, `assess.holds`, `assess.hold_candidates` and `assess.release_exceptions` by table name"; phase-27.md:913 "Neither is designed in phase 25 yet."
- Check: grep -n -i -E "alert\.model_scores|assistant_draft|assistant_eval|assess\." docs/plan/phase-25.md -> none. Section 3.8 step 1 lists "table row counts" with no table exclusion (no row content of any table is in the archive; the allow-list of step 2 holds).
- Proposed 7.x text: "### 7.3 Tables excluded from the support bundle and the SIEM (phases 27, 28, 29)". The support bundle CLI (section 3.8, step 1) writes row counts for no table in this list and no row content of any table. The SIEM reads the audit log only (section 3.5), so these tables reach it only through an audit payload, which carries ids and counts only (the audit events of phases 27 to 29). Excluded by table name: `alert.model_scores` (phase 27), `casework.assistant_drafts` and `admin.assistant_evaluations` (phase 28), `assess.decisions`, `assess.holds`, `assess.hold_candidates` and `assess.release_exceptions` (phase 29). Test (section 6): the bundle fixture's row-count list contains none of the seven names.

### G1-25-02 Release import of model and assistant artifacts (registered only)
- Severity: HIGH
- Category: Missing Integration
- Target: phase-25.md section 3.6 (import step 2, verify step 5)
- Sources: phase-27.md:557 "A phase 25 release that carries a model imports it through section 3.3 with status `registered` and nothing else."; phase-27.md:881 "a phase 25 release imports model artifacts as `registered` only"; phase-27.md:768 "written by ... the release import of phase 25 (starter models)"; phase-28.md:775 "the release import may carry a signed assistant artifact, which is `registered` only".
- Check: grep -n -i -E "artifact|model|assistant" docs/plan/phase-25.md -> no match. Section 3.6 step 5 inserts the help articles only (section 3.9); no model or assistant import exists.
- Proposed 7.x text: "### 7.4 Release import of model and assistant artifacts". On a verified bundle (section 3.6 step 5), the bundle's model artifacts go to `ModelRegistry::import_version` (phase 27 section 3.3) and its assistant artifacts to `AssistantVersionService` (phase 28, `ImportVersion`). Each import sets status `registered` and nothing else: the release never sets `live`, `shadow` or `challenger`, and the first install of a starter family is promoted by its own change request (phase 27 section 3.7, Open doubts 8). The imports run as their own jobs after the release reaches `verified`; a refused import of one artifact does not change the release status (its own refusal code, phase 27 or 28). Decision needed from the lead: the last sentence is a choice this draft makes, not a source statement.

### G1-25-03 Policy matrix file hash recorded by the release check
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-25.md section 3.6, verify job step 3
- Sources: phase-03.md:1359 "the release check of phase 25 records the hash it reads"; phase-08.md:275 "regenerated against the bank's active bundle by `akr-ctl policy matrix` during the phase 25 release check".
- Check: grep -n "policy-matrix" docs/plan/phase-25.md -> none. Step 3 writes `policy_matrix_diff` and `policy_bundle_hash` only.
- Proposed 7.x text: "### 7.5 Policy matrix file hash in the release check". In verify job step 3, the check also computes the SHA-256 of the file `frontend/src/app/policy-matrix.json` as produced by `akr-ctl policy matrix --json` (phase 03 section 7.12) for the bundle, and stores it as the key `matrix_file_sha256` inside the JSON object `policy_matrix_diff`. No new column. A mismatch with the bundle's own matrix is not a refusal in this phase (the diff of section 3.6 step 3 is the approval input).

### G1-25-04 Rotation runbook for keys/auth-hmac
- Severity: MEDIUM
- Category: Incomplete Coverage
- Target: phase-25.md section 4 (new runbook entry, cited by phase 01 as part of this phase)
- Sources: phase-01.md:920-923 "Rotating `keys/auth-hmac` re-mints CSRF cookies at the next request and invalidates every personal token ... the rotation runbook is part of phase 25."
- Check: grep -n -i "runbook|auth-hmac|rotat" docs/plan/phase-25.md -> line 1398 only (the upstream-disable runbook). No keys/auth-hmac runbook.
- Proposed 7.x text: "### 7.6 Runbook: rotation of keys/auth-hmac". Scope: the OpenBao KV entry `keys/auth-hmac` (relative path under mount `akr/`, DR-356) holds the master from which the HKDF-SHA-256 subkeys `csrf`, `token` and `login_identifier` are derived (phase 01). Steps for the bank operator: (1) write a new KV version of `akr/keys/auth-hmac` with a fresh random value under step-up (`akr-ctl` operator policy); (2) record the new KV version in the operator's change log; (3) tell users with personal tokens that every token is invalidated and must be created again (tokens expire within 90 days in any case, phase 01); (4) check: a request with a CSRF cookie minted before the rotation receives a new one on the next request; a request with an old personal token is refused. Effects stated by phase 01 only: CSRF cookies re-minted at the next request, every personal token invalid. Do not claim session revocation (phase 01 does not state it).

### G1-25-05 Phase 30 extensions of the release record (note only, owner phase 30)
- Severity: MEDIUM
- Category: Missing Fields
- Target: phase-25.md section 1.2 (`admin.releases.refusal_code`), section 3.6 (verify job), section 3.13 (`release.refused`), section 5.1 (A4)
- Sources: phase-30.md:22 "Widens the CHECK of `admin.releases.refusal_code` (phase 25 section 1.2) by `chart_signature_invalid`"; phase-30.md:324 "adds the chart count and the K3s version to `cosign_result`"; phase-30.md:452 "the chart check of section 3.12"; phase-30.md:411 "`release.refused` (phase 25) with the new `chart_signature_invalid`"; phase-30.md:507 "shows two additional facts from `cosign_result`".
- Check: grep -n -i "chart|k3s|cosign_result" docs/plan/phase-25.md -> the `cosign_result` column at line 180 and its comment "counts and digests" only; no chart or K3s wording.
- Proposed 7.x text: "### 7.7 Chart and K3s extensions owned by phase 30 (text note)". The CHECK list of `admin.releases.refusal_code` (section 1.2) is widened by phase 30 migration `0095` with `chart_signature_invalid`; phase 25 creates no second change. The verify job (section 3.6, step 1) gains check (g) `chart_signature_invalid`, the Helm chart check of phase 30 section 3.12, run after check (f); the refusal order of step 4 ends at (g). `cosign_result` gains the chart count and the K3s release version (counts and versions, no signature bytes; phase 30 section 3.12). A4 (section 5.1) shows both facts (phase 30 section 5.2). `release.refused` carries the new code, `refusal_code` only.

### G1-25-06 Operator's rollout record attached to the apply
- Severity: MEDIUM
- Category: Missing Fields
- Target: phase-25.md section 3.6, "Applied" (`POST /admin/releases/{id}/applied`), and section 1.2 (`admin.releases`)
- Sources: phase-30.md:314 "The operator's rollout record: the upgrade script writes `rollout.json` ... for the bank to attach to `POST /admin/releases/{id}/applied` (phase 25 section 3.6)"; phase-25.md:1408 (Open doubts 14) "Phase 30's cluster tooling may give a machine record of the rollout."
- Check: grep -n -i "rollout" docs/plan/phase-25.md -> none. The applied route has no attachment field (section 3.6 "Applied").
- Proposed 7.x text: "### 7.8 Rollout record on the apply". `POST /admin/releases/{id}/applied` takes an optional `rollout_evidence_id`: the bank operator uploads `rollout.json` first through the phase 02 EvidenceStore as kind `package` with owner kind `release` (the owner kind of section 3.6). Step 3 of the apply checks that the evidence exists and has owner kind `release` and this release's id (else `E_VALIDATION`). The id is stored in a new column, added by a text note to migration `0081` (this phase owns it): `ALTER TABLE admin.releases ADD COLUMN IF NOT EXISTS applied_evidence_id UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT;`, with `GRANT UPDATE (applied_evidence_id)` to `akr_app`, and a classification row (internal). The platform still cannot prove the apply (Open doubts 14); the record is the file's evidence, not a proof.

### G1-25-07 No cluster objects in the support bundle (explicit exclusion)
- Severity: LOW
- Category: Incomplete Coverage
- Target: phase-25.md section 3.8, step 1 (archive contents) and step 2 (allow-list)
- Sources: phase-30.md:377 "the bundle does not collect Kubernetes objects, events or pod logs from the cluster. Logs of the application that a support session needs are the redacted bundle of phase 25 only."
- Check: grep -n -i "kubernetes|cluster|pod" docs/plan/phase-25.md -> none in section 3.8. The step 1 list already omits them, so the rule is implied, not stated.
- Proposed 7.x text: "### 7.9 Support bundle scope for the cluster (phase 30)". Add to section 3.8 step 2: the archive holds no Kubernetes object, event, pod log or cluster state (phase 30 section 3.8); the application logs are the redacted logs of step 2 only.

### G1-25-08 Failover and restore reports reach SEC through phase 25 notices (UNCLEAR)
- Severity: MEDIUM
- Category: Missing Integration
- Target: phase-25.md section 3.3 (notices of the break-glass path)
- Sources: phase-30.md:417 "Viewable by `admin.system` and by the SEC role through the phase 25 notices only."; phase-25.md:810-811 (the notice table holds only `security.break_glass_waiting` and `security.break_glass_opened`).
- Check: grep -n -i "failover|restore|ha\." docs/plan/phase-25.md -> none. Notice codes in phase 22 (grep -n -i "failover" docs/plan/phase-22.md -> none).
- UNCLEAR: phase 30 says SEC reads failover and restore reports through phase 25 notices, but phase 25 has notices for break-glass only and no failover notice. Two options: (a) phase 30 names another reader (for example the A3 tiles of phase 03 and the SEC read of the control report of phase 24) and phase 25 adds nothing; (b) phase 25 adds a notice code for failover reports, which phase 22 must then send. The lead chooses; this draft recommends (a) because no source names a failover notice.

### G1-25-09 Read-only conformance check cited by phase 26
- Severity: LOW
- Category: Incomplete Coverage
- Target: phase-25.md section 6, item 4 (database tests)
- Sources: phase-26.md:863 "Read-only conformance (phase 25 check set): `akr_readonly` cannot read `kyc.possession_evidence` (permission denied); its grants are unchanged."
- Check: grep -n "akr_readonly" docs/plan/phase-25.md -> line 1314 (`EXECUTE` of `audit.siem_page` only) and line 512 "No grant to akr_readonly, akr_ingest or akr_audit_writer." No table-level read check exists.
- Proposed 7.x text: "### 7.10 Read-only grant check (phases 26 to 31 cite it)". Add to section 6 item 4: `has_table_privilege('akr_readonly', t, 'SELECT')` is false for each of the eleven tables created by `0080` to `0082` (section 1), and the test is a loop over the list of section 1.1 to 1.3, so each later phase adds its tables to the same list. Each later phase's own read-only check (for example phase 26 item 11) reuses this loop.

### G1-25-10 Service principal expiry: scope of the one-year rule
- Severity: LOW
- Category: Incomplete Coverage
- Target: phase-25.md section 3.5 (pull API, precondition) and section 4.5 (phase 01 sign-in bullet)
- Sources: phase-29.md:1520 "phase 25 rules that a service principal's expiry is within a year. The bank confirms whether a certificate of a longer life is accepted"
- Check: grep -n "within one year|within a year" docs/plan/phase-25.md -> section 3.5 pull API and section 4.5 (SIEM pull principal only).
- Proposed 7.x text: "### 7.11 Scope of the one-year expiry". The one-year expiry rule of section 3.5 and section 4.5 applies to the SIEM pull principal only. Other service principals (for example the gateway certificates of phase 29) are not governed by this phase; their expiry rule is phase 29's decision (Open doubts 24) and is not stated here.

### G1-25-11 Canary records have no owner (UNCLEAR)
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-25.md header "Requirements carried" (and a new section 3 subsection if the lead assigns it here), or phase-18.md section 3 (the other owner named by phase 24)
- Sources: phase-24.md:1357 "Research file 30 sections 8.6 and 8.8 name canary records and their triggers. No phase builds them. ... Phase 25 or an amendment of phase 18 owns them."; phase-24.md:616 "`canary_not_built: true` (no canary record exists in any phase; Open doubts 12)"; research/30-security-privacy.md:1403 "SP-30 ... 8.6".
- Check: grep -n -i "canary" docs/plan/phase-25.md -> none. grep -n "SP-30" docs/MODULES.md -> none (SP-30 is in no module row).
- UNCLEAR: phase 24 names two owners (phase 25 or phase 18). Canary records need the MLRO through maker-checker, excluded from every statistic, and a P1 alert on any open, hit or export. Recommend phase 25 (the access and insider-control phase, with the access-event trail of phase 02), but the lead decides, and the requirement SP-30 must first be added to a MODULES row.

- Findings: 11 (HIGH 1, MEDIUM 7, LOW 3; UNCLEAR 2 among them: G1-25-08, G1-25-11).

## Target phase 26 (Islamic pack)

### G1-26-01 Register warning call: signature and WarningTitle variant
- Severity: MEDIUM
- Category: Missing Integration
- Target: phase-26.md section 3.5 (possession evidence sweep, step 4) and the ports list at the top of section 2 (`RegisterWriter::warn`)
- Sources: phase-24.md:1293-1294 "`warn(tx, source, source_key, title_key: WarningTitle, body) -> Option<Uuid>`"; phase-24.md:1296 "later phases add their variants (phase 26, phase 29)"; research.md DR-388 (phase 24 section 7.7, the accepted writer); phase-26.md:274 "`RegisterWriter::warn(source, source_key, title_key, detail)`".
- Check: grep -n "WarningTitle" docs/plan/phase-26.md -> none. grep -n "RegisterWriter::warn" docs/plan/phase-26.md -> line 274 only, without the `tx` argument and with `detail` for `body`.
- Proposed 7.x text: "### 7.N Register warning call (DR-388)". (1) The call of section 3.5 step 4 is `RegisterWriter::warn(tx, 'scan', 'islamic_evidence_gap:' || bank_date, WarningTitle::IslamicEvidenceGap, body)`, inside the transaction of the sweep, with `body = {"missing": n, "late": n}`. (2) Add the variant `IslamicEvidenceGap` to the closed enum `WarningTitle` of phase 24 (section 7.7), with the bilingual title of step 4 ("fixed bilingual title" text written in section 3.5 step 4, as `title {ar, en}` in the message catalogue). (3) The ports list of section 2 reads `RegisterWriter::warn(tx, source, source_key, title_key, body)` (phase 24 DR-388). The insert is `ON CONFLICT (source_key) DO NOTHING` (phase 24), which keeps the "one row per bank day" rule of step 4.

### G1-26-02 Party read predicate: two-argument call in the deal read
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-26.md section 3.4 (deal read, step 7)
- Sources: phase-18.md:1308 "`PartyReadPredicate::allows(principal, party_id, context) -> bool`"; phase-26.md:273 "`PartyReadPredicate::allows(principal, party_id, context)`" (ports list); phase-26.md:368 "`PartyReadPredicate::allows(ctx, party_id)`".
- Check: grep -n "PartyReadPredicate::allows" docs/plan/phase-26.md -> lines 273 (three arguments) and 368 (two arguments). The phase 18 signature (section 7+) is the three-argument form.
- Proposed 7.x text: "### 7.N Party predicate form in the deal read". Step 7 of section 3.4 calls `PartyReadPredicate::allows(principal, party_id, context)` with the same three arguments as the ports list of section 2 and phase 18 section 7+ (the principal is the caller, the context is the deal read's context). Only the call changes; the omitted-party rule of step 7 stays.


## Target phase 27 (Models and model risk)

### G1-27-01 Validation pass sets the family's next validation date
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-27.md section 3.6 (validation pack and the `POST /models/{id}/validation` route, row at section 2) and section 3.1
- Sources: phase-04.md:1692 "phase 27 computes due dates from the tier and lists overdue items"; phase-04.md:1748 "no inventory screens or validation calendar (phase 27)"; phase-27.md:783 "`next_validation_due` of the family is set at creation and moved by the review calendar's existing route"; phase-24.md:702 "`InventoryService::update` (phase 04) sets `last_validated_at = now()` and `next_validation_due = today + review.rules_months`".
- Check: grep -n "next_validation_due" docs/plan/phase-27.md -> lines 381, 433 and 783 only. Route `POST /models/{id}/validation` (line 409) has no due-date step. Phase 24 `review_calendar` is read-only (phase-24.md:654), so it cannot move a date.
- Contradiction: phase 27 line 783 says the review calendar moves the date; phase 24 says the calendar is read only and only a passing validation sets it.
- Proposed 7.x text: "### 7.N Validation date of a model family". When the validation pack of a model version gets verdict `pass` (phase 24 pack, section 3.6 model branch), the same transaction calls phase 04 `InventoryService::update` on the family's inventory item (`kind = model`, `object_id = models.id`) to set `last_validated_at = now()` and `next_validation_due = today + review.rules_months` (the phase 24 setting, governed, 12). The phase 24 `review_calendar` then lists the family as a `rules` row (amber within 30 days, red past due); no model-specific calendar is added. Replace the sentence of section 2 "moved by the review calendar's existing route" with "moved by the passing validation pack of this section".

### G1-27-02 Inference probes for the model-score route (kind alert)
- Severity: MEDIUM
- Category: Incomplete Coverage
- Target: phase-27.md section 6 (verification, inference harness item) and section 3.12 (Cedar and the masking of the new route)
- Sources: phase-17.md:1815-1816 "`GET /alerts/{id}/model-score` (phase 27 route 16) joins the `alerts` tag and the probe set of kind `alert` in check 9. Phase 27 owns these changes"; phase-03.md:797 "probes are added by every phase that adds a masked kind" (SP-33 registry `MASKED_KINDS`).
- Check: grep -n -E "MASKED|inference|probe" docs/plan/phase-27.md -> none. Section 6 item k and l are curl checks only (line 839-840).
- Proposed 7.x text: "### 7.N Masking probes for the model-score route". Add to section 6 the probe set of kind `alert` for route 16 in the `MASKED_KINDS` registry (phase 03 section 3): (a) suspicion-tier alert, caller without step-up: 401 `E_STEPUP_REQUIRED`; (b) with step-up and purpose, caller outside the team: 404 with the same bytes as an unknown id (phase 17 rule 5); (c) the body of the 404 contains no score, band or reason code (searched for the known values, as item k does for the shadow value). Each probe is a row of the registry with its route id `GET /alerts/{id}/model-score`.

### G1-27-03 Retention class of model inventory items (UNCLEAR)
- Severity: LOW
- Category: Incomplete Coverage
- Target: phase-27.md section 1.1 (retention rows, the INSERT after line 354)
- Sources: phase-04.md:558 "Retirement = retired; model items move to R11 in phase 27"; phase-27.md:355-357 (R11 rows for `admin.models`, `admin.model_versions`, `admin.model_evaluations`).
- Check: grep -n "inventory_items" docs/plan/phase-27.md -> lines 733 and 783 only; no retention row for `admin.inventory_items`. Phase 04 line 558 gives the table the class R9 `retirement`.
- UNCLEAR: `admin.retention_assignment` is keyed by table (phase 04 and phase 23), so one table cannot be R9 for other rows and R11 for model rows. Options: (a) phase 27 says that model inventory items keep the table's R9 class, and their model evidence (families, versions, evaluations) is R11 through the rows of section 1.1; phase 04's note is then void. (b) phase 27 moves the model items to a separate table, which this draft does not propose. This draft recommends (a); the lead decides.


## Target phase 28 (Assistant)

### G1-28-01 MRO reads of the assistant monitoring row (UNCLEAR)
- Severity: LOW
- Category: Incomplete Coverage
- Target: phase-28.md section 2 (route `GET /assistant/monitoring`) and section 5.3 (D4 Assistant tab)
- Sources: phase-28.md:432 "`GET /assistant/monitoring` | model.risk, audit.read, dashboard.read | Counts per task over 30 days"; phase-24.md:682 "`fairness.read` is M, CH, IA and MRO" (the only report-rate reader named by phase 24 in this pass); gap item (x) "whether MRO sees the `report_narrative` monitoring row (phase 24 shows report rates only to `dashboard.read`)".
- Check: grep -n -i "report_rate|report rate|narrative" docs/plan/phase-24.md -> no tile with report rates. Phase 28 section 2 gives MRO (`model.risk`) the monitoring route.
- UNCLEAR: phase 28 lets MRO read the per-task counts (including `report_narrative`), and phase 24 names no report-rate tile for MRO. The two do not contradict each other in text; the gap item's premise (phase 24 limits report rates to dashboard.read) could not be confirmed in phase 24. Lead decides whether MRO keeps the route (this draft sees no tipping-off reason to remove it: counts only, suppressed below the threshold, DR-286). No text is proposed.


## Target phase 29 (Payment assessment and gateway)

### G1-29-01 Valkey outage: readiness and the gateway rate limit
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-29.md section 4.4 (readiness bullet) and section 3.1 step 3 (rate limit); a text note only
- Sources: phase-30.md:201-203 "Valkey is removed from the `akr-assess` readiness list (amends phase 00 ...)"; phase-30.md:202 "Gateway rate limit of `akr-assess` (phase 29 step 3) ... phase 29 has no rule for it. `LimiterFallback`: an in-process token bucket per node"; phase-30.md:481 "Readiness. Valkey is removed from the readiness list of `akr-assess` (phase 00 section 2 ... and phase 29 section 4.4)"; phase-30.md:632 "beyond the readiness list, the limiter fallback, the version check and the reconnect reload of section 4.5 items 2 and 14".
- Check: grep -n "Valkey" docs/plan/phase-29.md -> line 912 (step 3, the Valkey token bucket), 1071 (section 3.10), 1337 (section 4.4 readiness: "ready when the database answers, Valkey answers"). grep -c "LimiterFallback" -> 0.
- Proposed 7.x text: "### 7.N Valkey outage (text note; owner phase 30)". Readiness of `akr-assess` (section 4.4) no longer includes Valkey; phase 30 section 3.6 owns the change and the reason. Step 3 of section 3.1 and the rate limit of section 3.10 use the in-process `LimiterFallback` of phase 30 section 3.6 when Valkey does not answer, with the capacity and refill that phase 30 defines; the metric `akr_assess_limiter_fallback_total` (phase 30) counts each fallback answer. No other rule of step 3 changes.

### G1-29-02 Listener reconnect reload for the two assess caches
- Severity: LOW
- Category: Missing Logic
- Target: phase-29.md section 3.1 or 3.6 (`PolicyCache`) and section 3.10 (`GatewayCache`); a text note only
- Sources: phase-30.md:480 "Reconnect reload added to `akr_assess_policy`, `akr_assess_gateway` (phase 29) and `akr_source` (phase 05)"; phase-30.md:181 "notifications sent during the reconnect gap are lost"; phase-30.md:64 "`PolicyCache` and `GatewayCache` (phase 29) gain `reload_on_reconnect` and the 30-second version check".
- Check: grep -n -i "reconnect" docs/plan/phase-29.md -> none. The 30-second refresh already exists (phase-29.md:1024 "every 30 seconds by reading `assess.policy_state.set_version`"; phase-29.md:1069 "refreshed every `assess.gateway_cache_s` (30 seconds)"), so a lost notification is bounded to 30 seconds.
- Proposed 7.x text: "### 7.N Reconnect reload (text note; owner phase 30)". On a reconnect of the `LISTEN` connection for `akr_assess_policy` or `akr_assess_gateway`, `PolicyCache` and `GatewayCache` reload at once (phase 30 section 3.x, `reload_on_reconnect`). The 30-second refresh of both caches (this section) stays as the bound when a notification is lost.

### G1-29-03 Register warnings name no WarningTitle variant
- Severity: MEDIUM
- Category: Missing Integration
- Target: phase-29.md section 3.12 (watch, step 3; release exceptions, step 5) and section 1 (no enum change)
- Sources: phase-24.md:1294-1297 "`warn(tx, source, source_key, title_key: WarningTitle, body)` ... `WarningTitle` is a closed enum ... later phases add their variants (phase 26, phase 29)"; phase-29.md:1097 "warnings in the phase 24 register, source `assess_control`"; phase-29.md:1132 "One register warning per run and rail (`source_key = 'assess:release:' || rail || ':' || run_date`, body with counts only)".
- Check: grep -n -E "WarningTitle|RegisterWriter" docs/plan/phase-29.md -> none. Phase 29 names no title key for any of its warnings.
- Proposed 7.x text: "### 7.N Register warnings of the watch and the release exceptions (DR-388)". Add to `WarningTitle` (phase 24 section 7.7) four variants, one per warning this phase writes: `AssessKeyAge` (decision key older than 365 days), `AssessCertificateExpiry` (gateway certificate `not_after` within 30 days), `AssessFallbackStale` (fallback extract older than `assess.fallback_extract_hours`), `AssessReleaseException` (one per run and rail). Each has a bilingual title in the message catalogue. The writes call `RegisterWriter::warn(tx, 'assess_control', source_key, WarningTitle::<variant>, body)` from the job `assess_rollup_watch` only (the job rule of phase 24). Names are proposed here: no source names them.

### G1-29-04 Gateway circuit breaker threshold (UNCLEAR)
- Severity: LOW
- Category: Incomplete Coverage
- Target: phase-29.md section 4.4 (readiness, "the gateway's circuit open") or section 3.1
- Sources: phase-30.md:602 "`ops.failover_max_s` (60 seconds) and the gateway's circuit breaker threshold must agree"; phase-29.md:1337 "Not ready makes the gateway's circuit open, which is the bank's fail policy."
- Check: grep -n -i "breaker|threshold" docs/plan/phase-29.md -> no breaker threshold setting or number (only "circuit" at line 1337 and 1114, a bank log).
- UNCLEAR: phase 30 needs a breaker threshold that phase 29 does not name. The breaker is on the bank's gateway side (outside AKR-TMS), so phase 29 may only state that the threshold is the bank's setting and must not be shorter than `ops.failover_max_s`. The lead decides whether phase 29 or phase 30 owns that statement.


## Target phase 30 (Operations, HA and deployment)

### G1-30-01 Feature-flag gate on catalogue entries (the Polis entry)
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-30.md section 3.1 (inputs and steps of `akr-ctl deploy render`); section 3.19 (settings are unchanged)
- Sources: phase-31.md:908 "**Phase 30.** The catalogue has one new entry (`polis`), and the render takes the flag `pack.polis` as an input (section 4.4)."; phase-31.md:1007 "With `pack.polis` off, `akr-ctl deploy render --profile s` is byte-identical to the phase 00 bundle ... With it on, the render adds the `polis` service"; phase-31.md:889 "Compose and catalogue (phase 30 section 3.1): one entry `polis`".
- Check: grep -n -E "pack|flag" docs/plan/phase-30.md -> only the `pack.assess` mentions (lines 161, 601), no flag input to `akr-ctl deploy render`. The inputs at lines 71-88 are `deploy/zones.yaml` and `deploy/catalogue.yaml` only. The byte-for-byte acceptance test of step 2 exists (it covers the flag-off case).
- Proposed 7.x text: "### 7.N Pack flags in the catalogue (text note for phase 31)". (1) A catalogue entry may carry `pack: <flag name>`. (2) `akr-ctl deploy render` takes `--pack <name>=on|off` for each flag named in the catalogue; a flag that is not given is off. (3) An entry whose flag is off is omitted from every output (compose, chart, env keys and secret paths), so with all flags off the output is the phase 00 bundle byte for byte (step 2). (4) The entry for `polis` is phase 31's (its section 4.4 and DR-348); phase 30 owns only the mechanism. The flag itself is `pack.polis` in `admin.feature_flags` (phase 31 section 1.5), and the render reads it from the command line, not from the database.


## Target phase 31 (Training, capacity, exposure, correspondents and directory)

### G1-31-01 Real implementation of the directory link port (enable check)
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-31.md section 3.12 (disable from a directory) and section 4.5 item 4 (the phase 03 extension)
- Sources: phase-03.md:1345-1347 "`enable` refuses a linked account whose directory row has `directory_active = FALSE` ... It reads the link through the port `DirectoryLinkQuery { is_inactive(user_id) -> bool }`, whose phase 03 implementation `NoDirectory` returns false; phase 31 supplies the real one."
- Check: grep -n "DirectoryLinkQuery|is_inactive|NoDirectory" docs/plan/phase-31.md -> none. Phase 31 has the table `admin.directory_users` (section 1.4, columns `linked_user_id`, `directory_active`), so the port can read it.
- Proposed 7.x text: "### 7.N Directory link port (phase 03 section 7.10)". Add to section 3.12: `DirectoryLinkQuery::is_inactive(user_id) -> bool` is implemented by `PgDirectoryLinkQuery`, which returns TRUE when a row of `admin.directory_users` has `linked_user_id = user_id` and `directory_active = FALSE`, and FALSE otherwise (no link, or an active row). The implementation is wired into `UserAdminService` in `AppState` when the flag `pack.polis` is on; with the flag off the phase 03 `NoDirectory` (always FALSE) stays. Test: a linked user whose directory row is inactive gets `409 E_NOT_ALLOWED_IN_STATE` with sub-code `directory_inactive` on `enable`.


## PART B: system documents

### G1-DOC-01 DESIGN-SYSTEM 8.1 has no row for the Sharia board screen
- Severity: MEDIUM
- Category: Missing Integration
- Target: docs/DESIGN-SYSTEM.md section 8.1 (Screen catalogue), a new row (for example after the `G2` row, line 724)
- Sources: phase-26.md:782 "### 5.2 I1 Sharia board aggregates (`/islamic/sharia-board`; DASH; new screen, catalogue id to be assigned by the design owner, proposed `I1`, new in phase 26)"; docs/MODULES.md:738 "view's screen id is proposed `I1` and is not yet in the DESIGN-SYSTEM catalogue".
- Check: grep -n "I1" docs/DESIGN-SYSTEM.md -> none. Route `GET /islamic/sharia-board/aggregates` exists in API.md (dashboard.read).
- Proposed text (the row, in the 8.1 table format `| ID | Name | Route | Layout | Phase |`): `| I1 | Sharia board aggregates | \`/islamic/sharia-board\` | DASH | P2 |`. Name it in the same words as phase 26 section 5.2. The catalogue ID is the design owner's choice; `I1` is the proposed value and no existing ID uses it.

### G1-DOC-02 DESIGN-SYSTEM 8.3 has no S2 Holds tab and no D3 Assessment tab
- Severity: MEDIUM
- Category: Missing Integration
- Target: docs/DESIGN-SYSTEM.md section 8.3 (Specifications of the key screens): new paragraphs for S2 and D3, placed next to the G1 paragraph
- Sources: phase-29.md:1364 (section 5.1, "S2 Screening queue ... addition: tab \"Holds\""); phase-29.md:1371 (section 5.2, "D3 Policy studio ... addition: tab \"Assessment\"" with the panels "Fail policy", "Rail modes", "Prohibited rules"); phase-29.md:1516 "`docs/DESIGN-SYSTEM.md` rows S2, D3 and G1 get their additions. This phase edits none of them."
- Check: grep -n -E "^\*\*S2|^\*\*D3|Holds|Assessment" docs/DESIGN-SYSTEM.md -> the S2 and D3 rows of 8.1 (lines 702 and 712) only; no 8.3 paragraph for S2 or D3, and no "Holds" or "Assessment" text.
- Proposed text (new 8.3 paragraphs, same bold-lead style as G1): "**S2 Screening queue, tab Holds (phase 29 section 5.1).** A tab beside the alert queue lists payment holds (`GET /api/v1/assess/holds`) with the rail, the mode label (`held at the gateway`, `advisory: not held at the gateway`, `shadow: no gateway effect`), the reason codes and the candidates. A suspicion-tier subject the caller may not see is not listed. Claim, propose release or reject, and approve by a second person follow the alert rules of S2." and "**D3 Policy studio, tab Assessment (phase 29 section 5.2).** Three panels: Fail policy (the matrix as a table and the bands per currency), Rail modes (one row per rail with its mode, effective-from date and readiness criteria), Prohibited rules (read-only list). Each panel states that a change is material and needs MLRO and CHK, and CH for a decrease." The lead applies.

### G1-DOC-03 DESIGN-SYSTEM 8.3 G1 paragraph lacks the Assessment tile
- Severity: LOW
- Category: Incomplete Coverage
- Target: docs/DESIGN-SYSTEM.md section 8.3, the `G1 Command centre` paragraph (line 780, order of tiles)
- Sources: phase-29.md:1378 "### 5.3 G1 command centre (`/command`; phase 24 screen G1; addition: tile \"Assessment\")" and its text "The tile shows the modes per rail, the list freshness, the open blind periods and the open release exceptions (counts). Visible to M, CH and IA".
- Check: grep -n "Assessment" docs/DESIGN-SYSTEM.md -> none; the G1 paragraph (line 780) lists no tile named Assessment.
- Proposed text: add to the G1 paragraph of 8.3: "the Assessment tile (phase 29 section 5.3): modes per rail, list freshness, open blind periods and open release exceptions, counts only; visible to M, CH and IA; it opens the Assessment tab of D3 for M and CH."

## Summary

Findings by severity (run 4b, targets 25 to 31 and PART B): HIGH 1; MEDIUM 17; LOW 8; total 26.
- By target: phase 25: 11 (HIGH 1, MEDIUM 7, LOW 3). Phase 26: 2 (MEDIUM 2). Phase 27: 3 (MEDIUM 2, LOW 1). Phase 28: 1 (LOW 1, UNCLEAR). Phase 29: 4 (MEDIUM 2, LOW 2). Phase 30: 1 (MEDIUM 1). Phase 31: 1 (MEDIUM 1). PART B: 3 (MEDIUM 2, LOW 1).

Obligations already present (skipped): about 140. The count is approximate: each source line that names a target was checked with grep against the target, and the matches were not logged one by one. Groups skipped include the phase 25 section 4.5 extensions, the break-glass clauses of phases 17 to 19, the phase 22 notice codes, the phase 24 control-report counts, the phase 26 Islamic pack rules (widened CHECK, canonical entities, G9 and pack seed, freeze groups, pack.islamic guard), the phase 27 queue, route 16, retention and validation-pack branch, the phase 29 assess tables and zones, the phase 30 catalogue, job and readiness rules, and the phase 31 train schema, directory tables, builders and surge rule. The 'secret/' spellings in phase 28 are corrected by its own section 7+ and are not counted as findings. Gap item (w) (differencing of rolling-window points) is already stated in phase 27 section 5 and is not a finding. Gap item (v) (API.md section 6 codes) is already applied.

UNCLEAR (the lead decides):
- G1-25-08 failover reports via phase 25 notices (phase 30 names a notice path phase 25 does not have).
- G1-25-11 canary records (phase 24 names phase 25 or phase 18 as owner; SP-30 is in no MODULES row).
- G1-27-03 retention class of model inventory items (a retention class is per table; phase 04 asks for a per-row move).
- G1-28-01 MRO reads of the assistant monitoring row (phase 28 route admits model.risk; the premise in gap item (x) about phase 24 is not confirmed).
- G1-29-04 gateway circuit breaker threshold (phase 30 assumes one; phase 29 names none).

Notes for the lead: the already-decided items (phase 26 party predicate to phase 17 or 18 is decided; phase 27 Open item 9; phase 30 akr-sign-cert; phase 31 QA sampler citation; akr_retention DELETE grants; phase 29 payment release status; phase 30 Open doubts 23; phase 09 typology line) were not re-checked and produce no finding. G1-26-02 concerns the arity of the phase 26 call only, not the attribution decided earlier. Phase 27 is internally inconsistent on the release import: its line 785 (Phase 25 requirement) cites section 3.7, while its line 557 cites section 3.3 (the import). G1-25-02 uses section 3.3.
