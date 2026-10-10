# Phase 11: Production Ingestion & Freshness

**Goal:** The production load path from the bank's core banking system: watermark-driven incremental and sweep loads inside the signed load budget (poll reads of CT1, CT2 and CT4 databases, CT3 change capture and CT4 extract files behind their own feature flags), every batch landed raw as the replay log and mapped in the same transaction through the phase 10 `CanonicalWriter`, freshness classes F1-F5 with `data_as_of` on every response and a stale-source state, restore and failover detection that halts watermarks and opens blind periods, quarantine and row-conflict records, and the three mapping replay routes with a diff reviewed before any apply.

**Dependencies:** Phase 10 (mapping documents, `MappingExecutor`, `CanonicalWriter`, the canon tables). Through phase 10 it uses phase 05 (`SourceReader`, `SelectProof`, `LoadGovernor`, source pauses) and phase 09 (`BankCalendarClock`, the CBS calendar capture)
**Macro-phase:** P1
**Complexity:** L (12 new routes and one extended route; the load loop, the restore rules and the replay diff carry the weight)
**Binaries touched:** akr-api (routers `ingest` and `replays`; the freshness context on every CBS-derived response; the extended resume route) | akr-ingestd (the `ingest_*` jobs, the load loop, `RestoreDetector`, `FreshnessService`, the file and change-capture readers, the replay runner) | akr-worker unchanged (no new job). akr-assess unchanged
**Requirements carried:**
- C-03 (a fresh grant check before any resume of a halted or paused source; section 3.5)
- C-13 (every read passes the phase 05 `LoadGovernor`: signed budget, windows, blackout, slow-down; section 3.1)
- C-15 and C-16 (paused by bank, then automatic catch-up on resume: the watermark stays behind, so catch-up is the normal load loop sliced by `ingest.slice_days`; the operator-planned backfill of phase 12 is not repeated here)
- C-20 (an as-of marker on every batch; raw rows, the batch receipt, canonical writes and the watermark move in one transaction)
- C-21 (a watermark registry per table and branch; the re-read window comes from the mapping's `overlap`)
- C-22 (only closed business days reach canon; a late posting is a new row; the business date behind each batch is stored; settled-time scenario evaluation is phase 16)
- C-23 (a row the source deletes is never deleted from the monitored copy; deletes land as events and are counted)
- C-24 (idempotent loads on `(source, table, pk)` with row-hash guards; raw landing as the replay log; replays with a diff before apply)
- C-27 (blind periods are recorded with cause and dates for the MLRO coverage statement; the statement itself is phase 12)
- C-29 (freshness classes with ingest-lag measures and `data_as_of` with a `stale` flag on responses; F0 and F4 are consumed by phases 2 and 17-21)
- C-35 (the loads are the only reads of the CBS; the phase 05 no-on-demand-query test covers the new routes)
- C-54 (single-statement reads over approved objects through `SourceReader`; the statement journal of phase 05)
- C-62 (ICS BANKS on Oracle as the first adapter pack; section 3.9)
- C-64 (restore and failover detection: halt, alarm, rows that disappeared kept, blind period opened)
- SP-87 (AKR-TMS-down behaviour: blind periods declared by a steward and never back-dated; section 3.6)
- Gap pass 1 (coverage), implemented here: C-39 (the CT4 manifest contract with files, rows and checksums), C-14 and C-41 (section 7+), C-42 (the gateway database as a second source, section 7+); VA-03 (read-only extracts checked against control totals, with automatic catch-up), VA-04 (one runtime with jobs, secrets and alarms on late, partial or failed loads).
- Gap pass 1 (coverage), partly implemented: C-05 (ingest-lag measures and freshness classes), C-11 and TS-08 (the opt-in change-capture sidecar behind the signed agreement), C-12 (the change-capture agreement gate), C-19 (the closed business day gate; a late posting is a new row), TS-11 (the development fixture of the load path); T-41 (the watermark registry with the re-read window; rows the source deletes are kept); VA-17 (ICS BANKS on Oracle as the first adapter pack), VA-24 (the opt-in change-capture path beside the daily batch); VD-04 (the record-text reader for Transact XML records), VD-13 (MT103 and pacs.008 messages mapped); VC-16 (ISO 20022 and MT parsers by element path or field tag with the raw message kept; fixtures for pacs.008 and MT103).
- Gap pass 1 (coverage), not carried: S-20 (optional channel and card-processor links, P2, DR-394); VA-53 (migration kit from core-bundled AML modules, DR-402); VD-03 and VD-34 (vendor read stores and warehouse sources, P3), VD-24 (payment-quality monitor, DR-400).

---

## 1. SQLx Migration Changes

Three migrations, `0033_ingest_landing_registry`, `0034_ingest_replays` and `0035_ingest_settings` (canonical chain in
roadmap.md, DR-23). They follow group 11 of DATA-MODEL.md section 23: the watermark, batch and quarantine tables that
phase 05 listed and phase 11 now writes (DR-63), the blind-period register (DATA-MODEL s.6), the raw landing factory,
and the mapping replay table that the three replay routes of API.md 3.6 need. Two existing tables are modified
(`admin.sources` by a delivery column and the widened tier CHECK for CT3 and CT4 files). Nothing is inserted into
`raw` by a migration: landing tables are created by the factory on the first unit of a table, so a bank with no
changes yet has no landing tables and nothing breaks.

### 1.1 Migration `0033_ingest_landing_registry.sql`

```sql
-- migration 0033_ingest_landing_registry.sql
-- Phase 11. Source delivery modes (DR-63), watermarks, batch receipts, quarantine and row conflicts, blind periods,
-- and the raw landing factory (DATA-MODEL s.3 and s.6, DR-37, DR-39, C-20, C-21, C-27, C-64).

-- CT3 (change capture) and CT4 extract files (DR-63). Phase 05 (0017) has the column CHECK `tier IN ('CT1', 'CT2',
-- 'CT3', 'CT4')` and a separate unnamed table CHECK `tier <> 'CT3'` (PostgreSQL generates its name, normally
-- `sources_tier_check1`). The second one is dropped here by its definition, not by a guessed name; 0017 is never
-- edited. The migration test asserts that a CT3 row with delivery `cdc` inserts.
DO $$
DECLARE r RECORD;
BEGIN
    FOR r IN SELECT conname FROM pg_catalog.pg_constraint
             WHERE conrelid = 'admin.sources'::regclass AND contype = 'c'
               AND pg_catalog.pg_get_constraintdef(oid) LIKE '%<> ''CT3''%'
    LOOP
        EXECUTE pg_catalog.format('ALTER TABLE admin.sources DROP CONSTRAINT %I', r.conname);
    END LOOP;
END $$;
ALTER TABLE admin.sources ADD COLUMN IF NOT EXISTS delivery TEXT NOT NULL DEFAULT 'database'
    CHECK (delivery IN ('database', 'files', 'cdc'));
ALTER TABLE admin.sources ADD COLUMN IF NOT EXISTS inbox_key TEXT
    CHECK (inbox_key IS NULL OR inbox_key ~ '^[a-z][a-z0-9_]{1,31}$');
ALTER TABLE admin.sources DROP CONSTRAINT IF EXISTS ck_sources_delivery;
ALTER TABLE admin.sources ADD CONSTRAINT ck_sources_delivery CHECK (
    (delivery = 'cdc') = (tier = 'CT3')
    AND (delivery <> 'files' OR (tier = 'CT4' AND inbox_key IS NOT NULL))
);

DO $$ BEGIN
    CREATE TYPE admin.batch_status AS ENUM ('landed', 'mapped', 'partial', 'rejected');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

-- One row per interruption that leaves a coverage gap: a pause, a restore or failover suspicion, an unreachable
-- source, an AKR-TMS outage declared by a steward. R10: the register is never deleted (ARCHITECTURE s.18).
CREATE TABLE IF NOT EXISTS admin.blind_periods (
    id              UUID PRIMARY KEY,
    scope           TEXT NOT NULL CHECK (scope IN ('source', 'system', 'rail')),
    source_id       UUID REFERENCES admin.sources(id) ON DELETE RESTRICT,
    cause           TEXT NOT NULL CHECK (cause IN ('source_paused', 'restore_suspected', 'failover_suspected',
                                                    'unreachable', 'akr_outage', 'rail_outage', 'delivery_gap', 'declared')),
    started_at      TIMESTAMPTZ NOT NULL,
    ended_at        TIMESTAMPTZ,
    business_from   DATE,                       -- first business date the gap may cover (source scope)
    business_to     DATE,                       -- last business date covered; set when the period closes
    declared_by     UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,   -- the system principal when automatic
    comment         TEXT CHECK (comment IS NULL OR length(comment) <= 2000),
    statement       JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(statement) = 'object'),   -- counts, causes, dates; no values
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK ((scope = 'source') = (source_id IS NOT NULL)),
    CHECK (ended_at IS NULL OR ended_at >= started_at),
    CHECK (business_to IS NULL OR business_from IS NULL OR business_to >= business_from),
    CHECK (scope <> 'source' OR ended_at IS NULL OR business_to IS NOT NULL)
);
-- One open period per source: a second cause while one is open extends the statement, it does not open another.
CREATE UNIQUE INDEX IF NOT EXISTS uq_blind_open_source ON admin.blind_periods (source_id)
    WHERE scope = 'source' AND ended_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_blind_periods_started ON admin.blind_periods (started_at DESC, id DESC);
CREATE INDEX IF NOT EXISTS idx_blind_periods_source  ON admin.blind_periods (source_id) WHERE source_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_blind_periods_declared_by ON admin.blind_periods (declared_by);

-- The position of each (source, table, branch) read. Lands raw; the watermark moves only in the unit's transaction
-- (section 3.2). branch_code '*' means the table is not kept per branch.
CREATE TABLE IF NOT EXISTS admin.watermarks (
    source_id               UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table            TEXT NOT NULL CHECK (source_table ~ '^[A-Za-z][A-Za-z0-9_]{0,40}$'),
    branch_code             TEXT NOT NULL DEFAULT '*' CHECK (length(branch_code) BETWEEN 1 AND 32),
    kind                    TEXT NOT NULL CHECK (kind IN ('timestamp_column', 'rowversion', 'sequence_id', 't24_date_time',
                                                          'business_date', 'cdc_position', 'manifest', 'full_compare')),
    value                   TEXT,                -- NULL until the first unit has been read
    value_business_date     DATE,                -- business_date kind: the last closed business date read in full
    business_date_moved_at  TIMESTAMPTZ,         -- when value_business_date last advanced (the "stops moving" test)
    as_of_marker            TEXT,                -- the engine's marker at the last unit (SCN, LSN, row version) or NULL
    freshness_class         TEXT NOT NULL DEFAULT 'F3' CHECK (freshness_class IN ('F1', 'F2', 'F3', 'F5')),
    advanced_at             TIMESTAMPTZ,         -- last successful read of this table, empty or not (heartbeat)
    stale_since             TIMESTAMPTZ,         -- set by the freshness job, cleared on recovery
    stale_reason            TEXT CHECK (stale_reason IS NULL OR stale_reason IN ('not_moving', 'behind_business_date',
                                                                                 'lag_ceiling', 'never_loaded', 'halted')),
    halted_reason           TEXT CHECK (halted_reason IS NULL OR halted_reason IN ('business_date_regressed', 'as_of_regressed',
                                                                                   'cdc_gap', 'file_gap')),
    halted_at               TIMESTAMPTZ,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (source_id, source_table, branch_code),
    CHECK ((halted_reason IS NULL) = (halted_at IS NULL)),
    CHECK ((stale_since IS NULL) = (stale_reason IS NULL)),
    CHECK (value IS NULL OR kind <> 'business_date' OR value_business_date IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_watermarks_stale  ON admin.watermarks (stale_since) WHERE stale_since IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_watermarks_halted ON admin.watermarks (source_id) WHERE halted_reason IS NOT NULL;

-- The batch receipt (DATA-MODEL s.3). One row per unit of work: a slice of a table and branch, one file, or one
-- change-capture window. Written in the unit's transaction; `landed` waits for a published mapping version.
CREATE TABLE IF NOT EXISTS admin.batches (
    id                  UUID PRIMARY KEY,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL CHECK (source_table ~ '^[A-Za-z][A-Za-z0-9_]{0,40}$'),
    branch_code         TEXT NOT NULL DEFAULT '*' CHECK (length(branch_code) BETWEEN 1 AND 32),
    delivery            TEXT NOT NULL CHECK (delivery IN ('database', 'files', 'cdc')),
    watermark_kind      TEXT NOT NULL CHECK (watermark_kind IN ('timestamp_column', 'rowversion', 'sequence_id',
                                                                't24_date_time', 'business_date', 'cdc_position',
                                                                'manifest', 'full_compare')),
    watermark_from      TEXT,                    -- NULL on the first unit of a table
    watermark_to        TEXT,
    business_from       DATE,
    business_to         DATE,                    -- the newest closed business date in the unit
    as_of_marker        TEXT,
    mapping_version_id  UUID REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,   -- NULL while landed only
    file_name           TEXT CHECK (file_name IS NULL OR length(file_name) <= 255),
    file_sha256         BYTEA,                   -- required for file delivery, NULL otherwise
    manifest            JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(manifest) = 'object'),
    status              admin.batch_status NOT NULL,
    reason              TEXT CHECK (reason IS NULL OR reason ~ '^[a-z][a-z0-9_]{2,63}$'),
    rows_read           INT NOT NULL DEFAULT 0 CHECK (rows_read >= 0),
    rows_landed         INT NOT NULL DEFAULT 0 CHECK (rows_landed >= 0),
    control_totals      JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(control_totals) = 'object'),
    started_at          TIMESTAMPTZ NOT NULL,
    finished_at         TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (finished_at IS NULL OR finished_at >= started_at),
    CHECK ((status = 'rejected') = (reason IS NOT NULL)),
    CHECK ((file_sha256 IS NOT NULL) = (delivery = 'files')),
    CHECK (status = 'landed' OR mapping_version_id IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_batches_source_started ON admin.batches (source_id, started_at DESC, id DESC);
CREATE INDEX IF NOT EXISTS idx_batches_pending        ON admin.batches (source_id, source_table, started_at)
    WHERE status = 'landed';
CREATE INDEX IF NOT EXISTS idx_batches_mapped_bd      ON admin.batches (source_id, branch_code, business_to)
    WHERE status IN ('mapped', 'partial');
CREATE INDEX IF NOT EXISTS idx_batches_mapping_version ON admin.batches (mapping_version_id)
    WHERE mapping_version_id IS NOT NULL;

-- Rows that did not map (`row_failed`) and rows whose canonical value differs from a settled one (`row_conflict`,
-- returned by the CanonicalWriter, phase 10 section 3.7). Phase 12 owns dq_issues and reads these rows as its
-- source until it exists; this table never holds a value, only codes, column names and the row hash.
CREATE TABLE IF NOT EXISTS admin.quarantine_items (
    id                  UUID PRIMARY KEY,
    batch_id            UUID NOT NULL REFERENCES admin.batches(id) ON DELETE RESTRICT,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL CHECK (source_table ~ '^[A-Za-z][A-Za-z0-9_]{0,40}$'),
    source_pk           TEXT NOT NULL CHECK (length(source_pk) BETWEEN 1 AND 512),   -- DR-41 key tuple joined by '|'
    raw_id              UUID NOT NULL,
    raw_extracted_at    TIMESTAMPTZ NOT NULL,
    kind                TEXT NOT NULL CHECK (kind IN ('row_failed', 'row_conflict')),
    reason_code         TEXT NOT NULL CHECK (reason_code ~ '^[a-z][a-z0-9_]{2,63}$'),
    detail              JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(detail) = 'object'),
    row_hash            BYTEA NOT NULL,
    status              TEXT NOT NULL DEFAULT 'open' CHECK (status IN ('open', 'retried', 'accepted', 'excluded')),
    owner_id            UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    exclusion_reason    TEXT CHECK (exclusion_reason IS NULL OR length(exclusion_reason) BETWEEN 10 AND 2000),
    excluded_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    excluded_at         TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    CHECK ((status = 'excluded') = (exclusion_reason IS NOT NULL AND excluded_by IS NOT NULL AND excluded_at IS NOT NULL)),
    CHECK (kind = 'row_failed' OR status <> 'retried')
);
CREATE INDEX IF NOT EXISTS idx_quarantine_queue   ON admin.quarantine_items (source_id, source_table, status, created_at DESC, id DESC);
CREATE INDEX IF NOT EXISTS idx_quarantine_open    ON admin.quarantine_items (kind, created_at) WHERE status = 'open';
CREATE INDEX IF NOT EXISTS idx_quarantine_batch   ON admin.quarantine_items (batch_id);
CREATE INDEX IF NOT EXISTS idx_quarantine_owner   ON admin.quarantine_items (owner_id) WHERE owner_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_quarantine_excl_by ON admin.quarantine_items (excluded_by) WHERE excluded_by IS NOT NULL;

-- The raw landing layer (DATA-MODEL s.3): one landing table per source table, partitioned by extraction day (UTC).
-- Tables are created by the factory on the first unit of a table, not by a migration: the set depends on the bank's
-- approved objects. Names are `<source_code>__<source_table>` in lower case, so two sources may read a table with
-- the same name. `source_pk` is added to the DATA-MODEL columns so the change detector finds the last landed hash of a
-- key without parsing the payload.
CREATE OR REPLACE FUNCTION raw.landing_table_name(p_source_code TEXT, p_source_table TEXT) RETURNS TEXT
LANGUAGE plpgsql IMMUTABLE SET search_path = pg_catalog AS $$
DECLARE
    v_name TEXT;
BEGIN
    IF p_source_code !~ '^[a-z][a-z0-9_]{1,31}$' OR p_source_table !~ '^[A-Za-z][A-Za-z0-9_]{0,40}$' THEN
        RAISE EXCEPTION 'raw.landing_table_name: invalid source code or table name'
            USING ERRCODE = 'invalid_parameter_value';
    END IF;
    v_name := pg_catalog.lower(p_source_code || '__' || p_source_table);
    IF pg_catalog.length(v_name) > 50 THEN
        RAISE EXCEPTION 'raw.landing_table_name: name too long for a partition name' USING ERRCODE = 'invalid_parameter_value';
    END IF;
    RETURN v_name;
END $$;

-- Creates the parent table, its indexes, the grants (akr_ingest only) and the classification and retention rows.
-- Idempotent. Runs under an advisory lock so two units of the same table cannot race.
CREATE OR REPLACE FUNCTION raw.create_landing_table(p_source_code TEXT, p_source_table TEXT) RETURNS TEXT
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog AS $$
DECLARE
    v_name TEXT := raw.landing_table_name(p_source_code, p_source_table);
BEGIN
    PERFORM pg_catalog.pg_advisory_xact_lock(7307418562110395011);   -- fixed key: 'akr.raw.landing_factory'
    IF NOT EXISTS (SELECT 1 FROM pg_catalog.pg_class c JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace
                   WHERE n.nspname = 'raw' AND c.relname = v_name) THEN
        EXECUTE pg_catalog.format($f$
            CREATE TABLE raw.%I (
                id           UUID NOT NULL,
                source_id    UUID NOT NULL,
                batch_id     UUID NOT NULL,
                source_pk    TEXT NOT NULL,
                extracted_at TIMESTAMPTZ NOT NULL,
                row_hash     BYTEA NOT NULL,
                payload      JSONB NOT NULL CHECK (jsonb_typeof(payload) = 'object'),
                PRIMARY KEY (id, extracted_at)
            ) PARTITION BY RANGE (extracted_at)$f$, v_name);
        EXECUTE pg_catalog.format('CREATE INDEX %I ON raw.%I (source_id, source_pk, extracted_at DESC)',
                                  pg_catalog.left(v_name, 50) || '_key_idx', v_name);
        EXECUTE pg_catalog.format('CREATE INDEX %I ON raw.%I (batch_id)', pg_catalog.left(v_name, 50) || '_batch_idx', v_name);
        EXECUTE pg_catalog.format('CREATE INDEX %I ON raw.%I USING BRIN (extracted_at)',
                                  pg_catalog.left(v_name, 50) || '_time_brin', v_name);
        EXECUTE pg_catalog.format('GRANT SELECT, INSERT ON raw.%I TO akr_ingest', v_name);
        INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification) VALUES
            ('raw', v_name, 'id', 'internal'), ('raw', v_name, 'source_id', 'internal'),
            ('raw', v_name, 'batch_id', 'internal'), ('raw', v_name, 'source_pk', 'personal'),
            ('raw', v_name, 'extracted_at', 'internal'), ('raw', v_name, 'row_hash', 'internal'),
            ('raw', v_name, 'payload', 'personal')
        ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;
        INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note)
        VALUES ('raw', v_name, 'R1', 'partition', 'extracted_at', 'Raw landing, bank-set 30-400 days (ARCHITECTURE s.17)')
        ON CONFLICT (schema_name, table_name) DO NOTHING;
    END IF;
    RETURN v_name;
END $$;

-- Creates one day partition of a landing table (bounds are UTC midnights). The partition name is the table name
-- truncated to 50 characters plus the day, so it stays within the 63-character limit.
CREATE OR REPLACE FUNCTION raw.create_landing_partition(p_source_code TEXT, p_source_table TEXT, p_day DATE)
RETURNS TEXT LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog AS $$
DECLARE
    v_name TEXT := raw.landing_table_name(p_source_code, p_source_table);
    v_part TEXT := pg_catalog.format('%s_%s', pg_catalog.left(v_name, 50), pg_catalog.to_char(p_day, 'YYYYMMDD'));
BEGIN
    IF p_day < DATE '2000-01-01' OR p_day > DATE '2200-01-01' THEN
        RAISE EXCEPTION 'raw.create_landing_partition: day out of range' USING ERRCODE = 'invalid_parameter_value';
    END IF;
    PERFORM pg_catalog.pg_advisory_xact_lock(7307418562110395011);   -- the same key as the table factory
    IF NOT EXISTS (SELECT 1 FROM pg_catalog.pg_class c JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace
                   WHERE n.nspname = 'raw' AND c.relname = v_name) THEN
        RAISE EXCEPTION 'raw.create_landing_partition: % is not a landing table', v_name USING ERRCODE = 'undefined_table';
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_catalog.pg_class c JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace
                   WHERE n.nspname = 'raw' AND c.relname = v_part) THEN
        EXECUTE pg_catalog.format('CREATE TABLE raw.%I PARTITION OF raw.%I FOR VALUES FROM (%L) TO (%L)',
                                  v_part, v_name,
                                  p_day::text || ' 00:00:00+00', (p_day + 1)::text || ' 00:00:00+00');
    END IF;
    RETURN v_part;
END $$;

REVOKE ALL ON FUNCTION raw.create_landing_table(TEXT, TEXT) FROM PUBLIC;
REVOKE ALL ON FUNCTION raw.create_landing_partition(TEXT, TEXT, DATE) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION raw.create_landing_table(TEXT, TEXT) TO akr_ingest;
GRANT EXECUTE ON FUNCTION raw.create_landing_partition(TEXT, TEXT, DATE) TO akr_ingest;

-- Grants (DATA-MODEL s.21, phase 11 column grants). No DELETE for any role on any table here: receipts, quarantine
-- and blind periods are kept, and retention drops only raw partitions (phase 23).
GRANT SELECT, INSERT, UPDATE ON admin.watermarks TO akr_ingest;
GRANT SELECT ON admin.watermarks TO akr_app, akr_readonly;
GRANT SELECT, INSERT ON admin.batches TO akr_ingest;
GRANT UPDATE (status, reason, mapping_version_id, rows_read, rows_landed, control_totals, watermark_to, as_of_marker,
              business_to, finished_at) ON admin.batches TO akr_ingest;
GRANT SELECT ON admin.batches TO akr_app, akr_readonly;
GRANT SELECT, INSERT ON admin.quarantine_items TO akr_ingest;
GRANT UPDATE (status, updated_at, version) ON admin.quarantine_items TO akr_ingest;
GRANT SELECT ON admin.quarantine_items TO akr_readonly;
GRANT SELECT ON admin.quarantine_items TO akr_app;
GRANT UPDATE (status, owner_id, exclusion_reason, excluded_by, excluded_at, updated_at, version)
    ON admin.quarantine_items TO akr_app;
GRANT SELECT, INSERT ON admin.blind_periods TO akr_ingest;
GRANT UPDATE (ended_at, business_to, statement) ON admin.blind_periods TO akr_ingest;
GRANT SELECT, INSERT ON admin.blind_periods TO akr_app;
GRANT SELECT ON admin.blind_periods TO akr_readonly;

-- Classification (DATA-MODEL s.0; phase 00 check). Watermark values are positions, dates and markers: internal.
-- Quarantine keys identify customers, so source_pk is personal.
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('watermarks', 'internal', ARRAY['source_id','source_table','branch_code','kind','value','value_business_date',
                                   'business_date_moved_at','as_of_marker','freshness_class','advanced_at','stale_since',
                                   'stale_reason','halted_reason','halted_at','created_at','updated_at']),
  ('batches', 'internal', ARRAY['id','source_id','source_table','branch_code','delivery','watermark_kind','watermark_from',
                                'watermark_to','business_from','business_to','as_of_marker','mapping_version_id',
                                'file_name','file_sha256','manifest','status','reason','rows_read','rows_landed',
                                'control_totals','started_at','finished_at','created_at']),
  ('quarantine_items', 'internal', ARRAY['id','batch_id','source_id','source_table','raw_id','raw_extracted_at','kind',
                                         'reason_code','detail','row_hash','status','owner_id','exclusion_reason',
                                         'excluded_by','excluded_at','created_at','updated_at','version']),
  ('blind_periods', 'internal', ARRAY['id','scope','source_id','cause','started_at','ended_at','business_from',
                                      'business_to','declared_by','comment','statement','created_at'])
) AS v(t, cls, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification) VALUES
  ('admin', 'quarantine_items', 'source_pk', 'personal')
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

-- Retention (DATA-MODEL s.3 and s.6; the phase 00 CHECK: a real clock_column for partition and age, NULL for
-- retirement and never). Batch receipts are R9 rather than R1: canonical lineage cites batch_id after raw is gone.
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'watermarks',       'R9',  'retirement', NULL,         'Position of each read; follows its source'),
 ('admin', 'batches',          'R9',  'age',        'started_at', 'Receipts outlive raw: canonical lineage cites batch_id (open doubt 1)'),
 ('admin', 'quarantine_items', 'R1',  'age',        'created_at', 'Carries keys of raw rows; follows the raw retention'),
 ('admin', 'blind_periods',    'R10', 'never',      NULL,         'Coverage register: never deleted (ARCHITECTURE s.18)')
ON CONFLICT (schema_name, table_name) DO NOTHING;

### 1.2 Migration `0034_ingest_replays.sql`

```sql
-- migration 0034_ingest_replays.sql
-- Phase 11. Mapping replays over the raw landing layer (API.md 3.6, DR-88, DSL.md B.9 step 6, C-24). A replay is a
-- dry run of a candidate version over the landed rows of a business-date range; `apply` writes the reviewed result.

DO $$ BEGIN
    CREATE TYPE admin.replay_status AS ENUM ('queued', 'running', 'completed', 'failed', 'applying', 'applied');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS admin.mapping_replays (
    id                UUID PRIMARY KEY,
    mapping_id        UUID NOT NULL REFERENCES admin.mappings(id) ON DELETE RESTRICT,
    version_id        UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    document_sha256   BYTEA NOT NULL,                  -- the version document replayed; a later edit makes it stale
    business_from     DATE NOT NULL,
    business_to       DATE NOT NULL,
    source_tables     TEXT[] NOT NULL CHECK (cardinality(source_tables) BETWEEN 1 AND 50),
    status            admin.replay_status NOT NULL DEFAULT 'queued',
    estimate_rows     BIGINT NOT NULL CHECK (estimate_rows >= 0),   -- rows in raw for the range, counted before the run
    -- counts per table: read, unchanged, added, removed, changed per monitored attribute, quarantined; samples are shapes
    diff              JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(diff) = 'object'),
    volume_effect     admin.volume_effect,             -- phase 04 enum; set when the replay completes
    cursor            JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(cursor) = 'object'),   -- resumable position
    error_code        TEXT CHECK (error_code IS NULL OR error_code IN ('source_paused', 'cost_guard', 'version_mismatch',
                                                                       'fact_changes_pending', 'internal')),
    requested_by      UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    applied_by        UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    applied_at        TIMESTAMPTZ,
    started_at        TIMESTAMPTZ,
    finished_at       TIMESTAMPTZ,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    version           INT NOT NULL DEFAULT 1,
    CHECK (business_to >= business_from),
    CHECK ((status IN ('completed', 'applying', 'applied')) = (volume_effect IS NOT NULL)),
    CHECK ((status = 'failed') = (error_code IS NOT NULL)),
    CHECK ((status = 'applied') = (applied_at IS NOT NULL AND applied_by IS NOT NULL)),
    CHECK (status IN ('queued', 'running') OR applied_by IS NOT NULL OR status <> 'applying'),
    CHECK (status NOT IN ('completed', 'failed', 'applied') OR finished_at IS NOT NULL)
);
-- One active replay per version: two runs would race on the same cursor and the same diff.
CREATE UNIQUE INDEX IF NOT EXISTS uq_mapping_replays_active ON admin.mapping_replays (version_id)
    WHERE status IN ('queued', 'running', 'applying');
