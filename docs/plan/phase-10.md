# Phase 10: Mapping & Canonical Model

**Goal:** The mapping layer of the product: mapping documents (DSL.md Part B) held as admin-managed data and built from
a closed, versioned transform library with the card guard and survivorship rules; each document compiled into
read-only, proven `SELECT` statements over the approved objects of its source; code maps for transaction codes,
products, channels, countries, ID types, branches, purposes, statuses and dr/cr indicators, gated by kind, pre-filled
from the CBS's own code dictionary and confirmed by the bank before use, with currency codes typed by hand; drift
events that pause only the affected table; the `canon` schema (29 tables from DATA-MODEL.md section 5, facts
partitioned by month, bi-temporal parties and accounts, lineage on every row) written by the CanonicalWriter; and the
mapping workbench (screen T3) with a shape-only test and preview path. Production loads (watermarks, batches, quarantine,
replays at load time) are phase 11.

**Dependencies:** Phase 05 (sources, `SourceReader`, `SelectProof`, discovery profiles, field availability, code
lists), Phase 09 (`ref.countries`, `ref.currencies`, `ref.governorates` are foreign-key targets). The change-target and
shell work of phases 04 and 08 is used as built.
**Macro-phase:** P1 (the P0 draft of M6 is delivered by phases 05 and 06)
**Complexity:** XL (20 routes; the compiler, the test sampler, the canonical writer and the transform library carry the weight)
**Binaries touched:** akr-api (routers `mappings`, `code_maps`, `drift`, `canonical`, `attributes`) | akr-worker (jobs
`mapping_compile`, `drift_scan`, `code_map_prefill`, `mapping_preview_purge`) | akr-ingestd (job `ingest_mapping_sample`;
the `CanonicalWriter` library, called by phase 11) | `akr-ctl` (`mapping lint`, `canon verify-catalogue`). akr-assess unchanged.
**Requirements carried:**
- C-10 (character-set round trip and the Arabic and Kurdish name fields the transforms must preserve; research 25 s.3.1, 3.2)
- C-22, C-23, C-24 (the canonical write is idempotent on `(source, table, pk)` with row-hash guards; a changed settled
  row is not overwritten; research 25 s.7.4, 7.5, 7.7, 7.8, carried by the writer of section 3.7)
- C-36 and SP-48, SP-51 (card guard before landing, `mask_pan`, no PAN in the canon, no PCI scope; research 25 s.10.7,
  research 30 s.11.5 and 11.8)
- C-46 (canonical model with lineage on every row, bi-temporal entities, UUIDv7 ids, positive amounts with dr/cr,
  classification per column; research 25 s.13.1)
- C-47 (the entity set of research 25 s.13.2: the 29 canon tables of section 1)
- C-50, C-51, C-52 (admin-managed mapping documents with a closed library; maker-checker lifecycle; drift pausing only the
  affected table; research 25 s.14.1 to 14.4)
- N-12 (name data-quality profile reaches the mapping as a per-field note; research 22 s.6.1, 6.6)
- Gap pass 1 (coverage), implemented here: C-17 (lifecycle events from initiated to cancelled), C-18 (booking, value and business dates, the source local timestamp and the bank zone), C-49 (monthly partitions, wrapped-key encryption, keyed match hashes); VD-12 (bank fields and tables as extension attributes of the adapter pack).
- Gap pass 1 (coverage), partly implemented: C-37 (field references through the adapter pack dictionary), C-43 (the payment-link table and its mapping path; match rules and rung L4 are P2, DR-394), C-44 (IBAN checksum and decode), C-45 (the ISO-shaped payment model), C-53 (survivorship rules; merge and split are P2, DR-394), RP-10 (effective-dated code maps under change requests), S-11 (the counterparty table and its link on each transaction), S-16 (cleared and available balances), S-18 (the facility table), S-21 (cash events with denominations; the operator on each transaction); K-32 (the family number, the migrated-record placeholder flag; name variants are phase 14 and Hijri dates phase 09); L-06 (drift findings on new, changed and relabelled code values), L-07 (versioned mappings under change requests; schema and code-list drift), L-15 (the cash flag set by the code map), L-38 (quality flags on payment parties), L-39 (rejected and returned payment statuses), L-50 (the CBS CDD level on each customer); W-14 (the canonical trade-instrument table); VA-31 (the product-agnostic canonical model with the product table); VD-10 (lineage on every canonical row), VD-21 (stable canonical ids and mapping change requests with replay), VD-25 (the channel table and ISO 20022 party fields); VC-70 (row lineage on every canonical row), VC-72 (generic data-quality rules and quarantine).
- Gap pass 1 (coverage), not carried: T-26 (persistent entity resolution with unmerge, P2, DR-394); W-12 (payment-quality monitor, DR-400), W-13 (bundled and on-behalf-of payments, P2, DR-394); VA-41 (model-proposed mappings; P2, DR-402).

---

## 1. SQLx Migration Changes

Four migrations in the canonical chain (roadmap.md): `0029_mapping_registry`, `0030_canon_parties_accounts`,
`0031_canon_transactions` and `0032_canon_cards_activity`. They are group 10 of DATA-MODEL.md section 23 (`admin` mapping,
then `canon` with partitions). Phase 09 and phase 05 objects are referenced; nothing in this phase alters them.
Phase 11 adds the foreign key from `canon.*.batch_id` to `admin.batches` when it creates that table (group 11 follows
this one), so `batch_id` is a plain `UUID NOT NULL` here.

**Conventions applied to every table below** (DATA-MODEL.md section 0, phase 00):
- `id UUID PRIMARY KEY` with no default. Canonical versions of a bi-temporal entity share their `id` (section 2).
- `[lineage]` is the eight columns `source_id, source_table, source_pk, source_version, row_hash, batch_id, extracted_at,
  mapping_version_id`. Every canon table carries them. `source_pk` holds the key parts joined by `|` in the order the
  mapping lists them (DR-41).
- Encrypted personal columns are `<name>_enc BYTEA` (AES-GCM envelope, DATA-MODEL.md section 0) and their keyed-hash
  twins are `<name>_hk BYTEA` (HMAC-SHA256, match key). Both are classed `personal`. DATA-MODEL marks `enc` and `hk` on
  specific columns only; the table below follows that list.
- `source_local_ts` is `TEXT`, not `timestamp`: phase 00 forbids `timestamp without time zone` in any column, so the
  source's literal (ISO 8601 without zone, as read) is kept as text beside the UTC `posted_at` (DR-87).
- Nullability: a column is `NOT NULL` unless DATA-MODEL.md section 5 marks it `null`, or the text says it stays null
  until a source column is confirmed (for example `reverses_id`, `corrected_by_id`, `overlap` on `facility`). Where the
  DATA-MODEL row is silent, the column is nullable and the note says so.
- Money: `NUMERIC(28,6)`; amounts are positive, the sign is `dr_cr` (`D` or `C`), `currency CHAR(3)` references
  `ref.currencies(code)` (phase 09), and an unmapped currency never defaults to IQD (DATA-MODEL.md section 0, DR-36).
- No application role has DELETE on `canon`. Partitions are dropped only by the retention engine (phase 23) under its own role.

### 1.1 Migration `0029_mapping_registry.sql`

```sql
-- migration 0029_mapping_registry.sql
-- Phase 10. Mapping families and versions, code maps, drift events, the source-key cross-reference, the attribute
-- registry and mapping runs (DATA-MODEL s.4, DSL.md Part B, C-50 to C-52). The canonical tables follow in the next
-- migrations. Nothing here is applied to a source; the sampler (section 3.6) is the only reader of CBS rows.

DO $$ BEGIN
    CREATE TYPE admin.mapping_status AS ENUM ('draft', 'testing', 'approved', 'published', 'retired');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

DO $$ BEGIN
    CREATE TYPE admin.mapping_run_status AS ENUM ('queued', 'running', 'completed', 'failed');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

-- One family per source and canonical entity (DSL.md B.1: "one active mapping per source and entity").
CREATE TABLE IF NOT EXISTS admin.mappings (
    id                  UUID PRIMARY KEY,
    dsl_id              TEXT NOT NULL UNIQUE CHECK (dsl_id ~ '^[a-z][a-z0-9_]{1,31}\.[a-z][a-z_]{1,39}$'),  -- <source>.<entity>
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    target_entity       TEXT NOT NULL CHECK (target_entity ~ '^[a-z][a-z_]{1,39}$'),   -- checked against the catalogue
    name                JSONB NOT NULL CHECK (name ? 'ar' AND name ? 'en'),
    created_by          UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    CONSTRAINT uq_mappings_source_entity UNIQUE (source_id, target_entity)
);
CREATE INDEX IF NOT EXISTS idx_mappings_created_by ON admin.mappings (created_by);

-- Versions. `version` is the DSL version (increments on every publish, DSL.md B.1); `revision` is the If-Match lock
-- while the draft is edited (API.md section 1, concurrency). The compiled artefact (section 3.2) is written by the
-- validate route and by mapping_compile; a version that does not compile never leaves draft.
CREATE TABLE IF NOT EXISTS admin.mapping_versions (
    id                  UUID PRIMARY KEY,
    mapping_id          UUID NOT NULL REFERENCES admin.mappings(id) ON DELETE RESTRICT,
    version             INT NOT NULL CHECK (version >= 1),
    revision            INT NOT NULL DEFAULT 1 CHECK (revision >= 1),
    document            JSONB NOT NULL CHECK (jsonb_typeof(document) = 'object'),
    document_sha256     BYTEA NOT NULL CHECK (octet_length(document_sha256) = 32),
    status              admin.mapping_status NOT NULL DEFAULT 'draft',
    compiled            JSONB,
    compiled_sha256     BYTEA CHECK (compiled_sha256 IS NULL OR octet_length(compiled_sha256) = 32),
    library_version     TEXT CHECK (library_version IS NULL OR library_version ~ '^[0-9]+\.[0-9]+\.[0-9]+$'),
    change_request_id   UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    published_at        TIMESTAMPTZ,
    created_by          UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_mapping_versions_number UNIQUE (mapping_id, version),
    CHECK (status = 'draft' OR compiled IS NOT NULL),
    CHECK (status NOT IN ('approved', 'published') OR change_request_id IS NOT NULL),
    CHECK ((status = 'published') = (published_at IS NOT NULL))
);
-- One open version per mapping, and one published version per mapping (publish replaces the previous one).
CREATE UNIQUE INDEX IF NOT EXISTS uq_mapping_versions_open
    ON admin.mapping_versions (mapping_id) WHERE status IN ('draft', 'testing', 'approved');
CREATE UNIQUE INDEX IF NOT EXISTS uq_mapping_versions_published
    ON admin.mapping_versions (mapping_id) WHERE status = 'published';
CREATE INDEX IF NOT EXISTS idx_mapping_versions_cr         ON admin.mapping_versions (change_request_id)
    WHERE change_request_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_mapping_versions_created_by ON admin.mapping_versions (created_by);

-- Code maps: one per source, kind and CBS code list (DATA-MODEL s.4). `gate` is what happens to an unmapped value
-- (DSL.md B.7). Currency maps are typed by hand from the bank's currency table and are never prefilled (section 3.4).
CREATE TABLE IF NOT EXISTS admin.code_maps (
    id              UUID PRIMARY KEY,
    kind            TEXT NOT NULL CHECK (kind IN ('txn_type', 'product', 'channel', 'country', 'id_type', 'branch',
                                                  'purpose', 'currency', 'status', 'dr_cr')),
    source_id       UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_list     TEXT NOT NULL CHECK (length(source_list) BETWEEN 1 AND 120),
    gate            TEXT NOT NULL CHECK (gate IN ('quarantine', 'dq_issue')),
    name            JSONB NOT NULL CHECK (name ? 'ar' AND name ? 'en'),
    created_by      UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    version         INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    CONSTRAINT uq_code_maps_list UNIQUE (source_id, kind, source_list),
    CHECK (kind <> 'txn_type' OR gate = 'quarantine')       -- DSL.md B.7: txn_type is quarantined, never "other"
);
CREATE INDEX IF NOT EXISTS idx_code_maps_created_by ON admin.code_maps (created_by);

-- Entries are effective-dated and keyed by the CBS list they belong to (DATA-MODEL s.4: a generic column name can
-- point to many lists). origin `prefill` rows exist before any bank decision and are unmapped under the gate until a
-- change request sets verified_by_bank_at (section 3.4).
CREATE TABLE IF NOT EXISTS admin.code_map_entries (
    id                    UUID PRIMARY KEY,
    code_map_id           UUID NOT NULL REFERENCES admin.code_maps(id) ON DELETE RESTRICT,
    source_list           TEXT NOT NULL CHECK (length(source_list) BETWEEN 1 AND 120),
    source_code           TEXT NOT NULL CHECK (length(source_code) BETWEEN 1 AND 200),
    canonical_code        TEXT NOT NULL CHECK (length(canonical_code) BETWEEN 1 AND 60),
    origin                TEXT NOT NULL CHECK (origin IN ('prefill', 'change_request')),
    valid_from            DATE NOT NULL,
    valid_to              DATE,
    change_request_id     UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    verified_by_bank_at   TIMESTAMPTZ,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_code_map_entries_effective UNIQUE (code_map_id, source_list, source_code, valid_from),
    CHECK (valid_to IS NULL OR valid_to > valid_from),
    CHECK ((origin = 'change_request') = (change_request_id IS NOT NULL))
);
CREATE INDEX IF NOT EXISTS idx_code_map_entries_lookup ON admin.code_map_entries (code_map_id, source_list, source_code,
                                                                                 valid_from DESC);
CREATE INDEX IF NOT EXISTS idx_code_map_entries_cr     ON admin.code_map_entries (change_request_id)
    WHERE change_request_id IS NOT NULL;

-- Drift (C-52). `severity` is fixed by kind (section 3.5): `pause` stops the affected table, `gate` quarantines the
-- rows that carry the new value, `notify` changes nothing in the pipeline. One open event per kind and table.
CREATE TABLE IF NOT EXISTS admin.drift_events (
    id                  UUID PRIMARY KEY,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL CHECK (source_table ~ '^[^.@\s]+\.[^.@\s]+$'),   -- OWNER.NAME as catalogued
    kind                TEXT NOT NULL CHECK (kind IN ('new_table', 'new_column', 'removed_column', 'type_change',
                                                      'removed_object', 'new_code', 'value_drift', 'label_change')),
    severity            TEXT NOT NULL CHECK (severity IN ('pause', 'gate', 'notify')),
    detail              JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(detail) = 'object'),
    run_id              UUID REFERENCES admin.discovery_runs(id) ON DELETE RESTRICT,
    mapping_version_id  UUID REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    paused              BOOLEAN NOT NULL DEFAULT FALSE,
    resolved_at         TIMESTAMPTZ,
    resolved_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    CHECK ((resolved_at IS NULL) = (resolved_by IS NULL)),
    CHECK (paused = FALSE OR (severity = 'pause' AND resolved_at IS NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_drift_open ON admin.drift_events (source_id, source_table, kind)
    WHERE resolved_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_drift_paused    ON admin.drift_events (source_id, source_table) WHERE paused;
CREATE INDEX IF NOT EXISTS idx_drift_run       ON admin.drift_events (run_id) WHERE run_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_drift_version   ON admin.drift_events (mapping_version_id)
    WHERE mapping_version_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_drift_resolved_by ON admin.drift_events (resolved_by) WHERE resolved_by IS NOT NULL;

-- Source key to canonical id (DATA-MODEL s.4). Written only by the CanonicalWriter; never updated.
-- canonical_id is polymorphic across entity tables and carries no foreign key (section 2, bi-temporal parents).
CREATE TABLE IF NOT EXISTS admin.xref (
    source_id       UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_key      TEXT NOT NULL CHECK (length(source_key) BETWEEN 1 AND 300),
    entity          TEXT NOT NULL CHECK (entity ~ '^[a-z][a-z_]{1,39}$'),
    canonical_id    UUID NOT NULL,
    linked_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (source_id, source_key, entity)
);
CREATE INDEX IF NOT EXISTS idx_xref_canonical ON admin.xref (canonical_id);

-- Attribute registry (C-46: "a typed attribute registry for bank-specific fields"). Entries are classed; a value of a
-- personal or secret-classed attribute is stored as an envelope (section 3.4).
CREATE TABLE IF NOT EXISTS admin.attribute_definitions (
    id              UUID PRIMARY KEY,
    entity          TEXT NOT NULL CHECK (entity ~ '^[a-z][a-z_]{1,39}$'),
    name            TEXT NOT NULL CHECK (name ~ '^[a-z][a-z0-9_]{1,47}$'),
    type            TEXT NOT NULL CHECK (type IN ('text', 'integer', 'decimal', 'boolean', 'date', 'timestamp', 'code')),
    classification  admin.data_classification NOT NULL,
    owner_id        UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    doc             JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(doc) = 'object'),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    version         INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    CONSTRAINT uq_attribute_definitions UNIQUE (entity, name)
);
CREATE INDEX IF NOT EXISTS idx_attribute_definitions_owner ON admin.attribute_definitions (owner_id);

-- Attribute values are bi-temporal (DATA-MODEL s.4). The entity id has no foreign key (section 2).
CREATE TABLE IF NOT EXISTS admin.attribute_values (
    entity              TEXT NOT NULL CHECK (entity ~ '^[a-z][a-z_]{1,39}$'),
    entity_id           UUID NOT NULL,
    attribute_id        UUID NOT NULL REFERENCES admin.attribute_definitions(id) ON DELETE RESTRICT,
    value               JSONB NOT NULL,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    valid_from          DATE NOT NULL,
    valid_to            DATE,
    recorded_from       TIMESTAMPTZ NOT NULL,
    recorded_to         TIMESTAMPTZ,
    PRIMARY KEY (entity, entity_id, attribute_id, recorded_from),
    CHECK (valid_to IS NULL OR valid_to > valid_from),
    CHECK (recorded_to IS NULL OR recorded_to > recorded_from)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_attribute_values_current
    ON admin.attribute_values (entity, entity_id, attribute_id) WHERE recorded_to IS NULL;
CREATE INDEX IF NOT EXISTS idx_attribute_values_attr    ON admin.attribute_values (attribute_id);
CREATE INDEX IF NOT EXISTS idx_attribute_values_source  ON admin.attribute_values (source_id);
CREATE INDEX IF NOT EXISTS idx_attribute_values_mv      ON admin.attribute_values (mapping_version_id);

-- Runs of the sampler (section 3.6). `test` carries the impact summary against the published version; `preview`
-- carries shape rows only and is purged after mapping.preview_keep_hours. A run judges the document it was queued
-- with (document_sha256), so an edit after queueing cannot change its result.
CREATE TABLE IF NOT EXISTS admin.mapping_runs (
    id                  UUID PRIMARY KEY,
    version_id          UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    kind                TEXT NOT NULL CHECK (kind IN ('test', 'preview')),
    status              admin.mapping_run_status NOT NULL DEFAULT 'queued',
    document_sha256     BYTEA NOT NULL CHECK (octet_length(document_sha256) = 32),
    sample_spec         JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(sample_spec) = 'object'),
    result              JSONB CHECK (result IS NULL OR jsonb_typeof(result) = 'object'),
    preview             JSONB CHECK (preview IS NULL OR jsonb_typeof(preview) = 'array'),
    preview_purged_at   TIMESTAMPTZ,
    error_code          TEXT CHECK (error_code IN ('source_refused', 'source_paused', 'source_not_active',
                                                   'object_missing', 'statement_timeout', 'budget_window',
                                                   'sql_refused', 'internal')),
    requested_by        UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    started_at          TIMESTAMPTZ,
    finished_at         TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    CHECK ((status = 'failed') = (error_code IS NOT NULL)),
    CHECK ((status IN ('completed', 'failed')) = (finished_at IS NOT NULL)),
    CHECK (preview IS NULL OR preview_purged_at IS NULL)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_mapping_runs_active ON admin.mapping_runs (version_id, kind)
    WHERE status IN ('queued', 'running');
CREATE INDEX IF NOT EXISTS idx_mapping_runs_version      ON admin.mapping_runs (version_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_mapping_runs_requested_by ON admin.mapping_runs (requested_by);
CREATE INDEX IF NOT EXISTS idx_mapping_runs_preview      ON admin.mapping_runs (created_at) WHERE preview IS NOT NULL;

-- Settings and governance (admin-managed, DATA-MODEL s.1). `mapping.default_overlap_days` is governed and floored at
-- the DSL re-read window (DSL.md B.2): a narrower window is a detection change and goes through a change request.
INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('mapping.test_sample_rows',        '500', 'integer', NULL, FALSE,
  '{"ar": "عدد صفوف العينة لاختبار التعيين", "en": "Sample rows for a mapping test run"}'),
 ('mapping.preview_keep_hours',      '24', 'integer', NULL, FALSE,
  '{"ar": "مدة حفظ معاينة التعيين (ساعات)", "en": "Hours a mapping preview is kept"}'),
 ('mapping.default_overlap_days',    '31', 'integer', '{"min": 31, "basis": "DSL.md B.2 re-read window"}', TRUE,
  '{"ar": "نافذة إعادة القراءة الافتراضية (أيام عمل الحركة)", "en": "Default re-read window (days of business date)"}'),
 ('mapping.value_drift_points',      '20', 'integer', NULL, FALSE,
  '{"ar": "نقاط تغيّر حصة القيمة قبل إنذار الانحراف", "en": "Share change in points that raises value drift"}'),
 ('schedule.mapping_preview_purge',  '"0 50 1 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول حذف المعاينات المنتهية", "en": "Expired mapping preview purge schedule"}')
ON CONFLICT (key) DO NOTHING;

-- Classification (DATA-MODEL s.0; phase 00 check). The document and compiled artefact are the detection logic of the
-- bank; a preview is personal because its shape rows describe personal columns.
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('mappings', 'internal', ARRAY['id','dsl_id','source_id','target_entity','name','created_by','created_at',
                                 'updated_at','version']),
  ('mapping_versions', 'detection-logic', ARRAY['document','document_sha256','compiled','compiled_sha256',
                                 'library_version','status','version']),
  ('mapping_versions', 'internal', ARRAY['id','mapping_id','revision','change_request_id','published_at',
                                 'created_by','created_at','updated_at']),
  ('code_maps', 'internal', ARRAY['id','kind','source_id','source_list','gate','name','created_by','created_at',
                                 'updated_at','version']),
  ('code_map_entries', 'internal', ARRAY['id','code_map_id','source_list','source_code','canonical_code','origin',
                                 'valid_from','valid_to','change_request_id','verified_by_bank_at','created_at']),
  ('drift_events', 'internal', ARRAY['id','source_id','source_table','kind','severity','detail','run_id',
                                 'mapping_version_id','paused','resolved_at','resolved_by','created_at','version']),
  ('xref', 'personal', ARRAY['source_key']),
  ('xref', 'internal', ARRAY['source_id','entity','canonical_id','linked_at']),
  ('attribute_definitions', 'internal', ARRAY['id','entity','name','type','classification','owner_id','doc',
                                 'created_at','updated_at','version']),
  ('attribute_values', 'personal', ARRAY['value']),
  ('attribute_values', 'internal', ARRAY['entity','entity_id','attribute_id','source_id','mapping_version_id',
                                 'valid_from','valid_to','recorded_from','recorded_to']),
  ('mapping_runs', 'internal', ARRAY['id','version_id','kind','status','document_sha256','sample_spec','result',
                                 'preview_purged_at','error_code','requested_by','started_at','finished_at',
                                 'created_at','version']),
  ('mapping_runs', 'personal', ARRAY['preview'])
) AS v(t, cls, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'mappings',               'R9', 'retirement', NULL,         'Mapping family; versions cite it'),
 ('admin', 'mapping_versions',       'R9', 'retirement', NULL,         'Every published version kept: old runs cite it'),
 ('admin', 'code_maps',              'R9', 'retirement', NULL,         'Code list configuration'),
 ('admin', 'code_map_entries',       'R9', 'retirement', NULL,         'Reclassified entries kept with valid_to (DSL B.7)'),
 ('admin', 'drift_events',           'R9', 'age',        'created_at', 'Explains pauses and gates; kept as configuration history'),
 ('admin', 'xref',                   'R3', 'owner',            NULL,   'Follows the customer it links'),
 ('admin', 'attribute_definitions',  'R9', 'retirement', NULL,         'Registry entries'),
 ('admin', 'attribute_values',       'R3', 'owner',            NULL,   'Follows its entity'),
 ('admin', 'mapping_runs',           'R9', 'age',        'created_at', 'Preview column nulled after mapping.preview_keep_hours')
ON CONFLICT (schema_name, table_name) DO NOTHING;

-- Grants (DATA-MODEL s.21). akr_app: the API and the worker (compile, drift, prefill). akr_ingest: the sampler and the
-- CanonicalWriter. akr_readonly: reports and the assessor, no cross-reference or attribute values.
GRANT SELECT, INSERT ON admin.mappings, admin.mapping_versions, admin.code_maps, admin.code_map_entries,
    admin.drift_events, admin.attribute_definitions, admin.mapping_runs TO akr_app;
GRANT UPDATE (name, version, updated_at) ON admin.mappings TO akr_app;
GRANT UPDATE (document, document_sha256, revision, status, compiled, compiled_sha256, library_version,
              change_request_id, published_at, updated_at) ON admin.mapping_versions TO akr_app;
GRANT UPDATE (name, updated_at, version) ON admin.code_maps TO akr_app;
GRANT UPDATE (valid_to, verified_by_bank_at, change_request_id) ON admin.code_map_entries TO akr_app;
GRANT UPDATE (paused, resolved_at, resolved_by, version) ON admin.drift_events TO akr_app;
GRANT UPDATE (status, preview, preview_purged_at, version) ON admin.mapping_runs TO akr_app;
GRANT UPDATE (doc, updated_at, version) ON admin.attribute_definitions TO akr_app;
GRANT SELECT ON admin.xref, admin.attribute_values TO akr_app;

GRANT SELECT ON admin.mappings, admin.mapping_versions, admin.code_maps, admin.code_map_entries,
    admin.drift_events, admin.attribute_definitions, admin.mapping_runs, admin.xref, admin.attribute_values
    TO akr_ingest;
GRANT INSERT ON admin.xref, admin.attribute_values TO akr_ingest;
GRANT UPDATE (recorded_to) ON admin.attribute_values TO akr_ingest;
GRANT UPDATE (status, result, preview, error_code, started_at, finished_at, version) ON admin.mapping_runs TO akr_ingest;

GRANT SELECT ON admin.mappings, admin.mapping_versions, admin.code_maps, admin.code_map_entries,
    admin.drift_events, admin.attribute_definitions TO akr_readonly;
```

