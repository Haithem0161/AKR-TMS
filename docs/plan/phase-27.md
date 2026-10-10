# Phase 27: Models & Model Governance

**Goal:** A bank-trained or starter ONNX model can rank rule alerts inside their priority class and nothing else: it is imported only as a signed artifact with a card, validated through a phase 24 pack, run in shadow beside the rules against success criteria fixed in advance, promoted to live only by a material change request on `cr_material_validated`, watched for drift and overrides, and switched off at once by a kill switch that returns the queue to rule order.

**Dependencies:** Phase 00 (jobs, settings, cron schedules, network zones, the signed bundle and `bundle verify`, OpenBao KV), Phase 01 (`StepUpService` with the closed class `approval`; `admin.users`, `admin.role_grants`), Phase 02 (`EvidenceStore::stage` and `put_generated`, `admin.evidence_objects`, the evidence owner registry, the audit writer and kind catalogue with the `siem` flag), Phase 03 (Cedar bundle and range 80, `Ctx` with required `purpose` and `network`, `MASKED_KINDS`, `purpose_codes`), Phase 04 (`admin.inventory_items` with kind `model`, `ChangeTargetRegistry`, `ChangeRequestService`, `cr_material_validated`, `activate_change`), Phase 13 (`feat.feature_definitions` with `requires` and the bilingual `name`, `feat.feature_values`), Phase 16 (shadow pattern and the scenario kill switch: the idea reused, not the routes), Phase 17 (`alert.alerts` partitioned by `created_at` with PK `(id, created_at)`, `AlertIntakeService`, `PriorityClassifier` and `AlertScorer`, queue sort, `services::visibility::build`), Phase 18 (`casework.decisions` with `type` and `decided_at`; `casework.case_alerts` is phase 17's table), Phase 19 (`casework.qa_samples`, `casework.qa_reviews`, the daily forced sample), Phase 21 (`report.small_number_threshold`), Phase 24 (`admin.registers`, `admin.validation_packs` and `ValidationPackService::issue`, `admin.protected_attributes` and `ProtectedAttributeCheck`, `fairness_monthly`, the review calendar, `validation.evidence_max_days`), Phase 25 (release bundles, which may carry model artifacts; the release imports none and never sets a model live, DR-390)

**Macro-phase:** P2
**Complexity:** L (16 routes: 6 rows of API.md 3.15 owned here (`GET /models`, `POST /models`, `GET /models/{id}`, `POST /models/{id}/validation`, `POST /models/{id}/shadow`, `POST /models/kill-switch`; two of them change meaning, section 2 and Decisions DR-271), 9 rows added to 3.15, 1 row added to 3.12 (`GET /alerts/{id}/model-score`); 5 tables in schema `admin` and 1 in schema `alert` in 3 migrations (`0085` to `0087`); 2 altered CHECKs of phase 24 (`admin.registers.source`, `admin.fairness_metrics.metric`); 4 new job kinds (`model_import_verify`, `model_score_sweep`, `model_monitor`, `model_label_gate`) and 1 extended (phase 24 `fairness_monthly`); no timer kind; no outbox topic; 1 Cedar file; 17 DR numbers)
**Binaries touched:** akr-api (routers `models` and `models_kill`, `ModelRegistry`, `ModelKillSwitchService` and `ModelReads` in `AppState`; the `alerts` router gains `model-score`; the phase 17 queue items query gains the rank join) | akr-worker (jobs `model_import_verify`, `model_score_sweep`, `model_monitor`, `model_label_gate`; `ModelScorer`, `ExplanationService`, `ModelMonitor`, `ModelLabelGateService`, the `ModelPromotion` and `ModelSwitch` change targets registered in process; `ort` and `tract` linked into akr-worker only) | akr-ingestd unchanged | akr-assess unchanged (no model runs in it)

**Requirements carried:**
- Implemented here: T-13 (the scorer ranks rule alerts only; no suppression, no creation, no class change; forced low-score sample reused as the unbiased set, section 3.6), T-14 (the default family is a gradient-boosted tree ensemble on ONNX with monotone constraints declared in the manifest and checked at import and in validation; a glass-box baseline is the benchmark of section 3.6), T-15 (label gates per family kind, about 50, 200 and 500 QA-confirmed positives, as governed settings, and the model-ladder read of route 4), T-63 (starter families flagged `starter`, with a recalibration due date), TS-16 (signed ONNX through `ort` with `tract` fallback, TreeSHAP in house with golden tests, signed registry, shadow before use, drift monitoring), M-13 (the ML term of the score that phase 17 left out: it ranks inside the class, section 3.4)
- Implemented here (model part): GV-03 (the seven tiering answers stored in `inventory_items.tier_basis` and required for kind `model`), GV-05 (starter flag and recalibration due), GV-14 (no promotion without beating the transparent baseline on the bank's out-of-time data), GV-16 (evaluation results carry their out-of-time window, confidence intervals and by-segment rows), GV-18 (score drift, feature drift and override monitors with green, amber and red bands, section 3.8), GV-19 (a red breach opens a register finding that closes only with one of five recorded decisions), GV-20 (per-model and global kill switch to rules only, step-up, tested quarterly through the phase 24 calendar), GV-21 (overrides defined from phase 18 decisions, a per-model override-rate breach opens a fitness review), GV-24 (shadow with success criteria frozen at start, no parallel run of the old system), GV-25 (a permanent challenger beside a Tier 1 champion), GV-29 (version pin; a release never replaces a live model; advance notice on the registry read), GV-30 (card with the ten sections of research 29 section 9g), GV-39 (only QA-confirmed outcomes count as labels; the forced sample is the evaluation set), GV-40 (scores are readable only under the phase 17 purpose and visibility rules; no export route returns scores), GV-41 (model part: reason codes in Arabic and English tied to catalogue features, global and local attribution by TreeSHAP), GV-54 (model part: operating points and raw weights visible to MRO and M only)
- Connected to earlier phases (carried there, reused here): GV-01 and GV-04 (phase 04 inventory), GV-09 and GV-15 (phase 24 validation pack, parts P1 to P7 and the per-bank `bank_delta` key), GV-22 (phase 24 calendar; the kill-switch drill row is an addition, section 3.8), GV-32 (phase 24 and 04 validator never the owner), GV-34 (the findings register used by section 3.7), GV-36 and GV-37 (phase 24 register and proxy check, reused on features in section 3.5), GV-38 (phase 24 monthly metrics; one metric value is added in section 3.5)
- Not carried: GV-02 (version numbering is a unique id per version; the calibration-change clock is phase 24 Open doubts), GV-06, GV-07, GV-08, GV-10 to GV-13, GV-17 (phases 16, 24 and 26 or later, Open doubts), GV-23 (phase 24 KPI pack), GV-26 (phase 04 change classes; this phase uses them), GV-27 (rebuild from snapshot: the artifact and manifest are immutable, the training pipeline is outside the product), GV-28 (dynamic-model policy: no retraining runs in the product; any newer artifact is a new version and a new change), GV-31, GV-33 (validator workbench), GV-42 (automation-bias gate before closure of top-band alerts: Open doubts), GV-43 to GV-53 (GenAI and third-party checklist: phase 28 or Open doubts), GV-44 (customer notice text: phase 22), T-48 to T-51 and M-70 to M-73 (phase 28), TS-17 (phase 28)
- Gap pass 1 (coverage), partly implemented: SP-91 (signed ONNX artifacts, the operator allow-list, the golden check with fallback; GGUF reading in DR-396), SP-92 (golden regression and the forced low-score sample).
- Gap pass 1 (coverage), not carried: T-19 and T-20 (anomaly evaluation suite and graduation path), T-25 (graph neural networks), T-46 (explanation tests), T-52 to T-58 (cross-bank data model, sharing readiness, CBI reference designs, federation lab, public benchmarks, synthetic generator, leave-one-bank-out), T-62 (weak labels), T-64 (active-learning queue), T-66 (maturity ladder); not built by this phase, DR-399; W-02 (supervised and unsupervised scoring), W-11 (model governance pack); P2, DR-400; F-20 (explainable ML; P2, DR-401); VA-52 (cross-bank signal sharing; P2, DR-402); VB-04, VB-08, VB-11, VB-15, VB-19, VB-20 and VB-27 (model items; P2, DR-403).

---

## 1. SQLx Migration Changes

Three migrations, numbered after phase 26's `0084` (phase 25 used `0080` to `0082`; no other phase uses `0085` to `0087`; grep of docs/plan confirms). The governance tables are in schema `admin` (DATA-MODEL section 15 keeps the names `models` and `model_versions` and the schema of the registry). **`model_scores` is in schema `alert`, not `admin`:** a score is one row per alert, it must hold a composite foreign key `(alert_id, alert_created_at)` into the partitioned `alert.alerts` (phase 17 PK `(id, created_at)`), it takes the R5 alert clock through its owner, and placing it next to `alert.alerts` keeps the same role grants. The roadmap row 27 (lines 138, 178 and 216 of roadmap.md, which name `models`, `model_versions` and `model_scores` and the services) is aligned by the lead (Open doubts 14).

Six tables are created: `admin.models`, `admin.model_versions`, `admin.model_evaluations`, `admin.model_kill_state`, `admin.model_monitor_points` and `alert.model_scores`. Two phase 24 CHECK constraints are widened (section 1.1). No column of an earlier table is added. `alert.alerts` is not altered: the rank is a join, not a new column (DR-257).

**Reconciliation with DATA-MODEL section 15 (changed in the same change, DR-258):**
- `models` and `model_versions`: kept names and schema. Added: `starter`, `recalibration_due`, `inventory_item_id`, `live_version_id`, `pinned_version_id`, `next_kill_drill_due` (family); `version_no`, `inventory_item_id`, `status` with the values `importing` and `refused` added in front of the five of DATA-MODEL, `refusal_code`, `artifact_key`, `artifact_sha256`, `manifest` (the signed fields), `manifest_sha256`, `signature`, `signing_key_version`, `card_evidence_id`, `feature_keys`, `monotone`, `runtime`, `opset`, `reference_bins`, `top_band_cut`, `shadow_started_at`, `shadow_criteria` (frozen), `promotion_requested_at`, `live_from`, `retired_at` (versions). DATA-MODEL's `name`, `purpose`, `tier` (read from the item, not copied), `artifact_object_key`, `signature`, `card`, `training_manifest` (hashes only) and `label_gate` are mapped as: `tier` is read from `inventory_items.tier`; `artifact_object_key` is `artifact_key`; `card` is the evidence object `card_evidence_id`; `training_manifest` is the `manifest` key `training` (hashes only); `label_gate` is the `label_gate` evaluations of `model_evaluations`.
- `model_scores`: DATA-MODEL's `alert_id`, `alert_created_at`, `model_version_id`, `score`, `shap` and `scored_at` are kept, in schema `alert`. Added: `role` (`live`, `challenger`, `shadow`), `top_band`, `reason_codes`. Shadow and challenger rows are stored under the same table and are never returned by a route.
- New, not in DATA-MODEL: `admin.model_evaluations` (benchmark, out-of-time, calibration, label gate, shadow summary, kill drill, sensitivity), `admin.model_kill_state` (one row per switch scope), `admin.model_monitor_points` (PSI, override rate and counts per day, feature and version).

### 1.1 `0085_admin_models_registry.sql`

```sql
-- migration 0085_admin_models_registry.sql
-- Phase 27. Model families, versions, evaluations, kill-switch state and monitoring points (schema admin),
-- and the two widened CHECKs of phase 24. Idempotent: CREATE IF NOT EXISTS, constraints dropped and re-added.

CREATE TABLE IF NOT EXISTS admin.models (
    id                    UUID PRIMARY KEY,                       -- UUIDv7, app-side
    name                  JSONB NOT NULL CHECK (name ? 'ar' AND name ? 'en'),
    purpose               JSONB NOT NULL CHECK (purpose ? 'ar' AND purpose ? 'en'),
    starter               BOOLEAN NOT NULL DEFAULT FALSE,         -- GV-05, T-63: externally calibrated, temporary
    recalibration_due     DATE,                                   -- starter families only
    inventory_item_id     UUID NOT NULL UNIQUE REFERENCES admin.inventory_items(id) ON DELETE RESTRICT,   -- kind 'model', object = this id
    live_version_id       UUID,                                   -- FK added below (circular)
    pinned_version_id     UUID,                                   -- FK added below (GV-29)
    next_kill_drill_due   DATE NOT NULL,                          -- GV-20: quarterly drill, read by the phase 24 calendar
    created_by            UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    version               INT NOT NULL DEFAULT 1,
    CHECK (starter = (recalibration_due IS NOT NULL))
);
-- At most one family holds a live version in phase 27: the queue rank join reads one live version per bank.
CREATE UNIQUE INDEX IF NOT EXISTS uq_models_one_live_family ON admin.models ((live_version_id IS NOT NULL))
    WHERE live_version_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS admin.model_versions (
    id                      UUID PRIMARY KEY,
    model_id                UUID NOT NULL REFERENCES admin.models(id) ON DELETE RESTRICT,
    version_no              INT NOT NULL CHECK (version_no >= 1),
    inventory_item_id       UUID NOT NULL UNIQUE REFERENCES admin.inventory_items(id) ON DELETE RESTRICT,  -- kind 'model', object = this id
    status                  TEXT NOT NULL CHECK (status IN ('importing', 'refused', 'registered', 'shadow', 'challenger', 'live', 'retired')),
    refusal_code            TEXT CHECK (refusal_code IS NULL OR refusal_code IN ('signature_invalid', 'manifest_invalid', 'sha_mismatch',
                                        'operator_unsupported', 'golden_mismatch', 'feature_not_in_catalogue', 'prohibited_feature',
                                        'proxy_flagged', 'suspicion_source_feature', 'monotone_unsound')),
    artifact_key            TEXT NOT NULL CHECK (artifact_key ~ '^[a-z0-9][a-z0-9_./-]{2,200}[.]onnx$'),
    artifact_sha256         TEXT NOT NULL CHECK (artifact_sha256 ~ '^[0-9a-f]{64}$'),
    manifest                JSONB NOT NULL CHECK (jsonb_typeof(manifest) = 'object'),   -- signed fields; hashes and declared values only
    manifest_sha256         TEXT NOT NULL CHECK (manifest_sha256 ~ '^[0-9a-f]{64}$'),
    signature               TEXT NOT NULL CHECK (length(signature) BETWEEN 86 AND 88),  -- base64 of 64 bytes (Ed25519)
    signing_key_version     INT NOT NULL CHECK (signing_key_version >= 1),
    card_evidence_id        UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    feature_keys            JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(feature_keys) = 'array'),   -- [{key, version}]
    monotone                JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(monotone) = 'object'),      -- {key: +1 | -1}
    runtime                 TEXT CHECK (runtime IS NULL OR runtime IN ('ort', 'tract')),
    opset                   JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(opset) = 'object'),
    reference_bins          JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(reference_bins) = 'object'),  -- histogram counts, no rows
    top_band_cut            NUMERIC(6,4) CHECK (top_band_cut IS NULL OR top_band_cut BETWEEN 0 AND 1),
    shadow_started_at       TIMESTAMPTZ,
    shadow_criteria         JSONB CHECK (shadow_criteria IS NULL OR jsonb_typeof(shadow_criteria) = 'object'),  -- frozen at shadow start
    promotion_requested_at  TIMESTAMPTZ,
    live_from               TIMESTAMPTZ,
    retired_at              TIMESTAMPTZ,
    created_by              UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    version                 INT NOT NULL DEFAULT 1,
    CONSTRAINT uq_model_version_no UNIQUE (model_id, version_no),
    CONSTRAINT uq_model_artifact UNIQUE (artifact_sha256),          -- one artifact, one version
    CHECK ((status = 'refused') = (refusal_code IS NOT NULL)),
    CHECK ((status IN ('registered', 'shadow', 'challenger', 'live', 'retired')) = (runtime IS NOT NULL)),
    CHECK (status NOT IN ('shadow', 'challenger', 'live') OR card_evidence_id IS NOT NULL),
    CHECK (status NOT IN ('shadow', 'challenger') OR (shadow_started_at IS NOT NULL AND shadow_criteria IS NOT NULL)),
    CHECK (status <> 'live' OR live_from IS NOT NULL),
    CHECK ((status = 'retired') = (retired_at IS NOT NULL))
);
CREATE INDEX IF NOT EXISTS idx_model_versions_model ON admin.model_versions (model_id, status);
CREATE UNIQUE INDEX IF NOT EXISTS uq_model_one_shadow     ON admin.model_versions (model_id) WHERE status = 'shadow';
CREATE UNIQUE INDEX IF NOT EXISTS uq_model_one_challenger ON admin.model_versions (model_id) WHERE status = 'challenger';
CREATE UNIQUE INDEX IF NOT EXISTS uq_model_one_live       ON admin.model_versions (model_id) WHERE status = 'live';

DO $$ BEGIN
    ALTER TABLE admin.models ADD CONSTRAINT fk_models_live_version FOREIGN KEY (live_version_id)
        REFERENCES admin.model_versions(id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED;
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    ALTER TABLE admin.models ADD CONSTRAINT fk_models_pinned_version FOREIGN KEY (pinned_version_id)
        REFERENCES admin.model_versions(id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED;
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

-- Evaluations: benchmark, out-of-time, calibration, label gate, shadow summary, kill drill, sensitivity. Written once.
CREATE TABLE IF NOT EXISTS admin.model_evaluations (
    id                  UUID PRIMARY KEY,
    model_id            UUID NOT NULL REFERENCES admin.models(id) ON DELETE RESTRICT,
    model_version_id    UUID REFERENCES admin.model_versions(id) ON DELETE RESTRICT,
    kind                TEXT NOT NULL CHECK (kind IN ('benchmark', 'out_of_time', 'calibration', 'label_gate',
                                                      'shadow_summary', 'kill_drill', 'sensitivity')),
    result              JSONB NOT NULL CHECK (jsonb_typeof(result) = 'object'),   -- counts, metrics, CIs, bands; no ids of alerts or parties
    passed              BOOLEAN NOT NULL,
    data_as_of          TIMESTAMPTZ NOT NULL,
    created_by          UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (kind = 'kill_drill' OR model_version_id IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_model_evaluations_version ON admin.model_evaluations (model_version_id, kind, created_at);

-- Kill-switch state: one row per scope ('global' or 'model:<family id>'). Engaging is immediate; releasing is a change request.
CREATE TABLE IF NOT EXISTS admin.model_kill_state (
    scope           TEXT PRIMARY KEY CHECK (scope = 'global' OR scope ~ '^model:[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$'),
    engaged         BOOLEAN NOT NULL,
    reason_code     TEXT CHECK (reason_code IS NULL OR reason_code IN ('model_error', 'data_break', 'red_breach',
                                                                      'regulator_request', 'other')),
    engaged_by      UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    engaged_at      TIMESTAMPTZ,
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    version         INT NOT NULL DEFAULT 1,
    CHECK (NOT engaged OR (engaged_by IS NOT NULL AND engaged_at IS NOT NULL AND reason_code IS NOT NULL))
);
INSERT INTO admin.model_kill_state (scope, engaged) VALUES ('global', FALSE) ON CONFLICT (scope) DO NOTHING;

-- Monitoring points: one row per (version, day, metric, feature). Written once; suppressed rows carry no value.
CREATE TABLE IF NOT EXISTS admin.model_monitor_points (
    id                  UUID PRIMARY KEY,
    model_version_id    UUID NOT NULL REFERENCES admin.model_versions(id) ON DELETE RESTRICT,
    day                 DATE NOT NULL,
    metric              TEXT NOT NULL CHECK (metric IN ('score_psi', 'feature_psi', 'override_rate', 'scored_count')),
    feature_key         TEXT CHECK (feature_key IS NULL OR feature_key ~ '^[a-z][a-z0-9_]{2,62}$'),
    value               NUMERIC(10,6),
    population          INT CHECK (population IS NULL OR population >= 0),
    band                TEXT NOT NULL CHECK (band IN ('green', 'amber', 'red', 'suppressed', 'not_run')),
    register_id         UUID REFERENCES admin.registers(id) ON DELETE RESTRICT,   -- set on a red breach
    computed_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK ((band IN ('suppressed', 'not_run')) = (value IS NULL)),
    CHECK ((metric = 'feature_psi') = (feature_key IS NOT NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_model_monitor_point
    ON admin.model_monitor_points (model_version_id, day, metric, COALESCE(feature_key, ''));

-- Phase 24 CHECK widened, state here (Decisions DR-265, DR-268). Constraint names are the PostgreSQL defaults of the
-- inline CHECKs in phase 24 sections 1.1 and 1.2 (registers_source_check, fairness_metrics_metric_check).
ALTER TABLE admin.registers DROP CONSTRAINT IF EXISTS registers_source_check;
ALTER TABLE admin.registers ADD CONSTRAINT registers_source_check
    CHECK (source IN ('manual', 'post_review', 'fairness', 'review_calendar', 'scan', 'model_monitor'));
ALTER TABLE admin.fairness_metrics DROP CONSTRAINT IF EXISTS fairness_metrics_metric_check;
ALTER TABLE admin.fairness_metrics ADD CONSTRAINT fairness_metrics_metric_check
    CHECK (metric IN ('alert_rate', 'case_rate', 'str_rate', 'hold_rate', 'score_top_band_rate'));
```

Note on `model_versions`: a row in status `shadow` or `challenger` always has `shadow_started_at` and the frozen `shadow_criteria`; a `live` version keeps the columns it was shadowed with, and a version that went live without a shadow period cannot exist (the promotion check of section 3.6 refuses it).

### 1.2 `0086_alert_model_scores.sql`

```sql
-- migration 0086_alert_model_scores.sql
-- Phase 27. Model scores per alert and version (schema alert, R5 by owner). Written once; never updated.
CREATE TABLE IF NOT EXISTS alert.model_scores (
    alert_id            UUID NOT NULL,
    alert_created_at    TIMESTAMPTZ NOT NULL,
    model_version_id    UUID NOT NULL REFERENCES admin.model_versions(id) ON DELETE RESTRICT,
    role                TEXT NOT NULL CHECK (role IN ('live', 'challenger', 'shadow')),
    score               NUMERIC(6,4) NOT NULL CHECK (score BETWEEN 0 AND 1),
    top_band            BOOLEAN NOT NULL,
    reason_codes        TEXT[] NOT NULL DEFAULT '{}' CHECK (cardinality(reason_codes) <= 3),   -- catalogue keys 'feature_key@version'
    shap                JSONB NOT NULL DEFAULT '[]'::jsonb
                        CHECK (jsonb_typeof(shap) = 'array' AND jsonb_array_length(shap) <= 5),  -- live rows only; shadow rows hold []
    scored_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (alert_id, alert_created_at, model_version_id),
    CONSTRAINT fk_model_scores_alert FOREIGN KEY (alert_id, alert_created_at)
        REFERENCES alert.alerts(id, created_at) ON DELETE RESTRICT
);
CREATE INDEX IF NOT EXISTS idx_model_scores_version_day ON alert.model_scores (model_version_id, scored_at);
CREATE INDEX IF NOT EXISTS idx_model_scores_role_day    ON alert.model_scores (role, scored_at);
```

### 1.3 `0087_admin_models_guards_grants.sql`

```sql
-- migration 0087_admin_models_guards_grants.sql
-- Phase 27. Guards the application cannot bypass, grants per write path, settings, classification and retention.
-- Idempotent: functions replaced, triggers dropped before creation, inserts use ON CONFLICT DO NOTHING.

-- Version guard. Identity and artifact fields never change; status moves only along the listed edges;
-- 'live' is set only inside ModelPromotion::activate, which sets the transaction flag akr.model_promotion first.
CREATE OR REPLACE FUNCTION admin.model_versions_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        IF current_user = 'akr_retention' THEN RETURN OLD; END IF;   -- phase 23 retention only, after the FK children are gone
        RAISE EXCEPTION 'model versions are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.id IS DISTINCT FROM OLD.id OR NEW.model_id IS DISTINCT FROM OLD.model_id
       OR NEW.version_no IS DISTINCT FROM OLD.version_no OR NEW.inventory_item_id IS DISTINCT FROM OLD.inventory_item_id
       OR NEW.artifact_key IS DISTINCT FROM OLD.artifact_key OR NEW.artifact_sha256 IS DISTINCT FROM OLD.artifact_sha256
       OR NEW.manifest IS DISTINCT FROM OLD.manifest OR NEW.manifest_sha256 IS DISTINCT FROM OLD.manifest_sha256
       OR NEW.signature IS DISTINCT FROM OLD.signature OR NEW.signing_key_version IS DISTINCT FROM OLD.signing_key_version
       OR (OLD.shadow_criteria IS NOT NULL AND NEW.shadow_criteria IS DISTINCT FROM OLD.shadow_criteria
           AND NOT (OLD.status = 'retired' AND NEW.status IN ('shadow', 'challenger'))) THEN   -- rollback: new criteria
        RAISE EXCEPTION 'model version identity, artifact, manifest and frozen criteria are immutable'
            USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.status IS DISTINCT FROM OLD.status THEN
        IF (NEW.status = 'live' OR OLD.status = 'live')
           AND current_setting('akr.model_promotion', true) IS DISTINCT FROM 'on' THEN
            RAISE EXCEPTION 'live is entered and left only by the promotion change' USING ERRCODE = 'insufficient_privilege';
        END IF;
        IF (OLD.status, NEW.status) NOT IN (
               ('importing', 'registered'), ('importing', 'refused'),
               ('registered', 'shadow'), ('registered', 'challenger'), ('registered', 'retired'),
               ('shadow', 'challenger'), ('shadow', 'live'), ('shadow', 'retired'),
               ('challenger', 'live'), ('challenger', 'retired'),
               ('retired', 'shadow'), ('retired', 'challenger'),   -- rollback: a new shadow period with its own criteria
               ('live', 'retired')) THEN
            RAISE EXCEPTION 'model version status % -> % is not allowed', OLD.status, NEW.status
                USING ERRCODE = 'check_violation';
        END IF;
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_model_versions_guard ON admin.model_versions;
CREATE TRIGGER trg_model_versions_guard BEFORE UPDATE OR DELETE ON admin.model_versions
    FOR EACH ROW EXECUTE FUNCTION admin.model_versions_guard();

-- Family guard: identity and starter flag fixed; the live and pinned pointers move only in the promotion or pin paths.
CREATE OR REPLACE FUNCTION admin.models_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        IF current_user = 'akr_retention' THEN RETURN OLD; END IF;   -- phase 23 retention only, after its versions
        RAISE EXCEPTION 'model families are deleted only by retention' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.id IS DISTINCT FROM OLD.id OR NEW.inventory_item_id IS DISTINCT FROM OLD.inventory_item_id
       OR NEW.starter IS DISTINCT FROM OLD.starter OR NEW.created_by IS DISTINCT FROM OLD.created_by THEN
        RAISE EXCEPTION 'model family identity is immutable' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.live_version_id IS DISTINCT FROM OLD.live_version_id
       AND current_setting('akr.model_promotion', true) IS DISTINCT FROM 'on' THEN
        RAISE EXCEPTION 'the live pointer moves only in the promotion change' USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_models_guard ON admin.models;
CREATE TRIGGER trg_models_guard BEFORE UPDATE OR DELETE ON admin.models
    FOR EACH ROW EXECUTE FUNCTION admin.models_guard();

-- Evaluations and monitoring points are written once. Only the phase 23 retention role may delete them.
CREATE OR REPLACE FUNCTION admin.model_evidence_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' AND current_user = 'akr_retention' THEN RETURN OLD; END IF;
    RAISE EXCEPTION '% rows are written once; a new result is a new row', TG_TABLE_NAME
        USING ERRCODE = 'insufficient_privilege';
END $$;
DROP TRIGGER IF EXISTS trg_model_evaluations_guard ON admin.model_evaluations;
CREATE TRIGGER trg_model_evaluations_guard BEFORE UPDATE OR DELETE ON admin.model_evaluations
    FOR EACH ROW EXECUTE FUNCTION admin.model_evidence_guard();
DROP TRIGGER IF EXISTS trg_model_monitor_points_guard ON admin.model_monitor_points;
CREATE TRIGGER trg_model_monitor_points_guard BEFORE UPDATE OR DELETE ON admin.model_monitor_points
    FOR EACH ROW EXECUTE FUNCTION admin.model_evidence_guard();

-- Grants per write path. akr_app is the API and worker role. No table is granted to akr_readonly.
GRANT SELECT, INSERT ON admin.models TO akr_app;
GRANT UPDATE (name, purpose, recalibration_due, live_version_id, pinned_version_id, next_kill_drill_due, updated_at, version)
    ON admin.models TO akr_app;
GRANT SELECT, INSERT ON admin.model_versions TO akr_app;
GRANT UPDATE (status, refusal_code, card_evidence_id, feature_keys, monotone, runtime, opset, reference_bins, top_band_cut,
              shadow_started_at, shadow_criteria, promotion_requested_at, live_from, retired_at, updated_at, version)
    ON admin.model_versions TO akr_app;
GRANT SELECT, INSERT ON admin.model_evaluations TO akr_app;
GRANT SELECT, INSERT ON admin.model_monitor_points TO akr_app;
GRANT SELECT, INSERT ON admin.model_kill_state TO akr_app;
GRANT UPDATE (engaged, reason_code, engaged_by, engaged_at, updated_at, version) ON admin.model_kill_state TO akr_app;
GRANT SELECT, INSERT ON alert.model_scores TO akr_app;        -- no UPDATE grant exists: a score is never changed
-- Retention: SELECT on every new table (phase 23 grants SELECT on ALL TABLES only at its own time); DELETE only on
-- alert.model_scores, deleted by record_purge before an alert partition drop (DR-386, DR-387).
GRANT SELECT ON admin.models, admin.model_versions, admin.model_evaluations, admin.model_monitor_points,
    admin.model_kill_state TO akr_retention;
GRANT SELECT, DELETE ON alert.model_scores TO akr_retention;

-- Settings (copy of the phase 11 and 12 insert format). floor only when governed = TRUE (phase 00 CHECK).
-- A floor bounds the side that weakens the control: a shorter shadow, a lower gate and a higher trigger are the weakening sides.
INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('model.shadow_min_days',            '30', 'integer', '{"min": 30, "basis": "GV-24; equal to the phase 16 scenario shadow of 30 days"}'::jsonb, TRUE,
  '{"ar": "أدنى مدة للتشغيل التجريبي للنموذج (أيام)", "en": "Shortest shadow run of a model (days)"}'),
 ('model.gate_recalibrate_positives', '50', 'integer', '{"min": 50, "basis": "T-15 and research 21 4f: starter recalibration on local labels"}'::jsonb, TRUE,
  '{"ar": "حد الحالات المؤكدة لمعايرة النموذج المبدئي", "en": "QA-confirmed positives to recalibrate a starter model"}'),
 ('model.gate_shadow_positives',      '200', 'integer', '{"min": 200, "basis": "T-15: bank-trained models enter shadow from about 200 positives"}'::jsonb, TRUE,
  '{"ar": "حد الحالات المؤكدة للتشغيل التجريبي لنموذج مدرّب محليا", "en": "QA-confirmed positives before a bank-trained model enters shadow"}'),
 ('model.gate_live_positives',        '500', 'integer', '{"min": 500, "basis": "T-15: a local supervised model goes live from about 500 positives"}'::jsonb, TRUE,
  '{"ar": "حد الحالات المؤكدة للتفعيل لنموذج مدرّب محليا", "en": "QA-confirmed positives before a bank-trained model may go live"}'),
 ('model.psi_watch',                  '0.10', 'decimal', '{"max": 0.10, "basis": "research 29 9d: PSI 0.10 watch level; a bank may only tighten it"}'::jsonb, TRUE,
  '{"ar": "حد مؤشر الاستقرار للمراقبة", "en": "Population stability index: amber watch level"}'),
 ('model.psi_action',                 '0.25', 'decimal', '{"max": 0.25, "basis": "research 29 9d: PSI 0.25 action level"}'::jsonb, TRUE,
  '{"ar": "حد مؤشر الاستقرار للإجراء", "en": "Population stability index: red action level"}'),
 ('model.override_rate_action',       '0.30', 'decimal', '{"max": 0.30, "basis": "GV-21 fitness review; design proposal, Open doubts 2"}'::jsonb, TRUE,
  '{"ar": "نسبة التجاوز التي تفتح مراجعة ملاءمة النموذج", "en": "Override rate that opens a fitness review"}'),
 ('model.kill_drill_months',          '3', 'integer', '{"max": 3, "basis": "GV-20: kill switch tested quarterly"}'::jsonb, TRUE,
  '{"ar": "مدة بين تجارب إيقاف النموذج (أشهر)", "en": "Months between kill-switch drills"}'),
 ('model.top_band_percent',           '10', 'integer', NULL, FALSE,
  '{"ar": "نسبة أعلى الدرجات المعتبرة النطاق الأعلى", "en": "Share of highest scores that form the top band (per cent)"}'),
 ('model.label_window_days',          '180', 'integer', NULL, FALSE,
  '{"ar": "نافذة الحالات المؤكدة لبوابة الحالات (أيام)", "en": "Window of QA-confirmed outcomes for the label gate (days)"}'),
 ('model.sweep_batch',                '500', 'integer', NULL, FALSE,
  '{"ar": "حجم دفعة تسجيل الدرجات", "en": "Alerts scored per sweep batch"}'),
 ('schedule.model_score_sweep',       '"0 * * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول تسجيل درجات النموذج (كل دقيقة)", "en": "Model score sweep (every minute)"}'),
 ('schedule.model_monitor',           '"0 0 5 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مراقبة النموذج اليومية", "en": "Daily model monitor (05:00 bank time)"}'),
 ('schedule.model_label_gate',        '"0 0 4 * * 1"', 'cron', NULL, FALSE,
  '{"ar": "جدول بوابة الحالات المؤكدة الأسبوعية", "en": "Weekly label gate (Monday 04:00 bank time)"}')
ON CONFLICT (key) DO NOTHING;

-- Column classification (copy of the phase 24 insert format).
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('models', 'internal', ARRAY['id', 'name', 'purpose', 'starter', 'recalibration_due', 'inventory_item_id', 'live_version_id',
                               'pinned_version_id', 'next_kill_drill_due', 'created_by', 'created_at', 'updated_at', 'version']),
  ('model_versions', 'internal', ARRAY['id', 'model_id', 'version_no', 'inventory_item_id', 'status', 'refusal_code',
                                       'artifact_key', 'artifact_sha256', 'manifest_sha256', 'signature', 'signing_key_version',
                                       'card_evidence_id', 'feature_keys', 'runtime', 'opset', 'top_band_cut', 'shadow_started_at',
                                       'promotion_requested_at', 'live_from', 'retired_at', 'created_by', 'created_at', 'updated_at', 'version']),
  ('model_versions', 'detection-logic', ARRAY['manifest', 'monotone', 'reference_bins', 'shadow_criteria']),
  ('model_evaluations', 'internal', ARRAY['id', 'model_id', 'model_version_id', 'kind', 'passed', 'data_as_of', 'created_by', 'created_at']),
  ('model_evaluations', 'detection-logic', ARRAY['result']),
  ('model_kill_state', 'internal', ARRAY['scope', 'engaged', 'reason_code', 'engaged_by', 'engaged_at', 'updated_at', 'version']),
  ('model_monitor_points', 'internal', ARRAY['id', 'model_version_id', 'day', 'metric', 'feature_key', 'value', 'population',
                                             'band', 'register_id', 'computed_at'])
) AS v(t, cls, cols) CROSS JOIN LATERAL unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'alert', 'model_scores', c, v.cls::admin.data_classification
FROM (VALUES
  ('detection-logic', ARRAY['score', 'top_band', 'reason_codes', 'shap']),
  ('internal', ARRAY['alert_id', 'alert_created_at', 'model_version_id', 'role', 'scored_at'])
) AS v(cls, cols) CROSS JOIN LATERAL unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

-- Retention (copy of the phase 23 insert format). Clock columns as the rules of phase 23 require.
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'models',               'R11', 'retirement', NULL,          'Family kept with its versions; never purged while a version is referenced'),
 ('admin', 'model_versions',       'R11', 'retirement', NULL,          'Versions with card, manifest and signature; a retired version is the rollback point'),
 ('admin', 'model_evaluations',    'R11', 'age',        'created_at',  'Benchmark, out-of-time, gate and shadow evidence: ten years (DATA-MODEL R11)'),
 ('admin', 'model_monitor_points', 'R9',  'age',        'computed_at', 'Monitoring aggregates kept with the model history'),
 ('admin', 'model_kill_state',     'R10', 'never',      NULL,          'Kill-switch state of each scope: kept without limit'),
 ('alert', 'model_scores',         'R5',  'owner',      NULL,          'Follows its alert; the phase 23 engine deletes these rows before it drops an alert partition')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

Note on the `model_scores` retention row: its FK is `ON DELETE RESTRICT` into the partitioned `alert.alerts`, so a phase 23 partition drop is refused while any score row points into that partition (conventions, phase 23 review). The phase 23 engine must delete the child rows of the partition first, as a `record_purge` of this table, before `DETACH` or `DROP`. This is a requirement on phase 23, listed in Open doubts 13.

---

## 2. DDD Implementation (Rust)

- **Crate placement:** pure rules in `domain::models` (akr-core, no IO): the signed-manifest rule, the operator allow-list, the band and override arithmetic, PSI, the top-band cut, the label-gate counter, the seven-part mapping of section 3.6 and the reason-code selection (top three positive contributions). Services in `services::models` (akr-api for the registry, import request, kill switch and reads; akr-worker for verification, scoring, monitoring and the gate). Repositories in `db::models` (sqlx, `akr_app`). The ONNX runtimes are behind `crates/ml/` (new crate, worker only: `ort` and `tract`); akr-api has no dependency on them.
- **Names not used by phases 00 to 26 are new in phase 27** (`ArtifactStore`, `SignatureVerifier`, `OnnxInspector`, `ModelRuntime`, the `crates/ml` crate, the `model_version` owner kind, the `ModelKillSwitch` step-up constant, the `akr-ctl model keys import` command, and the enum values of section 1). The names of the services below were grepped and are free.
- **New names (checked with grep across docs/plan):** `ModelRegistry`, `ModelImportVerifier`, `ModelScorer`, `ExplanationService`, `ModelMonitor`, `ModelLabelGateService`, `ModelPromotion` (change target `model_version`), `ModelSwitch` (change target `model_switch`), `ModelKillSwitchService` (not `KillSwitch`: phase 16 has `KillSwitchService` for scenarios), `ModelReads`. Phase 16 `KillSwitchService` and phase 15 `RatingModelService` are not touched.
- **Domain entities** (`Debug, Serialize, Deserialize, FromRow, ToSchema`):
  - `Model { id, name: Bilingual, purpose: Bilingual, starter, recalibration_due, inventory_item_id, live_version_id, pinned_version_id, next_kill_drill_due, created_by, created_at, updated_at, version }`
  - `ModelVersion { id, model_id, version_no, inventory_item_id, status: VersionStatus, refusal_code: Option<RefusalCode>, artifact_key, artifact_sha256, manifest: Value, manifest_sha256, signature, signing_key_version, card_evidence_id, feature_keys: Vec<FeatureRef>, monotone: BTreeMap<String, i8>, runtime: Option<Runtime>, opset: Value, reference_bins: Value, top_band_cut: Option<Decimal>, shadow_started_at, shadow_criteria: Option<Value>, promotion_requested_at, live_from, retired_at, created_by, created_at, updated_at, version }`. `VersionStatus` = importing, refused, registered, shadow, challenger, live, retired.
  - `ModelEvaluation { id, model_id, model_version_id: Option<Uuid>, kind: EvalKind, result: Value, passed, data_as_of, created_by, created_at }`
  - `KillState { scope: KillScope (Global | Model(Uuid)), engaged, reason_code: Option<KillReason>, engaged_by, engaged_at, version }`
  - `MonitorPoint { id, model_version_id, day, metric: MonitorMetric, feature_key, value: Option<Decimal>, population, band: Band, register_id, computed_at }`
  - `ModelScore { alert_id, alert_created_at, model_version_id, role: ScoreRole, score: Decimal, top_band, reason_codes: Vec<String>, shap: Vec<Contribution>, scored_at }`
- **Create and update DTOs:**
  - `CreateModel { name, purpose, starter, recalibration_due?, inventory: { tier, tier_basis, developer, committee, user_roles, owner_id, next_validation_due? } }`. `inventory` creates the family item with `kind = model` and `object_id = models.id`, which the service generates first.
  - `ImportVersion { manifest: Value, manifest_sha256, signature, signing_key_version, artifact_key, card: Value }`. `card` must hold the ten sections of section 3.2 (validated by the schema of the card, not free text).
  - `RecordEvaluation { version_id, kind, result: Value, passed: bool, data_as_of }` (`result` is checked against the kind's schema in section 3.6).
  - `StartShadow { version_id, role: ShadowRole (shadow | challenger), success_criteria: Value }` (criteria are frozen by the row; no update DTO exists).
  - `EngageKill { scope: KillScope, reason_code }`; `PinVersion { version_id: Option<Uuid> }`; `RecordDrill { environment, engaged_at, released_at, rules_only_checked: bool, notes_code }`; `ReleaseKillRequest { scope, reason_text_code }`.
- **Repository traits** (domain layer; `&mut Tx` on writes; `If-Match` on versioned rows through `WHERE version = $expected`):
  - `ModelRepository`: `insert_family(tx, NewModel)`, `lock_family(tx, id) -> Model` (`FOR UPDATE`), `set_pointers(tx, id, live, pinned, drill_due)`, `list(conn, filter, keyset)`, `live_version(conn) -> Option<ModelVersion>` (cached 30 s in process).
  - `VersionRepository`: `insert_importing(tx, NewVersion)`, `lock_version(tx, id)`, `set_verified(tx, id, runtime, opset, card_evidence_id, reference_bins, feature_keys, monotone)`, `refuse(tx, id, code)`, `set_status(tx, id, from, to, expected_version)` (the guard enforces the edges), `active_for(conn, model_id) -> Vec<ModelVersion>`.
  - `EvaluationRepository`: `insert(tx, NewEvaluation)`, `latest(conn, version_id, kind) -> Option<ModelEvaluation>`.
  - `KillRepository`: `lock(tx, scope)`, `set_engaged(tx, scope, engaged, reason, by, expected_version)`, `all(conn)`.
  - `ScoreRepository` (akr-worker writes, akr-api reads one alert): `unscored_batch(conn, version_id, live_from, limit)`, `insert_many(tx, Vec<ModelScore>)` (`ON CONFLICT DO NOTHING`), `for_alert_live(conn, alert_id, alert_created_at)`: the latest `live` row of the alert, of any live version (section 3.4 "Single read", step 2).
  - `MonitorRepository`: `insert_many(tx, points)`, `list(conn, version_id, from, to)`.
- **Ports** (domain traits; implementations in `services::models` or `crates/ml`):
  - `ArtifactStore`: `read_sha256(key) -> Result<[u8; 32]>` and `read_bytes(key)` (worker only; a read-only volume mounted at the model store, section 4.4).
  - `SignatureVerifier`: `verify(manifest_bytes, signature, key_version) -> Result<(), SigError>` (Ed25519, verify-only key from OpenBao KV `keys/model-artifact/v<N>.pub`, cached per version).
  - `OnnxInspector`: `operators(artifact) -> Vec<OpRef>` and `graph_io(artifact)` (protobuf read, no execution).
  - `ModelRuntime`: `load(artifact, runtime) -> Session`, `predict(session, rows) -> Vec<f32>` (`ort` first, `tract` as recorded in the version row).
- **Route table** (`| Method | Path | Scope | Description (cedar action) |`; literal paths `/models/kill-switch` and `/models/ladder` are declared before `/models/{id}` so they are not read as ids):

| Method | Path | Scope | Description (cedar action) |
|-|-|-|-|
| GET | `/models` | model.risk, audit.read, dashboard.read | Families with live version, starter flag, next drill and pending version notices, cursor (`model.read`; API 3.15 row, extended) |
| POST | `/models` | model.risk | Create a family and its inventory item; 201; `Idempotency-Key` (`model.create`; API 3.15 row, meaning narrowed, DR-271) |
| GET | `/models/kill-switch` | model.risk, audit.read, dashboard.read | State of the global switch and of each model switch (`model.monitor.read`) (added) |
| GET | `/models/ladder` | model.risk, audit.read, dashboard.read | Model ladder per family: enabled tier, label counts and the next gate, counts only (`model.monitor.read`) (added) |
| GET | `/models/{id}` | model.risk, audit.read, dashboard.read | Family, live, shadow, challenger and pinned versions, gates, next drill (`model.read`; API 3.15 row) |
| POST | `/models/{id}/versions` | model.risk | Import a version: manifest, signature, card; 202 `importing`; synchronous refusal 422 `E_VALIDATION` for a bad signature or manifest; `Idempotency-Key` (`model.version.import`) (added) |
| GET | `/models/{id}/versions/{vid}` | model.risk, audit.read, dashboard.read | Version, card (ten sections), refusal code; `?view=operating_point` adds the thresholds and the top-band cut, MRO and M only (`model.read`; `model.operating_point.read`) (added) |
| POST | `/models/{id}/validation` | model.risk | Record a benchmark, out-of-time, calibration or sensitivity evaluation for a version; `Idempotency-Key` (`model.validation.record`; API 3.15 row, redefined: the pack itself is phase 24's route) |
| POST | `/models/{id}/shadow` | model.risk | Start a shadow or a challenger run with the success criteria frozen; `If-Match` on the version; `Idempotency-Key` (`model.shadow.start`; API 3.15 row) |
| POST | `/models/{id}/versions/{vid}/promote` | model.risk | Open change request `cr_material_validated` for the version (`model.promote.request`); `If-Match` on the version; `Idempotency-Key` (added) |
| POST | `/models/{id}/pin` | model.risk | Pin or unpin a version of the family (section 3.10); SU `approval`; `If-Match` on the family; `Idempotency-Key` (`model.pin`) (added) |
| POST | `/models/kill-switch` | model.risk | Engage the global switch or one model's switch; SU `approval`; `If-Match` on the scope row; `Idempotency-Key` (`model.kill.engage`; API 3.15 row, extended with the per-model scope) |
| POST | `/models/kill-switch/release-requests` | model.risk | Open change request `cr_material` that releases a switch (`model.kill.release_request`); `Idempotency-Key` (added) |
| POST | `/models/{id}/kill-drills` | model.risk | Record a quarterly drill and move `next_kill_drill_due`; `Idempotency-Key` (`model.kill.drill`) (added) |
| GET | `/models/{id}/monitoring` | model.risk, audit.read, dashboard.read | Stored monitoring points of the family's versions: bands, values, suppressed counts, no ids (`model.monitor.read`) (added) |
| GET | `/alerts/{id}/model-score` | queue.work | Live score, top-band flag and up to three reason codes of one alert; never shadow or challenger; suspicion tier masked 404 (`alert.model_score.read`) (added to API 3.12) |

- **Router functions:** `pub fn router() -> Router<AppState>` in `api::routers::models` (the sixteen rows above except the last) and `api::routers::alerts` (the last row, added to phase 17's router).
- **Scope check (API section 2):** every role a Cedar permit of section 3.12 admits is also admitted by the route's scope list: MRO by `model.risk`; IA, M and CH by `audit.read` or `dashboard.read` (M and CH by `dashboard.read` only; IA by `audit.read` and `dashboard.read`); A1, A2 and M by `queue.work`. A route scope may admit a role that no permit names: SEC is admitted by `audit.read` on the six GET routes that carry it, and Cedar refuses it with `E_FORBIDDEN` (403).
- **utoipa schemas:** `ModelView`, `ModelFamilyView`, `ModelVersionView`, `ModelCardView` (ten sections; `operating_point` present only for MRO and M), `ImportVersionRequest`, `ImportAccepted`, `RecordEvaluationRequest`, `StartShadowRequest`, `PromoteAccepted`, `PinRequest`, `KillStateView`, `EngageKillRequest`, `ReleaseKillAccepted`, `RecordDrillRequest`, `MonitoringView`, `MonitorPointView`, `LadderView`, `ModelScoreView` (`live: Option<LiveScore>`, `LiveScore { score, top_band, reason_codes: [ReasonView; <=3] }`, `ReasonView { code, label: Bilingual }`). The shadow and challenger scores have no schema.
- **AppState additions:** `api`: `models: Arc<ModelRegistry>`, `kill: Arc<ModelKillSwitchService>`, `reads: Arc<ModelReads>`. `worker`: `import_verifier: Arc<ModelImportVerifier>`, `scorer: Arc<ModelScorer>`, `monitor: Arc<ModelMonitor>`, `label_gate: Arc<ModelLabelGateService>`, `runtime: Arc<dyn ModelRuntime>`, `signatures: Arc<dyn SignatureVerifier>`; the change targets `ModelPromotion` and `ModelSwitch` are registered in `ChangeTargetRegistry` in both binaries (the same registry, phase 04 rule).
- **Evidence owner kind:** `model_version` is added to the phase 02 evidence owner registry (the card is staged with `owner_kind = model_version`, `owner_id = version id`, `kind = upload`, `content_type = application/json`).

---

## 3. Business Logic

### 3.1 Families, inventory and versions (`ModelRegistry`; GV-01, GV-03, GV-04, GV-05, T-63)

**`create_family(ctx, CreateModel, idem)`** (route 2; Cedar `model.create`, MRO):
1. Validate: `name` and `purpose` have `ar` and `en`; `starter` requires `recalibration_due` (CHECK); `inventory.tier_basis` has the seven keys of GV-03 (decision impact, coverage, autonomy, complexity, data novelty, regulatory exposure, fairness exposure), each `low`, `medium` or `high` with a reason (422 `E_VALIDATION`, `tier_basis_incomplete`); `owner_id` is an active human user; the validator is not set here (phase 04 CHECK).
2. In one transaction: generate the family id (UUIDv7, app-side), insert `admin.models` with `next_kill_drill_due = today plus model.kill_drill_months months` (bank calendar) and its `admin.model_kill_state` row (`scope = 'model:<id>'`, `engaged = false`), then call phase 04 `InventoryService::create` with `kind = model`, `object_id = models.id`, `next_validation_due` from `review.rules_months`. The row goes first because the phase 04 object resolver for kind `model` (registered by this phase) checks that the object exists. A second item for the same object is refused by `uq_inventory_items_object`.
3. (Folded into step 2.)
4. Audit `model.created {model_id, starter, tier}`. Idempotent by `Idempotency-Key`.

**The version item.** Every version is its own inventory item (`kind = model`, `object_id = model_versions.id`), created in the import transaction of section 3.3 (the version id is generated first; the resolver sees the version row in the same transaction) with the family's tier, `tier_basis`, owner, developer and committee. Tier and owner are copies that the item checks against the family at creation; a later change to the family's tier is a phase 04 inventory update on the family item and is not copied to versions already imported (Open doubts 5).

**Use gate (GV-01).** Every score, shadow, challenger or live, is written only for a version whose inventory item has `status = approved_for_use`. The check is the phase 04 GV-01 gate (`inventory_kind = model`), run in the scorer before each batch and at activation. The inventory status is moved to `approved_for_use` by the phase 04 `PATCH /inventory/{id}` route with step-up `approval` by a validator who is not the owner; this phase does not re-implement that check.

**Starter flag (GV-05, T-63).** A starter family (`starter = true`) is an externally calibrated model shipped in a release bundle. Its card section 7 states "externally calibrated, temporary". The family is due for local recalibration at `recalibration_due`; the phase 24 review calendar lists the date (a row derived from the family, not a new calendar). A starter family may be shadowed and promoted at the recalibration gate of section 3.6 (50 positives), not at the bank-trained gate.

**`import_version` is section 3.3; `start_shadow` and `record_evaluation` are section 3.6; `pin` and the reads are section 3.10.**

### 3.2 The model card (GV-30, GV-54)

The card is a JSON document with exactly ten sections, validated against its schema at import (`manifest_invalid` when a section is missing or empty). It is stored as the evidence object `card_evidence_id` (`owner_kind = model_version`, `kind = upload`, staged through phase 02 `EvidenceStore::stage`; the staged object is the card of record once its scan is clean). Free text only in the sections' own prose fields; no customer, alert or case identifier anywhere in the card.

1. Identity and purpose: family id, version number, owner, intended use, out-of-scope uses, approved population and period.
2. Data: sources and critical data elements by name, training window, label definition (section 3.6, the QA-confirmed rule), synthetic or external data share, exclusions, known gaps.
3. Method: algorithm (`gradient_boosted_trees`), features with catalogue keys and versions, why chosen, the baseline and its metrics (the transparent scorecard, section 3.6).
4. Performance: metrics with confidence intervals, by segment (the phase 24 `segment` groups only), the operating point, the top-band cut and the accepted miss rate.
5. Explainability: global importance, the reason-code logic of section 3.4, stability checks, the TreeSHAP golden test result.
6. Fairness: protected attributes considered (phase 24 register), the proxy scan result, the segment rates (section 3.5).
7. Limitations and operating boundaries, failure modes, fallback (rules only), and the `starter` statement.
8. Monitoring plan: metrics, green, amber and red thresholds, owners (section 3.8).
9. Validation history and open findings (phase 24 register ids), change log.
10. Contacts and approval record (owner, validator, committee; never a person's name in the card body: user ids only).

**View rule (GV-54).** `GET /models/{id}/versions/{vid}` returns sections 1 to 3, 5 to 10 to every role of its scope, and section 4's operating point (`operating_point` block: the threshold values, the top-band cut and the accepted miss rate) only with `?view=operating_point`, which Cedar `model.operating_point.read` permits to MRO and M alone. For IA with the same query the answer is 403 `E_FORBIDDEN` (the scope admits IA; Cedar denies). Raw weights and tree structures are never returned by any route.

### 3.3 Import, verification and load (`ModelRegistry::import_version`, `ModelImportVerifier`, `ModelRuntime`; TS-16)

**Import, synchronous (route 6, akr-api, `model.version.import`, MRO):**
1. Lock order 2: `SELECT ... FROM admin.models WHERE id = $1 FOR UPDATE`. A family with a pending import (a version in `importing`) answers 409 `E_NOT_ALLOWED_IN_STATE` (`import_pending`).
2. Canonicalise `manifest` (RFC 8785 style: sorted keys, no floats in identity fields). `sha256(canonical) == manifest_sha256`, else 422 `E_VALIDATION` `manifest_invalid`.
3. Verify the Ed25519 `signature` over the canonical bytes with `signing_key_version` (section 4.4). A failure is 422 `E_VALIDATION` `signature_invalid`; nothing is inserted; audit `model.version_refused {family, reason: signature_invalid}` (SIEM flag set; section 3.13).
4. The manifest names: `artifact_key`, `artifact_sha256`, `feature_keys`, `monotone`, `opset` (the ONNX domains and versions the author used), `reference_bins`, `golden` (20 rows of feature values and expected outputs, numbers only), `training` (hashes only: `dataset_sha256`, `code_sha256`, `params_sha256`, row counts) and `provenance` (supplier or internal team, licence). A missing name is `manifest_invalid`.
5. Insert `admin.model_versions` with `model_id` (the family of the path), `artifact_key` and `manifest_sha256` (from the request), `created_by` (the caller) and `status = importing`, `version_no` = the family's last + 1, `inventory_item_id` of the new version item (the version item paragraph of section 3.1, created in this transaction), `artifact_sha256`, `manifest`, `signature`, `signing_key_version`, `feature_keys`, `monotone`. The card: `EvidenceStore::stage` (phase 02; `kind = upload`, `owner_kind = model_version`, `owner_id` = the version id) in the same request; the stage's `evidence.staged` event is the phase 02 one.
6. Enqueue job `model_import_verify {version_id}` (coalesce `model_import_verify:<version_id>`) in the same transaction. Audit `model.version_imported {model_id, version_no, artifact_sha256}`. Answer 202 with the version id and `status: importing`.

**Verification (job `model_import_verify`, akr-worker, section 4.1):**
1. Wait for the card's evidence to be `clean` (`av_status`): while it is `pending` the job returns `JobError::Retryable` (`max_attempts` 10 with the phase 00 back-off, about half an hour in all). An `infected` or `rejected` card is refusal `manifest_invalid`. A card still `pending` at the last attempt (`JobContext.attempt`) is refused with `manifest_invalid` too, so a version never stays in `importing` and the family's `import_pending` refusal always ends. Step 8 writes the refusal in each case; the artifact of a refused version is not imported again (`uq_model_artifact`), so the supplier issues a new one.
2. Read the artifact through `ArtifactStore` (section 4.4). `sha256(bytes) == artifact_sha256`, else refusal `sha_mismatch`.
3. `OnnxInspector::operators(bytes)`: every node's `(domain, op_type, version)` must be in the allow-list: `ai.onnx.ml` `TreeEnsembleClassifier` and `TreeEnsembleRegressor`; `ai.onnx` `Sigmoid`, `Identity`, `Cast`, `Concat`. Anything else, including `ZipMap` and custom domains, is refusal `operator_unsupported` (ZipMap is refused because it returns maps; the model must return tensors).
4. Features: each `(key, version)` must be the active catalogue definition in `feat.feature_definitions` (refusal `feature_not_in_catalogue`); each feature's `requires` must not name a screening source (`suspicion_source_feature`, DR-270); `ProtectedAttributeCheck::check_features` (section 3.5) refuses a prohibited attribute (`prohibited_feature`) or a flagged proxy (`proxy_flagged`).
5. Monotone: every key in `monotone` is a feature of the list with sign +1 or -1; the constraint is re-checked by the validation sweep (section 3.6, P5), so a declared constraint that the model does not hold never reaches shadow. A declared key not in the list is `monotone_unsound`.
6. Runtime: load the golden rows with `ort`; compare every output with the expected value within 1e-6 (absolute). On failure, load with `tract` and repeat. The first runtime that passes is recorded in `runtime`; if both fail the refusal is `golden_mismatch`. The result is stored in `opset` (the runtime name and version as well as the ONNX opsets).
7. On success, in one transaction: `status = registered`, `runtime`, `opset`, `card_evidence_id` (the clean copy), `reference_bins` (from the manifest; counts only), audit `model.version_verified`. The version's inventory item, created at import (section 3.1), keeps its phase 04 default status `registered`.
8. On any refusal: `status = refused`, `refusal_code` set, audit `model.version_refused {reason}` (SIEM flag set). A refused artifact stays in the store for the evidence of its refusal and is never loaded.

**Load (every load, not only at import):** `ModelScorer` loads a version when a worker starts, when a promotion activates it, when a switch is released, and at each sweep if the cached session is older than 10 minutes. A load re-reads the artifact, re-checks `sha256` and the Ed25519 signature from the stored manifest and the stored `signing_key_version`, and runs the golden rows again. A failure is audit `model.load_refused` (SIEM flag set); the version is not used, the sweep scores nothing with it, and the queue ranks by rule score (section 3.4). The version's status is not changed by a load failure: the failure shows through the metric `scored_count` of section 3.8, and the bank decides.

**Runtime (DR-261).** `ort` is pinned to one exact release-candidate version recorded in the bundle and vendored with `cargo vendor`; the ONNX Runtime shared library is built from a pinned source tag and its SHA-256 is in the bundle manifest. `tract` is the fallback for a single version whose golden rows fail under `ort`. The fallback is recorded per version and is never mixed within a version. Both are linked into `akr-worker` only.

### 3.4 Scoring and the queue (`ModelScorer`, `ExplanationService`; T-13, M-13, GV-41)

**Ranking rule (the whole of the phase).** The score orders alerts inside their priority class. It never changes `priority_class` (phase 17 `PriorityClassifier`, which never reads a score), never suppresses, closes, hides or creates an alert, and never moves an alert to another queue. `alert.alerts.score` (the rule score of phase 17) is not replaced and not changed; the model score is a separate row (`alert.model_scores`, DR-257).

**Sweep (job `model_score_sweep`, every minute, akr-worker):**
1. Read the kill state (section 3.9): if `global` is engaged, stop. Load the live version (section 3.3 load). If the family's `model:<id>` switch is engaged, stop for that family.
2. Batch (`model.sweep_batch`, scored in chunks of 50 alerts; step 1 is applied again before each chunk, so an engaged scope stops the batch): `SELECT a.id, a.created_at FROM alert.alerts a WHERE a.state IN ('queued', 'claimed') AND a.created_at >= <live_from> AND NOT EXISTS (SELECT 1 FROM alert.model_scores s WHERE s.alert_id = a.id AND s.alert_created_at = a.created_at AND s.model_version_id = <version>) ORDER BY a.created_at, a.id LIMIT <batch>`. The `live_from` bound is the rule of drained alerts: alerts created before the champion went live are never scored by it (phase 17 section 3.4, "drained, not migrated", M-75). Shadow and challenger versions are scored on the same batch of alerts created after their `shadow_started_at`.
3. Features: for each alert read the stored values of the family's `feature_keys` from `feat.feature_values` by `entity = 'customer'` and `entity_id` = the subject customer (`canon.customer.id` of the alert's party, phase 18 rule), `feature_id` of the manifest's `(key, version)`, the currency the manifest names per feature (`XXX` for currency-free kinds, phase 13), and `as_of` = the latest closed month-end on or before the alert's `created_at` (no look-ahead); a row whose `status` is not `value` is passed as missing. Never from the CBS: the scorer has no source connection (phase 13 rule). A missing value is passed as missing (the tree's default branch); no per-row count is stored: `shap` holds only the contributions of step 5, and the batch total of values passed as missing is the `skipped_missing` count of step 6.
4. `runtime.predict` gives `score` in [0, 1]; `top_band = score >= top_band_cut`.
5. Live rows only: `ExplanationService::explain(session, row)` returns the TreeSHAP contributions. The reason codes are the three features with the largest positive contribution (`feature_key@version`), at most three (`reason_codes` CHECK); `shap` stores the five largest absolute contributions as `{key, version, contribution}` (no tree structure, no raw weights). Shadow and challenger rows store `shap = []` and `reason_codes = {}` (computing them is not needed and the values are never shown).
6. Insert `alert.model_scores` in one transaction per chunk of step 2, `ON CONFLICT DO NOTHING`; one audit `model.scored {version_id, role, rows, skipped_missing}` per batch, written with the last chunk that ran (counts of the committed chunks only, no ids). A batch that an engaged scope stops keeps its committed chunks and scores no further chunk.

**Queue order (phase 17 section 3.9, extended).** The phase 17 items query `ORDER BY priority_class, score DESC NULLS LAST, created_at, id` becomes `ORDER BY priority_class, model_rank DESC NULLS LAST, score DESC NULLS LAST, created_at, id`, where `model_rank` is `ms.score` from a `LEFT JOIN LATERAL (SELECT ms.score FROM alert.model_scores ms WHERE ms.alert_id = a.id AND ms.alert_created_at = a.created_at AND ms.role = 'live' ORDER BY ms.scored_at DESC LIMIT 1) ms ON TRUE`, with the kill condition `NOT EXISTS (SELECT 1 FROM admin.model_kill_state k WHERE k.engaged AND k.scope IN ('global', 'model:' || $family))` in the join's `ON`, so no live row is joined while the scope is engaged. `$family` is the ranking family (one live family at a time, section 1.1). When the switch is engaged the join returns nothing and the order is the rule order of phase 17 (DR-266). Unscored alerts sort after scored ones in their class for the sweep lag (at most one minute; Open doubts 7). The row carries `model_band` (`top`, `other`, or absent) for A1, A2 and M only, through the same visibility predicate as the row (section 3.14).

**Single read (route 16, `GET /alerts/{id}/model-score`, `alert.model_score.read`, A1, A2, M; the same purpose and step-up rules as phase 17 `GET /alerts/{id}`: rule 4 for restricted, rule 5 for suspicion):**
1. Resolve the alert through `services::visibility::build(principal, network)` (phase 17 section 3.10). A row the caller may not see answers the masked 404 `E_NOT_FOUND`, whatever header is missing: a caller outside the team never gets 401 or 422 (phase 03 section 7.14, DR-411). A caller the rule admits to a suspicion-tier alert, with no fresh `record_open` step-up, answers 401 `E_STEPUP_REQUIRED`. A missing id gives the body and the latency floor of the masked 404.
2. Read the latest `alert.model_scores` row of the alert with `role = 'live'` (any live version: the champion at scoring time, section 3.7 step 4). The response has `live` (or `null` when unscored, when the switch is engaged, or when the alert predates `live_from`), `top_band` and up to three `reason_codes`, each with its bilingual label from `feat.feature_definitions.name`. Shadow and challenger rows are never selected (the query has no role parameter).
3. Access event `view` (phase 02 `access_event`) with the alert's resource kind; no audit kind is added for a read.

**Explanation (`ExplanationService`, TreeSHAP in house; golden tests).** Interventional TreeSHAP on the tree ensemble of the artifact, computed from the node structure read by `OnnxInspector` at load (not stored). The golden test of the card (section 3.6, P3) compares its output with a reference computed offline on the same golden rows; a mismatch above 1e-4 refuses the version at import (`golden_mismatch`). Fidelity: the sum of contributions plus the base value equals the score logit within 1e-6 for every golden row.

### 3.5 Features, protected attributes and fairness (`ProtectedAttributeCheck`, `FairnessService`; GV-36, GV-37, GV-38, GV-40)

**Feature check (at import, step 4 of section 3.3; reused, not rebuilt).** `ProtectedAttributeCheck::check_features(features)` is the phase 24 pure function of section 3.5 there, given a second input shape (a list of catalogue features instead of a condition tree). For each feature, in order:
1. The feature's `requires` (phase 13) names its source columns. A source column that is an `admin.protected_attributes` row with `prohibited_as_input = TRUE` (name-derived, sect, tribe or ethnicity) refuses with `prohibited_feature`.
2. A source column that is a protected attribute whose `allowed_uses` does not contain `score` refuses with `prohibited_feature` (the allowed-use matrix of GV-36 is the register; the model's use is `score`).
3. A source column that the phase 24 proxy rule names as a proxy of a protected attribute (the proxy list of section 3.5 there) refuses with `proxy_flagged`.
4. A feature whose `requires` names a screening source (`screening`, `list_match`, `pair_memory`) refuses with `suspicion_source_feature`: a score on a normal-tier alert must not reveal a screening hit (phase 17 tier rule 2; DR-270).
5. Nationality, residence and geography are accepted only beside other features (GV-36: never the sole trigger of a score). The sole-trigger property is tested, not assumed: the sensitivity sweep of section 3.6 (item 4) moves each such feature alone across its range with the others at their medians and fails the family if any golden row changes its top-band flag.

**Fairness by model segment (monthly job, extended).** The phase 24 job `fairness_monthly` (section 3.5 there, step 2 and the metric table) gains one metric value, `score_top_band_rate` (the CHECK of section 1.1): for each group of `segment` (phase 13 `feat.segments`, the only group kind a model is measured by; the closed `group_kind` CHECK of phase 24 is unchanged), the number of top-band alerts of the group's customers per 1,000 customers, the reference being the same rate over all customers, `ratio` and `suppressed` as phase 24 defines them. Live-role scores only, from `alert.model_scores`; alerts of every tier count, as phase 24 counts its other rates, and the cell is suppressed below the threshold (section 3.14). A ratio at or beyond `fairness.review_ratio` opens the phase 24 review (a finding owned by the attribute's owner, or a warning of source `fairness`). Nothing in this phase feeds a score back.

**Purpose binding (GV-40).** A score is an AML/CFT output: `alert.model_score.read` takes the alert's own purpose rules (phase 17 rules 4 and 5), and the monitoring and ladder reads are counts with no purpose-free export. No route, file or audit export returns a score row: phase 25's SIEM export and support bundle must exclude `alert.model_scores` by table name (a requirement on phase 25, Open doubts 15).

### 3.6 Evaluations, label gate, shadow summary and the validation pack (`ModelRegistry::record_evaluation`, `ModelLabelGateService`, `ValidationPackService` model branch; GV-09, GV-14, GV-16, GV-24, T-15, T-14)

**Labels (GV-39, T-13).** Only QA-confirmed outcomes are labels:
- a positive is an alert of a case whose `casework.decisions.type` is `str`, `utr` or `tfr`, and whose case has a `casework.qa_samples` row with a `casework.qa_reviews.overall` of `pass` or `pass_with_comment`;
- a negative is an alert of a case closed with `no_report` and reviewed the same way. Low-scored alerts enter this set through the phase 19 daily forced sample (`low_score_forced`, `qa.forced_low_score_daily`), so the negatives do not depend on what the model flagged. A model never suppresses an alert, so no label is produced by its own suppression.
- Outcomes not yet QA-confirmed do not count. Every decided case counts, whatever its tier: phase 18 raises the subject to `suspicion` on every `utr`, `str` and `tfr` decision (phase 18 section 3, step 8), so excluding suspicion tier would remove almost every positive. The job reads the rows; only window totals leave it, suppressed below `report.small_number_threshold` (section 3.14).

**Record an evaluation (route 8, `model.validation.record`, MRO).** Kinds accepted: `benchmark`, `out_of_time`, `calibration` and `sensitivity`. `label_gate` and `shadow_summary` are written by the jobs and the promotion check, never by a person. `passed` is computed by a pure function of `result` (a person never sets it):
1. `benchmark`: the baseline is the transparent monotone scorecard that the bank's workbench trains on the same features (the card's section 3 names it); `result` holds the baseline and the model metric (the same metric and operating point, e.g. precision at the top-band cut), each with a 95 per cent interval, and the difference with its interval. `passed` iff the lower bound of the difference is at least 0 on the out-of-time window (GV-14: no go-live if the model does not beat the baseline; no waiver is built, Open doubts 4).
2. `out_of_time`: `result` holds `window_from`, `window_to`, `label_lag_days`, the AUC and PR-AUC with intervals and the positives count. `passed` iff the window ended at least `label_lag_days` before `data_as_of` (labels are mature) and the positives count reaches the gate of the family kind (GV-16).
3. `calibration`: the calibration slope and the ratio of expected to observed positives by score decile. `passed` iff the slope is within 0.8 and 1.2 and the ratio within 0.8 and 1.25 (design values, Open doubts 2).
4. `sensitivity`: the monotone sweep of section 3.3 step 5 (each constrained feature moved across its catalogue range with the others at their medians, 200 grid points) and the top-band share under a 10 per cent perturbation of each feature. `passed` iff the violations of the declared monotone directions are zero. A family without monotone keys reports `not_run` (passed false) and is allowed to pass only in the validation pack's P5 as `not_applicable` (the validation pack paragraph of section 3.6).
5. `data_as_of` must be within `validation.evidence_max_days` of now (phase 24 setting), else 422 `E_EVIDENCE_STALE`. Audit `model.evaluation_recorded {kind, passed}`.
6. The validation pack is phase 24's `POST /inventory/{id}/validation-packs` (section 3.6 there) with its model branch: the same seven part keys P1 to P7 carry: P1 the card sections 1 to 3 hashes, the feature list with catalogue versions and the declared monotone keys; P2 the open dq issues and drift events of the feature sources (phase 10, phase 12) and the latest feature-PSI band; P3 the golden-row and fidelity results of import; P4 the shadow summary and the label counts (aggregates); P5 the sensitivity evaluation (`not_applicable` for a family without monotone keys, `fail` for a family with monotone keys whose sweep is missing); P6 the QA reviewer rates and label counts of section 3.6 labels (phase 19, no new sample); P7 the benchmark and out-of-time evaluations. Verdict as phase 24: `fail` if P3, P5 or P7 fails, `pass_with_findings` if an open register finding names the item, else `pass`. A pack is issued only by the validator, never the owner (phase 24, 403 `E_SELF_APPROVAL`).

**Label gate (job `model_label_gate`, weekly; T-15).** Per family, over `model.label_window_days`: count QA-confirmed positives and negatives; the gates are `model.gate_shadow_positives` (200) and `model.gate_live_positives` (500) for a bank-trained family, and `model.gate_recalibrate_positives` (50) for both gates of a `starter` family (the starter is externally calibrated and needs local labels to recalibrate, not to be trained). Write one evaluation `label_gate` per version with `result = {positives, negatives, shadow_gate: met|not_met, live_gate: met|not_met, window_from, window_to}`; `passed` = the live gate is met. Counts below `report.small_number_threshold` are returned as `suppressed` in every read (phase 21 setting). The model-ladder read (route 4) shows the gate of each family and the next gate that is not met.

**Start a shadow or challenger run (route 9, `ModelRegistry::start_shadow`, `model.shadow.start`, MRO; `Idempotency-Key`; `If-Match` on the version):**
1. Lock the family, then the version (lock order 2 and 3). The version is `registered` or `retired` (a rollback, section 3.7 step 6), else 409 `E_NOT_ALLOWED_IN_STATE` `not_registered`; its inventory item is `approved_for_use` (`inventory_not_approved`); no other version of the family holds the requested role (`role_taken`; the partial unique indexes of section 1.1 are the backstop); role `challenger` needs a live champion in the family (`no_champion`).
2. Gate: the latest `label_gate` evaluation of the version meets the shadow gate (`model.gate_shadow_positives`, or `model.gate_recalibrate_positives` for a starter family), else `gate_not_met`.
3. Criteria: exactly the closed keys of the shadow summary below, each at or beyond its floor (`min_days >= model.shadow_min_days`, `min_scored_alerts >= 1000`, `min_rank_correlation_with_live >= 0.5`, `top_band_share_change_max <= 0.05`), else 422 `E_VALIDATION` `criteria_invalid`.
4. One update: `status` = the role, `shadow_started_at = now()`, `shadow_criteria` (frozen; on a rollback the guard admits new criteria only on this edge), `retired_at = NULL` on a rollback, and a provisional `top_band_cut` = the `model.top_band_percent` percentile of `reference_bins.score` (the import manifest), so `top_band` is defined for every shadow row. Audit `model.shadow_started {role, min_days}`.

**Shadow summary (computed at the promotion decision, `check_before_decision` of section 3.7; GV-24).** `shadow_criteria` is frozen at start (route 9) as a closed set of keys with their required values: `min_days` (at least `model.shadow_min_days`), `min_scored_alerts` (at least 1,000; a design constant of this phase), `min_rank_correlation_with_live` (at least 0.5 on the same alerts, Spearman; design constant), `top_band_share_change_max` (absolute change of the top-band share against the live champion, at most 0.05; design constant). The summary computes each observed value from `alert.model_scores` for the shadow role and the live role on the same alerts created after `shadow_started_at`, and `alert_volume_change`, which is zero by construction (the ranking never suppresses) and is checked, not assumed. Result `{days, scored, criteria: {key: {required, observed, passed}}}`; `passed` iff all criteria pass. The summary is an evaluation row written at decision time, so it is never stale; an evaluation older than the decision is not reused.

### 3.7 Promotion to live (`ModelPromotion`, change target `model_version`; GV-24, GV-26, GV-32)

`ModelPromotion` implements phase 04 `ChangeTarget` (section 3.3 there) with: `kind() = model_version`; `reason_families() = ["change"]`; `generic_create() = false` (the only entry is `POST /models/{id}/versions/{vid}/promote`); `emergency_allowed() = false`; `inventory_kind() = Some(model)` (the GV-01 gate); `revert` is not implemented (rollback below). The route map entry of phase 04 selects `cr_material_validated` when `Prepared.validated_route` is true and `cr_material` otherwise; this target sets it true only when the version's item is `approved_for_use`, so an unvalidated candidate can never reach the validated route.

1. **`prepare` (route 10, the change request is created and submitted in the request; `Idempotency-Key`; `If-Match` on the version):** the version is `shadow` or `challenger` (else 409 `E_NOT_ALLOWED_IN_STATE` `not_shadow`); its item is `approved_for_use` (else `inventory_not_approved`); the family is not pinned to another version (`pinned`); no kill switch is engaged for the family or globally (`kill_engaged`); no other family holds a live version (`other_family_live`, the one-live-family index of section 1.1; Open doubts 22); `object_key = model:<family id>`; `base_version = live version_no or 0`; class `material` (fixed: a new model is a new object); `volume_effect = neutral` (ranking never changes volume; section 3.6 checks it); `required_evidence` = `shadow_summary`, `out_of_time`, `benchmark`, `label_gate` and the validation pack, named as phase 04 evidence refs.
2. **`on_submitted`:** set `promotion_requested_at` on the version (one `UPDATE` in the request transaction under the phase 04 instance lock).
3. **`check_before_decision` (run by phase 04 at each approval step and again at activation):** write a fresh `shadow_summary` (section 3.6); refresh `label_gate` if the latest one is older than seven days; require the latest `benchmark` and `out_of_time` evaluations passed; require a validation pack for the version item with verdict `pass` or `pass_with_findings`, `issued_at >= shadow_started_at` and within `validation.evidence_max_days`, whose `sha256` matches its evidence object (else `pack_missing` or `pack_stale`, the E_NOT_ALLOWED_IN_STATE sub-codes of section 3.11); `shadow_summary.passed`; `days >= model.shadow_min_days`; the live gate met for the family kind. Each failure is a `ValidationIssue` with its sub-code. A failed check blocks approval; it does not close the request.
4. **`activate` (phase 04 effect `activate_change`, under the instance lock):** (a) lock the family and the candidate (lock order of section 3.16); (b) run the load check of section 3.3 on the candidate: a failure is `load_refused` and the approval step fails (phase 04 closes the request as `failed`); (c) `SET LOCAL akr.model_promotion = 'on'`; (d) the previous live version becomes `retired` with `retired_at = now()`; (e) the candidate becomes `live` with `live_from = now()`, and `top_band_cut` is the score at the top-band percentile (`model.top_band_percent`) of its shadow rows; (f) `admin.models.live_version_id` is the candidate; (g) audit `model.promoted {family, from_version_no, to_version_no}`. The previous champion's id is in the change request's `proposed` body, which is the rollback point. Scores already stored keep their role and version (section 3.4 reads the latest live row of each alert).
5. **`on_closed`:** a rejected, withdrawn or failed request changes nothing; a new request is allowed.
6. **Rollback:** no revert. A retired champion re-enters through `retired -> shadow` (route 9) with a new frozen criteria set, its own shadow period and a new validation pack; the artifact is the same, so its checks pass or fail again on current data. This keeps the guard's edge list (section 1.3) the only path.
7. **Two people (GV-32):** the maker of the request is recorded by phase 04; an approver who is the maker is refused with `E_SELF_APPROVAL` (403) by phase 04. The MRO approval of step `cr_material_validated` is by a second MRO; then CHK and M in parallel (phase 04 `cr_material`).

**The release path never sets live (GV-29).** A model that a phase 25 release carries is imported by people through section 3.3 with status `registered` and nothing else (DR-390). The first install of a starter family is promoted by the same change request, not by the release (Open doubts 8).

### 3.8 Monitoring: drift, overrides and breaches (`ModelMonitor`; GV-18, GV-19, GV-21)

Job `model_monitor` (daily, 05:00 bank time, section 4.1) computes the day `D` (the previous bank day) for every family's versions in status `shadow`, `challenger` and `live`, and for a `retired` version within 30 days of its retirement. Each metric has a population of the last 30 days ending `D`. Output rows are `admin.model_monitor_points`, written once (`ON CONFLICT DO NOTHING`).

| Metric | Definition | Green | Amber | Red |
|-|-|-|-|-|
| `score_psi` | population stability index of the version's stored scores (role of the version) against `reference_bins.score` (ten bins, from the import manifest); bins floored at 0.0001 | below `model.psi_watch` | from `model.psi_watch` to below `model.psi_action` | `model.psi_action` or above |
| `feature_psi` (per feature key) | the same index of the feature's stored values of the scored alerts' customers against the feature's reference bins | as above | as above | as above |
| `override_rate` (live only) | overrides over decisions made in the window on cases with live-scored alerts (definition below) | below half of `model.override_rate_action` | from half to below `model.override_rate_action` | `model.override_rate_action` or above (GV-21 fitness review) |
| `scored_count` | alerts the version scored in its role over alerts that entered the queue in the window; a live version counts alerts created on or after its `live_from`, a shadow or challenger version those created on or after its `shadow_started_at` | at least 0.95 | from 0.80 to below 0.95 | below 0.80, or zero scored while alerts entered (a load refusal or a sweep stop) |

- **Override (GV-21, from phase 18 records):** a decision is an override when (a) its `type` is `no_report` and at least one alert of its case is `top_band`, or (b) its `type` is `str`, `utr` or `tfr` and no alert of its case is `top_band`. Only `casework.decisions.type` and the alert flags are read; reason codes and text are not. The rate is overrides over decisions of the window with live-scored alerts. Decisions of every tier count (a report decision always raises the subject to `suspicion`, phase 18), as phase 24 counts `str_rate`; only the window rate leaves the job, suppressed below the threshold (section 3.14).
- **Suppression:** a population below `report.small_number_threshold` (phase 21, never below 3) writes `band = suppressed` and `value = NULL`; zero stays zero. Days when the global or the family switch was engaged write `band = not_run` for `scored_count`.
- **Red breach (GV-19):** a red point opens one register finding in phase 24's `admin.registers`: `source = model_monitor`, `source_key = model_monitor:<version>:<metric>:<feature or empty>:<D>` (unique, so a rerun inserts nothing), `kind = finding`, `severity = high`, `owner_id` = the family owner, `due_on` = today plus `register.finding_due_days`, the fixed bilingual `title` {"ar": "تجاوز الحد الأحمر في مراقبة النموذج", "en": "Model monitor: red breach"}, and a `body` with the metric, value, band, threshold and the five permitted decisions. The job calls phase 24's `RegisterRepository::find_by_source_key` first and `insert` only when no row has the key. The point's `register_id` is set in the same transaction. Audit `model.monitor_breach {metric, band}`.
- **Forced decision:** a `model_monitor` finding closes only with one of `RETRAINED`, `RECALIBRATED`, `RESTRICTED_USE`, `RULES_ONLY` or `JUSTIFIED_NO_ACTION` (a closure code of the phase 24 pattern). `RULES_ONLY` is recorded beside a kill-switch engagement in the same review; it does not engage the switch itself (the engagement is section 3.9, step 1). The phase 24 register update refuses any other closure code for this source with 422 `E_VALIDATION` `closure_code_not_allowed`.
- **Feature sole trigger:** the check of section 3.5 (geography, residence and nationality) is a sensitivity evaluation (section 3.6 item 4), not a monitor metric; its result is in the pack's P5.

### 3.9 Kill switch (`ModelKillSwitchService`, change target `model_switch`; GV-20)

Scopes: `global` (every family) and `model:<family id>`. The state is one row per scope in `admin.model_kill_state`.

1. **Engage (route 12; Cedar `model.kill.engage`; scope `model.risk`, MRO):** `Idempotency-Key`; `If-Match` on the scope's row (428 without it, 409 on a version mismatch); step-up `StepUpService::consume(tx, ctx, header, ModelKillSwitch, "model_kill:<scope>")` where `ModelKillSwitch` is the constant `{name: "model.kill_switch", class: approval}` (phase 01 closed classes; no new class; missing or reused assertion is 401 `E_STEPUP_REQUIRED`). Lock the scope row (created with the family, section 3.1; an unknown scope is 404 `E_NOT_FOUND`), set `engaged = true`, `reason_code`, `engaged_by`, `engaged_at`; audit `model.kill_engaged` (SIEM flag). No change request is opened: falling back to rules is the safe direction (the phase 16 scenario kill switch takes the same view, DR-108). The fallback is immediate: the sweep reads the state before each chunk of 50 alerts, and the queue's rank join excludes every live row while the scope is engaged (section 3.4), so the queue returns to rule order at the next request (the 30-second live cache of section 2 is not read for the kill state).
2. **Owner rule:** the GV-20 owner is the family owner; the route's scope is `model.risk`, which only MRO holds, so an owner who is not an MRO cannot press the switch (Open doubts 1).
3. **Release request (route 13; Cedar `model.kill.release_request`):** the scope must be engaged (409 `not_engaged`); it opens change request `model_switch` with `object_key = kill:<scope>`, class `material`, route `cr_material` (phase 04 map: not validated, so the parallel M (the MLRO desk) and CHK approval), `volume_effect = neutral`. `check_before_decision`: when the reason is `red_breach`, the family's `model_monitor` findings must be closed (else `breach_open`): the check reads phase 24's `RegisterRepository::list` with source `model_monitor` and a status other than `closed`, and keeps the rows whose `source_key` names a version of the family. `activate`: `engaged = false`, audit `model.kill_released` (SIEM flag). A released switch does not re-score old alerts: scores are written from the release onward.
4. **Drill (route 14; `model.kill.drill`, MRO):** a drill is run in the bank's test environment, not in production scoring: engage, check the queue returns to rule order for the test set, release. The route records `environment`, `engaged_at`, `released_at`, `rules_only_checked` and writes an evaluation of kind `kill_drill` (no version) and sets `next_kill_drill_due = today + model.kill_drill_months`. The phase 24 review calendar lists `next_kill_drill_due` as a derived row (kind `model_kill_drill`, amber 30 days before, red on the due date): a row of the existing calendar, not a second calendar (section 4.5).

### 3.10 Pinning, notices and reads (`ModelRegistry::pin`, `ModelReads`; GV-29, GV-30, T-15)

- **Pin (route 11; `model.pin`, MRO, step-up `approval` with the constant `ModelPin` = `{name: "model.pin", class: approval}` and the resource ref `model_pin:<family id>`; 401 `E_STEPUP_REQUIRED` without it):** sets `admin.models.pinned_version_id` to a version of the family or null; `If-Match` on the family. While a version is pinned, every promotion of another version is refused with `pinned`. Pinning the live version keeps it live against an advanced candidate; a pinned candidate is not promoted until unpinned. Audit `model.pinned {family, version or null}`.
- **Notice (GV-29):** `GET /models` returns `pending_notices`: versions in `registered` or `refused` that have not been shadowed or promoted, newest first, with the import time. No e-mail or in-app notice is sent in this release (Open doubts 9, closed in section 7+); the audit event `model.version_imported` is the record of the notice.
- **Reads:** `GET /models` and `GET /models/{id}` (families, versions, gates, pins, next drill and notices); `GET /models/{id}/versions/{vid}` (card, section 3.2 view rule); `GET /models/{id}/monitoring` (stored points of the last 30 days, suppressed rows with no value, no ids); `GET /models/kill-switch` (the rows of `model_kill_state` with reason and time); `GET /models/ladder` (per family: `enabled` (a live version exists), `tier` (read from the inventory item), `label_counts` (positives and negatives of the window, suppressed below the threshold), `next_gate` (the first gate not met, with its number; the model ladder of research 21 4d). No read takes a case, alert or party id. Reads write no audit event and the monitoring and ladder reads write one access event each (`view`, resource kind `model`).

### 3.11 Errors (`GovernanceModelsError`, `thiserror`; API section 6 codes only)

| Code (HTTP) | Sub-code (`errors[].code` or the body's `reason`) | Raised by |
|-|-|-|
| `E_VALIDATION` (422) | `manifest_invalid`, `signature_invalid`, `tier_basis_incomplete`, `kind_not_recordable`, `criteria_invalid`, `closure_code_not_allowed`, `name_not_bilingual` (a `name` or `purpose` without `ar` or `en`), `recalibration_due_required` (a starter without the date); the owner check is phase 04's own 422 | sections 3.1, 3.3, 3.6, 3.8, 3.10 |
| `E_NOT_ALLOWED_IN_STATE` (409) | `import_pending`, `inventory_not_approved`, `not_shadow`, `pack_missing`, `pack_stale`, `gate_not_met`, `shadow_too_short`, `shadow_criteria_not_met`, `benchmark_not_passed`, `pinned`, `load_refused`, `kill_engaged`, `not_engaged`, `breach_open`, `not_registered`, `role_taken`, `no_champion`, `other_family_live`, `not_pinnable` | sections 3.3, 3.6, 3.7, 3.9, 3.10 |
| `E_EVIDENCE_STALE` (422) | the part names in `errors[]` | evaluations (3.6), pack freshness (3.7) |
| `E_PURPOSE_REQUIRED` (422) | none | `GET /alerts/{id}/model-score` on a restricted alert without the purpose rule 4 of phase 17 requires (on a suspicion-tier alert a missing purpose is the masked 404 of section 3.4, as for any caller rule 5 does not admit) |
| `E_NOT_FOUND` (404) | none (identical body for missing and masked) | route 16 for suspicion-tier or invisible alerts |
| `E_FORBIDDEN` (403) | none | Cedar denies (for example IA with `?view=operating_point`) |
| `E_SCOPE` (403) | none | the route scope does not admit the caller's roles |
| `E_SELF_APPROVAL` (403) | none | a maker approves its own change; an owner issues a pack |
| `E_STEPUP_REQUIRED` (401) | none | kill engage, pin, route 16 for a team member of a suspicion-tier alert without step-up |
| `E_PRECONDITION_REQUIRED` (428), `E_VERSION_CONFLICT` (409) | none | `If-Match` on promote, pin, shadow, engage |
| `E_IDEMPOTENCY_REQUIRED` (400), `E_IDEMPOTENCY_REUSE` (422), `E_IDEMPOTENCY_IN_FLIGHT` (409) | none | every mutation of section 2 |

`E_UNAVAILABLE` (503) is returned when the artifact volume or the key service is down at import, so the request is retried and nothing is written. A refusal of a load or an import is not an HTTP error for the worker: it is the `refusal_code` or the audit event of section 3.13.

### 3.12 Cedar: actions, roles and the file `C1_models.cedar` (phase 03 range; `C1` is free)

**Schema additions** (appended to `schema.cedarschema`; no existing entity or attribute changes; `Alert`, `System` and `Ctx` exist in phases 03 and 17): one entity, `ModelFamily = { tier: Long, starter: Bool }` (the tier is the inventory item's, read at request time by the adapter). Actions on `ModelFamily`: `model.read`, `model.operating_point.read`, `model.version.import`, `model.validation.record`, `model.shadow.start`, `model.promote.request`, `model.pin`, `model.kill.drill`. Actions on `System`: `model.create`, `model.monitor.read`, `model.kill.engage`, `model.kill.release_request`. Action on `Alert`: `alert.model_score.read`. All take `principal: User` and `context: Ctx`.

**File `C1_models.cedar`** (the governance prefix `C`: `C0_governance`, `C0_packs`, `C0_policy` and `C0_reference` are taken by phases 24, 06, 09 and 09; `C1` is free, grep of docs/plan confirms; no other file is touched). One role per `@role` and per policy; `principal.roles.contains` (no `containsAny`); step-up is a handler check (`StepUpService::consume`), not a condition; the kill switch and the pin need the step-up class `approval` in their handlers.

```cedar
// C1_models.cedar (phase 27). Roles are principal.roles (phase 03 form). Default deny: no policy below names a role
// that is not listed in its route's scopes (section 2 table). Step-up is a handler check.

// Read of a family, its versions and the card (section 3.2 view rule applies in the handler).
@id("model.read.mro") @role("MRO")
permit (principal, action in [Akr::Action::"model.read", Akr::Action::"model.operating_point.read"], resource is Akr::ModelFamily)
when { principal.roles.contains("MRO") };

@id("model.read.m") @role("M")
permit (principal, action in [Akr::Action::"model.read", Akr::Action::"model.operating_point.read"], resource is Akr::ModelFamily)
when { principal.roles.contains("M") };

@id("model.read.ia") @role("IA")
permit (principal, action == Akr::Action::"model.read", resource is Akr::ModelFamily)
when { principal.roles.contains("IA") };

@id("model.read.ch") @role("CH")
permit (principal, action == Akr::Action::"model.read", resource is Akr::ModelFamily)
when { principal.roles.contains("CH") };

// Authoring: MRO only (validator and proposer). The owner of a pack is refused in the handler (E_SELF_APPROVAL).
@id("model.author.mro") @role("MRO")
permit (principal, action in [Akr::Action::"model.version.import", Akr::Action::"model.validation.record",
        Akr::Action::"model.shadow.start", Akr::Action::"model.promote.request", Akr::Action::"model.pin",
        Akr::Action::"model.kill.drill"], resource is Akr::ModelFamily)
when { principal.roles.contains("MRO") };

@id("model.system.mro") @role("MRO")
permit (principal, action in [Akr::Action::"model.create", Akr::Action::"model.kill.engage",
        Akr::Action::"model.kill.release_request"], resource is Akr::System)
when { principal.roles.contains("MRO") };

// Monitoring, ladder and kill-switch reads: counts and states only (section 3.14).
@id("model.monitor.read.mro") @role("MRO")
permit (principal, action == Akr::Action::"model.monitor.read", resource is Akr::System)
when { principal.roles.contains("MRO") };

@id("model.monitor.read.ia") @role("IA")
permit (principal, action == Akr::Action::"model.monitor.read", resource is Akr::System)
when { principal.roles.contains("IA") };

@id("model.monitor.read.m") @role("M")
permit (principal, action == Akr::Action::"model.monitor.read", resource is Akr::System)
when { principal.roles.contains("M") };

@id("model.monitor.read.ch") @role("CH")
permit (principal, action == Akr::Action::"model.monitor.read", resource is Akr::System)
when { principal.roles.contains("CH") };

// Score of one alert: the phase 17 rule 1 roles only. Visibility (services::visibility::build) is applied in the handler.
@id("model.score.a1") @role("A1")
permit (principal, action == Akr::Action::"alert.model_score.read", resource is Akr::Alert)
when { principal.roles.contains("A1") };

@id("model.score.a2") @role("A2")
permit (principal, action == Akr::Action::"alert.model_score.read", resource is Akr::Alert)
when { principal.roles.contains("A2") };

@id("model.score.m") @role("M")
permit (principal, action == Akr::Action::"alert.model_score.read", resource is Akr::Alert)
when { principal.roles.contains("M") };
```

Scope check against the permits: `GET /models*` admits MRO (`model.risk`), IA (`audit.read`, `dashboard.read`), M and CH (`dashboard.read`); every one of those four has a permit for the action of its route. `POST` routes of section 2 admit MRO only through `model.risk`. `GET /alerts/{id}/model-score` admits A1, A2 and M through `queue.work`, the same three roles as the alert's own permit.

**Bundle tests** (`tests/policy/c1_models`): MRO reads, imports and pins; IA and CH read the card and the monitoring but not the operating point; M reads the operating point; A1 reads a score on a visible alert and is refused on the operating point (`E_SCOPE`: A1 has no `model.risk`); CH is refused `model.kill.engage` (no permit, no scope); A1 and A2 are refused `model.monitor.read`.

### 3.13 Audit events emitted (one per mutation; payloads hold ids, names and counts only)

| Kind | Resource | SIEM flag | Payload |
|-|-|-|-|
| `model.created` | ModelFamily | false | `model_id`, `starter`, `tier` |
| `model.version_imported` | ModelVersion | false | `model_id`, `version_no`, `artifact_sha256` |
| `model.version_verified` | ModelVersion | false | `runtime`, `opset` |
| `model.version_refused` | ModelVersion | true | `reason` (refusal code) |
| `model.load_refused` | ModelVersion | true | `reason` |
| `model.scored` | ModelVersion (batch) | false | `role`, `rows`, `skipped_missing`; no alert id |
| `model.evaluation_recorded` | ModelFamily | false | `kind`, `passed` |
| `model.shadow_started` | ModelVersion | false | `role`, `min_days` |
| `model.promotion_requested` | ModelVersion | false | `change_request_id` |
| `model.promoted` | ModelFamily | false | `from_version_no`, `to_version_no` |
| `model.monitor_breach` | ModelVersion | false | `metric`, `band`, `register_id` |
| `model.pinned` | ModelFamily | false | `version_no` or null |
| `model.kill_engaged` | System (scope) | true | `scope`, `reason_code` |
| `model.kill_release_requested` | System (scope) | false | `change_request_id` |
| `model.kill_released` | System (scope) | true | `scope` |
| `model.kill_drill_recorded` | ModelFamily | false | `environment`, `rules_only_checked` |

The kinds are added to the phase 02 catalogue with their SIEM flags (phase 02 section 3 constants). Phase 25's SIEM export carries the flagged kinds only. A read writes an access event (phase 02 `access_event`, action `view`), never an audit row.

### 3.14 Tipping-off review (what a person outside a case's access list can learn; the answer must be "nothing")

| Surface | Who sees it | What they can learn about a case or party |
|-|-|-|
| Score, top-band flag and reason codes of one alert (route 16) | A1, A2, M, under the alert's visibility and the alert's own purpose rules; a suspicion-tier alert answers the masked 404 outside the team, and a team member needs the `record_open` step-up (401 without it) | Nothing outside the case team: a suspicion row is 404 for everyone else. Reason codes are catalogue feature keys and bilingual labels; the import rule of section 3.5 keeps screening sources out of every feature, so a normal-tier analyst's reason never names a list or a screening hit |
| `model_band` on a queue row | A1, A2, M in their queue rows | Same as the row itself; suspicion rows are not in the list for anyone outside the team |
| Model card (sections 1 to 10) | MRO, M, IA, CH (`model.read`); thresholds and the top-band cut for MRO and M only | Nothing about any case or party: the card has no alert, case or party identifier (section 3.2) |
| Monitoring points, label counts, ladder | MRO, IA, M, CH | Counts and distributions only, over all live-scored alerts and decisions of a 30-day window, every tier included (as phase 24 `str_rate`); every count under the small-number threshold is suppressed. Nothing about one case: no id, no party, no day-level row of one decision. A reader comparing two consecutive daily points can learn that some report decision was made in the window, never on which case (gap pass item) |
| Override rate | MRO, IA, M, CH | A window rate over decisions of every tier, suppressed below the threshold; no reason text, no decider, no case id |
| Shadow and challenger scores | Nobody. No route returns them; the monitor reads them only as aggregates (PSI) | Nothing |
| Kill-switch state | MRO, IA, M, CH | The scope, the time, the actor's role and a reason code from a closed list (`regulator_request` names no case or party) |
| Register findings of a red breach | Phase 24 register roles (its own review) | A metric name, a value and a band; no party or alert |
| Fairness rows of the model segment | Phase 24 roles (`fairness.read`) | A segment rate, suppressed below the threshold; no id |
| D4 screen (`/models`) | The same roles as the reads above | The composition of the rows above; nothing else |

No surface in this phase sends a notice, an e-mail or a SIEM event that carries a score, a band, a reason code or a count below the threshold. The SIEM carries only the flagged audit kinds of section 3.13, which hold no score.

### 3.15 Settings used

Keys of section 1.3 (`model.*` and three `schedule.model_*` cron keys). Reused keys: `validation.evidence_max_days` (phase 24), `review.rules_months` (phase 24), `register.finding_due_days` (phase 24, floor 30 to 365), `fairness.review_ratio` (phase 24), `report.small_number_threshold` (phase 21, never below 3), `model.*` floors as shown. Design constants (not settings, listed in Open doubts 2): the shadow criteria floors (1,000 scored alerts, rank correlation 0.5, top-band share change 0.05), the calibration bands (slope 0.8 to 1.2, expected-to-observed ratio 0.8 to 1.25), the PSI bin count (ten) and floor (0.0001), the 30-day population of the monitor, the 30-day retention of a retired version's monitoring, and the 50-alert chunk of the kill-switch check.

### 3.16 Lock order

Taken in this order, in every path that takes more than one:
1. The phase 04 workflow instance lock (`admin.workflow_instances`), first in any change-request path (promotion, kill release).
2. `admin.models` (the family row), `FOR UPDATE`.
3. `admin.model_versions` (the candidate, then the previous live version by id).
4. `admin.inventory_items` (read `FOR SHARE` for the use gate; written only by phase 04 inventory updates, which take their own lock first).
5. `admin.model_kill_state` (the scope row).
6. `admin.registers` (insert only, for a breach).

Scoring takes no lock on the family or the version: it reads the family, the version and the kill row, and writes `alert.model_scores` by insert. Import takes lock 2 and then inserts version rows. Engage takes lock 5 alone. The sweep and the monitor never hold a lock across the batch: a sweep chunk (section 3.4 step 6) and a monitor batch are each one transaction.

---

## 4. Infrastructure Updates

### 4.1 Job kinds (Postgres `jobs`; coalesce key in brackets)

| Kind | Binary | Payload | Concurrency, attempts | Schedule or trigger |
|-|-|-|-|-|
| `model_import_verify` | worker | `{ version_id }` [`model_import_verify:<version_id>`] | 1, 10 | enqueued in the request transaction of `POST /models/{id}/versions` (section 3.3); reads the artifact through `ArtifactStore` |
| `model_score_sweep` | worker | `{}` [`model_score_sweep`] | 1, 3; lease 5 min | `schedule.model_score_sweep` (every minute); section 3.4 |
| `model_monitor` | worker | `{ day }` [`model_monitor:<day>`] | 1, 3 | `schedule.model_monitor` (05:00 bank time), `day` = the previous bank day; section 3.8 |
| `model_label_gate` | worker | `{}` [`model_label_gate`] | 1, 3 | `schedule.model_label_gate` (Monday 04:00 bank time); section 3.6 |
| `fairness_monthly` (phase 24, extended) | worker | unchanged | unchanged | unchanged schedule; adds the metric `score_top_band_rate` (section 3.5) |

Every handler re-reads its owner first (the version row, the family, the kill row). No job of this phase reaches the core banking system: the workers have no source connection (phase 13 rule, phase 00 zones). The job names do not use the `ingest_` prefix.

### 4.2 Timers and outbox

No timer kind is added. No outbox topic is produced or consumed: the import, the promotion and the kill switch are synchronous changes, and the notice of GV-29 is the audit event and the registry read (section 3.10). `alert.created` keeps its one handler of phase 22 untouched; the sweep is a schedule, not a second consumer of the alert topic (phase 22 rule).

### 4.3 AppState additions

- `akr-api`: `models: Arc<ModelRegistry>`, `kill: Arc<ModelKillSwitchService>`, `reads: Arc<ModelReads>`. The `alerts` router resolves the alert through the phase 17 visibility predicate `services::visibility::build` and reads `alert.model_scores` through `ScoreRepository::for_alert_live`. The evidence store (phase 02) is already in AppState for the card stage.
- `akr-worker`: `import_verifier: Arc<ModelImportVerifier>`, `scorer: Arc<ModelScorer>`, `monitor: Arc<ModelMonitor>`, `label_gate: Arc<ModelLabelGateService>`, `runtime: Arc<dyn ModelRuntime>` (`ort` with `tract` fallback), `signatures: Arc<dyn SignatureVerifier>`, `artifacts: Arc<dyn ArtifactStore>` (read-only volume), `inspector: Arc<dyn OnnxInspector>`.
- `ChangeTargetRegistry` in both binaries: `ModelPromotion` (`model_version`) and `ModelSwitch` (`model_switch`); phase 04's `activate_change` effect is the only terminal effect used.

### 4.4 Network zones, artifact store, keys, metrics, readiness, OpenAPI

- **Zones:** `akr-worker` keeps its zone with no egress; it reaches PostgreSQL, Valkey and OpenBao only, as before. The model runtimes (`ort`, `tract`) are linked into the worker binary and make no network call. Scoring never reads the CBS (phase 13 rule).
- **Artifact store:** a read-only volume `model-store` mounted into `akr-worker` at `/var/akr/models` (`artifact_key` names a file under it, for example `import/<sha256>.onnx`). It is written by the signed bundle import (phase 00 `bundle import`, which also brings the starter models a phase 25 release carries; the release itself imports no model, DR-390), and by `akr-ctl model artifact add --file F` on the bank host for a bank-trained model (TS-16): the operator command writes `import/<sha256>.onnx` once, refuses an existing name, prints the sha256 for the manifest and writes no database row (the same host-side pattern as phase 25 `akr-ctl support-bundle`). No route writes it. `akr-api` does not mount it: the import route checks the manifest and the signature only, and the worker reads the bytes (section 3.3). Backups of the volume are part of the phase 00 backup set (Open doubts 5).
- **Keys (DR-261):** the artifact verification keys are Ed25519 public keys in OpenBao KV at `keys/model-artifact/v<N>.pub`, imported by `akr-ctl model keys import --pub FILE --fingerprint HEX` with the fingerprint received on a separate channel (the phase 00 bundle pattern). No transit signing key exists for models: the vendor or the bank's model team signs offline. The key purpose is new in this phase and is verify-only; the phase 00 transit list is unchanged.
- **Compose:** the `akr-worker` service gains the volume and the environment variable `AKR_MODEL_STORE=/var/akr/models`; `akr-api` gains nothing.
- **Metrics** (labels never carry an id, a case or a party): `akr_model_scored_total{role}`, `akr_model_load_refused_total{reason}`, `akr_model_import_refused_total{reason}`, `akr_model_kill_engaged{scope_kind}` (gauge, from a 60 s sample), `akr_model_monitor_red` (gauge, count of red points of the last run).
- **Readiness:** `akr-worker` reports `model_runtime` ready when the pinned `ort` loads its own self-test model; a live version whose load fails makes the worker report degraded (the scoring stops, the queue stays rule ordered, section 3.3).
- **OpenAPI tags:** `models` (routes 1 to 15) and `alerts` (route 16, added to the phase 17 tag). The literal paths `/models/kill-switch` and `/models/ladder` are registered before `/models/{id}`.

### 4.5 Existing entity, schema, route and code updates (extensions of earlier phases)

- **Phase 17:** `GET /queues/{id}/items` ORDER BY and join of section 3.4 (rank join, kill condition); queue row gains `model_band` for A1, A2 and M (section 5.2); `AlertIntakeService` is not changed (the sweep scores, no job is enqueued at intake); the MASKED_KINDS probe set of kind `alert` gains route 16 (phase 03 inference harness, probe set of phase 17 section 6 check 9). `alert.alerts` is not altered.
- **Phase 18:** none. Decisions are read through `casework.decisions.type` and `case_alerts` only.
- **Phase 19:** none. The forced sample stays on the rule score (its own population `low_score_forced`, which is the unbiased set of section 3.6).
- **Phase 24:** the two CHECKs of section 1.1; the `fairness_monthly` metric `score_top_band_rate` (section 3.5); the `ValidationPackService::issue` model branch (section 3.6); the review calendar gains the derived row kind `model_kill_drill` from `admin.models.next_kill_drill_due` (amber 30 days before, red on the due date) and the derived row of a starter family's `recalibration_due` (no new calendar table, DATA-MODEL rule); the closure-code list of phase 24 (section 3.4 there: `EFFECTIVE`, `RULE_CHANGED`, `COMMITTEE_ACCEPTED`, `NOT_REPRODUCED`, `DUPLICATE`) gains `RETRAINED`, `RECALIBRATED`, `RESTRICTED_USE`, `RULES_ONLY` and `JUSTIFIED_NO_ACTION`, and the register update refuses a closure code outside these five for source `model_monitor` (422 `E_VALIDATION` `closure_code_not_allowed`); the command centre (`CommandCentreService::tiles`) gains one counts-only tile `model_governance` (open red breaches, switches engaged), visible to M, CH and IA through `dashboard.read`.
- **Phase 02:** the evidence owner registry gains `model_version`; the audit kind catalogue gains the kinds of section 3.13 with their SIEM flags.
- **Phase 03:** `ChangeTargetRegistry` gains the two targets (section 4.3); the Cedar bundle gains `C1_models.cedar` and the schema additions of section 3.12.
- **Phase 04:** none. The inventory items of kind `model` use the existing create and update methods; `next_validation_due` of the family is set at creation and moved by a passing validation (section 7+).
- **Phase 23 (engine, requirement):** before a partition of `alert.alerts` is detached or dropped, the engine deletes the `alert.model_scores` rows that reference it (`record_purge`, class R5); the composite foreign key is `ON DELETE RESTRICT` (Open doubts 13).
- **Phase 25 (requirement):** the support bundle and the SIEM export exclude `alert.model_scores` by table name (section 3.5); a model that a release carries is imported by people through section 3.3 as `registered` only (DR-390).
- **Phase 00:** none, beyond the key purpose of section 4.4 and the `model-store` volume of the worker.

---

## 5. Frontend Impact

### 5.1 D4 Model registry and AI governance (`/models`; REGISTER; catalogue ID D4, `docs/DESIGN-SYSTEM.md` line 713)

REGISTER pattern (the phase 24 G4 registers pattern), four tabs: **Models** (family list with tier, starter flag, live version number, next drill and pending notices; family detail with the versions timeline), **Kill switch** (the global row and one row per family, with the step-up press and the release request), **Monitoring** (the drift and override points as bands, with the small-number suppression shown as "fewer than 3" in words; a dataviz line chart of PSI against the thresholds, palette from the dataviz skill), **Ladder** (per family: enabled, tier, label counts, next gate).

- The version detail shows the ten card sections as a reading column. The operating-point block is rendered only when the API returned it (MRO and M); IA and CH see the sections without it and no placeholder.
- The validation stepper of a version has five steps: benchmark, out-of-time, calibration, shadow criteria, validation pack. Each step shows the evaluation's `passed` and its date; the pack step links to the phase 24 pack page. The "Propose promotion" action is enabled only when all five pass and the server agrees (the server's answer is the authority; the stepper only reads).
- Kill-switch and pin actions open the step-up dialog of phase 01 before the request; a `E_STEPUP_REQUIRED` answer reopens it. The engage action asks for the reason code from the closed list and shows that the queue returns to rule order at once.
- Words: "Model" and "Score" in the UI; "suspicion" never appears on D4. Labels come from the API's bilingual objects (section 11 of DESIGN-SYSTEM, Arabic first). No score number appears in a URL, a title or a log line.

### 5.2 W2 Alert and case queue (`/queues/:id`; extension of phase 17 section 5)

- A row shows a small `model_band` chip (`top` or nothing) for A1, A2 and M only; the sort tooltip of W2 reads "Sorted by priority class, then model rank when the model is live, then rule score, then age".
- The alert panel (UX-08, evidence first) shows the rule facts first, then the model rank: the score as a band word, not a number, and at most three reason labels beside the facts they name. A shadow or challenger score is never shown. A suspicion-tier alert panel shows the step-up of phase 17 before any of this.
- "Pause live updates" and "Next" are unchanged.

### 5.3 Generated hooks (Orval names from the paths)

`useGetModels`, `usePostModels`, `useGetModelsKillSwitch`, `useGetModelsLadder`, `useGetModelsId`, `usePostModelsIdVersions`, `useGetModelsIdVersionsVid`, `usePostModelsIdValidation`, `usePostModelsIdShadow`, `usePostModelsIdVersionsVidPromote`, `usePostModelsIdPin`, `usePostModelsKillSwitch`, `usePostModelsKillSwitchReleaseRequests`, `usePostModelsIdKillDrills`, `useGetModelsIdMonitoring`, `useGetAlertsIdModelScore`. The phase 17 `useGetQueuesIdItems` hook keeps its name; its row type gains `modelBand`.

### 5.4 Browser and caching rules

- The kill-switch state is polled every 30 seconds while D4's Kill switch tab is open, and never cached past one poll.
- Scores and reason codes are never stored in the browser (no local storage, no URL); the query cache of the alert panel is dropped when the panel closes.
- Monitoring and ladder reads are cached for one minute; the stale-while-revalidate behaviour is the phase 01 default for read queries.

---

## 6. Verification

1. `make migrate` applies `0085_admin_models_registry`, `0086_alert_model_scores` and `0087_admin_models_guards_grants` on a clean database and again on the migrated one (idempotent). Check on a throwaway `docker run --rm postgres:17`: the two phase 24 CHECKs accept `model_monitor` and `score_top_band_rate` and reject `scan2`; the partial unique indexes reject a second `live` version of one family.
2. `cargo check` with the SQLx offline data regenerated for the new queries (the queue join of section 3.4 and the reads of section 3.10 are checked at compile time).
3. `cargo clippy -- -D warnings` for `akr-core`, `akr-api`, `akr-worker` and the new `crates/ml`.
4. `cargo test` unit tests in `domain::models`: the PSI arithmetic against a hand-computed table (ten bins, floor 0.0001); the band boundaries at exactly 0.10 and 0.25; the override definition on six decision/alert combinations; the label-gate counter (starter 50, bank-trained 200 and 500); the seven-part mapping of the validation pack; the operator allow-list (every listed operator accepted, `ZipMap` and a custom domain refused); `ProtectedAttributeCheck::check_features` on the three attribute classes of section 3.5; the reason-code selection (three positive contributions, never a negative one).
5. Golden tests: TreeSHAP of `ExplanationService` against the reference values computed offline for the 20 golden rows of a fixture model, within 1e-4; additivity (base value plus contributions equals the logit within 1e-6); `ort` and `tract` produce the same golden outputs within 1e-6 on the fixture, and a deliberately altered fixture (one leaf changed) fails with `golden_mismatch`.
6. Visibility pin: over the generated principals and alert rows of phase 17 check 6, route 16 answers the masked 404 on exactly the rows on which phase 17 `GET /alerts/{id}` answers it for the same caller and headers. Route 16 applies the visibility rules in the handler through `services::visibility::build` (section 3.4 step 1), and phase 17 check 6 pins that predicate to the Cedar decision for `alert.read`. The permits of `alert.model_score.read` are by role only (section 3.12) and no forbid of phase 17 names the action, so this pin is on the handler, not on a Cedar decision for the action.
7. Bundle tests of section 3.12 pass: `akr-ctl policy validate` prints the new bundle hash with `C1_models.cedar`; the matrix of who-may-do-what is the one written in section 3.12.
8. Curl suite `tests/curl/phase27_models.sh` (dev users `mro@bank.test` (MRO), `mro2@bank.test` (a second MRO for the approval), `chk@bank.test`, `m@bank.test`, `ch@bank.test`, `ia@bank.test`, `a1@bank.test`, `a2@bank.test`, `sec@bank.test`). Each check states its expected answer:
   - a. MRO creates a family with `Idempotency-Key`: 201; the same key and body again: 201 with `x-idempotent-replay: true`; the same key and another body: 422 `E_IDEMPOTENCY_REUSE`.
   - b. Import with a bad signature (a manifest signed by a key not in OpenBao): 422 `E_VALIDATION` `signature_invalid`; no version row; audit `model.version_refused` present with the SIEM flag.
   - c. Import of the fixture artifact that uses `ZipMap`: 202, then after the worker runs, `GET /models/{id}/versions/{vid}` shows `status: refused`, `refusal_code: operator_unsupported`.
   - d. Import of the good fixture: 202, then `registered`, with `runtime` set and the card in evidence.
   - e. Shadow start with success criteria before the status is `approved_for_use`: 409 `E_NOT_ALLOWED_IN_STATE` `inventory_not_approved`.
   - f. Promotion request without a passing validation pack: 409 `E_NOT_ALLOWED_IN_STATE` `pack_missing`; with `If-Match` omitted: 428 `E_PRECONDITION_REQUIRED`; with a stale version: 409 `E_VERSION_CONFLICT`.
   - g. Promotion request with a passing pack, a shadow run of at least `model.shadow_min_days` (test clock) and criteria met: 202; the approval by MRO `mro` who proposed it: 403 `E_SELF_APPROVAL`; the approval by `mro2`: the next step; the `cr_material` approvals by M (the MLRO desk) and CHK: the version is `live`; audit `model.promoted`.
   - h. Promotion of a candidate while the family is pinned to another version: 409 `pinned`.
   - i. Global kill engaged with `mro` and step-up: 200; without step-up: 401 `E_STEPUP_REQUIRED`; with a stale `If-Match`: 409 `E_VERSION_CONFLICT`; then `GET /queues/{id}/items` returns the rule order (the `model_band` of every row is absent) and the scored alerts' stored scores are unchanged in the database.
   - j. Release request with `mro` and approvals by `mro2`, M (the MLRO desk) and CHK: the switch is released and the queue returns to model order.
   - k. Shadow score never returned: for an alert with a shadow row and no live row, `GET /alerts/{id}/model-score` as `a1` returns `live: null` and no field carries the shadow score (the response is searched for the shadow value and found absent).
   - l. Suspicion-tier alert: `GET /alerts/{id}/model-score` as an `a1` on the case team without step-up: 401 `E_STEPUP_REQUIRED`; on the team with step-up and a missing purpose: 404 `E_NOT_FOUND`, the masked answer (rule 5 of phase 17 admits no caller without a purpose code; 422 `E_PURPOSE_REQUIRED` is the answer on a restricted alert only); as an `a1` outside the team, with or without the step-up and the purpose: 404 `E_NOT_FOUND` with the body identical to a missing id, and the latency within the phase 03 floor (phase 03 section 3.6 step 3 for the floor, section 7.14 for the answer).
   - m. Cedar deny: `ia@bank.test` on `GET /models/{id}/versions/{vid}?view=operating_point`: 403 `E_FORBIDDEN`; `ch@bank.test` on `POST /models/kill-switch`: 403 `E_SCOPE`; `a1@bank.test` on `GET /models/{id}/monitoring`: 403 `E_SCOPE`.
   - n. Monitoring with fewer than three decisions in the window: `band: suppressed`, `value: null`.
   - o. Kill drill recorded: 201; `next_kill_drill_due` moves by `model.kill_drill_months`; the calendar lists the new date.
   - p. A red breach finding closed with `DONE`: 422 `E_VALIDATION` `closure_code_not_allowed`; with `RULES_ONLY`: accepted.
9. `make generate-api` regenerates the client; the hook names of section 5.3 exist; the CI drift check passes.
10. Audit trail: after the curl session, the audit rows of section 3.13 appear with the expected kinds, the SIEM flags, and no payload contains an alert, party or customer identifier (a query over `audit.audit_log` payloads for the uuid pattern of alerts returns zero rows for these kinds).
11. Guard tests: a direct `UPDATE admin.model_versions SET status = 'live'` from `akr_app` outside the promotion transaction is refused; a direct `DELETE` on `admin.model_evaluations` by `akr_app` is refused; `akr_app` has no UPDATE on `alert.model_scores` (privilege check); a direct update of `artifact_sha256` is refused by the guard.
12. Phase 23 interaction: a retention dry run over a partition of `alert.alerts` lists the `alert.model_scores` children to purge first and does not attempt the partition drop while they exist.
13. Read-only conformance (phase 00 zones): the `akr-worker` network policy has no route to the CBS source; the scorer code path has no source pool in its dependency tree (a build-time check of the crate graph).
14. Existing suites (phases 17, 18, 19, 24 and 25) pass with the widened CHECKs and the changed queue order (the rule order is the default when no model is live).

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 2: corrections found while checking phases 03 and 04 (tools/agent/gap1-run2-verdict.md); lead-verified.

### 7.1 Change targets belong to phase 04's registry
Section 4.5's "Phase 03: ChangeTargetRegistry gains the two targets" reads "Phase 04": the registry is phase 04's.

Gap pass 1 (2026-10-10), run 4b, from tools/agent/gap1-run4b.md; lead-verified (tools/agent/gap1-run4b-verdict.md).

### 7.2 Validation date of a model family
When the validation pack of a model gets verdict `pass` (phase 24 section 3.6 step 8), phase 24 calls phase 04
`InventoryService::update` on the inventory item (`if_match` = the item's `version`, read in the same transaction) to set `last_validated_at = now()` and `next_validation_due = today +
review.rules_months`. The phase 24 review calendar is read-only; it lists the item when the date nears. Section 2's
phase 04 line says the date is moved by a passing validation.

### 7.3 Masking probe for the model-score route
This phase adds route 16 (`GET /alerts/{id}/model-score`) to the probe set of kind `alert` in phase 03's
`MASKED_KINDS` registry: an alert the caller may not see answers the masked 404, byte for byte the answer for an
unknown id, and the body carries no score, band or reason code. A caller outside the team gets that 404 with or without a step-up. Only a caller the visibility rule admits to a suspicion-tier alert gets 401 `E_STEPUP_REQUIRED` for a missing `record_open` step-up (phase 03 section 7.14, DR-411).

### 7.4 Inventory items keep R9 (DR-392)
`admin.inventory_items` keeps R9 for every row, model items included; model evidence is R11 through the retention rows
of section 1.1. Phase 04's note "model items move to R11 in phase 27" is corrected (phase 04 section 7+).

Gap pass 1 (2026-10-10), run 4b: items carried from run 4a and corrections found while checking phases 25 to 31 (tools/agent/gap1-run4b-verdict.md); lead-verified.

### 7.5 Advance notice (Open doubts 9 closed)
The GV-29 notice is the audit record `model.version_imported` and the `pending_notices` list on `GET /models`. No
e-mail or in-app notice is sent in this release (the rule of phase 22 Open doubts 14: the screens are the notice).

### 7.6 Release bundles import no model (DR-390)
A model that a phase 25 release carries is imported by people through section 3.3, as `registered` only; the release
itself imports nothing (phase 25 section 7+). The sentences of this phase that said "the release import" read so.

### 7.7 Retention DELETE grant (DR-386)
`akr_retention` gets SELECT on the six new tables and DELETE on `alert.model_scores` only: its rows are deleted by
`record_purge` before an alert partition is dropped (DR-387). `admin.model_evaluations` (R11) is `report_only`,
`admin.model_monitor_points` (R9) is kept, the R11 `retirement` and R10 tables are never purged. The delete guards
that admit `akr_retention` stay; they admit, they do not grant.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.8 The inventory item is created before its object row (V-27-02-a, HIGH; V-27-02-d, LOW; DR-413)
`inventory_item_id` is `NOT NULL` with a foreign key that is not deferrable, on `admin.models` and on `admin.model_versions`, so the row cannot be inserted before its item. Section 3.1 step 2, the version item paragraph and section 3.3 step 5 run in this order, in the one transaction they already use:
1. Generate the family id (or the version id) app-side.
2. Call phase 04 `InventoryService::create` with `kind = model`, `object_id` = that id and the fields of section 3.1. The family item's `name` is the family's bilingual `name`; a version item's `name` is the family's name with ` v<version_no>` appended in both languages.
3. Insert `admin.models` (or `admin.model_versions`) with that id and `inventory_item_id` = the new item's id. For a family, its `admin.model_kill_state` row follows in the same transaction.
Phase 04 allows an item before its object (phase 04 section 3.9) and calls no resolver at create. The resolver of kind `model` that this phase registers is called by the use gate of section 3.1, and it finds the id in `admin.models` or `admin.model_versions`. The transaction commits both or neither, so no item is left without its row.

### 7.9 Reads of the daily monitor, and its audit (V-27-03-a, V-27-03-c, MEDIUM; DR-413)
Section 2 gains the reads behind the metrics of section 3.8:
- `ScoreRepository::score_window(conn, version_id, role, from, to) -> ScoreWindow { scored, top_band, bins }`: the `alert.model_scores` rows of the version in that role and window; `bins` are the ten score counts of section 7.10.
- `ScoreRepository::entered_window(conn, from, to, not_before) -> i64`: the alerts of `alert.alerts` created in the window and on or after `not_before` (the denominator of `scored_count`).
- `MonitorRepository::override_window(conn, from, to) -> (i64, i64)`: the decisions of the window on cases with live-scored alerts and, of those, the overrides by the two rules of section 3.8. It reads `casework.decisions` (`type`, `decided_at`), `casework.case_alerts` and the live `top_band` flags.
- The feature values of `feature_psi` come from phase 13's `FeatureValueReader::read`, as in section 3.4 step 3.
After its last insert, the daily run writes one audit `model.monitored {day, points, suppressed, red}` (counts only; resource System; SIEM flag false), and section 3.13 gains that row. A day whose points all exist already writes none.

### 7.10 Reference bins and the provisional top-band cut (V-27-05-b, MEDIUM; DR-413)
`reference_bins` has one shape: `{"score": [ten counts], "features": {"<key>@<version>": {"edges": [nine ascending numbers], "counts": [ten counts]}}}`. The score uses ten equal-width bins on 0 to 1 (bin k covers from (k-1)/10 up to, not including, k/10; the last bin includes 1), so it carries no edges. Each feature of `feature_keys` carries its nine inner edges and ten counts; a missing value is not binned. A manifest without the score counts or without a feature of its list, or with arrays of another length, is `manifest_invalid` (section 3.3 step 4). `score_psi` and `feature_psi` of section 3.8 count the stored values in the same bins.
Provisional cut (section 3.6 step 4): scanning the score bins from the top, the cut is the lower edge of the first bin at which the cumulative share of the reference counts reaches `model.top_band_percent` / 100 (the setting is in per cent). Nothing is interpolated. The promotion replaces it with the percentile of the version's own shadow rows (section 3.7 step 4).

### 7.11 Shadow start checks If-Match (V-27-05-c, LOW; DR-413)
Section 6 check 8 gains item q: `POST /models/{id}/shadow` without `If-Match` answers 428 `E_PRECONDITION_REQUIRED`, and with a stale version it answers 409 `E_VERSION_CONFLICT`.

### 7.12 Pin targets are checked (V-27-06-b, MEDIUM; DR-413)
Section 3.10 Pin, before `pinned_version_id` is set: a `version_id` that is not a version of this family is 422 `E_VALIDATION` at `/version_id`; a version whose status is not `registered`, `shadow`, `challenger` or `live` is 409 `E_NOT_ALLOWED_IN_STATE` with sub-code `not_pinnable`. A null `version_id` unpins and needs neither check.

### 7.13 ModelSwitch: the other ChangeTarget methods (V-27-04-c, LOW; DR-413)
`ModelSwitch` implements phase 04 `ChangeTarget` with: `kind() = model_switch`; `reason_families() = ["change"]`; `generic_create() = false` (the only entry is the release route of section 3.9 step 3); `emergency_allowed() = false`; `inventory_kind() = None` (the GV-01 gate applies to promotion only, section 3.7); `revert` returns `ChangeError::NotRevertible`, as in phase 16 (it is never called, because `emergency_allowed` is false). `current_version(conn, cr)` returns the `version` of the `admin.model_kill_state` row named by the change's `object_key` (`kill:<scope>`), which is the base version of the release, so phase 04's `E_STALE_BASE` applies when it has moved. `on_submitted` writes nothing. `diff(cr)` shows `engaged` going from `true` to `false`; the scope and its `reason_code` do not change. `on_closed` changes nothing: a rejected, withdrawn or failed release leaves the scope engaged.

### 7.14 The shadow start is one repository call (V-27-05-a, MEDIUM; DR-413)
`VersionRepository` (section 2) gains `begin_shadow(tx, id, role, shadow_criteria, top_band_cut, expected_version)`. It runs one `UPDATE` of `admin.model_versions` with `WHERE id = $1 AND version = $expected_version`, which sets `status` to the role, `shadow_started_at = now()`, `shadow_criteria`, `top_band_cut`, `retired_at = NULL` and `version = version + 1`. Section 3.6 step 4 is this one call; `set_status` is not used for it. Every column it sets is in the `UPDATE` grant of section 1.3, and the version guard admits the edges it uses (`registered` to `shadow`, and the rollback `retired` to `shadow` with new criteria).

## Decisions

1. **DR-257 (the model score ranks inside the class and is stored beside the alert).** The score is a separate row in `alert.model_scores`, never a new column of `alert.alerts` and never a change of `alert.alerts.score`. It orders alerts inside their priority class only (queue order: class, model rank, rule score, age), never sets or changes the class, never suppresses, closes, hides or creates an alert, and is joined rather than copied, so a kill switch or a promotion changes the order without rewriting any alert. Rationale: M-03 and T-13 (the class is decided before any score; the scorer ranks rule alerts only), and phase 17 queues already sort by class then score, so the model term is one more key.

2. **DR-258 (schemas: registry in `admin`, scores in `alert`).** `models`, `model_versions`, `model_evaluations`, `model_kill_state` and `model_monitor_points` are in `admin` with DATA-MODEL's names; `model_scores` is in `alert` because its composite foreign key must point into the partitioned `alert.alerts` (phase 17 PK `(id, created_at)`), it takes the R5 alert clock through its owner, and it keeps the grants and the partition-drop rule of the alerts it belongs to (phase 23 requirement, Open doubts 13). Rationale: DATA-MODEL section 15 lists the table in the governance group, and this decision moves only its schema; the roadmap is aligned by the lead.

3. **DR-259 (every version is an inventory item and only `approved_for_use` items score).** A family and each of its versions are inventory items of kind `model` (`object_id` is the family id or the version id). Shadow, challenger and live scoring all require `approved_for_use` on the version's item, so a shadow period starts only after the validator has approved the version for use; promotion to live is a separate change request on `cr_material_validated`, which needs the same item status. The rule that no score comes from a version whose inventory item is not `approved_for_use` is applied to shadow scores as well, because a shadow score is a use of the model even though no analyst sees it. Rationale: GV-01, GV-32, and the phase 04 gate, which already names model kinds.

4. **DR-260 (import in two steps: synchronous checks in the API, artifact checks in the worker).** `POST /models/{id}/versions` checks the manifest hash and the Ed25519 signature synchronously (the cheap, decisive checks; a bad signature is refused before anything is written) and enqueues `model_import_verify`, which reads the bytes, checks the operator allow-list, the catalogue features, the protected-attribute rules and the golden rows. The artifact never travels through the API, the API has no ONNX runtime, and the card is staged as evidence in the same request. Rationale: TS-16 (signed registry), the M15 edge case of a bad signature refused at load, and the phase 02 rule that generated evidence is written by the worker.

5. **DR-261 (runtime, keys and loads).** `ort` is pinned and vendored; `tract` is the per-version fallback when the golden rows fail under `ort`; both live in `akr-worker` only, which has no egress. Signatures are Ed25519 over the canonical manifest with verify-only keys in OpenBao KV (`keys/model-artifact/v<N>.pub`), signed offline by the vendor or the bank's model team; a new key purpose is added and no transit signing key is created. The signature and the artifact hash are checked at import and at every load (worker start, promotion, switch release, every 10 minutes); a failed load is audited, and scoring stops for that version. Rationale: TS-16, the ort release-candidate status in tech-stack.md line 29, and the signed-bundle rule of phase 00.

6. **DR-262 (validation through phase 24, the model branch fills the seven parts).** The validation pack of a version is phase 24's pack with the same seven part keys and a model meaning for each (section 3.6), its verdict is computed by the phase 24 rule, its validator is never its owner, and a `pass` moves the inventory dates through phase 04. Promotion requires a pack issued after the shadow started. Rationale: GV-09 and GV-15 are already carried by phase 24; a second pack type would be a second evidence path.

7. **DR-263 (label gates from QA-confirmed outcomes).** Positives are QA-confirmed STR, UTR or TFR outcomes; negatives are QA-confirmed no-report outcomes including the low-score forced sample of phase 19. Gates are 50 for a starter family (recalibration), 200 to enter shadow and 500 to go live for a bank-trained family, all governed with floors. Rationale: T-15 and research 21 4f (about 50, 200 and 500), GV-39 (the labels do not come from the model's own suppression, which does not exist), and the phase 19 QA population that already exists.

8. **DR-264 (baseline, out-of-time and shadow criteria are fixed in advance).** A version beats a transparent monotone scorecard on the bank's out-of-time window with a 95 per cent interval that excludes zero; a shadow run's success criteria are frozen when it starts (a closed key set, with three design-constant floors); the shadow summary is recomputed at the decision and never reused. There is no mandatory parallel run of the old system: the comparison is on the same live alerts. Rationale: GV-14, GV-16 and GV-24 (research 29 9e: pre-agreed criteria, not re-detection of historic alerts).

9. **DR-265 (monitoring: PSI and override, breaches become register findings).** Score and feature drift use the population stability index against the reference bins of the import manifest; the override rate uses phase 18 decision types against the top band. Bands are green, amber and red with governed thresholds whose floors bound the weakening side (`psi_watch` at most 0.10, `psi_action` at most 0.25, `override_rate_action` at most 0.30). A red point opens one phase 24 register finding with source `model_monitor` (the phase 24 CHECK is widened, section 1.1) and closes only with one of five recorded decisions (GV-19). Rationale: research 29 9d (thresholds and the action levels), GV-18, GV-19 and GV-21.

10. **DR-266 (kill switch: immediate to engage, a change request to release).** Engaging the global or a per-family switch is immediate under step-up `approval` by an MRO, writes no change request, and makes the queue rule ordered at the next request; releasing is a change request on `cr_material` with the MLRO (M) and CHK approvals, so a single person cannot re-enable a switch that a red breach engaged. The drill is recorded quarterly and drives a phase 24 calendar row. Rationale: GV-20 and MAS 6.5.6 (executable at once), with the same safe-direction reasoning as phase 16's scenario kill switch (DR-108).

11. **DR-267 (pins and releases never change the live model).** A bank can pin a version; a pinned family refuses every promotion of another version until unpinned; a model that a phase 25 release carries is imported as `registered` only (by people, DR-390); the first install of a starter family goes live through the same promotion change request. Rationale: GV-29, the same rule as phase 16 scenario releases and phase 25 release installs (a release never replaces a live version the bank approved).

12. **DR-268 (fairness and features: refuse at import, measure by segment).** A model's features are checked against the protected-attribute register at import (prohibited inputs, the allowed-use matrix with the use `score`, the proxy rule, and screening sources, which are never a feature of a normal-tier score); the sensitivity sweep proves that geography, residence and nationality are never a sole trigger. Model fairness is one metric value added to phase 24's monthly segment rates (`score_top_band_rate`), with the closed group kinds unchanged. Rationale: GV-36, GV-37 and GV-38; the allowed-use matrix is data in phase 24's register, not code.

13. **DR-269 (reason codes from TreeSHAP, bilingual through the catalogue).** Reason codes are the three largest positive contributions of an interventional TreeSHAP computed in-house on the stored tree structure and checked against golden rows; they are catalogue feature keys with versions, and their Arabic and English labels are the catalogue's `name` (phase 13). Only three are ever shown, beside their facts (phase 17 UX-08). Rationale: GV-41 (model part), TS-16 (TreeSHAP in house with golden tests), and the M15 requirement of local explanations.

14. **DR-270 (tipping-off and visibility of scores).** Scores and reason codes are read through the phase 17 visibility predicate and the phase 03 masking of suspicion tier, with purpose binding; the monitoring, ladder and override reads are window counts from stored points over every tier, suppressed below the threshold (lead review); shadow and challenger scores are never returned by a route; no export or notice carries a score. Rationale: phase 17 rule 1 and rule 2, GV-40, and the tipping-off rule of DESIGN-SYSTEM section 11.

15. **DR-271 (API rows of 3.15 changed in meaning, six kept).** `POST /models` creates a family with its inventory item (the API row said "register a signed artifact"); version import is the new row `POST /models/{id}/versions`. `POST /models/{id}/validation` records a model evaluation (the pack is phase 24's route, which the API already lists for inventory items), and `POST /models/{id}/shadow` takes the role and the frozen criteria. `POST /models/kill-switch` takes a scope. The change is stated here because the lead's API review must accept it. Rationale: a route named for two things (the family and the artifact) would need two Cedar actions and two scope lists; the split keeps each route's permits exact.

16. **DR-272 (names and services).** The services are `ModelRegistry`, `ModelImportVerifier`, `ModelScorer`, `ExplanationService`, `ModelMonitor`, `ModelLabelGateService`, `ModelPromotion`, `ModelSwitch` and `ModelKillSwitchService` (the kill switch is named for its model scope because phase 16's `KillSwitchService` already exists for scenarios), with `ModelReads` for the read side. The jobs use the `model_` prefix and no `ingest_` prefix. Rationale: the grep of docs/plan (no earlier use of these names) and the conventions file's service-name rule.

17. **DR-273 (drained alerts keep their class and their rule order; no backfill).** The sweep scores only alerts created on or after the family's `live_from` (and shadow alerts after `shadow_started_at`), and a promotion does not rescore stored alerts; an alert created before the promotion keeps the score of the champion that scored it. Rationale: phase 17 "drained, not migrated" (M-75), and the rule that a release never changes an earlier decision.

**Lead corrections (review 2026-10-09).** The version guard froze `shadow_criteria` for ever and let a version leave `live` without the promotion flag, so the rollback by re-shadowing could not run and a retired champion could stay the family's live pointer; the criteria now change only on the edges `retired -> shadow` and `retired -> challenger`, and leaving `live` needs the flag. A bank-trained artifact had no way into the read-only store; `akr-ctl model artifact add` writes it on the host. Route 7 had no steps and `top_band_cut` was NULL during shadow while `top_band` is NOT NULL, so every shadow insert failed; `start_shadow` is written out with a provisional cut from the reference bins. Phase 18 raises the subject to `suspicion` on every report decision, so excluding suspicion tier from the counts removed almost every positive label and every override of kind (b); the jobs now count every tier and only suppressed window aggregates leave them, as phase 24 `str_rate` does. The kill-switch row of a family is created with the family (engage takes `If-Match` on it); the family guard lets `akr_retention` delete; route 16 takes the alert's own purpose rules; the feature lookup names the month-end, currency and status keys of `feat.feature_values`; the five closure codes join phase 24's list; a second family's promotion is refused with `other_family_live`.

## Open doubts (for gap pass 1 and the bank)

1. **Kill switch owner.** GV-20 says "the model owner"; the route scope `model.risk` is held by MRO alone, so a family owner who is not an MRO cannot press the switch. The bank must confirm that every model owner is an MRO, or a scope for owners is added to API section 2 (a change of the closed scope list).
2. **Design values.** The PSI bands (research 29 9d are conventions, not regulator numbers), the override action (0.30), the calibration bands (0.8 to 1.2 and 0.8 to 1.25), the shadow criteria floors (1,000 scored alerts, rank correlation 0.5, top-band share change 0.05), the top-band percentile (10 per cent) and the 30-day windows are proposals. They are settings where governed and constants where not. The bank validates them on its own data before the first family goes to shadow.
3. **Label gates.** 50, 200 and 500 are the starting points of research 21 4f, not regulator figures. The rule "a bank-trained model goes live at 500" is stricter than the research's "local supervised challenger from 500"; the bank may prefer 200 for live with the challenger at 500. Changing the floor needs a change request (settings floors are governed).
4. **No waiver for the baseline.** GV-14 allows no go-live without beating the baseline "without documented reason". This phase builds no waiver: a family that does not beat the scorecard does not go live. If the bank wants a documented exception, it is a change to section 3.6 (a recorded reason, a second approval, a sunset date).
5. **Artifact store and backups.** The artifact bytes live on a volume (`model-store`), not in the evidence vault, because the vault's allowed types do not include ONNX. Retention of the bytes against R11 and the backup set of the volume are phase 00 and phase 23 decisions to confirm. A version whose bytes are lost cannot be re-shadowed (the sha is unique and the check fails). Also: a family's tier change after versions exist is not copied to them; the bank confirms that the version items keep the tier at import.
6. **ONNX-ML operator coverage and the runtime.** `ort` is a release candidate and the ONNX-ML TreeEnsemble coverage is unverified (tech-stack.md line 29). The phase proves coverage only by the fixture set (golden rows, `ort` and `tract` agreement). If neither runtime passes the bank's first real model, the phase has no ML term and the rule ranking stays; this is the stated fallback, not a silent one. The bank's own build of the runtime is a release gate of phase 00.
7. **Sweep lag and unscored alerts.** The sweep runs every minute, so a new alert is unscored for up to one minute; in that minute an unscored alert sorts after scored alerts of the same class. Alerts that wait longer than the backlog of a sweep stop (kill, load refusal) stay unscored until the sweep returns, and they are never backfilled after the switch is released (DR-273). Confirm that the analysts accept this order in the first week.
8. **First install of a starter.** A release installs the starter artifact as `registered`; its first promotion is a change request. Confirm that the bank accepts the one-time change request at install, instead of a release-time activation that phase 25 would have to support.
9. **Advance notice (GV-29).** The notice is the audit record and a pending list on `GET /models`. An e-mail or in-app notice to MRO needs phase 22 to accept a notification that is not tied to a task (its `notify.request` is task-centric). Decide with phase 22's owner.
10. **Model-segment fairness.** `score_top_band_rate` is a proposal: it measures top-band share per segment, which is a score-derived rate and not a false-positive measure. The review ratio is the phase 24 setting; the bank must agree that the top-band share is the right fairness measure for a ranking-only model.
11. **Reason codes and CBI articles.** GV-41 asks for a tie to a CBI article as well as a feature and a typology. The catalogue has feature names; the typology and article mapping of a feature is not in the catalogue yet. Until the catalogue carries it, reason codes are feature labels only (partial GV-41).
12. **Validator independence.** The promotion needs an MRO approval and the validation pack needs an MRO validator who is not the owner. A bank with one MRO cannot promote or validate; the bank must name at least two MROs (phase 04 self-approval rule).
13. **Phase 23 and the alert partitions.** The composite foreign key from `alert.model_scores` into the partitioned `alert.alerts` makes a partition drop refuse while scores exist. Phase 23's engine must purge the child rows first (section 4.5). If phase 23 rejects a purge of a class-R5 child, the scores must move to the partition's retention or the foreign key must be dropped and the link checked by the verifier.
14. **Roadmap and status alignment.** roadmap.md lines 138, 178 and 216 name `models`, `model_versions`, `model_scores` as phase 27 tables, the services `ModelRegistry`, `ModelScorer`, `ExplanationService` and the migrations of phase 27; the schema of `model_scores` is `alert` here (DR-258), the kill switch service is `ModelKillSwitchService`, and there are three migrations, not one. The lead aligns the roadmap; this draft does not edit it.
15. **Requirements on phase 25.** The SIEM export and the support bundle must exclude `alert.model_scores` by table name, and the release imports no model (DR-390). Both are stated in phase 25 section 7+.
16. **Phase 19 forced sample.** The `low_score_forced` population stays on the rule score (its unbiased purpose). Once a model is live, a reviewer might expect the forced sample to use the model's low band; the bank should confirm that the rule score is the right basis for the unbiased set.
17. **Shadow length.** The model's shadow minimum (30 days) equals the scenario shadow of phase 16 (S-44). A model may need longer on a quarterly alert cycle; the floor is a governed setting and can be raised, not lowered.
18. **Rollback by re-shadowing.** A retired champion returns through shadow with its own criteria and a new pack. The bank may want an emergency rollback without a new shadow period; that is an emergency class of change (phase 04 `cr_emergency`), which this phase does not allow (`emergency_allowed = false`).
19. **Kill drill evidence.** The drill is recorded by a person's statement (route 14) and its environment; no automatic drill runs. Whether the drill is a scripted test in the bank's staging environment is for the bank's runbook.
20. **Monitoring of the ranking's effect on coverage.** The metrics show drift and overrides, not whether the ranking moved a true positive down the queue. Measuring that needs the forced sample and the QA outcomes, which are phase 19 and 24 outputs; a dedicated coverage metric is not built here.
21. **Model cards and the ten sections.** The card is written by the bank's model team as JSON against a schema; the phase does not generate the prose of sections 1 to 3. GV-30's "generated from the inventory and test results" is partly met: the test results are generated, the prose is not.
22. **One live family.** The unique index allows one family with a live version per bank, so a bank that changes family retires the old champion (a promotion change of the new family cannot run while the old one is live). Whether a family switch should be one change request (retire A, activate B) is for the bank and gap pass 1.

## What This Phase Does NOT Touch

- No GenAI assistant, no `/assistant/*` route, no `assistant_drafts`, no GV-45 to GV-50, T-48 to T-51, M-70 to M-73 or TS-17 (phase 28).
- No change to `alert.alerts` columns, to `PriorityClassifier`, to the priority rules, to the alert intake steps or to `AlertIntakeService` (the sweep reads the table; no job is enqueued at intake).
- No change to the rule score of phase 17 (`alert.alerts.score`, `ScoreComponents`), the screening alerts of phase 14 or the rating model of phase 15 (`/rating-models`, its shadow and its CR).
- No change to the scenario kill switch, the scenario shadow or the scenario versions of phase 16 (`scn.*`, `POST /scenarios/{id}/kill-switch`).
- No change to the forced QA sample of phase 19 (its population and its settings).
- No outbox topic produced or consumed; no timer kind; no change to `alert.created` or `case.event` handlers.
- No CBS connection, no read of the CBS by the scorer, no change to the source pools or the ingest binary (`akr-ingestd`).
- No change to `C0_governance.cedar`, `C0_packs.cedar`, `C0_policy.cedar`, `C0_reference.cedar`, `60_audit.cedar`, `61_retention.cedar`, `80_detection.cedar` or any other existing Cedar file; no change to `schema.cedarschema` beyond the additions of section 3.12.
- No change to the phase 04 change classes, route map, inventory routes or `activate_change` effect; no emergency change for a model.
- No change to the authority matrix, the step-up classes (`approval` is used as is) or the scope list of API section 2.
- No automatic retraining, no training pipeline in the product, no notebooks: the workbench is outside the product (GV-28 is not carried).
- No change to the bank's calendar, to `BankCalendarClock` or to the review calendar's table (only a derived row kind is added to its read, section 4.5).
- No new problem code (section 3.11 uses API section 6 codes with sub-codes only).
- No second register, no second pack type, no second calendar.
