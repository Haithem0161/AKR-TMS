# Phase 02: Audit Trail & Evidence Core

**Goal:** A tamper-evident record that every later phase writes through one crate: the monthly-partitioned
`audit.audit_log` and `audit.access_event`, append-only under four database layers, each stream hash-chained without
a break across partitions; Merkle anchors every five minutes, signed by the OpenBao transit key `akr-sign-anchor` and
copied to the COMPLIANCE-locked `akr-anchors` bucket before their row is written; a verifier that checks rows, roots,
signatures per key version and the locked copies (`akr-ctl audit verify` and a weekly job); partition maintenance; and
the evidence write path (staging, antivirus, per-object envelope encryption, object lock at the class floor), the
Gotenberg rendering service with the Arabic shaping check, and the pack register, so that the phase 00 and 01 events
persist from the day this phase merges.

**Dependencies:** Phase 00 (Phase 01 runs in parallel; its events persist once both are merged)
**Macro-phase:** P0
**Complexity:** M (no business routes; the size is the chain, the anchor and verifier pair, and the evidence path)
**Binaries touched:** akr-api (`PgAuditSink`, `AccessLog`, `RequestCtx` extractor; evidence staging and the verified
read, used from phase 17) | akr-worker (`anchor_audit`, `partition_maintenance`, `audit_verify`, `audit_lock_extend`,
`evidence_scan`; `maintenance_sweep` extended; the render and pack services) | `akr-ctl` (`db harden`, `audit verify`,
`audit keys export`, `audit partitions`). akr-ingestd and akr-assess gain the sink only (their events start in phases 05
and 29)
**Requirements carried:** SP-39, SP-35 (the access-event writer, purpose and tier rules, and the coverage test; the
first sensitive routes are phase 03 onwards), SP-42 (the `audit.searched` kind; the viewer that emits it is phase 24),
SP-43 (the event catalogue with severity and SIEM flag; the export adapters are phase 25), SP-44 (retention without
limit, yearly extension of anchor locks; the RFC 3161 option and cold tablespace are phase 25 and 23), SP-58 (evidence
locked at the class floor; extension at closure and for holds in phase 23), SP-09 and SP-56 (per-object data keys, so
a locked object can be shredded by erasing its wrapped key; the shredding run is phase 23), SP-11 (signing key
versions recorded per anchor), TS-12 (audit partitions and `partition_maintenance`), TS-22 (evidence writes behind the
S3 port), TS-39 (the restore-drill record now persists), M-04 and M-59 (immutability primitives later registers and
notes rely on), RQ-11 (Gotenberg image and the Arabic shaping check)
- Gap pass 1 (coverage), implemented here: SP-40 (the verifier over chain, Merkle roots, signatures per key version and the locked copies; scheduled and on demand; a critical finding pages as P1); VC-81 (the evidence vault with object lock, per-object hash, signed anchors, retention by class, legal holds and access events).
- Gap pass 1 (coverage), partly implemented: SP-41 (the audit kind catalogue with its SIEM flags, catalogue-listed keys without personal data, the correlation id, UTC partitions); L-68 (the language tag on every evidence object).
- Gap pass 1 (coverage), not carried: VC-83 (OCR of evidence documents; DR-404).

Parallel-track note: phase 01 runs beside this phase. Its events and the phase 00 events go through `AuditSink`; the
call sites already pass their transaction (or call `emit_standalone` when there is none), so the swap from
`TracingSink` to `PgAuditSink` changes no call site.

## 1. SQLx Migration Changes

Three migrations, `backend/migrations/0008` to `0010` (canonical chain in roadmap.md, DR-23). Conventions as in phases
00 and 01. The audit tables have no foreign keys by design: an append-only record must never block on, or cascade
from, a domain table (the DBS rule). Partition bounds are UTC months.

Role layout (DR-45). `akr_audit_writer` stays NOLOGIN; `akr-ctl db provision` grants it to `akr_app`, `akr_ingest` and
`akr_assess` (membership with INHERIT), so a service writes its audit rows on its own connection, inside the same
transaction as the change they record. Role membership is cluster-level, so like LOGIN and role settings it is
provisioning, not a migration (DR-22). The writer can read four columns of each stream (`id`, `occurred_at`,
`prev_hash`, `entry_hash`), which is all the chain head and the anchor job need: an application role holding the writer role still
cannot read audit details or the access log. Everything else is read through `akr_readonly`.

The tables are owned by `akr_migrate`. An owner bypasses grants, so the row and TRUNCATE triggers carry the weight
against it (they are set `ENABLE ALWAYS`, so `session_replication_role = replica` does not silence them); a superuser
can still bypass everything (`event_triggers = off`, dropping the event triggers, then altering the tables). That is
why the trail is tamper-evident rather than tamper-proof: the signed anchors in the WORM bucket make any rewrite
provable after the fact (research file 30 s.10.1).

