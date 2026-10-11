#!/usr/bin/env python3
"""Owner-decision round 2: the fixes for the 29 confirmed findings of the 16 round-2 agents and the four lead finds
(L2-1 to L2-4). Every verdict is in tools/agent/owner-verdict.md under "## Round 2". Two design rules are locked as
DR-423: phase 13 section 7.18 (a recompute resets the rows it no longer produces) and phase 04 section 7.21 (a
rejection of a change request is not held by the subject check)."""
import sys
sys.path.insert(0, "/home/haithem/Projects/AKR-TMS/tools/agent")
from ofix import Fixer

f = Fixer(round_no=2)
P = "docs/plan/phase-%s.md"

# ---------------------------------------------------------------- phases 07 and 14: the bank's cases
POINTER_OLD = "(phase 04 section 3.2: an immediate approval is rolled back with the error, and a scheduled activation ends `failed`)"
POINTER_NEW = "(phase 04: an immediate approval is rolled back with the error, section 3.4; a scheduled activation ends `failed`, section 3.6)"
f.fix("P-14b-1", P % "14", POINTER_OLD, POINTER_NEW)
f.fix("P-07b-2", P % "07", POINTER_OLD, POINTER_NEW)

f.fix("P-07a-1", P % "07",
      "   1. Load the case. A state other than `proposed` -> done; when the case has no `intake_result` (its request was withdrawn or rejected before the first run, so the uploaded file is still staged), first `delete_transient(\"goaml\", case_id)` and the audit event `goaml_corpus_case.transient_deleted {case_id, by: \"intake\"}`.",
      "   1. Load the case. When its state is not `proposed`: a case with no `intake_result` (its request was withdrawn or rejected before the first run, so the uploaded file is still staged) first has the staged file deleted with `delete_transient(\"goaml\", case_id)` and the audit event `goaml_corpus_case.transient_deleted {case_id, by: \"intake\"}` written; then the job is done. A `proposed` case goes on with step 2.")

f.fix("P-07b-1", P % "07",
      "the file is `secret-str` evidence that only the corpus runner opens.",
      "the file is `secret-str` evidence that only the corpus runner and the intake job (item 6 step 4) open.")

f.fix("P-07b-3/4", P % "07",
      "fails with\n   `profile versions are never deleted` (as `akr_app` it fails with permission denied, the last check of this item);",
      "fails with\n   `goAML profile versions are never deleted` (as `akr_app` it fails with permission denied; see the last check of this item);")

f.fix("P-07b-5", P % "07",
      "retention row. a `DELETE` fails, as `akr_app`",
      "retention row. A `DELETE` fails, as `akr_app`")

f.fix("P-07b-6", P % "07",
      "each deletion of an uploaded file is audited, and the mask",
      "each deletion of an uploaded file by an intake run or by the sweep is audited, and the mask")

f.fix("P-07b-7", P % "07",
      "This hook enqueues nothing: an issue is a refusal, phase 04 rolls the action back, and a job enqueued in a rolled-back transaction leaves no row.",
      "This hook enqueues nothing: an issue refuses an approval (a rejection is not checked, phase 04 section 7.21), phase 04 rolls the action back, and a job enqueued in a rolled-back transaction leaves no row.")
f.fix("P-07b-7", P % "07",
      "cannot be approved: the approver rejects the request or its maker withdraws it, and the case becomes `rejected`.",
      "cannot be approved: the approver rejects the request (the check does not hold a rejection back, phase 04 section 7.21) or its maker withdraws it, and the case becomes `rejected`.")

# ---------------------------------------------------------------- phase 04: a rejection is not held by the subject check
f.fix("P-07b-7", P % "04",
      "returned as 409 `E_STALE_BASE`, 422 `E_EVIDENCE_STALE` or 422 `E_VALIDATION`, and the task stays open.",
      "returned as 409 `E_STALE_BASE`, 422 `E_EVIDENCE_STALE` or 422 `E_VALIDATION`, and the task stays open. A `Reject` is not\nchecked (section 7.21).")

