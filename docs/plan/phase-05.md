# Phase 05: Sources, Read-only Connection & Discovery

**Goal:** `akr-ingestd` connects to a bank's core banking database without ever being able to change it, and proves
it: a registered source whose credentials never pass through `akr-api`, a grant check on every connect that judges
the session's effective privileges against an allow-list per engine and refuses anything else by name (the discovery
account that held DBA is refused, DR-40), every statement generated, parsed and proven a single bounded `SELECT` over
an approved object inside the engine's read-only transaction mode, a signed load budget with a kill switch, and the
"paused by bank" state; on top of it, read-only discovery in two steps (a catalogue from the engine dictionary, then
profiles of the objects the bank approved) that ranks tables by liveness, counts rows without optimizer statistics,
profiles columns without letting personal values out, captures the CBS's own code lists, grades field availability
against the canonical attributes, and exports the integration profile that phase 06 turns into the CBI coverage gap
report.

**Dependencies:** Phase 04
**Macro-phase:** P0
**Complexity:** XL (21 routes, five engine adapters, the grant policy per engine and the read-only conformance suite)
**Binaries touched:** akr-ingestd (the `sources` crate with the adapters; jobs `discovery_probe`, `discovery_run`,
`ingest_source_health`; `LISTEN akr_source`) | akr-api (routers `sources`, `discovery`; change targets
`load_budget` and `source_access`) | akr-worker (job `discovery_seal_report`) | `akr-ctl` (`source credential set`,
`source credential check`, `source disable`, `adapter verify`, `fixtures cbs`). akr-assess unchanged
**Requirements carried:** C-01 (tiers CT1-CT4 behind one pipeline; CT1, CT2 and CT4 as a bank landing database in
this phase), C-02 (no AKR object in the CBS; bank-created views listed in the footprint register), C-03 (read-only
attestation on every connect, fail closed, bank-signed exceptions, the signed report in the evidence vault), C-04
(read-only levels 1-3 stated and proved per source), C-06 (poll-first behind the change-source interface; `stream`
and `file` arrive in phase 11), C-07 (discovery checklist and engine profile: the integration profile), C-08
(read-consistency policy per engine), C-09 (no dependence on options the bank has not bought), C-10 (character-set
round trip at install), C-13 (signed load budget: sessions, statement time, rows per fetch and per minute, windows,
end-of-day blackout, lag ceiling, slow-down), C-15 (paused by bank with bounded back-off; the coverage gap and
backfill read the pause record in phases 11 and 12), C-30 (least privilege on named objects, the per-engine minimum
grants in the integration profile), C-35 (no read of the CBS by customer, account or case: statements carry no
predicate except their bound), C-36 (card and authentication columns never profiled), C-38 (adapter packs as signed
release artifacts; the discovery part of the ICS BANKS pack), C-54 (parsed single-statement read-only SELECTs over
approved objects, no ad hoc SQL), C-61 (Db2 and Sybase through ODBC with bank-supplied drivers), C-62 (ICS BANKS on
Oracle first), N-12 (name data-quality profile at onboarding), TS-04 (one `SourceReader` port, feature-gated
drivers), TS-07 (driver conformance suite in CI with a documented rig for Db2 and Sybase)
- Gap pass 1 (coverage), implemented here: TS-05 (generated SQL proved by `sqlparser` as one read-only SELECT over approved objects), TS-06 (blocking drivers on a pool sized by the load budget); VD-05 (Db2 and Sybase through ODBC), VD-11 (PostgreSQL, SQL Server, Oracle and ODBC adapters; Oracle not assumed); VC-85 (AKR-TMS never writes to the CBS; tasks, approvals and evidence live in AKR-TMS).
- Gap pass 1 (coverage), partly implemented: C-31 (the credential in OpenBao KV, read only by akr-ingestd), C-32 (TLS trust against the bank CA and the `tls_failed` check), C-40 (the catalogue run: tables, columns, partitions, character set, grants); VD-01 (ICS BANKS on Oracle as the first adapter pack), VD-02 (this phase reads the inputs to each mapping, the bank's dictionary, column comments, value lists and candidate columns; the mapping itself is written by its author in the phase 10 workbench and published under `cr_mapping`), VD-06 (the character-set round trip per source), VD-33 (engine facts from read-only discovery); VC-86 (the read-only CBS connection and the egress allow-list).
- Gap pass 1 (coverage), not carried: C-34 (native audit on the read account is the bank DBA's and SIEM's work); SP-82 (the CISO's exception register is the bank's, DR-396); VA-33 (vendor implementation timeline, DR-402); VD-07 (the bank's tool inventory meeting; bank process, DR-403).

## 1. SQLx Migration Changes

Three migrations, `backend/migrations/0017` to `0019` (canonical chain in roadmap.md, DR-23). Conventions as in
phases 00 to 04. All tables live in `admin`; nothing in `raw` yet. `watermarks`, `batches` and `quarantine_items`,
listed under phase 05 in earlier drafts, move to phase 11, where ingestion first writes them (DR-63); `source_pauses`
(the record a coverage gap and a backfill are built from), `discovery_runs` and `discovery_code_entries` are added.
The `sources` table follows DATA-MODEL.md section 3 with the columns this phase needs (`code`, `cbs_schemas`,
`access_exceptions`, `hints`, the state columns).

```sql
-- migration 0017_sources_budgets_grants.sql
-- Phase 05. Registered sources, grant checks, load budgets and pause records (ARCHITECTURE.md s.5.1, DR-40,
-- DR-59 to DR-62).

DO $$ BEGIN
    CREATE TYPE admin.source_state AS ENUM
        ('registered', 'refused', 'active', 'paused', 'paused_by_bank', 'halted', 'disabled');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS admin.sources (
    id                     UUID PRIMARY KEY,
    code                   TEXT NOT NULL CHECK (code ~ '^[a-z][a-z0-9_]{1,31}$'),   -- DSL `source.source`
    name                   JSONB NOT NULL CHECK (name ? 'ar' AND name ? 'en'),
    engine                 TEXT NOT NULL CHECK (engine IN ('postgres', 'mssql', 'oracle', 'db2', 'sybase')),
    tier                   TEXT NOT NULL CHECK (tier IN ('CT1', 'CT2', 'CT3', 'CT4')),
    -- Proved, never declared: 0 until a check passes; 1 data plane, 2 footprint-free, 3 load-free (C-04).
    readonly_level         SMALLINT NOT NULL DEFAULT 0 CHECK (readonly_level BETWEEN 0 AND 3),
    readonly_proof         JSONB NOT NULL DEFAULT '{}'::jsonb,     -- from the latest passing check (section 3.3)
    dsn_ref                TEXT NOT NULL,                          -- OpenBao KV path `sources/<id>` (DR-59)
    cbs_product            TEXT,                                   -- e.g. 'ICS BANKS'; NULL until the bank confirms
    adapter_pack           TEXT CHECK (adapter_pack ~ '^[a-z][a-z0-9_]{1,31}@[0-9]+\.[0-9]+\.[0-9]+$'),
    engine_version         TEXT,                                   -- observed at probe, e.g. 'Oracle 11.2.0.4.0'
    cbs_schemas            TEXT[] NOT NULL DEFAULT '{}',           -- owners that hold the CBS (catalogue scope)
    -- The footprint register (C-02, C-30): exact OWNER.NAME entries the bank approved, never patterns; empty =
    -- every data object refused (DSL.md source.object). Changed only by a `source_access` change request.
    approved_read_objects  JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(approved_read_objects) = 'array'),
    access_exceptions      JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(access_exceptions) = 'array'),
    access_version         INT NOT NULL DEFAULT 0 CHECK (access_version >= 0),
    hints                  JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(hints) = 'object'),
    state                  admin.source_state NOT NULL DEFAULT 'registered',
    state_reason           TEXT,
    state_changed_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_contact_at        TIMESTAMPTZ,
    last_check_id          UUID,                                   -- FK below (table created after this one)
    created_by             UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at             TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at             TIMESTAMPTZ NOT NULL DEFAULT now(),
    version                INT NOT NULL DEFAULT 1,
    CONSTRAINT uq_sources_code UNIQUE (code),
    CHECK (dsn_ref = 'sources/' || id::text),
    CHECK (tier <> 'CT3'),                       -- log-based capture is not offered before phase 11 (DR-63)
    CHECK (readonly_level = 0 OR state NOT IN ('registered', 'refused'))
);
CREATE INDEX IF NOT EXISTS idx_sources_created_by ON admin.sources (created_by);
CREATE INDEX IF NOT EXISTS idx_sources_state      ON admin.sources (state);

CREATE TABLE IF NOT EXISTS admin.source_grant_checks (
    id                  UUID PRIMARY KEY,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    trigger             TEXT NOT NULL CHECK (trigger IN ('probe', 'connect', 'health', 'resume')),
    requested_by        UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- NULL when akr-ingestd started it
    status              TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'done', 'failed')),
    result              TEXT CHECK (result IN ('ok', 'refused')),
    engine_facts        JSONB NOT NULL DEFAULT '{}'::jsonb,   -- release, edition, character sets, clocks, isdba ...
    privileges          JSONB NOT NULL DEFAULT '{}'::jsonb,   -- expanded role graph and every privilege found
    session_flags       JSONB NOT NULL DEFAULT '{}'::jsonb,   -- read-only mode accepted, links, remote synonyms
    findings            JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(findings) = 'array'),
    charset             JSONB NOT NULL DEFAULT '{}'::jsonb,   -- round-trip result (C-10)
    tls                 JSONB,                                -- protocol, cipher, peer certificate; NULL = clear text
    readonly_level      SMALLINT CHECK (readonly_level BETWEEN 0 AND 3),
    snapshot_sha256     BYTEA CHECK (octet_length(snapshot_sha256) = 32),   -- privileges + flags, canonical JSON
    evidence_object_id  UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,   -- the sealed report
    error_code          TEXT CHECK (error_code IN ('credential_missing', 'client_library_missing', 'unreachable',
                                                   'tls_failed', 'credentials_rejected', 'account_locked',
                                                   'password_expired', 'no_session_privilege', 'timeout',
                                                   'engine_unsupported', 'internal')),
    requested_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    checked_at          TIMESTAMPTZ,
    last_confirmed_at   TIMESTAMPTZ,          -- later connects that found the same snapshot
    confirmations       INT NOT NULL DEFAULT 0 CHECK (confirmations >= 0),
    CHECK ((status = 'done') = (result IS NOT NULL)),
    CHECK ((status = 'failed') = (error_code IS NOT NULL)),
    CHECK (status = 'pending' OR checked_at IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_source_grant_checks_source   ON admin.source_grant_checks (source_id, requested_at DESC);
CREATE INDEX IF NOT EXISTS idx_source_grant_checks_requester ON admin.source_grant_checks (requested_by)
    WHERE requested_by IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_source_grant_checks_evidence ON admin.source_grant_checks (evidence_object_id)
    WHERE evidence_object_id IS NOT NULL;

DO $$ BEGIN
    ALTER TABLE admin.sources ADD CONSTRAINT fk_sources_last_check
        FOREIGN KEY (last_check_id) REFERENCES admin.source_grant_checks(id) ON DELETE RESTRICT;
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
CREATE INDEX IF NOT EXISTS idx_sources_last_check ON admin.sources (last_check_id) WHERE last_check_id IS NOT NULL;

-- Immutable rows: a new budget is a new version. Version 0 is the discovery default copied from
-- `discovery.default_budget` at registration; production ingestion (phase 11) refuses to start on it (DR-62).
CREATE TABLE IF NOT EXISTS admin.load_budgets (
    id                    UUID PRIMARY KEY,
    source_id             UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    version               INT NOT NULL CHECK (version >= 0),
    basis                 TEXT NOT NULL CHECK (basis IN ('discovery_default', 'signed')),
    max_sessions          SMALLINT NOT NULL CHECK (max_sessions BETWEEN 1 AND 8),
    statement_timeout_ms  INT NOT NULL CHECK (statement_timeout_ms BETWEEN 1000 AND 600000),
    rows_per_fetch        INT NOT NULL CHECK (rows_per_fetch BETWEEN 100 AND 10000),
    rows_per_minute       INT NOT NULL CHECK (rows_per_minute BETWEEN 100 AND 2000000),
    windows               JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(windows) = 'array'),
    blackout              JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(blackout) = 'array'),
    lag_ceiling_s         INT NOT NULL DEFAULT 0 CHECK (lag_ceiling_s BETWEEN 0 AND 86400),   -- CT1; 0 = none
    slowdown              JSONB NOT NULL DEFAULT '{"p95_pct_of_timeout": 50, "factor": 0.5, "floor_pct": 10}'::jsonb,
    bank_signatory        JSONB,           -- {"name": ..., "title": ...} exactly as on the signed document
    signed_document_id    UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    change_request_id     UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    effective_from        TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_by            UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_load_budgets_version UNIQUE (source_id, version),
    CHECK ((basis = 'discovery_default') = (version = 0)),
    CHECK (basis = 'discovery_default'
           OR (bank_signatory ? 'name' AND signed_document_id IS NOT NULL AND change_request_id IS NOT NULL))
);
CREATE INDEX IF NOT EXISTS idx_load_budgets_document   ON admin.load_budgets (signed_document_id)
    WHERE signed_document_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_load_budgets_cr         ON admin.load_budgets (change_request_id)
    WHERE change_request_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_load_budgets_created_by ON admin.load_budgets (created_by);

-- One row per interruption of a source: operator pause, bank lock, refusal, unreachable. Phase 11 plans the
-- backfill from it and phase 12 shows it as a coverage gap (C-15); nothing here is ever deleted.
CREATE TABLE IF NOT EXISTS admin.source_pauses (
    id           UUID PRIMARY KEY,
    source_id    UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    cause        TEXT NOT NULL CHECK (cause IN ('operator', 'bank_request', 'incident', 'account_locked',
                                                'credentials_rejected', 'password_expired', 'grant_revoked',
                                                'grant_refused', 'unreachable')),
    started_at   TIMESTAMPTZ NOT NULL,
    ended_at     TIMESTAMPTZ,
    started_by   UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,   -- system principal for ingestd
    ended_by     UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    comment      TEXT CHECK (comment IS NULL OR length(comment) <= 2000),
    detail       JSONB NOT NULL DEFAULT '{}'::jsonb,     -- error codes, attempts, the check that ended it
    CHECK (ended_at IS NULL OR ended_at >= started_at),
    CHECK ((ended_at IS NULL) = (ended_by IS NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_source_pauses_open ON admin.source_pauses (source_id) WHERE ended_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_source_pauses_source     ON admin.source_pauses (source_id, started_at DESC);
CREATE INDEX IF NOT EXISTS idx_source_pauses_started_by ON admin.source_pauses (started_by);
CREATE INDEX IF NOT EXISTS idx_source_pauses_ended_by   ON admin.source_pauses (ended_by) WHERE ended_by IS NOT NULL;

-- Grants. akr-api registers, edits, pauses and activates change requests; akr-ingestd owns the observed facts
-- (state from the connection, proof, contact) and the checks; akr-worker seals check reports.
GRANT SELECT, INSERT ON admin.sources TO akr_app;
GRANT UPDATE (name, cbs_product, adapter_pack, cbs_schemas, hints, approved_read_objects, access_exceptions,
              access_version, state, state_reason, state_changed_at, updated_at, version)
    ON admin.sources TO akr_app;
GRANT SELECT ON admin.sources TO akr_ingest;
GRANT UPDATE (state, state_reason, state_changed_at, readonly_level, readonly_proof, engine_version,
              last_contact_at, last_check_id, updated_at, version)
    ON admin.sources TO akr_ingest;

GRANT SELECT, INSERT ON admin.source_grant_checks TO akr_app;
GRANT UPDATE (evidence_object_id) ON admin.source_grant_checks TO akr_app;
GRANT SELECT, INSERT ON admin.source_grant_checks TO akr_ingest;
GRANT UPDATE (status, result, engine_facts, privileges, session_flags, findings, charset, tls, readonly_level,
              snapshot_sha256, error_code, checked_at, last_confirmed_at, confirmations)
    ON admin.source_grant_checks TO akr_ingest;

GRANT SELECT, INSERT ON admin.load_budgets TO akr_app;
GRANT SELECT ON admin.load_budgets TO akr_ingest;

GRANT SELECT, INSERT ON admin.source_pauses TO akr_app, akr_ingest;
GRANT UPDATE (ended_at, ended_by, detail) ON admin.source_pauses TO akr_app, akr_ingest;

GRANT SELECT ON admin.sources, admin.source_grant_checks, admin.load_budgets, admin.source_pauses TO akr_readonly;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', v.t, c, 'internal'
FROM (VALUES
  ('sources',             ARRAY['id','code','name','engine','tier','readonly_level','readonly_proof','dsn_ref',
                                'cbs_product','adapter_pack','engine_version','cbs_schemas','approved_read_objects',
                                'access_exceptions','access_version','hints','state','state_reason',
                                'state_changed_at','last_contact_at','last_check_id','created_by','created_at',
                                'updated_at','version']),
  ('source_grant_checks', ARRAY['id','source_id','trigger','requested_by','status','result','engine_facts',
                                'privileges','session_flags','findings','charset','tls','readonly_level',
                                'snapshot_sha256','evidence_object_id','error_code','requested_at','checked_at',
                                'last_confirmed_at','confirmations']),
  ('load_budgets',        ARRAY['id','source_id','version','basis','max_sessions','statement_timeout_ms',
                                'rows_per_fetch','rows_per_minute','windows','blackout','lag_ceiling_s','slowdown',
                                'bank_signatory','signed_document_id','change_request_id','effective_from',
                                'created_by','created_at']),
  ('source_pauses',       ARRAY['id','source_id','cause','started_at','ended_at','started_by','ended_by','comment',
                                'detail'])
) AS v(t, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'sources',             'R9', 'retirement', NULL, 'Retirement = disabled; mappings and batches cite it'),
 ('admin', 'source_grant_checks', 'R9', 'retirement', NULL, 'Read-only attestation (C-03); as its source'),
 ('admin', 'load_budgets',        'R9', 'retirement', NULL, 'Superseded versions kept: they explain past load'),
 ('admin', 'source_pauses',       'R9', 'retirement', NULL, 'Record of who paused a source, when and why (C-15); as its source')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

`dsn_ref` is fixed by the CHECK so a source can only point at its own credential entry. The credential itself
(user, password, connect descriptor, TLS trust) is written to OpenBao by `akr-ctl source credential set` from the
management zone and read only by `akr-ingestd` (DR-25, DR-59); no table, request body or log line holds it.

```sql
-- migration 0018_discovery.sql
-- Phase 05. Discovery runs, per-object profiles, field availability and captured code lists (MODULES M5,
-- DATA-MODEL s.3, DR-38, DR-40, DR-62).

DO $$ BEGIN
    CREATE TYPE admin.discovery_run_status AS ENUM ('queued', 'running', 'completed', 'failed', 'cancelled');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS admin.discovery_runs (
    id                  UUID PRIMARY KEY,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    kind                TEXT NOT NULL CHECK (kind IN ('catalogue', 'profile')),
    scope               JSONB NOT NULL DEFAULT '{}'::jsonb,   -- {"objects": [...]} (profile), sample_pct, options
    status              admin.discovery_run_status NOT NULL DEFAULT 'queued',
    requested_by        UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    budget_id           UUID NOT NULL REFERENCES admin.load_budgets(id) ON DELETE RESTRICT,   -- in force at start
    access_version      INT NOT NULL CHECK (access_version >= 0),      -- approved list the run read under
    adapter_pack        TEXT,
    objects_total       INT NOT NULL DEFAULT 0 CHECK (objects_total >= 0),
    objects_done        INT NOT NULL DEFAULT 0 CHECK (objects_done >= 0),
    objects_incomplete  INT NOT NULL DEFAULT 0 CHECK (objects_incomplete >= 0),
    findings            JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(findings) = 'array'),
    read_only_proof     JSONB NOT NULL DEFAULT '{}'::jsonb,   -- statement journal summary (section 3.4)
    error_code          TEXT CHECK (error_code IN ('grant_refused', 'source_paused', 'credential_missing',
                                                   'unreachable', 'run_timeout', 'internal')),
    cancel_requested    BOOLEAN NOT NULL DEFAULT FALSE,
    report_object_id    UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,   -- sealed run report
    started_at          TIMESTAMPTZ,
    finished_at         TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    CHECK (objects_done + objects_incomplete <= objects_total OR status IN ('queued', 'running')),
    CHECK ((status = 'failed') = (error_code IS NOT NULL)),
    CHECK ((status IN ('completed', 'failed', 'cancelled')) = (finished_at IS NOT NULL))
);
-- One active run per source: two runs would share one load budget and race on the profiles.
CREATE UNIQUE INDEX IF NOT EXISTS uq_discovery_runs_active ON admin.discovery_runs (source_id)
    WHERE status IN ('queued', 'running');
CREATE INDEX IF NOT EXISTS idx_discovery_runs_source       ON admin.discovery_runs (source_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_discovery_runs_requested_by ON admin.discovery_runs (requested_by);
CREATE INDEX IF NOT EXISTS idx_discovery_runs_budget       ON admin.discovery_runs (budget_id);
CREATE INDEX IF NOT EXISTS idx_discovery_runs_report       ON admin.discovery_runs (report_object_id)
    WHERE report_object_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS admin.discovery_profiles (
    id                 UUID PRIMARY KEY,
    run_id             UUID NOT NULL REFERENCES admin.discovery_runs(id) ON DELETE RESTRICT,
    source_id          UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    object_name        TEXT NOT NULL CHECK (object_name ~ '^[^.@\s]+\.[^.@\s]+$'),   -- OWNER.NAME as catalogued
    object_type        TEXT NOT NULL CHECK (object_type IN ('table', 'partitioned_table', 'view',
                                                            'materialized_view', 'synonym', 'foreign_table')),
    level              TEXT NOT NULL CHECK (level IN ('catalogue', 'profile')),
    status             TEXT NOT NULL CHECK (status IN ('complete', 'incomplete', 'skipped')),
    incomplete_reason  TEXT CHECK (incomplete_reason IN ('statement_timeout', 'budget_window', 'not_approved',
                                                         'object_missing', 'grant_missing', 'cancelled')),
    row_count          BIGINT CHECK (row_count >= 0),
    count_method       TEXT CHECK (count_method IN ('exact', 'index_range', 'sampled')),
    newest_date        DATE,
    date_column        TEXT,
    candidate          TEXT NOT NULL DEFAULT 'unknown'
                       CHECK (candidate IN ('live', 'copy_candidate', 'scratch_candidate', 'log_table', 'empty',
                                            'unknown')),
    liveness           JSONB NOT NULL DEFAULT '{}'::jsonb,   -- score and signals (section 3.6)
    keys               JSONB NOT NULL DEFAULT '{}'::jsonb,   -- declared keys and unique indexes, leading columns
    strategy_hint      JSONB,                                -- proposed change strategy (DR-37), never applied
    columns            JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(columns) = 'array'),
    timings            JSONB NOT NULL DEFAULT '{}'::jsonb,   -- statements, total, max and p95 milliseconds
    profiled_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_discovery_profiles_object UNIQUE (run_id, object_name),
    CHECK ((row_count IS NULL) = (count_method IS NULL)),
    CHECK ((status = 'incomplete') = (incomplete_reason IS NOT NULL) OR status = 'skipped')
);
CREATE INDEX IF NOT EXISTS idx_discovery_profiles_source ON admin.discovery_profiles (source_id, object_name);

CREATE TABLE IF NOT EXISTS admin.field_availability (
    id                   UUID PRIMARY KEY,
    source_id            UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    run_id               UUID NOT NULL REFERENCES admin.discovery_runs(id) ON DELETE RESTRICT,
    canonical_attribute  TEXT NOT NULL CHECK (canonical_attribute ~ '^[a-z][a-z0-9_]*\.[a-z][a-z0-9_]*$'),
    family               TEXT NOT NULL,                     -- from the attribute catalogue (section 3.7)
    available            BOOLEAN NOT NULL,
    quality              TEXT NOT NULL CHECK (quality IN ('good', 'partial', 'poor', 'unpopulated', 'absent',
                                                          'unassessed')),
    source_ref           TEXT,                              -- OWNER.TABLE.COLUMN[+COLUMN...]
    ref_origin           TEXT NOT NULL CHECK (ref_origin IN ('adapter_hint', 'steward_hint', 'none')),
    null_pct             NUMERIC(5,2) CHECK (null_pct BETWEEN 0 AND 100),
    needs_declaration    BOOLEAN NOT NULL DEFAULT FALSE,    -- CBS-held judgement field (DR-38)
    note                 TEXT,
    assessed_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_field_availability UNIQUE (run_id, canonical_attribute),
    CHECK (available = (quality IN ('good', 'partial', 'poor'))),
    CHECK ((quality = 'absent') = (source_ref IS NULL)),
    CHECK ((ref_origin = 'none') = (source_ref IS NULL))
);
CREATE INDEX IF NOT EXISTS idx_field_availability_source ON admin.field_availability (source_id, canonical_attribute);

-- The CBS's own code lists, captured whole from the dictionary object the adapter pack names (for ICS BANKS the
-- bilingual code table). Reference data only; phase 10 pre-fills code maps from it, the bank confirms each map.
CREATE TABLE IF NOT EXISTS admin.discovery_code_entries (
    run_id       UUID NOT NULL REFERENCES admin.discovery_runs(id) ON DELETE RESTRICT,
    source_id    UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    list_key     TEXT NOT NULL CHECK (length(list_key) BETWEEN 1 AND 64),
    code         TEXT NOT NULL CHECK (length(code) BETWEEN 1 AND 64),
    label_en     TEXT,
    label_native TEXT,                       -- as read, converted to UTF-8 by the client (DR-36)
    attrs        JSONB NOT NULL DEFAULT '{}'::jsonb,   -- abbreviations, user code, the columns that use the list
    PRIMARY KEY (run_id, list_key, code)
);
CREATE INDEX IF NOT EXISTS idx_discovery_code_entries_source ON admin.discovery_code_entries (source_id, list_key);