```sql
-- migration 0008_audit_streams_anchors.sql
-- Phase 02. The append-only audit streams, their partition factory and the signed anchors. Four layers (DBS
-- migration 0004 and its phase-02 s.7.1): 1 grants (INSERT is the only write privilege that exists);
-- 2 BEFORE UPDATE OR DELETE row trigger (cloned to every partition); 3 BEFORE TRUNCATE statement trigger on the parent
-- AND on every partition (statement triggers are not cloned); 4 event triggers against DROP and against ALTER,
-- DETACH and DISABLE TRIGGER, created by `akr-ctl db harden` because CREATE EVENT TRIGGER needs a superuser.

CREATE OR REPLACE FUNCTION audit.block_mutation() RETURNS trigger
LANGUAGE plpgsql
SET search_path = pg_catalog
AS $$
BEGIN
    RAISE EXCEPTION 'audit.% is append-only (% refused)', TG_TABLE_NAME, TG_OP
        USING ERRCODE = 'insufficient_privilege';
END $$;

-- Layer 4, functions only. `akr-ctl db harden` creates the event triggers that call them (section 4).
-- sql_drop: any table, trigger, function or schema object dropped inside `audit`. Partitions are matched by schema,
-- not by name, because they are created at run time.
CREATE OR REPLACE FUNCTION audit.block_drop() RETURNS event_trigger
LANGUAGE plpgsql
SET search_path = pg_catalog
AS $$
DECLARE obj RECORD;
BEGIN
    FOR obj IN SELECT * FROM pg_catalog.pg_event_trigger_dropped_objects() LOOP
        IF obj.schema_name = 'audit' AND obj.object_type IN ('table', 'trigger', 'function', 'schema') THEN
            RAISE EXCEPTION '% % is append-only (DROP refused)', obj.object_type, obj.object_identity
                USING ERRCODE = 'insufficient_privilege';
        END IF;
    END LOOP;
END $$;

-- ddl_command_end: ALTER TABLE (DETACH PARTITION removes the cloned row trigger; DISABLE TRIGGER switches a layer
-- off), ALTER TRIGGER, CREATE TRIGGER, ALTER FUNCTION and CREATE FUNCTION (CREATE OR REPLACE could turn
-- block_mutation into a no-op) inside `audit`. A release migration that must change an
-- audit table sets `SET LOCAL akr.audit_ddl = 'on'` (migrations rule); the partition factory sets it for its own
-- CREATE TRIGGER. The setting is a guard against mistakes and casual tampering, not against an owner who knows it:
-- the anchors are the proof.
CREATE OR REPLACE FUNCTION audit.block_alter() RETURNS event_trigger
LANGUAGE plpgsql
SET search_path = pg_catalog
AS $$
DECLARE cmd RECORD;
BEGIN
    IF pg_catalog.current_setting('akr.audit_ddl', true) = 'on' THEN
        RETURN;
    END IF;
    FOR cmd IN SELECT * FROM pg_catalog.pg_event_trigger_ddl_commands() LOOP
        IF cmd.schema_name = 'audit' THEN
            RAISE EXCEPTION '% on % refused: audit objects are append-only', cmd.command_tag, cmd.object_identity
                USING ERRCODE = 'insufficient_privilege';
        END IF;
    END LOOP;
END $$;

CREATE TABLE IF NOT EXISTS audit.audit_log (
    id            UUID NOT NULL,                -- UUIDv7 from the writer
    occurred_at   TIMESTAMPTZ NOT NULL,         -- assigned by the writer under the stream lock: strictly increasing
    actor_id      UUID,                         -- admin.users id; NULL only for an unauthenticated act (failed sign-in)
    on_behalf_of  UUID,                         -- the delegator when the act used a delegated role (phase 01)
    action        TEXT NOT NULL CHECK (action ~ '^[a-z][a-z_]*(\.[a-z][a-z_]*)+$'),   -- the event kind
    resource_type TEXT NOT NULL CHECK (resource_type ~ '^[a-z][a-z_]*$'),           -- from the catalogue, per kind
    resource_id   UUID,
    resource_key  TEXT CHECK (length(resource_key) BETWEEN 1 AND 200),   -- non-UUID resources: setting and flag keys
    case_id       UUID,                         -- filled from phase 17 for case-linked acts
    decision      JSONB CHECK (decision IS NULL OR jsonb_typeof(decision) = 'object'),   -- Cedar, from phase 03
    request       JSONB NOT NULL CHECK (jsonb_typeof(request) = 'object'),   -- ip, session_id, token_id, job_id, correlation_id, ua
    details       JSONB NOT NULL CHECK (jsonb_typeof(details) = 'object'),   -- catalogue-listed keys only; no personal data
    prev_hash     BYTEA NOT NULL CHECK (octet_length(prev_hash) = 32),
    entry_hash    BYTEA NOT NULL CHECK (octet_length(entry_hash) = 32),
    PRIMARY KEY (id, occurred_at),              -- the partition key must be in every unique constraint
    CHECK (on_behalf_of IS NULL OR actor_id IS NOT NULL)
) PARTITION BY RANGE (occurred_at);

-- (occurred_at, id) serves the chain head (ORDER BY occurred_at DESC, id DESC LIMIT 1), anchor ranges and the
-- verifier's ordered walk; it replaces the BRIN index of the DATA-MODEL sketch.
CREATE INDEX IF NOT EXISTS idx_audit_log_chain    ON audit.audit_log (occurred_at, id);
CREATE INDEX IF NOT EXISTS idx_audit_log_actor    ON audit.audit_log (actor_id, occurred_at);
CREATE INDEX IF NOT EXISTS idx_audit_log_resource ON audit.audit_log (resource_type, resource_id, occurred_at);
CREATE INDEX IF NOT EXISTS idx_audit_log_res_key  ON audit.audit_log (resource_type, resource_key, occurred_at)
    WHERE resource_key IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_audit_log_case     ON audit.audit_log (case_id, occurred_at) WHERE case_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_audit_log_action   ON audit.audit_log (action, occurred_at);

CREATE TABLE IF NOT EXISTS audit.access_event (
    id             UUID NOT NULL,
    occurred_at    TIMESTAMPTZ NOT NULL,        -- as audit_log: strictly increasing within the stream
    actor_id       UUID NOT NULL,               -- the person; never a shared account
    actor_role     TEXT NOT NULL CHECK (actor_role ~ '^[A-Z][A-Z0-9]{0,3}$'),   -- the role the access relied on
    on_behalf_of   UUID,
    resource_type  TEXT NOT NULL CHECK (resource_type ~ '^[a-z][a-z_]*$'),     -- customer, account, alert, case, evidence, report, ...
    resource_id    UUID NOT NULL,               -- pseudonymous id, never a name
    subject_party  UUID,                        -- the customer the resource is about
    action         TEXT NOT NULL CHECK (action IN ('view', 'search_hit', 'export', 'print', 'download', 'unmask')),
    -- TEXT, not an enum: an append-only table must not depend on a type another schema may alter.
    tier           TEXT NOT NULL CHECK (tier IN ('normal', 'restricted', 'suspicion')),
    purpose_code   TEXT CHECK (purpose_code ~ '^[a-z][a-z0-9_]{1,63}$'),
    reason         BYTEA,                       -- free text, sealed under the `personal` family (AAD = id)
    decision       TEXT NOT NULL CHECK (decision IN ('allow', 'deny', 'conflict')),
    session_id     UUID,
    token_id       UUID,                        -- personal token or client (phase 01); NULL for a session
    client_ip      INET,
    correlation_id UUID NOT NULL,
    prev_hash      BYTEA NOT NULL CHECK (octet_length(prev_hash) = 32),
    entry_hash     BYTEA NOT NULL CHECK (octet_length(entry_hash) = 32),
    PRIMARY KEY (id, occurred_at),
    CHECK (session_id IS NOT NULL OR token_id IS NOT NULL),
    -- SP-35: an allowed read of a restricted or suspicion-tier record states its purpose.
    CHECK (tier = 'normal' OR decision <> 'allow' OR purpose_code IS NOT NULL)
) PARTITION BY RANGE (occurred_at);

CREATE INDEX IF NOT EXISTS idx_access_event_chain    ON audit.access_event (occurred_at, id);
CREATE INDEX IF NOT EXISTS idx_access_event_actor    ON audit.access_event (actor_id, occurred_at);
CREATE INDEX IF NOT EXISTS idx_access_event_resource ON audit.access_event (resource_type, resource_id, occurred_at);
CREATE INDEX IF NOT EXISTS idx_access_event_subject  ON audit.access_event (subject_party, occurred_at)
    WHERE subject_party IS NOT NULL;

-- Layer 2 on the parents (cloned to every partition, present and future). BEFORE row triggers on partitioned
-- tables need PostgreSQL 13 or later. CREATE TRIGGER has no IF NOT EXISTS: guarded.
DO $$ BEGIN
    CREATE TRIGGER trg_audit_log_immutable BEFORE UPDATE OR DELETE ON audit.audit_log
        FOR EACH ROW EXECUTE FUNCTION audit.block_mutation();
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    CREATE TRIGGER trg_access_event_immutable BEFORE UPDATE OR DELETE ON audit.access_event
        FOR EACH ROW EXECUTE FUNCTION audit.block_mutation();
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
-- Layer 3 on the parents; the factory adds it to each partition.
DO $$ BEGIN
    CREATE TRIGGER trg_no_truncate BEFORE TRUNCATE ON audit.audit_log
        FOR EACH STATEMENT EXECUTE FUNCTION audit.block_mutation();
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    CREATE TRIGGER trg_no_truncate BEFORE TRUNCATE ON audit.access_event
        FOR EACH STATEMENT EXECUTE FUNCTION audit.block_mutation();
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

-- ENABLE ALWAYS: the triggers fire even under session_replication_role = replica, which otherwise silences ordinary
-- triggers. Cloned row triggers take the parent's setting; `akr-ctl db harden` checks every partition anyway.
ALTER TABLE audit.audit_log    ENABLE ALWAYS TRIGGER trg_audit_log_immutable;
ALTER TABLE audit.access_event ENABLE ALWAYS TRIGGER trg_access_event_immutable;
ALTER TABLE audit.audit_log    ENABLE ALWAYS TRIGGER trg_no_truncate;
ALTER TABLE audit.access_event ENABLE ALWAYS TRIGGER trg_no_truncate;

-- The partition factory. SECURITY DEFINER (runs as akr_migrate, the owner) so the writer role can create a missing
-- month without DDL rights. search_path is pinned to pg_catalog and every object is schema-qualified: an unqualified
-- name inside a SECURITY DEFINER function would resolve with the caller's search_path. An advisory lock serialises
-- concurrent callers (two writers meeting a missing month at once). Returns the partition name.
CREATE OR REPLACE FUNCTION audit.create_partition(p_table TEXT, p_year INT, p_month INT) RETURNS TEXT
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog
AS $$
DECLARE
    v_from DATE;
    v_to   DATE;
    v_name TEXT;
BEGIN
    IF p_table NOT IN ('audit_log', 'access_event') THEN
        RAISE EXCEPTION 'audit.create_partition: % is not a partitioned audit table', p_table;
    END IF;
    IF p_year NOT BETWEEN 2024 AND 2200 OR p_month NOT BETWEEN 1 AND 12 THEN
        RAISE EXCEPTION 'audit.create_partition: month %-% out of range', p_year, p_month;
    END IF;
    PERFORM pg_catalog.pg_advisory_xact_lock(7307418562110395001);   -- fixed key: 'akr.audit.partition_factory'
    v_from := pg_catalog.make_date(p_year, p_month, 1);
    v_to   := (v_from + interval '1 month')::date;
    v_name := pg_catalog.format('%s_%s_%s', p_table, p_year, pg_catalog.lpad(p_month::text, 2, '0'));
    EXECUTE pg_catalog.format(
        'CREATE TABLE IF NOT EXISTS audit.%I PARTITION OF audit.%I FOR VALUES FROM (%L) TO (%L)',
        v_name, p_table, v_from::text || ' 00:00:00+00', v_to::text || ' 00:00:00+00');
    IF NOT EXISTS (
        SELECT 1 FROM pg_catalog.pg_trigger t
        JOIN pg_catalog.pg_class c ON c.oid = t.tgrelid
        JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'audit' AND c.relname = v_name AND t.tgname = 'trg_no_truncate'
    ) THEN
        PERFORM pg_catalog.set_config('akr.audit_ddl', 'on', true);
        EXECUTE pg_catalog.format(
            'CREATE TRIGGER trg_no_truncate BEFORE TRUNCATE ON audit.%I FOR EACH STATEMENT '
            'EXECUTE FUNCTION audit.block_mutation()', v_name);
        EXECUTE pg_catalog.format('ALTER TABLE audit.%I ENABLE ALWAYS TRIGGER trg_no_truncate', v_name);
        PERFORM pg_catalog.set_config('akr.audit_ddl', 'off', true);
    END IF;
    RETURN v_name;
END $$;

REVOKE ALL ON FUNCTION audit.create_partition(TEXT, INT, INT) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION audit.create_partition(TEXT, INT, INT) TO akr_audit_writer;

-- Seed partitions: the current month and the three ahead (audit.partition_ahead_months), through the factory so
-- every seeded partition carries its TRUNCATE trigger.
DO $$
DECLARE m DATE := date_trunc('month', now() AT TIME ZONE 'UTC')::date;
BEGIN
    FOR i IN 0..3 LOOP
        PERFORM audit.create_partition('audit_log',    extract(year FROM m + make_interval(months => i))::int,
                                                       extract(month FROM m + make_interval(months => i))::int);
        PERFORM audit.create_partition('access_event', extract(year FROM m + make_interval(months => i))::int,
                                                       extract(month FROM m + make_interval(months => i))::int);
    END LOOP;
END $$;

-- One signed anchor per (stream, partition, range). Not partitioned: about 300 rows a day per stream at most.
-- Anchors form a chain per stream (prev_anchor_id), so a deleted anchor row is itself detectable.
CREATE TABLE IF NOT EXISTS audit.audit_anchors (
    id                UUID PRIMARY KEY,
    stream            TEXT NOT NULL CHECK (stream IN ('audit_log', 'access_event', 'case_event')),
    partition_name    TEXT NOT NULL,
    range_from        TIMESTAMPTZ NOT NULL,     -- occurred_at of the first entry covered (inclusive)
    range_to          TIMESTAMPTZ NOT NULL,     -- occurred_at of the last entry covered (inclusive)
    first_entry_id    UUID NOT NULL,
    last_entry_id     UUID NOT NULL,
    last_entry_hash   BYTEA NOT NULL CHECK (octet_length(last_entry_hash) = 32),
    entry_count       BIGINT NOT NULL CHECK (entry_count > 0),
    merkle_root       BYTEA NOT NULL CHECK (octet_length(merkle_root) = 32),
    prev_anchor_id    UUID REFERENCES audit.audit_anchors(id) ON DELETE RESTRICT,
    statement_sha256  BYTEA NOT NULL CHECK (octet_length(statement_sha256) = 32),   -- of the exact signed bytes
    signature         BYTEA NOT NULL CHECK (octet_length(signature) = 64),          -- Ed25519
    key_name          TEXT NOT NULL CHECK (key_name = 'akr-sign-anchor'),
    key_version       INT NOT NULL CHECK (key_version > 0),
    anchored_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    object_key        TEXT NOT NULL,            -- anchors/<stream>/<partition>/<range_to micros>.json in akr-anchors
    object_version_id TEXT NOT NULL,            -- the store's version id: the verifier reads exactly this version
    CHECK (range_to >= range_from),
    CHECK (entry_count > 1 OR (range_to = range_from AND first_entry_id = last_entry_id))
);
-- The anchor chain is linear: one genesis per stream, and no anchor has two successors.
CREATE UNIQUE INDEX IF NOT EXISTS uq_anchor_genesis ON audit.audit_anchors (stream) WHERE prev_anchor_id IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uq_anchor_prev    ON audit.audit_anchors (prev_anchor_id)
    WHERE prev_anchor_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_anchor_stream_range  ON audit.audit_anchors (stream, range_to DESC);
CREATE INDEX IF NOT EXISTS idx_anchor_partition     ON audit.audit_anchors (partition_name, range_from);

DO $$ BEGIN
    CREATE TRIGGER trg_audit_anchors_immutable BEFORE UPDATE OR DELETE ON audit.audit_anchors
        FOR EACH ROW EXECUTE FUNCTION audit.block_mutation();
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    CREATE TRIGGER trg_no_truncate BEFORE TRUNCATE ON audit.audit_anchors
        FOR EACH STATEMENT EXECUTE FUNCTION audit.block_mutation();
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
ALTER TABLE audit.audit_anchors ENABLE ALWAYS TRIGGER trg_audit_anchors_immutable;
ALTER TABLE audit.audit_anchors ENABLE ALWAYS TRIGGER trg_no_truncate;

-- Layer 1. INSERT is the only write privilege anywhere. The writer reads the three chain columns only; the
-- verifier, viewer and reports read everything through akr_readonly. (The anchor job reads prev_hash too, for its
-- continuity check.)
REVOKE ALL ON audit.audit_log, audit.access_event, audit.audit_anchors FROM PUBLIC;
GRANT INSERT ON audit.audit_log, audit.access_event, audit.audit_anchors TO akr_audit_writer;
GRANT SELECT (id, occurred_at, prev_hash, entry_hash) ON audit.audit_log    TO akr_audit_writer;
GRANT SELECT (id, occurred_at, prev_hash, entry_hash) ON audit.access_event TO akr_audit_writer;
GRANT SELECT ON audit.audit_anchors TO akr_audit_writer;
GRANT SELECT ON audit.audit_log, audit.access_event, audit.audit_anchors TO akr_readonly;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification, encrypted, note)
VALUES
 ('audit', 'audit_log',    'request',   'personal', FALSE, 'Staff client IP and user agent; no customer data'),
 ('audit', 'access_event', 'reason',    'personal', TRUE,  'Free-text access reason, sealed'),
 ('audit', 'access_event', 'client_ip', 'personal', FALSE, 'Staff client IP')
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'audit', v.t, c, 'internal'
FROM (VALUES
  ('audit_log',     ARRAY['id','occurred_at','actor_id','on_behalf_of','action','resource_type','resource_id',
                          'resource_key','case_id','decision','details','prev_hash','entry_hash']),
  ('access_event',  ARRAY['id','occurred_at','actor_id','actor_role','on_behalf_of','resource_type','resource_id',
                          'subject_party','action','tier','purpose_code','decision','session_id','token_id',
                          'correlation_id','prev_hash','entry_hash']),
  ('audit_anchors', ARRAY['id','stream','partition_name','range_from','range_to','first_entry_id','last_entry_id',
                          'last_entry_hash','entry_count','merkle_root','prev_anchor_id','statement_sha256',
                          'signature','key_name','key_version','anchored_at','object_key','object_version_id'])
) AS v(t, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('audit', 'audit_log',     'R10', 'never', NULL, 'Unlimited (SP-44); cold storage of old partitions, never deletion'),
 ('audit', 'access_event',  'R10', 'never', NULL, 'Unlimited, as audit_log (SP-35)'),
 ('audit', 'audit_anchors', 'R10', 'never', NULL, 'Unlimited; the WORM copies are locked for audit.anchor_retention_days and extended yearly')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

The partition children inherit the parents' classification and retention rows: the phase 00 coverage tests skip
relations with `relispartition = true` (a change to the phase 00 schema check, section 4).

```sql
-- migration 0009_evidence_objects_packs.sql
-- Phase 02. The evidence register (one row per stored object, DR-44) and the pack register (P1 packs write here; the
-- first is the CBI gap report of phase 06).

DO $$ BEGIN
    CREATE TYPE admin.av_status AS ENUM ('pending', 'clean', 'infected', 'rejected');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS admin.evidence_objects (
    id                UUID PRIMARY KEY,
    kind              TEXT NOT NULL CHECK (kind IN ('upload', 'render', 'snapshot', 'package', 'certificate', 'pack')),
    owner_kind        TEXT NOT NULL CHECK (owner_kind ~ '^[a-z][a-z_]*$'),   -- case, alert, report, ops_task, pack, ...
    owner_id          UUID NOT NULL,            -- no FK: owners live in many tables; checked by the owning service
    retention_class   TEXT NOT NULL CHECK (retention_class ~ '^R([1-9]|1[0-3])$'),   -- the owner's class
    bucket            TEXT NOT NULL CHECK (bucket IN ('akr-quarantine', 'akr-evidence')),
    object_key        TEXT NOT NULL CHECK (length(object_key) BETWEEN 1 AND 512),
    version_id        TEXT,                     -- the final object's version id (NULL while staged)
    sha256            BYTEA NOT NULL CHECK (octet_length(sha256) = 32),   -- of the plaintext, before sealing
    size_bytes        BIGINT NOT NULL CHECK (size_bytes > 0),
    content_type      TEXT NOT NULL CHECK (content_type ~ '^[a-z]+/[a-z0-9.+-]+$'),
    file_name         BYTEA,                    -- the uploader's file name, sealed (`personal`, AAD = id)
    classification    admin.data_classification NOT NULL,
    lang              TEXT NOT NULL CHECK (lang IN ('ar', 'en', 'ku', 'mixed', 'und')),
    av_status         admin.av_status NOT NULL DEFAULT 'pending',
    av_engine_version TEXT,                     -- clamd VERSION at scan time (engine / signature database / date)
    av_signature      TEXT,                     -- detection name when infected; rejection code when rejected
    scanned_at        TIMESTAMPTZ,
    lock_mode         TEXT CHECK (lock_mode IN ('COMPLIANCE', 'GOVERNANCE')),
    lock_until        TIMESTAMPTZ,              -- extended only (phase 23); never shortened
    wrapped_dek       BYTEA,                    -- the object's own data key wrapped by akr-kek; erased = shredded
    key_version       INT CHECK (key_version > 0),   -- akr-kek version that wrapped it
    shredded_at       TIMESTAMPTZ,              -- set when wrapped_dek is erased (retention, phase 23)
    created_by        UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK ((av_status = 'clean') = (bucket = 'akr-evidence')),
    CHECK (av_status <> 'clean' OR (version_id IS NOT NULL AND scanned_at IS NOT NULL AND lock_mode IS NOT NULL
                                    AND lock_until IS NOT NULL AND key_version IS NOT NULL
                                    AND (wrapped_dek IS NOT NULL OR shredded_at IS NOT NULL))),
    CHECK ((av_status IN ('infected', 'rejected')) = (av_signature IS NOT NULL)),
    CHECK (shredded_at IS NULL OR wrapped_dek IS NULL),
    UNIQUE (bucket, object_key)
);
CREATE INDEX IF NOT EXISTS idx_evidence_owner   ON admin.evidence_objects (owner_kind, owner_id, created_at);
CREATE INDEX IF NOT EXISTS idx_evidence_sha256  ON admin.evidence_objects (sha256);
CREATE INDEX IF NOT EXISTS idx_evidence_pending ON admin.evidence_objects (created_at) WHERE av_status = 'pending';
CREATE INDEX IF NOT EXISTS idx_evidence_created_by ON admin.evidence_objects (created_by);

