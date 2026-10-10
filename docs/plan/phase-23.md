# Phase 23: Retention & Legal Holds

**Goal:** One retention engine disposes of records only when their class allows it: never before the Iraq floor, the bank's lengthened period or a legal hold. It plans each night from the phase 00 assignment table, drops high-volume partitions after copying held rows, purges customer-linked records on relationship-end and closure clocks, crypto-shreds evidence whose owner is gone, and certifies every disposal with a signed deletion certificate that a restored database replays before it reopens. Legal holds are placed by the MLRO under step-up and released only by four-eyes workflow; evidence orphans are reconciled.

**Dependencies:** Phase 00 (`admin.retention_assignment`, `admin.settings`, `admin.jobs`, `admin.timers`, `maintenance_sweep`, the role list), Phase 01 (`StepUpService` and the declared `StepUpAction` constants), Phase 02 (`admin.evidence_objects`, `EvidenceStore`, `put_generated`, the audit chain and the anchor key, `evidence.lock_days`, `akr_evidence_orphans_total`), Phase 03 (Cedar bundle and file ranges), Phase 04 (`WorkflowEngine`, `WorkflowSubject`, `TerminalEffect`, `Holders`, the task act route), Phase 09 (governed settings and E_BELOW_FLOOR), Phase 10 (canonical party, customer, account and transaction tables and their partitions), Phase 11 (raw landing partitions, `admin.batches`), Phases 13 to 17 (feature, KYC, screening and alert partitions and tables), Phase 18 (case seal, `casework.cases.closed_at` and `sealed_at`), Phase 20 (freezes and their `released_at`), Phase 21 (reports and filings). The roadmap row lists phase 18 only; phases 20 and 21 are added because the freeze and report clocks read their tables (Open doubts 21).

**Macro-phase:** P1
**Complexity:** M (9 routes: 6 rows of API.md 3.4 owned here and 3 rows added; 7 tables and 1 sequence in 2 migrations; 3 job kinds; 1 workflow definition and 1 terminal effect; 1 new transit key; the engine and the replay make it heavier than its route count)
**Binaries touched:** akr-api (`LegalHoldService`, the `/retention` router, `LegalHoldWorkflowSubject`, `ReleaseLegalHoldEffect`, `RetentionReportService`) | akr-worker (`RetentionEngine` on its own pool as role `akr_retention`; jobs `retention_run`, `evidence_reconcile`, `retention_lock_extend`; `CertificateService`) | akr-ingestd (one read of `admin.retention_tombstones` in the canonical writers, section 4.5) | akr-ctl (`retention replay`, `retention verify-certificates`) | akr-assess unchanged
**Requirements carried:** SP-53, SP-59, SP-60, M-05 and the retention half of M4 (`docs/MODULES.md` lines 125 to 152: the engine, the holds, the record purge and the dry run); SP-56 (crypto-shredding of locked evidence; the lock is never shortened) and SP-58 (lock extension at closure and for holds), both assigned to this phase by the phase 02 text. No new requirement IDs.
- Gap pass 1 (coverage), implemented here: SP-57 (record purge of whole party trees for relationship-end and case clocks); F-17 (records kept after the relationship ends, with the evidence snapshot, decisions and notes).
- Gap pass 1 (coverage), partly implemented: SP-55 (plan, dry run, detach and deletion certificates), SP-61 (no erasure outside the retention classes; CBS-origin data is never written), SP-63 (the per-class retention report); VD-19 (the seven-year floor, configurable upward).
- Gap pass 1 (coverage), not carried: SP-62 (vendor contract exit export, DR-396), SP-97 (data-subject requests are the bank's process, DR-396); VA-02 (vendor exit export, DR-396).

---

## 1. SQLx Migration Changes

Two migrations, `0071_retention_holds_runs` and `0072_retention_guards_grants`, numbered after phase 21's `0070` (no other phase uses 0071 or 0072; the roadmap chain of `docs/plan/roadmap.md` row 23 names "next numbers (from 0071)"). The schema `admin` exists (phase 00). Seven tables and one sequence are created: `admin.legal_holds`, `admin.legal_hold_copies`, `admin.retention_runs`, `admin.retention_run_items`, `admin.deletion_certificates`, `admin.retention_tombstones`, `admin.evidence_orphans`, and the sequence `admin.hold_ref_seq`. One existing table is altered (`admin.evidence_objects` gains one column; the rest of phase 02's table is unchanged). One SECURITY DEFINER function (`admin.retention_drop_partition`) and two trigger functions (`admin.retention_append_only`, `admin.legal_holds_guard`) are added. No existing row of `admin.retention_assignment` is changed; every new table gets its own row in migration 0072.

**Reconciliation with DATA-MODEL section 18 (aligned in the same change, DR-184):**
- `legal_holds`: DATA-MODEL's `id, scope, reason, placed_by, placed_at, released_by, released_at`. Kept: `id`, `scope` (now the canonical copy of the kind and its keys), `placed_by`, `placed_at`, `released_by`, `released_at`. Changed: `reason` becomes `reason_code` (a closed set) plus `reason_enc` (the free text, sealed). Added: `hold_ref`, `kind`, `party_id`, `case_id`, `range_table`, `range_from`, `range_to`, `state`, `release_instance_id`, `release_reason`, `release_proposed_by` and `version`.
- `retention_runs`: DATA-MODEL's `id, started_at, finished_at, dry_run, summary`. Kept. Added: `trigger`, `requested_by`, `as_of`, `state` and `error_code`.
- `deletion_certificates`: DATA-MODEL's `id, run_id, class, scope, rows, objects, signature, issued_at`. Renamed: `class` becomes `retention_class`, `rows` becomes `row_count` and `objects` becomes `object_count` (`ROWS` is a SQL keyword). Added: `kind` (run or replay), `item_digest`, `key_version`, `worm_object_key` and `worm_version_id`. One row per run, class and kind.
- Not in DATA-MODEL: `legal_hold_copies` (the held-row copies of section 3.4), `retention_run_items` (per-run, per-table counts), `retention_tombstones` (purged source keys that ingest must not reload, section 3.5) and `evidence_orphans` (the reconciliation register of phase 02, 17 and 21 orphans, section 3.9). Each is explained in its DDL comment.
- Retention: section 1.2 gives the rows. The holds, runs, items, certificates and tombstones are R10 (never disposed, DATA-MODEL section 18 and ARCHITECTURE section 17). A held-row copy is R6 on the clock `closure` of `hold_released_at`: it is shredded after its hold is released, never kept as a plain file.

### 1.1 `0071_retention_holds_runs.sql`

```sql
-- migration 0071_retention_holds_runs.sql
-- Phase 23. Legal holds and their held-row copies, retention runs and items, deletion certificates, tombstones of purged
-- source keys, and the evidence orphan register. Schema admin. Ids are UUIDv7 generated by the application.

CREATE SEQUENCE IF NOT EXISTS admin.hold_ref_seq;

-- Legal holds (DATA-MODEL s.18). One row per hold; a hold is never deleted and its placement is never edited.
-- kind party: every row of the party, in every table the registry names (section 3.2). kind case: the parties of the case
-- at each run (casework.case_parties) and the alerts of the case. kind range: every row of the parent table whose partition
-- overlaps the date range.
CREATE TABLE IF NOT EXISTS admin.legal_holds (
    id                  UUID PRIMARY KEY,
    hold_ref            TEXT NOT NULL UNIQUE CHECK (hold_ref ~ '^HLD-[0-9]{4}-[0-9]{6}$'),   -- from admin.hold_ref_seq
    kind                TEXT NOT NULL CHECK (kind IN ('party', 'case', 'range')),
    party_id            UUID,                            -- kind party; no foreign key (canonical party, phase 10)
    case_id             UUID REFERENCES casework.cases(id) ON DELETE RESTRICT,   -- kind case
    range_table         TEXT CHECK (range_table ~ '^(raw|canon|feat|alert|kyc)\.[a-z_][a-z0-9_]*$'),   -- kind range
    range_from          DATE,
    range_to            DATE,
    scope               JSONB NOT NULL CHECK (jsonb_typeof(scope) = 'object' AND scope->>'kind' = kind),
    reason_code         TEXT NOT NULL CHECK (reason_code IN ('regulator_request', 'court_order', 'investigation', 'litigation', 'other')),
    reason_enc          BYTEA,                           -- the free text, sealed (personal, AAD = id); scanned by PolicyPackService
    state               TEXT NOT NULL CHECK (state IN ('active', 'release_requested', 'released')),
    release_proposed_by UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    release_instance_id UUID,                            -- the workflow instance of the release (phase 04)
    release_reason      TEXT CHECK (release_reason IN ('no_longer_required', 'entered_in_error', 'superseded')),
    placed_by           UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    placed_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    released_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    released_at         TIMESTAMPTZ,
    version             INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK ((kind = 'party') = (party_id IS NOT NULL)),
    CHECK ((kind = 'case') = (case_id IS NOT NULL)),
    CHECK ((kind = 'range') = (range_table IS NOT NULL AND range_from IS NOT NULL AND range_to IS NOT NULL)),
    CHECK (range_from IS NULL OR range_to IS NULL OR range_from <= range_to),
    CHECK (state <> 'release_requested' OR release_instance_id IS NOT NULL),
    CHECK ((state = 'released') = (released_at IS NOT NULL AND released_by IS NOT NULL AND release_reason IS NOT NULL)),
    CHECK (released_by IS NULL OR released_by <> placed_by),                       -- the placer never releases
    CHECK (released_by IS NULL OR released_by <> release_proposed_by)              -- four-eyes: proposer and approver differ
);

-- One row per run. A dry run and a real run are the same plan; only trigger and dry_run differ (section 3.1).
CREATE TABLE IF NOT EXISTS admin.retention_runs (
    id            UUID PRIMARY KEY,
    trigger       TEXT NOT NULL CHECK (trigger IN ('schedule', 'dry_run_request', 'replay')),
    dry_run       BOOLEAN NOT NULL,
    requested_by  UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- set for dry_run_request
    as_of         DATE NOT NULL,                  -- the business date the plan is computed for
    state         TEXT NOT NULL CHECK (state IN ('queued', 'running', 'completed', 'failed')),
    error_code    TEXT CHECK (error_code IN ('E_CLAIMED', 'E_UNAVAILABLE', 'E_INTERNAL')),
    summary       JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(summary) = 'object'),
    started_at    TIMESTAMPTZ,
    finished_at   TIMESTAMPTZ,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK ((trigger = 'dry_run_request') = dry_run),                          -- the API can only request dry runs
    CHECK (trigger <> 'dry_run_request' OR requested_by IS NOT NULL),
    CHECK (state <> 'completed' OR finished_at IS NOT NULL)
);

-- Held-row copies (section 3.4): one row per hold, partition and run, written before a partition drop. The copy is an
-- evidence object (kind package, owner_kind legal_hold, sealed). partition_rows is the partition's row count at copy time; the
-- drop compares it under lock, so a row that arrived later blocks the drop until the copy is redone.
CREATE TABLE IF NOT EXISTS admin.legal_hold_copies (
    id                UUID PRIMARY KEY,
    hold_id           UUID NOT NULL REFERENCES admin.legal_holds(id) ON DELETE RESTRICT,
    run_id            UUID NOT NULL REFERENCES admin.retention_runs(id) ON DELETE RESTRICT,
    source_table      TEXT NOT NULL CHECK (source_table ~ '^(raw|canon|feat|alert|kyc)\.[a-z_][a-z0-9_]*$'),
    partition_name    TEXT NOT NULL CHECK (length(partition_name) BETWEEN 1 AND 128),
    partition_rows    BIGINT NOT NULL CHECK (partition_rows >= 0),   -- the partition's row count at copy time
    row_count         BIGINT NOT NULL CHECK (row_count >= 0),         -- the held rows of the hold in the partition
    evidence_id       UUID NOT NULL REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    sha256            BYTEA NOT NULL CHECK (octet_length(sha256) = 32),
    hold_released_at  TIMESTAMPTZ,                     -- the clock of the copy; set by the release effect (section 3.2)
    copied_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (hold_id, source_table, partition_name, run_id)
);

-- Per-run, per-table, per-partition counts. Held counts are visible to M only (section 3.8).
CREATE TABLE IF NOT EXISTS admin.retention_run_items (
    id                UUID PRIMARY KEY,                -- counts only, never a key or a value
    run_id            UUID NOT NULL REFERENCES admin.retention_runs(id) ON DELETE RESTRICT,
    schema_name       TEXT NOT NULL CHECK (schema_name ~ '^[a-z_][a-z0-9_]*$'),
    table_name        TEXT NOT NULL CHECK (table_name ~ '^[a-z_][a-z0-9_]*$'),
    partition_name    TEXT CHECK (length(partition_name) BETWEEN 1 AND 128),   -- partition_drop only
    retention_class   TEXT NOT NULL CHECK (retention_class ~ '^R([1-9]|1[0-3])$'),
    disposal          TEXT NOT NULL CHECK (disposal IN ('partition_drop', 'record_purge', 'evidence_shred', 'copy_only',
                                                        'report_only', 'kept', 'sweep')),
    eligible          BIGINT NOT NULL DEFAULT 0 CHECK (eligible >= 0),
    deleted           BIGINT NOT NULL DEFAULT 0 CHECK (deleted >= 0),
    held              BIGINT NOT NULL DEFAULT 0 CHECK (held >= 0),
    deferred          BIGINT NOT NULL DEFAULT 0 CHECK (deferred >= 0),
    copied            BIGINT NOT NULL DEFAULT 0 CHECK (copied >= 0),
    objects_shredded  BIGINT NOT NULL DEFAULT 0 CHECK (objects_shredded >= 0),
    state             TEXT NOT NULL CHECK (state IN ('planned', 'done', 'blocked', 'failed', 'skipped')),
    reason_code       TEXT CHECK (reason_code IN ('legal_hold', 'not_due', 'copy_stale', 'copy_failed', 'unmapped_batch',
                                                  'referenced', 'open_row', 'lock_timeout', 'floor_applied', 'retired',
                                                  'never', 'not_implemented', 'owner_open')),
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_retention_items_key
    ON admin.retention_run_items (run_id, schema_name, table_name, (COALESCE(partition_name, '')));
CREATE INDEX IF NOT EXISTS idx_retention_items_run ON admin.retention_run_items (run_id, retention_class);

-- Deletion certificates (section 3.7). One row per run, class and kind. No clear key and no clear value: scope lists tables,
-- disposals and partition names; item_digest is SHA-256 over the canonical item list, whose keys are keyed HMACs.
CREATE TABLE IF NOT EXISTS admin.deletion_certificates (
    id                UUID PRIMARY KEY,
    run_id            UUID NOT NULL REFERENCES admin.retention_runs(id) ON DELETE RESTRICT,
    retention_class   TEXT NOT NULL CHECK (retention_class ~ '^R([1-9]|1[0-3])$'),
    kind              TEXT NOT NULL CHECK (kind IN ('run', 'replay')),
    scope             JSONB NOT NULL CHECK (jsonb_typeof(scope) = 'object'),
    row_count         BIGINT NOT NULL CHECK (row_count >= 0),
    object_count      BIGINT NOT NULL CHECK (object_count >= 0),
    item_digest       BYTEA NOT NULL CHECK (octet_length(item_digest) = 32),
    signature         TEXT NOT NULL CHECK (length(signature) BETWEEN 1 AND 512),   -- OpenBao transit signature, vN form
    key_version       INT NOT NULL CHECK (key_version > 0),
    worm_object_key   TEXT NOT NULL CHECK (length(worm_object_key) BETWEEN 1 AND 512),
    worm_version_id   TEXT NOT NULL,
    issued_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (run_id, retention_class, kind)
);

-- Tombstones (section 3.5): a source identity purged from canon. Ingest refuses to write a source identity whose keyed hash is
-- here, so a closed customer still present in the CBS extract is not loaded again. The key is HMAC-SHA256 under the match key.
CREATE TABLE IF NOT EXISTS admin.retention_tombstones (
    schema_name       TEXT NOT NULL CHECK (schema_name ~ '^[a-z_][a-z0-9_]*$'),
    table_name        TEXT NOT NULL CHECK (table_name ~ '^[a-z_][a-z0-9_]*$'),
    source_key_hmac   BYTEA NOT NULL CHECK (octet_length(source_key_hmac) = 32),
    match_key_version INT NOT NULL CHECK (match_key_version > 0),
    run_id            UUID NOT NULL REFERENCES admin.retention_runs(id) ON DELETE RESTRICT,
    purged_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (schema_name, table_name, source_key_hmac)
);

-- Evidence orphans (section 3.9): objects under the evidence bucket with no admin.evidence_objects row (phase 02 put_generated
-- and stage crash windows; phase 17 snapshot orphans; phase 21 package orphans). Deleted only after their COMPLIANCE lock ends.
CREATE TABLE IF NOT EXISTS admin.evidence_orphans (
    id              UUID PRIMARY KEY,
    bucket          TEXT NOT NULL CHECK (bucket = 'akr-evidence'),
    object_key      TEXT NOT NULL CHECK (length(object_key) BETWEEN 1 AND 512),
    version_id      TEXT NOT NULL,
    size_bytes      BIGINT NOT NULL CHECK (size_bytes >= 0),
    retention_class TEXT CHECK (retention_class ~ '^R([1-9]|1[0-3])$'),   -- parsed from the key prefix ev/<class>/
    lock_until      TIMESTAMPTZ,
    found_by        TEXT NOT NULL CHECK (found_by IN ('reconcile', 'counter')),
    found_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at      TIMESTAMPTZ,
    UNIQUE (bucket, object_key, version_id)
);
CREATE INDEX IF NOT EXISTS idx_evidence_orphans_open ON admin.evidence_orphans (lock_until) WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_legal_holds_open ON admin.legal_holds (kind, party_id) WHERE state <> 'released';
CREATE INDEX IF NOT EXISTS idx_legal_holds_case ON admin.legal_holds (case_id) WHERE case_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_legal_hold_copies_hold ON admin.legal_hold_copies (hold_id);
```

