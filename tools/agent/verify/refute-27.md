# Refuter, phase 27 (Haiku draft for lead review)

### V-27-01-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: No method reads `feat.feature_values` for the sweep; step 3 is prose and section 2 names no feature reader.
- Evidence: docs/plan/phase-27.md:497 "for each alert read the stored values of the family's `feature_keys` from `feat.feature_values`". Section 2 names no feature reader (docs/plan/phase-27.md:390-396). The owner phase has a writer only: docs/plan/phase-13.md:466 "pub trait FeatureValueWriter: Send + Sync {". Greps for `FeatureValue` and "feature value" in phases 13 and 16 to 19 and in research.md find no reader method. docs/plan/phase-16.md:1012 "feature values, read as of the event's business date" is an evaluator context, not a method.
- Fix: phase 27, section 2 repository traits: add a feature-value reader with its parameters, which section 3.4 step 3 then calls.

### V-27-01-b  REFUTED   (first agent: LOW; refuter: none)
- Claim: The sweep's entry point is not named; ModelScorer has no method or signature and the sweep is only a job body.
- Evidence: the job kind is named with its owner and schedule: docs/plan/phase-27.md:752 "`model_score_sweep` | worker | `{}` [`model_score_sweep`]". The entry point is the phase 00 handler contract: docs/plan/phase-00.md:540 "Dispatch to the registered handler (`trait JobHandler { const KIND: &'static str; async fn run(&self, ctx:". The sweep steps call named methods, for example docs/plan/phase-27.md:499 "`ExplanationService::explain(session, row)`". The one unnamed call is the feature read (V-27-01-a).

### V-27-01-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Step 3 stores the missing-feature count in `shap`, but `shap` holds the five contributions of step 5, so no field holds the per-row count.
- Evidence: docs/plan/phase-27.md:497 "the count of missing features is stored in `shap` for live rows only". docs/plan/phase-27.md:499 "`shap` stores the five largest absolute contributions". The column accepts only an array of at most five: docs/plan/phase-27.md:187 "CHECK (jsonb_typeof(shap) = 'array' AND jsonb_array_length(shap) <= 5)". The batch count `skipped_missing` (line 500) is per batch. A grep of "missing" finds no per-row field.
- Fix: phase 27, section 3.4 step 3 and the `shap` comment of section 1.2: keep the count out of `shap`, or give it its own column.

### V-27-01-d  STANDS   (first agent: LOW; refuter: LOW)
- Claim: Section 3.9 says the sweep reads the kill state before each chunk of 50 alerts, but the numbered sweep steps read it once and have no chunk step.
- Evidence: docs/plan/phase-27.md:584 "the sweep reads the state before each chunk of 50 alerts". Section 3.4 step 1 reads it once: docs/plan/phase-27.md:495 "Read the kill state (section 3.9): if `global` is engaged, stop." The 50-alert chunk is a design constant: docs/plan/phase-27.md:729 "the 50-alert chunk of the kill-switch check". A grep of "chunk" finds lines 584 and 729 only.
- Fix: phase 27, section 3.4 step 1 or 2: state the kill-state read before each 50-alert chunk.

### V-27-01-e  STANDS   (first agent: LOW; refuter: LOW)
- Claim: Header line 12 cites section 3.4 for the forced low-score sample reused as the unbiased set; the text is in section 3.6.
- Evidence: docs/plan/phase-27.md:12 "forced low-score sample reused as the unbiased set, section 3.4". The text is at docs/plan/phase-27.md:528 "Low-scored alerts enter this set through the phase 19 daily forced sample". Section 3.4 (docs/plan/phase-27.md:490-509) has no such text.
- Fix: phase 27, header line 12: cite section 3.6 instead of section 3.4.
### V-27-02-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: Step 2 inserts `admin.models` before its inventory item, but `inventory_item_id` is NOT NULL and cannot be set on the family row later, so the order cannot be built.
- Evidence: docs/plan/phase-27.md:45 "UUID NOT NULL UNIQUE REFERENCES admin.inventory_items(id) ON DELETE RESTRICT" (not deferrable). Step 2 creates the item after the row: docs/plan/phase-27.md:437 "then call phase 04 `InventoryService::create` with `kind = model`". The family UPDATE grant leaves the column out: docs/plan/phase-27.md:281 "GRANT UPDATE (name, purpose, recalibration_due, live_version_id, pinned_version_id, next_kill_drill_due, updated_at, version)", and the guard refuses a change: docs/plan/phase-27.md:251 "NEW.inventory_item_id IS DISTINCT FROM OLD.inventory_item_id". Phase 04 allows the other order: docs/plan/phase-04.md:1344 "item may be registered before its object exists (GV-01: registered before use)". The version item has the same circularity (docs/plan/phase-27.md:63 and 473).
- Fix: phase 27, section 3.1 step 2 and the version item paragraph of section 3.1: create the inventory item first, with the object id generated in advance, then insert the family or version row with that item id.

