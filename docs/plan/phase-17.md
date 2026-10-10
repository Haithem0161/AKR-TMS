# Phase 17: Alerts, Triage & Queues

**Goal:** The engine's output becomes work. The `AlertSink` of phase 16 is replaced by `AlertIntakeService`, which turns each new detection identity into one alert with its fingerprint, evidence snapshot and producing versions; records that cannot form an alert go to a quarantine that never drops them; each alert joins the one open case of its party and family (merged, never hidden); cases carry priority classes P0 to P3 set by a governed rule, not by score; analysts claim, triage and assign work under a work-in-progress limit and a conflict check; queues (on the pools of phase 01) and saved views show what each person may see, with counts that are masked at read time and pushed over SSE; screens W1 (My work) and W2 (queue) are built.

**Dependencies:** Phase 00 (jobs, outbox topics, timers, settings, schedules), Phase 01 (`StepUpService`, `admin.pools`, the preferences rule for `default_queue_id`), Phase 02 (`EvidenceStore::put_generated`, the `case_event` anchor stream, the canonical encoder, `AccessLog`), Phase 03 (Cedar bundle and schema, masking registry, purpose codes, `ConflictCheck` port), Phase 04 (`ChangeTargetRegistry`, the governed `setting` target, `EvidenceResolver`, `GET /inbox/counts`), Phase 09 (`BankCalendarClock`, reason-code families `closure` and `detection`, the `Priority` order), Phase 10 (`canon.party`, `canon.txn`), Phase 11 (freshness snapshot for `data_as_of`), Phase 12 (`admin.coverage_ledger`), Phase 13 (`RatingSource`), Phase 14 (`alert.screening_alerts` with its states and clocks; screens S1 to S4), Phase 15 (rating level through `RatingSource`; DR-103), Phase 16 (`AlertSink`, `DetectionPayload`, `scn.detection_keys`, `scn.shadow_results`, `scn.scenarios.visibility`)
**Macro-phase:** P1
**Complexity:** L (24 product routes and one dev route; the sink contract, the case aggregate and the visibility predicate carry the weight; see Open doubts on the XL boundary of the template)
**Binaries touched:** akr-api (routers `work`, `queues`, `alerts`, `alert_quarantine`, `cases`, `events`; the `_dev` fixture route in non-release builds; change target `queue_definition` registered in process) | akr-worker (`AlertIntakeService` as the sink inside the phase 16 run path; jobs `timer_alert_touch`, `timer_alert_age`, `case_chain_check`, `quarantine_retry`, `quarantine_release`, and `dev_detection_emit` in dev builds; outbox topic handlers `alert.created` and `case.event`) | akr-ingestd and akr-assess unchanged
**Requirements carried:**
- Implemented here: M-03 (priority class before score, shown on every row), M-06 (one vocabulary: queues hold alerts, cases and screening alerts only, never control incidents), M-07 (evidence snapshot, stable fingerprint and producing versions at creation; records that cannot form an alert go to quarantine), M-08 (one open case per party and family, attach at any age, merge never hides, ledger of merges; the reopen window is an Open doubt), M-17 (no screen ranks people by speed; counts are bank- and queue-level), M-18 (alert and case statuses as events; the status column is a projection), M-23 (business clocks on the bank calendar), M-57 (work-in-progress limit of 10), M-65 (visibility policy generated from the Cedar permits and tested; suspicion-tier rows absent from lists and counts for non-members), L-17 (suppression is merge: duplicates are grouped, every alert stays visible, grouping counts are kept)
- Partly implemented (the rest is named in Open doubts): M-09 (per-alert contribution outcome column and inheritance rule; phase 18 writes it), M-11 (latency stored per alert, handling clock, surplus alerts queue with an age clock; the flat-line detector is phase 19), M-12 (screening alerts read in the queues and escalated into cases; their states and clocks stay phase 14's), M-13 (score from displayed components; the ML term is not built), M-14 (queue definitions versioned by change request; the branch-first routing flag is not built), M-15 (pool pull for queues, push by assignment, the queue limit, conflict check at assignment; time off and handover are not built), M-16 (age raises priority, must-touch-by date; the daily forced sample is phase 19), M-19 (L1 and L2 differ on one case: service check and database constraint; the decision four-eyes is phase 18), M-22 (touch and age clocks through `TimerService`; the 50, 100 and 150 percent breach actions are phase 19 alarms), M-67 (conflict check at assignment; declared-conflict reassignment is phase 19), M-74 (case aggregate in Rust with property tests; the workflow runtime is phase 18), M-75 (priority rules and queue definitions governed and versioned; the policy pack as a whole stays phase 09, with `policy_pack_version` pinned per case), UX-08 (evidence-first alert view: at most three headline reasons beside their facts, thresholds hidden from branch roles; peer comparison later), UX-13 (bulk assign and mark duplicate only; tag, snooze and request information later), T-21 and T-22 are not in this phase (phase 19)
- Gap pass 1 (coverage), partly implemented: SP-54 (the sealed evidence snapshot at case open); L-27 (the weighted score and priority), L-31 (the data latency clock on each alert), L-52 (the work-in-progress limit per analyst); T-35 (one open case per party with every contributing alert and scenario), T-36 (priority class and a score from displayed components, with an age clock); UX-11 (Next on one key; drafts kept in the tab), UX-37 (the pause control for live updates), UX-42 (three headline reasons beside their facts with mitigating factors).
- Gap pass 1 (coverage), implemented here: T-03 (the frozen evidence snapshot on every alert), T-47 (the evidence-first alert page; thresholds never shown to branch roles); UX-05 (merged alerts as one case row with the merged count), UX-12 (the shortcut layer, section 7+), UX-16 (response budgets tested at 50 concurrent sessions, section 7+); VA-39 (one open case per party and family with its alerts as evidence); VB-33 (the evidence snapshot with producing versions and the decision record per alert).
- Gap pass 1 (coverage), not carried: UX-04 (workspace telemetry for the pilot, DR-400); VB-22 (auto-close guardrails, DR-399).

---

## 1. SQLx Migration Changes

Four migrations, `0058_casework_registry_queues` to `0061_alert_casework_guards_grants`, numbered from the lead's range (0058 is the first free number; `grep` of docs/plan confirms no other phase uses 0058 to 0061). The schemas `alert` and `casework` are created by phase 00. Tables created here: `casework.case_families`, `casework.queues`, `casework.saved_views`, `alert.alerts` (partitioned by month), `alert.alert_index`, `alert.alert_events`, `alert.quarantine`, `casework.cases`, `casework.case_events`, `casework.case_alerts`, `casework.case_parties` and `casework.party_tiers`, plus the sequence `casework.case_ref_seq`, the partition factory `alert.create_partition` and the read function `casework.effective_tier`. Phase 14's `alert.screening_alerts` receives one foreign key on `case_id` (its DDL says "no foreign key until casework.cases (phase 17)"). No other existing table changes.

**Reconciliation with DATA-MODEL section 12 (aligned in the same change, see DR-117 and DR-131):**
- `alert.alerts`: `list_version_id` and `model_version_id` are not created (screening alerts stay in phase 14; the model source comes with its own phase), `reason_code` becomes `reason_codes text[]` (the payload carries a list), `case_id` is not a column (the link is `casework.case_alerts`, one row per scenario alert). Added: `alert.alert_index` (the identity map), `window_from`, `window_to`, `emitted_at`, `priority_rule_version`, the touch and age timers, `snapshot_sha256`, the amendment columns.
- `casework.cases`: added `case_ref`, `first_worker_id`, `second_worker_id` (L1 and L2), `assignee_id`, `assignee_pool_id`, `last_seq` and `head_hash` (the case chain head). `closed_at`, `sealed_at`, `seal_hash` and `outcome` exist but are written by phase 18 only.
- `casework.case_alerts` and `casework.case_parties` gain a surrogate key and the source kind (screening alerts join cases here).
- New: `casework.case_families`, `casework.party_tiers`, `casework.queues`, `casework.saved_views`, `alert.alert_events`.

### 1.1 `0058_casework_registry_queues.sql`

```sql
-- migration 0058_casework_registry_queues.sql
-- Phase 17. The closed case-family list, queue definitions on the phase 01 pools, and saved views.
-- Owner role: akr_migrate (DDL); akr_app writes through the services of section 3.

CREATE TABLE IF NOT EXISTS casework.case_families (
    code            TEXT PRIMARY KEY CHECK (code ~ '^[a-z][a-z_]{1,31}$'),
    label           JSONB NOT NULL CHECK (label ? 'ar' AND label ? 'en'),
    tier            TEXT NOT NULL CHECK (tier IN ('normal', 'restricted', 'suspicion')),
    tf_class        BOOLEAN NOT NULL DEFAULT FALSE,   -- terrorist or proliferation financing: P0 by rule
    active          BOOLEAN NOT NULL DEFAULT TRUE,    -- families are deactivated, never deleted
    CHECK (NOT tf_class OR tier = 'suspicion')
);

CREATE TABLE IF NOT EXISTS casework.queues (
    id              UUID PRIMARY KEY,
    code            TEXT NOT NULL UNIQUE CHECK (code ~ '^[a-z][a-z0-9_]{1,31}$'),
    kind            TEXT NOT NULL CHECK (kind IN ('alerts')),   -- screening queues stay phase 14's (GET /screening/alerts); later kinds widen this
    name            JSONB NOT NULL CHECK (name ? 'ar' AND name ? 'en'),
    pool_id         UUID NOT NULL REFERENCES admin.pools(id) ON DELETE RESTRICT,   -- the pool kind must be `alerts` (service check)
    -- {"families": [...], "min_priority": "P0".."P3", "states": [...]}: a closed allow-list of keys (section 3.9)
    filter          JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(filter) = 'object'),
    order_key       TEXT NOT NULL DEFAULT 'class_score_age' CHECK (order_key IN ('class_score_age', 'age')),
    active          BOOLEAN NOT NULL DEFAULT TRUE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    version         INT NOT NULL DEFAULT 1
);
CREATE INDEX IF NOT EXISTS idx_queues_pool ON casework.queues (pool_id) WHERE active;

CREATE TABLE IF NOT EXISTS casework.saved_views (
    id              UUID PRIMARY KEY,
    queue_id        UUID NOT NULL REFERENCES casework.queues(id) ON DELETE RESTRICT,
    owner_id        UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    name            TEXT NOT NULL CHECK (length(btrim(name)) BETWEEN 1 AND 80),
    filter          JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(filter) = 'object'),
    shared          BOOLEAN NOT NULL DEFAULT FALSE,   -- shared views are visible to the queue's pool members
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    version         INT NOT NULL DEFAULT 1,
    CONSTRAINT uq_saved_views_owner_name UNIQUE (queue_id, owner_id, name)
);
CREATE INDEX IF NOT EXISTS idx_saved_views_shared ON casework.saved_views (queue_id) WHERE shared;

-- The closed list of case families (DSL.md scenario `case_family`, closed under E_SCHEMA). The DSL examples map ACCT to
-- `profile` and DORM to `dormant`, so the case family is not the scenario family code. Seed values are proposals (Open doubts).
INSERT INTO casework.case_families (code, label, tier, tf_class) VALUES
 ('cash',                  '{"ar": "نقد", "en": "Cash"}',                                          'normal',     FALSE),
 ('transfer',              '{"ar": "حوالات", "en": "Transfers"}',                                  'normal',     FALSE),
 ('trade',                 '{"ar": "تجارة", "en": "Trade"}',                                       'normal',     FALSE),
 ('islamic',               '{"ar": "تمويل إسلامي", "en": "Islamic finance"}',                      'normal',     FALSE),
 ('digital',               '{"ar": "قنوات رقمية", "en": "Digital channels"}',                      'normal',     FALSE),
 ('loan',                  '{"ar": "قروض", "en": "Loans"}',                                        'normal',     FALSE),
 ('profile',               '{"ar": "انحراف عن الملف", "en": "Profile deviation"}',                 'normal',     FALSE),
 ('pep',                   '{"ar": "أشخاص مكشوفو السياسة", "en": "Politically exposed persons"}', 'normal',     FALSE),
 ('public_funds',          '{"ar": "أموال عامة", "en": "Public funds"}',                           'suspicion',  FALSE),
 ('correspondent',         '{"ar": "مراسلون", "en": "Correspondent banking"}',                     'normal',     FALSE),
 ('terrorist_financing',   '{"ar": "تمويل الإرهاب", "en": "Terrorist financing"}',                 'suspicion',  TRUE),
 ('proliferation_financing','{"ar": "تمويل الانتشار", "en": "Proliferation financing"}',           'suspicion',  TRUE),
 ('insurance',             '{"ar": "تأمين", "en": "Insurance"}',                                   'normal',     FALSE),
 ('dormant',               '{"ar": "حسابات خاملة", "en": "Dormant accounts"}',                     'normal',     FALSE),
 ('cheque',                '{"ar": "شيكات", "en": "Cheques"}',                                     'normal',     FALSE),
 ('fx',                    '{"ar": "صرف العملات", "en": "Foreign exchange"}',                      'normal',     FALSE),
 ('other',                 '{"ar": "أخرى", "en": "Other"}',                                        'normal',     FALSE),
 ('observation',           '{"ar": "ملاحظات الفروع", "en": "Branch observations"}',                'normal',     FALSE)
ON CONFLICT (code) DO NOTHING;
```

### 1.2 `0059_alert_alerts_quarantine.sql`

```sql
-- migration 0059_alert_alerts_quarantine.sql
-- Phase 17. Alerts (partitioned by the month of creation, R5), the identity map to alerts, the alert event trail and the
-- quarantine. The identity itself is scn.detection_keys (phase 16, DR-117); alert_index makes "one alert per identity"
-- a database fact, because a partitioned table cannot carry a global UNIQUE on fingerprint.

CREATE TABLE IF NOT EXISTS alert.alerts (
    id                      UUID NOT NULL,
    fingerprint             TEXT NOT NULL CHECK (fingerprint ~ '^[0-9a-f]{64}$'),
    source_kind             TEXT NOT NULL CHECK (source_kind IN ('scenario', 'observation', 'model', 'manual')),
    scenario_id             UUID REFERENCES scn.scenarios(id) ON DELETE RESTRICT,
    scenario_code           TEXT,
    scenario_version_id     UUID REFERENCES scn.scenario_versions(id) ON DELETE RESTRICT,
    logic_version           TEXT,
    parameter_set_id        UUID REFERENCES scn.parameter_sets(id) ON DELETE RESTRICT,
    parameter_set_version   INT,
    mapping_versions        JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(mapping_versions) = 'array'),
    reference_versions      JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(reference_versions) = 'object'),
    cbi_pack_version        TEXT,
    register_version        TEXT,
    party_id                UUID NOT NULL,                          -- no foreign key: canonical party (DR-87)
    branch_id               UUID,
    window_from             DATE,
    window_to               DATE,
    data_as_of              JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(data_as_of) = 'object'),
    latency_s               INT NOT NULL CHECK (latency_s >= 0),    -- emitted_at minus window_to: the data latency clock
    run_id                  UUID,                                   -- scn.scenario_runs (no foreign key: phase 16 partitions)
    mode                    TEXT CHECK (mode IN ('incremental', 'settled', 'sweep')),
    reason_codes            TEXT[] NOT NULL CHECK (cardinality(reason_codes) >= 1),   -- DET_ codes; the first is the headline
    severity                TEXT NOT NULL CHECK (severity IN ('low', 'medium', 'high', 'critical')),
    case_family             TEXT NOT NULL REFERENCES casework.case_families(code) ON DELETE RESTRICT,
    template                TEXT NOT NULL CHECK (length(template) BETWEEN 1 AND 80),
    measures                JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(measures) = 'object'),        -- financial
    matched_txn_ids         UUID[] NOT NULL DEFAULT '{}' CHECK (cardinality(matched_txn_ids) <= 500),          -- personal
    matched_count           INT NOT NULL DEFAULT 0 CHECK (matched_count >= cardinality(matched_txn_ids)),
    links                   JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(links) = 'array'),          -- personal
    linked_party_ids        UUID[] NOT NULL DEFAULT '{}',
    degraded                BOOLEAN NOT NULL DEFAULT FALSE,
    degraded_reasons        TEXT[] NOT NULL DEFAULT '{}',
    coverage                JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(coverage) = 'object'),
    score                   NUMERIC(6,4) CHECK (score IS NULL OR score BETWEEN 0 AND 1),                       -- detection-logic
    score_components        JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(score_components) = 'object'),
    priority_class          TEXT NOT NULL CHECK (priority_class IN ('P0', 'P1', 'P2', 'P3')),
    priority_rule_version   INT NOT NULL CHECK (priority_rule_version >= 1),
    state                   TEXT NOT NULL DEFAULT 'queued'
                            CHECK (state IN ('queued', 'claimed', 'triaged', 'duplicate', 'no_longer_matches')),
    claimed_by              UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    claimed_at              TIMESTAMPTZ,
    triaged_by              UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    triaged_at              TIMESTAMPTZ,
    duplicate_of            UUID,                                   -- an alert of the same case (no foreign key: partitioned)
    touch_due_at            TIMESTAMPTZ,
    touch_timer_id          UUID,                                   -- no foreign key: fired timers are pruned (phase 00)
    age_timer_id            UUID,
    snapshot_object_id      UUID NOT NULL REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    snapshot_sha256         BYTEA NOT NULL CHECK (octet_length(snapshot_sha256) = 32),
    contribution_outcome    TEXT,                                   -- written by phase 18 only (M-09)
    amended_at              TIMESTAMPTZ,
    amendment_reason        TEXT CHECK (amendment_reason IN ('MeasuresChanged', 'LateRowsAdded', 'NoLongerMatches')),
    emitted_at              TIMESTAMPTZ NOT NULL,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    version                 INT NOT NULL DEFAULT 1,
    PRIMARY KEY (id, created_at),
    CHECK ((claimed_by IS NULL) = (claimed_at IS NULL)),
    CHECK ((triaged_by IS NULL) = (triaged_at IS NULL)),
    CHECK (state <> 'queued' OR claimed_by IS NULL),
    CHECK (state <> 'claimed' OR claimed_by IS NOT NULL),
    CHECK ((state = 'duplicate') = (duplicate_of IS NOT NULL)),
    CHECK ((state = 'triaged') = (triaged_at IS NOT NULL)),
    CHECK ((amended_at IS NULL) = (amendment_reason IS NULL)),
    CHECK ((source_kind = 'scenario') = (scenario_id IS NOT NULL AND scenario_version_id IS NOT NULL))
) PARTITION BY RANGE (created_at);

CREATE INDEX IF NOT EXISTS idx_alerts_queue    ON alert.alerts (priority_class, created_at) WHERE state IN ('queued', 'claimed');
CREATE INDEX IF NOT EXISTS idx_alerts_claimed  ON alert.alerts (claimed_by) WHERE state = 'claimed';
CREATE INDEX IF NOT EXISTS idx_alerts_touch    ON alert.alerts (touch_due_at) WHERE state IN ('queued', 'claimed');
CREATE INDEX IF NOT EXISTS idx_alerts_party    ON alert.alerts (party_id, created_at);
CREATE INDEX IF NOT EXISTS idx_alerts_scenario ON alert.alerts (scenario_version_id, created_at);

-- One row per detection identity that became an alert. Not partitioned, so the UNIQUE holds across months.
CREATE TABLE IF NOT EXISTS alert.alert_index (
    fingerprint     TEXT PRIMARY KEY REFERENCES scn.detection_keys(fingerprint) ON DELETE RESTRICT,
    id              UUID NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL,
    CONSTRAINT fk_alert_index_alert FOREIGN KEY (id, created_at) REFERENCES alert.alerts(id, created_at) ON DELETE RESTRICT
);
CREATE INDEX IF NOT EXISTS idx_alert_index_id ON alert.alert_index (id);

-- The alert's own trail (statuses as events, M-18). Not hash-chained: the audit_log chain of phase 02 covers each event.
CREATE TABLE IF NOT EXISTS alert.alert_events (
    id                  UUID PRIMARY KEY,
    alert_id            UUID NOT NULL,
    alert_created_at    TIMESTAMPTZ NOT NULL,
    seq                 INT NOT NULL CHECK (seq >= 1),
    at                  TIMESTAMPTZ NOT NULL DEFAULT now(),
    actor_id            UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    actor_kind          TEXT NOT NULL CHECK (actor_kind IN ('user', 'system', 'job')),
    kind                TEXT NOT NULL CHECK (kind ~ '^alert\.[a-z_]+$'),
    payload             JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(payload) = 'object'),   -- ids and codes only
    CONSTRAINT fk_alert_events_alert FOREIGN KEY (alert_id, alert_created_at) REFERENCES alert.alerts(id, created_at) ON DELETE RESTRICT,
    CONSTRAINT uq_alert_events_seq UNIQUE (alert_id, seq),
    CHECK ((actor_kind = 'user') = (actor_id IS NOT NULL))
);

-- Payloads that cannot form an alert. Never dropped; released or rejected by a person (section 3.2).
CREATE TABLE IF NOT EXISTS alert.quarantine (
    id                  UUID PRIMARY KEY,
    source_kind         TEXT NOT NULL CHECK (source_kind IN ('scenario', 'observation', 'model', 'manual')),
    fingerprint         TEXT CHECK (fingerprint IS NULL OR fingerprint ~ '^[0-9a-f]{64}$'),
    run_id              UUID,
    party_id            UUID,                              -- NULL when the payload names no resolvable party
    reason              TEXT NOT NULL CHECK (reason IN ('unknown_party', 'unknown_scenario_version', 'missing_versions',
                                                        'inconsistent_payload', 'unknown_case_family', 'unknown_reason_code',
                                                        'fingerprint_mismatch', 'coverage_missing')),
    reason_detail       JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(reason_detail) = 'object'),   -- codes and ids only
    payload             JSONB NOT NULL CHECK (jsonb_typeof(payload) = 'object'),   -- personal; sealed at rest (section 1.6)
    status              TEXT NOT NULL DEFAULT 'open' CHECK (status IN ('open', 'released', 'rejected')),
    resolution_code     TEXT CHECK (resolution_code IS NULL OR resolution_code ~ '^CLS_[A-Z0-9_]{2,60}$'),
    resolved_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    resolved_at         TIMESTAMPTZ,
    released_alert_id   UUID,
    revalidated_at      TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    CHECK ((status = 'open') = (resolved_at IS NULL)),
    CHECK ((status = 'rejected') = (resolution_code IS NOT NULL)),
    CHECK ((status = 'released') = (released_alert_id IS NOT NULL))
);
-- A replayed batch re-presents the same identity: one open quarantine row per identity and reason.
CREATE UNIQUE INDEX IF NOT EXISTS uq_quarantine_open_identity ON alert.quarantine (fingerprint, reason)
    WHERE status = 'open' AND fingerprint IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_quarantine_open ON alert.quarantine (reason, created_at) WHERE status = 'open';

-- The partition factory (phase 13 and 16 pattern). One month of one allow-listed table. No DEFAULT partition: a missing
-- month fails loudly and the writer calls the factory once and retries.
CREATE OR REPLACE FUNCTION alert.create_partition(p_table TEXT, p_year INT, p_month INT) RETURNS TEXT
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog
AS $$
DECLARE
    v_from TIMESTAMPTZ;
    v_to   TIMESTAMPTZ;
    v_name TEXT;
BEGIN
    IF p_table NOT IN ('alerts') THEN
        RAISE EXCEPTION 'alert.create_partition: % is not a partitioned alert table', p_table;
    END IF;
    IF p_year NOT BETWEEN 2000 AND 2200 OR p_month NOT BETWEEN 1 AND 12 THEN
        RAISE EXCEPTION 'alert.create_partition: month %-% out of range', p_year, p_month;
    END IF;
    PERFORM pg_catalog.pg_advisory_xact_lock(7307418562110395117);   -- fixed key: 'akr.alert.partition_factory'
    v_from := pg_catalog.make_timestamptz(p_year, p_month, 1, 0, 0, 0, 'UTC');
    v_to   := v_from + interval '1 month';
    v_name := pg_catalog.format('%s_%s_%s', p_table, p_year, pg_catalog.lpad(p_month::text, 2, '0'));
    IF NOT EXISTS (SELECT 1 FROM pg_catalog.pg_class c
                   JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace
                   WHERE n.nspname = 'alert' AND c.relname = v_name) THEN
        EXECUTE pg_catalog.format('CREATE TABLE alert.%I PARTITION OF alert.%I FOR VALUES FROM (%L) TO (%L)',
                                  v_name, p_table, v_from, v_to);
    END IF;
    RETURN v_name;
END $$;
REVOKE ALL ON FUNCTION alert.create_partition(TEXT, INT, INT) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION alert.create_partition(TEXT, INT, INT) TO akr_app, akr_migrate;

-- Seed partitions: the current month and the three ahead (audit.partition_ahead_months, default 3).
DO $$
DECLARE
    v_m DATE;
    i   INT;
BEGIN
    FOR i IN 0..3 LOOP
        v_m := (date_trunc('month', now()) + make_interval(months => i))::date;
        PERFORM alert.create_partition('alerts', EXTRACT(YEAR FROM v_m)::int, EXTRACT(MONTH FROM v_m)::int);
    END LOOP;
END $$;
```

### 1.3 `0060_casework_cases_events_links.sql`

```sql
-- migration 0060_casework_cases_events_links.sql
-- Phase 17. Cases (R6), their hash-chained event trail, the alert and screening-alert links, the parties of a case, and
-- the tiers that casework raises on parties (DR-123). One open case per party and family is a database fact (partial UNIQUE).

CREATE SEQUENCE IF NOT EXISTS casework.case_ref_seq AS BIGINT START 1;

CREATE TABLE IF NOT EXISTS casework.cases (
    id                  UUID PRIMARY KEY,
    case_ref            TEXT NOT NULL UNIQUE CHECK (case_ref ~ '^CS-[0-9]{4}-[0-9]{6}$'),   -- from casework.case_ref_seq
    family              TEXT NOT NULL REFERENCES casework.case_families(code) ON DELETE RESTRICT,
    party_id            UUID NOT NULL,                       -- the subject; no foreign key (canonical party, DR-87)
    group_id            UUID,                                -- related-party group from evidenced links only (section 3.6)
    parent_case_id      UUID REFERENCES casework.cases(id) ON DELETE RESTRICT,   -- network parent (phase 19)
    merged_into         UUID REFERENCES casework.cases(id) ON DELETE RESTRICT,
    status              TEXT NOT NULL CHECK (status IN ('open', 'in_review', 'closed', 'merged')),
    priority_class      TEXT NOT NULL CHECK (priority_class IN ('P0', 'P1', 'P2', 'P3')),   -- the highest class of its alerts
    tier                TEXT NOT NULL CHECK (tier IN ('normal', 'restricted', 'suspicion')),   -- only rises (section 3.5)
    outcome             TEXT,                                -- phase 18 decisions (DATA-MODEL s.12); no CHECK here
    opened_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at           TIMESTAMPTZ,                         -- phase 18 closing sequence
    sealed_at           TIMESTAMPTZ,                         -- phase 18 (M-38, M-61)
    seal_hash           BYTEA CHECK (seal_hash IS NULL OR octet_length(seal_hash) = 32),
    policy_pack_version TEXT NOT NULL,                       -- pinned at opening (M-75)
    workflow_version    TEXT,                                -- phase 18
    first_worker_id     UUID REFERENCES admin.users(id) ON DELETE RESTRICT,    -- L1
    second_worker_id    UUID REFERENCES admin.users(id) ON DELETE RESTRICT,    -- L2
    assignee_id         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    assignee_pool_id    UUID REFERENCES admin.pools(id) ON DELETE RESTRICT,
    last_seq            INT NOT NULL DEFAULT 0 CHECK (last_seq >= 0),
    head_hash           BYTEA NOT NULL DEFAULT decode(repeat('00', 32), 'hex') CHECK (octet_length(head_hash) = 32),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    CHECK ((status = 'closed') = (closed_at IS NOT NULL)),
    CHECK ((status = 'merged') = (merged_into IS NOT NULL)),
    CHECK (sealed_at IS NULL OR closed_at IS NOT NULL),
    CHECK ((seal_hash IS NULL) = (sealed_at IS NULL)),
    CHECK (second_worker_id IS NULL OR first_worker_id IS NOT NULL),
    CHECK (second_worker_id IS NULL OR second_worker_id <> first_worker_id),   -- L1 and L2 differ (M-19)
    CHECK (merged_into IS NULL OR merged_into <> id)
);
-- One open case per party and family (M-08). A merged or closed case leaves the index.
CREATE UNIQUE INDEX IF NOT EXISTS uq_cases_open_party_family ON casework.cases (party_id, family)
    WHERE status IN ('open', 'in_review');
CREATE INDEX IF NOT EXISTS idx_cases_queue    ON casework.cases (priority_class, opened_at) WHERE status IN ('open', 'in_review');
CREATE INDEX IF NOT EXISTS idx_cases_assignee ON casework.cases (assignee_id) WHERE status IN ('open', 'in_review');
CREATE INDEX IF NOT EXISTS idx_cases_party    ON casework.cases (party_id);
CREATE INDEX IF NOT EXISTS idx_cases_group    ON casework.cases (group_id) WHERE group_id IS NOT NULL;

-- Append-only event trail. Per-case chain: prev_hash is the hash of seq - 1 (32 zero bytes for seq 1). The hash is the
-- phase 02 canonical encoding of the row without `hash` (section 3.7). The chain is written under the case row lock.
CREATE TABLE IF NOT EXISTS casework.case_events (
    case_id     UUID NOT NULL REFERENCES casework.cases(id) ON DELETE RESTRICT,
    seq         INT NOT NULL CHECK (seq >= 1),
    at          TIMESTAMPTZ NOT NULL,
    actor_id    UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    actor_kind  TEXT NOT NULL CHECK (actor_kind IN ('user', 'system', 'job')),
    kind        TEXT NOT NULL CHECK (kind ~ '^case\.[a-z_]+$'),
    payload     JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(payload) = 'object'),   -- ids and codes only
    prev_hash   BYTEA NOT NULL CHECK (octet_length(prev_hash) = 32),
    hash        BYTEA NOT NULL CHECK (octet_length(hash) = 32),
    PRIMARY KEY (case_id, seq),
    CHECK ((actor_kind = 'user') = (actor_id IS NOT NULL))
);
CREATE INDEX IF NOT EXISTS idx_case_events_at ON casework.case_events (at, case_id);

-- The link of record between a case and its alerts. A scenario alert has exactly one row (UNIQUE below); a screening alert
-- joins when it is escalated or confirmed (M-12). Merges move rows (UPDATE case_id); rows are never deleted.
CREATE TABLE IF NOT EXISTS casework.case_alerts (
    id                  UUID PRIMARY KEY,
    case_id             UUID NOT NULL REFERENCES casework.cases(id) ON DELETE RESTRICT,
    source_kind         TEXT NOT NULL CHECK (source_kind IN ('scenario', 'screening')),
    alert_id            UUID,
    alert_created_at    TIMESTAMPTZ,
    screening_alert_id  UUID REFERENCES alert.screening_alerts(id) ON DELETE RESTRICT,
    duplicate_of        UUID,                       -- an alert of the same case, marked by a person; the row stays visible
    attached_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    attached_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    CONSTRAINT fk_case_alerts_alert FOREIGN KEY (alert_id, alert_created_at)
        REFERENCES alert.alerts(id, created_at) ON DELETE RESTRICT,
    CHECK ((source_kind = 'scenario') = (alert_id IS NOT NULL AND alert_created_at IS NOT NULL)),
    CHECK ((source_kind = 'screening') = (screening_alert_id IS NOT NULL)),
    CHECK (duplicate_of IS NULL OR duplicate_of <> alert_id)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_case_alerts_alert     ON casework.case_alerts (alert_id) WHERE source_kind = 'scenario';
CREATE UNIQUE INDEX IF NOT EXISTS uq_case_alerts_screening ON casework.case_alerts (screening_alert_id) WHERE source_kind = 'screening';
CREATE INDEX IF NOT EXISTS idx_case_alerts_case ON casework.case_alerts (case_id);

CREATE TABLE IF NOT EXISTS casework.case_parties (
    case_id     UUID NOT NULL REFERENCES casework.cases(id) ON DELETE RESTRICT,
    party_id    UUID NOT NULL,                       -- no foreign key (canonical party)
    role        TEXT NOT NULL CHECK (role IN ('subject', 'related', 'counterparty', 'reporter')),
    added_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    added_by    UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    PRIMARY KEY (case_id, party_id, role)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_case_subject ON casework.case_parties (case_id) WHERE role = 'subject';
CREATE INDEX IF NOT EXISTS idx_case_parties_party ON casework.case_parties (party_id);

-- Tiers raised by casework on a party (DR-123). canon.party.sensitivity_tier stays the ingest's; masking reads the higher of
-- the two (section 3.10). A row is raised, never lowered, and never deleted.
CREATE TABLE IF NOT EXISTS casework.party_tiers (
    party_id        UUID PRIMARY KEY,                -- no foreign key (canonical party)
    tier            TEXT NOT NULL CHECK (tier IN ('restricted', 'suspicion')),
    source_case_id  UUID NOT NULL REFERENCES casework.cases(id) ON DELETE RESTRICT,
    raised_by       UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- NULL when the system raised it at case opening
    raised_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    version         INT NOT NULL DEFAULT 1
);
```

### 1.4 Migration notes

- Order: 0058 (families, queues, views) precedes 0060 (cases reference families and pools). 0059 precedes 0060 because
  `casework.case_alerts` takes a composite foreign key on `alert.alerts (id, created_at)`.
- `casework.case_families` rows are seeded here, not by the bank: a new family is a release (Open doubts).
- `casework.queues` rows are not seeded by migration (their `pool_id` is an install fact). `akr-ctl queues seed` creates the
  queues through the `queue_definition` change target (section 3.9); the first install may go live, later changes by request.
- `casework.case_ref_seq` is granted to `akr_app` in 0061. A case reference is `CS-<year>-<6 digits>`, generated by the
  service from the sequence in the opening transaction.

### 1.5 `0061_alert_casework_guards_grants.sql`

```sql
-- migration 0061_alert_casework_guards_grants.sql
-- Phase 17. Guards (second layer under the grants), grants per write path with UPDATE column lists, the foreign key
-- phase 14 deferred, the settings and schedules, the classification and the retention rows.

-- Append-only: alert_index, alert_events and case_events.
CREATE OR REPLACE FUNCTION casework.append_only() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION '% is append-only', TG_TABLE_NAME USING ERRCODE = 'insufficient_privilege';
END $$;
DROP TRIGGER IF EXISTS trg_case_events_append_only ON casework.case_events;
CREATE TRIGGER trg_case_events_append_only BEFORE UPDATE OR DELETE ON casework.case_events
    FOR EACH ROW EXECUTE FUNCTION casework.append_only();
DROP TRIGGER IF EXISTS trg_alert_events_append_only ON alert.alert_events;
CREATE TRIGGER trg_alert_events_append_only BEFORE UPDATE OR DELETE ON alert.alert_events
    FOR EACH ROW EXECUTE FUNCTION casework.append_only();
DROP TRIGGER IF EXISTS trg_alert_index_append_only ON alert.alert_index;
CREATE TRIGGER trg_alert_index_append_only BEFORE UPDATE OR DELETE ON alert.alert_index
    FOR EACH ROW EXECUTE FUNCTION casework.append_only();

-- Chain second layer: a case event must chain from the event before it (the verifier of section 3.7 checks the hashes).
CREATE OR REPLACE FUNCTION casework.case_events_chain_guard() RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE
    v_prev BYTEA;
BEGIN
    IF NEW.seq = 1 THEN
        IF NEW.prev_hash <> decode(repeat('00', 32), 'hex') THEN
            RAISE EXCEPTION 'case chain: the first event chains from 32 zero bytes' USING ERRCODE = 'check_violation';
        END IF;
        RETURN NEW;
    END IF;
    SELECT hash INTO v_prev FROM casework.case_events WHERE case_id = NEW.case_id AND seq = NEW.seq - 1;
    IF NOT FOUND OR NEW.prev_hash <> v_prev THEN
        RAISE EXCEPTION 'case chain: prev_hash does not match seq %', NEW.seq - 1 USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_case_events_chain ON casework.case_events;
CREATE TRIGGER trg_case_events_chain BEFORE INSERT ON casework.case_events
    FOR EACH ROW EXECUTE FUNCTION casework.case_events_chain_guard();

-- Alerts: never deleted; identity, party and versions immutable; state moves only along the section 3.8 table.
CREATE OR REPLACE FUNCTION alert.alerts_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'alerts are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.id <> OLD.id OR NEW.fingerprint <> OLD.fingerprint OR NEW.source_kind <> OLD.source_kind
       OR NEW.party_id <> OLD.party_id OR NEW.created_at <> OLD.created_at OR NEW.emitted_at <> OLD.emitted_at
       OR NEW.scenario_version_id IS DISTINCT FROM OLD.scenario_version_id
       OR NEW.priority_rule_version < OLD.priority_rule_version THEN
        RAISE EXCEPTION 'alert identity, party and versions are immutable' USING ERRCODE = 'check_violation';
    END IF;
    IF NEW.state IS DISTINCT FROM OLD.state AND NOT (
           (OLD.state = 'queued'     AND NEW.state IN ('claimed', 'duplicate', 'no_longer_matches'))
        OR (OLD.state = 'claimed'    AND NEW.state IN ('queued', 'triaged', 'duplicate', 'no_longer_matches'))
        OR (OLD.state = 'triaged'    AND NEW.state = 'no_longer_matches')
        OR (OLD.state = 'duplicate'  AND NEW.state = 'no_longer_matches')
        OR (OLD.state = 'no_longer_matches' AND NEW.state = 'queued')) THEN   -- the pattern holds again (section 3.13)
        RAISE EXCEPTION 'alert state % -> % is not allowed', OLD.state, NEW.state USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_alerts_guard ON alert.alerts;
CREATE TRIGGER trg_alerts_guard BEFORE UPDATE OR DELETE ON alert.alerts
    FOR EACH ROW EXECUTE FUNCTION alert.alerts_guard();

-- Quarantine: the payload never changes; a resolved row never changes; no deletes.
CREATE OR REPLACE FUNCTION alert.quarantine_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'quarantined payloads are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.payload IS DISTINCT FROM OLD.payload OR NEW.reason <> OLD.reason OR NEW.id <> OLD.id THEN
        RAISE EXCEPTION 'a quarantined payload is immutable' USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.status <> 'open' AND (NEW.status IS DISTINCT FROM OLD.status OR NEW.resolution_code IS DISTINCT FROM OLD.resolution_code) THEN
        RAISE EXCEPTION 'a resolved quarantine row does not change' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_quarantine_guard ON alert.quarantine;
CREATE TRIGGER trg_quarantine_guard BEFORE UPDATE OR DELETE ON alert.quarantine
    FOR EACH ROW EXECUTE FUNCTION alert.quarantine_guard();

-- Cases: never deleted; identity immutable; merged is terminal (a merged case never changes status again).
CREATE OR REPLACE FUNCTION casework.cases_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'cases are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.id <> OLD.id OR NEW.case_ref <> OLD.case_ref OR NEW.family <> OLD.family
       OR NEW.party_id <> OLD.party_id OR NEW.opened_at <> OLD.opened_at THEN
        RAISE EXCEPTION 'case identity is immutable' USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.status = 'merged' AND (NEW.status <> OLD.status OR NEW.merged_into <> OLD.merged_into) THEN
        RAISE EXCEPTION 'a merged case is terminal' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_cases_guard ON casework.cases;
CREATE TRIGGER trg_cases_guard BEFORE UPDATE OR DELETE ON casework.cases
    FOR EACH ROW EXECUTE FUNCTION casework.cases_guard();

-- Party tiers: raised, never lowered, never deleted (DR-123).
CREATE OR REPLACE FUNCTION casework.party_tiers_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'party tiers are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.party_id <> OLD.party_id THEN
        RAISE EXCEPTION 'party tier key is immutable' USING ERRCODE = 'check_violation';
    END IF;
    IF (CASE OLD.tier WHEN 'restricted' THEN 1 ELSE 2 END) > (CASE NEW.tier WHEN 'restricted' THEN 1 ELSE 2 END) THEN
        RAISE EXCEPTION 'a party tier is never lowered by casework' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_party_tiers_guard ON casework.party_tiers;
CREATE TRIGGER trg_party_tiers_guard BEFORE UPDATE OR DELETE ON casework.party_tiers
    FOR EACH ROW EXECUTE FUNCTION casework.party_tiers_guard();

-- The tier every masked reader uses (DR-123): the higher of the ingest's tier and the tier casework raised.
CREATE OR REPLACE FUNCTION casework.effective_tier(p_party UUID) RETURNS TEXT
LANGUAGE sql STABLE SECURITY INVOKER SET search_path = pg_catalog AS $$
    SELECT CASE GREATEST(
             COALESCE((SELECT CASE p.sensitivity_tier WHEN 'suspicion' THEN 2 WHEN 'restricted' THEN 1 ELSE 0 END
                         FROM canon.party p WHERE p.id = p_party AND p.recorded_to IS NULL
                        ORDER BY p.valid_from DESC LIMIT 1), 0),
             COALESCE((SELECT CASE t.tier WHEN 'suspicion' THEN 2 ELSE 1 END
                         FROM casework.party_tiers t WHERE t.party_id = p_party), 0))
           WHEN 2 THEN 'suspicion' WHEN 1 THEN 'restricted' ELSE 'normal' END
$$;
REVOKE ALL ON FUNCTION casework.effective_tier(UUID) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION casework.effective_tier(UUID) TO akr_app;

-- Grants. Every write path, with the UPDATE column lists the services use. Phase 18 adds its own column grants for
-- cases (closed_at, sealed_at, seal_hash, outcome, workflow_version) and for alerts (contribution_outcome).
GRANT SELECT, INSERT ON alert.alerts, alert.alert_index, alert.alert_events, alert.quarantine TO akr_app;
GRANT UPDATE (state, claimed_by, claimed_at, triaged_by, triaged_at, duplicate_of, priority_class, priority_rule_version,
              touch_due_at, touch_timer_id, age_timer_id, snapshot_object_id, snapshot_sha256, measures, matched_txn_ids,
              matched_count, links, linked_party_ids, degraded, degraded_reasons, coverage, score, score_components,
              amended_at, amendment_reason, updated_at, version)
    ON alert.alerts TO akr_app;
GRANT UPDATE (status, resolution_code, resolved_by, resolved_at, released_alert_id, revalidated_at, reason_detail, updated_at, version)
    ON alert.quarantine TO akr_app;
GRANT SELECT, INSERT ON casework.cases TO akr_app;
GRANT UPDATE (status, priority_class, tier, group_id, merged_into, first_worker_id, second_worker_id, assignee_id,
              assignee_pool_id, last_seq, head_hash, updated_at, version)
    ON casework.cases TO akr_app;
GRANT USAGE, SELECT ON SEQUENCE casework.case_ref_seq TO akr_app;
GRANT SELECT, INSERT ON casework.case_events TO akr_app;              -- no UPDATE, no DELETE: the trigger is the second layer
GRANT SELECT, INSERT ON casework.case_alerts TO akr_app;
GRANT UPDATE (case_id, duplicate_of) ON casework.case_alerts TO akr_app;   -- merges and duplicate marks
GRANT SELECT, INSERT ON casework.case_parties TO akr_app;
GRANT SELECT, INSERT ON casework.party_tiers TO akr_app;
GRANT UPDATE (tier, source_case_id, raised_by, raised_at, version) ON casework.party_tiers TO akr_app;
GRANT SELECT ON casework.case_families TO akr_app;
GRANT SELECT, INSERT ON casework.queues TO akr_app;                    -- written by the queue_definition change apply only
GRANT UPDATE (name, pool_id, filter, order_key, active, updated_at, version) ON casework.queues TO akr_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON casework.saved_views TO akr_app;
GRANT SELECT (case_id, seq, at, hash) ON casework.case_events TO akr_readonly;   -- the phase 02 verifier rebuilds the anchored heads; no payload and no actor column
-- No other grant to akr_readonly, and none to akr_ingest, akr_audit_writer or akr_monitor: these tables are personal and are read through the API.
-- Phase 14 already grants UPDATE (case_id) on alert.screening_alerts to akr_app; no grant is added here for it.

-- The one foreign key phase 14 deferred (its DDL: "no foreign key until casework.cases (phase 17)").
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'fk_screening_alerts_case') THEN
        ALTER TABLE alert.screening_alerts ADD CONSTRAINT fk_screening_alerts_case
            FOREIGN KEY (case_id) REFERENCES casework.cases(id) ON DELETE RESTRICT;
    END IF;
END $$;
```

### 1.6 `0061` (continued): settings, schedules, classification, retention, sealing

The rows below are part of migration `0061_alert_casework_guards_grants.sql` (one file, two blocks). Settings copy the phase 14
column list. `floor` is NULL throughout: this phase has no regulatory minimum of the `{min}` shape. `governed` is TRUE only where
a change must go through a change request (phase 04 `setting` target, route `cr_material` for a loosening, `cr_minor` for a
tightening, per the route map).

```sql
-- Phase 17 settings and schedules (DATA-MODEL s.12; phase 04 governed settings; phase 00 cron format).
INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('casework.priority_rules',
  '{"version": 1, "P0": [{"family_tf": true}, {"screening_class": "P0"}, {"authority_order": true}, {"stop_until_cleared": true}], "P1": [{"severity": "critical"}, {"screening_class": "P1"}, {"risk_level_gte": 4}], "P2": [{"severity": "high"}, {"screening_class": "P2"}, {"risk_level_gte": 3}], "P3": "default"}',
  'json', NULL, TRUE,
  '{"ar": "قواعد فئات الأولوية P0 إلى P3 (أول قاعدة تطابق)", "en": "Priority class rules P0 to P3 (first matching class wins)"}'),
 ('casework.score_weights',
  '{"typology": 0.20, "size": 0.25, "customer_risk": 0.20, "network": 0.10, "history": 0.10, "data_confidence": 0.15}',
  'json', NULL, TRUE,
  '{"ar": "أوزان مكونات درجة التنبيه (مجموعها 1)", "en": "Alert score component weights (sum to 1)"}'),
 ('casework.touch_by_hours',
  '{"P0": 1, "P1": 4, "P2": 24, "P3": 72}',
  'json', NULL, TRUE,
  '{"ar": "مهلة اللمس الأولى لكل فئة (ساعات عمل)", "en": "Must-touch-by hours per class (working hours, starting values)"}'),
 ('casework.age_raise_hours',          '24', 'integer', NULL, TRUE,
  '{"ar": "ساعات عمل قبل رفع فئة التنبيه خطوة واحدة (لا يتجاوز P0)", "en": "Working hours before a class rises one step (never above P0)"}'),
 ('casework.wip_limit',                '10', 'integer', NULL, TRUE,
  '{"ar": "حد العمل الجاري لكل محلل (تنبيهات مطالَبة وقضايا مسندة)", "en": "Work-in-progress limit per analyst (claimed alerts and assigned open cases)"}'),
 ('casework.reopen_window_days',       '30', 'integer', NULL, TRUE,
  '{"ar": "نافذة إعادة فتح القضية بعد الإغلاق (أيام)", "en": "Reopen window after closure (days; the automatic reopen is an Open doubt)"}'),
 ('casework.group_link_kinds',         '["family_no", "signatory", "owner"]', 'json', NULL, TRUE,
  '{"ar": "أنواع الروابط المؤكدة التي تجمع الأطراف في مجموعة", "en": "Evidenced link kinds that may group parties"}'),
 ('casework.history_window_days',      '180', 'integer', NULL, FALSE,
  '{"ar": "نافذة تاريخ التنبيهات لحساب مكون التاريخ (أيام)", "en": "Alert history window for the score (days)"}'),
 ('casework.bulk_cap',                 '25', 'integer', NULL, FALSE,
  '{"ar": "أقصى عدد للعناصر في الإجراء الجماعي", "en": "Largest bulk action (items)"}'),
 ('casework.saved_views_per_user',     '20', 'integer', NULL, FALSE,
  '{"ar": "أقصى عدد للعروض المحفوظة لكل مستخدم", "en": "Saved views per user"}'),
 ('casework.counts_cache_s',           '10', 'integer', NULL, FALSE,
  '{"ar": "مدة تخزين عدادات الطوابير (ثوان)", "en": "Queue counts cache lifetime (seconds)"}'),
 ('casework.sse_heartbeat_s',          '25', 'integer', NULL, FALSE,
  '{"ar": "نبضة الاتصال لقناة العدادات (ثوان)", "en": "Heartbeat of the counts stream (seconds)"}'),
 ('schedule.case_chain_check',         '"0 40 2 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول التحقق من سلاسل أحداث القضايا", "en": "Case event chain check schedule"}'),
 ('schedule.quarantine_retry',         '"0 15 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول إعادة التحقق من تنبيهات الحجر", "en": "Quarantine re-validation schedule"}')
ON CONFLICT (key) DO NOTHING;

-- Classification (DATA-MODEL s.0 classes). `financial` covers money amounts in measures; `personal` covers links and the
-- matched transaction ids (they name customer activity); `detection-logic` covers scores and weights.
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'alert', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('alerts', 'internal', ARRAY['id', 'fingerprint', 'source_kind', 'scenario_id', 'scenario_code', 'scenario_version_id',
                               'logic_version', 'parameter_set_id', 'parameter_set_version', 'mapping_versions',
                               'reference_versions', 'cbi_pack_version', 'register_version', 'party_id', 'branch_id',
                               'window_from', 'window_to', 'data_as_of', 'latency_s', 'run_id', 'mode', 'reason_codes',
                               'severity', 'case_family', 'template', 'matched_count', 'linked_party_ids', 'degraded',
                               'degraded_reasons', 'coverage', 'priority_class', 'priority_rule_version', 'state',
                               'claimed_by', 'claimed_at', 'triaged_by', 'triaged_at', 'duplicate_of', 'touch_due_at',
                               'touch_timer_id', 'age_timer_id', 'snapshot_object_id', 'snapshot_sha256',
                               'contribution_outcome', 'amended_at', 'amendment_reason', 'emitted_at', 'created_at',
                               'updated_at', 'version']),
  ('alerts', 'personal', ARRAY['links', 'matched_txn_ids']),
  ('alerts', 'financial', ARRAY['measures']),
  ('alerts', 'detection-logic', ARRAY['score', 'score_components']),
  ('alert_index', 'internal', ARRAY['fingerprint', 'id', 'created_at']),
  ('alert_events', 'internal', ARRAY['id', 'alert_id', 'alert_created_at', 'seq', 'at', 'actor_id', 'actor_kind', 'kind',
                                     'payload']),
  ('quarantine', 'internal', ARRAY['id', 'source_kind', 'fingerprint', 'run_id', 'party_id', 'reason', 'reason_detail',
                                   'status', 'resolution_code', 'resolved_by', 'resolved_at', 'released_alert_id',
                                   'revalidated_at', 'created_at', 'updated_at', 'version']),
  ('quarantine', 'personal', ARRAY['payload'])
) AS v(t, cls, cols)
CROSS JOIN LATERAL unnest(v.cols) AS c
ON CONFLICT DO NOTHING;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'casework', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('cases', 'internal', ARRAY['id', 'case_ref', 'family', 'party_id', 'group_id', 'parent_case_id', 'merged_into', 'status',
                              'priority_class', 'tier', 'outcome', 'opened_at', 'closed_at', 'sealed_at', 'seal_hash',
                              'policy_pack_version', 'workflow_version', 'first_worker_id', 'second_worker_id',
                              'assignee_id', 'assignee_pool_id', 'last_seq', 'head_hash', 'created_at', 'updated_at',
                              'version']),
  ('case_events', 'internal', ARRAY['case_id', 'seq', 'at', 'actor_id', 'actor_kind', 'kind', 'payload', 'prev_hash', 'hash']),
  ('case_alerts', 'internal', ARRAY['id', 'case_id', 'source_kind', 'alert_id', 'alert_created_at', 'screening_alert_id',
                                    'duplicate_of', 'attached_at', 'attached_by']),
  ('case_parties', 'internal', ARRAY['case_id', 'party_id', 'role', 'added_at', 'added_by']),
  ('party_tiers', 'internal', ARRAY['party_id', 'tier', 'source_case_id', 'raised_by', 'raised_at', 'version']),
  ('case_families', 'internal', ARRAY['code', 'label', 'tier', 'tf_class', 'active']),
  ('queues', 'internal', ARRAY['id', 'code', 'kind', 'name', 'pool_id', 'filter', 'order_key', 'active', 'created_at',
                               'updated_at', 'version']),
  ('saved_views', 'internal', ARRAY['id', 'queue_id', 'owner_id', 'name', 'filter', 'shared', 'created_at', 'updated_at',
                                    'version'])
) AS v(t, cls, cols)
CROSS JOIN LATERAL unnest(v.cols) AS c
ON CONFLICT DO NOTHING;

-- Sealed at rest (phase 02 `personal` column seal, AAD = row id): the quarantined payload.
UPDATE admin.column_classification SET encrypted = TRUE
 WHERE (schema_name, table_name, column_name) IN (('alert', 'quarantine', 'payload'));

-- Retention (phase 00 clocks; R5 alerts and dispositions, R6 cases and STRs, ARCHITECTURE s.17). Children of a case use
-- `closure`: the engine reads the parent's casework.cases.closed_at. Config tables are retired, never deleted.
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('alert', 'alerts',          'R5', 'partition', 'created_at', 'Monthly partitions; seven years from the report date, ten when sanctions-tied (ARCHITECTURE s.17)'),
 ('alert', 'alert_index',     'R5', 'age',       'created_at', 'Identity map of the alerts; kept as long as the alerts'),
 ('alert', 'alert_events',    'R5', 'age',       'at',         'Alert trail; kept with its alert class'),
 ('alert', 'quarantine',      'R5', 'age',       'created_at', 'Payloads that were never alerted; kept with the alert class'),
 ('casework', 'cases',        'R6', 'closure',   'closed_at',  'Cases and STRs: seven years from the later of closure and report'),
 ('casework', 'case_events',  'R6', 'closure',   'at',         'Chained trail; follows the case (parent closure)'),
 ('casework', 'case_alerts',  'R6', 'closure',   'attached_at','Links; follow the case'),
 ('casework', 'case_parties', 'R6', 'closure',   'added_at',   'Parties of the case; follow the case'),
 ('casework', 'party_tiers',  'R6', 'closure',   'raised_at',  'Tier raised by a case; follows the source case'),
 ('casework', 'case_families','R9', 'retirement', NULL,       'Closed list; deactivated, never deleted'),
 ('casework', 'queues',       'R9', 'retirement', NULL,       'Queue definitions; retired by active = false'),
 ('casework', 'saved_views',  'R9', 'retirement', NULL,       'Personal views; removed with the view')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

### 1.7 Index and constraint summary

| Table | Keys and constraints that carry the guarantees |
|-|-|
| `alert.alert_index` | PK `fingerprint` (one alert per detection identity); FK to `scn.detection_keys`; composite FK to `alert.alerts (id, created_at)` |
| `alert.alerts` | PK `(id, created_at)`; state CHECK and trigger `alerts_guard`; partitions by month through `alert.create_partition` |
| `alert.quarantine` | partial UNIQUE `(fingerprint, reason) WHERE status = 'open'`: a replayed batch does not duplicate a quarantine row |
| `casework.cases` | partial UNIQUE `(party_id, family) WHERE status IN ('open','in_review')`: one open case per party and family; `second_worker_id <> first_worker_id` (M-19) |
| `casework.case_events` | PK `(case_id, seq)`; trigger `case_events_chain_guard` (prev_hash of seq - 1); append-only trigger |
| `casework.case_alerts` | partial UNIQUE `alert_id` (scenario) and `screening_alert_id` (screening): one case per alert |
| `casework.party_tiers` | PK `party_id`; trigger `party_tiers_guard` (raised, never lowered) |

---

## 2. DDD Implementation (Rust)

**Crate placement** (phase 00 workspace): pure logic in `domain::alerting` (validation, fingerprint check, scoring, priority,
clocks as data) and `domain::casework` (case attach policy, merge plan, chain hash and verification, claim and WIP rules), both
without I/O. Ports and services in the shared application crate used by `api` and `worker` (`services::alerting`,
`services::casework`, `services::queues`, `services::counts`). Repositories in `infra::pg::alerting` and `infra::pg::casework`.
`api` mounts the routers; `worker` registers the job kinds, the timer handlers and the two outbox topic handlers. The
`Priority` enum (P0 highest, `Ord`) is the one of phase 14 (`domain::screening`), the same order phase 09 uses for the
authority matrix; it is not redefined.

**Shared value types**

```rust
// domain::alerting
pub enum AlertState { Queued, Claimed, Triaged, Duplicate, NoLongerMatches }        // alert.alerts.state
pub enum SourceKind { Scenario, Observation, Model, Manual }                          // screening stays in phase 14
pub enum AmendmentReason { MeasuresChanged, LateRowsAdded, NoLongerMatches }          // phase 16 port; unchanged
pub enum QuarantineReason {
    UnknownParty, UnknownScenarioVersion, MissingVersions, InconsistentPayload,
    UnknownCaseFamily, UnknownReasonCode, FingerprintMismatch, CoverageMissing,
}                                                                                     // alert.quarantine.reason
pub enum QuarantineStatus { Open, Released, Rejected }

// domain::casework
pub enum CaseStatus { Open, InReview, Closed, Merged }                                // casework.cases.status
pub enum CaseTier { Normal, Restricted, Suspicion }                                   // Ord: Normal < Restricted < Suspicion
pub enum ActorKind { User, System, Job }
pub struct CaseEvent { pub case_id: Uuid, pub seq: i32, pub at: DateTime<Utc>, pub actor: Actor, pub kind: CaseEventKind,
                       pub payload: serde_json::Value, pub prev_hash: [u8; 32], pub hash: [u8; 32] }
pub struct Actor { pub id: Option<Uuid>, pub kind: ActorKind }
```

**Ports** (domain traits; implemented in `services` and `infra`):

```rust
/// Phase 16 port, implemented here. The EmitOutcome contract is unchanged.
#[async_trait]
pub trait AlertSink: Send + Sync {
    async fn emit(&self, tx: &mut Tx, d: &DetectionPayload) -> Result<EmitOutcome, AlertSinkError>;
}
pub enum EmitOutcome { New, Duplicate, Amended(AmendmentReason) }                    // phase 16, unchanged

pub enum AlertSinkError {
    /// The payload cannot form an alert. emit() writes NOTHING for it (no detection key, no alert, no quarantine row:
    /// the transaction of the run must not roll back a quarantine). The caller records it with
    /// AlertQuarantineService::record_standalone in its own transaction (section 4.5, extension of the phase 16 runner).
    Quarantined(QuarantineRequest),
    Storage(StorageError),        // the snapshot could not be sealed; the run retries the bucket
    Partition(String),            // the month partition is missing; the writer calls alert.create_partition once and retries
    Db(sqlx::Error),
}
pub struct QuarantineRequest { pub payload: DetectionPayload, pub reason: QuarantineReason, pub detail: serde_json::Value }

/// Phase 13 port (RatingSource::levels) and phase 03 port (ConflictCheck) are reused as they are named there.
/// Phase 16 port DetectionKeyRepository is reused for scn.detection_keys (insert_or_get, digest updates).
#[async_trait]
pub trait ReferenceReader: Send + Sync {                       // existence checks of section 3.1 step 2
    async fn party_exists(&self, conn: &mut PgConnection, party_id: Uuid) -> Result<bool, RepoError>;
    async fn scenario_version(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<VersionRef>, RepoError>;
    async fn reason_code_active(&self, conn: &mut PgConnection, code: &str) -> Result<bool, RepoError>;   // family detection
    async fn tx_rows(&self, conn: &mut PgConnection, ids: &[Uuid], fields: &[String]) -> Result<Vec<TxRow>, RepoError>;     // canon.txn, for the snapshot
}
```

**Domain entities** (`Debug, Serialize, Deserialize, FromRow, ToSchema` where they cross the API):

- `Alert` (`alert.alerts`, plus `id_created_at` from the index): the columns of migration 0059. Score components are a
  `ScoreComponents` object: `{ typology, size, customer_risk, network, history, data_confidence, counterparty_corridor }`, each
  a number in [0, 1] or `null` with `wired: false` (the phase 15 convention for a factor that is not connected yet).
- `AlertEvent` (`alert.alert_events`), `QuarantineRow` (`alert.quarantine`; the payload is returned to A2 and M only).
- `Case` (`casework.cases`): the columns of migration 0060. `CaseAlert`, `CaseParty`, `CaseEventRow` (with `verified: bool`
  computed by the verifier), `PartyTier`, `CaseFamily`.
- `Queue` (`casework.queues`), `SavedView` (`casework.saved_views`), `QueueCounts { queue_id, items, claimed_by_me, view_counts }`.

**Create and update DTOs** (`Deserialize, ToSchema`; update fields `Option<T>`; every mutation carries `If-Match` where the row is
versioned):

- `TriageAlert { action: "accept" | "mark_duplicate" | "raise_class", duplicate_of?: Uuid, to_class?: Priority, reason_code?: String }`
  (`raise_class` may only raise; a lower class is refused, section 3.4).
- `AssignAlert { to: { user_id }, reason_code?: String }` (a user only, section 3.8); `AssignCase { to: { user_id } | { pool_id }, reason_code?: String }`.
- `CreateSavedView { name, filter, shared }`; `BulkRequest { ids: [Uuid] (max casework.bulk_cap), action: "assign" | "mark_duplicate", to?, duplicate_of?, dry_run: bool }`.
- `ResolveQuarantine { resolution_code?: String }` (reject requires `resolution_code`, a `CLS_` code).
- `MergeCase { target_case_id: Uuid, reason_code: String }`.
- `QueueItemsQuery { view?: Uuid, family?: String[], min_priority?: Priority, state?: AlertState[], assignee?: "me" | "unassigned", cursor, limit (max 100) }`
  (the filter keys are a closed allow-list; any other key is 422 `E_VALIDATION`).

**Repository traits (domain, selected methods):**

```rust
#[async_trait]
pub trait AlertRepository: Send + Sync {
    async fn resolve_id(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<(Uuid, DateTime<Utc>)>, RepoError>; // via alert_index
    async fn insert(&self, tx: &mut Tx, a: &NewAlert) -> Result<(), RepoError>;                 // also the alert_index row
    async fn lock(&self, tx: &mut Tx, id: Uuid, created_at: DateTime<Utc>) -> Result<Alert, RepoError>;  // FOR UPDATE, version
    async fn update(&self, tx: &mut Tx, a: &Alert, expected: i32) -> Result<Alert, RepoError>;  // version + 1, CAS
    async fn queue_page(&self, conn: &mut PgConnection, f: &QueueFilter, principal: &Principal, page: Keyset) -> Result<Page<QueueRow>, RepoError>; // no total
    async fn claim_next(&self, tx: &mut Tx, f: &QueueFilter, by: Uuid) -> Result<Option<Alert>, RepoError>;     // FOR UPDATE SKIP LOCKED
    async fn count_wip(&self, conn: &mut PgConnection, user: Uuid) -> Result<i64, RepoError>;
}
#[async_trait]
pub trait CaseRepository: Send + Sync {
    async fn open_for_party_family(&self, tx: &mut Tx, party: Uuid, family: &str) -> Result<Option<Case>, RepoError>; // FOR UPDATE
    async fn closed_sealed_latest(&self, conn: &mut PgConnection, party: Uuid, family: &str) -> Result<Option<Case>, RepoError>;
    async fn insert(&self, tx: &mut Tx, c: &NewCase) -> Result<(), RepoError>;
    async fn lock(&self, tx: &mut Tx, id: Uuid) -> Result<Case, RepoError>;                    // FOR UPDATE: the chain lock
    async fn update_projection(&self, tx: &mut Tx, c: &Case, expected: i32) -> Result<Case, RepoError>;
    async fn append_event(&self, tx: &mut Tx, case: &mut Case, e: NewCaseEvent) -> Result<CaseEvent, RepoError>; // seq, chain, head
    async fn events(&self, conn: &mut PgConnection, case: Uuid, page: Keyset) -> Result<Page<CaseEvent>, RepoError>;
}
```

**Route table.** Scopes from API.md section 2 (`queue.work` admits A1, A2, M; `audit.read` admits IA, SEC; `self` admits every
role). Rows marked `API 3.12` exist in API.md and are kept as written; the other rows are added by this phase (API.md 3.12 may
take them, as the lead allowed). Each route names its Cedar action in the description.

| Method | Path | Scope | Description |
|-|-|-|-|
| GET | `/api/v1/work` | `queue.work` | W1 composition: my claimed alerts, my open cases, items due within the business day, screening items I hold (`work.read`); no totals across people |
| GET | `/api/v1/queues` | `queue.work` | Queues visible to me with counts after the visibility filter (API 3.12; `queue.read`) |
| GET | `/api/v1/queues/{id}/items` | `queue.work` | Items by class, then score, then age; `view` parameter; cursor; no total (API 3.12; `queue.read`) |
| GET | `/api/v1/queues/{id}/views` | `queue.work` | Saved views of the queue visible to me, with counts (`queue.read`) |
| POST | `/api/v1/queues/{id}/views` | `queue.work` | Save a view (personal, or shared with the pool); `Idempotency-Key` (`queue.view.save`) |
| DELETE | `/api/v1/queues/{id}/views/{view_id}` | `queue.work` | Delete my view; `If-Match` (`queue.view.save`) |
| POST | `/api/v1/queues/{id}/next` | `queue.work` | "Next": claim the first unclaimed item of the view in queue order; 204 when none; `Idempotency-Key` (`queue.next`, `alert.claim`) |
| POST | `/api/v1/queues/{id}/bulk` | `queue.work` | Bulk assign or mark duplicate; `dry_run` preview; cap `casework.bulk_cap`; one event per item; `Idempotency-Key` (`queue.bulk`) |
| GET | `/api/v1/alerts/{id}` | `queue.work` | Alert, evidence-first: at most three headline reasons, coverage and `data_as_of`, versions, score components (API 3.12; `alert.read`). Suspicion tier: step-up `record_open` |
| GET | `/api/v1/alerts/{id}/snapshot` | `queue.work` | The sealed evidence snapshot (vault open; access event written) (`alert.snapshot.read`) |
| POST | `/api/v1/alerts/{id}/claim` | `queue.work` | Claim; 409 `E_CLAIMED` on a race; 409 `E_WIP_LIMIT` at the limit; `If-Match` (API 3.12; `alert.claim`) |
| POST | `/api/v1/alerts/{id}/unclaim` | `queue.work` | Return to the queue; `If-Match` (API 3.12; `alert.claim`) |
| POST | `/api/v1/alerts/{id}/triage` | `queue.work` | Accept into case work, mark duplicate of an alert of the same case, or raise the class; `If-Match` (`alert.triage`) |
| POST | `/api/v1/alerts/{id}/assign` | `queue.work` | Assign to a user (a pool target is 422); conflict check; `If-Match` (`alert.assign`) |
| GET | `/api/v1/alert-quarantine` | `queue.work` | Quarantined payloads with reason and age; the payload only to A2 and M (`alert.quarantine.read`) |
| POST | `/api/v1/alert-quarantine/{id}/release` | `queue.work` | Re-validate and queue the release into the sink (202; the worker job emits); step-up `decision`; `If-Match`; `Idempotency-Key` (`alert.quarantine.resolve`) |
| POST | `/api/v1/alert-quarantine/{id}/reject` | `queue.work` | Reject with a `CLS_` closure code; `If-Match` (`alert.quarantine.resolve`) |
| GET | `/api/v1/cases` | `queue.work` | Case search, post-filter, cursor, no totals (API 3.12; `case.read`) |
| GET | `/api/v1/cases/{id}` | `queue.work` | Case header: class, tier, status projection, alerts merged, clocks. Suspicion tier: step-up `record_open` (API 3.12; `case.read`) |
| GET | `/api/v1/cases/{id}/timeline` | `queue.work` | Events with a verification flag per event (API 3.12; `case.read`) |
| GET | `/api/v1/cases/{id}/chain` | `audit.read` | Chain verification of one case: result and first broken seq only, no event content (`case.chain.read`) |
| POST | `/api/v1/cases/{id}/assign` | `queue.work` | Assign L1 or L2 within the pool rules; conflict check; 403 `E_SELF_APPROVAL` for L2 equal to L1; `If-Match` (API 3.12; `case.assign`) |
| POST | `/api/v1/cases/{id}/merge` | `queue.work` | Merge into another open case of the same family; both trails kept; `If-Match` (API 3.12; `case.merge`) |
| GET | `/api/v1/events/counts` | `self` | SSE stream `queue.counts` for the caller's own channel (text/event-stream) (`counts.read`) |
| POST | `/api/v1/_dev/detections` | dev only | Enqueues one fixture `DetectionPayload` as the job `dev_detection_emit`, which calls `AlertIntakeService::emit` in the worker; compiled out of release builds (phase 03 `_dev` rule) |

Counted: 24 product routes (10 from API 3.12, 14 added) and the dev route. The mutating routes take `Idempotency-Key` (phase 00
middleware) and a versioned `If-Match` where a row version exists.

```rust
pub fn router() -> Router<AppState>   // in `api::work`, `api::queues`, `api::alerts`, `api::alert_quarantine`, `api::cases`, `api::events`
```

**OpenAPI.** Tags `alerts`, `cases`, `queues`, `work`, `events`. Schemas: `AlertView`, `AlertEvidence`, `ScoreComponents`,
`QueueItem`, `QueueCounts`, `SavedView`, `CaseView`, `CaseEventView`, `CaseChainResult`, `QuarantineView`, `BulkResult`,
`WorkView`, and the DTOs above. The frontend client is regenerated with `make generate-api` in the same change.

---

## 3. Business Logic

Every mutation below writes its audit event (section 3.16) in the same transaction, and every trigger-driven change is re-read
by its handler before it acts (phase 00 rule). Numbered steps are the order of the code.

### 3.1 Alert creation from the detection payload (`AlertIntakeService`, implements `AlertSink`; M-07, L-17, S-52)

`emit(tx, d) -> Result<EmitOutcome, AlertSinkError>`. The run of phase 16 calls it once per emitted detection (never for a shadow
run). The outcome contract of phase 16 is kept exactly: `New`, `Duplicate` and `Amended(reason)`.

1. Validate (section 3.2). An invalid payload returns `Err(AlertSinkError::Quarantined(request))` and writes nothing: no detection
   key, no alert, no quarantine row. The caller records it in its own transaction (section 4.5).
2. Identity (phase 16 section 3.7): `DetectionKeyRepository::insert_or_get(tx, d.fingerprint, ...)` under the advisory lock the
   bucket fold already holds. The key is either new or existing; the measures digest decides the rest:
   - new key: continue at step 3;
   - existing key, digest equal: return `Duplicate`. No audit, no alert, no event (a replayed batch stops here);
   - existing key, digest differs and `d.matches` is true: amendment (section 3.13), return `Amended(MeasuresChanged)` or
     `Amended(LateRowsAdded)`;
   - existing key with `matches` true and `d.matches` false: `Amended(NoLongerMatches)` (section 3.13).
3. Classify and score (section 3.4): `priority_class`, `priority_rule_version`, `score`, `score_components`.
4. Store the snapshot (section 3.3, phase 02 `put_generated`) before any alert row is written. It returns `snapshot_object_id` and
   `snapshot_sha256`.
5. Open or find the case of the party and family (section 3.5), locked `FOR UPDATE`.
6. Insert the alert: `id` (UUIDv7, app side), `created_at` from the database clock, every column of migration 0059 from the payload
   (`window_from`, `window_to`, `emitted_at`, `latency_s = emitted_at - window_to` in seconds, `data_as_of`, `reason_codes`,
   `severity`, `case_family`, `template`, `measures`, `matched_txn_ids` (the first 500), `matched_count`, `links`,
   `linked_party_ids`, `degraded`, `degraded_reasons`, `coverage`). Insert `alert.alert_index (fingerprint, id, created_at)` in the
   same transaction. If the insert fails because the month has no partition, return `AlertSinkError::Partition`; the writer calls
   `alert.create_partition('alerts', y, m)` once and retries (phase 13 and 16 pattern).
7. Link: `casework.case_alerts` row (`source_kind = 'scenario'`, `alert_id`, `alert_created_at`) and, when the case is new, the
   subject row in `casework.case_parties` (section 3.5).
8. Shadow link (section 3.14): `scn.shadow_results.live_alert_id` for the same detection key.
9. Clocks (section 3.12): schedule `alert_touch` and `alert_age` through `TimerService`; store `touch_timer_id`, `age_timer_id`,
   `touch_due_at`. The timers are scheduled with the bank calendar (`BankCalendarClock`).
10. Events: `alert.recorded` (sequence 1, `alert_events`) and `case.alert_attached` (case chain, section 3.7). The case priority
    is recomputed (section 3.5 step 4).
11. Audit (in the transaction): `detection.recorded` (phase 16 kind, resource the detection key) and `alert.recorded` (resource the
    alert). Payloads carry ids, codes and counts only.
12. Outbox (in the transaction, topic handlers of section 4.2): `alert.created {alert_id, case_id}` and `case.event {case_id, seq}`.
13. Return `EmitOutcome::New`.

**Naming.** Phase 14 already names its screening-decision service `AlertService` (phase 14 section 3.6), so the sink of this phase is
`AlertIntakeService` (DR-118); roadmap and phase 16 are aligned to that name.

### 3.2 Validation and quarantine (`AlertValidator`, `AlertQuarantineService`; M-07, L-05)

`AlertValidator::check(d, refs) -> Result<(), (QuarantineReason, Value)>` runs the checks in this order; the first failure wins.
`refs` is the `ReferenceReader` port of section 2, read in the caller's transaction.

1. `fingerprint_mismatch`: recompute `sha256(canonical(fingerprint_inputs))` with the phase 02 encoder (`cv = 1`) and compare with
   `d.fingerprint`; the inputs are `{scenario_id, entity_kind, anchor, bucket}`.
2. `unknown_party`: the party of the entity (the customer of an account or the entity itself) is a current row of `canon.party`
   (`ReferenceReader::party_exists`). The quarantine row keeps `party_id` NULL.
3. `unknown_scenario_version`: `scenario_version_id` exists, belongs to `scenario_id`, and is `live` (mode `incremental`, `settled`
   or `sweep`) or `shadow` (shadow never reaches the sink, so `shadow` is refused as `inconsistent_payload`).
4. `missing_versions`: `logic_version`, `register_version`, `cbi_pack_version` are present; `mapping_version_ids` is non-empty when
   the scenario's `needs` names a mapped attribute; `reference_versions` names every reference kind the scenario reads (calendar,
   country table, rating model, segment, feature versions).
5. `inconsistent_payload`: `matched_count >= len(matched_txn_ids)`; `degraded` is true if and only if `degraded_reasons` is not empty;
   `emitted_at >= window_to`; `severity`, `mode` and `entity_kind` are in their sets; every `links` kind is one of `family_no`,
   `phone`, `address`, `signatory`, `owner`; `matched_txn_ids` has no duplicates; `linked_party_ids` does not contain the subject;
   `data_as_of` has a key for every source the scenario's `needs` names.
6. `unknown_case_family`: `case_family` is an active row of `casework.case_families`.
7. `unknown_reason_code`: every code of `reason_codes` is active in the reason-code family `detection` (`reason_code_active`;
   phase 09 section 3.4; a `DET_` code that is seeded but not yet activated is unknown).
8. `coverage_missing`: `coverage` has all of `dark_units`, `excluded_units`, `exclusion_ids`, `blind_period_ids`, `stale_sources`
   (phase 16 always sends them; an empty object is refused, so a lost coverage flag cannot make an alert look covered).

`AlertQuarantineService::record_standalone(request)` runs in the caller's own transaction:
1. `INSERT INTO alert.quarantine (...) ON CONFLICT (fingerprint, reason) WHERE status = 'open' AND fingerprint IS NOT NULL DO NOTHING`
   with `reason_detail` (codes and ids only), `payload` (the whole `DetectionPayload`, sealed at rest), `party_id` when resolved,
   `run_id`, `source_kind`.
2. When a row was inserted: audit `alert.quarantined` (resource the quarantine row); a replay that hits the conflict writes
   nothing.
3. A quarantined payload is never dropped and never alerted without a person's release (section 3.2 release below).

**Release** (`POST /alert-quarantine/{id}/release`; step-up `decision` through `StepUpService::consume`; Cedar `alert.quarantine.resolve`):
1. Lock the row `FOR UPDATE`. Status must be `open`, else 409 `E_NOT_ALLOWED_IN_STATE`. `If-Match` on the row version.
2. Re-validate with fresh references (`AlertValidator::check`). Still invalid: write nothing (a refused request is rolled back, phase 00 section 7.17; the phase 03 decision row records the attempt, and `revalidated_at` is set by the retry job and by step 4 only) and
   answer 422 `E_VALIDATION` with `errors[]` (`still_invalid`, the reason code).
3. Valid: enqueue the job `quarantine_release {quarantine_id, actor_id, stepup_id}` (coalesce key the quarantine id) in this
   transaction, audit `alert.quarantine_release_requested`, and answer 202 with the row. The emit runs in the worker because the
   snapshot is stored through phase 02 `put_generated`, which is akr-worker only.
4. The job: lock the row, re-check `open` (else no-op), run `AlertIntakeService::emit(tx, payload)` in its transaction. `New`:
   `released_alert_id` is the new alert. `Duplicate` or `Amended`: `released_alert_id` is the existing alert found through
   `alert_index`. Status `released`, `resolved_by` the requesting actor; audit `alert.quarantine_released`. Still invalid at this
   point (a reference changed in between): `revalidated_at` and `alert.quarantine_revalidated`, the row stays open.

**Reject** (`POST /alert-quarantine/{id}/reject`; Cedar `alert.quarantine.resolve`; no step-up): requires `resolution_code`, a `CLS_`
code of the closure family (phase 09 section 3.4). Status `rejected`; audit `alert.quarantine_rejected`. The payload stays for the
audit trail (never deleted).

**Retry job** `quarantine_retry` (section 4.1): re-validates open rows whose reason is `unknown_party`, `unknown_scenario_version`,
`unknown_case_family` or `unknown_reason_code`. A row that has become valid gets `reason_detail.valid_now = true` and one audit
event `alert.quarantine_revalidated`; the retry never releases a row. A person still releases it.

### 3.3 Evidence snapshot (`SnapshotService`; M-07, M-58, T-03)

1. Build the snapshot body as canonical JSON (phase 02 encoder, `cv = 1`): the payload contract of phase 16 section 3.7 (ids,
   versions, counts, measures, coverage, `data_as_of`, window, reason codes), and the transaction rows named by the scenario's
   `evidence.fields` for the matched ids (`TxRow` from `canon.txn` through `ReferenceReader::tx_rows`; the fields are those the
   scenario declares, never names or contact values).
2. `snapshot_sha256 = sha256(bytes)`.
3. `EvidenceStore::put_generated(tx, GeneratedEvidence { owner_kind: 'alert', owner_id: alert id, kind: 'snapshot', bytes,
   content_type: 'application/json', classification: 'personal', lang: 'und', retention_class: 'R5' }, actor)`: the phase 02
   method for generated objects (render output, snapshots, certificates, packages; phase 02 section 3). It scans inline, writes the
   object to `akr-evidence` under the phase 02 object lock and inserts the evidence row as clean in `tx` (`evidence.stored`). It runs
   in akr-worker only, which is why every path into the sink runs in the worker (section 4.1). `Infected` (impossible for canonical
   JSON, still handled) maps to `AlertSinkError::Storage` and the bucket is retried.
4. Return `(object_id, sha256)`. An object written before a rolled-back transaction has no committed row: it is unreadable by
   construction and counted by phase 02 `akr_evidence_orphans_total`; the phase 23 reconciliation lists it.
5. Amendments write a new snapshot object (section 3.13); the earlier object stays, its id is kept in the amendment event.
   No change to phase 02 is needed.

### 3.4 Priority class and score (`PriorityClassifier`, `AlertScorer`; M-03, M-13, M-16)

**Priority is the sort key and is decided before the score (M-03).** `PriorityClassifier::classify(input, rules) -> ClassDecision
{ class, rule_id }` is pure.

Inputs (`ClassInput`): `family_tf` (the case family's `tf_class`), `severity` (the scenario output), `risk_level` (phase 13
`RatingSource::levels` for the subject, 1 to 5, or absent), `screening_class` (see the rule below), `authority_order` and
`stop_until_cleared` (absent until phases 19 and 20; read as false, and reported as `not wired`).

The rule set is the governed setting `casework.priority_rules` (section 1.6): an ordered list per class, `P0` first; a class
matches when any of its conditions holds; `P3` is the default. The first matching class wins. A change to the rules is a change
request through the phase 04 `setting` target (section 3.19); a loosening is material. The rules never read the score.

`screening_class` has one extra rule (DR-124): it is read only from screening alerts already linked to the case in
`casework.case_alerts` (`source_kind = 'screening'`). A scenario alert never takes its class from a screening hit of the same party,
because a screening alert is suspicion tier (phase 14) and a priority shown to a normal-tier analyst would reveal it. Linking a
screening alert to a case raises the case tier to `suspicion` (section 3.5 step 5).

Priority changes: the class of an alert rises (never falls) by triage (`raise_class`), by the age timer (section 3.12), or by
amendment (section 3.13). A rule change does not re-classify open alerts: the new rules apply to alerts created after the change
(drained, not migrated; M-75). `priority_rule_version` records the rule version that classified the alert.

**Score (`AlertScorer::score(input, weights) -> (Option<Decimal>, ScoreComponents)`, pure).** Each component is a number in [0, 1] or
absent (`wired: false`):
- `typology`: from `severity` (`low` 0.25, `medium` 0.50, `high` 0.75, `critical` 1.00);
- `size`: `min(1, measure / threshold)` where `threshold` is the first parameter of the scenario named as a threshold in
  `parameter_values` and `measure` is the matching measure; absent when either is missing;
- `customer_risk`: `level / 5` from `RatingSource`; absent without a rating;
- `network`: `min(1, len(linked_party_ids) / 5)`;
- `history`: `min(1, count of the party's alerts in casework.history_window_days / 5)` (the count excludes the new alert);
- `data_confidence`: 1.0 when not degraded, otherwise `1 - 0.5 * min(1, len(degraded_reasons) / 2)`;
- `counterparty_corridor`: absent (the payload contract does not carry it; phase 16 Open doubts).

The score is the weighted mean of the wired components with the weights of `casework.score_weights`, renormalised over the wired
components, rounded to four decimals. The stored score is never written to the rating, the feed, a segment or the screening
modifier (DR-103 of phase 15, rule 1: the history component uses alert history, and F3 stays withheld in phase 15). The ML term of
M-13 is not built.

Queue order (section 3.9): class, then score descending (absent last), then `created_at` ascending. Age is shown and used for the
tie, never as a rank of people (M-17).

### 3.5 Case attachment, opening, tiers and parties (`CaseAttachPolicy`; M-08, M-12, M-65, DR-121, DR-123)

`CaseAttachPolicy::decide(open_case, latest_sealed_case, family) -> Attach(case) | OpenNew { linked_prior }`, pure, after the lock:

1. An open case (`open` or `in_review`) of the party and the family exists (`uq_cases_open_party_family` guarantees at most one):
   `Attach`.
1a. Otherwise, the party's latest case of the family was merged: follow `merged_into` to its end; when that case is open, `Attach`
   to it (the party is one of its `case_parties` since the merge). A merge therefore keeps grouping later alerts, instead of opening
   a second case beside the target.
2. Otherwise `OpenNew`. When a closed and sealed case of the same party and family exists, `linked_prior` is its id and the new
   case gets the event `case.linked_prior {prior_case_id, within_window}` where `within_window` is true when the prior closure is
   inside `casework.reopen_window_days`. The new case does not reopen the prior one. The reopen decision is a person's (phase 18
   route `POST /cases/{id}/reopen`, `case.decide`); the automatic reopen of M-08 is an Open doubt.

**Attach** (under the case lock):
1. `INSERT casework.case_alerts` (section 3.1 step 7).
2. Event `case.alert_attached {alert_id, class, reason_code: reason_codes[1]}`.
3. Priority: `max(case.priority_class, alert class)`; event `case.priority_changed {from, to}` when it rises.
4. Tier: `max(case.tier, alert tier)`, where the alert tier is `suspicion` when the scenario's `visibility` is `restricted` (DSL
   section 2: a restricted scenario produces suspicion-tier alerts). A rise is an event `case.tier_changed`.
5. Screening link (section 3.4): a screening alert linked to the case sets the tier to `suspicion`.
6. Projection update (`last_seq`, `head_hash`, `version`).

**Open** (`OpenNew`):
1. `casework.case_ref_seq` gives `case_ref = 'CS-' || year || '-' || lpad(nextval, 6)`.
2. Insert the case: `status = 'open'`, `family`, `party_id` (subject), `priority_class` = the alert class, `tier` = the maximum of
   the family tier (`casework.case_families.tier`), the scenario-visibility tier of step 4 above and the current tier of the
   subject (`canon.party.sensitivity_tier` and `casework.party_tiers`, higher of the two), `policy_pack_version` = the active
   policy pack (phase 09), `opened_at` now, and `assignee_pool_id` = the pool of the first active `alerts` queue (by `code`) whose
   filter admits the family and class (routing; none: NULL, and only M sees a suspicion case until it is assigned; audit
   `case.opened` carries `routed: false`).
3. Parties: the subject (`role = 'subject'`); each `linked_party_ids` entry as `role = 'related'` (`ON CONFLICT DO NOTHING`).
4. Tier raise: when the family tier or the scenario visibility is `suspicion`, the subject's tier is raised in
   `casework.party_tiers` (insert or raise; never lowered; `raised_by` NULL for the system) and audit `party.tier_raised`. The
   subject's own page and every caller's masked answer then follow from the read rule of section 3.10.
5. Events `case.opened {family, class, tier, linked_prior?}` and then the attach steps above.
6. Group (section 3.6) is assigned here when the payload links match `casework.group_link_kinds` to another open case of the same family.

**Screening link** (port `ScreeningCaseLinker`, defined and implemented here; called by phase 14, an extension written into phase 14 in
gap pass 1): `link_screening(tx, screening_alert_id, actor) -> CaseId`. Phase 14's decision service calls it in its own transaction when
a screening alert of `subject_kind = 'party'` is decided `true` or `potential`.
1. Family: freeze lists `terrorist_financing`, list kind `pep` `pep`, other list kinds `other` (the mapping is part of the release).
2. Lock or open the case of (subject party, family) exactly as for a scenario alert (steps above), so one open case per party and family holds.
3. Insert `casework.case_alerts` (`source_kind = 'screening'`); set `alert.screening_alerts.case_id` (phase 14's grant); raise the case
   tier to `suspicion` (attach step 5) and the class to at least the screening alert's class; events `case.screening_attached` and
   `case.tier_changed`; audit `case.screening_attached`.
Counterparty and payment-party subjects have no party case; they stay in phase 14 until phase 18 decides their case shape (Open doubts).

The case aggregate is the only writer of `status`, `priority_class`, `tier` and the workers (M-74). A case is never deleted, and an
alert never moves to a different party's case.

### 3.6 Merge and grouping (`CaseMerger`; M-08, L-17, M-75)

"Merge" is the only way two cases become one, and it never hides anything (L-17). `CaseMerger::plan(source, target) -> MergePlan`
is pure; `CaseMerger::execute(tx, plan, actor)` runs in one transaction.

Validation, after locking both cases `FOR UPDATE` in ascending `id` order (no deadlock between two merges):
1. Source and target are different cases, both `open` or `in_review`, neither `merged`; neither is sealed (`E_CASE_SEALED` 409).
2. Same `family` (else 422 `E_VALIDATION` at `/target_case_id`, reason `family_differs`).
3. `target.tier >= source.tier` (else 422 `E_VALIDATION`, reason `tier_lower`). A merge never shows a suspicion-tier alert to the
   team of a lower-tier case.
4. Caller allowed by Cedar `case.merge` on both resources (a case the caller cannot see answers 404 identically to a missing one).

Execute:
1. `UPDATE casework.case_alerts SET case_id = target WHERE case_id = source` (scenario and screening rows; the rows stay).
2. `INSERT casework.case_parties` for every party of the source into the target (`ON CONFLICT DO NOTHING`; roles kept).
3. Target: `priority_class = max(target, source)`, `tier = max(target, source)`; the target's workers stay; the source's assignee is
   cleared.
4. Source: `status = 'merged'`, `merged_into = target`.
5. Events (both chains, section 3.7): `case.merged_out {target_case_id, alert_count}` on the source and `case.merged_in {source_case_id,
   alert_count, alert_ids (at most 500)}` on the target. This pair is the ledger of merges (M-08).
6. Suppression ledger (L-17): the counts of alerts that now sit in a case with other alerts of the same party are kept in the target's
   `case.merged_in` payload; the productive rate is phase 18 (outcomes inherited from the case decision, M-09).
7. Audit `case.merged {source_case_id, target_case_id, alert_count}`; `version + 1` on both cases.
8. Outbox `case.event` for both cases.

**Grouping** (`group_id`; related-party groups from evidenced links only): at attach or open, when the payload's `links` contain a
kind in `casework.group_link_kinds` that points to a party with another open case of the same family, both cases receive the same
`group_id` (the existing one, or a new UUID). A group never merges and never changes a decision; the event `case.group_set
{group_id, other_case_id}` is written to both. A dry run of a grouping policy change is an Open doubt (M-08 dry run).

### 3.7 Case events and the hash chain (`CaseChain`; DR-122; phase 02 `case_event` stream)

- Each case has its own chain. `hash = sha256(canonical({actor_id, actor_kind, at, case_id, kind, payload, prev_hash, seq}))` with
  the phase 02 encoder (`cv = 1`, sorted keys, fixed key set, bytes as lower-case hex, `at` in RFC 3339 with microseconds).
  `prev_hash` of `seq = 1` is 32 zero bytes; otherwise the `hash` of `seq - 1`.
- Append (`CaseRepository::append_event`): under the case row lock (`lock(tx, id)`), `seq = last_seq + 1`; `at = now()` from the database
  clock, and if it is not later than the previous event's `at`, `at = previous at + 1 microsecond` (the order of events is the order of
  time, as phase 02 requires of its streams). `prev_hash = head_hash`; insert; set `last_seq` and `head_hash` on the case. The
  database trigger `case_events_chain_guard` refuses a row whose `prev_hash` is not the previous hash (the second layer).
- Verify (`CaseChain::verify(events, head)`, used by `GET /cases/{id}/chain`, the job `case_chain_check` and the audit verifier of
  phase 24): recompute every hash in `seq` order; check `prev_hash` links, the `at` order, that the count equals `last_seq`, and
  that the last hash equals `head_hash`. Result: `Ok` or `Broken { first_bad_seq, reason }` with `reason` in `hash_mismatch`,
  `prev_mismatch`, `gap`, `head_mismatch`, `order_violation`.
- Anchors: phase 02's anchor job seals the `case_event` stream. Its Merkle root is taken over the heads of the cases that
  changed since the previous anchor (`case_id`, `last_seq`, `head_hash`), so an anchor says "every case event up to time T" (phase 02
  section 7+, the case_event anchor). A broken chain writes audit `case.chain_broken` and raises the alarm of phase 19 (M-53).
- Event content is never returned by the chain route: it returns the result, the number of events and the first broken `seq` only.

### 3.8 Claim, unclaim, triage, assignment, work in progress and conflicts (`AssignmentService`; M-15, M-19, M-57, M-67, DR-128)

**Claim** (`POST /alerts/{id}/claim`, and `POST /queues/{id}/next` through the same steps):
1. Resolve `id` through `alert_index` to `(id, created_at)`; a missing id and a masked id give the same 404 (section 3.10).
2. Cedar `alert.claim` on the alert (tier and scenario rules, section 3.17). A refusal is 403 `E_FORBIDDEN` for a visible record and 404
   for a masked one.
3. Lock the user's work count: `pg_advisory_xact_lock(hashtext('wip:' || user_id))`, so two claims by one person are serial.
4. Lock the alert `FOR UPDATE`. If `claimed_by` is the caller and state is `claimed`: answer 200 with the current row (idempotent). If
   claimed by someone else: 409 `E_CLAIMED`. If the state is not `queued`: 409 `E_NOT_ALLOWED_IN_STATE`.
5. `If-Match` (alert version): missing 428 `E_PRECONDITION_REQUIRED`; different 409 `E_VERSION_CONFLICT` with the current version.
6. Work in progress: `count_wip(user) = claimed alerts of the user + open cases assigned to the user`; at or above
   `casework.wip_limit` (default 10): 409 `E_WIP_LIMIT`.
7. Conflict check (M-67): the `ConflictCheck` port of phase 03 for the caller and the subject party. A declared or detected conflict
   refuses the claim with 403 `E_FORBIDDEN` (404 for a masked alert), writes audit `alert.conflict_refused` and leaves the alert queued.
   The MLRO and CISO alerts of M-67 are phase 19.
8. Update: `state = 'claimed'`, `claimed_by`, `claimed_at`, `version + 1`; `alert_events` `alert.claimed {via: 'claim' | 'next'}`; audit
   `alert.claimed`.

**Next** (`POST /queues/{id}/next`): one transaction: step 3 first (the per-person advisory lock, so a claim and a "Next" by the
same person take the locks in the same order and cannot deadlock), then the first row of the view in queue order that the caller
may see (`FOR UPDATE SKIP LOCKED`), then steps 6 to 8 above. No row: 204. Two analysts pressing "Next" get two different rows, and
no row is claimed twice (SKIP LOCKED).

**Unclaim**: the claimer, or M, returns the alert to the queue (`state = 'queued'`, `claimed_by` NULL) with `alert.unclaimed`. A
triaged or duplicate alert cannot be unclaimed.

**Triage** (`POST /alerts/{id}/triage`; the claimer or M; `If-Match`):
- `accept`: state `triaged`, `triaged_by`, `triaged_at`; event `case.alert_triaged` on the case; the alert leaves the claimed count
  (WIP) and stays in the case.
- `mark_duplicate {duplicate_of}`: `duplicate_of` must be an alert of the same case, different from this one; state `duplicate` (from
  `queued` or `claimed`); `casework.case_alerts.duplicate_of` set. The duplicate stays visible in the case (L-17); it is removed from
  the queue only.
- `raise_class {to_class}`: `to_class` above the current class (else 422 `E_VALIDATION` at `/to_class`, reason `lower_refused`); event
  `alert.reclassified {from, to, via: 'triage'}`; the case class is recomputed (rise only).
- Any other state: 409 `E_NOT_ALLOWED_IN_STATE`.

**Assign** (`POST /alerts/{id}/assign`; `POST /cases/{id}/assign`):
1. The target user holds `queue.work` through a role of the queue's kind and is a member of the queue's pool (role grant scope `pools`,
   phase 01); else 422 `E_VALIDATION` at `/to`.
2. Target WIP under the limit (step 6 of the claim); target conflict check (step 7).
3. Alerts: the target is a user only (a pool target is 422 `E_VALIDATION` at `/to`; an alert reaches a pool through the queue
   filter); the target becomes the claimer (state `claimed`); `alert.assigned {from, to}`.
4. Cases: `assignee_id` or `assignee_pool_id`. For L1 the first assignment sets `first_worker_id`. For L2 `second_worker_id` is set; when
   it equals `first_worker_id` the request is refused with 403 `E_SELF_APPROVAL` (M-19; the database CHECK holds it too). The
   four-eyes decision on the case is phase 18's.
5. `case.assigned {level, to}` on the case; `If-Match` on the case version.

**Concurrency and races** (the adversarial list of the template): two claims race: one row lock decides, the other gets 409
`E_CLAIMED` (or `E_NOT_ALLOWED_IN_STATE` after the first commit). A claim and a merge race: the merge locks the cases and the alert's
case link, so the alert either moved (the claim proceeds on the alert, its case is the target) or did not. A timer firing on a closed
case re-reads and does nothing (section 3.12).

### 3.9 Queues, saved views, "Next" and bulk actions (`QueueService`; M-14, M-15, M-17, UX-13, DR-126)

**Queue definition** (`casework.queues`): `kind` (`alerts` only in this phase), `pool_id` (the phase 01 pool; its `kind` must be
`alerts`, a service check), `filter` (closed keys: `families`, `min_priority`,
`states`), `order_key`, `active`. Queues are created and changed only through the change target `queue_definition` (section 3.19).

**Membership** is the phase 01 pool membership: a role grant whose `scope` names the pool in `pools` (`admin.role_grants.scope`). A grant
with `all_branches` alone is not pool membership. No second pool or membership table exists (DR-126). An A1, A2 or M grant reads alerts queues. Screening queues are not
defined here: phase 14 lists screening alerts at `GET /screening/alerts` for SS and M.

**List of queues** (`QueueService::queues(principal)`): the active queues whose pool the principal belongs to and whose kind the
principal's roles admit, with the counts of section 3.11. Cedar `queue.read` on each `Queue` resource.

**Items** (`QueueService::items(queue, view, principal, network, cursor)`):
1. The view's filter merged with the query's; keys outside the closed set are 422 `E_VALIDATION` (`/filter/<key>`).
2. Visibility (section 3.10) as a SQL predicate with bound parameters, built from the principal and the request's network.
3. `ORDER BY priority_class, score DESC NULLS LAST, created_at, id` (`class_score_age`) or `created_at, id` (`age`). Keyset cursor. At most
   100 rows. No total (the count is the counts endpoint's, post-filter).
4. Row: `{kind, id, priority_class, score, headline_reason, touch_due_at, state, merged_alert_count, age_h, owner_display}`; the reason
   chip is the first code of `reason_codes` (the code is shown, its thresholds never; branch roles would see the code only). `kind` is
   `alert` or `case`.

**System views** are fixed filters computed by the service and never stored: `all`, `alerts` (scenario alerts), `cases`, `mine` (claimed by or assigned to the caller), `unassigned` (no claimer, no assignee). They are the W2 tabs of section 5.

**Saved views**: a view is `{name, filter, shared}` on one queue. Personal views belong to the owner; a shared view is visible to the
pool's members. Save: `Idempotency-Key`, the filter keys closed, at most `casework.saved_views_per_user` personal views (else 422 `E_VALIDATION`
at `/name`), and `shared = true` only with `queue.view.save` and pool membership. Delete: owner only, `If-Match`. Views never contain
party names or ids.

**"Next"** is section 3.8.

**Bulk** (`POST /queues/{id}/bulk`; UX-13): `action` is `assign` or `mark_duplicate` only. No bulk closure, no bulk decision. At most
`casework.bulk_cap` ids (422 `E_VALIDATION` above). `dry_run = true` returns, per id, `allowed` and the reason it is refused (`claimed_by_other`,
`not_visible`, `p0_or_p1_duplicate`, `wip_limit`, `lower_refused`) without writing. The real run applies each item in its own savepoint and
returns per-item results; each applied item writes one event and one audit `queue.bulk_applied`. An id the caller cannot see is reported as
`not_found`, the same answer as a missing id. Marking a P0 or P1 alert as a duplicate in bulk is refused (the duplicate is still in the
case, but a person must mark it one at a time).

**Counts and the "Pause live updates" control** are section 3.11 and the frontend of section 5.

### 3.10 Visibility and masking (`VisibilityPredicate`; M-65, DR-127, phase 03 section 3.6)

For a principal `P`, a network `N` (the request's `network`) and a row (alert or case), the row is visible when every rule holds:

1. `P` holds one of `A1`, `A2` or `M` (the `queue.work` roles). Screening alerts in their own queue stay phase 14's (`screening.work`, SS
   and M); a screening alert linked to a case is read here as part of the case, under the case's rules.
2. The row's tier is resolved at read time as the highest of: `casework.effective_tier(subject)` (the higher of
   `canon.party.sensitivity_tier` and `casework.party_tiers.tier`, migration 0061), `casework.case_families.tier` of the family,
   and `suspicion` when `scn.scenarios.visibility = 'restricted'` for an alert's scenario (joined at read time, never copied).
3. Tier `normal`: no further rule.
4. Tier `restricted`: the roles of rule 1 and a purpose code of the `assigned_work` family (phase 03 `purpose_codes`).
5. Tier `suspicion`: `P` is `M`, or `P` is in the case's assignees (`first_worker_id`, `second_worker_id`, `assignee_id`), or `P` holds a grant on the
   case's `assignee_pool_id` (section 3.9 membership); `N = 'analyst'`; a purpose code is present; and the restricted-scenario rule (`P` is `M` or a
   named owner of the scenario, `scn.scenarios.owner_id`) when the alert's scenario is restricted. Phase 18 adds the case access list
   (`casework.case_acl`) as one more admitting clause; until then the case team is the assignees and M.

