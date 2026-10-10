# Phase 16: Scenario Engine, Packs & Studio

**Goal:** The scenario engine of DSL.md Part A: versioned declarative scenario documents, validated and compiled to parameterised SQL (aggregate, through the phase 13 measure compiler), CEL (event) and the phase 13 profile comparator; run incrementally on each mapped batch, in settled-time re-evaluation, in sweeps and in shadow; replays with a cost guard, above- and below-the-line sampling with blind labels, and seeded known-answer tests per CBI item; parameter sets with hard bounds and approved bands under change requests; one live version per scenario behind a go-live gate and a step-up kill switch; the CBI catalogue importer and bank scenarios 901-999; and the scenario register (D1, with the coverage matrix to readiness) and the scenario studio (D2).

**Dependencies:** Phase 00 (jobs, outbox, timers, settings, schedules), Phase 01 (`StepUpService`, action class `approval`), Phase 02 (evidence vault for sample packets and the catalogue issuance), Phase 03 (Cedar bundle, masking registry, access events), Phase 04 (`ChangeTargetRegistry`, route map, `EvidenceResolver` for the kinds `replay`, `btl_sample`, `shadow_comparison`), Phase 06 (the register, the CBI catalogue, readiness grades, `80_detection.cedar`), Phase 09 (`BankCalendarClock`, reason-code family `detection`), Phase 10 (`canon` tables and their batch ids), Phase 11 (`canon.touched`, `FreshnessService`, `admin.blind_periods`), Phase 12 (`admin.coverage_ledger`, `admin.exclusions`), Phase 13 (`FeatureCompiler` measure and filter compiler, `ProfileComparator`, `RatingSource` and `DeclaredProfileSource` ports, `ReaderCatalogue` port, segment memberships), Phase 15 (implements `RatingSource`; until it ships the null implementation applies, see Open doubts)
**Macro-phase:** P1
**Complexity:** XL (23 new routes and 2 extended, 11 new tables, the scenario engine and its replay and shadow stages)
**Binaries touched:** akr-api (routers `scenarios`, `replays`, `seeded`; `catalogue` and `coverage` extended; change targets `scenario`, `parameter_set` and `cbi_catalogue` registered in process; `RecordingAlertSink`; `ScenarioReaderCatalogue`) | akr-worker (the job kinds of section 4.1: `ScenarioRunner`, `ScenarioFanout`, `ReplayRunner`, `SampleBuilder`, `ShadowCompare`, `SeededSuite`, `ScenarioMetrics`, `ReaderReplayRunner`) | akr-ctl (`packs seed` and `packs verify-data` extended with the scenario document pack of section 3.15). akr-ingestd and akr-assess unchanged.

