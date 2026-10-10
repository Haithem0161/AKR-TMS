# Verification pass, phase 23 (Haiku draft for lead review)

## Items selected (STEP 1)

Header lists (Requirements carried, Implemented here, partly implemented; order as in the header). The phase has three
non-adversarial 7.x blocks (7.1, 7.2, 7.3), so one header item is added in their place. The only adversarial block is 7.4
(run 4) and it holds one deliverable, so two header items are added in its place.

- V-23-01 SP-53 plan and dry run (Requirements carried, 1st of 6): POST /retention/dry-runs, job retention_run, RetentionEngine::execute
- V-23-02 M-05 legal holds, placement (Requirements carried, 4th): POST /retention/holds, LegalHoldService::place
- V-23-03 SP-56 crypto-shredding of owner-following evidence (Requirements carried, 5th): EvidenceShredder
- V-23-04 SP-57 record purge of party trees (Implemented here, 1st): section 3.5 batch procedure
- V-23-05 F-17 records kept after the relationship ends (Implemented here, 2nd): section 3.5 blocking references and 3.2 R6
- V-23-06 SP-55 plan, detach and deletion certificates (partly implemented, 1st): admin.retention_drop_partition, CertificateService
- V-23-07 SP-63 per-class retention report (partly implemented, 3rd): GET /retention/report, RetentionReportService
- V-23-08 VD-19 seven-year floor, configurable upward (partly implemented, 4th): retention.period_years settings
- V-23-09 Gap 7.1 deletion-certificate key name (akr-sign-deletion)
- V-23-10 Gap 7.2 handlers for the tables of later phases
- V-23-11 Gap 7.3 DELETE grants follow the handlers
- V-23-12 Adversarial 7.4 (run 4) a case hold follows merged_into

Method: each truth was checked against the phase file and the owning phase files named in the evidence. Line numbers are
phase-23.md unless a file is named. Greps are listed per gap; a grep "found nothing" means no definition or statement
exists in docs/plan/phase-*.md and docs/plan/research.md.

---

### V-23-01 SP-53 plan and dry run (POST /retention/dry-runs, job retention_run, RetentionEngine::execute)   (source: header SP-53, Requirements carried; result: GAPS 5)
- T1 SQL: PASS - `admin.retention_runs` has trigger, dry_run, requested_by, as_of, state, error_code, summary (phase-23.md:75-90); `admin.retention_run_items` holds the counts (111-131). Settings read by the engine exist: "retention.batch_rows" (407), "retention.drop_timeout_s" (413), "schedule.retention_run" (418). Phase 00 `admin.jobs` has `coalesce_key` (phase-00.md:111).
- T2 Route: GAP - POST `/api/v1/retention/dry-runs` (phase-23.md:530, scope `admin.system`, 202) and Cedar `retention.run.dry` (843-844; ADM permit 867-870) are present. Two defects, see a and b.
- T3 Repository and types: GAP - `RetentionRunRepository` methods (phase-23.md:492) and `DryRunRequest {}` (486) are named with parameters. `NewRun` (492) and `RunSummary` (512) are named but never defined, see c.
- T4 Logic: PASS - numbered steps 1 to 9 (phase-23.md:548-556): advisory lock and E_CLAIMED (548), `as_of` read from the row (549), E_UNAVAILABLE and E_INTERNAL (555); dry-run variant (558). The route itself is one sentence (545); its idempotency rule is the phase 00 rule named at 790.
- T5 Audit: GAP - two mutations have no named event, see d and e. The dry-run finish is named: "retention.run_finished ... dry_run" (558; table 805). The start is named (804).
- T6 Cross-references: PASS - `admin.retention_assignment` (phase-00.md:303), `admin.settings` (phase-00.md:203), `admin.jobs` (phase-00.md:100); BusinessClock as `BankCalendarClock` (phase-09.md:8); problem codes E_CLAIMED (API.md:975) and E_UNAVAILABLE (API.md:1002); scope `admin.system` (API.md:104).
- T7 Consistency: PASS - "the job is never run from a request" (560) agrees with the enqueue-only route (545). 7.2 (1017-1019) and 7.4 (1031-1033) change drops and matching, not the dry run or the engine steps.
- Gaps:
  - `GAP V-23-01-a | MEDIUM | T2 | ADM requests the dry run but holds no audit.read; GET /retention/runs/{id} (scope audit.read, line 532) is the read check 5a runs as adm@ (line 974). Cedar permits ADM retention.run.read (868-869), so the route scope does not admit a role its policy permits | Section 2 route row 532, or the ADM permit at 867-870 | grep -n "ADM" docs/API.md section 2: ADM appears only with admin.system (API.md:104), never with audit.read (API.md:100).`
  - `GAP V-23-01-b | LOW | T2 | The route row for POST /retention/dry-runs names no Idempotency-Key, but section 5.1 (948) and check 5a (974) require one | Section 2 route row 530 | grep -n "Idempotency-Key" phase-23.md: rows 526, 729, 948, 960, 974, 979; row 530 is not among them.`
  - `GAP V-23-01-c | LOW | T3 | NewRun (create input) and RunSummary (return of execute) are named and not defined. Phase 16 uses the name NewRun for a different type, scenario runs (phase-16.md:732, 767) | Section 2 DTOs (483-487) and services (512) | grep -n "NewRun\|RunSummary" docs/plan/phase-*.md research.md: only phase-23.md:492 and 512, and the phase 16 and 31 uses of other types.`
  - `GAP V-23-01-d | MEDIUM | T5 | Step 8 marks a failed item "with reason_code and an audit event" (555), but no event kind is named for an item failure | Section 3.12 event table (796-817) | grep -n "fail" in 796-817 finds only legal_hold.release_ended (802); grep -n "item_failed\|retention.item" docs/plan/*.md: none.`
  - `GAP V-23-01-e | MEDIUM | T5 | POST /retention/dry-runs inserts a run row with requested_by and enqueues the job. The only run event, retention.run_started, is "on start" (804), so the request has no audit event | Section 3.12 table (796-817) or 3.1 first paragraph (545) | grep -n "dry_run_request" phase-23.md: lines 77, 79, 87, 88, 353, 545, 902; none is an event row.`

