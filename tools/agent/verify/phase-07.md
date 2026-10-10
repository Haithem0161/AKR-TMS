# Verification pass, phase 07 (Haiku draft for lead review)

## Sample (12 items, listed before checking)

Phase 07 has no "Adversarial pass" block (grep -i adversarial docs/plan/phase-07.md: no match), so the 3 adversarial
slots move to the header. Its gap-pass blocks are 7.1 and 7.2 only (both headed "Gap pass 1 (2026-10-10), run 3a"),
so the 4 gap slots are filled from them (7.1 has one method; 3 rows are taken from the 7.2 table).

Header items (8). The header list has 18 IDs: 15 in "Requirements carried" (lines 18-31) and 3 in the two bullets
(RP-08 and L-12 at line 32, VB-49 at line 33). Items are taken at list positions 1, 3, 6, 8, 10, 13, 15 and 18:
- V-07-01 RP-01 profile registry (header line 18, position 1)
- V-07-02 RP-03 watcher, diff, corpus run, promotion by change request (header line 20, position 3)
- V-07-03 RP-06 regression corpus (header line 23, position 6)
- V-07-04 RP-09 open-questions register per profile (header line 26, position 8)
- V-07-05 RP-28 business-rule catalogue changed by change request (header line 27, position 10)
- V-07-06 RP-42 rejection-message parser stub (header line 29, position 13)
- V-07-07 RP-83 business-rejection-rule corpus per profile (header line 30, position 15)
- V-07-08 VB-49 pre-submission goAML validator, failures block (header line 33, position 18)

Gap items (4), from 7.1 (line 963) and 7.2 (line 970):
- V-07-09 7.1 CheckService::validate_built (line 964)
- V-07-10 7.2 rule NAME-01 (line 976)
- V-07-11 7.2 rule PKG-07 (line 979)
- V-07-12 7.2 rule CTR-02 (line 981)

## Checks

### V-07-01 RP-01 profile registry   (source: header RP-01, phase-07.md:18; result: GAPS 4)
- T1 SQL: PASS - rpt.goaml_profiles (phase-07.md:54-98) has every column the logic names (status, source, patches, lookups, schema_summary, rules, limits, xsd_sha256, based_on, change_request_id). Owner tables exist: admin.evidence_objects (phase-02.md:390) and admin.change_requests (phase-04.md:177).
- T2 Route: GAP - see V-07-01-a. Writes: PUT /goaml/profiles/{id} (phase-07.md:450, scope config.author, CR goaml_profile). Reads: GET rows at phase-07.md:448-449; their scopes omit CHK.
- T3 Repository and types: GAP - see V-07-01-c for NewProfileVersion. PASS for GoamlProfileRepository (phase-07.md:363-373) and ProfileLoader::load(xsd, profile) -> LoadedProfile (phase-07.md:467).
- T4 Logic: GAP - see V-07-01-b for the loader refusals. PASS for the rest: ProfileLoader steps 1-4 (phase-07.md:468-482), the seed sequence (phase-07.md:484-492) and the advisory lock on insert (phase-07.md:402-404).
- T5 Audit: PASS - `goaml_profile.seeded {code, profile_version, xsd_sha256, activated}` (phase-07.md:491; row at :718). Candidate and activation events are at phase-07.md:719 and :721.
- T6 Cross-references: PASS - admin.evidence_objects (phase-02.md:390); put_generated (phase-02.md:876); owner kind goaml_profile at R9 (phase-02.md:1172); admin.change_requests (phase-04.md:177); ChangeTarget trait (phase-04.md:642); packs seed (phase-06.md:40); admin.users (phase-01.md:52).
- T7 Consistency: GAP - see V-07-01-d. Otherwise PASS: a shipped version may be activated without a change request (phase-07.md:488-489), which the CHECK at phase-07.md:97 allows.
- Gaps:
GAP V-07-01-a | MEDIUM | T2 | GET /goaml/profiles and GET /goaml/profiles/{id} do not admit CHK: their scopes are report.prepare and config.author only, but CHK holds config.approve (API.md:96, the scope table in section 2 of API.md). The Cedar policy admits CHK (phase-07.md:762-764), section 5 lists CHK as a reader of the profile page (phase-07.md:884) and the curl suite expects 200 for CHK (phase-07.md:947). | Route rows at phase-07.md:448-449 (and API.md:578-579) | greps: `grep -c config.approve docs/plan/phase-07.md` = 0. CHK is a profile reader in the action table (phase-07.md:737), Cedar, the note at :772-773 and sections 5 and 6, but in no route row.
GAP V-07-01-b | MEDIUM | T4 | ProfileLoader refusals name no error: a `<!DOCTYPE` (phase-07.md:468), an unknown rule code (phase-07.md:481) and weights not summing to 100 or bands not descending (phase-07.md:482) have no error name, and the watcher has no error code for them (phase-07.md:651-652 names only xsd_unparseable). GoamlError has no variant list (phase-07.md:709). | Section 3.1 steps 1 and 4; section 3.8 error mapping | greps: `grep -n GoamlError docs/plan/phase-07.md` = line 709 only; no DOCTYPE error name in phase-07.md.
GAP V-07-01-c | MEDIUM | T3 | NewProfileVersion, the input of GoamlProfileRepository::insert, has no field list. The caller-supplied fields and the loader-derived ones (xsd_sha256, lookups, schema_summary, effective_xsd_evidence_id) are not separated. | Section 2, Create/Update structs (phase-07.md:356) | greps: `grep -n NewProfileVersion docs/plan/phase-07.md` gives lines 356 and 364 only; no struct of that name in phase-00, -02, -04 or -21.
GAP V-07-01-d | LOW | T7 | Section 1 says content never changes once a version leaves candidate (phase-07.md:52-53, :109), but lookups and schema_summary are outside the freeze list (phase-07.md:115-121) and akr_app may UPDATE them (phase-07.md:167). Say whether they are frozen with the XSD. | Section 1, migration 0023 trigger and GRANT (phase-07.md:110-127, :167) | greps: no code path in section 2 or 3 updates lookups or schema_summary after activation.

