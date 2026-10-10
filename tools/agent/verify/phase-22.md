# Verification pass, phase 22 (Haiku draft for lead review)

Scope: docs/plan/phase-22.md (1071 lines). Read first: .claude/rules/planning.md (Phase File Template, Verification Pass), tools/agent/b3-conventions.md. No agents launched. Cross-phase names checked against docs/plan/phase-00.md to phase-25.md as needed, docs/API.md (problem codes and scopes) and docs/plan/research.md (DR entries).

Sample, fixed before checking (header "Implemented here" list has 12 IDs in this order: M-43, M-44, M-45, L-47, UX-02, UX-66, UX-67, S-22, S-23, M-53, M-66, M-68; header at phase-22.md:12):
- Header items, positions 1, 3, 8, 10, 12: V-22-01 M-43; V-22-02 M-45; V-22-03 S-22; V-22-04 M-53; V-22-05 M-68.
- Gap pass blocks not headed "Adversarial pass" (7.1, 7.2, 7.4, 7.5; the header at phase-22.md:918 and :945 is gap pass): V-22-06 7.1 break-glass notice codes; V-22-07 7.2 observation fields in the register; V-22-08 7.4 flag route and hybrid scenario; V-22-09 7.5 visit.recorded topic. Block 7.3 (no drift topic, no deliverable) is not sampled.
- Adversarial blocks: run 5 (header phase-22.md:955) holds 7.6 and 7.7; run 6 (header phase-22.md:978) holds 7.8. V-22-10 is 7.6, V-22-11 is 7.7, V-22-12 is 7.8. All three adversarial blocks are sampled.

### V-22-01 M-43 branch RFI list and escalation (source: header Implemented here, position 1; result: PASS)
- T1 SQL: PASS - casework.rfis has kind, recipient_kind, recipient_ref, party_id, tier, status, released_at, closed_at (phase-19.md:92-120); casework.cases.tier (phase-17.md:300); setting branch.item_closed_days seeded (phase-22.md:315). Who writes closed_at is checked under V-22-10-a.
- T2 Route: PASS - GET /api/v1/branch/items, scope branch.window, Cedar branch.item_list (phase-22.md:424); GET /api/v1/branch/items/{id}, branch.item_read (phase-22.md:425). Scope admits L only (docs/API.md:92). Cedar branch.members permit lists branch.item_list (phase-22.md:663-665).
- T3 Repository and types: PASS - RfiBranchRepository::released_items(conn, branch_id, cursor) and item(conn, rfi_id, branch_id) (phase-22.md:400); BranchWindowService::items(ctx) -> Page<BranchItemView> (phase-22.md:471); BranchItemView fields (phase-22.md:408); BranchStatusMapper table (phase-22.md:460-469).
- T4 Logic: PASS - items steps 1-4 (phase-22.md:472-475) and item steps 1-2 (phase-22.md:478-479): 404 for hidden items, identical body, cursor rule. Read only, so no lock is needed.
- T5 Audit: N/A - read routes, no mutation.
- T6 Cross-references: PASS - casework.effective_tier(p_party UUID) (phase-17.md:530); recipient is the liaison staff row (phase-19.md:1068); canon.staff.branch_id (phase-10.md:434); canon.branch.org_branch_id (phase-10.md:384); admin.branches liaison_user_id, alternate_user_id, archived_at (phase-03.md:193-195). MLRO desk escalation: rfi.overdue goes to "the MLRO desk, never the branch manager" (phase-19.md:1644), mapped to desk notices for RFI and M (phase-22.md:603). Liaison role code is an Open doubt in both phases (phase-22.md:1029; phase-19.md:1068), so not a gap.
- T7 Consistency: PASS - 7.6 replaces the two tier clauses of step 2 and the 3.12 row (phase-22.md:958-959, 968-970), so the block wins. No second rule for the list.
- Gaps: none.

