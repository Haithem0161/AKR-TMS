# Phase 12: Reconciliation, Coverage & Data Quality

**Goal:** The daily proof that the monitored data is complete: a reconciliation ladder per source and business date
(batch receipt, row counts, control totals and GL tie-out) that opens a data-quality issue for every break and never
nets a difference off; a coverage certificate per source and day, signed by the system and attested by the MLRO where
gaps are declared; a coverage ledger that marks dark units instead of assuming them quiet; the exclusion register with
four-eyes approval, expiry and a computed volume effect; the data-quality register with owners, ages and bank-signed
expiring known adjustments; and the quarantine queue with its owner, retry and exclusion workflow over the phase 11
table. Screens T1 (completed), T2 and T4.

**Dependencies:** Phase 02 (evidence vault and the transit key `akr-sign-certificate`), Phase 04 (ChangeTargetRegistry for
the `exclusion` and `known_adjustment` targets), Phase 05 (`SourceReader`, `SelectProof`, `LoadGovernor`, the adapter
pack that names the approved control-totals objects), Phase 08 (shell, data table and `StepUpDialog`; basic T1), Phase 09
(bank calendar for working days, `exclusion` reason codes), Phase 10 (`CanonicalWriter` and its `RowConflict`s; the
`canon` tables), Phase 11 (`batches`, `quarantine_items`, `watermarks`, `blind_periods`, the freshness service and the
load loop that calls the recorder of section 3.3)
**Macro-phase:** P1
**Complexity:** L (16 routes; the ladder, the certificate and the exclusion change target carry the weight)
**Binaries touched:** akr-api (routers `recon`, `coverage`, `dq`, `quarantine`; change targets `exclusion` and
`known_adjustment` registered in process) | akr-worker (jobs `recon_run`, `recon_sweep`, `coverage_ledger_refresh`,
`coverage_certificate`, `dq_expiry_sweep`) | akr-ingestd (job `ingest_control_totals`; the `DataQualityRecorder` library
called by phase 11's load loop). `akr-ctl` and `akr-assess` unchanged.
**Requirements carried:**
- C-25 (reconciliation ladder with the four outcomes reconciled, open, confirmed gap and source anomaly; rungs receipt,
  counts, control totals and GL tie-out of section 1; rungs L2, L4, L5 and L6 deferred, see Open doubts)
- C-26 (daily coverage certificate per source and business date, signed with `akr-sign-certificate` and sealed in the
  evidence vault; the feed into the annual certification pack is phase 24)
- C-27 (MLRO coverage statement with the covered share, data as of and blind periods; the banner on cases that touch a
  blind period is phase 18)
- C-28 (data-quality issues with owners, thresholds and ages; the bank-signed, expiring known-adjustments register)
- C-29 (`data_as_of` and the stale state on every response, screen and certificate of this phase; the freshness classes
  F0-F5 are phase 11)
- L-01 (coverage ledger of units, with dark units and the monitored share by count and by value)
- L-02 (exclusion register: owner, reason, approver, review date and expiry; an expired exclusion is an open finding)
- L-03 (partial: an unmapped code opens a data-quality issue that stays open until a resolution is recorded; the gate
  itself is phases 10 and 11)
- L-04 (daily reconciliation of records read, landed and mapped per table and group, with a tolerance and one issue per
  break)
- L-05 (no silent rejection: the quarantine queue with reason, age and owner, retry and exclusion, shown on T4)
- Gap pass 1 (coverage), partly implemented: K-31 (the data-quality register with owners and ages); L-14 (exclusions with reasons and volume counts in the coverage ledger); VA-15 (the coverage ledger with dark units and the monitored share); VC-32 (the data-quality register and the placeholder-date rule).
- Gap pass 1 (coverage), implemented here: B-01 (daily reconciliation of counts and sums against the CBS, breaks shown as data-quality issues); VA-16 (the coverage ledger by product, channel and transaction type; exclusions with owner, reason and expiry).
- Gap pass 1 (coverage), not carried: VC-19 (payment-format readiness report; DR-404).

Named by M7 and carried by other phases: C-48, S-02, S-03 and S-54 (phase 06); C-24 and L-06 (the writer's conflict
guard and drift, phase 10); the M7 P0 gap report (phase 06).

---

## 1. SQLx Migration Changes

Four migrations, `0036_recon_runs_results`, `0037_dq_issues`, `0038_coverage_ledger_certs_exclusions` and
`0039_recon_settings_grants`, all in `admin` (DATA-MODEL s.6 places the reconciliation and coverage tables there; group
12 of DATA-MODEL s.23). Six new tables: `recon_runs`, `recon_results`, `dq_issues`, `coverage_ledger`, `exclusions` and
`coverage_certificates`. No phase 11 table is changed: phase 12 uses the owner and status columns that phase 11 already
defines and grants on `quarantine_items`.
Every table below names its owning writer, its retention class and its grants in the migration itself.

```sql
-- migration 0036_recon_runs_results.sql
-- Phase 12. Reconciliation runs and their rung results (MODULES M7, DATA-MODEL s.6, C-25, L-04). A day run covers the
-- receipt, counts, control totals and GL tie-out of one source and business date; a batch run covers the receipt and
-- counts of one batch. Source-side expected figures are written by akr-ingestd (ingest_control_totals, akr_ingest);
-- AKR-side actuals and outcomes by akr-worker (recon_run, akr_app). No row is ever deleted.

DO $$ BEGIN
    CREATE TYPE admin.recon_status AS ENUM ('queued', 'awaiting_totals', 'running', 'completed', 'failed');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    CREATE TYPE admin.recon_outcome AS ENUM ('reconciled', 'open', 'confirmed_gap', 'source_anomaly');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS admin.recon_runs (
    id                  UUID PRIMARY KEY,
    scope               TEXT NOT NULL CHECK (scope IN ('batch', 'day')),
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    business_date       DATE NOT NULL,                            -- the source business date (DR-39)
    batch_id            UUID REFERENCES admin.batches(id) ON DELETE RESTRICT,   -- batch scope only
    status              admin.recon_status NOT NULL DEFAULT 'queued',
    requested_by        UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,   -- the system principal for sweeps
    totals_captured_at  TIMESTAMPTZ,                              -- set by ingest_control_totals (day scope)
    outcome_counts      JSONB,                                    -- {rung: {reconciled, open, confirmed_gap, source_anomaly}}
    open_results        INT NOT NULL DEFAULT 0 CHECK (open_results >= 0),
    error_code          TEXT CHECK (error_code IN ('totals_unavailable', 'source_paused', 'internal')),
    started_at          TIMESTAMPTZ,
    finished_at         TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    CHECK ((scope = 'batch') = (batch_id IS NOT NULL)),
    CHECK ((status IN ('completed', 'failed')) = (finished_at IS NOT NULL)),
    CHECK ((status = 'failed') = (error_code IS NOT NULL)),
    CHECK (status <> 'completed' OR outcome_counts IS NOT NULL)
);
-- One active day run per source and business date (a second request answers 409 run_active). A completed run does
-- not block a later one: late batches are re-run as a new revision of the certificate (section 3.8).
CREATE UNIQUE INDEX IF NOT EXISTS uq_recon_runs_day_active ON admin.recon_runs (source_id, business_date)
    WHERE scope = 'day' AND status IN ('queued', 'awaiting_totals', 'running');
CREATE UNIQUE INDEX IF NOT EXISTS uq_recon_runs_batch_active ON admin.recon_runs (batch_id)
    WHERE scope = 'batch' AND status IN ('queued', 'running');
CREATE INDEX IF NOT EXISTS idx_recon_runs_source_date ON admin.recon_runs (source_id, business_date DESC, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_recon_runs_batch       ON admin.recon_runs (batch_id) WHERE batch_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_recon_runs_requested_by ON admin.recon_runs (requested_by);
CREATE INDEX IF NOT EXISTS idx_recon_runs_waiting     ON admin.recon_runs (created_at) WHERE status = 'awaiting_totals';

CREATE TABLE IF NOT EXISTS admin.recon_results (
    id                  UUID PRIMARY KEY,
    run_id              UUID NOT NULL REFERENCES admin.recon_runs(id) ON DELETE RESTRICT,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    rung                TEXT NOT NULL CHECK (rung IN ('receipt', 'counts', 'control_totals', 'gl_tieout')),
    object              TEXT NOT NULL,                            -- source object, or batch:<uuid> on the receipt rung
    group_key           TEXT NOT NULL DEFAULT '',                 -- date|branch|currency|ledger on the sum rungs
    expected            NUMERIC(28,6),                            -- source side, written by ingest_control_totals
    actual              NUMERIC(28,6),                            -- AKR-TMS side, written by recon_run
    delta               NUMERIC(28,6),                            -- expected minus actual
    tolerance           NUMERIC(28,6) NOT NULL DEFAULT 0 CHECK (tolerance >= 0),   -- the value in force when evaluated
    within_tolerance    BOOLEAN,
    outcome             admin.recon_outcome,
    reason_code         TEXT,                                     -- count_mismatch, sum_mismatch, batch_not_mapped, ...
    detail              JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(detail) = 'object'),   -- codes and counts only
    source_captured_at  TIMESTAMPTZ,
    evaluated_at        TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_recon_results_key UNIQUE (run_id, rung, object, group_key),
    CHECK ((outcome IS NULL) = (evaluated_at IS NULL)),
    CHECK (outcome <> 'reconciled' OR within_tolerance IS TRUE)
);
CREATE INDEX IF NOT EXISTS idx_recon_results_source   ON admin.recon_results (source_id);
CREATE INDEX IF NOT EXISTS idx_recon_results_breaks   ON admin.recon_results (source_id, evaluated_at DESC)
    WHERE outcome IN ('open', 'source_anomaly');

GRANT SELECT, INSERT ON admin.recon_runs TO akr_app;
GRANT UPDATE (status, totals_captured_at, outcome_counts, open_results, error_code, started_at, finished_at, version,
              updated_at) ON admin.recon_runs TO akr_app;
GRANT SELECT ON admin.recon_runs TO akr_ingest;
GRANT UPDATE (totals_captured_at, version, updated_at) ON admin.recon_runs TO akr_ingest;
-- Expected rows are INSERTed by akr_ingest (ingest_control_totals) and receipt or evaluated rows by akr_app; the
-- evaluation columns are the only ones akr_app may UPDATE; no role may DELETE.
GRANT SELECT, INSERT ON admin.recon_results TO akr_app, akr_ingest;
GRANT UPDATE (actual, delta, tolerance, within_tolerance, outcome, reason_code, detail, evaluated_at)
    ON admin.recon_results TO akr_app;
GRANT SELECT ON admin.recon_runs, admin.recon_results TO akr_readonly;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('recon_runs', 'internal', ARRAY['id','scope','source_id','business_date','batch_id','status','requested_by',
                                   'totals_captured_at','outcome_counts','open_results','error_code','started_at',
                                   'finished_at','created_at','updated_at','version']),
  ('recon_results', 'financial', ARRAY['expected','actual','delta','tolerance']),
  ('recon_results', 'internal', ARRAY['id','run_id','source_id','rung','object','group_key','within_tolerance',
                                      'outcome','reason_code','detail','source_captured_at','evaluated_at','created_at'])
) AS v(t, cls, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'recon_runs',    'R9', 'age', 'created_at', 'Reconciliation history; outlives the issues it explains'),
 ('admin', 'recon_results', 'R9', 'age', 'created_at', 'Rung results behind each certificate; kept as its run')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

```sql
-- migration 0037_dq_issues.sql
-- Phase 12. The data-quality register (MODULES M7, DATA-MODEL s.6, C-28, L-03, DSL.md B.5 on_fail dq_issue). One open
-- row per (source, object, field, kind): the recorder of section 3.3 counts recurrences on it instead of writing one
-- row per record, so no personal value ever reaches this table. Resolved and accepted rows are kept as the record.

DO $$ BEGIN
    CREATE TYPE admin.dq_status AS ENUM ('open', 'resolved', 'accepted');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS admin.dq_issues (
    id                  UUID PRIMARY KEY,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    object              TEXT NOT NULL,                            -- source table, landed object or `<rung>:<object>` of a recon break
    field               TEXT NOT NULL DEFAULT '',                 -- attribute or column; '' when the issue is about the row
    kind                TEXT NOT NULL CHECK (kind IN ('row_conflict_fact', 'row_conflict_label', 'row_conflict_value',
                                                      'unmapped_code', 'validation_rule', 'unresolved_pointer',
                                                      'card_guard', 'recon_break')),
    severity            TEXT NOT NULL CHECK (severity IN ('low', 'medium', 'high', 'critical')),
    count               BIGINT NOT NULL DEFAULT 1 CHECK (count >= 1),
    first_seen          TIMESTAMPTZ NOT NULL,
    last_seen           TIMESTAMPTZ NOT NULL,
    due_at              TIMESTAMPTZ NOT NULL,                     -- first_seen plus the severity SLA (setting dq.sla_days)
    status              admin.dq_status NOT NULL DEFAULT 'open',
    owner_id            UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    run_id              UUID REFERENCES admin.recon_runs(id) ON DELETE RESTRICT,
    batch_id            UUID REFERENCES admin.batches(id) ON DELETE RESTRICT,
    resolution          TEXT CHECK (resolution IN ('fixed', 'explained', 'not_applicable', 'superseded')),
    resolution_note     TEXT CHECK (resolution_note IS NULL OR length(resolution_note) <= 2000),
    resolved_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    resolved_at         TIMESTAMPTZ,
    accepted_until      DATE,                                     -- set only by an activated known_adjustment
    signed_document_id  UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    change_request_id   UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    detail              JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(detail) = 'object'),   -- codes, counts, ids
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    CHECK (last_seen >= first_seen),
    CHECK ((status = 'resolved') = (resolved_at IS NOT NULL AND resolved_by IS NOT NULL AND resolution IS NOT NULL)),
    CHECK ((status = 'accepted') = (accepted_until IS NOT NULL AND signed_document_id IS NOT NULL
                                    AND change_request_id IS NOT NULL)),
    CHECK (status <> 'open' OR (resolved_at IS NULL AND accepted_until IS NULL)),
    CHECK (kind <> 'card_guard' OR status <> 'accepted')       -- card data is fixed at source, never accepted
);
-- The recorder's upsert arbiter: one open issue per (source, object, field, kind).
CREATE UNIQUE INDEX IF NOT EXISTS uq_dq_issues_open ON admin.dq_issues (source_id, object, field, kind)
    WHERE status = 'open';
