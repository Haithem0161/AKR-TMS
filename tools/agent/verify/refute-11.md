# Refuter, phase 11 (Haiku draft for lead review)

### V-11-01-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The resume route passes the "latest grant check" id to ingest_resume, the job makes a fresh grant check, and the text does not say which check id the halt_resolved audit carries.
- Evidence: docs/plan/phase-11.md:937-938 "the route checks the state and the latest grant check, then enqueues"; docs/plan/phase-11.md:946-947 "a fresh grant check, which is C-03 for a source that was halted"; docs/plan/phase-11.md:959 "audit `source.halt_resolved {decision, tables, check_id}`". The greps found no sentence that says which check id is audited: `check_id` appears only at phase-11.md:938, 959 and 1301 (line 1196 says "check id"). Phase 05 sets no check id input for the guard: docs/plan/phase-05.md:792 "Every physical session goes through `ConnectionGuard::connect(source, trigger)`", and the guard writes its own check row.
- Fix: Phase 11 section 3.5, resume step 4: say whether the audited check_id is the route's latest check or the job's fresh check.

### V-11-01-b  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: A halted resume has no request audit (route accepted, job enqueued); phase 05 names source.resume_requested for the paused branch only.
- Evidence: docs/plan/phase-03.md:706 "`audit_log` event `authz.allowed` or `authz.denied` with the action, the resource"; docs/plan/phase-03.md:708-709 "written by `AuditSink::emit_standalone` in its own short transaction **before** the handler runs". This is the general decision row of the authorize layer, written for every route before the handler runs. The halted resume is a route with the Cedar action `source.resume` (docs/plan/phase-11.md:706 "a halted source resumes through `ingest_resume`"), so the acceptance is audited as an allowed decision.

### V-11-01-c  STANDS   (first agent: LOW; refuter: LOW)
- Claim: Phase 11 names the trigger type `Trigger::Resume`, while phase 05 names the enum `CheckTrigger`.
- Evidence: docs/plan/phase-05.md:527 "trigger: CheckTrigger, by: Option<Uuid>"; docs/plan/phase-05.md:1003 "ConnectionGuard::connect(source, Trigger::Health)"; docs/plan/phase-11.md:946 "ConnectionGuard::connect(source, Trigger::Resume)". Grep `CheckTrigger` in phase-11.md: none. Phase 05 itself uses both spellings, so the name is not settled by phase 05 alone. The variant `resume` is a designed trigger value (docs/plan/phase-05.md:744, "trigger = resume").
- Fix: Phase 11 section 3.5, resume step 2: use one trigger type name and say it is phase 05's `CheckTrigger`.

### V-11-01-d  STANDS   (first agent: LOW; refuter: LOW)
- Claim: The base text says a false_alarm halt moves no watermark, the 7.13 block writes as_of_marker into every watermark, and no 7.x text says 7.13 replaces the base sentence.
- Evidence: docs/plan/phase-11.md:949-950 "Halts are cleared; no watermark moves."; docs/plan/phase-11.md:1606-1607 "writes the marker observed by the halting unit" and "into `as_of_marker` of every watermark of the source". Grep "replaces" and "supersedes" in phase-11.md: none in 7.13. The intent is clear from research.md:420 (DR-409) "`false_alarm` rebases the marker (phase 11)", so the base sentence is stale; the two texts still conflict on the page.
- Fix: Phase 11 section 3.5, resume step 3 false_alarm bullet: say that "no watermark moves" covers the value only and that the marker write of 7.13 applies.

### V-11-02-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: No step creates the admin.watermarks row of a (source, table, branch); the first advance from NULL is an UPDATE that matches no row, so no table can be loaded.
- Evidence: docs/plan/phase-11.md:755 "Read the watermark rows of the table (no lock yet) and the branch dates."; docs/plan/phase-11.md:625 "`advance` is a compare-and-set on `value`"; docs/plan/phase-11.md:112 "NULL until the first unit has been read". Phase 05 says the rows move to phase 11 "where ingestion first writes them (DR-63)" (docs/plan/phase-05.md:44), but nothing says who inserts them. Greps: `INSERT INTO admin.watermarks` in phase-11.md: none; no `insert` or `ensure` method in the WatermarkRepository trait (phase-11.md:545-559); the only `ensure_` methods are for landing tables and partitions (phase-11.md:607-608).
- Fix: Phase 11 section 3.1 step 2 or section 3.2 step 1: name the step that creates the (source, table, branch) row with value NULL, and when it runs.

### V-11-02-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `UnitPlanner::plan` takes no input for `ingest.sequence_overlap_keys` (7.10) or for the late-arrival lag and `ingest.lookback_max_days` (7.12).
- Evidence: docs/plan/phase-11.md:756 "`UnitPlanner::plan(watermarks, kind, overlap, settle, history_from, closed_dates, slice_days, max_unit_rows)`"; docs/plan/phase-11.md:1570 "`ingest.sequence_overlap_keys` (migration `0035`"; docs/plan/phase-11.md:1595-1596 "planner rule a uses the larger of `overlap` and 1.25 times that" and "largest lag, capped at `ingest.lookback_max_days`". Grep: `sequence_overlap_keys` appears only at line 1570; `lookback_max_days` only at 1596 and 1598. A reader can infer the three inputs from 7.10 and 7.12, so the gap is MEDIUM.
- Fix: Phase 11 section 3.1 step 3: add the sequence overlap, the measured lag and the lookback cap to the `plan` inputs, or say that the caller passes an effective overlap.

