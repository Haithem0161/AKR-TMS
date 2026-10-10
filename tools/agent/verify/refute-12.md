# Refuter, phase 12 (Haiku draft for lead review)

### V-12-01-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Step 8 upserts recon_break rows in dq_issues with no audit event. The only register event, dq.recorded, is defined per batch, and a day run has no batch.
- Evidence:
  - docs/plan/phase-12.md:679 "Every `open` or `source_anomaly` result upserts one `dq_issues` row" (step 8; it says the upsert is the one of section 3.3).
  - phase-12.md:747-748 "Audit: one `dq.recorded {source, batch_id, kinds: {kind: count}}` per batch with issues (counts only)"; the event needs a batch_id, which a day run does not have.
  - phase-12.md:683 "`recon.completed {run_id, rungs: outcome counts}`" is the nearest event. It carries outcome counts and names no register row or issue.
  - Greps: the event name dq.recorded appears at phase-12.md:747 and 974 only (1125 is a metric name). The rows are the steward's register (owner, status, due_at at phase-12.md:183), so the housekeeping exception does not apply.
- Fix: Section 3.1 step 8 (phase 12): name the event for a day run's register writes, either a dq.recorded keyed by run_id or a sentence that recon.completed covers them.

### V-12-01-b  REFUTED   (first agent: LOW; refuter: none)
- Claim: The claim to running (step 1) and set_waiting to awaiting_totals (step 3) change recon_runs.status and name no audit event. The run events are requested, totals_captured, completed and failed.
- Evidence:
  - phase-12.md:489 "Conditional claim: the row moves to `running` only from `queued` or `awaiting_totals`; None when another worker has it." The claim is the job's own conditional update, the bookkeeping the housekeeping rule covers.
  - The run is bracketed by named events: phase-12.md:642 "audit `recon.requested`" at the start; phase-12.md:650-651 "is failed with `totals_unavailable` (audit `recon.failed`)" for a wait that times out; phase-12.md:713 "Audit `recon.totals_captured {run_id, objects, groups, missing}`" when totals arrive; phase-12.md:683 "`recon.completed {run_id, rungs: outcome counts}`" and phase-12.md:973 "`recon.completed`, `recon.failed`" at the end.
  - The row keeps the moves in time: phase-12.md:84 "started_at" and phase-12.md:80 "totals_captured_at" (both TIMESTAMPTZ).
  - phase-00.md:654 "`job.dead` | a job reaching `dead`" is the only job-state event the harness audits.
- Why refuted: under the housekeeping rule, job-state moves that are covered by run-level events need no event of their own.

### V-12-02-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: "Signed by the bank" has no test. The document kind or flag that proves the signature is not named. Phase 05 defines SourceDocumentKind values budget_signed, access_approval and exception_signed.
- Evidence:
  - Refuted part (no test): phase-12.md:1274 "with 30 days and a bank-signed" and phase-12.md:1275 "document -> 202 with a change request id". Section 6 has a known-adjustment test that uses a bank-signed document; the first agent's grep for "signed by" missed this wording.
  - Standing part (kind or flag): phase-12.md:789 "is signed by the bank (else 422 reason `document_required`)" names no kind or flag. Greps: "exception_signed" and "budget_signed" have no hits in phase-12.md. phase-12.md:193 "signed_document_id UUID REFERENCES admin.evidence_objects(id)" has no kind check. phase-05.md:503 "`SourceDocumentKind` (`budget_signed`, `access_approval`, `exception_signed`," gives the values; phase-05.md:1223 names the kind for the budget case ("kind `budget_signed`"), while the known-adjustment case names none.
- Fix: Section 3.5 step 3 (phase 12): name the SourceDocumentKind, or the flag, that a known-adjustment document must carry to count as signed by the bank.