CREATE INDEX IF NOT EXISTS idx_dq_issues_queue     ON admin.dq_issues (status, severity, due_at);
CREATE INDEX IF NOT EXISTS idx_dq_issues_owner     ON admin.dq_issues (owner_id) WHERE owner_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_dq_issues_run       ON admin.dq_issues (run_id) WHERE run_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_dq_issues_batch     ON admin.dq_issues (batch_id) WHERE batch_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_dq_issues_accepted  ON admin.dq_issues (accepted_until) WHERE status = 'accepted';
CREATE INDEX IF NOT EXISTS idx_dq_issues_resolved_by ON admin.dq_issues (resolved_by) WHERE resolved_by IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_dq_issues_signed    ON admin.dq_issues (signed_document_id) WHERE signed_document_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_dq_issues_cr        ON admin.dq_issues (change_request_id) WHERE change_request_id IS NOT NULL;

-- akr_ingest records conflicts and validation failures (upsert: INSERT, then UPDATE of the count columns only).
-- akr_app assigns, resolves, accepts (writing detail.accepted_delta) and expires, and upserts the recon breaks of
-- recon_run (count, last_seen); nobody deletes.
GRANT SELECT, INSERT ON admin.dq_issues TO akr_app, akr_ingest;
GRANT UPDATE (count, last_seen, updated_at) ON admin.dq_issues TO akr_ingest;
GRANT UPDATE (owner_id, status, resolution, resolution_note, resolved_by, resolved_at, accepted_until,
              signed_document_id, change_request_id, due_at, updated_at, version, detail, count, last_seen) ON admin.dq_issues TO akr_app;
GRANT SELECT ON admin.dq_issues TO akr_readonly;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('dq_issues', 'internal', ARRAY['id','source_id','object','field','kind','severity','count','first_seen','last_seen',
                                  'due_at','status','owner_id','run_id','batch_id','resolution','resolved_by',
                                  'resolved_at','accepted_until','signed_document_id','change_request_id','detail',
                                  'created_at','updated_at','version']),
  -- free text typed by a steward may name a customer; masked outside the data steward role
  ('dq_issues', 'personal', ARRAY['resolution_note'])
) AS v(t, cls, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'dq_issues', 'R9', 'age', 'created_at', 'Issue history: closed and accepted issues are the record of each break')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

```sql
-- migration 0038_coverage_ledger_certs_exclusions.sql
-- Phase 12. The exclusion register (L-02, DSL.md A.7 admin.exclusions), the coverage ledger with dark units (L-01,
-- DATA-MODEL s.6) and the daily coverage certificates (C-26). Exclusions are created only by an activated change request
-- (section 3.6); the ledger is recomputed by coverage_ledger_refresh; certificates are written by coverage_certificate.

DO $$ BEGIN
    CREATE TYPE admin.certificate_status AS ENUM ('issued', 'pending_attestation', 'attested');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS admin.exclusions (
    id                      UUID PRIMARY KEY,
    source_id               UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    unit                    TEXT NOT NULL CHECK (unit IN ('branch', 'product', 'txn_type', 'channel', 'feed')),
    unit_ref                TEXT NOT NULL CHECK (length(unit_ref) BETWEEN 1 AND 120),
    reason_code             TEXT NOT NULL CHECK (reason_code ~ '^EXC_[A-Z_]{3,40}$'),   -- an active exclusion code (DR-82)
    reason                  TEXT NOT NULL CHECK (length(reason) BETWEEN 10 AND 2000),
    owner_id                UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    approved_by             UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,   -- the last decider
    review_due              DATE NOT NULL,
    expires_at              TIMESTAMPTZ NOT NULL,
    change_request_id       UUID NOT NULL REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    volume_effect_snapshot  JSONB NOT NULL CHECK (jsonb_typeof(volume_effect_snapshot) = 'object'),   -- impact at approval
    ended_at                TIMESTAMPTZ,
    ended_reason            TEXT CHECK (ended_reason IN ('expired', 'superseded')),
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    version                 INT NOT NULL DEFAULT 1,
    CHECK (approved_by <> owner_id),
    CHECK ((ended_at IS NULL) = (ended_reason IS NULL)),
    CHECK (ended_at IS NULL OR ended_at >= created_at)
);
-- One live exclusion per unit: a second proposal answers 409 exclusion_active until the first one ends.
CREATE UNIQUE INDEX IF NOT EXISTS uq_exclusions_active ON admin.exclusions (source_id, unit, unit_ref)
    WHERE ended_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_exclusions_expiry  ON admin.exclusions (expires_at) WHERE ended_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_exclusions_owner   ON admin.exclusions (owner_id);
CREATE INDEX IF NOT EXISTS idx_exclusions_approver ON admin.exclusions (approved_by);
CREATE INDEX IF NOT EXISTS idx_exclusions_cr      ON admin.exclusions (change_request_id);

CREATE TABLE IF NOT EXISTS admin.coverage_ledger (
    id                  UUID PRIMARY KEY,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    unit                TEXT NOT NULL CHECK (unit IN ('branch', 'product', 'txn_type', 'channel', 'feed')),
    unit_ref            TEXT NOT NULL CHECK (length(unit_ref) BETWEEN 1 AND 120),
    covered             BOOLEAN NOT NULL,
    reason              TEXT NOT NULL CHECK (reason IN ('monitored', 'excluded', 'blind_period', 'source_stale',
                                                        'no_postings_since')),
    since               DATE,                                     -- last posting date when dark; NULL if none in the lookback
    exclusion_id        UUID REFERENCES admin.exclusions(id) ON DELETE RESTRICT,
    as_of_business_date DATE NOT NULL,                            -- the source business date used (DR-39)
    volume_window_days  INT NOT NULL CHECK (volume_window_days BETWEEN 1 AND 365),
    volume_count        BIGINT CHECK (volume_count IS NULL OR volume_count >= 0),
    volume_value_lcy    NUMERIC(28,6) CHECK (volume_value_lcy IS NULL OR volume_value_lcy >= 0),
    share_count         NUMERIC(9,6) CHECK (share_count IS NULL OR share_count BETWEEN 0 AND 1),
    share_value         NUMERIC(9,6) CHECK (share_value IS NULL OR share_value BETWEEN 0 AND 1),
    volume_basis        TEXT NOT NULL CHECK (volume_basis IN ('canon_window', 'none')),
    computed_at         TIMESTAMPTZ NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    CONSTRAINT uq_coverage_ledger_unit UNIQUE (source_id, unit, unit_ref),
    CHECK (covered = (reason = 'monitored')),
    CHECK ((reason = 'excluded') = (exclusion_id IS NOT NULL)),
    CHECK ((volume_basis = 'none') = (volume_count IS NULL))
);
CREATE INDEX IF NOT EXISTS idx_coverage_ledger_dark ON admin.coverage_ledger (source_id, unit) WHERE NOT covered;
CREATE INDEX IF NOT EXISTS idx_coverage_ledger_exclusion ON admin.coverage_ledger (exclusion_id)
    WHERE exclusion_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS admin.coverage_certificates (
    id                  UUID PRIMARY KEY,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    business_date       DATE NOT NULL,
    revision            INT NOT NULL CHECK (revision >= 1),       -- 1 for the first certificate; no predecessor needed
    run_id              UUID NOT NULL REFERENCES admin.recon_runs(id) ON DELETE RESTRICT,
    status              admin.certificate_status NOT NULL,
    summary             JSONB NOT NULL CHECK (jsonb_typeof(summary) = 'object'),
    gaps                JSONB NOT NULL CHECK (jsonb_typeof(gaps) = 'array'),   -- declared gaps (section 3.8)
    data_as_of          JSONB NOT NULL CHECK (jsonb_typeof(data_as_of) = 'object'),
    stale               BOOLEAN NOT NULL,
    document_sha256     BYTEA NOT NULL,                           -- SHA-256 of the canonical sealed JSON
    evidence_object_id  UUID NOT NULL REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    signature           TEXT NOT NULL,                            -- akr-sign-certificate over document_sha256
    key_version         INT NOT NULL CHECK (key_version >= 1),
    issued_at           TIMESTAMPTZ NOT NULL,
    attested_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    attested_at         TIMESTAMPTZ,
    attestation_digest  BYTEA,                                    -- SHA-256 of the gaps array the MLRO attested
    superseded_at       TIMESTAMPTZ,                              -- set when a later revision is issued
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    CONSTRAINT uq_coverage_certificates_revision UNIQUE (source_id, business_date, revision),
    CHECK (status <> 'issued' OR jsonb_array_length(gaps) = 0),
    CHECK (status <> 'pending_attestation' OR jsonb_array_length(gaps) > 0),
    CHECK (status <> 'attested' OR jsonb_array_length(gaps) > 0),
    CHECK ((status = 'attested') = (attested_by IS NOT NULL AND attested_at IS NOT NULL AND attestation_digest IS NOT NULL)),
    CHECK (superseded_at IS NULL OR superseded_at >= issued_at)
);
-- One current certificate per source and date; older revisions keep superseded_at.
CREATE UNIQUE INDEX IF NOT EXISTS uq_coverage_certificates_current ON admin.coverage_certificates (source_id, business_date)
    WHERE superseded_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_coverage_certificates_date ON admin.coverage_certificates (business_date DESC, source_id);
CREATE INDEX IF NOT EXISTS idx_coverage_certificates_run  ON admin.coverage_certificates (run_id);
CREATE INDEX IF NOT EXISTS idx_coverage_certificates_evidence ON admin.coverage_certificates (evidence_object_id);
CREATE INDEX IF NOT EXISTS idx_coverage_certificates_attested_by ON admin.coverage_certificates (attested_by)
    WHERE attested_by IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_coverage_certificates_pending ON admin.coverage_certificates (issued_at)
    WHERE status = 'pending_attestation' AND superseded_at IS NULL;

-- No role has DELETE on these tables. Exclusions change only their end columns: a row is ended, never edited.
GRANT SELECT, INSERT ON admin.exclusions TO akr_app;
GRANT UPDATE (ended_at, ended_reason, version) ON admin.exclusions TO akr_app;
GRANT SELECT, INSERT ON admin.coverage_ledger TO akr_app;
GRANT UPDATE (covered, reason, since, exclusion_id, as_of_business_date, volume_window_days, volume_count,
              volume_value_lcy, share_count, share_value, volume_basis, computed_at, updated_at, version)
    ON admin.coverage_ledger TO akr_app;
GRANT SELECT, INSERT ON admin.coverage_certificates TO akr_app;
GRANT UPDATE (status, attested_by, attested_at, attestation_digest, superseded_at, version)
    ON admin.coverage_certificates TO akr_app;
GRANT SELECT ON admin.exclusions, admin.coverage_ledger, admin.coverage_certificates TO akr_readonly;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('exclusions', 'detection-logic', ARRAY['unit','unit_ref','reason_code','reason','volume_effect_snapshot']),
  ('exclusions', 'internal', ARRAY['id','source_id','owner_id','approved_by','review_due','expires_at',
                                   'change_request_id','ended_at','ended_reason','created_at','version']),
  ('coverage_ledger', 'detection-logic', ARRAY['unit','unit_ref','covered','reason','since','exclusion_id']),
  ('coverage_ledger', 'financial', ARRAY['volume_count','volume_value_lcy','share_count','share_value']),
  ('coverage_ledger', 'internal', ARRAY['id','source_id','as_of_business_date','volume_window_days','volume_basis',
                                        'computed_at','created_at','updated_at','version']),
  ('coverage_certificates', 'detection-logic', ARRAY['summary','gaps']),
  ('coverage_certificates', 'internal', ARRAY['id','source_id','business_date','revision','run_id','status',
                                              'data_as_of','stale','document_sha256','evidence_object_id','signature',
                                              'key_version','issued_at','attested_by','attested_at',
                                              'attestation_digest','superseded_at','created_at','version'])
) AS v(t, cls, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'exclusions',            'R9', 'retirement', NULL,         'Governance register; ended rows kept (L-02)'),
 ('admin', 'coverage_ledger',       'R9', 'retirement', NULL,         'Current state; history lives in the certificates'),
 ('admin', 'coverage_certificates', 'R6', 'age',        'issued_at', 'Evidence of coverage (DATA-MODEL s.6); seven-year floor')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

```sql
-- migration 0039_recon_settings_grants.sql
-- Phase 12. Settings and schedules for the ladder, the ledger and the register, for the ladder, the ledger and the register.
-- No column is added to a phase 11 table, and no grant: the owner and exclusion columns and their grants exist (0033).

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('recon.gl_tolerance',               '0.01', 'decimal', NULL, TRUE,
  '{"ar": "سماح مطابقة الأستاذ العام لكل مجموعة", "en": "GL tie-out tolerance per group (currency units, bank data)"}'),
 ('recon.control_total_tolerance',    '0', 'decimal', NULL, TRUE,
  '{"ar": "سماح مجاميع التحكم لكل مجموعة", "en": "Control-total tolerance per group"}'),
 ('recon.lookback_days',              '14', 'integer', NULL, FALSE,
  '{"ar": "عدد أيام العمل التي يعيد المجدول مطابقتها", "en": "Working days the daily sweep re-checks"}'),
 ('recon.totals_max_wait_h',          '6', 'integer', NULL, FALSE,
  '{"ar": "أقصى انتظار لمجاميع المصدر قبل الإخفاق (ساعات)", "en": "Longest wait for the source control totals (hours)"}'),
 ('coverage.dark_unit_days',          '5', 'integer', NULL, FALSE,
  '{"ar": "أيام عمل بلا قيود تجعل الوحدة معتمة", "en": "Working days without postings that make a unit dark"}'),
 ('coverage.ledger_window_days',      '90', 'integer', NULL, FALSE,
  '{"ar": "نافذة حجم الحركة لدفتر التغطية (أيام)", "en": "Volume window of the coverage ledger (days)"}'),
 ('coverage.exclusion_max_days',      '365', 'integer', NULL, TRUE,
  '{"ar": "أقصى مدة للاستثناء (أيام)", "en": "Longest term of an exclusion (days)"}'),
 ('coverage.exclusion_review_days',   '180', 'integer', NULL, TRUE,
  '{"ar": "أقصى مدة لمراجعة الاستثناء (أيام)", "en": "Longest review date of an exclusion (days)"}'),
 ('coverage.adjustment_max_days',     '90', 'integer', NULL, TRUE,
  '{"ar": "أقصى مدة للتسوية المعروفة (أيام)", "en": "Longest term of a known adjustment (days)"}'),
 ('dq.sla_days',                      '{"critical": 2, "high": 7, "medium": 14, "low": 30}', 'json', NULL, FALSE,
  '{"ar": "مهلة معالجة مشكلات البيانات حسب الخطورة (أيام)", "en": "Data-quality fix time by severity (days)"}'),
 ('schedule.recon_daily',             '"0 20 3 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مطابقة اليوم السابق", "en": "Daily reconciliation sweep schedule"}'),
 ('schedule.dq_expiry_sweep',         '"0 5 3 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول انتهاء الاستثناءات والتسويات المعروفة", "en": "Exclusion and known-adjustment expiry schedule"}')
ON CONFLICT (key) DO NOTHING;

-- Phase 11 table quarantine_items: phase 11 (0033) already grants akr_app SELECT and UPDATE of the status, owner and
-- exclusion columns, which is all the assign route and the conflict acceptance of this phase need. No grant here.