### V-27-02-b  PARTLY   (first agent: MEDIUM; refuter: LOW)
- Claim: Step 1 refusals (bilingual name and purpose, starter needs a recalibration date, owner must be active) have no HTTP status or problem code except `tier_basis_incomplete`.
- Evidence: REFUTED part (status): the general field-validation rule gives 422 `E_VALIDATION`: docs/API.md:985 "Field or document validation; `errors[]` (DSL.md s.7 codes)". The owner check is phase 04's create rule: docs/plan/phase-04.md:1340 "Owner and validator are active human users". STANDS part (sub-codes): section 3.11 names no `errors[]` codes for the name, purpose, starter or owner checks. Its E_VALIDATION row lists only "`manifest_invalid`, `signature_invalid`, `tier_basis_incomplete`, `kind_not_recordable`, `criteria_invalid`, `closure_code_not_allowed`" (docs/plan/phase-27.md:599).
- Fix (STANDS part): phase 27, section 3.11 E_VALIDATION row: name the `errors[]` codes for the name, purpose, starter and owner checks of section 3.1 step 1.

### V-27-02-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Version items copy tier, owner, developer and committee but not `tier_basis`, so they keep the phase 04 default and nothing checks them.
- Evidence: docs/plan/phase-27.md:13 "the seven tiering answers stored in `inventory_items.tier_basis` and required for kind `model`". docs/plan/phase-27.md:441 "with the family's tier, owner, developer and committee". The column default has no kind check: docs/plan/phase-04.md:519 "JSONB NOT NULL DEFAULT '{}'::jsonb". Open doubts 5 copies the tier only (docs/plan/phase-27.md:940).
- Fix: phase 27, section 3.1 version item paragraph: copy `tier_basis` to the version item, or state why version items do not carry it.

### V-27-02-d  STANDS   (first agent: LOW; refuter: LOW)
- Claim: `InventoryService::create` needs a bilingual `name`, but `CreateModel.inventory` and step 2 do not say which name the item takes.
- Evidence: docs/plan/phase-04.md:517 "JSONB NOT NULL CHECK (name ? 'ar' AND name ? 'en')". docs/plan/phase-04.md:1339 "**Create**: `kind`, `object_id`, `name`, `tier`". docs/plan/phase-27.md:385 "`inventory` creates the family item with `kind = model` and `object_id = models.id`" (the field list has no name). Only the family name is a candidate, and the text does not say so.
- Fix: phase 27, section 3.1 step 2: state that the family item takes the family's bilingual name.

### V-27-03-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The monitor reads the score window, the window's decisions and alert flags, and the feature values, but no repository method names any of them.
- Evidence: the only score reads are docs/plan/phase-27.md:395 "`unscored_batch(conn, version_id, live_from, limit)`" and "`for_alert_live(conn, alert_id, alert_created_at, live_id)`", and neither reads a window. The decision repository has no window read: docs/plan/phase-18.md:575 "`list(conn, case_id) -> Vec<Decision>`". The override definition is prose: docs/plan/phase-27.md:571 "overrides over decisions made in the window on cases with live-scored alerts". The feature read is V-27-01-a.
- Fix: phase 27, section 2 repository traits: add a score window read, a decision window read and the feature reader that section 3.8 calls.

### V-27-03-b  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The red-breach finding has no write method named, and its `title` (NOT NULL, bilingual) is not given; section 3.8 gives `body` only.
- Evidence: REFUTED part (method): the owner phase names the insert: docs/plan/phase-24.md:533 "`RegisterRepository`: `insert(tx, NewRegister) -> Register`". STANDS part (title): the table requires it: docs/plan/phase-24.md:47 "JSONB NOT NULL CHECK (jsonb_typeof(title) = 'object' AND title ? 'ar' AND title ? 'en')". Section 3.8 gives no title: docs/plan/phase-27.md:576 "and a `body` with the metric, value, band, threshold and the five permitted decisions". The closed title enum covers warnings only: docs/plan/phase-24.md:1300 "`WarningTitle` is a closed enum".
- Fix (STANDS part): phase 27, section 3.8 "Red breach": give the finding's bilingual title (a title key or a literal) in the same sentence that names the insert.

