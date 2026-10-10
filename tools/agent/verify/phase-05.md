# Verification pass, phase 05 (Haiku draft for lead review)

Sample (12 items). Header "Implemented here" has no adversarial blocks in this phase, so 3 extra header items are used in place of adversarial items.
- Header 1: V-05-01 TS-05 generated SQL proof (start of list)
- Header 2: V-05-02 VD-02 mapping from bank dictionary (middle)
- Header 3: V-05-03 VD-11 adapter pack (later)
- Header 4: V-05-04 VC-85 no write to CBS (end)
- Header 5: V-05-05 C-13 signed load budget (Requirements carried)
- 7.x 1: V-05-06 7.2 discovery_seal_report job
- 7.x 2: V-05-07 7.5 StatementKind and governor entry points
- 7.x 3: V-05-08 7.8 listener reload on reconnect
- 7.x 4: V-05-09 7.9 resume of a halted source
- Extra header 6: V-05-10 C-03 read-only attestation on connect
- Extra header 7: V-05-11 C-15 paused by bank state
- Extra header 8: V-05-12 C-07 integration profile export

### V-05-06 discovery_seal_report payload and enqueues   (source: 7.2; result: GAPS 2)
- T1 SQL: PASS - the evidence_object_id and report_object_id columns exist (phase-05.md:111 and phase-05.md:276, migrations 0017 and 0018); no new table.
- T2 Route: N/A - a job with no route.
- T3 Repository and types: GAP - 7.2 says the check is "stored as an evidence object owned by the source", but no evidence method is named for the check seal (phase-05.md:1511-1513). Phase 05 names no method, and the 3.6 text (phase-05.md:1140) names `EvidenceStore::stage`.
- T4 Logic: GAP - the job's own numbered steps are missing for the check branch. Step 9 of 3.3 (phase-05.md:814-816) describes the render and the signing in prose, with no step for the skip rule or the write of evidence_object_id. The 7.2 skip rule (phase-05.md:1514) is stated once, so this may be enough at MEDIUM.
- T5 Audit: PASS - `discovery_run.report_sealed` (phase-05.md:1262) covers runs; `evidence.sealed` is reused from phase 02 (phase-05.md:1264) and covers checks.
- T6 Cross-references: PASS - `coverage_grade` with reason `profile_run` in phase-06.md:587 (job) and phase-06.md:237 (profile_run_ids); `drift_scan` payload `{source_id, run_id}` at phase-10.md:1919 and phase-10.md:2163 (coalesce key `drift_scan:<source_id>`); `akr-sign-certificate` at phase-00.md:918.
- T7 Consistency: GAP (see below) - section 3.6 (base) says `EvidenceStore::stage`, and phase 02 limits `stage` to akr-api uploads (phase-02.md:839). Phase 02 gives the worker `put_generated` (phase-02.md:876). 7.2 does not resolve this.
- Gaps:
  - GAP V-05-06-a | MEDIUM | T3 | Check seal: no evidence method named for storing the signed check report (7.2 says only "stores it as an evidence object") | 7.2 (phase-05.md:1510-1518) and 3.3 step 9 (phase-05.md:814-816) | greps: `grep -n "put_generated\|EvidenceStore" docs/plan/phase-05.md` found only the 3.6 `stage` reference (phase-05.md:1140)
  - GAP V-05-06-b | MEDIUM | T6 and T7 | The base 3.6 text names `EvidenceStore::stage` for a worker job; phase 02 defines `stage` for akr-api uploads only and `put_generated` for akr-worker (phase-02.md:839 and :876). 7.2 does not override it | phase-05.md:1140, section 3.6 (fix in 7.2 or 3.6) | greps: `grep -n "stage(\|put_generated" docs/plan/phase-02.md`; phase-05 has no 7.x line naming the override
  - GAP V-05-06-c | MEDIUM | T4 | Seal job: the numbered steps for the check branch (skip when `evidence_object_id` is set, render, sign, store, write the id) are prose in 3.3 step 9 and 7.2 | section 3.3 step 9 or a numbered list under 7.2 (phase-05.md:814-816, 1514) | greps: `grep -n "evidence_object_id" docs/plan/phase-05.md` found the column DDL at line 111 and no step

