#!/usr/bin/env python3
"""Apply gap pass 1, run 2 (targets 03, 04) after the lead's checks.

Source: tools/agent/gap1-run2.md (Haiku draft); decisions: tools/agent/gap1-run2-verdict.md.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'

HDR = ('Gap pass 1 (2026-10-10), run 2, from tools/agent/gap1-run2.md; lead-verified '
       '(tools/agent/gap1-run2-verdict.md).')
CHDR = ('Gap pass 1 (2026-10-10), run 2: corrections found while checking phases 03 and 04 '
        '(tools/agent/gap1-run2-verdict.md); lead-verified.')

P04 = [
"""### 7.N WorkflowSubject: check_before_decision
Section 2, trait `WorkflowSubject`, gains one method (phase 18 section 4.5 item 1 states it):

    async fn check_before_decision(&self, tx: &mut Tx, inst: &WorkflowInstance, actor: &AuthContext,
        task: &Task) -> Result<(), WorkflowError>;   // default body: Ok(())

Section 3.4 step 6 reads: "Subject check before decision: the subject's `check_before_decision` runs in the same
transaction, before the action applies; the change subject runs `ChangeTarget::check_before_decision` through it. A
refusal rolls the action back and the task stays open." Subjects with no extra rule keep the default
(`CaseWorkflowSubject`, phase 18; `RfiWorkflowSubject`, phase 19; `FreezeWorkflowSubject` and
`OpsTaskWorkflowSubject`, phase 20, implement it).""",

"""### 7.N Evidence upload for change requests (DR-363)
The evidence owner kind `change_request` is registered in the phase 02 registry (owner id = the request id; phase 02
7.10). New route (gap item (d)):

| Method | Path | Scope | Description |
|-|-|-|-|
| POST | `/change-requests/{id}/evidence` | config.author, admin.users, admin.policies, admin.system | The maker attaches a file to their request while it is `draft`, or `submitted` with no decision recorded yet; `Idempotency-Key`; `If-Match` (request version); 201 with the evidence id |