GRANT SELECT, INSERT ON admin.discovery_runs TO akr_app;
-- report_object_id: written by discovery_seal_report in akr-worker (role akr_app).
GRANT UPDATE (cancel_requested, report_object_id, version) ON admin.discovery_runs TO akr_app;
GRANT SELECT ON admin.discovery_runs TO akr_ingest;
GRANT UPDATE (status, objects_total, objects_done, objects_incomplete, findings, read_only_proof, error_code,
              started_at, finished_at, version)
    ON admin.discovery_runs TO akr_ingest;
GRANT SELECT ON admin.discovery_profiles, admin.field_availability, admin.discovery_code_entries TO akr_app;
-- UPDATE: a run resumed after a crash rewrites the object it was on (ON CONFLICT DO UPDATE).
GRANT SELECT, INSERT, UPDATE ON admin.discovery_profiles, admin.field_availability, admin.discovery_code_entries
    TO akr_ingest;
GRANT SELECT ON admin.discovery_runs, admin.discovery_profiles, admin.field_availability,
                admin.discovery_code_entries TO akr_readonly;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', v.t, c, 'internal'
FROM (VALUES
  ('discovery_runs',         ARRAY['id','source_id','kind','scope','status','requested_by','budget_id',
                                   'access_version','adapter_pack','objects_total','objects_done',
                                   'objects_incomplete','findings','read_only_proof','error_code','cancel_requested',
                                   'report_object_id','started_at','finished_at','created_at','version']),
  ('discovery_profiles',     ARRAY['id','run_id','source_id','object_name','object_type','level','status',
                                   'incomplete_reason','row_count','count_method','newest_date','date_column',
                                   'candidate','liveness','keys','strategy_hint','columns','timings','profiled_at']),
  ('field_availability',     ARRAY['id','source_id','run_id','canonical_attribute','family','available','quality',
                                   'source_ref','ref_origin','null_pct','needs_declaration','note','assessed_at']),
  ('discovery_code_entries', ARRAY['run_id','source_id','list_key','code','label_en','label_native','attrs'])
) AS v(t, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'discovery_runs',         'R9', 'retirement', NULL, 'Evidence behind the gap report (phase 06); as its source'),
 ('admin', 'discovery_profiles',     'R9', 'retirement', NULL, 'No personal values by construction (section 3.6)'),
 ('admin', 'field_availability',     'R9', 'retirement', NULL, 'Readiness grades cite the run (phase 06)'),
 ('admin', 'discovery_code_entries', 'R9', 'retirement', NULL, 'CBS reference data; code maps cite the run (phase 10)')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

`discovery_profiles.columns` holds, per column: name, declared type and byte length, `null_pct`, `distinct`,
`min` and `max` (numbers and dates of unguarded columns only), length statistics for text (min, average, max, count
at the byte cap), sentinel counts, `values` (value list with counts, only for unguarded columns with at most
`discovery.value_list_max_distinct` distinct values), `guard` (why a column got no values: `name_pattern`,
`adapter_class`, `lob`, `deny`), `name_quality` (the N-12 counts, name columns only), `code_list` (the CBS list the
field dictionary links it to) and `cbs_field_description`. Section 3.6 states how each is obtained.

```sql
-- migration 0019_sources_settings.sql
-- Phase 05. Settings, the evidence owner kind for source documents, and the access rule in the route map.

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('discovery.default_budget',            '{"max_sessions": 1, "statement_timeout_ms": 30000, "rows_per_fetch": 500,
   "rows_per_minute": 60000, "windows": [], "blackout": [], "lag_ceiling_s": 0}', 'json', NULL, TRUE,
  '{"ar": "ميزانية الحمل الافتراضية للاستكشاف", "en": "Default load budget for discovery"}'),
 ('discovery.exact_count_cap',           '200000', 'integer', NULL, FALSE,
  '{"ar": "حد العد الدقيق للصفوف", "en": "Row cap for exact counts"}'),
 ('discovery.sample_pct',                '3', 'integer', NULL, FALSE,
  '{"ar": "نسبة عينة الكتل للجداول الكبيرة", "en": "Block sample percentage for large tables"}'),
 ('discovery.value_list_max_distinct',   '60', 'integer', NULL, TRUE,
  '{"ar": "أقصى عدد قيم مميزة لقائمة القيم", "en": "Maximum distinct values for a value list"}'),
 ('discovery.unpopulated_null_pct',      '95', 'integer', NULL, TRUE,
  '{"ar": "نسبة الفراغ التي يُعد عندها الحقل غير مملوء", "en": "Null percentage at which a field is unpopulated"}'),
 ('discovery.partial_null_pct',          '20', 'integer', NULL, TRUE,
  '{"ar": "نسبة الفراغ التي يُعد عندها الحقل جزئياً", "en": "Null percentage at which a field is partial"}'),
 ('discovery.code_capture_max_rows',     '200000', 'integer', NULL, FALSE,
  '{"ar": "أقصى عدد صفوف لالتقاط جداول الرموز", "en": "Maximum rows captured from a code dictionary"}'),
 ('discovery.stale_finding_days',        '3', 'integer', NULL, FALSE,
  '{"ar": "عمر أحدث قيد الذي يُعد عنده المصدر متقادماً (أيام)", "en": "Age of the newest posting that marks a source stale (days)"}'),
 ('discovery.max_run_h',                 '12', 'integer', NULL, FALSE,
  '{"ar": "أقصى مدة لتشغيل الاستكشاف (ساعات)", "en": "Maximum length of a discovery run (hours)"}'),
 ('sources.session_max_age_s',           '3600', 'duration_s', NULL, TRUE,
  '{"ar": "أقصى عمر لجلسة الاتصال بالمصدر (ثوانٍ)", "en": "Maximum age of a source session (seconds)"}'),
 ('sources.bank_pause_backoff_s',        '[300, 3600]', 'json', NULL, FALSE,
  '{"ar": "مهلة إعادة المحاولة عند إيقاف المصرف للاتصال (أدنى، أقصى)", "en": "Retry back-off while paused by the bank (min, max)"}'),
 ('sources.cancel_grace_s',              '5', 'duration_s', NULL, FALSE,
  '{"ar": "مهلة إلغاء الاستعلامات الجارية عند الإيقاف (ثوانٍ)", "en": "Grace period to cancel running statements on pause (seconds)"}'),
 ('schedule.source_health',              '"0 */15 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول فحص صحة المصادر", "en": "Source health check schedule"}')
ON CONFLICT (key) DO NOTHING;

-- Source documents (the DBA-signed budget, the bank's access approval, signed exceptions) are evidence objects
-- owned by the source; the owner kind is registered in code, the lock period here (R9 in evidence.lock_days).

-- Route map: source access and load budgets are access changes with a SEC checker (DR-62). The rule is added
-- only while the map is still the shipped one; a bank that already changed its map adds the rule by change
-- request (release note), and until then the two kinds fall to the class rules, still four-eyes.
UPDATE admin.settings
   SET value = jsonb_insert(value, '{rules,3}',
                            '{"object_kind": ["load_budget", "source_access"], "route": "cr_access"}'::jsonb),
       updated_at = now()
 WHERE key = 'change.route_map'
   AND version = 1
   AND updated_by IS NULL
   AND NOT (value::text LIKE '%source_access%');
```

The `UPDATE` keeps `version` at 1 on purpose: it is the shipped value, written by a migration (no actor), and the
next change by request will move it to 2 as usual. `setting.updated` is not emitted for it; the migration's own entry
in `_sqlx_migrations` is the record, and the release notes list the rule.

## 2. DDD Implementation (Rust)

**Crate placement** (workspace pinned in phase 00; layer rules in `ddd.md`; driver isolation by `cargo deny`, phase
00 section 2):