### V-22-02 M-45 branch answers stored as claims (source: header Implemented here, position 3; result: GAPS 4)
- T1 SQL: GAP (see V-22-02-a, V-22-02-b). Other columns PASS: casework.rfi_answers has source_kind with value branch_officer, answered_at, expires_at, claim_state, corroboration, corroborated_by, evidence_object_id, and the CHECKs that a branch claim satisfies (corroboration none, no evidence id) (phase-19.md:158-175).
- T2 Route: PASS - POST /branch/items/{id}/evidence (phase-22.md:426, branch.evidence_stage) and POST /branch/items/{id}/input (phase-22.md:427, branch.item_answer); Cedar permits at phase-22.md:672-675.
- T3 Repository and types: GAP (see V-22-02-d). BranchWindowService::answer(ctx, rfi_id, If-Match, Vec<AnswerInput>) and stage_evidence(ctx, rfi_id, upload) (phase-22.md:483, 490); BranchItemDetail and StagedEvidence (phase-22.md:408).
- T4 Logic: PASS - answer steps 1-8 with refusal codes (phase-22.md:491-498); lock order named (phase-22.md:491; 3.14 at phase-22.md:767-771).
- T5 Audit: PASS - branch.evidence_staged (phase-22.md:488) and branch.item_answered (phase-22.md:497) are both in 3.11 (phase-22.md:712-713); phase 19 writes rfi.answer_recorded (phase-19.md:1123).
- T6 Cross-references: GAP (see V-22-02-c). PASS elsewhere: RfiService::record_answers_in_tx (phase-19.md:1844); casework.rfi_claim_expiry_days (phase-19.md:815); PolicyPackService::scan scopes branch_form and rfi (phase-09.md:782); EvidenceStore::stage and NewEvidence (phase-02.md:611-612, 839); owner kind rfi_input (phase-02.md:1183); admin.evidence_objects.created_by (phase-02.md:377).
- T7 Consistency: PASS - step 5 (phase-22.md:495) agrees with phase 19 section 7.1, where the corroborates path accepts rfi_input objects (phase-19.md:1844-1846) and with 4.5 item 2 (phase-22.md:818).
- Gaps:
  - `GAP V-22-02-a | LOW | T1 | step 5 needs an object with status "scanned"; phase 02 has no such status. The scanned state is av_status = 'clean' (phase-02.md:350, 368, 381). "scanned_at" is a column, not a status. | 3.2 step 5 (phase-22.md:495) | greps: "status.*scanned" in phase-02.md matches only scanned_at column lines (381, 445, 451)`
  - `GAP V-22-02-b | LOW | T1 | step 3 passes lang: none. ContentLang and the column CHECK hold ar, en, ku, mixed, und (phase-02.md:367, 607). No "none" value. | 3.2 step 3 (phase-22.md:486) | greps: "lang.*none" and "'none'" in phase-02.md: no match`
  - `GAP V-22-02-c | MEDIUM | T6 | a branch correction is said to go "through corroborates, phase 19 section 3.3 step 2" (phase-22.md:493). Phase 19 uses supersedes for a correction (phase-19.md:983 and step 2 at :1119). corroborates only raises corroboration and writes no new claim (phase-19.md:1120). A branch correction as written would write no answer. | 3.2 step 3 (phase-22.md:493) and AnswerInput (phase-22.md:395) | greps: "supersedes" in phase-22.md: no match; "question_id" in phase-22.md: no match`
  - `GAP V-22-02-d | LOW | T3 | phase 22 AnswerInput (question_key, evidence_ids, corroborates; value up to 500) is not phase 19 AnswerInput (question_id, evidence_object_id, supersedes, source_ref; value up to 2000) (phase-19.md:983). The text says it is "the phase 19 AnswerInput" (phase-22.md:395) but gives no mapping for the call at step 6 (phase-22.md:496). | section 2 AnswerInput (phase-22.md:395) and 3.2 step 6 | greps: "question_id" in phase-22.md: no match`

### V-22-03 S-22 observation log (source: header Implemented here, position 8; result: GAPS 2)
- T1 SQL: PASS - casework.observations columns and CHECKs (phase-22.md:55-81); the eight codes match the IndicatorGroup enum (phase-22.md:380); internal_report_id is a unique FK to casework.internal_reports (phase-22.md:66; phase-19.md:187); channel 'branch_window' exists (phase-19.md:193).
- T2 Route: PASS - POST /api/v1/branch/observations, scope branch.window, Cedar branch.observation_create (phase-22.md:428, 663-664).
- T3 Repository and types: PASS with note - ObservationService::create(ctx, ObservationCreate) and ObservationCreate (phase-22.md:393, 502); ObservationReceipt (phase-22.md:408); HybridObservationJob::run (phase-22.md:513). NewObservation fields are not listed, but section 1.1 and steps 2, 4, 5 and 6(b) give them (phase-22.md:401, 508): inferable, not counted. ReceiveCommand: see V-22-03-a.
- T4 Logic: PASS - create steps 1-7: form rules, party check, scan, seal, one transaction (phase-22.md:503-509); refusal codes (phase-22.md:444-446); hybrid job steps 1-7 with lock order (phase-22.md:514-520).
- T5 Audit: PASS - branch.observation_recorded (phase-22.md:508, step 6d) and branch.observation_hybrid_alert (phase-22.md:518) are both in 3.11 (phase-22.md:714-715); phase 19 writes internal_report.received (phase-19.md:1138).
- T6 Cross-references: PASS except as noted in V-22-03-b. branch_of_party_account needs canon.account.branch_id, canon.branch.org_branch_id and canon.account_party_role (phase-10.md:384, 630-639, 668); InternalReportService::receive and intake_clock (phase-19.md:1135-1138); AlertIntakeService and EmitOutcome::Duplicate (phase-17.md:3, 754); DetectionPayload fields and source_kind (phase-16.md:1206-1217, 1960); family observation (phase-17.md:94); alert.alerts source_kind observation and linked_party_ids (phase-17.md:109, 136); observational scenario (phase-06.md:86); text_enc sealing pattern (phase-18.md:115).
- T7 Consistency: PASS - the branch never reads the observation back (phase-22.md:511) and 3.12 agrees (phase-22.md:745); steps 6 of 3.3 and 3.4 agree (phase-22.md:508, 528). The reader rule for the notes is checked under V-22-07-a.
- Gaps:
  - `GAP V-22-03-a | LOW | T3 | ReceiveCommand is named but not defined in phase 19 or phase 22. Phase 19 section 3.4 step 1 lists reporter_kind, reporter_id, channel, body and received_at only (phase-19.md:1136), but the call sites pass branch_id and subject_party_id too (phase-22.md:508, 528, 556). Both columns exist (phase-19.md:191-192). | phase 19 section 3.4 step 1 and phase 22 section 2 | greps: "ReceiveCommand" in docs/plan: phase-22.md only`
  - `GAP V-22-03-b | LOW | T6 | the groups are said to be read "through the register detail" (phase-22.md:511; Open doubts 9 at phase-22.md:1037). Phase 19 has no register detail route: only GET /internal-reports (a list) and POST and acknowledge (phase-19.md:1039-1041). The groups reach M through list rows (7.2, phase-22.md:930-933). | 3.3 (phase-22.md:511) and Open doubts 9 | greps: "internal-reports/{id}" GET in phase-19.md: no match; "register detail" in phase-19.md: no match`

