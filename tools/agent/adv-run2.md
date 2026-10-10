# Adversarial pass, run 2 (alerts under concurrency, replay and late data; Haiku draft for lead review)

### ADV-2-01 Failed or abandoned live scenario run never resumes; the batch is silently never evaluated   (verdict: FLAW; phase 16)
- Rule: "Failure: status `failed` with `error_code`; the job retries (five attempts)." (phase-16.md:1173). Start step: "`None`: the batch was already run for this version and mode; stop." (phase-16.md:1145). Target: a true detection must not be lost (CRITICAL class in the prompt).
- Attack:
  1. A live `scenario_run {version, incremental, batch}` starts. `start_once` inserts the `scn.scenario_runs` row (status `running`) and the run commits chunks of 5,000 groups, each chunk writing detections through `insert_or_get` and `AlertSink::emit`.
  2. Chunk 4 hits a transient storage error (or the worker dies, so the job lease expires and the reaper sets the job back to `queued`, phase-00.md:546-547). The run row is left `failed` or `running`.
  3. The job retries. Step 2 calls `RunRepository::start_once` on the batch key. The unique index `uq_scenario_runs_batch` covers every status, so the insert returns `None`, and the run stops (phase-16.md:1145).
  4. Groups in chunks 4 to n of that batch are never evaluated. No further attempt changes that, and nothing re-enqueues the batch: the fan-out is idempotent on the same key (phase-16.md:1137-1138), so a re-delivered `canon.touched` event is a no-op too.
  5. The run stays `failed` or `running`, with no alert and no audit `scenario.run_completed`. Live alerts for those customers are missing for good, while the scenario reports "operating".
- Guard search: `uq_scenario_runs_batch ON scn.scenario_runs (scenario_version_id, mode, batch_id) WHERE batch_id IS NOT NULL` (phase-16.md:209-210, no status predicate). `start_once` is documented as "INSERT ... ON CONFLICT DO NOTHING on the batch unique index; `None` when the batch was already run" (phase-16.md:767). The run table has no cursor column (phase-16.md:183-205). The replay job has a resumable cursor (phase-16.md:1272-1284), but the live run does not. Grep "resum", "cursor", "reclaim" in phase-16 section 3.6 and 3.7: none for `scenario_run`. Lease reaper (phase-00.md:546-547) re-queues the job but says nothing about the run row. Phase 16 has no takeover of a `failed` row.
- Severity (FLAW only): CRITICAL
- Proposed 7.x text (FLAW only): "### 7.N Live run retry takes over an unfinished run row
  `RunRepository::start_once` takes the existing row over when its status is `failed`, or `running` with the job lease expired (`locked_at` older than `jobs.lease_s`). The takeover sets status `running`, clears `error_code`, and restarts the batch from its first chunk. A restart writes no duplicate alert: each detection goes through `insert_or_get`, and an equal digest returns `Duplicate` (section 3.7). The run row never reaches `completed` with chunks skipped. A run that is `completed`, or `refused_cost`, still returns `None`. Audit `scenario.run_resumed {run_id, attempt}`."

### ADV-2-02 Two alerts can mark each other as duplicates and both leave the queues   (verdict: FLAW; phase 17)
- Rule: "`duplicate_of` must be an alert of the same case, different from this one; state `duplicate` (from `queued` or `claimed`)" (phase-17.md:1153-1154). Also "Marking a P0 or P1 alert as a duplicate in bulk is refused" (phase-17.md:1207-1213), which shows that a single duplicate mark is meant to be a person's act, not an automatic one.
- Attack:
  1. Alerts A and B are both `queued` in case C (same party, same family).
  2. Analyst X, in transaction T1, runs `mark_duplicate {duplicate_of: B}` on A. T1 locks A `FOR UPDATE`, checks that B is in case C, and sets A to `duplicate`.
  3. Analyst Y, in T2 at the same time, runs `mark_duplicate {duplicate_of: A}` on B. T2 locks B, checks that A is in case C, and sets B to `duplicate`.
  4. T1 and T2 lock different rows, so neither waits. The check in each reads the other alert's `case_id`, which has not changed, and it does not read the other alert's `state`. Both commit.
  5. A and B are both `duplicate`, each pointing at the other. Both are off the queue and out of `Next`. Case C has no `queued` or `claimed` alert left, though it still shows both rows.