### V-27-03-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The daily monitor writes `model_monitor_points` rows and names no audit kind for them; only the red breach has one.
- Evidence: docs/plan/phase-27.md:565 "Output rows are `admin.model_monitor_points`, written once (`ON CONFLICT DO NOTHING`)." Section 3.13 is headed "one per mutation" (docs/plan/phase-27.md:687 "### 3.13 Audit events emitted (one per mutation; payloads hold ids, names and counts only)"), and its only monitor kind is the breach (docs/plan/phase-27.md:701). The plan's own pattern for computed rows is one audit per run: docs/plan/phase-27.md:500 "one audit `model.scored" for the sweep, and docs/plan/phase-24.md:680 "Audit `fairness.computed {period, rows, suppressed, review}`" for the monthly metric rows. The housekeeping exemption does not apply: greps of phases 00 and 02 for "housekeeping" and "not audited" find nothing that covers computed rows.
- Fix: phase 27, section 3.8: one audit per daily run with counts only, and the matching row in the section 3.13 table with its SIEM flag.

### V-27-03-d  REFUTED   (first agent: LOW; refuter: none)
- Claim: A rerun "inserts nothing" because of the unique key, but the text names neither `ON CONFLICT` nor the lookup.
- Evidence: the insert form for job-opened findings is named by the owner: docs/plan/phase-24.md:665 "INSERT ... ON CONFLICT (source_key) DO NOTHING" (the post-review finding job). The owner makes `source_key` the dedupe key of a system row: docs/plan/phase-24.md:46 "-- dedupe of a system row". Section 3.8 states the outcome: docs/plan/phase-27.md:576 "(unique, so a rerun inserts nothing)".

### V-27-03-e  STANDS   (first agent: LOW; refuter: LOW)
- Claim: `scored_count` is defined as live-scored alerts, but section 3.8 runs the monitor for shadow and challenger versions too, where the metric is undefined.
- Evidence: docs/plan/phase-27.md:572 "live-scored alerts over alerts that entered the queue in the window". docs/plan/phase-27.md:565 "for every family's versions in status `shadow`, `challenger` and `live`". The only other mention is docs/plan/phase-27.md:575 "write `band = not_run` for `scored_count`", which covers switch-off days, not roles.
- Fix: phase 27, section 3.8 table, `scored_count` row: define the count for shadow and challenger rows, or limit the metric to the live role.

### V-27-03-f  STANDS   (first agent: LOW; refuter: LOW)
- Claim: Header line 13 cites section 3.7 for the monitors; they are in section 3.8.
- Evidence: docs/plan/phase-27.md:13 "override monitors with green, amber and red bands, section 3.7". Section 3.8 is "### 3.8 Monitoring: drift, overrides and breaches" (docs/plan/phase-27.md:563). Section 3.7 is promotion (docs/plan/phase-27.md:549).
- Fix: phase 27, header line 13: cite section 3.8.

### V-27-03-g  STANDS   (first agent: LOW; refuter: LOW)
- Claim: Header line 5 credits phase 18 with `casework.case_alerts`; phase 17 creates the table and phase 18 only uses it.
- Evidence: docs/plan/phase-27.md:5 "Phase 18 (`casework.decisions` with `type` and `decided_at`, `casework.case_alerts`)". docs/plan/phase-17.md:352 "CREATE TABLE IF NOT EXISTS casework.case_alerts (". Phase 18 only uses it: docs/plan/phase-18.md:440 "Existing grants used by this phase (phase 17)".
- Fix: phase 27, header line 5: name phase 17 as the owner of `casework.case_alerts`.
### V-27-04-a  STANDS   (first agent: LOW; refuter: LOW)
- Claim: Section 3.9 cites routes 10, 11 and 13 and section 3.10 cites route 9, but in the route table these rows are 12, 13, 14 and 11.
- Evidence: docs/plan/phase-27.md:584 "Engage (route 10; Cedar `model.kill.engage`". docs/plan/phase-27.md:586 "Release request (route 11". docs/plan/phase-27.md:587 "Drill (route 13". docs/plan/phase-27.md:591 "Pin (route 9". The rows are docs/plan/phase-27.md:416 "Pin or unpin the live version" (row 11), 417 "`/models/kill-switch`" (row 12), 418 "`/models/kill-switch/release-requests`" (row 13) and 419 "`/models/{id}/kill-drills`" (row 14). The same offset recurs in section 3.3 (route 4 for row 6) and section 3.6 (route 7 for row 9).
- Fix: phase 27, sections 3.3, 3.6, 3.7, 3.9 and 3.10: align the route numbers with the table order.