### V-22-04 M-53 liaison overdue alarm and supervisor notice (source: header Implemented here, position 10; result: GAPS 2)
- T1 SQL: PASS - the casework.alarms kind and subject_kind CHECKs are widened by 0073 with liaison_report_overdue and liaison_report (phase-22.md:42-50) over the base lists (phase-19.md:300-303); stage may be NULL (phase-19.md:302, CHECK stage IS NULL OR 100, 150); admin.timers (phase-00.md:138); liaison_monthly_reports.timer_id FK (phase-22.md:106).
- T2 Route: N/A - the timer handler and the alarm are not routes. M reads alarms through GET /api/v1/alarms (phase-19.md:1049).
- T3 Repository and types: PASS - AlarmService::raise(tx, NewAlarm) (phase-19.md:1299; AlarmRepository.insert_live at phase-19.md:1004); TimerService::schedule(tx, NewTimer {...}) (phase-00.md:561); LiaisonReportRepository::get (phase-22.md:402) for the re-read (phase-22.md:563). NewAlarm fields are given at the call (phase-22.md:564).
- T4 Logic: GAP (see V-22-04-b). Timer steps 1-2 are numbered (phase-22.md:563-564); the report lock is named in the lock order (phase-22.md:769).
- T5 Audit: PASS - alarm.raised is written at insert (phase-19.md:1312), listed in phase 19 audit table (phase-19.md:1352) and named in phase 22 section 3.11 (phase-22.md:729). The alarm is the mutation and it has its event.
- T6 Cross-references: GAP (see V-22-04-a). PASS elsewhere: alarm.raised outbox (phase-19.md:1312, 1676); timer job timer_liaison_due by the timer kind rule (phase-00.md:859); BusinessClock::add with WorkingDays (phase-09.md:434, 458-459); bank.timezone (phase-00.md:238); report.small_number_threshold (phase-21.md:518).
- T7 Consistency: PASS - the timer no-op on a signed report (phase-22.md:563) agrees with "a late signature is counted" (phase-22.md:563) and DR-211 (phase-22.md:1011). The 4.2 timer row (phase-22.md:793) agrees with 3.5 step 1e (phase-22.md:541).
- Gaps:
  - `GAP V-22-04-a | MEDIUM | T6 | phase 19 section 3.10 table has six kinds and no liaison_report_overdue row (phase-19.md:1301-1308). Phase 19 section 7.3 says each owner adds its rows (phase-19.md:1858-1862), and phase 22 adds none (section 4.5 has no such item). The supervisor (M) is stated only inline in 3.6 step 2 (phase-22.md:564). | phase 19 section 3.10 table, or phase 22 section 4.5 | greps: "liaison_report_overdue" in phase-19.md: only lines 1861-1862; in phase-22.md section 4.5: no match`
  - `GAP V-22-04-b | MEDIUM | T4 | 3.7 step 3 says a return "not re-signed by the due day raises the overdue alarm" (phase-22.md:575). But the timer fires once, at 00:00 after due_on, and a fire on a signed report is a no-op that is not re-armed (phase-22.md:563, 795). A return after that fire arms nothing, so no path raises the alarm, and the text does not say what happens. | 3.6 timer and 3.7 step 3 (phase-22.md:562-575) | greps: "re-arm\|reschedul" in phase-22.md: no match for the return path; section 4.2 (phase-22.md:795) says no cancel is needed, not that a return re-arms`

