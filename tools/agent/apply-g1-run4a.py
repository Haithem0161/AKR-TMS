#!/usr/bin/env python3
"""Apply gap pass 1, run 4a (targets 17 to 24) after the lead's checks, with the carried wording items.

Source: tools/agent/gap1-run4a.md (Haiku draft); decisions: tools/agent/gap1-run4a-verdict.md.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'

HDR = ('Gap pass 1 (2026-10-10), run 4a, from tools/agent/gap1-run4a.md; lead-verified '
       '(tools/agent/gap1-run4a-verdict.md).')
CHDR = ('Gap pass 1 (2026-10-10), run 4a: items carried from runs 3a and 3b and corrections found while checking '
        'phases 17 to 24 (tools/agent/gap1-run4a-verdict.md); lead-verified.')

P10C = [
"""### 7.N Drift notices stay on the screens (phase 22 section 7+)
Section 3.5 says notifications arrive with phase 22. Phase 22 defines no drift topic in this release: open drift
stays on T3 and the cockpit, which are the notice. No outbox row is written for drift.""",
]

P17 = [
"""### 7.N Hibernation gate, breach clock and open-hit clause (phase 19)
Phase 19 extends this phase and owns the extension (its section 4.5 item 4); no column of this phase changes:
- Section 3.1: a new step 4b, between steps 4 and 5, calls `HibernationGate::check(conn, party_id, scenario_id,
  measures, now)`. On `Suppress(h)`, steps 6, 8 and 11 run and the hit row (`casework.hibernation_hits`) is inserted
  after step 6; steps 5, 7, 9, 10 and 12 are skipped (no case is opened or joined, no clock runs, no case event and no
  `alert.created`). Step 9 calls `BreachService::arm(tx, "alert", id, created_at, touch_hours)`. A new method
  `attach_released(tx, alert_id, alert_created_at)` runs steps 5, 7, 9, 10 and 12 for the existing alert (used by the
  `hibernation_release` job; the clocks start at the release, not at the detection).
- Sections 3.9 (queue query), 3.10 (visibility predicate) and 3.11 (counts) gain the clause `AND NOT EXISTS (SELECT 1
  FROM casework.hibernation_hits h WHERE h.alert_id = a.id AND h.alert_created_at = a.created_at AND h.released_at IS
  NULL)`.
- Section 3.12: the touch handler `timer_alert_touch` calls `BreachService::breach(tx, "alert", id, 100)` with its
  breach write; `BreachService::breach` skips an alert with an open hit.""",

"""### 7.N Case surfaces extended by phase 18
Phase 18 owns these changes (its section 4.5 items 4, 5 and 7):
- `casework.cases` gains `escalation_level` and the `outcome` CHECK (phase 18 section 1.1).
- `AssignmentService` (section 3.8) writes the `casework.case_acl` grants of phase 18 section 3.2 in the assignment
  transaction: the assignee gets `work` (A1) or `decide` (A2), reason `assignment`; the second worker gets `decide`,
  reason `second_review`; a reassignment revokes the previous assignee's grant (`revoked_by` is the actor).
- `CaseRepository` gains the methods of phase 18 section 2.
- `GET /cases/{id}` gains `escalation_level`, `outcome`, `sealed`, `open_proposal_id`, the `affordances` object and,
  for M only, the reporter row (phase 18 section 3.10). The route and its scope do not change.""",

"""### 7.N Declared conflict raises an alarm (phase 19)
From phase 19 on, `AssignmentService` (section 3.8) raises `conflict_declared` through `AlarmService::raise` on a
declared conflict (phase 19 section 3.10). This closes Open doubts 16 for the alarm; a refused claim stays audit-only
(`alert.conflict_refused`).""",

"""### 7.N Break-glass clause in rule 5 (phase 25)
From phase 25 on, `services::visibility::build` (section 3.10) gains one admitting clause in the team of rule 5, only
under purpose `break_glass`: `OR ($purpose = 'break_glass' AND EXISTS (SELECT 1 FROM admin.break_glass_grants g WHERE
g.case_id = c.id AND g.requester_id = $1 AND g.status = 'active' AND g.expires_at > now()))`. The Cedar entity adapter
adds the requester to `Case.assignees` under the same condition (phase 25 section 3.2, "Use" step 2). Phase 25 extends
check 6 of section 6 with the purpose-keyed case and owns the table.""",

"""### 7.N Model rank in the queue order (phase 27)
From phase 27 on, the items query of section 3.9 orders by `priority_class, model_rank DESC NULLS LAST, score DESC
NULLS LAST, created_at, id`, where `model_rank` is the latest `live` score of `alert.model_scores` for the alert (a
`LEFT JOIN LATERAL` on `alert_id` and `alert_created_at`), under the kill condition of phase 27 section 3.4. The queue
row gains `model_band` for A1, A2 and M. `GET /alerts/{id}/model-score` (phase 27 route 16) joins the `alerts` tag and
the probe set of kind `alert` in check 9. Phase 27 owns these changes; `alert.alerts` is not altered.""",
]

P18 = [
"""### 7.N Watch start, closure gate, network hook and bulk reopen (phase 19)
Phase 19 extends this phase and owns the extension (its section 4.5 item 5):
- Section 3.8 step f (`RecordDecisionEffect`): after step 8 (the tier), for a `utr` decision,
  `WatchService::start_for_decision(tx, decision, case)` in the same transaction.
