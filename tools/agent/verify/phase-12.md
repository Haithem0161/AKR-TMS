# Verification pass, phase 12 (Haiku draft for lead review)

Sample (12 items, chosen before checking; file docs/plan/phase-12.md, 1397 lines):
- Header "Requirements carried" (5, start, middle and end of the list): C-25 (reconciliation ladder), C-28 (data-quality
  register and known adjustments), L-02 (exclusion register), L-05 (quarantine queue), VA-16 (coverage ledger by product,
  channel and transaction type, with exclusions; header "Gap pass 1 (coverage), implemented here").
- Gap-pass blocks (4, one from each of 7.1 to 7.4, none headed "Adversarial pass"): 7.1 Cedar form, 7.2 reconciliation
  runs after a restore, 7.3 backfill routes removed, 7.4 deferred rungs.
- Adversarial run 3 blocks (3, from 7.5 and 7.6 under the header "Adversarial pass (2026-10-10), run 3", phase-12.md:1327):
  7.5 activate writes accepted_delta, 7.5 step 7 confirmed_gap rule, 7.6 blind period covers the as-of date.

### V-12-01 C-25 reconciliation ladder   (source: header "Requirements carried", C-25; result: GAPS 2)
- T1 SQL: PASS - recon_runs (phase-12.md:72-89) and recon_results (phase-12.md:105-120) hold status, outcome, expected, actual, delta, tolerance, evaluated_at; grants at phase-12.md:131-140. admin.batches has business_from, business_to and control_totals (phase-11.md:136-158). canon.txn has dr_cr, amount, booking_date, ledger_code, status (phase-10.md:951-975). Settings at phase-12.md:394.
- T2 Route: PASS - POST /recon/runs (phase-12.md:589, data.steward, Cedar recon.run) queues the ladder; GET /recon/runs/{id} (phase-12.md:591) returns the rung results.
- T3 Repository and types: PASS - ReconRepository (phase-12.md:487-503) names insert_run, claim, set_waiting, mark_totals_captured, insert_expected, insert_receipts, evaluate, complete, fail, expected_for, pending_days, stale_waiting with parameters. DTOs NewReconRun, ExpectedResult, ResultEvaluation at phase-12.md:477-480. ReconService and RungEvaluator have no method signatures (see Summary, observation 1).
- T4 Logic: PASS - POST steps 1-3 with 409 reasons source_not_active, batch_not_closed, no_batches, run_active (phase-12.md:636-642); sweep steps (644-649); recon_run steps 1-10 with a conditional claim (655) and one transaction for evaluate and complete (682). The step 7 lookup key is filed as V-12-11-a.
- T5 Audit: GAP (V-12-01-a, V-12-01-b). recon.requested, recon.swept, recon.totals_captured, recon.completed and recon.failed are named (phase-12.md:970-973). Step 8 and the status steps name no event.
- T6 Cross-references: PASS - batch_status landed, mapped, partial, rejected (phase-11.md:73); admin.batches (phase-11.md:136); canon.txn (phase-10.md:951); BankCalendarClock as the bank calendar (phase-09.md:8); SelectProof (phase-05.md:472) and LoadGovernor (phase-05.md:944); DR-39 and DR-40 (research.md:50-51); E_NOT_ALLOWED_IN_STATE (API.md:982); E_OBJECT_NOT_APPROVED (API.md:1001).
- T7 Consistency: PASS - rungs L2, L4, L5 and L6 are deferred in 3.1 (phase-12.md:619-621) and 7.4 closes the open doubt (phase-12.md:1323-1325). 7.5 says which step-7 rule it replaces (phase-12.md:1331-1333). The step 7 key conflict is V-12-11-a.
- GAP V-12-01-a | MEDIUM | T5 | Step 8 upserts recon_break rows in dq_issues with no audit event; the only register event, dq.recorded, is defined per batch and a day run has no batch | section 3.1 step 8 (phase-12.md:679) and the audit table (phase-12.md:966-982) | greps: "dq.recorded" gives 747 and 974 only, both per batch; "recon_break" gives 178, 677 and 679 only
- GAP V-12-01-b | LOW | T5 | Claim to running (step 1) and set_waiting to awaiting_totals (step 3) change recon_runs.status and name no audit event; the run events are requested, totals_captured, completed and failed | section 3.1 steps 1 and 3 (phase-12.md:655, 660-661) and the audit table | greps: "awaiting_totals" and "running" in the audit table (none); "recon.started" (none)