### 1.2 Migration `0030_canon_parties_accounts.sql`

```sql
-- migration 0030_canon_parties_accounts.sql
-- Phase 10. The canonical reference entities (branch, product, channel, staff) and the party and account entities
-- (DATA-MODEL s.5, C-46, C-47). Bi-temporal entities (party, party_relationship, customer, account, account_party_role)
-- key their versions on (id, recorded_from): one stable id, one row per version, one current row per source key.
-- Writes by akr_ingest only, reads by akr_app (DATA-MODEL s.21). Retention R3 for relationship data, R9 for reference.

CREATE SCHEMA IF NOT EXISTS canon;

CREATE TABLE IF NOT EXISTS canon.branch (
    id                  UUID PRIMARY KEY,
    code                TEXT NOT NULL UNIQUE CHECK (length(code) BETWEEN 1 AND 20),
    name                JSONB NOT NULL CHECK (name ? 'ar' AND name ? 'en'),
    governorate         TEXT REFERENCES ref.governorates(code) ON DELETE RESTRICT,
    region              TEXT,
    kurdistan           BOOLEAN NOT NULL DEFAULT FALSE,
    type                TEXT,
    opened_on           DATE,
    org_branch_id       UUID REFERENCES admin.branches(id) ON DELETE RESTRICT,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS canon.product (
    id                      UUID PRIMARY KEY,
    code                    TEXT NOT NULL UNIQUE CHECK (length(code) BETWEEN 1 AND 40),
    family                  TEXT NOT NULL,
    islamic_contract_type   TEXT CHECK (islamic_contract_type IN ('murabaha', 'musharaka', 'mudaraba', 'ijara',
                                                                  'qard_hasan', 'istisna', 'salam', 'other')),
    risk_attrs              JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(risk_attrs) = 'object'),
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS canon.channel (
    id                  UUID PRIMARY KEY,
    code                TEXT NOT NULL UNIQUE CHECK (length(code) BETWEEN 1 AND 40),
    kind                TEXT NOT NULL,
    risk_attr           JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(risk_attr) = 'object'),
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS canon.staff (
    id                  UUID PRIMARY KEY,
    employee_id         TEXT NOT NULL UNIQUE CHECK (length(employee_id) BETWEEN 1 AND 40),
    party_id            UUID,                                -- no foreign key: party is bi-temporal (section 2)
    branch_id           UUID NOT NULL REFERENCES canon.branch(id) ON DELETE RESTRICT,
    role                TEXT NOT NULL,                       -- 'system' for a bank-confirmed batch operator (DR-42)
    authority_limits    JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(authority_limits) = 'object'),
    leave               JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(leave) = 'array'),
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Bi-temporal: person and entity (DATA-MODEL s.5). birth_date and established_on are routed by the person/entity
-- indicator; pep_status is never taken from an undeclared CBS flag (DR-38).
CREATE TABLE IF NOT EXISTS canon.party (
    id                  UUID NOT NULL,
    type                TEXT NOT NULL CHECK (type IN ('person', 'entity', 'arrangement')),
    status              TEXT NOT NULL CHECK (status IN ('active', 'inactive', 'closed', 'unknown')),
    nationality         CHAR(2) REFERENCES ref.countries(iso2) ON DELETE RESTRICT,
    residence_country   CHAR(2) REFERENCES ref.countries(iso2) ON DELETE RESTRICT,
    birth_date          DATE,
    birth_precision     TEXT CHECK (birth_precision IN ('day', 'month', 'year', 'unknown')),
    birth_place         TEXT,
    established_on      DATE,
    gender              TEXT CHECK (gender IN ('male', 'female', 'unknown')),
    occupation_isic     TEXT,
    legal_form          TEXT,
    registration_no_enc BYTEA,
    registration_no_hk  BYTEA,
    tax_no_enc          BYTEA,
    tax_no_hk           BYTEA,
    mother_name_enc     BYTEA,
    family_no_enc       BYTEA,
    family_no_hk        BYTEA,
    pep_status          TEXT NOT NULL DEFAULT 'unknown' CHECK (pep_status IN ('unknown', 'not_pep', 'pep')),
    staff_flag          BOOLEAN NOT NULL DEFAULT FALSE,
    related_to_staff    BOOLEAN,                             -- null until a named derivation rule sets it, never false
    sensitivity_tier    TEXT NOT NULL DEFAULT 'normal' CHECK (sensitivity_tier IN ('normal', 'restricted', 'suspicion')),
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    valid_from          DATE NOT NULL,
    valid_to            DATE,
    recorded_from       TIMESTAMPTZ NOT NULL,
    recorded_to         TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (id, recorded_from),
    CHECK (valid_to IS NULL OR valid_to > valid_from),
    CHECK (recorded_to IS NULL OR recorded_to > recorded_from),
    CHECK (type = 'person' OR birth_date IS NULL),
    CHECK (type = 'entity' OR established_on IS NULL)
);

CREATE TABLE IF NOT EXISTS canon.party_name (
    id                  UUID PRIMARY KEY,
    party_id            UUID NOT NULL,                       -- no foreign key (section 2)
    raw                 TEXT NOT NULL,
    light               TEXT NOT NULL,
    folded              TEXT NOT NULL,
    key                 TEXT NOT NULL,
    script              TEXT NOT NULL CHECK (script IN ('ar', 'lat', 'ku', 'mixed', 'other')),
    language            TEXT CHECK (language IN ('ar', 'ku', 'en')),
    name_type           TEXT NOT NULL CHECK (name_type IN ('legal', 'trade', 'alias', 'as_in_message', 'former')),
    source_field        TEXT NOT NULL,
    capture_source      TEXT NOT NULL DEFAULT 'typed' CHECK (capture_source IN ('card', 'passport', 'typed', 'generated')),   -- section 7.26
    parts               JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(parts) = 'array'),
    alias_class         TEXT,
    quality_flags       TEXT[] NOT NULL DEFAULT '{}',
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    superseded_at       TIMESTAMPTZ,                         -- set when a changed hash replaces the row (DR-87)
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS canon.party_identifier (
    id                  UUID PRIMARY KEY,
    party_id            UUID NOT NULL,
    type                TEXT NOT NULL,
    value_enc           BYTEA NOT NULL,
    value_hk            BYTEA,                               -- null for a placeholder: placeholders never match
    placeholder_flag    BOOLEAN NOT NULL DEFAULT FALSE,
    issuer              TEXT,
    country             CHAR(2) REFERENCES ref.countries(iso2) ON DELETE RESTRICT,
    issued_on           DATE,
    expires_on          DATE,
    expiry_flag         TEXT NOT NULL DEFAULT 'missing' CHECK (expiry_flag IN ('missing', 'impossible', 'expired', 'ok')),
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    superseded_at       TIMESTAMPTZ,                         -- set when a changed hash replaces the row (DR-87)
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (NOT placeholder_flag OR value_hk IS NULL),
    CHECK (expiry_flag <> 'impossible' OR expires_on IS NULL)
);

CREATE TABLE IF NOT EXISTS canon.party_contact (
    id                  UUID PRIMARY KEY,
    party_id            UUID NOT NULL,
    type                TEXT NOT NULL CHECK (type IN ('mobile', 'landline', 'email', 'address', 'other')),
    value_enc           BYTEA NOT NULL,
    value_hk            BYTEA,
    address             JSONB CHECK (address IS NULL OR jsonb_typeof(address) = 'object'),
    loaded_first_at     TIMESTAMPTZ NOT NULL,                -- AKR load times; no source change date is assumed
    loaded_last_at      TIMESTAMPTZ NOT NULL,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    superseded_at       TIMESTAMPTZ,                         -- set when a changed hash replaces the row (DR-87)
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (loaded_last_at >= loaded_first_at)
);

-- Bi-temporal. share_pct is null when the source holds no percentage; a test never reads null as zero (DATA-MODEL s.5).
CREATE TABLE IF NOT EXISTS canon.party_relationship (
    id                  UUID NOT NULL,
    from_party          UUID NOT NULL,
    to_party            UUID NOT NULL,
    type                TEXT NOT NULL CHECK (type IN ('owner', 'controller', 'director', 'signatory', 'attorney',
                                                      'guardian', 'family', 'employee_of', 'associate', 'unknown')),
    share_pct           NUMERIC(5,2) CHECK (share_pct IS NULL OR share_pct BETWEEN 0 AND 100),
    degree              SMALLINT CHECK (degree IS NULL OR degree BETWEEN 1 AND 9),
    evidence            JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(evidence) = 'object'),
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    valid_from          DATE NOT NULL,
    valid_to            DATE,
    recorded_from       TIMESTAMPTZ NOT NULL,
    recorded_to         TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (id, recorded_from),
    CHECK (from_party <> to_party),
    CHECK (valid_to IS NULL OR valid_to > valid_from),
    CHECK (recorded_to IS NULL OR recorded_to > recorded_from)
);

-- Bi-temporal. closed_on is set only for the closed status (DATA-MODEL s.5, account). Holds no balance.
CREATE TABLE IF NOT EXISTS canon.customer (
    id                  UUID NOT NULL,
    party_id            UUID NOT NULL,
    cif                 TEXT NOT NULL CHECK (length(cif) BETWEEN 1 AND 40),
    branch_id           UUID NOT NULL REFERENCES canon.branch(id) ON DELETE RESTRICT,
    rm_id               UUID REFERENCES canon.staff(id) ON DELETE RESTRICT,
    segment             TEXT,
    onboarded_on        DATE,
    channel_id          UUID REFERENCES canon.channel(id) ON DELETE RESTRICT,
    status              TEXT NOT NULL,
    cdd_level           TEXT CHECK (cdd_level IN ('sdd', 'cdd', 'edd')),
    expected_profile    JSONB CHECK (expected_profile IS NULL OR jsonb_typeof(expected_profile) = 'object'),
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    valid_from          DATE NOT NULL,
    valid_to            DATE,
    recorded_from       TIMESTAMPTZ NOT NULL,
    recorded_to         TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (id, recorded_from),
    CHECK (valid_to IS NULL OR valid_to > valid_from),
    CHECK (recorded_to IS NULL OR recorded_to > recorded_from)
);

-- Bi-temporal. The number and the IBAN are never stored in clear: envelope plus keyed-hash twins.
CREATE TABLE IF NOT EXISTS canon.account (
    id                  UUID NOT NULL,
    customer_id         UUID NOT NULL,
    number_enc          BYTEA NOT NULL,
    number_hk           BYTEA NOT NULL,
    iban_hk             BYTEA,
    type                TEXT NOT NULL,
    product_id          UUID NOT NULL REFERENCES canon.product(id) ON DELETE RESTRICT,
    currency            CHAR(3) NOT NULL REFERENCES ref.currencies(code) ON DELETE RESTRICT,
    branch_id           UUID NOT NULL REFERENCES canon.branch(id) ON DELETE RESTRICT,
    opened_on           DATE NOT NULL,
    closed_on           DATE,
    status              TEXT NOT NULL CHECK (status IN ('active', 'closed', 'dormant', 'stopped', 'blacklisted', 'deleted')),
    status_changed_on   DATE,
    dormant_since       DATE,                                -- derived from the last posting, never from the status alone
    parent_account_id   UUID,                                -- null until the bank confirms a parent link
    staff_account       BOOLEAN NOT NULL DEFAULT FALSE,
    correspondent       BOOLEAN NOT NULL DEFAULT FALSE,
    islamic_book        BOOLEAN NOT NULL DEFAULT FALSE,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    valid_from          DATE NOT NULL,
    valid_to            DATE,
    recorded_from       TIMESTAMPTZ NOT NULL,
    recorded_to         TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (id, recorded_from),
    CHECK (closed_on IS NULL OR status = 'closed'),
    CHECK (valid_to IS NULL OR valid_to > valid_from),
    CHECK (recorded_to IS NULL OR recorded_to > recorded_from)
);

CREATE TABLE IF NOT EXISTS canon.account_party_role (
    id                  UUID NOT NULL,
    account_id          UUID NOT NULL,
    party_id            UUID NOT NULL,
    role                TEXT NOT NULL CHECK (role IN ('owner', 'joint', 'signatory')),
    signing_rule        TEXT,                                -- null unless the signing fields are mapped
    derived             BOOLEAN NOT NULL DEFAULT FALSE,      -- copied from a per-customer signer list (assumption)
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    valid_from          DATE NOT NULL,
    valid_to            DATE,
    recorded_from       TIMESTAMPTZ NOT NULL,
    recorded_to         TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (id, recorded_from),
    CHECK (valid_to IS NULL OR valid_to > valid_from),
    CHECK (recorded_to IS NULL OR recorded_to > recorded_from)
);

CREATE TABLE IF NOT EXISTS canon.account_hold (
    id                  UUID PRIMARY KEY,
    account_id          UUID NOT NULL,                       -- no foreign key: account is bi-temporal (section 2)
    type                TEXT NOT NULL CHECK (type IN ('block', 'restriction', 'stop_payment', 'court', 'aml_freeze',
                                                      'sanctions_freeze', 'cbi_order', 'branch_hold', 'other')),
    authority           TEXT,                                -- court, CBI and AML types only when the reason map gives them
    reference           TEXT NOT NULL,
    start_at            TIMESTAMPTZ NOT NULL,
    end_at              TIMESTAMPTZ,
    amount              NUMERIC(28,6) CHECK (amount IS NULL OR amount >= 0),
    currency            CHAR(3) REFERENCES ref.currencies(code) ON DELETE RESTRICT,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (end_at IS NULL OR end_at > start_at),
    CHECK ((amount IS NULL) = (currency IS NULL)),
    CONSTRAINT uq_account_hold_source UNIQUE (source_id, source_table, source_pk)
);

-- Lineage and access-path indexes (DATA-MODEL s.0: foreign keys indexed; lineage looked up by source key).
DO $$
DECLARE t TEXT;
BEGIN
    FOREACH t IN ARRAY ARRAY['branch', 'product', 'channel', 'staff', 'party', 'party_name', 'party_identifier',
                             'party_contact', 'party_relationship', 'customer', 'account', 'account_party_role',
                             'account_hold'] LOOP
        EXECUTE format('CREATE INDEX IF NOT EXISTS %I ON canon.%I (source_id, source_table, source_pk)',
                       'idx_' || t || '_src', t);
        EXECUTE format('CREATE INDEX IF NOT EXISTS %I ON canon.%I (mapping_version_id)', 'idx_' || t || '_mv', t);
    END LOOP;
END $$;

-- Current versions: one per source key. The writer closes the previous version before it inserts the next one.
CREATE UNIQUE INDEX IF NOT EXISTS uq_party_current     ON canon.party (source_id, source_table, source_pk)
    WHERE recorded_to IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uq_customer_current  ON canon.customer (source_id, source_table, source_pk)
    WHERE recorded_to IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uq_account_current   ON canon.account (source_id, source_table, source_pk)
    WHERE recorded_to IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uq_party_rel_current ON canon.party_relationship (source_id, source_table, source_pk)
    WHERE recorded_to IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uq_acc_party_current ON canon.account_party_role (source_id, source_table, source_pk)
    WHERE recorded_to IS NULL;
CREATE INDEX IF NOT EXISTS idx_party_id                ON canon.party (id);
CREATE INDEX IF NOT EXISTS idx_customer_party          ON canon.customer (party_id, recorded_from DESC);
CREATE INDEX IF NOT EXISTS idx_customer_cif            ON canon.customer (cif);
CREATE INDEX IF NOT EXISTS idx_customer_branch         ON canon.customer (branch_id);
CREATE INDEX IF NOT EXISTS idx_customer_channel        ON canon.customer (channel_id) WHERE channel_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_customer_rm             ON canon.customer (rm_id) WHERE rm_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_account_customer        ON canon.account (customer_id, recorded_from DESC);
CREATE INDEX IF NOT EXISTS idx_account_number_hk       ON canon.account (number_hk);
CREATE INDEX IF NOT EXISTS idx_account_iban_hk         ON canon.account (iban_hk) WHERE iban_hk IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_account_branch          ON canon.account (branch_id);
CREATE INDEX IF NOT EXISTS idx_account_product         ON canon.account (product_id);
CREATE INDEX IF NOT EXISTS idx_acc_party_party         ON canon.account_party_role (party_id);
CREATE INDEX IF NOT EXISTS idx_acc_party_account       ON canon.account_party_role (account_id);
CREATE INDEX IF NOT EXISTS idx_party_rel_from          ON canon.party_relationship (from_party);
CREATE INDEX IF NOT EXISTS idx_party_rel_to            ON canon.party_relationship (to_party);
CREATE INDEX IF NOT EXISTS idx_party_name_party        ON canon.party_name (party_id);
CREATE INDEX IF NOT EXISTS idx_party_name_key          ON canon.party_name (key);
CREATE INDEX IF NOT EXISTS idx_party_identifier_party  ON canon.party_identifier (party_id);
CREATE INDEX IF NOT EXISTS idx_party_identifier_hk     ON canon.party_identifier (value_hk) WHERE value_hk IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_party_contact_party     ON canon.party_contact (party_id);
CREATE INDEX IF NOT EXISTS idx_party_contact_hk        ON canon.party_contact (value_hk) WHERE value_hk IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_account_hold_account    ON canon.account_hold (account_id, start_at DESC);
CREATE INDEX IF NOT EXISTS idx_staff_branch            ON canon.staff (branch_id);
CREATE INDEX IF NOT EXISTS idx_staff_party             ON canon.staff (party_id) WHERE party_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_branch_org              ON canon.branch (org_branch_id) WHERE org_branch_id IS NOT NULL;

-- Grants. Writes come from akr_ingest only; the bi-temporal tables may close a version (recorded_to, valid_to).
GRANT SELECT, INSERT ON canon.branch, canon.product, canon.channel, canon.staff, canon.party, canon.party_name,
    canon.party_identifier, canon.party_contact, canon.party_relationship, canon.customer, canon.account,
    canon.account_party_role, canon.account_hold TO akr_ingest;
GRANT UPDATE (recorded_to, valid_to) ON canon.party, canon.party_relationship, canon.customer, canon.account,
    canon.account_party_role TO akr_ingest;
GRANT UPDATE (loaded_last_at) ON canon.party_contact TO akr_ingest;
-- Party children are replaced by hash, never updated in place or deleted: the old row is marked superseded and the new
-- one inserted (section 3.7 step 6). One current row per source key and field.
GRANT UPDATE (superseded_at) ON canon.party_name, canon.party_identifier, canon.party_contact TO akr_ingest;
GRANT UPDATE (capture_source) ON canon.party_name TO akr_ingest;   -- the label of a name, section 7.26 item 5
CREATE UNIQUE INDEX IF NOT EXISTS uq_party_name_current       ON canon.party_name (source_id, source_table, source_pk, source_field, name_type)
    WHERE superseded_at IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uq_party_identifier_current ON canon.party_identifier (source_id, source_table, source_pk, type)
    WHERE superseded_at IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uq_party_contact_current    ON canon.party_contact (source_id, source_table, source_pk, type)
    WHERE superseded_at IS NULL;
GRANT SELECT ON canon.branch, canon.product, canon.channel, canon.staff, canon.party, canon.party_name,
    canon.party_identifier, canon.party_contact, canon.party_relationship, canon.customer, canon.account,
    canon.account_party_role, canon.account_hold TO akr_app;
GRANT SELECT ON canon.party, canon.party_name, canon.party_identifier, canon.customer, canon.account,
    canon.account_party_role, canon.account_hold, canon.branch, canon.product, canon.channel TO akr_readonly;

-- Classification. `personal` covers the encrypted and keyed-hash columns (section 1 conventions).
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'canon', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('branch', 'internal', ARRAY['id','code','name','governorate','region','kurdistan','type','opened_on','org_branch_id']),
  ('product', 'internal', ARRAY['id','code','family','islamic_contract_type']),
  ('product', 'detection-logic', ARRAY['risk_attrs']),
  ('channel', 'internal', ARRAY['id','code','kind']),
  ('channel', 'detection-logic', ARRAY['risk_attr']),
  ('staff', 'internal', ARRAY['id','employee_id','branch_id','role']),
  ('staff', 'personal', ARRAY['party_id','authority_limits','leave']),
  ('party', 'internal', ARRAY['id','type','status','nationality','residence_country','birth_precision','gender',
                              'occupation_isic','legal_form','staff_flag','sensitivity_tier','valid_from','valid_to']),
  ('party', 'personal', ARRAY['birth_date','birth_place','established_on','registration_no_enc','registration_no_hk',
                              'tax_no_enc','tax_no_hk','mother_name_enc','family_no_enc','family_no_hk']),
  ('party', 'detection-logic', ARRAY['pep_status','related_to_staff']),
  ('party_name', 'personal', ARRAY['raw','light','folded','key','parts']),
  ('party_name', 'internal', ARRAY['id','party_id','script','language','name_type','source_field','capture_source','alias_class',
                                   'quality_flags']),
  ('party_identifier', 'personal', ARRAY['value_enc','value_hk','issued_on','expires_on','issuer']),
  ('party_identifier', 'internal', ARRAY['id','party_id','type','placeholder_flag','country','expiry_flag']),
  ('party_contact', 'personal', ARRAY['value_enc','value_hk','address']),
  ('party_contact', 'internal', ARRAY['id','party_id','type','loaded_first_at','loaded_last_at']),
  ('party_relationship', 'personal', ARRAY['from_party','to_party','share_pct','evidence']),
  ('party_relationship', 'internal', ARRAY['id','type','degree','valid_from','valid_to']),
  ('customer', 'internal', ARRAY['id','party_id','cif','branch_id','rm_id','onboarded_on','channel_id','status',
                                 'valid_from','valid_to']),
  ('customer', 'detection-logic', ARRAY['segment','cdd_level']),
  ('customer', 'personal', ARRAY['expected_profile']),
  ('account', 'personal', ARRAY['number_enc','number_hk','iban_hk']),
  ('account', 'internal', ARRAY['id','customer_id','type','product_id','currency','branch_id','opened_on','closed_on',
                                'status','status_changed_on','dormant_since','parent_account_id','staff_account',
                                'correspondent','islamic_book','valid_from','valid_to']),
  ('account_party_role', 'internal', ARRAY['id','account_id','party_id','role','signing_rule','derived','valid_from',
                                           'valid_to']),
  ('account_hold', 'internal', ARRAY['id','account_id','type','authority','reference','start_at','end_at','currency']),
  ('account_hold', 'financial', ARRAY['amount'])
) AS v(t, cls, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

-- Retention (DATA-MODEL s.5 "Class" column; clocks per DATA-MODEL s.1 retention vocabulary).
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('canon', 'branch',              'R9', 'retirement',       NULL, 'Reference; outlives the accounts it names'),
 ('canon', 'product',             'R9', 'retirement',       NULL, 'Reference'),
 ('canon', 'channel',             'R9', 'retirement',       NULL, 'Reference'),
 ('canon', 'staff',               'R3', 'owner', NULL, 'Employment record copy'),
 ('canon', 'party',               'R3', 'owner', NULL, 'Seven years after the relationship ends'),
 ('canon', 'party_name',          'R3', 'owner', NULL, 'As its party'),
 ('canon', 'party_identifier',    'R3', 'owner', NULL, 'As its party'),
 ('canon', 'party_contact',       'R3', 'owner', NULL, 'As its party'),
 ('canon', 'party_relationship',  'R3', 'owner', NULL, 'As its parties'),
 ('canon', 'customer',            'R3', 'owner', NULL, 'Seven years after the relationship ends'),
 ('canon', 'account',             'R3', 'relationship_end', 'closed_on', 'Seven years after closure'),
 ('canon', 'account_party_role',  'R3', 'owner', NULL, 'As its account'),
 ('canon', 'account_hold',        'R3', 'owner', NULL, 'As its account')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

### 1.3 Migration `0031_canon_transactions.sql`

```sql
-- migration 0031_canon_transactions.sql
-- Phase 10. The partition factory for the canonical facts and the transaction, balance and payment-message tables
-- (DATA-MODEL s.5, s.0 partitioning: monthly, BRIN on time columns, B-tree on access paths). Facts are immutable once
-- settled; a correction is a new row that points at the original (reverses_id, corrected_by_id). Business-date
-- freshness uses business_date (DR-39), never the AKR clock.

