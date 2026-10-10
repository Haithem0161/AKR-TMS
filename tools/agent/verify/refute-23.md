# Refuter, phase 23 (Haiku draft for lead review)

### V-23-01-a  STANDS   (first agent: MEDIUM; refuter: HIGH)
- Claim: ADM requests the dry run but holds no audit.read, yet GET /retention/runs/{id} (scope audit.read) is the read that check 5a runs as adm@, while Cedar permits ADM retention.run.read.
- Evidence: phase-23.md:532 "`audit.read` | **added.** One run with its items" (route scope). API.md:100 "IA, SEC; M for their own scope" (the audit.read row has no ADM). phase-23.md:974 "`GET /api/v1/retention/runs/{id}` until `state = completed`" (check 5a as adm@). phase-23.md:869 "Akr::Action::"retention.report.read"], resource)" and phase-23.md:868 for the ADM permit on run.read. phase-03.md:685 "`AuthContext.scopes` must contain one of the route's scopes, else 403 `E_SCOPE`" (the scope gate runs before Cedar). Result: the route refuses ADM before Cedar, so check 5a cannot pass as written. Grep for ADM in the audit.read row of API.md section 2: none.
- Severity note: the two texts contradict and an acceptance check fails on the scope gate, so HIGH rather than MEDIUM.
- Fix: Section 2 route row 532 and check 5a (line 974): name the scope ADM holds (admin.system) or give ADM audit.read in API.md section 2, then make the ADM permit in section 3.14 (lines 867-870) agree.

### V-23-01-b  REFUTED   (first agent: LOW; refuter: none)
- Claim: the route row for POST /retention/dry-runs names no Idempotency-Key, yet section 5.1 and check 5a require one.
- Evidence: phase-00.md:514 "middleware on every POST, PUT, PATCH and DELETE under `/api/v1`" (general rule that every POST inherits). API.md:38 "Every POST, PUT, PATCH and DELETE requires `Idempotency-Key`". phase-23.md:960 "Every POST carries an `Idempotency-Key`" (section 5.3 states it for this phase). The row omits the marker that row 526 carries, but the rule that the requirement comes from is general.

### V-23-01-c  PARTLY   (first agent: LOW; refuter: LOW)
- Claim: NewRun (create input) and RunSummary (return of execute) are named and not defined; phase 16 uses the name NewRun for another type.
- Evidence: NewRun is refuted: phase-23.md:545 "inserts the row with `trigger = dry_run_request`, `dry_run = TRUE`, `requested_by` and `as_of`" and the retention_runs DDL (phase-23.md:75-90) state its fields, which the exemption for fields stated in prose accepts. RunSummary stands: phase-23.md:512 "`execute(run_id) -> RunSummary`" has no definition; its content appears only as "with the class totals" (phase-23.md:554) and "the summary carries `truncated = true`" (phase-23.md:553). phase-16.md:767 "start_once(&self, tx: &mut Tx, r: &NewRun)" is phase 16's own RunRepository, a name clash across modules, not a gap.
- Fix: Section 2 services (phase-23.md:512): one line naming the RunSummary fields (totals by class and the truncated flag), or say that it is the summary JSON of section 1.1.

### V-23-01-d  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: step 8 marks a failed item "with reason_code and an audit event", but no event kind for an item failure is named.
- Evidence: phase-23.md:555 "the affected item is `failed` with `reason_code` and an audit event". The event table (phase-23.md:796-817) has no item-failure kind; the only "failed" in it is in legal_hold.release_ended (line 802). phase-23.md:819 "Every mutation of this phase appears above" is therefore false for this one. The run-level exemption does not apply, because the item's own text asks for its own event. Greps that found nothing: grep -n "item_failed\|retention.item" over docs/plan/*.md (no hits).
- Fix: Section 3.12 event table (after line 817): add the item-failure row with its kind and payload keys, and name that kind in step 8 (line 555).