- Section 3.12 (`CaseService::close`): the closure gate of phase 19 section 3.3 (sub-code
  `uncorroborated_explanation`), and the network hook of phase 19 section 3.7 step 7 (a child and its parent are locked
  in ascending `id` order).
- Section 3.13: the new entry point `CaseService::reopen_under_batch(tx, case_id, reason_code, actor, finding_id)` for
  the bulk reopen of phase 19 section 3.9: the reopen rules of this section with the batch assertion of `POST
  /qa/bulk-reopen` in place of a second one (the route consumes the step-up once).""",

"""### 7.N Work-item breach clock (phase 19)
From phase 19 on, `WorkItemService::open` (section 3.6) calls `BreachService::arm(tx, "work_item", id, now,
casework.work_item_due_hours)`, and the `timer_work_item_due` handler calls `BreachService::breach(tx, "work_item", id,
100)` with its overdue write. The stages and the alarm `work_item_breach` are phase 19 section 3.10. This closes Open
doubts 11.""",

"""### 7.N Report checks on the case report approval (phase 21)
Phase 21 extends this phase and owns the extension (its section 4.5 items 1, 2, 4 and 5; DR-170):
- The sign step of `case_report_approval` uses step-up class `signing`, not `decision`. This closes Open doubts 3.
- `CaseWorkflowSubject::check_before_decision` (section 3.8 d), for an instance of `case_report_approval`, runs the
  report checks of phase 21 section 3.6 step 3 (the report exists, `status = validated`, `frozen_version =
  current_version`, no block in the latest run, and the act carries `via = "report"`).
- The decision route (`POST /cases/{id}/decisions`, action `sign` on `case_report_approval`) and the phase 04 generic
  task route for a `sign` task of `case_report_approval` or `report_correction` refuse with 409
  `E_NOT_ALLOWED_IN_STATE`, sub-code `report_sign_required`, unless `via = "report"`.
- `ReleaseForFilingEffect` (section 3.8 step g) takes the body of phase 21 section 3.7; the close gate of section 3.12
  step 5 is replaced by phase 21's acknowledged-filing check (`filing_not_acknowledged`), as Open doubts 10 says.""",

"""### 7.N Break-glass clause in the case rows (phase 25)
From phase 25 on, the rule-5 clause of section 3.10 (case rows) gains the same purpose-keyed break-glass clause as
phase 17 section 3.10 (phase 17 section 7+), and the Cedar `Case.assignees` adapter takes the requester under purpose
`break_glass` only (phase 25 section 3.2). The masked 404 does not change.""",

"""### 7.N AI involvement and the assistant summary (phase 28)
From phase 28 on, `RecordDecisionEffect` calls `AssistantInvolvement::for_case` once in the decision transaction
(phase 28 section 3.4) and writes its shape into `casework.decisions.ai_involvement`; the default `{"used": false}` is
unchanged. The `affordances` of `GET /cases/{id}` gain `assistant_summary` (computed on the server).""",

"""### 7.N Lock extension on seal (phase 23, DR-193)
From phase 23 on, the seal transaction of section 3.12 enqueues `retention_lock_extend` with `{case_id}`. No route,
state or event of this phase changes. This closes Open doubts 14.""",

"""### 7.N The case review clock (DR-384)
The clock `case.review` (phase 09 registry, days, bounds 30 to 365) needs no column: a case in `open` or `in_review` is
due at `casework.cases.opened_at` plus the clock's days in force; closed and merged cases have no due date. Phase 24's
clocks board computes it (phase 24 section 7+).""",