-- The partition factory (SECURITY DEFINER, as in phase 02 audit.create_partition). It creates one month of one
-- allow-listed canonical table. Bounds are dates for DATE keys and UTC midnights for TIMESTAMPTZ keys. An advisory
-- lock serialises two writers that meet a missing month.
CREATE OR REPLACE FUNCTION canon.create_partition(p_table TEXT, p_year INT, p_month INT) RETURNS TEXT
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog
AS $$
DECLARE
    v_from    DATE;
    v_to      DATE;
    v_name    TEXT;
    v_lo      TEXT;
    v_hi      TEXT;
BEGIN
    IF p_table NOT IN ('balance_snapshot', 'txn', 'txn_event', 'card_txn', 'payment_message', 'payment_message_party',
                       'cash_event', 'fx_deal', 'locker_access', 'cbs_user_activity') THEN
        RAISE EXCEPTION 'canon.create_partition: % is not a partitioned canonical table', p_table;
    END IF;
    IF p_year NOT BETWEEN 2000 AND 2200 OR p_month NOT BETWEEN 1 AND 12 THEN
        RAISE EXCEPTION 'canon.create_partition: month %-% out of range', p_year, p_month;
    END IF;
    PERFORM pg_catalog.pg_advisory_xact_lock(7307418562110395010);   -- fixed key: 'akr.canon.partition_factory'
    v_from := pg_catalog.make_date(p_year, p_month, 1);
    v_to   := (v_from + interval '1 month')::date;
    v_name := pg_catalog.format('%s_%s_%s', p_table, p_year, pg_catalog.lpad(p_month::text, 2, '0'));
    IF p_table IN ('card_txn', 'locker_access', 'cbs_user_activity', 'payment_message', 'payment_message_party') THEN
        v_lo := v_from::text || ' 00:00:00+00';
        v_hi := v_to::text   || ' 00:00:00+00';
    ELSE
        v_lo := v_from::text;
        v_hi := v_to::text;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_catalog.pg_class c
                   JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace
                   WHERE n.nspname = 'canon' AND c.relname = v_name) THEN
        EXECUTE pg_catalog.format('CREATE TABLE canon.%I PARTITION OF canon.%I FOR VALUES FROM (%L) TO (%L)',
                                  v_name, p_table, v_lo, v_hi);
    END IF;
    RETURN v_name;
END $$;

-- Balances: one signed snapshot per account, business date and source (DATA-MODEL s.5: signed, exempt from the
-- positive-amount rule). rebuilt_from_postings rows carry a flag; they are never stamped with today's balance.
CREATE TABLE IF NOT EXISTS canon.balance_snapshot (
    id                  UUID NOT NULL,
    account_id          UUID NOT NULL,                       -- no foreign key: account is bi-temporal (section 2)
    business_date       DATE NOT NULL,
    ledger_balance      NUMERIC(28,6) NOT NULL,              -- signed
    cleared_balance     NUMERIC(28,6),
    balance_lcy         NUMERIC(28,6),
    available_balance   NUMERIC(28,6),                       -- null until a source column is confirmed
    currency            CHAR(3) NOT NULL REFERENCES ref.currencies(code) ON DELETE RESTRICT,
    source              TEXT NOT NULL CHECK (source IN ('cbs_current', 'rebuilt_from_postings')),
    flags               TEXT[] NOT NULL DEFAULT '{}',
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (id, business_date),
    UNIQUE (account_id, business_date, source)
) PARTITION BY RANGE (business_date);

-- Counterparties (R2). The account is held only as a keyed hash; names are personal.
CREATE TABLE IF NOT EXISTS canon.counterparty (
    id                  UUID PRIMARY KEY,
    names               JSONB NOT NULL CHECK (jsonb_typeof(names) = 'object'),
    account_hk          BYTEA,
    bank_bic            TEXT CHECK (bank_bic IS NULL OR bank_bic ~ '^[A-Z]{6}[A-Z0-9]{2}([A-Z0-9]{3})?$'),
    bank_country        CHAR(2) REFERENCES ref.countries(iso2) ON DELETE RESTRICT,
    type                TEXT NOT NULL,
    party_id            UUID,                                -- no foreign key: party is bi-temporal (section 2)
    first_seen          DATE NOT NULL,
    last_seen           DATE NOT NULL,
    quality_flags       TEXT[] NOT NULL DEFAULT '{}',        -- missing_name, missing_account, missing_bic (DATA-MODEL s.5)
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (last_seen >= first_seen)
);

-- The monitoring copy of postings (R2, DATA-MODEL s.5). account_id is null for a general-ledger leg; ledger_code is
-- always set. `amount` is the unsigned source amount in the account currency; the sign is dr_cr. The source's own
-- local timestamp is kept as text (DR-87); posted_at stays null without a dated source timestamp (DR-37).
CREATE TABLE IF NOT EXISTS canon.txn (
    id                      UUID NOT NULL,
    account_id              UUID,                            -- no foreign key: account is bi-temporal (section 2)
    ledger_code             TEXT NOT NULL,
    document_key            TEXT,                            -- provisional until the vendor confirms the keying
    dr_cr                   CHAR(1) NOT NULL CHECK (dr_cr IN ('D', 'C')),
    amount                  NUMERIC(28,6) NOT NULL CHECK (amount >= 0),
    currency                CHAR(3) NOT NULL REFERENCES ref.currencies(code) ON DELETE RESTRICT,
    amount_lcy              NUMERIC(28,6) CHECK (amount_lcy IS NULL OR amount_lcy >= 0),
    amount_usd              NUMERIC(28,6) CHECK (amount_usd IS NULL OR amount_usd >= 0),
    rate_source             TEXT,
    rate_date               DATE,
    rate_flag               BOOLEAN NOT NULL DEFAULT FALSE,  -- amount x rate differs from amount_lcy by more than 1 percent
    booking_date            DATE NOT NULL,
    value_date              DATE NOT NULL,
    business_date           DATE NOT NULL,
    posted_at               TIMESTAMPTZ,
    source_local_ts         TEXT NOT NULL,                   -- the source literal, ISO 8601 without zone
    source_ts_precision     TEXT NOT NULL CHECK (source_ts_precision IN ('date', 'minute', 'second')),
    txn_type                TEXT NOT NULL,                   -- canonical code from the txn_type code map
    status                  TEXT NOT NULL CHECK (status IN ('posted', 'cancelled')),
    channel_id              UUID REFERENCES canon.channel(id) ON DELETE RESTRICT,
    branch_id               UUID NOT NULL REFERENCES canon.branch(id) ON DELETE RESTRICT,
    product_id              UUID REFERENCES canon.product(id) ON DELETE RESTRICT,
    counterparty_id         UUID REFERENCES canon.counterparty(id) ON DELETE RESTRICT,
    operator_id             UUID NOT NULL REFERENCES canon.staff(id) ON DELETE RESTRICT,   -- a batch operator is a 'system' staff row
    authoriser_id           UUID REFERENCES canon.staff(id) ON DELETE RESTRICT,            -- set only from an accepting approver (MAN_REP = 2)
    narrative_raw           TEXT NOT NULL,
    narrative_norm          TEXT NOT NULL,
    reverses_id             UUID,                            -- null unless the source pairs a reversal (no foreign key: partitioned)
    corrected_by_id         UUID,
    cancel_reason_code      TEXT,
    cash_flag               BOOLEAN NOT NULL,
    system_generated        BOOLEAN,                         -- null until the bank confirms the batch-operator list (DR-42)
    late                    BOOLEAN NOT NULL DEFAULT FALSE,
    source_id               UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table            TEXT NOT NULL,
    source_pk               TEXT NOT NULL,
    source_version          TEXT,
    row_hash                BYTEA NOT NULL,
    batch_id                UUID NOT NULL,
    extracted_at            TIMESTAMPTZ NOT NULL,
    mapping_version_id      UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (id, booking_date),
    CONSTRAINT uq_txn_source UNIQUE (source_id, source_table, source_pk, booking_date),
    CHECK (status <> 'cancelled' OR cancel_reason_code IS NOT NULL)
) PARTITION BY RANGE (booking_date);

-- Events of a transaction (DATA-MODEL s.5). `initiated` and `rejected` only from a source that logs them.
CREATE TABLE IF NOT EXISTS canon.txn_event (
    id                  UUID NOT NULL,
    txn_id              UUID NOT NULL,
    booking_date        DATE NOT NULL,
    kind                TEXT NOT NULL CHECK (kind IN ('initiated', 'pending', 'authorised', 'posted', 'reversed',
                                                      'rejected', 'cancelled')),
    at                  TIMESTAMPTZ NOT NULL,
    actor_id            UUID REFERENCES canon.staff(id) ON DELETE RESTRICT,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (id, booking_date),
    CONSTRAINT uq_txn_event_kind UNIQUE (txn_id, booking_date, kind),
    CONSTRAINT fk_txn_event_txn FOREIGN KEY (txn_id, booking_date) REFERENCES canon.txn (id, booking_date)
        ON DELETE RESTRICT
) PARTITION BY RANGE (booking_date);

-- Payment messages (R2, partitioned by received_at). The raw message is kept where the source holds it, encrypted.
-- Status changes after creation are appended to status_history, never overwritten (DATA-MODEL s.5).
CREATE TABLE IF NOT EXISTS canon.payment_message (
    id                      UUID NOT NULL,
    format                  TEXT NOT NULL CHECK (format IN ('iso20022', 'mt')),
    rail                    TEXT NOT NULL,                   -- domestic and central-bank transfers differ by rail
    direction               TEXT NOT NULL CHECK (direction IN ('in', 'out')),
    raw_enc                 BYTEA,
    uetr                    UUID,                            -- null when the source has none; never the only match key
    message_refs            JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(message_refs) = 'object'),
    instructed_amount       NUMERIC(28,6) CHECK (instructed_amount IS NULL OR instructed_amount >= 0),
    instructed_currency     CHAR(3) REFERENCES ref.currencies(code) ON DELETE RESTRICT,
    settled_amount          NUMERIC(28,6) CHECK (settled_amount IS NULL OR settled_amount >= 0),
    settled_currency        CHAR(3) REFERENCES ref.currencies(code) ON DELETE RESTRICT,
    charge_bearer           TEXT,
    purpose                 TEXT,
    status                  TEXT NOT NULL,
    status_history          JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(status_history) = 'array'),
    received_at             TIMESTAMPTZ NOT NULL,
    source_id               UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table            TEXT NOT NULL,
    source_pk               TEXT NOT NULL,
    source_version          TEXT,
    row_hash                BYTEA NOT NULL,
    batch_id                UUID NOT NULL,
    extracted_at            TIMESTAMPTZ NOT NULL,
    mapping_version_id      UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (id, received_at),
    CONSTRAINT uq_payment_message_source UNIQUE (source_id, source_table, source_pk, received_at)
) PARTITION BY RANGE (received_at);

CREATE TABLE IF NOT EXISTS canon.payment_message_party (
    id                      UUID NOT NULL,
    message_id              UUID NOT NULL,
    received_at             TIMESTAMPTZ NOT NULL,
    role                    TEXT NOT NULL CHECK (role IN ('debtor', 'creditor', 'ultimate_debtor', 'ultimate_creditor',
                                                          'agent', 'other')),
    name                    TEXT,                            -- null with a flag, never an empty string
    address                 JSONB CHECK (address IS NULL OR jsonb_typeof(address) = 'object'),
    identifier              TEXT,
    account_hk              BYTEA,
    quality_flags           TEXT[] NOT NULL DEFAULT '{}',    -- missing_name, missing_account, missing_bic
    source_id               UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table            TEXT NOT NULL,
    source_pk               TEXT NOT NULL,
    source_version          TEXT,
    row_hash                BYTEA NOT NULL,
    batch_id                UUID NOT NULL,
    extracted_at            TIMESTAMPTZ NOT NULL,
    mapping_version_id      UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (id, received_at),
    CONSTRAINT uq_payment_message_party_source UNIQUE (source_id, source_table, source_pk, received_at),
    CONSTRAINT fk_pmp_message FOREIGN KEY (message_id, received_at)
        REFERENCES canon.payment_message (id, received_at) ON DELETE RESTRICT
) PARTITION BY RANGE (received_at);

-- Links between a message and a posting (R2). Derived by the rule named in `rule`; the lineage is the message's.
CREATE TABLE IF NOT EXISTS canon.payment_link (
    id                      UUID PRIMARY KEY,
    message_id              UUID NOT NULL,
    message_received_at     TIMESTAMPTZ NOT NULL,
    txn_id                  UUID NOT NULL,
    booking_date            DATE NOT NULL,
    match_keys              JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(match_keys) = 'object'),
    confidence              NUMERIC(5,4) NOT NULL CHECK (confidence BETWEEN 0 AND 1),
    rule                    TEXT NOT NULL,
    source_id               UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table            TEXT NOT NULL,
    source_pk               TEXT NOT NULL,
    source_version          TEXT,
    row_hash                BYTEA NOT NULL,
    batch_id                UUID NOT NULL,
    extracted_at            TIMESTAMPTZ NOT NULL,
    mapping_version_id      UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_payment_link_pair UNIQUE (message_id, txn_id),
    CONSTRAINT fk_payment_link_message FOREIGN KEY (message_id, message_received_at)
        REFERENCES canon.payment_message (id, received_at) ON DELETE RESTRICT,
    CONSTRAINT fk_payment_link_txn FOREIGN KEY (txn_id, booking_date)
        REFERENCES canon.txn (id, booking_date) ON DELETE RESTRICT
);

-- Lineage indexes for the partitioned and plain tables of this migration.
DO $$
DECLARE t TEXT;
BEGIN
    FOREACH t IN ARRAY ARRAY['balance_snapshot', 'txn', 'txn_event', 'counterparty', 'payment_message',
                             'payment_message_party', 'payment_link'] LOOP
        EXECUTE format('CREATE INDEX IF NOT EXISTS %I ON canon.%I (source_id, source_table, source_pk)',
                       'idx_' || t || '_src', t);
        EXECUTE format('CREATE INDEX IF NOT EXISTS %I ON canon.%I (mapping_version_id)', 'idx_' || t || '_mv', t);
    END LOOP;
END $$;

-- Access paths (B-tree) and time indexes (BRIN on append-ordered time columns).
CREATE INDEX IF NOT EXISTS idx_txn_account          ON canon.txn (account_id, booking_date DESC) WHERE account_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_txn_branch           ON canon.txn (branch_id, booking_date);
CREATE INDEX IF NOT EXISTS idx_txn_counterparty     ON canon.txn (counterparty_id) WHERE counterparty_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_txn_channel          ON canon.txn (channel_id) WHERE channel_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_txn_product          ON canon.txn (product_id) WHERE product_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_txn_operator         ON canon.txn (operator_id, booking_date);
CREATE INDEX IF NOT EXISTS idx_txn_authoriser       ON canon.txn (authoriser_id) WHERE authoriser_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_txn_reverses         ON canon.txn (reverses_id) WHERE reverses_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_txn_cash             ON canon.txn (booking_date) WHERE cash_flag;
CREATE INDEX IF NOT EXISTS idx_txn_business_brin    ON canon.txn USING BRIN (business_date);
CREATE INDEX IF NOT EXISTS idx_txn_event_txn        ON canon.txn_event (txn_id, booking_date);
CREATE INDEX IF NOT EXISTS idx_txn_event_at_brin    ON canon.txn_event USING BRIN (at);
CREATE INDEX IF NOT EXISTS idx_txn_event_actor      ON canon.txn_event (actor_id) WHERE actor_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_balance_account      ON canon.balance_snapshot (account_id, business_date DESC);
CREATE INDEX IF NOT EXISTS idx_counterparty_hk      ON canon.counterparty (account_hk) WHERE account_hk IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_counterparty_party   ON canon.counterparty (party_id) WHERE party_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_counterparty_country ON canon.counterparty (bank_country) WHERE bank_country IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_pm_uetr              ON canon.payment_message (uetr) WHERE uetr IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_pm_received_brin     ON canon.payment_message USING BRIN (received_at);
CREATE INDEX IF NOT EXISTS idx_pmp_message          ON canon.payment_message_party (message_id, received_at);
CREATE INDEX IF NOT EXISTS idx_pmp_account_hk       ON canon.payment_message_party (account_hk) WHERE account_hk IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_payment_link_txn     ON canon.payment_link (txn_id, booking_date);
CREATE INDEX IF NOT EXISTS idx_payment_link_message ON canon.payment_link (message_id);

-- Grants. Writes by akr_ingest; the factory is callable by the two roles that write canon (section 3.6, DR-45 pattern).
GRANT SELECT, INSERT ON canon.balance_snapshot, canon.txn, canon.txn_event, canon.counterparty,
    canon.payment_message, canon.payment_message_party, canon.payment_link TO akr_ingest;
GRANT UPDATE (status, status_history) ON canon.payment_message TO akr_ingest;   -- append-only in practice (section 7+, status change of a payment message)
GRANT SELECT ON canon.balance_snapshot, canon.txn, canon.txn_event, canon.counterparty, canon.payment_message,
    canon.payment_message_party, canon.payment_link TO akr_app;
GRANT SELECT ON canon.balance_snapshot, canon.txn, canon.txn_event, canon.counterparty, canon.payment_message,
    canon.payment_link TO akr_readonly;
REVOKE ALL ON FUNCTION canon.create_partition(TEXT, INT, INT) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION canon.create_partition(TEXT, INT, INT) TO akr_ingest, akr_app;

