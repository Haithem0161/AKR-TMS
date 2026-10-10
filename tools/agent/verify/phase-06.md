# Verification pass, phase 06 (Haiku draft for lead review)

Scope: docs/plan/phase-06.md (1,057 lines). Read: .claude/rules/planning.md (Phase File Template, Verification Pass),
tools/agent/b3-conventions.md, and the cross-referenced phase files named in each item (greps quoted per item).
Adversarial pass check: `grep -i adversarial docs/plan/phase-06.md` returns no hits, so the 3 adversarial items are
replaced by 3 more header items (rule of STEP 1).

## Items sampled

1. V-06-01 GET /cbi-catalogue, the CBI item read (header "Implemented here", VA-19)
2. V-06-02 Register install by `akr-ctl packs seed` (header "Requirements carried", S-01, start of list)
3. V-06-03 CBI gap report run and its build job (S-02)
4. V-06-04 Readiness grading and the `coverage_grade` job (S-03, middle of list)
5. V-06-05 Gap report coverage measures in the summary (S-54, middle of list)
6. V-06-06 Readiness matrix route GET /coverage/readiness (C-48)
7. V-06-07 CBI minimum never retired: CHECK and guard trigger (T-02)
8. V-06-08 Not-covered declaration route PUT .../declaration (L-02, end of list)
9. V-06-09 Step-up actions of the declaration and the pack download (7.1)
10. V-06-10 AttributeCatalogue type (7.3)
11. V-06-11 Mapping completeness reason state `unmapped` (7.6)
12. V-06-12 GET /ref/typologies (7.8)

Spread: header items 1, 2, 3, 6, 7, 8, 9 of the carried list plus VA-19; 4 items from the four blocks 7.1, 7.3, 7.6, 7.8.

### V-06-01 GET /cbi-catalogue, the CBI item read   (source: header Implemented here, VA-19; result: GAPS 2)
- T1 SQL: PASS - `scn.cbi_catalogue_items` (phase-06.md:48 "CREATE TABLE IF NOT EXISTS scn.cbi_catalogue_items ("); `scenarios.visibility` (:90) and `scenario_catalogue_links.coverage` (:108) exist; every column of `CatalogueItemView` (:463) is in migration 0020.
- T2 Route: PASS - :479 "| GET | /cbi-catalogue | self | The 173 CBI items"; `self` is the API.md section 2 scope for "Everyone"; Cedar action `cbi_catalogue.read` (:751).
- T3 Repository and types: GAP - :396 "async fn active(&self, conn: &mut PgConnection) -> Result<Vec<CatalogueItem>, RepoError>;" returns items only; the view needs each item's scenario codes, and the only type that could carry them is `CatalogueQuery` (:851), which has no signature.
- T4 Logic: PASS - read route with the view rule stated at :463-465 ("no grade, no coverage kind, no restricted code"); the only refusal is the scope gate.
- T5 Audit: N/A - read only.
- T6 Cross-references: GAP - :764 "when { principal.kind == "human" && principal.active };" uses `principal.active`; phase-03.md:565 defines `status: String, // active | disabled` on entity User and no `active` attribute.
- T7 Consistency: PASS - 7.8 (:1041-1042) extends the same `catalogue.everyone` policy and does not change its base rule.
Gaps:
GAP V-06-01-a | MEDIUM | T3 | No method or query type returns each item with its scenario codes (full or partial links, normal visibility); `CatalogueQuery` is named only in AppState; `RegisterSnapshot` (:409) is defined nowhere | section 2, after CatalogueRepository (:393-398), with a signature | greps: `CatalogueQuery` 1 hit (:851); `RegisterSnapshot` 1 hit (:409); `links` in section 2 gives only `links_for_scenario` (:408)
GAP V-06-01-b | LOW | T6 | Attribute `principal.active` does not exist in the phase 03 User entity (`status`, phase-03.md:565); the policy would fail Cedar schema validation as written; the fix is `principal.status == "active"` | section 3.8 Cedar excerpt (:764) and attribute list (:812) | greps: `principal.active` in phase-03.md 0 hits; `active` on entity User 0 hits besides :565 `status`