### V-05-07 StatementKind and governor entry points   (source: 7.5; result: PASS)
- T1 SQL: N/A - no table in this block.
- T2 Route: N/A - no route in this block.
- T3 Repository and types: PASS - `StatementKind` variants are listed (phase-05.md:1538-1545); `LoadGovernor::acquire(&self, kind: StatementKind)` is in the base (phase-05.md:944); `LoadGovernor::for_source(source_id)` is named (phase-05.md:1546). The return type of `for_source` is not written (cosmetic).
- T4 Logic: PASS - the token rules point at the numbered step 4 of `acquire` (phase-05.md:949-951), which states one token per fetched row and one token for an aggregate row.
- T5 Audit: N/A - no mutation.
- T6 Cross-references: PASS - `SourceReader::fetch_keyset`, `branch_dates` and `as_of` exist in phase 11 (phase-11.md:640-643); `LoadGovernor::for_source` is called at phase-11.md:753; `StatementKind::ControlTotals` is in phase 12 (phase-12.md:704 and :1137); DR-365 is in research.md:376.
- T7 Consistency: PASS - base 3.5 (acquire takes `StatementKind`) and 7.5 agree; 7.5 does not contradict 7.6.
- Gaps: none. (Note only: phase-11.md:503 writes `bank_dates` in the layout tree while the trait at phase-11.md:642 says `branch_dates`; cosmetic, not a gap.)

### V-05-08 Listener reload on reconnect   (source: 7.8; result: GAPS 1)
- T1 SQL: N/A - no table in this block.
- T2 Route: N/A - no route.
- T3 Repository and types: PASS - the full read is covered by `SourceRepository::list` (phase-05.md:513), `LoadBudgetRepository::in_force` (phase-05.md:541) and the access rows of `SourceRepository::get` (phase-05.md:511). The base text names the governor rebuild (phase-05.md:940-942).
- T4 Logic: GAP - 7.8 gives the rule in prose ("rebuilds the LoadGovernor of every source from one full read ... before it waits again") with no numbered steps for the reload, the order of re-reads or what happens to a permit in use.
- T5 Audit: N/A - the reload writes no row (the reconnect is logged by `akr_source_state` metrics only, phase-05.md:1375).
- T6 Cross-references: PASS - phase 30 names the same reload as item 2 (phase-30.md:481) and DR-322 is in research.md:333.
- T7 Consistency: PASS - the hourly re-read in 7.8 (phase-05.md:1565) matches base 3.5 (phase-05.md:941-942).
- Gaps:
  - GAP V-05-08-a | MEDIUM | T4 | Reload steps: no numbered steps for the full read, the rebuild of each `LoadGovernor` and the handling of permits in use during a rebuild | 7.8 (phase-05.md:1562-1565), or a numbered list under section 3.5 | greps: `grep -n -i "reload\|rebuild" docs/plan/phase-05.md` found only 7.8 and the base sentence at phase-05.md:941

### V-05-09 Resume of a halted source   (source: 7.9; result: PASS)
- T1 SQL: PASS - the `halted` value of `admin.source_state` exists (phase-05.md:56); the `trigger` value `resume` of `admin.source_grant_checks` exists (phase-05.md:99).
- T2 Route: PASS - `POST /sources/{id}/resume` is in the phase 05 route table with scopes and action (phase-05.md:663, `source.resume`); phase 11 extends it (phase-11.md:706) with scopes `data.steward, security.ops` and step-up `backfill`.
- T3 Repository and types: PASS - `ResumeBody` is defined (phase-11.md:683); the grant check call `ConnectionGuard::connect(source, Trigger::Resume)` is in phase-11.md:947-948 and in base 3.3.
- T4 Logic: PASS - the steps are in phase 11 (phase-11.md:937-952): state check, decision by reason, fresh grant check, and refusal ends the job.
- T5 Audit: PASS - `source.resume_requested` (phase-05.md:1250) and `source.state_changed` for every transition (phase-05.md:1253) cover the route and the `halted -> refused` move (phase-11.md:1379).
- T6 Cross-references: PASS - `ingest_resume` job row (phase-11.md:1301); step-up class `backfill` is in the closed list (phase-01.md:710); `ResumeBody` is listed at phase-11.md:539.
- T7 Consistency: PASS - base 3.1 (phase-05.md:698-702) leaves `halted` to phase 11, which matches 7.9. The paused path stays with the base text (phase-11.md:938-940 says so).
- Gaps: none.

