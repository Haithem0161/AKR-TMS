# Adversarial pass, run 3 (watermarks, restores, failover, late batches, source pauses; Haiku draft for lead review)

### ADV-3-01 An operator pause freezes the CBS date, so freshness stays fresh and units look covered   (verdict: FLAW; phase 11, with 05 and 12)
- Rule: "Source state. A source is as fresh as its stalest watermark" (phase-11.md:1028); "a source whose business date or newest posting stops moving raises a stale-source state" (DR-39, research.md:50); a pause must not make a period look covered (run 3 target).
- Attack:
  1. A steward pauses an active CT2 source (`POST /sources/{id}/pause`, SU approval) on day D0+1. The last mapped receipt has `business_to` D0. The last CBS closed date read was D0+1 (phase-05.md:1002: the health job iterates only `active` or `paused_by_bank` sources; phase-05.md:937-950 the governor returns `Paused` for every load statement).
  2. For ten business days nothing reads the CBS: no `ingest_load`, no health job. The stored CBS date stays at D0+1.
  3. Freshness rule e (phase-11.md:1018) compares `data_as_of` (D0) with the stored CBS date (D0+1): one business day, not above `freshness.f3_max_business_days` = 1 (phase-11.md:447). Rule f needs the CBS date to have moved, which AKR never observes, so it does not fire. Rules a to g have no test on source state `paused` (phase-11.md:1013-1025 checked).
  4. The ledger takes `as_of_business_date` = D0 (phase-12.md:844). Precedence (phase-12.md:853-855): no live exclusion; no blind period covering D0 (see ADV-3-07); no `stale`; last posting D0 is not dark. Every unit is `monitored`, `covered = true`.
  5. The coverage statement counts postings only inside its window (phase-12.md section 3.9 step 2). D0+1 to today has no canon rows, so it is not in the denominator either. The covered share reads 100 percent after ten days without a read.
- Guard search: grep "paused" in phase-11.md section 3.7 finds only the state note (line 1028). phase-05.md:1002 skips paused sources in health. phase-12.md:853-855 has no state test. The blind period opened by the pause (phase-11.md:974-975) appears in the statement (phase-12.md section 3.9 step 3), so a person can see it: this is why the severity is not CRITICAL. No guard stops the covered reading. DR-39 (research.md:50) covers a business date or posting that stops moving, not a source that the bank has paused; no DR accepts a pause as covered.
- Severity: HIGH (a period is shown covered when it was not; the blind period is visible in the statement, so it is not CRITICAL).
- Proposed 7.9 text:
  "### 7.9 A paused source is stale" - In section 3.7, after rule b, add rule b2: a source in state `paused` or `paused_by_bank` is stale with `stale_reason = lag_ceiling` (an existing value of the `watermarks` CHECK), measured from `advanced_at`, whatever the stored CBS date says. The ledger then reports `source_stale` (phase 12 section 3.7 step 5). Test: pause an active source for three business days on the test clock; every watermark of the source is stale and every unit is dark.

### ADV-3-02 An endpoint change alone never halts the source (failover that dropped recent commits)   (verdict: FLAW; phase 11)
- Rule: research/25-core-banking-integration.md:434 (research 7.5, not carried into phase 11): a DR copy that "lost recent commits" must make the loader stop advancing and raise `source_regression`. Phase 11 uses `endpoint_changed` only to choose a cause: "The cause is `failover_suspected` when `endpoint_changed` is true" (phase-11.md:908-909).
- Attack:
  1. AKR reads the primary at marker X0 (watermark `as_of_marker` = X0).
  2. The bank fails over to a standby whose marker Y is above X0, because AKR had not yet read the commits between X0 and the failover. The standby lacks the commits that the primary made after Y.
  3. Next unit: the standby marker Y is not below X0, so `AsOfRegressed` does not fire (phase-11.md:906-908).
  4. The business date on the standby equals the primary's (the lost commits are on the open day), so `BusinessDateRegressed` does not fire (phase-11.md:905-906).
  5. `endpoint_changed` is true but triggers no halt, no blind period and no receipt rejection. Loads continue normally.
  6. The lost commits never reached AKR. The standby's day counts and sums agree with canon, so recon passes (see ADV-3-17).