The predicate is generated from the Cedar permits of section 3.17 by `services::visibility::build(principal, network)` and is the only
list filter for alerts, cases, queue items, counts and bulk. A test pins the predicate to Cedar over generated principals and rows
(section 6, check 6).

**Masking of single resources.** A row that the caller may not see answers the masked 404 `E_NOT_FOUND`, the same body, headers and
latency floor as a missing id (phase 03 section 3.6; the two paths run the same work and wait the same median). A row the caller may see
but Cedar denies (for example an A1 caller on `alert.quarantine.resolve`, which only A2 and M hold) answers 403 `E_FORBIDDEN`. Suspicion-tier single reads
(`GET /alerts/{id}`, `GET /cases/{id}`) also require step-up `record_open` (`StepUpService::consume`); a caller the rules above admit gets 401
`E_STEPUP_REQUIRED` when the assertion is missing. A caller they refuse gets the masked 404 whatever header is missing, so a caller outside the team learns nothing about the row (phase 03 section 7.14, DR-411).

**Registration.** Masked kinds `alert` (tier of its subject party and scenario visibility) and `case` (tier of the case) are added to the
phase 03 masking registry `MASKED_KINDS` with the complete probe set of section 6, check 9. The phase 15 kinds (`customer`,
`kyc_review`, `override`, `pep_entry`) and the phase 14 kinds (`screening_alert`, `pair_memory`) stay as they are.

