# Verification pass, phase 28 (Haiku draft for lead review)

Sample (12 items). Phase 28 has no "Adversarial pass" blocks (grep finds none), so 3 extra header items replace them. Its gap additions are 7.1, 7.2 and 7.3 only, so 7.1 to 7.3 are taken and 1 header item fills the fourth 7.x slot.
- Header 1 (start): V-28-01 M-70 task registry, switches, kill switch, per-task gate
- Header 2 (early): V-28-02 M-73 golden evaluation gate and monitoring panel
- Header 3 (middle): V-28-03 SP-95 per-task and global kill switch
- Header 4 (middle-late): V-28-04 GV-46 provenance per draft
- Header 5 (end): V-28-05 IR-23 labelled draft, saving needs an edit or accept
- 7.x 1: V-28-06 7.1 KV paths
- 7.x 2: V-28-07 7.2 retention DELETE grant (DR-386)
- 7.x 3: V-28-08 7.3 release bundles import no assistant version (DR-390)
- 7.x fill (header): V-28-09 TS-17 no external AI API, runtime optional on premises
- Extra header 6: V-28-10 GV-47 grounding and verification (GroundingVerifier)
- Extra header 7: V-28-11 GV-51 model supplier checklist fields in the signed manifest
- Extra header 8: V-28-12 GV-49 delimited data, timeouts, kill switch, action log

### V-28-01 M-70 task registry, switches, kill switch, per-task gate   (source: header "Implemented here"; result: PASS)
- T1 SQL: PASS - switch keys seeded at phase-28.md:327-330; kill rows table at :97-108; live versions table at :34; gate rows table at :79; pack flag at :321.
- T2 Route: PASS - GET /assistant/tasks at :429 (`model.risk, audit.read, dashboard.read`); POST /assistant/kill-switch at :432; release requests at :433; permits at :629-646; API.md rows at docs/API.md:700 and :703-704.
- T3 Repository and types: PASS - `AssistantTask` enum at :395; `AssistantKillRepository` lock, lock_shared, set_engaged, all at :414; `AssistantVersionRepository.live_for` at :412; `AssistantEvaluationRepository.latest_passed` at :413; `EngageAssistantKill` and `ReleaseAssistantKillRequest` at :409; `TaskView` and `KillStateView` at :438. Note: AssistantKillSwitchService and AssistantReads name no methods, but the steps call named repository methods, so no GAP.
- T4 Logic: PASS - task enum refusal 422 `E_VALIDATION` at `/task` (:395, :450); `task_disabled` 409 (:452); `not_live` 409 (:453); kill engage and release steps with errors (:570-575); `evaluation_missing` and `gate_not_met` 409 at :594.
- T5 Audit: PASS - `assistant.kill_engaged` at :570 and table :689; `assistant.kill_release_requested` in table :690; `assistant.kill_released` at :573 and table :691. The per-task switch is a governed setting: phase 04 writes the setting's own `setting.updated` audit event on activation (phase-04.md:915), and phase 11 routes governed setting changes to `cr_material` (phase-11.md:1373).
- T6 Cross-references: PASS - `StepUpService::consume` at phase-01.md:690 and class `approval` at phase-01.md:709; `cr_material` and `cr_material_validated` routes at phase-04.md:1265-1266; `ChangeTargetRegistry` at phase-04.md:123; `admin.feature_flags` at phase-00.md:217; flag-off 404 pattern at phase-26.md:360; kill switch immediate and two-person release at research.md:277 (DR-266).
- T7 Consistency: PASS - no 7.x block changes the registry, the switches or the kill switch (7.1 is path notation, 7.2 grants, 7.3 import).

