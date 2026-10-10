p='docs/plan/phase-20.md'
orig=open(p).read()
R=[]
def r(old,new,n=1): R.append((old,new,n))
# 1 retention
r("This phase assigns R5 (age) to all five tables, because freezes are sanctions-tied records (ARCHITECTURE section 17, as phase 14 notes) and R7 is the alert class (DR-166; Open doubts 12).",
  "This phase assigns R7 (screening and sanctions records, ARCHITECTURE section 17, the class phase 14 uses) to all five tables, with clocks that start only when the record ends: `freezes` at `released_at`, `refrain_instructions` and `ops_incidents` at `closed_at`, `ops_tasks` and `ops_verifications` as their owner record (DR-166; Open doubts 12). A freeze in force is never due for deletion (lead review).")
r("""-- Retention (phase 00 clocks; R5 seven years from the record, as phase 15 and phase 19 use it for sanctions and alarm
-- records). The clock column is the record's own timestamp. DATA-MODEL s.13 R6 and R7 are replaced (DR-166).
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('casework', 'ops_tasks',            'R5', 'age', 'created_at',         'Operations tasks: follow the freeze, refrain or exit record'),
 ('casework', 'ops_verifications',    'R5', 'age', 'checked_at',         'Verification history of a task: kept with the task'),
 ('casework', 'freezes',              'R5', 'age', 'listing_decision_at', 'Freeze records, sanctions-tied (ten-year question in Open doubts 12)'),
 ('casework', 'refrain_instructions', 'R5', 'age', 'received_at',        'Refrain instructions of the Office'),
 ('casework', 'ops_incidents',        'R5', 'age', 'raised_at',          'Failure incidents: kept with the record they concern')""",
"""-- Retention (phase 00 clocks; R7 screening and sanctions records, ARCHITECTURE s.17, as phase 14). The clock starts when the
-- record ends, so a freeze in force or an open refrain is never due (DR-166). DATA-MODEL s.13 R6 becomes R7.
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('casework', 'ops_tasks',            'R7', 'owner',   NULL,          'Operations tasks: follow the freeze, refrain or exit record'),
 ('casework', 'ops_verifications',    'R7', 'owner',   NULL,          'Verification history: follows its task'),
 ('casework', 'freezes',              'R7', 'closure', 'released_at', 'Freeze records from the release; a freeze in force stays (ten-year question in Open doubts 12)'),
 ('casework', 'refrain_instructions', 'R7', 'closure', 'closed_at',   'Refrain instructions of the Office, from the close'),
 ('casework', 'ops_incidents',        'R7', 'closure', 'closed_at',   'Failure incidents, from the close')""")
r("The five tables are retained under R5 (seven years from the record) instead of R6 and R7, because freezes are sanctions-tied records (ARCHITECTURE section 17) and R7 is the alert class; the ten-year question stays open (Open doubts 12).",
  "The five tables are retained under R7 (screening and sanctions records, ARCHITECTURE section 17, as phase 14), with clocks that start at the release or the close of the record (tasks and verifications follow their owner), so a freeze in force is never due for deletion; the ten-year question stays open (Open doubts 12).")
r("this phase uses R5 (seven years from the record), as phase 15 and phase 19 do.","this phase uses R7 from the release or the close, as phase 14 does for its screening records.")
r("the R5 class is assigned here","the R7 class is assigned here")
# 2 evidence upload route
r("an evidence item already staged for the owner kind `ops_task` and this task (section 3.3); the staging itself is `POST /evidence` of phase 02 and is not repeated here",
  "an evidence item already staged for the owner kind `ops_task` and this task through `POST /ops-tasks/{id}/evidence` (section 3.3; added here, because phase 02 has no generic upload route)")
r("| POST | `/api/v1/ops-tasks/{id}/attest` |",
  "| POST | `/api/v1/ops-tasks/{id}/evidence` | `case.decide` | Upload the attestation screenshot through `EvidenceStore::stage` (owner kind `ops_task`; Cedar `ops.verification.attest`); 422 `E_UPLOAD_REJECTED` on an antivirus or type refusal; `Idempotency-Key` (phase 20, added) |\n| POST | `/api/v1/ops-tasks/{id}/attest` |")
r("The route table has 21 rows: 11 owned rows of API.md 3.14 (the `POST /exits` row is not built here, see below) and 10 added rows.",
  "The route table has 22 rows: 11 owned rows of API.md 3.14 (the `POST /exits` row is not built here, see below) and 11 added rows.")