- Guard search: `CHECK (duplicate_of IS NULL OR duplicate_of <> alert_id)` on `casework.case_alerts` (phase-17.md:366) covers only self-reference, not a cycle. `alert.alerts` has only `CHECK ((state = 'duplicate') = (duplicate_of IS NOT NULL))` (phase-17.md:168). `alert.alerts_guard` checks state moves and not the target (phase-17.md:449-470). No trigger or unique index covers the target's state. The alert's lock covers only its own row (phase-17.md:1131 "Lock the alert `FOR UPDATE`"). Grep "duplicate_of" across phase-17: no other check, and no mention of a case lock on the mark path.
- Severity (FLAW only): MEDIUM
- Proposed 7.x text (FLAW only): "### 7.N Duplicate target must be a live alert of the same case
  On `mark_duplicate` and in each bulk item, lock the target alert `FOR SHARE` in the same transaction, and refuse with 422 `E_VALIDATION` at `/duplicate_of` (reason `target_not_live`) when the target's state is `duplicate`. Lock order: the case row `FOR UPDATE` first (as attach does, section 3.5), then the two alerts in ascending `id`. Result: a cycle cannot be formed, because the second writer sees the first writer's `duplicate` state after the case lock."

### ADV-2-03 Amendment racing a case decision: decided alert changes with no flag   (verdict: FLAW; phase 17, with phase 18)
- Rule: "Decided cases (M11, late data): when the case already has an outcome ... the decision is not rewritten. The amendment writes `case.alert_amended_after_decision`" (phase-17.md:1297-1299). Also amendments must never change a decided alert without anyone seeing it (HIGH class in the prompt).
- Attack:
  1. Decider D runs phase 18 `RecordDecisionEffect` on case C with alert A. The decision writes `casework.cases.outcome` and `casework.decisions` in T1 under the case row lock (phase-17.md:1111 "under the case row lock"; phase-18 writes the outcome column, phase-18.md:423).
  2. Before T1 commits, the worker emits `Amended(MeasuresChanged)` for A's detection key in T2. T2 finds A through `alert_index`, locks A `FOR UPDATE` (phase-17.md:1292 "lock it"), and does not lock case C.
  3. T2 reads C's `outcome` under READ COMMITTED. It still sees NULL, because T1 has not committed. T2 writes A's new `measures`, `matched_txn_ids`, `snapshot_sha256` and `amendment_reason`, and commits without a flag.
  4. T1 commits. The decision of C rests on the earlier evidence, A now holds different measures, and no `case.alert_amended_after_decision` event exists. Nobody sees the change.
- Guard search: the amendment takes only the alert lock (phase-17.md:1292). Phase 18 takes no `FOR SHARE` on alert rows; grep "FOR SHARE" in phase-18.md: none. Phase 16 step 9 holds `FOR SHARE` on the scenario row only (phase-16.md:1165). The case lock is named only for attach and merge (phase-17.md:1039, 1081). The serial order is safe; only the overlap is not guarded.
- Severity (FLAW only): HIGH
- Proposed 7.x text (FLAW only): "### 7.N Amendment takes the case lock before reading the outcome
  Section 3.13 amendment steps: first lock the case of the alert (`casework.case_alerts.case_id`) `FOR UPDATE`, then lock the alert, as attach does (lock order case, then alert). Read `outcome` and the decision event only after both locks. Then the flag rule of section 3.13 applies to the committed decision."