### V-27-04-b  STANDS   (first agent: LOW; refuter: LOW)
- Claim: Section 2 says no route admits a role its permits do not name, but SEC is admitted by `audit.read` on six GET routes and no Cedar permit names SEC.
- Evidence: docs/plan/phase-27.md:424 "No route admits a role that its permits do not name." docs/API.md:100 "IA, SEC; M for their own scope" (SEC holds `audit.read`). The GET routes with `audit.read` are docs/plan/phase-27.md:406 "model.risk, audit.read, dashboard.read" and lines 408, 409, 410, 412 and 420. A grep for the word SEC in docs/plan/phase-27.md finds nothing, so no permit in section 3.12 names SEC. The effect is a Cedar denial (403), not a hole.
- Fix: phase 27, the scope-check sentence at line 424: say that SEC is admitted by scope and denied by Cedar on these routes, or change the scope list of these routes.

### V-27-04-c  PARTLY   (first agent: LOW; refuter: LOW)
- Claim: ModelKillSwitchService has no method names or signatures for engage, release and drill; ModelSwitch needs `current_version` and `diff` from the phase 04 trait, which section 3.9 does not describe.
- Evidence: REFUTED part (engage, release, drill): the calls the steps make are named. docs/plan/phase-27.md:394 "`lock(tx, scope)`, `set_engaged(tx, scope, engaged, reason, by, expected_version)`" (engage and release), docs/plan/phase-27.md:393 "`EvaluationRepository`: `insert(tx, NewEvaluation)`" (drill record), and docs/plan/phase-27.md:584 "`StepUpService::consume(tx, ctx, header, ModelKillSwitch," (engage). STANDS part (trait methods): the phase 04 trait requires them: docs/plan/phase-04.md:649 "async fn current_version(&self, conn: &mut PgConnection, cr: &ChangeRequest)" and docs/plan/phase-04.md:655 "async fn diff(&self, cr: &ChangeRequest)". Section 3.9 describes only check_before_decision and activate for ModelSwitch (docs/plan/phase-27.md:586).
- Fix (STANDS part): phase 27, section 3.9: state the ModelSwitch behaviour of the other ChangeTarget methods, current_version, diff and the rest of the trait.

### V-27-04-d  STANDS   (first agent: LOW; refuter: LOW)
- Claim: The release step requires the family's `model_monitor` findings to be closed, but does not say how they are found.
- Evidence: docs/plan/phase-27.md:586 "the family's `model_monitor` findings must be closed (else `breach_open`)". The key format is given at docs/plan/phase-27.md:576 "source_key = model_monitor:<version>:<metric>:<feature or empty>:<D>", so a prefix lookup per version is inferable. The owner's list read takes a filter that is not defined for this: docs/plan/phase-24.md:533 "`list(conn, RegisterFilter, Keyset) -> Page<Register>`".
- Fix: phase 27, section 3.9 step 3: name the lookup (the source and the key prefix for each version of the family).

### V-27-05-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Step 4 is one UPDATE setting `status`, `shadow_started_at`, `shadow_criteria`, `top_band_cut` and `retired_at`, but `set_status` takes none of these fields.
- Evidence: docs/plan/phase-27.md:392 "`set_status(tx, id, from, to, expected_version)` (the guard enforces the edges)". The step: docs/plan/phase-27.md:545 "One update: `status` = the role, `shadow_started_at = now()`". The other version setter covers different fields: docs/plan/phase-27.md:392 "`set_verified(tx, id, runtime, opset, card_evidence_id, reference_bins, feature_keys, monotone)`". No method sets the shadow fields; a grep of `set_status` finds line 392 only.
- Fix: phase 27, section 2 `VersionRepository`: add a method (or extend `set_status`) that takes the shadow fields and the expected version, and name it in section 3.6 step 4.

### V-27-05-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The provisional top-band cut is a percentile of the ten-bin reference histogram, but no bin edges, bin shape or interpolation rule is given anywhere.
- Evidence: docs/plan/phase-27.md:545 "a provisional `top_band_cut` = the `model.top_band_percent` percentile of `reference_bins.score`". docs/plan/phase-27.md:79 "histogram counts, no rows". docs/plan/phase-27.md:569 "(ten bins, from the import manifest); bins floored at 0.0001". The manifest names no edges (docs/plan/phase-27.md:472). Greps of "percentile", "bin edge" and "histogram" across docs/plan find no interpolation rule outside phase 27 lines 545, 556 and 937.
- Fix: phase 27, section 3.6 step 4 and section 1.1 (the JSON shape of `reference_bins`): state the bin edges and the percentile rule.