-- Classification. The narrative, names and raw messages are personal; amounts are financial; codes are internal.
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'canon', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('balance_snapshot', 'financial', ARRAY['ledger_balance','cleared_balance','balance_lcy','available_balance']),
  ('balance_snapshot', 'internal', ARRAY['id','account_id','business_date','currency','source','flags']),
  ('txn', 'financial', ARRAY['amount','amount_lcy','amount_usd','rate_source','rate_date','currency','dr_cr','rate_flag']),
  ('txn', 'personal', ARRAY['narrative_raw','narrative_norm','account_id','counterparty_id','document_key']),
  ('txn', 'internal', ARRAY['id','ledger_code','booking_date','value_date','business_date','posted_at','source_local_ts',
                            'source_ts_precision','txn_type','status','channel_id','branch_id','product_id','operator_id',
                            'authoriser_id','reverses_id','corrected_by_id','cancel_reason_code','cash_flag',
                            'system_generated','late']),
  ('txn_event', 'internal', ARRAY['id','txn_id','booking_date','kind','at','actor_id']),
  ('counterparty', 'personal', ARRAY['names','account_hk','party_id','quality_flags']),
  ('counterparty', 'internal', ARRAY['id','bank_bic','bank_country','type','first_seen','last_seen']),
  ('payment_message', 'personal', ARRAY['raw_enc','message_refs','uetr']),
  ('payment_message', 'financial', ARRAY['instructed_amount','instructed_currency','settled_amount','settled_currency']),
  ('payment_message', 'internal', ARRAY['id','format','rail','direction','charge_bearer','purpose','status',
                                        'status_history','received_at']),
  ('payment_message_party', 'personal', ARRAY['name','address','identifier','account_hk']),
  ('payment_message_party', 'internal', ARRAY['id','message_id','received_at','role','quality_flags']),
  ('payment_link', 'internal', ARRAY['id','message_id','message_received_at','txn_id','booking_date','match_keys',
                                     'confidence','rule'])
) AS v(t, cls, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

-- Retention (DATA-MODEL s.5): facts R2 by booking month; reference-like counterparties by last sighting.
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('canon', 'balance_snapshot',       'R2', 'partition', 'business_date', 'Monthly partitions; 7 or 10 years by booking month'),
 ('canon', 'txn',                    'R2', 'partition', 'booking_date',  'Monitoring copy; 7 or 10 years by booking month'),
 ('canon', 'txn_event',              'R2', 'partition', 'booking_date',  'As its transaction'),
 ('canon', 'counterparty',           'R2', 'age',       'last_seen',     'Kept while any transaction names it'),
 ('canon', 'payment_message',        'R2', 'partition', 'received_at',   'Raw message kept where the source holds it'),
 ('canon', 'payment_message_party',  'R2', 'partition', 'received_at',   'As its message'),
 ('canon', 'payment_link',           'R2', 'age',       'created_at',    'As its message and transaction')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

### 1.4 Migration `0032_canon_cards_activity.sql`

```sql
-- migration 0032_canon_cards_activity.sql
-- Phase 10. Cards, card transactions, cash events, FX deals, facilities, trade instruments, locker access, KYC
-- documents and CBS user activity (DATA-MODEL s.5, C-36, SP-48, SP-51). A card row holds a masked PAN and a keyed hash
-- only: the pan_masked CHECK admits no unmasked digit run, and the card guard (section 3.3) stops a PAN before landing.

-- The card. Holder and account are referenced without a foreign key to the bi-temporal account and party (section 2).
CREATE TABLE IF NOT EXISTS canon.card (
    id                  UUID PRIMARY KEY,
    account_id          UUID NOT NULL,
    pan_masked          TEXT NOT NULL CHECK (pan_masked ~ '^[0-9]{6}[*]{3,9}[0-9]{4}$'),   -- first six, last four
    pan_hk              BYTEA NOT NULL,
    scheme              TEXT,
    product             TEXT,
    status              TEXT NOT NULL CHECK (status IN ('active', 'blocked', 'expired', 'closed', 'unknown')),
    holder_party_id     UUID,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_card_source UNIQUE (source_id, source_table, source_pk)
);

-- Card transactions (R2, partitioned by at). txn_id is set when the card posting is linked to a ledger leg.
CREATE TABLE IF NOT EXISTS canon.card_txn (
    id                  UUID NOT NULL,
    card_id             UUID NOT NULL REFERENCES canon.card(id) ON DELETE RESTRICT,
    txn_id              UUID,                                -- no foreign key: txn is partitioned by booking_date
    mcc                 CHAR(4),
    merchant_id         TEXT,
    terminal_id         TEXT,
    acquirer            TEXT,
    country             CHAR(2) REFERENCES ref.countries(iso2) ON DELETE RESTRICT,
    entry_mode          TEXT,
    ecommerce           BOOLEAN NOT NULL DEFAULT FALSE,
    response_code       TEXT,
    dr_cr               CHAR(1) NOT NULL CHECK (dr_cr IN ('D', 'C')),
    amount              NUMERIC(28,6) NOT NULL CHECK (amount >= 0),
    currency            CHAR(3) NOT NULL REFERENCES ref.currencies(code) ON DELETE RESTRICT,
    amount_lcy          NUMERIC(28,6) CHECK (amount_lcy IS NULL OR amount_lcy >= 0),
    amount_usd          NUMERIC(28,6) CHECK (amount_usd IS NULL OR amount_usd >= 0),
    rate_source         TEXT,
    rate_date           DATE,
    at                  TIMESTAMPTZ NOT NULL,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (id, at),
    CONSTRAINT uq_card_txn_source UNIQUE (source_id, source_table, source_pk, at)
) PARTITION BY RANGE (at);

-- Cash events (R2, partitioned by booking_date): one per cash leg, with the denomination count the teller recorded.
CREATE TABLE IF NOT EXISTS canon.cash_event (
    id                  UUID NOT NULL,
    txn_id              UUID NOT NULL,
    booking_date        DATE NOT NULL,
    denominations       JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(denominations) = 'array'),
    declared_source     TEXT,
    ctr_flag            BOOLEAN NOT NULL DEFAULT FALSE,      -- threshold is bank data (D20), not a constant here
    branch_id           UUID NOT NULL REFERENCES canon.branch(id) ON DELETE RESTRICT,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (id, booking_date),
    CONSTRAINT uq_cash_event_txn UNIQUE (txn_id, booking_date),
    CONSTRAINT fk_cash_event_txn FOREIGN KEY (txn_id, booking_date) REFERENCES canon.txn (id, booking_date)
        ON DELETE RESTRICT
) PARTITION BY RANGE (booking_date);

-- FX deals (R2, partitioned by value_date).
CREATE TABLE IF NOT EXISTS canon.fx_deal (
    id                  UUID NOT NULL,
    pair                CHAR(7) NOT NULL CHECK (pair ~ '^[A-Z]{3}/[A-Z]{3}$'),
    rates               JSONB NOT NULL CHECK (jsonb_typeof(rates) = 'object'),
    dr_cr               CHAR(1) CHECK (dr_cr IS NULL OR dr_cr IN ('D', 'C')),
    amount              NUMERIC(28,6) NOT NULL CHECK (amount >= 0),
    currency            CHAR(3) NOT NULL REFERENCES ref.currencies(code) ON DELETE RESTRICT,
    amount_lcy          NUMERIC(28,6) CHECK (amount_lcy IS NULL OR amount_lcy >= 0),
    amount_usd          NUMERIC(28,6) CHECK (amount_usd IS NULL OR amount_usd >= 0),
    rate_source         TEXT,
    rate_date           DATE,
    value_date          DATE NOT NULL,
    counterparty_id     UUID REFERENCES canon.counterparty(id) ON DELETE RESTRICT,
    exchange_house      TEXT,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (id, value_date),
    CONSTRAINT uq_fx_deal_source UNIQUE (source_id, source_table, source_pk, value_date)
) PARTITION BY RANGE (value_date);

-- Facilities (R3). outstanding stays null until the schedule tables are mapped: never zero (DATA-MODEL s.5, Islamic).
CREATE TABLE IF NOT EXISTS canon.facility (
    id                  UUID PRIMARY KEY,
    customer_id         UUID NOT NULL,                       -- no foreign key: customer is bi-temporal (section 2)
    product_id          UUID NOT NULL REFERENCES canon.product(id) ON DELETE RESTRICT,
    limit_amount        NUMERIC(28,6) CHECK (limit_amount IS NULL OR limit_amount >= 0),
    outstanding         NUMERIC(28,6) CHECK (outstanding IS NULL OR outstanding >= 0),
    currency            CHAR(3) REFERENCES ref.currencies(code) ON DELETE RESTRICT,
    purpose             TEXT,
    contract_type       TEXT,
    status              TEXT NOT NULL,
    events              JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(events) = 'array'),
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_facility_source UNIQUE (source_id, source_table, source_pk),
    CHECK ((outstanding IS NULL) OR currency IS NOT NULL)
);

-- Trade instruments (R3). Goods, HS codes, ports and the vessel are personal only where the party is a person.
CREATE TABLE IF NOT EXISTS canon.trade_instrument (
    id                  UUID PRIMARY KEY,
    customer_id         UUID NOT NULL,
    type                TEXT NOT NULL,
    parties             JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(parties) = 'array'),
    amount              NUMERIC(28,6) CHECK (amount IS NULL OR amount >= 0),
    currency            CHAR(3) REFERENCES ref.currencies(code) ON DELETE RESTRICT,
    amount_lcy          NUMERIC(28,6) CHECK (amount_lcy IS NULL OR amount_lcy >= 0),
    amount_usd          NUMERIC(28,6) CHECK (amount_usd IS NULL OR amount_usd >= 0),
    goods               TEXT,
    hs_codes            TEXT[] NOT NULL DEFAULT '{}',
    ports               JSONB CHECK (ports IS NULL OR jsonb_typeof(ports) = 'array'),
    vessel_imo          TEXT CHECK (vessel_imo IS NULL OR vessel_imo ~ '^[0-9]{7}$'),
    dates               JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(dates) = 'object'),
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_trade_source UNIQUE (source_id, source_table, source_pk)
);

-- Locker access (R2, partitioned by at). The locker is identified by the bank's reference.
CREATE TABLE IF NOT EXISTS canon.locker_access (
    id                  UUID NOT NULL,
    locker_ref          TEXT NOT NULL,
    party_id            UUID NOT NULL,                       -- no foreign key: party is bi-temporal (section 2)
    at                  TIMESTAMPTZ NOT NULL,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (id, at),
    CONSTRAINT uq_locker_access_source UNIQUE (source_id, source_table, source_pk, at)
) PARTITION BY RANGE (at);

-- KYC documents (R3). The image is never in the source views (C-36); evidence_object_id names a sealed copy when one exists.
CREATE TABLE IF NOT EXISTS canon.kyc_document (
    id                  UUID PRIMARY KEY,
    party_id            UUID NOT NULL,                       -- no foreign key: party is bi-temporal (section 2)
    type                TEXT NOT NULL,
    expires_on          DATE,
    evidence_object_id  UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_kyc_document_source UNIQUE (source_id, source_table, source_pk)
);

-- CBS user activity (R2, partitioned by at, optional per bank: DATA-MODEL s.5). Logins and overrides from the source's
-- own logs only; a CBS that keeps the last updater alone cannot show which operator changed a record, and this table
-- does not claim to. The source table name is the lineage column source_table.
CREATE TABLE IF NOT EXISTS canon.cbs_user_activity (
    id                  UUID NOT NULL,
    user_ref            TEXT NOT NULL,
    action              TEXT NOT NULL,
    object              TEXT,
    at                  TIMESTAMPTZ NOT NULL,
    override_flags      TEXT[] NOT NULL DEFAULT '{}',
    terminal_id         TEXT,
    ip_address_enc      BYTEA,
    session_id          TEXT,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (id, at),
    CONSTRAINT uq_cbs_user_activity_source UNIQUE (source_id, source_table, source_pk, at)
) PARTITION BY RANGE (at);

-- Lineage indexes, then access paths.
DO $$
DECLARE t TEXT;
BEGIN
    FOREACH t IN ARRAY ARRAY['card', 'card_txn', 'cash_event', 'fx_deal', 'facility', 'trade_instrument',
                             'locker_access', 'kyc_document', 'cbs_user_activity'] LOOP
        EXECUTE format('CREATE INDEX IF NOT EXISTS %I ON canon.%I (source_id, source_table, source_pk)',
                       'idx_' || t || '_src', t);
        EXECUTE format('CREATE INDEX IF NOT EXISTS %I ON canon.%I (mapping_version_id)', 'idx_' || t || '_mv', t);
    END LOOP;
END $$;

CREATE INDEX IF NOT EXISTS idx_card_account          ON canon.card (account_id);
CREATE INDEX IF NOT EXISTS idx_card_holder           ON canon.card (holder_party_id) WHERE holder_party_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_card_pan_hk           ON canon.card (pan_hk);
CREATE INDEX IF NOT EXISTS idx_card_txn_card         ON canon.card_txn (card_id, at DESC);
CREATE INDEX IF NOT EXISTS idx_card_txn_txn          ON canon.card_txn (txn_id) WHERE txn_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_card_txn_mcc          ON canon.card_txn (mcc, at);
CREATE INDEX IF NOT EXISTS idx_card_txn_at_brin      ON canon.card_txn USING BRIN (at);
CREATE INDEX IF NOT EXISTS idx_cash_event_branch     ON canon.cash_event (branch_id, booking_date);
CREATE INDEX IF NOT EXISTS idx_cash_event_ctr        ON canon.cash_event (booking_date) WHERE ctr_flag;
CREATE INDEX IF NOT EXISTS idx_fx_deal_cp            ON canon.fx_deal (counterparty_id) WHERE counterparty_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_facility_customer     ON canon.facility (customer_id);
CREATE INDEX IF NOT EXISTS idx_facility_product      ON canon.facility (product_id);
CREATE INDEX IF NOT EXISTS idx_trade_customer        ON canon.trade_instrument (customer_id);
CREATE INDEX IF NOT EXISTS idx_locker_party          ON canon.locker_access (party_id, at);
CREATE INDEX IF NOT EXISTS idx_kyc_document_party    ON canon.kyc_document (party_id);
CREATE INDEX IF NOT EXISTS idx_kyc_document_evidence ON canon.kyc_document (evidence_object_id)
    WHERE evidence_object_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_cbs_activity_user     ON canon.cbs_user_activity (user_ref, at);
CREATE INDEX IF NOT EXISTS idx_cbs_activity_at_brin  ON canon.cbs_user_activity USING BRIN (at);

-- Grants. Writes by akr_ingest only; reads by akr_app. The reporting role gets none of these tables in this phase:
-- phases 21 and 24 add the spelled-out list they need (DATA-MODEL s.21).
GRANT SELECT, INSERT ON canon.card, canon.card_txn, canon.cash_event, canon.fx_deal, canon.facility,
    canon.trade_instrument, canon.locker_access, canon.kyc_document, canon.cbs_user_activity TO akr_ingest;
GRANT SELECT ON canon.card, canon.card_txn, canon.cash_event, canon.fx_deal, canon.facility,
    canon.trade_instrument, canon.locker_access, canon.kyc_document, canon.cbs_user_activity TO akr_app;

-- Classification. Card data: `card` for the masked PAN and its keyed hash; cards are never landed unmasked (section 3.3).
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'canon', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('card', 'card', ARRAY['pan_masked','pan_hk']),
  ('card', 'personal', ARRAY['holder_party_id']),
  ('card', 'internal', ARRAY['id','account_id','scheme','product','status']),
  ('card_txn', 'internal', ARRAY['card_id']),
  ('card_txn', 'financial', ARRAY['amount','amount_lcy','amount_usd','rate_source','rate_date','currency','dr_cr']),
  ('card_txn', 'internal', ARRAY['id','txn_id','mcc','merchant_id','terminal_id','acquirer','country','entry_mode',
                                 'ecommerce','response_code','at']),
  ('cash_event', 'personal', ARRAY['denominations','declared_source']),
  ('cash_event', 'internal', ARRAY['id','txn_id','booking_date','ctr_flag','branch_id']),
  ('fx_deal', 'financial', ARRAY['amount','amount_lcy','amount_usd','rate_source','rate_date','currency','dr_cr',
                                 'rates','pair']),
  ('fx_deal', 'internal', ARRAY['id','value_date','counterparty_id','exchange_house']),
  ('facility', 'financial', ARRAY['limit_amount','outstanding','currency']),
  ('facility', 'personal', ARRAY['events']),
  ('facility', 'internal', ARRAY['customer_id']),
  ('facility', 'internal', ARRAY['id','product_id','purpose','contract_type','status']),
  ('trade_instrument', 'financial', ARRAY['amount','amount_lcy','amount_usd','currency']),
  ('trade_instrument', 'personal', ARRAY['parties','goods','vessel_imo','ports']),
  ('trade_instrument', 'internal', ARRAY['id','customer_id','type','hs_codes','dates']),
  ('locker_access', 'personal', ARRAY['locker_ref']),
  ('locker_access', 'internal', ARRAY['id','party_id','at']),
  ('kyc_document', 'personal', ARRAY['expires_on']),
  ('kyc_document', 'internal', ARRAY['id','party_id','type','evidence_object_id']),
  ('cbs_user_activity', 'personal', ARRAY['user_ref','terminal_id','session_id','object']),
  ('cbs_user_activity', 'internal', ARRAY['id','action','at','override_flags']),
  ('cbs_user_activity', 'personal', ARRAY['ip_address_enc'])
) AS v(t, cls, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

-- Lineage and bookkeeping columns of every canon table are internal (DATA-MODEL s.0 [lineage]). Parent tables only:
-- partitions are created later by the factory and are classed with their parent's column set.
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT c.table_schema, c.table_name, c.column_name, 'internal'::admin.data_classification
FROM information_schema.columns c
WHERE c.table_schema = 'canon'
  AND c.column_name IN ('id', 'source_id', 'source_table', 'source_pk', 'source_version', 'row_hash', 'batch_id',
                        'extracted_at', 'mapping_version_id', 'created_at', 'valid_from', 'valid_to',
                        'recorded_from', 'recorded_to', 'superseded_at')
  AND c.table_name IN (SELECT r.relname FROM pg_catalog.pg_class r
                       JOIN pg_catalog.pg_namespace n ON n.oid = r.relnamespace
                       WHERE n.nspname = 'canon' AND r.relkind IN ('r', 'p') AND NOT r.relispartition)
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

-- Retention (DATA-MODEL s.5 "Class" column).
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('canon', 'card',               'R3', 'owner',            NULL,         'Masked card copy; seven years after the relationship'),
 ('canon', 'card_txn',           'R2', 'partition',        'at',         'Monthly partitions by transaction month'),
 ('canon', 'cash_event',         'R2', 'partition',        'booking_date', 'As its transaction'),
 ('canon', 'fx_deal',            'R2', 'partition',        'value_date', 'Monthly partitions by value month'),
 ('canon', 'facility',           'R3', 'owner',            NULL,         'Follows the customer'),
 ('canon', 'trade_instrument',   'R3', 'owner',            NULL,         'Follows the customer'),
 ('canon', 'locker_access',      'R2', 'partition',        'at',         'Monthly partitions'),
 ('canon', 'kyc_document',       'R3', 'owner',            NULL,         'Follows the party'),
 ('canon', 'cbs_user_activity',  'R2', 'partition',        'at',         'Optional per bank; monthly partitions')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

### 1.5 Inventory of section 1

| Schema | Objects | Count | Partitioned by month | Bi-temporal | Retention |
|-|-|-|-|-|-|
| `admin` | `mappings`, `mapping_versions`, `code_maps`, `code_map_entries`, `drift_events`, `xref`, `attribute_definitions`, `attribute_values`, `mapping_runs`; enums `mapping_status`, `mapping_run_status`; five settings | 9 tables | no | `attribute_values` | R9 (R3 for `xref`, `attribute_values`) |
| `canon` | 29 tables of DATA-MODEL s.5 (sections 1.2 to 1.4 list them by group) | 29 tables | 10 (`balance_snapshot`, `txn`, `txn_event`, `card_txn`, `payment_message`, `payment_message_party`, `cash_event`, `fx_deal`, `locker_access`, `cbs_user_activity`) | 5 (`party`, `party_relationship`, `customer`, `account`, `account_party_role`) | R2 facts, R3 parties and accounts, R9 reference |
| `canon` function | `create_partition(table, year, month)`, SECURITY DEFINER, allow-listed to the ten partitioned tables | 1 | | | |

Totals: 9 + 29 = 38 tables, 1 SQL function, 2 enums, 5 settings, 0 feature flags. Every table has its
`column_classification` rows and its `retention_assignment` row in the same migration (phase 00 check). Per-table
grants are in the migration that creates the table. `CREATE INDEX` names are `idx_<table>_*`, `uq_<table>_*`, or
`idx_<table>_src` and `idx_<table>_mv` for lineage, created by a loop.

---

## 2. DDD Implementation (Rust)

**Crate placement** (workspace pinned in phase 00; layer rules in `ddd.md`; driver isolation by `cargo deny`):

```
backend/
├── catalogue/
│   ├── canonical-model.v1.json     # the 29 canon entities: table, columns, classification, nullability, required
│   │                               #   (generated once from DATA-MODEL s.5 by tools/build_canonical_catalogue.py, reviewed)
│   └── canonical-attributes.v1.json   # phase 05 file; every key must name a canon column (section 3.8 test)
├── schemas/
│   ├── mapping.v1.json             # DSL.md Part B body, the schema pass of section 3.2
│   └── canonical-model.v1.json     # the catalogue file's schema
├── crates/
│   ├── sqlproof/   # NEW, pure (no driver): SelectProof, ProofScope, ProvenSelect, dialect parsers (moved from
│   │               #   akr-sources, section 4.2). Used by domain (compiler), akr-sources (readers), akr-ctl
│   ├── dsl/        # + mapping (document model, schema pass), transform (closed library, LIBRARY_VERSION), expr (the
│   │               #   Expr subset of DSL.md s.1; phase 16 reuses it), code_map (kinds and gates)
│   ├── domain/     # + mapping (MappingFamily, MappingVersion, MappingCompiler, CompiledMapping, DriftDetector,
│   │               #   CodeMapService logic), canon (CanonicalEntity, LineageRef, CardGuard, RowHash, WriteOutcome),
│   │               #   attribute (AttributeDefinition, AttributeValue); change targets `mapping`, `code_map`
│   ├── db/         # + Pg repositories for the nine admin tables and the canon writer and reader (one generic
│   │               #   CanonRepository over the entity enum, section 2 notes)
│   ├── authz/      # + resource adapters Mapping, MappingVersion, CodeMap, DriftEvent, MappingRun, Attribute; ChangeKind
│   │               #   family `mapping` (kinds mapping, code_map)
│   └── jobs/       # + kinds mapping_compile, drift_scan, code_map_prefill, mapping_preview_purge (akr-worker) and
│                   #   ingest_mapping_sample (akr-ingestd)
├── api/            # + routers mappings (incl. GET /sources/{id}/view-ddl-pack), code_maps, drift, canonical, attributes
├── ingestd/        # + MappingSampler, the ingest_mapping_sample handler, the CanonicalWriter (library, phase 11 calls it)
├── worker/         # + the four job handlers, DriftDetector wiring, ViewDdlBuilder (pure, in domain)
└── ctl/            # + mapping lint <file>, canon verify-catalogue
```

akr-api and akr-worker link `sqlproof` and `dsl`, never `akr-sources`; akr-ingestd links both. The compiler and the
view-DDL builder are therefore runnable where the bank's metadata is already stored, with no CBS connection
(DR-85).

- **Domain entities** (`Debug, Serialize, Deserialize, FromRow, ToSchema`; fields mirror section 1): `MappingFamily`,
  `MappingVersion` (`status` as `#[sqlx(type_name = "admin.mapping_status")]`), `CodeMap`, `CodeMapEntry`,
  `DriftEvent`, `XrefRow`, `AttributeDefinition`, `AttributeValue`, `MappingRun` (`#[sqlx(type_name =
  "admin.mapping_run_status")]`).
- **Value objects:** `DslId` (`<source>.<entity>`), `CanonicalEntity` (closed enum of the 29 names, `Ord` by catalogue
  order; `CanonicalEntity::parse` refuses others with `E_TARGET_UNKNOWN`), `Transform` (closed enum: `ParseDate`,
  `ParseTimestamp`, `TzConvert`, `CombineDateTime`, `ParseDecimal`, `DecimalScale`, `CurrencyMinorUnits`, `Abs`,
  `DrCrFromSign`, `XmlField`, `MultivalueSplit`, `CodeMap(kind)`, `NormalisePhone(prefix)`, `SplitArabicName`,
  `DigitsWestern`, `Trim`, `Upper`, `Lower`, `IbanValidate`, `MaskPan`, `KeyedHash(purpose)`, `Coalesce`, `Concat`;
  each with a version in `LIBRARY_VERSION`), `ValidationRule` (`NotNull`, `Ref`, `Range`, `Enum`, `IbanChecksum`,
  `NotPlaceholderDate`, `NoMojibake`, `UniqueKey`), `OnFail` (`DqIssue`, `Quarantine`), `Strategy` (`Poll`, `Stream`,
  `File`), `WatermarkKind` (the eight of DSL.md B.2), `Engine` (re-exported from phase 05), `CodeMapKind` (the ten of
  migration 1), `Gate` (`Quarantine`, `DqIssue`), `DriftKind` and `DriftSeverity` (section 3.5), `RunKind` (`Test`,
  `Preview`), `ExprAst` (the Expr subset; cost bound `E_EXPR_COST`), `Classification` (re-exported from phase 00),
  `LineageRef` (the eight lineage values), `RowHash([u8; 32])`.
- **Create/Update DTOs:** `NewMappingFamily { dsl_id, source_id, target_entity, name }`, `NewMappingVersion { mapping_id,
  document }`, `DraftReplace { document, revision }` (the PUT body; `If-Match` carries `revision`), `NewCodeMap { kind,
  source_id, source_list, gate, name, prefill: bool }`, `CodeMapChange { ops: Vec<CodeMapOp> }` with `CodeMapOp::Map
  { source_list, source_code, canonical_code, valid_from }`, `Reclassify { ..., supersedes_valid_from }`, `Verify {
  entry_id }`, `NewAttributeDefinition { entity, name, type, classification, doc }`, `NewMappingRun { version_id,
  kind, sample: SampleSpec }`, `DriftResolve { comment? }`.
- **Repository traits** (domain layer; `Pg*` implementations in `db`):

```rust
#[async_trait]
pub trait MappingRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, m: &NewMappingFamily, actor: Uuid) -> Result<MappingFamily, RepoError>;  // 409 on uq
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<MappingFamily>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &MappingFilter, page: Keyset) -> Result<Page<MappingFamily>, RepoError>;
    async fn touch(&self, tx: &mut Tx, id: Uuid) -> Result<(), RepoError>;
}

#[async_trait]
pub trait MappingVersionRepository: Send + Sync {
    /// Next DSL version number for the family: MAX(version) + 1, under the family row lock (FOR UPDATE on mappings).
    async fn open_draft(&self, tx: &mut Tx, mapping_id: Uuid, document: &Value, sha: [u8; 32], actor: Uuid)
        -> Result<MappingVersion, RepoError>;                                                  // 409 on uq_mapping_versions_open
    async fn get(&self, conn: &mut PgConnection, mapping_id: Uuid, version: i32) -> Result<Option<MappingVersion>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, mapping_id: Uuid) -> Result<Vec<MappingVersion>, RepoError>;
    /// Compare-and-set on `revision`; None when the revision moved (the service answers 409 E_VERSION_CONFLICT).
    async fn replace_draft(&self, tx: &mut Tx, id: Uuid, expected_revision: i32, document: &Value, sha: [u8; 32])
        -> Result<Option<MappingVersion>, RepoError>;
    async fn store_compiled(&self, tx: &mut Tx, id: Uuid, compiled: &Value, sha: [u8; 32], lib: &str,
                            status: MappingStatus) -> Result<MappingVersion, RepoError>;
    async fn set_status(&self, tx: &mut Tx, id: Uuid, from: &[MappingStatus], to: MappingStatus,
                        cr: Option<Uuid>) -> Result<Option<MappingVersion>, RepoError>;   // conditional, as phase 05
    async fn publish(&self, tx: &mut Tx, id: Uuid, at: DateTime<Utc>) -> Result<MappingVersion, RepoError>;  // retires the prior published
    async fn published_for_source(&self, conn: &mut PgConnection, source_id: Uuid) -> Result<Vec<MappingVersion>, RepoError>;
}

#[async_trait]
pub trait CodeMapRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, c: &NewCodeMap, actor: Uuid) -> Result<CodeMap, RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<CodeMap>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &CodeMapFilter, page: Keyset) -> Result<Page<CodeMap>, RepoError>;
    async fn insert_entries(&self, tx: &mut Tx, rows: &[NewCodeMapEntry]) -> Result<u64, RepoError>;   // ON CONFLICT DO NOTHING
    async fn verify(&self, tx: &mut Tx, entry_ids: &[Uuid], at: DateTime<Utc>) -> Result<u64, RepoError>;
    async fn close(&self, tx: &mut Tx, entry_id: Uuid, valid_to: NaiveDate) -> Result<bool, RepoError>;
    async fn entries(&self, conn: &mut PgConnection, map_id: Uuid, page: Keyset) -> Result<Page<CodeMapEntry>, RepoError>;
    async fn effective(&self, conn: &mut PgConnection, map_id: Uuid, on: NaiveDate) -> Result<Vec<CodeMapEntry>, RepoError>;
}

#[async_trait]
pub trait DriftRepository: Send + Sync {
    /// Opens or refreshes the open event for (source, table, kind); returns the new row or None when one is open.
    async fn open(&self, tx: &mut Tx, e: &NewDriftEvent) -> Result<Option<DriftEvent>, RepoError>;
    async fn resolve(&self, tx: &mut Tx, id: Uuid, by: Uuid, expected_version: i32) -> Result<DriftEvent, RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &DriftFilter, page: Keyset) -> Result<Page<DriftEvent>, RepoError>;
    async fn open_pauses(&self, conn: &mut PgConnection, source_id: Uuid) -> Result<Vec<(String, DriftKind)>, RepoError>;
}

#[async_trait]
pub trait MappingRunRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, r: &NewMappingRun) -> Result<MappingRun, RepoError>;   // uq_mapping_runs_active -> 409
    async fn start(&self, tx: &mut Tx, id: Uuid) -> Result<Option<MappingRun>, RepoError>;
    async fn complete(&self, tx: &mut Tx, id: Uuid, result: &Value, preview: Option<&Value>) -> Result<MappingRun, RepoError>;
    async fn fail(&self, tx: &mut Tx, id: Uuid, code: MappingRunError) -> Result<MappingRun, RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<MappingRun>, RepoError>;
    async fn purge_previews(&self, tx: &mut Tx, older_than: DateTime<Utc>) -> Result<u64, RepoError>;
}

#[async_trait]
pub trait XrefRepository: Send + Sync {
    async fn get_many(&self, conn: &mut PgConnection, source_id: Uuid, entity: CanonicalEntity, keys: &[String])
        -> Result<HashMap<String, Uuid>, RepoError>;
    async fn insert_missing(&self, tx: &mut Tx, rows: &[XrefRow]) -> Result<u64, RepoError>;   // ON CONFLICT DO NOTHING
}

#[async_trait]
pub trait AttributeRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, a: &NewAttributeDefinition, actor: Uuid) -> Result<AttributeDefinition, RepoError>;
    async fn list(&self, conn: &mut PgConnection, entity: Option<CanonicalEntity>, page: Keyset)
        -> Result<Page<AttributeDefinition>, RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<AttributeDefinition>, RepoError>;
}

/// One generic repository over the 29 canon entities: the writer and the reader address a table by its enum value,
/// and each table's columns come from canonical-model.v1.json, so there is no per-entity CRUD.
#[async_trait]
pub trait CanonRepository: Send + Sync {
    async fn current_by_source_key(&self, tx: &mut Tx, e: CanonicalEntity, source_id: Uuid, keys: &[String])
        -> Result<HashMap<String, CurrentRow>, RepoError>;                                  // bi-temporal: recorded_to IS NULL
    async fn insert_rows(&self, tx: &mut Tx, e: CanonicalEntity, rows: &[CanonRow]) -> Result<u64, RepoError>;
    async fn close_versions(&self, tx: &mut Tx, e: CanonicalEntity, ids: &[(Uuid, DateTime<Utc>, NaiveDate)])
        -> Result<u64, RepoError>;                                                           // sets recorded_to, valid_to
    async fn ensure_partition(&self, tx: &mut Tx, e: CanonicalEntity, month: NaiveDate) -> Result<(), RepoError>;
}
```

- **Repository notes:** every list is keyset-paged: families by `dsl_id`, versions by `version DESC`, code maps by
  `(kind, source_list)`, entries by `(source_list, source_code, valid_from DESC)`, drift events by `(created_at DESC,
  id DESC)` with unresolved first, runs by `(created_at DESC, id DESC)`, attributes by `(entity, name)`. The
  `CanonRepository` inserts are multi-row `INSERT ... VALUES` batches of at most 1,000 rows, each with `RETURNING
  id`; a missing partition (SQLSTATE 23514, no partition of relation) calls `canon.create_partition` once and retries.
  `open_pauses` reads only `paused` rows (the index `idx_drift_paused`).

- **Ports:**

```rust
/// Discovery metadata already stored by phase 05 (no CBS access). Used by the compiler, the drift detector and the
/// view-DDL builder.
#[async_trait]
pub trait SourceCatalogue: Send + Sync {
    async fn latest_profile(&self, conn: &mut PgConnection, source_id: Uuid, object: &str)
        -> Result<Option<ObjectProfileSnapshot>, RepoError>;        // columns (name, data type, nullable, flags), newest completed run
    async fn approved_objects(&self, conn: &mut PgConnection, source_id: Uuid) -> Result<Vec<ApprovedObject>, RepoError>;
    async fn code_list_entries(&self, conn: &mut PgConnection, source_id: Uuid, list_key: &str)
        -> Result<Vec<CodeEntry>, RepoError>;                        // phase 05 discovery_code_entries
    async fn adapter_pack(&self, pack: &str) -> Result<Option<AdapterPackView>, RepoError>;   // dictionary hints, deny columns
}

/// The one reader of CBS rows in this phase: akr-ingestd only (phase 05 SourceReader, LoadGovernor, SelectProof).
#[async_trait]
pub trait SampleSource: Send + Sync {
    async fn sample(&self, run: &MappingRun, plan: &ProvenSampleSet, sink: &mut dyn RowSink) -> Result<SampleStats, SourceError>;
}

/// The pure gate. Implemented by sqlproof::SelectProof; the compiler calls prove() for every generated statement.
pub trait StatementGate: Send + Sync {
    fn prove(&self, sql: &str, dialect: Engine, scope: &ProofScope<'_>) -> Result<ProvenSelect, ProofError>;
}

/// Read by the sampler and by phase 11's ingest loop to find tables that must not be read.
pub trait DriftPauseReader: Send + Sync {
    fn paused_tables(&self, source_id: Uuid) -> BTreeSet<String>;   // OWNER.NAME of each open `pause` event
}
```

- **utoipa schemas:** `CanonicalCatalogueView` (entities with columns, classification, nullability, `required`,
  `partition`, `bitemporal`, `lineage: true`), `MappingFamilyView` (`dsl_id`, source code, entity, `published_version`,
  `open_version`, `affected_drift: [id]`), `MappingVersionView` (document, status, revision, `compiled` summary (reads,
  statements per dialect, warnings, `catalogue_hash`), `library_version`, `change_request_id`, `affordances`),
  `MappingDocumentView` (the DSL body; the editor's schema source), `MappingValidationView` (`ok`, `errors[]` with
  JSON-pointer `path`, DSL code and message in both languages), `MappingRunAccepted` (`run_id`, `Location`),
  `MappingRunView` (kind, status, sample spec, `result` (section 3.6), `preview` (only for `read_preview` callers;
  shape rows), error code), `CodeMapView` (kind, source, list, gate, `unmapped_count`, `pending_count`), `CodeMapEntryView`
  (entry, `origin`, `verified`, `valid_from/to`), `UnmappedCodeView` (`source_list`, `source_code`, `label_en`,
  `label_native`, `suggested_canonical?`), `CodeMapChangeRequest` (ops), `DriftEventView` (kind, severity, table, paused,
  detail, mapping version, resolvable flag), `DriftResolveRequest`, `AttributeDefinitionView`, `NewAttributeRequest`,
  `ViewDdlPackView` (statements per object, sha256; the `.sql` file is `?download=true`).

- **Route table** (base `/api/v1`; scopes from API.md section 2; **CR** = answers 202 with a change request (phase 04);
  **PR** = `read_preview` only, else the run is masked as 404; API.md 3.6 routes are listed first, the rest are added
  here and flagged in section 4.2):

| Method | Path | Scope | Description (cedar action) |
|-|-|-|-|
| GET | /canonical/catalogue | config.author, config.approve, dashboard.read | The 29 canon entities with columns, classification, nullability and required flags (`canonical.catalogue_read`) |
| GET | /mappings | config.author, config.approve, dashboard.read | Mapping families with the published and open version, filtered by `source_id`, `entity` (`mapping.list`) |
| POST | /mappings | config.author | Create a family for `source_id` and `target_entity`; `Idempotency-Key` (`mapping.create`) |
| GET | /mappings/{id} | config.author, config.approve, dashboard.read | Family detail with the version list and the drift events affecting it (`mapping.read`) |
| POST | /mappings/{id}/versions | config.author | Open a draft version, copied from the published one or empty; 201 (`mapping.version_open`) |
| PUT | /mappings/{id}/versions/{v} | config.author | Replace the draft document (DSL.md Part B); `If-Match: revision`; draft only (`mapping.version_update`) |
| POST | /mappings/{id}/versions/{v}/validate | config.author | Schema, semantic and compile validation; on success `draft` -> `testing` with the compiled artefact; 200 or 422 (`mapping.validate`) |
| POST | /mappings/{id}/versions/{v}/test | config.author | Queue a test run: sample rows, rule outcomes and the impact against the published version; 202 (`mapping.test`) |
| POST | /mappings/{id}/versions/{v}/preview | config.author | Queue a preview run: shape rows beside source shapes; 202 (`mapping.preview`) |
| GET | /mappings/{id}/versions/{v}/impact | config.author, config.approve, dashboard.read | The impact summary of the latest completed test run (`mapping.impact`) |
| GET | /code-maps | config.author, config.approve, dashboard.read | Code maps by `source_id`, `kind` (`code_map.list`) |
| POST | /code-maps | config.author | Create a code map for a source list; `prefill: true` queues `code_map_prefill`; 201 (`code_map.create`) |
| GET | /code-maps/{id}/entries | config.author, config.approve, dashboard.read | Effective-dated entries, with unmapped CBS codes listed first (`code_map.read`) |
| POST | /code-maps/{id}/entries | config.author | Map, reclassify or verify codes; **CR** `code_map` (`code_map.entry_propose`) |
| GET | /drift-events | data.steward | Drift events, unresolved first; `source_id`, `paused?` (`drift.read`) |
| POST | /drift-events/{id}/resolve | data.steward | Resolve after the affected mapping version is published or the drift is informational; `If-Match` (`drift.resolve`) |
| GET | /sources/{id}/view-ddl-pack | data.steward | The view DDL for the bank's DBA: one `CREATE VIEW` per approved object the mappings read, columns only (`mapping.view_ddl`) |
| GET | /mapping-runs/{id} | config.author, config.approve, dashboard.read | One run: status, result; preview rows for `read_preview` callers only, otherwise 404 (`mapping_run.read`, `mapping_run.read_preview`) |
| GET | /attributes | config.author, config.approve, dashboard.read | The attribute registry by entity (`attribute.read`) |
| POST | /attributes | config.author | Define an attribute: name, type, classification, owner (`attribute.define`) |

Scopes are API.md section 2 scopes only; Cedar narrows them (section 3.10). Routes that authorise a CBS read
(`/test`, `/preview`) are DS only in Cedar. There is no `POST /mappings/{id}/replays`, `GET /mapping-replays/{id}` or
`POST /mapping-replays/{id}/apply`: those are phase 11 (replay over the raw landing layer, DR-63).

- **Router functions:** `mappings::router() -> OpenApiRouter<AppState>` (11 routes: the families, versions, validate,
  test, preview, impact, `view-ddl-pack`, and `mapping-runs`), `code_maps::router()` (4), `drift::router()` (2),
  `canonical::router()` (1), `attributes::router()` (2); merged in `build_app` under the authz layer.
  `POST` routes take `Idempotency-Key`; `PUT` and the drift resolve take `If-Match`; `POST /code-maps/{id}/entries`
  answers 202 with `Location`. `GET /sources/{id}/view-ddl-pack?download=true` sets `Content-Type: application/sql`,
  `Content-Disposition: attachment; filename="akr-views-<code>-<date>.sql"` and `Cache-Control: no-store`.

## 3. Business Logic

### 3.1 Mapping families and draft versions (`MappingService`; C-51, DSL.md B.9 steps 1 and 7)

Methods: `create_family(ctx, NewMappingFamily) -> MappingFamily`, `open_version(ctx, mapping_id, from: Option<i32>)`,
`replace_draft(ctx, mapping_id, v, DraftReplace)`, `validate(ctx, mapping_id, v) -> MappingValidationView`,
`start_run(ctx, mapping_id, v, kind) -> MappingRun`, `resolve_drift(ctx, id, DriftResolve)`.

1. `create_family`: the source is registered and not `disabled`; `target_entity` is in the catalogue (`E_TARGET_UNKNOWN`,
   422 `E_VALIDATION`); `dsl_id = source.code || '.' || entity`; insert; audit `mapping.created`.
2. `open_version`: the family has no open version (`uq_mapping_versions_open`, 409 `E_NOT_ALLOWED_IN_STATE` reason
   `version_open`); `from` copies the published document or is empty with the entity's required fields listed; version
   number is the family's maximum plus one, taken under `SELECT ... FOR UPDATE` on the family row; audit
   `mapping.version_opened {mapping, version, from}`.
3. `replace_draft`: status must be `draft` (`E_NOT_ALLOWED_IN_STATE`, reason `not_draft`); `expected_revision` from
   `If-Match` (428 without it, 409 `E_VERSION_CONFLICT` with the current revision); `document` schema-checked (step 1
   of section 3.2); the new `document_sha256` and `revision + 1` are written by compare-and-set; audit
   `mapping.version_updated {mapping, version, changed: [json pointers], revision}` with the actor and the reason text
   (DSL.md B.9 step 1: every edit is audited with actor and reason).
4. `validate`: runs `MappingCompiler::compile` (section 3.2) synchronously; on success stores the artefact by
   `store_compiled` and moves `draft` -> `testing`; on failure returns 422 with `errors[]` and leaves the status alone.
   Audit `mapping.version_validated {mapping, version, ok, error_count, library_version}`.
5. `start_run`: status must be `testing` (for `test`) or `testing` or `published` (for `preview`), else
   `not_testing`; the source must be `active` (`source_not_active`, `source_paused` from phase 05 states), and its latest
   grant check `ok` (else `source_refused`, 409, phase 05 `E_SOURCE_REFUSED`); the run row is inserted with
   `document_sha256` of the current draft; the partial unique index gives 409 `run_active`; the job
   `ingest_mapping_sample {run_id}` is enqueued in the same transaction. Audit `mapping_run.requested {run, kind, version}`.
6. `resolve_drift` (section 3.5 step 6) and `replace_draft` never touch `published` rows: a published version changes
   only through a change request (section 3.9).

### 3.2 The compiler (`MappingCompiler`, pure, `domain::mapping` over `dsl` and `sqlproof`; C-50, C-52)

`MappingCompiler::compile(doc: &MappingDocument, env: &CompileEnv) -> Result<CompiledMapping, CompileError>` where
`CompileEnv` holds `catalogue: &CanonicalCatalogue`, `approved: &[ApprovedObject]`, `profile: Option<ObjectProfileSnapshot>`
per object, `codes: &[CodeMapView]` (effective entries per kind), `attributes: &[AttributeDefinition]`, the
`StatementGate`, and `library_version`. Numbered steps; the first failing step stops, and every error found in the
same step is reported in `errors[]`:
1. **Schema pass.** The body validates against `schemas/mapping.v1.json` (DSL.md Part B). `dsl_version` must be `"1"`
   (`E_UNKNOWN_DSL_VERSION`). Unknown keys are refused, not ignored.
2. **Target.** `entity` is in the catalogue (`E_TARGET_UNKNOWN`). `id` equals `<source.code>.<entity>` (`E_VALIDATION`,
   path `/id`).
3. **Source object.** `source.object` equals an exact `OWNER.NAME` entry of `approved_read_objects` (else
   `E_OBJECT_NOT_APPROVED`, 422, with the approved names listed). `source.format` is `relational` in this phase;
   `t24_*`, `landing_file`, `iso20022`, `mt` are refused with `E_VALIDATION` reason `format_not_in_phase`
   (phase 11 section 7+ adds them, DR-393). `source.key` names existing columns of the profile.
4. **Columns.** Each `from` names a column of the latest profile of the object (`COLUMN`, or `field:NAME` resolved
   through the adapter pack's dictionary positions, or `local:NAME` in the adapter pack's local references). A column
   the profile does not have is `E_VALIDATION` path `/fields/<attr>/from`. A column that the adapter pack's
   `deny_columns` lists, or that the profile marks card-like, is refused (`E_VALIDATION`, reason `denied_column`).
5. **Expressions.** Each `expr` parses with the `dsl::expr` subset over the typed row context; each `filter` expression
   is checked the same way; cost above the bound is `E_EXPR_COST`; a function outside the AKR subset is
   `E_VALIDATION`.
6. **Transforms.** Each `do` item names a `Transform` of the closed library with a valid argument (`E_VALIDATION` path
   `/fields/<attr>/do/<i>`). `code_map:<kind>` must name a code map of this source with that kind (`E_VALIDATION`,
   reason `code_map_missing`). A `mask_pan` transform is mandatory for a field whose profile shows a Luhn-valid run
   (section 3.3 card guard, `E_PAN_UNMASKED`).
7. **Required attributes.** Every attribute of the entity the catalogue marks `required` is mapped or has a `default`
   (`E_REQUIRED_TARGET_UNMAPPED`, listing the missing names).
8. **Defaults on monitored fields.** A `default` on an attribute the catalogue marks `monitored` (a scenario, rating or
   screening input) is `E_DEFAULT_ON_MONITORED`. Defaults elsewhere are allowed and listed as warnings.
9. **Extension attributes.** `attribute: true` requires a definition in the registry for this entity (`E_UNKNOWN_ATTRIBUTE`).
10. **Validation rules.** Each rule names a mapped field (or a key), and `on_fail` defaults to `quarantine` for key
    fields, `dq_issue` otherwise. `ref` rules name a canonical table.
11. **Change strategy.** `watermark.kind` is allowed for the entity's class (`none` refused for fact tables; a
    `time_of_day` column without a date refused); `overlap` is at least `mapping.default_overlap_days` (floor 31,
    setting of section 1.1); `deletes: key_reconcile` requires a key set.