### V-22-05 M-68 generic notifications and the dispatcher (source: header Implemented here, position 12; result: GAPS 4)
- T1 SQL: PASS - admin.notifications with the email_state and sent CHECKs (phase-22.md:159-176); admin.notification_templates CHECKs (phase-22.md:136-143); admin.notification_template_versions (phase-22.md:146-156); admin.user_notification_prefs (phase-22.md:183-188); settings notify.* (phase-22.md:317-334); admin.change_requests (phase-04.md:121); admin.users email and locale (phase-01.md:58, 65).
- T2 Route: PASS - the six /me/notifications and preference rows and GET /api/v1/admin/notification-templates, scopes self and admin.system (phase-22.md:415-420; docs/API.md:84, 104); Cedar actions at phase-22.md:634 and 668-701.
- T3 Repository and types: GAP (see V-22-05-a). PASS elsewhere: NotificationDispatcher::handle(tx, outbox_row) (phase-22.md:583); NoticeRepository methods (phase-22.md:403); insert_if_new fields in step 7 (phase-22.md:590); OutgoingMail and MailSender (phase-22.md:406, 735); NotifyEffect params (phase-22.md:821). Template version fields are inferable from 3.8 Template edits (phase-22.md:581), so not counted.
- T4 Logic: GAP (see V-22-05-b, V-22-05-c). Dispatcher steps 1-10 with recipients, visibility, counts and dedupe (phase-22.md:584-593); outbox lock in 3.14 item 6 (phase-22.md:772); relay error mapping (phase-22.md:736).
- T5 Audit: PASS - notification.dispatched (phase-22.md:592, 721); notification.read (phase-22.md:621, 722); notification.prefs_changed (phase-22.md:622, 723); notification.email_sent and email_failed (phase-22.md:724-725, 734); notification.swept (phase-22.md:726, 785); notification_template.changed (phase-22.md:581, 727).
- T6 Cross-references: GAP (see V-22-05-d). PASS elsewhere: case.assigned (phase-17.md:1169, 1365); case.work_item_overdue (phase-18.md:937); case.decision outbox (phase-18.md:1112); case.rfi_released payload (phase-19.md:1841-1843); rfi.overdue and rfi.reminder (phase-19.md:1644, 1673); internal_report.overdue (phase-19.md:1139, 1673); authority_request.overdue (phase-19.md:1153, 1673); watch.triggered and watch.expired (phase-19.md:1674); alarm.raised (phase-19.md:1312, 1676); credential_reset.requested (phase-03.md:1339-1343); notify.request reserved (phase-00.md:840-841); Holders::eligible (phase-04.md:703); case_acl revoked_at (phase-18.md:96); policy scope notification (phase-09.md:782); template_has_figures (phase-09.md:926); CountsHub and GET /events/counts (phase-17.md:854, 1246); GrantRepository::effective_roles (phase-01.md:450).
- T7 Consistency: PASS with reference - the notify.request row (phase-22.md:599) conflicts with 7.1 break-glass codes, recorded as V-22-06-a. Otherwise DR-208 (phase-22.md:1005), the closed-code list (phase-22.md:579) and the 3.12 notices row (phase-22.md:750) agree.
- Gaps:
  - `GAP V-22-05-a | MEDIUM | T3 | NotificationService::list renders the stored template version from admin.notification_template_versions (phase-22.md:619). TemplateRepository has only get, all and put_version (phase-22.md:404). No method reads one version. | section 2 TemplateRepository (phase-22.md:404) | greps: "get_version" and "version(conn" in phase-22.md: no match`
  - `GAP V-22-05-b | LOW | T4 | the template-change validations (both languages, placeholders, digits, scan, code exists) carry no sub-codes, except template_has_figures in test r (phase-22.md:581, 903). The 422 E_VALIDATION envelope is inherited from phase 04 (phase-04.md:1366). | 3.8 Template edits (phase-22.md:581) | greps: "forbidden_term" or "placeholder" sub-codes in phase-22.md 3.8 Template edits: no match`
  - `GAP V-22-05-c | LOW | T4 | desk.approval_waiting is seeded (phase-22.md:201) but no row of the mapping table produces it (phase-22.md:597-611). data.drift_open is also seeded with no producer, but 7.3 covers that (phase-22.md:935-938). | 3.8 mapping table (phase-22.md:597-611) | greps: "desk.approval_waiting" in phase-22.md: line 201 only`
  - `GAP V-22-05-d | LOW | T6 | serve.public.base_url is named as a setting read from phase 01 (phase-22.md:763) and used for the e-mail link (phase-22.md:734). Phase 01 mentions it only in prose as the public origin (phase-01.md:903). No settings row defines it. | 3.13 (phase-22.md:763) | greps: "serve.public.base_url" in docs/plan: phase-01.md:903 and phase-22.md only; no settings INSERT`