**Requirements carried:**
- S-01 (CBI baseline always on: every live `cbi` version is evaluated on every batch; default parameter sets and documents ship as signed pack data; no `cbi` scenario is retired)
- S-04 (AKR library: parameter basis tags and documents; library versions run in replay and shadow only until the feature flag `pack.akr_live` is set by change request)
- S-05 (bank namespace 901-999 through `POST /scenarios`; bank-tuned values stored as parameter-set versions)
- S-06 (catalogue importer for CBI and Office issuances: draft items with data requirements, adoption date, release notes, approval by change request)
- S-07 (the studio, the engine, the card and the matrix read the one register of phase 06)
- S-08 (aggregation across linked parties through one linked-party resolver; kinship links widen the group and are named on the alert)
- S-09 (posted events only, reversals netted, settled-time evaluation, late data amends an alert with an explanation and never silently changes it)
- S-10 (a missing declared value falls back to the peer statistic and marks the alert and customer degraded)
- S-13 (calendar-aware windows through the phase 09 business calendar; Hijri labels are display text only)
- S-38 (protected attributes refused in filters, grouping, segments and overrides)
- S-39 (parameter register: value, basis, reason code with its risk or capacity tag, approver, effective date; no parameter limits alert counts; capacity changes need below-the-line evidence)
- S-40 (every `DP` value carries `review_by`; expired values raise `W_DP_EXPIRED`)
- S-41 (replay on as-of history with alerts by segment, branch and month, reach, overlap and the look-ahead flag; analyst-hour estimates are left, see Open doubts)
- S-42 (above- and below-the-line sampling plan with strata, the rule-of-three sample size, seeded cases referenced, recorded tolerance)
- S-44 (champion and challenger: shadow of at least 30 days beside the live version, compared alert by alert; rollback point recorded)
- S-45 (scenario card generated from the document, the test results and the monitoring data; the independent replication test is phase 24, see Open doubts)
- S-47 (go-live gate: complete card, unit tests, replay, below-the-line sample within tolerance, inputs graded A or B, maker-checker, 30-day shadow)
- S-49 (known-answer seeded suite with one positive and one negative case per CBI item, run daily and on every release, with silence and flat-line alarms)
- S-50 (studio stages mapped to version statuses; plain-language text generated from the document)
- S-51 (declarative documents: JSON Schema, closed operator set, no free SQL, bilingual text)
- S-52 (logic semver, parameter-set versions, CBI pack and register versions recorded on every detection; kill switch to the previous approved version)
- S-53 (partial: the operational measures of section 3.16; outcome measures wait for alerts, cases and reports, see Open doubts)
- S-58 (history kept at least seven years through the setting floor and the retention classes of section 1)
- S-59 (evidence label required on `akr` documents; `N` labels are shown as unvalidated)
- T-02 (the CBI minimum is never retired; a kill switch falls back and never switches off)
- T-04 (currency modes separate by default; no conversion without the business-date rate)
- L-16 (no parameter caps alerts; a flat-line detector flags a scenario whose count repeats for N days)
- L-17 (suppression is merge: repeat detections fold into one open bucket and every underlying transaction is listed)
- L-18 (threshold change governance: reason code, before and after impact, below-the-line evidence, second approver outside operations)
- TS-14 (scenarios as validated documents compiled to SQL and CEL; bound parameters; statement timeouts; cost guard; replay and shadow; the catalogue importer)
- TS-15 (every scenario, parameter-set and catalogue change through a change request with replay evidence, a different approver, a fresh second factor and the volume flag)
- Gap pass 1 (coverage), partly implemented: S-25 (CBI references on every rule), S-27 (the cash family and linked-party aggregation), S-29 (restricted visibility of the public-funds family); K-20 (scenario outputs link parties by shared family number, phone, address, signatory and owner); L-13 (the run ledger with its parameter set; matched transactions on each alert), L-20 (reach by segment in replays), L-21 (segment, risk and currency overrides of parameters), L-22 (unit and seeded cases that block activation), L-23 (the scenario record and its go-live gates), L-29 (per-scenario silence alarms), L-51 (bounded limit parameters), L-61 (the dated passing-test gate for scenarios); T-16 (replay on as-of history with the look-ahead flag, precision and miss-rate estimates from samples), T-45 (the CBI coverage matrix), T-61 (above- and below-the-line sampling plans with seeded cases); F-04 (profile scenarios against declared figures, own history and peers), F-06 (aggregation across linked parties, accounts and days), F-24 (TRADE, PF and TF scenario families), B-06 (product and channel codes in scenario filters); VA-06 (rule scenarios with the one-hop network), VA-07 (replay on own history with the go-live gate), VA-27 (the bilingual scenario card with its change history), VA-28 (replay rows per entity, with near misses), VA-29 (the seeded known-answer suite), VA-35 (versioned scenario documents with the change-request diff), VA-37 (draft, replay, approve and activate in the product), VA-43 and VO-09 (the shipped image with bank parameter sets and bank scenarios), VA-45 (family-number links and the mother's name), VA-55 and VO-02 (replay on as-of history with reach and overlap); VB-07 (replay with alert counts and overlap), VB-16 (signed scenario documents with owner and typologies), VB-36 (the seeded known-answer suite), VB-45 (the per-bank replay summary), VD-16 (the CBI pack with replay and shadow).
- Gap pass 1 (coverage), not carried: S-28, S-30, S-33 (pass-through engine, front-company score, NPO scenarios: P2, DR-394); L-63 and L-66 (pass-through engine and front-company score, P2, DR-394), L-65 (typology exposure index, P2), L-40 (evasion-language search, P2, DR-397); T-39 (continuing-activity and auto-close switches; no auto-close in P1, DR-399); W-25 (PSP and MSB segment, DR-400).
- Gap pass 1 (coverage), implemented here: L-26 (client-level and linked-party aggregation through `LinkedPartyResolver`); T-01 (scenario documents compiled to parameterised SQL with incremental, sweep and replay runs), T-40 (the latency class on every scenario document; one definition run on the plan of its class); W-04 (below-the-line testing, champion and challenger, the governed scenario lifecycle); F-05 (scenarios for complex, unusual and purposeless patterns), B-02 (every parameter with its basis, bounds and bank scenarios 901 to 999), B-05 (internal audit reads the scenario card, metrics and change history), B-07 (scenarios combine CDD data, risk-level overrides and register typologies); VB-17 (the scenario studio with replay, approval and rollback), VB-23 (champion and challenger in shadow beside the live version), VB-24 (the replay of at least 12 months of history before go-live), VB-48 (scenarios as governed, versioned, tested documents).

Research basis: `research/27-transaction-monitoring-scenarios.md` B24 (tuning and testing) and B26 (catalogue, versions, studio, metrics); the governing spec is DSL.md Part A and Parts D and E. Where a shipped requirement and DSL.md differ, the difference is listed in Open doubts.

---

## 1. SQLx Migration Changes

Four migrations, `0054_scn_versions_parameters_imports` to `0057_scn_settings_schedules_classification`. Phase 15 reserved `0054` as unused, so the numbers are free. All new tables are in the existing schema `scn` (phase 06 created it). Ids are UUIDv7 generated in the application, with no database default. Eleven tables are added: `scenario_versions`, `parameter_sets`, `scenario_runs`, `replays`, `replay_results` (partitioned), `replay_samples`, `sample_labels`, `shadow_results` (partitioned), `detection_keys`, `seeded_tests` and `catalogue_imports`. The last four are the additions the brief asks for (samples, labels, the detection identity that makes a replayed batch idempotent, and the staging of CBI issuances); DATA-MODEL s.10 is aligned in the same change, with these renames: `scenario_versions.version` (semver) becomes `logic_version` and `version` becomes the optimistic-lock counter; `parameter_sets.segment` is replaced by per-parameter overrides inside `values` (section 3.5); `scenario_runs.alerts` becomes `detections` and `accounts` becomes `entities`; `scenario_runs.mode` drops `replay` (replays live in `scn.replays`); `replays.parameter_set jsonb` becomes `parameter_set_id` plus the bound snapshot `parameter_values`; `replay_results.score` and `below_the_line` become `measures` and `near_miss`.

No table of phase 06 is recreated. `scn.scenarios` takes no DDL change: `owner_id` (phase 06) is assigned by `POST /scenarios`, and its phase 06 grants already cover that write.

### 1.1 `0054_scn_versions_parameters_imports.sql`

```sql
-- migration 0054_scn_versions_parameters_imports.sql
-- Phase 16. Scenario versions (DSL.md A.10, S-52), parameter sets (DSL.md A.5, S-39, L-18) and the staging of CBI
-- catalogue issuances (S-06). Grants, guards, classification and retention are in 0056 and 0057.

DO $$ BEGIN
    CREATE TYPE scn.version_status AS ENUM ('draft', 'testing', 'shadow', 'live', 'superseded', 'retired');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS scn.scenario_versions (
    id                        UUID PRIMARY KEY,
    scenario_id               UUID NOT NULL REFERENCES scn.scenarios(id) ON DELETE RESTRICT,
    logic_version             TEXT NOT NULL CHECK (logic_version ~ '^[0-9]+\.[0-9]+\.[0-9]+$'),   -- DSL A.10
    document                  JSONB NOT NULL CHECK (jsonb_typeof(document) = 'object'),           -- DSL Part A
    document_sha256           BYTEA NOT NULL CHECK (octet_length(document_sha256) = 32),
    fixtures                  JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(fixtures) = 'object'),   -- unit fixtures: path to synthetic rows
    compiled_sql              TEXT,                           -- aggregate kind; set when validation has no errors
    compiled_program          JSONB,                          -- event: CEL source and typed context; profile: comparator plan
    compile_cost              JSONB,                          -- EXPLAIN summary: partition, total cost, estimated rows, bound
    status                    scn.version_status NOT NULL DEFAULT 'draft',
    cbi_pack_version          TEXT NOT NULL CHECK (length(cbi_pack_version) BETWEEN 1 AND 60),   -- DSL A.2
    register_version          TEXT NOT NULL CHECK (register_version ~ '^[0-9]+\.[0-9]+\.[0-9]+$'),
    tests_passed_at           TIMESTAMPTZ,                    -- set when the unit cases pass (7.17; DSL A.9)
    shadow_started_at         TIMESTAMPTZ,
    shadow_days_required      INT CHECK (shadow_days_required IS NULL OR shadow_days_required >= 30),
    change_request_id         UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,   -- the go-live request
    effective_from            TIMESTAMPTZ,
    superseded_by_version_id  UUID REFERENCES scn.scenario_versions(id) ON DELETE RESTRICT,
    superseded_at             TIMESTAMPTZ,
    retired_at                TIMESTAMPTZ,
    end_reason                TEXT CHECK (end_reason IN ('superseded', 'kill_switch', 'retired', 'withdrawn')),
    reinstated_at             TIMESTAMPTZ,                    -- set when a kill switch returns a superseded version to live
    created_by                UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at                TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at                TIMESTAMPTZ NOT NULL DEFAULT now(),
    version                   INT NOT NULL DEFAULT 1,         -- optimistic lock (If-Match)
    CONSTRAINT uq_scenario_logic_version UNIQUE (scenario_id, logic_version),
    CHECK (status <> 'live' OR effective_from IS NOT NULL),
    CHECK (status <> 'shadow' OR shadow_started_at IS NOT NULL),
    CHECK (status NOT IN ('testing', 'shadow', 'live') OR tests_passed_at IS NOT NULL),
    CHECK (status <> 'superseded' OR (superseded_at IS NOT NULL AND superseded_by_version_id IS NOT NULL)),
    CHECK (status <> 'retired' OR (retired_at IS NOT NULL AND end_reason IS NOT NULL))
);
-- At most one live version per scenario (DSL A.10, E_TWO_LIVE). Activation supersedes the old row before it promotes
-- the new one, so the index is never violated inside the transaction.
CREATE UNIQUE INDEX IF NOT EXISTS uq_scenario_one_live ON scn.scenario_versions (scenario_id) WHERE status = 'live';
-- At most one open edit (draft or testing) per scenario: a second one answers E_NOT_ALLOWED_IN_STATE (draft_open).
CREATE UNIQUE INDEX IF NOT EXISTS uq_scenario_one_open_edit ON scn.scenario_versions (scenario_id)
    WHERE status IN ('draft', 'testing');
CREATE INDEX IF NOT EXISTS idx_scenario_versions_status ON scn.scenario_versions (scenario_id, status);
CREATE INDEX IF NOT EXISTS idx_scenario_versions_cr     ON scn.scenario_versions (change_request_id)
    WHERE change_request_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_scenario_versions_super  ON scn.scenario_versions (superseded_by_version_id)
    WHERE superseded_by_version_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS scn.parameter_sets (
    id                      UUID PRIMARY KEY,
    scenario_version_id     UUID NOT NULL REFERENCES scn.scenario_versions(id) ON DELETE RESTRICT,
    set_version             INT NOT NULL CHECK (set_version >= 1),   -- the parameter-set number within the version
    -- {"<param>": {"value": <v>, "overrides": [{"when": "segment:<key>" | "risk:<1-5>" | "ccy:<ISO>", "value": <v>}]}}
    values                  JSONB NOT NULL CHECK (jsonb_typeof(values) = 'object'),
    -- {"<param>": {"basis": "CBI" | ..., "review_by": "YYYY-MM-DD" | null, "description": {"ar", "en"} | null}}
    basis                   JSONB NOT NULL CHECK (jsonb_typeof(basis) = 'object'),
    band_status             TEXT NOT NULL CHECK (band_status IN ('inside', 'outside')),   -- W_OUTSIDE_BAND, computed
    reason_code             TEXT NOT NULL REFERENCES ref.reason_codes(code) ON DELETE RESTRICT,
    reason_tag              TEXT NOT NULL CHECK (reason_tag IN ('risk', 'capacity')),
    reason_text             TEXT NOT NULL CHECK (length(btrim(reason_text)) BETWEEN 1 AND 2000),
    status                  TEXT NOT NULL CHECK (status IN ('draft', 'proposed', 'active', 'superseded', 'rejected')),
    change_request_id       UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    effective_from          TIMESTAMPTZ,
    superseded_at           TIMESTAMPTZ,
    created_by              UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    version                 INT NOT NULL DEFAULT 1,
    CONSTRAINT uq_parameter_set_version UNIQUE (scenario_version_id, set_version),
    CHECK (status <> 'active' OR effective_from IS NOT NULL),
    CHECK (status <> 'superseded' OR superseded_at IS NOT NULL)
);
-- One active parameter set per scenario version (the set the engine binds).
CREATE UNIQUE INDEX IF NOT EXISTS uq_parameter_set_active ON scn.parameter_sets (scenario_version_id)
    WHERE status = 'active';
-- One proposed set per version: the set a shadow run uses (section 3.11) and the set the change request approves.
CREATE UNIQUE INDEX IF NOT EXISTS uq_parameter_set_proposed ON scn.parameter_sets (scenario_version_id)
    WHERE status = 'proposed';
CREATE INDEX IF NOT EXISTS idx_parameter_sets_cr      ON scn.parameter_sets (change_request_id)
    WHERE change_request_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_parameter_sets_created ON scn.parameter_sets (created_by);
CREATE INDEX IF NOT EXISTS idx_parameter_sets_reason  ON scn.parameter_sets (reason_code);

-- A CBI or Office issuance staged as draft items (S-06). Applied by change request kind cbi_catalogue (section 3.15).
CREATE TABLE IF NOT EXISTS scn.catalogue_imports (
    id                    UUID PRIMARY KEY,
    catalogue_version     TEXT NOT NULL CHECK (catalogue_version ~ '^[0-9]{4}-[0-9]{2}-[0-9]{2}(\.[0-9]+)?$'),
    issuance_ref          TEXT NOT NULL CHECK (length(issuance_ref) BETWEEN 3 AND 200),
    evidence_object_id    UUID NOT NULL REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,   -- the issuance itself
    adoption_date         DATE NOT NULL,                       -- the bank's adoption date (S-06)
    release_notes         JSONB NOT NULL CHECK (release_notes ? 'ar' AND release_notes ? 'en'),
    items                 JSONB NOT NULL CHECK (jsonb_typeof(items) = 'array'),   -- [{ref, article, number, kind, title, text, data_requirements, counter_only}]
    item_count            INT NOT NULL CHECK (item_count BETWEEN 1 AND 500),
    status                TEXT NOT NULL CHECK (status IN ('draft', 'proposed', 'applied', 'rejected', 'withdrawn')),
    change_request_id     UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    created_by            UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    applied_at            TIMESTAMPTZ,
    version               INT NOT NULL DEFAULT 1,
    CHECK (status <> 'applied' OR applied_at IS NOT NULL),
    CHECK (status IN ('draft', 'withdrawn') OR change_request_id IS NOT NULL)
);
-- One open import per catalogue version; a rejected or withdrawn one may be replaced.
CREATE UNIQUE INDEX IF NOT EXISTS uq_catalogue_import_open ON scn.catalogue_imports (catalogue_version)
    WHERE status IN ('draft', 'proposed', 'applied');
CREATE INDEX IF NOT EXISTS idx_catalogue_imports_created ON scn.catalogue_imports (created_by);
CREATE INDEX IF NOT EXISTS idx_catalogue_imports_evidence ON scn.catalogue_imports (evidence_object_id);
```

### 1.2 `0055_scn_runs_replays_shadow.sql`

```sql
-- migration 0055_scn_runs_replays_shadow.sql
-- Phase 16. Run ledger, replays and their results, sampling plans and blind labels, shadow results, the detection
-- identity table and the seeded known-answer cases (DSL A.9, A.11; S-41, S-42, S-44, S-49). Two partitioned tables
-- with the partition factory of section 1.2 (the phase 13 feat.create_partition pattern).

CREATE TABLE IF NOT EXISTS scn.scenario_runs (
    id                      UUID PRIMARY KEY,
    scenario_version_id     UUID NOT NULL REFERENCES scn.scenario_versions(id) ON DELETE RESTRICT,
    mode                    TEXT NOT NULL CHECK (mode IN ('incremental', 'settled', 'sweep', 'shadow', 'seeded')),
    batch_id                UUID,                                   -- canon.touched batch (incremental, settled, shadow)
    touch_id                UUID,                                   -- the touch of that batch which started the run (7.9)
    trigger_ref             TEXT NOT NULL CHECK (length(trigger_ref) BETWEEN 1 AND 60),   -- 'batch', 'schedule', 'manual', 'kill_switch'
    window_from             DATE NOT NULL,
    window_to               DATE NOT NULL,
    parameter_set_id        UUID REFERENCES scn.parameter_sets(id) ON DELETE RESTRICT,
    entities                INT NOT NULL DEFAULT 0 CHECK (entities >= 0),
    detections              INT NOT NULL DEFAULT 0 CHECK (detections >= 0),
    amendments              INT NOT NULL DEFAULT 0 CHECK (amendments >= 0),
    coverage                JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(coverage) = 'object'),   -- section 3.8
    cost                    JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(cost) = 'object'),
    status                  TEXT NOT NULL CHECK (status IN ('queued', 'running', 'completed', 'failed', 'refused_cost')),
    error_code              TEXT,
    started_at              TIMESTAMPTZ,
    finished_at             TIMESTAMPTZ,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    version                 INT NOT NULL DEFAULT 1,
    CHECK (window_to >= window_from),
    CHECK (batch_id IS NULL OR mode IN ('incremental', 'settled', 'shadow')),
    CHECK (touch_id IS NULL OR batch_id IS NOT NULL),
    CHECK (status NOT IN ('completed', 'failed', 'refused_cost') OR finished_at IS NOT NULL)
);
-- A touch of a batch is run once per version and mode (7.9): a second delivery of the same canon.touched event finds the row and stops
-- (section 3.6, run step 2). This is the run-level half of the idempotency; the detection key is the emission-level half.
CREATE UNIQUE INDEX IF NOT EXISTS uq_scenario_runs_batch ON scn.scenario_runs (scenario_version_id, mode, touch_id)
    WHERE touch_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_scenario_runs_version   ON scn.scenario_runs (scenario_version_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_scenario_runs_pset      ON scn.scenario_runs (parameter_set_id)
    WHERE parameter_set_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_scenario_runs_status    ON scn.scenario_runs (status, created_at)
    WHERE status IN ('queued', 'running');

CREATE TABLE IF NOT EXISTS scn.replays (
    id                      UUID PRIMARY KEY,
    scenario_version_id     UUID NOT NULL REFERENCES scn.scenario_versions(id) ON DELETE RESTRICT,
    parameter_set_id        UUID REFERENCES scn.parameter_sets(id) ON DELETE RESTRICT,   -- NULL: the document defaults
    parameter_values        JSONB NOT NULL CHECK (jsonb_typeof(parameter_values) = 'object'),   -- the bound snapshot
    baseline_version_id     UUID REFERENCES scn.scenario_versions(id) ON DELETE RESTRICT,   -- the live version compared
    proposal_hash           BYTEA CHECK (proposal_hash IS NULL OR octet_length(proposal_hash) = 32),   -- section 3.14
    period_from             DATE NOT NULL,
    period_to               DATE NOT NULL,
    estimate                JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(estimate) = 'object'),
    status                  TEXT NOT NULL CHECK (status IN ('queued', 'running', 'completed', 'failed')),
    cursor                  JSONB,                                  -- keyset position, committed with each chunk
    evaluated               BIGINT NOT NULL DEFAULT 0 CHECK (evaluated >= 0),
    would_alert_count       BIGINT NOT NULL DEFAULT 0 CHECK (would_alert_count >= 0),
    near_miss_count         BIGINT NOT NULL DEFAULT 0 CHECK (near_miss_count >= 0),
    baseline_alert_count    BIGINT CHECK (baseline_alert_count IS NULL OR baseline_alert_count >= 0),
    alert_delta             BIGINT,
    effect                  TEXT CHECK (effect IN ('increase', 'neutral', 'decrease')),   -- NULL without a baseline
    look_ahead              BOOLEAN NOT NULL DEFAULT FALSE,
    result_summary          JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(result_summary) = 'object'),   -- counts only
    requested_by            UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    started_at              TIMESTAMPTZ,
    finished_at             TIMESTAMPTZ,
    version                 INT NOT NULL DEFAULT 1,
    CHECK (period_to >= period_from),
    CHECK (status <> 'completed' OR finished_at IS NOT NULL),
    CHECK ((effect IS NULL) = (alert_delta IS NULL))
);
CREATE INDEX IF NOT EXISTS idx_replays_version    ON scn.replays (scenario_version_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_replays_proposal   ON scn.replays (proposal_hash, status) WHERE proposal_hash IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_replays_pset       ON scn.replays (parameter_set_id) WHERE parameter_set_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_replays_baseline   ON scn.replays (baseline_version_id) WHERE baseline_version_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_replays_requested  ON scn.replays (requested_by);

-- Per-entity rows of a replay: alert rows and near-miss rows only (the rest is counted in replays). Partitioned by the
-- month the row was written; partitions come from scn.create_partition.
CREATE TABLE IF NOT EXISTS scn.replay_results (
    id                  UUID NOT NULL,
    replay_id           UUID NOT NULL REFERENCES scn.replays(id) ON DELETE RESTRICT,
    entity_kind         TEXT NOT NULL CHECK (entity_kind IN ('account', 'customer', 'counterparty', 'branch', 'staff')),
    entity_id           UUID NOT NULL,                           -- masked at read (section 3.9), never at write
    business_date       DATE NOT NULL,
    bucket              TEXT NOT NULL CHECK (length(bucket) BETWEEN 4 AND 60),
    would_alert         BOOLEAN NOT NULL,
    near_miss           BOOLEAN NOT NULL DEFAULT FALSE,
    segment_ref         TEXT CHECK (segment_ref IS NULL OR segment_ref ~ '^(segment|risk|ccy):'),
    branch_id           UUID,
    measures            JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(measures) = 'object'),
    detection_key       TEXT CHECK (detection_key IS NULL OR detection_key ~ '^[0-9a-f]{64}$'),
    coverage_state      TEXT NOT NULL DEFAULT 'monitored' CHECK (coverage_state IN ('monitored', 'degraded', 'dark')),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (id, created_at),
    CHECK (would_alert = (detection_key IS NOT NULL)),
    CHECK (NOT (near_miss AND would_alert))
) PARTITION BY RANGE (created_at);
CREATE INDEX IF NOT EXISTS idx_replay_results_replay ON scn.replay_results (replay_id, would_alert, near_miss);
CREATE INDEX IF NOT EXISTS idx_replay_results_keyset ON scn.replay_results (replay_id, id);

-- The sampling plan of a replay (S-42). One row per drawn item; stratum fields repeat for every item of a stratum.
CREATE TABLE IF NOT EXISTS scn.replay_samples (
    id                  UUID PRIMARY KEY,
    replay_id           UUID NOT NULL REFERENCES scn.replays(id) ON DELETE RESTRICT,
    line                TEXT NOT NULL CHECK (line IN ('above', 'below')),   -- above: would alert; below: near miss
    stratum             JSONB NOT NULL CHECK (jsonb_typeof(stratum) = 'object'),   -- {"segment", "branch", "month"}
    stratum_population  BIGINT NOT NULL CHECK (stratum_population >= 1),
    stratum_sampled     INT NOT NULL CHECK (stratum_sampled >= 1),
    plan_seed           BYTEA NOT NULL CHECK (octet_length(plan_seed) = 32),   -- reproduces the draw
    replay_result_id    UUID NOT NULL,                           -- replay_results.id (no foreign key: partitioned)
    entity_id           UUID NOT NULL,                           -- masked at read (section 3.10 step 7)
    packet_object_id    UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,   -- the blind packet
    status              TEXT NOT NULL CHECK (status IN ('open', 'labelled', 'withdrawn')),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_replay_sample_result UNIQUE (replay_id, replay_result_id),
    CHECK (stratum_sampled <= stratum_population)
);
CREATE INDEX IF NOT EXISTS idx_replay_samples_replay ON scn.replay_samples (replay_id, line);
CREATE INDEX IF NOT EXISTS idx_replay_samples_packet ON scn.replay_samples (packet_object_id)
    WHERE packet_object_id IS NOT NULL;

-- Blind labels. Append-only: one label per sample and labeller; a correction is a second label with a note code.
CREATE TABLE IF NOT EXISTS scn.sample_labels (
    id              UUID PRIMARY KEY,
    sample_id       UUID NOT NULL REFERENCES scn.replay_samples(id) ON DELETE RESTRICT,
    labeller_id     UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    label           TEXT NOT NULL CHECK (label IN ('suspicious', 'not_suspicious', 'cannot_tell')),
    note_code       TEXT CHECK (note_code IN ('missing_data', 'ambiguous_pattern', 'known_activity', 'other')),
    labelled_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_sample_label_per_labeller UNIQUE (sample_id, labeller_id)
);
CREATE INDEX IF NOT EXISTS idx_sample_labels_labeller ON scn.sample_labels (labeller_id);

-- Shadow comparison rows (S-44). Partitioned by the month written. live_alert_id is written by phase 17 when the live
-- alert exists; there is no foreign key here (alerts are phase 17's table).
CREATE TABLE IF NOT EXISTS scn.shadow_results (
    id                      UUID NOT NULL,
    scenario_version_id     UUID NOT NULL REFERENCES scn.scenario_versions(id) ON DELETE RESTRICT,
    run_id                  UUID NOT NULL,                       -- scn.scenario_runs.id (no foreign key: partitioned)
    entity_kind             TEXT NOT NULL CHECK (entity_kind IN ('account', 'customer', 'counterparty', 'branch', 'staff')),
    entity_id               UUID NOT NULL,
    business_date           DATE NOT NULL,
    bucket                  TEXT NOT NULL CHECK (length(bucket) BETWEEN 4 AND 60),
    would_alert             BOOLEAN NOT NULL,
    detection_key           TEXT CHECK (detection_key IS NULL OR detection_key ~ '^[0-9a-f]{64}$'),
    outcome                 TEXT CHECK (outcome IN ('agree_alert', 'agree_quiet', 'shadow_only', 'live_only')),
    live_alert_id           UUID,
    compared_at             TIMESTAMPTZ,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (id, created_at),
    CHECK (NOT would_alert OR detection_key IS NOT NULL)
) PARTITION BY RANGE (created_at);
CREATE INDEX IF NOT EXISTS idx_shadow_results_version_day ON scn.shadow_results (scenario_version_id, business_date);
CREATE INDEX IF NOT EXISTS idx_shadow_results_key         ON scn.shadow_results (detection_key)
    WHERE detection_key IS NOT NULL;

-- One row per detection identity (section 3.7). The sink of phase 16 (RecordingAlertSink) inserts here; phase 17 may keep
-- or migrate it. anchor_hk is the keyed hash of the group anchor, the key of the bucket fold.
CREATE TABLE IF NOT EXISTS scn.detection_keys (
    fingerprint         TEXT PRIMARY KEY CHECK (fingerprint ~ '^[0-9a-f]{64}$'),
    scenario_id         UUID NOT NULL REFERENCES scn.scenarios(id) ON DELETE RESTRICT,
    entity_kind         TEXT NOT NULL CHECK (entity_kind IN ('account', 'customer', 'counterparty', 'branch', 'staff')),
    entity_id           UUID NOT NULL,
    anchor_hk           BYTEA NOT NULL CHECK (octet_length(anchor_hk) = 32),
    bucket              TEXT NOT NULL CHECK (length(bucket) BETWEEN 4 AND 60),   -- ISO start date, or 'event:<txn id>'
    bucket_start        DATE,                                    -- NULL for event buckets
    first_version_id    UUID NOT NULL REFERENCES scn.scenario_versions(id) ON DELETE RESTRICT,
    last_version_id     UUID NOT NULL REFERENCES scn.scenario_versions(id) ON DELETE RESTRICT,
    measures_digest     BYTEA NOT NULL CHECK (octet_length(measures_digest) = 32),
    emitted_count       INT NOT NULL DEFAULT 1 CHECK (emitted_count >= 1),
    matches             BOOLEAN NOT NULL DEFAULT TRUE,           -- FALSE after a late change removes the pattern
    first_seen_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_seen_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK ((bucket_start IS NULL) = (bucket LIKE 'event:%'))
);
CREATE INDEX IF NOT EXISTS idx_detection_keys_anchor ON scn.detection_keys (scenario_id, anchor_hk, bucket_start)
    WHERE bucket_start IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_detection_keys_version ON scn.detection_keys (last_version_id);

-- Known-answer cases (S-49). One row per CBI item case; fixtures are synthetic canonical rows, never customer data.
CREATE TABLE IF NOT EXISTS scn.seeded_tests (
    id                      UUID PRIMARY KEY,
    catalogue_item_id       UUID NOT NULL REFERENCES scn.cbi_catalogue_items(id) ON DELETE RESTRICT,
    scenario_id             UUID NOT NULL REFERENCES scn.scenarios(id) ON DELETE RESTRICT,
    case_code               TEXT NOT NULL CHECK (case_code ~ '^SEED-CBI-A([1-9]|1[0-4])-[0-9]{2}-[PN]$'),   -- DSL A.9
    polarity                TEXT NOT NULL CHECK (polarity IN ('positive', 'negative')),
    fixture                 JSONB NOT NULL CHECK (jsonb_typeof(fixture) = 'object'),
    expected                JSONB NOT NULL CHECK (jsonb_typeof(expected) = 'object'),   -- {"alerts": 1, "severity": "medium"}
    active                  BOOLEAN NOT NULL DEFAULT TRUE,
    last_run_id             UUID REFERENCES scn.scenario_runs(id) ON DELETE RESTRICT,
    last_run_version_id     UUID REFERENCES scn.scenario_versions(id) ON DELETE RESTRICT,   -- the gate reads this
    last_run_at             TIMESTAMPTZ,
    last_result             TEXT NOT NULL DEFAULT 'not_run' CHECK (last_result IN ('pass', 'fail', 'error', 'not_run')),
    consecutive_failures    INT NOT NULL DEFAULT 0 CHECK (consecutive_failures >= 0),
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    version                 INT NOT NULL DEFAULT 1,
    CONSTRAINT uq_seeded_case UNIQUE (scenario_id, case_code),
    CHECK ((polarity = 'positive') = (case_code LIKE '%-P')),
    CHECK (last_run_id IS NULL OR last_run_version_id IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_seeded_tests_item ON scn.seeded_tests (catalogue_item_id);

-- The partition factory for the two partitioned tables. Creates one month of one allow-listed table. There is no DEFAULT
-- partition: a missing month fails loudly and the writer calls the factory once and retries (phase 13 pattern).
CREATE OR REPLACE FUNCTION scn.create_partition(p_table TEXT, p_year INT, p_month INT) RETURNS TEXT
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog
AS $$
DECLARE
    v_from TIMESTAMPTZ;
    v_to   TIMESTAMPTZ;
    v_name TEXT;
BEGIN
    IF p_table NOT IN ('replay_results', 'shadow_results') THEN
        RAISE EXCEPTION 'scn.create_partition: % is not a partitioned scn table', p_table;
    END IF;
    IF p_year NOT BETWEEN 2000 AND 2200 OR p_month NOT BETWEEN 1 AND 12 THEN
        RAISE EXCEPTION 'scn.create_partition: month %-% out of range', p_year, p_month;
    END IF;
    PERFORM pg_catalog.pg_advisory_xact_lock(7307418562110395100);   -- fixed key: 'akr.scn.partition_factory'
    v_from := pg_catalog.make_timestamptz(p_year, p_month, 1, 0, 0, 0, 'UTC');
    v_to   := v_from + interval '1 month';
    v_name := pg_catalog.format('%s_%s_%s', p_table, p_year, pg_catalog.lpad(p_month::text, 2, '0'));
    IF NOT EXISTS (SELECT 1 FROM pg_catalog.pg_class c
                   JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace
                   WHERE n.nspname = 'scn' AND c.relname = v_name) THEN
        EXECUTE pg_catalog.format('CREATE TABLE scn.%I PARTITION OF scn.%I FOR VALUES FROM (%L) TO (%L)',
                                  v_name, p_table, v_from, v_to);
    END IF;
    RETURN v_name;
END $$;
REVOKE ALL ON FUNCTION scn.create_partition(TEXT, INT, INT) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION scn.create_partition(TEXT, INT, INT) TO akr_app, akr_migrate;   -- migrate runs the seed below

-- Seed partitions: the current month and the three ahead (audit.partition_ahead_months, default 3), through the factory.
DO $$
DECLARE
    v_t TEXT;
    v_m DATE;
    i   INT;
BEGIN
    FOREACH v_t IN ARRAY ARRAY['replay_results', 'shadow_results'] LOOP
        FOR i IN 0..3 LOOP
            v_m := (date_trunc('month', now()) + make_interval(months => i))::date;
            PERFORM scn.create_partition(v_t, EXTRACT(YEAR FROM v_m)::int, EXTRACT(MONTH FROM v_m)::int);
        END LOOP;
    END LOOP;
END $$;
```

### 1.3 `0056_scn_guards_grants_indexes.sql`

```sql
-- migration 0056_scn_guards_grants_indexes.sql
-- Phase 16. Guards (the second layer under the grants), grants per write path, and the two indexes the fan-out needs.

-- Versions: a document is immutable once the version leaves draft; statuses move only along the DSL A.10 lifecycle;
-- a superseded version returns to live only by a kill switch (reinstated_at set); nothing is deleted.
CREATE OR REPLACE FUNCTION scn.version_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'scenario versions are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.status <> 'draft' AND (NEW.document IS DISTINCT FROM OLD.document
                                  OR NEW.document_sha256 IS DISTINCT FROM OLD.document_sha256
                                  OR NEW.fixtures IS DISTINCT FROM OLD.fixtures) THEN
        RAISE EXCEPTION 'the document of a % version is immutable', OLD.status USING ERRCODE = 'check_violation';
    END IF;
    IF NEW.status IS DISTINCT FROM OLD.status THEN
        IF NOT ((OLD.status, NEW.status) IN (
                ('draft', 'testing'), ('testing', 'draft'), ('testing', 'shadow'),
                ('shadow', 'live'), ('draft', 'retired'), ('testing', 'retired'), ('shadow', 'retired'),
                ('live', 'superseded'), ('live', 'retired'), ('superseded', 'live'), ('superseded', 'retired'))) THEN
            RAISE EXCEPTION 'scenario version status % to % is not allowed', OLD.status, NEW.status
                USING ERRCODE = 'check_violation';
        END IF;
        IF OLD.status = 'superseded' AND NEW.status = 'live'
           AND (NEW.reinstated_at IS NULL OR NEW.reinstated_at IS NOT DISTINCT FROM OLD.reinstated_at) THEN
            RAISE EXCEPTION 'a superseded version returns to live only through a kill switch'
                USING ERRCODE = 'check_violation';
        END IF;
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_scenario_versions_guard ON scn.scenario_versions;
CREATE TRIGGER trg_scenario_versions_guard BEFORE UPDATE OR DELETE ON scn.scenario_versions
    FOR EACH ROW EXECUTE FUNCTION scn.version_guard();

-- Parameter sets: values, basis and reason are immutable once proposed; statuses move draft, proposed, active,
-- superseded (with rejected and draft as the side exits of a proposal); nothing is deleted.
CREATE OR REPLACE FUNCTION scn.parameter_set_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'parameter sets are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.status <> 'draft' AND (NEW.values IS DISTINCT FROM OLD.values
                                  OR NEW.basis IS DISTINCT FROM OLD.basis
                                  OR NEW.band_status IS DISTINCT FROM OLD.band_status
                                  OR NEW.reason_code IS DISTINCT FROM OLD.reason_code
                                  OR NEW.reason_text IS DISTINCT FROM OLD.reason_text) THEN
        RAISE EXCEPTION 'a parameter set is immutable once proposed' USING ERRCODE = 'check_violation';
    END IF;
    IF NEW.status IS DISTINCT FROM OLD.status AND NOT ((OLD.status, NEW.status) IN (
                ('draft', 'proposed'), ('draft', 'active'), ('proposed', 'active'), ('proposed', 'rejected'),
                ('proposed', 'draft'), ('active', 'superseded'), ('superseded', 'active'))) THEN
        RAISE EXCEPTION 'parameter set status % to % is not allowed', OLD.status, NEW.status
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_parameter_sets_guard ON scn.parameter_sets;
CREATE TRIGGER trg_parameter_sets_guard BEFORE UPDATE OR DELETE ON scn.parameter_sets
    FOR EACH ROW EXECUTE FUNCTION scn.parameter_set_guard();

-- Evidence tables: no DELETE on any table of this migration; no UPDATE on the two result tables that carry judgement.
CREATE OR REPLACE FUNCTION scn.append_only_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION '% on scn.% is refused: the table is append-only', TG_OP, TG_TABLE_NAME
        USING ERRCODE = 'insufficient_privilege';
END $$;
DROP TRIGGER IF EXISTS trg_scenario_runs_nodelete   ON scn.scenario_runs;
DROP TRIGGER IF EXISTS trg_replays_nodelete         ON scn.replays;
DROP TRIGGER IF EXISTS trg_replay_results_guard     ON scn.replay_results;
DROP TRIGGER IF EXISTS trg_replay_samples_nodelete  ON scn.replay_samples;
DROP TRIGGER IF EXISTS trg_sample_labels_guard      ON scn.sample_labels;
DROP TRIGGER IF EXISTS trg_shadow_results_nodelete  ON scn.shadow_results;
DROP TRIGGER IF EXISTS trg_detection_keys_nodelete  ON scn.detection_keys;
DROP TRIGGER IF EXISTS trg_seeded_tests_nodelete    ON scn.seeded_tests;
DROP TRIGGER IF EXISTS trg_catalogue_imports_nodelete ON scn.catalogue_imports;
CREATE TRIGGER trg_scenario_runs_nodelete   BEFORE DELETE ON scn.scenario_runs
    FOR EACH ROW EXECUTE FUNCTION scn.append_only_guard();
CREATE TRIGGER trg_replays_nodelete         BEFORE DELETE ON scn.replays
    FOR EACH ROW EXECUTE FUNCTION scn.append_only_guard();
CREATE TRIGGER trg_replay_results_guard     BEFORE UPDATE OR DELETE ON scn.replay_results
    FOR EACH ROW EXECUTE FUNCTION scn.append_only_guard();
CREATE TRIGGER trg_replay_samples_nodelete  BEFORE DELETE ON scn.replay_samples
    FOR EACH ROW EXECUTE FUNCTION scn.append_only_guard();
CREATE TRIGGER trg_sample_labels_guard      BEFORE UPDATE OR DELETE ON scn.sample_labels
    FOR EACH ROW EXECUTE FUNCTION scn.append_only_guard();
CREATE TRIGGER trg_shadow_results_nodelete  BEFORE DELETE ON scn.shadow_results
    FOR EACH ROW EXECUTE FUNCTION scn.append_only_guard();
CREATE TRIGGER trg_detection_keys_nodelete  BEFORE DELETE ON scn.detection_keys
    FOR EACH ROW EXECUTE FUNCTION scn.append_only_guard();
CREATE TRIGGER trg_seeded_tests_nodelete    BEFORE DELETE ON scn.seeded_tests
    FOR EACH ROW EXECUTE FUNCTION scn.append_only_guard();
CREATE TRIGGER trg_catalogue_imports_nodelete BEFORE DELETE ON scn.catalogue_imports
    FOR EACH ROW EXECUTE FUNCTION scn.append_only_guard();

-- The fan-out (section 3.6, fan-out step 2) finds the touched accounts and customers of a batch by batch id. Phase 10 has no
-- index on it; one index per table is added here (a physical index, no column or grant change to phase 10).
CREATE INDEX IF NOT EXISTS idx_canon_txn_batch       ON canon.txn (batch_id);
CREATE INDEX IF NOT EXISTS idx_canon_txn_event_batch ON canon.txn_event (batch_id);

-- Grants per write path (DATA-MODEL conventions). akr_app is the only writer: the API and the worker.
GRANT SELECT, INSERT ON scn.scenario_versions TO akr_app;
GRANT UPDATE (status, document, document_sha256, fixtures, compiled_sql, compiled_program, compile_cost,
              tests_passed_at, shadow_started_at, shadow_days_required, change_request_id, effective_from,
              superseded_by_version_id, superseded_at, retired_at, end_reason, reinstated_at, updated_at, version)
    ON scn.scenario_versions TO akr_app;
GRANT SELECT, INSERT ON scn.parameter_sets TO akr_app;
GRANT UPDATE (status, change_request_id, effective_from, superseded_at, version) ON scn.parameter_sets TO akr_app;
GRANT SELECT, INSERT ON scn.catalogue_imports TO akr_app;
GRANT UPDATE (status, change_request_id, applied_at, version) ON scn.catalogue_imports TO akr_app;
GRANT SELECT, INSERT ON scn.scenario_runs TO akr_app;
GRANT UPDATE (status, entities, detections, amendments, coverage, cost, error_code, started_at, finished_at, version)
    ON scn.scenario_runs TO akr_app;
GRANT SELECT, INSERT ON scn.replays TO akr_app;
GRANT UPDATE (status, cursor, evaluated, would_alert_count, near_miss_count, baseline_alert_count, alert_delta, effect,
              look_ahead, result_summary, estimate, started_at, finished_at, version) ON scn.replays TO akr_app;
GRANT SELECT, INSERT ON scn.replay_results TO akr_app;           -- no UPDATE, no DELETE (guard)
GRANT SELECT, INSERT ON scn.replay_samples TO akr_app;
GRANT UPDATE (status, packet_object_id) ON scn.replay_samples TO akr_app;
GRANT SELECT, INSERT ON scn.sample_labels TO akr_app;            -- append only (guard)
GRANT SELECT, INSERT ON scn.shadow_results TO akr_app;
GRANT UPDATE (outcome, live_alert_id, compared_at) ON scn.shadow_results TO akr_app;
GRANT SELECT, INSERT ON scn.detection_keys TO akr_app;
GRANT UPDATE (last_version_id, measures_digest, emitted_count, matches, last_seen_at) ON scn.detection_keys TO akr_app;
GRANT SELECT, INSERT ON scn.seeded_tests TO akr_app;
GRANT UPDATE (active, last_run_id, last_run_version_id, last_run_at, last_result, consecutive_failures, version)
    ON scn.seeded_tests TO akr_app;
-- Partition factory (section 1.2) is the only DDL akr_app runs.
-- Read-only reports: the register, versions, parameter sets, runs, replays and seeded cases. The per-entity tables
-- (replay_results, replay_samples, sample_labels, shadow_results, detection_keys) are not granted to akr_readonly:
-- they are customer-level, as in phase 13 for feat.feature_values.
GRANT SELECT ON scn.scenario_versions, scn.parameter_sets, scn.catalogue_imports, scn.scenario_runs, scn.replays,
                scn.seeded_tests TO akr_readonly;
```

### 1.4 `0057_scn_settings_schedules_classification.sql`

```sql
-- migration 0057_scn_settings_schedules_classification.sql
-- Phase 16. Settings and schedules (phase 00 keys; floors only where governed = TRUE), the feature flag of the AKR
-- library, column classification and retention for the eleven tables (DATA-MODEL conventions).

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('scenario.history_years', '7', 'integer', '{"min": 7, "basis": "S-58: scenario history kept at least seven years"}', TRUE,
  '{"ar": "أقل مدة للاحتفاظ بتاريخ السيناريوهات (سنوات)", "en": "Years scenario history is kept (at least seven)"}'),
 ('scenario.shadow_min_days', '30', 'integer', '{"min": 30, "basis": "DSL A.10 and S-47: at least 30 days in shadow"}', TRUE,
  '{"ar": "أقل مدة للتشغيل الظلي قبل الاعتماد (أيام)", "en": "Shortest shadow run before go-live (days)"}'),
 ('scenario.replay_min_months', '12', 'integer', '{"min": 12, "basis": "S-47: go-live replay over a year of history where the history exists"}', TRUE,
  '{"ar": "أقل مدة إعادة تشغيل قبل الاعتماد (أشهر)", "en": "Shortest replay period before go-live (months)"}'),
 ('scenario.btl_tolerance_pct', '1.0', 'decimal', '{"max": 5.0, "basis": "S-42: accepted miss rate per scenario class; the outer bound of a sample"}', TRUE,
  '{"ar": "سماح نسبة الإغفال في العينة دون الخط (%)", "en": "Accepted miss rate in the below-the-line sample (per cent)"}'),
 ('scenario.near_miss_ratio', '0.8', 'decimal', '{"min": 0.5, "basis": "S-42: below-the-line band is the share of the threshold a non-alert must reach"}', TRUE,
  '{"ar": "نسبة العتبة التي تُعد قريبة من التنبيه", "en": "Share of the threshold that makes a non-alert a near miss"}'),
 ('scenario.replay_max_months', '60', 'integer', NULL, FALSE,
  '{"ar": "أقصى مدة لإعادة التشغيل (أشهر)", "en": "Longest replay period (months)"}'),
 ('scenario.hot_window_days', '400', 'integer', NULL, FALSE,
  '{"ar": "نافذة البيانات الساخنة في المخزن المعياري (أيام)", "en": "Hot window of the canonical store (days)"}'),
 ('scenario.cost_max_total', '5000000', 'integer', NULL, FALSE,
  '{"ar": "أقصى تكلفة مقدرة لاستعلام سيناريو", "en": "Cost-guard bound: planner total cost of one scenario statement"}'),
 ('scenario.cost_max_rows', '20000000', 'integer', NULL, FALSE,
  '{"ar": "أقصى عدد صفوف مقدر لاستعلام سيناريو", "en": "Cost-guard bound: estimated rows of one scenario statement"}'),
 ('scenario.replay_cost_max', '50000000', 'integer', NULL, FALSE,
  '{"ar": "أقصى تكلفة مقدرة لإعادة تشغيل", "en": "Cost-guard bound: estimated rows of one replay"}'),
 ('scenario.statement_timeout_s', '60', 'integer', NULL, FALSE,
  '{"ar": "مهلة استعلام السيناريو (ثوان)", "en": "Statement timeout of a scenario query (seconds)"}'),
 ('scenario.summary_min_cell', '5', 'integer', NULL, FALSE,
  '{"ar": "أصغر عدد يظهر في ملخص الإعادة", "en": "Smallest count shown in a replay or shadow summary cell"}'),
 ('scenario.flatline_days', '10', 'integer', NULL, FALSE,
  '{"ar": "أيام التكرار المتطابق قبل إنذار الخط المسطح", "en": "Days of identical counts before a flat-line alarm"}'),
 ('scenario.silence_business_days', '5', 'integer', NULL, FALSE,
  '{"ar": "أيام عمل بلا تنبيه قبل إنذار الصمت", "en": "Business days with no detection before a silence alarm"}'),
 ('scenario.sample_min_per_stratum', '50', 'integer', NULL, FALSE,
  '{"ar": "أقل حجم عينة لكل طبقة", "en": "Smallest sample per stratum"}')
ON CONFLICT (key) DO NOTHING;

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('schedule.scenario_sweep_daily',   '"0 30 3 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول المسح اليومي للسيناريوهات (L2)", "en": "Daily scenario sweep (latency class L2)"}'),
 ('schedule.scenario_sweep_monthly', '"0 0 4 1 * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول المسح الشهري للسيناريوهات (L3)", "en": "Monthly scenario sweep (latency class L3)"}'),
 ('schedule.shadow_compare',         '"0 45 5 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مقارنة التشغيل الظلي", "en": "Shadow comparison schedule"}'),
 ('schedule.seeded_suite_daily',     '"0 15 4 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول الاختبارات المعروفة النتائج", "en": "Seeded known-answer suite schedule"}'),
 ('schedule.scenario_metrics',       '"0 30 6 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مقاييس السيناريوهات والإنذارات", "en": "Scenario metrics and alarm schedule"}')
ON CONFLICT (key) DO NOTHING;

-- The AKR library extension stays in replay and shadow until the bank agrees with the Office (M11, S-04).
INSERT INTO admin.feature_flags (key, enabled, label) VALUES
 ('pack.akr_live', FALSE, '{"ar": "مكتبة AKR فعّالة للإنتاج", "en": "AKR library scenarios may go live (not replay and shadow only)"}')
ON CONFLICT (key) DO NOTHING;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'scn', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('scenario_versions', 'detection-logic', ARRAY['document','document_sha256','fixtures','compiled_sql',
                                                 'compiled_program','compile_cost']),
  ('scenario_versions', 'internal', ARRAY['id','scenario_id','logic_version','status','cbi_pack_version',
        'register_version','tests_passed_at','shadow_started_at','shadow_days_required','change_request_id',
        'effective_from','superseded_by_version_id','superseded_at','retired_at','end_reason','reinstated_at',
        'created_by','created_at','updated_at','version']),
  ('parameter_sets', 'detection-logic', ARRAY['values','basis','band_status']),
  ('parameter_sets', 'internal', ARRAY['id','scenario_version_id','set_version','version','reason_code','reason_tag','reason_text',
        'status','change_request_id','effective_from','superseded_at','created_by','created_at']),
  ('scenario_runs', 'detection-logic', ARRAY['coverage']),
  ('scenario_runs', 'internal', ARRAY['id','scenario_version_id','mode','batch_id','touch_id','trigger_ref','window_from',
        'window_to','parameter_set_id','entities','detections','amendments','cost','status','error_code',
        'started_at','finished_at','created_at','version']),
  ('replays', 'detection-logic', ARRAY['parameter_values','proposal_hash','baseline_version_id','evaluated',
        'would_alert_count','near_miss_count','baseline_alert_count','alert_delta','effect','look_ahead',
        'result_summary','cursor']),
  ('replays', 'internal', ARRAY['id','scenario_version_id','parameter_set_id','period_from','period_to','estimate',
        'status','requested_by','created_at','started_at','finished_at','version']),
  ('replay_results', 'personal', ARRAY['entity_id']),
  ('replay_results', 'detection-logic', ARRAY['would_alert','near_miss','segment_ref','measures','detection_key',
        'coverage_state','bucket']),
  ('replay_results', 'internal', ARRAY['id','replay_id','entity_kind','business_date','branch_id','created_at']),
  ('replay_samples', 'personal', ARRAY['entity_id']),
  ('replay_samples', 'detection-logic', ARRAY['line','stratum','stratum_population','stratum_sampled','plan_seed',
        'replay_result_id']),
  ('replay_samples', 'internal', ARRAY['id','replay_id','packet_object_id','status','created_at']),
  ('sample_labels', 'detection-logic', ARRAY['label','note_code']),
  ('sample_labels', 'internal', ARRAY['id','sample_id','labeller_id','labelled_at']),
  ('shadow_results', 'personal', ARRAY['entity_id']),
  ('shadow_results', 'detection-logic', ARRAY['would_alert','detection_key','bucket','outcome']),
  ('shadow_results', 'internal', ARRAY['id','scenario_version_id','run_id','entity_kind','business_date',
        'live_alert_id','compared_at','created_at']),
  ('detection_keys', 'personal', ARRAY['entity_id','anchor_hk']),
  ('detection_keys', 'detection-logic', ARRAY['bucket','bucket_start']),
  ('detection_keys', 'internal', ARRAY['fingerprint','scenario_id','entity_kind','first_version_id',
        'last_version_id','measures_digest','emitted_count','matches','first_seen_at','last_seen_at']),
  ('seeded_tests', 'detection-logic', ARRAY['case_code','polarity','fixture','expected']),
  ('seeded_tests', 'internal', ARRAY['id','catalogue_item_id','scenario_id','active','last_run_id',
        'last_run_version_id','last_run_at','last_result','consecutive_failures','created_at','version']),
  ('catalogue_imports', 'detection-logic', ARRAY['items']),
  ('catalogue_imports', 'internal', ARRAY['id','catalogue_version','issuance_ref','evidence_object_id',
        'adoption_date','release_notes','item_count','status','change_request_id','created_by','created_at',
        'applied_at','version'])
) AS v(t, cls, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('scn', 'scenario_versions', 'R9', 'retirement', NULL,         'Logic history; retired versions are kept (S-58)'),
 ('scn', 'parameter_sets',    'R9', 'age',        'created_at', 'Parameter history at least seven years (S-58, L-18)'),
 ('scn', 'scenario_runs',     'R9', 'age',        'created_at', 'Run ledger with the seven-year floor (S-58)'),
 ('scn', 'replays',           'R9', 'age',        'created_at', 'Replay evidence behind change requests (S-41)'),
 ('scn', 'replay_results',    'R9', 'age',        'created_at', 'Alert and near-miss rows; suspicion-tier rows masked at read'),
 ('scn', 'replay_samples',    'R9', 'age',        'created_at', 'Sampling plans behind a go-live decision (S-42)'),
 ('scn', 'sample_labels',     'R9', 'age',        'labelled_at', 'Blind labels as validation evidence'),
 ('scn', 'shadow_results',    'R9', 'age',        'created_at', 'Shadow comparison evidence (S-44)'),
 ('scn', 'detection_keys',    'R5', 'age',        'first_seen_at', 'Detection identities outlive the alerts they explain'),
 ('scn', 'seeded_tests',      'R9', 'retirement', NULL,         'Known-answer cases; deactivated, never deleted'),
 ('scn', 'catalogue_imports', 'R9', 'age',        'created_at', 'Issuance staging; applied items are kept (T-02)')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

## 2. DDD Implementation (Rust)

**Crate placement** (workspace pinned in phase 00; layer rules in `ddd.md`):

```
backend/
├── packs/scenarios/documents.v1.0.0.json   # signed release data (C-38): CBI baseline and AKR library documents,
│                                           #   default parameter sets, unit fixtures, seeded cases (section 3.15)
├── schemas/
│   ├── scenario-document.v1.json           # DSL.md Part A as JSON Schema; additionalProperties false at every level
│   └── parameter-set.v1.json
├── crates/
│   ├── domain/   # + scenario: ScenarioDocument, Measure, Filter, Expr, Override, SegmentRef, LogicVersion, VersionStatus,
│   │             #   ParameterSet, Bump; ScenarioValidator (pure part), ScenarioCompiler (pure), Fingerprint, Bucket,
│   │             #   DetectionPayload, ScenarioCardRenderer (pure, the A.12 sentences); ports AlertSink, ReaderCatalogue,
│   │             #   EventEvaluator, LinkedPartyResolver
│   ├── db/       # + Pg repositories of section 2 for the eleven tables; the partition factory call on 23514
│   ├── authz/    # + resource adapters ScenarioVersion, ParameterSet, Replay, ReplaySample, CatalogueImport
│   └── jobs/     # + the nine job kinds of section 4.1 (akr-worker)
├── api/          # + routers `scenarios`, `replays`, `seeded`; `catalogue` (POST imports) and `coverage` (GET matrix)
│                 #   extended
├── worker/       # + ScenarioRunner, ScenarioFanout, ReplayRunner, SampleBuilder, ShadowCompare, SeededSuite,
│                 #   ScenarioMetrics, ReaderReplayRunner; EventEvaluator (the CEL adapter)
└── ctl/          # packs seed and packs verify-data extended with the document pack
```

Layer rule: the domain holds the document model, the validator's pure checks, the compiler's pure parts, the fingerprint
and the payload. Anything that reads or writes `canon`, `feat`, `admin` or `scn` is in `db` or `worker`.

- **Domain entities** (`Debug, Serialize, Deserialize, FromRow, ToSchema`; fields mirror section 1; enums as Rust enums
  with `#[sqlx(type_name = "scn.version_status")]` for the one Postgres enum): `ScenarioVersion`, `ParameterSet`,
  `ScenarioRun`, `Replay`, `ReplayResult` (the partitioned table; read by keyset), `ReplaySample`, `SampleLabel`,
  `ShadowResult`, `DetectionKey`, `SeededTest`, `CatalogueImport`.
- **Value objects:** `LogicVersion` (semver; `Bump::required(old: &ScenarioDocument, new: &ScenarioDocument) -> Bump`
  with `Patch`, `Minor`, `Major` as DSL A.10 defines them); `VersionStatus` with `transition_allowed(from, to)` (the same
  table as the section 1.3 trigger, tested against it); `ScenarioDocument` (serde, `deny_unknown_fields`); `Measure
  { op, field, where_, alias }`; `Filter` (the DSL Filter subset as an AST); `Expr` (CEL source with its declared
  context); `SegmentRef` (`Segment(String)`, `Risk(u8)`, `Currency(IsoCode)`); `ParamValue { value, overrides:
  Vec<Override> }`; `Fingerprint([u8; 32])` (`hex()` gives the stored text); `Bucket` (`Start(NaiveDate)` or
  `Event(Uuid)`); `Line` (`Above`, `Below`); `DetectionOutcome` (`AgreeAlert`, `AgreeQuiet`, `ShadowOnly`, `LiveOnly`);
  `Coverage { dark_units, excluded_units, blind_period_ids, stale_sources, degraded, degraded_reasons }`;
  `EffectDirection` (`Increase`, `Neutral`, `Decrease`; the phase 04 `VolumeEffect` is built from it, and `Unknown` is
  never produced here).
- **Create/Update structs:** `NewScenario` (`code`, `family`, `name`, `summary`, `typologies`, `needs`, `external_needs`,
  `observational`, `visibility`, `owner_id`, `research_grade = 'D'` for bank scenarios); `NewScenarioVersion`
  (`logic_version`, `document`, `document_sha256`, `fixtures`, `cbi_pack_version`, `register_version`, `created_by`);
  `DraftUpdate { document?, fixtures?, document_sha256 }` (`Option` fields); `NewParameterSet`; `NewReplay`;
  `NewRun`; `NewSampleSet`; `NewLabel { sample_id, labeller_id, label, note_code? }`; `NewDetectionKey`;
  `NewCatalogueImport`.
- **Repository traits** (domain layer; `Pg*` implementations in `db`; every write takes the caller's transaction):

```rust
#[async_trait]
pub trait ScenarioVersionRepository: Send + Sync {
    async fn insert_draft(&self, tx: &mut Tx, v: &NewScenarioVersion) -> Result<ScenarioVersion, RepoError>;
    /// Only in `draft`; `expected_version` is the If-Match value (E_VERSION_CONFLICT on a race).
    async fn update_draft(&self, tx: &mut Tx, id: Uuid, u: &DraftUpdate, expected_version: i32)
        -> Result<ScenarioVersion, RepoError>;
    async fn open_edit(&self, conn: &mut PgConnection, scenario: Uuid) -> Result<Option<ScenarioVersion>, RepoError>;   // draft or testing
    async fn live(&self, conn: &mut PgConnection, scenario: Uuid) -> Result<Option<ScenarioVersion>, RepoError>;
    async fn shadows(&self, conn: &mut PgConnection, scenario: Uuid) -> Result<Vec<ScenarioVersion>, RepoError>;
    async fn list_for(&self, conn: &mut PgConnection, scenario: Uuid) -> Result<Vec<ScenarioVersion>, RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<ScenarioVersion>, RepoError>;
    /// The status change is guarded by `WHERE status = from`; a zero row count is RowConflict (a concurrent move).
    async fn move_status(&self, tx: &mut Tx, id: Uuid, from: VersionStatus, to: VersionStatus, patch: &StatusPatch)
        -> Result<(), RepoError>;
}

#[async_trait]
pub trait ParameterSetRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, p: &NewParameterSet) -> Result<ParameterSet, RepoError>;
    async fn next_version(&self, conn: &mut PgConnection, version: Uuid) -> Result<i32, RepoError>;
    async fn active(&self, conn: &mut PgConnection, version: Uuid) -> Result<Option<ParameterSet>, RepoError>;
    async fn last_superseded(&self, conn: &mut PgConnection, version: Uuid) -> Result<Option<ParameterSet>, RepoError>;
    async fn move_status(&self, tx: &mut Tx, id: Uuid, from: ParamStatus, to: ParamStatus, at: DateTime<Utc>)
        -> Result<(), RepoError>;
    async fn list(&self, conn: &mut PgConnection, version: Uuid, page: Keyset) -> Result<Page<ParameterSet>, RepoError>;
}

#[async_trait]
pub trait RunRepository: Send + Sync {
    /// INSERT ... ON CONFLICT DO NOTHING on the batch unique index; `None` when the batch was already run.
    async fn start_once(&self, tx: &mut Tx, r: &NewRun) -> Result<Option<Uuid>, RepoError>;
    async fn progress(&self, tx: &mut Tx, id: Uuid, p: &RunProgress) -> Result<(), RepoError>;
    async fn finish(&self, tx: &mut Tx, id: Uuid, f: &RunFinish) -> Result<(), RepoError>;
    async fn recent(&self, conn: &mut PgConnection, version: Uuid, limit: i64) -> Result<Vec<ScenarioRun>, RepoError>;
}

#[async_trait]
pub trait ReplayRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, r: &NewReplay) -> Result<Replay, RepoError>;
    /// Commits one chunk of results and the cursor in the same transaction (section 3.9 step 4).
    async fn append_chunk(&self, tx: &mut Tx, replay: Uuid, rows: &[NewReplayResult], cursor: &serde_json::Value,
                          counters: &ReplayCounters) -> Result<(), RepoError>;
    async fn finish(&self, tx: &mut Tx, id: Uuid, f: &ReplayFinish) -> Result<(), RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<Replay>, RepoError>;
    async fn results(&self, conn: &mut PgConnection, replay: Uuid, after: Option<Uuid>, limit: i64, include_masked: bool)
        -> Result<Vec<ReplayResult>, RepoError>;
    async fn summary_cells(&self, conn: &mut PgConnection, replay: Uuid) -> Result<SummaryCells, RepoError>;
}

#[async_trait]
pub trait SampleRepository: Send + Sync {
    async fn insert_plan(&self, tx: &mut Tx, rows: &[NewReplaySample]) -> Result<u64, RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<ReplaySample>, RepoError>;
    /// Returns `Err(RowConflict)` on the unique (sample, labeller) pair; the service maps it to 409.
    async fn insert_label(&self, tx: &mut Tx, l: &NewLabel) -> Result<SampleLabel, RepoError>;
    async fn labels_for(&self, conn: &mut PgConnection, replay: Uuid) -> Result<Vec<LabelRow>, RepoError>;
}

#[async_trait]
pub trait ShadowRepository: Send + Sync {
    async fn append_chunk(&self, tx: &mut Tx, rows: &[NewShadowResult]) -> Result<u64, RepoError>;
    /// Compares the day's shadow rows with `detection_keys` and writes the outcomes (section 3.11 step 3).
    async fn compare_day(&self, tx: &mut Tx, version: Uuid, live: Uuid, day: NaiveDate) -> Result<CompareCounts, RepoError>;
    async fn counts(&self, conn: &mut PgConnection, version: Uuid, from: NaiveDate, to: NaiveDate)
        -> Result<Vec<DayCounts>, RepoError>;
}

#[async_trait]
pub trait DetectionKeyRepository: Send + Sync {
    /// Inserts the key, or returns the existing row (the caller then compares the measures digest).
    async fn insert_or_get(&self, tx: &mut Tx, k: &NewDetectionKey) -> Result<KeyOutcome, RepoError>;
    async fn touch(&self, tx: &mut Tx, fp: &Fingerprint, version: Uuid, digest: &[u8; 32], matches: bool)
        -> Result<(), RepoError>;
    /// The newest key of the anchor whose bucket starts on or after `since` (the dedupe fold, section 3.7).
    async fn open_bucket(&self, tx: &mut Tx, scenario: Uuid, anchor_hk: &[u8; 32], since: NaiveDate)
        -> Result<Option<DetectionKey>, RepoError>;
}

#[async_trait]
pub trait SeededRepository: Send + Sync {
    async fn active_for(&self, conn: &mut PgConnection, scenario: Uuid) -> Result<Vec<SeededTest>, RepoError>;
    async fn record(&self, tx: &mut Tx, id: Uuid, rec: &SeededRecord) -> Result<(), RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &SeededFilter, page: Keyset) -> Result<Page<SeededTest>, RepoError>;
}

#[async_trait]
pub trait CatalogueImportRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, i: &NewCatalogueImport) -> Result<CatalogueImport, RepoError>;
    async fn move_status(&self, tx: &mut Tx, id: Uuid, from: ImportStatus, to: ImportStatus) -> Result<(), RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<CatalogueImport>, RepoError>;
}
```

- **Ports** (domain traits; implementations registered at start-up in `api` and `worker`):

```rust
/// The one detection hook. Phase 16 ships RecordingAlertSink; phase 17 replaces it in AppState with AlertIntakeService.
#[async_trait]
pub trait AlertSink: Send + Sync {
    async fn emit(&self, tx: &mut Tx, d: &DetectionPayload) -> Result<EmitOutcome, AlertSinkError>;
}
pub enum EmitOutcome { New, Duplicate, Amended(AmendmentReason) }
pub enum AmendmentReason { MeasuresChanged, LateRowsAdded, NoLongerMatches }

/// Phase 13 port (names as phase 13 section 3.8). Implemented here from scenario versions and their replays.
#[async_trait]
pub trait ReaderCatalogue: Send + Sync {
    async fn readers(&self, conn: &mut PgConnection, object: &ObjectRef) -> Result<Vec<ReaderRef>, RepoError>;
    async fn measure(&self, conn: &mut PgConnection, object: &ObjectRef, proposal: &serde_json::Value)
        -> Result<Option<VolumeEffect>, RepoError>;
}

/// Event conditions (DSL A.6.2). The CEL implementation is named in Open doubts; the port hides it.
pub trait EventEvaluator: Send + Sync {
    fn compile(&self, source: &str, context: &TypedContext) -> Result<CelProgram, ExprError>;
    /// `Err(EvalError::Budget)` is fail-closed: the run fails (section 3.6 step 6), never skips the record.
    fn eval(&self, p: &CelProgram, values: &CelValues, budget: EvalBudget) -> Result<bool, EvalError>;
}

/// The shared linked-party service (S-08). One hop per requested link type, current rows only.
#[async_trait]
pub trait LinkedPartyResolver: Send + Sync {
    async fn groups(&self, conn: &mut PgConnection, customers: &[Uuid], types: &[LinkType], as_of: NaiveDate)
        -> Result<HashMap<Uuid, LinkedGroup>, LinkError>;
}
```

- **Consumed ports** (not defined here, except that this phase adds the four entry points of sections 3.2 and 3.6 to phase 13's module, as phase 13 section 7.2 says): phase 13 `FeatureCompiler` (measure and filter compiler), `ProfileComparator`,
  `RatingSource::levels(customers, as_of)`, `DeclaredProfileSource`; phase 11 `FreshnessService`; phase 09
  `BankCalendarClock` (with the backward walk of section 3.8); phase 04 `ChangeTarget`; phase 01 `StepUpService`.

- **utoipa schemas** (request and response DTOs; `ToSchema`): `ScenarioCreate { code, family, name, summary, typologies, needs, external_needs, observational, visibility, document }` (`owner_id` is the caller and `research_grade` is `D`, section 3.15 step 1), `ScenarioView` (register fields plus the
  live and shadow versions, `has_open_declaration`, `bank_grade`, `tests`), `ScenarioVersionCreate { logic_version?,
  document?, changelog }`, `ScenarioVersionUpdate { document, fixtures? }`, `ValidationReport { errors, warnings,
  compile }`, `ParameterSetCreate { values, basis, reason_code, reason_text }`, `ReplayCreate { scenario_version_id,
  parameter_set_id?, parameter_values?, baseline ("live" or "none"), period_from, period_to }`, `ReplayView`,
  `ReplaySummaryView` (cells below `scenario.summary_min_cell` are returned as `null`), `ReplayResultPage`,
  `SampleCreate { lines }`, `SampleLabelCreate { label, note_code? }`, `SamplePacketView`, `ShadowStart`, `ShadowView`,
  `KillSwitchRequest { reason_code, reason_text }`, `KillSwitchResult`, `CardView`, `MetricsView`, `SeededTestPage`,
  `SeededRunRequest { scenario_version_id? }`, `CatalogueImportCreate`, `CoverageMatrixPage`, `RetireRequest { reason_code,
  reason_text, effective_to }`.

- **Route table** (scopes from API.md section 2; "NEW" marks the two rows this phase adds to API.md 3.11; the cedar action follows each
  description; reads list every scope that admits a role the Cedar policy admits, section 3.17):

| Method | Path | Scope | Description (cedar action) |
|-|-|-|-|
| POST | `/api/v1/scenarios` | config.author | Create a bank scenario 901-999 with its first draft; owner is the caller (`scenario.create`) |
| GET | `/api/v1/scenarios` | config.author | Register list (phase 06 row; extended with live and shadow version, bank grade and open declaration) (`scenario.list`) |
| GET | `/api/v1/scenarios/{id}` | config.author, config.approve, dashboard.read | Detail (phase 06 row, scope extended); versions, parameter sets and tests shown only where `scenario.version.read` allows (`scenario.read`, `scenario.version.read`) |
| POST | `/api/v1/scenarios/{id}/versions` | config.author | Open a draft version (`scenario.version.write`) |
| PUT | `/api/v1/scenarios/{id}/versions/{v}` | config.author | Replace the draft document and fixtures; `If-Match` (`scenario.version.write`) |
| POST | `/api/v1/scenarios/{id}/versions/{v}/validate` | config.author | Schema, semantic checks and the cost guard; no state change (`scenario.version.write`) |
| POST | `/api/v1/scenarios/{id}/versions/{v}/test` | config.author | Unit cases of the draft (7.17; seeded cases run by section 3.12); `draft` to `testing` on pass (`scenario.version.write`) |
| GET | `/api/v1/scenarios/{id}/parameter-sets` | config.author, config.approve | Parameter-set versions with basis, band and reason (`scenario.parameter_set.read`) |
| POST | `/api/v1/scenarios/{id}/parameter-sets` | config.author | Draft a parameter set; then the change request `parameter_set` (`scenario.parameter_set.write`) |
| POST | `/api/v1/scenarios/{id}/replays` | config.author | Replay a version and parameter set over an as-of period; cost-guarded, 202 (`scenario.replay.run`) |
| GET | `/api/v1/replays/{id}` | config.author, config.approve | Replay summary: counts by segment, branch and month, reach, overlap, look-ahead, effect (`scenario.replay.read`) |
| GET | `/api/v1/replays/{id}/results` | config.author, config.approve | Per-entity alert and near-miss rows (cursor); masked rows only for M; `X-Access-Purpose: qa_review` (`scenario.replay.results.read`) |
| POST | `/api/v1/replays/{id}/samples` | config.author | Generate the above- and below-the-line sampling plan and the blind packets, 202 (`scenario.sample.plan`) |
| POST | `/api/v1/samples/{id}/labels` | queue.work | Record a blind reviewer's label; never the scenario's author (`scenario.sample.label`) |
| GET | `/api/v1/samples/{id}/packet` (NEW) | queue.work | The blind packet: transaction facts and profile facts, no outcome, thresholds or reason code (`scenario.sample.label`) |
| POST | `/api/v1/scenarios/{id}/shadow` | config.author | Start a shadow run of a `testing` version (`scenario.shadow.run`) |
| GET | `/api/v1/scenarios/{id}/shadow` | config.author, config.approve | Shadow comparison with the live version by day and outcome (`scenario.shadow.read`) |
| POST | `/api/v1/scenarios/{id}/kill-switch` | config.approve | Fall back to the previous approved version; **SU** `approval` (`scenario.kill_switch`) |
| GET | `/api/v1/scenarios/{id}/card` | config.author, config.approve, dashboard.read | The generated scenario card (`scenario.card.read`) |
| GET | `/api/v1/scenarios/{id}/metrics` | config.author, config.approve, dashboard.read | Operational metrics with red and amber status (`scenario.metrics.read`) |
| POST | `/api/v1/scenarios/{id}/retire-requests` (NEW) | config.author | Propose the retirement of a bank or AKR scenario; change request `scenario` (`scenario.retire.request`) |
| GET | `/api/v1/seeded-tests` | config.author, config.approve | Known-answer suite with last results and the consecutive-failure count (`scenario.seeded.read`) |
| POST | `/api/v1/seeded-tests/runs` | config.author | Run the suite now, 202 (`scenario.seeded.run`) |
| POST | `/api/v1/cbi-catalogue/imports` | config.author | Stage a CBI or Office issuance as draft items; change request `cbi_catalogue` (`cbi_catalogue.import`) |
| GET | `/api/v1/coverage/matrix` | dashboard.read | CBI item to scenario to readiness; alert, case and STR columns join in phases 17, 18 and 21 (`coverage.matrix.read`) |

- **Router functions:** `pub fn router() -> Router<AppState>` in `scenarios` (the rows above except the replay, seeded
  and catalogue rows), `replays` (replay, sample and packet rows), `seeded`; the rows `POST /cbi-catalogue/imports` and
  `GET /coverage/matrix` are added to the phase 06 routers `catalogue` and `coverage`.

## 3. Business Logic

### 3.1 Documents and validation (`ScenarioValidator`; S-51, S-38, S-39, S-40, S-59, TS-14; DSL.md s.7)

`ScenarioValidator::check(doc, ctx) -> ValidationReport { errors, warnings }`. `ctx` holds the active CBI items and
their links (phase 06), the scenario's register row, the canonical catalogue and its protected-attribute list (phase 10),
the feature keys and segment keys of phase 13, the active reason codes of families `detection`, `exclusion` and `override`
(phase 09), the calendars (phase 09), `scenario.hot_window_days` and the seeded cases of the scenario. Each rule adds
`ValidationIssue { path, code, message, severity }`. Rules run in this order; rule 1 stops the rest (paths are not
meaningful on a malformed document); the others run to the end so the maker sees every issue at once.

1. JSON Schema `scenario-document.v1.json` (`E_SCHEMA`; the path names the key). Stops.
2. `dsl` is `scenario` and `dsl_version` is `"1"` (`E_UNKNOWN_DSL_VERSION`). Stops.
3. Identity: `id` matches `SCN-<FAMILY>-NN` and the family equals the register row's family (`E_NAMESPACE`); `NN` in
   901 to 999 only for `pack: bank`, and `pack` equals the register row's pack (`E_NAMESPACE`). `pack: cbi` requires the
   register row to be `cbi` (a bank document cannot claim a CBI item).
4. Text: `name`, `description` and, on leaving draft, `changelog` have `ar` and `en` (`E_MISSING_I18N`); placeholders
   `{name}` match between the languages (`E_I18N_PLACEHOLDERS`).
5. CBI references: each `cbi_refs` entry is an active item of the catalogue (`E_CBI_REF_UNKNOWN`); `pack: cbi` has at
   least one (`E_CBI_REFS_REQUIRED`); the set of items named equals the register's links for this scenario (a difference
   is `E_CBI_REF_UNKNOWN` at the item that differs; the register, not the document, owns links, phase 06 section 3.1).
6. Inputs: every CanonicalRef read by `where`, measures, `having`, `condition`, `context`, `outputs` and `exclusions` is
   listed in `data_requirements` (`E_UNDECLARED_INPUT`); every name is a canonical attribute, a feature key of phase 13
   (`feat.<key>`), a rating attribute (`rating.level`, `rating.cbi_level`) or a screening fact (`screening.<fact>`)
   (`E_UNKNOWN_ATTRIBUTE`). A protected attribute (gender, religion, ethnicity, nationality, tribe, sect) in a filter,
   grouping, segment, override or rating factor is `E_PROTECTED_ATTRIBUTE`; age band and occupation pass only with a
   `cbi_refs` entry that names them (S-38). A `?` marks an optional input (DSL section 1).
7. Parameters: names match `^[a-z][a-z0-9_]{1,40}$`; a name equal to or containing `max_alerts`, `top_n`, `alert_limit`,
   `alert_cap` or `max_per_period` is `E_ALERT_CAP` (S-39, L-16; the pinned `limit` of DSL example 9.1 passes: it limits
   an amount, not a count of alerts); `default` lies inside `min`, `max` or `allowed` (`E_PARAM_OUT_OF_BOUNDS`);
   `default` lies inside `band` when a band is set (`E_DEFAULT_OUTSIDE_BAND`); `basis` is present; `DP` has `review_by`
   (`E_DP_NO_REVIEW`, S-40) and a `review_by` before today is `W_DP_EXPIRED`; an override `when` names a segment that
   exists (`E_DANGLING_REF`), a risk level 1 to 5, or a currency in `ref.currencies`; a parameter with `min` equal to
   `max` is pinned and cannot be changed by a parameter set.
8. `logic.kind` is `aggregate`, `event` or `profile` (`E_SCHEMA`); the kind's rules follow (9, 10, 11).
9. Aggregate and profile measures: `where` and `having` parse under the DSL Filter subset (`E_FILTER_NOT_COMPILABLE`);
   `window.length` is no longer than `scenario.hot_window_days` (`E_WINDOW_TOO_LONG`); `window.calendar` is `default` or
   a calendar of `ref.calendars` (`E_DANGLING_REF`); each measure op is in the closed set and `sum`, `avg`, `max` name a
   numeric field (`E_FIELD_TYPE`); a `ratio` names two existing aliases (`E_FIELD_TYPE`); `group_by` is the aggregation
   level or a canonical key; every `:name` is a declared parameter (`E_UNKNOWN_PARAM`).
10. Event: `on` is one of the closed list of DSL A.6.2 (`E_SCHEMA`); each `context` entry is an entity or a feature key
    (`E_UNKNOWN_ATTRIBUTE`); `condition` compiles and returns bool within the cost bound (`E_EXPR_COMPILE`, `E_EXPR_TYPE`,
    `E_EXPR_COST`); it reads nothing outside `context` (`E_UNDECLARED_INPUT`).
11. Profile: `expected[].source` is `declared`, `own_history` or `peer`; a `peer` comparator names a peer group or segment
    statistic of phase 13 (`E_DANGLING_REF`); `combine` is `any` or `all`; `min_history` is at least one month.
12. Aggregation: `level` and `currency_mode` are closed sets (`E_SCHEMA`); `linked_parties` is a subset of the listed kinds
    (`E_SCHEMA`), and a type with no canonical link (section 3.8) is refused as `E_UNKNOWN_ATTRIBUTE`; `posted_only: false`
    requires the logic to name `txn_event.kind` (`E_SCHEMA` at `/aggregation/posted_only`); `settle_delay` is between zero
    and one day (`E_SCHEMA`).
13. Exclusions and suppression: a static exclusion carries an active `exclusion` reason code (`E_DANGLING_REF`); a
    suppression rule that stops evaluation is `E_HIBERNATION_SKIPS_EVALUATION` (DSL A.7); `dedupe.same_entity_within` is at
    least one day (`E_SCHEMA`).
14. Outputs: at least one reason code (`E_NO_REASON_CODE`); each code is active in family `detection` (`E_DANGLING_REF`);
    each `facts` binding names a measure, a parameter or a declared input (`E_DANGLING_REF`); `evidence.fields` are declared
    inputs (`E_UNDECLARED_INPUT`); `case_family` is in the closed list (`E_SCHEMA`).
15. Evidence label: `evidence_label` is required for `pack: akr` (`E_SCHEMA`). `N` is shown on the card as unvalidated
    until the bank's own cases validate it (S-59); no error is raised.
16. Tests: `tests.unit` and `tests.seeded` name fixtures present in `fixtures` (`E_DANGLING_REF`) and cases present in
    `scn.seeded_tests` for this scenario (`E_DANGLING_REF`); leaving draft requires three positive and three negative unit
    cases (`E_TESTS_MISSING`, section 3.3).
17. Warnings: `W_OUTSIDE_BAND` and `W_LOOSENING` come from parameter sets (section 3.5); `W_DEGRADED_INPUT` for an optional
    input whose bank readiness grade is C or D (phase 06); `W_DP_EXPIRED` from rule 7.
18. Cost (aggregate and profile): the statement is explained against the cost guard (section 3.2 step 6; `E_COST_GUARD`).

**Validation response.** `POST .../validate` returns `{errors, warnings, compile: {kind, plan_hash, cost}}` with status
200 and changes no state. When there are no errors, the compiled artefacts are stored on the draft (`compiled_sql`,
`compiled_program`, `compile_cost`), so the studio and the card read what the engine will run.

### 3.2 Compilation and the cost guard (`ScenarioCompiler`, pure parts in domain; S-51, TS-14)

`ScenarioCompiler::compile(doc, plan: &ParameterPlan, window: &WindowBinds) -> CompiledScenario { kind, sql, binds,
plan_hash }`. No document text reaches the SQL string; every literal is a bound parameter.

1. **Aggregate statement.** From `canon.txn` (joined to `canon.account` current rows for account-level scenarios and to
   the linked-group bound array of section 3.8 when `aggregation.linked_parties` is not empty). The counting terms of DSL
   A.4 are added by the compiler and cannot be switched off by a document: `status = 'posted'` unless `posted_only` is
   false, `system_generated IS NOT TRUE` unless the logic names the system-generated kind, and reversal netting (a row with
   `reverses_id` and its original are excluded as a pair by `NOT EXISTS` over the window). Currency separates the groups
   when `currency_mode` is `separate`.
2. **Measures and filters are compiled by phase 13.** Each measure of the document becomes an ephemeral feature
   definition `{entity: aggregation.level, expression: {kind: op, field, where}, window: the window binds, currency_mode}`
   and is compiled by `FeatureCompiler::compile_measure` (the measure and filter compiler of phase 13 section 3.1, exposed
   for this use); `where` and `having` go through `FeatureCompiler::compile_filter`. The ephemeral definition is never
   stored in `feat.feature_definitions`. There is no second compiler: a scenario measure and a feature with the same
   expression produce the same SQL (golden test, section 6).
3. **Parameters are bound, not interpolated.** Effective values per group come from the parameter plan (section 3.5 step
   3) as a `VALUES` CTE `(group_key, currency, parameter, value)`; `having` and `near_miss` reference that CTE by alias.
4. **Windows.** `rolling` with a duration: `booking_date BETWEEN $from AND $to`, `$from` from the length in calendar days or
   from `BankCalendarClock` for business days (section 3.8 step 5). `calendar`: the bounds of the period. Partition pruning
   follows from the bounds (the `booking_date` partitions of `canon.txn`).
5. **Identifiers come only from the closed catalogue** (phase 10 `CanonicalCatalogue`, the phase 13 feature whitelist).
   The compiler takes no identifier as input from the document. A name outside the catalogue cannot reach the compiler
   because the validator refused it (rule 6).
6. **Cost guard.** `EXPLAIN (FORMAT JSON)` of the statement with the window bound to the newest closed month of one
   partition. A planner total cost above `scenario.cost_max_total`, or an estimated row count above
   `scenario.cost_max_rows`, is `E_COST_GUARD` (422) with `{estimated_rows, total_cost, bound, partition}` in the body
   (API.md section 6). Every run sets `statement_timeout` to `scenario.statement_timeout_s` with `SET LOCAL`.
7. **Event kind.** `EventEvaluator::compile(condition, context)`. The context carries only the declared entity fields and
   feature values, read as of the event's business date (`feat.feature_values` rows computed on or before that date). CEL
   has no query access.
8. **Profile kind.** The measure compiles as in step 2. The comparator plan is `ProfileComparator::plan(expected, combine,
   min_history)` (phase 13): `declared` through `DeclaredProfileSource`, `own_history` through the customer's earlier
   feature values, `peer` through the segment statistic of the period before (`p99`; no look-ahead). A `fallback_only`
   comparator is used only when an earlier one cannot be computed; its use marks the alert `degraded` (S-10). A missing
   declared value is never read as "no expectation": the peer comparator applies and the case shows "declared income
   missing".
9. **Output.** `compiled_sql` (aggregate), `compiled_program` (event: CEL source and typed context; profile: the comparator
   plan), `compile_cost`. The `plan_hash` (SHA-256 over the canonical SQL and the bind names) identifies the compilation.

### 3.3 Lifecycle and the one live version (`ScenarioVersionService`; DSL A.10, S-52; E_TWO_LIVE)

| From | To | Caused by | Guard |
|-|-|-|-|
| draft | testing | `POST .../test` passes | section 3.1 rules 1 to 16 with zero errors; three positive and three negative unit cases (`E_TESTS_MISSING`) |
| testing | draft | `PUT .../versions/{v}` (an edit) | `If-Match`; clears `tests_passed_at` |
| testing | shadow | `POST /scenarios/{id}/shadow` (section 3.11) | a completed replay of this version and its proposed set |
| shadow | live | the approved change request of kind `scenario` (section 3.14) | the go-live gate (section 3.4), re-run inside the activation transaction |
| live | superseded | the activation of a newer live version, same transaction | the old row moves first, so the partial unique index holds |
| live | retired | retirement change request (section 3.15) or kill switch (section 3.13) | `cbi` never retired (`E_BASELINE_RETIRE`) |
| superseded | live | kill switch only (`reinstated_at` set) | the fallback of section 3.13 |
| draft, testing | retired | withdrawal with `POST /scenarios/{id}/retire-requests` (section 3.15) | no change request: the version never evaluated anything |

The database enforces the table (the trigger of section 1.3 `scn.version_guard`) and the partial unique index
`uq_scenario_one_live` enforces one live version per scenario (`E_TWO_LIVE`). A shadow version is never edited: a
change to a shadow is a new version, which keeps the comparison honest. One open edit (draft or testing) per scenario is
enforced by `uq_scenario_one_open_edit` (`E_NOT_ALLOWED_IN_STATE`, reason `draft_open`).

**Opening a version** (`POST /scenarios/{id}/versions`):
1. Cedar `scenario.version.write` (CFG); scope `config.author`; `Idempotency-Key`.
2. Refuse when an open edit exists (`draft_open`). The new `logic_version` must be a valid successor of the newest one:
   `Bump::required(previous, new)` is computed from the documents (`Patch` for text, `Minor` for an optional parameter,
   an exclusion type, an evidence field or a bound change, `Major` for a new operator, window or aggregation level). A
   lower bump than required is `E_VALIDATION` with `errors[].code = E_LOGIC_VERSION_BUMP` at `/logic_version` (added to DSL.md
   section 7 by this phase).
3. Insert `draft` with `document_sha256`, `cbi_pack_version` (the installed CBI pack) and `register_version`. The default
   `document` is a copy of the newest live or shadow version. Audit `scenario.version_opened`.

**Activation** (the approval transaction of the `scenario` target, section 3.14):
1. `SELECT ... FROM scn.scenarios WHERE id = $1 FOR UPDATE`; refuse with `pack_not_agreed` when the version is `pack: akr`
   and `pack.akr_live` is off.
2. Re-run the go-live gate (section 3.4). Any failure is `E_NOT_ALLOWED_IN_STATE` and the decision is refused.
3. Move the live version L (if any) to `superseded` with `superseded_by_version_id` = N and `superseded_at` = now.
4. Move N from `shadow` to `live` with `effective_from` = `effective_at` of the request or now, and `change_request_id`.
5. The proposed parameter set S of N becomes `active` (the previous active set of N, if any, becomes `superseded`).
6. Audit `scenario.version_activated {scenario_id, version_id, logic_version, superseded_version_id, parameter_set_id}`;
   outbox `scenario.changed {scenario_id, version_id, change: "activated"}`.

### 3.4 The go-live gate (`GoLiveGate::check`; S-47, S-49, S-42, TS-15; DSL A.10)

`GoLiveGate::check(conn, N, S, now) -> GateReport` returns every condition with its result, so the maker sees all the
refusals at once. At `prepare` it is advisory; at `check_before_decision` and at activation it blocks. Failures are
`E_VALIDATION` (errors[] with the DSL code) for document and test evidence, and `E_NOT_ALLOWED_IN_STATE` with a `reason`
for state conditions.

| Gate | Condition | Refusal |
|-|-|-|
| G1 document | zero validation errors (section 3.1) | `E_VALIDATION` with errors[] |
| G2 units | three positive and three negative unit cases; `tests_passed_at` set | `E_TESTS_MISSING` |
| G3 seeded | for every CBI item the version implements (`full` links), a positive and a negative seeded case whose `last_run_version_id` = N, `last_result` = `pass`, `last_run_at` within one business day | `E_NOT_ALLOWED_IN_STATE` `seeded_not_passing` or `seeded_missing` |
| G4 replay | a completed replay of N with set S over at least `scenario.replay_min_months` of history (or all history when shorter), not a reader replay (no `proposal_hash`); its monthly alert rate inside the range stated on the request (`impact.expected_monthly_alerts`) | `E_EVIDENCE_MISSING` (errors[]) or `E_NOT_ALLOWED_IN_STATE` `replay_outside_range` |
| G5 below the line | a sampling plan for that replay with both lines; every sample labelled by a labeller other than the scenario's author; the estimated below-the-line miss rate (section 3.10 step 7) at most `scenario.btl_tolerance_pct` | `E_NOT_ALLOWED_IN_STATE` `btl_tolerance_exceeded`; a missing plan is `E_EVIDENCE_MISSING` |
| G6 inputs | the bank readiness grade of the scenario is A or B (phase 06; C and D block, S-47) | `E_NOT_ALLOWED_IN_STATE` `readiness_blocked` |
| G7 shadow | shadow started, elapsed days at least `shadow_days_required` (floor `scenario.shadow_min_days`), and every day since the start compared (`shadow_compare`) | `E_NOT_ALLOWED_IN_STATE` `shadow_too_short` |
| G8 route | the change request's route is `cr_material_validated` for `pack: cbi` (section 3.14) | refused at submit |
| G9 pack | `pack: akr` only with `pack.akr_live` on | `E_NOT_ALLOWED_IN_STATE` `pack_not_agreed` |
| G10 evidence age | replay and shadow evidence younger than `change.evidence_max_age_days` at approval (phase 04) | `E_EVIDENCE_STALE` |
| G11 volume | `volume_effect` is computed from the replay (`increase`, `neutral` or `decrease`), never `unknown` for a material route | phase 04 refuses `unknown` as material |

Maker and checker, four eyes and step-up on the decision are phase 04's (TS-15). A version whose bank readiness grade
falls to C or D after it is live is not switched off: it stays live with its declared "not covered" state and is shown
"not operating as intended" (DSL A.3; phase 06 edge case). The gate therefore applies to going live, not to staying live.

### 3.5 Parameter sets, bounds, bands and the change class (`ParameterService`; S-39, L-18, S-40; DSL A.5, D.3)

**Values.** `values` maps each parameter to `{value, overrides: [{when, value}]}`. Overrides are the DSL overrides of the
document (`when` names a `segment:<key>`, `risk:<n>` or `ccy:<ISO>`); a parameter set changes the values of overrides that
the document already declares. A new `when` is a logic change (a new version), refused as `E_DANGLING_REF`. This is how
per-segment values are stored: DATA-MODEL's `segment` column is replaced by these overrides (section 1).

**Drafting** (`POST /scenarios/{id}/parameter-sets`; Cedar `scenario.parameter_set.write`, CFG; scope `config.author`):
1. The version must be draft, testing, shadow or live (`version_closed` for superseded or retired).
2. Each value passes the hard bounds (`min`, `max`, `allowed`; `E_PARAM_OUT_OF_BOUNDS`); a pinned parameter (`min` equal to
   `max`) cannot change; a `DP` basis needs a `review_by` (`E_DP_NO_REVIEW`).
3. Effective values per group: for each group the first matching override, else the default. `band_status` is `inside`
   when every value that differs from the document's default lies inside the document's `band` (or the parameter has no
   band), else `outside` (W_OUTSIDE_BAND; the change is material by DSL D.3).
4. Reason: `reason_code` is an active code of family `tuning` or `change`; `reason_tag` is copied from the code. A
   `capacity` reason needs a below-the-line sample as evidence (phase 04 `E_CAPACITY_NO_BTL`) and an approver outside
   operations (S-39).
5. Lock the scenario row (`FOR UPDATE`, as section 3.13 step 3 does), so that two drafts cannot take the same number; then insert `draft` with `set_version` = the next number for the version (`ParameterSetRepository::next_version`). Audit
   `parameter_set.drafted {scenario_id, version_id, parameter_set_id, band_status}`.
6. Proposing: `POST /change-requests` (phase 04 generic create, `object_kind = parameter_set`, `object_key =
   parameter_set:<scenario_id>`, `proposed = {parameter_set_id}`). The draft moves to `proposed` at submit; the partial
   unique index `uq_parameter_set_proposed` allows one.

**Change class** (`ParameterSetTarget::prepare`, phase 04 classification rules; DSL D.3):
- `base_class`: `minor` when `band_status` is `inside` and the volume effect is `increase` or `neutral`; `material`
  otherwise (outside the band, or any volume effect of `decrease`, which is a loosening).
- `volume_effect`: from the completed replay of this set against the live baseline (`replays.effect`, section 3.9). No
  such replay gives `unknown`, which can never be approved as material (phase 04).
- The phase 04 rule "volume_effect = decrease raises to at least material" and the capacity rule apply on top.
- `impact` for the approver: before and after counts from the baseline and the candidate replay, by segment and branch
  (L-18), the parameter names that moved and their basis tags.

**Activation** (section 3.3 step 5): the proposed set becomes `active`; the previous active set becomes `superseded`; audit
`parameter_set.activated`; outbox `scenario.changed {change: "parameters"}`.

**Retention of parameter history.** Sets are never deleted (guard of section 1.3). They are kept for `scenario.history_years`
(at least seven years, S-58), as the retention row of section 1.4 states.

**The default sets.** The shipped default values are the document's `default`s and the signed default parameter sets of
the pack (section 3.15). A bank value is always a parameter set, so the card can show both (DSL A.5).

### 3.6 Execution: incremental, settled and sweep (`ScenarioFanout`, `ScenarioRunner`; S-09, S-44, TS-14; DSL A.11)

**Fan-out** (job `scenario_touch_fanout`, triggered by the outbox topic `canon.touched` of phase 11):
1. Read the batch from the payload (`batch_id`, `touch_id`, `source_id`, `entity_kinds`, `business_date`).
2. Collect the touched keys: `SELECT DISTINCT account_id FROM canon.txn WHERE batch_id = $1` (and `canon.txn_event`), the
   customers of the touched accounts (current `canon.account` rows and their `customer_id`), and the customers and parties
   whose rows carry the batch id (`canon.customer`, `canon.party`). The index of section 1.3 serves the batch lookup.
3. For each version in status `live` (mode `incremental`) and each version in `shadow` (mode `shadow`) whose document reads
   one of the touched entity kinds: enqueue `scenario_run {version, mode, batch_id}`. For an aggregate or profile version
   with a non-zero `settle_delay`, also enqueue `scenario_run {version, mode: settled, batch_id}` with `run_at` = now plus
   the settle delay (admin.jobs `run_at`). Event versions are evaluated at once and have no settled run unless their
   document declares a delay.
4. The fan-out is idempotent: the run ledger's unique index on (version, mode, batch) makes a second delivery of the same
   event a no-op at the row level (section 1.2).

**Run** (job `scenario_run {scenario_version_id, mode, batch_id?, window_to?}`):
1. Load the version. A version that is neither live (for `incremental`, `settled`, `sweep`) nor shadow (for `shadow`) is a
   no-op: the scenario was retired or killed after the job was queued.
2. `RunRepository::start_once` on the batch key. `None`: the batch was already run for this version and mode; stop. The planner
   estimate of the statement (section 3.2 step 6) is checked here as well: above the bound the run finishes `refused_cost` with the
   estimate in `cost`, which DSL A.11 requires; audit `scenario.run_refused_cost`.
3. Snapshot the context: the active set, or the proposed set for a shadow version, or the document defaults; segment
   versions (phase 13); the rating model version (`RatingSource`); feature versions read; the CBI pack, register,
   calendar and reference versions; the mapping versions of the touched rows. These go into every detection.
4. Coverage (section 3.8): freshness per source, dark units, exclusions, blind periods.
5. Candidate groups. Incremental and shadow: the touched customers, expanded one hop by `LinkedPartyResolver` for the
   linked types of the document. Sweep: every customer with activity in the window (set-based, not a loop per account).
   Each group is anchored at its smallest customer id.
6. Event kind: for each event row of the batch matching `on`, evaluate the CEL condition under the budget
   (`EventEvaluator::eval`). A budget breach or an evaluation error fails the run (`error_code` `cel_budget` or
   `cel_error`): the record is never skipped (DSL A.6.2). The run is retried; after the last attempt the scenario shows
   "not operating as intended" (DSL A.3).
7. Aggregate kind: one bound statement per chunk of 5,000 groups (section 3.2), with the excluded-unit list and the
   parameter `VALUES` plan. Groups with a dark unit in the window are evaluated, and an alert is still raised, marked
   `degraded`; such a group is never recorded as quiet.
8. Profile kind: the period measure per customer and `ProfileComparator::evaluate` with `combine`; a comparator fallback
   marks the alert `degraded` (S-10).
9. For each group that would alert: the detection identity (section 3.7). Live: inside one transaction per chunk,
   `insert_or_get` on `detection_keys` and `AlertSink::emit` (the version is re-checked `FOR SHARE` on the scenarios row
   in that transaction, so a kill switch that committed first stops the emission). Shadow: a `shadow_results` row, no sink.
10. Non-alerting groups produce no output, with one exception: a group whose live detection key has `matches = true` and
    that no longer matches, after a late change, is emitted as an amendment `NoLongerMatches`. The alert is kept and its
    explanation says the pattern no longer holds; it is never silently changed (DSL A.4, S-09).
11. Progress is written per chunk (`RunRepository::progress`). Finish: counters (`entities`, `detections`, `amendments`),
    `coverage`, `cost` (planner cost and duration), status `completed`. Audit `scenario.run_completed {run_id, version_id,
    mode, entities, detections, amendments}` (counts only).
12. Failure: status `failed` with `error_code`; the job retries (five attempts). Audit `scenario.run_failed {run_id,
    version_id, mode, error_code}`.

**Late data.** A row with `late = true`, or a booking date before the newest evaluated window, triggers the settled run
for its batch. A changed measure gives `Amended(MeasuresChanged)` or `Amended(LateRowsAdded)` at the sink; phase 17 amends
the alert with an explanation and keeps the decision (DSL edge case, MODULES M11).

**Sweeps** (`scenario_sweep`, schedules `schedule.scenario_sweep_daily` and `schedule.scenario_sweep_monthly`). The daily
sweep runs the `L2` versions with the window ending on the bank business date; the monthly sweep runs the `L3` versions
with the month window. Each run is one set-based statement per version over the recent partitions. Shadow versions are
swept in mode `shadow`.

### 3.7 Detection identity, fold and the sink (`AlertSink`, `RecordingAlertSink`; S-09, L-17, S-52)

**Fingerprint.** `fingerprint = SHA-256(canonical encoding of {scenario_id, entity_kind, anchor, bucket})`, hex, lower case,
with the phase 02 canonical encoder (not `serde_json`). `anchor` is the customer id of the group's anchor (or the entity id
for account and counterparty scenarios). `bucket` is the bucket start date (aggregate and profile) or `event:<txn id>`
(event kind). The fingerprint does not contain the version, the run or the parameter values: a re-run, a replayed batch
or a version change under the same pattern produces the same key (DSL A.7: "scenario_id + entity + window bucket").

**Bucket fold** (aggregate and profile; `dedupe.same_entity_within`, default seven days). Under `pg_advisory_xact_lock`
keyed on (scenario, anchor), `DetectionKeyRepository::open_bucket(scenario, anchor_hk, since = window_to - days + 1)`
returns the newest key of the anchor whose bucket starts within the span. If one exists, the new detection reuses its
bucket and fingerprint: repeats fold into the open alert with every transaction listed (L-17). Otherwise the bucket starts
at `window_to`. `anchor_hk` is the keyed hash of the anchor (`hk` convention).

**Measures digest.** `measures_digest = SHA-256(canonical {measures, bound parameter values, matched transaction ids sorted})`.
The sink's outcome follows from the digest:
- the key is new: `New` (audit `detection.recorded`);
- the key exists and the digest is equal: `Duplicate` (no audit, no alert): a replayed batch stops here;
- the digest differs: `Amended(MeasuresChanged)` or `Amended(LateRowsAdded)` (audit `detection.amended`);
- the key exists with `matches = true` and the group no longer matches: `Amended(NoLongerMatches)`; `matches` is set false.

**The payload** (`DetectionPayload`, the contract phase 17 builds alerts from; ids, versions and counts only, except the
entity id that the sink receives inside the transaction):
- `fingerprint`, `fingerprint_inputs {scenario_id, entity_kind, entity_id, anchor_hk, bucket}`;
- `scenario_id`, `scenario_code`, `scenario_version_id`, `logic_version`, `parameter_set_id`, `parameter_set_version` (null
  for the document defaults), `parameter_values` (the values bound for this group);
- `mapping_version_ids`, `reference_versions` (calendar, country table, rating model, segment versions, feature versions),
  `cbi_pack_version`, `register_version`;
- `data_as_of` per source (phase 11 `FreshnessService`), `window_from`, `window_to`, `run_id`, `mode` (`incremental`,
  `settled` or `sweep`; a shadow run never reaches the sink);
- `measures` (alias to value), `matched_txn_ids` (the first 500 ids; `matched_count` gives the total), `linked_party_ids`
  and `links` (each link with its kind: `family_no`, `phone`, `address`, `signatory`, `owner`);
- `reason_code`, `severity` (the outputs rule of DSL A.8), `case_family`, `template`, `degraded` with `degraded_reasons`;
- `coverage {dark_units, excluded_units, exclusion_ids, blind_period_ids, stale_sources}` (section 3.8);
- `matches` (false only for an amendment that says the pattern no longer holds), `emitted_at`.

**`RecordingAlertSink`** (phase 16): writes `detection_keys` through `insert_or_get` and the outcome above, and returns the
outcome. It does not create alerts. Phase 17 replaces it in AppState with the alert service and keeps the same outcome
contract. Phase 17 keeps `detection_keys` as the identity table (phase 17 DR-117) and adds `AlertSinkError` variants: on
`Quarantined(request)` the runner records the request through phase 17 `AlertQuarantineService::record_standalone` in a new transaction and
continues; on `Partition` it calls `alert.create_partition` for the month and retries the emit once; on `Storage` the bucket is retried by
the run's own retry; on `Db` the bucket aborts (phase 17 section 4.5).

### 3.8 Coverage, calendars and linked parties in a run (`CoverageContext`, `LinkedPartyResolver`; S-08, S-13, L-01, S-03)

1. **Dark units** (phase 12 `admin.coverage_ledger`): a unit with `covered = false` in the window is dark. A group with any
   dark unit is `degraded` and its quiet verdict is recorded as `unverified` in the shadow and replay summaries, never as
   quiet. The ledger is read, never written, by this phase (phase 12 defers scenario use to phases 13 to 16).
2. **Exclusions** (phase 12 `admin.exclusions` with `ended_at IS NULL` and `expires_at > now`): the rows of an excluded unit
   are removed from every measure of the statement (the excluded unit list is bound). The run lists the exclusion ids on
   the detection. Reach counts still include excluded rows (DSL A.7).
3. **Blind periods** (phase 11 `admin.blind_periods`): a window overlapping a source-scope or system-scope blind period
   carries its ids. The alert is still raised; the period is shown with it (phase 18 shows the banner on cases).
4. **Freshness** (phase 11 `FreshnessService`): `data_as_of` per source is recorded. A scenario reading a stale source is
   shown "not operating as intended" (DSL A.3) and is never scored low because of the stale source.
5. **Calendar-aware windows.** `window.calendar` names a calendar of phase 09. A window of N business days ending at `d` starts
   at `BankCalendarClock::working_days_back(d, N, calendar)`, the backward walk over the same day set and weekend pattern
   that phase 09 section 3.3 uses forward (an extension of the same pure function; Open doubts). Only the local date is used;
   Hijri dates are display text (phase 09 rule). A calendar change never moves a window of a run already started (the
   calendar version is recorded in the snapshot).
6. **Settled time.** `settle_delay` is applied as the job's `run_at` after the batch (section 3.6, fan-out step 3), so late rows of the
   same business moment are included (C-22).
7. **Linked parties** (`LinkedPartyResolver`, S-08). One hop per requested type, current rows only:
   - `family_no`: equal `canon.party.family_no_hk` between two customers' parties. Kinship widens the group for the alert
     and is named on it; the rating (phase 15) still excludes kinship-only links from automated scores (T-24, DSL A.4).
   - `phone` and `address`: equal `value_hk` in `canon.party_contact` of the same `type` (`mobile` or `landline` for phone,
     `address` for address); a contact with a null `value_hk` makes no link.
   - `signatory` and `owner`: a `canon.account_party_role` row of the account with that role (`signatory`, `owner`).
   - `companion` and `depositor`: no canonical link exists in phase 10 (`canon.cash_event` has no depositor column; Open
     doubts). A document naming them is refused at validation (section 3.1 rule 12).
   The group's anchor is its smallest customer id; `links` on the payload name every link used.

### 3.9 Replays and the cost guard (`ReplayService`, job `replay_run`; S-41, TS-15; DSL A.11)

**Request** (`POST /scenarios/{id}/replays`; Cedar `scenario.replay.run`, CFG; scope `config.author`; `Idempotency-Key`):
1. The version is draft, testing, shadow or live (superseded and retired are `version_closed`). `period_to` is at least
   `period_from`; the period is at most `scenario.replay_max_months` (`E_WINDOW_TOO_LONG` at `/period_to`).
2. Parameters: `parameter_set_id` (must belong to the version) or `parameter_values` (a draft set, bound as given, validated by
   section 3.5 step 2). The baseline is the live version (`baseline: "live"`) or none.
3. **Estimate.** For each partition of `canon.txn` overlapping the period, `EXPLAIN` of the statement bound to that partition's
   range. The sum of estimated rows and cost is compared with `scenario.replay_cost_max`. Above it, the request is refused
   with `E_COST_GUARD` (422) and the body holds the estimate: `{period_from, period_to, partitions, estimated_rows,
   estimated_cost, bound}`. No replay row is written; audit `replay.refused_cost`.
4. Within the bound: insert the `replays` row (`status` queued, `parameter_values` snapshot, `estimate`, `requested_by`),
   create the replay-result partitions for the period's months through `scn.create_partition`, enqueue `replay_run`, audit
   `replay.requested {replay_id, version_id, period, baseline}`. Answer 202 with `Location: /replays/{id}`.

**Run** (job `replay_run {replay_id}`; coalesce `replay:<id>`; resumable):
1. Read the cursor. Groups are read in keyset order (anchor id) in chunks of 5,000 with activity in the period.
2. Evaluation points: each business day on which the group has a posting in the period (the incremental equivalent), plus
   each month-end for `L2` and `L3` versions (the sweep equivalent). At each point the statement is evaluated as of that
   point: rows with `recorded_from <= point` (the bi-temporal store, phase 10), feature values with `computed_at <= point`,
   segment memberships with `assigned_at <= point`, ratings from `RatingSource::levels(customers, point)`. The shadow of
   history is therefore what the engine would have said then (S-41).
3. Each point gives `would_alert`, `near_miss` and the measures. **Near miss** (the below-the-line population) is a non-alert
   whose primary measure reaches `scenario.near_miss_ratio` (default 0.8) of its threshold: the compiler emits the `having`
   predicate again with each numeric threshold multiplied by that ratio, as a second boolean column.
4. Each chunk writes its `replay_results` rows (alert and near-miss rows only, each with its entity id; masking is a read
   rule, below) and the counters, and advances the cursor, in one transaction (`ReplayRepository::append_chunk`). A retry resumes
   at the committed cursor, so no row is written twice.
5. **Look-ahead.** Any input with a timestamp later than its evaluation point (a feature value recomputed afterwards, a
   membership assigned later) is counted in `look_ahead_inputs` and sets `look_ahead` true. The as-of reads of step 2 keep
   this at zero; a non-zero count means a source is not bi-temporal and the summary says so (S-41).
6. **Baseline and effect.** With a live baseline, the same points are evaluated against the live version; `overlap` is the
   number of replay alerts whose detection key the live version also produces. `alert_delta` = replay alerts minus baseline
   alerts; `effect` is `increase` (delta above zero), `decrease` (below zero) or `neutral`. Without a baseline, `alert_delta`
   is the replay's alerts and `effect` is `increase` above zero, else `neutral`. A completed replay therefore always has an
   effect: a baseline-less first version is an increase, never `unknown`.
7. **Summary** (`result_summary`, counts only): `evaluated`, `would_alert`, `near_miss`, `unverified` (dark), `degraded`,
   `look_ahead_inputs`, `reach {groups, alerting_groups}` (reach counts every evaluated group, excluded and masked ones
   included), `overlap`, and the cells by segment reference, branch and month. A cell below `scenario.summary_min_cell` is
   returned as `null` by the API (section 2) so that a single suspicion-tier alert cannot be read off a cell.
8. Finish: status `completed`, `finished_at`, `alert_delta`, `effect`, `look_ahead`; audit `replay.completed {replay_id,
   evaluated, would_alert, near_miss, look_ahead}`. On failure: `failed`, audit `replay.failed {replay_id, error_code}`.
9. Reader replays (section 3.14) are the same job with a `proposal_hash` and the proposal applied; they never count toward gate
   G4.

**Reading a replay** (`GET /replays/{id}`; Cedar `scenario.replay.read`: CFG, MRO, CHK, M, CH). The summary is returned to
these roles; the per-entity results (`GET /replays/{id}/results`, Cedar `scenario.replay.results.read`: CFG, MRO, M, with
`X-Access-Purpose: qa_review`) return the alert and near-miss rows of non-suspicion parties. The tier is read at request time
from the party (`canon.party.sensitivity_tier`, phase 10), never stored on the row, so a party that becomes suspicion-tier after
the replay is hidden too. Suspicion-tier rows are returned to M only; to others they are absent, not masked-404 (the rows are a list, and existence is not revealed by a list). The
read writes one access event per response (phase 03).

### 3.10 Sampling plan and blind labels (`SampleBuilder`; S-42, S-41; DSL A.9)

**Plan** (`POST /replays/{id}/samples`; Cedar `scenario.sample.plan`, CFG; job `replay_sample_plan`):
1. The replay is `completed`; one plan per replay (`plan_exists` otherwise). Lines default to `above` and `below`.
2. **Strata**: segment reference, branch and business month of the replay rows (section 3.9 step 7).
3. **Size per stratum.** Below the line (near misses, the rare-event side): `n = min(N, max(scenario.sample_min_per_stratum,
   ceil(3 / tolerance)))`, where `tolerance = scenario.btl_tolerance_pct / 100` (the rule of three: with zero misses in `n`
   items the 95 per cent upper bound on the miss rate is about 3/n). Above the line (alerts, the precision side): `n = min(N,
   max(scenario.sample_min_per_stratum, 50))`. A plan is capped at 5,000 items.
4. **Draw.** A fresh 32-byte `plan_seed` is stored on every row of the plan. Within a stratum, replay rows are ordered by
   `SHA-256(plan_seed || replay_result_id)` and the first `n` are drawn. Anyone holding the seed reproduces the draw.
5. **Seeded positives** are not drawn from customers. The seeded results of the candidate version (section 3.12) are recorded
   in the plan's audit detail (`seeded_summary`).
6. **Blind packets.** For each drawn item, an evidence object (phase 02, `kind = snapshot`, `owner_kind = replay_sample`,
   `owner_id` = the sample id, retention R9) holds the facts a reviewer needs to answer: the transactions of the window (dates,
   amounts and currency, channel and branch codes, transaction type), the linked-party kinds (not the other parties' names or
   ids), the customer's segment, CDD level and declared-profile band, and the questions. It never holds the outcome
   (`would_alert`, `near_miss`), a threshold, a parameter value, a reason code, the scenario id or code or family. A test
   searches every packet for those names (section 6).
7. **Masking.** The sample keeps its entity id. Its Cedar attribute `tier` is resolved at request time from the party
   (`canon.party.sensitivity_tier`, phase 10); a suspicion-tier sample and its packet are M's alone (section 3.18) and are not offered
   to A1 or A2.
8. Audit `sample.plan_created {replay_id, plan_id, lines, strata, sampled, seed_digest}`; the seed itself stays in the row.

**Label** (`POST /samples/{id}/labels`; Cedar `scenario.sample.label`: A1, A2, M; scope `queue.work`):
1. The reviewer is not the scenario's author (Cedar `resource.scenario_author == principal` is refused; the author would know
   the design) and, for a suspicion-tier sample, is M.
