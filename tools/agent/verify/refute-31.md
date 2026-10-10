# Refuter, phase 31 (Haiku draft for lead review)

### V-31-01-a  PARTLY   (first agent: LOW; refuter: LOW)
- Claim: "passed_first" has no defined course-level meaning; section 5.5 shows it as "passed on first attempt" beside the counts.
- Evidence: phase-31.md:83 "bool_or(f.passed)                      AS passed_first," defines the flag inside the view train.practice_scores (:75-91), so the "no text defines it" part is wrong. The part that stands: phase-31.md:952 "passed on first attempt" with the number of cases passed first time out of the cases tried. The flag is TRUE when any one case passed, and no line reconciles that with the label.
- Fix: section 5.5 (phase-31.md:952) or section 3.2 rule 4 (:503): one sentence saying that the label means at least one case was passed first time, or change the label to match the count.

### V-31-02-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: ClassAggregate, the return type of CapacityRepository::window_aggregate, is named but never defined and its fields are not given.
- Evidence: grep -rn "ClassAggregate" docs/ research/ finds only phase-31.md:430 "window_aggregate(from, to) -> Vec<ClassAggregate>" (sums by pool, work kind and class; never by person). No DTO line in section 2.1 (:419-425) names it. The nearest text is the forecast step 1 at phase-31.md:534 "m_pc = handling_minutes_sum / decided", which uses summed fields but does not name them as the struct's fields, so the claim stands at the first agent's severity.
- Fix: phase 31, section 2.1 (after the CapacitySnapshot line at :420): list the fields of ClassAggregate as the per-pool, work-kind and class sums that step 1 of section 3.4 reads.

### V-31-02-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the hourly upsert of casework.capacity_snapshots writes a table and names no audit event.
- Evidence: phase-31.md:526 "A re-run replaces the values of the bucket and does nothing else." and the heading at phase-31.md:722 "### 3.14 Audit events (one per mutation; payloads hold ids, codes and counts only)". The 3.14 table (:724-740) has no snapshot kind. No general rule covers it: the only unaudited system write in phase 02 is anchoring (phase-02.md:777 "Anchoring writes no `audit_log` row per anchor"), and the snapshot is not housekeeping of the kinds named in the planning rule. Greps: `grep -rn "snapshot" research/*.md docs/plan/research.md` show no decision that the upsert is unaudited; DR-341 (research.md:352) covers only the no-person rule.
- Fix: phase 31, section 3.3 step 3 or the 3.14 table: say either that the hourly upsert is a derived rebuild with no audit event (as phase 02 does for anchors), or name one run-level kind for it.

### V-31-02-c  STANDS   (first agent: MEDIUM; refuter: HIGH)
- Claim: the 2.3 signature takes a ForecastQuery with handling_pct and arrivals_pct, but the 3.4 step 7 grid is fixed and no step reads those fields.
- Evidence: HIGH by the rubric (two texts contradict). phase-31.md:437 "`forecast(ctx, query: ForecastQuery { handling_pct, arrivals_pct })`"; phase-31.md:540 "Sensitivity (fixed grid, not a user input)". grep -rn "ForecastQuery\|handling_pct\|arrivals_pct" docs/ research/ finds only phase-31.md:437. The route row at phase-31.md:458 names no query parameter. research/28-case-management-workflow.md:809 gives the same fixed set of cases (+20 percent, +25 percent), and no decision (research.md, phase 31 Decisions) gives the user multipliers. The claim holds in full.
- Fix: phase 31, section 2.3 (phase-31.md:437): remove the query argument so the signature matches the fixed grid of section 3.4 step 7, or, if the bank wants its own multipliers, add them as a named input of step 7 and the route row.

### V-31-02-d  STANDS   (first agent: LOW; refuter: LOW)
- Claim: step 3 writes NewAlarm with subject: ('pool', pool_id), but the alarm table and phase 22 use subject_kind and subject_id.
- Evidence: phase-31.md:548 "`AlarmService::raise(tx, NewAlarm { kind: capacity_forecast, subject: ('pool', pool_id), stage: NULL })`". The column pair is phase-19.md:303-304 "subject_kind        TEXT NOT NULL CHECK" and "subject_id          UUID NOT NULL". The call shape is phase-22.md:564 "NewAlarm { kind: liaison_report_overdue, subject_kind: liaison_report, subject_id: report_id". No phase defines the NewAlarm fields (grep "NewAlarm" in docs/plan finds only phase-19.md:1004, :1299, phase-22.md:564, phase-31.md:548), so the tuple is a naming difference only, and LOW stands.
- Fix: phase 31, section 3.4 alarm step 3 (phase-31.md:548): write the subject as subject_kind = pool and subject_id = pool_id, the names phase 22 uses.

