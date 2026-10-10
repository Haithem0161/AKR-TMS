# Phase 15: Risk Rating & KYC

**Goal:** The customer risk rating engine RM-IQ (DSL.md Part C, factor catalogue of research file 26 section 14d: 27 factors in six blocks, floors, completeness cap, triggers, stickiness and the controls matrix), a versioned model changed only through change requests and a shadow run, every rating row stored with its model version, inputs, factor contributions and reason codes, re-scored on canonical, country, PEP, review and override events and nightly; governed overrides (upward by KYC or the MLRO, downward by compliance under four eyes, one level, expiring within 12 months); the one-way rating feed for the bank; the KYC review queue with two clocks per customer, the light yearly check for all and the EDD checklist; the PEP register with tiers, relatives and close associates confirmed by a person, sourced from the phase 14 `pep` lists, with the 24-month pair-memory cap; beneficial ownership by chain calculation; the relationship-conflict check that phase 03 left open; and the masking of every customer-linked record (the `customer` kind becomes a masked kind). Screens K1 to K4.

**Dependencies:** Phase 00 (jobs, outbox, timers, settings, schedules), Phase 01 (step-up, `StepUpService`), Phase 02 (evidence objects and staging, access events, partition registry), Phase 03 (Cedar bundle, masking registry, `ConflictCheck` port, the `customer` resource adapter of phase 13), Phase 04 (`ChangeTargetRegistry`, route map, workflow engine), Phase 06 (`AttributeCatalogue`), Phase 09 (`CountryView`, `ref.country_level_changed`, `ref.reason_codes`), Phase 10 (`canon.party`, `canon.customer`, `canon.party_relationship`, `canon.kyc_document`), Phase 11 (`canon.touched`, `FreshnessService`, `admin.blind_periods`), Phase 12 (coverage ledger and dark units), Phase 13 (`DeclaredProfileSource` and `RatingSource` ports, `profile.read`), Phase 14 (`ref.lists` of kind `pep`, `alert.screening_alerts`, `PairMemoryService`, the risk-modifier hook)
**Macro-phase:** P1
**Complexity:** XL (34 routes: the 32 of section 2 and the two count views of sections 7.4 and 7.5; 16 tables and four ports; the rating engine, the review clocks and the ownership calculation carry the weight)
**Binaries touched:** akr-api (routers `rating`, `kyc`, `pep`, `ownership`, `exit`, `rating_models`; change targets `rating_model`, `pep_entry` and `rating_override` registered in process; `RelationshipConflictCheck`) | akr-worker (the job kinds of section 4.1; `RatingEngine`, `KycReviewService`, `PepService`, `OwnershipService`) | akr-ctl (`rating seed`, `rating verify-golden`). akr-ingestd and akr-assess unchanged. Nothing in this phase reaches a CBS: `canon` is read only, and the feed is written to the bank's own tables and export files.

**Requirements carried:**
- K-01 to K-11, K-13 to K-18, K-21, K-22, K-24, K-26 to K-30, K-33, K-35, K-37 and K-42 (the rating model, floors, completeness cap, triggers, overrides, controls, computation contract, lifecycle and the country event; the light yearly check K-10, the exit register K-13, the review workbench K-14, EDD playbooks K-15, the source-of-wealth checklist item K-16, beneficial ownership by chain K-18, the PEP register and kinship K-21 and K-22). The other K IDs are on the gap pass 1 lines below (Open doubts 19)
- TR-01 to TR-26 (trigger library, DSL.md C.5: this phase wires the triggers whose source topics exist after phase 11; TR-10 country level change and TR-15 case decision are named by the DSL)
- FL-01 to FL-15 (floors, research file 26 s.14; FL-03, FL-05, FL-07 to FL-12 are cited by DSL.md Part C)
- L-10, L-43 and GV-37 (as MODULES M10 lists them)
- C-29 (`data_as_of` and the stale state on every rating, explanation and ownership response; the classes are phase 11's)
- Gap pass 1 (coverage), implemented here: SP-29 (the relationship-conflict check, section 3.9), K-34 (the rating model as governed configuration; the model card and validation packs are phases 27 and 24); T-30 (versioned rating models with the factor catalogue, maker-checker, shadow runs and sensitivity), T-32 (overrides with reason code and expiry, a second approver for downward overrides, override analytics), T-34 (the per-customer rating explanation); UX-59 (the risk profile of mockup 04; the open-case strip from phase 18 on); F-02 (every factor weighed and explained factor by factor, controls by level), F-07 (review clocks by level; the profile-breach trigger from phase 17 on), F-10 (PEP register for customers and owners with relatives and associates, EDD sign-off, source-of-wealth evidence, the former state), B-10 (CDD documents and every review outcome recorded), B-13 (the rating is a one-way feed that blocks nobody; the light check as compensating monitoring); VD-36 (the rating model mapped to the CBI risk levels); VC-55 (the full shareholder table with percentages, evidence and dates), VC-59 (registry and licence documents tied to ownership inputs as evidence objects).
- Gap pass 1 (coverage), partly implemented: K-19 (owner percentages entered with evidence; the coverage view, section 7+), K-25 (override analytics by proposer and branch with follow-on alerts and STRs, section 7+), K-36 (the level 1 row of the controls matrix with its CDD depth and monitoring profile; re-rating by triggers); L-09 (a customer without a rating shown as a control failure), L-28 (declared turnover and counts beside the actuals), L-37 (the geography block from nationality and residence), L-44 (the completeness cap and the degraded flag), L-45 (overdue reviews and the exit-candidate state); T-31 (event triggers with a fall held until the review completes), T-33 (the distribution sanity band and sensitivity counts); W-15 (source-of-funds and source-of-wealth evidence; declared against actual figures), W-21 (the PEP register by position, confirmed relatives, the former state, pair memory), W-24 (the trigger library and the refresh clock); F-03 (CDD depth set per level by the controls rows), F-08 (missing mandatory data caps the best level), F-09 (ownership and control chains with percentages and legal-arrangement parties), F-22 (the onboarding channel factor), B-03 (declared figures beside the actuals on the customer view); VA-36 (open, versioned rating models with factors, weights, bands and overrides); VC-21 (the ownership chain at the 20 percent threshold), VC-39 (ownership inputs linked to evidence objects), VC-47 (the PEP register with tiers and relatives), VC-48 (kinship confirmed by a person), VC-57 (the evidence link and date per ownership edge), VC-66 (the controls matrix by level and the review interval), VC-67 (reviews on a level change), VC-68 (versioned EDD checklist templates), VC-69 (the checklist template and the completer recorded), VC-79 (the review queue with calendar and event due bases), VC-80 (triggers for screening, PEP and case events).
- Gap pass 1 (coverage), not carried: K-41 (pre-purchase data pilot, a procurement step, DR-396); L-67 (staff and branch behaviour analytics, P2, DR-394); W-16 (adverse-media module; no media source contracted), W-17 (enterprise-wide risk assessment, DR-400); F-01 (the bank's own enterprise-wide risk assessment, DR-401); VC-41 and VC-45 (vendor risk-data connectors and profile snapshots, Q12), VC-51 to VC-54 (adverse-media sources, feeds, delta runs and entity extraction, Q12), VC-58 (vendor ownership data, Q12), VC-73 and VC-78 (onboarding and identity-proofing fields from the CBS), VC-77 (recorded verification calls); DR-404.

Research basis: `research/26-kyc-cdd-risk-rating.md` sections 6c (triggers), 14d (factors, anchors, block and factor weights), 14e (worked examples, used as the golden test) and 14g (controls matrix). The governing spec is DSL.md Part C; where research and DSL differ, DSL wins and the difference is listed in Open doubts.

---

## 1. SQLx Migration Changes

Four migrations, `0050_kyc_rating_core` to `0053_kyc_settings_schedules_classification`, after the phase 14 group (`0045` to `0049`); `0054` is not used. All tables are in the new schema `kyc`. DATA-MODEL s.11 lists eight of them; this phase adds eight more: `rating_current`, `rating_shadow_runs`, `rating_shadow_items`, `review_queries`, `review_documents`, `declared_profiles`, `ownership_inputs` and `exit_register`, and DATA-MODEL s.11 is aligned in the same change.

Ownership of the tables: `akr_migrate` creates them; `akr_app` (api and worker) writes them under the grants of section 1.5; `akr_readonly` reads them for reports. No table is written by `akr_ingest` or `akr_assess`. Ids are UUIDv7 generated in the application with no database default.

### 1.1 `0050_kyc_rating_core.sql`

```sql
-- migration 0050_kyc_rating_core.sql
-- Phase 15. Schema kyc: rating model versions (DSL.md Part C), shadow runs, overrides, the monthly rating history with
-- its current pointer, and the one-way rating feed. Retention classes and classification rows are in 0053.

CREATE SCHEMA IF NOT EXISTS kyc;

-- RM-IQ and any later rating model (DSL.md C.9). One live version per model id (partial unique index below). The
-- document is the Part C JSON; its content is immutable once the version leaves draft (guard, migration 0052).
CREATE TABLE IF NOT EXISTS kyc.rating_models (
    id                  UUID PRIMARY KEY,
    model_id            TEXT NOT NULL CHECK (model_id ~ '^RM-[A-Z]{2,6}$'),                  -- 'RM-IQ'
    version             TEXT NOT NULL CHECK (version ~ '^[0-9]+\.[0-9]+\.[0-9]+$'),          -- semver (DSL.md s.0)
    status              TEXT NOT NULL CHECK (status IN ('draft', 'shadow', 'live', 'retired')),
    regulatory_profile  TEXT NOT NULL CHECK (regulatory_profile IN ('iraq_cbi', 'eu_amlr')),
    document            JSONB NOT NULL CHECK (jsonb_typeof(document) = 'object'),
    document_sha256     BYTEA NOT NULL CHECK (octet_length(document_sha256) = 32),          -- of the canonical encoding
    validation          JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(validation) = 'object'),   -- {errors: [], warnings: []}
    change_request_id   UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,        -- NULL for the seeded version
    effective_from      TIMESTAMPTZ,
    retired_at          TIMESTAMPTZ,
    created_by          UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    row_version         INT NOT NULL DEFAULT 1,                                             -- If-Match on PUT of a draft
    CONSTRAINT uq_rating_models_version UNIQUE (model_id, version),
    CHECK (status <> 'live' OR effective_from IS NOT NULL),
    CHECK (status <> 'retired' OR retired_at IS NOT NULL),
    CHECK (status = 'draft' OR (validation ? 'errors' AND jsonb_array_length(validation -> 'errors') = 0))  -- DSL.md s.7: zero errors past draft
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_rating_models_live ON kyc.rating_models (model_id) WHERE status = 'live';
CREATE INDEX IF NOT EXISTS idx_rating_models_status ON kyc.rating_models (model_id, status);

-- A shadow run over production data (DSL.md C.9): the evidence a material model change needs. Retention R11.
CREATE TABLE IF NOT EXISTS kyc.rating_shadow_runs (
    id                      UUID PRIMARY KEY,
    model_version_id        UUID NOT NULL REFERENCES kyc.rating_models(id) ON DELETE RESTRICT,
    as_of                   DATE NOT NULL,
    status                  TEXT NOT NULL CHECK (status IN ('queued', 'running', 'completed', 'failed')),
    parties_total           INT CHECK (parties_total >= 0),
    parties_scored          INT CHECK (parties_scored >= 0),
    parties_control_failure INT CHECK (parties_control_failure >= 0),
    level_up                INT CHECK (level_up >= 0),
    level_down              INT CHECK (level_down >= 0),
    tier_migration          JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(tier_migration) = 'object'),  -- {"3>4": n, ...}
    sensitivity             JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(sensitivity) = 'object'),     -- counts per perturbation
    volume_effect           TEXT CHECK (volume_effect IN ('increase', 'decrease', 'neutral', 'unknown')),
    failure_code            TEXT,
    requested_by            UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    started_at              TIMESTAMPTZ,
    finished_at             TIMESTAMPTZ,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (status <> 'completed' OR (finished_at IS NOT NULL AND volume_effect IS NOT NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_rating_shadow_active ON kyc.rating_shadow_runs (model_version_id)
    WHERE status IN ('queued', 'running');
CREATE INDEX IF NOT EXISTS idx_rating_shadow_version ON kyc.rating_shadow_runs (model_version_id, created_at DESC);

-- The disagreement list of a shadow run: only parties whose shadow result differs from the live result (or is a
-- control failure). Retention R11, kept with its run.
CREATE TABLE IF NOT EXISTS kyc.rating_shadow_items (
    run_id                  UUID NOT NULL REFERENCES kyc.rating_shadow_runs(id) ON DELETE RESTRICT,
    party_id                UUID NOT NULL,                                 -- no foreign key: party is bi-temporal (DR-87)
    live_level              SMALLINT CHECK (live_level BETWEEN 1 AND 5),
    shadow_level            SMALLINT CHECK (shadow_level BETWEEN 1 AND 5),
    shadow_cbi_level        TEXT CHECK (shadow_cbi_level IN ('high', 'medium', 'low')),
    shadow_control_failure  BOOLEAN NOT NULL DEFAULT FALSE,
    drivers_delta           JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(drivers_delta) = 'array'),
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (run_id, party_id),
    CHECK (shadow_control_failure OR live_level IS DISTINCT FROM shadow_level)
);

-- Overrides (DSL.md C.6; MODULES M10). Upward: effective at once, reason code and expiry, no second approver.
-- Downward: a change request of route kyc_override_down (DSL.md E.4), one level, evidence, a second approver who is
-- not the proposer, expiry within 12 months. Floors and the completeness cap are applied by the engine, never here.
CREATE TABLE IF NOT EXISTS kyc.overrides (
    id                  UUID PRIMARY KEY,
    party_id            UUID NOT NULL,                                      -- no foreign key: party is bi-temporal (DR-87)
    direction           TEXT NOT NULL CHECK (direction IN ('up', 'down')),
    computed_level      SMALLINT NOT NULL CHECK (computed_level BETWEEN 1 AND 5),   -- the engine's level when proposed
    to_level            SMALLINT NOT NULL CHECK (to_level BETWEEN 1 AND 5),
    CHECK ((direction = 'up' AND to_level > computed_level) OR (direction = 'down' AND to_level = computed_level - 1)),
    reason_code         TEXT NOT NULL CHECK (reason_code ~ '^OVR_[A-Z0-9_]{3,56}$'),          -- family override (phase 09)
    reason_text         TEXT NOT NULL CHECK (length(reason_text) BETWEEN 1 AND 500),          -- personal
    status              TEXT NOT NULL CHECK (status IN ('proposed', 'active', 'ended', 'withdrawn', 'rejected')),
    end_reason          TEXT CHECK (end_reason IN ('expired', 'superseded', 'withdrawn', 'model_retired', 'rejected')),
    change_request_id   UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,     -- every downward override
    evidence_object_id  UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,    -- required when downward
    proposed_by         UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    approved_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    effective_from      TIMESTAMPTZ,
    expires_at          TIMESTAMPTZ NOT NULL,
    ended_at            TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    row_version         INT NOT NULL DEFAULT 1,
    CHECK (expires_at > created_at),
    CHECK (direction = 'up' OR change_request_id IS NOT NULL),
    CHECK (direction = 'up' OR evidence_object_id IS NOT NULL),
    CHECK (approved_by IS NULL OR approved_by <> proposed_by),                              -- four eyes (K-24)
    CHECK (direction = 'up' OR status IN ('proposed', 'rejected', 'withdrawn')
           OR (approved_by IS NOT NULL AND effective_from IS NOT NULL
               AND expires_at <= effective_from + INTERVAL '12 months')),                    -- C.6: within 12 months
    CHECK ((status = 'ended') = (ended_at IS NOT NULL AND end_reason IS NOT NULL))
);
-- One open override per party (proposed or active); a second is refused by the service and by this index.
CREATE UNIQUE INDEX IF NOT EXISTS uq_overrides_open ON kyc.overrides (party_id) WHERE status IN ('proposed', 'active');
CREATE INDEX IF NOT EXISTS idx_overrides_expiry ON kyc.overrides (expires_at) WHERE status = 'active';
CREATE INDEX IF NOT EXISTS idx_overrides_party ON kyc.overrides (party_id, created_at DESC);

-- The rating history. A row is written only when the computed state differs from the previous row (level, CBI label,
-- rating status, input hash, override or model version); a nightly run that changes nothing writes nothing. Monthly
-- partitions on effective_on (the bank business date of the change), created by kyc.create_partition (section 1.1).
CREATE TABLE IF NOT EXISTS kyc.ratings (
    id                  UUID NOT NULL,
    party_id            UUID NOT NULL,                                      -- no foreign key: party is bi-temporal (DR-87)
    model_version_id    UUID NOT NULL REFERENCES kyc.rating_models(id) ON DELETE RESTRICT,
    effective_on        DATE NOT NULL,                                      -- partition key
    computed_at         TIMESTAMPTZ NOT NULL,
    input_hash          BYTEA NOT NULL CHECK (octet_length(input_hash) = 32),   -- SHA-256 of the canonical inputs and the model
    rating_status       TEXT NOT NULL CHECK (rating_status IN ('rated', 'control_failure')),
    level               SMALLINT CHECK (level BETWEEN 1 AND 5),
    cbi_level           TEXT CHECK (cbi_level IN ('high', 'medium', 'low')),
    score               NUMERIC(6,2) CHECK (score BETWEEN 0 AND 100),
    inherent_score      NUMERIC(6,2) CHECK (inherent_score BETWEEN 0 AND 100),
    peak_term           NUMERIC(4,3) NOT NULL CHECK (peak_term >= 0 AND peak_term < 0.5),
    block_scores        JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(block_scores) = 'object'),   -- detection-logic
    factors             JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(factors) = 'array'),         -- personal
    drivers             JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(drivers) = 'array'),         -- personal
    floors_fired        TEXT[] NOT NULL DEFAULT '{}',
    cap_applied         TEXT CHECK (cap_applied IN ('min_level_3', 'control_failure')),
    stickiness          TEXT NOT NULL DEFAULT 'none' CHECK (stickiness IN ('none', 'held_fall')),
    override_id         UUID REFERENCES kyc.overrides(id) ON DELETE RESTRICT,
    missing_share       NUMERIC(5,4) NOT NULL CHECK (missing_share BETWEEN 0 AND 1),
    degraded            BOOLEAN NOT NULL,
    degraded_reasons    TEXT[] NOT NULL DEFAULT '{}',
    trigger_code        TEXT,                                               -- a TR-nn id or a neutral TRG_ code (section 3.6)
    data_as_of          DATE,
    PRIMARY KEY (id, effective_on),
    CHECK ((rating_status = 'rated') = (level IS NOT NULL AND cbi_level IS NOT NULL AND score IS NOT NULL)),
    CHECK (level IS NULL OR cbi_level = CASE WHEN level <= 2 THEN 'low' WHEN level = 3 THEN 'medium' ELSE 'high' END),
    CHECK (rating_status = 'rated' OR cap_applied = 'control_failure')
) PARTITION BY RANGE (effective_on);
CREATE INDEX IF NOT EXISTS idx_ratings_party ON kyc.ratings (party_id, computed_at DESC);
CREATE INDEX IF NOT EXISTS idx_ratings_hash  ON kyc.ratings (party_id, input_hash);

-- The partition factory (SECURITY DEFINER, the phase 13 `feat.create_partition` pattern). One allow-listed table.
CREATE OR REPLACE FUNCTION kyc.create_partition(p_table TEXT, p_year INT, p_month INT) RETURNS TEXT
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog
AS $$
DECLARE
    v_from DATE;
    v_to   DATE;
    v_name TEXT;
BEGIN
    IF p_table NOT IN ('ratings') THEN
        RAISE EXCEPTION 'kyc.create_partition: % is not a partitioned kyc table', p_table;
    END IF;
    IF p_year NOT BETWEEN 2000 AND 2200 OR p_month NOT BETWEEN 1 AND 12 THEN
        RAISE EXCEPTION 'kyc.create_partition: month %-% out of range', p_year, p_month;
    END IF;
    PERFORM pg_catalog.pg_advisory_xact_lock(7307418562110395013);   -- fixed key: 'akr.kyc.partition_factory'
    v_from := pg_catalog.make_date(p_year, p_month, 1);
    v_to   := (v_from + interval '1 month')::date;
    v_name := pg_catalog.format('%s_%s_%s', p_table, p_year, pg_catalog.lpad(p_month::text, 2, '0'));
    IF NOT EXISTS (SELECT 1 FROM pg_catalog.pg_class c
                   JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace
                   WHERE n.nspname = 'kyc' AND c.relname = v_name) THEN
        EXECUTE pg_catalog.format('CREATE TABLE kyc.%I PARTITION OF kyc.%I FOR VALUES FROM (%L) TO (%L)',
                                  v_name, p_table, v_from::text, v_to::text);
    END IF;
    RETURN v_name;
END $$;
REVOKE ALL ON FUNCTION kyc.create_partition(TEXT, INT, INT) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION kyc.create_partition(TEXT, INT, INT) TO akr_app;

-- Seed partitions: the current month and the three ahead (audit.partition_ahead_months), through the factory.
DO $$
DECLARE
    v_m DATE;
    i   INT;
BEGIN
    FOR i IN 0..3 LOOP
        v_m := (date_trunc('month', now()) + make_interval(months => i))::date;
        PERFORM kyc.create_partition('ratings', EXTRACT(YEAR FROM v_m)::int, EXTRACT(MONTH FROM v_m)::int);
    END LOOP;
END $$;

-- The current rating per party: one row, upserted by the engine in the same transaction as the history row. It is
-- the pointer that the lists, the feed and the segment risk overlay read (phase 13 RatingSource).
CREATE TABLE IF NOT EXISTS kyc.rating_current (
    party_id            UUID PRIMARY KEY,
    rating_id           UUID NOT NULL,
    rating_effective_on DATE NOT NULL,
    model_version_id    UUID NOT NULL REFERENCES kyc.rating_models(id) ON DELETE RESTRICT,
    rating_status       TEXT NOT NULL CHECK (rating_status IN ('rated', 'control_failure')),
    level               SMALLINT CHECK (level BETWEEN 1 AND 5),
    cbi_level           TEXT CHECK (cbi_level IN ('high', 'medium', 'low')),
    score               NUMERIC(6,2) CHECK (score BETWEEN 0 AND 100),
    degraded            BOOLEAN NOT NULL,
    computed_at         TIMESTAMPTZ NOT NULL,
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    FOREIGN KEY (rating_id, rating_effective_on) REFERENCES kyc.ratings (id, effective_on) ON DELETE RESTRICT
);
CREATE INDEX IF NOT EXISTS idx_rating_current_level ON kyc.rating_current (level, cbi_level);

-- The one-way rating feed for the bank (MODULES M10): one row per change of the CBI label, never a level-only change.
-- Read by the bank's pull (GET /rating/feed, cursor on seq) and exported as a sealed file by rating_feed_export.
-- Suspicion-tier parties are never written here (section 3.6).
CREATE TABLE IF NOT EXISTS kyc.rating_feed (
    seq                 BIGINT GENERATED ALWAYS AS IDENTITY UNIQUE,
    id                  UUID PRIMARY KEY,
    party_id            UUID NOT NULL,
    cif                 TEXT NOT NULL CHECK (length(cif) BETWEEN 1 AND 40),                     -- personal
    previous_cbi_level  TEXT CHECK (previous_cbi_level IN ('high', 'medium', 'low')),
    cbi_level           TEXT NOT NULL CHECK (cbi_level IN ('high', 'medium', 'low')),
    rating_id           UUID NOT NULL,
    rating_effective_on DATE NOT NULL,
    effective_at        TIMESTAMPTZ NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    delivered_at        TIMESTAMPTZ,                                                            -- set by the export job only
    FOREIGN KEY (rating_id, rating_effective_on) REFERENCES kyc.ratings (id, effective_on) ON DELETE RESTRICT,
    CHECK (previous_cbi_level IS DISTINCT FROM cbi_level)
);
CREATE INDEX IF NOT EXISTS idx_rating_feed_party ON kyc.rating_feed (party_id, effective_at);
CREATE INDEX IF NOT EXISTS idx_rating_feed_undelivered ON kyc.rating_feed (seq) WHERE delivered_at IS NULL;
```

```

### 1.2 `0051_kyc_reviews_pep_ownership.sql`

```sql
-- migration 0051_kyc_reviews_pep_ownership.sql
-- Phase 15. KYC reviews and their queries and evidence links, the declared profile that phase 13 reads as its KYC
-- source, the PEP register and relatives, the ownership inputs entered by the bank and the ownership results, and the
-- exit register. Every row is customer-linked and follows the party for retention (section 1.4).

-- One review per trigger, clock or periodic due date (section 3.4). Its due date comes from the rating level and the
-- controls row; a sentinel or empty review date is recorded with date_unknown and is never read as on time.
CREATE TABLE IF NOT EXISTS kyc.kyc_reviews (
    id                  UUID PRIMARY KEY,
    party_id            UUID NOT NULL,                                       -- no foreign key: party is bi-temporal (DR-87)
    customer_id         UUID,                                                -- canon.customer logical id the review is about
    kind                TEXT NOT NULL CHECK (kind IN ('periodic', 'event', 'light_yearly', 'light', 'edd')),
    trigger_code        TEXT CHECK (trigger_code ~ '^(TR-[0-9]{2}|TRG_[A-Z_]{3,40})$'),
    due_at              TIMESTAMPTZ,                                         -- NULL only when date_unknown
    due_basis           TEXT NOT NULL CHECK (due_basis IN ('kyc_refresh_clock', 'classification_clock', 'light_clock',
                                                          'yearly_clock', 'trigger_sla', 'level_change', 'edd_playbook',
                                                          'date_unknown')),
    date_unknown        BOOLEAN NOT NULL DEFAULT FALSE,
    rating_id           UUID,
    rating_effective_on DATE,
    level_at_open       SMALLINT CHECK (level_at_open BETWEEN 1 AND 5),
    status              TEXT NOT NULL CHECK (status IN ('open', 'in_progress', 'awaiting_answer', 'completed',
                                                       'cancelled', 'superseded')),
    assignee_id         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    claimed_at          TIMESTAMPTZ,
    escalated_at        TIMESTAMPTZ,                                         -- set by the overdue timer (section 3.4)
    checklist_template  TEXT NOT NULL CHECK (checklist_template ~ '^[a-z][a-z0-9_]{2,40}_v[0-9]+$'),   -- e.g. edd_entity_v1
    checklist           JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(checklist) = 'object'), -- items, values, dates, draft declared figures
    outcome             TEXT CHECK (outcome IN ('refresh_completed', 'no_change', 'level_change_proposed',
                                                'edd_completed', 'exit_candidate', 'escalated')),
    outcome_detail      JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(outcome_detail) = 'object'),
    completed_by        UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    completed_at        TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,                              -- If-Match on claim, PATCH and complete
    FOREIGN KEY (rating_id, rating_effective_on) REFERENCES kyc.ratings (id, effective_on) ON DELETE RESTRICT,
    CHECK (date_unknown = (due_at IS NULL)),
    CHECK ((status = 'completed') = (outcome IS NOT NULL AND completed_at IS NOT NULL AND completed_by IS NOT NULL)),
    CHECK (claimed_at IS NULL OR assignee_id IS NOT NULL)
);
-- One open review per party and kind: a second event trigger joins the open event review (trigger_code is updated).
CREATE UNIQUE INDEX IF NOT EXISTS uq_kyc_reviews_open ON kyc.kyc_reviews (party_id, kind)
    WHERE status IN ('open', 'in_progress', 'awaiting_answer');