### V-11-02-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `StaleChange`, `StaleTransition` and `WatermarkWithSource` are named in the repository traits and never defined, with no fields.
- Evidence: docs/plan/phase-11.md:556 "async fn set_stale(&self, tx: &mut Tx, changes: &[StaleChange]) -> Result<Vec<StaleTransition>, RepoError>;"; docs/plan/phase-11.md:558 "async fn list_all_active(&self, conn: &mut PgConnection) -> Result<Vec<WatermarkWithSource>, RepoError>;". Grep for the three names across docs/plan/*.md, docs/*.md and research/*.md: only these two lines. The domain entity list says the fields "mirror section 1" (docs/plan/phase-11.md:518-519), which covers `Watermark` only.
- Fix: Phase 11 section 2 repository notes: give the fields of StaleChange, StaleTransition and WatermarkWithSource, one line each.

### V-11-03-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: The replay POST (akr-api) step 3 calls `LandingStore::estimate`, but section 2 and section 4.3 say only akr-ingestd reads the raw layer, so the 422 E_COST_GUARD answer cannot be built as written.
- Evidence: docs/plan/phase-11.md:1061 "Estimate: `LandingStore::estimate` counts the landed rows of each table in the window"; the route is akr-api (docs/plan/phase-11.md:1056). docs/plan/phase-11.md:1358 "Raw landing and replays read `raw` only inside akr-ingestd"; docs/plan/phase-11.md:605 "The raw landing layer. Only akr-ingestd calls it". Grep `LandingStore::estimate`: lines 619 (trait) and 1061 only. Two texts contradict, and no 7.x block resolves it.
- Fix: Phase 11 section 3.8, POST step 3: name the component that counts the landed rows, since akr-api may not read raw.

### V-11-03-b  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: Apply step 2 writes a replay receipt with the table's `delivery`; for a `files` table `file_sha256` must be NOT NULL, and the text gives no value for it.
- Evidence: docs/plan/phase-11.md:164 "CHECK ((file_sha256 IS NOT NULL) = (delivery = 'files'))"; docs/plan/phase-11.md:1119 "Receipts per page: `delivery` of the table, `mapping_version_id` set, watermark fields NULL". Grep for `file` and `sha` inside the apply steps (phase-11.md:1105-1124): none. No text excludes files tables from replay.
- Fix: Phase 11 section 3.8, apply step 2: state the file name and sha256 of a replay receipt for a files table, or refuse replays of files tables.

### V-11-03-c  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Apply step 2 says "watermark fields NULL", but `admin.batches.watermark_kind` is NOT NULL, and the receipt status of a replay page is not stated.
- Refuted part: the NULL-able move fields are `watermark_from` and `watermark_to` (docs/plan/phase-11.md:145-146, "NULL on the first unit of a table"). `watermark_kind` (docs/plan/phase-11.md:142, "TEXT NOT NULL CHECK (watermark_kind IN") is the table's kind, not a field that a move sets, and the step's parenthesis "(a replay moves no watermark)" (docs/plan/phase-11.md:1119-1120) limits the NULLs to the move fields.
- Stands: the receipt status is not stated for replay pages. Section 3.2 step 6 gives the rule (docs/plan/phase-11.md:805 "`status = mapped` (no quarantined rows), `partial` (some rows quarantined)"), but apply step 2 does not refer to it, and apply step 1 names only steps 2 to 5 (docs/plan/phase-11.md:1116).
- Fix: Phase 11 section 3.8, apply step 2: state the receipt status of a replay page (mapped or partial, by the step 6 rule).

### V-11-03-d  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: `NewReplay`, `ReplayCompletion`, `ReplayDiff`, `ReplayCounts` and `ReplayDiffer` have no field list or signature.
- Evidence: docs/plan/phase-11.md:371 "counts per table: read, unchanged, added, removed, changed per monitored attribute, quarantined; samples are shapes" (the fields of ReplayDiff and ReplayCounts, in the DDL comment); docs/plan/phase-11.md:1065 "Insert the row (`status = queued`, `document_sha256` of the version document, `estimate_rows`)" (the fields of NewReplay); docs/plan/phase-11.md:1086 "Complete: `volume_effect = neutral` when every `changed`, `added` and `removed` count" and docs/plan/phase-11.md:1088 "Status `completed`, `finished_at`" (the fields of ReplayCompletion); docs/plan/phase-11.md:1078 "`ReplayDiffer` (pure) classifies each row" (the method, stated in prose). A type whose fields or methods are stated in prose anywhere in the plan is not a gap.

### V-11-03-e  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: The move from queued to running (started_at) and the per-page cursor saves are mutations with no audit event.
- Evidence: docs/plan/phase-11.md:1202 "`mapping_replay.requested`, `.completed`, `.failed` | replay lifecycle (window, counts, volume effect, error)"; docs/plan/phase-11.md:1071 "`queued` -> `running` (`started_at`)"; docs/plan/phase-11.md:1074 "After each page the `cursor` is stored." The replay row is a run record, and the status move and the checkpoints are job state inside a run that the requested and completed or failed events bracket. That is the housekeeping case the rule describes (a timer row marked fired is the example), covered by a run-level event.

### V-11-03-f  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Apply step 1 runs "section 3.2 steps 2 to 5", where step 2 compares with the last landed hash and step 4 inserts raw rows; the text does not say whether apply lands any raw row.
- Evidence: docs/plan/phase-11.md:1116-1117 "the unit machinery of section 3.2 steps 2 to 5 without the watermark step, in one transaction per page"; docs/plan/phase-11.md:794 "raw rows are never updated" (step 4 inserts landed rows); docs/plan/phase-11.md:1049 "A replay reads the landing layer only, never". Grep `insert_rows` in phase-11.md: lines 612, 628 and 793 only, none in section 3.8. Step 2 compares with the last landed hash per key, so an older landed version inside the replay window would count as changed and be landed again, unless the text says otherwise.
- Fix: Phase 11 section 3.8, apply step 1: say whether apply lands any raw row and which rows it reads (one sentence).

### V-11-03-g  STANDS   (first agent: LOW; refuter: LOW)
- Claim: The failure path (step 7) sets error_code but not finished_at, which the CHECK requires for status failed.
- Evidence: docs/plan/phase-11.md:390 "CHECK (status NOT IN ('completed', 'failed', 'applied') OR finished_at IS NOT NULL)"; docs/plan/phase-11.md:1090 "Failure: status `failed` with `error_code`". Grep `finished_at` in phase-11.md: line 1088 (completion, step 6) only. As written, the step would fail the CHECK.
- Fix: Phase 11 section 3.8, job step 7: add finished_at to the failure update.

### V-11-03-h  STANDS   (first agent: LOW; refuter: LOW)
- Claim: Open doubt 7 says the phase 04 approver port is "not in phase 04 today", but phase 04 section 7.5 defines it and says it closes the doubt.
- Evidence: docs/plan/phase-04.md:1681 "`ChangeRequestService` gains the read method `approver_ids(conn, object_kind, object_key) -> Vec<Uuid>`"; docs/plan/phase-04.md:1685 "(phase 11 Open doubts 7 closed)"; docs/plan/phase-11.md:1633 "that port is not in phase 04 today." The first agent is right that the doubt is stale. The port itself exists, so the dependency is met; only the doubt text is out of date.
- Fix: Phase 11 Open doubts item 7: mark it closed by phase 04 section 7.5.

### V-11-04-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: A system or rail declaration may stay open (ended_at is optional), no step closes it, the automatic close covers source periods only, the partial unique index covers source scope only, and akr_app has no UPDATE on blind_periods, so the period stays open for ever.
- Evidence: docs/plan/phase-11.md:993-994 "optional `ended_at`" (declaration rules); docs/plan/phase-11.md:985 "**Automatic close.** The freshness job (section 3.7) closes an open source period"; docs/plan/phase-11.md:313 "GRANT UPDATE (ended_at, business_to, statement) ON admin.blind_periods TO akr_ingest;" and docs/plan/phase-11.md:314 "GRANT SELECT, INSERT ON admin.blind_periods TO akr_app;". The unique index covers scope source only (docs/plan/phase-11.md:98-99). Phase 12 adds no close route: docs/plan/phase-12.md:1387 "No blind-period table, cause, opening or route (phase 11, section 3.6)". Research DR-90 (research.md:101) says source periods "close at catch-up" and says nothing about closing declared periods. Phase 30 (docs/plan/phase-30.md:230) writes system periods with ended_at already set, so the open case is not covered there.
- Fix: Phase 11 section 3.6: add the close rule, route or job and grant for a system or rail period that was declared without ended_at.

### V-11-04-b  PARTLY   (first agent: MEDIUM; refuter: LOW)
- Claim: Four refusals have no code: started_at in the future, a comment outside 10 to 2,000 characters, ended_at before started_at, and business_to before business_from.
- Refuted part: every body validation already answers 422 `E_VALIDATION` with JSON pointers (docs/plan/phase-00.md:499 "`Validation(Vec<ValidationError>)` | 422 | `E_VALIDATION` (`errors[]` with JSON pointers)"), so the four refusals have a status and a pointer. The two date-order rules are database CHECKs (docs/plan/phase-11.md:93 "CHECK (ended_at IS NULL OR ended_at >= started_at),"; docs/plan/phase-11.md:94 "CHECK (business_to IS NULL OR business_from IS NULL OR business_to >= business_from),").
- Stands: the reason strings of the four refusals are not named. The E_VALIDATION row lists only other reasons (docs/plan/phase-11.md:1173 "`E_VALIDATION` | 422 | every POST | `range_inverted`, `range_too_long`"). Greps for "future", "comment", "ended_before" and "business_to_before" near that row: none. Severity LOW.
- Fix: Phase 11 section 3.10, E_VALIDATION row: add reason strings for the four refusals.

### V-11-04-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The declaration has no service method: the route handler names no call and BlindPeriodService has no methods.
- Evidence: docs/plan/phase-11.md:968 "### 3.6 Blind periods (`BlindPeriodService`; C-27, SP-87)"; docs/plan/phase-11.md:1339 "`blind: Arc<BlindPeriodService>` (declarations)"; docs/plan/phase-11.md:511 lists the service in the ingestd type list. Grep `BlindPeriodService` in phase-11.md: lines 511, 968 and 1339 only, with no method anywhere. The declaration rules (docs/plan/phase-11.md:991-996) are prose and name no method. The repository trait has `open` (docs/plan/phase-11.md:588), so the step is inferable, which keeps the gap at MEDIUM.
- Fix: Phase 11 section 3.6: name the BlindPeriodService method that the declaration route calls and give its steps.

### V-11-04-d  STANDS   (first agent: MEDIUM; refuter: HIGH)
- Claim: The register read GET /coverage/blind-periods is scoped to dashboard.read only, but Cedar blind_period.read also permits DS and section 3.6 says the steward reads the register.
- Evidence: docs/plan/phase-11.md:701 "| GET | /coverage/blind-periods | dashboard.read |"; docs/plan/phase-11.md:998 "The register is read by the MLRO, the compliance head, internal audit and the steward (Cedar"; docs/plan/phase-11.md:1214 "| `freshness.read`, `blind_period.read` | System, Freshness, BlindPeriod | M, CH, IA, DS |". The DS role does not hold dashboard.read (docs/API.md:99 "| `dashboard.read` | Command centre, metrics, board packs | M, CH, IA |"; docs/API.md:97 "| `data.steward` | ... | DS |"). The sibling GET /freshness admits data.steward (docs/plan/phase-11.md:700 "| GET | /freshness | dashboard.read, data.steward |"), and docs/API.md:330 repeats the dashboard.read-only scope. Section 3.6 and the route table contradict each other, and no 7.x block says which wins, so the severity is HIGH.
- Fix: Phase 11 route table row at line 701: add data.steward to the scope, and the matching API.md row at line 330.

### V-11-04-e  REFUTED   (first agent: LOW; refuter: none)
- Claim: "Flagged with their creation time" for alerts of phases 16 and 17 is not defined in those phases.
- Evidence: docs/plan/phase-16.md:1236-1237 "a window overlapping a source-scope or system-scope blind period carries its ids. The alert is still raised; the period is shown with it" (the flag: the blind period ids travel with each alert, and phase 17 requires them in the alert's coverage, docs/plan/phase-17.md:935-936). The creation time is the alert's own `created_at` (docs/plan/phase-17.md:174 "ON alert.alerts (priority_class, created_at)"), so a late alert shows when it was created. Both parts of the requirement have a home in phases 16 and 17.

### V-11-05-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `SpoolSegment`, `SpoolError`, `ChangeEvent` and `ChangeCaptureConsumer` have no fields or methods, so the spool reader and the consumer cannot be coded from the text.
- Refuted part: the SpoolSegment fields are stated in prose: docs/plan/phase-11.md:871 "The file name carries the first and last position and a sequence" (plus the `.sha256` file beside it, same paragraph). The ChangeEvent fields are stated in prose: docs/plan/phase-11.md:877 "Each event carries `op` (`c`, `u`, `d`), the key tuple, the `after` image and its position."
- Stands: `SpoolError` has no variant list. The variants are implied by the rules of docs/plan/phase-11.md:873-876 (sequence gap, repeated sequence with another hash, SHA-256 mismatch), but nothing names them. `ChangeCaptureConsumer` has no entry method: it appears only in the type lists (docs/plan/phase-11.md:511 and 1342). Greps: `enum SpoolError` and `ChangeCaptureConsumer::` in phase-11.md: none. The ports name the signatures only (docs/plan/phase-11.md:650-651 "-> Result<Vec<SpoolSegment>, SpoolError>").
- Fix: Phase 11 section 2 ChangeSpool port: list the SpoolError variants and name the entry method of ChangeCaptureConsumer.

### V-11-05-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The planner has no rule for `cdc_position`. Rules a to d cover the date, timestamp, rowversion and sequence kinds, and the CT3 read is "the segments after the stored position" with no start, cut or end rule.
- Evidence: the start rule names only the date, timestamp, rowversion and sequence kinds (docs/plan/phase-11.md:758-761), and its no-value case is "With no value yet: `ingest.history_from`" (docs/plan/phase-11.md:761), which has no meaning for a position. The cut rule covers ranges only: docs/plan/phase-11.md:766 "c. Slices. A range longer than `ingest.slice_days` is cut into slices". The CT3 read is docs/plan/phase-11.md:873 "2. `ingest_load` for a `cdc_position` table reads the segments after the stored position". Grep `cdc_position` inside section 3.1 (phase-11.md:723-767): none. Lines 111, 143, 524, 873 and 1004 are the only other hits.
- Fix: Phase 11 section 3.1 step 3: add a rule for `cdc_position`, covering the start with no stored position and how many segments make one unit.

### V-11-05-c  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: No rule gives the idle poll interval for CT3. A 60-minute heartbeat conflicts with the 900-second F1 limit.
- Refuted part: the interval is given. Rule 2c applies to every declared table, so an idle CT3 table is due after 60 minutes: docs/plan/phase-11.md:740 "c. the last successful read (`advanced_at`) is older than 60 minutes".
- Stands: that interval contradicts the F1 limit. docs/plan/phase-11.md:1016 "c. F1: `now - advanced_at` above `freshness.f1_lag_seconds` -> stale, `lag_ceiling`." with docs/plan/phase-11.md:443 "('freshness.f1_lag_seconds', '900'". The only faster re-poll is "30 seconds after a unit that read events" (docs/plan/phase-11.md:881), which covers busy units only. Grep `idle` in phase-11.md: none. MEDIUM.
- Fix: Phase 11 section 3.4 CT3 step 4: give the idle poll interval for `cdc_position` tables that meets the 900-second F1 limit.

### V-11-05-d  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: POST /sources must accept CT3 when the flag is on, but phase 05 still refuses CT3 and no 7.x block of phase 05 changes it.
- Evidence: the change is designed. research.md:74 (DR-63) "CT3 is refused at registration until then" (until phase 11 ships). Phase 11 carries the change in section 4.4: docs/plan/phase-11.md:1375-1376 "`POST /sources` accepts CT3 only with the flag". The phase 05 refusal is the interim state and cites DR-63 itself: docs/plan/phase-05.md:712-713 "`tier` CT1, CT2 or CT4, CT3 -> 422 `E_VALIDATION` `tier_not_available` (DR-63)". Phase 05 has no contrary design text, only the interim one. The planning template puts updates to earlier phases in section 4 (`Existing entity/schema/route updates required by this phase`). Phase 05 still reads as refused, so a note there would stop drift; that is a documentation point, not a gap in the design.

### V-11-05-e  PARTLY   (first agent: MEDIUM; refuter: LOW)
- Claim: The signed-agreement precondition of CT3 (docs/plan/phase-11.md:851) and of the flag (docs/plan/phase-11.md:467, 1329) has no record, column, check or change request in phases 00 to 05.
- Refuted part (the record): a record mechanism exists. Signed documents are uploaded through `POST /sources/{id}/documents` with a `SourceDocumentKind` (docs/plan/phase-05.md:750), and that kind list ends with `other` (docs/plan/phase-05.md:503-504 "`SourceDocumentKind` (`budget_signed`, `access_approval`, `exception_signed`," / "`other`)."). Each flag change is audited (docs/plan/phase-03.md:1053 "`setting.updated` and `flag.updated` (phase 00)").
- Stands (the check): nothing ties CT3 creation or the flag to a document that evidences the agreement. Phase 11 states the precondition only in prose: docs/plan/phase-11.md:851 "(only with `ingest.cdc_enabled` and a signed agreement)". Grep `agreement` in docs/plan/phase-00.md to phase-05.md: none. LOW.
- Fix: Phase 11 section 3.4: name the document kind that evidences the agreement, or say that the audited flag change is the record.

### V-11-05-f  STANDS   (first agent: LOW; refuter: LOW)
- Claim: A CT3 or CT4 gap halt is audited as `source.restore_suspected`, which is the event for restore or failover signals.
- Evidence: docs/plan/phase-11.md:934 "**Other halts.** `cdc_gap` and `file_gap` (section 3.4) use the same path"; docs/plan/phase-11.md:926 "Audit `source.restore_suspected {signal, observed, expected, cause}`"; docs/plan/phase-11.md:1194 "| `source.restore_suspected` | a restore or failover signal (signal, observed, expected) | source |". Grep `cdc_gap` in the 3.10 audit rows (phase-11.md:1189-1203): none. An audit event is named, so the gap is the event name only, which is LOW.
- Fix: Phase 11 section 3.5, "Other halts": name the audit kind for `cdc_gap` and `file_gap` halts.

### V-11-06-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The value set of `Quarantine.gate` is not listed, and the card-guard derivation is not written. Section 7.3 gives only the transaction-type case, while the phase 12 tests use a card-guard gate row.
- Evidence: docs/plan/phase-11.md:1487-1488 "the Cedar entity `Quarantine` gains `gate: String`" and "derived and not stored (a row held for a transaction-type code"; docs/plan/phase-12.md:948 "`card_guard_not_excludable` for a card reason (card data is fixed at source)"; docs/plan/phase-12.md:1237 "exclusion of a `txn_type` gate row and of a `card_guard` row is refused". The gate concept comes from docs/DSL.md:635 "For kinds marked `gate: quarantine` (default for `txn_type`)". Greps: no list of gate values in phase-11.md or phase-12.md (phase-12.md:990 and 1041 use `gate` only as a Cedar attribute). The card reason has a refusal but no gate value or derivation.
- Fix: Phase 12 section 3.11: list the gate values and state the rule that derives the card-guard gate from the reason code.

### V-11-06-b  REFUTED   (first agent: LOW; refuter: none)
- Claim: `QuarantineFilter` is named and extended but has no field list in phase 11.
- Evidence: docs/plan/phase-11.md:697 "`kind?`, `status?`, `source_id?`, `source_table?`; keyset (`quarantine.read`)" (the base filters, in the route table); docs/plan/phase-11.md:1489-1490 "`QuarantineFilter` gains" and "`owner_id`, `older_than_days` and the order `oldest`" (the extension). The fields are stated in a table and in prose, which the rule counts as stated.

### V-11-07-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: The landing_file columns are "the list pinned in the adapter pack", but section 3.9 pins no extract columns; the pack covers only ICS BANKS database objects, so a landing_file table cannot be parsed.
- Evidence: docs/plan/phase-11.md:895-896 "parses the rows with the column list" / "pinned in the adapter pack"; docs/plan/phase-11.md:1505 "the columns are the list pinned in the adapter pack (section". Section 3.9 starts at docs/plan/phase-11.md:1126 "### 3.9 The first adapter pack: ICS BANKS on Oracle" and covers database objects and deny-list columns only. docs/DSL.md:516 "`landing_file` (CT4 contract with manifest, C-39)" names no columns. Greps: "column list" in phase-11.md: line 895 only; "pinned": 896 and 1505 (494 is unrelated). No CT4 column layout in research/ or docs/.
- Fix: Phase 11 section 3.9: add the column list of each extract table the bank delivers, or name the file that pins it.

### V-11-07-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The record-text column is named "in the mapping's `source` block", but phase 10's source block has only object, format and key, and unknown keys are refused.
- Evidence: docs/plan/phase-11.md:1507-1508 "an approved object with one key column and one record-text column, both named in the mapping's" / "`source` block". The DSL source block has four keys: docs/DSL.md:517 "| `source.key` | string[] | yes |" (with `source.source`, `source.object` and `source.format` above it). docs/plan/phase-10.md:1814 "Unknown keys are refused, not ignored." Greps: `source.record` in phase-10.md: none; "record" inside the compile steps (phase-10.md:1813-1839): none.
- Fix: DSL.md section B.1 source block and phase 10 section 3 step 3: add the key that names the record-text column of a `t24_xmlrecord` source.

### V-11-07-c  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `ExtractInbox` has no row reader, and the manifest types are undefined, so "parses the rows" has no method.
- Refuted part (types): the manifest fields are stated in prose: docs/plan/phase-11.md:888 "Its manifest (schema `akr.extract.v1`) names `source_code`" and docs/plan/phase-11.md:890 "the control totals and `produced_at`". `ExtractManifest` is a named return type (docs/plan/phase-11.md:658 "fn read_manifest(&self, m: &ManifestRef) -> Result<ExtractManifest, InboxError>;").
- Stands (row reader): no method reads the rows. docs/plan/phase-11.md:895-896 "parses the rows with the column list" names no method. Grep `read_rows` and `parse_rows` in docs/plan/*.md: none. MEDIUM.
- Fix: Phase 11 section 2, `ExtractInbox` port: add the row reader method with its return type.

### V-11-07-d  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: A t24_xmlrecord or t24_multivalue record that fails to decode, and a landing_file row that fails the column check, have no outcome: no quarantine kind, reason code or receipt status.
- Refuted part (t24): the record fields are read with phase 10's transform (docs/plan/phase-11.md:1508 "fields are read with phase 10's `XmlField` transform (DSL `xml_field`)"). A transform error is a row failure with a reason code (docs/plan/phase-11.md:797 "row failures (transform error, required attribute missing, unmapped currency: reason codes from phase 10)"), so an undecodable record becomes a `row_failed` quarantine row.
- Stands (landing_file): the only outcome written is for control totals (docs/plan/phase-11.md:896-897 "A mismatch rejects the batch with reason `control_total_mismatch`"), which does not cover a row that fails the column check. Grep `decode`: docs/plan/phase-11.md:1512-1513 only. No quarantine kind or receipt status is given for a landing_file row. MEDIUM.
- Fix: Phase 11 section 3.4, extract step 3: state whether a row that fails the column check rejects the batch or becomes a `row_failed` quarantine row.

### V-11-08-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: No parser signature, result type or `from` grammar exists for an ISO 20022 element path or an MT field tag.
- Evidence: docs/plan/phase-11.md:1520-1521 "(an ISO 20022 element path, or an MT field tag)"; docs/DSL.md:516 "`iso20022`, `mt` (raw message parsed by a versioned parser, C-41)"; docs/plan/phase-10.md:1821 "4. **Columns.** Each `from` names a column of the latest profile of the object", which covers relational columns only. Greps: "element path" in docs/plan/phase-10.md: none; "field tag": none; a parser struct or `fn parse` in docs/plan/phase-11.md: none.
- Fix: Phase 11 section 7.6 step 2, or DSL.md section B.3: give the `from` grammar for `iso20022` and `mt` entries, plus the parser signature and result type.

### V-11-08-b  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The parser version that "the compiler" writes into the compiled mapping has no compile rule or column in phase 10.
- Refuted part (design): the parser version is part of the compiled mapping by decision: research.md:404 "the parser version is part of the compiled mapping, so `mapping_version_id` records it on every row"; the mechanism is docs/plan/phase-11.md:1523 "Parser version: each parser has a version constant that the compiler writes into the compiled mapping".
- Stands (phase 10): the compile steps (docs/plan/phase-10.md:1813-1839, steps 1 to 12) have no parser-version entry. Grep `parser` in docs/plan/phase-10.md: line 1554 (SQL dialect parsers, unrelated), 2382, and 2459 "no ISO 20022 or MT parser (phases 11 and". MEDIUM.
- Fix: Phase 10 section 3, compile step 3 or a 7.x block: add the parser-version write for `iso20022` and `mt` mappings.

### V-11-09-a  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: The gateway database is a new network edge read from akr-ingestd, but no zone row or deploy/zones.yaml entry names it.
- Evidence: docs/ARCHITECTURE.md:124 "| Z0 CBS network | CBS database, replica, payment gateway database (bank-owned) | `akr-ingestd` only, to listener ports of bank-owned views |"; docs/ARCHITECTURE.md:125 "Z0 listener ports" (the Z1 row, which akr-ingestd sits in, reaches them). The gateway is therefore in the existing CBS zone and needs no new edge. docs/plan/phase-05.md:1356 "the CBS listener must be declared in `deploy/zones.yaml` at install" applies to a gateway listener the same way.

### V-11-09-b  REFUTED   (first agent: LOW; refuter: none)
- Claim: `admin.sources.tier` is NOT NULL with four values, and section 7.7 does not say which tier the gateway uses.
- Evidence: the tier is a registration input for every source: docs/plan/phase-05.md:712 "`tier` CT1, CT2 or CT4, CT3 ->". Section 7.7 makes the gateway "its own `admin.sources` row": docs/plan/phase-11.md:1533 "A payment-gateway database is its own `admin.sources` row with its own grant proof and load budget (phase 05)". The bank chooses the tier at registration, like any source, so phase 11 need not fix it.

### V-11-11-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `business_from` for a pause or refusal of a source with no `value_business_date` (refused from registered, for example) has no value; the rule takes the earliest value plus one day, and the null case is not written.
- Evidence: docs/plan/phase-11.md:1563 "set `business_from` as the halt path does (the earliest `value_business_date` of the source plus one day)". The halt path always has a value, so the rule has nothing to take when none was read. The column is nullable: docs/plan/phase-11.md:86 "-- first business date the gap may cover (source scope)". Phase 12 reads NULL as covering from the start (docs/plan/phase-12.md:1337 "a blind period covers the as-of date `d` when its `business_from` is NULL or on or before `d`"), but phase 11 never says NULL is the value. research.md:420 (DR-409) says the period "has a business_from, with a stated cover rule (phases 11, 12)". Grep `business_from` in phase-11.md: 17 lines, none states the null case.
- Fix: Phase 11 section 7.9: state the `business_from` value for a source that has no `value_business_date` (NULL, or another value).

### V-11-11-b  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: Phase 05 never opens a blind period on pause, bank pause or refusal, but phase 11 section 4.4 says it does and section 7.9 relies on it.
- Evidence: the change is designed in phase 11: docs/plan/phase-11.md:1381-1382 "`POST /sources/{id}/pause` and the bank pause and refusal transitions open a source blind period in their transaction (section 3.6)." The automatic open is specified in section 3.6: docs/plan/phase-11.md:975 "A phase 05 pause (`POST /sources/{id}/pause`, any cause): cause `source_paused`". The decision is research.md:101 (DR-90): "source blind periods open automatically on pause, bank lock, refusal, halt and delivery gap and close at catch-up". Phase 05 has no text on blind periods (grep "blind" in docs/plan/phase-05.md: none), so nothing contradicts the design.

### V-11-12-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: The late-arrival alert that section 7.12 says the freshness job raises is not in section 4.3, so the steward is never alerted.
- Evidence: docs/plan/phase-11.md:1593-1594 "audit `ingest.late_arrival_near_window ...` once per table and day, and the alert rule of section 4.3."; docs/plan/phase-11.md:1356-1357 "**Alert rules:** a source halted for more than one hour" (the full list has no late-arrival rule); docs/plan/phase-11.md:1599-1600 "the alarm is how a person learns the tail is near it"; research.md:420 (DR-409) "late arrivals are measured and alarmed". Greps: `late_arrival` in phase-11.md section 4 (lines 1287-1400): none; `near_window` in the alert list: none. The text relies on an alert that is not written.
- Fix: Phase 11 section 4.3, alert rules: add a rule for `ingest.late_arrival_near_window`.

### V-11-12-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Two settings (`ingest.late_alarm_pct`, `ingest.lookback_max_days`) are stated as additions but have no INSERT row or label.
- Evidence: docs/plan/phase-11.md:1598 "Migration `0035` adds `ingest.late_alarm_pct` (integer, default 80) and `ingest.lookback_max_days` (integer, default 93), both governed." The section 1.3 INSERT block (docs/plan/phase-11.md:434-465) has neither key, and neither has a label. The same form applies to `ingest.sequence_overlap_keys` in 7.10 (docs/plan/phase-11.md:1570), which the first agent did not list.
- Fix: Phase 11 section 1.3: add the INSERT rows with labels in both languages for the two keys, and for `ingest.sequence_overlap_keys`.

### V-11-12-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: "Once per table and day" has no storage key, no caller is named for `ingest.lookback_widened`, and the planner is pure.
- Evidence: docs/plan/phase-11.md:1594 "once per table and day" (no key, ledger or column named); docs/plan/phase-11.md:1596 "a change audits `ingest.lookback_widened {source_id, source_table,"; docs/plan/phase-11.md:757 "(pure). The rules:" (the planner is pure, so it cannot write the audit). Grep `once per table` and `lookback_widened`: one hit each, both in 7.12.
- Fix: Phase 11 section 7.12, items 2 and 3: name the caller that writes `ingest.lookback_widened`, and the storage key that makes the near-window audit once per table and day.

### V-11-12-d  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The two kinds named in section 7.12 are not in the phase 11 audit table.
- Evidence: the audit table (docs/plan/phase-11.md:1189-1203) has no row for `ingest.late_arrival_near_window` or `ingest.lookback_widened`; both names appear only at docs/plan/phase-11.md:1593-1600. docs/plan/phase-02.md:1224 "Each phase's audit table gives, for every kind, its resource type, severity, SIEM flag and detail keys"; docs/plan/phase-02.md:1226 "A test serialises a sample of every kind and fails on a detail key outside the catalogue".
- Fix: Phase 11 section 3.10 audit table: add both kinds with resource type, severity, SIEM flag and detail keys.

### V-11-12-e  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Phase 10 has no change-strategy rule for `lookback: auto` or a duration, so a compile of a mapping that uses `lookback` has no defined check; only `watermark.kind`, `overlap` and `deletes` are named.
- Refuted part (value shape): the schema pass checks the body against the DSL schema, which fixes `lookback` to `auto` or a duration: docs/plan/phase-10.md:1813 "The body validates against `schemas/mapping.v1.json` (DSL.md Part B)."; docs/DSL.md:534 "| `lookback` | `auto` or a duration |".
- Stands (semantics): the DSL rule that a lookback is "never narrower than the source's trailing re-read window" (docs/DSL.md:534) has no compile check. Step 11 checks `overlap` against `mapping.default_overlap_days` (docs/plan/phase-10.md:1839-1841) and says nothing about `lookback`. Grep `lookback` in docs/plan/phase-10.md: none. MEDIUM.
- Fix: Phase 10 section 3, compile step 11: add the check that a duration `lookback` is not narrower than the overlap.

### V-11-12-f  STANDS   (first agent: LOW; refuter: LOW)
- Claim: The settings count (fifteen) and the governed list in section 3.10 and section 6 predate sections 7.10 and 7.12.
- Evidence: docs/plan/phase-11.md:1276 "The fifteen keys of migration 0035"; docs/plan/phase-11.md:1283 "governed keys (`ingest.history_from`, `ingest.settle_seconds`, the five `freshness.*` thresholds" (the list omits `ingest.sequence_overlap_keys`, `ingest.late_alarm_pct` and `ingest.lookback_max_days`); docs/plan/phase-11.md:487 "| 15 settings, 2 flags | rows | 0035 | n/a | n/a |"; docs/plan/phase-11.md:1437 "Settings: `make seed` installs 15 settings and 2 flags". The counts are stale.
- Fix: Phase 11 sections 1.4, 3.10 and 6 item 4: update the count and the governed list for the keys of 7.10 and 7.12.

## Summary

- REFUTED: 9
- STANDS: 26
- PARTLY: 9

Total claims decided: 44 (each Gap ID handled once).

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-11-01-a | STANDS | MEDIUM | Which check id (the route's latest or the job's fresh one) the halt_resolved audit carries is not stated | Phase 11 section 3.5, resume step 4 |
| V-11-01-c | STANDS | LOW | Trigger type named `Trigger::Resume` in phase 11; phase 05 names the type `CheckTrigger` | Phase 11 section 3.5, resume step 2 |
| V-11-01-d | STANDS | LOW | Base text "no watermark moves" on false_alarm conflicts with the 7.13 marker write; no 7.x text says which wins | Phase 11 section 3.5, resume step 3 |
| V-11-02-a | STANDS | HIGH | No step creates the admin.watermarks row; the first advance matches no row | Phase 11 section 3.1 step 2 or 3.2 step 1 |
| V-11-02-b | STANDS | MEDIUM | UnitPlanner::plan has no inputs for sequence_overlap_keys, the measured lag or lookback_max_days | Phase 11 section 3.1 step 3 |
| V-11-02-c | STANDS | MEDIUM | StaleChange, StaleTransition and WatermarkWithSource are named and never defined | Phase 11 section 2 repository notes |
| V-11-03-a | STANDS | HIGH | The akr-api replay route calls LandingStore::estimate, but only akr-ingestd may read raw | Phase 11 section 3.8, POST step 3 |
| V-11-03-b | STANDS | HIGH | A replay receipt for a files table needs file_sha256 NOT NULL; no value is given | Phase 11 section 3.8, apply step 2 |
| V-11-03-c | PARTLY | MEDIUM | The receipt status of a replay page is not stated (the watermark_kind part is refuted) | Phase 11 section 3.8, apply step 2 |
| V-11-03-f | STANDS | MEDIUM | Whether apply lands any raw row, and which rows it reads, is not stated | Phase 11 section 3.8, apply step 1 |
| V-11-03-g | STANDS | LOW | The failure path (step 7) does not set finished_at, which the CHECK requires | Phase 11 section 3.8, job step 7 |
| V-11-03-h | STANDS | LOW | Open doubt 7 still says the phase 04 approver port is missing; phase 04 7.5 defines it | Phase 11 Open doubts item 7 |
| V-11-04-a | STANDS | HIGH | A declared system or rail period with no ended_at has no close rule, route, job or grant | Phase 11 section 3.6 |
| V-11-04-b | PARTLY | LOW | The reason strings of the four refusals are not named (the 422 E_VALIDATION code is covered by phase 00) | Phase 11 section 3.10, E_VALIDATION row |
| V-11-04-c | STANDS | MEDIUM | The declaration names no BlindPeriodService method | Phase 11 section 3.6 |
| V-11-04-d | STANDS | HIGH | GET /coverage/blind-periods is scoped to dashboard.read, which excludes DS, but section 3.6 says the steward reads it | Phase 11 route table line 701; API.md line 330 |
| V-11-05-a | PARTLY | MEDIUM | SpoolError variants and the ChangeCaptureConsumer entry method are not named (SpoolSegment and ChangeEvent fields are stated in prose) | Phase 11 section 2, ChangeSpool port |
| V-11-05-b | STANDS | MEDIUM | No planner rule for cdc_position: start with no stored position, and unit size | Phase 11 section 3.1 step 3 |
| V-11-05-c | PARTLY | MEDIUM | The 60-minute idle interval (rule 2c) conflicts with the 900-second F1 limit (the "no interval" part is refuted) | Phase 11 section 3.4, CT3 step 4 |
| V-11-05-e | PARTLY | LOW | Nothing ties CT3 creation or the flag to a document that evidences the signed agreement (a document kind and an audited flag change exist) | Phase 11 section 3.4 |
| V-11-05-f | STANDS | LOW | cdc_gap and file_gap halts use the source.restore_suspected audit kind | Phase 11 section 3.5, "Other halts" |
| V-11-06-a | STANDS | MEDIUM | The value set of Quarantine.gate is not listed, and the card-guard gate derivation is not written | Phase 12 section 3.11 |
| V-11-07-a | STANDS | HIGH | The landing_file column list is not pinned anywhere; section 3.9 covers ICS BANKS database objects only | Phase 11 section 3.9 |
| V-11-07-b | STANDS | MEDIUM | The record-text column has no key in the DSL source block; phase 10 refuses unknown keys | DSL.md section B.1; phase 10 section 3 step 3 |
| V-11-07-c | PARTLY | MEDIUM | No row-reader method on ExtractInbox (manifest types are stated in prose, refuted) | Phase 11 section 2, ExtractInbox port |
| V-11-07-d | PARTLY | MEDIUM | A landing_file row that fails the column check has no outcome (t24 decode failures are covered as row failures) | Phase 11 section 3.4, extract step 3 |
| V-11-08-a | STANDS | MEDIUM | No from grammar, parser signature or result type for ISO 20022 element paths or MT field tags | Phase 11 section 7.6 step 2, or DSL.md section B.3 |
| V-11-08-b | PARTLY | MEDIUM | Phase 10 compile has no parser-version rule (the design is in research DR-393 and phase 11 7.6 step 3) | Phase 10 section 3, compile step 3 |
| V-11-11-a | STANDS | MEDIUM | business_from has no value for a source that never had a business date read | Phase 11 section 7.9 |
| V-11-12-a | STANDS | HIGH | The late-arrival alert rule that 7.12 cites is missing from section 4.3 | Phase 11 section 4.3 |
| V-11-12-b | STANDS | MEDIUM | Two settings (plus sequence_overlap_keys) lack INSERT rows and labels | Phase 11 section 1.3 |
| V-11-12-c | STANDS | MEDIUM | "Once per table and day" has no storage key; no caller for ingest.lookback_widened | Phase 11 section 7.12, items 2 and 3 |
| V-11-12-d | STANDS | MEDIUM | The two 7.12 audit kinds are missing from the section 3.10 audit table | Phase 11 section 3.10 |
| V-11-12-e | PARTLY | MEDIUM | No compile check that a duration lookback is not narrower than the overlap (the format check is covered by the schema pass) | Phase 10 section 3, compile step 11 |
| V-11-12-f | STANDS | LOW | The settings count (fifteen) and the governed list predate 7.10 and 7.12 | Phase 11 sections 1.4, 3.10 and 6 item 4 |