- Guard search: grep "endpoint_changed" in phases 05, 11 and 12 finds only phase-11.md:903, 908 and the test list line 1441 (cause selection only). The three signals (phase-11.md:903-919) are the only halt paths. Row drop warnings apply only to `full_compare` (phase-11.md:912-914). No guard found. DR-90 (research.md:101) names the two regressions as the halt triggers and `failover_suspected` only as a cause label. It does not accept a failover that keeps the marker, so this is not a designed exception; the lead should resolve the conflict between DR-90 and research 7.5.
- Severity: HIGH (a period shown covered when it was not; the defect is the missing halt and blind period).
- Proposed 7.10 text:
  "### 7.10 An endpoint change halts the source" - In `RestoreDetector::evaluate`, `endpoint_changed` (phase 05 `engine_facts`) raises `AsOfRegressed` by itself, whatever the marker value. The halt uses the existing `halted_reason` value `as_of_regressed` and the cause `failover_suspected`. A planned switchover is cleared by the steward with `false_alarm` (section 3.5 resume). Test: change the fixture's endpoint identity with a higher marker; the source halts with `failover_suspected`.

### ADV-3-03 A sequence key committed late is never read (no commit-order guard for `sequence_id`)   (verdict: FLAW; phase 11, with 12 as the only detector)
- Rule: DSL.md:531, `overlap`: "Re-read window behind the watermark (commit-order problem, file 25 s.7.3)". Research 7.3 (research/25-core-banking-integration.md:419): "A transaction can take a key ... early and commit late. A poller that remembers 'highest value seen' then skips it forever." Phase 11 instead says: "For `sequence_id`: from the stored key tuple, no overlap (a sequence needs none)" (phase-11.md:759-761).
- Attack:
  1. Two CBS sessions insert into a table keyed by a sequence. Session A takes key 500 and does not commit yet.
  2. Session B takes key 501 and commits.
  3. An AKR unit reads keys above the stored value and sees up to 501. The unit commits and moves the stored value to 501 (phase-11.md:809-811 compare-and-set).
  4. Session A commits key 500.
  5. Every later unit reads keys above 501. Key 500 is never read again. No overlap re-reads it and `settle_seconds` applies only to timestamp rows (phase-11.md:860).
  6. The day's counts rung (phase-12.md section 3.1 step 4) compares the source count over the batch range with `control_totals.read`: the source count includes 500, so the result is `open` / `count_mismatch` on a later day run. The sums rung also breaks if 500 has a value.
- Guard search: phase-11.md:759-761 (no overlap for `sequence_id`); phase-11.md:860 (settle only for timestamp and t24 rows); no key re-read in section 3.1 step 4; no unique or CHECK that would catch a skipped key. The only guard is the recon break, which comes after the fact and leaves the row out of canon until a steward acts (no backfill route, phase-11 section 7+ 7.4). Not CRITICAL, because the break is visible.
- Severity: HIGH (a range is read wrong and stays wrong until a person acts; visible as an open break).
- Proposed 7.11 text:
  "### 7.11 Sequence keys re-read a commit window" - For `sequence_id`, the start of a unit is the stored key minus the mapping's `overlap`, counted in keys (the mapping's `overlap` for this kind is a whole number of keys, not a duration; the compile step of phase 10 accepts it only for `sequence_id`). The change detector (section 3.2 step 2) absorbs the repeated keys. Test: key 500 committed after 501 is read once, in the next unit, and lands once.