B0421 = """
### 7.21 A rejection of a change request is not held by the subject check (owner-decision check round 2, HIGH; DR-423)
Section 3.2 (`decide`) and section 3.4 step 6 run the subject check before every action, a rejection included, and the hook of section 7.1 is not told which action it guards. For a change request this leaves a request that can be neither approved nor rejected whenever its check fails: a moved base, evidence past `change.evidence_max_age_days`, `shadow_too_short`, `volume_effect_changed` (section 7.9), or a refusal of a later phase's target such as `intake_not_met` (phase 07 section 7.6). Only its maker could then end it, by a withdraw. Section 3.6 says of an offboarded maker's submitted requests that "their checkers still decide them", and phase 07 section 7.6 with phase 08 section 7.11 closes an unmet corpus case by the approver's reject. So:

1. Trait `WorkflowSubject` (section 2; section 7.1) gains one method with a default body:

       fn holds_reject(&self) -> bool { true }

2. Section 3.4 step 6 (as section 7.1 reads) now reads: "Subject check before decision: the subject's `check_before_decision` runs in the same transaction, before the action applies; it is skipped when the action is `TaskAction::Reject` and the subject's `holds_reject()` is `false`. A refusal rolls the action back and the task stays open."
3. The change subject returns `false`. A rejection of a change request therefore runs neither the evidence-age check of section 3.2 nor `ChangeTarget::check_before_decision`. Steps 1 to 5 of section 3.4 (task and instance state, step type, Cedar, four-eyes, step-up) apply as before, and the outcome handling of section 3.2 is unchanged: `set_decided(rejected)`, `target.on_closed(Rejected)`, audit `change_request.rejected`.
4. Every other subject keeps the default `true`: the checks of the subjects of later phases (the case, RFI, freeze, operations-task, report and legal-hold subjects among them) run for a rejection as they do for an approval.
5. Reading rule. Where this file or a later phase says that the check of a change target runs "at every deciding action" or refuses "the decision", it means every action other than a rejection. No target changes: `ChangeTarget::check_before_decision` keeps its signature and is not called for a rejection.
6. Tests (added to section 6): a submitted `_dev_object` request whose dev row's version was bumped: approve -> 409 `E_STALE_BASE` and the task stays open; reject by the same approver, with a comment -> 200, the request is `rejected`, `on_closed(Rejected)` ran and `change_request.rejected` is in the audit trail. A request whose evidence is older than `change.evidence_max_age_days`: approve -> 422 `E_EVIDENCE_STALE`; reject -> 200. A test subject that keeps the default and whose check refuses: a reject returns that refusal and the task stays open.
"""
f.fix("P-07b-7", P % "04",
      "\n## What This Phase Does NOT Touch\n",
      B0421 + "\n## What This Phase Does NOT Touch\n")

# ---------------------------------------------------------------- phase 08: the panel Bank cases on E1p
f.fix("P-08-1", P % "08",
      "For a change-request task the panel links to G10 detail.",
      "For a change-request task the panel links to G10 detail for a role that G10 lists (the screen table above) and shows the reference of the request as text for any other role (an A2 maker of a corpus case, section 7.11 item 3).")
f.fix("P-08-2", P % "08",
      "(the client checks the type only; no response gives the size limit, so a file",
      "(the client checks the type only; no response open to A2 or M gives the size limit, so a file")

# ---------------------------------------------------------------- phase 10: the writer
f.fix("P-10-1", P % "10",
      "A name already stored takes a new label when its row is mapped again: at the next load of its source row, or in the apply of a mapping replay (phase 11 section 3.8).",
      "A name already stored takes a new label when its row is mapped again. A source row that changed lands again, and its name row is replaced with the mapped label. A source row that did not change is not landed again (phase 11 section 3.2 step 2), so a mapping edit alone reaches it only in the apply of a mapping replay (phase 11 section 3.8).")
f.fix("P-10-2", P % "10",
      "This phase's parts: DR-422 locks the corrections of the check round of 2026-10-11: the payment columns and the grant date are compared as mapped values, and the index on `(facility_id, seq)` is not unique.",
      "DR-422 locks the corrections of the check round of 2026-10-11: the payment columns and the grant date are compared as mapped values, and the index on `(facility_id, seq)` is not unique. This phase's parts:")