12. **Generated statements.** For each dialect of the source engine (`postgres`, `mssql`, `oracle`, `odbc` for Db2 and
    Sybase), the generator emits one statement: `SELECT <approved source columns> FROM <owner>.<object> WHERE
    (<watermark column>, <key columns>) > (:wm, :k1, ...) [AND <watermark column> < :upper] ORDER BY <watermark
    column>, <key columns> FETCH FIRST :rows ROWS ONLY`, bind variables only, no other predicate. The sample form
    (`sample_spec`) is the same statement with `ORDER BY <key> DESC` and no lower bound. The generator has no per-entity
    filter, so a customer, account or case never appears in a predicate (C-35).
13. **Proof.** Each statement passes `StatementGate::prove` with `ProofScope::Data { approved, bound }`; a refusal is a
    product defect: `E_INTERNAL` with an incident, and the version does not compile.
14. **Catalogue hash.** `approved_catalogue_hash` (DSL.md B.1) = SHA-256 over the sorted lines `OWNER.NAME || 0x00 ||
    column || 0x00 || data_type || 0x00 || nullable` of every column the mapping reads, from the profile. Drift compares
    against it (section 3.5).
15. **Artefact.** Output `CompiledMapping { entity, source_object, statements: {dialect: {sql, shape_sha256}},
    reads: [{column, data_type}], fields: [...], validations, filters, watermark, catalogue_hash, library_version,
    warnings: [{code, path}] }`. Its canonical JSON hash is `compiled_sha256`.

### 3.3 Transform library, card guard and survivorship (`dsl::transform`, `domain::canon::CardGuard`; C-36, SP-48, SP-51, N-12)

- **Transforms** are pure functions `apply(Transform, Value) -> Result<Value, TransformFailure>`. `ParseDate` and
  `ParseTimestamp` never guess: an invalid input is a validation failure. `DigitsWestern` maps Arabic-Indic and
  Persian digits to Western (C-10). `FoldArabic` is the folding of research file 22, shared with phase 14 matching.
  `IbanValidate` checks the 23-character Iraqi IBAN checksum and decodes the bank and branch (C-44). `KeyedHash` takes the
  match key from OpenBao through the `keys/*` policy; its output is the `_hk` value.
- **Card guard** (`CardGuard::scan(text) -> Vec<CardHit>`): finds digit runs of 13 to 19 digits, allowing single
  separators, Luhn-validates them, and checks the issuer ranges and the bank's exclusion patterns (SP-48 list). A hit
  in a field whose mapping does not apply `MaskPan` stops that field: in the sampler it drops the row and adds a
  `card_guard` count to the result (section 3.6); in phase 11 the row is not written: it becomes a quarantine row with reason `card_guard` (7.16), and phase 12 opens a `card_guard` data-quality issue for the field, which is the data incident. The
  matched value is never written to a log, a run result or an audit detail; only the field name and the count are.
- **`MaskPan`** keeps the first six and the last four digits and replaces the middle with `*`. The `canon.card.pan_masked`
  CHECK of section 1.4 admits nothing else.
