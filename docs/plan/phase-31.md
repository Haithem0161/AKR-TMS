# Phase 31: Training, Capacity & Correspondent Packs

**Goal:** The last P2 phase: a training workspace of synthetic practice cases kept apart from live data by its own schema, a capacity workbench that projects backlog from measured handling times and runs a simulation harness on phase 16 replays (advice only, never a cap), correspondent and validator evidence packs built from system facts, a typology exposure index per bank, branch and correspondent, the correspondent side of the RFI desk, and optional SAML and SCIM through Ory Polis that never grant a role.

**Dependencies:** Phase 19 (RFI objects with kind `correspondent`, `rfi_release_external`, `rfi_clock`, `AlarmService`, the `casework.alarms` CHECKs, QA samples and calibration, the RFI desk), Phase 24 (`admin.training_records` and the training register, `PackService` kinds `correspondent` and `validator` in the CHECK, `admin.validation_packs`, the command centre G1, the registers and their small-number rule); also Phase 00 (jobs, settings, schedules, feature flags, `admin.idempotency_keys`, compose profiles, `deploy/zones.yaml`, OpenBao AppRoles), Phase 01 (`admin.users`, `admin.pools`, `admin.releases` is phase 25, service principals, the database provisioning of `akr-ctl db provision`), Phase 02 (`PackService`, `RenderService`, `EvidenceStore`, audit catalogue, `pack.build_failed`), Phase 03 (`UserAdminService::disable` and `enable`, `OffboardingService`, `StepUpService` classes `identity_admin` and `export`, `Ctx.via`, roles and grants, `POST /admin/users`), Phase 04 (`ChangeTargetRegistry`, `cr_material`, the `capacity` reason tag and its evidence rule), Phase 06 (`pack_build` job, `POST /packs`, `scn.scenarios.typologies`), Phase 09 (`BusinessClock`, reason-code family `decision`), Phase 10 (`canon.account.correspondent`, `canon.txn`, `canon.balance_snapshot`), Phase 12 (`admin.coverage_certificates`), Phase 14 (`alert.harness_runs`, `ref.list_versions`, list freshness), Phase 16 (`scn.replays`, `scn.replay_results`, `scn.seeded_tests` fixture shape), Phase 17 (`alert.alerts`, `casework.cases`, `casework.queues`, `casework.touch_by_hours`, the queue `pool_id` and `filter`), Phase 21 (`report.small_number_threshold`), Phase 22 (notice code `supervisor.alarm`), Phase 25 (Kratos generic OIDC upstream, `auth.upstream_mode`, the release `admin.releases`), Phase 30 (`deploy/catalogue.yaml`, the profile render and drift check). The roadmap lists "19 and 24"; the other dependencies are used by the parts named in the body.

**Macro-phase:** P2 (optional per bank: five feature flags, all off by default; each part does nothing while its flag is off)
**Complexity:** L (15 routes: 2 rows of API.md 3.16 owned here and 13 added; 9 new tables in four schemas (`train`, new; `casework`, `rpt` and `admin`, existing) and 1 view in `train`; 5 migrations `0096` to `0100`; 1 CHECK widened (`casework.alarms`); 5 new job kinds and 1 extended (`pack_build`); 0 outbox topics; 0 timer kinds; 3 Cedar files `E0_directory.cedar`, `E1_training_capacity.cedar`, `E2_exposure_packs.cedar`; 14 settings; 5 feature flags; 18 DR numbers, DR-338 to DR-355)
**Binaries touched:** akr-api (routers `training`, `capacity`, `exposure` (on `GET /typology-exposure`), `directory`; the `rfis` router extended; `TrainingService`, `CapacityService`, `ExposureService`, `DirectoryService` in `AppState`; `UserAdminService::disable_from_directory` added) | akr-worker (jobs `capacity_snapshot`, `capacity_watch`, `capacity_simulation`, `exposure_compute`, `directory_reconcile`; `pack_build` kinds `correspondent` and `validator`) | akr-ctl (`training seed`, `polis connection add`, `deploy render` extended with the `polis` profile, `db provision` extended with database `polis`) | Ory Polis container (third-party, catalogue entry, profile `polis` only). akr-ingestd and akr-assess are unchanged.