```
backend/
├── adapters/                         # adapter packs: signed release artifacts (C-38), data only, no code
│   └── ics_banks/                    # manifest.json, discovery.json (hints), deny_columns.json, README (quirks)
├── catalogue/canonical-attributes.v1.json   # the attributes field availability grades (section 3.7)
├── schemas/integration-profile.v1.json      # the export of section 3.8
├── crates/
│   ├── sources/   # akr-sources: SourceReader port, adapters (feature-gated: pg, mssql, oracle, odbc), GrantPolicy
│   │              #   per engine, SelectProof, dictionary statements per engine, LoadGovernor, statement journal
│   ├── domain/    # + source (Source, LoadBudget, SourcePause, GrantCheck, state machine, SourceAccess value
│   │              #   objects), discovery (DiscoveryRun, ObjectProfile, FieldAvailability, CodeEntry, Finding),
│   │              #   adapter pack model (pure, no IO), change targets load_budget and source_access
│   ├── db/        # + Pg repositories for the eight tables
│   ├── authz/     # + resource adapters Source, DiscoveryRun; ChangeKind family `source`
│   └── jobs/      # + kinds discovery_probe, discovery_run, ingest_source_health (ingestd), discovery_seal_report
│                  #   (worker)
├── api/           # + routers sources, discovery; SourceRegistry and DiscoveryQuery wiring (no akr-sources dependency)
├── ingestd/       # + ConnectionGuard, DiscoveryProfiler, the three job handlers, the source listener
├── worker/        # + discovery_seal_report
└── ctl/           # + source credential set|check, source disable, adapter verify, fixtures cbs
```

`akr-api` never links `akr-sources` (phase 00 bans check): everything that touches a CBS runs in `akr-ingestd`, and
`akr-api` only writes intent rows and jobs and reads results (DR-59). The domain crate holds the pure parts both
sides need (state machine, budget validation, adapter-pack parsing, availability grading).

- **Domain entities** (`Debug, Serialize, Deserialize, FromRow, ToSchema`; fields mirror section 1, enums as Rust
  enums with `#[sqlx(type_name = "admin.source_state")]` and `admin.discovery_run_status`): `Source`, `GrantCheck`,
  `LoadBudget`, `SourcePause`, `DiscoveryRun`, `ObjectProfile`, `FieldAvailability`, `CodeEntry`.
- **Value objects:** `SourceCode`, `Engine` (`Postgres`, `MsSql`, `Oracle`, `Db2`, `Sybase`), `Tier` (`Ct1`, `Ct2`,
  `Ct4`; `Ct3` exists for parsing and is refused, DR-63), `ReadonlyLevel(u8)`, `ObjectName { owner, name }` (exact,
  case kept as the catalogue gives it; `Display` = `OWNER.NAME`), `ApprovedObject { name: ObjectName, object_type,
  designated: bool, bank_created: bool, purpose: String }` (`bank_created` = a view the bank's DBA created for AKR, the
  footprint register of C-02), `AccessException { finding: FindingCode, subject: String, document_id: Uuid,
  valid_to: Option<NaiveDate> }`, `SourceAccess { objects: Vec<ApprovedObject>, exceptions: Vec<AccessException> }`,
  `BudgetSpec` (the budget columns, validated by `BudgetSpec::validate()` against the CHECK bounds plus: windows and
  blackout entries are `{days: [1-7], from: "HH:MM", to: "HH:MM"}` in bank time, no zero-length entry, a window and a
  blackout may overlap only if the blackout wins), `Finding { code: FindingCode, severity: Refuse | Exceptionable |
  Warn | Info, subject: String, via: Vec<String>, detail: serde_json::Value }`, `PauseCause`, `RunKind`,
  `CountMethod`, `Candidate`, `Quality`, `SourceDocumentKind` (`budget_signed`, `access_approval`, `exception_signed`,
  `other`).
- **Repository traits** (domain layer; `Pg*` implementations in `db`):

```rust
#[async_trait]
pub trait SourceRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, s: &NewSource, actor: Uuid) -> Result<Source, RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<Source>, RepoError>;
    async fn get_by_code(&self, conn: &mut PgConnection, code: &str) -> Result<Option<Source>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &SourceFilter, page: Keyset) -> Result<Page<Source>, RepoError>;
    async fn update_attrs(&self, tx: &mut Tx, id: Uuid, p: &SourcePatch, expected_version: i32)
        -> Result<Source, RepoError>;
    /// Conditional transition: `UPDATE ... WHERE id = $1 AND state = ANY($2)`; None when the state moved.
    async fn transition(&self, tx: &mut Tx, id: Uuid, from: &[SourceState], to: SourceState, reason: &str)
        -> Result<Option<Source>, RepoError>;
    async fn set_access(&self, tx: &mut Tx, id: Uuid, access: &SourceAccess, expected_access_version: i32)
        -> Result<Source, RepoError>;
    async fn record_contact(&self, tx: &mut Tx, id: Uuid, check_id: Uuid, proof: &ReadonlyProof,
                            level: ReadonlyLevel, engine_version: &str) -> Result<(), RepoError>;
}

#[async_trait]
pub trait GrantCheckRepository: Send + Sync {
    async fn insert_pending(&self, tx: &mut Tx, source_id: Uuid, trigger: CheckTrigger, by: Option<Uuid>)
        -> Result<GrantCheck, RepoError>;
    async fn complete(&self, tx: &mut Tx, id: Uuid, outcome: &CheckOutcome) -> Result<GrantCheck, RepoError>;
    async fn fail(&self, tx: &mut Tx, id: Uuid, code: CheckError) -> Result<GrantCheck, RepoError>;
    /// Same snapshot as the latest done check: bump `confirmations` and `last_confirmed_at`, no new row.
    async fn confirm_latest(&self, tx: &mut Tx, source_id: Uuid, snapshot: &[u8; 32]) -> Result<bool, RepoError>;
    async fn latest(&self, conn: &mut PgConnection, source_id: Uuid) -> Result<Option<GrantCheck>, RepoError>;
    async fn history(&self, conn: &mut PgConnection, source_id: Uuid, page: Keyset) -> Result<Page<GrantCheck>, RepoError>;
    async fn attach_evidence(&self, tx: &mut Tx, id: Uuid, evidence_id: Uuid) -> Result<(), RepoError>;
}

#[async_trait]
pub trait LoadBudgetRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, b: &NewLoadBudget) -> Result<LoadBudget, RepoError>;
    async fn in_force(&self, conn: &mut PgConnection, source_id: Uuid) -> Result<LoadBudget, RepoError>;   // max version
    async fn history(&self, conn: &mut PgConnection, source_id: Uuid) -> Result<Vec<LoadBudget>, RepoError>;
}

#[async_trait]
pub trait SourcePauseRepository: Send + Sync {
    async fn open(&self, tx: &mut Tx, source_id: Uuid, cause: PauseCause, by: Uuid, comment: Option<&str>,
                  detail: serde_json::Value) -> Result<SourcePause, RepoError>;     // uq_source_pauses_open guards
    async fn close(&self, tx: &mut Tx, source_id: Uuid, by: Uuid, detail: serde_json::Value)
        -> Result<Option<SourcePause>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, source_id: Uuid, page: Keyset) -> Result<Page<SourcePause>, RepoError>;
}

#[async_trait]
pub trait DiscoveryRepository: Send + Sync {
    async fn insert_run(&self, tx: &mut Tx, r: &NewDiscoveryRun) -> Result<DiscoveryRun, RepoError>;   // uq active
    async fn get_run(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<DiscoveryRun>, RepoError>;
    async fn list_runs(&self, conn: &mut PgConnection, f: &RunFilter, page: Keyset) -> Result<Page<DiscoveryRun>, RepoError>;
    async fn request_cancel(&self, tx: &mut Tx, id: Uuid, expected_version: i32) -> Result<DiscoveryRun, RepoError>;
    async fn start(&self, tx: &mut Tx, id: Uuid, objects_total: i32) -> Result<Option<DiscoveryRun>, RepoError>;
    async fn progress(&self, tx: &mut Tx, id: Uuid, done: i32, incomplete: i32) -> Result<(), RepoError>;
    async fn finish(&self, tx: &mut Tx, id: Uuid, end: RunEnd, findings: &[Finding], proof: &ReadOnlyProofSummary)
        -> Result<DiscoveryRun, RepoError>;
    async fn upsert_profile(&self, tx: &mut Tx, p: &ObjectProfile) -> Result<(), RepoError>;   // (run_id, object_name)
    async fn completed_objects(&self, conn: &mut PgConnection, run_id: Uuid) -> Result<HashSet<String>, RepoError>;
    async fn profiles(&self, conn: &mut PgConnection, f: &ProfileFilter, page: Keyset) -> Result<Page<ObjectProfile>, RepoError>;
    async fn latest_catalogue(&self, conn: &mut PgConnection, source_id: Uuid) -> Result<Option<DiscoveryRun>, RepoError>;
    async fn replace_availability(&self, tx: &mut Tx, run_id: Uuid, rows: &[FieldAvailability]) -> Result<(), RepoError>;
    async fn availability(&self, conn: &mut PgConnection, source_id: Uuid, run_id: Option<Uuid>)
        -> Result<Vec<FieldAvailability>, RepoError>;
    async fn insert_codes(&self, tx: &mut Tx, rows: &[CodeEntry]) -> Result<u64, RepoError>;   // COPY-style batches
    async fn code_lists(&self, conn: &mut PgConnection, f: &CodeFilter, page: Keyset) -> Result<Page<CodeEntry>, RepoError>;
}
```

- **Repository notes:** every list is keyset-paged (`(created_at DESC, id DESC)`, profiles by `object_name`, codes by
  `(list_key, code)`); `GET /discovery/profiles` without `run_id` reads, per object, the newest `profile` level row,
  else the newest `catalogue` row, through `DISTINCT ON (object_name) ... ORDER BY object_name, level = 'profile'
  DESC, profiled_at DESC` on `idx_discovery_profiles_source`; inserts use `RETURNING *`; `transition` and
  `set_access` bump `version` and `updated_at` in the same statement.

- **Ports** (in `akr-sources`; only `akr-ingestd` implements and calls them):

```rust
/// One per engine (TS-04). Feature-gated: `pg` (sqlx), `mssql` (tiberius), `oracle` (the oracle crate over ODPI-C,
/// OCI 19), `odbc` (odbc-api: Db2, Sybase, and the Oracle second backend). Blocking drivers run on a bounded
/// spawn_blocking pool sized to the budget's max_sessions.
#[async_trait]
pub trait SourceReader: Send + Sync {
    fn engine(&self) -> Engine;
    fn dictionary(&self) -> &'static DictionaryCatalog;        // the fixed catalogue statements of section 3.5
    fn grant_policy(&self) -> &'static dyn GrantPolicy;
    fn session_control(&self) -> &'static [&'static str];      // the only non-SELECT text that may reach the engine
    async fn open(&self, cfg: &ConnectConfig, cred: &Credential) -> Result<Box<dyn SourceSession>, SourceError>;
}

#[async_trait]
pub trait SourceSession: Send {
    async fn identify(&mut self) -> Result<EngineFacts, SourceError>;
    async fn begin_read_only(&mut self) -> Result<ReadOnlyMode, SourceError>;   // Enforced | Advisory (section 3.4)
    /// Only a ProvenSelect can be executed: the type has no public constructor outside SelectProof.
    async fn query(&mut self, stmt: &ProvenSelect, binds: &[Bind], limits: &StatementLimits,
                   sink: &mut dyn RowSink) -> Result<QueryStats, SourceError>;
    fn cancel_handle(&self) -> CancelHandle;                    // OCI break, PG cancel request, TDS attention, SQLCancel
    async fn end(&mut self) -> Result<(), SourceError>;        // rollback; never commit
}

pub trait GrantPolicy: Send + Sync {
    /// The statements that read the session's effective privileges (all ProvenSelects over the dictionary).
    fn snapshot_statements(&self) -> &'static [DictionaryStatement];
    /// Pure: judge a snapshot against the engine allow-list (section 3.3). Never reads the database.
    fn judge(&self, snap: &PrivilegeSnapshot, cbs_schemas: &[String], exceptions: &[AccessException]) -> Judgement;
}
pub struct Judgement {
    pub result: CheckResult,                // Ok | Refused
    pub findings: Vec<Finding>,             // every refusal named with the path that granted it
    pub readonly_level: ReadonlyLevel,      // with the tier (section 3.3)
    pub proof: ReadonlyProof,
}

/// The one gate for statement text (C-54, DR-61).
pub struct SelectProof;
impl SelectProof {
    pub fn prove(sql: &str, dialect: Engine, scope: &ProofScope<'_>) -> Result<ProvenSelect, ProofError>;
}
pub enum ProofScope<'a> {
    Dictionary(&'static DictionaryCatalog),   // catalogue and grant statements, DUAL for the round trip
    Data { approved: &'a [ApprovedObject], bound: &'a BoundSpec },
}
pub struct ProvenSelect { sql: String, object: ObjectName, bound: Bound, shape_sha256: [u8; 32], binds: usize }

pub trait RowSink: Send { fn row(&mut self, r: &SourceRow) -> Result<(), SinkError>; }
```

- **utoipa schemas:** `SourceView` (the source without `dsn_ref`'s secret, which it never has, plus `budget_in_force`,
  `open_pause`, `latest_check` summary, `documents[]` and `credential_status`: `unknown`, `present`, `missing`, as
  last reported by akr-ingestd), `NewSourceRequest` (`code`, `name`, `engine`, `tier`, `cbs_product?`, `adapter_pack?`,
  `cbs_schemas?`), `SourcePatchRequest` (`name?`, `cbs_product?`, `adapter_pack?`, `cbs_schemas?`, `hints?`; all
  `Option`), `ProbeAccepted` (`check_id`, `Location`), `GrantCheckView` (all columns; `privileges` collapsed to the
  role graph and counts unless `?detail=true`), `LoadBudgetView`, `LoadBudgetProposal` (`BudgetSpec` +
  `bank_signatory` + `signed_document_id` + the change fields `reason_code`, `reason_text`, `effective_at?`),
  `SourceAccessProposal` (`objects[]`, `exceptions[]`, `access_approval_document_id`, change fields),
  `PauseRequest` (`cause`: `operator`, `bank_request`, `incident`; `comment`), `ResumeRequest` (`comment`),
  `SourceDocumentView`, `DiscoveryRunRequest` (`source_id`, `kind`, `objects?`, `sample_pct?`), `DiscoveryRunView`,
  `ObjectProfileView`, `FieldAvailabilityView`, `CodeEntryView`, `IntegrationProfile` (the v1 document, section 3.8).
  The change-request responses reuse the phase 04 `ChangeRequestView` (202).

- **Route table** (base `/api/v1`; every route names its Cedar action, section 3.10; **CR** = answers 202 with a change
  request, phase 04; **SU** = step-up assertion required):

| Method | Path | Scope | Description |
|-|-|-|-|
| GET | /sources | data.steward, security.ops | Sources with tier, proved read-only level, state, last contact, open pause (`source.list`) |
| POST | /sources | data.steward | Register a source; creates budget version 0 from `discovery.default_budget`; state `registered` (`source.create`) |
| GET | /sources/{id} | data.steward, security.ops | Detail: budget in force, latest check summary, open pause, documents, credential status (`source.read`) |
| PATCH | /sources/{id} | data.steward | Edit non-governed attributes (name, product, adapter pack, CBS schemas, hints); `If-Match` (`source.update`) |
| POST | /sources/{id}/probe | data.steward | Queue a connect and grant check in akr-ingestd; 202 with the check id (`source.probe`) |
| GET | /sources/{id}/grants | data.steward | Latest check and history; `?check_id=` for one; `?detail=true` for every privilege (`source.read_grants`) |
| GET | /sources/{id}/load-budget | data.steward | The budget in force and its history (`source.read`) |
| PUT | /sources/{id}/load-budget | data.steward | Propose a budget signed by the bank's DBA; **CR** `load_budget` (`change_request.create`, family `source`) |
| PUT | /sources/{id}/access | data.steward | Propose the approved object list and signed exceptions; **CR** `source_access` (same) |
| POST | /sources/{id}/pause | data.steward, security.ops | Pause now: running statements cancelled, sessions closed (the kill switch); **SU** (`source.pause`) |
| POST | /sources/{id}/resume | data.steward, security.ops | Resume after a fresh passing check (`source.resume`) |
| POST | /sources/{id}/documents | data.steward | Upload a signed document (multipart; staged, scanned and sealed by phase 02) (`source.document_upload`) |
| GET | /sources/{id}/documents/{doc_id} | data.steward, security.ops | Stream a source document through `EvidenceStore::open` (`source.document_read`) |
| GET | /discovery/runs | data.steward | Runs per source, newest first (`discovery.read`) |
| POST | /discovery/runs | data.steward | Queue a catalogue or profile run (`discovery.run`) |
| GET | /discovery/runs/{id} | data.steward | Progress, findings and the read-only proof summary (`discovery.read`) |
| POST | /discovery/runs/{id}/cancel | data.steward | Request cancellation; the run stops after the current statement (`discovery.run`) |
| GET | /discovery/profiles | data.steward | Object profiles: `source_id`, `run_id?`, `candidate?`, `q?` (name prefix) (`discovery.read`) |
| GET | /discovery/field-availability | data.steward | Availability and quality per canonical attribute for a source and run (`discovery.read`) |
| GET | /discovery/code-lists | data.steward | Captured CBS code lists: `source_id`, `list_key?`, `run_id?` (`discovery.read`) |
| GET | /discovery/integration-profile | data.steward | The integration profile and discovery checklist (JSON; `?download=true` as a file) (`discovery.export`) |

`security.ops` (phase 03 scope, SEC) on pause, resume and document reads lets the security officer act on an incident
and read what they must approve; SEC holds no other source route. The roles behind each action are in section 3.10.