# ---------------------------------------------------------------- phases 11 and 12: the parent_missing reason
f.fix("P-11-3", P % "11",
      "-- Rows that did not map (`row_failed`) and rows whose canonical value differs from a settled one (`row_conflict`,\n-- returned by the CanonicalWriter, phase 10 section 3.7).",
      "-- Rows that did not map (`row_failed`) and rows the CanonicalWriter returns as a conflict (`row_conflict`, phase 10\n-- section 3.7): a canonical value that differs from a settled one, or a parent key that has not landed (`parent_missing`).")
f.fix("P-11-1", P % "12",
      "`ConflictKind` (`FactChangedInSource`,\n  `LabelChange`, `ValueChange`; the mapping to `DqKind` is section 3.3)",
      "`ConflictKind` (`FactChangedInSource`,\n  `LabelChange`, `ValueChange`, `ParentMissing`; the mapping to `DqKind` is section 3.3)")
f.fix("P-11-1", P % "12",
      "the three `ConflictKind` values map to their kinds and severities",
      "the four `ConflictKind` values map to their kinds and severities")
f.fix("P-11-2", P % "12",
      "is a mapped row that disagrees with a settled one, so it is never retried; it is closed when its issue is resolved\n(section 3.4).",
      "is a mapped row that disagrees with a settled one, so it is never retried; it is closed when its issue is resolved\n(section 3.4). One kind is the exception: a `parent_missing` row disagrees with nothing (its parent has not landed yet),\nand the trailing re-read window of phase 10 section 7.20 meets it again.")

# ---------------------------------------------------------------- phase 13: source tags and the recompute
f.fix("P-13b-1", P % "13",
      "for the source-tag proposal of section 7.17 item 6 only.",
      "for the source-tag proposal, its decision check and its activation (section 7.17 items 6 and 7) only.")
f.fix("P-13b-2", P % "13",
      "and the hourly run does not take those months again. So, when",
      "and the hourly run takes again only the months within `features.catchup_months` (section 3.2 step 1). So, when")
f.fix("P-13b-4", P % "13",
      "(a row ended on day D still holds on D, so the month of D is the first that can change)",
      "(a row ended on day D still holds on D, so the first date that can change is the day after D; taking the month of D is at most one month early, which costs one recompute and changes no value)")
f.fix("P-13b-3", P % "13",
      "under the key `feature_compute` a queued hourly run would swallow the enqueue (phase 00 `ON CONFLICT DO NOTHING`) and the reopen would be lost;",
      "under the plain key `feature_compute` another queued job with that key (the catch-up enqueue of a feature activation, or the enqueue of a replay) would swallow the enqueue (phase 00 `ON CONFLICT DO NOTHING`) and the reopen would be lost; the scheduler's hourly job is not one of them, because its key is `feature_compute:<slot>` (phase 00);")
f.fix("L2-2", P % "13",
      "computes it again whatever its `basis` says (the register is not part of the basis), and step 5 updates only the rows whose value changed.",
      "computes it again whatever its `basis` says (the register is not part of the basis), step 5 updates only the rows whose value changed, and section 7.18 resets the rows the recompute no longer produces (a customer whose only matching credits came from the changed tag).")
f.fix("P-13b-2/3", P % "13",
      "the activation of a change set that voids a tag whose `valid_from` lies two months back enqueues one `feature_compute` job with that month's month-end as `reopen_from`, also while an hourly `feature_compute` job is queued, and after it ran the feature rows of the two months no longer count the credits.",
      "the activation of a change set that voids a tag whose `valid_from` lies five months back (outside `features.catchup_months`) enqueues one `feature_compute` job with that month's month-end as `reopen_from`, also while a `feature_compute` job with the plain key is queued, and after it ran the feature rows of those months no longer count the credits: the row of a customer with other payroll credits holds the smaller sum, and the row of a customer whose only payroll credits came from the voided tag holds no value (`no_history`, `basis.reason = no_activity`; section 7.18).")
f.fix("P-13b-3", P % "13",
      "is dropped while another `feature_compute` job is queued (phase 00: `ON CONFLICT DO NOTHING`), and the reopen is lost.",
      "is dropped while another job with that plain key is queued (phase 00: `ON CONFLICT DO NOTHING`; the scheduler's hourly job has the key `feature_compute:<slot>` and is not one), and the reopen is lost.")
f.fix("L2-1", P % "13",
      "Counterparty to party links. Owner akr_app; retention R2",
      "Counterparty to party links. Owner akr_migrate; written by akr_app; retention R2")
