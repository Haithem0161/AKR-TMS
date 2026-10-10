# Re-check, phase 22 (Haiku draft for lead review)

### R-22-1  MEDIUM   (fix F24)
- Problem: The fix says an approver of an open deciding task learns of it through `task.waiting`, but no step of the plan sends that notice to holders when a deciding task opens. Phase 04 sends `task.overdue` to holders from its reminder and `task.waiting` only to the escalation target, and the `notify` effect addresses only the initiator or the subject team.
- Proof: docs/plan/phase-22.md:1000 "An approver learns of an open deciding task through `task.waiting`." and docs/plan/phase-04.md:1712 "`remind` writes, in its transaction, the outbox row `notify.request {task_id, template_code: "task.overdue", to:" and docs/plan/phase-04.md:1713 "`escalate` writes `notify.request {task_id, template_code: "task.waiting", to: "escalation_target"}`."
- Smallest fix: In docs/plan/phase-22.md line 1000, replace "An approver learns of an open deciding task through `task.waiting`." with the path the plan actually has (the inbox, plus the `task.overdue` reminder of phase 04), or add a named step that sends `task.waiting` to the holders when a deciding task opens.

### R-22-2  MEDIUM   (fix F25)
- Problem: The new method returns `Vec<Observation>`, but `Observation` has no notes field and never selects `notes_enc`, while the observation key must carry `notes`, decrypted for M only (section 7.2, and `ObservationView` of section 2). The fix neither returns the M-only view nor names the decryption step or the caller that merges the rows into `GET /internal-reports`.
- Proof: docs/plan/phase-22.md:1003 "`by_internal_report(conn, internal_report_ids: &[Uuid]) -> Vec<Observation>`" and docs/plan/phase-22.md:386 "`Observation { id, branch_id, reporter_id, party_id, indicator_groups: Vec<IndicatorGroup>, escalation_criteria: Vec<EscalationCriterion>, reporter_ack_at, internal_report_id, hybrid_alert_id: Option<Uuid>, hybrid_checked_at: Option<DateTime<Utc>>, created_at, version }`"
- Smallest fix: In docs/plan/phase-22.md line 1003, change "-> Vec<Observation>" to "-> Vec<ObservationView>" and add that the method selects `notes_enc` and decrypts it through the phase 02 personal seal (AAD = the observation id).

## Summary
- Fixes checked: 25 (F1 to F25).
- Findings: HIGH 0, MEDIUM 2, LOW 0.
- CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok, F8 ok, F9 ok, F10 ok, F11 ok, F12 ok, F13 ok, F14 ok, F15 ok, F16 ok, F17 ok, F18 ok, F19 ok, F20 ok, F21 ok, F22 ok, F23 ok, F24 R-22-1, F25 R-22-2
