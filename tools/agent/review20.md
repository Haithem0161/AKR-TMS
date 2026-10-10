# Phase 20 review findings (lead, 2026-10-09)
1. HIGH retention: draft moves all 5 tables to R5 'age' claiming "R7 is the alert class"; ARCHITECTURE s.17 R7 = screening and
   sanctions records (phase 14 uses R7). Age from listing_decision_at would delete a freeze still in force after 7 years.
   Fix: freezes R7 closure released_at; refrain R7 closure closed_at; incidents R7 closure closed_at; ops_tasks and
   ops_verifications R7 owner (follow source record). DR-166 and reconciliation bullet rewritten; DATA-MODEL R6->R7 for ops_tasks.
2. HIGH attestation upload: AttestOpsTask cites "POST /evidence of phase 02", which does not exist (phase 02 has no business
   routes; uploads are per owner, e.g. POST /cases/{id}/evidence). Fix: add POST /ops-tasks/{id}/evidence (case.decide,
   EvidenceStore::stage, owner kind ops_task) as an added route.
3. MEDIUM StartSpec: draft uses StartSpec { definition, resource_kind, resource_id, maker, params }; phase 04 is
   StartSpec { definition_key, resource: (kind, id), purpose, initiator, maker } (no params). Fix: evidence_id frozen into
   workflow_instances.context by OpsTaskWorkflowSubject::start_context (attr ops.evidence_id); record_verification reads
   inst.context. Same check for the freeze_release start.
4. MEDIUM config failure inside the sanctions decision: ops.freeze_task_hours outside 1..8 -> FreezeService::start returns
   E_INTERNAL inside phase 14 decision step 8 transaction, rolling back a true-match decision. Fix: out-of-bounds value
   falls back to 8 with audit ops.config_fallback; the bound is also stated for the phase 04 `setting` change target.
5. LOW curl h: m@ called "the requester" of an attestation a2@ requested; ops_attestation holders stated as "A2 or M" in 3.3 but
   assignee role A2 in 4.5. Fix: assignee A2 (bank); curl h: a2@ self-approve 403, a2b@ approves.
6. MEDIUM phase 18 behaviour copied wrong: second_review_not_shipped is 409 E_NOT_ALLOWED_IN_STATE (DR-143), draft says 422
   E_VALIDATION; refusal sub-code is role_not_permitted, draft invents role_cannot_decide.
7. LOW create_ops_task resource kinds: phase 15 instance resource kind is `exit_decision`, not `exit`; `case` is not a user.
8. HIGH lift tasks never verified: check step 1 is a no-op when the owner is released/closed, so the release task created by
   release_freeze / refrain close is never checked; and V1 means "hold present", wrong for a lift. Fix: action-aware check:
   a `release` task reaches V1 when the copy (current to the effective date) shows no open V1-type hold carrying the
   original task's cbs_reference; no V2, no debit watch; release tasks are checked whatever the owner state, until V1.
9. LOW V1 hold types omit `restriction` (the neutral text says "apply restriction"); needing_check(limit 500) unordered can
   starve tasks. Fix: add restriction; order by the oldest last check.
Severity: 0 critical / 3 high (1, 2, 8) / 3 medium (3, 4, 6) / 3 low (5, 7, 9) = 9. Alignment: API.md /exits row marked superseded.