Steps: only the maker (else 403 `E_FORBIDDEN`); status `draft`, or `submitted` with no `approve` or `reject` event in
the current pass (else 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `evidence_closed`); `EvidenceStore::stage` with owner
kind `change_request`; append `{kind: evidence_object, id}` to `change_requests.evidence`; version + 1. Audit
`change_request.evidence_added {evidence_id, kind}` (resource `change_request`, SIEM no). A request created through a
dedicated route (a governed setting, a grant, a policy bundle) receives its new documents the same way, before its
first decision; the freshness rule of section 3.2 applies at each deciding action as before.""",

"""### 7.N Engine step 5 passes the declared step-up action
Section 3.4 step 5 reads: "Deciding steps and every `sign`: `StepUpService::consume(tx, ctx, header, action,
resource_ref = "task:<id>")` (phase 01 signature), where `action` is the declared `StepUpAction` of the act:
`task.approve` and `task.reject` (class `approval`), `task.sign` (class `signing`); on the change routes
`change.approve` and `change.reject` (class `approval`). A subject's own constants are passed the same way. 401
`E_STEPUP_REQUIRED` as before; the assertion id goes to `tasks.stepup_id`." """.rstrip(),

"""### 7.N Holders rule 5 (case_acl)
Section 3.5 rule 5 reads: "`case_acl`: an active `casework.case_acl` row with `access = decide` on the instance's case;
when the instance's definition is `case_report_approval`, the principal also holds role `M` (phase 18 section 4.5
item 2). The inbox, the Task adapter and the engine read this one rule. Until phase 18 ships the case subject, a
definition that names `case_acl` is refused at publish (`E_ASSIGNEE_UNAVAILABLE`)." """.rstrip(),

"""### 7.N Approver ids of open change requests (DR-92)
`ChangeRequestService` gains the read method `approver_ids(conn, object_kind, object_key) -> Vec<Uuid>`: for every
`submitted` request on that object, the users who may hold its open deciding task (`Holders::eligible`, section 3.5)
and the users who already decided in the current pass. The maker is not included (phase 11 admits the requester by
its own rule). Read only, no step-up, no audit row. Phase 11 sets the Cedar attribute `resource.approver_ids` of a
mapping replay from it (phase 11 Open doubts 7 closed).""",

"""### 7.N A validation pass sets the inventory dates
Section 3.9 "Update" gains: "A validation pass (phase 24 section 3.6 step 8) calls `InventoryService::update` in the
validation transaction with the validator's context. It sets `last_validated_at` and `next_validation_due` and nothing
else; the status does not change, so no step-up applies. Audit `inventory_item.updated {fields, status_from,
status_to}` with `status_from = status_to`." The sentence "Next validation dates are entered by the model-risk officer
here" reads: "The model-risk officer enters a date by hand, or a passing validation sets it (phase 24); phase 27
computes due dates from the tier and lists overdue items." """.rstrip(),

"""### 7.N role_grant revoke and the platform_release target (phase 25)
- Section 3.3, `role_grant` gains the operation `revoke` (phase 25 section 3.1, DR-234): maker SEC, route
  `cr_access_adm`, checker ADM. Apply: `UPDATE admin.role_grants SET revoked_at = now(), revoked_by = <checker> WHERE
  id = $grant AND revoked_at IS NULL`, then `invalidate_user`; the apply writes `role_grant.revoked` (phase 03 kind). A
  proposal for a grant that is not active: 422 `grant_not_active`, no request created. The phase 03 `DELETE
  /admin/users/{id}/grants/{grant_id}` is unchanged.
- New target kind `platform_release` (phase 25 section 4.5): route `cr_access`, object key
  `release:<release_version>`, `volume_effect = neutral`, evidence kind `package` with owner kind `release` (phase 02
  7.10), its `EvidenceResolver` registered with the target.
- The `change.route_map` seed of section 1 lists the kind in the second rule, so no later migration edits the map:
  `{"object_kind": ["role_grant", "policy_bundle", "task_assignment", "platform_release"], "route": "cr_access"}`. The
  first rule (maker SEC -> `cr_access_adm`) keeps its three kinds. Before phase 25 registers the target, the extra name
  matches nothing.""",

"""### 7.N Notice requests from reminders, escalations and the notify effect (phase 22)
From phase 22, which registers the topic `notify.request` and its handler in the same release (phase 22 section 4.5
item 5):
- `remind` writes, in its transaction, the outbox row `notify.request {task_id, template_code: "task.overdue", to:
  "holders"}`; `escalate` writes `notify.request {task_id, template_code: "task.waiting", to: "escalation_target"}`.
- The terminal effect `notify` is `NotifyEffect` (all resource kinds) with params `{"template_code": <a closed code of
  phase 22 section 1.2>, "to": "initiator" | "subject_team"}`; it writes `notify.request {workflow_instance_id,
  template_code, to}`. Until then `notify` stays deferrable (section 3.4) and no row is written.""",

"""### 7.N Volume effect is fixed at submit (DR-360)
A request's class and `volume_effect` are computed at submit (section 3.1) and are not recomputed. At each deciding
action, a target whose volume depends on a measurement (a replay, phase 16; a reader measure, phase 13) re-measures
in its `check_before_decision`; when the result differs from the stored `volume_effect`, the decision is refused with
422 `E_VALIDATION`, sub-code `volume_effect_changed`, and the maker withdraws and submits again, so the approvers
always decide on the class they see. No recompute entry point exists (phase 16 Open doubts 12 closed).""",

"""### 7.N One setting caps an assessor grant (DR-361)
The cap of an `AS` grant is the phase 04 key `access.assessor_max_days` (prefix `access.`, route `cr_access`). Its seed
becomes default `30` with the floor `{"max": 30, "basis": "M-62 time-boxed read-only assessor grant"}`, and the
settings validation of section 3.10 reads 1-30 instead of 1-365. Phase 24's key `assessor.grant_max_days` is not created;
phases 24 and 25 read this key (their 7.x record the correction).""",

"""### 7.N Families and routes added by later phases
Section 3.1 family list: phase 04's families are `access`, `setting`, `workflow` and `dev`. Later families are added
by the phase that owns them: `detection`, `mapping`, `kyc_model`, `reference` (as already written), `source`
(phase 05) and `reporting` (phase 07). Route rules named by later phases (phase 05 migration 0019, phase 09
`cr_authority`) are written by those phases. The nine rows of the shipped table are this phase's set; definitions
shipped by later phases (09, 15, 18 to 21, 23) are listed in those phases.""",
]

P03 = [
"""### 7.N Masked kinds register by phase
The registry `MASKED_KINDS` is open: a phase registers each masked kind it introduces, with its complete probe set
(section 3.6), in the phase that first serves the record. Registered so far:

| Masked kind | Phase |
|-|-|
| `_dev_masked` (development only) | 03 |
| `task_restricted` | 04 |
| `screening_alert`, `pair_memory` | 14 |
| `customer`, `kyc_review`, `override`, `pep_entry` | 15 |
| `replay_sample` | 16 |
| `alert`, `case` | 17 |
| `rfi` | 19 |
| `branch_item`, `liaison_report` (phase 22 names them by their Cedar entities `BranchItem`, `LiaisonReport`) | 22 |
| `audit_event`, `audit_target` | 24 |

Section 2's list "(customer, alert, case, report, evidence, RFI, freeze)" and the header's "the tiered resources
arrive from phase 15" read "the masked kinds of this table": reports, evidence and freezes are masked through the
case rule, not as kinds of their own (phase 20). Phase 14's deviation note (Open doubts) is closed by this rule.""",

"""### 7.N Purpose code for report filing (DR-362)
Migration 0011's purpose-code seed gains one row (phase 21 Open doubts 13):

    ('report_filing', '{"ar": "تقديم بلاغ", "en": "Report filing"}', '{restricted,suspicion}', FALSE, 45)

It is the purpose of the report package download (phase 21 section 3.9). Other reporting reads use the existing
codes (`report_preparation`, `qa_review`, `internal_audit`, `authority_request`).""",

"""### 7.N Cockpit tiles and probe inputs added by later phases
The tile table of section 3.10 is extended by the tiles each phase registers through `OpsCockpit::snapshot()`, with
the thresholds and states that phase gives: parked tasks with their ages (phase 04 section 3.6); Sources (phase 05
section 3.10); the goAML line (phase 07 section 3.8); the three reference-data tiles (phase 09 section 3.8); the
tiles of phase 25 section 3.10; the eight tiles of phase 30 section 3.13 (the replication lag tile covers the DR
replica). `ops_probe` also computes the inputs and the outage detection of phase 30 section 3.8. A tile whose probe
snapshot is stale shows unknown (existing rule).""",

"""### 7.N Flag guards and change hooks
`FlagsService::update` consults two registries, both empty in this phase:
- `FlagGuard` (one per key, registered by the phase that owns the flag): runs before the write; a refusal is 409
  `E_NOT_ALLOWED_IN_STATE` with the guard's sub-code. Phase 26 registers the `pack.islamic` guard (no FALSE while a
  `live` islamic version exists); phase 28 registers the same pattern for `pack.assistant`.
- `FlagChangeHook` (registered by the consuming phase): runs in the same transaction after the write. Phase 06
  registers the hook that enqueues `coverage_grade {reason: "flag"}` when `branch.observations_live` or
  `pack.islamic` changes.
Audit `flag.updated` is unchanged.""",

"""### 7.N Ctx.via values
The comment on `Ctx.via` reads `// "session" | "pat" | "client_credentials" | "gateway_cert" | "webhook_hmac"`:
`gateway_cert` is a gateway listener with a pinned certificate (phase 29), `webhook_hmac` the directory webhook
(phase 31). The type is unchanged.""",

"""### 7.N Window limit from the authority matrix
Port `DelegationAuthority` gains `window_limit_days(roles: &[RoleCode]) -> Option<u32>` (phase 09 section 3.6: the
smallest `max_days` of the roles); `NoMatrix` returns `None`. `DelegationService` step 2 reads: "length at most
`delegation.max_days` and, when the port returns `Some(n)`, at most `n` days." The phase 03 test that expects
`NoMatrix` asserts the `None` path.""",

"""### 7.N One branch per liaison or alternate (phase 22, DR-205)
`BranchService::update` takes `pg_advisory_xact_lock` on each user it assigns and refuses a user who is already liaison
or alternate of another active branch: 422 `E_VALIDATION` at `/liaison_user_id` or `/alternate_user_id`, sub-code
`already_liaison_elsewhere`. Backstop in migration 0012, replacing the two plain indexes:

    CREATE UNIQUE INDEX IF NOT EXISTS uq_branches_liaison   ON admin.branches (liaison_user_id)
        WHERE liaison_user_id IS NOT NULL;
    CREATE UNIQUE INDEX IF NOT EXISTS uq_branches_alternate ON admin.branches (alternate_user_id)
        WHERE alternate_user_id IS NOT NULL;