### ADV-3-04 A key or timestamp high-water mark below the watermark raises no signal where the engine marker is unvalidated   (verdict: FLAW; phase 11, open doubt 8 left open)
- Rule: research/25-core-banking-integration.md:434 (research 7.5): when the bank restores or fails over, "the source's high-water mark ... falls below AKR-TMS's watermark ... The loader treats this as a `source_regression` event: it stops advancing the watermarks". Phase 11 halts only on `AsOfRegressed` or `BusinessDateRegressed` (phase-11.md:903-911). Open doubt 8 (phase-11.md:1576): the SQL Server LSN, Db2 and Sybase markers "need a probe ... before they can halt a source".
- Attack:
  1. A CT2 source on SQL Server (or Db2 or Sybase) has a `sequence_id` or `timestamp_column` table and no business-date table that moves in the same source.
  2. The bank restores the database to a backup from earlier the same business day. The restored copy's keys and timestamps are below the stored watermark.
  3. `BusinessDateRegressed` does not fire (same date). `AsOfRegressed` cannot fire, because the marker for this engine is not validated (open doubt 8).
  4. `ingest_load` reads from the stored value, so it returns nothing for the restored range. Keys committed after the restore that fall below the watermark are never read. `advanced_at` moves on each empty read (phase-11.md:740 and 809, the heartbeat), so rules c and g (F1 and F5) do not fire, and rule e does not fire because `data_as_of` follows other tables only.
  5. The row-drop warning applies only to `full_compare` (phase-11.md:912-914).
  6. Only a later day run shows the difference, as an open `count_mismatch` (phase-12.md section 3.1 step 4).
- Guard search: grep "high-water", "high water" and "source_regression" across docs/plan returns no match. The `RestoreDetector` signals (phase-11.md:903-919) are the only halt paths. The business-date rule (rule f, phase-11.md:1021-1023) covers only `business_date` kinds. Recon is the only detector, and it is after the fact.
- Severity: HIGH (a range is never read, and the period looks fresh until a day run flags it).
- Proposed 7.12 text:
  "### 7.12 High-water mark check before each unit" - In section 3.1 step 5 (restore check), read the aggregate `MAX(<key column>)` (or `MAX(<timestamp column>)`) of the table through an approved statement (one `Read` statement under the governor, proven by `sqlproof`). When it is below the stored `value`, raise `BusinessDateRegressed`-equivalent: halt with `halted_reason = as_of_regressed` and cause `restore_suspected` (section 3.5). This works on every engine without a marker. Test: lower the fixture's maximum key below the stored value; the source halts.

### ADV-3-05 Late or back-dated rows older than the re-read window are never read, and no alarm covers them   (verdict: FLAW; phase 11 and 12)
- Rule: DSL.md:534, `lookback`: "`auto` derives the window from measured late-arrival percentiles ... and widens it automatically (C-21)". Research 7.3 (research/25-core-banking-integration.md:421): "alarm when a row arrives later than `L`, and widen `L` automatically". Phase 11 takes only `overlap` (phase-11.md:758-759). Phase 11 and 12 contain no late-arrival measurement, alarm or widening (grep "late-arrival", "widen", "arrives later" finds none in phases 05, 11, 12, except the unrelated UI wording).
- Attack:
  1. A back-dated posting with business date D-40 (more than 31 business days before today) appears in the CBS. Or a teller edits a settled transaction (its counterparty, not its amount) that carries business date D-40.
  2. Re-read units start at the watermark minus `overlap` (phase-11.md:758-759). The row is never inside a unit, so it is never landed.
  3. Recon: a day run covers only `recon.lookback_days` = 14 working days (phase-12.md:645 and the setting at phase-12.md:399). D-40 is never reconciled, and no batch range (which starts at the overlap start) reaches D-40 for the counts rung.
  4. For a late posting (an addition), the sums rung would show a break if D-40 were reconciled, but it is not. For an edit to a settled fact, the amount does not change, so the sums agree even if D-40 were reconciled; the hash conflict (`row_conflict_fact`, phase-10.md:1981-1984 and 1995) never fires, because the row is never read again.
  5. Scale: phase-11.md:1140 records "Back-dated entries reach 30 days (1.6% of legs), which is why the overlap is 31 days". The window covers the observed tail only.