- **Router functions:** `sources::router() -> OpenApiRouter<AppState>` (13 routes) and `discovery::router() ->
  OpenApiRouter<AppState>` (8 routes), merged in `build_app` under the authz layer. Mutations take `Idempotency-Key`
  (phase 00) except `POST /sources/{id}/probe` and `POST /discovery/runs/{id}/cancel`, which are naturally idempotent
  (a probe while one is pending returns the pending check; a second cancel is a no-op 200); `PATCH /sources/{id}`
  and the cancel take `If-Match`; the two **CR** routes take `If-Match` with the current budget version or
  `access_version` as the change request's base.

## 3. Business Logic

### 3.1 Sources and their states (`SourceRegistry`, akr-api)

**States** (`admin.source_state`; transitions are conditional UPDATEs, so a race between akr-api and akr-ingestd
loses cleanly with 409 or a no-op):

| From | To | By | When |
|-|-|-|-|
| `registered` | `active` | akr-ingestd | first check `ok` (probe, or the connect of a discovery run) |
| `registered`, `active`, `paused_by_bank` | `refused` | akr-ingestd | a check finds a `Refuse` finding, or an `Exceptionable` one without an exception in force |
| `refused` | `active` | akr-ingestd | a later check passes (the bank removed the grants or an exception came into force) |
| `active`, `refused`, `paused_by_bank` | `paused` | akr-api | `POST /pause` (operator, bank request, incident); an operator hold takes over a bank pause |
| `paused` | `active` | akr-ingestd | after `POST /resume`, when the resume check passes |
| `active` | `paused_by_bank` | akr-ingestd | the engine reports a locked or expired account, rejected credentials, no session privilege, a revoked grant on an approved object, or the listener is unreachable for longer than one back-off step (C-15) |
| `paused_by_bank` | `active` | akr-ingestd | a back-off attempt connects and its check passes |
| any but `disabled` | `disabled` | `akr-ctl source disable` | decommissioning (management zone, ceremony); no route |
| `active` | `halted` | phase 11 | watermarks halted after a CBS restore or failover (C-64); not reachable in this phase |

Every transition writes `state_reason`, `state_changed_at` and an audit event (section 3.10), and `NOTIFY
akr_source, '<id>'` in the same transaction; akr-ingestd listens and also polls every 30 seconds. A move into
`refused`, `paused` or `paused_by_bank` opens a `source_pauses` row with the matching cause; the move back to
`active` closes it with the id of the check that ended it in `detail`. `registered` and `disabled` open no pause:
nothing was being read.

**`create(ctx, NewSourceRequest)`** (`POST /sources`):
1. Validate: `code` pattern and unused (409 `E_NOT_ALLOWED_IN_STATE`, `code_taken`); `name` both languages;
   `engine` known and its adapter compiled into this release (`engine_unavailable`); `tier` CT1, CT2 or CT4, CT3 ->
   422 `E_VALIDATION` `tier_not_available` (DR-63); `adapter_pack`, when given, is a pack shipped in this release
   for this engine (`unknown_adapter_pack`); `cbs_schemas` entries are identifiers, at most 32.
2. `id` = UUIDv7; `dsn_ref` = `sources/<id>`.
3. In one transaction: insert the source (`registered`, `readonly_level` 0, empty access); insert `load_budgets`
   version 0, `basis = discovery_default`, values from `discovery.default_budget` (validated by
   `BudgetSpec::validate`; a corrupt setting fails the request with 500 and an ops alert, never a guessed budget);
   audit `source.registered {code, engine, tier, adapter_pack}`.
4. Return 201 with `SourceView`, `credential_status: unknown`, and the next step in `next_actions`:
   `["set_credential", "probe"]` (the credential is set out of band, section 3.2).

**`patch(ctx, id, SourcePatchRequest, If-Match)`**: only the attributes in the request schema; `hints` validated
against the adapter-pack hint schema (section 3.6: `attributes`, `name_columns`, `business_date`, `code_dictionary`,
`field_dictionary`, `log_tables`, `copy_patterns`), each object reference written `OWNER.NAME` exactly; a change to
`adapter_pack` or `cbs_schemas` adds `rerun_catalogue` to `next_actions`. Not allowed in `disabled` (409). Audit
`source.updated {fields}` (names only). Tier and engine never change: a different connection is a different source.

**`pause(ctx, id, PauseRequest)`** (**SU**, action class `approval`, phase 01):
1. Transition `active | paused_by_bank | refused` -> `paused` (from `refused` it also blocks probes and runs until a
   resume); `registered` or `disabled` -> 409 `E_NOT_ALLOWED_IN_STATE` (`nothing_to_pause`); a pause on a source
   already `paused` -> 200 with the open pause, no new row.
2. Open the pause row (`cause` from the request; when the source was `paused_by_bank` or `refused`, close that row
   first with `detail.superseded_by = operator` and open the operator's at the same instant, so the gap stays
   continuous), set
   `cancel_requested = true` on any active discovery run, `NOTIFY akr_source`.
3. akr-ingestd, on the notification: calls `CancelHandle::cancel()` on every running statement of the source, ends
   and closes every session within `sources.cancel_grace_s`, and refuses new sessions while the state is not
   `active`. The conformance suite measures the stop (section 6 item 8).
4. Audit `source.paused {cause, comment_hash, cancelled_statements}` (the count is added by akr-ingestd as
   `source.sessions_closed {count}`).

**`resume(ctx, id, ResumeRequest)`**: only from `paused` (else 409 `E_NOT_ALLOWED_IN_STATE`, `not_paused`); inserts
a pending check with `trigger = resume` and enqueues `discovery_probe`; the state stays `paused` until the check
passes (akr-ingestd then moves it to `active` and closes the pause). Answers 202 with the check id. A resume check
that refuses moves the source to `refused` and the pause row's cause stays as it was, with the refusal check in
`detail`. Backfill of the paused interval is planned by phase 11 from the closed pause row (C-16 there). Audit
`source.resume_requested`; the state change is audited when it happens.

**Documents** (`POST /sources/{id}/documents`): multipart with `kind` (`SourceDocumentKind`) and one file; calls
`EvidenceStore::stage(tx, NewEvidence { owner_kind: "source", owner_id: id, kind, retention_class: R9, ... })`
(phase 02: type sniffing, size limit, antivirus scan, sealing; the owner kind `source` is registered in the evidence
owner registry by this phase). Answers 202 with the evidence id and `av_status: pending`; a change request that cites a
document still pending or rejected fails `prepare` (`evidence_not_clean`). `GET .../documents/{doc_id}` streams
through `EvidenceStore::open` only for an evidence object whose owner is this source (else the 404 of a missing
document); every read writes an access event (`resource: source_document`, phase 02), and the document's tier is
`internal`, so no purpose code is asked. Audit `source.document_uploaded {kind, sha256, size_bytes}`.

**Reads.** `list` and `detail` join the budget in force, the open pause and the latest check summary (result, findings
counts by severity, checked_at). `credential_status` is what akr-ingestd last saw (`present` after a connect,
`missing` after `credential_missing`); akr-api never reads OpenBao for it.

### 3.2 Credentials and the connection path (DR-59)

- `akr-ctl source credential set <code>` (zone Z5, operator ceremony with the bank's DBA present): prompts for the
  user name, password (no echo), connect descriptor (host, port, service or database; or a TNS alias for Oracle),
  TLS trust (CA bundle path or wallet directory reference) and optional options (`application_intent=readonly` for a
  SQL Server availability-group secondary); writes them to OpenBao KV `akr/sources/<id>` with an operator token
  issued for the ceremony; prints the KV version, never the values. `akr-ctl source credential check <code>` asks
  akr-ingestd (management endpoint on `z5_mgmt`) to open and close one session and reports the outcome code only.
  Both write audit events with the operator principal (`source.credential_set {kv_version}`,
  `source.credential_checked {outcome}`).
- akr-ingestd reads the entry at each connect (AppRole `akr-ingestd`, KV read on `sources/*` only, phase 00), keeps
  it in memory for the life of the session pool and drops it on `NOTIFY akr_source` or pool recycle; OpenBao's audit
  device records every read (DR-25). A missing entry -> check `failed` with `credential_missing`, source state
  unchanged, `credential_status: missing`.
- Client libraries: the Oracle Instant Client 19 and the Db2 and Sybase ODBC drivers are the bank's, mounted
  read-only at `/opt/vendor` (`ORACLE_HOME`-free Instant Client, `odbcinst.ini` from the mount); the akr-ingestd
  image carries none (TS-04). A source whose adapter needs a missing library -> `client_library_missing`, and
  readiness of akr-ingestd lists it.
- Oracle sessions are opened with the client character set AL32UTF8 (`NLS_LANG=.AL32UTF8` in the process
  environment, set by akr-ingestd before the first OCI call), so single-byte Arabic is converted on the server and
  arrives as UTF-8 (DR-36); `NLS_DATE_FORMAT` is never relied on: every date is fetched as a typed value and bound as
  a typed value.
- Every physical session lives at most `sources.session_max_age_s` and is recycled after, so the grant check runs on
  every connect and at least that often on a busy source (C-03).
- TLS: required unless the bank's DBA states in the integration profile that the CBS listener has no TLS; the check
  then records `tls: null` and the finding `clear_text_transport` (Warn), shown on the checklist. Oracle native
  network encryption is reported as such when negotiated.

### 3.3 The grant check (`ConnectionGuard::connect`, akr-ingestd; C-03, C-04, DR-40, DR-60)

Every physical session goes through `ConnectionGuard::connect(source, trigger)`; there is no other way to obtain a
`SourceSession` (the adapters' `open` is `pub(crate)` to `akr-sources`, and the guard is the crate's only export that
returns a session):
1. Refuse to start when the source is `disabled`, or `paused` and the trigger is not `resume`, or the budget in force
   is missing (409-equivalent job error `source_not_active`, the job ends without retry).
2. Acquire a session slot from the `LoadGovernor` (section 3.5), read the credential (section 3.2), `open` with the
   connect timeout `min(10 s, statement_timeout_ms)`.
3. Map connect errors to the engine taxonomy (`SourceError::classify`; table below). A bank-side refusal moves an
   `active` source to `paused_by_bank` with the matching cause; `unreachable` does so only after it lasted one
   back-off step, so a single network blip does not open a coverage gap.
4. `identify()` -> `EngineFacts` (release, edition, character sets, length semantics, database and session time
   zones, the database clock, SYSDBA flag, undo retention where readable, optimizer statistics presence).
5. `begin_read_only()` (section 3.4); a refusal of the session-control statement is itself a finding
   (`read_only_mode_refused`, Refuse).
6. Run the policy's `snapshot_statements()` (each a `ProvenSelect` over the dictionary) into a `PrivilegeSnapshot`,
   canonicalise it (sorted keys, sorted arrays) and hash it.
7. If the hash equals the latest `done` check's `snapshot_sha256` and the trigger is `connect` or `health`:
   `confirm_latest` and skip to step 10 with that check's judgement (re-judged with the current exceptions, so an
   exception that expired refuses even with unchanged grants).
8. Else `judge()` the snapshot with the source's `cbs_schemas` and `access_exceptions`; run the character-set round
   trip (section 3.6 rule "round trip") when the trigger is `probe` or `resume`, or the engine facts changed.
9. Write the check (`complete`), and when the result or the snapshot changed, enqueue `discovery_seal_report
   {check_id}` for akr-worker (it renders the check as canonical JSON, signs it with `akr-sign-certificate`, stores
   it as an evidence object owned by the source, and attaches it: the signed attestation of C-03).
10. `Refused`: end and close the session, transition to `refused` (opening the pause), audit `source.refused {check_id,
    findings: [codes]}`, raise the cockpit tile to critical, and return `GuardError::Refused`; no statement other
    than the snapshot and identity statements was sent. `Ok`: `record_contact` (level, proof, engine version), a
    `registered` or `refused` source moves to `active`, and the session is returned.

**Connect error taxonomy** (per engine in the adapter; unknown codes -> `internal` and the raw code in the log only):

| Class | Oracle | PostgreSQL | SQL Server | Pause cause |
|-|-|-|-|-|
| `account_locked` | ORA-28000 | (role `NOLOGIN`: 28000 "not permitted to log in") | 18486 | `account_locked` |
| `credentials_rejected` | ORA-01017 | 28P01 | 18456 | `credentials_rejected` |
| `password_expired` | ORA-28001 | (`rolvaliduntil` passed: 28P01) | 18487, 18488 | `password_expired` |
| `no_session_privilege` | ORA-01045 | 42501 on connect | 916, 4060 | `grant_revoked` |
| `object_grant_revoked` | ORA-00942, ORA-01031 on an approved object | 42501, 42P01 | 229, 208 | `grant_revoked` |
| `unreachable` | ORA-12541, ORA-12170, ORA-12514 | connection refused, timeout | 53, 10060 | `unreachable` |
| `tls_failed` | ORA-28860 family, ORA-29024 | SSL errors | 20 (SSL) | none: check `failed` |

`credentials_rejected` backs off at least `sources.bank_pause_backoff_s[0]` between attempts and never retries within
one job, so AKR-TMS cannot be the cause of a lockout on a bank account that has a failed-login limit.

**The policy is an allow-list** (DR-60). The snapshot expands every role through nested roles (Oracle role graph,
PostgreSQL `pg_auth_members` recursively, SQL Server `fn_my_permissions`, which is already effective); a role passes
only when every privilege it grants passes, so a bank's own read role is accepted and the DBA role is refused by its
content, not only by its name. Anything not on the allow-list is a finding with the path that granted it (`via`:
`["BANKSYS", "role DBA", "role IMP_FULL_DATABASE"]`).

| Engine | Allowed (no finding) | Accepted with a Warn finding | Refused (Refuse) | Exceptionable (refused unless an exception is in force) |
|-|-|-|-|-|
| Oracle | `CREATE SESSION`; `SELECT_CATALOG_ROLE`; `SELECT ANY DICTIONARY`; object `SELECT` (and `READ` on 12.1 and later) on any object; roles whose expansion is only these; PUBLIC grants on objects owned by Oracle-supplied schemas (fixed list per release) | `SELECT ANY TABLE`, `SELECT ANY SEQUENCE`, `SELECT ANY TRANSACTION` (`broad_read`: prefer object grants, C-30); `clear_text_transport` | every other system privilege, including any `ANY` privilege outside the SELECT ANY family and `ALTER SYSTEM`; the DBA role and every role expanding to a refused privilege; any object privilege other than SELECT and READ (INSERT, UPDATE, DELETE, ALTER, INDEX, REFERENCES, EXECUTE, DEBUG, FLASHBACK, ON COMMIT REFRESH, QUERY REWRITE, WRITE, UNDER); an admin or grant option on anything; a SYSDBA, SYSOPER or proxy session; the account owning any object (`owns_objects`: an owner can write its own tables whatever its grants) | a database link the session can use, private or PUBLIC (`db_link_usable`, DR-40); a write privilege granted to PUBLIC on an object owned by a `cbs_schemas` owner (`public_write_grant`) |
| PostgreSQL | `LOGIN`; `CONNECT` on the database; `USAGE` on schemas; `SELECT` on relations; `pg_read_all_settings`, `pg_read_all_stats`, `pg_stat_scan_tables`, `pg_monitor` | `pg_read_all_data` (`broad_read`); `TEMPORARY` on the database (`temp_allowed`: no persistent footprint, level 2 still holds) | `SUPERUSER`, `CREATEROLE`, `CREATEDB`, `REPLICATION`, `BYPASSRLS`; `pg_write_all_data`, `pg_read_server_files`, `pg_write_server_files`, `pg_execute_server_program`, `pg_signal_backend`, `pg_checkpoint`, `pg_create_subscription` and any role expanding to them; `INSERT`, `UPDATE`, `DELETE`, `TRUNCATE`, `REFERENCES`, `TRIGGER`, `MAINTAIN` on any relation; `CREATE` on any schema or the database; ownership of any object (`owns_objects`); an admin option | `USAGE` on a foreign server or a callable `dblink` function (`foreign_server_usable`); `EXECUTE` on a `SECURITY DEFINER` function in a `cbs_schemas` schema (`definer_function_executable`) |
| SQL Server | server `CONNECT SQL`, `VIEW ANY DATABASE`; database `CONNECT`, `SELECT`, `VIEW DEFINITION`, `SHOWPLAN` | `VIEW SERVER STATE`, `VIEW DATABASE STATE` (`broad_metadata`) | every other permission returned by `fn_my_permissions` at server or database level (`CONTROL SERVER` implies them all, so `sysadmin` is refused by content); object-level `INSERT`, `UPDATE`, `DELETE`, `ALTER`, `CONTROL`, `TAKE OWNERSHIP`, `EXECUTE`, `REFERENCES`, `IMPERSONATE` found through `sys.database_permissions` for the session's `sys.user_token`; ownership of any schema or object | a linked server the login can use (`linked_server_usable`: a mapping in `sys.linked_logins` for the login or for all logins, or self-mapping) |
| Db2 (ODBC) | `CONNECTAUTH`; `SELECT` on tables and views; roles expanding only to these (`SYSCAT.ROLEAUTH`) | `SQLADM` (monitoring only, `broad_metadata`) | `DBADM`, `SECADM`, `DATAACCESS`, `LOAD`, `CREATETAB`, `BINDADD`, `IMPLICIT_SCHEMA`, any table privilege other than SELECT, `EXECUTE` on routines in `cbs_schemas`; ownership | a federated server or nickname the session can use (`federation_usable`) |
| Sybase ASE (ODBC) | `connect`; `select` on tables and views; user roles expanding only to these | none | `sa_role`, `sso_role`, `oper_role`, `dtm_tm_role` and any role expanding to them; any permission other than select in `sysprotects` for the user and its groups and roles; ownership | a remote server or proxy table the login can use (`remote_server_usable`) |

