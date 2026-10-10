# Gap pass 1, run 3a (targets 05 to 10; Haiku draft for lead review)

## Target phase 05 (sources, read-only connection and discovery)

### G1-05-01 discovery_seal_report enqueues two jobs that phase 05 does not name
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-05.md section 3.6 (discovery runs) and section 4 (job kinds, the `discovery_seal_report` entry at line 1334)
- Sources: phase-06.md:872 "`discovery_seal_report` enqueues `coverage_grade` after a profile run"; phase-10.md:2206 "`discovery_seal_report` enqueues `drift_scan` in its transaction"; phase-10.md:1916 "enqueued by phase 05 `discovery_seal_report` after a"
- Check: grep -n "coverage_grade" docs/plan/phase-05.md -> none; grep -n "drift_scan" docs/plan/phase-05.md -> none; grep -n "discovery_seal_report" docs/plan/phase-05.md -> lines 19, 358, 475, 479, 811, 1036, 1137, 1334 (no enqueue of another job)
- Proposed 7.x text:
  "### 7.N discovery_seal_report enqueues the phase 06 and phase 10 jobs (gap pass 1, run 3a)
  In its transaction, after a completed profile run, `discovery_seal_report` enqueues `coverage_grade` (phase 06, section 3.3,
  coalesce key `coverage_grade`, payload `{reason: "profile_run"}`, the value phase 06 section 3.3 defines for a profile run) and `drift_scan` (phase 10, section 3.5, payload `{source_id, run_id}`).
  Both enqueues are idempotent under the coalesce keys. The job's own idempotency rule (a run with `report_object_id` set is skipped) is unchanged."
  Lead to confirm the reason string of `coverage_grade` against phase 06 section 3.3 before applying.

### G1-05-02 Calendar capture step and calendar block of the adapter pack hints are missing
- Severity: HIGH
- Category: Missing Logic
- Target: phase-05.md section 3.6 (discovery runs, the `discovery_run` steps) and section 1 (the adapter pack `hints` shape in the ICS BANKS pack, phase-05.md line 464 area)
- Sources: phase-09.md:669 "The adapter pack's hints gain a `calendar` block naming two approved objects"; phase-09.md:1035 "`akr-ingestd` gains one step inside `discovery_run`"; phase-09.md:1103 "`discovery_run` gains the calendar capture step (ingestd)"; phase-09.md:672 "row cap `calendar.capture_max_rows`"
- Check: grep -n -i "calendar" docs/plan/phase-05.md -> 3 hits (lines 1006, 1096, 1513), none is a capture step or a hints block; grep -n "HOLIDAY\|CURRENCY" docs/plan/phase-05.md -> none. Setting `calendar.capture_max_rows` exists in phase-09.md:369.
- Proposed 7.x text:
  "### 7.N Calendar capture inside discovery_run (phase 09 section 3.3)
  The ICS BANKS adapter pack's `hints` gain a `calendar` block naming two approved objects: `HOLIDAY` and `CURRENCY` (the columns phase 09 section 3.3 lists). `discovery_run` (akr-ingestd, section 3.6) gains one step after the object profiles: the calendar capture of phase 09 section 3.3, which reads each object whole with the row cap `calendar.capture_max_rows`, through `SelectProof` on each statement and `E_OBJECT_NOT_APPROVED` for an object outside the approved list. A cap hit records the finding `calendar_capture_truncated` and the run is not sealed as complete. Rows go to `ref.cbs_calendar_captures` in the run transaction. No customer, account or posting table is read; no new statement form and no new connection."
  Two sources (phase 09 section 3.3 and this text) must match on the object names; the lead confirms them against the ICS BANKS signature in phase 05 section 1.