### V-07-02 RP-03 watcher, diff, corpus run and promotion by change request   (source: header RP-03, phase-07.md:20; result: GAPS 2)
- T1 SQL: PASS - rpt.goaml_watch_runs (phase-07.md:137-158) holds every column that steps 1-6 write (status, attachment_list, xsd_sha256, candidate_profile_id, error_code, detail, finished_at). Settings used exist in migration 0025: goaml.watch_enabled, watch_url, watch_hosts, watch_match, watch_stale_days and schedule.goaml_watch (phase-07.md:287-297). diff_from_base and based_on are on rpt.goaml_profiles (phase-07.md:84-85).
- T2 Route: PASS - GET and POST /goaml/watch-runs (phase-07.md:451-452) with scopes and Cedar actions goaml_watch.read and goaml_watch.run; PUT /goaml/profiles/{id} for promotion (phase-07.md:450). The import path is the CLI `akr-ctl goaml import-xsd` (phase-07.md:659) and needs no route.
- T3 Repository and types: GAP - see V-07-02-a for NewWatchRun and WatchEnd. PASS for GoamlWatchRepository (phase-07.md:376-381: start, finish, list, last_success) and SchemaDiff::diff(base, new) -> ProfileDiff (phase-07.md:666).
- T4 Logic: PASS - 3.5 steps 1-6 are numbered (phase-07.md:643-657). Refusals are named: watch_disabled and run_active as 409 E_NOT_ALLOWED_IN_STATE (phase-07.md:641); failure codes at phase-07.md:148-150 and :155. One running watch per code is enforced by the index (phase-07.md:159-160).
- T5 Audit: PASS - goaml_watch.requested, .changed, .unchanged, .unreachable, .failed (phase-07.md:722); goaml_profile.candidate_created (phase-07.md:655); goaml_profile.corpus_ran (phase-07.md:674).
- T6 Cross-references: PASS - ChangeTarget (phase-04.md:642); cr_material route rule (phase-04.md:575); the reporting family is named as phase 07's (phase-04.md:1734); E_STALE_BASE is in API.md section 6 (API.md:980); egress.proxy and z2_egress are in phase 00's block 7.7 (phase-00.md:930).
- T7 Consistency: GAP - see V-07-02-b. Otherwise PASS: a watcher candidate cannot become active without a change request (phase-07.md:661-662), which the CHECK at phase-07.md:97 enforces.
- Gaps:
GAP V-07-02-a | MEDIUM | T3 | NewWatchRun (the input of start) and WatchEnd (the input of finish) are named but have no field lists. The columns exist (trigger, requested_by, profile_code, source_url; status, attachment_list, xsd_sha256, candidate_profile_id, error_code, detail) but the split between caller input and derived values is not given. | Section 2, Create/Update structs (phase-07.md:358) and GoamlWatchRepository (phase-07.md:377-378) | greps: `grep -n 'NewWatchRun\|WatchEnd' docs/plan/phase-07.md` gives lines 358, 377 and 378 only; no struct of either name in phase-00, -02, -04, -21.
GAP V-07-02-b | LOW | T7 | The action table lists goaml_watch.read for A2, M and CFG only (phase-07.md:738), but the Cedar policy goaml.profile_readers grants it to CHK as well (phase-07.md:763). The note at phase-07.md:773 says the route scope keeps CHK off the watch routes, so the table row or the policy is wrong. | Section 3.8 action table (phase-07.md:738) or the Cedar excerpt (phase-07.md:762-764) | greps: `grep -n 'goaml_watch.read' docs/plan/phase-07.md` gives lines 451, 738, 763 and 773 only; the table has no CHK.