### V-12-02 C-28 data-quality register and known adjustments   (source: header "Requirements carried", C-28; result: GAPS 2)
- T1 SQL: PASS - dq_issues (phase-12.md:171-205) has owner_id, status (open, resolved, accepted), severity, due_at, resolution, resolution_note (at most 2000), resolved_by, resolved_at, accepted_until, signed_document_id, change_request_id, detail, version; uq_dq_issues_open at phase-12.md:207. Grants for insert, assign, resolve and accept at phase-12.md:220-223. The foreign keys point to admin.evidence_objects (phase-02.md:353) and admin.change_requests (phase-04).
- T2 Route: PASS - GET /dq/issues (phase-12.md:599), POST /dq/issues/{id}/assign (600), POST /dq/issues/{id}/resolve (601), GET /dq/known-adjustments (602) and POST /dq/known-adjustments (603), with scope data.steward and the Cedar actions dq_issue.* and known_adjustment.* (3.12 table). API.md:332-336 matches.
- T3 Repository and types: PASS - DqRepository (phase-12.md:522-533) names upsert_increments, assign, resolve, accept, reopen_expired, list, get and accepted_for with parameters; DTOs DqIncrement, DqAssign, DqResolve, AcceptInput at phase-12.md:477-480; views and inputs at phase-12.md:577-580. The phase 04 ChangeTarget methods are at phase-04.md:642-655.
- T4 Logic: PASS - assign steps 1-3 with 404, 409 not_open and 422 owner_not_active (phase-12.md:757-759); resolve steps (762-767); expiry steps 1-3 (772-777); propose steps 1-4 with 422 kind_not_adjustable, review_too_far, review_in_past and document_required (785-790). The version is in the WHERE of transition and assign (phase-12.md:543). The "signed by the bank" test is missing: V-12-02-a.
- T5 Audit: PASS - dq_issue.assigned (phase-12.md:759, 975), dq_issue.resolved (767, 975), dq_issue.adjustment_expired (774, 976), known_adjustment.activated (977). The change request events are reused from phase 04 (phase-12.md:984).
- T6 Cross-references: PASS - ChangeTarget and ChangeTargetRegistry (phase-04.md:123, 642-655); E_STALE_BASE 409 (API.md:980); DR-58 (research.md:69); POST /sources/{id}/documents (phase-05.md:664). Naming defect: V-12-02-b.
- T7 Consistency: PASS - no 7.x block changes the register itself; the activation change in 7.5 is covered under V-12-10.
- GAP V-12-02-a | MEDIUM | T4 | "Signed by the bank" has no test: the document kind or flag that proves the signature is not named; phase 05 defines SourceDocumentKind values budget_signed, access_approval and exception_signed | section 3.5 step 3 (phase-12.md:789) | greps: "signed by" gives 789 only; "exception_signed" and "budget_signed" do not occur in phase-12.md
- GAP V-12-02-b | LOW | T6 | The proposal calls ChangeRequestService::submit with object kind, object key and body. Phase 04 submit takes an existing request id and If-Match (phase-04.md:859); creation with kind and key is create(origin = Dedicated), with submit: true (phase-04.md:858, 881) | section 3.5 step 4 (phase-12.md:790) | greps: "ChangeRequestService::create" and "submit: true" do not occur in phase-12.md