### V-23-02 M-05 legal hold placement (POST /retention/holds, LegalHoldService::place)   (source: header M-05 and M4 retention half, Requirements carried; result: GAPS 4)
- T1 SQL: PASS - `admin.legal_holds` (phase-23.md:41-72) has every column the place step writes: hold_ref (43), kind (44), party_id (45), case_id (46), range_table/from/to (47-49), scope (50), reason_code (51), reason_enc (52), state (53), placed_by (57), placed_at (58), version (61). Sequence `admin.hold_ref_seq` (35) and "GRANT SELECT, INSERT ON admin.legal_holds TO akr_app" (350). FK targets: admin.users (phase-01.md:52), casework.cases (phase-17.md:290), canon.party (phase-10.md:451).
- T2 Route: PASS - POST `/api/v1/retention/holds` (phase-23.md:526): scope `report.file`, SU `legal_hold.place`, `Idempotency-Key`. `report.file` is defined for M (API.md:94). Cedar `retention.hold.place` is named at 718 and 843; the M-only permit is at 852-855.
- T3 Repository and types: GAP - `PlaceLegalHold` (484) and `LegalHoldService::place(ctx, PlaceLegalHold, idem) -> LegalHold` (508) are named with parameters. `NewLegalHold` (490) is named and not defined; the `HoldView` fields are not listed, see b.
- T4 Logic: GAP - numbered steps 1 to 9 (phase-23.md:719-727) give the errors (E_STEPUP_REQUIRED 782, E_VALIDATION 720-721, E_NOT_FOUND 720, E_FORBIDDEN 780) and the lock order (722). The 500-character limit on reason_text has no step, see c.
- T5 Audit: PASS - `legal_hold.placed` with payload `{ hold_ref, reason_code }` (798; step 6 at 724). The scope is kept out of the payload (727).
- T6 Cross-references: GAP - PASS for `PolicyPackService::scan(scope, text)` (phase-09.md:797), scope `reason_text` (phase-09.md:782), E_VALIDATION with errors[] (phase-09.md:926), `StepUpService::consume` (phase-01.md:731), class `decision` (phase-01.md:1107), Cedar file range "60 audit" (phase-03.md:543; Open doubts 23 at 1100 asks the bundle owner to confirm 61). Gaps: HoldView name clash (a), wrong section reference (d).
- T7 Consistency: PASS - 712 says matching applies "at every run, not at placement"; the placement lock (722) covers only the held case's parties at placement. 7.4 (1031-1033) changes run-time matching only, so there is no conflict.
- Gaps:
  - `GAP V-23-02-a | MEDIUM | T6 | HoldView is also a utoipa schema of phase 29 with different fields (phase-29.md:867: "HoldView { id, hold_ref, rail, mode, kind, state, reason_codes"). Phase 29 is served by akr-api (phase-29.md:9), so the OpenAPI names collide | Section 2 utoipa list (519) and the place response (726) | grep -n "HoldView" docs/plan/phase-*.md research.md: phase-23.md:508, 519 and phase-29.md:867 only.`
  - `GAP V-23-02-b | LOW | T3 | NewLegalHold (repository insert input) and the fields of HoldView (place response 726, list and detail 750) are not defined; 750 gives only the omitted fields | Section 2 (490, 508) | grep -n "NewLegalHold" docs/plan/*.md research.md: phase-23.md:490 only; grep -n "HoldView {" docs/plan/phase-23.md: none.`
  - `GAP V-23-02-c | LOW | T4 | The 500-character limit on reason_text (484) has no step in 3.8 and no error code; step 2 (720) lists the field combination and existence checks only | Section 3.8 step 2 (720) | grep -n "500" docs/plan/phase-23.md: line 484 only.`
  - `GAP V-23-02-d | LOW | T6 | The DTO says the text is "scanned, section 3.2" (484); the scan is in section 3.8 step 3 (721). Section 3.2 is classes and clocks | Section 2 DTO (484) | grep -n "scanned" docs/plan/phase-23.md: lines 484 and 721 only.`

