# Owner-decision round, phase 07 (Haiku draft for lead review)

### O-07-1  HIGH   (section 7.6 item 7 `check_before_decision` and item 14 verification, check 6)
- Problem: The `check_before_decision` row enqueues `goaml_case_intake` inside the decision transaction and then issues `intake_stale`, which is a refusal. Phase 04 rolls back the whole action on a refusal, and a job enqueued in a rolled-back transaction leaves no row. The job never runs, so the approver cannot get a fresh intake, and the stated check "then approval succeeds" cannot pass.
- Proof: docs/plan/phase-07.md:1185 "`intake_result.profile_version` differs from the active version's -> enqueue `goaml_case_intake {case_id}` again and issue `intake_stale`"; docs/plan/phase-04.md:1649 "refusal rolls the action back and the task stays open."; docs/plan/phase-00.md:788 "enqueue inside a rolled-back transaction leaves no row"; docs/plan/phase-07.md:1226 "after a profile activation -> 422 `intake_stale` and a new `goaml_case_intake` job, then approval succeeds".
- Smallest fix: In line 1185 delete "enqueue `goaml_case_intake {case_id}` again and", and append to the `goaml_profile` Activate row (section 3.6, line 691) "and enqueue `goaml_case_intake {case_id}` for each proposed case of the code", so the job commits with the profile activation before any approver retries.

### O-07-2  HIGH   (section 7.6 item 14, curl check, check 6)
- Problem: The curl check expects `403 E_SCOPE` for DS on the GET. The API.md scope table gives DS the `config.author` scope, and the route accepts `config.author`, so DS passes gate 1. Cedar then denies DS, which phase 03 maps to `E_FORBIDDEN`. The stated check fails as written.
- Proof: docs/plan/phase-07.md:1228 "as DS the GET -> 403 `E_SCOPE`"; docs/API.md:95 "`config.author` ... CFG, DS (mappings and code maps), MRO (models)"; docs/plan/phase-07.md:1128 "GET | /goaml/corpus-cases | report.prepare, config.author, config.approve"; docs/plan/phase-03.md:1016 "`E_FORBIDDEN` (Cedar deny on a resource the caller may".
- Smallest fix: In line 1228 replace "as DS the GET -> 403 `E_SCOPE`" with "as DS the GET -> 403 `E_FORBIDDEN` (gate 1 passes on `config.author`; Cedar denies)".

### O-07-3  MEDIUM   (section 7.6 item 14, curl check, check 6)
- Problem: The curl sequence approves the case right after the POST, with no wait for the `goaml_case_intake` job. Until that job has run, `check_before_decision` issues `intake_not_run` (422), so "the case `active`" is not reached by the sequence as written.
- Proof: docs/plan/phase-07.md:1226 "a decision before the intake ran -> 422 `intake_not_run`"; docs/plan/phase-07.md:1228 "as CHK approve -> the case `active`".
- Smallest fix: In line 1228 insert "after the `goaml_case_intake` job has completed (poll until `intake_result` is set)" before "as CHK approve".

### O-07-4  MEDIUM   (section 7.6 item 6 steps 1 and 8, check 2)
- Problem: Step 1 ends the job for any case that is no longer `proposed`, and step 8 deletes the uploaded file only after a first run's transaction. A case withdrawn before its intake runs keeps its uploaded file in the transient store until the sweep (`evidence.staging_ttl_h`, 24 hours by default), which contradicts the text's statement that the file lives only until the end of the first intake run.
- Proof: docs/plan/phase-07.md:1165 "A state other than `proposed` -> done."; docs/plan/phase-07.md:1174 "The uploaded file therefore lives only sealed in the transient store, from the request to the end of the first intake run."; docs/plan/phase-02.md:518 "'evidence.staging_ttl_h'".
- Smallest fix: In step 1 (line 1165) replace "A state other than `proposed` -> done." with "A state other than `proposed` -> `delete_transient("goaml", case_id)` and done."