### V-06-02 Register install by `akr-ctl packs seed`   (source: header Requirements carried, S-01; result: PASS)
- T1 SQL: PASS - tables :48, :72, :104; every column the seed writes is granted: :146 "GRANT SELECT, INSERT ON scn.cbi_catalogue_items TO akr_app;", :147 "GRANT UPDATE (status, superseded_by)", :153 "GRANT UPDATE (coverage, note, register_version)".
- T2 Route: N/A - CLI command; no route.
- T3 Repository and types: PASS - :394-395 `insert_missing`, `supersede`; :402-405 `insert_missing`, `update_from_register`, `upsert_links`; pack-file types :385-388 ("NewCatalogueItem", "NewScenario", "ScenarioRegisterUpdate", "NewLink").
- T4 Logic: PASS - steps 1-7 numbered (:510-531); exit 1 on mismatch (:511) and exit 2 for verify-data (:535); one transaction (:515 "One transaction (as `akr_app`)"); `--dry-run` rolls back (:533).
- T5 Audit: PASS - :529 "Audit `cbi_pack.seeded {catalogue_version, register_version," in step 6, listed at :732; it covers the supersede and insert writes of the same transaction.
- T6 Cross-references: PASS - `db migrate` (phase-00.md:624); `db harden` (phase-02.md:17 "`akr-ctl` (`db harden`, `audit verify`"); manifest of SHA-256 digests and signatures (phase-00.md:750 "a manifest of SHA-256 digests"); job `coverage_grade` (:587-601).
- T7 Consistency: PASS - 7.4 (:1002-1004) adds a goAML step after the CBI pack and says "The CBI pack steps of section 3.1 do not change."

### V-06-03 CBI gap report run and its build job   (source: header Requirements carried, S-02; result: GAPS 5)
- T1 SQL: PASS - `scn.gap_report_runs` (:230 "CREATE TABLE IF NOT EXISTS scn.gap_report_runs ("); active-run unique index (:254-255); grants for every column the job writes (:271-274); `document_object_id` FK to `admin.evidence_objects` (:241), which phase-02.md:353 defines.
- T2 Route: PASS - :485 "| POST | /coverage/cbi-gap-report/runs | dashboard.read, data.steward |"; the scopes admit M, CH, IA and DS; Cedar `gap_report.run` (:790-792) permits DS and M.
- T3 Repository and types: GAP - :429-432 use `NewGapReportRun` and `RunCompletion` (:431), neither defined with fields; the document type comes from a schema file the phase does not give (:338, :861).
- T4 Logic: PASS - steps 1-9 numbered (:632-669); 409 `run_active` (:629-630); single active run by the index (:253-255); retry rule (:632 "max attempts 3").
- T5 Audit: GAP - step 1 (:633 "1. `start`; a run that is not `queued` -> done.") changes status and `started_at` with no audit event; `gap_report.requested` (:630), `.completed` (:665) and `.failed` (:667-669) are named.
- T6 Cross-references: GAP - :638 "`cover`: bank name (phase 01 settings)" names no key (phase-00.md:238 has only `bank.timezone`); :721 "`E_IDEMPOTENCY_REPLAY`" is not a problem code (API.md:39-41 defines E_IDEMPOTENCY_REQUIRED, E_IDEMPOTENCY_REUSE and E_IDEMPOTENCY_IN_FLIGHT; replay is a response header). PASS: `put_generated` (phase-02.md:876), `RenderService::render` (phase-02.md:897), `PackService::publish` (phase-02.md:915), `pack.created` (phase-02.md:920), `POST /packs/verify` (phase-24.md:570).
- T7 Consistency: PASS - 3.3 payload `reason: "report"` (:589) matches 3.5 step 3 (:635 "reason: "report"").
Gaps:
GAP V-06-03-a | MEDIUM | T3 | Fields of `NewGapReportRun` (:429) and `RunCompletion` (:431) are not given; step 8 (:664-666) lists completion values only, and nothing lists the insert values | section 2 Create/Update structs (:388) | greps: `RunCompletion` 1 hit (:431); `NewGapReportRun` 2 hits (:388, :429)
GAP V-06-03-b | MEDIUM | T3 | Schema `cbi-gap-report.v1.json` is named (:338) and `CbiGapReportV1` is generated from it (:861), but no schema or field table is given; step 4 (:637-654) is prose | section 2 file list (:336-338) or a schema block in 3.5 | greps: `cbi-gap-report.v1` 1 hit (:338); `CbiGapReportV1` 1 hit (:861)
GAP V-06-03-c | MEDIUM | T5 | The `start` transition (:633) mutates the run and names no audit event; the audit table (:737) lists requested, completed and failed only; step 9 (:668) is unclear for the step 2 `fail(no_profile_run)` and `fail(register_missing)` (:634) | section 3.5 step 1 and step 9; audit table (:737) | greps: `gap_report\.` hits at :630, :665, :667-669, :737; no `started` event
GAP V-06-03-d | MEDIUM | T6 | Cover names "bank name (phase 01 settings)" (:638); no setting for the bank name exists in phase 00, 01 or 03 | section 3.5 step 4 (cover), or add the key in migration 0022 | greps: `bank.name`, `bank_name`, `legal_name`, `institution.name` in phase-00, -01, -03 and -10 to -13: 0 hits
GAP V-06-03-e | LOW | T6 | `E_IDEMPOTENCY_REPLAY` (:721) is not a problem code; API.md section 6 has no such code and replay is a response header (API.md:39-41) | section 3.8 error mapping (:721) | greps: `E_IDEMPOTENCY_REPLAY` in docs/API.md and phase-00.md: 0 hits