### V-12-02-b  STANDS   (first agent: LOW; refuter: LOW)
- Claim: The proposal calls ChangeRequestService::submit with object kind, object key and body. Phase 04 submit takes an existing request id and If-Match. Creation with kind and key is create(origin = Dedicated) with submit: true.
- Evidence: phase-12.md:790 "`ChangeRequestService::submit` with object kind `known_adjustment`, object key `dq_issue:<id>`, the proposal in the body." docs/plan/phase-04.md:858 "pub async fn create(&self, ctx: &AuthContext, req: CreateChangeRequest, origin: Origin)"; phase-04.md:859 "pub async fn submit(&self, ctx: &AuthContext, id: Uuid, if_match: i32)"; phase-04.md:881 "Dedicated routes and `submit: true` continue with `submit` in the same transaction." Greps: "ChangeRequestService::create" and "submit: true" have no hits in phase-12.md.
- Fix: Section 3.5 step 4 (phase 12): name create(origin = Dedicated) with submit: true, as phase 04 section 3.2 step 6 gives it, in place of submit with kind, key and body.

### V-12-03-a  STANDS   (first agent: LOW; refuter: LOW)
- Claim: Same naming defect for the exclusion proposal: submit is named with kind exclusion and key exclusion:<source>:<unit>:<unit_ref>; the call that takes kind and key is create(origin = Dedicated) with submit: true.
- Evidence: phase-12.md:814 "`ChangeRequestService::submit` with object kind `exclusion`, object key `exclusion:<source>:<unit>:<unit_ref>`." phase-12.md:815 names the dedicated route ("phase 04 dedicated routes") but not the call. phase-04.md:858, 859 and 881 as in V-12-02-b. Grep "ChangeRequestService::create" in phase-12.md: no hits.
- Fix: Section 3.6 step 3 (phase 12): name create(origin = Dedicated) with submit: true, as phase 04 section 3.2 step 6 gives it.

### V-12-04-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: card_guard_not_excludable applies to "a card reason", but no reason code is named for a card row. Phase 11 quarantine rows are row_failed or row_conflict only, and phase 10 stops a PAN before landing, so the refusal has no defined trigger.
- Evidence:
  - phase-12.md:948 "`card_guard_not_excludable` for a card reason (card data is fixed at source)" (section 3.11). phase-12.md:1237 expects "exclusion of a `txn_type` gate row and of a `card_guard` row is refused", a quarantine row that no step creates.
  - The gate has a named reason: phase-11.md:1488 "reason `unmapped_code` with field". The card refusal has none.
  - phase-11.md:186 "kind TEXT NOT NULL CHECK (kind IN ('row_failed', 'row_conflict'))". Row failures take reason codes from phase 10: phase-11.md:797 "(transform error, required attribute missing, unmapped currency: reason codes from phase 10)".
  - phase-10.md:1867 "in phase 11 it stops the field's load and opens a data incident"; a card-guard stop opens a dq issue (phase-12.md:740 "A card-guard stop -> `card_guard` (critical)"), not a quarantine row.
  - phase-12.md:1348 (Decisions, DR-96) names "card reasons" as refused, but gives no code.
  - Greps: no card reason_code value appears in phase-10.md or phase-11.md.
- Fix: Section 3.11 (phase 12): name the reason code that a card-guard quarantine row carries, or state that a card-guard stop writes no quarantine row and drop the card refusal and the test row at phase-12.md:1237.

### V-12-04-b  STANDS   (first agent: LOW; refuter: LOW)
- Claim: Section 6 expects "open or retried to assigned". No status enum or CHECK has an assigned value, and section 3.11 names no status change for assign.
- Evidence: phase-12.md:1236-1237 "Quarantine transitions (table test): ... `open` or `retried` to `assigned`". phase-12.md:476 "`QuarantineStatus` (`Open`, `Retried`, `Accepted`, `Excluded`)". phase-11.md:190 "CHECK (status IN ('open', 'retried', 'accepted', 'excluded'))". phase-12.md:943-944 "the row must be `open` or `retried`" names the precondition only. Greps: "assigned" as a status appears only at phase-12.md:1237; phase-11.md has no "assigned" at all.
- Fix: Section 6, quarantine transitions test (phase 12, lines 1236-1237): state the assign case as an owner change that keeps the status open or retried, as section 3.11 implies.