The Db2 and Sybase policies carry `certified: false` until the conformance rig has run against the bank's own
client and instance (TS-07: no CI image for Sybase; Db2 Community runs nightly); a check on an uncertified policy
adds the finding `policy_uncertified` (Warn) and phase 11 refuses production ingestion until the rig result is
recorded.

**Read-only level** (C-04), computed by `judge` and stored on the check and the source:
- **1 data plane**: the result is `Ok` and the read-only transaction mode is `Enforced` (Oracle, PostgreSQL), or the
  mode is `Advisory` (SQL Server, Db2, Sybase) and the allow-list holds; the proof says which.
- **2 footprint-free**: level 1 and no privilege to create anything persistent on the engine (no `CREATE` of any
  kind, no ownership), and the tier is CT1, CT2 or CT4. AKR-TMS itself creates nothing (C-02); the only CBS-side
  objects are the bank-created views on the approved list, flagged `bank_created`.
- **3 load-free**: level 2 and the tier is CT1 (a replica the bank runs anyway). A CT2 source is load-budgeted, never
  level 3.

`readonly_proof` = `{level, mode: enforced | advisory, session_control: [...], allow_list_version, findings_counts,
exceptions_in_force: [...], links_usable: n, links_used: 0, check_id}`; `links_used` is always 0 by construction
(section 3.4 rule 5) and the conformance suite proves it.

**Finding codes** (stable, used in the checklist and the gap report): Refuse: `write_privilege`, `dba_class_role`,
`any_privilege`, `admin_option`, `owns_objects`, `sysdba_session`, `create_privilege`, `object_privilege`,
`read_only_mode_refused`; Exceptionable: `db_link_usable`, `public_write_grant`, `foreign_server_usable`,
`definer_function_executable`, `linked_server_usable`, `federation_usable`, `remote_server_usable`; Warn:
`broad_read`, `broad_metadata`, `temp_allowed`, `clear_text_transport`, `policy_uncertified`, `remote_synonym`
(Oracle synonym through a link: never usable, section 3.4, but listed), `exception_expiring` (within 30 days); Info:
`no_optimizer_statistics`, `charset_single_byte`.

For the discovery account of the studied installation, the check refuses with `dba_class_role` (DBA and the roles it
brings), `any_privilege` (for example `DELETE ANY TABLE`, `DROP ANY TABLE`, `ALTER SYSTEM`), `owns_objects` (the
account owns the CBS schema) and `db_link_usable` (14 links, 11 PUBLIC), and the checklist states what the bank must
create instead: a dedicated account with `CREATE SESSION` and `SELECT` on the named objects (CBS-DISCOVERY.md s.1.3).

### 3.4 Statements: the proof, the read-only session and the journal (C-08, C-35, C-54, DR-61)