r("**Complexity:** L (21 routes: 11 rows of API.md 3.14 built here, 10 rows added;","**Complexity:** L (22 routes: 11 rows of API.md 3.14 built here, 11 rows added;")
r("Swagger shows the 21 routes","Swagger shows the 22 routes")
r("**API.md 3.14** gains the ten rows of section 2","**API.md 3.14** gains the eleven rows of section 2")
r("`usePostOpsTasksIdAttest`,","`usePostOpsTasksIdEvidence`, `usePostOpsTasksIdAttest`,")
r("the \"Attest\" action (**SU**, evidence upload through phase 02 first)","the \"Attest\" action (**SU**, after the screenshot upload through `POST /ops-tasks/{id}/evidence`)")
# 3 StartSpec
r("4. `WorkflowEngine::start(tx, StartSpec { definition: ops_attestation, resource_kind: ops_task, resource_id: task id, maker: actor, params: { evidence_id } })` (phase 04). The instance is the proposal.",
  "4. `WorkflowEngine::start(tx, StartSpec { definition_key: \"ops_attestation\", resource: (\"ops_task\", task id), purpose: Decision, initiator: actor, maker: actor })` (phase 04; `StartSpec` carries no parameters). `OpsTaskWorkflowSubject::start_context` freezes the attribute `ops.evidence_id` (the clean upload of step 3, the latest when several exist) into `workflow_instances.context`, so the effect reads the evidence from the instance, not from a later upload. The instance is the proposal.")
r("insert `ops_verifications` (kind `attestation`, level `v3`, `evidence_id`,","insert `ops_verifications` (kind `attestation`, level `v3`, `evidence_id` from `inst.context`,")
r("5. `WorkflowEngine::start(tx, StartSpec { definition: freeze_release, resource_kind: freeze, resource_id, maker: actor, params: { reason_code } })`. The maker is the proposer (four-eyes, phase 04).",
  "5. `WorkflowEngine::start(tx, StartSpec { definition_key: \"freeze_release\", resource: (\"freeze\", freeze id), purpose: Decision, initiator: actor, maker: actor })`; the reason code (the only release code) is frozen by `FreezeWorkflowSubject::start_context` as `freeze.reason_code`. The maker is the proposer (four-eyes, phase 04).")
# 4 config fallback
r("(default 8; the service refuses a value outside 1 to 8, `E_INTERNAL` with `Config`)",
  "(default 8; a value outside 1 to 8 never fails the screening decision: the service uses 8 and writes audit `ops.config_fallback {key, value}`, lead review)")
r("bounds 1 to 8 checked by `FreezeService`)","bounds 1 to 8: the phase 04 `setting` change target refuses a value outside them, and `FreezeService` falls back to 8 if one is found)")
r("| `Config { key }` | `E_INTERNAL` | 500 | a setting outside its bounds (`ops.freeze_task_hours`) |",
  "| `Config { key }` | `E_INTERNAL` | 500 | a setting outside its bounds on a request path; never returned by `FreezeService::start`, which falls back to 8 (`ops.freeze_task_hours`) |")
r("a setting of 0 or 9 is refused (`Config`);","a setting of 0 or 9 falls back to 8 with the audit `ops.config_fallback`;")
r("| `ops.letter_recorded` |","| `ops.config_fallback` | `FreezeService::start` used 8 hours for an out-of-bounds setting (key, value) | freeze |\n| `ops.letter_recorded` |")
# 5 holders and curl h
r("holders by phase 04 `Holders` rule 5 as extended: role `A2` or `M` at scope bank, excluding the maker and the completer",
  "holders by phase 04 `Holders` from the step's assignee `{kind: role, role: A2, scope: bank}`, excluding the maker; the completer is refused by `check_before_decision`")
r("   - h. `m@` (the requester of the attestation) `POST /api/v1/tasks/{task_id}/actions/approve` on the `ops_attestation` task: 403 `E_SELF_APPROVAL` (the maker exclusion of phase 04). `a2@` approves with step-up: 200;",
  "   - h. `a2@` (the requester of the attestation) `POST /api/v1/tasks/{task_id}/actions/approve` on the `ops_attestation` task: 403 `E_SELF_APPROVAL` (the maker exclusion of phase 04). `a2b@` (role A2 at bank scope, the step's assignee) approves with step-up: 200;")