### V-12-05-a  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: The freshness snapshot type DataAsOf, returned by FreshnessService::data_as_of, is not defined in any phase file, and its fields appear only in a comment. BranchDate is also undefined.
- Evidence:
  - phase-12.md:567 "fn data_as_of(&self, source_id: Uuid) -> DataAsOf;   // per branch business date, newest posting, stale flag". The fields are named in a code block, and the rule for what is not a gap says a full struct is not required when the fields are named.
  - phase-11.md:570 "/// data_as_of: the newest business date of a mapped batch per branch." names BranchDate's content in the code block that uses it.
  - phase-11.md:1009 "The business date of the newest **mapped** receipt of each branch" states the same content in prose (section 3.7).

### V-12-07-a  PARTLY   (first agent: HIGH; refuter: LOW)
- Claim: 7.2 says a restore-gap run is valid as it stands and that a date not reloaded shows as a break. 3.1 step 1 refuses a day run with no batch (409 no_batches) and a source that is not active (409 source_not_active). Phase 11 halts a source on restore suspicion, and phase 30 queues one run per gap date through the POST path.
- Evidence:
  - Refuted part (the refusals reach the restore path): the step 1 refusals belong to the route: phase-12.md:635 "**`POST /recon/runs`** (akr-api; Cedar `recon.run`; `Idempotency-Key`)" and phase-12.md:636 "1. Validate: `source_id` exists and is `active`". The restore check does not call the route: phase-30.md:283 "It runs on the restored database before the API is opened"; phase-30.md:288 "queues one phase 12 `recon_run` job per source and date (the job behind `POST /recon/runs`)"; phase-30.md:599 (DR-329) repeats that it "runs before the API opens".
  - The job refuses nothing on a no-batch date: phase-12.md:659 "An empty scope -> `source_anomaly`, `no_batches`." The job's steps (phase-12.md:655-685) check no source state except a governor pause on error (step 10).
  - Phase 11 does halt the source (phase-11.md:926-927 "`source.state_changed {from: active, to: halted, cause}`"), but the job does not read that state, so the halt does not block the run.
  - Standing part (LOW): phase-12.md:1312 "shows as a break" disagrees with phase-12.md:659 (source_anomaly). Section 7.2 (phase-12.md:1309-1313) does not say which path the restore check takes, so a reader who uses the route would hit the 409s.
- Fix: Section 7.2 (phase 12): state that the restore check enqueues the recon_run job directly, without the section 3.1 step 1 validation, and that a date with no batches ends as source_anomaly, no_batches.

### V-12-08-a  REFUTED   (first agent: LOW; refuter: none)
- Claim: Phase 11 still says the operator-planned backfill of API.md 3.5 (phase 12) exists for explicit slices (phase-11.md:962-963). Phase 12 section 7.3 removes those rows and makes explicit ranges beyond the overlap P2, so the phase 11 text is stale.
- Evidence: phase-11.md:1493 "### 7.4 No operator-planned backfill (DR-380)"; phase-11.md:1494 "The backfill rows are removed from API.md 3.5."; the base sentence "the operator-planned backfill of phase 12 is not repeated here" "now read: no phase builds an operator-planned backfill in this release." (phase-11.md:1497-1498).
- Why refuted: the 7.4 block names the sentence it replaces and says what it now reads, so it wins over the base text at phase-11.md:962-963.