### 3.11 Counts and the SSE stream (`QueueService::counts`, `CountsHub`; DR-127, phase 00 Valkey fan-out, phase 04 polling)

**Counts** (`QueueService::counts(principal, network) -> QueueCounts`): per queue the number of visible open items (`queued` and
`claimed`), the number claimed by the caller, and per saved view the number of visible rows. For a caller who holds `screening.work` (SS, M), the same
event also carries phase 14's `AlertRepository::counts_for` (post-filter), so one stream serves both queues (phase 14 named this stream). Computed with the section 3.10 predicate, never a plain count. Cached in Valkey for
`casework.counts_cache_s` seconds per principal and network (key `counts:cache:<user>:<network>`); the cache is dropped on every
trigger for that principal. Counts are numbers only: no ids, no tiers, no families of hidden rows.

**Trigger** (worker, outbox topic handlers `alert.created` and `case.event`, section 4.2): the handler resolves the principals who may
see the affected row (the pool members of the queue for an ordinary row; for a suspicion row only the case assignees and M, from the
network `analyst`) and publishes an empty trigger on Valkey channel `counts:<user_id>` for each. The trigger carries no data.

**Stream** (`GET /events/counts`, scope `self`, `text/event-stream`):
1. On connect: send one `event: queue.counts` with the caller's counts (the snapshot; nothing is replayed, so a reconnect needs no
   `Last-Event-ID`).
