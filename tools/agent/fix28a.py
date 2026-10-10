p='docs/plan/phase-28.md'
s=open(p).read()
def rep(a,b,n=1):
    global s
    assert s.count(a)==n,(s.count(a),a[:90])
    s=s.replace(a,b)

# 6: version number uniqueness
rep("    CONSTRAINT uq_assistant_version_no UNIQUE (kind, task, version_no),",
    "    CONSTRAINT uq_assistant_version_no UNIQUE NULLS NOT DISTINCT (kind, task, version_no),   -- models have task NULL (PostgreSQL 15+)")

# 3: run ids on evaluations and drafts
rep("    golden_set_sha256   TEXT NOT NULL CHECK (golden_set_sha256 ~ '^[0-9a-f]{64}$'),   -- HMAC-SHA256 of the sorted golden case ids (section 3.13)",
    "    run_id              UUID NOT NULL,                          -- one run, two stage rows (section 3.6); the drafts carry it as eval_run_id\n"
    "    golden_set_sha256   TEXT NOT NULL CHECK (golden_set_sha256 ~ '^[0-9a-f]{64}$'),   -- HMAC-SHA256 of the sorted golden case ids (section 3.13)")
rep("CREATE INDEX IF NOT EXISTS idx_assistant_evaluations_triple\n    ON admin.assistant_evaluations (task, model_version_id, template_version_id, created_at);",
    "CREATE INDEX IF NOT EXISTS idx_assistant_evaluations_triple\n    ON admin.assistant_evaluations (task, model_version_id, template_version_id, created_at);\n"
    "CREATE UNIQUE INDEX IF NOT EXISTS uq_assistant_evaluation_stage ON admin.assistant_evaluations (run_id, (result->>'stage'));")
rep("    mode                TEXT NOT NULL CHECK (mode IN ('live', 'evaluation')),  -- evaluation: hidden golden or shadow output",
    "    mode                TEXT NOT NULL CHECK (mode IN ('live', 'evaluation')),  -- evaluation: hidden golden or shadow output\n"
    "    eval_run_id         UUID,                                   -- the evaluation run (admin.assistant_evaluations.run_id); evaluation mode only")
rep("    CHECK (mode = 'live' OR status NOT IN ('accepted')),",
    "    CHECK (mode = 'live' OR status NOT IN ('accepted')),\n    CHECK ((mode = 'evaluation') = (eval_run_id IS NOT NULL)),")
rep("CREATE INDEX IF NOT EXISTS idx_assistant_drafts_model   ON casework.assistant_drafts (model_version_id, template_version_id);",
    "CREATE INDEX IF NOT EXISTS idx_assistant_drafts_model   ON casework.assistant_drafts (model_version_id, template_version_id);\n"
    "CREATE INDEX IF NOT EXISTS idx_assistant_drafts_run     ON casework.assistant_drafts (eval_run_id) WHERE eval_run_id IS NOT NULL;")
rep("                          'assistant_golden', 'assistant_assisted'));\n```",
    "                          'assistant_golden', 'assistant_assisted'));\n"
    "-- Phase 19 source kind CHECK widened by one value: an assistant_golden item is the evaluation draft itself (lead review).\n"
    "ALTER TABLE casework.qa_samples DROP CONSTRAINT IF EXISTS qa_samples_source_kind_check;\n"
    "ALTER TABLE casework.qa_samples ADD CONSTRAINT qa_samples_source_kind_check\n"
    "    CHECK (source_kind IN ('alert', 'case', 'decision', 'hibernation_hit', 'pair_memory_suppression', 'assistant_draft'));\n"
    "ALTER TABLE casework.qa_samples DROP CONSTRAINT IF EXISTS qa_samples_assistant_draft_case;\n"
    "ALTER TABLE casework.qa_samples ADD CONSTRAINT qa_samples_assistant_draft_case\n"
    "    CHECK (source_kind <> 'assistant_draft' OR case_id IS NOT NULL);   -- source_id = draft id, source_at = requested_at\n```")
rep("       OR NEW.task IS DISTINCT FROM OLD.task OR NEW.mode IS DISTINCT FROM OLD.mode OR NEW.language IS DISTINCT FROM OLD.language",
    "       OR NEW.task IS DISTINCT FROM OLD.task OR NEW.mode IS DISTINCT FROM OLD.mode OR NEW.language IS DISTINCT FROM OLD.language\n"
    "       OR NEW.eval_run_id IS DISTINCT FROM OLD.eval_run_id")
rep("  ('assistant_evaluations', 'internal', ARRAY['id', 'task', 'model_version_id', 'template_version_id', 'golden_set_sha256',",
    "  ('assistant_evaluations', 'internal', ARRAY['id', 'run_id', 'task', 'model_version_id', 'template_version_id', 'golden_set_sha256',")
rep("  ('internal', ARRAY['id', 'case_id', 'report_id', 'task', 'mode', 'language',",
    "  ('internal', ARRAY['id', 'case_id', 'report_id', 'task', 'mode', 'eval_run_id', 'language',")