# 6 phase 18 behaviour
r("false: 403 `E_FORBIDDEN`, sub `role_cannot_decide`; a permit with `second_review`: 422 `E_VALIDATION`, sub `second_review_not_shipped` (phase 18 behaviour, repeated).",
  "false: 403 `E_FORBIDDEN`, sub `role_not_permitted`; a permit with `second_review`: 409 `E_NOT_ALLOWED_IN_STATE`, sub `second_review_not_shipped` (phase 18 behaviour, DR-143).")
r("`unknown_party`, `role_cannot_decide`, `delegation_refused`","`unknown_party`, `role_not_permitted`, `delegation_refused`")
# 7 effect resource kinds
r("and `create_ops_task` (resource kinds `freeze`, `refrain`, `case`, and `exit` as phase 15 uses it).",
  "and `create_ops_task` (resource kind `exit_decision`, the instance resource kind of phase 15; freeze and refrain tasks are created by the services directly).")
# 8 lift tasks
r("1. Read the task and its owner without a lock; a task that is not `completed`, or whose owner is `released` (freeze) or `closed` (refrain), is a no-op (the phase 00 timer rule: the handler re-reads its owner).",
  "1. Read the task and its owner without a lock; a task that is not `completed` is a no-op. A hold task (action `freeze`, `restrict`, `hold` or `exit`) whose owner is `released` (freeze) or `closed` (refrain) is a no-op (the phase 00 timer rule: the handler re-reads its owner). A `release` task follows the lift rule below instead of steps 2 to 10.")
r("**Confirm (V0)** (`POST /ops-tasks/{id}/confirm`",
  "**Lift rule** (`release` tasks; lead review): a lift task reaches V1 when the copy is current (`canon_as_of` on or after the effective date) and no target account has an open hold of the V1 types carrying the `cbs_reference` of the hold task it lifts (the owner's `ops_task_id`); a lift has no V2 and no debit watch, and it is checked whatever the owner's state, until V1. A lift still below V1 with a current copy after its due time raises incident `task_failed` with payload `{lift_pending: true}`. V0 by a second operator is allowed as for a hold task.\n\n**Confirm (V0)** (`POST /ops-tasks/{id}/confirm`")
# 9 hold types, sweep order
r("a hold row of type `block`, `aml_freeze`, `sanctions_freeze` or `cbi_order`","a hold row of type `block`, `restriction`, `aml_freeze`, `sanctions_freeze` or `cbi_order`")
r("`OpsTaskRepository::needing_check(limit 500)` returns the completed tasks whose owner is not released (freeze) or closed (refrain), regardless of level: the post-order watch lasts until the release, not until V3.",
  "`OpsTaskRepository::needing_check(limit 500)` returns, oldest last check first (so no task starves), the completed hold tasks whose owner is not released (freeze) or closed (refrain), regardless of level (the post-order watch lasts until the release, not until V3), and the completed `release` tasks below V1.")
# lead corrections
r("\n## Open doubts (for gap pass 1 and the bank)\n",
  "\n**Lead corrections (review 2026-10-09).** Retention is R7 (ARCHITECTURE section 17 gives R7 to screening and sanctions records; the draft called R7 the alert class and used an age clock from the listing, which would have deleted a freeze still in force). The attestation screenshot needs its own upload route (`POST /ops-tasks/{id}/evidence`): phase 02 has no generic `POST /evidence`. `StartSpec` is `{definition_key, resource, purpose, initiator, maker}` with no parameters; the evidence id and the release reason are frozen into the instance context by `start_context`. An out-of-bounds `ops.freeze_task_hours` falls back to 8 instead of failing the phase 14 true-match decision. `second_review_not_shipped` is 409 `E_NOT_ALLOWED_IN_STATE` and the refusal sub-code is `role_not_permitted`, as phase 18 has them. A lift task is verified by its own rule (no open hold with the lifted reference), because the check skipped tasks of released owners. The V1 hold types include `restriction`; the sweep takes the oldest checks first. `create_ops_task` runs on the `exit_decision` resource kind. The curl attestation step uses `a2b@` as the approver.\n\n## Open doubts (for gap pass 1 and the bank)\n")
t=orig
for old,new,n in R:
    c=orig.count(old)
    assert c==n,(c,n,old[:70])
for old,new,n in R:
    t=t.replace(old,new)
open(p,'w').write(t)
print("applied",len(R))