### V-12-03 L-02 exclusion register   (source: header "Requirements carried", L-02; result: GAPS 1)
- T1 SQL: PASS - exclusions (phase-12.md:253-273) has unit, reason_code (EXC_ prefix CHECK), reason (10 to 2000), owner_id, approved_by, review_due, expires_at, change_request_id, volume_effect_snapshot, ended_at, ended_reason and CHECK approved_by <> owner_id; uq_exclusions_active at phase-12.md:275; grants at phase-12.md:353-354; coverage_ledger for the unit check at phase-12.md:282; settings at phase-12.md:394.
- T2 Route: PASS - GET /coverage/exclusions (phase-12.md:597, data.steward and dashboard.read, exclusion.read) and POST /coverage/exclusions (598, data.steward, exclusion.propose). API.md:328-329 matches.
- T3 Repository and types: PASS - CoverageRepository active_exclusions, insert_exclusion, end_exclusions and list_exclusions (phase-12.md:506-518) with parameters; NewExclusion (phase-12.md:477-480); ExclusionProposal and ExclusionView (phase-12.md:576-577).
- T4 Logic: PASS - POST steps 1-3 (phase-12.md:807-814) with 422 unknown_unit, reason_code_inactive, term_too_long, review_too_far, review_before_expiry and 409 exclusion_active; owner_not_active is in the E_VALIDATION reason list (phase-12.md:957). ChangeTarget prepare, check_before_decision and activate (phase-12.md:816-829) with 403 E_SELF_APPROVAL. volume_effect unknown is refused at submit with E_EVIDENCE_MISSING (phase-04.md:836).
- T5 Audit: PASS - exclusion.activated and exclusion.expired (phase-12.md:978); the expiry step 2 (775-776); the proposal is a change request whose events are reused from phase 04 (phase-12.md:984).
- T6 Cross-references: PASS - family exclusion and prefix EXC_ (phase-09.md:739, 741; DR-82 at research.md:93); the family CHECK includes exclusion (phase-04.md:60-61); a material base class routes to cr_material (phase-04.md:574-575); canon.txn amount_lcy, branch_id, product_id and channel_id (phase-10.md:951-975); E_SELF_APPROVAL 403 (API.md:971). E_EVIDENCE_MISSING is defined at phase-04.md:833 and 836 but is not in API.md section 6 (Summary, observation 2). Naming defect: V-12-03-a.
- T7 Consistency: PASS - no 7.x block changes the exclusion; the table CHECK approved_by <> owner_id agrees with 3.13 (phase-12.md:1067-1086).
- GAP V-12-03-a | LOW | T6 | The proposal names ChangeRequestService::submit with object kind exclusion and object key exclusion:<source>:<unit>:<unit_ref>; the call that takes kind and key is create(origin = Dedicated) with submit: true | section 3.6 step 3 (phase-12.md:814) | greps: "ChangeRequestService::create" does not occur in phase-12.md; phase-04.md:858 and 881 hold the create and submit: true rules

### V-12-04 L-05 quarantine queue   (source: header "Requirements carried", L-05; result: GAPS 2)
- T1 SQL: PASS - admin.quarantine_items (phase-11.md:178-200) has owner_id, status (open, retried, accepted, excluded at phase-11.md:190), version and kind. akr_app may UPDATE status, owner_id, exclusion_reason, excluded_by, excluded_at, updated_at and version (phase-11.md:310-311), which covers the assign route. Phase 12 adds no DDL (phase-12.md:51-54).
- T2 Route: PASS - POST /quarantine/{id}/assign (phase-12.md:604, data.steward, Cedar quarantine.assign). GET /quarantine, retry and exclude are phase 11's (phase-11.md:697-699) and are extended in 3.11 (phase-12.md:939-948).
- T3 Repository and types: PASS - QuarantineRepository assign(tx, id, owner, expected_version) and accept_conflicts (phase-12.md:534-537); QuarantineFilter extension (phase-12.md:538); QuarantineAssignInput and QuarantineExclusionInput (phase-12.md:580-581); QuarantineItem view.
- T4 Logic: PASS - assign rule (owner must be an active steward; row open or retried; 409 quarantine_not_open) at phase-12.md:943-944; list extension (939-942); exclusion refusals (946-948); retry rules (950-953). The owner refusal takes owner_not_active from the E_VALIDATION list (phase-12.md:957). The card refusal has no trigger: V-12-04-a.
- T5 Audit: PASS - quarantine.assigned names id, owner_id and reason_code (phase-12.md:945, 982).
- T6 Cross-references: PASS - phase 11 section 7.3 confirms the extension and the gate (phase-11.md:1486-1491); phase 11 Cedar actions quarantine.read, quarantine.retry and quarantine.exclude (phase-11.md:1211-1213); ingest_map_pending job (phase-11.md:823-826); DSL.md B.7 code maps (DSL.md:624); step-up class decision (phase-01.md:710).
- T7 Consistency: GAP (V-12-04-b) for the section 6 test that expects an assigned state. Otherwise PASS: the gate is derived and not stored (phase-11.md:1487-1488), as 3.11 and section 5 assume.
- GAP V-12-04-a | MEDIUM | T4 | card_guard_not_excludable applies to "a card reason", but no reason code is named for a card row. Phase 11 quarantine rows are row_failed or row_conflict only (phase-11.md:186), and phase 10 stops a PAN before landing (phase-10.md:1199), so the refusal has no defined trigger | section 3.11 exclusion refusals (phase-12.md:946-948) | greps: "card" in phase-11.md gives 1155 and 1491 only; no card reason_code value appears in phase-10.md or phase-11.md
- GAP V-12-04-b | LOW | T7 | Section 6 expects "open or retried to assigned"; no status enum or CHECK has an assigned value (phase-12.md:476; phase-11.md:190), and 3.11 says assign only sets the owner | section 6, quarantine transitions test (phase-12.md:1236-1237) | greps: "assigned" as a status occurs only at phase-12.md:1237; no such value in phase-11.md