### V-22-06 7.1 break-glass notice codes (source: 7.1, gap pass block at phase-22.md:920-925; result: GAPS 2)
- T1 SQL: PASS - the two codes are seeded by migration 0082 (phase-25.md:548-549); the code regex and the params CHECK accept them (phase-22.md:137, 142).
- T2 Route: N/A - no route; the notices are dispatched by the dispatcher.
- T3 Repository and types: PASS - GrantRepository::active_holders(role) -> Vec<Uuid> (phase-01.md:451).
- T4 Logic: GAP (see V-22-06-b).
- T5 Audit: PASS - notification.dispatched names the topic and template code (phase-22.md:592).
- T6 Cross-references: PASS with note - phase 25 names the producers and codes (phase-25.md:775, 811-814) and says the rule is recorded in the phase 22 recipient table (phase-25.md:816). That table has no rows for these codes: see V-22-06-a.
- T7 Consistency: GAP (see V-22-06-a). The seventeen-code count (phase-22.md:921) correctly replaces the fifteen-code count (phase-22.md:383, 579).
- Gaps:
  - `GAP V-22-06-a | MEDIUM | T7 | the mapping row for notify.request allows only task.waiting or task.overdue, and the producer must name one of the two (phase-22.md:599). Phase 25 sends security.break_glass_waiting and security.break_glass_opened on notify.request (phase-25.md:775, 814). 7.1 adds the codes and a recipient rule but no mapping row, and the phase 22 recipient table that phase 25 relies on has no rows (phase-25.md:816). | 3.8 mapping table (phase-22.md:597-611) | greps: "break_glass" in phase-22.md: only line 922 (7.1); "active holders" in 3.8: only 7.1 lines 923-924`
  - `GAP V-22-06-b | MEDIUM | T4 | dispatcher steps 2 and 3 read a source row by the ids in the payload and compute a tier (phase-22.md:585-586). A break-glass notify.request carries only template_code and count (phase-25.md:775), so there is no source row and no tier. Step 6 would compute the count over a visible set (phase-22.md:589), not use the payload count. 7.1 gives recipients but not this path. | 3.8 steps 2 to 6 (phase-22.md:585-589) and 7.1 (phase-22.md:920-925) | greps: "count" as a notify.request payload key in phase-22.md: no match; a no-source-row case in 3.8: no match`

### V-22-07 7.2 observation fields in the internal-report register (source: 7.2, gap pass block at phase-22.md:929-933; result: GAPS 3)
- T1 SQL: PASS - casework.observations.internal_report_id is a unique FK to casework.internal_reports (phase-22.md:66; phase-19.md:187); notes_enc and companion_id_enc are sealed columns (phase-22.md:62-63).
- T2 Route: PASS - GET /api/v1/internal-reports, scope case.decide, unchanged as the block says (phase-22.md:933; phase-19.md:1039). Cedar readers are followup.a2.report and followup.m.report (phase-19.md:1436, 1472).
- T3 Repository and types: GAP (see V-22-07-c). The list read that adds the observation key has no named repository method or response type.
- T4 Logic: PASS - the rule is explicit: the observation key is for M only, notes are decrypted for M only, and other readers get no key (phase-22.md:930-932).
- T5 Audit: N/A - read only.
- T6 Cross-references: GAP (see V-22-07-b). DR-385 matches the block (research.md:396).
- T7 Consistency: GAP (see V-22-07-a). The register-detail wording of 3.3 is covered under V-22-03-b.
- Gaps:
  - `GAP V-22-07-a | MEDIUM | T7 | section 2 says ObservationView is "for M and the desk only" (phase-22.md:386). 7.2 and DR-385 say M only (phase-22.md:931-932; research.md:396). The block does not say it replaces section 2. | section 2 ObservationView (phase-22.md:386) | greps: "ObservationView" in phase-22.md: section 2 line 386 only; "M and the desk" in 7.2: no match`
  - `GAP V-22-07-b | LOW | T6 | the notes are decrypted "through the phase 02 sealing service" (phase-22.md:386). Phase 02 names no sealing service. Sealing appears only as a column property and in the AKRE1 format (phase-02.md:362-365, 710, 830). | section 2 (phase-22.md:386) | greps: "sealing service", "SealService" and "sealer" in phase-02.md: no match`
  - `GAP V-22-07-c | LOW | T3 | the list response gains an observation key with four sub-fields, but no response type or repository read is named. The only named type is ObservationView (phase-22.md:386), which is the M and desk view (see V-22-07-a). | 7.2 (phase-22.md:930-933) | greps: "observation {" as a type name in phase-22.md: 7.2 only (line 931)`