### V-23-03 SP-56 crypto-shredding of owner-following evidence (EvidenceShredder)   (source: header SP-56, Requirements carried; result: GAPS 2)
- T1 SQL: PASS - `admin.evidence_objects` (phase-02.md:353) has owner_kind (356), owner_id (357), retention_class (358), version_id (361), lock_until (373), wrapped_dek (374), shredded_at (376), updated_at (379). Phase 23 adds `object_deleted_at` (phase-23.md:255), also documented in phase 02 7.3 (phase-02.md:1209); both are IF NOT EXISTS with the same type. The UPDATE grant is at phase-23.md:369.
- T2 Route: N/A - no route; the shred runs in the owner's transaction (phase-23.md:666). Reads of a shredded object answer 404 E_NOT_FOUND (phase-02.md:882, 956).
- T3 Repository and types: GAP - the EvidenceShredder methods are named with parameters (phase-23.md:500; phase-02.md:1202-1208). `ShredOutcome`, the return of `shred`, is never defined, see a.
- T4 Logic: PASS - shred: FOR UPDATE, erase wrapped_dek, set shredded_at, audit (666). delete_object: delete the versions, set object_deleted_at, a COMPLIANCE refusal is recorded as blocked and retried (668). Lock order owner then evidence (3.15 item 5, line 890). The repeat-shred outcome is open, see a.
- T5 Audit: PASS - `retention.evidence_shredded` (810; 666) and `retention.object_deleted` (811) are named.
- T6 Cross-references: GAP - PASS for the phase 02 7.3 names shred, delete_object and extend_lock (phase-02.md:1202-1208), NotClean and Shredded (phase-02.md:596, 882), evidence.lock_days R10 = 3650 (phase-02.md:511), casework.evidence_items.object_id (phase-18.md:220). Gap: section reference, see b.
- T7 Consistency: PASS - phase 02 7.3 (1202-1208) agrees with 3.6 (666-668) on erasing wrapped_dek and on deleting only after lock_until.
- Gaps:
  - `GAP V-23-03-a | MEDIUM | T3 | ShredOutcome (return of shred) is not defined and its values are not listed. The text does not say what a repeat shred of a shredded row returns or whether shredded_at changes | Section 3.6 (666) or the port at 500 | grep -n "ShredOutcome" docs/plan/*.md research.md: phase-23.md:500 and phase-02.md:1202 only.`
  - `GAP V-23-03-b | LOW | T6 | Phase 02 7.3 cites "phase 23 section 1.1" for the classification row of object_deleted_at. The row is in section 1.2 (phase-23.md:449, inside the classification insert at 424-452) | The text at phase-02.md:1210, which the phase 02 owner must correct; phase 23 is correct | grep -n "section 1.1" docs/plan/phase-02.md: line 1210 only; grep -n "object_deleted_at" phase-23.md: 255, 369, 449, 668, 935 (none in section 1.1, lines 28-187).`

