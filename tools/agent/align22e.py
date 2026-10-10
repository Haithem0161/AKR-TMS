p='docs/plan/status.md'
s=open(p).read()
R=[
("phases 09 to 21 and 23 (batches B1 to B6; phases 19, 20, 21 and 23 are B6) reviewed 2026-10-09.",
 "phases 09 to 23 (batches B1 to B7; phases 19, 20, 21 and 23 are B6, phase 22 opens B7) reviewed 2026-10-09."),
("| 22 | Branch Window & Notifications | Planned | | | 7 | | | 4 |",
 "| 22 | Branch Window & Notifications | Spec ready | | | 6 (`branch_inputs` and `confidential_reports` dropped for phase 19 claims and internal reports, DR-201, DR-202) | `casework.alarms` kind and subject CHECKs; phase 00 outbox registry becomes a handler list per topic; phase 19 `record_answers_in_tx` and owner kind `rfi_input`; phase 03 one branch per liaison, `UserRoleQuery`; phase 04 `notify` effect and `notification_template` change target (gap pass 1) | 17 | 13 (BranchWindowService, BranchStatusMapper, ObservationService, HybridObservationJob, LiaisonReportService, NotificationDispatcher, NotificationService, TemplateService, NotifyEffect, NotificationTemplateTarget, SmtpRelaySender, RfiBranchRepository, UserRoleQuery) |"),
("| Domain tables (specified) | 0 | 212 |","| Domain tables (specified) | 0 | 218 |"),
("| Routes | 0 | 431 |","| Routes | 0 | 448 |"),
("| Phases specified | 0 | 23 |","| Phases specified | 0 | 24 |"),
("| Screens | 0 | 42 (X1, X3 task half,","| Screens | 0 | 49 (X1, X3,"),
("R1, R2, R3, G7) |","R1, R2, R3, G7, B1, B2, B3, B4, B5, B6, R5) |"),
("| 0 | 485 (some partial; `python3 tools/idcount.py 23`:","| 0 | 493 (some partial; `python3 tools/idcount.py 23`, phases 00-23:"),
]
for a,b in R:
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
a=[l for l in s.split('\n') if l.startswith('| `evidence_reconcile` | 23 |')][0]+'\n'
jobs="""| `branch_monthly_draft` | 22 | `schedule.branch_monthly_draft` (first day of the month, 00:30): drafts each branch's report for the month before |
| `timer_liaison_due` | 22 | Timer kind `liaison_due` at `due_on` plus one day: alarm `liaison_report_overdue` while the report is a draft |
| `observation_hybrid` | 22 | Enqueued by the observation transaction: S-23 hybrid alert check (off until `branch.hybrid_scenario_code` is set) |
| `notification_email` | 22 | Enqueued by the dispatcher for an immediate recipient when `notify.email_enabled`: one generic e-mail through the bank relay |
| `notification_digest` | 22 | `schedule.notification_digest` (hourly): one digest e-mail per recipient |
| `notification_sweep` | 22 | `schedule.notification_sweep` (daily 02:10): deletes notices older than `notify.retention_days` |
"""
assert '`branch_monthly_draft` | 22' not in s
s=s.replace(a,a+jobs)
open(p,'w').write(s)
print('ok')