### V-31-03-a  STANDS   (first agent: LOW; refuter: LOW)
- Claim: fact F4 names ref.lists as the source of the activation versions, but loaded_at and activated_at are columns of ref.list_versions.
- Evidence: phase-31.md:587 "per list in `ref.lists`, the versions activated in the period, and for each the hours from `loaded_at` to `activated_at` (phase 14)". The columns are phase-14.md:115 "loaded_at           TIMESTAMPTZ," and phase-14.md:116 "activated_at        TIMESTAMPTZ," inside ref.list_versions (created at phase-14.md:92). ref.lists is the registry (phase-14.md:65). The text groups by list and so is not wrong in substance, but it never names the version table, so the claim holds at LOW.
- Fix: phase 31, section 3.7 fact F4 (phase-31.md:587): name ref.list_versions as the source of the two timestamps and the join to ref.lists for the list.

### V-31-03-b  PARTLY   (first agent: LOW; refuter: LOW)
- Claim: POST /packs for correspondent and validator has no written request body (period, lang) or 202 response.
- Evidence: the 202 part is refuted. phase-06.md:488 "| POST | /packs | dashboard.read, data.steward | Build a pack: ... ; 202 (`pack.create`) |" gives the response for the route whatever the kind. The body part is largely covered: phase-31.md:581 "Scope `{period}`; languages `ar` and `en`; `RenderService` (phase 02) renders both", and the body shape of the route is shown at phase-06.md:954 as {"kind", "scope"}. What stands: no line writes the request body of the two new kinds as a shape, and the "lang" field the claim lists is not sent at all, since both languages are rendered.
- Fix: phase 31, section 3.7 "Kinds and build" (phase-31.md:581): add one sentence giving the request body as kind plus scope with period, and say that no language field is sent because both languages are rendered.

### V-31-03-c  STANDS   (first agent: LOW; refuter: LOW)
- Claim: the Cedar pointer "(section 3.16, E2)" in the kinds paragraph names the settings section; the Cedar file is section 3.17.
- Evidence: phase-31.md:581 "Cedar: create for M only (section 3.16, E2)". phase-31.md:755 "### 3.16 Settings used (phase 31 keys in `0100`; reused keys from earlier phases)". phase-31.md:761 "### 3.17 Cedar: the three files `E0_directory.cedar`, `E1_training_capacity.cedar`, `E2_exposure_packs.cedar`". The other pointers to Cedar (:700 "section 3.17, E0"; :902 "section 3.17") are right. No refutation found.
- Fix: phase 31, section 3.7 (phase-31.md:581): change the pointer from section 3.16 to section 3.17.

### V-31-04-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: step 4 requires "status built" for the pack, but admin.packs has no status column in any migration and no phase adds one.
- Evidence: phase-31.md:635 "The pack has kind `correspondent` or `validator`, status built, and a period that has ended". The admin.packs DDL at phase-02.md:423-438 has no status column. phase-06.md:470 gives the only status concept: "`status` of the build job: `queued`, `ready`, `failed`", which is a job state, not a pack column, and its value is "ready", not "built". Greps: `grep -rln "ADD COLUMN status" docs/plan` finds nothing; `grep -n "ALTER TABLE admin.packs" docs/plan/*.md` finds only the kind-CHECK rewrites at phase-06.md:262 and phase-24.md:240. The inference is available: a pack row is inserted only at publish ("insert the pack with `evidence_id` and `sha256`", phase-02.md:920), so a row's existence is the built state. That is why the gap is MEDIUM and not HIGH.
- Fix: phase 31, section 3.9 step 4 (phase-31.md:635): state the built check either as the existence of the admin.packs row or as the build job's status, using one of phase 06's values.