### V-23-01-e  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: POST /retention/dry-runs inserts a run row with requested_by and enqueues the job; the only run event, retention.run_started, is written at start, so the request has no audit event.
- Evidence: phase-23.md:545 "and enqueues `retention_run` with payload `{ run_id }`" (the request writes a row and a job, with no event named). phase-23.md:804 "`retention.run_started` | start | `retention_run` | `run_id`, `trigger`, `dry_run`, `as_of`" (no requested_by in the payload). phase-23.md:819 "Every mutation of this phase appears above". API.md:121 "writes the audit event named in its phase file" (the rule every mutation must meet). Greps: the dry_run_request hits at lines 77, 79, 87, 88, 353, 545 and 902 include no event row.
- Fix: Section 3.12 event table (after line 817): add the request event with run_id and requested_by in its payload, and name it in section 3.1 (line 545).


### V-23-02-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: HoldView is also a utoipa schema of phase 29 with different fields, and phase 29 is served by akr-api, so the OpenAPI names collide.
- Evidence: phase-29.md:867 "HoldView { id, hold_ref, rail, mode, kind, state, reason_codes, candidates, proposal, expires_at, party_names }" (phase 29 utoipa list). phase-29.md:9 "akr-api (router `assess` with the 14 people routes". phase-23.md:519 "**utoipa schemas:** `PlaceLegalHold`, `ProposeHoldRelease`, `HoldView`". Both schemas sit in the same akr-api document. Grep for HoldView over docs/plan, docs and research: phase-23.md:508 and 519, and phase-29.md:867 only.
- Fix: Section 2 utoipa list (phase-23.md:519) and the place response (line 726): rename the hold view schema in phase 23 (for example LegalHoldView), because the name is taken in akr-api by phase 29.

### V-23-02-b  REFUTED   (first agent: LOW; refuter: none)
- Claim: NewLegalHold (repository insert input) and the fields of HoldView are not defined; line 750 gives only the omitted fields.
- Evidence: phase-23.md:723 "`state = active`, `version = 1`, `placed_by = actor`" and the DDL at phase-23.md:41-72 name the insert fields of NewLegalHold. phase-23.md:474 "`LegalHold { id, hold_ref, kind: HoldKind, party_id: Option<Uuid>, case_id: Option<Uuid>, range:" gives the base fields of HoldView. phase-23.md:508 says the scope, the party or case id and the reason text are fields that the view builder omits for principals without the right, and phase-23.md:750 "M sees everything, including the kind, the party or case id and the sealed reason text". Fields stated in prose and in code blocks are covered by the exemption.

### V-23-02-c  PARTLY   (first agent: LOW; refuter: LOW)
- Claim: the 500-character limit on reason_text has no step in 3.8 and no error code; step 2 lists only the field combination and the existence checks.
- Evidence: the limit is stated on the DTO: phase-23.md:484 "(`reason_text` at most 500 characters; scanned, section 3.2)". The error code is refuted: phase-23.md:784 "`Invalid` | `E_VALIDATION` (422) | shape, range, reason scan" (section 3.11) covers a shape violation. What stands: no step of 3.8 names the length check (step 2, phase-23.md:720, "Validate the body: the field combination must match `kind` (`E_VALIDATION`)"). Grep for "500" in phase-23.md finds line 484 only.
- Fix: Section 3.8 step 2 (phase-23.md:720): add the 500-character check, returning E_VALIDATION.

### V-23-02-d  STANDS   (first agent: LOW; refuter: LOW)
- Claim: the DTO says the text is "scanned, section 3.2"; the scan is in section 3.8 step 3, and section 3.2 is classes and clocks.
- Evidence: phase-23.md:484 "scanned, section 3.2"; phase-23.md:564 "### 3.2 Classes, floors, clocks and the disposal registry"; phase-23.md:721 "3. `reason_text`, when given, is scanned with `PolicyPackService::scan`". Grep for "scanned" in phase-23.md finds lines 484 and 721 only.
- Fix: Section 2 DTO (phase-23.md:484): change "section 3.2" to "section 3.8".

### V-23-03-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: ShredOutcome, the return of shred, is not defined and its values are not listed; the text does not say what a repeat shred returns or whether shredded_at changes.
- Evidence: phase-23.md:500 "`shred(tx, evidence_id, reason) -> ShredOutcome`" and phase-02.md:1202 "`shred(tx, evidence_id, reason) -> ShredOutcome`", which points to "(phase 23 section 2 ports)" at line 1203. Neither defines the type. phase-23.md:666 "sets `shredded_at = now()`" applies to every call. A repeat call is reachable: phase-23.md:670 covers "held-row copies after their grace period", and the phase 02 guard at phase-02.md:406 "IF OLD.shredded_at IS NOT NULL AND NEW.wrapped_dek IS NOT NULL THEN" does not protect shredded_at itself. Grep for ShredOutcome over docs/plan, docs and research: phase-23.md:500 and phase-02.md:1202 only.
- Fix: Section 2 port (phase-23.md:500): list the ShredOutcome values and say in one sentence what a repeat shred returns and whether shredded_at changes.