### O-07-5  MEDIUM   (section 7.6 item 6 step 8 and item 9, check 6)
- Problem: The deletion of a case's uploaded file (step 8, and the sweep's deletion of an object left by a crash) changes stored data, but no audit event names it. Section 4 gives the same deletion for check runs the event `goaml_check.transient_deleted`; item 9 lists no equivalent for cases.
- Proof: docs/plan/phase-07.md:1172 "An object left by a crash is deleted by `maintenance_sweep` after `evidence.staging_ttl_h`"; docs/plan/phase-07.md:1195 "Reused: `change_request.*` (phase 04), `evidence.stored` (phase 02)."; docs/plan/phase-07.md:819 "the run flagged `transient_deleted` (audit `goaml_check.transient_deleted` with".
- Smallest fix: Add to item 9 (line 1195) the kind "`goaml_corpus_case.transient_deleted {case_id, by}`" and emit it after each `delete_transient` in item 6 step 8 and for objects deleted by the sweep.

### O-07-6  MEDIUM   (section 7.6 item 12 tipping-off review, check 6)
- Problem: Item 12 says a key that names a person is refused at approval, "as any reason text is". No step refuses it: the Prepare and `check_before_decision` rows check no key content, phase 04 checks `reason_text` only for length, and the key pattern admits names. The refusal has no step, status or code, so a developer cannot build it, and the tipping-off answer relies on it.
- Proof: docs/plan/phase-07.md:1218 "so a key that names a person is refused at approval, as any reason text is"; docs/plan/phase-07.md:1025 "case_key ~ '^[a-z][a-z0-9-]{2,60}$'"; docs/plan/phase-04.md:134 "TEXT NOT NULL CHECK (length(btrim(reason_text)) BETWEEN 1 AND 2000)".
- Smallest fix: In item 12 replace "so a key that names a person is refused at approval, as any reason text is" with "and nothing in this phase screens the key for names, so the form asks for a code, not a name".

### O-07-7  MEDIUM   (section 7.6 item 2 mask table, check 2)
- Problem: The mask maps only U+0621 to U+063A and U+0641 to U+064A to Arabic letters of the same range. The entity-name class that patch P-01 applies (U+0621 to U+064A) also accepts U+063B to U+0640, and those letters are masked to U+06A9, which the class rejects. A valid name can gain an XSD-04 block after masking, which breaks the claim that the mask keeps letter class.
- Proof: docs/plan/phase-07.md:479 "the entity `name` pattern `^[a-zA-Zء-ي\s'&-]*"; docs/plan/phase-07.md:1117 "letter in U+0621 to U+063A or U+0641 to U+064A | a random letter of those two ranges"; docs/plan/phase-07.md:1122 "The mask keeps what the rules look at (length, letter class, digit script".
- Smallest fix: In the table row of line 1117 replace "letter in U+0621 to U+063A or U+0641 to U+064A | a random letter of those two ranges" with "letter in U+0621 to U+064A | a random letter of that range".

### O-07-8  MEDIUM   (base section 3.8 tipping-off review and base section 3.2 "What is kept", check 2)
- Problem: Base section 3.8 says nothing derived from the uploaded files holds a value, a name or the narrative, and base section 3.2 says never a value from the file. Item 2 keeps dates, enumeration codes, booleans and the bank's identifiers in the stored masked copy, which is derived from the upload. Neither base sentence is replaced or excepted by a 7.N block, so the phase states two rules for the same thing.
- Proof: docs/plan/phase-07.md:789 "nothing derived from them holds a value, a name or the narrative"; docs/plan/phase-07.md:572 "Never a value from the file, never the libxml2 message"; docs/plan/phase-07.md:1122 "What stays in a stored case is the structure, enumeration codes, booleans, dates and date-times".
- Smallest fix: In section 3.8 (line 789) insert "except the masked bank cases of section 7.6" after "nothing derived from them holds a value, a name or the narrative", and add the same exception to the sentence at line 572.