### ADV-2-04 A duplicate alert cannot move to no_longer_matches: the CHECK refuses it   (verdict: FLAW; phase 17)
- Rule: "The alert moves to `no_longer_matches` (from `queued`, `claimed`, `triaged` or `duplicate`; the guard allows it)" (phase-17.md:1300-1301). The guard lists `OLD.state = 'duplicate' AND NEW.state = 'no_longer_matches'` as allowed (phase-17.md:467-468).
- Attack:
  1. Alert A is `duplicate`, with `duplicate_of = B` (set by a person earlier, section 3.8).
  2. A replayed or late batch yields `d.matches = false` for A's key. The sink takes `Amended(NoLongerMatches)` and sets state `no_longer_matches`, leaving `duplicate_of` set (section 3.13 names no step that clears it).
  3. The row would now hold `state = 'no_longer_matches'` and `duplicate_of IS NOT NULL`. The CHECK `(state = 'duplicate') = (duplicate_of IS NOT NULL)` evaluates to `FALSE = TRUE`, so the UPDATE fails with a check violation.
  4. `AlertSinkError::Db` aborts the bucket (phase-16.md:1226). The run fails and retries (see ADV-2-01). A's state stays `duplicate`, and the case shows a duplicate of a detection that no longer matches.
  5. Matches again (section 3.13) has the same trap: `no_longer_matches` back to `queued` with `duplicate_of` still set fails the same CHECK.
- Guard search: `CHECK ((state = 'duplicate') = (duplicate_of IS NOT NULL))` (phase-17.md:168). No step in section 3.13 or 3.8 clears `duplicate_of` on the alert or on `casework.case_alerts`. Grep "duplicate_of" in section 3.13: none.
- Severity (FLAW only): MEDIUM
- Proposed 7.x text (FLAW only): "### 7.N Leaving the duplicate state clears the link
  Any transition out of `duplicate` (to `no_longer_matches`) sets `alert.alerts.duplicate_of = NULL` in the same UPDATE, and the event `alert.no_longer_matches` payload carries the previous `duplicate_of` id. The matches-again move (`no_longer_matches` to `queued`) keeps `duplicate_of` NULL. `casework.case_alerts.duplicate_of` is kept, so the case history still shows the mark."

### ADV-2-05 Assignment ignores the target's work-in-progress lock, so the limit can be exceeded   (verdict: FLAW; phase 17)
- Rule: "at or above `casework.wip_limit` (default 10): 409 `E_WIP_LIMIT`" (phase-17.md:1134-1135), and "Target WIP under the limit (step 6 of the claim)" for assignment (phase-17.md:1163).
- Attack:
  1. Analyst U holds 9 claimed alerts. Supervisor S assigns alert X to U (`POST /alerts/{id}/assign`).
  2. S's transaction counts U's WIP (9), finds it under the limit, and does not take `pg_advisory_xact_lock(hashtext('wip:' || U))`, because the assign steps name no such lock.
  3. At the same moment U claims alert Y. The claim takes the `wip:U` lock (phase-17.md:1130), counts 9, and claims Y.
  4. Both commit. U holds 11 claimed alerts against a limit of 10.
- Guard search: the `wip:` advisory lock appears only in the claim (phase-17.md:1130) and in "Next" (phase-17.md:1142). Assignment steps 1 to 5 (phase-17.md:1160-1170) do not name it. No CHECK on counts is possible (WIP is derived). Grep "wip" in phase-17: lines 1130, 1134, 1163, 1941, 1972 only.
- Severity (FLAW only): MEDIUM
- Proposed 7.x text (FLAW only): "### 7.N Assignment takes the target's WIP lock
  Section 3.8 Assign step 2 first runs `pg_advisory_xact_lock(hashtext('wip:' || target_user_id))`, the same key and order as the claim step 3, and then counts WIP. A refusal is 409 `E_WIP_LIMIT` at `/to`."

### ADV-2-06 A repeat of a pattern folds into an alert whose case is already closed and sealed; the new detection makes no alert   (verdict: FLAW; phase 16 with phase 17)
- Rule: "repeats fold into the open alert with every transaction listed (L-17)" (phase-16.md:1196); DSL fold "into the open alert" (docs/DSL.md:388). Also "a sealed case takes no alerts" (phase-17.md:1329), and a new alert of a closed case opens a new case beside it (phase-18.md:864).
- Attack:
  1. Customer P has aggregate detection D1 in window ending W. The bucket is W, so alert A is created in case C (family F) and decided `no_report`. C is closed and sealed (phase 18 `sealed_at`).
  2. Four days later P repeats the pattern. The window of the new detection ends at W+4, so the bucket fold runs `open_bucket(scenario, anchor_hk, since = W+4-6)`. That returns key K1 (bucket W), because the fold asks only for buckets that start on or after `since` and checks no alert state or case status (phase-16.md:810, 1193-1196).
  3. The new detection has the same fingerprint as K1 (bucket W reused). Step 2 of the sink (phase-17.md:884-890) sees an existing key with a different digest and `d.matches = true`. It returns `Amended(MeasuresChanged)` before the attach step, so no new alert, no new case and no case lookup (phase-17.md:1297).
  4. A is amended with the new transactions. The flag `case.alert_amended_after_decision` is written to case C, which is sealed; section 3.13 does not check the seal. Alert A is `triaged` and is in no queue (section 3.8 Triage), so the repeat appears in no work list.
  5. The true new detection is recorded as a change inside a closed alert and never reaches an analyst. The case closed on the earlier evidence stays closed.