### V-12-05 VA-16 coverage ledger by product, channel and transaction type, with exclusions   (source: header "Gap pass 1 (coverage), implemented here", VA-16; result: GAPS 1)
- T1 SQL: PASS - coverage_ledger.unit CHECK covers branch, product, txn_type, channel and feed (phase-12.md:285); the reason CHECK matches 3.7 step 5 (phase-12.md:288). exclusions has unit, owner_id, reason and expires_at (phase-12.md:253-270). canon.branch, canon.product and canon.channel (phase-10.md:375, 396, 414); admin.code_map_entries with valid_from and valid_to (phase-10.md:150-162); canon.txn channel_id, product_id, txn_type and branch_id (phase-10.md:970-973).
- T2 Route: PASS - GET /coverage/ledger (phase-12.md:596, data.steward and dashboard.read, coverage.read) with the covered and source filters; GET and POST /coverage/exclusions (597-598).
- T3 Repository and types: GAP (V-12-05-a) for the freshness snapshot type. Otherwise PASS: CoverageRepository upsert_units, list_units and list_exclusions (phase-12.md:506-518) with parameters; NewCoverageUnit (phase-12.md:477-480); CoverageUnitView (phase-12.md:575).
- T4 Logic: PASS - 3.7 steps 1-7 (phase-12.md:843-860): snapshot (843), units by kind including product, channel and txn_type (845-846), last posting (847), volume (849-851), reason precedence (853-855), upsert (857), audit (858-860). One job per source at concurrency 1 (phase-12.md:841).
- T5 Audit: PASS - coverage.unit_dark, coverage.unit_recovered and coverage.ledger_refreshed (phase-12.md:979; step 7 at 858-860). Exclusion events are covered under V-12-03.
- T6 Cross-references: PASS - FreshnessService is named in phase 11 section 3.7 (phase-11.md:1001) and the data-as-of rule is at phase-11.md:1009; bank calendar (phase-09.md:8); DR-39 (research.md:50); canon masters (phase-10.md:375, 396, 414).
- T7 Consistency: PASS - 7.6 states the cover rule for step 5 (phase-12.md:1337-1340), and the reason names match 3.7 (phase-12.md:853-855).
- GAP V-12-05-a | MEDIUM | T3 | The freshness snapshot type DataAsOf, returned by FreshnessService::data_as_of, is not defined in any phase file; its fields appear only in a comment. Phase 11 uses BranchDate in its batch repository and BranchDate is also undefined | section 2 port (phase-12.md:566-567); phase 11 section 3.7 (phase-11.md:571, 1009) | greps: "struct DataAsOf" and "DataAsOf {" find nothing in docs/plan; the DataAsOf hit in phase-08.md:340 is a UI badge name; "BranchDate {" and "struct BranchDate" find nothing