-- Facts never change; the state columns move in one direction only. Column grants stop the first, this trigger the
-- second: clean is terminal, a set data key can only be erased, a lock can only be extended.
CREATE OR REPLACE FUNCTION admin.evidence_objects_guard() RETURNS trigger
LANGUAGE plpgsql
SET search_path = pg_catalog
AS $$
BEGIN
    IF OLD.av_status <> 'pending' AND NEW.av_status <> OLD.av_status THEN
        RAISE EXCEPTION 'evidence %: av_status % is terminal', OLD.id, OLD.av_status;
    END IF;
    IF OLD.wrapped_dek IS NOT NULL AND NEW.wrapped_dek IS NOT NULL AND NEW.wrapped_dek <> OLD.wrapped_dek THEN
        RAISE EXCEPTION 'evidence %: a data key can be erased, never replaced', OLD.id;
    END IF;
    IF OLD.shredded_at IS NOT NULL AND NEW.wrapped_dek IS NOT NULL THEN
        RAISE EXCEPTION 'evidence %: shredded', OLD.id;
    END IF;
    IF OLD.lock_until IS NOT NULL AND (NEW.lock_until IS NULL OR NEW.lock_until < OLD.lock_until) THEN
        RAISE EXCEPTION 'evidence %: lock_until can only be extended', OLD.id;
    END IF;
    IF OLD.version_id IS NOT NULL AND NEW.version_id IS DISTINCT FROM OLD.version_id THEN
        RAISE EXCEPTION 'evidence %: version_id is fixed once stored', OLD.id;
    END IF;
    RETURN NEW;
END $$;

DO $$ BEGIN
    CREATE TRIGGER trg_evidence_objects_guard BEFORE UPDATE ON admin.evidence_objects
        FOR EACH ROW EXECUTE FUNCTION admin.evidence_objects_guard();
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS admin.packs (
    id           UUID PRIMARY KEY,
    kind         TEXT NOT NULL CHECK (kind IN ('board', 'case', 'report', 'assessor', 'correspondent', 'validator',
                                               'cbi_gap')),
    scope        JSONB NOT NULL CHECK (jsonb_typeof(scope) = 'object'),   -- {"case_id": ...}, {"period": ...}, ...
    scope_key    BYTEA NOT NULL CHECK (octet_length(scope_key) = 32),     -- SHA-256 of the canonical scope
    lang         TEXT NOT NULL CHECK (lang IN ('ar', 'en')),
    version      INT NOT NULL CHECK (version > 0),                       -- per (kind, scope_key, lang)
    evidence_id  UUID NOT NULL REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    sha256       BYTEA NOT NULL CHECK (octet_length(sha256) = 32),       -- equals the evidence object's sha256
    -- What the pack was built from: generator name and version, the ids and data_as_of of every fact it shows,
    -- the template version. Ids and codes only.
    manifest     JSONB NOT NULL CHECK (jsonb_typeof(manifest) = 'object'),
    created_by   UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (kind, scope_key, lang, version)
);
CREATE INDEX IF NOT EXISTS idx_packs_kind_created ON admin.packs (kind, created_at DESC, id DESC);
CREATE INDEX IF NOT EXISTS idx_packs_evidence     ON admin.packs (evidence_id);
CREATE INDEX IF NOT EXISTS idx_packs_created_by   ON admin.packs (created_by);

GRANT SELECT, INSERT ON admin.evidence_objects TO akr_app;
GRANT UPDATE (bucket, object_key, version_id, av_status, av_engine_version, av_signature, scanned_at, lock_mode,
              lock_until, wrapped_dek, key_version, shredded_at, updated_at)
    ON admin.evidence_objects TO akr_app;
GRANT SELECT, INSERT ON admin.packs TO akr_app;     -- a pack is never edited; a new version is a new row
-- The register for the viewer and assessor: never the data key or the sealed file name.
GRANT SELECT (id, kind, owner_kind, owner_id, retention_class, bucket, object_key, version_id, sha256, size_bytes,
              content_type, classification, lang, av_status, av_engine_version, av_signature, scanned_at, lock_mode,
              lock_until, key_version, shredded_at, created_by, created_at)
    ON admin.evidence_objects TO akr_readonly;
GRANT SELECT ON admin.packs TO akr_readonly;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification, encrypted, note)
VALUES
 ('admin', 'evidence_objects', 'file_name',   'personal',   TRUE,  'Uploader-supplied file name; may name a customer'),
 ('admin', 'evidence_objects', 'wrapped_dek', 'secret-str', FALSE, 'Object data key wrapped by akr-kek; erasing it shreds the object')
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', v.t, c, 'internal'
FROM (VALUES
  ('evidence_objects', ARRAY['id','kind','owner_kind','owner_id','retention_class','bucket','object_key','version_id',
                             'sha256','size_bytes','content_type','classification','lang','av_status',
                             'av_engine_version','av_signature','scanned_at','lock_mode','lock_until','key_version',
                             'shredded_at','created_by','created_at','updated_at']),
  ('packs',            ARRAY['id','kind','scope','scope_key','lang','version','evidence_id','sha256','manifest',
                             'created_by','created_at'])
) AS v(t, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'evidence_objects', 'R6', 'owner', NULL,         'Per row: retention_class and the owner''s clock; locked objects are shredded, not deleted, before their lock ends'),
 ('admin', 'packs',            'R6', 'age',   'created_at', 'Board, assessor and report packs; case packs follow the case (phase 17)')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

```sql
-- migration 0010_audit_evidence_settings.sql
-- Phase 02. Audit, anchor, evidence and render settings. Hard bounds live in the settings registry (DR-34); the
-- regulatory floor on evidence locks is M-05 (7 years for case, alert, decision and report classes).

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('audit.partition_ahead_months',   '3', 'integer', NULL, FALSE,
  '{"ar": "عدد الأشهر المُنشأة مسبقاً لسجل التدقيق", "en": "Audit partitions created ahead (months)"}'),
 ('audit.anchor_max_entries',       '100000', 'integer', NULL, FALSE,
  '{"ar": "أقصى عدد قيود في المرساة الواحدة", "en": "Maximum entries per anchor"}'),
 ('audit.anchor_lag_alert_s',       '900', 'duration_s', NULL, FALSE,
  '{"ar": "مهلة التنبيه عند تأخر ختم سجل التدقيق (ثوانٍ)", "en": "Anchor lag alert threshold (seconds)"}'),
 ('audit.anchor_retention_days',    '3650', 'integer', NULL, TRUE,
  '{"ar": "مدة قفل نسخ المراسي غير القابلة للتعديل (أيام)", "en": "Anchor copy lock period (days)"}'),
 ('audit.anchor_extend_below_days', '1825', 'integer', NULL, TRUE,
  '{"ar": "تمديد قفل المرساة عندما يقل المتبقي عن (أيام)", "en": "Extend an anchor lock when less than this remains (days)"}'),
 ('schedule.anchor_audit',          '"0 */5 * * * *"', 'cron', NULL, TRUE,
  '{"ar": "موعد ختم سجل التدقيق", "en": "Audit anchoring schedule"}'),
 ('schedule.partition_maintenance', '"0 15 1 * * *"', 'cron', NULL, FALSE,
  '{"ar": "موعد صيانة أقسام الجداول", "en": "Partition maintenance time"}'),
 ('schedule.audit_verify',          '"0 0 4 * * 6"', 'cron', NULL, FALSE,
  '{"ar": "موعد التحقق الأسبوعي من سلسلة التدقيق", "en": "Weekly audit chain verification time"}'),
 ('schedule.audit_lock_extend',     '"0 30 4 1 * *"', 'cron', NULL, FALSE,
  '{"ar": "موعد تمديد أقفال المراسي", "en": "Anchor lock extension time"}'),
 ('evidence.max_bytes',             '52428800', 'integer', NULL, TRUE,
  '{"ar": "أقصى حجم لملف الدليل (بايت)", "en": "Maximum evidence file size (bytes)"}'),
 ('evidence.allowed_types',
  '["application/pdf", "image/png", "image/jpeg", "image/tiff", "text/plain", "text/csv", "message/rfc822", "application/vnd.openxmlformats-officedocument.wordprocessingml.document", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"]',
  'json', NULL, TRUE,
  '{"ar": "أنواع الملفات المسموح بها كأدلة", "en": "Allowed evidence file types"}'),
 ('evidence.lock_days',
  '{"R3": 2555, "R5": 2555, "R6": 2555, "R7": 2555, "R8": 2555, "R9": 3650, "R10": 3650, "R11": 3650}',
  'json',
  '{"min": 2555, "applies_to": ["R5", "R6", "R7", "R8"], "basis": "CBI Detailed Assessment Guide D2.1(c)(1): 7 years (M-05)"}',
  TRUE,
  '{"ar": "مدة القفل الأولي لملفات الأدلة حسب فئة الاحتفاظ (أيام)", "en": "Initial evidence lock by retention class (days)"}'),
 ('evidence.av_max_signature_age_h', '72', 'integer', NULL, TRUE,
  '{"ar": "أقصى عمر مقبول لتواقيع مكافحة الفيروسات (ساعات)", "en": "Maximum antivirus signature age (hours)"}'),
 ('evidence.staging_ttl_h',         '24', 'integer', NULL, FALSE,
  '{"ar": "مدة بقاء الملفات المرحلية اليتيمة (ساعات)", "en": "Orphaned staging object lifetime (hours)"}'),
 ('evidence.dek_cache_s',           '300', 'duration_s', NULL, FALSE,
  '{"ar": "مدة الاحتفاظ بمفاتيح الأدلة في الذاكرة (ثوانٍ)", "en": "Evidence key cache lifetime (seconds)"}'),
 ('render.timeout_s',               '60', 'duration_s', NULL, FALSE,
  '{"ar": "مهلة إنشاء ملف PDF (ثوانٍ)", "en": "PDF rendering time-out (seconds)"}')
ON CONFLICT (key) DO NOTHING;
```

Notes:
- `audit_log.occurred_at` has no default on purpose: the writer assigns it under the stream lock so the chain order is
  the time order (section 3). A row inserted with any other time is caught by the verifier as `order_violation`.
- `case_event` is reserved in the anchor `stream` CHECK for the case event stream of phase 17, which is chained and
  anchored by the same code.
- `evidence_objects.owner_id` has no FK because owners live in many tables (cases, reports, tasks, packs); each owning
  phase checks the owner in its service and lists its `owner_kind` in the evidence owner registry (section 3).
- `evidence.lock_days` floors only the four classes M-05 names; the registry refuses a map that drops any of them
  below 2,555 days, with `E_BELOW_FLOOR`.

## 2. DDD Implementation (Rust)

Crate placement per the phase 00 workspace. The `audit` crate grows from the trait and stub into the writer, the
access log, the anchor service and the verifier; it depends on `db` (pools, transactions), `crypto` and `objectstore`,
never on axum. Evidence entities, repositories and services live in `domain::evidence`, with Pg impls in `db`. The
clamd and Gotenberg adapters are outbound adapters in `common::clamd` and `common::gotenberg` (as `common::ory` in
phase 01). The `RequestCtx` and `AccessScope` extractors live in `api`.