f.fix("P-DR-2", P % "13",
      "`UPDATE feat.feature_definitions SET expression = '{}'` fails with `feature definition content is immutable`;",
      "`UPDATE feat.feature_definitions SET expression = '{}'` fails with permission denied (`expression` is not in the\n   column grant) and, as the owner `akr_migrate`, with `feature definition content is immutable`;")
f.fix("P-19b-2", P % "13",
      "`DELETE FROM feat.source_tags` fails, as `akr_app` with permission denied (no DELETE grant) and as the owner `akr_migrate` with `source tags are never deleted`.",
      "A `DELETE` of a row of `feat.source_tags` fails, as `akr_app` with permission denied (no DELETE grant; with or without a row) and as the owner `akr_migrate` with `source tags are never deleted` (the guard is a row trigger, so the owner's statement needs a row).")
f.fix("L2-2", P % "13",
      "    /// Set-based insert-or-update on the primary key; touches only rows whose basis differs. Returns (inserted, updated).",
      "    /// Set-based insert-or-update on the primary key; touches only rows whose basis differs. Returns (inserted, updated).\n    /// Section 7.18: the same statement resets the rows of the pair that the SELECT no longer produces; they count as updated.")
f.fix("L2-2", P % "13",
      "   zero rows. A re-run with changed data updates only the rows whose value changed.",
      "   zero rows. A re-run with changed data updates only the rows whose value changed. A row that the re-run no longer\n   produces is not left as it was: it is reset (section 7.18).")

B1318 = """
### 7.18 A recompute resets the rows it no longer produces (owner-decision check round 2, HIGH; DR-423)
The month-end statement of section 3.1 inserts or updates the rows its SELECT produces and leaves every other row of the pair `(feature, as_of)` as it was, and `akr_app` has no DELETE on `feat.feature_values` (section 1, by design). When a re-run no longer produces a group that an earlier run stored, the old value stayed. Two cases reach it: a replay that cancels the only posting an entity had in the window (section 3.2, `reopen_from`), and a source tag that is voided or ended when its credits were a customer's only match (section 7.17 item 7). The row then kept a value the data no longer supports, in the reopened months and in every later run.

1. **Rule.** The write of a pair `(feature, as_of)` also resets, in the same statement, every stored row of that pair that holds a value and whose `(entity_id, currency)` the SELECT did not produce. The reset sets `value` to NULL, `status` to `no_history` (`beyond_window` for the kind `days_since`, as section 7.11 says of an account with no posting in its window), `basis` to the run's basis with the member `"reason": "no_activity"` added, and `computed_at` to `now()`. No row is deleted and no grant changes: `akr_app` holds UPDATE on these four columns (section 1). The reset runs only for a pair that passes the gate of section 3.2 step 2; a `not_ready` or `dark` pair writes nothing, as before.
2. **Readers.** A row without a value is read as an absent row. Section 3.4 step 2 and the profile build of section 3.6 already take `status = value` only; "absent" in section 3.2 step 4 and in section 3.5 step 1 means no row with a value. So a reset row gives zero for a `count` or `sum` feature under a `computed` run and "not applicable" for the other kinds, and it is not counted in a segment statistic.
3. **The statement.** `FeatureCompiler::compile` wraps the SELECT of the feature as the CTE `cur` (one row per `(entity_id, currency)`, with the value and the status the INSERT of section 3.1 selected) and emits one statement of four parts for every kind. For `credit_turnover_1m` it replaces the SQL of section 3.1:

```sql
WITH cur AS MATERIALIZED (
    SELECT a.customer_id AS entity_id, t.currency AS currency, SUM(t.amount) AS value, 'value'::text AS status
    FROM canon.txn t
    JOIN canon.account a ON a.id = t.account_id AND a.recorded_to IS NULL
    WHERE t.booking_date BETWEEN $4 AND $2 AND t.dr_cr = 'C' AND t.status = 'posted' AND t.system_generated IS NOT TRUE
    GROUP BY a.customer_id, t.currency
), had AS MATERIALIZED (                      -- the keys stored before this statement (one snapshot for all parts)
    SELECT entity_id, currency FROM feat.feature_values WHERE feature_id = $1 AND as_of = $2
), up AS (
    INSERT INTO feat.feature_values (entity, entity_id, feature_id, currency, as_of, value, status, basis, computed_at)
    SELECT 'customer', entity_id, $1, currency, $2, value, status, $3, now() FROM cur
    ON CONFLICT (entity, entity_id, feature_id, currency, as_of) DO UPDATE
        SET value = EXCLUDED.value, status = EXCLUDED.status, basis = EXCLUDED.basis, computed_at = now()
        WHERE feat.feature_values.value IS DISTINCT FROM EXCLUDED.value
           OR feat.feature_values.basis IS DISTINCT FROM EXCLUDED.basis
    RETURNING entity_id, currency
), gone AS (
    UPDATE feat.feature_values v
       SET value = NULL, status = 'no_history', basis = $3 || '{"reason": "no_activity"}'::jsonb, computed_at = now()
     WHERE v.feature_id = $1 AND v.as_of = $2 AND v.status = 'value'
       AND NOT EXISTS (SELECT 1 FROM cur c WHERE c.entity_id = v.entity_id AND c.currency = v.currency)
    RETURNING 1
)
SELECT (SELECT count(*) FROM up u WHERE NOT EXISTS
            (SELECT 1 FROM had h WHERE h.entity_id = u.entity_id AND h.currency = u.currency)) AS inserted,
       (SELECT count(*) FROM up u WHERE EXISTS
            (SELECT 1 FROM had h WHERE h.entity_id = u.entity_id AND h.currency = u.currency))
     + (SELECT count(*) FROM gone)                                                             AS updated;
```

   `up` and `gone` never touch the same row: `up` writes the keys of `cur`, `gone` the stored keys that are not in `cur`. The lookup of `had` and `gone` uses `idx_feature_values_feature` inside the one monthly partition of `$2`. `write_month_end` reads the one result row and returns `(inserted, updated)`; reset rows count as updated, so `feature_runs.rows_written` and the audit event `feature.computed` include them. Only a row with `status = 'value'` is reset, so a second run for the same months still writes zero rows (section 3.2 step 5). A reset row whose entity has activity again is written by `up` with the run's basis, without the reason.
4. **Tests** (added to section 6): a month with the customers A and B is computed; the fixture then cancels B's only posting of the month and the job runs with `reopen_from` = that month-end. B's row then has `value` NULL, `status = 'no_history'` and `basis->>'reason' = 'no_activity'`; A's row is unchanged; the run reports one updated row and the next run reports zero. `segment_stats_refresh` for the period does not count B in `n`. When B has a posting again and the month is reopened, B's row holds the new value and a basis without `reason`. As `akr_app`, `DELETE FROM feat.feature_values` still fails with permission denied (section 6 item 1).
"""
f.fix("L2-2", P % "13", "\n## Decisions\n", B1318 + "\n## Decisions\n")