### V-06-04 Readiness grading and the `coverage_grade` job   (source: header Requirements carried, S-03; result: GAPS 3)
- T1 SQL: PASS - `scn.readiness_grades` (:186 "CREATE TABLE IF NOT EXISTS scn.readiness_grades ("); its declaration and grade columns are granted (:268-270); settings and flags inserted at :301-319; `source_id` FK to `admin.sources` (:190), defined at phase-05.md:59.
- T2 Route: N/A - job, no route; the route that reads its output is V-06-06.
- T3 Repository and types: GAP - :415 `upsert_grades` and :422 `lapse_resolved` take their parameters; :451 `AvailabilitySource::latest` is defined; but the grader's return type `Grading` and its input `Flags` (:544-545) are undefined, and `RegisterSnapshot` (:409) is undefined.
- T4 Logic: PASS - attribute states (:548-553), scenario grade rules 1 to 6 (:557-566), item grade rules 1 to 3 (:572-578), job steps 1 to 5 (:593-601); one transaction in step 3 (:596 "One transaction: `upsert_grades` for the bank scope").
- T5 Audit: PASS - step 3 :598 "audit `readiness.declaration_lapsed {scenario_code, grade}`" and step 4 :599 "Audit `readiness.graded {reason, scope_counts"; both listed at :733 and :735; the upsert is covered by `readiness.graded`.
- T6 Cross-references: GAP - phase 05 enqueues `coverage_grade` (phase-05.md:1516-1517 "enqueues `coverage_grade`"): PASS; phase 03 flag hook (phase-03.md:1305 "enqueues coverage_grade {reason: "flag"}"): PASS; phase 05 `admin.field_availability` quality states (phase-05.md:331 "quality IN ('good', 'partial', 'poor', 'unpopulated', 'absent',"): PASS; phase 04 setting hook: no match, see V-06-04-b.
- T7 Consistency: GAP (LOW) - reason-state comment at :192 lists `"missing"` while the grader uses `absent` (:550-551, :559); see V-06-04-c.
Gaps:
GAP V-06-04-a | MEDIUM | T3 | `Grading` (return type) and `Flags` (input) of `ReadinessGrader::grade` (:544-545), and `RegisterSnapshot` (returned by `all_with_links`, :409), are named but not defined | section 2 value objects (:377-384) or section 3.2 | greps: `Grading` 1 hit (:545); `RegisterSnapshot` 1 hit (:409); `Flags` as a type 1 hit (:545); the only `flags` (:383) is a field of GradeBasis
GAP V-06-04-b | MEDIUM | T6 | Section 4 (:875) says "Phase 04: the setting change target enqueues `coverage_grade` on activating `coverage.maintained_fields`"; phase 04 has no such hook (its generic setting target writes `setting.updated`, phase-04.md:915) | phase 04 setting change target section, or a note there | greps: `maintained_fields` in phase-04.md 0 hits; `coverage_grade` in phase-04.md 0 hits
GAP V-06-04-c | LOW | T7 | Same no-source-column state is `absent` in the grader (:550-551, :559) and DR-38 (research.md:49), but `missing` in the reason-state comment (:192) and in 7.6 (:1015); phase-05.md:331 uses `absent` | section 1 comment (:192) and 7.6 wording (:1015) | greps: `"missing"` in phase-06.md section 1 comment (:192) and 7.6 (:1015) only