- Guard search: `open_bucket` is documented as "the newest key of the anchor whose bucket starts on or after `since`" (phase-16.md:810) with no status filter. The sink's Duplicate and Amended branches return before step 5 (case lock and attach, phase-17.md:892). Grep "sealed" in phase-17 section 3.13: none. The sealed-case refusal is stated for phase 18 services (phase-18.md:676, 691), not for the intake path. Grep "open alert" in phase-16 and phase-17: only the fold text and DSL. No guard found.
- Severity (FLAW only): CRITICAL
- Proposed 7.x text (FLAW only): "### 7.N A repeat after closure opens a new bucket
  `open_bucket` (phase 16 section 3.7) returns a key only when its alert is in an open case: join `alert.alert_index` to `alert.alerts` (state `queued`, `claimed`, `triaged` or `no_longer_matches`) and to `casework.case_alerts` and `casework.cases` (status `open` or `in_review`). A closed, sealed or merged case is never folded into. The new detection then gets a new bucket (the window end) and a new fingerprint, so the sink returns `New` and section 3.1 opens a case beside the sealed one with `case.linked_prior`. The fold also requires the key's bucket start to be on or before `window_to` (see ADV-2-07)."

### ADV-2-07 A late window folds into a newer bucket and overwrites its measures; the window columns do not follow   (verdict: FLAW; phase 16 with phase 17)
- Rule: the fold is "within the dedupe span" (phase-16.md:1193-1196), and the alert's evidence must describe the window it was raised for (phase-16.md:1189 "`bucket` is the bucket start date"; phase-17.md:1292-1296 amendment changes measures but not window).
- Attack:
  1. Aggregate detection D1 for customer P, window ending 1 June, opens bucket 1 June and alert A (window 1 June to 1 June).
  2. Later, a settled run (phase-16.md:1176, "a row with `late = true` ... triggers the settled run") evaluates an older window ending 30 May for P. `open_bucket(since = 30 May - 6)` returns key K1 (bucket 1 June). Its start is on or after `since`, and nothing requires it to be on or before the new `window_to`.
  3. The 30 May detection folds into K1. Its digest differs, so the sink returns `Amended(MeasuresChanged)`. A's `measures`, `matched_txn_ids` and `snapshot` change to the 30 May evidence. `window_from` and `window_to` stay at 1 June, because section 3.13 does not list them.
  4. The next run of the 1 June window changes the digest back, and A is amended again. Each run writes `detection.amended` and `alert.amended`, and the alert swaps between two windows. The evidence no longer matches its own window columns, and the 1 June rows that were visible are replaced.
- Guard search: no check on `window_to` in section 3.7 or the `open_bucket` signature (phase-16.md:810-811). Phase 17 3.13 update list (phase-17.md:1292-1296) excludes `window_from` and `window_to`. Grep "window_to" in phase-17 section 3.1 and 3.13: set at insert only.
- Severity (FLAW only): MEDIUM
- Proposed 7.x text (FLAW only): "### 7.N Fold only into buckets at or before the window end
  `open_bucket` takes `until = window_to` as well as `since`, and returns only keys with `bucket_start` between `since` and `until` inclusive. A detection whose window ends before the newest bucket of its anchor gets its own bucket (its window end) and its own fingerprint. The sink then returns `New` or `Duplicate` for it, and the alert of the newer window keeps its evidence."