### V-05-01 Generated SQL proved as one read-only SELECT (TS-05)   (source: header Implemented here, first item; result: PASS)
- T1 SQL: N/A - the proof writes no table; the approved-object list it checks is `approved_read_objects` (phase-05.md:75, PASS).
- T2 Route: N/A - no route; the proof runs inside the connect and the job path.
- T3 Repository and types: PASS - `SelectProof::prove(sql, engine, scope)` (phase-05.md:624), `ProofScope::{Dictionary, Data}` (phase-05.md:626-629), `ProvenSelect` (phase-05.md:631), `SourceSession::query` accepts only `ProvenSelect` (phase-05.md:602).
- T4 Logic: PASS - rules 1 to 8 are numbered (phase-05.md:889-909), the Oracle pre-pass is stated (phase-05.md:912), and a refusal fails the object with `akr_select_proof_refusals_total` (phase-05.md:916).
- T5 Audit: N/A - no mutation; a refusal is recorded by the job's `incomplete` state.
- T6 Cross-references: PASS - the generator that emits `Data` statements is phase 10 (phase-05.md:884-886 says "from the mapping compiler in phase 10"); phase-10.md:1714 says the compiler calls `prove()`; phase-10.md:1554 moves the type to `crates/sqlproof` (7.6). Three names checked.
- T7 Consistency: PASS - 7.5 and 7.6 do not change rules 1 to 8.
- Gaps: none.

### V-05-02 Mapping derived from the bank's own dictionary, comments and samples (VD-02)   (source: header Implemented here, second item; result: GAPS 1)
- T1 SQL: N/A - no mapping table in this phase; the catalogue read is in phase 05 (dictionary columns, phase-05.md:1045).
- T2 Route: N/A - no route for mapping derivation in this phase (the phase 05 routes are source, discovery and read routes, phase-05.md:655-684).
- T3 Repository and types: GAP - none of the mapping-derivation types or methods are named in phase 05; the only comment reads are `ALL_TAB_COMMENTS` and `ALL_COL_COMMENTS` in the catalogue (phase-05.md:1045).
- T4 Logic: GAP - no numbered steps for deriving a mapping from DDL, comments and samples and no change-control step in phase 05. Mapping is phase 10 (phase-10.md:13 depends on phase 05; phase-10.md:1714 names the compiler).
- T5 Audit: N/A - nothing in phase 05 creates a mapping.
- T6 Cross-references: PASS - phase 05 reads the comments that phase 10 uses (phase-05.md:1045 and the phase 10 dependency line). Name check only.
- T7 Consistency: GAP - the header says "implemented here" (phase-05.md:36) while section 3 places mapping in phase 10 (phase-05.md:884-886). The 7.x blocks do not resolve it.
- Gaps:
  - GAP V-05-02-a | MEDIUM | T4 | VD-02 is listed as implemented in phase 05, but the mapping-derivation steps and the change-control step are not written here; the owning step is in phase 10 | header "Implemented here" (phase-05.md:36) or a pointer "left to phase 10" | greps: `grep -n -i "mapping" docs/plan/phase-05.md` found only 7.4, 7.7 and the base references at 884-886; `grep -n "VD-02" docs/plan/phase-*.md` found no other phase file

### V-05-03 PostgreSQL, SQL Server, Oracle and ODBC adapters (VD-11)   (source: header Implemented here, third item; result: PASS)
- T1 SQL: PASS - engine CHECK `('postgres', 'mssql', 'oracle', 'db2', 'sybase')` on `admin.sources.engine` (phase-05.md:63).
- T2 Route: N/A - adapters are not routes.
- T3 Repository and types: PASS - `SourceReader` trait (phase-05.md:585-605: `engine()`, `grant_policy()`, `open`, `identify`, `begin_read_only`, `query`, `cancel_handle`, `end`); feature gates `pg`, `mssql`, `oracle`, `odbc` (phase-05.md:585-587).
- T4 Logic: PASS - the per-engine read-only session table (phase-05.md:919-927, section 3.4) and the grant policy per engine (phase-05.md:843-850).
- T5 Audit: N/A - no mutation.
- T6 Cross-references: PASS - phase 05 `GrantPolicy` at phase-05.md:612; phase-05.md:1361 (`readers: AdapterRegistry`) matches the header item; the packs are phase 00 signed artifacts (C-38, not checked here).
- T7 Consistency: PASS - base 3.4 and 3.3 agree on the per-engine policies.
- Gaps: none.

