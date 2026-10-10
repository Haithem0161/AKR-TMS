# Refuter, phase 21 (Haiku draft for lead review)

### V-21-01-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: No numbered step inserts rows into rpt.report_links. insert_link has no caller. The "escalation" that section 3.20 names as a source of links is defined nowhere.
- Evidence: docs/plan/phase-21.md:647 is the only place insert_link is named. docs/plan/phase-21.md:1069 "links come from the draft's `earlier_reports` and from the escalation". The escalation is the UTR-to-STR link, which phase 18 hands to this phase: docs/plan/phase-18.md:10 "the UTR-to-STR link is phase 21". Section 3.1 step 6 (docs/plan/phase-21.md:777) pre-fills earlier_reports into the draft content only. RP-19 requires the links as rows: research/23-regulatory-reporting-goaml.md:761 "stored as structured links (relation: previous, amending, superseding, escalated from UTR)". Grep for "escalated_from" and "escalation" in docs/plan/*.md finds no writer in any phase.
- Fix: Phase 21, section 3.1 (or a new escalation act in section 3.5): one sentence naming the act that inserts the rpt.report_links rows through insert_link, and that it is audited as report.linked.

### V-21-01-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: NewReportLink (insert) and ReportLink (return type of links) are named without fields. ReportLink is missing from the section 2 entity list.
- Evidence: docs/plan/phase-21.md:647 "insert_link(&self, tx: &mut Tx, l: &NewReportLink)" and :648 "links(&self, conn: &mut PgConnection, id: Uuid) -> Result<Vec<ReportLink>, RepoError>". The entity list at lines 620-631 has no ReportLink struct. Grep for "NewReportLink" finds line 647 only. The columns are in the DDL at docs/plan/phase-21.md:173-181, so the fields can be inferred.
- Fix: Phase 21, section 2 (domain entities and create DTOs): name the fields of ReportLink and of NewReportLink.

### V-21-02-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: NewReportVersion (ReportVersionRepository::insert) and SealedContent (set_content) are named without fields.
- Evidence: docs/plan/phase-21.md:652 "insert(&self, tx: &mut Tx, v: &NewReportVersion)" and :655 "set_content(&self, tx: &mut Tx, id: Uuid, content: &SealedContent)". Grep finds NewReportVersion at 652 only and SealedContent at 655 only. The fields are inferable from the DDL at docs/plan/phase-21.md:118-132 and from section 3.1 step 8 (line 779): "content_sha256 of the canonical JSON of the plaintext, `content_enc` sealed with AAD = the version id".
- Fix: Phase 21, section 2 (repository create DTOs): name the fields of NewReportVersion and of SealedContent.

### V-21-03-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: Revise (section 3.5a) clears signed_at and signed_by of a rejected or returned report. reports_guard raises "a signature is written once" unless OLD.status is draft or validated, so every revise fails. Section 6 step k expects 200.
- Evidence: docs/plan/phase-21.md:426 "AND OLD.status NOT IN ('draft', 'validated') THEN" and :427 "RAISE EXCEPTION 'a signature is written once';". The guard has no exception for revise (the message occurs at line 427 only). Section 3.5a at docs/plan/phase-21.md:843 says the revise "sets `status = draft`, clears `submitted_*`, `signed_*` and `frozen_version`". Section 6 at docs/plan/phase-21.md:1322 expects "by `a2@`: 200, status `draft`, `current_version` 2". The block 7+ additions (lines 1340-1389) do not touch reports_guard.
- Fix: Phase 21, section 1.3 reports_guard (allow the rejected or returned to draft edge), or section 3.5a (keep signed_at and signed_by as the record). Pick one.

### V-21-03-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: RP-34 requires the approval bound to the manifest hash. The manifest is built after the sign. The sign binds only to the frozen version. The phase never names the hash the sign binds to.
- Evidence: docs/plan/phase-21.md:11 (header, RP-34) "approval bound to the manifest hash, step-up `signing`". research/23-regulatory-reporting-goaml.md:779 (RP-34) "approval is bound to the manifest hash and needs step-up". The sign act binds to "`frozen_version = current_version`" (docs/plan/phase-21.md:850). The manifest is built later: docs/plan/phase-21.md:880 "`manifest_sha256` is the SHA-256 of the canonical manifest bytes." Sections 3.6 and 3.7 (lines 845-873) never mention a manifest. DR-172 (line 1401) names no binding. The frozen content is a deterministic input to the manifest, so the reader can infer the intent, but the phase does not state it (MEDIUM).
- Fix: Phase 21, section 3.6 step 3: name the hash the sign binds to (the frozen version's content hash, or a manifest computed before the sign).

### V-21-03-c  STANDS   (first agent: MEDIUM; refuter: HIGH)
- Claim: The state list in section 3.5 omits two edges that other sections write: validated to packaged for a CTR file, and filed to returned on a poor grade.
- Evidence: docs/plan/phase-21.md:833 "The services write only these transitions:". Section 3.14 writes the CTR edge: docs/plan/phase-21.md:964 "So the file goes from `validated` to `packaged` in the job itself". Section 3.8 agrees: docs/plan/phase-21.md:876 "Status `signed`, or `validated` for a CTR file (section 3.14)". Section 3.12 writes the filed edge: docs/plan/phase-21.md:927 "The report must be `filed`, `acknowledged` or `returned`" and :928 "`poor`: the report becomes `returned`". Section 3.5 names only "`acknowledged` to `returned`: a poor grade (section 3.12)" (line 840). The section 6 table test says "every other pair of the eight statuses is refused (a table test)" (docs/plan/phase-21.md:1305), so the test refuses edges the design writes. No 7.x block says which text wins. Severity raised to HIGH because two binding texts cannot both be satisfied.
- Fix: Phase 21, section 3.5 state list: add both edges, and add them to the section 6 table test.

### V-21-04-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Acknowledgement step 1 locks the filing, then writes the report status with no lock on the report. Section 3.24 takes the report row before its filing.
- Evidence: docs/plan/phase-21.md:906 "1. Lock the filing." Section 3.24 at docs/plan/phase-21.md:1203 "then its package, filing, feedback, findings and links, in that order." Line 1209: "No path takes a lower step and then a higher one." The filing step (line 898) does lock the report first. RP-41 asks for the same: research/23-regulatory-reporting-goaml.md:787 "Acknowledgement capture (reference, receipt upload) that locks the report".
- Fix: Phase 21, section 3.10 acknowledgement step 1: lock the report, then the filing, as section 3.24 requires.

### V-21-04-b  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: The precondition "The report is `filed`" in acknowledgement step 1 has no refusal: no HTTP status and no problem code. Sub-code not_filed is used only by the rejection.
- Evidence: docs/API.md:982 "`E_NOT_ALLOWED_IN_STATE` | 409 | Action not allowed in the resource's status" (the general code for an action refused by status). docs/plan/phase-00.md:497 "`NotAllowedInState` | 409 | `E_NOT_ALLOWED_IN_STATE`". docs/plan/phase-21.md:1042 lists "`not_filed`" among the sub-codes of `E_NOT_ALLOWED_IN_STATE` (409). A failed "report is filed" check is therefore 409 E_NOT_ALLOWED_IN_STATE with sub-code not_filed. Both the code and the sub-code are in the plan. That the rejection also uses the sub-code is a detail, not a gap.

### V-21-04-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The receipt's EvidenceStore::stage names no retention_class, which NewEvidence requires. The portal capture of the same owner kind is R6.
- Evidence: docs/plan/phase-21.md:907 "`EvidenceStore::stage` of the receipt (kind `upload`, owner kind `filing`, owner id = the filing)" (no retention class). The portal capture at line 900 has "retention_class: R6". docs/plan/phase-02.md:611-612 "`NewEvidence { kind, owner_kind,`" then "`owner_id, retention_class, content_type`". docs/plan/phase-02.md:848 "`retention_class` must have an entry in `evidence.lock_days` -> `NoLockPolicy`." The owner registry row (docs/plan/phase-02.md:1182) says "R6 or R7 (phase 21 item 11)", which does not pick one for filings. The filings table is R6: docs/plan/phase-21.md:603 "'Filing records follow their report'". R6 is inferable.
- Fix: Phase 21, section 3.10 acknowledgement step 2: add retention_class R6 to the receipt's NewEvidence, as the filing step does.

### V-21-05-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: The build (line 993) updates generated_by, but the UPDATE grant on rpt.periodic_reports lists no generated_by, so the update is refused by column privilege.
- Evidence: docs/plan/phase-21.md:507 "GRANT UPDATE (status, facts, suppressed_cells, evidence_id, signed_by, signed_at, updated_at, version) ON rpt.periodic_reports TO akr_app;". Line 993: "the row update (`facts`, `suppressed_cells`, `evidence_id`, `generated_by`)". Line 493 says "Every UPDATE lists the columns its service writes", and this one does not. Grep for "generated_by" finds it only at lines 354, 363, 589, 630, 993 and 1000.
- Fix: Phase 21, section 1.3 grants (add generated_by to the UPDATE list), or section 3.16 step 2 (drop it from the row update if step 1 stores it; see the next item).

### V-21-05-b  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: generated_by is never stored. Step 1 inserts no generated_by and the job payload carries no actor. A NULL generated_by satisfies the CHECK and leaves nothing to compare in the sign check, so the person who generated a pack can sign it.
- Evidence: docs/plan/phase-21.md:992 "The row is inserted as `draft` with the period bounds" (no actor). The job payload at line 1223 is "`{ periodic_id }`". The CHECK at line 363 is "CHECK (signed_by IS NULL OR generated_by IS NULL OR signed_by <> generated_by)", so NULL passes. Line 1000: "`signed_by` differs from `generated_by` (403 `E_SELF_APPROVAL`)". With NULL on the pack, that comparison does not refuse.
- Fix: Phase 21, section 3.16 step 1: store generated_by = the requesting actor in the insert, and have the sign check refuse a pack whose generated_by is NULL.

### V-21-05-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Concurrent generates for one kind and period have no lock and no stated order. The unique index uq_periodic_live has no error mapping.
- Evidence: docs/plan/phase-21.md:992 "a draft is superseded by the new draft in the same transaction" (no order between supersede and insert). docs/plan/phase-21.md:365 "CREATE UNIQUE INDEX IF NOT EXISTS uq_periodic_live ON rpt.periodic_reports (kind, period) WHERE status <> 'superseded';". Section 3.16 (lines 988-1002) has no lock step, and the lock order of section 3.24 (lines 1199-1209) has no periodic entry. Grep for "23505" or "unique violation" in docs/plan/phase-00.md finds nothing, and the general problem-code table (docs/plan/phase-00.md:486-505) has no conflict code.
- Fix: Phase 21, section 3.16 step 1: lock the kind and period, supersede, then insert, and map the unique violation to a named code.

### V-21-05-d  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The supersede of a live draft (line 992) changes a row's status with no audit event. Section 3.20 names only periodic.generated and periodic.signed.
- Evidence: docs/plan/phase-21.md:1052 "### 3.20 Audit events (one per mutation;". The table has "| `periodic.generated` | pack built (3.16) | periodic pack |" (line 1083) and "| `periodic.signed` | pack finalised (3.16) | periodic pack |" (line 1084), and nothing for the supersede. The supersede is a status change of a business record, not system housekeeping, so the housekeeping exclusion does not apply.
- Fix: Phase 21, section 3.20: add a periodic.superseded row, and name it in section 3.16 step 1.

### V-21-05-e  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: PeriodicReportService generate and sign have no method names or signatures. NewPeriodic and PeriodicFilter are named but not defined.
- Evidence: docs/plan/phase-21.md:722 "`PeriodicReportService` (section 3.16) and `AnalyticsService` (section 3.17)." (no method names). Line 707 "insert(&self, tx: &mut Tx, p: &NewPeriodic)" and line 710 "PeriodicFilter" (each named once, neither defined). Grep for "generate(" and "sign(" in docs/plan/phase-21.md finds nothing. The steps of section 3.16 are headed by route, not by method name. The fields of NewPeriodic and PeriodicFilter can be inferred from the table at lines 343-365.
- Fix: Phase 21, section 2 (services and repository): name the two service methods, and give the NewPeriodic and PeriodicFilter fields.

### V-21-06-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM for the stands part)
- Claim: "computes whole months" (line 901) has no rounding rule, and no value when the report has no transaction.
- Refuted part (rounding): docs/plan/phase-21.md:901 "computes whole months from the newest booking date of the report's transactions to `filed_at`". "Whole months" names the unit as complete months, so no fraction is kept. The rounding question is answered by the text.
- Stands part (no transaction): RP-16 covers reports without transactions: research/23-regulatory-reporting-goaml.md:758 "for SAR, UTR without transactions and AIF without transactions". The rule at line 901 has no branch for an empty transaction list. The column allows NULL (docs/plan/phase-21.md:194), but the step does not say so (MEDIUM).
- Fix: Phase 21, section 3.10 filing step 4: state the value when the report has no transaction (NULL, unless the bank decides otherwise).

### V-21-06-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: NewFiling (FilingRepository::insert) has no fields. The fields are inferable from the rpt.filings DDL.
- Evidence: docs/plan/phase-21.md:667 "insert(&self, tx: &mut Tx, f: &NewFiling) -> Result<Filing, RepoError>;" (the only mention of NewFiling). The columns are at docs/plan/phase-21.md:185-200.
- Fix: Phase 21, section 2 (create DTOs and repository inserts): name the fields of NewFiling.

### V-21-07-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: CTR-03 "an extreme or non-round amount" has no threshold and no parameter. The Parameters column of the phase 07 row is empty, so the rule cannot be coded.
- Evidence: docs/plan/phase-07.md:974 "| Code | Condition | Severity | Parameters |" and docs/plan/phase-07.md:982 "| CTR-03 | an extreme or non-round amount in a CTR transaction | warn | |". Section 3.13 step 12 (docs/plan/phase-21.md:951) "CTR-03 amount sanity (an extreme or non-round amount is a warn)". The requirement gives no number either: research/23-regulatory-reporting-goaml.md:498 "CTR amounts that are extreme or odd (not round) suggest an EFT sent as a CTR". Grep for "non-round" and "extreme" in docs/plan finds only those lines.
- Fix: Phase 21, block 7.1: state the amount bounds and the roundness test for CTR-03, or name them as governed settings, since neither the research nor phase 07 gives them.

### V-21-07-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: CONS-04 has two meanings: the account rule of section 3.4 step 7, and phase 07's chronological-order info code. Block 7.1 says it is not new.
- Evidence: docs/plan/phase-21.md:816 "CONS-04, new in phase 21 for the RP-30 account rule". Block 7.1 at docs/plan/phase-21.md:1342 "`CONS-04` is not new: phase 07 already has it (info)." Phase 07 at docs/plan/phase-07.md:613 "| CONS-04 | transactions not in chronological order by `date_transaction` | info | |". Block 7.1 keeps phase 07's entry, so it does not settle which meaning the account rule uses. Section 4.5 item 6 (line 1261) still lists CONS-04 among the codes added.
- Fix: Phase 21, section 3.4 step 7: give the account rule its own rule code, and drop the "new" wording for CONS-04 in block 7.1.

### V-21-08-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: Validation race. The job checks the version once, at start. Step 9 calls set_validated with no lock and no re-check. A PATCH in between clears validated_run_id and step 9 sets it again, so an edited, unvalidated version can be submitted.
- Evidence: docs/plan/phase-21.md:810 "Start: the version must still be the current one and unfrozen; otherwise the job completes with `superseded` and writes nothing." Step 9 at line 818: "`set_validated(run, summary, now)`", with no lock or re-check in lines 807-822. Line 835: "a PATCH after validation clears `validated_run_id` on the version." Submit (line 827) then requires "the version must carry `validated_run_id`", which the race leaves set. Nothing blocks a PATCH while the job runs; section 3.2 (line 784) checks only If-Match.
- Fix: Phase 21, section 3.4: hold the version lock from start to step 9 (ReportVersionRepository::lock), or re-check content_sha256 before set_validated.

### V-21-08-b  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: A PATCH on a report that is not draft (signed, packaged, filed or acknowledged) has no refusal in 3.2 or 3.19. "draft only" names no status and no code.
- Evidence: docs/plan/phase-21.md:1349 "(draft only, the narrative limit of section 3.2," (block 7.2 states the rule for PATCH). docs/API.md:982 "`E_NOT_ALLOWED_IN_STATE` | 409 | Action not allowed in the resource's status" (the code for an action refused by status). docs/plan/phase-00.md:497 "`NotAllowedInState` | 409 | `E_NOT_ALLOWED_IN_STATE`". So a PATCH on any of the four named statuses is refused with 409 E_NOT_ALLOWED_IN_STATE. The status set follows from "draft only". The sub-code is not named, which is a detail.

### V-21-08-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: A draft edit changes the content and suspicion_formed_on with no audit event. Section 3.20 names none for PATCH.
- Evidence: docs/plan/phase-21.md:1052 "### 3.20 Audit events (one per mutation;", so the section requires one event per mutation. The table at lines 1054-1087 has no PATCH or draft-edit row (grep finds no "report.edited" or "report.updated"). Line 1068 "| `report.attachment_staged` | attachment staged (3.2) | report |" covers attachments only. A draft edit changes business data, so the housekeeping exclusion does not apply.
- Fix: Phase 21, section 3.20: add a report.edited row for PATCH, and name it in section 3.2.

### V-21-08-d  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: FieldPatch, the element of content_patch, is named and not defined. The text "a field key and a value" makes it inferable.
- Evidence: docs/plan/phase-21.md:635 "`content_patch: Vec<FieldPatch>` (a field key and a value, checked against the builder schema of section 3.2)". The fields of the type are stated in prose, which the refute rules treat as not a gap.

### V-21-08-e  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Section 3.2 says each PATCH writes "a new version of the content". That conflicts with section 1.1, section 3.5a and set_content, which edits the current row in place.
- Evidence: docs/plan/phase-21.md:784 "a new version of the content only when the version is not frozen". Section 1.1 at line 117: "a correction after a rejection is a new version". Section 3.5a at line 843: "It creates version `current_version + 1`". Repository method at line 655: "set_content(&self, tx: &mut Tx, id: Uuid, content: &SealedContent) -> Result<(), RepoError>;   // refused by the trigger once frozen". The in-place reading has three texts behind it, so the gap is wording (MEDIUM).
- Fix: Phase 21, section 3.2: say that the unfrozen current version is replaced in place, and that only revise (section 3.5a) creates a new version.

### V-21-10-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM for the stands part)
- Claim: No hook applies the extra exclusions (the drafter, the version authors and the submitter). Phase 04 computes excluded from the instance maker and the earlier deciders. Section 4.5 item 1 lists the report state checks but not the exclusions.
- Refuted part (hook named): the sign act names where the four-eyes set is applied. docs/plan/phase-21.md:850 "Eligibility is `Holders`: ... Four-eyes: the proposer of the instance, the submitter of the report and the earlier deciders are excluded". Block 7.4 item 1 at docs/plan/phase-21.md:1374 "The excluded set of the `sign` task adds the report's `created_by` and the `created_by` of every version of the". The drafter and the version authors therefore go into the same Holders exclusion as the submitter.
- Stands part (not in the change list): section 4.5 item 1 (docs/plan/phase-21.md:1256) extends check_before_decision with "the report checks of section 3.6 step 3", which are status and version checks, not exclusions. Phase 04 computes excluded only from the maker and the earlier deciders (docs/plan/phase-04.md:1080 "`excluded` = the principal or `acting_for` is the instance maker"). No listed change adds the report's exclusions to Holders (MEDIUM).
- Fix: Phase 21, section 4.5: add an item for the Holders of the sign task (phase 18 CaseWorkflowSubject), naming the drafter, the version authors and the submitter as excluded holders.

### V-21-10-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: ReportVersionSummary is undefined. No named query returns the created_by of every version of a report.
- Evidence: docs/plan/phase-21.md:658 "summaries(&self, conn: &mut PgConnection, report: Uuid) -> Result<Vec<ReportVersionSummary>, RepoError>;" (the only mention of ReportVersionSummary, with no fields). Block 7.4 item 1 needs "the `created_by` of every version of the report" (line 1374). The route row at line 736 mentions only "frozen and validated times".
- Fix: Phase 21, section 2 (ReportVersionRepository): give ReportVersionSummary its fields, including created_by, or name the query that returns the authors.

### V-21-12-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: FreezeService::letter_delivered is specified to record the delivery on the freeze, but no column of casework.freezes holds it. Section 4.5 item 9 keeps the method "proposed only if" gap pass 1 decides so.
- Evidence: docs/plan/phase-21.md:1389 "records the delivery on the freeze. Open doubts 4 is closed." Section 4.5 item 9 at line 1264: "is proposed only if gap pass 1 decides Open doubts 4 that way". The freezes table (docs/plan/phase-20.md:115-160) has committee_report_at at docs/plan/phase-20.md:136, "-- the Committee letter was requested (the start of the two-month clock)", and no delivery column. Grep for "deliver" in that table finds nothing. Phase 20 reads delivery from the letter state (docs/plan/phase-20.md:1080). A column the logic uses and no migration defines is a HIGH gap.
- Fix: Phase 21, block 7.5: either name a delivery column on casework.freezes with its ALTER, or drop the method and rely on the phase 20 letter-state rule.

### V-21-12-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The FK that phase 20 defers to phase 21 on casework.freezes.committee_report_letter_id is not added.
- Evidence: docs/plan/phase-20.md:135 "-- phase 21 letter id; no FK until phase 21" (the deferral). Grep of docs/plan/phase-21.md finds no "ALTER TABLE casework.freezes" and no "REFERENCES rpt.letters". The only ALTER in migration 0070 is on casework.alarms (docs/plan/phase-21.md:377-383).
- Fix: Phase 21, section 1.3 (migration 0070): add the foreign key from casework.freezes.committee_report_letter_id to rpt.letters.

### V-21-12-c  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: The delivery takes the letter and then reaches the freeze, with no lock or transaction for the freeze. Section 3.24 step 5 requires the owner row first.
- Evidence: docs/plan/phase-21.md:1205 "For a letter: the owner row first (phase 20's freeze or refrain, which phase 20 locks before it calls the port), then `rpt.letters`." The rule says "For a letter", so it covers the delivery path and not only the port. docs/plan/phase-21.md:1209 "No path takes a lower step and then a higher one." docs/plan/phase-21.md:637 says the repository signatures "carry the lock and the optimistic-version rules of section 3.24". The transaction is covered by docs/plan/phase-21.md:714 "each takes the transaction of its caller, or opens one". The delivery text at line 982 is silent on the lock, but the general order governs it.

### V-21-12-d  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The freeze side of a delivery names no audit event.
- Evidence: docs/plan/phase-21.md:1052 "### 3.20 Audit events (one per mutation;". The only delivery event is "| `letter.delivered` | delivery record (3.15) | letter |" (line 1081), which covers the letter. Phase 20's freeze events name no delivery: docs/plan/phase-20.md:796 "| `freeze.committee_report_requested` | the first Committee letter requested | freeze |" is the nearest. A freeze write needs an event (the freeze side exists only if block 7.5 is kept).
- Fix: Phase 21, section 3.15 delivery and section 3.20: name the freeze event written with the delivery, or drop the freeze write if V-21-12-a drops the method.

### V-21-12-e  STANDS   (first agent: LOW; refuter: LOW)
- Claim: Section 3.15 delivery points to "section 4.5 item 10". The FreezeService method is item 9. Item 10 is the phase 19 AlarmService change.
- Evidence: docs/plan/phase-21.md:982 "phase 20's `FreezeService` method named in section 4.5 item 10". Line 1264: "9. **Phase 20**: ... `FreezeService::letter_delivered(letter_id, delivered_at)`". Line 1265: "10. **Phase 19**: `AlarmService::raise` accepts the two new alarm kinds".
- Fix: Phase 21, section 3.15 delivery: change "section 4.5 item 10" to "section 4.5 item 9".

### V-21-12-f  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Open doubts 4 is answered two ways. Block 7.5 answers it with a freeze method. Phase 20 block 7.3 answers it with the letter state. Open doubts 4 still says "gap pass 1 decides".
- Evidence: docs/plan/phase-21.md:1389 "Open doubts 4 is closed." (block 7.5). Open doubts 4 at line 1432 still says "Gap pass 1 decides whether phase 20's column means "requested" or "delivered"". docs/plan/phase-20.md:1080 "Phase 21 Open doubts 4 is answered by" (the letter-state rule).
- Fix: Phase 21, Open doubts 4 and block 7.5: keep one answer (the freeze method or the phase 20 letter-state rule), stated the same way in both places.

## Summary

- Claims checked: 31 (every GAP line of tools/agent/verify/phase-21.md; the Summary table of that file repeats them and was not counted twice).
- REFUTED: 4 (V-21-04-b, V-21-08-b, V-21-08-d, V-21-12-c)
- STANDS: 25
- PARTLY: 2 (V-21-06-a, V-21-10-a)

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-21-01-a | STANDS | HIGH | No step inserts report_links rows; the UTR-to-STR link that phase 18 hands to this phase has no writer | section 3.1 or a section 3.5 act |
| V-21-01-b | STANDS | MEDIUM | NewReportLink and ReportLink fields not named; ReportLink not in the entity list | section 2 |
| V-21-02-a | STANDS | MEDIUM | NewReportVersion and SealedContent fields not named | section 2 |
| V-21-03-a | STANDS | HIGH | revise clears signed_* that reports_guard refuses; every revise fails | section 1.3 or 3.5a |
| V-21-03-b | STANDS | MEDIUM | the approval is not bound to the manifest hash that RP-34 names | section 3.6 step 3 |
| V-21-03-c | STANDS | HIGH | section 3.5 list omits the CTR validated-to-packaged and the filed-to-returned edges; the section 6 test refuses them | section 3.5 and section 6 |
| V-21-04-a | STANDS | MEDIUM | acknowledgement locks the filing before the report (section 3.24 order; RP-41) | section 3.10 acknowledgement step 1 |
| V-21-04-c | STANDS | MEDIUM | receipt stage has no retention_class (R6 is inferable) | section 3.10 acknowledgement step 2 |
| V-21-05-a | STANDS | HIGH | UPDATE grant lacks generated_by, which the build writes | section 1.3 grants or 3.16 step 2 |
| V-21-05-b | STANDS | HIGH | generated_by never stored; a NULL value lets the maker sign their own pack | section 3.16 step 1 |
| V-21-05-c | STANDS | MEDIUM | concurrent generates: no lock, no supersede order, no error code | section 3.16 step 1 |
| V-21-05-d | STANDS | MEDIUM | supersede of a live draft has no audit event | section 3.20 |
| V-21-05-e | STANDS | MEDIUM | generate and sign methods unnamed; NewPeriodic and PeriodicFilter fields not named | section 2 |
| V-21-06-a | PARTLY | MEDIUM | no filing value for an activity age when the report has no transaction | section 3.10 filing step 4 |
| V-21-06-b | STANDS | MEDIUM | NewFiling fields not named | section 2 |
| V-21-07-a | STANDS | MEDIUM | CTR-03 bounds for extreme and non-round amounts not given | block 7.1 |
| V-21-07-b | STANDS | MEDIUM | CONS-04 has two meanings (account rule and chronological info) | section 3.4 step 7 and block 7.1 |
| V-21-08-a | STANDS | HIGH | validation race: no lock or re-check at step 9 after a PATCH | section 3.4 |
| V-21-08-c | STANDS | MEDIUM | draft edit has no audit event | section 3.20 |
| V-21-08-e | STANDS | MEDIUM | PATCH "new version" wording against in-place set_content | section 3.2 |
| V-21-10-a | PARTLY | MEDIUM | the drafter, version authors and submitter are in no Holders change of section 4.5 | section 4.5 |
| V-21-10-b | STANDS | MEDIUM | ReportVersionSummary undefined; no query returns version authors | section 2 |
| V-21-12-a | STANDS | HIGH | letter_delivered records a delivery on a freeze column that does not exist | block 7.5 |
| V-21-12-b | STANDS | MEDIUM | FK on freezes.committee_report_letter_id that phase 20 defers to this phase is not added | section 1.3 |
| V-21-12-d | STANDS | MEDIUM | freeze side of a delivery has no audit event | section 3.15 and section 3.20 |
| V-21-12-e | STANDS | LOW | wrong reference "section 4.5 item 10" (should be item 9) | section 3.15 delivery |
| V-21-12-f | STANDS | MEDIUM | Open doubts 4 answered two ways | Open doubts 4 and block 7.5 |

## Notes

- Severity changed from the first agent: V-21-03-c is raised from MEDIUM to HIGH. Two binding texts cannot both be satisfied, and the section 6 table test would refuse a CTR path that section 3.14 writes. All other STANDS and PARTLY severities match the first agent's.
- Not a claim: the same no-fields pattern holds for NewReport (docs/plan/phase-21.md:641), NewReportFinding (:662), NewFeedback (:675), NewReportTask (:680), NewCtrBatch (:689), NewCtrItem (:692) and NewLetter (:698). No GAP line lists them. If the lead accepts V-21-01-b, V-21-02-a, V-21-05-e or V-21-06-b, the same fix should cover these too.