### ADV-2-08 A name that changes and changes back leaves no open screening alert for the current match   (verdict: FLAW; phase 14)
- Rule: "a name change supersedes only undecided alerts" (docs/plan/research.md DR-101). Dedupe key: "`ON CONFLICT (fingerprint) DO NOTHING`" (phase-14.md:721 and 999). A true match must reach review (CRITICAL class: a sanctions or freeze hit lost).
- Attack:
  1. Subject S has name N1 that matches list entry E. The run inserts X1 with `fingerprint = f(S, E, subject_fp(N1), entry_fp, major)`. X1 is `open`.
  2. S's name changes to N2 (a correction or a second source). The onboarding run inserts X2 with `f(..., subject_fp(N2), ...)`. Step 4 supersedes X1 (`superseded_at` set, undecided). The queue now shows X2 only.
  3. S's name reverts to N1 (the source flips back, or the first record is restored). The run computes `f(S, E, subject_fp(N1), ...)`, which equals X1's fingerprint.
  4. `INSERT ... ON CONFLICT (fingerprint) DO NOTHING` hits X1, which is still in the table (superseded). No row is returned, so step 4 (supersede) never runs for this candidate.
  5. X1 stays hidden (superseded), X2 stays open with the content of N2, and no open alert holds the N1 match. A rerun has the same result, so the true N1 match is never reviewed. If an analyst decides X2 `incorrect` with `remember_pair`, the pair memory stores `subject_fp(N2)`, so N1 is not suppressed either, and the N1 match is still never reviewed.
- Guard search: the column `fingerprint BYTEA NOT NULL UNIQUE` is a full unique constraint, not a partial one that excludes superseded rows (phase-14.md:260). The conflict target in the insert is the full key (phase-14.md:721). The queue index is partial on `superseded_at IS NULL` (phase-14.md:293-294), which only affects listing. Grep "superseded" in section 3.5: step 4 runs only for new rows. No other reopen or un-supersede step found in phase 14.
- Severity (FLAW only): CRITICAL
- Proposed 7.x text (FLAW only): "### 7.N Superseded alerts do not block their fingerprint
  Replace the column constraint `fingerprint BYTEA NOT NULL UNIQUE` (phase-14.md:260) with `CREATE UNIQUE INDEX uq_screening_alerts_fingerprint_live ON alert.screening_alerts (fingerprint) WHERE superseded_at IS NULL;` and make the insert `ON CONFLICT (fingerprint) WHERE superseded_at IS NULL DO NOTHING` (section 2.4). A decided alert is never superseded, so a decided fingerprint still blocks a rerun. A reverted name inserts a new row, and step 4 then supersedes the undecided X2. The superseded X1 stays in the table for the trail."

### ADV-2-09 A decision can be taken on a superseded screening alert   (verdict: FLAW; phase 14)
- Rule: transition table: "`true`, `incorrect`, or superseded | any | refused | 409 `E_NOT_ALLOWED_IN_STATE`" (phase-14.md:1016). Decision step 3: "Apply the transition table; refuse with 409 otherwise" (phase-14.md:1023).
- Attack:
  1. Alert X for subject S and entry E (freeze list) is `open`. A later run supersedes X (`superseded_at` set, `superseded_by` set to X2) after S's name changed.
  2. An analyst or a direct API client, who still holds X's id and version, posts `POST /screening/alerts/{X}/decision` with outcome `true`.
  3. Step 2 `get_for_update` reads state `open`, and the If-Match equals `version`. The supersede step (phase-14.md:999) does not say it increments `version`, so the If-Match check passes.
  4. Step 3 applies the table on `state` alone. `state` is `open`, so the transition is allowed. The table's "or superseded" row is never tested.
  5. The freeze trigger (step 8) starts for the old name S, and the decision is recorded on a superseded alert.