### V-23-04 SP-57 record purge of party trees (section 3.5 batch procedure, RelationshipEndResolver)   (source: header SP-57, Gap pass 1 implemented here; result: GAPS 4)
- T1 SQL: GAP - the party path is named per table (phase-23.md:647). Most columns exist: canon.customer.party_id (phase-10.md:601), valid_to (620), recorded_to (622); canon.account.closed_on (641) and status (642); canon.account_party_role.party_id (671); canon.party_relationship.from_party and to_party (572-573); party_id on canon.kyc_document, party_name, party_identifier and party_contact (phase-10.md:495-547, 1381). The tombstone table is at phase-23.md:157-165. Two paths do not resolve, see a and b.
- T2 Route: N/A - engine batch (651-658); no route.
- T3 Repository and types: GAP - the hold re-read uses named methods (LegalHoldRepository::active_for_party and active_case_holds, phase-23.md:490). The blocking-reference check is a closed table list (649). RelationshipEndResolver is named only in the 3.5 heading (643) and has no signature, see c.
- T4 Logic: PASS - numbered steps 1 to 7 (phase-23.md:652-658): candidate rule (652), advisory lock and re-read of holds and references (653), delete order (654), shred in the same transaction (655), tombstones (656), manifest (657), rollback of one party only (658).
- T5 Audit: PASS - `retention.purge_deferred` (809; step 2 at 653), `retention.records_purged` (808; step 6 at 657), `retention.evidence_shredded` (810; 655). The failure mark of step 7 has no event and is logged once at V-23-01-d.
- T6 Cross-references: GAP - PASS for canon.customer valid_to and recorded_to (phase-10.md:620, 622), alert.alerts.party_id (phase-17.md:120), casework.cases.party_id (phase-17.md:294), casework.case_parties.party_id (phase-17.md:374), rpt.reports.subject_party_id (phase-21.md:82), DR-190 and DR-191 (phase-23.md:1052, 1054), section 3.15 (885), metric akr_ingest_tombstoned_total (925). Open doubts 22 is inaccurate, see d.
- T7 Consistency: GAP (same defect as V-23-12-a, counted there once) - step 2 (653) re-reads "case holds whose case_parties contain it", while 649 defers to section 3.8 and 7.4 (1031-1033) widens the case-hold match.
- Gaps:
  - `GAP V-23-04-a | MEDIUM | T1 | canon.facility and canon.trade_instrument have no account_id, but 3.5 gives their path as "through account_id" (phase-23.md:647). Facility has customer_id (phase-10.md:1312); trade_instrument has customer_id (1337) and a parties JSONB (1339). A row with no path is never purged (647) | Section 3.5 party path (647) | grep -n "account_id" over phase-10.md 1310-1358: none in facility or trade_instrument.`
  - `GAP V-23-04-b | LOW | T1 | kyc.declared_profiles (customer_id, review_id) and kyc.review_queries (review_id) have no party_id, but 3.5 says the KYC family is purged "by party_id" (647). The owner rule (605) covers them through kyc.kyc_reviews (phase-15.md:363, 318), but no path is written | Section 3.5 party path (647) | grep -n "party_id" over phase-15.md 316-331 and 356-370: none.`
  - `GAP V-23-04-c | LOW | T3 | RelationshipEndResolver computes the party end date and is named only in the 3.5 heading (643). It has no signature in section 2 (507-517) | Section 2 services, or 3.5 with a signature | grep -n "RelationshipEndResolver" docs/plan/*.md research.md: phase-23.md:643 only.`
  - `GAP V-23-04-d | LOW | T6 | Open doubts 22 says canon.txn "has no index on account_id in phase 10 as far as this draft reads". Phase 10 has idx_txn_account (phase-10.md:1120), so the need is already met | Open doubts 22 (phase-23.md:1099) | grep -n "idx_txn_account" docs/plan/phase-10.md: line 1120.`

### V-23-05 F-17 records kept after the relationship ends (party deferral, R6 records)   (source: header F-17, Gap pass 1 implemented here; result: GAPS 1)
- T1 SQL: PASS - the four blocking columns exist: alert.alerts.party_id (phase-17.md:120), casework.cases.party_id (294), casework.case_parties.party_id (374), rpt.reports.subject_party_id (phase-21.md:82). Evidence kept with its owner: alert.alerts.snapshot_object_id (phase-17.md:154). Notes and decisions are keyed by case (phase-18.md:71, 137).
- T2 Route: N/A - no route; the deferral runs in the purge batch (phase-23.md:649, 653).
- T3 Repository and types: PASS - the hold re-read uses named methods (490); the blocking list is named (649).
- T4 Logic: PASS - a closed list of four references (649) and "deferred ... next party" before any delete (653). R6 rows are report_only (596) and case evidence is not shredded (670).
- T5 Audit: PASS - `retention.purge_deferred` carries reason and count, no party id (phase-23.md:809; step 2 at 653).
- T6 Cross-references: GAP - see a. The other names are PASS as listed in T1.
- T7 Consistency: PASS - R6 report_only (596) and the closed deferral list (649) agree with DR-191 (phase-23.md:1054).
- Gaps:
  - `GAP V-23-05-a | MEDIUM | T6 | alert.screening_alerts carries a party subject (subject_kind 'party' with subject_id, phase-14.md:254-255) and is R7 kept (phase-23.md:597). The closed blocking list (649) omits it, so a purged party leaves a kept screening record pointing at a missing party. Open doubts 22 defers only the column check (1099), and phase 14 has no purge rule for it | Section 3.5 blocking references (649) | grep -n -i "screening" docs/plan/phase-23.md: lines 5, 402, 597, 1054, 1099, 1102 only, none a blocking rule; grep -n "screening_alerts" docs/plan/research.md: none.`