### V-05-04 AKR-TMS never writes to the CBS (VC-85)   (source: header Implemented here, fourth item; result: PASS)
- T1 SQL: N/A - no table.
- T2 Route: N/A - no route.
- T3 Repository and types: PASS - `GrantPolicy::judge` and `PrivilegeSnapshot` (phase-05.md:612), the statement proof (phase-05.md:624-631).
- T4 Logic: PASS - the grant check refuses any write privilege (`write_privilege`, phase-05.md:869-870) and any `INSERT`, `UPDATE`, `DELETE` object privilege in the table at phase-05.md:843-850; the proof refuses every non-SELECT statement (rule 1 and 2, phase-05.md:889-893) and the session control text is listed (phase-05.md:593).
- T5 Audit: N/A - no mutation on the CBS.
- T6 Cross-references: PASS - tasks, approvals and evidence stay in AKR-TMS tables (phase-05.md:1260 `evidence.sealed` is phase 02; the sealed report is in `admin.evidence_objects`, phase-05.md:111).
- T7 Consistency: PASS - 3.4 read-only session table (phase-05.md:923) and 3.3 table agree.
- Gaps: none.

### V-05-05 Signed load budget with kill switch and windows (C-13)   (source: header Requirements carried, Requirements list middle; result: PASS)
- T1 SQL: PASS - `admin.load_budgets` has `max_sessions`, `statement_timeout_ms`, `rows_per_fetch`, `rows_per_minute`, `windows`, `blackout`, `lag_ceiling_s`, `slowdown`, `bank_signatory`, `signed_document_id` (phase-05.md:138-160), with the CHECK that a signed budget has a document and a change request (phase-05.md:159-160).
- T2 Route: PASS - `GET` and `PUT /sources/{id}/load-budget` with scope `data.steward` and the `load_budget` CR (phase-05.md:659-660).
- T3 Repository and types: PASS - `LoadBudgetRepository::insert`, `in_force`, `history` (phase-05.md:540-542); `BudgetSpec::validate` (phase-05.md:1223, 3.9); `LoadGovernor::acquire` (phase-05.md:944).
- T4 Logic: PASS - governor gates 1 to 5 are numbered (phase-05.md:945-960); the change target steps are in the table row (phase-05.md:1223).
- T5 Audit: PASS - `load_budget.activated` (phase-05.md:1260) and the CR events reused from phase 04.
- T6 Cross-references: PASS - phase 04 `ChangeTarget` trait at phase-04.md:642 and the registry at phase-04.md:123; `SourceRepository::set_access` at phase-05.md:519 is the other target.
- T7 Consistency: PASS - 7.5 does not change the governor rules.
- Gaps: none.

### V-05-10 Read-only attestation on every connect (C-03)   (source: header Requirements carried, extra item; result: PASS)
- T1 SQL: PASS - `admin.source_grant_checks` with `trigger`, `status`, `result`, `snapshot_sha256`, `evidence_object_id` (phase-05.md:96-122).
- T2 Route: PASS - `GET /sources/{id}/grants` (phase-05.md:658) with `source.read_grants` (DS).
- T3 Repository and types: PASS - `GrantCheckRepository::insert_pending`, `complete`, `fail`, `confirm_latest`, `latest`, `attach_evidence` (phase-05.md:527-535).
- T4 Logic: PASS - `ConnectionGuard::connect` steps 1 to 10 (phase-05.md:800-818).
- T5 Audit: PASS - `source.grant_checked` (phase-05.md:1251) and `source.refused` (phase-05.md:1252); the seal writes `evidence.sealed` (phase 02).
- T6 Cross-references: PASS - `discovery_seal_report` is in 7.2; `akr-sign-certificate` at phase-00.md:918.
- T7 Consistency: PASS - 7.2 agrees with 3.3 step 9.
- Gaps: none.

### V-05-11 Paused by bank with bounded back-off (C-15)   (source: header Requirements carried, extra item; result: GAPS 1)
- T1 SQL: PASS - `admin.source_pauses` with the cause enum, and the open-pause unique index (phase-05.md:170-185).
- T2 Route: PASS - `POST /sources/{id}/pause` (phase-05.md:662) and `POST /sources/{id}/resume` (phase-05.md:663).
- T3 Repository and types: PASS - `PauseRepository::open`, `close`, `list` (phase-05.md:547-551).
- T4 Logic: PASS - the state move `active -> paused_by_bank` (phase-05.md:698-702); the back-off at phase-05.md:834 and 992-994; the setting `sources.bank_pause_backoff_s` (phase-05.md:431).
- T5 Audit: PASS - `source.paused` (phase-05.md:1254) and `source.sessions_closed` (phase-05.md:1255).
- T6 Cross-references: GAP - the header and 3.x text say phase 11 backfill and phase 12 coverage gap read the pause record (phase-05.md:44-45 and 168-169). Phase 11 never names `admin.source_pauses`; it uses its own `admin.blind_periods` (phase-11.md:76-82, with cause `source_paused`). Phase 12 names no pause table (phase-12.md:83 uses an error code only).
- T7 Consistency: GAP - phase 11 line 5 says it uses "source pauses" through phase 05, while phase 11 writes its own gap rows. Two records of one pause are not linked.
- Gaps:
  - GAP V-05-11-a | MEDIUM | T6 | The coverage gap (phase 11 `blind_periods`, phase 12) and the backfill (phase 11 `ingest_backfill`) are not built from `admin.source_pauses`, though phase 05 says they are (phase-05.md:44-45, 168-169). Nothing links a pause row to a blind period row | phase 11 section 1 or 3 (a pause -> blind period rule with the source_pauses id) | greps: `grep -n "source_pauses" docs/plan/phase-11.md docs/plan/phase-12.md` found nothing; `grep -n "blind" docs/plan/phase-05.md` found nothing