### V-22-08 7.4 flag route and the hybrid scenario (source: 7.4, gap pass block at phase-22.md:940-943; result: PASS)
- T1 SQL: PASS - branch.observations_live is seeded FALSE (phase-06.md:317); the observational column on the register scenario (phase-06.md:86); branch.hybrid_scenario_code ships empty (phase-22.md:313).
- T2 Route: PASS - PUT /admin/flags/{key}, scope admin.system (phase-03.md:518; docs/API.md:104). The 4.5 item 9 cross-reference (phase-22.md:825) names the same route.
- T3 Repository and types: N/A - no new type or method.
- T4 Logic: N/A - a rule of the register; section 3.3.1 step 2 agrees (phase-22.md:515).
- T5 Audit: N/A - no mutation in this phase (the flag write belongs to phase 03).
- T6 Cross-references: PASS - DetectionPayload.source_kind from phase 22 (phase-16.md:1960); DR-382 (research.md:393); family observation (phase-17.md:94).
- T7 Consistency: PASS - section 4.5 item 4 (phase-22.md:820) and Open doubts 2 (phase-22.md:1030) say the same thing. Step 4 of 3.3.1 uses family observation (phase-22.md:517), which the block permits.
- Gaps: none.

### V-22-09 7.5 visit.recorded topic (source: 7.5, gap pass block at phase-22.md:947-953; result: GAPS 1)
- T1 SQL: PASS - visit.recorded is a dotted topic name (phase-00.md:840-841 reservation rule); the answer format yes_no_unknown exists (phase-19.md:148).
- T2 Route: N/A - no route; the write happens inside the answer route.
- T3 Repository and types: PASS - RfiService::record_answers_in_tx is the transaction that writes the outbox row (phase-19.md:1844).
- T4 Logic: PASS - the condition and payload are explicit: a no to visit_result writes visit.recorded with party_id, rfi_id and result negative; yes or unknown writes nothing (phase-22.md:949-951).
- T5 Audit: PASS - the outbox row is written in the same transaction as rfi.answer_recorded (phase-19.md:1123), which is the audited mutation. No separate mutation to name.
- T6 Cross-references: GAP (see V-22-09-a). PASS elsewhere: phase 15 names visit.recorded as the TR-19 producer for phase 22 (phase-15.md:2121); rule 3 of section 3.6 includes visit.recorded (phase-15.md:1459-1460); TRG_REVIEW_REQUESTED (phase-15.md:2105).
- T7 Consistency: PASS - agrees with DR-405 (research.md:416) and with 3.12 notices (phase-22.md:750-751). The mapping table has no row for the topic (phase-22.md:597-611), so the dispatcher does not consume it.
- Gaps:
  - `GAP V-22-09-a | MEDIUM | T6 | the question key visit_result is said to be in the branch RFI templates of the shipped pack (phase-22.md:948). The shipped pack is backend/packs/policy/policy-pack.v1.json (phase-09.md:834). No plan file defines a branch RFI template or its question keys, so no seed carries visit_result. | 7.5 (phase-22.md:948) or the phase 09 pack contents | greps: "visit_result" in docs/plan: phase-22.md and research.md:416 only; "branch_info" in phase-09.md: no match; "question_key" seeds outside phase 19: no match`

### V-22-10 7.6 released item that moves to suspicion reads as closed (source: 7.6, adversarial pass run 5 at phase-22.md:955-970; result: GAPS 2)
- T1 SQL: PASS - rfi.tier and rfi.closed_at exist (phase-19.md:92-120); branch.item_closed_days seeded (phase-22.md:315); casework.effective_tier (phase-17.md:530).
- T2 Route: PASS - GET /api/v1/branch/items/{id} (phase-22.md:425); input and evidence answer 409 not_released for a closed item (phase-22.md:484, 491; 7.6 item 2 at phase-22.md:964-965).
- T3 Repository and types: PASS - RfiBranchRepository::released_items and item (phase-22.md:400); BranchStatusMapper and neutral_counts (phase-22.md:460, 630).
- T4 Logic: PASS - 7.6 items 1 to 4 are numbered, with the status and the refusal codes (phase-22.md:958-970).
- T5 Audit: N/A - read path; the dispatcher audit is unchanged (phase-22.md:592).
- T6 Cross-references: GAP (see V-22-10-a).
- T7 Consistency: GAP (see V-22-10-b). The replacements are explicit and PASS: the two step 2 clauses (phase-22.md:958-959), the 3.12 row and test e (phase-22.md:968-970).
- Gaps:
  - `GAP V-22-10-a | MEDIUM | T6 | 7.6 and the base list depend on a closed state and a closed_at clock that start "after phase 19 closes or withdraws" the RFI (phase-22.md:960-961). Phase 19 writes no step that moves released or answered to closed. Its RfiService methods are draft, submit, release, record_answers, extend and kpis (phase-19.md:1060-1135). The RFI closed_at appears only in the DDL and a grant (phase-19.md:124, 779). | phase 19 section 3.2 or 3.3 (a close step that sets closed_at), or phase 22 section 3.1 | greps: "RfiService::close" and "RfiService::withdraw" in phase-19.md: no match; "closed_at = now" and "SET closed_at" in phase-19.md: no match`
  - `GAP V-22-10-b | LOW | T7 | phase 19 allows withdrawal only from draft or pending_release (phase-19.md:541-543). A released RFI can never become withdrawn, so no branch item is ever withdrawn after release. 7.6 says a withdrawn request "also shows" as closed (phase-22.md:968-969); a withdrawn RFI was never released, so it is never listed (released_at is required, phase-22.md:473). | 7.6 item 4 (phase-22.md:968-969) and 3.1 step 2 (phase-22.md:473) | greps: "withdrawn" in phase-19.md after release: only the guard at lines 541-546 (no release-to-withdrawn move)`