### O-07-9  LOW   (section 7.6 items 1 and 7, check 4)
- Problem: Item 7 Prepare accepts any `rule_code` of an active rejection pattern, but the table CHECK admits only `OFF-nn`. Base section 3.7 does not restrict pattern codes to OFF-, so an amendment that names a pattern with another code passes Prepare and then fails the insert at the database instead of returning a 422.
- Proof: docs/plan/phase-07.md:1028 "office_code ~ '^OFF-[0-9]{2,3}$'"; docs/plan/phase-07.md:1183 "`office_code`, when given, is the `rule_code` of a rejection pattern of the active version"; docs/plan/phase-07.md:79 "[{id, pattern, rule_code, element?, message_key}]".
- Smallest fix: In item 7 Prepare (line 1183) replace "is the `rule_code` of a rejection pattern of the active version" with "is the `rule_code` of a rejection pattern of the active version and matches `OFF-nn`".

### O-07-10  LOW   (section 7.6 item 7 Activate, check 3)
- Problem: The Activate row says a target refusal at activation ends the request `failed`. Phase 04 aborts an immediate final approval whose terminal effect fails: the approval is rolled back and the error is returned. Only the scheduled `cr_activate` path ends the request `failed`.
- Proof: docs/plan/phase-07.md:1186 "the target refuses and the request ends `failed`"; docs/plan/phase-04.md:1023 "An effect error aborts the whole action (the approval is rolled back and answered"; docs/plan/phase-04.md:1103 "a stale base or a target refusal -> `failed`".
- Smallest fix: In the Activate row (line 1186) replace "the target refuses and the request ends `failed`" with "the target refuses; an immediate approval is rolled back with the error (phase 04 section 3.4 step 9), and a scheduled activation ends `failed`".

### O-07-11  LOW   (base section 2 crate placement, check 2)
- Problem: The crate placement in section 2 still lists three job kinds, one change target and "the three job handlers". Section 4 and the header now list `goaml_case_intake`, the change target `goaml_corpus_case` and a fourth handler, so the same listing states different numbers.
- Proof: docs/plan/phase-07.md:334 "# + kinds goaml_check, goaml_watch, goaml_corpus (akr-worker)"; docs/plan/phase-07.md:336 "# + router goaml; change target goaml_profile"; docs/plan/phase-07.md:337 "# + the three job handlers, SandboxedXsdValidator, ProxySchemaFetcher"; docs/plan/phase-07.md:834 "and the fourth handler `goaml_case_intake`".
- Smallest fix: In lines 334, 336 and 337 add `goaml_case_intake`, add `goaml_corpus_case`, and change "the three job handlers" to "the four job handlers" respectively.

## Summary

- Blocks checked: 7.6 (items 1 to 14, including the table SQL of item 1), the 7.5 owner note and the 7.3 replacement paragraph. Changed base places checked: 23 (every changed hunk of the diff outside the new 7.5 note and the 7.6 insert).
- Findings by severity: HIGH 2, MEDIUM 6, LOW 3 (11 in total).
- CHECKED: 7.3 (replacement paragraph) checks 1 to 6 ok.
- CHECKED: 7.5 (owner note) checks 1 to 6 ok.
- CHECKED: 7.6 checks 1 to 6: findings O-07-1 to O-07-10. O-07-11 is in base section 2, which the new block contradicts.
- Checked without a finding: the rpt.goaml_corpus_cases SQL (17 columns, each named once in the classification rows; every CHECK is met by the states the steps write; the column-level UPDATE grant covers every column the steps update; the partial unique index matches the rule; the retention row; the guard trigger); the 12 routes (9 + 3) and the 133 operations (121 + 12); the three API.md rows; phase 08 section 7.11; the phase 04 names (ChangeTarget members, create(ctx, req, origin), cr_minor, cr_material, the route map, section 7.17); the phase 02 names (stage_transient, open_transient, delete_transient, put_generated and GeneratedEvidence, open, the registry row, evidence.staging_ttl_h); the phase 00 names (secret-str, internal, the column_classification and retention_assignment keys).