2. Subscribe to `counts:<caller>`. On each trigger, debounce 2 seconds per caller, recompute `counts(caller)` and send a new
   `queue.counts` event.
3. A comment line every `casework.sse_heartbeat_s` seconds keeps the connection open through proxies.
4. The stream ends when the session ends (phase 01 revocation closes it within one heartbeat).

**Client behaviour** (section 5): the stream is the default; when it is down for 30 seconds the client polls `GET /queues` every 60
seconds (the cadence of `GET /inbox/counts`, phase 04); the "Pause live updates" control stops the stream without closing the page.

**Masking of the stream.** Each caller's stream carries only that caller's post-filter numbers, so a suspicion-tier item that the caller
may not see changes no number the caller sees. The trigger reaches only principals who may see the row, so a non-member receives no
event at all about it (the inference check of section 6, check 9).

### 3.12 Clocks (`AlertClockHandler`; M-11, M-16, M-22, M-23; phase 00 timers, phase 09 calendar)

Two timer kinds per alert, scheduled at creation through `TimerService::schedule` with `calendar_id` = the bank calendar:
- `alert_touch` (job `timer_alert_touch`): fires at `BusinessClock::add(created_at, touch_by_hours[class], BusinessUnit::Hours)`
  (`BankCalendarClock`, phase 09; `fire_at` and `calendar_id` from the `ClockResult`)
  (`casework.touch_by_hours`: P0 1, P1 4, P2 24, P3 72 working hours; starting values, Open doubts). Handler: re-read the alert; when the
  state is `queued` or `claimed` and the case is open, write `alert.touch_breached {class, due_at}` once (the handler checks the event
  does not already exist) and audit `alert.touch_breached`. Other states: no-op. The alert then shows as overdue; the alarm and the
  supervisor task are phase 19 (M-53).