### V-22-11 7.7 who reads the internal-report register (source: 7.7, adversarial pass run 5 at phase-22.md:955 and 972-976; result: GAPS 1)
- T1 SQL: N/A - the block adds no table or column.
- T2 Route: PASS - GET /api/v1/internal-reports, scope case.decide (phase-19.md:1039); Cedar followup.a2.report and followup.m.report permit internal_report.read to A2 and M (phase-19.md:1436, 1472).
- T3 Repository and types: N/A - no method added.
- T4 Logic: N/A - a rule, with no numbered steps or refusals.
- T5 Audit: N/A - read only.
- T6 Cross-references: GAP (see V-22-11-a).
- T7 Consistency: PASS - the block says Open doubts 27 is closed and replaces its "M-only fact" sentence (phase-22.md:972-976; base at phase-22.md:1055), so the block wins. Section 3.12 agrees: the MLRO learns the reporter and the register (phase-22.md:746).
- Gaps:
  - `GAP V-22-11-a | LOW | T6 | the reader rule is cited to "phase 19 section 3.14" (phase-22.md:973). That section is the tipping-off table (phase-19.md:1597-1620) and names no reader roles. The readers are in the route table and the section 3.13 Cedar policies (phase-19.md:1039, 1436). The claim that A2 reads the subject party (phase-22.md:974) matches no view field list: phase 19 names only reporter_id for InternalReportView (phase-19.md:1140). | 7.7 citation (phase-22.md:973-974) | greps: "InternalReportView" in phase-19.md: line 1140 only; "subject_party" in phase-19.md view lists: no match`

### V-22-12 7.8 an e-mail is sent only for a queued notice (source: 7.8, adversarial pass run 6 at phase-22.md:978 and 980-985; result: GAPS 1)
- T1 SQL: PASS - email_state CHECK includes queued and sent (phase-22.md:169); the sent CHECK ties email_sent_at to the state (phase-22.md:174).
- T2 Route: N/A - job, no route.
- T3 Repository and types: GAP (see V-22-12-a). MailSender::send and OutgoingMail are named (phase-22.md:406, 735). set_email_state exists (phase-22.md:403).
- T4 Logic: PASS - the state rule and the re-run result are explicit (phase-22.md:981-983; base at phase-22.md:734).
- T5 Audit: PASS - notification.email_sent and notification.email_failed are named (phase-22.md:724-725, 734).
- T6 Cross-references: PASS - admin.notifications columns (phase-22.md:159-176); DR-412 names the same rule (research.md:423).
- T7 Consistency: PASS - the digest paragraph agrees: digest notices move to sent in one transaction per user (phase-22.md:735, 984-985).
- Gaps:
  - `GAP V-22-12-a | MEDIUM | T3 | the immediate job "loads the notice FOR UPDATE" (phase-22.md:981; base at phase-22.md:734). NoticeRepository has no read by id. Its methods are insert_if_new, list, unread_count, mark_read, digest_due, set_email_state and sweep_read_older_than (phase-22.md:403). | section 2 NoticeRepository (phase-22.md:403) | greps: "get(conn, id)" in NoticeRepository: no match (the only get(conn, id) is LiaisonReportRepository, phase-22.md:402); "get_for_update" in phase-22.md: no match`

## Summary