"""### 7.N Lock order, sub-codes and party routes extended by phase 19
- Section 3.16: phase 19 section 3.15 adds rank 3 (`casework.rfis`, `rfi_questions`, `rfi_answers`,
  `authority_requests`, `internal_reports`, `watches`, `hibernations`, `hibernation_hits`, `alarms`, `qa_samples`) and
  rank 4 (`casework.case_acl`, `work_items`, `case_events`, `network_edges`, `network_snapshots`).
- Section 3.17: the 24 sub-codes of phase 19 section 3.11 join the closed list: `recipient_not_liaison`,
  `tier_forbids_rm`, `contact_not_allowed`, `template_unknown`, `purpose_not_in_pack`, `already_registered`,
  `watch_needs_utr`, `already_hibernating`, `hibernation_not_allowed`, `no_network`, `not_due`,
  `uncorroborated_explanation`, `no_eligible_reviewer`, `reviewer_is_closer`, `not_sample_reviewer`, `sample_closed`,
  `finding_criterion_passed`, `score_mismatch`, `not_bulk_case`, `no_linked_case`, `role_not_permitted`,
  `not_on_case`, `unknown_party`, `forbidden_term`.
- The party routes (`api::routes::parties`) gain phase 19's graph and network-case routes; `GET /cases/{id}` is
  unchanged.""",

"""### 7.N The party read predicate
The party rule of section 3.10 (party rows) is exposed as `PartyReadPredicate::allows(principal, party_id, context) ->
bool`, implemented here and used by this phase's party routes and by phase 26 (deal reads take the customer's party
rule). It reads `casework.effective_tier(party_id)` (phase 17) and the team and access-list clauses of section 3.10.""",
]

P19 = [
"""### 7.N Branch answers and the release event (phase 22)
Phase 22 extends this phase and owns the extension (its section 4.5 items 1 and 2):
- Section 3.2, `RfiService::release` step 3: the payload of `case.rfi_released` gains `recipient_kind` (`{rfi_id, kind,
  recipient_kind}`).
- Section 3.3: the body of `RfiService::record_answers` is split into `record_answers_in_tx(tx, actor, rfi_id,
  answers, source_kind)`, called by the desk route after its `rfi.answer` check and by phase 22's branch route after
  `branch.item_answer`; the branch path always writes `corroboration = 'none'`. The `corroborates` path also accepts an
  evidence object whose owner kind is `rfi_input` and whose owner id is the RFI (DR-201).
- `GET /internal-reports` carries the observation fields for M only (DR-385, phase 22 section 7+).""",

"""### 7.N Assistant QA populations (phase 28)
From phase 28 on (migration `0089`), `casework.qa_samples.population` gains `assistant_golden` and
`assistant_assisted`, and `source_kind` gains `assistant_draft`. `assistant_assisted` is the decisions whose
`ai_involvement.used` is true, drawn by the weekly draw of section 3.8 at `assistant.qa_sample_percent` with the draw's
existing seed rule, skipping decisions already in a sample; `assistant_golden` is drawn by `POST /qa/samples` (phase 28
section 3.6). The QA review screen shows the text of an `assistant_golden` draft to the reviewers of that sample.""",

"""### 7.N Alarm kinds of later phases
The CHECKs of `casework.alarms` are widened by the owners' migrations; this phase's DDL is not edited:
`post_order_debit`, `freeze_overdue`, `refrain_overdue` with subjects `freeze`, `refrain`, `ops_task` (phase 20,
`0066`); `report_filing_overdue`, `report_rework_overdue` with subjects `report`, `report_task` (phase 21, `0070`);
`liaison_report_overdue` with subject `liaison_report` (phase 22); `capacity_forecast` with subject `pool` (phase 31,
`0097`). Each owner adds its rows to the table of section 3.10 and raises through `AlarmService::raise`.""",

"""### 7.N Correspondent routes (phase 31)
From phase 31 on, `RfiService` gains `attach_pack` and `correspondent_list`, and the `rfis` router gains `GET
/rfis/correspondents` (`rfi.correspondent.list`) and `POST /rfis/{id}/packs` (`rfi.attach_pack`). Phase 31 owns
them.""",

"""### 7.N Surge sampling (phase 31)
From phase 31 on, while `capacity.surge_active` is true, the sampler of section 3.8 takes `max(its rate,
capacity.surge_qc_percent)`; no sample is removed by a surge. Open doubts 10 keeps the rest of M-54.""",

"""### 7.N Break-glass and the RFI team (phase 25)
From phase 25 on, a break-glass grantee enters `Rfi.assignees` only under purpose `break_glass` (phase 25 section 3.2
step 2), and under that purpose `51_platform.cedar` refuses every RFI action (phase 25 section 3.12). The rules of
`RfiWorkflowSubject` do not change.""",
]