### G1-05-03 Control-totals block of the adapter pack discovery.json is missing
- Severity: HIGH
- Category: Missing Logic
- Target: phase-05.md section 1 (the ICS BANKS pack `discovery.json`, line 464 area) and section 3.6 (discovery runs: the pack hints read)
- Sources: phase-12.md:1133 "the adapter pack `discovery.json` gains a `control_totals` block (object names, date columns, branch, currency, ledger and sign columns: data, not code)"; phase-12.md:1114 readiness "`control_totals_plan` (each active source's adapter pack names its control-totals objects)"
- Check: grep -n "control_totals\|ControlTotals" docs/plan/phase-05.md -> none; grep -n "discovery.json" docs/plan/phase-05.md -> lines 464, 1053 (no control-totals key)
- Proposed 7.x text:
  "### 7.N control_totals block in the adapter pack (phase 12 section 4.5)
  A source's adapter pack `discovery.json` may carry a `control_totals` block: the object names, the date columns, the branch, currency, ledger and sign columns of the bank's control totals (data, not code; read by phase 12's `ControlTotalsReader`). An absent block is a declaration of absence, not an error (phase 12 readiness warns). The block is validated by the pack loader like the other `discovery.json` keys; each named object must be in the approved list before any read."

### G1-05-04 Port and governor names that phases 11 and 12 use are not in phase 05
- Severity: MEDIUM
- Category: Missing Integration
- Target: phase-05.md section 3.4 (the `SourceReader` port, line 586 area) and section 3.5 (`LoadGovernor`, line 941 area)
- Sources: phase-11.md:637-640 "Extends the phase 05 port" with `fetch_keyset`, `branch_dates`, `as_of`; phase-11.md:750 "`LoadGovernor::for_source` (phase 05)"; phase-11.md:766 "`LoadGovernor::acquire(Read)`"; phase-12.md:1134 "`StatementKind::ControlTotals` added to the `LoadGovernor`"; phase-11.md:524 `AsOfMarker` and `KeyTuple` live in phase 11
- Check: grep -n "fetch_keyset\|branch_dates\|as_of" docs/plan/phase-05.md -> none; grep -n "for_source" docs/plan/phase-05.md -> none; grep -n "StatementKind" docs/plan/phase-05.md -> one hit (line 941, `acquire(&self, kind: StatementKind)`), no variant list anywhere in the file
- Proposed 7.x text:
  "### 7.N Port extension and governor entry points used by phases 11 and 12
  (a) `SourceReader` is extended, not changed, by phase 11 (its section 2, `fetch_keyset`, `branch_dates` and `as_of`); the phase 05 methods above stay as they are. (b) `LoadGovernor::for_source(source_id)` builds the governor of section 3.5 from the budget in force; it is the constructor that phase 11's `ingest_load` step 1 calls. (c) `StatementKind` is a closed enum; this phase lists its variants. Phase 11 uses `Read`; phase 12 adds `ControlTotals`. The variant list is UNCLEAR: the lead chooses the names and the rule for each variant (dictionary, probe, data and the others that section 3.4 step 7 distinguishes)."
  UNCLEAR: the phase 05 text never names the variants of `StatementKind`; the lead confirms whether `Read` is the variant name meant by phase 11.

### G1-05-05 SelectProof moves to crate sqlproof and the keyset statement form is added
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-05.md section 3.4 (statements, the crate layout of line 469 and step 7 of the statement grammar, line 904 area)
- Sources: phase-10.md:2203-2206 "`SelectProof`, `ProofScope` and `ProvenSelect` move from `crates/sources` to ... `crates/sqlproof`... The statement grammar gains the keyset form ... and `FETCH FIRST :n ROWS ONLY`"; phase-11.md:502 "`sqlproof/` unchanged: every generated read is proven before it runs (phase 05, DR-85)"
- Check: grep -n "sqlproof" docs/plan/phase-05.md -> none; grep -n "keyset" docs/plan/phase-05.md -> only the repository list (line 573), not a CBS statement form; phase-05.md:469 places `SelectProof` in `crates/sources`; phase-05.md:904 has `FETCH FIRST` only as one row-cap form
- Proposed 7.x text:
  "### 7.N SelectProof lives in crates/sqlproof (phase 10 section 2, DR-85)
  `SelectProof`, `ProofScope` and `ProvenSelect` move from `crates/sources` to the pure crate `crates/sqlproof`; `crates/sources` re-exports them, so the call sites of this phase are unchanged. The statement grammar of section 3.4 step 7 gains the keyset form (a row-value comparison over bound parameters only, the `after` key of phase 11) and `FETCH FIRST :n ROWS ONLY` as a row cap. Both stay single-object, single-statement `SELECT`s proven before they run. The crate-layout line of this phase is read as the state after phase 10."
  Note: phase-11.md:502 says `sqlproof` is 'unchanged' in phase 11, which is consistent with phase 10 creating it; the lead confirms the order.

### G1-05-06 code_suggestions key of discovery.json is missing
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-05.md section 1 (adapter pack `discovery.json`, line 464 area) and section 3.6
- Sources: phase-10.md:2208-2209 "`discovery.json` may carry `code_suggestions` (a CBS code and list to a canonical code, for the prefill of section 3.4)"
- Check: grep -n "code_suggestions" docs/plan/phase-05.md -> none; phase-05.md:464 lists `discovery.json (hints)` with no key list
- Proposed 7.x text:
  "### 7.N discovery.json code_suggestions (phase 10 section 3.4)
  A pack's `discovery.json` may carry `code_suggestions`: a CBS code and list mapped to a canonical code, read by phase 10's code-map prefill. It is data, not code, and it is optional; an absent key means no suggestions."

### G1-05-07 Listener reload on reconnect for akr_source is missing
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-05.md section 3.5 (the `LoadGovernor` rebuild on `NOTIFY akr_source`, line 938 area) and section 4 (readiness `source_listener`, line 1369 area)
- Sources: phase-30.md:480 "Reconnect reload added to `akr_assess_policy`, `akr_assess_gateway` (phase 29) and `akr_source` (phase 05)"; phase-30.md:488 "its listener reload is item 2"
- Check: grep -n -i "reconnect\|reload" docs/plan/phase-05.md -> none about reconnect; phase-05.md:938 rebuild happens only on NOTIFY and on the hourly re-read
- Proposed 7.x text:
  "### 7.N Listener reload on reconnect (phase 30 DR-322)
  When the `LISTEN akr_source` connection of akr-ingestd is lost and re-established, akr-ingestd rebuilds the `LoadGovernor` of every source and reads the state, budget and access rows in full before it waits again. A notification is a hint and carries no state the reload relies on. The 30-second poll and the hourly re-read stay as the fallback."

### G1-05-08 Resume route: job and step-up class from phase 11 are not in phase 05
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-05.md section 4 (routes table, `POST /sources/{id}/resume`, line 660) and section 3.1 (state machine, resume)
- Sources: phase-11.md:703 "Phase 05 route, extended: a halted source resumes through `ingest_resume` with `ResumeBody` after a passing grant check; 202; **SU** `backfill` when the source is halted"; phase-11.md:934 "the phase 05 route extended"
- Check: grep -n "ingest_resume\|ResumeBody" docs/plan/phase-05.md -> none; phase-05.md:660 row: "Resume after a fresh passing check (`source.resume`)" with no step-up note
- Proposed 7.x text:
  "### 7.N Resume route (phase 11 section 2 and section 3.5)
  `POST /sources/{id}/resume` enqueues `ingest_resume` (akr-ingestd, phase 11 section 3.5) with the body `ResumeBody` after a passing grant check and returns 202. When the source is `halted`, the call also needs step-up class `backfill` (phase 01 closed list, existing value). The route otherwise keeps this phase's rules."
  Note: `ingest_resume` is a phase 11 job name; the `ingest_` prefix is allowed to akr_ingest (phase 00). Lead to confirm whether the job is owned by phase 05 or phase 11.

### G1-05-09 Dev fixture generator gains the PROCESS table and restore-to mode
- Severity: LOW
- Category: Missing Logic
- Target: phase-05.md section 6 or 4 (dev profile `fixtures`, `cbs-fixture-pg`, line 1340 area)
- Sources: phase-11.md:1328-1331 "The phase 05 `cbs-fixture-pg` gains a `PROCESS` table with per-branch `BANK_DATE`, a `TRANSACT`-shaped fixture of about one million rows over ten business days, a `restore-to` mode of `akr-ctl fixtures cbs`"
- Check: grep -n "PROCESS" docs/plan/phase-05.md -> lines 1004, 1124 (pack hint, not a generated table); grep -n "restore-to\|TRANSACT" docs/plan/phase-05.md -> none
- Proposed 7.N text: "The generator `akr-ctl fixtures cbs generate` of this phase includes the `PROCESS` table with per-branch `BANK_DATE` (the ICS BANKS signature object of section 1), a `TRANSACT`-shaped table of about one million rows over ten business days, and the `restore-to` mode of phase 11 section 4.2; all values invented."


## Target phase 06 (CBI catalogue, readiness grades and gap report)

### G1-06-01 AttributeCatalogue is used but never defined; phases 13 and 15 extend it
- Severity: HIGH
- Category: Missing Integration
- Target: phase-06.md section 2 (DDD, domain `coverage` types, line 340 area) and section 3.2 (the grader signature, line 540 area)
- Sources: phase-13.md:936-937 "`AttributeCatalogue` gains the active feature keys with their `requires` (extends phase 06 with method `feature_keys()`)"; phase-15.md:1853-1854 "`AttributeCatalogue` gains `rating.level` and `rating.cbi_level` (method `register_rating_attributes()`"; phase-06.md:540-541 "`catalogue_attrs: &AttributeCatalogue`"
- Check: grep -n "struct AttributeCatalogue\|AttributeCatalogue {" docs/plan/phase-06.md -> none; grep -n "AttributeCatalogue" docs/plan/phase-06.md -> one hit (line 541, as a parameter type). grep -n "AttributeService" docs/plan/phase-*.md -> phase-10.md:2173 only (UNCLEAR, see Summary)
- Proposed 7.x text:
  "### 7.N The AttributeCatalogue type (phase 13 and phase 15 extend it)
  This phase defines `AttributeCatalogue` in `domain::coverage`, loaded from `backend/catalogue/canonical-attributes.v1.json` (the file phase 05 ships, section 1 of phase 05). It holds one entry per canonical attribute with the fields that section 3.2 reads: the key, the family, the module, and `needs_declaration`. Methods of this phase: `get(key)`, `keys()` and `needs_declaration(key)`. Phase 13 adds `feature_keys()` (the active feature keys and their `requires`) and phase 15 adds `register_rating_attributes()` (keys `rating.level` and `rating.cbi_level`). Both are additions, not changes, of this type. The grader of section 3.2 takes `&AttributeCatalogue` unchanged.
  UNCLEAR: the key format of the shipped file is `<canon table>.<column>` (phase 05 section 1), while phases 13 and 15 name keys such as `rating.level`; the lead decides the key form before this text is applied."

### G1-06-02 akr-ctl packs seed also installs the goAML profile directories
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-06.md section 3.1 (`akr-ctl packs seed`, line 504 area) and section 4 (ctl commands, line 851 area)
- Sources: phase-07.md:482-483 "`akr-ctl packs seed` (phase 06 command, extended): after the CBI pack, for each `backend/packs/goaml/*/` directory in the manifest"; phase-07.md:855 "Phase 06: `packs seed` and `packs verify-data` extended to the goAML profile directories"
- Check: grep -n -i "goaml" docs/plan/phase-06.md -> none
- Proposed 7.x text:
  "### 7.N packs seed and packs verify-data also cover goAML profiles (phase 07 section 3.1 and section 5)
  `akr-ctl packs seed` and `akr-ctl packs verify-data` gain one step after the CBI pack: for each `backend/packs/goaml/*/` directory in the release manifest, the step runs phase 07's profile load (SHA-256 check, `put_generated` of the XSDs, version insert, activation of a code with no active version). The CBI pack steps of section 3.1 are unchanged."

### G1-06-03 GET /scenarios and GET /scenarios/{id} gain fields and scopes from phase 16
- Severity: MEDIUM
- Category: Missing Endpoint
- Target: phase-06.md section 2 (route table rows 476 and 477, the `scenarios` rows) and section 3.1 (register response)
- Sources: phase-16.md:1828-1829 "`GET /scenarios` gains the live and shadow version, the bank grade and the open-declaration flag; `GET /scenarios/{id}` gains versions, parameter sets and test summary and its scope gains `config.approve` and `dashboard.read`"; status.md row 16 lists the same change
- Check: grep -n "shadow" docs/plan/phase-06.md -> none; phase-06.md:476-477 scopes are `config.author` only
- Proposed 7.x text:
  "### 7.N Scenario register rows (phase 16 section 4.5)
  `GET /scenarios` returns the live version, the shadow version (or null), the bank grade and the `declaration_open` flag beside the rows of section 2. `GET /scenarios/{id}` returns the versions, the parameter sets and the test summary (the detail fields of phase 16 section 2); its scope becomes `config.author, config.approve, dashboard.read`. Versions and parameter sets are shown only where `scenario.version.read` allows (phase 16). Cedar actions and the masked 404 of section 3 are unchanged."

### G1-06-04 Phase 16 routes join the catalogue and coverage routers
- Severity: LOW
- Category: Missing Endpoint
- Target: phase-06.md section 2 (route table; the router list of section 1 header, line 346 area)
- Sources: phase-16.md:1830-1831 "`POST /cbi-catalogue/imports` joins the `catalogue` router and `GET /coverage/matrix` the `coverage` router"
- Check: grep -n "cbi-catalogue/imports\|coverage/matrix" docs/plan/phase-06.md -> none
- Proposed 7.N text: "Phase 16 adds `POST /cbi-catalogue/imports` to the `catalogue` router and `GET /coverage/matrix` to the `coverage` router (phase 16 section 2). The route rows are owned by phase 16; this phase's tables stay as they are, and the OpenAPI tags are unchanged."

### G1-06-05 readiness reason state `unmapped` and the mapping-completeness input are missing
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-06.md section 1 (the reason-state comment of `scn.readiness_grades`, line 188) and section 3.2 (grader rules, line 540-560)
- Sources: phase-10.md:2015-2021 "a monitored attribute ... is `unmapped` ... `unmapped` is one more reason state beside phase 06's `missing`, `unpopulated`, `unassessed`, `degraded` and `declaration_required`; it grades D as `missing` does"; phase-10.md:2210 "readiness reason state `unmapped` added (section 3.8)"
- Check: grep -n "unmapped" docs/plan/phase-06.md -> none; phase-06.md:188 state list is `missing | unpopulated | unassessed | degraded | declaration_required`
- Proposed 7.x text:
  "### 7.N Reason state unmapped (phase 10 section 3.8)
  The reason-state list of `scn.readiness_grades.reasons` (section 1 comment) gains `unmapped`. The grader of section 3.2 takes a new input `MappingCompleteness { source_id, entity, column, published_version }` (computed by phase 10 from `admin.mapping_versions.compiled`); an attribute that has no published mapping for the source is `unmapped`, which grades D exactly as `missing` does. Draft and testing versions do not count. The `scn.readiness_grades` columns are unchanged."

### G1-06-06 POST /packs, GET /packs/{id} and the download accept scope assessor.read
- Severity: MEDIUM
- Category: Missing Policy
- Target: phase-06.md section 2 (route rows 484 to 486, the scope column)
- Sources: phase-24.md:1164 "Phase 06 `POST /packs`, `GET /packs/{id}` and the download: accept the scope `assessor.read` beside `dashboard.read` and `data.steward` (AS holds no other scope; Cedar keeps AS to kind `assessor`)"
- Check: grep -n "assessor.read" docs/plan/phase-06.md -> line 19 only (prose, not a route scope); phase-06.md:484-486 scopes are `dashboard.read, data.steward`
- Proposed 7.N text: "Phase 24 adds the scope `assessor.read` to `POST /packs`, `GET /packs/{id}` and `GET /packs/{id}/download` (phase 24 section 4.5). The Cedar rules of phase 06 do not change: a role that holds `assessor.read` reaches only kind `assessor` (phase 24 Cedar)." (Lead confirms the Cedar wording against phase 24 section 3.11.)

### G1-06-07 Pack kind CHECK is widened again by phase 24 (certification)
- Severity: MEDIUM
- Category: Missing Table
- Target: phase-06.md section 1 (the `packs_kind_check` statement, lines 258-260)
- Sources: phase-24.md:235 "Pack kind `certification` (GV-35, the annual certification pack). The phase 02 CHECK, widened by phase 06, is replaced"; phase-24.md:233 "Pack kind `certification` (GV-35 ...) The phase 02 CHECK" (migration 0078 starts at line 212)
- Check: grep -n "certification" docs/plan/phase-06.md -> none; phase-06.md:260 CHECK list ends at `'vendor_request'`
- Proposed 7.N text: "Phase 24 replaces `packs_kind_check` with a CHECK that adds `certification` (phase 24 migration `0078_admin_verify_viewer`, section 1). The list of this phase is the base; the statement itself is not changed here. Phase 31 adds no kind (its kinds `correspondent` and `validator` are already in the list)."
  Note: the migration number is the one phase 24 states; the lead confirms the number in phase 24 section 1.

### G1-06-08 pack_build section 3.6 names only phase 24 as the next kinds source
- Severity: LOW
- Category: Incomplete Coverage
- Target: phase-06.md section 3.6 (first paragraph, "until phase 24 adds the rest", line 674 area)
- Sources: phase-31.md:872 "`pack_build` (phase 06, extended) ... `POST /packs` with kind `correspondent` or `validator`"; phase-31.md:901 "The `pack_build` handler builds the kinds `correspondent` and `validator`"; phase-24.md:1164 "the `unknown_pack_kind` refusal of phase 06 section 3.6 narrows to the kinds still unbuilt (correspondent and validator, P2)"
- Check: grep -n "until phase 24" docs/plan/phase-06.md -> line 674; no mention of phase 31 in phase 06 (grep -n "phase 31\|correspondent" -> none in section 3.6)
- Proposed 7.N text: "Section 3.6: `POST /packs` accepts three kinds until phase 24 adds the five kinds of its section 3.11 and phase 31 adds `correspondent` and `validator`; any other kind is refused with `unknown_pack_kind`. The `pack_build` job dispatches each kind to the builder that phase registers (phase 02 `PackService`)."


## Target phase 07 (goAML checker)

### G1-07-01 CheckService::validate_built is missing
- Severity: HIGH
- Category: Missing Integration
- Target: phase-07.md section 3.2 (checking files, `CheckService`, line 500 area) and section 4 (ports, the `goaml_check` handler, line 817 area)
- Sources: phase-21.md:1258 "**Phase 07 engine**: `CheckService::validate_built(bytes, profile, layers)` (the pipeline of section 3.4 for an already-built XML ...)"; phase-21.md:1345 "validation reuses the phase 07 engine"
- Check: grep -n "validate_built" docs/plan/phase-07.md -> none; grep -n "CheckService" docs/plan/phase-07.md -> lines 500, 817 (the upload path only)
- Proposed 7.N text:
  "### 7.N CheckService::validate_built (phase 21 section 4.5 item 6)
  `CheckService::validate_built(bytes: &[u8], profile: &ProfileVersion, layers: &[Layer]) -> Result<CheckOutcome, CheckError>` runs the pipeline of section 3.2 (steps 3 to 9, the same layers as an upload) on XML that the caller has already built and not uploaded. The upload path and its request body are unchanged. It is a pure library call: no file is stored by it, and the caller stores the outcome. The layers argument excludes L0 (phase 21 adds that layer itself)."
  The lead confirms the step numbers of section 3.2 before applying.

### G1-07-02 Rule codes NAME-01, DATE-01, ID-01, PKG-07 and CTR-01 to CTR-03 are not in the rule catalogue
- Severity: HIGH
- Category: Missing Logic
- Target: phase-07.md section 3.3 (the rule catalogue table, lines 600-615) and section 3.2 (package checks, PKG rules, lines 526-538)
- Sources: phase-21.md:1258 "rule codes `CONS-04`, `NAME-01`, `DATE-01`, `ID-01`, `PKG-07`, `CTR-01`, `CTR-02` and `CTR-03` added to the rule catalogue, which is closed in code"; phase-21.md:798 "a warn `DATE-01` (new in phase 21)"; phase-21.md:800 "raises `ID-01` (new in phase 21, warn ...)"; phase-21.md:814 "is a block `PKG-07` (new in phase 21 ...)"; phase-21.md:948 "Rule codes CTR-01 to CTR-03 are new in phase 21 and enter the phase 07 rule catalogue by change request (section 4.5)"
- Check: grep -n "NAME-01\|DATE-01\|ID-01\|PKG-07\|CTR-0" docs/plan/phase-07.md -> none; phase-07.md:611 has `CONS-04` already (so CONS-04 needs no change); phase-07.md:529-530 has PKG-04 to PKG-06 only
- Proposed 7.N text (rows for the table of section 3.3; the severities are those phase 21 gives, the rest UNCLEAR):
  | DATE-01 | a birth date with precision `year` (the month and day are UNKNOWN) | warn | |
  | ID-01 | a placeholder national identifier (the national ID is missing) | warn | |
  | PKG-07 | an attachment whose evidence status is still `pending` ("being checked") | block | |
  | CTR-01 | cash on one side of a CTR transaction and an account on the other | UNCLEAR (phase 21 gives no severity) | |
  | CTR-02 | both countries of a CTR transaction domestic | UNCLEAR (phase 21 gives no severity) | |
  | CTR-03 | amount sanity: an extreme or non-round amount | warn | |
  Each code is a Rust rule in `akr-goaml` (section 3.3, closed in code), enabled by the profile as the other rules are.
  Open items for the lead: NAME-01 has no definition anywhere in phase 21 (it is only listed in item 6), so no row can be written for it; CONS-04 is already in phase 07 line 611, so phase 21 item 6 is wrong to call it new (LOW wording fix in phase 21).


## Target phase 08 (frontend foundation)

### G1-08-01 Screen catalogue entries for D3 (phase 09) and T3 (phase 10), and the RailGroup values
- Severity: LOW
- Category: Incomplete Coverage
- Target: phase-08.md section 3.5 (the screen catalogue, `ScreenDef`, lines 236-250; the `RailGroup` type at line 239)
- Sources: phase-09.md:1111 "`screens.ts` already lists D3 at P1"; phase-10.md:2212 "the screen catalogue `screens.ts` gains T3 (section 5)"; phase-10.md:2245-2246 "`rail: { group: <the data group of phase 08's RailGroup>, ...}`" with `available: 'P1'`
- Check: grep -c "D3" docs/plan/phase-08.md -> 0; grep -c "T3" docs/plan/phase-08.md -> 0; grep -n "RailGroup" docs/plan/phase-08.md -> line 239 only (type name, no value list, no data group)
- Proposed 7.N text:
  "### 7.N Not-yet-built entries of screens.ts (phase 09 section 5, phase 10 section 5)
  The file lists every screen of DESIGN-SYSTEM 8.1 that a later phase builds, with `available: 'P1'` and `rail: null` until its phase: D3 (`/policy`, STUDIO, phase 09) and T3 (`/data/mapping`, STUDIO, phase 10, `available: 'P1'`). The `RailGroup` type has the values listed in DESIGN-SYSTEM 8.2 (UNCLEAR: the lead copies the list; phase 10 needs a group for data screens, and the name `data` is not in phase 08 today)."

### G1-08-02 The Name component is in the UX-25 requirement but not in the component table
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-08.md section 3.7 (the shared component layer table, lines 326-353) and the requirements line (line 30, UX-25)
- Sources: phase-14.md:1258 "the Name component (original and transliteration, chain alignment, mother's name, quality flags ...) is added to the phase 08 component layer for S1 and the customer screens"; phase-14.md:1332 "Phase 08's component list does not name it; this phase adds it to the layer."
- Check: grep -n "Name" docs/plan/phase-08.md -> line 30 (UX-25 "the shared Name component") and line 139 (lint script); no row for `Name` in the table of section 3.7
- Proposed 7.N text:
  "### 7.N The Name component (UX-25; phase 14, shared components in section 4)
  The shared component `Name` is a row of the section 3.7 table: it shows an original name and its transliteration, the chain alignment (father and grandfather), the mother's name and the quality flags, in both directions, with words for every flag (no colour alone). Phase 14 uses it on S1 and the customer screens; its props are the fields of the phase 14 Name view. The component is in the layer from phase 08; phase 14 does not change its file."
  Lead confirms the props against phase 14.


## Target phase 09 (reference data, calendars and policy pack)

### G1-09-01 ClockRegistry lacks the six clocks that phases 20 and 21 add
- Severity: HIGH
- Category: Missing Logic
- Target: phase-09.md section 3.5 (the closed registries, the clock list at lines 768-771)
- Sources: phase-20.md:920 "`freeze.limit` (hours, 24), `freeze.committee_report` (hours, 48), `freeze.second_report` (calendar months, 2), `refrain.clock` (working days, 7). These four entries are new in phase 20 and are added to phase 09's registry code"; phase-21.md:1016-1017 "`report.filing_target`: hours; shipped 24; bounds 1 to 24" and "`report.rework`: working days; shipped 10; bounds 5 to 20 ... `fixed: false`"; phase-21.md:1260 "the closed `ClockRegistry` gains `report.filing_target` and `report.rework`"
- Check: grep -n "freeze.limit\|freeze.committee\|freeze.second\|refrain.clock\|report.filing_target\|report.rework" docs/plan/phase-09.md -> none; the clock list (line 768-770) names only `rfi.response`, `ops.task.due` and `case.review`. Line 771 mentions the seven-working-day refrain clock in prose only.
- Proposed 7.N text (add to the clock list of section 3.5; values are the shipped values of the later phases):
  - `freeze.limit`: hours, shipped 24, `fixed: true` (phase 20)
  - `freeze.committee_report`: hours, shipped 48, `fixed: true` (phase 20)
  - `freeze.second_report`: calendar months, shipped 2, `fixed: true` (phase 20)
  - `refrain.clock`: working days, shipped 7, `fixed: true` (phase 20; API.md 3.14)
  - `report.filing_target`: hours, shipped 24, bounds 1 to 24, `fixed: false` (phase 21 section 3.18)
  - `report.rework`: working days, shipped 10, bounds 5 to 20, `fixed: false` (phase 21 section 3.18)
  "Calendar months" is not a value of `BusinessUnit` (section 2 of this phase: `WorkingDays`, `Hours`, `Days`). Phase 20 keeps the month arithmetic in its own function `add_calendar_months` (phase 20 section 3.14, `domain::ops::clock`). The unit for `freeze.second_report` therefore needs a decision: UNCLEAR (the lead chooses between a fourth unit value and a separate entry kind that calls phase 20's function)."

### G1-09-02 Reason codes named by phases 14 and 15 are not in the phase 09 seed description
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-09.md section 3.4 (reason-code families, the "Seed" bullet, line 741)
- Sources: phase-14.md:1244 "the reason family `decision` gains eight codes `DEC_SCR_*` (name differs, identifier differs, birth date differs, family name only, confirmed designation, potential needs review, insufficient data, other) in `backend/packs/reference/reason-codes.v1.json`"; phase-15.md:1801-1804 "family `override`: `OVR_UPWARD_ADVERSE_INFO`, `OVR_UPWARD_PEP_JUDGEMENT`, `OVR_UPWARD_EXPECTED_ACTIVITY`, `OVR_UPWARD_COMPLIANCE_JUDGEMENT`, `OVR_DOWN_SOURCE_DOCUMENTED`, `OVR_DOWN_STRUCTURE_VERIFIED`; and in family `decision`, `DEC_EXIT_FINANCIAL_CRIME`"
- Check: grep -n "DEC_SCR\|OVR_UPWARD\|OVR_DOWN\|DEC_EXIT" docs/plan/phase-09.md -> none; grep -n "^Seed\|- \*\*Seed:\*\*" docs/plan/phase-09.md -> line 741 says the file "holds the drafted families" without listing codes; backend/ does not exist yet
- Proposed 7.N text:
  "### 7.N Seed rows added by phases 14 and 15 (reason-codes.v1.json, inactive until the family change request)
  Family `decision` gains `DEC_SCR_*` (phase 14: eight codes, the name, identifier, birth date and family-name outcomes, confirmed designation, potential needs review, insufficient data, other) and `DEC_EXIT_FINANCIAL_CRIME` (phase 15, the exit register). Family `override` gains the six `OVR_UPWARD_*` and `OVR_DOWN_*` codes of phase 15 section 4.2. All are inactive at seed; `akr-ctl ref seed` inserts them with `is_active = false`. The exact code list is copied from phase 14 section 4.5 and phase 15 section 4.2 by the lead."
  Note: phase 15 names `OVR_UPWARD_COMPLIANCE_JUDGEMENT` and `OVR_DOWN_*` in the same list; the lead confirms the section numbers of phase 15 (the reason-code list is at phase-15.md line 1801).

### G1-09-03 Stale "no consumer until phase 15" sentences
- Severity: LOW
- Category: Incomplete Coverage
- Target: phase-09.md section 3.1 (outbox event `ref.country_level_changed`, line 615) and section 4 (line 1034, the rating note)
- Sources: phase-15.md:1786-1788 "`ref.country_level_changed` (phase 09; the phase 09 note 'no consumer until phase 15' is closed by this phase)"
- Check: grep -n "no consumer until phase 15" docs/plan/phase-09.md -> lines 615 and 1034
- Proposed 7.N text: "The sentences 'Until phase 15 the event has no consumer' (section 3.1) and 'no consumer until phase 15' (the rating note of section 4) are answered by phase 15: the consumer is registered there (`rating_country_fanout`). Phase 09 keeps the outbox write unchanged."

### G1-09-04 BankCalendarClock::working_days_back is missing
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-09.md section 2 (`BankCalendarClock` trait, the `add` signature at line 457 area) and section 3.2 (the business-day arithmetic)
- Sources: phase-16.md:1832-1833 "**Phase 09:** the backward working-day walk `BankCalendarClock::working_days_back` (section 3.8 step 5); a pure function over the same day set, with its own unit test"; phase-16.md:1238 "at `BankCalendarClock::working_days_back(d, N, calendar)`"
- Check: grep -c "working_days_back" docs/plan/phase-09.md -> 0; phase-09.md:457 defines only `fn add(&self, from, n, unit)`
- Proposed 7.N text:
  "### 7.N Backward working-day walk (phase 16 section 3.8 step 5)
  `BankCalendarClock::working_days_back(d: NaiveDate, n: u32, calendar: CalendarId) -> NaiveDate` walks back `n` working days from the local date `d` over the same day set and weekend pattern as `add` (section 3.2). It is a pure function with its own unit test. It is not a new unit of `BusinessUnit`."

### G1-09-05 D3 gains an "Assessment" tab (phase 29)
- Severity: LOW
- Category: Incomplete Coverage
- Target: phase-09.md section 5 (screen D3, the tab rules at lines 1136-1141)
- Sources: phase-29.md:1371 "### 5.2 D3 Policy studio (`/policy`; phase 09 screen D3; addition: tab \"Assessment\")"; gap-items.md item (y) "D3 tab Assessment"
- Check: grep -n "Assessment" docs/plan/phase-09.md -> none in section 5; section 5 says the tabs follow the rail rules of DESIGN-SYSTEM 8.2 without a tab list
- Proposed 7.N text: "Screen D3 gains the tab `Assessment` (phase 29 section 5.2: panels \"Fail policy\", \"Rail modes\" and \"Prohibited rules\"). The rail rule for the tab is UNCLEAR: phase 29 section 5.2 names no roles, so the lead decides it from DESIGN-SYSTEM 8.2. The tab list of this section is otherwise unchanged."

### G1-09-06 The reference router mounts the segments router (phase 13)
- Severity: LOW
- Category: Missing Endpoint
- Target: phase-09.md section 1 (API routers, the `reference` router at line 8 and the route table of section 3.1 area)
- Sources: phase-13.md:939 "Phase 09: the reference router mounts the `segments` router under `/ref`"; phase-13.md:515 "`/ref` by the phase 09 reference router"
- Check: grep -n "segments" docs/plan/phase-09.md -> line 1312-1313 only (out-of-scope note: "`/ref/segments` is not touched"); the phase 09 router list (line 8) names `reference` without a nested mount
- Proposed 7.N text: "The `reference` router of this phase mounts the `segments` router of phase 13 under `/ref` (a nested mount; no route row of this phase changes). The out-of-scope note 'no segments' stays true for the content of the routes, not for the mount."


## Target phase 10 (mapping and canonical model)

### G1-10-01 canon.product.islamic_contract_type lacks the four values of phase 26
- Severity: MEDIUM
- Category: Missing Fields
- Target: phase-10.md section 1 (`canon.product`, the `islamic_contract_type` CHECK, line 397)
- Sources: phase-26.md:881 "Four values are added to `canon.product.islamic_contract_type` (`tawarruq`, `diminishing_musharaka`, `wakala`, `wadiah`)"; phase-26.md:755 "**Phase 10**: `canon.product` CHECK widened (section 1.1)"; status.md row 26
- Check: grep -c "tawarruq" docs/plan/phase-10.md -> 0; phase-10.md:397-398 CHECK list is murabaha, musharaka, mudaraba, ijara, qard_hasan, istisna, salam, other
- Proposed 7.N text:
  "### 7.N islamic_contract_type values (phase 26 DR-247; phase 26 migration widens the CHECK)
  Phase 26 widens the CHECK of `canon.product.islamic_contract_type` by four values: `tawarruq`, `diminishing_musharaka`, `wakala`, `wadiah`. The base list of this phase is unchanged. The change is made by phase 26's own migration; this text records it so that the phase 10 list is read with the four values in an islamic pack install."
  Note: a text note only (phase 26 issues the ALTER), per the run 1 rule.

### G1-10-02 CanonicalEntity and canonical-model.v1.json gain the two phase 26 entities
- Severity: MEDIUM
- Category: Missing Table
- Target: phase-10.md section 2 (`CanonicalEntity`, line 1578) and section 1 (the file `canonical-model.v1.json`, lines 1544 and 1549)
- Sources: phase-26.md:322 "The `CanonicalEntity` closed enum (phase 10, 29 names) gains `islamic_deal` and `deal_party` at the end of the catalogue order"; phase-26.md:756 "`CanonicalEntity` gains two names; `canonical-model.v1.json` gains two entries; mapping families `islamic_deal` and `deal_party` registered"
- Check: grep -n "islamic_deal\|deal_party" docs/plan/phase-10.md -> none; phase-10.md:1578 "closed enum of the 29 names"; phase-10.md:1544 "the 29 canon entities"
- Proposed 7.N text:
  "### 7.N Two catalogue entities of the optional Islamic pack (phase 26 section 1 and section 3.1)
  `CanonicalEntity` gains `islamic_deal` and `deal_party` at the end of the catalogue order (31 names in all when the pack is installed; the enum is closed in code, so the pack's release adds them). `canonical-model.v1.json` gains the two entries (table, columns, classification) that phase 26 section 1 gives, and the mapping families `islamic_deal` and `deal_party` are registered in the mapping DSL catalogue. A deal party is never a new subject kind (phase 26 section 3.1). The two tables are created by phase 26."
  Note: the phase 26 section numbers are as the 26 file gives them; the lead confirms.

### G1-10-03 Canonical writers read admin.retention_tombstones before a customer or account insert
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-10.md section 3.7 (the `CanonicalWriter` steps, line 1968 area) and section 3 (the writer of `canon.customer` and `canon.account`)
- Sources: phase-23.md:930 "Phase 10 and phase 11 canonical writers (akr-ingestd). Before inserting a `canon.customer` or `canon.account` identity, the writer checks `admin.retention_tombstones` for the identity's HMAC (section 3.5). A hit skips the row and increments `akr_ingest_tombstoned_total`"; phase-23.md:22 (the table `retention_tombstones`, created by phase 23 migration 0072)
- Check: grep -c "tombstone" docs/plan/phase-10.md -> 0; grep -n "retention_tombstones" docs/plan/phase-23.md -> lines 22 and 930
- Proposed 7.N text:
  "### 7.N Tombstone check in the canonical writer (phase 23 section 4.5 item 3)
  Before `CanonicalWriter` inserts a `canon.customer` or `canon.account` identity, it reads `admin.retention_tombstones` for the identity's keyed hash (the HMAC of phase 23 section 3.5). A hit skips the row and increments `akr_ingest_tombstoned_total`. The table is created by phase 23 (migration 0072); the writer's grants are the read grant that phase 23 gives. This is a read-only addition; the writer's transaction and the other statements are unchanged."

### G1-10-04 Batch indexes on canon.txn and canon.txn_event are added by phase 16
- Severity: MEDIUM
- Category: Missing Table
- Target: phase-10.md section 1 (indexes of `canon.txn` at lines 1117-1121 and of `canon.txn_event`)
- Sources: phase-16.md:1834 "**Phase 10:** the two batch indexes of migration 0056 (physical indexes only)"; status.md row 16 "indexes on `canon.txn` and `canon.txn_event` (batch id)"
- Check: grep -n "batch" docs/plan/phase-10.md | grep -i index -> none; the txn index list (lines 1117-1121) has no batch_id index
- Proposed 7.N text: "Phase 16 migration `0056` creates two indexes on `(batch_id)` of `canon.txn` and `canon.txn_event` (physical indexes only, no column or constraint change). The phase 10 index list is unchanged; the indexes are owned by phase 16." (Lead confirms the index names from phase 16 migration 0056 before applying.)

### G1-10-05 MappingExecutor::map_payload is added by phase 11
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-10.md section 2 (ports and the `MappingExecutor`, around line 1700 area) and section 3.5 (mapping execution)
- Sources: phase-11.md:1384 "`MappingExecutor` gains `map_payload` (the pure transforms applied to a stored payload, used by replays and retries)"
- Check: grep -n "map_payload" docs/plan/phase-10.md -> none; grep -n "MappingExecutor::map" docs/plan/phase-10.md -> the method `map(version, rows)` only (phase-11.md:793 cites it)
- Proposed 7.N text: "`MappingExecutor` gains `map_payload(version, payload)`: the pure transforms of section 3.2 applied to one stored payload (a raw landing row), with the same `MappedBatch` result as `map`. It has no IO. Phase 11 uses it for replays and retries. `map` is unchanged." (The lead confirms the signature against phase 11.)

### G1-10-06 Mapping change target reads volume_effect from the replay
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-10.md section 3.9 or section 2 (the `mapping` change target and its `volume_effect` rule, the table row at line 2031)
- Sources: phase-11.md:1385 "The `mapping` change target reads `volume_effect` from the completed replay of the proposed version (DR-92)"; phase-11.md:1092 "replay for that version, instead of its own rule, when a replay exists (DR-92; the first publication keeps"
- Check: phase-10.md:2031 gives the rule `neutral` for the first publication and `unknown` otherwise, computed from the impact; no mention of the replay (grep -n "replay" docs/plan/phase-10.md near 2031 -> none)
- Proposed 7.N text: "The `volume_effect` of a `mapping` change request is read from the completed replay of the proposed version (phase 11 `mapping_replays`, DR-92) when a replay exists; the first publication keeps `neutral`; without a replay the rule above (`unknown`) applies. No other part of the change target changes."

### G1-10-07 Address keys used by phase 21 are not defined
- Severity: MEDIUM
- Category: Missing Fields
- Target: phase-10.md section 1 (`canon.party_contact`, the `address` JSONB column, line 550)
- Sources: phase-21.md:799 "the current `canon.party_contact` row of type `address` gives `city` and `district`. `governorate` must be a code of `ref.governorates` (`IQ-XX`, phase 09)"
- Check: grep -n -i "city\|district\|governorate" docs/plan/phase-10.md -> none; phase-10.md:550 `address JSONB` has only the object check
- Proposed 7.N text: "The `address` object of `canon.party_contact` (type `address`) carries the keys `city`, `district` and `governorate` (the last a code of `ref.governorates`, phase 09). The keys are the canonical names for phase 21 and phase 15 reads; a source's mapping puts them there. No constraint changes; the key names are documented in the table comment and in `canonical-model.v1.json`."
  The lead checks whether phase 10 mapping DSL already has a field for these; if not, a key list belongs in `canonical-model.v1.json`.

### G1-10-08 canon.party_name folded and key are computed by a phase 10 copy of the folding rules, not by the names crate
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-10.md section 1.2 (`canon.party_name`, columns `folded` and `key`, line 497-498) and section 3.3 (`FoldArabic` in the transform library, line 1858)
- Sources: phase-14.md:662 "`crates/names` is the only place where the normalisation rules live. Every other crate calls it. The SQL twin ... is generated by hand from the same tables and proven equal by the parity corpus"; phase-14.md:23 "The key function is the SQL twin of the Rust `names` crate"; phase-10.md:1858 "`FoldArabic` is the folding of research file 22, shared with phase 14 matching"
- Check: grep -n "crates/names\|names crate" docs/plan/phase-10.md -> none; grep -n "FoldArabic" docs/plan/phase-10.md -> line 1858 (a phase 10 transform); phase-10.md:498 `key TEXT NOT NULL` with no function named
- Proposed 7.N text:
  "### 7.N Name keys come from the names crate (phase 14 section 3.1)
  The columns `folded` and `key` of `canon.party_name` are produced by the `names` crate of phase 14 (`NameNormaliser`, the fold and key tables, and the SQL twin of its key function). The transform `FoldArabic` of section 3.3 is removed from this phase; a `fold` transform, if the mapping DSL needs one, calls the names crate. The `script` and `language` columns are unchanged.
  UNCLEAR (sequence): phase 10 runs before phase 14, so the writer of this phase cannot call `crates/names` until phase 14 lands. The lead decides between a stub that is replaced by phase 14 and the reordering of the two crate deliveries."


## Summary

Findings by severity (35 in all):
- HIGH: 6 (G1-05-02, G1-05-03, G1-06-01, G1-07-01, G1-07-02, G1-09-01)
- MEDIUM: 22
- LOW: 7 (G1-05-09, G1-06-04, G1-06-08, G1-08-01, G1-09-03, G1-09-05, G1-09-06)
- By target: phase 05 9, phase 06 8, phase 07 2, phase 08 2, phase 09 6, phase 10 8.

Obligations already present in the target (skipped, not written as findings): about 70. Counted by hand per target, so the count is approximate: phase 05 about 14 (for example the SEC scopes on `GET /sources`, `discovery.read`, `PATCH /sources/{id}`, the `candidate` and `liveness` columns, `IntegrationProfileBuilder` in section 3.8, the `akr_source` state payload, the CT3 note); phase 06 about 10 (the `readiness.read` M, CH, IA rows, the observation flag, the `pack.download` step-up of 7.1); phase 07 about 7 (the profile clocks, the `RejectionParser` stub, the goAML readiness check, `CONS-04`, `PKG-01` to `PKG-06`); phase 08 about 10 (the layout patterns, `StepUpDialog`, `FileDrop`, `DiffView`, `E1p`, `E2`); phase 09 about 14 (the `qa.review`, `case.decision`, `freeze.decision`, `rfi.release` and `ops.verify` decision rows, the `rfi.response` clock, `reason_text`, `task` scope, `Hours` unit); phase 10 about 16 (the `kyc_document`, `canon.staff` and `customer.rm_id` columns, `superseded_at`, `name_type`, `parts`, `placeholder_flag`, `party_contact`, `sensitivity_tier`, `booking_date`, the link tables, `canon.written`).

Gap-items cross-check: items (g), (p), (q), (r), (t), (v), (z) and (aa) that name phases 05 to 10 map to G1-10-08, G1-09-01, G1-07-01 and G1-07-02, G1-10-03, G1-06-06 and G1-06-07, G1-10-01 and G1-10-02, G1-05-07 and G1-06-08. Item (a) was already done and is skipped. Item (d) is phase 04 and out of scope.

Scope and limits of this run: the grep sweep covered section 4 (including 4.5) of phases 06 to 31, the lines of phases 06 to 31 that name phases 05 to 10 with a verb, the status.md table column for those phases, and the DR rows that carry "gap pass 1". The prose of sections 1 to 3 of phases 11 to 31 was checked only through those greps, not read in full. Names were verified by grep in the target; no DDL was executed.

UNCLEAR (the lead decides before applying):
1. G1-05-04: phase 05 never lists the variants of `StatementKind`; phase 11 uses `Read` and phase 12 adds `ControlTotals`.
2. Phase 05 internal inconsistency (not a run 3a finding): step 9 at phase-05.md:811 enqueues `discovery_seal_report {check_id}`; the job table at phase-05.md:1334 gives payload `{run_id}`.
3. G1-06-01: the key form of `AttributeCatalogue` (`<table>.<column>` in phase 05 versus `rating.level` in phase 15), and whether phase 10's `AttributeService` (phase-10.md:2173) is the same type.
4. G1-07-02: NAME-01 is listed by phase 21 (line 1258) but defined nowhere; CTR-01 and CTR-02 have no severity; phase 21 says the CTR codes enter the catalogue by change request (line 948) while the same catalogue is closed in code (line 1258).
5. G1-08-01: the values of `RailGroup` (phase 10 needs a `data` group that phase 08 does not name).
6. G1-09-01: `freeze.second_report` is in calendar months, which is not a `BusinessUnit` value; the lead chooses a unit or a separate kind.
7. G1-10-08: phase 10 runs before phase 14, so the `names` crate cannot be called by the phase 10 writer until phase 14 lands; sequencing decision.
8. Phase 07 line 967 says a new candidate notice comes from phase 22; phase 22 item 14 says it defines no topic for it. Owner not settled.
9. Phase 22 item 9 cites "phase 06 section 3" for `PUT /admin/flags/branch.observations_live`; the flag route is phase 03 (phase-03.md:515). Citation fix only.
10. Phase 29 release control assumes that a row in `canon.payment_message` means a release (phase-29.md:1517, an open bank confirmation); phase 10 gives `status` as free text with no value list, so the status value that means "released" is not defined anywhere. Phase 29 also refers to a `released` payment status it does not define in phase 10.
11. Phase 21 item 6 calls `CONS-04` new; phase 07 already has it at line 611 (wording fix in phase 21).