### V-23-06 SP-55 plan, detach and deletion certificates (admin.retention_drop_partition, CertificateService)   (source: header SP-55, Gap pass 1 partly implemented; result: GAPS 2)
- T1 SQL: GAP - the drop function (phase-23.md:260-341), admin.deletion_certificates (138-153) and retention.drop_timeout_s (413) exist. The R4 row names feat.feature_runs as a partition_drop parent (592) and the allow-list names it (282). The table is not partitioned, see a. The other allow-list parents are partitioned (phase-10.md:900-1402; phase-13.md:172, 190, 207; phase-17.md:106).
- T2 Route: N/A - the drop and the certificates are engine calls (phase-23.md:552, 609, 680-688). The read route is at 533.
- T3 Repository and types: PASS - PartitionDropper (505), CertificateSigner (501), WormWriter (502), CertificateService::issue(tx, run_id, retention_class, kind, manifests) (513) and DeletionCertificate (480) are named with parameters. Manifest fields are listed (680).
- T4 Logic: PASS - step 5 (a) to (d) (552); result table with each effect (611-619); E_UNAVAILABLE stops before a class's deletes (555); lock timeout SQLSTATE 55P03 (619); certificate steps (682-688).
- T5 Audit: PASS - `retention.partition_dropped` (806; 613), `retention.partition_blocked` (807; 615), `retention.certificate_issued` (812; 688), `retention.replayed` (813; 696). WORM writes are not database mutations.
- T6 Cross-references: GAP - see b. PASS for admin.batches.mapping_version_id (phase-11.md:150), raw batch_id (phase-11.md:242), evidence.lock_days R10 (phase-02.md:511), the transit key name (phase-00.md:918).
- T7 Consistency: PASS - the 7.2 drop steps (1017-1019) extend 3.3 (621) and do not contradict it. The allow-list they need is logged at V-23-10-a.
- Gaps:
  - `GAP V-23-06-a | MEDIUM | T1 | feat.feature_runs is a plain table with PRIMARY KEY (as_of, feature_id) (phase-13.md:158-169), not a partitioned parent. The R4 row treats it as partition_drop (phase-23.md:592) and the allow-list names it (282). Step 2 of the function returns not_partition on every run, which 3.3 records as failed (618), so this class is never disposed | Section 3.2 R4 row (592) and the allow-list (282) | grep -n "PARTITION BY" docs/plan/phase-13.md: lines 172, 190, 207 only; research.md:109 says feature_runs records whether an absent row is zero or unknown; it makes no partition statement.`
  - `GAP V-23-06-b | LOW | T6 | The WORM bucket is named only as "the COMPLIANCE bucket of phase 02, under the prefix certificates/" (502) and "the WORM bucket" (680). Phase 02 names its COMPLIANCE-locked bucket akr-anchors (phase-02.md:6, 765) and has no certificates/ prefix. The S3 grant at 923 names no bucket | Section 4.4 S3 bullet (923) and Section 2 WormWriter (502) | grep -n "certificates/" docs/plan/phase-*.md: phase-23.md only (502, 680, 687, 759, 923) for S3 keys; grep -n "akr-anchors" docs/plan/phase-02.md: lines 6, 765, 1033 (COMPLIANCE), also 280 and 810.`

### V-23-07 SP-63 per-class retention report (GET /retention/report, RetentionReportService)   (source: header SP-63, Gap pass 1 partly implemented; result: GAPS 3)
- T1 SQL: PASS - the run and item columns (phase-23.md:75-131) hold eligible, deleted, deferred, blocked (item state, 126) and held. The later-of inputs exist: casework.cases.sealed_at (phase-17.md:304), rpt.filings.filed_at (phase-21.md:192), rpt.reports.case_id (78) and created_at (98).
- T2 Route: GAP - GET `/api/v1/retention/report` (phase-23.md:529, scope `audit.read`) and GET runs (531-532) do not admit ADM, but Cedar `retention.adm` permits ADM on retention.report.read and retention.run.read (867-870). ADM holds no audit.read (API.md:100). The IA, SEC and M permits agree with the scope (852-865). See a.
- T3 Repository and types: PASS - `RetentionRunRepository::latest_completed()` and `items(run_id, include_held)` are named (phase-23.md:492); `RetentionReportService { runs, items, clock }` (511). RetentionReportView is named without fields, see b.
- T4 Logic: PASS - latest completed run, not a live count (771); no completed run answers 200 with run null (771); held is null for IA and SEC (771; 826). No request runs the plan (771, 773).
- T5 Audit: N/A - read-only route.
- T6 Cross-references: PASS - audit.read (API.md:100); Cedar retention.report.read named at 843; casework.cases.sealed_at (phase-17.md:304); rpt.filings.filed_at (phase-21.md:192).
- T7 Consistency: GAP - who computes the later-of count is stated two ways, see c.
- Gaps:
  - `GAP V-23-07-a | MEDIUM | T2 | GET /retention/report (529) and GET /retention/runs (531) carry scope audit.read, which ADM lacks (API.md:100), but Cedar permits ADM retention.report.read and retention.run.read (868-869). A route's scope list must admit every role its Cedar policies permit | Section 2 route rows 529 and 531, or the ADM permit at 867-870 | grep -n "audit.read" docs/API.md: line 100 (IA, SEC; M) and the endpoint rows at 204-232; no ADM in any audit.read row.`
  - `GAP V-23-07-b | LOW | T3 | RetentionReportView is in the utoipa list (519) with no field list. Its fields exist only in prose (771) | Section 2 (519) | grep -n "RetentionReportView" docs/plan/*.md research.md: phase-23.md:511 (return type) and 519 (utoipa) only.`
  - `GAP V-23-07-c | MEDIUM | T7 | Who computes the later-of count is stated two ways. 3.2 says "RetentionReportService applies this rule" (580), but its declared dependencies (511) have no port to rpt.reports, rpt.filings or casework.cases. 3.10 says "the summary also gives" the count (771), so the run must compute it. The engine plan for rpt.reports (551) uses the closure clock (570, 596), which carries no later-of rule | Section 3.2 (580) and Section 2 service dependencies (511) | grep -n "later-of" docs/plan/phase-23.md: lines 580, 596, 771, 949, 1054 (DR-191), 1111; none states who computes the count at run time.`