### V-07-03 RP-06 regression corpus   (source: header RP-06, phase-07.md:23-24; result: GAPS 2)
- T1 SQL: PASS - corpus_result JSONB (phase-07.md:86) holds the run result. The corpus cases are release files (phase-07.md:320-321: corpus/valid, corpus/cases and cases.json with 63 cases), so no table is needed.
- T2 Route: N/A - the corpus runs as job goaml_corpus (phase-07.md:804-805) and as CLI `akr-ctl goaml corpus` (phase-07.md:675). Its result is read through GET /goaml/profiles/{id} (phase-07.md:449).
- T3 Repository and types: PASS - GoamlProfileRepository::set_corpus_result(tx, id, &CorpusResult) (phase-07.md:369); the CorpusResult shape is given at phase-07.md:674 (ran_at, passed, failed, cases: [{file, expected, got}]).
- T4 Logic: GAP - see V-07-03-a (no refusal when corpus_result is missing; no outcome when the job fails). PASS for the comparison rule: outcome plus the exact set of block codes, warnings listed (phase-07.md:671-674).
- T5 Audit: PASS - goaml_profile.corpus_ran {code, profile_version, passed, failed} (phase-07.md:674; row at :720).
- T6 Cross-references: GAP - see V-07-03-b. PASS for the rest: packs seed enqueues goaml_corpus (phase-07.md:490; packs seed is in phase-06.md:40); the cases.json shape is in section 2 (phase-07.md:320-321).
- T7 Consistency: PASS - the corpus needs no transient store because it is release data (phase-07.md:672). Section 6 expects each of the 63 cases to give its expected block codes (phase-07.md:902-903), the same rule as 3.6.
- Gaps:
GAP V-07-03-a | MEDIUM | T4 | A candidate without corpus_result has no refusal. The promote precondition "corpus_result present" (phase-07.md:686) gets an error only for failed cases (422 corpus_failed). If goaml_corpus fails both attempts (max attempts 2, phase-07.md:805) corpus_result stays NULL and no outcome is stated, so the change request cannot be decided. | Section 3.6 Prepare row (phase-07.md:686) and section 4, goaml_corpus (phase-07.md:804-805) | greps: `grep -n corpus_failed docs/plan/phase-07.md` gives lines 686 and 708 only; no error name for a missing corpus_result.
GAP V-07-03-b | MEDIUM | T6 | The FIU-answers corpus is said to come from phase 21: "Phase 21 adds the bank's own anonymised rejection cases" (phase-07.md:617; also :24 and :700). Phase 21 has no such text. Its only corpus mention is the dependency line that names phase 07's regression corpus (phase-21.md:5), and it defines no cases, table, route or job for it. | Section 3.3 (phase-07.md:615-617) against phase 21 section 3 | greps: `grep -n -i 'corpus\|anonymi\|fiu-answer' docs/plan/phase-21.md` gives line 5 only; `grep -n -i corpus docs/plan/research.md` gives DR-68, DR-71 and DR-100 only.

### V-07-04 RP-09 open-questions register per profile   (source: header RP-09, phase-07.md:26; result: GAPS 1)
- T1 SQL: PASS - open_questions JSONB with its shape in the column comment (phase-07.md:81-82); UPDATE granted (phase-07.md:168); classification row (phase-07.md:182).
- T2 Route: PASS - additions and answers go through PUT /goaml/profiles/{id} as an amendment (phase-07.md:450). The register is read through GET /goaml/profiles/{id} (phase-07.md:449) and the profile page (phase-07.md:884-885). The CHK scope gap is logged at V-07-01-a.
- T3 Repository and types: PASS - ProfileAmendment includes open_questions (phase-07.md:357). OpenQuestion is a value object (phase-07.md:353) whose JSON shape is in the comment at phase-07.md:81.
- T4 Logic: PASS - the amend Prepare row runs the body checks and inserts a candidate (phase-07.md:686). Loosening needs "an answered open question" (phase-07.md:686).
- T5 Audit: PASS - an amendment creates a candidate and audits goaml_profile.candidate_created (phase-07.md:719; audit table at :714-722).
- T6 Cross-references: PASS - ChangeTarget trait (phase-04.md:642), the cr_material route rule (phase-04.md:575) and the volume_effect enum with "neutral" (phase-04.md:50).
- T7 Consistency: GAP - see V-07-04-a.
- Gaps:
GAP V-07-04-a | LOW | T7 | Section 1 says content never changes once a version leaves candidate (phase-07.md:52-53; trigger message at :109). The freeze list leaves open_questions out (phase-07.md:115-121) and akr_app may UPDATE it on an active version (phase-07.md:168). No repository method (phase-07.md:363-373) or route writes it outside an amendment. Either state that answers go through a change request only, or freeze the column. | Section 1, migration 0023 trigger and GRANT (phase-07.md:110-127, :168) | greps: `grep -n open_questions docs/plan/phase-07.md` gives lines 82, 168, 182, 357 and 498 only; no method or route writes the column directly.

