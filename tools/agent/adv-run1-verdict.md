# Adversarial pass run 1: lead verdict (2026-10-10; decisions final, applied by apply-adv-run1.py)

Haiku draft tools/agent/adv-run1.md (four-eyes and self-approval): 19 candidates; FLAW 3 (HIGH 2, LOW 1), DOUBT 7,
GUARDED 9. About 229k tokens.
Lead result: 6 confirmed (HIGH 2, MEDIUM 1, LOW 3), 4 rejected, 9 guarded accepted as checked.

## Confirmed
- ADV-1-01 HIGH (phase 04). The route map's first rule `{"class": "emergency", "route": "cr_emergency"}` wins over the
  access rules, so a `setting` under `authz.`, `identity.`, `stepup.`, `session.`, `delegation.` or `access.` (and any
  access kind whose target allows emergency) goes live on one MLRO approval, and its review runs on the business family.
  This breaks "Access changes never escalate to business roles". Fix: classify step 5 refuses `emergency = true` for
  `role_grant`, `policy_bundle`, `task_assignment` and the access setting prefixes (`E_VALIDATION`,
  `emergency_not_allowed`). `cr_access` is already one security approval, so emergency adds no speed for them.
- ADV-1-02 HIGH (phase 21). The sign task excludes the instance proposer, the submitter and earlier deciders; the CHECK
  compares only `signed_by` with `submitted_by`. An M who created the draft (`created_by`) or wrote a version
  (`rpt.report_versions.created_by`) can sign after an A2 submits it. Fix: the engine's excluded set for the sign task
  adds the report's `created_by` and every version's `created_by`; Cedar forbid on `report.sign` when
  `resource.created_by == principal`; `0068` gains `CHECK (signed_by IS NULL OR created_by IS NULL OR signed_by <>
  created_by)`.
- ADV-1-08 MEDIUM (phase 18). Closing a `utr`, `str` or `tfr` case needs `case.filing_released`, so every such closed
  case has a released filing; A2 alone may still reopen it within the window and clear `outcome`. Fix: a case whose
  chain has `case.filing_released` (from phase 21 on, a filed report) is reopened only through `case.reopen_late`
  (M only), whatever the window.
- ADV-1-04 LOW (phase 01). `consume` accepts an assertion with a NULL `resource_ref` for any resource. Fix: for the
  classes `decision`, `approval`, `signing` and `filing` the predicate is `resource_ref IS NOT DISTINCT FROM $6`.
- ADV-1-09 LOW (phase 15). "`approve` role CHK or M" cannot be built: a task has one role (`assignee_kind = 'role'`).
  Fix: `approve` role CHK, timeout `esc` M after 3 business days (the phase 04 pattern).
- ADV-1-10 LOW (phases 04, 18). Phase 04 says `rework` is allowed; phase 18 Open doubts 16 says there is no rework.
  Fix: rework is a reject mode of a definition, not a task action; the shipped case definitions use `mode: end`; in a
  rework pass the maker stays excluded and an approver of an earlier pass may approve again. Closes phase 18 Open
  doubts 16.

## Rejected
- ADV-1-03 (freeze release while a re-listing activates): in serial order the release is valid when it commits; the
  activation's look-back raises a new alert and the freeze path follows. No lock needed.
- ADV-1-05 (one person dismisses a freeze-list hit): the decision CHECK `NOT (decider_id = ANY (prior_decider_ids))`
  applies to every decision, so the person who escalated to `potential` cannot clear it; `open` to `incorrect` by one
  person is the designed first-level clear (MODULES M9: "four-eyes freeze `true`").
- ADV-1-06 (a tightening emergency change stays live unreviewed): designed (DR-55); a tightening is never reverted
  because that would loosen a control; it escalates to CH. With ADV-1-01, access keys cannot be emergency.
- ADV-1-07 (scheduled CTR batch confirmed by one M): designed; the job is the maker and the M the checker.

## Guarded (accepted as checked)
ADV-1-11 to ADV-1-19: change-request columns not updatable; per-pass unique deciders and the instance lock; delegation
CHECK and exclusion; person-level (not role-level) exclusion; the freeze `true` CHECK and Cedar forbid; pair-memory
self-approval forbid; case decision CHECK and engine exclusion; RFI release CHECK; periodic pack, list activation,
attestation and freeze-release checks.

DR: DR-407 (the run 1 fixes).