CREATE INDEX IF NOT EXISTS idx_kyc_reviews_queue ON kyc.kyc_reviews (status, due_at)
    WHERE status IN ('open', 'in_progress', 'awaiting_answer');
CREATE INDEX IF NOT EXISTS idx_kyc_reviews_assignee ON kyc.kyc_reviews (assignee_id, status) WHERE assignee_id IS NOT NULL;

-- Queries raised on a review. Completion is refused with E_OPEN_QUERIES (API.md s.6) while one is open.
CREATE TABLE IF NOT EXISTS kyc.review_queries (
    id              UUID PRIMARY KEY,
    review_id       UUID NOT NULL REFERENCES kyc.kyc_reviews(id) ON DELETE RESTRICT,
    item_key        TEXT CHECK (item_key ~ '^[a-z][a-z0-9_.]{1,60}$'),
    text            TEXT NOT NULL CHECK (length(text) BETWEEN 1 AND 1000),              -- personal
    status          TEXT NOT NULL CHECK (status IN ('open', 'answered', 'closed')),
    raised_by       UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    answer          TEXT CHECK (length(answer) BETWEEN 1 AND 1000),                      -- personal
    answered_by     UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    answered_at     TIMESTAMPTZ,
    closed_by       UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    closed_at       TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (status = 'open' OR (answer IS NOT NULL AND answered_at IS NOT NULL)),
    CHECK (status <> 'closed' OR closed_at IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_review_queries_open ON kyc.review_queries (review_id) WHERE status <> 'closed';

-- Evidence attached to a review: a staged evidence object (phase 02, owner kind kyc_review) or a canonical KYC document
-- row that the CBS holds (canon.kyc_document, phase 10: never recreated here).
CREATE TABLE IF NOT EXISTS kyc.review_documents (
    id                      UUID PRIMARY KEY,
    review_id               UUID NOT NULL REFERENCES kyc.kyc_reviews(id) ON DELETE RESTRICT,
    party_id                UUID NOT NULL,
    item_key                TEXT CHECK (item_key ~ '^[a-z][a-z0-9_.]{1,60}$'),
    kind                    TEXT NOT NULL CHECK (kind IN ('identity', 'address', 'source_of_funds', 'source_of_wealth',
                                                          'registration', 'ownership', 'pep_declaration',
                                                          'declared_profile', 'other')),
    evidence_object_id      UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    canon_kyc_document_id   UUID REFERENCES canon.kyc_document(id) ON DELETE RESTRICT,
    attached_by             UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    attached_at             TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (evidence_object_id IS NOT NULL OR canon_kyc_document_id IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_review_documents_review ON kyc.review_documents (review_id);
CREATE INDEX IF NOT EXISTS idx_review_documents_party ON kyc.review_documents (party_id);

-- The KYC declared profile: expected monthly figures per customer record and currency, declared at a completed review.
-- Phase 13's DeclaredProfileSource (variant `kyc`) reads the open row for the period. The CBS field
-- canon.customer.expected_profile is a different source and is never overwritten.
CREATE TABLE IF NOT EXISTS kyc.declared_profiles (
    id                      UUID PRIMARY KEY,
    customer_id             UUID NOT NULL,                                -- canon.customer logical id (no foreign key)
    currency                CHAR(3) NOT NULL CHECK (currency ~ '^[A-Z]{3}$'),
    expected_credit_1m      NUMERIC(20,3) CHECK (expected_credit_1m >= 0),         -- financial
    expected_debit_1m       NUMERIC(20,3) CHECK (expected_debit_1m >= 0),          -- financial
    expected_txn_count_1m   INT CHECK (expected_txn_count_1m >= 0),
    review_id               UUID NOT NULL REFERENCES kyc.kyc_reviews(id) ON DELETE RESTRICT,
    declared_on             DATE NOT NULL,
    valid_from              DATE NOT NULL,
    valid_to                DATE,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (valid_to IS NULL OR valid_to > valid_from),
    CHECK (expected_credit_1m IS NOT NULL OR expected_debit_1m IS NOT NULL OR expected_txn_count_1m IS NOT NULL)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_declared_profiles_open ON kyc.declared_profiles (customer_id, currency)
    WHERE valid_to IS NULL;

-- The PEP register (MODULES M10; DSL.md C.3 FL-03 reads pep.tier and pep.former). Candidates arrive from a decided
-- `pep` list alert (phase 14) or a CBS class the bank designates as political (DR-38); a candidate has no tier and is
-- never a status. A confirmed entry is set by a change request (route cr_minor, section 3.5), not by its proposer.
-- P3 is the highest tier (FL-03 floors `pep.tier == 'P3'`). The tier scheme itself is an open doubt.
CREATE TABLE IF NOT EXISTS kyc.pep_register (
    id                      UUID PRIMARY KEY,
    party_id                UUID,                                         -- NULL for a name-only candidate (no foreign key: bi-temporal)
    name                    JSONB CHECK (name IS NULL OR (name ? 'ar' AND name ? 'en')),        -- personal
    status                  TEXT NOT NULL CHECK (status IN ('candidate', 'confirmed', 'ended', 'rejected')),
    tier                    TEXT CHECK (tier IN ('P1', 'P2', 'P3')),
    position                JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(position) = 'object'),   -- {"ar","en","key"}
    country                 CHAR(2) REFERENCES ref.countries(iso2) ON DELETE RESTRICT,
    office_from             DATE,
    office_to               DATE,
    date_quality            TEXT NOT NULL DEFAULT 'exact' CHECK (date_quality IN ('exact', 'unknown')),
    former                  BOOLEAN NOT NULL DEFAULT FALSE,
    persistence_until       DATE,                                         -- office end plus the model's decay (FL-07 to FL-09)
    assessment_documented   BOOLEAN NOT NULL DEFAULT FALSE,               -- FL-03 `has(pep.assessment_documented)`
    assessment_evidence_id  UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    source                  TEXT NOT NULL CHECK (source IN ('pep_list', 'cbs_class', 'manual', 'declaration')),
    source_alert_id         UUID,                                         -- decided alert (phase 14), no foreign key; never returned by a route
    proposed_by             UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    confirmed_by            UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    change_request_id       UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    version                 INT NOT NULL DEFAULT 1,
    CHECK (status <> 'candidate' OR tier IS NULL),
    CHECK (status <> 'confirmed' OR (tier IS NOT NULL AND party_id IS NOT NULL AND confirmed_by IS NOT NULL
                                     AND confirmed_by <> proposed_by)),
    CHECK (source <> 'pep_list' OR source_alert_id IS NOT NULL),
    CHECK (party_id IS NOT NULL OR name IS NOT NULL),
    CHECK (office_to IS NULL OR office_from IS NULL OR office_to >= office_from)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_pep_register_source_alert ON kyc.pep_register (source_alert_id)
    WHERE source_alert_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_pep_register_party ON kyc.pep_register (party_id) WHERE party_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_pep_register_persistence ON kyc.pep_register (status, persistence_until);

-- Relatives and close associates of a PEP (MODULES M10: kinship to the second degree, confirmed by a person). A
-- `suggested` row comes from a canonical family link (canon.party_relationship, type family, degree 1 or 2); it counts
-- for nothing until a person confirms it.
CREATE TABLE IF NOT EXISTS kyc.pep_relatives (
    id                  UUID PRIMARY KEY,
    pep_id              UUID NOT NULL REFERENCES kyc.pep_register(id) ON DELETE RESTRICT,
    party_id            UUID,                                             -- no foreign key (bi-temporal)
    name                JSONB CHECK (name IS NULL OR (name ? 'ar' AND name ? 'en')),        -- personal
    relation            TEXT NOT NULL CHECK (relation IN ('spouse', 'child', 'parent', 'sibling', 'grandparent',
                                                          'grandchild', 'in_law', 'close_associate',
                                                          'business_partner', 'other')),
    degree              SMALLINT CHECK (degree BETWEEN 1 AND 2),          -- family relations only
    status              TEXT NOT NULL CHECK (status IN ('suggested', 'confirmed', 'rejected', 'ended')),
    basis               TEXT NOT NULL CHECK (basis IN ('canon_family_link', 'manual', 'declaration')),
    proposed_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- NULL for a system suggestion
    confirmed_by        UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    CHECK (party_id IS NOT NULL OR name IS NOT NULL),
    CHECK (status <> 'confirmed' OR confirmed_by IS NOT NULL),
    CHECK (status <> 'suggested' OR basis = 'canon_family_link'),
    CHECK ((relation IN ('close_associate', 'business_partner', 'other')) = (degree IS NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_pep_relatives_open ON kyc.pep_relatives (pep_id, party_id)
    WHERE party_id IS NOT NULL AND status IN ('suggested', 'confirmed');
CREATE INDEX IF NOT EXISTS idx_pep_relatives_pep ON kyc.pep_relatives (pep_id);

-- Ownership entered by the bank where the CBS holds no percentage (MODULES M10). A NULL share is "owner known, share
-- not known": the chain result is then unknown, never clear. A row is closed (valid_to), never edited.
CREATE TABLE IF NOT EXISTS kyc.ownership_inputs (
    id                  UUID PRIMARY KEY,
    owned_party_id      UUID NOT NULL,                                    -- no foreign key (bi-temporal)
    owner_party_id      UUID NOT NULL,
    share_pct           NUMERIC(5,2) CHECK (share_pct IS NULL OR share_pct BETWEEN 0 AND 100),   -- financial
    evidence_object_id  UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    entered_by          UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    valid_from          DATE NOT NULL,
    valid_to            DATE,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (owned_party_id <> owner_party_id),
    CHECK (valid_to IS NULL OR valid_to > valid_from)
);
CREATE INDEX IF NOT EXISTS idx_ownership_inputs_owned ON kyc.ownership_inputs (owned_party_id) WHERE valid_to IS NULL;

-- The beneficial-ownership result of a party (section 3.7). Superseded, never edited: a new computation sets the old row's
-- superseded_at. Chain paths hold party ids and percentages (personal); no name is stored here.
CREATE TABLE IF NOT EXISTS kyc.ownership_results (
    id                  UUID PRIMARY KEY,
    party_id            UUID NOT NULL,
    computed_at         TIMESTAMPTZ NOT NULL,
    as_of               DATE NOT NULL,
    threshold_pct       NUMERIC(5,2) NOT NULL CHECK (threshold_pct > 0 AND threshold_pct <= 100),
    chain               JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(chain) = 'array'),              -- personal
    beneficial_owners   JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(beneficial_owners) = 'array'),  -- personal
    unknown_count       INT NOT NULL DEFAULT 0 CHECK (unknown_count >= 0),
    loops_detected      BOOLEAN NOT NULL DEFAULT FALSE,
    loop_paths          JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(loop_paths) = 'array'),
    depth_truncated     BOOLEAN NOT NULL DEFAULT FALSE,
    input_hash          BYTEA NOT NULL CHECK (octet_length(input_hash) = 32),
    superseded_at       TIMESTAMPTZ
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_ownership_results_current ON kyc.ownership_results (party_id)
    WHERE superseded_at IS NULL;

-- The exit register: parties exited for financial-crime reasons (API.md 3.10). Each entry is recorded from a completed
-- `exit_decision` workflow instance (DSL.md E.4). Suspicion tier: see section 3.6.
CREATE TABLE IF NOT EXISTS kyc.exit_register (
    id                      UUID PRIMARY KEY,
    party_id                UUID NOT NULL,                                -- no foreign key (bi-temporal)
    review_id               UUID NOT NULL REFERENCES kyc.kyc_reviews(id) ON DELETE RESTRICT,
    workflow_instance_id    UUID NOT NULL REFERENCES admin.workflow_instances(id) ON DELETE RESTRICT,
    reason_code             TEXT NOT NULL CHECK (reason_code ~ '^DEC_[A-Z0-9_]{3,56}$'),         -- family decision (phase 09)
    status                  TEXT NOT NULL CHECK (status IN ('recorded', 'closed')),
    recorded_by             UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    recorded_at             TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at               TIMESTAMPTZ,
    CHECK (status <> 'closed' OR closed_at IS NOT NULL),
    CONSTRAINT uq_exit_register_instance UNIQUE (workflow_instance_id)                         -- one entry per exit decision
);
CREATE INDEX IF NOT EXISTS idx_exit_register_party ON kyc.exit_register (party_id);
```

### 1.3 `0052_kyc_guards_views_grants.sql`

```sql
-- migration 0052_kyc_guards_views_grants.sql
-- Phase 15. Guards (the second layer under the grants: no DELETE for akr_app anywhere in kyc), the staff link that the
-- relationship-conflict check reads (section 3.8), the PEP status view, and the grants per write path.

-- The relationship-conflict check needs the link from a login to a staff row (canon.staff, phase 10). Set by the user
-- administration of phase 03 (`PATCH /admin/users/{id}`, field `staff_id`, admin.users scope, ADM; extended, section 4).
ALTER TABLE admin.users ADD COLUMN IF NOT EXISTS staff_id UUID REFERENCES canon.staff(id) ON DELETE RESTRICT;
CREATE UNIQUE INDEX IF NOT EXISTS uq_users_staff ON admin.users (staff_id) WHERE staff_id IS NOT NULL;

-- Generic guards.
CREATE OR REPLACE FUNCTION kyc.refuse_delete() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION 'kyc rows are never deleted: %', TG_TABLE_NAME USING ERRCODE = 'insufficient_privilege';
END $$;

CREATE OR REPLACE FUNCTION kyc.append_only() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION '% is append-only', TG_TABLE_NAME USING ERRCODE = 'insufficient_privilege';
END $$;

-- Rating models: content is frozen once a version leaves draft; statuses move forward only.
CREATE OR REPLACE FUNCTION kyc.rating_models_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF OLD.status <> 'draft' AND (NEW.document IS DISTINCT FROM OLD.document
            OR NEW.document_sha256 IS DISTINCT FROM OLD.document_sha256
            OR NEW.model_id <> OLD.model_id OR NEW.version <> OLD.version
            OR NEW.regulatory_profile <> OLD.regulatory_profile) THEN
        RAISE EXCEPTION 'rating model content is immutable once it leaves draft' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.status IS DISTINCT FROM OLD.status AND NOT (
           (OLD.status = 'draft' AND NEW.status = 'shadow')
        OR (OLD.status = 'shadow' AND NEW.status IN ('live', 'retired'))
        OR (OLD.status = 'live' AND NEW.status = 'retired')) THEN
        RAISE EXCEPTION 'rating model status cannot move from % to %', OLD.status, NEW.status
            USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_rating_models_guard ON kyc.rating_models;
CREATE TRIGGER trg_rating_models_guard BEFORE UPDATE ON kyc.rating_models
    FOR EACH ROW EXECUTE FUNCTION kyc.rating_models_guard();
DROP TRIGGER IF EXISTS trg_rating_models_no_delete ON kyc.rating_models;
CREATE TRIGGER trg_rating_models_no_delete BEFORE DELETE ON kyc.rating_models
    FOR EACH ROW EXECUTE FUNCTION kyc.refuse_delete();

-- The rating history and the shadow items are append-only (partitioned parent: triggers reach every partition).
DROP TRIGGER IF EXISTS trg_ratings_append_only ON kyc.ratings;
CREATE TRIGGER trg_ratings_append_only BEFORE UPDATE OR DELETE ON kyc.ratings
    FOR EACH ROW EXECUTE FUNCTION kyc.append_only();
DROP TRIGGER IF EXISTS trg_rating_shadow_items_append_only ON kyc.rating_shadow_items;
CREATE TRIGGER trg_rating_shadow_items_append_only BEFORE UPDATE OR DELETE ON kyc.rating_shadow_items
    FOR EACH ROW EXECUTE FUNCTION kyc.append_only();

-- The feed: only delivered_at may be set, once.
CREATE OR REPLACE FUNCTION kyc.rating_feed_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'rating_feed rows are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.seq <> OLD.seq OR NEW.party_id <> OLD.party_id OR NEW.cbi_level <> OLD.cbi_level
       OR NEW.cif <> OLD.cif OR NEW.effective_at <> OLD.effective_at OR NEW.rating_id <> OLD.rating_id
       OR (OLD.delivered_at IS NOT NULL AND NEW.delivered_at IS DISTINCT FROM OLD.delivered_at) THEN
        RAISE EXCEPTION 'rating_feed: only delivered_at may be set, once' USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_rating_feed_guard ON kyc.rating_feed;
CREATE TRIGGER trg_rating_feed_guard BEFORE UPDATE OR DELETE ON kyc.rating_feed
    FOR EACH ROW EXECUTE FUNCTION kyc.rating_feed_guard();

-- Overrides: the decision fields are fixed at proposal; only the lifecycle moves.
CREATE OR REPLACE FUNCTION kyc.overrides_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'overrides are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.party_id <> OLD.party_id OR NEW.direction <> OLD.direction OR NEW.computed_level <> OLD.computed_level
       OR NEW.to_level <> OLD.to_level OR NEW.reason_code <> OLD.reason_code OR NEW.reason_text <> OLD.reason_text
       OR NEW.expires_at <> OLD.expires_at OR NEW.proposed_by <> OLD.proposed_by
       OR NEW.change_request_id IS DISTINCT FROM OLD.change_request_id
       OR NEW.evidence_object_id IS DISTINCT FROM OLD.evidence_object_id THEN
        RAISE EXCEPTION 'override decision fields are fixed at proposal' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.status IS DISTINCT FROM OLD.status AND NOT (
           (OLD.status = 'proposed' AND NEW.status IN ('active', 'rejected', 'withdrawn'))
        OR (OLD.status = 'active' AND NEW.status IN ('ended', 'withdrawn'))) THEN
        RAISE EXCEPTION 'override status cannot move from % to %', OLD.status, NEW.status
            USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_overrides_guard ON kyc.overrides;
CREATE TRIGGER trg_overrides_guard BEFORE UPDATE OR DELETE ON kyc.overrides
    FOR EACH ROW EXECUTE FUNCTION kyc.overrides_guard();

-- A completed review is a record: no change to it, ever.
CREATE OR REPLACE FUNCTION kyc.kyc_reviews_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'kyc_reviews are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.status = 'completed' THEN
        RAISE EXCEPTION 'a completed KYC review is immutable' USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_kyc_reviews_guard ON kyc.kyc_reviews;
CREATE TRIGGER trg_kyc_reviews_guard BEFORE UPDATE OR DELETE ON kyc.kyc_reviews
    FOR EACH ROW EXECUTE FUNCTION kyc.kyc_reviews_guard();

-- PEP register: a confirmed entry never returns to candidate; ended and rejected entries are fixed.
CREATE OR REPLACE FUNCTION kyc.pep_register_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'pep_register rows are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.status IN ('ended', 'rejected') AND NEW.status IS DISTINCT FROM OLD.status THEN
        RAISE EXCEPTION 'a % PEP entry does not change status', OLD.status USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.status = 'confirmed' AND NEW.status = 'candidate' THEN
        RAISE EXCEPTION 'a confirmed PEP entry cannot return to candidate' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.source_alert_id IS DISTINCT FROM OLD.source_alert_id OR NEW.proposed_by <> OLD.proposed_by THEN
        RAISE EXCEPTION 'PEP source and proposer are fixed' USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_pep_register_guard ON kyc.pep_register;
CREATE TRIGGER trg_pep_register_guard BEFORE UPDATE OR DELETE ON kyc.pep_register
    FOR EACH ROW EXECUTE FUNCTION kyc.pep_register_guard();

-- Declared profiles: a row is closed (valid_to) and never edited.
CREATE OR REPLACE FUNCTION kyc.declared_profiles_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'declared profiles are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.customer_id <> OLD.customer_id OR NEW.currency <> OLD.currency
       OR NEW.expected_credit_1m IS DISTINCT FROM OLD.expected_credit_1m
       OR NEW.expected_debit_1m IS DISTINCT FROM OLD.expected_debit_1m
       OR NEW.expected_txn_count_1m IS DISTINCT FROM OLD.expected_txn_count_1m
       OR NEW.review_id <> OLD.review_id OR NEW.valid_from <> OLD.valid_from THEN
        RAISE EXCEPTION 'a declared profile row is closed, not edited' USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_declared_profiles_guard ON kyc.declared_profiles;
CREATE TRIGGER trg_declared_profiles_guard BEFORE UPDATE OR DELETE ON kyc.declared_profiles
    FOR EACH ROW EXECUTE FUNCTION kyc.declared_profiles_guard();

-- Ownership results: only superseded_at may be set.
CREATE OR REPLACE FUNCTION kyc.ownership_results_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'ownership results are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.party_id <> OLD.party_id OR NEW.chain IS DISTINCT FROM OLD.chain
       OR NEW.beneficial_owners IS DISTINCT FROM OLD.beneficial_owners
       OR NEW.input_hash <> OLD.input_hash OR NEW.computed_at <> OLD.computed_at THEN
        RAISE EXCEPTION 'ownership results are superseded, not edited' USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_ownership_results_guard ON kyc.ownership_results;
CREATE TRIGGER trg_ownership_results_guard BEFORE UPDATE OR DELETE ON kyc.ownership_results
    FOR EACH ROW EXECUTE FUNCTION kyc.ownership_results_guard();

-- Tables that are never updated in a way the guard above does not cover: no DELETE at all.
DO $$
DECLARE
    v_t TEXT;
BEGIN
    FOREACH v_t IN ARRAY ARRAY['rating_shadow_runs', 'review_queries', 'review_documents', 'pep_relatives',
                               'ownership_inputs', 'exit_register'] LOOP
        EXECUTE pg_catalog.format('DROP TRIGGER IF EXISTS trg_%I_no_delete ON kyc.%I', v_t, v_t);
        EXECUTE pg_catalog.format('CREATE TRIGGER trg_%I_no_delete BEFORE DELETE ON kyc.%I
                                   FOR EACH ROW EXECUTE FUNCTION kyc.refuse_delete()', v_t, v_t);
    END LOOP;
END $$;

-- The PEP status of a party, read by the rating factors (B-block PEP floors) and by the party page. A projection, not a
-- stored status: canon.party.pep_status (phase 10) is CBS-derived and is never written by this phase, because the
-- canonical tables take writes from akr_ingest only.
CREATE OR REPLACE VIEW kyc.party_pep_state AS
SELECT r.party_id,
       CASE
           WHEN bool_or(r.status = 'confirmed' AND NOT r.former) THEN 'pep'
           WHEN bool_or(r.status = 'confirmed' AND r.former
                        AND (r.persistence_until IS NULL OR r.persistence_until >= CURRENT_DATE)) THEN 'former_pep'
           WHEN bool_or(r.status = 'candidate') THEN 'candidate'
           ELSE 'none'                                   -- a former entry whose persistence has passed
       END AS pep_state,
       MAX(r.tier) FILTER (WHERE r.status = 'confirmed'
                           AND (NOT r.former OR r.persistence_until >= CURRENT_DATE)) AS tier,
       BOOL_OR(r.assessment_documented) AS assessment_documented
FROM kyc.pep_register r
WHERE r.party_id IS NOT NULL AND r.status IN ('candidate', 'confirmed')
GROUP BY r.party_id;

-- Grants per write path. No DELETE on any kyc table. Column lists name every column a service updates.
GRANT USAGE ON SCHEMA kyc TO akr_app, akr_readonly;
GRANT UPDATE (staff_id) ON admin.users TO akr_app;

GRANT SELECT, INSERT ON kyc.rating_models TO akr_app;
GRANT UPDATE (status, document, document_sha256, validation, change_request_id, effective_from, retired_at,
              updated_at, row_version) ON kyc.rating_models TO akr_app;
GRANT SELECT, INSERT ON kyc.rating_shadow_runs, kyc.rating_shadow_items TO akr_app;
GRANT UPDATE (status, parties_total, parties_scored, parties_control_failure, level_up, level_down, tier_migration,
              sensitivity, volume_effect, failure_code, started_at, finished_at) ON kyc.rating_shadow_runs TO akr_app;
GRANT SELECT, INSERT ON kyc.overrides TO akr_app;
GRANT UPDATE (status, end_reason, approved_by, effective_from, ended_at, row_version) ON kyc.overrides TO akr_app;
GRANT SELECT, INSERT ON kyc.ratings TO akr_app;
GRANT SELECT, INSERT ON kyc.rating_current TO akr_app;
GRANT UPDATE (rating_id, rating_effective_on, model_version_id, rating_status, level, cbi_level, score, degraded,
              computed_at, updated_at) ON kyc.rating_current TO akr_app;
GRANT SELECT, INSERT ON kyc.rating_feed TO akr_app;
GRANT UPDATE (delivered_at) ON kyc.rating_feed TO akr_app;

GRANT SELECT, INSERT ON kyc.kyc_reviews TO akr_app;
GRANT UPDATE (status, trigger_code, due_at, due_basis, date_unknown, assignee_id, claimed_at, escalated_at, checklist,
              outcome, outcome_detail, completed_by, completed_at, updated_at, version) ON kyc.kyc_reviews TO akr_app;
GRANT SELECT, INSERT ON kyc.review_queries, kyc.review_documents TO akr_app;
GRANT UPDATE (status, answer, answered_by, answered_at, closed_by, closed_at) ON kyc.review_queries TO akr_app;
GRANT SELECT, INSERT ON kyc.declared_profiles TO akr_app;
GRANT UPDATE (valid_to) ON kyc.declared_profiles TO akr_app;

GRANT SELECT, INSERT ON kyc.pep_register, kyc.pep_relatives, kyc.ownership_inputs TO akr_app;
GRANT UPDATE (party_id, name, status, tier, position, country, office_from, office_to, date_quality, former,
              persistence_until, assessment_documented, assessment_evidence_id, confirmed_by, change_request_id,
              updated_at, version) ON kyc.pep_register TO akr_app;
GRANT UPDATE (party_id, name, status, confirmed_by, updated_at, version) ON kyc.pep_relatives TO akr_app;
GRANT UPDATE (valid_to) ON kyc.ownership_inputs TO akr_app;
GRANT SELECT, INSERT ON kyc.ownership_results TO akr_app;
GRANT UPDATE (superseded_at) ON kyc.ownership_results TO akr_app;
GRANT SELECT, INSERT ON kyc.exit_register TO akr_app;
GRANT UPDATE (status, closed_at) ON kyc.exit_register TO akr_app;

GRANT EXECUTE ON FUNCTION kyc.create_partition(TEXT, INT, INT) TO akr_app;
GRANT SELECT ON ALL TABLES IN SCHEMA kyc TO akr_readonly;
GRANT SELECT ON kyc.party_pep_state TO akr_app, akr_readonly;
```

### 1.4 `0053_kyc_settings_schedules_classification.sql`

```sql
-- migration 0053_kyc_settings_schedules_classification.sql
-- Phase 15. Settings and schedules (admin registry, phase 00), column classification and retention rows for every table
-- of section 1.1 to 1.2. The rating model document itself is data (kyc.rating_models), not a setting.

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('ownership.threshold_pct', '20', 'decimal', NULL, TRUE,
  '{"ar": "حد نسبة الملكية للمستفيد الحقيقي (%)", "en": "Beneficial-owner threshold (per cent; CBI default 20, bank may set)"}'),
 ('ownership.max_depth', '6', 'integer', NULL, FALSE,
  '{"ar": "أقصى عمق لسلسلة الملكية", "en": "Deepest layer followed in an ownership chain; deeper paths mark depth_truncated"}'),
 ('kyc.light_yearly_months', '12', 'integer', NULL, TRUE,
  '{"ar": "الفحص الخفيف السنوي لكل عميل (أشهر)", "en": "Light yearly check interval for every customer (months)"}'),
 ('kyc.declared_max_age_months', '12', 'integer', '{"min": 1, "basis": "proposed starting value; the bank confirms (phase 15 Open doubts)"}', TRUE,
  '{"ar": "أقصى عمر للبيانات المعلنة للعميل (أشهر)", "en": "Age after which a declared profile counts as stale (months)"}'),
 ('kyc.review_open_ahead_days', '30', 'integer', NULL, FALSE,
  '{"ar": "فتح المراجعة قبل موعدها (أيام)", "en": "Periodic reviews open this many days before their due date"}'),
 ('kyc.feed_export_enabled', 'false', 'boolean', NULL, FALSE,
  '{"ar": "تفعيل تصدير ملف تصنيف المخاطر", "en": "Write the rating feed export file (off until the bank asks for it)"}'),
 ('rating.trigger_debounce_s', '60', 'integer', NULL, FALSE,
  '{"ar": "مهلة تجميع أحداث إعادة التقييم (ثوان)", "en": "Seconds a party waits to collect re-scoring triggers"}'),
 ('rating.page_size', '5000', 'integer', NULL, FALSE,
  '{"ar": "حجم صفحة إعادة التقييم الليلية", "en": "Parties per page of the nightly re-scoring"}'),
 ('rating.country_fanout_page', '2000', 'integer', NULL, FALSE,
  '{"ar": "حجم صفحة تغيير مستوى الدولة", "en": "Parties per page when a country level changes (TR-10)"}'),
 ('screening.pair_memory_pep_max_months', '24', 'integer', NULL, TRUE,
  '{"ar": "أقصى مدة لذاكرة الأزواج لقوائم الأشخاص السياسيين (أشهر)", "en": "Longest pair-memory row for a pep list entry (months; phase 14 cap is 12 otherwise)"}'),
 ('screening.rating_modifiers', '{"1": 0, "2": 0, "3": 0, "4": 0, "5": 0}', 'json', NULL, TRUE,
  '{"ar": "تعديلات حد التنبيه حسب مستوى المخاطر", "en": "Additive screening-threshold modifiers per rating level (zero until the bank sets them)"}'),
 ('kyc.pep_cbs_class_values', '[]', 'json', NULL, TRUE,
  '{"ar": "فئات العملاء المعدّة كأشخاص سياسيين", "en": "CBS customer classes the bank designates as political (candidates only, no tier)"}')
ON CONFLICT (key) DO NOTHING;

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('schedule.rating_nightly',        '"0 30 1 * * *"',   'cron', NULL, FALSE,
  '{"ar": "جدول إعادة التقييم الليلية", "en": "Nightly re-scoring (bank time)"}'),
 ('schedule.kyc_review_sweep',      '"0 0 2 * * *"',    'cron', NULL, FALSE,
  '{"ar": "جدول فتح مراجعات العناية الواجبة", "en": "KYC review sweep (bank time)"}'),
 ('schedule.pep_list_sweep',        '"0 */10 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول قراءة قرارات قوائم الأشخاص السياسيين", "en": "PEP list decisions read every ten minutes"}'),
 ('schedule.pep_persistence_sweep', '"0 15 2 * * *"',   'cron', NULL, FALSE,
  '{"ar": "جدول انتهاء فترة استمرار صفة الشخص السياسي", "en": "PEP persistence sweep (bank time)"}'),
 ('schedule.rating_feed_export',    '"0 0 6 * * *"',    'cron', NULL, FALSE,
  '{"ar": "جدول تصدير ملف تصنيف المخاطر", "en": "Rating feed export (bank time)"}')
ON CONFLICT (key) DO NOTHING;

-- Column classification (admin.column_classification). Identifiers and lifecycle columns are internal; free text,
-- names, facts about a person and the chain are personal; amounts are financial; model logic is detection-logic.
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'kyc', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('rating_models', 'internal', ARRAY['id','model_id','version','status','regulatory_profile','document_sha256',
        'change_request_id','effective_from','retired_at','created_by','created_at','updated_at','row_version']),
  ('rating_models', 'detection-logic', ARRAY['document','validation']),
  ('rating_shadow_runs', 'internal', ARRAY['id','model_version_id','as_of','status','parties_total','parties_scored',
        'parties_control_failure','level_up','level_down','volume_effect','failure_code','requested_by','started_at',
        'finished_at','created_at']),
  ('rating_shadow_runs', 'detection-logic', ARRAY['tier_migration','sensitivity']),
  ('rating_shadow_items', 'internal', ARRAY['run_id','party_id','live_level','shadow_level','shadow_cbi_level',
        'shadow_control_failure','created_at']),
  ('rating_shadow_items', 'personal', ARRAY['drivers_delta']),
  ('overrides', 'internal', ARRAY['id','party_id','direction','computed_level','to_level','reason_code','status',
        'end_reason','change_request_id','evidence_object_id','proposed_by','approved_by','effective_from','expires_at',
        'ended_at','created_at','row_version']),
  ('overrides', 'personal', ARRAY['reason_text']),
  ('ratings', 'internal', ARRAY['id','party_id','model_version_id','effective_on','computed_at','input_hash',
        'rating_status','level','cbi_level','score','inherent_score','peak_term','floors_fired','cap_applied','stickiness',
        'override_id','missing_share','degraded','degraded_reasons','trigger_code','data_as_of']),
  ('ratings', 'detection-logic', ARRAY['block_scores']),
  ('ratings', 'personal', ARRAY['factors','drivers']),
  ('rating_current', 'internal', ARRAY['party_id','rating_id','rating_effective_on','model_version_id','rating_status',
        'level','cbi_level','score','degraded','computed_at','updated_at']),
  ('rating_feed', 'internal', ARRAY['seq','id','party_id','previous_cbi_level','cbi_level','rating_id',
        'rating_effective_on','effective_at','created_at','delivered_at']),
  ('rating_feed', 'personal', ARRAY['cif']),
  ('kyc_reviews', 'internal', ARRAY['id','party_id','customer_id','kind','trigger_code','due_at','due_basis','date_unknown',
        'rating_id','rating_effective_on','level_at_open','status','assignee_id','claimed_at','escalated_at',
        'checklist_template','outcome','completed_by','completed_at','created_at','updated_at','version']),
  ('kyc_reviews', 'personal', ARRAY['checklist','outcome_detail']),
  ('review_queries', 'internal', ARRAY['id','review_id','item_key','status','raised_by','answered_by','answered_at',
        'closed_by','closed_at','created_at']),
  ('review_queries', 'personal', ARRAY['text','answer']),
  ('review_documents', 'internal', ARRAY['id','review_id','party_id','item_key','kind','evidence_object_id',
        'canon_kyc_document_id','attached_by','attached_at']),
  ('declared_profiles', 'internal', ARRAY['id','customer_id','currency','review_id','declared_on','valid_from',
        'valid_to','created_at']),
  ('declared_profiles', 'financial', ARRAY['expected_credit_1m','expected_debit_1m','expected_txn_count_1m']),
  ('pep_register', 'internal', ARRAY['id','party_id','status','tier','country','office_from','office_to','date_quality',
        'former','persistence_until','assessment_documented','assessment_evidence_id','source','proposed_by',
        'confirmed_by','change_request_id','created_at','updated_at','version']),
  ('pep_register', 'personal', ARRAY['name','position']),
  ('pep_register', 'detection-logic', ARRAY['source_alert_id']),
  ('pep_relatives', 'internal', ARRAY['id','pep_id','party_id','relation','degree','status','basis','proposed_by',
        'confirmed_by','created_at','updated_at','version']),
  ('pep_relatives', 'personal', ARRAY['name']),
  ('ownership_inputs', 'internal', ARRAY['id','owned_party_id','owner_party_id','evidence_object_id','entered_by',
        'valid_from','valid_to','created_at']),
  ('ownership_inputs', 'financial', ARRAY['share_pct']),
  ('ownership_results', 'internal', ARRAY['id','party_id','computed_at','as_of','threshold_pct','unknown_count',
        'loops_detected','depth_truncated','input_hash','superseded_at']),
  ('ownership_results', 'personal', ARRAY['chain','beneficial_owners','loop_paths']),
  ('exit_register', 'internal', ARRAY['id','party_id','review_id','workflow_instance_id','reason_code','status',
        'recorded_by','recorded_at','closed_at'])
) AS v(t, cls, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

-- Retention (ARCHITECTURE s.17). Customer-linked classes use the owner clock (follows the party, as canon.party does in
-- phase 10); the retention engine resolves the relationship end. Child tables follow their parent.
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('kyc', 'rating_models',       'R9',  'retirement', NULL,         'Model versions: configuration history; every rating cites its version'),
 ('kyc', 'rating_shadow_runs',  'R11', 'age',        'created_at', 'Model validation evidence (DSL.md C.9)'),
 ('kyc', 'rating_shadow_items', 'R11', 'age',        'created_at', 'Disagreement list of a shadow run, kept with its run'),
 ('kyc', 'ratings',             'R8',  'owner',      NULL,         'Rating history; purged record by record on the party clock, never by partition drop'),
 ('kyc', 'rating_current',      'R8',  'owner',      NULL,         'Pointer to the current rating of each party'),
 ('kyc', 'overrides',           'R8',  'owner',      NULL,         'Overrides with their reasons and approvers'),
 ('kyc', 'rating_feed',         'R8',  'owner',      NULL,         'One-way feed rows; the bank keeps its own copy'),
 ('kyc', 'kyc_reviews',         'R8',  'owner',      NULL,         'Review history with outcomes'),
 ('kyc', 'review_queries',      'R8',  'owner',      NULL,         'Follows its review and party'),
 ('kyc', 'review_documents',    'R8',  'owner',      NULL,         'Evidence links; the evidence objects keep their own class'),
 ('kyc', 'declared_profiles',   'R3',  'owner',      NULL,         'KYC profile (ARCHITECTURE s.17 R3): follows the customer'),
 ('kyc', 'pep_register',        'R8',  'owner',      NULL,         'PEP records; name-only candidates need an age clock (Open doubts)'),
 ('kyc', 'pep_relatives',       'R8',  'owner',      NULL,         'Follows its PEP entry'),
 ('kyc', 'ownership_inputs',    'R3',  'owner',      NULL,         'Ownership facts about the party'),
 ('kyc', 'ownership_results',   'R8',  'owner',      NULL,         'Derived ownership result, kept with the party'),
 ('kyc', 'exit_register',       'R5',  'age',        'recorded_at', 'Financial-crime exit: a disposition (seven years from the record)')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

### 1.5 Tables, classes and writers

| Table | Migration | Class | Partitioned | Written by (service) | Read by |
|-|-|-|-|-|-|
| `kyc.rating_models` | 0050 | R9 | no | `RatingModelService` (CR `rating_model`; `rating seed`) | engine, K1, CFG, MRO |
| `kyc.rating_shadow_runs`, `kyc.rating_shadow_items` | 0050 | R11 | no | `RatingShadowService` (job `rating_shadow_run`) | CFG, MRO, CHK, CH, M |
| `kyc.overrides` | 0050 | R8 | no | `OverrideService` (upward direct; downward through CR) | engine, K1, overrides list |
| `kyc.ratings` | 0050 | R8 | monthly on `effective_on` | `RatingEngine` (the only writer) | K1 history, explanation, feed |
| `kyc.rating_current` | 0050 | R8 | no | `RatingEngine`, same transaction as the history row | lists, segment risk overlay (phase 13), phase 14 modifiers |
| `kyc.rating_feed` | 0050 | R8 | no | `RatingEngine` (on a CBI-label change); `rating_feed_export` sets `delivered_at` | `GET /rating/feed` |
| `kyc.kyc_reviews`, `kyc.review_queries`, `kyc.review_documents` | 0051 | R8 | no | `KycReviewService` | K2, K1 strip |
| `kyc.declared_profiles` | 0051 | R3 | no | `KycReviewService` at completion | phase 13 `DeclaredProfileSource` (variant kyc) |
| `kyc.pep_register`, `kyc.pep_relatives` | 0051 | R8 | no | `PepService` (candidates by job `pep_list_sweep`; entries by CR `pep_entry`) | K4, engine (`pep.*`), screening pair memory (phase 14) |
| `kyc.ownership_inputs`, `kyc.ownership_results` | 0051 | R3, R8 | no | `OwnershipService` | K3, engine (`ownership.*`) |
| `kyc.exit_register` | 0051 | R5 | no | `ExitService` through `POST /exit-register` (M), from a completed `exit_decision` instance | `GET /exit-register` (M) |
| `kyc.party_pep_state` (view) | 0052 | none | n/a | none (projection) | engine, K1, K4 |

Only `RatingEngine` writes `kyc.ratings`, `kyc.rating_current` and `kyc.rating_feed`: the rating is one function of
one snapshot, and a second writer would make a past rating impossible to reconstruct (K-26). `akr_ingest` and
`akr_assess` have no privilege on any `kyc` table; `akr_readonly` reads all of them for reports.

---

## 2. DDD Implementation (Rust)

**Crate placement** (workspace pinned in phase 00; layer rules in `ddd.md`):

```
backend/
├── packs/rating/rm-iq.v1.0.0.json            # Part C document of RM-IQ 1.0.0 (section 3.1); listed in the release manifest
├── packs/rating/rm-iq.golden.v1.0.0.json     # research 26 s.14e worked examples and their expected levels
├── schemas/rating-model.v1.json              # JSON Schema of DSL.md Part C (answers GET /dsl/rating_model/schema, phase 04)
├── crates/
│   ├── domain/   # + kyc: RatingModelDoc (and every Part C type), RatingEngine (pure), OwnershipCalculator (pure),
│   │             #   PepRules (pure), ReviewClock (pure), KycReviewService, OverrideService, PepService, RatingService
│   │             #   (orchestration), ports (PartyFactsSource, DeclaredProfileSource impl, RatingSource impl,
│   │             #   RiskModifierSource, RelationshipConflictCheck)
│   ├── db/       # + Pg repositories for the 16 tables of section 1; PgPartyFactsSource (canon, read only)
│   ├── authz/    # + resource adapters Customer (EXTENDED: tier; phase 13 registered it unmasked), KycReview, Override,
│   │             #   PepEntry, ExitEntry; masked kinds registered (section 3.6)
│   └── jobs/     # + the kinds of section 4.1
├── api/          # + routers rating, kyc, pep, ownership, exit, rating_models; change targets in process
├── worker/       # + job handlers; RatingFanout; the evidence owner kind kyc_review (phase 02 registry)
└── ctl/          # + rating seed, rating verify-golden
```

The engine is product code and the model is data. `RatingEngine` has no IO: the job of section 3.2 gathers one
`PartyFacts` snapshot, the engine returns a `RatingOutcome`, and the service writes it. Decimal arithmetic uses fixed
scale (`rust_decimal`, scale 6 inside the engine, scale 2 stored); no `f64` touches a score.

- **Domain entities** (`Debug, Serialize, Deserialize, FromRow, ToSchema`; fields mirror section 1): `RatingModel`
  (`status` is TEXT in the table and maps to the `RatingModelStatus` enum in `domain`),
  `RatingShadowRun`, `ShadowItem`, `Rating` (the history row, partitioned), `RatingCurrent`, `Override`, `FeedRow`,
  `KycReview`, `ReviewQuery`, `ReviewDocument`, `DeclaredProfile`, `PepEntry`, `PepRelative`, `OwnershipInput`,
  `OwnershipResult`, `ExitEntry`.
- **Value objects:** `RatingLevel` (1 to 5; `TryFrom<u8>`), `CbiLabel` (`High`, `Medium`, `Low`; derived from the
  level through `levels[].cbi` of the model, checked against the migration CHECK), `Score` (Decimal 0 to 100, scale
  2), `FactorId` (`A1`..`F7`), `BlockId` (`A`..`F`), `FactorState` (`Known`, `Unknown`, `Stale`, `NotApplicable`,
  `NotWired`, `Withheld`), `Stickiness` (`None`, `HeldFall`), `CapKind` (`MinLevel3`, `ControlFailure`), `Direction` (`Up`,
  `Down`), `TriggerCode` (`TR-nn` or a neutral `TRG_*` code), `DueBasis`, `ReviewKind`, `ReviewStatus`,
  `ReviewOutcome`, `PepTier` (`P1`, `P2`, `P3`; `Ord`, `P3` highest), `PepStatus`, `PepSource`, `Relation`,
  `Degree` (1 or 2), `ShareInput` (`Known(Decimal)` or `NotKnown`), `DegradedReason`.
- **Create and update structs:** `NewRatingModelDraft { model_id, version, document }`, `DraftPatch { document,
  row_version }`, `NewOverride { party_id, direction, to_level, reason_code, reason_text, expires_at,
  evidence_object_id?, change_request? }` (`Deserialize, ToSchema`), `ReviewClaim { version }`, `ReviewPatch { checklist,
  trigger_code?, version }`, `QueryCreate { item_key?, text }`, `QueryPatch { answer?, close: bool, version }`,
  `CompleteReview { outcome, outcome_detail, declared: Vec<DeclaredFigures>, version }`, `NewPepEntry { party_id?,
  name?, position, country?, office_from?, office_to?, date_quality, former, source, source_alert_id? }`,
  `PepPatch { tier?, office_from?, office_to?, persistence_until?, assessment_documented?, assessment_evidence_id?,
  version }`, `RelativeInput { pep_id, party_id? , name?, relation, degree? }`, `OwnershipInputSet { owners: Vec<OwnerLink> }`
  (each `OwnerLink { owner_party_id, share: ShareInput, evidence_object_id? }`), `NewExitEntry { party_id,
  workflow_instance_id, reason_code }`.
- **Repository traits** (domain layer; `Pg*` implementations in `db`; `Tx` is the phase 00 transaction type):

```rust
#[async_trait]
pub trait RatingModelRepository: Send + Sync {
    async fn live(&self, conn: &mut PgConnection, model_id: &str) -> Result<Option<RatingModel>, RepoError>;
    async fn get(&self, conn: &mut PgConnection, model_id: &str, version: &str) -> Result<Option<RatingModel>, RepoError>;
    async fn insert_draft(&self, tx: &mut Tx, m: &NewRatingModelDraft, by: Uuid) -> Result<RatingModel, RepoError>;
    async fn update_draft(&self, tx: &mut Tx, id: Uuid, p: &DraftPatch, expected: i32) -> Result<RatingModel, RepoError>; // 409 on race
    async fn transition(&self, tx: &mut Tx, id: Uuid, from: RatingModelStatus, to: RatingModelStatus,
                        cr: Option<Uuid>, effective_from: Option<DateTime<Utc>>) -> Result<RatingModel, RepoError>;
}

#[async_trait]
pub trait RatingRepository: Send + Sync {
    /// Inserts the history row (partition by effective_on, created by the factory on a missing month).
    async fn insert(&self, tx: &mut Tx, r: &NewRating) -> Result<(), RepoError>;
    /// Upsert of the pointer in the same transaction as the history row.
    async fn upsert_current(&self, tx: &mut Tx, c: &RatingCurrent) -> Result<(), RepoError>;
    async fn current(&self, conn: &mut PgConnection, party: Uuid) -> Result<Option<RatingCurrent>, RepoError>;
    async fn current_many(&self, conn: &mut PgConnection, parties: &[Uuid]) -> Result<Vec<RatingCurrent>, RepoError>;
    async fn history(&self, conn: &mut PgConnection, party: Uuid, page: Keyset) -> Result<Page<Rating>, RepoError>;
    async fn by_id(&self, conn: &mut PgConnection, id: Uuid, effective_on: NaiveDate) -> Result<Option<Rating>, RepoError>;
    async fn last_input_hash(&self, conn: &mut PgConnection, party: Uuid) -> Result<Option<[u8; 32]>, RepoError>;
}

#[async_trait]
pub trait OverrideRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, o: &NewOverride, by: Uuid) -> Result<Override, RepoError>;   // partial unique index: 409
    async fn set_status(&self, tx: &mut Tx, id: Uuid, expected: i32, to: OverrideStatus, approved_by: Option<Uuid>,
                        effective_from: Option<DateTime<Utc>>, end: Option<EndReason>) -> Result<Override, RepoError>;
    async fn open_for_party(&self, conn: &mut PgConnection, party: Uuid) -> Result<Option<Override>, RepoError>;
    async fn active_for_parties(&self, conn: &mut PgConnection, parties: &[Uuid], now: DateTime<Utc>) -> Result<Vec<Override>, RepoError>;
    async fn expire_due(&self, tx: &mut Tx, now: DateTime<Utc>) -> Result<Vec<Uuid>, RepoError>;   // ids for the audit and re-score
    async fn list(&self, conn: &mut PgConnection, f: &OverrideFilter, page: Keyset) -> Result<Page<Override>, RepoError>;
}

#[async_trait]
pub trait KycReviewRepository: Send + Sync {
    /// Opens a review unless one of the same party and kind is open. A new event trigger on
    /// an open event review updates its trigger_code and returns created = false.
    /// The review, whether it was created, and the due_at the open row had before the statement (7.18).
    async fn open_or_join(&self, tx: &mut Tx, r: &NewReview) -> Result<(KycReview, bool, Option<DateTime<Utc>>), RepoError>;
    async fn claim(&self, tx: &mut Tx, id: Uuid, by: Uuid, expected: i32) -> Result<KycReview, RepoError>;   // E_CLAIMED, E_VERSION_CONFLICT
    async fn save(&self, tx: &mut Tx, id: Uuid, p: &ReviewPatch, by: Uuid, expected: i32) -> Result<KycReview, RepoError>;
    async fn complete(&self, tx: &mut Tx, id: Uuid, c: &CompleteReview, by: Uuid, expected: i32) -> Result<KycReview, RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<KycReview>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &ReviewFilter, page: Keyset) -> Result<Page<KycReview>, RepoError>;
    async fn due_for_sweep(&self, conn: &mut PgConnection, today: NaiveDate, ahead_days: i32) -> Result<Vec<DueCandidate>, RepoError>;
}

#[async_trait]
pub trait PepRepository: Send + Sync {
    /// ON CONFLICT (source_alert_id) DO NOTHING; returns true when a candidate was created.
    async fn insert_candidate(&self, tx: &mut Tx, c: &NewPepCandidate) -> Result<bool, RepoError>;
    async fn insert(&self, tx: &mut Tx, e: &NewPepEntry, by: Uuid) -> Result<PepEntry, RepoError>;
    async fn set_confirmed(&self, tx: &mut Tx, id: Uuid, tier: PepTier, confirmed_by: Uuid, cr: Uuid, expected: i32)
        -> Result<PepEntry, RepoError>;
    async fn update(&self, tx: &mut Tx, id: Uuid, p: &PepPatch, expected: i32) -> Result<PepEntry, RepoError>;
    async fn due_persistence(&self, conn: &mut PgConnection, today: NaiveDate) -> Result<Vec<Uuid>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &PepFilter, page: Keyset) -> Result<Page<PepEntry>, RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<PepEntry>, RepoError>;
    async fn upsert_relative(&self, tx: &mut Tx, r: &NewRelative) -> Result<PepRelative, RepoError>;
}

#[async_trait]
pub trait OwnershipRepository: Send + Sync {
    async fn current(&self, conn: &mut PgConnection, party: Uuid) -> Result<Option<OwnershipResult>, RepoError>;
    /// Sets superseded_at on the current row and inserts the new one (same transaction).
    async fn supersede_and_insert(&self, tx: &mut Tx, party: Uuid, r: &NewOwnershipResult) -> Result<OwnershipResult, RepoError>;
    async fn open_inputs(&self, conn: &mut PgConnection, owned: Uuid) -> Result<Vec<OwnershipInput>, RepoError>;
    async fn replace_inputs(&self, tx: &mut Tx, owned: Uuid, set: &OwnershipInputSet, by: Uuid, as_of: NaiveDate)
        -> Result<u64, RepoError>;   // closes open rows (valid_to) and inserts the new set
}
```
- **Repository impl notes** (`db`): the history insert and the pointer upsert share one transaction; the
  `ratings` insert uses `ON CONFLICT DO NOTHING` on the primary key only (ids are new); a missing month raises
  SQLSTATE 23514 and the writer calls `kyc.create_partition` once and retries (the phase 10 and 13 pattern). The
  `open_or_join` is one statement: `INSERT ... ON CONFLICT (party_id, kind) WHERE status IN ('open', 'in_progress',
  'awaiting_answer') DO UPDATE SET trigger_code = EXCLUDED.trigger_code, due_at = CASE WHEN r.kind = 'event' AND NOT r.date_unknown THEN LEAST(r.due_at, EXCLUDED.due_at) ELSE r.due_at END RETURNING ...` (the partial unique index
  `uq_kyc_reviews_open`; `created` is `xmax = 0` in the returned row).
  `PgPartyFactsSource` reads the canonical tables with `recorded_to IS NULL` and `valid_from <= as_of <= valid_to`,
  and never joins across `canon` and `alert` tables.

**Ports** (`domain::ports`, section 3.8):

```rust
#[async_trait]
pub trait PartyFactsSource: Send + Sync {
    async fn snapshot(&self, conn: &mut PgConnection, party: Uuid, as_of: NaiveDate) -> Result<PartyFacts, RepoError>;
}
// Phase 13 ports implemented here: `RatingSource` (reads kyc.rating_current) and the `kyc` variant of
// `DeclaredProfileSource` (reads the open kyc.declared_profiles row). Both are registered in AppState and the worker.
#[async_trait]
pub trait RiskModifierSource: Send + Sync {
    /// Additive modifier per level from `screening.rating_modifiers`; 0 when unrated or the modifier is unset.
    async fn modifier(&self, conn: &mut PgConnection, party: Uuid) -> Result<Decimal, RepoError>;
}
```

- **utoipa schemas:** `RatingView` (level, cbi_label, score, status, floors, completeness, controls row, degraded and
  reasons, `data_as_of`, `model_version`), `RatingExplanationView` (drivers sorted by points with reasons and refresh
  dates, missing-input share, floors and triggers in words, override history), `RatingHistoryView` (rows with model
  version and trigger), `OverrideView` (with expiry and reason), `FeedRowView` (`seq`, `cif`, `cbi_label`, `effective_at`),
  `DivergenceRowView` (`cbs_cdd_level`, `expected_cdd`, `status`: `agree`, `diverge`, `not_populated`), `DistributionView`
  (counts per level and per CBI label, sanity-band status), `RatingModelView`, `RatingModelDraft`, `ShadowRunView`,
  `ShadowItemView`, `KycReviewView`, `KycReviewWorkspace`, `ClaimBody`, `ReviewPatch`, `QueryCreate`, `QueryPatch`,
  `EvidenceView`, `CompleteBody`, `PepEntryView`, `NewPepEntry`, `PepPatch`, `PepRelativeView`, `RelativeBody`,
  `OwnershipView`, `OwnershipInputSet`, `ExitEntryView`, `NewExitEntry`. Every response that carries CBS-derived facts
  has `data_as_of` (C-29). `GET /kyc/reviews/{id}/evidence/...` documents `application/octet-stream` and
  `X-Content-SHA256`.
- **Route table** (`| Method | Path | Scope | Description (cedar action) |`; SU and CR markers as API.md section 3; every
  read of a party-linked record requires `X-Access-Purpose` because the rating, the ownership and the PEP data are
  restricted tier (DR-46). Rows marked "added" were not in API.md 3.10 and are added there by this phase:

| Method | Path | Scope | Description (cedar action) |
|-|-|-|-|
| GET | `/api/v1/parties/{id}/rating` | `kyc.work` | Current rating: CBI label, level, score, floors, completeness, controls, degraded reasons, `data_as_of`, model version; purpose required (`rating.read`) |
| GET | `/api/v1/parties/{id}/rating/history` | `kyc.work` | Rating history with model versions and trigger codes, cursor (`rating.read`) |
| GET | `/api/v1/parties/{id}/rating/explanation` | `kyc.work` | Explanation panel data: drivers with points, reasons and refresh dates, gaps, floors and triggers in words, overrides (`rating.read`) |
| POST | `/api/v1/parties/{id}/overrides` | `kyc.work` | Upward: active at once (201); downward: a change request of route `kyc_override_down`, 202 with `Location`; **SU** (`decision`); `Idempotency-Key` (`override.propose`) |
| GET | `/api/v1/overrides` | `kyc.work` | Overrides with expiry; filters `party_id`, `status`, `direction`, `expires_before`; cursor (`override.read`) |
| GET | `/api/v1/rating/feed` | `kyc.work` | One-way feed of CBI-label changes, cursor on `seq`; never a suspicion-tier party (`rating.feed.read`) |
| GET | `/api/v1/rating/divergence` | `kyc.work` | Divergence of the computed CDD level against `canon.customer.cdd_level`; "not populated" is never a divergence (`rating.divergence.read`) |
| GET | `/api/v1/rating/distribution` | `dashboard.read` | Distribution by level and label, sanity-band status, escapes (`rating.distribution.read`) |
| GET | `/api/v1/kyc/reviews` | `kyc.work` | Review queue: periodic, event, light yearly, light, EDD; filters `party_id`, `kind`, `status`, `due_before`, `assignee`; cursor (`kyc_review.read`) |
| GET | `/api/v1/kyc/reviews/{id}` | `kyc.work` | Workspace: checklist, new and changed facts since the last review, queries, evidence, declared figures; purpose required (`kyc_review.read`) |
| POST | `/api/v1/kyc/reviews/{id}/claim` | `kyc.work` | Claim an open review; `If-Match`; `E_CLAIMED` when another person holds it (`kyc_review.claim`) |
| PATCH | `/api/v1/kyc/reviews/{id}` | `kyc.work` | Save checklist progress, draft declared figures and the trigger note; `If-Match` (`kyc_review.work`) |
| POST | `/api/v1/kyc/reviews/{id}/queries` | `kyc.work` | Raise a query on the review (added) (`kyc_review.work`) |
| PATCH | `/api/v1/kyc/reviews/{id}/queries/{qid}` | `kyc.work` | Answer a query or close it (added); `If-Match` on the review (`kyc_review.work`) |
| POST | `/api/v1/kyc/reviews/{id}/evidence` | `kyc.work` | Upload evidence (multipart), staged and scanned by phase 02, owner kind `kyc_review` (added) (`kyc_review.work`) |
| GET | `/api/v1/kyc/reviews/{id}/evidence/{evidence_id}` | `kyc.work` | Stream one evidence object of the review, decrypted and hash-checked on serve; audited (added) (`kyc_review.read`) |
| POST | `/api/v1/kyc/reviews/{id}/complete` | `kyc.work` | Complete: refused with `E_OPEN_QUERIES` while a query is open; writes the declared profile and the outcome; **SU** (`decision`); `If-Match` (`kyc_review.complete`) |
| POST | `/api/v1/kyc/reviews/{id}/reassign` | `kyc.work` | Reassign an open review to another KYC user (added) (`kyc_review.reassign`) |
| GET | `/api/v1/pep/register` | `kyc.work` | PEP register: status, tier words, persistence, relatives count; filters `status`, `tier`, `source`; cursor; never the alert id (`pep.read`) |
| POST | `/api/v1/pep/register` | `kyc.work` | Add a candidate or propose a confirmed entry (CR `pep_entry` for a confirmed one); `Idempotency-Key` (`pep.write`) |
| PATCH | `/api/v1/pep/register/{id}` | `kyc.work` | Change tier, office dates, persistence or assessment; a tier change is CR `pep_entry`; `If-Match` (`pep.write`) |
| POST | `/api/v1/pep/register/{id}/relatives` | `kyc.work` | Confirm a suggested relative or add a relative or close associate (human confirmation; `pep.relative.confirm`) |
| GET | `/api/v1/parties/{id}/ownership` | `kyc.work` | Ownership chain, effective percentages, unknown nodes, loops; purpose required (`ownership.read`) |
| PUT | `/api/v1/parties/{id}/ownership/inputs` | `kyc.work` | Replace the owner set entered by the bank where the CBS holds no percentage (added); appended, not versioned: `Idempotency-Key` and no `If-Match`; the result is recomputed (`ownership.input.write`) |
| GET | `/api/v1/exit-register` | `kyc.work` | Exit register, cursor; suspicion tier, masked (`exit.read`) |
| POST | `/api/v1/exit-register` | `kyc.work` | Record an exit from a completed `exit_decision` instance; **SU** (`decision`) (`exit.record`) |
| GET | `/api/v1/rating-models` | `config.author`, `config.approve` | Model versions with status, effective date, change request (`rating_model.read`) |
| POST | `/api/v1/rating-models/versions` | `config.author` | Open a draft from the shipped or the live document; `Idempotency-Key` (`rating_model.propose`) |
| PUT | `/api/v1/rating-models/versions/{v}` | `config.author` | Replace a draft document; `If-Match`; refused once the version is not draft (`rating_model.propose`) |
| POST | `/api/v1/rating-models/versions/{v}/validate` | `config.author` | Validate against DSL.md Part C codes; stores the result; zero errors required to leave draft (`rating_model.validate`) |
| POST | `/api/v1/rating-models/versions/{v}/shadow` | `config.author` | Start a shadow run on production data (job `rating_shadow_run`); 202 with `Location` (`rating_model.shadow.run`) |
| GET | `/api/v1/rating-models/versions/{v}/shadow` | `config.author`, `config.approve` | Latest shadow run: disagreements (suspicion-tier parties masked), tier migration, sensitivity, volume effect (`rating_model.shadow.read`) |

- **Router:** `pub fn router() -> Router<AppState>` in each of `rating`, `kyc`, `pep`, `ownership`, `exit`,
  `rating_models`; `kyc` and `pep` share the `kyc.work` gate and the masking layer.

---

## 3. Business Logic

Every mutation below names its audit event in 3.11. Every party-linked read names its masking in 3.6. Numbers that a
bank may want different next month are settings (migration 0053) or model parameters (the Part C document); none is
a constant in code except the closed enums of section 2.

### 3.1 The rating model: install, change and lifecycle (`RatingModelService`, `rating seed`; K-06, K-28, DSL.md C.9)

**Install (`akr-ctl rating seed [--dry-run]`, zone Z5, run after `db migrate` and `db harden`):**
1. Read the release manifest; compare the SHA-256 of `backend/packs/rating/rm-iq.v1.0.0.json` with the manifest entry
   and verify the manifest signature (phase 00 release signing). Mismatch: exit 1, nothing written.
2. Validate the document against `rating-model.v1.json` and the Part C rules of section 3.1.1. Any error: exit 1.
3. One transaction as `akr_app`: if no `live` version of `RM-IQ` exists, insert the version with `status = live`,
   `effective_from = now()`, `change_request_id = NULL`, the validation result and the document hash; audit
   `rating_model.seeded`. If a live version exists, the seed writes nothing and exits 0 with `already_seeded`. A seed
   never replaces a live version; a later version goes through 3.1.2.
4. Enqueue `rating_nightly {reason: "model"}` (coalesce key `rating_nightly`).
5. `akr-ctl rating verify-golden` runs `RatingEngine::score` over `rm-iq.golden.v1.0.0.json` (research 26 s.14e
   examples, each with every one of the 27 inputs supplied as fixture values, and the level each must produce) and
   the consistency fixtures of section 6. Any mismatch: exit 2.

**3.1.1 Part C validation (`RatingModelDoc::validate`, pure; DSL.md s.7 Part C codes, returned in `errors[]` of
`E_VALIDATION`):** `E_WEIGHTS_SUM` (block weights sum to 100; factor weights in each block sum to 100); `E_PEAK_TERM_RANGE`
(`peak_term` in [0, 0.5)); `E_LEVEL_BANDS` (five bands, half-open, covering 0 to 100 without gap or overlap, the top
band including 100); `E_CBI_MAPPING` (labels monotonic: levels 1-2 low, 3 medium, 4-5 high); `E_UNKNOWN_IS_LOW`
(`unknown_score` at least the lowest score of the first level mapped to medium; 60 satisfies it); `E_FLOOR_LEVEL` (1 to 5);
`E_DECAY_MISSING` (FL-07, FL-08, FL-09 and any bank floor on a dated fact need `decay`); `E_TRIGGER_INCOMPLETE`;
`E_CONTROLS_MISSING` (a controls row for every level); `E_REVIEW_INTERVAL_LIMIT` (no classification review above 24
months under `iraq_cbi`); `E_OVERRIDE_LIMITS` (downward: one level, twelve months, maker-checker); `E_PROTECTED_ATTRIBUTE`
(a factor input that is a protected attribute: names, tribe, religion, sect, region of origin, gender; nationality and
residence are allowed for C1); `E_DANGLING_REF` (a factor input that names no canonical attribute or no registered
`pep.*`, `ownership.*`, `country_table.*`, `feat.*` or `rating.*` input); `E_UNKNOWN_ATTRIBUTE`; `E_SCHEMA`. Warnings:
`W_SHADOW_NOT_RUN` and `W_SANITY_BAND`. Moving past draft requires zero errors (DSL.md s.7).

**3.1.2 Change path (`RatingModelService`; CR kind `rating_model`, route `cr_material_validated`, DSL.md C.9 and E.4):**
1. `POST /rating-models/versions` opens a draft: copies the live document (or a given one) into a new row with
   `status = draft`, `version` = the next semver the maker names, `row_version = 1`; audit `rating_model.proposed`.
2. `PUT /rating-models/versions/{v}` replaces a draft document (`If-Match` on `row_version`, 409 `E_VERSION_CONFLICT`
   with the current version in the body); refused with 409 `E_NOT_ALLOWED_IN_STATE` once the version is not draft.
   Audit `rating_model.proposed` again with the new document hash.
3. `POST .../validate` runs 3.1.1, stores `validation`, returns 200 with the lists. Errors block only the transition.
4. `POST .../shadow` (202): requires zero errors; inserts `rating_shadow_runs` with `status = queued` (partial unique
   index: one active run per version), enqueues `rating_shadow_run {shadow_run_id}`, and moves the version
   `draft -> shadow` in the same transaction. Audit `rating_model.shadow_started`.
5. Job `rating_shadow_run` (section 4.1): for every party with a live rating, the engine runs once with the shadow
   version and the same snapshot as the live rating of that party; a disagreement (level differs, or the shadow is a
   control failure) writes one `rating_shadow_items` row with the drivers that moved. It counts `tier_migration` keyed
   `live>shadow` per level pair; `sensitivity` counts the parties whose level changes when each block weight moves
   by one point up and down and when `peak_term` moves by 0.05. `volume_effect` is `increase` when the number of
   parties in the high CBI label rises, `decrease` when it falls, `neutral` when it does not move, and `unknown` when
   the run did not complete. Audit `rating_model.shadow_completed`.
6. `POST /change-requests` (phase 04, generic create; `object_kind = rating_model`, `object_key = rating_model:RM-IQ`,
   `base_version` = the row version of the live version, `proposed` = `{version_id}`, reason code of family `change`,
   for example `CHG_RISK_ASSESSMENT`). Approval: MRO (validator, `approve`), then parallel MLRO and CHK; CH joins when
   `volume_effect = decrease` (phase 04 `cr_material`). `unknown` is never approvable (DR-96).
7. Activation (`RatingModelTarget::activate`, in the approval transaction): the shadow version `shadow -> live` with
   `effective_from` from the change request and `change_request_id`; the previous live version `live -> retired`
   with `retired_at` (the guard of migration 0052 allows both); enqueue `rating_nightly {reason: "model"}`; audit
   `rating_model.activated`. Rejection or withdrawal: the version moves `shadow -> retired` (audit
   `rating_model.retired`); a retired version is never reopened, a new draft is opened instead.

**3.1.3 Lifecycle table:** `draft -> shadow` (shadow run started), `shadow -> live` (activation), `live -> retired`
(replaced), `shadow -> retired` (rejected). Content is frozen from `shadow` on. One live version per model id.

### 3.2 The rating engine (`RatingEngine` pure; `RatingService::rescore`; K-01, K-26; DSL.md C.1 to C.8)

**The factor catalogue** (research 26 s.14d, the 27 factors of the shipped model; each factor's `input` is its
DSL `input` expression. "Wired" means the input exists in this phase's snapshot. "Pending" factors are excluded
from the block average and marked `NotWired` until a later model version wires them (a material change under 3.1.2);
they are listed once on the model version and in the explanation as "not yet in use", and they do not make a party's
rating degraded, because degraded describes the party's data, not the product's coverage. "Withheld" factors take
suspicion-tier data and are never scored into the stored rating (DR-103, section 3.6):

| Block (weight) | Factor (weight in block) | Input (snapshot source) | Status |
|-|-|-|-|
| A customer and structure (25) | A1 customer type (30) | `canon.party.legal_form`, `type` (phase 10) | wired |
| | A2 entity age (5) | `canon.party.established_on` (entities only) | wired |
| | A3 ownership transparency (25) | `kyc.ownership_results` (layers, verification; 3.7) | wired after the first ownership run |
| | A4 identity assurance (10) | `canon.party_identifier`, `canon.kyc_document.expires_on` (sentinel or impossible date: missing, degraded) | wired |
| | A5 reputation (20) | adverse media (the media feed, later phase); the STR and exit-register parts are suspicion-tier data | pending (media); withheld (STR, exit; DR-103) |
| | A6 registration and disclosure (10) | `canon.party.registration_no_enc` presence, A3 disclosure | wired |
| B business, occupation and wealth (15) | B1 industry or occupation class (45) | `canon.party.occupation_isic` through the model's class table | wired |
| | B2 public-sector exposure (15) | `canon.party.occupation_isic`, declared procurement role (review checklist) | wired after a review |
| | B3 source of wealth and funds (25) | completed review checklist item `source_of_wealth` | wired after a review; unknown before |
| | B4 expected activity profile (15) | `kyc.declared_profiles` (declared, partial, none) | wired |
| C geography and foreign nexus (20) | C1 nationality and residence (35) | `canon.party.nationality`, `residence_country` | wired |
| | C2 registration and ownership countries (20) | registration country (mapping to confirm, Open doubts), `kyc.ownership_results` countries | pending the mapping |
| | C3 linked-country risk (35) | highest `CountryView::effective_level` over nationality, residence and beneficial-owner residence (registration joins when it is mapped, Open doubts 10) | wired |
| | C4 branch location (10) | `canon.branch` border attribute (mapping to confirm) | pending the mapping |
| D products, services and value (15) | D1 highest-risk product (50) | `canon.account` joined to `canon.product` (phase 10) | wired |
| | D2 number and complexity of products (15) | count of current accounts of the customer records | wired |
| | D3 expected monthly value (20) | `kyc.declared_profiles.expected_credit_1m` (phase 13 features after phase 13 is live) | wired (declared) |
| | D4 cash and third-party features (15) | `canon.cash_event` and third-party deposits (phase 10) | wired |
| E channel (5) | E1 onboarding channel (60) | `canon.customer.channel_id` through `canon.channel` | wired |
| | E2 intermediaries and powers of attorney (40) | `canon.account_party_role` with role `attorney` or signing rule | wired |
| F behaviour and experience (20, dynamic) | F1 declared versus actual activity (30) | ratio of phase 13 actuals to the declared figure; the stored deviation is never read (phase 13 section 3.12) | wired (ratio, not deviation) |
| | F2 observed corridors and counterparties (20) | phase 13 corridor feature | pending (phase 13 feature kind, later) |
| | F3 alert and case history (20) | alerts and cases (phases 17 and 18) | withheld (DR-103) |
| | F4 profile change velocity (10) | key changes in 90 days: rating level changes, declared-profile versions, party facts | wired |
| | F5 network risk (10) | network snapshots and STR-linked parties (phase 19) | withheld (DR-103) |
| | F6 responsiveness (5) | RFI desk outcomes (phase 19) | pending |
| | F7 authority requests (5) | authority requests (phase 19) | withheld (DR-103) |

Block weights 25, 15, 20, 15, 5, 20 sum to 100; factor weights within each block sum to 100 (checked by 3.1.1).

**The computation contract (one party, one as-of date, one transaction):**
1. Serialise: take `pg_advisory_xact_lock` on the party (key from the first 8 bytes of its UUID) so two triggers for
   one party never interleave. Load the live model (`RatingModelRepository::live(RM-IQ)`). No live model: the job
   completes with `skipped: no_live_model` and writes nothing.
2. Snapshot (`PartyFactsSource::snapshot(party, as_of)`, the bank business date of the last closed day, read only):
   the current `canon.party` and `canon.customer` rows of the party, identifiers and KYC documents, current accounts
   and their products, current ownership result, `kyc.party_pep_state`, the open override, the last completed review
   of each kind, the open declared profile, `CountryView` levels, the phase 11 freshness snapshot of each source
   (`stale`, `dark`, `halted`), the phase 11 blind periods and the phase 12 excluded units of the party's branches.
   Protected attributes are never selected. The snapshot records `data_as_of` (C-29).
3. Factor states (per factor, DSL.md C.2): a value older than `freshness` (months) is `Stale`; a missing value is
   `Missing`; a value whose input is not wired is `NotWired`; a withheld factor (DR-103) is `Withheld`; a factor that does not apply to the party type
   (`applies_to`) is `NotApplicable`. A `Stale` or `Missing` factor scores `unknown_score` (60) and counts as unknown.
   A stale factor also opens a refresh review once (trigger `TR-23`, section 3.4). A sentinel or impossible date is
   `Missing`, flagged degraded, never valid and never expired (MODULES M10 edge case).
4. Factor scores from the anchors: `categorical` (value map), `bands` (thresholds), `country_table` (low 10, medium
   40, high 70, very high 100), `expr` (an Expr returning 0 to 100, DSL.md shared types). Scores are Decimal, scale 2.
5. Block score = `sum(w_f x s_f) / sum(w_f)` over the factors of the block that are `Known`, `Unknown` or `Stale`
   (`NotApplicable`, `NotWired` and `Withheld` are left out). A block with no applicable factor is left out of step 6 and the
   other block weights renormalise (Open doubts: DSL.md does not say).
6. Inherent score (DSL.md C.8 step 4, normalised): `(1 - p) x sum(w_b/100 x s_b) + p x max(s_b)` over the blocks
   of step 5, with `p = peak_term`. The DSL writes `sum(block weight x block score)`; the research worked example
   (weighted 7.9, peak 15.0, inherent 10.4 with p = 0.35) only holds with the `/100` normalisation. Rounded half-even to
   two decimals once, at the end.
7. Score level: the band of the inherent score that contains it (half-open bands; 100 in the top band).
8. Floors (DSL.md C.3): evaluate every floor's `when` over the snapshot. A floor fires when its `when` is true and
   its `decay` has not passed (the decay runs from the fact's date: office end for a PEP, the decay months of the
   tier, section 3.8). A floor whose decay has passed stops applying and opens an event review (K-30: never an
   automatic downgrade). `floor_level` = the highest level of the firing floors. Level = max(score level, floor_level).
9. Completeness cap (DSL.md C.4): the mandatory factors of the party type (`completeness_cap.mandatory`) that are
   unknown are counted. None: no cap. One: level = max(level, 3) and `cap_applied = min_level_3`. Two or more:
   `rating_status = control_failure`, no level, no CBI label, `cap_applied = control_failure`, and an event review
   with `escalate_review` (K-04: a control failure is shown as a control failure, never as Low).
10. Override (DSL.md C.6): the open override applies when `status = active` and `expires_at > now`. Upward:
    `level = max(level, to_level)`. Downward: applies only while `to_level < computed level`; then
    `level = max(to_level, floor_level, cap_min)`, so no override crosses a floor or the cap. An expired or superseded
    override is ignored (the sweep of 3.5 ends it). An override whose `to_level` no longer makes sense (the computed
    level moved) is ignored with the degraded reason `override_void`.
11. Stickiness (DSL.md C.1, K-30): against the previous rated state of the party (`rating_current` when
    `rating_status = rated`). A rise applies at once. A fall applies only when a review completed after the previous
    rating (`kyc_reviews.completed_at > prior.computed_at`) or when an approved downward override is active; otherwise
    the level stays at the previous level (`stickiness = held_fall`), which is never below a floor that still fires.
12. CBI label from `levels[].cbi`; the controls row of the level (`cdd`, `approver`, `monitoring_profile`,
    `classification_review`, `kyc_refresh`, `light_check`, `edd_playbook`) is returned with the rating from the model
    version, not stored again.
13. Degraded (MODULES M10 edge cases): `degraded = true` with reasons when any of: a mandatory or applicable factor is
    `Missing` or `Stale`, or the override is `override_void`; the country of a C factor is unknown (scores 60, degraded, as
    the MODULES edge case says); a source of the party is `stale`, `dark` or `halted`; a blind period or an excluded
    unit touches the party. `missing_share` = unknown applicable factor weight / applicable factor weight.
14. `input_hash` = SHA-256 of the canonical encoding (phase 02 encoder, never `serde_json`) of: the live version id,
    the document hash, each factor's state and value, the floor facts, the open override id and status, the
    previous rated state (level, status, computed day) and the completion day of the last review. `as_of` itself is
    not hashed: a day with unchanged states produces the same hash.
15. Persist, only when something changed. Compare with `rating_current`: when `rating_status`, `level`, `cbi_level`,
    `input_hash`, `override_id` and `model_version_id` are all equal, stop (no row, no audit). Otherwise, in the same
    transaction: insert the `kyc.ratings` row (`effective_on` = the as-of day, `computed_at` = now), upsert
    `kyc.rating_current`; when the CBI label changed or the first rated state appears, insert a `kyc.rating_feed` row
    (not for a suspicion-tier party, 3.6); when the level changed or the status became `control_failure`, open the
    review of section 3.4 (`due_basis = level_change`); audit `rating.changed {party_id, rating_id, level_from,
    level_to, cbi_from, cbi_to, trigger_code, model_version_id}` (or `rating.control_failure`); outbox
    `rating.changed {party_id, rating_id}`. The audit payload carries ids and labels only.
16. The `trigger_code` of the row is the DSL trigger id (`TR-nn`) when the trigger is a public one (section 3.3), or
    the neutral `TRG_*` code for a suspicion-linked trigger (3.6). The reason of the job is never stored.

### 3.3 Re-score triggers and fan-out (`RatingFanout`; K-42, TR-10, DSL.md C.5)

Consumers are registered in the phase 00 topic registry. Each enqueues work and writes nothing itself.

| Source | Consumer and job | Effect | Wired in this phase |
|-|-|-|-|
| `canon.touched {batch_id, source_id, source_table, entity_kinds, business_date}` (phase 11) | `rating_touch_fanout {batch_id}` (coalesce `rating_fanout:<batch_id>`) reads the batch's changed rows of `party`, `customer`, `party_relationship`, `account`, `party_identifier`, `kyc_document`, and enqueues `rating_compute {party_id, reason: "canon"}` (coalesce `rating:<party_id>`, run after `rating.trigger_debounce_s`) and `ownership_compute` for owned parties of a changed relationship | Re-score on canonical change (TR-01 family, `party.changed`, `relationship.changed`) | yes |
| `ref.country_level_changed {iso2, from, to, cause}` (phase 09) | `rating_country_fanout {iso2, cause, effective_at}` (coalesce `rating_country:<iso2>`, concurrency 1): pages of `rating.country_fanout_page` parties whose nationality or residence country, or a beneficial owner's residence, is `iso2` (the registration country joins when a canonical column holds it, Open doubts 10); enqueues `rating_compute {reason: "country"}` | TR-10: every affected customer is re-scored the same day and listed with the effective date (K-42) | yes |
| `pep.changed {party_id, pep_id, change}` (own, 3.8) | `rating_compute {reason: "pep"}` | floors FL-03 and the PEP floors follow the register | yes |
| `kyc_review.completed {review_id, party_id, outcome}` (own, 3.4) | `rating_compute {reason: "review"}` | completion lets a fall apply (3.2 step 11) | yes |
| `override.changed {override_id, party_id, status}` (own, 3.5) | `rating_compute {reason: "override"}` | the override applies or ends | yes |
| `dq.stale {source_id, ...}` (phase 11) | no immediate job: the source's parties are degraded on the next nightly run | freshness reasons in 3.2 step 13 | yes (nightly) |
| `rating_nightly {reason}` (schedule `schedule.rating_nightly`, daily 01:30; also enqueued by a model activation) | pages of `rating.page_size` parties in `party_id` order; enqueues `rating_compute {reason: "nightly" or "model"}` for every party; unchanged parties write nothing (3.2 step 15) | the nightly contract of DSL.md C.8 | yes |
| `screening.hit`, `case.decision`, `authority_request.recorded`, `internal_report.recorded`, `visit.recorded`, `outreach.failed`, `graph.link`, `staff_link.recorded`, `txn.new_corridor`, `profile.breach`, `media.recorded`, `account.opened`, `account.reactivated`, `islamic.signal` (DSL.md C.5) | registered by the phase that produces each topic (14 to 19, 26); not wired here | the TR-15 and suspicion-linked triggers map to `TRG_REVIEW_REQUESTED` (3.6) | no |

Rules: a re-score never runs inside the consumer's transaction (the consumer enqueues); two triggers for one party
collapse into one job (coalesce key); the job re-reads the party (phase 00 rule), so a trigger for a party that has
gone (party status `closed` and no open review) is a no-op.

### 3.4 KYC reviews: clocks, queue, workspace and completion (`KycReviewService`, `ReviewClock` pure; K-07 to K-09, MODULES M10)

**Clocks (`ReviewClock`, pure; DSL.md C.7 controls row of the level, the two clocks per customer and the light yearly
check for all):**
- Clock 1, `kyc_refresh`: next refresh = last completed `periodic` review (or onboarding date) + `controls[level].kyc_refresh`.
- Clock 2, `classification_clock`: next classification review = last completed classification + `controls[level].classification_review` (at most 24 months under `iraq_cbi`).
- Periodic due = the earlier of clock 1 and clock 2; `due_basis` names the clock that gave the date.
- `light` (kind `light`): `controls[level].light_check` months after the last light review, where the level defines one.
- `light_yearly` (kind `light_yearly`): `kyc.light_yearly_months` (12) after the last light yearly review, for every customer whatever the level.
- `edd` (kind `edd`): for a level whose `controls[level].cdd` is `edd`, due at the `kyc_refresh` interval after the last completed EDD; the checklist is `edd_entity_v1` or `edd_person_v1` by party type (`controls[level].edd_playbook`).
- A date that is empty, a sentinel (before 1990-01-01 or more than 100 years after today) or impossible (after today for a completed review) is `date_unknown`: the review is opened with `due_at = NULL`, `date_unknown = true`, `due_basis = date_unknown`, and is escalated at once. It is never read as on time.

**Opening (job `kyc_review_sweep`, daily, section 4.1):**
1. Page the current customers (`canon.customer` with `recorded_to IS NULL`, ordered by party) and compute the due dates
   of 3.4 for each kind.
2. Open a review when the due date is within `kyc.review_open_ahead_days` (30) of today, or is past, and no open review
   of that kind exists: `KycReviewRepository::open_or_join` (the partial unique index `uq_kyc_reviews_open`). Insert the
   row with the checklist template, `level_at_open`, `rating_id`; schedule the timer (step 3).
3. Schedule timer `kyc_review_clock {review_id}` at `due_at` (phase 00 `TimerService::schedule`, owner `kyc_review`,
   business-day arithmetic through `BankCalendarClock` where the due basis is a working-day interval). Its handler
   `timer_kyc_review_clock` re-reads the review: when it is still open and past due, it sets `escalated_at` and
   audits `kyc_review.overdue`; a closed or cancelled review does nothing (phase 00 rule).
4. Overrides whose `expires_at` has passed are ended in the same sweep (`status = ended`, `end_reason = expired`) and
   `override.changed` is written; the engine re-scores them (3.5).
5. Event reviews are opened by the engine on a level change (`due_basis = level_change`, due `review_within` business
   days of the model's trigger, default five), by a stale factor (`TR-23`, due five business days), by a lapsed floor
   decay (3.2 step 8, trigger `TRG_FLOOR_DECAY`), and by a PEP persistence end (3.8). An open event review of the same
   party absorbs a new event (its `trigger_code` is updated; the review does not multiply).

**Working a review (K2 workspace):**
1. Claim (`POST .../claim`, `If-Match`): status `open -> in_progress`, `assignee_id` = caller, `claimed_at`. Another
   assignee: 409 `E_CLAIMED`. The relationship-conflict check (3.9) refuses a claim by the customer's own relationship
   owner (masked 404 on the customer, 403 `E_FORBIDDEN` on a non-masked read).
2. Save (`PATCH`, `If-Match`): checklist values, item dates, the draft declared figures and a note; status unchanged
   (`in_progress` stays). Only the assignee may save.
3. Queries (`POST .../queries`, `PATCH .../queries/{qid}`): a query sets the review to `awaiting_answer`; an answer
   sets the query to `answered`; when no query is open the review returns to `in_progress`. The raiser or M closes a
   query (`closed`). `E_OPEN_QUERIES` is the completion refusal below.
4. Evidence (`POST .../evidence`, multipart): `EvidenceStore::stage` (phase 02) with owner kind `kyc_review`, owner id
   = the review, retention class R8, then one `kyc.review_documents` row (`kind` from the form, `item_key`) and audit
   `kyc_review.evidence_attached {review_id, evidence_id, kind}`. The object is "being checked" until its antivirus
   scan is clean (phase 02); a rejected object refuses the link with 422 `E_UPLOAD_REJECTED`.
5. Evidence open (`GET .../evidence/{evidence_id}`): `EvidenceStore::open` (decrypt, segment authentication and hash
   on serve, DR-44); refused with 422 `E_NOT_ALLOWED_IN_STATE` when the object is not clean. Audited as an access event
   (phase 02 `AccessLog`, tier of the party).
6. Reassign (`POST .../reassign`, M only): sets `assignee_id` to another active KYC user; audit `kyc_review.reassigned`.
7. Complete (`POST .../complete`, SU `decision`, `If-Match`, `Idempotency-Key`), one transaction:
   1. The conflict check (3.9) and the masking of 3.6 run first.
   2. Any `review_queries` row with status `open` or `answered` but not `closed`: 422 `E_OPEN_QUERIES`.
   3. Any evidence object of the review not clean: 409 `E_NOT_ALLOWED_IN_STATE`.
   4. The checklist of the template must hold a value for every required item: else 422 `E_VALIDATION` with
      `errors[].code = E_SCHEMA` and the item path.
   5. Declared figures in the body: close the open `kyc.declared_profiles` row of each (customer, currency) with
      `valid_to = declared_on` and insert the new row (`valid_from = declared_on`). Audit `declared_profile.recorded`.
   6. Outcome: `refresh_completed` for periodic and light kinds; `no_change` when the level and every checklist value
      are unchanged; `edd_completed` for EDD; `escalated` when M escalates; `exit_candidate` opens the workflow instance
      `exit_decision` (DSL.md E.4, shipped by this phase, section 4.2) and the review completes; the exit runs on its
      own instance (step 8);
      `level_change_proposed` records that the reviewer proposes an override, which the reviewer then submits through
      3.5 (the review does not change the rating itself).
   7. Set `status = completed`, `completed_by`, `completed_at`, `outcome`; the guard of migration 0052 makes the row
      immutable from here. An EDD review of level 4 or 5 (`controls[level].cdd = edd`) does not complete here: this sub-step
      opens the workflow `kyc_edd_signoff` (DSL.md E.4: approve by the controls matrix approver, not the reviewer; SU
      `approval`) and answers 202 with its id; the terminal effect `record_decision` then runs sub-steps 7 to 9 with the
      approver as completer (`E_SELF_APPROVAL` for the reviewer).
   8. Audit `kyc_review.completed {review_id, kind, outcome}`; outbox `kyc_review.completed {review_id, party_id, outcome}`.
   9. Enqueue `rating_compute {reason: "review"}`.
8. Exit (`exit_decision` instance, then `POST /exit-register`): the terminal effect `record_decision` stores the approved
   outcome on the instance only. M then records the entry (`POST /exit-register`, SU `decision`, `Idempotency-Key`):
   the instance must be completed with an `approve` outcome (else 409 `E_NOT_ALLOWED_IN_STATE`), the reason code must
   be an active code of family `decision` (`DEC_`), and the unique `workflow_instance_id` allows one entry per decision
   (a second attempt is 409 `E_NOT_ALLOWED_IN_STATE`). Insert `kyc.exit_register` (status `recorded`) and audit
   `exit.recorded {exit_id, review_id, reason_code}` with no party name. The register is the single writer path.

**Queue (K2 list):** open and in-progress reviews ordered by `due_at` (NULLs first, as overdue), then by `kind`;
`date_unknown` reviews are shown first with the words "Date not known". Filters `kind`, `status`, `due_before`,
`assignee`. The list is a masked-kind list (3.6): a suspicion-tier party's review is filtered out in SQL, and the count
excludes it.

### 3.5 Overrides, the expiry sweep and the change-request targets (`OverrideService`; K-24, DSL.md C.6, E.4)

**Upward (`POST /parties/{id}/overrides`, `direction` derived from `to_level > computed_level`; SU `decision`):**
1. Cedar `override.propose` (KYC, M); scope `kyc.work`; the conflict check (3.9) passes.
2. Validate: `to_level` in 1 to 5 and above the computed level; `reason_code` active in `ref.reason_codes` family
   `override` (phase 09; the `OVR_` prefix); `reason_text` 1 to 500 characters; `expires_at` after now and within
   twelve months (the DSL bounds the downward case; the same bound applies upward, DR-104).
   Failure: 422 `E_VALIDATION` with the path.
3. No open override for the party (`uq_overrides_open`): 409 `E_NOT_ALLOWED_IN_STATE`.
4. Insert `kyc.overrides` with `status = active`, `effective_from = now`, `proposed_by = caller`; the engine applies it.
   Audit `override.applied {override_id, direction: "up", from_level, to_level, reason_code}`; outbox `override.changed`;
   enqueue `rating_compute {reason: "override"}`. Answer 201.

**Downward (same route; compliance only; one level; twelve months; evidence; four eyes):**
1. Cedar `override.propose` as above; the conflict check refuses a proposer who is the customer's relationship owner
   (K-17; DSL.md C.6 "exclude relationship owner"). Refusal: 403 `E_FORBIDDEN`, or the masked 404 when the party is
   suspicion tier.
2. `to_level = computed_level - 1` exactly (`E_OVERRIDE_LIMITS` in `errors[]` otherwise); `evidence_object_id` must be a
   clean evidence object of the caller's staging (`E_VALIDATION` otherwise); `expires_at` within twelve months.
3. The floors and the cap: when `to_level < floor_level` or `to_level < cap_min` the request is refused with 422
   `E_BELOW_FLOOR` (the override may not cross them, K-07, K-24). The check runs again at approval.
4. Create the change request (phase 04 `ChangeRequestService::create`, `object_kind = rating_override`,
   `object_key = override:<party_id>`, reason code of family `change` (for example `CHG_RISK_ASSESSMENT`), class
   `material`, route `kyc_override_down`); insert `kyc.overrides` with `status = proposed` and the change request id.
   Answer 202 with `Location` to the change request. Audit `override.proposed {override_id, change_request_id}`.
5. Route `kyc_override_down` (shipped by this phase, section 4.2): one step, `approve`, by role CHK or M, not the
   proposer (`E_SELF_APPROVAL`), not the relationship owner (3.9), then `record_decision`.
6. Activation (`OverrideTarget::activate`, in the approval transaction): `status = active`, `approved_by` = the
   approver, `effective_from` = now, `expires_at` as proposed (the CHECK of migration 0050 bounds a downward override to twelve months); audit
   `override.applied {direction: "down", ...}`; outbox `override.changed`; enqueue `rating_compute {reason: "override"}`.
   `on_closed` with `Rejected` or `Withdrawn`: `status = rejected`, `end_reason = rejected`.
7. `prepare` and `check_before_decision` re-run steps 2 and 3, so a floor that started to fire after the proposal
   refuses the decision with `E_BELOW_FLOOR`.

**Expiry (sweep of 3.4 step 4):** an `active` override with `expires_at <= now` is ended (`status = ended`,
`end_reason = expired`, `ended_at = now`); the rating returns to the computed level at once, not to an older override
(DSL.md C.6). An open review is not closed by an expiry (MODULES M10 edge case). Audit `override.ended {override_id, reason: "expired"}`.

**No withdrawal route in this phase.** An active override ends at expiry, or by a superseding downward change; a
withdrawal route would be a new change target (Open doubts).

**Change-request targets (phase 04 `ChangeTargetRegistry`; DR-53; the trait of phase 04 section 2):**

| Kind | Base class | Route | Generic create | Emergency | Families | Activation (inside the approval transaction) |
|-|-|-|-|-|-|-|
| `rating_model` | material, validated | `cr_material_validated` | yes | no | change | version `shadow -> live`; previous live `-> retired`; enqueue `rating_nightly {reason: "model"}`; audit `rating_model.activated` |
| `pep_entry` | minor when the change raises a tier or confirms; material when it lowers a tier or ends the entry | `cr_minor` or `cr_material` (route by the class, through the route map) | no (dedicated routes) | no | change | `status = confirmed` with `tier`, `confirmed_by` (the final approver), `change_request_id`; or the new tier; or `status = ended`; enqueue `pep_relative_suggest {pep_id}` and `rating_compute {reason: "pep"}`; audit `pep.confirmed`, `pep.updated` or `pep.ended` |
| `rating_override` | material | `kyc_override_down` (added, section 4.2) | no | no | change | as step 6 of the downward route above |

Common rules. `prepare` runs the kind's validator (3.1.1 for a model; the rules of 3.8 for a PEP entry; the floor
check of 3.5 for an override). `check_before_decision` runs it again: a model whose draft changed, or an override whose
floor moved, refuses the decision with `E_VALIDATION` or `E_BELOW_FLOOR`. `volume_effect` for a model is 3.1.2 step
5; for a PEP entry it is `neutral` (no scenario reads the register in this phase; the rating effect is the floors, which
the rating measures itself); for an override it is `decrease` (the downward route) and an upward override is not a
change request. `revert`: none (no emergency route for these kinds). `on_closed` with `Rejected` or `Withdrawn`: a
rejected model version is retired; a rejected PEP entry keeps its candidate status; a rejected override is `rejected`.
Route selection is data (DR-54): the `change.route_map` rows for these kinds are added in the release data of this
phase (section 4.2).

### 3.6 Masking and tipping-off (phase 03 masking layer; phase 13 `customer` adapter; DR-46, DR-48)

**The `customer` kind becomes masked.** Phase 13 registered the `customer` resource adapter unmasked (restricted tier).
This phase extends that adapter, without re-registering it: the loaded attributes gain `tier` (from
`canon.party.sensitivity_tier` of the customer's party: `normal`, `restricted` or `suspicion`), and `ResourceKind::masked()`
returns true for `customer`. A suspicion-tier party answers 404 `E_NOT_FOUND` to every caller the permits of 3.12 do not
admit, with the same body and headers as a missing record (phase 03 section 3.6); a refusal writes its access event
(`deny`) and a decision row.

**Further masked kinds registered here (phase 03 `MASKED_KINDS`, each with its complete probe set):** `kyc_review`
(reviews of a suspicion-tier party), `override` (overrides of a suspicion-tier party), `pep_entry` (entries linked to a
suspicion-tier party). The exit register is not a kind of its own: its rows are filtered through the `customer` kind (below). Lists of these kinds filter in SQL by the
visibility predicate that equals the Cedar permit (tier, branch scope where it applies), with a test per kind that
pins the predicate to the policy over generated principals and rows. Counts, cursors and totals come from the filtered
query only.

**The probe set** (`api/tests/inference/kyc.rs`): get (one id of each kind, suspicion and normal, for a KYC user, an A1
user and a masked caller), list count (`/kyc/reviews`, `/overrides`, `/pep/register`, `/rating/divergence`,
`/rating/feed`), search (none in this phase: the probe asserts that each list rejects a `q` filter with 400
`E_BAD_REQUEST`), export (the feed export file, section 3.6 below, and the rating export rows), error body (byte-identical
404 for a missing and a masked id), notification text (none emitted in this phase: the probe asserts that no notification
names a party), SSE (none emitted in this phase: the probe asserts that no event carries a party id), audit viewer (the
audit rows of a suspicion-tier party's rating events name no tier, trigger cause or factor), and the latency comparison
(median of 200 masked refusals against 200 misses, under 5 ms apart). A kind without a complete probe set fails the build.

**The suspicion-derived inputs (DR-103).** Four factors take their values from suspicion data or from data that
suspicion produces: A5 (its STR and exit-register parts), F3 (alert and case history), F5 (network risk to an
STR-linked party) and F7 (authority requests). A level that moved on them would tell a KYC user, a segment (phase 13)
or a screening modifier (phase 14) that compliance holds something about the party. So:
1. These factors are `Withheld`: the engine never scores them into the stored rating, in any phase. The stored rating,
   the feed, the segment overlay and the screening modifier therefore carry no suspicion data. The explanation lists
   them as "not used in the computed rating" for every reader, the same words for every party.
2. Suspicion-derived risk reaches the rating only as an upward override made by a person on the case's access list (M
   until phase 18), with the reason code `OVR_UPWARD_COMPLIANCE_JUDGEMENT`. The same code serves non-suspicion
   judgements (adverse information, a declined explanation), so its presence reveals no case. The `reason_text` of an
   override with this code is returned to M only; other readers see the code's words. The override follows the upward
   rules of 3.5 (expiry within twelve months; renewed at the next review if still justified).
3. `trigger_code` of a row driven by a suspicion-linked trigger (`screening.hit`, `case.decision`, `authority_request.recorded`,
   `internal_report.recorded`, `visit.recorded`, `outreach.failed`, `graph.link`, `staff_link.recorded`) is the neutral
   `TRG_REVIEW_REQUESTED`; the TR-15 id is never stored for a non-M reader.
4. Event reviews opened by such a trigger carry the neutral `trigger_code` and checklist content that names no alert,
   case or report.

**Feed, divergence, shadow and export rules.** The rating feed (`GET /rating/feed`, the export file) excludes
suspicion-tier parties by their current tier (a row written before the tier changed remains in the history and is not
recalled: Open doubts). The divergence list (`GET /rating/divergence`) and the disagreement list of a shadow run
(`GET /rating-models/versions/{v}/shadow`) exclude suspicion-tier parties for every caller except M. The exit register
(`GET /exit-register`) is M only for every row: an exit for financial-crime reasons is a suspicion-tier fact whatever the
party's stored tier, so its rows are filtered to M and every other caller receives the masked 404 (Cedar `exit.read`
on the Customer resource, tier suspicion; the entry is recorded by M from a completed `exit_decision`, 3.4 step 8).

**The rating-feed export (job `rating_feed_export`, schedule daily 06:00, gated by `kyc.feed_export_enabled`):**
the day's undelivered `kyc.rating_feed` rows of normal and restricted parties are written to one CSV with the
columns `seq, cif, previous_cbi_label, cbi_label, effective_at`; `EvidenceStore::put_generated` seals it with
owner kind `rating_feed_export`, retention R8, kind `package`; `delivered_at` is set on the exported rows in the same
transaction (the only column the feed guard lets change). The export has no party id, only the CIF the bank already
holds. Step-up class `export` for a manual download by M.

### 3.7 Beneficial ownership (`OwnershipService`, `OwnershipCalculator` pure; MODULES M10)

**Sources and precedence.** Edges come from `canon.party_relationship` with type `owner` or `controller` that are current
(`recorded_to IS NULL`, `valid_to IS NULL OR valid_to >= as_of`) and from `kyc.ownership_inputs` (open rows). For one
edge the CBS share (`share_pct` not null) is used; otherwise the bank-entered share is used; when both exist and differ
by more than half a percentage point, the CBS share is used and a `conflict` flag is listed in the chain. An edge with
neither share is `NotKnown`. The bank enters a share only when the CBS holds none (MODULES M10; the entry route is
`PUT /parties/{id}/ownership/inputs`, 3.8 and section 2).

**Algorithm (depth-first from the party upward; `ownership.max_depth`, 6 by default):**
1. Start from the party as the owned node with effective percentage 100.
2. For each owner edge `o -> n`: the path's effective percentage is the product of the edge shares divided by 100;
   an edge with `NotKnown` makes the path `unknown` (not zero).
3. If the owner already appears on the current path, record the loop path in `loop_paths`, set `loops_detected`, and do
   not descend (never an infinite walk).
4. At `max_depth`, stop descending, set `depth_truncated = true`, and keep the path as unknown.
5. A natural person (`canon.party.type = 'person'`) reached by paths is a beneficial owner when the sum of the known
   path percentages is at least `threshold_pct` (`ownership.threshold_pct`, 20 by default, governed; the calculation uses
   the setting of the computation and stores it in `threshold_pct`). It is `unknown` when the sum is below the threshold
   and at least one of its paths is unknown: a missing percentage gives an unknown result, never clear. Otherwise it is
   below the threshold and counted, not listed.
6. `unknown_count` = number of unknown beneficial-owner candidates and unknown paths. `layers` (for factor A3) = the
   deepest path length of a natural-person owner; "verified" for A3 means a natural person identified in
   `canon.party_identifier` and an edge backed by an evidence object (`ownership_inputs.evidence_object_id`) or by a
   CBS-held share.
7. Write the result: insert the new `kyc.ownership_results` row and set `superseded_at` on the previous current row (one
   transaction; `uq_ownership_results_current`). Only when the input hash changed. Audit `ownership.computed
   {party_id, owners_above, unknown_count, loops_detected}`; enqueue `rating_compute {reason: "ownership"}`.
8. Read projection (`GET /parties/{id}/ownership`): the stored chain is returned to KYC and M; a node that is a
   suspicion-tier party is replaced by `{node: "restricted"}` for every caller, so the chain shows its shape and not the
   identity of a suspicion-tier party; `as_of`, `data_as_of` and the threshold are shown with the result.

**Job `ownership_compute {party_id}`** (coalesce `ownership:<party_id>`, concurrency 2, attempts 3): enqueued by
`rating_touch_fanout` for owned parties of a changed relationship, and by the input route. A loop, a truncated depth
and a conflict are results, not errors: the job completes.

### 3.8 The PEP register, relatives and the pair-memory cap (`PepService`, `PepRules` pure; MODULES M10; phase 14)

**Sources of candidates** (none of them is a status):
1. `pep_list_sweep` (job, every ten minutes, section 4.1): reads `alert.screening_alerts` with `list_kind = 'pep'` and
   `state = 'true'` that have no `kyc.pep_register` row (`uq_pep_register_source_alert`), and creates a candidate per
   alert: `party_id` = the alert's `subject_id` when `subject_kind = 'party'`, else null; `name` = the designation's
   display name from `ref.list_entry_names` (the list entry, not the customer: `ar` is the `raw` of the entry's row with `alias_class = 'primary'` and `script = 'ar'`, and `en` that of its row with `alias_class = 'primary'` and `script = 'lat'`, the lowest `name_slot` first; a script with no such row takes the other script's text, since the CHECK needs both keys); `source = 'pep_list'`,
   `source_alert_id`. A candidate is inserted with `ON CONFLICT DO NOTHING`; audit `pep.candidate_created {pep_id,
   source}` with ids only. The alert id is never returned by a route (3.6). PEP-list alerts are suspicion tier in phase
   14; the candidate is a CDD fact, and its existence is visible to KYC (DR-106).
2. The same job, second step: customers whose `canon.customer.segment` is one of the values the bank designates as
   political (setting `kyc.pep_cbs_class_values`, a JSON list, empty by default) become candidates with `source = 'cbs_class'`
   and no tier (DR-38: a CBS PEP flag is read only when the bank declares it maintained; `canon.party.pep_status` is not
   read by the engine).
3. `POST /pep/register` (KYC, M): a manual candidate (`source = 'manual'`) or a declared one (`source = 'declaration'`,
   also created by a completed review whose checklist answers the PEP declaration). Audit `pep.proposed`.

**Confirmation.** A candidate becomes `confirmed` only through change request `pep_entry` (3.5): the proposer sets the
tier (`P1`, `P2` or `P3`; P3 is the highest, FL-03), the office dates and the assessment. `persistence_until` =
`office_to` plus the decay months of the tier taken from the model document (`pep.persistence_months` per tier, a new
field of the Part C document, DSL.md C.3; values from the bank). `date_quality = unknown` (an empty or sentinel
office date) keeps the entry as ongoing and flagged. On activation: `status = confirmed`, `tier`, `confirmed_by` (not the
proposer, guard); enqueue `pep_relative_suggest {pep_id}` and `rating_compute {reason: "pep"}`; outbox `pep.changed
{party_id, pep_id, change: "confirmed"}`; audit `pep.confirmed`.

**Changes and ends.** `PATCH /pep/register/{id}` (`If-Match`): a tier change or an end is a `pep_entry` change request
(3.5). An office-date or assessment correction without a tier change is direct (audit `pep.updated`). A `former` entry
whose `persistence_until` has passed is ended by `pep_persistence_sweep` (daily; `status = ended`, audit `pep.ended
{reason: "persistence"}`, outbox `pep.changed`); the floors stop applying and the engine opens an event review
(`TRG_PEP_PERSISTENCE_END`, 3.4 step 5). An ended entry never changes status again (guard).

**Relatives and close associates.** `pep_relative_suggest {pep_id}` reads the canonical family links of the PEP's party
(`canon.party_relationship` type `family` with `degree` 1 or 2, both directions) and inserts `suggested` rows
(`basis = canon_family_link`). A suggestion counts for nothing until a person confirms it:
`POST /pep/register/{id}/relatives` (Cedar `pep.relative.confirm`) sets `confirmed` with `confirmed_by`, or adds a
relative or a close associate by hand (`basis = manual`, `degree` NULL for a close associate or a business partner).
Audit `pep.relative_suggested` (count) and `pep.relative_confirmed {pep_id, relative_id, relation}`. The engine reads
the highest confirmed tier of a party's relatives and close associates as `pep.related_tier` (Open doubts: the Part C
floor expressions must name it).

**The pair-memory cap (extends phase 14 `PairMemoryService::propose`).** Phase 14 caps a pair-memory row at
`screening.pair_memory_max_months` (12). This phase extends the method: when the source alert's `list_kind` is `pep`,
`expires_on` is at most `screening.pair_memory_pep_max_months` months ahead (24, governed). The rest of the phase 14
rules are unchanged: an approved row, the four-eyes approval and the invalidation paths.

**Risk modifiers (phase 14 hook).** `RiskModifierSource::modifier(party)` returns the additive modifier of the party's
current rating level from `screening.rating_modifiers` (all zero until the bank sets them), and 0 when the party is not
rated. Phase 14 adds it to the threshold within its own bounds. Suspicion-tier parties get the same modifier table as
every other party (a modifier per level does not reveal a case).

**Read and list rules.** K4 lists entries with the tier as words ("Tier P3, highest"), the persistence date, the
relatives count and the status; the candidate's source is shown as "Politically exposed person list" or "Declared",
never an alert identifier. `kyc.party_pep_state` (migration 0052) is the projection the engine reads.

### 3.9 Relationship-conflict check (phase 03 `ConflictCheck` port; SP-29; K-17)

**Decision: this phase ships the check** (`RelationshipConflictCheck`, the implementation of phase 03's `ConflictCheck`
port that replaces `NoConflictCheck`). `check(principal, subject_party) -> ConflictOutcome`:
1. Load the principal's staff link `admin.users.staff_id` (migration 0052). No link: `Clear` (the check is exact only
   for linked staff; the link is set by ADM, section 4.2, and the gap is an Open doubt).
2. `Conflict` when either holds: (a) a current `canon.customer` row of the subject party has `rm_id` equal to the
   principal's `staff_id` (the relationship owner); (b) `canon.staff.party_id` of the principal's staff row equals the
   subject party (a staff member's own record).
3. Outcome `Conflict` is `DenyReason::Conflict` in phase 03 step 6, so a masked kind answers 404 and a non-masked one
   answers 403 `E_FORBIDDEN`. The check runs for: override proposal and approval (downward), review claim, save and
   completion, PEP confirmation and relative confirmation, and ownership input entry.

### 3.10 Errors and problem codes (`KycError`, thiserror; API.md section 6 only)

| Variant | HTTP and code | Note |
|-|-|-|
| `Validation(Vec<Issue>)` | 422 `E_VALIDATION` | `errors[]` with the DSL.md s.7 Part C codes of 3.1.1, `E_OVERRIDE_LIMITS`, `E_SCHEMA` (checklist and field paths), `E_PROTECTED_ATTRIBUTE`, `E_DANGLING_REF` |
| `BelowFloor` | 422 `E_BELOW_FLOOR` | an override below a firing floor or the completeness cap (K-07, K-24) |
| `OpenQueries` | 422 `E_OPEN_QUERIES` | completion with an open query |
| `UploadRejected` | 422 `E_UPLOAD_REJECTED` | evidence failed antivirus or type checks (phase 02) |
| `NotAllowedInState` | 409 `E_NOT_ALLOWED_IN_STATE` | a model that is not draft; a second open override; a completed review; evidence not clean; a second open PEP relative |
| `Claimed` | 409 `E_CLAIMED` | the review is held by another person |
| `StaleVersion` | 409 `E_VERSION_CONFLICT` | `If-Match` does not match; the current version in the body |
| `PreconditionRequired` | 428 `E_PRECONDITION_REQUIRED` | a versioned mutation without `If-Match` |
| `StepUpRequired` | 401 `E_STEPUP_REQUIRED` | an SU route without a fresh assertion (phase 01) |
| `SelfApproval` | 403 `E_SELF_APPROVAL` | the proposer or an earlier decider approves (override, PEP entry, model) |
| `Conflict`, `Forbidden` | 403 `E_FORBIDDEN` (non-masked) or 404 `E_NOT_FOUND` (masked) | 3.9 and 3.6 |
| `Scope` | 403 `E_SCOPE` | the token or session lacks the route's scope |
| `PurposeRequired` | 422 `E_PURPOSE_REQUIRED` | a restricted-tier read without `X-Access-Purpose` (DR-46) |
| `NotFound` | 404 `E_NOT_FOUND` | missing, or masked (identical body) |
| `Idempotency*` | 400, 422, 409 | phase 00 middleware |
| `Unavailable` | 503 `E_UNAVAILABLE` | the decision or access row could not be written (DR-48); no change is made |
| `Internal` | 500 `E_INTERNAL` | correlation id only |

No problem code is added. A DSL code that API.md section 6 does not list travels inside `errors[]` of `E_VALIDATION`.

### 3.11 Audit events (one per mutation; reads write access events)

| Event kind | On | Resource |
|-|-|-|
| `rating_model.seeded` | `rating seed` | `rating_model` |
| `rating_model.proposed` | draft opened or replaced | `rating_model` |
| `rating_model.shadow_started`, `rating_model.shadow_completed` | shadow run start and end | `rating_shadow_run` |
| `rating_model.activated`, `rating_model.retired` | CR activation; replacement or rejection | `rating_model` |
| `rating.changed` | a row written by the engine | `rating` (party id, levels, labels) |
| `rating.control_failure` | a row with status `control_failure` | `rating` |
| `override.proposed` | downward proposal | `override` |
| `override.applied` | upward at once; downward at activation | `override` |
| `override.ended` | expiry, supersession or rejection | `override` |
| `kyc_review.opened` | sweep or engine opens a review | `kyc_review` (system) |
| `kyc_review.overdue` | overdue timer | `kyc_review` |
| `kyc_review.claimed`, `kyc_review.reassigned` | claim; reassignment | `kyc_review` |
| `kyc_review.saved` | checklist save | `kyc_review` |
| `kyc_review.query_raised`, `kyc_review.query_answered`, `kyc_review.query_closed` | query lifecycle | `kyc_review` |
| `kyc_review.evidence_attached` | evidence link | `kyc_review` |
| `kyc_review.completed` | completion | `kyc_review` |
| `declared_profile.recorded` | declared figures at completion | `declared_profile` |
| `exit.recorded` | `POST /exit-register` (M) from a completed `exit_decision` | `exit_register` |
| `pep.candidate_created` | list sweep, CBS class sweep, manual or declared | `pep_register` |
| `pep.proposed` | manual or declared entry | `pep_register` |
| `pep.confirmed`, `pep.updated`, `pep.ended` | CR activation; direct correction; persistence end | `pep_register` |
| `pep.relative_suggested`, `pep.relative_confirmed` | suggestion run; human confirmation | `pep_relative` |
| `ownership.input_recorded` | `PUT .../ownership/inputs` | `ownership_input` |
| `ownership.computed` | a new result supersedes the current one | `ownership_result` |
| `rating.feed_exported` | export file sealed | `rating_feed_export` (system) |

Every payload carries ids, labels, levels, counts and reason codes; never a name, a note, a chain node or an alert id.

### 3.12 Cedar actions and the file `86_kyc.cedar`

Entities added to `schema.cedarschema` (phase 03 form; the roles are `principal.roles`, there is no Role entity): the
phase 13 `Customer` entity gains `tier: String`; `RatingModel { status: String }`; `RatingShadow { status: String }`;
`KycReview { kind: String, status: String, tier: String }`; `Override { direction: String, status: String, tier: String,
proposed_by: User }`; `PepEntry { status: String, tier: String, pep_tier: String }`. Every party-linked entity has
`tier`, so one forbid covers the masked kinds. Action names: `rating.read`, `rating.distribution.read`,
`rating.divergence.read`, `rating.feed.read`, `rating.feed.export`, `override.read`, `override.propose`,
`override.approve_down`, `kyc_review.read`, `kyc_review.claim`, `kyc_review.work`, `kyc_review.complete`,
`kyc_review.reassign`, `pep.read`, `pep.write`, `pep.relative.confirm`, `ownership.read`, `ownership.input.write`,
`exit.read`, `exit.record`, `rating_model.read`, `rating_model.propose`, `rating_model.validate`,
`rating_model.shadow.run`, `rating_model.shadow.read`. Each action's `appliesTo` lists its resource types
(`Customer`, `KycReview`, `Override`, `PepEntry`, `RatingModel`, `RatingShadow`, `System`).

```cedar
// 86_kyc.cedar (phase 15). Party-linked records are masked kinds (phase 03 section 3.6): a suspicion-tier party
// answers the same 404 as a missing one to every caller that the forbid below stops. Roles are principal.roles.

@id("kyc.work.kyc_m") @role("KYC,M")
permit (principal, action in [Akr::Action::"rating.read", Akr::Action::"rating.divergence.read",
        Akr::Action::"rating.feed.read", Akr::Action::"override.read", Akr::Action::"override.propose",
        Akr::Action::"kyc_review.read", Akr::Action::"kyc_review.claim", Akr::Action::"kyc_review.work",
        Akr::Action::"kyc_review.complete", Akr::Action::"pep.read", Akr::Action::"pep.write",
        Akr::Action::"pep.relative.confirm", Akr::Action::"ownership.read", Akr::Action::"ownership.input.write"], resource)
when { principal.roles.containsAny(["KYC", "M"]) };

@id("kyc.reassign.m") @role("M")
permit (principal, action == Akr::Action::"kyc_review.reassign", resource is Akr::KycReview)
when { principal.roles.contains("M") };

@id("kyc.feed_export.m") @role("M")
permit (principal, action == Akr::Action::"rating.feed.export", resource)
when { principal.roles.contains("M") };

@id("kyc.exit.m") @role("M")
permit (principal, action in [Akr::Action::"exit.read", Akr::Action::"exit.record"], resource is Akr::Customer)
when { principal.roles.contains("M") };

@id("kyc.override.approve_down") @role("CHK,M")
permit (principal, action == Akr::Action::"override.approve_down", resource is Akr::Override)
when { principal.roles.containsAny(["CHK", "M"]) };

// Four eyes at the resource level (DR-57): the proposer never approves.
@id("kyc.override.not_proposer") @deny("self_approval")
forbid (principal, action == Akr::Action::"override.approve_down", resource is Akr::Override)
when { resource.proposed_by == principal };

@id("kyc.rating_model.read") @role("CFG,MRO,CHK,CH,M")
permit (principal, action == Akr::Action::"rating_model.read", resource is Akr::RatingModel)
when { principal.roles.containsAny(["CFG", "MRO", "CHK", "CH", "M"]) };

@id("kyc.rating_model.author") @role("CFG,MRO")
permit (principal, action in [Akr::Action::"rating_model.propose", Akr::Action::"rating_model.validate",
        Akr::Action::"rating_model.shadow.run"], resource is Akr::RatingModel)
when { principal.roles.containsAny(["CFG", "MRO"]) };

@id("kyc.rating_model.shadow_read") @role("CFG,MRO,CHK,CH,M")
permit (principal, action == Akr::Action::"rating_model.shadow.read", resource is Akr::RatingShadow)
when { principal.roles.containsAny(["CFG", "MRO", "CHK", "CH", "M"]) };

@id("kyc.distribution.read") @role("M,CH,IA")
permit (principal, action == Akr::Action::"rating.distribution.read", resource)
when { principal.roles.containsAny(["M", "CH", "IA"]) };

// Suspicion tier is the MLRO's alone until phase 18 adds the case access list. The annotation makes the denial a
// masked one (phase 03 DenyReason::Tier).
@id("kyc.suspicion_mlro_only") @deny("tier")
forbid (principal, action in [Akr::Action::"rating.read", Akr::Action::"rating.divergence.read",
        Akr::Action::"override.read", Akr::Action::"override.propose", Akr::Action::"override.approve_down",
        Akr::Action::"kyc_review.read", Akr::Action::"kyc_review.claim", Akr::Action::"kyc_review.work",
        Akr::Action::"kyc_review.complete", Akr::Action::"pep.read", Akr::Action::"pep.write",
        Akr::Action::"pep.relative.confirm", Akr::Action::"ownership.read", Akr::Action::"ownership.input.write",
        Akr::Action::"exit.read", Akr::Action::"exit.record", Akr::Action::"profile.read"], resource)
when { resource has tier && resource.tier == "suspicion" && !principal.roles.contains("M") };
// profile.read is phase 13's action on the Customer resource: naming it in a forbid adds the masking of this phase and
// changes no phase 13 permit.
```

**Bundle tests** (`tests/policy/86_kyc`): KYC reads a normal-tier rating and a restricted one; A1 is refused by scope
(`E_SCOPE`) and by Cedar (`E_FORBIDDEN` on a non-masked kind); KYC on a suspicion-tier party is refused with the masked
404 and `deny` access event; M reads it; CHK approves a downward override and CHK cannot approve its own proposal; CFG
reads, proposes and validates models; MRO validates; IA reads the distribution and no party data; CH reads the
shadow list; `exit.read` for KYC is refused on every tier. The scope gates of section 2 admit every role these policies
admit (`kyc.work`: KYC, M; `config.author`: CFG, MRO; `config.approve`: CHK, M, CH, MRO; `dashboard.read`: M, CH, IA).

### 3.13 Tipping-off review (for every screen, route, export and notice the phase adds)

| Surface | What a person outside the case's access list can learn from it |
|-|-|
| K1 `GET /parties/{id}/rating*` | The customer's risk level and its drivers (normal and restricted tier). Nothing about a case: suspicion-derived factors are never scored (DR-103), the trigger is neutral, a compliance-judgement override shows its code only, and suspicion-tier parties get the masked 404. |
| K2 `GET /kyc/reviews*` and the workspace | The review kind, due date, status and checklist of a party the KYC user may see. A review of a suspicion-tier party is not listed (filtered) and not retrievable (masked 404). The trigger shows only TR or neutral codes. |
| K3 `GET /parties/{id}/ownership` | Owners and percentages; a suspicion-tier node is shown as "restricted". |
| K4 `GET /pep/register`, `POST`, `PATCH`, relatives | PEP status and tier of a party. A list-sourced candidate shows "Politically exposed person list" with no alert id and no score. Suspicion-tier linked entries are masked. |
| `GET /rating/feed` and the export | The CBI label of a party and its change date, for parties that are not suspicion tier. Nothing about a case or an alert. |
| `GET /rating/divergence` | The CBS CDD level against the computed one; suspicion-tier parties excluded. |
| `GET /rating/distribution` | Counts only, with no party identifier. |
| `GET /overrides` | Overrides with reason codes and approvers, for normal and restricted tier. |
| Downward override approval (CHK, M) | The reason code, the level change and the evidence reference. Not the case, the alert or the source of the evidence. |
| `GET /exit-register`, `POST /exit-register` | M only: the exit is a suspicion-tier fact. Every other caller gets the masked 404. |
| `GET /rating-models/versions/{v}/shadow` | Counts and level migration; the disagreement list excludes suspicion-tier parties for every caller except M. |
| Audit events of 3.11 | Ids, labels, levels, counts and reason codes; no tier, no trigger cause for a suspicion-linked trigger. |
| Notifications and SSE | None are emitted in this phase. The inference probes of 3.6 prove it. |

The answer for every row is "nothing beyond the record's own level, dates and codes". No row lets a person learn the
existence of a case, an alert, a report or a filing.

### 3.14 Settings and schedules used

Settings of migration 0053: `ownership.threshold_pct` (governed), `ownership.max_depth`, `kyc.light_yearly_months`
(governed), `kyc.declared_max_age_months` (governed), `kyc.review_open_ahead_days`, `kyc.feed_export_enabled`,
`rating.trigger_debounce_s`, `rating.page_size`, `rating.country_fanout_page`, `screening.pair_memory_pep_max_months`
(governed), `screening.rating_modifiers` (governed, JSON), and the schedules `schedule.rating_nightly`,
`schedule.kyc_review_sweep`, `schedule.pep_list_sweep`, `schedule.pep_persistence_sweep`, `schedule.rating_feed_export`.
Setting `kyc.pep_cbs_class_values` (JSON list, default `[]`, governed; section 3.8 step 2) is in migration 0053.
Model parameters (not settings): `unknown_score`, `peak_term`, the bands, the floors and their decay, the completeness
cap, the triggers and their SLAs, the stickiness, the overrides limits and the controls matrix, all in the Part C document.

---

## 4. Infrastructure Updates

### 4.1 Job kinds (Postgres `jobs`)

| Kind | Binary | Payload | Coalesce key | Concurrency | Attempts | Schedule or trigger |
|-|-|-|-|-|-|-|
| `rating_compute` | akr-worker | `{party_id, reason, trigger_code?}`; reason one of `canon`, `country`, `pep`, `review`, `override`, `ownership`, `nightly`, `model` | `rating:<party_id>` | 4 | 5 | enqueued by the consumers of section 3.3, run after `rating.trigger_debounce_s`; by `rating_nightly` |
| `rating_touch_fanout` | akr-worker | `{batch_id}` | `rating_fanout:<batch_id>` | 2 | 5 | outbox `canon.touched` (phase 11; consumer registered here) |
| `rating_country_fanout` | akr-worker | `{iso2, cause, effective_at}` | `rating_country:<iso2>` | 1 | 5 | outbox `ref.country_level_changed` (phase 09; consumer registered here; TR-10) |
| `rating_nightly` | akr-worker | `{reason, cursor?}` | `rating_nightly` | 1 | 3 | `schedule.rating_nightly` daily; enqueued by a model activation (`reason = model`) |
| `rating_shadow_run` | akr-worker | `{shadow_run_id}` | `rating_shadow:<model_version_id>` | 1 | 2 | `POST /rating-models/versions/{v}/shadow` |
| `kyc_review_sweep` | akr-worker | `{}` | `kyc_review_sweep` | 1 | 3 | `schedule.kyc_review_sweep` daily; opens due reviews (3.4), ends expired overrides (3.5) |
| `timer_kyc_review_clock` | akr-worker | timer row `{review_id}` | `kyc_review_clock:<review_id>` | 2 | 3 | timer kind `kyc_review_clock` (below), scheduled at the review's `due_at`; escalates an overdue review |
| `pep_list_sweep` | akr-worker | `{}` | `pep_list_sweep` | 1 | 3 | `schedule.pep_list_sweep` every ten minutes; creates candidates from decided `pep` alerts and the CBS class setting (3.8) |
| `pep_persistence_sweep` | akr-worker | `{}` | `pep_persistence_sweep` | 1 | 3 | `schedule.pep_persistence_sweep` daily; ends expired former entries (3.8) |
| `pep_relative_suggest` | akr-worker | `{pep_id}` | `pep_suggest:<pep_id>` | 1 | 3 | activation of a `pep_entry` change request (3.5) |
| `ownership_compute` | akr-worker | `{party_id}` | `ownership:<party_id>` | 2 | 3 | `rating_touch_fanout` for owned parties of a changed relationship; the `PUT .../ownership/inputs` route |
| `rating_feed_export` | akr-worker | `{day}` | `rating_feed_export:<day>` | 1 | 3 | `schedule.rating_feed_export` daily, gated by `kyc.feed_export_enabled` (3.6) |

No kind starts with `ingest_` (that prefix is akr-ingestd's). Every handler re-reads its owner before it acts (phase 00
rule). The `rating_compute` handler takes the party's advisory lock (3.2 step 1), so the coalesce key and the lock never
disagree.

### 4.2 Timers, outbox, shipped definitions and the route map

- **Timer kind** `kyc_review_clock` (owner kind `kyc_review`), scheduled by `KycReviewService` at the review's due
  instant; calendar-based intervals use `BankCalendarClock` (phase 09) and store `clock` and `calendar_id` in the payload.
  Handler `timer_kyc_review_clock` (3.4 step 3).
- **Outbox topics produced** (catalogued in the phase 00 topic registry): `rating.changed {party_id, rating_id}`,
  `pep.changed {party_id, pep_id, change}`, `kyc_review.completed {review_id, party_id, outcome}`,
  `override.changed {override_id, party_id, status}`. Ids only, never a name or a note.
- **Outbox topics consumed:** `canon.touched` (phase 11, section 3.3), `ref.country_level_changed` (phase 09; the
  phase 09 note "no consumer until phase 15" is closed by this phase), `dq.stale` (phase 11; nightly effect only), and the
  topics of own production above.
- **Shipped workflow definitions** (phase 04 `backend/workflows/shipped/`, extended by three files, the DSL.md E.4
  keys; `resource_kinds: ["change_request", "kyc_review", "exit_decision"]` as the instance resource kinds require):
  - `kyc_override_down`: `approve` role CHK or M, not the proposer, not the relationship owner (3.9) -> `record_decision`
    (3.5 step 5). Timeouts: `remind` after two business days, then escalate to CH after three.
  - `kyc_edd_signoff`: `approve` by the controls matrix approver of the level (`position:senior_management_and_mlro` at
    level 4 and 5, DSL.md C.7), not the reviewer -> `record_decision` (3.4 step 7).
  - `exit_decision`: one step `approve` role M (decision type `kyc.decision` of the phase 09 matrix, whose roles include M), not the reviewer who completed the review -> `record_decision` (stores the outcome; M then
    records the register entry, 3.4 step 8). Version 1 ends there; phase 20 seeds version 2, which adds `create_ops_task` (section 7.2).
  The phase 04 shipped set grows from nine to twelve; `akr-ctl workflow validate` must pass for all twelve.
- **Route map** (`change.route_map`, governed setting, DR-54; rows added by this phase's release data): object kind
  `rating_model` -> `cr_material_validated`; `pep_entry` with class `minor` -> `cr_minor`; `pep_entry` with class
  `material` -> `cr_material`; `rating_override` -> `kyc_override_down`.
- **Reason codes** (phase 09 seed file `backend/packs/reference/reason-codes.v1.json`, family `override`, inactive until
  the family change request of phase 09 activates them): `OVR_UPWARD_ADVERSE_INFO`, `OVR_UPWARD_PEP_JUDGEMENT`,
  `OVR_UPWARD_EXPECTED_ACTIVITY`, `OVR_UPWARD_COMPLIANCE_JUDGEMENT` (DR-103), `OVR_DOWN_SOURCE_DOCUMENTED`,
  `OVR_DOWN_STRUCTURE_VERIFIED`; and in family `decision`, `DEC_EXIT_FINANCIAL_CRIME` for the exit register.
  Change requests use the family `change` (for example `CHG_RISK_ASSESSMENT`, phase 04).
- **Checklist templates** (`backend/packs/kyc/checklists/*.json`, release data, versioned by name): `periodic_v1`,
  `light_v1`, `light_yearly_v1`, `edd_person_v1`, `edd_entity_v1`. Each item has `key`, `required`, `answer_kind` and
  its wording in both languages.

### 4.3 AppState additions, masking registry and OpenAPI

- `api`: `rating: Arc<RatingQuery>`, `overrides: Arc<OverrideService>`, `kyc: Arc<KycReviewService>`,
  `pep: Arc<PepService>`, `ownership: Arc<OwnershipService>`, `rating_models: Arc<RatingModelService>`,
  `exit: Arc<ExitService>`; ports: `conflict: Arc<RelationshipConflictCheck>` (replaces `NoConflictCheck`), the phase 13
  `RatingSource` and the `kyc` variant of `DeclaredProfileSource`, `RiskModifierSource` (phase 14 hook); change targets
  `rating_model`, `pep_entry`, `rating_override` registered in the `ChangeTargetRegistry`; evidence owner kinds
  `kyc_review`, `pep_register`, `ownership_input`, `override`, `rating_feed_export` registered (phase 02 section 3).
- `worker`: `RatingService`, `RatingFanout`, `RatingShadowService`, `KycReviewService` (sweep and timer), `PepService`
  (list and persistence sweeps), `OwnershipService`, `RatingFeedExporter`; the `OwnershipCalculator`, `ReviewClock` and
  `PepRules` pure libraries.
- Masking: `customer` becomes masked; `kyc_review`, `override` and `pep_entry` are registered masked kinds with the probe
  sets of 3.6 in `api/tests/inference/` (the phase 03 `MASKED_KINDS` registry).
- OpenAPI: tags `rating` (8 operations), `kyc` (10), `pep` (4), `ownership` (2), `exit` (2), `rating-models` (6): 32 in
  all, each with `x-akr-action` (section 2). The two routes of sections 7.4 and 7.5 join the tags `ownership` and
  `rating`, which makes 34. SU operations declare `X-StepUp-Assertion` and reference the step-up
  scheme; every POST, PUT and PATCH documents `Idempotency-Key`; versioned mutations document `If-Match` and 428 and 409;
  restricted reads document `X-Access-Purpose` and `Cache-Control: no-store`; the evidence stream documents
  `application/octet-stream` and `X-Content-SHA256`; `POST` routes that create a change request document 202 with
  `Location`.

### 4.4 Network zones, readiness, metrics and existing updates

- **Network zones:** no change. akr-api and akr-worker read `canon` and `alert` (phases 10 and 14) and write `kyc`;
  nothing reaches a CBS or the gateway zone.
- **Readiness:** akr-api `/readyz` adds `rating_model`: a live version of RM-IQ exists. Without one the rating routes
  answer 409 `E_NOT_ALLOWED_IN_STATE` and readiness is not green.
- **Metrics:** `akr_ratings_level{level}` and `akr_ratings_cbi{label}` (gauges, bank scope); `akr_rating_control_failures`
  (gauge); `akr_kyc_reviews{kind, state}` with `state` in `open`, `overdue`, `date_unknown` (gauge);
  `akr_overrides_active{direction}` (gauge); `akr_pep_candidates_open` (gauge); `akr_rating_fanout_seconds` (histogram,
  from the `canon.touched` event to the `rating.changed` row); `akr_rating_shadow_seconds` (histogram).
- **Alert rules:** a review overdue for more than seven days; `date_unknown` reviews older than two working days; a
  `rating_nightly` run not completed by 06:00 bank time; a control-failure count rising by more than the bank's
  threshold in a day; a PEP candidate open for more than five working days; a failed `rating_feed_export`.
- **Phase 00:** settings and schedules of migration 0053; topic registry; timer kind `kyc_review_clock`; job kinds of 4.1.
- **Phase 01:** no new step-up class (`decision`, `approval` and `export` are used).
- **Phase 02:** evidence owner kinds of 4.3; `kyc.ratings` in the partition registry with the factory `kyc.create_partition`
  and `ahead_months = audit.partition_ahead_months`; access events for every read of a party-linked record, through the
  existing access layer.
- **Phase 03:** Cedar file `86_kyc.cedar` (section 3.12) and the entities of 3.12 in `schema.cedarschema`; the `customer`
  resource adapter extended with `tier` and masking (phase 13 registered it); adapters `KycReview`, `Override`, `PepEntry`;
  masked kinds; `RelationshipConflictCheck` implements `ConflictCheck`; the user update route `PATCH /admin/users/{id}`
  gains the field `staff_id` (ADM, `admin.users` scope; the change is a phase 03 route extension, no new route).
- **Phase 04:** three change-request target kinds registered; three shipped definitions and the route-map rows of 4.2.
- **Phase 06:** `AttributeCatalogue` gains `rating.level` and `rating.cbi_level` (method `register_rating_attributes()`
  added to the catalogue; phase 13 section 3.3 reads them for risk-overlay segments).
- **Phase 09:** the `ref.country_level_changed` consumer; the reason-code seed rows of 4.2 (inactive).
- **Phase 10:** no change to `canon`; `canon.kyc_document`, `canon.staff` and `canon.customer.rm_id` are read.
- **Phase 11:** the `canon.touched` consumer; the freshness snapshot and blind periods are read.
- **Phase 13:** the `customer` adapter extended (not re-registered); the `DeclaredProfileSource` kyc variant and the
  `RatingSource` implemented; `profile.read` (`GET /parties/{id}/profiles`) gains the masking of the customer kind with no
  route change.
- **Phase 14:** `PairMemoryService::propose` extended with the PEP cap (3.8); the risk-modifier hook implemented; the
  `pep` list outcomes are read by `pep_list_sweep` (no change to phase 14 code).
- **API.md 3.10:** the section 2 table has 32 rows: the 26 rows of API.md 3.10 (`GET /parties/{id}/profiles` stays with
  phase 13) and six added rows, marked. The scope list of API.md s.2 is unchanged. **DATA-MODEL s.11:** aligned with section 1 (eight tables added, `kyc_reviews` gains the columns
  of 0051). **DSL.md Part C:** C.2 withheld factors (DR-103), C.3 the PEP inputs `pep.persistence_months` and
  `pep.related_tier`, C.8 the normalised inherent score (DR-104).
- **DESIGN-SYSTEM.md:** K2, K3 and K4 have catalogue rows only (8.1); section 5 of this phase is the proposed spec.

---

## 5. Frontend Impact

Screens by catalogue ID (DESIGN-SYSTEM.md 8.1): K1 `/parties/:id/risk` (WORK), K2 `/kyc` (LIST + WORK), K3
`/parties/:id/ownership` (WORK), K4 `/pep` (REGISTER). All four are in the KYC rail group (8.2) and are shown to the
KYC and M roles only; a role without the scope sees no entry and gets the X6 page for a direct URL. A masked party
(suspicion tier) gets the X6 page, byte-identical to a missing record. Every page shows `data_as_of` (C-29). Purpose
is asked once per session per party through the existing purpose dialog (phase 08); a `E_PURPOSE_REQUIRED` answer
reopens it. Money, percentages and dates follow the product formats of phase 08; tier and status are shown as words.

- **K1 Customer risk profile (mockup 04; UX-59).** The level word, the CBI label and the score; a 12-month trend drawn
  from `GET /parties/{id}/rating/history` (a line, with units and a table alternative; no decorative gauge). Drivers as
  sorted bars with points, reason, refresh date and the missing-input share; the restricted driver line of 3.6 for
  non-M users; floors and triggers in words ("Floor applies: politically exposed person, P3"); the override history with
  expiry; rating-change reasons; the EDD checklist with its dates (from the current review); the open-case and watch strip
  is hidden in this phase (phase 18 section 7+ adds it within the user's access). Actions: "Propose override" (SU; upward and downward
  with the reason list of 4.2; the downward one shows "Goes to compliance for approval"). Hooks: `useGetPartiesIdRating`,
  `useGetPartiesIdRatingHistory`, `useGetPartiesIdRatingExplanation`, `usePostPartiesIdOverrides`, `useGetOverrides`
  (with `party_id`), `useGetKycReviews` (with `party_id`).
- **K2 KYC review queue and workspace (LIST + WORK).** Left: the queue as a data table with columns customer name
  (`display_name` in `KycReviewView`, the current primary-script name from `canon.party_name`), kind word, due date, due
  basis word ("Refresh clock", "Level change", "Date not known"), status word and assignee. Tabs: "Overdue and date not
  known" (first), "Due in 30 days", "Mine", "Awaiting answer", "Completed in 30 days". Right: the workspace of one review:
  header (kind, due, basis, level at open), checklist by item with dates and required marks, the queries panel (raise,
  answer, close), the evidence panel ("Being checked" until clean; "Open" when clean; the evidence stream with a hash
  note), the declared-figures form (per currency), the complete button (SU `decision`; a `E_OPEN_QUERIES` answer lists
  the open queries inline), and "Reassign" (M only). Hooks: `useGetKycReviews`, `useGetKycReviewsId`,
  `usePostKycReviewsIdClaim`, `usePatchKycReviewsId`, `usePostKycReviewsIdQueries`,
  `usePatchKycReviewsIdQueriesQid`, `usePostKycReviewsIdEvidence`, `useGetKycReviewsIdEvidenceEvidenceId`,
  `usePostKycReviewsIdComplete`, `usePostKycReviewsIdReassign`.
- **K3 Ownership and evidence vault (WORK).** The chain as an indented list, not a graph library: each node shows its
  name, the effective percentage in figures and words ("20 per cent, above the threshold of 20"), the basis ("Held by the
  bank system" or "Entered by the bank, evidence attached"), the evidence link, "Not known" for an unknown share, "Loop
  detected" for a loop, and "Restricted party" for a suspicion-tier node. A threshold line and the result date. Where
  the CBS holds no percentage, a form enters owners and shares (SU not required; the entry is audited). Hooks:
  `useGetPartiesIdOwnership`, `usePutPartiesIdOwnershipInputs`.
- **K4 PEP register and relatives (REGISTER).** A data table: name (words for a name-only candidate), status word
  ("Candidate", "Confirmed", "Ended"), tier word ("P3, highest"), office dates ("Date not known" for an unknown one),
  persistence date, source word ("Politically exposed person list", "Declared", "Manual", "Designated class"), relatives
  count. Filters by status, tier and source. Row actions: "Propose as confirmed entry" (opens the `pep_entry` change
  request, 202), "Change tier" (change request), "Confirm relative" for a suggested row. A candidate never shows a tier.
  Hooks: `useGetPepRegister`, `usePostPepRegister`, `usePatchPepRegisterId`, `usePostPepRegisterIdRelatives`.
- **Shared components** (phase 08): data table with sticky header and keyboard row navigation, `StepUpDialog`, purpose
  dialog, the X6 page, the status and tier chips, the change-request panel of phase 08 (G10) for CR-backed actions.
- **Model screen.** The rating model lifecycle (draft, shadow, live, retired), the validation list and the shadow results
  belong to the phase 16 studio pattern and are not built here: CFG and MRO use the Swagger UI and `akr-ctl` until then
  (Open doubts). The shadow result list is read through `GET /rating-models/versions/{v}/shadow`
  (`useGetRatingModelsVersionsVShadow`).

---

## 6. Verification

1. `make migrate`: 0050 to 0053 apply; a second run is a no-op. The seed partitions `kyc.ratings_<yyyy>_<mm>` for the
   current month and three ahead exist. `SELECT kyc.create_partition('overrides', 2030, 1)` fails (only the allow-listed
   `ratings` gets partitions).
2. Negative database checks, as `akr_app`: `UPDATE kyc.ratings SET level = 2` fails with `append-only`; `DELETE FROM kyc.ratings`
   fails; `UPDATE kyc.rating_models SET document = '{}'` on a live row fails with `immutable`; a live version moved to
   draft fails; a second `live` row for RM-IQ fails `uq_rating_models_live`; an `overrides` row of direction `down` without
   `change_request_id` fails its CHECK; `approved_by = proposed_by` fails; a `rating_feed` update of `cbi_level` fails; a
   completed `kyc_reviews` row cannot be updated; a `pep_register` row confirmed by its proposer fails its CHECK; a
   confirmed entry set back to candidate fails; `DELETE` on any `kyc` table fails for `akr_app`; `akr_ingest` and `akr_assess`
   have no privilege on any `kyc` table; `akr_readonly` reads every table; `admin.users.staff_id` is unique where set.
3. Classification and retention rows exist for the 16 tables of section 1 (`admin.retention_assignment` where
   `schema_name = 'kyc'` returns 16 rows; `admin.column_classification` has a row for every column of each table, with
   the lineage columns internal); the phase 00 classification check passes.
4. `cargo check` with `cargo sqlx prepare`; `cargo clippy --all-targets -- -D warnings`; `cargo deny check`.
5. `akr-ctl rating seed` installs RM-IQ 1.0.0 as live (`rating_model.seeded`); a second run answers `already_seeded` and
   writes nothing. `akr-ctl rating verify-golden` exits 0: each research 26 s.14e example reproduces its final level (the
   teacher with a public payroll account at level 1; the import LLC with letters of credit at level 3) from fixtures that
   supply all 27 inputs.
6. `cargo test`, including:
   - engine (`domain/kyc/tests`): the normalised inherent formula on the research numbers (block scores 6.7, 15.0, 10.0,
     10.0, 10.0, 0.0 with weights 25, 15, 20, 15, 5, 20 give weighted 7.9, peak 15.0, inherent 10.4 at p = 0.35); unknown
     applicable factor scores 60; a block with no applicable factor renormalises; one mandatory unknown gives
     `min_level_3`; two give `control_failure` with no level; FL-03 fires for P3 and does not fire after its decay with an
     event review opened; an upward override raises the level; a downward override never crosses a floor (E_BELOW_FLOOR at
     proposal and at approval); a fall is held until a review completes (`held_fall`); an unchanged state writes no row
     (hash stability, and `as_of` alone does not change the hash); a stale factor opens one TR-23 review;
   - validation (`RatingModelDoc::validate`): every Part C code of 3.1.1 raised by its fixture; `E_PROTECTED_ATTRIBUTE` for
     a factor on `customer.gender`; `E_REVIEW_INTERVAL_LIMIT` at 25 months;
   - ownership: product of shares; the threshold boundary (exactly 20 is above); an unknown share gives unknown and never
     zero; a loop is recorded and not walked; depth truncation; the CBS share wins over an entered share with a conflict flag;
   - clocks: a sentinel date gives `date_unknown` and escalation; each kind's due date from the controls row; one open
     review per party and kind; an event joins the open event review;
   - pair-memory cap: a PEP list row may run 24 months, a sanctions list row 12;
   - masking: the probe set of 3.6 complete for `customer`, `kyc_review`, `override` and `pep_entry`; the latency comparison;
     the list predicate equals the Cedar policy over generated principals and rows;
   - the consistency fixtures: with every factor `good` and no suspicion input, every RM-IQ floor off, the computed levels
     match the committed golden file `backend/packs/rating/rm-iq.golden.v1.0.0.json`.
7. Cedar bundle tests (`tests/policy/86_kyc`, section 3.12): every permit and forbid of the file is exercised by at least
   one principal; the scope gates of section 2 admit every role the policies admit.
8. Curl suite `tests/curl/phase15_kyc.sh` (dev users `kyc@bank.test`, `m@bank.test`, `cfg@bank.test`, `mro@bank.test`,
   `chk@bank.test`, `ch@bank.test`, `ia@bank.test`, `a1@bank.test`, `ds@bank.test`, `adm@bank.test`; the seeded parties
   `P1` (normal tier, a rated customer), `P2` (entity with a CBS-held share), `P3` (a party in a relationship loop) and
   `PS` (suspicion tier); the phase 14 suite has left one `pep` alert decided `true` for `P1`'s counterparty):
   - As A1 `GET /api/v1/parties/<P1>/rating` -> 403 `E_SCOPE`; as IA the same -> 403 `E_SCOPE`; as DS -> 403 `E_SCOPE`.
   - As KYC `GET /api/v1/parties/<P1>/rating` without `X-Access-Purpose` -> 422 `E_PURPOSE_REQUIRED`; with the purpose ->
     200, `data_as_of` present, `cbi_label` present, no factor entry named A5, F3, F5 or F7 in the response.
   - As KYC `GET /api/v1/parties/<PS>/rating` with the purpose -> 404 `E_NOT_FOUND`, and `cmp` of the body and headers
     against `GET /api/v1/parties/<unknown uuid>/rating` shows no difference; the access event `deny` exists (`psql` as
     `akr_readonly`). As M the same -> 200.
   - As KYC `POST /api/v1/parties/<P1>/overrides` with `Idempotency-Key: ovr-0001`, direction up, one level above the
     computed level, `OVR_UPWARD_ADVERSE_INFO`, expiry 60 days, and no step-up -> 401 `E_STEPUP_REQUIRED`; with the
     assertion -> 201, `status: active`; the same key and body -> 201 with `x-idempotent-replay: true`; a second key while
     active -> 409 `E_NOT_ALLOWED_IN_STATE`; expiry 13 months -> 422 `E_VALIDATION`; without `Idempotency-Key` -> 400
     `E_IDEMPOTENCY_REQUIRED`.
   - As KYC `POST /api/v1/parties/<P1>/overrides` direction down with `to_level` two levels below -> 422 `E_VALIDATION`
     (`E_OVERRIDE_LIMITS`); one level below the floor -> 422 `E_BELOW_FLOOR`; one level down with evidence and a step-up
     -> 202 with `Location`; the change request is of class material and route `kyc_override_down`.
   - As M approve the downward change request it proposed -> 403 `E_SELF_APPROVAL`; as CHK -> 200 and the override is
     `active` with `approved_by` = the CHK user; as KYC approve -> 403 `E_FORBIDDEN`.
   - Reviews: as KYC `GET /api/v1/kyc/reviews?party_id=<P1>` -> 200; `POST .../claim` with `If-Match` -> 200; the same
     claim as another KYC user -> 409 `E_CLAIMED`; `PATCH` without `If-Match` -> 428 `E_PRECONDITION_REQUIRED`; with a stale
     version -> 409 `E_VERSION_CONFLICT`; `POST .../queries` -> 201; `POST .../complete` with step-up -> 422
     `E_OPEN_QUERIES`; answer and close the query, complete without step-up -> 401 `E_STEPUP_REQUIRED`; with step-up -> 200
     `status: completed`; a second complete -> 409 `E_NOT_ALLOWED_IN_STATE`; `GET /api/v1/kyc/reviews?party_id=<PS>` -> 200
     with no row of `PS`.
   - Evidence: `POST .../evidence` with a benign PDF -> 201; `GET .../evidence/<id>` before the scan is clean -> 422
     `E_NOT_ALLOWED_IN_STATE`; after it -> 200 with `X-Content-SHA256` equal to the evidence row's sha256.
   - PEP: as KYC `POST /api/v1/pep/register` with `status: "confirmed"` -> 422 `E_VALIDATION`; without a status (a
     candidate) -> 201; the candidate's entry in `GET /api/v1/pep/register` has no `source_alert_id` key; a confirmation by
     the change request: KYC approves -> 403 `E_SELF_APPROVAL`; CHK approves -> 200 and the entry shows its tier word;
     `POST /api/v1/pep/register/<id>/relatives` as KYC -> 200; as CHK -> 403 `E_FORBIDDEN`; the decided `pep` alert of the
     phase 14 suite yields one `source: pep_list` candidate after `pep_list_sweep` runs.
   - Ownership: `PUT /api/v1/parties/<P2>/ownership/inputs` with a share of 60 and evidence -> 200; `GET
     /api/v1/parties/<P2>/ownership` -> 200 with the effective share 60 and `data_as_of`; an owner with a missing share shows
     `unknown` and never `0`; `GET /api/v1/parties/<P3>/ownership` -> 200 with `loops_detected: true` and no walk past the loop.
   - Model lifecycle: as CFG `POST /api/v1/rating-models/versions` -> 201 draft; `PUT` with a weight that does not sum to 100
     and `POST .../validate` -> 200 with `E_WEIGHTS_SUM`; corrected -> zero errors; `POST .../shadow` -> 202 with
     `Location`; `GET .../shadow` -> 200 with `tier_migration` and `volume_effect`; `POST /api/v1/change-requests` for the
     version -> 201 class material; the approvals in order (MRO, then CHK and M in parallel) -> activation; `PUT` on the
     retired version -> 409 `E_NOT_ALLOWED_IN_STATE`; as DS `POST /api/v1/rating-models/versions` -> 403 `E_SCOPE`.
   - Feed and lists: as KYC `GET /api/v1/rating/feed` -> 200 with a cursor and no row of `PS`; as IA `GET
     /api/v1/rating/feed` -> 403 `E_SCOPE`; as IA `GET /api/v1/rating/distribution` -> 200 with counts and no party id;
     as KYC `GET /api/v1/exit-register` -> 200 with no row of a suspicion-tier party; as M a recorded exit is listed.
   - Idempotency replay and `If-Match` on the model draft are covered above; every mutation of the suite writes one audit
     event of section 3.11.
9. Swagger UI lists 34 new operations (the 32 of section 2 and the two of sections 7.4 and 7.5), each with `x-akr-action`; `make generate-api` twice gives zero diff; the hooks of
   section 5 exist in the generated client.
10. Audit trail after the curl run: `audit_log` holds `rating.changed`, `override.applied` (up and down), `override.proposed`,
    `kyc_review.completed`, `kyc_review.evidence_attached`, `pep.candidate_created`, `pep.confirmed`,
    `pep.relative_confirmed`, `ownership.computed`, `rating_model.activated`, `rating_model.shadow_completed`,
    `authz.denied` (the A1, IA and DS attempts and the KYC attempts on `PS`) and the masked `deny` access event; no payload
    contains a party name, a reason text, a note, an answer or a `source_alert_id`; `akr-ctl audit verify` passes.
11. No CBS connection: the phase 05 statement journal shows no statement between the start and the end of the curl run;
    the phase 10 canonical tables are unchanged by this phase (`akr_app` has no write on `canon`).
12. Phases 00 to 14 tests still pass. The phase 13 curl suite still passes with the customer masking enabled: its
    `profile.read` checks for suspicion-tier parties now answer 404.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 3b, from tools/agent/gap1-run3b.md; lead-verified (tools/agent/gap1-run3b-verdict.md).

### 7.1 Tier of the customer kind (phase 17)
From phase 17 on, the `tier` of the masked `customer` kind is `casework.effective_tier(party_id)`; until then it is
`canon.party.sensitivity_tier`.

### 7.2 exit_decision and create_ops_task (phase 20, DSL E.4)
This phase ships the workflow `exit_decision` ending at `record_decision`; an effect that is not registered fails
publish, so it does not name `create_ops_task`. Phase 20 registers the effect `create_ops_task` and ships version 2 of
`exit_decision`, which adds it after `record_decision`.

Gap pass 1 (2026-10-10), run 3b: corrections found while checking phases 11 to 16 and the API.md route owners (tools/agent/gap1-run3b-verdict.md); lead-verified.

### 7.3 The screening.hit handler is registered here (DR-375)
The last row of the trigger table (section 3.3), which lists `screening.hit` with the later topics as "not wired
here", is corrected for `screening.hit`: phase 14 runs before this phase, so this phase registers the topic's first
handler (its trigger on `screening.hit`, DSL.md C.5, stored with the neutral `trigger_code` `TRG_REVIEW_REQUESTED`,
section 3.6) and adds the outbox write to phase 14's decision transaction (phase 14 section 7+): payload `{alert_id,
party_id, list_kind, outcome}` for a party subject decided `potential` or `true`. The handler follows the rules under
the table (it enqueues, never re-scores in the consumer's transaction). The other topics of that row are registered by
the phases that produce them (17 to 19, 26).

Gap pass 1 (2026-10-10), run 5b (coverage of SP and K IDs), from tools/agent/gap1-run5b.md; lead-verified (tools/agent/gap1-run5b-verdict.md).

### 7.4 Ownership coverage view (K-19, DR-395)
| Method | Path | Scope | Description |
|---|---|---|---|
| GET | `/api/v1/ownership/coverage` | `dashboard.read` | Beneficial-ownership coverage by level, counts only (`rating.distribution.read`) |

`OwnershipService::coverage(ctx)` (Cedar `rating.distribution.read`, M, CH and IA; no new action): one SELECT over
`kyc.rating_current` joined to `kyc.ownership_results` (`superseded_at IS NULL`) by `party_id`, grouped by current
level: parties rated, parties with a current ownership result, results with `unknown_count > 0`, results with
`loops_detected`, results with `depth_truncated`, and results whose `as_of` is older than the party's KYC refresh
interval of the controls matrix. Suspicion-tier parties are counted with the rest and never listed; no party id is
returned. The registry import path of K-19 is P2 (DR-396). Shown on K3 as a header strip. Test: a fixture of three
parties (one result with an unknown owner, one without a result) gives the expected counts.

### 7.5 Override analytics (K-25, DR-395)
| Method | Path | Scope | Description |
|---|---|---|---|
| GET | `/api/v1/rating/overrides/analytics` | `dashboard.read` | Override counts by proposer and branch, downward share, expiries, follow-on alerts and STRs (`rating.distribution.read`) |

`OverrideService::analytics(ctx, from, to)` (Cedar `rating.distribution.read`, M, CH and IA; no new action), counts
only, over `kyc.overrides` with `effective_from` in the window (at most 366 days):
1. Per `proposed_by`: total, `direction = 'down'`, `end_reason = 'expired'`; the downward share is down over total.
   User ids are returned, never names.
2. Per branch: the same counts, the branch being `canon.customer.branch_id` of the party's current customer rows
   (`recorded_to IS NULL`); a party with customer rows at two branches counts at each.
3. From phase 18 on: overall counts (not per person or branch) of overrides whose party has an `alert.alerts` row
   created within 90 days after `effective_from`, and of those whose party is on a case (`casework.case_parties`) with a
   `casework.decisions` row of type `str` decided within 180 days after it. Before phase 18 the two fields are null.
No audit event (a read). Test: fixture overrides by two proposers give the expected counts and share.

Gap pass 1 (2026-10-10), run 7b (coverage of F, B and U IDs), from tools/agent/gap1-run7b.md; lead-verified (tools/agent/gap1-run7b-verdict.md).

### 7.6 The profile.breach handler (phase 17)
From phase 17 on, `profile.breach` (TR-11) is written by phase 17's alert intake, which registers this phase's
handler for it with the neutral `trigger_code` `TRG_REVIEW_REQUESTED` (phase 17 section 7+).

Gap pass 1 (2026-10-10), gap items (ac), (ad) and (ae) of tools/agent/gap-items.md (found in runs 7b and 8c); lead work, no Haiku draft.

### 7.7 Trigger evaluation and the account events (K-11, DSL.md C.5; gap item (ac); DR-405)
Section 3.3 lists the trigger topics, but no step said how a trigger's `on`, `when` and `action` are applied, and five
topics had no producer. This block is that path. It replaces the last row of the section 3.3 table ("not wired here").
1. **Job.** `rating_compute` gains the reason `trigger` and two payload fields: `event` (a DSL.md C.5 `on` name) and
   `event_ref` (the id the producer gave: an alert, case, RFI, account or report id). A trigger job uses the coalesce key
   `rating_trigger:<party_id>:<event>:<event_ref>`, so two events for one party are never merged into one job; the
   handler still takes the party's advisory lock (3.2 step 1). `event_ref` is never stored on a rating or a review.
2. **Evaluation** (`RatingService::rescore` with reason `trigger`): read the triggers of the live model whose `on`
   equals `event`, in id order. Evaluate each `when` (the shared Expr, CEL, as the `expr` anchors of 3.2 step 4) over
   `event` (the producer's payload fields) and the party's facts of this computation. No trigger holds: an ordinary
   re-score, and the job ends.
3. **Action** of the first trigger that holds (later ones that hold are listed in the audit payload by id):
   - `rescore`: the re-score of section 3.2, with `trigger_code` set (3.2 step 16).
   - `rescore_and_review`, `refresh_task`, `reclassify`, `pep_workflow`: the re-score, then an event review (3.4 rule 5:
     an open event review absorbs it) with `due_basis = trigger_sla` and `due_at` from `BusinessClock::add(now,
     review_within.value, BusinessUnit::WorkingDays)` on the bank calendar (7.17). `pep_workflow` uses the PEP checklist of the level's controls row.
   - `hold_and_escalate`, `block_and_escalate`: the same review with `escalated_at` set at once. AKR-TMS never writes
     to the CBS, so the hold or block is the bank's act, recorded in the review's outcome.
   Audit `rating.trigger_applied {party_id, trigger_code, action, review_id?}` (ids and codes only). The stored
   `trigger_code` is the `TR-nn` id, or `TRG_REVIEW_REQUESTED` for the suspicion-linked topics of 3.6 rule 3 and for a
   trigger whose `when` reads `party.on_exit_register`.
4. **New fact.** The facts of a computation gain `party.on_exit_register` (TRUE when the party has a `kyc.exit_register`
   row with status `recorded`). It is a suspicion-tier fact (the register is M-only, 3.6): it is never a factor, and it
   is read only by a trigger's `when`.
5. **Events this phase derives** (`rating_touch_fanout`, which already reads the batch's changed `account` rows; no
   outbox topic):
   - `account.reactivated` (TR-14): an account row of the batch with status `active` whose previous version (the row
     whose `recorded_to` equals its `recorded_from`) has status `dormant`.
   - `account.opened` (TR-24): an account whose first version is in the batch and whose `opened_on` is on or after the
     business date of the previous mapped batch of the same source table. The first batch of a source table derives
     none (the initial load is not a wave of new accounts).
   Each enqueues `rating_compute {reason: "trigger", event, event_ref: account id}` for the account's customer party.
   TR-24's `when` reads `party.on_exit_register`, so its reviews carry the neutral code.
6. **Producers in later phases** (the DR-375 pattern: the later phase writes the topic and registers this phase's
   handler, which enqueues the job of step 1): `screening.hit` (7.3), `profile.breach` (7.6), `outreach.failed`
   (TR-20, phase 19 section 7+), `visit.recorded` (TR-19, phase 22 section 7+). The other topics of the row keep the
   producers named in section 3.3 and in phases 17 to 19 and 26.
7. **Pending triggers.** `media.recorded` (TR-09; adverse media needs a source, ROADMAP Q12), `txn.new_corridor`
   (TR-12; the corridor feature is pending, as factor F2 is in 3.2) and `staff_link.recorded` (TR-25; no HR link is on
   a P0/P1 line) have no producer in this release. They stay in the shipped library and never fire. The model page
   lists them as pending triggers, the way it lists pending factors (DR-104).
8. Section 4.1: the `rating_compute` row gains the reason `trigger`, the fields `event` and `event_ref`, and the
   trigger coalesce key. Section 3.11 gains `rating.trigger_applied`.

### 7.8 The PEP list source in this release (gap item (ad); DR-406)
Source 1 of section 3.8 reads screening alerts with `list_kind = 'pep'`. In this release no list has kind `pep`: the
list codes are closed to six (DR-100, `pep` is a kind only), and a PEP data list needs a licensed source (ROADMAP Q12).
Source 1 therefore finds nothing, and `pep_list_sweep` runs only its second step (the CBS class) until a later change
adds a list code of kind `pep`. Candidates in this release come from the CBS class, manual entry and declarations. The
known-answer case of section 6 that expects a `source: pep_list` candidate is run only when such a list exists. Open
doubts 13 (latency of the PEP list path) is moot until then.

Adversarial pass (2026-10-10), run 1 (four-eyes and self-approval), from tools/agent/adv-run1.md; lead-verified (tools/agent/adv-run1-verdict.md).

### 7.9 The approver of a downward override (ADV-1-09, LOW; DR-407)
A phase 04 task has one role, so "`approve` role CHK or M" (section 3.5 step 5 and section 4.2) is shipped as: one step
`approve` role CHK, not the proposer, not the relationship owner, timeout `esc` M after 3 business days ->
`record_decision`. The bank may change the role by a `workflow_definition` change request.

Adversarial pass (2026-10-10), run 4 (clocks and timers on closed, merged or reopened items), from tools/agent/adv-run4.md; lead-verified (tools/agent/adv-run4-verdict.md).

### 7.10 An absorbed event review keeps the earlier deadline and its clock (ADV-4-02 MEDIUM, ADV-4-08 LOW; DR-410)
Section 3.4 rule 5 and the trigger actions: when an open event review absorbs a new event, `due_at` becomes the earlier
of the current `due_at` and the new event's due date (an absorb never extends a deadline). When it moves earlier, the
same transaction runs `TimerService::cancel_for_owner(tx, 'kyc_review', review_id, 'kyc_review_clock')` and schedules
`kyc_review_clock` at the new `due_at`. The handler `timer_kyc_review_clock` re-arms itself at `due_at` when `due_at` is
still later than now, and writes `escalated_at` and the audit `kyc_review.overdue` only when `escalated_at IS NULL`, so
an at-least-once re-run writes once. Test: an absorb that moves `due_at` from day 10 to day 3 escalates on day 3, once.

Adversarial pass (2026-10-10), run 6 (idempotency, audit chain and outbox replay), from tools/agent/adv-run6.md; lead-verified (tools/agent/adv-run6-verdict.md).

### 7.11 A trigger job applies once (ADV-6-05, MEDIUM; DR-412)
Section 7.7: the `rating_compute` handler finishes with `JobContext::complete_in(tx)` (phase 00 section 7+) in the
transaction that writes the re-score, the review and the audit `rating.trigger_applied`. A re-run after a lost
completion therefore finds the job taken or done and writes nothing, so a review closed in between is not followed by a
second review for the same event. `event_ref` is still never stored.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.12 record_decision for the three KYC workflows (V-15-01-d, HIGH; DR-413)
Phase 04 leaves `record_decision` to later phases and phase 18 registers it for `case` only, so this phase registers its own: `KycRecordDecisionEffect` (`TerminalEffect`, kind `record_decision`, resource kinds `change_request`, `kyc_review` and `exit_decision`; not deferrable). `run` branches on the resource kind of the instance:
- `change_request` (route `kyc_override_down`): it calls the phase 04 activation of the change request, as `activate_change` does, so `OverrideTarget::activate` runs in the approval transaction (section 3.5 step 6).
- `kyc_review` (`kyc_edd_signoff`): it runs section 3.4 step 7 sub-steps 7 to 9 with the approver as completer.
- `exit_decision`: it stores the approved outcome in the instance context and writes nothing else (section 3.4 step 8).
With it the three shipped definitions of section 4.2 publish in this phase.

### 7.13 The workflow subjects of kyc_review and exit_decision (V-15-06-a, V-15-01-b, HIGH; DR-413)
This phase registers one `WorkflowSubject` (phase 04) for each of its two resource kinds. Neither declares parties; `check_before_decision` keeps its default.
- `KycReviewWorkflowSubject`: `resource_kind` `kyc_review`, `attribute_prefix` `review`, attributes `kind` (text) and `level_at_open` (integer); the resource is `("kyc_review", review_id)` and the title `{ar: "مراجعة اعرف عميلك", en: "KYC review"}`. `start_context` freezes both, sets `maker_id` to the reviewer, and sets `classification` to `restricted` when the party's tier is `restricted` or `suspicion`, else `internal`. `visible_to` and `visibility_sql` apply the masking of section 3.6 to the review's party. `on_ended` changes nothing: the review completes only through `record_decision` (section 3.4 step 7).
- `ExitDecisionWorkflowSubject`: `resource_kind` `exit_decision`, `attribute_prefix` `exit`, no attributes. The instance is started with the resource `("exit_decision", review_id)`, and its title is `{ar: "قرار إنهاء العلاقة", en: "Exit decision"}`. `start_context` reads the review and its party from that id and freezes `review_id` (the completed review whose outcome `exit_candidate` opened the instance, section 3.4 step 7 sub-step 6) and `party_id`, sets `maker_id` to that review's reviewer and `classification` to `restricted`. `visible_to` and `visibility_sql`: role M only, as the exit register. `on_ended` changes nothing.
Section 3.4 step 8: the `review_id` of the `kyc.exit_register` row is read from the instance context, never from the body; `NewExitEntry` keeps its fields. Its `party_id` must equal the party frozen in the context: a different one is refused with 422 `E_VALIDATION` at `/party_id`, so an exit is never recorded against a party other than the one decided.

### 7.14 The exit register's repository (V-15-01-a, MEDIUM; DR-413)
Section 2 gains the trait `ExitRepository` for `kyc.exit_register`: `async fn insert(&self, tx: &mut Tx, e: &NewExitEntry, review_id: Uuid, by: Uuid) -> Result<ExitEntry, RepoError>;` (section 3.4 step 8; the unique `workflow_instance_id` gives the 409 of that step) and `async fn list(&self, conn: &mut PgConnection, page: Keyset) -> Result<Page<ExitEntry>, RepoError>;` (`GET /exit-register`). `ExitService` has two methods, `record(ctx, body: NewExitEntry)` and `list(ctx, page)`, which call them.

### 7.15 The country fan-out: page query, effective date and trigger (V-15-02-b, V-15-02-c, V-15-02-d, MEDIUM; DR-413)
Section 3.3 row 2.
- Section 2, the port `PartyFactsSource` gains `async fn parties_for_country(&self, conn: &mut PgConnection, iso2: &str, page: Keyset) -> Result<Page<Uuid>, RepoError>;`. It returns one keyset page of party ids in id order, sized by `rating.country_fanout_page`: parties with a current row (`recorded_to IS NULL`) whose `nationality` or `residence_country` is `iso2`, and the `owned_party_id` of each open `kyc.ownership_inputs` row (`valid_to IS NULL`) whose `owner_party_id` is a party with `residence_country` equal to `iso2`.
- `effective_at` of `rating_country_fanout` is the `created_at` of the outbox row the handler runs for. Phase 09 writes that row in the transaction that changes the level, so it is the moment the level took effect. The event payload does not change.
- The topic `ref.country_level_changed` is the DSL event `country_table.changed`, the `on` value of TR-10 (DSL.md C.5). TR-10's action is `rescore`, and the fan-out is that action: it opens no review and does not run the trigger path of section 7.7.

### 7.16 Response types of the coverage and analytics reads (V-15-07-a, V-15-08-a, LOW; DR-413)
`GET /api/v1/ownership/coverage` (section 7.4) answers `OwnershipCoverageView`: one row per current level with `parties_rated`, `parties_with_result`, `results_unknown`, `results_loops`, `results_depth_truncated` and `results_stale`, and no party id. `GET /api/v1/rating/overrides/analytics` (section 7.5) takes the query parameters `from` and `to` (dates; a window above 366 days is 422 `E_VALIDATION`) and answers `OverrideAnalyticsView` with `by_proposer`, `by_branch` and `overall`, in the order of the steps of section 7.5. Both types join the utoipa list of section 2.

### 7.17 The due instant of an event review (V-15-09-b, LOW; DR-413)
Section 7.7 step 3: `review_within` is a number of business days (DSL.md C.5). The call is `BusinessClock::add(now, review_within.value, BusinessUnit::WorkingDays)` on the bank calendar, and `due_at` is the `fire_at` of the returned `ClockResult`.

### 7.18 The absorb statement moves the deadline (V-15-11-a, MEDIUM; DR-413)
Section 2 `open_or_join`: the insert names the table `AS r`, and its conflict update sets `due_at` to the earlier of the two dates for an event review, as section 7.10 requires; a `date_unknown` review keeps `due_at` NULL (the CHECK of section 1.2), and the other kinds keep their `due_at`. The method returns `(KycReview, bool, Option<DateTime<Utc>>)`: the third value is the `due_at` the open row had before the statement, and the new `due_at` is in the returned review. RETURNING shows only the updated row in PostgreSQL 17, so the old value is read by a leading CTE of the same statement (`WITH prev AS (SELECT due_at FROM kyc.kyc_reviews WHERE party_id = $1 AND kind = $2 AND status IN ('open', 'in_progress', 'awaiting_answer'))`) and returned as `(SELECT due_at FROM prev)`. The caller moves the timer as section 7.10 says when the date moved. No audit kind is added: `rating.trigger_applied` (section 7.7 step 3) names the review.

## Decisions

Recorded in docs/plan/research.md: DR-103 (suspicion-derived factors A5 (STR and exit parts), F3, F5 and F7 are
withheld: never scored into the stored rating, so the rating, the feed, the segment overlay and the screening modifier
carry no suspicion data; suspicion-derived risk reaches the rating only as an upward override by a person on the
case's access list with the shared code `OVR_UPWARD_COMPLIANCE_JUDGEMENT`, whose text M alone reads; suspicion-linked
triggers store the neutral `TRG_REVIEW_REQUESTED`), DR-104 (the engine contract: the inherent score normalised as
`(1 - p) x sum(w_b/100 x s_b) + p x max(s_b)`, as the research worked example requires; a block with no applicable
factor is dropped and the others renormalise; pending factors are excluded and listed on the model, not a degraded
reason; a history row only when the computed state changes, with `kyc.rating_current` the one pointer every reader
uses and the engine the only writer; overrides upward direct (KYC or M) and downward by change request
`kyc_override_down` with evidence, one level and four eyes, both expiring within twelve months, never crossing a floor
or the cap; a model's volume effect is the change in the number of parties in the high CBI label from its shadow run,
and `unknown` is never approved; the first live version is installed by `rating seed`), DR-105 (masking and
conflicts: `customer` becomes a masked kind, with `kyc_review`, `override` and `pep_entry`; suspicion tier is M-only
until phase 18, including phase 13's `profile.read`; the relationship-conflict check ships here from
`admin.users.staff_id` and `canon.customer.rm_id`; the exit register is M-only, written once per `exit_decision` by
`POST /exit-register`; the feed and its sealed export exclude suspicion-tier parties) and DR-106 (PEP and ownership: a
decided `pep` list match is a CDD fact, so it creates a KYC-visible candidate with no alert id and never a status; a
tier comes only from a `pep_entry` change request, P3 highest; relatives are suggested from canonical family links and
count only when a person confirms them; the PEP pair-memory cap is 24 months; beneficial ownership uses the CBS share
when held, else the bank-entered share, and a missing share is unknown, never clear).

Lead corrections at review (2026-10-09): the downward-override CHECK refused a rejected or withdrawn proposal (it
required an approver for every status but `proposed`); `kyc.party_pep_state` showed a former PEP whose persistence had
passed as a candidate (now `none`); `kyc.kyc_reviews` lacked UPDATE grants on `due_basis` and `date_unknown`, which the
CHECK ties to `due_at`; the exit register took `OVR_` codes (now family `decision`, `DEC_`) and had two writers (the
terminal effect and the route; now the route only, unique per instance); the PEP route choice was described as by
volume effect (it is by class); the Cedar bundle test contradicted the policy on CFG validation; the suspicion forbid
now names phase 13's `profile.read`; the requirement list gained K-10, K-13 to K-16, K-18, K-21 and K-22, which the
phase carries. Settled doubts: the completeness cap wording of MODULES M10 and DSL.md C.4 agree ("caps the best level"
means the best reachable level is 3); DSL.md C.8 now states the normalised formula; the six added routes, the three
shipped workflow definitions (phase 04 named kyc_review and exit_decision as phase 15's) and the route-map rows are
approved; roadmap row 15 no longer lists `kyc_document`.

## Open doubts (for gap pass 1 and the bank)

1. **Who sets the suspicion tier.** `canon.party.sensitivity_tier` is written by the ingest only (phase 10), and a
   party becomes suspicion tier through a case or a report (phases 17 and 18). No phase yet defines that write; the
   masking of this phase depends on it. Target: phase 17 or 18, gap pass 1.
2. **Relationship conflicts need the staff link.** A user without `staff_id` is not checked (3.9). ADM must link every
   staff login; the phase 03 user form needs the field. A claim refused for conflict needs a fallback assignee (M or a
   named pool).
3. **Tier scheme and persistence.** `P1` to `P3` are placeholders aligned to FL-03. The persistence months per tier
   (`pep.persistence_months`) and the relative input `pep.related_tier` are named in DSL.md C.3 by this review; the bank
   and the MLRO supply the values.
4. **Upward override roles.** DSL.md C.6 allows A2 to raise a level; API.md scope `kyc.work` excludes A2. This phase
   follows the API (KYC and M); an A2 asks M. Confirm with the bank.
5. **Withdrawal of an active override.** No route: an active override ends at expiry only.
6. **Evidence reads.** Phase 18's `GET /evidence/{id}` is `queue.work`; this phase's evidence routes are `kyc.work`.
   Confirm two routes over the same vault, and that phase 02 registers the owner kinds of 4.3.
7. **The bank's channel for the feed.** A personal access token cannot hold `kyc.work` (API.md s.2), so the sealed export
   file of 3.6 is the channel. The cursor of `GET /rating/feed` reads an identity column written by up to four
   concurrent jobs; a reader can pass a row whose transaction commits late. Adversarial pass: settle the read (a settle
   delay or a commit-ordered cursor).
8. **Feed rows already written.** A row written before a party became suspicion tier stays in the history; the read
   excludes the party but the row is not recalled.
9. **Retention of name-only PEP candidates.** They have no party for the owner clock; an age clock on `created_at` is the
   likely answer. The relationship-end resolution belongs to phase 23.
10. **Two factor inputs have no canonical column.** C2 (registration country) and C4 (branch border status) stay pending
    until phase 10 maps them.
11. **K2, K3 and K4 have no design specs.** DESIGN-SYSTEM.md 8.3 specifies only K1; section 5 is the proposal.
12. **Declared versus actual.** F1 reads the ratio of phase 13 actuals to the declared figure, never the stored deviation.
13. **Latency of the PEP list path.** Phase 14 has no outbox topic for decisions; `pep_list_sweep` polls every ten minutes.
14. **The CBS class for PEP candidates.** `kyc.pep_cbs_class_values` names `canon.customer.segment` values. Confirm the
    bank carries its political class there (DR-38).
15. **Trigger identifiers.** Only TR-10 and TR-15 have DSL ids among the triggers wired here; the `TRG_*` names should
    join the trigger library (research 26 s.6c).
16. **Sentinel thresholds.** Review dates before 1990-01-01 or more than 100 years ahead are this phase's values.
17. **Light yearly check for all.** Applied to every current customer record; a dormant or closed customer may not need it.
18. **Ownership threshold.** 20 per cent is MODULES M10's default; the bank confirms (`ownership.threshold_pct`, governed).
19. **K-IDs placed by gap pass 1 (DR-395, DR-396).** K-12 phase 19; K-19, K-25 and K-36 here (section 7+ and the controls
    matrix); K-20 phase 16; K-23 phase 21; K-31 phase 12; K-32 phase 10; K-34 here; K-38 phase 24; K-39, K-40 and K-41
    are not carried. Closed.

## What This Phase Does NOT Touch

- No CBS connection and no `akr-ingestd` job; `canon` is read only, and no canonical row is written, including
  `canon.party.pep_status` (the PEP state is the view `kyc.party_pep_state`).
- No scenario, feature, segment statistic or alert logic; the rating feeds the segment risk overlay (phase 13) and the
  screening modifiers (phase 14) as data only.
- No case, alert, report, filing, RFI, watch, network or branch-window data (phases 17 to 22); the four suspicion-derived
  factors are withheld (DR-103).
- No freeze, sanctions decision or list change (phase 14 and phase 20); the PEP list outcomes are read, not decided here.
- No new step-up action class, no new problem code (API.md s.6), no new role, no new network zone.
- No change to the phase 00 to 14 tables beyond the additions of section 4.4 (`admin.users.staff_id`, the owner kinds,
  the change-request target registrations, the shipped definitions and the route-map rows); no change to a `canon` table
  or grant; `canon.kyc_document` is read, not recreated.
- No withdrawal route for overrides, no emergency change route for any kind of this phase, no revert.
- No screen beyond K1 to K4 and no design system change; the rating model studio (D-series) is not built here.
- No backfill of historical ratings before the seed: the first rating is computed on the first run after the seed.
- No DSL change beyond the wording the lead made at review (DSL.md C.2 withheld factors, C.3 PEP inputs, C.8 normalised
  formula).