### V-12-06 7.1 Cedar form   (source: 7.1, gap pass 1 item (a); result: PASS)
- T1 SQL: N/A - the block changes Cedar text only and adds no table.
- T2 Route: PASS - each permitted action maps to a route row: recon.run (phase-12.md:589), coverage.statement (595), certificate.attest (594), exclusion.propose (598), known_adjustment.propose (603), quarantine.assign (604). The permits are at phase-12.md:1013-1031.
- T3 Repository and types: N/A - no repository method is involved.
- T4 Logic: PASS - role tests use principal.roles.contains or containsAny (phase-12.md:1015, 1021, 1026, 1031). The AAL test is a string comparison in the forbid (phase-12.md:1034-1036), and the form is stated at phase-12.md:1303-1306. No policy line uses "principal in Akr::Role"; the only hit is the rule text at phase-12.md:1304.
- T5 Audit: N/A - no mutation.
- T6 Cross-references: PASS - principal.roles is Set<String> and context.aal is a String in phase 03 (phase-03.md:566 and 554).
- T7 Consistency: PASS - the base 3.12 policies already use the form the block states (phase-12.md:1015-1031).
- Observation, not a GAP: the CH and IA permit carries @role("CH,IA") at phase-12.md:1023; b3-conventions asks for one role per @role (Rust and API section).

### V-12-07 7.2 reconciliation runs after a restore   (source: 7.2, gap pass 1 run 3b; result: GAPS 1)
- T1 SQL: PASS - recon_runs and recon_results (phase-12.md:72-120) and coverage_ledger (phase-12.md:282) are the tables phase 30 names for the restore check (phase-30.md:288).
- T2 Route: PASS - POST /recon/runs (phase-12.md:589) and the recon_run job in the section 4 job table.
- T3 Repository and types: PASS - ReconRepository insert_run (phase-12.md:488) and claim (490) are the calls the POST path makes for a restore-gap run.
- T4 Logic: GAP (V-12-07-a). The block adds no rule for the restore case, and the base step 1 refusals apply to it.
- T5 Audit: PASS - recon.requested (phase-12.md:970) and recon.completed or recon.failed (973) name the run events.
- T6 Cross-references: PASS - phase 30 section 3.9 steps 2 and 5 (phase-30.md:283-288) match the order 7.2 relies on; phase 30 cites this block (phase-30.md:578) and names phase 12 recon_run (phase-30.md:495).
- T7 Consistency: GAP (V-12-07-a). 7.2 says a restore-gap run is "valid as it stands", but 3.1 step 1 refuses both cases the restore produces.
- GAP V-12-07-a | HIGH | T7 | 7.2 says a restore-gap run is valid as it stands and that a date not reloaded shows as a break. 3.1 step 1 refuses a day run for a date with no batch (409 no_batches) and a source that is not active (409 source_not_active). Phase 11 halts a source on restore suspicion, and phase 30 queues one run per gap date through the POST path. The block resolves neither case | section 7.2 or 3.1 step 1 (phase-12.md:637-639, 1309-1313); phase 11 (phase-11.md:926, 1378); phase 30 (phase-30.md:288) | greps: no restore-specific rule in phase-12.md; "no_batches" refuses only at phase-12.md:639; "source_not_active" refuses only at 637 and 959

### V-12-08 7.3 backfill routes removed   (source: 7.3, gap pass 1 run 3b; result: GAPS 1)
- T1 SQL: N/A - no table.
- T2 Route: PASS - no backfill row in the phase 12 route table (phase-12.md:589-604) or in the OpenAPI count of 16 operations (phase-12.md:1113-1115). API.md has no /sources/{id}/backfills or /backfills/{id} row; API.md:243 cites DR-380 only.
- T3 Repository and types: N/A - no repository method is involved.
- T4 Logic: PASS - the block states the replacement: the re-read window (DR-37) and phase 11 automatic catch-up (DR-90) cover gaps, and an explicit range beyond the overlap is P2 (phase-12.md:1316-1319).
- T5 Audit: N/A - no mutation.
- T6 Cross-references: PASS - DR-380 (research.md:391), DR-37 (research.md:48), DR-90 (research.md:101) and API.md:243. The phase 11 reference is stale: V-12-08-a.
- T7 Consistency: PASS - the block closes the open doubt "Backfill routes" (phase-12.md:1355-1357); the block says what it closes, so it wins.
- GAP V-12-08-a | LOW | T7 | Phase 11 still says the operator-planned backfill of API.md 3.5 (phase 12) exists for explicit slices. 7.3 removes those rows and makes explicit ranges beyond the overlap P2 | phase 11 section 3.5 step 5 (phase-11.md:962-963), outside this phase | greps: "backfill" in phase-12.md after 7.3 occurs only in the open doubt (1355-1357) and 7.3 itself; API.md has no backfill route rows