- `alert_age` (job `timer_alert_age`): fires after `casework.age_raise_hours` working hours and, while the class is above P0, schedules
  the next one. Handler: re-read; when the state is `queued`, `claimed` or `triaged` and the case is open and the class is above P0,
  raise the class one step (P3 to P2, P2 to P1, P1 to P0), write `alert.reclassified {from, to, via: 'age'}`, recompute the case class
  (rise only), audit `alert.reclassified`, schedule the next `alert_age`. Never above P0; age never lowers a class (M-16).

A timer that fires on a closed or merged case re-reads the case and does nothing (phase 00 rule; the adversarial target "clocks firing
on a closed case"). Cancelling timers on a decision is an optimisation only. The handlers write nothing when they do nothing.

### 3.13 Amendments and no-longer-matches (`AlertIntakeService`, phase 16 section 3.7; M-59, M11 edge case)

- `Amended(MeasuresChanged)` or `Amended(LateRowsAdded)`: find the alert through `alert_index`; lock it; seal a new snapshot (section 3.3)
  from the amended payload; update `measures`, `matched_txn_ids`, `matched_count`, `links`, `linked_party_ids`, `degraded`,
  `degraded_reasons`, `coverage`, `snapshot_object_id`, `snapshot_sha256`, `score`, `score_components`, `amended_at`, `amendment_reason`,
  `version + 1`. The class may rise by the rules (never fall). Event `alert.amended {reason, previous_snapshot_id, snapshot_id}`; audit
  `alert.amended` and the phase 16 `detection.amended`.
- Decided cases (M11, late data): when the case already has an outcome (phase 18 column `outcome` not NULL, or a decision event), the
  decision is not rewritten. The amendment writes `case.alert_amended_after_decision {alert_id, reason}` for the decider; the case shows
  the flag (phase 18 screen W3).
- `Amended(NoLongerMatches)`: the key's `matches` is false (phase 16 wrote it). The alert moves to `no_longer_matches` (from `queued`,
  `claimed`, `triaged` or `duplicate`; the guard allows it). `amended_at`, `amendment_reason = 'NoLongerMatches'`. The alert stays in its
  case and in the queue history (never hidden). A claimed alert is unclaimed by the system with `alert.unclaimed {via: 'no_longer_matches'}`
  and the claimer sees it on the W1 screen. Event `case.alert_no_longer_matches` on the case; audit `alert.no_longer_matches`.
- Matches again: an alert in `no_longer_matches` whose detection key receives an amendment with `d.matches = true` returns to
  `queued` (the guard allows only this exit), with `alert.matches_again {reason}` and the steps of the first bullet; the touch
  clock is scheduled again.
- Amendments never change `priority_rule_version`, `reason_codes` (the headline of an amended alert keeps its code), `party_id` or
  `scenario_version_id`; the trigger refuses a change to them.

### 3.14 Shadow link (phase 16 section 3.11; S-44)

After the live alert is inserted (step 8 of section 3.1), the same transaction runs
`UPDATE scn.shadow_results SET live_alert_id = $alert_id WHERE detection_key = $fingerprint AND live_alert_id IS NULL`. The index
`idx_shadow_results_key` of phase 16 serves it; the grant `UPDATE (live_alert_id)` exists in phase 16. A shadow run never calls
`emit` and never creates an alert: the shadow rows only point at live alerts.

### 3.15 Errors (`AlertError`, `CaseError`, `thiserror`; API section 6)

| Variant | Problem code | Status | Note |
|-|-|-|-|
| `NotFound` (missing or masked) | `E_NOT_FOUND` | 404 | one body, one latency floor |
| `Forbidden` | `E_FORBIDDEN` | 403 | Cedar denied on a visible row |
| `Scope` | `E_SCOPE` | 403 | token or session lacks `queue.work` or `audit.read` |
| `StepUpRequired` | `E_STEPUP_REQUIRED` | 401 | `record_open` or `decision` |
| `Claimed` | `E_CLAIMED` | 409 | the item is held by someone else |
| `WipLimit` | `E_WIP_LIMIT` | 409 | `casework.wip_limit` reached |
| `VersionConflict` | `E_VERSION_CONFLICT` | 409 | `If-Match` differs; current version in the body |
| `NotAllowedInState` | `E_NOT_ALLOWED_IN_STATE` | 409 | for example a claim of a `duplicate` alert |
| `CaseSealed` | `E_CASE_SEALED` | 409 | a sealed case takes no alerts, merges or assignments |
| `SelfApproval` | `E_SELF_APPROVAL` | 403 | L2 equal to L1 |
| `StaleAssignee` | `E_STALE_ASSIGNEE` | 409 | the assignee lost the queue's pool between read and write |
| `Validation` | `E_VALIDATION` | 422 | `errors[]` with the pointer and the reason codes above |
| `PreconditionRequired` | `E_PRECONDITION_REQUIRED` | 428 | versioned mutation without `If-Match` |
| `IdempotencyRequired` | `E_IDEMPOTENCY_REQUIRED` | 400 | mutation without `Idempotency-Key` |

`AlertSinkError` (section 2) is not a problem response: `Quarantined` is handled inside the run, and `Storage`, `Partition` and `Db`
abort the bucket for a retry.

### 3.16 Audit events (one per mutation; payloads carry ids, codes and counts, never names or values)

| Event kind | On | Resource |
|-|-|-|
| `detection.recorded`, `detection.amended` (phase 16 kinds, now written here) | a new or amended detection key | detection key (fingerprint) |
| `alert.recorded` | a new alert | alert |
| `alert.amended` | an amendment | alert |
| `alert.no_longer_matches` | `NoLongerMatches` | alert |
| `alert.matches_again` | an amendment that restores the pattern | alert |
| `alert.quarantined` | a new quarantine row (`record_standalone`) | quarantine row |
| `alert.quarantine_revalidated` | a retry finds the row valid now | quarantine row |
| `alert.quarantine_release_requested` | release route (after step-up) | quarantine row |
| `alert.quarantine_released` | release job | quarantine row |
| `alert.quarantine_rejected` | reject | quarantine row |
| `alert.claimed`, `alert.unclaimed` | claim or next; unclaim | alert |
| `alert.triaged`, `alert.marked_duplicate` | triage accept; mark duplicate (also in bulk) | alert |
| `alert.reclassified` | triage raise or age timer | alert |
| `alert.assigned` | assignment of an alert | alert |
| `alert.touch_breached` | touch clock | alert |
| `alert.conflict_refused` | claim or assignment refused by the conflict check | alert |
| `case.opened` | a new case | case |
| `case.alert_attached` | attach | case |
| `case.linked_prior` | a new case beside a sealed prior case | case |
| `case.priority_changed`, `case.tier_changed` | rise of class or tier | case |
| `case.group_set` | grouping | case |
| `case.screening_attached` | a screening alert linked by `ScreeningCaseLinker` | case |
| `case.assigned` | L1 or L2 assignment | case |
| `case.merged` | merge | source case |
| `case.chain_broken` | a failed chain check | case |
| `party.tier_raised` | a party tier raised by casework | party id |
| `saved_view.created`, `saved_view.deleted` | view change | saved view |
| `queue.bulk_applied` | one applied bulk item | alert or case |

Events of the trails (`alert.*` in `alert.alert_events`, `case.*` in `casework.case_events`) are written in the same transaction as the
audit event. Reads of a suspicion-tier record write an access event (phase 02 `AccessLog`, reason and purpose code), not an audit event.
Queue definition changes are audited by the change request of phase 04.

### 3.17 Cedar actions and the file `90_alerts.cedar` (phase 03 bundle, range 90)

The bundle gets one file, `90_alerts.cedar` (phase 03 section 3.1; phase 18 uses 91 and later). Entities and actions are appended to
`schema.cedarschema` by this phase, the same way phase 14 appended its entities (Open doubts: the bundle hash changes with the file).

Schema additions (namespace `Akr`):
```cedar
// `assignees` is the case team resolved by the entity adapter at request time: first_worker_id, second_worker_id, assignee_id and
// the members of assignee_pool_id (section 3.10 rule 5); phase 18 adds the case access list. For an alert it is its case's team.
entity Alert = { tier: String, visibility: String, state: String, assignees: Set<User>, scenario_owners: Set<User> };
entity Case = { tier: String, status: String, assignees: Set<User> };
entity Queue = { kind: String, pool: Pool };
entity QuarantineRow = { status: String };
// actions: queue.read, queue.view.save, queue.bulk, queue.next (Queue); alert.read, alert.snapshot.read, alert.claim, alert.triage,
// alert.assign (Alert); alert.quarantine.read, alert.quarantine.resolve (QuarantineRow); case.read, case.assign, case.merge (Case);
// case.chain.read (Case); work.read, counts.read (System)
```

```cedar
// 90_alerts.cedar (phase 17). Roles are principal.roles (phase 03 form). No Role entity exists.

@id("alerts.work.a1") @role("A1")
permit (principal, action in [
        Akr::Action::"queue.read", Akr::Action::"queue.view.save", Akr::Action::"queue.bulk", Akr::Action::"queue.next",
        Akr::Action::"alert.read", Akr::Action::"alert.snapshot.read", Akr::Action::"alert.claim",
        Akr::Action::"alert.triage", Akr::Action::"alert.assign", Akr::Action::"case.read",
        Akr::Action::"case.assign", Akr::Action::"case.merge", Akr::Action::"work.read", Akr::Action::"counts.read"], resource)
when { principal.roles.contains("A1") };

@id("alerts.work.a2") @role("A2")
permit (principal, action in [
        Akr::Action::"queue.read", Akr::Action::"queue.view.save", Akr::Action::"queue.bulk", Akr::Action::"queue.next",
        Akr::Action::"alert.read", Akr::Action::"alert.snapshot.read", Akr::Action::"alert.claim",
        Akr::Action::"alert.triage", Akr::Action::"alert.assign", Akr::Action::"alert.quarantine.read",
        Akr::Action::"alert.quarantine.resolve", Akr::Action::"case.read", Akr::Action::"case.assign",
        Akr::Action::"case.merge", Akr::Action::"work.read", Akr::Action::"counts.read"], resource)
when { principal.roles.contains("A2") };

@id("alerts.work.m") @role("M")       // M holds every A2 action; M is the MLRO desk
permit (principal, action in [
        Akr::Action::"queue.read", Akr::Action::"queue.view.save", Akr::Action::"queue.bulk", Akr::Action::"queue.next",
        Akr::Action::"alert.read", Akr::Action::"alert.snapshot.read", Akr::Action::"alert.claim",
        Akr::Action::"alert.triage", Akr::Action::"alert.assign", Akr::Action::"alert.quarantine.read",
        Akr::Action::"alert.quarantine.resolve", Akr::Action::"case.read", Akr::Action::"case.assign",
        Akr::Action::"case.merge", Akr::Action::"work.read", Akr::Action::"counts.read"], resource)
when { principal.roles.contains("M") };

@id("alerts.counts.ss") @role("SS")     // the one stream also carries phase 14 screening counts (section 3.11)
permit (principal, action == Akr::Action::"counts.read", resource)
when { principal.roles.contains("SS") };

@id("alerts.chain.read.ia") @role("IA")
permit (principal, action == Akr::Action::"case.chain.read", resource)
when { principal.roles.contains("IA") };

@id("alerts.chain.read.sec") @role("SEC")
permit (principal, action == Akr::Action::"case.chain.read", resource)
when { principal.roles.contains("SEC") };

// Suspicion tier (alert resource): the named case team and the MLRO, from the analyst network, with a purpose code.
@id("alerts.forbid.suspicion.alert")
forbid (principal, action in [Akr::Action::"alert.read", Akr::Action::"alert.snapshot.read", Akr::Action::"alert.claim",
        Akr::Action::"alert.triage", Akr::Action::"alert.assign"], resource)
when { resource.tier == "suspicion" && !(principal.roles.contains("M") || resource.assignees.contains(principal)) };

@id("alerts.forbid.suspicion.network.alert")
forbid (principal, action in [Akr::Action::"alert.read", Akr::Action::"alert.snapshot.read", Akr::Action::"alert.claim",
        Akr::Action::"alert.triage", Akr::Action::"alert.assign"], resource)
when { resource.tier == "suspicion" && context.network != "analyst" };

@id("alerts.forbid.suspicion.purpose.alert")
forbid (principal, action in [Akr::Action::"alert.read", Akr::Action::"alert.snapshot.read", Akr::Action::"alert.claim",
        Akr::Action::"alert.triage", Akr::Action::"alert.assign"], resource)
when { resource.tier == "suspicion" && context.purpose == "" };

@id("alerts.forbid.restricted_scenario.alert")
forbid (principal, action in [Akr::Action::"alert.read", Akr::Action::"alert.snapshot.read", Akr::Action::"alert.claim",
        Akr::Action::"alert.triage", Akr::Action::"alert.assign"], resource)
when { resource.visibility == "restricted" && !(principal.roles.contains("M") || resource.scenario_owners.contains(principal)) };

// The suspicion, network and purpose rules for the Case resource (Cedar checks attribute access per resource type, so the case set is separate).
@id("alerts.forbid.suspicion.case")
forbid (principal, action in [Akr::Action::"case.read", Akr::Action::"case.assign", Akr::Action::"case.merge"], resource)
when { resource.tier == "suspicion" && !(principal.roles.contains("M") || resource.assignees.contains(principal)) };

@id("alerts.forbid.suspicion.network.case")
forbid (principal, action in [Akr::Action::"case.read", Akr::Action::"case.assign", Akr::Action::"case.merge"], resource)
when { resource.tier == "suspicion" && context.network != "analyst" };

@id("alerts.forbid.suspicion.purpose.case")
forbid (principal, action in [Akr::Action::"case.read", Akr::Action::"case.assign", Akr::Action::"case.merge"], resource)
when { resource.tier == "suspicion" && context.purpose == "" };
```

Notes on the policies:
- A permit is never widened by a role outside the matrix: `alerts.work.*` admits only A1, A2 and M; CH, KYC, CFG, SS (screening
  work stays in phase 14's file) and the others get no permit here. `counts.read` is also permitted to SS (policy
  `alerts.counts.ss`), and the stream route (scope `self`) answers an SS caller with the screening counts only. `queue.work` is the scope gate, Cedar is the fine gate.
- The step-up of a suspicion read is not a Cedar condition: the handler calls `StepUpService::consume` with the class `record_open`
  (phase 01 closed list). The quarantine release (action `alert.quarantine.resolve`) uses the class `decision`.
- Four-eyes at case assignment (M-19) is a service check plus the database CHECK, not a Cedar condition (Cedar does not see the target
  of an assignment).
- Test: each permit and forbid has a fixture for every role of the matrix (section 6, check 4), and the generated visibility predicate
  of section 3.10 is compared with these clauses (section 6, check 6).

### 3.18 Tipping-off review (M-65, M-68, phase 14 section 3.15 model)

For every screen, route, stream and artefact of this phase, what a person outside the case's access list can learn:

| Surface | Outside the access list, the person learns |
|-|-|
| W1 `GET /work` | own claimed alerts and cases only; nothing about another person's work or a hidden row (the query runs the predicate) |
| W2 `GET /queues/{id}/items`, "Next", bulk | only visible rows; "Next" returns the first visible row; bulk answers `not_found` for an id it cannot see, the same as a missing id |
| Queue counts `GET /queues`, the stream | numbers after the predicate; a non-member receives no trigger for a hidden row |
| `GET /alerts/{id}`, `/snapshot`, `GET /cases/{id}`, `/timeline` | the masked 404 (one body, one latency floor) |
| `GET /cases` (search) | visible rows only; no total; the `q` parameter is not accepted (400 `E_BAD_REQUEST`), so no free-text probe exists |
| `GET /cases/{id}/chain` | the result and the first broken sequence, for auditors only; no event content |
| Quarantine list | the reason and the age to A1 and A2 (both are queue staff); the payload only to A2 and M; a quarantine row of a suspicion party is masked as in 3.10 |
| Saved views and shared views | filters only (no party names or ids); a shared view changes no count for a non-member |
| Case timeline events | `case.linked_prior` and `case.merged_*` name case ids only; the detail of a suspicion case is in its masked timeline |
| Notifications and exports | none in this phase (phase 22 notifications, phase 21 reports); the outbox payloads carry ids only |
| Audit viewer (phase 24) | event kinds and ids; no tier, trigger cause or factor in the payload of a `party.tier_raised` event beyond the source case id |