- **Survivorship** (DSL.md B.6): `source_priority` and `most_recent` are evaluated by the writer when two mappings of the
  same entity supply one attribute for one entity id; `longest_verified` is refused in this phase (`E_VALIDATION`
  reason `rule_not_in_phase`). The chosen value records the rule and the source (`source_ref` in the attribute value).
  Party merge and split are P2 (C-53, DR-394).
- **Name data-quality** (N-12): a field of `party_name` carries the profile's name-quality counts as `quality_flags`
  (`maybe_truncated` when the value reaches the source byte cap, `digits_in_name`); a flag never lowers a match score
  (DATA-MODEL s.5).

### 3.4 Code maps (`CodeMapService`, `code_map_prefill`; C-50, DSL.md B.7, DR-10, DR-88)

1. `create` (POST /code-maps): the source exists; `(source, kind, source_list)` is new (409 `E_NOT_ALLOWED_IN_STATE`
   reason `code_map_exists`); `gate` defaults to `quarantine` for `txn_type` and `dq_issue` otherwise; audit
   `code_map.created {kind, source_list, gate}`. When `prefill` is true and `kind <> 'currency'`, the job
   `code_map_prefill {code_map_id, requested_by}` is enqueued in the same transaction.
2. `code_map_prefill` (akr-worker): reads `discovery_code_entries` of the source's latest completed profile run for
   `source_list`; for each code that the adapter pack's `code_suggestions` maps to a canonical code, inserts an entry with
   `origin = 'prefill'`, `valid_from = current business date`, `verified_by_bank_at` null. Codes without a suggestion are
   not inserted: they appear as unmapped in the entries view with the CBS label. Audit `code_map.prefilled {code_map,
   inserted, unsuggested}`. Currency maps are never prefilled (DATA-MODEL s.0 Money: a CBS ISO column is never copied).
3. `entries` (GET): the effective entries on today's business date, the unmapped CBS codes first, then the pending
   prefilled entries, then verified entries. Each unmapped code shows `gate`.
4. `propose` (POST /code-maps/{id}/entries): creates the change request `code_map` with the operations; the request's
   class is computed (DSL.md D.3): `minor` when every operation maps a previously unmapped code, `material` when any
   operation reclassifies a mapped code (`Reclassify`). On activation (section 3.9): `Map` inserts an entry with
   `origin = 'change_request'`; `Reclassify` sets `valid_to` on the previous effective entry and inserts the new one
   with `valid_from` (history kept, DSL.md B.7); `Verify` sets `verified_by_bank_at` on the named entries (a prefilled
   entry is unmapped under the gate until this step). Audit `code_map.entry_proposed` on submit and
   `code_map.entry_activated {code_map, op, source_code, canonical_code, valid_from}` on activation.
5. The gate applies in the writer and in the sampler: a value with no effective, verified entry is unknown with a
   data-quality count (`dq_issue`), or quarantined for a `quarantine` gate (`txn_type`), and is replayed after the
   mapping changes (phase 11). It is never monitored as "other" (MODULES M6 edge case).

### 3.5 Drift detection (`DriftDetector`, job `drift_scan`; C-52, DSL.md B.8)

`DriftDetector::compare(profile: &ObjectProfileSnapshot, previous: &ObjectProfileSnapshot, mapping: &CompiledMapping,
codes: &[CodeMapView], labels: &[CodeEntry]) -> Vec<NewDriftEvent>`, pure. Kinds and their effect are fixed:

| Kind | Found when | Severity | Effect |
|-|-|-|-|
| `removed_object` | an approved object of a published mapping is missing from the catalogue | `pause` | the affected tables stop (bulkhead) |
| `removed_column` | a column the mapping reads is missing | `pause` | the affected table stops |
| `type_change` | a read column changes data type or nullability | `pause` | the affected table stops |
| `new_column` | a column appears on an object a mapping reads | `notify` | nothing; the steward may add it to a draft |
| `new_table` | a `live` candidate with no mapping appears in an approved schema | `notify` | nothing is read until a mapping names it (DATA-MODEL s.3) |
| `new_code` | a profiled value of a code-mapped column has no effective, verified entry | `gate` | the rows carrying it are quarantined or dq-counted (section 3.4 step 5) |
| `value_drift` | a mapped code's share of rows moves by at least `mapping.value_drift_points` between two profiles | `notify` | nothing |
| `label_change` | a code list's label differs from the stored entry's label | `notify` | the label is never overwritten silently |

Steps of `drift_scan` (akr-worker; payload `{source_id, run_id}`; enqueued by phase 05 `discovery_seal_report` after a
profile run is sealed, in the same transaction as the seal):
1. Load the newest completed profile run and the previous one of the source; load the published versions of the
   source and their compiled artefacts.
2. For each published version and each object it reads, call `DriftDetector::compare`.
3. For each finding, `DriftRepository::open` (one open event per `(source, table, kind)`; a repeat finds the open
   row and does nothing); `paused = true` when `severity = pause`. Audit `drift.opened {kind, table, severity,
   mapping_version}` and, for `pause`, `source.table_paused {table, cause}` (the source stays `active`; only the table
   stops).
4. Enqueue `mapping_compile {version_id, reason: "drift"}` for each affected version (coalesced per version); the
   recompile records whether the version still compiles.
5. Any `pause` event on a table of a source sets the pause in `DriftPauseReader`, which the ingest loop of phase 11 and
   the sampler read before every read. An approved change request of kind `mapping` that fixes the table ends the
   pause (step 6).
6. `POST /drift-events/{id}/resolve` (DS, `If-Match`): allowed when the affected version is `published` and compiles
   against the current profile (else 409 `E_NOT_ALLOWED_IN_STATE`, reason `version_not_published`), or for the
   `notify` kinds at any time; `paused` becomes false, `resolved_at` and `resolved_by` are set. Audit `drift.resolved`.
   There is no notification to DS in this phase: notifications arrive with phase 22, so open drift shows on T3 and in
   `GET /drift-events`.

### 3.6 The sampler (`MappingSampler`, akr-ingestd, job `ingest_mapping_sample`; C-35, C-36, C-13, DR-62)

Only akr-ingestd reads the CBS. The job is `ingest_` prefixed, so `akr_ingest` may run it under its RLS policy (phase 00).
1. Claim the run (`start`); `document_sha256` of the run must equal the stored draft's (else the run fails with
   `error_code = internal`; a new run is queued by the author, never by the job).
2. Re-check the source: `active` and not in `open_pauses` (`source_paused`), the grant check passing (`source_refused`),
   and the object still approved (`object_missing` on a removed object). The `LoadGovernor` (phase 05 section 3.5) admits
   the session: a window or blackout refusal is `budget_window`; a timeout is `statement_timeout` (not retried wider).
3. Build the sample statement (section 3.2 step 12, sample form, `FETCH FIRST rows`), with `rows` from
   `mapping.test_sample_rows` (default 500) for a test and 20 for a preview. The statement is proven again by the reader
   (`SelectProof`); a refusal is `sql_refused`.
4. For each row, `MappingExecutor::apply(row) -> MappedRow | Rejected`: transforms in order, validation rules,
   code-map lookups at the row's business date, the card guard on every text field (a hit drops the row and counts
   `card_guard`), and the lineage (`row_hash`, `source_pk`). No row value is written anywhere by this step.
5. Accumulate the result: rows read, mapped, rejected by rule, quarantined by the gate, unknown codes, and per field
   the null count, a shape histogram (a shape is `digits:12;scale:2`, `text:ar;len:8-30`, `date:yyyy-MM-dd`; never a
   value), and the source-side shape of the same column for comparison. For a test run only, the impact (step 6).
6. **Impact** (test runs). The rows are mapped again with the published version's compiled plan (if any) and compared:
   rows whose canonical value differs, per attribute, and the monitored attributes among them (names only). The
   scenario and coverage columns are filled by phases 16 and 06; this phase writes `scenarios: []` and the attribute
   names. A test without a published version writes `impact: {baseline: "none"}`.
7. **Preview** (preview runs): the first `preview_rows` mapped rows as shape rows: for a field whose classification is
   `personal`, `financial` or `secret-str`, or `card`, the row holds only its shape (for example `text:ar;len:14`,
   `decimal:digits:12;scale:2`, `null`); other fields hold their values. No personal or financial value leaves
   akr-ingestd in any run.
8. Complete the run (`complete(result, preview)`) in one transaction with the audit event. A card guard hit on a
   test run is never an error: it is counted and reported to the bank as a data-quality item (C-36).
9. `mapping_preview_purge` (akr-worker, daily) sets `preview = NULL` and `preview_purged_at = now()` for previews older
   than `mapping.preview_keep_hours`.

### 3.7 The canonical writer (`CanonicalWriter`, akr-ingestd library; C-22, C-23, C-24, C-46; phase 11 calls it)

`CanonicalWriter::write(tx, batch: &MappedBatch) -> Result<WriteOutcome, WriteError>`. It is a library in this phase:
no route and no job calls it except the tests and the sampler's dry-run mode, which never calls `write`. Phase 11
calls it in the load loop, inside the batch transaction that also inserts the receipt (C-20).
1. Group the mapped rows by `(entity, source_key)`. For each entity `e` in dependency order (branch, product, channel,
   staff, counterparty, party, party_name, party_identifier, party_contact, party_relationship, customer, account,
   account_party_role, account_hold, balance_snapshot, txn, txn_event, payment_message, payment_message_party,
   payment_link, card, card_txn, cash_event, fx_deal, facility, trade_instrument, locker_access, kyc_document,
   cbs_user_activity).
2. **Keys.** `xref` (source, key, entity) is read in one statement (`get_many`); a missing key gets a new UUIDv7 and an
   `XrefRow` (`insert_missing`, `ON CONFLICT DO NOTHING`; a concurrent insert returns the existing id on re-read).
3. **Idempotency.** `row_hash` is compared with the stored row for the same key:
   - same hash: no write (`unchanged`);
   - different hash on a **fact** (txn, txn_event, payment_message, payment_message_party, payment_link, account_hold, card_txn, cash_event, fx_deal, locker_access,
     cbs_user_activity, balance_snapshot): not overwritten; a `RowConflict` is returned for the caller to record as a
     data-quality issue (phase 12 table) and the new version stays in `raw` (DATA-MODEL s.0, DR-37);
   - different hash on a **bi-temporal** entity (party, party_relationship, customer, account, account_party_role): a
     new version. Step 4 closes the current one.
   - different hash on a **dimension** (branch, product, channel, staff, counterparty, card, facility, trade, kyc):
     a `RowConflict` with kind `label_change` when only text changed, else `value_change`; never an overwrite.
4. **Bi-temporal versions.** `valid_from` is the batch's business date of the row (DR-39), `recorded_from` is the
   writer's transaction time (`clock_timestamp()` read once per batch, passed explicitly so it is unique per id).
   `close_versions` sets `recorded_to` = the same time and `valid_to` = `valid_from - 1 day` on the current row when its
   `valid_from` is earlier; `insert_rows` inserts the new version. The partial unique index
   `uq_<table>_current` enforces one current row per source key.
5. **Facts.** `INSERT ... ON CONFLICT <target> DO NOTHING`, where the target is a unique constraint of the table
   (PostgreSQL refuses a target that no unique index matches): the source key with the partition key where a
   `uq_*_source` constraint holds it (`txn`, `payment_message`, `payment_message_party`, `card_txn`, `fx_deal`,
   `locker_access`, `cbs_user_activity`), the source key alone for `account_hold`, `(txn_id, booking_date, kind)` for
   `txn_event`, `(txn_id, booking_date)` for `cash_event`, `(account_id, business_date, source)` for
   `balance_snapshot` and `(message_id, txn_id)` for `payment_link`; a
   partition missing for the month (SQLSTATE 23514) calls `canon.create_partition` once and retries; a second missing
   partition is `E_INTERNAL` and the batch fails.
6. **Children.** `party_name`, `party_identifier` and `party_contact` are keyed by their source key and field (the
   `uq_*_current` indexes) and replaced by hash, not versioned (DATA-MODEL s.5 does not mark them bi-temporal): a changed
   hash sets `superseded_at` on the current row and inserts the new one, so history is kept and nothing is deleted; a change to a child of a party
   whose version closes in the same batch is written against the new party version.
7. **Encryption.** `_enc` columns and envelope attribute values are encrypted by the writer with the wrapped data key
   (OpenBao, phase 02 envelope); `_hk` values are computed with the match key (`KeyedHash`). Plain text never reaches
   the database for a column the catalogue classes as encrypted.
8. Return `WriteOutcome { inserted, unchanged, versions_closed, conflicts: Vec<RowConflict>, xref_linked }` and audit
   `canon.written {source, entity, batch_id, counts}` once per entity and batch (counts only, no keys or values).

### 3.8 The canonical catalogue and the attribute registry (`CanonicalCatalogue`; C-46)

- The catalogue file (`canonical-model.v1.json`) lists every canon entity with its columns, classification, nullability
  and two flags: `required` (mapping must supply it: keys, the money and date columns of a fact, the branch of a
  transaction, the `dr_cr` and `currency` of a posting) and `monitored` (an input of a scenario, rating or screening;
  filled by phases 13 to 16 from their DSL, the initial set is the risk-bearing columns of DATA-MODEL s.5 and the
  `CanonicalRef` examples of DSL.md s.1). Nullable columns in DATA-MODEL s.5 are nullable here.
- `akr-ctl canon verify-catalogue` (and a test) compares the file with the migrated `information_schema`: every column
  of every canon table appears in the catalogue with the same nullability and type, and no catalogue column is missing
  from the table. A drift between the two fails the build.
