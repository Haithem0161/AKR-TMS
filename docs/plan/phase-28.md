# Phase 28: Assistant

**Goal:** A bank that switches the assistant on gets two drafting tasks (a case summary in Arabic or English, and a report narrative in both languages) that run on an on-premises open-weight language model with no egress, are checked against the case record by a deterministic verifier, reach the case only when a named person edits or accepts them, are sealed with their provenance, and are gated by a golden evaluation before go-live; every task can be switched off per bank, and a two-part kill switch returns all drafting to the manual path at once.

**Dependencies:** Phase 00 (jobs, settings, cron schedules, network zones `deploy/zones.yaml`, the signed release bundle, the `assistant-store` volume pattern), Phase 01 (`StepUpService` with the closed classes `approval` and `record_open`; `admin.users`, `admin.role_grants`), Phase 02 (audit catalogue with the `siem` flag, `access_event`), Phase 03 (Cedar bundle and range C2, `Ctx` with required `purpose` and `network`, `admin.feature_flags`, purpose codes `investigation` and `report_preparation`, `qa_review`), Phase 04 (`admin.inventory_items` with kinds `llm_component` and `prompt_template`, status `approved_for_use`, `ChangeTargetRegistry`, routes `cr_material` and `cr_material_validated`, `activate_change`), Phase 09 (`PolicyPackService::scan` with scope `reason_text`), Phase 17 (visibility rules 1 to 5 and `services::visibility::build`, `CaseChain::append_event`), Phase 18 (`casework.cases`, `case_acl`, `casework.notes` through `NoteService::post`, `casework.opened_views` and its panel keys, `casework.decisions.ai_involvement`, `RecordDecisionEffect`, `E_CASE_SEALED`), Phase 19 (`casework.qa_samples` and `qa_reviews`, populations and `overall` values), Phase 21 (`rpt.reports`, the narrative fields of the builder and `PATCH /reports/{id}`, `report.small_number_threshold`), Phase 24 (`admin.registers` and its closed `source` list, `ValidationPackService::issue` with seven parts, review calendar), Phase 26 (the optional-pack pattern: flag off means 404 `E_NOT_FOUND`), Phase 27 (the model governance pattern: inventory kind `model`, signed import in two steps, verify-only Ed25519 keys at `keys/model-artifact/v<N>.pub`, `model-store` read-only volume, kill switch engaged at once and released by `cr_material`, `model_monitor` register source)

**Macro-phase:** P2
**Complexity:** M (12 routes: the 3 rows of API.md 3.15 owned here, `POST /assistant/drafts`, `POST /assistant/drafts/{id}/accept` and `GET /assistant/evaluations`, and 9 rows added to 3.15; 4 tables in schema `admin` and `casework` in 3 migrations `0088` to `0090`; 2 widened CHECKs of earlier phases (`admin.registers.source` in phase 24, `casework.qa_samples.population` in phase 19); 4 new job kinds; 1 Cedar file; 2 compose services (`llama`, `llama-eval`), 1 internal network and 1 read-only volume; 18 DR numbers, DR-274 to DR-291)
**Binaries touched:** akr-api (router `assistant`; `AssistantService` (gateway), `AssistantReads`, `AssistantKillSwitchService`, `AssistantVersionService` in `AppState`; `ReportService::update_draft` (the PATCH path of `/reports/{id}` in phase 21 section 3.2, called in-process; not a new route; phase 21 does not name the method, so gap pass 1 names it in phase 21)) | akr-worker (jobs `assistant_generate`, `assistant_evaluate`, `assistant_version_verify`, `assistant_monitor`; `GroundingVerifier`, `PromptBuilder`, `LlamaServerClient` (the `AssistantModelPort` implementation), `AssistantEvaluationService`, `AssistantMonitor`) | akr-ingestd unchanged | akr-assess unchanged | akr-ctl (`assistant artifact add`, `assistant serve`, `assistant eval-server`; keys through the phase 27 `keys import`)

