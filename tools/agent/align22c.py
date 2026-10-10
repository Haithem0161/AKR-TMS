p='docs/DATA-MODEL.md'
s=open(p).read()
R=[
("""| observations | id, branch_id, reporter_id, party_ref jsonb, indicator_groups text[], notes (enc), acknowledged_at, alert_id null | R6 |
| confidential_reports | id, branch_id, reporter_id, body (enc), received_at, case_id null | R6 |
| branch_inputs | id, item_ref, branch_id, explanation_claim (enc), documents_on_file bool, visit_result, submitted_by, submitted_at | R6 |
| liaison_monthly_reports | id, branch_id, period, draft jsonb, signed_by null, signed_at null, stepup_id null, reviewed_by null | R6 |
""",
"""| observations | id, branch_id, reporter_id, party_id null (no customer named), indicator_groups text[] (eight codes), escalation_criteria text[], companion_id_enc null, notes_enc, reporter_ack_code, reporter_ack_at, internal_report_id unique, hybrid_alert_id null, hybrid_checked_at null, created_at, updated_at, version | R6 (owner) |
| liaison_monthly_reports | id, branch_id, period, period_end, due_on, status (draft, signed, reviewed), draft jsonb (neutral counts), suppressed_cells, drafted_at, signed_by, signed_at, stepup_id, internal_report_id unique, returned_by, returned_at, return_comment_enc, reviewed_by, reviewed_at, review_comment_enc, timer_id, created_at, updated_at, version; unique (branch_id, period) | R6 (age) |

Phase 22 (DR-201, DR-202): the branch's answers are phase 19 `rfi_answers` claims (source kind `branch_officer`, never
self-corroborated) and confidential reports and observations are phase 19 `internal_reports` with channel
`branch_window`, so the earlier `branch_inputs` and `confidential_reports` tables are not created. Phase 22 also widens
the `casework.alarms` CHECKs (kind `liaison_report_overdue`, subject kind `liaison_report`).
"""),
("""| notifications | id, user_id, template_code, params jsonb (never names or reasons), read_at null, created_at | R12 |
| notification_templates | code pk, channel (in_app, email), body jsonb, version | R9 |
| user_notification_prefs | user_id, channel, mode (immediate, digest) | R9 |
""",
"""| notifications | id, user_id, template_code, template_version, params jsonb (count, due_date, branch_code, alarm_kind only; at most 512 bytes), dedupe_key unique, source_topic, read_at null, email_state (none, digest, queued, sent, failed), email_attempts, email_sent_at, email_error_code (closed list), created_at | R12 (age; swept after `notify.retention_days`) |
| notification_templates | code pk, body jsonb (ar, en), params text[], version, updated_at | R9 (retirement) |
| notification_template_versions | id, code, version, body, params, change_request_id, approved_by, created_at; unique (code, version); append-only | R9 (owner) |
| user_notification_prefs | user_id pk, email_mode (immediate, digest; default digest), updated_at, version | R9 (owner) |

Phase 22: one template set serves the in-app notice and the e-mail (no channel column); text changes by change request
with an append-only version, and a notice keeps the version it was written with (DR-208, DR-210).
"""),
("""| 20 | branch window; notifications | 22 |""",
"""| 20 | `casework`: the `casework.alarms` kind and subject CHECKs widened, observations, liaison monthly reports (0073); `admin`: notification templates (seeded), template versions, notifications, user notification preferences (0074); guards, grants, settings, classification and retention (0075) | 22 |"""),
]
for a,b in R:
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
anchor="Phase 23 grants: "
i=s.index(anchor); j=s.index("\n",i)+1
assert 'Phase 22 grants' not in s
g=("Phase 22 grants: `akr_app` has SELECT and INSERT on casework.observations, casework.liaison_monthly_reports, "
"admin.notifications, admin.notification_templates, admin.notification_template_versions and "
"admin.user_notification_prefs, with UPDATE only on `observations (hybrid_alert_id, hybrid_checked_at, updated_at, version)`, "
"the draft, sign, return, review and timer columns of `liaison_monthly_reports`, `notifications (read_at, email_state, "
"email_attempts, email_sent_at, email_error_code)`, `notification_templates (body, params, version, updated_at)` and "
"`user_notification_prefs (email_mode, updated_at, version)`; DELETE on `notifications` only, for the R12 sweep (the guard "
"refuses a notice younger than `notify.retention_days`); observations and liaison reports are never deleted (guards) and "
"template versions are append-only; no grant to `akr_readonly`, `akr_ingest`, `akr_audit_writer` or `akr_monitor` "
"(phase-22 section 1.3).\n")
s=s[:j]+g+s[j:]
open(p,'w').write(s)
print('ok')
