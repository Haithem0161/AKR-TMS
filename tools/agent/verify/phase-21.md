# Verification pass, phase 21 (Haiku draft for lead review)

Source file: docs/plan/phase-21.md (1475 lines). Read in full: header, sections 1 to 6, 7+, Decisions, Open doubts and the trailer; also .claude/rules/planning.md (template and verification sections) and tools/agent/b3-conventions.md. Other files checked by grep and by cited lines only: phases 00, 01, 02, 04, 07, 09, 10, 15, 17, 18, 19, 20 and 28, docs/API.md section 6, DATA-MODEL.md:687 and docs/plan/research.md (DR-171, DR-368, DR-407, DR-410). Research file set 01-33 not read.

## Selection (listed before the checks)

Quota note: the phase has three gap-pass blocks (7.1, 7.2, 7.3), so six header items were taken instead of five. Adversarial blocks: 7.4 sits under the run 1 header (phase-21.md:1369) and 7.5 under the run 4 header (phase-21.md:1384); both carry ADV IDs, so they are counted as adversarial, not gap-pass.

- V-21-01 RP-19 related-report links (header "Implemented here", position 6 of 40)
- V-21-02 RP-24 locked versions and corrections as versions (position 9)
- V-21-03 RP-34 report state machine and the sign act (position 15)
- V-21-04 RP-41 acknowledgement capture (position 21)
- V-21-05 RP-68 periodic packs (position 34)
- V-21-06 M-41 activity age at filing (position 40)
- V-21-07 block 7.1 CTR rule codes enter the phase 07 catalogue (gap pass run 3a)
- V-21-08 block 7.2 draft update method, PATCH /reports/{id} (gap pass run 4a)
- V-21-09 block 7.3 PEP section of the cbi_monthly pack (gap pass run 5b)
- V-21-10 block 7.4 sign exclusion of the drafter and version authors (adversarial run 1)
- V-21-11 block 7.4 Cedar forbid report.sign_not_drafter (adversarial run 1)
- V-21-12 block 7.5 FreezeService::letter_delivered (adversarial run 4)

## Items