### V-06-05 Gap report coverage measures in the summary   (source: header Requirements carried, S-54; result: PASS)
- T1 SQL: PASS - `summary JSONB` (:240) carries the measures; the comment at :239 names "coverage measures (S-54)".
- T2 Route: PASS - :484 "| GET | /coverage/cbi-gap-report | dashboard.read, data.steward |"; Cedar `gap_report.read` (:786-788) admits M, CH, IA and DS.
- T3 Repository and types: PASS - `GapReportView` (:468) carries the sealed document; the measures are fields of its `summary` written at step 8 (:664).
- T4 Logic: PASS - :642-643 "`items_with_scenario` (172 full or partial, 1 proxy, as counts and share of 173)" and "`items_data_ready` (A or B, count and share)"; denominators stated.
- T5 Audit: PASS - :665 "audit `gap_report.completed {run_id, items: {A, B, C, D}, packs: 4}`" and :737 in the audit table; the `summary` write is in the same step 8.
- T6 Cross-references: PASS - CBS module of an attribute (phase-05.md:1149-1150 "module: core | kyc | teller"); `source_ref` and `null_pct` columns (phase-05.md:333 and :335); research baseline is restated from 3.1 (:538 "A 47, B 80, C 26, D 20").
- T7 Consistency: PASS - :536-538 (99 cbi, 62 akr, grades 47/80/26/20) match :642-643; "172 full or partial, 1 proxy" matches the Art.3 no.9 proxy rule (:537).

### V-06-06 Readiness matrix route GET /coverage/readiness   (source: header Requirements carried, C-48; result: GAPS 3)
- T1 SQL: PASS - filter columns `scope`, `source_id`, `grade`, `declaration_status` exist (:189-199); indexes :218-224.
- T2 Route: PASS - :482 "| GET | /coverage/readiness | config.author, dashboard.read |"; the two scopes admit CFG, DS, MRO, M, CH and IA, the roles of Cedar `readiness.read` (:766-769 and :778-780).
- T3 Repository and types: PASS - :416 `ReadinessRepository::list(conn, ReadinessFilter, Keyset) -> Page<ReadinessGrade>`; the filter fields are the route's query parameters (:482); `ReadinessView` fields listed at :466-467.
- T4 Logic: GAP - no rule says what a list returns for a restricted-scenario row: :720 "E_NOT_FOUND (also the masked restricted scenario)" and :771-776 define a single-read 404 only, and ReadinessView points to "the masked form of section 3.8" (:467), which 3.8 does not define.
- T5 Audit: N/A - read only.
- T6 Cross-references: GAP - :772 "@id("scenario.restricted") @deny("masked")" needs Scenario and ReadinessGrade to be masked kinds; phase-03.md:406-407 says masked kinds are registered from phase 15 and phase-15.md:908-909 lists none of them. `Keyset` and `Page<T>` (:416, :439) are defined in no plan file.
- T7 Consistency: PASS - :753 "readiness.read | System, ReadinessGrade | CFG, DS, MRO; M, CH, IA" matches Cedar :778-780.
Gaps:
GAP V-06-06-a | HIGH | T4 | A readiness list has no rule for restricted scenarios: a single 404 cannot apply to one row, "the masked form of section 3.8" (:467) is not defined, so a DS list could show a restricted scenario's grade | section 3.8 (new step for the list) and ReadinessView (:466-467) | greps: `masked form` 1 hit (:467); "masked" in 3.8 (lines 718-830) has no list rule; `list` next to `mask` in phase-03.md: 0 hits
GAP V-06-06-b | MEDIUM | T6 | Scenario and ReadinessGrade are not registered as masked resource kinds; without it a refused read gets 403, not the 404 that section 6 expects (:939-941) | section 4 authz adapters (AppState list) or 3.8 | greps: phase-03.md:389 (`kind.masked()` check) and :406-407 (kinds list); `ReadinessGrade` or `Scenario` with "mask" in phase-15 and phase-16: 0 hits
GAP V-06-06-c | LOW | T6 | `Keyset` and `Page<T>` (:416, :439) are used but defined in no plan file or in .claude/rules; phase 03 uses `Cursor` and `Page<UserRow>` (phase-03.md:436) | section 2 repository notes (:439) or the phase 00 shared types | greps: `struct Keyset`, `Keyset {` and `Keyset =` in docs/plan and .claude/rules: 0 hits