2. `label` is `suspicious`, `not_suspicious` or `cannot_tell`; `note_code` is from the closed list of section 1.2. A second label
   by the same labeller is `E_NOT_ALLOWED_IN_STATE` `already_labelled` (the unique pair). The response is `{accepted: true}`:
   it never shows the outcome or the other labels. Audit `sample.labelled {sample_id, labeller_id}` (no label value in the
   audit detail; the value lives in `sample_labels`).
3. `GET /samples/{id}/packet` (NEW; Cedar `scenario.sample.label`) returns the packet's evidence stream to the same roles,
   with `X-Access-Purpose: qa_review`.

**Estimates** (`GET /replays/{id}`, section block `sampling`), computed from the labels:
4. Weight of an item: `w = stratum_population / stratum_sampled` of its stratum.
5. Above the line: precision estimate = the weighted share of `suspicious` among above-line items, with the labelled count and
   the share of `cannot_tell`.
6. Below the line: estimated suspicious non-alerts = the sum over below-line items of `w` where the label is `suspicious`.
7. Miss rate (below the line) = estimated suspicious non-alerts divided by the below-line population `N_below`. `cannot_tell`
   counts as not suspicious and is reported separately.
8. `within_tolerance` is true when the miss rate is at most `scenario.btl_tolerance_pct / 100` and every sample is labelled.
9. Gate G5 (section 3.4) reads `within_tolerance`. The estimates go to the change request as the `btl_sample` evidence.