### V-12-10-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: activate writes detail.accepted_delta, but akr_app has no UPDATE grant on admin.dq_issues.detail, so the write is refused at run time.
- Evidence:
  - phase-12.md:222-223 "GRANT UPDATE (owner_id, status, resolution, resolution_note, resolved_by, resolved_at, accepted_until," / "signed_document_id, change_request_id, due_at, updated_at, version) ON admin.dq_issues TO akr_app;" The list has no detail. The only GRANT lines naming dq_issues are at phase-12.md:220-224.
  - phase-12.md:1330 "`activate` of the `known_adjustment` target also writes `detail.accepted_delta`" (block 7.5). No grant is added in that block.
  - The UPDATE grant that lists detail is on recon_results, a different table: phase-12.md:139 "GRANT UPDATE (actual, delta, tolerance, within_tolerance, outcome, reason_code, detail, evaluated_at)".
  - The write runs as akr_app: phase-04.md:847 "the change activates in the transaction of the final approval"; the akr-api role is db/akr_app (phase-00.md:597).
- Fix: Section 1, migration 0037 grants (phase 12, lines 222-223): add detail to the akr_app UPDATE list.
- Note, not a claim of this pass and not counted: the same akr_app list also lacks count and last_seen. The step 8 upsert (section 3.3, phase-12.md:745 "count = dq_issues.count + EXCLUDED.count, last_seen = GREATEST(...)") runs as akr_app inside recon_run, so its conflict branch would be refused for a recurring break. The grant inventory says only akr_ingest upserts counts (phase-12.md:432).

### V-12-10-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: No repository method returns the latest recon result for an object and field. ReconRepository has expected_for by run only.
- Evidence: phase-12.md:498 "async fn expected_for(&self, conn: &mut PgConnection, run_id: Uuid) -> Result<Vec<ReconResult>, RepoError>;" is the only method in ReconRepository (phase-12.md:487-503) that returns results; the others take ids, rows or filters and return runs, counts or ids. phase-12.md:531 "accepted_for" returns issues, not results. The lookup appears only in prose: phase-12.md:1330 "the delta of the issue's latest recon result for its object and field".
- Fix: Section 2, ReconRepository (phase 12): add the read that returns the latest result for an object and field, or name the query in block 7.5.

### V-12-10-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: "Latest" is undefined (by evaluated_at, run created_at or run id). The issue object is stored as <rung>:<object> while recon_results keeps rung and object apart. The no-result case is not stated.
- Evidence: phase-12.md:1330 "the delta of the issue's latest recon result for its object and field" gives no order. phase-12.md:679 stores "object `<rung>:<object>`", while phase-12.md:109-110 keep "rung TEXT NOT NULL" and "object TEXT NOT NULL" apart. By analogy, the plan orders runs by finished time (phase-12.md:647 "the finished time of the latest completed day run"), which makes the order inferable for runs but not for results. Greps: "latest" occurs at phase-12.md:647, 1167, 1170 and 1330 only; no text states the no-result case.
- Fix: Block 7.5 or section 3.5 activate (phase 12): define which result counts as latest and what activate does when no result exists.

### V-12-10-d  STANDS   (first agent: LOW; refuter: LOW)
- Claim: 7.5 names DqIssueRepository::accepted_for. Section 2 defines DqRepository, with accepted_for at phase-12.md:531.
- Evidence: phase-12.md:1331 "`DqIssueRepository::accepted_for` takes the field". phase-12.md:522 "pub trait DqRepository: Send + Sync {" carries accepted_for at phase-12.md:531. Grep "DqIssueRepository": phase-12.md:1331 only.
- Fix: Block 7.5 (phase 12, line 1331): use the trait name that section 2 defines, DqRepository.

### V-12-11-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Step 7 looks up the accepted issue by "the same object", which is the source object. Step 8 stores dq_issues.object as <rung>:<object>, so the lookup as written never matches and confirmed_gap never fires. The key is not stated.
- Evidence:
  - phase-12.md:677-678 "`accepted_for(source, object, kind = recon_break)` returns an accepted issue for the same object and group" (step 7).
  - phase-12.md:679 "object `<rung>:<object>`" (step 8). phase-12.md:174 gives a third form: "source table, landed object or the rung of a break".
  - phase-12.md:1332-1333 (block 7.5): "an accepted issue of the same object and field", the same ambiguity.
  - Grep "<rung>:<object>": phase-12.md:679 only. Recon results keep the object without a rung prefix (phase-12.md:110).