### V-27-05-c  STANDS   (first agent: LOW; refuter: LOW)
- Claim: The table row for shadow gives `Idempotency-Key` only, but step 1 of section 3.6 requires `If-Match` on the version, and section 6 does not test it.
- Evidence: docs/plan/phase-27.md:414 "Start a shadow or a challenger run with the success criteria frozen; `Idempotency-Key`". docs/plan/phase-27.md:541 "`If-Match` on the version". Section 6 check 8.e (docs/plan/phase-27.md:837) tests the inventory refusal only, and no check sends If-Match to the shadow route.
- Fix: phase 27, route row 9 (docs/plan/phase-27.md:414): add `If-Match` on the version; section 6 check 8: add an If-Match case for the shadow start.

### V-27-05-d  STANDS   (first agent: LOW; refuter: LOW)
- Claim: Sections 3.6 and 3.7 call the shadow route "route 7", but the table row is 9.
- Evidence: docs/plan/phase-27.md:541 "Start a shadow or challenger run (route 7". docs/plan/phase-27.md:558 "A retired champion re-enters through `retired -> shadow` (route 7)". The row is docs/plan/phase-27.md:414 "`/models/{id}/shadow`" (row 9).
- Fix: phase 27, sections 3.6 and 3.7 step 6: use row 9 of the table.

### V-27-06-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The pin step-up refers to a `ModelPin` constant that is never defined.
- Evidence: docs/plan/phase-27.md:591 "`ModelPin` constant". Section 2 declares only the kill-switch constant: docs/plan/phase-27.md:375 "the `ModelKillSwitch` step-up constant". Each phase must declare its own action constants: docs/plan/phase-01.md:711 "phase declares its actions as `StepUpAction { name, class }` constants". A grep of "ModelPin" finds line 591 only. The class `approval` appears in prose (docs/plan/phase-27.md:591 "step-up `approval`"), but no name string is given.
- Fix: phase 27, section 3.10 Pin bullet or section 2: declare the `ModelPin` step-up constant with its name string and class `approval`.

### V-27-06-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Pin accepts "a version of the family or null", but no refusal covers a version of another family, or a version in `importing` or `refused`.
- Evidence: docs/plan/phase-27.md:591 "sets `admin.models.pinned_version_id` to a version of the family or null". The pin foreign key checks only that the version exists: docs/plan/phase-27.md:110 "REFERENCES admin.model_versions(id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED". The pin refusals in section 3.11 are only the If-Match codes: docs/plan/phase-27.md:608 "`E_PRECONDITION_REQUIRED` (428), `E_VERSION_CONFLICT` (409)". Greps of phases 00 to 03 for a cross-parent rule ("belongs to", "another family", "not a version") find no general rule.
- Fix: phase 27, section 3.10 Pin bullet: add the check that the target belongs to the family and is in a pinnable state, with its code (422 `E_VALIDATION` or 409 `E_NOT_ALLOWED_IN_STATE`).

### V-27-06-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The route row says pin applies to the live version, but section 3.10 lets a candidate be pinned, and section 3.7 refuses promotion while a version is pinned.
- Evidence: docs/plan/phase-27.md:416 "Pin or unpin the live version". docs/plan/phase-27.md:591 "a version of the family or null" and "a pinned candidate is not promoted until unpinned".
- Fix: phase 27, route row 11 (docs/plan/phase-27.md:416) or section 3.10: make both state the same target set.

### V-27-07-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Step 1 answers the masked 404 for a suspicion-tier row without step-up, but phase 17 and section 6.8(l) give 401 before any tier answer.
- Evidence: docs/plan/phase-27.md:505 "a suspicion-tier row for a caller without `record_open` step-up, answers the masked 404 `E_NOT_FOUND`", and in the same step "the step-up check runs before the tier answer". docs/plan/phase-17.md:1239 "the step-up answer is 401". docs/plan/phase-17.md:1240 "`E_STEPUP_REQUIRED` before any tier answer". docs/plan/phase-27.md:844 "without step-up: 401 `E_STEPUP_REQUIRED`". Section 3.11 names only kill engage and pin for E_STEPUP_REQUIRED (docs/plan/phase-27.md:607 "kill engage, pin").
- Fix: phase 27, section 3.4 step 1: answer 401 first for a suspicion row without step-up, and the 404 only after step-up; add route 16 to the E_STEPUP_REQUIRED row of section 3.11.