**`SelectProof::prove(sql, engine, scope)`**. Every statement text that reaches a source is generated by AKR-TMS
(dictionary statements are constants in the adapter, data statements come from the discovery generator now and from
the mapping compiler in phase 10) and is proven on its final text, after rendering, before it is sent. The proof
parses with `sqlparser` in the engine's dialect (`PostgreSqlDialect`, `MsSqlDialect`; for Oracle, Db2 and Sybase the
generic dialect after the pre-pass below) and refuses unless every rule holds:
1. Exactly one statement; no comment tokens; no string literal (values are bind variables only, C-13; the
   dictionary statements' fixed literals such as `'Y'` are part of their reviewed constant text and their `shape`).
2. A plain query: `SELECT` with no `INTO`, no locking clause (`FOR UPDATE`, `FOR SHARE`), no table hint of any kind
   (`WITH (UPDLOCK)`, `NOLOCK` included, C-08), no `WITH` clause, no set operation, no `TOP` with `PERCENT`.
3. Exactly one table factor, named `OWNER.NAME` and equal, case and all, to an entry of the scope: an approved object
   for `Data` (its `object_type` table, view or materialized view; never a synonym or a foreign table, so a name
   cannot reach through a link), a dictionary view or `DUAL` for `Dictionary`. A three-part SQL Server name is
   accepted only with the source's own database; four-part names, `OPENQUERY`, `OPENROWSET`, `OPENDATASOURCE` are
   refused.
4. No join, no subquery anywhere (select list, `FROM`, `WHERE`, `HAVING`), no lateral or table function.
5. No `@` token after an identifier (Oracle database link syntax), whatever the rest; together with rule 3 this is why
   `links_used` is 0 by construction.
6. Functions from a closed list only: `COUNT`, `MIN`, `MAX`, `SUM`, `AVG`, `LENGTH`, `LENGTHB`, `OCTET_LENGTH`,
   `DATALENGTH`, `NVL`, `COALESCE`, `CASE`, `TRIM`, `CAST`, `TRUNC` (dates), `EXTRACT`, `REGEXP_COUNT`, `REGEXP_LIKE`
   and the PostgreSQL `~` operator (name-quality counts, section 3.6), `ORA_HASH` (Oracle sampling fallback); no
   user function, package call, `NEXTVAL` or `CURRVAL`, `SYS_CONTEXT` outside dictionary statements.
7. A bound for `Data` statements, one of: a range on the declared bound column (`col >= :1 AND col < :2`, binds only),
   a block sample (`SAMPLE BLOCK (p) SEED (s)` on Oracle, `TABLESAMPLE SYSTEM (p) REPEATABLE (s)` on PostgreSQL and
   SQL Server), or a row cap (`ROWNUM <= :n`, `LIMIT $n`, `TOP (@n)`, `FETCH FIRST`). No other column appears in
   `WHERE` or `HAVING`: a statement can never select rows by customer, account or case (C-35).
8. `GROUP BY` on at most one column, and only in the value-list and per-branch statements of section 3.6 (the column
   is unguarded and the distinct count is already known to be at or below the cap).

The Oracle pre-pass removes exactly one `SAMPLE BLOCK (<int>) SEED (<int>)` token sequence at the table reference,
records it as the bound, and refuses any other token sqlparser's generic dialect cannot place; `ROWNUM` parses as an
identifier and is checked by rule 7. The proof returns `ProvenSelect` with `shape_sha256` = SHA-256 of the statement
with bind positions normalised; the type has no other constructor, and `SourceSession::query` accepts nothing else.
`ProofError` carries the rule number; a refusal is a bug in the generator, so it fails the object (`incomplete`,
reason in the log), raises `akr_select_proof_refusals_total`, and is never retried with another text.

**Read-only session per engine** (`begin_read_only`, C-08):

| Engine | Session control (the only non-SELECT text sent) | Mode | Consistency |
|-|-|-|-|
| Oracle | `SET TRANSACTION READ ONLY` at the start of each unit of work; the unit ends by an OCI rollback call (no SQL text) | Enforced: the engine refuses DML in the transaction (ORA-01456) | one read-only transaction per object or slice, its statement timeout capped at 80% of `undo_retention` when readable, so a long read ends before ORA-01555 |
| PostgreSQL | startup parameters `default_transaction_read_only=on`, `statement_timeout`, `idle_in_transaction_session_timeout`, `application_name=akr-ingestd`; `BEGIN READ ONLY` ... `ROLLBACK` | Enforced (SQLSTATE 25006) | `REPEATABLE READ` per unit of work |
| SQL Server | `SET TRANSACTION ISOLATION LEVEL SNAPSHOT` when snapshot isolation is allowed, else `READ COMMITTED` with `SET LOCK_TIMEOUT <ms>`; `SET NOCOUNT ON`; `ApplicationIntent=ReadOnly` on CT1 | Advisory: no engine read-only mode; level 1 rests on the allow-list and the proof | snapshot or short read-committed statements, never `NOLOCK` |
| Db2 | ODBC `SQL_ATTR_ACCESS_MODE = SQL_MODE_READ_ONLY`, isolation cursor stability | Advisory | short statements |
| Sybase | ODBC read-only access mode, isolation level 1 | Advisory | short statements |

**Statement journal.** Every statement sent is journalled in memory per job: `shape_sha256`, object, bound kind,
rows, elapsed milliseconds, outcome. At the end of a run or check the summary goes to `read_only_proof`
(`{statements, by_kind: {select, session_control}, distinct_shapes: [{sha256, object, count}], links_used: 0,
max_ms, p95_ms, cancelled, timeouts}`) and the same counters to Prometheus; the shapes, not the texts, are stored, and
the texts of the shipped shapes are listed by `akr-ctl adapter verify --shapes` for the bank's DBA, who sees the same
texts in the engine's own SQL monitoring.


### 3.5 The load budget, the kill switch and source health (`LoadGovernor`, akr-ingestd; C-13, C-15, DR-62)

One `LoadGovernor` per source lives in akr-ingestd and wraps every `SourceSession`; no statement reaches a source
without passing it. It is built from the budget in force (the highest `version` with `effective_from <= now()`) and
rebuilt on `NOTIFY akr_source` (payload `{source_id, what: budget | access | state}`) and on the hourly re-read that
covers a missed notification.

`LoadGovernor::acquire(&self, kind: StatementKind) -> Result<Permit, GovernorError>`:
1. State gate: the source is `active` (or the job is a `probe`, `health` or `resume` check, which may connect while
   `registered`, `refused`, `paused_by_bank` or a pending resume); otherwise `GovernorError::Paused { cause }`, and the
   job ends without a retry.
2. Calendar gate in bank time (`bank.timezone`, phase 00): inside a `blackout` entry -> `GovernorError::Blackout {
   until }`; when `windows` is not empty and the moment is outside every window -> `GovernorError::OutsideWindow {
   next }`. Discovery jobs reschedule themselves at `until` or `next` (the job's `run_at`, phase 00); they never wait
   holding a session. No statement starts inside a blackout; one that is running when a blackout begins is cancelled
   at its start (the governor arms a timer for the next boundary while a permit is out) and the object is recorded
   `incomplete` with reason `budget_window`, resumed after it.
3. Session gate: a `tokio::sync::Semaphore` of `max_sessions` permits; the probe and health checks use the same
   semaphore, so a check never adds a session the budget does not allow.
4. Rate gate: a token bucket of `rows_per_minute` tokens refilled continuously; every fetch of up to
   `rows_per_fetch` rows takes that many tokens before the fetch call, and a statement whose result is one aggregate
   row takes one. The effective rate is `rows_per_minute x slowdown_factor` (step 6).
5. The permit carries the statement timeout, the `CancelHandle` of the session and the deadline of the unit of work.

**Statement timeout.** Each statement runs under `statement_timeout_ms`, set where the engine enforces it: Oracle
`OCI_ATTR_CALL_TIMEOUT` on the service context (client 18 and later; the shipped client is 19), PostgreSQL the
session `statement_timeout` (startup parameter, section 3.4), SQL Server the client side: `tokio::time::timeout`
around the `tiberius` call followed by an attention signal so the server stops; Db2 and Sybase `SQL_ATTR_QUERY_TIMEOUT`.
A timeout cancels the statement, rolls back the unit of work and records the object `incomplete` with reason
`statement_timeout`; the profiler never retries the same object with a wider bound or a longer timeout (a wider read
is exactly what the budget forbids). It retries once with a narrower bound where the method allows it (a count over
a smaller range, a smaller sample), then leaves the object for the steward.

**Adaptive slow-down** (C-13): the governor keeps the elapsed times of the last 50 statements per source. When their
p95 exceeds `slowdown.p95_pct_of_timeout` percent of the statement timeout, the rate is multiplied by
`slowdown.factor`, down to `slowdown.floor_pct` percent of `rows_per_minute`; after ten minutes with the p95 back
under the line, it steps back up by the same factor until it reaches 100 percent. Every change is logged with the
p95 and the new rate, raises the gauge `akr_source_rate_factor`, and appears in the run's `read_only_proof`
(`slowdowns: n`). A slower CBS makes AKR-TMS read less, never more.

**Session recycling.** A session older than `sources.session_max_age_s` is closed after its current unit of work and a
new one opened through `ConnectionGuard::connect` (a fresh grant check, trigger `connect`), so a grant revoked during
the day is seen within the hour (C-03).

**The kill switch** (`POST /sources/{id}/pause`, section 3.1): the API commits the pause and `NOTIFY akr_source`
`{what: state}`; the listener in akr-ingestd calls `LoadGovernor::halt(cause)`, which closes the semaphore, fires the
`CancelHandle` of every permit out (`OCIBreak`, the PostgreSQL cancel request, the TDS attention, `SQLCancel`), waits
up to `sources.cancel_grace_s` for each session to return, then drops the sessions; a session that has not returned
is abandoned with its socket closed. The listener writes `detail.sessions_closed_at` on the pause row and the audit event
`source.sessions_closed {open_sessions, cancelled, abandoned, elapsed_ms}`. The jobs that were running end with
`GovernorError::Paused` and are not retried; a discovery run moves to `failed` with `error_code = source_paused` and
its completed objects kept. If akr-ingestd is down when the pause is committed, it reads the state on start and opens no
session; the API answer says `sessions_closed: pending` until the listener confirms.

**Paused by the bank** (C-15). When a connect fails with a cause the taxonomy of section 3.3 maps to the bank
(`account_locked`, `credentials_rejected`, `password_expired`, `grant_revoked`, `unreachable`), the guard opens a
pause with that cause and moves the source to `paused_by_bank`. The health job then tries one connect per attempt,
spaced by an exponential back-off from `sources.bank_pause_backoff_s[0]` doubling to `[1]` (300 s, 600 s, ... 3,600 s),
so a locked account is never hammered into a longer lock by AKR-TMS. A successful connect with a passing check
closes the pause (`resolved_by_check_id`) and returns the source to `active` with the audit event
`source.state_changed {from: paused_by_bank, to: active, cause}`; a passing connect whose check is refused moves it
to `refused`. The pause row's `started_at` and `ended_at` are what phases 11 and 12 read to plan the backfill and
show the coverage gap.

**Source health** (job `ingest_source_health`, scheduled by `schedule.source_health`, every 15 minutes; one job per
run that iterates the sources that are `active` or `paused_by_bank` and due by their back-off):
1. `ConnectionGuard::connect(source, CheckTrigger::Health)`: the full grant check of section 3.3 (a dictionary read of a
   few hundred rows), so privilege drift is seen at least every 15 minutes even with no ingestion running; an
   unchanged snapshot (`snapshot_sha256` equal to the last check's) updates `last_confirmed_at` and `confirmations` on
   that check instead of inserting a row.
2. When the adapter pack names a business-date statement and its object is approved (ICS BANKS: `PROCESS.BANK_DATE`,
   per branch), read it and keep `{branch_count, min_date, max_date, distinct_dates}` in `sources.hints.health`
   (system-written key, not editable through `PATCH`); a business date older than the calendar date by more than
   `discovery.stale_finding_days` is logged as `business_date_lagging` for the cockpit.
3. Update `last_contact_at`; set the gauge `akr_source_state{source, state}`.
4. The lag ceiling (`lag_ceiling_s`, CT1 replicas) is stored and validated now and enforced from phase 11, where a
   replica lag is measured against the landing watermarks.

### 3.6 Discovery runs (`DiscoveryProfiler`, akr-ingestd; C-07, C-09, C-10, C-36, C-62, N-12, DR-36, DR-37, DR-38, DR-62)

**Requesting a run** (`POST /discovery/runs`, akr-api, `DiscoveryRunService::request`):
1. Cedar `discovery.run` on the source; the source is `active` (a passing probe makes a `registered` source
   `active`, section 3.1); otherwise 409 `E_NOT_ALLOWED_IN_STATE` `source_not_active`, or 409 `E_SOURCE_REFUSED` when
   the state is `refused`.
2. `kind = catalogue`: `objects` must be absent. `kind = profile`: `objects` absent means every approved object; when
   given, each must be on `approved_read_objects` (else 422 `E_OBJECT_NOT_APPROVED` with the names); `sample_pct`,
   when given, within 1-20, else `discovery.sample_pct`.
3. `budget_id` = the budget in force; version 0 (the discovery default) is accepted for discovery and only for
   discovery. `access_version` = the source's current one; a run whose access version is no longer current when it
   starts (an access change approved in between) re-reads the list and drops the objects no longer approved.
4. Insert the run `queued` and the job `discovery_run {run_id}` in one transaction; a second active run on the source
   fails on `uq_discovery_runs_active` -> 409 `run_active` with the active run's id. Audit `discovery_run.requested`.

**Running** (job `discovery_run`; one at a time per source, which `uq_discovery_runs_active` guarantees): the handler
sets `running` and `started_at`, opens one session through the guard (trigger `connect`, so the run starts on a fresh
passing check), and writes progress after every object (`objects_total`, `objects_done`, `objects_incomplete`,
`statements`). Between statements it reads `cancel_requested` (set by `POST /discovery/runs/{id}/cancel`, which also
sends `NOTIFY akr_source {what: run_cancel}` so a long statement is broken through its `CancelHandle`); a run older
than `discovery.max_run_h` stops as `failed` with `error_code = run_timeout` and its completed objects kept. A
restarted job resumes: it skips objects already `complete` in `discovery_profiles` for the run (one checkpoint per
object). At the end: status `completed` (shown as "completed with gaps" when `objects_incomplete > 0`), `read_only_proof`
from the journal, findings, audit `discovery_run.completed {objects, incomplete, statements, findings_by_severity}`,
and the job `discovery_seal_report` for the worker.

**Catalogue run** (dictionary statements only; `ProofScope::Dictionary`):
1. Owners: `cbs_schemas`; when empty, the run reads the owner list only (`ALL_USERS` with object counts from
   `ALL_OBJECTS`), raises `cbs_schemas_unset` (Warn) and stops there: the steward sets the schemas, then runs again.
2. Objects per engine: Oracle `ALL_TABLES`, `ALL_TAB_COLUMNS`, `ALL_INDEXES`, `ALL_IND_COLUMNS`, `ALL_CONSTRAINTS` and
   `ALL_CONS_COLUMNS`, `ALL_TAB_COMMENTS`, `ALL_COL_COMMENTS`, `ALL_OBJECTS` (type, status, `LAST_DDL_TIME`),
   `ALL_DEPENDENCIES` (who references each table, and `REFERENCED_LINK_NAME`), `ALL_DB_LINKS`, `ALL_SYNONYMS`,
   `ALL_TAB_PARTITIONS` (counts), `DBA_SEGMENTS` bytes only when the account can read it (else size unknown, no
   finding), and `ALL_SOURCE` line 1 per program unit to count wrapped units; PostgreSQL `pg_catalog` (`pg_class`,
   `pg_attribute`, `pg_index`, `pg_constraint`, `pg_depend` via `pg_rewrite` for views, `pg_description`,
   `pg_stat_user_tables` when visible, `pg_total_relation_size`); SQL Server `sys.tables`, `sys.views`, `sys.columns`,
   `sys.indexes`, `sys.index_columns`, `sys.foreign_keys`, `sys.sql_expression_dependencies`,
   `sys.dm_db_partition_stats` when visible, `sys.extended_properties`; Db2 `SYSCAT.*`; Sybase `sysobjects`,
   `syscolumns`, `sysindexes`. Results go to `discovery_profiles` with `level = catalogue` (one row per table, view and
   materialized view; `columns` with name, type, length, nullability, comment; `keys` with primary, unique and
   foreign keys and indexes).
3. Adapter recognition: each installed pack's `discovery.json` lists signature objects and columns; the score is the
   share present. The best score above 0.8 becomes the finding `adapter_candidate {pack, version, score}` (Info). The
   pack is never set automatically (DR-36): the steward sets `adapter_pack` by `PATCH`, and the profile run uses it.
4. Candidate classification per object (`candidate`):
   - `copy_candidate`: the pack's copy patterns, or a generic suffix (`_BKP`, `_BAK`, `_TMP`, `_OLD`, `_NEW`, `_MRG`,
     `_COPY`, `_TEST`, a trailing date such as `_20240131` or `_2024`, a trailing number), or a column list equal to
     that of a live object whose name is its prefix (the studied installation has many such copies, CBS-DISCOVERY.md
     s.2);
   - `scratch_candidate`: the pack's scratch patterns (work tables a report or batch fills and truncates);
   - `log_table`: the pack's log list, or a name ending in `_LOG`, `_AUDIT`, `_HIST` with no foreign key to it;
   - `empty`: no segment (Oracle `DBA_SEGMENTS` readable) or zero pages;
   - `live`: everything else; `unknown` when the dictionary gave too little to decide.
5. Liveness score 0-100 per object (`liveness = {score, signals}`): referenced by program units (`ALL_DEPENDENCIES`
   count, weighted most), indexes beyond the primary key, foreign keys to or from it, `LAST_DDL_TIME` within a year,
   optimizer statistics present, segment size. It ranks; it never excludes. The steward decides what is approved.
6. Findings: `no_optimizer_statistics` (Info, with the share of tables without `LAST_ANALYZED`; the studied installation
   has none, so counts never rely on `NUM_ROWS`), `wrapped_code_share` (Info, wrapped units / all units: the business
   logic cannot be read from the database, DR-36), `db_links` (Warn, count and names), `copies_count` (Info),
   `charset_single_byte` (Warn when `NLS_CHARACTERSET` is not a Unicode set), `dictionaries_present` (Info, the pack's
   field and code dictionaries found), `partitioned_tables` (Info).
7. Character-set round trip (C-10, DR-36), run once per catalogue run: Oracle `SELECT CAST(:1 AS VARCHAR2(400)) FROM
   DUAL` and the same with `NVARCHAR2`, bound with the probe string `discovery::CHARSET_PROBE` (every Arabic letter,
   the alef, yeh, kaf and heh variants, tatweel, the eight tashkeel marks, Arabic-Indic and extended (Persian) digits,
   the Kurdish letters and Persian letters outside the Arabic block, and a Latin control set); the returned string is
   compared code point by code point and the result stored in the run's findings as `charset_round_trip
   {db_charset, nchar_charset, preserved, lost: [code points], replaced_with}` (Warn when anything is lost, with the
   checklist line "names in the lost letters cannot be stored by the CBS and arrive replaced"). PostgreSQL reads
   `server_encoding` (UTF8 passes without a round trip); SQL Server reads the collation code page of each name column
   in the profile run and flags non-`N` columns under a non-Arabic code page.

**Profile run** (data statements; `ProofScope::Data`; objects in ascending order of size, so the small reference
tables are done first and a timeout on a large table never starves the rest):
1. **Row count**, in this order:
   - bounded exact count (Oracle): `SELECT COUNT(*) FROM O.T WHERE ROWNUM <= :1` with `discovery.exact_count_cap + 1`;
     a result at or below the cap is `exact`. PostgreSQL and SQL Server cannot bound a count without a subquery
     (rule 4): SQL Server reads `sys.dm_db_partition_stats.row_count` when visible (maintained by the engine,
     `exact`); otherwise both go to the next two methods;
   - else, when a date-led index exists (the pack's date column or an index whose first column is a date), one
     `COUNT(*)` per year range `col >= :1 AND col < :2` from the newest year backwards until a year returns zero or ten
     years are read: `index_range` with the per-year counts kept (they show the volume curve of the object over the
     years);
   - else `SELECT COUNT(*) FROM O.T SAMPLE BLOCK (p) SEED (s)` scaled by `100 / p`: `sampled`, with the confidence noted.
2. **Newest date** on the date column (the pack hint, else the first date column of a date-led index): `MAX(col)`
   under a widening range bound (31 days, 1 year, 3 years, 10 years from the calendar date), stopping at the first
   non-null result; future dates beyond tomorrow are counted separately as `future_dated` (Info) and excluded.
3. **Column statistics**: over the whole object when its count is `exact`, else over a block sample of `sample_pct`
   with a fixed seed per run (repeatable): `null_pct`, distinct count (exact on the sample), length statistics
   (`min`, `p50`, `max` of `LENGTH`, and the count of values whose `LENGTHB` equals the declared byte length: the field
   was cut, `at_cap`), sentinel counts for dates (`0001-01-01`, `1900-01-01`, `9999-*`) and for numbers (`0`, `-1`,
   `999999...` at the declared precision); `min` and `max` only for numeric and date columns that are not guarded.
4. **Column guard** (personal values never leave the CBS through discovery, DR-62): a column is guarded when its name
   matches the guard patterns (`NAME`, `NAM`, `ID`, `NUM`, `PHONE`, `TEL`, `MOB`, `ADDR`, `CARD`, `PAN`, `IBAN`, `ACCT`,
   `REMARK`, `NARR`, `DESC`, `MEMO`, `EMAIL`, `BIRTH`, `DOB`, `SIGN` as whole tokens of the name split on `_`), when the
   pack classes it personal, or when its type is LOB, RAW, LONG or binary. A guarded column gets counts and lengths
   only: no `min`, `max`, value list or example. A **deny** column (the pack's `deny_columns.json` plus any name
   containing `PASSWORD`, `PWD`, `PIN`, `CVV`, `CVC`, `TRACK`, `PINBLOCK`) is not read at all, not even counted; its
   profile entry says `denied` (C-36). The guard and deny lists are part of the signed pack and the release tests.
5. **Value lists**: for an unguarded column with a distinct count at or below `discovery.value_list_max_distinct`, one
   `SELECT col, COUNT(*) FROM O.T SAMPLE ... GROUP BY col` (rule 8) gives the values and their shares; they are what
   the mapping author needs to write a code mapping in phase 10.
6. **Name quality** (N-12) on the columns the pack or the steward marks as name columns: counts only, through
   `REGEXP_COUNT` (Oracle) or `~` (PostgreSQL) over the sample: values with Arabic letters, Latin letters only, `?`
   (lost characters), Persian-only letters, tatweel, tashkeel, digits, leading or trailing space, a double space,
   length at the cap, word counts 1, 2, 3, 4 and 5 or more, final yeh against alef maksura, teh marbuta against heh.
   The result is `columns[].name_quality`; no name is read into AKR-TMS.
7. **Code dictionaries** (the pack's `field_dictionary` and `code_dictionary`; ICS BANKS: `FLD_CODE` and `TEXT_TAB`):
   read whole with a row cap of `discovery.code_capture_max_rows` (rule 7, a `ROWNUM` cap; reaching the cap is the
   finding `code_capture_truncated`), joined in AKR-TMS (never in the CBS, rule 4) into `discovery_code_entries`
   (`list_key` = the field code, `code`, `label_en`, `label_native`, `attrs`) and into each profiled column's
   `cbs_field_description` when the pack maps the column to a field code. These are the bank's own code labels
   (transaction types, account types, branch codes), not personal data.
8. **Business date** per branch from the pack's hint (ICS BANKS `PROCESS.BANK_DATE`): the finding `business_date
   {branches, min, max, distinct}`; a branch more than one day behind the others is listed by branch code.
9. **Staleness and silent branches**: `stale_source` (Warn) when the newest posting date is older than
   `discovery.stale_finding_days` against the clock; `silent_branches` (Warn) from one `GROUP BY` on the branch column
   of the posting table over the last 31 days (a range bound), listing branch codes known to the branch dictionary
   that posted nothing.
10. **Strategy hint** for phase 11 (`strategy_hint`): a unique or date-led index whose leading column is the posting
    or business date -> `business_date`; an object at or below the exact-count cap -> `full_compare`; else a unique
    numeric key -> `key_range`; else `none` with the reason. A `VARCHAR2(6)` column named like `*TIME*` beside a date
    column is noted `time_text_not_watermark` (DR-37): it is never offered as a watermark.
11. **Timings**: per object the statements, rows and elapsed milliseconds (`timings = {statements, rows, p50_ms,
    p95_ms, max_ms}`); the integration profile turns them into the proposed budget (section 3.8).

`discovery_seal_report` (akr-worker) renders the run's findings, the read-only proof and the profile summary (no
values: counts, types, availability) as JSON, stores it through `EvidenceStore::put_generated` (the worker's path for generated output; `stage` is for uploads through akr-api) with kind `render` and owner kind `source`,
retention R9, scanned and sealed by phase 02; the evidence object id is written on the run (`report_object_id`) and
the report is the signed record C-03 asks for.

### 3.7 Field availability (`FieldAvailabilityGrader`, domain, pure; run by `discovery_run` at the end of a profile run; DR-38)

**The attribute catalogue** `backend/catalogue/canonical-attributes.v1.json` ships with the release: one entry per
canonical attribute that a rating factor, scenario, screening rule or report needs, `{key: "<canon table>.<column>",
family: party | customer | account | txn | card | trade | payment | reference, module: core | kyc | teller |
payments | trade | cards | facilities | lockers | securities | digital (the CBS module a vendor request names, phase
06), label: {ar, en}, needs_declaration: bool, used_by: [...]}` (`used_by` names the consumers by code once phases 13-21 register them;
empty is allowed now). A phase 10 test asserts that every key names a column of the canonical model, so the catalogue
cannot drift from DATA-MODEL s.5. `needs_declaration` is true for the CBS-held judgement fields: `party.pep_status`
(from a CBS PEP flag), `customer.cdd_level` (from a CBS risk level) and `customer.expected_profile` (from CBS
expected-activity fields).

`grade(catalogue, hints, profiles, settings) -> Vec<FieldAvailability>`:
1. Candidate column per attribute: the steward's hint (`sources.hints.attributes["<key>"] = "OWNER.TABLE.COLUMN"`,
   `ref_origin = steward_hint`) over the adapter pack's hint (`adapter_hint`); neither -> `absent`, `source_ref` null,
   `ref_origin = none`. A hint naming an object that is not approved, or a column the catalogue run did not find, is
   kept with quality `unassessed` and the note `object_not_approved` or `column_missing`.
2. Not profiled in this run (the object was skipped or `incomplete`) -> `unassessed`.
3. `unpopulated` (DR-38): a single distinct value over the read (constant), only sentinel values, or `null_pct >=
   discovery.unpopulated_null_pct`.
4. `poor`: `null_pct >= 50`.
5. `partial`: `null_pct >= discovery.partial_null_pct`, or at least 1 percent of values at the byte cap, or at least
   1 percent sentinels.
6. Else `good`. `available` follows the CHECK (good, partial, poor).
7. `needs_declaration` copies the catalogue flag; such a row carries the note `bank_declaration_required` whatever its
   grade. The value is not a rating or scenario input until the bank declares the field maintained (phases 15 and 16
   enforce it; until then it is unknown, never low).

Rows are inserted per run (`uq_field_availability` on `run_id`, attribute), so the history shows how availability
changed between runs; the API answers the latest profile run by default. A copy candidate never supplies a hint: the
grader warns `hint_on_copy` when a steward hint points at one.

### 3.8 The integration profile (`IntegrationProfileBuilder`, `domain::discovery` over `db`, called by akr-api; C-07, C-30)

`GET /discovery/integration-profile?source_id=&run_id=` builds the document from stored rows only (no CBS access),
validated against `backend/schemas/integration-profile.v1.json` before it is returned; `?download=true` adds
`Content-Disposition` with `integration-profile-<code>-<date>.json`. Sections:
1. `source`: code, name, engine, tier, CBS product, adapter pack, engine facts from the latest check (version,
   character sets, time zone, `NLS_*`), read-only level and proof summary, `cbs_schemas`.
2. `access`: the latest grant check summary (result, findings by severity, the role graph collapsed), the exceptions
   in force and their documents, and the **minimum grants text** per engine (C-30): the exact statements the bank's
   DBA runs to create the AKR account (Oracle `CREATE USER ... ; GRANT CREATE SESSION TO ...; GRANT SELECT ON
   <owner>.<object> TO ...` for each approved object, no role; PostgreSQL `CREATE ROLE ... LOGIN; GRANT USAGE ON
   SCHEMA ...; GRANT SELECT ON ...`; SQL Server `CREATE LOGIN`, `CREATE USER`, `GRANT SELECT ON OBJECT::...`), generated
   from the approved list.
3. `objects`: each approved object with type, `designated`, `bank_created`, candidate, liveness score, count and
   method, newest date, strategy hint, timings, and the CT2 warning `base_table_on_primary` when the tier is CT2 and
   the object is a base table (C-30 prefers a bank-created view or a replica).
4. `budget`: the budget in force and a **proposed budget** from the timings: `statement_timeout_ms` = 3 x the largest
   object p95, at least 5,000; `rows_per_fetch` 500 (1,000 when every p95 is under one second); `max_sessions` 1 for
   CT2, 2 for CT1; `rows_per_minute` from the daily posting volume of the posting table (per-year counts / 250
   working days x 4, so the expected daily rows load within a quarter of the day), within the CHECK bounds; windows and
   blackout left for the bank (the end-of-day batch hours are the bank's to state). The proposal is a starting point
   for the DBA's signature, never applied.
5. `charset`: the round-trip result of section 3.6.
6. `business_date` and `staleness`: the per-branch business date, the newest posting date, `stale_source` and
   `silent_branches`.
7. `field_availability`: counts by family and quality, and the rows themselves.
8. `code_lists`: list keys with entry counts (not the entries, which the code-lists route gives).
9. `checklist` (C-07): items with `status: done | open | blocked` and the finding behind each: a dedicated read-only
   account (refused check -> blocked, with the finding codes); approved objects set; a signed load budget (version 0
   -> open); character set round trip clean; business-date source identified; adapter pack confirmed; `cbs_schemas`
   set; every CT2 base table warned; open questions generated from Warn findings (one line each, worded for the
   bank's IT).

The PDF rendering, the bilingual cover and the CBI coverage gap report built from this document are phase 06. The
builder lives in `domain::discovery` over the `db` repositories (stored rows only), so phase 06's akr-worker builds
the same document for the PDF without linking `akr-sources`.

### 3.9 The change targets `load_budget` and `source_access` (phase 04 `ChangeTarget` port; DR-62)

Both register in the `ChangeTargetRegistry` of akr-api and akr-worker (the phase 04 test that both binaries register
the same kinds covers them), family `source`, `generic_create = false` (the dedicated `PUT` routes create them),
`emergency_allowed = false`, `reason_families = ["change"]`, no inventory kind. The route map rule of migration 0019
sends both to `cr_access`: a DS maker and an SEC checker.

| Kind | `object_key` | Base version | Prepare | Activate | Revert | Closed |
|-|-|-|-|-|-|-|
| `load_budget` | `source:<id>:budget` | the budget version in force | source not `disabled`; `BudgetSpec::validate` (bounds 422 `budget_out_of_bounds` with the field); `bank_signatory.name` and `.title` present; `signed_document_id` is an evidence object owned by this source, kind `budget_signed`, scan `clean` (else 422 `evidence_not_clean`); class `minor`; volume effect `neutral`; `required_evidence = [evidence_object]` resolved to the signed document; impact = the field-by-field difference and, when a profile run exists, the objects whose p95 exceeds the new timeout (warning `timeout_below_observed`) | insert version n+1 (`basis = signed`, `change_request_id`, `created_by` = the maker, `effective_from` = the request's `activated_at`); `NOTIFY akr_source {what: budget}`; audit `load_budget.activated {version, change_request_id}` | not allowed (a new request proposes the old values) | nothing |
| `source_access` | `source:<id>:access` | `sources.access_version` | source not `disabled`; every object is in the latest completed catalogue run with type table, partitioned table, view or materialized view (a synonym or foreign table is refused, 422 `object_type_not_allowed`); a view whose `ALL_DEPENDENCIES` show a `REFERENCED_LINK_NAME` is refused (`view_uses_link`); a deny-listed column does not stop approval but is listed; `bank_created` objects must be views; `access_approval_document_id` owned by the source, kind `access_approval`, clean; each exception names an `Exceptionable` finding code of the latest check and a clean document of kind `exception_signed`, `valid_to` at most one year ahead; CT2 base tables -> warning `base_table_on_primary`; class `minor`, volume `neutral`; `required_evidence = [evidence_object]` | `SourceRepository::set_access(tx, id, access, base_version)` (bumps `access_version`; 409 `E_STALE_BASE` on a race); `NOTIFY akr_source {what: access}` (the next connect re-judges with the new exceptions); audit `source.access_changed {added, removed, exceptions_added, exceptions_removed, access_version}` | not allowed | nothing |

`diff` shows the budget field by field, and the access change as objects added and removed and exceptions added and
removed. An exception whose `valid_to` passes is dropped by the next grant check (the finding comes back as
`Exceptionable` without cover, the check refuses, the source moves to `refused`), and `exception_expiring` warns 30
days ahead in the health job's log and on the cockpit tile.

### 3.10 Errors, audit events, Cedar actions

**Error mapping.** Two codes in API.md s.5: `E_OBJECT_NOT_APPROVED` (422, the names in `errors[]`, new) and
`E_SOURCE_REFUSED` (409, the latest grant check refused the source; the finding codes in `errors[]`; it replaces the
earlier `E_SOURCE_WRITE_PRIVILEGE`, since a refusal has more causes than a write privilege and a probe answers 202).
`E_SOURCE_REFUSED` answers a discovery run on a `refused` source. Used: `E_SCOPE`; `E_FORBIDDEN`; `E_NOT_FOUND`; `E_VERSION_CONFLICT`; `E_PRECONDITION_REQUIRED`;
`E_STEPUP_REQUIRED` (pause); `E_STALE_BASE` (the two change targets); `E_NOT_ALLOWED_IN_STATE` with reasons
`code_taken`, `nothing_to_pause`, `not_paused`, `run_active`, `source_not_active`, `run_not_active` (cancel of a finished run), `source_disabled`; `E_VALIDATION` with reasons `tier_not_available`
(CT3), `engine_unavailable` (the build lacks the driver feature or the bank has not supplied its client),
`unknown_adapter_pack`, `evidence_not_clean`, `budget_out_of_bounds`, `object_type_not_allowed`, `view_uses_link`,
`exception_not_exceptionable`, `document_wrong_owner`, `sample_pct_out_of_bounds`. `SourceError`, `GovernorError`,
`ProofError` and `DiscoveryError` (`thiserror`) map in `api::error` (the API side) or end the job (the ingestd side).

**Audit events emitted** (catalogue additions; all SIEM-forwarded from phase 25):

| Event kind | On | Resource |
|-|-|-|
| `source.registered` | create (code, engine, tier) | source |
| `source.updated` | patch (field names, never hint values verbatim beyond the names) | source |
| `source.probe_requested`, `source.resume_requested` | probe and resume (check id) | source |
| `source.grant_checked` | every new check row (trigger, result, level, findings by severity, snapshot hash; confirmations of an unchanged snapshot are counted, not audited) | source_grant_check |
| `source.refused` | a check refuses (finding codes) | source |
| `source.state_changed` | every state transition (from, to, cause, check or pause id) | source |
| `source.paused` | pause opened (cause, by user or by the guard) | source_pause |
| `source.sessions_closed` | the listener closed the sessions (counts, elapsed) | source_pause |
| `source.document_uploaded` | document staged (kind, sha256, size) | source |
| `source.credential_set`, `source.credential_checked` | `akr-ctl` (never the secret; actor the system principal, with the operator's OS user and host in the details, since the management zone has no AKR session) | source |
| `source.disabled` | `akr-ctl source disable` (reason) | source |
| `source.access_changed` | section 3.9 | source |
| `load_budget.activated` | section 3.9 | load_budget |
| `discovery_run.requested`, `.started`, `.completed`, `.failed`, `.cancel_requested`, `.cancelled` | run lifecycle (kind, counts, error code) | discovery_run |
| `discovery_run.report_sealed` | the worker sealed the report (evidence id, sha256) | discovery_run |

Reused: `change_request.*` (phase 04) for the two targets; `evidence.staged` (source documents) and `evidence.stored` (the run report, through `put_generated`) (phase 02).

**Cedar** (bundle file `70_sources.cedar`, the range phase 03 reserved for sources and mapping). Entities added to
`schema.cedarschema`: `Source { state: String, engine: String, tier: String }`, `DiscoveryRun { source: Source,
requested_by: User, status: String }`; the `ChangeKind` family `source` (kinds `load_budget`, `source_access`).

| Action | Resource | Who (shipped bundle) |
|-|-|-|
| `source.list`, `source.read` | System, Source | DS, SEC (SEC to reach the kill switch and documents; added while writing phase 08) |
| `source.read_grants` | System, Source | DS |
| `source.create`, `source.update`, `source.probe`, `source.document_upload` | System, Source | DS |
| `source.pause`, `source.resume` | Source | DS, SEC |
| `source.document_read` | Source | DS, SEC |
| `discovery.read`, `discovery.run`, `discovery.export` | System, Source | DS |
| `change_request.create` (family `source`) | ChangeKind | DS |
| `change_request.read` (family `source`) | ChangeRequest | DS, SEC, ADM, IA, M, or a member of `parties` |

```cedar
// 70_sources.cedar (excerpt)
@id("source.steward") @role("DS")
permit (principal, action in [Akr::Action::"source.list", Akr::Action::"source.read",
        Akr::Action::"source.read_grants", Akr::Action::"source.create", Akr::Action::"source.update",
        Akr::Action::"source.probe", Akr::Action::"source.document_upload", Akr::Action::"source.document_read",
        Akr::Action::"source.pause", Akr::Action::"source.resume", Akr::Action::"discovery.read",
        Akr::Action::"discovery.run", Akr::Action::"discovery.export"], resource)
when { principal.roles.contains("DS") };

@id("source.security") @role("SEC")
permit (principal, action in [Akr::Action::"source.list", Akr::Action::"source.read",
        Akr::Action::"source.pause", Akr::Action::"source.resume", Akr::Action::"source.document_read"], resource)
when { principal.roles.contains("SEC") };

// A disabled source is history: nothing but reads.
@id("source.disabled_readonly") @deny("state")
forbid (principal, action in [Akr::Action::"source.update", Akr::Action::"source.probe",
        Akr::Action::"source.pause", Akr::Action::"source.resume", Akr::Action::"source.document_upload",
        Akr::Action::"discovery.run"], resource is Akr::Source)
when { resource.state == "disabled" };
```

Attributes evaluated: `principal.roles`, `delegated_roles`; `resource.state`; `context.aal` and `stepup_age_s` on
pause. IA and M reach source facts through the change requests of family `source` and, from phase 06, the gap report;
they hold no source route (the route table's `data.steward` scope). The bundle tests assert that no role but DS can
run discovery or read a profile, and that SEC cannot create, update or probe a source.

**Tipping-off review.** This phase holds no customer, alert or case data, and its CBS-facing side is built so the
bank's IT cannot learn anything about an investigation from it:
- Every statement AKR-TMS sends selects by range, sample or row cap over a whole object, with binds only; no
  statement names a customer, account or case (rule 7, C-35). The bank's DBA, who sees every statement in the engine's
  own monitoring, sees the same shapes every day whether or not an investigation exists.
- Profiles, value lists and the integration profile hold no personal values: guarded columns give counts and lengths
  only, deny columns are not read (C-36), name quality is counts only. Code lists are the bank's own reference labels.
- The integration profile and the sealed run report go to the steward and, through phase 06, to the bank's IT and
  the CBI; neither contains anything derived from a case or an alert.
- Audit details carry ids, codes, counts and hashes; hint values and document contents never enter them.
The answer for a person outside a case's access list is: nothing.

**Settings and configuration used:** the 13 keys of migration 0019 (registry bounds: `discovery.default_budget`
validated by `BudgetSpec::validate`; `discovery.exact_count_cap` 1,000-5,000,000; `discovery.sample_pct` 1-20;
`discovery.value_list_max_distinct` 10-500; `discovery.unpopulated_null_pct` 80-100; `discovery.partial_null_pct`
1-79 and below the unpopulated value; `discovery.code_capture_max_rows` 1,000-2,000,000; `discovery.stale_finding_days`
1-30; `discovery.max_run_h` 1-72; `sources.session_max_age_s` 300-86,400; `sources.bank_pause_backoff_s` two integers
60-86,400, ascending; `sources.cancel_grace_s` 1-60; `schedule.source_health` a cron at most hourly); phase 00
`bank.timezone`; phase 02 `evidence.lock_days`; phase 04 `change.route_map`.

## 4. Infrastructure Updates

- **Job kinds registered:**
  - akr-ingestd (jobs RLS allows `akr_ingest` the prefixes `ingest_` and `discovery_`, phase 00): `discovery_probe`
    (enqueued by create-probe and resume; payload `{check_id}`; concurrency 2; max attempts 1: a failed connect is a
    result, not a retry), `discovery_run` (payload `{run_id}`; concurrency 2 across sources, one per source by
    `uq_discovery_runs_active`; max attempts 3, resuming from the per-object checkpoint), `ingest_source_health`
    (scheduled, `schedule.source_health`; concurrency 1).
  - akr-worker: `discovery_seal_report` (payload `{run_id}` or `{check_id}`, DR-366; concurrency 2; max attempts 10 with the phase 00
    back-off; idempotent: a run with `report_object_id` set is skipped).
  - **Notification channel:** `akr_source` (payload `{source_id, what: state | budget | access | run_cancel}`),
    sent by akr-api and the change targets, listened to by akr-ingestd with a 30-second poll as the fallback. No outbox
    topic and no timer kind in this phase.
- **Compose:**
  - Dev profile `fixtures` adds `cbs-fixture-pg` (PostgreSQL 17 with a synthetic ICS-like schema generated by `akr-ctl
    fixtures cbs generate --seed <n> --rows <n>`: the same owners, table names and column types as the adapter pack's
    signature, filled with invented values; nothing is derived from `cbs-discovery/` or `test-poc/` data) on `z1_cbs`,
    with a `akr_reader` account holding only `CONNECT` and `SELECT` on the fixture objects and a second account
    `akr_bad` holding `INSERT` and a role chain to `pg_write_all_data` for the refusal tests.
  - Dev profile `fixtures-oracle` (opt-in, large images): `gvenzl/oracle-xe:11.2.0.2-slim` for the 11.2 dialect and
    `gvenzl/oracle-free` for the current one, both seeded by the same generator in Oracle form, with a read-only
    account, a DBA-class account and a PUBLIC database link for the refusal tests; dev profile `fixtures-mssql` with
    `mcr.microsoft.com/mssql/server:2022` the same way. Images are pinned by digest in `deploy/images.lock`.
  - `akr-ingestd` mounts `/opt/vendor` read-only (the bank-supplied Oracle Instant Client 19 and ODBC drivers, C-61;
    the release image carries none) and sets `NLS_LANG=.AL32UTF8` and `LD_LIBRARY_PATH` for it.
  - Zones: `z1_cbs` carries real traffic from this phase; in dev the fixtures join `z1_cbs` and `make zone-check`
    uses the `fixtures` expectation file (akr-ingestd reaches the fixture port, no other service does). In production
    the CBS listener must be declared in `deploy/zones.yaml` at install (`akr-ctl net render`); an undeclared one
    fails the connect as `unreachable` with the hint `egress_not_declared`.
- **AppState additions:** `api`: `sources: Arc<SourceRegistry>`, `discovery: Arc<DiscoveryQuery>`
  (`DiscoveryRunService`, `FieldAvailabilityQuery`, `IntegrationProfileBuilder`), the change targets `load_budget` and
  `source_access` in `change_targets`, the evidence owner kind `source`. `ingestd`: `guard: Arc<ConnectionGuard>`,
  `readers: AdapterRegistry` (feature-gated drivers), `profiler: Arc<DiscoveryProfiler>`, `governors:
  GovernorMap` (one `LoadGovernor` per source), the `akr_source` listener, `credentials: Arc<dyn CredentialStore>`
  (OpenBao KV `akr/sources/<id>`, read-only token). `worker`: the seal handler and the same change targets. `ctl`:
  `source credential set|check`, `source disable`, `adapter verify [--shapes]`, `fixtures cbs generate`.
- **OpenAPI registration:** tags `sources` (13 operations) and `discovery` (8), 21 in all, each with `x-akr-action`;
  the two **CR** routes document 202 with `ChangeRequestView` and `Location`; `POST /sources/{id}/pause` references the
  step-up scheme and declares `X-StepUp-Assertion`; probe and cancel state that they take no `Idempotency-Key`; the
  documents upload is `multipart/form-data` with the phase 02 size limit; the integration profile response references
  the `IntegrationProfileV1` schema generated from `integration-profile.v1.json`.
- **Network zones:** no new edge. akr-ingestd keeps its phase 00 networks (`z1_cbs`, `z1_db`, `z1_bao`, `z5_mgmt`) and
  is still the only service on `z1_cbs`; akr-api reaches no CBS and no OpenBao KV path under `akr/sources/`.
- **Readiness:** akr-ingestd's `/readyz` adds `source_listener` (the `LISTEN akr_source` connection is up) and
  `adapters` (every compiled adapter's client library loads; a missing Oracle client makes the Oracle adapter
  `unavailable`, not the binary unready, and `POST /sources` with that engine answers `engine_unavailable`).
- **Metrics:** `akr_source_state{source, state}` (gauge), `akr_source_statements_total{source, kind}`,
  `akr_source_statement_seconds{source}` (histogram), `akr_source_rate_factor{source}` (gauge),
  `akr_select_proof_refusals_total{rule}`, `akr_grant_check_total{source, result}`, `akr_discovery_objects_total{source,
  status}`. Alert rules: a source `refused` or `paused_by_bank`, a proof refusal (any), a grant check with a new
  finding against the previous snapshot, a health check missed for an hour.
- **Ops cockpit:** a `Sources` tile on `GET /admin/ops` (state per source, last contact, the open pause and its age,
  exceptions expiring within 30 days, the business-date lag), read from the tables, no CBS access.
- **Existing entity, schema or route updates:**
  - Phase 00: akr-ingestd readiness checks above; `deploy/images.lock` gains the fixture images.
  - Phase 02: evidence owner kind `source` registered (retention R9).
  - Phase 03: Cedar file `70_sources.cedar`; the cockpit tile; `schema.cedarschema` entities.
  - Phase 04: the route-map rule (migration 0019), the `ChangeKind` family `source`, two change targets.
  - Offboarding: nothing (a source has no owner person; a DS leaving changes no source).
  - Planning docs aligned in the same change: API.md 3.5 and s.5; DATA-MODEL s.3, s.21 and s.23; ROADMAP A4;
    roadmap rows 05 and 11.

## 5. Frontend Impact

Backend phase for screens. `make generate-api` regenerates the client with the hooks `useGetSources`,
`usePostSources`, `useGetSourcesId`, `usePatchSourcesId`, `usePostSourcesIdProbe`, `useGetSourcesIdGrants`,
`useGetSourcesIdLoadBudget`, `usePutSourcesIdLoadBudget`, `usePutSourcesIdAccess`, `usePostSourcesIdPause`,
`usePostSourcesIdResume`, `usePostSourcesIdDocuments`, `useGetSourcesIdDocumentsDocId`, `useGetDiscoveryRuns`,
`usePostDiscoveryRuns`, `useGetDiscoveryRunsId`, `usePostDiscoveryRunsIdCancel`, `useGetDiscoveryProfiles`,
`useGetDiscoveryFieldAvailability`, `useGetDiscoveryCodeLists`, `useGetDiscoveryIntegrationProfile`.

Phase 08 builds T1 "Data connection" on them (source list and detail, the grant check with findings by name, the
budget and access proposals, pause and resume with the `StepUpDialog`) and the discovery part of E2 (runs, profiles
ranked by liveness, field availability by family, code lists); phase 06 adds the gap report and the PDF of the
integration profile. Until then the steward works through `akr-ctl` and Swagger UI.

## 6. Verification

1. `make migrate`: 0017-0019 apply; a second run is a no-op; the phase 00 schema checks pass for the eight new tables
   (classification and retention rows present); a source with `tier = 'CT3'` or a `dsn_ref` other than
   `sources/<id>` fails its CHECK; a second budget version 0 or a `signed` budget without a document fails; a second
   open pause or a second active discovery run on one source fails its unique index; `akr_app` cannot `UPDATE
   admin.load_budgets` or `admin.source_grant_checks.result`; the route map holds the `cr_access` rule for
   `load_budget` and `source_access` on a fresh install, and is untouched on a database whose map was changed before.
2. `cargo check` with `cargo sqlx prepare`; `cargo clippy --all-targets -- -D warnings`; `cargo deny check` (the bans
   check fails if `akr-api` or `akr-worker` depends on `akr-sources` or a driver crate).
3. `akr-ctl adapter verify backend/adapters/ics_banks` -> signature valid, every dictionary statement and shipped
   shape proves; `--shapes` prints the statement texts for the DBA.
4. `cargo test`, including:
   - proof corpus (`sources/tests/select_proof.rs`): at least 200 statements, each expected to pass or to fail with a
     named rule: two statements, a comment, a literal, `FOR UPDATE`, `NOLOCK`, a `WITH` clause, a union, a join, a
     subquery in each clause, `OWNER.NAME@LINK`, a synonym, a four-part name, `OPENQUERY`, a user function,
     `NEXTVAL`, a missing bound, a `WHERE` on a second column, a `GROUP BY` on two columns, `SAMPLE BLOCK` placed
     anywhere but the table reference; a property test renders random generator inputs and asserts every output proves;
   - grant policy (`judge` is pure): fixture snapshots per engine for each finding code, including a nested role chain
     to DBA, `owns_objects`, a PUBLIC link with and without an exception in force, an expired exception, and an SQL
     Server login in `sysadmin` through a Windows group;
   - governor: rows per minute within 2 percent over a five-minute test-clock run; `max_sessions` never exceeded under
     20 concurrent tasks; a blackout boundary cancels the running statement at its start; the slow-down factor steps
     down and back;
   - discovery: the copy, scratch and log classification on the fixture names; the guard keeps `min`, `max` and value
     lists off every guarded column and the deny columns unread (the statement journal has no shape naming them);
     field availability grades each DR-38 case (constant, sentinel-only, 95 percent null, 30 percent null, good,
     unassessed, absent); the integration profile validates against its schema; a run killed mid-object resumes and
     skips completed objects.
5. Curl suite `tests/curl/phase05_sources.sh` (dev users `ds@bank.test`, `sec@bank.test`, `adm@bank.test`; codes from
   `oathtool`; profile `fixtures` up):
   - As DS `POST /api/v1/sources` with `Idempotency-Key: src-0001` and `{"code":"CBS1","name":{"ar":"...","en":"Core
     banking"},"engine":"postgres","tier":"CT2","adapter_pack":"ics_banks@1.0.0","cbs_schemas":["BANKS"]}` -> 201,
     `state: registered`, `next_actions: ["set_credential","probe"]`; the same key and body -> byte-identical body with
     `x-idempotent-replay: true`; the same code with a new key -> 409 `code_taken`; `"tier":"CT3"` -> 422
     `tier_not_available`.
   - `akr-ctl source credential set CBS1 --user akr_bad ...`; `POST .../probe` -> 202 with `check_id`; polling `GET
     .../grants?check_id=` -> `result: refused`, `findings[0].code: write_privilege` with `via`; `GET
     /api/v1/sources/<id>` -> `state: refused`. `POST /api/v1/discovery/runs` -> 409 `E_SOURCE_REFUSED`.
   - Credential switched to `akr_reader`; probe -> `ok`, `readonly_level: 2`, state `active`.
   - `POST /api/v1/discovery/runs {"source_id":...,"kind":"catalogue"}` -> 202; a second one -> 409 `run_active`;
     the run completes; `GET /api/v1/discovery/profiles?source_id=&candidate=copy_candidate` lists the fixture copies.
   - `POST /api/v1/discovery/runs {"kind":"profile","objects":["BANKS.CUSTOMER"]}` before access is approved -> 422
     `E_OBJECT_NOT_APPROVED`.
   - Upload the access approval (`POST .../documents`, kind `access_approval`) -> 202; `PUT .../access` with
     `If-Match` and the objects -> 202, `route: cr_access`; as SEC `POST /api/v1/change-requests/<id>/approve` with
     step-up -> 200 `effective`; `GET /api/v1/sources/<id>` shows `access_version: 1`.
   - Profile run -> completed; `GET /api/v1/discovery/field-availability` shows `party.pep_status` with
     `needs_declaration: true` and the note `bank_declaration_required`; `GET /api/v1/discovery/profiles` contains no
     `min`, `max` or `values` on any column of the guarded names (`jq` assertion over the body); `GET
     /api/v1/discovery/integration-profile` validates against the schema (`check-jsonschema`) and its checklist shows
     `signed_budget: open`.
   - Budget: upload `budget_signed`; `PUT .../load-budget` with `rows_per_minute: 5` -> 422 `budget_out_of_bounds`;
     a valid one -> 202; DS approving its own request -> 403 `E_SELF_APPROVAL` (the four-eyes forbid wins before
     eligibility); SEC approves -> version 1 in force.
   - Cedar deny: as ADM `GET /api/v1/sources` -> 403 `E_SCOPE`; as SEC `POST /api/v1/sources` -> 403 `E_SCOPE`; as SEC
     `GET /api/v1/sources` -> 200 and `GET /api/v1/sources/<id>/grants` -> 403 `E_SCOPE`; as SEC
     `POST /api/v1/sources/<id>/pause` without step-up -> 401 `E_STEPUP_REQUIRED`, with it -> 200 and the fixture's
     `pg_stat_activity` shows no `akr-ingestd` session within 5 seconds; `POST .../resume` -> 202, then `active`.
   - `GET /api/v1/sources/<id>/documents/<a document of another source>` -> 404.
6. Swagger UI lists 21 new operations (107 with phases 01, 03 and 04), each with `x-akr-action`; `make generate-api`
   twice gives zero diff.
7. Audit: after the curl run `audit_log` holds `source.registered`, `source.grant_checked`, `source.refused`,
   `source.state_changed`, `source.paused`, `source.sessions_closed`, `source.resume_requested`,
   `source.document_uploaded`, `source.access_changed`, `load_budget.activated`, `discovery_run.requested`,
   `.completed`, `.report_sealed`, `change_request.effective`, and `authz.denied` with `self_approval`; `akr-ctl
   audit verify` passes.
8. **Read-only conformance suite** (`make conformance`; against `cbs-fixture-pg` in
   CI on every change to `sources`, against the Oracle and SQL Server fixtures nightly, Db2 Community nightly, Sybase
   by the documented rig, TS-07):
   1. `akr_bad` (write privilege, a role chain to a write-all role) -> check `refused` with `write_privilege` and the
      role path; no data statement is ever sent.
   2. The Oracle DBA-class account -> `dba_class_role`, `any_privilege`, `owns_objects`; the account owning the
      fixture schema alone -> `owns_objects`.
   3. A usable PUBLIC database link -> `db_link_usable`, refused; with a signed exception through a `source_access`
      request -> the check passes, `links_usable: 1`, `links_used: 0`, and the engine's link usage view shows no use.
   4. Forced DML inside the read-only transaction (a test-only hook that bypasses the proof) -> ORA-01456 on Oracle,
      SQLSTATE 25006 on PostgreSQL; the hook is compiled only with the `conformance` feature, never in a release.
   5. The engine's own statement log (`pg_stat_statements`, Oracle `V$SQL` by module `akr-ingestd`, SQL Server Query
      Store) after a full discovery run shows only `SELECT` texts and the session-control texts of section 3.4.
   6. A statement over the timeout is cancelled on the engine (the session shows no running statement within 2
      seconds), the object is `incomplete` with `statement_timeout`, and no wider retry appears in the log.
   7. Rows per minute and `max_sessions` hold as measured on the engine (`pg_stat_activity`, `V$SESSION`).
   8. A blackout window: no statement starts inside it.
   9. The fixture account locked by the test -> the next connect moves the source to `paused_by_bank` with
      `account_locked`; connect attempts follow the back-off; unlocked -> `active` and the pause closed.
   10. `SELECT` on an approved object revoked mid-run -> the object fails, the next health check finds `grant_missing`
       and the source moves to `paused_by_bank` with `grant_revoked`.
   11. `POST /sources/{id}/pause` during a long statement -> the engine shows the statement gone and the sessions
       closed within `sources.cancel_grace_s`; `source.sessions_closed` audited.
   12. The charset round trip on the Oracle 11.2 fixture with `WE8ISO8859P1` reports the Arabic letters lost; on
       `AR8MSWIN1256` it reports the Persian and Kurdish letters lost and the Arabic ones preserved.
9. Phase 00 to 04 tests still pass; `make zone-check` passes with and without the `fixtures` profile.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), gap item (a): Cedar form corrected in place (tools/agent/fix-g1-cedar.py); lead-verified.

### 7.1 Cedar form (gap item (a))
The Cedar policies of this phase are corrected in place: a role test is `principal.roles.contains("X")` (several
roles: `principal.roles.containsAny([...])`) in a `when` clause, never `principal in Akr::Role::"X"`, which phase 03's
schema does not model (2 occurrences rewritten; the `@role` annotations are unchanged).

Gap pass 1 (2026-10-10), run 3a, from tools/agent/gap1-run3a.md; lead-verified (tools/agent/gap1-run3a-verdict.md).

### 7.2 discovery_seal_report: payload and the jobs it enqueues (DR-366)
The payload is exactly one of `{run_id}` or `{check_id}`. Section 3.3 step 9 enqueues `{check_id}`: the job renders
the grant check as canonical JSON, signs it with `akr-sign-certificate`, stores it as an evidence object owned by the
source and writes `admin.source_grant_checks.evidence_object_id` (a check with it set is skipped). Section 3.6 enqueues
`{run_id}`: the run report of section 3.6, written to `admin.discovery_runs.report_object_id` (a run with it set is
skipped). The job table of section 4 reads "payload `{run_id}` or `{check_id}`".
For a completed profile run only, the same transaction that writes `report_object_id` also enqueues `coverage_grade`
`{reason: "profile_run"}` (phase 06 section 3.3, coalesce key `coverage_grade`) and `drift_scan {source_id, run_id}`
(phase 10 section 3.5, coalesce key `drift_scan:<source_id>`). A catalogue run and a grant check enqueue neither.

### 7.3 Calendar capture inside discovery_run (phase 09 section 3.3)
The adapter-pack hint schema (section 3.1 `patch`, section 3.6) gains the key `calendar`, which names two approved
objects of the ICS BANKS profile: the holiday table `HOLIDAY` and the currency table `CURRENCY` (the columns phase 09
section 3.3 lists). `discovery_run` gains one step at the end of a profile run: the calendar capture of phase 09
section 3.3. It reads each object whole under the row cap `calendar.capture_max_rows` (reaching the cap is the finding
`calendar_capture_truncated`, and the run is not sealed as complete), proves each statement with `SelectProof` (scope
`Data`), and refuses an object outside the approved list with `E_OBJECT_NOT_APPROVED`. Rows go to
`ref.cbs_calendar_captures` (phase 09 migration; `akr_ingest` holds INSERT) in the run's transaction. No customer,
account or posting table is read, and no new statement form or connection is added. The statements take
`StatementKind::Read` (DR-365).

### 7.4 control_totals in the adapter pack (phase 12 section 4)
The hint schema gains the optional key `control_totals`: the object names, date columns and the branch, currency,
ledger and sign columns of the bank's control totals (data, not code; read by phase 12's control-totals reader). An
absent block declares that the pack has none; it is not an error here (phase 12 readiness warns). The pack loader
validates the block like the other keys, and every object it names must be approved before any read.

### 7.5 StatementKind and the governor entry points (DR-365)
`StatementKind` is a closed enum with three variants in this phase:
- `Dictionary`: the catalogue, grant and round-trip statements of `ProofScope::Dictionary`. These are the only
  statements a `probe`, `health` or `resume` check runs while the source is not `active` (step 1).
- `Aggregate`: a data statement whose result is one aggregate row (the profile counts of section 3.6). It takes one
  token (step 4).
- `Read`: a data statement that returns rows (the code-list and calendar captures, and the loads of phase 11). It
  takes one token per row fetched (step 4).
Phase 12 adds `ControlTotals` (grouped aggregates, one token per row returned). Later phases extend, and do not change:
`SourceReader` gains `fetch_keyset`, `branch_dates` and `as_of` (phase 11 section 2), and `LoadGovernor::for_source(
source_id)` is the constructor that builds the governor of section 3.5 from the budget in force (phase 11
`ingest_load` step 1).

### 7.6 SelectProof moves to crates/sqlproof (phase 10, DR-85)
Phase 10 moves `SelectProof`, `ProofScope` and `ProvenSelect` from `crates/sources` to the pure crate
`crates/sqlproof`; `crates/sources` re-exports them, so this phase's call sites do not change. Phase 10 also adds two
forms to the grammar of section 3.4: the keyset comparison (a row-value comparison over bound parameters only) and
`FETCH FIRST :n ROWS ONLY`. Both remain single-object, single-statement `SELECT`s proven before they run. The crate
layout of this phase is the state before phase 10.

### 7.7 code_suggestions in discovery.json (phase 10 section 3.4)
The hint schema gains the optional key `code_suggestions`: a CBS code and its list mapped to a canonical code, read by
phase 10's code-map prefill. It is data; an absent key means no suggestions. (`deny_columns.json` is already part of
the pack, section 3.6.)