### 3.11 Shadow runs and their comparison (`ShadowService`, job `shadow_compare`; S-44, DSL A.10, A.11)

**Start** (`POST /scenarios/{id}/shadow`; Cedar `scenario.shadow.run`, CFG; body `ShadowStart {scenario_version_id,
shadow_days_required?}`):
1. The version is `testing`; its unit cases pass (`tests_passed_at`); a completed replay exists for this version and its
   proposed set (`replay_missing` otherwise). The proposed set moves to status `proposed` if it is still `draft`.
2. `shadow_days_required` defaults to `scenario.shadow_min_days` and may be longer (for seasonal patterns, the bank chooses
   two cycles); a value below the floor is `E_VALIDATION` at `/shadow_days_required` (the DB CHECK also holds it at 30).
3. The version moves `testing` to `shadow` with `shadow_started_at` = now. Audit `scenario.shadow_started {version_id,
   shadow_days_required}`. Shadow runs start with the next batch and the next sweep.

**Runs.** Shadow versions run in mode `shadow` (section 3.6) and write `shadow_results` rows: `would_alert`, `detection_key` (the
fingerprint computed with the bucket fold of section 3.7, where the fold reads the live open buckets, read only, so that agreeing
alerts have equal fingerprints), the bucket and the business date. Shadow never calls the sink.