(An archived branch has both columns NULL by its CHECK.) The case of one user as liaison of one branch and alternate
of another is caught by the service rule only.""",

"""### 7.N UserRoleQuery port (phase 22)
Ports gain `UserRoleQuery { holders(role: RoleCode, scope: HolderScope) -> Vec<Uuid> }` over the effective roles
(direct grants and active delegations, the source of `User.roles` in the bundle), with `HolderScope { Any,
Branch(Uuid), Pool(Uuid) }`: `Any` is every active holder, `Branch(b)` a holder whose grant scope is
`all_branches` or lists `b`, `Pool(p)` a holder whose grant scope lists `p`. Read only, no audit row; used by the
phase 22 dispatcher.""",

"""### 7.N Outbox topic for credential resets (phase 22)
From phase 22, which registers the topic and its handler in the same release, `CredentialResetService::request`
writes in its transaction the outbox row `credential_reset.requested {reset_id}`. Section 4 "Outbox topics" reads:
"`credential_reset.requested` (produced in section 3.8 from phase 22; handled by phase 22)." """.rstrip(),

"""### 7.N Directory extensions to user administration (phase 31)
- Section 3.7 gains `UserAdminService::disable_from_directory` beside `disable` (its body and refusals are phase 31
  section 3.12): a reversible disable; grants, delegations and liaison assignments stay.