P19C = [
"""### 7.N Screening hits and suppression records (DR-375)
Open doubts 9 is closed. `screening.hit` is written by phase 14's decision transaction for a party subject decided
`potential` or `true`, from phase 15 on (phase 14 section 7+); this phase appends its handler after phase 15's in the
ordered handler list. The suppression records sampled by `qa_suppression_sample` are the `alert.pair_memory` rows whose
`last_suppressed_at` falls in the ISO week.""",
]

P20 = [
"""### 7.N Account-class groups for freeze tasks (phase 26)
From phase 26 on, and only while `pack.islamic` is on (with the flag off every account is `unrestricted`, which is the
single task of this phase):
- Section 3.4 step 6: target accounts are grouped by class (phase 26 section 3.8, setting `islamic.account_class_map`);
  one `OpsTaskService::create` per non-empty group, action `freeze` (unrestricted), `restrict` (restricted) or `hold`
  (pooled). `freezes.ops_task_id` holds the unrestricted task when there is one, else the first group's; the others are
  found by `casework.ops_tasks.source_id = freeze.id`.
- The audit `freeze.created` gains `groups: {unrestricted, restricted, pooled}` (counts only).
- Section 3.5: `propose_release` creates one release task per group.
Phase 26 owns these changes; the two-person rule for pooled holds stays its Open doubts 9.""",
]

P20C = [
"""### 7.N Delisting by absence (DR-376)
Open doubts 22 is closed: phase 14 delists by absence (versions are immutable), sets `delisted_on` only from a
publisher date, and guards a silent drop with `screening.max_count_drop_pct` (phase 14 section 7+). The gate of section
3.5 stays as written.""",
]

P21 = [
"""### 7.N Draft update method
`PATCH /reports/{id}` is served by `ReportService::update_draft(tx, report_id, UpdateReportDraft, actor, if_match)`
under the rules of this phase (draft only, the narrative limit of section 3.2, `validated` back to `draft` clears the
validation, section 3.5). Phase 28 calls the same method for an accepted narrative.""",
]

P22 = [
"""### 7.N Break-glass notice codes (phase 25)
From phase 25 on (migration `0082` seeds the two template rows), `NoticeCode` has seventeen codes: phase 25 adds
`security.break_glass_waiting` (to CH and SEC, parameter `count`) and `security.break_glass_opened` (to SEC and M,
parameter `count`). They use a new recipient rule of the dispatcher (section 3.8), "active holders of the role", read
at send time from phase 01 `GrantRepository::active_holders(role)`. Neither text names a case, party, requester or
reason, and neither links to the request.""",
]

P22C = [
"""### 7.N Observation fields in the internal-report register (DR-385)
This phase owns the read extension of Open doubts 9 (a): the rows of phase 19's `GET /internal-reports` whose internal
report has a `casework.observations` row (`internal_report_id`) carry `observation {indicator_groups,
escalation_criteria, notes}` for M only; `notes` is decrypted for M only, and other readers get no `observation` key.
Phase 19's route, scope and Cedar action do not change.""",

"""### 7.N Drift and candidate notices (closes Open doubts 14)
Phase 07's line "No notifications (phase 22)" keeps a new candidate on the screens, the cockpit and an alert rule; it
promises no notice. Phase 10's open drift shows on T3. No drift or candidate topic is defined in this release; the
screens are the notice (phase 10 section 7+).""",

"""### 7.N Flag route and the hybrid scenario (DR-382)
Section 4.5 item 9 reads the flag route as phase 03's (`PUT /api/v1/admin/flags/{key}`); phase 06 seeds the flag. The
hybrid scenario of section 4.5 item 4 and Open doubts 2 is a register scenario with `observational = true` (phase 06),
not a scenario of a family `observation`.""",
]