# ---------------------------------------------------------------- phase 18: the case_acl clause of the database tests
f.fix("L2-3", P % "18",
      "a `casework.case_acl` row cannot change its `access` (the guard raises).",
      "a `casework.case_acl` row cannot change its `access` (permission denied for `akr_app`, whose column grant is `revoked_by` and `revoked_at`; as the owner `akr_migrate` the guard raises `an access grant is revoked, never changed`).")

# ---------------------------------------------------------------- phase 19: the pre-fill map
f.fix("P-19b-1", P % "19",
      "and for CFG and CHK the `rfi_prefill_map.read` and `rfi_prefill_map.propose` permits of section 7.21)",
      "and for CFG the `rfi_prefill_map.read` and `rfi_prefill_map.propose` permits and for CHK the `rfi_prefill_map.read` permit of section 7.21)")
f.fix("P-19b-2", P % "19",
      "`DELETE FROM casework.rfi_prefill_map` fails, as `akr_app` with permission denied (no DELETE grant) and as the owner `akr_migrate` with `pre-fill map rows are never deleted`.",
      "A `DELETE` of a row of `casework.rfi_prefill_map` fails, as `akr_app` with permission denied (no DELETE grant; with or without a row) and as the owner `akr_migrate` with `pre-fill map rows are never deleted` (the guard is a row trigger, so the owner's statement needs a row).")
f.fix("P-19b-3", "docs/DESIGN-SYSTEM.md",
      "through a change request. CHK and M\nread the tab;",
      "through a change request. CFG, CHK and M\nread the tab;")