- Guard search: the 31-day window is a designed setting (DR-37, phase-11 section 3.9); the explicit range beyond the overlap is P2 (phase-11.md:1493-1501, section 7+ 7.4). The designed bound is accepted; the missing alarm is not. No alarm, widening or key-set compare exists in phases 11 or 12.
- Severity: HIGH (a period is shown covered when it was not; the bound is a designed setting, so the defect is the missing detection; the lead may rate CRITICAL if a settled-fact edit counts as a lost detection).
- Proposed 7.5 text (phase 12 section 7+):
  "### 7.5 Reconciliation covers the whole re-read window" - Set the floor of `recon.lookback_days` to the largest `overlap` of any approved mapping (31 business days by default), with `floor = {"min": 31, "basis": "DR-37 overlap"}` and `governed = TRUE`. A late addition older than the window then appears in the day sweep as an `open` `control_totals` break (section 3.1 step 5) instead of never appearing. Record in Decisions that edits to a settled fact beyond the window stay undetected until a key-set compare (research 7.5) is built, and that widening `lookback: auto` is P2.

### ADV-3-06 A back-dated version of a bi-temporal row cannot be written, so the table stalls on every unit   (verdict: FLAW; phase 10 section 3.7, with the phase 11 receipt path)
- Rule: phase-10.md:1990-1993 (step 4) and 1995-1996 (step 5 facts): `valid_from` is "the batch's business date of the row (DR-39)"; `close_versions` closes the current row "when its `valid_from` is earlier", then `insert_rows` inserts the new version. The guard is `uq_party_current ... WHERE recorded_to IS NULL` (phase-10.md:731-732, and the same for customer, account and party_relationship at 733-737). Phase 11 step 9 (phase-11.md:814-815): a non-row error rolls back and writes `rejected` / `writer_failed`, and the watermark does not move.
- Attack:
  1. A party row (phase 10 bi-temporal entity) was last written with `valid_from` = D-5 (its current version).
  2. A correction to the same party arrives in the overlap re-read with business date D-10. Its hash differs, so it is a new version with `valid_from` = D-10.
  3. Step 4: the current row's `valid_from` (D-5) is not earlier than the new one's (D-10), so the current row is not closed (`recorded_to` stays NULL).
  4. `insert_rows` inserts the new row with `recorded_to` NULL. The partial unique index `uq_party_current` now sees two current rows for one source key and refuses the insert.
  5. The writer returns `E_INTERNAL` (not a row error), the batch rolls back with `writer_failed`, and the watermark stays. Phase 11 step 9 retries up to five times, and the same rows fail on every unit, so the table stops advancing for that branch.
- Guard search: the unique index stops a second current row from being written, so no wrong state is stored; the stall is the outcome. Grep "valid_from" in phase-10.md finds no rule for a version that is older than the current one. The writer's step 3 sends only dimensions to `RowConflict`, not bi-temporal entities. No designed exception found.
- Severity: HIGH (a range of source reads stops for the table until a steward intervenes; each attempt is a rejected batch, so it is not silent, but the table does not advance).
- Proposed 7.13 text (phase 10 section 7+):
  "### 7.13 Back-dated bi-temporal versions are conflicts" - In section 3.7 step 3, before step 4: when the new row's `valid_from` is not later than the current row's `valid_from`, do not write a version. Return a `RowConflict` of kind `value_change` (or `label_change` when only text changed) with the new version kept in raw, as for dimensions. Step 4 then runs only for a new `valid_from` that is later than the current one. Test: a party corrected with an older business date lands as a `row_conflict`, the batch is `partial`, and the watermark moves.

### ADV-3-07 Blind periods from pauses and halts carry no business dates, so "covers the as-of date" is undefined   (verdict: DOUBT; phase 11 section 3.6 and phase 12 section 3.7)
- Rule: phase-12.md:854: "`blind_period` (a blind period of the source or the system covers the as-of date)". Automatic pause periods are opened with `started_at` (wall-clock) and no `business_from` (phase-11.md:974-976). `business_from` is nullable (phase-11.md:86). Only the halt path sets `business_from` (phase-11.md:924).
- The two readings:
  (a) A pause period covers the as-of date only when its start, read as a business date, is on or before the as-of date. A pause that starts after the last closed date D0 does not cover D0, so the ledger shows the unit `monitored` (ADV-3-01 holds).
  (b) A source-scope period with no `business_from` covers every date from its start. Then the ledger shows the unit dark, which is the correct outcome.