- Guard search: the table row for superseded (phase-14.md:1016) has no SQL form; `superseded_at` is not a `state` value (CHECK at phase-14.md:272, only `open`, `potential`, `true`, `incorrect`, `needs_more_data`). Trigger `alert.screening_state_guard` checks only `true` and `incorrect` as terminal (phase-14.md:486-493). Cedar: `ScreeningAlert { ..., superseded: Bool }` is declared (phase-14.md:1079), but no `permit` or `forbid` in section 3.11 reads `superseded` (grep "superseded" in phase-14: no policy use). Decision steps 1 to 11 (phase-14.md:1020-1032) do not read `superseded_at`.
- Severity (FLAW only): HIGH
- Proposed 7.x text (FLAW only): "### 7.N Decision refuses a superseded alert
  Decision step 3 refuses with 409 `E_NOT_ALLOWED_IN_STATE` when `superseded_at IS NOT NULL` (read in the step 2 `get_for_update`). The supersede step (section 3.5 step 4) takes the old row `FOR UPDATE` and sets `version = version + 1`, so an If-Match from before the supersede fails with 409 `E_VERSION_CONFLICT`. The Cedar schema attribute `superseded` is then used in `screening.alert.decide`: `forbid (principal, action == Akr::Action::"screening.alert.decide", resource) when { resource.superseded };`."

### ADV-2-10 Rows that map late through a quarantine retry never reach detection   (verdict: FLAW; phase 11 with phase 16)
- Rule: a row that fails mapping is kept as `row_failed` and mapped later: "Success: the quarantine row becomes `accepted`" (phase-11.md:836-837). The detection trigger is the outbox event: "when the writer inserted or changed any row, the outbox event `canon.touched` ... is written" (phase-11.md:833-834). Detection must cover every row that reaches canon (phase-16.md:1129).
- Attack:
  1. Batch B is landed and mapped (`ingest_map_pending`). Row R1 of account A fails (`row_failed`, for example an unmapped currency code). The rest of B is written and `canon.touched {batch_id: B}` runs. The incremental run for (live version, B) is started and evaluates the customers of B's touched accounts. A is not among them, because R1 never reached canon.
  2. An analyst adds the code to the mapping and publishes. The retry of `row_failed` (phase-11.md:835-838, step 2 "Retries") maps R1 under the newest published version and writes it through `CanonicalWriter`. Step 2 names no outbox event. The only event is `canon.written` (phase 10), and no detection consumer reads it.
  3. R1's transaction now sits in canon for account A and its customer. No `canon.touched` is written for B, so the fan-out (phase-16.md:1129-1135) never runs for it. Even if it did, the run ledger key (version, mode, B) already exists (phase-16.md:205-210), so `start_once` returns `None`.
  4. The customer's detections involving R1 are never evaluated by the incremental path. The sweeps (phase-16.md:1180-1183) cover only the recent window (daily) or the month window (monthly), so an older month never gets a run.
- Guard search: grep "canon.touched" across phases: emitted only by phase-11.md:833 (pending path). Section 3.3 step 2 (phase-11.md:835-838) names no outbox event. Grep "canon.written" consumers in phases 14, 15, 16, 17, 19: none for detection. The sweep text (phase-16.md:1180-1183) gives no month range for the monthly sweep, so the reach of a late month is not stated.
- Severity (FLAW only): HIGH (CRITICAL where the late month is outside the sweep reach, which the text does not state)
- Proposed 7.x text (FLAW only): "### 7.N Quarantine retries emit canon.touched
  Section 3.3 retry step 2: when a retried row is written, the same transaction writes the outbox event `canon.touched {batch_id: <the row's batch_id>, source_id, source_table, entity_kinds, business_date: <the row's business date>}` (phase 11 section 3.3 step 1 rule). The fan-out of phase 16 then runs under a new run key: the ledger key of section 1.2 (version, mode, batch) must include the retry sequence, so that a late row starts its own run. Add `retry_seq INT NOT NULL DEFAULT 0` to `scn.scenario_runs` with the unique index on (scenario_version_id, mode, batch_id, retry_seq)."

### ADV-2-11 A pending batch mapped over several runs is run once; rows mapped in later runs are never evaluated   (verdict: FLAW; phase 11 with phase 16)
- Rule: "up to `ingest.max_unit_rows` rows in total per run" for pending batches (phase-11.md:827-829); `canon.touched` is written per run with the same `batch_id` (phase-11.md:833). "A batch is run once per version and mode" (phase-16.md:205-206). The run must evaluate every touched customer of the batch.
- Attack:
  1. Batch B holds 50,000 rows and is `landed` with no published version. Mapping run 1 maps the first `ingest.max_unit_rows` rows, writes canon, and emits `canon.touched {batch_id: B}`.
  2. The fan-out job (phase-16.md:1129) reads `SELECT DISTINCT account_id FROM canon.txn WHERE batch_id = B` at its own start time. It runs before run 2 commits, so it sees only the first slice. It enqueues `scenario_run {version, incremental, B}`, and `start_once` inserts the ledger row. The run evaluates the slice.
  3. Mapping run 2 maps the next slice and emits `canon.touched {batch_id: B}` again. The fan-out finds the same key, and `start_once` returns `None` (phase-16.md:1145). The run stops.
  4. The customers of the second slice are never evaluated in incremental mode. Only the sweeps (phase-16.md:1180-1183) look at them, and only if their window is still recent.
