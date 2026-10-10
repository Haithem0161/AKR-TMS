#!/usr/bin/env python3
"""Apply adversarial pass run 1 (four-eyes and self-approval) after the lead's checks.

Source: tools/agent/adv-run1.md (Haiku draft); decisions: tools/agent/adv-run1-verdict.md.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'

HDR = ('Adversarial pass (2026-10-10), run 1 (four-eyes and self-approval), from tools/agent/adv-run1.md; '
       'lead-verified (tools/agent/adv-run1-verdict.md).')

BLOCKS = {
    '04': [
        """### 7.N Emergency changes never touch access (ADV-1-01, HIGH; DR-407)
The first rule of `change.route_map` sends every `emergency` request to `cr_emergency` (one MLRO approval), ahead of the
access rules, and its review runs on the business family. That would let an access change escalate to a business role,
which the shipped definitions forbid ("Access changes never escalate to business roles"). Section 3.1 classify step 5 therefore refuses `emergency = true` (422 `E_VALIDATION`,
`emergency_not_allowed`) for the kinds `role_grant`, `policy_bundle` and `task_assignment`, and for a `setting` whose key
starts with `authz.`, `identity.`, `stepup.`, `session.`, `delegation.` or `access.`. These changes keep their access
route (`cr_access` or `cr_access_adm`), which is already one security approval. The emergency rule of the route map
then applies only to the other kinds and keys. Test: an emergency `setting` change on `stepup.max_failures` is 422
`emergency_not_allowed`.""",
        """### 7.N Rework is a reject mode (ADV-1-10, LOW; DR-407)
Section 3.4 delta 8 allows `rework`: it is the `mode` of a definition's reject, not a task action. A reject with mode
`rework` returns the instance to the maker and increments `pass` on every later task. In each pass the maker stays
excluded and the per-pass unique deciders apply; an approver of an earlier pass may approve a later pass. The case
definitions of phase 18 use `mode: end` (phase 18 section 7+).""",
    ],
    '21': [
        """### 7.N The drafter and the authors never sign (ADV-1-02, HIGH; DR-407)
Section 3.6 step 3 excludes the instance proposer, the submitter and the earlier deciders. The report's drafter
(`rpt.reports.created_by`) and the author of any version (`rpt.report_versions.created_by`) are also makers, so:
1. The excluded set of the `sign` task adds the report's `created_by` and the `created_by` of every version of the
   report (403 `E_SELF_APPROVAL`).
2. Cedar (section 3.22): `@id("report.sign_not_drafter") @deny("self_approval") forbid (principal, action ==
   Akr::Action::"report.sign", resource is Akr::Report) when { resource has created_by && resource.created_by ==
   principal };` (the entity already carries `created_by`).
3. Migration `0068` adds to `rpt.reports`: `CHECK (signed_by IS NULL OR created_by IS NULL OR signed_by <>
   created_by)`, next to the submitter CHECK.
A bank with one compliance officer has an A2 prepare the draft. Test: M1 creates the draft, an A2 edits and submits it,
M1's sign is 403 `E_SELF_APPROVAL`.""",
    ],
    '18': [
        """### 7.N Reopen after a filing is M only (ADV-1-08, MEDIUM; DR-407)
A closed `utr`, `str` or `tfr` case always has `case.filing_released` in its chain (section 3.12 step 5). Section 3.13
step 2 therefore chooses `case.reopen_late` (M only) for such a case whatever the window: an A2 reopen is 403
`E_FORBIDDEN`. From phase 21 on, the same rule reads the filing record that replaces the chain check. The reopen
still clears `outcome`; the decision rows, the filing and the earlier seals stay.""",
        """### 7.N No rework in the case definitions (ADV-1-10, LOW; DR-407)
Phase 04 rework is a reject mode (phase 04 section 7+). The case definitions of this phase use `mode: end`: a rejected
proposal ends the instance and the proposer proposes again. This closes Open doubts 16.""",
    ],
    '01': [
        """### 7.N Decision step-ups name their resource (ADV-1-04, LOW; DR-407)
For the action classes `decision`, `approval`, `signing` and `filing`, `consume` step 2 matches the resource exactly:
`resource_ref IS NOT DISTINCT FROM $6` in place of `(resource_ref IS NULL OR resource_ref = $6)`. An assertion started
without a resource is then good only for an action that passes none. The other classes keep the current predicate.
Test: an assertion of class `decision` started without `resource_ref` is 401 `E_STEPUP_REQUIRED` on a task decision.""",
    ],
    '15': [
        """### 7.N The approver of a downward override (ADV-1-09, LOW; DR-407)
A phase 04 task has one role, so "`approve` role CHK or M" (section 3.5 step 5 and section 4.2) is shipped as: one step
`approve` role CHK, not the proposer, not the relationship owner, timeout `esc` M after 3 business days ->
`record_decision`. The bank may change the role by a `workflow_definition` change request.""",
    ],
}

out = {}


def load(p):
    if isinstance(p, str):
        p = PLAN / f'phase-{p}.md'
    return p, out.get(p, p.read_text())


for pp, blocks in BLOCKS.items():
    p, s = load(pp)
    st = s.index('## 7+. Gap Additions')
    nx = s.find('\n## ', st + 5)
    assert 'Adversarial pass (2026-10-10), run 1' not in s[st:nx if nx != -1 else len(s)], pp
    out[p] = add_section7(s, HDR, blocks)

r = PLAN / 'research.md'
p, rs = load(r)
anchor = [l for l in rs.split('\n') if l.startswith('| DR-406 |')]
assert len(anchor) == 1
DR = ("| DR-407 | 2026-10-10 | Adversarial run 1 (four-eyes): emergency requests are refused for access kinds and access "
      "setting prefixes (phase 04); the report drafter and every version author are excluded from signing, with a "
      "Cedar forbid and a CHECK (phase 21); a case with a released filing is reopened by M only (phase 18); decision, "
      "approval, signing and filing step-ups match their resource exactly (phase 01); the downward override approver "
      "is role CHK escalating to M (phase 15); rework is a reject mode and the case definitions end on reject "
      "(phases 04, 18) | Each closes a path where one person or one business approver could defeat a stated "
      "separation; the rejected candidates were designed exceptions or already guarded (tools/agent/adv-run1-verdict.md) |")
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + DR + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