### V-07-05 RP-28 business-rule catalogue changed by change request   (source: header RP-28, phase-07.md:27; result: GAPS 1)
- T1 SQL: PASS - rules JSONB (phase-07.md:72-73), keyed by the closed catalogue. The findings rule_code CHECK (phase-07.md:239) accepts every code shape in 3.3 and 7.2, for example NAME-01 and CTR-02.
- T2 Route: PASS - PUT /goaml/profiles/{id} with a ProfileAmendment (phase-07.md:450, :356-357), CR goaml_profile, scope config.author, Cedar change_request.create (phase-07.md:740).
- T3 Repository and types: GAP - see V-07-05-a for the rule parameter schema. PASS for RuleCode, RuleConfig { enabled, severity, params } and Severity (phase-07.md:349-350).
- T4 Logic: PASS - the catalogue rows give the condition of each code (phase-07.md:584-613). The amend refusals are named: rule_unknown, params_invalid and loosening_needs_basis as E_VALIDATION reasons (phase-07.md:708); the loosening rule is at phase-07.md:686.
- T5 Audit: PASS - goaml_profile.candidate_created (phase-07.md:719) and goaml_profile.activated (phase-07.md:721).
- T6 Cross-references: PASS - volume_effect value "neutral" (phase-04.md:50), cr_material route rule (phase-04.md:575), validation_run as a phase 04 evidence kind used by required_evidence (phase-04.md:632; phase-07.md:686).
- T7 Consistency: PASS - "never add code" (phase-07.md:577) agrees with the release-only codes in 7.2 (phase-07.md:971-972). The overlap between BR-13 and CTR-02 is logged once, at V-07-12-b.
- Gaps:
GAP V-07-05-a | MEDIUM | T3 | The rule parameter schema is required but not written. The profile is validated "against goaml-profile.v1.json and each rule's parameter schema" (phase-07.md:480-481). The table gives parameter names and some defaults only; BR-03 report_codes (phase-07.md:586), BR-14 placeholders (phase-07.md:597) and BR-18 currencies (phase-07.md:601) have no value or type. | Section 3.1 step 4 and section 2 (phase-07.md:324, schemas/goaml-profile.v1.json), with the parameter types in section 3.3 | greps: `grep -n 'parameter schema' docs/plan/phase-07.md` matches line 686 only (the phrase wraps at 480-481); no schema file content anywhere in the phase.

### V-07-06 RP-42 rejection-message parser stub   (source: header RP-42, phase-07.md:29; result: GAPS 1)
- T1 SQL: PASS - rejection_patterns JSONB (phase-07.md:79-80, shape in the comment); with_rejection_text (phase-07.md:216); findings layer 'office' (phase-07.md:237-238) and the rule_code CHECK, which accepts OFF-99 (phase-07.md:239).
- T2 Route: PASS - POST /goaml/check takes rejection_text (CheckRequest at phase-07.md:430; route row at :444, "optional Office rejection text"); Cedar goaml_check.run (phase-07.md:733), scope report.prepare.
- T3 Repository and types: PASS - RejectionParser::parse(text, tree, patterns) -> Vec<OfficeFinding> (phase-07.md:693); RejectionPattern is a value object (phase-07.md:353) with its shape in the comment at phase-07.md:79. OfficeFinding's fields follow from 3.7 (layer office, the pattern's rule_code, element paths and lines) and the findings table (phase-07.md:233-247).
- T4 Logic: GAP - see V-07-06-a (length limits without a named reason). PASS for the rest: 3.7 gives the normalise, pattern-order, OFF-99 and paths-at-most-20 steps (phase-07.md:693-699).
- T5 Audit: PASS - the text is never stored (phase-07.md:699; section 6 at :923). The run's audit event carries with_rejection (phase-07.md:519).
- T6 Cross-references: PASS - the 400 error for a missing key, E_IDEMPOTENCY_REQUIRED, is in API.md section 6 (API.md:966); the body-size code E_PAYLOAD_TOO_LARGE is at API.md:997; the rejection_text part 0 is defined in 3.2 step 4 (phase-07.md:515-517).
- T7 Consistency: PASS - part 0 exists only for one file (phase-07.md:508, :558) and the text is never stored (phase-07.md:699, :923), which agree with each other.
- Gaps:
GAP V-07-06-a | MEDIUM | T4 | The 20,000-character limit on rejection_text has no error reason. The violation rule (422 E_VALIDATION) lists reasons that do not include a text-length reason (too_many_files, file_too_large, type_not_allowed, rejection_needs_one_file, profile_not_usable), and section 3.8 adds none for it. The 8 MiB limit on xml_text has no separate reason either. | Section 3.2 step 2 (phase-07.md:508, :511-512) and section 3.8 reason list (phase-07.md:707-708) | greps: `grep -n '20,000\|8 MiB\|too_long\|too long' docs/plan/phase-07.md` gives line 508 only; no text-length reason anywhere in the phase.