Responses for a restricted scenario (`scn.scenarios.visibility = 'restricted'`) follow the same rows: the owners and M see the alert,
everyone else gets the masked answer.

### 3.19 Change targets and governed configuration (phase 04 `ChangeTargetRegistry`; DR-126, DR-124)

- **`queue_definition`** (new target, registered in `api` by `services::queues::register_targets`): `kind() = queue_definition`,
  `reason_families() = ["change"]`, `generic_create() = true`, `emergency_allowed() = false`, `inventory_kind() = None`. `prepare` validates the
  filter keys (closed set), the pool kind, and the seed of the queue code; `activate` writes `casework.queues` (INSERT or UPDATE with
  `version + 1`); `revert` restores the previous filter; `prepare` also refuses a change after which an active case family has no
  active queue that admits it (no orphaned work). `volume_effect` is `neutral` always: a queue routes alerts and never creates or
  suppresses one. Route: `cr_material` always, because a queue definition decides who sees which rows (M-14, M-75).
- **Priority rules and score weights** are governed settings (`casework.priority_rules`, `casework.score_weights`,
  `casework.touch_by_hours`). They change through the phase 04 `setting` target; the route map sends a loosening to `cr_material` and a
  tightening to `cr_minor`. The `priority_rules` change is evidenced by a replay (phase 16 `replay`) that shows the class counts before and
  after. No new target is registered for them.
- **Case families** (`casework.case_families`) are a release, not a setting (Open doubts).
- Evidence: `evidence_object` (phase 02) and `replay` (phase 16, for a `priority_rules` change) are the only evidence kinds cited; both are
  registered by their phases, so this phase registers no `EvidenceResolver` of its own.

## 4. Infrastructure Updates

### 4.1 Job kinds (Postgres `jobs`; coalesce key in brackets)

| Kind | Binary | Payload | Concurrency, attempts | Schedule or trigger |
|-|-|-|-|-|
| `timer_alert_touch` | worker | timer row (owner `alert`, kind `alert_touch`) [alert id] | 4, 3 | fired by the phase 00 timer sweep; scheduled at alert creation (section 3.1 step 9); handler section 3.12 |
| `timer_alert_age` | worker | timer row (owner `alert`, kind `alert_age`) [alert id] | 4, 3 | fired by the timer sweep; re-schedules itself while the class is above P0 (section 3.12) |
| `case_chain_check` | worker | `{}` [`case_chain_check`] | 1, 3 | `schedule.case_chain_check` (daily 02:40); checks the chains of open and in-review cases and of cases closed in the last 30 days; writes `case.chain_broken` for each break (section 3.7) |
| `quarantine_retry` | worker | `{}` [`quarantine_retry`] | 1, 3 | `schedule.quarantine_retry` (hourly at minute 15); re-validates open rows of the four retryable reasons; never releases (section 3.2) |
| `quarantine_release` | worker | `{quarantine_id, actor_id, stepup_id}` [quarantine id] | 2, 3 | enqueued by `POST /alert-quarantine/{id}/release` after the step-up; emits through the sink and sets `released` (section 3.2 step 4) |
| `dev_detection_emit` | worker | a fixture `DetectionPayload` [fingerprint] | 1, 1 | enqueued by `POST /_dev/detections`; registered only in non-release builds |

Every handler re-reads its owner first. The sink itself is not a job: it runs inside the phase 16 `scenario_run` job in the modes
`incremental`, `settled` and `sweep`; a `shadow` run never calls it. No new job prefix is used (`ingest_` is for akr-ingestd only).

### 4.2 Timers, outbox topics and the counts trigger

- **Timers** (phase 00 `TimerService`): kinds `alert_touch` and `alert_age`, owner kind `alert`, owner id the alert id,
  `calendar_id` the bank calendar of phase 09 (`BankCalendarClock`). Cancel on a triage, a duplicate or a no-longer-matches
  transition is an optimisation; the handler re-check is the guarantee.
- **Outbox topics** (phase 00 `TopicHandler` registry; the table lists `alert.created` and `case.event` as registered by this phase):

| Topic | Payload (ids only) | Handler (worker) |
|-|-|-|
| `alert.created` | `{alert_id, case_id}` | counts trigger for the principals who may see the row (section 3.11) |
| `case.event` | `{case_id, seq}` | counts trigger for the case's principals; the phase 22 notification dispatcher subscribes later (not here) |

A failing handler backs off under the phase 00 rule and stops at `outbox.max_attempts`; the trigger is idempotent, so a repeat is
harmless.

### 4.3 AppState additions

- `akr-api`: no sink (the release route and the dev route enqueue worker jobs; `put_generated` is akr-worker only), `assignment: Arc<AssignmentService>`, `queues: Arc<QueueService>`, `alert_quarantine: Arc<AlertQuarantineService>`,
  `case_chain: Arc<CaseChain>`, `counts_hub: Arc<CountsHub>` (Valkey subscriptions), `visibility: Arc<VisibilityBuilder>`, and the change
  target `QueueDefinitionTarget` in `change_targets`. `RecordingAlertSink` is removed from AppState (phase 16 Open doubt 16 is closed
  by DR-117 and DR-118).
- `akr-worker`: `alerts: Arc<dyn AlertSink>` (`AlertIntakeService`, handed to the phase 16 `ScenarioRunner` instead of `RecordingAlertSink`),
  `alert_quarantine: Arc<AlertQuarantineService>`, `alert_clock: Arc<AlertClockHandler>`, `chain_checker: Arc<CaseChainChecker>`, the
  `quarantine_release` and `dev_detection_emit` (dev builds only) handlers, the two topic handlers and the two counts publishers.

### 4.4 Network zones, metrics, readiness, OpenAPI

- **Zones**: `akr-api` subscribes to Valkey (`z4_cache`) for the counts stream; `akr-worker` publishes triggers to it. Both are already
  admitted to Valkey by phase 00 (Open doubts: confirm the worker's rule). No binary gains a route to another service.
- **Metrics**: `akr_alerts_recorded_total{family}`, `akr_alerts_amended_total{reason}`, `akr_alert_quarantine_open{reason}` (gauge),
  `akr_alert_touch_breached_total{class}`, `akr_case_chain_broken_total` (alarm when above zero), `akr_counts_streams` (gauge),
  `akr_counts_triggers_total`. No metric carries a party, a principal or a tier value.
- **Readiness**: no new check; Valkey is already in the readiness list of phase 00.
- **OpenAPI**: the paths of section 2 under tags `alerts`, `cases`, `queues`, `work`, `events`; `POST /_dev/detections` is documented only
  in dev builds. `make generate-api` regenerates the client in this change.

### 4.5 Existing entity, schema, route and code updates (extensions of earlier phases)

1. **Phase 16 runner** (section 3.6 of phase 16): the `ScenarioRunner` matches the `AlertSinkError` of section 2. `Quarantined(req)`:
   `AlertQuarantineService::record_standalone` in a new transaction, then the run continues. `Partition(_)`: `alert.create_partition` for the
   month, then one retry of the emit. `Storage(_)`: the bucket is retried by the run's own retry. `Db(_)`: the bucket aborts. The
   phase 16 file gets this sentence in its runner section (gap pass 1).
2. **Phase 16 sink**: `RecordingAlertSink` is replaced in AppState; `scn.detection_keys` is kept as the identity table (DR-117) with its
   columns and grants unchanged; the detection audit kinds move from the phase 16 sink to `AlertIntakeService`.
3. **Phase 02**: (a) the snapshot uses the existing `EvidenceStore::put_generated` (no new method); the owner kind `alert` is added to the
   evidence owner registry, and `alert.alerts` is added to the partition registry with the factory `alert.create_partition` and
   `ahead_months = audit.partition_ahead_months` (the phase 02 `partition_maintenance` job keeps months ahead). (b) Phase 02 reserved `case_event` as one chained stream; this phase chains per case (DR-122), so the `case_event`
   anchor payload carries a digest of the heads of the cases that changed since the previous anchor (section 3.7). (b) is written into the
   phase 02 file in gap pass 1.
4. **Phase 01**: the preferences rule "`default_queue_id` must be null until queues exist in phase 17" is lifted: the id must be an active
   queue visible to the caller (422 `E_VALIDATION` at `/default_queue_id` otherwise). The field stays inside the preferences object (no
   column, no foreign key).
5. **Phase 03**: the bundle gains `90_alerts.cedar` (section 3.17) and the schema gains the entities and actions of section 3.17 (appended
   to `schema.cedarschema`, as phase 14 did). The masking registry `MASKED_KINDS` gains `alert` and `case` with complete probe sets
   (section 6, check 9). The phase 03 purpose codes are used as they are (`assigned_work`, `investigation`).
6. **Phase 04**: the change target `queue_definition` is registered (section 3.19). Routes unchanged. The `setting` target carries the
   three governed settings of section 1.6.
7. **Phase 14**: `alert.screening_alerts.case_id` gains its foreign key to `casework.cases` (migration 0061). Phase 14's `counts_for`
   is called by `QueueService::counts`; the phase 14 Cedar file is unchanged.
8. **Phase 15**: no change to its tables or grants. DR-103 holds: the alert score never reaches the rating, and F3 (alert and case history)
   stays withheld. The rating read of the priority rules (`risk_level`) is the rating level only, which is not suspicion-derived.
9. **Phase 10**: no change to `canon.party` or its grants (DR-123). The ingest keeps `sensitivity_tier`; casework writes
   `casework.party_tiers` only. Every masked reader of phases 13 to 16 that reads `canon.party.sensitivity_tier` must read the higher of the
   two values; this phase ships the SQL function `casework.effective_tier(party_id) RETURNS TEXT` (STABLE, `SECURITY INVOKER`, in
   migration 0061, `GRANT EXECUTE` to akr_app) for that, and gap pass 1 points those readers at it (Open doubts 6).
10. **DATA-MODEL.md section 12**: aligned to migrations 0058 to 0061 in the same change (the reconciliation list of section 1). The
    `detection_keys` row of section 13 stays (kept, not migrated).
11. **API.md section 3.12**: the ten rows that ship here stay as written; the fourteen added rows of section 2 are added to 3.12 in the
    same change, and the rows of phases 18 and 19 are marked with their phase.

## 5. Frontend Impact

Two screens, both in the catalogue of DESIGN-SYSTEM section 8.1 (`/work` W1 DASH and `/queues/:id` W2 LIST, both P1). Their entries go
into `frontend/src/app/screens.ts` with the roles that see them (W1 and W2: A1, A2, M, the `queue.work` roles; SS keeps the phase 14
screening screens). The route-to-policy coverage test maps W1 to `work.read` and W2 to `queue.read`.

### 5.1 W1 My work (`/work`, DASH)

- Four panels: "Claimed by me" (alerts, class and touch clock), "My cases" (assigned open cases, class and tier badge), "Due within the
  business day" (touch clocks), "Screening I hold" (phase 14 items claimed by the caller; shown only to a caller who also holds
  `screening.work`, that is M). Each row opens its W2 item or W3 case.
- Counts and rows come from `GET /work` (one request, the predicate applied). Tasks from phase 04 (`GET /inbox`) sit in a fifth panel
  linked to the inbox, not duplicated.
- Empty states use the words "Nothing claimed" and "Nothing due"; no panel shows another person's numbers.
- Note: DESIGN-SYSTEM section 8.3 has no W1 specification (catalogue row only, line 689); this draft specifies it in outline (Open doubts).

### 5.2 W2 Alert and case queue (`/queues/:id`, LIST)

Specified by DESIGN-SYSTEM section 8.3, W2 (line 787 onward), with the phase 17 details:
- Columns: priority class (code and word), customer (name, Arabic first), top reason (`ReasonChip`, the code, never a threshold), age, next
  due (`Clock`), merged-alert count, owner. Sort by class, then score, then age; the order is explained in a tooltip.
- Tabs: "All", "Alerts", "Cases", "Mine", "Unassigned" (system views, section 3.9) and the queue's saved views, each with its count.
- "Next" (key `n`, button "Next"): claims the first unclaimed visible item of the view (`POST /queues/{id}/next`); 204 shows the empty state
  "No item in this view". Keyboard path and focus return are tested (section 6, check 27).
- Row selection opens the alert panel (UX-08, evidence-first): raw rows from the snapshot (`GET /alerts/{id}/snapshot`, read on demand),
  network before reasons, at most three headline reasons each beside its fact, mitigating factors. Thresholds are never shown to the
  branch roles (none of the three queue roles is a branch role).
- Bulk bar appears only when the selection is identical in view and never offers closure; "Mark duplicate" is refused on P0 and P1 in bulk
  with the reason shown.
- "Pause live updates" stops the stream; the banner "Live updates paused" stays visible; when the stream is down for 30 seconds the banner
  reads "Refreshed every minute" and the client polls.
- States: loading (skeleton rows), empty, masked-and-missing (the same "Not found or no access" text as X6), error, and offline.

### 5.3 Generated hooks (Orval names from the paths)

`useGetWork`, `useGetQueues`, `useGetQueuesIdItems`, `useGetQueuesIdViews`, `usePostQueuesIdViews`, `useDeleteQueuesIdViewsViewId`,
`usePostQueuesIdNext`, `usePostQueuesIdBulk`, `useGetAlertsId`, `useGetAlertsIdSnapshot`, `usePostAlertsIdClaim`,
`usePostAlertsIdUnclaim`, `usePostAlertsIdTriage`, `usePostAlertsIdAssign`, `useGetAlertQuarantine`,
`usePostAlertQuarantineIdRelease`, `usePostAlertQuarantineIdReject`, `useGetCases`, `useGetCasesId`, `useGetCasesIdTimeline`,
`useGetCasesIdChain`, `usePostCasesIdAssign`, `usePostCasesIdMerge`.

Hand-written (not Orval, EventSource): `useQueueCounts`, which opens `GET /events/counts`, applies each `queue.counts` event to the query
cache, falls back to polling `GET /queues` every 60 seconds after 30 seconds without a stream, and closes on "Pause live updates".

### 5.4 Browser and caching rules

- Alert and case routes use `gcTime: 0` in the TanStack cache (the phase 14 rule DR-75): nothing of a suspicion-tier row stays in the
  cache after the page closes.
- axe on W1 and W2 in Arabic and English, light and dark themes; keyboard only for claim, next, triage and bulk.
- At 390 px width the list becomes cards with the same fields, a 16 px gutter and no horizontal page scroll.
- Numerals follow the user's numeral setting (phase 01 preferences); priority words are the phase 14 words for P0 (Immediate) and P3
  (Routine); P1 and P2 words are proposed in Open doubts.


## 6. Verification

Dev users, one per role (conventions): `a1@bank.test`, `a2@bank.test`, `a2b@bank.test`, `m@bank.test`, `ss@bank.test`,
`kyc@bank.test`, `ch@bank.test`, `ia@bank.test`, `sec@bank.test`, `cfg@bank.test`, `ds@bank.test`, `mro@bank.test`, `adm@bank.test`.
Fixture detections come from the dev route `POST /api/v1/_dev/detections` (phase 16 seeded scenarios through the real run are used in
check 7). `$API` is the dev base URL; `$S(role)` is the session cookie and CSRF header of that role (phase 01 helper).

1. `make migrate` applies 0058 to 0061 cleanly, and a second run is a no-op. `\d+ alert.alerts` lists the partitions `alerts_<y>_<m>` for
   the current month and the three ahead; `SELECT alert.create_partition('alerts', 2030, 1)` returns `alerts_2030_01` the first time and the
   same name the second time; `SELECT alert.create_partition('cases', 2030, 1)` fails with `is not a partitioned alert table`.
2. `cargo check` passes (the SQL of the services is checked at compile time); `cargo clippy -- -D warnings` is clean; `cargo test` passes,
   including the unit and property tests of check 3.
3. Domain property tests (`domain::casework`, `domain::alerting`):
   (a) a random sequence of 500 operations (open, attach, merge, group, tier raise, close) keeps at most one open case per party and family, and
   every merged case has `merged_into` set and no open alerts;
   (b) `CaseChain::verify` is `Ok` after any append sequence, and `Broken` with the right `first_bad_seq` after one byte of one event's payload
   is changed;
   (c) the class of an alert never falls: over random ages, amendments and triage raises, the class sequence is monotone and never above P0
   in the wrong direction; the age timer stops at P0;
   (d) the score is the renormalised weighted mean of the wired components, with an absent component never counted as zero;
   (e) the fingerprint recomputed from `fingerprint_inputs` equals the phase 16 fixture for each of the 18 case families' scenarios.
4. Cedar: `akr-ctl policy validate` accepts the bundle with `90_alerts.cedar` and the appended schema; the route-to-policy test maps every route of
   section 2 to a Cedar action; the role matrix fixture (roles A1, A2, M, IA, SEC, CH, SS, KYC, CFG, DS, MRO, ADM) gives the expected answer for
   each action of section 3.17 (permit, 403 `E_FORBIDDEN`, 404 masked, or 403 `E_SCOPE`).
5. Curl suite `tests/curl/phase17_alerts.sh` (every row below is one request with the stated answer):
   - `GET /queues` as `a1` → 200, an array; each item has `counts.items`; as `ch` → 403 `E_SCOPE` (no `queue.work`).
   - `GET /queues/{id}/items` as `a1` with `?family=cash&min_priority=P1` → 200, rows ordered by class then score; `?q=x` → 400 `E_BAD_REQUEST`;
     a filter key outside the set (`?owner=x`) → 422 `E_VALIDATION` at `/filter/owner`.
   - `POST /alerts/{id}/claim` as `a1` with `If-Match: 1` and `Idempotency-Key: k1` → 200, `state: claimed`; the same key and body again →
     200 with `x-idempotent-replay: true`, byte-identical; the same key with another body → 422 `E_IDEMPOTENCY_REUSE`; no `If-Match` → 428;
     `If-Match: 0` → 409 `E_VERSION_CONFLICT` with the current version in the body; `a2` claiming the same alert → 409 `E_CLAIMED`.
   - `POST /alerts/{id}/triage` `{action: raise_class, to_class: P0}` as the claimer → 200 and an `alert.reclassified` event; `{to_class: P3}` → 422
     `E_VALIDATION` at `/to_class` (`lower_refused`).
   - `POST /alerts/{id}/assign` `{to: {user_id: <a2b>}}` as `a1` → 200 when `a2b` is in the pool, 422 `E_VALIDATION` at `/to` when not.
   - `POST /cases/{id}/assign` as `a1` (L1) then the same caller as L2 → 403 `E_SELF_APPROVAL`; `a2` as L2 → 200.
   - `POST /cases/{id}/merge` into a case of another family → 422 `E_VALIDATION` (`family_differs`); into a sealed case → 409 `E_CASE_SEALED`;
     into an open case of the same family with `tier >=` → 200; the source's `GET /cases/{id}` then shows `status: merged`.
   - `GET /cases/{id}/timeline` → 200 with a `verified: true` flag on every event; `GET /cases/{id}/chain` as `ia` → 200
     `{result: "ok", events, first_bad_seq: null}`; as `a1` → 403 `E_SCOPE`.
   - `POST /queues/{id}/next` as `a1` twice in parallel → two different ids; a third call with nothing visible → 204.
   - `POST /queues/{id}/bulk` `{action: mark_duplicate, ids: [...26 ids]}` → 422 `E_VALIDATION` (cap 25); `dry_run: true` returns per-id
     answers and writes nothing (the list is unchanged afterwards).
   - `GET /alert-quarantine` as `a1` → 200 without payloads; `POST /alert-quarantine/{id}/release` as `a1` → 403 `E_FORBIDDEN` (Cedar
     `alert.quarantine.resolve` is A2 and M only); as `a2` without a step-up assertion → 401 `E_STEPUP_REQUIRED`; with it → 200
     `status: released`; `POST /alert-quarantine/{id}/reject` without `resolution_code` → 422; with `CLS_OTHER_CHECKED` → 200.
   - Masking: `GET /alerts/{id}` as `a2b` for a suspicion-tier alert the caller is not assigned to → 404 `E_NOT_FOUND`, and the body is
     byte-identical to `GET /alerts/<random uuid>` as the same caller (diff empty), with the latency of the two within 5 ms median over 200
     requests each. As `a1` on a visible normal alert → 200. As `ch` on the same suspicion alert → 404 (`ch` has no `queue.work`: the scope gate
     answers first with 403 `E_SCOPE`; the check confirms the scope answer, not the masked one).
   - `GET /work` as `a1` → 200 with the four panels; as `m` → the same panels with the MLRO's rows.
   - `GET /queues/{id}/views`, `POST /queues/{id}/views` with `shared: true` as `a1` → 201; the same name again → 422 `E_VALIDATION` at `/name`;
     `DELETE /queues/{id}/views/{view_id}` without `If-Match` → 428.
6. Visibility equivalence (`api/tests/visibility.rs`): generated principals (roles, pools, network `analyst` or `other`, purpose present or absent)
   against generated rows (tier normal, restricted, suspicion; scenario visibility normal or restricted; assigned or not) → the SQL predicate of
   section 3.10 admits a row exactly when the Cedar decision for `alert.read` (or `case.read`) is allow, over the whole generated set.
7. Real-run integration (Postgres, the phase 16 seeded scenarios through `scenario_run`): the number of emitted detections equals
   `New + Duplicate + Amended + Quarantined` of the sink; a second run over the same batch creates zero alerts and zero quarantine rows;
   `SELECT count(*) FROM alert.alert_index` equals the count of `New`; `SELECT fingerprint FROM alert.alert_index GROUP BY 1 HAVING count(*) > 1`
   is empty.
8. Quarantine: a fixture batch of 1,000 payloads (800 valid, 100 duplicates of them, 100 invalid across the eight reasons) yields 800
   alerts, 100 duplicates, and 100 quarantine rows, one per reason and identity; `SELECT reason, count(*) FROM alert.quarantine GROUP BY 1`
   matches the fixture; nothing is dropped. A payload with an unknown case family is `unknown_case_family`, never an alert.
9. Masking and the inference harness (phase 03 `MASKED_KINDS`, kinds `alert` and `case`): get (suspicion and normal, for A1, A2b and a
   masked caller), list count (`/queues/{id}/items`, `/cases`), search (`q` → 400), export (none; the probe asserts no export route exists for
   the kinds), error body (identical 404 bodies), notification text (none in this phase: no outbox payload names a party), SSE (a
   non-member's `queue.counts` stream gets no event after a suspicion trigger), audit viewer (the `party.tier_raised` payload names the source
   case id only), and the latency comparison (median of 200 masked refusals against 200 misses, under 5 ms apart). A kind without the complete
   set fails the build.
10. Audit: after the curl session, `akr-ctl audit verify` passes, and the audit rows of the alert and case ids (through the phase 24 viewer `GET /audit/events` of API.md
    section 3, or until phase 24 ships, a read of `audit.audit_log` in the test database) list the kinds of section
    3.16 that the session caused (`alert.recorded`, `alert.claimed`, `alert.triaged`, `alert.reclassified`, `alert.assigned`,
    `alert.quarantined`, `alert.quarantine_released`, `alert.quarantine_rejected`, `case.opened`, `case.alert_attached`, `case.assigned`,
    `case.merged`, `saved_view.created`, `queue.bulk_applied`), each with ids and codes and no names.
11. Case chain: a test-only superuser change to one `case_events` payload is refused for `akr_app` by the trigger (`case_events` has no UPDATE
    grant); the same change made by a superuser in the test database makes `GET /cases/{id}/chain` answer `broken` with that `seq`, and the
    nightly `case_chain_check` writes `case.chain_broken`.
12. Append-only and grants: as `akr_app`, `UPDATE casework.case_events SET payload = '{}'` → permission denied; `DELETE FROM alert.alert_index` →
    denied; `UPDATE casework.cases SET party_id = gen_random_uuid()` → denied (no column grant; the trigger refuses it too); as `akr_readonly`,
    `SELECT count(*) FROM casework.cases` → permission denied.
13. Party tiers: a case of family `terrorist_financing` opened for a party raises `casework.party_tiers` to `suspicion`; a second case cannot lower
    it (`a party tier is never lowered by casework`); `canon.party.sensitivity_tier` of the same party is unchanged.
