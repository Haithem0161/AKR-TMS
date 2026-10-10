p='docs/plan/roadmap.md'
s=open(p).read()
R=[
("| 22 | Branch Window & Notifications | Branch inputs, observations, confidential reports, monthly liaison reports with the liaison review routes and screen R5 (moved from phase 21); notifications, templates and preferences (generic text only); the branch bundle (B1-B6) and X3 | L | P1 | 19, 20 | Planned |",
 "| 22 | Branch Window & Notifications | Branch window on phase 19 (answers are RFI claims, confidential reports and observations are internal reports with channel `branch_window`), items listed to the addressed liaison's branch only, closed neutral status map, 404 without scope; observation log (eight groups, optional customer) and the S-23 hybrid alert; monthly liaison report drafted, signed with step-up, reviewed on R5, overdue alarm; one notification dispatcher on the outbox (handler list per topic), fifteen closed templates by change request, e-mail relay off by default; the branch bundle (B1-B6), R5 and the notification half of X3 | L | P1 | 17, 19, 20, 21 | Spec ready |"),
("| 22 | `observations`, `confidential_reports`, `branch_inputs`, `liaison_monthly_reports`, `notifications`, `notification_templates`, `user_notification_prefs` |",
 "| 22 | `casework.observations`, `casework.liaison_monthly_reports`, `admin.notifications`, `admin.notification_templates`, `admin.notification_template_versions`, `admin.user_notification_prefs` (`branch_inputs` and `confidential_reports` dropped: phase 19 claims and internal reports, DR-201, DR-202; `casework.alarms` CHECKs widened) |"),
("| 22 | `BranchService`, `ObservationService`, `NotificationDispatcher`, `SseHub` |",
 "| 22 | `BranchWindowService`, `BranchStatusMapper`, `ObservationService`, `HybridObservationJob`, `LiaisonReportService`, `NotificationDispatcher`, `NotificationService`, `TemplateService`, `NotifyEffect`, `NotificationTemplateTarget`, `SmtpRelaySender` (port `MailSender`), `RfiBranchRepository`, port `UserRoleQuery` (phase 03); no `SseHub` (phase 17's `CountsHub`, DR-214) |"),
("| 22 | next numbers (from 0073) | 20 |",
 "| 22 | `0073_casework_branch_window`, `0074_admin_notifications`, `0075_branch_guards_grants` | 20 |"),
("| 24 | next numbers | 22 |","| 24 | next numbers (from 0076) | 22 |"),
]
for a,b in R:
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
row23=[l for l in s.split('\n') if l.startswith('| 0 (B6 review: phase 23)')][0]
new=("| 0 (B7 review: phase 22) | 2026-10-09 | 7 lead corrections to the Haiku draft | 0 | 2 | 2 | 3 | HIGH: a branch item was listed "
"to every branch where the party holds an account, through a column that does not exist (`canon.account.org_branch_id`), so "
"another branch's liaison could see and answer it (now the branch of the addressed liaison, `rfi.recipient_ref` -> "
"`canon.staff.branch_id` -> `canon.branch.org_branch_id`); phase 00's outbox routes a topic to one handler and `case.event` and "
"`case.decision` already have one (the registry becomes a handler list per topic). MEDIUM: branch document answers were "
"corroborated by their own maker against phase 19's claim CHECKs, passing the M-45 closure gate (now `corroboration = 'none'`, "
"document ids in the sealed value, the desk raises it; `record_answers_in_tx`); `observations.party_id` NOT NULL blocked walk-in "
"and third-party observations (nullable). LOW: confidential reports counted on branch surfaces (R5 only); due-day floor "
"`{\"max\": 10}`; names (`BranchWindowService`, `case.assigned`, section 3.8.3, Cedar fence) |")
assert 'B7 review: phase 22' not in s
s=s.replace(row23+'\n',row23+'\n'+new+'\n')
open(p,'w').write(s)
print('ok')