**Requirements carried:**
- Implemented here: UX-41 (synthetic practice cases with a known answer and an expert path, a permanent TRAINING mark, practice results shown beside the training register, synthetic tipping-off content; sections 3.1, 3.2, 5.1), M-51 (the training-workspace part only: the register of phase 24 and the feedback of phase 19 are unchanged; sections 3.1, 3.2), M-52 (capacity workbench: workload by class from measured handling times, staffing, shrinkage and occupancy as inputs, fluid backlog projection, sensitivity table, team-level only; sections 3.3, 3.4), M-53 (the capacity-forecast alarm only, through phase 19 `AlarmService`; section 3.4), M-54 (the parts of surge mode that do not suppress alerts: a temporary pool, higher quality-control sampling by change request, hourly progress on G2; section 3.6), M-76 (simulation harness that replays a phase 16 replay through a candidate queue model and reports queue size, clock breaches and analyst hours; section 3.5), W-06 (correspondent and validator packs built from system facts only; section 3.7), GV-35 (one evidence pack source for correspondent questionnaires and the annual certification; section 3.7), S-31 (typology exposure index per bank, branch and correspondent, with trend and a respondent view; research 27 row SCN-CORR-07; section 3.8), W8 (correspondent requests and the pack attachment on the RFI desk; section 3.9), TS-25 (SAML through Ory Polis as the phase 25 OIDC upstream; section 3.10), SP-14 (identities provisioned by an administrator: SCIM never creates a sign-in account; section 3.11), G-12 (the optional directory service of section 3.10 and 3.11; local step-up always; SCIM deactivation disables the account, reversibly, and the final offboarding stays an administrator's act; sections 3.11, 3.12). Design rule D21 (no capacity or simulation output caps, raises or suppresses an alert; sections 3.4, 3.5, 3.6).
- Connected to earlier phases (reused, not repeated): M-42 and M-44 (RFI desk and external release; phase 19), S-39 (capacity reason tag and its evidence rule; phase 04 and 16), S-41 (replay with cost guard; phase 16), S-49 (the seeded fixture shape is the source of practice case structure; phase 16), M-47 and M-49 (QA sampling; phase 19), M-57 (work-in-progress limit of 10; phase 17), S-09 and S-13 (posted events only; phase 16 and 09), GV-32 (validator is never the owner; phase 04 inventory), SP-93 (no score in responses; phase 29), TS-29 (one catalogue; phase 30).
- Not carried: M-50 (queue access gated by a calibration or practice result; phase 19 Open doubts 14), M-53 growth, latency and silence alarms (phase 19 Open doubts 10), M-54 pre-clear of false positives from pair memory (it suppresses alerts; D21; DR-343), M-55 (volume-change preview as a change-request attachment; the harness output exists, the attachment to change requests is left to a later task), M-51 AI golden set (phase 28), the CBDDQ and FCCQ questionnaire content (the bank's compliance owner writes the templates; phase 19 DR-149), the Kurdish (Sorani) practice content (research 31 section 6.4, later), the training coverage report by role and branch (research 31 section 6.4; Open doubts 9), the SCIM group-to-role mapping, SCIM password sync, SCIM creation of sign-in accounts, SAML or SCIM for the local administrator and recovery paths (SP-14 keeps them local), the Polis admin interface and its management API (disabled), LDAP (research 32 14.3: needs a broker, not in this phase).
- Gap pass 1 (coverage), not carried: SP-78 (vendor assurance programme, DR-396); W-22 (correspondent and nested-relationship module, P2 correspondent pack); F-11 (correspondent-bank register), B-12 (correspondent annex); P2 correspondent pack, DR-401; VA-40 (synthetic data generator; P2, DR-402).

---

## 1. SQLx Migration Changes

Five migrations, numbered after phase 30's `0095` (grep of docs/plan confirms `0096` to `0100` are free). Nine tables are created: two in a new schema `train` (DR-338), three in `casework` (capacity), one in `casework` for the RFI pack links, one in `rpt`, two in `admin` (directory). One view is created in `train`. Earlier objects change in exactly four ways, all additive: the `casework.alarms` kind and subject CHECKs are widened (`0097`); five feature flags and fourteen setting rows (four of them schedules) are inserted (`0100`); the grants of phase 24 and phase 19 tables are not touched; and `admin.packs` is not changed at all, because its `packs_kind_check` already lists `correspondent` and `validator` (phase 06 and phase 24 section 1.3 list). `0100` asserts that the constraint still lists them, and fails loudly otherwise.

Ids are UUIDv7 generated app-side (no database default). Every table names its owning role through the grants of `0100`. Writers are `akr_app` (api and worker) and `akr_migrate` (the training seed and the schema). `akr_readonly` is granted nothing in this phase: the readonly pool serves no training, capacity, exposure or directory read.

### 1.1 `0096_train_schema.sql`

```sql
-- migration 0096_train_schema.sql
-- Phase 31, part 1: the training workspace in its own schema (DR-338). No table outside schema train references
-- schema train (test in section 6, item 4). Synthetic data only: the facts of every case carry "synthetic": true.
CREATE SCHEMA IF NOT EXISTS train AUTHORIZATION akr_migrate;

-- Practice cases. Written only by `akr-ctl training seed` (as akr_migrate) from a signed pack document (section 3.1).
-- The API never writes this table. known_answer and expert_path are detection-logic (section 1.5): they are sent to a
-- trainee only after that trainee's own attempt on the case.
CREATE TABLE IF NOT EXISTS train.practice_cases (
    id                UUID PRIMARY KEY,
    case_code         TEXT NOT NULL UNIQUE CHECK (case_code ~ '^PRAC-[A-Z]{2,6}-[0-9]{3}$'),
    course_code       TEXT NOT NULL CHECK (course_code ~ '^[A-Z][A-Z0-9_]{2,30}$'),   -- the bank's course list (phase 24 section 1.2)
    path              TEXT NOT NULL CHECK (path IN ('l1', 'l2', 'mlro', 'rfi_desk', 'screening', 'tipping_off')),
    typologies        SMALLINT[] NOT NULL DEFAULT '{}' CHECK (cardinality(typologies) <= 6),   -- research 27 B2 numbers (phase 06)
    title             JSONB NOT NULL CHECK (title ? 'ar' AND title ? 'en'),
    facts             JSONB NOT NULL CHECK (jsonb_typeof(facts) = 'object' AND facts ->> 'synthetic' = 'true'),
    known_answer      JSONB NOT NULL CHECK (jsonb_typeof(known_answer) = 'object'
                                           AND (known_answer ? 'decision' OR known_answer ? 'choice')),
    expert_path       JSONB NOT NULL CHECK (jsonb_typeof(expert_path) = 'array' AND jsonb_array_length(expert_path) BETWEEN 1 AND 20),
    pack_version      TEXT NOT NULL CHECK (length(pack_version) BETWEEN 1 AND 40),
    active            BOOLEAN NOT NULL DEFAULT TRUE,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_practice_cases_path ON train.practice_cases (path, course_code) WHERE active;

-- Practice attempts. One row per submitted answer; never updated. A decision is one of the four outcomes of the
-- phase 18 proposal vocabulary (no_report, utr, str, tfr); a tipping-off item is answered with a choice number instead.
CREATE TABLE IF NOT EXISTS train.practice_attempts (
    id                UUID PRIMARY KEY,
    case_id           UUID NOT NULL REFERENCES train.practice_cases(id) ON DELETE RESTRICT,
    user_id           UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    decision          TEXT CHECK (decision IN ('no_report', 'utr', 'str', 'tfr')),
    choice            SMALLINT CHECK (choice IS NULL OR choice BETWEEN 1 AND 9),
    reason_codes      TEXT[] NOT NULL DEFAULT '{}' CHECK (cardinality(reason_codes) <= 10),   -- ref.reason_codes family decision
    decision_match    BOOLEAN NOT NULL,
    reasons_matched   SMALLINT NOT NULL CHECK (reasons_matched BETWEEN 0 AND 100),   -- share of required codes chosen
    passed            BOOLEAN NOT NULL,
    attempted_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK ((decision IS NULL) <> (choice IS NULL))
);
CREATE INDEX IF NOT EXISTS idx_practice_attempts_user ON train.practice_attempts (user_id, attempted_at DESC);
CREATE INDEX IF NOT EXISTS idx_practice_attempts_case ON train.practice_attempts (case_id);

-- Per trainee and course: practice results scored on each trainee's FIRST attempt per case only (the first attempt
-- returns the known answer and the expert path, so a later pass of the same case is practice, not a result). Read by
-- the training register (phase 24 section 3.10) as a display column only. It is a view inside schema train, so it
-- cannot be read by any count of the live schemas.
CREATE OR REPLACE VIEW train.practice_scores AS
WITH first_attempt AS (
    SELECT DISTINCT ON (a.user_id, a.case_id) a.user_id, a.case_id, a.passed, a.attempted_at
    FROM train.practice_attempts a
    ORDER BY a.user_id, a.case_id, a.attempted_at, a.id
)
SELECT f.user_id,
       c.course_code,
       bool_or(f.passed)                      AS passed_first,
       count(*) FILTER (WHERE f.passed)       AS cases_passed_first,
       count(*)                               AS cases_attempted,
       (SELECT count(*) FROM train.practice_attempts x JOIN train.practice_cases y ON y.id = x.case_id
         WHERE x.user_id = f.user_id AND y.course_code = c.course_code) AS attempts,
       max(f.attempted_at)                    AS latest_first_attempt_at
FROM first_attempt f
JOIN train.practice_cases c ON c.id = f.case_id
GROUP BY f.user_id, c.course_code;
```

### 1.2 `0097_casework_capacity.sql`

```sql
-- migration 0097_casework_capacity.sql
-- Phase 31, part 2: capacity inputs (per pool and bank week), measured hourly snapshots by work kind and class
-- (never per person), simulation runs on phase 16 replays, the RFI pack links, and the alarm kind capacity_forecast.
CREATE TABLE IF NOT EXISTS casework.capacity_inputs (
    id                  UUID PRIMARY KEY,
    pool_id             UUID NOT NULL REFERENCES admin.pools(id) ON DELETE RESTRICT,
    week_start          DATE NOT NULL CHECK (EXTRACT(ISODOW FROM week_start) = 7),   -- a Sunday: the bank week (research 28 11.2)
    analysts_staffed    NUMERIC(6,2) NOT NULL CHECK (analysts_staffed >= 0 AND analysts_staffed <= 500),
    hours_per_day       NUMERIC(4,2) NOT NULL CHECK (hours_per_day BETWEEN 1 AND 12),
    shrinkage           NUMERIC(4,3) NOT NULL CHECK (shrinkage BETWEEN 0 AND 0.600),   -- leave, training, meetings, QA duty
    occupancy_target    NUMERIC(4,3) NOT NULL CHECK (occupancy_target BETWEEN 0.500 AND 0.900),   -- at most capacity.occupancy_max
    note_code           TEXT CHECK (note_code IN ('leave', 'hiring', 'temporary_pool', 'training', 'other')),
    entered_by          UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    entered_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    superseded_by       UUID REFERENCES casework.capacity_inputs(id) ON DELETE RESTRICT,
    superseded_at       TIMESTAMPTZ,
    version             INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    CHECK ((superseded_at IS NULL) = (superseded_by IS NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_capacity_inputs_live ON casework.capacity_inputs (pool_id, week_start) WHERE superseded_at IS NULL;

-- Hourly measures per pool, work kind and class. Counts and sums only. handling_minutes_sum is the sum of the elapsed
-- minutes of the items decided in the hour (section 3.3); there is no person column in this table, by design (DR-341).
CREATE TABLE IF NOT EXISTS casework.capacity_snapshots (
    id                    UUID PRIMARY KEY,
    pool_id               UUID REFERENCES admin.pools(id) ON DELETE RESTRICT,   -- NULL = the unassigned total (section 3.3)
    bucket_start          TIMESTAMPTZ NOT NULL,                  -- the hour, bank time (bank.timezone)
    work_kind             TEXT NOT NULL CHECK (work_kind IN ('alert', 'case')),
    class_code            TEXT NOT NULL CHECK (length(class_code) BETWEEN 2 AND 12),   -- P0 to P3 for alerts; family code for cases
    arrivals              INT NOT NULL CHECK (arrivals >= 0),
    decided               INT NOT NULL CHECK (decided >= 0),
    handling_minutes_sum  BIGINT NOT NULL DEFAULT 0 CHECK (handling_minutes_sum >= 0),
    backlog_end           INT NOT NULL CHECK (backlog_end >= 0),
    aged_over_sla         INT NOT NULL DEFAULT 0 CHECK (aged_over_sla >= 0),
    computed_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_capacity_snapshot UNIQUE NULLS NOT DISTINCT (pool_id, bucket_start, work_kind, class_code)
);
CREATE INDEX IF NOT EXISTS idx_capacity_snapshots_pool ON casework.capacity_snapshots (pool_id, bucket_start DESC);

-- Simulation runs. The request names one phase 16 replay (completed); the result is written only here.
CREATE TABLE IF NOT EXISTS casework.capacity_runs (
    id                UUID PRIMARY KEY,
    replay_id         UUID NOT NULL REFERENCES scn.replays(id) ON DELETE RESTRICT,
    label             TEXT NOT NULL CHECK (length(btrim(label)) BETWEEN 1 AND 80),
    inputs            JSONB NOT NULL CHECK (jsonb_typeof(inputs) = 'object'),   -- pools, weeks, class mix, touch hours, analysts
    status            TEXT NOT NULL CHECK (status IN ('queued', 'running', 'completed', 'failed')),
    result            JSONB CHECK (result IS NULL OR jsonb_typeof(result) = 'object'),
    requested_by      UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    requested_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at      TIMESTAMPTZ,
    CHECK ((status = 'completed') = (result IS NOT NULL)),
    CHECK (status NOT IN ('completed', 'failed') OR completed_at IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_capacity_runs_replay ON casework.capacity_runs (replay_id);

-- The RFI pack links (section 3.9). A pack is attached to an outbound correspondent request while it is in draft.
CREATE TABLE IF NOT EXISTS casework.rfi_packs (
    rfi_id      UUID NOT NULL REFERENCES casework.rfis(id) ON DELETE RESTRICT,
    pack_id     UUID NOT NULL REFERENCES admin.packs(id) ON DELETE RESTRICT,
    linked_by   UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    linked_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (rfi_id, pack_id)
);

-- Alarm register (phase 19 section 1.1; widened by phases 20, 21 and 22; the lists below repeat all earlier values).
-- Phase 31 adds the kind capacity_forecast and the subject kind pool (the team of section 3.4).
ALTER TABLE casework.alarms DROP CONSTRAINT IF EXISTS alarms_kind_check;
ALTER TABLE casework.alarms ADD CONSTRAINT alarms_kind_check CHECK (kind IN (
    'touch_breach', 'work_item_breach', 'chain_broken', 'conflict_declared', 'flat_line', 'qa_critical',
    'post_order_debit', 'freeze_overdue', 'refrain_overdue', 'report_filing_overdue', 'report_rework_overdue',
    'liaison_report_overdue', 'capacity_forecast'));
ALTER TABLE casework.alarms DROP CONSTRAINT IF EXISTS alarms_subject_kind_check;
ALTER TABLE casework.alarms ADD CONSTRAINT alarms_subject_kind_check CHECK (subject_kind IN (
    'alert', 'work_item', 'case', 'scenario', 'qa_review', 'freeze', 'refrain', 'ops_task', 'report', 'report_task',
    'liaison_report', 'pool'));
```

### 1.3 `0098_rpt_exposure.sql`

```sql
-- migration 0098_rpt_exposure.sql
-- Phase 31, part 3: the typology exposure index (section 3.8). One row per month, typology and scope. Aggregates only.
CREATE TABLE IF NOT EXISTS rpt.typology_exposure (
    id                UUID PRIMARY KEY,
    period_month      DATE NOT NULL CHECK (EXTRACT(DAY FROM period_month) = 1),
    typology          SMALLINT NOT NULL CHECK (typology BETWEEN 1 AND 99),
    scope_kind        TEXT NOT NULL CHECK (scope_kind IN ('bank', 'branch', 'correspondent')),
    scope_ref         UUID,                                    -- canon.branch.id or canon.account.id; NULL for the bank
    flows             INT,                                     -- distinct posted transactions matched; NULL when suppressed
    value_lcy         NUMERIC(28,6),                           -- sum of amount_lcy of those transactions; never converted
    total_flows       INT,
    total_value_lcy   NUMERIC(28,6),
    assets_lcy        NUMERIC(28,6),                           -- month-end balance_lcy of the scope accounts
    suppressed        BOOLEAN NOT NULL DEFAULT FALSE,
    computed_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_typology_exposure UNIQUE NULLS NOT DISTINCT (period_month, typology, scope_kind, scope_ref),
    CHECK ((scope_kind = 'bank') = (scope_ref IS NULL)),
    CHECK (NOT suppressed OR (flows IS NULL AND value_lcy IS NULL AND total_flows IS NULL AND total_value_lcy IS NULL))
);
CREATE INDEX IF NOT EXISTS idx_typology_exposure_scope ON rpt.typology_exposure (scope_kind, scope_ref, period_month DESC);
```

### 1.4 `0099_admin_directory.sql`

```sql
-- migration 0099_admin_directory.sql
-- Phase 31, part 4: the directory rows fed by Ory Polis (section 3.11). A directory row is not a sign-in account: it
-- becomes one only through the link of section 3.11 (an administrator's act). Names and e-mail are sealed.
CREATE TABLE IF NOT EXISTS admin.directory_users (
    id                  UUID PRIMARY KEY,
    external_id         TEXT NOT NULL UNIQUE CHECK (length(external_id) BETWEEN 1 AND 200),   -- the SCIM id
    user_name_enc      BYTEA NOT NULL,                          -- the directory e-mail, sealed (personal, AAD = id)
    display_name_enc    BYTEA,                                   -- sealed (personal, AAD = id)
    state               TEXT NOT NULL DEFAULT 'pending' CHECK (state IN ('pending', 'linked')),
    directory_active    BOOLEAN NOT NULL DEFAULT TRUE,
    linked_user_id      UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    linked_by           UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    linked_at           TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    CHECK ((state = 'linked') = (linked_user_id IS NOT NULL)),
    CHECK ((linked_user_id IS NULL) = (linked_by IS NULL)),
    CHECK ((linked_user_id IS NULL) = (linked_at IS NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_directory_linked_user ON admin.directory_users (linked_user_id) WHERE linked_user_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_directory_pending ON admin.directory_users (created_at) WHERE state = 'pending';

-- The event log: one row per processed event, inserted in the transaction that applied it and never updated.
-- event_ref is the Idempotency-Key of the call (the replay itself is answered by admin.idempotency_keys, phase 00).
CREATE TABLE IF NOT EXISTS admin.directory_events (
    id                  UUID PRIMARY KEY,
    event_ref           TEXT NOT NULL UNIQUE CHECK (length(event_ref) BETWEEN 8 AND 120),
    directory_user_id   UUID REFERENCES admin.directory_users(id) ON DELETE RESTRICT,   -- NULL when the user is unknown
    op                  TEXT NOT NULL CHECK (op IN ('create', 'update', 'deactivate', 'activate')),
    payload_sha256      BYTEA NOT NULL CHECK (octet_length(payload_sha256) = 32),
    outcome             TEXT NOT NULL CHECK (outcome IN ('applied', 'ignored', 'refused')),
    reason_code         TEXT CHECK (reason_code IS NULL OR reason_code IN ('last_holder', 'stale_event', 'reactivation_ignored',
                                                                          'unknown_user')),
    received_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK ((outcome = 'applied') OR reason_code IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_directory_events_user ON admin.directory_events (directory_user_id, received_at DESC);
```

### 1.5 `0100_phase31_guards_grants_settings.sql`

```sql
-- migration 0100_phase31_guards_grants_settings.sql
-- Phase 31, part 5: guards, grants, the pack-kind assertion, feature flags, settings, schedules, classification, retention.
-- Idempotent. Settings copy the INSERT column list of phase 29 section 1.4 (key, value, value_type, floor, governed, label).

-- Practice attempts are final. Only the retention engine (role akr_retention, phase 23) may delete them.
CREATE OR REPLACE FUNCTION train.attempts_guard() RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog AS $$
BEGIN
    IF TG_OP = 'UPDATE' THEN
        RAISE EXCEPTION 'practice attempts are final' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF current_user <> 'akr_retention' THEN
        RAISE EXCEPTION 'practice attempts leave only through their retention run' USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN OLD;
END $$;
DROP TRIGGER IF EXISTS trg_practice_attempts_guard ON train.practice_attempts;
CREATE TRIGGER trg_practice_attempts_guard BEFORE UPDATE OR DELETE ON train.practice_attempts
    FOR EACH ROW EXECUTE FUNCTION train.attempts_guard();

-- A superseded capacity input is changed in two columns only; its replacement is a new row (section 3.3).
-- A simulation run changes only in status, result and completed_at; a directory event row is never changed.

-- The pack kinds of this phase are already in the CHECK (phase 06, widened by phase 24). Fail loudly if that changes.
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint
                   WHERE conname = 'packs_kind_check'
                     AND pg_get_constraintdef(oid) LIKE '%''correspondent''%'
                     AND pg_get_constraintdef(oid) LIKE '%''validator''%'
                     AND pg_get_constraintdef(oid) LIKE '%''certification''%') THEN
        RAISE EXCEPTION 'admin.packs CHECK packs_kind_check lacks correspondent, validator or certification';
    END IF;
END $$;

-- Grants (every write path, with its column list).
GRANT USAGE ON SCHEMA train TO akr_app, akr_retention;
GRANT SELECT ON train.practice_cases TO akr_app;
GRANT SELECT, INSERT ON train.practice_attempts TO akr_app;
GRANT SELECT ON train.practice_scores TO akr_app;
GRANT SELECT ON train.practice_attempts TO akr_retention;   -- R9 kept: no DELETE until a period and handler exist (DR-386)

GRANT SELECT, INSERT ON casework.capacity_inputs TO akr_app;
GRANT UPDATE (superseded_by, superseded_at) ON casework.capacity_inputs TO akr_app;
GRANT SELECT, INSERT ON casework.capacity_snapshots TO akr_app;
GRANT UPDATE (arrivals, decided, handling_minutes_sum, backlog_end, aged_over_sla, computed_at) ON casework.capacity_snapshots TO akr_app;
GRANT SELECT, DELETE ON casework.capacity_snapshots TO akr_retention;
GRANT SELECT, INSERT ON casework.capacity_runs TO akr_app;
GRANT UPDATE (status, result, completed_at) ON casework.capacity_runs TO akr_app;
GRANT SELECT, DELETE ON casework.capacity_runs TO akr_retention;
GRANT SELECT, INSERT ON casework.rfi_packs TO akr_app;

GRANT SELECT, INSERT ON rpt.typology_exposure TO akr_app;
GRANT UPDATE (flows, value_lcy, total_flows, total_value_lcy, assets_lcy, suppressed, computed_at) ON rpt.typology_exposure TO akr_app;
GRANT SELECT ON rpt.typology_exposure TO akr_retention;   -- R7 report_only (DR-386)

GRANT SELECT, INSERT ON admin.directory_users TO akr_app;
GRANT UPDATE (user_name_enc, display_name_enc, state, directory_active, linked_user_id, linked_by, linked_at, updated_at, version)
    ON admin.directory_users TO akr_app;
GRANT SELECT, INSERT ON admin.directory_events TO akr_app;
GRANT SELECT, DELETE ON admin.directory_events TO akr_retention;

-- Feature flags: every part is off until the bank switches it on (phase 26 and 29 pattern; a route answers 404 E_NOT_FOUND).
INSERT INTO admin.feature_flags (key, enabled, scope, label) VALUES
 ('pack.training', FALSE, '{}', '{"ar": "مساحة التدريب بحالات تجريبية", "en": "Training workspace with synthetic cases (phase 31)"}'),
 ('pack.capacity', FALSE, '{}', '{"ar": "مركز الطاقة الاستيعابية", "en": "Capacity workbench and simulation (phase 31)"}'),
 ('pack.exposure', FALSE, '{}', '{"ar": "مؤشر تعرض الأنماط", "en": "Typology exposure index (phase 31)"}'),
 ('pack.correspondent', FALSE, '{}', '{"ar": "حزم المراسلين والمدققين وطلبات المراسلين", "en": "Correspondent and validator packs and correspondent requests (phase 31)"}'),
 ('pack.polis', FALSE, '{}', '{"ar": "الدخول الموحد SAML والمزامنة SCIM", "en": "SAML and SCIM through Ory Polis (phase 31)"}')
ON CONFLICT (key) DO NOTHING;

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('train.pass_pct', '70', 'integer',
  '{"min": 60, "basis": "a practice attempt that chooses the right decision with fewer than 60 percent of the required reasons does not show competence"}', TRUE,
  '{"ar": "نسبة النجاح في حالة التدريب", "en": "Pass mark of a practice attempt (percent of required reasons)"}'),
 ('capacity.horizon_weeks', '4', 'integer',
  '{"min": 4, "basis": "research 28 section 11.4: the capacity forecast looks four weeks ahead; a shorter horizon hides a building backlog"}', TRUE,
  '{"ar": "أفق توقعات الطاقة (أسابيع)", "en": "Capacity forecast horizon (weeks)"}'),
 ('capacity.occupancy_max', '0.85', 'decimal',
  '{"max": 0.90, "basis": "research 28 section 11.2: the wait before work rises sharply above 90 percent occupancy"}', TRUE,
  '{"ar": "أقصى نسبة إشغال مستهدفة", "en": "Highest occupancy target an input may carry"}'),
 ('capacity.min_working_days', '20', 'integer',
  '{"min": 10, "basis": "a projection from fewer measured working days of handling time is not shown; no assumed handling time is used"}', TRUE,
  '{"ar": "أقل عدد أيام عمل مقاسة قبل عرض التوقعات", "en": "Measured working days needed before a projection is shown"}'),
 ('capacity.surge_active', 'false', 'boolean', NULL, TRUE,
  '{"ar": "وضع الطوارئ مفعّل (رفع العينات فقط)", "en": "Surge mode is active (raises quality sampling only)"}'),
 ('capacity.surge_qc_percent', '15', 'decimal',
  '{"min": 5, "basis": "a surge raises quality-control sampling and never lowers the bank normal rate, which the floor keeps at five percent"}', TRUE,
  '{"ar": "نسبة عينة الجودة أثناء الطوارئ", "en": "Quality-control sample rate while a surge is active (percent)"}'),
 ('exposure.trend_months', '3', 'integer', NULL, FALSE,
  '{"ar": "عدد الأشهر السابقة لحساب الاتجاه", "en": "Earlier months averaged for the exposure trend"}'),
 ('exposure.trend_band_pp', '0.5', 'decimal',
  '{"min": 0.1, "basis": "a smaller band calls monthly noise a trend; percentage points of the flow share"}', TRUE,
  '{"ar": "نطاق الاتجاه المعتبر ثابتاً (نقاط مئوية)", "en": "Band inside which the exposure trend is flat (percentage points)"}'),
 ('directory.event_max_age_s', '600', 'duration_s',
  '{"max": 900, "basis": "a signed directory event older than fifteen minutes is stale; a longer window lets a replayed event wait"}', TRUE,
  '{"ar": "أقصى عمر لحدث الدليل (ثوان)", "en": "Oldest directory event accepted (seconds)"}'),
 ('directory.pending_alert_days', '14', 'integer',
  '{"min": 7, "basis": "directory users waiting for a link are listed to the security officer after this many days"}', TRUE,
  '{"ar": "أيام انتظار المستخدم الوارد من الدليل قبل التنبيه", "en": "Days a directory user waits for a link before the security officer is told"}'),
 ('schedule.capacity_snapshot', '"0 5 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول لقطات الطاقة الاستيعابية", "en": "Capacity snapshot schedule (hourly)"}'),
 ('schedule.capacity_watch', '"0 20 6 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مراقبة توقعات الطاقة", "en": "Capacity forecast watch schedule (daily)"}'),
 ('schedule.exposure_compute', '"0 30 1 2 * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول حساب مؤشر التعرض الشهري", "en": "Typology exposure monthly schedule"}'),
 ('schedule.directory_reconcile', '"0 15 3 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مطابقة دليل المستخدمين", "en": "Directory reconciliation schedule (daily)"}')
ON CONFLICT (key) DO NOTHING;

-- Column classification (copy of the phase 29 format). Encrypted columns are personal and sealed (phase 02).
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'train', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('practice_cases', 'internal', ARRAY['id', 'case_code', 'course_code', 'path', 'typologies', 'title', 'pack_version', 'active', 'created_at']),
  ('practice_cases', 'detection-logic', ARRAY['known_answer', 'expert_path']),
  ('practice_cases', 'internal', ARRAY['facts']),
  ('practice_attempts', 'internal', ARRAY['id', 'case_id', 'user_id', 'decision', 'choice', 'reason_codes', 'decision_match',
                                          'reasons_matched', 'passed', 'attempted_at'])
) AS v(t, cls, cols) CROSS JOIN LATERAL unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'casework', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('capacity_inputs', 'internal', ARRAY['id', 'pool_id', 'week_start', 'analysts_staffed', 'hours_per_day', 'shrinkage',
                                        'occupancy_target', 'note_code', 'entered_by', 'entered_at', 'superseded_by', 'superseded_at', 'version']),
  ('capacity_snapshots', 'internal', ARRAY['id', 'pool_id', 'bucket_start', 'work_kind', 'class_code', 'arrivals', 'decided',
                                           'handling_minutes_sum', 'backlog_end', 'aged_over_sla', 'computed_at']),
  ('capacity_runs', 'internal', ARRAY['id', 'replay_id', 'label', 'inputs', 'status', 'result', 'requested_by', 'requested_at', 'completed_at']),
  ('rfi_packs', 'internal', ARRAY['rfi_id', 'pack_id', 'linked_by', 'linked_at'])
) AS v(t, cls, cols) CROSS JOIN LATERAL unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'rpt', 'typology_exposure', c, 'internal'::admin.data_classification
FROM unnest(ARRAY['id', 'period_month', 'typology', 'scope_kind', 'scope_ref', 'flows', 'value_lcy', 'total_flows',
                  'total_value_lcy', 'assets_lcy', 'suppressed', 'computed_at']) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('directory_users', 'internal', ARRAY['id', 'external_id', 'state', 'directory_active', 'linked_user_id', 'linked_by', 'linked_at',
                                        'created_at', 'updated_at', 'version']),
  ('directory_users', 'personal', ARRAY['user_name_enc', 'display_name_enc']),
  ('directory_events', 'internal', ARRAY['id', 'event_ref', 'directory_user_id', 'op', 'payload_sha256', 'outcome', 'reason_code', 'received_at'])
) AS v(t, cls, cols) CROSS JOIN LATERAL unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;
UPDATE admin.column_classification SET encrypted = TRUE
 WHERE (schema_name, table_name, column_name) IN (
   ('admin', 'directory_users', 'user_name_enc'),
   ('admin', 'directory_users', 'display_name_enc'));

-- Retention. R9 for configuration-like evidence kept with the staff or the bank, R12 for operations logs, R7 for
-- statistics, R6 for what follows a case. A clock column is set exactly for partition, age, closure and decision.
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('train', 'practice_cases',      'R9',  'retirement', NULL,          'Synthetic cases; kept while their pack version is in use'),
 ('train', 'practice_attempts',   'R9',  'age',        'attempted_at', 'Practice attempts of staff, kept with the training evidence (period: Open doubts 8)'),
 ('casework', 'capacity_inputs',  'R9',  'retirement', NULL,          'Staffing inputs by pool and week: the basis of every projection shown'),
 ('casework', 'capacity_snapshots', 'R12', 'age',     'computed_at', 'Hourly operations measures without persons'),
 ('casework', 'capacity_runs',    'R12', 'age',        'requested_at', 'Simulation runs: operations evidence of the capacity decision'),
 ('casework', 'rfi_packs',        'R6',  'owner',      NULL,          'Follows its RFI'),
 ('rpt', 'typology_exposure',     'R7',  'age',        'computed_at', 'Statistics derived from posted transactions and alerts'),
 ('admin', 'directory_users',     'R9',  'retirement', NULL,          'Directory rows kept after the account is retired (who was provisioned when)'),
 ('admin', 'directory_events',    'R12', 'age',        'received_at', 'Directory event log; the audit chain keeps the evidence of each change')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

---

## 2. DDD Implementation (Rust)

Crate placement follows phase 00 (`akr-api` for routes and services that answer requests, `akr-worker` for the jobs; the domain types sit in the `domain` module of the crate that owns the table). Names were grepped in docs/plan before use: `TrainingService`, `CapacityService` and `CorrespondentPackService` are the names of the roadmap row 31; every other name below is new in phase 31 and occurs nowhere else in docs/plan.

### 2.1 Domain types

- `PracticeCase { id, case_code, course_code, path: PracticePath, typologies: Vec<i16>, title: Bilingual, facts: Value, pack_version, active }`. `PracticeCaseView` is the trainee's form: no `known_answer`, no `expert_path`. `PracticeCaseAnswer` (known answer and expert path) is returned only inside an `AttemptResult`.
- `PracticeAttempt { id, case_id, user_id, decision: Option<PracticeDecision>, choice: Option<u8>, reason_codes: Vec<String>, decision_match: bool, reasons_matched: u8, passed: bool, attempted_at }`. `PracticeDecision` is the closed set `no_report`, `utr`, `str`, `tfr` (phase 18 outcome words).
- `CapacityInput { id, pool_id, week_start: NaiveDate, analysts_staffed: Decimal, hours_per_day: Decimal, shrinkage: Decimal, occupancy_target: Decimal, note_code: Option<CapacityNote>, entered_by, entered_at, superseded_at, version }`. `CapacityNote` is the closed set of the table CHECK.
- `CapacitySnapshot { pool_id, bucket_start, work_kind: WorkKind, class_code, arrivals, decided, handling_minutes_sum, backlog_end, aged_over_sla }`. `WorkKind` is `alert` or `case`. There is no person type in this module.
- `CapacityRun { id, replay_id, label, inputs: SimulationInputs, status: RunStatus, result: Option<SimulationResult>, requested_by, requested_at, completed_at }`.
- `TypologyExposureRow { period_month, typology: i16, scope: ExposureScope, flows: Option<i64>, value_lcy: Option<Decimal>, total_flows: Option<i64>, total_value_lcy: Option<Decimal>, assets_lcy: Option<Decimal>, suppressed: bool }`. `ExposureScope` is `Bank`, `Branch(Uuid)` or `Correspondent(Uuid)`.
- `DirectoryUser { id, external_id, user_name: Sealed<String>, display_name: Option<Sealed<String>>, state: DirectoryState, directory_active, linked_user_id: Option<Uuid>, linked_by, linked_at, version }`, `DirectoryState` is `Pending` or `Linked`. `DirectoryOp` is `Create`, `Update`, `Deactivate`, `Activate`. `DirectoryEvent { id, event_ref, directory_user_id: Option<Uuid>, op, payload_sha256, outcome: EventOutcome, reason_code: Option<EventReason> }`.

DTOs (`Deserialize, ToSchema`; response DTOs `Serialize, ToSchema`): `PracticeCaseList`, `PracticeCaseDetail`, `AttemptBody { decision | choice, reason_codes }`, `AttemptResult { passed, decision_match, reasons_matched, expert_path, known_answer, mark: "TRAINING - synthetic data" }`, `CapacityInputBody`, `CapacityInputView`, `CapacityForecast` (section 3.4), `SimulationBody { replay_id, label, inputs }`, `SimulationView`, `ExposureQuery`, `ExposureView`, `CorrespondentRfiRow`, `AttachPackBody { pack_id }`, `DirectoryEventBody { op, external_id, user_name, display_name?, active, occurred_at, groups? }` (`groups` is accepted and never read, section 3.11), `DirectoryUserView`, `LinkBody { user_id }`.

### 2.2 Repository traits

- `PracticeRepository`: `active_cases(path: Option<PracticePath>, cursor) -> Vec<PracticeCase>`; `case(id) -> Option<PracticeCase>`; `insert_attempt(tx, &PracticeAttempt)`; `own_attempts(user_id, cursor)`; `course_scores(user_ids: &[Uuid]) -> HashMap<(Uuid, String), PracticeScore>` (reads `train.practice_scores`, section 3.2).
- `CapacityRepository`: `live_input(tx, pool_id, week_start) -> Option<CapacityInput>` (`FOR UPDATE`); `insert_input(tx, &CapacityInput)`; `supersede_input(tx, old_id, new_id, at)`; `inputs(pool_ids, weeks) -> Vec<CapacityInput>`; `upsert_snapshots(tx, &[CapacitySnapshot])` (`ON CONFLICT (pool_id, bucket_start, work_kind, class_code) DO UPDATE`); `window_aggregate(from, to) -> Vec<ClassAggregate>` (sums by pool, work kind and class; never by person); `insert_run(tx, &CapacityRun)`; `run(id) -> Option<CapacityRun>`; `start_run(tx, id)` (sets `running`); `finish_run(tx, id, status, result, completed_at)`; `replay_daily_counts(replay_id) -> Vec<(NaiveDate, i64)>` (per `business_date`, the number of `scn.replay_results` rows of the replay with `would_alert = TRUE`; it selects no other column).
- `ExposureRepository`: `upsert_rows(tx, &[TypologyExposureRow])`; `rows(period_from, period_to, scope_kind, scope_ref) -> Vec<TypologyExposureRow>`.
- `DirectoryRepository`: `lock_by_external_id(tx, external_id) -> Option<DirectoryUser>` (`FOR UPDATE`); `insert_pending(tx, &DirectoryUser)`; `update_profile(tx, id, names, version)`; `set_active(tx, id, active, version)`; `link(tx, id, user_id, by, at, version)`; `list(filter, cursor) -> Vec<DirectoryUser>`; `insert_event(tx, &DirectoryEvent)`; `pending_older_than(days) -> Vec<DirectoryUser>`; `linked_inactive() -> Vec<DirectoryUser>` (reconciliation).

### 2.3 Services

- `TrainingService` (`akr-api`): `list_cases(ctx, path, cursor)`, `get_case(ctx, id)`, `attempt(ctx, case_id, body, idem) -> AttemptResult`, `own_attempts(ctx, cursor)`, and `practice_column(ctx, user_ids)` used by phase 24 `TrainingRegisterService::list` (section 4.5).
- `CapacityService` (`akr-api`): `inputs(ctx, query)`, `record_input(ctx, body, if_match, idem)`, `forecast(ctx)`, `request_simulation(ctx, body, idem)`, `simulation(ctx, id)`.
- `CapacitySnapshotter`, `CapacityWatcher`, `SimulationHarness` (`akr-worker`, job handlers of section 4.1). Their arithmetic is pure functions in the domain module: `workload_hours`, `fluid_projection`, `sensitivity`, `clock_breach_estimate`; each has property tests (section 6).
- `ExposureService` (`akr-api` for `read`; `akr-worker` for `compute`): `compute(period_month) -> ExposureRunSummary` (`ExposureRunSummary { period_month, rows, suppressed_rows }`: the month, the rows upserted and those of them with `suppressed = TRUE`, which is the detail of `exposure.computed`), `read(ctx, query) -> ExposureView`.
- `CorrespondentPackService` (`akr-worker`, registered with phase 02 `PackService` through the phase 06 `pack_build` handler): `CorrespondentFacts::build(period)` and `ValidatorFacts::build(period)`, each returning the manifest and the bilingual body of section 3.7.
- `DirectoryService` (`akr-api`; `directory_reconcile` in `akr-worker`): `event(verified: VerifiedWebhook, body, idem)`, `list(ctx, filter, cursor)`, `link(ctx, id, body, assertion, if_match, idem)`, `reconcile(now)`.
- `DirectoryWebhookVerifier` (`akr-api`, pure): `verify(headers, body, now, key) -> Result<VerifiedWebhook, DirectoryError>` (section 3.11 step 1).
- `UserAdminService::disable_from_directory` (extends phase 03 `UserAdminService::disable`, sections 3.12 and 4.5).
- `RfiService::attach_pack` and `RfiService::correspondent_list` (extend phase 19 `RfiService`, section 4.5).

### 2.4 Routes

Each router is `pub fn router() -> Router<AppState>`: `training::router()` (four routes), `capacity::router()` (five rows, one owned), `exposure::router()` (one row, owned), `rfis::router()` (phase 19; two routes added here) and `directory::router()` (three routes). Rows marked "owned" are API.md 3.16 rows 726 and 727, which phase 31 implements; rows marked "added" are new in this phase, with the reason. The total is 15 routes: 13 added and 2 owned. `POST /packs` keeps its phase 06 route and accepts the two kinds through the phase 06 `pack_build` job; it is not counted again.

| Method | Path | Scope | Description (Cedar action) |
|-|-|-|-|
| GET | /training/practice/cases | queue.work | Active synthetic cases of the caller's paths, cursor, no answers (`train.practice.read`; flag `pack.training`) (added: the workspace is new; W12) |
| GET | /training/practice/cases/{id} | queue.work | One case: facts, question, reason list; no known answer (`train.practice.read`) (added) |
| POST | /training/practice/cases/{id}/attempts | queue.work | Submit a decision or a choice; scored against the known answer; returns the expert path; `Idempotency-Key` (`train.practice.attempt`) (added) |
| GET | /training/practice/attempts | queue.work | The caller's own attempts, cursor (`train.practice.own`) (added) |
| GET | /capacity/inputs | dashboard.read | Staffing inputs by pool and bank week (`capacity.inputs.read`) (added: the inputs are the basis of the forecast) |
| POST | /capacity/inputs | capacity.plan | Record the input of a pool and week, or supersede the live one; `If-Match` on the live version; `Idempotency-Key` (`capacity.inputs.write`) (added; scope new, section 3.16) |
| GET | /capacity/forecast | dashboard.read | Workload, backlog projection, sensitivity table, hourly surge progress; live, `no-store`, team-level only (`capacity.forecast.read`) (owned: API.md 3.16 row 726) |
| POST | /capacity/simulations | capacity.plan | Start the simulation harness on a completed phase 16 replay; 202; `Idempotency-Key` (`capacity.simulation.run`) (added) |
| GET | /capacity/simulations/{id} | dashboard.read | Status and result: queue peak, clock breaches, analyst hours by week and class (`capacity.simulation.read`) (added) |
| GET | /typology-exposure | dashboard.read | Typology exposure by bank, branch and correspondent, with trend; suppressed cells null (`exposure.read`) (owned: API.md 3.16 row 727) |
| GET | /rfis/correspondents | rfi.desk | Correspondent requests in and out: deadline, answer progress, recipient account id; no case id, family, priority or tier (`rfi.correspondent.list`) (added: W8 correspondent side) |
| POST | /rfis/{id}/packs | rfi.desk | Attach a correspondent or validator pack to an outbound correspondent request in draft; `If-Match`; `Idempotency-Key` (`rfi.attach_pack`) (added) |
| POST | /directory/events | directory.sync | Directory event from Ory Polis (create, update, deactivate, activate); HMAC-verified webhook; `Idempotency-Key` = event reference (`directory.event`) (added: SCIM, G-12) |
| GET | /directory/users | admin.users | Directory users with link state, cursor; names shown to ADM only (`directory.read`) (added) |
| POST | /directory/users/{id}/link | admin.users | Link a pending directory user to an active account; step-up class `identity_admin`; `If-Match` (`directory.link`) (added) |

Every route of this phase answers `404 E_NOT_FOUND` while its flag is off (section 4.4). `directory.sync` is a new scope (API.md section 2 gains the row, section 3.16); `capacity.plan` is a new scope (M only).

---

## 3. Business Logic

### 3.1 Practice cases and attempts (`TrainingService`; UX-41, M-51 part; routes 1 to 4)

**Authoring** (`akr-ctl training seed --file FILE`, as `akr_migrate`; no route). The file is a signed pack document (phase 02 pack signature). For each case:
1. Refuse the file unless every case has `facts.synthetic = true` and a `case_code` of the table pattern (422 in the command: `not_synthetic`, `case_code`).
2. Refuse any id or value of the facts that equals an existing identifier of `canon.*`, `casework.*` or `rpt.*` (`SELECT 1` per table; the seed refuses the file, nothing is inserted). The check exists because the fixture layout of phase 16 section 3.12 (rows by canonical table with stable ids) may be reused as a shape, and a real id must never reach a practice case.
3. Every number in `typologies` appears in `scn.scenarios.typologies` of the installed pack (phase 06), and every reason code of `known_answer.reason_codes` exists in `ref.reason_codes` with family `decision` (phase 09).
4. Insert the rows; a changed case is a new `case_code` (rows are never updated, except `active`, which only `akr_migrate` may set). Audit `train.cases_seeded {pack_version, cases}`.

The content source is the typology library (research 27, the scenario `typologies` of phase 06) and the case structure of phase 16. The golden set of phase 28 (QA-confirmed live cases) is not a source: it is real case data (DR-340).

**Attempt** (`POST /training/practice/cases/{id}/attempts`; Cedar `train.practice.attempt`; scope `queue.work`; `Idempotency-Key`):
1. Flag `pack.training`: off means 404 `E_NOT_FOUND`.
2. Load the case. Unknown id: 404. `active = FALSE`: 409 `E_NOT_ALLOWED_IN_STATE` sub-code `case_retired`.
3. Body: exactly one of `decision` (one of `no_report`, `utr`, `str`, `tfr`) or `choice` (1 to 9). Neither or both: 422 `E_VALIDATION` sub-code `decision_or_choice`. A decision for a case whose known answer is a choice (or the reverse): 422 sub-code `answer_kind`.
4. `reason_codes`: at most 10, each in `ref.reason_codes` family `decision`: 422 sub-code `reason_unknown`, or `reason_limit`.
5. Score. `decision_match` is the equality of the chosen decision or choice with the known answer. `required` is `known_answer.reason_codes` (possibly empty). `reasons_matched` is `round(100 * |chosen intersect required| / |required|)`, and 100 when `required` is empty. Extra codes do not lower the score; they are shown back to the trainee as "not required". `passed = decision_match AND reasons_matched >= train.pass_pct`.
6. Insert `train.practice_attempts`. Audit `train.practice_attempted {case_code, attempt_id, passed}`. Nothing else is written: no outbox topic, no notice, no row outside `train.*`, `admin.idempotency_keys` and `audit.audit_log`.
7. Response 201 `AttemptResult`: `passed`, `decision_match`, `reasons_matched`, the `known_answer` and the `expert_path` (sent only now, after this trainee's own attempt), and the mark `"TRAINING - synthetic data"`. The response carries `Cache-Control: no-store`.

**Case list and detail** (routes 1 and 2): `path` filter (optional), cursor; the list and the detail return `PracticeCaseView` (title, facts, reason list, path, course code, typologies), never `known_answer` or `expert_path`. Every response carries the mark.

**Own attempts** (route 4): filter `user_id = caller` is applied in the SQL, not in the response; there is no parameter that names another user (a 404 for another person's attempt id).

### 3.2 Isolation of the training data and the register (DR-338, DR-339)

Rules, each one tested in section 6 item 4:
1. Only `TrainingService` writes `train.*`. No SQL outside the `training` module names schema `train`, except `TrainingService::practice_column` (rule 4).
2. No table, view or function outside schema `train` references schema `train` (`pg_depend` and `pg_views` check, section 6 item 4). The one exception is the display column of rule 4, which is code, not a view.
3. No count, tile, register total, export, report, pack, goAML file or audit aggregate reads `train.*`. The phase 24 tiles (section 3.7 of phase 24), registers (section 3.4) and the training register counts (section 3.10) return the same numbers with and without practice rows.
4. The training register (`GET /training/register`, phase 24 section 3.10) gets one display column, `practice`: for each listed user and course, `TrainingService::practice_column` reads `train.practice_scores` for the users on the page and returns `passed_first`, `cases_passed_first`, `cases_attempted` and `attempts`. `passed_first` is TRUE when at least one case of the course was passed on its first attempt. It is not a course pass: the two counts beside it are the measure. Only each user's first attempt per case is scored: the first attempt returns the known answer and the expert path, so a later pass of the same case is practice and stays in `train.practice_attempts` without counting. It is a column beside the record, never a count and never a record.
5. A practice pass never creates a training record. Recording a completion stays the phase 24 act of an administrator for another person (`training.no_self`, CHECK `user_id <> recorded_by`). An administrator who credits a pass writes `evidence_ref = 'practice:' || attempt_id` in `POST /training/register` (phase 24 route 30); the text is a reference, not a foreign key, so the register does not depend on `train.*` for its integrity.
6. Retention: `train.practice_attempts` leave only through the retention run of phase 23 (`akr_retention`; the guard of `0100` admits no other deletion). Holds of phase 23 that name a person hold that person's attempts.

Why a schema and not a flag column: a flag on the live tables (`casework.cases`, `alert.alerts`, `admin.training_records`) would have to be excluded by every count, report, export, register and goAML filing, and one missed predicate would publish practice cases as live. A separate schema makes the exclusion a fact of the database (no join path) instead of a discipline of the queries.

### 3.3 Capacity inputs and measured handling (`CapacityService`, `CapacitySnapshotter`; M-52; DR-341)

**Inputs** (`POST /capacity/inputs`; Cedar `capacity.inputs.write`; scope `capacity.plan`; M only):
1. Flag `pack.capacity`; `Idempotency-Key`. If the pool and week already have a live input, `If-Match` must carry its `version`: missing 428 `E_PRECONDITION_REQUIRED`, different 409 `E_VERSION_CONFLICT` with the current version in the body.
2. Validate: `week_start` is a Sunday (CHECK); `occupancy_target` at most `capacity.occupancy_max` (422 sub-code `occupancy_above_max`); `shrinkage` at most 0.600 (CHECK); `analysts_staffed` is a count, not a list of names (the body has no name field).
3. Lock order (section 3.18): the pool row is read, then the live input row `FOR UPDATE`.
4. Insert the new row with `version = live.version + 1` (or 1). If a live row exists, set its `superseded_by` and `superseded_at` in the same transaction; the partial unique index keeps one live row per pool and week.
5. Audit `capacity.input_recorded {pool_id, week_start, input_id, superseded_id}`. Response 201 `CapacityInputView` (`ETag` = the version).

**Measured handling** (`CapacitySnapshotter`, job `capacity_snapshot`, hourly, section 4.1). For each complete bank hour not yet snapshotted (and re-computed for the previous hour, which can still receive late events):
1. Alerts. Phase 17 gives alerts no pool (queues have `pool_id` and `filter`; only cases have `assignee_pool_id`), so this phase sets its own attribution rule, new in phase 31 and used for capacity only: an alert counts, once, in the pool of the first active `alerts` queue, ordered by `code`, whose filter admits it. An alert admitted by no queue is counted in an `unassigned` row (`pool_id` NULL) that G2 shows as a total without a pool; the forecast, the sensitivity table and the alarm read pool rows only (Open doubts 4).
   - `arrivals`: alerts with `created_at` in the hour, grouped by pool and `priority_class` (P0 to P3).
   - `decided`: alerts with `triaged_at` in the hour and `state = 'triaged'`.
   - `handling_minutes_sum`: the sum over those alerts of `triaged_at - created_at` in minutes (elapsed time, never paused: research 28 section 11.5).
   - `backlog_end`: alerts of the pool and class in state `queued` or `claimed` at the end of the hour.
   - `aged_over_sla`: backlog alerts whose `touch_due_at` (phase 17) is already past.
2. Cases. `arrivals` are cases with `opened_at` in the hour, grouped by `assignee_pool_id` and `family`; cases with no pool are not counted in a team row. `decided` are cases with `closed_at` in the hour; `handling_minutes_sum` is `closed_at - opened_at` in minutes; `backlog_end` are open cases; `aged_over_sla` are open cases older than 60 calendar days (research 28 section 11.4, the default of its ageing alarm; Open doubts 4).
3. Upsert keyed by `(pool_id, bucket_start, work_kind, class_code)`. A re-run replaces the values of the bucket and does nothing else. The snapshot is a derived rebuild of counts, so it writes no audit row (section 3.14 lists none for it), as the anchors of phase 02 write none; `computed_at` is its record.
4. The job's SQL selects no user column and writes no user column (test, section 6 item 7). There is no per-person measure anywhere in the snapshot.

**Measured, not assumed.** The product has no default handling time. The research 28 worked example (36.9 minutes a case, 61.4 hours a day) is an illustration and is never loaded as a value. Until `capacity.min_working_days` bank working days of decided items exist for a class, that class has no measure and the forecast says so (section 3.4 step 5).

### 3.4 Forecast, sensitivity and the capacity alarm (`CapacityService::forecast`, `CapacityWatcher`; M-52, M-53 part; D21)

**Forecast** (`GET /capacity/forecast`; Cedar `capacity.forecast.read`; scope `dashboard.read`; `Cache-Control: no-store`). The window is the last 28 days of snapshots (four bank weeks). For each pool with a live input and each class:
1. Rates. `W` is the number of bank working days in the window that have snapshots (Sunday to Thursday, phase 09 calendar). `a_pc = arrivals / W` per working day. `m_pc = handling_minutes_sum / decided` (minutes), only when `decided >= 1`; otherwise the class has no measure.
2. Workload (research 28 section 11.2): `L_p = sum over c of a_pc * m_pc / 60` hours per working day.
3. Available hours: `C_p = analysts_staffed * hours_per_day * (1 - shrinkage) * occupancy_target` hours per working day, from the live input of the current week.
4. Required analysts: `R_p = L_p / (hours_per_day * (1 - shrinkage) * occupancy_target)`.
5. Insufficient data. If `W < capacity.min_working_days` or no class of the pool has a measure, the pool answers `status: "insufficient_data"` with its counts; no projection, no sensitivity and no alarm is produced.
6. Fluid backlog over `H = capacity.horizon_weeks * 5` working days. Start `B_0` is the sum of `backlog_end` in the last bucket. Per working day `B_(t+1) = max(0, B_t + sum over c of a_pc - mu_p)`, where `mu_p = C_p * 60 / mbar_p` and `mbar_p` is the arrival-weighted mean of `m_pc`. Below capacity the backlog grows linearly until someone acts (research 28 section 11.2); the response also gives `L = lambda * W` on the last bucket (Little's law) as a check of the measured figures.
7. Sensitivity (fixed grid, not a user input): handling time +20 percent; arrivals +25 percent; both; occupancy 0.80, 0.85 and 0.90 (each capped at `capacity.occupancy_max`). Each cell is `R_p` recomputed with the multipliers of steps 2 to 4.
8. Suppression. A pool with fewer than `report.small_number_threshold` analysts staffed shows no pool row (its figures would describe a few people): the response lists the pool id only. A class with fewer than `report.small_number_threshold` decided items shows its measure as `null` with `suppressed: true`.
9. Surge progress. The last 24 hourly totals per pool (arrivals and decided, all classes together) and the state of `capacity.surge_active`.
10. Output per pool: staffed analysts, `L_p`, `R_p`, the backlog path by week, the sensitivity table, the classes with no measure by code, and `data_as_of` (the latest `computed_at`). No key of the response names a person: the test of section 6 item 5 checks the key set.

**Alarm** (`CapacityWatcher`, job `capacity_watch`, daily 06:20; idle while `pack.capacity` is off). For each pool with a live input for the current week and whose status is not `insufficient_data`:
1. Compute steps 1 to 6 of the forecast for the horizon.
2. Raise when the backlog at the horizon end is larger than `B_0` and `R_p` over the horizon is larger than `analysts_staffed` (forecast hours exceed available hours in the next horizon weeks, research 28 section 11.4).
3. `AlarmService::raise(tx, NewAlarm { kind: capacity_forecast, subject_kind: pool, subject_id: pool_id, stage: NULL })` (phase 19 section 3.10). The stage stays NULL, as the phase 19 CHECK requires for every kind other than the two breaches; the live index `uq_alarms_live` is on `COALESCE(stage, 0)`, so a repeat of the same kind and pool returns `None` and writes nothing. The supervisor is M. Phase 22 notice `supervisor.alarm` with `alarm_kind = capacity_forecast` (no new notice code). A person closes the alarm through the phase 19 acknowledgement route.

**Advice only (D21).** Nothing in this phase is an input to detection. The jobs of section 4.1 read no table of schema `alert` for write, no `scn.*` for write, no `admin.settings` for write, and do not read `casework.touch_by_hours`, `casework.wip_limit` or any threshold for write. The forecast does not set a queue limit, a work-in-progress limit or a touch window. A rule or threshold change prompted by a capacity result is a phase 04 change request with the `capacity` reason tag and its below-the-line evidence (S-39), exactly as before this phase. Section 6 item 7 proves the write-set.

### 3.5 Simulation harness (`SimulationHarness`; M-76; DR-342)

**Request** (`POST /capacity/simulations`; Cedar `capacity.simulation.run`; scope `capacity.plan`; `Idempotency-Key`):
1. Flag `pack.capacity`. The body: `replay_id`, `label` (1 to 80 characters), and `inputs`: `pools` (one to five pool ids), `weeks` (1 to 8), `class_mix` (shares of P0 to P3 that sum to 1; required, with no default: the requester states the mix, DR-342 and Open doubts 5; a missing mix or one that does not sum to 1 is 422 `E_VALIDATION` at `/inputs/class_mix`), `touch_hours` (default `casework.touch_by_hours`), `analysts` (per pool; default the live `analysts_staffed` of the first week), and `handling_minutes` (optional override per class; any override is listed in the result as `assumed`).
2. The replay must exist and be `completed` (409 `E_NOT_ALLOWED_IN_STATE` sub-code `replay_not_completed`). Its window must not exceed `weeks` (422 sub-code `period_too_long`). The replay is phase 16 `POST /scenarios/{id}/replays` (cost guard included); this phase does not start one.
3. Insert `casework.capacity_runs` with `status = 'queued'`; enqueue `capacity_simulation {run_id}`; audit `capacity.simulation_requested {run_id, replay_id}`; response 202 with `Location: /capacity/simulations/{id}`.

**Job** `capacity_simulation` (`akr-worker`, concurrency 2):
1. Mark the run `running` (`CapacityRepository::start_run`). This write has no audit row of its own: the pair `capacity.simulation_requested` and `capacity.simulation_completed` of section 3.14 brackets the run, and step 8 records how it ended. Re-read the replay with phase 16 `ReplayRepository::get`; a replay that is no longer `completed` fails the run.
2. Read `scn.replay_results` of the replay through `CapacityRepository::replay_daily_counts`, for the columns `business_date` and `would_alert` only (no `entity_id`, no `detection_key`, no `segment_ref` text in the output). `A_d` is the would-alert count per business date; the working-day rate is the mean of `A_d` over the window's working days.
3. Split by `class_mix` into daily arrivals `a_c` per class. The replay carries no priority class, so the split is the input (Open doubts 5).
4. Per working day and class, the service rate is `mu_c = analysts_p * hours_per_day * (1 - shrinkage) * occupancy_target * 60 / m_c`, with `m_c` the measured mean handling of section 3.4 (or the override). Capacity is spent in priority order P0, P1, P2, P3 (the phase 17 priority rule): `served_c = min(backlog_c + a_c, remaining capacity)`. Queue peak per class and in total.
5. Clock breaches (estimate, labelled as such): `breach_c,t = max(0, backlog_c,t - mu_c * touch_days_c)` where `touch_days_c = touch_hours_c / hours_per_day`. This is a fluid estimate, not a discrete queue; the result says so.
6. Analyst hours by week and class: `sum over the week of a_c * m_c / 60` (research 28 section 11.2), beside the required analysts per week.
7. Result: `queue_peak_by_class`, `queue_peak_total`, `breaches_by_class`, `analyst_hours_by_week_class`, `required_analysts_by_week`, `assumed` (every override) and `caveats` (fluid estimate; no order inside a class; no waits for information; the replay's alerts are the candidate's, not the live ones). Every count below `report.small_number_threshold` is `null`.
8. Write `status = 'completed'`, `result` and `completed_at`; audit `capacity.simulation_completed {run_id, status}`. On error: `status = 'failed'`, no result, audit with the same kind and `status = 'failed'`.

**Writes:** only `casework.capacity_runs`. A test counts rows of `alert.*`, `scn.*` and `casework.alarms` before and after a run: no change (section 6 item 14). A result activates nothing: no scenario version, parameter set, threshold or alert is changed. A rule change that relies on a result goes through phase 04 with the phase 16 `replay` evidence of the candidate, as today.

### 3.6 Surge mode, the parts carried (M-54; DR-343)

- **Temporary pool.** An `admin.pools` row created by ADM through the phase 03 pool route, with its reviewers' role grants ending on a date (phase 03 grant expiry). The capacity input of the week names it with `note_code = 'temporary_pool'`.
- **Quality-control sampling.** While `capacity.surge_active` is `TRUE`, the phase 19 QA sampler (section 4.5, item 9) samples at least `capacity.surge_qc_percent` percent of the population of a draw sized from the settings. Outside a surge the rate is unchanged.
- **Hourly progress** to the MLRO: the surge progress block of section 3.4 step 9 on G2. No notice is sent per hour (no new notice code).
- **Activation.** `capacity.surge_active` is a governed setting. Turning it on or off is a change request (phase 04 setting target) with reason tag `capacity`. The phase 04 rule applies as written: the change is at least material, needs a `btl_sample` evidence item and an approver outside operations. The change never suppresses an alert, so the below-the-line sample shows no suppression.
- **Not carried: pre-clear from pair memory.** Surge mode in research 28 section 11.3 pre-clears known false positives from pair memory. Pair memory (phase 14 section 3.7) suppresses screening candidates only, pair by pair, under four-eyes approval; pre-clearing alerts would suppress them in bulk on a volume argument. That is the alert suppression D21 forbids. Not carried; Open doubts 6.

### 3.7 Correspondent and validator packs (`CorrespondentPackService`; W-06, GV-35; DR-345)

**Kinds and build.** `correspondent` and `validator` are created by `POST /packs` (phase 06 route; the CHECK of `admin.packs` already lists both, phase 06 and phase 24) and built by the phase 06 `pack_build` job through `CorrespondentFacts::build(period)` and `ValidatorFacts::build(period)`. `period` is a bank month (`YYYY-MM`). Scope `{period}`: the body of `POST /packs` is the phase 06 `PackRequest` as `{kind, scope: {period}}`, answered by the phase 06 202 `PackAccepted`; `lang` is left out, and the build publishes one pack row per language. Languages `ar` and `en`; `RenderService` (phase 02) renders both; the SHA-256 is in the `pack.created` audit row; a rebuild is the next version of `(kind, scope_key, lang)`. Cedar: create for M only (section 3.17, E2).

**Facts of `correspondent`** (each fact names its source and `data_as_of`; aggregates only):
- F1 Tool: product name and the `release_version` of the live release (`admin.releases`, phase 25).
- F2 Last calibration: date, sample size and agreement share of the latest QA calibration sample (`casework.qa_samples` with `is_calibration`, phase 19).
- F3 Screening tests: per list kind, the latest `alert.harness_runs` row: `ran_at`, `kind`, `passed_gate`, `recall_clean` (phase 14).
- F4 List-update lag: per list in `ref.lists`, the versions of `ref.list_versions` activated in the period, and for each version the hours from its `loaded_at` to its `activated_at` (phase 14). Phase 14 stores no source publication time; the lag is therefore measured from the load (Open doubts 10).
- F5 Review metrics for the period (phase 19 KPIs): median and 90th-percentile days to decision by class, share closed by deadline, and the QA error rate of `casework.qa_reviews`.
- F6 RFI times: correspondent requests opened and answered in the period and their median days (phase 19).
- F7 Coverage certificates issued in the period: identifier, scope and result (`admin.coverage_certificates`, phase 12).
- F8 Validation packs issued in the period: identifier, verdict and hash (`admin.validation_packs`, phase 24).
- F9 Data quality: open and closed `admin.dq_issues` of the period by severity.
- F10 Exposure, respondent view: the bank scope of section 3.8 for the period only (typology shares; no branch, no correspondent row).

**Facts of `validator`:** F1, F2, F3, F4, F7, F8, and F11: the count of CBI catalogue items by readiness grade (`scn.readiness_grades`, phase 06). It has no F5, F6, F9 or F10.

**Guards.**
1. Every count below `report.small_number_threshold` (phase 21, never below 3) is `null` with `suppressed: true`; a zero stays zero.
2. The builder refuses (`pack.build_failed {reason: "case_reference"}`, phase 24 rule) any value equal to a `casework.cases.id`, `rpt.reports.id`, `canon.party.id` or `canon.account.id`. A test runs this on every build (section 6 item 12).
3. Download is the phase 06 route with step-up class `export` and the `download` access event (phase 24 rule). The pack leaves the bank only through that download.

**Tipping-off review.** The pack names no case, party, account, narrative or customer. A correspondent or validator that receives it learns the bank's tool, controls and bank-level shares. Nothing in it points to a person outside the bank's own staff list, which the pack does not show.

### 3.8 Typology exposure index (`ExposureService`; S-31, SCN-CORR-07; DR-346)

**Definition.** For a month `m`, a scope `s` and a typology `t` (a number of `scn.scenarios.typologies`):
- `T(s,m)`: posted `canon.txn` rows with `business_date` in `m` and in the scope: the bank (all rows); a branch (`branch_id = b`); a correspondent (`account_id = c` where `canon.account.correspondent = TRUE`).
- `total_flows = |T(s,m)|`; `total_value_lcy = sum of amount_lcy over T(s,m)`. Values are LCY only: IQD and USD are never added (T-04).
- `S_t`: scenarios with `t` in `typologies` and `visibility = 'normal'`. Restricted scenarios are excluded for every viewer (DR-346; Open doubts 7).
- `M(s,m,t)`: distinct transactions of `T(s,m)` that appear in `alert.alerts.matched_txn_ids` of alerts created in `m` with `scenario_id` in `S_t`. A transaction matched by two scenarios of one typology counts once.
- `flows = |M|`; `value_lcy = sum of amount_lcy over M`.
- `assets_lcy`: the sum of `balance_lcy` of `canon.balance_snapshot` rows at the last business date of `m` for the scope accounts (correspondent: the account; branch: `canon.account.branch_id = b`; bank: all).
- `share_flows = flows / total_flows`; `share_assets = value_lcy / assets_lcy`.
- `trend`: `delta_pp = 100 * (share_flows(m) - mean of share_flows over the previous exposure.trend_months months)`; `up` above `+exposure.trend_band_pp`, `down` below its negative, `flat` otherwise; `null` when a previous month is missing.

**Suppression.** If `flows < report.small_number_threshold`, `flows`, `value_lcy` and `share_flows` are `null` and `suppressed = TRUE`. If `total_flows` is below the threshold, every share of the scope is `null`. If the scope has no balance snapshot at month end, `assets_lcy` and `share_assets` are `null`. A null never means zero.

**Computation** (`ExposureService::compute(period_month)`, job `exposure_compute`, monthly on the 2nd at 01:30 for the previous month, idle while `pack.exposure` is off):
1. Scopes: the bank; every branch with posted transactions in the month; every correspondent account with posted transactions in the month.
2. For each scope, the totals and, per typology in use, `M(s,m,t)` through one `SELECT` that reads `canon.txn` of the month and `alert.alerts` of the month only (the BRIN index `idx_txn_business_brin` on `business_date`, phase 10, because `canon.txn` is partitioned by `booking_date`; `alert.alerts` is pruned by creation month). No row is read from a restricted scenario's matches into the output.
3. Upsert one `rpt.typology_exposure` row per `(period_month, typology, scope_kind, scope_ref)` (`NULLS NOT DISTINCT`); a re-run replaces values. Audit `exposure.computed {period_month, rows, suppressed_rows}`.

**Read** (`GET /typology-exposure?from=YYYY-MM&to=YYYY-MM&scope=bank|branch:<id>|correspondent:<id>`; Cedar `exposure.read`; scope `dashboard.read`). A branch or correspondent scope requires its id (422 sub-code `scope_required`); a range longer than 24 months is refused (422 `period_range`). The correspondent appears as its `canon.account` id, never a name (the table has none). The response carries the rows, the trend and `data_as_of`; `Cache-Control: no-store`.

**Respondent view.** A correspondent does not log in. Its view is the bank scope of section 3.7 F10 inside the correspondent pack: the typology shares of the bank for the period. The index does not expose another correspondent's row, a branch row or a count below the threshold to any respondent.

### 3.9 RFIs to correspondents (W8; DR-347)

**List** (`GET /rfis/correspondents`; Cedar `rfi.correspondent.list`; scope `rfi.desk`): RFIs with `kind = 'correspondent'`, filter `direction` (`in` or `out`), cursor. Each row: `id`, `direction`, `status`, `deadline_at`, `recipient_ref` (the `canon.account` id of the correspondent), questions answered over questions, `overdue` (a word, not a colour), and the attached pack ids and kinds. Not shown: case id, family, priority, tier, party id (phase 19 desk rule, section 3.14 of phase 19). An RFI of a suspicion case is absent from the list for a caller who cannot see the case (phase 19 masking; the list is not a 404). Direct access to such an RFI answers 404 `E_NOT_FOUND` with the same body as a missing id.

**Attach** (`POST /rfis/{id}/packs`; Cedar `rfi.attach_pack`; scope `rfi.desk`; `If-Match` = the RFI version; `Idempotency-Key`):
1. Flag `pack.correspondent`. Load the RFI with the phase 19 masking (404 when hidden).
2. Status must be `draft` (409 `E_NOT_ALLOWED_IN_STATE`).
3. `kind = 'correspondent'` (422 sub-code `kind_not_correspondent`) and `direction = 'out'` (422 sub-code `direction_not_out`). An inbound request keeps its template; it attaches no pack.
4. The pack is a row of `admin.packs` (a row exists only once its build is published: phase 02 `PackService::publish`, step 3; the table has no status column), of kind `correspondent` or `validator`, whose `scope.period` is a bank month that has ended. Anything else, a pack of another kind and an unknown id included, is refused with 422 sub-code `pack_not_ready`.
5. Insert `casework.rfi_packs`. A repeat of the same pair is a no-op answered `200` (the primary key is the idempotent form). Bump the RFI version; audit `rfi.pack_attached {rfi_id, pack_id}`.
6. Release is unchanged: a correspondent request always takes the phase 19 external release (`rfi_release_external`). The approver's view lists the attached pack ids and hashes.

**Tipping-off.** A correspondent request names the correspondent's account id and the bank's own question keys. Its text is the template of the bank's pack (phase 19 rule); the attached pack has no case data (section 3.7). A correspondent learns nothing about any case from the request.

### 3.10 SAML through Ory Polis (TS-25, G-12; DR-348)

- **Component.** Ory Polis (formerly BoxyHQ Jackson; Apache-2.0; version v26.2.0 as research 32 section 14.3 records; the digest is pinned in `deploy/catalogue.yaml`, phase 30 section 3.1). One catalogue entry, profile `polis`, rendered only while `pack.polis` is on. The admin interface and the management API of Polis are disabled in its configuration.
- **SAML path.** Bank IdP (SAML) -> Polis assertion consumer service (through Caddy, the pinned path) -> Polis OpenID Connect issuer -> Kratos generic OIDC upstream of phase 25 (`provider: generic`, issuer = Polis). AKR-TMS never parses a SAML message. The upstream adds no authority: no role, scope or group from the assertion (phase 25 DR-239); the link to a local account is the explicit phase 25 command; the local second factor and step-up still apply to every person (phase 25 section 3.4).
- **Mode.** `auth.upstream_mode = oidc` (phase 25, governed: change request). Switching it needs no redeploy; adding the Polis issuer does.
- **Connection.** `akr-ctl polis connection add --metadata FILE --name NAME` takes the bank IdP metadata file, mounts it read-only into the Polis container at the next `deploy render`, and audits `polis.connection_added {connection_id}` (SIEM true). The metadata is not stored in the AKR database.
- **Secrets** (OpenBao): `ory/polis` (database password, provisioned like `ory/kratos` in phase 01), `kv/polis/keys` (Polis encryption key, generated and rotated by `akr-ctl`, never printed), `kv/polis/directory` (the directory webhook secret of section 3.11; read by the akr-api AppRole, which gains this one path), and the Kratos client credentials `kv/kratos/oidc/generic` of phase 25.
- **Edge.** Caddy routes to Polis exactly the paths the browser and the bank IdP must reach in the pinned Polis version: the OAuth authorize endpoint (Kratos redirects the browser there), the SAML assertion consumer service and the SCIM 2.0 base path. The list is taken from the pinned version (Open doubts 13) and written in the catalogue entry; every other Polis path answers 404 at the edge. Caddy refuses `/api/v1/directory/events` (404): Polis reaches `akr-api` directly on `z3_edge` and is the route's only caller. Kratos reaches the Polis token and discovery endpoints on `z3_ory`, never through the edge.
- **Refused.** SAML group or role mapping; a second upstream; Polis as an authority for any role, scope or grant; LDAP (research 32 section 14.3: a broker would be needed).

### 3.11 SCIM directory sync (DR-349, DR-350, DR-351, DR-352)

**Flow.** The bank IdP calls Polis's SCIM 2.0 endpoint (create, update, deactivate). Polis's directory-sync webhook calls `POST /directory/events` with one event per SCIM operation. The route is an HMAC-verified webhook; there is no bearer token.

**Event steps** (`DirectoryService::event`):
1. `DirectoryWebhookVerifier::verify`: read the timestamp and the signature headers of the pinned Polis version (the header format is Open doubts 13) and verify the body with the key of `kv/polis/directory`. A signature that does not verify, or a timestamp more than 60 seconds in the future, is refused: 401 `E_UNAUTHENTICATED`, no write, no audit row. A signed timestamp older than `directory.event_max_age_s` is not refused: the event is recorded and answered `ignored` with reason `stale_event` (recorded in step 6; step 5 applies nothing), so that a late retry by Polis gets an answer and changes nothing. The verified principal is the service principal `polis-directory` (kind `service`, the one scope `directory.sync`, `Ctx.via = "webhook_hmac"`). Flag `pack.polis` off: 404.
2. `Idempotency-Key` required (400 `E_IDEMPOTENCY_REQUIRED`); the replay is answered by phase 00 `admin.idempotency_keys`.
3. The payload hash is the SHA-256 of the body. `groups` is accepted and never read, stored or mapped.
4. Lock `admin.directory_users` by `external_id` (`FOR UPDATE`).
5. Apply by `op`:
   - `create`: no row: insert `pending` with `directory_active` = `active`, the e-mail and the display name sealed (personal, AAD = id). A row already exists: treated as `update`.
   - `update`: the display name and the directory e-mail (`user_name_enc`) of the directory row are replaced. The e-mail is never applied to an account, linked or not: `admin.users.email` changes only through phase 03, and the reconciliation counts the difference (item 3). The outcome is `applied`.
   - `deactivate`: set `directory_active = FALSE`. If linked and the account is active: `UserAdminService::disable_from_directory` (section 3.12), which disables the account reversibly; it never offboards. Outcome `applied` (also when the account is already disabled or offboarded: nothing more is done); the outcome stays `applied` when the Kratos step after the commit fails (phase 03 enqueues `offboard_finalize`; the account is already disabled and its tokens revoked); `refused` with reason `last_holder` when the account is the last ADM or SEC holder (then `directory_active` stays `FALSE`, the link stays, and audit `directory.deactivate_refused` is written; the reconciliation lists it for SEC). A pending user is only marked inactive.
   - `update`, `deactivate` or `activate` with no row for the `external_id`: `ignored`, reason `unknown_user`; nothing is created.
   - `activate`: set `directory_active = TRUE`. A linked and offboarded account: `ignored` with reason `reactivation_ignored` (offboarding is final, phase 03 section 3.8 step 4). Otherwise `applied`. Activation never enables an account, including one the directory disabled; enabling is the phase 03 act of an administrator with step-up.
6. Insert `admin.directory_events` in the same transaction (`outcome` and `reason_code`). Audit `directory.event_applied {event_ref, op, outcome, reason_code, directory_user_id}` (SIEM true); for a refusal `directory.deactivate_refused {directory_user_id, reason_code}` (SIEM true).
7. Response 200 `{outcome, reason_code}`. The response never repeats a name or an e-mail. Every verified event answers 200, including `ignored` and `refused`, so the webhook does not retry a decision; only an unverified event answers 401.

**Link** (`POST /directory/users/{id}/link`; Cedar `directory.link`; scope `admin.users`; step-up class `identity_admin`; `If-Match` = the directory row version; `Idempotency-Key`):
1. Flag `pack.polis`. The step-up assertion is consumed once (401 `E_STEPUP_REQUIRED` without it).
2. The row must be `pending` (409 `E_NOT_ALLOWED_IN_STATE` sub-code `not_pending` or `already_linked`) and `directory_active = TRUE` (409 sub-code `directory_inactive`).
3. The target account: kind `human`, status `active` (422 `E_VALIDATION` sub-code `user_not_active`), not already linked (the unique index; 409 `already_linked`), and not the caller (403 `E_SELF_APPROVAL`).
4. The decrypted directory e-mail equals `admin.users.email` case-insensitively (422 `E_VALIDATION` sub-code `email_mismatch`).
5. Update `state = 'linked'`, `linked_user_id`, `linked_by`, `linked_at`, `version + 1`. Audit `directory.linked {directory_user_id, user_id}` (SIEM true). Linking grants nothing.

**The state of a directory-created person with no role.** A SCIM create creates no account (DR-349). Until an administrator creates the account (phase 03 `POST /admin/users`, the temporary password shown once) and links it, the person cannot sign in through SSO, holds no grant and appears as `pending` in `GET /directory/users`. After the link, sign-in works and the person has only the `self` scope (phase 01) until an administrator proposes a role grant (phase 04 `POST /admin/users/{id}/grants`, change request `cr_access`, approved by SEC). A directory group never proposes or approves anything.

**Reads.** `GET /directory/users` (`admin.users`; cursor; filter `state`, `directory_active`, `older_than`) returns the names to ADM only, and each read of a name writes a phase 02 access event `view`.

**Reconciliation** (`DirectoryService::reconcile`, job `directory_reconcile`, daily at 03:15, idle while `pack.polis` is off):
1. Linked, `directory_active = FALSE`, account still active: run `disable_from_directory` again; audit `directory.reconciled {count}`. Linked accounts disabled by the directory (audit `user.disabled` with `source: directory`) for longer than `directory.pending_alert_days` and not yet offboarded: audit `directory.disabled_stale {count}`; the list is `GET /directory/users?state=linked&directory_active=false` for ADM, who decides the final offboarding (phase 03 `POST /admin/users/{id}/offboard`, step-up `identity_admin`). The job never offboards.
2. Pending for longer than `directory.pending_alert_days`: audit `directory.pending_stale {count}`; the list is `GET /directory/users?state=pending&older_than=...`.
3. Mismatches: audit `directory.mismatch {count, email_count}` (SIEM true), written on a run where either count is above zero. `count` is the linked rows whose account is offboarded while `directory_active = TRUE`. Local wins; nothing is changed. `email_count` is the second mismatch of DR-351, counted over every linked row whatever the state of its account: the linked rows (`DirectoryRepository::list` with state `linked`) whose decrypted `user_name_enc` differs from `admin.users.email` of the linked account, compared case-insensitively as link step 4 does. The job decrypts in memory and writes the two counts only; no e-mail is written or logged.
4. The job writes counts only. Deactivations refused for last holder (section 3.11 step 5) are in the audit log for SEC.

**Conflicts (the rules that decide them).**
- Directory deactivation against a local enable: deactivation wins. Phase 03 `POST /admin/users/{id}/enable` on a linked account whose `directory_active` is `FALSE` is refused 409 `E_NOT_ALLOWED_IN_STATE` sub-code `directory_inactive` (phase 03 gains the check, section 4.5).
- Directory activation against a local offboarding: offboarding wins; activation is `ignored`.
- Local disable (suspension) against directory activation: independent; the local disable stays until an administrator enables. The directory never enables.
- Directory groups against grants: no effect. A grant is created only by phase 03 with change request `cr_access`.
- Directory names against local profile: names are display text; locale and branch scope are local (phase 03 `PATCH /admin/users/{id}`).
- Directory e-mail against a linked account: recorded as `directory.mismatch` and not applied.

### 3.12 Disable from a directory (extends phase 03 `UserAdminService::disable`; DR-350)

`UserAdminService::disable_from_directory(ctx, user_id, reason_code = "directory_deactivated")`:
1. Target: a human account in status `active` (already `disabled` or offboarded: outcome `applied`, nothing done), not the last ADM or SEC holder (outcome `refused`, reason `last_holder`). The caller check of phase 03 (not the caller) does not apply: the caller is the directory, not a person.
2. One transaction, the phase 03 `disable` body: status `disabled`, reason `admin`; the user's tokens revoked (`admin`); grants, delegations and liaison assignments stay (a suspension, not an offboarding). No step-up is consumed: phase 03 `disable` consumes none, and the act is reversible. Audit `user.disabled {reason: admin, reason_code: "directory_deactivated", source: "directory"}` (phase 03 payload gains `source`).
3. After commit: `invalidate_user`, Kratos `revoke_all_sessions` and `set_state(inactive)`; a Kratos failure enqueues `offboard_finalize` (phase 03 section 4) and the outcome stays `applied`: the event row is written in the transaction of step 2 and never updated, the account is already disabled, and the phase 03 mirror refuses its sessions at their next request. No outcome `deferred` exists (DR-413).
4. `ctx` is the caller context that every service method takes; no new type is added. On the event path it holds the verified service principal `polis-directory`, and Cedar checks `user.disable` with `context.via == "webhook_hmac"` (section 3.17, E0). On the reconciliation path (section 3.11, item 1) it is the job's system context: no Cedar check runs, as for every job, and the audit actor is `System` (phase 02).

Why a disable and not an offboarding: phase 03 offboarding is final (a returning leaver is a new account), and a directory event arrives with no person behind it. One forged or mistaken event, a leaked webhook secret or an IdP administrator's error would otherwise end every linked account for good. A disable removes access at once and can be undone by an administrator with step-up (phase 03 `enable`, refused while `directory_active` is `FALSE`). The final offboarding stays the phase 03 act of an administrator with step-up `identity_admin`; the reconciliation (section 3.11) lists the accounts waiting for it. Phase 03's last-holder rule still refuses the last ADM or SEC.

### 3.13 Errors (`TrainingError`, `CapacityError`, `ExposureError`, `DirectoryError`, `thiserror`; API section 6 codes only)

| Error | Code | HTTP | Sub-codes (in `errors[].code`) |
|-|-|-|-|
| `TrainingError::Validation` | `E_VALIDATION` | 422 | `decision_or_choice`, `answer_kind`, `reason_unknown`, `reason_limit` |
| `TrainingError::CaseRetired` | `E_NOT_ALLOWED_IN_STATE` | 409 | `case_retired` |
| `CapacityError::Validation` | `E_VALIDATION` | 422 | `occupancy_above_max`, `period_too_long` |
| `CapacityError::ReplayNotCompleted` | `E_NOT_ALLOWED_IN_STATE` | 409 | `replay_not_completed` |
| `CapacityError::Precondition` / `Version` | `E_PRECONDITION_REQUIRED` / `E_VERSION_CONFLICT` | 428 / 409 | none |
| `ExposureError::Validation` | `E_VALIDATION` | 422 | `scope_required`, `period_range` |
| `DirectoryError::Unauthenticated` | `E_UNAUTHENTICATED` | 401 | none |
| `DirectoryError::State` | `E_NOT_ALLOWED_IN_STATE` | 409 | `not_pending`, `already_linked`, `directory_inactive` |
| `DirectoryError::Validation` | `E_VALIDATION` | 422 | `user_not_active`, `email_mismatch` |
| `DirectoryError::SelfLink` | `E_SELF_APPROVAL` | 403 | none |
| `PackError` (phase 19 RFI attach) | `E_VALIDATION` | 422 | `kind_not_correspondent`, `direction_not_out`, `pack_not_ready` |

Every flag-off route answers `E_NOT_FOUND` (404). Scope refusals are `E_SCOPE` (403), Cedar refusals `E_FORBIDDEN` (403). Every code used is in API.md section 6 (grep verified).

### 3.14 Audit events (one per mutation; payloads hold ids, codes and counts only)

| Kind | Resource type | SIEM | Detail keys |
|-|-|-|-|
| `train.practice_attempted` | `practice_attempt` | false | `case_code`, `attempt_id`, `passed` |
| `train.cases_seeded` | `practice_pack` | false | `pack_version`, `cases` |
| `capacity.input_recorded` | `capacity_input` | false | `pool_id`, `week_start`, `input_id`, `superseded_id` |
| `capacity.simulation_requested` | `capacity_run` | false | `run_id`, `replay_id` |
| `capacity.simulation_completed` | `capacity_run` | false | `run_id`, `status` |
| `exposure.computed` | `typology_exposure` | false | `period_month`, `rows`, `suppressed_rows` |
| `rfi.pack_attached` | `rfi` | false | `rfi_id`, `pack_id` |
| `polis.connection_added` | `polis_connection` | true | `connection_id` |
| `directory.event_applied` | `directory_user` | true | `event_ref`, `op`, `outcome`, `reason_code`, `directory_user_id` |
| `directory.deactivate_refused` | `directory_user` | true | `directory_user_id`, `reason_code` |
| `directory.linked` | `directory_user` | true | `directory_user_id`, `user_id` |
| `directory.pending_stale` | `directory` | false | `count` |
| `directory.mismatch` | `directory` | true | `count`, `email_count` |
| `directory.reconciled` | `directory` | false | `count` |
| `directory.disabled_stale` | `directory` | true | `count` |

Existing kinds that this phase emits: `pack.created` and `pack.build_failed` (phase 02 and 06), `alarm.raised` (phase 19; `capacity_forecast`), `user.disabled` (phase 03; payload gains `source`), `user.updated` (phase 03). A read of a directory name writes a phase 02 access event (`view`), not an audit row.

### 3.15 Tipping-off review (what a person outside the access list can learn; the answer must be "nothing")

- **W12 practice** (`/training/practice`, section 5.1): the cases are synthetic and marked. A trainee who is not on a live case learns nothing from them. The known answer and the expert path reach a trainee only after that trainee's own attempt. Nothing in `train.*` can be joined to a live party.
- **G2 capacity and surge progress** (`/capacity`): team and class figures; no person, no case id, no alert id; small teams and small classes suppressed. An M, CH or IA reader learns the workload, not who did it.
- **Simulation result**: counts by class, week and pool; no entity id and no detection key (the columns are not read). A reader of a result learns nothing about a customer.
- **Exposure index and its route** (`/typology-exposure`, panel on G1, section 5.6): bank, branch and correspondent aggregates; restricted scenarios excluded, so a suspicion-tier match does not reveal its typology or branch; counts below the threshold null.
- **Correspondent and validator packs**: no case, party, account, narrative or customer (section 3.7); the download is step-up `export`.
- **RFI correspondent desk** (`/rfi`, section 5.3): the list has no case id, family, priority or tier (phase 19 rule); a suspicion case's correspondent RFI is absent from the list of a caller who cannot see the case. The correspondent learns nothing about any case from the request.
- **Directory and link** (`/admin/users`, section 5.7): the directory names are personal and sealed; ADM sees them with an access event; the webhook answers only `outcome` and `reason_code`. A directory user's record shows no case, no alert and no decision.
- **SAML**: the upstream carries no case or customer data; the SSO sign-in reveals nothing that the local login does not.

### 3.16 Settings used (phase 31 keys in `0100`; reused keys from earlier phases)

New: `train.pass_pct`, `capacity.horizon_weeks`, `capacity.occupancy_max`, `capacity.min_working_days`, `capacity.surge_active`, `capacity.surge_qc_percent`, `exposure.trend_months`, `exposure.trend_band_pp`, `directory.event_max_age_s`, `directory.pending_alert_days`, and the four schedules `schedule.capacity_snapshot`, `schedule.capacity_watch`, `schedule.exposure_compute`, `schedule.directory_reconcile`. Governed (change request): all except the schedules and `exposure.trend_months`.

Reused: `report.small_number_threshold` (phase 21; never below 3), `casework.touch_by_hours` (phase 17), `auth.upstream_mode` (phase 25), `bank.timezone` (phase 00), the phase 19 QA rate settings of section 4.5 item 9, and the flags of `admin.feature_flags` (phase 00).

### 3.17 Cedar: the three files `E0_directory.cedar`, `E1_training_capacity.cedar`, `E2_exposure_packs.cedar`

Phase 03 ranges: `E` is free (grep of docs/plan confirms). Roles are read as `principal.roles` (phase 03 form); one role per policy, annotated `@role`; step-up is not a Cedar condition (the handler calls `StepUpService::consume`). New scopes are route gates and are not Cedar attributes. The route scopes of section 2 admit every role below (section 6 item 13).

```cedar
// E0_directory.cedar (phase 31). The directory webhook is a service principal with the one scope directory.sync and
// Ctx.via = "webhook_hmac" (the value added to the phase 03 list by section 4.5); the route scope admits only
// polis-directory. No @role: a service principal holds no role (the phase 29 gateway form). It may disable
// (reversible), never offboard. Account links are ADM acts.
@id("directory.event.webhook")
permit (principal, action in [Akr::Action::"directory.event", Akr::Action::"user.disable"], resource)
when { principal.kind == "service" && context.via == "webhook_hmac" };

@id("directory.read.adm") @role("ADM")
permit (principal, action in [Akr::Action::"directory.read", Akr::Action::"directory.link"], resource)
when { principal.roles.contains("ADM") };
```

```cedar
// E1_training_capacity.cedar (phase 31). Analysts practise; only M plans capacity; M, CH and IA read team figures.
// A1, A2 and nobody else has any capacity action: the default is deny.
@id("train.practice.a1") @role("A1")
permit (principal, action in [Akr::Action::"train.practice.read", Akr::Action::"train.practice.attempt",
        Akr::Action::"train.practice.own"], resource)
when { principal.roles.contains("A1") };

@id("train.practice.a2") @role("A2")
permit (principal, action in [Akr::Action::"train.practice.read", Akr::Action::"train.practice.attempt",
        Akr::Action::"train.practice.own"], resource)
when { principal.roles.contains("A2") };

@id("train.practice.m") @role("M")
permit (principal, action in [Akr::Action::"train.practice.read", Akr::Action::"train.practice.attempt",
        Akr::Action::"train.practice.own"], resource)
when { principal.roles.contains("M") };

@id("capacity.plan.m") @role("M")
permit (principal, action in [Akr::Action::"capacity.inputs.write", Akr::Action::"capacity.simulation.run",
        Akr::Action::"capacity.inputs.read", Akr::Action::"capacity.forecast.read", Akr::Action::"capacity.simulation.read"], resource)
when { principal.roles.contains("M") };

@id("capacity.read.oversight") @role("CH")
permit (principal, action in [Akr::Action::"capacity.inputs.read", Akr::Action::"capacity.forecast.read",
        Akr::Action::"capacity.simulation.read"], resource)
when { principal.roles.contains("CH") };

@id("capacity.read.ia") @role("IA")
permit (principal, action in [Akr::Action::"capacity.inputs.read", Akr::Action::"capacity.forecast.read",
        Akr::Action::"capacity.simulation.read"], resource)
when { principal.roles.contains("IA") };
```

```cedar
// E2_exposure_packs.cedar (phase 31). Exposure is an aggregate view for M, CH and IA. Correspondent and validator
// packs are created by M only; read and download follow the board pack rule (M, CH, IA). RFI correspondent actions
// follow the RFI desk (RFI and M).
@id("exposure.read.oversight") @role("M")
permit (principal, action == Akr::Action::"exposure.read", resource)
when { principal.roles.contains("M") };

@id("exposure.read.ch") @role("CH")
permit (principal, action == Akr::Action::"exposure.read", resource)
when { principal.roles.contains("CH") };

@id("exposure.read.ia") @role("IA")
permit (principal, action == Akr::Action::"exposure.read", resource)
when { principal.roles.contains("IA") };

@id("pack.correspondent.create") @role("M")
permit (principal, action == Akr::Action::"pack.create", resource)
when { principal.roles.contains("M") && ["correspondent", "validator"].contains(context.pack_kind) };

@id("pack.correspondent.read.m") @role("M")
permit (principal, action in [Akr::Action::"pack.read", Akr::Action::"pack.download"], resource)
when { principal.roles.contains("M") && ["correspondent", "validator"].contains(context.pack_kind) };

@id("pack.correspondent.read.ch") @role("CH")
permit (principal, action in [Akr::Action::"pack.read", Akr::Action::"pack.download"], resource)
when { principal.roles.contains("CH") && ["correspondent", "validator"].contains(context.pack_kind) };

@id("pack.correspondent.read.ia") @role("IA")
permit (principal, action in [Akr::Action::"pack.read", Akr::Action::"pack.download"], resource)
when { principal.roles.contains("IA") && ["correspondent", "validator"].contains(context.pack_kind) };

@id("rfi.correspondent.desk") @role("RFI")
permit (principal, action in [Akr::Action::"rfi.correspondent.list", Akr::Action::"rfi.attach_pack"], resource)
when { principal.roles.contains("RFI") };

@id("rfi.correspondent.m") @role("M")
permit (principal, action in [Akr::Action::"rfi.correspondent.list", Akr::Action::"rfi.attach_pack"], resource)
when { principal.roles.contains("M") };
```

The Cedar entity attributes admit exactly what the SQL predicates admit: `context.pack_kind` is the phase 06 attribute, `context.via` the phase 03 attribute with the one value added here, `principal.kind` the phase 01 principal kind. Route scope gates (section 2) are checked before Cedar, so a role without the scope gets `E_SCOPE` and never reaches these policies.

### 3.18 Lock order

One order on every path, the phase 17 and 27 rule: (1) the pool row (`admin.pools`, read); (2) the live capacity input row (`FOR UPDATE`); (3) the directory row (`FOR UPDATE`); (4) the `admin.users` row (the phase 03 disable); (5) the RFI row; (6) the `casework.rfi_packs` insert. Snapshot upserts take their own bucket rows and no other lock. The disable keeps phase 03 order internally.

---

## 4. Infrastructure Updates

### 4.1 Job kinds (Postgres `jobs`; coalesce key in brackets)

| Kind | Binary | Payload | Concurrency, attempts | Schedule or trigger |
|-|-|-|-|-|
| `capacity_snapshot` | worker | `{}` [`capacity_snapshot`] | 1, 3 | `schedule.capacity_snapshot` (hourly, minute 5); section 3.3; returns at once while `pack.capacity` is off |
| `capacity_watch` | worker | `{}` [`capacity_watch`] | 1, 3 | `schedule.capacity_watch` (06:20 bank time); section 3.4 alarm; idle while `pack.capacity` is off |
| `capacity_simulation` | worker | `{ run_id }` [`capacity_simulation:<run_id>`] | 2, 3 | enqueued by `POST /capacity/simulations`; section 3.5 |
| `exposure_compute` | worker | `{ period_month }` [`exposure_compute:<month>`] | 1, 3 | `schedule.exposure_compute` (2nd of the month, 01:30) for the previous month; section 3.8; idle while `pack.exposure` is off |
| `directory_reconcile` | worker | `{}` [`directory_reconcile`] | 1, 3 | `schedule.directory_reconcile` (03:15); section 3.11; idle while `pack.polis` is off |
| `pack_build` (phase 06, extended) | worker | `{ kind, scope, lang?, requested_by }` [kind + canonical scope + lang] | 2, 3 (unchanged) | `POST /packs` with kind `correspondent` or `validator`; section 3.7 |

No job reads the core banking system; the snapshotter and the exposure job read the canonical copy (`canon.*`, phase 10) and the alert store. No job name uses the `ingest_` prefix. Every job reads its flag first and returns without work while the flag is off. Every handler re-reads its owner before it writes.

### 4.2 Timers and outbox

No timer kind and no outbox topic. The alarm of section 3.4 uses the phase 19 `AlarmService` and its existing outbox event `alarm.raised`; the supervisor notice uses the phase 22 code `supervisor.alarm`. No PostgreSQL notification channel is added.

### 4.3 AppState additions

- `akr-api`: `training: Arc<TrainingService>`, `capacity: Arc<CapacityService>`, `exposure: Arc<ExposureService>` (read), `directory: Arc<DirectoryService>`, `directory_verifier: Arc<DirectoryWebhookVerifier>`, `directory_link: Arc<dyn DirectoryLinkQuery>` (`PgDirectoryLinkQuery` while `pack.polis` is on, phase 03 `NoDirectory` while it is off) (the webhook key is read from OpenBao at start; a rotation is a restart, and the old key is refused at once because the verifier holds one key).
- `akr-worker`: `capacity_snapshotter: Arc<CapacitySnapshotter>`, `capacity_watcher: Arc<CapacityWatcher>`, `simulation: Arc<SimulationHarness>`, `exposure_compute: Arc<ExposureService>`, `directory_reconcile: Arc<DirectoryService>`, and `correspondent_packs: Arc<CorrespondentPackService>`, registered with the phase 02 `PackService` through the phase 06 `pack_build` handler.
- `akr-ctl`: `training seed`, `polis connection add`, and the flags of `deploy render` and `db provision` of section 4.5.

### 4.4 Network zones, compose, edge, secrets, flags, OpenAPI

- **Zones** (`deploy/zones.yaml`; no new zone name, no second matrix). Polis is attached to `z3_ory` (`kratos` reaches it for OpenID Connect discovery and token calls; it reaches `postgres:5432` for its own database, and nothing else on that network) and to `z3_edge` (Caddy reaches it on the pinned paths of section 3.10; it reaches `akr-api:8080` for `POST /directory/events` only). Network `z3_edge` is not `internal`, so `akr-ctl net render` adds a host rule that drops every outbound packet of the Polis container except to the Postgres and Kratos addresses. Polis has no egress need: the IdP metadata is a file (section 3.10). `akr-api` keeps its networks.
- **Compose and catalogue** (phase 30 section 3.1): one entry `polis`: image Ory Polis v26.2.0 pinned by digest; command and ports as the pinned version documents; environment literals that disable the admin interface and the management API; `secret_paths` `ory/polis` and `kv/polis/keys`; networks `z3_ory`, `z3_edge`; profile `polis` (rendered only while `pack.polis` is on at render time); replicas 1 in every profile. Non-root, read-only root file system, dropped capabilities (phase 00 rule for every container). A separate AppRole `akr-polis` reads only `ory/polis` and `kv/polis/keys`.
- **Edge** (Caddy, phase 00): the pinned authorize, assertion consumer and SCIM paths of section 3.10 to `polis` while the profile is rendered; every other path of the Polis host answers 404 at the edge; `/api/v1/directory/events` answers 404 at the edge in every profile (the phase 00 rule `/api/*` -> akr-api gains this one exclusion).
- **OpenBao**: `kv/polis/directory` (the webhook secret, written by `akr-ctl`, read by the `akr-api` AppRole, which gains this one path); `kv/polis/keys` and `ory/polis` (section 3.10). The database role `polis` (phase 01 provisioning) has no privilege in any other database.
- **Feature flags** (`admin.feature_flags`, section 1.5): `pack.training`, `pack.capacity`, `pack.exposure`, `pack.correspondent`, `pack.polis`, all `FALSE`. A route whose flag is off answers `404 E_NOT_FOUND` with the unknown-id body (phase 26 and 29 pattern); a job returns without work.
- **OpenAPI**: new tags `training`, `capacity`, `exposure`, `directory`; the `rfis` tag gains two rows; `POST /packs` keeps its tag. Every new route carries `x-akr-action` (the Cedar action of section 2), `x-akr-stepup` where it applies (`POST /directory/users/{id}/link`: `identity_admin`), and `x-akr-idempotency` on every POST. `make generate-api` regenerates the client (section 5.8).

### 4.5 Existing entity, schema, route and code updates (extensions of earlier phases; each is additive)

1. **Phase 00.** `deploy/zones.yaml` gains the Polis rows of section 4.4 (no zone is added). `admin.feature_flags` gains five keys (`0100`). The `akr-api` AppRole gains KV read of `kv/polis/directory`; the `akr-polis` AppRole is new. `admin.settings` gains fourteen rows (`0100`).
2. **Phase 01.** `akr-ctl db provision` creates the database `polis` and the LOGIN role `polis` (password in OpenBao `ory/polis`), the same way it creates `kratos` and `hydra` (phase 01 provisioning). It runs when `pack.polis` is switched on, not before.
3. **Phase 02.** The audit catalogue gains the kinds and SIEM flags of section 3.14. `admin.packs` keeps its CHECK (already lists `correspondent` and `validator`). The `PackService` kind registry gains the two builders of section 3.7.
4. **Phase 03.** `UserAdminService::disable_from_directory` (section 3.12) is added beside `disable`; `user.disabled` gains the payload key `source`. `OffboardingService` is unchanged: the directory never offboards. `POST /admin/users/{id}/enable` refuses a linked account whose directory row is inactive (409 `E_NOT_ALLOWED_IN_STATE`, sub-code `directory_inactive`; section 3.11 conflicts). The phase 03 `Ctx.via` value list gains `webhook_hmac` (documentation of the String values, as phase 29 did for `gateway_cert`).
5. **Phase 06.** The `pack_build` handler builds the kinds `correspondent` and `validator` (section 3.7). `POST /packs` accepts them under the Cedar of section 3.17 and the flag `pack.correspondent`.
6. **Phase 16.** None. The harness reads `scn.replays` and `scn.replay_results`; the replay route and the cost guard are unchanged.
7. **Phase 17.** None in code. The snapshotter reads the queue `pool_id` and `filter` (the attribution rule of section 3.3 is this phase's, not phase 17's) and `touch_due_at`; `casework.wip_limit` and the touch windows are read by nothing in this phase except the simulation's default touch hours.
8. **Phase 19.** `RfiService::attach_pack` and `RfiService::correspondent_list` are added (section 3.9); `casework.alarms` gains the kind `capacity_forecast` and the subject `pool` (`0097`); the `AlarmService` kind table gains the row of section 3.4.
9. **Phase 19, QA sampler.** While `capacity.surge_active` is `TRUE`, the sampler of phase 19 section 3.8 raises the size of a draw that takes its size from the settings to at least `ceil(N * capacity.surge_qc_percent / 100)` items (the formula is in phase 19 section 7.5); otherwise it is unchanged. No sample is removed by a surge.
10. **Phase 24.** `TrainingRegisterService::list` gains the display column `practice` (section 3.2 rule 4; the column is `TrainingService::practice_column`). `admin.training_records` is unchanged. G1 gains a read-only panel of section 5.6, not a tile (`GET /typology-exposure` for M, CH and IA). The G1 tile definitions and their suppression are unchanged.
11. **Phase 25.** No change to `OidcUpstream`. The upstream issuer may be Polis (configuration, DR-348). `auth.upstream_mode` is unchanged.
12. **Phase 30.** The catalogue has one new entry (`polis`), and the render takes the flag `pack.polis` as an input (section 4.4).
13. **Specifications.** API.md section 2 gains `capacity.plan` (M) and `directory.sync` (the service principal `polis-directory`, webhook authentication); API.md section 3.16 rows 726 and 727 are implemented here; the routes of section 2 of this phase are added to API.md (training, capacity and exposure rows in section 3.16, the two `rfis` rows in 3.12, the three `directory` rows in 3.2). DATA-MODEL gains the nine tables and the view of section 1. DESIGN-SYSTEM gains the screen rows of section 5 against W12, G2, W8, R4, G1, G9 and A1.

---

## 5. Frontend Impact

All screens are bilingual (Arabic first, right-to-left) and work at 320 CSS px with no horizontal page scroll. Catalogue IDs and specs are DESIGN-SYSTEM 8.1 and 8.3. Every screen of this phase is behind its flag: with the flag off the screen is not in the navigation and its route answers 404.

### 5.1 W12 Seeded cases and training workspace (`/training/practice`; WORK; P2; flag `pack.training`; A1, A2, M)

- A permanent band at the top of every screen of the workspace reads "TRAINING - synthetic data" (the training mark of DESIGN-SYSTEM, in both languages). The band is not dismissible and is in the page title and in the print view.
- Entry from the training menu only. The workspace has no link to a live case, alert, party or account, and no search that reaches live data.
- Case list: the path filter, the course code, the title and the typologies (as words). Cards carry the mark.
- Case screen: the facts (synthetic, in a table), the question, the decision choice (four outcomes, with their plain-language consequences as in the case proposal form of phase 18), the reason list (multi-select, with "required" hidden), and "Submit answer". The submit is one action with an idempotency key.
- Result screen (after submit only): passed or not passed in words, the decision and the reasons that matched, the known answer, the expert path as a numbered list, and the attempt's date. Nothing is shown before the submit. The result can be left and reopened from "My attempts".
- Errors and states: `E_NOT_ALLOWED_IN_STATE` (case retired) is a message on the list; a flag-off 404 never reaches the screen.
- Hooks: `useGetTrainingPracticeCases`, `useGetTrainingPracticeCasesId`, `usePostTrainingPracticeCasesIdAttempts`, `useGetTrainingPracticeAttempts`.

### 5.2 G2 Capacity workbench (`/capacity`; STUDIO; P2; flag `pack.capacity`; M edits, CH and IA read)

- Four panels in one page: **Inputs** (per pool, per bank week: staffed analysts, hours per day, shrinkage, occupancy target, note; the live row and its version; M only edits), **Forecast** (per pool: workload hours per working day, required analysts against staffed analysts, the backlog path by week as a table with a short chart described in words, the sensitivity grid as a table; a pool below the threshold shows the words "too few people to show"), **Simulation** (the form of section 3.5 with a replay picker, the class mix as four numbers that must sum to 100, the result as tables: queue peak, breaches as estimates labelled "estimate", analyst hours by week and class), and **Surge** (the state of `capacity.surge_active`, the change request link, the hourly progress table of the last 24 hours, the surge QC rate).
- Every figure has its definition on hover and in the help panel (the metric catalogue of research 28 section 11.5); the words "estimate" and "measured" are shown beside every number that is not a count.
- No screen lists an analyst or a person. A row is a pool and a class. Alerts that no queue admits are shown as one "unassigned" total under the table, never inside a pool.
- The insufficient-data state reads "not enough measured working days yet" with the count of measured days; no number is shown in its place.
- Hooks: `useGetCapacityInputs`, `usePostCapacityInputs`, `useGetCapacityForecast`, `usePostCapacitySimulations`, `useGetCapacitySimulationsId`.

### 5.3 W8 RFI desk, correspondent side (`/rfi`; LIST + FORM; flag `pack.correspondent`; RFI and M)

- A "Correspondents" filter on the existing desk list (phase 19 section 5.2): rows of kind correspondent, the direction (in or out), the deadline with the overdue word, the answer progress, and the attached pack names.
- Detail of an outbound request in draft: a "Attach pack" action that lists built correspondent and validator packs of ended periods (name, period, language, hash in short form). After attach, the pack is listed with its hash and the release panel of phase 19 shows the same list to the approver.
- The inbound request shows its questions and answers (claims) as in phase 19; no attach action.
- No case id, family, priority or tier is shown (phase 19 rule). A correspondent request of a suspicion case is not in the list.
- Hooks: `useGetRfisCorrespondents`, `usePostRfisIdPacks`; the existing RFI hooks of phase 19 are unchanged.

### 5.4 R4 Pack builder: correspondent and validator kinds (`/packs`; STEPPER; flag `pack.correspondent`; M creates)

- Two kinds added to the kind list, visible to M only: "Correspondent evidence pack" and "Validator evidence pack", each with the period (a month) and the language. The stepper is the phase 24 one: choose kind, choose period, review the facts list with its sources and `data_as_of` (the facts of section 3.7, each with its count or "suppressed"), build.
- A built pack shows its version, hash and download, which asks for step-up (`export`) before it starts.
- Hooks: the existing `usePostPacks`, `useGetPacksId`, `useGetPacksIdDownload` of phase 06 (no new hook).

### 5.5 G9 Training register: practice column (`/training`; REGISTER; P1 screen, column of this phase)

- One column "Practice" beside each course record: "passed on first attempt" (shown when `passed_first` is TRUE, that is, at least one case was passed first time) with the number of cases passed first time out of the cases tried, "not passed on first attempt" (cases tried, none passed first time), or "no attempt"; repeat attempts are shown as a count labelled "practice repeats, not scored". The column is labelled "practice, synthetic cases" and carries the TRAINING mark in its header. It never adds a row and never changes the register's counts.
- The record form is unchanged: an administrator records a completion for another person and may cite `practice:<attempt id>` in the evidence reference (section 3.2 rule 5).
- Hooks: the existing `useGetTrainingRegister` (the response gains the `practice` field) and `usePostTrainingRegister` (unchanged).

### 5.6 G1 Typology exposure panel (`/command`; DASH; flag `pack.exposure`; M, CH and IA)

- A read-only panel "Typology exposure" below the tiles (not a tile: the tile definitions of phase 24 are unchanged). Columns: typology (its name from the scenario document, bilingual), share of flows this month, trend word (up, down, flat, or "not enough months"), and the scope selector: bank, a branch, or a correspondent account by its id.
- Suppressed cells read "below the reporting threshold". A null never shows as zero.
- Hooks: `useGetTypologyExposure`.

### 5.7 A1 Users: directory panel (`/admin/users`; A1; flag `pack.polis`; ADM)

- On a user's page and in the list filter "Directory": the directory state (pending, linked, directory active or inactive), the e-mail of the directory row (shown to ADM only, with the access event of section 3.11), and the action "Link to this account" for a pending row, which asks for step-up (`identity_admin`) and shows the e-mail check result before it confirms.
- A linked account whose directory row is inactive shows the words "deactivated in the directory" and an explanation: the account is suspended, its grants are kept, and an administrator decides whether to offboard it (the phase 03 action, with step-up). Enabling it is disabled with that reason.
- The directory is never shown as a source of roles: the page lists the grants of phase 03 only.
- Hooks: `useGetDirectoryUsers`, `usePostDirectoryUsersIdLink`; the phase 03 user hooks are unchanged.

### 5.8 Generated hooks (Orval names from the paths)

`useGetTrainingPracticeCases`, `useGetTrainingPracticeCasesId`, `usePostTrainingPracticeCasesIdAttempts`, `useGetTrainingPracticeAttempts`, `useGetCapacityInputs`, `usePostCapacityInputs`, `useGetCapacityForecast`, `usePostCapacitySimulations`, `useGetCapacitySimulationsId`, `useGetTypologyExposure`, `useGetRfisCorrespondents`, `usePostRfisIdPacks`, `usePostDirectoryEvents` (not used by the browser; present because the route is in the OpenAPI document, and excluded from the client by its tag), `useGetDirectoryUsers`, `usePostDirectoryUsersIdLink`. `make generate-api` must regenerate the client with no drift.

### 5.9 Browser and caching rules

- Capacity forecast, simulation results, practice results and the exposure view are `Cache-Control: no-store`; the client does not keep them in its query cache beyond the page.
- The practice attempt form keeps no draft in browser storage (an unsent answer is lost on reload; the draft is not sensitive but is a per-viewer convenience only).
- No screen of this phase writes to `localStorage`, `sessionStorage` or IndexedDB.

---

## 6. Verification

Concrete checks, in order. Dev users are one per role as the conventions give (`<role>@bank.test`). The curl suite is `tests/curl/phase31_training_capacity.sh`; it logs in each role through the phase 01 dev flow and asserts the status and the error code of every case below.

1. **Migrations.** `make migrate` applies `0096` to `0100` as `akr_migrate`; a second run changes nothing. Throwaway test: `docker run --rm -d --name p31pg -e POSTGRES_PASSWORD=test postgres:17`, create the roles `akr_app`, `akr_retention`, `akr_readonly`, `akr_migrate` and stub tables for every object the migrations read (`admin.users`, `admin.pools`, `admin.packs`, `admin.feature_flags`, `admin.settings`, `admin.column_classification`, `admin.retention_assignment`, `admin.evidence_objects`, `casework.rfis`, `casework.alarms`, `casework.cases`, `scn.replays`, `scn.replay_results`, `scn.scenarios`, `alert.alerts`, `canon.txn`, `canon.account`, `canon.balance_snapshot`, `canon.branch`, `ref.reason_codes`) with the columns named in sections 1 and 3, then run the five files twice with `PGOPTIONS="-c role=akr_migrate"`. Second run: no error. `\d casework.alarms` shows `capacity_forecast` and `pool`. The `packs_kind_check` assertion of `0100` passes. The container is removed with `--rm` only; no `docker rm`, no prune. Draft check (2026-10-09, this document): `0096` to `0100` applied twice as `akr_migrate` on `postgres:17` with stub objects, with no error on either run; the 14 settings, the 5 flags, the 9 retention rows, the two encrypted columns and the widened alarm CHECK were read back; the append-only guard refused an `UPDATE` and a `DELETE` by `akr_migrate`; `train` referenced by no view outside it.
2. **Settings and flags.** `SELECT count(*) FROM admin.settings WHERE key ~ '^(train|capacity|exposure|directory)\.' OR key ~ '^schedule\.(capacity|exposure|directory)'` returns 14. `SELECT key FROM admin.settings WHERE floor IS NOT NULL AND NOT governed` returns no rows (phase 00 CHECK). `SELECT key, enabled FROM admin.feature_flags WHERE key LIKE 'pack.%'` returns the five new keys, all `FALSE`.
3. **Compile and lint.** `cargo check`, `cargo clippy -- -D warnings`, `cargo test`. The property tests of section 3.4 are part of the suite: `fluid_projection` never returns a negative backlog; a run with arrivals at or below capacity keeps the backlog at its start value or lower; `workload_hours` with the research 28 section 11.2 inputs (100 cases a day; weighted handling 36.85 minutes unrounded, shown as 36.9 in the research table) returns 61.4 hours and 12.9 analysts at 85 percent occupancy.
4. **Training isolation.** (a) `SELECT count(*) FROM pg_views WHERE schemaname <> 'train' AND definition ILIKE '%train.%'` returns 0; `SELECT count(*) FROM pg_proc p JOIN pg_namespace n ON n.oid = p.pronamespace WHERE n.nspname <> 'train' AND prosrc ILIKE '%train.%'` returns 0; the Rust test `no_train_reference_outside_module` greps the crates and allows only the `training` module and `TrainingService::practice_column`. (b) With three synthetic cases and five attempts inserted, every phase 24 tile (its section 3.7 definitions) and every count of the registers (sections 3.4 and 3.10) and every phase 21 count returns the same values as with `train.*` empty. The test is in the phase 24 suite and runs with training rows present. (c) `GET /training/register` shows the `practice` field and its `total` is unchanged.
5. **No per-person capacity.** `curl -s -H "$M" "$API/capacity/forecast" | jq -r 'paths | map(tostring) | join(" ")' | grep -Ei 'user|analyst_id|claimed|person|name'` returns nothing. `SELECT count(*) FROM information_schema.columns WHERE table_schema = 'casework' AND table_name LIKE 'capacity_%' AND column_name ~ '(user|claimed|assignee|analyst|person)'` returns 0. (`entered_by` and `requested_by` name the M who acted, never an analyst; they are not in the pattern.)
6. **Small-number suppression.** A fixture pool with two analysts staffed produces no pool row in `GET /capacity/forecast` and an entry in `suppressed_pools`; a class with two decided items shows `measure: null, suppressed: true`. A `rpt.typology_exposure` cell with three flows (threshold 5) returns `flows: null, suppressed: true` in `GET /typology-exposure`; the same cell with five flows is shown.
7. **Advice only (D21).** Record the hash of every row of `alert.*`, `scn.*` and `admin.settings`, and the count of `casework.alarms` and `casework.wip_limit`. Run `capacity_snapshot`, `capacity_watch` and one `capacity_simulation`. The hashes are unchanged; the alarm count rises by at most one (`capacity_forecast`, only if the fixture triggers it). The job SQL writes no column of `alert.*`, `scn.*` or `admin.settings`: a grep of the module for `UPDATE alert.`, `UPDATE scn.`, `INSERT INTO alert.`, `INSERT INTO scn.` and `admin.settings` returns nothing.
8. **Snapshot without persons.** The `capacity_snapshot` SQL selects and writes no `*user*`, `*claimed*`, `*assignee_id*`, or `*analyst*` column (grep of the module, in the test of step 5).
9. **Typology index, restricted scenarios.** A fixture with one restricted-scenario alert matching typology 2 gives the same `rpt.typology_exposure` rows with and without that alert (the restricted match is not counted). A correspondent scope returns only its own account id, no name key.
10. **Correspondent pack.** M builds `correspondent` for `2026-09` (`POST /packs`, 202); IA builds the same kind (403 `E_FORBIDDEN`, Cedar deny: IA holds `dashboard.read` and the scope passes, the Cedar action is M only). `GET /packs/{id}/download` without an assertion: 401 `E_STEPUP_REQUIRED`; with an `export` assertion: 200. A fixture that puts a `casework.cases.id` into a fact makes the build fail with `pack.build_failed {reason: "case_reference"}`. Every count below 5 is `null`. Audit `pack.created` holds the SHA-256.
11. **Simulation.** `POST /capacity/simulations` on a `running` replay: 409 `E_NOT_ALLOWED_IN_STATE` sub-code `replay_not_completed`; on a completed replay of eight weeks or less: 202 with `Location`. The result JSON contains no `entity_id` and no `detection_key` (grep of the stored result). Row counts of `alert.*`, `scn.*` and `casework.alarms` do not change during the run (step 7).
12. **Directory webhook.** (a) A body with a wrong signature: 401 `E_UNAUTHENTICATED`, no row in `admin.directory_users` or `admin.directory_events`. (b) A `create` event for the invented fixture user "Ahmed Kareem Hassan" (`external_id` `scim-0001`): 200 `outcome: applied`; `GET /directory/users` (ADM) lists it `pending`. (c) The same event repeated with the same `Idempotency-Key`: the same bytes and `x-idempotent-replay: true`; the same key with a different body: 422 `E_IDEMPOTENCY_REUSE`. (d) An event with a timestamp 15 minutes old: 200 `outcome: ignored, reason_code: stale_event`. (e) With `pack.polis` off: 404 `E_NOT_FOUND`.
13. **SCIM groups never grant.** A linked user's `update` event with `groups: ["ADM", "SEC"]` returns 200; `GET /admin/users/{id}` (the phase 03 detail, which lists the grants) before and after shows the same grants, and `admin.role_grants` has no new row.
14. **SCIM deactivate disables, never offboards.** A `deactivate` event for a linked user who is not the last holder: 200 `outcome: applied`; `GET /admin/users/{id}` shows `disabled` with `disabled_reason: admin`; the user's sessions and tokens are gone (phase 03 counts); the grants are unchanged; the audit row `user.disabled` carries `source: "directory"`; no `user.offboarded` row exists. With the directory row set active again and an ADM `POST /admin/users/{id}/enable` with step-up: 200 (the account is back with its grants). The deactivation of the only ADM: 200 `outcome: refused, reason_code: last_holder`; audit `directory.deactivate_refused`; the account is unchanged.
15. **SCIM activate after an administrator's offboarding.** ADM offboards a directory-disabled user (phase 03 route, step-up `identity_admin`). An `activate` event for that user: 200 `outcome: ignored, reason_code: reactivation_ignored`. `POST /admin/users/{id}/enable` on that user: 409 `E_NOT_ALLOWED_IN_STATE` (phase 03 rule `offboarded`). The reconciliation run against a user disabled by the directory more than `directory.pending_alert_days` ago writes `directory.disabled_stale {count: 1}` and changes no account.
16. **Link.** `POST /directory/users/{id}/link` without a step-up assertion: 401 `E_STEPUP_REQUIRED`. With a step-up and a user whose e-mail differs: 422 `E_VALIDATION` sub-code `email_mismatch`. Linking the caller's own account: 403 `E_SELF_APPROVAL`. Linking a pending row whose `directory_active` is `FALSE`: 409 `E_NOT_ALLOWED_IN_STATE` sub-code `directory_inactive`. A correct link: 200; `admin.directory_users.state = 'linked'`; audit `directory.linked`; grants unchanged.
17. **Local enable against directory deactivation.** After step 14, `POST /admin/users/{id}/enable` for a linked user whose directory row is inactive: 409 `directory_inactive` (phase 03 route with the phase 31 check).
18. **Cedar and scope matrix.** For each route of section 2 and each of the fourteen roles (ds, cfg, chk, m, ch, ia, ss, kyc, a1, a2, a2b, adm, sec, mro): the expected result is 200, 201, 202 or one of 403 `E_SCOPE`, 403 `E_FORBIDDEN`, 404 `E_NOT_FOUND`. The matrix is in the suite and generated from section 2 and sections 3.16 and 3.17. A role whose scope admits a route but whose Cedar policy does not gets `E_FORBIDDEN`, never 200 (the phase 24 rule: every route scope admits every role its Cedar policies permit; a mismatch fails the build).
19. **Masking.** An RFI of a suspicion-tier case: `POST /rfis/{id}/packs` by RFI answers 404 `E_NOT_FOUND` with the same body as an unknown id; `GET /rfis/correspondents` does not list it.
20. **Practice.** A1 submits a decision on a case: 201 with `expert_path` and `known_answer`; the same A1's `GET /training/practice/cases/{id}` before the attempt has no `known_answer` or `expert_path` key; a retired case: 409 `case_retired`; a body with both `decision` and `choice`: 422 `decision_or_choice`; a reason code not in `ref.reason_codes`: 422 `reason_unknown`; the same key replayed: same bytes. With `pack.training` off: 404.
21. **Capacity idempotency and concurrency.** `POST /capacity/inputs` superseding a live row without `If-Match`: 428 `E_PRECONDITION_REQUIRED`; with a stale version: 409 `E_VERSION_CONFLICT` and the current version in the body; two concurrent supersedes of one pool and week: one succeeds, the other 409 (the partial unique index). A `occupancy_target` of 0.95: 422 `occupancy_above_max`. A `week_start` that is not a Sunday: 422 (CHECK, mapped to `E_VALIDATION`).
22. **Audit.** Every mutation of section 3.14 writes exactly one audit row with the listed detail keys. `SELECT count(*) FROM audit.audit_log WHERE details::text ~ '@'` returns 0 for the phase 31 kinds (no e-mail in any payload). The SIEM-flagged kinds appear in the phase 25 SIEM cursor stream.
23. **Retention and classification.** `SELECT count(*) FROM admin.retention_assignment WHERE (schema_name, table_name) IN (('train','practice_cases'),('train','practice_attempts'),('casework','capacity_inputs'),('casework','capacity_snapshots'),('casework','capacity_runs'),('casework','rfi_packs'),('rpt','typology_exposure'),('admin','directory_users'),('admin','directory_events'))` returns 9. `SELECT column_name FROM admin.column_classification WHERE encrypted` returns exactly `user_name_enc` and `display_name_enc`. `akr_retention` holds SELECT only on `train.practice_attempts` in this release (section 7.3), so no deletion path exists yet. A direct `DELETE` by `akr_app` fails with `insufficient_privilege`.
24. **Zones and Polis.** With `pack.polis` off, `akr-ctl deploy render --profile s` is byte-identical to the phase 00 bundle (the phase 30 acceptance test). With it on, the render adds the `polis` service and the pinned Caddy paths and nothing else. `curl -s -o /dev/null -w '%{http_code}' -X POST https://$EDGE/api/v1/directory/events` answers 404 with the profile on and off. `make zone-check` passes with the Polis rows: from the Polis position `akr-api:8080` is open on `/api/v1/directory/events` only, `objectstore` and `valkey` are closed, and the host rule drops other outbound packets. `kratos` reaches the Polis discovery document; Caddy answers 404 for the Polis host's root path.
25. **Frontend.** `make generate-api` regenerates the client with no diff. Playwright at 320 CSS px in Arabic and English: W12 shows the TRAINING band on every view; G2 has no person-named row; `/rfi` correspondent filter shows no case id; the `/packs` stepper shows the facts with "suppressed" where counts are small. axe reports no serious violation on those four screens.
26. **Regressions.** Run the phase 19, 24 and 29 curl suites and the phase 03 disable, enable and offboarding suites: no regression. `make test` green.

---

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 1: corrections found while checking phases 00 to 02 (tools/agent/gap1-run1-verdict.md); lead-verified.

### 7.1 KV paths
KV paths are relative to the one mount `akr/` (phase 00 7.5, DR-356): `kv/polis/directory` is `polis/directory`,
`kv/polis/keys` is `polis/keys` and `kv/kratos/oidc/generic` is `kratos/oidc/generic`; `ory/polis` is unchanged.

---

Gap pass 1 (2026-10-10), run 4b, from tools/agent/gap1-run4b.md; lead-verified (tools/agent/gap1-run4b-verdict.md).

### 7.2 Directory link port (phase 03 section 7+)
The `directory` module of this phase (not section 3.12, which holds the disable only) implements phase 03's `DirectoryLinkQuery::is_inactive(user_id) -> bool` as `PgDirectoryLinkQuery`: TRUE
when a row of `admin.directory_users` has `linked_user_id = user_id` and `directory_active = FALSE`, else FALSE. It is
wired into `AppState` when `pack.polis` is on; with the flag off, phase 03's `NoDirectory` stays. Test: enabling a
linked user whose directory row is inactive is refused as phase 03 states.

Gap pass 1 (2026-10-10), run 4b: items carried from run 4a and corrections found while checking phases 25 to 31 (tools/agent/gap1-run4b-verdict.md); lead-verified.

### 7.3 Retention DELETE grants (DR-386)
`train.practice_attempts` is R9 and kept in this release (its period is Open doubts 8), so `akr_retention` gets SELECT
only until a period and its handler exist. `rpt.typology_exposure` is R7 `report_only`: SELECT only.
`casework.capacity_snapshots`, `casework.capacity_runs` and `admin.directory_events` are R12 `age` with no owner sweep,
so the engine's `record_purge` deletes them and their DELETE grant stays.

### 7.4 QA sampler citation
Section 4.5 item 9: the sampler is phase 19 section 3.8.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.5 ClassAggregate and the snapshot reads of the forecast (V-31-02-a, MEDIUM; DR-413)
Section 2.1: `ClassAggregate { pool_id: Uuid, work_kind: WorkKind, class_code: String, arrivals: i64, decided: i64, handling_minutes_sum: i64 }`. Each number is the sum over the window of the snapshot column of the same name, for pool rows only (the unassigned total is left out), grouped by pool, work kind and class. The type holds no person field. It is what `CapacityRepository::window_aggregate` returns and what step 1 of section 3.4 reads.
Section 2.2: `CapacityRepository` gains two reads that steps 1, 6, 9 and 10 of section 3.4 need and no method gave.
1. `snapshot_dates(from, to) -> Vec<NaiveDate>`: the distinct bank dates in the window that have a snapshot row. `W` of step 1 is the number of bank working days among them (phase 09 calendar).
2. `recent_buckets(hours: i64) -> Vec<(CapacitySnapshot, DateTime<Utc>)>`: the rows of the latest `hours` complete buckets, each with its `computed_at`. The forecast calls it with 24. `B_0` of step 6 is the sum of `backlog_end` over the pool's rows of the newest bucket. The hourly totals of step 9 are the sums of `arrivals` and `decided` per pool and bucket. `data_as_of` of step 10 is the newest `computed_at`.

### 7.6 Repository methods of the correspondent list and the pack links (V-31-04-b, MEDIUM; DR-413)
Section 2.2 gains two things.
1. `RfiPackRepository` (new; it owns `casework.rfi_packs`): `insert(tx, rfi_id, pack_id, linked_by, at) -> bool` with `ON CONFLICT (rfi_id, pack_id) DO NOTHING`, and `for_rfis(conn, rfi_ids: &[Uuid]) -> Vec<(Uuid, Uuid, String)>` (RFI id, pack id and pack kind, joined to `admin.packs`). `insert` returns `false` for a repeat of the pair; section 3.9 step 5 then answers 200 with no version bump and no second audit row.
2. The phase 19 `RfiRepository` gains `correspondent_page(conn, direction: Option<RfiDirection>, page) -> Vec<Rfi>`: the rows of `kind = 'correspondent'` under the same `visibility_sql` as `desk`, newest first. `desk` is not reused, because its filter is by status.
Section 2.3: `RfiService::attach_pack(ctx, rfi_id, body: AttachPackBody, if_match, idem)` and `RfiService::correspondent_list(ctx, direction: Option<RfiDirection>, cursor)`, which returns a cursor page of `CorrespondentRfiRow`. That row has the fields of the list in section 3.9: `id`, `direction`, `status`, `deadline_at`, `recipient_ref`, `questions_answered`, `questions_total`, `overdue` and `packs` (pack id and kind). The two question counts come from the phase 19 question and answer repositories.

## Decisions

**DR-338 (the training workspace lives in its own schema `train`).** Synthetic practice cases and attempts are in the new schema `train`, written only by `TrainingService` and the `training seed` command. A flag column on the live tables was rejected: every count, report, export, register, pack and goAML filing would need the exclusion, and one missed predicate would publish practice cases as live. With a separate schema the exclusion is a fact of the database: no view, function or join outside `train` references it, and a test proves it (section 6 item 4). The schema is created by `akr_migrate` (phase 00 owns the schema list, so this is the one new schema of the phase).

**DR-339 (practice results reach the register without a training record).** A practice pass is not a training record. The register of phase 24 keeps its rule that an administrator records a completion for another person, and the CHECK `user_id <> recorded_by` is untouched. The register shows a `practice` display column read from `train.practice_scores`, which scores only each trainee's first attempt per case (a pass after the known answer was shown is not a pass), and an administrator may cite `practice:<attempt id>` in the evidence reference of a completion. The column is never a count, so the live numbers stay the same with or without practice rows.

**DR-340 (the content of practice cases).** Cases come from the typology library (the typology numbers of phase 06 scenarios, research 27) and take their structure from the seeded fixtures of phase 16. Every value is invented, marked `synthetic: true`, and checked by the seed against every identifier of the canonical and case schemas before insert. The QA-confirmed golden set of phase 28 is not a source, because it is real case data. The known answer is a decision or a choice with its required reason codes; the score is the decision match plus the share of required reasons, and a pass needs `train.pass_pct`. The expert path is returned only after the trainee's own attempt.

**DR-341 (the capacity workbench measures teams, not people).** Inputs are counts per pool and bank week (staffed analysts, hours, shrinkage, occupancy target), and the handling times are measured from the timestamps of phase 17 (`created_at` to `triaged_at` for alerts; `opened_at` to `closed_at` for cases), aggregated per pool, work kind and class in an hourly snapshot with no person column. The product has no default handling time: until `capacity.min_working_days` measured working days exist for a class, the forecast says "insufficient data" and projects nothing. The formula is research 28 section 11.2 (workload, staffing with shrinkage and occupancy, the fluid backlog, Little's law as a check), and a pool with fewer people than the small-number threshold is not shown. Every output is advice (D21): no job writes a table of detection, screening or scenarios, and no threshold or limit is read for a write.

**DR-342 (the simulation harness reuses phase 16 replays).** The harness does not run its own replay. It takes a completed phase 16 replay, reads its daily would-alert counts (no entity identifier, no detection key), splits them by a stated class mix, and runs a priority-ordered fluid queue with the measured handling times and the staffing of the request. It reports queue peak, clock breaches as labelled estimates and analyst hours by week and class. It writes only `casework.capacity_runs` and activates nothing; a rule change that relies on it uses the replay as phase 04 evidence, as today. This is the M-76 harness of the roadmap, built on the replay engine that exists, not a second engine.

**DR-343 (surge mode: the parts that do not suppress alerts).** A surge carries a temporary pool (an `admin.pools` row, time-boxed reviewer grants), a higher quality-control sample while `capacity.surge_active` is on (the phase 19 sampler takes the larger of its rate and `capacity.surge_qc_percent`), and hourly progress on G2. Switching the surge on or off is a change request with the `capacity` reason tag under the phase 04 rules (material, a below-the-line evidence item, an approver outside operations). The pre-clearing of known false positives from pair memory is not carried: it would suppress alerts in bulk on a volume argument, which D21 forbids, and phase 14 pair memory suppresses screening candidates only, one approved pair at a time.

**DR-344 (the capacity forecast alarm is the only alarm of M-53 in this phase).** `capacity_forecast` is raised through the phase 19 `AlarmService`, subject `pool`, supervisor M, when the horizon backlog grows and the required analysts over the horizon exceed the staffed analysts. The supervisor is told through the existing notice `supervisor.alarm`, so no notice code is added. The growth, latency and silence alarms stay with phase 19 Open doubts 10.

**DR-345 (correspondent and validator packs from system facts only).** Both kinds are already in the CHECK of `admin.packs` (phases 02, 06 and 24), so no widening is needed, and `0100` asserts it. Each pack is built by the phase 06 `pack_build` job from system facts with a source and a `data_as_of` for each: tool and release, calibration, screening test runs, list-update lag, review metrics, correspondent request times, coverage certificates, validation packs and data-quality counts. The correspondent pack has the bank-level exposure as its respondent view. Counts below the phase 21 threshold are null, and the builder refuses any value that equals a case, report, party or account identifier. Download is the step-up `export` route of phase 06. This makes one evidence source for the correspondent questionnaire and the annual certification (GV-35), and the questionnaire text itself stays the bank's template.

**DR-346 (the typology exposure index).** For each month, scope (bank, branch or correspondent account) and typology, the index counts distinct posted transactions matched by alerts of scenarios tagged with the typology, and their LCY value, against the scope's total flows and value, and against its month-end balances. Values are LCY only, because IQD and USD are never added (T-04). Restricted scenarios are excluded for every viewer: their matches are suspicion-tier, and a count by typology and branch would disclose their existence. Cells below the threshold are null, a null is never zero, and the trend is a percentage-point change against the earlier months, flat inside a band that is a setting. The respondent view is the bank scope only, inside the correspondent pack.

**DR-347 (the correspondent side of the RFI desk).** The desk already carries correspondent requests (phase 19 DR-148). This phase adds the correspondent list with its filters and the attachment of a built pack to an outbound correspondent request while it is in draft. Nothing about a case is shown, the release stays the phase 19 external release, and an inbound request keeps its template. The counterparty is the `canon.account` identifier, because a correspondent is an account flagged `correspondent` (phase 10), and the bank has no other correspondent register to point to.

**DR-348 (SAML through Ory Polis).** Ory Polis is an optional catalogue entry, rendered only while `pack.polis` is on. It presents the bank IdP's SAML login as an OpenID Connect issuer, and Kratos uses it as the phase 25 generic OIDC upstream with no new provider. AKR-TMS never parses SAML and the upstream adds no authority: roles come only from phase 03 grants, the link to an account is the phase 25 explicit command, and the local second factor and step-up still apply. Polis gets its own database and role, its secrets from OpenBao, and only the pinned authorize, assertion consumer and SCIM paths at the edge, with its admin interface disabled; the webhook route is not reachable through the edge. This is the default that research 32 section 14.3 recommends for banks that ask for SAML.

**DR-349 (SCIM creates no account; a link is an administrator's act).** A SCIM create writes a `pending` directory row and nothing else. `admin.users.id` is the Kratos identity id (phase 01), and creating an identity with a password is a phase 03 act of an administrator (the one-time password shown once), so a directory event cannot create a sign-in account (SP-14). A linked account is created by an administrator, linked to its directory row with step-up `identity_admin`, and given roles only by change request `cr_access`. A directory-created person with no role can sign in after the link and sees only the `self` scope. The alternative of automatic account creation is Open doubts 17.

**DR-350 (SCIM deactivation disables the account; it never offboards).** A directory deactivation of a linked account calls `UserAdminService::disable_from_directory`, the phase 03 disable with `source: directory`: status `disabled` with reason `admin`, tokens and sessions revoked, grants kept. No step-up is consumed, as for every phase 03 disable, because the act is reversible. An unattended external signal never triggers an irreversible act: phase 03 offboarding is final, so one forged or mistaken event would otherwise end accounts for good. The final offboarding stays the phase 03 act of an administrator with step-up, and the daily reconciliation lists accounts disabled by the directory longer than `directory.pending_alert_days`. Phase 03's last-holder rule still refuses the last ADM or SEC, and an enable is refused while the directory row is inactive.

**DR-351 (conflicts and reconciliation).** The directory is authoritative for deactivation and for nothing else. A local enable is refused while the directory deactivation stands; a local offboarding wins over a directory activation; directory groups never change grants; a directory e-mail change is recorded as a mismatch and never applied to an account. A daily reconciliation repeats a failed disable, lists pending rows and directory-disabled accounts older than `directory.pending_alert_days` (the second list to the administrator, who decides the offboarding), and records mismatches, with counts in the audit log only.

**DR-352 (the webhook authenticates with a signature, and its principal holds one scope).** `POST /directory/events` is verified by the webhook signature of the pinned Polis version and the webhook secret in OpenBao, with a timestamp window of `directory.event_max_age_s`. The principal is the service principal `polis-directory` with the single scope `directory.sync`, and the phase 03 `Ctx.via` value `webhook_hmac` marks the path in Cedar. Every verified event answers 200 with its outcome, so Polis never retries a decision that was already applied, and only an unverified event answers 401.

**DR-353 (scopes and Cedar).** Two scopes are added to API.md section 2: `capacity.plan` (M only) and `directory.sync` (the one service principal). The Cedar files are `E0_directory.cedar`, `E1_training_capacity.cedar` and `E2_exposure_packs.cedar`: the phase 03 ranges had `E` free, and the next free names follow the phase 29 and phase 30 pattern. Roles are read through `principal.roles.contains`, one role per `@role` policy; the route scopes admit every role the policies permit (section 6 item 18).

**DR-354 (five flags, all off by default).** `pack.training`, `pack.capacity`, `pack.exposure`, `pack.correspondent` and `pack.polis` gate the parts of this phase. A flag-off route answers 404 `E_NOT_FOUND`, the jobs return without work, and the catalogue renders the Polis service only with its flag. A bank that never switches a flag on sees no change in any screen, route, count or container.

**DR-355 (screens and the names of services).** The practice workspace is W12 at `/training/practice`, the capacity workbench is G2 at `/capacity`, the correspondent list is a filter of the W8 desk, the two pack kinds are added to R4, the practice column is added to G9, the exposure view is a read-only panel of G1 that is not a tile, and the directory link is a panel of the A1 users screen. The roadmap row 31 names `TrainingService`, `CapacityService` and `CorrespondentPackService`; this phase uses those names and adds the others of section 2, which occur nowhere else in docs/plan.

**Lead corrections (review 2026-10-09).** A SCIM deactivation ran the final phase 03 offboarding with no person and no step-up, so one forged or mistaken directory event could end every linked account for good; it now runs a reversible phase 03 disable (`UserAdminService::disable_from_directory`, `source: directory`, grants kept), the reconciliation lists directory-disabled accounts to ADM, and the final offboarding stays an administrator's act with step-up (Cedar `user.disable`, not `user.offboard`; the outcome is `applied`, and a Kratos failure after the commit is the phase 03 `offboard_finalize` retry, DR-413). The edge exposed only `/sso/*` and `/scim/v2/*`, which missed the Polis authorize and assertion consumer paths, and left `POST /api/v1/directory/events` open to the bank network under the phase 00 `/api/*` rule; the edge now exposes the pinned Polis paths and refuses the webhook route. `train.practice_scores` counted a pass made after the trainee had seen the answer; it now scores each trainee's first attempt per case only. The alert-to-pool attribution was credited to phase 17, which has no such rule; it is stated as this phase's rule, with an `unassigned` total (`pool_id` NULL, `UNIQUE NULLS NOT DISTINCT`).

## Open doubts (for gap pass 1 and the bank)

1. **Authoring and course mapping.** Who writes the synthetic cases (DS or the compliance owner) and how the `course_code` of a case maps to the bank's course list (phase 24 Open doubts 7). The seed refuses non-synthetic files; it does not decide the content.
2. **Queue access (M-50).** Whether a failed or missing practice result closes queue access. Phase 19 Open doubts 14 keeps the calibration gate unbuilt. This phase links nothing to queue access.
3. **Pass mark.** `train.pass_pct` defaults to 70 with a floor of 60. The bank confirms the mark and whether reason codes count at all.
4. **Ageing of cases.** Open cases older than 60 calendar days are counted as aged (research 28 section 11.4, the default of its ageing alarm). Each class's SLA is a bank setting in research; this phase counts the 60-day default only. The alert-to-pool attribution (first active `alerts` queue by `code` whose filter admits the alert, else `unassigned`) is this phase's rule; the bank may prefer another, for example the pool of the claiming analyst's queue.
5. **Class mix of the simulation.** The replay's rows carry no priority class (phase 16 `scn.replay_results` has `bucket` and `segment_ref` only), so the class split is an input of the request. The bank confirms how the priority of a candidate scenario maps to P0 to P3, or accepts a stated input.
6. **Surge pre-clear.** The pre-clearing of known false positives (research 28 section 11.3) is not carried. The MLRO may want a triage order for known patterns without any closure; that is a new rule to be designed under D21, not a surge setting.
7. **Restricted scenarios in the index.** Matches of restricted scenarios are excluded from the exposure index for every viewer. A board view that counts them would need a suspicion-cleared reader and a review under phase 17 rule 4.
8. **Retention of practice attempts and directory events.** The class and clock are R9 by `attempted_at` and R12 by `received_at`. The bank confirms the period of each (the training evidence period and the operations-log period).
9. **Training coverage report.** Research 31 section 6.4 asks for coverage by role and branch. The register of phase 24 filters by user and course; a coverage view by role and branch needs the branch of each user, which is not built here.
10. **List-update lag.** Phase 14 stores no source publication time, so the lag of F4 is measured from the load to the activation. A publication-time column would be a change of phase 14, to be decided with the list feeds.
11. **Placement of the exposure view.** The proposal puts it as a read-only panel of G1 (not a tile). The lead decides whether it belongs on G1, on a new board screen, or in R4 with the board pack.
12. **Respondent view.** The default gives a correspondent the bank-level shares only. A bank may want a correspondent to see its own branch of flows; that would widen the view and needs a decision of the MLRO.
13. **Polis version and webhook format.** The pinned version and its licence (Apache-2.0 as research 32 records, v26.2.0) are confirmed before the catalogue digest is set. The exact header format of the directory-sync webhook signature, and the authorize, assertion consumer and SCIM paths that the edge exposes, are checked against the pinned version before the verifier and the Caddy rules are written; this section states no header name or path on purpose.
14. **Identity of the validator.** The validator pack is for an independent validator. Whether that is the bank's own model validator (MRO) or an external party is the bank's decision; the pack grants no access to anyone.
15. **Questionnaire content.** The CBDDQ and FCCQ question texts (W-06) are the bank's template content (phase 19 DR-149). This phase builds the evidence pack only.
16. **Deactivation without step-up (lead decision 2026-10-09).** A directory deactivation disables the account (reversible, no step-up, like every phase 03 disable) and never offboards; the final offboarding is the phase 03 act of an administrator with step-up. The bank may still want automatic offboarding after a number of days; that would make an external signal final and is refused by default.
17. **SCIM creates no account.** The alternative is automatic account creation for each directory create. It is rejected by default (DR-349) because identity creation is an administrator act; the bank may ask for it.
18. **Directory identifier changes.** A changed e-mail in the directory is recorded as a mismatch and not applied. The administrator changes the identifier through phase 03; the bank confirms this is the intended process.
19. **Scope of capacity planning.** `capacity.plan` is M only. The bank may give the input right to CH or to a named team lead; each would be a Cedar and scope change.
20. **Growth, latency and silence alarms.** M-53 growth, latency and silence alarms are not built (phase 19 Open doubts 10). The capacity forecast alarm of this phase is the only one.
21. **Small teams.** A pool with fewer than five analysts staffed (the threshold of phase 21) shows no pool row. Banks with many small pools may ask for pool groups; that is a design of the pool structure, not of this phase.
22. **Roadmap names.** Roadmap row 31 names the three services used here; the lead updates the roadmap service list for the other names of section 2.
23. **Reporting currency.** Exposure values and assets are LCY (IQD where the bank reports in IQD). The board may ask for a USD view; T-04 forbids adding the two, so any USD figure needs its own rate rule.
24. **Phase 03 extensions.** The disable method `disable_from_directory`, the payload key `source` of `user.disabled`, the enable check and the `Ctx.via` value `webhook_hmac` are additions to phase 03 that the phase 03 owner must accept.
25. **Trend band default.** `exposure.trend_band_pp` defaults to 0.5 percentage points. The bank confirms the band after one quarter of data.

## What This Phase Does NOT Touch

- No CBS connection, no `akr-ingestd` change, no read of the core banking system. The snapshotter and the exposure job read the canonical copy only.
- No change to `akr-assess`, the decision path, the gateway or the signed decision logic of phase 29.
- No change to any alert, case, decision, report, filing, screening result, list, scenario version, parameter set, threshold, limit, queue, touch window or work-in-progress limit. The capacity code is advice; section 6 item 7 proves its write set.
- No change to the training register's rules: records are still made by an administrator for another person; `admin.training_records` keeps its columns, guard and grants.
- No change to the phase 24 tiles, registers, command centre definitions, or packs of phases 02, 06 and 24, except the added register column and the kinds the builder now builds.
- No new scope other than `capacity.plan` and `directory.sync`; no new role; no new step-up class; no new notice code; no new outbox topic; no timer kind; no new audit kind outside section 3.14.
- No SAML or SCIM handling inside AKR-TMS: the SAML message, the IdP metadata and the SCIM protocol stay in the Polis container. No group, role or scope is taken from a directory.
- No new Kratos provider and no change to the phase 25 OIDC upstream beyond its configured issuer; no change to the local sign-in, second factor, session or step-up rules.
- No public or unauthenticated route; the webhook is the only route without a bearer token, and it is verified by signature and refused while its flag is off.
- No Kurdish content, no AI-generated practice content, no use of the golden set as a practice source.
- No change to `akr-readonly`: it gets no grant on any table of this phase.
- No new zone name and no change to the internal networks of phase 00; the Polis container is on two existing networks, with a host egress rule.
- No change to the software of the Ory stack (Kratos, Hydra) beyond the configured OIDC issuer.
- No visual change to any screen while its flag is off.