### V-06-07 CBI minimum never retired: CHECK and guard trigger   (source: header Requirements carried, T-02; result: PASS)
- T1 SQL: PASS - :92 "status TEXT NOT NULL DEFAULT 'registered' CHECK (status IN ('registered', 'retired'))"; :99 "CHECK (NOT (pack = 'cbi' AND status = 'retired'))"; trigger function :117 "CREATE OR REPLACE FUNCTION scn.register_guard()"; triggers :133-144; no DELETE grant on `scn` tables (:146-154).
- T2 Route: N/A - no route in this phase; the retirement route is phase 16's (phase-16.md:904 "POST /api/v1/scenarios/{id}/retire-requests").
- T3 Repository and types: PASS - :404 `update_from_register(tx, code, u: &ScenarioRegisterUpdate)`; the granted columns (:149-151) and the seed's step 3.4 list (:524-525) do not include `status`.
- T4 Logic: PASS - 3.1 step 3.4 (:524-527): a `cbi` code "cannot be" retired by the seed; an `akr` code "waits for phase 16's retirement through a change request".
- T5 Audit: N/A - the refusal is a constraint or trigger; the seed event is `cbi_pack.seeded` (:529-530).
- T6 Cross-references: PASS - phase-16.md:904 (retire-requests, change request `scenario`); phase-16.md:1032 "live | retired | retirement change request" with "`cbi` never retired (`E_BASELINE_RETIRE`)".
- T7 Consistency: PASS - 7.5 (:1009-1010) keeps phase 06 Cedar actions and masked 404 unchanged; no 7.x block changes the CHECK or the trigger.

### V-06-08 Not-covered declaration route PUT .../declaration   (source: header Requirements carried, L-02; result: GAPS 4)
- T1 SQL: PASS - declaration columns (:198-208 "declaration_status TEXT NOT NULL DEFAULT 'none'"; "interim_route", "owner_id", "reason", "review_due", "declared_by", "declared_at", "version") and their grants (:268-270); owner check reads `admin.users` `kind` and `status` (phase-01.md:56 "kind admin.principal_kind NOT NULL", :59 "status admin.user_status"); the setting row at :302.
- T2 Route: PASS - :483 "| PUT | /coverage/readiness/{id}/declaration | config.approve |"; API.md section 2 gives `config.approve` to CHK, M, CH, MRO; Cedar `readiness.declare` (:782-784 "@id("readiness.mlro_declares") @role("M")") permits M only.
- T3 Repository and types: GAP - `ReadinessRepository::declare` (:419-420) is defined with its parameters; `DeclarationInput` (:387) and `Declaration` (:382) have fields; but `DeclarationService` (heading :603, AppState :852) has no method named with parameters.
- T4 Logic: GAP - steps 1 to 5 are numbered with codes (:606-618), but (a) the bounds 30 to 365 of `coverage.declaration_review_max_days` (:833 "30-365, governed") are not in the setting row (:302-303, floor NULL), and (b) the DS refusal is a scope refusal, but the scope gate is not a step and section 6 gives E_FORBIDDEN (see c).
- T5 Audit: GAP - :617 "the audit event keeps the previous values' hash"; the step 5 payload (:618 "`readiness.declared {scenario_code, grade, interim_route, owner_id, review_due, replaced: bool}`") and the table (:734) have no hash field.
- T6 Cross-references: PASS - `StepUpService::consume` (phase-01.md:731 "`consume(tx, ctx, header, action: StepUpAction, resource_ref`"); class `approval` (phase-01.md:709-710); header `X-StepUp-Assertion` (phase-01.md:733); API.md:968 `E_STEPUP_REQUIRED`, :977 `E_VERSION_CONFLICT`, :982 `E_NOT_ALLOWED_IN_STATE`, :985 `E_VALIDATION`, :994 `E_PRECONDITION_REQUIRED`.
- T7 Consistency: PASS - 7.1 (:977) "`StepUpAction { name: "readiness.declare", class: approval }`" settles the class named in 3.4 step 1 (:606).
Gaps:
GAP V-06-08-a | MEDIUM | T3 | `DeclarationService` is named (:603, :852) but no method is given with its parameters; steps 1 to 5 read as a route handler | section 3.4 (add the service signature) | greps: `DeclarationService` 2 hits (:603, :852)
GAP V-06-08-b | MEDIUM | T4 | The 30-365 bounds for `coverage.declaration_review_max_days` (:833) are not stored: the row (:302-303) has floor NULL, though phase 00 stores governed bounds in `floor` ("regulatory limit {"min": n} | {"max": n}", phase-00.md:208) | migration 0022 setting row (:302-303) | greps: `floor` values in migration 0022: NULL on every row (:302-313)
GAP V-06-08-c | LOW | T4 | The DS refusal is named 403 `E_FORBIDDEN` (:950 "as DS -> 403 `E_FORBIDDEN`"), but DS does not hold `config.approve`, so the scope gate answers `E_SCOPE` (API.md:969 "Token or session lacks the scope"); 3.4 has no scope step | section 3.4 step 1 and section 6 item 5 (:950) | greps: `E_SCOPE` in phase-06.md: 0 hits in section 3.4; the scope-refused lines in section 6 are :938 (A1 and CFG) only
GAP V-06-08-d | MEDIUM | T5 | "keeps the previous values' hash" (:617) has no field in the event payload (:618) or the audit table (:734) | section 3.4 step 5 | greps: `hash` in phase-06.md: 1 hit (:617)