### V-23-03-b  STANDS   (first agent: LOW; refuter: LOW)
- Claim: phase 02 7.3 cites "phase 23 section 1.1" for the classification row of object_deleted_at; that row is in section 1.2.
- Evidence: phase-02.md:1210 "the one phase 23 section 1.1 defines (migration 0072 adds the column". phase-23.md:189 "### 1.2 `0072_retention_guards_grants.sql`" and phase-23.md:449 "('evidence_objects', 'internal', FALSE, FALSE, ARRAY['object_deleted_at'])". Section 1.1 (lines 28-187) is migration 0071 and has no evidence_objects row. Phase 23 is right and the citation is wrong.
- Fix: Phase 02 section 7.3 (phase-02.md:1210): change "section 1.1" to "section 1.2".


### V-23-04-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: canon.facility and canon.trade_instrument have no account_id, but section 3.5 gives their party path as "through account_id", and a row with no path is never purged.
- Evidence: phase-23.md:647 "`facility`, `trade_instrument` (through `account_id`)". phase-10.md:1312 "-- no foreign key: customer is bi-temporal (section 2)" (facility carries customer_id and no account_id). phase-10.md:1337 and 1339 (trade_instrument: customer_id, and "parties JSONB NOT NULL DEFAULT '[]'::jsonb"). Grep for account_id in phase-10.md lines 1310-1358: no hit for either table. Grep for a later ALTER or ADD COLUMN on canon.facility or canon.trade_instrument in docs/plan: none.
- Severity note: a developer sees the column is missing and can infer customer_id, so MEDIUM rather than HIGH.
- Fix: Section 3.5 party path (phase-23.md:647): give customer_id (through canon.customer.party_id) for facility and trade_instrument in place of account_id.

### V-23-04-b  REFUTED   (first agent: LOW; refuter: none)
- Claim: kyc.declared_profiles and kyc.review_queries have no party_id, yet section 3.5 says the KYC family is purged "by party_id"; no path is written for them.
- Evidence: phase-23.md:576 "the row is deleted in the same transaction as its owner, in the owner's batch (section 3.5), and never before it" (the owner clock rule, which covers any child row). phase-15.md:859 "Follows its review and party" (review_queries, R8 owner). phase-15.md:861 "follows the customer" (declared_profiles, R3 owner). Both children are deleted in the batch of their owner, so the party path is the owner's, and phase-15.md:316 and 356 carry review_id to kyc_reviews.

### V-23-04-c  PARTLY   (first agent: LOW; refuter: LOW)
- Claim: RelationshipEndResolver computes the party end date and is named only in the 3.5 heading; it has no signature in section 2.
- Evidence: the computation is specified in prose: phase-23.md:645 "The end date is `greatest(max(canon.customer.valid_to), max(canon.account.closed_on))` over the party's rows." That part is refuted. What stands: section 2 services (phase-23.md:507-517) do not name the resolver or its signature. Greps for RelationshipEndResolver over docs/plan, docs and research: phase-23.md:643, roadmap.md:174 and status.md:38 (inventory rows only, no signature).
- Fix: Section 2 services (after phase-23.md:517): one line naming RelationshipEndResolver, its input (party id) and its output (end date).

### V-23-04-d  PARTLY   (first agent: LOW; refuter: LOW)
- Claim: Open doubts 22 says canon.txn has no index on account_id in phase 10; phase 10 has idx_txn_account, so the need is already met.
- Evidence: phase-10.md:1120 "ON canon.txn (account_id, booking_date DESC)" (index idx_txn_account exists), so the need is met and that part is refuted. What stands: phase-23.md:1099 "`canon.txn` has no index on `account_id` in phase 10 as far as this draft reads, and the account purge needs one" is stale text.
- Fix: Open doubts 22 (phase-23.md:1099): delete the index sentence, since phase 10 line 1120 carries the index.