### V-27-07-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Step 2 reads the champion at scoring time, but the repository method filters on the current live id, so after a promotion an alert scored by the old champion reads as null.
- Evidence: docs/plan/phase-27.md:506 "(any live version: the champion at scoring time". docs/plan/phase-27.md:395 "`for_alert_live(conn, alert_id, alert_created_at, live_id)`". docs/plan/phase-27.md:930 "an alert created before the promotion keeps the score of the champion that scored it". No text says the `live_id` parameter is unused.
- Fix: phase 27, section 2 `ScoreRepository`: drop `live_id` or state its rule, so that the read matches section 3.4 step 2.
### V-27-08-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Step 5's INSERT omits four NOT NULL columns: `model_id`, `artifact_key`, `manifest_sha256` and `created_by`.
- Evidence: docs/plan/phase-27.md:473 "Insert `admin.model_versions` with `status = importing`". The four columns are NOT NULL in the DDL: docs/plan/phase-27.md:61 "UUID NOT NULL REFERENCES admin.models(id) ON DELETE RESTRICT", docs/plan/phase-27.md:68 "TEXT NOT NULL CHECK (artifact_key", docs/plan/phase-27.md:71 "TEXT NOT NULL CHECK (manifest_sha256", docs/plan/phase-27.md:86 "UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT". The request carries `artifact_key` and `manifest_sha256` (docs/plan/phase-27.md:386 "`ImportVersion { manifest: Value, manifest_sha256, signature, signing_key_version, artifact_key, card: Value }`"), and the family is locked in step 1 (docs/plan/phase-27.md:469). The values are inferable.
- Fix: phase 27, section 3.3 step 5: add the four columns and their sources.

### V-27-08-b  STANDS   (first agent: MEDIUM; refuter: HIGH)
- Claim: After the third failed card wait the version stays `importing`, and no transition or refusal covers a card that is never `clean` or is `rejected`.
- Evidence: docs/plan/phase-27.md:477 "else retry (attempts 3, phase 00 back-off). An infected card: refusal `manifest_invalid`." Phase 00 marks the job dead after its last attempt (docs/plan/phase-00.md:545 "`status = 'dead', finished_at = now()`"), but no text moves the version row. A family with an importing version refuses further imports (docs/plan/phase-27.md:469 "answers 409 `E_NOT_ALLOWED_IN_STATE` (`import_pending`)"). The av status `rejected` exists (docs/plan/phase-02.md:350 "CREATE TYPE admin.av_status AS ENUM ('pending', 'clean', 'infected', 'rejected');"), and only `infected` is mapped. Severity HIGH: a step is missing and no text gives a way out of `importing`, so the family stays blocked.
- Fix: phase 27, section 3.3 verification step 1: name the refusal code, the transition out of `importing` and the audit event for a card that is not clean after the last attempt, and for a rejected card.

### V-27-08-c  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: The refusal audit of step 3 would be lost with the 422 rollback, and the section does not say the audit is written in its own transaction.
- Evidence: the general rule covers it. docs/plan/phase-02.md:579 "Writes one event in its own transaction: acts with no domain change" (`AuditSink::emit_standalone`). docs/plan/phase-02.md:684 "A refused sign-in or a CLI check uses it". A refused import changes no domain row: docs/plan/phase-27.md:471 "nothing is inserted; audit".

### V-27-08-d  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The version's inventory item is created in the import transaction (section 3.1 and step 5), but step 7 says it "is created in `registered`" at verification.
- Evidence: docs/plan/phase-27.md:441 "created in the import transaction of section 3.3". docs/plan/phase-27.md:483 "The inventory item is created in `registered` (phase 04 default)." Step 5 needs the item id at insert, so the import transaction must create the item: docs/plan/phase-27.md:63 "UUID NOT NULL UNIQUE REFERENCES admin.inventory_items(id) ON DELETE RESTRICT".
- Fix: phase 27, section 3.3 step 7: delete the creation sentence, or state that the item already exists and only its status stays `registered`.

### V-27-08-e  REFUTED   (first agent: LOW; refuter: none)
- Claim: `VersionRepository::insert_importing(tx, NewVersion)` names a type with no fields.
- Evidence: the fields are named in the insert text: docs/plan/phase-27.md:473 "`artifact_sha256`, `manifest`, `signature`, `signing_key_version`, `feature_keys`, `monotone`", and the import DTO carries the rest: docs/plan/phase-27.md:386 "`ImportVersion { manifest: Value, manifest_sha256, signature, signing_key_version, artifact_key, card: Value }`". A Rust type whose fields are stated in prose or in a table anywhere in the plan is not a gap. The four missing NOT NULL columns are V-27-08-a.

### V-27-08-f  STANDS   (first agent: LOW; refuter: LOW)
- Claim: Section 3.3 cites route 4 for import; the table row is 6.
- Evidence: docs/plan/phase-27.md:468 "Import, synchronous (route 4, akr-api". The row is docs/plan/phase-27.md:411 "POST | `/models/{id}/versions`" (row 6). See V-27-04-a.
- Fix: phase 27, section 3.3 header: use row 6 of the table.