**Comparison** (job `shadow_compare`, schedule `schedule.shadow_compare`; one run per shadow version; per completed day `D`):
1. Shadow rows of day `D` with `would_alert` and a live key with `matches = true` are `agree_alert`; shadow alerts with no live key
   are `shadow_only`; shadow quiet rows with no live key are `agree_quiet`.
2. Live keys of day `D` with no shadow row get a `shadow_results` row with `would_alert = false`, `detection_key` = the live
   fingerprint and `outcome = live_only`.
3. `ShadowRepository::compare_day` writes the outcomes and `compared_at`; audit `shadow.compared {version_id, day, counts}`.
4. A day with unverified dark rows counts them in `unverified`, not in agree-quiet.

**Read** (`GET /scenarios/{id}/shadow`; Cedar `scenario.shadow.read`: CFG, MRO, CHK, M, CH): `ShadowView {version, started_at,
days_required, days_elapsed, days_compared, days: [{day, agree_alert, agree_quiet, shadow_only, live_only, unverified}],
ready}`. `ready` is true when the elapsed days reach `shadow_days_required` and every day is compared; gate G7 reads it. The
view carries counts only, so a suspicion-tier party is not visible in it.

**End of a shadow.** A shadow version leaves by going live (section 3.3) or by withdrawal (section 3.15, no change request). A
changed shadow is a new version (section 3.3).

### 3.12 Seeded known-answer tests and silence alarms (`SeededSuite`, job `seeded_suite`; S-49, L-16)

**Cases.** Each `scn.seeded_tests` row names a CBI item (`catalogue_item_id`), the scenario that implements it, a polarity and
a fixture of synthetic canonical rows (`fixture.rows` by canonical table, with stable ids) and the expected result
(`{alerts, severity?}`). Cases are installed from the document pack (section 3.15); a changed fixture is refused (rows are
immutable; a new case code is a new case).

**Run.** For each active case of the scenario:
1. Start a `scenario_runs` row (mode `seeded`, `trigger_ref` `schedule`, `release` or `manual`, `window` = the fixture window).
2. In one transaction: create temporary tables `pg_temp.fx_txn`, `pg_temp.fx_account`, `pg_temp.fx_customer` and the others the
   document reads, with the column lists of the closed catalogue; insert the fixture rows; `SET LOCAL statement_timeout`; run the
   compiled statement of the candidate version against them (the compiler's source map names the temporary tables; no document
   text is involved); read the alert count and the severity; roll back. The sink is never called: a seeded run emits nothing.
3. `pass` when the alert count and any stated severity equal `expected`; `fail` when they differ; `error` when the statement or
   the CEL evaluation fails.
4. Record on the row: `last_run_id`, `last_run_version_id` (the version tested), `last_run_at`, `last_result`; `consecutive_failures`
   plus one on `fail` or `error`, zero on `pass`. Audit `seeded_test.result {case_code, version_id, result}` on every `fail`,
   `error` and on the first `pass` after failures; `seeded_suite.completed {cases, pass, fail, error}` once per run.

**Schedules and triggers.** Daily (`schedule.seeded_suite_daily`) for the live versions; on every release (the pack seed enqueues
the suite); on `POST /seeded-tests/runs` (202) for a chosen version; and for the gate (G3) for the candidate version.

**Silence and flat-line (L-16, S-49; computed by `ScenarioMetrics` on read, section 3.16).** A live scenario with zero detections
for `scenario.silence_business_days` business days while its inputs were evaluated is `silent`. A scenario whose detection count
repeats the same value for `scenario.flatline_days` consecutive days is `flatline`. A failing `cbi` seeded case is red at once.
The nightly job audits transitions only (`scenario.silence_detected`, `scenario.flatline_detected`, `scenario.recovered`).

### 3.13 Kill switch (`KillSwitchService`; S-52, T-02; route `POST /scenarios/{id}/kill-switch`)

The kill switch falls back to the previous approved version. It never switches a scenario off (T-02; DSL A.10).

1. Cedar `scenario.kill_switch` (CHK, M) on the live version; scope `config.approve`; `Idempotency-Key`; `If-Match` with the
   live version's `version` (`E_PRECONDITION_REQUIRED` without it, `E_VERSION_CONFLICT` on a race).
2. `StepUpService::consume(tx, ctx, header, StepUpAction::ScenarioKillSwitch, Some("scenario:<id>"))` where
   `ScenarioKillSwitch` is the phase 16 constant `{name: "scenario.kill_switch", class: approval}` (phase 01 closed classes; no new
   class). Missing, expired or reused: `E_STEPUP_REQUIRED` (401).
3. Lock the scenario row (`FOR UPDATE`) and the live version. No live version: `E_NOT_ALLOWED_IN_STATE` `not_live`.
4. Find the fallback F: the version with `superseded_by_version_id` = L.id and status `superseded`, newest first. None:
   `E_NOT_ALLOWED_IN_STATE` `no_fallback`. The scenario keeps running live; the bank changes it by change request.
5. Move L to `retired` (`retired_at` now, `end_reason = 'kill_switch'`) and its active parameter set to `superseded`.
6. Move F from `superseded` to `live` (`reinstated_at` and `effective_from` now); F's most recent superseded parameter set moves
   back to `active` (the guard allows `superseded` to `active` for this case only).
7. No change request is written (DR-108). The fallback F and its parameter set were approved by an earlier change request,
   so no unapproved logic or value runs; and a phase 04 request cannot be recorded with its approver as its own maker (the
   `checker_id <> maker_id` CHECK, and an `effective` request needs a submitted route and a workflow instance). The body
   carries `reason_code` (family `change`, for example `CHG_CORRECTION`) and `reason_text`. The card, the metrics and D2 show
   "fallback active since <date>" in red until a new version goes live through the gate; the killed version never returns
   (a retired version stays retired; its document can be copied into a new draft).
8. Audit `scenario.kill_switched {scenario_id, killed_version_id, fallback_version_id, reason_code}`; outbox
   `scenario.changed {change: "kill_switch"}`. Answer 200 with `{killed, fallback}`.
9. A run that commits an emission after step 5 sees the version as retired under `FOR SHARE` (section 3.6, run step 9) and writes
   nothing.

### 3.14 Reader catalogue, change targets and the volume effect (`ScenarioReaderCatalogue`, `ScenarioTarget`, `ParameterSetTarget`, `CatalogueTarget`; phase 13 section 3.8; DR-53)

**`ReaderCatalogue::readers(object)`** (phase 13 port). Object kinds: `feature_definition` (a feature key), `segment` (a static or
risk-overlay key), `peer_build` (a peer group id). The readers are the scenario versions in status draft, testing, shadow or live
whose document reads the object: a `feat.<key>` in `data_requirements`, a `segment:<key>` or `risk:<n>` in an override `when`,
or a peer comparator of a profile. Each reader is `{scenario_version_id, status, scenario_code}`.