### V-07-07 RP-83 business-rejection-rule corpus per profile   (source: header RP-83, phase-07.md:30-31; result: GAPS 1)
- T1 SQL: GAP - see V-07-07-a. Cases added by a profile amendment have no column or table. corpus_result (phase-07.md:86) holds results only, and section 1 defines four tables with no case storage (phase-07.md:54-98, :137, :206, :233).
- T2 Route: GAP - see V-07-07-a. No route takes a case. PUT /goaml/profiles/{id} takes a ProfileChangeRequest whose amendment has no cases field (phase-07.md:435-436, :450).
- T3 Repository and types: GAP - see V-07-07-a. ProfileAmendment has seven fields (phase-07.md:356-357), none for cases, and the repository traits have no case method (phase-07.md:363-397).
- T4 Logic: GAP - see V-07-07-a. The amend Prepare row (phase-07.md:686) says nothing about adding or validating cases.
- T5 Audit: N/A until V-07-07-a is fixed. If the cases travel inside the amendment, goaml_profile.candidate_created (phase-07.md:719) would cover them.
- T6 Cross-references: PASS - the goaml_corpus job runs every corpus file (phase-07.md:671-672, :804-805) and the cases.json shape is at phase-07.md:320-321. The FIU-answers claim is logged at V-07-03-b.
- T7 Consistency: GAP - see V-07-07-a. Section 3.3 says the corpus includes "the cases a profile amendment adds (section 3.6)" (phase-07.md:615), but 3.6 does not describe it (phase-07.md:664-689).
- Gaps:
GAP V-07-07-a | HIGH | T1 | No storage, field or route for the cases a profile amendment adds, which is what makes RP-83 "grown by profile change". Each such case is a file with its expected rule codes and the Office message that revealed the rule (phase-07.md:615-617), but nothing in sections 1, 2 or 3.6 stores, accepts or validates one. | Section 1 (a column or case table), section 2 (ProfileAmendment field and PUT body, phase-07.md:356-357 and :435-436) and section 3.6 (Prepare row, phase-07.md:686) | greps: `grep -n cases docs/plan/phase-07.md` gives lines 24, 86, 320-321, 615, 617, 672, 674, 686 and 903; none of them adds a case to a profile. No case method in the repository traits (phase-07.md:363-397).

### V-07-08 VB-49 pre-submission goAML validator (schema and Office rules, failures block)   (source: header VB-49, phase-07.md:33; result: GAPS 4)
- T1 SQL: PASS - goaml_check_runs (phase-07.md:206-231) holds the run, the pinned profile, files, summary, error_code (CHECK at :218) and transient_deleted. goaml_check_findings (phase-07.md:233-249) holds layer, rule_code, severity, path and params. Every name the job writes exists.
- T2 Route: PASS - POST /goaml/check (phase-07.md:444; scope report.prepare; goaml_check.run), GET /goaml/check-runs/{id} (:446) and GET .../findings (:447). Idempotency-Key is required (phase-07.md:505 and :458).
- T3 Repository and types: GAP - see V-07-08-c for the write types, and V-07-08-d for the stage_transient types. PASS for GoamlCheckRepository (phase-07.md:384-397) and for CheckRequest, CheckAccepted, CheckRunView and CheckFindingView (phase-07.md:430-436).
- T4 Logic: PASS - POST steps 1-5 (phase-07.md:505-520) and job steps 1-13 (phase-07.md:523-566) are numbered. The 422 reasons are named (phase-07.md:511-512). The failure codes profile_unavailable and signatures_stale are in the run CHECK (phase-07.md:218). Completion is one transaction (phase-07.md:561-562). The retry conflict is logged at T7 as V-07-08-a.
- T5 Audit: PASS - goaml_check.requested (phase-07.md:519), .completed (:561-562), .failed (:716) and .transient_deleted (:717; sweep at :807-808).
- T6 Cross-references: GAP - see V-07-08-d. PASS for the rest: stage_transient, open_transient and delete_transient are defined in phase 02 section 7.2 (phase-02.md:1194-1197); clamd INSTREAM (phase-02.md:862); evidence.staging_ttl_h (phase-02.md:518); maintenance_sweep schedule (phase-00.md:256); E_IDEMPOTENCY_REQUIRED (phase-00.md:488) and E_PAYLOAD_TOO_LARGE (phase-00.md:498).
- T7 Consistency: GAP - see V-07-08-a (retry) and V-07-08-b (candidate use).
- Gaps:
GAP V-07-08-a | HIGH | T7 | The retry path contradicts the first job step. Step 1 calls start and then says "a run not queued -> done" (phase-07.md:523; start returns an Option, phase-07.md:386). After a first attempt has started the run, a retry finds it no longer queued and exits. Yet the antivirus step says the job retries and finally fails signatures_stale (phase-07.md:527-528), and section 4 says "a retry restarts the run's files" (phase-07.md:798-799). As written, the fail-closed path never completes and the run stays running. Phase 02 avoids this by letting a retried job proceed from its pending state (phase-02.md:857). | Section 3.2 job step 1 (phase-07.md:523) against section 4 (phase-07.md:798-799) and the start method (phase-07.md:386) | greps: `grep -n -i retr docs/plan/phase-07.md` gives 527, 798 and 803 only (803 is the watcher); no line says how a running check run is resumed.
GAP V-07-08-b | MEDIUM | T7 | The candidate rule disagrees with the Cedar grant. Step 2 allows a candidate "only for M" (phase-07.md:509-510). The Cedar policy goaml.watch_runners lets CFG use goaml_check.use_candidate (phase-07.md:767), and the action table says M, CFG (phase-07.md:736). No route row or step names this action. | Section 3.2 step 2 (phase-07.md:509-510), or the action table and Cedar excerpt (phase-07.md:736, :767) | greps: `grep -n use_candidate docs/plan/phase-07.md` gives 736 and 767 only; no route row or step in sections 2-4 uses it.
GAP V-07-08-c | MEDIUM | T3 | The job's write types have no field lists. NewCheckRun is named at phase-07.md:357 and :385, NewFinding at :358 and :387, and FileResult and CheckSummary at :388. FileResult's fields appear only in the column comment (phase-07.md:212-213). CheckSummary is described only as "totals by layer and severity, grades" (phase-07.md:217). | Section 2 create structs and GoamlCheckRepository (phase-07.md:357-358, :385-388), with section 1 comments | greps: `grep -n 'FileResult\|CheckSummary\|NewFinding\|NewCheckRun' docs/plan/phase-07.md` gives 357, 358, 385, 387 and 388 only; no struct of these names in phase-00, -02, -04 or -21.
GAP V-07-08-d | MEDIUM | T6 | Phase 02 says the stage_transient types of n and stream are "those defined" in phase 07 section 3.2 (phase-02.md:1192-1193). Section 3.2 step 4 calls stage_transient with n (phase-07.md:515) and defines no type for n or for the stream. Section 4 gives only the call form (phase-07.md:849). | Section 3.2 step 4 (phase-07.md:515-518), as phase 02 expects | greps: `grep -n stream docs/plan/phase-07.md` gives 460, 515, 535 and 849, and none defines a type; phase-02.md:1194-1197 gives the signatures without types.