# ---------------------------------------------------------------- phase 26: SCN-ISL-07 on schedule tables
f.fix("P-26-1", P % "26",
      "The three inputs marked `?` are optional inputs (DSL.md, CanonicalRef)",
      "The four inputs marked `?` (`facility.granted_on` and the three `facility_instalment` fields) are optional inputs (DSL.md, CanonicalRef)")
f.fix("P-26-1", P % "26",
      "is not published, the three are listed in the `reasons` of the readiness result",
      "is not published, the inputs that are missing are listed in the `reasons` of the readiness result")
f.fix("P-26-2", P % "26",
      "has a `admin.column_classification` row",
      "has an `admin.column_classification` row")
f.fix("P-26-3", P % "26",
      "the payment still lands, and a changed due date or amount is recorded as a conflict (phase 10 section 7.27 item 3).",
      "the payment still lands; a changed due date or amount that arrives with no payment change is recorded as a conflict, and one that arrives in the same version as the payment is neither written nor reported (phase 10 section 7.27 item 3).")

# ---------------------------------------------------------------- system documents
f.fix("L2-4", "docs/DATA-MODEL.md",
      "where it does not, the scenario\nis declared not covered.",
      "where it does not, the facility\nlegs of the scenario are not evaluable and its readiness grade does not change (phase 26 section 7.13 item 5).")
f.fix("L2-4", "docs/DATA-MODEL.md",
      "and `status` and `status_history` of `payment_message`; no role has DELETE on",
      "and `status` and `status_history` of `payment_message` (phase 26 adds the updates named in its own paragraph below); no role has DELETE on")
f.fix("L2-2", "docs/DATA-MODEL.md",
      "; sparse by activity | as_of (month) | R4 |",
      "; sparse by activity (a row that a recompute no longer produces is kept without a value: `no_history`, `basis.reason = no_activity`; phase 13 section 7.18) | as_of (month) | R4 |")

# ---------------------------------------------------------------- research.md: DR-422 brought in step, DR-423 added
R = "docs/plan/research.md"
f.fix("P-DR-1", R,
      "each deletion of an uploaded file writes `goaml_corpus_case.transient_deleted`;",
      "each deletion of an uploaded file by an intake run or by the sweep writes `goaml_corpus_case.transient_deleted` (a failed proposal has no case row and writes no event);")
f.fix("P-26-4", R,
      "`paid_amount` may be set while `paid_on` is null. (2)",
      "`paid_amount` may be set while `paid_on` is null; the four optional inputs of SCN-ISL-07 (`facility.granted_on` and the three `facility_instalment` fields) do not change the readiness grade, and while they are not published the facility legs are not evaluable. (2)")

ROW = ("| DR-423 | 2026-10-11 | Design corrections of round 2 of the check on the owner decisions (every verdict is in "
       "tools/agent/owner-verdict.md, under Round 2). "
       "(1) Feature values, phase 13 section 7.18: the write of a month-end resets, in the same statement, every stored row "
       "of the pair (feature, month-end) that holds a value and that its SELECT no longer produces: `value` NULL, status "
       "`no_history` (`beyond_window` for the kind `days_since`), `basis.reason = no_activity`. No row is deleted and no "
       "grant changes, and a reader takes a row without a value as an absent row. "
       "(2) Change requests, phase 04 section 7.21: the subject check is not run for a rejection of a change request "
       "(`WorkflowSubject::holds_reject`, default `true`, `false` for the change subject), so a request that can no longer be "
       "approved is closed by its approver's reject as well as by its maker's withdraw. Every other subject keeps its check "
       "for a rejection "
       "| Found by the lead and by a round-2 agent. Without (1) a voided source tag, or a replay that cancels a customer's "
       "only posting of a month, left the old feature value in place, and a stated test of phase 13 section 7.17 failed. "
       "Without (2) the approver's rejection of a corpus case whose intake was not met (phase 07 section 7.6, DR-415) was "
       "refused by the same check that stops its approval, and an A2 maker has no withdraw screen (phase 08 section 7.11) |\n")
ANCHOR = "and the replay enqueue under the plain coalesce key (phase 13 Open doubts) |\n"
f.fix("DR-423", R, ANCHOR, ANCHOR + ROW)

f.done()