### V-06-09 Step-up actions of the declaration and the pack download   (source: 7.1, DR-359; result: GAPS 1)
- T1 SQL: N/A - no table in 7.1.
- T2 Route: N/A - the two routes are checked in V-06-08 and 3.6 (:694-697); 7.1 changes only their step-up class.
- T3 Repository and types: PASS - `StepUpAction { name, class }` is the phase 01 type (phase-01.md:427 "`StepUpAction { name: &'static str, class }`"); 7.1 declares both actions with it (:977-978).
- T4 Logic: PASS - 7.1 says "Every mention of a class `readiness_declare` or `pack_download` (sections 3.4, 3.6 and 4) reads as these action names" (:979-980).
- T5 Audit: N/A - no mutation in 7.1.
- T6 Cross-references: GAP - :980 points to "section 4.5's" bullet; section 4 has no subsections (headings at :839-881 are plain bullets); the other names are correct: class `export` (phase-01.md:709-710 "filing`, `export`, `backfill`"), DR-230 (research.md:241), DR-359 (research.md:370).
- T7 Consistency: PASS - 3.6 (:694) says "a single-use step-up assertion of class `pack_download`", which 7.1 explicitly reads as the action name; DR-359 (research.md:370) gives the same result.
Gaps:
GAP V-06-09-a | LOW | T6 | "section 4.5" (:980) does not exist; the sentence it names is the bullet "Phase 01: two step-up action classes" in section 4 (:870) | 7.1 wording (:980) | greps: `4.5` in phase-06.md: 1 hit (:980); `^### 4` in phase-06.md: 0 hits

### V-06-10 AttributeCatalogue type   (source: 7.3, DR-367; result: PASS)
- T1 SQL: N/A - domain type loaded from a file; no table.
- T2 Route: N/A - no route.
- T3 Repository and types: PASS - :993 "Methods here: `get(key)`, `keys()`, `needs_declaration(key)`"; the fields are those of the phase 05 entry (phase-05.md:1151 "label: {ar, en}, needs_declaration: bool, used_by: [...]").
- T4 Logic: PASS - key form `<namespace>.<name>` and the two namespaces registered in code (:993-997 "`feat.`" and "`rating.`"); the grader takes the type unchanged (:997).
- T5 Audit: N/A - no mutation.
- T6 Cross-references: PASS - phase-05.md:1147 "**The attribute catalogue** `backend/catalogue/canonical-attributes.v1.json` ships with the release"; phase-10.md:1549 "every key must name a canon column (section 3.8 test)"; phase-13.md:940 "`feature_keys()`"; phase-15.md:1856 "`register_rating_attributes()`"; phase-10.md:229 "CREATE TABLE IF NOT EXISTS admin.attribute_values (" is the separate AttributeService.
- T7 Consistency: PASS - :545 "catalogue_attrs: &AttributeCatalogue" is what 7.3 says the grader keeps; :834-835 "catalogue keys with `needs_declaration = true`" matches 7.3 (:993).