- Items checked: 12 (V-22-01 to V-22-12).
- Items fully PASS: 2 (V-22-01, V-22-08).
- GAP lines by severity: HIGH 0; MEDIUM 10; LOW 12 (total 22).
- Items with gaps: V-22-02 (4), V-22-03 (2), V-22-04 (2), V-22-05 (4), V-22-06 (2), V-22-07 (3), V-22-09 (1), V-22-10 (2), V-22-11 (1), V-22-12 (1).

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-22-02-a | LOW | T1 | No "scanned" evidence status; the scanned state is av_status = 'clean' | 3.2 step 5 (phase-22.md:495) |
| V-22-02-b | LOW | T1 | lang: none is not a ContentLang or CHECK value (phase 02 holds ar, en, ku, mixed, und) | 3.2 step 3 (phase-22.md:486) |
| V-22-02-c | MEDIUM | T6 | A correction is supersedes in phase 19, not corroborates; the branch correction path would write no answer | 3.2 step 3 (phase-22.md:493); AnswerInput (phase-22.md:395) |
| V-22-02-d | LOW | T3 | Phase 22 AnswerInput fields differ from phase 19 AnswerInput; no mapping to the record_answers_in_tx call | section 2 (phase-22.md:395); 3.2 step 6 (phase-22.md:496) |
| V-22-03-a | LOW | T3 | ReceiveCommand is undefined; phase 19 step 1 omits branch_id and subject_party_id that the call sites pass | phase 19 section 3.4 step 1 (phase-19.md:1136); phase 22 3.3 step 6(a) (phase-22.md:508) |
| V-22-03-b | LOW | T6 | "Register detail" does not exist in phase 19 (list route only) | 3.3 (phase-22.md:511); Open doubts 9 (phase-22.md:1037) |
| V-22-04-a | MEDIUM | T6 | No liaison_report_overdue row in the phase 19 section 3.10 table; supervisor M stated only inline | phase 19 section 3.10 table (phase-19.md:1301-1308) or phase 22 section 4.5 |
| V-22-04-b | MEDIUM | T4 | A return after the due timer has fired arms no alarm; 3.7 step 3 is ambiguous for that case | 3.6 timer (phase-22.md:562-564); 3.7 step 3 (phase-22.md:575) |
| V-22-05-a | MEDIUM | T3 | No repository read of a stored template version for the in-app list | TemplateRepository (phase-22.md:404); 3.8.1 (phase-22.md:619) |
| V-22-05-b | LOW | T4 | Template-change validations carry no sub-codes except template_has_figures | 3.8 Template edits (phase-22.md:581); test r (phase-22.md:903) |
| V-22-05-c | LOW | T4 | desk.approval_waiting is seeded with no mapping row that produces it | 3.8 mapping table (phase-22.md:597-611); seed (phase-22.md:201) |
| V-22-05-d | LOW | T6 | serve.public.base_url is a phase 01 prose term, not a defined settings key | 3.13 (phase-22.md:763); phase 01 (phase-01.md:903) |
| V-22-06-a | MEDIUM | T7 | notify.request row allows only task.waiting and task.overdue; 7.1 sends break-glass codes through notify.request and adds no row; the recipient table phase 25 relies on has no rows | 3.8 mapping table (phase-22.md:599); 7.1 (phase-22.md:920-925) |
| V-22-06-b | MEDIUM | T4 | Count-only break-glass notices have no source row or tier; steps 2 to 6 do not cover them | 3.8 steps 2 to 6 (phase-22.md:585-589); 7.1 (phase-22.md:920-925) |
| V-22-07-a | MEDIUM | T7 | Section 2 gives ObservationView to "M and the desk"; 7.2 gives the observation key to M only | section 2 (phase-22.md:386); 7.2 (phase-22.md:931-932) |
| V-22-07-b | LOW | T6 | "Phase 02 sealing service" is not named in phase 02 | section 2 (phase-22.md:386) |
| V-22-07-c | LOW | T3 | The list response type and the repository read for the observation key are not named | 7.2 (phase-22.md:930-933) |
| V-22-09-a | MEDIUM | T6 | The visit_result question key, said to be in the shipped branch templates, is defined by no plan file | 7.5 (phase-22.md:948); phase 09 pack file (phase-09.md:834) |
| V-22-10-a | MEDIUM | T6 | No phase 19 step moves an RFI to closed or sets closed_at; the closed state and its clock have no writer | 7.6 (phase-22.md:960-961); phase 19 RfiService (phase-19.md:1060-1135) |
| V-22-10-b | LOW | T7 | Withdrawal after release is impossible under phase 19's guard, so "a withdrawn request also shows" cannot occur | 7.6 item 4 (phase-22.md:968-969); phase 19 guard (phase-19.md:541-546) |
| V-22-11-a | LOW | T6 | Reader rule cited to phase 19 section 3.14, which names no readers; subject-party visibility is in no view field list | 7.7 (phase-22.md:973-974); phase 19 (phase-19.md:1039, 1436, 1140) |
| V-22-12-a | MEDIUM | T3 | The immediate job loads the notice FOR UPDATE, but NoticeRepository has no read by id | NoticeRepository (phase-22.md:403); 7.8 (phase-22.md:981) |

Notes for the lead: no gap is HIGH. Most gaps are cross-phase names or wording. Two need a decision: V-22-06-a and V-22-06-b (how break-glass notices reach the dispatcher) and V-22-10-a (phase 19 has no step that closes an RFI, so the closed state and its clock have no writer).