```

### 1.1 Inventory of section 1

| Table | Schema | Writers (grants) | Readers | Retention (clock) | Migration |
|-|-|-|-|-|-|
| `recon_runs` | admin | `akr_app` insert and status; `akr_ingest` sets `totals_captured_at` | `akr_app`, `akr_ingest`, `akr_readonly` | R9 (age, `created_at`) | 0036 |
| `recon_results` | admin | `akr_ingest` inserts expected rows; `akr_app` inserts receipt rows and updates evaluation columns | the same plus `akr_readonly` | R9 (age, `created_at`) | 0036 |
| `dq_issues` | admin | `akr_app` assigns, resolves, accepts, expires and upserts the recon breaks of `recon_run`; `akr_ingest` upserts counts | the same plus `akr_readonly` | R9 (age, `created_at`) | 0037 |
| `exclusions` | admin | `akr_app` inserts on activation and ends rows | `akr_readonly` | R9 (retirement) | 0038 |
| `coverage_ledger` | admin | `akr_app` (refresh job) | `akr_readonly` | R9 (retirement) | 0038 |
| `coverage_certificates` | admin | `akr_app` (job and attestation) | `akr_readonly` | R6 (age, `issued_at`) | 0038 |
| `quarantine_items` (phase 11) | admin | `akr_app` (owner and status; phase 11 grant) | `akr_readonly` | R1 (phase 11) | 0033 (phase 11) |

Every table is `id uuid primary key` with no database default, every timestamp is `timestamptz`, and no table has a
`DELETE` grant. The `[std]` columns are the convention of DATA-MODEL s.0; `version` is compared through `If-Match` on
the aggregates a steward or the MLRO changes (`dq_issues`, `coverage_certificates`, `quarantine_items`, `exclusions`).

## 2. DDD Implementation (Rust)

**Crate placement** (workspace pinned in phase 00; layer rules in `ddd.md`):

```
backend/
├── schemas/
│   ├── coverage-certificate.v1.json        # the sealed certificate document (section 3.8)
│   └── recon-control-totals.v1.json        # the shape ingest_control_totals writes (section 3.2)
├── crates/
│   ├── domain/    # + recon (ReconRun, RungEvaluator (pure), Tolerance, GroupKey), coverage (CoverageUnit,
│   │              #   DarkRule (pure), Exclusion, CertificateBuilder (pure), VolumeEffect), dq (DqIssue,
│   │              #   DataQualityRecorder grouping (pure), Quarantine transitions (pure))
│   ├── db/        # + Pg repositories for the six tables and the quarantine grants; PgControlTotalsSink
│   ├── authz/     # + resource adapters ReconRun, Certificate, DqIssue, Exclusion, CoverageUnit (Quarantine is phase 11's)
│   ├── jobs/      # + kinds recon_run, recon_sweep, coverage_ledger_refresh, coverage_certificate, dq_expiry_sweep
│   └── sources/   # the SourceSession used by ingest_control_totals (phase 05, unchanged)
├── api/           # + routers recon, coverage, dq, quarantine; change targets exclusion and known_adjustment
├── worker/        # + the five job handlers; CertificateSigner (OpenBao transit akr-sign-certificate)
└── ingestd/       # + ControlTotalsReader (ingest_control_totals), DataQualityRecorder (called by the load loop)
```

- **Domain entities** (`Debug, Serialize, Deserialize, FromRow, ToSchema`; fields mirror section 1): `ReconRun`
  (`status` as `#[sqlx(type_name = "admin.recon_status")]`), `ReconResult`, `DqIssue` (`admin.dq_status`),
  `CoverageUnit`, `Exclusion`, `CoverageCertificate` (`admin.certificate_status`), `QuarantineItem` (a view of the phase
  11 row; the columns it reads and writes are listed in section 1).