rep("  - `AssistantEvaluation { id, task, model_version_id,","  - `AssistantEvaluation { id, run_id, task, model_version_id,")
rep("  - `AssistantDraft { id, case_id, report_id: Option<Uuid>, task, mode: DraftMode (live | evaluation),",
    "  - `AssistantDraft { id, case_id, report_id: Option<Uuid>, task, mode: DraftMode (live | evaluation), eval_run_id: Option<Uuid>,")

# 5: accept scope
rep("| POST | `/assistant/drafts/{id}/accept` | queue.work, report.prepare | Accept an edited or an as-is draft: writes a case note or the report narrative; `If-Match` (draft version); `Idempotency-Key` (`assistant.draft.accept`; API 3.15 row, owned; the second scope admits A2 and M for reports, Cedar decides the rest) |",
    "| POST | `/assistant/drafts/{id}/accept` | queue.work | Accept an edited or an as-is draft: writes a case note or the report narrative; `If-Match` (draft version); `Idempotency-Key` (`assistant.draft.accept`; API 3.15 row, owned; phase 21 `report.edit` decides the narrative) |")
rep("**Route `POST /assistant/drafts/{id}/accept`** (`queue.work`, `report.prepare`; Cedar","**Route `POST /assistant/drafts/{id}/accept`** (`queue.work`; Cedar")
rep("the draft routes admit A1, A2 and M through `queue.work` (and A2 and M also through `report.prepare` on the accept), every one",
    "the draft routes admit A1, A2 and M through `queue.work`, every one")
rep("The API.md row of the accept (`queue.work` in 3.15) gains the scope `report.prepare` so that A2 and M can reach the narrative accept; Cedar still decides, and A1 keeps the case-summary path only (section 3.10).",
    "The API.md row of the accept keeps `queue.work` alone (A2 and M already hold it); phase 21 `report.edit` refuses A1 on a narrative, so A1 keeps the case-summary path only (section 3.10; lead review: the draft's added `report.prepare` admitted nobody new).")

# 8: note prefix
rep("a case summary is at most `casework.note_max_chars` (phase 18),",
    "a case summary is at most `casework.note_max_chars` (phase 18) minus the length of the first line that step 10 adds,")

# 9: register owner
rep("the owner follows the phase 24 owner rule for register findings, as phase 27 section 3.8 does for `model_monitor`.",
    "the owner is the owner of the task's live template inventory item (the live model item's owner when the task has none; phase 27 uses the family owner, which the assistant does not have).")

# 2: template-only promotion
rep("1. The model version is `registered` (409 `not_registered`); each named template is `registered` or `live` and is for its task (422 `E_VALIDATION` at `/template_version_ids` otherwise).",
    "1. The model version is `registered` or `live` (409 `not_registered`); each named template is `registered` or `live` and is for its task (422 `E_VALIDATION` at `/template_version_ids` otherwise); a request in which the model and every named template are already live is 409 `nothing_to_promote` (a template-only change keeps the live model: its artifact cannot be imported twice).")
rep("set the previous live model and each previous live template of the named tasks to `retired` with `retired_at`, set the new versions to `live` with `live_from = now()`;",
    "set to `retired` with `retired_at` only the live versions being replaced (the live model when another model is named, the live template of a task when another template is named), leave a named version that is already live unchanged, and set the new versions to `live` with `live_from = now()`;")
rep("`not_registered`, `inventory_not_approved`, `evaluation_missing`,","`not_registered`, `nothing_to_promote`, `inventory_not_approved`, `evaluation_missing`,")

# 1 and 7: evaluation server and serve command
rep("**Verification** (`assistant_version_verify`, worker): (a) `ArtifactStore::read_sha256(artifact_key)` must equal `artifact_sha256` (`sha_mismatch` otherwise); (b) the server loads the file in the test profile and answers `/health` with the expected `context_tokens` (`model_load_failed` otherwise);",
    "**Verification** (`assistant_version_verify`, worker): (a) `ArtifactStore::read_sha256(artifact_key)` must equal `artifact_sha256` (`sha_mismatch` otherwise); (b) `llama-eval` (section 4.4), started by the operator with this version, reports the artifact's SHA-256 equal to `artifact_sha256` and answers `/health` with the expected `context_tokens` (`model_load_failed` otherwise; the job retries for at most one hour while the server is not started);")
rep("**Golden set (`GoldenSet::build(task)`):**",
    "**Server and principal (lead review).** A candidate model is served by `llama-eval` (section 4.4), never by the production `llama`, which holds only the live model (and none before the first go-live). The job first checks that `llama-eval` reports the candidate's `artifact_sha256`; a mismatch or no answer ends the run with `passed = false` and reason `eval_server_mismatch`. Evaluation drafts carry `requested_by` = the run's initiator (a record of who asked) and `eval_run_id` = the run; their retrieval runs as the system principal with purpose `qa_review`, never with the initiator's visibility, and the sealed-case refusal of section 3.1 step 9 applies to live drafts only (golden cases are sealed by definition).\n\n"
    "**Golden set (`GoldenSet::build(task)`):**")