**`ReaderCatalogue::measure(object, proposal)`** (phase 13 port). `proposal_hash` is the SHA-256 of the canonical proposal JSON.
1. No readers: `Some(Neutral)` (phase 13's null rule, now true because no scenario reads the object).
2. For each reader, the completed replay with this `proposal_hash` and a live baseline (or none) is looked up. If any reader has
   none, the function returns `None` (the volume effect is `unknown`, not approvable as material) and enqueues a reader replay
   (job `reader_replay {object_kind, object_key, proposal_hash, reader_version_ids}`, coalesce `reader_replay:<hash>`; the
   replay applies the proposal to the reader: a segment proposal changes membership, a feature proposal replaces the feature for
   the replay, a peer proposal replaces the cluster membership).
3. With all readers measured: any reader `decrease` gives `Some(Decrease)`; otherwise any `increase` gives `Some(Increase)`;
   otherwise `Some(Neutral)`. The conservative order is deliberate: a decrease in any reader needs the second approver of phase 04.
4. Phase 13's `prepare` and `check_before_decision` call it again; the value moves from `unknown` to a measured effect once the
   reader replay completes (phase 13 must re-call at decision; Open doubts).

**Change targets** (phase 04 `ChangeTarget`; registered in process by `api` and `worker`; the table rows are DR-53's shape):

| Kind | Base class | Route | Generic create | Emergency | Families | Activation (inside the approval transaction) |
|-|-|-|-|-|-|-|
| `scenario` | material; `cr_material_validated` when `pack: cbi` | `cr_material` or `cr_material_validated` (by the class and the validated flag of the target) | yes (proposal = a version id) | no | change | section 3.3 activation; for `action = retire`, section 3.15 |
| `parameter_set` | minor or material (section 3.5) | `cr_minor` or `cr_material` (by class) | yes | no | tuning, change | section 3.5 activation |
| `cbi_catalogue` | material; validated | `cr_material_validated` | no (dedicated route, section 3.15) | no | change | section 3.15 install |

Common rules. `prepare` runs the kind's validator (section 3.1 for a version, section 3.5 for a set, section 3.15 for an
import) and the gate (advisory). `check_before_decision` re-runs them: a moved base or a changed document refuses the decision
(`E_STALE_BASE` or `E_VALIDATION`). No kind has an emergency path, so `revert` is never called (it returns
`ChangeError::NotRevertible`); the kill switch is not a change request (section 3.13). `on_closed` with `Rejected` or `Withdrawn`: a rejected version stays in shadow or testing as the author left it; a rejected
set becomes `rejected`; a rejected import becomes `rejected`. No route here uses `route_map` rows: the phase 04 defaults for
`material` (with the validated flag) and `minor` cover the three kinds, so `change.route_map` is not changed by this phase.

### 3.15 CBI catalogue importer, bank scenarios, retirement and the pack seed (`CatalogueImportService`, `ScenarioAdminService`, `ScenarioPackInstaller`; S-01, S-05, S-06, T-02, DR-65 of phase 06)

**Importer** (`POST /cbi-catalogue/imports`; Cedar `cbi_catalogue.import`, CFG; scope `config.author`; S-06):
1. Body `CatalogueImportCreate {catalogue_version, issuance_ref, evidence_object_id, adoption_date, release_notes {ar, en},
   items: [{ref, article, number, kind, title {ar, en}, text {ar, en}, counter_only, data_requirements}]}`. `evidence_object_id`
   is an evidence object of kind `upload` that phase 02 has staged and scanned clean, owner kind `cbi_import` (registered in
   section 4.3).
2. Checks: `catalogue_version` matches the phase 06 pattern and no other open import holds it (`uq_catalogue_import_open`);
   each `ref` matches `Art.N no.M` with the article and number ranges and the scenario/indicator rule of phase 06; no `ref` repeats;
   `title` and `text` are bilingual (`E_MISSING_I18N`); each `data_requirements` entry is a canonical attribute
   (`E_UNKNOWN_ATTRIBUTE`); at most 500 items (`E_VALIDATION`). The evidence object must exist and be clean (`E_DANGLING_REF`
   at `/evidence_object_id` otherwise).
3. One transaction: insert `catalogue_imports` (status `draft`, `item_count`, the staged items); create the change request of
   kind `cbi_catalogue` (`object_key = cbi_catalogue:<catalogue_version>`, `proposed = {import_id}`, reason family `change`,
   route `cr_material_validated`) and move the import to `proposed` with the request id. Audit `cbi_catalogue.imported
   {import_id, catalogue_version, item_count}`. Answer 201 with the change request reference.
4. **Install** (the target's activation, after MRO validates and CHK and M approve in parallel): the phase 06 installer (section 3.1,
   step 3, sub-steps 1 and 2: insert the missing items for the new version; supersede the old rows per ref, a ref absent from the new
   version staying active and listed as a warning). Audit `cbi_catalogue.applied`. The import becomes `applied`.
   New items have no scenario yet; the readiness view shows them D (`no_scenario`) until scenarios implement them (phase 06).

**Bank scenarios** (`POST /scenarios`; Cedar `scenario.create`, CFG; scope `config.author`; S-05):
1. `code` matches `SCN-<FAMILY>-9NN`, NN 901 to 999 (`E_NAMESPACE`), the family is one of the eighteen of phase 06. `owner_id` is the
   caller (an active CFG user, `owner_not_active` otherwise). `research_grade` is `D` (the bank's own scenarios have no research
   basis until their cases validate them; S-59). `visibility` is `normal` or `restricted` (restricted requires the CFG role).
2. One transaction: insert `scn.scenarios` (`pack = bank`, `register_version` = the installed register, status `registered`),
   the first draft version `1.0.0` with the given document (validated for schema only; errors are saved with the draft), and audit
   `scenario.created`. Answer 201. No change request: a bank scenario is a draft until it goes live through the gate (DSL Part D:
   the new bank scenario is material at go-live, not at creation).

**Retirement and withdrawal** (`POST /scenarios/{id}/retire-requests`; Cedar `scenario.retire.request`, CFG; scope `config.author`):
1. Body `RetireRequest {reason_code (family change), reason_text, effective_to?}`, or `{withdraw_version_id}` for a version that is
   not live.
2. `pack: cbi`: refused with `E_VALIDATION` `E_BASELINE_RETIRE` (T-02). `pack: akr`: `effective_to` is required (a temporary
   switch-off needs an end date, DSL A.10), and re-enabling needs a new version through the gate. `pack: bank`: allowed; its
   evidence is the metrics of section 3.16 (outcome evidence waits for phase 24).
3. A live version: one change request of kind `scenario`, `action = retire`, route `cr_material`; on activation the live version
   moves to `retired` (`end_reason = 'retired'`) and, for a bank scenario, the scenario row moves to `retired`. Audit
   `scenario.retire_requested` and then `scenario.retired`.
4. A draft, testing or shadow version (`withdraw_version_id`): no change request; the version moves to `retired` with
   `end_reason = 'withdrawn'` (it never evaluated a batch for the bank). Audit `scenario.version_withdrawn`.

**Pack seed** (`akr-ctl packs seed`, extended; the release manifest lists `packs/scenarios/documents.v1.0.0.json` with its
SHA-256 and signature; phase 06 section 3.1 step 1 applies to it):
1. Validate every document with section 3.1 (zero errors) and every fixture; a failure exits 1 and writes nothing.
2. One transaction (as `akr_app`): for each document with a new `logic_version` only, insert the version. A `pack: cbi`
   document of a scenario that has no live version yet (the first install) is installed at status `live` (`effective_from` now,
   `tests_passed_at` now, `change_request_id` NULL, audit `scenario_version.seeded {pack: "cbi", approval: "release_signature",
   logic_version}`), which is the baseline of S-01 and the install acceptance the bank signs (Open doubts). Every other new
   version (a `cbi` upgrade of a scenario that already has a live version, and every `pack: akr` document) is installed at status
   `shadow` (`shadow_started_at` now, `shadow_days_required = 30`) and goes live only through the gate and a `scenario` change
   request (DSL A.10 library releases), so a release never replaces a version the bank approved or tuned. `akr` versions never go
   live while `pack.akr_live` is off.
3. The default parameter sets of the pack are installed as `active` for a version installed live and as `proposed` for a version
   installed in shadow (the set its shadow runs bind) (`reason_code` `CHG_RISK_ASSESSMENT`, basis from the document,
   `band_status` computed); a default outside the hard bounds is `E_PARAM_OUT_OF_BOUNDS` and a default outside the band is
   `E_DEFAULT_OUTSIDE_BAND`; both refuse the seed.
4. Seeded cases are upserted on `(scenario_id, case_code)` with `ON CONFLICT DO NOTHING`; a fixture that differs from the installed one
   is refused (cases are immutable).
5. Enqueue `seeded_suite {reason: "release"}` (coalesce `seeded_suite:release`). Audit `cbi_pack.scenarios_seeded {counts, added}`;
   a second run with the same files writes nothing and reports zeros.
6. `akr-ctl packs verify-data` adds to phase 06's checks: every active CBI item has at least one positive and one negative
   seeded case (names carry the article and number); every `cbi` document has a live version or is listed as `no logic`; no
   live `akr` version while `pack.akr_live` is off (exit 2 on any failure; `no logic` is a listed warning).

### 3.16 Card, metrics and coverage matrix (`ScenarioCardRenderer`, `ScenarioMetrics`, `CoverageMatrixQuery`; S-07, S-45, S-53, S-54)

**Card** (`GET /scenarios/{id}/card`; Cedar `scenario.card.read`: CFG, MRO, CHK, M, CH, IA; a restricted scenario: CFG and MRO
only). `ScenarioCardRenderer` (pure) builds, from the register, the document, the tests and the monitoring:
- identity (id, code, family, pack, research grade, evidence label; `N` marked "not yet validated on this bank's cases");
- purpose (the register summary, bilingual);
- logic in words: the sentences generated from the document's AST by the renderer (DSL A.12), never typed text, so they cannot
  drift from the logic (S-50); the example of DSL A.12 is the test fixture;
- data requirements with the readiness grade and the missing field (phase 06 readiness);
- parameters: default, bank value (the active set), basis, `review_by`, band, overrides, and the `W_DP_EXPIRED` mark;
- exclusions and suppression in words; the tests (counts and the last seeded result); performance (section below);
- change history (change requests with class, volume effect and status); CBI links with their coverage kinds (`full`,
  `partial`, `proxy`); version status (live since, shadow days, superseded and kill-switch history).
The card contains no customer or account data.

**Metrics** (`GET /scenarios/{id}/metrics`; Cedar `scenario.metrics.read`: CFG, MRO, CHK, M, CH, IA; scope `dashboard.read`,
`config.author`, `config.approve`). Computed on read over the last 30 days of runs and detection keys; no metrics table:
- volume: detections per day and per week; stability: week-over-week change (amber at 50 per cent, red at 100 per cent);
- reach: entities evaluated per run and in the newest replay;
- timeliness: the median and 95th percentile of seconds from the batch time to the emission (`detection_keys.first_seen_at` minus
  the batch time), amber and red at the settled-time delay plus one hour and two hours;
- cost: run duration and planner cost; amber at 80 per cent of `scenario.statement_timeout_s`, red at 100 per cent;
- coverage: the dark share and the degraded share of evaluated groups (amber 10 per cent, red 25 per cent);
- silence (red at `scenario.silence_business_days`), flat-line (red at `scenario.flatline_days`), failing seeded cases (red at one
  failing `cbi` case);
- last run: failed or completed.
The outcome measures of S-53 (productivity, uniqueness, outcome, misses, quality, fairness) need alerts, cases and reports
(phases 17 to 21) and are phase 24's (Open doubts). The nightly job `scenario_metrics` audits only the transitions of silence,
flat-line and recovery (section 3.12).

**Coverage matrix** (`GET /coverage/matrix`; Cedar `coverage.matrix.read`: M, CH, IA; scope `dashboard.read`). Rows are the active
CBI items (phase 06 `CatalogueRepository::active`); each row lists its scenario links (phase 06 `scenario_catalogue_links`) with the
live version's status and the scenario's bank readiness grade (phase 06 `readiness_grades`, bank scope); the term is Operable for A,
Degraded for B and Blocked for C and D (C-48, phase 06). The columns `alerts`, `cases` and `strs` are returned as `null` with
`pending: [17, 18, 21]`; phases 17, 18 and 21 add them to the same route without a new row. Filters: article, term, grade. Restricted
scenarios are omitted for readers other than CFG and MRO. No customer data appears.

### 3.17 Errors and audit events

**Problem codes** (API.md section 6 only): `E_VALIDATION` (422; `errors[]` with the DSL codes of section 3.1 and of
gates G2, G4, G5 as `E_TESTS_MISSING`, `E_EVIDENCE_MISSING`, `E_EVIDENCE_STALE`), `E_COST_GUARD` (422; estimate in the body),
`E_NOT_ALLOWED_IN_STATE` (409; `reason` one of `not_live`, `no_fallback`, `draft_open`, `version_closed`, `replay_missing`,
`shadow_too_short`, `seeded_not_passing`, `seeded_missing`, `readiness_blocked`, `btl_tolerance_exceeded`, `replay_outside_range`,
`pack_not_agreed`, `plan_exists`, `already_labelled`), `E_VERSION_CONFLICT` (409, `If-Match`), `E_PRECONDITION_REQUIRED` (428),
`E_STEPUP_REQUIRED` (401), `E_NOT_FOUND` (404; a restricted scenario for a caller outside CFG and MRO, byte-identical to a missing
one), `E_FORBIDDEN` (403), `E_SCOPE` (403), `E_SELF_APPROVAL` (403, phase 04), `E_STALE_BASE` (409, phase 04),
`E_IDEMPOTENCY_REQUIRED` (400), `E_IDEMPOTENCY_REUSE` (422), `E_PURPOSE_REQUIRED` (422, the results and packet reads),
`E_UPLOAD_REJECTED` (422, a catalogue issuance that fails the antivirus check at staging; phase 02).

**Audit events emitted** (one per mutation; payloads carry ids, versions, counts and codes, never a customer value, a label value
or a note):

| Event kind | On | Resource |
|-|-|-|
| `scenario.created` | `POST /scenarios` | scenario |
| `scenario.version_opened`, `scenario.version_saved`, `scenario.version_tested`, `scenario.version_withdrawn` | version routes and withdrawal | scenario_version |
| `scenario_version.seeded` | pack seed (approval `release_signature`) | scenario_version |
| `scenario.shadow_started` | `POST .../shadow` | scenario_version |
| `scenario.version_activated` | activation of a `scenario` change request | scenario |
| `scenario.kill_switched` | `POST .../kill-switch` | scenario |
| `scenario.retire_requested`, `scenario.retired` | retirement | scenario |
| `parameter_set.drafted`, `parameter_set.activated` | `POST .../parameter-sets` and its activation | parameter_set |
| `replay.requested`, `replay.refused_cost`, `replay.completed`, `replay.failed` | replay routes and job | replay |
| `sample.plan_created`, `sample.labelled` | sampling and label routes | replay_sample (plan id) |
| `shadow.compared` | `shadow_compare` (counts) | scenario_version |
| `seeded_suite.completed`, `seeded_test.result` | seeded suite | seeded_test |
| `scenario.run_completed`, `scenario.run_failed`, `scenario.run_refused_cost` | `scenario_run` (counts, codes, estimate) | scenario_version |
| `detection.recorded`, `detection.amended` | `RecordingAlertSink` | detection_key (fingerprint) |
| `scenario.silence_detected`, `scenario.flatline_detected`, `scenario.recovered` | `scenario_metrics` transitions | scenario |
| `cbi_catalogue.imported`, `cbi_catalogue.applied` | importer and its install | catalogue_import |
| `cbi_pack.scenarios_seeded` | pack seed | system |

Reused: `change_request.*` (phase 04), `step_up.*` (phase 01), `evidence.staged` (phase 02), access events and `authz.denied`
(phase 03).

### 3.18 Cedar actions and the file `82_scenarios.cedar`

Phase 06 owns `80_detection.cedar` (actions `scenario.list`, `scenario.read`, `cbi_catalogue.read`) and is not edited. This phase
adds the file `82_scenarios.cedar` (phase 03 reserves the range 80 for detection). Entities added to `schema.cedarschema`
in the phase 03 form (roles are `principal.roles`; there is no Role entity): `ScenarioVersion { status: String, scenario: Scenario,
author: User }`, `ParameterSet { status: String, scenario: Scenario, proposed_by: User }`, `Replay { status: String, scenario:
Scenario, requested_by: User }`, `ReplaySample { line: String, tier: String, scenario: Scenario, scenario_author: User }`,
`CatalogueImport { status: String }`. The phase 06 `Scenario` entity already has `pack`, `family` and `visibility`.

Actions, with `appliesTo` the resource types named: `scenario.create` (System), `scenario.version.read`, `scenario.version.write`,
`scenario.parameter_set.read`, `scenario.parameter_set.write`, `scenario.replay.run`, `scenario.replay.read`,
`scenario.replay.results.read`, `scenario.sample.plan`, `scenario.sample.label`, `scenario.shadow.run`, `scenario.shadow.read`,
`scenario.kill_switch`, `scenario.card.read`, `scenario.metrics.read`, `scenario.retire.request`, `scenario.seeded.read`,
`scenario.seeded.run`, `cbi_catalogue.import`, `coverage.matrix.read` (System). Each name is a constant of the phase 03 action
enumeration; the bundle test asserts the two sets are equal.

```cedar
// 82_scenarios.cedar (phase 16). Roles are principal.roles. Step-up is not a Cedar condition (the handler consumes it).

// Authors: scenario logic, parameter drafts, replays, shadow starts, seeded runs, retirement proposals, importer.
@id("scenario.authors") @role("CFG")
permit (principal, action in [Akr::Action::"scenario.create", Akr::Action::"scenario.version.write",
        Akr::Action::"scenario.parameter_set.write", Akr::Action::"scenario.replay.run",
        Akr::Action::"scenario.sample.plan", Akr::Action::"scenario.shadow.run",
        Akr::Action::"scenario.retire.request", Akr::Action::"cbi_catalogue.import"], resource)
when { principal.roles.contains("CFG") };

@id("scenario.seeded_run") @role("CFG,MRO")
permit (principal, action == Akr::Action::"scenario.seeded.run", resource)
when { principal.roles.containsAny(["CFG", "MRO"]) };

// Readers of the register's versions, the card and the metrics: the authors, the approvers and the oversight roles
// (scope: GET /scenarios/{id} admits all six roles through config.author, config.approve and dashboard.read).
@id("scenario.version_readers") @role("CFG,MRO,CHK,M,CH,IA")
permit (principal, action in [Akr::Action::"scenario.version.read", Akr::Action::"scenario.card.read",
        Akr::Action::"scenario.metrics.read"], resource)
when { principal.roles.containsAny(["CFG", "MRO", "CHK", "M", "CH", "IA"]) };

// Parameter sets, replay summaries, shadow comparisons and seeded results: the authors, model risk and the approvers.
// The routes admit exactly these roles through config.author and config.approve (section 2).
@id("scenario.approver_readers") @role("CFG,MRO,CHK,M,CH")
permit (principal, action in [Akr::Action::"scenario.parameter_set.read", Akr::Action::"scenario.replay.read",
        Akr::Action::"scenario.shadow.read", Akr::Action::"scenario.seeded.read"], resource)
when { principal.roles.containsAny(["CFG", "MRO", "CHK", "M", "CH"]) };

// Per-entity replay rows: the authors and model risk, and the MLRO. Other readers never list a row (section 3.9).
@id("scenario.replay_rows") @role("CFG,MRO,M")
permit (principal, action == Akr::Action::"scenario.replay.results.read", resource)
when { principal.roles.containsAny(["CFG", "MRO", "M"]) };

@id("scenario.coverage_matrix") @role("M,CH,IA")
permit (principal, action == Akr::Action::"coverage.matrix.read", resource)
when { principal.roles.containsAny(["M", "CH", "IA"]) };

// Approvers of the kill switch: the MLRO and the compliance approver, never through the author's role alone.
@id("scenario.kill_switch") @role("CHK,M")
permit (principal, action == Akr::Action::"scenario.kill_switch", resource is Akr::ScenarioVersion)
when { principal.roles.containsAny(["CHK", "M"]) };

// Blind labelling: analysts and the MLRO; the author of the scenario never labels its own samples.
@id("scenario.sample_label") @role("A1,A2,M")
permit (principal, action in [Akr::Action::"scenario.sample.label"], resource is Akr::ReplaySample)
when { principal.roles.containsAny(["A1", "A2", "M"]) };

@id("scenario.sample_label.not_author") @deny("self_label")
forbid (principal, action == Akr::Action::"scenario.sample.label", resource is Akr::ReplaySample)
when { resource.scenario_author == principal };

// Suspicion-tier samples are the MLRO's alone (the phase 15 rule, extended to replay samples; DR-105 of phase 15).
@id("scenario.sample_label.suspicion") @deny("tier")
forbid (principal, action == Akr::Action::"scenario.sample.label", resource is Akr::ReplaySample)
when { resource.tier == "suspicion" && !principal.roles.contains("M") };

// Restricted scenarios (phase 06 visibility) are the authors' and model risk's: others get the masked 404.
@id("scenario.restricted_masked") @deny("masked")
forbid (principal, action in [Akr::Action::"scenario.version.read", Akr::Action::"scenario.version.write",
        Akr::Action::"scenario.parameter_set.read", Akr::Action::"scenario.parameter_set.write",
        Akr::Action::"scenario.replay.run", Akr::Action::"scenario.replay.read",
        Akr::Action::"scenario.replay.results.read", Akr::Action::"scenario.sample.plan",
        Akr::Action::"scenario.sample.label", Akr::Action::"scenario.shadow.run",
        Akr::Action::"scenario.shadow.read", Akr::Action::"scenario.kill_switch", Akr::Action::"scenario.card.read",
        Akr::Action::"scenario.metrics.read", Akr::Action::"scenario.retire.request", Akr::Action::"scenario.seeded.read",
        Akr::Action::"scenario.seeded.run"], resource)
when { (resource has scenario && resource.scenario.visibility == "restricted")
       && !principal.roles.containsAny(["CFG", "MRO"]) };

// The author of the live version never pulls its kill switch.
@id("scenario.kill_switch_not_author") @deny("self_approval")
forbid (principal, action == Akr::Action::"scenario.kill_switch", resource is Akr::ScenarioVersion)
when { resource.author == principal };
```

**Bundle tests** (`tests/policy/82_scenarios`): CFG creates a bank scenario and drafts a version; A1 is refused the version write by scope (`E_SCOPE`); CHK reads a version and cannot run a replay (`E_FORBIDDEN`); M reads a replay's per-entity rows and CHK does not; a restricted scenario answers the masked 404 to A2, IA and CH on every action of the `scenario.restricted_masked` forbid, and CFG and MRO read it normally; A2 is refused a suspicion-tier sample with the masked 404 (the `@deny("tier")` forbid, which phase 03 answers as masked), and M labels it; the author is refused its own sample (`self_label`); a version's author is refused its own kill switch (`self_approval`); MRO runs the seeded suite and cannot write a draft (no CFG role); the list of actions in the file equals the phase 03 action enumeration in both directions.

### 3.19 Tipping-off review (DR-105 of phase 15; phase 06 line 748)

- **Restricted scenarios** (the public-funds family and others the bank marks restricted) are invisible outside CFG and MRO on every
  route of this phase: masked 404, identical to a missing scenario. A restricted scenario's cards, matrix rows, replay summaries and
  shadow counts are withheld from the same roles. The open point is the approvers of such a scenario (Open doubts).
- **Replays and shadow runs** contain alerts that would have been raised. Suspicion-tier parties are never listed to anyone but M
  (section 3.9); the summaries count them only in cells of at least `scenario.summary_min_cell`; the shadow view counts them and names
  no one.
- **Blind samples.** A reviewer sees a packet with transaction and profile facts only. The packet carries no outcome, threshold,
  parameter value, reason code or scenario identity, so a reviewer learns nothing about which detection the sample came from. A
  suspicion-tier sample is M's alone.
- **Detections.** The payload is internal (phase 17 builds alerts from it); the alert's visibility follows the scenario's
  `classification` and visibility (DSL section 1, S-29) and is phase 17's rule. Branch roles never receive a detection; they see
  reason codes without thresholds (DSL A.8, phase 22).
- **Coverage.** The matrix and the card state which typologies are blind (dark units, exclusions, blind periods, a scenario without
  live logic). They are detection logic, readable by the roles of section 3.18 only, never by branch, operations, KYC or the
  front line.
- **Audit.** Events carry ids, versions, counts and codes. A label value, a note and a customer value appear in no audit detail.
- The answer for a person outside these roles is nothing: the routes refuse at the scope gate (403 `E_SCOPE`) or answer the masked
  404 on a restricted scenario, and the matrix and card never reach them.

### 3.20 Settings, schedules, flags and the reason codes this phase uses

Settings and schedules of migration 0057 (section 1.4). Governed (floor where a floor is stated): `scenario.history_years`,
`scenario.shadow_min_days`, `scenario.replay_min_months`, `scenario.btl_tolerance_pct`, `scenario.near_miss_ratio`. Ungoverned
operating bounds: `scenario.replay_max_months`, `scenario.hot_window_days`, `scenario.cost_max_total`, `scenario.cost_max_rows`,
`scenario.replay_cost_max`, `scenario.statement_timeout_s`, `scenario.summary_min_cell`, `scenario.flatline_days`,
`scenario.silence_business_days`, `scenario.sample_min_per_stratum`. Schedules: `schedule.scenario_sweep_daily`,
`schedule.scenario_sweep_monthly`, `schedule.shadow_compare`, `schedule.seeded_suite_daily`, `schedule.scenario_metrics`. Feature flag:
`pack.akr_live` (off). Read from phase 04: `change.evidence_max_age_days`, `change.route_map`.
The dedupe span is the scenario's own `dedupe.same_entity_within` (DSL A.7), not a setting.

Reason codes this phase cites (families as phase 04 and phase 09 seed them): `TUNE_RECALIBRATION_PCT`, `TUNE_BTL_RESULT`,
`TUNE_NEW_TYPOLOGY`, `TUNE_DATA_CHANGE` (risk), `TUNE_FALSE_POSITIVE`, `TUNE_VOLUME_CAPACITY` (capacity) for parameter sets;
`CHG_CORRECTION` (kill switch), `CHG_RISK_ASSESSMENT` (default sets at install), `CHG_REGULATORY_ORDER` (catalogue issuance) and the
other `CHG_*` codes of phase 04 for changes; detection codes of the documents are in family `detection` (section 3.1 rule 14; the
prefix question is in Open doubts).

## 4. Infrastructure Updates

### 4.1 Job kinds (Postgres `jobs`)

| Kind | Binary | Payload | Coalesce key | Concurrency | Attempts | Schedule or trigger |
|-|-|-|-|-|-|-|
| `scenario_touch_fanout` | akr-worker | `{batch_id, touch_id, source_id, entity_kinds, business_date}` | `scenario_fanout:<touch_id>` (7.9) | 2 | 5 | outbox `canon.touched` (phase 11; consumer registered here, section 3.6) |
| `scenario_run` | akr-worker | `{scenario_version_id, mode: incremental, settled, shadow or sweep, batch_id?, touch_id?, window_to?}` | `scenario_run:<version>:<mode>:<touch_id or window_to>` (7.9) | 4 | 5 | enqueued by the fan-out (`settled` with `run_at` = batch time plus the settle delay) and by the sweeps |
| `scenario_sweep` | akr-worker | `{latency_class: "L2" or "L3", business_date}` | `scenario_sweep:<class>:<date>` | 1 | 3 | `schedule.scenario_sweep_daily` (L2), `schedule.scenario_sweep_monthly` (L3) |
| `replay_run` | akr-worker | `{replay_id}` | `replay:<replay_id>` | 2 | 3 | `POST /scenarios/{id}/replays`; resumes at the committed cursor (section 3.9) |
| `replay_sample_plan` | akr-worker | `{replay_id, lines}` | `replay_sample:<replay_id>` | 1 | 3 | `POST /replays/{id}/samples` (section 3.10) |
| `shadow_compare` | akr-worker | `{scenario_version_id}` | `shadow_compare:<version>` | 1 | 3 | `schedule.shadow_compare` daily, one job per shadow version (section 3.11) |
| `seeded_suite` | akr-worker | `{scenario_version_id?, reason: "schedule", "release", "gate" or "manual"}` | `seeded_suite:<reason>:<version or all>` | 1 | 3 | `schedule.seeded_suite_daily`; the pack seed (`release`); `POST /seeded-tests/runs`; gate G3 (`gate`) |
| `scenario_metrics` | akr-worker | `{}` | `scenario_metrics` | 1 | 3 | `schedule.scenario_metrics` daily; audits the alarm transitions only |
| `reader_replay` | akr-worker | `{object_kind, object_key, proposal_hash, reader_version_ids}` | `reader_replay:<proposal_hash>` | 1 | 3 | `ReaderCatalogue::measure` when a reader has no replay for the proposal (section 3.14) |

No kind starts with `ingest_` (akr-ingestd's prefix). Every handler re-reads its owner before it acts (phase 00 rule). Each
`scenario_run` takes a transaction-scoped advisory lock per (scenario, anchor) only where it folds a bucket (section 3.7), so the
coalesce key and the lock never disagree about what runs once.

### 4.2 Timers, outbox, shipped data and the route map

- **Timers:** none new. The settled-time run is a job with `run_at` (section 3.6), so the phase 00 timer table is not used.
- **Outbox topics produced** (phase 00 topic registry): `scenario.changed {scenario_id, version_id, change}` where `change` is one of
  `activated`, `parameters`, `kill_switch`, `retired`. Ids and names only.
- **Outbox topics consumed:** `canon.touched` (phase 11) by `scenario_touch_fanout`.
- **Shipped data** (release manifest, `packs/scenarios/documents.v1.0.0.json`, verified as in phase 06): the documents of the CBI
  baseline (`pack: cbi`, live at first install, shadow on a later release), the documents of the AKR library (`pack: akr`, shadow at install), the default parameter
  sets, the unit fixtures and the seeded cases. The committed JSON is built once per research revision by a developer and reviewed,
  as phase 06 builds its register; the release only signs and lists it. Which of the 99 CBI and 62 AKR documents ship in this release
  is an Open doubt.
- **Reason codes:** this phase seeds none. The detection codes its documents cite are in family `detection` in the phase 09 seed file
  (inactive until the family change activates them, phase 09 section 3.4). The DET_ prefix question is in Open doubts.
- **Route map** (`change.route_map`): no rows (section 3.14).
- **Step-up constant:** `scenario.kill_switch` with class `approval` (phase 01 closed enumeration).

### 4.3 AppState additions, registries and OpenAPI

- `api`: `scenarios: Arc<ScenarioAdminService>`, `versions: Arc<ScenarioVersionService>`, `parameters: Arc<ParameterService>`,
  `replays: Arc<ReplayService>`, `samples: Arc<SampleService>`, `shadow: Arc<ShadowService>`, `kill: Arc<KillSwitchService>`,
  `seeded: Arc<SeededQuery>`, `catalogue_imports: Arc<CatalogueImportService>`, `card: Arc<ScenarioCardQuery>`,
  `metrics: Arc<ScenarioMetrics>`, `matrix: Arc<CoverageMatrixQuery>`, `validator: Arc<ScenarioValidator>`.
- Ports: `alerts: Arc<dyn AlertSink>` (`RecordingAlertSink` here; phase 17 replaces it), `readers: Arc<dyn ReaderCatalogue>`
  (`ScenarioReaderCatalogue` replaces the phase 13 null implementation), `event_eval: Arc<dyn EventEvaluator>` (the CEL adapter),
  `linked: Arc<dyn LinkedPartyResolver>`. The phase 13 `RatingSource` and `DeclaredProfileSource` are consumed as registered there.
- `worker`: `ScenarioFanout`, `ScenarioRunner`, `ReplayRunner`, `SampleBuilder`, `ShadowCompare`, `SeededSuite`, `ScenarioMetrics`,
  `ReaderReplayRunner`; the pure `ScenarioCompiler` and the phase 13 `ProfileComparator`.
- Change targets `scenario`, `parameter_set` and `cbi_catalogue` registered in the phase 04 `ChangeTargetRegistry` by `api` and
  `worker` (section 3.14).
- Evidence owner kinds (phase 02 registry): `replay_sample`, `cbi_import`.
- Phase 04 `EvidenceResolver` registrations (phase 04 section 3.2 "the rest from their phases"): `replay` resolves a
  `scn.replays` id in status `completed` (its age is `finished_at`); `btl_sample` resolves a replay id whose sampling plan is
  fully labelled (its age is the newest `labelled_at`); `shadow_comparison` resolves a scenario version id with at least one
  compared day (its age is the newest `compared_at`). Anything else is `evidence_unknown` (phase 04).
- Partition registry (phase 02): `scn.replay_results` and `scn.shadow_results` with the factory `scn.create_partition` and
  `ahead_months = audit.partition_ahead_months`.
- Masking registry (phase 03): `replay_sample` is a masked kind (its tier is the party's); `scenario_version`, `parameter_set`,
  `replay` and `catalogue_import` carry no party and are restricted by the scenario's visibility through the `masked` forbid of
  section 3.18, not as masked kinds.
- OpenAPI: tags `scenarios` (15 operations), `replays` (6), `seeded` (2), with `catalogue` (1 new) and `coverage` (1 new) and the
  two extended rows of section 4.4. The SU operation `kill-switch` declares `X-StepUp-Assertion` and the step-up scheme. Every POST,
  PUT carries `Idempotency-Key`; PUT versions, kill-switch carry `If-Match` (428 and 409); the results and packet reads document
  `X-Access-Purpose` and `Cache-Control: no-store`; POST routes that create a change request document 202 with `Location`.
  `make generate-api` regenerates the client (section 5).

### 4.4 Network zones, readiness, metrics, alerts and existing-table updates

- **Network zones:** no change. akr-api and akr-worker read `canon`, `feat` and `admin.coverage_ledger`, and write `scn`. Nothing reaches
  a CBS, the gateway zone or the screening lists.
- **Readiness:** akr-api `/readyz` adds `scenario_baseline`: at least one live `cbi` version exists (the CBI baseline is on). Without it
  the deployment is not ready, as phase 06 does for the register.
- **Metrics:** `akr_scenario_runs{mode, status}` (counter), `akr_scenario_run_seconds{mode}` (histogram), `akr_detections_total{scenario}`
  (counter; the scenario code is a bounded label, 161 plus bank codes), `akr_seeded_failures{pack}` (gauge), `akr_replay_running`
  (gauge), `akr_replay_refused_cost_total` (counter).
- **Alert rules:** a `cbi` document with no live version (page); a failing `cbi` seeded case (page); a run failed three times within one
  hour for one version (warning); a `flatline` or `silence` transition (warning); a replay refused on cost (information); a shadow
  comparison not run for a day (warning).
- **Phase 00:** the topic `scenario.changed`; the job kinds of section 4.1; the settings and schedules of migration 0057; the feature
  flag `pack.akr_live`.
- **Phase 01:** the step-up constant `scenario.kill_switch` (class `approval`); no new class.
- **Phase 02:** evidence owner kinds of section 4.3; the partition registry; access events for every per-entity read (the results
  and the packet), through the existing access layer. The audit catalogue (`audit.audit_log.resource_type` and the detail keys) gains
  the resource types `scenario`, `scenario_version`, `parameter_set`, `replay`, `replay_sample`, `seeded_test`, `detection_key`,
  `catalogue_import` and `system`, and the detail keys named in the payloads of section 3.7 and section 3.17; the detail carries no
  personal value (phase 02 rule).
- **Phase 03:** the Cedar file `82_scenarios.cedar` and the entities of section 3.18 in `schema.cedarschema`; the masked kind
  `replay_sample`; the bundle tests of section 3.18.
- **Phase 04:** the three change targets; no `change.route_map` row; no emergency path (the kill switch is not a change request).
- **Phase 06:** `GET /scenarios` gains the live and shadow version, the bank grade and the open-declaration flag; `GET /scenarios/{id}`
  gains versions, parameter sets and test summary and its scope gains `config.approve` and `dashboard.read` (section 2); `POST
  /cbi-catalogue/imports` joins the `catalogue` router and `GET /coverage/matrix` the `coverage` router. The phase 06 installer
  (section 3.1) is called by section 3.15.
- **Phase 09:** the backward working-day walk `BankCalendarClock::working_days_back` (section 3.8 step 5); a pure function over the
  same day set, with its own unit test.
- **Phase 10:** the two batch indexes of migration 0056 (physical indexes only).
- **Phase 11:** the consumer of `canon.touched`; no change to the topic or the freshness classes.
- **Phase 12:** read only (coverage ledger and exclusions).
- **Phase 13:** `FeatureCompiler::compile_measure` and `compile_filter` exposed for scenario use (section 3.2 step 2); `ProfileComparator::plan`
  and `evaluate` public; the `ReaderCatalogue` implementation registered in place of the null one.
- **Phase 15:** `RatingSource` consumed; the phase 15 `kyc` masking is unchanged.
- **Phase 16 own routes:** section 2 table. The two rows added to API.md section 3.11 are `GET /samples/{id}/packet` and `POST
  /scenarios/{id}/retire-requests` (`POST /scenarios` was already listed); two rows are extended (`GET /scenarios`, `GET /scenarios/{id}`,
  section 2); the read rows gain `config.approve` (and the card and metrics `config.author` and `config.approve`) so every role the Cedar
  policy admits passes the scope gate; the rest are the rows of API.md 3.11 unchanged, with `POST /samples/{id}/labels` kept at `queue.work` (section 3.10). `GET /scenarios/{id}/harness-diff`
  is not shipped here (phase 24, Open doubts).

## 5. Frontend Impact

Screens by catalogue ID (DESIGN-SYSTEM.md section 8.1):

- **D1 Scenario library and coverage matrix** (`/scenarios`, REGISTER). The register table: code, name, family, pack, live version and
  its status, shadow days, bank grade and the open-declaration mark (with the phase 06 vocabulary: operable, degraded, blocked).
  A second tab, the coverage matrix: CBI item to scenario to readiness, with the `alerts`, `cases` and `strs` columns shown as "from
  phase 17, 18, 21" until they arrive. "New bank scenario" opens the creation form (`POST /scenarios`). Restricted rows are absent for
  readers outside CFG and MRO and the matrix is absent for CFG and MRO (dashboard route). No specification in DESIGN-SYSTEM section 8.3
  exists for D1; this is the proposal, open doubt 28.
- **D2 Scenario studio** (`/scenarios/:id/studio`, STUDIO; DESIGN-SYSTEM section 8.3, mockup 05, UX-60). The stages strip maps the
  studio stages of DSL A.10 (author, test, replay, sample, shadow, approve, deploy, monitor). Parameters by segment with hard bounds and
  the approved band shown; the plain-language description "written automatically" (section 3.16); the test panel (unit and seeded results);
  the replay panel with the cost-guard message and the estimate on refusal, the summary with suppressed cells shown as "fewer than five",
  and the labelled sample estimates; the shadow panel (days, outcome counts, ready); the change-request panel with the computed class,
  volume effect and route (DESIGN-SYSTEM section 7.17); version history; the effective date. The example reason in the mockup is replaced by a
  risk reason, and capacity wording is flagged.
- Screens not touched: D3 policy studio (phase 09), S1 to S5 and K1 to K4 (phases 14 and 15), the E2 gap report (phase 06).

Generated-client hooks consumed (operationIds as section 4.3; Orval names follow the `useGet`/`usePost`/`usePut` pattern):
`useGetScenarios`, `usePostScenarios`, `useGetScenariosId`, `usePostScenariosIdVersions`, `usePutScenariosIdVersionsV`,
`usePostScenariosIdVersionsVValidate`, `usePostScenariosIdVersionsVTest`, `useGetScenariosIdParameterSets`,
`usePostScenariosIdParameterSets`, `usePostScenariosIdReplays`, `useGetReplaysId`, `useGetReplaysIdResults`, `usePostReplaysIdSamples`,
`usePostSamplesIdLabels`, `useGetSamplesIdPacket`, `usePostScenariosIdShadow`, `useGetScenariosIdShadow`, `usePostScenariosIdKillSwitch`,
`useGetScenariosIdCard`, `useGetScenariosIdMetrics`, `usePostScenariosIdRetireRequests`, `useGetSeededTests`, `usePostSeededTestsRuns`,
`usePostCbiCatalogueImports`, `useGetCoverageMatrix` (path-derived names, as phase 06 `useGetScenariosId`).
Phase 06 hooks `useGetCbiCatalogue` and `useGetCoverageReadiness` are reused on D1.

## 6. Verification

1. `make migrate`: migrations 0054 to 0057 apply cleanly, and a second run changes nothing (every `CREATE` is `IF NOT EXISTS`, every
   `INSERT` is `ON CONFLICT DO NOTHING`). Then `\d scn.replay_results` shows the partition key `created_at` and the primary key `(id,
   created_at)`.
2. `cargo check`: compile-time SQL verification passes for every repository of section 2.
3. `cargo clippy -- -D warnings`.
4. `cargo test`, including: (a) the three DSL worked examples 9.1 to 9.3 validate with zero errors (9.2 saved with empty unit tests
   validates, and leaving draft fails `E_TESTS_MISSING`); (b) the compiler golden: the SQL and `plan_hash` of SCN-CASH-02 equal the
   committed golden file, and a scenario measure equals the phase 13 `compile_measure` output for the same expression; (c) `VersionStatus::
   transition_allowed` equals the section 1.3 trigger on the full status matrix; (d) the fingerprint is stable across two runs and a
   replayed batch yields `Duplicate` at the sink; (e) the bucket fold (7-day span) folds a second alert of the same anchor and opens a new
   bucket after the span; (f) the rule-of-three sample size (300 for a 1 per cent tolerance); (g) the packet of a sample contains none of
   the parameter names, reason codes or scenario codes of its replay (a search over the rendered packet); (h) the card sentence of DSL
   A.12 is produced from the AST exactly.
5. Curl-test each route in `tests/curl/phase16_scenarios.sh` (users `cfg@`, `mro@`, `chk@`, `m@`, `ch@`, `ia@`, `ds@`, `a1@`, `a2@`,
   `kyc@`, `ss@`, `adm@`, `sec@`):
   - a. `POST /scenarios` as `cfg@` with code `SCN-CASH-901` returns 201; with `SCN-CASH-02` (outside 901-999) returns 422 with `errors[0].code = E_NAMESPACE`;
     as `a1@` returns 403 `E_SCOPE`.
   - b. `PUT /scenarios/{id}/versions/{v}` without `If-Match` returns 428 `E_PRECONDITION_REQUIRED`; with a stale value 409
     `E_VERSION_CONFLICT`; the same request twice with one `Idempotency-Key` returns the same body and writes one audit row; the same key with
     another body returns 422 `E_IDEMPOTENCY_REUSE`.
   - c. `POST .../validate` with an unknown attribute returns 200 with `errors[].code = E_UNKNOWN_ATTRIBUTE` and no state change; a parameter
     below its minimum returns `E_PARAM_OUT_OF_BOUNDS`.
   - d. `POST /scenarios/{id}/replays` for sixty months with `scenario.replay_cost_max` set to 1000 returns 422 `E_COST_GUARD` with the estimate
     in the body; no `replays` row exists afterwards.
   - e. `POST /scenarios/{id}/kill-switch` without the step-up assertion returns 401 `E_STEPUP_REQUIRED`; with it, as `chk@`, returns 200, the
     fallback is `live` and the killed version `retired` with `end_reason = kill_switch`; a second call on a scenario without a superseded
     version returns 409 `E_NOT_ALLOWED_IN_STATE` with `reason = no_fallback`.
   - f. `POST /scenarios/{id}/retire-requests` for a `cbi` scenario returns 422 with `errors[].code = E_BASELINE_RETIRE`.
   - g. `GET /scenarios/{id}` for a restricted (PUB) scenario: as `cfg@` 200; as `a2@`, `ia@` and `ch@` 404 `E_NOT_FOUND` with a body byte-identical
     to the one for a random id (masked 404).
   - h. `GET /replays/{id}/results` as `cfg@` returns no row of a suspicion-tier party, and as `m@` the same request returns those rows as
     well; after the party's tier is set to suspicion, `cfg@` no longer sees its rows; the access events of both are written with `purpose = qa_review`.
   - i. `POST /samples/{id}/labels` by the scenario's author returns 403 `E_FORBIDDEN`; by `a1@` on a suspicion-tier sample returns the masked 404
     `E_NOT_FOUND`; the same label twice by the same reviewer returns 409 `E_NOT_ALLOWED_IN_STATE` `already_labelled`; the response never contains
     the outcome.
   - j. `GET /samples/{id}/packet` as `a1@` returns the packet; the packet's body contains none of the strings `would_alert`, `near_miss`,
     the replay's parameter names and the reason codes of the scenario.
   - k. `POST /scenarios/{id}/shadow` before a replay exists returns 409 `E_NOT_ALLOWED_IN_STATE` `replay_missing`; after it, the version is `shadow`
     and `GET /scenarios/{id}/shadow` shows `ready = false` with `days_elapsed = 0`.
   - l. The go-live request (`POST /change-requests`, `object_kind = scenario`) for a version with no completed replay returns 422 with
     `errors[].code = E_EVIDENCE_MISSING`; the same request approved by its proposer returns 403 `E_SELF_APPROVAL` (phase 04).
   - m. `POST /cbi-catalogue/imports` with a ref `Art.9 no.1` for article 9 kind `scenario` returns 422 (the kind rule); a valid import returns 201 and
     creates one `cbi_catalogue` change request in `proposed`; as `cfg@` the approval attempt returns 403 `E_SCOPE` (config.approve is not in CFG's scope).
   - n. `POST /seeded-tests/runs` as `cfg@` returns 202; `GET /seeded-tests` then shows `last_result` per case and `consecutive_failures`.
   - o. `GET /scenarios/{id}/metrics` as `a1@` returns 403 `E_SCOPE`; as `ch@` returns 200 with the metric names of section 3.16.
   - p. `GET /coverage/matrix` as `cfg@` returns 403 `E_SCOPE`; as `ch@` returns 200 with `pending: [17, 18, 21]` on each alert, case and STR column.
6. Swagger shows the 23 new operations and the 2 extended ones; `make generate-api` regenerates the client with the hooks of section 5 and no other diff.
7. The audit trail for the curl session contains `scenario.created`, `scenario.version_saved`, `replay.requested`, `replay.refused_cost`,
   `scenario.kill_switched`, `sample.labelled` (without the label value), `seeded_suite.completed` and `cbi_catalogue.imported`; a
   `SELECT count(*) FROM audit.audit_log WHERE action = 'detection.recorded'` after a batch run and the same batch replayed equals the count
   before the replay (the replayed batch emits nothing).
8. For the database, `INSERT` of a second live version for one scenario fails with SQLSTATE 23505 (`uq_scenario_one_live`); an `UPDATE` of
   the `document` of a shadow version fails with `check_violation`; a `DELETE` on `scn.replay_results` fails with `insufficient_privilege`.
9. The read-only conformance: `akr_readonly` can `SELECT` from `scn.scenario_versions` and cannot `SELECT` from `scn.replay_results`,
   `scn.shadow_results` or `scn.detection_keys` (the customer-level tables).
10. The partition factory: `SELECT scn.create_partition('replay_results', 2030, 1)` succeeds once and returns the name; `SELECT
    scn.create_partition('scenario_versions', 2030, 1)` fails (not allow-listed).
11. `akr-ctl packs seed --dry-run` on the shipped documents prints the counts and writes nothing; `akr-ctl packs seed` run twice writes once
    (the second run reports zero added); `akr-ctl packs verify-data` exits 0 with every active CBI item holding a positive and a negative case,
    or exits 2 listing the items without one. A second release whose file carries a new `logic_version` of a `cbi` document that is live
    installs it in `shadow` with a `proposed` default set and leaves the live version and its active set unchanged.
12. Existing tests run with no regressions (phase 06 register and readiness tests, phase 13 feature tests, phase 04 change-request tests).

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 3b, from tools/agent/gap1-run3b.md; lead-verified (tools/agent/gap1-run3b-verdict.md).

### 7.1 Tier of request-time reads (phase 17)
From phase 17 on, the request-time tier reads of sections 3.9 and 3.10 and of the decisions use
`casework.effective_tier(party_id)`.

### 7.2 G9 and the Islamic pack (phase 26)
Phase 26 extends this phase and owns the extension: G9 reads `pack: akr` or `pack: islamic`, with the refusal
`pack_not_enabled` while `pack.islamic` is off; the seed skips a document whose `requires_flag` is off; `packs
verify-data` refuses an ISL document without a further condition.

### 7.3 Protected-attribute check on a scenario change (phase 24)
From phase 24 on, the `scenario` change target's `check_before_decision` runs `ProtectedAttributeCheck` (phase 24
section 3.5) on the proposed version; a finding refuses the decision as 422 `E_VALIDATION` with the `errors[]` code `E_PROTECTED_ATTRIBUTE` (the code of section 3.1 rule 6). The check is not part of `GoLiveGate`.

### 7.4 DetectionPayload source kind (DR-382)
From phase 22 on, `DetectionPayload` gains `source_kind` (`scenario` or `observation`); the runner of this phase writes
`scenario`. The scenario named by phase 22's `branch.hybrid_scenario_code` is a register scenario with
`observational = true` (phase 06), not a scenario of a family `observation`: a family code matches `^[A-Z]{2,5}$` and is
one of the 18.

### 7.5 The alert sink (phase 17)
Phase 17 replaces this phase's alert sink with `AlertIntakeService`, which writes the intake audit kinds. The
detection payload and the runner do not change.

### 7.6 Activation instant
A scenario version's activation instant is `scn.scenario_versions.effective_from`; a reader that needs the moment a
version went live reads that column. This closes phase 24 Open doubts 30.

### 7.7 Reader keys of the replay (phase 30, DR-327)
Phase 30 adds the keys `read_source` and `replay_lag_s` to `scn.scenario_runs.cost` (DR-327, standby reads for
scenario runs); the existing keys do not change.

### 7.8 harness-diff removed (DR-380)
`GET /scenarios/{id}/harness-diff` is removed from API.md 3.11: S-48 (an independent re-implementation by another
author) is P2, and phase 24 does not carry it (its Open doubts 11 lists S-48 as not carried). This closes Open
doubts 22.

Adversarial pass (2026-10-10), run 2 (alerts under concurrency, replay and late data), from tools/agent/adv-run2.md; lead-verified (tools/agent/adv-run2-verdict.md).

### 7.9 One run per touch, not per batch (ADV-2-10, ADV-2-11, ADV-2-13; CRITICAL; DR-408)
Phase 11 now touches a batch once per writing transaction and names each touch by `touch_id` (phase 11 section 7+).
Migration `0055` adds `touch_id UUID` to `scn.scenario_runs` with `CHECK (touch_id IS NULL OR batch_id IS NOT NULL)`,
and `uq_scenario_runs_batch` becomes `ON scn.scenario_runs (scenario_version_id, mode, touch_id) WHERE touch_id IS NOT
NULL`. The fan-out passes `touch_id` into every `scenario_run` job it enqueues (incremental, settled, shadow); the
coalesce keys become `scenario_fanout:<touch_id>` and `scenario_run:<version>:<mode>:<touch_id or window_to>`. A second
delivery of the same event is still a no-op; a new touch of the same batch starts its own run. The run still reads
every row of the batch (fan-out step 2): rows of an earlier touch stop at the sink as `Duplicate` (equal digest), and a
row mapped late, a later slice or a retried row is evaluated for the first time. A touch whose rows have a booking date
before the newest evaluated window takes the late-data path of section 3.6 (the settled run). Test: a batch mapped in
two runs gives two `completed` incremental runs, and the second run's new detections are `New`.

### 7.10 A retried run takes over its own row (ADV-2-01, CRITICAL; DR-408)
Run step 12 retries a failed run, and the phase 00 lease reaper re-queues a run whose worker died, but step 2 stopped on
any existing ledger row, so the chunks after the failure were never evaluated. `RunRepository::start_once` is therefore
`INSERT ... ON CONFLICT (scenario_version_id, mode, touch_id) WHERE touch_id IS NOT NULL DO UPDATE SET status =
'running', error_code = NULL, started_at = now(), finished_at = NULL, entities = 0, detections = 0, amendments = 0,
version = scn.scenario_runs.version + 1 WHERE scn.scenario_runs.status = 'failed' OR (scn.scenario_runs.status =
'running' AND scn.scenario_runs.started_at < now() - <the scenario_run lease>) RETURNING id`, where the lease is
`jobs.lease_s` or the kind's override in the registry (the bound of the phase 00 reaper). `None` now means the touch was
run (`completed`, `refused_cost`) or is running under a live lease. A takeover restarts at the first chunk; detections
already emitted stop at the sink as `Duplicate`, so no alert is written twice. The earlier attempt's
`scenario.run_failed` stays in the audit. After the last attempt the run stays `failed` and the scenario shows "not
operating as intended" (DSL A.3). Test: a run failed at chunk 2 of 4 completes on the retry with every group
evaluated.

### 7.11 The fold stays inside its window and never reaches a closed case (ADV-2-06 HIGH, ADV-2-07 MEDIUM; DR-408)
`DetectionKeyRepository::open_bucket` takes an upper bound: `open_bucket(tx, scenario, anchor_hk, since = window_to -
days + 1, until = window_to)` returns the newest key of the anchor whose bucket starts between `since` and `until`
inclusive. A settled run of an older window then gets its own bucket and no longer overwrites the evidence of a newer
alert. From phase 17 on, the query also skips a key whose alert (`alert.alert_index` to `alert.alerts`) sits in a case
(`casework.case_alerts` to `casework.cases`) with status `closed`: a repeat after closure gets a new bucket and a new
fingerprint, the sink returns `New`, and phase 17 section 3.5 opens a case beside the closed one with `case.linked_prior`
(DR-140). This is the DSL rule that repeats fold "into the open alert". Late rows for the window that created a closed
case's alert still reach that alert when its bucket is that window's end, and write `case.alert_amended_after_decision`
(phase 17 section 3.13). Tests: a repeat four days after the case closed opens a new case with `case.linked_prior`; a
settled run of 30 May beside a 1 June bucket leaves the 1 June alert unchanged.

### 7.12 Features recomputed for a reopened month (ADV-2-13; open)
A replay of a closed month makes `feature_compute` reopen it (phase 13 section 3.2), and phase 13 writes no topic. The
settled run of the touch can start before the recompute and read the old feature values for that month. This is left
open for the verification pass: the two options are a phase 13 topic on a recomputed month that phase 16 consumes, or a
settled-run start after the next `feature_compute` slot.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.13 Bank scenario create: the insert and a taken code (V-16-01-a, MEDIUM; DR-413)
Section 3.15, Bank scenarios, step 2: the scenario row is inserted with the phase 06
`ScenarioRepository::insert_missing(tx, &[new])`. A result of 0 means the `code` is already in use: 409
`E_NOT_ALLOWED_IN_STATE` with reason `code_taken` (the answer phase 05 gives for a taken source code), and nothing is
written. Section 3.17: `code_taken` joins the reason list of `E_NOT_ALLOWED_IN_STATE`. No new repository method.

### 7.14 Refusals of the owner and reason checks (V-16-01-c, V-16-03-a, MEDIUM; DR-413)
- Section 3.15, Bank scenarios, step 1: an owner who is not an active CFG user answers 422 `E_VALIDATION` with reason
  `owner_not_active`, as phase 06 section 3.4 does for a declaration owner.
- Section 3.5 step 4: a reason code that is inactive, or not of family `tuning` or `change`, answers 422
  `E_VALIDATION` with reason `reason_not_allowed`, and no draft is inserted (phase 04 gives the same answer at proposal).
- Section 3.17: the `E_VALIDATION` entry gains the reasons `owner_not_active` and `reason_not_allowed`; its `errors[]`
  codes stay the DSL codes.

### 7.15 A minor parameter set needs the replay only (V-16-03-d, LOW; DR-413)
DSL D.3 holds, with the `capacity` rule of the `reason_code` row of DSL D.1 (S-39): a set inside the band that does not loosen and has no `capacity` reason is minor and needs a replay; the below-the-line sample is
required for a set outside the band, for any loosening and for every `capacity` reason (section 3.5 step 4; phase 04
derives `required_evidence` by class). L-18 is met this way: the replay records the before and after alert counts of
every set, and the below-the-line evidence is taken wherever a change can lose alerts.

### 7.16 Reasons of the shadow start and of the pack refusal (V-16-04-a, V-16-06-a, LOW; DR-413)
Section 3.11 step 1: a version that is not `testing` answers 409 `E_NOT_ALLOWED_IN_STATE` with reason `not_testing`.
The unit cases need no refusal of their own: the status CHECK of section 1.1 keeps `tests_passed_at` set on every
`testing` version. Section 3.17: `not_testing` and `pack_not_enabled` (block 7.2, G9) join the reason list of
`E_NOT_ALLOWED_IN_STATE`.

### 7.17 Run of the unit cases on `POST .../test` (V-16-05-a, MEDIUM; DR-413)
1. Authorize: Cedar `scenario.version.write`, scope `config.author` (the route row of section 2).
2. State: the version is `draft` (section 3.3; `VersionStatus::transition_allowed` refuses any other source), else 409
   `E_NOT_ALLOWED_IN_STATE`.
3. Validate: section 3.1 rules 1 to 16. Any error answers 422 `E_VALIDATION` with the `errors[]` of section 3.1, and
   the version stays `draft`.
4. Run: for each entry of `tests.unit` (`name`, `fixture`, `expect`), run section 3.12 step 2 on the named fixture and
   read the alert count and the severity; the sink is not called and no `scenario_runs` row is written. A case passes
   by the criterion of section 3.12 step 3.
5. Write: when every case passes and the three positive and three negative counts hold (section 3.3), the version
   moves to `testing` and `tests_passed_at` is set. Otherwise 422 `E_VALIDATION` with the `errors[]` code
   `E_TESTS_MISSING`, and the version stays `draft`.
6. Audit `scenario.version_tested`, once for the run.
The automatic cases of DSL A.9 (null, currency, reversal and late data) are added by the studio (section 5) as
ordinary `tests.unit` entries with their fixtures before the document is saved; this run treats them like the
author's cases.

## Decisions

The decisions below are stated here; the lead records them in `docs/plan/research.md` with the dates they are accepted. They
number from DR-107, the first free number after DR-106 of phase 15.

**DR-107 (2026-10-09). One compiler and closed inputs for every scenario kind.** Aggregate measures and filters compile through the
phase 13 measure and filter compiler, each measure as an ephemeral feature definition that is never stored; event conditions compile
to CEL behind the `EventEvaluator` port; profiles run through the phase 13 `ProfileComparator`. No document text reaches the SQL,
every value is bound, identifiers come only from the closed catalogue, and every statement passes an `EXPLAIN` cost guard. Rationale:
DSL A.6 and TS-14 require one closed operator set, and a second compiler would be a second place where the same expression can
mean two things; the golden test of section 6 holds the two to one output.

**DR-108 (2026-10-09). One live version, supersede before promote, and a kill switch that falls back and never switches off.** The
partial unique index on live versions holds at every moment because activation supersedes the old row first. A kill switch is a
step-up action of the approvers (`CHK` or `M`, class `approval`) that moves the live version to `retired` and returns the most
recent superseded version to `live`. It is not a change request: the fallback and its parameter set were approved before, and a
phase 04 request cannot carry its approver as its maker (lead correction; the draft wrote an `effective` emergency request that
the phase 04 CHECKs refuse). The scenario shows "fallback active" in red until a new version goes live through the gate; the killed
version never returns. There is no route that switches a scenario off, and the `cbi` baseline is never retired (T-02). Rationale:
DSL A.10 requires a fallback to the previous approved version and forbids two live versions; a fallback to approved logic needs
speed, not a second approval, and the step-up, the reason code and the audit event make it accountable.

**DR-109 (2026-10-09). The go-live gate is checkable, and the volume effect is measured, never assumed.** A version goes live only
when its required inputs grade A or B, its candidate parameter set has a completed replay over at least the governed replay period
with an alert rate inside the range the request states, its seeded cases pass for that version, its below-the-line labels are within
the recorded tolerance, and its shadow has run the governed minimum (30 days) with every day compared. A first version with no
baseline is an increase when it alerts, never `unknown`. Rationale: S-47 and DSL A.10 list these conditions; turning them into
explicit refusal codes makes them testable and lets the approver read them in the change request.

**DR-110 (2026-10-09). Detection identity does not depend on the version or the run.** The fingerprint is the hash of the scenario,
the group's anchor and the bucket; the bucket starts at the first detection of the anchor and lasts the dedupe span (seven days by
default). The measures digest decides between a duplicate, an amendment and a new alert. A replayed batch is a no-op at the run ledger
(unique on version, mode and batch) and at the sink (duplicate). Rationale: DSL A.7 defines the alert fingerprint this way; a key that
included the version would double-emit after every version change, and a key that included the run would double-emit after every
replay of a batch.

**DR-111 (2026-10-09). Coverage gaps are recorded on every run and detection and never read as quiet.** Dark units, exclusions, blind
periods and stale sources are facts of the run; a group with a dark unit is evaluated and marked degraded, and its non-alert is
counted as unverified in the summaries. Exclusions remove rows from the measures but not from the reach. Rationale: phase 12 states
that a dark unit is "never a quiet day"; detection must not turn a blind window into an assurance, and the exclusion register must
show its effect where the detection is read.

**DR-112 (2026-10-09). Replays keep only the rows a reviewer needs, and the sample is drawn so that anyone can reproduce it.** A
replay stores alert and near-miss rows only; a near miss is a non-alert that reaches 0.8 of its threshold (governed, floor 0.5). The
below-the-line sample size follows the rule of three on the tolerance; the draw is ordered by a hash over a stored seed. Blind packets
carry no outcome, threshold, parameter or scenario identity. Labels come from A1, A2 and M, never from the scenario's author, and a
suspicion-tier sample is M's alone. Summary cells under five are withheld. Rationale: S-41 and S-42 need reproducible, bounded
samples, and the tipping-off rule of phase 03 must hold for the reviewer who labels a customer.

**DR-113 (2026-10-09). Parameter sets are per version, bounded, banded, and classed by their measured effect.** A value is stored per
group as a declared override of the document, so per-segment values need no new column. Hard bounds refuse a value; the band decides
between minor and material; a set whose replay shows fewer alerts is material, as a loosening, and `unknown` is never approved. The
reader catalogue reports a combined effect that is `decrease` when any reader decreases, because the conservative answer routes the
change to the second approver. Rationale: L-18, TS-15 and DSL D.3; the conservative combination follows the phase 04 rule that a
decrease needs the compliance head.

**DR-114 (2026-10-09). The CBI baseline installs live from the signed release; the AKR library installs in shadow; bank scenarios start
as drafts.** `pack: cbi` documents go live at the first install, because the baseline is the minimum the bank must run and the bank's
install acceptance is the record; a later release installs a new `cbi` logic version in shadow beside the live one (DSL A.10 library
releases), and every later change of a `cbi` scenario, including a release upgrade, is a change request with the validated route
(lead correction: the draft installed every new `cbi` version live, which would replace a version the bank had approved or tuned). `pack: akr` documents
install in shadow and cannot go live while the feature flag `pack.akr_live` is off, which the bank sets by change request. Bank
scenarios 901 to 999 start as grade-D drafts with the research grade D, until their own cases validate them. Rationale: S-01, S-04,
S-05 and T-02; the install acceptance is the one point where a change request cannot precede the baseline, and the Open doubts ask
the bank to confirm it.

**DR-115 (2026-10-09). Scenario history is kept seven years by the floor, in monthly partitions where the volume is high.** Versions,
parameter sets, runs, replays, shadow results and samples are class R9 with the seven-year floor as a governed setting; detection
identities are class R5 (they outlive the alerts they explain). The two high-volume tables are partitioned by month through the
factory, the phase 13 pattern. Rationale: S-58 and DATA-MODEL s.10; the partitions let the retention engine of phase 23 drop by month
without scanning customer rows.

**DR-116 (2026-10-09). Linked parties widen the scenario group by the links the canonical store holds, and rating stays narrower.**
The linked-party resolver uses only canonical links (family number hash, contact hashes, account roles); a link type with no canonical
column is refused, not inferred. Kinship widens a scenario group and is named on the alert, while the rating (phase 15) still excludes
kinship-only links from automated scores (T-24). Rationale: DSL A.4 and S-08 call for aggregation across linked parties; a link the store
does not hold cannot be audited, so it is not used.

**Lead corrections (review of 2026-10-09).** `scn.parameter_sets` declared `version` twice (the set number and the optimistic
lock); the set number is now `set_version`. Suspicion-tier rows were stored with a NULL entity id and a `masked` flag, which froze
the tier at write time and left the blind packet without the entity whose transactions it must show; every per-entity table now
keeps its entity id and the tier is resolved at read time from `canon.party.sensitivity_tier` (phase 03 masking is a read rule,
DR-105). The kill switch wrote an `effective` emergency change request with the caller as both maker and approver, which the phase
04 CHECKs refuse; it is now a step-up fallback with no change request (DR-108). The pack seed installed every new `cbi` logic
version live; only the first install is live now (DR-114). The version guard accepted an old `reinstated_at`; it now needs a new
one. The coverage-matrix Cedar annotation was commented out; the hooks now use the path-derived names of phase 06; open doubts 3,
4 and 15 were resolved in the review (DSL examples renamed to `DET_`, `E_LOGIC_VERSION_BUMP` added, roadmap row 16 depends on 13
and 15). Phase 13 gains two public entry points, `FeatureCompiler::compile_measure` and `compile_filter` (section 3.2), and
`ProfileComparator::plan` and `evaluate` become public: additions to phase 13's module made by this phase, not redefinitions.

## Open doubts (for gap pass 1 and the bank)

1. **CEL implementation.** No phase and no row of `tech-stack.md` names the CEL implementation; the stack row says only "event logic in
   CEL". The port of section 2 hides the choice. The lead chooses the crate and its licence before the phase 16 build.
2. **Depositor and companion links.** `canon.cash_event` has no depositor column and phase 10 maps no companion link, while S-08 and
   DSL A.4 name both. Until a canonical source exists (S-21 teller capture is in no phase), a document naming them is refused
   (section 3.1 rule 12). Target: phase 10 gap pass, with the phase that owns S-21.
3. **The `DET_` prefix.** Resolved in review: the phase 09 rule stands and the DSL examples are renamed (`DET_CASH02_SPLIT_BELOW_LIMIT`,
   `DET_DORM01_LARGE_FIRST_MOVE`, `DET_ACCT01_ABOVE_DECLARED`).
4. **Logic-version bump code.** Resolved in review: `E_LOGIC_VERSION_BUMP` is added to DSL.md section 7.
5. **Kill-switch oversight.** The kill switch is not a change request (DR-108). Whether a second person must confirm a fallback after the
   fact (for example a task for CH, or a line in the phase 24 decisions register) is for the bank and gap pass 1. Target: phase 24.
6. **Approvers of restricted scenarios.** Phase 06 limits `scenario.read` on a restricted scenario to CFG and MRO, so CHK, M and CH cannot read
   the document, the card or the diff of a restricted change, yet they approve changes (phase 04). Options: restricted changes approved by MRO
   only, or summary-only approval views with the diff withheld. Target: the bank and the lead, with phase 04's diff masking.
7. **Matrix grade reasons for restricted-only items.** Phase 06 grades an item with no candidate `D` with reason `no_scenario`; for an item whose
   only candidate is restricted, M, CH and IA can infer that a restricted scenario exists. Options: a neutral reason for such items, or a
   restricted-candidate reason shown to CFG and MRO only. Target: phase 06 gap pass.
8. **Who labels the blind samples.** API.md 3.11 gives `POST /samples/{id}/labels` the scope `queue.work` (A1, A2, M). QA reviewers (phase 19's
   QA service) cannot label under that scope. Confirm A1 and A2 as the blind labellers. Target: the bank (MRO).
9. **Second labels for agreement.** The plan takes one label per sample. The share of samples labelled twice for inter-rater agreement is not set.
   Target: the bank (MRO), before the first go-live.
10. **Placeholders of the replay and sample rules.** `scenario.near_miss_ratio` (0.8), `scenario.btl_tolerance_pct` (1.0 per cent),
    `scenario.summary_min_cell` (5), `scenario.sample_min_per_stratum` (50) and the seven-day dedupe span are placeholders for the bank and the
    Office to confirm (S-42).
11. **Replay evaluation points.** Business days with activity plus month-ends for `L2` and `L3` approximate the live runs. Before the first
    go-live, a replay of one live month must agree with the live detections by at least a number the bank sets. Target: phase 16 acceptance (MRO).
12. **Re-measure at decision.** Phase 13 must call `ReaderCatalogue::measure` again in its `check_before_decision` (its section 3.7 does not say
    so), and phase 04 needs a recompute entry point for the volume effect of a request that waits for a reader replay. Target: phase 13 and phase 04
    gap passes.
13. **Backward working-day walk.** Calendar-aware windows need `BankCalendarClock::working_days_back` (section 3.8 step 5), an extension of the
    phase 09 pure function. Target: phase 09 review; this phase tests it.
14. **Hot window.** `scenario.hot_window_days` (400) is a placeholder: phase 10 does not set the canonical hot window. Target: phase 10 and the bank
    (retention).
15. **Roadmap dependency.** Resolved in review: roadmap row 16 now depends on 13 and 15 (risk-level overrides read phase 15's `RatingSource`; with
    the null implementation risk overrides never match and the default value applies).
16. **Live alert id and the key table.** Resolved by phase 17: `scn.detection_keys` stays the identity table and `alert.alert_index` maps it to
    the alert (phase 17 DR-117); `live_alert_id` is written by phase 17 and keeps no foreign key (the alert table is partitioned).
17. **Document content.** The 99 CBI and 62 AKR documents, their fixtures and seeded cases are most of the content. This phase ships the engine and
    the verification (section 3.15); which documents ship in the first release, and which wait, is a decision for the bank and CFG under research 27
    B26. Target: the lead and CFG.
18. **Install acceptance of CBI documents.** `pack: cbi` documents go live at install without a change request (DR-114). The install checklist must
    record the bank's acceptance; the lead confirms that this is an accepted exception to "every change through a change request".
19. **Seasonal shadow.** The shadow length is an input at start (`shadow_days_required`), not a flag in the document, so a seasonal scenario needs the
    bank's choice of two cycles at that point. Target: MRO.
20. **Return of a killed version.** A killed version stays retired; to run its logic again the bank copies it into a new draft and goes through the
    gate (section 3.13). The bank confirms that no faster reinstatement is wanted. Target: MRO.
21. **S-requirements left to later phases.** Carried in part here: S-01, S-02 (the matrix shows the item-to-readiness part), S-04, S-05, S-06, S-07,
    S-08, S-09, S-10, S-13, S-38 to S-42, S-44, S-45 (card; the replication test is item 22), S-47, S-49 to S-52, S-53 (operational part), S-58,
    S-59, T-02, T-04, L-16 to L-18, TS-14, TS-15. Left to later phases or gap pass 1: S-03 (phase 06, carried there); S-11 and T-29 (counterparty
    resolution: phase 13 section 7+ writes the resolver and phase 18 section 7+ takes the analyst decisions, DR-398); S-12 and S-14 to S-20
    (phases 09, 10, 13 and the connector phases); S-21 (teller capture, item 2); S-22 to S-24 (phase 22, with the branch window); S-25 (hard-rule
    content, after the documents of item 17); S-26 (phase 2 flag, phase 29); S-27, S-28 and S-30 to S-37 (content and engines of later phases or
    P2); S-43 (tuning workbench, phase 24 or a later studio phase); S-46 (review calendar and recalibration clock, phase 24); S-48 (independent
    harness, item 22); S-54 (risk-statement measure: phase 06 names phase 16; proposed phase 24 with the risk assessment); S-55 (risk-statement
    importer, phase 24); S-56 and S-57 (assessor mode and pack, phase 24); S-60 (Office engagement pack, phase 24); S-05's card export for the Office
    (phase 24 PackBuilder). Gap pass 1 assigns each.
22. **Independent re-implementation (S-48) and `GET /scenarios/{id}/harness-diff`.** A second implementation by a different author belongs with the
    validation packs of phase 24; the route row stays in API.md 3.11 with that target. Target: gap pass 1 and MRO.
23. **Outcome metrics (S-53).** Productivity, uniqueness, outcome, misses, quality and fairness need the alerts, cases and reports of phases 17 to 21 and
    the fairness metrics of phase 24. Target: phase 24.
24. **Analyst-hour estimates (S-41).** No phase holds an analyst time model. Target: phase 24 or phase 17 (triage metrics).
25. **Alert columns of the matrix.** `alerts`, `cases` and `strs` join `GET /coverage/matrix` in phases 17, 18 and 21 without a new row. Recorded here
    so those phases extend the same route.
26. **Standby reads.** DSL A.6.1 says runs use the standby where one exists. No standby exists before phase 30. Target: phase 30.
27. **Install reason code.** `CHG_RISK_ASSESSMENT` is the install reason of the default sets; the bank may prefer a regulatory-order code for CBI-basis
    defaults. Target: MRO.
28. **D1 design specification.** DESIGN-SYSTEM.md section 8.3 specifies D2 and K1 to K4 and S1, not D1. Section 5 proposes the D1 layout (register table,
    matrix tab, creation form); the designer confirms or replaces it before the frontend build. Target: the designer, with phase 16 frontend.

## What This Phase Does NOT Touch

- The phase 06 Cedar file `80_detection.cedar`, its actions (`scenario.list`, `scenario.read`, `cbi_catalogue.read`), its tables and its
  register rules. The only phase 06 rows changed are the extensions of section 4.4 (`GET /scenarios`, `GET /scenarios/{id}`) and the two routers
  that gain a row each.
- Alerts, cases, the case access list, reports, STRs, RFIs, watches, freezes, operations tasks, branch items and notifications (phases 17 to 22).
  The detection payload of section 3.7 is the contract; the sink is a port that phase 17 replaces.
- Ingest, landing, mapping and the canonical writer (phases 10 and 11). The two batch indexes of migration 0056 are physical indexes only; no column,
  constraint or grant of a phase 10 table changes.
- Features, segments, peer builds, profile tables and their routes (phase 13). This phase consumes the compiler, the comparator and the ports, and
  adds the `ReaderCatalogue` implementation.
- Ratings, overrides, KYC reviews, PEP and ownership (phase 15); `RatingSource` is consumed only.
- Screening, lists and the screening harness (phase 14).
- Models, ranking detectors and model validation (phase 27). Scenario documents are rules; `scenario.kill_switch` is not a model action.
- Validation packs, registers, the command centre, fairness metrics, the harness-diff route, the Office card export and the outcome metrics
  (phase 24 and phases 17 to 21 for their data).
- Retention deletion and legal holds (phase 23). This phase assigns classes and the seven-year floor; it deletes nothing.
- Standby reads and failover (phase 30).
- New step-up action classes, problem codes, roles, network zones, topics other than `scenario.changed`, and `change.route_map` rows.
- DSL.md beyond three alignments made in review: the `superseded` status in A.10, the code `E_LOGIC_VERSION_BUMP` in section 7, and the
  `DET_` prefix on the reason codes of examples A.8, 9.1 to 9.3 (open doubt 3); the worked examples are otherwise tested, not edited.
- The D3 policy studio (phase 09) and the D1 design specification in DESIGN-SYSTEM.md (no specification exists; section 5 proposes it, open doubt 28).
- Content: the scenario documents, fixtures and seeded cases of the shipped pack (Open doubts 17) and the counterparty resolver (Open doubts 2 and 21).
- The phase 13 segment and feature routes, and `/feature-definitions` and `/peer-groups` of API.md 3.11 (phase 13's rows).