### V-12-09 7.4 deferred rungs   (source: 7.4, gap pass 1 run 5a block; result: PASS)
- T1 SQL: PASS - the L4 source table is canon.payment_link (phase-10.md:1082), which 7.4 names. No phase 12 table is needed for L2, L4, L5 or L6.
- T2 Route: N/A - no route serves the deferred rungs.
- T3 Repository and types: N/A - no repository method is involved.
- T4 Logic: PASS - the deferred rungs are excluded in 3.1 (phase-12.md:619-621); the receipt, counts, control totals and GL tie-out rungs are numbered in steps 2 to 6 (phase-12.md:656-672).
- T5 Audit: N/A - no mutation is involved for the deferred rungs.
- T6 Cross-references: PASS - MODULES M7 lists the four rungs for P1 (docs/MODULES.md:292); DR-394 (research.md:405); C-43 and DR-394 in phase 10 (phase-10.md:34).
- T7 Consistency: PASS - 7.4 closes the open doubt on the deferred rungs, which the base text still lists (phase-12.md:1361-1362). The block says it closes the doubt, so it wins.

### V-12-10 7.5 activate writes detail.accepted_delta   (source: 7.5, adversarial pass run 3 header at phase-12.md:1327; result: GAPS 4)
- T1 SQL: GAP (V-12-10-a). dq_issues.detail exists (phase-12.md:195). recon_results.delta is granted to akr_app (phase-12.md:138-140). The akr_app UPDATE grant on dq_issues (phase-12.md:222-223) has no detail column.
- T2 Route: N/A - activation runs inside the phase 04 decide transaction; no route is added here (3.5, phase-12.md:799).
- T3 Repository and types: GAP (V-12-10-b) for the latest-result lookup. The accept call is named (phase-12.md:527).
- T4 Logic: GAP (V-12-10-c) for the undefined "latest" result. Otherwise the activate rule is stated (phase-12.md:799-801) and 7.5 gives a test (phase-12.md:1335).
- T5 Audit: PASS - known_adjustment.activated (phase-12.md:977) names the activation that carries the write.
- T6 Cross-references: PASS - DR-409 (research.md:420) says known adjustments bind the field and the amount; DR-58 (research.md:69). The naming defect is V-12-10-d.
- T7 Consistency: PASS - 7.5 replaces "same object and group" in step 7 with "same object and field" (phase-12.md:1331-1333), and step 8 stores the group key as field (phase-12.md:679). The object-key question is V-12-11-a.
- GAP V-12-10-a | HIGH | T1 | activate writes detail.accepted_delta, but akr_app has no UPDATE grant on admin.dq_issues.detail, so the write is refused at run time | section 1, migration 0037 grants (phase-12.md:222-223), or block 7.5 (phase-12.md:1330) | greps: no GRANT line for dq_issues lists detail; the only UPDATE grants that list detail are on recon_results (phase-12.md:139-140)
- GAP V-12-10-b | MEDIUM | T3 | No repository method returns the latest recon result for an object and field; ReconRepository has expected_for by run only | section 2, ReconRepository (phase-12.md:487-503) | greps: "latest" occurs only at phase-12.md:1330-1331; no ReconRepository method takes an object and a group key
- GAP V-12-10-c | MEDIUM | T4 | "Latest" is undefined (by evaluated_at, run created_at or run id). The issue object is stored as <rung>:<object> (step 8) while recon_results keeps rung and object apart. The no-result case is not stated | section 3.5 activate (phase-12.md:799-801) or 7.5 (phase-12.md:1330-1331) | greps: "latest" only at 1330-1331; "<rung>:<object>" only at phase-12.md:679
- GAP V-12-10-d | LOW | T6 | 7.5 names DqIssueRepository::accepted_for; section 2 defines DqRepository, with accepted_for at phase-12.md:531 | block 7.5 (phase-12.md:1331), or section 2 (phase-12.md:522) | greps: "DqIssueRepository" occurs only at phase-12.md:1331