- Guard search: the ledger is keyed on batch only (phase-16.md:205-210). No cursor is kept for live runs (phase-16.md:183-205, see ADV-2-01). Section 3.3 does not say the pending batch is mapped in one run (phase-11.md:827-830 "up to ... rows in total per run"). Grep "max_unit_rows" in phase-16.md: no occurrence, so the scenario side does not know a batch is split.
- Severity (FLAW only): CRITICAL
- Proposed 7.x text (FLAW only): "### 7.N One run per touch of a batch
  The run ledger key becomes (scenario_version_id, mode, batch_id, touch_seq), where `touch_seq` is a new column on the `canon.touched` payload written by phase 11 section 3.3 (a counter per batch and table, incremented at each mapping run). `start_once` then accepts each touch once. A touch's fan-out reads `canon.txn` rows of the batch that are at or below its own `touch_seq` (by a new `mapped_seq` value on the row) so that each run evaluates only the rows it has been told about and no row is evaluated twice."

### ADV-2-12 Assignment and bulk assignment can take over a live claim   (verdict: DOUBT; phase 17)
- Rule: a claim is "409 `E_CLAIMED`" when held by someone else (phase-17.md:1131-1137); bulk dry-run refuses "`claimed_by_other`" (phase-17.md:1208-1210). Assign step 3 says only "the target becomes the claimer (state `claimed`)" (phase-17.md:1164-1165).
- Reading A (the text as written): a supervisor may reassign an alert that another analyst holds, as a push (M-15). Then the previous claimer loses the claim silently, and the event is `alert.assigned {from, to}`.
- Reading B (the bulk rule): an alert held by another person is never taken over, by a bulk action or by a single assignment.
- Race under reading B: a bulk dry run marks alert X `allowed` (queued). Analyst U claims X after the dry run. The real run applies "each item in its own savepoint" (phase-17.md:1209-1210) and sets `claimed_by` to the target without re-checking `claimed_by`, so U's claim is overwritten. Nothing in the text re-reads the claim under the row lock in the real run.
- Guard search: the assign steps (phase-17.md:1160-1170) have no check of `claimed_by`. Trigger `alert.alerts_guard` (phase-17.md:449-470) lets `claimed` to `claimed` pass (no state change). No Cedar rule on assignment of a held alert found by grep "assign" in phase-17.md section 3.17 (not read in full here).
- Question for the lead: does a supervisor's single assignment take over a live claim (reading A), or is it refused (reading B)? The bulk real run needs the same answer and a re-check under the row lock.
- Severity (FLAW only): none (DOUBT)

### ADV-2-13 Corrections that land in closed months change canon and features with no detection run   (verdict: DOUBT; phase 11, phase 13 and phase 16)
- Rule: a late or corrected fact must reach the detection that explains it: "Late data ... triggers the settled run for its batch" (phase-16.md:1176-1177); a replay of a closed month "sets `reopen_from`" for features (phase-13.md:600-601).
- Attack, two paths:
  1. Mapping replay apply (phase-11.md:1115-1122, `ingest_mapping_replay_apply`) runs "the unit machinery of section 3.2 steps 2 to 5 without the watermark step" (phase-11.md:1117) and writes new bi-temporal canon versions. Section 3.8 names no `canon.touched` for apply; that event is named only in the pending path (phase-11.md:833). So apply changes canon for a closed month and triggers no fan-out.
  2. Feature recompute with `reopen_from` (phase-13.md:600-625) rewrites `feat.feature_values` of a closed month. Section 3.2 step 6 writes only an audit event, and no outbox topic is produced (phase-13.md:43 "no outbox topic"). A scenario that reads that feature for the closed month is not re-run.
  3. The only later re-evaluation is the sweep: the daily sweep covers L2 windows ending at the bank date and the monthly sweep "the month window" (phase-16.md:1180-1182). The text does not say which months the monthly sweep reaches. An alert raised on the old feature or canon values is therefore never amended, and a new detection that the correction creates is never raised for an older month.