### V-23-08 VD-19 seven-year floor, configurable upward (retention.period_* settings)   (source: header VD-19, Gap pass 1 partly implemented; result: GAPS 1)
- T1 SQL: GAP - the class-suffixed keys cannot be inserted. `retention.period_days.R1` (phase-23.md:389) and `retention.period_years.R2` to `R8` (391-403) contain an upper-case class code, which phase 00's CHECK refuses: `key ~ '^[a-z][a-z0-9_]*(\.[a-z0-9_]+)+$'` (phase-00.md:204). The first INSERT of 0072 (388-415) aborts the migration, so no period is ever seeded. The other keys (407-413, 418-421) match the pattern. The floor rule holds: `CHECK (floor IS NULL OR governed)` (phase-00.md:214), and every floor row is governed TRUE.
- T2 Route: N/A - no route in this phase. A period change goes through the change request route `cr_material` (phase-04.md:571-575) under scope `config.author` (API.md:95).
- T3 Repository and types: PASS - the effective-period rule is a formula (phase-23.md:566). The loader is not named in section 2, but the rule is complete.
- T4 Logic: PASS - `effective(class) = max(configured, floor.min)` (566); cut-off (568); E_BELOW_FLOOR at the change request and the run-time maximum (769).
- T5 Audit: PASS - `retention.floor_applied` once per run and class (566; table 814).
- T6 Cross-references: PASS - phase 09 floor rule E_BELOW_FLOOR (phase-09.md:803); phase 00 `floor` and `governed` columns (phase-00.md:208-209); phase 04 cr_material (571-575); scope config.author (API.md:95).
- T7 Consistency: PASS - floors of 7 years on R2, R3, R5 to R8 (391-403) match DR-186 (1044) and 3.10 (769). R4 floor 1 (395) matches Open doubts 6 (1083).
- Gaps:
  - `GAP V-23-08-a | HIGH | T1 | The class-suffixed period keys fail phase 00's settings key CHECK (phase-00.md:204), so migration 0072 aborts at its first settings INSERT and no period is seeded. Check 4b (969) and check 5i (982) use the same keys | Section 1.2 settings INSERT (phase-23.md:388-415): lower-case the class suffix, or a phase 00 CHECK change in a 7.x block | grep -n "CHECK (key" docs/plan/phase-00.md: lines 190 (idempotency key), 204 (admin.settings), 218 (admin.feature_flags) only; grep -n "key ~" in phase-00.md 7.x blocks (831 onward): none; grep -n "period_years\|period_days" docs/plan/phase-*.md: phase-14.md:1351 and phase-23.md only.`

### V-23-09 Gap 7.1 deletion-certificate key name (akr-sign-deletion)   (source: 7.1 gap block; result: GAPS 1)
- T1 SQL: N/A - the block adds no SQL, and no DDL in phase 23 names the key.
- T2 Route: N/A - no route.
- T3 Repository and types: N/A - CertificateSigner (phase-23.md:501) is a port whose key name changes; no type changes.
- T4 Logic: PASS - the block states the new name and its policy paths (phase-23.md:1002-1005).
- T5 Audit: N/A - no event changes.
- T6 Cross-references: GAP - see a. PASS for phase 00 7.6, which defines `akr-sign-deletion` for deletion certificates and batch manifests and records the correction (phase-00.md:918-919), and `akr-sign-certificate` for coverage certificates (phase-00.md:918).
- T7 Consistency: PASS - the block names what it replaces ("Every mention (sections 2, 3 and 4.4, DR-194)", phase-23.md:1003). The base text still reads `akr-sign-cert` at 501, 680, 920 and 1060; the block wins.
- Gaps:
  - `GAP V-23-09-a | MEDIUM | T6 | Section 4.4 says the transit key is created "by akr-ctl ops provision" (phase-23.md:920). No such command exists: the only akr-ctl ops subcommand named in phase 00 or phase 02 is record-restore-drill | Section 4.4 (920) | grep -n "ops provision" docs/plan/*.md research.md: phase-23.md:920 only; grep -n "akr-ctl ops" docs/plan/phase-00.md docs/plan/phase-02.md: record-restore-drill (phase-00.md:277, 653, 821) only.`