- **Value objects:** `Scope` (`Batch(Uuid)`, `Day(NaiveDate)`), `Rung` (`Receipt`, `Counts`, `ControlTotals`,
  `GlTieout`; ordered as listed), `Outcome` (`Reconciled`, `Open`, `ConfirmedGap`, `SourceAnomaly`), `GroupKey` (date,
  branch, currency and ledger; serialised `date|branch|currency|ledger` with the bank's own codes, never a name),
  `Tolerance` (non-negative decimal; `Tolerance::within(delta)` is `|delta| <= t`), `ConflictKind` (`FactChangedInSource`,
  `LabelChange`, `ValueChange`, `ParentMissing`; the mapping to `DqKind` is section 3.3), `DqKind`, `Severity` (ordered `Low` < `Critical`),
  `DqStatus`, `Resolution` (`Fixed`, `Explained`, `NotApplicable`, `Superseded`), `UnitKind` (`Branch`, `Product`,
  `TxnType`, `Channel`, `Feed`), `DarkReason` (`Monitored`, `Excluded`, `BlindPeriod`, `SourceStale`, `NoPostingsSince`),
  `CertStatus`, `Attestation { by, at, digest }`, `VolumeEffect` (`Increase`, `Neutral`, `Decrease`, `Unknown`; the phase
  04 enum), `QuarantineStatus` (`Open`, `Retried`, `Accepted`, `Excluded`).
- **Create and update DTOs:** `NewReconRun { scope, source_id, business_date, batch_id?, requested_by }`,
  `ExpectedResult { run_id, rung, object, group_key, expected | reason_code, source_captured_at }` (written by the ingest
  job), `ResultEvaluation { id, actual, tolerance, outcome, reason_code, detail }`, `NewCoverageUnit`, `NewExclusion`
  (from the activated change request only), `NewCertificate`, `DqIncrement { source_id, object, field, kind, severity,
  count, seen_at, run_id?, batch_id? }`, `DqAssign { owner_id }`, `DqResolve { resolution, note }`,
  `AcceptInput { accepted_until, signed_document_id, change_request_id }`, `QuarantineExclusion { reason }`.
- **Repository traits** (domain layer; `Pg*` in `db`; every write takes the caller's transaction):

```rust
#[async_trait]
pub trait ReconRepository: Send + Sync {
    async fn insert_run(&self, tx: &mut Tx, r: &NewReconRun) -> Result<ReconRun, RepoError>;   // uq active -> 409
    /// Conditional claim: the row moves to `running` only from `queued` or `awaiting_totals`; None when another worker has it.
    async fn claim(&self, tx: &mut Tx, id: Uuid) -> Result<Option<ReconRun>, RepoError>;
    async fn set_waiting(&self, tx: &mut Tx, id: Uuid) -> Result<(), RepoError>;                 // -> awaiting_totals
    async fn mark_totals_captured(&self, tx: &mut Tx, id: Uuid, at: DateTime<Utc>) -> Result<(), RepoError>;   // akr_ingest
    async fn insert_expected(&self, tx: &mut Tx, rows: &[ExpectedResult]) -> Result<u64, RepoError>;          // akr_ingest
    async fn insert_receipts(&self, tx: &mut Tx, rows: &[ReconResultRow]) -> Result<u64, RepoError>;
    async fn evaluate(&self, tx: &mut Tx, rows: &[ResultEvaluation], now: DateTime<Utc>) -> Result<u64, RepoError>;
    async fn complete(&self, tx: &mut Tx, id: Uuid, counts: &OutcomeCounts, open: i32) -> Result<ReconRun, RepoError>;
    async fn fail(&self, tx: &mut Tx, id: Uuid, code: ReconError) -> Result<ReconRun, RepoError>;
    async fn expected_for(&self, conn: &mut PgConnection, run_id: Uuid) -> Result<Vec<ReconResult>, RepoError>;
    async fn pending_days(&self, conn: &mut PgConnection, source_id: Uuid, since: NaiveDate) -> Result<Vec<PendingDay>, RepoError>;
    async fn stale_waiting(&self, conn: &mut PgConnection, before: DateTime<Utc>) -> Result<Vec<Uuid>, RepoError>;
    async fn list_runs(&self, conn: &mut PgConnection, f: &RunFilter, page: Keyset) -> Result<Page<ReconRun>, RepoError>;
    async fn get_run(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<ReconRun>, RepoError>;
}

#[async_trait]
pub trait CoverageRepository: Send + Sync {
    async fn upsert_units(&self, tx: &mut Tx, source_id: Uuid, units: &[NewCoverageUnit]) -> Result<UnitDiff, RepoError>;
    async fn active_exclusions(&self, conn: &mut PgConnection, source_id: Uuid) -> Result<Vec<Exclusion>, RepoError>;
    async fn insert_exclusion(&self, tx: &mut Tx, e: &NewExclusion) -> Result<Exclusion, RepoError>;   // uq active -> 409
    async fn end_exclusions(&self, tx: &mut Tx, ids: &[Uuid], reason: &str, now: DateTime<Utc>) -> Result<Vec<Uuid>, RepoError>;
    async fn list_exclusions(&self, conn: &mut PgConnection, f: &ExclusionFilter, page: Keyset) -> Result<Page<Exclusion>, RepoError>;
    async fn list_units(&self, conn: &mut PgConnection, f: &UnitFilter, page: Keyset) -> Result<Page<CoverageUnit>, RepoError>;
    async fn supersede_current(&self, tx: &mut Tx, source_id: Uuid, date: NaiveDate, at: DateTime<Utc>) -> Result<Option<Uuid>, RepoError>;
    async fn insert_certificate(&self, tx: &mut Tx, c: &NewCertificate) -> Result<CoverageCertificate, RepoError>;
    async fn attest(&self, tx: &mut Tx, id: Uuid, a: &Attestation, expected_version: i32) -> Result<CoverageCertificate, RepoError>;
    async fn certificates_for_date(&self, conn: &mut PgConnection, date: NaiveDate, source: Option<Uuid>) -> Result<Vec<CoverageCertificate>, RepoError>;
    async fn list_certificates(&self, conn: &mut PgConnection, f: &CertFilter, page: Keyset) -> Result<Page<CoverageCertificate>, RepoError>;
    async fn statement_inputs(&self, conn: &mut PgConnection, source: Option<Uuid>) -> Result<StatementInputs, RepoError>;
}

#[async_trait]
pub trait DqRepository: Send + Sync {
    /// INSERT ... ON CONFLICT (source_id, object, field, kind) WHERE status = 'open' DO UPDATE count and last_seen.
    async fn upsert_increments(&self, tx: &mut Tx, rows: &[DqIncrement]) -> Result<u64, RepoError>;
    async fn assign(&self, tx: &mut Tx, id: Uuid, a: &DqAssign, expected_version: i32) -> Result<DqIssue, RepoError>;
    async fn resolve(&self, tx: &mut Tx, id: Uuid, r: &DqResolve, by: Uuid, expected_version: i32) -> Result<DqIssue, RepoError>;
    async fn accept(&self, tx: &mut Tx, id: Uuid, a: &AcceptInput, at_version: i32) -> Result<DqIssue, RepoError>;   // CR activation
    async fn reopen_expired(&self, tx: &mut Tx, today: NaiveDate) -> Result<Vec<Uuid>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &DqFilter, page: Keyset) -> Result<Page<DqIssue>, RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<DqIssue>, RepoError>;
    async fn accepted_for(&self, conn: &mut PgConnection, source_id: Uuid, object: &str, field: &str, kind: DqKind) -> Result<Vec<DqIssue>, RepoError>;
}

// Phase 11's QuarantineRepository (section 2 there) gains two methods here:
//   async fn assign(&self, tx: &mut Tx, id: Uuid, owner: Uuid, expected_version: i32) -> Result<QuarantineItem, RepoError>;
//   async fn accept_conflicts(&self, tx: &mut Tx, source: Uuid, table: &str, reason: &str, up_to: DateTime<Utc>)
//       -> Result<u64, RepoError>;   // section 3.4: row_conflict rows close with their issue
// and its `QuarantineFilter` gains `owner_id`, `older_than_days` and the `oldest` order.
```

- **Repository notes:** keyset lists: runs by `(created_at DESC, id DESC)`; issues by `(severity rank DESC, due_at ASC,
  id)`, so the oldest critical item is first; quarantine by `(created_at ASC, id)`, oldest first; certificates by
  `(business_date DESC, source_id)`. `transition` and `assign` carry the version in their `WHERE`, so two stewards
  racing on one row cannot both succeed. `upsert_increments` never changes `status`, `owner_id` or `version`: a
  recurrence is not an edit a steward must re-read. `end_exclusions` and `reopen_expired` are the only bulk writes; each
  returns the ids it changed, one audit event each.

- **Ports:**

```rust
/// akr-ingestd only: the approved control-totals objects of a source, read as ProvenSelects through the LoadGovernor.
#[async_trait]
pub trait ControlTotalsReader: Send + Sync {
    async fn read(&self, session: &mut dyn SourceSession, source: &SourceRow, date: NaiveDate,
                  plan: &ControlTotalsPlan, limits: &StatementLimits) -> Result<ControlTotals, SourceError>;
}
pub struct ControlTotals { pub counts: Vec<CountRow>, pub sums: Vec<SumRow>, pub ledger: Vec<SumRow>, pub missing: Vec<String> }

/// akr-worker: signs the SHA-256 of the canonical certificate with OpenBao transit `akr-sign-certificate` (phase 00).
#[async_trait]
pub trait CertificateSigner: Send + Sync {
    async fn sign(&self, digest: &[u8; 32]) -> Result<Signature, CryptoError>;   // Signature { text, key_version }
}

/// Phase 11: the freshness snapshot the ledger, the certificate and the statement read (C-29).
pub trait FreshnessService: Send + Sync {   // phase 11 section 3.7; the snapshot this phase reads
    fn data_as_of(&self, source_id: Uuid) -> DataAsOf;   // per branch business date, newest posting, stale flag
}
```

- **utoipa schemas:** `ReconRunCreate` (`source_id`, `business_date`, `scope`, `batch_id?`), `ReconRunAccepted`
  (`run_id`, `Location`), `ReconRunView` (status, totals captured, outcome counts, open results, error code, `data_as_of`),
  `ReconResultView` (rung, object, group key, expected, actual, delta, tolerance, outcome word, reason code), `CertificateView`
  (status, revision, summary, gaps, `data_as_of`, `stale`, signature, key version, attestation fields; the sealed document
  is fetched through the evidence route of phase 18), `CoverageStatementView` (section 3.9), `CoverageUnitView`,
  `ExclusionProposal` (`source_id`, `unit`, `unit_ref`, `reason_code`, `reason`, `owner_id`, `review_due`, `expires_at`),
  `ExclusionView`, `DqIssueView` (no value of any record: kind, object, field,
  count, dates, age in days, owner name, status), `DqAssignInput` (`owner_id`), `DqResolveInput` (`resolution`, `note`),
  `KnownAdjustmentProposal` (`dq_issue_id`, `accepted_until`, `reason`, `signed_document_id`), `KnownAdjustmentView`,
  `QuarantineView` (id, source table, reason code, gate, status, owner, `age_days`, `detail_keys` as names only, batch id),
  `QuarantineAssignInput`, `QuarantineExclusionInput` (`reason`). Every list envelope carries `next_cursor`
  and `data_as_of`; every response of a mutation carries `affordances` (API.md section 1).

- **Route table** (base `/api/v1`; every route names its Cedar action, section 3.12; **SU** = step-up assertion; `If-Match`
  where the row is versioned; every POST takes `Idempotency-Key`):

| Method | Path | Scope | Description |
|-|-|-|-|
| POST | /recon/runs | data.steward | Queue a ladder run for one source and business date (day) or one batch; 202 (`recon.run`) |
| GET | /recon/runs | data.steward, dashboard.read | Runs, newest first; `source_id`, `business_date?`, `status?` (`recon.read`) |
| GET | /recon/runs/{id} | data.steward, dashboard.read | Run status and every rung result: outcome word, expected, actual, delta, tolerance, reason (`recon.read`) |
| GET | /recon/certificates | dashboard.read, data.steward | Current and superseded certificates; `source_id?`, `status?` (`certificate.read`) |
| GET | /recon/certificates/{business_date} | dashboard.read, data.steward | Certificates of one date, one per source; `source_id?`; superseded revisions with `superseded_at` (`certificate.read`) |
| POST | /recon/certificates/{business_date}/attest | dashboard.read | MLRO attests a pending certificate of one source (`source_id` in the body); **SU** class `signing`; `If-Match` (`certificate.attest`) |
| GET | /coverage/statement | dashboard.read | MLRO coverage statement: covered share, blind periods, exclusions in force, open high findings, `data_as_of` (`coverage.statement`) |
| GET | /coverage/ledger | data.steward, dashboard.read | Coverage ledger units: covered word, reason, since, volume and share; `source_id`, `covered?` (`coverage.read`) |
| GET | /coverage/exclusions | data.steward, dashboard.read | Exclusion register: unit, reason, owner, approver, review date, expiry, ended (`exclusion.read`) |
| POST | /coverage/exclusions | data.steward | Propose an exclusion; creates change request `exclusion` (material); 202 with the CR id (`exclusion.propose`) |
| GET | /dq/issues | data.steward | Data-quality issues: `source_id?`, `kind?`, `severity?`, `status?`, `owner_id?`, `overdue?` (`dq_issue.read`) |
| POST | /dq/issues/{id}/assign | data.steward | Set the owner (an active steward); `If-Match` (`dq_issue.assign`) |
| POST | /dq/issues/{id}/resolve | data.steward | Resolve with a resolution and a note; `If-Match` (`dq_issue.resolve`) |
| GET | /dq/known-adjustments | data.steward, dashboard.read | Known adjustments in force: accepted issues with expiry, document and CR (`known_adjustment.read`) |
| POST | /dq/known-adjustments | data.steward | Propose a known adjustment for one open issue; creates change request `known_adjustment`; 202 (`known_adjustment.propose`) |
| POST | /quarantine/{id}/assign | data.steward | Set the owner of a quarantined row; `If-Match` (`quarantine.assign`) |

Phase 11 owns `GET /quarantine`, `POST /quarantine/{id}/retry` and `POST /quarantine/{id}/exclude` (DR-93, DR-97). This
phase adds the assign route above and extends two of phase 11's: the list gains the filters `owner_id` and
`older_than_days` and the order `oldest` (T4), and the exclusion gains two refusals (section 3.11).

- **Router functions:** `recon::router()` (6 routes), `coverage::router()` (4), `dq::router()` (5), `quarantine::router()`
  (4), each `-> OpenApiRouter<AppState>`, merged in `build_app` under the authz layer. Phase 04's `POST /change-requests`
  refuses the kinds `exclusion` and `known_adjustment` (they have dedicated routes, phase 04 section 1), so the only way in
  is the two proposal routes above.

## 3. Business Logic

### 3.1 The reconciliation ladder (`ReconService`, `RungEvaluator`; C-25, L-04)

**Rungs.** The ladder reads four rungs, the `recon_results.rung` enum of DATA-MODEL s.6. Receipt (L0 of C-25): every batch
of the business date is closed and mapped. Counts (L1): rows the source holds for the date per object equal rows landed
for it. Control totals (L1, sums): signed movement per branch and currency, source against canon. GL tie-out (L3): the
source's own ledger movement per branch, currency and ledger against the posting legs in canon. A batch-scope run covers
the receipt and the counts of one batch. Rungs L2 (account continuity), L4 (payment tie-out), L5 (sequence gaps) and L6
(cross-system) of C-25 are not in this phase (Open doubts).

**Outcomes** (C-25's four): `reconciled` (within tolerance), `open` (a difference or a batch not yet closed, with a
reason code and an issue), `confirmed_gap` (a difference explained by an accepted known adjustment in force), and
`source_anomaly` (the source side is missing or cannot be read: no control totals, a refused read, an object outside
the approved list).

**Signed movement (the sum rungs):** a credit adds `amount`, a debit subtracts it; `status = 'posted'` only, so a
cancelled posting is excluded from both sides. The source-side sum uses the same sign convention, applied in the
statement of section 3.2.

**`POST /recon/runs`** (akr-api; Cedar `recon.run`; `Idempotency-Key`):
1. Validate: `source_id` exists and is `active` (else 409 `E_NOT_ALLOWED_IN_STATE`, reason
   `source_not_active`); `business_date` is not after the bank date today; `scope` is `day` or `batch`; a batch run names
   a `batch_id` of that source with status `mapped`, `partial` or `rejected` (else 409 reason `batch_not_closed`); a day
   run names a date that has at least one batch (else 409 reason `no_batches`).
2. Insert `recon_runs` (status `queued`, `requested_by` the actor). A second active day run for the same source and date
   fails the partial unique index: 409 `E_NOT_ALLOWED_IN_STATE` reason `run_active` with the active run id in `errors[]`.
3. Enqueue `recon_run {run_id}` (coalesce key `recon_run:<run_id>`); audit `recon.requested`; 202 with `Location`.

**`recon_sweep`** (akr-worker, scheduled `schedule.recon_daily`, concurrency 1; section 4): for each source in state
`active`, for each business date in the last `recon.lookback_days` working days (bank calendar, phase 09) that has at
least one batch and no active day run:
1. Insert a day run when the newest batch of the date is newer than the finished time of the latest completed day run for
   that date (a late batch, C-22) or no completed day run exists. A date whose batches are not all closed is skipped until
   they are.
2. A run in `awaiting_totals` whose `created_at` is older than `recon.totals_max_wait_h` is failed with
   `totals_unavailable` (audit `recon.failed`).
3. Audit `recon.swept {source, dates_queued, dates_skipped_open}`; no per-date event.

**`recon_run`** (akr-worker; concurrency 2; max attempts 3; coalesce `recon_run:<run_id>`):
1. `claim`: move the run to `running` from `queued` or `awaiting_totals`, set `started_at`. Zero rows (another worker,
   or a finished run) ends the job.
2. Receipt rung (both scopes). For each batch of the scope, one result with `object = batch:<id>` and `group_key = ''`:
   `mapped` -> `reconciled`; `partial` -> `open`, `batch_partial`; `landed` -> `open`, `batch_not_mapped`; `rejected` ->
   `open`, `batch_rejected`. An empty scope -> `source_anomaly`, `no_batches`. `within_tolerance` is true for `reconciled`.
3. Either scope, when `totals_captured_at` is null: if the run has no expected rows yet, `set_waiting` (status
   `awaiting_totals`) and enqueue `ingest_control_totals {run_id}` (coalesce `ingest_control_totals:<run_id>`); end the job.
   The ingest job sets `totals_captured_at` and enqueues `recon_run` again, so the next attempt continues from step 4.
4. Counts rung. The units of the run are the batch receipts of `admin.batches` (phase 11, one per source table and
   branch over a business-date range `business_from..business_to`): a batch run has its one unit; a day run has every unit
   whose `business_to` is the run's business date. For each unit, the expected row (object `source_table`, group key
   `branch|business_from|business_to`) holds the source's count over that range (step 2 of section 3.2), and `actual` is
   `control_totals.read` of the receipt, the rows phase 11 read from the source for the unit. `expected` null
   (`control_totals_unavailable`) -> `source_anomaly`; equal -> `reconciled`; different -> `open`, `count_mismatch`.
5. Control totals rung (day scope only): for each expected row of rung `control_totals` (group `date|branch|currency`), `actual` is the signed
   sum over `canon.txn` of the source for the business date and that group (`booking_date` range of the source
   business date, DR-39). Missing on either side: `source_anomaly` with `group_missing_on_source`, or `open` with
   `group_missing_in_canon`. Tolerance `recon.control_total_tolerance`.
6. GL tie-out rung (day scope only): the same over the ledger movement groups (`date|branch|currency|ledger`) against `canon.txn.ledger_code`.
   Tolerance `recon.gl_tolerance`, read once per run and stored on every result (`tolerance`), so a later change never
   rewrites an old result.
7. Evaluate every result: `delta = expected - actual`; `within_tolerance = Tolerance::within(delta)`; outcome `reconciled`
   when within; otherwise `confirmed_gap` when `accepted_for(source, <rung>:<object>, <field>, kind = recon_break)` (the field of the result, block 7.5) returns an accepted issue
   for the same object and group, else `open`. `source_anomaly` rows keep their reason and no delta.
8. Every `open` or `source_anomaly` result upserts one `dq_issues` row (kind `recon_break`, object `<rung>:<object>`,
   field = the group key, `run_id`, severity by rung: `gl_tieout` critical, `control_totals` high, `counts` and
   `receipt` medium; a `source_anomaly` on the GL rung is high). The upsert is the one of section 3.3.
9. One transaction: `evaluate`, the issue upserts of step 8 (their audit event is this step's `recon.completed`, whose `open` and `source_anomaly` counts per rung are the issues written), `complete` with `outcome_counts` (rung by outcome) and `open_results`, audit
   `recon.completed {run_id, rungs: outcome counts}`. Then enqueue `coverage_ledger_refresh {source_id}` (coalesce by
   source) and, for a day run, `coverage_certificate {source_id, business_date, run_id}` (section 3.8).
10. Any error after the last attempt: `fail(internal)` (or `source_paused` when the governor reports a pause, which ends
    without a retry); audit `recon.failed {run_id, error_code}`. Nothing half-evaluated is kept: step 9 is one transaction.

**Break handling.** A break is never netted off, closed silently or re-scored by hand: it stays `open` until a person
resolves its issue (section 3.4) and the next day run re-checks the same group. A resolved issue whose break still shows
on the next run opens a new issue (the partial unique index only covers `open` rows); the old one keeps its resolution.

### 3.2 Source control totals (`ControlTotalsReader`, job `ingest_control_totals`, akr-ingestd; C-25, L-04, C-35)

The only CBS read of this phase, and it is an aggregate read. The source's control-totals objects are named in the adapter
pack (`discovery.json`, phase 05, data not code) and approved by change request (`source_access`, phase 05 DR-62); an
object not on the approved list is never read (`E_OBJECT_NOT_APPROVED`).

1. Load the run. `totals_captured_at` set -> end. Scope must be `day`.
2. Build the plan from the units of the run (section 3.1 step 4): for each unit, one `count(*)` over its `business_from..
   business_to` range and branch on the approved counts object (`index_range` method, DR-40). A day run adds, for each approved
   sum object, one grouped `SUM` per date by branch and currency, and for the ledger object one grouped `SUM` per date by
   branch, currency and ledger. Each statement is generated, rendered and proven by
   `SelectProof` (phase 05, now in `sqlproof`); a statement that fails proof is refused and its rung is `source_anomaly`.
3. Each statement runs through the `LoadGovernor` (`StatementKind::ControlTotals`): the session semaphore, the row-rate
   bucket and the blackout windows of the budget apply. A `Blackout` or `OutsideWindow` answer reschedules the job at
   `until` or `next` without consuming an attempt (phase 05 section 3.5). A `Paused` answer ends the job with
   `source_paused`.
4. In one transaction, insert `recon_results` expected rows: rung `counts` per unit (`expected` = the count, `group_key` =
   `branch|business_from|business_to`), rung `control_totals` and `gl_tieout` per group (`expected` = the signed sum). An object missing from the plan
   or refused writes a row with `reason_code = control_totals_unavailable` and `expected` null, so the worker can report
   `source_anomaly`. `source_captured_at = now()`.
5. Set `recon_runs.totals_captured_at = now()` (grant to `akr_ingest`), commit, then enqueue `recon_run {run_id}`.
6. Audit `recon.totals_captured {run_id, objects, groups, missing}` (counts only, no values); emit the writer's ingest
   metric `akr_recon_totals_seconds`.

The job never reads a row (only aggregates), never a customer or account key, and never writes to the source. Its
statement list appears in the statement journal of phase 05, which the verification of section 6 checks.

### 3.3 Recording conflicts and validation failures (`DataQualityRecorder`, akr-ingestd library; C-24, C-28, DSL.md B.5)

**Where conflicts go** (DR-97). The writer's `RowConflict`s (phase 10 section 3.7) are kept at two levels. Phase 11
writes one `quarantine_items` row of kind `row_conflict` per conflicting row (codes, column names and the row hash, no
value; phase 11 section 3.2 step 5): that is the row-level evidence a steward can open. This recorder, called by the same
load loop in the same transaction, counts the same conflicts, the row failures and the gate counts into one open
`dq_issues` row per `(source, object, field, kind)`: that is the register with owners, severities and ages. A conflict
is a mapped row that disagrees with a settled one, so it is never retried; it is closed when its issue is resolved
(section 3.4). One kind is the exception: a `parent_missing` row disagrees with nothing (its parent has not landed yet),
and the trailing re-read window of phase 10 section 7.20 meets it again.

Called by phase 11's load loop inside the batch transaction, after `CanonicalWriter::write` returns its `WriteOutcome`, so a
rolled-back batch leaves no issue behind. Signature:
`DataQualityRecorder::record_batch(tx, source_id, batch_id, run_id?, inputs: &BatchDqInputs) -> Result<u64, DqError>`,
where `BatchDqInputs` carries the `conflicts: Vec<RowConflict>` of the writer, the rule failures of DSL.md B.5 with
`on_fail: dq_issue`, the unmapped codes (phase 10 `unknown` plus issue), the unresolved pointers, and any card-guard stop.

1. Map each `RowConflict` to a kind, with the reason names phase 11 writes on its quarantine rows: a fact conflict
   (`fact_changed_in_source`, a settled fact whose source hash changed; the new version stays in `raw`) ->
   `row_conflict_fact` (high); `label_change` -> `row_conflict_label` (low); `value_change` -> `row_conflict_value`
   (medium); `parent_missing` (a row whose parent key has not landed, phase 10 sections 7.20 and 7.27) ->
   `unresolved_pointer` (medium). Rule failures -> `validation_rule`, with `field` the attribute name (severity from the rule's `severity`, default
   medium). Unmapped code -> `unmapped_code`, `field` = the code map kind (`txn_type`, `currency`, ...; never the code
   value). Unresolved pointer -> `unresolved_pointer` (medium). A card-guard stop -> `card_guard` (critical), `field` the
   column name.
2. Aggregate in memory by `(object, field, kind)`: one `DqIncrement` per group, with the count and the first and last
   `seen_at` of the batch. No value, key or row is ever in the increment; the `detail` carries codes and counts.
3. Upsert every increment: `INSERT ... ON CONFLICT (source_id, object, field, kind) WHERE status = 'open' DO UPDATE SET
   count = dq_issues.count + EXCLUDED.count, last_seen = GREATEST(...), updated_at = now()`. A new row gets `due_at =
   first_seen + dq.sla_days[severity]` and `batch_id`. An existing open row keeps its owner, status and version.
4. Return the number of rows written; metric `akr_dq_recorded_total{kind}`. Audit: one `dq.recorded {source, batch_id,
   kinds: {kind: count}}` per batch with issues (counts only); a batch with no issue writes no event.

The writer's conflict set is returned, not logged: the recorder is the only place it becomes data-quality state. A settled
fact that changed at source stays in `raw` (phase 11) and opens a `row_conflict_fact` issue; the canon row is never
overwritten (DATA-MODEL s.0).

### 3.4 The data-quality register and its expiry (`DataQualityService`, job `dq_expiry_sweep`; C-28, L-03, L-05)

**`POST /dq/issues/{id}/assign`** (Cedar `dq_issue.assign`; `If-Match`):
1. Load the issue (404 when missing). Status must be `open` (else 409 `E_NOT_ALLOWED_IN_STATE`, reason `not_open`).
2. `owner_id` must be an active human holding the data steward role (else 422 `E_VALIDATION`, reason `owner_not_active`).
3. `assign` with the version; audit `dq_issue.assigned {issue_id, owner_id, kind, severity}`.

**`POST /dq/issues/{id}/resolve`** (Cedar `dq_issue.resolve`; `If-Match`):
1. Status must be `open` (else 409 reason `not_open`). `resolution` in the enum; `note` 10 to 2,000 characters for `explained`
   and `not_applicable`, optional for `fixed`.
2. `resolve` sets `resolved_at`, `resolved_by`, `resolution` and the note, and moves the row to `resolved`. For a
   `row_conflict_*` issue, the same transaction moves the open `row_conflict` quarantine rows of that source, table and
   reason created up to the issue's `last_seen` to `accepted` (phase 11 grant on `status`), so the row-level evidence
   closes with its issue. Audit `dq_issue.resolved {issue_id, kind, resolution, quarantine_rows_closed}`; the note is
   never in the audit detail (tipping-off: the note stays in the row, classified `personal`).
3. A `fixed` resolution is trusted until the next day run re-checks the group (section 3.1 break handling).

**`dq_expiry_sweep`** (akr-worker, scheduled `schedule.dq_expiry_sweep`, concurrency 1, max attempts 3):
1. `reopen_expired(today)`: every `accepted` issue whose `accepted_until` is before today returns to `open` with
   `accepted_until` and the accepted state cleared (the signed document and CR stay as history). Audit
   `dq_issue.adjustment_expired {issue_id, kind}` each.
2. `end_exclusions` for every active exclusion with `expires_at <= now()` (section 3.6 step 6). Audit `exclusion.expired`
   each, and enqueue `coverage_ledger_refresh` for each affected source.
3. Gauge `akr_dq_overdue{severity}` is computed from `due_at` at scrape time; the sweep writes no event for overdue issues.

**Owners and age (T4).** Age is `now - first_seen`; `overdue` is `due_at < now()` on an open issue. Both are computed in
the query, never stored, so a steward cannot age an issue by editing it.

### 3.5 Known adjustments (change target `known_adjustment`; C-28, L-03)

`POST /dq/known-adjustments` (akr-api; Cedar `known_adjustment.propose`; `Idempotency-Key`):
1. The issue exists and is `open`; its kind is not `card_guard` (else 422 `E_VALIDATION`, reason `kind_not_adjustable`).
2. `accepted_until` is after today (bank time) and at most today plus `coverage.adjustment_max_days` (else 422 reason
   `review_too_far` or `review_in_past`).
3. `signed_document_id` is a document of this source that was staged and scanned (phase 05 `POST /sources/{id}/documents`)
   and is signed by the bank, that is of kind `exception_signed` (phase 05 `SourceDocumentKind`; else 422 reason `document_required`). `reason` 10 to 2,000 characters.
4. `ChangeRequestService::create(origin = Dedicated)`, which continues with `submit` in the same transaction (phase 04 section 3.2), with object kind `known_adjustment`, object key `dq_issue:<id>`, the proposal in the
   body. Class: the target's base class `material` (an acceptance hides a break from the ladder's open count); route
   `cr_material` by the existing class rule of phase 04 (checker: the MLRO and CHK). 202 with the CR id and `Location`.

`ChangeTarget` `known_adjustment` (phase 04 protocol):
- `prepare`: the issue, its version and the proposal; `volume_effect = neutral` (no monitored row changes); `base_class = material`;
  `emergency_allowed = false`.
- `current_version`: the issue's `version`. `check_before_decision`: the issue is still `open` and not a `card_guard` (else
  `failed` with a reason; phase 04 `E_STALE_BASE` when the version moved).