- Fix: Section 3.1 step 7 and block 7.5 (phase 12): state the object key the lookup passes, in the form step 8 stores it, and make the dq_issues.object comment at phase-12.md:174 agree.

### V-12-12-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: No named method returns the blind periods that cover a date for a source or the system. Section 2 CoverageRepository has none. Phase 11 BlindPeriodRepository has open, close, open_for_source (source scope only) and list(f: &BlindFilter), and BlindFilter is defined nowhere.
- Evidence: phase-12.md:853-854 "`blind_period` (a blind period of the source or the system covers the as-of date)" (section 3.7 step 5) names no method. phase-12.md:927 "in three places: the ledger" (section 3.10) names none either. CoverageRepository (phase-12.md:506-518) has no blind-period method; no "blind" hit appears in the section 2 trait range (phase-12.md:487-560). phase-11.md:591 "async fn open_for_source(&self, conn: &mut PgConnection, source: Uuid)" (source scope only); phase-11.md:592 "async fn list(&self, conn: &mut PgConnection, f: &BlindFilter, page: Keyset)". "BlindFilter" occurs only at phase-11.md:592 across docs/plan and docs.
- Fix: Section 2 CoverageRepository (phase 12): name a read that returns the periods covering a date for a source or the system; if the phase 11 list is meant for this, define BlindFilter in the phase 11 BlindPeriodRepository section.

## Summary

- REFUTED: 3 (V-12-01-b, V-12-05-a, V-12-08-a)
- STANDS: 11
- PARTLY: 2 (V-12-02-a, V-12-07-a)

Note on the counts: 16 unique Gap IDs were checked, each once. The first agent's Summary table repeats them, and those repeats were not counted again.

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-12-01-a | STANDS | MEDIUM | Step 8 register writes of a day run have no named event; dq.recorded is per batch and recon.completed carries outcome counts only | phase 12, section 3.1 step 8 |
| V-12-02-a | PARTLY | MEDIUM | The kind or flag that proves "signed by the bank" is not named (a bank-signed test exists at phase-12.md:1274) | phase 12, section 3.5 step 3 |
| V-12-02-b | STANDS | LOW | Call named as submit with kind, key and body; phase 04 has create(origin = Dedicated) with submit: true | phase 12, section 3.5 step 4 |
| V-12-03-a | STANDS | LOW | Same naming defect for the exclusion proposal | phase 12, section 3.6 step 3 |
| V-12-04-a | STANDS | MEDIUM | No reason code for a card row; a card-guard stop writes no quarantine row, so the refusal cannot fire | phase 12, section 3.11 |
| V-12-04-b | STANDS | LOW | Section 6 expects status assigned; no enum or CHECK has it | phase 12, section 6 quarantine transitions test |
| V-12-07-a | PARTLY | LOW | Restore path not stated (the job, not the route, runs before the API opens); 7.2 says break where step 2 gives source_anomaly | phase 12, section 7.2 |
| V-12-10-a | STANDS | HIGH | akr_app has no UPDATE grant on dq_issues.detail | phase 12, section 1 migration 0037 grants (lines 222-223) |
| V-12-10-b | STANDS | MEDIUM | No method returns the latest recon result for an object and field | phase 12, section 2 ReconRepository |
| V-12-10-c | STANDS | MEDIUM | "Latest" result undefined; no-result case not stated | phase 12, block 7.5 or section 3.5 activate |
| V-12-10-d | STANDS | LOW | 7.5 names DqIssueRepository; section 2 defines DqRepository | phase 12, block 7.5 (line 1331) |
| V-12-11-a | STANDS | MEDIUM | Step 7 lookup key does not match the step 8 key | phase 12, section 3.1 step 7 and block 7.5 |
| V-12-12-a | STANDS | MEDIUM | No method returns blind periods covering a date; BlindFilter defined nowhere | phase 12, section 2 CoverageRepository |