### V-23-10 Gap 7.2 handlers for the tables of later phases (DR-387)   (source: 7.2 gap block; result: GAPS 2)
- T1 SQL: GAP - 7.2 says "from phase 29 on, an assess.decisions partition is dropped only when no assess.holds row references it" (phase-23.md:1017-1019). The drop allow-list (280-283) and the function's refusal of an unlisted parent (285) do not contain assess.decisions. The block adds no allow-list change and no DDL, see a.
- T2 Route: N/A - no route.
- T3 Repository and types: PASS - the registry type DisposalHandler is named (497); later phases add their handlers in the same release (1010-1011).
- T4 Logic: PASS - each class and clock rule is explicit (1010-1016): R3 and R8 owner rows in the party batch, R6, R7 and R11 report_only, R9 kept, R10 never, retirement never due, R12 by the owner's named sweep or record_purge. The R7 rule is contradicted in T7.
- T5 Audit: PASS - record_purge deletes fall under `retention.records_purged` (808).
- T6 Cross-references: PASS - notification_sweep (phase-22.md:785); assess_ledger_sweep job (phase-29.md:1320); alert.model_scores (phase-27.md:178, FK at 190-191); assess.decisions (phase-29.md:85); assess.holds (phase-29.md:274); DR-386 and DR-387 (phase-27.md:293).
- T7 Consistency: GAP - the block says "R6, R7 and R11 are report_only" (1011-1012), but phase 29 classes assess.decisions as R7 with a partition clock (phase-29.md:798), and 7.2 then drops its partitions (1017-1019). Phase 29 repeats the mismatch at phase-29.md:1502. See b.
- Gaps:
  - `GAP V-23-10-a | MEDIUM | T1 | The phase 29 drop step needs assess.decisions in the drop allow-list, which section 1.2 does not contain (phase-23.md:280-283; the function refuses unlisted parents at 285). Phase 29 relies on this (phase-29.md:1502). The block names neither an allow-list change nor DDL | Section 1.2 allow-list (280-283) or the 7.2 block | grep -n "assess.decisions" docs/plan/phase-23.md: line 1018 only, and it has no allow-list entry.`
  - `GAP V-23-10-b | MEDIUM | T7 | R7 is report_only in 7.2 (1011-1012) and in phase 29 (phase-29.md:1502: "the R7 tables are report_only"), yet assess.decisions is R7 with clock partition (phase-29.md:798) and is dropped under 7.2 (1017-1019). One class gives two rules for the same table | Section 7.2 (1011-1012 and 1017-1019), or the phase 29 row at 798 | grep -n "R7" docs/plan/phase-23.md: the R7 rule at 597 and 1011-1012 has no assess row; grep -n "'assess', 'decisions'" docs/plan/phase-29.md: line 798 only.`

### V-23-11 Gap 7.3 DELETE grants follow the handlers (DR-386)   (source: 7.3 gap block; result: PASS)
- T1 SQL: PASS - the DELETE grant (phase-23.md:374-381) covers every record_purge table in the registry: R3 party family (590) at 374-377, R8 KYC family (598) at 378-380, R1 admin.quarantine_items (587) at 377. Every granted table exists (phase-10.md:201-1381; phase-11.md:178; phase-15.md:108-443). alert.model_scores, the later record_purge table, has DELETE for akr_retention (phase-27.md:296), as the block requires.
- T2 Route: N/A - no route.
- T3 Repository and types: N/A - no types.
- T4 Logic: PASS - DELETE only for deleting handlers, and check 4a extended (1022-1025, 1024).
- T5 Audit: N/A - a grant is not a data mutation; the deletions are covered by 808.
- T6 Cross-references: PASS - DR-198 (1068) is the decision the block amends; phase 26 grants DELETE to akr_retention on its own tables (phase-26.md:118), consistent with the rule.
- T7 Consistency: PASS - 1025 states the replacement for DR-198's wording ("reads the deleting handlers only"), which resolves the base text at 1068.

### V-23-12 Adversarial 7.4 (run 4) a case hold follows merged_into   (source: adversarial block run 4, ADV-4-07; result: GAPS 2)
- T1 SQL: PASS - casework.cases.merged_into (phase-17.md:297) references casework.cases(id). casework.case_parties (372-374) and casework.case_alerts (352-363) exist. "Merges move rows (UPDATE case_id)" (phase-17.md:351).
- T2 Route: N/A - the matching runs in the engine; no route changes.
- T3 Repository and types: PASS - the case-hold matching uses LegalHoldRepository::active_case_holds() (phase-23.md:490) and CaseLookup (517) for parties and alerts. The path walk is a rule, not a separate method.
- T4 Logic: GAP - the rule itself is explicit (1031-1033). The test named in the block is missing from section 6, see b.
- T5 Audit: N/A - the matching writes nothing; copies and purges are audited (808 and 636).
- T6 Cross-references: PASS - "phase 17 section 3.6" is the merge section (phase-17.md:1076, "Merge and grouping (CaseMerger)").
- T7 Consistency: GAP - the purge's hold check does not follow the new matching, see a.
- Gaps:
  - `GAP V-23-12-a | HIGH | T7 | The purge's hold check (3.5 step 2) matches case holds only through the held case's own case_parties (phase-23.md:653). Line 649 defers to section 3.8, and 7.4 widens the 3.8 match to every case on the merge path (1031-1033). A party held only through a merged case is not seen by the purge and can be deleted under a hold. The block does not replace step 2 | Section 3.5 step 2 (653) | grep -n "case_parties contain" docs/plan/phase-23.md: line 653 only; grep -n "merged_into" docs/plan/phase-23.md: line 1031 only, so no purge step was changed.`
  - `GAP V-23-12-b | MEDIUM | T4 | The test named in the block ("hold case S, merge S into T: the next run still holds S's alerts in T", phase-23.md:1033-1034) is not in section 6. Section 6 ends at check 9 (995), and no check among 5a to 5r (974-991) covers a merge | Section 6 (962-995) | grep -n -i "merge" docs/plan/phase-23.md: lines 1027, 1029-1031, 1033 only.`