### V-31-04-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: attach_pack and correspondent_list are named without parameters, and no repository method inserts casework.rfi_packs or lists correspondent RFIs.
- Evidence: phase-31.md:444 "`RfiService::attach_pack` and `RfiService::correspondent_list` (extend phase 19 `RfiService`, section 4.5)" has no parameters. The inputs of attach are inferable from phase-31.md:631 "**Attach** (`POST /rfis/{id}/packs` ... `If-Match` = the RFI version; `Idempotency-Key`)" and the DTO at :425, so that part is LOW. The repository part is not inferable: section 2.2 (phase-31.md:427-432) lists Practice, Capacity, Exposure and Directory repositories only, and step 5 says "Insert `casework.rfi_packs`" (:636) with no method. The list at phase-31.md:629 needs the attached pack ids and kinds, and phase-19.md:996 gives RfiRepository a `desk(conn, filter, page)` whose filter is "the status filter", so it cannot serve that list. (The first agent's list of RfiRepository methods is incomplete: phase-19.md:996 also has set_status, set_submitted, set_released, set_extension and kpi_rows. The missing insert and list do not change.)
- Fix: phase 31, section 2.2 (phase-31.md:427-432): add a repository line naming the rfi_packs insert and the correspondent list with their parameters, and give the two service parameters at section 2.3 (:444).

### V-31-04-c  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: If-Match is required on POST /rfis/{id}/packs, but no 428 or 409 refusal is given for a missing or stale header.
- Evidence: a general rule covers it. docs/API.md:43 "Mutations on versioned resources require `If-Match: "<version>"`; a missing header -> 428", and :44 "`E_PRECONDITION_REQUIRED`; a mismatch -> 409 `E_VERSION_CONFLICT`". phase-03.md:531 "every versioned resource requires `If-Match`." The RFI is versioned (phase-19.md:127 "version INT NOT NULL DEFAULT 1", and phase-31.md:636 "Bump the RFI version"). The two codes are also in the phase 00 problem table (phase-00.md:495 "`VersionConflict { current }` | 409", :502 "`PreconditionRequired` | 428").
- Fix: none.

### V-31-04-d  PARTLY   (first agent: LOW; refuter: LOW)
- Claim: step 4 covers only pack_not_ready; no refusal is given for a pack whose kind is neither correspondent nor validator (for example board).
- Evidence: the refusal exists for that case through the same step. phase-31.md:635 "4. The pack has kind `correspondent` or `validator`, status built, and a period that has ended (422 sub-code `pack_not_ready`)." A board pack therefore gets 422 with the sub-code pack_not_ready. What stands: no separate sub-code is named for a wrong pack kind, and the PackError row at phase-31.md:718 lists `pack_not_ready` among the sub-codes with no kind sub-code. A developer has to choose a sub-code, which is a naming gap.
- Fix: phase 31, section 3.9 step 4 (phase-31.md:635): either name a sub-code for a pack whose kind is not correspondent or validator, or state that pack_not_ready covers the kind check too, and list it in the error row at :718.

### V-31-04-e  STANDS   (first agent: LOW; refuter: LOW)
- Claim: phase 31 describes recipient_ref as the canon.account id of the correspondent, but the phase 19 column comment reads "branch, staff or unit id".
- Evidence: phase-31.md:629 "`recipient_ref` (the `canon.account` id of the correspondent)". phase-19.md:104 "recipient_ref         UUID,                               -- branch, staff or unit id (canonical; no foreign key)". The correspondent recipient rule in phase 19 (phase-19.md:1071 "`correspondent`: `direction` is required (`in` or `out`); the release is external.") says nothing about recipient_ref, so the phase 31 meaning is the only one written for correspondents. No 7.x block in phase 19 or 31 reconciles the two. The claim holds at LOW: a documentation mismatch that does not change the logic.
- Fix: phase 31, section 3.9 list (phase-31.md:629): say that a correspondent RFI holds the canon.account id here, as a 7.x note that names the phase 19 column comment (phase-19.md:104) as the text it extends.

### V-31-05-a  STANDS   (first agent: MEDIUM; refuter: HIGH)
- Claim: the outcome "deferred" is decided after the commit, but the event row is written in the same transaction with its outcome and is never updated, so the two rules cannot both hold.
- Evidence: HIGH by the rubric (two texts contradict on when the outcome is known). phase-31.md:666 "6. Insert `admin.directory_events` in the same transaction (`outcome` and `reason_code`)." phase-31.md:225 "-- The event log: one row per processed event, inserted in the transaction that applied it and never updated." phase-31.md:699 "3. After commit: ... a Kratos failure enqueues `offboard_finalize` ... and the outcome is `deferred` (reason `disable_deferred`)." The lock order at phase-31.md:858 "(3) the directory row (`FOR UPDATE`); (4) the `admin.users` row (the phase 03 disable)" suggests the disable runs inside the event's flow, so the commit that the "After commit" step refers to may come after the row is written; the text does not settle it. Neither the 7.x blocks nor the lead corrections (phase-31.md:1081) say which rule wins. Greps: `grep -n "deferred" docs/plan/phase-31.md` gives :233, :235, :663, :699, :1081 and no timing rule.
- Fix: phase 31, section 3.11 step 6 or 3.12 step 3: say whether the Kratos step runs before the event row is written (then the row carries the outcome), or record the deferral as its own audit event, since the row is never updated.

### V-31-05-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: DirectoryCtx, the first parameter of disable_from_directory, is named but never defined; the caller is described only as "the directory".
- Evidence: phase-31.md:696 "`UserAdminService::disable_from_directory(ctx: DirectoryCtx, user_id, reason_code = "directory_deactivated")`". grep -rn "DirectoryCtx" docs/ research/ finds only that line. The nearest designed caller is the phase 03 context, so the type is inferable and the gap is MEDIUM: phase-03.md:1310 "The comment on `Ctx.via` reads ... `"webhook_hmac"`" and phase-31.md:767 "// Ctx.via = "webhook_hmac" (the value added to the phase 03 list by section 4.5)". It is not stated whether DirectoryCtx is that Ctx or a new struct.
- Fix: phase 31, section 2.1 or section 3.12 (phase-31.md:696): say whether DirectoryCtx is the phase 03 Ctx with via `webhook_hmac`, or list the fields of a new struct.

### V-31-05-c  STANDS   (first agent: LOW; refuter: LOW)
- Claim: the lead correction names the outcome disable_deferred, but the event outcome CHECK admits only applied, ignored, refused and deferred, and section 3.11 uses outcome deferred with reason disable_deferred.
- Evidence: phase-31.md:1081 "(Cedar `user.disable`, not `user.offboard`; outcome `disable_deferred`)". phase-31.md:233 "outcome TEXT NOT NULL CHECK (outcome IN ('applied', 'ignored', 'refused', 'deferred'))". phase-31.md:663 "`deferred` with reason `disable_deferred`". The reason CHECK at :235 does list disable_deferred, so the name is in the right column. No text says the correction wins, so the mismatch stands, at LOW because it is a name in a note and the CHECK settles the value.
- Fix: phase 31, section "Lead corrections" (phase-31.md:1081): name the outcome deferred and keep disable_deferred as the reason code.

### V-31-06-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the job reads scn.replays (step 1) and the daily would-alert counts of scn.replay_results (step 2), but no repository or port method is named.
- Evidence: the re-read is covered. phase-16.md:780 "async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<Replay>, RepoError>;" is in phase 16's ReplayRepository, which owns scn.replays, so step 1 has a method, though phase 31 does not name it. The counts part stands. phase-16.md:781-782 "async fn results(... ) -> Result<Vec<ReplayResult>, RepoError>;" returns entity-level rows, which phase 31 step 2 forbids (phase-31.md:561 "no `entity_id`, no `detection_key`"), and phase-16.md:783 "async fn summary_cells(...) -> Result<SummaryCells, RepoError>;" has no field list anywhere (grep "SummaryCells" in phase 16 finds only that line). No method returns business_date and would_alert only, and phase 31 section 2.2 (:427-432) names none.
- Fix: phase 31, section 2.2 (phase-31.md:427-432): name one read method that returns the per-date would-alert counts of one replay, and say in section 3.5 job step 1 that the re-read uses phase 16 `ReplayRepository::get`.

### V-31-06-b  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: step 2 refuses a replay that is not completed (409), but no code is given for a replay id that does not exist.
- Evidence: a general rule covers it. docs/plan/phase-00.md:494 "| `NotFound` | 404 | `E_NOT_FOUND` (one body for missing and masked, phase 03) |". Phase 31 uses the same mapping elsewhere in the same convention: phase-31.md:486 "2. Load the case. Unknown id: 404." Step 2 of section 3.5 (phase-31.md:556) says "The replay must exist", so the missing case is a refusal of the general NotFound row.
- Fix: none.

### V-31-06-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: job step 1 sets the run to running, a status mutation of casework.capacity_runs, and no audit event is named for it (only completed and failed are audited).
- Evidence: phase-31.md:560 "1. Mark the run `running`. Re-read the replay; a replay that is no longer `completed` fails the run." phase-31.md:142 "status TEXT NOT NULL CHECK (status IN ('queued', 'running', 'completed', 'failed'))". The 3.14 rows at phase-31.md:729-730 are "capacity.simulation_requested" and "capacity.simulation_completed" only. The housekeeping exemption does not reach it: it applies to writes that the item's text or phase 02 declares unaudited or covered by a run-level event, and this text says neither. The requested and completed pair brackets the run, which is the only argument for leaving it out. Grep "running" in phase 31 finds no running event (only :142, :560 and :995 mention the word).
- Fix: phase 31, section 3.14 (phase-31.md:730) or 3.5 job step 1 (:560): either name a kind for the running mark, or state that the requested and completed events cover it.

### V-31-06-d  STANDS   (first agent: LOW; refuter: LOW)
- Claim: the default class_mix is "the measured mix of section 3.4", but section 3.4 computes per-class rates only and has no measured mix output.
- Evidence: phase-31.md:555 "`class_mix` (shares of P0 to P3 that sum to 1; default: the measured mix of section 3.4)". grep -ci "mix" over phase-31.md:531-548 returns 0; the forecast output at phase-31.md:543 "10. Output per pool: staffed analysts, `L_p`, `R_p`, ..." has no mix. Open doubts 5 (phase-31.md:1089) says the class split "is an input of the request" and gives no default. The shares could be derived from the per-class arrivals of section 3.4 step 1, which is why the gap is LOW.
- Fix: phase 31, section 3.4 step 10 (phase-31.md:543) or section 3.5 step 1 (:555): state how the measured mix is computed (the per-class share of the window's arrivals), or drop the default and make class_mix required.

### V-31-06-e  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: SimulationInputs and SimulationResult are used as field types of CapacityRun but never defined; their fields exist only as prose.
- Evidence: the fields are stated in prose, which the planning rule accepts for a Rust type (a full struct is not required when the fields are named). phase-31.md:555 "`inputs`: `pools` (one to five pool ids), `weeks` (1 to 8), `class_mix`" gives the SimulationInputs fields, and phase-31.md:566 "7. Result: `queue_peak_by_class`, `queue_peak_total`, `breaches_by_class`, `analyst_hours_by_week_class`, `required_analysts_by_week`, `assumed`" gives the SimulationResult fields. The only other mention is the CapacityRun line at phase-31.md:421.
- Fix: none.

### V-31-07-a  STANDS   (first agent: LOW; refuter: LOW)
- Claim: step 2 says canon.txn is read with "partition pruning by business date", but canon.txn is partitioned by booking_date, so a business_date filter does not prune partitions.
- Evidence: phase-31.md:620 "(partition pruning by business date and by creation month)". phase-10.md:951 "CREATE TABLE IF NOT EXISTS canon.txn (" ends at phase-10.md:998 ") PARTITION BY RANGE (booking_date);". The business_date partition key at phase-10.md:922 belongs to the earlier table, canon.balance_snapshot. No text in phase 10 or 31 gives canon.txn a business_date partition, so the pruning claim is wrong for this table. The effect is a wrong performance rationale, which is LOW.
- Fix: phase 31, section 3.8 step 2 (phase-31.md:620): name booking_date as the pruning key for canon.txn, or drop the pruning phrase.

### V-31-07-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: ExposureService::compute(period_month) -> ExposureRunSummary names a return type that is never defined; its fields are not given beyond the audit detail keys.
- Evidence: phase-31.md:439 "`compute(period_month) -> ExposureRunSummary`" is the only occurrence (grep -rn "ExposureRunSummary" docs/ research/ finds only that line). The step at phase-31.md:621 "3. Upsert one `rpt.typology_exposure` row ... Audit `exposure.computed {period_month, rows, suppressed_rows}`" gives three keys that would fit the summary, but they are the audit payload, not the type's fields, so the type's content must be inferred. That is why the gap is MEDIUM and not HIGH.
- Fix: phase 31, section 2.1 or section 2.3 (phase-31.md:439): define ExposureRunSummary with the fields of the audit payload of section 3.8 step 3 (the period, the rows written and the suppressed rows).

### V-31-08-a  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: POST /directory/users/{id}/link takes If-Match (the directory row version), but no 428 or 409 refusal is given.
- Evidence: the general rule covers it. docs/API.md:43 "Mutations on versioned resources require `If-Match: "<version>"`; a missing header -> 428", and :44 "`E_PRECONDITION_REQUIRED`; a mismatch -> 409 `E_VERSION_CONFLICT`". phase-03.md:531 "every versioned resource requires `If-Match`." The directory row is versioned: phase-31.md:217 "version             INT NOT NULL DEFAULT 1 CHECK (version >= 1)", and the link route names it at phase-31.md:669 "`If-Match` = the directory row version".
- Fix: none.

### V-31-08-b  STANDS   (first agent: MEDIUM; refuter: HIGH)
- Claim: a directory e-mail that differs from a linked account is "recorded as directory.mismatch", but the update step records only directory.event_applied, and directory.mismatch is written only by the reconciliation for offboarded accounts.
- Evidence: HIGH by the rubric (a step is missing; two texts give two meanings to directory.mismatch). phase-31.md:692 "- Directory e-mail against a linked account: recorded as `directory.mismatch` and not applied." phase-31.md:662 "`update`: the display name is replaced; the e-mail of a linked user is not applied to the account", and phase-31.md:666 "Audit `directory.event_applied {event_ref, op, outcome, reason_code, directory_user_id}`" is the only audit of the update. phase-31.md:683 "3. Linked, account offboarded, `directory_active = TRUE`: audit `directory.mismatch {count}`". The reconciliation items at phase-31.md:679-684 compare no e-mails. DR-351 (phase-31.md:1071) "a directory e-mail change is recorded as a mismatch and never applied" has no step behind it. Greps: `grep -n -i "mismatch" docs/plan/phase-31.md` gives :683, :692, :738 and the decision and doubt lines, and no per-event write.
- Fix: phase 31, section 3.11 update step (phase-31.md:662) or reconciliation item 3 (:683): name the step that detects an e-mail difference and the audit kind it writes, and use the same kind for the offboarded case or give it another.

### V-31-08-c  STANDS   (first agent: LOW; refuter: LOW)
- Claim: the grant route is attributed to phase 03, but phase 04 owns POST /admin/users/{id}/grants.
- Evidence: phase-31.md:676 "(phase 03 `POST /admin/users/{id}/grants`, ..." attributes the route to phase 03. phase-04.md:806 "| POST | `/admin/users/{id}/grants` | admin.users | Propose a role grant; **CR** (`cr_access`); 202 |" is the route row in phase 04. phase-03.md:524 "`POST /admin/users/{id}/grants` (CR) ... are phase 04 and 09" points away from phase 03, and phase-03.md:503 lists only the DELETE of a grant. The claim holds; it is a wrong attribution with no logic effect.
- Fix: phase 31, section 3.11 "state of a directory-created person" (phase-31.md:676): name phase 04 as the owner of the grant route.

### V-31-10-a  STANDS   (first agent: LOW; refuter: LOW)
- Claim: the 7.2 block says section 3.12 implements the port as PgDirectoryLinkQuery, but section 3.12 has no text about it.
- Evidence: phase-31.md:1027 "Section 3.12 implements phase 03's `DirectoryLinkQuery::is_inactive(user_id) -> bool` as `PgDirectoryLinkQuery`". grep -n "PgDirectoryLinkQuery\|DirectoryLinkQuery" docs/plan/phase-31.md gives only :1027, so the text of section 3.12 (phase-31.md:694-702) does not mention the port. The rule itself is complete in the 7.2 block, so the gap is a cross-reference that does not land, which is LOW.
- Fix: phase 31, section 3.12 (phase-31.md:694-702): add one sentence saying that the directory link query port is implemented there, or change the 7.2 block to name where the port lives.

### V-31-10-b  STANDS   (first agent: LOW; refuter: LOW)
- Claim: the 7.2 block says the port is wired into AppState when pack.polis is on, but the section 4.3 AppState additions do not name it.
- Evidence: phase-31.md:1029 "wired into `AppState` when `pack.polis` is on". The akr-api list at phase-31.md:882 "`directory: Arc<DirectoryService>`, `directory_verifier: Arc<DirectoryWebhookVerifier>`" has no field for the link query, and grep -n "AppState" docs/plan/phase-31.md finds only lines 9, 448, 881 and 1029.
- Fix: phase 31, section 4.3 akr-api line (phase-31.md:882): add the link-query field next to `directory`, with the flag condition.

### V-31-11-a  PARTLY   (first agent: LOW; refuter: LOW)
- Claim: section 6 item 23 expects akr_retention to delete practice attempts through the retention run, but 7.3 and the 0100 grant give SELECT only.
- Evidence: the requirement is not refuted. Item 23 asks for no DELETE grant: its test is that "a direct `DELETE` by `akr_app` fails with `insufficient_privilege`", and "only through the retention run" is the same conditional rule as phase-31.md:505 "leave only through the retention run of phase 23 (`akr_retention`; the guard of `0100` admits no other deletion)". The deletion is deferred: phase-31.md:1035 "so `akr_retention` gets SELECT only until a period and its handler exist". What stands: the present-tense wording of item 23 at phase-31.md:1007 "`akr_retention` deletes practice attempts only through the retention run" reads as an existing deletion path that 7.3 and the grant at phase-31.md:283 "GRANT SELECT ON train.practice_attempts TO akr_retention;" deny.
- Fix: phase 31, section 6 item 23 (phase-31.md:1007): say that no deletion path for practice attempts exists in this release, and that the check is the SELECT-only grant.

### V-31-12-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the surge rule raises the sampler's "rate" to at least capacity.surge_qc_percent, but phase 19 has no single rate, and the two operands are on different scales.
- Evidence: phase-31.md:906 "9. **Phase 19, QA sampler.** While `capacity.surge_active` is `TRUE`, the sampler of phase 19 section 3.8 takes `max(its rate, capacity.surge_qc_percent)`". The setting is a percent: phase-31.md:329 "('capacity.surge_qc_percent', '15', 'decimal'". Phase 19 computes a count, not a rate: phase-19.md:1236 "**Sample size** (`QaSampler::sample_size(C, p, N) -> u32`, pure)", from phase-19.md:827 "('qa.confidence', '0.95'" and phase-19.md:829 "('qa.expected_defect_rate', '0.05'". The screening percent is a third setting, phase-19.md:835 "('qa.suppression_sample_percent', '5'". Neither file defines the base of "its rate", so a developer cannot compare a count with a percent without choosing one. The gap is MEDIUM because the base can be inferred (the implied sampled fraction of the population), but the text does not say so.
- Fix: phase 31, section 4.5 item 9 (phase-31.md:906): name the base that the surge percent is compared with, on the same scale, and state how the sample size is raised to meet it.

## Summary

- REFUTED: 4 (V-31-04-c, V-31-06-b, V-31-06-e, V-31-08-a)
- STANDS: 21
- PARTLY: 5 (V-31-01-a, V-31-03-b, V-31-04-d, V-31-06-a, V-31-11-a)

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-31-01-a | PARTLY | LOW | the label "passed on first attempt" is not reconciled with a flag that is TRUE when any one case passed (the flag is defined at :83) | phase 31 section 5.5 (:952) or 3.2 rule 4 (:503) |
| V-31-02-a | STANDS | MEDIUM | ClassAggregate is named but has no field list | phase 31 section 2.1 (after :420) |
| V-31-02-b | STANDS | MEDIUM | the hourly snapshot upsert names no audit event and no rule says it is an unaudited rebuild | phase 31 section 3.3 step 3 (:526) or 3.14 |
| V-31-02-c | STANDS | HIGH | the 2.3 ForecastQuery signature contradicts the fixed sensitivity grid of 3.4 step 7 | phase 31 section 2.3 (:437) or 3.4 step 7 (:540) |
| V-31-02-d | STANDS | LOW | NewAlarm is written with a subject tuple; the alarm columns are subject_kind and subject_id | phase 31 section 3.4 alarm step 3 (:548) |
| V-31-03-a | STANDS | LOW | fact F4 names ref.lists for timestamps that are columns of ref.list_versions | phase 31 section 3.7 F4 (:587) |
| V-31-03-b | PARTLY | LOW | no request-body shape is written for the two new kinds (the 202 is given by phase 06 at :488) | phase 31 section 3.7 (:581) |
| V-31-03-c | STANDS | LOW | the Cedar pointer says section 3.16; the Cedar file is section 3.17 | phase 31 section 3.7 (:581) |
| V-31-04-a | STANDS | MEDIUM | "status built" has no column; the built state is only inferable from the pack row, and phase 06 uses "ready" | phase 31 section 3.9 step 4 (:635) |
| V-31-04-b | STANDS | MEDIUM | no repository method for the casework.rfi_packs insert or the correspondent list; both service methods have no parameters | phase 31 section 2.2 (:427-432) and 2.3 (:444) |
| V-31-04-d | PARTLY | LOW | a pack of another kind gets 422 pack_not_ready, with no kind sub-code | phase 31 section 3.9 step 4 (:635) and error row (:718) |
| V-31-04-e | STANDS | LOW | recipient_ref meaning for correspondents differs from the phase 19 column comment | phase 31 section 3.9 list (:629) or phase 19 column comment (:104) |
| V-31-05-a | STANDS | HIGH | the deferred outcome is decided after commit, but the event row is written in the same transaction and never updated | phase 31 section 3.11 step 6 (:666) or 3.12 step 3 (:699) |
| V-31-05-b | STANDS | MEDIUM | DirectoryCtx is named but not defined; it is inferable as the phase 03 Ctx with via webhook_hmac | phase 31 section 2.1 or 3.12 (:696) |
| V-31-05-c | STANDS | LOW | the lead correction names the outcome disable_deferred; the CHECK has deferred with that reason | phase 31 lead corrections (:1081) |
| V-31-06-a | PARTLY | MEDIUM | no method returns the per-date would-alert counts; the scn.replays re-read is covered by phase 16 ReplayRepository::get | phase 31 section 2.2 (:427-432) and 3.5 job step 1 (:560) |
| V-31-06-c | STANDS | MEDIUM | the job step that marks the run running names no audit event | phase 31 section 3.14 (:730) or 3.5 job step 1 (:560) |
| V-31-06-d | STANDS | LOW | the default class_mix is "the measured mix of section 3.4", which 3.4 does not compute | phase 31 section 3.4 step 10 (:543) or 3.5 step 1 (:555) |
| V-31-07-a | STANDS | LOW | the canon.txn pruning key is named as business_date; the table is partitioned by booking_date | phase 31 section 3.8 step 2 (:620) |
| V-31-07-b | STANDS | MEDIUM | ExposureRunSummary is named but not defined; its only fields are the audit keys | phase 31 section 2.1 or 2.3 (:439) |
| V-31-08-b | STANDS | HIGH | the e-mail mismatch "recorded as directory.mismatch" has no step behind it, and the same kind means an offboarded-account count elsewhere | phase 31 section 3.11 update step (:662) or reconciliation item 3 (:683) |
| V-31-08-c | STANDS | LOW | the grant route is attributed to phase 03; phase 04 owns it | phase 31 section 3.11 (:676) |
| V-31-10-a | STANDS | LOW | the 7.2 block says section 3.12 implements the port; section 3.12 does not mention it | phase 31 section 3.12 (:694-702) |
| V-31-10-b | STANDS | LOW | the link-query port is wired into AppState according to the 7.2 block but is missing from the 4.3 list | phase 31 section 4.3 akr-api line (:882) |
| V-31-11-a | PARTLY | LOW | item 23 reads as an existing deletion path for practice attempts; the grant is SELECT only | phase 31 section 6 item 23 (:1007) |
| V-31-12-a | STANDS | MEDIUM | the base of "its rate" is undefined, and the surge percent and the sample size are on different scales | phase 31 section 4.5 item 9 (:906) |