### V-12-11 7.5 step 7 confirmed_gap rule   (source: 7.5 and section 3.1 step 7, phase-12.md:677; result: GAPS 1)
- T1 SQL: PASS - dq_issues status, accepted_until and detail (phase-12.md:171-205); recon_results delta and within_tolerance (phase-12.md:112-116).
- T2 Route: N/A - the evaluation runs in recon_run (phase-12.md:654-684) and has no route.
- T3 Repository and types: PASS - accepted_for is named (phase-12.md:531) and 7.5 gives its new parameter list (phase-12.md:1331); evaluate takes ResultEvaluation (phase-12.md:495).
- T4 Logic: GAP (V-12-11-a) for the lookup key. The rest is testable: confirmed_gap needs abs(accepted_delta) at least abs(delta) (phase-12.md:1332-1333), and the test case gives values (phase-12.md:1335).
- T5 Audit: PASS - the outcomes are written in one transaction and counted in recon.completed (phase-12.md:682, 973).
- T6 Cross-references: PASS - DR-409 (research.md:420) states the field and amount rule.
- T7 Consistency: PASS - 7.5 replaces "same object and group" with "same object and field" (phase-12.md:1331-1333), and field is the group key (phase-12.md:679). The open part is the key in V-12-11-a.
- GAP V-12-11-a | MEDIUM | T4 | Step 7 looks up the accepted issue by "the same object", which is the source object. Step 8 stores dq_issues.object as <rung>:<object>, so the lookup as written never matches and confirmed_gap never fires; the key is not stated | section 3.1 step 7 (phase-12.md:677) and 7.5 (phase-12.md:1331-1333) | greps: "<rung>:<object>" occurs only at phase-12.md:679; the accepted_for call at 677 passes object only

### V-12-12 7.6 blind period covers the as-of date   (source: 7.6, adversarial pass run 3 header at phase-12.md:1327; result: GAPS 1)
- T1 SQL: PASS - admin.blind_periods has business_from and business_to as nullable DATE columns (phase-11.md:86-87). The rule in 7.6 reads these columns.
- T2 Route: N/A - no route here; declarations are phase 11's POST /coverage/blind-periods (API.md:331).
- T3 Repository and types: GAP (V-12-12-a).
- T4 Logic: PASS - the cover rule is explicit (phase-12.md:1337-1338): business_from is NULL or on or before d, and business_to is NULL or on or after d.
- T5 Audit: PASS - a change of reason emits coverage.unit_dark or coverage.unit_recovered (phase-12.md:858-860, 979).
- T6 Cross-references: PASS - phase 11 sets business_from on automatic opens and reads source_stale for a paused source (phase-11.md:1558-1564); base 3.7 step 5 matches (phase-12.md:853-855).
- T7 Consistency: PASS - base 3.7 step 5 (phase-12.md:853-855) and 3.10 (phase-12.md:925-933) use the same reason names, and 7.6 states the rule for step 5.
- GAP V-12-12-a | MEDIUM | T3 | No named method returns the blind periods that cover a date for a source or the system. Section 2 CoverageRepository has none. Phase 11 BlindPeriodRepository has open, close, open_for_source (source scope only) and list(f: &BlindFilter), and BlindFilter is defined nowhere | section 2 CoverageRepository (phase-12.md:506-518) or phase 11 BlindPeriodRepository (phase-11.md:586-592) | greps: "BlindFilter" occurs only at phase-11.md:592; no covering or date method in the section 2 traits of phase-12.md

## Summary

- Items checked: 12. Five from the header "Requirements carried" and "Implemented here" lists (C-25, C-28, L-02, L-05, VA-16); four from 7.1 to 7.4 (gap pass blocks); three from the adversarial run 3 blocks 7.5 and 7.6.
- Items fully PASS: 2 (V-12-06 7.1 Cedar form; V-12-09 7.4 deferred rungs).
- GAP lines by severity: HIGH 2, MEDIUM 8, LOW 6 (16 in all).
- Line numbers refer to docs/plan/phase-12.md unless another file is named. Every grep named in a GAP line was run.