CREATE INDEX IF NOT EXISTS idx_mapping_replays_version   ON admin.mapping_replays (version_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_mapping_replays_mapping   ON admin.mapping_replays (mapping_id);
CREATE INDEX IF NOT EXISTS idx_mapping_replays_requested ON admin.mapping_replays (requested_by);
CREATE INDEX IF NOT EXISTS idx_mapping_replays_applied   ON admin.mapping_replays (applied_by) WHERE applied_by IS NOT NULL;

GRANT SELECT, INSERT ON admin.mapping_replays TO akr_app;
GRANT UPDATE (status, applied_by, updated_at, version) ON admin.mapping_replays TO akr_app;
GRANT SELECT ON admin.mapping_replays TO akr_ingest;
GRANT UPDATE (status, diff, volume_effect, cursor, error_code, applied_at, started_at, finished_at, updated_at, version)
    ON admin.mapping_replays TO akr_ingest;
GRANT SELECT ON admin.mapping_replays TO akr_readonly;
-- The blind-period declaration reads the newest anchor time (section 3.6, SP-87); a column grant only.
GRANT SELECT (anchored_at) ON audit.audit_anchors TO akr_app;

-- The replay diff holds counts and shapes only (DR-86), but the cursor holds the last key tuple read: personal.
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', 'mapping_replays', c, v.cls::admin.data_classification
FROM (VALUES
  ('internal',        ARRAY['id','mapping_id','version_id','document_sha256','business_from','business_to',
                            'source_tables','status','estimate_rows','error_code','requested_by','applied_by',
                            'applied_at','started_at','finished_at','created_at','updated_at','version']),
  ('detection-logic', ARRAY['diff','volume_effect']),
  ('personal',        ARRAY['cursor'])
) AS v(cls, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

-- R9 with age: a replay is the evidence a mapping change request cites (evidence kind `replay`, phase 04).
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'mapping_replays', 'R9', 'age', 'created_at', 'Evidence of a mapping change request; kept with its version')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

### 1.3 Migration `0035_ingest_settings.sql`

```sql
-- migration 0035_ingest_settings.sql
-- Phase 11. Load, freshness, restore and replay settings, schedules and the two delivery flags.
-- Floors are JSONB objects or NULL (phase 00 convention). Governed keys change through a change request.

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('ingest.history_from', '"2019-01-01"', 'json', NULL, TRUE,
  '{"ar": "أقدم تاريخ عمل يُقرأ في التحميل الأول", "en": "Earliest business date read by the first load"}'),
 ('ingest.slice_days', '31', 'integer', NULL, FALSE,
  '{"ar": "طول شريحة التحميل (أيام عمل)", "en": "Business days per load slice"}'),
 ('ingest.max_unit_rows', '50000', 'integer', NULL, FALSE,
  '{"ar": "أقصى عدد صفوف في وحدة تحميل واحدة", "en": "Maximum rows in one load unit"}'),
 ('ingest.settle_seconds', '120', 'integer', '{"min": 60, "basis": "DSL.md B.2 settle (PT2M) for timestamp and sequence kinds"}', TRUE,
  '{"ar": "مهلة استقرار الصفوف الحديثة (ثوان)", "en": "Settle time for recent rows (seconds)"}'),
 ('freshness.f1_lag_seconds', '900', 'integer', '{"min": 60, "basis": "ARCHITECTURE s.5.5 F1: p95 under 60 s; alarm at 15 minutes"}', TRUE,
  '{"ar": "أقصى تأخر لقراءات الفئة F1 (ثوان)", "en": "Longest read gap for class F1 (seconds)"}'),
 ('freshness.f2_max_minutes', '90', 'integer', '{"min": 60, "basis": "ARCHITECTURE s.5.5 F2: hourly aggregates"}', TRUE,
  '{"ar": "أقصى تأخر لقراءات الفئة F2 (دقائق)", "en": "Longest read gap for class F2 (minutes)"}'),
 ('freshness.f3_max_business_days', '1', 'integer', '{"min": 1, "basis": "ARCHITECTURE s.5.5 F3: end of day within 60 minutes"}', TRUE,
  '{"ar": "أقصى تأخر لبيانات نهاية اليوم (أيام عمل)", "en": "Longest lag of end-of-day data (business days)"}'),
 ('freshness.f5_max_hours', '12', 'integer', '{"min": 12, "basis": "ARCHITECTURE s.5.5 F5: every 12 hours under the 2025 controls"}', TRUE,
  '{"ar": "أقصى تأخر لبيانات المرجع (ساعات)", "en": "Longest lag of reference data (hours)"}'),
 ('freshness.stale_business_days', '2', 'integer', '{"min": 2, "basis": "DR-39: a newest posting that does not move for two business days (proposed)"}', TRUE,
  '{"ar": "أيام عمل بلا حركة قبل إعلان المصدر متأخراً", "en": "Business days without movement before a source is stale"}'),
 ('restore.row_drop_warn_pct', '20', 'integer', NULL, FALSE,
  '{"ar": "نسبة انخفاض الصفوف التي تُنبّه دون إيقاف", "en": "Row drop that warns without halting (percent)"}'),
 ('replay.max_range_days', '92', 'integer', NULL, TRUE,
  '{"ar": "أقصى مدة لإعادة التشغيل (أيام عمل)", "en": "Longest replay range (business days)"}'),
 ('replay.estimate_max_rows', '5000000', 'integer', NULL, TRUE,
  '{"ar": "أقصى عدد صفوف لإعادة التشغيل قبل حارس التكلفة", "en": "Replay row count above which the cost guard refuses"}'),
 ('schedule.ingest_tick', '"0 * * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مجدول التحميل", "en": "Load scheduler tick"}'),
 ('schedule.ingest_file_scan', '"*/30 * * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول فحص مجلد الملفات", "en": "Extract file inbox scan schedule"}'),
 ('schedule.ingest_freshness', '"0 */5 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول فحص الحداثة", "en": "Freshness check schedule"}')
ON CONFLICT (key) DO NOTHING;

-- Delivery flags (phase 03 flag service). Off until the bank has signed the change-capture or extract agreement.
INSERT INTO admin.feature_flags (key, enabled, label) VALUES
 ('ingest.cdc_enabled',   FALSE, '{"ar": "قراءة سجل التغييرات (CT3) مفعّلة", "en": "Change capture (CT3) enabled"}'),
 ('ingest.files_enabled', FALSE, '{"ar": "استلام ملفات الاستخراج (CT4) مفعّل", "en": "Extract file delivery (CT4) enabled"}')
ON CONFLICT (key) DO NOTHING;
```

### 1.4 Inventory of section 1

| Object | Kind | Migration | Owner and grants | Class (retention) |
|-|-|-|-|-|
| `admin.sources` | modified: tier CHECK (CT3), `delivery`, `inbox_key`, `ck_sources_delivery` | 0033 | akr_app and akr_ingest as phase 05 | as phase 05 |
| `admin.blind_periods` | table | 0033 | akr_ingest closes; akr_app declares | R10, never |
| `admin.watermarks` | table | 0033 | akr_ingest writes; akr_app and akr_readonly read | R9, retirement |
| `admin.batches` | table | 0033 | akr_ingest writes; akr_app and akr_readonly read | R9, age `started_at` |
| `admin.quarantine_items` | table | 0033 | akr_ingest writes; akr_app changes status and exclusion | R1, age `created_at` |
| `admin.batch_status`, `admin.replay_status` | enum types | 0033, 0034 | n/a | n/a |
| `raw.<source>__<table>` | landing tables (factory, not migrated) | at first unit | akr_ingest only | R1, partition `extracted_at` |
| `raw.create_landing_table`, `raw.create_landing_partition`, `raw.landing_table_name` | functions | 0033 | EXECUTE for akr_ingest | n/a |
| `admin.mapping_replays` | table | 0034 | akr_app requests; akr_ingest completes | R9, age `created_at` |
| 18 settings, 2 flags | rows | 0035 | n/a | n/a |

Nothing in `canon` changes in this phase: the canonical tables and their grants are phase 10's, and the writer is
called as a library.

## 2. DDD Implementation (Rust)

**Crate placement** (workspace pinned in phase 00; layer rules in `ddd.md`). The planning rules of phase 10 apply: the
pure logic is in `domain`, the IO is in `db` and in the akr-ingestd binary, and the drivers stay in `akr-sources`.

```
backend/
├── crates/
│   ├── domain/    # + ingest: WatermarkKind, Watermark, UnitPlan (pure), ChangeDetector (pure), RestoreDetector (pure),
│   │              #   FreshnessPolicy (pure), ReplayDiffer (pure), BlindPeriod, QuarantineItem, Batch, value objects
│   ├── db/        # + Pg repositories: watermarks, batches, quarantine, blind periods, replays; PgLandingStore (raw.*)
│   ├── sources/   # (akr-sources, phase 05) + fetch_keyset on SourceReader, bank_dates and as_of readers,
│   │              #   the ICS BANKS statement set in the adapter pack (section 3.9)
│   ├── sqlproof/  # unchanged: every generated read is proven before it runs (phase 05, DR-85)
│   ├── authz/     # + resource adapters Watermark, Quarantine, MappingReplay, BlindPeriod, Freshness
│   └── jobs/      # + kinds ingest_tick, ingest_load, ingest_file_scan, ingest_map_pending, ingest_freshness,
│                  #   ingest_resume, ingest_mapping_replay, ingest_mapping_replay_apply (all akr-ingestd)
├── api/           # + routers ingest (9 routes) and replays (3); FreshnessContext middleware; resume extension
└── ingestd/       # + IngestScheduler, LoadUnitRunner, LandingWriter, WatermarkService, RestoreDetectorIo,
                   #   FileScanner, ChangeCaptureConsumer, ReplayRunner, FreshnessService, ResumeHandler
```

The planner, the change detector, the restore detector, the freshness policy and the replay differ are pure functions
in `domain::ingest`. They take values and return values, so each rule of section 3 has a unit test that needs no
database. akr-api links `domain` and `db` but not `akr-sources`; only akr-ingestd links a driver (phase 05 rule).
`BlindPeriodService` is not in the akr-ingestd crate: it is a module of `db`, beside its repository, because akr-api
(declarations), akr-ingestd (the periods the ingest path opens and closes) and, from phase 30, akr-worker and
akr-ctl all call it, and of these four only akr-ingestd links a driver.

**Domain entities** (`Debug, Serialize, Deserialize, FromRow, ToSchema` where they are shown in the API; fields mirror
section 1): `Watermark`, `Batch` (`status` as `#[sqlx(type_name = "admin.batch_status")]`), `QuarantineItem`,
`BlindPeriod`, `MappingReplay` (`status` as `admin.replay_status`).

**Value objects:**
- `WatermarkKind` (the eight kinds of DSL.md B.2), `Delivery` (`Database`, `Files`, `Cdc`), `Tier`, with the rule
  that a kind is allowed only for its delivery: `cdc_position` for `Cdc`, `manifest` for `Files`, the others for
  `Database`.
- `BusinessDateRange { from: NaiveDate, to: NaiveDate }` (inclusive, closed days only; section 3.1).
- `KeyTuple(Vec<String>)` rendered as `source_pk` with `|` (DR-41); `SourcePk`, `RowHash([u8; 32])`.
- `AsOfMarker` (`Scn(u64)`, `Lsn(String)`, `RowVersion(u64)`, `Timestamp(DateTime<Utc>)`, `None`), compared only
  within one engine.
- `FreshnessClass` (`F1`, `F2`, `F3`, `F5`), `StaleReason` (`NotMoving`, `BehindBusinessDate`, `LagCeiling`,
  `NeverLoaded`, `Halted`), `HaltReason` (the four values of section 1.1).
- `RestoreSignal` (`BusinessDateRegressed { branch, observed, watermark }`, `AsOfRegressed { observed, watermark }`,
  `RowDropWarning { table, dropped_pct }`) and `RestoreDecision` (`FalseAlarm`, `Restored { to: BTreeMap<String,
  String> }` keyed by `<source_table>|<branch_code>`, `GapResolved`).
- `ReplayDiff`, `ReplayCounts`, `VolumeEffect` (the phase 04 enum, reused).

**Create/Update structs:** `NewBatch`, `NewQuarantine`, `NewBlindPeriod` (`Deserialize, ToSchema` for the declaration
body), `WatermarkMove { from: Option<String>, to: String, business_to: Option<NaiveDate>, as_of: Option<String> }`,
`NewReplay`, `ReplayCompletion`, `ResumeBody` (section 2 schemas).

**Repository traits** (domain layer; `Pg*` implementations in `db`):

```rust
#[async_trait]
pub trait WatermarkRepository: Send + Sync {
    /// Locks the rows of one table (SELECT ... FOR UPDATE), one per branch.
    async fn lock_table(&self, tx: &mut Tx, source: Uuid, table: &str) -> Result<Vec<Watermark>, RepoError>;
    /// Compare-and-set on `value`: false when another writer moved it (the caller rolls back and reschedules).
    async fn advance(&self, tx: &mut Tx, source: Uuid, table: &str, branch: &str, mv: &WatermarkMove)
        -> Result<bool, RepoError>;
    /// Halts every watermark of a source; returns the number of rows changed.
    async fn halt_source(&self, tx: &mut Tx, source: Uuid, reason: HaltReason, at: DateTime<Utc>) -> Result<u64, RepoError>;
    /// Restore decision: sets each listed watermark to its restored value and clears the halt.
    async fn reset_and_clear(&self, tx: &mut Tx, source: Uuid, to: &BTreeMap<String, String>) -> Result<u64, RepoError>;
    async fn clear_halt(&self, tx: &mut Tx, source: Uuid) -> Result<u64, RepoError>;
    async fn set_stale(&self, tx: &mut Tx, changes: &[StaleChange]) -> Result<Vec<StaleTransition>, RepoError>;
    async fn list_for_source(&self, conn: &mut PgConnection, source: Uuid) -> Result<Vec<Watermark>, RepoError>;
    async fn list_all_active(&self, conn: &mut PgConnection) -> Result<Vec<WatermarkWithSource>, RepoError>;
}

#[async_trait]
pub trait BatchRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, b: &NewBatch) -> Result<Uuid, RepoError>;
    /// landed -> mapped or partial, with the counts of the mapping run; the only status change allowed.
    async fn complete_mapping(&self, tx: &mut Tx, id: Uuid, m: &MappingOutcome) -> Result<(), RepoError>;
    async fn landed_pending(&self, conn: &mut PgConnection, source: Uuid, table: &str, limit: i64)
        -> Result<Vec<BatchRef>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, source: Uuid, f: &BatchFilter, page: Keyset) -> Result<Page<Batch>, RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<Batch>, RepoError>;
    /// data_as_of: the newest business date of a mapped batch per branch.
    async fn data_as_of(&self, conn: &mut PgConnection, source: Uuid) -> Result<Vec<BranchDate>, RepoError>;
}

#[async_trait]
pub trait QuarantineRepository: Send + Sync {
    async fn insert_many(&self, tx: &mut Tx, rows: &[NewQuarantine]) -> Result<u64, RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &QuarantineFilter, page: Keyset) -> Result<Page<QuarantineItem>, RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<QuarantineItem>, RepoError>;
    /// `open` -> `retried`, guarded by `version`; 409 on a race.
    async fn mark_retried(&self, tx: &mut Tx, id: Uuid, expected_version: i32) -> Result<QuarantineItem, RepoError>;
    async fn exclude(&self, tx: &mut Tx, id: Uuid, reason: &str, by: Uuid, expected_version: i32)
        -> Result<QuarantineItem, RepoError>;
}

#[async_trait]
pub trait BlindPeriodRepository: Send + Sync {
    /// None when the source already has an open period (the partial unique index answers).
    async fn open(&self, tx: &mut Tx, b: &NewBlindPeriod) -> Result<Option<BlindPeriod>, RepoError>;
    async fn close(&self, tx: &mut Tx, id: Uuid, ended_at: DateTime<Utc>, business_to: NaiveDate,
                   statement: &serde_json::Value) -> Result<(), RepoError>;
    async fn open_for_source(&self, conn: &mut PgConnection, source: Uuid) -> Result<Option<BlindPeriod>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &BlindFilter, page: Keyset) -> Result<Page<BlindPeriod>, RepoError>;
}

#[async_trait]
pub trait ReplayRepository: Send + Sync {
    /// Partial unique index on the version: a second active replay answers 409 `replay_active`.
    async fn insert(&self, tx: &mut Tx, r: &NewReplay) -> Result<MappingReplay, RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<MappingReplay>, RepoError>;
    /// completed -> applying (akr-api, guarded by version).
    async fn set_applying(&self, tx: &mut Tx, id: Uuid, by: Uuid, expected_version: i32) -> Result<MappingReplay, RepoError>;
    async fn complete(&self, tx: &mut Tx, id: Uuid, c: &ReplayCompletion) -> Result<MappingReplay, RepoError>;
}

/// The raw landing layer. Only akr-ingestd calls it; the tables are named by the factory (section 1.1).
pub trait LandingStore: Send + Sync {
    fn ensure_table(&self, tx: &mut Tx, source_code: &str, table: &str) -> Result<String, RepoError>;
    fn ensure_partition(&self, tx: &mut Tx, source_code: &str, table: &str, day: NaiveDate) -> Result<String, RepoError>;
    /// The last landed hash per key of a chunk: one statement per chunk, never per row.
    async fn last_hashes(&self, conn: &mut PgConnection, source: Uuid, table: &str, keys: &[String])
        -> Result<HashMap<String, RowHash>, RepoError>;
    async fn insert_rows(&self, tx: &mut Tx, table: &str, rows: &[LandedRow]) -> Result<u64, RepoError>;
    /// Keyset read of one batch's raw rows (for pending mapping and for retries).
    async fn read_batch(&self, conn: &mut PgConnection, table: &str, batch: Uuid, after: Option<RawKey>, limit: i64)
        -> Result<Vec<LandedRow>, RepoError>;
    /// Keyset read of a business-date range by extraction time (for replays): `extracted_at` bounds, UTC.
    async fn read_window(&self, conn: &mut PgConnection, table: &str, from: DateTime<Utc>, to: DateTime<Utc>,
                         after: Option<RawKey>, limit: i64) -> Result<Vec<LandedRow>, RepoError>;
    async fn estimate(&self, conn: &mut PgConnection, table: &str, from: DateTime<Utc>, to: DateTime<Utc>)
        -> Result<i64, RepoError>;
}
```

- **Repository notes:** `lock_table` takes `FOR UPDATE` on the watermark rows of one table, so the load unit and a
  resume never move the same position. `advance` is a compare-and-set on `value` (`WHERE value IS NOT DISTINCT FROM
  $from`), so a lost race rolls the unit back. `complete_mapping` is the only status update on `batches` other than
  `rejected`, and it refuses any other transition. `set_stale` returns the transitions only, so the outbox event is
  written once per change. `insert_rows` uses `ON CONFLICT (id, extracted_at) DO NOTHING`. `read_window` and
  `read_batch` order by `(extracted_at, id)`; `RawKey` is that pair.
- **Keyset pagination:** batches by `(started_at DESC, id DESC)`; quarantine by `(created_at DESC, id DESC)`; blind
  periods by `(started_at DESC, id DESC)`; watermarks are not paged (one source has tens of tables).

**Ports** (akr-ingestd side; implemented by the phase 05 driver adapters):

```rust
/// Extends the phase 05 port. The statement is generated from the approved object, proven by `sqlproof` and run
/// inside the engine's read-only transaction mode (DR-61). One range and one page per call.
#[async_trait]
pub trait SourceReader: Send + Sync {
    async fn fetch_keyset(&self, s: &mut SourceSession, read: &ReadSpec, after: Option<KeyTuple>, limit: u32)
        -> Result<Vec<SourceRow>, ReadError>;
    async fn branch_dates(&self, s: &mut SourceSession) -> Result<Vec<BranchBusinessDate>, ReadError>;
    async fn as_of(&self, s: &mut SourceSession) -> Result<Option<AsOfMarker>, ReadError>;
}
pub struct BranchBusinessDate { pub branch: String, pub bank_date: NaiveDate, pub previous: NaiveDate,
                                pub last_batch_date: Option<NaiveDate> }

/// Position-ordered change events written by the change-capture sidecar into the bank's inbox (section 3.4).
pub trait ChangeSpool: Send + Sync {
    fn segments_after(&self, source_key: &str, position: &str) -> Result<Vec<SpoolSegment>, SpoolError>;
    fn read_segment(&self, seg: &SpoolSegment, verify_sha256: bool) -> Result<Vec<ChangeEvent>, SpoolError>;
}

/// Manifests and data files of the extract inbox (section 3.4). `incoming/` is read-only for akr-ingestd; bank files
/// are never moved, renamed or deleted; `mark_processed` writes a done marker under `processed/`.
pub trait ExtractInbox: Send + Sync {
    fn pending_manifests(&self, inbox_key: &str) -> Result<Vec<ManifestRef>, InboxError>;
    fn read_manifest(&self, m: &ManifestRef) -> Result<ExtractManifest, InboxError>;
    fn verify_file(&self, m: &ManifestRef, f: &ManifestFile) -> Result<(), InboxError>;   // sha256 and size
    fn mark_processed(&self, m: &ManifestRef) -> Result<(), InboxError>;
}
```

**Utoipa schemas** (API DTOs; every CBS-derived response also carries `data_as_of` and `stale`, section 3.7):
- `WatermarkView`: `source_table`, `branch_code`, `kind`, `value`, `business_date`, `freshness_class`, `advanced_at`,
  `stale: {since, reason}?`, `halted: {reason, at}?`.
- `BatchView` (list row) and `BatchDetailView` (adds `manifest`, `control_totals`, `file: {name, sha256}?`,
  `mapping_version_id`, `reconciliation: null` until phase 12 fills the rung results).
- `QuarantineView`: `id`, `batch_id`, `source_table`, `kind`, `reason_code`, `detail`, `status`, `owner`,
  `created_at`, `source_pk_hk` (the first 12 hexadecimal characters of the keyed hash of the key, for lists); the
  detail adds `source_pk` in full, and reading it writes an access event `view` (phase 03; normal tier, no purpose
  code needed).
- `QuarantineExcludeBody { reason: String (10 to 2,000 characters) }`.
- `FreshnessView`: `source_id`, `source_code`, `source_table`, `branch_code`, `freshness_class`, `data_as_of`,
  `lag_business_days`, `lag_seconds`, `stale`, `stale_reason`, `halted_reason`, `never_loaded`.
- `BlindPeriodView` (scope, source code, cause, dates, `open`, `statement`), `BlindPeriodDeclaration` (`scope`
  `system` or `rail` only, `cause` `akr_outage`, `rail_outage` or `declared`, `started_at`, `ended_at`, `business_from?`,
  `business_to?`, `comment`). Source periods open on their own (section 3.6); a declaration with scope `source` answers
  422 `E_VALIDATION` `scope_invalid`.
- `ReplayRequest { version_id, business_from, business_to, source_tables?: [string] }`, `ReplayView` (status, dates,
  `diff` as counts, `volume_effect`, `estimate_rows`, `error_code`, `applied_at`), `ReplayAccepted` (`replay_id`,
  `Location`). The apply takes no body: `If-Match` carries the replay's `version`.
- `ResumeBody { restore_decision?: "false_alarm" | "restored" | "gap_resolved", restored_to?: [{source_table, branch_code,
  value}] }`:
  required when a halted source resumes (422 `restore_decision_required` when missing).

**Route table** (base `/api/v1`; every route names its Cedar action, section 3.10; **SU** = step-up assertion,
`X-StepUp-Assertion`, with the class named in section 3.10). Twelve routes are new and one (`POST
/sources/{id}/resume`) is the phase 05 route extended. API.md 3.5, 3.6 and 3.7 give the paths; `GET /freshness` is
new (DR-93).

| Method | Path | Scope | Description (cedar action) |
|-|-|-|-|
| GET | /sources/{id}/watermarks | data.steward | Watermarks per table and branch: kind, value, business date, freshness class, stale and halted state (`watermark.read`) |
| GET | /sources/{id}/batches | data.steward | Batch receipts, newest first; `status?`, `source_table?` filters; keyset (`batch.read`) |
| GET | /batches/{id} | data.steward | One receipt: control totals, watermark move, as-of marker, mapping version, file name and sha256 (`batch.read`) |
| GET | /quarantine | data.steward | Quarantined rows and row conflicts; `kind?`, `status?`, `source_id?`, `source_table?`; keyset (`quarantine.read`) |
| POST | /quarantine/{id}/retry | data.steward | Map the stored raw row again under the published version; `If-Match`; 202 (`quarantine.retry`) |
| POST | /quarantine/{id}/exclude | data.steward | Exclude with a reason; the row is counted in the coverage statement; `If-Match`; **SU** `decision` (`quarantine.exclude`) |
| GET | /freshness | dashboard.read, data.steward | Freshness per source, table and branch: class, `data_as_of`, lag, stale and halted state (`freshness.read`) |
| GET | /coverage/blind-periods | dashboard.read, data.steward | Blind periods with cause, dates and statement; `open?` filter (`blind_period.read`) |
| POST | /coverage/blind-periods | data.steward | Declare a system or rail blind period (an AKR-TMS or rail outage); source periods open on their own (`blind_period.declare`) |
| POST | /mappings/{id}/replays | data.steward | Replay a `testing` or `approved` version over the landed rows of a business-date range; 202 (`mapping_replay.create`) |
| GET | /mapping-replays/{id} | data.steward, config.approve | Status, diff counts per table and monitored attribute, volume effect; samples are shapes only; masked 404 for principals outside section 3.8 (`mapping_replay.read`) |
| POST | /mapping-replays/{id}/apply | data.steward | Apply a completed replay to the published version; `If-Match`; 202; **SU** `backfill` (`mapping_replay.apply`) |
| POST | /sources/{id}/resume | data.steward, security.ops | Phase 05 route, extended: a halted source resumes through `ingest_resume` with `ResumeBody` after a passing grant check; 202; **SU** `backfill` when the source is halted (`source.resume`) |

- **Router functions:** `ingest::router() -> OpenApiRouter<AppState>` (9 routes, tag `ingest`), `replays::router()`
  (3 routes, tag `replays`), both merged under the authz layer in `build_app`. The phase 05 sources router keeps its
  route and its handler gains the halted branch.
- **Idempotency:** the two replay POSTs, the quarantine retry and exclusion, the blind-period declaration and the
  resume take `Idempotency-Key` (phase 00). A replay of the same key returns the stored 202 with
  `x-idempotent-replay: true`.
- **Concurrency:** `POST /quarantine/{id}/retry`, `.../exclude` and `POST /mapping-replays/{id}/apply` take `If-Match`
  on the row's `version`; a missing header is 428 `E_PRECONDITION_REQUIRED`, a mismatch 409 `E_VERSION_CONFLICT`.
- **Freshness context:** every route that returns CBS-derived data (`/sources/{id}`, the batch and quarantine
  routes, the pack and coverage routes of phases 06 and 12) gets `X-Data-As-Of` and `stale` from one `FreshnessContext`
  middleware in akr-api, which reads `watermarks` and `batches` through `FreshnessQuery` with a 30-second cache per
  source.

## 3. Business Logic

### 3.1 The scheduler, the planner and the load unit (`IngestScheduler`, `UnitPlanner`; jobs `ingest_tick`, `ingest_load`; C-13, C-16, C-21)

**Who is read.** A table is read only when a mapping family in state `testing`, `approved` or `published` declares it in
its `change` block (DSL.md B.2: `strategy`, `watermark`, `overlap`, `settle`, `deletes`, `lookback`) and the source
holds it on the approved object list (phase 05 `approved_read_objects`). A table without a declaration is not read:
there are no undeclared reads. A table read by several mapping families is landed once; its watermark is per table
and branch, not per mapping. Testing is enough to start landing because landing is the replay log and needs no
published mapping (section 3.2 step 5); the first publication maps what has already landed (section 3.3).

**`ingest_tick`** (scheduled `schedule.ingest_tick`, every minute; akr-ingestd; concurrency 1; max attempts 1: a missed
tick is repeated by the next one):
1. Select the sources in state `active`. Sources in `halted`, `paused`, `paused_by_bank`, `refused`, `registered` or
   `disabled` are not read here; their checks belong to phase 05 (health, probe, resume).
2. For each active source and each declared table, decide whether the table is due:
   a. never read (no watermark row with a `value` yet), or
   b. a closed business date exists for a branch after its `value_business_date` (the `branch_dates` read of the
      health job, phase 05 section 3.5), or
   c. the last successful read (`advanced_at`) is older than 60 minutes (a heartbeat for re-reads and for a source
      whose business date is not readable).
   A due table gets an `ingest_load {source_id, source_table}` job with coalesce key `ingest_load:<source_id>:<source_table>`.
3. For each landing table, ensure the day partitions of today and tomorrow (UTC) exist through
   `raw.create_landing_partition`; the landing path also creates a missing partition inline and retries once.
4. For each (source, table) with a `batches` row in `landed` and a published version, enqueue `ingest_map_pending`
   (section 3.3).
5. No audit event (no business data changes); counters `akr_ingest_due_tables{source}`.

**`ingest_load {source_id, source_table}`** (akr-ingestd; concurrency 4 across sources; one per table by its coalesce
key; max attempts 5 with the phase 00 back-off). The job stops when `ingest.max_unit_rows` rows have been read in the
run, or when the governor returns a window or blackout, and then reschedules itself with `run_at` at the next opening
(a discovery job never waits holding a session, phase 05 section 3.5):
1. Load the source and its budget; `LoadGovernor::for_source` (phase 05). Not `active`: end without retry
   (`GovernorError::Paused`).
2. Read the watermark rows of the table (no lock yet) and the branch dates.
3. `UnitPlanner::plan(watermarks, kind, overlap, lookback, lag_days, lookback_max_days, sequence_overlap_keys, settle, history_from, closed_dates, slice_days, max_unit_rows)`
   (pure). The rules:
   a. Start. For `business_date`: the first business date after `value_business_date`, minus `overlap` business days
      (the re-read window; the mapping's `overlap`, default 31 days, DR-37). For `timestamp_column`, `t24_date_time`,
      `rowversion`: the value minus the overlap as a duration. For `sequence_id`: from the stored key tuple, no
      overlap (a sequence needs none; a restart is a restore, section 3.5). With no value yet: `ingest.history_from`
      (first load; the bank can set it through a change request).
   b. End. The last closed business date of each branch (`BANK_DATE - 1`, adapter pack section 3.9). Dates are closed
      days only (DR-89). A closed day with no rows is an empty unit: the watermark still moves, because the bank
      declared the day closed; a branch with no postings for days is a coverage fact for phase 12, not a read error.
   c. Slices. A range longer than `ingest.slice_days` is cut into slices, oldest first. A slice whose estimated rows
      exceed `ingest.max_unit_rows` is cut by business date until one day fits.
   d. Output: an ordered list of `UnitPlan { branch, range, kind }`. An empty list is "caught up" and the job ends.
4. For each unit in order: `LoadGovernor::acquire(Read)` before each page (phase 05 section 3.5), keyset pages of
   `rows_per_fetch` through `SourceReader::fetch_keyset` (proven by `sqlproof`, one table, one range, no join). A
   statement that exceeds `statement_timeout_ms` is cancelled and the unit is recorded with reason `unit_timeout`; the
   range is not widened and the job retries the same unit later (phase 05 rule).
5. Restore check for the branch dates and the as-of marker just read (section 3.5). A signal stops the unit before any
   write.
6. Change detection and the landing transaction (section 3.2). The unit ends with its batch receipt.
7. The governor's slow-down (phase 05) lowers the page rate; the job never raises it.

### 3.2 The landing transaction (`LandingWriter`, `ChangeDetector`; C-20, C-23, C-24)

One PostgreSQL transaction per unit (or per page of a day larger than `ingest.max_unit_rows`, section below), as
`akr_ingest`. A page commit writes its batch receipt with the watermark unchanged, and the page that completes the
range moves it. Every step is in the same transaction, so a crash leaves either all of a unit or none of it (C-20):
1. `WatermarkRepository::lock_table` (`FOR UPDATE` on the table's rows). If the stored `value` differs from the unit's
   `from` (another writer or a resume moved it): roll back, end with reason `lost_race`, reschedule.
2. Change detection (`ChangeDetector`, pure). Each source row is canonicalised: the adapter deny-list columns are
   dropped at extraction (DATA-MODEL s.3; ICS BANKS in section 3.9), free-text card-like digit runs are masked (phase
   10 card guard), keys are formed as a tuple joined by `|` (DR-41), and `row_hash` is SHA-256 over the canonical JSON
   of the remaining payload with sorted keys. Per chunk of 1,000 keys, `LandingStore::last_hashes` returns the last
   landed hash of each key in one statement. A row is `changed` when its key is new or its hash differs; otherwise it
   is `unchanged` and is not landed. A row that disappeared is a delete only when the mapping declares `deletes:
   key_reconcile` (a separate keyed comparison under the budget); it lands as an event with `op = 'd'`.
3. `LandingStore::ensure_partition` for the unit's extraction day (one UTC timestamp per unit, passed explicitly).
4. `LandingStore::insert_rows`: the changed and deleted rows, with `id` (UUIDv7), `source_id`, `batch_id`, `source_pk`,
   `extracted_at`, `row_hash` and `payload`. This is the replay log (C-24): raw rows are never updated.
5. Mapping, when the table has a published version (the newest one at the time of the job):
   `MappingExecutor::map(version, rows)` (phase 10, pure transforms) returns `MappedBatch` with row failures
   (transform error, required attribute missing, unmapped currency: reason codes from phase 10) and counts of
   unmapped codes (DSL.md B.7: `unknown` plus a count; phase 12 owns the data-quality issue). Then
   `CanonicalWriter::write(tx, &mapped)` (phase 10 section 3.7) returns `WriteOutcome`. Each `RowConflict` becomes a
   `quarantine_items` row of kind `row_conflict` with reason `fact_changed_in_source` (a settled fact whose source hash
   changed; the new version stays in raw), `value_change` or `label_change` (dimensions). Each
   row failure becomes a `row_failed` row with its reason code. Quarantine rows hold codes, column names and the
   row hash, never a value (section 1.1). From phase 12 the same transaction then calls `DataQualityRecorder::record_batch`, which counts these conflicts and failures into `dq_issues` (DR-97).
   Without a published version the step is skipped and the receipt is `landed`: canon is untouched and nothing is lost.
6. `BatchRepository::insert` with the receipt: `status = mapped` (no quarantined rows), `partial` (some rows
   quarantined) or `landed` (no published version; `mapping_version_id` NULL). `control_totals` holds `read`,
   `unchanged`, `landed`, `inserted`, `unchanged_in_canon`, `versions_closed`, `deletes`, `quarantined: {row_failed,
   row_conflict}`, `unmapped_codes` and `as_of`. `business_to` is the newest closed date of the unit.
7. `WatermarkRepository::advance` (compare-and-set on `value`; sets `advanced_at = now()` always, `value_business_date`
   and `business_date_moved_at` when the business date moved forward, and `as_of_marker`). A `landed` receipt moves the
   watermark too: landing is what the watermark tracks; `data_as_of` follows mapped receipts only (section 3.7).
8. Audit `batch.recorded {batch_id, source_id, source_table, branch_code, status, rows_read, rows_landed,
   watermark_to}` (counts only). The writer emits `canon.written` for each entity (phase 10).
9. Commit. Any error that is not a row error (`E_INTERNAL` from the writer, a database failure) rolls the transaction
   back and writes a separate receipt with status `rejected` and reason `writer_failed` (the watermark is untouched);
   the job retries with the phase 00 back-off, up to five attempts.

**Page commits.** When a single closed day is larger than `ingest.max_unit_rows`, the unit is committed page by page.
Each page writes its receipt with `watermark_from = watermark_to`. Only the page that completes the day moves the
watermark. A crash between pages re-reads the day from its start; the change detector makes the re-read idempotent
(unchanged rows are not written).

### 3.3 Mapping pending and quarantine retry (job `ingest_map_pending`; C-24)

Payload `{source_id, source_table, version_id}` for pending batches, or `{quarantine_ids}` for a retry. Coalesce key
`ingest_map_pending:<source_id>:<source_table>`; concurrency 2; max attempts 5.

1. **Pending batches.** `BatchRepository::landed_pending` returns `landed` receipts of the table, oldest first. For each
   receipt, up to `ingest.max_unit_rows` rows in total per run: read its raw rows (`LandingStore::read_batch`, keyset),
   map them under the newest published version, write through `CanonicalWriter` in one transaction, and call
   `BatchRepository::complete_mapping` with `mapped` or `partial`, the version id and the counts. The watermark is not
   touched: it moved at landing.
   In the same transaction, when the writer inserted or changed any row, the outbox event `canon.touched {batch_id,
   source_id, source_table, entity_kinds, business_date}` is written (the topic phase 00 reserves for this phase; ids
   only, never row values). Its consumers register later: phase 14 onboarding screening, phases 16 and 17 detection
   (lead addition at the phase 14 review, 2026-10-09).
2. **Retries.** For each quarantine id of kind `row_failed` and status `open` or `retried`: read the raw row by
   `(raw_id, raw_extracted_at)`, map it under the newest published version, write it. Success: the quarantine row
   becomes `accepted`. Failure: a new `row_failed` row is written for the new reason and the old row stays `retried`.
   A `row_conflict` cannot be retried; its resolution is phase 12's (route 409 `conflict_needs_review`).
3. Idempotent: a receipt whose `mapping_version_id` already equals the version is skipped; the writer's hash guard
   covers the rest.
4. **The first publication is not a replay.** Landed receipts of a family mapped for the first time are an ordinary
   mapping run: no diff, no approval beyond the publication itself (DR-88: the first publication has a neutral volume
   effect). A later publication does not re-map history: each receipt keeps the version it ran under (DSL.md B.9
   step 5). Re-interpreting history is the replay of section 3.8, and only that.

### 3.4 The three delivery modes (database, change capture, extract files; C-54, C-62, DR-63)

The delivery of a source is fixed by its tier (`sources.delivery`): `database` for CT1, CT2 and a CT4 landing database,
`cdc` for CT3 (only with `ingest.cdc_enabled`; the bank's signed agreement is the reason a person turns the flag on, the audited flag change is its record in the product, and no step checks the paper), `files` for CT4 extract files (only with
`ingest.files_enabled`). The watermark kind of each table comes from its mapping's `change` block and must match the
delivery (the `WatermarkKind` rule of section 2).

**Database delivery** (poll reads through `SourceReader::fetch_keyset`):

| Kind | Read | Watermark value | Freshness class |
|-|-|-|-|
| `business_date` | closed business days per branch, from the overlap start | last closed business date read in full (`value_business_date`) | F3 |
| `timestamp_column`, `t24_date_time` | keyset on the index, from value minus overlap; rows younger than `ingest.settle_seconds` wait | last key tuple read | F3 (F2 when the mapping declares it) |
| `sequence_id` | keyset on the key tuple from the stored value | last key tuple read | F3 |
| `rowversion` | from the stored binary value, shown as hexadecimal text | last value read | F3 |
| `full_compare` | full keyset pages of the table; rows compared by hash; deletes by key reconciliation when declared | the snapshot's completion marker (its start time) | F5 |

For `full_compare` the snapshot is kept as the audit trail (DSL.md B.2): every page's raw rows are the snapshot, and the
completion marker is written once per complete pass.

**Change capture, CT3** (`delivery = cdc`):
1. The change-capture sidecar (Debezium Server, compose profile `cdc`, section 4.2) runs in the source's own network
   zone and writes position-ordered JSON Lines segments into the source's inbox (`<inbox_root>/<inbox_key>/cdc/`), one
   file per segment with a `.sha256` file beside it. The file name carries the first and last position and a sequence
   number. akr-ingestd reads the spool through `ChangeSpool`; it never connects to the CBS for changes.
2. `ingest_load` for a `cdc_position` table reads the segments after the stored position, verifies each segment's
   SHA-256 and checks continuity: the sequence number of the next segment must be the successor of the last one read,
   and its first position must follow the stored position. A gap or a repeated sequence with a different hash halts the
   source with reason `cdc_gap` (section 3.5), and nothing of the segment is written.
3. Each event carries `op` (`c`, `u`, `d`), the key tuple, the `after` image and its position. Inserts and updates pass
   through change detection and the landing transaction. Deletes land as events, are counted in `deletes` and produce no
   canonical change in this phase (C-23: the monitored copy keeps the row).
4. The watermark is the last position; its freshness class is F1 and `advanced_at` is the heartbeat of every successful
   poll, empty or not. The job reschedules itself 30 seconds after a unit that read events, so change capture does not
   wait for the tick.

**Extract files, CT4** (`delivery = files`; job `ingest_file_scan`, scheduled `schedule.ingest_file_scan`, every 30
seconds, concurrency 1, max attempts 3; it processes the manifests of every file source):
1. The bank drops, for each table and branch, one or more data files and one manifest into `<inbox_root>/<inbox_key>/
   incoming/`. `incoming/` is read-only for akr-ingestd; only `processed/` is writable (section 4.2).
   Its manifest (schema `akr.extract.v1`) names `source_code`,
   `source_table`, `branch_code`, `business_from`, `business_to`, `sequence`, the files (`name`, `sha256`, `bytes`,
   `rows`), the control totals and `produced_at`.
2. Per (table, branch), in sequence order: a manifest whose `sequence` is `value + 1` is a unit; one whose sequence is
   at or below `value` with the same file hashes is a duplicate and is marked done; the same sequence with other hashes
   is `file_hash_mismatch` (batch `rejected`, alarm, no halt); a sequence above `value + 1` halts the source with reason
   `file_gap` (the missing sequence is named in the statement).
3. The unit verifies each file's size and SHA-256 (`ExtractInbox::verify_file`), parses the rows with the column list
   of the manifest (`columns`; the extract file contract of section 7+), and checks the row count and the control totals against the manifest. A mismatch rejects
   the batch with reason `control_total_mismatch`; the manifest stays pending for the bank to correct.
4. Section 3.2 runs with `watermark_kind = manifest` and the sequence as the value (compare-and-set). After the commit
   akr-ingestd writes `processed/<manifest>.done` beside the manifest. It never moves, renames or deletes a bank file.

### 3.5 Restore and failover detection, halts and resume (`RestoreDetector`, `ResumeHandler`; C-64, C-03)

**The signals** (pure `RestoreDetector::evaluate(watermarks, branch_dates, as_of, previous_as_of, endpoint_changed)`):
1. **Business date regressed.** For a `business_date` watermark of a branch, the CBS's last closed date
   (`bank_date - 1`) is earlier than `value_business_date`. A branch's business date never moves backwards in normal
   operation (DR-39: a per-branch `PROCESS.BANK_DATE`). Signal `BusinessDateRegressed`.
2. **As-of marker regressed.** The engine's as-of marker (SCN, LSN, row version; DSL.md B.2 and C-20) is lower than the
   marker of the previous unit. Signal `AsOfRegressed`. The cause is `failover_suspected` when `endpoint_changed` is
   true (the instance or server identity recorded by the last grant check differs, phase 05 `engine_facts`), otherwise
   `restore_suspected`. An engine that offers no marker (`none`) produces no as-of signal, and the business-date rule
   stands alone.
3. **Row drop.** A `full_compare` table whose row count falls by more than `restore.row_drop_warn_pct` percent
   produces `RowDropWarning`. A warning only: deletes are normal (DSL.md B.2), and the batch control totals carry the
   drop for the steward.

**The halt path** (one transaction, run by the unit that detected the signal, which then writes nothing else):
1. Conditional update of `admin.sources.state` from `active` to `halted` (phase 05 state machine; a losing race ends
   the job).
2. `WatermarkRepository::halt_source(reason)` on every watermark of the source, with `halted_at = now()`. The reason is
   `business_date_regressed` or `as_of_regressed` according to the signal that fired; every table of the source takes it,
   because a restore affects all of them. The cause (`restore_suspected` or `failover_suspected`) goes to the blind period
   and the audit event, not to the watermark.
3. `BlindPeriodRepository::open` (scope `source`, cause `restore_suspected` or `failover_suspected`, `started_at = now()`,
   `business_from` = the earliest `value_business_date` of the source plus one day (`ingest.history_from` when no table of the source has one yet), `statement = {signal, observed,
   expected, tables}`). Section 3.6.
4. Audit `source.restore_suspected {signal, observed, expected, cause}` and `source.state_changed {from: active, to:
   halted, cause}` (the phase 05 event). Outbox `ingest.halted {source_id}`, read by the SSE channel `ingest.status`
   (phase 00 SSE), and `NOTIFY akr_source {what: state}`.
5. The receipt of the unit is written as a separate transaction with status `rejected` and reason `restore_suspected`
   (or `failover_suspected`): the evidence of what was seen is kept, and no row of the unit is written.
6. The alarm to the MLRO and to the bank's DBA (C-64) is the outbox event. Its delivery as an inbox task is phase 18;
   until then the SSE banner and the alert rule of section 4.3 carry it.

**Other halts.** `cdc_gap` and `file_gap` (section 3.4) use the same path without the `source.restore_suspected` event (step 4 audits `source.state_changed` with cause `delivery_gap` only, and the receipt of step 5 has reason `cdc_gap` or `file_gap`), with `halted_reason` `cdc_gap` or `file_gap`
and a blind period of cause `delivery_gap`. No restore decision applies to them.

**Resume** (`POST /sources/{id}/resume`, the phase 05 route extended; the route checks the state and the latest grant
check, then enqueues `ingest_resume {source_id, restore_decision?, restored_to?, requested_by, check_id}`, coalesce key
`ingest_resume:<source_id>`, concurrency 1, max attempts 1 because a refused check is a result):
1. Source state `halted` (else the phase 05 paused path: `discovery_probe` with trigger `resume`, unchanged). Route
   checks: a halted source needs `restore_decision`; `false_alarm`, `restored` or `gap_resolved` per reason:
   - a restore or failover suspicion (blind period cause `restore_suspected` or `failover_suspected`; watermark reason
     `business_date_regressed` or `as_of_regressed`): `false_alarm` or `restored` (`gap_resolved` answers 422
     `restore_decision_invalid`).
   - `cdc_gap`, `file_gap`: `gap_resolved` only.
2. The job runs `ConnectionGuard::connect(source, CheckTrigger::Resume)` (phase 05 section 3.3): a fresh grant check, which
   is C-03 for a source that was halted. A refusal moves the source to `refused` and ends the job (result, no retry).
3. Decision:
   - `false_alarm`: the bank explains the regression (for example a corrected view). The as-of marker of section 7.13 is written when the halt was `as_of_regressed`. Halts are cleared; no watermark value
     moves.
   - `restored`: `restored_to` names a value per table and branch; it must not be later than the stored value (422
     `restored_to_after_watermark`), and every watermark of the source must be named (422 `restored_to_missing`). The
     watermarks are reset (`reset_and_clear`) and audited as `watermark.reset {tables, from, to}`. Rows after the
     restored point are read again and compared by hash: a settled fact whose hash changed becomes a `row_conflict`
     quarantine row and is not overwritten; a row that the restored CBS no longer has stays in canon and in raw
     (C-64: rows that disappeared are kept).
   - `gap_resolved`: the bank re-delivered the missing positions or files; halts are cleared; the table is read from its
     stored position again.
4. Conditional update of the state from `halted` to `active`; audit `source.halt_resolved {decision, tables, check_id}` (plus `as_of_marker` on a `false_alarm` after an `as_of_regressed` halt, 7.13; `check_id` is the fresh check of step 2; the id in the job payload is the check the route read)
   and `source.state_changed {from: halted, to: active, cause: decision}`; outbox `ingest.resumed`.
5. The open blind period stays open until catch-up is complete (section 3.6). Catch-up is automatic (C-16): the normal
   load loop reads from the stored (or reset) position, in slices of `ingest.slice_days` under the budget. No backfill
   plan is needed for the gap; the operator-planned backfill of API.md 3.5 (phase 12) is for explicit slices only.

**Restore drill evidence.** The rules above are tested on the fixture CBS by moving the fixture's `PROCESS` date
backwards and by replaying an older SCN (section 6, item 4). A restore test that passes without a halt fails the phase.

### 3.6 Blind periods (`BlindPeriodService`; C-27, SP-87)

A blind period is the coverage gap of a source or of AKR-TMS itself: the time in which the system could not see the
data it was meant to watch. It is recorded from the moment the gap is known, never removed, and carries its cause and
dates for the MLRO statement (phase 12).

**Automatic opens** (source scope; the `BlindPeriodRepository::open` call is in the transaction of the event):
- A phase 05 pause (`POST /sources/{id}/pause`, any cause): cause `source_paused`, `started_at` = the pause's start,
  `statement.pause_cause` the phase 05 cause.
- A bank pause (`paused_by_bank`, phase 05 section 3.5): cause `source_paused`, `statement.pause_cause` as the engine
  reported it (`account_locked`, `credentials_rejected`, and so on).
- A refusal (`refused`, phase 05 section 3.1): cause `source_paused`, `statement.pause_cause = grant_refused`.
- A halt (section 3.5): `restore_suspected`, `failover_suspected` or `delivery_gap`.

While a source period is open, a second cause extends its `statement` (the causes list grows) and does not open
another period (the partial unique index).

**Automatic close.** The freshness job (section 3.7) closes an open source period when the source is `active` and no
watermark of the source is behind: for each branch of a business-date table the stored business date is the last closed
date, and for each other table the last read reached the current range end. The close sets `ended_at = now()` and
`business_to` = the newest business date landed during the period, and writes `statement` with `{cause, started_at,
ended_at, business_from, business_to, tables, rows_landed_in_catch_up, halts}`. Audit `blind_period.closed`.

**Declarations** (`POST /coverage/blind-periods`, data steward, Cedar `blind_period.declare`): scope `system` or `rail`
only, cause `akr_outage`, `rail_outage` or `declared`, `started_at` (not in the future; not earlier than the newest
`audit_anchors.anchored_at` of phase 02, so a declaration cannot reach back past sealed evidence: SP-87), optional
`business_from` and `business_to`, a required `ended_at` (not in the future, not before `started_at`; a declaration records a period that is over, so no declared period stays open and none needs a close route) and a `comment` of 10 to 2,000 characters. The row records the true start;
the alerts produced when the data catch up are flagged with their creation time (SP-87, phases 16 and 17). Audit
`blind_period.declared {scope, cause, started_at, ended_at}`.

**Visibility.** The register is read by the MLRO, the compliance head, internal audit and the steward (Cedar
`blind_period.read`). The MLRO statement and the banner on a case that touches a blind period (C-27) are phase 12.

### 3.7 Freshness and `data_as_of` (`FreshnessService`, `FreshnessPolicy`, job `ingest_freshness`; C-29, DR-39)

**Class.** Each watermark carries its freshness class (`freshness_class`, section 1.1), set on its first unit from the
kind and the mapping's declaration: `cdc_position` is F1; `full_compare` is F5; `manifest` is F3, or F5 when the mapping
declares a reference table; the other kinds are F3, or F2 when the mapping declares an intraday aggregate. Class F0
(assessment latency) belongs to phase 2, and F4 (calendar outputs such as the monthly liaison report) is asserted by its
consumers in phases 17 to 21; this phase measures F1, F2, F3 and F5 of the loads.

**`data_as_of`.** The business date of the newest **mapped** receipt of each branch (`business_to` of receipts with
status `mapped` or `partial`), never the AKR server clock (DR-39). For tables without a branch, the newest receipt of the
table. Null when nothing is mapped yet, which is a state, not an error (section 3.7 rule b).

**Rules** (pure `FreshnessPolicy::classify(watermark, branch_bank_date, now, settings)`, first match wins):
a. `halted_reason` set -> stale, `halted`.
b. `value` null -> stale, `never_loaded`. A fresh install is stale until its first unit: correct, not an error.
c. F1: `now - advanced_at` above `freshness.f1_lag_seconds` -> stale, `lag_ceiling`.
d. F2: `now - advanced_at` above `freshness.f2_max_minutes` -> stale, `lag_ceiling`.
e. F3: business days between `data_as_of` and the CBS's last closed date above `freshness.f3_max_business_days` -> stale,
   `behind_business_date`. Days are counted with `BankCalendarClock` (phase 09): weekends and holidays per currency are
   not lag (DR-79).
f. F3 with kind `business_date`: `value_business_date` unchanged (`business_date_moved_at` older than the count of
   `freshness.stale_business_days` business days) while the CBS's date moved -> stale, `not_moving`. This is the rule of
   DR-39: the newest posting stops moving, and the source is stale with its age shown. The ICS BANKS snapshot of section
   3.9 is stale by this rule.
g. F5: `now - advanced_at` above `freshness.f5_max_hours` -> stale, `lag_ceiling`.
Otherwise fresh.

**Source state.** A source is as fresh as its stalest watermark. The API shows the worst state per source and the state
per table and branch (`GET /freshness`).

**Transitions** (once per change, in one transaction per run): `WatermarkRepository::set_stale` returns the changes;
an entry into stale writes `stale_since`, `stale_reason`, audit `freshness.stale_entered {source_id, source_table,
branch_code, class, reason, data_as_of}` and the outbox event `dq.stale` (DSL.md A.3; the event name is the DSL's); a
return to fresh clears both and audits `freshness.recovered`. Blind periods close as in section 3.6.

**Read path.** `FreshnessQuery` answers the `data_as_of` of a set of sources from `watermarks` and `batches` with a
30-second cache per source. Every CBS-derived response carries `X-Data-As-Of` (the oldest `data_as_of` among the sources
the response draws on) and `stale` (true when any of those sources is stale), and the body carries the per-source
values (API.md section 1).

**Ingest lag.** Histogram `akr_ingest_lag_seconds{source, class}` observes, per batch, the time from extraction to
receipt; it is the ingest-lag histogram that C-29 asks for, published per source (section 4.3). The objectives are the
ARCHITECTURE s.5.5 F1 p95 under 60 seconds and F3 within 60 minutes of the CBS end of day; the alarm thresholds are the
settings of section 1.3.

### 3.8 Mapping replays over the raw layer (`ReplayService` in akr-api, `ReplayRunner` in akr-ingestd; C-24, DR-88)

A replay answers one question before a mapping change is approved or published: which canonical values would change
if the candidate version ran over the rows already landed, and by how much. A replay reads the landing layer only, never
the CBS, so it needs no source session and no load budget. It writes nothing to canon until `apply`.

**Order of use** (DSL.md B.9, with the DR-88 rule): draft, validate, test (`testing`), replay (dry, on the `testing` or
`approved` version), change request with the replay as evidence (phase 04 evidence kind `replay`), publication, then
apply (on the `published` version). A first publication needs no replay (section 3.3 step 4).

**`POST /mappings/{id}/replays`** (akr-api; Cedar `mapping_replay.create`; `Idempotency-Key`; 202):
1. Load the mapping and the version. The version must be `testing` or `approved`; otherwise 409
   `E_NOT_ALLOWED_IN_STATE` with reason `version_not_candidate`.
2. `business_from <= business_to` (422 `range_inverted`); the range is at most `replay.max_range_days` business days
   (422 `range_too_long`); every `source_tables` entry is read by the version (422 `table_not_in_version`).
3. Estimate: `BatchRepository::rows_landed_in` sums `rows_landed` over the receipts of each table in the window (converted to UTC from Baghdad
   days). Above `replay.estimate_max_rows`: 422 `E_COST_GUARD` with the estimate in the body (API.md s.6).
4. A replay of the same version already `queued`, `running` or `applying`: 409 `E_NOT_ALLOWED_IN_STATE` with reason
   `replay_active` (the partial unique index answers).
5. Insert the row (`status = queued`, `document_sha256` of the version document, `estimate_rows`); enqueue
   `ingest_mapping_replay {replay_id}`; audit `mapping_replay.requested {mapping_id, version_id, business_from,
   business_to, source_tables, estimate_rows}`; 202 with `Location: /mapping-replays/{id}`.

**`ingest_mapping_replay {replay_id}`** (akr-ingestd; concurrency 1 per version; max attempts 3; resumable through
`cursor`):
1. `queued` -> `running` (`started_at`). If the version's document hash differs from `document_sha256` (an edit
   happened after the request), the replay fails with `version_mismatch`.
2. For each table, read the landed rows of the window (`LandingStore::read_window`, keyset by `(extracted_at, id)`, pages
   of `rows_per_fetch`). After each page the `cursor` is stored.
3. Map each row under the candidate version (`MappingExecutor::map_payload`, the pure transforms of phase 10 applied to
   the stored payload). Read the canonical current row of the source key (`xref`, then the current version of the
   canonical row; `akr_ingest` reads canon).
4. `ReplayDiffer` (pure) classifies each row: `unchanged`; `added` (no canonical row now, a row that maps); `removed`
   (a canonical row that the candidate no longer produces; counted, never deleted, C-23); `changed` per monitored
   attribute (the `monitored` flags of `canonical-model.v1.json`, phase 10); `fact_changed` (a settled fact whose mapped
   values differ: facts are not overwritten, so this needs a correction, see section 3.8 apply); `failed` (row failures of
   the candidate).
5. `diff` accumulates counts per table and per monitored attribute, and per attribute a shape: a histogram of value
   lengths and classes (DR-86: counts and shapes only, never a value; the masking of phase 10 applies). No sample holds a
   value.
6. Complete: `volume_effect = neutral` when every `changed`, `added` and `removed` count over the monitored entities is
   zero; otherwise `unknown`. `unknown` cannot be approved as material (phase 04 section 3.1), and it stays so until the
   scenario replay of phase 16 supplies alert counts. Status `completed`, `finished_at`, audit `mapping_replay.completed
   {counts, volume_effect}`.
7. Failure: status `failed` with `finished_at` and `error_code` (`version_mismatch`, `internal`, `cost_guard` when the job's own
   estimate, taken before its first page, is above `replay.estimate_max_rows`); audit `mapping_replay.failed {error_code}`.

The phase 04 evidence kind `replay` resolves to this row when it is `completed` and its `document_sha256` equals the
version the change request proposes. The phase 10 change target `mapping` reads the `volume_effect` from the completed
replay for that version, instead of its own rule, when a replay exists (DR-92; the first publication keeps
phase 10's neutral rule without a replay).

**`GET /mapping-replays/{id}`** (Cedar `mapping_replay.read`). A replay of a version that is not `published` is visible
to its requester, to role M, and to a checker (`config.approve`) who holds an open change request for that version (the
replay is the evidence the checker needs). Every other principal receives the masked 404 of API.md s.1, identical to an
unknown id, because the diff of an unapproved change reveals what the bank's detection would see before anyone has
approved it (detection-logic, DR-92). The API computes `approver_ids` from the open change requests of the version
and the Cedar policy reads it. A published version's replay is visible to every principal with the scope.

**`POST /mapping-replays/{id}/apply`** (akr-api; Cedar `mapping_replay.apply`; SU class `backfill`; `If-Match` on the
replay's `version`; `Idempotency-Key`; 202):
1. Status `completed` (else 409 `replay_not_completed`; `applied` gives 409 `replay_already_applied`). The version is
   `published` (else 409 `version_not_published`) and its document hash equals `document_sha256` (else 409
   `version_mismatch`; the replay must be run again).
2. `diff.fact_changed` is zero (else 409 `fact_changes_need_correction`). The apply does not change settled facts: a
   changed fact is a correction, which arrives as a new row linked to the original (DATA-MODEL s.0; open doubt 2).
3. `ReplayRepository::set_applying` (guarded by `version`); audit `mapping_replay.apply_requested {replay_id}`; enqueue
   `ingest_mapping_replay_apply {replay_id}`.

**`ingest_mapping_replay_apply {replay_id}`** (akr-ingestd; concurrency 1 per version; max attempts 3; resumable):
1. For each page of the window, under the published version: the mapping and the writer of section 3.2 step 5 over the landed rows of the window, read in `(extracted_at, id)` order with `LandingStore::read_window` (apply lands no raw row, so steps 2 to 4 of section 3.2 do not run), without the
   watermark step, in one transaction per page. The writer inserts new keys, opens new bi-temporal versions of changed
   dimension and party values (valid from the row's own business date, DR-39, and recorded now), and writes no delete.
2. Receipts per page: `delivery` of the table, `mapping_version_id` set, `status` `mapped` or `partial` by the rule of section 3.2 step 6, `watermark_from`, `watermark_to` and `as_of_marker` NULL and `watermark_kind` copied from the table's watermark, since the column is NOT NULL (a replay moves no
   watermark), and `manifest = {"replay_id": ..., "replay": true}`.
3. Conflicts found during apply become `row_conflict` quarantine rows and are counted; they do not stop the apply.
4. Complete: status `applied`, `applied_at`, `applied_by` (set at the request), audit `mapping_replay.applied {counts}`.
   A failure returns the status to `completed` (the replay can be applied again; the writer's hash guard makes a repeat
   idempotent, C-24) and audits `mapping_replay.apply_failed {error_code}`.

### 3.9 The first adapter pack: ICS BANKS on Oracle (`ics_banks@1.0.0`; C-62)

The pack is a signed file of the release (its SHA-256 in the release manifest, phase 00), read by discovery (phase 05) and
by the loads of this phase. It carries the statements that are fixed by the product and the defaults the bank confirms.
The facts below are from the discovery of one ICS BANKS installation (`cbs-discovery/CBS-DISCOVERY.md`, Oracle
11.2.0.4, a snapshot 22 months behind the clock, DR-36). They are the defaults of the pack, and each one is confirmed by
the bank before a table is approved.

- **Business date.** `PROCESS` holds one row per branch: `BANK_DATE` (current business date), `PRE_BANK_DATE`,
  `NEX_BANK_DATE`, `LAS_RUN_DATE_BAT` and `RET_STA` (0 means the last run succeeded). The branch dates statement reads
  `PROCESS` only. The last closed business date is `BANK_DATE - 1` (the pack's rule, to be confirmed per bank; open
  doubt 3).
- **Postings.** `TRANSACT` (about 5.4 million rows in the snapshot), business day `TRA_DATE` (a DATE at midnight). The
  change strategy is `business_date` with a keyset range on the unique index from `TRA_DATE >= watermark - 31 days`, per
  branch. Back-dated entries reach 30 days (1.6% of legs), which is why the overlap is 31 days. `UPD_TIME` is a
  six-character time of day and is never a watermark (DATA-MODEL s.5; DR-37). `source_local_ts` is kept as text (DR-87).
- **Ledger totals.** `LED_BALA` is read daily by `ACCT_DATE` for the GL tie-out of phase 12; it is a business-date table.
- **Master tables.** `CUSTOMER`, `CUST_PRO`, `ADDRESS`, `ACCOUNT`, `ACCT_SIG`, `TIT_INF`, `TIT_REL`, `CUST_BEN`, `CUST_BOD`,
  `BLK_FUND`, `ACCT_RES`, `STOP_PAY`, `TELLER`: `full_compare`, a nightly snapshot with row hashes, F5. `CUSTOMER`,
  `CUST_PRO` and `ACCOUNT` have no change-date index, so no date range is possible.
- **Transfers.** `OUT_CTRF`, `CTRF_HIS`, `CEN_TRF` and `TRANSFER`: re-read the last 90 days by `TRA_DATE` each night and
  diff the whole table weekly, because statuses change after creation and rows can be deleted by the CBS batch.
- **Payment feed.** `SWF_INPT`, `SWF_IDET` and `SWF_DETL` by `MESS_RECV_DATE`. In the snapshot the feed has been frozen
  since 2022-02-09. The loader reads it normally; its freshness state is `not_moving` (section 3.7 rule f). The stoppage
  is a finding for the steward and the bank, not a loader error.
- **Never read.** `WS_TRANS`, `WS_TRANS_BKP`, `XERROR` and `AUTH_TEL` (log tables; a single aggregate on one of them did
  not return in 200 seconds). `TRANS_HIST`, the legacy history of 2016 to 2021, is a one-time `full_compare` load,
  not part of the schedule.
- **Deny list (dropped at extraction, never landed).** The `PASSWORD` columns of `CUSTOMER`, `TELLER` and `TIT_INF` (the
  snapshot could not show whether they are plaintext). Card track, PAN and PIN fields follow the phase 10 card guard.
  `PEND_TRA`, the pending-transaction queue, is optional and its card number is masked at extraction.
- **Statements.** Every statement is generated from the approved object and proven by `sqlproof` before it runs (phase 05,
  DR-61). The pack names the approved objects of the default plan; the bank's change request (`source_access`, phase
  05) approves them.

The pack's version is recorded on each source (`sources.adapter_pack = ics_banks@1.0.0`) and on each receipt through the
mapping version it ran under.

### 3.10 Errors, audit events, Cedar actions, tipping-off review and settings

**Error mapping.** Only codes of API.md s.6 are used; no new problem code is proposed. Reasons below are the `code` or
`errors[].reason` values of those codes.

| Code | Status | Where | Reasons |
|-|-|-|-|
| `E_NOT_ALLOWED_IN_STATE` | 409 | resume, apply, retry, exclude, replay create | `source_not_halted`, `source_not_active`, `conflict_needs_review`, `replay_active`, `replay_not_completed`, `replay_already_applied`, `version_not_candidate`, `version_not_published`, `version_mismatch`, `fact_changes_need_correction`, `quarantine_not_open` |
| `E_SOURCE_REFUSED` | 409 | resume | the latest grant check refused the source; finding codes in `errors[]` |
| `E_VALIDATION` | 422 | every POST | `range_inverted`, `range_too_long`, `table_not_in_version`, `restore_decision_required`, `restore_decision_invalid`, `restored_to_missing`, `restored_to_after_watermark`, `scope_invalid`, `started_before_anchor`, `in_future` (at `/started_at` or `/ended_at`), `too_short` and `too_long` (at `/comment`); `range_inverted` also answers an `ended_at` before `started_at` and a `business_to` before `business_from` |
| `E_COST_GUARD` | 422 | replay create | `estimate_rows` above `replay.estimate_max_rows`; the estimate in the body |
| `E_STEPUP_REQUIRED` | 401 | exclude, apply, resume when halted | missing, reused or wrong-class assertion |
| `E_VERSION_CONFLICT` | 409 | exclude, retry, apply | `If-Match` does not match |
| `E_PRECONDITION_REQUIRED` | 428 | exclude, retry, apply | no `If-Match` |
| `E_NOT_FOUND` | 404 | every read | missing, or the masked replay of section 3.8 |
| `E_SCOPE` / `E_FORBIDDEN` | 403 | every route | scope or Cedar denial |

Errors inside jobs do not reach an HTTP answer. They end a unit with a receipt reason (`writer_failed`,
`control_total_mismatch`, `file_hash_mismatch`, `column_check_failed`, `unit_timeout`, `budget_window`, `source_paused`, `lost_race`,
`restore_suspected`, `failover_suspected`, `cdc_gap`, `file_gap`, `version_mismatch`) or a replay error code (section 3.8).
`IngestError`, `RestoreError`, `ReplayError`, `FreshnessError` and `LandingError` are `thiserror` enums that map to those
codes in `ingestd` and to the HTTP codes above in `api::error`.

**Audit events emitted** (catalogue additions; SIEM-forwarded from phase 25):

| Event kind | On | Resource |
|-|-|-|
| `batch.recorded` | every receipt (status, counts, watermark move; counts only) | batch |
| `batch.rejected` | a unit refused before or after its transaction (reason) | batch |
| `file.rejected` | manifest or file hash, size or control-total failure (file name, sha256, reason) | source |
| `source.restore_suspected` | a restore or failover signal (signal, observed, expected) | source |
| `source.state_changed` | phase 05 event, used for `active` to `halted` and `halted` to `active` | source |
| `source.halt_resolved` | resume with a decision (decision, tables, check id; `as_of_marker` on a `false_alarm` after an `as_of_regressed` halt, 7.13) | source |
| `watermark.reset` | a `restored` decision (tables, from, to) | source |
| `blind_period.opened`, `blind_period.closed` | automatic opens and closes (cause, dates, statement counts) | blind_period |
| `blind_period.declared` | a steward's declaration (scope, cause, dates) | blind_period |
| `quarantine.retry_requested`, `quarantine.excluded` | retry and exclusion (reason length, not the reason text) | quarantine_item |
| `freshness.stale_entered`, `freshness.recovered` | a change of stale state (class, reason, data as of) | watermark |
| `mapping_replay.requested`, `.completed`, `.failed` | replay lifecycle (window, counts, volume effect, error) | mapping_replay |
| `mapping_replay.apply_requested`, `.applied`, `.apply_failed` | apply lifecycle (counts, error) | mapping_replay |

Reused: `canon.written` (phase 10, per entity and batch), `source.paused`, `source.state_changed` (phase 05).

**Cedar actions** (new file `backend/policies/72_ingest.cedar`, in the 70 range with `70_sources` and `71_mapping`):

| Action | Resource | Who (shipped bundle) |
|-|-|-|
| `watermark.read`, `batch.read`, `quarantine.read` | System, Watermark, Batch, Quarantine | DS |
| `quarantine.retry` | Quarantine | DS (`row_failed` only, by the resource) |
| `quarantine.exclude` | Quarantine | DS, with step-up class `decision` |
| `freshness.read`, `blind_period.read` | System, Freshness, BlindPeriod | M, CH, IA, DS |
| `blind_period.declare` | BlindPeriod | DS (system and rail scope only) |
| `mapping_replay.create` | MappingReplay | DS |
| `mapping_replay.read` | MappingReplay | DS, CHK and M (scope `data.steward` or `config.approve`); unpublished versions masked 404 for principals outside section 3.8 |
| `mapping_replay.apply` | MappingReplay | DS, with step-up class `backfill` |
| `source.resume` (phase 05, extended) | Source | DS, SEC; a halted source needs step-up class `backfill` |

```cedar
// 72_ingest.cedar (excerpt; the entities Watermark, Batch, Quarantine, BlindPeriod, MappingReplay and Freshness are
// added to schema.cedarschema, with the attributes listed under "Attributes evaluated")
@id("ingest.steward") @role("DS")
permit (principal, action in [Akr::Action::"watermark.read", Akr::Action::"batch.read",
        Akr::Action::"quarantine.read", Akr::Action::"quarantine.retry", Akr::Action::"quarantine.exclude",
        Akr::Action::"freshness.read", Akr::Action::"blind_period.read", Akr::Action::"blind_period.declare",
        Akr::Action::"mapping_replay.create", Akr::Action::"mapping_replay.read", Akr::Action::"mapping_replay.apply"],
        resource)
when { principal.roles.contains("DS") };

@id("ingest.freshness_readers") @role("M,CH,IA")
permit (principal, action in [Akr::Action::"freshness.read", Akr::Action::"blind_period.read"], resource)
when { principal.roles.containsAny(["M", "CH", "IA"]) };

@id("ingest.replay_reviewers") @role("M,CHK")
permit (principal, action == Akr::Action::"mapping_replay.read", resource is Akr::MappingReplay)
when { principal.roles.containsAny(["M", "CHK"]) };

// Only row failures are retried; a row conflict is reviewed in phase 12.
@id("ingest.retry_row_failed_only") @deny("state")
forbid (principal, action == Akr::Action::"quarantine.retry", resource is Akr::Quarantine)
when { resource.kind != "row_failed" };

// Replays of unpublished versions are visible to the requester, to M and to checkers with an open change request on
// the version; everyone else gets the masked 404 (existence masking, API.md s.1).
@id("ingest.replay_unpublished_masked") @deny("masked")
forbid (principal, action == Akr::Action::"mapping_replay.read", resource is Akr::MappingReplay)
when { resource.version_status != "published" && resource.requested_by != principal
       && !principal.roles.contains("M") && !resource.approver_ids.contains(principal) };
```

Step-up is not a Cedar condition: the handlers of `quarantine.exclude` (class `decision`), `mapping_replay.apply`
(class `backfill`) and the halted branch of `source.resume` (class `backfill`) call the phase 01
`StepUpService::consume` with their `StepUpAction` constants inside the action's transaction, as every SU route does
(DR-29). Attributes evaluated: `principal` role membership; `resource.state` (Source), `resource.version_status`,
`resource.requested_by` and `resource.approver_ids` (MappingReplay), `resource.kind` (Quarantine); `context.aal`.
Bundle tests: DS reaches every ingest action; A1, A2, KYC, RFI and OPS reach none of them; CH and IA read the freshness
and the blind periods only; SEC reaches only the phase 05 source actions; a replay of a `testing` version is masked 404
for a checker without an open change request on it and visible to M; a `row_conflict` retry is denied.

**Tipping-off review.**
- The data plane has no customer-facing surface: no branch screen, no customer screen and no notice in this phase. The
  branch roles (L, A1, A2 and the rest) have no route in this phase.
- Quarantine rows carry the key of a raw row, which identifies a customer. The list shows a 12-character keyed-hash prefix
  (`source_pk_hk`); the full key is shown only in the detail, to the steward, and reading it writes an access event `view`
  (phase 03). Such a row is never shown to a case, an alert or a branch.
- A blind period, a stale source and a halt say that data did not arrive. They say nothing about any customer or about
  any alert, and the branch roles cannot read them. Ordinary data gaps do not reveal a suspicion; the MLRO statement of
  phase 12 uses the same facts without customer detail.
- Replay diffs are detection-logic. Their samples are shapes, and an unpublished version's diff is masked (section 3.8).
- Audit details carry counts, codes, dates and identifiers. The text of a quarantine exclusion reason and the
  statement of a declaration stay in their rows and are not copied into audit details.
- The answer for a person outside the roles above is: nothing about this phase is visible.

**Settings and configuration used.** The eighteen keys of migration 0035: `ingest.sequence_overlap_keys`, `ingest.late_alarm_pct` and `ingest.lookback_max_days` of sections 7.10 and 7.12, and the fifteen of section 1.3 (`ingest.history_from`, `ingest.slice_days`,
`ingest.max_unit_rows`, `ingest.settle_seconds`, `freshness.f1_lag_seconds`, `freshness.f2_max_minutes`,
`freshness.f3_max_business_days`, `freshness.f5_max_hours`, `freshness.stale_business_days`, `restore.row_drop_warn_pct`,
`replay.max_range_days`, `replay.estimate_max_rows` and the three `schedule.ingest_*` cron keys); the two flags
`ingest.cdc_enabled` and `ingest.files_enabled`; phase 05 `LoadGovernor` keys (`rows_per_fetch`, `statement_timeout_ms`,
`slowdown`, `sources.session_max_age_s`); phase 10 `mapping.default_overlap_days` (the default re-read window of 31
business days, floor 31); phase 09 `BankCalendarClock` and the calendar reference; phase 00 `bank.timezone`. The
governed keys (`ingest.history_from`, `ingest.settle_seconds`, the five `freshness.*` thresholds, `replay.max_range_days`,
`replay.estimate_max_rows`, `ingest.sequence_overlap_keys`, `ingest.late_alarm_pct`, `ingest.lookback_max_days`) change through a change request of class `material`, because each one changes what the bank
is told is covered. Inbox paths are deployment configuration (`AKR_INGEST_INBOX_ROOT`), not settings.

## 4. Infrastructure Updates

### 4.1 Jobs and partitions

- **Job kinds registered** (all `ingest_` prefix, claimed by akr-ingestd under the phase 00 RLS policy `jobs_ingest_own`;
  akr-api enqueues the replay, retry and resume kinds through `akr_app`, whose policy allows every kind):

| Kind | Payload | Coalesce key | Concurrency | Max attempts | Schedule or trigger |
|-|-|-|-|-|-|
| `ingest_tick` | `{}` | `ingest_tick` | 1 | 1 | `schedule.ingest_tick`, every minute (section 3.1) |
| `ingest_load` | `{source_id, source_table}` | `ingest_load:<source_id>:<source_table>` | 4 across sources; the governor caps sessions per source | 5 | enqueued by the tick; reschedules itself (section 3.1) |
| `ingest_file_scan` | `{}` | `ingest_file_scan` | 1 | 3 | `schedule.ingest_file_scan`, every 30 seconds (section 3.4) |
| `ingest_map_pending` | `{source_id, source_table, version_id}` or `{quarantine_ids}` | `ingest_map_pending:<source_id>:<source_table>` for pending; `ingest_map_pending:retry:<request uuid>` for retries | 2 | 5 | the tick (landed receipts and a published version); the retry route (section 3.3) |
| `ingest_freshness` | `{}` | `ingest_freshness` | 1 | 3 | `schedule.ingest_freshness`, every 5 minutes (section 3.7) |
| `ingest_resume` | `{source_id, restore_decision?, restored_to?, requested_by, check_id}` | `ingest_resume:<source_id>` | 1 per source | 1 | the resume route (section 3.5) |
| `ingest_mapping_replay` | `{replay_id}` | `ingest_mapping_replay:<version_id>` | 1 per version (`uq_mapping_replays_active`) | 3 | the replay route (section 3.8) |
| `ingest_mapping_replay_apply` | `{replay_id}` | `ingest_mapping_replay_apply:<version_id>` | 1 per version | 3 | the apply route (section 3.8) |

The retry key carries a request uuid so that two retries of different rows are not collapsed into one. No akr-worker
job is added: every operation of this phase touches the CBS landing, the canon writer or the raw layer, which only
akr-ingestd may reach.

- **Partitions.** Raw landing partitions are daily (UTC) and are created by `raw.create_landing_partition`: the tick
  makes today's and tomorrow's partitions of every landing table, and the landing path creates a missing one inline
  and retries once (a missing partition is never a silent drop: no DEFAULT partition exists). The phase 02
  `partition_maintenance` job and its registry are monthly and are not used for raw. Partitions are dropped by the
  retention engine of phase 23 (R1, clock `partition` on `extracted_at`); this phase never drops one.
- **Blind periods, watermarks, batches and quarantine** are plain tables, not partitioned: their volume is one row per
  unit or per interruption (ICS BANKS: a few thousand receipts a day at the planned schedule).
- **Notification channel:** `akr_source` gains the value `state` for halts and resumes (phase 05 channel; the listener
  already handles it). No new channel.
- **Outbox topics:** `canon.touched` (section 3.3; consumers from phase 14), `dq.stale` (DSL.md A.3), `ingest.halted`, `ingest.resumed`; consumed by the SSE channel
  (`ingest.status`, operations roles) and, from phase 18, by the inbox.

### 4.2 Docker compose and network zones

- **Inbox volume.** One host path `AKR_INGEST_INBOX_ROOT` (default `/var/akr/inbox`) mounted into akr-ingestd only. The
  bank delivers into `<inbox_key>/incoming/` from its own file share, which is not mounted into any AKR container. akr-ingestd
  reads `incoming/` and `cdc/`, and writes only `processed/` (section 3.4). Directory ownership enforces it.
- **Change capture (opt-in).** Compose profile `cdc` adds `akr-cdc`, a Debezium Server container running in the
  source's zone with read access to the CBS log under its own account. It writes segments to `<inbox_key>/cdc/` on the
  inbox volume and reaches no AKR service. The profile is not started unless `ingest.cdc_enabled` is on, the source is
  CT3 and the bank has signed the change-capture agreement (DR-91). This is the only new network edge in the phase,
  and it is a CBS-facing edge that the bank must accept; the zone table of ARCHITECTURE s.4 gets a row for it.
- **Development fixtures.** The phase 05 `cbs-fixture-pg` gains a `PROCESS` table with per-branch `BANK_DATE`, a
  `TRANSACT`-shaped fixture of about one million rows over ten business days, a `restore-to` mode of
  `akr-ctl fixtures cbs` that moves the fixture's `PROCESS` dates and its SCN backwards, and `akr-ctl fixtures extracts
  generate`, which writes manifests and files for the CT4 tests. Nothing is derived from `cbs-discovery/` data.

### 4.3 AppState additions, OpenAPI, readiness and metrics

- **`api`:** `ingest: Arc<IngestQuery>` (watermarks, batches, quarantine, freshness reads), `quarantine:
  Arc<QuarantineService>`, `blind: Arc<BlindPeriodService>` (declarations), `replays: Arc<ReplayService>`, `freshness:
  Arc<FreshnessQuery>` (30-second cache, the `FreshnessContext` middleware), and the resume handler of the phase 05 route.
- **`ingestd`:** `IngestScheduler`, `LoadUnitRunner`, `LandingWriter`, `WatermarkService`, `RestoreDetectorIo`,
  `FileScanner`, `ChangeCaptureConsumer`, `ReplayRunner`, `FreshnessService`, `ResumeHandler`, the blind-period close path,
  and the phase 10 `CanonicalWriter` and `MappingExecutor` as libraries.
- **`worker`:** no change.
- **OpenAPI registration:** tag `ingest` (9 operations) and tag `replays` (3), each with `x-akr-action` (the Cedar action
  of section 3.10); `POST /sources/{id}/resume` is documented with `ResumeBody`. The SU operations declare
  `X-StepUp-Assertion` and the step-up scheme. Every POST documents `Idempotency-Key` and the 202 with `Location`.
  Every CBS-derived response documents `X-Data-As-Of` and `stale`. `make generate-api` regenerates the client with the
  hooks of section 5.
- **Readiness:** akr-ingestd `/readyz` adds `ingest_scheduler` (the tick job is claimable) and `landing_factory`
  (`raw.landing_table_name` answers for a test name). akr-api `/readyz` adds `freshness` (the watermark query answers).
- **Metrics:** `akr_ingest_units_total{source, table, status}` (counter), `akr_ingest_rows_total{source, table, kind}`
  (landed, unchanged, quarantined, deleted), `akr_ingest_lag_seconds{source, class}` (histogram, section 3.7),
  `akr_watermark_stale{source, class, reason}` (gauge), `akr_source_halts_total{cause}` (counter), `akr_blind_periods_open
  {scope}` (gauge), `akr_replay_seconds` (histogram) and `akr_replay_runs{status}` (counter).
- **Alert rules:** a source halted for more than one hour; a source stale with reason `not_moving` for more than two
  business days; any `file.rejected` event; a replay `failed`. The inbox delivery of these alerts is phase 18.
- **Network zones:** no change for AKR services. Raw landing and replays read `raw` only inside akr-ingestd; akr-api reads
  watermarks, batches, quarantine and replays from PostgreSQL and never reads `raw`.

### 4.4 Existing entity, schema and route updates required by this phase

- **Phase 00:** settings and flags registered by 0035; outbox topics above; the classification and retention checks
  accept the dynamic `raw` tables because the factory registers them in the same transaction; the job RLS prefixes are
  unchanged.
- **Phase 01:** no new action class. The phase uses `decision` and `backfill`, both in the closed enum.
- **Phase 02:** `audit.audit_anchors.anchored_at` readable by `akr_app` (column grant in 0034, for the declaration
  check); `partition_maintenance` unchanged.
- **Phase 03:** Cedar files `72_ingest.cedar` and the entities `Watermark`, `Batch`, `Quarantine`, `BlindPeriod`,
  `MappingReplay` and `Freshness` in `schema.cedarschema`; the attribute `Source.state` is read (existing entity).
  Feature flags `ingest.cdc_enabled` and `ingest.files_enabled` are read by the source registry.
- **Phase 04:** the evidence kind `replay` resolves to `admin.mapping_replays` (its resolver is registered by this phase);
  the `ChangeTargetRegistry` is unchanged; the route map sends governed setting changes of section 3.10 to `cr_material`.
- **Phase 05:**
  - `admin.sources`: tier CHECK widened to CT3; `delivery` and `inbox_key` columns (0033); `POST /sources` accepts CT3 only
    with the flag and CT4 files only with `inbox_key`.
  - `SourceReader` gains `fetch_keyset`, `branch_dates` and `as_of` (section 2).
  - The state machine of section 3.1 gains `active -> halted` (phase 11, section 3.5), `halted -> active` and
    `halted -> refused` (`ingest_resume`).
  - `ingest_source_health` runs `RestoreDetectorIo` on the statement it already reads (section 3.5).
  - `POST /sources/{id}/pause` and the bank pause and refusal transitions open a source blind period in their
    transaction (section 3.6).
  - `POST /sources/{id}/resume` is extended for halted sources (section 3.5); its paused behaviour is unchanged.
  - The `load_budget` and `source_access` change targets are unchanged.
- **Phase 09:** `BankCalendarClock` counts lag in business days; the CBS calendar capture is unchanged.
- **Phase 10:**
  - `MappingExecutor` gains `map_payload` (the pure transforms applied to a stored payload, used by replays and retries).
  - The `mapping` change target reads `volume_effect` from the completed replay of the proposed version (DR-92).
  - `RowConflict` kinds map to quarantine reasons (section 3.2 step 5); no change to the writer.
  - The `monitored` flags of `canonical-model.v1.json` are read by the replay differ; no change to the file.
- **Phase 12:** its `DataQualityRecorder` is called in the landing transaction after the quarantine rows (DR-97); it adds
  `POST /quarantine/{id}/assign`, two list filters and two exclusion refusals, and closes `row_conflict` rows with their
  issue; the `reconciliation` field of `GET /batches/{id}` is `null` until the ladder exists.
- **DATA-MODEL.md (aligned):** s.3 gains `source_pk` on raw tables and the
  watermark and batch columns of section 1.1; s.6 `blind_periods` gains the causes `source_paused` and `delivery_gap`;
  `batches` is R9 (DR-93); s.21 grants of phase 11 are those of section 1; `admin.mapping_replays` is added.
- **API.md (aligned):** 3.5 gains `GET /freshness` (DR-93) and the
  resume body; 3.6 gives the replay GET the scope `config.approve` and the apply class `backfill`; the blind-period routes
  of 3.7 move to this phase (DR-90).

## 5. Frontend Impact

Backend phase; the screens below are touched after phase 08 builds the shell and the components. No new screen is
added (the roadmap gives phase 11 none). Freshness is always shown as words with the colour as a second signal
(DESIGN-SYSTEM, colour-blind rules): "Current", "Behind", "Not moving", "Halted", "Never loaded", "Blind".

- **Shell (phase 08):** the header shows `data_as_of` for the screen's sources from `X-Data-As-Of`, with the stale state
  when `stale` is true; the export dialog states the same value (C-29).
- **T1 data connection (phase 05 screen, mockup 08):**
  - A freshness band per source: each table and branch with its class, `data_as_of`, lag in business days and its state
    word.
  - A "Halted" banner with the reason and the time, and, for DS, the resume drawer: the restore decision (three radio
    options with the consequence of each in words), the restored point per table when `restored` is chosen, and the
    `StepUpDialog` (class `backfill`) before the request is sent with `Idempotency-Key`.
  - A batch list (status, rows, watermark move, mapping version) and a batch detail.
  - A "Rows held back" tab for DS listing quarantine rows (kind, reason, age); the exclusion drawer requires a reason of
    10 characters or more and the step-up class `decision`. Phase 12's T4 replaces this tab.
  - A blind-period strip with the open period, its cause and its start.
- **T3 mapping workbench (phase 10 screen):** a Replay panel on each candidate version: range pickers (bounded by
  `replay.max_range_days`), tables, the estimate, a status line polled every 3 seconds until final (as phase 06 polls its
  report runs), the diff counts per table and monitored attribute, the volume effect with its words ("No change", "Needs
  scenario replay"), and the apply action (DS, step-up class `backfill`) for a published version. A masked replay shows the
  same empty state as an unknown one.
- **E2 discovery screen (phase 06 screen):** the freshness column in the sources band.
- **Hooks consumed** (generated client): `useGetSourcesIdWatermarks`, `useGetSourcesIdBatches`, `useGetBatchesId`,
  `useGetQuarantine`, `usePostQuarantineIdRetry`, `usePostQuarantineIdExclude`, `useGetFreshness`,
  `useGetCoverageBlindPeriods`, `usePostCoverageBlindPeriods`, `usePostMappingsIdReplays`, `useGetMappingReplaysId`,
  `usePostMappingReplaysIdApply`, `usePostSourcesIdResume`.
- **Polling:** the halted banner and the replay status poll; every other view refetches on the SSE events
  `ingest.status` and `task.updated` through TanStack Query invalidation (API.md s.1).

## 6. Verification

1. `make migrate`: 0033 to 0035 apply, and a second run is a no-op. Negative checks: `delivery = 'cdc'` on a CT1 source fails `ck_sources_delivery`; a `files` source without `inbox_key` fails; `DELETE FROM admin.watermarks` fails for every role; a `source` blind period without `source_id` fails; a second open period for one source fails `uq_blind_open_source`; a `batches` row with status `rejected` and no reason fails; a `mapped` receipt with no mapping version fails; a second active replay of one version fails `uq_mapping_replays_active`; `akr_app` has no INSERT on any `raw.*` table.
2. `raw.create_landing_table('ics1', 'TRANSACT')` creates `raw.ics1__transact` with seven classification rows and one R1 retention row; a second call changes nothing; a row whose `extracted_at` has no partition fails loudly; `raw.create_landing_partition` creates exactly one day partition per call.
3. `cargo check` with `cargo sqlx prepare`; `cargo clippy --all-targets -- -D warnings`; `cargo deny check`; `cargo tree -p akr-api` shows no `akr-sources` or driver crate.
4. Settings: `make seed` installs 18 settings and 2 flags; `SELECT key FROM admin.settings WHERE floor IS NOT NULL AND jsonb_typeof(floor) <> 'object'` returns no rows.
5. `cargo test`, domain unit tests (no database):
   - `UnitPlanner`: a first load starts at `ingest.history_from`; the overlap is 31 business days; only closed days are planned; slices follow `ingest.slice_days`; a day above `ingest.max_unit_rows` is split; an empty closed day advances the watermark.
   - `ChangeDetector`: an unchanged row is not landed; a changed row is landed; a delete is an event only with `key_reconcile`.
   - `RestoreDetector`: an earlier business date halts; an equal date does not; an as-of regression with a changed endpoint is `failover_suspected`; no marker means no as-of signal; a row drop warns and never halts.
   - `FreshnessPolicy`: each rule a to g at its boundary, first match wins; Baghdad weekends and holidays do not count as lag through `BankCalendarClock`.
   - `ReplayDiffer` and the volume effect: `neutral` only when every changed, added and removed count is zero; a fact change is counted apart.
   - The resume decision matrix of section 3.5, including `restored_to` later than the stored value.
6. Integration tests on the fixture CBS and PostgreSQL 17 (`cargo test -p ingest --test integration`):
   - the landing transaction is atomic: a fault injected after the canonical write leaves no raw row, no receipt and no watermark move;
   - a lost compare-and-set rolls the unit back with reason `lost_race`;
   - a restore to an earlier business date (fixture `restore-to`) halts the source, opens one blind period, writes nothing for the unit, and the job audits `source.restore_suspected`;
   - `restored` with `restored_to` re-reads the rows after the point, and a changed settled fact becomes a `row_conflict` row without an overwrite;
   - a CT4 manifest with a sequence gap halts with `file_gap`; a sequence with a different hash rejects the batch and does not halt; a corrupted file is `file_hash_mismatch`;
   - a CT3 segment with a sequence gap halts with `cdc_gap`;
   - `ingest_load` with the fixture's slow-down doubles the statement time and the governor's rate factor drops (phase 05 gauge);
   - the replay writes no canonical row (the statement journal shows no INSERT or UPDATE on `canon`) and the apply is refused while `fact_changed` is above zero.
7. Curl suite `tests/curl/phase11_ingest.sh` (dev users `ds@bank.test`, `m@bank.test`, `chk@bank.test`, `ch@bank.test`, `a1@bank.test`; the phase 05 suite has left `CBS1` active with a completed profile run and the phase 10 suite has left `cbs1.txn` in `testing`):
   - As DS `GET /api/v1/sources/<CBS1>/watermarks` returns 200 with `data_as_of` in the body and `X-Data-As-Of` in the header; as A1 the same route returns 403 `E_SCOPE`.
   - As M and as CH `GET /api/v1/freshness` returns 200; as A1 it returns 403 `E_SCOPE`.
   - As DS `POST /api/v1/quarantine/<row_failed id>/exclude` without step-up returns 401 `E_STEPUP_REQUIRED`; with the `decision` assertion, `If-Match` and a 20-character reason it returns 200 with status `excluded`; the same request again returns 409 `E_NOT_ALLOWED_IN_STATE` with reason `quarantine_not_open`.
   - As DS `POST /api/v1/quarantine/<row_conflict id>/retry` returns 409 with reason `conflict_needs_review`; the `row_failed` retry with `Idempotency-Key: q-0001` returns 202, and the same key and body returns the stored 202 with `x-idempotent-replay: true`.
   - As DS `POST /api/v1/mappings/<cbs1.txn>/replays` with `Idempotency-Key: rp-0001` returns 202; the same key with another body returns 422 `E_IDEMPOTENCY_REUSE`; a 93-day range returns 422 `range_too_long`; a published version returns 409 `version_not_candidate`.
   - Masked 404: a replay of `cbs1.txn` requested by DS, read as `chk@bank.test` who holds no open change request on that version, returns 404 `E_NOT_FOUND` with a body byte-identical to `GET /api/v1/mapping-replays/<unknown uuid>`; as `m@bank.test` the same replay returns 200.
   - As DS `GET /api/v1/mapping-replays/<id>` after completion returns the counts and `volume_effect`; a `jq` walk finds no string value in `diff` outside the shape histograms.
   - As DS `POST /api/v1/mapping-replays/<id>/apply` without step-up returns 401; with the `backfill` assertion on a `testing` version it returns 409 `version_not_published`.
   - As M `GET /api/v1/coverage/blind-periods` returns 200; as DS `POST /api/v1/coverage/blind-periods` with `scope: source` returns 422 `scope_invalid`; with `scope: system`, cause `akr_outage`, a `started_at` after the newest anchor and an `ended_at` returns 200, and without `ended_at` 422; as A1 the GET returns 403.
   - On the restored fixture: `POST /api/v1/sources/<CBS1>/resume` without a decision returns 422 `restore_decision_required`; with `false_alarm` and no step-up returns 401; with the `backfill` assertion returns 202, and the source is `active` after `ingest_resume` (polled).
   - C-35: every GET of the suite leaves the phase 05 statement journal unchanged (no statement from an API read).
8. Swagger UI lists the 12 new operations and the extended resume route, each with `x-akr-action`; `make generate-api` run twice gives zero diff.
9. Audit trail after the curl run: `batch.recorded`, `quarantine.excluded`, `quarantine.retry_requested`, `mapping_replay.requested`, `mapping_replay.completed`, `blind_period.declared`, `source.halt_resolved`, `source.restore_suspected` and `authz.denied` (the A1 and CHK refusals) are present; `akr-ctl audit verify` passes.
10. Phases 00 to 10 tests still pass; the phase 05 read-only conformance suite passes (no write statement, only single SELECTs, load budget enforced); the change-capture sidecar container is absent under default compose.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), gap item (a): Cedar form corrected in place (tools/agent/fix-g1-cedar.py); lead-verified.

### 7.1 Cedar form (gap item (a))
The Cedar policies of this phase are corrected in place: a role test is `principal.roles.contains("X")` (several
roles: `principal.roles.containsAny([...])`) in a `when` clause, never `principal in Akr::Role::"X"`, which phase 03's
schema does not model (7 occurrences rewritten; the `@role` annotations are unchanged).

Gap pass 1 (2026-10-10), run 3b, from tools/agent/gap1-run3b.md; lead-verified (tools/agent/gap1-run3b-verdict.md).

### 7.2 Tombstone check on every canonical write (phase 10 section 7+)
This phase writes canon only through phase 10 `CanonicalWriter::write`, so the tombstone check that phase 23 adds to
that writer (phase 10 section 7+) covers every row the load loop, catch-up and `ingest_resume` write. No check of its
own is added here.

### 7.3 Quarantine extensions owned by phase 12 (phase 12 section 3.11)
Phase 12 extends this phase's quarantine and owns the extension: the Cedar entity `Quarantine` gains `gate: String`,
derived and not stored (a row held for a transaction-type code that is not mapped: reason `unmapped_code` with field
`txn_type`, DSL.md B.7); `QuarantineRepository` gains `assign` and `accept_conflicts`; `QuarantineFilter` gains
`owner_id`, `older_than_days` and the order `oldest`; the exclusion gains the refusals `gate_not_excludable` and
`card_guard_not_excludable`. The quarantine routes and actions of this phase do not change.

### 7.4 No operator-planned backfill (DR-380)
The backfill rows are removed from API.md 3.5. A gap is covered by the re-read window (DR-37, overlap default 31 days)
and the automatic catch-up of this phase (DR-90); an explicit range beyond the overlap is P2. The sentence of
section 3.5 "the operator-planned backfill of API.md 3.5 (phase 12) is for explicit slices only" and the C-15 line
"the operator-planned backfill of phase 12 is not repeated here" now read: no phase builds an operator-planned backfill
in this release.

Gap pass 1 (2026-10-10), run 5a (coverage of C, E, RP, S and TS IDs), from tools/agent/gap1-run5a.md; lead-verified (tools/agent/gap1-run5a-verdict.md).

### 7.5 Source formats accepted from this phase (C-14, DR-393)
Phase 10's compile step 3 refuses every format but `relational` and leaves the others to this phase. From this phase
on it also accepts:
- `landing_file` for a source with `delivery = files`: the columns are the `columns` of the table's newest accepted manifest (the extract file contract of section 7.20, which corrects section
  3.4, extract files step 3);
- `t24_xmlrecord`: an approved object with one key column and one record-text column, both named in the mapping's
  `source` block; fields are read with phase 10's `XmlField` transform (DSL `xml_field`);
- `t24_multivalue`: the same object shape, values split with phase 10's `MultivalueSplit`;
- `iso20022` and `mt` (next subsection).
The source read is the keyset statement of section 3.1 over the key column: no XML function and no predicate beyond
the bound reaches the CBS (C-14). The record text lands unchanged in the raw rows and is decoded in akr-ingestd. Any
other value stays refused with `format_not_in_phase`. Test: a fixture record decodes to the expected fields, and a
mapping with an unknown format is refused.

### 7.6 Payment messages in ISO 20022 and MT (C-41, DR-393)
1. Source shape: an approved object, in the CBS or in the payment-gateway database (next subsection), with a key
   column and a raw message column. The mapping has `source.format = iso20022` or `mt` and entity `payment_message`;
   `payment_message_party` is its own mapping over the same object.
2. A pure parser module (no IO) beside phase 10's transform library parses the raw text into the elements the
   mapping's `from` entries name (an ISO 20022 element path, or an MT field tag). Each parser has golden fixtures and a
   fuzz target that CI runs for a bounded time.
3. Parser version: each parser has a version constant that the compiler writes into the compiled mapping, so the
   `mapping_version_id` of every canonical row names the parser version. No column is added; a parser upgrade is a new
   mapping version through phase 10's lifecycle.
4. The raw message is kept in `canon.payment_message.raw_enc` where the source holds it (phase 10).
5. A message that does not parse is held as a quarantine row of kind `row_failed`, reason `message_parse_failed`
   (section 3.3), never written to canon.
6. Tests: a pacs.008 and an MT103 fixture each give the expected `payment_message` and party rows; a truncated message
   lands in quarantine; the fuzz target runs in CI.

### 7.7 Payment-gateway database as a second source (C-42)
A payment-gateway database is its own `admin.sources` row with its own grant proof and load budget (phase 05), its
own watermarks (section 3.1) and its own freshness state (section 3.7). Its approved objects map to
`canon.payment_message`, `canon.payment_message_party` and `canon.payment_link` through relational mappings (phase 10
catalogue) or through the formats of the subsection above. A gateway database on an engine phase 05 supports needs no
new adapter; a halt of the gateway source does not change the CBS source's freshness.

Adversarial pass (2026-10-10), run 2 (alerts under concurrency, replay and late data), from tools/agent/adv-run2.md; lead-verified (tools/agent/adv-run2-verdict.md).

### 7.8 Every canonical write is a touch (ADV-2-10, ADV-2-11, ADV-2-13; CRITICAL; DR-408)
A batch can reach canon in several transactions: a pending batch mapped over several runs (`ingest.max_unit_rows`), a
quarantine retry, and a page of a mapping replay apply. The payload of `canon.touched` therefore gains `touch_id`
(UUIDv7, one per writing transaction): `{batch_id, touch_id, source_id, source_table, entity_kinds, business_date}`.
1. Section 3.3 step 1 writes a new `touch_id` in each mapping transaction, so a batch mapped over three runs is touched
   three times.
2. Section 3.3 step 2 (retries) writes `canon.touched` in the transaction that wrote a retried row, when the writer
   inserted or changed a row, with the row's own `batch_id` and business date.
3. Section 3.8 `ingest_mapping_replay_apply` step 1 writes it for each page under the same rule, with the page receipt
   as `batch_id`.
The consumers that key on the batch (phase 14 onboarding, phase 15 rating fan-out, phase 19 network edges) re-read the
batch and are idempotent; they ignore `touch_id`. Phase 16 keys its run ledger on it (phase 16 section 7+). Test: a
`row_failed` row retried after a mapping publication writes one `canon.touched` with a new `touch_id`.

Adversarial pass (2026-10-10), run 3 (watermarks, restores, failover, late batches, source pauses), from tools/agent/adv-run3.md; lead-verified (tools/agent/adv-run3-verdict.md).

### 7.9 A paused source is stale (ADV-3-01 HIGH, ADV-3-07 LOW; DR-409)
While a source is paused nothing reads the CBS date, so rule e of section 3.7 compares `data_as_of` with a frozen date
and rule f never fires: the source looked fresh and every unit covered. `FreshnessPolicy::classify` therefore also takes
the source state, and a rule a2 follows rule a: a source in state `paused`, `paused_by_bank` or `refused` is stale with
`stale_reason = lag_ceiling` (no read is being made, so the lag has no ceiling). The coverage ledger then reads
`source_stale` (phase 12 section 3.7). The automatic blind-period opens of section 3.6 for a pause, a bank pause and a
refusal set `business_from` as the halt path does (the earliest `value_business_date` of the source plus one day, or `ingest.history_from` when the source has none yet), and
phase 12 states when a period covers an as-of date (phase 12 section 7+). Test: a source paused for three business days
on the test clock has every watermark stale and every unit dark.

### 7.10 Sequence keys re-read a commit window (ADV-3-03, HIGH; DR-409)
Research 25 section 7.3: a key taken early and committed late is skipped by a reader that keeps only the highest value
seen. Section 3.1 planner rule a for `sequence_id` therefore starts at the last component of the stored key tuple minus
`ingest.sequence_overlap_keys` (migration `0035`: integer, default 10000, governed, `floor = {"min": 100, "basis":
"research 25 s.7.3 commit order"}`), not at the stored key. The window must exceed the keys the CBS can hand out during
its longest transaction; the change detector's hash guard absorbs the repeated keys. Test: key 500 committed after key
501 was read lands once, in the next unit.

### 7.11 The high-water check before each unit (ADV-3-04, HIGH; DR-409)
Research 25 section 7.5 names the regression signal: the source's high-water mark falls below AKR's watermark. The
business-date and marker signals of section 3.5 miss a same-day restore where the marker is unvalidated (Open doubts 8).
Before the first page of a unit of kind `sequence_id`, `timestamp_column`, `t24_date_time` or `rowversion`, the unit reads
`MAX(<watermark column>)` of the table and branch in one approved aggregate statement under the governor (the form of
the phase 12 control-totals reads, proven by `sqlproof`). A maximum below the stored `value` is the signal
`HighWaterRegressed` of `RestoreDetector`: the halt path runs with `halted_reason = as_of_regressed` (the closed list)
and `statement.signal = high_water_regressed`, cause `failover_suspected` when the endpoint changed, else
`restore_suspected`. The check works on every engine. A legitimate delete of the newest rows is answered with
`restored` and `restored_to` = the observed maximum (no later than the stored value); `false_alarm` moves no watermark
and would halt again on the next unit. Test: lowering the fixture's maximum key below the stored value halts the source.

### 7.12 Late arrivals are measured and the auto lookback widens (ADV-3-05, HIGH; C-21; DR-409)
DSL B.2 `lookback: auto` derives the window from measured late arrivals and widens it; this phase parsed `lookback` only.
1. Each receipt records `manifest.late_days_max`: the largest number of days between a new or changed row's business
   date (or its watermark timestamp) and the receipt's extraction date. Unchanged repeats of the re-read window do not
   count. Histogram `akr_ingest_late_arrival_days{source, table}`.
2. The freshness job (section 3.7) reads, per table, the largest `late_days_max` of the last 90 days. Above
   `ingest.late_alarm_pct` percent of the table's window: audit `ingest.late_arrival_near_window {source_id,
   source_table, late_days, window_days}` written by the load unit when the tail grows (section 7.22, item 4), and the alert rule of section 4.3.
3. For a table whose mapping says `lookback: auto`, planner rule a uses the larger of `overlap` and 1.25 times that
   largest lag, capped at `ingest.lookback_max_days`; a change audits `ingest.lookback_widened {source_id, source_table,
   from_days, to_days}`. A fixed `lookback` duration is used as written.
Migration `0035` adds `ingest.late_alarm_pct` (integer, default 80) and `ingest.lookback_max_days` (integer, default 93),
both governed. A row later than the window stays unread: that bound is DR-37, and the alarm is how a person learns the
tail is near it. Test: a row landed 28 days late against a 31-day window writes `ingest.late_arrival_near_window`.

### 7.13 What the resume decisions write (ADV-3-08 LOW, ADV-3-09 MEDIUM; DR-409)
Section 3.5 resume step 3:
- `restored` on a `business_date` table: `restored_to` is a date, and `reset_and_clear` writes `value`,
  `value_business_date` and `business_date_moved_at = now()`, so the planner re-reads from the restored date.
- `false_alarm` after an `as_of_regressed` halt writes the marker observed by the halting unit (its rejected receipt)
  into `as_of_marker` of every watermark of the source, in the same transaction, and `source.halt_resolved` carries it as `as_of_marker`, beside `decision`, `tables` and `check_id`.
  The next unit compares with that marker, so a standby the bank accepts does not halt the source again. A
  `business_date_regressed` halt cleared by `false_alarm` halts again while the date is still behind: the bank fixes the
  date or decides `restored`.
Test: `false_alarm` after a lower standby marker lets the next unit run.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.14 The watermark rows are created before the first unit (V-11-02-a, HIGH; DR-413)
`advance` is an UPDATE, so a table with no row could never take its first value. `WatermarkRepository` gains
`ensure_rows(conn, source, table, branches: &[String], kind: WatermarkKind, class: FreshnessClass) -> Result<u64, RepoError>`: an insert with
`value` NULL and `ON CONFLICT (source_id, source_table, branch_code) DO NOTHING`, run as its own statement before the
unit's transaction (`akr_ingest` already holds INSERT on `admin.watermarks`). The insert also writes `freshness_class`: the caller
works it out by the class rule of section 3.7 and passes it as one more argument, `class: FreshnessClass`, so the
column default is never relied on. `ingest_load` step 2 calls it after the
branch-dates read: one row per branch for a `business_date` table, one row with `branch_code` `*` for every other
kind. `ingest_file_scan` calls it with the manifest's `branch_code` before the sequence comparison of section 3.4
extract step 2; a NULL `value` reads as sequence 0, so the first manifest of a table and branch has sequence 1. A new
row is `never_loaded` for the freshness job (section 3.7 rule b). No audit event: the first `batch.recorded` records
the first move.

### 7.15 Fields of the watermark port types (V-11-02-c, LOW; DR-413)
`StaleChange` (`WatermarkRepository::set_stale`) is `{ source_id, source_table, branch_code, class: FreshnessClass,
reason: Option<StaleReason>, data_as_of: Option<NaiveDate> }`; a `reason` of None clears the stale mark.
`StaleTransition` has the same fields and is returned only for a row whose stored mark changed: with a reason it is
`freshness.stale_entered`, with None `freshness.recovered`. `WatermarkWithSource` is `{ watermark: Watermark,
source_code: String, source_state: SourceState }` (phase 05's enum, read by rule a2 of section 7.9).

### 7.16 The replay estimate reads receipts, not raw (V-11-03-a, HIGH; DR-413)
akr-api never reads `raw` (section 4.3) and `LandingStore` is akr-ingestd's, so the route of section 3.8 cannot call
`LandingStore::estimate`. `BatchRepository` gains `rows_landed_in(conn, source, table, from: DateTime<Utc>, to:
DateTime<Utc>) -> Result<i64, RepoError>`: the sum of `rows_landed` over the table's receipts whose `started_at` is in
the window, replay receipts left out. Step 3 of `POST /mappings/{id}/replays` uses it for `estimate_rows` and for the
`E_COST_GUARD` refusal. `LandingStore::estimate` is called by the job only (akr-ingestd), before its first page, for
the `cost_guard` comparison of job step 7.

### 7.17 Replay receipts of a files table carry no file hash (V-11-03-b, HIGH; DR-413)
A replay page is not a file, yet the `admin.batches` CHECK asked every receipt of a `files` table for `file_sha256`.
Migration 0033 (section 1.1): the CHECK `((file_sha256 IS NOT NULL) = (delivery = 'files'))` becomes
`CHECK ((file_sha256 IS NOT NULL) = (delivery = 'files' AND NOT (manifest ? 'replay')))`. A replay receipt (section
3.8 apply step 2) has `file_name` and `file_sha256` NULL for every delivery.

### 7.18 The declaration call (V-11-04-a HIGH, V-11-04-c MEDIUM; DR-413)
`BlindPeriodService::declare(ctx, body: BlindPeriodDeclaration) -> Result<BlindPeriod, AppError>` is the call of
`POST /coverage/blind-periods`; phase 30 uses the same service for system periods. In one transaction:
1. Refuse a `scope` other than `system` or `rail` (422 `scope_invalid`).
2. Refuse a `started_at` or `ended_at` in the future (`in_future`), a `started_at` before the newest
   `audit.audit_anchors.anchored_at` (`started_before_anchor`), an `ended_at` before `started_at` or a `business_to`
   before `business_from` (`range_inverted`), and a `comment` outside 10 to 2,000 characters (`too_short`, `too_long`).
   A body without `ended_at` fails the schema (422 `E_VALIDATION` at `/ended_at`).
3. `BlindPeriodRepository::open(tx, &NewBlindPeriod)` inserts the row with its `ended_at`; the partial unique index
   covers source scope only, so the insert always returns a row.
4. Audit `blind_period.declared {scope, cause, started_at, ended_at}`.
A declared period is never open, so the automatic close of section 3.6 stays for source periods only and `akr_app`
needs no UPDATE grant on `admin.blind_periods`.

### 7.19 Change capture: spool errors, the consumer entry, the first unit and the poll interval (V-11-05-a, V-11-05-b, V-11-05-c, MEDIUM; DR-413)
1. `SpoolError` has `SequenceGap` (the next segment is not the successor of the last one read, or its first position
   does not follow the stored position), `HashConflict` (a repeated sequence with another SHA-256), `HashMismatch` (a
   segment differs from its `.sha256` file) and `Io`. The first two halt the source with `cdc_gap` (section 3.4 change
   capture step 2). `HashMismatch` ends the unit with a `rejected` receipt of reason `file_hash_mismatch` and nothing
   written, the outcome of a corrupted extract file. A segment with no `.sha256` file beside it is still being written
   and `segments_after` does not list it.
2. `ChangeCaptureConsumer::read_unit(source, table, after: Option<&str>, max_events: u32) -> Result<(Vec<ChangeEvent>,
   String), SpoolError>` is what `ingest_load` calls for a `cdc_position` table. It calls `segments_after` and
   `read_segment`, keeps the events of the table and returns them with the last position read. A unit takes whole
   segments until `ingest.max_unit_rows` events are read; a segment is never split. Overlap, settle and slices do not
   apply to this kind.
3. First unit: with no stored position `after` is None and the unit starts at the first segment of the spool. The
   sidecar's initial snapshot is its first segments; a snapshot event has `op` `r` and is handled as `c`.
4. Poll interval: `ingest_tick` rule 2 gains a case d: a `cdc_position` table is due at every tick. The 60 minutes of
   rule 2c would leave an idle table stale under `freshness.f1_lag_seconds` (900 seconds); the 30-second reschedule
   after a unit that read events stays.

### 7.20 The extract file contract: file format, columns and the row reader (V-11-07-a HIGH, V-11-07-c, V-11-07-d MEDIUM; DR-413)
Section 3.4 and section 7.5 said the column list of a `landing_file` table is "pinned in the adapter pack". No pack
holds one: section 3.9 lists database objects. The contract is:
1. A data file is UTF-8 CSV as RFC 4180 writes it (comma, double quote, CRLF or LF) with one header row. Each field
   lands as text under its header name in the raw payload, and an empty field is absent. Types are the mapping's work
   (phase 10 transforms).
2. The manifest `akr.extract.v1` gains `columns`, the ordered column names of the table. The header row of every data
   file must equal it, and every row must have that many fields. Otherwise the batch is `rejected` with reason
   `column_check_failed` and audited as `file.rejected`; the manifest stays pending and no row of the file lands. It
   is not a `row_failed` quarantine row, because a quarantine row needs a raw row (`raw_id`). The t24 decode failures
   stay row failures of section 3.2 step 5.
3. `ExtractInbox` gains `fn read_rows(&self, m: &ManifestRef, f: &ManifestFile, skip: u64, limit: u32) ->
   Result<Vec<SourceRow>, InboxError>;`, which parses one verified data file (after `verify_file`) into the row type
   that `SourceReader::fetch_keyset` returns.
4. The table is an approved object of the source like any other (phase 05 `source_access`); the manifest's
   `source_table` is the NAME part of the object. From this phase on, phase 10 compile steps 3 and 4 check `source.key`
   and every `from` of a `landing_file` mapping against the `columns` of the table's newest accepted manifest (its receipt's `manifest`
   column), which stands in for the profile. A table with no accepted manifest yet is refused with `E_VALIDATION` at
   `/source/object`, reason `no_delivery_yet`.

### 7.21 The record column and the `from` grammar of the text formats (V-11-07-b, V-11-08-a MEDIUM, V-11-08-b LOW; DR-413)
1. The mapping's `source` block gains the key `source.record`, the column that holds the record text
   (`t24_xmlrecord`, `t24_multivalue`) or the raw message (`iso20022`, `mt`). It is required for these four formats
   and refused as an unknown key for the others; compile step 3 checks it as it checks `source.key`. The schema
   `schemas/mapping.v1.json` and DSL.md B.1 gain it.
2. `from` for `iso20022`: element local names joined by `/`, starting at the child of `Document`, with an optional
   last step `@Name` for an attribute (`FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt/@Ccy`). Namespaces are ignored
   and the first match in document order is taken.
3. `from` for `mt`: a field tag of text block 4 with its option letter (`20`, `32A`, `50K`), optionally followed by
   `/N` for line N of the field, counted from 1. The two names `sender` and `receiver` give the BICs of blocks 1 and
   2. Splitting inside a line (the date, currency and amount of `32A`) is the mapping's `expr`.
4. The parser is `parse(format, raw: &str) -> Result<ParsedMessage, ParseError>` with `ParsedMessage::get(&self,
   from: &str) -> Option<&str>`; an absent element is null and a `ParseError` is the `message_parse_failed` row of
   section 7.6 step 5. A `from` that does not fit the grammar is refused at compile with `E_VALIDATION` at
   `/fields/<attr>/from`.
5. The parser version of section 7.6 step 3 is the key `parser_version` of the `compiled` artefact, so
   `compiled_sha256` covers it.

### 7.22 Who measures late arrivals and who writes the two events (V-11-02-b, V-11-12-c MEDIUM, V-11-12-d LOW; DR-413)
Section 7.12 named two audit events with no writer, no stored day and no planner input.
1. `BatchRepository` gains `late_stats(conn, source, table, since: DateTime<Utc>) -> Result<LateStats, RepoError>`
   with `LateStats { late_days_max: Option<u32>, lookback_days: Option<u32> }`: the largest `manifest.late_days_max`
   of the table's receipts started since `since`, and the `manifest.lookback_days` of its newest receipt.
2. `ingest_load` step 2 calls it with `since` 90 days back. The planner (section 3.1 step 3) takes `lookback` (the
   mapping's value: `auto`, a duration or none), `lag_days` (that `late_days_max`), `ingest.lookback_max_days` and
   `ingest.sequence_overlap_keys`. Every receipt records the window the planner used as `manifest.lookback_days`.
3. `ingest.lookback_widened` is written in the transaction of the run's first unit when that window differs from the
   `lookback_days` of the newest receipt (`from_days`, `to_days`). The first receipt of a table writes none.
4. `ingest.late_arrival_near_window` is written by the unit, in its transaction, when its own `late_days_max` is above
   `ingest.late_alarm_pct` percent of the window and above the `late_days_max` read in item 2. A table raises it when
   its tail grows, and no day is stored. The freshness job writes no late-arrival event; it sets the gauge of the
   alert rule block below.
5. The audit table of section 3.10 gains two rows with resource `watermark`: `ingest.late_arrival_near_window` (a unit
   whose late arrival is a new 90-day maximum near the window) and `ingest.lookback_widened` (a change of the auto
   window).

### 7.23 Alert rule and gauge for late arrivals (V-11-12-a, MEDIUM; DR-413)
Section 4.3 gains the gauge `akr_ingest_late_near_window{source, table}`: the freshness job (section 3.7) sets it to 1
when the largest `late_days_max` of the last 90 days (`BatchRepository::late_stats`) is above `ingest.late_alarm_pct`
percent of the table's window, else to 0. The alert rules gain "a table whose `akr_ingest_late_near_window` is 1".
Its inbox delivery is phase 18, as for the other rules of section 4.3.

### 7.24 The three settings of sections 7.10 and 7.12 in migration 0035 (V-11-12-b, LOW; DR-413)
The settings INSERT of section 1.3 gains three rows, all governed:
```sql
 ('ingest.sequence_overlap_keys', '10000', 'integer', '{"min": 100, "basis": "research 25 s.7.3 commit order"}', TRUE,
  '{"ar": "عدد المفاتيح التسلسلية التي تُعاد قراءتها قبل المفتاح المحفوظ", "en": "Sequence keys re-read before the stored key"}'),
 ('ingest.late_alarm_pct', '80', 'integer', NULL, TRUE,
  '{"ar": "نسبة إنذار الوصول المتأخر من نافذة إعادة القراءة (بالمئة)", "en": "Late-arrival alarm level (percent of the re-read window)"}'),
 ('ingest.lookback_max_days', '93', 'integer', NULL, TRUE,
  '{"ar": "أقصى مدة لنافذة إعادة القراءة التلقائية (أيام)", "en": "Longest automatic re-read window (days)"}')
```

### 7.25 `BlindFilter` (V-12-12-a, LOW; DR-413)
`BlindFilter` (`BlindPeriodRepository::list`) is `{ open: Option<bool>, source_id: Option<Uuid> }`. The route
`GET /coverage/blind-periods` takes `source_id?` beside `open?`; the phase 12 panel reads one source's periods with it.

### 7.26 The business date of the previous mapped batch (V-15-09-a, MEDIUM; DR-413)
From phase 15 on, `BatchRepository` gains `async fn previous_mapped_business_to(&self, conn: &mut PgConnection, source_id: Uuid, source_table: &str, batch_id: Uuid) -> Result<Option<NaiveDate>, RepoError>;`. It returns the `business_to` of the newest batch of `source_id` and `source_table` with status `mapped`, other than `batch_id`, and `None` for the first mapped batch of a table. Phase 15 section 7.7 step 5 calls it.

### 7.27 System periods written without a session (phase 30; V-30-07-a, MEDIUM; DR-413)
Phase 30 extends this phase and owns the extension (phase 30 section 7+): `BlindPeriodService::record_system(tx, started_at, ended_at, statement)`, with two callers, the `ops_probe` job and `akr-ctl ha outage record`. It writes a closed row of scope `system` and cause `akr_outage` through `BlindPeriodRepository::open`, with `declared_by` = the system principal, keeps the anchor rule of section 3.6, refuses an overlap with another such row, and audits `blind_period.opened`. It has no Cedar action. `declare` stays the only call of the steward route.

## Decisions

Recorded in research.md: DR-89 (the landing unit: raw rows, receipt, canonical writes, quarantine rows and the watermark
in one transaction; landing from a `testing` mapping with no published version; runtime landing factory; closed business
days only; first load from `ingest.history_from`; `data_as_of` from mapped receipts), DR-90 (restore and failover halts,
the resume decision with step-up `backfill`, automatic blind periods with system and rail declarations only), DR-91
(delivery modes: CT3 through an opt-in sidecar spool, CT4 files with a contiguous sequence, the signed `ics_banks@1.0.0`
pack), DR-92 (mapping replays: own table, masked while unpublished, volume effect from the replay, no settled-fact
change on apply), DR-93 (writer conflicts as `row_conflict` quarantine rows until phase 12, receipts R9, `GET
/freshness`, governed thresholds, no new problem code). The Cedar file `72_ingest.cedar` follows the phase 03 range
convention.

## Open doubts (for gap pass 1 and the bank)

1. Batch receipts as R9 (DR-93) must be checked against the bank's retention policy and the canon lineage rules; if the bank wants R1 for receipts, lineage must carry its own copy.
2. The correction path for a settled fact changed by a mapping fix (DR-92) is open: linked correction rows or a versioned fact key both need phase 10 changes that are not written here.
3. The closed-day rule `BANK_DATE - 1` is an inference from the discovery snapshot (CBS-DISCOVERY s.6, marked "inferred" for the end-of-day marker); it is confirmed per bank before any table is approved.
4. Intraday freshness (F2) for CT1 and CT2 depends on the bank's batch times and on reading the open day; both are deferred, and F3 end-of-day is what this phase delivers.
5. Raw payloads are personal and stored as plain JSONB in the landing layer; DATA-MODEL s.3 does not say whether raw is field-encrypted like the canon `_enc` columns, and the phase 02 envelope would change the payload type.
6. API.md 3.5 says the resume of a source plans "the backfill of the gap" from phase 11, while its backfill routes are phase 12; this phase implements automatic catch-up only, and planned backfills stay in phase 12.
7. DR-92 needs the change-request service of phase 04 to expose the approver ids of an open change request for a version; phase 04 section 7+ adds that port (`approver_ids`), so this doubt is closed.
8. As-of restore detection is reliable for Oracle SCN and PostgreSQL LSN; the SQL Server LSN, Db2 and Sybase markers need a probe in the phase 05 engine conformance before they can halt a source.
9. A business-date regression as a halt signal assumes the CBS never corrects a branch date backwards; a DBA correction would halt the source, and the bank's DBA must confirm the process.
10. Whether the bank accepts a Debezium Server sidecar in the source's zone, and its licence and operating footprint, are not decided (DR-91).
11. A fresh install is stale (`never_loaded`) until its first unit, which may trip the MLRO alert on day one; the alert rule should exclude sources in their first 24 hours, and the lead should decide.
12. The floor of two business days for `freshness.stale_business_days` is a proposal (DR-39), not a sourced figure.
13. The outbox event `dq.stale` has no consumer until phase 18, and its payload is the DSL's name, not yet a catalogued event kind.

## What This Phase Does NOT Touch

- No new CBS connection or grant, and no change to the read-only proof, the grant check or the statement journal of phase 05; CT3 and CT4 are delivery modes of sources the bank has approved.
- No mapping authoring, compile or validation, no change to the `CanonicalWriter` or to the canonical tables and their grants (phase 10); the replay reads the mapping, it does not change it.
- No DQ issue table, no reconciliation ladder or certificate, no coverage ledger or exclusion register, no MLRO coverage statement, and no operator-planned backfill plan (phase 12).
- No scenario evaluation, settled-time alerting, alert amendment or scenario replay (phases 16 and 17); the volume effect of a monitored change waits for them.
- No retention job: raw partitions are dropped by the retention engine (phase 23); this phase assigns the classes only.
- No inbox task, notification or delivery of halts and stale sources (phases 18 and 22), and no operations-cockpit tile (phase 24).
- No akr-worker job, no change to the phase 02 partition registry or to the audit chain; no new action class and no new problem code.
- No Islamic, screening, feature, rating or detection use of canon attributes (phases 13 to 15 and 26); no F0 or F4 measurement.
- No payment tie-out: match rules and rung L4 are P2 (C-43, DR-394). The payment formats and the gateway source are in section 7+.
- No restore drill on a bank's servers (SP-85 is operations); the detector is tested on the fixture only.
