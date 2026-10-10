# Phase 13: Features & Segments

**Goal:** The feature catalogue (versioned definitions over nine closed expression kinds, month-end values per customer,
account, counterparty and branch, computed in akr-worker only for closed and fresh business days), the segment register
(static and risk-overlay definitions by change request, cluster segments from deterministic peer builds activated by
change request), the robust segment statistics with the 200-member floor and merge-upward rule, and the expected-profile
store (actual against declared figures, own history and peers, held for the alert engine and never shown with its
deviation), all served as data to the scenario engine of phase 16 and to the party page without any signal that a
customer is under suspicion.

**Dependencies:** Phase 11 (`data_as_of`, freshness states, blind periods, watermarks), Phase 12 (coverage ledger and
dark units, exclusions), and through them Phase 10 (the canonical tables and `CanonicalCatalogue`) and Phase 09
(business calendar and FX rates). Phase 04 (`ChangeTargetRegistry`, route classes), Phase 02 (partition registry,
access events), Phase 03 (purpose codes, resource adapters) and Phase 06 (`AttributeCatalogue`, `coverage.maintained_fields`).
Phase 15 later supplies the declared KYC profile and the rating source; both are ports with null implementations here.

**Macro-phase:** P1
**Complexity:** M (11 routes; the feature engine, the segment assignment and the peer clustering carry the weight)
**Binaries touched:** akr-api (routers `segments` (mounted under `/ref`), `feature_definitions`, `peer_groups`,
`profiles`; change targets `segment`, `feature_definition` and `peer_build` registered in process) | akr-worker (jobs
`feature_compute`, `segment_assign`, `segment_stats_refresh`, `peer_group_build`, `profile_refresh`) | akr-ctl (`features
seed`, `features verify`). akr-ingestd and akr-assess unchanged; nothing in this phase reaches a CBS.
**Requirements carried:** T-07 (three-level segmentation: static, behavioural cluster, risk overlay; sections 3.3 and
3.5), S-10 (a missing value falls back to the segment statistic and marks the customer degraded;
unknown is never read as low; section 3.6), S-12 (the robust baseline service: percentiles and median-MAD by segment,
currency and period, a minimum segment size of 200 with merge upward; sections 3.3 to 3.5).
- Gap pass 1 (coverage), partly implemented: S-17 (public-salary accounts can form a static peer segment on `account.type`; the exclusion of the payroll pool account is a static exclusion of the scenario document (docs/DSL.md A.7; phase 16 section 3.1 rule 13) with an active reason code of family `exclusion`, `EXC_POOL_ACCOUNT` in the DSL example, and the payroll and pension batch tags and the government-source flag are P2, because the plan holds no CBS field for them), S-36 (the `dormant_days` feature); SP-46 (protected attributes blocked as inputs); L-49 (segment assignment from observed attributes); T-08 (deviations on monthly profiles against own history and peers), T-09 (the segment membership history), T-17 (month-end feature values keyed by as-of date), T-18 (robust median-MAD segment statistics), T-29 (the counterparty resolver for transfer counterparties with analyst confirmations, section 7.5), T-42 (daily event-time features compiled by the same compiler), T-65 (cash-share features and percentile statistics by segment, currency and period); VB-14 (the per-customer baseline store, offline: month-end feature values, monthly profiles and segment statistics, sections 3.2, 3.4 and 3.6; the online use within a latency budget is the requirement's Phase 2 and is not built).
- Gap pass 1 (coverage), partly implemented here: VB-05 (features keyed by counterparty for counterparties seen only in transactions, section 7+; counterparty profiles are P2).
- Gap pass 1 (coverage), not carried: VB-47 (digital-channel signals, DR-397).

---

## 1. SQLx Migration Changes

Four migrations, `0040_feat_definitions_segments` to `0043_feat_settings_schedules`; `0044` was not used here and now holds the counterparty links of section 7+ (the shipped
catalogue is installed from a signed file by `akr-ctl features seed`, section 3.1, as phase 06 installs its register).
Schema `feat` exists since phase 00 (DATA-MODEL s.8). The DATA-MODEL s.8 table list is kept; this phase adds the
columns and the two tables that the design needs and DATA-MODEL does not name, each marked "extends DATA-MODEL s.8":
`feat.feature_runs` (the as_of ledger), `feat.segment_memberships` (the assignment history) and `currency` in the keys
of `feature_values`, `segment_stats` and `profiles`. Identifiers avoid the PostgreSQL reserved words (DR-20);
`window` is named `window_spec`.

Eight tables and no outbox topic: `feature_definitions` (R9), `feature_runs` (R4), `feature_values` (R4, monthly
partitions), `segments` (R9), `segment_memberships` (R4), `segment_stats` (R4, monthly partitions), `peer_groups` (R4),
`profiles` (R4, monthly partitions).

```sql
-- migration 0040_feat_definitions_segments.sql
-- Phase 13. Feature definitions (versioned, immutable content), peer-group builds, segments and the segment
-- membership history. Non-partitioned. Ids are UUIDv7 from the application (no database default).

CREATE TABLE IF NOT EXISTS feat.feature_definitions (
    id                  UUID PRIMARY KEY,
    feature_key         TEXT NOT NULL CHECK (feature_key ~ '^[a-z][a-z0-9_]{2,62}$'),
    version             INT NOT NULL CHECK (version >= 1),
    entity              TEXT NOT NULL CHECK (entity IN ('account', 'customer', 'counterparty', 'branch')),
    name                JSONB NOT NULL CHECK (name ? 'ar' AND name ? 'en'),
    value_kind          TEXT NOT NULL CHECK (value_kind IN ('amount', 'count', 'ratio', 'days', 'flag', 'statistic')),
    window_spec         JSONB NOT NULL CHECK (jsonb_typeof(window_spec) = 'object'),
    expression          JSONB NOT NULL CHECK (jsonb_typeof(expression) = 'object'
                                              AND expression ->> 'kind' IN ('count', 'sum', 'avg', 'max',
                                                  'distinct_count', 'ratio', 'days_since', 'exists', 'segment_stat')),
    requires            JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(requires) = 'array'),  -- derived
    currency_mode       TEXT NOT NULL CHECK (currency_mode IN ('per_currency', 'usd_equivalent', 'none')),
    state               TEXT NOT NULL CHECK (state IN ('active', 'retired')),
    change_request_id   UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    created_by          UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    activated_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    retired_at          TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_feature_version UNIQUE (feature_key, version),
    CHECK ((state = 'retired') = (retired_at IS NOT NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_feature_one_active ON feat.feature_definitions (feature_key) WHERE state = 'active';
CREATE INDEX IF NOT EXISTS idx_feature_entity_state ON feat.feature_definitions (entity, state);
CREATE INDEX IF NOT EXISTS idx_feature_change_request ON feat.feature_definitions (change_request_id)
    WHERE change_request_id IS NOT NULL;

-- Built per currency from the feature month-end values. A build is a candidate until its change request is approved;
-- one build per currency and month-end (a rebuild for the same month-end returns the existing row).
CREATE TABLE IF NOT EXISTS feat.peer_groups (
    id                  UUID PRIMARY KEY,
    currency            CHAR(3) NOT NULL CHECK (currency ~ '^[A-Z]{3}$'),
    build_no            INT NOT NULL CHECK (build_no >= 1),
    as_of               DATE NOT NULL,
    algorithm           TEXT NOT NULL CHECK (algorithm = 'kmeans_pp_v1'),
    k                   INT NOT NULL CHECK (k >= 1),
    definition          JSONB NOT NULL CHECK (jsonb_typeof(definition) = 'object'),
    centroids           JSONB NOT NULL CHECK (jsonb_typeof(centroids) = 'array'),
    members_count       INT NOT NULL CHECK (members_count >= 0),
    wcss                NUMERIC(28,6) CHECK (wcss IS NULL OR wcss >= 0),
    stability           NUMERIC(9,6) CHECK (stability IS NULL OR stability BETWEEN 0 AND 1),
    state               TEXT NOT NULL CHECK (state IN ('candidate', 'activating', 'active', 'superseded', 'rejected')),
    change_request_id   UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    requested_by        UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    computed_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    activated_at        TIMESTAMPTZ,
    version             INT NOT NULL DEFAULT 1,
    CONSTRAINT uq_peer_build_no UNIQUE (currency, build_no),
    CONSTRAINT uq_peer_build_asof UNIQUE (currency, as_of),
    CHECK ((state IN ('activating', 'active', 'superseded')) = (activated_at IS NOT NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_peer_live ON feat.peer_groups (currency) WHERE state IN ('activating', 'active');
CREATE INDEX IF NOT EXISTS idx_peer_change_request ON feat.peer_groups (change_request_id) WHERE change_request_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_peer_requested_by ON feat.peer_groups (requested_by);

-- Static and risk-overlay segments come from change requests; cluster segments are written by a peer build (level
-- 'cluster' carries the currency and the build). min_size may never be below the S-12 floor of 200 (setting floor and
-- this CHECK agree). merged_into is the effective merge target computed by the assignment job, never typed.
CREATE TABLE IF NOT EXISTS feat.segments (
    id                  UUID PRIMARY KEY,
    key                 TEXT NOT NULL UNIQUE CHECK (key ~ '^[a-z][a-z0-9_.]{1,80}$'),
    name                JSONB NOT NULL CHECK (name ? 'ar' AND name ? 'en'),
    level               TEXT NOT NULL CHECK (level IN ('static', 'cluster', 'risk_overlay')),
    definition          JSONB NOT NULL CHECK (jsonb_typeof(definition) = 'object'),
    sort_order          INT NOT NULL DEFAULT 0,
    min_size            INT NOT NULL DEFAULT 200 CHECK (min_size >= 200),
    members_count       INT NOT NULL DEFAULT 0 CHECK (members_count >= 0),
    merged_into         UUID REFERENCES feat.segments(id) ON DELETE RESTRICT,
    currency            CHAR(3) CHECK (currency IS NULL OR currency ~ '^[A-Z]{3}$'),
    peer_group_id       UUID REFERENCES feat.peer_groups(id) ON DELETE RESTRICT,
    status              TEXT NOT NULL CHECK (status IN ('candidate', 'active', 'superseded', 'retired')),
    change_request_id   UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    version             INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    created_by          UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (merged_into IS NULL OR merged_into <> id),
    CHECK ((level = 'cluster') = (peer_group_id IS NOT NULL)),
    CHECK (level <> 'cluster' OR currency IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_segments_level_status ON feat.segments (level, status);
CREATE INDEX IF NOT EXISTS idx_segments_merged_into ON feat.segments (merged_into) WHERE merged_into IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_segments_change_request ON feat.segments (change_request_id) WHERE change_request_id IS NOT NULL;

-- Assignment history (extends DATA-MODEL s.8). One open row per customer and level; a change closes the row at the
-- as_of date and opens a new one, so statistics can join the membership that held on each month-end.
CREATE TABLE IF NOT EXISTS feat.segment_memberships (
    customer_id         UUID NOT NULL,                       -- canon.customer logical id (no foreign key: bi-temporal)
    level               TEXT NOT NULL CHECK (level IN ('static', 'cluster', 'risk_overlay')),
    segment_id          UUID NOT NULL REFERENCES feat.segments(id) ON DELETE RESTRICT,
    valid_from          DATE NOT NULL,
    valid_to            DATE,
    assigned_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (customer_id, level, valid_from),
    CHECK (valid_to IS NULL OR valid_to > valid_from)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_membership_open ON feat.segment_memberships (customer_id, level) WHERE valid_to IS NULL;
CREATE INDEX IF NOT EXISTS idx_membership_segment ON feat.segment_memberships (segment_id, valid_from);

-- migration 0041_feat_partitioned_facts.sql
-- Phase 13. The as_of ledger, the monthly fact tables and the partition factory (SECURITY DEFINER, as phase 02
-- audit.create_partition and phase 10 canon.create_partition).

-- One row per feature and closed month-end (extends DATA-MODEL s.8). 'computed' means that an absent value row is a
-- true zero (count, sum) or not applicable (other kinds); 'not_ready' and 'dark' mean that the month is unknown and
-- consumers must show it as unknown, never as zero. 'basis' names the excluded and blocking units.
CREATE TABLE IF NOT EXISTS feat.feature_runs (
    as_of               DATE NOT NULL,
    feature_id          UUID NOT NULL REFERENCES feat.feature_definitions(id) ON DELETE RESTRICT,
    state               TEXT NOT NULL CHECK (state IN ('computed', 'not_ready', 'dark')),
    reason              TEXT CHECK (reason IN ('not_closed', 'behind_business_date', 'stale', 'halted', 'missing_rate',
                                               'blind_period', 'dark_unit', 'no_postings_since')),
    basis               JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(basis) = 'object'),
    rows_written        INT NOT NULL DEFAULT 0 CHECK (rows_written >= 0),
    computed_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (as_of, feature_id),
    CHECK ((state = 'computed') = (reason IS NULL))
);
CREATE INDEX IF NOT EXISTS idx_feature_runs_feature ON feat.feature_runs (feature_id, as_of);

CREATE TABLE IF NOT EXISTS feat.feature_values (
    entity              TEXT NOT NULL CHECK (entity IN ('account', 'customer', 'counterparty', 'branch')),
    entity_id           UUID NOT NULL,                       -- logical id (no foreign key: canon is bi-temporal)
    feature_id          UUID NOT NULL REFERENCES feat.feature_definitions(id) ON DELETE RESTRICT,
    currency            CHAR(3) NOT NULL CHECK (currency ~ '^[A-Z]{3}$'),   -- 'XXX' for currency-free kinds
    as_of               DATE NOT NULL,                       -- a closed month-end (bank calendar)
    value               NUMERIC(28,6),
    status              TEXT NOT NULL CHECK (status IN ('value', 'no_history', 'beyond_window')),
    basis               JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(basis) = 'object'),
    computed_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (entity, entity_id, feature_id, currency, as_of),
    CHECK ((status = 'value') = (value IS NOT NULL))
) PARTITION BY RANGE (as_of);
CREATE INDEX IF NOT EXISTS idx_feature_values_feature ON feat.feature_values (feature_id, as_of);
CREATE INDEX IF NOT EXISTS brin_feature_values_asof ON feat.feature_values USING BRIN (as_of);

-- Written only with at least `segments.min_size` members (never below 200, the CHECK). Suppressed populations have
-- no row at all.
CREATE TABLE IF NOT EXISTS feat.segment_stats (
    segment_id          UUID NOT NULL REFERENCES feat.segments(id) ON DELETE RESTRICT,
    feature_id          UUID NOT NULL REFERENCES feat.feature_definitions(id) ON DELETE RESTRICT,
    currency            CHAR(3) NOT NULL CHECK (currency ~ '^[A-Z]{3}$'),
    period              DATE NOT NULL,                       -- first day of the month
    as_of               DATE NOT NULL,                       -- the month-end the values were read at
    n                   INT NOT NULL CHECK (n >= 200),
    p50                 NUMERIC(28,6) NOT NULL,
    p95                 NUMERIC(28,6) NOT NULL,
    p99                 NUMERIC(28,6) NOT NULL,
    mad                 NUMERIC(28,6) NOT NULL CHECK (mad >= 0),
    computed_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (segment_id, feature_id, currency, period)
) PARTITION BY RANGE (period);
CREATE INDEX IF NOT EXISTS idx_segment_stats_feature ON feat.segment_stats (feature_id, period);

-- Derived monthly profile per customer record and currency; rebuilt per period in one transaction.
CREATE TABLE IF NOT EXISTS feat.profiles (
    customer_id         UUID NOT NULL,                       -- canon.customer logical id (no foreign key)
    period              DATE NOT NULL,                       -- first day of the month
    currency            CHAR(3) NOT NULL CHECK (currency ~ '^[A-Z]{3}$'),
    as_of               DATE NOT NULL,
    status              TEXT NOT NULL CHECK (status IN ('complete', 'degraded')),
    segment_id          UUID REFERENCES feat.segments(id) ON DELETE RESTRICT,
    expected            JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(expected) = 'object'),
    actual              JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(actual) = 'object'),
    deviation           JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(deviation) = 'object'),
    computed_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (customer_id, period, currency)
) PARTITION BY RANGE (period);
CREATE INDEX IF NOT EXISTS idx_profiles_segment ON feat.profiles (segment_id, period) WHERE segment_id IS NOT NULL;

-- The partition factory. Creates one month of one allow-listed feat table. Bounds are dates (both partition keys are
-- DATE). The advisory lock key is fixed to this factory. There is no DEFAULT partition: a missing month fails loudly
-- and the writer calls the factory once and retries (phase 10 pattern).
CREATE OR REPLACE FUNCTION feat.create_partition(p_table TEXT, p_year INT, p_month INT) RETURNS TEXT
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog
AS $$
DECLARE
    v_from DATE;
    v_to   DATE;
    v_name TEXT;
BEGIN
    IF p_table NOT IN ('feature_values', 'segment_stats', 'profiles') THEN
        RAISE EXCEPTION 'feat.create_partition: % is not a partitioned feat table', p_table;
    END IF;
    IF p_year NOT BETWEEN 2000 AND 2200 OR p_month NOT BETWEEN 1 AND 12 THEN
        RAISE EXCEPTION 'feat.create_partition: month %-% out of range', p_year, p_month;
    END IF;
    PERFORM pg_catalog.pg_advisory_xact_lock(7307418562110395012);   -- fixed key: 'akr.feat.partition_factory'
    v_from := pg_catalog.make_date(p_year, p_month, 1);
    v_to   := (v_from + interval '1 month')::date;
    v_name := pg_catalog.format('%s_%s_%s', p_table, p_year, pg_catalog.lpad(p_month::text, 2, '0'));
    IF NOT EXISTS (SELECT 1 FROM pg_catalog.pg_class c
                   JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace
                   WHERE n.nspname = 'feat' AND c.relname = v_name) THEN
        EXECUTE pg_catalog.format('CREATE TABLE feat.%I PARTITION OF feat.%I FOR VALUES FROM (%L) TO (%L)',
                                  v_name, p_table, v_from::text, v_to::text);
    END IF;
    RETURN v_name;
END $$;
REVOKE ALL ON FUNCTION feat.create_partition(TEXT, INT, INT) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION feat.create_partition(TEXT, INT, INT) TO akr_app;

-- Seed partitions: the current month and the three ahead (audit.partition_ahead_months), through the factory.
DO $$
DECLARE
    v_t TEXT;
    v_m DATE;
    i   INT;
BEGIN
    FOREACH v_t IN ARRAY ARRAY['feature_values', 'segment_stats', 'profiles'] LOOP
        FOR i IN 0..3 LOOP
            v_m := (date_trunc('month', now()) + make_interval(months => i))::date;
            PERFORM feat.create_partition(v_t, EXTRACT(YEAR FROM v_m)::int, EXTRACT(MONTH FROM v_m)::int);
        END LOOP;
    END LOOP;
END $$;
```

```sql
-- migration 0042_feat_guards_grants.sql
-- Phase 13. Content guards (under the grants), grants (DATA-MODEL s.21), column classification and retention.

CREATE OR REPLACE FUNCTION feat.definition_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'feature definitions are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF (NEW.feature_key, NEW.version, NEW.entity, NEW.value_kind, NEW.window_spec, NEW.expression, NEW.requires,
        NEW.currency_mode, NEW.name) IS DISTINCT FROM (OLD.feature_key, OLD.version, OLD.entity, OLD.value_kind,
        OLD.window_spec, OLD.expression, OLD.requires, OLD.currency_mode, OLD.name) THEN
        RAISE EXCEPTION 'feature definition content is immutable; propose a new version' USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_feature_definition_guard ON feat.feature_definitions;
CREATE TRIGGER trg_feature_definition_guard BEFORE UPDATE OR DELETE ON feat.feature_definitions
    FOR EACH ROW EXECUTE FUNCTION feat.definition_guard();

CREATE OR REPLACE FUNCTION feat.peer_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'peer builds are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF (NEW.currency, NEW.build_no, NEW.as_of, NEW.algorithm, NEW.k, NEW.definition, NEW.centroids, NEW.members_count,
        NEW.wcss, NEW.requested_by) IS DISTINCT FROM (OLD.currency, OLD.build_no, OLD.as_of, OLD.algorithm, OLD.k,
        OLD.definition, OLD.centroids, OLD.members_count, OLD.wcss, OLD.requested_by) THEN
        RAISE EXCEPTION 'peer build content is immutable' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.state IS DISTINCT FROM OLD.state AND NOT (
           (OLD.state = 'candidate'  AND NEW.state IN ('activating', 'rejected'))
        OR (OLD.state = 'activating' AND NEW.state = 'active')
        OR (OLD.state = 'active'     AND NEW.state = 'superseded')) THEN
        RAISE EXCEPTION 'illegal peer build transition % -> %', OLD.state, NEW.state USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_peer_group_guard ON feat.peer_groups;
CREATE TRIGGER trg_peer_group_guard BEFORE UPDATE OR DELETE ON feat.peer_groups
    FOR EACH ROW EXECUTE FUNCTION feat.peer_guard();

-- akr_app (api and worker): no DELETE on definitions, runs, values, segments, memberships or peer builds. DELETE is
-- granted on the two derived tables only, which are rebuilt per period inside one transaction.
GRANT SELECT, INSERT ON feat.feature_definitions TO akr_app;
GRANT UPDATE (state, retired_at) ON feat.feature_definitions TO akr_app;
GRANT SELECT, INSERT ON feat.feature_runs TO akr_app;
GRANT UPDATE (state, reason, basis, rows_written, computed_at) ON feat.feature_runs TO akr_app;
GRANT SELECT, INSERT ON feat.feature_values TO akr_app;
GRANT UPDATE (value, status, basis, computed_at) ON feat.feature_values TO akr_app;   -- recompute on changed basis
GRANT SELECT, INSERT ON feat.segments TO akr_app;
GRANT UPDATE (name, definition, sort_order, min_size, members_count, merged_into, peer_group_id, status,
              change_request_id, version, updated_at) ON feat.segments TO akr_app;
GRANT SELECT, INSERT ON feat.segment_memberships TO akr_app;
GRANT UPDATE (valid_to) ON feat.segment_memberships TO akr_app;
GRANT SELECT, INSERT, DELETE ON feat.segment_stats TO akr_app;
GRANT UPDATE (as_of, n, p50, p95, p99, mad, computed_at) ON feat.segment_stats TO akr_app;
GRANT SELECT, INSERT ON feat.peer_groups TO akr_app;
GRANT UPDATE (state, stability, activated_at, change_request_id, version) ON feat.peer_groups TO akr_app;
GRANT SELECT, INSERT, DELETE ON feat.profiles TO akr_app;
GRANT SELECT ON feat.segment_stats, feat.peer_groups, feat.feature_runs, feat.feature_definitions, feat.segments TO akr_readonly;
GRANT SELECT ON feat.feature_definitions, feat.feature_values, feat.feature_runs, feat.segments, feat.segment_stats
    TO akr_assess;
-- akr_readonly reads definitions, runs, segments, statistics and peer builds. It does not read feature values,
-- memberships or profiles: those are customer-level and are read through the restricted profile route only.
-- akr_ingest has no privilege on feat.

-- Classification (DATA-MODEL conventions; phase 00 classes). detection-logic: definitions, segment definitions,
-- statistics, centroids, profile expected and deviation.
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'feat', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('feature_definitions', 'internal', ARRAY['id','feature_key','version','entity','name','value_kind','window_spec',
        'currency_mode','state','change_request_id','created_by','activated_at','retired_at','created_at']),
  ('feature_definitions', 'detection-logic', ARRAY['expression','requires']),
  ('feature_runs', 'internal', ARRAY['as_of','feature_id','state','reason','basis','rows_written','computed_at']),
  ('feature_values', 'personal', ARRAY['entity_id']),
  ('feature_values', 'internal', ARRAY['entity','feature_id','currency','as_of','status','basis','computed_at']),
  ('feature_values', 'financial', ARRAY['value']),
  ('segments', 'internal', ARRAY['id','key','name','level','sort_order','min_size','members_count','merged_into',
        'currency','peer_group_id','status','change_request_id','version','created_by','created_at','updated_at']),
  ('segments', 'detection-logic', ARRAY['definition']),
  ('segment_memberships', 'personal', ARRAY['customer_id']),
  ('segment_memberships', 'internal', ARRAY['level','segment_id','valid_from','valid_to','assigned_at']),
  ('segment_stats', 'internal', ARRAY['segment_id','feature_id','currency','period','as_of','computed_at']),
  ('segment_stats', 'detection-logic', ARRAY['n','p50','p95','p99','mad']),
  ('peer_groups', 'internal', ARRAY['id','currency','build_no','as_of','algorithm','k','members_count','stability',
        'state','change_request_id','requested_by','computed_at','activated_at','version']),
  ('peer_groups', 'detection-logic', ARRAY['definition','centroids','wcss']),
  ('profiles', 'personal', ARRAY['customer_id']),
  ('profiles', 'internal', ARRAY['period','currency','as_of','status','segment_id','computed_at']),
  ('profiles', 'financial', ARRAY['actual']),
  ('profiles', 'detection-logic', ARRAY['expected','deviation'])
) AS v(t, cls, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('feat', 'feature_definitions', 'R9', 'retirement', NULL,        'Feature versions: the record each scenario version pins'),
 ('feat', 'feature_runs',        'R4', 'age',        'as_of',     'Which months are computed, not ready or dark'),
 ('feat', 'feature_values',      'R4', 'partition',  'as_of',     'Month-end features; 3 years or the longest look-back'),
 ('feat', 'segments',            'R9', 'retirement', NULL,        'Segment register; superseded cluster segments kept'),
 ('feat', 'segment_memberships', 'R4', 'age',        'valid_from', 'Assignment history behind every statistic'),
 ('feat', 'segment_stats',       'R4', 'partition',  'period',    'Robust statistics per period'),
 ('feat', 'peer_groups',         'R4', 'age',        'computed_at', 'Peer builds: the model behind each cluster segment'),
 ('feat', 'profiles',            'R4', 'partition',  'period',    'Monthly profiles; the look-back of own-history comparators')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

```sql
-- migration 0043_feat_settings_schedules.sql
-- Phase 13. Settings and schedules. Floors only where governed = TRUE (phase 00 CHECK). Cron values are six-field.

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('segments.min_size', '200', 'integer',
  '{"min": 200, "basis": "S-12: minimum segment size of 200, merged upward (research file 27 B28)"}', TRUE,
  '{"ar": "أدنى عدد أعضاء للقطاع", "en": "Minimum members per segment (statistics and assignment)"}'),
 ('features.max_window_days', '400', 'integer',
  '{"min": 31, "basis": "DSL.md A.6.1 hot window of the canonical store; proposed value, see Open doubts"}', TRUE,
  '{"ar": "أقصى نافذة للميزات بالأيام", "en": "Longest feature window in days (the hot window)"}'),
 ('features.catchup_months', '3', 'integer', NULL, FALSE,
  '{"ar": "عدد أشهر اللحاق عند التأخر", "en": "Month-ends a feature run may catch up after a delay"}'),
 ('profile.min_history_months', '3', 'integer',
  '{"min": 3, "basis": "DSL.md A.6.3 min_history of three months (proposed floor)"}', TRUE,
  '{"ar": "أدنى تاريخ للملف الشخصي بالأشهر", "en": "Months of complete history before own-history comparators"}'),
 ('profile.lookback_months', '12', 'integer', NULL, FALSE,
  '{"ar": "مدة تاريخ الملف الشخصي بالأشهر", "en": "Months of history used for the own-history comparator"}'),
 ('peer.k', '6', 'integer', '{"min": 2, "basis": "a single cluster is no peer grouping"}', TRUE,
  '{"ar": "عدد المجموعات السلوكية", "en": "Number of behavioural clusters per currency"}'),
 ('peer.sample_max', '50000', 'integer', NULL, FALSE,
  '{"ar": "أقصى عينة للتجميع", "en": "Largest sample used to fit the clusters"}'),
 ('peer.min_stability', '0.70', 'decimal', NULL, FALSE,
  '{"ar": "أدنى ثبات مقبول للمجموعات", "en": "Stability shown as a warning below this share"}'),
 ('schedule.feature_compute', '"0 10 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "حساب الميزات", "en": "Feature computation (hourly, bank time)"}'),
 ('schedule.segment_assign', '"0 40 2 * * *"', 'cron', NULL, FALSE,
  '{"ar": "إسناد القطاعات", "en": "Segment assignment (daily)"}'),
 ('schedule.segment_stats_refresh', '"0 30 3 2 * *"', 'cron', NULL, FALSE,
  '{"ar": "تحديث إحصاءات القطاعات", "en": "Segment statistics (second day of the month)"}'),
 ('schedule.peer_group_build', '"0 0 4 5 * *"', 'cron', NULL, FALSE,
  '{"ar": "بناء المجموعات السلوكية", "en": "Peer-group candidate builds (fifth day of the month)"}'),
 ('schedule.profile_refresh', '"0 30 3 3 * *"', 'cron', NULL, FALSE,
  '{"ar": "تحديث الملفات الشخصية", "en": "Profile refresh (third day of the month)"}')
ON CONFLICT (key) DO NOTHING;
```

## 2. DDD Implementation (Rust)

**Crate placement** (workspace pinned in phase 00; layer rules in `ddd.md`). Pure logic lives in `domain` so that the
worker, the API and the tests share one implementation; no pure module performs IO.

```
backend/
├── packs/features/catalogue.v1.json      # shipped feature catalogue (section 3.1), listed in the release manifest
├── schemas/feature-catalogue.v1.json, segment-definition.v1.json, peer-build.v1.json
├── crates/
│   ├── domain/   # + features (FeatureDefinition, FeatureExpr, WindowSpec, FilterExpr, FeatureCompiler, Validator),
│   │             #   segments (SegmentDefinition, Assigner, MergePlanner), peers (KMeansPlusPlus, RobustScaler,
│   │             #   Stability), profiles (ProfileComparator), ports (DeclaredProfileSource, RatingSource)
│   ├── db/       # + PgFeatureRepository, PgFeatureRunRepository, PgSegmentRepository, PgMembershipRepository,
│   │             #   PgSegmentStatsRepository, PgPeerRepository, PgProfileRepository (SQL over `canon`)
│   ├── authz/    # + resource adapters Customer (see Open doubts) and FeatureDefinition, Segment, PeerGroup
│   └── jobs/     # + kinds feature_compute, segment_assign, segment_stats_refresh, peer_group_build, profile_refresh
├── api/          # + routers segments (under /ref), feature_definitions, peer_groups, profiles
├── worker/       # + FeatureEngine, SegmentService, ProfileService and the five handlers
└── ctl/          # + features seed, features verify
```

- **Domain entities** (`Debug, Serialize, Deserialize, FromRow, ToSchema`; fields mirror section 1):
  `FeatureDefinition`, `FeatureRun`, `FeatureValue`, `Segment`, `SegmentMembership`, `SegmentStat`, `PeerGroup`,
  `Profile`. The enum `FeatureRunState` is `#[sqlx(type_name = "text")]` with the three states of section 1.
- **Value objects:** `FeatureKey`, `EntityKind` (`Account`, `Customer`, `Counterparty`, `Branch`), `FeatureExpr` (an enum
  with exactly nine variants: `Count`, `Sum`, `Avg`, `Max`, `DistinctCount`, `Ratio`, `DaysSince`, `Exists`,
  `SegmentStat`; no other variant exists), `WindowSpec` (`Rolling { days | business_days | months, n }` or `Calendar
  { month | quarter }`, `n <= features.max_window_days` for rolling), `FilterExpr` (the DSL.md Filter subset: comparisons,
  AND, OR, NOT, IN literal list, IS NULL, BETWEEN, arithmetic, `within_band`, `pct_of`; nothing else parses),
  `CanonRef` (`entity.attribute` of the phase 10 catalogue), `Currency` (`IQD`, `USD`, any `ref.currencies` code; `XXX`
  for currency-free values), `SegmentRule` (conjunction of `Predicate { ref, op, value }`; `op` in `=`, `<>`, `in`,
  `not_in`, `>=`, `<=`, `between`, `is_null`), `Stat` (`P50`, `P95`, `P99`, `Mad`), `Status` (`Value`, `NoHistory`,
  `BeyondWindow`), `ProfileStatus` (`Complete`, `Degraded`), `Comparator` (`Declared`, `OwnHistory`, `Peer`).
- **Create and Update DTOs:** `NewFeatureDefinition` (proposal: key, entity, name, value kind, window, expression,
  currency mode), `NewSegment` (proposal: key, name, level `static` or `risk_overlay`, rule, merge target, sort order,
  min size, `base_version?`), `PeerBuildRequest { currency }`. No Update struct: definitions and segments change only by
  new versions through change requests, and segment statistics and memberships are written by jobs.
- **Repository traits** (domain layer):

```rust
#[async_trait]
pub trait FeatureRepository: Send + Sync {
    async fn active(&self, conn: &mut PgConnection) -> Result<Vec<FeatureDefinition>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &FeatureFilter, page: Keyset) -> Result<Page<FeatureDefinition>, RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<FeatureDefinition>, RepoError>;
    async fn activate_version(&self, tx: &mut Tx, d: &FeatureDefinition, cr: Uuid) -> Result<(), RepoError>; // retires the previous active row
}
#[async_trait]
pub trait FeatureValueWriter: Send + Sync {
    /// Set-based insert-or-update on the primary key; touches only rows whose basis differs. Returns (inserted, updated).
    async fn write_month_end(&self, tx: &mut Tx, feature: &FeatureDefinition, as_of: NaiveDate, sql: &CompiledFeature)
        -> Result<(u64, u64), RepoError>;
}
#[async_trait]
pub trait SegmentRepository: Send + Sync {
    async fn list(&self, conn: &mut PgConnection, page: Keyset) -> Result<Page<Segment>, RepoError>;
    async fn get_by_key(&self, conn: &mut PgConnection, key: &str) -> Result<Option<Segment>, RepoError>;
    async fn active_static(&self, conn: &mut PgConnection) -> Result<Vec<Segment>, RepoError>;   // ordered by sort_order
    async fn set_membership_batch(&self, tx: &mut Tx, changes: &[MembershipChange], as_of: NaiveDate) -> Result<u64, RepoError>;
    async fn membership_at(&self, conn: &mut PgConnection, level: Level, as_of: NaiveDate) -> Result<MembershipSnapshot, RepoError>;
}
#[async_trait]
pub trait PeerRepository: Send + Sync {
    async fn active(&self, conn: &mut PgConnection, currency: Currency) -> Result<Option<PeerGroup>, RepoError>;
    async fn set_state(&self, tx: &mut Tx, id: Uuid, from: PeerState, to: PeerState, expected_version: i32) -> Result<PeerGroup, RepoError>;
}
#[async_trait]
pub trait ProfileRepository: Send + Sync {
    /// Deletes and inserts the period's rows for the given customers in the caller's transaction.
    async fn replace_period(&self, tx: &mut Tx, period: NaiveDate, rows: &[Profile]) -> Result<u64, RepoError>;
    async fn for_party(&self, conn: &mut PgConnection, customer_ids: &[Uuid], from: NaiveDate, to: NaiveDate)
        -> Result<Vec<Profile>, RepoError>;
}
```

- **utoipa schemas** (request and response DTOs; the `ProfileView` projection has no `deviation`, `segment_id` or
  `expected.own_history` field, and the projection is a type of its own, so a serializer cannot emit them):
  `SegmentView`, `SegmentDetailView`, `SegmentStatsView` (rows with `period`, `feature_id`, `currency`, `n`, `p50`, `p95`, `p99` and `mad`;
  a suppressed population has no row and no count), `SegmentProposal`, `FeatureDefinitionView`, `FeatureProposal`, `PeerGroupView`,
  `PeerGroupDetailView` (centroids in scaled space, cluster sizes), `PeerBuildRequest`, `PeerBuildAccepted` (202),
  `PartyProfilesView` (per customer record: periods with `actual`, `declared` and `status`), `ChangeRequestAccepted`
  (the CR id, class, route, volume effect; reused from phase 04 where it exists).
- **Route table** (`| Method | Path | Scope | Description (cedar action) |`; `CR` marks a change request; `4E`
  marks four-eyes at approval, which phase 04 enforces):

| Method | Path | Scope | Description (cedar action) |
|-|-|-|-|
| GET | `/api/v1/ref/segments` | `config.author` | Segment register: key, level, members, merged_into, min_size, status; no membership lists; `members_count` below `min_size` shown as "below minimum" (`segment.list`) |
| POST | `/api/v1/ref/segments` | `config.author` | Propose a static or risk-overlay segment, or a new version by `key` with `base_version`; `Idempotency-Key`; CR `segment` (`segment.propose`) |
| GET | `/api/v1/ref/segments/{id}` | `config.author` | Detail: rule, merge chain, version, change request, last assignment run (`segment.read`) |
| GET | `/api/v1/ref/segments/{id}/stats` | `config.author` | Robust statistics per feature, currency and period; a suppressed population has no row and is never shown (`segment.stats.read`) |
| GET | `/api/v1/feature-definitions` | `config.author`, `config.approve` | Active and retired feature versions with the window and the expression kind (`feature.read`) |
| POST | `/api/v1/feature-definitions` | `config.author` | Propose a new feature or a new version; `Idempotency-Key`; CR `feature_definition` (`feature.propose`) |
| GET | `/api/v1/feature-definitions/{id}` | `config.author`, `config.approve` | One version: expression, requires, recent run states, readers count (`feature.read`) |
| GET | `/api/v1/peer-groups` | `config.author`, `dashboard.read` | Peer builds per currency: state, k, members, WCSS, stability, as_of (`peer_group.read`) |
| POST | `/api/v1/peer-groups` | `config.author` | Request a candidate build for one currency at the last closed month-end; 202 with the job id; `Idempotency-Key` (`peer_group.build`) |
| GET | `/api/v1/peer-groups/{id}` | `config.author`, `dashboard.read` | Centroids, cluster sizes (clusters below the floor shown as merged), definition, stability (`peer_group.read`) |
| GET | `/api/v1/parties/{id}/profiles` | `queue.work`, `kyc.work` | Monthly actual and declared figures per customer record of the party; purpose required (restricted tier); `Cache-Control: no-store` (`profile.read`) |

- **Router:** `pub fn router() -> Router<AppState>` in each of the four modules; the `segments` router is nested under
  `/ref` by the phase 09 reference router.

## 3. Business Logic

### 3.1 Feature catalogue: definitions, validation, compilation and seed (`FeatureValidator`, `FeatureCompiler`; S-12)

A feature definition is `{feature_key, entity, name, value_kind, window_spec, expression, currency_mode}`. Example
(`credit_turnover_1m`): `window_spec {"kind": "calendar", "period": "month"}`, `expression {"kind": "sum", "field":
"txn.amount", "where": "txn.dr_cr = 'C' AND txn.status = 'posted' AND txn.system_generated IS NOT TRUE"}`,
`currency_mode per_currency`. The nine closed kinds are `count`, `sum`, `avg`, `max`, `distinct_count` and `ratio` (the
DSL.md A.6.1 measures), `days_since` (a day count from the last matching posting), `exists` (a boolean) and
`segment_stat` (a statistic of a monthly source feature for the entity's static segment, section 3.4). Nothing else is a
feature kind; a document or request naming another kind is refused.

`FeatureValidator::check(def, catalogue)` is pure and runs in this order; the first failure stops the check:
1. JSON Schema `feature-catalogue.v1.json`, then the closed enumerations (entity, kind, value kind, currency mode).
2. Every `CanonRef` in `field`, `where`, `group` and joins exists in the phase 10 `CanonicalCatalogue` (`E_UNKNOWN_ATTRIBUTE`),
   is not a class `personal`, `secret-str` or `card` column (one exception: the `field` of a `distinct_count` may be
   `txn.counterparty_id`, a surrogate id that is counted and never stored or shown), and is not a protected attribute: gender, religion,
   ethnicity, nationality, tribe and sect (DSL.md A.5, `E_PROTECTED_ATTRIBUTE`; the rating allowance of C.2 for
   nationality does not carry over).
3. `sum`, `avg` and `max` need a numeric field (`E_FIELD_TYPE`); `where` parses under the DSL.md Filter subset
   (`E_FILTER_NOT_COMPILABLE`).
4. `rolling` windows are at most `features.max_window_days` long (`E_WINDOW_TOO_LONG`); calendar windows are `month`
   or `quarter`.
5. Currency: `per_currency` features read the native `txn.amount` and group by `txn.currency`; `usd_equivalent`
   features read `txn.amount_usd` only and are stored under currency `USD`, labelled in `basis`. A feature never adds
   IQD to USD and never reads `amount_lcy` (DR-80 and T-04: no conversion without the business-date rate).
6. `segment_stat`: the source is an active monthly feature of kind `sum`, `count`, `avg` or `max` (no chains), the stat
   is `p50`, `p95`, `p99` or `mad`, `lag_months` is 1 (a statistic of the previous month only, so no look-ahead), and the
   entity is `customer` (`E_DANGLING_REF` for an unknown source).
7. `days_since` is on entity `account` with `currency_mode none`; `ratio` takes its numerator and denominator from
   measures of the same definition; a zero denominator gives `no_history`.
8. Cost: `FeatureCompiler` generates the SQL, `EXPLAIN` runs on one partition of the source, and a plan above the bound
   is refused (`E_EXPR_COST`).
9. `requires` is derived from the references of step 2 and stored; it is never typed by the author. The phase 06
   grader reads it (section 4).

`FeatureCompiler::compile(def, as_of)` is pure and returns `CompiledFeature { sql, binds }`. Identifiers come only from
the catalogue whitelist; every literal is a bound parameter; the grouping is by `(entity id, currency)`. The measure and
filter compiler is this module's; phase 16 reuses it for scenario aggregates and does not write a second one. The
month-end SQL of `credit_turnover_1m` is:

```sql
INSERT INTO feat.feature_values (entity, entity_id, feature_id, currency, as_of, value, status, basis, computed_at)
SELECT 'customer', a.customer_id, $1, t.currency, $2, SUM(t.amount), 'value', $3, now()
FROM canon.txn t
JOIN canon.account a ON a.id = t.account_id AND a.recorded_to IS NULL
WHERE t.booking_date BETWEEN $4 AND $2 AND t.dr_cr = 'C' AND t.status = 'posted' AND t.system_generated IS NOT TRUE
GROUP BY a.customer_id, t.currency
ON CONFLICT (entity, entity_id, feature_id, currency, as_of) DO UPDATE
    SET value = EXCLUDED.value, status = EXCLUDED.status, basis = EXCLUDED.basis, computed_at = now()
    WHERE feat.feature_values.value IS DISTINCT FROM EXCLUDED.value
       OR feat.feature_values.basis IS DISTINCT FROM EXCLUDED.basis;
```

`basis` holds only stable facts: `{definition_sha256, window_from, excluded_units}`. Excluded transactions (system-generated
legs by DR-42, unposted rows, cancelled rows) are filtered by the default `where` of the definition; a definition opts in
to system-generated legs by name, never by default.

**Seed (`akr-ctl features seed [--dry-run]`).** The shipped catalogue `backend/packs/features/catalogue.v1.json` holds
nine features, each checked by section 3.1 before it is written:

| Key | Entity | Kind | Window | Currency | Used by |
|-|-|-|-|-|-|
| `credit_turnover_1m` | customer | sum of credits | calendar month | per currency | profiles, peer builds |
| `debit_turnover_1m` | customer | sum of debits | calendar month | per currency | profiles |
| `txn_count_1m` | customer | count | calendar month | per currency | profiles, peer builds |
| `cash_share_1m` | customer | ratio of cash-flagged count | calendar month | per currency | peer builds |
| `distinct_counterparties_3m` | customer | distinct count | rolling 90 days | per currency | peer builds |
| `avg_credit_size_3m` | customer | average of credits | rolling 90 days | per currency | phase 16 |
| `max_single_credit_usd_1m` | customer | maximum of USD equivalents | calendar month | usd equivalent | `p99_single_credit_usd_segment` |
| `p99_single_credit_usd_segment` | customer | segment statistic (p99) | previous month | usd equivalent | phase 16 |
| `dormant_days` | account | days since the last posting | rolling 400 days | none | phase 16 |

The steps: (1) verify the manifest hash and signature of the file; (2) validate every feature; any failure exits 1 and
writes nothing; (3) in one transaction insert each key not yet present as version 1, `state active`, `created_by` the
system principal, and the root segment `static.all` (no predicate, sort order 1000000) if absent; a key whose installed version differs from the file is refused with exit 2 (new versions come only
through change requests); (4) audit `feature.seeded {count, keys}` and enqueue `feature_compute`; (5) a second run writes
nothing and audits zeros.

### 3.2 Feature computation (`FeatureEngine`, job `feature_compute`; S-12)

Payload `{reopen_from?: date}`; coalesce key `feature_compute`; concurrency 1; hourly. A replay of a phase 11 batch with
booking dates in a closed month sets `reopen_from` to that month-end. Steps:
1. Candidate month-ends: every month-end `M` no later than the last closed business date (BANK_DATE − 1 of the adapter
   pack, read through phase 11 `FreshnessQuery`), within `features.catchup_months` back from the newest, plus every month
   from `reopen_from`. A month-end is "closed" only when the phase 09 default calendar says the day is a working day that
   has ended.
2. Gate for each `(M, feature)`, evaluated once per month for all features that read the same source tables:
   a. `halted` watermark of a needed table → `not_ready`, reason `halted`; a `stale` one → `stale`; `data_as_of` (per
      branch, phase 11) before `M` → `behind_business_date`, as classed in phase 11 section 3.7.
   b. An open or covering blind period (`admin.blind_periods`, phase 11) for a source → `dark`, reason `blind_period`.
   c. A unit of the coverage ledger (phase 12, unit `branch`) with reason `blind_period`, `source_stale` or
      `no_postings_since` at `M` → `dark`, reason `dark_unit`, unless the unit has an approved exclusion (phase 12
      `exclusions`) at `M`: then the unit is listed in `basis.excluded_units` and computation continues without it
      (section 3.6 degrades the profiles of its customers).
   d. A `usd_equivalent` feature with a posted row in the window whose `amount_usd` is null → `not_ready`, reason
      `missing_rate` (no partial sum; DR-80).
3. Compute the passing `(M, feature)` pairs: run the compiled SQL (section 3.1) in one transaction, then
   `feature_runs` upsert with `state computed`, `rows_written` = inserted plus updated, `basis` as in section 3.1.
   A `segment_stat` feature reads `feat.segment_stats` of the previous period; a customer whose segment has no row
   (suppressed) gets `no_history` with `basis.reason = segment_suppressed`.
4. Rows are written only for entities with activity in the window (sparse). For a `computed` run, an absent row means
   zero for `count` and `sum` and "not applicable" for the other kinds. For `not_ready` and `dark` runs an absent row
   means unknown and every consumer shows it as unknown.
5. Idempotency: the primary key, the `basis` comparison and `feature_runs` make a second run for the same months write
   zero rows. A re-run with changed data updates only the rows whose value changed.
6. Audit one `feature.computed {computed, not_ready, dark, inserted, updated, excluded_units}` per job run.

Currency: IQD and USD are computed separately by `txn.currency`; a cross-currency measure exists only as
`usd_equivalent`, whose rate is the business-date rate that the mapping stored in `amount_usd` (phase 09, DR-80).

### 3.3 Segment definitions and assignment (`SegmentService`, job `segment_assign`; S-12, S-10)

**Definition.** `{scope: "customer", match: [Predicate...], merge_into: key | null}`. Predicates are conjunctions only (a
disjunction is two segments in sort order). Static segments may read `customer.*`, `party.*`, `account.*` and
`branch.*` attributes of the phase 10 catalogue that are not protected (section 3.1 step 2). A risk-overlay segment may
read `rating.*` attributes only (phase 15 registers them; until then a risk overlay cannot be proposed:
`E_DANGLING_REF`). No definition may read alert, case, screening, watch, STR, decision or any `casework.*`, `alert.*`
or `screening.*` attribute (`E_SEGMENT_INPUT_NOT_ALLOWED`). `merge_into` names a segment of the same level, must not
form a cycle (`E_MERGE_CYCLE`) and is never set on a risk overlay. The root `static.all` has no predicate, sort order
1000000 and no merge target; it is seeded by the catalogue install.

**`SegmentService::propose(caller, NewSegment)`** (`POST /ref/segments`):
1. Validate: key pattern; level `static` or `risk_overlay` (`cluster` is refused: cluster segments come from peer builds);
   predicates; `min_size >= segments.min_size` (`E_SEGMENT_MIN_SIZE`); `sort_order` unique among static segments.
2. If the key exists, `base_version` must equal the current version (`E_STALE_BASE`, 409). An open change request on the
   same `object_key` gives `E_NOT_ALLOWED_IN_STATE` (409).
3. Create the change request through phase 04 `ChangeRequestService` (kind `segment`, `object_key segment:<key>`, reason
   family `change`); the route is `cr_material` (parallel MLRO and CHK). Volume effect: `neutral` when no scenario version
   reads the segment (`ReaderCatalogue`, section 3.8); otherwise the effect that `ReaderCatalogue::measure` returns from
   the phase 16 replay of the reading versions (`increase`, `decrease` or `neutral`; CH joins the route for a `decrease`),
   and `unknown` when no replay can measure it, which phase 04 never approves as material (as DR-96 for exclusions).
4. Respond 201 with the CR; `change_request.created` is written by phase 04.

**Assignment (`segment_assign {level: static | risk_overlay}`):**
1. Load the active segments of the level in `sort_order`; compile each predicate list to one SQL `CASE` evaluated per
   customer page; the first match wins.
2. Keyset pages of 5,000 current customers (`canon.customer` with `recorded_to IS NULL`, ordered by `id`) in one
   transaction per page, joined to `canon.party`, `canon.account` (current rows) and `canon.branch`.
3. Count the raw members of each segment. A segment below its `min_size` is merged upward: its members move to
   `merged_into` (the definition's target), repeated along the chain (at most four steps; the root is never merged). A
   risk-overlay segment is not merged: it is flagged `below_min` and its statistics stay suppressed.
4. For each customer whose effective segment differs from the open membership row, close that row (`valid_to = as_of`)
   and open a new row (`valid_from = as_of`); an unchanged customer writes nothing.
5. Risk overlay: the levels come from the `RatingSource` port (section 3.8); a customer without a rating has no row
   (absence means "not assessed"), counted in the audit.
6. Update `segments.members_count` and the effective `merged_into` (null when not merged); each changed merge target is
   audited as `segment.merged`.
7. Audit `segment.assigned {level, customers, moved, unassessed, below_min}`. If any static membership moved, enqueue
   `segment_stats_refresh` for the current period.

### 3.4 Segment statistics (`segment_stats_refresh {period}`; S-12)

1. Require the month-end `M` of the period to be `computed` for every monthly feature (section 3.2) or to have only
   excluded units. Otherwise reschedule the job (coalesce key `segment_stats:<period>`) and write nothing.
2. For each active static segment and each active cluster segment of an active build, each monthly feature and each
   currency present: take the values with `status = value` at `M` for members whose membership row holds at `M`
   (`valid_from <= M < coalesce(valid_to, infinity)`).
3. If `n < segments.min_size`, write no row (the suppressed count goes to the audit).
4. Else compute `n`, `p50`, `p95`, `p99` with `percentile_cont` and `mad = percentile_cont(0.5)` of `|x - p50|`.
5. One transaction: delete the period's rows of these `(segment, feature, currency)` keys and insert the new rows (a
   rebuild, so a segment that fell below the floor loses its row).
6. Audit `segment_stats.refreshed {period, rows, suppressed}`; enqueue `profile_refresh` for the period.

Statistics are population figures. The API returns them only for the populations of section 3.4 step 3 and never per
branch, per customer or per alert.

### 3.5 Peer groups (`peer_group_build {currency}`, algorithm `kmeans_pp_v1`; S-12)

A build clusters the customers of one currency on the month-end `M` of the last month whose monthly features are
`computed` (or excluded). Its definition fixes the features (default `credit_turnover_1m`, `txn_count_1m`,
`cash_share_1m`, `distinct_counterparties_3m`), `k` (`peer.k`), the sample size (`peer.sample_max`), the transform and
the seed. Any change to the algorithm or the features is a new algorithm name, hence a new build. A build for an
existing `(currency, as_of)` returns that row (idempotent). Steps:
1. Vectors: a customer with a value for each feature at `M` (count and sum features absent under a `computed` run count as
   zero; a ratio or distinct feature absent excludes the customer). Customers of excluded units are excluded and counted.
2. Transform: `log1p` for amount, count and distinct features; ratios unchanged.
3. Robust scaling on the sample: centre = median, scale = 1.4826 × MAD (fallback IQR / 1.349, then 1.0); values clipped
   to [−5, 5].
4. Sample: when the population exceeds `peer.sample_max`, the first `sample_max` customers in `id` order (UUIDv7 time order,
   so the sample is deterministic).
5. `k = min(peer.k, max(1, floor(sample / (2 × segments.min_size))))`; `k = 1` records the flag `single_group`.
6. Seed: the first 8 bytes of SHA-256(`peer|<currency>|<M>|kmeans_pp_v1`) seed a ChaCha8 generator.
7. Initialisation (k-means++): the first centroid is drawn uniformly; each next is drawn with probability proportional to
   the squared distance to the nearest chosen centroid, by cumulative sampling with the generator.
8. Lloyd iterations, at most 100: assign each point to its nearest centroid (ties to the lower index), recompute the
   means; an empty cluster takes the point farthest from its centroid (ties to the smallest id). Stop when no assignment
   changes; record `iterations` and `converged`.
9. Assign every customer of the population (not only the sample) to the nearest final centroid.
10. Merge undersized clusters: repeatedly merge the smallest cluster below `min_size` into the nearest cluster of at least
    `min_size` (distance between centroids, ties to the lower index); recompute the centroids as member means; renumber
    by size descending, then by the smallest member id.
11. `wcss` is the sum of squared distances to the assigned centroid. Centroids are stored in the scaled space, with the
    scaling parameters in `definition`.
12. Stability against the active build of the currency, on common customers: each new cluster is matched one-to-one, by
    greatest overlap, to an old cluster; `stability` is the share of common customers whose old cluster equals their
    matched one. The first build has no stability. Below `peer.min_stability` the change request shows a warning; it does
    not block.
13. One transaction: insert the `peer_groups` row as `candidate` (`build_no` = next number for the currency) and one
    `segments` row per cluster (level `cluster`, `status candidate`, key `peer.<currency lower>.<build_no>.<n>`,
    `peer_group_id`, `min_size`, `members_count`, `currency` (the build's), `name` (`{"ar": "المجموعة السلوكية <n>", "en": "Behavioural cluster <n>"}`), `definition` (`{"scope": "customer", "cluster": <n>}`) and `created_by` (the build's `requested_by`)). Audit `peer_group.built {currency, build_no, k, members, wcss,
    stability, converged}`.

A candidate becomes live only through its change request (section 3.7). The job never activates a build.

### 3.6 Expected profiles (`ProfileService`, job `profile_refresh {period}`, read route; S-10, S-12)

**Refresh.** `period` is the first day of the month; `M` is its month-end. Steps:
1. Preconditions: `credit_turnover_1m`, `debit_turnover_1m` and `txn_count_1m` are `computed` at `M`, and the period's segment statistics are written or no segment is assigned yet; otherwise reschedule and write nothing.
2. Customers: current `canon.customer` rows. A customer whose home branch (`canon.customer.branch_id`) is in
   `basis.excluded_units` of the runs gets no row (the read shows `unknown`).
3. Actual per currency `c` in {IQD, USD}: credit turnover, debit turnover and count from `feat.feature_values` at `M`; an
   absent value under a `computed` run is zero. A row is written only when the actual figures are non-zero or a declared
   figure exists.
4. Declared: `DeclaredProfileSource::monthly(customer, c, period)` (section 3.8) returns `declared` with an amount and its
   source (`kyc` from phase 15, or `cbs_maintained`), `undeclared`, or `not_maintained`. The CBS field
   `canon.customer.expected_profile` is read only when `customer.expected_profile` is in `coverage.maintained_fields`
   (phase 06, DR-38); otherwise the value is unknown, never low.
5. Own history: the credit turnover of the customer's previous rows with `status complete` within
   `profile.lookback_months`. With at least `profile.min_history_months` of them: `median` and `mad`. Otherwise a
   short-history fallback: no own-history summary and no own z are stored, the peer comparator of step 6 applies (DSL.md A.6.3 `min_history`), and the row is `degraded` (step 8). Degraded rows never enter any history.
6. Peer: the customer's static segment at `M` (membership row holding at `M`) and its `credit_turnover_1m` statistic of the
   previous period (`p99`; no look-ahead). A suppressed or absent statistic is a peer fallback that is unavailable.
7. Comparators stored in `deviation` (never returned by any route): declared ratio `actual / declared` (declared above zero);
   own robust z `(actual - median) / (1.4826 × mad)` (`mad` zero gives no value, reason `no_spread`); peer ratio
   `actual / p99`.
8. Status: `complete` when the declared figure exists and own history is sufficient; `degraded` when any comparator is a
   fallback (declared missing or not maintained, short history, peer unavailable, `no_spread`). The `expected` column
   holds the declared figure, the own-history summary and the peer statistic (`detection-logic`).
9. One transaction per period: delete the period's rows and insert the new ones (`replace_period`); audit
   `profile.refreshed {period, rows, complete, degraded, excluded_customers}`.

**Read (`GET /parties/{id}/profiles?from=&to=`).**
1. Resolve the party to its current customer records (`canon.customer` with `party_id`); none gives `E_NOT_FOUND`.
2. Cedar `profile.read` on each `Customer` (section 3.11). A deny is a 403 `E_FORBIDDEN` (a restricted record is not masked:
   the party exists to every queue user). `authz.denied` is written by phase 03.
3. `X-Access-Purpose` is required (`E_PURPOSE_REQUIRED`, 422): `assigned_work` for A1 and A2, `kyc_review` for KYC,
   `investigation` for M. One access event per customer record in the response (`resource_type` `customer`, `resource_id` = the record's id), tier `restricted`, `subject_party` = the party, written before
   the body is sent (phase 02).
4. Range: `from` and `to` are periods; the default is the last twelve months and the maximum is twenty-four. A longer range is 422 `E_VALIDATION` at `/from` with the sub-code `range_too_long`, and a `from` after `to` is 422 `E_VALIDATION` with `range_inverted` (the sub-codes of the phase 11 `E_VALIDATION` row).
5. Projection per customer record and period: a stored row gives `complete` or `degraded`; no row with a `computed` run
   and no excluded unit gives `no_activity` with zero actuals; a `dark`, `not_ready` or excluded case gives `unknown` with
   no figures.
6. The response carries exactly `period`, `currency`, `status`, `actual {credit_turnover, debit_turnover, txn_count}` (null
   when unknown), `declared {status, amount?, source?}` and `data_as_of` (DR-39). It never carries `deviation`, own-history
   or peer values, segment identifiers, feature values, alert or case identifiers, or scores. Response headers:
   `Cache-Control: no-store`.

### 3.7 Change targets (phase 04 `ChangeTargetRegistry`; DR-53, DR-58)

| Kind | Base class | Route | Generic create | Emergency | Families | Activation (inside the approval transaction) |
|-|-|-|-|-|-|-|
| `segment` | material | `cr_material` | no (dedicated route) | no | change | Insert the `segments` row as `active` or update it (version + 1, `change_request_id`); enqueue `segment_assign {level}`; audit `segment.applied` |
| `feature_definition` | material | `cr_material` | no (dedicated route) | no | change | Insert the new version as `active`; retire the previous active row (`state`, `retired_at`); enqueue `feature_compute {catchup}`; audit `feature.activated` |
| `peer_build` | material | `cr_material` | yes | no | change | Previous active build `superseded` and its cluster segments `superseded`; candidate `activating` with `activated_at`; new cluster segments `active`; enqueue `segment_assign {level: cluster, build_id}`; the worker sets the build `active` when every membership batch is written (audit `peer_group.activated`) |

Common rules. `prepare` runs the validator of the kind against the current catalogue and returns the impact (for a
segment, the number of customers whose effective segment would change, computed by a dry run; for a peer build, the
stability and the cluster sizes). `check_before_decision` runs the validator again: a referenced feature that has been
retired refuses the decision with `E_DANGLING_REF`. Nothing is reverted by a `revert` (no emergency route). `on_closed`
with `Rejected` or `Withdrawn`: a candidate peer build becomes `rejected` and its cluster segments `retired` (audit
`peer_group.rejected`); the other two kinds write nothing. Volume effect: `neutral` when `ReaderCatalogue` reports no
reader for the object, else the measured effect of `ReaderCatalogue::measure`, and `unknown` (not approvable) when it
cannot be measured (section 3.3 step 3).

### 3.8 Ports with null implementations (`domain::ports`)

- `DeclaredProfileSource::monthly(customer, currency, period) -> DeclaredValue`. Null implementation: `NotMaintained`. The
  CBS-maintained variant (section 3.6 step 4) is implemented here; phase 15 adds the KYC declared profile.
- `RatingSource::levels(customers, as_of) -> HashMap<Uuid, RatingLevel>`. Null implementation: empty (every customer is
  not assessed). Phase 15 implements it from ratings; this phase never reads the phase 15 tables.
- `ReaderCatalogue::readers(conn, object: &ObjectRef) -> Result<Vec<ReaderRef>, RepoError>` and `measure(conn, object: &ObjectRef, proposal: &serde_json::Value) -> Result<Option<VolumeEffect>, RepoError>` (`conn` is `&mut PgConnection`; `ObjectRef { kind, key }` holds the change-request kind and the `object_key`). Null
  implementation: no readers (so the volume effect is `neutral`) and `measure` returns `Some(Neutral)` (`None` means that the effect cannot be measured, which is `unknown`). Phase 16 implements both
  from scenario versions (draft, shadow and live) and its replays; before phase 16 no scenario reads a feature or a
  segment, so `neutral` is the true effect, not a default.

### 3.9 Errors and problem codes (`FeatError`, thiserror)

| Variant | HTTP and code | Note |
|-|-|-|
| `Validation(Vec<Issue>)` | 422 `E_VALIDATION` | `errors[]` with DSL.md s.7 codes: `E_UNKNOWN_ATTRIBUTE`, `E_PROTECTED_ATTRIBUTE`, `E_DANGLING_REF`, `E_FIELD_TYPE`, `E_FILTER_NOT_COMPILABLE`, `E_WINDOW_TOO_LONG`, `E_EXPR_COST`, `E_SCHEMA`, and the phase 13 codes `E_FEATURE_KIND`, `E_SEGMENT_MIN_SIZE`, `E_SEGMENT_INPUT_NOT_ALLOWED`, `E_MERGE_CYCLE` (DSL.md s.7) |
| `NotFound` | 404 `E_NOT_FOUND` | also for an unknown party |
| `StaleBase` | 409 `E_STALE_BASE` | `base_version` moved |
| `NotAllowedInState` | 409 `E_NOT_ALLOWED_IN_STATE` | open request on the key; peer build not `candidate` |
| `PurposeRequired` | 422 `E_PURPOSE_REQUIRED` | profile read without purpose |
| `Forbidden` / `Scope` | 403 `E_FORBIDDEN` / `E_SCOPE` | phase 03 gates |
| `Idempotency*` | 400, 422, 409 | phase 00 middleware |

### 3.10 Audit events (one per mutation; the profile read writes an access event)

| Event kind | On | Resource |
|-|-|-|
| `feature.seeded` | `akr-ctl features seed` | `feature_definition` |
| `feature.activated` | `feature_definition` change applied | `feature_definition` |
| `feature.computed` | end of a `feature_compute` run | `feature_run` (system) |
| `segment.applied` | `segment` change applied | `segment` |
| `segment.assigned` | end of a `segment_assign` run | `segment` (level) |
| `segment.merged` | an effective merge target changed | `segment` |
| `segment_stats.refreshed` | end of a `segment_stats_refresh` run | `segment_stats` (period) |
| `peer_group.built` | end of a `peer_group_build` run | `peer_group` |
| `peer_group.activated` | last membership batch of a `peer_build` written | `peer_group` |
| `peer_group.rejected` | `peer_build` rejected or withdrawn | `peer_group` |
| `profile.refreshed` | end of a `profile_refresh` run | `profile` (period) |
| `change_request.created` | proposal (phase 04, reused) | `change_request` |

The profile read is an access event (`profile.read`, tier `restricted`, purpose code, party), written by the phase 02
access layer, not an audit row. A refused read is `authz.denied` (phase 03).

### 3.11 Cedar actions

| Action | Resource | Who (shipped bundle) |
|-|-|-|
| `segment.list`, `segment.read`, `segment.stats.read` | Segment | CFG |
| `segment.propose`, `peer_group.build` | System | CFG |
| `feature.read` | FeatureDefinition | CFG, CHK, M, CH, MRO |
| `feature.propose` | System | CFG, MRO |
| `peer_group.read` | PeerGroup | CFG, M, CH, IA |
| `profile.read` | Customer | A1 and A2 within branch scope; KYC; M |

```cedar
// 81_features.cedar (excerpt; schema additions: entity Customer = { branch: Branch })
@id("segment.config") @role("CFG")
permit (principal, action in [Akr::Action::"segment.list", Akr::Action::"segment.read",
        Akr::Action::"segment.stats.read", Akr::Action::"segment.propose", Akr::Action::"peer_group.build"], resource)
when { principal.roles.contains("CFG") };

@id("feature.readers") @role("CFG,CHK,M,CH,MRO")
permit (principal, action == Akr::Action::"feature.read", resource)
when { principal.roles.containsAny(["CFG", "CHK", "M", "CH", "MRO"]) };

@id("feature.proposers") @role("CFG,MRO")
permit (principal, action == Akr::Action::"feature.propose", resource)
when { principal.roles.containsAny(["CFG", "MRO"]) };

@id("peer_group.readers") @role("CFG,M,CH,IA")
permit (principal, action == Akr::Action::"peer_group.read", resource)
when { principal.roles.containsAny(["CFG", "M", "CH", "IA"]) };

@id("profile.read.queue") @role("A1,A2")
permit (principal, action == Akr::Action::"profile.read", resource is Akr::Customer)
when { principal.roles.containsAny(["A1", "A2"]) && (principal.all_branches || resource.branch in principal.branches) };

@id("profile.read.kyc_mlro") @role("KYC,M")
permit (principal, action == Akr::Action::"profile.read", resource is Akr::Customer)
when { principal.roles.containsAny(["KYC", "M"]) };
```

Bundle tests: CFG reads and proposes segments; CHK, CH and MRO read feature definitions but CHK cannot propose one; A1
reads a profile in its branch and not outside it; A2 likewise; KYC reads any branch; IA reads peer groups and no profile.
Scope gates (section 2) admit every role these policies admit.

### 3.12 Tipping-off review

- **Segment register, statistics and definitions** (`/ref/segments*`, CFG): a CFG user learns the bank's segment rules and
  the shape of populations of at least 200, and nothing about any customer. No definition may read alert, case,
  screening, watch, STR or decision data, so a segment cannot encode suspicion; a risk overlay reads only ratings. Counts
  below the floor are not shown.
- **Feature definitions** (`/feature-definitions*`, CFG, CHK, M, CH, MRO): detection logic for the people who decide on
  it; no customer-level data.
- **Peer builds** (`/peer-groups*`, CFG, M, CH, IA): centroids and cluster sizes of populations; no member lists; clusters
  below the floor shown as merged.
- **Profiles** (`/parties/{id}/profiles`): the answer is "nothing beyond the customer's own monthly totals and declared
  figures", which the party page and transaction explorer already show to the same queue roles (W4, W5). Not returned:
  deviation, own-history or peer values, segment identifiers, feature values, alert or case identifiers and scores.
  `status` comes from data availability only (`complete`, `degraded`, `no_activity`, `unknown`), never from alerts, so a
  `degraded` status reveals a data gap, not a case. Every read leaves an access event.
- **Exports and packs**: this phase defines none. Profiles and segment statistics are excluded from every pack and export
  of phases 21 and 24, and from the vendor annex of phase 06.
- **Deviation** exists only inside alert evidence (phases 16 and 17), where the suspicion tier governs it.

### 3.13 Settings and schedules

Settings (migration 0043): `segments.min_size` (governed, floor 200), `features.max_window_days` (governed, floor 31),
`features.catchup_months`, `profile.min_history_months` (governed, floor 3), `profile.lookback_months`, `peer.k` (governed,
floor 2), `peer.sample_max`, `peer.min_stability`. Schedules: `schedule.feature_compute`, `schedule.segment_assign`,
`schedule.segment_stats_refresh`, `schedule.peer_group_build`, `schedule.profile_refresh`. Each change of a governed key
is a change request (phase 04).

## 4. Infrastructure Updates

- **Job kinds registered** (all akr-worker; reads of `canon`, `feat`, `admin.freshness` snapshot and coverage ledger; no
  CBS connection; `akr_app` role):

| Kind | Payload | Coalesce key | Concurrency | Attempts | Schedule or trigger |
|-|-|-|-|-|-|
| `feature_compute` | `{reopen_from?}` | `feature_compute` | 1 | 5 | `schedule.feature_compute` hourly at :10 bank time; enqueued by a `feature_definition` activation (with `catchup`) and by a replay of phase 11 |
| `segment_assign` | `{level, build_id?}` | `segment_assign:<level>` | 1 | 5 | `schedule.segment_assign` daily; enqueued by a `segment` or `peer_build` activation |
| `segment_stats_refresh` | `{period}` | `segment_stats:<period>` | 1 | 3 | `schedule.segment_stats_refresh` on day 2 of the month; enqueued by `segment_assign` after a membership change |
| `peer_group_build` | `{currency, requested_by}` | `peer_group_build:<currency>` | 1 | 3 | `schedule.peer_group_build` on day 5 for IQD and USD; `POST /peer-groups` |
| `profile_refresh` | `{period}` | `profile_refresh:<period>` | 1 | 3 | `schedule.profile_refresh` on day 3; enqueued by `segment_stats_refresh` |

- **Partitions (no new job).** `feature_values`, `segment_stats` and `profiles` are monthly. They are created by phase 02
  `partition_maintenance` (daily, `schedule.partition_maintenance`), which iterates `PartitionRegistry`; this phase adds
  three entries with the factory `feat.create_partition` and `ahead_months = audit.partition_ahead_months` (3). A
  writer that meets a missing month calls the factory once and retries (SQLSTATE 23514), as phase 10 does. There is no
  DEFAULT partition. A second maintenance job (`feat_partition_maintain`) is not created: it would duplicate the
  registry.
- **Docker compose:** no change. No new service, no network edge.
- **Outbox:** no topic. Membership changes are read from `valid_from` by the consumers of phases 16 and 17.
- **AppState additions.** `api`: `features: Arc<FeatureQuery>`, `segments: Arc<SegmentService>` (propose and reads),
  `peers: Arc<PeerGroupService>`, `profiles: Arc<ProfileQuery>`, `targets` (three change targets registered in the
  phase 04 registry). `worker`: `FeatureEngine`, `SegmentService` (assignment and statistics), `PeerBuilder`,
  `ProfileService`, the null ports of section 3.8 and their real CBS-maintained variant. `ctl`: `features seed`,
  `features verify` (the catalogue validated against the installed rows).
- **OpenAPI registration.** Tag `segments` (routes under `/ref`: 4 operations), `feature-definitions` (3), `peer-groups`
  (3) and `profiles` (1): 11 operations in all, each with `x-akr-action` and `x-akr-access` on the profile read. The
  profile operation documents `X-Access-Purpose` as required and the `Cache-Control: no-store` header. Proposals document
  `Idempotency-Key` and the 201 or 202 response with `Location`.
- **Network zones:** no change. Only the worker reads `canon` for this phase; the API reads `feat` only.
- **Readiness:** `/readyz` of akr-api adds `feature_catalogue` (the active catalogue equals the signed file's keys and
  versions) and akr-worker adds `peer_builds` (no `activating` build older than one hour).
- **Metrics:** `akr_feature_runs{state}` (counter), `akr_feature_compute_seconds` (histogram),
  `akr_segment_members{level}` (gauge, members of effective segments), `akr_segment_suppressed{period}` (gauge),
  `akr_peer_stability{currency}` (gauge). Alert: a feature run `dark` for more than 3 days, a `peer_group.built` without a
  decision after 30 days.
- **Existing entity, schema or route updates:**
  - Phase 04: three change targets registered (`segment`, `feature_definition`, `peer_build`); the registry gains
    these three kinds, and the route map gains no route.
  - Phase 02: `PartitionRegistry` gains three feat entries; the access layer gains the `profile.read` action.
  - Phase 03: the resource adapter `customer` is registered (Open doubts) and the `Customer` entity joins the schema.
  - Phase 06: `AttributeCatalogue` gains the active feature keys with their `requires` (extends phase 06 with method
    `feature_keys()`), so the readiness grade of a scenario reads `feat.<key>` as an attribute; `coverage.maintained_fields`
    is read, not changed.
  - Phase 09: the reference router mounts the `segments` router under `/ref`.
  - Planning docs aligned in the same change: API.md 3.8 (no change; the two segment rows stay), API.md 3.11 (add the
    feature, peer-group and profile rows of section 2), DATA-MODEL s.8 (the extensions of section 1), DATA-MODEL s.21
    (the phase 13 grants of 0042), DATA-MODEL s.23 (group 13 files), MODULES M11 (requirement list) and roadmap row 13.

## 5. Frontend Impact

Backend-only phase; no screen of the catalogue (DESIGN-SYSTEM.md 8.1) shows features, segment statistics or profiles.
Segment definitions and feature versions are reviewed through the change request screen G10 (phase 08) and the
change-request detail, which shows the diff, the class, the volume effect and the route. The segment register is read
by the studio D2 (phase 16) for parameters by segment (DESIGN-SYSTEM D2). The party page W4 and the transaction explorer
W5 (phases 08 and 18) may show the profile of section 3.6 as a panel for `queue.work` and `kyc.work` users; that panel is
a phase 18 decision and is not specified here.

Generated-client hooks (available after `make generate-api`): `useGetRefSegments`, `usePostRefSegments`,
`useGetRefSegmentsId`, `useGetRefSegmentsIdStats`, `useGetFeatureDefinitions`, `usePostFeatureDefinitions`,
`useGetFeatureDefinitionsId`, `useGetPeerGroups`, `usePostPeerGroups`, `useGetPeerGroupsId`, `useGetPartiesIdProfiles`.

## 6. Verification

1. `make migrate`: 0040 to 0043 apply; a second run is a no-op. `\d feat.*` lists eight tables (seven of DATA-MODEL s.8
   plus `feature_runs`) and the three monthly parents. `SELECT count(*) FROM admin.column_classification WHERE
   schema_name = 'feat'` returns 93 (one row per column); `SELECT count(*) FROM admin.retention_assignment WHERE
   schema_name = 'feat'` returns 8. Negative checks as `akr_app`: `DELETE FROM feat.feature_values` fails with permission
   denied; `UPDATE feat.feature_definitions SET expression = '{}'` fails with `feature definition content is immutable`;
   `UPDATE feat.peer_groups SET state = 'active'` on a `candidate` row fails with `illegal peer build transition`;
   `DELETE FROM feat.segment_memberships` fails; `SELECT feat.create_partition('feature_values', 2026, 12)` returns
   `feature_values_2026_12`, and a second call returns the same name.
2. `cargo check` with `cargo sqlx prepare`; `cargo clippy --all-targets -- -D warnings`; `cargo deny check`.
3. `akr-ctl features seed --dry-run` lists nine features and the root `static.all`, writes nothing. `akr-ctl features
   seed` audits `feature.seeded {count: 9}`; a second run audits zeros; `akr-ctl features verify` exits 0; a tampered
   catalogue file (one byte) exits 1 with nothing written; a catalogue with a protected attribute exits 1.
4. `cargo test`, including:
   - validator (`domain/features/tests`): each rule of section 3.1 in isolation (unknown attribute, nationality and gender
     refused, `sum` over a text field, a filter outside the subset, a 401-day rolling window, `usd_equivalent` on
     `txn.amount`, a `segment_stat` on a `segment_stat` source, a `segment_stat` with `lag_months` 0, `days_since` with
     a currency, an expression above the cost bound);
   - segments (`domain/segments/tests`): first match wins; merge chain to the root; a cycle refused; a risk overlay never
     merged; a customer without a predicate lands in the root;
   - statistics: 199 members write no row, 200 members write one row (`n` = 200), the MAD of a fixed fixture equals its
     hand-computed value;
   - peers (`domain/peers/tests`): the golden fixture `tests/fixtures/peer/kmeans_pp_v1.golden.json` (1,000 synthetic
     vectors, fixed seed) reproduces its labels and centroids exactly; a permutation of the input order gives the same
     centroids; an undersized cluster is merged into its nearest large neighbour; `k = 1` for a population below twice
     the floor; stability of identical builds is 1.0;
   - profiles: MAD zero gives `no_spread` and no z value; short history gives `degraded`; a dark unit gives no row; the
     `ProfileView` projection has no field named `deviation`, `own_history`, `peer` or `segment_id` (serde test).
5. Curl suite `tests/curl/phase13_features.sh` (dev users `cfg@bank.test`, `chk@bank.test`, `m@bank.test`,
   `mro@bank.test`, `ch@bank.test`, `ia@bank.test`, `a1@bank.test`, `a2@bank.test`, `kyc@bank.test`, `ds@bank.test`):
   - As CFG `POST /api/v1/ref/segments` with `min_size: 150` -> 422 `E_VALIDATION`, `errors[0].code = E_SEGMENT_MIN_SIZE`.
     With `customer.nationality` in the rule -> 422, `E_PROTECTED_ATTRIBUTE`. With `alert.status` in the rule -> 422,
     `E_SEGMENT_INPUT_NOT_ALLOWED`. With `level: risk_overlay` on `rating.level` before phase 15 -> 422 `E_DANGLING_REF`.
   - As CFG a valid static segment `retail_salaried` with `Idempotency-Key: seg-0001` -> 201 with a CR of class material
     and route `cr_material`; the same key and body -> the stored 201 with `x-idempotent-replay: true`; the same key with
     another body -> 422 `E_IDEMPOTENCY_REUSE`. As DS the same POST -> 403 `E_FORBIDDEN`.
   - As CHK and M approve the segment CR (parallel, four-eyes); as CFG the approval -> 403 `E_SCOPE`. After activation
     `GET /ref/segments/{id}` shows `version: 2` and the members after the assignment run.
   - As MRO `POST /api/v1/feature-definitions` with a new key -> 201 (CR `feature_definition`); MRO approves its own CR
     -> 403 `E_SELF_APPROVAL` (MRO holds `config.approve` as validator; phase 04 fixes the order of its checks).
   - As CFG `POST /api/v1/feature-definitions` with `expression.kind = median` -> 422 `E_VALIDATION`, `E_SCHEMA`.
   - As A1 `GET /api/v1/feature-definitions` -> 403 `E_SCOPE`; as CHK -> 200 with nine active rows.
   - As CFG `GET /api/v1/ref/segments/{id}/stats` -> 200; every row has `n >= 200`; no row for a suppressed population
     (the response holds no count of them).
   - As CFG `POST /api/v1/peer-groups` `{"currency":"IQD"}` with `Idempotency-Key: peer-0001` -> 202 with a job id; a
     second key for the same month-end -> the existing build; as M `GET /api/v1/peer-groups` -> 200 with a `candidate`
     row; as CH `GET /api/v1/peer-groups/{id}` -> 200 with centroids; as IA -> 200; as A1 -> 403 `E_SCOPE`.
   - As A1 `GET /api/v1/parties/<party of a branch-A customer>/profiles` without `X-Access-Purpose` -> 422
     `E_PURPOSE_REQUIRED`; with `X-Access-Purpose: assigned_work` -> 200, `Cache-Control: no-store`, and a `jq` walk finds
     none of the keys `deviation`, `own_history`, `peer`, `segment_id`, `alert`, `case`, `score`.
   - As A1 for a party of branch B -> 403 `E_FORBIDDEN`; as KYC with `kyc_review` -> 200; as M with `investigation` -> 200;
     as CFG -> 403 `E_SCOPE`; as DS -> 403 `E_SCOPE`; an unknown party id -> 404 `E_NOT_FOUND`.
   - No suspicion-tier resource exists in this phase, so the masked 404 is not exercised here; the profile read is a
     restricted-tier read (the 422 purpose check above is its gate).
6. Swagger UI lists 11 new operations, each with `x-akr-action`; `make generate-api` run twice gives zero diff.
7. Audit after the curl run: `audit_log` holds `feature.seeded`, `change_request.created` (segment and feature),
   `segment.applied`, `feature.activated`, `peer_group.built`, `profile.refreshed` and `feature.computed`;
   `audit.access_event` holds one `view` row per customer record of each profile response, with tier `restricted` and the purpose code;
   `authz.denied` covers the DS, CHK-config and A1 refusals; `akr-ctl audit verify` passes.
8. Monthly run on the fixture data: a feature run for a month with a blind period on branch BR2 is `dark` with reason
   `dark_unit`, writes no value rows for that branch's customers, and the profile read of a BR2 customer for that month is
   `unknown` with no figures. The same run after an approved exclusion of BR2 (phase 12) is `computed` with
   `basis.excluded_units = [BR2]`, and the profile of a BR2 customer is `unknown`, never zero. A second run writes zero rows.
   No CBS connection is made: the phase 05 statement journal shows no statement during the run.
9. Phases 00 to 12 tests still pass.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 1: corrections found while checking phases 00 to 02 (tools/agent/gap1-run1-verdict.md); lead-verified.

### 7.1 The profile read's access event
The profile read writes the access event `view` with `resource_type = 'customer'`, tier `restricted`, the purpose
code and the party; `profile.read` is the Cedar action, not the access value (phase 02 7.9, DR-358). Section 3.10's
"access event (`profile.read`, ...)" reads accordingly, and the "Phase 02" bullet of the existing-updates list in section 4, which says "the access layer gains the `profile.read`
action" is void: phase 02's access layer needs no change.

Gap pass 1 (2026-10-10), run 3b, from tools/agent/gap1-run3b.md; lead-verified (tools/agent/gap1-run3b-verdict.md).

### 7.2 Compiler and comparator extensions (phase 16 section 3.2)
Phase 16 extends this phase and owns the extension: `FeatureCompiler` gains `compile_measure` and `compile_filter`,
and `ProfileComparator::plan` and `ProfileComparator::evaluate` become public. The feature definitions, their storage
and this phase's callers do not change.

### 7.3 Re-measure at decision (DR-379)
`check_before_decision` (section 3.7) calls `ReaderCatalogue::measure` again on the proposed change, under the volume-effect rule of section 3.7 (no reader: `neutral`; not measurable: `unknown`). If the measured
effect differs from the one stored on the change request, the decision is refused with `volume_effect_changed`
(DR-360). There is no recompute entry point: the maker resubmits once the reader replay completes. This closes phase
16 Open doubts 12.

### 7.4 Tier of profile reads
This phase reads no tier: a profile read is a restricted-tier read, and its access event keeps tier `restricted`. From
phase 15 on the party is loaded through the masked `customer` adapter (the tier from phase 15, and
`casework.effective_tier` from phase 17); the access event does not change.

Gap pass 1 (2026-10-10), run 6b (coverage of T IDs), from tools/agent/gap1-run6b.md; lead-verified (tools/agent/gap1-run6b-verdict.md).

### 7.5 Counterparty resolver (T-29, S-11, DR-398)
MODULES M11 puts counterparty resolution in P1 and DSL A.6 names the resolver as a feature writer; this phase owns it.
`canon.counterparty` is mapped and append-only (phase 10), so its `party_id` holds only what the CBS itself links. The
resolver's links live in a table of their own (extends DATA-MODEL s.8), written by akr-worker, and by the phase 18 decide route for the two analyst methods (phase 18 section 7+).

```sql
-- migration 0044_feat_counterparty_links.sql
-- Phase 13 (gap pass 1). Counterparty to party links. Owner akr_app; retention R2 (follows canon.counterparty).
CREATE TABLE IF NOT EXISTS feat.counterparty_links (
    id                  UUID PRIMARY KEY,
    counterparty_id     UUID NOT NULL REFERENCES canon.counterparty(id) ON DELETE RESTRICT,
    party_id            UUID,                                -- no foreign key: party is bi-temporal
    method              TEXT NOT NULL CHECK (method IN ('own_account', 'confirmed_account', 'name_key', 'none',
                                                        'analyst_confirmed', 'analyst_rejected')),
    decided_by          UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    valid_from          TIMESTAMPTZ NOT NULL,
    valid_to            TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK ((method = 'none') = (party_id IS NULL)),
    CHECK ((method IN ('analyst_confirmed', 'analyst_rejected')) = (decided_by IS NOT NULL)),
    CHECK (valid_to IS NULL OR valid_to >= valid_from)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_counterparty_link_open ON feat.counterparty_links (counterparty_id)
    WHERE valid_to IS NULL;
CREATE INDEX IF NOT EXISTS idx_counterparty_link_party ON feat.counterparty_links (party_id)
    WHERE valid_to IS NULL AND method IN ('own_account', 'confirmed_account', 'analyst_confirmed');

GRANT SELECT, INSERT ON feat.counterparty_links TO akr_app;
GRANT UPDATE (valid_to) ON feat.counterparty_links TO akr_app;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'feat', 'counterparty_links', c, v.cls::admin.data_classification
FROM (VALUES
  ('personal', ARRAY['party_id']),
  ('internal', ARRAY['id','counterparty_id','method','decided_by','valid_from','valid_to','created_at'])
) AS v(cls, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('feat', 'counterparty_links', 'R2', 'owner', NULL, 'Counterparty links: deleted with their counterparty (owner clock)')
ON CONFLICT (schema_name, table_name) DO NOTHING;

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('counterparty.recheck_days', '30', 'integer', NULL, FALSE,
  '{"ar": "أيام إعادة مطابقة الطرف المقابل", "en": "Days before an unlinked or suggested counterparty is checked again"}'),
 ('schedule.counterparty_resolve', '"0 20 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "مطابقة الأطراف المقابلة", "en": "Counterparty resolution (hourly, bank time)"}')
ON CONFLICT (key) DO NOTHING;
```

A link counts (is read by later phases) only for `own_account`, `confirmed_account` and `analyst_confirmed`.
`name_key` is a suggestion shown to the case team and never read by detection; `none` and `analyst_rejected` mean no
link. The row has no case reference, so the table says nothing about suspicion; the case of an analyst decision is in
the case chain (phase 18 section 7+).

Job `counterparty_resolve` (akr-worker; payload `{}`; coalesce key `counterparty_resolve`; concurrency 1; schedule
`schedule.counterparty_resolve`):
1. Candidates, in chunks of 5,000 by id: counterparties with no row in `feat.counterparty_links`, and those whose open
   row is `none` or `name_key` with `valid_from` older than `counterparty.recheck_days`. An open `own_account`,
   `confirmed_account`, `analyst_confirmed` or `analyst_rejected` row is never replaced by the job.
2. `own_account`: `account_hk` equals `number_hk` or `iban_hk` of a current `canon.account` row (`recorded_to IS
   NULL`); the party is the account's current `owner` (`canon.account_party_role`). Exactly one party is required.
3. `confirmed_account`: `account_hk` equals the `account_hk` of another counterparty whose open row is
   `analyst_confirmed`; the party is that row's party. Exactly one party is required. This is how the resolver learns
   from analyst confirmations.
4. `name_key`: the key of a name in `names` (the key function of `crates/names` that fills `canon.party_name.key` (phase 10 section 7.9), DR-371) equals
   the `key` of current `canon.party_name` rows of exactly one party, and that party was never rejected for this
   counterparty (`analyst_rejected`, open or closed).
5. Otherwise, or when a step finds more than one party, `none` (counted as `ambiguous` when more than one).
6. One transaction per chunk: close the replaced open row (`valid_to = now()`) and insert the new row
   (`valid_from = now()`). A re-run that finds the same method and party writes nothing.
7. Audit one `counterparty.resolved {own_account, confirmed_account, name_key, none, ambiguous}` per job run (counts
   only).

Depositor names of cash deposits are not resolved: `canon.cash_event` holds no depositor (S-21; phase 16 Open doubts
2), so T-29 is partly implemented. Verification: a fixture with three new counterparties, one whose account hash is an
own account, one whose name key matches one party and one with no match, gives `own_account`, `name_key` and `none`;
a second run writes nothing; after an analyst confirms the second (phase 18 section 7+), a new counterparty with the
same account hash gets `confirmed_account`.

### 7.6 Open doubts on segments closed (T-08, T-09, DR-399)
The Open doubts on segment-of-one and on membership history are decided. Segment of one for large corporates and
government bodies is P2; such customers stay in the root segment. Segment-migration alerts and a KYC refresh on a
cluster change are P2. The membership history of `feat.segment_memberships` is the record a later phase reads.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.7 Counterparty-keyed features; no counterparty profile (V-13-05-a, V-13-05-b, MEDIUM; DR-413)
VB-05 is carried as features only. A definition with entity `counterparty` is compiled with the grouping `(t.counterparty_id, t.currency)` and no join to `canon.account`; rows whose `counterparty_id` is null are left out, and the values are `feat.feature_values` rows with `entity = 'counterparty'`. The shipped catalogue of section 3.1 holds no counterparty feature; a bank adds one by change request (section 3.7). Profiles stay per customer record: `feat.profiles`, `profile_refresh`, `ProfileRepository`, the profile read and `profile.refreshed` have no counterparty form, and a counterparty profile is P2.

### 7.8 SegmentRepository: the missing methods and types (V-13-01-a, V-13-03-b, V-13-03-c, MEDIUM; DR-413)
Section 2, `SegmentRepository` gains three methods:
- `async fn apply_version(&self, tx: &mut Tx, s: &Segment, cr: Uuid) -> Result<Segment, RepoError>;` inserts the key as `active` with version 1, or updates the key's row (`name`, `definition`, `sort_order`, `min_size`, `version` + 1, `change_request_id`, `updated_at`). The `segment` target of section 3.7 calls it inside the approval transaction.
- `async fn active_by_level(&self, conn: &mut PgConnection, level: Level, peer_group_id: Option<Uuid>) -> Result<Vec<Segment>, RepoError>;` returns the `active` segments of a level in `sort_order`, and only the cluster segments of one build when the id is given. `active_static` is its `static` case; section 3.3 step 1 calls `active_by_level` for `risk_overlay`.
- `async fn set_counts(&self, tx: &mut Tx, rows: &[SegmentCount]) -> Result<(), RepoError>;` writes `members_count` and the effective `merged_into` (section 3.3 step 6); `SegmentCount` is `{ segment_id, members_count, merged_into }`.
`Level` is `Static`, `Cluster` or `RiskOverlay`. `MembershipChange { customer_id, segment_id }` names a customer's new effective segment (section 3.3 step 4). `MembershipSnapshot` is the list of the level's `feat.segment_memberships` rows that hold at `as_of`, with `customer_id`, `segment_id`, `currency`, `valid_from` and `valid_to`.

### 7.9 Refusal codes of a segment proposal (V-13-01-b, MEDIUM; DR-413)
Section 3.3, `SegmentService::propose` step 1: a key outside the key pattern, a level other than `static` or `risk_overlay`, and a `sort_order` that another static segment already holds are each 422 `E_VALIDATION` with the `errors[]` code `E_SCHEMA` and the pointer `/key`, `/level` or `/sort_order`. No new code is added.

### 7.10 Cluster membership: one open row per currency, and the activation steps (V-13-03-d, HIGH; DR-413)
Peer builds are per currency, so a customer active in IQD and in USD belongs to one cluster of each live build. Migration 0040 (section 1): `feat.segment_memberships` gains `currency CHAR(3) NOT NULL DEFAULT 'XXX' CHECK (currency ~ '^[A-Z]{3}$')` (the segment's `currency`; `XXX` for static and risk-overlay rows), its primary key becomes `(customer_id, level, currency, valid_from)`, and `uq_membership_open` becomes `(customer_id, level, currency) WHERE valid_to IS NULL`. Migration 0042: the classification row of the table lists `currency` as `internal`. `set_membership_batch` sets the column from the segment.

Section 2, `PeerRepository` gains `async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<PeerGroup>, RepoError>;` and `async fn insert_candidate(&self, tx: &mut Tx, g: &PeerGroup, clusters: &[Segment]) -> Result<(), RepoError>;` (section 3.5 step 13). `set_state` moves the build's cluster segments in the same statement group: `active` with `activating`, `superseded` with `superseded`, `retired` with `rejected`.

`segment_assign {level: cluster, build_id}` (enqueued by the `peer_build` target of section 3.7) runs these steps in place of section 3.3 steps 1 to 7:
1. Load the build through `PeerRepository::get`. If its state is not `activating`, the job ends and writes nothing. Load its `centroids`, the features and the scaling parameters of its `definition` (section 3.5 steps 2, 3 and 11), and its cluster segments through `active_by_level(Cluster, Some(build_id))`.
2. Keyset pages of 5,000 customers by `id`, one transaction per page. The population is the customers with a vector at the build's `as_of` (section 3.5 step 1). Each vector is transformed, scaled and clipped with the stored parameters and takes the nearest centroid (ties to the lower index).
3. Each page is written through `set_membership_batch` with the build's `as_of`: a customer whose open cluster row of the build's currency names another segment has that row closed (`valid_to = as_of`) and a new row opened (`valid_from = as_of`); a customer with no open cluster row of that currency (every customer at the first build) has only the new row opened; an unchanged customer writes nothing.
4. After the last page, in one transaction: close the open cluster rows of the currency whose segment belongs to another build (customers outside the new population); write the `members_count` of the build's segments through `set_counts`; `PeerRepository::set_state` from `activating` to `active`; audit `peer_group.activated {currency, build_no, members}`; enqueue `segment_stats_refresh` for the current period.
A retried job repeats the pages and writes nothing for rows that are already open. The `peer_build` target refuses, at `prepare` and at `check_before_decision`, a candidate whose `as_of` is not later than the `as_of` of the live build of its currency (409 `E_NOT_ALLOWED_IN_STATE`), so a closed row always ends after it starts.

### 7.11 Expressions of the nine seed features (V-13-04-a, MEDIUM; DR-413)
Section 3.1, seed table. `P` stands for the default filter `txn.status = 'posted' AND txn.system_generated IS NOT TRUE`. `credit_turnover_1m` is the example of section 3.1; the other eight are:

| Key | `expression` |
|-|-|
| `debit_turnover_1m` | `{"kind": "sum", "field": "txn.amount", "where": "txn.dr_cr = 'D' AND P"}` |
| `txn_count_1m` | `{"kind": "count", "where": "P"}` |
| `cash_share_1m` | `{"kind": "ratio", "num": {"kind": "count", "where": "txn.cash_flag = true AND P"}, "den": {"kind": "count", "where": "P"}}` |
| `distinct_counterparties_3m` | `{"kind": "distinct_count", "field": "txn.counterparty_id", "where": "P"}` |
| `avg_credit_size_3m` | `{"kind": "avg", "field": "txn.amount", "where": "txn.dr_cr = 'C' AND P"}` |
| `max_single_credit_usd_1m` | `{"kind": "max", "field": "txn.amount_usd", "where": "txn.dr_cr = 'C' AND P"}` |
| `p99_single_credit_usd_segment` | `{"kind": "segment_stat", "source": "max_single_credit_usd_1m", "stat": "p99", "lag_months": 1}` |
| `dormant_days` | `{"kind": "days_since", "where": "P"}` |

A `ratio` holds its two measures inline as `num` and `den` (section 3.1 step 7). `cash_share_1m` is the share of cash-flagged rows among all posted rows, credits and debits together. `distinct_count` does not count a null `counterparty_id`. `dormant_days` with no posting inside its 400 days has status `beyond_window`.

### 7.12 Segment statistics and feature runs: the repositories and the read (V-13-04-b, V-13-04-c, MEDIUM; DR-413)
Section 2 gains the traits behind `PgSegmentStatsRepository` and `PgFeatureRunRepository`.
- `SegmentStatsRepository`: `async fn replace_period(&self, tx: &mut Tx, period: NaiveDate, segment_ids: &[Uuid], feature_ids: &[Uuid], rows: &[SegmentStat]) -> Result<u64, RepoError>;` deletes the period's rows of the given segments and features and inserts `rows` (section 3.4 step 5). `async fn get(&self, conn: &mut PgConnection, segment_id: Uuid, feature_id: Uuid, currency: Currency, period: NaiveDate) -> Result<Option<SegmentStat>, RepoError>;` serves section 3.6 step 6 and the `segment_stat` kind. `async fn for_segment(&self, conn: &mut PgConnection, segment_id: Uuid, page: Keyset) -> Result<Page<SegmentStat>, RepoError>;` serves the read, newest `period` first, then `feature_id` and `currency`.
- `FeatureRunRepository`: `async fn upsert(&self, tx: &mut Tx, run: &FeatureRun) -> Result<(), RepoError>;` (section 3.2 step 3) and `async fn at(&self, conn: &mut PgConnection, as_of: NaiveDate) -> Result<Vec<FeatureRun>, RepoError>;` (the gates of sections 3.4 to 3.6).

The read `GET /ref/segments/{id}/stats` runs in this order:
1. Cedar `segment.stats.read` on the segment (section 3.11). An unknown id is 404 `E_NOT_FOUND`.
2. `SegmentStatsRepository::for_segment` returns one keyset page.
3. The response is `SegmentStatsView` with those rows and nothing else. A suppressed population has no row (section 3.4 step 3), so the response says nothing about it; the count of suppressed populations is in the audit event `segment_stats.refreshed` only, and `suppressed_periods` is not a field.

### 7.13 One protected-attribute list for features and segments (V-13-06-a, MEDIUM; DR-11, DR-413)
Section 3.1 step 2 and the segment predicates of section 3.3 use one fixed list, held in code: the six attributes of DSL.md A.5 (gender, religion, ethnicity, nationality, tribe and sect). The names and the region of origin that DR-11 also forbids are refused by the same step, as class `personal` columns (`canon.party_name`, `canon.party.birth_place`). From phase 24 on, the register `admin.protected_attributes` does not widen or narrow this check: its `allowed_uses` govern the fairness metrics and the reports of phase 24, and the `segmentation` use of its nationality, residence and governorate rows is a breakdown of those reports, never a detection segment or a feature input.

### 7.14 Forms of the four extension points (V-13-10-a, LOW; DR-413)
Section 7.2: phase 16 owns the two compiler entry points and gives their arguments (phase 16 section 3.2 step 2). Both return `CompiledFeature`, as `compile` does, and take no `as_of`: the window binds come from the caller. `ProfileComparator` is the pure type of `domain::profiles` behind section 3.6 steps 5 to 8. `plan(expected: &[Comparator], combine, min_history) -> ComparatorPlan` fixes the comparators in order with their fallbacks. `evaluate(plan: &ComparatorPlan, actual, declared, history, peer_stat) -> ComparatorResult` returns the ratios and the z of step 7 and the `ProfileStatus` of step 8. `profile_refresh` calls both with the fixed plan of section 3.6: declared, own history, peer, with `min_history` = `profile.min_history_months` and `combine` = `all`. `combine` decides a scenario's verdict only; it changes nothing that `profile_refresh` stores (the ratios, the z and the status of step 8).

### 7.15 Counterparty link writes: one method, one lock (V-13-12-b, V-13-12-d, MEDIUM; DR-398, DR-413)
Section 7.5: the table has one repository, `CounterpartyLinkRepository` (`PgCounterpartyLinkRepository` in `db`), with one write method: `async fn replace_open(&self, tx: &mut Tx, counterparty_id: Uuid, party_id: Option<Uuid>, method: LinkMethod, decided_by: Option<Uuid>) -> Result<bool, RepoError>;`. `LinkMethod` has the six values of the `method` CHECK. The method reads the counterparty's open row `FOR UPDATE`. For a job method (`own_account`, `confirmed_account`, `name_key`, `none`) it writes nothing and returns `false` when the open row is `own_account`, `confirmed_account`, `analyst_confirmed` or `analyst_rejected` (step 1), or has the same method and party (step 6). Otherwise it closes the open row (`valid_to = now()`) and inserts the new row with `ON CONFLICT (counterparty_id) WHERE valid_to IS NULL DO NOTHING`; when nothing was inserted, because another writer opened the counterparty's first row at the same moment, it reads the open row again and applies the same rule once more. Step 6 of the job and the phase 18 decide route (`POST /cases/{id}/counterparty-links`) both write through it, so an analyst row written while a chunk runs is seen by the chunk and kept.

### 7.16 Feature values read for the model sweep (V-27-01-a, MEDIUM; DR-413)
From phase 27 on, `FeatureValueWriter` has a read partner in the same module: `FeatureValueReader::read(conn, entity, entity_ids, features, as_of) -> Result<Vec<FeatureValueRow>, RepoError>`. `features` is a list of `(feature_id, currency)` pairs and `as_of` is a closed month-end. It reads `feat.feature_values` by its primary key columns and returns `FeatureValueRow { entity_id, feature_id, currency, value: Option<Decimal>, status }`; a pair with no row is not returned. The grant `SELECT, INSERT ON feat.feature_values TO akr_app` covers it. Phase 27 calls it in its sweep (section 3.4 step 3, alerts grouped by month-end) and for `feature_psi` (section 3.8); a pair with no row, or a row whose `status` is not `value`, is passed as missing.

## Decisions

Recorded in docs/plan/research.md: DR-98 (the feature catalogue: nine closed expression kinds compiled from JSON by one
compiler that phase 16 reuses; monthly month-end snapshots, sparse by activity, with `feature_runs` saying whether an
absent row is zero or unknown; a month is computed only when closed, fresh, unblocked and not dark, or with dark units
excluded under phase 12; per-currency by default and a labelled USD equivalent from the stored business-date rate, never
a partial sum; protected attributes refused for features and segments; monthly partitions from the phase 02 registry and
`feat.create_partition`) and DR-99 (segments, peers and profiles: static and risk-overlay segments and feature versions
are `material` change targets, a peer build is a generic-created `material` target and the job never activates it; the
volume effect is `neutral` with no reader, else measured through phase 16's `ReaderCatalogue::measure`, and `unknown`
is never approved; static segments are disjoint by sort order under the root `static.all` and merge upward below the
governed floor of 200; statistics exist only for populations of at least 200; clustering is `kmeans_pp_v1` with a
SHA-256 seed, deterministic and idempotent per currency and month-end; profiles store declared, own-history and peer
comparators but the read returns only actual and declared figures, as a restricted-tier read with a purpose, one access
event and branch scope for queue roles).

Lead corrections at review (2026-10-09): the volume effect of a segment or feature read by a scenario was always
`unknown`, which would have made every such change unapprovable; it is now measured by phase 16 (section 3.3 step 3,
3.7, 3.8). The segment list route names `segment.list`. The Cedar excerpt uses the phase 03 schema form
(`principal.roles.contains`), which is correct; phases 05 and 10 to 12 use `principal in Akr::Role::"X"`, for which
the phase 03 schema has no entity, and are fixed in gap pass 1.

## Open doubts (for gap pass 1 and the bank)

- **Bank-wide gating.** A feature month is computed for the whole bank, so one silent branch makes it `dark` until a
  steward excludes the branch under phase 12. Per-branch computation needs branch attribution for every transaction of a
  customer across branches, which the canonical model does not give. The lead should confirm the trade-off.
- **Object-kind and requirement IDs.** DSL.md s.9 lists `segment` under phase 16; this draft registers it here because
  API.md 3.8 puts `POST /ref/segments` in M8 and the CR needs a kind. DATA-MODEL s.8 cites T-07 for segmentation, which
  MODULES M11 does not list; T-08 to T-10 (deviation store, segment migration, minimum size) are research IDs not carried
  here. T-07 is now carried (lead, review); T-08 and T-09 are decided in section 7+.
- **Resource adapter `customer`.** Phase 03 lists masked kinds (customer among them) from phase 15. Phase 13 needs a
  `customer` resource for the profile read before phase 15; this draft registers it unmasked (restricted, not suspicion).
  Phase 15 extends it with its masking. Confirm.
- **Declared income for queue roles.** The profile shows declared income and actual totals to A1, A2 and KYC, as the party
  page already does for queue roles. The bank should confirm that declared income may be shown to analysts outside a case.
- **Nationality.** DSL.md A.5 lists nationality as protected for segments and filters; C.2 allows it for ratings. This
  draft applies A.5 to features and segments. Confirm.
- **Cadence.** Features are month-end snapshots. Event-time features that a scenario needs daily are evaluated inline by
  phase 16 from `canon` with the same compiler; this phase stores month-ends only. Volume is unknown until the bank's
  size is known (sparse rows per customer, feature, currency and month); the lead should ask for the customer count.
- **Hot window.** DSL.md `E_WINDOW_TOO_LONG` refers to "the hot window of the canonical store", which no phase defines.
  This draft uses `features.max_window_days` (400). The definition belongs in DSL.md or ARCHITECTURE.md.
- **Volume effect before phase 16.** Until phase 16 supplies `ReaderCatalogue`, no scenario reads a feature or segment,
  so every change is truly `neutral`. Phase 16 must implement `measure` with its replay, or every change to a read
  segment or feature becomes `unknown` and unapprovable (lead correction; recorded in DR-99).
- **Declared profile source.** Only the CBS-maintained route exists until phase 15. Confirm that `customer.expected_profile`
  (phase 10 `canon.customer`) is the CBS field that DR-38 names, and that `coverage.maintained_fields` uses that key.
- **Segment-of-one.** T-08 (large corporates and government bodies with no peers) is not built: such customers fall to the
  root segment and their statistics are the bank-wide figures. Decided: P2 (section 7+, DR-399).
- **Membership history.** Changes are read from `valid_from` by phases 16 and 17 (no outbox topic). Decided: the
  segment-migration alert of T-09 is P2 (section 7+, DR-399).
- **Dark-unit profiles.** A customer of an excluded unit gets `unknown`, although the exclusion is approved. Confirm that
  this is intended under DR-64 and DR-38.

## What This Phase Does NOT Touch

- No CBS connection and no akr-ingestd job; the only source is `canon` and the phase 11 and 12 read models.
- No scenario documents, parameter sets, replays or alert logic (phase 16 and 17); no `scn` table.
- No rating, KYC, PEP or ownership table (phase 15); the rating and declared sources are ports with null implementations.
- No screening data is read (phase 14) and no alert, case, STR or watch data is read or written.
- No segment or feature statistic is shown per branch, per customer or in any export or pack.
- No new change-request route (the three kinds use `cr_material` and the existing routes), no new problem code in API
  section 6, no new step-up action class, no new role.
- No change to phase 00 to 12 tables and no change to a phase 11 or phase 12 table or grant; `canon` is read only.
- No outbox topic and no second partition maintenance job.
- No screen (DESIGN-SYSTEM.md 8.1 lists none for this data); the party-page profile panel is phase 18.