- **Mapping completeness for readiness** (phase 06's trailer defers it here): a monitored attribute of a scenario's
  needs, for a source, is `unmapped` when no published version of a mapping for that source and entity supplies its
  canonical column; draft and testing versions do not count. The grader (phase 06 section 3.2) reads this through
  `MappingCompleteness { source_id, entity, column, published_version }` computed from `admin.mapping_versions.compiled`
  (`reads` and `fields`), so the grade moves when a mapping is published and falls when a drift pauses its table.
  `unmapped` is one more reason state beside phase 06's `absent`, `unpopulated`, `unassessed`, `degraded` and
  `declaration_required`; it grades D as `absent` does.
- The phase 05 test (Open doubts 1): every key of `canonical-attributes.v1.json` names a column of the
  catalogue, so the field-availability attributes cannot drift from DATA-MODEL s.5.
- `AttributeDefinition` values are stored in `admin.attribute_values` (bi-temporal). A personal or secret-classed
  definition stores `{"enc": "<base64 AES-GCM>"}` in `value`; the reader decrypts only inside the caller's access check.

### 3.9 Change targets `mapping` and `code_map` (phase 04 `ChangeTarget`; DR-53)

| Kind | Route (route map) | `object_key` | Base version | Prepare | Activate | Revert | Closed |
|-|-|-|-|-|-|-|-|
| `mapping` | `cr_mapping` (object kind rule of phase 04 section 1) | `mapping:<mapping_id>` | published version number (0 when none) | the draft is `testing`; its compiled artefact matches `compiled_sha256` and the current profile; a `test` run completed for the same `document_sha256` (evidence `test_run`); the impact attached (evidence `impact_analysis`); class `material` (DSL.md D.3); `volume_effect`: `neutral` for the first publication of a family (base version 0: nothing was monitored from it before) and when the impact shows no change in a monitored attribute, else `unknown` | in one transaction: the version `published` (`published_at` = effective time), the previous published version `retired`, the drift events of the family resolved by the change, audit `mapping.published` | not allowed (rollback is a new change request that publishes the older version, DSL.md B.9 step 7) | `status` back to `testing` on rejection or withdrawal |
| `code_map` | `cr_mapping` | `code_map:<code_map_id>` | the code map's `version` | the operations are valid (section 3.4); class computed (`minor` or `material`, section 3.4 step 4); `volume_effect` `neutral` for `Map` and `Verify`, `unknown` for `Reclassify` until an impact run exists | the entries of section 3.4 step 4 | the entries are closed (`valid_to`) for the `Map` operations of the change | nothing |

- Evidence kinds resolved here: `impact_analysis` (a `mapping_run` of kind `test`; phase 04 lists it as resolving from
  phase 10) and `test_run` (the same table, extended by this phase's resolver).
- The route `cr_mapping` (approve role CHK, escalation CH after 3 business days, DSL.md Part E) is the one phase 04
  already ships. A maker never checks (phase 04 SoD); the DSL.md Part B approver "bank compliance or IT officer" is the
  CHK role.
- `volume_effect` `unknown` cannot be approved as material (phase 04 section 3.1). The first publication of a family is
  `neutral`, so an installation can publish its mappings in this phase. Until phase 11 provides the replay over the raw
  layer, a change to a published mapping that moves a monitored attribute waits for that replay (DR-88).

### 3.10 Errors, audit events, Cedar actions, tipping-off review

**Error mapping.** Top-level codes used: `E_VALIDATION` (422, `errors[]` with DSL.md s.7 codes `E_UNKNOWN_DSL_VERSION`,
`E_TARGET_UNKNOWN`, `E_REQUIRED_TARGET_UNMAPPED`, `E_DEFAULT_ON_MONITORED`, `E_PAN_UNMASKED`, `E_UNKNOWN_ATTRIBUTE`,
`E_EXPR_COST`, `E_I18N_PLACEHOLDERS`, and the reasons of sections 3.1 to 3.4); `E_OBJECT_NOT_APPROVED` (422);
`E_NOT_ALLOWED_IN_STATE` (409, reasons `not_draft`, `not_testing`, `version_open`, `run_active`, `code_map_exists`,
`source_not_active`, `version_not_published`); `E_VERSION_CONFLICT` and `E_PRECONDITION_REQUIRED`; `E_SOURCE_REFUSED`
(409); `E_FORBIDDEN` and `E_SCOPE`; `E_NOT_FOUND` (the masked preview run); `E_INTERNAL` (500, correlation id only: a
refused generated statement, a missing second partition). **No new problem codes** are added.

**Audit events emitted:**

| Event kind | On | Resource |
|-|-|-|
| `mapping.created` | family created | mapping |
| `mapping.version_opened`, `mapping.version_updated`, `mapping.version_validated` | draft lifecycle (actor, reason, changed pointers) | mapping_version |
| `mapping.published`, `mapping.retired` | effective change (section 3.9) | mapping_version |
| `mapping_run.requested`, `.completed`, `.failed` | run lifecycle (kind, counts, error code; no values) | mapping_run |
| `mapping_run.preview_read` | a preview read by a `read_preview` caller (access event `view`, tier normal) | mapping_run |
| `code_map.created`, `code_map.prefilled` | section 3.4 steps 1 and 2 | code_map |
| `code_map.entry_proposed`, `code_map.entry_activated` | section 3.4 step 4 | code_map |
| `drift.opened`, `drift.resolved` | section 3.5 | drift_event |
| `source.table_paused` | a `pause` event (table, cause) | source |
| `source.view_ddl_generated` | view DDL pack built (objects, sha256) | source |
| `attribute_definition.created` | registry entry | attribute_definition |
| `canon.written` | per entity and batch, section 3.7 step 8 | system |
| `audit.partition_created` (phase 02 event, reused) | `canon.create_partition` or `partition_maintenance` | canon partition |

Reused from earlier phases: `change_request.created` and `.activated` (phase 04), `authz.denied`, access events
(phase 03), `evidence.stored` (phase 02). Partition creation for the canon tables is registered in the phase 02
`PartitionRegistry` in code (section 4.1) and uses `audit.partition_created`.

**Cedar.** Bundle file `71_mapping.cedar` (range 70, sources and mapping; phase 05 uses `70_sources.cedar`). Entities
added to `schema.cedarschema`: `Mapping { source_state: String, target_entity: String }`, `MappingVersion { status:
String, mapping: Mapping }`, `CodeMap { kind: String, gate: String }`, `DriftEvent { kind: String, severity: String,
paused: Bool }`, `MappingRun { kind: String, status: String, version: MappingVersion }`, `Attribute { entity: String,
classification: String }`; `ChangeKind` family `mapping` with kinds `mapping`, `code_map`.

| Action | Resource | Who (shipped bundle) |
|-|-|-|
| `canonical.catalogue_read` | System | CFG, DS, CHK, M, CH, IA |
| `mapping.list`, `mapping.read`, `mapping_run.read` (test summary) | System, Mapping, MappingRun | CFG, DS, CHK, M, CH, IA |
| `mapping.create`, `mapping.version_open`, `mapping.version_update`, `mapping.validate`, `mapping.view_ddl` | Mapping, System | DS |
| `mapping.test`, `mapping.preview` | MappingVersion | DS (reads CBS rows, section 3.6) |
| `mapping.impact` | MappingVersion | CFG, DS, CHK, M, CH, IA |
| `mapping_run.read_preview` | MappingRun | DS, M; others masked |
| `code_map.list`, `code_map.read` | System, CodeMap | CFG, DS, CHK, M, CH, IA |
| `code_map.create`, `code_map.entry_propose` | System, CodeMap | DS |
| `drift.read`, `drift.resolve` | System, DriftEvent | DS |
| `attribute.read` | Attribute | CFG, DS, CHK, M, IA |
| `attribute.define` | System | DS |
| `change_request.create` (family `mapping`) | ChangeKind | DS |
| `change_request.read` (family `mapping`) | ChangeRequest | DS, CHK, CFG, M, CH, IA, or a member of `parties` |

```cedar
// 71_mapping.cedar (excerpt)
@id("mapping.steward_authoring") @role("DS")
permit (principal, action in [Akr::Action::"mapping.create", Akr::Action::"mapping.version_open",
                   Akr::Action::"mapping.version_update", Akr::Action::"mapping.validate",
                   Akr::Action::"mapping.test", Akr::Action::"mapping.preview", Akr::Action::"mapping.view_ddl"],
        resource)
when { principal.roles.contains("DS") };

// A published version is changed only by a change request; a disabled source is read-only (phase 05).
@id("mapping.published_immutable") @deny("state")
forbid (principal, action in [Akr::Action::"mapping.version_update", Akr::Action::"mapping.validate"],
        resource is Akr::MappingVersion)
when { resource.status == "published" || resource.status == "retired" };

@id("mapping.run_preview_restricted") @deny("masked")
forbid (principal, action == Akr::Action::"mapping_run.read_preview", resource is Akr::MappingRun)
when { !principal.roles.containsAny(["DS", "M"]) };

@id("mapping.masked_preview_run") @deny("masked")
forbid (principal, action == Akr::Action::"mapping_run.read", resource is Akr::MappingRun)
when { resource.kind == "preview" && !principal.roles.containsAny(["DS", "M"]) };

@id("drift.steward") @role("DS")
permit (principal, action in [Akr::Action::"drift.read", Akr::Action::"drift.resolve"], resource)
when { principal.roles.contains("DS") };
```

Attributes evaluated: `principal.roles`, `delegated_roles`, `active`; `resource.status`, `resource.kind`,
`resource.source_state`, `resource.paused`; `context.aal` on resolve and on publish (through the change request).
A masked deny is a 404 with the same body as a missing run (API.md section 1). Bundle tests: no role but DS can start a
CBS read (`/test`, `/preview`, `/view-ddl-pack`); CFG reads summaries and not previews; DS cannot approve its own
mapping change (the maker-checker pair of phase 04); a published version cannot be edited.

**Tipping-off review.**
- The mapping workbench, the compiled statements, the drift events and the code maps describe how the bank's core
  banking data is read: which tables, columns and CBS codes feed which canonical field. They name no customer, account,
  alert or case. The steward and the model roles may see them; front-line, branch, operations and KYC roles have no
  route to them.
- The canonical rows are the monitoring copy. Nothing in this phase shows a canonical row to a user: the sampler returns
  counts, shapes and, for detection-logic and internal fields only, the value of a code. A preview is masked to shapes
  and is itself masked (404) for every role except the steward and the MLRO, so its existence reveals nothing.
- The view-DDL pack for the bank's DBA contains object names and column names of approved objects and no predicate,
  so the bank's DBA learns which tables the monitoring reads and nothing about any investigation (the phase 05 rule
  about statements applies).
- The answer for a person outside a case's access list is: nothing, because no case exists here and no customer row is
  returned by any route in this phase.

**Settings and configuration used:** `mapping.test_sample_rows` (50 to 5000), `mapping.preview_keep_hours` (1 to 168),
`mapping.default_overlap_days` (governed, floor 31), `mapping.value_drift_points` (1 to 100), `schedule.mapping_preview_purge`
(cron, at most hourly); phase 05 `discovery.*` and the load budget; phase 00 `jobs.*`. No step-up action class is added
(section 4.2).

## 4. Infrastructure Updates

### 4.1 Jobs, partitions and the registry

- **Job kinds registered:**
  - akr-ingestd (`akr_ingest`; prefix `ingest_`, phase 00): `ingest_mapping_sample` (payload `{run_id}`; concurrency 2
    across sources, the phase 05 `LoadGovernor` admits each session; max attempts 1: a failed sample is a result, the run
    records `error_code`, not a retry).
  - akr-worker (`akr_app`; the prefixes `mapping_`, `drift_`, `code_map_` belong to `akr_app`, as phases 06 and 07 do):
    `mapping_compile` (payload `{version_id, reason}`; coalesce key `mapping_compile:<version_id>`; concurrency 2; max
    attempts 3), `drift_scan` (payload `{source_id, run_id}`; coalesce key `drift_scan:<source_id>`; concurrency 1; max
    attempts 5; enqueued by phase 05 `discovery_seal_report` in its transaction), `code_map_prefill` (payload
    `{code_map_id, requested_by}`; coalesce key `code_map_prefill:<id>`; concurrency 1; max attempts 3),
    `mapping_preview_purge` (scheduled `schedule.mapping_preview_purge`, daily 01:50 bank time; concurrency 1; max
    attempts 3).
- **Partitions.** The `canon` monthly partitions are created ahead by phase 02 `partition_maintenance`, which iterates
  the code `PartitionRegistry`. Phase 10 registers the ten partitioned canon tables with the factory
  `canon.create_partition` and `ahead_months = audit.partition_ahead_months` (phase 02 setting, default 3). A missing month
  at write time is created inline by the writer (section 3.7 step 5). No DEFAULT partition is created: a row with no
  partition fails loudly. Partitions are never dropped in this phase; the retention engine (phase 23) drops them.
- **Docker compose:** no change. No new service, no new network edge.
- **AppState additions:** `api`: `mappings: Arc<MappingService>`, `compiler: Arc<MappingCompiler>` (pure, shared with
  the worker), `code_maps: Arc<CodeMapService>`, `drift: Arc<DriftQuery>`, `canonical: Arc<CanonicalCatalogue>`,
  `attributes: Arc<AttributeService>`, `view_ddl: Arc<ViewDdlBuilder>`. `worker`: the four handlers, `DriftDetector`,
  `CompileHandler`, `PreviewPurger`. `ingestd`: `MappingSampler`, `MappingExecutor`, `CanonicalWriter`,
  `DriftPauseReader` (the `open_pauses` view). `ctl`: `mapping lint`, `canon verify-catalogue`.
- **OpenAPI registration:** tags `mappings` (11 operations), `code_maps` (4), `drift` (2), `canonical` (1),
  `attributes` (2): 20 in all, each with `x-akr-action` (the Cedar action).
  `POST` routes document `Idempotency-Key` and 202 with `Location` where they queue a run or a change request.
- **Network zones:** only akr-ingestd reaches a source (Z-zone rule of phase 05, unchanged); akr-api and akr-worker reach
  PostgreSQL only; no new edge. `akr-worker` and `akr-api` link `sqlproof`, not any driver.
- **Readiness:** `akr-ingestd` `/readyz` adds `mapping_sampler` (the job queue of `ingest_mapping_sample` is claimable),
  and akr-api `/readyz` adds `canon_catalogue` (the catalogue file loaded and equal to the migrated tables, section 3.8).
- **Metrics:** `akr_mapping_runs{kind,status}` (counter), `akr_mapping_run_seconds` (histogram), `akr_drift_open{kind,
  severity}` (gauge), `akr_canon_rows_written_total{entity}` (counter, from `canon.written`), `akr_canon_conflicts_total
  {entity,kind}` (counter), `akr_card_guard_hits_total{stage}` (counter; never a value). Alert rules: a `pause` drift open
  for more than 24 hours; a `mapping_run` failed with `source_refused`.

### 4.2 Existing entity, schema and route updates required by this phase

- **Phase 00:** no new role or prefix; the `ingest_` prefix covers the sampler; `canon` schema created with the first
  migration of this phase (`CREATE SCHEMA IF NOT EXISTS`); phase 00's no-`timestamp` check passes because
  `source_local_ts` is `TEXT`.
- **Phase 01:** no new step-up action class. Approvals use the existing `approval` class through the phase 04 workflow;
  the preview and the view-DDL pack need no step-up (no personal value leaves the sampler, section 3.6 step 7).
- **Phase 02:** evidence owner kind `mapping_run` registered (for `impact_analysis` and `test_run`); the phase 02
  `PartitionRegistry` gains the ten canon entries; `audit.partition_ahead_months` applies to them.
- **Phase 03:** Cedar bundle file `71_mapping.cedar` and the entities of section 3.10; the `ChangeKind` family `mapping`
  with kinds `mapping` and `code_map`; `schema.cedarschema` gains the six entities.
- **Phase 04:** change targets `mapping` and `code_map` registered in `ChangeTargetRegistry` (section 3.9); the existing
  `cr_mapping` route covers both (phase 04 section 1, route rule `object_kind` `mapping`, `code_map`); the `test_run`
  evidence resolver gains a `mapping_run` branch and `impact_analysis` is registered (phase 04 lists it as resolving from
  phase 10).
- **Phase 05:** `SelectProof`, `ProofScope` and `ProvenSelect` move from `crates/sources` to the new pure crate
  `crates/sqlproof` (section 2, DR-85); `crates/sources` re-exports them, so phase 05 call sites are unchanged. The
  statement grammar gains the keyset form of section 3.2 step 12 (a row-value comparison over bound parameters only) and
  `FETCH FIRST :n ROWS ONLY`; both are still single-object, single-statement `SELECT`s. `discovery_seal_report` enqueues
  `drift_scan` in its transaction.
- **Phase 05 adapter packs:** `discovery.json` may carry `code_suggestions` (a CBS code and list to a canonical code, for
  the prefill of section 3.4) and `deny_columns` is read by the compiler (section 3.2 step 4); both are data, no code.
- **Phase 06:** readiness reason state `unmapped` added (section 3.8); the grader's mapping input is the compiled artefact
  of the published version (section 3.8). Phase 06's `scn.readiness_grades` columns are unchanged.
- **Phase 08:** the screen catalogue `screens.ts` gains T3 (section 5); the phase 08 `E2` and `T1` specs are unchanged.
- **Phase 09:** `ref.countries`, `ref.currencies`, `ref.governorates` are the foreign-key targets of section 1; phase 09's
  reference tables must exist before this phase's migrations run (group 9 precedes group 10, DATA-MODEL s.23).
- **Planning docs aligned in the same change:** API.md 3.6 (add `POST /code-maps`, `GET /mapping-runs/{id}`,
  `GET /attributes`, `POST /attributes`; the three replay routes move to phase 11; the `/validate`, `/test`, `/preview`
  and `/impact` paths unchanged); DATA-MODEL s.4 (`mapping_versions.revision`, `mapping_runs`, `code_maps.gate` and
  `source_list`, `code_map_entries.origin`, drift kind `removed_column`, `drift_events.severity`) and s.5 (the nullability
  and the `payment_link.message_received_at` column, `payment_message.status_history`, `facility.currency`); ARCHITECTURE 5.2
  step 3 and 5.3 (the canonical writer runs in akr-ingestd, DR-85); DSL.md B.8 (the sample form of the statement);
  roadmap row 10 (`mapping_runs`, the four jobs and the route count).

## 5. Frontend Impact

Screen **T3 Mapping workbench** (DESIGN-SYSTEM.md section 8, `/data/mapping`, STUDIO pattern, reduced shell): the editor
(schema-driven over `mapping.v1.json`, the field grid with transforms chosen from the closed library, never free text),
the test and validate panel at the end (counts, rule outcomes, shapes, the card guard count with its word "Card-like
value stopped" and no value), the preview panel (shape rows: source shape beside canonical shape, personal and financial
fields marked "shape only"), the impact summary (rows changed and monitored attributes changed), and the change-request
panel docked below (the computed class and volume effect as badges with words, the diff with old values struck through,
the evidence list). Tabs: **Code maps** (unmapped CBS codes first with the CBS label and the gate word; the pending
prefills marked "Not yet confirmed by the bank"; propose and verify actions, CR), **Drift** (open events by table; the
paused tables in a banner with the cause; resolve for DS), **Attributes** (registry by entity; define for DS). The
data-as-of stamp is shown where the test uses source rows. Editing, testing, proposing and resolving are DS only; CFG,
CHK, M, CH and IA see the same screen read-only, and CHK acts on the change request in the inbox (G10 and X3). Masked
preview runs are not listed for roles without `read_preview`; the screen shows no preview tab to them.

Generated-client hooks consumed (`make generate-api`): `useGetCanonicalCatalogue`, `useGetMappings`, `usePostMappings`,
`useGetMappingsId`, `usePostMappingsIdVersions`, `usePutMappingsIdVersionsV`, `usePostMappingsIdVersionsVValidate`,
`usePostMappingsIdVersionsVTest`, `usePostMappingsIdVersionsVPreview`, `useGetMappingsIdVersionsVImpact`, `useGetCodeMaps`,
`usePostCodeMaps`, `useGetCodeMapsIdEntries`, `usePostCodeMapsIdEntries`, `useGetDriftEvents`,
`usePostDriftEventsIdResolve`, `useGetSourcesIdViewDdlPack`, `useGetMappingRunsId` (polled every 2 seconds until final,
as E1 polls), `useGetAttributes`, `usePostAttributes`.

`screens.ts` entry (phase 08 section 3.5): `{ id: 'T3', route: '/data/mapping', pattern: 'STUDIO', shell: 'reduced',
available: 'P1', rail: { group: <the data group of phase 08's RailGroup>, order: <next free>, label_key: 'nav.dataMapping', icon: <mapping icon from IconName> }, roles: ['DS', 'CFG',
'CHK', 'M', 'CH', 'IA'] }`. The route-to-policy test of phase 08 checks that the mutating actions are hidden for every
role but DS.


## 6. Verification

1. `make migrate`: the four migrations apply after phase 09 (group 9) and phase 05; a second run is a no-op. Negative
   checks: `INSERT INTO canon.txn` for a booking date with no partition fails with SQLSTATE 23514 and the writer then
   creates it (`SELECT canon.create_partition('txn', 2026, 11)` returns `txn_2026_11`); `INSERT` into `canon.txn` with
   `dr_cr = 'X'` fails its CHECK; a `card` row whose `pan_masked` holds 16 unbroken digits fails `pan_masked` CHECK;
   two `published` versions of one family fail `uq_mapping_versions_published`; a `txn_event` with an unknown `txn_id`
   fails its foreign key; `akr_app` has no INSERT on any `canon` table and no DELETE on any table of this phase.
2. `cargo check` (`cargo sqlx prepare --check`); `cargo clippy --all-targets -- -D warnings`; `cargo deny check` (akr-api and
   akr-worker do not depend on `akr-sources`, and `sqlproof` has no driver crate).
3. `akr-ctl mapping lint tests/fixtures/mapping/ics_core.txn.v1.json` prints `ok` and the compiled statement hash; the
   same file with one unknown transform prints the `errors[]` with `/fields/<attr>/do/<i>`; `akr-ctl canon verify-catalogue`
   exits 0 against the migrated database and exits 1 after `ALTER TABLE canon.txn ADD COLUMN zz TEXT` (a catalogue gap).
4. `cargo test`, including:
   - transforms (`dsl::transform`): each transform's property tests (round trip, idempotence where defined, refusal on
     invalid input, never a guessed date); `digits_western` over the Arabic-Indic and Persian digit ranges; `iban_validate`
     on fixture IBANs (good, bad checksum, wrong length);
   - card guard: 13 to 19 digit runs with and without separators; Luhn-valid and invalid; issuer-range and exclusion
     fixtures; `MaskPan` output; no matched value appears in the scan result's `Debug` output;
   - compiler: each step of section 3.2 with one failing and one passing fixture; the keyset statement passes
     `SelectProof` in all four dialects; a statement that names a `customer_id` predicate is refused by the generator's
     own assertion (C-35);
   - drift: each row of the section 3.5 table on a pair of fixture profiles; `new_table` never pauses; a second `compare`
     with an open event does not open a duplicate (the partial unique index);
   - writer (against a test database): same hash unchanged; different hash on a fact is a conflict and no overwrite;
     a party change closes the current version (one row with `recorded_to` set) and inserts the next (one current row);
     a missing partition is created once under concurrent callers (the advisory lock);
   - catalogue: every key of `canonical-attributes.v1.json` names a column of `canonical-model.v1.json`
     (the phase 05 test of section 3.8);
   - the golden test: `mapping-golden.v1.json` (synthetic rows, no personal value) gives the expected canonical rows
     through `MappingExecutor`, and the same rows through the writer yield identical `row_hash` values.
5. Curl suite `tests/curl/phase10_mapping.sh` (dev users `ds@bank.test`, `cfg@bank.test`, `m@bank.test`, `chk@bank.test`,
   `a1@bank.test`; phase 05's source `CBS1` active with a completed profile run; phase 09 reference data loaded):
   - As DS `POST /api/v1/mappings` `{"source_id":"<CBS1>","target_entity":"txn"}` with `Idempotency-Key: map-0001` -> 201,
     `dsl_id: "cbs1.txn"`. The same key and body -> the stored 201 with `x-idempotent-replay: true`. The same key with a
     different body -> 422 `E_IDEMPOTENCY_REUSE`.
   - As A1 `GET /api/v1/mappings` -> 403 `E_SCOPE` (A1 holds no `config.author`); as CFG `POST /api/v1/mappings/<id>/versions/1/test`
     -> 403 `E_FORBIDDEN` (the scope is held; Cedar denies the CBS read to CFG).
   - As DS `PUT /api/v1/mappings/<id>/versions/1` without `If-Match` -> 428 `E_PRECONDITION_REQUIRED`; with a stale
     revision -> 409 `E_VERSION_CONFLICT` with the current revision in the body; with the current revision and a document
     that names an unapproved object -> 422 `E_OBJECT_NOT_APPROVED` listing the approved names.
   - As DS `POST /api/v1/mappings/<id>/versions/1/validate` on the valid fixture -> 200, `status: "testing"`, `compiled.catalogue_hash`
     present; on a fixture without `customer.party_id` mapped -> 422 `E_VALIDATION` with `E_REQUIRED_TARGET_UNMAPPED`.
   - As DS `POST /api/v1/mappings/<id>/versions/1/test` -> 202 with `Location`; the run polled to `completed`:
     `result.rows_read` at most 500, `result.card_guard` is a count and the response holds no digit run of 13 or more
     digits (a `grep -E '[0-9]{13,}'` over the body finds nothing).
   - As CFG `GET /api/v1/mapping-runs/<preview run id>` -> 404 `E_NOT_FOUND`, byte-identical to `GET /api/v1/mapping-runs/<unknown uuid>`
     (the masked case). As DS the same run -> 200 with `preview` rows whose personal fields hold `shape` strings and no value.
   - As DS `POST /api/v1/mappings/<id>/versions/1/preview` -> 202; the purge job later sets `preview` to null
     (`psql` as `akr_readonly`, `SELECT preview IS NULL FROM admin.mapping_runs WHERE id = ...`, after the purge runs in test
     with `mapping.preview_keep_hours = 0`).
   - As DS `POST /api/v1/code-maps` `{"kind":"txn_type","source_id":"<CBS1>","source_list":"TEXT_TAB","prefill":true}` -> 201;
     `GET /api/v1/code-maps/<id>/entries` lists the CBS codes unmapped first; a `kind: "currency"` prefill request leaves
     `origin = 'prefill'` rows at zero.
   - As DS `POST /api/v1/code-maps/<id>/entries` with one `Map` operation -> 202 with a change request of family `code_map`,
     class `minor`; as CHK approve it (step-up `approval`) -> the change `effective`, the entry `origin: "change_request"`,
     `verified_by_bank_at` set on a `Verify` operation only; as CFG the same POST -> 403 `E_FORBIDDEN`.
   - As DS `POST /api/v1/mappings/<id>/versions/1/validate` after a published version is edited -> 422 or 409 with
     `E_NOT_ALLOWED_IN_STATE` reason `not_draft`; as DS `PUT` on the published version -> 409 `E_NOT_ALLOWED_IN_STATE`.
   - As DS `GET /api/v1/drift-events?source_id=<CBS1>` -> 200; `POST /api/v1/drift-events/<id>/resolve` with a stale
     `If-Match` -> 409 `E_VERSION_CONFLICT`; with the current version of a `notify` event -> 200 `resolved_at` set; a
     `pause` event whose version is not published -> 409 `E_NOT_ALLOWED_IN_STATE` reason `version_not_published`.
   - As DS `GET /api/v1/sources/<CBS1>/view-ddl-pack` -> 200 with `application/json` summary; `?download=true` ->
     `application/sql`, one `CREATE VIEW` per approved object, no `WHERE` clause (a `grep -c WHERE` over the file is 0).
   - As DS `POST /api/v1/attributes` `{"entity":"party","name":"branch_note","type":"text","classification":"personal"}` ->
     201; the stored `value` of an attribute value written by the writer test is an `{"enc": ...}` envelope.
6. Swagger UI lists 20 new operations, each with `x-akr-action`; `make generate-api` twice gives
   zero diff; the frontend build has no import of a removed hook.
7. Audit: after the curl run `audit_log` holds `mapping.created`, `mapping.version_opened`, `mapping.version_updated` (with
   actor and reason), `mapping.version_validated`, `mapping_run.requested` and `.completed`, `code_map.created`,
   `code_map.prefilled`, `code_map.entry_proposed`, `code_map.entry_activated`, `drift.opened` (if the fixture drifts),
   `authz.denied` for the A1, CFG and CHK attempts, and the masked `deny` access event for the CFG preview request;
   `akr-ctl audit verify` passes.
8. The CBS side: the statement journal of phase 05 shows only single `SELECT` statements from `akr-ingestd` during the
   curl run, each with a `FETCH FIRST` bound, and none from `akr-api` or `akr-worker`; the read-only conformance suite of
   phase 05 passes unchanged.
9. Phases 00 to 09 tests still pass; the phase 05 conformance suite and the phase 06 coverage suite run against the
   same database.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), gap item (a): Cedar form corrected in place (tools/agent/fix-g1-cedar.py); lead-verified.

### 7.1 Cedar form (gap item (a))
The Cedar policies of this phase are corrected in place: a role test is `principal.roles.contains("X")` (several
roles: `principal.roles.containsAny([...])`) in a `when` clause, never `principal in Akr::Role::"X"`, which phase 03's
schema does not model (2 occurrences rewritten; the `@role` annotations are unchanged).

Gap pass 1 (2026-10-10), run 3a, from tools/agent/gap1-run3a.md; lead-verified (tools/agent/gap1-run3a-verdict.md).

### 7.2 Islamic contract types (phase 26)
Phase 26's migration widens the CHECK on `canon.product.islamic_contract_type` with `tawarruq`,
`diminishing_musharaka`, `wakala` and `wadiah`. The base list of this phase does not change.

### 7.3 The two entities of the Islamic pack (phase 26 section 3.2)
Phase 26 adds `islamic_deal` and `deal_party` at the end of `CanonicalEntity` (31 names) and of
`canonical-model.v1.json`, and registers the mapping families `<source>.islamic_deal` and `<source>.deal_party`.
Phase 26 creates both tables. A deal party is never a new subject kind.

### 7.4 Tombstone check in the canonical writer (phase 23 section 4.5 item 3)
From phase 23 on, before `CanonicalWriter` inserts a `canon.customer` or `canon.account` identity, it reads
`admin.retention_tombstones` for the identity's keyed hash (phase 23 section 3.5). A hit skips the row and increments
`akr_ingest_tombstoned_total`. Phase 23 creates the table and grants `akr_ingest` SELECT on it; the writer's other
statements do not change.

### 7.5 Batch indexes (phase 16)
Phase 16 migration `0056` creates `idx_canon_txn_batch` on `canon.txn (batch_id)` and `idx_canon_txn_event_batch` on
`canon.txn_event (batch_id)`. They are physical indexes only; the index list of this phase does not change.

### 7.6 MappingExecutor::map_payload (phase 11)
Phase 11 adds `MappingExecutor::map_payload`: the pure transforms of a version applied to one stored landing payload,
with the same result type as `map`, for replays and retries. `map` does not change.

### 7.7 Volume effect of a mapping change from its replay (phase 11, DR-92)
When a completed replay of the proposed version exists (phase 11), the `mapping` change target takes `volume_effect`
from it. The first publication of a family keeps `neutral`; without a replay the rule of the change-target table
applies. Per DR-360 the value is fixed when the request is submitted.

### 7.8 Address keys of canon.party_contact (DR-374)
The `address` object of a `canon.party_contact` row of type `address` holds at most three keys: `governorate` (a code
of `ref.governorates`, `IQ-XX`, phase 09), `district` and `city` (text as the CBS holds it). The full address stays in
`value_enc`. The mapping fills the keys; a key the source does not hold is absent, never invented. Phases 21 and 24
read these keys. `canonical-model.v1.json` documents them; no constraint changes.

### 7.9 The names crate starts here (DR-371)
This phase creates the pure crate `crates/names` (no IO, no database) with the fold and key tables of research file
22 and the functions behind `FoldArabic` and `SplitArabicName`. The transforms call the crate, so the columns
`light`, `folded` and `key` of `canon.party_name` come from it. Phase 14 extends the same crate (normaliser, forms,
parser, SQL twin, parity corpus) and stays the only place where the rules live. Screening recomputes its forms from
`raw` and never reads the stored columns; when phase 14 changes a table, the stored columns are refreshed by a mapping
replay (phase 11), not by a separate backfill.

### 7.10 T3 in screens.ts (DR-369)
The T3 entry of section 5 uses `rail.group: 'data'` (phase 08 `RailGroup`), the next free `order` in that group, and
the mapping icon of `IconName`.

Gap pass 1 (2026-10-10), run 4a: items carried from runs 3a and 3b and corrections found while checking phases 17 to 24 (tools/agent/gap1-run4a-verdict.md); lead-verified.

### 7.11 Drift notices stay on the screens (phase 22 section 7+)
Section 3.5 says notifications arrive with phase 22. Phase 22 defines no drift topic in this release: open drift
stays on T3 and the cockpit, which are the notice. No outbox row is written for drift.

Gap pass 1 (2026-10-10), run 4b: items carried from run 4a and corrections found while checking phases 25 to 31 (tools/agent/gap1-run4b-verdict.md); lead-verified.