### 7.8 Listener reload on reconnect (phase 30, DR-322)
When the `LISTEN akr_source` connection of akr-ingestd drops and is re-established, akr-ingestd rebuilds the
`LoadGovernor` of every source from one full read of the state, budget and access rows before it waits again. A
notification is a hint and carries nothing the reload depends on. The hourly re-read stays as the fallback.

### 7.9 Resume of a halted source (phase 11)
Phase 11 extends `POST /sources/{id}/resume` and owns the extension: for a source in its `halted` state the route
enqueues `ingest_resume` (phase 11 job table) after a passing grant check, returns 202, and needs step-up class
`backfill`. A `paused` source resumes as section 3.1 states.

### 7.10 Fixture additions (phase 11)
Phase 11 adds to `cbs-fixture-pg` a `PROCESS` table with a per-branch `BANK_DATE`, a `TRANSACT`-shaped table of about
one million rows over ten business days, and the `restore-to` mode of `akr-ctl fixtures cbs`. All values are
invented; nothing comes from `cbs-discovery/`.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.11 Permits in use at a listener reload (V-05-08-a, MEDIUM; DR-413)
A rebuild of a `LoadGovernor` (the reload of 7.8 and the rebuilds on NOTIFY in section 3.5) does not release a permit
that is already out. The permit's session still counts against `max_sessions` in the rebuilt governor (section 3.5
step 3), and the rebuilt rate bucket keeps the level it had (step 4). A rebuild therefore grants no session and no
rows per minute that the budget does not allow.