**Requirements carried:**
- Implemented here: M-70 (task registry, tiers, autonomy mode `draft`, per-task switch, kill switch, per-task gate; no close, escalate, hibernate, file, sign, send or contact), M-71 (provenance store, grounding verifier, draft state, save and export blocked until an edit or an accept), M-72 (partly: the draft is hidden until the panels it cites are opened, and no accept-all; the recommendation parts are left to a later task), M-73 (golden evaluation gate per task in Arabic and English, shadow period through evaluation mode, staged release, regression on every change, and the monitoring panel), T-48 (two switchable use cases, both drafts, none autonomous), T-49 (the evaluation workbench of section 3.6, golden set, error counts, QA sampling), T-50 (containment rules; no agent with tools is built, section 3.9), T-51 (Arabic-capable open weights on premises, model choice by the bank's evaluation; no candidate model is selected here), TS-17 (no external AI API; the runtime is optional and on premises), SP-94 (untrusted text as data, no tools or write path, permission-aware retrieval, output inherits the highest tier of its inputs, no path to a customer channel, a separate network zone without egress, prompts and outputs logged as suspicion-tier records), SP-95 (per-task and global kill switch to manual drafting, DR-281), GV-45 (technically enforced permitted list: the two tasks are the only ones the task registry accepts; every other use is refused as an unknown task), GV-46 (provenance per draft, section 1 and section 3.4), GV-47 (grounding and verification, section 3.3), GV-48 (no save or export before a person edits or accepts), GV-49 (delimited data, timeouts, kill switch, full action log), GV-50 (evaluation suite in Arabic and English, run before any model or template change), GV-51 (model-side supplier checklist: provenance, licence and training-data statement are required fields of the signed manifest; security tests and exit plan are Open doubts), GV-52 (on-premises by default), IR-23 (labelled draft with sentence-level sources; saving requires an edit or an explicit accept), UX-45 (draft label, sentence sources, removed or marked sentences, a "not considered" line, no accept-all, a useful-or-not control, provenance one click away, names and amounts copied)
- Connected to earlier phases (reused, not repeated): GV-03 and GV-04 (phase 04 inventory items of kinds `llm_component` and `prompt_template`, tier 3), GV-09 and GV-15 (phase 24 validation pack, the assistant branch of section 3.6), GV-32 (validator is never the owner), GV-39 (only QA-confirmed outcomes make golden items, phase 19 `qa_reviews`), GV-40 (no export route returns drafts), GV-42 (only the evidence-opened rule of section 3.5 is carried; the closure gate stays in phase 27 Open doubts)
- Not carried: GV-42 (automation-bias gate before closure: phase 27 Open doubts), GV-43 (filing and signing stay human: no change, existing design), GV-44 (phase 22), GV-53 (phase 2 pre-release API, `akr-assess`), GV-54 (phase 27 and 16), recommendation and disposition parts of M-72 and research 21 use case 5 (Open doubts 1), extraction, adverse-media triage, investigation chat, RFI drafting, QA triage and regulatory mapping of research 21 section 12a (Open doubts 2)
- Gap pass 1 (coverage), not carried: UX-46 (explanation usability protocol), W-05 (LLM assistants); P2, DR-400; VA-10 (generative assistant features), VO-12 (agent register); P2, DR-402; VB-09, VB-10, VB-29, VB-30, VB-39, VB-40, VB-41, VB-42, VB-46, VB-50 and VB-53 (assistant and agent items; P2, DR-403).

---

## 1. SQLx Migration Changes

Three migrations, numbered after phase 27's `0087` (phase 26 used `0083` and `0084`; phase 27 used `0085` to `0087`; grep of docs/plan confirms `0088` to `0090` are free). Four tables: `admin.assistant_versions` (signed model artifacts and prompt templates, one table with a `kind`), `admin.assistant_evaluations` (write-once gate results), `admin.assistant_kill_state` (one row per scope, the phase 27 pattern), and `casework.assistant_drafts` (the drafts, in the case schema, section 1.2). No column of an earlier table is added. Two CHECKs of earlier phases are widened in `0089` and stated there.

**Why `casework` for drafts and `admin` for versions (DR-276, DR-277):** a draft holds case content, is written beside its case, is refused on a sealed case, and must follow the case's retention clock and visibility; it therefore lives in `casework` next to `casework.notes`, with the same tier rule as its case. The version and evaluation registry holds no case content and follows the model registry of phase 27 in `admin`.

**Why a separate registry and not a phase 27 family (DR-276):** the `admin.model_versions` CHECKs of phase 27 are written for tree ensembles (`artifact_key` ends in `.onnx`, `runtime` is `ort` or `tract`, feature keys and monotone constraints, alert scoring). A language model has none of these and writes no alert score. Loosening those CHECKs silently would weaken phase 27, so this phase creates its own table, reuses the phase 27 key purpose and the phase 04 inventory, and changes no phase 27 CHECK.

### 1.1 `0088_admin_assistant_registry.sql`

```sql
-- migration 0088_admin_assistant_registry.sql
-- Phase 28. Assistant versions (signed model artifacts and prompt templates), write-once evaluations, kill state (schema admin),
-- and the widened phase 24 registers.source CHECK. Idempotent: CREATE IF NOT EXISTS; constraints dropped and re-added.

CREATE TABLE IF NOT EXISTS admin.assistant_versions (
    id                      UUID PRIMARY KEY,                       -- UUIDv7, app-side
    kind                    TEXT NOT NULL CHECK (kind IN ('model', 'template')),
    task                    TEXT CHECK (task IS NULL OR task IN ('case_summary', 'report_narrative')),   -- templates only
    version_no              INT NOT NULL CHECK (version_no >= 1),
    inventory_item_id       UUID NOT NULL UNIQUE REFERENCES admin.inventory_items(id) ON DELETE RESTRICT,  -- kind llm_component or prompt_template
    status                  TEXT NOT NULL CHECK (status IN ('importing', 'refused', 'registered', 'live', 'retired')),
    refusal_code            TEXT CHECK (refusal_code IS NULL OR refusal_code IN ('signature_invalid', 'manifest_invalid', 'sha_mismatch',
                                        'model_load_failed', 'template_placeholder_unknown', 'template_scan_hit', 'template_too_long')),
    artifact_key            TEXT CHECK (artifact_key IS NULL OR artifact_key ~ '^[a-z0-9][a-z0-9_./-]{2,200}[.]gguf$'),
    artifact_sha256         TEXT CHECK (artifact_sha256 IS NULL OR artifact_sha256 ~ '^[0-9a-f]{64}$'),
    quantisation            TEXT CHECK (quantisation IS NULL OR quantisation ~ '^[A-Za-z0-9_]{2,16}$'),
    context_tokens          INT CHECK (context_tokens IS NULL OR context_tokens BETWEEN 512 AND 32768),
    body_text               TEXT CHECK (body_text IS NULL OR char_length(body_text) BETWEEN 1 AND 20000),   -- templates only
    body_sha256             TEXT CHECK (body_sha256 IS NULL OR body_sha256 ~ '^[0-9a-f]{64}$'),
    manifest                JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(manifest) = 'object'),  -- signed fields (section 3.9)
    manifest_sha256         TEXT CHECK (manifest_sha256 IS NULL OR manifest_sha256 ~ '^[0-9a-f]{64}$'),
    signature               TEXT CHECK (signature IS NULL OR length(signature) BETWEEN 86 AND 88),          -- Ed25519, base64 of 64 bytes
    signing_key_version     INT CHECK (signing_key_version IS NULL OR signing_key_version >= 1),
    placeholders            JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(placeholders) = 'array'),   -- templates: the placeholder kinds used
    created_by              UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    live_from               TIMESTAMPTZ,
    retired_at              TIMESTAMPTZ,
    version                 INT NOT NULL DEFAULT 1,
    CONSTRAINT uq_assistant_version_no UNIQUE NULLS NOT DISTINCT (kind, task, version_no),   -- models have task NULL (PostgreSQL 15+)
    CHECK ((kind = 'model') = (artifact_key IS NOT NULL AND artifact_sha256 IS NOT NULL AND quantisation IS NOT NULL
                               AND context_tokens IS NOT NULL AND task IS NULL AND body_text IS NULL)),
    CHECK ((kind = 'template') = (body_text IS NOT NULL AND body_sha256 IS NOT NULL AND task IS NOT NULL
                                  AND artifact_key IS NULL AND artifact_sha256 IS NULL)),
    CHECK (status = 'importing' OR (manifest_sha256 IS NOT NULL AND signature IS NOT NULL AND signing_key_version IS NOT NULL)),
    CHECK ((status = 'refused') = (refusal_code IS NOT NULL)),
    CHECK (status <> 'live' OR live_from IS NOT NULL),
    CHECK ((status = 'retired') = (retired_at IS NOT NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_assistant_artifact ON admin.assistant_versions (artifact_sha256) WHERE artifact_sha256 IS NOT NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uq_assistant_body     ON admin.assistant_versions (body_sha256) WHERE body_sha256 IS NOT NULL AND kind = 'template';
-- One live model for the bank; one live template per task (the runtime serves one model at a time).
CREATE UNIQUE INDEX IF NOT EXISTS uq_assistant_one_live_model ON admin.assistant_versions ((kind)) WHERE kind = 'model' AND status = 'live';
CREATE UNIQUE INDEX IF NOT EXISTS uq_assistant_one_live_template ON admin.assistant_versions (task)
    WHERE kind = 'template' AND status = 'live';
CREATE INDEX IF NOT EXISTS idx_assistant_versions_kind ON admin.assistant_versions (kind, status);

-- Evaluations: deterministic metrics and human review counts of one (task, template, model) triple. Written once. No case id.
CREATE TABLE IF NOT EXISTS admin.assistant_evaluations (
    id                  UUID PRIMARY KEY,
    task                TEXT NOT NULL CHECK (task IN ('case_summary', 'report_narrative')),
    model_version_id    UUID NOT NULL REFERENCES admin.assistant_versions(id) ON DELETE RESTRICT,
    template_version_id UUID NOT NULL REFERENCES admin.assistant_versions(id) ON DELETE RESTRICT,
    run_id              UUID NOT NULL,                          -- one run, two stage rows (section 3.6); the drafts carry it as eval_run_id
    golden_set_sha256   TEXT NOT NULL CHECK (golden_set_sha256 ~ '^[0-9a-f]{64}$'),   -- HMAC-SHA256 of the sorted golden case ids (section 3.13)
    result              JSONB NOT NULL CHECK (jsonb_typeof(result) = 'object'),   -- counts, rates and gate verdicts; no ids, no text
    passed              BOOLEAN NOT NULL,
    data_as_of          TIMESTAMPTZ NOT NULL,
    created_by          UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_assistant_evaluations_triple
    ON admin.assistant_evaluations (task, model_version_id, template_version_id, created_at);
CREATE UNIQUE INDEX IF NOT EXISTS uq_assistant_evaluation_stage ON admin.assistant_evaluations (run_id, (result->>'stage'));

-- Kill state: one row per scope. Engaging is immediate (one MRO, step-up); releasing is a change request (section 3.8).
CREATE TABLE IF NOT EXISTS admin.assistant_kill_state (
    scope           TEXT PRIMARY KEY CHECK (scope = 'global' OR scope IN ('task:case_summary', 'task:report_narrative')),
    engaged         BOOLEAN NOT NULL,
    reason_code     TEXT CHECK (reason_code IS NULL OR reason_code IN ('model_error', 'data_break', 'red_breach',
                                                                      'regulator_request', 'other')),
    engaged_by      UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    engaged_at      TIMESTAMPTZ,
    next_drill_due  DATE NOT NULL,                       -- GV-20 drill, read by the phase 24 review calendar
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    version         INT NOT NULL DEFAULT 1,
    CHECK (NOT engaged OR (engaged_by IS NOT NULL AND engaged_at IS NOT NULL AND reason_code IS NOT NULL))
);
INSERT INTO admin.assistant_kill_state (scope, engaged, next_drill_due) VALUES
 ('global', FALSE, (CURRENT_DATE + INTERVAL '3 months')::date),
 ('task:case_summary', FALSE, (CURRENT_DATE + INTERVAL '3 months')::date),
 ('task:report_narrative', FALSE, (CURRENT_DATE + INTERVAL '3 months')::date)
ON CONFLICT (scope) DO NOTHING;

-- Phase 24 CHECK widened (the phase 27 state of 0085 is kept; this adds one value). Constraint name is the PostgreSQL default.
ALTER TABLE admin.registers DROP CONSTRAINT IF EXISTS registers_source_check;
ALTER TABLE admin.registers ADD CONSTRAINT registers_source_check
    CHECK (source IN ('manual', 'post_review', 'fairness', 'review_calendar', 'scan', 'model_monitor', 'assistant_monitor'));
```

Note on `admin.assistant_versions`: a `live` template is the only template of its task that can serve drafts, and the live model is the only model. A version that went live is never edited (section 1.3 guard). A `refused` row keeps its signature and manifest so the refusal can be shown to MRO (`refusal_code` is one of the seven codes above, the same style as phase 27's `refusal_code`).

### 1.2 `0089_casework_assistant_drafts.sql`

```sql
-- migration 0089_casework_assistant_drafts.sql
-- Phase 28. Drafts of the two tasks (schema casework: they hold case content and follow the case's visibility and clock),
-- and the widened phase 19 population CHECK. Idempotent.

CREATE TABLE IF NOT EXISTS casework.assistant_drafts (
    id                  UUID PRIMARY KEY,                       -- UUIDv7, app-side
    case_id             UUID NOT NULL REFERENCES casework.cases(id) ON DELETE RESTRICT,
    report_id           UUID REFERENCES rpt.reports(id) ON DELETE RESTRICT,   -- required for report_narrative only
    task                TEXT NOT NULL CHECK (task IN ('case_summary', 'report_narrative')),
    mode                TEXT NOT NULL CHECK (mode IN ('live', 'evaluation')),  -- evaluation: hidden golden or shadow output
    eval_run_id         UUID,                                   -- the evaluation run (admin.assistant_evaluations.run_id); evaluation mode only
    language            TEXT NOT NULL CHECK (language IN ('ar', 'en', 'both')),
    status              TEXT NOT NULL CHECK (status IN ('requested', 'ready', 'refused', 'accepted', 'discarded')),
    refusal_code        TEXT CHECK (refusal_code IS NULL OR refusal_code IN ('kill_engaged', 'task_disabled', 'not_visible',
                                        'input_too_large', 'model_unavailable', 'output_unparseable', 'nothing_supported')),
    template_version_id UUID NOT NULL REFERENCES admin.assistant_versions(id) ON DELETE RESTRICT,
    model_version_id    UUID NOT NULL REFERENCES admin.assistant_versions(id) ON DELETE RESTRICT,
    input_sha256        TEXT CHECK (input_sha256 IS NULL OR input_sha256 ~ '^[0-9a-f]{64}$'),   -- digest of the delimited input, not the input
    source_ids          JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(source_ids) = 'array'),  -- record references: kind and id
    cited_panels        JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(cited_panels) = 'array'),  -- panel keys of section 3.2
    output_enc          BYTEA,                                  -- verified sentences, sealed under AAD = id
    grounding_result    JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(grounding_result) = 'object'),  -- counts and codes only
    scan_hits           INT NOT NULL DEFAULT 0 CHECK (scan_hits >= 0),
    final_enc           BYTEA,                                  -- the text the person accepted, sealed under AAD = id
    accept_mode         TEXT CHECK (accept_mode IS NULL OR accept_mode IN ('as_is', 'edited')),
    edit_distance       INT CHECK (edit_distance IS NULL OR edit_distance >= 0),
    edit_ratio          NUMERIC(5,4) CHECK (edit_ratio IS NULL OR edit_ratio BETWEEN 0 AND 1),
    useful              BOOLEAN,                                -- the useful-or-not control (UX-45), set on accept or discard
    target_kind         TEXT CHECK (target_kind IS NULL OR target_kind IN ('note', 'report')),
    target_id           UUID,                                   -- the note id (case_summary) or the report id (report_narrative)
    requested_by        UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    requested_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    accepted_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    accepted_at         TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    CHECK ((task = 'report_narrative') = (report_id IS NOT NULL)),
    CHECK ((task = 'report_narrative') = (language = 'both')),
    CHECK (mode = 'live' OR status NOT IN ('accepted')),
    CHECK ((mode = 'evaluation') = (eval_run_id IS NOT NULL)),
    CHECK ((status = 'refused') = (refusal_code IS NOT NULL)),
    CHECK (status NOT IN ('ready', 'accepted') OR output_enc IS NOT NULL),
    CHECK ((status = 'accepted') = (accepted_by IS NOT NULL AND accepted_at IS NOT NULL AND final_enc IS NOT NULL
                                    AND accept_mode IS NOT NULL AND edit_distance IS NOT NULL AND target_id IS NOT NULL
                                    AND target_kind IS NOT NULL)),
    CHECK (accept_mode IS DISTINCT FROM 'as_is' OR edit_distance = 0)
);
CREATE INDEX IF NOT EXISTS idx_assistant_drafts_case   ON casework.assistant_drafts (case_id, requested_at);
CREATE INDEX IF NOT EXISTS idx_assistant_drafts_status ON casework.assistant_drafts (status, requested_at);
CREATE INDEX IF NOT EXISTS idx_assistant_drafts_task_day ON casework.assistant_drafts (task, mode, requested_at);
CREATE INDEX IF NOT EXISTS idx_assistant_drafts_model   ON casework.assistant_drafts (model_version_id, template_version_id);
CREATE INDEX IF NOT EXISTS idx_assistant_drafts_run     ON casework.assistant_drafts (eval_run_id) WHERE eval_run_id IS NOT NULL;

-- Phase 19 population CHECK widened by two values (constraint name is the PostgreSQL default of the inline CHECK).
-- assistant_golden: hidden evaluation outputs sampled for human factual review. assistant_assisted: decisions with ai_involvement used.
ALTER TABLE casework.qa_samples DROP CONSTRAINT IF EXISTS qa_samples_population_check;
ALTER TABLE casework.qa_samples ADD CONSTRAINT qa_samples_population_check
    CHECK (population IN ('closed_alerts', 'closed_cases', 'shelved_decisions', 'mlro_decisions', 'hibernation_hits',
                          'screening_suppressions', 'low_score_forced', 'calibration', 'targeted_repeat',
                          'assistant_golden', 'assistant_assisted'));
-- Phase 19 source kind CHECK widened by one value: an assistant_golden item is the evaluation draft itself (lead review).
ALTER TABLE casework.qa_samples DROP CONSTRAINT IF EXISTS qa_samples_source_kind_check;
ALTER TABLE casework.qa_samples ADD CONSTRAINT qa_samples_source_kind_check
    CHECK (source_kind IN ('alert', 'case', 'decision', 'hibernation_hit', 'pair_memory_suppression', 'assistant_draft'));
ALTER TABLE casework.qa_samples DROP CONSTRAINT IF EXISTS qa_samples_assistant_draft_case;
ALTER TABLE casework.qa_samples ADD CONSTRAINT qa_samples_assistant_draft_case
    CHECK (source_kind <> 'assistant_draft' OR case_id IS NOT NULL);   -- source_id = draft id, source_at = requested_at
```

Notes on `casework.assistant_drafts`:
- `target_id` names the note (`casework.notes.id`, phase 18) for `case_summary` and the report (`rpt.reports.id`, phase 21) for `report_narrative`; no column of those tables is added.
- `useful` is the one-click control of UX-45; it is set by the accept or discard body and read only by the monitoring counts of section 3.7.
- `edit_distance` and `edit_ratio` are computed by the server from `output` and `final` (section 3.5 step 6), never sent by the client.

### 1.3 `0090_assistant_guards_grants.sql`

```sql
-- migration 0090_assistant_guards_grants.sql
-- Phase 28. Guards the application cannot bypass, grants per write path, the pack flag, settings, classification and retention.
-- Idempotent: functions replaced, triggers dropped before creation, inserts use ON CONFLICT DO NOTHING.

-- Version guard. Identity, artifact, manifest and body are immutable; status moves only along the listed edges;
-- live is entered and left only inside AssistantVersionService::activate, which sets the transaction flag akr.assistant_promotion.
CREATE OR REPLACE FUNCTION admin.assistant_versions_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        IF current_user = 'akr_retention' THEN RETURN OLD; END IF;
        RAISE EXCEPTION 'assistant versions are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.id IS DISTINCT FROM OLD.id OR NEW.kind IS DISTINCT FROM OLD.kind OR NEW.task IS DISTINCT FROM OLD.task
       OR NEW.version_no IS DISTINCT FROM OLD.version_no OR NEW.inventory_item_id IS DISTINCT FROM OLD.inventory_item_id
       OR NEW.artifact_key IS DISTINCT FROM OLD.artifact_key OR NEW.artifact_sha256 IS DISTINCT FROM OLD.artifact_sha256
       OR NEW.body_text IS DISTINCT FROM OLD.body_text OR NEW.body_sha256 IS DISTINCT FROM OLD.body_sha256
       OR NEW.manifest IS DISTINCT FROM OLD.manifest OR NEW.manifest_sha256 IS DISTINCT FROM OLD.manifest_sha256
       OR NEW.signature IS DISTINCT FROM OLD.signature OR NEW.signing_key_version IS DISTINCT FROM OLD.signing_key_version
       OR NEW.placeholders IS DISTINCT FROM OLD.placeholders THEN
        RAISE EXCEPTION 'assistant version identity, artifact, body and manifest are immutable' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.status IS DISTINCT FROM OLD.status THEN
        IF (NEW.status = 'live' OR OLD.status = 'live')
           AND current_setting('akr.assistant_promotion', true) IS DISTINCT FROM 'on' THEN
            RAISE EXCEPTION 'live is entered and left only by the promotion change' USING ERRCODE = 'insufficient_privilege';
        END IF;
        IF (OLD.status, NEW.status) NOT IN (
               ('importing', 'registered'), ('importing', 'refused'),
               ('registered', 'live'), ('registered', 'retired'),
               ('live', 'retired')) THEN
            RAISE EXCEPTION 'assistant version status % -> % is not allowed', OLD.status, NEW.status
                USING ERRCODE = 'check_violation';
        END IF;
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_assistant_versions_guard ON admin.assistant_versions;
CREATE TRIGGER trg_assistant_versions_guard BEFORE UPDATE OR DELETE ON admin.assistant_versions
    FOR EACH ROW EXECUTE FUNCTION admin.assistant_versions_guard();

-- Kill state: the scope is fixed; engaging needs a reason; releasing (engaged true -> false) only inside the release change.
CREATE OR REPLACE FUNCTION admin.assistant_kill_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'kill-switch rows are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.scope IS DISTINCT FROM OLD.scope THEN
        RAISE EXCEPTION 'kill-switch scope is immutable' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.engaged AND NOT NEW.engaged
       AND current_setting('akr.assistant_release', true) IS DISTINCT FROM 'on' THEN
        RAISE EXCEPTION 'a kill switch is released only by its change request' USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_assistant_kill_guard ON admin.assistant_kill_state;
CREATE TRIGGER trg_assistant_kill_guard BEFORE UPDATE OR DELETE ON admin.assistant_kill_state
    FOR EACH ROW EXECUTE FUNCTION admin.assistant_kill_guard();

-- Draft guard. Identity fixed; status moves only along the listed edges; the sealed texts are written once.
CREATE OR REPLACE FUNCTION casework.assistant_drafts_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        IF current_user = 'akr_retention' THEN RETURN OLD; END IF;   -- phase 23 retention only, after the case's own clock
        RAISE EXCEPTION 'assistant drafts are deleted only by retention' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.id IS DISTINCT FROM OLD.id OR NEW.case_id IS DISTINCT FROM OLD.case_id OR NEW.report_id IS DISTINCT FROM OLD.report_id
       OR NEW.task IS DISTINCT FROM OLD.task OR NEW.mode IS DISTINCT FROM OLD.mode OR NEW.language IS DISTINCT FROM OLD.language
       OR NEW.eval_run_id IS DISTINCT FROM OLD.eval_run_id
       OR NEW.template_version_id IS DISTINCT FROM OLD.template_version_id OR NEW.model_version_id IS DISTINCT FROM OLD.model_version_id
       OR NEW.requested_by IS DISTINCT FROM OLD.requested_by OR NEW.requested_at IS DISTINCT FROM OLD.requested_at THEN
        RAISE EXCEPTION 'assistant draft identity is immutable' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.output_enc IS NOT NULL AND NEW.output_enc IS DISTINCT FROM OLD.output_enc THEN
        RAISE EXCEPTION 'a verified output is written once' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.final_enc IS NOT NULL AND NEW.final_enc IS DISTINCT FROM OLD.final_enc THEN
        RAISE EXCEPTION 'an accepted text is written once' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.status IS DISTINCT FROM OLD.status AND (OLD.status, NEW.status) NOT IN (
           ('requested', 'ready'), ('requested', 'refused'), ('requested', 'discarded'),
           ('ready', 'accepted'), ('ready', 'discarded')) THEN
        RAISE EXCEPTION 'assistant draft status % -> % is not allowed', OLD.status, NEW.status
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_assistant_drafts_guard ON casework.assistant_drafts;
CREATE TRIGGER trg_assistant_drafts_guard BEFORE UPDATE OR DELETE ON casework.assistant_drafts
    FOR EACH ROW EXECUTE FUNCTION casework.assistant_drafts_guard();

-- Evaluations are written once. Only the phase 23 retention role may delete them.
CREATE OR REPLACE FUNCTION admin.assistant_evaluations_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' AND current_user = 'akr_retention' THEN RETURN OLD; END IF;
    RAISE EXCEPTION 'assistant evaluations are written once; a new run is a new row' USING ERRCODE = 'insufficient_privilege';
END $$;
DROP TRIGGER IF EXISTS trg_assistant_evaluations_guard ON admin.assistant_evaluations;
CREATE TRIGGER trg_assistant_evaluations_guard BEFORE UPDATE OR DELETE ON admin.assistant_evaluations
    FOR EACH ROW EXECUTE FUNCTION admin.assistant_evaluations_guard();

-- Grants per write path (akr_app is the API and worker role). No table is granted to akr_readonly.
GRANT SELECT, INSERT ON admin.assistant_versions TO akr_app;
GRANT UPDATE (status, refusal_code, live_from, retired_at, updated_at, version) ON admin.assistant_versions TO akr_app;
GRANT SELECT, INSERT ON admin.assistant_evaluations TO akr_app;
GRANT SELECT, INSERT ON admin.assistant_kill_state TO akr_app;
GRANT UPDATE (engaged, reason_code, engaged_by, engaged_at, next_drill_due, updated_at, version) ON admin.assistant_kill_state TO akr_app;
GRANT SELECT, INSERT ON casework.assistant_drafts TO akr_app;
GRANT UPDATE (status, refusal_code, input_sha256, source_ids, cited_panels, output_enc, grounding_result, scan_hits, final_enc,
              accept_mode, edit_distance, edit_ratio, useful, target_kind, target_id, accepted_by, accepted_at, updated_at, version)
    ON casework.assistant_drafts TO akr_app;
-- Retention: SELECT on every new table (phase 23 grants SELECT on ALL TABLES only at its own time). No DELETE: no
-- handler of these tables deletes rows in this release (R11 and R6 report_only, R10 never; DR-386).
GRANT SELECT ON admin.assistant_versions, admin.assistant_evaluations, admin.assistant_kill_state TO akr_retention;
GRANT SELECT ON casework.assistant_drafts TO akr_retention;

-- Pack flag (the phase 26 pattern for pack.islamic; phase 03 flag service). Off by default.
INSERT INTO admin.feature_flags (key, enabled, label) VALUES
 ('pack.assistant', FALSE, '{"ar": "حزمة المساعد للمسودات", "en": "Assistant pack (drafts)"}')
ON CONFLICT (key) DO NOTHING;

-- Settings (copy of the phase 27 INSERT format). floor only when governed = TRUE; a floor bounds the side that weakens the control.
INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('assistant.case_summary.enabled',    'false', 'boolean', NULL, TRUE,
  '{"ar": "تشغيل مسودة ملخص القضية", "en": "Case summary task switched on"}'),
 ('assistant.report_narrative.enabled', 'false', 'boolean', NULL, TRUE,
  '{"ar": "تشغيل مسودة سرد البلاغ", "en": "Report narrative task switched on"}'),
 ('assistant.gate_min_items',          '200', 'integer', '{"min": 200, "basis": "M-73 and research 21 12e: at least 200 QA-confirmed golden cases per task"}'::jsonb, TRUE,
  '{"ar": "أدنى عدد حالات ذهبية لبوابة المساعد", "en": "Fewest golden cases for the assistant gate"}'),
 ('assistant.gate_max_removed_rate',   '0.05', 'decimal', '{"max": 0.05, "basis": "design proposal: share of raw sentences the verifier removes; Open doubts 4"}'::jsonb, TRUE,
  '{"ar": "أعلى نسبة جمل تحذفها المراجعة الآلية في بوابة الاختبار", "en": "Highest share of raw sentences removed at the gate"}'),
 ('assistant.qa_sample_percent',       '10', 'integer', '{"min": 10, "basis": "research 21 12e: 10 to 20 per cent of assisted dispositions re-reviewed; M-72"}'::jsonb, TRUE,
  '{"ar": "نسبة المراجعة لقرارات بمساعدة", "en": "Share of assisted decisions re-reviewed (per cent)"}'),
 ('assistant.kill_drill_months',       '3', 'integer', '{"max": 3, "basis": "GV-20 and phase 27: kill switch tested quarterly"}'::jsonb, TRUE,
  '{"ar": "مدة بين تجارب إيقاف المساعد (أشهر)", "en": "Months between assistant kill-switch drills"}'),
 ('assistant.discard_rate_red',        '0.50', 'decimal', '{"max": 0.50, "basis": "design proposal: share of live drafts discarded in 30 days; Open doubts 5"}'::jsonb, TRUE,
  '{"ar": "نسبة المسودات المتروكة التي تفتح سجل المخاطر", "en": "Discarded share that opens a register finding"}'),
 ('assistant.removed_rate_red',        '0.10', 'decimal', '{"max": 0.10, "basis": "design proposal: share of live sentences removed in 30 days; Open doubts 5"}'::jsonb, TRUE,
  '{"ar": "نسبة الجمل المحذوفة التي تفتح سجل المخاطر", "en": "Removed-sentence share that opens a register finding"}'),
 ('assistant.max_input_chars',         '24000', 'integer', NULL, FALSE,
  '{"ar": "أقصى حجم للمدخلات المرسلة للنموذج (حروف)", "en": "Largest model input (characters); no truncation"}'),
 ('assistant.generate_timeout_s',      '120', 'integer', NULL, FALSE,
  '{"ar": "مهلة توليد المسودة (ثوان)", "en": "Generation timeout (seconds)"}'),
 ('assistant.edit_minor_ratio',        '0.20', 'decimal', NULL, FALSE,
  '{"ar": "حد التعديل الطفيف لمسودة", "en": "Edit ratio up to which an edit is minor"}'),
 ('schedule.assistant_monitor',        '"0 20 5 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مراقبة المساعد اليومية", "en": "Daily assistant monitor (05:20 bank time)"}')
ON CONFLICT (key) DO NOTHING;

-- Column classification (copy of the phase 27 insert format). Sealed texts are personal; prompts are never stored.
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('assistant_versions', 'internal', ARRAY['id', 'kind', 'task', 'version_no', 'inventory_item_id', 'status', 'refusal_code',
                                           'artifact_key', 'artifact_sha256', 'quantisation', 'context_tokens', 'body_sha256',
                                           'manifest_sha256', 'signature', 'signing_key_version', 'placeholders', 'created_by',
                                           'created_at', 'updated_at', 'live_from', 'retired_at', 'version']),
  ('assistant_versions', 'detection-logic', ARRAY['body_text', 'manifest']),
  ('assistant_evaluations', 'internal', ARRAY['id', 'run_id', 'task', 'model_version_id', 'template_version_id', 'golden_set_sha256',
                                              'passed', 'data_as_of', 'created_by', 'created_at']),
  ('assistant_evaluations', 'detection-logic', ARRAY['result']),
  ('assistant_kill_state', 'internal', ARRAY['scope', 'engaged', 'reason_code', 'engaged_by', 'engaged_at', 'next_drill_due',
                                             'updated_at', 'version'])
) AS v(t, cls, cols) CROSS JOIN LATERAL unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'casework', 'assistant_drafts', c, v.cls::admin.data_classification
FROM (VALUES
  ('personal', ARRAY['output_enc', 'final_enc']),
  ('internal', ARRAY['id', 'case_id', 'report_id', 'task', 'mode', 'eval_run_id', 'language', 'status', 'refusal_code', 'template_version_id',
                     'model_version_id', 'input_sha256', 'accept_mode', 'edit_distance', 'edit_ratio', 'useful', 'target_kind',
                     'target_id', 'requested_by', 'requested_at', 'accepted_by', 'accepted_at', 'created_at', 'updated_at', 'version']),
  ('detection-logic', ARRAY['source_ids', 'cited_panels', 'grounding_result', 'scan_hits'])
) AS v(cls, cols) CROSS JOIN LATERAL unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

-- Retention (copy of the phase 27 insert format). Clock columns as the phase 23 rules require.
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'assistant_versions',    'R11', 'retirement', NULL,          'Signed model and template versions kept with their evaluations'),
 ('admin', 'assistant_evaluations', 'R11', 'age',        'created_at',  'Gate evidence: ten years (DATA-MODEL R11)'),
 ('admin', 'assistant_kill_state',  'R10', 'never',      NULL,          'Kill-switch state of each scope: kept without limit'),
 ('casework', 'assistant_drafts',   'R6',  'owner',      NULL,          'Follows its case (or the case of its report); drafts of an evaluation follow the golden case')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

---

## 2. DDD Implementation (Rust)

- **Crate placement:** pure rules in `domain::assistant` (akr-core, no IO): the closed task registry (`AssistantTask`), the draft state machine, the grounding rules (section 3.3), the placeholder grammar, the edit ratio and edit band, the `ai_involvement` value builder, the golden-gate counter and the suppression rule. Services in `services::assistant` (akr-api for the gateway, draft reads, accept, discard, kill switch, version import and promotion request, evaluation request and reads; akr-worker for generation, verification, evaluation, monitoring and version verification). Repositories in `db::assistant` (sqlx, `akr_app`). The model client lives in `crates/llm` (new crate, worker only): `LlamaServerClient` is the only code that opens a connection to the model server. Names not used by phases 00 to 27 are new in phase 28 (grep of docs/plan confirms): `AssistantTask`, `AssistantService`, `AssistantReads`, `AssistantKillSwitchService`, `AssistantVersionService`, `AssistantGenerator`, `GroundingVerifier`, `PromptBuilder`, `AssistantEvaluationService`, `AssistantMonitor`, `AssistantInvolvement`, `AssistantModelPort`, `LlamaServerClient`, and the `crates/llm` crate.
- **No `PromptTemplateService`:** a prompt template is an `admin.assistant_versions` row of kind `template`, imported, evaluated and promoted by `AssistantVersionService` like a model version (DR-290). One name per job, not one per table.
- **Task registry (closed in code, mirrored by the CHECKs of section 1):** `AssistantTask::{CaseSummary, ReportNarrative}`; each has a `tier` (3 for both, from research 29 9k "drafting aids are Tier 3"), `autonomy = Draft`, a `switch_key` (`assistant.case_summary.enabled` or `assistant.report_narrative.enabled`), a `kill_scope` (`task:case_summary` or `task:report_narrative`), `purposes` (`investigation` for the case summary, `report_preparation` for the report narrative), `languages` (`ar` or `en` for the summary, `both` for the narrative), `panels` (the phase 18 panel keys of section 3.2) and `max_output_sentences` (40, a constant). A value outside this enum is 422 `E_VALIDATION` at `/task` (GV-45).
- **Domain entities** (`Debug, Serialize, Deserialize, FromRow, ToSchema`):
  - `AssistantVersion { id, kind: VersionKind (model | template), task: Option<AssistantTask>, version_no, inventory_item_id, status: VersionStatus, refusal_code: Option<VersionRefusal>, artifact_key, artifact_sha256, quantisation, context_tokens, body_text, body_sha256, manifest: Value, manifest_sha256, signature, signing_key_version, placeholders: Vec<PlaceholderKind>, created_by, created_at, updated_at, live_from, retired_at, version }`. `VersionStatus` = importing, refused, registered, live, retired.
  - `AssistantEvaluation { id, run_id, task, model_version_id, template_version_id, golden_set_sha256, result: EvalResult, passed, data_as_of, created_by, created_at }`.
  - `AssistantKillState { scope: AssistantKillScope (Global | Task(AssistantTask)), engaged, reason_code: Option<KillReason>, engaged_by, engaged_at, next_drill_due, updated_at, version }`. `KillReason` is the phase 27 list (model_error, data_break, red_breach, regulator_request, other).
  - `AssistantDraft { id, case_id, report_id: Option<Uuid>, task, mode: DraftMode (live | evaluation), eval_run_id: Option<Uuid>, language: Language (ar | en | both), status: DraftStatus, refusal_code: Option<DraftRefusal>, template_version_id, model_version_id, input_sha256, source_ids: Vec<SourceRef>, cited_panels: Vec<String>, output: Option<VerifiedOutput> (sealed; decrypted only for the case team), grounding_result: GroundingResult, scan_hits, final_text: Option<Sealed<String>>, accept_mode: Option<AcceptMode>, edit_distance, edit_ratio, useful: Option<bool>, target_kind, target_id, requested_by, requested_at, accepted_by, accepted_at, created_at, updated_at, version }`. `SourceRef { label: String, kind: String, id: Uuid }` (kind is a closed list of section 3.2).
  - `VerifiedOutput { sentences: Vec<VerifiedSentence>, not_considered: Vec<String> }` and `VerifiedSentence { index, lang, text, refs: Vec<String>, state: SentenceState (kept | marked) }`. A `removed` sentence is never stored; it is counted in `GroundingResult`.
  - `GroundingResult { sentences_raw, kept, marked, removed, reasons: BTreeMap<String, u32>, not_considered_categories: Vec<String>, cited_panels: Vec<String> }` (counts and codes only; stored in `grounding_result`).
- **Create and update DTOs:**
  - `RequestDraft { task, case_id, report_id: Option<Uuid>, language: Option<Lang> }` (language required for `case_summary`: `ar` or `en`; for `report_narrative` it must be absent or `both`).
  - `AcceptDraft { final_text: Vec<TextField { lang, text }>, mode: AcceptMode (as_is | edited), useful: Option<bool> }`; `as_is` requires `final_text` equal to the output (edit distance 0), `edited` requires a different text; no value means 422 `mode_required`.
  - `DiscardDraft { useful: Option<bool>, reason_code: DiscardReason (not_useful | wrong_facts | not_needed | other) }`.
  - `ImportVersion { kind, task (templates), manifest: Value, manifest_sha256, signature, signing_key_version, artifact_key (models), body_text (templates), quantisation, context_tokens }`. A template is synchronous (201); a model is accepted with 202 and verified by job `assistant_version_verify`.
  - `RequestEvaluation { task, model_version_id, template_version_id }`; `PromoteRequest { model_version_id, template_version_ids: Vec<Uuid> (one per enabled task) }`.
  - `EngageAssistantKill { scope: AssistantKillScope, reason_code }`; `ReleaseAssistantKillRequest { scope, reason_text_code }`.
- **Repository traits** (domain layer; `&mut Tx` on writes; `If-Match` through `WHERE version = $expected`):
  - `AssistantDraftRepository`: `insert_requested(tx, NewDraft)`, `lock_draft(tx, id) -> AssistantDraft` (`FOR UPDATE`), `set_generated(tx, id, status, output_enc, grounding, scan_hits, input_sha256, source_ids, cited_panels, refusal)`, `set_accepted(tx, id, final_enc, mode, distance, ratio, useful, target, by, at, expected)`, `set_discarded(tx, id, useful, expected)`, `get(conn, id)`, `list_for_case(conn, case_id, keyset)`, `accepted_before(conn, case_id, decided_at) -> Vec<AssistantDraft>` (for `AssistantInvolvement`), `counts(conn, task, window, mode)` (monitoring, section 3.7).
  - `AssistantVersionRepository`: `insert_importing(tx, NewVersion)`, `lock_version(tx, id)`, `set_verified(tx, id, status, refusal)`, `set_status(tx, id, from, to, expected)`, `live_for(conn, task_or_model) -> Option<AssistantVersion>` (cached 30 s in process), `list(conn, filter)`.
  - `AssistantEvaluationRepository`: `insert(tx, NewEvaluation)`, `latest_passed(conn, task, model_id, template_id) -> Option<AssistantEvaluation>`, `list(conn, task)`.
  - `AssistantKillRepository`: `lock(tx, scope)` (`FOR UPDATE`), `lock_shared(tx, scope)` (`FOR SHARE`), `set_engaged(tx, scope, engaged, reason, by, expected)`, `all(conn)`.
- **Ports** (domain traits; implementations as named):
  - `AssistantModelPort`: `complete(&self, req: &CompletionRequest) -> Result<RawOutput, ModelCallError>` with `CompletionRequest { prompt: String, max_tokens: u32 (800), temperature: f32 (0.0), json_schema: Value (the output shape of section 3.2), timeout: Duration (`assistant.generate_timeout_s`) }`. `ModelCallError` = `Timeout | Unavailable | Refused`. Implemented by `LlamaServerClient` (worker only).
  - `AssistantInvolvement`: `for_case(tx, case_id, decided_at) -> AiInvolvement` (pure read of accepted drafts; called by phase 18 `RecordDecisionEffect`, section 3.4).
  - `SignatureVerifier` and `ArtifactStore` (phase 27 ports, reused unchanged for the `assistant-store` volume through a second instance with the root `/var/akr/assistant`).
- **Route table** (`| Method | Path | Scope | Description (cedar action) |`; literal paths `/assistant/kill-switch` and `/assistant/kill-switch/release-requests` are declared before `/assistant/drafts/{id}` so that they are not read as ids):

| Method | Path | Scope | Description (cedar action) |
|-|-|-|-|
| POST | `/assistant/drafts` | queue.work | Request a draft of one task for one case (or its report); 202 with the draft id in `requested`; no text in the body; `Idempotency-Key`; `X-Access-Purpose` (`assistant.draft.create`; API 3.15 row, owned) |
| GET | `/assistant/drafts/{id}` | queue.work | The draft: state, sentences with source labels, removed and marked counts, the `not_considered` line, the scan count; the text only to the case team; `X-Access-Purpose` (`assistant.draft.read`; added) |
| POST | `/assistant/drafts/{id}/accept` | queue.work | Accept an edited or an as-is draft: writes a case note or the report narrative; `If-Match` (draft version); `Idempotency-Key` (`assistant.draft.accept`; API 3.15 row, owned; phase 21 `report.edit` decides the narrative) |
| POST | `/assistant/drafts/{id}/discard` | queue.work | Discard a draft and record the useful-or-not answer; `If-Match`; `Idempotency-Key` (`assistant.draft.discard`; added) |
| POST | `/assistant/evaluations` | model.risk | Run the golden evaluation of one (task, template, model) triple; 202 with job `assistant_evaluate`; `Idempotency-Key` (`assistant.evaluation.run`; added) |
| GET | `/assistant/evaluations` | model.risk | Gate results: counts, rates, verdicts and the golden-set hash, newest first, cursor; no ids, no text (`assistant.evaluation.read`; API 3.15 row, owned) |
| GET | `/assistant/tasks` | model.risk, audit.read, dashboard.read | The task registry: tier, autonomy, switch state (read from settings), kill state, live template and model, gate status, next drill; no text (`assistant.monitor.read`; added) |
| POST | `/assistant/versions` | model.risk | Import a model (202, job `assistant_version_verify`) or a prompt template (201, synchronous); bad signature or manifest 422 `E_VALIDATION`; `Idempotency-Key` (`assistant.version.import`; added) |
| POST | `/assistant/versions/{id}/promote` | model.risk | Open change request `cr_material_validated` for one model and one template per enabled task; `If-Match` on the model version; `Idempotency-Key` (`assistant.version.promote_request`; added) |
| POST | `/assistant/kill-switch` | model.risk | Engage the global switch or one task's switch; immediate; step-up `approval`; `If-Match` on the scope row; `Idempotency-Key` (`assistant.kill.engage`; added) |
| POST | `/assistant/kill-switch/release-requests` | model.risk | Open change request `cr_material` that releases a switch; `Idempotency-Key` (`assistant.kill.release_request`; added) |
| GET | `/assistant/monitoring` | model.risk, audit.read, dashboard.read | Counts per task over 30 days: requested, ready, refused, accepted, edited, discarded, removed and marked sentences, useful answers; suppressed below `report.small_number_threshold` (`assistant.monitor.read`; added) |

- **Router functions:** `pub fn router() -> Router<AppState>` in `api::routers::assistant` (the twelve rows above). Every handler's first step is the flag check of section 3.1 step 1, so with `pack.assistant` FALSE every row answers 404.
- **Scope check (API section 2):** the Cedar permits of section 3.10 admit A1, A2 and M on the draft routes (all three hold `queue.work`), MRO on every `model.risk` route (`model.risk`), IA through `audit.read` and M and CH through `dashboard.read` on `GET /assistant/tasks` and `GET /assistant/monitoring` (the Cedar permit `assistant.monitor.read` names exactly MRO, IA, M and CH). No route admits a role that its permits do not name. A1 holds `queue.work` but has no permit on the report narrative accept, which the handler refuses with 403 `E_FORBIDDEN`.
- **utoipa schemas:** `DraftRequestAccepted`, `DraftView`, `SentenceView { index, lang, text, refs: [label], marked: bool }`, `GroundingCountsView`, `AcceptDraftRequest`, `AcceptedView`, `DiscardDraftRequest`, `EvaluationView`, `EvaluationRequest`, `EvaluationAccepted`, `TaskView`, `VersionView`, `ImportVersionRequest`, `ImportAccepted`, `PromoteRequest`, `PromoteAccepted`, `KillStateView`, `EngageAssistantKill`, `ReleaseAssistantKillRequest`, `ReleaseKillAccepted`, `MonitoringView`, `MonitorCountsView`. `DraftView` carries the text only when the caller is on the case team and the draft is `live`; a `live` draft in `requested` has no text.
- **AppState additions:** `api`: `assistant: Arc<AssistantService>`, `assistant_reads: Arc<AssistantReads>`, `assistant_kill: Arc<AssistantKillSwitchService>`, `assistant_versions: Arc<AssistantVersionService>`. `worker`: `assistant_gen: Arc<AssistantGenerator>`, `verifier: Arc<GroundingVerifier>` (pure), `evaluator: Arc<AssistantEvaluationService>`, `assistant_monitor: Arc<AssistantMonitor>`, `llm: Arc<dyn AssistantModelPort>` (`LlamaServerClient` to `llama`), `llm_eval: Arc<dyn AssistantModelPort>` (to `llama-eval`), `signatures: Arc<dyn SignatureVerifier>`, `artifacts: Arc<dyn ArtifactStore>` (the `assistant-store` mount). `ChangeTargetRegistry` in both binaries gains `AssistantPromotion` (kind `assistant_version`) and `AssistantSwitch` (kind `assistant_switch`); phase 04 `activate_change` is the only terminal effect used. `casework` (phase 18 `RecordDecisionEffect`) gains the `AssistantInvolvement` port.

---

## 3. Business Logic

### 3.1 Request path and the generation job (`AssistantService::request_draft`, `AssistantGenerator::run`; M-70, GV-49, SP-94)

**Route `POST /assistant/drafts`** (`queue.work`; Cedar `assistant.draft.create` on the case; `Idempotency-Key`; `X-Access-Purpose`). The model server is never called inside the request: the API writes the request and enqueues `assistant_generate`, so no browser waits on a model and no unverified text is returned to a browser.

1. Flag: read `pack.assistant` from `admin.feature_flags`. When FALSE, return 404 `E_NOT_FOUND` with the unknown-id body; no access event. This is step 1 of every `/assistant` route (the phase 26 pattern).
2. Parse the body. A task outside `AssistantTask` is 422 `E_VALIDATION` at `/task` (GV-45). `language` is required for `case_summary` (`ar` or `en`) and absent or `both` for `report_narrative`, otherwise 422 at `/language`.
3. Purpose: `X-Access-Purpose` is required (422 `E_PURPOSE_REQUIRED` when absent) and must be the task's purpose (`investigation` or `report_preparation`, phase 03 purpose codes); any other value is 422 `E_VALIDATION` with sub-code `purpose_not_for_task`.
4. Task switch: read the governed setting `switch_key`; FALSE gives 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `task_disabled`.
5. Live versions: `live_for(kind = model)` and `live_for(task)` must exist; missing gives 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `not_live`.
6. Kill state: `FOR SHARE` on the `global` row and on the task's row (lock 1 of section 3.15). Either engaged gives 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `kill_engaged`.
7. Case: `FOR SHARE` on `casework.cases` (lock 2). The case must be visible to the caller under phase 17 rules 1 to 5 (`services::visibility::build`) with the phase 18 `case_acl` clause; otherwise 404 `E_NOT_FOUND` with the identical masked body. A suspicion-tier case needs the step-up `record_open` (`StepUpService::consume`, 401 `E_STEPUP_REQUIRED` without it).
8. Cedar `assistant.draft.create` on `Akr::Case` with `Ctx { purpose, network }`; deny gives 403 `E_FORBIDDEN`.
9. Sealed case (a row in `casework.case_seals`, phase 18; the `sealed` flag of `GET /cases/{id}`): 409 `E_CASE_SEALED`. No draft is written to a sealed case.
10. `report_narrative` only: `report_id` must name a report of this case in status `draft` (phase 21) under `FOR SHARE` (lock 3); otherwise 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `report_not_draft`.
11. Insert `casework.assistant_drafts` with status `requested`, mode `live`, the live template and model ids, `requested_by` = the caller, `requested_at` = now; audit `assistant.draft_requested {draft_id, task}`; enqueue `assistant_generate {draft_id}` (coalesce key `assistant_generate:<draft_id>`) in the same transaction.
12. Respond 202 with `{ id, status: "requested" }`. The browser polls `GET /assistant/drafts/{id}` (every 2 s, at most 120 s) and renders nothing until the status is `ready`.

**Job `assistant_generate` (`AssistantGenerator::run(draft_id)`).** Runs as `akr-worker`; its only outbound call is to `llama` on zone `z2_model` (section 4.4).
1. Read the draft. Any status other than `requested` ends the job (a replay does nothing).
2. Kill state (global and task) read `FOR SHARE`. Engaged: step 13 with `kill_engaged`.
3. Switch read again; FALSE: step 13 with `task_disabled`.
4. Visibility re-check for the requester (roles, `case_acl`, tier) at this moment, with the predicate of step 7. Not visible: step 13 with `not_visible`; no record is read.
5. Retrieval (`PromptBuilder::retrieve`, section 3.2): the records of the task's panels that the requester may open, each with a label `R1` to `Rn`. The records are read through the repositories the panels use; there is no second query path and no evidence documents (Open doubts 6).
6. Prompt (`PromptBuilder::build`, section 3.2): the live template body, the records in delimited data blocks, and the output shape. `input_sha256` is the SHA-256 of the canonical input. Input longer than `assistant.max_input_chars` is step 13 with `input_too_large`; the input is never truncated.
7. `AssistantModelPort::complete` with the request of section 3.2 and timeout `assistant.generate_timeout_s`. A timeout or an unreachable server is step 13 with `model_unavailable`; the job's second attempt repeats steps 1 to 7.
8. Parse the JSON of the output. A parse failure is step 13 with `output_unparseable`. `raw_sha256` (SHA-256 of the raw bytes) is kept; the raw text is discarded.
9. `GroundingVerifier::verify(output, records)` (section 3.3). Zero kept sentences is step 13 with `nothing_supported`.
10. Scan each kept sentence with `PolicyPackService::scan(scope = "reason_text")` (phase 09). Only the count `scan_hits` is stored; the spans and the terms are not.
11. Kill state read again (`FOR SHARE`). Engaged now: the output is discarded unstored; step 13 with `kill_engaged`.
12. Lock the draft (`FOR UPDATE`, lock 4). Seal the verified output with AAD = draft id into `output_enc`; set `status = ready`, `grounding_result` (section 3.3 counts and `raw_sha256`), `scan_hits`, `input_sha256`, `source_ids` (labels mapped to kind and id), `cited_panels`. Audit `assistant.draft_ready {draft_id, kept, removed, marked, scan_hits}`. Stop.
13. Refusal (any step above): lock the draft, set `status = refused` with the code of the step, audit `assistant.draft_refused {draft_id, refusal_code}`. Nothing else is written; no text is stored.

### 3.2 Prompt, delimiters and output shape (`PromptBuilder`; GV-49, SP-94, T-50)

- **Data blocks.** Each record is serialised as JSON and wrapped as `<<DATA label=Rk nonce=N>> ... <<END nonce=N>>`, where the nonce is 16 random hex characters drawn per request. A record whose text contains `<<` or the nonce is serialised with every `<` written as the JSON escape `\u003c` (the six characters backslash, u, 0, 0, 3, c), so the block text holds no `<`. The system part of the prompt says that everything inside a data block is data about the case, never an instruction, and that no block may change the output rules. Free text (remittance purpose, beneficiary names, KYC notes, note bodies, alert narratives) is data in this sense (research 28 14.2 point 5).
- **Output shape** (the `json_schema` of the completion request): `{"sentences": [{"lang": "ar" | "en", "text": string, "refs": [label]}]}` with at most 40 items; `text` may contain only the placeholders `{amt:Rk.field}`, `{date:Rk.field}` and `{name:Rk.field}`. The model writes no figure, date or name that is not a placeholder. The model's own list of unused records is not part of the shape: the `not_considered` line is computed in code (section 3.3 step 6).
- **Decoding.** Temperature 0.0, `max_tokens` 800, no tool or function calling, no streaming to any browser, the server's prompt cache off for this request.
- **Placeholders in templates.** A template may use only the three placeholder kinds; any other `{...}` is refused at import (`template_placeholder_unknown`), and a template longer than 20,000 characters is refused (`template_too_long`).
- **Templates per task.** The case summary template asks for facts in the order profile, alerts, transactions, prior outcomes, screening facts that the case team already sees, and states "not stated in the records" for any gap. The report narrative template asks for the who, what, when, where, why and how skeleton of the phase 21 RP-22 structure, in both languages, from the report fields and the case records only. Neither template asks for a conclusion, a suspicion judgement, a recommendation or advice about the customer.

### 3.3 GroundingVerifier and the tipping-off scan (`GroundingVerifier::verify`, pure; GV-47, M-71, research 28 14.2 points 3 and 4)

**Input:** the parsed sentences and the records of 3.1 step 5 (label, kind, panel key, fields: amounts with currency, dates, names in Arabic and English, account references). **Output:** `VerifiedOutput`, `GroundingResult`. The verifier is deterministic: the same input gives the same output, and it reads no clock and no database.

1. A sentence whose `refs` are empty is removed (`no_source`); a `ref` that names no record is removed (`unknown_ref`); a sentence whose `lang` is not requested is removed (`lang_not_requested`).
2. Placeholders are resolved in order. `{amt:Rk.f}` becomes the record's amount and currency in canonical form (decimal string and ISO currency); `{date:Rk.f}` becomes the record's date in `YYYY-MM-DD`; `{name:Rk.f}` becomes the record's name in the sentence's language. A placeholder whose `Rk` is not in the sentence's `refs` is removed (`unknown_placeholder`). A placeholder whose record lacks the field makes the sentence `marked` (`source_incomplete`) and keeps the sentence visible with the mark.
3. On the text with the substitutions marked as spans, the verifier scans the rest of the sentence. Any digit outside a span is removed (`unsupported_figure`); any date-shaped token outside a span (`YYYY-MM-DD`, `DD/MM/YYYY`, a month name with a number, or a Hijri marker) is removed (`unsupported_date`); any currency code (IQD, USD) outside a span is removed (`unsupported_figure`); any token 2 to 4 long, folded with phase 09 `text::fold`, equal to a name of the supplied records outside a name span is removed (`unsupported_name`, a bare copy).
4. A sentence that passes 1 to 3 is `kept`; a sentence with a `source_incomplete` mark is `marked`. A marked sentence is never removed by the verifier, because the person decides; the accept of section 3.5 refuses any marked sentence that is still in the final text.
5. More than 40 sentences: the extra ones are removed (`over_limit`).
6. `not_considered`: the panel categories of the task that no kept sentence cites are listed by code (for example "screening: not considered in this summary"). The model's list is not used.
7. `GroundingResult` holds the counts `sentences_raw`, `kept`, `marked`, `removed`, the map `reasons` (code to count), `not_considered_categories`, `cited_panels`, and `raw_sha256`. It holds no text, no label value and no name.
8. Limit, stated for the bank: the verifier proves that every figure, date and name in the output is copied from a supplied record and that every sentence cites one. It cannot prove that the cited record supports the sentence's claim. That is what the human factual review of section 3.6 measures.
9. Tipping-off scan (step 10 of 3.1): the kept sentences are scanned with scope `reason_text` (phase 09 `PolicyPackService::scan`, the bank's term list). A hit is counted in `scan_hits`; the UI re-scans the displayed text and highlights the span. The term is never returned to the caller, and the accept of 3.5 refuses the final text while a hit remains (422 `E_VALIDATION`, sub-code `forbidden_term`). The phase 04 name `TippingOffTerms` is satisfied by this scan; no struct of that name is created (DR-279).

### 3.4 Draft state, saving, and `ai_involvement` (`AssistantInvolvement`; M-71, GV-48, IR-23, UX-45, DR-280)

- **States.** `requested` to `ready`, `refused` or `discarded`; `ready` to `accepted` or `discarded`. `refused`, `accepted` and `discarded` are final. The guard of section 1.3 enforces the edges. An `evaluation`-mode draft never reaches `accepted`.
- **No save without a person.** Nothing in the system writes a draft's text to a case, a report, a note, an export, a package, a notice or a SIEM event except the accept of 3.5. There is no save route, no export route and no package path for a draft (GV-40 pattern). The only route that returns text is `GET /assistant/drafts/{id}`, to the case team, for `live` drafts in `ready` or `accepted` state.
- **Shape of `casework.decisions.ai_involvement`** (phase 18 column, default `{"used": false}`; the shape is defined here):
  `{"used": false}` when no accepted draft of the case exists at the decision time; otherwise
  `{"used": true, "drafts": [{"draft_id": uuid, "task": "case_summary" | "report_narrative", "accept_mode": "as_is" | "edited", "edit_band": "none" | "minor" | "major", "template_version_id": uuid, "model_version_id": uuid}]}`.
  No text, no name and no ratio is in the value. `edit_band` is `none` when the distance is 0, `minor` when `edit_ratio` is at most `assistant.edit_minor_ratio`, and `major` otherwise.
- **Writer and time.** `AssistantInvolvement::for_case(tx, case_id, decided_at)` is called once, by phase 18 `RecordDecisionEffect` (the step of phase 18 section 3.7 that inserts `casework.decisions`) inside the decision transaction. It reads the accepted `live` drafts of the case with `accepted_at <= decided_at`. The value is written with the decision row and never updated (phase 18 decisions are immutable). This is an extension of phase 18 with one method call; phase 18's `{"used": false}` default is unchanged for cases with no accepted draft.
- **Reports.** A narrative accepted into a report of the case counts for the case decision whose `decided_at` follows it. Phase 21 report signing is not changed (Open doubts 7).
- **Readers.** Phase 18 `GET /cases/{id}/decisions` returns `ai_involvement` as it does today. QA population `assistant_assisted` (phase 19, section 1.2) selects decisions whose value has `used = true`.

### 3.5 Accept and discard (`AssistantService::accept`, `AssistantService::discard`; M-71, GV-48, UX-45)

**Route `POST /assistant/drafts/{id}/accept`** (`queue.work`; Cedar `assistant.draft.accept`; `If-Match` on the draft; `Idempotency-Key`; `X-Access-Purpose`).
1. Flag (404 when off); `If-Match` missing is 428 `E_PRECONDITION_REQUIRED`; mismatch 409 `E_VERSION_CONFLICT` with the current version in the body.
2. Body: `final_text` has one text per language of the draft (`both` needs `ar` and `en`; otherwise 422 `E_VALIDATION` at `/final_text`). `mode` is required: missing gives 422 `E_VALIDATION` sub-code `mode_required`; `as_is` with a text different from the output gives `as_is_mismatch`; `edited` with a text equal to the output gives `edited_unchanged`. A bulk or "accept all" body is not a shape the route accepts: one draft per call.
3. Kill state `FOR SHARE` on the global and the task row (lock 1): engaged gives 409 `kill_engaged`.
4. Case `FOR UPDATE` (lock 2; the note write of step 10 takes the same row). Visibility and purpose exactly as section 3.1 step 7 (masked 404 for a non-team caller); the suspicion-tier step-up `record_open` applies.
5. Cedar `assistant.draft.accept` on `Akr::Case` (A1, A2 and M only; section 3.10). For `report_narrative` the phase 21 Cedar `report.edit` on the report is checked by the phase 21 handler in step 10; a deny is 403 `E_FORBIDDEN`. A1 cannot accept a narrative.
6. For `report_narrative`, first the report row `FOR UPDATE` (lock 3 of section 3.15; `report_id` is read from the draft without a lock). Then the draft `FOR UPDATE` (lock 4): status must be `ready`; otherwise 409 `E_NOT_ALLOWED_IN_STATE` with sub-code `not_ready` (`requested`, `refused`) or `already_decided` (`accepted`, `discarded`).
7. Evidence before accept (M-72, GV-42 part): every key of `cited_panels` must have a `casework.opened_views` row of this actor on this case (`view_kind = 'panel'`, phase 18). A missing key is 409 `E_NOT_ALLOWED_IN_STATE` with sub-code `evidence_not_opened` and the missing panel keys in the body (keys only, no content).
8. Final text: no sentence of the output with state `marked` may appear in the final text (422 `E_VALIDATION`, sub-code `unsupported_sentence_kept`); a case summary is at most `casework.note_max_chars` (phase 18) minus the length of the first line that step 10 adds, each narrative field at most 8,000 characters (phase 21 RP-21); `PolicyPackService::scan(scope = "reason_text")` on each text: a hit is 422 `E_VALIDATION` at the field, sub-code `forbidden_term`, with the spans and never the term.
9. Edit measure (server side only): Levenshtein distance on the `text::fold` form, summed over the languages; `edit_ratio = distance / max(1, length of the output)`, capped at 1. `accept_mode` is `as_is` when the distance is 0 and `edited` otherwise.
10. Write the target in this transaction:
    - `case_summary`: `NoteService::post(ctx, case_id, CreateNote { body })` (phase 18 section 3.3, `case.note.write`, with its checks, including `E_CASE_SEALED`). The note body is the accepted text with one line first: "Summary drafted by assistant; accepted by the author of this note; draft <draft_id>". The note author is the accepting person, so the saved text names who is responsible. `target_kind = note`, `target_id` = the note id.
    - `report_narrative`: `ReportService::update_draft` (phase 21 section 3.2, the PATCH of `/reports/{id}`) with `narrative.ar` and `narrative.en`, under the phase 21 lock order (lock 3 of section 3.15). The report must still be in status `draft` (409 `E_NOT_ALLOWED_IN_STATE`, sub-code `report_not_draft`); phase 21 `report.edit`, the scan of phase 21 and its length checks apply in full. `target_kind = report`, `target_id` = the report id.
11. Update the draft: `status = accepted`, `final_enc` (sealed, AAD = draft id), `accept_mode`, `edit_distance`, `edit_ratio`, `useful` when given, `target_kind`, `target_id`, `accepted_by` = the caller, `accepted_at` = now(); version + 1.
12. Audit `assistant.draft_accepted {draft_id, task, accept_mode, edit_band, target_kind}`. The audit carries no text and no distance value.
13. Respond 200 with `{ target_kind, target_id, accept_mode, edit_band }`.

**Route `POST /assistant/drafts/{id}/discard`** (`queue.work`; Cedar `assistant.draft.discard`; `If-Match`; `Idempotency-Key`): flag; `If-Match`; visibility and purpose as section 3.1 step 7; the draft must be `ready` or `requested` (a discard of a `requested` draft cancels it: the job of section 3.1 stops at its step 1). Sets `status = discarded`, `useful` and the reason code of the body (`not_useful`, `wrong_facts`, `not_needed`, `other`; the reason code is audited, not stored as text). Audit `assistant.draft_discarded {draft_id, reason_code}`.

### 3.6 Evaluation gate (`AssistantEvaluationService`; M-73, GV-50, T-49, research 21 12e; DR-283)

**Route `POST /assistant/evaluations`** (MRO; `model.risk`; `Idempotency-Key`) accepts `{task, model_version_id, template_version_id}` (both versions `registered` or `live`) and enqueues `assistant_evaluate` with coalesce key `assistant_evaluate:<task>:<model_id>:<template_id>`.

**Who may run it and under which access (decided here).** The initiator is an MRO, who holds no case access and never reads case text (section 3.13). The job runs as the system principal with purpose `qa_review` (phase 03), reads the golden cases through the same retrieval as section 3.1, and writes counts only. No person reads a golden text in the run. Human factual review is a phase 19 QA sample (step 4), drawn and reviewed by the QA roles of phase 19 under their own access.

**Server and principal (lead review).** A candidate model is served by `llama-eval` (section 4.4), never by the production `llama`, which holds only the live model (and none before the first go-live). The job first checks that `llama-eval` reports the candidate's `artifact_sha256`; a mismatch or no answer ends the run with `passed = false` and reason `eval_server_mismatch`. Evaluation drafts carry `requested_by` = the run's initiator (a record of who asked) and `eval_run_id` = the run; their retrieval runs as the system principal with purpose `qa_review`, never with the initiator's visibility, and the sealed-case refusal of section 3.1 step 9 applies to live drafts only (golden cases are sealed by definition).

**Golden set (`GoldenSet::build(task)`):**
1. Cases closed and sealed, with a phase 19 QA review whose `overall` is `pass` or `pass_with_comment` (population `closed_cases` or `targeted_repeat`). For `report_narrative`, the case also holds a phase 21 report in a filed state.
2. Sorted by case id; the first `assistant.gate_min_items` items per language are taken after a deterministic stratification by family and language. `golden_set_sha256` is the HMAC-SHA256 of the sorted ids, keyed as section 3.13 states. Fewer items than `assistant.gate_min_items` is a run with `passed = false` and reason `too_few_items`; the run still records its counts.
3. Each golden item is generated exactly as a live draft (section 3.1 steps 5 to 12) with mode `evaluation`, so the pipeline tested is the pipeline used. Evaluation drafts are hidden: no route returns them, and they are never `accepted`.
4. Human review: the QA lead draws the phase 19 population `assistant_golden` for the run's evaluation drafts (`POST /qa/samples` of phase 19, `n = assistant.gate_min_items`, the drafts whose `eval_run_id` is the run); each item has `source_kind = assistant_draft`, `source_id` = the draft id, `case_id` = the golden case, and phase 19 `GET /qa/samples/{id}/item` shows its reviewer the verified output with its cited records under the time-boxed grant (phase 19 section 3.8 step 7 and its section 7+ block). Reviews use the phase 19 rubric; each `critical` finding is counted.

**Metrics (counts and rates only; no id, no text in `result`):** `items`, `refused` (by code), `sentences_raw`, `removed_rate`, `marked_rate`, `scan_hit_rate` (reported, not a gate), `coverage` (the share of the mandatory facts of the record that appear in the kept output: for the case summary the alert priority classes and dates, the transaction amounts and dates of the case panels and the party names; for the narrative the subject names, account references, amounts and dates of the report), `by_language` (counts, suppressed below `report.small_number_threshold`), `reviewed` and `critical`.

**Gates (a go-live needs `stage = final` and `passed = true`):**
- `items >= assistant.gate_min_items` (governed, floor 200) and at least 50 items per language (design value, Open doubts 3);
- `refused / items <= 0.05` (design value);
- `removed_rate <= assistant.gate_max_removed_rate` (governed, floor max 0.05);
- `coverage >= 0.90` (design value, Open doubts 3);
- human: `reviewed >= assistant.gate_min_items` and `critical = 0` (research 21 12e: with zero critical errors in n reviewed items the 95 per cent upper bound is about 3 over n; 200 items bound it near 1.5 per cent; the bank may raise n).

**Two stages.** The job writes `result.stage = "deterministic"` when the generation of all items ends, with `passed` set to the deterministic gates only. The daily job `assistant_monitor` (section 3.7) writes a second row with the same `run_id` and `stage = "final"` for each run whose QA review of `assistant_golden` (the samples whose draft carries the run's `eval_run_id`) is complete, with `passed` over all gates. `AssistantEvaluationService::latest_passed(task, model, template)` reads only `final` rows. A run is never edited: a new run is a new row (guard 1.3).

**Re-run rule.** Any change of the live model or of a template (section 3.9) needs a new `final` pass for every enabled task before promotion; the promotion of section 3.9 names the rows it relies on.

### 3.7 Monitoring (`AssistantMonitor`, route `GET /assistant/monitoring`, job `assistant_monitor`; M-73, DR-286)

- **Counts** over the last 30 bank days, `live` drafts only, per task: `requested`, `ready`, `refused` (by the code groups `kill_or_switch`, `visibility`, `model` (with `input_too_large`), `output`, `grounding`), `accepted` (`as_is`, `edited`), `discarded`, `useful_yes`, `useful_no`, sentences `kept`, `removed`, `marked`, and the edit bands `none`, `minor`, `major`.
- **Suppression.** Every count below `report.small_number_threshold` (phase 21, never below 3) is returned as suppressed ("fewer than 3" in words on the screen). A rate is computed only from unsuppressed counts. Counts cover every tier (the phase 24 `str_rate` pattern, DR-270). There is no split by report type, by outcome, by decider, by branch or by day, so a reader cannot learn which case or which decision a count came from.
- **Job `assistant_monitor`** (`schedule.assistant_monitor`, 05:20 bank time, after `model_monitor` of phase 27). Steps: (1) finalise every `deterministic` evaluation whose `assistant_golden` review is complete (section 3.6); (2) compute the 30-day counts; (3) for each task, if the removed share `removed / (kept + removed)` is above `assistant.removed_rate_red`, or the discard share `discarded / (ready + discarded)` is above `assistant.discard_rate_red`, and the base count is at least the threshold, open one phase 24 register finding with source `assistant_monitor`, `source_key = assistant:<task>:<YYYY-MM>` (one finding per task and month), title and body holding the metric name and band only; the owner is the owner of the task's live template inventory item (the live model item's owner when the task has none; phase 27 uses the family owner, which the assistant does not have). Audit `assistant.monitor_breach {task, metric, band, register_id}`.
- **Panel.** `GET /assistant/monitoring` returns the counts of the last computed run (computed on demand with a one-minute cache when the job has not run that day). It never returns a draft id, a case id, a party id or a text.

### 3.8 Kill switch (`AssistantKillSwitchService`; SP-95, GV-20 pattern of phase 27; DR-281)

Scopes: `global` (both tasks) and `task:case_summary` or `task:report_narrative`. The state is one row per scope in `admin.assistant_kill_state`.

1. **Engage** (`POST /assistant/kill-switch`; Cedar `assistant.kill.engage`; MRO; `Idempotency-Key`; `If-Match` on the scope row: 428 without it, 409 on a mismatch). Step-up: `StepUpService::consume(tx, ctx, header, AssistantKill, "assistant_kill:<scope>")` with the constant `AssistantKill { name: "assistant.kill_switch", class: approval }` (a closed class of phase 01; a missing or reused assertion is 401 `E_STEPUP_REQUIRED`). Lock the scope row `FOR UPDATE` (lock 1, alone). Set `engaged = true`, `reason_code`, `engaged_by`, `engaged_at`. Audit `assistant.kill_engaged` (SIEM flag true). No change request is opened: the fall-back to manual drafting must be immediate, as in phase 27 DR-266.
2. **Effect.** From the next step of any request or job, drafts are refused with `kill_engaged`, accepts are refused with `kill_engaged`, and the phase 18 and phase 21 screens keep their manual flows (the assistant block is hidden, section 5). A draft already in `ready` stays readable and can be discarded, but cannot be accepted while the switch is engaged.
3. **Two-person rule (SP-95).** Engaging is one MRO, immediately, because the stop must not wait for a second person; releasing is two-person by change request. This is the decision of DR-281 and it differs from the literal wording of research 28 14.2 point 6 (Open doubts 8).
4. **Release request** (`POST /assistant/kill-switch/release-requests`; Cedar `assistant.kill.release_request`; MRO): the scope must be engaged (409 `E_NOT_ALLOWED_IN_STATE`, sub-code `not_engaged`). It opens change request `assistant_switch` with `object_key = kill:<scope>`, class `material`, route `cr_material` (phase 04: parallel M and CHK approval; the maker never approves, `E_SELF_APPROVAL`). `check_before_decision`: when the reason is `red_breach`, the open `assistant_monitor` findings of the task must be closed (409 `breach_open`). `activate`: sets `engaged = false` inside the transaction flag `akr.assistant_release = 'on'` (guard 1.3), sets `next_drill_due = today + assistant.kill_drill_months`, and audits `assistant.kill_released` (SIEM flag true). The release change's evidence is the drill record (Open doubts 9).
5. **Calendar.** The phase 24 review calendar reads `next_drill_due` as a derived row of kind `assistant_kill_drill` (amber 30 days before, red on the due date). It is a row of the existing calendar, not a second one (section 4.5).
6. **No re-run.** A release does not re-generate refused drafts. A `refused` draft stays refused.

### 3.9 Versions, validation, promotion and go-live (`AssistantVersionService`; GV-03, GV-04, GV-09, GV-32, GV-51, T-51, TS-17; DR-275, DR-284)

**Import of a model** (`POST /assistant/versions`, kind `model`, MRO, `Idempotency-Key`, 202):
1. Flag (404 when off). The body carries `manifest`, `manifest_sha256`, `signature`, `signing_key_version`, `artifact_key`, `quantisation`, `context_tokens`.
2. The manifest's canonical bytes (the canonical form of phase 27 section 3.3 step 2) hash to `manifest_sha256` (422 `E_VALIDATION`, sub-code `manifest_invalid` otherwise). The manifest must hold the GV-51 provenance fields: `publisher`, `base_model` (name and version), `licence` (identifier and the SHA-256 of the licence text), `provenance` (source of the weights and the date received), `training_data_statement_sha256`, `architecture`, `languages` (must include `ar` and `en`), and `chat_template_sha256`. Missing fields are `manifest_invalid`.
3. Ed25519 verification of `signature` over the canonical manifest with the verify-only key `keys/model-artifact/v<N>.pub` (the phase 27 key purpose; no new key is created; DR-261). Failure is 422 `E_VALIDATION`, sub-code `signature_invalid`, and nothing is written.
4. In one transaction: insert the inventory item (phase 04 `InventoryService::create`, kind `llm_component`, tier 3, owner = caller, `object_id` = the new version id, `tier_basis` filled with the seven tiering answers of phase 27 section 3.1) and the version row with status `importing`; enqueue `assistant_version_verify {version_id}`; audit `assistant.version_imported`. Respond 202.

**Verification** (`assistant_version_verify`, worker): (a) `ArtifactStore::read_sha256(artifact_key)` must equal `artifact_sha256` (`sha_mismatch` otherwise); (b) `llama-eval` (section 4.4), started by the operator with this version, reports the artifact's SHA-256 equal to `artifact_sha256` and answers `/health` with the expected `context_tokens` (`model_load_failed` otherwise; the job retries for at most one hour while the server is not started); (c) status `importing` to `registered`, or to `refused` with the code. Audit `assistant.version_verified` or `assistant.version_refused` (SIEM true). A bad artifact is never loaded by the production server.

**Import of a prompt template** (`POST /assistant/versions`, kind `template`, 201, synchronous): the body carries `task`, `body_text`, `manifest` (`{task, body_sha256, placeholders}`), `manifest_sha256` and `signature` (signed by the bank's model team with the same verify-only key purpose). Checks: the body is at most 20,000 characters (`template_too_long`); the only placeholder kinds are `amt`, `date` and `name` (`template_placeholder_unknown`); `PolicyPackService::scan(scope = "reason_text")` finds no hit (`template_scan_hit`); the signature verifies (`signature_invalid`). The row is inserted as `registered` with its inventory item (kind `prompt_template`, tier 3) in one transaction; audit `assistant.version_imported`.

**Validation pack** (phase 24 `POST /inventory/{id}/validation-packs`, MRO, validator never the owner, GV-32). `ValidationPackService::issue` takes the assistant branch for kinds `llm_component` and `prompt_template`, with the seven parts of phase 24 section 3.6 given their assistant meaning: P1 conceptual soundness: the task, the template hash, the tier basis and the record kinds the template may use; P2 data: the record kinds of the task's panels and the open `admin.dq_issues` of their source tables; P3 implementation: the deterministic fixture tests of the verifier (their pass and fail counts) and the latest `final` evaluation of the item's triples; P4 outcomes: the live counts of the last 30 days (counts only, section 3.7); P5 sensitivity: the latest `final` evaluation of the previous live version, `not_run` when none; P6 QA: the phase 19 reviews of population `assistant_assisted` for the item's task (pass and fail counts); P7 benchmark: the candidate's latest `final` evaluation against the live version's. The per-bank delta (`bank_delta`) lists the switch values and the gate floors of the task. The verdict is computed by the phase 24 rule; `pass` moves the inventory dates through phase 04; a pack older than `validation.evidence_max_days` is `E_EVIDENCE_STALE`.

**Promotion request** (`POST /assistant/versions/{id}/promote`, MRO, `If-Match` on the model version, `Idempotency-Key`; body names one template version per task):
1. The model version is `registered` or `live` (409 `not_registered`); each named template is `registered` or `live` and is for its task (422 `E_VALIDATION` at `/template_version_ids` otherwise); a request in which the model and every named template are already live is 409 `nothing_to_promote` (a template-only change keeps the live model: its artifact cannot be imported twice).
2. Inventory items of the model and of every named template are `approved_for_use` (409 `inventory_not_approved`); the validator of each is not its owner (phase 04 CHECK).
3. For every task and each (task, model, template) triple: a latest `final` evaluation exists (409 `evaluation_missing`, the task without an evaluation, which is the refusal of a go-live without a passing evaluation) and is `passed` (409 `gate_not_met`).
4. A validation pack with verdict `pass` issued after that evaluation exists for each item (409 `pack_missing`).
5. Open change request `cr_material_validated` (target `AssistantPromotion`, object key `assistant:<model version id>`, body listing the triples); audit `assistant.promotion_requested {change_request_id}`. The versions keep their status until the change is activated.

**Activation** (`AssistantPromotion::activate`, the phase 04 `activate_change` effect; `check_before_decision` repeats steps 1 to 4 of the request): in one transaction, lock the versions (lock 5 of section 3.15), set the transaction flag `akr.assistant_promotion = 'on'`, set to `retired` with `retired_at` only the live versions being replaced (the live model when another model is named, the live template of a task when another template is named), leave a named version that is already live unchanged, and set the new versions to `live` with `live_from = now()`; audit `assistant.promoted {model_version_no, template_version_nos}`. A version never goes from `retired` back to `registered` (guard 1.3): a rollback is a new promotion of another registered version through a new change request, and the emergency route is not allowed (`emergency_allowed = false`, Open doubts 10).

**Switch-off of the pack.** Setting the flag `pack.assistant` to FALSE is refused while any `live` version exists (409 `E_NOT_ALLOWED_IN_STATE`, sub-code `assistant_versions_live`), the phase 26 rule for `pack.islamic`. The bank's fast way off is the kill switch of section 3.8, which the flag does not replace.

**Governance read.** `GET /assistant/tasks` (section 2) lists, per task, the tier, the autonomy `draft`, the switch read from settings, the kill state, the live template and model versions with their numbers, the status of the last `final` evaluation and the next drill date.

### 3.10 Cedar actions and the file `C2_assistant.cedar` (phase 03 range; `C2` is free; grep of docs/plan confirms)

**Schema additions** (appended to `schema.cedarschema`; no existing entity or attribute changes; `Case`, `System`, `Report` and `Ctx` exist in phases 03, 17, 18 and 21): actions on `Akr::Case`: `assistant.draft.create`, `assistant.draft.read`, `assistant.draft.accept`, `assistant.draft.discard`. Actions on `Akr::System`: `assistant.monitor.read`, `assistant.evaluation.run`, `assistant.evaluation.read`, `assistant.version.import`, `assistant.version.promote_request`, `assistant.kill.engage`, `assistant.kill.release_request`. All take `principal: User` and `context: Ctx`.

**File `C2_assistant.cedar`** (copy-paste ready; one role per `@role` and per policy; `principal.roles.contains` only; no `containsAny`; step-up is a handler check; the case-team membership and the visibility are the handler's (`services::visibility::build` and `case_acl`), as phase 18 does for its own note actions; the report narrative permit is phase 21's `report.edit`):

```cedar
// C2_assistant.cedar (phase 28). Default deny: no policy below names a role that its route's scopes do not admit (API 3.15 rows).
// Case-team membership, the visibility predicate, the purpose and the suspicion-tier step-up are service checks, not conditions here.

@id("assistant.case.a1") @role("A1")
permit (principal, action in [Akr::Action::"assistant.draft.create", Akr::Action::"assistant.draft.read",
        Akr::Action::"assistant.draft.accept", Akr::Action::"assistant.draft.discard"], resource is Akr::Case)
when { principal.roles.contains("A1") };

@id("assistant.case.a2") @role("A2")
permit (principal, action in [Akr::Action::"assistant.draft.create", Akr::Action::"assistant.draft.read",
        Akr::Action::"assistant.draft.accept", Akr::Action::"assistant.draft.discard"], resource is Akr::Case)
when { principal.roles.contains("A2") };

@id("assistant.case.m") @role("M")
permit (principal, action in [Akr::Action::"assistant.draft.create", Akr::Action::"assistant.draft.read",
        Akr::Action::"assistant.draft.accept", Akr::Action::"assistant.draft.discard"], resource is Akr::Case)
when { principal.roles.contains("M") };

@id("assistant.system.mro") @role("MRO")
permit (principal, action in [Akr::Action::"assistant.monitor.read", Akr::Action::"assistant.evaluation.run",
        Akr::Action::"assistant.evaluation.read", Akr::Action::"assistant.version.import",
        Akr::Action::"assistant.version.promote_request", Akr::Action::"assistant.kill.engage",
        Akr::Action::"assistant.kill.release_request"], resource is Akr::System)
when { principal.roles.contains("MRO") };

@id("assistant.monitor.ia") @role("IA")
permit (principal, action == Akr::Action::"assistant.monitor.read", resource is Akr::System)
when { principal.roles.contains("IA") };

@id("assistant.monitor.m") @role("M")
permit (principal, action == Akr::Action::"assistant.monitor.read", resource is Akr::System)
when { principal.roles.contains("M") };

@id("assistant.monitor.ch") @role("CH")
permit (principal, action == Akr::Action::"assistant.monitor.read", resource is Akr::System)
when { principal.roles.contains("CH") };
```

**Scope check against the permits:** the draft routes admit A1, A2 and M through `queue.work`, every one of which has a permit above; `GET /assistant/tasks` and `GET /assistant/monitoring` admit MRO (`model.risk`), IA (`audit.read`), M and CH (`dashboard.read`), each with its `assistant.monitor.read` permit; the other `model.risk` routes admit MRO only, with permits above.

**Bundle tests** (`tests/policy/c2_assistant`): A1, A2 and M create, read, accept and discard on a case (the handler then decides the team); A1 on a report narrative accept is refused by phase 21 `report.edit` (403 `E_FORBIDDEN`); CH, IA and A1 on `assistant.evaluation.run`: refused (no permit, no scope); IA on `assistant.kill.engage`: refused; MRO on `assistant.draft.read` on any case: refused (no permit, and `model.risk` admits no draft route); MRO on `assistant.monitor.read`: permitted.

### 3.11 Errors (`AssistantError`, `thiserror`; API section 6 codes; sub-codes in `errors[].code` or the body's `reason`)

| Code (HTTP) | Sub-code | Raised by |
|-|-|-|
| `E_NOT_FOUND` (404) | none (identical body for a missing, masked or flagged-off resource) | every route while `pack.assistant` is FALSE (3.1 step 1); a non-team caller (3.1 step 7, 3.5 step 4) |
| `E_VALIDATION` (422) | `task_unknown` at `/task`, `purpose_not_for_task`, `mode_required`, `as_is_mismatch`, `edited_unchanged`, `unsupported_sentence_kept`, `forbidden_term` (spans in `errors[]`, never the term), `manifest_invalid`, `signature_invalid`, `template_placeholder_unknown`, `template_too_long`, `template_scan_hit` | 3.1, 3.5, 3.9 |
| `E_PURPOSE_REQUIRED` (422) | none | 3.1 step 3, 3.5 step 4 |
| `E_NOT_ALLOWED_IN_STATE` (409) | `task_disabled`, `not_live`, `kill_engaged`, `not_ready`, `already_decided`, `evidence_not_opened` (panel keys in the body), `report_not_draft`, `not_registered`, `nothing_to_promote`, `inventory_not_approved`, `evaluation_missing`, `gate_not_met`, `pack_missing`, `breach_open`, `not_engaged`, `assistant_versions_live` | 3.1, 3.5, 3.6, 3.8, 3.9 |
| `E_CASE_SEALED` (409) | none | 3.1 step 9, 3.5 step 10 (through phase 18 `NoteService`) |
| `E_FORBIDDEN` (403) | none | Cedar denies (3.1 step 8, 3.5 step 5) |
| `E_SCOPE` (403) | none | the route scope does not admit the caller's roles |
| `E_SELF_APPROVAL` (403) | none | a maker approves its own change request (3.8 step 4) |
| `E_STEPUP_REQUIRED` (401) | none | suspicion-tier request, accept, read (`record_open`); kill engage (`approval`) |
| `E_PRECONDITION_REQUIRED` (428), `E_VERSION_CONFLICT` (409) | none | `If-Match` on accept, discard, engage, promote |
| `E_IDEMPOTENCY_REQUIRED` (400), `E_IDEMPOTENCY_REUSE` (422), `E_IDEMPOTENCY_IN_FLIGHT` (409) | none | every mutation of section 2 |
| `E_UNAVAILABLE` (503) | none | the artifact volume or the key service is down at import; the request is retried and nothing is written |

Generation refusals are not HTTP errors: they are the `refusal_code` of section 1.2 on the draft, read by the poll of section 3.1 step 12.

### 3.12 Audit events emitted (one per mutation; payloads hold ids, codes and counts only)

| Kind | Resource | SIEM flag | Payload |
|-|-|-|-|
| `assistant.draft_requested` | AssistantDraft | false | `draft_id`, `task` |
| `assistant.draft_ready` | AssistantDraft | false | `draft_id`, `kept`, `removed`, `marked`, `scan_hits` |
| `assistant.draft_refused` | AssistantDraft | false | `draft_id`, `refusal_code` |
| `assistant.draft_accepted` | AssistantDraft | false | `draft_id`, `task`, `accept_mode`, `edit_band`, `target_kind` |
| `assistant.draft_discarded` | AssistantDraft | false | `draft_id`, `reason_code` |
| `assistant.version_imported` | AssistantVersion | false | `kind`, `version_no`, `artifact_sha256` or `body_sha256` |
| `assistant.version_verified` | AssistantVersion | false | `kind` |
| `assistant.version_refused` | AssistantVersion | true | `kind`, `refusal_code` |
| `assistant.evaluation_run` | AssistantVersion (model) | false | `task`, `model_version_id`, `template_version_id`, `items` |
| `assistant.evaluation_recorded` | AssistantVersion (model) | false | `task`, `stage`, `passed` |
| `assistant.promotion_requested` | AssistantVersion (model) | false | `change_request_id` |
| `assistant.promoted` | AssistantVersion (model) | false | `model_version_no`, `template_version_nos` |
| `assistant.monitor_breach` | System (scope) | false | `task`, `metric`, `band`, `register_id` |
| `assistant.kill_engaged` | System (scope) | true | `scope`, `reason_code` |
| `assistant.kill_release_requested` | System (scope) | false | `change_request_id` |
| `assistant.kill_released` | System (scope) | true | `scope` |

The kinds are added to the phase 02 catalogue with their SIEM flags. A read writes an access event (phase 02 `access_event`, action `view`) for a draft text only; the list of tasks and the monitoring panel write no audit row. No payload holds a prompt, an output, a final text, a record value, a name, an amount, a span or a term; a test greps the audit detail of a fixture run for the fixture's output text and expects no match.

### 3.13 Tipping-off review (what a person outside a case's access list can learn; the answer must be "nothing")

| Surface | Who sees it | What they can learn about a case or party |
|-|-|-|
| Draft text (`GET /assistant/drafts/{id}`) | A1, A2, M on the case (phase 17 rules and the `case_acl` clause); a suspicion case needs `record_open` | Nothing outside the team: any other caller gets the masked 404 |
| Accept and discard | The same team; the note and the narrative follow phase 18 and phase 21 visibility | Nothing outside the team |
| Case decision `ai_involvement` (phase 18) | Phase 18 readers of `GET /cases/{id}/decisions` (unchanged) | The draft id, the task and the edit band; no text. Visible only where the decision already is |
| Task registry and monitoring panel | MRO, IA, M, CH | Switches, versions and counts over 30 days across every tier, suppressed below the threshold; no split by report type, decider, branch or day, and no id. A count cannot name a case |
| Evaluation results | MRO (`model.risk`) | Counts and rates over golden items; no id, no text. The golden set is named by `golden_set_sha256`, an HMAC keyed by an OpenBao KV secret that only the worker reads, so MRO cannot test whether a given case is in the set (Open doubts 11) |
| Kill-switch state and events | MRO, IA, M, CH; the flagged audit kinds go to the SIEM | The scope, the time and a reason code from a closed list; no case, no party, no score |
| Register finding `assistant_monitor` | Phase 24 register roles | A task, a metric name and a band; no case |
| D4 AI governance tab (phase 27 screen, section 5) | The same roles as the reads above | The composition of the rows above; nothing else |
| Export, package, notice, SIEM | None: no draft, no evaluation text and no count below the threshold leaves through any of them (the phase 25 exclusion of section 4.5) | Nothing |

### 3.14 Settings used

Keys of section 1.3 only: `assistant.case_summary.enabled`, `assistant.report_narrative.enabled`, `assistant.gate_min_items`, `assistant.gate_max_removed_rate`, `assistant.qa_sample_percent`, `assistant.kill_drill_months`, `assistant.discard_rate_red`, `assistant.removed_rate_red`, `assistant.max_input_chars`, `assistant.generate_timeout_s`, `assistant.edit_minor_ratio`, `schedule.assistant_monitor`. Reused: `report.small_number_threshold` (phase 21, never below 3), `casework.note_max_chars` (phase 18), `validation.evidence_max_days` and `review.rules_months` (phase 24). The design constants of this phase (not settings, listed in Open doubts 3 and 4): the coverage floor 0.90, the refusal ceiling 0.05 of the gate, the per-language minimum of 50 items, the 40-sentence limit and the 800-token output limit.

### 3.15 Lock order

Taken in this order, in every path that takes more than one:
1. `admin.assistant_kill_state` (the scope row): `FOR SHARE` in generation, request and accept; `FOR UPDATE` in engage and release.
2. `casework.cases` (the case row): `FOR SHARE` in request and generation; `FOR UPDATE` in accept, because `NoteService::post` takes the same row.
3. `rpt.reports` (the report row): `FOR SHARE` in request; `FOR UPDATE` in the narrative accept, under phase 21's own lock order.
4. `casework.assistant_drafts` (the draft row): `FOR UPDATE` in the generation's final transition, in accept and in discard.
5. `admin.assistant_versions` rows: the candidate model first, then the previous live rows by id, in activation.
6. `admin.inventory_items` (read `FOR SHARE` for the use gate; written only by phase 04).
7. `admin.registers` (insert only, for a breach).

Phase 04 change paths take the workflow instance lock first (phase 04 order 1), before any of the above. Each step of the generation job is its own transaction, and no lock is held across the model call. Engage takes lock 1 alone.

### 3.16 Agent containment (T-50)

No agent is built in this phase. A draft is one completion with no tools: the model's output is data, the verifier decides what survives, and the only writes it causes are the draft's own status transitions. The worker has no outbound route except the model server on zone `z2_model`. Any later agentic feature is a change request on route `cr_material_validated` and must meet these rules before it ships: least-privilege, allow-listed action names; every action checked by Cedar with the human principal's context, never the service's; a runtime policy check before each action; timeouts; the kill switch of section 3.8 covering it; a full action log in the audit trail; no write to a case, a list or a configuration without a person approving the specific action (research 29 9k); `emergency_allowed = false`.

---

## 4. Infrastructure Updates

### 4.1 Job kinds (Postgres `jobs`; coalesce key in brackets)

| Kind | Binary | Payload | Concurrency, attempts | Schedule or trigger |
|-|-|-|-|-|
| `assistant_generate` | worker | `{ draft_id }` [`assistant_generate:<draft_id>`] | 2 (equal to the server's slots), 2 attempts; the second only for `model_unavailable` | enqueued in the request transaction (section 3.1 step 11) |
| `assistant_evaluate` | worker | `{ task, model_version_id, template_version_id, run_started_at }` [`assistant_evaluate:<task>:<model>:<template>`] | 1, 1; lease 6 hours | enqueued by `POST /assistant/evaluations` (section 3.6) |
| `assistant_version_verify` | worker | `{ version_id }` [`assistant_version_verify:<version_id>`] | 1, 3 | enqueued by `POST /assistant/versions` for kind `model` (section 3.9) |
| `assistant_monitor` | worker | `{ day }` [`assistant_monitor:<day>`] | 1, 3 | `schedule.assistant_monitor` (05:20 bank time), `day` = the previous bank day; section 3.7 |

Every handler re-reads its owner first (the draft, the version, the kill rows, the case). No job of this phase reaches the core banking system: the worker has no source connection (phase 13 rule, phase 00 zones). The job names do not use the `ingest_` prefix. Each step of `assistant_generate` is its own transaction; the model call holds no lock.

### 4.2 Timers and outbox

No timer kind. No outbox topic is produced or consumed: the request, the accept, the discard and the switches are synchronous, and the decision of phase 18 (outbox `case.decision`) is unchanged. The `AssistantInvolvement` read is a call inside phase 18's decision transaction, not a consumer.

### 4.3 AppState additions

Listed in section 2 (last bullet). The `assistant` router is mounted after `models` in the OpenAPI registration; its literal paths are registered before `/assistant/drafts/{id}`.

### 4.4 Network zones, the language-model server, the artifact store and compose

- **Runtime (DR-275).** One container `llama` runs the llama.cpp server (`llama-server`), serving one GGUF model over HTTP on port 8080. The image `akr/llama-server` is built by the release pipeline from a pinned llama.cpp release; the release tag and the commit are recorded in the release manifest and in the SBOM and NOTICE file, and the image is signed like the other images of phase 00. This draft does not fix the tag (Open doubts 12). The server runs non-root, with a read-only root file system, dropped capabilities, no published port, and a memory limit sized from the weights and the KV cache of `context_tokens`. CPU is the default; a compose profile `gpu` adds the device reservation and is not part of the default bundle (section 8.5 of research 32; DR-291).
- **Zone (new network `z2_model`, `internal: true`, no egress).** Members: `akr-worker`, `llama` and `llama-eval` only. `akr-api` is not a member: the API never reaches the model server, and no browser request waits on it. `llama` has no route to PostgreSQL, Valkey, OpenBao, the object store or the internet. `akr-worker` may reach `llama:8080` and nothing else new. The zone rules of `deploy/zones.yaml` gain these two rows; `make zone-check` probes them (section 6, check 13).
- **Authentication.** `llama` accepts only a bearer secret read from a file mounted read-only (the server's authentication option is confirmed at the phase 00 smoke test). The worker reads the secret from OpenBao KV `secret/assistant/llama` at start (class `secret-str`).
- **Artifact store (`assistant-store`).** A read-only volume mounted into `llama` and `akr-worker`, root `/var/akr/assistant`. It is written only by `akr-ctl assistant artifact add --file F` on the bank host, which refuses an existing name and prints the SHA-256 for the import of section 3.9. No route, job or code path downloads weights at run time. A file reaches the host as a signed artifact with its manifest, the same way as a phase 27 model.
- **Live model at start.** `llama` starts with the artifact of the live model version. The worker's readiness check (section 4.4, metrics) compares the artifact's SHA-256 reported by the container label `akr.model_sha256` with the live version; a mismatch makes the worker report `assistant_runtime` degraded and refuses new drafts with `model_unavailable`. A promotion takes effect when the operator runs `akr-ctl assistant serve`, which renders the `llama` command from the live version and restarts the container (`akr-ctl net render` renders the nftables zone rules only, phase 00); until then the readiness mismatch refuses drafts with `model_unavailable` (fail closed). With no live version `llama` is not started.
- **Evaluation server (`llama-eval`, lead review).** The same image in compose profile `assistant-eval`, on `z2_model`, with the `assistant-store` volume read-only; started by `akr-ctl assistant eval-server --version <id>` with one `registered` or `live` artifact and stopped by `akr-ctl assistant eval-server --stop`. It serves only `assistant_version_verify` and `assistant_evaluate` through a second `AssistantModelPort` instance (`llm_eval`), never a live draft, and reports the SHA-256 of the file it loaded.
- **Keys.** The worker reads the verify-only key `keys/model-artifact/v<N>.pub` (phase 27 purpose), the HMAC key of `golden_set_sha256` at `secret/assistant/golden-hmac` (section 3.13) and the bearer secret above, all from OpenBao KV. No signing key is created in this phase.
- **Compose.** Add the service `llama` (networks `z2_model`; volume `assistant-store` read-only), add the service `llama-eval` (profile `assistant-eval`, same networks and volume), add `z2_model` and the volume to `akr-worker`, and add the test-only service `llama-stub` (deterministic JSON replies, profile `test`, never in the production bundle). `akr-api` gains nothing.
- **Zone table (phase 00 table, rows added).** `llama` | `akr/llama-server` (pinned, signed) | `z2_model` | internal only, no egress; `llama-eval` | `akr/llama-server` (same image, profile `assistant-eval`) | `z2_model` | internal only, no egress.
- **Metrics** (labels never carry an id, a case or a party): `akr_assistant_draft_total{task,status}`, `akr_assistant_refused_total{code}`, `akr_assistant_model_seconds` (histogram), `akr_assistant_kill_engaged{scope_kind}` (gauge, from a 60 s sample), `akr_assistant_monitor_red` (gauge, the red points of the last run).
- **Readiness.** `akr-worker` reports `assistant_runtime` ready when `llama` answers `/health` and the artifact checksum of section 4.4 matches the live version; with no live version the component reports `not_configured` and the drafts are refused with `not_live` (section 3.1 step 5).

### 4.5 Existing entity, schema, route and code updates (extensions of earlier phases)

- **Phase 02:** the audit kind catalogue gains the kinds of section 3.12 with their SIEM flags. No evidence owner kind is added (drafts are sealed in their own row).
- **Phase 03:** the Cedar bundle gains `C2_assistant.cedar` and the schema additions of section 3.10; the feature flag `pack.assistant` is seeded by `0090`; the purpose codes `investigation`, `report_preparation` and `qa_review` are reused.
- **Phase 04:** `ChangeTargetRegistry` gains `AssistantPromotion` (kind `assistant_version`) and `AssistantSwitch` (kind `assistant_switch`); inventory kinds `llm_component` and `prompt_template` are reused as they stand.
- **Phase 00:** the zone rows, the network `z2_model`, the volume `assistant-store`, the image `llama` and `schedule.assistant_monitor` (`0090`).
- **Phase 18:** `RecordDecisionEffect` calls `AssistantInvolvement::for_case` once in the decision transaction (section 3.4), and `casework.decisions.ai_involvement` takes the shape of section 3.4 (its default is unchanged). `GET /cases/{id}` affordances gain `assistant_summary` (true when the flag, the task switch, the live versions and the kill state allow a draft for the caller; computed on the server; Open doubts 20). `NoteService` and `opened_views` are used unchanged.
- **Phase 19:** `casework.qa_samples.population` gains `assistant_golden` and `assistant_assisted` and `source_kind` gains `assistant_draft` (`0089`); the `assistant_assisted` draw skips decisions already in a sample (unique `(source_kind, source_id)`); the weekly draw includes `assistant_assisted` at `assistant.qa_sample_percent` (requirement, Open doubts 13); the QA review screen shows the evaluation draft text of an `assistant_golden` sample (requirement, Open doubts 14).
- **Phase 21:** `ReportService::update_draft` is called by the narrative accept without change. `report_versions`, the validation, the signing and the filing are not changed.
- **Phase 24:** `admin.registers.source` gains `assistant_monitor` (`0088`); `ValidationPackService::issue` gains the assistant branch of section 3.9; the review calendar gains the derived row kind `assistant_kill_drill` read from `admin.assistant_kill_state.next_drill_due` (amber 30 days before, red on the due date; a row of the existing calendar, no table).
- **Phase 25:** the support bundle and the SIEM export exclude `casework.assistant_drafts` and `admin.assistant_evaluations` by table name; a release bundle may carry a signed assistant artifact, which people import through this phase's version import as `registered` only; the release imports none (DR-390, Open doubts 15).
- **Phase 27:** the D4 screen gains an "Assistant" tab (section 5.3). No table, CHECK or route of phase 27 changes; the `keys/model-artifact` purpose and the kill-switch pattern are reused.
- **Phase 23 (engine, requirement):** a draft is purged only with its case (owner clock). No handler deletes a case's drafts in this release (7.2, DR-386: they are `report_only` with their case); the foreign key is `ON DELETE RESTRICT`.

---

## 5. Frontend Impact

### 5.1 W3 assistant block (`/cases/:id`; phase 18 screen, addition; catalogue ID W3, `docs/DESIGN-SYSTEM.md` line 691; assistant block at line 797)

- Shown only when the caller's affordance `assistant_summary` is true (section 4.5). For a caller without it, the block is absent: no placeholder, no disabled control.
- **Request.** "Draft summary" with a language choice (Arabic or English). On request the block shows "Preparing a draft" and polls `GET /assistant/drafts/{id}` every 2 seconds for at most 120 seconds. A `refused` answer shows the reason in words for the code and offers the manual path ("write the note yourself").
- **Draft (`ready`).** The label "Draft" in the Info tone (DESIGN-SYSTEM 7.10 and the status-tone table, line 156); the sentences in order, each with its source labels shown as chips naming the record panel ("transactions", "alerts"); a marked sentence is shown with the mark "not verified: edit or remove" and cannot be accepted as it is; a line "n sentences removed: not supported by the record" (counts only); a line "Not considered: <categories>" (from `not_considered_categories`); a tipping-off highlight from the client-side re-scan of the displayed text (the scan result is the server's list of spans, not a list of terms).
- **Evidence first.** The accept control is disabled while a cited panel is unopened; the disabled control names the panels to open (the opened state is the phase 18 `opened_views`). Opening a panel is the normal phase 18 action.
- **Actions.** "Accept as is" (enabled only when the text equals the output), "Edit and accept" (opens the text for editing; sends `mode: edited`), "Discard" (with the useful-or-not control: two buttons, "useful" and "not useful", and a reason list). There is no "accept all" and no control that accepts more than one draft.
- **After accept.** The note shows the label "Drafted by assistant · edited by <role>" (DESIGN-SYSTEM 7.10), or "Drafted by assistant" for an as-is accept. The label names the role, not the person.
- **Words.** "Draft" and "Summary" in the UI; no score, no number from the model, no "suspicion" wording beyond phase 18's own text.

### 5.2 R1 narrative (`/reports/:id`; phase 21 screen, addition; catalogue ID R1, `docs/DESIGN-SYSTEM.md` line 714; narrative badge at line 826)

- In the narrative step, for the roles of `report.prepare` (A2, M) and only while the report is `draft`: "Draft narrative" produces both languages. Each language is a list of sentences in the W3 format of section 5.1 with its source chips; the narrative counter of phase 21 shows the characters of the accepted text.
- The badge "Draft" stays on the narrative field until an accept; after the accept the field caption reads "Drafted by assistant · edited by <role>" (the phase 21 tipping-off phrase check runs on save as before, and the assistant's own scan runs on accept).
- Accept, discard and the evidence-first rule are as in section 5.1. A narrative longer than 8,000 characters is refused at the field by phase 21 (`/narrative/ar` or `/narrative/en`).

### 5.3 D4 AI governance, the Assistant tab (`/models`; phase 27 screen, addition; catalogue ID D4, `docs/DESIGN-SYSTEM.md` line 713)

Phase 27's D4 has four tabs; this phase adds a fifth, **Assistant**, with:
- **Tasks:** one row per task with tier, autonomy ("draft"), the switch state (read from settings, changed by change request only), the live template and model numbers, the status of the last final evaluation and the next drill date.
- **Kill switch:** the global row and one row per task, with the step-up press and the release request, as the phase 27 kill-switch tab does.
- **Evaluation:** the stages and gate verdicts of the last runs per task (counts and rates, the gate rows as ticks and crosses with their numbers, no id); "Run evaluation" for the initiator of section 3.6.
- **Monitoring:** the counts of section 3.7, with "fewer than 3" in words where suppressed.
- **Versions:** import dialogs for a model (manifest and signature) and a template (body and signature), and the promotion request with the list of triples.
- "Suspicion" never appears on D4. Labels come from the bilingual objects of the API (section 11 of DESIGN-SYSTEM, Arabic first).

### 5.4 Generated hooks (Orval names from the paths)

`usePostAssistantDrafts`, `useGetAssistantDraftsId`, `usePostAssistantDraftsIdAccept`, `usePostAssistantDraftsIdDiscard`, `usePostAssistantEvaluations`, `useGetAssistantEvaluations`, `useGetAssistantTasks`, `usePostAssistantVersions`, `usePostAssistantVersionsIdPromote`, `usePostAssistantKillSwitch`, `usePostAssistantKillSwitchReleaseRequests`, `useGetAssistantMonitoring`. The W3 affordance is read from the phase 18 `useGetCasesId` response.

### 5.5 Browser and caching rules

- Draft text is never kept in local storage, session storage, a URL, a page title, a log line or the query cache beyond the open panel; the cache entry is dropped when the panel closes. An unsent edit lives in component state only.
- Draft responses carry `Cache-Control: no-store` (the suspicion-tier rule of section 1 of the API).
- The kill state is polled every 30 seconds while D4's Assistant tab is open. Tasks and monitoring are cached for one minute.
- The drafts poll stops at `ready`, `refused`, `accepted` or `discarded`, or after 120 seconds.

---

## 6. Verification

1. `make migrate` applies `0088_admin_assistant_registry`, `0089_casework_assistant_drafts` and `0090_assistant_guards_grants` on a clean database and again on the migrated one (idempotent). On a throwaway `docker run --rm postgres:17`: `admin.registers` accepts source `assistant_monitor` and refuses `scan2`; `casework.qa_samples` accepts population `assistant_golden` and refuses `scan`; a second `live` model, and a second `live` template of one task, are refused by the partial unique indexes; `casework.assistant_drafts` refuses `task = 'report_narrative'` with `language = 'ar'`; the guard refuses a change of `output_enc` once set and the edge `ready` to `requested`.
2. `cargo check` with the SQLx offline data regenerated for the queries of sections 3.1, 3.5, 3.7 and 3.9.
3. `cargo clippy -- -D warnings` for `akr-core`, `akr-api`, `akr-worker` and `crates/llm`.
4. `cargo test`, unit tests in `domain::assistant` and `services::assistant`:
   - Grounding fixtures (section 3.3): an amount written as text with no placeholder is removed (`unsupported_figure`); `{amt:R2.amount}` with R2 in `refs` is kept with the record's canonical amount; a date not in the records is removed (`unsupported_date`); a bare name of the records is removed (`unsupported_name`); `{name:R3.name}` where R3 has no English name in an English sentence is marked (`source_incomplete`); a sentence without refs is removed (`no_source`); the 41st sentence is removed (`over_limit`); `not_considered` equals the categories with no kept citation whatever list the model sent.
   - Data blocks: a record text containing the request's nonce is escaped and cannot close its block.
   - `AssistantInvolvement` builds `{"used": false}` for no accepted draft and the exact shape of section 3.4 otherwise; `edit_band` is `none` at 0, `minor` at the threshold 0.20 and `major` above it.
   - The golden counter: 199 items fails `assistant.gate_min_items`, 200 passes; fewer than 50 items in one language is `too_few_items` in the result.
   - The guards of section 1.3: engaging a kill row without the flag is allowed, releasing it without the flag is refused; `live` without the promotion flag is refused.
5. Model stub tests (`llama-stub`): a timeout on both attempts gives `model_unavailable`; unparseable JSON gives `output_unparseable`; a reply in which every sentence is unsupported gives `nothing_supported`.
6. Tipping-off test: a narrative fixture containing a term of the bank's `reason_text` list gives `scan_hits >= 1`; the accept of its text with the term gives 422 `forbidden_term`; the same text without the term is accepted.
7. Visibility pin (phase 18 check 6 pattern, extended): generated principals with pool and `case_acl` memberships, cases with tiers and suspicion rows; the predicate of the draft reads, the Cedar permits of section 3.10 and the masked 404 agree on every row, and `GET /assistant/drafts/{id}` agrees with `GET /cases/{id}` for the same principal and case.
8. Bundle: `akr-ctl policy validate` prints the new bundle hash with `C2_assistant.cedar`; the bundle tests of section 3.10 pass.
9. Curl suite `tests/curl/phase28_assistant.sh` (dev users `a1@bank.test` (A1), `a2@bank.test` (A2 on the case), `a2b@bank.test` (A2 not on the case), `m@bank.test` (M), `mro@bank.test` (MRO), `mro2@bank.test` (a second MRO for the approval), `ia@bank.test`, `ch@bank.test`, `ss@bank.test`). Each check states its expected answer:
   - a. Flag off (`pack.assistant` FALSE): `POST /assistant/drafts` as `a1` gives 404 `E_NOT_FOUND`; each of the twelve routes returns a body byte-equal to the body for an unknown id.
   - b. Flag on, `assistant.case_summary.enabled` FALSE: `POST /assistant/drafts` gives 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `task_disabled`.
   - c. Scope and Cedar: `ss@` on `POST /assistant/drafts` gives 403 `E_SCOPE`; `a1@` on `POST /assistant/drafts/{R1}/accept` for a report narrative draft gives 403 `E_FORBIDDEN`.
   - d. Masked 404: `a2b@` on `GET /assistant/drafts/{D1}` for a case it is not on, with `X-Access-Purpose: investigation`, gives 404 `E_NOT_FOUND`, body byte-equal to the body for an unknown uuid.
   - e. Purpose: `a1@` on `POST /assistant/drafts` without `X-Access-Purpose` gives 422 `E_PURPOSE_REQUIRED`.
   - f. Invented amount: with the stub replying "the customer moved 9,999 IQD", the draft becomes `ready` with `grounding_result.removed = 1` and the reason `unsupported_figure`; the output has no sentence with that figure.
   - g. Accept refusals: no `mode` gives 422 `mode_required`; `as_is` with a different text gives 422 `as_is_mismatch`; a final text that contains a marked sentence gives 422 `unsupported_sentence_kept`.
   - h. Evidence first: accept before the cited panels are opened gives 409 `evidence_not_opened` with the panel keys; after `POST /cases/{id}/opened-views` for each key, the same accept gives 200.
   - i. `If-Match`: accept without the header gives 428 `E_PRECONDITION_REQUIRED`; with a stale version 409 `E_VERSION_CONFLICT`.
   - j. Go-live: `POST /assistant/versions/{M1}/promote` with no final evaluation gives 409 `evaluation_missing`; with a final evaluation that failed gives 409 `gate_not_met`.
   - k. Kill switch: `mro@` `POST /assistant/kill-switch` (scope `global`) without a step-up assertion gives 401 `E_STEPUP_REQUIRED`; with an `approval` assertion gives 200; then `POST /assistant/drafts` gives 409 `kill_engaged`. The release request gives 201 with a `cr_material` change request; `mro@` approving it gives 403 `E_SELF_APPROVAL`; `m@` and the CHK approver approve; after activation the drafts work again.
   - l. Idempotency: the same `Idempotency-Key` and body on `POST /assistant/drafts` gives 202 with `x-idempotent-replay: true` and the same draft id; the same key with another body gives 422 `E_IDEMPOTENCY_REUSE`.
   - m. Tipping-off narrative: the fixture narrative with a term gives `scan_hits >= 1`; accept gives 422 `forbidden_term`, with spans and without the term in the body.
   - n. Decision: after an accepted draft and a decision on the case, `GET /cases/{id}/decisions` shows `ai_involvement.used = true` with the draft id; a decision with no accepted draft shows `{"used": false}`.
   - o. Monitoring: `mro@` and `ia@` on `GET /assistant/monitoring` get counts, with "suppressed" for every count below 3; `a1@` gets 403 `E_SCOPE`.
   - p. Audit: `assistant.draft_accepted` is present with `accept_mode` and no text; a search of the audit detail of the session for the stub's output text finds nothing.
10. Swagger UI shows the twelve `assistant` routes; the literal paths `/assistant/kill-switch` and `/assistant/kill-switch/release-requests` are listed before `/assistant/drafts/{id}`; `make generate-api` regenerates the client cleanly.
11. The audit trail of the curl session shows the kinds of section 3.12 with their SIEM flags (kill engaged and released, version refused) and no kind outside the catalogue.
12. Read-only conformance: `has_table_privilege('akr_readonly', t, 'SELECT')` is false for the four new tables.
13. Zone check: `make zone-check` matches `deploy/zones.yaml`: from the position of `llama` only `akr-worker` is reachable and no other service or internet address is; from the position of `akr-api` `llama:8080` is closed.
14. Existing tests pass: phase 18 decision tests (the default `ai_involvement` unchanged), phase 21 narrative tests, phase 19 QA tests with the widened population CHECK, phase 24 register tests with the widened source CHECK, and phase 27 model tests.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 1: corrections found while checking phases 00 to 02 (tools/agent/gap1-run1-verdict.md); lead-verified.

### 7.1 KV paths
KV paths are relative to the one mount `akr/` (phase 00 7.5, DR-356): `secret/assistant/llama` is `assistant/llama`
and `secret/assistant/golden-hmac` is `assistant/golden-hmac`.

---

Gap pass 1 (2026-10-10), run 4b: items carried from run 4a and corrections found while checking phases 25 to 31 (tools/agent/gap1-run4b-verdict.md); lead-verified.

### 7.2 Retention DELETE grant (DR-386)
`akr_retention` gets SELECT only on the four new tables: no handler deletes their rows in this release (R11 and the
R6 drafts are `report_only`, R10 is never purged). The delete guards that admit `akr_retention` stay.

### 7.3 Release bundles import no assistant version (DR-390)
A signed assistant artifact that a phase 25 release carries is imported by people through this phase's version
import, as `registered` only; the release itself imports nothing (phase 25 section 7+).

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.4 Publisher country in the model manifest (V-00-02-b, MEDIUM; DR-413)
Section 3.9, import step 2: the manifest field `publisher` is an object `{name, country}`, and `country` is an ISO 3166-1
alpha-2 code. A manifest without it is `manifest_invalid`. Reason: the supply-chain legal screen (VO-07, phase 00) needs
the maintainer and country of every component, and a model is outside the release SBOM (DR-390).

### 7.5 The QA sample keeps its evaluation run (V-28-02-b, MEDIUM; DR-413)
Migration `0089` (section 1.2), next to the two constraint changes on `casework.qa_samples`: `ALTER TABLE
casework.qa_samples ADD COLUMN IF NOT EXISTS eval_run_id UUID;` with its classification row (`internal`). The table
grant of phase 19 covers the column. The draw of section 3.6 step 4 passes the run as `eval_run_id` (phase 19 section
7+).

### 7.6 Provenance on the draft view (V-28-04-a, MEDIUM; DR-413)
`GET /assistant/drafts/{id}` and `DraftView` gain a `provenance` object, null unless the draft carries its text
(section 3.4, the case team only): `requested_by`, `requested_at`, `model_version_id` and `template_version_id` (each
with its `version_no` from `admin.assistant_versions`), and `source_ids`. These are the DR-277 fields the draft row
already keeps; the case, the output and the edits are left out because the view shows them.

## Decisions

1. **DR-274 (two tasks; the rest not carried).** The registry holds two tasks, the case summary (`case_summary`) and the report narrative (`report_narrative`), both at tier 3 with autonomy `draft`. The roadmap row 28 names exactly these two ("Assistant drafts for case summaries and report narratives"). Translation is not a task: the narrative writes both languages from the same records, and names and amounts are copied by code. Disposition recommendation (research 21 use case 5, research 28 14.1 "recommend") is not carried, because it needs the automation-bias gate of GV-42 (phase 27 Open doubts 1) and a validated golden set of critical errors first; it returns as a later staged change request. Extraction, adverse-media triage, the investigation chat, RFI drafting, QA triage and regulatory mapping are not carried (Open doubts 2). Rationale: M-70 and the enforced permitted-use list of research 29 9k; the smallest set that the evaluation gate can prove.

2. **DR-275 (runtime: llama.cpp server on premises, no egress, reached by the worker only).** The language model runs in one container, `llama`, as the llama.cpp server with one GGUF file, on an internal network `z2_model` with no egress. CPU is the default; a GPU is an optional profile. The worker is the only client, through the port `AssistantModelPort`; the API never reaches the model server, because a model call must not hold a browser request and the generation runs as a job (DR-278). The image is pinned to a llama.cpp release and signed like the other images; the weights are signed artifacts placed on a read-only volume and never downloaded at run time. Rationale: TS-17, GV-52 and T-51; research 32 8.5 names llama.cpp or vLLM class servers for optional on-premises language features; llama.cpp needs no Python in the product and runs on CPU. vLLM was not chosen because its serving path is GPU-first and Python-based, which the product's no-Python and CPU-default choices exclude.

3. **DR-276 (a separate registry, not a phase 27 family).** Language-model versions are rows of `admin.assistant_versions` (kind `model` or `template`). Phase 27's `admin.model_versions` is not reused: its CHECKs assume ONNX (`artifact_key` ends in `.onnx`, `runtime` is `ort` or `tract`), its rows hold tree features and monotone constraints, and its scoring writes `alert.model_scores`, none of which a language model has. Loosening those CHECKs would weaken phase 27, so none is changed. The inventory (phase 04) still governs both kinds, with the existing kinds `llm_component` and `prompt_template`. Rationale: the conventions file rule against loosening an earlier CHECK silently; the lead asked for the choice and its reason to be stated.

4. **DR-277 (drafts in `casework`, sealed, followers of their case).** `casework.assistant_drafts` sits beside `casework.notes` because a draft holds case content, is refused on a sealed case, follows the case's visibility and needs the case's clock. Its output and its final text are sealed under AAD = the row id (the phase 18 pattern). The prompt is never stored: the draft keeps `input_sha256`, the record references and the panel keys, which is enough for GV-46 provenance (user, case, time, model and version, template and version, retrieved record ids, output, edits and final text) without a second copy of the case in prompts. Retention is R6 with the owner clock (the case governs it, section 1.3). Rationale: DATA-MODEL section 15 lists `assistant_drafts` as R6; SP-94 says the prompts and outputs are suspicion-tier records.

5. **DR-278 (generation is a job; the browser polls; no streaming).** A draft request writes a `requested` row and returns 202; the job `assistant_generate` calls the model, verifies and seals the output; the browser polls the draft until it is `ready`. The model is never called inside a request, no unverified text reaches a browser, and a timeout or a crash is a refusal code on the row rather than an open connection. Retrieval is re-checked for the requester when the job runs, because the case may have changed since the request. Rationale: GV-49 (timeouts, kill switch, full action log), SP-94 (no path from output to a customer channel), and the phase 00 job pattern.

6. **DR-279 (grounding by placeholders; the tipping-off scan is phase 09's).** The model writes figures, dates and names only as placeholders that the code fills from the records, and the deterministic `GroundingVerifier` removes any sentence with a figure, date, name, currency or source it cannot match, and marks a sentence whose record lacks a field. The verifier proves copying and citation, not the truth of the claim; the human factual review of section 3.6 measures the rest (stated as a limit). The tipping-off check uses phase 09 `PolicyPackService::scan` with scope `reason_text`, the same list the narrative of phase 21 uses. The name `TippingOffTerms` of phase 04 is satisfied by that scan; no struct of that name is created. Rationale: research 28 14.2 points 3 and 4; GV-47 and M-71; DR-137 (phase 18) for the scanner.

7. **DR-280 (saving needs a person; one draft per accept; `ai_involvement` shape).** Nothing leaves a draft except through the accept of one draft by one person, with `mode` required (`as_is` or `edited`), the edit measured by the server, and the target written through the existing paths: a case note (`NoteService::post`, which names the accepting person as author) or the phase 21 narrative PATCH. There is no accept-all, no save route and no export route. `casework.decisions.ai_involvement` takes the shape of section 3.4 and is written once, inside phase 18's decision transaction, from the drafts accepted at or before the decision. The API.md row of the accept keeps `queue.work` alone (A2 and M already hold it); phase 21 `report.edit` refuses A1 on a narrative, so A1 keeps the case-summary path only (section 3.10; lead review: the draft's added `report.prepare` admitted nobody new). Rationale: GV-48, IR-23, UX-45, DESIGN-SYSTEM 7.10; the decision record must show AI involvement without holding its text (research 28 14.2 point 2).

8. **DR-281 (kill switch: immediate to engage by one MRO; two-person to release).** The global and the per-task switch are rows of `admin.assistant_kill_state`. Engaging is immediate under step-up `approval` by an MRO, writes no change request, and refuses every new draft and every accept at the next step; releasing is a change request on `cr_material` with the parallel approvals of M and CHK, so one person cannot re-enable a switch that a red breach engaged. This follows phase 27 DR-266. It differs from the literal wording of research 28 14.2 point 6 ("two-person"), which this phase reads as two-person for the release, because a stop that waits for a second person is not a stop; the bank may decide otherwise (Open doubts 8). Rationale: SP-95, GV-20, the phase 27 pattern.

9. **DR-282 (per-task switch as governed settings; pack flag `pack.assistant`).** Each task has its own governed boolean setting (`assistant.case_summary.enabled`, `assistant.report_narrative.enabled`, default FALSE, changed only by change request), and the whole pack has the feature flag `pack.assistant` (default FALSE), which gives 404 `E_NOT_FOUND` on every route, the phase 26 pattern. The flag cannot be turned off while a live version exists (phase 26 rule). A switch is a configuration decision and a flag is an installation decision, so both layers stay separate. Rationale: M-70 (its own switch per bank) and the pack pattern of phase 26.

10. **DR-283 (the evaluation gate: golden set from QA-confirmed closed cases, two stages, counts only).** Golden items are closed cases with a phase 19 QA review of `pass` or `pass_with_comment`, drawn deterministically, at least `assistant.gate_min_items` (200) and at least 50 per language. The MRO who initiates a run reads no text; the job reads the golden text as the system under purpose `qa_review`, and only the counts are stored. A human factual review on population `assistant_golden` (phase 19 rubric) supplies the critical count, and the run is final only when that review covers the minimum with zero critical findings. Metrics are refusal rate, removed-sentence rate, coverage of the record's mandatory facts, and the human counts. Rationale: M-73, GV-50, T-49, research 21 12e (golden set, QA-confirmed labels per GV-39, rule of three), and the phase 19 populations that already exist.

11. **DR-284 (go-live is one change request for the runtime set).** A promotion names one model version and one template version per task, needs an `approved_for_use` inventory item for each, a final passing evaluation for each triple and a `pass` pack per item, and opens `cr_material_validated` (phase 04 route, MRO approval). Activation retires the previous live versions and sets the new ones live inside the change, with the transaction flags of the guard. A retired version is not revived; a rollback is a new promotion. The emergency route is not used. Rationale: GV-24 and GV-26 (phase 27 pattern), and one model serving both tasks means one decision on the whole set.

12. **DR-285 (provenance of versions: signed manifests with the GV-51 fields; templates under change control).** A model manifest carries publisher, base model, licence and the SHA-256 of its text, the provenance of the weights, the SHA-256 of the training-data statement, architecture, languages (Arabic and English required) and the chat template hash, and is verified with the phase 27 verify-only key purpose. A prompt template is signed by the bank's model team over its body and placeholder kinds, and is refused on an unknown placeholder or a scan hit. Each version is an inventory item (tier 3, owner = the importer, validator never the owner). Rationale: GV-51 (the checklist fields that a file can carry; the security tests, audit right and exit plan are procurement records, Open doubts 17), research 29 9k "versioning and change", and GV-03 and GV-04.

13. **DR-286 (monitoring: counts across all tiers, suppressed, no split; one register finding per task and month).** The panel shows 30-day counts per task on live drafts only, every tier included (the phase 24 `str_rate` pattern), every count below `report.small_number_threshold` suppressed, and no split by report type, decider, branch or day. A red breach (removed share or discard share) opens one phase 24 register finding per task and month with source `assistant_monitor`. Rationale: M-73 (use, acceptance, edit distance, override, QA findings per task), GV-18 and GV-19 as applied in phase 27, and DR-270 of phase 27 (counts only, no speed league table; research 28 14.3).

14. **DR-287 (the QA sample of assisted decisions never goes to zero).** Decisions whose `ai_involvement.used` is true form the phase 19 population `assistant_assisted`, drawn weekly at `assistant.qa_sample_percent` (governed, floor 10 per cent, research 21 12e point 5). The population is the reviewer's own check of the assistance, not a second channel. Rationale: M-72 (the QA sample of assisted dispositions never goes to zero) and the phase 19 sampler, which already draws by population and rate.

15. **DR-288 (evidence before accept, the partly carried M-72 rule).** An accept needs the phase 18 `opened_views` of every panel the draft cites, for this actor and this case. This carries the spirit of "evidence before suggestion" (research 28 14.3) for a draft that suggests nothing; the recommendation half of M-72 belongs to the deferred recommendation task (DR-274). Rationale: UX-45 ("recommendations only after evidence is opened") as applied to drafts, and the phase 18 opened-view record that already exists.

16. **DR-289 (containment: no agent, no tools, rules for any later agent).** No agent or tool is built. The model receives one completion with no tool list and no function calling; its output is data; the only writes it causes are the draft's status transitions. Any later agentic change must meet the rules of section 3.16 and enter as a change request. Rationale: T-50, SAFR and OWASP LLM06 as applied by research 29 9k and research 21 12a use case 6, which this phase does not carry.

17. **DR-290 (names and services; no `PromptTemplateService`).** The services are `AssistantService` (gateway: request, accept, discard), `AssistantReads` (task registry, draft reads, monitoring reads), `AssistantKillSwitchService`, `AssistantVersionService` (models and templates, validation branch, promotion), `AssistantGenerator` and `AssistantMonitor` (worker), `GroundingVerifier` (pure), `PromptBuilder`, `AssistantEvaluationService` (golden sets, runs, gate, finalisation), and the port `AssistantModelPort` implemented by `LlamaServerClient` in the new crate `crates/llm`. The name `PromptTemplateService` is not used: prompt templates are versions handled by `AssistantVersionService`. Rationale: conventions file (one name per job; grep of docs/plan confirms the names are free).

18. **DR-291 (hardware: CPU by default; the model is the bank's choice by evaluation).** The first deployment runs on CPU; the memory limit covers the quantised weights and the KV cache of `context_tokens`; a GPU profile exists but is not bought by this phase. The model is not selected here: the candidates named by research 21 12f (Jais, ALLaM, and multilingual open models) are chosen by the bank's golden evaluation (T-51), and the licence and training-data statement are checked at import (DR-285). Rationale: research 32 8.5 (no GPU for optional language features until a load test shows the need; the DBS declined GPU capacity twice, DBS decision 110).

**Lead corrections (review 2026-10-09).** The production `llama` serves only the live model, so a candidate could never be verified or evaluated and no model could go live; a separate `llama-eval` server in profile `assistant-eval` serves the verify and evaluation jobs, and the job checks the artifact hash it reports. The promotion required a `registered` model and retired the previous live set, so a template change with the live model kept was impossible; a live model or template named in the request now stays live and only replaced versions retire. The golden review had no item: the phase 19 `source_kind` CHECK is widened by `assistant_draft`, the QA item view shows the evaluation draft, and `eval_run_id` and `run_id` tie drafts, samples and both stage rows of a run. Evaluation drafts are retrieved as the system principal with purpose `qa_review`, not with the MRO initiator's visibility. The accept keeps `queue.work` alone; model version numbers are unique with `NULLS NOT DISTINCT`; `akr-ctl assistant serve` (not `net render`) restarts the model server; the summary limit leaves room for the note's first line; a breach finding is owned by the task's template item owner.

## Open doubts (for gap pass 1 and the bank)

1. **Disposition recommendation.** Deferred (DR-274). It needs the GV-42 closure gate, a validated critical-error rate and a staged change request. The bank must decide whether it is wanted at all; the phase 27 Open doubts 1 (owner is MRO) apply too.
2. **Dropped candidate tasks.** Extraction needs documents, which are not in the product (Open doubts 6); adverse-media triage overlaps the screening of phase 14 and needs source texts; the investigation chat needs parameterised query tools, which are an agent (DR-289); RFI drafting belongs to phase 19's owner; QA triage would act on QA, which research 28 14.1 forbids; regulatory mapping is a compliance process. Each returns with its own DR.
3. **Design values.** Coverage floor 0.90, refusal ceiling 0.05, 50 items per language, `assistant.edit_minor_ratio` 0.20, 40 sentences per output, 800 output tokens, 24,000 input characters, 120-second timeout and 30-day windows are proposals. The bank tests them on its own data before the first go-live.
4. **Gate arithmetic.** At least 200 reviewed items with zero critical findings bounds the critical rate near 1.5 per cent (3 over n); research 21 12e uses 300 alerts for about 1 per cent. The gate removed-rate ceiling of 0.05 is a proposal. The bank may raise the review count (not lower it: the floor is governed).
5. **Monitoring thresholds.** `assistant.discard_rate_red` 0.50 and `assistant.removed_rate_red` 0.10 are proposals and are settings with a floor where governed.
6. **Evidence documents are not retrieved.** Document text needs extraction (OCR is not in the product through phase 27). A later phase may add it with its own verifier rules.
7. **Report versions carry no AI flag.** The narrative's involvement is recorded in the case decision (DR-280), not in `rpt.report_versions`. A bank that needs the flag on the report version needs an addition to phase 21 (a column and its CHECK); this phase does not add it.
8. **Two-person engage.** DR-281 keeps a single MRO for the engage, because the stop must be immediate. If the bank requires two people to engage, the stop waits for the second person, and DR-281 changes; this phase does not build that.
9. **Kill drill evidence.** The drill is evidenced by the release change request's attachments and the bank's runbook (phase 27 Open doubts 19 applies). No automatic drill runs.
10. **Rollback speed.** A retired version is not revived and no emergency change is allowed for models (phase 27 Open doubts 18). A bank that needs a faster rollback needs an emergency class in phase 04, which is a separate decision.
11. **Golden-set confidentiality.** `golden_set_sha256` is an HMAC keyed by a secret that only the worker reads (section 3.13). The bank confirms the key path `secret/assistant/golden-hmac`, its rotation, and that MRO may see the hash.
12. **Runtime pin and model candidates.** The llama.cpp release tag, the server's bearer-authentication option, its JSON-schema constraint support and the context size of the first model are confirmed at the phase 00 smoke test, not in this draft. The model family and licence are not chosen (DR-291). `tech-stack.md` has no language-model row; the lead adds one ("Language model runtime: llama.cpp server, GGUF, on premises, CPU default") in the same change. This draft does not edit `tech-stack.md`.
13. **Phase 19 weekly draw.** The weekly draw must include `assistant_assisted` at `assistant.qa_sample_percent` (requirement; phase 19 owner to confirm the draw seed).
14. **Phase 19 review screen.** The QA review screen must show the text of an `assistant_golden` evaluation draft to the QA roles who review that sample (requirement; phase 19 owner to confirm).
15. **Phase 25 requirements.** The support bundle and the SIEM export exclude `casework.assistant_drafts` and `admin.assistant_evaluations` by table name, and the release imports no assistant artifact (DR-390). Both are stated in phase 25 section 7+.
16. **Golden-set volume.** Two hundred QA-confirmed closed cases per task and language may take months for a new bank, and the QA sample is biased toward cases the QA team chose. The bank reports how many QA-confirmed closed cases each family produces per quarter before the gate date is set.
17. **Supplier checklist.** GV-51 asks for security tests, a right to audit and an exit plan of the model supplier. The product carries the fields a file can hold (section 3.9); the rest are procurement and contract records outside the product.
18. **Accept rights.** Any case-team member (A1, A2 or M on the case) may accept a case summary. The bank may require A2 or M for the narrative (already A2 and M by Cedar) and may require A2 for summaries.
19. **Arabic quality.** The golden review needs Iraqi-dialect reviewers in the QA roles; the phase 19 rubric is in Arabic and English. Research 21 12f says the quality of Arabic compliance writing is untested; the first evaluation decides.
20. **Affordance on W3.** The `assistant_summary` affordance of section 4.5 is a new field of phase 18's `GET /cases/{id}` response (requirement; the lead to confirm with phase 18).
21. **No live shadow period.** Research 21 12e point 3 asks for shadow mode on live cases for a defined period. This phase's shadow is the offline golden evaluation with human review, not hidden drafts on live cases. A live shadow period (hidden drafts for live cases for 30 days, with a load budget and a privacy review) is a later decision.
22. **Classification of assistance.** Drafting aids are tier 3 (research 29 9k). If the bank treats `ai_involvement` as an input to decisions, it may place the assistant at tier 2, which changes the approvals of the family item (phase 04). The bank sets the tier basis answers at import.
23. **Phase 04 inventory tier basis.** The seven tiering answers of phase 27 section 3.1 are reused for the two kinds; the bank confirms they make sense for a language model (no score, no alert effect).
24. **Phase 23 purge order.** Settled by 7.2 (DR-386): no handler deletes a case's drafts in this release. They are `report_only` with their case (phase 23: a child of a `report_only` owner is `report_only` too), and the foreign key stays `RESTRICT`.

## What This Phase Does NOT Touch

- No scoring, no alert change, no change to `alert.alerts`, `alert.model_scores`, the class, the rule score or the queue order (phases 17 and 27 unchanged).
- No change to the phase 27 model families, `admin.models`, `admin.model_versions`, their CHECKs or guards, the `model-store` volume, the phase 27 kill switch or its scopes; the key purpose `keys/model-artifact` is reused, not changed.
- No outbox topic, no timer kind, no change to `case.decision`, `alert.created` or any handler; the decision of phase 18 is changed only by its one call to `AssistantInvolvement` (section 4.2).
- No change to the phase 18 notes table, to the case access list, to the sealing of cases, to the visibility predicate, or to phase 18's Cedar file `91_cases.cedar`; `C2_assistant.cedar` is the only Cedar file added.
- No change to phase 21's report versions, validation, signing, filing or package; the narrative accept calls the existing PATCH path only.
- No CBS connection, no read of the CBS by the worker, no change to `akr-ingestd` or the source pools; the model server has no path to any service but the worker.
- No agent, no tool, no function calling, no write path for the model; no fine-tuning, no training pipeline, no weights download at run time (section 3.16, DR-289).
- No GPU purchase and no model selection (DR-291).
- No new problem code (section 3.11 uses API section 6 codes and sub-codes only); no change to API section 2 scopes.
- No change to the phase 04 change classes, route map or `activate_change` effect; no emergency change for the assistant.
- No change to the step-up classes of phase 01 (`approval` and `record_open` are used as they are).
- No email, notice, export, package or SIEM event that carries draft text, a model output, an evaluation text or a count below the threshold.
- No change to `tech-stack.md`, `docs/API.md`, `docs/DATA-MODEL.md`, `docs/DESIGN-SYSTEM.md`, `docs/plan/roadmap.md` or any other file of the plan; the lead applies the cross-file alignments (Open doubts 12 and 13 to 15, and the API rows of section 2 that are new).