### V-07-09 7.1 CheckService::validate_built   (source: 7.1, phase-07.md:963-968; result: GAPS 2)
- T1 SQL: N/A - the method writes no row ("It writes no run row, stages nothing and stores nothing", phase-07.md:966-967).
- T2 Route: N/A - no route; "The upload path does not change" (phase-07.md:968).
- T3 Repository and types: GAP - see V-07-09-a (CheckOutcome) and V-07-09-b (ProfileVersion). PASS for the signature (phase-07.md:964-965), Layer (phase-07.md:351) and CheckError (phase-07.md:709).
- T4 Logic: PASS - the block names the job steps it runs: "section 3.2, from the package step on" (phase-07.md:964-966). Those are job steps 4 to 9 (phase-07.md:530-557), which are numbered. The layers argument is the upload set, per phase-21.md:1261.
- T5 Audit: N/A - no mutation here. The caller stores the outcome, and its audit belongs to phase 21.
- T6 Cross-references: PASS - phase-21.md:1261 names CheckService::validate_built(bytes, profile, layers) under section 4.5 item 6, with the same signature. DR-171 (research.md:182) records the same decision.
- T7 Consistency: PASS - "The upload path does not change" (phase-07.md:968) agrees with section 3.2 (phase-07.md:504-520 and :522-566). Skipping the antivirus step (job step 3) follows from "from the package step on".
- Gaps:
GAP V-07-09-a | MEDIUM | T3 | CheckOutcome, the return type that the caller stores, is not defined in phase 07 or in the phases that call it. Its fields are not listed (per-file results, findings, summary). | Section 2, beside the value objects (phase-07.md:347-355), and the signature at phase-07.md:964-965 | greps: `grep -n CheckOutcome` gives phase-07.md:964 only; phase-21.md, phase-02.md and phase-04.md have no match.
GAP V-07-09-b | LOW | T3 | The second parameter is typed ProfileVersion (phase-07.md:964), which no section defines. The entity is GoamlProfile (phase-07.md:344), and the value object is ProfileRef (phase-07.md:347). | Signature at phase-07.md:964-965 | greps: `grep -n ProfileVersion docs/plan/phase-07.md` gives 356, 364 (inside NewProfileVersion) and 964 only; no type of that name.

### V-07-10 7.2 rule NAME-01   (source: 7.2, phase-07.md:976; result: GAPS 1)
- T1 SQL: PASS - the rule_code CHECK on findings (phase-07.md:239) accepts NAME-01, and params is a JSON object (phase-07.md:246). No new column is needed.
- T2 Route: N/A - a rule. Its findings are read through GET /goaml/check-runs/{id}/findings (phase-07.md:447).
- T3 Repository and types: PASS - the parameter removed_count is named in the row (phase-07.md:976) and defined by its emitter (phase-21.md:800).
- T4 Logic: PASS by reference - phase 21's name split raises the finding: "each removal is a warn finding NAME-01 (new in phase 21) with params.removed_count" (phase-21.md:800, section 3.3). The check needs the source name, which only the builder holds (see T7).
- T5 Audit: N/A - a finding inside a run. The run's audit is goaml_check.completed (phase-07.md:561-562), and report events belong to phase 21.
- T6 Cross-references: PASS - phase-21.md:800 (section 3.3). DR-368 says "NAME-01 warn as phase 21 section 3.3 defines" (research.md:379).
- T7 Consistency: GAP - see V-07-10-a.
- Gaps:
GAP V-07-10-a | LOW | T7 | Section 3.3 says catalogue rules "are Rust functions in akr-goaml" (phase-07.md:577), and 7.2 adds NAME-01 to that catalogue (phase-07.md:970-972). Its only emitter is the phase 21 builder (phase-21.md:800). Its condition cannot be computed from an uploaded file, because the upload has no record of which characters were removed. Section 7.2 does not say that the upload path never raises NAME-01, so a profile can enable a code the checker never produces. | Section 7.2 row NAME-01 (phase-07.md:976) against section 3.3 (phase-07.md:577) | greps: `grep -n 'NAME-0' docs/plan/phase-07.md` gives line 976 only; no akr-goaml function or section 3.2 step names NAME-01.