- The text does not say which applies. The lead must decide. Under (a) ADV-3-01 is a FLAW; under (b) it is GUARDED.
- Proposed text: none (DOUBT). The lead may choose to set `business_from` on every automatic source opening to the day after the newest mapped receipt, and to state that a period with a `business_from` covers every as-of date on or after it.
- Severity: DOUBT (HIGH if reading a).

### ADV-3-08 `reset_and_clear` does not say it resets `value_business_date`   (verdict: DOUBT; phase 11 section 3.5 resume)
- Rule: phase-11.md:113-114 and 129: `value_business_date` is the business-date cursor for `business_date` kinds, and the CHECK at phase-11.md:129 requires it whenever `value` is set. The restore decision text (phase-11.md:953-955): "The watermarks are reset (`reset_and_clear`) ... Rows after the restored point are read again and compared by hash." The trait (phase-11.md:554) is `reset_and_clear(tx, source, to: &BTreeMap<String, String>)`, one text value per table and branch.
- The two readings:
  (a) The reset also sets `value_business_date` to the restored date. The planner then re-reads from there, and the regression test (phase-11.md:905) stays quiet.
  (b) The reset sets only `value`. The planner starts from the old `value_business_date`, so the restored days are never re-read and no `row_conflict` is raised for a settled fact the restore changed. The regression test (phase-11.md:905: bank date minus 1 earlier than `value_business_date`) then fires again on the next tick, and the source halts again.
- (b) is visible (a repeated halt). The silent part applies only if the halt is suppressed. The text must name the column that the reset writes for each kind.
- Proposed text: none (DOUBT). Suggested clarification for the lead: section 3.5 says that for `business_date` tables the reset writes both `value` and `value_business_date`, and that `restored_to` for those tables is a date.
- Severity: DOUBT (LOW to MEDIUM).

### ADV-3-09 A `false_alarm` decision may not clear an as-of regression, because the baseline marker stays high   (verdict: DOUBT; phase 11 section 3.5 resume)
- Rule: phase-11.md:949: "`false_alarm` ... Halts are cleared; no watermark moves." Phase-11.md:908: the as-of signal compares the marker with "the marker of the previous unit". Phase-11.md:809: `advance` writes `as_of_marker` only on a successful advance.
- The two readings:
  (a) "Previous unit" means the last successful advance. After a failover that the bank explains as a false alarm (for example, a standby with a lower marker that the bank accepts), the stored baseline stays high. Every later unit with the lower marker raises `AsOfRegressed` again, so the source halts on each tick and never progresses.
  (b) "Previous unit" means the rejected receipt written by the halting unit (phase-11.md:929 writes it with the observed marker). Then the baseline moves to the lower marker and the source runs normally.
- The text does not say which. Reading (a) is a visible loop, not a silent gap, so it is not a CRITICAL matter, but it means the `false_alarm` decision cannot do what it says.
- Proposed text: none (DOUBT). Suggested clarification: section 3.5 `false_alarm` writes the observed marker as the new baseline (`watermarks.as_of_marker` for every table of the source), audited with the check id.
- Severity: DOUBT (MEDIUM if reading a).