### 7.12 Coverage gap and backfill: the blind period, not the pause row (V-05-11-a, MEDIUM; DR-413)
The coverage gap is the source blind period of `admin.blind_periods` (phase 11 section 3.6, cause `source_paused`).
Phase 11 opens it in the transaction of the pause, the bank pause or the refusal, and phase 12 reads that period
(its section 3.10), not `admin.source_pauses`. No backfill is planned from the pause row: phase 11 builds none (its
7.4, DR-380), and a paused interval is caught up by the automatic catch-up on resume (C-16). `admin.source_pauses`
stays the record of who paused a source, when and why. This block replaces each base sentence that says the pause row
feeds a coverage gap or a backfill: the C-15 item of the header, the section 1 prose and SQL comment, the resume
paragraph of section 3.1 and the "Paused by the bank" paragraph of section 3.5.

## What This Phase Does NOT Touch

- No raw landing, no watermarks, batches or quarantine items, no mapping, no backfill planner (landing is phase 11, DR-63; no phase builds a backfill planner, DR-380):
  discovery reads counts and statistics only and writes nothing but its own `admin` rows; `lag_ceiling_s` is stored,
  not enforced.
- No CT3 change data capture and no CT4 file delivery (phase 11); CT4 as a bank landing database is a source like any
  other here.
- No card guard SP-48 for card data in transit (phase 11); card columns are deny-listed and never read.
- No bank calendars (phase 09): windows and blackouts are weekday and time rules in bank time.
- No readiness grades, CBI coverage gap report or PDF rendering (phase 06); no screens (phase 08).
- No canonical tables (phase 10): the attribute catalogue names them; a phase 10 test checks it.
- No change to phase 00-04 tables; the phase 04 route map gains one rule by migration, only while untouched.