### V-27-08-g  STANDS   (first agent: LOW; refuter: LOW)
- Claim: Step 1 of the import says "Lock order 1", but section 3.16 numbers the family lock 2.
- Evidence: docs/plan/phase-27.md:469 "Lock order 1:". docs/plan/phase-27.md:735 "2. `admin.models` (the family row), `FOR UPDATE`." Section 3.16 governs every path (docs/plan/phase-27.md:733 "Taken in this order, in every path that takes more than one:").
- Fix: phase 27, section 3.3 step 1: use the numbering of section 3.16.

### V-27-09-a  PARTLY   (first agent: LOW; refuter: LOW)
- Claim: Block 7.2 names the update call but gives no `if_match` value and no actor context.
- Evidence: REFUTED part (actor): phase 04 section 7.6 fixes the context: docs/plan/phase-04.md:1689 "validation transaction with the validator's context". STANDS part (`if_match`): the call takes one: docs/plan/phase-04.md:1338 "`update(ctx, id, UpdateInventoryItem, if_match, assertion?)`". Block 7.2 gives no version read for it: docs/plan/phase-27.md:867 "`InventoryService::update` on the inventory item".
- Fix (STANDS part): phase 27, block 7.2: say that the update reads the item's version in the same transaction and passes it as `if_match`.

### V-27-10-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Block 7.3 says an alert the caller may not see gets the masked 404, but phase 17 gives 401 first for a suspicion row without step-up.
- Evidence: docs/plan/phase-27.md:873 "an alert the caller may not see answers the masked 404, byte for byte". docs/plan/phase-17.md:1239 "the step-up answer is 401". docs/plan/phase-17.md:1240 "`E_STEPUP_REQUIRED` before any tier answer". The block has no exception for suspicion rows. Same conflict as V-27-07-a.
- Fix: phase 27, block 7.3: exclude a suspicion row without step-up, which answers 401 first as phase 17 says.

### V-27-10-b  STANDS   (first agent: LOW; refuter: LOW)
- Claim: Block 7.3 says section 6 of phase 27 adds route 16 to the MASKED_KINDS probe set; the probe set belongs to phase 17's section 6, check 9.
- Evidence: docs/plan/phase-27.md:872 "Section 6 adds route 16". Phase 27 section 6 check 6 is the visibility pin: docs/plan/phase-27.md:830 "6. Visibility pin:". Phase 17 owns the probe set: docs/plan/phase-17.md:1243 "with the complete probe set of section 6, check 9". Section 4.5 says so: docs/plan/phase-27.md:781 "the MASKED_KINDS probe set of kind `alert` gains route 16".
- Fix: phase 27, block 7.3 first sentence: name phase 17 section 6, check 9.

### V-27-12-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The family and version rows are classed R11 on clock `retirement`, but phase 23 defines R11 on clock `age` only and gives `retirement` to R9, where it is never due.
- Evidence: docs/plan/phase-27.md:359 "'R11', 'retirement', NULL" (families) and docs/plan/phase-27.md:360 "'R11', 'retirement', NULL" (versions). docs/plan/phase-23.md:601 "| R11 | `age` | kept | 10 years |". docs/plan/phase-23.md:599 "| R9 | `retirement`, `age` | kept |". docs/plan/phase-23.md:577 "`retirement`: never due in this phase". The phase 00 CHECK accepts the pair, since it only ties R10 to `never` (docs/plan/phase-00.md:312 "CHECK ((retention_class = 'R10') = (clock = 'never'))"). The rows are never due, which block 7.7 intends (docs/plan/phase-27.md:893 "the R11 `retirement` and R10 tables are never purged"), but the class and the clock disagree.
- Fix: phase 27, section 1.3 rows for the two tables and block 7.7: use class R9 on clock `retirement`, or give R11 an `age` clock and say how the version guard interacts with it.
## Summary