14. One open case per party and family: opening a second open case for the same (party, family) pair is refused by `uq_cases_open_party_family`;
    after the first is merged, the same pair may open again.
15. SSE: `curl -N -H "Cookie: $S(a1)" $API/api/v1/events/counts` receives `event: queue.counts` on connect; after
    `POST /_dev/detections` for a visible alert, a new `queue.counts` event arrives within 5 seconds with the increased count; the same
    fixture with a suspicion party arrives for `m` and not for `a2b`, whose stream stays silent for 10 seconds; a reconnect with `Last-Event-ID`
    gets a fresh snapshot and no replay.
16. Poll fallback: with the stream blocked, the client polls `GET /queues` every 60 seconds; the numbers equal the last stream event (check 15).
17. Clocks: a P0 alert created at 09:00 on a working day has `touch_due_at` at 10:00 (one working hour, bank calendar); with fake time, the touch
    timer fires, writes `alert.touch_breached` once, and a second fire writes nothing. A triaged alert's timer fires as a no-op. The age timer
    raises P3 to P2 after 24 working hours, never above P0; a closed case's timer does nothing (phase 00 re-check).
18. Sink contract: `grep -rn RecordingAlertSink backend/ --include=*.rs` lists only tests (the sink is replaced in AppState); `EmitOutcome` in the
    code is exactly `New`, `Duplicate`, `Amended(AmendmentReason)`.
19. Priority governance: a loosening of `casework.priority_rules` by change request is `cr_material` (phase 04 route map); an alert created after
    the change carries the new `priority_rule_version`; an alert created before keeps its class and version (drained, not migrated).
20. Shadow: a shadow run of a version writes `shadow_results` rows and calls no sink (no `alert_index` row for the shadow key); after a live alert is
    created for the same key, `live_alert_id` is set on the shadow row.
21. Screening link: `ScreeningCaseLinker::link_screening` for a party screening alert decided `potential` (test harness call, the phase 14 caller
    lands in gap pass 1) inserts `casework.case_alerts` with `source_kind = 'screening'`, sets `screening_alerts.case_id` and the case tier to `suspicion`; `INSERT INTO alert.screening_alerts ... case_id = <random>` is refused by `fk_screening_alerts_case`.
22. Default queue: `PATCH /me/preferences` with `default_queue_id` of a queue the caller cannot see → 422 `E_VALIDATION` at `/default_queue_id`; of a
    visible queue → 200 with `ETag`.
23. Amendment: a fixture amendment with `MeasuresChanged` writes a new snapshot object, keeps the old one (its id in the `alert.amended` event),
    and changes no `reason_codes`, `party_id` or `priority_rule_version` (the trigger refuses). A `NoLongerMatches` amendment keeps the alert
    visible in its case with state `no_longer_matches`. A late amendment on a case with an outcome writes `case.alert_amended_after_decision`.
24. Snapshot: `GET /alerts/{id}/snapshot` as `a1` → 200 with `sha256` equal to `snapshot_sha256` of the row; the object's `av_status` is `clean`
    and the object was stored through `put_generated` (owner kind `alert`, kind `snapshot`, R5); an access event is written with the reason of the request.
25. Retention and classification: `SELECT schema_name, table_name, retention_class FROM admin.retention_assignment WHERE schema_name IN ('alert','casework')`
    returns 12 rows; `SELECT count(*) FROM admin.column_classification WHERE schema_name IN ('alert','casework')` equals the number of columns
    of the 12 tables that are classified; `encrypted = true` for `alert.quarantine.payload`.
26. Regeneration and frontend: `make generate-api` regenerates the client; the diff contains only the hooks of section 5.3 and the new schema
    types; the W1 and W2 tests pass in Arabic and English; axe reports no violations on W1 and W2 in light and dark themes.
27. Keyboard and width: on W2, `n` claims the next item and focus moves to the alert panel; `Esc` returns focus to the row. At 390 px the list shows
    cards with no horizontal page scroll and a 16 px gutter.
28. Idempotent migrations and grants at restart: restarting `akr-api` and `akr-worker` leaves the counts stream and the timers unchanged; the
    worker's scheduled `case_chain_check` and `quarantine_retry` appear in `admin.settings` as `schedule.*` with their cron strings.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 4a, from tools/agent/gap1-run4a.md; lead-verified (tools/agent/gap1-run4a-verdict.md).

### 7.1 Hibernation gate, breach clock and open-hit clause (phase 19)
Phase 19 extends this phase and owns the extension (its section 4.5 item 4); no column of this phase changes:
- Section 3.1: a new step 4b, between steps 4 and 5, calls `HibernationGate::check(conn, party_id, scenario_id,
  measures, now)`. On `Suppress(h)`, steps 6, 8 and 11 run and the hit row (`casework.hibernation_hits`) is inserted
  after step 6; steps 5, 7, 9, 10 and 12 are skipped (no case is opened or joined, no clock runs, no case event and no
  `alert.created`). Step 9 calls `BreachService::arm(tx, "alert", id, created_at, casework.touch_by_hours[class])`. A new method
  `attach_released(tx, alert_id, alert_created_at)` runs steps 5, 7, 9, 10 and 12 for the existing alert (used by the
  `hibernation_release` job; the clocks start at the release, not at the detection).
- Sections 3.9 (queue query), 3.10 (visibility predicate) and 3.11 (counts) gain the clause `AND NOT EXISTS (SELECT 1
  FROM casework.hibernation_hits h WHERE h.alert_id = a.id AND h.alert_created_at = a.created_at AND h.released_at IS
  NULL)`.
- Section 3.12: the touch handler `timer_alert_touch` calls `BreachService::breach(tx, "alert", id, 100)` with its
  breach write; `BreachService::breach` skips an alert with an open hit.

### 7.2 Case surfaces extended by phase 18
Phase 18 owns these changes (its section 4.5 items 4, 5 and 7):
- `casework.cases` gains `escalation_level` and the `outcome` CHECK (phase 18 section 1.1).
- `AssignmentService` (section 3.8) writes the `casework.case_acl` grants of phase 18 section 3.2 in the assignment
  transaction: the assignee gets `work` (A1) or `decide` (A2), reason `assignment`; the second worker gets `decide`,
  reason `second_review`; a reassignment revokes the previous assignee's grant of these two reasons (`revoked_by` is
  the actor; a grant of another reason stays, 7.25).
- `CaseRepository` gains the methods of phase 18 section 2.
- `GET /cases/{id}` gains `escalation_level`, `outcome`, `sealed`, `open_proposal_id`, the `affordances` object and,
  for M only, the reporter row (phase 18 section 3.10). The route and its scope do not change.

### 7.3 Declared conflict raises an alarm (phase 19)
From phase 19 on, `AssignmentService` (section 3.8) raises `conflict_declared` through `AlarmService::raise` on a
declared conflict (phase 19 section 3.10). This closes Open doubts 16 for the alarm; a refused claim stays audit-only
(`alert.conflict_refused`).

### 7.4 Break-glass clause in rule 5 (phase 25)
From phase 25 on, `services::visibility::build` (section 3.10) gains one admitting clause in the team of rule 5, only
under purpose `break_glass`: `OR ($purpose = 'break_glass' AND EXISTS (SELECT 1 FROM admin.break_glass_grants g WHERE
g.case_id = c.id AND g.requester_id = $1 AND g.status = 'active' AND g.expires_at > now()))`. The Cedar entity adapter
adds the requester to `Case.assignees` under the same condition (phase 25 section 3.2, "Use" step 2). Phase 25 extends
check 6 of section 6 with the purpose-keyed case and owns the table.

### 7.5 Model rank in the queue order (phase 27)
From phase 27 on, the items query of section 3.9 orders by `priority_class, model_rank DESC NULLS LAST, score DESC
NULLS LAST, created_at, id`, where `model_rank` is the latest `live` score of `alert.model_scores` for the alert (a
`LEFT JOIN LATERAL` on `alert_id` and `alert_created_at`), under the kill condition of phase 27 section 3.4. The queue
row gains `model_band` for A1, A2 and M. `GET /alerts/{id}/model-score` (phase 27 route 16) joins the `alerts` tag and
the probe set of kind `alert` in check 9. Phase 27 owns these changes; `alert.alerts` is not altered.

Gap pass 1 (2026-10-10), run 7a (coverage of UX and W IDs, gap item (ab)), from tools/agent/gap1-run7a.md; lead-verified (tools/agent/gap1-run7a-verdict.md).

### 7.6 Shortcut layer (UX-12; DESIGN-SYSTEM 7.18)
The shell component of DESIGN-SYSTEM 7.18 (phase 08 left it to P1), first used by W1, W2 and W3:
1. Bindings match `KeyboardEvent.code`, never `key`, and show the Latin keycap, so an Arabic layout gives the same
   keys. The Next binding of section 5 (`KeyN`) is the first.
2. No binding fires while focus is in an input, textarea, select or contenteditable element.
3. `?` opens a cheat sheet of the current screen's bindings.
4. The phase 01 preference `shortcuts { enabled, modifier }` (`PATCH /me/preferences`) turns the layer off or requires
   Ctrl or Alt; the switch sits in X4. No route and no audit event are added.
5. Tests: a key in a text field does nothing; with `enabled: false` no binding fires; with `modifier: Ctrl` a bare key
   does nothing.

### 7.7 Response budgets (UX-16; ROADMAP P1 quality gate)
A load run in the release pipeline (not on every push) with 50 concurrent sessions over a synthetic fixture (SP-06;
at least 10,000 alerts across the queues): first content of `GET /queues/{id}/items` and `GET /cases/{id}` within 1 s;
`POST /queues/{id}/next` p95 under 300 ms; from phase 18 on, `GET /search` p95 under 300 ms (API.md budgets). A budget
missed fails the release job, and the figures go to the release notes.

Gap pass 1 (2026-10-10), run 7b (coverage of F, B and U IDs), from tools/agent/gap1-run7b.md; lead-verified (tools/agent/gap1-run7b-verdict.md).

### 7.8 The profile.breach topic (F-07; TR-11, DSL.md C.5; DR-401)
No phase wrote `profile.breach`, the source of trigger TR-11 (declared against actual breach) in the shipped trigger
library; phase 15 section 3.3 leaves the topic to the phase that produces it. This phase writes it and registers its
first handler, the outbox rule of DR-375.
1. Section 3.1, step 12: when the outcome is `New`, the producing scenario version has `logic.kind = profile` and one
   of its `expected[]` entries has `source = declared` (phase 16 section 3.1), the same transaction also writes outbox
   `profile.breach {party_id}`, where `party_id` is the subject party of the case of step 5. `Duplicate` and `Amended`
   outcomes write nothing, so a replayed batch writes no second row.
2. The payload carries the party id only: no alert id, scenario, measure or tier.
3. Handler (worker; phase 15 trigger table, section 3.3): the TR-11 trigger of the library, stored with the neutral
   `trigger_code` `TRG_REVIEW_REQUESTED` (phase 15 section 3.6). An alert is suspicion data, so the TR-11 id and its
   cause are never stored for a non-M reader. The handler enqueues and never re-scores in the consumer's transaction.
4. Section 4.2 gains the row `profile.breach` | `{party_id}` | the phase 15 trigger.
5. Tests: a profile scenario with a declared expectation creates one alert, one outbox row and one trigger job with
   `TRG_REVIEW_REQUESTED`; the replayed batch returns `Duplicate` and writes no second row; a profile scenario with
   peer expectations only writes none.

Adversarial pass (2026-10-10), run 2 (alerts under concurrency, replay and late data), from tools/agent/adv-run2.md; lead-verified (tools/agent/adv-run2-verdict.md).

### 7.9 Amendments take the case lock (ADV-2-03, HIGH; DR-408)
Section 3.13, every bullet: under the bucket fold's advisory lock, resolve the alert's case through
`casework.case_alerts`, lock the case row `FOR UPDATE`, then lock the alert (case, then alert: the order of attach in
section 3.5 and of the phase 18 decision). `outcome` and the decision events are read after both locks. A decision that
committed first is therefore seen, and the amendment writes `case.alert_amended_after_decision`; a decision still open
waits on the case lock. Test: a decision and an amendment of one of its alerts started together always end with either
the flag or the amendment before the decision.

### 7.10 Leaving the duplicate state clears the link (ADV-2-04, MEDIUM; DR-408)
The move `duplicate` to `no_longer_matches` (section 3.13) sets `alert.alerts.duplicate_of = NULL` in the same update,
as the CHECK `(state = 'duplicate') = (duplicate_of IS NOT NULL)` requires; the event `case.alert_no_longer_matches`
carries the earlier `duplicate_of`. `casework.case_alerts.duplicate_of` is kept, so the case history still shows the
mark. Matches again returns the alert to `queued` with `duplicate_of` NULL. Test: a duplicate alert whose detection no
longer matches moves to `no_longer_matches` and the run completes.

### 7.11 A duplicate target is a live alert (ADV-2-02, MEDIUM; DR-408)
Triage `mark_duplicate` and each bulk `mark_duplicate` item lock both alerts `FOR UPDATE` in ascending `id` order, then
refuse a target whose state is `duplicate` (422 `E_VALIDATION` at `/duplicate_of`, reason `target_is_duplicate`). Two
analysts marking A and B as duplicates of each other are then serial, and the second is refused. Test: the two marks
started together leave one alert `duplicate` and one in the queue.