### V-07-11 7.2 rule PKG-07   (source: 7.2, phase-07.md:979; result: PASS)
- T1 SQL: PASS - the "pending" state is admin.av_status, whose values include 'pending' (phase-02.md:350). The findings table accepts rule PKG-07 (phase-07.md:239) with severity block (phase-07.md:240).
- T2 Route: N/A - a rule, read through GET /goaml/check-runs/{id}/findings (phase-07.md:447).
- T3 Repository and types: PASS - the row has no parameters (phase-07.md:979). Its input is the attachment's phase 02 status, given by phase 21 (phase-21.md:817).
- T4 Logic: PASS by reference - "An attachment whose phase 02 status is still `pending` is a block `PKG-07`" (phase-21.md:817, section 3.3 L5). On the upload path the check never fires: each attachment is scanned synchronously in job step 4 ("each attachment scanned by clamd", phase-07.md:533) and no evidence row exists (phase-07.md:517), which fits phase 21 owning the rule.
- T5 Audit: N/A - a finding inside a run (audited by goaml_check.completed, phase-07.md:561-562).
- T6 Cross-references: PASS - admin.av_status with 'pending' (phase-02.md:350); phase-21.md:817 defines the trigger; PKG-07 is listed in phase-21.md:1261 and DR-171 (research.md:182).
- T7 Consistency: PASS - the 7.2 header says these codes are phase 21's (phase-07.md:970), and the severity (block) matches phase-21.md:817.
- Gaps: none.

### V-07-12 7.2 rule CTR-02   (source: 7.2, phase-07.md:981; result: GAPS 2)
- T1 SQL: N/A - a rule. The finding columns hold it (phase-07.md:237-246).
- T2 Route: N/A - a rule, read through GET /goaml/check-runs/{id}/findings (phase-07.md:447).
- T3 Repository and types: GAP - see V-07-12-a. The row names no parameter. The only domestic list is BR-13's parameter `domestic` [IQ] (phase-07.md:596). The row does not name the two country elements of a CTR transaction or the report code that makes a file a CTR.
- T4 Logic: PASS - the condition is one test per CTR transaction (both countries domestic), with severity block (phase-07.md:981). The layer is not named in the row; the findings CHECK accepts "rule" for the L2 rules (phase-07.md:237-238).
- T5 Audit: N/A - a finding inside a run.
- T6 Cross-references: PASS - phase-21.md:951 defines CTR-02 as "both countries domestic", and phase-21.md:1343 gives it block severity. DR-368 (research.md:379) agrees.
- T7 Consistency: GAP - see V-07-12-b.
- Gaps:
GAP V-07-12-a | MEDIUM | T3 | CTR-02 has no parameter for its domestic test. The only domestic list is the parameter `domestic` [IQ] of BR-13 (phase-07.md:596), and phase 21 uses "domestic" without a list (phase-21.md:951). The row does not name the country elements of a transaction or the report code that makes a file a CTR (report_code is recorded at phase-07.md:559). The profile jurisdiction column (phase-07.md:59) could supply IQ, but no text says so. | Section 3.3 CTR-02 row (phase-07.md:981), with BR-13's parameter (phase-07.md:596) | greps: `grep -n domestic docs/plan/phase-07.md` gives 596 and 981 only; `grep -n domestic docs/plan/phase-21.md` gives line 951 only.
GAP V-07-12-b | MEDIUM | T7 | CTR-02 and BR-13 test the same thing at different severities. BR-13 is warn for "cash mode on one side and an account on the other, both countries domestic" (phase-07.md:596). CTR-01 and CTR-02 are block, and CTR-02 is "not both domestic" (phase-07.md:980-981). The trailer says BR-13 "only checks the shape of an uploaded CTR" (phase-07.md:990). An uploaded CTR with a foreign country therefore gets a warn and a block for one fault, and no row says which one stands. Phase 21 does not mention BR-13. | Section 3.3 BR-13 row (phase-07.md:596) against the 7.2 rows (phase-07.md:980-981) and the trailer (phase-07.md:990) | greps: `grep -n BR-13 docs/plan/phase-07.md` gives 596 and 990 only; `grep -n BR-13 docs/plan/phase-21.md` gives nothing; DR-368 (research.md:379) does not mention BR-13.

## Summary