### V-28-02 M-73 golden evaluation gate, staged release, monitoring panel   (source: header "Implemented here"; result: GAPS 3)
- T1 SQL: PASS - `admin.assistant_evaluations` columns (task, version ids, run_id, golden_set_sha256, result, passed, data_as_of) at phase-28.md:79-90; stage uniqueness at :94; gate settings at :331-336; `schedule.assistant_monitor` at :349; monitoring reads `casework.assistant_drafts` columns at :134-158.
- T2 Route: PASS - POST /assistant/evaluations at :427, GET /assistant/evaluations at :428, GET /assistant/monitoring at :434; Cedar `assistant.evaluation.run`, `assistant.evaluation.read` for MRO at :630-633; `assistant.monitor.read` at :636-646; API.md rows at docs/API.md:698-699 and :705.
- T3 Repository and types: PASS - `AssistantEvaluationRepository` insert, latest_passed, list at :413; `AssistantDraftRepository.counts(conn, task, window, mode)` at :411; `GoldenSet::build(task)` at :540; `RequestEvaluation` at :408; `EvaluationView`, `EvaluationAccepted`, `MonitoringView`, `MonitorCountsView` at :438.
- T4 Logic: GAP - see GAP V-28-02-a. Other steps PASS: route enqueue on "both versions registered or live" at :534 (refusal 409 `not_registered` is listed at :660); golden set steps 1-4 at :541-544; gates at :548-553; two stages at :555.
- T5 Audit: PASS - `assistant.evaluation_run` and `assistant.evaluation_recorded` in table :684-685; `assistant.monitor_breach` at :688; reads write no audit row (:693).
- T6 Cross-references: GAP - see GAP V-28-02-b and GAP V-28-02-c. PASS for the rest: `pass`, `pass_with_comment`, `critical` in phase-19.md:409; populations `closed_cases` and `targeted_repeat` at phase-19.md:368-370; `POST /qa/samples` and `GET /qa/samples/{id}/item` at phase-19.md:1043-1044; `filed` status at phase-21.md:84; `report.small_number_threshold` at phase-21.md:518; `schedule.model_monitor` at phase-27.md:325; `source_key` on `admin.registers` at phase-24.md:46.
- T7 Consistency: PASS - no 7.x block changes M-73 (7.1 is path notation only).
- Gaps:
GAP V-28-02-a | MEDIUM | T4 | The monitoring counts group refusals into five code groups (kill_or_switch, visibility, model, output, grounding). The refusal code `input_too_large` belongs to none of them, so those refusals are counted in no group. | section 3.7 (line 561), against the code list of section 1.2 (line 140) | grep 'input_too_large' phase-28.md (lines 140 and 468 only); grep 'kill_or_switch' and 'code group' phase-28.md (line 561 only); grep -l input_too_large docs/plan/*.md docs/API.md (phase-28.md only).
GAP V-28-02-b | MEDIUM | T6 | The golden sample is drawn for "the drafts whose eval_run_id is the run", but the draw route takes no run input (population, mode, strata_by, n, window, reviewer_id) and `casework.qa_samples` has no run column, so one run's samples cannot be selected. | section 3.6 step 4 (line 544); phase 19 draw at phase-19.md:1254-1256 and qa_samples DDL at :365-392 | grep -n -i 'eval_run\|run_id\|run id' phase-19.md (no match); no run column in phase-19.md:365-392; section 1.2 adds no column (phase-28.md:130-193).
GAP V-28-02-c | MEDIUM | T6 | Golden items are read "under the time-boxed grant (requirement on phase 19, section 4.5)". Phase 19's grant step covers only source_kind `case` or `decision`, and its section 4.5 requirement names no grant. | section 3.6 step 4 (line 544); phase 19 section 3.8 step 7 (phase-19.md:1261) and the 7.2 block (phase-19.md:1850-1855) | grep -n -i 'assistant' phase-19.md (lines 1850-1855 and 2004 only); grep -n 'time-boxed' phase-19.md (lines 10, 22, 47, 386, 834: none about assistant drafts).

### V-28-03 SP-95 per-task and global kill switch   (source: header "Implemented here"; result: GAPS 1)
- T1 SQL: PASS - `admin.assistant_kill_state` at phase-28.md:97-108 (scope key and CHECK, reason_code CHECK, engaged-needs-reason CHECK at :107); seed rows at :109-113; guard at :245-261; UPDATE grant at :310.
- T2 Route: PASS - POST /assistant/kill-switch at :432 (`model.risk`, Cedar `assistant.kill.engage`); POST /assistant/kill-switch/release-requests at :433 (`assistant.kill.release_request`); MRO permits at :632-633; API.md rows at docs/API.md:703-704.
- T3 Repository and types: GAP - see GAP V-28-03-a (LOW). Repository methods lock, lock_shared, set_engaged, all at :414; step-up constant `AssistantKill` at :570; DTOs at :409; `KillStateView` at :438.
- T4 Logic: PASS - engage steps at :570 (428 without If-Match, 409 on mismatch, step-up 401) and effect at :571; release steps at :573 (`not_engaged` 409, change request `cr_material`, `breach_open` 409, `E_SELF_APPROVAL` 403); lock 1 at :570 and :716.
- T5 Audit: PASS - `assistant.kill_engaged` at :570 (table :689, SIEM true); `assistant.kill_release_requested` in table :690; `assistant.kill_released` at :573 (table :691, SIEM true).
- T6 Cross-references: PASS - `StepUpService::consume` and `StepUpAction { name, class }` at phase-01.md:690 and :427; class `approval` at phase-01.md:709; `cr_material` at phase-04.md:1265; `check_before_decision` at phase-04.md:651; DR-266 at research.md:277; phase 27 kill-reason list at phase-27.md:133; `admin.model_kill_state` at phase-27.md:25.
- T7 Consistency: PASS - no 7.x block changes the kill switch.
- Gaps:
GAP V-28-03-a | LOW | T3 | The engage body has two names: the request DTO is `EngageAssistantKill` (line 409) and the utoipa list names `EngageKillRequest` (line 438). `ReleaseAssistantKillRequest` (line 409) has no utoipa entry. | section 2 utoipa schemas (line 438) | grep 'EngageKillRequest\|ReleaseAssistantKillRequest\|EngageAssistantKill' phase-28.md (lines 409 and 438 only; the release body name is absent from 438).

### V-28-04 GV-46 provenance per draft   (source: header "Implemented here"; result: GAPS 1)
- T1 SQL: PASS - provenance columns of `casework.assistant_drafts`: template_version_id and model_version_id (phase-28.md:141-142); input_sha256 and source_ids (:143-144); output_enc (:146); final_enc, accept_mode, edit_distance, edit_ratio (:149-152); requested_by, requested_at, accepted_by, accepted_at (:156-159).
- T2 Route: GAP - see GAP V-28-04-a. The store is complete, but no route or field returns the provenance.
- T3 Repository and types: PASS - `AssistantDraftRepository` insert_requested, set_generated, set_accepted, get at :411; `AssistantDraft` entity with the provenance fields at :400.
- T4 Logic: PASS - provenance set at request (:459, step 11), at generation (:474, step 12: source_ids, cited_panels, input_sha256) and at accept (:522 step 9 server-side edit measure; :526 step 11).
- T5 Audit: PASS - `assistant.draft_requested` at :676, `assistant.draft_ready` at :677, `assistant.draft_accepted` at :679.
- T6 Cross-references: PASS - `casework.cases` at phase-17.md:290; `rpt.reports` at phase-21.md:74; `admin.users` at phase-01.md:52; sealed text with AAD = id at phase-18.md:70 and :238.
- T7 Consistency: PASS - no 7.x block changes the draft store.
- Gaps:
GAP V-28-04-a | MEDIUM | T2 | Provenance (model and template version ids, source ids, requester and times) is stored, but no route returns it. GET /assistant/drafts/{id} lists state, sentences, counts and scan count; `DraftView` has no provenance field; UX-45 asks for "provenance one click away" (line 12) and section 5.1 has no element for it. | section 2 route row (line 424) and DraftView (line 438); section 5.1 (lines 787-793) | grep -n -i provenance phase-28.md (lines 3, 12, 581, 890, 906 only; none in the route table or section 5); grep -n DraftView phase-28.md (line 438 only).

### V-28-05 IR-23 labelled draft, saving needs an edit or an explicit accept   (source: header "Implemented here"; result: GAPS 1)
- T1 SQL: PASS - `output_enc` at phase-28.md:146, `final_enc` at :149, `accept_mode` at :150; the accepted-state CHECK requires final_enc, accept_mode and accepted_by at :169-172; UPDATE grant at :312-314.
- T2 Route: PASS - GET /assistant/drafts/{id} at :424 and POST /assistant/drafts/{id}/accept at :425 (`queue.work`); Cedar `assistant.draft.read` and `assistant.draft.accept` for A1, A2 and M at :614-627; API.md rows at docs/API.md:695-696.
- T3 Repository and types: PASS - `AcceptDraft` at :405; `VerifiedOutput` and `VerifiedSentence` at :401; `SentenceView`, `AcceptDraftRequest`, `AcceptedView` at :438; `AssistantDraftRepository.set_accepted` at :411.
- T4 Logic: GAP - see GAP V-28-05-a. Other steps PASS: `mode` required and `as_is_mismatch` / `edited_unchanged` at :515; evidence first at :520; `unsupported_sentence_kept` and `forbidden_term` at :521; server edit measure at :522.
- T5 Audit: PASS - `assistant.draft_accepted` at :527 (table :679); the payload carries no text and no distance.
- T6 Cross-references: PASS - `NoteService::post` and `CreateNote` at phase-18.md:674 and :559; `casework.note_max_chars` at phase-18.md:461; `E_CASE_SEALED` at phase-18.md:655; `casework.opened_views` with view_kind 'panel' at phase-18.md:268-272; `ReportService::update_draft` at phase-21.md:1348; `report.edit` at phase-21.md:735; narrative fields and 8,000-character limit at phase-21.md:788; `PolicyPackService::scan` at phase-09.md:797.
- T7 Consistency: PASS - no 7.x block changes accept or discard.
- Gaps:
GAP V-28-05-a | MEDIUM | T4 | The narrative accept takes the draft row (lock 4) at step 6 and the report row (lock 3) at step 10. Section 3.15 says every path takes lock 3 before lock 4, so two writers can take the two rows in opposite orders. | section 3.5 steps 6 and 10 (lines 519, 525) against the order list of section 3.15 (lines 716-722) | grep -n 'lock [0-9]' phase-28.md (the accept path names lock 4 at line 519 and lock 3 at line 525; the order list is at lines 716-722 only); no other lock-order statement covers the accept path.

### V-28-06 7.1 KV paths   (source: 7.x block 7.1; result: PASS)
- T1 SQL: N/A - a naming rule, no table or column.
- T2 Route: N/A - no route in this block.
- T3 Repository and types: N/A - no type in this block.
- T4 Logic: N/A - the block changes only the path notation; the secret reads at phase-28.md:757 and :761 keep their logic.
- T5 Audit: N/A - no mutation.
- T6 Cross-references: PASS - phase-00.md:892 "7.5 KV mount rule"; phase-00.md:893 "One KV v2 mount" (DR-356, mount `akr/`); phase-00.md:894 lists `assistant/<x>` as a path written without a mount prefix.
- T7 Consistency: PASS - the base text uses `secret/assistant/llama` (phase-28.md:757, :761); the block at :867-868 states that `secret/assistant/llama` is `assistant/llama`, which resolves the base wording.

### V-28-07 7.2 retention DELETE grant (DR-386)   (source: 7.x block 7.2; result: GAPS 2)
- T1 SQL: PASS - grants at phase-28.md:317-318 give akr_retention SELECT only, on the four new tables; no DELETE grant exists in section 1.3, which matches the block at :875-876.
- T2 Route: N/A - no route in this block.
- T3 Repository and types: N/A - no type in this block.
- T4 Logic: N/A - a grant only; the purge wording is judged under T7.
- T5 Audit: N/A - no mutation.
- T6 Cross-references: GAP - see GAP V-28-07-b (LOW). PASS for the rest: DR-386 at research.md:397; R6 disposal is report_only under DR-191 at phase-23.md:596 (DR-191 text at phase-23.md:1054); R10 is never disposed at phase-23.md:26; the akr_retention role at phase-23.md:9.
- T7 Consistency: GAP - see GAP V-28-07-a. The block says no handler deletes drafts; section 4.5 and Open doubts 24 still say the engine purges them.
- Gaps:
GAP V-28-07-a | MEDIUM | T7 | Section 4.5 says the phase 23 engine "deletes a case's drafts in its own record_purge" before the case row, and Open doubts 24 says the same. Block 7.2 says no handler deletes these rows in this release (R6 drafts report_only), and phase 23 gives R6 disposal report_only (DR-191). The block does not say it replaces 4.5. The grants at lines 317-318 give no DELETE, so the base purge cannot run. | section 4.5 phase 23 bullet (line 779) and Open doubts 24 (line 947) | grep -n -i 'purge' phase-28.md (lines 779 and 947 require a draft purge; line 876 says R6 drafts are report_only); grep 'GRANT' phase-28.md | grep -i -c delete (0: no DELETE grant in section 1.3); phase-23.md:596 and :1054 give report_only for R6.
GAP V-28-07-b | LOW | T6 | Migration 0090 gives `admin.assistant_versions` the retention class R11 with clock `retirement` (phase-28.md:382). Phase 23 lists R11 with clock `age` only (phase-23.md:601); `retirement` is listed for R9 (phase-23.md:599). | section 1.3 retention rows (line 382) | grep -n 'R11' phase-23.md (line 601 is the only R11 clock row; lines 1012, 1054 and 1085 are disposal text); grep -n retirement phase-23.md (lines 577, 599, 1013, 1023: none for R11).

### V-28-08 7.3 release bundles import no assistant version (DR-390)   (source: 7.x block 7.3; result: PASS)
- T1 SQL: PASS - status `registered` is in the status CHECK at phase-28.md:40 and the guard edge `importing` to `registered` at :231; no new column is needed.
- T2 Route: PASS - POST /assistant/versions at :430 is the version import the block names; API.md row at docs/API.md:701.
- T3 Repository and types: PASS - `ImportVersion` at :407; `ImportAccepted` at :438; `AssistantVersionRepository` insert_importing and set_verified at :412.
- T4 Logic: PASS - model import steps 1-4 and verification (c) end in `registered` (:579-585); template import is inserted as `registered` (:587); live is set only by promotion (:598).
- T5 Audit: PASS - `assistant.version_imported` at table :681 and `assistant.version_verified` at :682 cover the import.
- T6 Cross-references: PASS - phase-25.md:1371-1374 (section 7.4, DR-390: the release imports no model or assistant version); research.md:401 (DR-390); the phase 27 import path at phase-27.md:466 (section 3.3).
- T7 Consistency: PASS - section 4.5 phase 25 bullet (phase-28.md:777) and Open doubts 15 (:938) say the same.

### V-28-09 TS-17 no external AI API, runtime optional and on premises   (source: header "Implemented here"; result: GAPS 1)
- T1 SQL: N/A - no table in this item.
- T2 Route: N/A - the runtime has no route; the API never reaches it (phase-28.md:756).
- T3 Repository and types: PASS - `AssistantModelPort`, `CompletionRequest` and `ModelCallError` at :416; `LlamaServerClient` in crate `crates/llm` at :393; AppState fields `llm` and `llm_eval` at :439.
- T4 Logic: GAP - see GAP V-28-09-a (LOW). Other steps PASS: no egress and the one allowed route `llama:8080` (:756); readiness and fail-closed mismatch (:759, :765); no live version means llama is not started (:759).
- T5 Audit: N/A - the model call writes no audit row of its own; the draft audits are in V-28-02 and V-28-04.
- T6 Cross-references: PASS - phase-00.md:948-951 (llama and assistant-store volume); phase-00.md:714-718 (deploy/zones.yaml, make zone-check); phase-00.md:933 (z2_model members include llama-eval); phase-27.md:772 (model-store volume pattern).
- T7 Consistency: PASS - the block at :867-868 resolves the KV path wording; no other block touches the runtime.
- Gaps:
GAP V-28-09-a | LOW | T4 | Section 4.4 says deploy/zones.yaml "gain these two rows" (line 756), but the zone table adds one row, for llama (line 763). llama-eval is named as a z2_model member (line 756; phase 00 at phase-00.md:933), but no zone row is written for it. | section 4.4 (lines 756 and 763) | grep 'llama-eval' phase-00.md (lines 933 and 948: membership, no zone row); grep -n 'these two rows' phase-28.md (line 756 only, no second row named).

### V-28-10 GV-47 grounding and verification (GroundingVerifier)   (source: header "Implemented here"; result: PASS)
- T1 SQL: PASS - `grounding_result` JSONB at phase-28.md:147 and `scan_hits` at :148 on `casework.assistant_drafts`.
- T2 Route: N/A - the verifier is a pure function run inside the job (step 9 at :471); no route.
- T3 Repository and types: PASS - `GroundingVerifier::verify(output, records)` at :487; `VerifiedOutput`, `VerifiedSentence`, `GroundingResult` at :401-402; output shape at :480.
- T4 Logic: PASS - numbered steps 1-9 at :489-497 with reason codes (no_source, unknown_ref, lang_not_requested, unknown_placeholder, source_incomplete, unsupported_figure, unsupported_date, unsupported_name, over_limit); zero kept gives `nothing_supported` at :471. Note, not a GAP: "any token 2 to 4 long" (:491) does not say whether words or characters.
- T5 Audit: N/A - the verifier writes nothing; its counts reach `assistant.draft_ready` at :677.
- T6 Cross-references: PASS - `text::fold` at phase-09.md:789; `PolicyPackService::scan(scope, text)` at phase-09.md:797 and scope `reason_text` at :749; `TippingOffTerms` at phase-04.md:1242 (port empty until phase 09 ships the term list); DR-279 at research.md:290.
- T7 Consistency: PASS - the fixture list at phase-28.md:830 matches steps 1-5 of section 3.3.

### V-28-11 GV-51 model supplier checklist fields in the signed manifest   (source: header "Implemented here"; result: GAPS 1)
- T1 SQL: PASS - `manifest` JSONB with object CHECK at phase-28.md:49; `manifest_sha256` at :50; `signature` at :51; `signing_key_version` at :52. The GV-51 field names are JSON keys checked in code at :581.
- T2 Route: PASS - POST /assistant/versions at :430; a bad signature or manifest is 422 `E_VALIDATION` at :430 and :581; API.md row at docs/API.md:701.
- T3 Repository and types: PASS - `ImportVersion` with `manifest: Value` at :407; `ImportAccepted` at :438; `AssistantVersionRepository.insert_importing` at :412; `InventoryService::create` at phase-04.md:1337.
- T4 Logic: GAP - see GAP V-28-11-a (LOW). Other steps PASS: manifest hash and required fields at :581; Ed25519 check, nothing written on failure, at :582; inventory item and version row in one transaction at :583.
- T5 Audit: PASS - `assistant.version_imported` at table :681; `assistant.version_refused` (SIEM true) at :683.
- T6 Cross-references: PASS - `SignatureVerifier` and `ArtifactStore::read_sha256` at phase-27.md:398-399; keys at `keys/model-artifact/v<N>.pub` at phase-27.md:399; the seven tiering keys of `inventory.tier_basis` at phase-27.md:436 (section 3.1, heading at :433); `InventoryService::create` at phase-04.md:1337.
- T7 Consistency: PASS - block 7.3 at phase-28.md:879-880 does not change the manifest fields.
- Gaps:
GAP V-28-11-a | LOW | T4 | Step 2 hashes "the manifest's canonical bytes" and step 3 verifies the signature "over the canonical manifest", but phase 28 gives no canonical form and cites no rule. Phase 27 defines one (RFC 8785 style, section 3.3 step 2). | section 3.9 steps 2 and 3 (lines 581-582) | grep -n -i canonical phase-28.md (lines 468, 490, 830 use the word in other senses; 581 and 582 are the only manifest uses); grep -c 'RFC 8785' phase-28.md (0); grep -n -i canonical phase-27.md names the rule at phase-27.md:470 and :471.

### V-28-12 GV-49 delimited data, timeouts, kill switch, action log   (source: header "Implemented here"; result: GAPS 1)
- T1 SQL: PASS - settings `assistant.max_input_chars` at phase-28.md:343 and `assistant.generate_timeout_s` at :345; kill rows read `FOR SHARE` at :454 and :464.
- T2 Route: PASS - POST /assistant/drafts at :423 enqueues the job; the 202 and polling step at :460; the job is named at :462.
- T3 Repository and types: PASS - `AssistantGenerator::run(draft_id)` at :462; `PromptBuilder::retrieve` and `PromptBuilder::build` at :467-468; `CompletionRequest` with timeout and `ModelCallError` at :416.
- T4 Logic: GAP - see GAP V-28-12-a (MEDIUM). Other steps PASS: job steps 1-13 at :463-475 with kill re-checks at :464 and :473; a nonce per request at :479; decoding at :481; second attempt for `model_unavailable` at :469 and :738.
- T5 Audit: PASS - `assistant.draft_requested` at :676, `assistant.draft_ready` at :677 and `assistant.draft_refused` at :678; payloads hold codes and counts only (:693).
- T6 Cross-references: PASS - coalesce key pattern at phase-00.md:549; `PolicyPackService::scan(scope = "reason_text")` at phase-09.md:797; `admin.settings` at phase-00.md:203.
- T7 Consistency: PASS - the data-block fixture at :831 depends on the escape rule (see GAP V-28-12-a); no 7.x block changes the job.
- Gaps:
GAP V-28-12-a | MEDIUM | T4 | The escape rule for a record that contains `<<` or the nonce reads "every `<` replaced by `<`". The replacement is the same character, so the rule changes nothing and the escape that stops a record closing its block is undefined. The fixture at line 831 cannot be built from the text. | section 3.2 data blocks (line 479) | grep 'replaced by' phase-28.md (line 479 only); grep -c '&lt;' phase-28.md (0); grep -n -i escape phase-28.md (line 831 only, no replacement value).

## Summary

- Items checked: 12. Five core header items (start, middle and end of "Implemented here"), four further header items (one filling the fourth 7.x slot, three in place of adversarial items, since phase 28 has none), and three items from the 7.x blocks 7.1, 7.2 and 7.3.
- Items fully PASS: 4 (V-28-01 M-70, V-28-06 7.1, V-28-08 7.3, V-28-10 GV-47).
- Items with at least one GAP: 8 (V-28-02, V-28-03, V-28-04, V-28-05, V-28-07, V-28-09, V-28-11, V-28-12).
- GAP lines: 11 in total. HIGH 0, MEDIUM 7, LOW 4.

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-28-02-a | MEDIUM | T4 | `input_too_large` is in no refused group of the monitoring counts | section 3.7 (line 561) |
| V-28-02-b | MEDIUM | T6 | golden draw has no run selector; `casework.qa_samples` has no run column | section 3.6 step 4 (line 544); phase 19 draw (phase-19.md:1254-1256) |
| V-28-02-c | MEDIUM | T6 | time-boxed grant for golden items is not defined in phase 19 | section 3.6 step 4 (line 544); phase 19 section 3.8 step 7 (phase-19.md:1261) |
| V-28-03-a | LOW | T3 | engage body has two names; release body absent from the utoipa list | section 2 (lines 409, 438) |
| V-28-04-a | MEDIUM | T2 | no route or field returns draft provenance; UX-45 "provenance one click away" has no element | section 2 (lines 424, 438); section 5.1 (lines 787-793) |
| V-28-05-a | MEDIUM | T4 | narrative accept takes lock 4 before lock 3, against the order of section 3.15 | section 3.5 steps 6 and 10 (lines 519, 525); section 3.15 (lines 716-722) |
| V-28-07-a | MEDIUM | T7 | section 4.5 and Open doubts 24 require a draft purge that block 7.2 and phase 23 exclude | section 4.5 (line 779); Open doubts 24 (line 947) |
| V-28-07-b | LOW | T6 | R11 row uses clock `retirement`; phase 23 gives R11 clock `age` only | section 1.3 retention rows (line 382) |
| V-28-09-a | LOW | T4 | "these two rows" in 4.4 but one zone row (llama); llama-eval row not written | section 4.4 (lines 756, 763) |
| V-28-11-a | LOW | T4 | canonical manifest form is not stated or cited (phase 27 defines it) | section 3.9 steps 2 and 3 (lines 581-582) |
| V-28-12-a | MEDIUM | T4 | escape replacement for `<` is the same character, so the escape is undefined | section 3.2 (line 479) |