rep("4. Human review: the QA lead draws the phase 19 population `assistant_golden` for the run's evaluation drafts (`POST /qa/samples` of phase 19, `n = assistant.gate_min_items`, the window of the run).",
    "4. Human review: the QA lead draws the phase 19 population `assistant_golden` for the run's evaluation drafts (`POST /qa/samples` of phase 19, `n = assistant.gate_min_items`, the drafts whose `eval_run_id` is the run); each item has `source_kind = assistant_draft`, `source_id` = the draft id, `case_id` = the golden case, and phase 19 `GET /qa/samples/{id}/item` shows its reviewer the verified output with its cited records under the time-boxed grant (requirement on phase 19, section 4.5).")
rep("The daily job `assistant_monitor` (section 3.7) writes `stage = \"final\"` for each run whose QA review of `assistant_golden` is complete,",
    "The daily job `assistant_monitor` (section 3.7) writes a second row with the same `run_id` and `stage = \"final\"` for each run whose QA review of `assistant_golden` (the samples whose draft carries the run's `eval_run_id`) is complete,")
rep("- **Zone (new network `z2_model`, `internal: true`, no egress).** Members: `akr-worker` and `llama` only.",
    "- **Zone (new network `z2_model`, `internal: true`, no egress).** Members: `akr-worker`, `llama` and `llama-eval` only.")
rep("A promotion takes effect when `llama` is restarted with the rendered command (`akr-ctl net render`), which the phase 00 release procedure performs in the same step.",
    "A promotion takes effect when the operator runs `akr-ctl assistant serve`, which renders the `llama` command from the live version and restarts the container (`akr-ctl net render` renders the nftables zone rules only, phase 00); until then the readiness mismatch refuses drafts with `model_unavailable` (fail closed). With no live version `llama` is not started.\n"
    "- **Evaluation server (`llama-eval`, lead review).** The same image in compose profile `assistant-eval`, on `z2_model`, with the `assistant-store` volume read-only; started by `akr-ctl assistant eval-server --version <id>` with one `registered` or `live` artifact and stopped by `akr-ctl assistant eval-server --stop`. It serves only `assistant_version_verify` and `assistant_evaluate` through a second `AssistantModelPort` instance (`llm_eval`), never a live draft, and reports the SHA-256 of the file it loaded.")
rep("add `z2_model` and the volume to `akr-worker`, and add the test-only service `llama-stub`",
    "add the service `llama-eval` (profile `assistant-eval`, same networks and volume), add `z2_model` and the volume to `akr-worker`, and add the test-only service `llama-stub`")
rep("`llm: Arc<dyn AssistantModelPort>` (`LlamaServerClient`),","`llm: Arc<dyn AssistantModelPort>` (`LlamaServerClient` to `llama`), `llm_eval: Arc<dyn AssistantModelPort>` (to `llama-eval`),")
rep("akr-ctl (`assistant artifact add`, `assistant keys` reuses the phase 27 `keys import`)",
    "akr-ctl (`assistant artifact add`, `assistant serve`, `assistant eval-server`; keys through the phase 27 `keys import`)")
rep("- **Phase 19:** `casework.qa_samples.population` gains `assistant_golden` and `assistant_assisted` (`0089`);",
    "- **Phase 19:** `casework.qa_samples.population` gains `assistant_golden` and `assistant_assisted` and `source_kind` gains `assistant_draft` (`0089`); the `assistant_assisted` draw skips decisions already in a sample (unique `(source_kind, source_id)`);")

# lead corrections paragraph before Open doubts
rep("## Open doubts (for gap pass 1 and the bank)",
"**Lead corrections (review 2026-10-09).** The production `llama` serves only the live model, so a candidate could never be verified or evaluated and no model could go live; a separate `llama-eval` server in profile `assistant-eval` serves the verify and evaluation jobs, and the job checks the artifact hash it reports. The promotion required a `registered` model and retired the previous live set, so a template change with the live model kept was impossible; a live model or template named in the request now stays live and only replaced versions retire. The golden review had no item: the phase 19 `source_kind` CHECK is widened by `assistant_draft`, the QA item view shows the evaluation draft, and `eval_run_id` and `run_id` tie drafts, samples and both stage rows of a run. Evaluation drafts are retrieved as the system principal with purpose `qa_review`, not with the MRO initiator's visibility. The accept keeps `queue.work` alone; model version numbers are unique with `NULLS NOT DISTINCT`; `akr-ctl assistant serve` (not `net render`) restarts the model server; the summary limit leaves room for the note's first line; a breach finding is owned by the task's template item owner.\n\n"
"## Open doubts (for gap pass 1 and the bank)")
open(p,'w').write(s)
print('ok')
