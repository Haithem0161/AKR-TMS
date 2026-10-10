# Refuter, phase 22 (Haiku draft for lead review)

### V-22-02-a  STANDS  (first agent: LOW; refuter: LOW)
- Claim: 3.2 step 5 needs an evidence object with status "scanned", and phase 02 has no such status.
- Evidence: phase-02.md:350 "CREATE TYPE admin.av_status AS ENUM ('pending', 'clean', 'infected', 'rejected');". Greps: "scanned" (case-insensitive) in phase-02.md hits only the `scanned_at` column (lines 371, 381, 445, 451, 467, 602, 620). In phase-22.md, "scanned" (lines 479, 487, 495, 572, 846, 858) is used but never defined as a status. research.md: no hit for "scanned" or "av_status". No 7.x block defines it.
- Fix: Phase 22, section 3.2 step 5: name the state as `av_status = 'clean'` with `version_id` set, the phase 02 enum value, instead of "scanned".

### V-22-02-b  STANDS  (first agent: LOW; refuter: LOW)
- Claim: 3.2 step 3 passes `lang: none`, which is not a ContentLang or column CHECK value.
- Evidence: phase-02.md:367 "TEXT NOT NULL CHECK (lang IN ('ar', 'en', 'ku', 'mixed', 'und'))"; phase-02.md:607-608 `ContentLang { Ar, En, Ku,` continued on line 608 by `Mixed, Und }`. Greps: "'none'" and "lang.*none" in phase-02.md: no match. In phase-22.md, `lang: none` appears only at line 486, and "und" appears nowhere in phase-22.md.
- Fix: Phase 22, section 3.2 step 3: pass `lang: und` (the phase 02 value for undetermined) or name the value the branch upload takes.

### V-22-02-c  STANDS  (first agent: MEDIUM; refuter: HIGH)
- Claim: a branch correction goes "through corroborates", but corroborates writes no new claim, so a correction would store no answer. Phase 19 uses supersedes.
- Evidence: phase-22.md:493 "(or `answered` when it corrects a current claim through `corroborates`, phase 19 section 3.3 step 2)". phase-19.md:1119 "(or `answered` when the claim corrects a current one through `supersedes`; the service sets the old claim `superseded`)". phase-19.md:1120 "writes no new claim for that entry". research.md:161 (DR-150) "corrections supersede". Greps: "supersede" in phase-22.md has no hit in section 3.2 and no supersedes field in AnswerInput (line 395); the only hit is the planning note at line 1052.
- Severity note: the two texts contradict on the correction step, which the scale grades HIGH. The first agent gave MEDIUM.
- Fix: Phase 22, section 3.2 step 3 and section 2 `AnswerInput`: name `supersedes` (phase 19 section 3.3 step 2) as the correction path and add a `supersedes` field to AnswerInput.

### V-22-02-d  STANDS  (first agent: LOW; refuter: LOW)
- Claim: phase 22 `AnswerInput` is not the phase 19 `AnswerInput`, and step 6 gives no mapping for the call.
- Evidence: phase-22.md:395 "(the phase 19 `AnswerInput` with the branch's source kind fixed to `branch_officer`)", but its fields are `question_key`, `value` (max 500), `evidence_ids` and `corroborates`. phase-19.md:983 "AnswerInput { question_id, answer_kind, value: String (1 to 2000, sealed), ...". Partly mapped: phase-22.md:495 puts evidence ids in the sealed value, not in `evidence_object_id`. Not mapped: `question_key` to `question_id`. Grep "question_id" in phase-22.md: no match. The lookup is inferable from phase-19.md:153 "UNIQUE (rfi_id, question_key)". DR-201 (phase-22.md:991) names no mapping.
- Fix: Phase 22, section 2 `AnswerInput` or section 3.2 step 6: one sentence on the `question_key` to `question_id` lookup and on the 500-character cap that differs from phase 19.