Observations (not GAP lines):
1. No service method signature is given anywhere in phase 12 (ReconService, RungEvaluator, DataQualityService, CoverageService, CoverageStatementService, QuarantineService, CertificateBuilder, CoverageLedger). Section 2 gives repository traits and ports only (phase-12.md:487-567). T3 was passed where the repository methods and DTOs carry the item. planning.md asks for service method signatures, so the lead should decide whether to add them.
2. E_EVIDENCE_MISSING is defined by phase 04 (phase-04.md:833, 836) and used in section 6 (phase-12.md:1233), but it is not in API.md section 6 (zero occurrences in docs/API.md).
3. The annotation @role("CH,IA") at phase-12.md:1023 names two roles in one annotation; b3-conventions.md (Rust and API section) asks for one role per @role. Not counted as a truth failure.
4. Name clash: "DataAsOf" also appears in phase-08.md:340 as a UI badge name; it is not a type definition there.

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-12-01-a | MEDIUM | T5 | Audit event for the recon-break dq_issues upsert in step 8 | section 3.1 step 8 (phase-12.md:679) |
| V-12-01-b | LOW | T5 | Audit event for the claim and set_waiting status changes | section 3.1 steps 1 and 3 (phase-12.md:655, 660-661) |
| V-12-02-a | MEDIUM | T4 | Test for "signed by the bank" (document kind or flag) | section 3.5 step 3 (phase-12.md:789) |
| V-12-02-b | LOW | T6 | Proposal names submit; the call with kind and key is create(origin = Dedicated) with submit: true | section 3.5 step 4 (phase-12.md:790) |
| V-12-03-a | LOW | T6 | Same method naming for the exclusion proposal | section 3.6 step 3 (phase-12.md:814) |
| V-12-04-a | MEDIUM | T4 | Reason code or other trigger for the card_guard_not_excludable refusal | section 3.11 (phase-12.md:946-948) |
| V-12-04-b | LOW | T7 | Section 6 test expects an assigned status that no enum or CHECK defines | section 6, quarantine transitions test (phase-12.md:1236-1237) |
| V-12-05-a | MEDIUM | T3 | Type DataAsOf (and BranchDate) is not defined | section 2 port (phase-12.md:566-567); phase 11 (phase-11.md:571, 1009) |
| V-12-07-a | HIGH | T7 | Restore-gap runs are refused by 3.1 step 1 (no_batches, source_not_active); 7.2 does not resolve it | section 7.2 or 3.1 step 1 (phase-12.md:637-639, 1309-1313) |
| V-12-08-a | LOW | T7 | Phase 11 still refers to the phase 12 backfill that 7.3 removed | phase-11.md:962-963 (outside this phase) |
| V-12-10-a | HIGH | T1 | akr_app has no UPDATE grant on dq_issues.detail, so activate cannot write accepted_delta | section 1, migration 0037 grants (phase-12.md:222-223) |
| V-12-10-b | MEDIUM | T3 | No repository method returns the latest recon result for an object and field | section 2, ReconRepository (phase-12.md:487-503) |
| V-12-10-c | MEDIUM | T4 | "Latest" result and the object key for the delta lookup are undefined; no-result case not stated | section 3.5 activate (phase-12.md:799-801) or 7.5 (phase-12.md:1330-1331) |
| V-12-10-d | LOW | T6 | 7.5 names DqIssueRepository; section 2 defines DqRepository | block 7.5 (phase-12.md:1331); section 2 (phase-12.md:522) |
| V-12-11-a | MEDIUM | T4 | Step 7 lookup key ("same object") does not match the <rung>:<object> key stored by step 8 | section 3.1 step 7 (phase-12.md:677) and 7.5 (phase-12.md:1331-1333) |
| V-12-12-a | MEDIUM | T3 | No method returns blind periods covering a date; BlindFilter is defined nowhere | section 2 CoverageRepository (phase-12.md:506-518) or phase 11 BlindPeriodRepository (phase-11.md:586-592) |