P23 = [
"""### 7.N Handlers for the tables of later phases (DR-387)
Each later phase that inserts `admin.retention_assignment` rows adds, in the same release, one handler per table to the
registry of section 3.2, chosen by class and clock: R3 and R8 `owner` rows go in the party batch of section 3.5; R6,
R7 and R11 are `report_only` (DR-191), and so is any child of a `report_only` owner; R9 is `kept`; R10 is never
deleted; a `retirement` clock is never due. An R12 `age` row is deleted by the owner's named sweep (phase 22
`notification_sweep`, phase 29 `assess_ledger_sweep`) or, where the owner names none, by this engine's `record_purge`
by age in batches. A `closure` clock on the table's own column (for example `closed_at`, `ended_at`, `resolved_at`)
treats the row as its own owner: due when the column is set and on or before the cut-off. Check 4a covers these rows.
Two drops gain a step: from phase 27 on, the `alert.model_scores` rows of an `alert.alerts` partition are deleted
(`record_purge`, R5) before the partition is dropped; from phase 29 on, an `assess.decisions` partition is dropped only
when no `assess.holds` row references it.""",

"""### 7.N DELETE grants follow the handlers (DR-386)
`akr_retention` holds DELETE on exactly the tables whose handler deletes rows (`record_purge`, including the later
tables of the previous block), and none on a table whose handler is `report_only`, `kept`, `retirement` or never.
Partitions go through `admin.retention_drop_partition`. Check 4a also fails when a deleting handler lacks the grant or a
grant has no deleting handler. DR-198's "party tree and KYC family only" reads "the deleting handlers only".""",
]

P24 = [
"""### 7.N Register sources, fairness metric and closure codes of later phases
The CHECKs are widened by the owners' migrations; this phase's DDL is not edited. `admin.registers.source` gains
`model_monitor` (phase 27), `assistant_monitor` (phase 28, `0088`) and `assess_control` (phase 29, DR-307).
`admin.fairness_metrics.metric` gains `score_top_band_rate` (phase 27). The closure codes of section 3.4 gain
`RETRAINED`, `RECALIBRATED`, `RESTRICTED_USE`, `RULES_ONLY` and `JUSTIFIED_NO_ACTION` (phase 27); for source
`model_monitor` a code outside these five is 422 `E_VALIDATION`, sub-code `closure_code_not_allowed`.""",

"""### 7.N Review calendar rows and command-centre tiles of later phases
- Review calendar (section 3.4), derived rows, no table: `model_kill_drill` from `admin.models.next_kill_drill_due` and
  a starter family's `recalibration_due` (phase 27); `assistant_kill_drill` from
  `admin.assistant_kill_state.next_drill_due` (phase 28); `assess_control_test`, due `assess.control_test_max_days` after
  the last caught test per rail (phase 29). Amber 30 days before, red on the due date.
- Finding closure: the register route closes the release exceptions a finding of source `assess_control` names
  (phase 29 section 3.14 step 6).
- Command centre (section 3.7): the counts-only tiles `model_governance` (phase 27), the Assessment tile (phase 29) and
  the two tiles of phase 26.""",

"""### 7.N Model and assistant branches of the validation pack
From phase 27 on, `ValidationPackService::issue` (section 3.6) has the model branch of phase 27; from phase 28 on, the
assistant branch for the kinds `llm_component` and `prompt_template`, with the seven parts of section 3.6 (phase 28
section 3.9). The signature, the verifier and the pack kinds do not change.""",

"""### 7.N Two counts in the control report (phase 25)
From phase 25 on, `GET /access/control-report` gains two counts under this phase's small-number suppression:
break-glass grants activated in the month and access reviews overdue (phase 25 tables). Counts only.""",

"""### 7.N Practice column in the training register (phase 31)
From phase 31 on, `TrainingRegisterService::list` (section 3.10) gains the display column `practice`, computed by
`TrainingService::practice_column` (phase 31 section 3.2 rule 4). `admin.training_records` is unchanged.""",

"""### 7.N The register warning writer (DR-388)
This phase defines the port `RegisterWriter`, implemented by `GovernanceRegisterService`: `warn(tx, source,
source_key, title_key: WarningTitle, body) -> Option<Uuid>` inserts a row of kind `warning` with `ON CONFLICT
(source_key) WHERE source_key IS NOT NULL DO NOTHING` (index `uq_registers_source_key`; `None` when the key exists).
`WarningTitle` is a closed enum in `domain::governance` that maps each key to `title {ar, en}`; later phases add their
variants (phase 26, phase 29). `body` holds ids, codes and counts only. It is called only from jobs (section 1.1), and
this phase's own sweeps use it.""",

"""### 7.N The case review clock (DR-384)
The clocks board row `case.review` reads `casework.cases.opened_at` plus the clock's days for cases in `open` or
`in_review` (phase 18 section 7+). This closes Open doubts 19; the clocks table of section 3.8 reads this rule.""",
]