### V-06-11 Mapping completeness reason state `unmapped`   (source: 7.6, phase 10 section 3.8; result: GAPS 1)
- T1 SQL: N/A - no column change; the reason-state comment at :192 gains `unmapped` (7.6 :1013).
- T2 Route: N/A - no route.
- T3 Repository and types: GAP - 7.6 says "The grader of section 3.2 reads phase 10's `MappingCompleteness`" (:1014), but the grader signature (:544 "grade(register, availability: &[SourceAvailability], maintained: &[AttributeKey], feeds") and the job's input list (:593 "1. Load the register (`all_with_links`), `AvailabilitySource::latest`, the setting, the flags") do not include it.
- T4 Logic: PASS - 7.6 gives the rule: a monitored attribute with no published mapping for the source is `unmapped` and grades D (:1014-1015); "Draft and testing versions do not count." (:1015).
- T5 Audit: N/A - the grade change is audited by `readiness.graded` (:599).
- T6 Cross-references: PASS - `MappingCompleteness { source_id, entity, column, published_version }` is defined in phase 10 (phase-10.md:2021); phase-10.md:2020 "The grader (phase 06 section 3.2) reads this through".
- T7 Consistency: PASS - 3.2 already defers the mapping half to phase 10 ("From phase 10 a missing or draft mapping ... also degrades or blocks", :568-569) and the trailer agrees (:1053); 7.6 gives the mechanism and grades D, which is within "degrades or blocks". Not counted: the sentence "this phase reads availability only" (:569) is not replaced in words, but phase 10 and the trailer say the same thing.
Gaps:
GAP V-06-11-a | MEDIUM | T3 | The mapping completeness input that 7.6 says the grader reads is not in the grader signature (:544-545) nor in the job's input step (:593) | section 3.2 signature and section 3.3 step 1 | greps: `MappingCompleteness` in phase-06.md 1 hit (:1014); `published_version` in phase-06.md 1 hit (:1014)

### V-06-12 GET /ref/typologies   (source: 7.8, DR-381; result: GAPS 2)
- T1 SQL: PASS - `scn.scenarios.typologies SMALLINT[]` (:82 "typologies SMALLINT[] NOT NULL DEFAULT '{}'") keeps the numbers as 7.8 says (:1040 "Scenarios keep `typologies SMALLINT[]` (the numbers)"); the file `backend/catalogue/typologies.v1.json` is not a table.
- T2 Route: PASS - :1034 "| GET | /ref/typologies | self | The shared typology set"; API.md:365 "Shared typology set: number, tag and label (phase 06, DR-381)" gives the same scope; Cedar `typology.read` is added to `catalogue.everyone` (:1041-1042).
- T3 Repository and types: PASS - response shape :1038 "`[{number, tag, label: {ar, en}}]`" is given.
- T4 Logic: PASS - loader rules :1039-1040 "The loader refuses a file whose numbers are not exactly 1 to 21 or whose tags repeat."
- T5 Audit: N/A - read only.
- T6 Cross-references: GAP - :1042-1043 says "API.md 3.6 marks the row", but the row is in API.md section 3.8 (API.md:365, under "### 3.8 Reference data and calendars (M8)" at API.md:338); section 3.6 starts at API.md:278. PASS otherwise: DSL.md:176 "Tags from the shared typology set" (the `tag` strings); research/27 file exists (`research/27-transaction-monitoring-scenarios.md`).
- T7 Consistency: GAP - 7.8 says the catalogue router has 4 routes (:1030 "joins the `catalogue` router (4 routes)"), but the header says "13 routes" (:13), section 4 says "catalogue (3 operations)" and "13 in all" (:857), and section 6 item 6 says "Swagger UI lists 13 new operations" (:961).
Gaps:
GAP V-06-12-a | LOW | T6 | "API.md 3.6" (:1043) should read API.md section 3.8; the row is at API.md:365 under section 3.8 (API.md:338) | 7.8 last sentence (:1043) | greps: `ref/typologies` in docs/API.md 1 hit (:365); `### 3.6` at API.md:278
GAP V-06-12-b | LOW | T7 | Route and operation counts still say 13 after 7.8 makes the catalogue router 4 routes (:1030) | header (:13), section 4 (:857), section 6 item 6 (:961) | greps: "13" counts at :13, :857, :961