## Summary

- Items checked: 12 (V-23-01 to V-23-12).
- Items fully PASS: 1 (V-23-11).
- GAP lines by severity: HIGH 2, MEDIUM 14, LOW 11 (27 in total). V-23-04 T7 repeats V-23-12-a and is counted once, at V-23-12-a.

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-23-01-a | MEDIUM | T2 | ADM requests the dry run but has no audit.read; GET runs/{id} refused, Cedar permits it | Section 2 route row 532, or 3.14 ADM permit (867-870) |
| V-23-01-b | LOW | T2 | Idempotency-Key required by 5.1 and 5a, absent from the route row | Section 2 route row 530 |
| V-23-01-c | LOW | T3 | NewRun and RunSummary named, not defined | Section 2 (483-487, 512) |
| V-23-01-d | MEDIUM | T5 | Failed item marked with "an audit event", no event kind named | Section 3.12 event table (796-817) |
| V-23-01-e | MEDIUM | T5 | Dry-run request inserts a run and enqueues a job with no audit event | Section 3.12 table (796-817) or 3.1 (545) |
| V-23-02-a | MEDIUM | T6 | HoldView also a phase 29 utoipa schema with different fields | Section 2 utoipa list (519) |
| V-23-02-b | LOW | T3 | NewLegalHold undefined; HoldView fields not listed | Section 2 (490, 508) |
| V-23-02-c | LOW | T4 | reason_text 500-character limit has no step or error | Section 3.8 step 2 (720) |
| V-23-02-d | LOW | T6 | DTO cites section 3.2 for the reason scan (scan is 3.8 step 3) | Section 2 DTO (484) |
| V-23-03-a | MEDIUM | T3 | ShredOutcome undefined; repeat-shred outcome not stated | Section 3.6 (666) and port (500) |
| V-23-03-b | LOW | T6 | Phase 02 cites phase 23 section 1.1; classification row is in 1.2 | phase-02.md:1210 |
| V-23-04-a | MEDIUM | T1 | facility and trade_instrument have no account_id; party path wrong | Section 3.5 party path (647) |
| V-23-04-b | LOW | T1 | declared_profiles and review_queries have no party_id; "by party_id" claim | Section 3.5 (647) |
| V-23-04-c | LOW | T3 | RelationshipEndResolver has no signature | Section 2 services (507-517) |
| V-23-04-d | LOW | T6 | Open doubts 22 says no account_id index on canon.txn; idx_txn_account exists | Open doubts 22 (1099) |
| V-23-05-a | MEDIUM | T6 | Screening alerts with a party subject are not in the blocking list | Section 3.5 blocking references (649) |
| V-23-06-a | MEDIUM | T1 | feat.feature_runs treated as a partition_drop parent; it is not partitioned | Section 3.2 R4 row (592), allow-list (282) |
| V-23-06-b | LOW | T6 | WORM bucket not named (phase 02 COMPLIANCE bucket is akr-anchors) | Section 2 WormWriter (502), 4.4 (923) |
| V-23-07-a | MEDIUM | T2 | Report and runs routes need audit.read; Cedar permits ADM | Section 2 rows 529, 531, or 3.14 (867-870) |
| V-23-07-b | LOW | T3 | RetentionReportView has no field list | Section 2 (511, 519) |
| V-23-07-c | MEDIUM | T7 | Who computes the later-of count is stated two ways | Section 3.2 (580), Section 2 service deps (511) |
| V-23-08-a | HIGH | T1 | Period keys with an upper-case class code fail the phase 00 key CHECK; 0072 aborts | Section 1.2 settings insert (388-415) |
| V-23-09-a | MEDIUM | T6 | "akr-ctl ops provision" is not a defined command | Section 4.4 (920) |
| V-23-10-a | MEDIUM | T1 | assess.decisions is not in the drop allow-list | Section 1.2 allow-list (280-283) |
| V-23-10-b | MEDIUM | T7 | R7 is report_only in 7.2 but assess.decisions (R7) is dropped | Section 7.2 (1011-1012, 1017-1019), phase 29 row 798 |
| V-23-12-a | HIGH | T7 | Purge hold re-read (3.5 step 2) not updated for the 7.4 merge path | Section 3.5 step 2 (653) |
| V-23-12-b | MEDIUM | T4 | Merge test named in 7.4 is not in section 6 | Section 6 (962-995) |