### 1.2 `0072_retention_guards_grants.sql`

```sql
-- migration 0072_retention_guards_grants.sql
-- Phase 23. The role akr_retention, the append-only and hold guards, the partition drop function and its guard, the one
-- column added to a phase 02 table, grants per write path, settings, schedules, classification and retention rows.
-- Idempotent: roles, triggers and functions are created with EXCEPTION or OR REPLACE, grants are re-issued.

-- The retention role (phase 00 role list). NOLOGIN here; the LOGIN role akr_retention_login is provisioned by akr-ctl db
-- provision (section 4.5) and is a member of this role only. It is the only role that deletes in canon, kyc and admin rows
-- of section 3.2 and the only role that calls the drop function. It holds the audit writer membership like every writer.
DO $$ BEGIN CREATE ROLE akr_retention NOLOGIN; EXCEPTION WHEN duplicate_object THEN NULL; END $$;
GRANT akr_audit_writer TO akr_retention;

-- Append-only tables: no UPDATE, no DELETE, any role (the phase 17 and 21 pattern).
CREATE OR REPLACE FUNCTION admin.retention_append_only() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog AS $$
BEGIN
    RAISE EXCEPTION '% is append only', TG_TABLE_NAME USING ERRCODE = 'check_violation';
END $$;

DO $$ BEGIN
    CREATE TRIGGER trg_retention_run_items_append BEFORE UPDATE OR DELETE ON admin.retention_run_items
        FOR EACH ROW EXECUTE FUNCTION admin.retention_append_only();
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    CREATE TRIGGER trg_deletion_certificates_append BEFORE UPDATE OR DELETE ON admin.deletion_certificates
        FOR EACH ROW EXECUTE FUNCTION admin.retention_append_only();
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    CREATE TRIGGER trg_retention_tombstones_append BEFORE UPDATE OR DELETE ON admin.retention_tombstones
        FOR EACH ROW EXECUTE FUNCTION admin.retention_append_only();
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

-- Legal holds: never deleted; the placement is immutable; a released hold is final; the placer and the proposer never
-- approve their own release (also a CHECK in 0071 for the columns; this trigger covers the transitions).
CREATE OR REPLACE FUNCTION admin.legal_holds_guard() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'legal hold %: holds are never deleted', OLD.hold_ref USING ERRCODE = 'check_violation';
    END IF;
    IF (NEW.id, NEW.hold_ref, NEW.kind, NEW.party_id, NEW.case_id, NEW.range_table, NEW.range_from, NEW.range_to,
        NEW.scope, NEW.reason_code, NEW.reason_enc, NEW.placed_by, NEW.placed_at)
       IS DISTINCT FROM
       (OLD.id, OLD.hold_ref, OLD.kind, OLD.party_id, OLD.case_id, OLD.range_table, OLD.range_from, OLD.range_to,
        OLD.scope, OLD.reason_code, OLD.reason_enc, OLD.placed_by, OLD.placed_at) THEN
        RAISE EXCEPTION 'legal hold %: the placement is immutable', OLD.hold_ref USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.state = 'released' AND (NEW.state <> 'released' OR NEW.released_at IS DISTINCT FROM OLD.released_at) THEN
        RAISE EXCEPTION 'legal hold %: a released hold cannot be reopened; place a new hold', OLD.hold_ref
            USING ERRCODE = 'check_violation';
    END IF;
    IF NEW.version < OLD.version THEN
        RAISE EXCEPTION 'legal hold %: version never decreases', OLD.hold_ref USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;

DO $$ BEGIN
    CREATE TRIGGER trg_legal_holds_guard BEFORE UPDATE OR DELETE ON admin.legal_holds
        FOR EACH ROW EXECUTE FUNCTION admin.legal_holds_guard();
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

-- One column of a phase 02 table (ALTER, never a redefinition). Set when the object's bytes are deleted after its COMPLIANCE
-- lock ended (section 3.6). A shredded row keeps wrapped_dek NULL and shredded_at set; this column records the byte deletion.
ALTER TABLE admin.evidence_objects ADD COLUMN IF NOT EXISTS object_deleted_at TIMESTAMPTZ;

-- Drop of one partition (section 3.3). SECURITY DEFINER, owned by akr_migrate, executable by akr_retention only. The caller
-- sets lock_timeout = '2s' and runs in its own transaction. Returns one of: dropped, not_due, not_partition, blocked_hold,
-- blocked_copy, blocked_unmapped. A blocked result is never an error: the run records it and the partition stays.
CREATE OR REPLACE FUNCTION admin.retention_drop_partition(p_parent TEXT, p_partition TEXT, p_cutoff DATE, p_run_id UUID)
RETURNS TEXT
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog, admin
AS $$
DECLARE
    v_bound       TEXT;
    v_lower       DATE;
    v_upper       DATE;
    v_rows        BIGINT;
    v_range_holds BIGINT;
    v_party_holds BIGINT;
    v_unmapped    BOOLEAN;
BEGIN
    -- 0. Lock order (section 3.1): the holds table first, then the parent. A hold placement inserts into admin.legal_holds
    -- (ROW EXCLUSIVE), which waits here; a row written into the partition (ROW EXCLUSIVE) waits for the partition lock below.
    LOCK TABLE admin.legal_holds IN SHARE ROW EXCLUSIVE MODE;

    -- 1. Allow-list: the partitioned parents of section 3.3, or a raw landing table created by phase 11.
    IF NOT (p_parent IN ('canon.txn', 'canon.txn_event', 'canon.balance_snapshot', 'canon.card_txn', 'canon.cash_event',
                         'canon.cbs_user_activity', 'canon.fx_deal', 'canon.locker_access', 'canon.payment_message',
                         'canon.payment_message_party', 'feat.feature_values', 'feat.segment_stats',
                         'feat.profiles', 'alert.alerts', 'assess.decisions')
            OR p_parent ~ '^raw\.[a-z_][a-z0-9_]*$') THEN
        RAISE EXCEPTION 'retention: % is not a droppable parent', p_parent USING ERRCODE = 'check_violation';
    END IF;

    -- 2. The partition is a child of the parent, and its bounds are read from the catalogue.
    IF NOT EXISTS (SELECT 1 FROM pg_inherits i
                    WHERE i.inhparent = to_regclass(p_parent) AND i.inhrelid = to_regclass(p_partition)) THEN
        RETURN 'not_partition';
    END IF;
    SELECT pg_get_expr(c.relpartbound, c.oid) INTO v_bound FROM pg_class c WHERE c.oid = to_regclass(p_partition);
    v_lower := substring(v_bound FROM 'FROM \(''([^'']+)''\)')::date;
    v_upper := substring(v_bound FROM 'TO \(''([^'']+)''\)')::date;
    IF v_upper > p_cutoff THEN
        RETURN 'not_due';
    END IF;

    -- 3. Range holds that overlap the partition block the drop: a range is never kept by a copy.
    SELECT count(*) INTO v_range_holds FROM admin.legal_holds h
     WHERE h.state <> 'released' AND h.kind = 'range' AND h.range_table = p_parent
       AND daterange(h.range_from, h.range_to, '[]') && daterange(v_lower, v_upper, '[)');
    IF v_range_holds > 0 THEN
        RETURN 'blocked_hold';
    END IF;

    -- 4. The partition (not the parent) is locked against writers, so the count below is final for this drop; writers of
    --    the other partitions go on. The caller sets statement_timeout from retention.drop_timeout_s (lead review).
    EXECUTE format('LOCK TABLE %s IN SHARE MODE', to_regclass(p_partition));
    EXECUTE format('SELECT count(*) FROM %s', to_regclass(p_partition)) INTO v_rows;

    -- 5. Party and case holds need a copy of this partition made in this run, whose partition_rows equals the count now.
    --    A raw landing table is neither copied nor blocked by party or case holds: its rows are mapped into canon in the
    --    same transaction (phase 11), the held copy is taken from canon, and step 6 keeps unmapped batches (lead review).
    SELECT count(*) INTO v_party_holds FROM admin.legal_holds h
     WHERE h.state <> 'released' AND h.kind IN ('party', 'case')
       AND p_parent !~ '^raw\.'
       AND NOT EXISTS (SELECT 1 FROM admin.legal_hold_copies c
                        WHERE c.hold_id = h.id AND c.run_id = p_run_id
                          AND c.source_table = p_parent AND c.partition_name = p_partition
                          AND c.partition_rows = v_rows);
    IF v_party_holds > 0 THEN
        RETURN 'blocked_copy';
    END IF;

    -- 6. Raw rows whose batch is not yet mapped into canon are still needed (phase 11): the partition stays.
    IF p_parent ~ '^raw\.' THEN
        EXECUTE format('SELECT EXISTS (SELECT 1 FROM %s r JOIN admin.batches b ON b.id = r.batch_id
                         WHERE b.mapping_version_id IS NULL)', to_regclass(p_partition)) INTO v_unmapped;
        IF v_unmapped THEN
            RETURN 'blocked_unmapped';
        END IF;
    END IF;

    -- 7. Drop. DETACH takes the parent lock under the caller's lock_timeout (2 s). The audit event is written by the
    --    caller in the same transaction.
    EXECUTE format('ALTER TABLE %s DETACH PARTITION %s', to_regclass(p_parent), to_regclass(p_partition));
    EXECUTE format('DROP TABLE %s', to_regclass(p_partition));
    RETURN 'dropped';
END $$;

REVOKE ALL ON FUNCTION admin.retention_drop_partition(TEXT, TEXT, DATE, UUID) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION admin.retention_drop_partition(TEXT, TEXT, DATE, UUID) TO akr_retention;

-- Grants per write path (DATA-MODEL s.21 style; no grant on a table not listed here).
GRANT USAGE ON SEQUENCE admin.hold_ref_seq TO akr_app;

-- akr_app: holds (place, propose a release, the release effect inside the approval transaction), dry-run requests, reads.
GRANT SELECT, INSERT ON admin.legal_holds TO akr_app;
GRANT UPDATE (state, release_proposed_by, release_instance_id, release_reason, released_by, released_at, version, updated_at)
    ON admin.legal_holds TO akr_app;
GRANT SELECT, INSERT ON admin.retention_runs TO akr_app;        -- INSERT only with trigger dry_run_request (CHECK, 0071)
GRANT SELECT ON admin.retention_run_items, admin.deletion_certificates, admin.legal_hold_copies TO akr_app;
GRANT UPDATE (hold_released_at) ON admin.legal_hold_copies TO akr_app;   -- set once by the release effect
GRANT SELECT ON admin.evidence_orphans TO akr_app;              -- counts for the cockpit only

-- akr_retention: the engine, the reconciliation, the lock extension and the replay.
GRANT SELECT, INSERT ON admin.retention_runs TO akr_retention;
GRANT UPDATE (state, started_at, finished_at, summary, error_code) ON admin.retention_runs TO akr_retention;
GRANT SELECT, INSERT ON admin.retention_run_items TO akr_retention;
GRANT SELECT, INSERT ON admin.legal_hold_copies TO akr_retention;
GRANT SELECT, INSERT ON admin.deletion_certificates TO akr_retention;
GRANT SELECT, INSERT ON admin.retention_tombstones TO akr_retention;
GRANT SELECT, INSERT ON admin.evidence_orphans TO akr_retention;
GRANT UPDATE (deleted_at) ON admin.evidence_orphans TO akr_retention;
GRANT SELECT ON admin.legal_holds, admin.retention_assignment, admin.settings, admin.batches,
                admin.evidence_objects, admin.workflow_instances, admin.column_classification, admin.quarantine_items TO akr_retention;
GRANT UPDATE (wrapped_dek, shredded_at, object_deleted_at, lock_until, updated_at) ON admin.evidence_objects TO akr_retention;
GRANT USAGE ON SCHEMA canon, feat, alert, kyc, casework, rpt, scn, ref TO akr_retention;
GRANT SELECT ON ALL TABLES IN SCHEMA canon, feat, alert, kyc, casework, rpt, scn, ref TO akr_retention;
-- Record purge (section 3.2): the only DELETE grants in the schema canon and in the kyc owner family. No other role has DELETE
-- on canon (DATA-MODEL s.21, phase 10). Partitions are never deleted by row: they go through admin.retention_drop_partition.
GRANT DELETE ON canon.party, canon.party_name, canon.party_identifier, canon.party_contact, canon.party_relationship,
                canon.customer, canon.account, canon.account_party_role, canon.account_hold, canon.card,
                canon.facility, canon.trade_instrument, canon.kyc_document,
                admin.xref, admin.attribute_values, admin.quarantine_items,
                kyc.ratings, kyc.rating_current, kyc.rating_feed, kyc.overrides, kyc.kyc_reviews, kyc.review_documents,
                kyc.review_queries, kyc.ownership_inputs, kyc.ownership_results, kyc.declared_profiles, kyc.pep_register,
                kyc.pep_relatives
    TO akr_retention;

-- akr_ingest: reads the tombstones to refuse a purged source identity (section 3.5). Nothing else.
GRANT SELECT ON admin.retention_tombstones TO akr_ingest;

-- Settings (copy of the phase 21 column list). The period settings are governed: a change is a change request (cr_material,
-- phase 04) and a value below the floor is refused with E_BELOW_FLOOR (phase 09). The engine also takes max(value, floor).
INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('retention.period_days.r1', '90', 'integer', '{"min": 30, "basis": "ARCHITECTURE s.17 range 30 to 400; the 30 is a product bound, not a statutory floor (Open doubts 7)"}'::jsonb, TRUE,
  '{"ar": "مدة الاحتفاظ بالبيانات الخام بالأيام", "en": "Raw landing retention in days (R1)"}'),
 ('retention.period_years.r2', '10', 'integer', '{"min": 7, "basis": "Iraq profile floor 7 years (research 30 s.12.3, D2.1(c)(1)); default 10"}'::jsonb, TRUE,
  '{"ar": "مدة الاحتفاظ بنسخة المراقبة للمعاملات بالسنوات", "en": "Monitoring copy of transactions in years (R2)"}'),
 ('retention.period_years.r3', '7', 'integer', '{"min": 7, "basis": "Iraq profile floor 7 years after the relationship ends (D2.1(c)(1))"}'::jsonb, TRUE,
  '{"ar": "مدة الاحتفاظ ببيانات العميل بعد انتهاء العلاقة بالسنوات", "en": "Master data after the relationship ends, in years (R3)"}'),
 ('retention.period_years.r4', '3', 'integer', '{"min": 1, "basis": "research 30 s.12.3 gives no floor; the longest scenario look-back is added at run time (Open doubts 6)"}'::jsonb, TRUE,
  '{"ar": "مدة الاحتفاظ بالميزات والتجميعات بالسنوات", "en": "Features and aggregates in years (R4)"}'),
 ('retention.period_years.r5', '10', 'integer', '{"min": 7, "basis": "Iraq profile floor 7 years from the report date (D2.1(c)(1)); default 10"}'::jsonb, TRUE,
  '{"ar": "مدة الاحتفاظ بالتنبيهات بالسنوات", "en": "Alerts in years (R5)"}'),
 ('retention.period_years.r6', '7', 'integer', '{"min": 7, "basis": "Iraq profile floor 7 years from the later of closure and report (D2.1(c)(1)); default 10"}'::jsonb, TRUE,
  '{"ar": "مدة الاحتفاظ بالقضايا والتقارير بالسنوات", "en": "Cases and reports in years (R6)"}'),
 ('retention.period_years.r7', '7', 'integer', '{"min": 7, "basis": "Iraq profile floor 7 years (D2.1(c)(1)); 10 where OFAC applies (Open doubts 9)"}'::jsonb, TRUE,
  '{"ar": "مدة الاحتفاظ بسجلات الفحص والعقوبات بالسنوات", "en": "Screening and sanctions records in years (R7)"}'),
 ('retention.period_years.r8', '7', 'integer', '{"min": 7, "basis": "Iraq profile floor 7 years after the relationship ends (D2.1(c)(1))"}'::jsonb, TRUE,
  '{"ar": "مدة الاحتفاظ بسجل التصنيف بعد انتهاء العلاقة بالسنوات", "en": "Rating history after the relationship ends, in years (R8)"}'),
 ('retention.held_copy_grace_days', '30', 'integer', '{"min": 0, "basis": "a held copy may only be kept longer"}'::jsonb, TRUE,
  '{"ar": "أيام الانتظار بعد رفع الحجز قبل إتلاف نسخة الصفوف المحجوزة", "en": "Days after a hold is released before its row copy is shredded"}'),
 ('retention.batch_rows', '5000', 'integer', NULL, FALSE,
  '{"ar": "عدد الصفوف في الدفعة الواحدة للحذف", "en": "Rows per deletion batch"}'),
 ('retention.batch_pause_ms', '200', 'integer', NULL, FALSE,
  '{"ar": "فترة توقف بين دفعات الحذف بالمللي ثانية", "en": "Pause between deletion batches in milliseconds"}'),
 ('retention.max_run_minutes', '120', 'integer', NULL, FALSE,
  '{"ar": "أقصى مدة لتشغيل الاحتفاظ بالدقائق", "en": "Maximum duration of one retention run in minutes"}'),
 ('retention.drop_timeout_s', '600', 'duration_s', NULL, FALSE,
  '{"ar": "مهلة معاملة حذف القسم الواحد بالثواني", "en": "Time-out of one partition drop transaction in seconds"}')
ON CONFLICT (key) DO NOTHING;

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('schedule.retention_run', '"0 45 3 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول تشغيل الاحتفاظ الليلي (الحذف الفعلي)", "en": "Nightly retention run (real deletion)"}'),
 ('schedule.evidence_reconcile', '"0 10 4 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مطابقة ملفات الأدلة مع سجلاتها", "en": "Daily reconciliation of evidence objects with their rows"}')
ON CONFLICT (key) DO NOTHING;

-- Classification (DATA-MODEL s.0 classes). encrypted only where the column is sealed; keyed_hash only for the HMAC twin.
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification, encrypted, keyed_hash)
SELECT 'admin', v.t, c, v.cls::admin.data_classification, v.enc, v.hk
FROM (VALUES
  ('legal_holds', 'internal', FALSE, FALSE, ARRAY['id', 'hold_ref', 'kind', 'party_id', 'case_id', 'range_table', 'range_from',
                                                  'range_to', 'scope', 'reason_code', 'state', 'release_proposed_by',
                                                  'release_instance_id', 'release_reason', 'placed_by', 'placed_at',
                                                  'released_by', 'released_at', 'version', 'created_at', 'updated_at']),
  ('legal_holds', 'personal', TRUE, FALSE, ARRAY['reason_enc']),
  ('legal_hold_copies', 'internal', FALSE, FALSE, ARRAY['id', 'hold_id', 'run_id', 'source_table', 'partition_name',
                                                        'partition_rows', 'row_count', 'evidence_id', 'sha256',
                                                        'hold_released_at', 'copied_at']),
  ('retention_runs', 'internal', FALSE, FALSE, ARRAY['id', 'trigger', 'dry_run', 'requested_by', 'as_of', 'state', 'error_code',
                                                     'summary', 'started_at', 'finished_at', 'created_at']),
  ('retention_run_items', 'internal', FALSE, FALSE, ARRAY['id', 'run_id', 'schema_name', 'table_name', 'partition_name',
                                                          'retention_class', 'disposal', 'eligible', 'deleted', 'held',
                                                          'deferred', 'copied', 'objects_shredded', 'state', 'reason_code',
                                                          'created_at']),
  ('deletion_certificates', 'internal', FALSE, FALSE, ARRAY['id', 'run_id', 'retention_class', 'kind', 'scope', 'row_count',
                                                            'object_count', 'item_digest', 'signature', 'key_version',
                                                            'worm_object_key', 'worm_version_id', 'issued_at']),
  ('retention_tombstones', 'internal', FALSE, FALSE, ARRAY['schema_name', 'table_name', 'match_key_version', 'run_id', 'purged_at']),
  ('retention_tombstones', 'personal', FALSE, TRUE, ARRAY['source_key_hmac']),
  ('evidence_orphans', 'internal', FALSE, FALSE, ARRAY['id', 'bucket', 'object_key', 'version_id', 'size_bytes', 'retention_class',
                                                       'lock_until', 'found_by', 'found_at', 'deleted_at']),
  ('evidence_objects', 'internal', FALSE, FALSE, ARRAY['object_deleted_at'])
) AS v(t, cls, enc, hk, cols)
CROSS JOIN LATERAL unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

-- Retention rows for every table created by this phase (phase 00 CHECKs: R10 only with clock never; a clock other than
-- partition, age, relationship_end, closure or decision needs clock_column NULL). The engine reads these rows (section 3.2).
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'legal_holds',           'R10', 'never',   NULL,              'Legal holds: kept without limit (DATA-MODEL s.18)'),
 ('admin', 'legal_hold_copies',     'R6',  'closure', 'hold_released_at', 'Held-row copies: shredded after the hold is released plus retention.held_copy_grace_days (section 3.4)'),
 ('admin', 'retention_runs',        'R10', 'never',   NULL,              'Run records: kept without limit (DATA-MODEL s.18)'),
 ('admin', 'retention_run_items',   'R10', 'never',   NULL,              'Per-run counts: kept with the run'),
 ('admin', 'deletion_certificates', 'R10', 'never',   NULL,              'Deletion certificates: R10 (ARCHITECTURE s.17)'),
 ('admin', 'retention_tombstones',  'R10', 'never',   NULL,              'Purged source identities: kept so that ingest never reloads a purged closed customer'),
 ('admin', 'evidence_orphans',      'R12', 'age',     'deleted_at',      'Orphan register: 90 days after the object is deleted; a row not yet deleted is never due')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

---

## 2. DDD Implementation (Rust)

Crate placement follows phase 00: `domain` (entities, enums, traits), `app` (services), `db` (repositories), `api` (routers, akr-api), `worker` (engine and jobs, akr-worker), `ctl` (akr-ctl). The retention code lives in a module `retention` in `app` and `db`; the legal hold code in `legal_hold`.

**Domain entities** (`Debug, Clone, Serialize, Deserialize, FromRow, ToSchema`):
- `LegalHold { id, hold_ref, kind: HoldKind, party_id: Option<Uuid>, case_id: Option<Uuid>, range: Option<HoldRange>, reason_code: HoldReason, state: HoldState, release_instance_id: Option<Uuid>, release_reason: Option<ReleaseReason>, placed_by: Uuid, placed_at, released_by: Option<Uuid>, released_at: Option<DateTime<Utc>>, version: i32 }`. `reason_enc` is never in the struct; the free text is decrypted only into the detail view of M (section 3.8).
- `HoldKind { Party, Case, Range }`; `HoldState { Active, ReleaseRequested, Released }`; `HoldReason { RegulatorRequest, CourtOrder, Investigation, Litigation, Other }`; `ReleaseReason { NoLongerRequired, EnteredInError, Superseded }`; `HoldRange { table: String, from: NaiveDate, to: NaiveDate }`.
- `RetentionRun { id, trigger: RunTrigger, dry_run: bool, requested_by: Option<Uuid>, as_of: NaiveDate, state: RunState, error_code: Option<ProblemCode>, summary: serde_json::Value, started_at, finished_at }`; `RunTrigger { Schedule, DryRunRequest, Replay }`; `RunState { Queued, Running, Completed, Failed }`.
- `RunItem { run_id, schema_name, table_name, partition_name: Option<String>, retention_class: RetentionClass, disposal: Disposal, eligible, deleted, held, deferred, copied, objects_shredded, state: ItemState, reason_code: Option<ItemReason> }`.
- `Disposal { PartitionDrop, RecordPurge, EvidenceShred, CopyOnly, ReportOnly, Kept, Sweep }`; `ItemState { Planned, Done, Blocked, Failed, Skipped }`.
- `RetentionClass(u8)` (R1 to R13, validated against the regex of phase 00); `RetentionClock { Partition, Age, RelationshipEnd, Closure, Decision, Retirement, Owner, Never }` (the phase 00 enum).
- `DeletionCertificate { id, run_id, retention_class, kind: CertKind, scope: serde_json::Value, row_count: i64, object_count: i64, item_digest: [u8; 32], signature: String, key_version: i32, worm_object_key, worm_version_id, issued_at }`; `CertKind { Run, Replay }`.
- `EvidenceOrphan { id, bucket, object_key, version_id, size_bytes, retention_class: Option<RetentionClass>, lock_until, found_by, found_at, deleted_at }`.

**Create and update DTOs** (`Deserialize, ToSchema`):
- `PlaceLegalHold { kind: HoldKind, party_id: Option<Uuid>, case_id: Option<Uuid>, range: Option<HoldRange>, reason_code: HoldReason, reason_text: Option<String> }` (`reason_text` at most 500 characters; scanned, section 3.8).
- `ProposeHoldRelease { release_reason: ReleaseReason }` (the route is versioned: `If-Match` is the hold `version`).
- `DryRunRequest {}` (empty body; `as_of` is the server's bank business date).
- `HoldFilter { kind, state, cursor, limit }`, `RunFilter { dry_run, cursor, limit }`, `CertificateFilter { run_id, cursor, limit }`.

**Repository traits** (domain layer):
- `LegalHoldRepository`: `insert(tx, NewLegalHold) -> LegalHold`; `get(id) -> Option<LegalHold>`; `get_for_update(tx, id) -> Option<LegalHold>` (`SELECT ... FOR UPDATE`); `list(filter, principal_view) -> Page<LegalHold>`; `set_release_requested(tx, id, expected_version, proposed_by, instance_id) -> bool`; `set_released(tx, id, expected_version, released_by, reason) -> bool`; `set_active(tx, id, expected_version) -> bool` (a rejected or ended release); `active_for_party(tx, party_id) -> Vec<LegalHold>`; `active_ranges(tx, table, from, to) -> Vec<LegalHold>`; `active_case_holds() -> Vec<LegalHold>`; `reason_text(id) -> Option<String>` (read for the detail view of M only, section 3.8).
- `HoldCopyRepository`: `insert(tx, NewHoldCopy) -> Uuid`; `exists(tx, hold_id, run_id, table, partition) -> bool`; `set_released_for_hold(tx, hold_id, at)`.
- `RetentionRunRepository`: `create(NewRun) -> RetentionRun`; `mark_running(tx, id)`; `add_item(tx, RunItem)`; `finish(tx, id, state, summary, error_code)`; `get(id)`; `items(run_id, include_held: bool) -> Vec<RunItem>` (`include_held = false` sets `held` to `None` in the view); `latest_completed() -> Option<RetentionRun>`.
- `CertificateRepository`: `insert(tx, DeletionCertificate)`; `list(filter) -> Page<DeletionCertificate>`.
- `TombstoneRepository`: `insert(tx, schema, table, hmac, match_key_version, run_id)` (ON CONFLICT DO NOTHING); `exists(schema, table, hmac) -> bool` (used by the ingest writers).
- `OrphanRepository`: `upsert_found(batch)`, `mark_deleted(tx, id, at)`, `due_for_delete(now, limit) -> Vec<EvidenceOrphan>`.

**Disposal registry** (`DisposalHandler`, one implementation per registered `(schema, table)`; section 3.2 gives the list): `fn key(&self) -> (&'static str, &'static str)`, `fn disposal(&self) -> Disposal`, `fn clock(&self) -> RetentionClock`, `fn parent_key(&self) -> Option<ParentKey>` (for owner-following children), `async fn plan(&self, ctx: &mut PlanCtx) -> Result<PlanItem, RetentionError>`, `async fn execute(&self, ctx: &mut ExecCtx, batch: Batch) -> Result<ExecOutcome, RetentionError>`. The registry is built at start-up from a constant list and checked by a unit test: every row of `admin.retention_assignment` maps to exactly one handler, and every handler key has a row (section 6, check 4a).

**Ports** (defined in this phase; the owning phase implements or the concrete type is named):
- `EvidenceShredder` (phase 02 `EvidenceStore`): `shred(tx, evidence_id, reason) -> ShredOutcome` (erases `wrapped_dek`, sets `shredded_at`, audit `retention.evidence_shredded`); `delete_object(evidence_id) -> ()` (only after `lock_until`; the S3 guard of phase 02 refuses earlier); `extend_lock(tx, evidence_id, until)` (the guard of phase 02 accepts only a later date).
- `CertificateSigner` (OpenBao transit key `akr-sign-cert`, section 4.4): `sign(payload: &[u8]) -> Result<(String, i32), SignError>`; `verify(payload, signature) -> Result<bool, SignError>`.
- `WormWriter` (the phase 02 bucket `akr-evidence`, COMPLIANCE mode, under the prefix `certificates/`; every PUT sets its own lock, section 3.7): `put_locked(key, bytes, until) -> VersionId`; `get(key, version) -> bytes`.
- `MatchKeyHmac` (phase 10 match key; `hmac(domain, bytes) -> [u8; 32]` with the key version): used for `source_key_hmac` and the certificate item keys.
- `BusinessClock` (phase 09 `BankCalendarClock`): `as_of() -> NaiveDate` for the run date.
- `PartitionDropper`: the one call to `admin.retention_drop_partition` (section 1.2), which returns the drop outcome.

**Services** (`akr-api` unless marked):
- `LegalHoldService { holds: Arc<dyn LegalHoldRepository>, copies: Arc<dyn HoldCopyRepository>, workflow: Arc<WorkflowEngine>, scan: Arc<PolicyPackService>, audit: Arc<dyn AuditSink>, stepup: Arc<StepUpService>, ids: IdGen, clock: Arc<dyn Clock>, cases: Arc<dyn CaseLookup> }`, methods `place(ctx, PlaceLegalHold, idem) -> LegalHold` (section 3.8), `propose_release(ctx, id, ProposeHoldRelease, if_match, idem) -> ReleaseProposal` (section 3.8), `list(ctx, HoldFilter) -> Page<LegalHoldView>`, `get(ctx, id) -> LegalHoldView`. `LegalHoldView` (so named because phase 29 has a `HoldView` in the same OpenAPI document) is the view of section 3.8 (the scope, the party or case id and the reason text are fields that the view builder omits for principals without the right).
- `ReleaseLegalHoldEffect` (`TerminalEffect`, kind `release_legal_hold`, resource kinds `["legal_hold"]`; registered in `EffectRegistry` at start-up; runs inside the approval transaction).
- `LegalHoldWorkflowSubject` (`WorkflowSubject`, resource kind `legal_hold`, attribute prefix `hold`; attributes `state` (text), `kind` (text), `has_case` (boolean), `range_table` (text)).
- `RetentionReportService { runs, items, clock }` (`akr-api`): `latest_report(ctx) -> RetentionReportView` (section 3.10).
- `RetentionEngine` (`akr-worker`, section 3.1): `execute(run_id) -> RunSummary` (the value stored in the `summary` column of section 1.1: the class totals, `truncated`, and the later-of count and earliest due date of section 3.10); holds the `RetentionPool` (role `akr_retention`) and the `DisposalRegistry`.
- `CertificateService` (`akr-worker`): `issue(tx, run_id, retention_class, kind, manifests) -> DeletionCertificate` (section 3.7).
- `EvidenceReconciler` (`akr-worker`, job `evidence_reconcile`; section 3.9).
- `LockExtensionService` (`akr-worker`, job `retention_lock_extend`; section 3.6 step 4 and section 3.5).
- `ReplayService` (`akr-ctl`, section 3.7): `replay(source) -> ReplayReport`; `verify_certificates(source) -> VerifyReport`.
- `CaseLookup` (phase 18 read port): the parties of a case (`casework.case_parties`), its alerts (`casework.case_alerts`) and its seal instant (`casework.cases.sealed_at`); used by holds (section 3.8) and the lock extension (section 3.6).

**utoipa schemas:** `PlaceLegalHold`, `ProposeHoldRelease`, `LegalHoldView`, `HoldPage`, `ReleaseProposal` (the workflow instance and its open task, phase 04 view), `DryRunRequest`, `RetentionRunView`, `RunItemView`, `RetentionReportView`, `DeletionCertificateView`, `CertificatePage`.

**Route table** (the six rows of API.md 3.4 owned here, marked as they are; three rows added in this phase, marked **added**; markers as in API.md section 3):

| Method | Path | Scope | Description |
|-|-|-|-|
| GET | `/api/v1/retention/holds` | `audit.read` | Legal holds, cursor page. For IA and SEC, case holds are omitted and party holds show `kind` and `scope` `restricted` (section 3.8) |
| POST | `/api/v1/retention/holds` | `report.file` | Place a legal hold (party, case or range); **SU** `legal_hold.place` (class `decision`); `Idempotency-Key` |
| GET | `/api/v1/retention/holds/{id}` | `audit.read` | **added.** One hold; a case hold is `E_NOT_FOUND` for every principal except M (section 3.8) |
| POST | `/api/v1/retention/holds/{id}/release` | `report.file` | **added detail:** starts the workflow `legal_hold_release`; `If-Match` (hold version); the approval is `POST /api/v1/tasks/{id}/actions/approve` (phase 04) with **SU** `legal_hold.release` (class `decision`) and **4E** |
| GET | `/api/v1/retention/report` | `audit.read`, `admin.system` | What is due by class and what is held, from the latest completed run (section 3.10); `held` is `null` for IA, SEC and ADM |
| POST | `/api/v1/retention/dry-runs` | `admin.system` | Simulate a retention run; `202` with the run id; no deletion (section 3.1) |
| GET | `/api/v1/retention/runs` | `audit.read`, `admin.system` | **added.** Runs, newest first, dry and real |
| GET | `/api/v1/retention/runs/{id}` | `audit.read`, `admin.system` | **added.** One run with its items; `held` is `null` for IA, SEC and ADM |
| GET | `/api/v1/retention/certificates` | `audit.read` | Deletion certificates: run, class, kind, counts, digest, signature; never a clear key or value |

Routes of the other phases that this phase calls: `POST /api/v1/tasks/{id}/actions/approve` (phase 04), the case seal (phase 18, which enqueues `retention_lock_extend`; section 4.5). No route of another phase is changed by this phase's text except the seal transaction (section 4.5 item 1).

**Router:** `pub fn router() -> Router<AppState>` in `api/src/routers/retention.rs`, mounted under `/api/v1/retention`. The hold routes share the `legal_hold` module; the router does not hold any business rule.

---

## 3. Business Logic

### 3.1 The engine: plan, dry run and real run (`RetentionEngine::execute`; job `retention_run`)

One code path plans and runs. A **dry run** and a **real run** compute the same plan with the same predicates and the same holds; the difference is only that the real run executes the disposals of section 3.2 to 3.6. A run is one `admin.retention_runs` row. `POST /retention/dry-runs` (`admin.system`) inserts the row with `trigger = dry_run_request`, `dry_run = TRUE`, `requested_by` and `as_of`, and enqueues `retention_run` with payload `{ run_id }`. The nightly schedule inserts the row with `trigger = schedule` and `dry_run = FALSE`; a replay inserts `trigger = replay`.

Steps of `execute(run_id)`:
1. Take the session advisory lock `pg_try_advisory_lock(hashtext('akr.retention'))` on the retention connection. Not acquired: the run is marked `failed` with `error_code = E_CLAIMED` and the job ends (another run is active; the schedule retries the next night). Released at the end of the run.
2. `as_of` is read from the run row (never from the clock at execution time, so a run that starts late plans for its own date). Set `state = running`, `started_at = now()`.
3. Load the effective periods (section 3.10) and the registry (section 3.2). Every row of `admin.retention_assignment` is considered, in ascending class order and then by table name. A row with no handler is recorded as `not_implemented` (the coverage test of section 6 makes this impossible in a release build).
4. For each row, compute the plan item: `eligible` (due rows or due partitions by the clock of section 3.2), then `held`, `deferred` and `referenced` (section 3.5), then the disposal. Each plan item becomes one `admin.retention_run_items` row with `state = planned`.
5. Real run only, per class and per table, in batches of `retention.batch_rows` (section 3.5 and 3.3): for each batch (a) build the batch manifest (section 3.7: the HMAC keys or the partition name, the counts and the `as_of`), (b) sign it with `CertificateSigner` and write it to the WORM bucket, (c) run the disposal of the batch in one transaction that also writes the audit event and the item counts, (d) commit, then `pg_sleep` for `retention.batch_pause_ms`. The manifest is written before the rows it names are deleted, so no deletion happens without a signed record of what it deleted.
6. Stop early when `now() - started_at > retention.max_run_minutes`: the remaining items stay `planned`, the summary carries `truncated = true`, and the next night continues from the plan (the plan is recomputed, so nothing is remembered in the database beyond the items and the manifests).
7. After each class: issue the class certificate (section 3.7). Then `finish(state = completed, summary)` with the class totals and the audit event `retention.run_finished`.
8. Any error that is not a blocked result: the affected item is `failed` with `reason_code` and an audit event; the run continues with the other tables. `E_UNAVAILABLE` (OpenBao or WORM unreachable) stops the run before any delete of that class: nothing is deleted without its manifest. A run with any failed item finishes `failed` with `error_code = E_INTERNAL` when the failure is not an outage, else `E_UNAVAILABLE`.
9. Release the advisory lock.

Dry run: steps 1 to 4, and the summary of step 7, only. Step 5 is replaced by the item rows with `deleted = 0`, `copied = 0`, `objects_shredded = 0`; `held` and `eligible` are exact; no manifest, no certificate, no copy, no shred, no tombstone, no lock extension, no `DROP`, no `DELETE`. The dry run's audit event is `retention.run_finished` with `dry_run = true`. Section 6 check 1 asserts that nothing was deleted.

Batch and load limits: `retention.batch_rows` (default 5000) rows per batch; `retention.batch_pause_ms` (default 200) between batches; `retention.max_run_minutes` (default 120); `SET LOCAL statement_timeout = '30s'` per batch (a partition drop transaction uses `retention.drop_timeout_s`, default 600 s, because it counts the partition); `SET LOCAL lock_timeout = '2s'` for every DDL and every party lock. No lock is held across batches. The schedule (`schedule.retention_run`, default 03:45 bank time) is the window; the job is never run from a request.

Concurrency: the job kind has concurrency 1 and the coalesce key `retention_run:<run_id>`, so two runs are never planned at once; the advisory lock of step 1 also covers a replay started by hand.

### 3.2 Classes, floors, clocks and the disposal registry

**Effective period.** For every class with a period, `effective(class) = max(configured, floor.min)`, where `configured` is the governed setting `retention.period_*` (section 3.10) and `floor.min` is the floor stored in the same setting row. The engine also writes `retention.floor_applied` (audit, once per run and class) when the configured value was below the floor. A jurisdiction profile floor (OFAC 10 years for R7, EU 5-year deletion) is not built in this phase (Open doubts 9); the engine has no profile input and the term is 0.

**Cut-off.** `cutoff(class) = as_of - effective(class) years` (R1: `as_of - days`). A row or partition is due when its clock value is on or before the cut-off, by the rules of the clock below.

**Clocks** (`retention_assignment.clock`; the resolver is named in the registry):
- `partition`: a partition is due when its exclusive upper bound is on or before the cut-off (`pg_inherits` and `pg_get_expr(relpartbound)`). Raw landing partitions are daily UTC; canon and feature partitions are monthly.
- `age`: `clock_column <= cutoff`. A NULL clock value is never due.
- `closure`: `clock_column <= cutoff` and the owner is closed or sealed (`casework.cases` with `sealed_at`, `casework.freezes` with `released_at`, `rpt.report_tasks` with `closed_at`). An open owner is never due.
- `decision`: `clock_column` (the decision instant) `<= cutoff`.
- `relationship_end`: the party end date of section 3.5 `<= cutoff`.
- `owner`: no clock of its own; the row is deleted in the same transaction as its owner, in the owner's batch (section 3.5), and never before it.
- `retirement`: never due in this phase (the configuration history stays, research 30 s.12.3: "kept (small)").
- `never`: R10; never due, never deleted.

**The later-of rule (reports).** `rpt.reports` is assigned `closure` on `created_at`, which is only the column the assignment names. The due instant of a report is `greatest(casework.cases.sealed_at for its case_id, rpt.filings.filed_at for the report)`; a case-less report (CTR) uses its filing alone; a report with no filing and no sealed case is not due. The engine computes the number of reports due under this rule into the run summary (section 3.10), and `RetentionReportService` reads it from there. The rows are not deleted in this phase (DR-191).

**Registry.** The disposal registry is the list below: each `(schema, table)` of the assignment table has one handler with its disposal. The engine never deletes a table that has no handler, and the coverage test (section 6, check 4a) fails the build when a row has no handler or a handler has no row.

| Class | Clock in the assignment | Iraq floor (setting) | Default | Disposal in this phase | Tables |
|-|-|-|-|-|-|
| R1 | `partition` on `extracted_at` | 30 days (product bound) | 90 days | `partition_drop` | `raw.<source>` landing partitions (phase 11) |
| R1 | `age` on `created_at` | 30 days (product bound) | 90 days | `record_purge` by age in batches (lead review) | `admin.quarantine_items` (phase 11; partitioned by `extracted_at`, purged by row because its assignment clock is `age`) |
| R2 | `partition` on `booking_date`, `business_date`, `at`, `value_date`, `received_at` | 7 years | 10 years | `partition_drop` | the ten partitioned canon tables of phase 10 |
| R2 | `age` | 7 years | 10 years | `report_only` | `canon.counterparty` (`last_seen`), `canon.payment_link` (`created_at`); their purge needs the reference check of section 3.5 (Open doubts 22) |
| R3 | `owner` and `relationship_end` | 7 years after the end | 7 years | `record_purge` by party (section 3.5) | the party family, `customer`, `account` (`closed_on`), `account_party_role`, `account_hold`, `card`, `facility`, `trade_instrument`, `kyc_document`, `admin.xref`, `admin.attribute_values` |
| R3 | `owner` (staff) | 7 years | 7 years | `report_only` | `canon.staff` (no customer link) |
| R4 | `partition` on `as_of` or `period` | 3 years (look-back, Open doubts 6) | 3 years | `partition_drop` | `feat.feature_values`, `feat.segment_stats`, `feat.profiles`; `feat.feature_runs` is not partitioned and leaves by `record_purge` by age on `as_of`, in batches (section 7+) |
| R4 | `age` | 3 years | 3 years | `report_only` | `feat.peer_groups`, `feat.segment_memberships` |
| R5 | `partition` on `created_at` | 7 years from the report date | 10 years | `partition_drop` (section 3.3 guard: no open alert) | `alert.alerts` |
| R5 | `age` | 7 years | 10 years | `report_only` | `alert.alert_index`, `alert.alert_events`, `alert.quarantine`, `scn.detection_keys` (phase 16, `first_seen_at`) |
| R6 | `closure` | 7 years from the later of closure and report | 7 years | `report_only` (DR-191) | `casework.*` (the cases, their children and their seals), `rpt.reports` (the later-of rule), `rpt.filings`, `rpt.report_tasks` |
| R7 | `closure`, `decision`, `age` | 7 years | 7 years (10 under an OFAC profile, Open doubts 9) | `report_only` | `alert.screening_*`, `alert.good_guy_rules`, `alert.pair_memory`, `ref.list_*`, `casework.freezes`, `casework.refrain_instructions`, `rpt.letters` |
| R8 | `owner` | 7 years after the end | 7 years | `record_purge` by party (section 3.5) | the KYC family: `kyc.ratings`, `rating_current`, `rating_feed`, `overrides`, `kyc_reviews`, `review_documents`, `review_queries`, `ownership_inputs`, `ownership_results`, `declared_profiles`, `pep_register`, `pep_relatives` |
| R9 | `retirement`, `age` | kept | none | `kept` | configuration history: `scn.*` (versions, parameter sets, replays, results), `kyc.rating_models`, `ref.*` reference tables, `admin.settings`, `admin.users`, `admin.role_grants` and the other phase 00 to 09 R9 tables |
| R10 | `never` | none | unlimited | never | `audit.*`, `admin.legal_holds`, the phase 23 register tables |
| R11 | `age`, `retirement` | kept | 10 years | `report_only` | `kyc.rating_shadow_runs`, `kyc.rating_shadow_items`; on the `retirement` clock (never due): `admin.models`, `admin.model_versions` (phase 27) and `admin.assistant_versions` (phase 28) |
| R12 | `age` | none | 30 to 90 days | `kept` (`evidence_orphans`); other R12 tables by `maintenance_sweep` (phase 00) | `admin.idempotency_keys`, `admin.jobs`, `admin.outbox`, `admin.timers`, `admin.stepup_assertions`, `rpt.goaml_watch_runs`, `admin.evidence_orphans` |
| R13 | not a table | pgBackRest expiry | about 5 weeks | outside this engine | backups |

A row whose clock is `owner` and whose owner is not in the same run (for example a child whose parent table is `report_only`) is `report_only` too: a child never outlives or leaves its owner by itself.

### 3.3 Partition drops (`admin.retention_drop_partition`, section 1.2)

The drop is the only way a partition leaves the database. The engine calls the function once per due partition, in its own transaction, after the copies of section 3.4 are committed. The function's steps (section 1.2) are: the lock order (holds, then parent), the allow-list, the cut-off, range holds (block), the parent lock and the count, party and case holds (copy required; raw is not copied, step 5), the raw batch check, then `DETACH` and `DROP`. Results and what the engine records:

| Result | Item state | `reason_code` | Effect |
|-|-|-|-|
| `dropped` | `done` | none | the audit event `retention.partition_dropped` (run, parent, partition, `partition_rows`); the manifest was written before the call |
| `not_due` | `skipped` | `not_due` | none (the plan and the call disagree; the plan is recomputed next run) |
| `blocked_hold` | `blocked` | `legal_hold` | a range hold overlaps; audit `retention.partition_blocked` |
| `blocked_copy` | `blocked` | `copy_stale` or `copy_failed` | a party or case hold has no copy of this partition in this run, or the copy's `partition_rows` differs from the count now (a row arrived late); the next run copies again |
| `blocked_unmapped` | `blocked` | `unmapped_batch` | a raw row's batch is not mapped into canon (phase 11) |
| `not_partition` | `failed` | `not_implemented` | the registry names a partition that is not a child of the parent: a defect, audited |
| lock timeout (SQLSTATE 55P03) | `blocked` | `lock_timeout` | retried by the next run |

**The alert partition guard (`alert.alerts`; lead review).** Before the call, the engine checks the partition, in order: (1) a row whose `state` is `queued` or `claimed` (the open states of phase 17): `blocked` with `open_row`; (2) a row named by `casework.case_alerts` (its foreign key `(alert_id, alert_created_at)` is `ON DELETE RESTRICT`, so the `DETACH` would fail, and the case is R6 and kept, DR-191): `blocked` with `referenced`; (3) a row whose disposition instant (`triaged_at` for `triaged`, `updated_at` for `duplicate` and `no_longer_matches`) is after the R5 cut-off: `skipped` with `not_due`. Only a partition that passes all three is dropped, so no alert leaves before seven years from its own disposition, whatever its `created_at`. Alerts of a month are created in that month, so the check cannot race with a new alert in an old partition; an attach to a case after the check waits on the partition lock of the function.

**Companions.** Two parents are dropped as pairs in one run for the same month: `canon.payment_message` with `canon.payment_message_party` (both `received_at`), and `canon.txn` with `canon.txn_event` (both `booking_date`). The second partition of a pair is dropped only when the first one is dropped; a blocked first partition blocks its companion (`reason_code = referenced`).

**Audit partitions are never dropped.** `audit.audit_log` and `audit.access_event` are R10 `never`; the function's allow-list does not contain them and the registry has no handler for them.

### 3.4 Held-row copies (the copy before a drop)

A party or case hold on a partition that is about to be dropped needs a copy of its held rows, made in the same run, before the drop. Where the copy goes: **an evidence object** of phase 02, written with `EvidenceStore::put_generated` (kind `package`, owner kind `legal_hold`, owner id the hold id, retention class `R6`, classification `personal`, sealed with a per-object key). It is not a file and not a database table of rows. The content is one JSON array of the held rows (all columns, as stored) in a ZIP entry `rows.json`; the ZIP is the generated type of phase 02.

**Party paths of the droppable parents (lead review).** `canon.txn` and `canon.balance_snapshot`: `account_id` through `canon.account_party_role` (`party_id`); `canon.card_txn`: `card_id` through `canon.card.account_id` and the roles; `canon.locker_access` and `alert.alerts`: `party_id`; `feat.feature_values` and `feat.profiles`: `entity_id` where `entity` is a party or an account (through the roles); `canon.txn_event`: the events of the held `canon.txn` rows (its companion). `canon.cash_event`, `canon.cbs_user_activity`, `canon.fx_deal`, `canon.payment_message`, `canon.payment_message_party` and `feat.segment_stats` have no party path in this phase: no copy can be made, so their drop is blocked (`copy_failed`) while a party or case hold is active (Open doubts 22). Raw landing tables are not copied (section 3.3, step 5).

Steps for one `(hold, partition)` pair of the run (case holds are resolved to their parties first, section 3.8):
1. Select the held rows of the partition in batches of `retention.batch_rows`, by the party path above. A parent with no party path gets no copy and its drop is blocked.
2. Count the partition's rows (`partition_rows`) and the held rows (`row_count`), write the ZIP as an evidence object with `put_generated` (a clean scan is required; a rejected scan fails the copy with `copy_failed`).
3. One transaction: insert `admin.legal_hold_copies` (`run_id`, `source_table`, `partition_name` schema-qualified, `partition_rows`, `row_count`, `evidence_id`, `sha256` of the ZIP), and audit `legal_hold.copy_written` (hold ref, table, partition, `row_count`, `evidence_id`; no party id).
4. A zero-row copy is written too (its `row_count` is 0); the drop needs the row to exist.

The copy is dropped with its hold: the release effect (section 3.8) sets `hold_released_at` on the hold's copies; the copy's clock is `closure` on that column, so the engine shreds it after `retention.held_copy_grace_days` (section 3.6).

**Dry run:** no copy is written. The item shows `copied = 0`, `state = planned` and `reason_code = copy_stale` when a party or case hold is active on the partition, because a real run would have to copy first; the dry run never reports `blocked` for a missing copy.

### 3.5 Record purge of customer-linked records (R3 and R8; `RelationshipEndResolver`)

**Party end (the relationship-end clock).** A party's relationship ends when all of these hold: no customer row of the party is current (`canon.customer` with `valid_to IS NULL` in its current recorded row, phase 10 bi-temporal rule); every account the party holds through `canon.account_party_role` is closed (`canon.account.status = 'closed'` with `closed_on` set, phase 10). The end date is `greatest(max(canon.customer.valid_to), max(canon.account.closed_on))` over the party's rows. A party with no customer and no account role is not due: its owner is missing and the row is `report_only` with reason `owner_open` (Open doubts 4 and 5). The retention rows of phase 10 name `owner` for the party family and `relationship_end` for `canon.account` (`closed_on`); this phase keeps those rows as they are and resolves the owner's clock here, so the assignment rows do not change.

**Party path.** Every table of the record-purge list has a party path, which the registry names: `canon.party` (its `id`), `party_name`, `party_identifier`, `party_contact` and `customer` (`party_id`), `party_relationship` (both party columns), `account_party_role` (`party_id`), `account` (through its roles: purged when every role points at a purged party), `account_hold` and `card` (each through `account_id`), `facility` and `trade_instrument` (each through `customer_id` and `canon.customer.party_id`), `kyc_document` (`party_id`), `admin.xref` and `admin.attribute_values` (the entity that is the party), and the KYC family by `party_id`. A row without a party path is never purged by this phase.

**Blocking references.** A party is deferred (not purged, item `deferred`, reason `referenced`) while any of these rows names it: `alert.alerts` (`party_id`), `casework.cases` (`party_id`), `casework.case_parties` (`party_id`), `rpt.reports` (`subject_party_id`), `alert.screening_alerts` (`subject_id`, where `subject_kind` is `party`). The list is closed in this phase: a case, an alert, a report or a screening alert is R5, R6 or R7 and is not deleted here, so a party that was ever in a case is kept with it (DR-190, DR-191). A party under a hold is deferred with reason `legal_hold` (section 3.8).

**Batch procedure** (one party is one unit; a batch holds at most `retention.batch_rows` rows of the party tree):
1. Candidates: parties whose end date is on or before the cut-off of R3 or R8 (the longer of the two effective periods applies to a party that has both classes; both are 7 years in the Iraq profile).
2. Per party, in its own transaction: `pg_advisory_xact_lock(hashtext('party:' || party_id))` (section 3.15). Then re-read the holds for the party (party holds on it, and case holds matched as in section 3.8, which follows `merged_into` as 7.4 says) and the blocking references. Any hit: item `deferred` for the party, audit `retention.purge_deferred` with the reason and the count, next party.
3. Delete the tree children first, then the parents (the foreign keys are RESTRICT): KYC family, `kyc_document`, `card`, `facility`, `trade_instrument`, `account_hold`, `account_party_role`, `customer`, `party_relationship`, `party_identifier`, `party_contact`, `party_name`, `admin.xref`, `admin.attribute_values`, accounts whose roles are all gone, and last `canon.party`.
4. Evidence objects owned by a deleted row are shredded in the same transaction (section 3.6).
5. Insert a tombstone (section below) for every deleted `customer` and `account` identity.
6. Manifest: the HMAC of every deleted row id, per table (section 3.7). Audit `retention.records_purged` with the run id, the class, the table and the row count. No id, no name and no value appears in the event.
7. Commit. The next party starts a new transaction; a failure in one party rolls back that party only and the item records `failed` with `reason_code` (never a partial party).

**Tombstones and the CBS.** The CBS still holds a customer whose relationship ended years ago, so a later extract could load it again. Each deleted `canon.customer` and `canon.account` identity leaves `admin.retention_tombstones (schema_name, table_name, source_key_hmac, match_key_version, run_id)`, where the key is `HMAC-SHA256(match key, source_id || '|' || source_table || '|' || source_pk)`. The canonical writers of phase 10 (`akr-ingestd`, section 4.5 item 3) check the tombstone before an insert of the same identity: a hit skips the row and increments the counter `akr_ingest_tombstoned_total` (no row, no audit event per row). A purged identity therefore stays purged until a person removes its tombstone through a change request (Open doubts 12).

**The party path for held copies.** The same party path finds the held rows of a party in a partition for section 3.4. Raw landing tables are not copied (section 3.3).

### 3.6 Evidence: crypto-shredding, byte deletion and lock extension at closure

**Shredding (SP-56).** An evidence object follows its owner (`owner` clock). When the owner row is deleted (section 3.5), or its owner is due for a disposal of its own, the engine calls `EvidenceShredder::shred(tx, evidence_id, reason)` in the owner's transaction. `shred` takes `SELECT ... FOR UPDATE` on the row, erases `wrapped_dek` (the phase 02 guard refuses any other change of the key), sets `shredded_at = now()`, and audits `retention.evidence_shredded` (evidence id, owner kind, run id). A shredded object is unreadable by construction: `open` returns `NotClean` and `Shredded` (phase 02), which reach callers as `E_NOT_FOUND`.

**Byte deletion.** A shredded object whose `lock_until` has passed is deleted from the bucket: `EvidenceShredder::delete_object(evidence_id)` deletes every version the row names (`version_id`) and sets `object_deleted_at = now()` (new column, section 1.2). A COMPLIANCE-locked version cannot be deleted earlier; the phase 02 S3 guard refuses the call, the engine records `blocked` and the object is retried by the next run. Before its lock ends, a shredded object stays in the bucket as ciphertext nobody can open; the certificate states that (section 3.7). Backups keep the wrapped key until they expire (R13, about five weeks), which the certificate also records.

**Scope of shredding in this phase.** Only owner-following evidence is shredded: evidence of KYC reviews, overrides and rating records of a purged party, and held-row copies after their grace period (`hold_released_at` plus `retention.held_copy_grace_days`, `closure` clock of section 3.2). Evidence owned by a case, a report or a pack is not shredded in this phase, because their owners are not purged (DR-191).

**Lock extension at closure (SP-58; decided here, DR-193; M-60 is cited by phase 18's Open doubt 14).** Phase 02 extends a lock only when told to, and the phase 02 guard accepts only a later `lock_until`. Phase 18 closes and seals a case without extending its evidence locks (its Open doubt 14). This phase owns the extension:
1. The case seal transaction of phase 18 enqueues `retention_lock_extend` with payload `{ case_id }` (section 4.5 item 1). The enqueue is part of the seal transaction, so a sealed case always has its extension queued.
2. The job takes `SELECT ... FOR SHARE` on the sealed case row (its `sealed_at` is read), then the evidence set: the objects with owner kind `case` and owner id the case, and the objects named by the case's evidence rows (phase 18 `casework.evidence_items.object_id`).
3. For each object: `lock_until = greatest(lock_until, sealed_at + evidence.lock_days[retention_class] days)` through `EvidenceShredder::extend_lock` (the floor of the phase 02 setting `evidence.lock_days` is 2555 days for R6, so the extension is never shorter than seven years from the seal). Audit `evidence.lock_extended` (evidence id, `lock_until`; no case id).
4. The job is idempotent: a second run computes the same `greatest`. A reopened and re-sealed case extends again, never shortens.

### 3.7 Deletion certificates and the replay after a restore (`CertificateService`, `ReplayService`)

**Batch manifest (written before its deletes).** Each batch of section 3.1 step 5 writes one manifest, a JSON document with fields `v` (1), `run_id`, `as_of`, `retention_class`, `schema`, `table`, `disposal`, `partition` (the partition name, or NULL), `keys` (the lower-case hex HMAC-SHA256 of each deleted row id, or of each shredded evidence id; empty for a partition), `tombstones` (the source identity HMACs of section 3.5), `row_count`, `object_count`, `match_key_version`, `prev` (the SHA-256 of the previous manifest of the same run, or 64 zeros), and `issued_at`. The manifest is signed with the transit key `akr-sign-cert` (section 4.4); the signature and the key version are stored in the manifest object. The object is written to the WORM bucket under `certificates/<run_id>/<retention_class>/<batch_no>.json` with a COMPLIANCE lock until `now() + evidence.lock_days[R10]` (3650 days). The database write of the deletion follows only after the WORM write returns a version id.

**Class certificate.** After the last batch of a class, `CertificateService::issue` writes one row of `admin.deletion_certificates` for `(run_id, retention_class, kind = run)`:
- `scope`: the tables of the class with their disposal and their partition names (names only; never a key);
- `row_count` and `object_count`: sums of the manifests;
- `item_digest`: SHA-256 over the concatenation of the SHA-256 values of the class's manifests in `batch_no` order;
- `signature` and `key_version`: transit signature over the canonical bytes of the certificate document (the same fields as the row, plus `item_digest` in hex), and its key version;
- `worm_object_key` and `worm_version_id`: the certificate document in the WORM bucket under `certificates/<run_id>/<retention_class>/certificate.json`, with the same lock.
Audit `retention.certificate_issued` (certificate id, run id, class, `item_digest`). A class whose run died before its certificate has manifests but no certificate: the next run first issues the missing certificates from the manifests in the bucket (the same `item_digest` rule), before it plans anything.

**Certificates in the database.** The row holds counts and digests; the WORM copy holds the same plus the batch manifests by reference. Neither holds a clear key or a value; a keyed HMAC is not reversible without the match key.

**Replay after a restore.** A restored backup is older than some certificates, so its rows may be present that a certificate has already deleted. The procedure (`akr-ctl retention replay --source worm`) runs before the API is opened:
1. List the certificate documents in the WORM bucket newer than the latest `deletion_certificates.issued_at` of the restored database, in `issued_at` order. Verify each certificate signature (`transit/verify/akr-sign-cert`, the recorded key version) and the signature and `prev` chain of each of its manifests. A broken chain stops the replay with `E_INTERNAL`; nothing is deleted.
2. For each manifest in order:
   - a partition: if the partition exists and its bound is due at the manifest's `as_of` (the same guard of section 1.2 runs again, with the holds that exist now), drop it; a blocked result is reported, not forced;
   - a record set: scan the restored table in batches of `retention.batch_rows`, compute the HMAC of each row id under `match_key_version` from the manifest, and delete the rows whose HMAC is in `keys`, with the same batch transaction and audit event as a real run (`retention.replayed`, counts only). Tombstones named in `tombstones` are inserted;
   - an evidence set: the same scan over `admin.evidence_objects` by the HMAC of the evidence id, with the shred of section 3.6.
3. Write the replay's own class certificates with `kind = replay`, signed and written to the WORM bucket, so a second restore replays them too.
4. Report: certificates read, manifests applied, rows and objects re-deleted, blocked partitions. The API stays closed (section 4.5 item 6) until the report has no `failed` line.

The restored row that a certificate deleted is found by the HMAC of its id, not by its id alone: the replay therefore needs the match key of the manifest's version (the phase 10 match key, read through the `keys/*` policy for `KeyedHash`; the key version is in the manifest). A rotated key that is no longer readable is a replay error (Open doubts 14).

**Verification without changing data.** `akr-ctl retention verify-certificates` lists every certificate and manifest in the bucket, verifies each signature and each `prev` chain, and compares each certificate's `item_digest` with the manifests. Exit code 0 when all verify. It writes nothing to the database.

### 3.8 Legal holds (`LegalHoldService`, `LegalHoldWorkflowSubject`, `ReleaseLegalHoldEffect`; G7)

**Scope shapes.** The `scope` column is the canonical copy of the placement, and the database checks of 0071 enforce its shape. Exactly one of three shapes:
- party: `{"kind": "party", "party_id": "<uuid>"}`;
- case: `{"kind": "case", "case_id": "<uuid>"}`;
- range: `{"kind": "range", "table": "<schema.table>", "from": "YYYY-MM-DD", "to": "YYYY-MM-DD"}`, with `table` one of the parents of section 3.3 or a raw landing table, and `to - from` at most 3660 days.

**Matching (the rule the engine applies at every run, not at placement).** A hold is matched live, so a row that arrives after the placement is held too.
- A range hold holds the rows of `table` whose partition key lies in `[from, to]`; a partition that overlaps the range blocks its drop (section 3.3, step 3). It does not copy rows.
- A party hold holds every row whose party path (section 3.5) resolves to `party_id`.
- A case hold holds the rows of the parties named by `casework.case_parties` for `case_id`, read at the run, and the alerts of the case (`casework.case_alerts` and their `alert.alerts` rows, which carry `party_id`). A party added to the case later is held from the next run.
Holds are never matched by name, by free text or by account number.

**Place (`POST /retention/holds`, scope `report.file`, Cedar `retention.hold.place`, step-up `legal_hold.place`).** Steps:
1. `StepUpService::consume(tx, ctx, header, StepUpAction { name: "legal_hold.place", class: decision }, None)` (the class `decision` is in the closed list of phase 01; the handler calls it before any write).
2. Validate the body: the field combination must match `kind` (`E_VALIDATION`), and `reason_text`, when given, is at most 500 characters (`E_VALIDATION`); a party must exist in `canon.party`, a case in `casework.cases` (`E_NOT_FOUND` when missing); a range must satisfy the rules of the scope shapes above.
3. `reason_text`, when given, is scanned with `PolicyPackService::scan` for the scope `reason_text` (phase 09 section 3.5; a hit is `E_VALIDATION` with the scanner's code in `errors[]`). The text is sealed into `reason_enc` (AAD = the hold id) and is returned in the detail view of M only.
4. Lock order (section 3.15): take `pg_advisory_xact_lock(hashtext('party:' || id))` for each party of the hold (the party itself, or for a case hold the parties of its `case_parties` at this instant), in ascending id order.
5. Insert the row: `hold_ref` from `admin.hold_ref_seq` (`HLD-<yyyy>-<nnnnnn>`), `state = active`, `version = 1`, `placed_by = actor`.
6. Audit `legal_hold.placed` with `{ hold_ref, reason_code }` only. The event carries neither the kind, nor any id, nor the case (a case hold must not announce itself in the audit trail that IA and SEC read).
7. Range holds only: the response lists the disposals already done on that table in the range (from `admin.retention_run_items` with state `done`, partition names and run ids). A party or case hold response carries a fixed note: rows disposed before the placement are not restored by a hold; the recovery is a restore from backup (R13).
8. Response `201` with the hold view of the caller (section 3.8 view rules) and `Location`.
9. The scope is never copied into the audit payload and never into the workflow context.

**Propose the release (`POST /retention/holds/{id}/release`, scope `report.file`, `If-Match` = hold `version`, `Idempotency-Key`).**
1. Lock the hold row (`FOR UPDATE`). `If-Match` missing: `428 E_PRECONDITION_REQUIRED`; not equal to `version`: `409 E_VERSION_CONFLICT` with the current version in the body.
2. State: `released` gives `409 E_NOT_ALLOWED_IN_STATE` with sub-code `hold_released`; `release_requested` gives `409 E_NOT_ALLOWED_IN_STATE` with sub-code `release_pending` (one open release at a time).
3. Body `release_reason` in `no_longer_required`, `entered_in_error`, `superseded`; otherwise `E_VALIDATION`.
4. `WorkflowEngine::start(tx, StartSpec { definition_key: "legal_hold_release", resource: ("legal_hold", id), purpose: Decision, initiator: actor, maker: actor })`. The maker is the proposer (phase 04 four-eyes).
5. Set `state = release_requested`, `release_proposed_by = actor`, `release_instance_id`, `release_reason`, `version + 1`.
6. Audit `legal_hold.release_proposed` with `{ hold_ref, release_reason, instance_id }`. Response: the instance and its open task (the phase 04 view).

**Definition `legal_hold_release`** (new in phase 23; DSL E.4 gains the row): one step `approve`; assignee `{ "kind": "role", "role": "M", "scope": "bank" }`; the holders exclude the placer and the proposer (section 3.8 Holders rule below); timers `remind` and `escalate` only (no automatic outcome, DSL E.2); terminal effect `release_legal_hold` on approve; `on_ended` on reject, withdraw and failure. The step-up of the approval is the engine's step 5 with the declared action `legal_hold.release` (`StepUpAction { name: "legal_hold.release", class: decision }`), consumed once for the act.

**`Holders` rule.** `LegalHoldWorkflowSubject::party_members` returns the M principals with bank scope, minus `placed_by` and minus `release_proposed_by` (the phase 04 `Holders::eligible` runs this rule once for the inbox, the Cedar Task adapter and the engine). With a single M in the bank the release cannot be approved until a second M exists (Open doubts 11).

**`LegalHoldWorkflowSubject`** (phase 04 `WorkflowSubject`, resource kind `legal_hold`, attribute prefix `hold`; attributes `state`, `kind`, `has_case`, `range_table`): `start_context` freezes the attributes and `classification = restricted`, `maker_id = proposer`, neutral titles (no party, no case, no name). `visible_to` and `visibility_sql` are: M at bank scope for every hold, and nobody else (the approval inbox of M shows `hold_ref` and the release reason only). `check_before_decision` (phase 04 hook, step 2 of the engine): the hold row is locked (instance row first, then the hold row, section 3.15); the hold is `release_requested` with `release_instance_id = inst.id`; the actor is not the placer, not the proposer and not acting for another person (`E_FORBIDDEN` with sub-code `delegation_refused`). `on_ended`: `rejected`, `withdrawn` and `failed` call `set_active` (state `active`, `release_instance_id` kept as history) and audit `legal_hold.release_rejected` or `legal_hold.release_ended`.

**`ReleaseLegalHoldEffect`** (terminal effect `release_legal_hold`, resource kind `legal_hold`; runs on approve in the approval transaction, after the engine's step 5):
1. Re-read the hold `FOR UPDATE`; it must still be `release_requested` with this instance (else `409 E_NOT_ALLOWED_IN_STATE`, which rolls the approval back).
2. Set `state = released`, `released_by = actor`, `released_at = now()`, `version + 1`. The 0071 CHECKs refuse `released_by` equal to `placed_by` or `release_proposed_by`, and the 0072 trigger refuses any later change of the state.
3. Set `hold_released_at = now()` on every `admin.legal_hold_copies` row of the hold (section 3.4): the copies become due after the grace period.
4. Audit `legal_hold.released` with `{ hold_ref, release_reason }`.
A released hold is final. A hold placed again on the same party is a new row with a new `hold_ref`.

**View rules (who sees what).** The list (`GET /retention/holds`), the detail (`GET /retention/holds/{id}`), the run items and the report apply the rules of section 3.13 in the view builder. In short: M sees everything, including the kind, the party or case id and the sealed reason text (in the detail view only); IA and SEC (`audit.read`) see range holds in full, party holds as `kind: party` with `scope: "restricted"`, and never a case hold (omitted from lists and counts; `E_NOT_FOUND` on the detail, the same body as a missing id).

**Cedar.** The hold actions of file `61_retention.cedar` (section 3.14) are checked before the view is built. A Cedar forbid on a case hold for a non-M principal is the deny of the `E_NOT_FOUND` rule (section 3.11): the handler answers as if the hold did not exist.

### 3.9 Evidence reconciliation (job `evidence_reconcile`; `EvidenceReconciler`)

Orphans are objects in the evidence bucket that no `admin.evidence_objects` row names. Their sources are the crash windows that phase 02 counts as `akr_evidence_orphans_total` (a second final object of `mark_clean`; a `put_generated` that wrote the object and rolled back), the snapshot orphans of phase 17 (its step 11) and the package orphans of phase 21 (its step 8). The reconciler does not need to know the source: it compares the bucket with the table.

Steps of one run (daily, `schedule.evidence_reconcile`, 04:10):
1. For each retention class that evidence uses (R3 to R9), for each month of the last 400 days, list the object versions under `ev/<class>/<yyyy>/<mm>/` (`ListObjectVersions`, pages of 1000). Certificates and their manifests are plain WORM objects under `certificates/` and are outside this listing. Stop at `retention.max_run_minutes`; the next day continues from the start of the list (the job is idempotent).
2. For each version, check `admin.evidence_objects` by `(object_key, version_id)`. No row: insert into `admin.evidence_orphans` (`found_by = reconcile`, `lock_until` from the object lock, `size_bytes`, `retention_class` parsed from the key) with `ON CONFLICT DO NOTHING`. A row that exists and is shredded is not an orphan.
3. Each orphan whose `lock_until` has passed (or that has no lock) is deleted version by version, `deleted_at = now()`. An orphan under a live COMPLIANCE lock is kept and counted; its bytes are unreadable (the key was never committed, phase 02), so there is no disclosure while it waits.
4. Audit `evidence.orphan_found` once per run with the count per class (no key); `evidence.orphan_deleted` per deleted orphan with its id.
5. Gauges: `akr_evidence_orphans_open` (orphans not deleted) and `akr_evidence_orphans_total` (phase 02 counter, unchanged). Phase 02's counter is the alarm; the reconciliation is the listing.

Pending staging objects are not orphans: they are in `akr-quarantine` and `maintenance_sweep` (phase 02) removes them after `evidence.staging_ttl_h`.

### 3.10 The retention report, the periods and the schedules

**Where the per-class periods live.** In `admin.settings`, one key per class (section 1.2): `retention.period_days.r1` and `retention.period_years.r2` to `r8`, governed, each with the Iraq floor as `floor.min`. The effective period is `max(configured, floor.min)` (section 3.2). A bank may lengthen a period: a change is a change request (phase 04 `config.author` drafts, route `cr_material`, MLRO and CHK in parallel, CH too when the change would shorten; the volume effect of a lengthened period is `neutral`: it changes no alert and no threshold). A value below its floor is refused with `E_BELOW_FLOOR` (phase 09 settings rule, 422) at the change request, and the engine applies the same maximum again at run time, so a value that bypassed the request cannot shorten a period. The schedules (`schedule.retention_run`, `schedule.evidence_reconcile`) and the batch limits are not governed (`floor` NULL, `governed` FALSE): they change the cost of a run, never its outcome.

**The report (`GET /retention/report`, scope `audit.read` or `admin.system`).** Returns the latest completed run (dry or real), not a live count: `as_of`, the run id, `dry_run`, and per class: `eligible`, `deleted` (zero for a dry run), `deferred`, `blocked` and `held`. `held`, `deferred` and the reason codes are `null` for IA, SEC and ADM (section 3.13). For `rpt.reports` the summary also gives the number of reports due under the later-of rule (section 3.2) and the earliest due date, without ids. With no completed run the response is `200` with `run: null`. A report is therefore never older than the last night's plan, and a request never runs the plan.

**Dry run (`POST /retention/dry-runs`, scope `admin.system`).** Section 3.1. It returns `202` with the run id; the result is read with `GET /retention/runs/{id}`.

### 3.11 Errors (`RetentionError`, `thiserror`; API section 6 codes only)

| Error | Code | When |
|-|-|-|
| `Missing` | `E_NOT_FOUND` (404) | the hold does not exist, or a case hold is read by a principal other than M (identical body) |
| `Denied` | `E_FORBIDDEN` (403) | Cedar denies on a resource the caller may know exists |
| `Scope` | `E_SCOPE` (403) | the session or token lacks the scope (checked before the handler) |
| `StepUp` | `E_STEPUP_REQUIRED` (401) | placement or approval without a valid assertion of its class |
| `SelfApproval` | `E_SELF_APPROVAL` (403) | the placer or the proposer tries to approve the release |
| `Invalid` | `E_VALIDATION` (422) | shape, range, reason scan (`errors[]`) |
| `BelowFloor` | `E_BELOW_FLOOR` (422) | a period below its floor, at the change request |
| `State` | `E_NOT_ALLOWED_IN_STATE` (409) | sub-codes `hold_released`, `release_pending`, `not_release_task`, `hold_changed` |
| `Version` | `E_VERSION_CONFLICT` (409) | `If-Match` differs from `version` |
| `Precondition` | `E_PRECONDITION_REQUIRED` (428) | release without `If-Match` |
| `Claimed` | `E_CLAIMED` (409) | another retention run holds the advisory lock |
| `Idempotency` | `E_IDEMPOTENCY_REQUIRED` (400), `E_IDEMPOTENCY_REUSE` (422), `E_IDEMPOTENCY_IN_FLIGHT` (409) | the phase 00 rules |
| `Unavailable` | `E_UNAVAILABLE` (503) | OpenBao, WORM bucket or S3 unreachable; the run records it and deletes nothing of that class |
| `Internal` | `E_INTERNAL` (500) | a certificate chain or digest mismatch, a copy that fails after its retries; correlation id only |

### 3.12 Audit events (one per mutation; payloads carry ids, counts, codes and hashes; never a name, a value, a reason text or a case id in an event that IA or SEC can read)

| Event | On | Resource | Payload |
|-|-|-|-|
| `legal_hold.placed` | place | `legal_hold` (hold ref) | `hold_ref`, `reason_code` |
| `legal_hold.release_proposed` | propose | `legal_hold` | `hold_ref`, `release_reason`, `instance_id` |
| `legal_hold.released` | release effect | `legal_hold` | `hold_ref`, `release_reason` |
| `legal_hold.release_rejected` | `on_ended` rejected | `legal_hold` | `hold_ref`, `instance_id` |
| `legal_hold.release_ended` | `on_ended` withdrawn or failed | `legal_hold` | `hold_ref`, `instance_id`, end kind |
| `legal_hold.copy_written` | copy (section 3.4) | `legal_hold` | `hold_ref`, `source_table`, `partition`, `row_count`, `evidence_id` |
| `retention.run_started` | start | `retention_run` | `run_id`, `trigger`, `dry_run`, `as_of` |
| `retention.run_finished` | end | `retention_run` | `run_id`, `state`, `dry_run`, totals by class |
| `retention.partition_dropped` | drop | partition name | `run_id`, `parent`, `partition`, `partition_rows` |
| `retention.partition_blocked` | blocked drop | partition name | `run_id`, `parent`, `partition`, `reason_code` |
| `retention.records_purged` | purge batch | table | `run_id`, `retention_class`, `table`, `rows` |
| `retention.purge_deferred` | deferral | table | `run_id`, `table`, `reason_code`, `count` (no party id) |
| `retention.evidence_shredded` | shred | `evidence_object` | `evidence_id`, `owner_kind`, `run_id` |
| `retention.object_deleted` | byte deletion | `evidence_object` | `evidence_id`, `version_id` |
| `retention.certificate_issued` | class certificate | `deletion_certificate` | `certificate_id`, `run_id`, `retention_class`, `kind`, `item_digest` |
| `retention.replayed` | replay batch | `deletion_certificate` | `source_certificate_id`, `rows`, `objects` |
| `retention.floor_applied` | run start, per class | setting key | `key`, `configured`, `floor` |
| `evidence.lock_extended` | lock job | `evidence_object` | `evidence_id`, `lock_until` |
| `evidence.orphan_found` | reconcile, once per run | `evidence_orphan` | `count` per class |
| `evidence.orphan_deleted` | reconcile | `evidence_orphan` | `evidence_orphan_id` |

Every mutation of this phase appears above; the registry (section 3.2) names the tables that the purge events cover. The chain of phase 02 carries each event, and the anchors of phase 02 sign them.

### 3.13 Tipping-off review (what a person outside the case access list can learn; the answer must be "nothing")

- **Hold list and detail (IA and SEC, `audit.read`).** They learn that a range hold exists (its table and dates: no case, no party). They learn that a party hold exists on some party, as `kind: party` without the party id. They learn nothing about a case hold: it is omitted from the list and from every count, and the detail answers `E_NOT_FOUND`. The hold list header ("N active holds") counts only the holds they can see.
- **Placement audit event.** It carries the hold ref and the reason code. No kind, no id, no case (so an auditor reading the chain learns neither a case nor a party from `legal_hold.placed`).
- **Release workflow (M only).** The inbox of the approver shows the hold ref and the release reason; the case is not named.
- **Run items and the report.** `held`, `deferred` and the reason code `legal_hold` are `null` for IA, SEC and ADM. The counts per table are visible to them (they show a party's purge only as a number of rows in a table, with no party id).
- **Certificates (IA and SEC).** Counts, table names, disposal names and digests. Never an id, never a key, never a value.
- **Evidence reconciliation.** No output outside the audit trail (counts and ids of orphans, which are not case objects: an orphan has no owner row).
- **Ingest tombstones.** Silent: a skipped row produces a counter, not an event per row.
- **Customers.** No notice is sent by this phase. The retention run never writes to a channel that a customer reads.

### 3.14 Visibility and Cedar (the file `61_retention.cedar`; phase 03 bundle)

The file is in the audit range of phase 03 (60 is the audit file of phase 24, `60_audit.cedar`; 61 is the next free name in that range, Open doubts 23). Scope gates are API.md section 2 (`report.file`, `audit.read`, `admin.system`); these policies are the fine gate. Step-up is never a condition (the handler consumes it). The approval of the release is phase 04's `task.approve` on the shared Task resource (`56_workflow.cedar`), with eligibility from `Holders` (section 3.8).

Schema additions (the phase 03 form; `User`, `Ctx` and `System` are phase 03's):

```cedar
entity LegalHold = { kind: String, state: String, has_case: Bool, placed_by?: User };
entity RetentionRun = { dry_run: Bool, state: String };
entity DeletionCertificate = { retention_class: String, kind: String };
action "retention.hold.read", "retention.hold.release" appliesTo { principal: User, resource: LegalHold, context: Ctx };
action "retention.hold.place", "retention.report.read", "retention.run.dry", "retention.run.read",
       "retention.certificate.read" appliesTo { principal: User, resource: System, context: Ctx };
```

The policies (one role per `@id`; the `@role` tag names the role):

```cedar
// 61_retention.cedar (phase 23). Copy-paste ready. Scopes are API.md section 2; these are the fine gates.

@id("retention.m") @role("M")
permit (principal, action in [Akr::Action::"retention.hold.read", Akr::Action::"retention.hold.release",
        Akr::Action::"retention.hold.place", Akr::Action::"retention.report.read", Akr::Action::"retention.run.read"], resource)
when { principal.roles.contains("M") };

@id("retention.ia") @role("IA")
permit (principal, action in [Akr::Action::"retention.hold.read", Akr::Action::"retention.report.read",
        Akr::Action::"retention.run.read", Akr::Action::"retention.certificate.read"], resource)
when { principal.roles.contains("IA") };

@id("retention.sec") @role("SEC")
permit (principal, action in [Akr::Action::"retention.hold.read", Akr::Action::"retention.report.read",
        Akr::Action::"retention.run.read", Akr::Action::"retention.certificate.read"], resource)
when { principal.roles.contains("SEC") };

@id("retention.adm") @role("ADM")
permit (principal, action in [Akr::Action::"retention.run.dry", Akr::Action::"retention.run.read",
        Akr::Action::"retention.report.read"], resource)
when { principal.roles.contains("ADM") };

@id("retention.case_hold_hidden") @deny("tipping_off")
forbid (principal, action == Akr::Action::"retention.hold.read", resource)
when { resource.kind == "case" && !(principal.roles.contains("M")) };

@id("retention.release_not_placer") @deny("self_approval")
forbid (principal, action == Akr::Action::"retention.hold.release", resource)
when { resource has placed_by && resource.placed_by == principal };
```

The last forbid reads `placed_by`, which the handler supplies on the `LegalHold` entity (optional, so a hold without a placer in the request context is not an error). The case forbid denies the read for IA and SEC, and the handler answers `E_NOT_FOUND` (section 3.11).

### 3.15 Lock order (phase 18 section 3.16 extended; DR-193)

Every path that takes two of these locks takes them in the order below:
1. **Party advisory locks** (`pg_advisory_xact_lock(hashtext('party:' || id))`, ascending ids): taken by hold placement (section 3.8 step 4) and by the party purge (section 3.5 step 2). Neither takes any other lock of this list before them.
2. **`admin.legal_holds` table lock** (`SHARE ROW EXCLUSIVE`) then the **partition lock** (`SHARE`) and, for the `DETACH` only, the parent (lock_timeout 2 s): taken by `admin.retention_drop_partition` (section 1.2) only. A hold placement inserts into `legal_holds` (ROW EXCLUSIVE) and waits while a drop runs; a drop never waits for a party lock.
3. **Workflow instance row, then hold row** (`FOR UPDATE`): the approval and the release effect (section 3.8), as phase 04 and phase 20 order them.
4. **Case row (`FOR SHARE`), then the evidence rows (`FOR UPDATE`)**: the lock extension of section 3.6 and the shred of a case object (none in this phase).
5. **Owner row, then evidence row**: the shred of section 3.6 runs in the owner's transaction after the owner's lock.

The purge of a party (step 1) and a hold placement on the same party cannot interleave: whichever takes the party lock first decides, and the other re-reads the holds after its lock. A drop (step 2) and a placement cannot interleave either: the placement waits on the table lock, and the hold check of the drop sees every committed hold.

---

## 4. Infrastructure Updates

### 4.1 Job kinds (Postgres `jobs`; coalesce key in brackets)

| Kind | Binary | Payload | Concurrency, attempts | Schedule or trigger |
|-|-|-|-|-|
| `retention_run` | worker | `{ run_id }` [`retention_run:<run_id>`] | 1, 1 (a failed run is re-planned by the next night, never retried in place) | `schedule.retention_run` (nightly, real run, trigger `schedule`); `POST /retention/dry-runs` (dry run, trigger `dry_run_request`); `akr-ctl retention run --now` and `akr-ctl retention replay` (section 4.5 item 9) |
| `retention_lock_extend` | worker | `{ case_id }` [`retention_lock_extend:<case_id>`] | 2, 5 (back-off) | enqueued by the case seal transaction of phase 18 (section 4.5 item 1); section 3.6 |
| `evidence_reconcile` | worker | `{}` [`evidence_reconcile`] | 1, 2 | `schedule.evidence_reconcile` (daily, 04:10); section 3.9 |

Every handler re-reads its owner first. The hold placement and the release are request-transaction acts in akr-api (section 3.8), not jobs. The retention run runs on the retention pool (section 4.3). No new job prefix is used (`ingest_` is for akr-ingestd only).

### 4.2 Timers and outbox

No new timer kind and no timer row: the approval reminders of the `legal_hold_release` definition are phase 04's `remind` and `escalate` timers. No new outbox topic: the phase 23 events are audit events only (section 3.12).

### 4.3 AppState additions

- `akr-api`: `legal_holds: Arc<LegalHoldService>`, `retention_reports: Arc<RetentionReportService>`. The hold workflow subject and the release effect are registered in the phase 04 registries at start-up.
- `akr-worker`: `retention: Arc<RetentionEngine>`, `evidence_reconcile: Arc<EvidenceReconciler>`, `lock_extension: Arc<LockExtensionService>`, `certificates: Arc<CertificateService>`, and `retention_pool`: a second Postgres pool with the LOGIN role `akr_retention_login` (member of `akr_retention`; at most 2 connections; default `statement_timeout = 30s`; a drop transaction sets `retention.drop_timeout_s`). Only the retention handlers take a connection from this pool.
- `akr-ctl`: `ReplayService`, the `retention` subcommands.

### 4.4 Keys, network zones, metrics, readiness and OpenAPI

- **Transit key `akr-sign-cert`** (Ed25519, created with the other transit keys of the OpenBao layout by the ceremony runbook, and by `make bao-dev` in development (phase 00 section 7+); rotation yearly). It is a new key and not the anchor key `akr-sign-anchor` (DR-194). `akr-worker` policy: `transit/sign/akr-sign-cert`, `transit/verify/akr-sign-cert`, read on `transit/keys/akr-sign-cert` (public keys). `akr-api` gets none of them.
- **Match key**: `keys/*` policy of phase 10 (`KeyedHash`), read by akr-worker for the HMACs; versions are kept while any manifest names them (Open doubts 14).
- **Database**: the LOGIN role `akr_retention_login` with its password in OpenBao KV `db/akr_retention_login` (provisioning, section 4.5 item 4).
- **S3**: the worker's identity gains `s3:ListBucketVersions` and `s3:DeleteObjectVersion` on `akr-evidence` under `ev/` only, and `s3:PutObject` on `akr-evidence` under `certificates/`. No governance-bypass permission is granted: the COMPLIANCE lock refuses an early delete, and the phase 02 S3 guard refuses the call before it.
- **Network zones**: akr-worker reaches Postgres, OpenBao and S3 as before; akr-ingestd reads `admin.retention_tombstones` on its existing database connection. No new zone rule.
- **Metrics**: `akr_retention_runs_total{dry_run,state}`, `akr_retention_run_seconds` (histogram), `akr_retention_rows_deleted_total{retention_class}`, `akr_retention_blocked_total{reason_code}`, `akr_evidence_orphans_open` (gauge), `akr_ingest_tombstoned_total` (counter, section 3.5).
- **Readiness**: the akr-api readiness check reports `E_UNAVAILABLE` (503) while a restore replay is pending (section 3.7; the gate is Open doubts 15).
- **OpenAPI**: tag `retention`; the nine routes of section 2; schemas of section 2. `make generate-api` regenerates the client in the same change.

### 4.5 Existing entity, schema, route and code updates (extensions of earlier phases)

1. **Phase 18, case seal.** The closing sequence's seal transaction (phase 18 section 3, the `case.sealed` event) enqueues `retention_lock_extend` with `{ case_id }` in the same transaction. No route, no state and no event of phase 18 changes; this closes phase 18 Open doubt 14 (DR-193).
2. **Phase 04, definitions and registries.** DSL E.4 gains the definition `legal_hold_release` (section 3.8); the `EffectRegistry` gains `release_legal_hold`; the `SubjectRegistry` gains `LegalHoldWorkflowSubject` (resource kind `legal_hold`); `Holders` gains the rule of section 3.8 for that kind (M at bank scope, minus the placer and the proposer).
3. **Phase 10 and phase 11 canonical writers (akr-ingestd).** Before inserting a `canon.customer` or `canon.account` identity, the writer checks `admin.retention_tombstones` for the identity's HMAC (section 3.5). A hit skips the row and increments `akr_ingest_tombstoned_total`. This is a read-only addition to the writers; the CBS connection and its conformance suite are unchanged.
4. **Phase 00, roles and provisioning.** `akr_retention` (NOLOGIN, 0072) and the LOGIN role `akr_retention_login` created by `akr-ctl db provision`, a member of `akr_retention`, with the audit writer membership of section 1.2 and the password in OpenBao. The grant list of akr_app in section 1.2 is added to the provisioning check.
5. **Phase 02, `admin.evidence_objects`.** One column added (`object_deleted_at`, 0072) and its classification row. The guard trigger and the grants of phase 02 are unchanged except the retention role's update grant (section 1.2).
6. **Restore runbook (phase 00 operations).** After a restore: `akr-ctl retention verify-certificates`, then `akr-ctl retention replay`, then the API opens (section 3.7). The order is a runbook step, enforced by the readiness gate of section 4.4.
7. **Phase 03, policy bundle.** `61_retention.cedar` is added to the bundle in the manifest order of phase 03; the bundle hash changes and the release notes carry it. The `StepUpAction` constants `legal_hold.place` and `legal_hold.release` (class `decision`) are declared in the phase 23 code; the phase 01 closed list is unchanged.
8. **Phase 09 settings.** The setting rows of section 1.2 are inserted by 0072; the phase 09 floor rule refuses a governed value below its floor with `E_BELOW_FLOOR`.
9. **akr-ctl.** `akr-ctl retention run --now` (operator, on the host: inserts a run with trigger `schedule` and enqueues the job, for the first run and for the test suite); `akr-ctl retention replay --source worm`; `akr-ctl retention verify-certificates`; `akr-ctl evidence reconcile --now`. None of them deletes without a manifest.
10. **Documents (reconciliation pass, not in this change).** `docs/API.md` section 3.4 gains the three added rows (`GET /retention/holds/{id}`, `GET /retention/runs`, `GET /retention/runs/{id}`) and the release row's approval note; `docs/DATA-MODEL.md` section 18 gains the seven tables and their column changes (section 1); `docs/DSL.md` E.4 gains `legal_hold_release`; `docs/plan/roadmap.md` row 23 lists dependencies 18, 20 and 21 (Open doubts 21); `docs/plan/research.md` gains DR-184 to DR-200.

## 5. Frontend Impact

### 5.1 G7 Retention and legal holds (`/retention`; REGISTER; catalogue row G7 of DESIGN-SYSTEM section 8.1; P1)

A register with four tabs. The register pattern is DESIGN-SYSTEM section 8 "REGISTER" (a read-mostly table with filters and an export through an SU job).
- **Holds** (default tab; `GET /retention/holds`). Filters: kind, state. Columns: `hold_ref`, kind (party, case or range as the viewer sees it), reason code label, state, `placed_at`. M sees the party or case id, the range and, in the detail drawer only, the reason text (fetched with the detail, never in the list). IA and SEC see the range holds and the restricted party holds; case holds are not in the table. Actions for M: "Place hold" (step-up dialog, `POST /retention/holds`) and "Propose release" (`POST /retention/holds/{id}/release` with `If-Match`); the approval is in the inbox (phase 04) with the step-up of the act.
- **Runs** (`GET /retention/runs`, `GET /retention/runs/{id}`). A list with the dry-run flag; ADM sees "Run dry simulation" (`POST /retention/dry-runs`, `Idempotency-Key`). The detail shows the items by class and table; the `held`, `deferred` and reason columns are shown to M only.
- **Due** (`GET /retention/report`). The report by class: eligible, deleted (zero on a dry run), blocked, and held (M only). A line for "reports due under the later-of rule" (count and earliest date, no ids). The screen says in words that the figures come from the last night's plan, with its `as_of` date.
- **Certificates** (`GET /retention/certificates`; IA and SEC). A table of certificate rows: run, class, kind, counts, `item_digest` (shortened), signature key version, issued time. No detail beyond the row. "Download" is not offered in this phase.

Layout: a single column at phone width with a 16px gutter; the tab bar scrolls horizontally inside its own container, never the page. Arabic labels (RTL) for every column; reason codes use the labels of their translation table, not the code.

### 5.2 Generated hooks (Orval names from the paths)

`useGetRetentionHolds`, `usePostRetentionHolds`, `useGetRetentionHoldsId`, `usePostRetentionHoldsIdRelease`, `useGetRetentionReport`, `usePostRetentionDryRuns`, `useGetRetentionRuns`, `useGetRetentionRunsId`, `useGetRetentionCertificates`, and the phase 04 hook `usePostTasksIdActionsApprove` used by the approval inbox.

### 5.3 Browser and caching rules

No hold, run, certificate or report is kept in the browser: no local storage, no service-worker cache. A party name is never shown in this phase (the screen shows ids to M and nothing to others). The reason text is fetched only on the detail request of M and is not cached by the client beyond the open drawer. Every POST carries an `Idempotency-Key`; the place and release forms keep their draft in memory only.

## 6. Verification

1. `make migrate`: `0071_retention_holds_runs` and `0072_retention_guards_grants` apply cleanly; a second run is a no-op. `\d admin.legal_holds` shows the three kind CHECKs, `released_by <> placed_by` and `released_by <> release_proposed_by`.
2. `cargo check`: compile-time SQL passes for the repositories of section 2.
3. `cargo clippy -- -D warnings`: no lint errors.
4. `cargo test`:
   - 4a. Registry coverage: a test reads `admin.retention_assignment` and fails unless every row has exactly one `DisposalHandler` and every handler has a row; a row with clock `never` is R10; a row with clock `owner` names an existing parent row.
   - 4b. Effective periods: `retention.period_years.r2` configured at 5 with floor 7 gives 7; configured at 10 gives 10; R1 at 20 days gives 30.
   - 4c. Partition bound parsing: the upper bound of a monthly partition `canon.txn_2026_10` is `2026-11-01`; a daily raw partition is `2026-10-10`.
   - 4d. HMAC identities: the same id under the same key version gives the same digest; a different match key version gives a different one.
   - 4e. The manifest chain: `prev` of batch 2 is the SHA-256 of batch 1; a changed byte breaks verification.
5. curl suite `tests/curl/phase23_retention.sh` (dev users `adm@`, `m@`, `m2@`, `ia@`, `sec@`, `cfg@`, `ds@`; `m2@bank.test` is a dev fixture with role M, added to the dev seed by this phase). Each check prints the expected code.
   - 5a. Dry run: `POST /api/v1/retention/dry-runs` as `adm@` with `Idempotency-Key`: `202` with a run id; `GET /api/v1/retention/runs/{id}` until `state = completed`; every item has `deleted = 0`, `copied = 0`, `objects_shredded = 0`; `psql`: the partition count of `canon.txn` and the fixture row counts are unchanged; no `retention.partition_dropped` or `retention.records_purged` event carries the run id.
   - 5b. `403 E_SCOPE`: `m@` on `POST /api/v1/retention/dry-runs` (no `admin.system`); `ia@` on `POST /api/v1/retention/holds` (no `report.file`); `m@` on `GET /api/v1/retention/certificates` (no `audit.read`).
   - 5c. Cedar deny with the scope present: `ia@` on `GET /api/v1/retention/holds/{id}` for the fixture case hold: `404 E_NOT_FOUND`, the body byte-identical to a random uuid's.
   - 5d. Masking in lists: `ia@` on `GET /api/v1/retention/holds` does not contain the case hold's `hold_ref`; `m@` does. `ia@` on `GET /api/v1/retention/runs/{id}` shows `held: null` and `deferred: null`.
   - 5e. Step-up: `m@` on `POST /api/v1/retention/holds` without an assertion: `401 E_STEPUP_REQUIRED`.
   - 5f. Idempotency: the same request twice with one `Idempotency-Key` gives one row (same `hold_ref`); the same key with a different body gives `422 E_IDEMPOTENCY_REUSE`; no key gives `400 E_IDEMPOTENCY_REQUIRED`.
   - 5g. Release, the four-eyes path: `POST /api/v1/retention/holds/{id}/release` without `If-Match`: `428 E_PRECONDITION_REQUIRED`; with a stale one: `409 E_VERSION_CONFLICT`; with the current one: `201` and an instance id. `m@` approves its own task on `POST /api/v1/tasks/{id}/actions/approve`: `403 E_SELF_APPROVAL`. `m2@` approves with an assertion of class `decision`: `200`; the hold is `released`, `released_by = m2`; `psql`: `hold_released_at` is set on the hold's copies.
   - 5h. Reason scan: `reason_text` with a forbidden term: `422 E_VALIDATION` with `errors[]`.
   - 5i. Floor: a change request by `cfg@` setting `retention.period_years.r2` to 5 is refused: `422 E_BELOW_FLOOR`.
   - 5j. A range hold blocks a drop: place a range hold on the dev fixture partition `canon.txn_2019_01`, then `akr-ctl retention run --now`; the item is `blocked` with `legal_hold`, and `to_regclass('canon.txn_2019_01')` still exists.
   - 5k. A party hold makes a copy: place a party hold on a fixture party with rows in the fixture partition `canon.txn_2019_02`; `akr-ctl retention run --now`; one `admin.legal_hold_copies` row exists with `partition_rows` equal to the partition's count and an evidence object of kind `package` owned by `legal_hold`; the partition is then dropped only if that copy matches (the fixture is built to match), and the drop is audited with `partition_rows`.
   - 5l. Copy race: insert one row into the fixture partition after the copy and before the run's drop (a test hook pauses the run); the item is `blocked` with `copy_stale`.
   - 5m. Certificates: `akr-ctl retention verify-certificates` exits `0`; `ia@` on `GET /api/v1/retention/certificates` sees the class rows with counts and digests and no key-shaped field.
   - 5n. Replay: restore the dump taken before 5k into a scratch database, run `akr-ctl retention replay --source worm`; the partition dropped in 5k is dropped again, and the replay certificate row exists with `kind = replay`.
   - 5o. Tombstone: a purged fixture customer is re-extracted by the ingest fixture; no `canon.customer` row is written and `akr_ingest_tombstoned_total` increments.
   - 5p. Audit payloads: `psql` on `audit.audit_log` for the session: `legal_hold.placed` payloads have keys `hold_ref` and `reason_code` only; no payload of this phase holds a key named `party_id`, `case_id`, `kind` or `reason_text`.
   - 5r. Alert guard: a fixture alert partition past the cut-off with one alert attached to a fixture case gives `blocked` with `referenced`; the same partition without the attachment and with one `triaged_at` after the cut-off gives `skipped` with `not_due`; neither partition is dropped.
   - 5q. The sealed case's lock extension: seal a fixture case; `retention_lock_extend` runs; each evidence object's `lock_until` is at least `sealed_at + 2555 days`, and a second run changes nothing.
6. Swagger UI shows the nine `/retention` routes with their scopes; `make generate-api` regenerates the client and the hooks of section 5.2 exist.
7. The audit trail for the curl session holds the events of section 3.12 that the session triggers (placed, release proposed, released, copy written, run started and finished, partition dropped or blocked, certificate issued), each with the payload keys of its row.
8. The policy bundle: `akr-ctl policy validate` accepts `61_retention.cedar` and prints the new bundle hash; the Cedar allow and deny table of section 3.14 passes for every role listed.
9. Existing tests still pass. No CBS connection change: the read-only conformance suite of phase 05 and phase 10 runs unchanged (the tombstone read of check 5o is a read in akr-ingestd on the AKR database only).

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 1: corrections found while checking phases 00 to 02 (tools/agent/gap1-run1-verdict.md); lead-verified.

### 7.1 Deletion-certificate key name
The transit key `akr-sign-cert` of this phase is named `akr-sign-deletion` (phase 00 7.6, DR-357), so that it is not
confused with the coverage-certificate key `akr-sign-certificate`. Every mention (sections 2, 3 and 4.4, DR-194) and
the `akr-worker` policy paths (`transit/sign/akr-sign-deletion`, `transit/verify/akr-sign-deletion`, read on
`transit/keys/akr-sign-deletion`) use the new name. Nothing else changes: it stays a separate key from the anchor key.

Gap pass 1 (2026-10-10), run 4a, from tools/agent/gap1-run4a.md; lead-verified (tools/agent/gap1-run4a-verdict.md).

### 7.2 Handlers for the tables of later phases (DR-387)
Each later phase that inserts `admin.retention_assignment` rows adds, in the same release, one handler per table to the
registry of section 3.2, chosen by class and clock: R3 and R8 `owner` rows go in the party batch of section 3.5; R6,
R7 and R11 are `report_only` (DR-191), and so is any child of a `report_only` owner; R9 is `kept`; R10 is never
deleted; a `retirement` clock is never due. An R12 `age` row is deleted by the owner's named sweep (phase 22
`notification_sweep`, phase 29 `assess_ledger_sweep`) or, where the owner names none, by this engine's `record_purge`
by age in batches. A `closure` clock on the table's own column (for example `closed_at`, `ended_at`, `resolved_at`)
treats the row as its own owner: due when the column is set and on or before the cut-off. Check 4a covers these rows.
Two drops gain a step: from phase 27 on, the `alert.model_scores` rows of an `alert.alerts` partition are deleted
(`record_purge`, R5) before the partition is dropped; from phase 29 on, an `assess.decisions` partition is dropped only
when no `assess.holds` row references it.

### 7.3 DELETE grants follow the handlers (DR-386)
`akr_retention` holds DELETE on exactly the tables whose handler deletes rows (`record_purge`, including the later
tables of the previous block), and none on a table whose handler is `report_only`, `kept`, `retirement` or never.
Partitions go through `admin.retention_drop_partition`. Check 4a also fails when a deleting handler lacks the grant or a
grant has no deleting handler. DR-198's "party tree and KYC family only" reads "the deleting handlers only".

Adversarial pass (2026-10-10), run 4 (clocks and timers on closed, merged or reopened items), from tools/agent/adv-run4.md; lead-verified (tools/agent/adv-run4-verdict.md).

### 7.4 A case hold follows a merge (ADV-4-07, HIGH; DR-410)
A merge moves the alerts of the source case to the target (phase 17 section 3.6), so a hold read from the source alone
would lose them at the next run. The matching of a case hold (section 3.8) therefore follows `merged_into` from the held
case to its end and holds, at the run, the parties of `casework.case_parties` and the alerts of `casework.case_alerts` of
every case on that path. A hold may cover more than the original file; it never covers less. Test: hold case S, merge S
into T: the next run still holds S's alerts in T.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.5 The canonical writer takes the party lock (V-10-06-a, MEDIUM; DR-413)
Section 3.15 step 1 gains a third taker of the party lock, beside hold placement and the party purge: the canonical writer of phase 10 (phase 10 section 7+, the party lock in the writer), before its tombstone read of a `canon.customer` or `canon.account` row, in ascending party id, held to the commit of its batch transaction.

### 7.6 Counterparty links follow their counterparty (V-13-12-a, MEDIUM; DR-413)
Section 3.2: `feat.counterparty_links` (phase 13 section 7.5) is an R2 row with clock `owner`. Its handler's `parent_key` is `canon.counterparty` by `counterparty_id`, so the table is `report_only` while its parent is (the R2 row of section 3.2, Open doubts 22), and its rows are deleted in the counterparty's transaction once that purge opens.

### 7.7 Two audit events of the run (V-23-01-d, V-23-01-e, MEDIUM; DR-413)
Section 3.12 gains two rows. `retention.dry_run_requested` | `POST /retention/dry-runs` (section 3.1), in the transaction that inserts the run row | `retention_run` | `run_id`, `as_of` (the actor is the requester). `retention.item_failed` | a failed item (section 3.1 step 8) | table | `run_id`, `retention_class`, `table`, `reason_code` (a partition item adds `partition`; no party id and no case id). Both names are new.

### 7.8 A repeat shred is a no-op (V-23-03-a, MEDIUM; DR-413)
`ShredOutcome { Shredded, AlreadyShredded }`. `EvidenceShredder::shred` on a row whose `shredded_at` is set returns `AlreadyShredded`, keeps the first `shredded_at` and writes no second `retention.evidence_shredded` event. Section 3.6 "sets `shredded_at = now()`" holds for the first call only.

### 7.9 RelationshipEndResolver in section 2 (V-23-04-c, LOW; DR-413)
Section 2, services: `RelationshipEndResolver::end_date(conn, party_id: Uuid) -> Result<Option<NaiveDate>, RepoError>` (`akr-worker`). It gives the party end date of section 3.5, and `None` while the party is not due: the relationship is open, or the party has no customer and no account role.

### 7.10 `feat.feature_runs` is purged by row (V-23-06-a, MEDIUM; DR-413)
Lead decision. Phase 13 does not partition `feat.feature_runs` (one row per month and feature), so a partition drop could never run. Its R4 handler is `record_purge` by age on `as_of`, in batches, as for `admin.quarantine_items`. Migration `0072` (section 1.2): the DELETE grant of `akr_retention` gains `feat.feature_runs`, and the table leaves the allow-list of `admin.retention_drop_partition`. The phase 13 assignment row takes the clock `age`. A row of this table holds no party data (one row per month and feature), so no party or case hold stops its purge, and it is no longer among the droppable parents of section 3.4.

### 7.11 `assess.decisions` in the drop function, and the R7 exception (V-23-10-a, MEDIUM; V-23-10-b, LOW; DR-413)
The allow-list of `admin.retention_drop_partition` (section 1.2 step 1) names `assess.decisions` from this migration on. The table arrives with phase 29; until then step 2 refuses the name, because no partition is a child of it. Phase 29 therefore re-creates nothing. Class R7 is `report_only` (section 3.2 and 7.2) with this one exception: `assess.decisions` is R7 on a partition clock and leaves by partition drop under the rule of 7.2.

## Decisions

Numbered from DR-184 (the last number in the research log is DR-183). Each decision is one paragraph with its rationale.

**DR-184 (shape of phase 23).** Migrations `0071_retention_holds_runs` and `0072_retention_guards_grants`; seven tables and one sequence in `admin`; one column added to `admin.evidence_objects`; nine routes of `/retention` (the six rows of API.md 3.4 owned here, three added: a hold detail and the run list and detail); job kinds `retention_run`, `retention_lock_extend` and `evidence_reconcile`; the Cedar file `61_retention.cedar`. The DATA-MODEL section 18 names are kept where they fit and the changes are listed in section 1: `reason` becomes a closed code plus a sealed text, and the certificate counts are `row_count` and `object_count`. The phase is M by its route count, but its logic is the heaviest part of the milestone (one plan, a signed manifest per batch, a replay), and the complexity line says so.

**DR-185 (one engine, one plan, a registry checked by a test).** The nightly run and the dry run are the same code: a dry run cannot differ from a real run except by the writes it skips. Each assignment row of phase 00 maps to one disposal handler in code; a unit test fails when a row has no handler or a handler has no row. No second assignment table is created: the phase 00 table is the assignment, and its rows of earlier phases are not changed. A table that is not assigned cannot be deleted by the engine, so a new phase cannot forget its retention without the build failing.

**DR-186 (periods are governed settings; the floor wins).** The per-class periods are governed settings with the Iraq floors (research 30 section 12.3, the Iraq profile of D2.1(c)(1)) as `floor.min`; the effective period is the maximum of the configured value and the floor, and the engine applies the same maximum at run time. A bank lengthens a period through a change request; it cannot shorten below the floor (E_BELOW_FLOOR). Schedules and batch limits are not governed. The jurisdiction profiles (OFAC for R7, the EU deletion rule) are deferred (Open doubts 9): the product must not delete a Iraq-profile record early, and it does not delete any record early for a profile it does not yet apply.

**DR-187 (partitions leave only through one guarded function).** A partition is dropped only by `admin.retention_drop_partition`, a SECURITY DEFINER function executable by `akr_retention`, with an allow-list of parents, the range-hold check, the party-hold copy check under the partition lock, the raw batch check and the `DETACH` and `DROP`. The alert partition additionally waits for its open alerts, for every alert a case names and for the disposition instant of every row (lead review). Companion partitions are dropped as pairs. The audit partitions and the audit tables are never in the allow-list (R10, phase 02).

**DR-188 (copy before drop, into an evidence object).** A party or case hold on a dropped partition needs a copy of that partition's held rows, made in the same run and written as a sealed evidence object (kind `package`, owner `legal_hold`, class R6), with the partition's row count recorded and compared under the partition lock. A range hold does not copy: it blocks the drop, because no copy keeps a range. A raw landing table is neither copied nor blocked by a party or case hold, because its rows are in canon where the copy is taken; a parent with no party path is blocked while such a hold is active (Open doubts 22). The copy is shredded after the hold's release and a grace period of 30 days (a governed setting that may only lengthen).

**DR-189 (holds are placed by step-up and released by four-eyes workflow).** A hold is placed by M under the step-up class `decision` and never edited or deleted; its scope is one of three shapes and it is matched live at each run, so rows that arrive later are held. Release is a workflow `legal_hold_release` with one approval by an M who is neither the placer nor the proposer, and its terminal effect writes the release; a released hold is final, and a new hold is a new row. A hold placed after a partition has been dropped cannot restore its rows: the placement response says so and the only recovery is the backup (R13). The race is closed by the table lock of the drop (section 3.15), not by the timing of the run.

**DR-190 (record purge by party, with references and tombstones).** Customer-linked records (R3 and R8) are purged party by party on the relationship-end clock of the party (every account closed, no current customer row, the latest closing date). A party named by an alert, a case, a case party or a report is deferred, because those rows are retained (DR-191). Each deleted customer and account identity leaves a keyed tombstone, so the CBS extract cannot load a purged closed customer again. The purge is not a DELETE by any application role: only `akr_retention` holds the grant, on the tables of the party tree.

**DR-191 (cases and reports are due but not purged in this phase).** R5 alerts outside partitions, R6 cases, reports, STR copies and their evidence, R7 screening records and R9 and R11 history are reported as due and are not deleted by this phase. The reason: the purge of a case needs a change to phase 17 and 18 append-only guards, which is a bank decision on the record-keeping policy, and the first release must not decide it by accident. The report applies the later-of rule to reports (the case's seal and the filing), so the due date the bank will need is already computed (Open doubts 1).

**DR-192 (case holds are invisible to IA and SEC).** A case hold is omitted from the lists and counts that IA and SEC read, answers `E_NOT_FOUND` on the detail, carries no kind and no id in the audit event, and shows no held or deferred count to them. A party hold is shown to IA and SEC without the party id. Reason: the audit viewer and the audit trail are read by IA and SEC, and a hold on a case is the case's existence; the rule of the case access list (the tipping-off rule of the phase template) must hold for every reader outside it. The bank decides whether an internal auditor sees case holds through its own access (Open doubts 24).

**DR-193 (phase 23 owns the lock extension at closure).** Phase 02 extends a lock only when told and never shortens it, and phase 18 left the extension at closure open (its Open doubt 14). Phase 23 extends the evidence locks of a sealed case to `sealed_at` plus the class lock days, through a job the seal transaction enqueues, and never shortens them. The requirement M-60 that phase 18 cites is not listed in `docs/MODULES.md`; the reconciliation pass decides whether it is added there. The lock order of section 3.15 puts the case row before its evidence rows, as the rest of the phase does.

**DR-194 (certificates: a new key, a manifest before each delete, a plain WORM copy).** Deletion certificates are signed with a new transit key `akr-sign-cert`, not with the anchor key of the audit chain: the two keys have different verifiers, rotation and custody, and a certificate must stay verifiable when the audit chain is being verified. Each batch writes its manifest, signed and locked in the COMPLIANCE bucket, before its rows are deleted, so no row leaves without a signed record of which keyed identity it was. The identities are HMACs under the match key, so a certificate, a manifest or a tombstone never carries a clear id, key or value. The certificate documents are plain JSON objects in the WORM bucket, not sealed evidence, because the replay must read them when the database that would hold their data key is the one that was restored.

**DR-195 (replay before reopening).** After a restore, the certificates newer than the restored database are verified and their manifests re-applied: partitions by name under the same guard, rows and evidence by the HMAC of their id. The replay writes its own certificates, so a second restore replays them too. The API stays closed until the replay reports no failed line; `akr-ctl retention verify-certificates` checks the chain without changing data. The HMAC identities make the replay possible without storing the ids, at the price of keeping every match key version that a manifest names (Open doubts 14).

**DR-196 (a request never runs the plan).** The API can insert a dry-run request only (the CHECK of 0071 refuses any other trigger), and `GET /retention/report` reads the latest completed run. A real run is only the schedule or an operator command on the host. This keeps the database load of the plan in the nightly window and makes the report the same figures the bank saw in the dry run.

**DR-197 (evidence follows its owner; orphans are listed and removed after their lock).** Only owner-following evidence is shredded in this phase (KYC records of a purged party and held-row copies after their grace period); evidence of cases, reports and packs is kept with its owner. The byte deletion waits for the COMPLIANCE lock, and the reconciliation lists the orphans that phase 02, 17 and 21 leave, deleting them only after their lock ends, because an orphan's key was never committed and it can be neither read nor shortened.

**DR-198 (the retention role).** `akr_retention` is a NOLOGIN role with a LOGIN user of its own in akr-worker's second pool. It has DELETE only on the tables of the party tree and the KYC family, EXECUTE on the drop function, UPDATE on the columns the phase needs, and SELECT on the schemas it plans from (`GRANT SELECT ON ALL TABLES` in those schemas, because a plan reads what the registry names; the coverage test of check 4a reads the list of tables it may read, so the grant is no wider than the registry). It has no DDL and no access to raw landing tables: the raw checks run inside the function, owned by `akr_migrate`.

**DR-199 (limits and the window).** Batches of `retention.batch_rows` rows, a pause between batches, a run limit of `retention.max_run_minutes`, `statement_timeout` of 30 seconds per batch and `lock_timeout` of 2 seconds for every DDL and party lock; one run at a time (the job concurrency, a session advisory lock, and the coalesce key); the window is the nightly schedule at 03:45 bank time. A run that is cut short is re-planned by the next night, so the database keeps no state between runs except the items and the manifests.

**DR-200 (the Cedar file).** `61_retention.cedar` is placed in the audit range of phase 03 because range 60 belongs to the audit file of phase 24; the number is the next free one in that range (Open doubts 23). Its permits are one role per `@id`, the case forbid is a tipping-off deny whose answer the handler turns into `E_NOT_FOUND`, and the release forbid is the four-eyes check on the placer.

**Lead corrections (review 2026-10-09).** An alert partition is dropped only when no row is open, no row is named by `casework.case_alerts` (its foreign key is `ON DELETE RESTRICT`, so the draft's "plain reference" was wrong and the `DETACH` would fail) and every row's disposition is past the R5 cut-off; the `created_at` partition alone could delete an alert before seven years from its disposition. Raw landing partitions are not blocked by party or case holds (only range holds and unmapped batches): the draft blocked every raw drop while any hold existed, keeping personal data years past its 90 days. The party paths of the droppable parents are listed in section 3.4; a parent with no path is blocked while a party or case hold is active. The drop counts under the partition's SHARE lock, not the parent's EXCLUSIVE lock, with its own time-out `retention.drop_timeout_s` (the pool's 30 s would have failed every large month). `E_CLAIMED` is allowed in `retention_runs.error_code`. The registry names `scn.detection_keys` (not `alert.detection_keys`) and adds `admin.quarantine_items` (R1, age). The R6 and R7 defaults are 7 years, as ARCHITECTURE section 17 gives.

## Open doubts (for gap pass 1 and the bank)

1. **Cases, reports, STR copies and their evidence are not purged (DR-191).** The record-keeping policy of the bank decides whether R6 records are deleted after seven or ten years and in what order. Until it decides, the due report will show growing counts. Default: keep, report.
2. **Sanctions-tied alerts.** ARCHITECTURE section 17 and the research give alerts tied to sanctions ten years. `alert.alerts` has no flag for it, so every R5 partition uses the seven-year floor. Phase 14's open question on freeze-list alerts (R7 or a ten-year R5) is the same question.
3. **The R5 clock.** The assignment says `partition` on `created_at`; the research says seven years from the report date. This phase drops an alert partition only when no row is open, no row is named by a case and every row's disposition is past the cut-off (section 3.3, lead review), so a month with one case-linked alert stays until the case purge is decided (Open doubts 1). The bank confirms that the disposition instant is the start of the R5 clock for alerts that led to no report.
4. **The relationship end of a customer.** `canon.customer.status` has no list of values in phase 10 (`TEXT` without a CHECK). The resolver uses `valid_to` and the account `closed_on` only. A party with no customer and no account role is `report_only`. The bank confirms the closed values.
5. **Name-only PEP candidates.** `kyc.pep_register` rows with `party_id` NULL have no owner (phase 15 item 9 and its retention note). Proposal: an age clock on `created_at` with the seven-year floor. Until decided they are `report_only`.
6. **The R4 look-back.** The research gives R4 "three years or the longest scenario look-back"; the look-back is not stored anywhere. The engine uses three years. A look-back registry from the scenario catalogue (phases 06 and 16) is needed before a bank shortens R4 below its longest look-back.
7. **The R1 floor.** ARCHITECTURE gives 30 to 400 days and the research gives no statutory floor. The 30 is a product bound in the floor JSON, not a legal one. Default: 90 days.
8. **Classes kept in this phase.** R9 and R11 are kept (research: configuration history outlives every alert it explains); R12 tables are pruned by `maintenance_sweep` (phase 00); R13 is pgBackRest expiry. The bank confirms that R13 is about five weeks in the backup plan (the DBS scheme of research 30 section 12.3; the AKR backup plan file is not yet in `docs/plan`).
9. **Jurisdiction profiles.** The OFAC ten-year rule for R7 and the EU five-year deletion rule are not built: there is no profile setting and no profile floor. A per-bank profile setting with its floor, governed, is needed before a second profile is switched on.
10. **Cost of the partition count.** The drop counts the partition under its own SHARE lock (writers of other partitions continue) inside a transaction bounded by `retention.drop_timeout_s` (600 s). For a very large month the count may still exceed it; the item is then `failed` and retried. An estimate with a re-count is the alternative and weakens the check.
11. **One M in the bank.** The release needs an M who is neither the placer nor the proposer. With one M the release cannot be approved. The single-officer exception of phase 21 (`report.single_officer_exception`) is not applied to holds. The bank names a second M.
12. **Tombstones and the ingest writers.** The read of tombstones is an addition to the phase 10 and 11 writers, and the lifting of a tombstone (for a customer who returns) is not specified. Default: lifting is a change request by M and CHK, not designed here.
13. **Monitoring copy versus archive.** The purge deletes AKR-TMS's monitoring copy. The bank's own archive keeps the record (research 30 section 12.2). The bank confirms that its record-keeping statement accepts a monitoring copy that ends at the class period.
14. **Match key versions.** A replay needs the match key version a manifest names, for as long as the manifest exists (ten years in the COMPLIANCE bucket). The OpenBao retention of old key versions and the procedure when a version is gone are not specified. A missing version stops the replay with `E_UNAVAILABLE`.
15. **The restore gate.** The mechanism that keeps the API closed until the replay is clean (a readiness flag, a maintenance mode, the name of the reopen command) is not in phase 00. Default: a readiness flag set by `akr-ctl retention replay` and cleared by a clean report.
16. **Orphans and storage.** A COMPLIANCE-locked orphan stays for the class lock (seven years for R5 to R8 by `evidence.lock_days`). The bank accepts the storage cost, or phase 02's lock floors are reviewed.
17. **The lock extension rule.** The extension is `sealed_at` plus `evidence.lock_days[class]`. A case that is reopened and sealed again extends again. The bank confirms that the seal, not the object's creation, starts the seven years.
18. **Held copies.** The copy's grace period (30 days after release) and its class (R6) are proposals. The bank confirms that a copy of a released hold may be shredded in 30 days.
19. **Size of a held copy.** One evidence object per partition and hold can be large for a party with many rows. The batch limit and the evidence size limit of phase 02 must be checked for a large party before the first hold.
20. **Counts for IA and SEC.** The held and deferred counts are shown to M only (DR-192). The bank may want the counts for audit; that is a decision, not a default.
21. **The roadmap.** Row 23 of `docs/plan/roadmap.md` lists dependency 18 only; this phase needs 20 and 21 (the freeze and report clocks). Row 22 says "next numbers (from 0071)", which is the number of this phase's first migration: phase 22 must take 0073 or later, or the lead renumbers.
22. **Reference columns not verified.** The party columns of `ref.list_*` are not checked. The blocking list of section 3.5 names five tables; the party subject of `alert.screening_alerts` is in the phase 14 DDL (`subject_kind`, `subject_id`). `canon.txn` has the index `idx_txn_account` on `account_id` in phase 10, which the account purge uses. `canon.counterparty` and `canon.payment_link` are `report_only` until their references are checked. The parents without a party path (section 3.4) are blocked while any party or case hold is active; the bank's first hold will show whether their paths (through `canon.txn` for cash events and payment messages) are needed.
23. **The Cedar file number.** `61_retention.cedar` sits in the range 60 to 69 that phase 03 gives to audit. Phase 24's audit file is `60_audit.cedar`. The bundle owner confirms the number.
24. **Case holds and internal audit.** DR-192 hides a case hold from IA and SEC. An internal auditor checking the bank's compliance with a regulator's request may need to see it. The bank decides whether IA sees case holds through the case access list (a grant on the case) rather than through the hold list.
25. **Families not covered by a purge.** R7 screening decisions, R5 alert children, R4 non-partitioned features, R3 staff and R9 history are `report_only` in this phase. The bank and the gap pass name the phase that purges each of them, or accept that they are kept.
26. **Deferred parties.** A party that was once in a case is deferred for as long as the case exists (DR-190, DR-191). The first report will show how many parties are deferred this way; the bank should expect that number to be large until the case purge is decided (Open doubts 1).

## What This Phase Does NOT Touch

- No change to phase 02's audit partitions, audit tables, the hash chain or the anchors; `audit.audit_log` and `audit.access_event` are never dropped, and their R10 class stays `never`.
- No DELETE on `canon` or `kyc` for any role other than `akr_retention`; no DDL for `akr_retention`; no change to the phase 10 rule that partitions leave only through the retention engine.
- No purge of R6 cases, reports, STR copies or their evidence, and no change to the append-only guards of phases 17, 18, 19 and 21 (DR-191).
- No change to `maintenance_sweep` or to the phase 00 operations tables and their R12 classes; no change to the phase 00 settings rows.
- No change to the `admin.retention_assignment` rows of earlier phases: the relationship-end resolver and the later-of rule are code of this phase (DR-185, section 3.2).
- No backup retention change: pgBackRest and restic expiry are operations (R13); no archive of personal data is added.
- No CBS connection change and no change to the phase 05 and 10 conformance suites; the tombstone read is a read on the AKR database.
- No audit viewer, verifier or export route (phase 24 owns `/audit/*`); the evidence stream of `/evidence/{id}` is phase 18's.
- No new reason-code family: phase 04's CHECK on `ref.reason_codes` is unchanged (the hold reasons are a closed set in the table).
- No screen beyond G7; no notice to a customer, to the Office or to goAML; no CBI report.
- No new binary, network zone, outbox topic or timer kind.
- No edit of `docs/API.md`, `docs/DATA-MODEL.md`, `docs/DSL.md`, `docs/plan/roadmap.md`, `docs/plan/research.md` or `docs/plan/status.md`: the reconciliation pass of section 4.5 item 10 applies them.