- REFUTED: 4 (V-27-01-b, V-27-03-d, V-27-08-c, V-27-08-e)
- STANDS: 32
- PARTLY: 4 (V-27-02-b, V-27-03-b, V-27-04-c, V-27-09-a)

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-27-01-a | STANDS | MEDIUM | No feature-value reader in section 2 or in the owner phase | Section 2 repository traits (add a reader) |
| V-27-01-c | STANDS | MEDIUM | `shap` holds the contributions; the per-row missing count has no field | Section 3.4 step 3 or 5; section 1.2 `shap` comment |
| V-27-01-d | STANDS | LOW | The read before each 50-alert chunk is not in the numbered steps | Section 3.4 steps 1 and 2 |
| V-27-01-e | STANDS | LOW | Header cites section 3.4; the text is in section 3.6 | Header line 12 |
| V-27-02-a | STANDS | HIGH | Family and version rows need their item id before the insert, which the order does not allow | Section 3.1 step 2 and the version item paragraph |
| V-27-02-b | PARTLY | LOW | The errors[] sub-codes for the name, purpose, starter and owner checks are not named | Section 3.11 E_VALIDATION row |
| V-27-02-c | STANDS | MEDIUM | Version items do not copy `tier_basis`, and nothing checks it | Section 3.1 version item paragraph |
| V-27-02-d | STANDS | LOW | The name the family item takes is not stated | Section 3.1 step 2 |
| V-27-03-a | STANDS | MEDIUM | No repository method for the score, decision or feature window reads | Section 2 repository traits |
| V-27-03-b | PARTLY | MEDIUM | The red-breach finding has no bilingual title, which the table requires | Section 3.8 "Red breach" |
| V-27-03-c | STANDS | MEDIUM | Monitor points are written with no audit kind for the daily run | Section 3.8; section 3.13 table |
| V-27-03-e | STANDS | LOW | `scored_count` is undefined for shadow and challenger rows | Section 3.8 table, `scored_count` row |
| V-27-03-f | STANDS | LOW | Header cites section 3.7; the monitors are in section 3.8 | Header line 13 |
| V-27-03-g | STANDS | LOW | Header credits phase 18 with `casework.case_alerts`; phase 17 owns it | Header line 5 |
| V-27-04-a | STANDS | LOW | Route numbers in sections 3.9 and 3.10 (and 3.3, 3.6) do not match the table | Sections 3.3, 3.6, 3.7, 3.9, 3.10 |
| V-27-04-b | STANDS | LOW | The section 2 scope sentence is false for SEC on the audit.read routes | Section 2, line 424 |
| V-27-04-c | PARTLY | LOW | ModelSwitch does not describe the ChangeTarget methods `current_version` and `diff` | Section 3.9 |
| V-27-04-d | STANDS | LOW | The lookup of the family's model_monitor findings at release is not named | Section 3.9 step 3 |
| V-27-05-a | STANDS | MEDIUM | The shadow-start UPDATE sets fields that no repository method takes | Section 2 `VersionRepository` |
| V-27-05-b | STANDS | MEDIUM | No bin edges or percentile rule for the provisional top-band cut | Section 3.6 step 4; section 1.1 |
| V-27-05-c | STANDS | LOW | The shadow route row lacks If-Match, which step 1 requires; section 6 has no test | Route row 9; section 6 check 8 |
| V-27-05-d | STANDS | LOW | The shadow route is cited as route 7 | Sections 3.6 and 3.7 step 6 |
| V-27-06-a | STANDS | MEDIUM | The `ModelPin` step-up constant is referenced but not declared | Section 3.10 Pin bullet, or section 2 |
| V-27-06-b | STANDS | MEDIUM | No refusal for a pin target of another family, or one that is importing or refused | Section 3.10 Pin bullet |
| V-27-06-c | STANDS | MEDIUM | The route row says pin applies to the live version; section 3.10 allows a candidate | Route row 11 or section 3.10 |
| V-27-07-a | STANDS | MEDIUM | Step 1 gives the masked 404 where phase 17 and section 6.8(l) give 401 | Section 3.4 step 1; section 3.11 |
| V-27-07-b | STANDS | MEDIUM | `for_alert_live` filters by the current live id, against step 2 and DR-273 | Section 2 `ScoreRepository` |
| V-27-08-a | STANDS | MEDIUM | The import INSERT omits four NOT NULL columns | Section 3.3 step 5 |
| V-27-08-b | STANDS | HIGH | No terminal transition for a card that is never clean or is rejected | Section 3.3 verification step 1 |
| V-27-08-d | STANDS | MEDIUM | The version item's creation time differs between step 5 and step 7 | Section 3.3 step 7 |
| V-27-08-f | STANDS | LOW | The import route is cited as route 4 | Section 3.3 header |
| V-27-08-g | STANDS | LOW | The lock order numbering differs from section 3.16 | Section 3.3 step 1 |
| V-27-09-a | PARTLY | LOW | The `if_match` of the inventory update is not stated | Block 7.2 |
| V-27-10-a | STANDS | MEDIUM | Block 7.3's masked 404 conflicts with phase 17's 401-first rule | Block 7.3 |
| V-27-10-b | STANDS | LOW | Block 7.3 names section 6 for the probe set; phase 17 owns it | Block 7.3 first sentence |
| V-27-12-a | STANDS | MEDIUM | R11 on clock `retirement` contradicts the phase 23 class table | Section 1.3 rows; block 7.7 |