### V-21-01 RP-19 related-report links   (source: header "Implemented here", RP-19; result: GAPS 2)
- T1 SQL: PASS - rpt.report_links with report_id, related_report_id, relation (four values, including escalated_from_utr), CHECK report_id <> related_report_id: phase-21.md:173-181. Grant :495, retention :602, classification :570, append-only trigger :396-398.
- T2 Route: N/A - no route by design: phase-21.md:1069 "POST /reports/{id}/links is not a route".
- T3 Repository and types: GAP (see b). insert_link and links are named with parameters (phase-21.md:647-648), but NewReportLink and ReportLink have no fields and ReportLink is not in the section 2 entity list (:620-631).
- T4 Logic: GAP (see a). No numbered step writes a row of rpt.report_links.
- T5 Audit: PASS - phase-21.md:1069 "`report.linked` | link written" names the event; each write audited.
- T6 Cross-references: PASS - earlier_reports is a column of casework.proposals (phase-18.md:124, JSONB array); the DATA-MODEL row (DATA-MODEL.md:687) matches the DDL relation list.
- T7 Consistency: PASS - no 7.x block changes RP-19.
- Gaps:
  - GAP V-21-01-a | HIGH | T4 | no step inserts rpt.report_links. insert_link has no caller (only :647 names it). Section 3.1 step 6 copies earlier_reports into the sealed content only. The "escalation" that 3.20 names as a source is defined nowhere. | section 3.1 (new step after step 8) or a new escalation act in section 3.5 | greps: "insert_link" in phase-21.md (only :647); "escalat" in phase-21.md (hits :11, :172, :177, :632, :1019, :1024, :1069, :1413; none defines an act that writes a link); "escalated_from_utr" in docs/plan/*.md (phase-21.md and DATA-MODEL.md:687 only)
  - GAP V-21-01-b | MEDIUM | T3 | NewReportLink (insert) and ReportLink (return type of links) are named without fields; ReportLink is missing from the entity list. Fields are inferable from :173-181. | section 2 domain entities and create DTOs | greps: "NewReportLink" in docs/plan/*.md (phase-21.md only, :647); "ReportLink" in phase-21.md (only :647-648)

### V-21-02 RP-24 locked versions; corrections as linked versions   (source: header; result: GAPS 1)
- T1 SQL: PASS - rpt.report_versions with content_enc, content_sha256, frozen_at and UNIQUE (report_id, version): phase-21.md:118-132. Guard "a frozen report version is immutable" at :410 (versions_guard :404-415). UPDATE grant :500.
- T2 Route: PASS - POST /reports/{id}/submit (phase-21.md:740; report.prepare; report.submit) and POST /reports/{id}/revise (:741; report.edit).
- T3 Repository and types: GAP (see a). ReportVersionRepository methods are named with parameters (:651-659). NewReportVersion and SealedContent have no fields. The rest PASS.
- T4 Logic: PASS for submit, steps 1 to 5 (:826-831): "1. Lock the report and the version", 409 not_validated and version_stale, 422 E_REPORT_INVALID. The revise steps (:843) are written, but the revise edge is refused by the guard; see V-21-03-a.
- T5 Audit: PASS - report.submitted (:831, :1058) and report.revised (:843, :1059).
- T6 Cross-references: PASS - the sealed-content pattern is phase 18's "text_enc BYTEA NOT NULL," (phase-18.md:143); the sign act's instance table is admin.workflow_instances (phase-04.md:328).
- T7 Consistency: PASS - the 7.2 block keeps "validated back to draft clears the validation" consistent with :835. The 3.2 wording on PATCH versions is at V-21-08-e.
- Gaps:
  - GAP V-21-02-a | MEDIUM | T3 | NewReportVersion (ReportVersionRepository::insert) and SealedContent (set_content) are named without fields. Fields are inferable from the DDL (:118-132) and the phase 18 text_enc pattern. | section 2 repository and create DTOs | greps: "NewReportVersion" and "SealedContent" in docs/plan/*.md (phase-21.md only, one hit each)

### V-21-03 RP-34 report state machine, maker is not checker, sign act   (source: header; result: GAPS 3)
- T1 SQL: PASS - status CHECK (:84-85), CHECK signed_by <> submitted_by (:107), reports_guard (:417-433), UPDATE grant including status, signed_at, signed_by (:498-499).
- T2 Route: PASS - POST /reports/{id}/sign (phase-21.md:742; report.file; report.sign; SU signing; 4E). POST /reports/{id}/filing (:745; filing.record; M only per :897).
- T3 Repository and types: PASS - ReportApprovalService::sign (:716) with the engine call at :850 (task_id, action, if_match, idem, assertion); Holders (phase-04.md:702) and Eligibility with excluded (:706).
- T4 Logic: GAP (see a and b). The sign steps 1 to 7 (:848-854) are numbered and carry 401, 403 E_SELF_APPROVAL, 404 and 409 case_sealed.
- T5 Audit: PASS - phase 04 writes task.signed (:853; phase-04.md:1390); report.signed (:864 and :1060).
- T6 Cross-references: PASS - WorkflowEngine::act (phase-04.md:1038); CaseWorkflowSubject::check_before_decision (phase-18.md:758); RecordDecisionEffect (phase-18.md:1130); case_report_approval (phase-18.md:113); class signing (phase-01.md:710).
- T7 Consistency: GAP (see c). Block 7.4 extends the excluded set in line with 3.6 step 3 (:1372-1374), so that part PASSES.
- Gaps:
  - GAP V-21-03-a | HIGH | T4 | revise (section 3.5a, :843) clears signed_at and signed_by of a rejected or returned report. reports_guard (:425-427) raises "a signature is written once" unless OLD.status is draft or validated, so every revise fails. Section 6 step k (:1322) expects 200. | section 1.3 reports_guard (allow the rejected or returned to draft edge) or section 3.5a (keep signed_* as the record) | greps: "signature is written once" in phase-21.md (only :427); no exception for revise in phase-21.md; "revise" in phase-21.md (:741, :840, :843, :921, :1059, :1322, and the lines :635, :715, :1290; none changes the guard)
  - GAP V-21-03-b | MEDIUM | T4 | RP-34 requires the approval bound to the manifest hash (header line 11). The manifest is built after the sign (3.8 step 4, :880), and the sign binds only to the frozen version (:850-851). Phase 04 records a "content hash for sign" (phase-04.md:1390). The phase never names the hash the sign binds to. | section 3.6 step 3 (name the bound hash) | greps: "manifest" in phase-21.md (no hit in 3.6 or 3.7); "approval bound" in docs/plan/*.md (none)
  - GAP V-21-03-c | MEDIUM | T7 | the state machine in 3.5 (:833-841) omits two edges that other sections write: validated to packaged for a CTR file (:876 "validated for a CTR file"; :964 "goes from validated to packaged") and filed to returned on a poor grade (3.12 step 1 at :927 accepts a grade on a filed report; step 2 at :928 writes returned). The section 6 table test (:1305) would refuse both. | section 3.5 state list | greps: "validated" with "packaged" in phase-21.md (3.5 list has none for CTR); "filed" with "returned" (3.12 only)

### V-21-04 RP-41 acknowledgement capture   (source: header; result: GAPS 3)
- T1 SQL: PASS - filings ack_at, ack_by, ack_evidence_id (phase-21.md:195-197); CHECK "(ack_at IS NULL) = (ack_by IS NULL)" (:204); filings_guard "an acknowledgement is written once" (:447); UPDATE grant (:501).
- T2 Route: PASS - POST /filings/{id}/acknowledgement (phase-21.md:746; report.file; filing.acknowledge), listed in the Cedar actions (:1125) and the filers permit (:1161).
- T3 Repository and types: PASS - FilingRepository::lock and set_ack (:668-669) with parameters; RecordAcknowledgement (:635); FilingService::record_acknowledgement (:719).
- T4 Logic: GAP (see a, b, c). Steps 1 to 3 are at :906-908: "1. Lock the filing." then "The report is `filed`." The receipt stage is at :907.
- T5 Audit: PASS - report.acknowledged {report_id, filing_id} at :908 and :1064; evidence.staged is emitted by phase 02 (phase-02.md:853).
- T6 Cross-references: PASS - EvidenceStore::stage(tx, NewEvidence, actor) (phase-02.md:839); owner kind filing is registered by phase 21 in the phase 02 owner registry (phase-02.md:1182); class filing (phase-01.md:710).
- T7 Consistency: PASS - no 7.x block changes the item; the close gate at :910 reads the same column.
- Gaps:
  - GAP V-21-04-a | MEDIUM | T4 | lock order. Step 1 locks the filing, then step 3 writes the report status with no lock on the report. Section 3.24 step 3 (:1203) takes the report row before its filing, and :1209 says "No path takes a lower step and then a higher one". | section 3.10 acknowledgement step 1 (lock the report, then the filing) | greps: "Lock the report" in phase-21.md (:898 and :917 only, not in the acknowledgement); "Lock the filing" (:906 only)
  - GAP V-21-04-b | MEDIUM | T4 | the precondition "The report is `filed`" (:906) has no refusal: no HTTP status and no problem code. Section 3.19 has sub-code not_filed under 409 (:1042), used only by the rejection (:917). | section 3.10 acknowledgement step 1 (409 E_NOT_ALLOWED_IN_STATE, sub-code not_filed) | greps: "not_filed" in phase-21.md (:917 and :1042 only); "report is" in the acknowledgement block (:906) with no code
  - GAP V-21-04-c | MEDIUM | T4 | the receipt's EvidenceStore::stage (:907) names no retention_class, which NewEvidence requires (phase-02.md:611-612). The portal capture of the same owner kind is R6 (:900). | section 3.10 acknowledgement step 2 (retention_class R6) | greps: "retention" in phase-21.md lines 905-908 (none); "R6" in lines 905-908 (none)

### V-21-05 RP-68 periodic packs   (source: header; result: GAPS 5)
- T1 SQL: GAP (see a). The table rpt.periodic_reports (:343-365), its CHECKs (:360-363), the suppression floor setting (:518) and the period setting (:528) exist; PASS for those. The build's row update names generated_by (:993), and the grant on the table (:507) lists no generated_by.
- T2 Route: PASS - GET /reports/periodic (phase-21.md:760; dashboard.read; periodic.read), POST /reports/periodic (:761; report.file; periodic.generate), GET /reports/periodic/{id} (:762), POST /reports/periodic/{id}/sign (:763; periodic.sign).
- T3 Repository and types: GAP (see e). PeriodicRepository (:706-710), GeneratePeriodic (:635) and PeriodicReport (:630) are named. NewPeriodic and PeriodicFilter have no fields, and the generate and sign methods of PeriodicReportService have no names (:722).
- T4 Logic: GAP (see b, c). Generate (:992), build (:993) and sign (:1000) are numbered and carry 422 /period, 409 already_final, 409 not_built and 403 E_SELF_APPROVAL.
- T5 Audit: GAP (see d) for the supersede. periodic.generated (:993, :1083) and periodic.signed (:1000, :1084) PASS.
- T6 Cross-references: PASS - casework.internal_reports with channel 'liaison_monthly' (phase-19.md:193); casework.alarms kinds (phase-21.md:377-380); casework.freezes (phase-20.md:115); casework.decisions with type (phase-18.md:142); RenderService::render (phase-02.md:897).
- T7 Consistency: PASS - block 7.3 adds a section to cbi_monthly and leaves the small-number rule (:993) and the "says so on its cover" line (:994) unchanged.
- Gaps:
  - GAP V-21-05-a | HIGH | T1 | the build (:993) updates generated_by, but the UPDATE grant on rpt.periodic_reports (:507) lists no generated_by, so the update is refused by column privilege. The INSERT grant (:494-495) covers the table, but step 1 (:992) inserts no generated_by. | section 1.3 grants (add generated_by) or section 3.16 step 1 (insert it) | greps: "generated_by" in phase-21.md (:354, :363, :589, :630, :993, :1000 only); "GRANT.*generated_by" in phase-21.md (none)
  - GAP V-21-05-b | HIGH | T4 | the actor is never stored. Step 1 (:992) inserts no generated_by, and the build job payload {periodic_id} (:1223) carries no actor. A NULL generated_by satisfies the CHECK (:363) and leaves nothing to compare in the sign check (:1000), so the person who generated a pack can sign it. | section 3.16 step 1 (insert generated_by = the requesting actor) | greps: "generated_by" (no insert in step 1); "periodic_build" in phase-21.md (:9, :761, :988, :992, :1223, :1244; the payload at :1223 is {periodic_id} only)
  - GAP V-21-05-c | MEDIUM | T4 | concurrent generates for one kind and period have no lock and no stated order. "a draft is superseded by the new draft in the same transaction" (:992) does not say supersede before insert, and the unique index uq_periodic_live (:365) has no error mapping. | section 3.16 step 1 (lock the kind and period; supersede, then insert; map the unique violation to a code) | greps: "lock" as a whole word in 3.16 lines 989-1002 (none); "uq_periodic_live" in phase-21.md (:365 only, no error mapping)
  - GAP V-21-05-d | MEDIUM | T5 | the supersede of a live draft (:992) changes a row's status with no audit event. Section 3.20 names only periodic.generated and periodic.signed (:1083-1084). | section 3.16 step 1 and section 3.20 (add periodic.superseded) | greps: "periodic.superseded" in phase-21.md (none); "superseded" in phase-21.md (:349 and :365 are the status CHECK and the index; :992 is the pack supersede)
  - GAP V-21-05-e | MEDIUM | T3 | PeriodicReportService generate and sign have no method names or signatures (:722 names only the service). NewPeriodic and PeriodicFilter are named but not defined; fields are inferable from :343-365. | section 2 services and repository | greps: "NewPeriodic" and "PeriodicFilter" in docs/plan/*.md (phase-21.md only); "generate(" and "sign(" in the 3.16 block (none)

### V-21-06 M-41 activity age at filing   (source: header; result: GAPS 2)
- T1 SQL: PASS - filings.activity_age_months INT with CHECK >= 0 (phase-21.md:194); INSERT grant on rpt.filings (:494-495); setting report.activity_age_months_threshold, governed, floor min 1 (:526-527).
- T2 Route: PASS - POST /reports/{id}/filing (phase-21.md:745; report.file; filing.record).
- T3 Repository and types: GAP (see b). RecordFiling with optional activity_age_months (:897, :635) and FilingService::record_filing (:719) PASS. FilingRepository::insert takes NewFiling (:667), which has no fields.
- T4 Logic: GAP (see a). Filing step 4 (:901) gives the rule for an omitted value but not the rounding or the no-transaction case.
- T5 Audit: PASS - "report.filed {report_id, filing_id, goaml_reference, activity_age_months}" (:902; :1063).
- T6 Cross-references: PASS - canon.txn has PRIMARY KEY (id, booking_date) and booking_date DATE NOT NULL (phase-10.md:964, :995); the setting key is defined in section 1.3 (:526); the analytics output at :1008 reads the same column.
- T7 Consistency: PASS - section 3.17 item 3 (:1008) reads the same column and threshold as 3.10 step 4.
- Gaps:
  - GAP V-21-06-a | MEDIUM | T4 | "computes whole months" (:901) has no rounding rule, and no value when the report has no transaction. Header line 11 carries RP-16 (activity-based reports for SAR, UTR and AIF without transactions). | section 3.10 filing step 4 (state floor to whole months; NULL when there is no transaction) | greps: "whole months" in phase-21.md (:901 only); "without transactions" in phase-21.md (:11 header only)
  - GAP V-21-06-b | MEDIUM | T3 | NewFiling (FilingRepository::insert, :667) has no fields. Fields are inferable from the rpt.filings DDL (:185-200). | section 2 create DTOs | greps: "NewFiling" in docs/plan/*.md (phase-21.md :667 only)

### V-21-07 block 7.1 CTR rule codes enter the phase 07 catalogue   (source: 7.1, gap pass run 3a; result: GAPS 2)
- T1 SQL: N/A - rule codes, no table.
- T2 Route: N/A - no route.
- T3 Repository and types: N/A - the rules run inside phase 07's RuleCatalogue in the section 3.14 pipeline (phase-21.md:963); no method of this phase.
- T4 Logic: GAP (see a). The rules are named at phase-21.md:951 and defined in the phase 07 table (phase-07.md:980-982). CTR-01 and CTR-02 have definitions (phase-07.md:980-981, BR-13 at :596); CTR-03 has none.
- T5 Audit: N/A - the rules write findings; report.validated names counts (phase-21.md:818).
- T6 Cross-references: PASS - rows NAME-01, DATE-01, ID-01, PKG-07 and CTR-01 to CTR-03 exist at phase-07.md:976-982 with the severities of 7.1 and 3.13 step 12; CheckService::validate_built at phase-07.md:963-966; DR-368 at docs/plan/research.md:379.
- T7 Consistency: GAP (see b). Block 7.1 says "CONS-04 is not new" (phase-21.md:1342). Section 3.4 step 7 (:816) and DR-171 (:1399) say it is new for the account rule, and 4.5 item 6 (:1261) adds it to the catalogue. Phase 07 gives CONS-04 a chronological-order meaning (phase-07.md:613, :984).
- Gaps:
  - GAP V-21-07-a | MEDIUM | T4 | CTR-03 "an extreme or non-round amount" (phase-21.md:951; phase-07.md:982) has no threshold and no parameter; the Parameters column of the phase 07 row is empty, so the rule cannot be coded. | block 7.1 (state the amount bounds and the roundness test; make them admin-managed if the bank may change them) | greps: "non-round" in docs/plan/*.md (phase-07.md:982 and phase-21.md:951 only); "extreme" in docs/plan/*.md (the same two lines); "threshold" next to CTR-03 (none)
  - GAP V-21-07-b | MEDIUM | T7 | CONS-04 has two meanings: the account rule of 3.4 step 7 (phase-21.md:816, "new in phase 21 for the RP-30 account rule") and phase 07's chronological-order info code (phase-07.md:613). Block 7.1 (:1342) says it is not new. | section 3.4 step 7 (give the account rule its own code) and 4.5 item 6 (:1261) | greps: "CONS-04" in phase-21.md (:816, :1261, :1342, :1399) and phase-07.md (:613, :984)

### V-21-08 block 7.2 draft update method, PATCH /reports/{id}   (source: 7.2, gap pass run 4a; result: GAPS 5)
- T1 SQL: PASS - draft writes are under grants: rpt.reports.suspicion_formed_on (phase-21.md:498-499), rpt.report_versions content columns (:500), with set_content at :655 and updated_at and version.
- T2 Route: PASS - PATCH /reports/{id} (phase-21.md:735; report.prepare; report.edit); the Cedar permits report.preparers and report.edit_drafts (:1144-1148, :1166-1168) cover it.
- T3 Repository and types: GAP (see d). ReportService::update_draft (phase-21.md:1348) and UpdateReportDraft (:635) are named with parameters; FieldPatch is named (:635) and not defined.
- T4 Logic: GAP (see a and b). The narrative limit is numbered (:788, 422 E_VALIDATION at /narrative/ar), and If-Match gives 428 and 409 (:1044-1045; curl step c at :1314). The "draft only" rule (:1349) has no refusal.
- T5 Audit: GAP (see c). Section 3.20 (:1054-1087) names no event for a draft edit. report.attachment_staged (:1068) covers attachments only.
- T6 Cross-references: PASS - PolicyPackService::scan with scope reason_text (docs/plan/phase-09.md:797) for narrative writes; phase 28 calls the method (phase-28.md:525, :775).
- T7 Consistency: GAP (see e). Section 3.2 (:784) says each PATCH writes "a new version of the content", but 1.1 (:117) and 3.5a (:843) say versions are created only by revise, and set_content (:655) edits the current row.
- Gaps:
  - GAP V-21-08-a | HIGH | T4 | validation race. The validation job checks once, at start, that the version is current (:810). At step 9 it calls set_validated (:818) with no lock and no re-check. A PATCH in between clears validated_run_id (:835) and step 9 sets it again, so an edited, unvalidated version can be submitted (step 1 at :827 passes). | section 3.4 (hold the version lock from step 1 to step 9, using ReportVersionRepository::lock at :654, or re-check content_sha256 at step 9) | greps: whole word "lock" in lines 807-822 of phase-21.md (none); "set_validated" in phase-21.md (:656 and :818 only)
  - GAP V-21-08-b | MEDIUM | T4 | a PATCH on a report that is not draft (signed, packaged, filed or acknowledged) has no refusal in 3.2 or 3.19. "draft only" (:1349) names no status and no code. | section 3.2 (add 409 E_NOT_ALLOWED_IN_STATE, sub-code not_draft) and section 3.19 | greps: "not_draft" in phase-21.md (no sub-code; the only hit is the Cedar id report.sign_not_drafter at :1376); "draft only" in phase-21.md (:1349 only)
  - GAP V-21-08-c | MEDIUM | T5 | a draft edit changes the content and suspicion_formed_on with no audit event; section 3.20 names none for PATCH. | section 3.20 (add report.edited) and section 3.2 | greps: "report.edited" and "report.updated" in phase-21.md (none); "PATCH" in the 3.20 block (none)
  - GAP V-21-08-d | MEDIUM | T3 | FieldPatch (the element of content_patch, :635) is named and not defined. The text "a field key and a value" makes it inferable. | section 2 create and update DTOs | greps: "FieldPatch" in docs/plan/*.md (phase-21.md :635 only)
  - GAP V-21-08-e | MEDIUM | T7 | section 3.2 (:784) "a new version of the content" against section 1.1 (:117, "a correction after a rejection is a new version"), 3.5a (:843, "creates version current_version + 1") and set_content (:655), which edits the current row in place. | section 3.2 (say that the unfrozen current version is replaced in place) | greps: "new version" in phase-21.md (:117, :784, :843, :1407, :1440; no PATCH-created version anywhere)

### V-21-09 block 7.3 PEP section of the cbi_monthly pack   (source: 7.3, gap pass run 5b; result: PASS)
- T1 SQL: PASS - kyc.pep_register with status, tier, source (values pep_list, cbs_class, manual, declaration) and former (phase-15.md:378-395); kyc.pep_relatives with status (:416-428); kyc.kyc_reviews with kind and status (:274-288); kyc.exit_register with party_id (:480); view kyc.party_pep_state (:682); casework.case_parties (phase-17.md:372); casework.decisions with type (phase-18.md:142).
- T2 Route: N/A - a section of the cbi_monthly pack, no route of its own; the generate route is phase-21.md:761.
- T3 Repository and types: N/A - built inside PeriodicReportService (phase-21.md:993).
- T4 Logic: PASS - items a to e are counts under the small-number rule (:993). The EDD sign-off outcome is admin.workflow_instances.outcome (phase-04.md:340); the EDD review is the sign-off's resource (opened from the review sub-step, phase-15.md:1346), and kyc_review is a listed instance resource kind (phase-15.md:1793).
- T5 Audit: N/A - read-only aggregates; periodic.generated names the pack (:1083).
- T6 Cross-references: PASS - kyc_edd_signoff is a workflow definition (phase-15.md:1346); the kyc tables above match the names in 7.3; casework.case_parties (phase-17.md:372) and casework.decisions (phase-18.md:137).
- T7 Consistency: PASS - section 3.16 (:994) and 7.3 (:1365-1366) both say the cover lists the items the system does not hold.
- Gaps: none.

### V-21-10 block 7.4 sign exclusion of the drafter and version authors   (source: 7.4, adversarial run 1; result: GAPS 2)
- T1 SQL: PASS - rpt.reports.created_by (phase-21.md:97) and rpt.report_versions.created_by (:128).
- T2 Route: PASS - POST /reports/{id}/sign (phase-21.md:742).
- T3 Repository and types: GAP (see b). No method returns the authors of all versions. ReportVersionRepository::summaries (:658) returns ReportVersionSummary, which has no fields and no author field.
- T4 Logic: GAP (see a). Block 7.4 step 1 (:1374-1375) writes the rule with 403 E_SELF_APPROVAL, but no code path applies it.
- T5 Audit: N/A - a refused sign writes no row.
- T6 Cross-references: PASS - E_SELF_APPROVAL (docs/API.md:971); Holders and Eligibility.excluded (phase-04.md:702, :706); the excluded definition (phase-04.md:1080).
- T7 Consistency: PASS - matches section 3.6 step 3 (:850) and DR-407 (docs/plan/research.md:418).
- Gaps:
  - GAP V-21-10-a | MEDIUM | T4 | no hook applies the extra exclusions. Phase 04 computes excluded from the instance maker and the earlier deciders (phase-04.md:1080). Section 4.5 item 1 (phase-21.md:1256) lists the report state checks but not the drafter, the version authors or the submitter, although 3.6 step 3 (:850) names the submitter. | section 4.5 item 1 (add the excluded parties to CaseWorkflowSubject::check_before_decision) or the Holders rule | greps: "excluded" in the sign sections of phase-21.md (:850 and :1374 only; other hits are excluded_count and CTR exclusion codes); "check_before_decision" in 4.5 item 1 (:1256, no excluded parties)
  - GAP V-21-10-b | MEDIUM | T3 | ReportVersionSummary (:658) is undefined, and no named query returns the created_by of every version of a report. | section 2 ReportVersionRepository (name the query and its return) | greps: "ReportVersionSummary" in docs/plan/*.md (phase-21.md :658 only)

### V-21-11 block 7.4 Cedar forbid report.sign_not_drafter   (source: 7.4, adversarial run 1; result: PASS)
- T1 SQL: PASS for the attribute - Report entity has created_by?: User (phase-21.md:1113) and rpt.reports.created_by exists (:97).
- T2 Route: PASS - action "report.sign" is listed (:1124) and permitted to M in report.filers (:1158-1159); route POST /reports/{id}/sign (:742).
- T3 Repository and types: N/A - policy only.
- T4 Logic: PASS - the rule is copy-paste ready (:1376-1378): one @id, @deny("self_approval"), forbid on report.sign for resource is Akr::Report, guarded by "resource has created_by".
- T5 Audit: N/A - a forbid writes no row.
- T6 Cross-references: PASS - deny reason "self_approval" is already used by letter.requester_not_signer (:1186-1188); the Akr::Report entity (:1113) and the action (:1124).
- T7 Consistency: PASS - no permit gives A2 report.sign (the A2 permits at :1145-1147 and :1166-1168 list no report.sign), so the forbid conflicts with no permit; it agrees with 3.6 step 3 (:850) and DR-407 (docs/plan/research.md:418).
- Gaps: none.

### V-21-12 block 7.5 FreezeService::letter_delivered   (source: 7.5, adversarial run 4; result: GAPS 6)
- T1 SQL: GAP (see a and b). Block 7.5 says the method "records the delivery on the freeze" (phase-21.md:1388-1389), but the freezes table has no delivered column (phase-20.md:115-160), and 7.5 names none. phase-20.md:135 defers the FK on committee_report_letter_id to phase 21, and phase 21 adds no ALTER on casework.freezes.
- T2 Route: N/A - the caller is POST /letters/{id}/delivery (phase-21.md:759).
- T3 Repository and types: PASS - FreezeService::letter_delivered(letter_id, delivered_at) is named with parameters (:1264 and :1388).
- T4 Logic: GAP (see c). Block 7.5 has no steps. The delivery step (:982) gives no lock order, while section 3.24 step 5 (:1205) puts the owner row first.
- T5 Audit: GAP (see d). letter.delivered (:982; :1081) covers the letter only. The freeze-side write, if any, has no event, and phase 20 names no delivery event (phase-20.md:791-800).
- T6 Cross-references: PASS - committee_report_at (phase-20.md:136); report_missed in the incident kinds (:203) and timer_committee_report_due (:657); the deadline rule that reads the letter state (phase-20.md:1076-1082). The reference in 3.15 to "section 4.5 item 10" is wrong (see e).
- T7 Consistency: GAP (see f). Block 7.5 says the freeze method closes Open doubts 4 (:1388-1389). Phase 20 block 7.3 says the letter-state rule answers it (phase-20.md:1080).
- Gaps:
  - GAP V-21-12-a | HIGH | T1 | FreezeService::letter_delivered is specified to record the delivery on the freeze, but no column of casework.freezes holds it (phase-20.md:115-160), and 4.5 item 9 (:1264) keeps the method "proposed only if" gap pass 1 decides so. | block 7.5 (name the column and its ALTER, or drop the method and cite the phase 20 letter-state rule) | greps: "delivered" in phase-20.md (:1079-1081 only, the letter state, no freeze column); "letter_delivered" in docs/plan/*.md (phase-21.md only); "committee_report_delivered" in docs/plan/*.md (none)
  - GAP V-21-12-b | MEDIUM | T1 | the FK that phase 20 defers to phase 21 on casework.freezes.committee_report_letter_id (phase-20.md:135, "no FK until phase 21") is not added. | section 1.3 or block 7.5 (ALTER TABLE casework.freezes ADD CONSTRAINT ... REFERENCES rpt.letters) | greps: "REFERENCES rpt.letters" and "ALTER TABLE casework.freezes" in phase-21.md (none)
  - GAP V-21-12-c | MEDIUM | T4 | the delivery (:982) takes the letter and then reaches the freeze, but gives no lock or transaction for the freeze. Section 3.24 step 5 (:1205) requires the owner row first. | section 3.15 delivery (lock the freeze, then the letter) | greps: whole word "lock" in lines 980-982 of phase-21.md (none); "lock order" in 3.15 (:970 covers the port only, not the delivery)
  - GAP V-21-12-d | MEDIUM | T5 | the freeze side of a delivery names no audit event. Phase 20 has no delivery event (phase-20.md:791-800) and section 3.20 (:1054-1087) names none for the freeze. | section 3.15 delivery and section 3.20 (name the freeze event, e.g. freeze.committee_report_delivered) | greps: "letter_delivered" and "freeze.committee" in docs/plan/*.md (no delivery event name)
  - GAP V-21-12-e | LOW | T6 | section 3.15 delivery (:982) points to "section 4.5 item 10"; the FreezeService method is item 9 (:1264). Item 10 is the phase 19 AlarmService change (:1265). | section 3.15 delivery | greps: "item 10" in phase-21.md (:982 only)
  - GAP V-21-12-f | MEDIUM | T7 | Open doubts 4 is answered two ways: block 7.5 (:1388-1389) by a freeze method, phase 20 block 7.3 (phase-20.md:1080) by the letter state that the deadline timer reads. Open doubts 4 (:1432) still says "gap pass 1 decides". | Open doubts 4 and block 7.5 (one answer) | greps: "Open doubts 4" in phase-21.md (:982, :1264, :1386, :1389; the open doubt itself at :1432) and phase-20.md (:1080)

## Summary

- Items checked: 12 (V-21-01 to V-21-12).
- Items fully PASS: 2 (V-21-09, V-21-11).
- GAP lines by severity: HIGH 6, MEDIUM 24, LOW 1 (31 in total).

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-21-01-a | HIGH | T4 | no step inserts rpt.report_links; the escalation source is undefined | section 3.1 (new step) or 3.5 |
| V-21-01-b | MEDIUM | T3 | NewReportLink and ReportLink undefined; ReportLink not in the entity list | section 2 |
| V-21-02-a | MEDIUM | T3 | NewReportVersion and SealedContent undefined | section 2 |
| V-21-03-a | HIGH | T4 | revise clears signed_* that reports_guard refuses; every revise fails | section 1.3 or 3.5a |
| V-21-03-b | MEDIUM | T4 | approval not bound to the manifest hash; the sign binds no named hash | section 3.6 step 3 |
| V-21-03-c | MEDIUM | T7 | state list omits validated to packaged (CTR) and filed to returned | section 3.5 |
| V-21-04-a | MEDIUM | T4 | acknowledgement locks the filing before the report (3.24 order) | section 3.10 ack step 1 |
| V-21-04-b | MEDIUM | T4 | "report is filed" precondition has no refusal code | section 3.10 ack step 1 |
| V-21-04-c | MEDIUM | T4 | receipt stage has no retention_class | section 3.10 ack step 2 |
| V-21-05-a | HIGH | T1 | UPDATE grant lacks generated_by, which the build writes | section 1.3 grants |
| V-21-05-b | HIGH | T4 | generated_by never stored; the maker can sign own pack | section 3.16 step 1 |
| V-21-05-c | MEDIUM | T4 | concurrent generate: no lock, no supersede order, no error code | section 3.16 step 1 |
| V-21-05-d | MEDIUM | T5 | supersede of a live draft has no audit event | section 3.16 and 3.20 |
| V-21-05-e | MEDIUM | T3 | generate and sign methods unnamed; NewPeriodic, PeriodicFilter undefined | section 2 |
| V-21-06-a | MEDIUM | T4 | whole-month rounding and no-transaction case not stated | section 3.10 filing step 4 |
| V-21-06-b | MEDIUM | T3 | NewFiling undefined | section 2 |
| V-21-07-a | MEDIUM | T4 | CTR-03 extreme and non-round amount have no parameters | block 7.1 |
| V-21-07-b | MEDIUM | T7 | CONS-04 has two meanings (account rule and chronological info) | section 3.4 step 7, 4.5 item 6 |
| V-21-08-a | HIGH | T4 | validation race: no lock or re-check at step 9 after a PATCH | section 3.4 |
| V-21-08-b | MEDIUM | T4 | PATCH on a non-draft report has no refusal | section 3.2 and 3.19 |
| V-21-08-c | MEDIUM | T5 | draft edit has no audit event | section 3.20 |
| V-21-08-d | MEDIUM | T3 | FieldPatch undefined | section 2 |
| V-21-08-e | MEDIUM | T7 | PATCH "new version" wording against in-place set_content | section 3.2 |
| V-21-10-a | MEDIUM | T4 | no hook applies the drafter and author exclusions | section 4.5 item 1 or Holders |
| V-21-10-b | MEDIUM | T3 | ReportVersionSummary undefined; no version-author query | section 2 |
| V-21-12-a | HIGH | T1 | letter_delivered records a delivery on a freeze column that does not exist | block 7.5 |
| V-21-12-b | MEDIUM | T1 | FK on freezes.committee_report_letter_id deferred to phase 21 is not added | section 1.3 or block 7.5 |
| V-21-12-c | MEDIUM | T4 | delivery locks the letter before the freeze; no lock stated | section 3.15 delivery |
| V-21-12-d | MEDIUM | T5 | freeze side of a delivery has no audit event | section 3.15 and 3.20 |
| V-21-12-e | LOW | T6 | wrong reference "section 4.5 item 10" (should be item 9) | section 3.15 delivery |
| V-21-12-f | MEDIUM | T7 | Open doubts 4 answered two ways (block 7.5 and phase 20 block 7.3) | Open doubts 4 |

## Notes outside the sample (not counted as GAP lines)

- Section 3.13 step 6 (phase-21.md:945) closes the fx_rate_missing task with `closed_by` NULL ("the system"), but the CHECK at :256, "CHECK ((closed_at IS NULL) = (closed_by IS NULL))", refuses it.
- Section 4.2 names handler jobs timer_report_filing_target and timer_report_rework_due (:1234-1235) that are not in the section 4.1 job table (:1217-1226).
- Section 3.12 step 8 (:934) points to "section 3.7 step 3" for closing the returned task; step 3 (:864) closes nothing.
- Section 3.15 manual letters (:984) use sub-code not_approved, which section 3.19 (:1042) does not list.
- Phase 28 cites "lock 3 of section 3.15" (docs/plan/phase-28.md:525); the lock order is section 3.24 (:1199).
- Cedar file: @role("A2,M") (:1144) and @role("M,CH,IA") (:1178) break the one-role-per-@role rule (tools/agent/b3-conventions.md:92-93).
- Section 3.16 liaison pack uses a fixed "tenth of the following month" (:995); planning.md section 3 wants such values admin-managed.