- `activate` (in the final approval's transaction, DR-58): `accept(issue, input, at_version)`; the row becomes `accepted`
  with `accepted_until`, `signed_document_id` and `change_request_id`. Audit `known_adjustment.activated {issue_id,
  accepted_until, change_request_id}`. The next day run finds the matching break as `confirmed_gap` (section 3.1 step 7).
- `revert`: not offered (`emergency_allowed = false`). `diff`: the proposal against the current row.

### 3.6 Exclusions (change target `exclusion`; L-02, phase 04 protocol)

**`POST /coverage/exclusions`** (akr-api; Cedar `exclusion.propose`; `Idempotency-Key`; 202):
1. Validate: `unit` in the enum; the unit exists in `coverage_ledger` for the source (else 422 `E_VALIDATION`, reason
   `unknown_unit`); no live exclusion for the unit (else 409 `E_NOT_ALLOWED_IN_STATE`, reason `exclusion_active`); the
   `reason_code` is an active code of family `exclusion` (phase 09, DR-82; else 422 reason `reason_code_inactive`);
   `reason` 10 to 2,000 characters; `owner_id` an active human steward.
2. Term: `expires_at` after now and at most `coverage.exclusion_max_days` ahead (bank time); `review_due` after today,
   at most `coverage.exclusion_review_days` ahead, and not after `expires_at` (else 422 `E_VALIDATION`, reasons
   `term_too_long`, `review_too_far`, `review_before_expiry`).
3. `ChangeRequestService::create(origin = Dedicated)`, which continues with `submit` in the same transaction (phase 04 section 3.2), with object kind `exclusion`, object key `exclusion:<source>:<unit>:<unit_ref>`. The
   generic `POST /change-requests` refuses this kind (phase 04 dedicated routes).

`ChangeTarget` `exclusion`:
- `prepare`: measures the unit over `coverage.ledger_window_days` from `canon.txn` (count and LCY value of the unit, and the
  total of its unit kind for the source) and writes `impact` = `{unit, unit_ref, share_count, share_value, window_days}`.
  `volume_effect`: `unknown` when the window cannot be measured (the source has no canonical rows in the window, or the
  source is not `active`); `neutral` when the unit's window volume is zero (a dormant branch: excluding it removes nothing
  from monitoring); `decrease` otherwise. `base_class = material` (the lead's rule for exclusions); the route is then
  `cr_material`, and the `decrease` branch adds the CH approval (phase 04 `cr_material` definition).
- `current_version`: none (an exclusion has no prior version). `check_before_decision`: the unit still has no live
  exclusion (else `failed`); the approver is not `owner_id` (else 403 `E_SELF_APPROVAL` on the decision, the owner cannot
  decide their own exclusion, the database CHECK `approved_by <> owner_id` is the last line).
- `activate` (the final approval's transaction): insert the `exclusions` row (`approved_by` the last decider,
  `volume_effect_snapshot` = `impact`, `expires_at`, `review_due`, `change_request_id`); enqueue
  `coverage_ledger_refresh {source_id}`. Audit `exclusion.activated {unit, unit_ref, reason_code, expires_at, approved_by}`.
- `revert`: not offered. `diff`: the proposal and the impact.

**Expiry.** `dq_expiry_sweep` ends an exclusion at `expires_at` with `ended_reason = expired` and an audit event; the
ledger then recomputes the unit (section 3.7). An early withdrawal is not in this phase (Open doubts). A new exclusion for
the same unit waits until the old one has ended (`uq_exclusions_active`).

**Why an exclusion is never a silent gap.** The ledger shows the unit as `excluded` with the exclusion id; the certificate
of each day lists it under gaps (section 3.8); the statement counts it; an ended exclusion is kept with its end reason.

### 3.7 The coverage ledger (`CoverageLedger`, job `coverage_ledger_refresh`; L-01, C-29)

Payload `{source_id}`; coalesce key `coverage_ledger_refresh:<source_id>`; concurrency 1; max attempts 3.

1. Read the freshness snapshot of the source (`FreshnessService::data_as_of`, phase 11 section 3.7): business date per branch, newest
   posting, `stale`. The `as_of_business_date` of every row is the source's newest business date (DR-39), never the clock.
2. Units: the canonical masters of the source (`canon.branch`, `canon.product`, `canon.channel`), the active canonical
   codes of the `txn_type` code map (phase 10), and every unit seen in `canon.txn` in the window. A unit is one row per
   `(source, unit, unit_ref)`; the `feed` unit is the source itself.
3. Last posting: `MAX(booking_date)` per unit over a lookback of four windows (`4 x coverage.ledger_window_days`); none
   found -> `since = NULL`.
4. Volume over `coverage.ledger_window_days`: `count(*)` and the LCY sum per unit. `share_count` and `share_value` are the
   unit's share of the total of its unit kind in the window (0 to 1, six decimals); `volume_basis = none` and NULL volumes
   when the source has no rows in the window.
5. Reason, in this precedence (the first that holds): `excluded` (a live exclusion for the unit; `exclusion_id` set),
   `blind_period` (a blind period of the source or the system covers the as-of date), `source_stale` (`stale` is true),
   `no_postings_since` (the last posting is more than `coverage.dark_unit_days` working days before the as-of date, or
   none exists), otherwise `monitored`. `covered = (reason = monitored)`.
6. Upsert every unit on `(source_id, unit, unit_ref)`; the row's `version` moves only when `covered`, `reason`, `since`
   or the exclusion changed.
7. Audit `coverage.unit_dark {unit, unit_ref, reason, since}` on a covered-to-dark transition, `coverage.unit_recovered` on
   the reverse, and one `coverage.ledger_refreshed {source_id, units, dark, excluded}` per run. Gauge `akr_dark_units{source}`.

A dark unit is shown as dark with its reason and date (DATA-MODEL s.6: "never a quiet day"). The working-day count uses the
phase 09 bank calendar, so a holiday is not counted as a day without postings.

### 3.8 Coverage certificates (`CertificateBuilder`, job `coverage_certificate`, attestation; C-26, C-27, C-29)

`coverage_certificate {source_id, business_date, run_id}` (akr-worker; coalesce key as in section 4; concurrency 1; max
attempts 5, because the signer may be retried):
1. Load the run. Not `completed`, or not the newest completed day run of `(source, business_date)` -> end (a later run has
   superseded it before this job ran; that run enqueued its own job).
2. Gather the inputs of this run: its results and their outcomes; the open `high` and `critical` issues with this `run_id`;
   the live exclusions at this moment (their ids); the dark units of the source; the blind periods covering the date; the
   accepted issues in force (confirmed gaps); the freshness snapshot (`data_as_of`, `stale`).
3. Build `gaps`: one item per `open_result`, `source_anomaly`, `confirmed_gap`, `open_issue`, `dark_unit`, `exclusion`,
   `blind_period` and a `stale` item when the freshness snapshot is stale. Each item carries its kind, object, rung or
   unit, group key, reason, `since` where known, and the id of the row it came from. Sorted by kind, object, group key and
   id, so the digest is stable.
4. Status: `issued` when `gaps` is empty, otherwise `pending_attestation` (a declared gap waits for the MLRO's attestation below).
   The certificate of a clean day is final: no human step is asked for where there is nothing to declare.
5. `summary`: outcome counts per rung; open issue counts by severity; the covered share by count and by value (monitored
   branch units over all branch units of the source, from the ledger).
6. Canonical JSON of the document (`coverage-certificate.v1`: source code, business date, revision, run id, `generated_at`
   in UTC, status, summary, gaps, `data_as_of`, `stale`), validated against its schema; `document_sha256` = SHA-256 of
   those bytes.
7. `CertificateSigner::sign(document_sha256)` (transit `akr-sign-certificate`, ed25519, phase 00). Outside any database
   transaction: a network call is never held inside one.
8. One transaction: `revision` = the highest revision of `(source, business_date)` plus one (1 for the first certificate,
   which has no predecessor to require); `supersede_current` sets `superseded_at` on the current row; `put_generated`
   stores the sealed document (phase 02, owner kind `coverage_certificate`, owner id the new certificate id, R6,
   classification `detection-logic`); `insert_certificate` writes the row with `signature`, `key_version`, `issued_at`.
9. Audit `certificate.issued {source_id, business_date, revision, status, gaps: n, key_version}`.

**Attestation** (`POST /recon/certificates/{business_date}/attest`, body `{source_id}`; Cedar `certificate.attest`, which
admits the MLRO only; **SU** class `signing`):
1. Consume a single-use step-up assertion of class `signing` (phase 01 `StepUpService::consume`); missing, expired,
   reused or wrong-class -> 401 `E_STEPUP_REQUIRED`. `If-Match` is required (428 `E_PRECONDITION_REQUIRED`).
2. Load the current certificate of `(source, business_date)` (`superseded_at` null). None -> 404 `E_NOT_FOUND`. Status
   not `pending_attestation` -> 409 `E_NOT_ALLOWED_IN_STATE`, reason `not_pending`. Version mismatch -> 409
   `E_VERSION_CONFLICT`. A revision issued since the MLRO read the page moves the version, so an attestation can only apply
   to the gaps the MLRO saw.
3. `attestation_digest` = SHA-256 of the canonical JSON of `gaps`; `attest` sets `attested_by`, `attested_at`, the digest,
   status `attested`, version plus one.
4. Audit `certificate.attested {source_id, business_date, revision, gaps: n}`; 200 with the view.

The attestation is a recorded human signature (the row and the audit chain), not a second transit signature (Open
doubts). A certificate revised after an attestation starts again at `issued` or `pending_attestation`.

### 3.9 The coverage statement (`CoverageStatementService`; C-27, C-29)

`GET /coverage/statement` (akr-api; Cedar `coverage.statement`; `?source_id=` optional). Read-only, computed from the
ledger, the certificates, the blind periods and the exclusions; no `canon` row is returned.
1. Per source: the freshness snapshot (`data_as_of`, `stale`) and the current certificate's status, with
   `pending_attestation` flagged.
2. Transactions covered: the count and the LCY value of postings in the window whose branch unit is `monitored`, over all
   postings of the window, with the share. Accounts covered: distinct accounts with a posting in the window in a monitored
   branch, over all distinct accounts of the window (`share_count`).
3. Blind periods overlapping the window (scope, cause, start, end and the comment).
4. Exclusions in force: count, unit kinds, nearest expiry. An expired exclusion that is not yet ended is impossible after
   the sweep; if it appears, the statement reports `expired_unended` as a finding.
5. Open `high` and `critical` issues: count and the oldest age.
6. `data_as_of` and `stale` on the response and `X-Data-As-Of`; the screen shows the stale state beside every figure (C-29).

### 3.10 Blind periods (read here; the table, causes and routes are phase 11's, section 3.6)

`admin.blind_periods`, its causes, its scopes, its automatic openings and its two routes (`GET` and `POST
/coverage/blind-periods`) belong to phase 11. This phase adds no route, no cause and no grant for them. It reads the table
in three places: the ledger (a unit covered by a period of the source or of the system at the as-of date is dark with the
reason `blind_period`, section 3.7); the certificate (each period covering the date is a gap of kind `blind_period`,
section 3.8); and the statement (the periods overlapping the window, section 3.9).

A period declared after the certificate of its day was issued shows at once in the statement and on the next ledger
refresh. The certificate of that day is revised only when a new day run completes (section 3.1, late data), so a
declared period is never back-filled into an issued certificate.

### 3.11 The quarantine queue (`QuarantineService`; L-05; the table, the list, retry and exclude routes and the retry job are phase 11's)

T4 lists the rows of `admin.quarantine_items` through phase 11's `GET /quarantine`: `row_failed` rows (a landed row that
did not map) by default, and `row_conflict` rows under a filter, each linked to its data-quality issue.

- **List extension** (phase 11 route, Cedar `quarantine.read`): filters `owner_id` and `older_than_days`; order
  `oldest` (`created_at ASC, id`) beside phase 11's newest-first default; `age_days` computed; `detail_keys` are the
  names of the keys of `detail`, never their values.
- `POST /quarantine/{id}/assign` (Cedar `quarantine.assign`; `If-Match`): the owner must be an active steward; the row must
  be `open` or `retried` (else 409 `E_NOT_ALLOWED_IN_STATE`, reason `quarantine_not_open`). Audit `quarantine.assigned {id,
  owner_id, reason_code}`.
- **Exclusion refusals** (phase 11's `POST /quarantine/{id}/exclude`, extended): 409 `E_NOT_ALLOWED_IN_STATE` with reason
  `gate_not_excludable` for a row whose reason is a transaction type that is not mapped (the code must be mapped first;
  DSL.md B.7), and `card_guard_not_excludable` for a row whose `reason_code` is `card_guard`, the phase 10 row failure that phase 11 writes as a `row_failed` row (card data is fixed at source). An excluded row stays
  counted as landed in the counts rung and appears in the statement by reason.
- **Retry** stays phase 11's: the route enqueues `ingest_map_pending {quarantine_ids}`; success moves the row to
  `accepted`, a new failure leaves it `retried` with a new `row_failed` row (phase 11 section 3.3). A `row_conflict` row
  is never retried (409 `conflict_needs_review`); it closes with its issue (section 3.4).

### 3.12 Errors, audit events, Cedar actions, tipping-off review

**Error mapping.** No new problem codes. Used: `E_VALIDATION` (422, `errors[]` with reasons `review_too_far`,
`review_in_past`, `review_before_expiry`, `term_too_long`, `owner_not_active`, `document_required`, `kind_not_adjustable`,
`unknown_unit`, `reason_code_inactive`); `E_NOT_ALLOWED_IN_STATE`
(409, reasons `source_not_active`, `batch_not_closed`, `no_batches`, `run_active`, `not_open`, `exclusion_active`,
`not_pending`, `gate_not_excludable`, `card_guard_not_excludable`, `conflict_needs_review`, `not_open`); `E_VERSION_CONFLICT` and
`E_PRECONDITION_REQUIRED` (versioned rows); `E_STEPUP_REQUIRED` (attestation); `E_SELF_APPROVAL` (the owner deciding their
own exclusion); `E_STALE_BASE` (phase 04, a known adjustment whose issue moved); `E_OBJECT_NOT_APPROVED` (control totals
read, recorded as `control_totals_unavailable`, never an HTTP answer); `E_FORBIDDEN`, `E_SCOPE`, `E_NOT_FOUND`;
`E_INTERNAL` (500, correlation id only). A refused source does not fail a run: its objects report `source_anomaly`.

**Audit events emitted:**

| Event kind | On | Resource |
|-|-|-|
| `recon.requested` | `POST /recon/runs` | recon_run |
| `recon.swept` | `recon_sweep` (counts) | source (system) |
| `recon.totals_captured` | `ingest_control_totals` (counts only) | recon_run |
| `recon.completed`, `recon.failed` | `recon_run` end (outcome counts or error code) | recon_run |
| `dq.recorded` | `DataQualityRecorder` per batch with issues (kinds and counts) | source |
| `dq_issue.assigned`, `dq_issue.resolved` | assign and resolve routes (no note text) | dq_issue |
| `dq_issue.adjustment_expired` | `dq_expiry_sweep` | dq_issue |
| `known_adjustment.activated` | activation of the `known_adjustment` change request | dq_issue |
| `exclusion.activated`, `exclusion.expired` | activation of the `exclusion` change request; sweep | exclusion |
| `coverage.ledger_refreshed`, `coverage.unit_dark`, `coverage.unit_recovered` | `coverage_ledger_refresh` (transitions only) | source, coverage_unit |
| `certificate.issued` | `coverage_certificate` | coverage_certificate |
| `certificate.attested` | attestation route | coverage_certificate |
| `quarantine.assigned` | assign route (retry and exclude events are phase 11's) | quarantine_item |

Reused: `change.submitted`, `change.approved`, `change.activated` (phase 04, on every change request of this phase),
`evidence.stored` (phase 02), `authz.denied` (phase 03).

**Cedar.** One bundle file, `73_recon.cedar` (the range phase 03 reserved after `70_sources` and `71_mapping`). Entities added
to `schema.cedarschema`: `ReconRun { source: String, scope: String, status: String }`, `Certificate { source: String,
status: String, attested: Bool }`, `DqIssue { kind: String, severity: String, status: String, owner: User }`,
`Exclusion { unit: String, ended: Bool }`, `CoverageUnit { unit: String, covered: Bool }`, and the phase 11 entity `Quarantine` gains `gate: String`. Attributes evaluated: see the excerpt below.

| Action | Resource | Who (shipped bundle) |
|-|-|-|
| `recon.run` | System | DS |
| `recon.read` | System, ReconRun | DS, M, IA |
| `certificate.read` | System, Certificate | DS, M, CH, IA |
| `certificate.attest` | Certificate | M only, with `context.aal == "aal2"` and a consumed `signing` step-up |
| `coverage.statement` | System | M, CH, IA |
| `coverage.read` | System, CoverageUnit | DS, M, IA |
| `exclusion.read` | System, Exclusion | DS, M, CH, IA |
| `exclusion.propose` | System | DS (the approval runs in the change request: M, CHK and, for `decrease`, CH) |
| `dq_issue.read`, `dq_issue.assign`, `dq_issue.resolve` | DqIssue | DS |
| `known_adjustment.read` | System | DS, M, IA |
| `known_adjustment.propose` | System | DS (the approval runs in the change request: M and CHK) |
| `quarantine.assign` | Quarantine | DS (`quarantine.read`, `.retry` and `.exclude` are phase 11's, `72_ingest.cedar`) |

```cedar
// 73_recon.cedar (excerpt; phase 05 idiom)
@id("recon.steward") @role("DS")
permit (principal, action in [Akr::Action::"recon.run", Akr::Action::"recon.read",
        Akr::Action::"certificate.read", Akr::Action::"coverage.read", Akr::Action::"exclusion.read",
        Akr::Action::"exclusion.propose", Akr::Action::"dq_issue.read", Akr::Action::"dq_issue.assign",
        Akr::Action::"dq_issue.resolve", Akr::Action::"known_adjustment.read", Akr::Action::"known_adjustment.propose",
        Akr::Action::"quarantine.assign"], resource)
when { principal.roles.contains("DS") };

@id("recon.mlro") @role("M")
permit (principal, action in [Akr::Action::"recon.read", Akr::Action::"certificate.read",
        Akr::Action::"certificate.attest", Akr::Action::"coverage.statement", Akr::Action::"coverage.read",
        Akr::Action::"exclusion.read", Akr::Action::"known_adjustment.read"], resource)
when { principal.roles.contains("M") };

@id("recon.oversight") @role("CH,IA")
permit (principal, action in [Akr::Action::"certificate.read", Akr::Action::"coverage.statement",
        Akr::Action::"exclusion.read"], resource)
when { principal.roles.containsAny(["CH", "IA"]) };

@id("recon.audit_reads") @role("IA")
permit (principal, action in [Akr::Action::"recon.read", Akr::Action::"coverage.read",
        Akr::Action::"known_adjustment.read"], resource)
when { principal.roles.contains("IA") };

// Attestation needs AAL2; the `signing` step-up is consumed by the handler (phase 01 StepUpService::consume, DR-29).
@id("recon.attest_aal2") @deny("aal")
forbid (principal, action == Akr::Action::"certificate.attest", resource is Akr::Certificate)
when { context.aal != "aal2" };
```

A card-guard issue is never accepted and a card-guard row never excluded: the services refuse both (sections 3.5 and
3.11), because the kind is read after the policy decision. Attributes evaluated: role membership, `principal.status`,
`context.aal`, `resource.status`, `resource.gate`.

Bundle tests: only M can attest, and only with a consumed `signing` assertion; DS reads certificates and the ledger but
cannot attest; CH and IA read the statement and the certificates and cannot propose anything; an A1 principal reaches none
of these actions (403 `E_SCOPE` at the scope gate before Cedar).

**Tipping-off review.**
- Nothing in this phase names a case, an alert, a customer, an account or a party. The ledger, the certificates and the
  register speak in branches, products, transaction types, channels, feeds and counts; the landed values stay in
  `canon` and `raw`, which no route here returns.
- The coverage statement and the ledger are detection logic (DR-64 style): a dark unit or an exclusion tells the reader
  which typologies are blind. They are readable only by DS, M, CH and IA through their own roles, never by the
  branch, operations, KYC or analyst roles. The branch window (phase 22) never sees a dark unit.
- The audit events carry codes, counts, ids and dates. Free text (`reason`, `resolution_note`, `exclusion_reason`, `comment`)
  stays in the row, never in the audit detail. The quarantine detail returns key names only.
- The answer for a person outside the roles above is nothing: the routes refuse at the scope gate (403) and no answer
  reveals which units exist.

**Settings and configuration used:** the twelve keys of migration 0039 (`recon.gl_tolerance` and
`recon.control_total_tolerance` governed decimals; `recon.lookback_days`, `recon.totals_max_wait_h`,
`coverage.dark_unit_days`, `coverage.ledger_window_days` integers; `coverage.exclusion_max_days`,
`coverage.exclusion_review_days`, `coverage.adjustment_max_days` governed integers with registry bounds (DR-34);
`dq.sla_days` JSON by severity; `schedule.recon_daily`, `schedule.dq_expiry_sweep` cron); the bank calendar of phase 09
(working days); phase 11 freshness and `admin.batches.control_totals`; phase 01 `stepup.ttl_s`; phase 02 `evidence.lock_days`
for R6 evidence. Change-request routing uses the shipped `change.route_map` (phase 04) unchanged.

### 3.13 Adversarial checks (decision-adjacent: exclusions and attestation)

Each candidate below is checked against the schema or the logic of this section; none is accepted as safe on reading alone.
- Two stewards propose an exclusion for the same unit at once: the live-exclusion partial unique index and the
  change request's `object_key` partial unique index admit one; the other answers 409 or is refused at submit.
- The owner approves their own exclusion through a second account: `approved_by <> owner_id` is a CHECK, and the owner
  check in `check_before_decision` refuses first; the approver's own maker check is phase 04's four-eyes in three layers.
- An exclusion ends at the same moment a certificate is built: the certificate records the exclusion ids it read, so the
  record is self-consistent; the ledger refresh that follows recomputes the unit.
- A late batch lands after a certificate was issued: the sweep queues a new day run; the new certificate supersedes the old
  one (`superseded_at`), the old one is never edited, and an attestation applies only to the revision the MLRO read.
- Two attestations on the same certificate: the second sees a moved version (`If-Match`) or a non-pending status and
  answers 409.
- A steward retries and excludes the same quarantine row: the conditional `transition` admits one; the other answers
  409 with the current state.
- A replayed `POST /recon/runs` with the same `Idempotency-Key`: the stored 202 is returned byte for byte; a changed body
  under the same key answers 422 `E_IDEMPOTENCY_REUSE`.
- A known adjustment proposed on a break that the next run has already re-checked: the CR's `check_before_decision` sees the
  moved issue version and refuses with `E_STALE_BASE`.

## 4. Infrastructure Updates

- **Job kinds registered.** Jobs of akr-worker run as `akr_app` and keep their own prefixes (`recon_`, `coverage_`, `dq_`);
  the one job of akr-ingestd keeps the `ingest_` prefix that `akr_ingest` owns (phase 00).

| Binary | Kind | Payload | Coalesce key | Concurrency | Max attempts | Enqueued by or schedule |
|-|-|-|-|-|-|-|
| akr-worker | `recon_run` | `{run_id}` | `recon_run:<run_id>` | 2 | 3 | `POST /recon/runs`; `recon_sweep`; `ingest_control_totals` on completion |
| akr-worker | `recon_sweep` | `{}` | `recon_sweep` | 1 | 3 | `schedule.recon_daily`, daily 03:20 bank time |
| akr-worker | `coverage_ledger_refresh` | `{source_id}` | `coverage_ledger_refresh:<source_id>` | 1 | 3 | `recon_run` completion; exclusion activation; `dq_expiry_sweep` |
| akr-worker | `coverage_certificate` | `{source_id, business_date, run_id}` | `coverage_certificate:<source_id>:<date>:<run_id>` | 1 | 5 | `recon_run` completion of a day run |
| akr-worker | `dq_expiry_sweep` | `{}` | `dq_expiry_sweep` | 1 | 3 | `schedule.dq_expiry_sweep`, daily 03:05 bank time |
| akr-ingestd | `ingest_control_totals` | `{run_id}` | `ingest_control_totals:<run_id>` | 2 across sources (the `LoadGovernor` admits each session) | 2 | `recon_run` step 3 |

A `Blackout`, `OutsideWindow` or `Paused` answer from the governor is not an attempt (section 3.2). `recon_sweep` and
`dq_expiry_sweep` are scheduled through the phase 00 timers, so a restart does not double a day.

- **Docker compose:** no change. No new service and no new network edge.
- **AppState additions:**
  - `api`: `recon: Arc<ReconService>`, `coverage: Arc<CoverageService>` (`CoverageStatementService`),
    `dq: Arc<DataQualityService>`, `quarantine: Arc<QuarantineService>`; change targets `ExclusionTarget` and
    `KnownAdjustmentTarget` registered in `ChangeTargetRegistry`.
  - `worker`: `RungEvaluator`, the `recon_run`, `recon_sweep`, `coverage_ledger_refresh`, `coverage_certificate` and
    `dq_expiry_sweep` handlers, `CertificateSigner` (transit `akr-sign-certificate`).
  - `ingestd`: `ControlTotalsReader` (with the plan read from the adapter pack), `DataQualityRecorder` (called by the phase
    11 load loop).
- **OpenAPI registration:** tags `recon` (6 operations), `coverage` (4), `dq` (5), `quarantine` (1, the assign route): 16 in all, each with
  `x-akr-action`. Every POST documents `Idempotency-Key`; the runs and the proposals document 202 with `Location`; the
  attestation and the assign, resolve, retry and exclude routes document `If-Match` and 428; the attestation documents
  `X-StepUp-Assertion`; every response documents `X-Data-As-Of`.
- **Network zones:** only akr-ingestd reaches a source, and only through `ingest_control_totals` (the zone rule of phase 05,
  unchanged). akr-worker signs through OpenBao on the signing key it already holds (phase 00 AppRole policy lists
  `akr-sign-certificate`). No new edge.
- **Readiness:** akr-ingestd `/readyz` adds `control_totals_plan` (each active source's adapter pack names its control-totals
  objects or declares them absent; an absent declaration is a warning, not a failure). akr-worker `/readyz` adds
  `certificate_signer` (the transit key answers a test signature over a fixed digest).
- **Metrics:** gauges `akr_recon_results{rung,outcome}`, `akr_dq_issues_open{kind,severity}`, `akr_dq_overdue{severity}`,
  `akr_dark_units{source}`, `akr_certificates{status}`, `akr_quarantine_open{reason}`, `akr_quarantine_oldest_age_days`;
  counter `akr_dq_recorded_total{kind}`; histograms `akr_recon_run_seconds` and `akr_recon_totals_seconds`. Alert rules: a day
  run `failed`; a certificate `pending_attestation` for more than two working days; a `critical` issue overdue for more than
  one day; the oldest quarantine item older than seven days.
- **Existing entity, schema and route updates required by this phase:**
  - Phase 00: settings and schedules of migration 0039; the `ingest_` prefix covers `ingest_control_totals`.
  - Phase 01: no new step-up class; the attestation uses `signing`, and change-request approvals use `approval` (phase 04).
  - Phase 02: evidence owner kind `coverage_certificate` registered (R6); the certificate's sealed document is stored
    through `put_generated` in the transaction that inserts the row.
  - Phase 03: Cedar file `73_recon.cedar` and the six entities of section 3.12 in `schema.cedarschema`.
  - Phase 04: change targets `exclusion` and `known_adjustment` registered; `change.route_map` unchanged (both are material by
    base class and route to `cr_material` by the class rule); the generic create refuses both kinds.
  - Phase 05: the adapter pack `discovery.json` gains a `control_totals` block (object names, date columns, branch, currency,
    ledger and sign columns: data, not code); `StatementKind::ControlTotals` added to the `LoadGovernor`.
  - Phase 09: the reason codes of family `exclusion` (prefix `EXC_`) must be activated by a family change request before the
    first exclusion (DR-82).
  - Phase 10: no change. The writer's `RowConflict`s are read through the reason names phase 11 writes
    (`fact_changed_in_source`, `value_change`, `label_change`, `parent_missing`; section 3.3).
  - Phase 11: the batch receipts carry `control_totals.read` for each unit (section 3.1 step 4; `business_from` is the
    first date read, the re-read window included); the load loop calls `DataQualityRecorder::record_batch` in the batch
    transaction after it writes its quarantine rows (DR-97); `QuarantineRepository` gains `assign` and
    `accept_conflicts`; its list gains two filters and its exclusion two refusals (section 3.11); the blind-period
    table and routes are used as phase 11 defines them.
  - Planning docs aligned: API.md 3.7 (the 12 rows listed there for this phase plus `POST /recon/runs`, the attestation,
    `POST /dq/issues/{id}/assign`) and 3.5 (`POST /quarantine/{id}/assign`); DATA-MODEL s.6 (the columns of section 1),
    s.21 (phase 12 grants) and s.23 (group 12 file names); MODULES M7; roadmap row 12 and the migration chain.

## 5. Frontend Impact

Screens T1 (completed), T2 and T4 (DESIGN-SYSTEM.md 8.1 catalogue: T1 `/data/connection` DASH, T2 `/data/recon` REGISTER,
T4 `/data/quality` LIST). Phase 08 builds the shell, the data-table component, `StepUpDialog`, the status-word and severity
components, and the basic T1 of phase 08. This phase adds the panels below. Every figure states its `data_as_of`; a stale
source shows the stale state beside the figure it affects (research 31 IR-74).

**Generated-client hooks consumed** (`make generate-api`): `useGetReconRuns`, `usePostReconRuns`, `useGetReconRunsId`,
`useGetReconCertificates`, `useGetReconCertificatesBusinessDate`, `usePostReconCertificatesBusinessDateAttest`,
`useGetCoverageStatement`, `useGetCoverageLedger`, `useGetCoverageExclusions`, `usePostCoverageExclusions`,
`useGetDqIssues`, `usePostDqIssuesIdAssign`,
`usePostDqIssuesIdResolve`, `useGetDqKnownAdjustments`, `usePostDqKnownAdjustments`, `usePostQuarantineIdAssign`, and the
phase 11 hooks `useGetQuarantine`, `usePostQuarantineIdRetry`, `usePostQuarantineIdExclude`,
`useGetCoverageBlindPeriods`.

- **T1 Data connection (completed; roles DS, M, ADM; DASH).** The phase 08 tile (last grant check, "Read-only verified",
  replica delay, kill switch) stays. Added panels: the reconciliation table of the latest completed day run of each source
  with the outcome words "Match", "Open", "Confirmed gap" and "Source anomaly" and the rung counts; the data-quality list
  with severity words and counts, each count linking to T4; the quarantine count and the oldest age in words ("Oldest item
  9 days"); the dark-unit count linking to T2; the link to the certificate of the latest date with its status word. ADM
  holds no `dashboard.read` or `data.steward` scope, so the phase 12 panels are not rendered for ADM (hidden, not disabled).
- **T2 Reconciliation and coverage ledger (new; roles DS, M, IA; REGISTER).**
  - Header: source selector, business date, the `data_as_of` banner with the stale state.
  - Certificate band: status word ("Issued", "Awaiting attestation", "Attested"), the declared gaps by kind, the covered share
    by count and by value as a gapped, labelled bar with the words beside it. M sees "Attest" with the `StepUpDialog`
    (class `signing`) and sends `If-Match`; DS and IA see the status only.
  - Rung table: rung, object or group, expected, actual, delta, tolerance, outcome word, reason words. Sticky header,
    keyboard row navigation, figures in Western digits with the user's digit setting for labels.
  - Coverage ledger: tabs per unit kind (branch, product, transaction type, channel, feed); columns unit, covered words
    ("Monitored", "Dark since 12 Mar 2026", "Excluded", "Source stale", "Blind period"), volume, value and share; filter "Dark
    only".
  - Exclusions panel (DS; hidden for other roles, not disabled): the register, and "Propose exclusion" as a drawer with the
    unit (chosen from the ledger), the reason code (active `EXC_` codes only), the reason text, the owner (active stewards),
    the review date and the expiry, both bounded by the settings. Submit answers 202 and shows the change request, its
    route and its state.
  - Blind periods panel: the periods of the selected source, read through phase 11's `GET /coverage/blind-periods`; the
    declaration action sits in phase 11's own screen section, not here.
- **T4 Data-quality and quarantine queue (new; role DS only; LIST).** Two tabs.
  - Issues: filters (source, kind in plain words, severity, status, owner, overdue). Columns: severity word, kind label, object,
    field, count, first seen, age in days, due date, owner, status word ("Open", "Overdue", "Resolved", "Accepted until").
    Row actions: assign owner, resolve (a dialog with the resolution and a note), propose known adjustment (a drawer: the
    issue, the accepted-until date bounded by the setting, the reason, and the bank-signed document chosen from the source's
    documents). A `card_guard` row has no adjustment action.
  - Quarantine: filters (source, status, reason, owner, older than). Columns: age (oldest first), reason label, source table,
    gate word ("Blocks until mapped" for a transaction type), status word, owner. Row actions: assign, retry (a row of
    kind conflict has none), exclude (a reason dialog behind the step-up `decision`; hidden for rows the service refuses). The detail drawer lists key names only.
  - Keyboard: arrow keys move rows; Enter opens the row; the single-letter shortcuts are on the phase 08 layer and can be
    turned off (WCAG 2.1.4).
- **Browser checks (phase 08 harness):** Playwright runs T1, T2 and T4 in both languages at 320 CSS px; axe reports no
  serious violation; every status and severity is a word as well as a colour (WCAG 1.4.1).

## 6. Verification

1. `make migrate`: 0036 to 0039 apply; a second run is a no-op; the phase 00 checks pass for the six new tables and the three
   quarantine columns (classification and retention rows present; no reserved word). Negative checks, each as the role
   named: a second active day run for one source and date fails `uq_recon_runs_day_active`; a second `open` issue with the
   same `(source_id, object, field, kind)` fails `uq_dq_issues_open`; a `card_guard` row set to `accepted` fails its CHECK;
   an `attested` certificate with an empty `gaps` array fails its CHECK; `approved_by = owner_id` on an exclusion fails its
   CHECK; a second live exclusion for one unit fails `uq_exclusions_active`; a second current certificate for one source and
   date fails `uq_coverage_certificates_current`; `akr_app` gets `permission denied` on DELETE of every table of this phase;
   `akr_app` UPDATE of `admin.exclusions.unit` is refused (only the end columns are granted); `akr_ingest` UPDATE of
   `dq_issues.status` is refused and UPDATE of `dq_issues.count` succeeds; `akr_ingest` UPDATE of `recon_results.actual` is
   refused and its INSERT of expected rows succeeds.
2. `cargo check` with `cargo sqlx prepare`; `cargo clippy --all-targets -- -D warnings`; `cargo deny check` (akr-ingestd gains
   no dependency beyond the phase 05 `sqlproof` crate; akr-api and akr-worker link no source driver).
3. `cargo test`, including:
   - `RungEvaluator` (`domain/recon/tests`): a difference exactly at the tolerance is within; one unit above is not; a
     `source_anomaly` carries no delta; an open break with an accepted matching issue becomes `confirmed_gap`; the receipt
     rung maps `landed` to `open` with `batch_not_mapped`, `rejected` to `open` with `batch_rejected`, and `mapped` to
     `reconciled`; `Outcome` serialises to the four C-25 words.
   - `GroupKey` and `Tolerance`: round trip of `date|branch|currency|ledger`; a negative tolerance is refused; ordering is
     by date then branch.
   - `DataQualityRecorder`: two batches with the same conflict produce one row whose `count` is the sum; a rolled-back batch
     leaves no row; the four `ConflictKind` values map to their kinds and severities; a property test asserts that a
     serialised `DqIncrement` contains only the allowed keys (no value, no key, no row).
   - `DarkRule` (`domain/coverage/tests`): a table test over the precedence `excluded`, `blind_period`, `source_stale`,
     `no_postings_since`, `monitored`; `since` is NULL when no posting exists in the lookback; `covered` equals
     `reason = monitored` in every row.
   - `CertificateBuilder`: no gaps gives `issued`; one gap gives `pending_attestation`; a second revision sets
     `superseded_at` on the first and the first has no predecessor requirement; the canonical JSON of a fixed fixture matches
     its committed golden digest; the attestation digest equals SHA-256 of the canonical `gaps`.
   - `ExclusionTarget.prepare`: a unit with zero window volume gives `neutral`; a source with no window rows gives `unknown`,
     which cannot be approved as material (phase 04 `E_EVIDENCE_MISSING` at submit); otherwise `decrease`; the route is
     `cr_material`; the owner's own decision is refused with `E_SELF_APPROVAL`.
   - `KnownAdjustmentTarget`: a `card_guard` issue is refused at prepare; a moved issue version gives `E_STALE_BASE`.
   - Quarantine transitions (table test): `open` to `retried`, `open` or `retried` to `excluded`;
     `excluded` to anything is refused; an assign of an `open` or `retried` row sets its owner and keeps its status; exclusion of a `txn_type` gate row and of a `card_guard` row is refused.
   - Expiry on the test clock: an accepted issue past `accepted_until` returns to `open` with the signed document kept; an
     exclusion past `expires_at` ends with `ended_reason = expired` and its unit recomputes as dark or monitored.
   - Keyset order: quarantine oldest first; issues by severity rank, then `due_at`, then id.
4. Curl suite `tests/curl/phase12_recon.sh`. Preconditions: the phase 05 suite has left `CBS1` active with a completed profile
   run and an approved control-totals object; the phase 11 suite has landed and mapped batches for two business dates (`D1`,
   `D2`) and left one `card_guard` issue, one `txn_type` gate quarantine row and one open `recon_break`. Dev users: `ds`,
   `m`, `ia`, `ch`, `a1`, `adm`, `sec` (there is no assessor dev user; the assessor reads are not exercised here).
   - As A1 `GET /api/v1/recon/runs?source_id=<CBS1>` -> 403 `E_SCOPE`; as IA `GET /api/v1/dq/issues` -> 403 `E_SCOPE`.
   - As DS `POST /api/v1/recon/runs` with `Idempotency-Key: rec-0001` and `{"source_id":"<CBS1>","business_date":"<D1>","scope":"day"}`
     -> 202 with `Location`. The same key and body -> the stored 202 with `x-idempotent-replay: true`. The same key with
     `business_date` changed -> 422 `E_IDEMPOTENCY_REUSE`. A second key for `<D1>` while the first run is active -> 409
     `E_NOT_ALLOWED_IN_STATE`, reason `run_active`.
   - Poll `GET /api/v1/recon/runs/<run_id>` until `status` is `completed` (a blackout defers the totals read; the test waits
     for the window): rung results present, each with an outcome word, and `X-Data-As-Of` on the response.
   - As DS `GET /api/v1/recon/runs/<random UUIDv7>` -> 404 `E_NOT_FOUND` with the standard problem body. The masked 404 of
     existence masking is not exercised: no resource of this phase is suspicion-tier (Open doubts).
   - As DS `GET /api/v1/recon/certificates/<D1>` -> 200 with `status` `issued` or `pending_attestation`, a `signature` and a
     `key_version` of at least 1.
   - As M `POST /api/v1/recon/certificates/<D1>/attest` with `{"source_id":"<CBS1>"}`: without step-up -> 401
     `E_STEPUP_REQUIRED`; with a fresh `signing` assertion and no `If-Match` -> 428 `E_PRECONDITION_REQUIRED`; with the
     current `If-Match` -> 200 and `status` `attested` when pending, or 409 `E_NOT_ALLOWED_IN_STATE` (`not_pending`) when
     `issued`; re-using the assertion -> 401 `E_STEPUP_REQUIRED`.
   - As IA the same attest -> 403 `E_FORBIDDEN` (IA holds `dashboard.read`, so the scope gate passes and Cedar denies); as
     DS -> 403 `E_SCOPE`.
   - As DS `POST /api/v1/coverage/exclusions` on a branch unit of `<CBS1>` with `owner_id` of `ds`, a reason code of the active
     `EXC_` family and the review and expiry dates inside the settings -> 202 with a change request id. An unknown `unit_ref`
     -> 422 reason `unknown_unit`; an inactive code -> 422 reason `reason_code_inactive`. A second proposal for the same unit
     while the first is live -> 409 reason `exclusion_active`.
   - An exclusion proposal with `owner_id` of `m`, approved by `m` on `POST /api/v1/change-requests/<cr>/approve` ->
     403 `E_SELF_APPROVAL`.
   - As DS `POST /api/v1/dq/issues/<recon_break id>/assign` with `owner_id` of `a1` -> 422 `E_VALIDATION` reason
     `owner_not_active`; with `owner_id` of `ds` and no `If-Match` -> 428; with a stale `If-Match` -> 409
     `E_VERSION_CONFLICT`; with the current version -> 200.
   - As DS `POST /api/v1/dq/issues/<id>/resolve` with `resolution: explained` and a 12-character note -> 200 and `status`
     `resolved`; the same call again -> 409 reason `not_open`.
   - As DS `POST /api/v1/dq/known-adjustments` for the `card_guard` issue -> 422 reason `kind_not_adjustable`; for the
     `recon_break` issue with `accepted_until` 400 days ahead -> 422 reason `review_too_far`; with 30 days and a bank-signed
     document -> 202 with a change request id.
   - As DS `GET /api/v1/quarantine?source_id=<CBS1>&order=oldest` -> 200, oldest first, `detail_keys` present and no value anywhere in the
     body (`jq` finds no key whose value is a string of a customer field); as A1 -> 403 `E_SCOPE`.
   - As DS `POST /api/v1/quarantine/<txn_type row>/exclude` with `If-Match` and a reason -> 409 reason `gate_not_excludable`.
     As DS `POST /api/v1/quarantine/<open row>/assign` with `owner_id` of `ds` and `If-Match` -> 200; on an excluded row ->
     409 reason `quarantine_not_open`. Resolving the `row_conflict_fact` issue moves its open `row_conflict` quarantine rows
     to `accepted` (checked with `GET /api/v1/quarantine?kind=row_conflict`).
   - As M `GET /api/v1/coverage/statement` -> 200 with `data_as_of` and `stale` per source; as CH -> 200; as DS -> 403
     `E_SCOPE`. As DS `GET /api/v1/coverage/ledger?source_id=<CBS1>&covered=false` -> the dark units with their reasons and
     `since` dates.
5. Swagger UI lists the 16 operations under the four tags, each with `x-akr-action`; `make generate-api` run twice gives zero
   diff.
6. Audit: after the curl session `audit_log` holds `recon.requested`, `recon.totals_captured`, `recon.completed`,
   `certificate.issued`, `certificate.attested`, `dq_issue.assigned`, `dq_issue.resolved`, `quarantine.assigned`
   and `coverage.ledger_refreshed`, plus `authz.denied` for the A1, IA and
   DS refusals; `akr-ctl audit verify` passes.
7. Source access: the statement journal of phase 05 shows, between the start and the end of the run, only the approved
   control-totals statements (counts and grouped sums, each a ProvenSelect); no statement reads a customer, account or
   transaction row; the conformance suite of phase 05 is unaffected; akr-api and akr-worker open no source connection.
8. Browser checks (phase 08 harness): T1, T2 and T4 render in Arabic and English at 320 CSS px with no horizontal page
   scroll; every outcome, status and severity has a word; axe reports no serious violation.
9. Phases 00 to 11 tests still pass.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), gap item (a): Cedar form corrected in place (tools/agent/fix-g1-cedar.py); lead-verified.

### 7.1 Cedar form (gap item (a))
The Cedar policies of this phase are corrected in place: a role test is `principal.roles.contains("X")` (several
roles: `principal.roles.containsAny([...])`) in a `when` clause, never `principal in Akr::Role::"X"`, which phase 03's
schema does not model (5 occurrences rewritten; the `@role` annotations are unchanged). The AAL test compares the string: `context.aal == "aal2"` (or `!= "aal2"` in a forbid), as phase 03 writes it; `context.aal` is not a number.

Gap pass 1 (2026-10-10), run 3b, from tools/agent/gap1-run3b.md; lead-verified (tools/agent/gap1-run3b-verdict.md).

### 7.2 Reconciliation runs after a restore (phase 30 section 3.9)
Phase 30's `restore verify` runs its steps in order and stops at the first failure: the retention replay (step 2)
completes before step 5 queues `recon_run`. A reconciliation run for a restore gap is therefore valid as it stands. Phase 30 inserts the run row and queues the `recon_run` job itself, as `recon_sweep` step 1 does, so the route checks of `POST /recon/runs` step 1 (`source_not_active`, `no_batches`) do not run for it. A
business date that ingestion has not reloaded yet shows as a break when some of its batches are in and as `source_anomaly` with `no_batches` (section 3.1 `recon_run` step 2) when none is, which is the intended result: the gap is
reported, not waited for. No claim condition or ordering rule is added here.

### 7.3 Backfill routes removed (DR-380)
The Open doubt "Backfill routes" is closed: the rows `POST /sources/{id}/backfills` and `GET /backfills/{id}` are
removed from API.md 3.5. The re-read window (DR-37) and phase 11's automatic catch-up (DR-90) cover gaps; an explicit
range beyond the overlap is P2.

Gap pass 1 (2026-10-10), run 5a (coverage of C, E, RP, S and TS IDs), from tools/agent/gap1-run5a.md; lead-verified (tools/agent/gap1-run5a-verdict.md).

### 7.4 Deferred rungs (Open doubts closed, DR-394)
MODULES M7 puts four rungs in P1 (receipt, counts, control totals, GL tie-out). Rungs L2, L4, L5 and L6 are P2. For L4,
the payment-link table of phase 10 is filled only by a mapping from a source that holds the link; match rules and the
routing of unmatched items to L4 (C-43) are P2. The Open doubts item on the deferred rungs is closed.

Adversarial pass (2026-10-10), run 3 (watermarks, restores, failover, late batches, source pauses), from tools/agent/adv-run3.md; lead-verified (tools/agent/adv-run3-verdict.md).

### 7.5 Known adjustments bind the group and the amount (ADV-3-10, HIGH; DR-409)
`activate` of the `known_adjustment` target also writes `detail.accepted_delta`: the delta of the issue's latest recon
result for its object and field. `DqRepository::accepted_for` takes the field: `accepted_for(conn, source_id,
object, field, kind)`. Section 3.1 step 7 returns `confirmed_gap` only for an accepted issue of the same stored object (`<rung>:<object>`, section 3.1 `recon_run` step 8) and
field whose `abs(accepted_delta)` is at least `abs(delta)`; a larger break stays `open` and step 8 writes a new issue.
Test: a break of 50,000,000 IQD in a group accepted for 1,000 IQD stays `open`.

### 7.6 When a blind period covers the as-of date (ADV-3-07, LOW; DR-409)
Section 3.7 step 5: a blind period covers the as-of date `d` when its `business_from` is NULL or on or before `d`, and its
`business_to` is NULL or on or after `d`. Phase 11 sets `business_from` on every automatic source open (phase 11
section 7+). A paused source is dark through `source_stale` (phase 11 section 7+), because the as-of date is the last
date read, which the pause does not cover.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.7 Values of the quarantine gate (V-11-06-a, MEDIUM; DR-413)
`gate` (section 3.11; phase 11 section 7+) has two values, derived when the row is read and never stored: `txn_type`
for a row whose `reason_code` is `unmapped_code` and whose `detail.field` is `txn_type`, and `none` for every other
row, so the Cedar attribute is always a string. A card-guard row is known by its `reason_code` `card_guard`, not by
the gate: the service refuses its exclusion with `card_guard_not_excludable` after the policy decision.

### 7.8 The latest recon result of an issue (V-12-10-b, V-12-10-c, MEDIUM; DR-413)
`ReconRepository` gains one read:
`async fn latest_result(&self, conn: &mut PgConnection, source_id: Uuid, rung: Rung, object: &str, group_key: &GroupKey) -> Result<Option<ReconResult>, RepoError>;`
It returns the `recon_results` row with this rung, object and group key from the completed run of the source with the
newest `finished_at`, or `None`. In block 7.5, the issue's latest recon result for its object and field is this read: the
issue stores its object as `<rung>:<object>` (section 3.1 `recon_run` step 8), which the caller takes apart, and its
field is the group key. An issue of another kind than `recon_break` has no recon result; for it, and whenever the read
returns `None`, `activate` writes no `accepted_delta`.

### 7.9 Blind periods that cover a date (V-12-12-a, MEDIUM; DR-413)
`CoverageRepository` gains one read, used by section 3.7 step 5 and by the certificate of section 3.8:
`async fn blind_periods_covering(&self, conn: &mut PgConnection, source: Uuid, as_of: NaiveDate) -> Result<Vec<BlindPeriod>, RepoError>;`
It reads `admin.blind_periods` (phase 11's table; `akr_app` holds SELECT) and returns the periods of scope `system`
and the periods of scope `source` of this source that cover `as_of` by the rule of block 7.6. `BlindPeriod` is the
phase 11 entity. Nothing is written.

## Decisions

Recorded in research.md: DR-94 (the ladder: four rungs and the four C-25 outcomes, expected figures read by
akr-ingestd and actuals by akr-worker, breaks never netted, rungs L2, L4, L5 and L6 deferred), DR-95 (certificates:
signed by the system, MLRO attestation with step-up `signing` only when gaps are declared, revisions with no
predecessor rule), DR-96 (exclusions and known adjustments as `material` change targets with a measured volume effect;
row exclusion refused for unmapped transaction types and card reasons), DR-97 (the DQ register: one open issue per
key counted by a recorder in phase 11's batch transaction beside the row-level quarantine rows; conflict rows close
with their issue; route split with phase 11; six job kinds; no new problem code). The Cedar file `73_recon.cedar`
follows the phase 03 range convention.

## Open doubts (for gap pass 1 and the bank)

- **Backfill routes.** API.md 3.5 lists `POST /sources/{id}/backfills` and `GET /backfills/{id}` under phase 12, but
  neither this phase nor phase 11 (automatic catch-up, DR-90) builds an operator-planned backfill. Gap pass 1 decides the
  owner or drops the rows.
- **Batch units.** The counts rung assumes that a receipt's `control_totals.read` counts the rows read from the source for the
  unit's business-date range `business_from..business_to`, without the re-read overlap that precedes `business_from`. Phase 11
  must confirm what `read` counts; if it includes the overlap, the rung needs the overlap's count from the source too.
- **Rungs L2, L4, L5 and L6** of C-25 are deferred (section 1 and the trailer). The lead should confirm that M7 P1 does not
  require them in this phase.
- **Sources without ledger totals.** A source that keeps no ledger movement totals reports `source_anomaly` on the GL rung and the
  run completes. The bank may prefer that the GL rung is absent for such a source; decide with the adapter pack review.
- **Masked 404.** No resource of this phase is suspicion-tier, so the existence-masking case the template asks for is not
  exercised; the curl suite asserts the standard `E_NOT_FOUND` body only. The lead should decide whether card-guard issues or
  quarantine details must mask for non-steward roles.
- **Excluding a landed row.** Row-level quarantine exclusion does not need a change request in this draft (step-up `decision`
  only); the lead should decide whether the rule for exclusions (material) applies once a threshold of rows is reached.
- **Early withdrawal of an exclusion.** Not in this phase; an exclusion ends at expiry only. A withdrawal route and its change
  target are needed if the bank asks for one.
- **Attestation signature.** The MLRO's attestation is a recorded signature (row and audit chain), not a second transit
  signature. Confirm that this meets C-26's "signed".
- **Assessor and dev users.** T2 lists the assessor among its roles (research 31), but no assessor dev user exists
  (`as@bank.test` is not in the roster), so the assessor read is untested here; no user is invented.
- **Defaults to confirm with the bank.** `coverage.dark_unit_days` (5 working days), `recon.gl_tolerance` (0.01, taken from the
  ICS BANKS observation in DATA-MODEL s.6, not from IQD minor-unit practice) and `dq.sla_days` are placeholders.
- **API.md route identifiers.** The approval route is `POST /change-requests/{id}/approve` (API.md, phase 04); if phase 04 changes
  it, the curl suite changes with it.

## What This Phase Does NOT Touch

- No production load: no watermarks, batches, landing, load loop or replay (phase 11). The `DataQualityRecorder` is a library
  here; phase 11 calls it, and the batch transaction stays phase 11's.
- No freshness classes F0 to F5, no stale-source detection and no restore or failover detection (phase 11, its section 3.7 and
  section 3.5); this phase reads the freshness snapshot and shows it.
- No blind-period table, cause, opening or route (phase 11, section 3.6); this phase reads the table only.
- No replay routes and no mapping changes (phases 10 and 11); the writer and its `RowConflict`s are phase 10.
- No rungs L2, L4, L5 and L6 of C-25 (a later phase, Open doubts), no payment tie-out and no sequence-gap check.
- No backfill planning (Open doubts), no CBS write, no new grant on any source, and no new source tier.
- No annual certification pack and no `POST /packs` kinds (phase 24, and phase 06 for the three kinds it already has).
- No branch-window display of coverage or of the blind-period banner on cases (phases 18 and 22); no case, alert or STR counts.
- No scenario, feature, screening or rating use of the ledger (phases 13 to 16).
- No new step-up action class, no new problem code, and no change to the phase 01 closed enum.
- No change to the phase 04 routes or to `change.route_map`; the two new change targets only register with the existing routes.
- No change to the phase 00 to 09 tables and no change to a phase 11 table or its grants; only the settings and schedules
  of migration 0039; no new role.