- `enable` refuses a linked account whose directory row has `directory_active = FALSE`: 409 `E_NOT_ALLOWED_IN_STATE`,
  sub-code `directory_inactive`. It reads the link through the port `DirectoryLinkQuery { is_inactive(user_id) ->
  bool }`, whose phase 03 implementation `NoDirectory` returns false; phase 31 supplies the real one.
- `user.disabled` gains the detail key `source`: `admin` for `disable`, `directory` for `disable_from_directory`.""",

"""### 7.N staff_id in the user update (phase 15)
`UpdateUser` gains `staff_id?` (ADM only). The column, its unique index and the grant are phase 15's (its migration);
this phase only accepts the field. The `PATCH /admin/users/{id}` row keeps the `admin.users` scope and names the field
as ADM-only.""",

"""### 7.N akr-ctl policy matrix (phase 08)
The `akr-ctl policy` commands of section 3.1 gain `akr-ctl policy matrix --bundle <dir> [--json]` (zone Z5):
`AccessMatrixBuilder` over the bundle directory with no database, printing `{bundle_hash, roles: {<role>: [<action>,
...]}}` from the permits' `@role` annotations (phase 08 section 2). No audit row (no database); `make generate-api`
writes it to `frontend/src/app/policy-matrix.json`, and the release check of phase 25 records the hash it reads.""",

"""### 7.N Development routes (DR-364)
Every development path (phase 00 `/_echo`, phase 03 `/_dev/masked`, phase 17 `/_dev/detections`, and the phase 04
subjects `_dev_object` and `_dev_masked`) is compiled only with the cargo feature `dev-routes`, which the release
build never enables, and in any build is mounted only when `AKR_ENV != production`. Section 6 gains a release-build
check: the inventory of development paths is listed in one file, and the release build answers 404 for each. Later
phases add their paths to that inventory.""",
]

CORR = {
    '02': ["""### 7.N Evidence owner kinds added by run 2