### 7.12 Assignment locks and a live claim (ADV-2-05, ADV-2-12, MEDIUM; DR-408)
Assign (alert), in this order: `pg_advisory_xact_lock(hashtext('wip:' || target_user_id))` (the key of claim step 3),
then the alert `FOR UPDATE`, then `If-Match` on the alert version (the route row's `If-Match`), then the target's WIP
count and conflict check. A claim by the target and an assignment to the target are then serial and the limit holds. A
single assignment is a push (M-15): it may take over an alert another person has claimed, and `alert.assigned {from,
to}` names that person. The bulk real run takes the target's `wip:` lock once before its items, re-reads `claimed_by`
under each item's row lock and refuses `claimed_by_other` as the dry run does. Test: nine claimed alerts, an assignment
and a claim to the same analyst at once: one is 409 `E_WIP_LIMIT`.

Adversarial pass (2026-10-10), run 5 (existence masking leaks), from tools/agent/adv-run5.md; lead-verified (tools/agent/adv-run5-verdict.md).

### 7.13 Merge checks visibility before any other answer (ADV-5-10, HIGH; DR-411)
Section 3.6 validation: after the two `FOR UPDATE` locks, step 4 runs first. The source is decided by the authorize
layer on the path; the target, read from the body, is decided by `authorize` from the handler (phase 03 section 3.2) on
Cedar `case.merge`. A target the caller may not see answers the masked 404 (one body, one latency floor), and no state,
seal, family or tier check runs for it. Steps 1 to 3, and the refusals added from phase 18 on (`proposal_running`,
`already_decided`, `work_items_open`) and phase 19 on (`rfi_open`), run only when both cases are visible. Test: a merge into a
sealed suspicion case of another family, by a caller outside its team, gives the same body and headers as a merge into a
random id.

### 7.14 The bulk dry run says not_found for a hidden id (ADV-5-11, HIGH; DR-411)
Section 3.9 bulk: with `dry_run = true` an id the caller cannot see is reported `allowed: false, reason: "not_found"`, the
answer for a missing id, as the real run already does. `not_visible` leaves the list of bulk reasons, which becomes
`not_found`, `claimed_by_other`, `p0_or_p1_duplicate`, `wip_limit`, `lower_refused`; the last four are given only for
visible ids. Test: per id, the dry-run result for a suspicion alert outside the caller's access equals that of a random
id.

### 7.15 The counts trigger reaches every member of a suspicion row's team (ADV-5-08, LOW; DR-411)
Section 3.11 trigger: for a suspicion row the handler resolves the principals of section 3.10 rule 5 (M, the case
assignees, the holders of a grant on the case's `assignee_pool_id`, and from phase 18 on the active rows of
`casework.case_acl`), from the network `analyst`. The pool members and access-list members who may see the row then get
fresh counts too; a non-member still gets no trigger.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.16 Case event appends and the anchor settle point (V-02-09-a, MEDIUM; DR-413)
Section 3.7, Append: after the case row lock and before it assigns `at`, the append takes the phase 02 stream lock key
of `case_event` in share mode (`pg_advisory_xact_lock_shared`), and it takes `at` from `clock_timestamp()`, not from
the transaction start time. The anchor's settle takes the same key exclusively (phase 02 section 7+), so no event at
or before an anchor's `range_to` commits after the anchor read it. Share mode does not make appends wait for each
other, so DR-122 holds. The anchor's Merkle root is taken over the heads; no list of heads is stored in the anchor.

### 7.17 The closed-case skip of the fold is built here (V-16-12-a, LOW; DR-413)
Phase 16 block 7.11 assigns one change to this phase: `DetectionKeyRepository::open_bucket` skips a key whose alert
sits in a case with status `closed` (the join `alert.alert_index` to `alert.alerts` to `casework.case_alerts` to
`casework.cases`). It is built and tested with this phase; the test is the one of that block (a repeat four days after
the case closed opens a new case with `case.linked_prior`, section 3.5).

### 7.18 Alert quarantine repository and service methods (V-17-01-b, MEDIUM; DR-413)
Phase 11 owns the names `QuarantineService`, `QuarantineRepository` and the AppState field `quarantine` (the data quality quarantine). The service of section 3.2 is therefore `AlertQuarantineService` (AppState field `alert_quarantine`), and its repository is `AlertQuarantineRepository`.
Section 2, after `CaseRepository`: `AlertQuarantineRepository` has `insert(tx, r: &QuarantineRequest, party_id: Option<Uuid>) -> Result<bool, RepoError>` (`ON CONFLICT DO NOTHING`; false on a replay), `lock(tx, id: Uuid) -> Result<QuarantineRow, RepoError>` (`FOR UPDATE`, with the version), `update(tx, q: &QuarantineRow, expected: i32) -> Result<QuarantineRow, RepoError>` (version + 1, compare and set), `page(conn, status: Option<QuarantineStatus>, page: Keyset) -> Result<Page<QuarantineRow>, RepoError>` (the list route) and `open_retryable(conn, limit: i64) -> Result<Vec<Uuid>, RepoError>` (open rows of the four retryable reasons, oldest first). `QuarantineStatus` is the enum of section 2.
Section 3.2, service methods: `release(&self, tx: &mut Tx, principal: &Principal, id: Uuid, if_match: i32, stepup_id: Uuid) -> Result<QuarantineRow, AlertError>` (steps 1 to 3; the route consumes the step-up first), `reject(&self, tx: &mut Tx, principal: &Principal, id: Uuid, if_match: i32, req: &ResolveQuarantine) -> Result<QuarantineRow, AlertError>` and `retry(&self, pool: &PgPool) -> Result<u32, AlertError>` (the job `quarantine_retry`; it returns the number of rows found valid). Release step 4 is the handler of the job `quarantine_release` (section 4.1).

### 7.19 Reject and the retry job lock the row (V-17-01-d, V-17-01-e, MEDIUM; DR-413)
Section 3.2 Reject, before its write: lock the row `FOR UPDATE`; status must be `open`, else 409 `E_NOT_ALLOWED_IN_STATE`; `If-Match` on the row version, as release step 1 does.
Section 3.2 Retry job, for each open row: lock it `FOR UPDATE` in a transaction of its own and skip it when it is no longer `open`. Then run `AlertValidator::check`. When the row is valid, set `reason_detail.valid_now = true` and `revalidated_at = now()` and write audit `alert.quarantine_revalidated` in that transaction. A row that is still invalid is not written. A row that a person released or rejected a moment earlier is therefore never written by the retry.

### 7.20 Alert trail writes and the event number (V-17-02-a, V-17-02-b, MEDIUM; DR-413)
Section 2, `AlertRepository` gains `append_event(&self, tx: &mut Tx, alert: &Alert, e: NewAlertEvent) -> Result<AlertEvent, RepoError>`. The caller holds the alert's row lock (`lock`), or has just inserted the alert in this transaction (section 3.1).
`pub struct NewAlertEvent { pub actor: Actor, pub kind: &'static str, pub payload: serde_json::Value }`. `kind` is an `alert.<verb>` name that passes the CHECK of `alert.alert_events.kind`; the payload carries ids and codes only.
`seq` is `COALESCE(max(seq), 0) + 1` over the alert's rows of `alert.alert_events`, read under that lock; `at` is the database time. No counter column is added: every writer of an alert event locks the alert first (claim step 4, 7.9, 7.12), and `uq_alert_events_seq` refuses a writer that missed the lock.

### 7.21 CaseEventKind and NewCaseEvent (V-17-02-c, MEDIUM; DR-413)
Section 2: `pub struct CaseEventKind(pub &'static str);` holds a `case.<verb>` name that passes the CHECK of `casework.case_events.kind`. Each phase that writes case events declares its kinds as constants of this type (this phase: the kinds of sections 3.4 to 3.8 and 3.13; phases 18 and 19 add theirs). It is not the phase 02 `EventKind`: the case chain has no catalogue row, and the audit kinds of section 3.16 are a separate list.
`pub struct NewCaseEvent { pub actor: Actor, pub kind: CaseEventKind, pub payload: serde_json::Value }`. `CaseRepository::append_event` sets the case, `seq`, `at`, `prev_hash` and `hash` (section 3.7).

### 7.22 Assignment: methods, the pool test and the accepted states (V-17-03-a, V-17-03-b, V-17-03-c, MEDIUM; DR-413)
Section 3.8 Assign, the two entry points, each in the request transaction: `AssignmentService::assign_alert(&self, tx: &mut Tx, principal: &Principal, id: Uuid, if_match: i32, req: &AssignAlert) -> Result<AlertView, AlertError>` and `AssignmentService::assign_case(&self, tx: &mut Tx, principal: &Principal, id: Uuid, if_match: i32, req: &AssignCase) -> Result<CaseView, AlertError>`.
Step 1, "the queue's pool": for an alert, the target is a member of the pool of at least one active `alerts` queue whose filter admits the alert (`families`, `min_priority`, `states`). For a case, the target is a member of the case's `assignee_pool_id`, or of the pool of an active `alerts` queue whose filter admits the case's family and class (the routing test of section 3.5). A pool target of a case (`to: { pool_id }`) must be the case's own `assignee_pool_id` or the pool of such a queue; else 422 `E_VALIDATION` at `/to`. The work limit and the conflict check of step 2 are for a user target only. The bulk route tests the pool of the queue in its URL.
Step 3, alerts: the state must be `queued` or `claimed` (a push may take over another person's claim, 7.12). Any other state answers 409 `E_NOT_ALLOWED_IN_STATE`, checked under the alert lock of 7.12 before the write.

### 7.23 Bulk: the method, the item checks and one reason list (V-17-04-a, V-17-04-b, MEDIUM; V-17-04-c, LOW; DR-413)
Section 3.9 Bulk: `QueueService::bulk(&self, tx: &mut Tx, principal: &Principal, network: &str, queue_id: Uuid, req: &BulkRequest) -> Result<BulkResult, AlertError>` serves both runs. `req.dry_run = true` runs the checks below and writes nothing; `false` applies each item in its own savepoint.
Checks, in this order; the first failing check gives the item's reason:
1. Target, once for the request (`assign` only): assign step 1 of section 3.8 against the pool of the queue in the URL. A failure is 422 `E_VALIDATION` at `/to`, and no item runs. The real run then takes the target's `wip:` lock once (7.12).
2. Visibility: a hidden or missing id is `not_found`.
3. Lock the alert `FOR UPDATE` (both alerts in ascending `id` for `mark_duplicate`, 7.11); the dry run reads without the lock. A claim by a person other than the caller is `claimed_by_other`.
4. A state other than `queued` or `claimed` is `not_allowed_in_state`.
5. `mark_duplicate`: a P0 or P1 alert is `p0_or_p1_duplicate`; a target that is missing, hidden, the item itself or not an alert of the item's case is `not_same_case` (the rule of triage in section 3.8; a missing and a hidden target get the same answer); a target in state `duplicate` is `target_is_duplicate` (7.11).
6. `assign`: a target at the work limit (claim step 6) is `wip_limit`.
7. `assign`: a declared or detected conflict of the target with the subject party (claim step 7) is `conflict_refused`; the real run writes audit `alert.conflict_refused`.
Closed list of reasons, which replaces the lists of section 3.9 and of 7.14: `not_found`, `claimed_by_other`, `not_allowed_in_state`, `p0_or_p1_duplicate`, `not_same_case`, `target_is_duplicate`, `wip_limit`, `conflict_refused`. `lower_refused` leaves the list: it belongs to the single triage `raise_class`, which is not a bulk action. Every reason except `not_found` is given only for a visible id (7.14).

### 7.24 Snapshot method and TxRow (V-17-05-a, LOW; V-17-05-b, MEDIUM; DR-413)
Section 3.3: `SnapshotService::seal(&self, tx: &mut Tx, alert_id: Uuid, d: &DetectionPayload) -> Result<(Uuid, [u8; 32]), AlertSinkError>` runs steps 1 to 4 in `tx` for the alert it seals (the new alert at intake, or the existing alert on an amendment, section 3.13) and returns the object id and the sha256. The actor of step 3 is the job's system actor. A storage failure of step 3 is `AlertSinkError::Storage`.
Section 2: `pub struct TxRow { pub id: Uuid, pub booking_date: NaiveDate, pub fields: BTreeMap<String, Canon> }` (`Canon` is the phase 02 value type). `fields` holds only the `txn.*` inputs that the scenario version names in `evidence.fields`; `ReferenceReader::tx_rows` receives those names in its `fields` parameter and reads no other column of `canon.txn`. An `evidence.fields` entry of another entity is not read here.

### 7.25 Assignment writes the case_acl rows itself (V-17-07-a, LOW; DR-413)
7.2, second bullet: `AssignmentService` inserts and revokes the `casework.case_acl` rows itself, in the assignment transaction, under the `case.assign` check of section 3.17. It does not call the phase 18 `grant` or `revoke`: both need `case.access.change`, which A1 does not hold, and step 5 of `grant` is for manual grants. It applies steps 3 and 4 of `grant` in its own way. Step 3: a second worker must hold `A2` or `M`, else 422 `E_VALIDATION` at `/to` with the phase 18 sub-code `role_cannot_decide` (the `E_SELF_APPROVAL` refusal of section 3.8 step 4 is checked before this one, so a first worker named as second worker gets the 403 whatever role they hold); an assignee who holds `A2` or `M` gets `decide`, and one who holds only `A1` gets `work`. Step 4: the index `uq_case_acl_active` allows one active grant per case and principal, so when the target already holds one (a manual grant, or the `mlro_desk` grant of escalation), a grant of the same or a wider access (`read`, then `work`, then `decide`) is kept and no row is written; a narrower one is revoked and the new row is inserted in the same transaction. A reassignment revokes the previous assignee's grant only when its reason is `assignment` or `second_review`. Each row it writes or revokes gets the chain and audit event of `grant` step 6 (`case.acl_granted {principal_id, access, reason}`) or of `revoke` (`case.acl_revoked {principal_id}`).

### 7.26 Keyset cursor of the queue order (V-17-08-a, MEDIUM; DR-413)
Section 3.9 Items item 3: the keyset cursor is the signed cursor of phase 00 and carries every sort key of the active order, in order: `priority_class, score, created_at, id` for `class_score_age`, and from phase 27 on `priority_class, model_rank, score, created_at, id` (7.5). A null `score` or `model_rank` is carried as null, and the next-page test places it after every value of its key (`NULLS LAST`). A cursor made under one order is refused under another with 422 `E_VALIDATION` at `/cursor`.

### 7.27 The read behind `GET /cases/{id}` (V-24-05-b, LOW; DR-413)
The answer of the row `GET /api/v1/cases/{id}` is built by `CaseReadService::header(ctx, case_id) -> CaseView` (akr-api), under the visibility predicate of section 3.10 and with the step-up of the row. Phase 18 adds its fields to the same view (phase 18 section 4.5 item 5), and the case pack of phase 24 calls this method instead of a second query.

## Decisions

Numbered from DR-117 (the last number in `research.md` is DR-116). Each decision is one paragraph with its rationale.

**DR-117 (identity and the alert table).** `scn.detection_keys` stays the one detection identity (phase 16 section 3.7); no second fingerprint
scheme exists. `alert.alerts` is partitioned by the month of creation, so it cannot carry a global UNIQUE on the fingerprint (a partitioned
unique must include the partition key). The identity map `alert.alert_index` is a small table that is not partitioned: its primary key is
the fingerprint, it references `scn.detection_keys` and the alert `(id, created_at)`, and it makes "one alert per identity" a database fact
across months. The alternative (migrating the key table into alerts) would have replaced the fold and digest rules that phase 16 already
tests and that the shadow comparison reads; it was rejected. DATA-MODEL section 12 is aligned as listed in section 1 (no `list_version_id`,
no `model_version_id`, no `case_id` column on alerts; screening alerts stay in phase 14).

**DR-118 (the sink and its name).** `AlertIntakeService` implements the phase 16 `AlertSink` port, keeps the `EmitOutcome` contract exactly, and
replaces `RecordingAlertSink` in AppState. A payload that cannot form an alert is not written inside `emit`: `emit` returns
`AlertSinkError::Quarantined(request)` with no writes, because a quarantine row written in a transaction that the run later rolls back would
be lost, and a payload must never be lost. The caller records it in its own transaction (an extension of the phase 16 runner). The
detection audit kinds `detection.recorded` and `detection.amended` move to this service, and the alert gets its own kinds. Phase 14 already names
its screening decision service `AlertService` (phase 14 section 3.6), so the sink is `AlertIntakeService` (lead review): two structs with one
name in one workspace would be a standing source of wrong imports. The sink runs only in akr-worker, because the snapshot goes through the
phase 02 `put_generated` path; the quarantine release and the dev fixture route enqueue worker jobs.

**DR-119 (the evidence snapshot).** The snapshot is written before the alert row, as an evidence object of kind `snapshot` with owner kind
`alert` and retention class R5, through the phase 02 method for generated objects, `put_generated` (inline scan, object lock, clean row in the
caller's transaction), so the sink runs in akr-worker. Its sha256 is recorded in the alert row. An object written before a rolled-back
transaction has no committed row; phase 02 counts it as an orphan and phase 23 reconciles it (lead review: the draft's new
`seal_generated` method and its `lock_mode = 'generated'` value would have broken the phase 02 object-lock CHECK). Amendments write a new object and keep the old one, so the evidence
of a decision is never overwritten (M-07, M-58, M-59 in spirit).

**DR-120 (quarantine).** A payload that fails validation is recorded in `alert.quarantine` with a closed reason, its identity, the payload
sealed at rest and the party when it resolves; a replay produces no second open row (the partial unique index). Release is a person's act
under step-up `decision` after a fresh validation, and the released alert comes from the same sink path as any other, so a released payload
cannot bypass a rule. Reject needs a `CLS_` closure code. The retry job re-validates and never releases: a payload that becomes valid is shown,
not alerted. This meets "records that cannot form an alert go to the quarantine queue, never lost" (M-07) and the no-silent-rejection rule
(L-05) without making the quarantine a second engine.

**DR-121 (case identity and attachment).** One open case per party and family is guaranteed by a partial UNIQUE index on
`casework.cases (party_id, family)` while the status is open or in review. An alert attaches to that case at any age; when the only case is
sealed, the new case is opened beside it and carries `case.linked_prior` with a flag for the reopen window. The automatic reopen of M-08 is not
built here: reopening is a person's decision in phase 18 (`POST /cases/{id}/reopen`, `case.decide`, step-up), and a silent reopen would move a
sealed record. Related parties come from the payload's linked parties as `related` rows; a group (`group_id`) is set only from the evidenced link
kinds of a setting, and a group never merges. Merges are the only way two cases become one, and both trails stay (L-17, M-08).

**DR-122 (the case event chain).** Each case has its own hash chain, written under the case row lock so that the chain order is the time order,
with the phase 02 canonical encoding and 32 zero bytes as the genesis. A database trigger checks `prev_hash` on every insert, the second layer
under the grants; the verifier recomputes every hash. The phase 02 stream `case_event` is the anchor: its payload gains the heads of the cases that
changed since the previous anchor, so the external anchor says what every case said at time T. Alert events are not chained, because the audit
chain already covers each of them; the chain is reserved for the case, where a decision is made.

**DR-123 (party tiers raised by casework).** `canon.party.sensitivity_tier` belongs to the ingest (phase 10 writes the bi-temporal party and
grants `akr_app` only SELECT on it). An ingest version written after a tier change would reset a column casework had set, so casework does not
write there. It writes `casework.party_tiers`, which is raised and never lowered by a trigger. Masking reads the higher of the two values at
read time (phase 03 rule: masking is a read rule). This resolves phase 15's open question 1 for the write path; phase 15's read helper must take
the higher value (Open doubts 6).

**DR-124 (priority classes).** The class of an alert is decided by a governed setting, an ordered rule list `P0` to `P3` evaluated in order, through
the phase 04 `setting` target, not by score and not by hand. Score sorts within a class (M-03, M-13). Screening hits contribute to a case's class
only through screening alerts already linked to that case; a scenario alert never takes a class from a screening hit of its party, because a
screening alert is suspicion tier and a visible class would reveal it. A class rises by triage, by the age timer, or by an amendment, and never
falls; a rule change applies to alerts created after it (drained, not migrated, M-75). Age raises a class one step per the setting and never above
P0 (M-16). The P0 rule set in the default value follows M-03 (list match, terrorist or proliferation financing, authority order, stop until
cleared); the last two inputs are read as absent until phases 19 and 20.

**DR-125 (score components).** The score is a pure function of displayed components, each of which is a number or explicitly absent, with the
weights in a governed setting and renormalised over the wired components, so a missing input never reads as a low risk. The alert score is never
written to a rating, a feed, a segment or a screening modifier, and the alert history component does not reach the rating (DR-103 of phase 15,
F3 stays withheld). The ML term of M-13 and the EUR 100 versus 99 million ordering test are part of the release gate of the model work, not of
this phase.

**DR-126 (queues on the existing pools).** A queue is a row in `casework.queues` that points at a phase 01 pool (`admin.pools`, kind `alerts`;
screening queues stay phase 14's `GET /screening/alerts`) and reads membership from the role grants whose scope names the pool. A second membership table would give two
answers to "who may work this queue"; the grant is already the authority of the principal mirror. A queue's definition changes who sees which
rows, so it goes through the new change target `queue_definition` as `cr_material` always; the saved views and the fixed system views filter
inside a queue with a closed key set and never widen it. "Next" and the bulk actions are the only mass operations, and bulk excludes closure
(UX-13).

**DR-127 (visibility and counts).** The visibility predicate is generated from the Cedar permits and the tier rules, and a test pins the two to
each other over generated principals and rows; every list, count, stream and bulk answer uses the predicate, and a single-row read that the
caller may not see answers the masked 404. Counts are numbers after the predicate, per principal. The stream carries a principal's own counts and
is triggered only for principals who may see the row, so a non-member learns nothing, not even the timing of a suspicion item (phase 03 masking
and the phase 14 pattern). A suspicion read additionally needs the analyst network, a purpose code and step-up `record_open`.

**DR-128 (claims, work in progress and four-eyes at assignment).** A claim takes the row lock and a per-person advisory lock for the work count; "Next"
uses `FOR UPDATE SKIP LOCKED`, so no row is claimed twice. The work-in-progress limit (10, governed) counts claimed alerts and assigned open cases;
the conflict check of phase 03 runs at claim and at assignment, and a conflicting claim is refused with the alert left queued. L1 and L2 on one case
differ by a service check and a database CHECK (M-19); the decision four-eyes is phase 18's.

**DR-129 (clocks).** Two timers per alert, scheduled through the phase 00 `TimerService` on the bank calendar: the touch clock (the must-touch-by date,
M-16) records one breach event, and the age clock raises the class one step. Both handlers re-read the alert and the case and do nothing on a closed
or decided item (the phase 00 rule and the adversarial target "clocks firing on a closed case"). Breach actions, alarms and supervisor tasks are
phase 19's, so this phase only records what happened.

**DR-130 (screening alerts in queues and cases).** Screening alerts are read in place (in phase 14's own queue, and inside a case once linked), never copied into `alert.alerts`; phase 14 keeps
their states, clocks and decisions. A party screening alert joins a case through `casework.case_alerts` with `source_kind = 'screening'` when phase 14
decides it `true` or `potential`, through the port `ScreeningCaseLinker` that this phase defines and phase 14 calls (M-12); the foreign key of phase 14's `case_id` is added here by migration 0061, because phase 14 deferred it to this phase. Linking a
screening alert raises the case tier to suspicion, so the case is visible only to the people who may see screening alerts.

**DR-131 (migrations, partitions and the route count).** Four migrations (0058 to 0061), numbered from the first free number of the lead's range. Partitions
are created through a factory allow-listed to `alerts` (phase 13 and 16 pattern), with an explicit writer retry. Twenty-four product routes and one dev
route: the template puts 25 or more routes at XL and 15 to 25 at L; the roadmap says L and the draft keeps it, with the boundary flagged in Open
doubts 26. The fourteen added rows of API.md section 3.12 are listed in section 2 and in section 4.5.

**DR-132 (retention and classification).** Alerts and their trail follow R5 with the partition clock on `created_at`; the quarantine follows R5 with the
age clock; cases, their events, links, parties and tiers follow R6 with the closure clock, and the children read the parent's closure. Money amounts in
measures are `financial`, links and matched transaction ids are `personal` (they name customer activity), scores are `detection-logic`, and the quarantined
payload is sealed at rest. Case families, queues and saved views are configuration (R9, retired, never deleted).

**Lead corrections (review 2026-10-09).** The sink is `AlertIntakeService`, not a second `AlertService` (DR-118). The snapshot uses the existing
phase 02 `put_generated` instead of a new `seal_generated` with a `lock_mode = 'generated'` value that the phase 02 object-lock CHECK refuses;
the sink therefore runs only in akr-worker, and the quarantine release and the dev route enqueue worker jobs (`quarantine_release`,
`dev_detection_emit`). Screening queues are not redefined (phase 14 owns `GET /screening/alerts`; SS lacks `queue.work`), so `casework.queues.kind`
is `alerts` only, W1 and W2 are A1, A2 and M screens, and SS gets only `counts.read`. A party screening alert reaches a case through the new
port `ScreeningCaseLinker`, which phase 14 calls; the draft named a phase 14 escalation route that does not exist. The Cedar M permit listed
its actions as a placeholder and the IA and SEC permit carried a two-role annotation; both are written out. The suspicion team (`assignees`) now
includes the members of the case's pool, matching the visibility predicate, and a new case is routed to a pool. "Next" takes the per-person lock
before `SKIP LOCKED` (lock order). An alert is assigned to a user only. A merged party's later alerts follow `merged_into` instead of opening a
second case. An alert in `no_longer_matches` returns to `queued` when the pattern holds again. Clocks use `BusinessClock::add` (phase 09), not
an `add_working` method. The `queue_definition` target's volume effect is `neutral` (the draft used `none`, not in the phase 04 enum) and it refuses
a change that leaves an active family without a queue. `casework.effective_tier(party_id)` is added so that every masked reader takes the higher of
the ingest tier and the casework tier. Grants: `casework.queues` UPDATE is a column list; one duplicate grant removed.

## Open doubts (for gap pass 1 and the bank)

1. **Name collision.** Resolved in review: the sink is `AlertIntakeService`; phase 14 keeps `AlertService` (DR-118). Roadmap services row 17 and the
   phase 16 port comment are aligned.
2. **Phase 16 runner** must match the `AlertSinkError` variants (section 4.5 item 1). Lead review wrote the sentence into phase 16 section 3.7.
3. **Phase 02 additions.** The snapshot uses the existing `put_generated` (resolved in review). Still open for gap pass 1: the `case_event`
   anchor payload for per-case chains (DR-122; phase 02 reserved one chained stream) and the owner kind `alert` in the evidence owner registry.
4. **Automatic reopen** within `casework.reopen_window_days` (M-08): the draft links and does not reopen. Phase 18 or the bank decides whether a closure
   inside the window is reopened by the system (with a re-seal, M-61) or always by a person.
5. **Case-family seeds.** The eighteen families and their tiers are proposals: `public_funds`, `terrorist_financing` and `proliferation_financing` at
   suspicion (the DSL states the public-funds rule, S-29, and TF and PF are suspicion by nature); `pep` at normal. The bank confirms each tier.
   Families are a release, not a setting: adding one needs a migration.
6. **Masked readers of phases 13 to 16.** They read `canon.party.sensitivity_tier` only; they must read `casework.effective_tier(party_id)`, the
   higher of that and `casework.party_tiers` (DR-123). Phase 15's open question 1 is answered for the write path by this phase; the read change is
   a gap item for phases 13, 14, 15 and 16 (gap item i).
7. **Priority values and clocks.** The P0 to P2 rule set, the touch hours (P0 1, P1 4, P2 24, P3 72 working hours) and the age raise (24 working hours)
   are starting values with no regulatory source; the MLRO confirms them. M-03 and M-16 name the classes, not the hours.
8. **P1 and P2 words.** Phase 14 defines P0 (Immediate) and P3 (Routine); the words for P1 and P2 are proposals for the screen and the bank's
   terms (DESIGN-SYSTEM priority legend).
9. **Score weights and components.** The weights are starting values. They are validated on the bank's own labelled cases before any use in
   ordering beyond the class (research 21 section 9b). The counterparty and corridor component is absent (open item 10).
10. **Counterparty and corridor.** The payload contract (phase 16 section 3.7) does not carry the corridor; the component is absent until the payload
    carries it (a phase 16 payload extension, gap pass 1).
11. **Snapshot orphans.** Resolved in review: phase 02 `put_generated` already counts orphans (`akr_evidence_orphans_total`) and phase 23 reconciles
    them.
12. **Release step-up.** Releasing a quarantined payload uses step-up class `decision`. The bank's security owner may prefer `approval` or none; the
    release is an analytic act that starts work on a party.
13. **Screening queue shape.** Resolved in review: no screening queue is defined here (`GET /queues` needs `queue.work`, which SS does not hold);
    SS works in phase 14's screens, and W1 shows screening items only to M. Whether the bank wants one mixed queue for M is a product question.
14. **W1 specification.** DESIGN-SYSTEM section 8.3 has no W1 text (catalogue row only). Section 5.1 is an outline; the design owner writes the
    full W1 specification before build.
15. **Case access list.** Phase 18 adds `casework.case_acl` as a visibility clause (section 3.10). Until then, an A2 analyst sees a suspicion-tier case
    only when assigned to it, and the MLRO sees all. This is intended and tested (check 6), but it is a usability cost the bank should accept knowingly.
16. **Conflict alerts.** The MLRO and CISO alerts of M-67, the declared-conflict reassignment without visible reason, and the M-67 test of the
    conflict register are phase 19 (or 24). The claim refusal here is the first half.
17. **Dry run of grouping and merge policies** (M-08): phase 19 or 24 (a change request with a dry run on history, M-75).
18. **Bulk tag, snooze and request information** (UX-13): phases 18 and 19, once their objects exist.
19. **Flat-line detector and forced sample** (M-11 and M-16): the daily forced sample of low-score alerts is a QA sample (phase 19, M-47 to M-49); the
    flat-line detector (M-11) is phase 19 with the alarms.
20. **Leave, handover and branch routing** (M-15, M-14): time off, sign-in block with handover belong to the admin of phase 03 (leave); not built here. The
    branch-first routing flag stays fixed at `false`; its five conditions are P2 (phase 09 section 7+, DR-399).
21. **Breach actions and alarms** (M-22, M-53): the 50, 100 and 150 percent actions, the supervisor task and the alarm service are phase 19; this phase
    records the touch breach only.
22. **Chain verification route.** `GET /cases/{id}/chain` shows results only; the audit verifier of phase 24 (G6) reads the same chain. A broken chain
    alarm is phase 19 (M-53).
23. **Problem code for the stream.** When Valkey is unavailable the counts stream answers 503. No problem code in API section 6 fits that case, so the
    draft names none; the API owner chooses one from section 6 or adds it to the table (no code is invented here).
24. **Worker Valkey rule.** The worker publishes triggers to Valkey; phase 00's zone table must admit the worker to `z4_cache` for publish. Confirm.
25. **Dev fixture route.** `POST /api/v1/_dev/detections` exists only in non-release builds (phase 03 `_dev` rule). The release build's test that the
    route is absent belongs to phase 03's inventory.
26. **Size.** 24 product routes is L by the roadmap and at the boundary of XL by the template (25 or more). Lead decides; the draft keeps L (DR-131).
27. **Analyst network.** Suspicion visibility needs `authz.analyst_networks` to be installed (phase 03 fail-closed rule). The install checklist of
    phase 03 lists it; this phase depends on it.
28. **M-IDs not carried here, with target phases.** M12 requirements are: M-01 to M-31, M-37 to M-42, M-46 to M-76, T-21, T-22, L-17, UX-06, UX-08, UX-13,
    UX-64, UX-65. Implemented here (in part or whole): M-03, M-06, M-07, M-08, M-09 (partly), M-11 (partly), M-12 (queue part), M-13 (partly), M-14
    (partly), M-15 (partly), M-16 (partly), M-17, M-18, M-19 (partly), M-22 (partly), M-23, M-57, M-65, M-67 (partly), M-74 (partly), M-75 (partly),
    L-17, UX-08 (partly), UX-13 (partly). Phase 14 delivers M-24 (screening states) and the screening part of M-12. Not carried, with target:
    - Phase 18 (case decisions and investigation): M-01, M-02, M-04, M-20, M-21, M-26, M-27, M-28, M-29, M-36, M-38, M-40, M-58, M-59, M-60, M-61,
      M-68, UX-06, UX-64, UX-65; and the decision four-eyes of M-19.
    - Phase 19 (RFIs, watches, network, QA, hibernations, alarms): M-10, M-35, M-37, M-39, M-42 to M-54, M-55, M-76, T-21, T-22, M-11 (flat line),
      M-16 (forced sample), M-67 (MLRO and CISO alerts).
    - Phase 20 (operations and freezes): none of the M-IDs of M12 (M-32 to M-34 are M14's and belong to phase 20).
    - Phase 21 (reports): M-30, M-31, M-41.
    - Phase 22 (branch and notifications): M-25, M-66 (the branch routing of M-14 is P2, DR-399).
    - Phase 23 (retention): M-05 (the seven-year floor per class).
    - Phase 24 (governance, packs, assessor): M-56, M-62, M-63, M-64, M-69 (the inference suite completion), M-76 if not in 19.
    - Not yet numbered (P2 AI gateway): M-70, M-71, M-72, M-73.
    Each target is the lead's to confirm in gap pass 1.

## What This Phase Does NOT Touch

- No decisions, no notes, no evidence uploads, no proposals, no escalation and no reopen (phase 18); no case closure, no sealing, no outcome, no
  `contribution_outcome` writes (phase 18); the workflow runtime and `admin.tasks` (phase 04 and 18).
- No party page, no transaction explorer, no network and no RFIs, watches, hibernations, internal reports, authority requests or QA (phases 18
  and 19). `GET /parties/{id}` is phase 18.
- No freeze, no operations task, no CBS-facing action (phases 20 and 14); `FreezeTrigger` and `RecordingFreezeTrigger` are not touched (phase 20).
- No screening states, clocks, decisions or lists (phase 14); its tables change only by the foreign key of migration 0061.
- No scenario, parameter set, replay, shadow or seeded change (phase 16); `scn.detection_keys` keeps its shape and grants; `scn.shadow_results` is
  written for `live_alert_id` only, under the grant phase 16 already gives.
- No rating, override, segment, feature or feed write (phases 13 and 15); the alert score never reaches them (DR-103).
- No write to `canon.party`, `canon.txn` or any canonical table (phase 10); no CBS write (the connection stays read-only, phase 05).
- No new role, no new step-up class (the closed list of phase 01 is used as it is), no new problem code (API section 6 only), no new Valkey database.
- No notifications, no outbox topic beyond `alert.created`, `case.event` and `profile.breach` (section 7+; phase 22 subscribes later), no exports and no report (phases 21 and 22).
- No retention sweep and no legal hold (phase 23); the snapshot orphan sweep is an Open doubt.
- No data-model change outside the twelve tables of section 1 and the one foreign key on phase 14's table.
- No change to the phase 00 to 16 tables beyond the listed extensions (sections 4.5 and 1); no change to the phase 03 `_dev` rule or the masked-kind registry format.
- No change to the change target registry's routes, the route map or `change.route_map`; `queue_definition` registers with the existing routes.
- No AI assistant of any kind (P2) and no model score.