### ADV-3-10 A known adjustment accepts later breaks in the same group, whatever their size   (verdict: FLAW; phase 12 sections 3.1 step 7 and 3.5)
- Rule: phase-12.md:677-678: "`confirmed_gap` when `accepted_for(source, object, kind = recon_break)` returns an accepted issue for the same object and group, else `open`." Planning target: an exclusion or known adjustment must not hide a gap it was not approved for.
- Attack:
  1. A steward proposes a known adjustment for a `control_totals` break in group `D|B1|IQD` with a delta of 1,000 IQD. The bank signs the document. The change request is approved (`cr_material`, MLRO and CHK). The issue becomes `accepted` with `accepted_until` = D+30 days (phase-12.md:786-800; `accept` at phase-12.md:527).
  2. On D+3 a 50,000,000 IQD difference appears in the same group (for example, a source posting that never reached canon).
  3. The new break creates a new `open` issue (the partial unique index covers only `open` rows, phase-12.md:207).
  4. Next day run, step 7: the delta is not within tolerance, and `accepted_for(...)` returns the accepted issue, so the result is `confirmed_gap`. Step 8 writes no `open` issue for a `confirmed_gap`.
  5. The ladder's open count and the data-quality register show nothing for the group. The certificate lists the group under gaps (phase-12.md:875-876), so the gap stays visible there with status `pending_attestation`.
- Guard search: the proposal and the activation store `accepted_until`, `signed_document_id` and `change_request_id` only (phase-12.md:482 `AcceptInput`, phase-12.md:579 `KnownAdjustmentProposal`). The amount is not stored or compared. The trait signature `accepted_for(conn, source_id, object, kind)` (phase-12.md:531) has no group key, so the group match in step 7 is not enforced by the trait. The certificate keeps the item visible, which is why the severity is not CRITICAL.
- Severity: HIGH (a gap is shown as explained, outside the open count; visible in the certificate).
- Proposed 7.6 text (phase 12 section 7+):
  "### 7.6 Adjustments bind the amount and the group" - At activation, store the issue's delta in `dq_issues.detail` as `accepted_delta` (the existing JSONB column). `accepted_for` gains a `field` parameter (the group key) in the trait at phase-12.md:531. Step 7 returns `confirmed_gap` only when the accepted issue matches the object and the group key, and the absolute delta is not larger than `accepted_delta`. Anything larger stays `open` as a new issue. Test: a 50,000,000 IQD break in an adjusted group of 1,000 IQD stays `open`.

### ADV-3-11 An exclusion masks a blind period and a stale source in the coverage ledger   (verdict: FLAW; phase 12 section 3.7)
- Rule: phase-12.md:853-855: reason "in this precedence (the first that holds): `excluded` ... `blind_period` ... `source_stale`". Phase 12 section 3.8 lists each of these as its own gap item (phase-12.md:875-876).
- Attack:
  1. Branch B1 is excluded with an active code of family `exclusion` (phase-09.md:739; phase-12 section 3.6), approved when it had zero volume (phase-12.md:821, a dormant branch).
  2. Later the source is halted (`restore_suspected`) or paused for ten days.
  3. The ledger row for B1 shows `reason = excluded` (the first in precedence) and `exclusion_id` set. `blind_period` and `source_stale` are not shown for B1.
  4. The certificate still lists the `blind_period` and `stale` gap items (phase-12.md:875-876), and the statement lists the period (phase-12 section 3.9 step 3), so the gap is visible to the MLRO. The unit's own status says only "excluded".