### V-22-03-a  PARTLY  (first agent: LOW; refuter: LOW)
- Claim: `ReceiveCommand` is named but not defined in phase 19 or 22. Phase 19 section 3.4 step 1 lists five fields, but the call sites also pass `branch_id` and `subject_party_id`.
- Evidence (refuted part): phase-19.md:985 "`CreateInternalReport { reporter_kind, reporter_id: Uuid, branch_id: Option<Uuid>, subject_party_id: Option<Uuid>, channel, body: String (1 to 4000, sealed), received_at }`". The two fields are defined there, in section 2. phase-19.md:1135 says the branch window calls the port "with the same command".
- Evidence (standing part): grep "ReceiveCommand" in docs/plan/*.md: phase-22.md:508 only. Phase 19 never uses the name, and phase 22 never uses `CreateInternalReport`. The fields exist; only the name differs.
- Fix: Phase 22, section 3.3 step 6(a): name the command `CreateInternalReport` (phase 19 section 2) instead of `ReceiveCommand`.

### V-22-03-b  PARTLY  (first agent: LOW; refuter: LOW)
- Claim: the groups are read "through the register detail", a route phase 19 does not have. Phase 19 has only `GET /internal-reports` (a list), `POST` and acknowledge.
- Evidence (refuted part): phase-22.md:930 "This phase owns the read extension of Open doubts 9 (a): the rows of phase 19's `GET /internal-reports` whose internal" and phase-22.md:933 "Phase 19's route, scope and Cedar action do not change." The read is given as the list rows, so no route is missing.
- Evidence (standing part): grep "register detail" in docs/plan/*.md: phase-22.md:511 and phase-22.md:1037 only. No GET on `internal-reports/{id}` in phase-19.md (the routes are at phase-19.md:1039-1041). The phrase names no phase 19 route.
- Fix: Phase 22, section 3.3 "Tipping-off in this flow" (line 511) and Open doubts 9(a): name the list rows of `GET /internal-reports` (section 7.2) in place of "the register detail".

### V-22-04-a  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: phase 19 section 3.10 has no `liaison_report_overdue` row, phase 22 adds none, and the supervisor (M) is stated only inline.
- Evidence: phase-19.md:1861-1862 "`liaison_report_overdue` with subject `liaison_report` (phase 22)" and "Each owner adds its rows to the table of section 3.10". The 3.10 table (phase-19.md:1301-1308) holds six kinds. Greps: "liaison_report_overdue" in phase-22.md at lines 46, 564, 607, 793 and 897 only, none of them a table row. Section 4.5 (phase-22.md:813-827): no match for "alarm" or "3.10". The supervisor is inline at phase-22.md:564: "the supervisor is M, section 3.10 table". The trailer (phase-22.md:1060) excludes changes to phase 19's alarm tables, which are database tables; the 3.10 table is documentation, so the trailer does not cover it.
- Severity note: MEDIUM, because the supervisor is inferable from the inline text.
- Fix: Phase 22, section 4.5: add an item that adds a `liaison_report_overdue` row (subject `liaison_report`, supervisor M) to the phase 19 section 3.10 table.

### V-22-04-b  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: 3.7 step 3 promises an overdue alarm for a returned, unsigned report. The timer fires once, and a fire on a signed report is a no-op that is not re-armed, so a return after the fire raises no alarm, and the text does not say what happens.
- Evidence: phase-22.md:575 "a report returned and not re-signed by the due day raises the overdue alarm". phase-22.md:563 "Status `signed` or `reviewed`: no-op". The return step at phase-22.md:574 "(status back to `draft`, `returned_at`, `returned_by`; the signature fields stay as history)" schedules no timer. phase-22.md:795 "Scheduled by the transaction that inserts the draft". DR-211 (phase-22.md:1011) "the timer fires the next morning, once". phase-00.md:563 "set `fired_at = now()`" and phase-00.md:1013 "already fired or was cancelled matches nothing". Open doubts 15 (phase-22.md:1043) covers a late signature only, not a return. Greps: "re-arm" and "reschedul" in phase-22.md: no match on the return path.
- Severity note: the outcome is stated in 3.7 step 3, so a developer can infer a check on return, but the text does not pick the mechanism. MEDIUM.
- Fix: Phase 22, section 3.7 step 2e: say whether a return after the timer has fired raises the alarm at once when past due, or schedules a new `liaison_due` timer.

### V-22-05-a  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `NotificationService::list` renders the stored template version from `admin.notification_template_versions`, but no repository method reads one version.
- Evidence: phase-22.md:404 "`TemplateRepository`: `get(conn, code)`, `all(conn)`, `put_version(tx, NewVersion)`". `get(conn, code)` takes no version. phase-22.md:403 gives `NoticeRepository::list(conn, user, cursor, unread_only)` with no join named. phase-22.md:619 "each item renders the stored `template_version` from `admin.notification_template_versions`". phase-22.md:581 "Notices keep the version they were written with". Greps: "get_version" and "version(conn" in phase-22.md: no match.
- Severity note: the table and the key (code, version) are in the DDL at phase-22.md:146-156, so the read is inferable. MEDIUM.
- Fix: Phase 22, section 2 `TemplateRepository` (line 404): add a read of one version by (code, version), or name the join in `NoticeRepository::list`.

### V-22-05-b  STANDS  (first agent: LOW; refuter: LOW)
- Claim: the template-change checks in 3.8 "Template edits" carry no per-check error codes, except `template_has_figures`. The 422 envelope is inherited from phase 04.
- Evidence: phase-22.md:581 "Validation before the change is accepted: both languages present; placeholders from the template's `params` only; no digit outside a placeholder (phase 09 rule `template_has_figures`)". phase-00.md:445-446 `ValidationError {` and `pointer, code, params }`, so each error carries a code. phase-04.md:1366 "`E_VALIDATION` with `errors[]` codes from DSL.md s.7". Greps: "forbidden_term" or "placeholder" sub-codes in phase-22.md 3.8 Template edits: no match.
- Fix: Phase 22, section 3.8 "Template edits": name one error code for each check, since each error needs a `code`.

### V-22-05-c  STANDS  (first agent: LOW; refuter: LOW)
- Claim: `desk.approval_waiting` is seeded, but no row of the mapping table produces it.
- Evidence: phase-22.md:201 "('desk.approval_waiting', ..., '{count}')". The mapping table at phase-22.md:597-611 has no approval topic. Greps: "approval_waiting" and "approval.waiting" in docs/plan/*.md: phase-22.md:201 only. Approvals reach users as tasks: phase-04.md:1713 "`escalate` writes `notify.request {task_id, template_code: "task.waiting", ...`". Open doubts 8 (phase-22.md:1036) acknowledges the same pattern for `staff.report_decision`; no doubt covers `desk.approval_waiting`.
- Fix: Phase 22, section 3.8 mapping table: name the topic that produces `desk.approval_waiting`, or drop the seed and the code from section 1.2.

### V-22-05-d  STANDS  (first agent: LOW; refuter: LOW)
- Claim: `serve.public.base_url` is named as a setting read from phase 01, but phase 01 mentions it only in prose and no settings row defines it.
- Evidence: phase-22.md:763 "Settings read from other phases: ... `serve.public.base_url` (phase 01)". phase-01.md:903 "`serve.public.base_url` is the public origin", which is prose. The public origin that phase 01 defines is an environment variable: phase-01.md:879 "`AKR_PUBLIC_ORIGIN`, `AKR_TRUSTED_PROXIES`". Greps: "serve.public" in docs/plan/*.md: phase-01.md:903 and phase-22.md only. No `serve.` key in the settings seeds of phase-00.md or phase-01.md.
- Fix: Phase 22, sections 3.8.3 (line 734) and 3.13 (line 763): name the phase 01 config item `AKR_PUBLIC_ORIGIN` in place of the settings key `serve.public.base_url`.

### V-22-06-a  STANDS  (first agent: MEDIUM; refuter: HIGH)
- Claim: the 3.8 mapping row for `notify.request` admits only `task.waiting` or `task.overdue`, but phase 25 sends the break-glass codes on `notify.request`. 7.1 adds the codes and a recipient rule but no mapping row, and the phase 22 recipient table that phase 25 relies on has no rows.
- Evidence: phase-22.md:599 "the producer names the code; the code is one of the two" (template column: "`task.waiting` or `task.overdue` (count 1)"). phase-25.md:775 "Enqueue the notice `notify.request {template_code: ..., count}`". phase-25.md:816 "recorded in the phase 22 recipient table". The same row conflicts with phase 22's own section 4.5 item 5(a) (phase-22.md:821): the notify effect may write `notify.request {workflow_instance_id, template_code, to}` with "one of the closed codes of section 1.2". 7.1 (phase-22.md:920-925) adds the codes and says nothing about the row. research.md:254 (DR-243) records the two codes but no row. Greps: "break_glass" in phase-22.md: lines 922-924 only; "active holders" in phase-22.md: 7.1 only.
- Severity note: the row and the phase 25 producer contradict each other, and no 7.x text says which rule wins. The scale grades a contradiction HIGH; the first agent gave MEDIUM. The path carries a security notice.
- Fix: Phase 22, section 3.8 mapping table, row `notify.request`: list the two break-glass codes and their active-holder recipient rule, and align the row with section 4.5 item 5(a).

### V-22-06-b  STANDS  (first agent: MEDIUM; refuter: HIGH)
- Claim: dispatcher steps 2 and 3 read a source row by payload ids and compute a tier. A break-glass `notify.request` carries only `template_code` and `count`, so there is no source row and no tier. Step 6 counts over a visible set, not the payload count. 7.1 gives recipients only.
- Evidence: phase-22.md:585 "Read the source row by the ids in the payload (no text is read from the payload; the payload carries ids and codes only)." phase-22.md:586 "Compute the resource's tier". phase-22.md:589 "Counts (`count`) are computed per recipient over that recipient's visible set, never over the resource's full set." phase-25.md:775 "`notify.request {template_code: ..., count}`". Every other notify.request payload carries an id: phase-04.md:1712-1713 (`task_id`) and phase-22.md:821 (`workflow_instance_id`). phase-22.md:583 "phase 00 back-off applies on error", so the row retries under back-off. Greps: "count" as a `notify.request` payload key in phase-22.md: no match; a no-source-row case in 3.8: no match. 7.1 (phase-22.md:920-925) names recipients and says "Neither text names a case, party, requester or reason"; it does not say that steps 2, 3 or 6 are skipped.
- Severity note: a developer cannot run steps 2, 3 and 6 for a count-only notice, and no text says which to skip. A missing step is HIGH. The first agent gave MEDIUM.
- Fix: Phase 22, section 3.8 steps 2 to 6: say that a notice with a `count` payload and no source row skips steps 2 and 3, has no tier, and takes its count from the payload.

### V-22-07-a  STANDS  (first agent: MEDIUM; refuter: HIGH)
- Claim: section 2 says `ObservationView` is "for M and the desk only". 7.2 and DR-385 say M only, and the block does not say it replaces section 2.
- Evidence: phase-22.md:386 "`ObservationView` (for M and the desk only) carries the decrypted notes". phase-22.md:931-932 (7.2) "`notes` is decrypted for M only, and other readers get no `observation` key". Open doubts 9(a) (phase-22.md:1037) "the register detail gains them for M only". research.md:396 (DR-385) "(M only, notes decrypted for M)". phase-19.md:1848 (7.1) "`GET /internal-reports` carries the observation fields for M only". Greps: "M and the desk" in 7.2: no match.
- Severity note: the texts say who may read decrypted notes, which is a confidentiality rule. Only section 2 gives the desk access, and 7.2, Open doubts 9(a) and DR-385 all say M only. The scale grades a contradiction HIGH; the first agent gave MEDIUM.
- Fix: Phase 22, section 2 `Observation` line (phase-22.md:386): change "for M and the desk only" to M only, or say in 7.2 that it replaces that phrase.

### V-22-07-b  STANDS  (first agent: LOW; refuter: LOW)
- Claim: the notes are decrypted "through the phase 02 sealing service", and phase 02 names no sealing service.
- Evidence: phase-22.md:386 "carries the decrypted notes through the phase 02 sealing service". phase-02.md:632-633 lists the services: `PgAuditSink`, `PgAccessLog`, `AnchorService`, `Verifier`, `PartitionMaintenance`, `EvidenceStore`, `RenderService`, `PackService`. phase-02.md:540 names a `crypto` crate, and phase-02.md:830 the AKRE1 format. Greps: "sealing service", "SealService" and "sealer" in phase-02.md: no match. phase-22.md:507 (3.3 step 5) names the phase 18 field-sealing pattern for these same notes.
- Fix: Phase 22, section 2 `Observation` line (phase-22.md:386): name the phase 18 field-sealing pattern that 3.3 step 5 already uses, in place of "the phase 02 sealing service".

### V-22-07-c  PARTLY  (first agent: LOW; refuter: LOW)
- Claim: the list response gains an `observation` key with four sub-fields, but no response type or repository read is named. The only named type is `ObservationView`.
- Evidence (refuted part, the count): phase-22.md:931 "carry `observation {indicator_groups," and phase-22.md:932 "escalation_criteria, notes}` for M only". That is three sub-fields, not four.
- Evidence (standing part): no repository read for the list join is named. `ObservationRepository` (phase-22.md:401) has `insert`, `set_hybrid` and `unchecked` only. The list row type is not named; phase 19 names `InternalReportView` (phase-19.md:1140), which 7.2 extends. Grep "observation {" as a type name in phase-22.md: 7.2 only.
- Fix: Phase 22, section 2 `ObservationRepository` or 7.2: name the read of observations by `internal_report_id` that the register list uses, and the list row type it extends.

### V-22-09-a  REFUTED  (first agent: MEDIUM; refuter: none)
- Claim: the question key `visit_result` is said to be in the branch RFI templates of the shipped pack, but no plan file defines a branch RFI template or its question keys, so no seed carries it.
- Evidence: phase-19.md:1964 (Open doubts 2) "The RFI templates, the question keys, the purpose codes and the `authority_response.v1` template are pack content that the compliance owner writes (phase 09)." phase-09.md:765 "the templates (bilingual neutral texts for tasks, notices, RFI and branch forms)". phase-19.md:1699 "The pack document gains the RFI templates". The key name is fixed in research.md:416 (DR-405) "phase 22 writes `visit.recorded` on a `no` to `visit_result`". So the missing seed is the designed choice: the pack is bank configuration, and the compliance owner writes its keys.
- Note for the lead: phase-22.md:948 assumes the compliance owner writes `visit_result`. That dependency is covered by the phase 19 Open doubts 2 owner rule, so it is not a phase 22 gap.

### V-22-10-a  STANDS  (first agent: MEDIUM; refuter: HIGH)
- Claim: 7.6 and the base list rely on a closed state and a `closed_at` clock that start "after phase 19 closes or withdraws" the RFI, but phase 19 never moves an RFI to `closed` or sets `closed_at`.
- Evidence: phase-22.md:960-961 "after phase 19 closes or withdraws it, for `branch.item_closed_days` from `closed_at`". phase-19.md:544-545 guard allows the move ("OLD.status = 'released'" to "NEW.status IN ('answered', 'closed')"), but no step performs it. The RfiService steps at phase-19.md:1060-1135 (draft, submit, release, record_answers, extend, kpis) set no `closed` status and no `closed_at`. The `closed_at` column at phase-19.md:124 and the grant at phase-19.md:779 are the only other mentions. Greps: "RfiService::close" and "RfiService::withdraw" in phase-19.md: no match; "closed_at = now" and "SET closed_at" in phase-19.md: no match. Section 7.10 (phase-19.md:1903-1914) suspends reminders for a closed case; it closes no RFI.
- Severity note: the base list (phase-22.md:473) reads the `closed` state and nothing writes it, so no RFI ever closes. The trigger is not inferable from the text, so the scale grades it HIGH, as a missing step. The first agent gave MEDIUM.
- Fix: Phase 19, section 3.3 (phase-19.md:1115): name the step and trigger that move a released or answered RFI to `closed` and set `closed_at`.

### V-22-10-b  STANDS  (first agent: LOW; refuter: LOW)
- Claim: phase 19 allows withdrawal only before release, so a released RFI never becomes withdrawn. Yet 7.6 says a withdrawn request also shows as closed, and a withdrawn RFI was never released, so it is never listed.
- Evidence: phase-19.md:542-543 "AND NEW.status IN ('pending_release', 'withdrawn'))" (from draft) and "AND NEW.status IN ('released', 'rejected', 'withdrawn'))" (from pending_release). The guard at phase-19.md:544-546 has no released-to-withdrawn move. phase-22.md:473 "`rfi.released_at IS NOT NULL`" excludes an RFI that was never released. phase-22.md:968-969 "which a withdrawn" and "request also shows". phase-19.md:1106 (on_ended withdrawn) is also before release. Greps: "withdrawn" in phase-19.md after the release steps: no release-to-withdrawn move.
- Fix: Phase 22, section 7.6 item 4 (phase-22.md:968-969): drop the "withdrawn request also shows" comparison, since a withdrawn request is never released and never listed.

### V-22-11-a  PARTLY  (first agent: LOW; refuter: LOW)
- Claim: the register reader rule is cited to "phase 19 section 3.14". That section is the tipping-off table, names no reader roles, and the readers are in the route table and section 3.13. The claim that A2 reads the subject party matches no view field list, since phase 19 names only `reporter_id` for `InternalReportView`.
- Evidence (standing part, the citation): phase-19.md:1597 "### 3.14 Tipping-off review" names no roles. The Cedar readers are in section 3.13 (heading at phase-19.md:1357): phase-19.md:1436 `@id("followup.a2.report") @role("A2")` and phase-19.md:1472 `@id("followup.m.report") @role("M")`. phase-19.md:1607 mentions only "the register readers".
- Evidence (refuted part, the subject party): the Cedar permit at phase-19.md:1437-1439 grants A2 `internal_report.read` on `resource is Akr::InternalReport` with no attribute condition. The entity at phase-19.md:965 holds `subject_party_id`. So the A2 read is supported by the permit and the entity, even without a view field list.
- Fix: Phase 22, section 7.7 (phase-22.md:973): cite phase 19 section 3.13 (Cedar `followup.a2.report` and `followup.m.report`) in place of section 3.14.

### V-22-12-a  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the immediate job "loads the notice FOR UPDATE", but `NoticeRepository` has no read by id.
- Evidence: phase-22.md:981 "`notification_email` loads the notice `FOR UPDATE` and sends only when `email_state = 'queued'`". phase-22.md:734 "load the notice; the recipient's address is `admin.users.email`". phase-22.md:403 gives the `NoticeRepository` methods: `insert_if_new`, `list`, `unread_count`, `mark_read`, `digest_due`, `set_email_state`, `sweep_read_older_than`. Greps: "get(conn, id)" in phase-22.md: line 402 only (`LiaisonReportRepository`). "get_for_update" in phase-22.md: no match. "FOR UPDATE" in phase-22.md: lines 554, 570, 772 and 981, none of them a notice read.
- Severity note: the table and key (`admin.notifications.id`, the primary key) are named, so the read is inferable. MEDIUM, as the first agent gave.
- Fix: Phase 22, section 2 `NoticeRepository` (line 403): add one read of a notice by id that takes the row lock, used by the immediate job.

## Summary

- Claims checked: 22 (V-22-02-a to V-22-12-a; V-22-08 has no gap lines).
- REFUTED: 1 (V-22-09-a). PARTLY: 4 (V-22-03-a, V-22-03-b, V-22-07-c, V-22-11-a). STANDS: 17.
- Severity raised above the first agent on V-22-02-c, V-22-06-a, V-22-06-b, V-22-07-a and V-22-10-a: each is a contradiction or a missing step, which the scale grades HIGH.

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-22-02-a | STANDS | LOW | "scanned" is not an av_status value; the state is `clean` | 3.2 step 5 (phase-22.md:495) |
| V-22-02-b | STANDS | LOW | `lang: none` is not a phase 02 value; the value is `und` | 3.2 step 3 (phase-22.md:486) |
| V-22-02-c | STANDS | HIGH | A correction is said to use `corroborates`, which writes no claim; phase 19 uses `supersedes` | 3.2 step 3 (phase-22.md:493); section 2 AnswerInput (phase-22.md:395) |
| V-22-02-d | STANDS | LOW | AnswerInput differs from phase 19; the `question_key` to `question_id` lookup is not stated | section 2 AnswerInput (phase-22.md:395); 3.2 step 6 (phase-22.md:496) |
| V-22-03-a | PARTLY | LOW | The name `ReceiveCommand` is not a phase 19 name; phase 19 calls the type `CreateInternalReport` | 3.3 step 6(a) (phase-22.md:508) |
| V-22-03-b | PARTLY | LOW | "Register detail" names no phase 19 route; 7.2 gives the list rows | 3.3 (phase-22.md:511); Open doubts 9 (phase-22.md:1037) |
| V-22-04-a | STANDS | MEDIUM | No `liaison_report_overdue` row in the phase 19 section 3.10 table | section 4.5 (phase-22.md:813-827) or phase 19 section 3.10 (phase-19.md:1301-1308) |
| V-22-04-b | STANDS | MEDIUM | A return after the timer fired arms no alarm; step 3 does not say how | 3.7 step 2e (phase-22.md:574); 3.7 step 3 (phase-22.md:575) |
| V-22-05-a | STANDS | MEDIUM | No repository read of one stored template version | TemplateRepository (phase-22.md:404) |
| V-22-05-b | STANDS | LOW | Template-change checks carry no per-check error code | 3.8 Template edits (phase-22.md:581) |
| V-22-05-c | STANDS | LOW | `desk.approval_waiting` is seeded with no producer | 3.8 mapping table (phase-22.md:597-611); seed (phase-22.md:201) |
| V-22-05-d | STANDS | LOW | `serve.public.base_url` is not a phase 01 settings key; phase 01 defines `AKR_PUBLIC_ORIGIN` | 3.8.3 (phase-22.md:734); 3.13 (phase-22.md:763) |
| V-22-06-a | STANDS | HIGH | The `notify.request` row allows two codes; phase 25 and section 4.5 item 5(a) send others; no break-glass mapping row | 3.8 mapping table (phase-22.md:599) |
| V-22-06-b | STANDS | HIGH | Steps 2, 3 and 6 need a source row and payload id that a count-only break-glass notice lacks | 3.8 steps 2 to 6 (phase-22.md:585-589) |
| V-22-07-a | STANDS | HIGH | Section 2 gives `ObservationView` to the desk; 7.2 and Open doubts 9(a) give M only | section 2 (phase-22.md:386) |
| V-22-07-b | STANDS | LOW | "Phase 02 sealing service" is not a phase 02 name | section 2 (phase-22.md:386) |
| V-22-07-c | PARTLY | LOW | No repository read or list row type named for the observation key; the key has three sub-fields, not four | section 2 ObservationRepository (phase-22.md:401); 7.2 (phase-22.md:930-933) |
| V-22-10-a | STANDS | HIGH | No step moves an RFI to `closed` or sets `closed_at` | phase 19 section 3.3 (phase-19.md:1115-1131) |
| V-22-10-b | STANDS | LOW | 7.6 item 4 says a withdrawn request shows, but a withdrawn RFI is never released or listed | 7.6 item 4 (phase-22.md:968-969) |
| V-22-11-a | PARTLY | LOW | The citation is section 3.14; the roles are in 3.13. The A2 subject-party read is supported by the Cedar permit and the entity | 7.7 (phase-22.md:973-974) |
| V-22-12-a | STANDS | MEDIUM | No read of a notice by id with a row lock | section 2 NoticeRepository (phase-22.md:403) |