P24C = [
"""### 7.N S-48 and the activation instant (DR-380)
Open doubts 11: S-48 (harness diff) is P2 (DR-380); the other items of that list go to the coverage runs of gap pass 1.
Open doubts 30 is closed: the activation instant is `scn.scenario_versions.effective_from` (phase 16 section 7+).""",
]

TARGETS = [('10', CHDR, P10C), ('17', HDR, P17), ('18', HDR, P18), ('19', HDR, P19), ('19', CHDR, P19C),
           ('20', HDR, P20), ('20', CHDR, P20C), ('21', HDR, P21), ('22', HDR, P22), ('22', CHDR, P22C),
           ('23', HDR, P23), ('24', HDR, P24), ('24', CHDR, P24C)]

out = {}
for pp, hdr, blocks in TARGETS:
    p = PLAN / f'phase-{pp}.md'
    s = out.get(p, p.read_text())
    s = add_section7(s, hdr, blocks)
    i = s.index('## 7+. Gap Additions')
    j = s.find('\n## ', i + 5)
    tail = s[i:j if j != -1 else len(s)]
    for w in ('UNCLEAR', 'the lead decides', 'lead confirms'):
        assert w not in tail, (p.name, w)
    out[p] = s

p22 = PLAN / 'phase-22.md'
s = out[p22]
s = rep(s, '(phase 06 section 3, `PUT /api/v1/admin/flags/branch.observations_live`)',
        '(phase 03 route `PUT /api/v1/admin/flags/{key}`; phase 06 seeds the flag)')
s = rep(s, 'is seeded by the phase 16 scenario seed with family `observation`;',
        'is a register scenario with `observational = true` (phase 06, DR-382);')
s = rep(s, 'The bank names an observational scenario of family `observation` (phase 16 seed)',
        'The bank names a register scenario with `observational = true` (phase 06, DR-382)')
out[p22] = s

p24 = PLAN / 'phase-24.md'
s = out[p24]
s = rep(s, '| `case.review` | the case review column named by phase 18 (Open doubts 19; zero rows until named) |',
        '| `case.review` | `casework.cases.opened_at` plus the clock days, open and in-review cases (18, DR-384) |')
out[p24] = s

r = PLAN / 'research.md'
rs = r.read_text()
anchor = [l for l in rs.split('\n') if l.startswith('| DR-383 |')]
assert len(anchor) == 1
DRS = [
    "| DR-384 | 2026-10-10 | The clock `case.review` has no column: due = `casework.cases.opened_at` + the clock days for cases `open` or `in_review` | Phase 09 named the clock and phase 24 waited for a column no phase defined; the open date is the natural anchor (gap pass 1, phase 18 section 7+) |",
    "| DR-385 | 2026-10-10 | Phase 22 owns the observation fields on phase 19's internal-report register rows (M only, notes decrypted for M) | Phase 22 left the read unowned; it owns the observation data (gap pass 1, phase 19 and 22 section 7+) |",
    "| DR-386 | 2026-10-10 | `akr_retention` DELETE is granted on exactly the tables whose handler deletes rows; check 4a asserts it; DR-198 amended | Later phases granted DELETE on every new table, wider than DR-198 and than the handlers need (gap pass 1, phase 23 section 7+) |",
    "| DR-387 | 2026-10-10 | Later phases add one disposal handler per retention row by class; later R12 `age` rows are purged by the owner's sweep or the engine; `closure` on a table's own column; model scores deleted before an alert partition drop; decision partitions kept while a hold references them | The registry and its coverage test named only phase 00 to 23 tables (gap pass 1, phase 23 section 7+) |",
    "| DR-388 | 2026-10-10 | Phase 24 defines `RegisterWriter::warn(tx, source, source_key, title_key: WarningTitle, body) -> Option<Uuid>`, implemented by `GovernanceRegisterService`, insert-or-nothing on `source_key` | Phases 26 and 29 wrote warnings through a writer phase 24 never named (gap pass 1, phase 24 section 7+) |",
]
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + '\n'.join(DRS) + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
