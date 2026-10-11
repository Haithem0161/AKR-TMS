-- migration 0002_platform_jobs_outbox_timers.sql
-- Phase 00. The DBS harness: durable jobs (FOR UPDATE SKIP LOCKED), the transactional outbox and timers.
-- Enqueued ONLY inside the transaction that caused them (.claude/rules/jobs.md golden rule).
-- Payloads carry ids and codes only: never a name, a reason or free text (tipping-off, section 3).

DO $$ BEGIN
    CREATE TYPE admin.job_status AS ENUM ('queued', 'running', 'done', 'failed', 'dead');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS admin.jobs (
    id            UUID PRIMARY KEY,                       -- UUIDv7, app-generated
    kind          TEXT NOT NULL CHECK (kind ~ '^[a-z][a-z0-9_]*$'),
    payload       JSONB NOT NULL DEFAULT '{}'::jsonb,
    status        admin.job_status NOT NULL DEFAULT 'queued',
    run_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    attempts      INT NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    max_attempts  INT NOT NULL DEFAULT 5 CHECK (max_attempts > 0),
    last_error    TEXT,                                   -- redacted (section 3, telemetry)
    locked_at     TIMESTAMPTZ,
    locked_by     TEXT,                                   -- worker instance id
    coalesce_key  TEXT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    finished_at   TIMESTAMPTZ,
    CHECK ((status IN ('done', 'dead')) = (finished_at IS NOT NULL))
);
-- The poll: WHERE status = 'queued' AND run_at <= now() ORDER BY run_at.
CREATE INDEX IF NOT EXISTS idx_jobs_poll        ON admin.jobs (status, run_at);
CREATE INDEX IF NOT EXISTS idx_jobs_kind_status ON admin.jobs (kind, status);
CREATE INDEX IF NOT EXISTS idx_jobs_finished    ON admin.jobs (finished_at) WHERE finished_at IS NOT NULL;
-- One queued job per (kind, coalesce_key): repeated enqueues collapse; a job already running does not block the
-- next one (a change after it started needs a fresh run).
CREATE UNIQUE INDEX IF NOT EXISTS uq_jobs_coalesce_queued
    ON admin.jobs (kind, coalesce_key) WHERE status = 'queued' AND coalesce_key IS NOT NULL;

CREATE TABLE IF NOT EXISTS admin.outbox (
    id             UUID PRIMARY KEY,
    topic          TEXT NOT NULL CHECK (topic ~ '^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$'),
    payload        JSONB NOT NULL,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    available_at   TIMESTAMPTZ NOT NULL DEFAULT now(),   -- back-off moves it forward
    dispatched_at  TIMESTAMPTZ,
    attempts       INT NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    last_error     TEXT
);
CREATE INDEX IF NOT EXISTS idx_outbox_pending    ON admin.outbox (available_at) WHERE dispatched_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_outbox_dispatched ON admin.outbox (dispatched_at) WHERE dispatched_at IS NOT NULL;

CREATE TABLE IF NOT EXISTS admin.timers (
    id            UUID PRIMARY KEY,
    owner_kind    TEXT NOT NULL CHECK (owner_kind ~ '^[a-z][a-z0-9_]*$'),
    owner_id      UUID NOT NULL,
    kind          TEXT NOT NULL CHECK (kind ~ '^[a-z][a-z0-9_]*$'),
    fire_at       TIMESTAMPTZ NOT NULL,
    calendar_id   UUID,              -- FK to ref.calendars added by phase 09 (deferred FK)
    payload       JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    fired_at      TIMESTAMPTZ,
    cancelled_at  TIMESTAMPTZ,
    CHECK (fired_at IS NULL OR cancelled_at IS NULL)
);
CREATE INDEX IF NOT EXISTS idx_timers_due   ON admin.timers (fire_at) WHERE fired_at IS NULL AND cancelled_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_timers_owner ON admin.timers (owner_kind, owner_id);

GRANT SELECT, INSERT, UPDATE, DELETE ON admin.jobs   TO akr_app;   -- DELETE: maintenance_sweep prunes finished rows
GRANT SELECT, INSERT, UPDATE, DELETE ON admin.outbox TO akr_app;   -- DELETE: dispatched rows only
GRANT SELECT, INSERT, UPDATE, DELETE ON admin.timers TO akr_app;   -- DELETE: fired or cancelled rows only

-- akr-ingestd runs its own job loop over its own kinds and enqueues mapping jobs for akr-worker. Row-level
-- security keeps it to those kinds (zone Z1 hardening, SP-05); the policy applies to akr_ingest only.
GRANT SELECT, INSERT, UPDATE ON admin.jobs TO akr_ingest;
ALTER TABLE admin.jobs ENABLE ROW LEVEL SECURITY;
DO $$ BEGIN
    CREATE POLICY jobs_app_all ON admin.jobs FOR ALL TO akr_app USING (true) WITH CHECK (true);
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    CREATE POLICY jobs_ingest_own ON admin.jobs FOR SELECT TO akr_ingest
        USING (kind LIKE 'ingest\_%' OR kind LIKE 'discovery\_%');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    CREATE POLICY jobs_ingest_update ON admin.jobs FOR UPDATE TO akr_ingest
        USING (kind LIKE 'ingest\_%' OR kind LIKE 'discovery\_%')
        WITH CHECK (kind LIKE 'ingest\_%' OR kind LIKE 'discovery\_%');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    CREATE POLICY jobs_ingest_insert ON admin.jobs FOR INSERT TO akr_ingest
        WITH CHECK (kind LIKE 'ingest\_%' OR kind LIKE 'discovery\_%' OR kind LIKE 'map\_%');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