### 7.12 Canonical payment status values (DR-389)
`canon.payment_message.status` takes the canonical values `received`, `released`, `rejected`, `returned` and
`cancelled`, mapped from the CBS value through the `status` code map of this phase (kind `status` already exists in
`admin.code_maps`; an unmapped value follows the map's `gate`). The column of migration section 1 reads
`status TEXT NOT NULL CHECK (status IN ('received', 'released', 'rejected', 'returned', 'cancelled', 'unknown'))` (`unknown` is what the `dq_issue` gate of the `status` code map writes for an unmapped value, section 3.4 step 5 and DSL.md B.7; without it the CHECK would fail the batch; phase 29 keeps `unknown` under control); the values
appended to `status_history` use the same list. Phase 29's release control reads this column (phase 29 section 7+).

Adversarial pass (2026-10-10), run 3 (watermarks, restores, failover, late batches, source pauses), from tools/agent/adv-run3.md; lead-verified (tools/agent/adv-run3-verdict.md).

### 7.13 Versions with an equal or older business date (ADV-3-06, HIGH; DR-409)
Section 3.7 step 4 closed the current version only when its `valid_from` was earlier. A second change of a party on the
same business date (two units in one day), or a changed row met again in the re-read window with an older business
date, then inserted a second current row; `uq_<table>_current` refused it, the batch failed as `writer_failed` on every
attempt and the table stopped. Step 4 therefore sets the new version's `valid_from = GREATEST(<the batch's business date of step 4, phase 11's admin.batches.business_to>,
<the current version's valid_from>)`. When it equals the current version's `valid_from`, `close_versions` sets only
`recorded_to` on the current row (its valid range is not shortened); when it is later, step 4 runs as written. The raw
row is unchanged (its batch row holds the business date), and `recorded_from` records when AKR learned the value. Test: two changes of one
party on one business date give two versions with the same `valid_from`, the first closed in recorded time.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.14 Name flags for bidirectional and zero-width characters (V-08-08-b, MEDIUM; DR-413)
UX-20's flagging as data quality is carried by the `quality_flags` of N-12 (section 3.3), with no new column and no register kind. From phase 11 on the canonical writer sets `bidi_control` on a `party_name` row whose `raw` holds a character of the `bidi` set of phase 08 (section 7+, the code points of scanControls) and `zero_width` on a row whose `raw` holds one of its `zero_width` set; the same two values are set in `quality_flags` of a `counterparty` or `payment_message_party` row whose name holds one. The raw value is stored unchanged. As N-12 says, a flag never lowers a match score. Phase 08's pointer "flagging as data quality arrives with ingestion in phase 11" means this.

### 7.15 Validate stores the artefact only while the revision holds (V-10-01-a, MEDIUM; DR-413)
Section 3.1 step 4: `validate` reads `revision` with the version at its start. The store of the compiled artefact and the move `draft` -> `testing` are one compare-and-set on that revision, as `replace_draft` is in step 3; when the revision has moved, nothing is stored and the answer is 409 `E_VERSION_CONFLICT`. `MappingVersionRepository::store_compiled` takes `expected_revision: i32` after `id` and returns `Result<Option<MappingVersion>, RepoError>` (None when the revision moved). In section 6 item 5 a validate of a version that is not `draft` is 409 `E_NOT_ALLOWED_IN_STATE` (`not_draft`) only; "422 or 409" reads "409".

### 7.16 MappingExecutor::map and its row types (V-10-05-a, V-10-07-a, MEDIUM; DR-413)
`MappingExecutor::map(version: &MappingVersion, rows: &[SourceRow]) -> MappedBatch` is the batch entry point that phase 11 calls. It applies `MappingExecutor::apply` (section 3.6 step 4) to each row in order. `SourceRow { source_pk: String, extracted_at: DateTime<Utc>, row_hash: RowHash, payload: Value }` is one landed or sampled row.
`MappedBatch { rows: Vec<MappedRow>, rejected: Vec<Rejected>, unmapped_codes: u64 }`. `MappedRow { entity: CanonicalEntity, source_pk: String, extracted_at: DateTime<Utc>, row_hash: RowHash, values: Vec<(String, Value)> }` carries the catalogue columns of the entity. `Rejected { source_pk: String, reason: RowFailure, field: Option<String> }` names a field, never a value.
`RowFailure` is a closed enum whose codes phase 11 writes as quarantine reasons: `transform_failed` (a `TransformFailure` of section 3.3), `required_missing` (a required attribute with no value, section 3.8), `unmapped_currency` (a currency not in `ref.currencies`), `card_guard` (section 3.3), `unmapped_code` (a row that the gate of section 3.4 step 5 holds for a `txn_type` value with no entry; its `field` is `txn_type`) and `rule_rejected` (a validation rule with `on_fail = quarantine`). `unmapped_codes` of `MappedBatch` counts the unknown values; the rows held by the gate are in `rejected`.
`MappingExecutor::map_payload(version: &MappingVersion, row: &SourceRow) -> MappedBatch` (section 7.6) runs the same code on one stored landing row.

### 7.17 Row types and three writer rules (V-10-10-a, V-10-11-a, V-10-12-a, MEDIUM; DR-413)
`CurrentRow` (returned by `current_by_source_key`) holds `id` and `row_hash`, and for a bi-temporal entity also `recorded_from` and `valid_from` (a fact has neither column and is read by its source key with no `recorded_to` filter); for `canon.payment_message` also `status` and the hashes kept in `status_history`, and for `canon.account_hold` also `end_at`; sections 7.26 and 7.27 add `capture_source` for `canon.party_name`, `granted_on` for `canon.facility`, and `paid_on` and `paid_amount` for `canon.facility_instalments`. `CanonRow` (taken by `insert_rows`) holds the catalogue columns of the entity, `id` from `admin.xref` and the lineage columns; a bi-temporal entity adds `valid_from` and `recorded_from`.
`close_versions`: the third element of each tuple is `Option<NaiveDate>`. `Some(d)` sets `recorded_to` and `valid_to = d`; `None` sets only `recorded_to` and leaves `valid_to` null (the equal-date case of section 7.13).
Section 3.7 step 1: a group `(entity, source_key)` with more than one row keeps its newest row (by `extracted_at`, then landing order). The older rows of the group are not written and are counted with `unchanged`; they stay in raw. So one batch writes one version per id, and the one clock per batch of step 4 stays unique per id.
Section 3.7 step 8: `xref_linked` is the number of `admin.xref` rows that step 2 inserted.

### 7.18 Status change of a payment message (V-10-09-a, HIGH; DR-413)
The fact rule of section 3.7 step 3 left no way to write a status change, though the table keeps `status_history` and `akr_ingest` may update the two columns. For `canon.payment_message` step 3 reads: a row whose hash equals the stored `row_hash`, or the `row_hash` of any `status_history` entry, is `unchanged`. Otherwise, when the mapped `status` differs from the stored one, the writer calls `CanonRepository::append_status(tx, id, received_at, status, entry)`, which sets `status` and appends `entry = {status, at: extracted_at, row_hash}` to `status_history`; no conflict is returned. Otherwise the fact rule applies (`RowConflict`). A change of another column that arrives in the same source version as a status change is not written to canon; the raw row keeps it (DR-37).
`CanonRepository` gains `async fn append_status(&self, tx: &mut Tx, id: Uuid, received_at: DateTime<Utc>, status: &str, entry: &Value) -> Result<u64, RepoError>;`.

### 7.19 The end of an account hold (V-10-02-b, MEDIUM; DR-413)
A hold that the source releases comes back as a changed row with `end_at` set. For `canon.account_hold` the fact rule of section 3.7 step 3 has one exception: when the stored `end_at` is null and the mapped `end_at` is set, the writer calls `CanonRepository::end_hold(tx, id, end_at, row_hash)`, which sets `end_at` and `row_hash`; no conflict is returned. Any other difference is a `RowConflict`. Migration `0030` (section 1.2) gains `GRANT UPDATE (end_at, row_hash) ON canon.account_hold TO akr_ingest;`. Phase 20 reads the open and the ended hold from this table.

### 7.20 A payment link names its two ends (V-10-02-a, MEDIUM; DR-413)
Section 3.7 step 2 for `canon.payment_link`: the mapped row carries the source key of its payment message and of its transaction. Each is resolved through `admin.xref` with one `get_many` call per entity, and the canonical ids found are written as `message_id` and `txn_id`. A link end with no `admin.xref` row gets no new id: the link is not written, a `RowConflict` of kind `parent_missing` is returned, which phase 11 records as a `row_conflict` quarantine row with reason `parent_missing`, and the trailing re-read window (DR-37) meets the link again when its end has landed. A link taken from a source that holds it (phase 12 section 7+) is written with `rule = 'declared'` and `confidence = 1.0000`; match rules are P2 (DR-394).

### 7.21 Code-map changes: class, refusals and the revert event (V-10-03-a, V-10-03-b, MEDIUM; DR-413)
Section 3.4 step 4: a change whose operations are `Verify` or `Map` only is `minor` (a prefilled entry is unmapped under the gate until it is verified, so `Verify` maps a previously unmapped code, the DSL.md D.3 test); a change with any `Reclassify` is `material`. An operation that cannot apply (a `Map` of a code that has an effective entry; a `Reclassify` with no effective entry or an unmatched `supersedes_valid_from`; a `Verify` of an entry that does not exist) is refused at propose with 422 `E_VALIDATION` at `/ops/<i>`, reason `operation_invalid`.
Section 3.9, `code_map` Revert: the revert closes the `Map` entries of the change (`valid_to`) and writes one audit event per closed entry, `code_map.entry_reverted {code_map, source_code, canonical_code, valid_to}`, in the transaction of the revert; the section 3.10 audit table gains that row. Phase 04's `change_request.reverted` is written as well.

### 7.22 cdd_level stays null (V-10-04-a, LOW; DR-413)
`customer.cdd_level` has no mapping path in this plan: it is not a required attribute (section 3.8), none of the ten code-map kinds maps it, and DATA-MODEL keeps it out of every rating and scenario input. The column stays null, and the CHECK of section 1.2 is reached by no writer. The KYC level a customer holds in this product is phase 15's.

### 7.23 The name columns and their transforms (V-10-08-a, MEDIUM; DR-413)
Section 3.3: `raw` is the source value as landed. `folded` is `FoldArabic` run through all of its steps (research file 22 section 6.4, N1 to N11). `light` is `FoldArabic` stopped after step N4, its argument `stage: Light` (the other value is `Full`). `key` is written by `SplitArabicName` (the consonant key of research file 22 section 7.4). `FoldArabic` joins the `Transform` list of section 2.

### 7.24 The party lock in the writer (V-10-06-a, MEDIUM; DR-413)
Section 7.4 (from phase 23 on), extended: before the tombstone read of a `canon.customer` row the writer takes `pg_advisory_xact_lock(hashtext('party:' || party_id))` (phase 23 section 3.15 step 1) for that row's party; before the tombstone read of a `canon.account` row it takes the same lock for the party of the customer the account names, when that customer has a row in canon. Locks are taken in ascending party id and held to the commit of the batch transaction, so the tombstone read and the insert cannot interleave with a party purge. An account whose customer has no canon row takes no party lock: a purge of that party has either committed (its tombstone is visible) or never ran.

### 7.25 The key `internal_ref` of `message_refs` (V-29-04-d, MEDIUM; DR-413)
From phase 29 on, `canon.payment_message.message_refs` has one named key: `internal_ref`, a string that holds the payment's reference in the gateway database. The mapping of a payment source fills it when the source has such a column. The release control of phase 29 (section 3.14 step 3) reads it for a payment without a UETR. The column stays an open object, and no CHECK changes.

Owner decisions (2026-10-10): the product owner overturned two lead decisions that touch this phase (the label of a Latin name, phase 08 section 7.9; the repayment event of schedule tables, phase 26); lead-written, checked in the owner-decision round (tools/agent/owner-verdict.md).

### 7.26 `capture_source` of a name (owner decision 9; DR-420)
The owner's decision: the four source labels of DESIGN-SYSTEM 7.6 (card, passport, typed, generated) are kept, and the mapping says which one applies to each name. DR-422 locks the correction of the check round of 2026-10-11: the label is compared as a mapped value, also when the hash of the landed row is equal (item 5).

1. **Column.** `canon.party_name` gains one column after `source_field` (written into the table DDL of section 1.2 in the same change; no new table):

```sql
    capture_source      TEXT NOT NULL DEFAULT 'typed' CHECK (capture_source IN ('card', 'passport', 'typed', 'generated')),
```

   The `internal` classification row of `party_name` (section 1.2) gains `capture_source`. Indexes and the retention row are unchanged; one column grant is added (item 5).
2. **Meaning.** `card`: the bank copied the name from the national card. `passport`: the bank copied it from the passport. `typed`: staff keyed it with no document named as its source. `generated`: a program of the bank produced it from another name, for example a Latin form made from the Arabic name. The value says how the bank captured the name; the product does not check it against a document.
3. **Mapping.** `capture_source` is a column of the entity `party_name` in `canonical-model.v1.json`: not `required`, not `monitored`. A mapping of `party_name` supplies it as any field of DSL.md B.3: a literal (`"capture_source": { "expr": "'passport'" }`) when every name of the source object has one source, or an `expr` over the source row when a column of the bank tells them apart. A mapping that leaves it out writes `typed` (item 5), which is what phase 08 showed for every Latin name before this decision.
4. **Compiler** (section 3.2 step 5): a literal `expr` for `capture_source` that is not one of the four values is refused with `E_VALIDATION`, path `/fields/capture_source`.
5. **Executor and writer** (section 3.6 step 4 and section 3.7). The compiler adds the validation rule `enum(capture_source, [card, passport, typed, generated])` with `on_fail: dq_issue` to every `party_name` mapping that supplies the field. This block's rule for a value that fails it: the executor puts `typed` in the mapped row and counts the data-quality issue. (DSL.md B.5 alone says only that the row loads and the count rises; the failing value would then meet the column CHECK.) A mapped row with no value for the field, because the mapping leaves it out or its `expr` gives null, is written with `typed`: the writer always supplies the column and does not rely on the column default. `row_hash` is the hash of the landed source row (phase 11 section 3.2 step 2), so a mapping version that changes only this value leaves the hash as it is and step 3 writes nothing. The label is therefore tested on the mapped value: when the hash is equal and the stored `capture_source` of the current row differs from the mapped one, the writer calls `CanonRepository::set_capture_source(tx, id, capture_source)`, which sets that one column; when the hash differs, the row is replaced as step 6 says and the new row carries the mapped value. `CurrentRow` (section 7.17) holds `capture_source` for `canon.party_name`. `CanonRepository` gains `async fn set_capture_source(&self, tx: &mut Tx, id: Uuid, capture_source: &str) -> Result<u64, RepoError>;`, and migration `0030` (section 1.2) gains `GRANT UPDATE (capture_source) ON canon.party_name TO akr_ingest;`. A name already stored takes a new label when its row is mapped again. A source row that changed lands again, and its name row is replaced with the mapped label. A source row that did not change is not landed again (phase 11 section 3.2 step 2), so a mapping edit alone reaches it only in the apply of a mapping replay (phase 11 section 3.8).
6. **Readers.** A response that returns rows of `canon.party_name` returns `capture_source` with each row (phase 18 `PartyPage`, its section 7.28). A response that returns one display string (`display_name` of phases 15 and 19) is unchanged. The `Name` component shows the label (phase 08 section 7.10).
7. **Audit, Cedar, tipping-off.** No new route, event or action: the value is set by the mapping, and a mapping changes only through its change path (`mapping.*` events and the `mapping` change target of section 3.9). The value says nothing about a case.
8. **Tests** (added to section 6): a `party_name` mapping with `"capture_source": { "expr": "'passport'" }` writes rows with `passport`; the same mapping without the field writes `typed`; the literal `'scan'` fails `validate` with `E_VALIDATION` at `/fields/capture_source`; a row whose `expr` gives null writes `typed`; a row whose `expr` reads `scan` from a source column writes `typed` and counts one data-quality issue; a stored name with `typed`, mapped again from the same landed row by a version that gives `passport`, has `passport` afterwards and no second row; `akr-ctl canon verify-catalogue` passes with the new column.

### 7.27 `facility_instalment` and `facility.granted_on` in the writer (owner decision 6; DR-419)
Phase 26 section 7.13 adds the table `canon.facility_instalments` and the column `canon.facility.granted_on` (migration `0083`), so that a schedule table gives the repayment event SCN-ISL-07 needs. DR-422 locks the corrections of the check round of 2026-10-11: the payment columns and the grant date are compared as mapped values, and the index on `(facility_id, seq)` is not unique. This phase's parts:

1. **Entity.** Phase 26 adds `facility_instalment` at the end of `CanonicalEntity` (32 names, after the two of section 7.3) and of `canonical-model.v1.json`, and registers the mapping family `<source>.facility_instalment`. The catalogue entry of the entity names its table, `canon.facility_instalments` (an entry holds the table, the columns, the classification and the nullability: phase 26 section 3.2 item 2), as the entry of `islamic_deal` names `canon.islamic_deals`. In the dependency order of section 3.7 step 1 it follows `facility`.
2. **Parent key** (section 3.7 step 2). The mapped row carries the source key of its facility in the field `facility_id`. The writer resolves it through `admin.xref` (entity `facility`) with one `get_many` call, as section 7.20 does for a payment link. A row whose facility has no `admin.xref` row is not written: a `RowConflict` of kind `parent_missing` is returned, and phase 11 records it as section 7.20 says (reason `parent_missing`, phase 11 section 3.2 step 5).
3. **Fact rule** (section 3.7 steps 3 and 5). `facility_instalment` is a fact. Its insert target is `uq_facility_instalments_source`, the only unique constraint of the table beside the primary key, so a second source key for an instalment number the facility already holds is inserted like any new row. One exception to the fact rule, like the end of a hold (section 7.19): an instalment is open while its stored `paid_on` is null. For an open instalment whose mapped `paid_on` or `paid_amount` differs from the stored value (`IS DISTINCT FROM`), the writer calls `CanonRepository::pay_instalment(tx, id, paid_on, paid_amount, row_hash)`, which sets the three columns where `paid_on IS NULL`; `paid_on` stays null for a part payment. No conflict is returned, and the row counts as written for this batch: a difference in `seq`, `due_on`, `amount` or `currency` that arrives in the same version is not written and not reported. The test is on the mapped values, not on the hash, so it also runs when the hash is equal (the landed row is the same and the mapping changed, as in the apply of phase 11 section 3.8). In every other case step 3 stands: a different hash on a paid instalment, or on an open one whose two payment columns did not move, is the `RowConflict` of a fact. `CurrentRow` holds `paid_on` and `paid_amount` for this entity.
4. **`granted_on` of a facility.** `facility` stays a dimension. One exception to the dimension rule of step 3: when the stored `granted_on` is null and the mapped one is set, the writer calls `CanonRepository::set_granted_on(tx, id, granted_on, row_hash)`, which sets the two columns where `granted_on IS NULL`. No conflict is returned, and the row counts as written for this batch: a difference in another column that arrives in the same version is not written and not reported. As in item 3 the test is on the mapped value, not on the hash, so a mapping version that adds the field reaches the facilities already stored when they are mapped again. In every other case step 3 stands, and a stored `granted_on` is never changed. `CurrentRow` holds `granted_on` for `canon.facility`. The grants for both updates are in phase 26's migration `0083`.
5. **Repository.** `CanonRepository` gains `async fn pay_instalment(&self, tx: &mut Tx, id: Uuid, paid_on: Option<NaiveDate>, paid_amount: Option<Decimal>, row_hash: &[u8]) -> Result<u64, RepoError>;` and `async fn set_granted_on(&self, tx: &mut Tx, id: Uuid, granted_on: NaiveDate, row_hash: &[u8]) -> Result<u64, RepoError>;`.
6. **Catalogue.** Phase 26 adds `facility.granted_on` and the columns of `facility_instalment` to `canonical-model.v1.json` in its own change, with the flags of its section 7.13 item 2, as it adds the two entities of section 7.3. `akr-ctl canon verify-catalogue` therefore passes before phase 26 (neither the file nor the database has them) and after it (both have them).
7. **Tests** (added to section 6, run from phase 26 on): an instalment row whose facility key is unknown returns `parent_missing` and writes nothing; the same row after the facility has landed is written; a second version with `paid_on` set updates the row and returns no conflict; a second version with only `paid_amount` set (a part payment) updates `paid_amount` and leaves `paid_on` null; a second version of an open instalment with only a different `amount` returns a `RowConflict`, and so does any second version of a paid instalment; the same landed row mapped by a version that now gives `paid_on` updates the row; a second source key for an instalment number the facility already holds is inserted; a facility row that arrives again with `granted_on` set updates only `granted_on` and `row_hash`, and a later version with another `granted_on` leaves the stored date and returns the dimension `RowConflict`.

## Decisions

Recorded in research.md: DR-85 (writer in akr-ingestd, `sqlproof` crate, synchronous validation, `ingest_` sample job),
DR-86 (shapes only, masked previews, DS-only authoring and CBS reads), DR-87 (canon physical design: bi-temporal keys
without parent foreign keys, partition factory, `source_local_ts` TEXT, retention clocks), DR-88 (drift severities,
unverified prefills, volume effect of the first publication and of later changes, replay routes in phase 11). The Cedar
file `71_mapping.cedar` follows the phase 03 range convention.

## Open doubts (for gap pass 1 and the bank)

Resolved by the lead: `source_local_ts` as TEXT (DR-87, DATA-MODEL s.0 amended); the `SelectProof` move (DR-85, phase 05
amended by section 4.2 of this file); the writer's binary (DR-85, ARCHITECTURE amended); the volume effect of mapping
changes before phase 11 (DR-88: first publication neutral); `admin.readiness_grades` in DSL.md (corrected to
`scn.readiness_grades`). Still open:

1. **`monitored` and `required` flags** of `canonical-model.v1.json` are initial values; phases 13 to 16 review them
   against their DSL needs.
2. **`reverses_id` and `corrected_by_id` are nullable** because DATA-MODEL s.5 lists them without `null` while its text
   says they stay null until a source column is confirmed; `cancel_reason_code` is required for `cancelled` rows.
3. **`pep_status` values** are `unknown`, `not_pep`, `pep`; phase 15 may add a lead state for the political-class leads
   of DATA-MODEL s.5.
4. **Bank-operator rows** (`staff.role = 'system'`) are needed for `txn.operator_id NOT NULL`; the list is the bank's
   (DR-42), so until it is confirmed `system_generated` stays null.
5. **Bi-temporal versions and the writer's clock:** `recorded_from` is one `clock_timestamp()` per batch; the writer
   serialises per entity id with an advisory lock, which phase 11's load loop must honour.

## What This Phase Does NOT Touch

- No production load: no watermarks, batches, quarantine items, raw landing partitions, settled-time or late-data
  handling, freshness classes and no `ingest_` load loop (phase 11). The `CanonicalWriter` is a library here; phase 11
  calls it.
- No replay over the raw landing layer and no `POST /mappings/{id}/replays` or apply routes (phase 11). A mapping change
  that moves a monitored value waits for that replay (section 3.9).
- No `dq_issues`, reconciliation or data-quality queue (phase 12): conflicts are returned by the writer and recorded there.
- No scenario, feature, screening or rating use of canon attributes (phases 13 to 16); the `monitored` flags are the initial
  set only.
- No Islamic tables beyond the `islamic_book` flag and the Islamic product type (phase 26).
- No notifications (phase 22): drift shows on T3 and in the API, not in an inbox or a notification.
- No CBS connection change: no new grant, no new source tier, no CT3 capture (phase 05 and phase 11).
- No new step-up action class, no new problem code, no change to the phase 01 closed enum.
- No retention job: the retention engine drops canon partitions (phase 23); this phase only assigns classes.
- No parties merge or split, no survivorship across sources beyond `source_priority` and `most_recent` (phase 12, C-53).
- No `longest_verified` survivorship, no `t24_*` or `landing_file` formats, no ISO 20022 or MT parser (phases 11 and
  later): the compiler refuses them with a named reason.