The registry of 7.1 also holds `change_request` (phase 04 7.2, the change-request evidence route; DR-363) and
`release` (phase 25, the evidence of a `platform_release` request; phase 04 7.7)."""],
    '24': ["""### 7.N The assessor grant cap
The setting `assessor.grant_max_days` is not created: the cap of an `AS` grant is phase 04's
`access.assessor_max_days` (default 30, maximum 30; phase 04 7.10, DR-361). Section 1's seed row and section 3.9
read accordingly."""],
    '25': ["""### 7.N The assessor grant cap
Every mention of `assessor.grant_max_days` (the settings table and Open doubts 15) reads `access.assessor_max_days`
(phase 04 7.10, DR-361); the cap stays 30 days."""],
    '27': ["""### 7.N Change targets belong to phase 04's registry
Section 4.5's "Phase 03: ChangeTargetRegistry gains the two targets" reads "Phase 04": the registry is phase 04's."""],
}

out = {}
p = PLAN / 'phase-04.md'
out[p] = add_section7(p.read_text(), HDR, P04)
p = PLAN / 'phase-03.md'
out[p] = add_section7(p.read_text(), HDR, P03)
for pp, blks in CORR.items():
    p = PLAN / f'phase-{pp}.md'
    out[p] = add_section7(p.read_text(), CHDR, blks)

for p, s in out.items():
    tail = s[s.index('## 7+. Gap Additions'):]
    for w in ('UNCLEAR', 'the lead decides', 'lead confirms'):
        assert w not in tail, (p.name, w)

# API.md: the new route row after the withdraw row
a = ROOT / 'docs' / 'API.md'
s = a.read_text()
row = [l for l in s.split('\n') if l.startswith('| POST | /change-requests/{id}/withdraw |')]
assert len(row) == 1
s = rep(s, row[0] + '\n', row[0] + '\n' + '| POST | /change-requests/{id}/evidence | config.author, admin.users, '
        'admin.policies, admin.system | The maker attaches a file while the request is a draft, or submitted with no '
        'decision yet; `Idempotency-Key`, `If-Match`; 201 (phase 04 7.2) |\n')
out[a] = s

# research.md DRs
r = PLAN / 'research.md'
rs = r.read_text()
anchor = [l for l in rs.split('\n') if l.startswith('| DR-359 |')]
assert len(anchor) == 1
DRS = [
    "| DR-360 | 2026-10-10 | A change request's class and volume effect are fixed at submit; a changed measurement at decision refuses the act (`volume_effect_changed`) and the maker resubmits | Approvers always decide on the class they see; no recompute path to secure (gap pass 1, phase 04 7.9) |",
    "| DR-361 | 2026-10-10 | One cap for assessor grants: `access.assessor_max_days`, default 30, maximum 30; phase 24's `assessor.grant_max_days` is not created | Phase 04 owns the grant check; the `access.` prefix routes it to `cr_access`; M-62 asks for 30 days (gap pass 1, phase 04 7.10) |",
    "| DR-362 | 2026-10-10 | Purpose code `report_filing` (restricted and suspicion) for the report package download | Phase 21 Open doubts 13; other reporting reads use existing codes (gap pass 1, phase 03 7.2) |",
    "| DR-363 | 2026-10-10 | Change-request evidence is uploaded by the maker through `POST /change-requests/{id}/evidence` (owner kind `change_request`) before the first decision | Gap item (d); no generic upload route exists; dedicated-route requests need the same path (gap pass 1, phase 04 7.2) |",
    "| DR-364 | 2026-10-10 | Development paths are compiled only with the cargo feature `dev-routes` and mounted only when `AKR_ENV != production`; a release-build test checks each is absent | Phases 04 and 17 gated by build, phases 00 and 03 by environment; both gates together (gap pass 1, phase 03 7.13) |",
]
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + '\n'.join(DRS) + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