- Guard search: certificate gaps (phase-12.md:875-876) and statement step 3 (section 3.9) keep the period visible. No guard in the ledger itself.
- Severity: MEDIUM (a person would see the gap in the certificate; the unit's own status is wrong).
- Proposed 7.7 text (phase 12 section 7+):
  "### 7.7 A dark cause outranks an exclusion" - In section 3.7 step 5, reorder the precedence: `blind_period`, `source_stale`, `excluded`, `no_postings_since`, `monitored`. The `exclusion_id` is still written to the row whatever the reason, so the exclusion stays visible. Test: exclude a branch, then halt the source; the ledger row reads `blind_period` and keeps the `exclusion_id`.

## GUARDED items (one line each)
- ADV-3-12 Two `ingest_load` jobs for one table run at once: GUARDED. Coalescing collapses only queued jobs (phase-00.md:120-123), but `lock_table` (phase-11.md:783-785) runs before change detection, so the second job reads committed hashes and writes nothing; the cost is a second source read.
- ADV-3-13 A business-date batch for an earlier date lands after a later one: GUARDED. Units run oldest first (phase-11.md:766), the stored value is compared under the lock (phase-11.md:783-785), and the re-read window (phase-11.md:758-759) brings late rows of closed days back in. Beyond the window, see ADV-3-05.
- ADV-3-14 Extract files delivered twice, out of order or with a changed hash: GUARDED. A duplicate is marked done, the same sequence with another hash is `file_hash_mismatch` (rejected, alarm), and a sequence gap halts with `file_gap` (phase-11.md:892-893).
- ADV-3-15 Delivery mode switch in the middle of a range: GUARDED. The tier and engine never change (phase-05.md:727), delivery is fixed by tier (phase-11.md:850), and the watermark kind must match delivery (phase-11 section 3.4).
- ADV-3-16 Kill switch in the middle of a load: GUARDED. Running jobs end with `Paused` and are not retried (phase-05.md:986); a unit is one transaction or whole page receipts (phase-11.md:819), so partial reads are discarded.
- ADV-3-17 Totals and rows read at different points: GUARDED in part. A skew gives open breaks, not passes (a source row added after the landing read raises the expected figure). A pass needs an identity swap with equal sums; the overlap re-read catches a changed fact only inside the window (see ADV-3-05).
- ADV-3-18 Credential rotation or revoked grant in the middle of a load: GUARDED. Each session passes the grant check (phase-05.md:785 and 977 session age), and a bank-side refusal or pause opens a source blind period (phase-11.md:975-979), so the gap is recorded.
- ADV-3-19 A `row_conflict` issue is resolved and its conflict rows closed without any canon change: GUARDED. The resolution is trusted only until the next day run re-checks the group (phase-12.md:769), and a break that still shows reopens as a new issue (phase-12.md:688), so an insider edit surfaces on the next run.
- ADV-3-20 A restore keeps rows the restored CBS no longer has: GUARDED. Rows are kept in canon and raw (phase-11.md:956; DR-90), and the next day run shows the control-totals and count breaks as open issues.
- ADV-3-21 `data_as_of` is the newest mapped date per branch across all tables: GUARDED. A stalled table can sit behind a newer `X-Data-As-Of`, but `stale` is true when any watermark of the source is stale (phase-11.md:1036 read path), and the certificate lists a stale item (phase-12.md:875).
- ADV-3-22 A declared table with no batch for a business date: GUARDED. A day run covers only dates that have a batch (phase-12.md:648), but a table that is not being read goes stale (phase-11.md:1013-1025 rules b, e and f) and the certificate then lists a stale gap (phase-12.md:875).

## Summary
- Candidates analysed: 22 (ADV-3-01 to ADV-3-22).
- By verdict: FLAW 8 (ADV-3-01, 02, 03, 04, 05, 06, 10, 11); DOUBT 3 (ADV-3-07, 08, 09); GUARDED 11 (ADV-3-12 to 22, one line each above).
- FLAW items by severity: HIGH 7 (ADV-3-01 pause freezes freshness; 02 endpoint change never halts; 03 sequence key commit order; 04 high-water mark below watermark on unvalidated engines; 05 back-dated rows beyond the re-read window with no alarm; 06 back-dated bi-temporal version stalls the table; 10 known adjustment has no amount bound); MEDIUM 1 (ADV-3-11 exclusion masks a dark cause in the ledger). No CRITICAL item.
- DOUBT items, one line each: ADV-3-07 whether a pause or halt blind period with no business date covers the as-of date (HIGH if the answer is "no"); ADV-3-08 whether `reset_and_clear` resets `value_business_date` (a repeated halt if not); ADV-3-09 whether `false_alarm` moves the as-of baseline (a repeated halt if not).
- Open doubts read: phase 10 (Open doubts), phase 11 (Open doubts 1 to 13; doubt 8 is a real flaw left open, see ADV-3-04), phase 12 (Open doubts). Phase 05 has no Decisions or Open doubts section.
- Designed-exception check: DR-39 (research.md:50) and DR-90 (research.md:101) were grepped for pauses and failover; DR-90 names only the two regressions as halt triggers, so ADV-3-02 is not a designed exception, but the lead should resolve the conflict with research 7.5.