## Summary

- Items checked: 12.
- Items fully PASS: 4 (V-06-02, V-06-05, V-06-07, V-06-10).
- Items with at least one GAP: 8 (V-06-01, V-06-03, V-06-04, V-06-06, V-06-08, V-06-09, V-06-11, V-06-12).
- GAP lines by severity: HIGH 1, MEDIUM 12, LOW 8 (21 in all).

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-06-01-a | MEDIUM | T3 | Query returning each item with its scenario codes; `CatalogueQuery` has no signature; `RegisterSnapshot` undefined | section 2 after CatalogueRepository |
| V-06-01-b | LOW | T6 | `principal.active` is not an attribute; phase 03 has `status` | section 3.8 Cedar (phase-06.md:764) |
| V-06-03-a | MEDIUM | T3 | Fields of `NewGapReportRun` and `RunCompletion` | section 2 Create/Update structs |
| V-06-03-b | MEDIUM | T3 | Schema `cbi-gap-report.v1.json` content | section 2 file list or 3.5 |
| V-06-03-c | MEDIUM | T5 | Audit event for the `start` transition; step 9 wording for step 2 failures | section 3.5 steps 1 and 9 |
| V-06-03-d | MEDIUM | T6 | Setting key for the bank name | section 3.5 step 4; migration 0022 |
| V-06-03-e | LOW | T6 | `E_IDEMPOTENCY_REPLAY` is not a problem code | section 3.8 (phase-06.md:721) |
| V-06-04-a | MEDIUM | T3 | `Grading`, `Flags`, `RegisterSnapshot` undefined | section 2 value objects or 3.2 |
| V-06-04-b | MEDIUM | T6 | Phase 04 hook for `coverage.maintained_fields` and `coverage_grade` | phase 04 setting change target |
| V-06-04-c | LOW | T7 | `absent` (grader) vs `missing` (reason comment, 7.6) | section 1 comment (:192), 7.6 (:1015) |
| V-06-06-a | HIGH | T4 | List rule for restricted-scenario rows; "masked form of section 3.8" undefined | section 3.8 (new list step); ReadinessView (:467) |
| V-06-06-b | MEDIUM | T6 | Scenario and ReadinessGrade not registered as masked kinds | section 4 authz adapters |
| V-06-06-c | LOW | T6 | `Keyset` and `Page<T>` defined in no plan file | section 2 repository notes or phase 00 |
| V-06-08-a | MEDIUM | T3 | `DeclarationService` has no method signature | section 3.4 |
| V-06-08-b | MEDIUM | T4 | Bounds 30-365 not in the setting floor | migration 0022 (phase-06.md:302) |
| V-06-08-c | LOW | T4 | DS refusal code is E_FORBIDDEN; scope gate gives E_SCOPE | section 3.4 step 1; section 6 item 5 (:950) |
| V-06-08-d | MEDIUM | T5 | Previous-values hash not in the `readiness.declared` payload | section 3.4 step 5 (phase-06.md:618) |
| V-06-09-a | LOW | T6 | "section 4.5" does not exist | 7.1 wording (phase-06.md:980) |
| V-06-11-a | MEDIUM | T3 | Mapping completeness input not in the grader signature or job inputs | section 3.2 signature (:544-545); section 3.3 step 1 (:593) |
| V-06-12-a | LOW | T6 | "API.md 3.6" should be section 3.8 | 7.8 last sentence (:1043) |
| V-06-12-b | LOW | T7 | Route and operation counts still say 13 | header (:13), section 4 (:857), section 6 (:961) |

## Notes outside the sample (not counted as GAP lines)

- Cedar scope of restricted scenarios: the 3.8 table (phase-06.md:752) says `scenario.list` and `scenario.read` are restricted to CFG and MRO, but the forbid at :772-776 names only `scenario.read` and `readiness.read`. As written, GET /scenarios (:480, :767) would show restricted scenario codes to DS.
- Cedar `readiness.declare` (:782-784) has no restricted-scenario check, while M cannot read restricted rows (:771-776). The declare response would reveal such a row.
- The phase file has no "## Decisions" and no "## Open doubts" section (grep: 0 hits each), which tools/agent/b3-conventions.md (Method) requires. No sampled item depended on them.