- Guard search: grep "canon.touched" across all phases: emitted only in phase-11.md:833 (pending path). Grep "apply" in phase-11 section 3.8 for an outbox: none. Phase 13 has no outbox topic (phase-13.md:43).
- Question for the lead: should a mapping apply or a `reopen_from` feature run emit a touch (with the replay id as the batch), so that closed-month detections are re-run and amended through the normal amendment rules? If the bank accepts the sweep as the only path, the monthly sweep window must be stated.
- Severity (FLAW only): none (DOUBT)

## Guarded items checked (one line each)
- GUARDED: mapping replay writes no alert rows; it writes canon only at apply, and the replay rows go to `replay_results` (phase-11.md:1050; phase-16.md:1257-1263).
- GUARDED: seeded runs and replays never call the sink: "The sink is never called: a seeded run emits nothing." (phase-16.md:1396).
- GUARDED: shadow runs never call emit; only `scn.shadow_results.live_alert_id` is set (phase-17.md:1310-1316).
- GUARDED: a crash-retry that re-presents the same detection returns `Duplicate` when the digest is equal (phase-17.md:884-890; phase-16.md:1199-1203).
- GUARDED: a second live alert for one identity is blocked by the primary key of `alert.alert_index` (phase-17.md:182, `fingerprint TEXT PRIMARY KEY`).
- GUARDED: an open quarantine row per identity and reason, released or rejected only once: `uq_quarantine_open_identity` (phase-17.md:231-232) and the release job re-checks `open` (phase-17.md:953-956).
- GUARDED: two claims or two "Next" presses on one alert: row lock, If-Match, per-person advisory lock and SKIP LOCKED (phase-17.md:1126-1145).
- GUARDED: illegal alert state moves: `alert.alerts_guard` allows only the listed transitions (phase-17.md:449-470).
- GUARDED: case event chain forks: `case_events_chain_guard` and the case row lock (phase-17.md:428-441, 1111).
- GUARDED: merge deadlock: both cases locked in ascending id (phase-17.md:1081).
- GUARDED: clocks firing on a closed alert: handlers re-read and do nothing (phase-17.md:1276-1288).
- GUARDED: a feature or profile change after an alert was raised does not rewrite its snapshot; a later run can only amend or mark no longer matching, visibly (phase-16.md:1168-1170).
- GUARDED: screening onboarding per batch can miss a later slice, but `daily_base` re-screens every party in chunks (phase-14.md:985-987).
- GUARDED: an active screening decision is terminal for `true` and `incorrect` (phase-14.md:486-493).

## Summary
Counts by verdict: FLAW 11, DOUBT 2, GUARDED 14 (listed above; ADV-2-01 to ADV-2-11 are FLAW; ADV-2-12 and ADV-2-13 are DOUBT).

FLAW items by severity:
- CRITICAL: ADV-2-01 (failed or abandoned live run never resumes), ADV-2-06 (repeat folds into a closed, sealed alert and makes no new alert), ADV-2-08 (reverted name blocked by a superseded row, no open alert for the current match), ADV-2-11 (rows mapped in a later run of a split pending batch never evaluated).
- HIGH: ADV-2-03 (amendment racing a decision, no flag), ADV-2-09 (decision on a superseded screening alert), ADV-2-10 (quarantine retry writes canon with no touch).
- MEDIUM: ADV-2-02 (two alerts mark each other as duplicates), ADV-2-04 (duplicate cannot move to no_longer_matches; CHECK refuses it), ADV-2-05 (assignment bypasses the target WIP lock), ADV-2-07 (late window folds into newer bucket and overwrites its measures).

DOUBT items:
- ADV-2-12: may a supervisor's assignment or a bulk real run take over an alert another analyst has claimed? The text does not say.
- ADV-2-13: mapping apply and feature reopen emit no touch, so closed-month corrections are re-run only by sweeps whose month reach is not stated.