- **`audit` crate modules:**
  - `canonical`: `Canon` values and the hand-written encoder (section 3). `Canon` has no float variant; the
    conversion from `serde_json::Value` refuses a non-integer number (`AuditError::FloatInPayload`), a string holding
    U+0000 and a non-object `details` (`AuditError::DetailsNotAnObject`).
  - `catalogue`: `EventKind { name: &'static str, resource_type: &'static str, stream: Stream, severity: Severity,
    siem: bool, detail_keys: &'static [&'static str] }` constants and `CATALOGUE: &[&EventKind]`; phase 00, 01 and 02
    kinds now, every later phase appends its own (section 3 lists this phase's). `Severity { Info, Notice, Warning,
    Critical }`; `siem` marks the security events of research file 30 s.10.4 that phase 25 exports.
  - `event`: `AuditEvent { kind: &'static EventKind, actor: Actor, resource: ResourceRef, case_id: Option<Uuid>,
    decision: Option<DecisionRecord>, request: RequestCtx, details: BTreeMap<&'static str, Canon> }` built with
    `AuditEvent::new(&kinds::SETTING_UPDATED, actor, request).resource_key(key).detail("before", v)?`; `detail`
    refuses a key outside `kind.detail_keys`. `Actor { User { id, on_behalf_of: Option<Uuid> } | System | Anonymous }`
    (`System` writes the phase 01 system principal id). `ResourceRef { id: Option<Uuid>, key: Option<String> }`.
    `DecisionRecord { policy_ids: Vec<String>, effect: Allow | Deny, attrs_hash: [u8; 32] }` (filled from phase 03).
  - `request`: `RequestCtx { ip: Option<IpAddr>, session_id: Option<Uuid>, token_id: Option<Uuid>, job_id:
    Option<Uuid>, correlation_id: Uuid, ua: Option<String> }`; `RequestCtx::system(job_id, correlation_id, kind)`
    (`ua = "job:<kind>"`) for workers; `RequestCtx::cli(correlation_id, version)` (`ua = "akr-ctl/<version>"`) for
    `akr-ctl`. `ua` is cut to 256 characters.
  - `sink`: the trait below, `PgAuditSink`, `TracingSink` (development and tests only; refused in production by
    `Config::validate`) and `RecordingSink` (unit tests: keeps events in memory).
  - `access`: `AccessEvent { actor_id, actor_role: RoleCode, on_behalf_of, resource_type: &'static str, resource_id,
    subject_party, action: AccessAction, tier: Tier, purpose_code: Option<PurposeCode>, reason: Option<String>,
    decision: AccessDecision, session_id, token_id, client_ip, correlation_id }`; `AccessLog` trait and
    `PgAccessLog`.
  - `chain`: stream lock keys, head read, `entry_hash`; `merkle`: tree and inclusion proofs; `anchor`:
    `AnchorService`; `verify`: `Verifier` and `VerifyReport`; `partition`: `PartitionRegistry` and the maintenance
    routine.
- **Sink trait** (pinned here; the phase 00 stub implements the same shape):

```rust
#[async_trait]
pub trait AuditSink: Send + Sync {
    /// Writes one event inside the caller's transaction, after the domain change it records.
    async fn emit(&self, tx: &mut Transaction<'_, Postgres>, event: AuditEvent) -> Result<AuditReceipt, AuditError>;
    /// Writes one event in its own transaction: acts with no domain change (a refused sign-in, a CLI check).
    async fn emit_standalone(&self, event: AuditEvent) -> Result<AuditReceipt, AuditError>;
}
pub struct AuditReceipt { pub id: Uuid, pub occurred_at: DateTime<Utc>, pub entry_hash: [u8; 32] }

#[async_trait]
pub trait AccessLog: Send + Sync {
    /// Own transaction, committed before the protected content is sent (section 3).
    async fn record(&self, events: Vec<AccessEvent>) -> Result<(), AuditError>;
}
```

- **Error enums** (`thiserror`): `AuditError { FloatInPayload { key }, DetailsNotAnObject, DetailKeyNotInCatalogue {
  kind, key }, Canonicalization(String), NotInTransaction, PartitionMissing { table, month }, Database(sqlx::Error) }`
  and, kept separate so a sink caller never handles anchor failures, `AnchorError { OpenBao(String), ObjectStore(String),
  Audit(AuditError), Database(sqlx::Error), ChainMoved }`. `EvidenceError { TypeNotAllowed { declared, sniffed },
  TooLarge { size, max }, Empty, Infected { signature }, Rejected { code }, ScannerUnavailable, SignaturesStale {
  age_h }, NoLockPolicy { class }, HashMismatch { id }, Shredded { id }, NotClean { id, status }, OwnerUnknown { kind },
  Store(String), Crypto(String), Database(sqlx::Error) }`. `RenderError { Unavailable, Timeout, Refused(String),
  FontMissing }`.
- **Domain entities** (`domain::evidence`, derives `Debug, Serialize, Deserialize, FromRow, ToSchema`):
  - `EvidenceObject { id, kind: EvidenceKind, owner_kind, owner_id, retention_class: RetentionClass, bucket,
    object_key, version_id, sha256, size_bytes, content_type, classification, lang: ContentLang, av_status: AvStatus,
    av_engine_version, av_signature, scanned_at, lock_mode: Option<LockMode>, lock_until, key_version, shredded_at,
    created_by, created_at, updated_at }` (`wrapped_dek` and the sealed `file_name` live only in a non-serialised `db`
    row struct).
  - `Pack { id, kind: PackKind, scope: Value, scope_key, lang, version, evidence_id, sha256, manifest: Value,
    created_by, created_at }`.
  - Value objects: `EvidenceKind`, `AvStatus`, `LockMode { Compliance, Governance }`, `ContentLang { Ar, En, Ku,
    Mixed, Und }`, `PackKind`, `Tier { Normal, Restricted, Suspicion }` (also used by phase 03), `AccessAction`,
    `AccessDecision { Allow, Deny, Conflict }`, `PurposeCode` (format `^[a-z][a-z0-9_]{1,63}$`; the list of codes is
    admin data from phase 03), `Stream { AuditLog, AccessEvent, CaseEvent }`.
- **Create structs** (`Deserialize, ToSchema` where they cross the API later): `NewEvidence { kind, owner_kind,
  owner_id, retention_class, content_type, file_name: Option<String>, classification, lang, bytes: Bytes }`,
  `GeneratedEvidence` (same without `file_name`, worker only), `NewPack { kind, scope, lang, manifest, pdf: Bytes }`.
  Nothing here is updated through the API, so there are no Update structs.
- **Repository traits** (`domain::repositories`):
  - `EvidenceRepository { insert_staged(tx, StagedRow); get(id) -> Option<EvidenceObject>; row_for_open(id) ->
    Option<EvidenceOpenRow>; mark_clean(tx, id, CleanFacts) -> EvidenceObject; mark_infected(tx, id, engine,
    signature); mark_rejected(tx, id, code); pending_older_than(cutoff, limit) -> Vec<Uuid>; by_owner(kind, id) ->
    Vec<EvidenceObject>; insert_final(tx, FinalRow) }`. `CleanFacts { bucket, object_key, version_id, engine,
    scanned_at, lock_mode, lock_until, wrapped_dek, key_version }`; `mark_*` are conditional updates (`WHERE av_status
    = 'pending'`), so a scan job retried after a crash never moves a row twice.
  - `PackRepository { next_version(tx, kind, scope_key, lang) -> i32; insert(tx, NewPackRow) -> Pack; get(id);
    list(kind, cursor, limit) }`.
- **Repository impls** (`db`): `PgEvidenceRepository` (`mark_clean` = `UPDATE admin.evidence_objects SET bucket =
  'akr-evidence', ..., updated_at = now() WHERE id = $1 AND av_status = 'pending' RETURNING ...`; zero rows ->
  already handled, the job ends), `PgPackRepository` (`next_version` takes `pg_advisory_xact_lock` on the first 8 bytes
  of `scope_key` and reads `max(version) + 1`; the unique constraint is the backstop).
- **Ports** (`domain::ports`), with adapters in `common`: `VirusScanner { scan(reader, len) -> ScanOutcome { Clean,
  Infected(String), Rejected(code) }; version() -> ScannerVersion { engine, db_version, db_date } }` (`ClamdScanner`
  over INSTREAM; `FakeScanner` in tests knows the EICAR string); `PdfRenderer { render(RenderJob) -> Bytes }`
  (`GotenbergRenderer`; `FakeRenderer` in tests returns a fixed PDF).
- **Services** (section 3): `PgAuditSink`, `PgAccessLog`, `AnchorService`, `Verifier`, `PartitionMaintenance` (in
  `audit`); `EvidenceStore`, `RenderService`, `PackService` (in `domain::evidence`, over the ports and repositories).
- **Extractors** (`api`): `RequestCtx` (from the correlation extension of phase 00, the auth context of phase 01, the
  client IP rule of phase 01 and `User-Agent`); `AccessScope` (wraps `AuthContext`, `RequestCtx`, the optional
  `X-Access-Purpose` and `X-Access-Reason` headers, and the `AccessLog`). `Accessed<T>` is the only response wrapper
  a route marked sensitive may return, and it can be built only from the `AccessReceipt` that `AccessScope::record`
  returns (section 3).
- **utoipa schemas:** none exposed in this phase. `EvidenceObjectView`, `PackView`, `AuditEventView`, `AnchorView`
  and `ChainStatusView` are defined by the phases that add their routes (18, 24).
- **Route table:** no routes. `GET /api/v1/cases/{id}/evidence/{evidence_id}/content` is phase 18 (it calls `EvidenceStore::open`); the audit
  viewer and verifier routes of API.md 3.4 are phase 24; retention routes are phase 23. Under `cfg(test)` only, the
  router mounts `GET /api/v1/_test/sensitive/{id}` to exercise `AccessScope` (section 6). Its scope is `none (dev only)`, as `POST /api/v1/_echo` has in the phase 00 route table, and it has no Cedar action (section 3).
- **Router function:** none; `build_app` gains the `RequestCtx` extractor and the access-coverage registry (section 4).

## 3. Business Logic

**Chain model** (DR-43). Each stream (`audit_log`, `access_event`; `case_event` from phase 17) is one hash chain that
runs through its monthly partitions without a break: the first row of a month takes as `prev_hash` the `entry_hash`
of the last row of the month before, and the first row of the stream takes 32 zero bytes. Writers of a stream are
serialised by a transaction-scoped advisory lock, and `occurred_at` is assigned under that lock from the database
clock, strictly increasing, so the chain order is the time order and an anchor can say "every row up to time T". The
DBS seeded a new partition from the previous partition's latest anchor; rows written after that anchor were then
bound to nothing until the next anchor, and deleting them before it ran left no trace. Chaining from the last row
closes that window.

**Stream lock keys:** the first 8 bytes of `SHA-256("akr.audit.stream:" || stream)` with the top bit cleared (a
positive 63-bit `bigint`); constants with a unit test that recomputes them.

**`PgAuditSink::emit(tx, event)`** (`audit::sink`):
1. Look the kind up in the catalogue; it must belong to the `audit_log` stream. Build the canonical `details`
   (already checked against `detail_keys` by the builder).
2. `SELECT pg_advisory_xact_lock($key)`. Then confirm the lock is held by this backend (`SELECT EXISTS (SELECT 1
   FROM pg_locks WHERE locktype = 'advisory' AND pid = pg_backend_pid() AND granted AND ((classid::bigint << 32) |
   objid::bigint) = $key AND objsubid = 1)`); absent means the statement ran outside a transaction and the lock
   already ended -> `AuditError::NotInTransaction` (the DBS sink detected the same case; the `Transaction` type makes
   it unlikely, the check makes it impossible to miss). Lock wait goes to `akr_audit_lock_wait_seconds{stream}`.
3. Head: `SELECT id, occurred_at, entry_hash FROM audit.audit_log ORDER BY occurred_at DESC, id DESC LIMIT 1` (the
   ordered partition scan reads the newest partition first). A transaction that emits twice sees its own first row.
4. `occurred_at` = `SELECT greatest(clock_timestamp(), $head_occurred_at + interval '1 microsecond')` (just
   `clock_timestamp()` for the first row). One clock, the database's, for every writer.
5. `prev_hash` = the head's `entry_hash`, or 32 zero bytes. `id` = UUIDv7. `entry_hash = SHA-256(prev_hash ||
   canonical(row))`.
6. Insert inside a savepoint. "No partition of relation found for row" (SQLSTATE 23514 on a partitioned table) ->
   roll back to the savepoint, `SELECT audit.create_partition('audit_log', year, month)`, insert again once; a second
   failure -> `PartitionMissing`. The inline creation logs a warning and emits `audit.partition_created {inline:
   true}`: it means `partition_maintenance` has not run for months.
7. Return `AuditReceipt`; count `akr_audit_events_total{stream, kind}`.

The lock is held until the caller commits, so **the audit emit is the last statement before commit** (rule added to
`.claude/rules/rust-backend.md`): emitting early would serialise the rest of the transaction's work behind every
other writer. A failed emit fails the transaction: a change is never committed without its record.

`emit_standalone(event)`: begin on the service's own pool, `emit`, commit. A refused sign-in or a CLI check uses it;
if it fails, the request's outcome does not change (it was a refusal) but `akr_audit_write_failures_total` counts it
and an error is logged.

**Canonical encoding** (`audit::canonical`, version `cv = 1`; hand-written, never serde's map order):
- UTF-8 JSON without whitespace. Every object's keys sorted by byte order, at every depth; arrays keep their order.
- Fixed key sets, every key present (`null` when absent):
  - `audit_log`: `action`, `actor_id`, `case_id`, `cv`, `decision`, `details`, `id`, `occurred_at`, `on_behalf_of`,
    `request`, `resource_id`, `resource_key`, `resource_type`; `request` = `correlation_id`, `ip`, `job_id`,
    `session_id`, `token_id`, `ua`; `decision` = `attrs_hash`, `effect`, `policy_ids`, or `null`.
  - `access_event`: `action`, `actor_id`, `actor_role`, `client_ip`, `correlation_id`, `cv`, `decision`, `id`,
    `occurred_at`, `on_behalf_of`, `purpose_code`, `reason`, `resource_id`, `resource_type`, `session_id`,
    `subject_party`, `tier`, `token_id`.
- Values: strings escape only `"`, `\` and U+0000-U+001F (always as `\u00xx`, lowercase hex); no Unicode
  normalisation (the bytes stored are the bytes hashed); integers as `i64` decimals; no floats anywhere (amounts are
  decimal strings, API.md section 1); UUIDs lowercase hyphenated; timestamps `YYYY-MM-DDTHH:MM:SS.ffffffZ` (UTC, six
  digits); bytes (hashes, the sealed `reason`) lowercase hex; IP addresses in their textual form (IPv4 dotted, IPv6
  RFC 5952).
- `entry_hash = SHA-256(prev_hash || canonical bytes)`; the hashes themselves are never inside the canonical form.
- Round trip: the verifier rebuilds the canonical bytes from the stored row. `jsonb` reorders keys (harmless: the
  encoder sorts) and refuses U+0000 (refused earlier by the builder); integers keep their digits.
- Golden vectors: `audit/tests/vectors/*.json` (Arabic and Kurdish text, a character outside the BMP, control
  characters, nested `details`, all-null optionals, `i64::MIN` and `i64::MAX`, IPv6) with expected canonical bytes and
  hashes; `tools/audit_canon.py`, an independent Python implementation, must produce the same hashes in CI.

**`PgAccessLog::record(events)`** (SP-35; one transaction, several rows for a list of search hits):
1. Seal each `reason` under the `personal` family with AAD = the event id.
2. Lock the `access_event` stream, read its head, then for each event in order: `occurred_at`, `prev_hash`, `id`,
   `entry_hash` as in `emit`; insert (same partition fallback).
3. Commit. Any failure -> `AuditError`, and the caller answers 503 `E_UNAVAILABLE` without the content.

Access events are always written in their own transaction, never inside a domain transaction (DR-46): a transaction
then holds at most one stream lock, so the two streams cannot deadlock, and the read is on record before a byte of
the record leaves the server.

