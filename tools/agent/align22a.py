import re
p='docs/plan/research.md'
s=open(p).read()
rows=[
("DR-201","A branch answer is a phase 19 claim recorded through `RfiService::record_answers_in_tx` with source kind `branch_officer`, `corroboration = 'none'` and no `evidence_object_id` (the staged document ids sit in the sealed value; the desk raises the corroboration); branch documents are owned by the RFI (owner kind `rfi_input`); DATA-MODEL `branch_inputs` dropped","Phase 19 already holds answers as claims; a claim corroborated by its own maker would pass the M-45 closure gate (lead review)"),
("DR-202","Confidential reports and observations are written by `InternalReportService::receive` with channel `branch_window`; `casework.observations` holds only the structure; DATA-MODEL `confidential_reports` dropped","The reporter identity (M only, M-68), the acknowledgment and its timer exist once, in phase 19"),
("DR-203","The monthly liaison report is signed with step-up `signing`; the first signature writes one internal report with channel `liaison_monthly` (sealed body: report id and period); a return sends it to `draft` and a second signature reuses the entry","Phase 21's `liaison` pack counts those entries; a second entry would count a month twice"),
("DR-204","Only `awaiting_input`, `received` and `closed` reach the branch, through two `match` maps without a wildcard arm; an unmapped state is not listed","M-66 and research 28 s.13.2; a new state fails the build until someone decides"),
("DR-205","A caller's branch is the `admin.branches` row where they are liaison or alternate (one branch per user, phase 03); an item's branch is the branch of the liaison it is addressed to (`rfi.recipient_ref` -> `canon.staff.branch_id` -> `canon.branch.org_branch_id`); the session carries no branch","A party with accounts at two branches must not show the item to the second branch (lead review)"),
("DR-206","Branch routes answer 404 to a caller without the `branch.window` scope and to an item not theirs; a deliberate exception to API.md s.6","A 403 tells a manager that a branch window exists"),
("DR-207","One notification dispatcher computes recipients at dispatch from the source row and keeps only those who could read the source; replays write nothing (`dedupe_key`); phase 00's outbox registry becomes a list of handlers per topic","A stored recipient list goes stale; `case.event` and `case.decision` already have handlers (lead review)"),
("DR-208","Fifteen closed template codes with four parameters (`count`, `due_date`, `branch_code`, `alarm_kind`); a suspicion-tier source is always `task.waiting`","ARCHITECTURE s.15 and DESIGN-SYSTEM s.11, applied where the notice is built"),
("DR-209","E-mail goes only through the bank's SMTP relay, generic text and link, no attachment; `notify.email_enabled` false by default; per-user default is a digest; relay credential in OpenBao, worker only; closed error codes","TS-21: no SMS, push or messaging app"),
("DR-210","Template text changes by phase 04 change request (`cr_material`, CHK and M) with a placeholder validator, the scanner and append-only versions; a notice keeps its version","What staff are told is a bank decision"),
("DR-211","Monthly due day `branch.liaison_due_day` (10, floor `{\"max\": 10}`), moved to the next working day when closed; timer `liaison_due` fires once the next morning; overdue alarm `liaison_report_overdue` (M-53); late signatures accepted","The simpler reading of \"by the 10th\" (Open doubt 11)"),
("DR-212","S-23's hybrid alert: an observation and an existing scenario alert of the same party within `branch.hybrid_window_days`, emitted through `AlertIntakeService::emit` with source kind `observation`; off until `branch.hybrid_scenario_code` is set; stops when the observation names no party; no S-24 scenario here","No transaction logic of its own, so it cannot drift from the scenario engine"),
("DR-213","Observation groups are a closed list of eight codes (research 01 s.A items 1-8, the seven criteria required by group 8); the acknowledgment is the code `OBS_ACK_V1` with glossary wording","The wording can change without a schema change"),
("DR-214","The real-time signal is `notification.new {notification_id}` on phase 17's `CountsHub` (`GET /events/counts`); no second `SseHub`","One place decides a user's channel; the event carries no text"),
("DR-215","The branch window reads no transaction and no CBS table; B6 is the list of documents the branch is asked for","Branch management has no part in the process (CBI)"),
("DR-216","Branch counts use phase 21's small-number suppression and the same predicate as the list they summarise; confidential reports are counted on R5 for M only, never on a branch surface","A count must not reveal an item or that the liaison reported (lead review)"),
("DR-217","Retention: observations R6 owner, monthly reports R6 age, notifications R12 (swept after `notify.retention_days`, floor 30), templates, versions and preferences R9","Notices are operations records, not evidence"),
]
anchor=re.search(r'^\| DR-200 \|.*\n',s,re.M)
assert anchor
add=''.join(f"| {d} | 2026-10-09 | {a} | {b} |\n" for d,a,b in rows)
assert '| DR-201 |' not in s
s=s[:anchor.end()]+add+s[anchor.end():]
open(p,'w').write(s)
print('ok',len(rows))