### V-05-12 Integration profile export (C-07)   (source: header Requirements carried, extra item; result: PASS)
- T1 SQL: N/A - the export reads stored rows; the tables are in section 1 (`discovery_runs`, `discovery_code_entries`, PASS at phase-05.md:250-300 range).
- T2 Route: PASS - `GET /discovery/integration-profile` with scope `data.steward` and action `discovery.export` (phase-05.md:673 in the discovery route table).
- T3 Repository and types: PASS - `IntegrationProfileBuilder` (section 3.8, phase-05.md:1177); `ObjectProfileView`, `FieldAvailabilityView`, `CodeEntryView` and `IntegrationProfile` named (phase-05.md:645).
- T4 Logic: PASS - sections 1 to N of the profile are listed (phase-05.md:1181-1186); the rendering is phase 06 (phase-05.md:1210).
- T5 Audit: N/A - the export is a read; no mutation.
- T6 Cross-references: PASS - phase 06 names the integration profile as its input (phase-06.md:9 "the integration profile PDF promised by phase 05"); phase 05 says the PDF is phase 06 (phase-05.md:1210). The two sentences disagree on who promised the PDF, which is LOW.
- T7 Consistency: PASS - the route table row and 3.8 agree on `?download=true`.
- Gaps: none. (LOW note: phase-06.md:9 says phase 05 "promised" the PDF; phase-05.md:1210 says phase 06 owns it. Wording only.)

## Summary

- Items checked: 12 (5 header "Implemented here" or "Requirements carried" items spread across the list, 4 items from 7.x blocks, 3 extra header items because phase 05 has no "Adversarial pass" block).
- Items fully PASS: 8 (V-05-01, V-05-03, V-05-04, V-05-05, V-05-07, V-05-09, V-05-10, V-05-12).
- Items with gaps: 4 (V-05-02, V-05-06, V-05-08, V-05-11).
- GAP lines by severity: HIGH 0, MEDIUM 6, LOW 0.

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-05-06-a | MEDIUM | T3 | No evidence method named for storing the signed check report | 7.2 (phase-05.md:1510-1518) and 3.3 step 9 |
| V-05-06-b | MEDIUM | T6/T7 | Base 3.6 names `EvidenceStore::stage` for a worker job; phase 02 limits `stage` to akr-api and gives the worker `put_generated` | section 3.6 (phase-05.md:1140), fix in 3.6 or a 7.x override |
| V-05-06-c | MEDIUM | T4 | Seal job check-branch steps are prose | section 3.3 step 9 (phase-05.md:814-816) or under 7.2 |
| V-05-08-a | MEDIUM | T4 | Listener reload has no numbered steps (read order, rebuild, permits in use) | 7.8 (phase-05.md:1562-1565) or under section 3.5 |
| V-05-02-a | MEDIUM | T4/T7 | VD-02 is claimed as implemented here, but mapping derivation and change control are not written in phase 05 (phase 10 owns the mapping) | header "Implemented here" (phase-05.md:36) or a pointer to phase 10 |
| V-05-11-a | MEDIUM | T6/T7 | Coverage gap and backfill are not built from `admin.source_pauses` as phase 05 says; phase 11 uses `admin.blind_periods` and phase 12 names no pause table; no link between a pause row and a blind period row | phase 11 (section 1 or 3) with a pause-to-blind-period rule |

Notes for the lead: all GAP lines are MEDIUM, so none blocks a build of its item on its own. V-05-11-a is the most consequential. V-05-06-b is a wrong method name in base text, which a developer would take literally. Two LOW wording points were not logged as gaps: the PDF ownership sentence in phase-06.md:9 vs phase-05.md:1210, and the `bank_dates` label at phase-11.md:503 vs `branch_dates` at phase-11.md:642.