**`AccessScope`** (the extractor every sensitive route takes, `api::access`):
1. `scope.record(Access { resource_type, resource_id, subject_party, tier, action })` after the handler has loaded
   the record and before it builds the response.
2. Tier `restricted` or `suspicion`: `X-Access-Purpose` must hold a purpose code, else 422 `E_PURPOSE_REQUIRED`
   (written as a `deny` access event first, so a probe without a purpose is on record). The code list is admin data
   from phase 03; until then the format is checked. `X-Access-Reason` is optional free text, at most 500 characters,
   percent-encoded UTF-8.
3. `actor_role` = the effective role the authorisation relied on (phase 03 fills it from the Cedar decision; until
   then the first effective role that carries the route's scope); `on_behalf_of`, `session_id`, `token_id`,
   `client_ip` and `correlation_id` from `AuthContext` and `RequestCtx`; `decision = allow` (phase 03 also writes
   `deny` and `conflict` events from the policy layer).
4. Returns an `AccessReceipt`; `Accessed::new(receipt, body)` is the only way to build the response type of a
   sensitive route, and it sets `Cache-Control: no-store`.
5. Lists: `scope.record_hits(rows)` writes one `search_hit` per restricted or suspicion-tier row shown and nothing
   for normal-tier rows (research file 30 s.9.2), in one transaction.

**Coverage:** a route is sensitive when it carries the OpenAPI extension `x-akr-access: <resource_type>` (set by the
`sensitive!` route macro). Two tests: the set of operations with the extension equals the registry
`api::access::SENSITIVE` (each entry names a fixture request); and every registered fixture, run against a seeded
database, writes exactly the expected access events. A route that returns `Accessed<T>` without the extension fails
to compile (the macro is the only implementer of the marker trait), and an extension without a fixture fails the
first test. In this phase only the `cfg(test)` route `GET /api/v1/_test/sensitive/{id}` is registered; phases 03
onwards register theirs as they add them.

**`AnchorService::run(stream)`** (job `anchor_audit`, both streams in turn; worker):
1. Last anchor of the stream (`ORDER BY range_to DESC LIMIT 1`): `prev_id`, `prev_root`, `prev_range_to`,
   `prev_last_hash` (none: genesis, `prev_last_hash` = 32 zero bytes).
2. Settle point: a short transaction takes the stream lock, reads the head, commits. Every row at or before the head
   is now committed (writers hold the lock until commit), and every later row gets a later time.
3. Nothing after `prev_range_to` -> set the lag gauge to 0 and stop.
4. Read `(id, occurred_at, prev_hash, entry_hash)` for `occurred_at > prev_range_to AND occurred_at <= head`, in
   order, at most `audit.anchor_max_entries`, and split it by partition (month); each segment becomes one anchor,
   chained to the one before.
5. Continuity: the first row's `prev_hash` must equal `prev_last_hash` and each row's the previous row's
   `entry_hash`. A mismatch is never signed over: emit `audit.chain_break_detected {stream, entry_id}` (critical),
   raise `akr_audit_chain_break{stream} = 1`, fail the run permanently and stop anchoring that stream until `akr-ctl
   audit acknowledge-break` (below).
6. Merkle root over the segment: leaf = `SHA-256(0x00 || entry_hash)`, node = `SHA-256(0x01 || left || right)`, an odd
   node is promoted unchanged, one leaf is its own root (RFC 6962 domain separation; the DBS hashed pairs without
   prefixes).
7. Statement = canonical JSON of `{anchor_id, cv, entry_count, first_entry_id, key_name, last_entry_hash,
   last_entry_id, merkle_root, partition_name, prev_anchor_id, prev_merkle_root, range_from, range_to, stream}`.
8. Sign with OpenBao `transit/sign/akr-sign-anchor` (Ed25519); keep the version from `vault:vN:` as `key_version`.
   OpenBao sealed or unreachable -> `AnchorError::OpenBao`; emit `audit.anchor_failed {stream, stage: sign}` and end
   the run (the next slot covers the larger range: anchors queue, the chain stays verifiable, M4 edge case).
9. The first time a key version signs, publish its public key first: PUT `keys/akr-sign-anchor/v<N>.pub` to
   `akr-anchors` (COMPLIANCE) and emit `audit.signing_key_published {key_version, fingerprint}`.
10. PUT the artefact `{statement (base64 of the exact bytes), signature, key_name, key_version}` to `akr-anchors` at
    `anchors/<stream>/<partition>/<range_to as microseconds>.json` with `x-amz-object-lock-mode: COMPLIANCE`, retain
    until `now + audit.anchor_retention_days`, and `x-amz-checksum-sha256`; keep the version id. Failure -> emit
    `audit.anchor_failed {stage: store}` and end.
11. Insert the anchor row. A unique violation on `prev_anchor_id` (a second runner) -> `ChainMoved`, end; the object
    written in step 10 is an orphan the verifier reports as information only. The object is written before the row,
    so a row never points at a missing object.
12. After a run that follows a failed one (`anchor_fail:{stream}` in Valkey), emit `audit.anchor_recovered {stream,
    gap_s, entries}`. Set `akr_audit_anchor_lag_seconds{stream}` = now minus the oldest unanchored row (0 when none);
    the alert rule fires above `audit.anchor_lag_alert_s`.

Anchoring writes no `audit_log` row per anchor (the anchor table is the record); only failures, recoveries, key
publication and breaks are events.

**`Verifier::run(scope)`** (`akr-ctl audit verify`, job `audit_verify`; reads as `akr_readonly`; research file 30
s.10.1: the DBS verifier never checked signatures or the WORM copy, this one checks both):
1. Rows, per stream and partition in order: the seed is the last `entry_hash` of the previous partition (32 zero
   bytes for the stream's first); walk rows by `(occurred_at, id)` in batches of 10,000; finding `chain_break` when
   `prev_hash` differs from the expected value, `hash_mismatch` when the recomputed `entry_hash` differs,
   `order_violation` when `occurred_at` does not increase, `unknown_detail_key` (warning) for a key outside the
   catalogue.
2. Anchors, per stream along `prev_anchor_id` from the genesis: `anchor_chain_gap` when a link or a range does not
   continue the previous; recompute count, first and last ids, last hash and root over the rows of `[range_from,
   range_to]` (`anchor_root_mismatch`); rebuild the statement bytes and compare `statement_sha256`; verify the
   signature with `transit/verify` for `vault:v<key_version>:`, or offline with exported public keys
   (`signature_invalid`, `unknown_key_version`); `unanchored_rows` with a count for rows after the last anchor older
   than `audit.anchor_lag_alert_s`.
3. WORM copies, per anchor: GET `object_key` at `object_version_id` (`object_missing`); the stored statement equals
   the rebuilt one (`object_mismatch`); HEAD shows COMPLIANCE and a retain-until not before `anchored_at +
   audit.anchor_retention_days` (`object_retention_short`); the newest version of the key is not a delete marker
   (`object_hidden_by_delete_marker`: COMPLIANCE stops destruction, but a DELETE without a version id still adds a
   marker that hides the object from a plain GET). Objects under `anchors/<stream>/` with no row -> `orphan_object`
   (information).
4. Guards: every partition has `trg_no_truncate`; every audit trigger is enabled ALWAYS; both event triggers exist and
   are enabled ALWAYS (`hardening_missing`).
5. Report `{started_at, finished_at, scope, partitions, rows, anchors, objects, findings: [{kind, severity, stream,
   partition, entry_id?, anchor_id?, detail}]}`; the result is `pass` when no finding is an error or critical.
   `chain_break`, `hash_mismatch`, `anchor_root_mismatch`, `signature_invalid` and every `object_*` finding except
   `orphan_object` are critical. Emit `audit.verify_completed {trigger, result, partitions, rows, anchors,
   findings_by_kind}`; set `akr_audit_verify_last_success_timestamp` on a pass and `akr_audit_verify_findings{kind}`
   always; a critical finding pages as a P1 incident (alert rule in `deploy/observability/`).

Scope: the weekly job checks the current and previous month of each stream in full, plus every anchor and every WORM
copy (anchors are small); `akr-ctl audit verify --all` checks everything. Offline (`--offline --pubkeys keys.json`)
the CLI runs against a restored backup and a read credential for `akr-anchors`, without the application, which is
how internal audit verifies on its own schedule (research file 30 s.10.5).

**`PartitionMaintenance`** (job `partition_maintenance`, daily): for each entry of `PartitionRegistry` (`audit_log` and
`access_event` now; later phases register their tables with their own factory), create the months from the current
one through `audit.partition_ahead_months` ahead; emit `audit.partition_created {table, partition, inline: false}`
per new partition; set `akr_partitions_ahead_months{table}`; an alert fires when the next month is missing three days
before month end. Audit partitions are never dropped (R10); later registrants may drop under the retention engine.

**`audit_lock_extend`** (job, monthly; SP-44, SP-58): anchors with `anchored_at < now - (audit.anchor_retention_days -
audit.anchor_extend_below_days)` are candidates; HEAD each object, and when its retain-until is less than
`audit.anchor_extend_below_days` away, PUT retention COMPLIANCE until `now + audit.anchor_retention_days` (extension
only; the store refuses shortening); the published key objects the same. Emit `audit.lock_extended {objects,
retain_until}`. Each anchor is extended about every five years, so no copy ever approaches its end.

**`akr-ctl audit acknowledge-break --stream --entry --incident --operator --second-operator`:** after a chain break
has been investigated, two operators (each with their ceremony token, as in phase 00) record `audit.chain_break_acknowledged
{stream, entry_id, incident_ref}`; anchoring resumes from that entry, and the verifier keeps reporting the break,
marked acknowledged with the incident reference. Nothing is rewritten.

**Sealed object format AKRE1** (evidence and staging objects; DR-44): a 33-byte header (`AKRE`, version `0x01`, key
kind `0x01` object key or `0x02` family key, the family key id or 16 zero bytes, segment size `u32` = 65,536, a
7-byte random nonce prefix), then segments of at most 64 KiB, each AES-256-GCM with nonce = prefix || segment counter
(`u32`, big-endian) || last-segment flag, and AAD = header || evidence id. This is the STREAM construction used by
`age` and Tink's streaming AEAD: each segment is authenticated before its plaintext is released, and truncation or
reordering fails.

**`EvidenceStore`** (`domain::evidence`; DR-44):

`stage(tx, NewEvidence, actor) -> EvidenceObject` (akr-api; untrusted bytes from the upload routes of phases 17 and
18):
1. Size: empty -> `Empty`; above `evidence.max_bytes` -> `TooLarge` (the upload route's body limit is the same value
   plus multipart overhead).
2. Type: the declared type must be in `evidence.allowed_types`, and the sniffed type must match it (magic bytes; OOXML
   needs `[Content_Types].xml` and the right main part; text and CSV must be UTF-8 without U+0000, CSV must parse) ->
   else `TypeNotAllowed`. OOXML with `vbaProject.bin` -> `Rejected(macro)`; a PDF with `/JavaScript`, `/JS`, `/Launch`
   or `/EmbeddedFile` -> `Rejected(pdf_active_content)`.
3. `owner_kind` must be in the evidence owner registry (`pack` now; every owning phase adds its kind) ->
   `OwnerUnknown`; `retention_class` must have an entry in `evidence.lock_days` -> `NoLockPolicy`.
4. SHA-256 of the plaintext; `id` = UUIDv7; seal as AKRE1 with this month's `evidence` family key; PUT to
   `akr-quarantine` at `staging/<yyyy>/<mm>/<id>` with the stager credential. The object is written before the row;
   one without a row is an orphan `maintenance_sweep` deletes after `evidence.staging_ttl_h`.
5. In `tx`: insert the row (`av_status = pending`, `file_name` sealed), enqueue `evidence_scan {evidence_id}`
   (coalesce key = the id), emit `evidence.staged {owner_kind, owner_id, kind, content_type, size_bytes}`.
6. Return the row; owning screens show "being checked" until it is clean (phase 18).

`evidence_scan` (job, akr-worker, concurrency 4, `max_attempts` 8):
1. Load the row; not `pending` -> done (a retried job never moves a row twice).
2. Scanner version: signatures older than `evidence.av_max_signature_age_h` -> retry later (`SignaturesStale`);
   evidence stays pending and is never served while signatures are stale (fail closed).
3. GET the staging object, open it, and compare the SHA-256 with the row: a difference -> `mark_rejected
   (hash_mismatch)` and `evidence.hash_mismatch` (critical).
4. Stream the plaintext to clamd (`INSTREAM`, 64 KiB chunks): `OK` -> clean; `<name> FOUND` -> infected;
   `Heuristics.Encrypted.*` -> rejected `encrypted`; size limit exceeded -> rejected `scan_limit`; clamd unreachable ->
   retry.
5. Infected or rejected: `mark_infected` or `mark_rejected` with the engine version, emit `evidence.rejected {reason,
   signature}`. The sealed object stays in quarantine for the security officer (no sweep deletes it, section 4; it leaves only with its owner's disposal, the `owner` clock of phase 23). Done.
6. Clean: a fresh data key from `transit/datakey/wrapped/akr-kek` (plaintext key, wrapped key, key version); seal as
   AKRE1 with that key; PUT to `akr-evidence` at `ev/<retention_class>/<yyyy>/<mm>/<id>` with COMPLIANCE retention
   until `now + evidence.lock_days[class]` and the SHA-256 checksum; keep the version id. Then one transaction:
   `mark_clean` and `evidence.stored {sha256, size_bytes, lock_mode, lock_until, av_engine_version}`. Then DELETE the
   staging object (a failure leaves an orphan for the sweep). The plaintext key is zeroised after use.
   If `mark_clean` finds the row already handled, the second final object is an orphan whose wrapped key was never
   committed: unreadable by construction, counted by `akr_evidence_orphans_total` (the phase 23 reconciliation lists
   them).

`put_generated(tx, GeneratedEvidence, actor) -> EvidenceObject` (akr-worker; render output, snapshots, certificates,
packages): steps 1-3 of `stage` with the generated-type list (PDF, JSON, CSV, XML, ZIP), an inline clamd scan (a
rendered PDF can embed an uploaded image), then step 6 directly, inserting the row as clean (`insert_final`) in `tx`
and emitting `evidence.stored`. Infected -> `Infected` and nothing is stored; the calling job fails.

`open(id) -> VerifiedReader` (akr-api for the phase 18 route; akr-worker for pack assembly):
1. The row must be clean (`NotClean`) and not shredded (`Shredded`); both reach callers as 404 `E_NOT_FOUND`.
2. Unwrap `wrapped_dek` with `transit/decrypt` on `akr-kek`; unwrapped keys are cached in process memory for
   `evidence.dek_cache_s` (at most 256, zeroised on eviction), so a repeated open does not call OpenBao.
3. GET the object at `version_id` (a later version written under the same key cannot substitute content).
4. Stream-decrypt: every segment is authenticated before release; the SHA-256 accumulates; at the last segment it
   must equal the row's. Any failure aborts the stream (the response is reset, never completed), emits
   `evidence.hash_mismatch {version_id}` (critical, standalone) and counts `akr_evidence_hash_mismatch_total`; before
   the first byte it is 500 `E_HASH_MISMATCH`. Because the AEAD fails on the first altered segment, no altered byte is
   ever released.
5. The route records the access event (`view` or `download`) through `AccessScope` before calling `open`.

Shredding (phase 23) erases `wrapped_dek` and sets `shredded_at`: a COMPLIANCE-locked object then holds ciphertext
nobody can open, which is how an object is disposed of before its lock ends (SP-56). Backups keep the wrapped key
until they expire (R13, about five weeks); the deletion certificate states that date.

**`RenderService::render(RenderJob) -> RenderedPdf`** (akr-worker; RQ-11):
1. The caller renders HTML from a versioned template (minijinja, autoescape on) with `dir` and `lang` set. The service
   refuses HTML that refers to any URL other than the bundled asset names (`http:`, `https:`, `file:`, `//`), before
   Gotenberg's own deny list sees it.
2. POST multipart to `gotenberg:3000/forms/chromium/convert/html` (`index.html` plus assets) with `pdfa=PDF/A-2b`,
   `pdfua=true`, `emulatedMediaType=print`, `failOnConsoleExceptions=true`, `preferCssPageSize=true`, header
   `Gotenberg-Trace: <correlation id>`, time-out `render.timeout_s`.
3. The body must start with `%PDF-`; 503 or a time-out -> `Unavailable` or `Timeout` (the calling job retries).
4. Return the bytes and their SHA-256; `akr_render_duration_seconds`, `akr_render_failures_total{reason}`.
   The caller stores the PDF through `put_generated`.

Arabic shaping check (CI, `render_golden_arabic`, against the real Gotenberg image): a fixed template with lam-alef
ligatures, Arabic-Indic and Western digits, an English name inside a right-to-left paragraph and an IBAN-like
left-to-right run. `pdffonts` must list IBM Plex Sans Arabic embedded and no fallback font (DejaVu, Noto) ->
`FontMissing`; `pdftotext -raw`, with presentation forms mapped back, must equal the input in logical order; veraPDF
must accept PDF/A-2b; a page image from `pdftoppm` is compared with the committed golden within a pixel threshold
(the golden is approved by a person once). The results close RQ-11.

**`PackService::publish(tx, NewPack, actor) -> Pack`** (akr-worker; the first caller is the CBI gap report of phase
06):
1. Canonicalise `scope` with the audit encoder; `scope_key` = its SHA-256.
2. `id` = UUIDv7; `put_generated` the PDF with `owner_kind = pack`, `owner_id = id`, `kind = pack`, the caller's
   retention class (R6 default) and classification.
3. `next_version(kind, scope_key, lang)`; insert the pack with `evidence_id` and `sha256`; emit `pack.created {kind,
   version, lang, sha256}`.
4. A pack is never edited; a regenerated pack is the next version. Download routes, audited and with step-up, are
   phases 16 and 24.

**`akr-ctl` additions** (zone Z5; every command records the OS user and PAM session as in phase 00):
- `db harden` (superuser, through the installer's ceremony credential; run after every `db migrate`): creates
  `CREATE EVENT TRIGGER akr_audit_no_drop ON sql_drop EXECUTE FUNCTION audit.block_drop()` and `CREATE EVENT TRIGGER
  akr_audit_no_alter ON ddl_command_end WHEN TAG IN ('ALTER TABLE', 'ALTER TRIGGER', 'CREATE TRIGGER', 'ALTER
  FUNCTION', 'CREATE FUNCTION') EXECUTE FUNCTION audit.block_alter()` (each guarded against `duplicate_object`),
  `ALTER EVENT TRIGGER ... ENABLE ALWAYS` on both, and sets every audit trigger on every partition to ENABLE ALWAYS
  where it is not. Emits `audit.hardened {created, fixed}`. A deployment that cannot run it is not ready: `db check`
  and the readiness of `akr-worker` (and of `akr-api` in production) fail without it.
- `audit verify [--stream] [--from YYYY-MM] [--to YYYY-MM] [--all] [--offline --pubkeys FILE] [--json FILE]`: exit
  code 0 pass, 2 findings, 1 error.
- `audit keys export --out FILE`: every `akr-sign-anchor` public key version with its SHA-256 fingerprint, for
  internal audit's own record; emits `audit.keys_exported`.
- `audit partitions [--create-ahead]`: partitions with row counts and the last anchored time; `--create-ahead` runs
  the maintenance routine now.
- `audit acknowledge-break` (above).
- `av import <dir>`: copies ClamAV `.cvd` or `.cld` files from a signed bundle or the bank's mirror into the clamd
  volume and sends `RELOAD`; clamd verifies the database signatures on load and refuses unsigned files. Emits
  `av.signatures_imported {db_version, db_date}`.

**Settings registry additions** (DR-34): the 16 keys of migration 0010, with hard bounds refused with 422
`E_VALIDATION`: `audit.partition_ahead_months` 2-12; `audit.anchor_max_entries` 1,000-1,000,000;
`audit.anchor_lag_alert_s` 300-3600; `audit.anchor_retention_days` 3650-36500; `audit.anchor_extend_below_days`
365 up to the retention minus 365; `schedule.anchor_audit` any cron whose longest gap between fires is at most 300
seconds (SP-39: every five minutes); `evidence.max_bytes` 1 MiB-100 MiB (clamd's stream limit ships at 128 MiB);
`evidence.allowed_types` a subset of the types the sniffer supports; `evidence.lock_days` 365-36500 per class, every
class that an owner kind uses present, and the M-05 floor on R5-R8 (`E_BELOW_FLOOR`); `evidence.av_max_signature_age_h`
6-336; `evidence.staging_ttl_h` 1-168; `evidence.dek_cache_s` 0-900; `render.timeout_s` 10-300.

**Error mapping** (no new codes; all are in API.md section 6 and are first returned by the routes of phases 17 and
18): `TypeNotAllowed`, `Rejected`, `Infected` -> 422 `E_UPLOAD_REJECTED` with `errors[]` at `/file` (`type_not_allowed`,
`macro`, `pdf_active_content`, `encrypted`, `scan_limit`, `infected`); `TooLarge` -> 413 `E_PAYLOAD_TOO_LARGE`;
`Empty` -> 422 `E_VALIDATION`; `HashMismatch` -> 500 `E_HASH_MISMATCH`; `NotClean`, `Shredded` -> 404
`E_NOT_FOUND`; `ScannerUnavailable`, `SignaturesStale`, `Store`, `Crypto` and any `AuditError` from the access log ->
503 `E_UNAVAILABLE`; a missing purpose -> 422 `E_PURPOSE_REQUIRED`.

**Audit events emitted** (this phase's kinds; the catalogue also lists the 4 kinds of phase 00 and the 22 of phase 01):

| Event kind | On | Resource |
|-|-|-|
| `audit.partition_created` | `partition_maintenance` or the sink's inline fallback (table, partition, inline) | audit_partition (key = partition name) |
| `audit.anchor_failed` | an anchor run fails (stream, stage: sign, store, insert) | audit_stream (key) |
| `audit.anchor_recovered` | the first success after a failure (stream, gap_s, entries) | audit_stream (key) |
| `audit.chain_break_detected` | the anchor continuity check or the verifier (stream, entry id, kind); critical, SIEM | audit_stream (key) |
| `audit.chain_break_acknowledged` | `akr-ctl audit acknowledge-break` (stream, entry id, incident ref, both operators) | audit_stream (key) |
| `audit.signing_key_published` | first anchor under a new key version (version, fingerprint) | signing_key (key) |
| `audit.verify_completed` | verifier run (trigger, result, partitions, rows, anchors, findings by kind) | audit_stream (key) |
| `audit.lock_extended` | `audit_lock_extend` (objects, retain_until) | audit_stream (key) |
| `audit.keys_exported` | `akr-ctl audit keys export` (versions) | signing_key (key) |
| `audit.hardened` | `akr-ctl db harden` (created, fixed) | database (key) |
| `audit.searched` | reserved for the viewer of phase 24 (filters, result count; SP-42) | audit_log (key) |
| `av.signatures_imported` | `akr-ctl av import` (database version and date) | antivirus (key) |
| `evidence.staged` | `EvidenceStore::stage` (owner kind and id, kind, content type, size) | evidence |
| `evidence.stored` | clean and stored (sha256, size, lock mode and until, engine version) | evidence |
| `evidence.rejected` | infected or rejected (reason, signature, engine version); SIEM when infected | evidence |
| `evidence.hash_mismatch` | integrity failure at scan or open (version id); critical, SIEM | evidence |
| `pack.created` | `PackService::publish` (kind, version, lang, sha256) | pack |

Access events are the `access_event` stream itself, not `audit_log` rows. Every kind is in the catalogue with its
detail keys, severity and SIEM flag; a test serialises a sample of every kind and fails on a detail key outside the
list, a float, or a value matching the redaction patterns of phase 00 (names, IBANs, card-like runs, national-ID
patterns).

**Cedar actions:** none; the one route this phase adds (the `cfg(test)` route of section 2) has none. Phase 03 puts the policy decision into `audit_log.decision` and the
`access_event` decision column, and pins the actions of the phase 24 viewer (`audit.read_events`,
`audit.chain_status`, `audit.verify`) and the phase 18 evidence read (`evidence.read`).

**Tipping-off review.** This phase adds no screen, task, notice or CBS-facing artefact. What it stores that someone
outside a case's access list might read, and why they learn nothing:
- `audit_log.details` carries only catalogue-listed keys, which are ids, codes, counts and hashes, never names, free
  text or amounts; `case_id` is a pseudonymous id. The application roles cannot read audit details or the access log
  at all (column grants); `akr_readonly` serves the viewer, whose routes (phase 24) apply `audit.read` and existence
  masking, and the access-history tab is for the MLRO, the CISO and internal audit only (research file 30 s.9.2).
- `access_event.reason` is sealed; `subject_party` is a pseudonymous id.
- SIEM-flagged kinds carry no suspicion-tier content; the phase 25 exporter strips `case_id` and resource ids of
  suspicion-tier resources (noted for that phase).
- Object keys hold a class, a month and an id, never an owner, a case or a name; objects carry no user metadata; file
  names are sealed. A store administrator listing buckets learns counts by class and month only.
- Pack scopes may hold a case id; packs are listed only by phase 16 and 24 routes behind Cedar and existence masking.
- Metrics carry stream, kind and reason labels only, never an evidence, case or party id.
The answer for a person outside a case's access list is: nothing.

**Settings and configuration used:** the 16 keys of migration 0010 and the phase 00 keys. Environment additions
(validated by `Config::validate`): `GOTENBERG_URL` and `CLAMD_ADDR` (akr-worker). New OpenBao items in section 4.

## 4. Infrastructure Updates

- **Job kinds registered (akr-worker):**

| Kind | Trigger | Concurrency | Notes |
|-|-|-|-|
| `anchor_audit` | `schedule.anchor_audit` (every 5 min) | 1 | `max_attempts` 1: a failed slot is not retried, the next slot covers the larger range; lease 240 s |
| `partition_maintenance` | `schedule.partition_maintenance` (daily 01:15) | 1 | audit streams now; later phases register their tables |
| `audit_verify` | `schedule.audit_verify` (Saturday 04:00) | 1 | lease 6 h; reads through the `akr_readonly` pool |
| `audit_lock_extend` | `schedule.audit_lock_extend` (monthly, 1st, 04:30) | 1 | extends anchor and key-object locks |
| `evidence_scan` | enqueued by `EvidenceStore::stage` (coalesce key = evidence id) | 4 | `max_attempts` 8 |

  `maintenance_sweep` (phase 00) also deletes `akr-quarantine` objects under `staging/` older than
  `evidence.staging_ttl_h` that have no row or whose row is clean (a staging copy left by a crash); it never deletes
  an infected or rejected object. **Outbox topics:** none. **Timers:** none.
- **Compose** (both bundles):

| Service | Change |
|-|-|
| `gotenberg` | Now used. Flags: Chromium allow list `^file:///tmp/.*` and deny everything else, JavaScript disabled, download-from and webhooks disabled, LibreOffice routes disabled, JSON logs (flag names re-checked on the pinned release, RQ-01); health check `/health`; reachable from `akr-worker` only |
| `clamd` | Now used. `clamd.conf`: `StreamMaxLength 128M`, `MaxFileSize 128M`, `MaxScanSize 512M`, `ScanArchive yes`, `AlertEncrypted yes`, `AlertOLE2Macros yes`, `AlertBrokenExecutables yes`; `freshclam` off (no egress): signatures come from the release bundle or the bank's mirror through `akr-ctl av import`; reachable from `akr-worker` only |
| `akr-ctl` | `db harden` needs the superuser credential, issued per ceremony by OpenBao; never stored in the container |

- **Store** (`akr-ctl store init`): bucket default retention COMPLIANCE 2,555 days on `akr-evidence` (a safety net;
  every PUT sets its own retention) and COMPLIANCE 3,650 days on `akr-anchors`. A fourth credential, **stager**: PUT
  only on `akr-quarantine/staging/*`, used by `akr-api` to stage uploads; `akr-api` reads `akr-evidence` with the
  GET-only presign credential through the guard; `akr-worker` keeps the writer credential (GET and PUT on the three
  buckets, DELETE in `akr-quarantine` only, PUT retention). The conformance suite gains: the stager cannot GET, cannot
  PUT outside `staging/`, cannot DELETE; a GET by version id returns that version after a newer version is written.
- **OpenBao:** `akr-worker` policy gains `transit/datakey/wrapped/akr-kek` (per-object data keys),
  `transit/verify/akr-sign-anchor`, read on `transit/keys/akr-sign-anchor` (public keys) and KV read `db/akr_readonly`
  (verifier pool); `akr-api` gains KV read `s3/stager`. New KV item `s3/stager`. No policy gains `transit/export`: the
  private signing key never leaves OpenBao.
- **Database provisioning** (`akr-ctl db provision`, DR-22, DR-45): `GRANT akr_audit_writer TO akr_app, akr_ingest,
  akr_assess` (INHERIT); `db check` asserts it, and asserts that no role holds UPDATE, DELETE or TRUNCATE on any
  `audit` relation.
- **Zone rules** (`deploy/zones.yaml`, `make zone-check`): new edges `akr-worker -> gotenberg:3000` and `akr-worker ->
  clamd:3310`; `gotenberg` and `clamd` reach nothing (clamd may reach the bank's signature mirror only when one is
  declared at install). From `akr-api`'s position `gotenberg` and `clamd` are closed.
- **AppState additions:** `api`: `audit: Arc<dyn AuditSink>` is now `PgAuditSink`, `access_log: Arc<dyn AccessLog>`,
  `evidence: Arc<EvidenceStore>` (stage and open only). `worker`: `anchors: AnchorService`, `verifier: Verifier`,
  `partitions: PartitionMaintenance`, `evidence: Arc<EvidenceStore>`, `render: RenderService`, `packs: PackService`,
  `scanner: Arc<dyn VirusScanner>`, `readonly_pool: PgPool`. `ingestd` and `assess`: `audit: Arc<dyn AuditSink>`.
- **Readiness:** `akr-worker` adds `audit_hardening` (event triggers present and enabled ALWAYS; audit triggers
  ALWAYS on every partition), `audit_partitions` (current and next month exist for both streams), `clamd` (an EICAR
  probe expecting `FOUND`, cached for 5 minutes, so readiness proves the engine has signatures, not only that it
  answers) and `gotenberg` (`/health`). `akr-api` adds `audit_partitions`, and `audit_hardening` in production.
- **Metrics:** `akr_audit_events_total{stream, kind}`, `akr_audit_lock_wait_seconds{stream}`,
  `akr_audit_write_failures_total{stream}`, `akr_audit_anchor_lag_seconds{stream}`,
  `akr_audit_anchor_failures_total{stage}`, `akr_audit_chain_break{stream}`,
  `akr_audit_verify_last_success_timestamp`, `akr_audit_verify_findings{kind}`, `akr_partitions_ahead_months{table}`,
  `akr_evidence_scans_total{result}`, `akr_evidence_hash_mismatch_total`, `akr_evidence_orphans_total`,
  `akr_clamd_signature_age_hours`, `akr_render_duration_seconds`, `akr_render_failures_total{reason}`. Alert
  rules in `deploy/observability/`: anchor lag, any chain break, a critical verifier finding, no passing verify in 8
  days, next month's partition missing, signature age, a hash mismatch.
- **OpenAPI registration:** no paths. The `x-akr-access` extension is documented in the API conventions; the test
  route is not in the published document.
- **Existing entity, schema or route updates:**
  - phase 00 schema checks skip partitions (`relispartition = true`) in the classification and retention coverage
    tests, and now run over `audit` too;
  - phase 00 `Config::validate` refuses `TracingSink` in production; `maintenance_sweep` gains the staging step;
    `db provision`, `db check` and `store init` gain the steps above;
  - phase 00 and 01 events persist; their kinds join the catalogue with detail keys, severity and SIEM flag, and the
    request block test runs over the phase 01 curl suite (every row's `request.correlation_id` equals the
    `X-Correlation-Id` of the response that caused it: the DBS shipped this column empty everywhere);
  - `.claude/rules/migrations.md` gains "an audit DDL change sets `SET LOCAL akr.audit_ddl = 'on'` and says why in
    its phase file"; `.claude/rules/rust-backend.md` gains "the audit emit is the last statement before commit" and
    "access events are written in their own transaction before the content is sent";
  - DATA-MODEL.md s.2, s.18 and s.21, API.md (the evidence row and the purpose headers) and ARCHITECTURE.md s.12 are
    updated in the same change as this file.

## 5. Frontend Impact

Backend-only phase; no frontend impact. The audit viewer and chain verifier (G6), retention and holds (G7) and the
evidence viewer (K3) come in phases 24, 23 and 18; nothing here is exposed through the API.

## 6. Verification

1. `make migrate`: 0008-0010 apply; a second run is a no-op; the phase 00 schema checks pass for the five new tables
   (classification and retention coverage, partitions skipped, no `timestamp without time zone`, no `id` default, no
   reserved word); `\d+ audit.audit_log` lists four seeded partitions; `\dp audit.audit_log` shows `akr_audit_writer`
   with INSERT and column SELECT on (`id`, `occurred_at`, `prev_hash`, `entry_hash`), `akr_readonly` with SELECT and
   no other grantee.
2. `akr-ctl db harden` then `akr-ctl db check`: both event triggers exist and are enabled ALWAYS; every audit trigger
   on every partition is ALWAYS; `akr_app`, `akr_ingest` and `akr_assess` are members of `akr_audit_writer`.
3. `cargo check` with `cargo sqlx prepare`; `cargo clippy --all-targets -- -D warnings`; `cargo deny check`.
4. `cargo test`, including:
   - append-only, as `akr_app` (`SET ROLE`): `UPDATE`, `DELETE` and `TRUNCATE` on `audit.audit_log` -> permission
     denied; `SELECT details FROM audit.audit_log` -> permission denied; as the owner `akr_migrate`: `UPDATE ... SET
     details = '{}'` and `DELETE` -> the trigger's exception; `TRUNCATE audit.audit_log` and `TRUNCATE
     audit.audit_log_<current month>` -> exception; after `db harden`: `DROP TABLE audit.audit_log_<next month>`,
     `ALTER TABLE audit.audit_log DETACH PARTITION ...`, `ALTER TABLE audit.audit_log DISABLE TRIGGER ALL` and
     `CREATE OR REPLACE FUNCTION audit.block_mutation() ...` -> exception; as a superuser with
     `session_replication_role = replica`, `UPDATE` still raises (ENABLE ALWAYS); the same for `access_event` and
     `audit_anchors`;
   - chain: 50 concurrent transactions each emitting two events -> 100 rows with strictly increasing `occurred_at`
     and a valid chain; a rolled-back transaction leaves no row and no gap; a sink call on a non-transactional
     connection -> `NotInTransaction`; a month boundary (the test writer's `min_occurred_at` hook starts the stream on
     the last second of a month) -> the first row of the new month carries the last `entry_hash` of the old one; a
     missing partition -> created inline, one `audit.partition_created {inline: true}`;
   - canonical: the golden vectors match, and `tools/audit_canon.py` reproduces every hash; a float in `details` ->
     `FloatInPayload`; a detail key outside the catalogue -> refused; every catalogue kind's sample passes the
     redaction test;
   - anchors (with `InMemoryCrypto` and the real store in CI): two runs over 1,000 rows give contiguous anchors whose
     roots match an independent Merkle computation; OpenBao stopped -> the run fails, `audit.anchor_failed {stage:
     sign}`, writes continue, the lag gauge grows; restarted -> one anchor covers the whole gap and
     `audit.anchor_recovered` is emitted; a row altered by a superuser (`event_triggers = off`, triggers disabled) ->
     the next run emits `audit.chain_break_detected` and signs nothing;
   - verifier: a clean database -> `pass`; then each tamper gives its finding: an altered `details` ->
     `hash_mismatch` at that id; a deleted row -> `chain_break`; the deleted tail of last month -> `chain_break` at
     the first row of this month; an anchor row inserted with a wrong root -> `anchor_root_mismatch`; a signature
     from another key -> `signature_invalid`; the anchor object overwritten with a new version -> no finding (the
     verifier reads the recorded version, so a later version cannot stand in for it); the object hidden by a delete marker
     (admin credential, DELETE without version id) -> `object_hidden_by_delete_marker`; a partition whose TRUNCATE
     trigger was dropped by a superuser -> `missing_partition_trigger`;
   - access log: the test route with tier `suspicion` and no `X-Access-Purpose` -> 422 `E_PURPOSE_REQUIRED` and one
     `deny` event; with a purpose -> 200, `Cache-Control: no-store`, one `allow` event whose `reason` is ciphertext;
     with the database refusing inserts -> 503 and no body; a list of five rows (two restricted) -> two
     `search_hit` events; the coverage tests pass with the one registered route and fail when the extension is
     removed from it;
   - evidence (FakeScanner and FakeRenderer in unit tests, real clamd and store in CI): the EICAR string ->
     `infected`, never in `akr-evidence`; a PDF declared as PNG -> `TypeNotAllowed`; a DOCX with a macro and a PDF with
     `/JavaScript` -> rejected; a clean PDF -> `akr-evidence` object with COMPLIANCE retain-until equal to now plus
     `evidence.lock_days[R6]` (one minute tolerance) and a row with `wrapped_dek`; `open` returns the original bytes;
     a new version written over the object by the admin credential -> `open` still returns the original (version
     pinned); the recorded version's ciphertext altered in a test store -> `open` fails on the first altered segment
     with no byte released and `evidence.hash_mismatch`; a truncated object -> fails; `wrapped_dek` erased ->
     `Shredded`; the scan job run twice on one row -> one final object state, the second run a no-op; signatures
     older than the limit -> the row stays pending; `UPDATE admin.evidence_objects SET sha256 = ...` as `akr_app` ->
     permission denied; replacing `wrapped_dek` or shortening `lock_until` -> the guard trigger refuses;
   - render: the Arabic golden (fonts, logical text, PDF/A-2b, pixel threshold); HTML with
     `<img src="http://example.org/x.png">` -> refused before Gotenberg; with the pre-check disabled in the test,
     Gotenberg's deny list refuses it too; a template with a script tag renders without running it;
   - packs: two publishes of the same scope -> versions 1 and 2, both rows immutable (`UPDATE` -> permission denied).
5. curl: none; this phase has no business route (the `cfg(test)` route is exercised in step 4). No Cedar 403 or
   existence-masking 404 case exists yet (phase 03), and no idempotent mutation.
6. Swagger UI is unchanged; `make generate-api` gives zero diff.
7. Audit, end to end on the dev stack: run the phase 01 curl suite (`tests/curl/phase01_identity.sh`) and `akr-ctl ops
   record-restore-drill --result passed --evidence drill.pdf`; then `akr-ctl audit verify --all` -> exit 0; `psql` as
   `akr_readonly`: `SELECT action, count(*) FROM audit.audit_log GROUP BY 1` lists the phase 01 kinds exercised and
   `restore_drill.recorded`; every row's `request ->> 'correlation_id'` is non-null and matches the curl responses'
   `X-Correlation-Id`; after five minutes `SELECT stream, entry_count FROM audit.audit_anchors` shows an anchor per
   stream with rows, and `akr-ctl audit verify --offline --pubkeys keys.json` (keys from `akr-ctl audit keys export`)
   against a pgBackRest restore of the same database also exits 0.
8. Platform: `make zone-check` shows the two new edges and nothing else; `akr-ctl store conformance` passes with the
   stager checks; `akr-ctl av import` with a tampered `.cvd` -> clamd refuses it and readiness stays on the previous
   database.
9. Phase 00 and 01 tests still pass.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 1, from tools/agent/gap1-run1.md; lead-verified (tools/agent/gap1-run1-verdict.md).

### 7.1 Evidence owner registry: kinds and owning phases
The evidence owner registry holds the kinds below. `stage` refuses an owner kind that is not in the registry (step 3).
Each row is registered by the phase named. The retention column shows the class where the owning phase states one;
a dash means the owning phase states none (the verification pass checks each).

| Owner kind | Owning phase | Retention (as stated) |
|-|-|-|
| `pack` | 02 (existing) | per pack kind |
| `source` | 05 | R9 |
| `gap_report_run` | 06 | R6 |
| `goaml_profile` | 07 | R9 |
| `goaml_corpus_case` (kind `snapshot`; phase 07 section 7.6, DR-415) | 07 | R9 (classification `secret-str`) |
| `coverage_certificate` | 12 | R6 |
| `fx_rate` | 09 (manual rates, evidence kind `upload`) | - |
| `mapping_run` | 10 | - |
| `kyc_review`, `pep_register`, `ownership_input`, `override`, `rating_feed_export` | 15 | - |
| `replay_sample`, `cbi_import` | 16 | - |
| `alert` | 17 | - |
| `case` | 18 | - |
| `network_snapshot` (kind `snapshot`) | 19 | R6 |
| `ops_task` | 20 | - |
| `report`, `report_package`, `filing`, `letter`, `periodic_report`, `fiu_feedback` | 21 | R6 or R7 (phase 21 item 11) |
| `rfi_input` | 22 | R6 (classification personal) |
| `legal_hold` (kind `package`) | 23 | R6 |
| `validation_pack` | 24 | R11 |
| `register_export` | 24 | R6 (register kind `decisions`), R9 (every other register) |
| `training_record` | 24 | - |
| `islamic_deal` | 26 | R3 |
| `model_version` | 27 | - |
| `assess_extract` | 29 | - |
| `ops_drill` | 30 | R10 |

### 7.2 EvidenceStore transient objects (for goAML)
Three methods on `EvidenceStore`, with the signatures of phase 07 section 3.2 (the types of `n` and `stream` are those
defined there):
- `stage_transient(purpose, owner, n, stream)`: writes a sealed AKRE1 object under `transient/<purpose>/` in
  `akr-quarantine`. No evidence row is written.
- `open_transient(purpose, owner, n)`: opens such an object for the worker.
- `delete_transient(purpose, owner)`: deletes the objects of that purpose and owner.
S3 guard policy for the prefix: the stager credential writes, the worker credential reads and deletes (section 4).
The `transient/` prefix is swept by `maintenance_sweep` (proposed 7.11 for phase 00).

### 7.3 EvidenceStore disposal methods and the deletion column
- `shred(tx, evidence_id, reason) -> ShredOutcome`: erases `wrapped_dek` and sets `shredded_at` in the same transaction;
  audit `retention.evidence_shredded` (phase 23 section 2 ports). A locked object is shredded, never deleted, before its
  lock ends.
- `delete_object(evidence_id) -> ()`: removes the stored object only after `lock_until` has passed; the S3 guard refuses
  an earlier delete.
- `extend_lock(tx, evidence_id, until)`: sets `lock_until` only to a later date; the trigger of section 1 (`admin.evidence_objects_guard`) refuses
  a shorter one.
- DDL: `admin.evidence_objects` gains `object_deleted_at TIMESTAMPTZ` (nullable), set only by `delete_object`; its
  classification row is the one phase 23 section 1.2 defines (migration 0072 adds the column; this text documents it).

### 7.4 PartitionRegistry: entries registered by later phases
Each entry has its own factory and `ahead_months = audit.partition_ahead_months`. Registered entries, by phase:
- phase 10: the ten partitioned `canon` tables (list in phase 10 section 1), factory `canon.create_partition`.
- phase 13: `feat.feature_values`, `feat.segment_stats`, `feat.profiles`, factory `feat.create_partition`.
- phase 15: `kyc.ratings`, factory `kyc.create_partition`.
- phase 16: `scn.replay_results`, `scn.shadow_results`, factory `scn.create_partition`.
- phase 17: `alert.alerts`, factory `alert.create_partition`.
- phase 29: `assess.decisions`, factory `assess.create_partition`.
A missing month is created once by the writer and retried (SQLSTATE 23514; phases 10 and 13). There is no
DEFAULT partition.

### 7.5 Audit catalogue: kinds emitted by later phases
`CATALOGUE` holds every kind that any phase emits. Each phase's audit table gives, for every kind, its resource type,
severity, SIEM flag and detail keys (the audit section of each phase 03 to 31; that section is normative for its kinds). A kind that is not
listed is added with those values; a listed kind is reused. A test serialises a sample of every kind and fails on a
detail key outside the catalogue (phase 02 section 3, already stated).
Resource types added for phase 16 (section 4.4 of phase 16): `scenario`, `scenario_version`, `parameter_set`, `replay`,
`replay_sample`, `seeded_test`, `detection_key`, `catalogue_import`, `system`.
The kinds are not copied here: `CATALOGUE` is the union of the phases' audit tables, and a test fails on a kind
emitted by code that is in no phase's table.

### 7.6 Column read for akr_app on audit_anchors
`GRANT SELECT (anchored_at) ON audit.audit_anchors TO akr_app;`
Used by the declaration check of phase 11 (section 4.4). Only this column is granted; the anchor rows stay unreadable to
`akr_app` otherwise.

### 7.7 case_event: one chain per case
- The stream `case_event` is chained per case, not as one chain: each case has its own hash chain, written under the case
  row lock (DR-122, phase 17 section 3.7). Alert events are not chained.
- The anchor for `case_event` carries, in its payload, the digest of the heads of the cases that changed since the
  previous anchor, so the external anchor records what every case said at that time (phase 17 section 3.7).
- The chain rule of this section (one chain per stream, through the monthly partitions) applies to `audit_log` and
  `access_event` only.

### 7.8 admin.packs: kind list after phases 06 and 24
The table DDL lists the kinds as they stand after migrations 0021 (phase 06) and 0078 (phase 24):
`kind IN ('board', 'case', 'report', 'assessor', 'correspondent', 'validator', 'cbi_gap', 'integration_profile', 'vendor_request', 'certification')`.
The ALTER statements of phases 06 and 24 stay in those phases.

### 7.9 access_event.action holds the access verb
The `action` column of `audit.access_event` holds the access verb (`view`, `search_hit`, `export`, `print`,
`download`, `unmask`), never a Cedar action name; the CHECK is unchanged (DR-358). The profile read of phase 13
(`GET /parties/{id}/profiles`, Cedar action `profile.read`) writes `action = 'view'`, `resource_type = 'customer'`,
`tier = 'restricted'`, with the purpose code and the party; phase 13 wrote the Cedar name as the access value,
corrected in its 7.1.

Gap pass 1 (2026-10-10), run 2: corrections found while checking phases 03 and 04 (tools/agent/gap1-run2-verdict.md); lead-verified.

### 7.10 Evidence owner kinds added by run 2
The registry of 7.1 also holds `change_request` (phase 04 7.2, the change-request evidence route; DR-363) and
`release` (phase 25, the evidence of a `platform_release` request; phase 04 7.7) and `access_review` (phase 25 section 3.1 snapshot step 4: the quarterly snapshot, owner id the review id; its `EvidenceResolver` is phase 25's and admits the readers of the review).

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.11 AccessReceipt is an opaque proof (V-02-01-a, LOW; DR-413)
`AccessReceipt` has no public field and no public constructor. Only `AccessScope::record` returns it (section 3,
step 4), after the access events are written, and `Accessed::new(receipt, body)` needs one.

### 7.12 PartitionMaintenance: handler and a failing table (V-02-03-a, V-02-03-b, LOW; DR-413)
`PartitionMaintenance` implements the phase 00 `JobHandler` for kind `partition_maintenance`: `run(&self, ctx:
JobContext, payload: Value) -> Result<(), JobError>` walks the `PartitionRegistry` entries of 7.4 in order. A factory
call that fails returns `JobError::Retryable`, so the attempt ends at that table; the tables after it get their months
on the retry, and until then the writer creates a missing month itself (7.4).

### 7.13 Catalogue values for the kinds of phases 00, 01 and 02 (V-02-04-a, MEDIUM; DR-413)
The audit tables of phases 00, 01 and 02 have no severity or SIEM column, and 7.5 covers phases 03 to 31. For their
kinds:
- Detail keys: the names in parentheses in the kind's row. `restore_drill.recorded` carries `result` and
  `evidence_sha256` (the SHA-256 of the file named by `--evidence`).
- Severity: `Info`, except `Warning` for `auth.login_failed`, `auth.account_locked`, `stepup.denied` and `job.dead`,
  `Notice` for `setting.updated` and `flag.updated`, and the phase 02 kinds whose row says critical.
- SIEM flag set: the sign-in and step-up kinds SP-18 needs (`auth.login_succeeded`, `auth.login_failed`,
  `auth.account_locked`, `stepup.issued`, `stepup.denied`); the account and credential changes
  (`auth.password_changed`, `auth.mfa_enrolled`, `auth.mfa_removed`, `session.revoked`, `token.created`,
  `token.revoked`, `user.bootstrapped`, `user.unlocked`, `user.disabled`, `role_grant.created` and the three
  `service_client.*` kinds); `setting.updated`, `flag.updated` and `restore_drill.recorded` (as phase 30 flags
  `failover_drill.recorded`); and three phase 02 kinds: `audit.chain_break_detected`, `evidence.hash_mismatch` and
  `evidence.rejected`. The flag is one boolean per kind, so `evidence.rejected` is exported for both outcomes, and its
  `reason` tells an infected file from a rejected one. No other kind of these three phases is exported.

### 7.14 RenderJob fields (V-02-05-a, LOW; DR-413)
`RenderJob` holds what steps 1 and 2 of `RenderService::render` read: `html` (the rendered `index.html`), `assets` (the
bundled asset files by name), and the `dir` and `lang` the caller set. It has no template name and no template
version: the caller renders the template before the call. For the same reason `akr_render_duration_seconds` has no
`template` label (step 4 and section 4).

### 7.15 The two render errors that are not retried (V-02-05-b, LOW; DR-413)
Step 3 of `RenderService::render` gives the retry for `Unavailable` and `Timeout`. `Refused` (the URL refusal of
step 1) and `FontMissing` are permanent: the same HTML or font set fails the same way on every attempt. The calling job
returns `JobError::Permanent` for both, so it goes to `dead` without a retry (phase 00 job harness).

### 7.16 ShredOutcome (V-02-07-a, LOW; DR-413)
`ShredOutcome` is what `shred` returns (7.3): the enum `{ Shredded, AlreadyShredded }` of phase 23 section 7.8.
`Shredded` is the call that erased `wrapped_dek` and wrote `shredded_at` in the same transaction. `AlreadyShredded` is
a repeat: it keeps the first `shredded_at` and writes no second event.

### 7.17 The case_event anchor, step by step (V-02-09-a, MEDIUM; DR-413)
`AnchorService::run` for stream `case_event` seals case heads, not rows (7.7, DR-122). It differs from the steps of
section 3 as follows.
- Step 2, settle point. Case events are written under the case row lock, not under a stream lock, so the stream lock
  key of `case_event` is used in two modes. An append takes it in share mode (`pg_advisory_xact_lock_shared`) and
  assigns `at` under it (phase 17 section 7+). The settle transaction takes it exclusively, reads the database clock as
  `head` and commits. Every event at or before `head` is then committed, and appends still do not wait for each other.
  The settle runs with `SET LOCAL lock_timeout = '2s'`; on a time-out the run ends for this stream and the next slot
  retries (the lag gauge of step 12 shows a stream that never settles).
- Step 4 reads `(case_id, seq, at, hash)` from `casework.case_events` for `at > prev_range_to AND at <= head`, ordered
  by `(at, case_id)`. When more than `audit.anchor_max_entries` rows exist, `head` is lowered to the `at` of the last
  row inside the limit and every row at or before it is read. The table is not partitioned, so there is one segment,
  and `partition_name` is `case_events`. The heads are the last row read of each case, as `{case_id, last_seq,
  head_hash}`, sorted by `case_id`.
- Step 5 does not apply: each chain is checked by the trigger `case_events_chain_guard` and by `CaseChain::verify`
  (phase 17).
- Step 6: one leaf per head, leaf = `SHA-256(0x00 || canonical JSON of the head)`. This root is the digest of the
  heads that DR-122 names; the statement of step 7 gains no key.
- Step 7 and the anchor row: `entry_count` is the number of rows read; `range_from` and `range_to` are the `at` of the
  first and of the last row; `first_entry_id` and `last_entry_id` are the `case_id` of the first and of the last head;
  `last_entry_hash` is the last head's `head_hash`.
- `Verifier::run`: step 1 skips this stream (the chains are verified by phase 17's `case_chain_check`); step 2
  rebuilds the heads from the rows of `[range_from, range_to]` and compares the count, the ids, the last hash and the
  root (`anchor_root_mismatch`). It reads the four columns `(case_id, seq, at, hash)` as `akr_readonly`, through the
  column grant of phase 17 section 1; no payload and no actor column is granted.

### 7.18 Verifier reads on akr-anchors (V-02-10-a, MEDIUM; DR-413)
Section 4, Store bullet: the `akr-worker` writer credential also holds `s3:ListBucketVersions` and
`s3:GetObjectRetention` on `akr-anchors`. Step 3 of `Verifier::run` needs the first for the delete-marker and orphan
checks and the second for the retain-until check. The S3 guard already passes both kinds of request (phase 00, object
store port and S3 guard).

### 7.19 Repository row types (V-02-11-c, LOW; DR-413)
`StagedRow`, `FinalRow`, `EvidenceOpenRow` and `NewPackRow` are plain structs of the columns that the step using them
sets or reads, and they hold nothing else: `StagedRow` the `admin.evidence_objects` columns `stage` sets in its step
5; `FinalRow` the same columns as `put_generated` inserts them clean, with the `CleanFacts` fields; `EvidenceOpenRow`
the columns `open` reads in its steps 1 to 4; `NewPackRow` the `admin.packs` columns that step 3 of
`PackService::publish` sets.

## What This Phase Does NOT Touch

- No business routes: the audit viewer, chain status and verify-run routes and screens (API.md 3.4, G6) are phase 24;
  `GET /cases/{id}/evidence/{evidence_id}/content` (K3) is phase 18; retention, legal holds, deletion certificates and shredding runs (G7) are
  phase 23; upload routes that call `stage` are phases 17 and 18.
- No Cedar decisions (phase 03): `audit_log.decision` stays null and every access event is `allow` or a purpose
  `deny`.
- No case event stream (phase 17), case sealing (M-61, phase 17) or decision signing (phase 29); the anchor table
  already accepts the `case_event` stream.
- No SIEM export, pull API, RFC 3161 time stamps or cold tablespace (phase 25 and 23).
- No retention engine: nothing is deleted or shredded; locks are set at the floor and only anchor locks are extended.
- No CBS connection; akr-ingestd and akr-assess gain the sink and emit nothing yet.
- No frontend.
- No change to phase 00 or 01 tables; the changes to their code are the sink swap, the schema-check exclusion of
  partitions, the provisioning and store steps, and the catalogue entries.