- Items checked: 12 (8 header items, 4 gap-pass items from 7.1 and 7.2, 0 adversarial items: phase 07 has no "Adversarial pass" block).
- Items fully PASS: 1 (V-07-11).
- GAP lines: 21 in total. HIGH 2, MEDIUM 14, LOW 5.

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-07-01-a | MEDIUM | T2 | GET /goaml/profiles and GET /goaml/profiles/{id} do not admit CHK (no config.approve), but CHK reads profiles per Cedar and section 6 | Route rows phase-07.md:448-449 (API.md:578-579) |
| V-07-01-b | MEDIUM | T4 | ProfileLoader refusals (DOCTYPE, unknown rule code, weights or bands) name no error | Section 3.1 steps 1 and 4; section 3.8 |
| V-07-01-c | MEDIUM | T3 | NewProfileVersion has no field list | Section 2, create structs (phase-07.md:356) |
| V-07-01-d | LOW | T7 | lookups and schema_summary can be updated after activation, but content is said never to change | Section 1 trigger and GRANT (phase-07.md:115-121, :167) |
| V-07-02-a | MEDIUM | T3 | NewWatchRun and WatchEnd have no field lists | Section 2 (phase-07.md:358, :377-378) |
| V-07-02-b | LOW | T7 | Action table omits CHK for goaml_watch.read; Cedar grants it | Section 3.8 table (phase-07.md:738) against Cedar (:763) |
| V-07-03-a | MEDIUM | T4 | No refusal for a candidate without corpus_result; no outcome when goaml_corpus fails twice | Section 3.6 Prepare (phase-07.md:686); section 4 (:804-805) |
| V-07-03-b | MEDIUM | T6 | FIU-answers corpus is said to come from phase 21, which does not define it | Section 3.3 (phase-07.md:615-617); phase-21.md |
| V-07-04-a | LOW | T7 | open_questions can change on an active version, but content is said to be frozen | Section 1 trigger and GRANT (phase-07.md:115-121, :168) |
| V-07-05-a | MEDIUM | T3 | Rule parameter schema (goaml-profile.v1.json) is required but not written | Section 3.1 step 4 (phase-07.md:480-481); section 2 (:324) |
| V-07-06-a | MEDIUM | T4 | No 422 reason for the 20,000-character rejection_text limit | Section 3.2 step 2 (phase-07.md:508, :511-512); section 3.8 (:707-708) |
| V-07-07-a | HIGH | T1 | No storage, field or route for the cases a profile amendment adds (RP-83) | Section 1; section 2 (phase-07.md:356-357); section 3.6 (:686) |
| V-07-08-a | HIGH | T7 | Retry contradicts job step 1 ("a run not queued -> done"); the fail-closed path cannot complete | Section 3.2 job step 1 (phase-07.md:523) against section 4 (:798-799) |
| V-07-08-b | MEDIUM | T7 | Candidate use: step 2 allows M only; Cedar and the action table also give CFG | Section 3.2 step 2 (phase-07.md:509-510); Cedar (:767) |
| V-07-08-c | MEDIUM | T3 | NewCheckRun, NewFinding, FileResult and CheckSummary have no field lists | Section 2 (phase-07.md:357-358, :385-388) |
| V-07-08-d | MEDIUM | T6 | stage_transient types of n and stream: phase 02 points to phase 07 section 3.2, which does not define them | Section 3.2 step 4 (phase-07.md:515-518); phase-02.md:1192-1193 |
| V-07-09-a | MEDIUM | T3 | CheckOutcome, the return type the caller stores, is not defined | Section 2 and the signature (phase-07.md:964-965) |
| V-07-09-b | LOW | T3 | Parameter typed ProfileVersion, which is not a defined type (the entity is GoamlProfile) | Signature (phase-07.md:964) |
| V-07-10-a | LOW | T7 | NAME-01 is emitted only by the phase 21 builder; 3.3 says catalogue rules are akr-goaml functions | Section 7.2 row (phase-07.md:976) against section 3.3 (:577) |
| V-07-12-a | MEDIUM | T3 | CTR-02 names no domestic-country list and no country elements | Section 3.3 CTR-02 row (phase-07.md:981), BR-13 parameter (:596) |
| V-07-12-b | MEDIUM | T7 | CTR-02 and BR-13 test the same shape and countries at warn and block; no replacement stated | Section 3.3 BR-13 (phase-07.md:596) against 7.2 (:980-981) |

## Notes (not gaps)

- Phase 07 has no "## Decisions" or "## Open doubts" section (grep -i finds neither). The b3 conventions (tools/agent/b3-conventions.md:10) and this pass's own layout list both, before the trailer; planning.md's template does not name them. This is structure, so it is not counted; the lead decides.
- Outside this phase's file: docs/plan/phase-01.md:1078 cites "phase 07 section 4.5" for the a2b@bank.test fixture, but phase 07 has no section 4.5 (the fixture is a bullet in section 4).
- Outside this phase's file: docs/plan/phase-21.md:1261 cites phase 07 "section 3.4" for the pipeline that validate_built runs; the pipeline is section 3.2 (3.4 is the score).
- Outside this phase's file: docs/plan/phase-21.md:1261 lists CONS-04 among the codes "added"; phase-07.md:984 and phase-21.md:1342 say CONS-04 already exists.
- Stale note in phase 07: phase-07.md:454-455 says API.md lists the profile and watch GETs under report.prepare only, but API.md:578-581 already lists config.author too.
- Cedar tag: phase-07.md:758 uses @deny("masked"), which is not one of the deny tags phase 03 defines (phase-03.md:704: "out_of_scope" | "tier"). This is outside the sampled items; the lead decides whether to add it to phase 03.