### V-23-05-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: alert.screening_alerts carries a party subject and is R7 and kept, but it is not in the closed blocking list of section 3.5, so a purged party leaves a kept record that points at a missing party.
- Evidence: phase-14.md:254 "CHECK (subject_kind IN ('party', 'counterparty', 'payment_party'))"; phase-14.md:255 "no foreign key: canonical tables are bi-temporal (DR-87)". phase-23.md:597 "`report_only` | `alert.screening_*`" (R7, kept). phase-23.md:649 "The list is closed in this phase" (four tables: alert.alerts, casework.cases, casework.case_parties, rpt.reports). The texts disagree: phase-23.md:1052 "A party named by an alert, a case, a case party or a report is deferred", which can be read to cover screening alerts. phase-23.md:1099 (Open doubts 22) defers exactly this column check. phase-14.md:628 "Run records; counts only, no subject names outside alerts" shows that alerts carry subject data. Greps: "screening" in phase-23.md has no blocking rule; no hit for screening_alerts in research/*.md.
- Fix: Section 3.5 blocking references (phase-23.md:649): add alert.screening_alerts for subject_kind party, or state in one sentence what happens to its subject link when the party is purged; then close Open doubts 22.

### V-23-06-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: feat.feature_runs is a plain table with a primary key and no partitioning, but the R4 row treats it as partition_drop and the drop allow-list names it; the function returns not_partition and the item fails.
- Evidence: phase-13.md:158 "CREATE TABLE IF NOT EXISTS feat.feature_runs (" with phase-13.md:167 "PRIMARY KEY (as_of, feature_id)," and no PARTITION BY. The PARTITION BY lines (184, 203, 219) belong to feature_values, segment_stats and profiles. phase-23.md:592 "`partition_drop` | `feat.feature_values`, `feat.feature_runs`" and phase-23.md:282 "'feat.feature_values', 'feat.feature_runs', 'feat.segment_stats',". phase-13.md:370 gives feature_runs the clock partition on as_of, which presumes partitions the table does not have.
- Correction to the claim: the planner enumerates partitions, and phase-23.md:571 "a partition is due when its exclusive upper bound is on or before the cut-off" finds none for this table, so the function is never called and no failed item is written. The effect is silent: the R4 rule never runs for feature_runs. The table is small and nothing is deleted early, which is why the severity stays MEDIUM.
- Fix: Section 3.2 R4 row (phase-23.md:592): give feat.feature_runs a row-level disposal by as_of, or have phase 13 partition the table; name one of the two.

### V-23-06-b  STANDS   (first agent: LOW; refuter: MEDIUM)
- Claim: the WORM bucket is named only as "the COMPLIANCE bucket of phase 02" and "the WORM bucket"; phase 02 names akr-anchors and has no certificates/ prefix; the S3 grant names no bucket.
- Evidence: phase-02.md:1032 "bucket default retention COMPLIANCE 2,555 days on `akr-evidence`" and phase-02.md:1033 "COMPLIANCE 3,650 days on `akr-anchors`". Phase 02 has two COMPLIANCE buckets, so "the COMPLIANCE bucket of phase 02" does not name one. phase-23.md:502 "`WormWriter` (the COMPLIANCE bucket of phase 02, under the prefix `certificates/`)". phase-23.md:923 "and `s3:PutObject` under `certificates/`" names no bucket. The 3650-day lock matches akr-anchors, which is an inference and not a statement. The certificates/ prefix is used as an S3 key only in phase 23; phase-12 and API.md use it for a REST route.
- Severity note: a grant without a bucket name is a missing grant target, so MEDIUM rather than LOW.
- Fix: Section 2 WormWriter (phase-23.md:502) and section 4.4 S3 bullet (phase-23.md:923): name the one bucket (akr-anchors or akr-evidence) in both places.


### V-23-07-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: GET /retention/report and GET /retention/runs carry audit.read, which ADM lacks, but Cedar permits ADM retention.report.read and retention.run.read.
- Evidence: phase-23.md:529 "| GET | `/api/v1/retention/report` | `audit.read` |" and phase-23.md:531 "| GET | `/api/v1/retention/runs` | `audit.read` |". API.md:100 "IA, SEC; M for their own scope". phase-23.md:869 "Akr::Action::"retention.report.read"], resource)" (ADM permit). The scope gate runs before Cedar (phase-03.md:685), so the two ADM permits can never be reached. phase-23.md:834 "Scope gates are API.md section 2". Impact is dead policy text: no check in section 6 calls either route as adm@ (check 5a uses the single-run read, see V-23-01-a).
- Fix: Section 2 rows 529 and 531 and the ADM permit (phase-23.md:867-870): settle this with V-23-01-a, one rule for the three ADM reads.

### V-23-07-b  REFUTED   (first agent: LOW; refuter: none)
- Claim: RetentionReportView is in the utoipa list with no field list; its fields exist only in prose.
- Evidence: phase-23.md:771 "Returns the latest completed run (dry or real), not a live count: `as_of`, the run id, `dry_run`, and per class: `eligible`, `deleted` (zero for a dry run), `deferred`, `blocked` and `held`." The same paragraph gives the later-of count and the earliest due date. Prose field lists are covered by the exemption.

### V-23-07-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: who computes the later-of count is stated two ways: section 3.2 gives it to RetentionReportService, whose declared dependencies have no port to the report tables, while section 3.10 has the run summary carry it.
- Evidence: phase-23.md:580 "`RetentionReportService` applies this rule (section 3.10)." phase-23.md:511 "`RetentionReportService { runs, items, clock }` (`akr-api`)" has no port to rpt.reports, rpt.filings or casework.cases. phase-23.md:771 "the summary also gives the number of reports due under the later-of rule". The report reads the latest completed run, so the run must compute the count. Grep for "later-of" lists no sentence that names the component computing it at run time (lines 580, 596, 771, 949, 1054, 1111 only).
- Fix: Section 3.2 (phase-23.md:580): state that the engine computes the later-of count into the run summary, and that section 3.10 reads it from there.

### V-23-08-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: the class-suffixed period keys fail phase 00's settings key CHECK, so migration 0072 aborts at its first settings insert and no period is seeded.
- Evidence: phase-00.md:204 "CHECK (key ~ '^[a-z][a-z0-9_]*(\.[a-z0-9_]+)+$')" refuses the upper-case class code. phase-23.md:389 "('retention.period_days.R1', '90', 'integer'," and phase-23.md:391 "('retention.period_years.R2', '10', 'integer'," are the first rows of the INSERT at line 388; ON CONFLICT does not catch a CHECK violation. No 7.x block alters the CHECK: grep "CHECK (key" over phase-00.md finds lines 190, 204 and 218 only. The same key appears in phase-14.md:1351 "Where OFAC applies, the bank sets `retention.period_years.R7`", so phase 14 is affected too. Checks 4b (phase-23.md:969) and 5i (phase-23.md:982) use the same keys.
- Fix: Section 1.2 settings insert (phase-23.md:388-415) and phase 14 section 7.8 (phase-14.md:1351): lower-case the class suffix in every such key, or add a CHECK change for the key pattern in a phase 00 7.x block.

### V-23-09-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: section 4.4 says the transit key is created "by akr-ctl ops provision", and no such command exists.
- Evidence: phase-23.md:920 "created with the phase 02 anchor key by `akr-ctl ops provision`". The only ops subcommand of phase 00 is phase-00.md:630 "`ops record-restore-drill --result passed|failed --evidence <file>`" (phase 02 line 1146 names the same one; backfill-freezes and record-failover-drill come from phases 20 and 30). Grep for "ops provision" over docs/plan, docs and research: phase-23.md:920 only. The gap is wider than the name: no step in phase 00 or phase 02 creates any transit key; phase-00.md:590 "transit keys `akr-kek` (aes256-gcm96, wraps DEKs)" is a layout list with no creation step.
- Fix: Section 4.4 (phase-23.md:920): name the step that creates akr-sign-anchor and akr-sign-deletion (a provisioning command or a key-ceremony step in phase 00 section 3), in one sentence.

### V-23-10-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the phase 29 drop step needs assess.decisions in the drop allow-list, which section 1.2 does not contain; the function refuses unlisted parents, and the 7.2 block names no allow-list change or DDL.
- Evidence: phase-23.md:280 "IF NOT (p_parent IN ('canon.txn'" opens the allow-list (lines 280-283), which has no assess entry. phase-23.md:285 "RAISE EXCEPTION 'retention: % is not a droppable parent', p_parent". phase-29.md:1502 "`assess.decisions` partitions leave through `admin.retention_drop_partition`" and phase-29.md:1528 "Partitions leave only through `admin.retention_drop_partition` (DR-187)" rely on the function accepting the table. Grep for assess.decisions in phase-23.md: line 1018 only, with no allow-list change. Phase 29 names the function at lines 798, 1502 and 1528 and names no allow-list change.
- Fix: Section 7.2 (phase-23.md:1017-1019): add that the phase 29 migration re-creates admin.retention_drop_partition with assess.decisions in its allow-list, in one sentence; or add the entry in section 1.2 (phase-23.md:280-283).

### V-23-10-b  PARTLY   (first agent: MEDIUM; refuter: LOW)
- Claim: R7 is report_only in 7.2 and in phase 29, yet assess.decisions is R7 with a partition clock and is dropped under 7.2; one class gives two rules for one table.
- Evidence: the two rules are one general rule and its named exception, so the contradiction is refuted. phase-23.md:1012 "R7 and R11 are `report_only` (DR-191)" is the general rule. The exception sits in the same block at phase-23.md:1018-1019 "from phase 29 on, an `assess.decisions` partition is dropped only when no `assess.holds` row references it". phase-29.md:1502 names the same exception ("the R7 tables are `report_only`; `assess.decisions` partitions leave through"), and phase-29.md:1528 (DR-313) says "decisions R7 by partition". What stands (LOW): the general R7 sentence (phase-23.md:1012) and the R7 class row (phase-23.md:597, "`report_only`") carry no pointer to the exception, so a reader who builds handlers from the class alone gets report_only.
- Fix: Section 7.2 (phase-23.md:1012): add "except `assess.decisions` (below)" to the R7 sentence, and the same note to the R7 row at phase-23.md:597.


### V-23-12-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: the purge hold check (section 3.5 step 2) matches case holds only through the held case's own case_parties; the 7.4 block widens the section 3.8 match to every case on the merge path; a party held only through a merged case can be deleted under a hold.
- Evidence: phase-23.md:653 "re-read the holds for the party (party holds on it, case holds whose `case_parties` contain it)". phase-23.md:1031-1032 "follows `merged_into` from the held" and "case to its end and holds, at the run, the parties of `casework.case_parties`" (7.4, the sentence wraps). phase-17.md:351 "Merges move rows (UPDATE case_id); rows are never deleted." After a merge the source case's case_parties rows belong to the target, so step 2 finds no hold for them. phase-23.md:649 "A party under a hold is deferred with reason `legal_hold` (section 3.8)" points to 3.8, but step 2 restates a narrower rule, and no 7.x block says that 3.8 wins for step 2. Grep for "merged_into" in phase-23.md: line 1031 only, so no purge step was changed.
- Fix: Section 3.5 step 2 (phase-23.md:653): restate the case-hold match as in section 3.8 with the merged_into path of 7.4.

### V-23-12-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the test named in the 7.4 block ("hold case S, merge S into T ...") is not in section 6; section 6 ends at check 9 and no check among 5a to 5r covers a merge.
- Evidence: phase-23.md:1033-1034 "Test: hold case S, merge S" and "into T: the next run still holds S's alerts in T." (the sentence wraps). phase-23.md:995 "9. Existing tests still pass." is the last check of section 6. Grep -n -i merge over phase-23.md: lines 1027, 1029, 1030, 1031 and 1033 only, all in 7.4 and its heading. The test is specified in the block, so the gap is the missing checklist entry.
- Fix: Section 6 (after check 9, phase-23.md:995): add one check for the merged-case hold, using the test sentence of 7.4.

## Summary

- REFUTED: 4 (V-23-01-b, V-23-02-b, V-23-04-b, V-23-07-b)
- STANDS: 18
- PARTLY: 5 (V-23-01-c, V-23-02-c, V-23-04-c, V-23-04-d, V-23-10-b)
- Gap IDs checked: 27 (V-23-01-a to V-23-12-b). Severity for STANDS: HIGH 3 (V-23-01-a, V-23-08-a, V-23-12-a), MEDIUM 13, LOW 2 (V-23-02-d, V-23-03-b). Severity changed from the first agent in: V-23-01-a (MEDIUM to HIGH), V-23-06-b (LOW to MEDIUM), V-23-10-b (MEDIUM to LOW, partly). V-23-06-a keeps MEDIUM but its mechanism is corrected in the block.

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-23-01-a | STANDS | HIGH | ADM lacks audit.read on the single-run read that check 5a uses; Cedar permits ADM; gate 1 refuses first | Section 2 row 532 and check 5a (974); section 3.14 ADM permit (867-870) |
| V-23-01-c | PARTLY | LOW | RunSummary named (512) with no definition; its fields only in prose (553-554) | Section 2 services (512) |
| V-23-01-d | STANDS | MEDIUM | failed-item audit event named in step 8 but absent from the event table | Section 3.12 table (after 817); step 8 (555) |
| V-23-01-e | STANDS | MEDIUM | POST /retention/dry-runs writes a run row and job with no named audit event | Section 3.12 table; section 3.1 (545) |
| V-23-02-a | STANDS | MEDIUM | HoldView name clashes with phase 29's HoldView in the same akr-api document | Section 2 utoipa list (519) and place response (726) |
| V-23-02-c | PARTLY | LOW | the 500-character limit has no step in 3.8 (its error code is E_VALIDATION by 3.11) | Section 3.8 step 2 (720) |
| V-23-02-d | STANDS | LOW | DTO cites section 3.2 for the reason scan; the scan is in 3.8 step 3 | Section 2 DTO (484) |
| V-23-03-a | STANDS | MEDIUM | ShredOutcome values undefined; repeat-shred result and shredded_at behaviour not stated | Section 2 port (500); section 3.6 (666) |
| V-23-03-b | STANDS | LOW | phase 02 7.3 cites phase 23 section 1.1 for the classification row (it is 1.2) | Phase 02 section 7.3 (1210) |
| V-23-04-a | STANDS | MEDIUM | facility and trade_instrument have no account_id; the path must be customer_id | Section 3.5 party path (647) |
| V-23-04-c | PARTLY | LOW | RelationshipEndResolver has no section 2 entry or signature (its rule is in 645) | Section 2 services (after 517) |
| V-23-04-d | PARTLY | LOW | Open doubts 22 keeps a stale "no index" sentence; phase 10 has idx_txn_account | Open doubts 22 (1099) |
| V-23-05-a | STANDS | MEDIUM | kept R7 screening alerts with a party subject are outside the closed blocking list | Section 3.5 blocking references (649); Open doubts 22 |
| V-23-06-a | STANDS | MEDIUM | feat.feature_runs is not partitioned, yet its R4 row says partition_drop; the rule never runs | Section 3.2 R4 row (592), or phase 13 partitions the table |
| V-23-06-b | STANDS | MEDIUM | WORM bucket not named; phase 02 has two COMPLIANCE buckets; the S3 grant names no bucket | Section 2 WormWriter (502); section 4.4 S3 bullet (923) |
| V-23-07-a | STANDS | MEDIUM | report and run-list scopes exclude ADM while Cedar permits ADM (dead policy text) | Section 2 rows 529 and 531; section 3.14 (867-870) |
| V-23-07-c | STANDS | MEDIUM | the owner of the later-of count is stated two ways | Section 3.2 (580); section 3.10 (771) |
| V-23-08-a | STANDS | HIGH | upper-case class keys fail the phase 00 settings CHECK; migration 0072 aborts | Section 1.2 settings insert (388-415); phase 14 section 7.8 (1351) |
| V-23-09-a | STANDS | MEDIUM | "akr-ctl ops provision" is not a command, and no step creates the transit keys | Section 4.4 (920) |
| V-23-10-a | STANDS | MEDIUM | assess.decisions is not in the drop function's allow-list | Section 7.2 (1017-1019) or section 1.2 (280-283) |
| V-23-10-b | PARTLY | LOW | the general R7 sentence has no pointer to the assess.decisions exception | Section 7.2 (1012); R7 row (597) |
| V-23-12-a | STANDS | HIGH | purge hold check (step 2) does not follow the merged_into path of 7.4 | Section 3.5 step 2 (653) |
| V-23-12-b | STANDS | MEDIUM | the merge test named in 7.4 is missing from section 6 | Section 6, after check 9 (995) |
