# Phase 04: Change Control & Approval Workflows

**Goal:** One governed path for every change to an object that affects detection, reporting or access: a change
request with a computed class, volume effect and route, approved on the ported DBS workflow engine by people who are
never its maker, activated atomically with its audit and outbox entries, and followed by the emergency review and the
30- and 90-day post-change reviews; the engine itself (definitions, versions, instances, append-only events, the
task projection, claims, timeouts) with the inbox and task actions every later approval rides on; workflow-definition
authoring with validation, dry runs and publication by change request; the change and tuning reason codes; the model
and rule inventory; and the phase 03 promises kept: role grants, governed settings and Cedar bundles now change only
by change request.

**Dependencies:** Phase 03
**Macro-phase:** P0
**Complexity:** XL (32 routes with the evidence route of section 7.2, and the workflow engine)
**Binaries touched:** akr-api (routers `change_control`, `inbox`, `workflow_admin`, `inventory`; the phase 03 routers
`admin_users`, `authz` and `admin_system` gain the change-request branches) | akr-worker (timer handlers
`timer_cr_activate`, `timer_cr_emergency_review`, `timer_cr_post_review`, `timer_task_timeout`; scheduled
`change_sweep`) | `akr-ctl` (`workflow validate`, `workflow seed`, `workflow rebuild-instance`). akr-ingestd and
akr-assess unchanged
**Requirements carried:** SP-22 (one maker-checker mechanism: maker never approves, fresh second factor, atomic
activation with its audit entry, emergency changes that revert after 24 hours without approval when they loosen, a
second approver for volume-reducing changes; expiring whitelists arrive with their kinds in phases 12 and 14), SP-21
(four-eyes on change requests and tasks as `forbid` rules plus database checks, delegation included), SP-25 (Cedar
bundles activated by change request, dry run in shadow before enforcement), GV-26 (three change classes, maker-checker,
impact, the fewer-alerts flag, post-change reviews at 30 and 90 days), GV-32 (proposer is not approver; the validator
is not the owner), GV-01 (no activation without an inventory record, for the kinds that declare one; those kinds
arrive from phase 14 on), GV-04 (owner, developer, validator and committee on every inventory record), GV-02 and
GV-27 (every activated version names its change request and can be rolled back by a new one; scenario and model
versions arrive with their phases), S-39 (a `capacity` reason makes a change material and needs an approver outside
operations; the parameter register is phase 16), S-46 (the post-change review clock; the scenario review calendar is
phase 16), UX-60 (the risk or capacity tag of the reason on every volume-reducing change), UX-14 ("My inbox", partial)

## 1. SQLx Migration Changes

Three migrations, `backend/migrations/0014` to `0016` (canonical chain in roadmap.md, DR-23). Conventions as in
phases 00 to 03. `ref.reason_codes` moves here from phase 09 because a change request cannot exist without a reason
code (DR-53); phase 09 adds its other families and its administration routes. Nothing here touches the `audit`
schema.

```sql
-- migration 0014_change_requests_reason_codes.sql
-- Phase 04. Reason codes (change and tuning families), the change-request register and the deferred FKs owed by
-- phases 01 and 03 (DSL.md Part D, DR-53).

DO $$ BEGIN
    CREATE TYPE admin.change_class AS ENUM ('minor', 'material', 'emergency');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

DO $$ BEGIN
    CREATE TYPE admin.volume_effect AS ENUM ('increase', 'neutral', 'decrease', 'unknown');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

DO $$ BEGIN
    CREATE TYPE admin.change_status AS ENUM
        ('draft', 'submitted', 'approved', 'rejected', 'withdrawn', 'effective', 'failed', 'reverted');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS ref.reason_codes (
    code            TEXT PRIMARY KEY CHECK (code ~ '^[A-Z][A-Z0-9_]{2,63}$'),
    family          TEXT NOT NULL CHECK (family IN ('detection', 'decision', 'tuning', 'change', 'closure',
                                                    'override', 'exclusion')),
    tag             TEXT CHECK (tag IN ('risk', 'capacity')),            -- tuning and change only (S-39, UX-60)
    label           JSONB NOT NULL CHECK (label ? 'ar' AND label ? 'en'),
    template        JSONB,                                               -- detection: bilingual text (phase 09)
    typology        TEXT,
    cbi_refs        JSONB,
    display_fields  TEXT[],
    object_kinds    TEXT[],          -- change kinds this code may be used for; NULL = any kind of its family
    version         INT NOT NULL DEFAULT 1,
    active          BOOLEAN NOT NULL DEFAULT TRUE,                       -- never deleted: requests cite the code
    updated_by      UUID REFERENCES admin.users(id) ON DELETE RESTRICT,  -- NULL = seeded
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK ((family IN ('tuning', 'change')) = (tag IS NOT NULL)),
    CHECK (family = 'detection' OR template IS NULL)
);
CREATE INDEX IF NOT EXISTS idx_reason_codes_family ON ref.reason_codes (family) WHERE active;

-- Seeds: the change family (governed objects of every kind) and the tuning family (scenarios and parameter sets,
-- used from phase 16). Codes are product data; banks add their own through phase 09 (POST /ref/reason-codes, CR).
INSERT INTO ref.reason_codes (code, family, tag, label, object_kinds) VALUES
 ('CHG_REGULATORY_ORDER',    'change', 'risk',
  '{"ar": "أمر أو تعليمات من الجهة الرقابية", "en": "Regulator order or instruction"}', NULL),
 ('CHG_SANCTIONS_LIST',      'change', 'risk',
  '{"ar": "تغيير مرتبط بقوائم العقوبات", "en": "Sanctions-list driven change"}', NULL),
 ('CHG_AUDIT_FINDING',       'change', 'risk',
  '{"ar": "معالجة ملاحظة تدقيق", "en": "Remediation of an audit finding"}', NULL),
 ('CHG_RISK_ASSESSMENT',     'change', 'risk',
  '{"ar": "تحديث تقييم المخاطر", "en": "Updated risk assessment"}', NULL),
 ('CHG_NEW_PRODUCT',         'change', 'risk',
  '{"ar": "منتج أو قناة جديدة", "en": "New product or channel"}', NULL),
 ('CHG_SECURITY',            'change', 'risk',
  '{"ar": "متطلب أمني", "en": "Security requirement"}', NULL),
 ('CHG_CORRECTION',          'change', 'risk',
  '{"ar": "تصحيح إعداد غير صحيح", "en": "Correction of a wrong setting"}', NULL),
 ('CHG_OPERATIONAL_CAPACITY','change', 'capacity',
  '{"ar": "الطاقة التشغيلية", "en": "Operational capacity"}', NULL),
 ('ACC_JOINER',              'change', 'risk',
  '{"ar": "انضمام إلى الوظيفة", "en": "Joiner to the function"}', '{role_grant}'),
 ('ACC_MOVER',               'change', 'risk',
  '{"ar": "انتقال بين الوظائف", "en": "Move between functions"}', '{role_grant}'),
 ('ACC_TEMPORARY_COVER',     'change', 'risk',
  '{"ar": "تغطية مؤقتة", "en": "Temporary cover"}', '{role_grant}'),
 ('ACC_ASSESSMENT',          'change', 'risk',
  '{"ar": "مهمة تقييم أو تفتيش", "en": "Assessment or inspection engagement"}', '{role_grant}'),
 ('ACC_TASK_STUCK',          'change', 'risk',
  '{"ar": "مهمة متوقفة", "en": "Stuck task"}', '{task_assignment}'),
 ('TUNE_RECALIBRATION_PCT',  'tuning', 'risk',
  '{"ar": "إعادة معايرة النسب المئوية", "en": "Percentile recalibration"}', NULL),
 ('TUNE_BTL_RESULT',         'tuning', 'risk',
  '{"ar": "نتيجة عينة ما دون العتبة", "en": "Below-the-line sample result"}', NULL),
 ('TUNE_NEW_TYPOLOGY',       'tuning', 'risk',
  '{"ar": "نمط جديد", "en": "New typology"}', NULL),
 ('TUNE_DATA_CHANGE',        'tuning', 'risk',
  '{"ar": "تغيير في البيانات المصدرية", "en": "Change in source data"}', NULL),
 ('TUNE_FALSE_POSITIVE',     'tuning', 'capacity',
  '{"ar": "خفض التنبيهات غير المنتجة", "en": "Fewer unproductive alerts"}', NULL),
 ('TUNE_VOLUME_CAPACITY',    'tuning', 'capacity',
  '{"ar": "حجم التنبيهات مقابل الطاقة", "en": "Alert volume against capacity"}', NULL)
ON CONFLICT (code) DO NOTHING;

CREATE TABLE IF NOT EXISTS admin.change_requests (
    id                  UUID PRIMARY KEY,
    -- Open set: every phase registers its kinds in code (ChangeTargetRegistry, DR-53); an unknown kind is refused
    -- by the service, the CHECK only keeps the shape. `_` prefix = dev-only kinds (never in production builds).
    object_kind         TEXT NOT NULL CHECK (object_kind ~ '^_?[a-z][a-z0-9_]{1,47}$'),
    object_id           UUID,             -- the governed object when it has a UUID (grant, bundle, definition, task)
    object_key          TEXT NOT NULL CHECK (length(object_key) BETWEEN 1 AND 200),   -- natural key, one open request
    base_version        INT CHECK (base_version >= 0),   -- object version the change was made against; NULL = new
    proposed            JSONB NOT NULL,   -- normalised by the target: inline patch or draft reference (DSL.md D.1)
    maker_id            UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    maker_role          TEXT NOT NULL CHECK (maker_role ~ '^[A-Z0-9]{1,4}$'),   -- actor_role of the proposal (phase 03)
    reason_code         TEXT NOT NULL REFERENCES ref.reason_codes(code) ON DELETE RESTRICT,
    reason_tag          TEXT NOT NULL CHECK (reason_tag IN ('risk', 'capacity')),   -- copied at creation
    reason_text         TEXT NOT NULL CHECK (length(btrim(reason_text)) BETWEEN 1 AND 2000),
    evidence            JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(evidence) = 'array'),
    class_requested     admin.change_class,    -- the maker may raise the class, never lower it (E_CLASS_LOWERED)
    change_class        admin.change_class NOT NULL,
    volume_effect       admin.volume_effect NOT NULL,
    impact              JSONB NOT NULL DEFAULT '{}'::jsonb,
    warnings            JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(warnings) = 'array'),
    accept_warnings     BOOLEAN NOT NULL DEFAULT FALSE,
    emergency           BOOLEAN NOT NULL DEFAULT FALSE,
    route               TEXT CHECK (route ~ '^[a-z][a-z0-9_]{2,63}$'),   -- workflow definition key, fixed at submit
    instance_id         UUID,             -- FK added by 0015 (the engine tables come next)
    status              admin.change_status NOT NULL DEFAULT 'draft',
    submitted_at        TIMESTAMPTZ,
    decided_at          TIMESTAMPTZ,
    checker_id          UUID REFERENCES admin.users(id) ON DELETE RESTRICT,  -- the decision that ended the route
    stepup_id           UUID,             -- the checker's assertion; no FK (the step-up ledger is pruned)
    effective_at        TIMESTAMPTZ,      -- requested activation time; NULL = on approval
    activated_at        TIMESTAMPTZ,
    failure_reason      TEXT CHECK (failure_reason ~ '^[a-z][a-z_]*$'),
    review_instance_id  UUID,             -- emergency review on the material route (FK added by 0015)
    review_due_at       TIMESTAMPTZ,
    review_outcome      TEXT CHECK (review_outcome IN ('pending', 'confirmed', 'rejected', 'reverted', 'escalated')),
    reverted_at         TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    -- Maker is never checker (DSL.md D.4, GV-32); the engine and Cedar refuse it first (section 3.5).
    CHECK (checker_id IS NULL OR checker_id <> maker_id),
    CHECK ((change_class = 'emergency') = emergency),
    CHECK (class_requested IS NULL OR class_requested <= change_class),
    CHECK (status <> 'draft' OR submitted_at IS NULL),
    CHECK (status IN ('draft', 'withdrawn') OR (submitted_at IS NOT NULL AND route IS NOT NULL
                                                AND instance_id IS NOT NULL)),
    CHECK ((status IN ('draft', 'submitted', 'withdrawn')) = (decided_at IS NULL)),
    CHECK ((decided_at IS NULL) = (checker_id IS NULL)),
    CHECK ((status IN ('effective', 'reverted')) = (activated_at IS NOT NULL)),
    CHECK ((status = 'failed') = (failure_reason IS NOT NULL)),
    CHECK ((status = 'reverted') = (reverted_at IS NOT NULL)),
    CHECK ((review_outcome IS NULL) = (review_due_at IS NULL)),
    CHECK (review_outcome IS NULL OR (emergency AND activated_at IS NOT NULL)),
    CHECK (effective_at IS NULL OR change_class <> 'emergency')
);
-- One open request per governed object (an emergency change under review still counts as open).
CREATE UNIQUE INDEX IF NOT EXISTS uq_change_requests_open ON admin.change_requests (object_kind, object_key)
    WHERE status IN ('draft', 'submitted', 'approved') OR (status = 'effective' AND review_outcome = 'pending');
CREATE INDEX IF NOT EXISTS idx_change_requests_status  ON admin.change_requests (status, created_at DESC, id DESC);
CREATE INDEX IF NOT EXISTS idx_change_requests_kind    ON admin.change_requests (object_kind, created_at DESC, id DESC);
CREATE INDEX IF NOT EXISTS idx_change_requests_maker   ON admin.change_requests (maker_id);
CREATE INDEX IF NOT EXISTS idx_change_requests_checker ON admin.change_requests (checker_id) WHERE checker_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_change_requests_reason  ON admin.change_requests (reason_code);
CREATE INDEX IF NOT EXISTS idx_change_requests_object  ON admin.change_requests (object_id) WHERE object_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_change_requests_review  ON admin.change_requests (review_due_at)
    WHERE review_outcome = 'pending';

-- Deferred FKs owed by phases 01 and 03.
DO $$ BEGIN
    ALTER TABLE admin.role_grants ADD CONSTRAINT fk_role_grants_change_request
        FOREIGN KEY (change_request_id) REFERENCES admin.change_requests(id) ON DELETE RESTRICT;
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    ALTER TABLE admin.policy_bundles ADD CONSTRAINT fk_policy_bundles_change_request
        FOREIGN KEY (change_request_id) REFERENCES admin.change_requests(id) ON DELETE RESTRICT;
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
CREATE INDEX IF NOT EXISTS idx_role_grants_change_request ON admin.role_grants (change_request_id)
    WHERE change_request_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_policy_bundles_change_request ON admin.policy_bundles (change_request_id)
    WHERE change_request_id IS NOT NULL;

-- Content columns are INSERT-only: what the checker approves is byte for byte what the maker proposed. The
-- lifecycle columns are the only updates, with one exception: evidence, which the maker's evidence route
-- appends to (sections 7.2 and 7.18).
GRANT SELECT ON ref.reason_codes TO akr_app, akr_readonly;
GRANT SELECT, INSERT ON admin.change_requests TO akr_app;
GRANT UPDATE (route, instance_id, status, submitted_at, decided_at, checker_id, stepup_id, activated_at,
              failure_reason, review_instance_id, review_due_at, review_outcome, reverted_at, updated_at, version, evidence)
    ON admin.change_requests TO akr_app;
GRANT SELECT ON admin.change_requests TO akr_readonly;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification, note) VALUES
 ('admin', 'change_requests', 'proposed', 'detection-logic', 'Thresholds and logic of detection kinds (GV-54)'),
 ('admin', 'change_requests', 'impact',   'detection-logic', 'Alert volumes before and after')
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT v.s, v.t, c, 'internal'
FROM (VALUES
  ('ref',   'reason_codes',    ARRAY['code','family','tag','label','template','typology','cbi_refs','display_fields',
                                     'object_kinds','version','active','updated_by','updated_at']),
  ('admin', 'change_requests', ARRAY['id','object_kind','object_id','object_key','base_version','maker_id',
                                     'maker_role','reason_code','reason_tag','reason_text','evidence',
                                     'class_requested','change_class','volume_effect','warnings','accept_warnings',
                                     'emergency','route','instance_id','status','submitted_at','decided_at',
                                     'checker_id','stepup_id','effective_at','activated_at','failure_reason',
                                     'review_instance_id','review_due_at','review_outcome','reverted_at',
                                     'created_at','updated_at','version'])
) AS v(s, t, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('ref',   'reason_codes',    'R9', 'retirement', NULL, 'Retirement = inactive; requests and decisions cite codes'),
 ('admin', 'change_requests', 'R9', 'retirement', NULL,
  'Retirement = the governed object version is retired; configuration history (GV-27)')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

```sql
-- migration 0015_workflow_engine.sql
-- Phase 04. The DBS workflow engine tables, ported with their DBS names into `admin` (DATA-MODEL s.1, DR-08,
-- DR-56): definitions, versions, instances, append-only events and the task projection.

DO $$ BEGIN
    CREATE TYPE admin.workflow_def_status AS ENUM ('active', 'archived');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    CREATE TYPE admin.workflow_version_state AS ENUM ('draft', 'published', 'superseded');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    CREATE TYPE admin.instance_state AS ENUM ('running', 'completed', 'rejected', 'withdrawn', 'failed');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    CREATE TYPE admin.task_state AS ENUM ('open', 'claimed', 'done', 'cancelled', 'parked');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS admin.workflow_definitions (
    id              UUID PRIMARY KEY,
    key             TEXT NOT NULL UNIQUE CHECK (key ~ '^_?[a-z][a-z0-9_]{2,63}$'),
    status          admin.workflow_def_status NOT NULL DEFAULT 'active',
    shipped         BOOLEAN NOT NULL DEFAULT FALSE,   -- came with the release (E.4); still changed only by CR
    resource_kinds  TEXT[] NOT NULL CHECK (cardinality(resource_kinds) > 0),   -- subjects it may run on
    created_by      UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_wf_defs_created_by ON admin.workflow_definitions (created_by);

CREATE TABLE IF NOT EXISTS admin.workflow_definition_versions (
    id                 UUID PRIMARY KEY,
    definition_id      UUID NOT NULL REFERENCES admin.workflow_definitions(id) ON DELETE RESTRICT,
    version            INT NOT NULL CHECK (version >= 1),
    state              admin.workflow_version_state NOT NULL DEFAULT 'draft',
    dsl                JSONB NOT NULL,      -- semantic document; the layout key is split out (DBS)
    layout             JSONB,               -- canvas positions; the engine never reads it
    sha256             BYTEA NOT NULL CHECK (octet_length(sha256) = 32),   -- canonical hash of dsl (phase 02 Canon)
    validation         JSONB NOT NULL DEFAULT '{}'::jsonb,                 -- last report {errors[], warnings[]}
    changelog          JSONB CHECK (changelog IS NULL OR (changelog ? 'ar' AND changelog ? 'en')),
    published_via      TEXT CHECK (published_via IN ('seed', 'change_request')),
    change_request_id  UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    published_at       TIMESTAMPTZ,
    published_by       UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_by         UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    row_version        INT NOT NULL DEFAULT 1,   -- If-Match target for draft edits
    CONSTRAINT uq_wf_def_versions_family_version UNIQUE (definition_id, version),
    CHECK ((state = 'draft') = (published_at IS NULL)),
    CHECK ((published_at IS NULL) = (published_by IS NULL)),
    CHECK ((published_at IS NULL) = (published_via IS NULL)),
    CHECK ((published_via = 'change_request') = (change_request_id IS NOT NULL)),
    CHECK (state = 'draft' OR changelog IS NOT NULL)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_wf_def_versions_one_draft
    ON admin.workflow_definition_versions (definition_id) WHERE state = 'draft';
CREATE UNIQUE INDEX IF NOT EXISTS uq_wf_def_versions_one_published
    ON admin.workflow_definition_versions (definition_id) WHERE state = 'published';
CREATE INDEX IF NOT EXISTS idx_wf_def_versions_published_by ON admin.workflow_definition_versions (published_by)
    WHERE published_by IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_wf_def_versions_cr ON admin.workflow_definition_versions (change_request_id)
    WHERE change_request_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_wf_def_versions_created_by ON admin.workflow_definition_versions (created_by);

-- Publish freeze (DBS 0007): a published row is immutable except the flip to superseded; superseded is final.
CREATE OR REPLACE FUNCTION admin.wf_def_version_freeze() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog, admin AS $$
BEGIN
    IF OLD.state = 'superseded' THEN
        RAISE EXCEPTION 'workflow_definition_versions: superseded version % is immutable', OLD.id;
    END IF;
    IF OLD.state = 'published' THEN
        IF NEW.state = 'superseded'
           AND NEW.definition_id = OLD.definition_id AND NEW.version = OLD.version
           AND NEW.dsl = OLD.dsl AND NEW.layout IS NOT DISTINCT FROM OLD.layout AND NEW.sha256 = OLD.sha256
           AND NEW.changelog IS NOT DISTINCT FROM OLD.changelog
           AND NEW.published_at IS NOT DISTINCT FROM OLD.published_at
           AND NEW.published_by IS NOT DISTINCT FROM OLD.published_by
           AND NEW.published_via IS NOT DISTINCT FROM OLD.published_via
           AND NEW.change_request_id IS NOT DISTINCT FROM OLD.change_request_id THEN
            RETURN NEW;
        END IF;
        RAISE EXCEPTION 'workflow_definition_versions: published version % is immutable', OLD.id;
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_wf_def_version_freeze ON admin.workflow_definition_versions;
CREATE TRIGGER trg_wf_def_version_freeze BEFORE UPDATE ON admin.workflow_definition_versions
    FOR EACH ROW EXECUTE FUNCTION admin.wf_def_version_freeze();

CREATE TABLE IF NOT EXISTS admin.workflow_instances (
    id                     UUID PRIMARY KEY,
    definition_version_id  UUID NOT NULL REFERENCES admin.workflow_definition_versions(id) ON DELETE RESTRICT,
    resource_kind          TEXT NOT NULL CHECK (resource_kind ~ '^_?[a-z][a-z0-9_]{1,47}$'),
    resource_id            UUID NOT NULL,
    purpose                TEXT NOT NULL DEFAULT 'decision'
                           CHECK (purpose IN ('decision', 'emergency_review', 'post_review')),
    classification         TEXT NOT NULL DEFAULT 'internal' CHECK (classification IN ('internal', 'restricted')),
    initiator_id           UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    maker_id               UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- excluded from deciding steps
    context                JSONB NOT NULL DEFAULT '{}'::jsonb,   -- subject attributes frozen at start (cr.*, case.*)
    state                  admin.instance_state NOT NULL DEFAULT 'running',
    outcome                TEXT CHECK (outcome ~ '^[a-z][a-z_]*$'),
    started_at             TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at           TIMESTAMPTZ,
    version                INT NOT NULL DEFAULT 1,   -- If-Match target for step actions (DBS I2)
    CHECK ((state = 'running') = (completed_at IS NULL)),
    CHECK ((state = 'running') = (outcome IS NULL))
);
CREATE INDEX IF NOT EXISTS idx_instances_resource  ON admin.workflow_instances (resource_kind, resource_id);
CREATE INDEX IF NOT EXISTS idx_instances_state     ON admin.workflow_instances (state, started_at);
CREATE INDEX IF NOT EXISTS idx_instances_def       ON admin.workflow_instances (definition_version_id);
CREATE INDEX IF NOT EXISTS idx_instances_initiator ON admin.workflow_instances (initiator_id);
CREATE INDEX IF NOT EXISTS idx_instances_maker     ON admin.workflow_instances (maker_id) WHERE maker_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS admin.instance_events (
    id            UUID PRIMARY KEY,
    instance_id   UUID NOT NULL REFERENCES admin.workflow_instances(id) ON DELETE RESTRICT,
    seq           INT NOT NULL CHECK (seq >= 1),
    occurred_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    actor_id      UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- NULL = the engine
    on_behalf_of  UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- the delegator when acting by delegation
    kind          TEXT NOT NULL CHECK (kind ~ '^[a-z][a-z_]*$'),
    payload       JSONB NOT NULL DEFAULT '{}'::jsonb,
    CONSTRAINT uq_instance_events_seq UNIQUE (instance_id, seq)
);
CREATE INDEX IF NOT EXISTS idx_instance_events_actor  ON admin.instance_events (actor_id) WHERE actor_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_instance_events_behalf ON admin.instance_events (on_behalf_of)
    WHERE on_behalf_of IS NOT NULL;

CREATE OR REPLACE FUNCTION admin.instance_events_append_only() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog, admin AS $$
BEGIN
    RAISE EXCEPTION 'instance_events is append-only (attempted % on instance %)', TG_OP, OLD.instance_id;
END $$;
DROP TRIGGER IF EXISTS trg_instance_events_append_only ON admin.instance_events;
CREATE TRIGGER trg_instance_events_append_only BEFORE UPDATE OR DELETE ON admin.instance_events
    FOR EACH ROW EXECUTE FUNCTION admin.instance_events_append_only();

CREATE TABLE IF NOT EXISTS admin.tasks (
    id             UUID PRIMARY KEY,
    instance_id    UUID NOT NULL REFERENCES admin.workflow_instances(id) ON DELETE RESTRICT,
    step_id        TEXT NOT NULL CHECK (step_id ~ '^[a-z][a-z0-9_]{0,63}$'),
    step_type      TEXT NOT NULL CHECK (step_type IN ('form', 'approve', 'sign', 'review')),
    deciding       BOOLEAN NOT NULL,                 -- approve and sign steps
    pass           SMALLINT NOT NULL DEFAULT 1 CHECK (pass >= 1),   -- 1 + rework returns before this task
    state          admin.task_state NOT NULL DEFAULT 'open',
    assignee_kind  TEXT NOT NULL CHECK (assignee_kind IN ('role', 'pool', 'named_officer', 'case_acl', 'subject_party')),
    role           TEXT CHECK (role ~ '^[A-Z0-9]{1,4}$'),
    pool_id        UUID REFERENCES admin.pools(id) ON DELETE RESTRICT,
    officer        TEXT CHECK (officer ~ '^[a-z][a-z_]*$'),
    party          TEXT CHECK (party ~ '^[a-z][a-z_]*$'),
    acl_grant      TEXT CHECK (acl_grant IN ('read', 'work', 'decide')),
    branch_id      UUID REFERENCES admin.branches(id) ON DELETE RESTRICT,   -- branch-scoped routing; NULL = bank
    maker_id       UUID REFERENCES admin.users(id) ON DELETE RESTRICT,      -- copied from the instance (deciding)
    claimed_by     UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    claimed_at     TIMESTAMPTZ,
    acting_for     UUID REFERENCES admin.users(id) ON DELETE RESTRICT,      -- delegator when acting by delegation
    completed_by   UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    completed_at   TIMESTAMPTZ,
    outcome        TEXT CHECK (outcome IN ('approved', 'rejected', 'signed', 'submitted', 'reviewed')),
    decider        UUID GENERATED ALWAYS AS (COALESCE(acting_for, completed_by)) STORED,
    stepup_id      UUID,                                                    -- no FK (ledger pruned)
    due_at         TIMESTAMPTZ,
    timer_id       UUID REFERENCES admin.timers(id) ON DELETE RESTRICT,
    reminded_at    TIMESTAMPTZ,
    escalated_at   TIMESTAMPTZ,
    title          JSONB NOT NULL CHECK (title ? 'ar' AND title ? 'en'),        -- neutral wording (SP-31)
    step_label     JSONB NOT NULL CHECK (step_label ? 'ar' AND step_label ? 'en'),
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    version        INT NOT NULL DEFAULT 1,           -- If-Match target for claim and release (DBS I2)
    CHECK ((assignee_kind = 'role') = (role IS NOT NULL)),
    CHECK ((assignee_kind = 'pool') = (pool_id IS NOT NULL)),
    CHECK ((assignee_kind = 'named_officer') = (officer IS NOT NULL)),
    CHECK ((assignee_kind = 'subject_party') = (party IS NOT NULL)),
    CHECK ((assignee_kind = 'case_acl') = (acl_grant IS NOT NULL)),
    CHECK (NOT deciding OR step_type IN ('approve', 'sign')),
    CHECK (state NOT IN ('open', 'parked') OR claimed_by IS NULL),
    CHECK (state <> 'claimed' OR claimed_by IS NOT NULL),
    CHECK ((claimed_by IS NULL) = (claimed_at IS NULL)),
    CHECK ((state = 'done') = (completed_by IS NOT NULL)),
    CHECK ((completed_by IS NULL) = (completed_at IS NULL)),
    CHECK ((state = 'done') = (outcome IS NOT NULL)),
    CHECK (acting_for IS NULL OR acting_for <> claimed_by),
    -- Four-eyes (DATA-MODEL, DR-57): the person completing a deciding step is never the maker, and never acts for
    -- the maker by delegation; a deciding step is never completed without a step-up assertion.
    CHECK (completed_by IS NULL OR maker_id IS NULL OR completed_by <> maker_id),
    CHECK (acting_for IS NULL OR maker_id IS NULL OR acting_for <> maker_id),
    CHECK (state <> 'done' OR NOT deciding OR stepup_id IS NOT NULL)
);
-- Nobody decides twice in one pass of one instance, in person or through a delegate (DR-57).
CREATE UNIQUE INDEX IF NOT EXISTS uq_tasks_decision_person ON admin.tasks (instance_id, pass, completed_by)
    WHERE deciding AND state = 'done';
CREATE UNIQUE INDEX IF NOT EXISTS uq_tasks_decision_decider ON admin.tasks (instance_id, pass, decider)
    WHERE deciding AND state = 'done';
-- Inbox paths (section 3.7).
CREATE INDEX IF NOT EXISTS idx_tasks_role_open    ON admin.tasks (role, state) WHERE state IN ('open', 'parked');
CREATE INDEX IF NOT EXISTS idx_tasks_pool_open    ON admin.tasks (pool_id, state) WHERE state IN ('open', 'parked');
CREATE INDEX IF NOT EXISTS idx_tasks_officer_open ON admin.tasks (officer, state) WHERE state IN ('open', 'parked');
CREATE INDEX IF NOT EXISTS idx_tasks_claimed      ON admin.tasks (claimed_by, state) WHERE state = 'claimed';
CREATE INDEX IF NOT EXISTS idx_tasks_completed    ON admin.tasks (completed_by, completed_at DESC)
    WHERE completed_by IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_tasks_instance     ON admin.tasks (instance_id);
CREATE INDEX IF NOT EXISTS idx_tasks_due          ON admin.tasks (due_at) WHERE state IN ('open', 'claimed');
CREATE INDEX IF NOT EXISTS idx_tasks_maker        ON admin.tasks (maker_id) WHERE maker_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_tasks_acting_for   ON admin.tasks (acting_for) WHERE acting_for IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_tasks_branch       ON admin.tasks (branch_id) WHERE branch_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_tasks_timer        ON admin.tasks (timer_id) WHERE timer_id IS NOT NULL;

-- FKs from 0014.
DO $$ BEGIN
    ALTER TABLE admin.change_requests ADD CONSTRAINT fk_change_requests_instance
        FOREIGN KEY (instance_id) REFERENCES admin.workflow_instances(id) ON DELETE RESTRICT;
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    ALTER TABLE admin.change_requests ADD CONSTRAINT fk_change_requests_review_instance
        FOREIGN KEY (review_instance_id) REFERENCES admin.workflow_instances(id) ON DELETE RESTRICT;
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
CREATE INDEX IF NOT EXISTS idx_change_requests_instance ON admin.change_requests (instance_id)
    WHERE instance_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_change_requests_review_instance ON admin.change_requests (review_instance_id)
    WHERE review_instance_id IS NOT NULL;

GRANT SELECT, INSERT ON admin.workflow_definitions, admin.workflow_definition_versions, admin.workflow_instances,
                         admin.instance_events, admin.tasks TO akr_app;
GRANT UPDATE (status) ON admin.workflow_definitions TO akr_app;
GRANT UPDATE (state, dsl, layout, sha256, validation, changelog, published_via, change_request_id, published_at,
              published_by, updated_at, row_version)
    ON admin.workflow_definition_versions TO akr_app;
GRANT UPDATE (state, outcome, completed_at, version) ON admin.workflow_instances TO akr_app;
GRANT UPDATE (state, claimed_by, claimed_at, acting_for, completed_by, completed_at, outcome, stepup_id, due_at,
              timer_id, reminded_at, escalated_at, title, step_label, role, pool_id, officer, party, acl_grant,
              branch_id, assignee_kind, version)
    ON admin.tasks TO akr_app;
-- The one sanctioned delete: the projection rebuild, under the instance lock, audited (section 3.4).
GRANT DELETE ON admin.tasks TO akr_app;
GRANT SELECT ON admin.workflow_definitions, admin.workflow_definition_versions, admin.workflow_instances,
                admin.instance_events, admin.tasks TO akr_readonly;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', v.t, c, 'internal'
FROM (VALUES
  ('workflow_definitions',         ARRAY['id','key','status','shipped','resource_kinds','created_by','created_at']),
  ('workflow_definition_versions', ARRAY['id','definition_id','version','state','dsl','layout','sha256','validation',
                                         'changelog','published_via','change_request_id','published_at',
                                         'published_by','created_by','created_at','updated_at','row_version']),
  ('workflow_instances',           ARRAY['id','definition_version_id','resource_kind','resource_id','purpose',
                                         'classification','initiator_id','maker_id','context','state','outcome',
                                         'started_at','completed_at','version']),
  ('instance_events',              ARRAY['id','instance_id','seq','occurred_at','actor_id','on_behalf_of','kind',
                                         'payload']),
  ('tasks',                        ARRAY['id','instance_id','step_id','step_type','deciding','pass','state',
                                         'assignee_kind','role','pool_id','officer','party','acl_grant','branch_id',
                                         'maker_id','claimed_by','claimed_at','acting_for','completed_by',
                                         'completed_at','outcome','decider','stepup_id','due_at','timer_id',
                                         'reminded_at','escalated_at','title','step_label','created_at','version'])
) AS v(t, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'workflow_definitions',         'R9', 'retirement', NULL, 'Retirement = archived'),
 ('admin', 'workflow_definition_versions', 'R9', 'retirement', NULL,
  'Retirement = superseded; instances pin their version'),
 ('admin', 'workflow_instances',           'R9', 'retirement', NULL,
  'R9 for change requests; instances on other resources take the resource class (R6 cases and reports), phase 23'),
 ('admin', 'instance_events',              'R9', 'retirement', NULL, 'As its instance'),
 ('admin', 'tasks',                        'R9', 'retirement', NULL, 'As its instance')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

```sql
-- migration 0016_inventory_change_settings.sql
-- Phase 04. The inventory of rules, models and tools (GV-01, GV-04) and the change-control settings.

CREATE TABLE IF NOT EXISTS admin.inventory_items (
    id                   UUID PRIMARY KEY,
    kind                 TEXT NOT NULL CHECK (kind IN ('rule', 'parameter_set', 'model', 'matcher', 'reference_list',
                                                       'llm_component', 'prompt_template')),
    object_id            UUID NOT NULL,          -- the scenario, parameter set, model ... it describes
    name                 JSONB NOT NULL CHECK (name ? 'ar' AND name ? 'en'),
    tier                 SMALLINT NOT NULL CHECK (tier BETWEEN 1 AND 3),
    tier_basis           JSONB NOT NULL DEFAULT '{}'::jsonb,   -- answers to the tiering grid (GV-03, phase 27)
    owner_id             UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    developer            TEXT NOT NULL CHECK (length(btrim(developer)) BETWEEN 1 AND 200),   -- vendor, bank unit or firm
    validator_id         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    committee            TEXT NOT NULL CHECK (length(btrim(committee)) BETWEEN 1 AND 200),
    user_roles           TEXT[] NOT NULL DEFAULT '{}',   -- roles that act on its outputs
    status               TEXT NOT NULL DEFAULT 'registered'
                         CHECK (status IN ('registered', 'in_validation', 'approved_for_use', 'restricted', 'retired')),
    last_validated_at    TIMESTAMPTZ,
    next_validation_due  DATE,
    created_by           UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    version              INT NOT NULL DEFAULT 1,
    CONSTRAINT uq_inventory_items_object UNIQUE (kind, object_id),
    CHECK (validator_id IS NULL OR validator_id <> owner_id),               -- GV-32
    CHECK (status <> 'approved_for_use' OR (validator_id IS NOT NULL AND last_validated_at IS NOT NULL))
);
CREATE INDEX IF NOT EXISTS idx_inventory_items_owner     ON admin.inventory_items (owner_id);
CREATE INDEX IF NOT EXISTS idx_inventory_items_validator ON admin.inventory_items (validator_id)
    WHERE validator_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_inventory_items_due       ON admin.inventory_items (next_validation_due)
    WHERE status <> 'retired';
CREATE INDEX IF NOT EXISTS idx_inventory_items_created_by ON admin.inventory_items (created_by);

GRANT SELECT, INSERT ON admin.inventory_items TO akr_app;
GRANT UPDATE (name, tier, tier_basis, owner_id, developer, validator_id, committee, user_roles, status,
              last_validated_at, next_validation_due, updated_at, version)
    ON admin.inventory_items TO akr_app;
GRANT SELECT ON admin.inventory_items TO akr_readonly;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', 'inventory_items', c, 'internal'
FROM unnest(ARRAY['id','kind','object_id','name','tier','tier_basis','owner_id','developer','validator_id',
                  'committee','user_roles','status','last_validated_at','next_validation_due','created_by',
                  'created_at','updated_at','version']) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'inventory_items', 'R9', 'retirement', NULL, 'Retirement = retired; model items stay R9, model evidence is R11 in phase 27 tables (DR-392)')
ON CONFLICT (schema_name, table_name) DO NOTHING;

-- Settings. Bounds in the registry (DR-34). The route map is data the bank changes by change request (DR-54).
INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('change.evidence_max_age_days',        '30', 'integer', NULL, TRUE,
  '{"ar": "أقصى عمر للأدلة عند الاعتماد (أيام)", "en": "Maximum evidence age at approval (days)"}'),
 ('change.route_map',                    '{"rules": [
    {"class": "emergency", "route": "cr_emergency"},
    {"object_kind": ["role_grant", "policy_bundle", "task_assignment"], "maker_role": "SEC", "route": "cr_access_adm"},
    {"object_kind": ["role_grant", "policy_bundle", "task_assignment"], "route": "cr_access"},
    {"object_kind": ["setting"], "key_prefix": ["authz.", "identity.", "stepup.", "session.", "delegation.", "access."],
     "route": "cr_access"},
    {"object_kind": ["setting"], "key_prefix": ["change."], "route": "cr_material"},
    {"object_kind": ["workflow_definition"], "route": "cr_workflow"},
    {"object_kind": ["mapping", "code_map"], "route": "cr_mapping"},
    {"class": "material", "validated": true, "route": "cr_material_validated"},
    {"class": "material", "route": "cr_material"},
    {"class": "minor", "route": "cr_minor"}]}', 'json', NULL, TRUE,
  '{"ar": "مسارات اعتماد طلبات التغيير", "en": "Change-request approval routes"}'),
 ('change.emergency_review_h',           '24', 'integer',
  '{"max": 24, "basis": "SP-22: an emergency change that loosens is reverted if not approved within 24 hours"}', TRUE,
  '{"ar": "مهلة مراجعة التغيير الطارئ المخفِّف (ساعات)", "en": "Review window of a loosening emergency change (hours)"}'),
 ('change.emergency_review_working_days','5', 'integer',
  '{"max": 5, "basis": "research file 27 B26.2: emergency changes reviewed on the material route within 5 working days"}', TRUE,
  '{"ar": "مهلة مراجعة التغيير الطارئ (أيام عمل)", "en": "Review window of other emergency changes (working days)"}'),
 ('change.post_review_days',             '[30, 90]', 'json', NULL, TRUE,
  '{"ar": "مواعيد المراجعة اللاحقة للتغيير الجوهري (أيام)", "en": "Post-change review points for material changes (days)"}'),
 ('change.policy_shadow_min_h',          '24', 'integer', NULL, TRUE,
  '{"ar": "أدنى مدة تشغيل تجريبي لحزمة الصلاحيات قبل اعتمادها (ساعات)", "en": "Minimum shadow run of a policy bundle before approval (hours)"}'),
 ('change.draft_ttl_days',               '30', 'integer', NULL, FALSE,
  '{"ar": "مدة بقاء مسودات طلبات التغيير (أيام)", "en": "Lifetime of change-request drafts (days)"}'),
 ('access.assessor_max_days',            '90', 'integer', NULL, TRUE,
  '{"ar": "أقصى مدة لصلاحية المقيِّم أو المفتش (أيام)", "en": "Maximum length of an assessor grant (days)"}'),
 ('schedule.change_sweep',               '"0 15 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مسح طلبات التغيير والمهام", "en": "Change and task sweep schedule"}')
ON CONFLICT (key) DO NOTHING;
```

The route map is evaluated top to bottom, first match wins (section 3.1). A rule may name a family that is not yet
published (the shipped definitions are published by `akr-ctl workflow seed` in the same install step, section 3.8);
submit then answers 409 `E_NOT_ALLOWED_IN_STATE` (`route_unavailable`) rather than guessing another route. A change
to the map is validated: every class has a fallback rule, and every named family exists and runs on
`change_request`.

## 2. DDD Implementation (Rust)

**Crate placement** (workspace pinned in phase 00; layer rules in `ddd.md`):

```
backend/
├── schemas/workflow-dsl.v1.json      # the DBS file ported with the AKR changes (DSL.md Part E), include_str!
├── schemas/change-request.v1.json    # DSL.md D.1 body
├── workflows/shipped/*.json          # the nine shipped definitions (section 3.8); product data, not fixtures
├── crates/
│   ├── dsl/      # + workflow (serde model, schema pass, semantic validator, split/merge layout, pure stepper used by
│   │             #   the engine and the test-run) and change_request (body model, schema pass); IO-free (DBS)
│   ├── domain/   # + change (entities, ChangeTarget port, registry, class and route computation),
│   │             #   workflow (entities, WorkflowSubject, TerminalEffect, Holders, TaskProjector, events), inventory
│   ├── db/       # + Pg repositories for the eight tables; engine tx helpers (instance lock)
│   ├── authz/    # + resource adapters ChangeRequest, ChangeKind, Task, WorkflowDefinition, InventoryItem
│   └── jobs/     # + timer kinds cr_activate, cr_emergency_review, cr_post_review, task_timeout
├── api/          # + routers change_control, inbox, workflow_admin, inventory; ChangeRequestService wiring
├── worker/       # + timer handlers and change_sweep
└── ctl/          # + workflow validate | seed | rebuild-instance
```

- **Domain entities** (`Debug, Serialize, Deserialize, FromRow, ToSchema`; fields mirror section 1 exactly, enums as
  Rust enums with `#[sqlx(type_name = "admin.change_class")]` and so on): `ReasonCode`, `ChangeRequest`,
  `WorkflowDefinition`, `WorkflowDefinitionVersion`, `WorkflowInstance`, `InstanceEvent`, `Task`, `InventoryItem`.
  DSL types carry `#[schema(as = WfDsl*)]` from the start (DBS decision 62: utoipa's flat registry let a DSL `FieldType`
  overwrite a domain type of the same name).
- **Value objects:** `ObjectKind(&'static str)` (registered kinds only), `ObjectKey`, `ChangeClass`, `VolumeEffect`,
  `ChangeStatus`, `ReasonTag`, `EvidenceRef { kind: EvidenceKind, id: Uuid }` with `EvidenceKind` = `evidence_object`
  (phase 02 vault: regulator orders, signed documents), `shadow_report`, `validation_run`, `test_run`, `replay`,
  `btl_sample`, `shadow_comparison`, `impact_analysis` (the last four resolve from phases 16 and 10), `RouteRule`,
  `RouteMap`, `AssigneeSpec` (`Role { role, scope }`, `Pool { pool }`, `NamedOfficer { officer }`, `CaseAcl { grant
  }`, `SubjectParty { party }`), `TaskOutcome`, `InstancePurpose`.
- **Ports** (domain traits; implementations registered at start-up in `api` and `worker`, the same registry in both):

```rust
/// One per governed object kind (DR-53). Phase 04 registers role_grant, setting, policy_bundle,
/// workflow_definition, task_assignment and, in dev builds only, _dev_object.
#[async_trait]
pub trait ChangeTarget: Send + Sync {
    fn kind(&self) -> ObjectKind;
    fn reason_families(&self) -> &'static [&'static str];     // ["change"] or ["tuning", "change"]
    fn generic_create(&self) -> bool;                          // reachable through POST /change-requests
    fn emergency_allowed(&self) -> bool;                       // requires revert()
    fn inventory_kind(&self) -> Option<InventoryKind>;         // GV-01 gate (none in this phase)
    async fn prepare(&self, ctx: &AuthContext, body: &serde_json::Value) -> Result<Prepared, ChangeError>;
    async fn current_version(&self, conn: &mut PgConnection, cr: &ChangeRequest) -> Result<Option<i32>, ChangeError>;
    async fn on_submitted(&self, tx: &mut Tx, cr: &ChangeRequest) -> Result<(), ChangeError>;
    async fn check_before_decision(&self, tx: &mut Tx, cr: &ChangeRequest) -> Result<Vec<ValidationIssue>, ChangeError>;
    async fn activate(&self, tx: &mut Tx, cr: &ChangeRequest, actor: ActorRef) -> Result<Activated, ChangeError>;
    async fn revert(&self, tx: &mut Tx, cr: &ChangeRequest, actor: ActorRef) -> Result<(), ChangeError>;
    async fn on_closed(&self, tx: &mut Tx, cr: &ChangeRequest, how: ClosedAs) -> Result<(), ChangeError>;
    async fn diff(&self, cr: &ChangeRequest) -> Result<ChangeDiff, ChangeError>;
}
pub struct Prepared {
    pub object_id: Option<Uuid>, pub object_key: String, pub base_version: Option<i32>,
    pub proposed: serde_json::Value, pub base_class: ChangeClass, pub volume_effect: VolumeEffect,
    pub validated_route: bool, pub impact: serde_json::Value, pub warnings: Vec<ValidationIssue>,
    pub required_evidence: Vec<EvidenceKind>,               // for the computed class (DSL.md D.3)
    pub route_attrs: BTreeMap<String, String>,              // e.g. {"key_prefix": "authz."} for settings
}
pub enum ClosedAs { Rejected, Withdrawn, Failed, Reverted }

/// One per resource kind an instance can run on. Phase 04 registers change_request (and _dev_masked in dev builds);
/// case (18), report (21), kyc_review (15), exit_decision (15) follow.
#[async_trait]
pub trait WorkflowSubject: Send + Sync {
    fn resource_kind(&self) -> &'static str;
    fn attribute_prefix(&self) -> &'static str;                // "cr" for change_request
    fn attributes(&self) -> &'static [(&'static str, AttrType)];   // what conditions may read (validator)
    fn parties(&self) -> &'static [&'static str];              // what subject_party may name: ["maker", "approvers"]
    async fn start_context(&self, conn: &mut PgConnection, id: Uuid) -> Result<SubjectContext, WorkflowError>;
    async fn party_members(&self, conn: &mut PgConnection, id: Uuid, party: &str) -> Result<Vec<Uuid>, WorkflowError>;
    async fn visible_to(&self, principal: &AuthContext, id: Uuid) -> Result<bool, WorkflowError>;
    // SQL predicates over `t` (tasks) and `i` (instances) with the principal bound as $1, so the inbox filters and
    // counts in one query (section 3.7, phase 03 rule). None = every principal may see the subject's tasks.
    fn party_sql(&self, party: &str) -> Option<&'static str>;
    fn visibility_sql(&self) -> Option<&'static str>;
    async fn on_ended(&self, tx: &mut Tx, inst: &WorkflowInstance, end: InstanceEnd) -> Result<(), WorkflowError>;
}
pub struct SubjectContext {
    pub attrs: serde_json::Map<String, serde_json::Value>,     // frozen into workflow_instances.context
    pub classification: Classification, pub maker_id: Option<Uuid>, pub title: I18nString,   // neutral
    pub branch_id: Option<Uuid>,
}

/// Terminal effects (DSL.md E.3). Phase 04 registers activate_change, open_review and record_post_review and declares
/// notify deferrable; record_decision (18), release_for_filing (21) and create_ops_task (20) register later, and a
/// definition naming an unregistered, non-deferrable effect cannot be published (E_EFFECT_UNAVAILABLE).
#[async_trait]
pub trait TerminalEffect: Send + Sync {
    fn kind(&self) -> &'static str;
    fn resource_kinds(&self) -> &'static [&'static str];
    async fn run(&self, tx: &mut Tx, inst: &WorkflowInstance, params: &serde_json::Value, actor: ActorRef)
        -> Result<EffectReport, WorkflowError>;
}

/// The single computation of "who may hold this task" (DBS lesson 8.6: the inbox, the Cedar adapter and the engine
/// read it from one place).
pub trait Holders: Send + Sync {
    async fn eligible(&self, conn: &mut PgConnection, task: &Task, principal: &AuthContext) -> Result<Eligibility, WorkflowError>;
    async fn holder_count(&self, conn: &mut PgConnection, task: &Task) -> Result<i64, WorkflowError>;
}
pub struct Eligibility { pub eligible: bool, pub acting_for: Option<Uuid>, pub excluded: bool }

pub trait NamedOfficers: Send + Sync {          // phase 04: mlro -> active holders of M (MLRO and deputy)
    async fn members(&self, conn: &mut PgConnection, officer: &str) -> Result<Vec<Uuid>, WorkflowError>;
    fn known(&self) -> &'static [&'static str];
}
```

- **Repository traits** (`domain::repositories`; engine writes take `&mut PgConnection` inside the caller's tx):
  - `ReasonCodeRepository { get(code) -> Option<ReasonCode>; list(family, active_only) -> Vec<ReasonCode> }`
  - `ChangeRequestRepository { insert(tx, NewChangeRequest) -> ChangeRequest; get(id); get_for_update(tx, id);
    list(ChangeFilter, Cursor, limit) -> Page<ChangeRequest>; open_for(object_kind, object_key) ->
    Option<ChangeRequest>; set_submitted(tx, id, route, instance_id, expected_version); set_decided(tx, id, status,
    checker_id, stepup_id); set_effective(tx, id, activated_at, review: Option<(DateTime, ReviewOutcome)>);
    set_failed(tx, id, reason); set_withdrawn(tx, id, expected_version); set_review(tx, id, ReviewOutcome);
    set_reverted(tx, id, at); due_drafts(cutoff, limit) -> Vec<Uuid> }`
  - `WorkflowDefinitionRepository` (DBS) `{ find_by_key(key); get(id); list(Cursor, limit) -> Page<DefinitionRow>;
    versions(definition_id) -> Vec<WorkflowDefinitionVersion>; published(definition_id) -> Option<Version>; draft(
    definition_id) -> Option<Version>; insert_family(tx, NewDefinition); insert_version(tx, NewVersion);
    replace_draft(tx, id, dsl, layout, sha256, validation, expected_row_version); publish(tx, version_id,
    changelog, via, cr_id, actor) (supersedes the previous published version in the same statement pair) }`
  - `InstanceRepository { insert(tx, NewInstance); get(id); get_for_update(tx, id) (the serialization point,
    DBS I1); update_projection(tx, id, state, outcome, completed_at) (bumps version); for_resource(kind, id) ->
    Vec<WorkflowInstance> }`
  - `EventRepository { append(tx, NewEvent) -> InstanceEvent; list(instance_id) -> Vec<InstanceEvent>; max_seq(tx,
    instance_id) -> i32 }`
  - `TaskRepository { insert(tx, NewTask); get(id); for_instance(tx, instance_id, states); update(tx, &Task)
    (version-bumped full projection write); delete_for_instance(tx, instance_id) (rebuild only); inbox(InboxQuery,
    Cursor, limit) -> Page<TaskRow>; counts(InboxQuery) -> InboxCounts; claimed_by_ineligible(limit);
    open_without_holders(limit); parked(limit) }`
  - `InventoryRepository { list(InventoryFilter, Cursor, limit); get(id); by_object(kind, object_id); insert(tx,
    NewInventoryItem); update(tx, id, InventoryPatch, expected_version) }`
- **Repository impls** (`db`): `Pg*` for each, `query_as!` macros, `RETURNING *`. `PgTaskRepository::inbox` is the
  hot query (section 3.7); `PgChangeRequestRepository::list` uses keyset `(created_at DESC, id DESC)`.
- **Services** (section 3): `ChangeRequestService`, `RouteSelector`, `WorkflowEngine`, `TaskProjector` (pure),
  `DslValidator` (facade over `dsl::workflow`), `DefinitionService` (authoring and the seed loader), `InboxService`,
  `InventoryService`, `ChangeTimers` (worker).
- **DTOs** (`Deserialize, ToSchema` requests; `Serialize, ToSchema` responses):

| DTO | Purpose |
|-|-|
| `CreateChangeRequest` | `object_kind`, `body` (the target's proposal), `reason_code`, `reason_text`, `evidence[]`, `class_requested?`, `effective_at?`, `emergency?`, `accept_warnings?`, `submit?` (default false) |
| `ChangeRequestView` | All columns except `proposed` and `impact` for callers without `detection-logic` clearance (section 3.10), plus `reason_label`, `route_label`, `current_steps[]`, `my_task_id?`, `can_withdraw`, `instance_version` |
| `ChangeRequestDetail` | `ChangeRequestView` + `diff` (`ChangeDiff`), `evidence[]` resolved (`kind`, `id`, `created_at`, `fresh`), `impact`, `warnings[]`, `tasks[]` (`TaskSummary`), `timeline[]` (`TimelineEvent`) |
| `ChangeDiff` | `{ format: "fields" \| "files" \| "json_paths", entries: [{path, before, after}] }` (DESIGN-SYSTEM diff rule: old struck through, new with the start-edge bar) |
| `DecideRequest` | `comment?` (approve and sign, section 3.4 step 3), `comment` (reject) |
| `ProposeGrant` | `role`, `scope` (`{all_branches: true}` or `{branches[], pools[]}`), `valid_from?`, `valid_to?`, `reason_code`, `reason_text`, `effective_at?` |
| `ProposePolicy` | exactly one of `staged_version` (an installed release bundle) or `bundle` (`{schema, files: [{name, content}], sod, manifest}`); `reason_code`, `reason_text`, `effective_at?` |
| `UpdateSetting` (phase 03, extended) | `value`, plus `reason_code`, `reason_text`, `effective_at?`, `emergency?`, `evidence[]` required when the key is governed |
| `ProposeReassign` | `to` (`{user_id}` or `{pool_id}` or `"queue"`), `reason_code`, `reason_text` |
| `InboxQuery` | `view` (`mine`, `pool`, `waiting`, `history`), `kind?` (resource kind), `step_type?`, `overdue?`, cursor |
| `TaskRow` | `id`, `title`, `step_label`, `step_type`, `deciding`, `state`, `resource_kind`, `resource_id` (only when visible), `due_at`, `overdue`, `claimed_by_me`, `escalated`, `version`, `instance_version` |
| `InboxCounts` | `mine`, `pool`, `waiting`, `overdue` (computed after the visibility filter) |
| `TaskDetail` | `TaskRow` + `assignee` (kind and label, never other people's names for restricted instances), `step` (type, label, form fields, reject rules), `subject` (`kind`, `id`, `summary` from the subject), `timeline[]` (last 20 events), `affordances` (`claim`, `release`, `approve`, `reject`, `sign`, `submit_form`, `review` booleans with a reason code when false) |
| `SubmitFormRequest` | `form` (object) |
| `ReviewRequest` | `comment?` |
| `DefinitionRow`, `DefinitionDetail` | family, versions (state, changelog, published_at, CR id), the draft DSL with layout merged back |
| `CreateDefinition` | `dsl` (full document incl. optional `layout`), `resource_kinds[]` |
| `ReplaceDraft` | `dsl` |
| `PublishDefinition` | `changelog` (`{ar, en}`), `accept_warnings?`, `reason_code`, `reason_text` |
| `TestRunRequest` | `subject` (`{kind, attrs}` synthetic), `actors[]` (`{label, roles[], pools[]}`), `script?` (`[{step, actor, action, comment?}]`; absent = first eligible actor approves every step) |
| `TestRunTrace` | `steps[]` (`{seq, step, event, eligible_actors[], excluded_actors[], chosen_actor?, outcome}`), `effects[]` (would run, not run), `ended_as`, `issues[]` |
| `ValidationReport`, `ValidationIssue` | re-exported from `dsl` as `WfValidationReport`, `WfValidationIssue` |
| `CreateInventoryItem`, `UpdateInventoryItem`, `InventoryItemView` | section 3.9 |

- **Route table** (32 operations with the evidence route of section 7.2; scope = API.md s.2; Cedar actions in section 3.10; every mutation takes
  `Idempotency-Key` except the two marked exempt):

| Method | Path | Scope | Description |
|-|-|-|-|
| GET | `/dsl/{kind}/schema` | config.author, admin.system | JSON Schema of `change_request` or `workflow`; other kinds 404 until their phase (mapping 10, rating_model 15, scenario 16) |
| GET | `/change-requests` | config.author, config.approve, admin.users, admin.policies, admin.system, security.ops, audit.read | Change requests (cursor; filters `kind`, `class`, `status`, `mine`, `awaiting_me`, `emergency`, `review_pending`) |
| POST | `/change-requests` | config.author | Create a draft (or create and submit with `submit: true`) for a kind registered for generic creation; 201 |
| GET | `/change-requests/{id}` | as the list | Detail with diff, evidence, impact, route, tasks and timeline |
| POST | `/change-requests/{id}/submit` | config.author, admin.users, admin.policies, admin.system | Submit a draft: class, route and instance; `If-Match` (request version) |
| POST | `/change-requests/{id}/approve` | self | Approve on the caller's task in the route; **SU** (`change.approve`, `approval`); **4E**; `If-Match` (instance version) |
| POST | `/change-requests/{id}/reject` | self | Reject with a comment on the caller's task; **SU** (`change.reject`, `approval`); `If-Match` (instance version) |
| POST | `/change-requests/{id}/withdraw` | self | The maker withdraws before a decision; `If-Match` (request version) |
| GET | `/inventory` | model.risk, audit.read | Inventory items (cursor; filters `kind`, `tier`, `status`, `due_before`) |
| POST | `/inventory` | model.risk | Register an item before use; 201 |
| PATCH | `/inventory/{id}` | model.risk | Owner, developer, validator, committee, tier, status, validation dates; **SU** when the status becomes `approved_for_use` (`inventory.update`, `approval`); `If-Match` |
| GET | `/inbox` | self | My tasks: `view=mine` (claimed by me or open to me), `pool`, `waiting` (requests I made that wait on others), `history` |
| GET | `/inbox/counts` | self | Badge counts after the visibility filter |
| GET | `/tasks/{id}` | self | Task with the subject summary, timeline slice and affordances; 404 when restricted and not visible |
| POST | `/tasks/{id}/claim` | self | Claim an open task; `If-Match` (task version) |
| POST | `/tasks/{id}/release` | self | Release my claim; `If-Match` (task version) |
| POST | `/tasks/{id}/actions/approve` | self | Approve; **SU** on deciding steps (`task.approve`, `approval`); **4E**; `If-Match` (instance version) |
| POST | `/tasks/{id}/actions/reject` | self | Reject with a comment; **SU** (`task.reject`, `approval`); `If-Match` (instance version) |
| POST | `/tasks/{id}/actions/sign` | self | Sign; **SU** always (`task.sign`, `signing`); **4E**; `If-Match` (instance version) |
| POST | `/tasks/{id}/actions/submit-form` | self | Submit a form step; `If-Match` (instance version) |
| POST | `/tasks/{id}/actions/review` | self | Record a review on a non-blocking `review` step; `If-Match` (task version) |
| POST | `/admin/tasks/{id}/reassign` | admin.system | Propose a reassignment of a stuck or parked task; **CR** (`task_assignment`); **SU** (`task.reassign`, `access_change`); 202 |
| GET | `/admin/workflow-definitions` | admin.system | Families with their versions |
| GET | `/admin/workflow-definitions/{id}` | admin.system | Family detail with every version's DSL (layout merged) and validation report |
| POST | `/admin/workflow-definitions` | admin.system | Create a family and its draft v1 from a DSL body; 201 |
| PUT | `/admin/workflow-definitions/{id}/draft` | admin.system | Replace the draft; `If-Match` (draft row version) |
| POST | `/admin/workflow-definitions/{id}/validate` | admin.system | Validate the draft (DBS catalogue plus AKR codes); exempt from `Idempotency-Key` |
| POST | `/admin/workflow-definitions/{id}/test-run` | admin.system | Dry run with synthetic actors; nothing written but the audit row; exempt from `Idempotency-Key` |
| POST | `/admin/workflow-definitions/{id}/publish` | admin.system | Propose publication of the draft; **CR** (`workflow_definition`, ADM proposes, MLRO approves); 202 |
| PUT | `/admin/policies` | admin.policies | Propose a bundle (a staged release or an authored one); **CR**; the bundle runs in shadow while the request is open; `If-Match` (active version); 202 |
| POST | `/admin/users/{id}/grants` | admin.users | Propose a role grant; **CR** (`cr_access`); 202 |

`PUT /admin/settings/{key}` (phase 03) changes behaviour: a governed key now answers 202 with the change request
instead of 409. Every 202 carries `Location: /api/v1/change-requests/{id}` and the `ChangeRequestView`.

- **Routers:** `api::routes::change_control::router()` (8 + the three phase 03 CR branches; 9 with the evidence route of section 7.2),
  `api::routes::inbox::router()` (10), `api::routes::workflow_admin::router()` (8 incl. reassign),
  `api::routes::inventory::router()` (3); each built with `OpenApiRouter` and `authz_route!` (phase 03) so every
  operation carries `x-akr-action`.

## 3. Business Logic

### 3.1 The change-request model

- **Statuses** (DSL.md D.4, extended): `draft` -> `submitted` -> `approved` -> `effective`; `submitted` ->
  `rejected` | `withdrawn`; `draft` -> `withdrawn`; `approved` -> `failed` (stale base or a target refusal at a
  scheduled activation); `effective` -> `reverted` (emergency review, DR-55). `rejected`, `withdrawn`, `failed` and
  `reverted` are final; a rollback of an effective change is a new change request (DSL.md s.6).
- **Class** (`ChangeRequestService::classify`, never set by the maker):
  1. `base` = the target's `Prepared.base_class` (DSL.md D.3 table; for a setting: `minor`, or `material` when the key
     carries a floor).
  2. `volume_effect = decrease` -> at least `material` (any loosening is material).
  3. `reason_tag = capacity` -> at least `material` (S-39); the capacity rule also needs a `btl_sample` evidence item
     for kinds whose `required_evidence` lists it (`E_CAPACITY_NO_BTL`).
  4. `class_requested` higher than the result -> raised to it; lower -> 422 `E_VALIDATION` (`E_CLASS_LOWERED`).
  5. `emergency = true` -> `emergency`, only when the target allows it (`E_VALIDATION`, `emergency_not_allowed`), the
     reason code is `CHG_REGULATORY_ORDER` or `CHG_SANCTIONS_LIST` or `CHG_SECURITY`, and an `evidence_object` (the
     order or the list reference) is attached (`E_EVIDENCE_MISSING`); `effective_at` must be null.
- **Volume effect** comes from the target: neutral for every phase 04 kind; from the attached replay for scenario and
  parameter kinds (phase 16); `unknown` when a kind needs a replay and none is attached, and an `unknown` change
  cannot be approved as material (`E_EVIDENCE_MISSING` at submit).
- **Route** (`RouteSelector::select(cr, prepared) -> route key`): the rules of `change.route_map` in order; a rule
  matches when every key it names matches (`object_kind` list, `class`, `maker_role`, `key_prefix` against
  `route_attrs.key_prefix`, `validated` against `Prepared.validated_route`); first match wins. The chosen family must
  be published, active and list `change_request` in `resource_kinds` -> else 409 `E_NOT_ALLOWED_IN_STATE`
  (`route_unavailable`). The route is fixed at submit and pinned through the instance's definition version.
- **Evidence**: each item resolves through the `EvidenceResolver` registry (`evidence_object` phase 02,
  `shadow_report` this phase, `validation_run` and `test_run` this phase for workflow definitions, the rest from
  their phases); an unresolvable item -> 422 `E_VALIDATION` (`evidence_unknown`). At every deciding action the items
  must be younger than `change.evidence_max_age_days` -> else 422 `E_EVIDENCE_STALE` (the check runs inside the
  engine envelope, so a stale request cannot be approved even by its last approver).
- **Effective time**: `effective_at` null -> the change activates in the transaction of the final approval; a future
  time -> a `cr_activate` timer; a past time on a material change -> 422 `E_VALIDATION` (`E_BACKDATED`); a past time
  on a minor change is treated as null.

### 3.2 `ChangeRequestService`

```rust
pub struct ChangeRequestService { repo, reasons, targets: Arc<ChangeTargetRegistry>, routes: RouteSelector,
    engine: Arc<WorkflowEngine>, evidence: Arc<EvidenceRegistry>, timers: TimerService, settings: SettingsService,
    audit: Arc<dyn AuditSink>, clock: Arc<dyn BusinessClock> }
impl ChangeRequestService {
    pub async fn create(&self, ctx: &AuthContext, req: CreateChangeRequest, origin: Origin) -> Result<ChangeRequestView, ChangeError>;
    pub async fn submit(&self, ctx: &AuthContext, id: Uuid, if_match: i32) -> Result<ChangeRequestView, ChangeError>;
    pub async fn decide(&self, ctx: &AuthContext, id: Uuid, d: Decision, if_match: i32, a: StepUpAssertionId) -> Result<ChangeRequestView, ChangeError>;
    pub async fn withdraw(&self, ctx: &AuthContext, id: Uuid, if_match: i32) -> Result<ChangeRequestView, ChangeError>;
    pub async fn activate(&self, tx: &mut Tx, id: Uuid, actor: ActorRef) -> Result<(), ChangeError>;      // effect + timer
    pub async fn emergency_review_due(&self, id: Uuid) -> Result<(), ChangeError>;                     // timer
    pub async fn start_post_review(&self, id: Uuid, day: i32) -> Result<(), ChangeError>;              // timer
    pub async fn expire_drafts(&self, now: DateTime<Utc>) -> Result<u32, ChangeError>;                 // sweep
}
```

**`create(ctx, req, origin)`** (`origin` = `Generic` for `POST /change-requests`, or the dedicated route):
1. Target = `targets.get(req.object_kind)` -> unknown -> 422 `E_VALIDATION` (`unknown_object_kind`); `Generic` on a
   kind without `generic_create` -> 422 (`use_dedicated_route`).
2. Body shape against `change-request.v1.json`; reason code active, of a family the target accepts, and listing the
   kind in `object_kinds` when that is set -> else 422 (`reason_not_allowed`).
3. `prepared = target.prepare(ctx, body)` (validation, normalisation, base version, impact, warnings).
4. Class, volume effect (section 3.1); evidence resolution; warnings present without `accept_warnings` at submit ->
   422 listing them (warnings may be saved on a draft).
5. One transaction: `INSERT` with `maker_id = ctx.user`, `maker_role = ctx.decision.actor_role`, `reason_tag` from the
   code; the unique index refuses a second open request on the same `object_key` -> 409 `E_NOT_ALLOWED_IN_STATE`
   (`open_request_exists`, with the open request's id when the caller may read it); audit `change_request.created
   {object_kind, object_key, class, volume_effect, reason_code, emergency}`.
6. Dedicated routes and `submit: true` continue with `submit` in the same transaction.

**`submit(ctx, id, if_match)`**:
1. Lock the request; maker only (Cedar `change_request.submit`), status `draft`, version equals `If-Match`.
2. Re-run steps 2 to 4 of `create` against current state (a draft may sit for days); stale base -> 409 `E_STALE_BASE`.
3. Route (section 3.1); the target's `inventory_kind` set and no `approved_for_use` inventory record for the object
   -> 422 `E_VALIDATION` (`E_INVENTORY_MISSING`, GV-01).
4. `engine.start(tx, StartSpec { definition_key: route, resource: ("change_request", id), purpose: Decision,
   initiator: ctx.user, maker: ctx.user })` (section 3.4); `set_submitted`; `target.on_submitted` (the policy bundle
   turns shadow on); audit `change_request.submitted {route, definition_version, class}`.

**`decide`** is a convenience over the engine: find the caller's actionable task on the request's running instance
(eligible and not excluded, section 3.5) -> none -> 403 `E_FORBIDDEN` (or `E_SELF_APPROVAL` when the caller is the
maker or an earlier decider), more than one (a person eligible for two parallel branches) -> the first by step order
(the second refuses them anyway, unique index); then `engine.act(task, Approve | Reject, if_match, assertion)`.
Inside the engine envelope, before the action applies, the change subject runs **`check_before_decision`**: evidence
freshness, the target's re-check (SoD for grants, shadow age for bundles, base version for all) -> failures are
returned as 409 `E_STALE_BASE`, 422 `E_EVIDENCE_STALE` or 422 `E_VALIDATION`, and the task stays open.

**Outcome handling** (the change subject's `on_ended` and the effects; all inside the engine transaction):
- Final approval reaches the terminal `activate_change` effect: `set_decided(approved, checker = the deciding person,
  stepup)`; `effective_at` null or past -> `activate`; future -> timer `cr_activate` at `effective_at` (owner the
  request).
- Reject (mode `end`) -> `set_decided(rejected)`; `target.on_closed(Rejected)`; audit `change_request.rejected
  {step, comment_hash}`.
- Withdraw -> `engine.withdraw(instance)` (tasks cancelled, instance `withdrawn`); `set_withdrawn`;
  `target.on_closed(Withdrawn)`; audit `change_request.withdrawn`.
- Instance `failed` (DBS failure rule) -> request `failed` (`failure_reason = workflow_failed`), target closed.

**`activate(tx, id, actor)`** (atomic activation, DSL.md D.4; the DBS bundle-activation pattern):
1. `get_for_update`; status `approved` -> else return (the timer re-check rule of phase 00: do nothing when the owner
   no longer calls for it).
2. `target.current_version` differs from `base_version` -> `set_failed(stale_base)`, `target.on_closed(Failed)`,
   audit `change_request.failed {reason}`; return.
3. `target.activate(tx, cr, actor)` -> writes the new object version and the target's own audit event (`setting.updated
   {source: change_request}`, `role_grant.created`, `policy_bundle.activated {activation: change_request}`,
   `workflow_definition.published`, `task.reassigned`).
4. `set_effective(activated_at = now())`; outbox `change.effective {change_request_id, object_kind, object_key}`
   (phase 00 outbox, consumed by caches and from phase 17 by the alert engine's version pinning); audit
   `change_request.effective`.
5. Material: timers `cr_post_review` at each day of `change.post_review_days` after activation (S-46, GV-26).
   Emergency: `review_due_at` = now + `change.emergency_review_h` hours when `volume_effect` is `decrease` or
   `unknown`, else `clock.add_working(now, change.emergency_review_working_days)`; `review_outcome = pending`; timer
   `cr_emergency_review` at `review_due_at`; the `open_review` effect starts the review instance (section 3.3 below,
   DR-55).

**Emergency review** (DR-55):
- `open_review` starts a second instance on the same request with `purpose = emergency_review`, on the family the
  route map gives for `class = material` and the request's kind (as if the change had been material); its maker is the
  request's maker and the emergency approver is added to the excluded set, so the review is two further people.
- Review approved -> `set_review(confirmed)`, the timer cancelled, post-change review timers scheduled; audit
  `change_request.review_confirmed`.
- Review rejected -> `target.revert` (restores the base version, writing its own audit event), `set_reverted`,
  `set_review(rejected)`; audit `change_request.reverted {cause: review_rejected}`.
- `timer_cr_emergency_review` while still `pending`: a loosening (`decrease`) or `unknown` change -> `target.revert`,
  review instance withdrawn, `set_review(reverted)`, `set_reverted`; audit `change_request.reverted {cause:
  review_overdue}` (SP-22). A tightening or neutral change is never reverted by the clock (reverting a sanctions
  addition would loosen a control): `set_review(escalated)`, the review's open tasks get `escalated_at` and a task for
  CH is added to the review instance (`task_escalated` event); audit `change_request.review_escalated`.

**Post-change review** (`timer_cr_post_review`): start an instance of `cr_post_review` with `purpose = post_review`
(a `form` step for `subject_party: approvers` asking `outcome` = `effective` | `concern` | `rollback_needed` and
`notes`, then the `record_post_review` effect) unless the request was reverted; audit `change_request.post_review_started
{day}`. `record_post_review` writes the event `post_review_recorded {day, outcome}` on the request's timeline and the
audit `change_request.post_review_recorded`; `concern` and `rollback_needed` feed the findings register in phase 24
(until then the governance list filter `review_concern` shows them).

**Drafts**: `change_sweep` withdraws drafts older than `change.draft_ttl_days` (`withdrawn`, actor the system
principal, audit `change_request.withdrawn {cause: draft_expired}`).

### 3.3 The phase 04 change targets

| Kind | Route (route map default) | `object_key` | Base version | Prepare | Activate | Revert | Closed |
|-|-|-|-|-|-|-|-|
| `role_grant` | `cr_access` (maker ADM, checker SEC) | `user:<id>:role:<code>` | none (new grant) | user exists, active, human or service principal; role one of the 19 and not `VS` (support sessions, phase 25: `role_not_grantable`); scope branches and pools exist and are not archived; `AS` needs `valid_to` within `access.assessor_max_days`; no identical active grant (409 `duplicate_grant`); `SodRules::check_roles(effective roles ∪ {role})` (DR-50) -> 422 `sod_conflict` | `pg_advisory_xact_lock` on the user, SoD re-checked against committed state, insert `role_grants` (`source = change_request`, `change_request_id`, `granted_by` = the checker), `invalidate_user`; audit `role_grant.created {role, scope, valid_to, change_request_id}` | not allowed | nothing to clean |
| `setting` | by key prefix and class | `setting:<key>` | `settings.version` | key known and governed (non-governed keys stay direct, phase 03); value type, registry bounds, floor (`E_BELOW_FLOOR`); base class `material` when the key has a floor, else `minor`; `route_attrs.key_prefix` = the key's first segment with its dot | `SettingsService::apply(tx, key, value, base_version, actor, ChangeSource::ChangeRequest(id))` | restore the before value through `apply` (`setting.updated {source: revert}`) | nothing |
| `policy_bundle` | `cr_access` (or `cr_access_adm` when the maker acted as SEC) | `policy:active` (one bundle change at a time) | active bundle version | `staged_version` exists and is `staged`; or the authored bundle passes `loader::load` (strict validation, SoD pair test, route coverage both ways) and is inserted `staged`, `source = authored` (audit `policy_bundle.authored`); class `material`; `required_evidence = [shadow_report]` (generated, section 3.10) | `PolicyBundleRepository::activate(tx, id, actor, Activation::ChangeRequest, Some(cr))` (phase 03: supersedes, clears shadow, `NOTIFY akr_policy`) | not allowed | `set_shadow(off)`; `refuse(tx, id, change_rejected \| change_withdrawn \| change_failed)` |
| `workflow_definition` | `cr_workflow` (ADM proposes, MLRO approves) | `workflow:<definition_id>` | published version number (0 when none) | a draft exists; publish-mode validation has no errors (warnings need `accept_warnings`); `changelog` both languages; class `material`; impact = count of running instances on the published version (they stay pinned); evidence `validation_run` generated | `WorkflowDefinitionRepository::publish(tx, draft, changelog, change_request, cr, actor)` | not allowed | nothing (the draft stays editable) |
| `task_assignment` | `cr_access` | `task:<id>` | task version | task `open`, `claimed` or `parked`; its instance running; not a task of a `task_assignment` request (`self_reference`); class `minor`; target user eligible for the task now (`Holders`), not excluded, or the task's pool, or `queue` (release to its role or pool) | under the instance lock: re-point (`claimed` by the target with `task_reassigned {cause: admin}`) or release or unpark; version bump | not allowed | nothing |
| `_dev_object` (dev builds) | by class | `dev:<name>` | its version | body `{name, value, class, volume_effect}` so tests can drive every class and route | writes the dev table row | restores the value | nothing |

`PUT /admin/settings/{key}`, `POST /admin/users/{id}/grants`, `PUT /admin/policies`, `POST
/admin/workflow-definitions/{id}/publish` and `POST /admin/tasks/{id}/reassign` call `create(origin = Dedicated)` with
`submit = true` and answer 202. The phase 03 direct path stays for non-governed settings (200).

**Policy bundle in shadow** (SP-25, DR-58): `on_submitted` calls `set_shadow(on)`; one shadow bundle at a time is
already enforced by the phase 03 unique index, and `object_key = policy:active` keeps one open bundle request.
`check_before_decision` refuses approval while the bundle has been in shadow for less than `change.policy_shadow_min_h`
(422 `E_VALIDATION`, `shadow_too_short`, with the time left) and attaches the `shadow_report` evidence: counts of
`authz.shadow_diff` rows for the bundle by action and effect pair since `on_submitted`, generated at decision time so
it is never stale.

### 3.4 The workflow engine (DBS port)

The DBS engine (DBS docs/plan/phase-06.md sections 3.1 to 3.8, read-only) is ported with its invariants unchanged:
**I1** per-instance `SELECT ... FOR UPDATE` before any task, event or projection write; **I2** `If-Match` on the
instance version for step actions and on the task version for claim, release and review, inside the lock; **I3** one
winner in a claim race; **I4** two-layer idempotency (middleware replay, then the crash-window repair from the
`idempotency_key` recorded on `action_taken`); **I5** join evaluation under the lock; **I7** and **I8** the task and
instance state machines, with the AKR states below. The common mutation envelope (DBS 3.4, twelve steps) and the
pure `TaskProjector` used by both live writes and `rebuild_projection` are taken as they are. Events written: DBS
`started`, `step_entered`, `task_created`, `task_claimed`, `task_released`, `action_taken`, `step_completed`,
`branch_joined`, `rework_returned`, `version_invalidated`, `terminal_reached`, `task_cancelled`, `task_parked`,
`task_unparked`, `task_reassigned`, `instance_failed` (DBS decision 61: a state without an event cannot be rebuilt), plus
AKR `instance_withdrawn`, `task_reminded`, `task_escalated`, `review_recorded`, `post_review_recorded`, `effect_ran`, `effect_deferred`.

**AKR deltas** (DR-56):
1. **No start or recall routes.** Instances are started by domain services through `WorkflowEngine::start(tx,
   StartSpec)` inside their own transaction (change requests now, cases and reports later); there is no user catalogue
   of startable definitions. DBS recall becomes `withdraw(tx, instance, actor, reason)`, called by the subject's own
   withdraw (the maker withdrawing a request).
2. **Start** (`start`): resolve the family by key -> latest published version (none -> `route_unavailable`); the
   family's `resource_kinds` must contain the subject kind; the subject gives `SubjectContext` (attributes frozen into
   `context`, classification raised to the definition's `classification_default` if lower, maker, neutral title,
   branch); insert the instance, `started`, and enter the start step (a start `form` step is a task like any other:
   AKR instances are never started with form data).
3. **Step types:** `form`, `approve`, `sign`, `review`, `parallel`, `terminal`. `reply`, `attach`, `acknowledge` and
   `execute` are refused by the validator (DBS `E_UNSUPPORTED_STEP_TYPE`). `sign` = an approval that always needs step-up
   and records `content_sha256` (the subject's canonical summary at signing, phase 02 Canon) in `action_taken`. `review`
   follows the DBS 3.6b semantics (does not hold the token; no reject; `comment_required` defaults to false) and is
   recorded through `POST /tasks/{id}/actions/review` (event `review_recorded`, task `done`, outcome `reviewed`).
4. **Form fields:** `text`, `number`, `money`, `date`, `select` (DBS), plus `user` (an active user id), `branch` (an
   active branch id) and `evidence` (an evidence object the actor can read); DBS `file`, `employee` and `org_unit`
   are not used.
5. **Conditions** read `<prefix>.<attr>` from the frozen subject context (`cr.change_class`, `cr.volume_effect`,
   `cr.reason_tag`, `cr.object_kind`, `cr.maker_role`, `cr.emergency` for change requests) and `form.<step>.<field>`;
   the validator checks names and types against the resource kinds the family declares (`E_SUBJECT_ATTR_UNKNOWN`).
6. **Assignees** (DSL.md E.2): `role` (with `scope: bank | branch`; `branch` uses the subject's branch), `pool`,
   `named_officer`, `case_acl` (resolves from phase 18; refused at publish before then), `subject_party` (a party the
   subject declares: `maker`, `approvers` for change requests). DBS `initiator`, `position`, `org_unit`, `relative` and
   `user` are not used. A task with no current holder is created `parked` (DBS 3.10 semantics, the cockpit lists it,
   `change_sweep` unparks it when a holder appears).
7. **Claims:** an eligible person may act on an `open` task without claiming it first; the action claims and completes
   it in one transaction (`task_claimed {cause: act}` then `action_taken`). The DBS concern about silently stolen races
   does not arise: the action carries `If-Match` on the instance version, so a person acting on a stale page gets 409,
   and a claimed task held by someone else answers 409 `E_CLAIMED`. Claim and release stay for people who want to
   take a task off the shared list while they read the evidence.
8. **Reject** on AKR deciding steps defaults to `mode: end` with `comment_required: true`; `rework` is allowed (cases
   use it from phase 18) and increments `pass` on every task created after the `rework_returned` event.
9. **Terminal effects** run through the `TerminalEffect` registry in the terminal transaction (`effect_ran`
   events). A non-deferrable effect that is not registered cannot be published; `notify` is deferrable until phase 22
   and writes `effect_deferred`. An effect error aborts the whole action (the approval is rolled back and answered
   with the effect's error, for example `E_STALE_BASE`): unlike the DBS failure rule, an AKR approval whose effect
   cannot run did not happen.
10. **Timeouts** (DSL.md E.2): `remind`, `escalate { to: Assignee }` and `reassign { to: Assignee }`; DBS
    `notify_superior` is replaced by `escalate` (AKR has no reporting line), `auto_approve` and `auto_reject` are
    refused (`E_AUTO_DECISION`). Section 3.6.
11. **No watchers** (DBS `instance_watchers` is not ported): people follow a request through its subject, which is
    access-controlled; a watcher list on a restricted instance would be a list of who knows.
12. **Instance states:** `running`, `completed`, `rejected`, `withdrawn`, `failed`.

```rust
pub struct WorkflowEngine { defs, instances, events, tasks, projector: TaskProjector, holders: Arc<dyn Holders>,
    subjects: Arc<SubjectRegistry>, effects: Arc<EffectRegistry>, stepup: StepUpService, timers: TimerService,
    clock: Arc<dyn BusinessClock>, audit: Arc<dyn AuditSink> }
impl WorkflowEngine {
    pub async fn start(&self, tx: &mut Tx, spec: StartSpec) -> Result<WorkflowInstance, WorkflowError>;
    pub async fn act(&self, ctx: &AuthContext, task_id: Uuid, action: TaskAction, if_match: i32, idem: &str,
                     assertion: Option<StepUpAssertionId>) -> Result<ActionOutcome, WorkflowError>;
    //   TaskAction::{Approve{comment}, Reject{comment}, Sign{comment}, SubmitForm{form}, Review{comment}}
    pub async fn claim(&self, ctx: &AuthContext, task_id: Uuid, if_match: i32, idem: &str) -> Result<TaskDetail, WorkflowError>;
    pub async fn release(&self, ctx: &AuthContext, task_id: Uuid, if_match: i32, idem: &str) -> Result<TaskDetail, WorkflowError>;
    pub async fn withdraw(&self, tx: &mut Tx, instance_id: Uuid, actor: ActorRef, reason: &str) -> Result<(), WorkflowError>;
    pub async fn reassign(&self, tx: &mut Tx, task_id: Uuid, to: ReassignTarget, actor: ActorRef) -> Result<(), WorkflowError>;
    pub async fn on_timeout(&self, task_id: Uuid, timer_id: Uuid) -> Result<(), WorkflowError>;
    pub async fn sweep(&self, limit: u32) -> Result<SweepReport, WorkflowError>;          // section 3.6
    pub async fn rebuild_projection(&self, instance_id: Uuid, dry_run: bool) -> Result<RebuildReport, WorkflowError>;
}
```

**`act`** (envelope steps 1 to 12 of DBS 3.4, with these AKR checks between envelope step 8 and the action):
1. Task `parked` -> 409 `E_NOT_ALLOWED_IN_STATE` (`task_parked`); `done` or `cancelled` -> the I4 repair check, then
   409 (`task_not_actionable`); instance not `running` -> 409 (`instance_not_running`).
2. Step-type gate (approve on an approve task, sign on a sign task, ...) -> 422 `E_VALIDATION` (`E_ACTION_MISMATCH`).
3. Cedar (`task.approve`, `task.reject`, `task.sign`, `task.submit_form`, `task.review`) with the Task adapter built
   live under the lock from `Holders` (section 3.5). A refusal on a restricted instance the caller cannot see -> the
   masked 404.
4. Four-eyes (section 3.5): the caller or the delegator they act for is the maker or an earlier decider of this pass
   -> 403 `E_SELF_APPROVAL`.
5. Deciding steps and every `sign`: `StepUpService::consume(tx, action, class, resource_ref = "task:<id>")` -> 401
   `E_STEPUP_REQUIRED`; the assertion id goes to `tasks.stepup_id`.
6. Subject check before decision (`check_before_decision` for change requests; other subjects register theirs).
7. Apply (DBS 3.5 and 3.6: events, projection, advance, join, terminal effects), audit (section 3.10), commit.

**`rebuild_projection`** (DBS 3.8): `akr-ctl workflow rebuild-instance <id> [--dry-run]`; folds the events, reports
divergences, deletes and re-inserts the instance's task rows under the lock (the one sanctioned delete), audit
`workflow_instance.projection_rebuilt {divergences}`. Denormalised titles and labels are folded from event payloads,
never re-derived (DBS 8.8).

### 3.5 Eligibility and four-eyes

`Holders::eligible(task, principal)` (one implementation, used by the inbox query builder, the Task adapter and the
engine):
1. `role`: the role is among the principal's effective roles (direct or delegated, phase 01) and, for `scope:
   branch`, the role's grant covers `task.branch_id`; a role held only by delegation sets `acting_for` = the delegator.
2. `pool`: a grant scope of the principal lists the pool (phase 03: pool members come from grant scopes).
3. `named_officer`: the principal is in `NamedOfficers::members(officer)` (`mlro` = active holders of `M`).
4. `subject_party`: the principal is in `WorkflowSubject::party_members(resource, party)`.
5. `case_acl`: phase 18 (refused at publish until then).
6. `excluded` = the principal or `acting_for` is the instance maker, or completed (in person or as delegator) a
   deciding task of the same instance and pass, or, on an emergency review instance, made the emergency approval.

Four-eyes is enforced three times (DR-57): Cedar `forbid` with `@deny("self_approval")` on `resource.excluded` (the
adapter's set: maker plus earlier deciders), the engine's delegation check (Cedar sees the principal, not whom they
act for), and the database (`tasks` CHECKs on `completed_by` and `acting_for` against `maker_id`, the two unique
indexes on `(instance_id, pass, completed_by)` and `(instance_id, pass, decider)`, and `change_requests.checker_id <>
maker_id`). A bundle test asserts that no shipped definition can be completed by fewer than two people, and the
validator refuses a deciding step that tries to switch the exclusion off (`E_FOUR_EYES_OFF`).

The role-level SoD pair `["change_request.create", "change_request.decide"]` that phase 03 announced is **not**
added (DR-54): `MRO` holds `config.author` and `config.approve` by design, and `ADM` both proposes access changes and
approves `cr_access_adm`; a role-level pair would refuse every such role. Separation on change requests is
per-request, by the three layers above.

### 3.6 Timers, timeouts and the sweep

**Timer kinds** (phase 00 `admin.timers`; each maps to the job `timer_<kind>`; every handler re-reads its owner first
and does nothing when the owner no longer calls for it, the phase 00 rule):

| Timer kind | Owner | Fires at | Re-check (else no-op) | Action |
|-|-|-|-|-|
| `cr_activate` | change_request | `effective_at` | status `approved` | `ChangeRequestService::activate` in one transaction, actor the system principal; a stale base or a target refusal -> `failed` (section 3.2) |
| `cr_emergency_review` | change_request | `review_due_at` | status `effective`, `review_outcome = pending` | revert or escalate by volume effect (section 3.2, DR-55) |
| `cr_post_review` | change_request | `activated_at` + each day of `change.post_review_days`; payload `{day}` | status `effective`; no `post_review` instance yet for that day (`context.day`) | start `cr_post_review` (section 3.2) |
| `task_timeout` | task | `tasks.due_at`; payload `{action, round}` | task `open`, `claimed` or `parked`; `tasks.timer_id` equals this timer | the step's timeout action, below |

**Step timeouts** (DSL.md E.2, AKR delta 10): `"timeout": { "after": Duration, "action": { "kind": ..., ... } }`.
- `due_at` is set when the task is created: `BusinessClock::add_working(created_at, after.value, after.unit)` for
  `business_days`, plain addition for `hours` and `days`. Until phase 09 the clock is `CalendarDaysClock`, so a
  timeout in business days fires earlier than it will with the bank calendar, never later. The timer is scheduled in
  the same transaction and its id written to `tasks.timer_id`.
- `remind` (`{ "kind": "remind", "every": Duration? }`): sets `reminded_at`, event `task_reminded {round}`; with
  `every`, a new timer for the next round. The notice itself arrives with phase 22 (`notify` is deferrable); until then
  the inbox shows the task as overdue.
- `escalate` (`{ "kind": "escalate", "to": Assignee }`): sets `escalated_at` and creates, on the same instance and
  step, a non-blocking `review` task for the `to` assignee (`deciding = false`, neutral title "Overdue approval");
  event `task_escalated {task_id, escalation_task_id, to}`. The escalation holder sees the overdue item in their inbox,
  can record a comment, and may act on the original task only if they are eligible for it under its own assignee;
  escalation never widens who may decide. The escalation task is cancelled when the step completes
  (`task_cancelled {cause: step_completed}`).
- `reassign` (`{ "kind": "reassign", "to": Assignee }`): the task's assignee columns are re-pointed to `to`, a claim is
  released (`task_released {cause: timeout}`), event `task_reassigned {cause: timeout, from, to}`; with no holder the
  task is parked.
- `auto_approve` and `auto_reject` are refused at publish (`E_AUTO_DECISION`).
- Every timeout runs under the instance lock (I1) and bumps the task and instance versions, so a person acting at
  the same moment gets 409 `E_VERSION_CONFLICT` and retries on the new state.

**`change_sweep`** (scheduled, `schedule.change_sweep`, hourly at minute 15; concurrency 1; at most 500 rows per step
per run, the rest next run):
1. Drafts older than `change.draft_ttl_days` -> withdrawn (section 3.2).
2. Claimed tasks whose holder is no longer eligible (grant revoked or expired, delegation ended, user disabled):
   claim released under the instance lock, event `task_released {cause: sweep}`, audit `task.released`.
3. Open tasks with no eligible, non-excluded holder (`Holders::holder_count = 0`) -> `parked`, event `task_parked`;
   parked tasks with a holder again -> `open`, event `task_unparked`. The cockpit (phase 03 A3) gains the tile "parked
   tasks" with their ages; `POST /admin/tasks/{id}/reassign` is the way out when no holder will appear.
4. Report `{drafts_expired, claims_released, parked, unparked}` in the job row's result.

`Holders::holder_count` counts people who are eligible **and not excluded**: a CHK task on `cr_material` whose only
CHK holder already approved as MLRO is parked rather than left open with nobody able to act.

**Offboarding steps** (the phase 03 `OffboardingStep` contract): `open_tasks` (order 70): every task claimed by the
user is released (`task_released {cause: offboarded}`) and, when no holder remains, parked; `change_drafts` (order
71): the user's drafts are withdrawn (`change_request.withdrawn {cause: offboarded}`). Submitted requests made by the
user stay: their checkers still decide them. Both return counts for `user.offboarded {steps}`.

### 3.7 The inbox

`InboxService { tasks, holders, subjects, authz }`, `list(ctx, InboxQuery, cursor, limit) -> Page<TaskRow>` and
`counts(ctx) -> InboxCounts`.

**Views:**
- `mine`: tasks claimed by the caller, and open tasks the caller is eligible for and not excluded from (role, pool,
  named officer, subject party, and escalation tasks addressed to them); parked tasks are not shown.
- `pool`: the open tasks of the pools the caller belongs to (a subset of `mine`, for pool workers).
- `waiting`: running instances whose maker is the caller, one row per open task, showing the step and the assignee
  kind and label (never the names of the people who hold it).
- `history`: tasks the caller completed, newest first.

**Query.** One SQL statement per view, built from the caller's effective roles (with branch scope), pools, named
offices and delegations (`acting_for`), using the inbox indexes of migration 0015; subject-party and visibility rules
come from the subjects' `party_sql` and `visibility_sql` predicates; the exclusion rule (section 3.5 step 6) is part
of the statement. Keyset order `(due_at NULLS LAST, created_at, id)` for `mine` and `pool`, `(completed_at DESC, id
DESC)` for `history`.

**Visibility and masking** (SP-31, SP-33):
- Rows of a `restricted` instance appear only when the subject's visibility predicate admits the caller; counts come
  from the same filtered statement, so a badge never counts what a list would hide (phase 03 rule).
- `GET /tasks/{id}`, the claim, release and action routes answer the masked 404 for a restricted task the caller cannot
  see, identical to a missing id (one body, the same work on both paths, the latency floor). The inference harness
  gains the kind `task_restricted` in `MASKED_KINDS` with probes for the surfaces this phase has (task get, inbox list,
  inbox counts, the five action routes, the error body, latency); its fixtures run on the dev subject `_dev_masked`
  (phase 03), registered as a `WorkflowSubject` in dev builds. Its other probes arrive with their surfaces (SSE 17,
  notifications 22, the audit viewer 24).
- Titles and step labels are neutral (`title` from `SubjectContext`: for a change request "Change request: <kind label>",
  never the proposed value); `TaskRow.resource_id` is present only when the caller can read the subject.
- `waiting` never names the holders of a task; `TaskDetail.assignee` shows the role, pool or office label.

**Live updates:** none in this phase. The client polls `GET /inbox/counts` every 60 seconds; SSE arrives with phase
17. **Budget:** p95 300 ms for `GET /inbox` and 100 ms for `/inbox/counts` with 10,000 open tasks and a principal
holding five roles (`cargo bench` fixture, section 6).

### 3.8 Definition authoring, validation and the shipped definitions

```rust
pub struct DefinitionService { repo: Arc<dyn WorkflowDefinitionRepository>, validator: DslValidator,
    stepper: dsl::workflow::Stepper, subjects: Arc<SubjectRegistry>, effects: Arc<EffectRegistry>,
    changes: Arc<ChangeRequestService>, audit: Arc<dyn AuditSink> }
impl DefinitionService {
    pub async fn create(&self, ctx: &AuthContext, req: CreateDefinition) -> Result<DefinitionDetail, WorkflowError>;
    pub async fn replace_draft(&self, ctx: &AuthContext, id: Uuid, req: ReplaceDraft, if_match: i32) -> Result<DefinitionDetail, WorkflowError>;
    pub async fn validate(&self, ctx: &AuthContext, id: Uuid) -> Result<WfValidationReport, WorkflowError>;
    pub async fn test_run(&self, ctx: &AuthContext, id: Uuid, req: TestRunRequest) -> Result<TestRunTrace, WorkflowError>;
    pub async fn propose_publish(&self, ctx: &AuthContext, id: Uuid, req: PublishDefinition) -> Result<ChangeRequestView, ChangeError>;
    pub async fn seed(&self, dir: &Path, offer_updates: bool) -> Result<SeedReport, WorkflowError>;   // akr-ctl
}
```

**`create`**: 1. `key` matches the column rule, is free (422 `E_VALIDATION`, `taken`), and starts with `_` only in dev
builds. 2. Every `resource_kinds` entry is a registered `WorkflowSubject` (422 `unknown_resource_kind`). 3. Split the
layout out of the document (DBS), canonical SHA-256 of the semantic part (phase 02 `Canon`), draft-mode validation
(any mix of issues may be saved). 4. One transaction: family (`shipped = false`) and version 1 `draft`; audit
`workflow_definition.created {key, resource_kinds, sha256}`.

**`replace_draft`**: `If-Match` is the draft's `row_version`, or `0` to open a new draft (version = highest + 1) when
none exists; a draft under an open `workflow_definition` change request cannot change (409 `E_NOT_ALLOWED_IN_STATE`,
`open_request_exists`: the checker approves what they saw); same steps 3 and 4; audit
`workflow_definition.draft_updated {version, sha256, errors, warnings}`.

**`validate`**: publish-mode validation of the draft; returns the report and changes nothing (exempt from
`Idempotency-Key`, no audit).

**`test_run`** (DBS has no dry run; AKR adds it so a checker sees how a definition routes before approving it):
1. The draft (or, with `?version=n`, a published version) is loaded; the synthetic subject's attributes are checked
   against the declared resource kind's `attributes()`.
2. The pure `Stepper` (the same code the engine uses to advance) walks from `start`. At each task: eligible synthetic
   actors by the `Holders` rules applied to the synthetic roles and pools (named officers: actors with role `M`;
   subject parties: the synthetic maker and earlier deciders), minus the excluded ones (maker, earlier deciders of the
   pass); the script's actor for that step or else the first eligible one acts; no eligible actor -> the trace records
   `parked` and stops.
3. Terminal effects are listed as `would_run`; nothing is written except the audit row
   `workflow_definition.test_run {version, sha256, ended_as, steps}`. That audit row is the `test_run` evidence a
   publish request may cite. A guard of 200 steps stops a rework loop the script never leaves.

**`propose_publish`**: draft exists; publish-mode validation has no errors and warnings are accepted; `changelog` in
both languages; then `ChangeRequestService::create(origin = Dedicated, object_kind = workflow_definition, body =
{definition_id, draft_version_id, sha256, changelog, accept_warnings})` with `submit = true` and the generated
`validation_run` evidence (the report, its time and the draft hash). At activation the target re-checks that the
draft hash still equals the proposed `sha256` (else `E_STALE_BASE`).

**Validator** (`dsl::workflow`, pure; schema pass then semantic pass; DBS pipeline and modes): the whole DBS catalogue
(`E_SCHEMA`, `E_NO_START`, `E_BAD_START`, `E_CYCLE`, `E_DANGLING_REF`, `E_INACTIVE_REF`, `E_TYPE_MISMATCH`,
`E_NO_ELSE`, `E_ORPHAN_JOIN`, `E_MISSING_I18N`, `E_STEPUP_DISABLED`, `E_UNSUPPORTED_STEP_TYPE`, `W_UNREACHABLE`,
`W_NO_TIMEOUT`; `W_USER_ASSIGNEE` cannot arise, the `user` kind is refused by the schema). `E_DANGLING_REF` covers
AKR references: a role outside the 19 codes, a pool, an office not in `NamedOfficers::known()`; `E_INACTIVE_REF` an
archived pool. AKR codes (DSL.md s.7):

| Code | Severity | Check |
|-|-|-|
| `E_AUTO_DECISION` | error | A timeout action `auto_approve` or `auto_reject` |
| `E_FOUR_EYES_OFF` | error | A deciding step carrying any key that would narrow the exclusion (`exclude: false`, `allow_maker`, `allow_repeat_decider`) |
| `E_STEPUP_DISABLED` | error | (DBS rule widened) `"stepup": false` on any `approve` or `sign` step: deciding steps always need step-up |
| `E_TIPPING_OFF_TERM` | error | A label, title or option text of a definition that a branch-tier role can see contains a term from `TippingOffTerms` (the port is empty until phase 09 ships the bank's term list) |
| `E_ACL_ASSIGNEE_REQUIRED` | error | `classification_default: restricted` and a deciding step not assigned through `case_acl` |
| `E_ASSIGNEE_UNAVAILABLE` | error | An assignee kind the family's resource kinds cannot resolve: `case_acl` outside case and report subjects (and before phase 18), `subject_party` naming a party the subject does not declare |
| `E_EFFECT_UNAVAILABLE` | error | A terminal effect that is not registered and not deferrable, or that does not serve the family's resource kinds |
| `E_SUBJECT_ATTR_UNKNOWN` | error | A condition path `<prefix>.<attr>` the declared resource kinds do not provide, or of the wrong type |
| `W_ROLE_WITHOUT_HOLDERS` | warning | A step assigned to a role or office with no active holder today (its tasks would be parked) |

**Seed loader** (`akr-ctl workflow seed [--offer-updates]`, run by the install and upgrade procedure after `db
migrate`): for each file in `backend/workflows/shipped/`: the family is missing -> insert it (`shipped = true`,
`resource_kinds` from the file) with version 1 validated in publish mode and published (`published_via = seed`,
`published_by` = the system principal of phase 01, `changelog` from the file), audit `workflow_definition.seeded {key,
sha256}`; the family exists -> left alone and reported (`present`, or `modified` when the bank's published hash differs
from the shipped one). With `--offer-updates`, a newer shipped file becomes a draft of a family that has none, so the
bank publishes it by change request; a release never changes a bank's published definition by itself. A shipped
file that fails validation stops the seed with the report (a release defect).

**Shipped definitions** (`backend/workflows/shipped/`, `resource_kinds: ["change_request"]`, `classification_default:
"internal"`; every deciding step `stepup: true` and `reject: {comment_required: true, mode: "end"}`; "esc CH 3bd" =
`timeout {after: 3 business_days, action: escalate to role CH}`):

| Key | Steps | Timeouts | Terminal effects |
|-|-|-|-|
| `cr_minor` | `approve` role CHK | esc CH 3bd | `activate_change` |
| `cr_material` | `parallel` [`approve` named_officer `mlro`, `approve` role CHK] join all -> if `cr.volume_effect = decrease`: `approve` role CH | esc CH 3bd on both branches; CH step `remind` after 2 business days, every 1 | `activate_change` |
| `cr_material_validated` | `approve` role MRO -> then as `cr_material` | esc CH 3bd | `activate_change` |
| `cr_emergency` | `approve` named_officer `mlro` | `remind` after 2 hours, every 2 hours | `activate_change`, `open_review` |
| `cr_mapping` | `approve` role CHK | esc CH 3bd | `activate_change` |
| `cr_access` | `approve` role SEC | `remind` after 1 business day, every 1 | `activate_change` |
| `cr_access_adm` | `approve` role ADM | `remind` after 1 business day, every 1 | `activate_change` |
| `cr_workflow` | `approve` named_officer `mlro` | esc CH 3bd | `activate_change` |
| `cr_post_review` | `form` subject_party `approvers`: `outcome` (select `effective`, `concern`, `rollback_needed`), `notes` (text, required unless `effective`) | `remind` after 5 business days, every 5 | `record_post_review` |

Access changes never escalate to business roles (research file 30 s.7.4: administration and security stay apart from
the business); they remind the security officer or administrator instead. `cr_post_review` is not deciding: it records
an opinion and changes nothing. `cr_material` in full as shipped (DSL.md 9.7 is aligned to it in the same change; `resource_kinds` and `changelog`
are read by the seed loader and stripped before validation):

```json
{
  "dsl_version": "1",
  "key": "cr_material",
  "name": { "ar": "اعتماد تغيير جوهري", "en": "Material change approval" },
  "resource_kinds": ["change_request"],
  "classification_default": "internal",
  "start": "approvals",
  "steps": {
    "approvals": {
      "type": "parallel",
      "branches": ["mlro_approval", "second_approval"],
      "join": "all",
      "next": [ { "if": { "field": "cr.volume_effect", "op": "eq", "value": "decrease" }, "to": "compliance_approval" },
                { "else": true, "to": "activate" } ]
    },
    "mlro_approval": {
      "type": "approve",
      "label": { "ar": "موافقة مسؤول الإبلاغ", "en": "MLRO approval" },
      "assignee": { "kind": "named_officer", "officer": "mlro" },
      "stepup": true,
      "timeout": { "after": { "value": 3, "unit": "business_days" },
                   "action": { "kind": "escalate", "to": { "kind": "role", "role": "CH", "scope": "bank" } } },
      "reject": { "comment_required": true, "mode": "end" },
      "next": "@join"
    },
    "second_approval": {
      "type": "approve",
      "label": { "ar": "موافقة ثانية من خارج العمليات", "en": "Second approval outside operations" },
      "assignee": { "kind": "role", "role": "CHK", "scope": "bank" },
      "stepup": true,
      "timeout": { "after": { "value": 3, "unit": "business_days" },
                   "action": { "kind": "escalate", "to": { "kind": "role", "role": "CH", "scope": "bank" } } },
      "reject": { "comment_required": true, "mode": "end" },
      "next": "@join"
    },
    "compliance_approval": {
      "type": "approve",
      "label": { "ar": "موافقة رئيس الامتثال", "en": "Compliance head approval" },
      "assignee": { "kind": "role", "role": "CH", "scope": "bank" },
      "stepup": true,
      "timeout": { "after": { "value": 2, "unit": "business_days" },
                   "action": { "kind": "remind", "every": { "value": 1, "unit": "business_days" } } },
      "reject": { "comment_required": true, "mode": "end" },
      "next": "activate"
    },
    "activate": { "type": "terminal", "outcome": "approved", "effects": [ { "kind": "activate_change" } ] }
  },
  "changelog": { "ar": "الإصدار الأول", "en": "First release" }
}
```

The engine excludes the maker and earlier deciders of the pass from every deciding step, so a person holding both `M`
and `CHK` approves one branch at most, and a material change always has at least two approvers besides its maker
(three with the compliance head when it reduces alerts).

### 3.9 Inventory (GV-01, GV-04)

`InventoryService { repo, users, audit, stepup }`: `list(ctx, filter, cursor, limit)`, `create(ctx,
CreateInventoryItem)`, `update(ctx, id, UpdateInventoryItem, if_match, assertion?)`.
- **Create**: `kind`, `object_id`, `name`, `tier`, `tier_basis?`, `owner_id`, `developer`, `validator_id?`,
  `committee`, `user_roles[]`, `next_validation_due?`. Owner and validator are active human users; the validator is
  not the owner (422 `E_VALIDATION`, `validator_is_owner`; the CHECK backs it); `user_roles` are role codes; one item
  per `(kind, object_id)` (409 `E_NOT_ALLOWED_IN_STATE`, `already_registered`). Phase 04 has no object resolvers: the
  objects (scenarios, parameter sets, models, matchers, lists, LLM components, prompts) arrive from phase 14 on, so an
  item may be registered before its object exists (GV-01: registered before use), and each later phase registers a
  resolver that checks the id. Status starts `registered`. Audit `inventory_item.created {kind, object_id, tier,
  owner_id, validator_id}`.
- **Update** (`If-Match`): fields as `Option`s; status moves `registered` -> `in_validation` -> `approved_for_use` ->
  `restricted` | `retired`, `restricted` -> `approved_for_use` | `retired`, `retired` final (409 otherwise).
  `approved_for_use` needs a validator and `last_validated_at` (the CHECK), a step-up (`inventory.update`, `approval`),
  and is never set by the item's owner (403 `E_SELF_APPROVAL`, Cedar `inventory.no_owner_approval`). Audit
  `inventory_item.updated {fields, status_from, status_to, stepup_id?}`.
- **The GV-01 gate**: a target with `inventory_kind` refuses submission without an `approved_for_use` item for the
  object (section 3.2 step 3) and re-checks at activation; `restricted` and `retired` items fail the gate. No phase 04
  target declares one; scenario, parameter-set, model and matcher kinds do from phases 14, 15 and 16.
- Next validation dates are entered by the model-risk officer here; phase 27 computes them from the tier and lists
  overdue items.

### 3.10 Errors, audit events, Cedar actions

**Error mapping.** No new top-level codes (API.md s.5). Used: `E_SCOPE`; `E_FORBIDDEN`; `E_SELF_APPROVAL` (maker,
earlier decider, delegate of either, emergency approver on its review, inventory owner); `E_NOT_FOUND` (missing or
masked); `E_CLAIMED` (task held by someone else); `E_STALE_ASSIGNEE` (the caller lost eligibility between page and
action); `E_STALE_BASE`; `E_VERSION_CONFLICT`; `E_PRECONDITION_REQUIRED`; `E_STEPUP_REQUIRED`; `E_EVIDENCE_STALE`;
`E_BELOW_FLOOR` (a setting change under its floor); `E_NOT_ALLOWED_IN_STATE` with reasons `open_request_exists`,
`route_unavailable`, `task_parked`, `task_not_actionable`, `instance_not_running`, `not_draft`, `already_decided`,
`already_registered`; `E_VALIDATION` with `errors[]` codes from DSL.md s.7 (`E_CLASS_LOWERED`, `E_EVIDENCE_MISSING`,
`E_CAPACITY_NO_BTL`, `E_BACKDATED`, `E_INVENTORY_MISSING`, `E_ACTION_MISMATCH`, `E_COMMENT_REQUIRED`, the validator
codes of section 3.8) and reasons `unknown_object_kind`, `use_dedicated_route`, `reason_not_allowed`,
`emergency_not_allowed`, `evidence_unknown`, `shadow_too_short`, `sod_conflict`, `duplicate_grant`,
`role_not_grantable`, `taken`, `unknown_resource_kind`, `validator_is_owner`, `self_reference`. `ChangeError` and
`WorkflowError` (`thiserror`) carry these and map in `api::error`.

**Audit events emitted** (catalogue additions; resource in brackets; all SIEM-forwarded from phase 25 except the task
events, which the timeline already shows):

| Event kind | On | Resource |
|-|-|-|
| `change_request.created` | request created (kind, key, class, volume effect, reason code, emergency) | change_request |
| `change_request.submitted` | submitted (route, definition version, class) | change_request |
| `change_request.rejected` | rejected (step, comment hash) | change_request |
| `change_request.withdrawn` | withdrawn (cause: maker, draft_expired, offboarded) | change_request |
| `change_request.effective` | activated (activated_at, by approval or timer) | change_request |
| `change_request.failed` | failed (reason: stale_base, target_refused, workflow_failed) | change_request |
| `change_request.reverted` | reverted (cause: review_rejected, review_overdue) | change_request |
| `change_request.review_confirmed`, `change_request.review_escalated` | emergency review outcome | change_request |
| `change_request.post_review_started`, `change_request.post_review_recorded` | post-change review (day, outcome) | change_request |
| `workflow_instance.started`, `.completed`, `.rejected`, `.withdrawn`, `.failed` | instance lifecycle (definition version, purpose) | workflow_instance |
| `workflow_instance.projection_rebuilt` | rebuild (divergences) | workflow_instance |
| `task.claimed`, `task.released` | claim and release (cause: user, act, sweep, timeout, offboarded) | task |
| `task.approved`, `task.rejected`, `task.signed` | deciding actions (step-up id, acting_for, content hash for `sign`) | task |
| `task.form_submitted`, `task.reviewed` | non-deciding actions (field names, never values of restricted forms) | task |
| `task.parked`, `task.unparked`, `task.reassigned`, `task.reminded`, `task.escalated`, `task.cancelled` | engine and sweep | task |
| `workflow_definition.created`, `.draft_updated`, `.test_run`, `.published`, `.seeded` | authoring (version, hash, issue counts) | workflow_definition |
| `inventory_item.created`, `inventory_item.updated` | section 3.9 | inventory_item |
| `policy_bundle.authored`, `policy_bundle.refused` | bundle authored for a request; refused when the request closes | policy_bundle |

Reused: `setting.updated` (now with `source: change_request | revert`), `role_grant.created`,
`policy_bundle.activated` (`activation: change_request`), `policy_bundle.shadow_changed` (the request turns shadow on
and off), `user.offboarded` (two more step counts). Every event carries the request's decision record (phase 03).

**Cedar** (bundle files `55_change_control.cedar` and `56_workflow.cedar`, in the gap the phase 03 number ranges leave
before 60; phase 03 section 3.1 is aligned). Entities added to `schema.cedarschema`:
- `ChangeKind { kind: String, family: String }` (the resource of create and propose; families in phase 04: `access`,
  `setting`, `workflow`, `dev`; `detection`, `mapping`, `kyc_model` and `reference` arrive later);
- `ChangeRequest { kind: String, family: String, maker: User, status: String, parties: Set<User> }` (`parties` =
  maker, deciders and the current holders of its open tasks);
- `Task { step_type: String, deciding: Bool, state: String, eligible: Bool, excluded: Bool, subject_visible: Bool,
  classification: String, claimed_by_other: Bool }` (built per request from `Holders` under the instance lock: the
  policy answers who, the engine answers when, DBS decision 60);
- `WorkflowDefinition { key: String, shipped: Bool }`, `InventoryItem { kind: String, owner: User, validator: User? }`.

| Action | Resource | Who (shipped bundle) |
|-|-|-|
| `dsl.schema` | System | CFG, DS, MRO, ADM, SEC, CHK, M, CH |
| `change_request.list` | System | ADM, SEC, IA, CHK, M, CH, CFG, DS, MRO (rows filtered by `change_request.read`) |
| `change_request.read` | ChangeRequest | the family audience (access: ADM, SEC, IA; setting: ADM, SEC, CHK, M, CH, IA; workflow: ADM, SEC, CHK, M, IA; dev: ADM, CHK, M, CH) or a member of `parties` |
| `change_request.read_logic` | ChangeRequest | `proposed` and `impact` in the view; phase 04 families: the read audience; `detection` (phase 10 on): CFG, CHK, M, CH, MRO, DS, IA |
| `change_request.create` | ChangeKind | `access`: ADM (and SEC for `policy_bundle`); `setting`: ADM; `workflow`: ADM; `dev`: ADM, CHK (dev bundle only) |
| `change_request.submit`, `change_request.withdraw` | ChangeRequest | `principal == resource.maker` and status `draft` (submit) or not decided (withdraw) |
| `role_grant.propose` | User | ADM; never on self |
| `policy.propose` | System | ADM, SEC |
| `setting.update` | Setting | ADM (unchanged; the governed branch now creates a request) |
| `task.reassign` | Task | ADM |
| `workflow_definition.list`, `.read`, `.create`, `.update_draft`, `.validate`, `.test_run`, `.publish` | System, WorkflowDefinition | ADM; `list` and `read` also SEC, M, IA |
| `inbox.read` | System | every active human |
| `task.read` | Task | `resource.eligible || resource.subject_visible` |
| `task.claim`, `task.release` | Task | `resource.eligible` (release: the holder, or ADM through `task.reassign`) |
| `task.approve`, `task.reject`, `task.sign`, `task.submit_form`, `task.review` | Task | `resource.eligible` |
| `inventory.list` | System | MRO, IA |
| `inventory.create`, `inventory.update` | System, InventoryItem | MRO |

```cedar
// 56_workflow.cedar (excerpt)
@id("task.act") @role("*")
permit (principal, action in [Akr::Action::"task.approve", Akr::Action::"task.reject", Akr::Action::"task.sign",
                              Akr::Action::"task.submit_form", Akr::Action::"task.review"], resource is Akr::Task)
when { resource.eligible && resource.state != "parked" };

// Four-eyes (SP-21, DR-57): the maker, an earlier decider of the pass, or someone acting for either never decides.
@id("task.four_eyes") @deny("self_approval")
forbid (principal, action in [Akr::Action::"task.approve", Akr::Action::"task.reject", Akr::Action::"task.sign"],
        resource is Akr::Task)
when { resource.deciding && resource.excluded };

// 55_change_control.cedar (excerpt)
@id("inventory.no_owner_approval") @deny("self_approval")
forbid (principal, action == Akr::Action::"inventory.update", resource is Akr::InventoryItem)
when { resource.owner == principal && context.approve_for_use };
```

`POST /change-requests/{id}/approve` and `/reject` evaluate `task.approve` and `task.reject` on the task they resolve
to; their route actions are `change_request.approve` and `change_request.reject`, aliases in the schema's action
groups so the route coverage test holds. Attributes evaluated: `principal.roles`, `delegated_roles`, `status`;
`resource.family`, `maker`, `parties`, `eligible`, `excluded`, `deciding`, `state`, `subject_visible`, `owner`;
`context.aal`, `stepup_age_s`, `approve_for_use`. The bundle tests assert: no role is permitted `change_request.create`
for a family and `task.approve` on its own request (the forbid wins for every role); no shipped definition completes
with fewer than two people; ADM cannot read a `detection` request's logic.

**Tipping-off review.** This phase touches staff and configuration only; no customer, alert or case data enters a
change request, a task title or the inbox:
- Change requests in phase 04 change grants, settings, bundles, definitions and task assignments; their views go to
  the family audience and the parties, and `proposed` and `impact` (classified `detection-logic`) are hidden from
  readers without `change_request.read_logic`, so a branch-tier reader of a later detection request cannot learn a
  threshold.
- Task titles and step labels are neutral and checked against the tipping-off term list (`E_TIPPING_OFF_TERM`) once
  phase 09 fills it; `waiting` never names the people holding a task.
- Restricted tasks (none in production in this phase; the dev subject proves the rule) are invisible to anyone the
  subject does not admit: the masked 404, counts after the filter, no title in any list.
- No notification text exists yet (phase 22) and no SSE (phase 17); the timeline shows actors only to readers of the
  request.
- Audit details carry ids, codes, hashes and counts; comments are stored on the instance events and hashed in the
  audit row.
The answer for a person outside a case's access list is: nothing.

**Settings and configuration used:** the nine keys of migration 0016 (registry bounds: `change.evidence_max_age_days`
1-365; `change.emergency_review_h` 1-24; `change.emergency_review_working_days` 1-5; `change.post_review_days` 1 to 3
integers each 7-365, ascending; `change.policy_shadow_min_h` 1-168; `change.draft_ttl_days` 1-365;
`access.assessor_max_days` 1-365; `change.route_map` validated as in section 1); phase 03 `delegation.max_days` and
the `authz.*` keys; phase 01 `stepup.fresh_window_s`; phase 00 `timers.sweep_interval_s`.

## 4. Infrastructure Updates

- **Job kinds registered** (worker): `timer_cr_activate`, `timer_cr_emergency_review`, `timer_cr_post_review`,
  `timer_task_timeout` (enqueued by the phase 00 timer sweep; concurrency 4; max attempts 10 with the phase 00
  back-off; each handler idempotent through the owner re-check); `change_sweep` (scheduled, `schedule.change_sweep`;
  concurrency 1). **Timer kinds:** `cr_activate`, `cr_emergency_review`, `cr_post_review`, `task_timeout` (section
  3.6). **Outbox topic:** `change.effective {change_request_id, object_kind, object_key}`; consumers in this phase: the
  settings cache (publishes `settings.changed` on Valkey early instead of waiting for the TTL); the authz bundle
  watcher keeps its `NOTIFY akr_policy` path and ignores the topic; from phase 17 the alert engine pins versions from
  it.
- **Compose:** no infrastructure changes needed.
- **AppState additions** (`api`): `changes: Arc<ChangeRequestService>`, `change_targets: Arc<ChangeTargetRegistry>`,
  `routes: RouteSelector`, `engine: Arc<WorkflowEngine>`, `subjects: Arc<SubjectRegistry>`, `effects:
  Arc<EffectRegistry>`, `holders: Arc<dyn Holders>`, `officers: Arc<dyn NamedOfficers>`, `definitions:
  Arc<DefinitionService>`, `inbox: Arc<InboxService>`, `inventory: Arc<InventoryService>`, `evidence:
  Arc<EvidenceRegistry>`; the offboarding registry gains `open_tasks` and `change_drafts`. `worker` builds the same
  registries (a test asserts both binaries register the same kinds, subjects and effects) and registers the five jobs.
  `ctl` gains `workflow validate <file>` (offline validation, exit code 1 on errors), `workflow seed
  [--offer-updates]` and `workflow rebuild-instance <id> [--dry-run]`.
- **OpenAPI registration:** tags `change-control` (9: the eight of section 2 and the evidence route of section 7.2), `inbox` (10), `workflow-admin` (8, including reassign),
  `inventory` (3), plus `PUT /admin/policies` under the phase 03 tag `authz` and `POST /admin/users/{id}/grants` under
  `admin-users`; 32 operations (the 31 of section 2 and the evidence route of section 7.2, a ninth operation of the tag `change-control`, mounted in `change_control::router()`), each with `x-akr-action`; `PUT /admin/settings/{key}` documents its new 202 answer; the SU operations (`/change-requests/{id}/approve` and `/reject`, `/tasks/{id}/actions/approve`,
  `/reject`, `/sign`, `/admin/tasks/{id}/reassign`, and `PATCH /inventory/{id}` when approving for use) reference the
  step-up scheme and declare `X-StepUp-Assertion`; `validate` and `test-run` state that they take no
  `Idempotency-Key`; the 202 responses document `Location`. The DSL schemas register under the `WfDsl*` and
  `CrDsl*` prefixes (DBS decision 62).
- **Network zones:** no new edge; the engine and the inbox live in `akr-api` and `akr-worker` on `z2_app`.
- **Readiness:** `akr-api` adds `workflow_routes` (every route named by `change.route_map` resolves to a published
  family); `akr-ctl db check` lists unpublished route families and parked tasks older than one day.
- **Metrics:** `akr_change_requests_open{class}` (gauge), `akr_change_requests_total{kind, outcome}`,
  `akr_tasks_open{state}` (gauge), `akr_tasks_parked` (gauge), `akr_tasks_overdue` (gauge),
  `akr_workflow_act_seconds` (histogram), `akr_inbox_query_seconds{view}` (histogram), `akr_emergency_reverts_total`.
  Alert rules: an emergency review past due without outcome, a parked task older than one working day, a route
  family unavailable.
- **Existing entity, schema or route updates:**
  - Phase 03 `SettingsAdminService::update`: a governed key creates a `setting` change request (202) instead of 409
    `governed`; non-governed keys stay direct (200). `UpdateSetting` gains the change fields.
  - Phase 03 `UserAdminService::detail` lists open `role_grant` requests for the user; `GET /admin/policies` shows the
    open bundle request and its shadow report; `role_grants.change_request_id` and `policy_bundles.change_request_id`
    get their FKs (0014).
  - Phase 03 `sod.json` keeps its one pair; the announced change-request pair is not added (phase 03 section 3.1 and section 3.5 of this phase, DR-54).
  - Phase 03 `OffboardingService` gains two steps; the cockpit gains the parked-tasks tile.
  - Phase 03 inference harness: `MASKED_KINDS` gains `task_restricted`.
  - Planning docs aligned in the same change: API.md 3.3, 3.15, 3.20 and 3.21; DATA-MODEL s.1 (change requests,
    engine tables, tasks, inventory, reason codes) and s.23; DSL.md D.1, D.4, E.2, E.3, E.4, 9.7 and s.7; phase-03.md
    (Cedar file ranges, the SoD sentence); roadmap rows 04, 08 and 09.

## 5. Frontend Impact

Backend phase for screens. `make generate-api` regenerates the client with the hooks `useGetDslKindSchema`,
`useGetChangeRequests`, `usePostChangeRequests`, `useGetChangeRequestsId`, `usePostChangeRequestsIdSubmit`,
`usePostChangeRequestsIdApprove`, `usePostChangeRequestsIdReject`, `usePostChangeRequestsIdWithdraw`,
`useGetInventory`, `usePostInventory`, `usePatchInventoryId`, `useGetInbox`, `useGetInboxCounts`, `useGetTasksId`,
`usePostTasksIdClaim`, `usePostTasksIdRelease`, `usePostTasksIdActionsApprove`, `usePostTasksIdActionsReject`,
`usePostTasksIdActionsSign`, `usePostTasksIdActionsSubmitForm`, `usePostTasksIdActionsReview`,
`usePostAdminTasksIdReassign`, `useGetAdminWorkflowDefinitions`, `useGetAdminWorkflowDefinitionsId`,
`usePostAdminWorkflowDefinitions`, `usePutAdminWorkflowDefinitionsIdDraft`,
`usePostAdminWorkflowDefinitionsIdValidate`, `usePostAdminWorkflowDefinitionsIdTestRun`,
`usePostAdminWorkflowDefinitionsIdPublish`, `usePutAdminPolicies`, `usePostAdminUsersIdGrants`.

Phase 08 builds on them (roadmap row 08 is widened to match): the minimal inbox (the task half of X3 "My inbox":
views, counts badge, overdue and escalated flags, claim and release), the task panel with the affordances and the
`StepUpDialog` for deciding actions, the change-request list and detail with the diff (DESIGN-SYSTEM diff rule: old
value struck through, new value with the start-edge bar), evidence, route and timeline, and the propose forms in A1
(role grant) and A2 (governed setting: "propose a change" replaces the read-only state of phase 03). The workflow
designer canvas is on no P0/P1 line (DR-402) and the inventory screens wait for phase 27; ADM authors definitions as JSON
in a plain editor screen with validate and test-run panels, built in phase 08 only if time allows, else by `akr-ctl`.

## 6. Verification

1. `make migrate`: 0014-0016 apply; a second run is a no-op; the phase 00 schema checks pass for the eight new tables
   (classification and retention rows present); `\dp admin.change_requests` shows `akr_app` with UPDATE on the
   lifecycle columns and `evidence` only (section 7.18); `UPDATE admin.change_requests SET proposed = '{}'` as `akr_app` fails (permission); an
   `UPDATE` or `DELETE` on `admin.instance_events` fails in the trigger; changing the `dsl` of a published version
   fails in the freeze trigger; a second open request on one `object_key` fails on `uq_change_requests_open`; a task
   row with `completed_by = maker_id`, or `acting_for = maker_id`, or a second deciding `done` task with the same
   `(instance_id, pass, decider)`, fails its CHECK or index; an inventory item with `validator_id = owner_id` fails.
2. `cargo check` with `cargo sqlx prepare`; `cargo clippy --all-targets -- -D warnings`; `cargo deny check`.
3. `akr-ctl workflow validate backend/workflows/shipped/*.json` -> 0 errors for the nine; fixtures that each break
   one rule -> exactly their code (`E_AUTO_DECISION`, `E_FOUR_EYES_OFF`, `E_STEPUP_DISABLED`, `E_ACL_ASSIGNEE_REQUIRED`,
   `E_ASSIGNEE_UNAVAILABLE`, `E_EFFECT_UNAVAILABLE`, `E_SUBJECT_ATTR_UNKNOWN`, `E_UNSUPPORTED_STEP_TYPE`,
   `E_DANGLING_REF` for an unknown role, `W_ROLE_WITHOUT_HOLDERS`). `akr-ctl workflow seed` publishes nine families;
   a second run reports `present` nine times and writes nothing.
4. `cargo test`, including:
   - engine invariants (DBS I1-I8 suite ported): two concurrent approvals on one task -> one 200, one 409; two claims
     -> one winner; a crash between the action and the response replays from the idempotency record; a rebuild of a
     healthy instance reports zero divergences and changes no row;
   - four-eyes: the maker approving -> 403 `E_SELF_APPROVAL` from Cedar; a delegate acting for the maker -> 403 from
     the engine; the same person on both `cr_material` branches -> the second refused; with Cedar and the engine
     bypassed (repository call) the database refuses each case; the bundle test over every role finds no permit that
     escapes `task.four_eyes`;
   - routes: every rule of the default route map selects its family for a matching request; a map naming an
     unpublished family -> submit 409 `route_unavailable`; a map change without a fallback for a class -> 422;
   - classes: a `decrease` on a minor kind becomes `material`; `capacity` without a below-the-line sample on a kind that
     needs one -> `E_CAPACITY_NO_BTL`; `class_requested` lower -> `E_CLASS_LOWERED`; `emergency` on `role_grant` -> 422
     `emergency_not_allowed`;
   - activation: an effect error rolls the approval back (task still open, no `approved` row); a stale base at approval
     -> 409 and at a timer -> `failed`; `effective_at` in the future activates at the timer; the outbox row is written
     in the activation transaction;
   - emergency: a loosening setting change under emergency is reverted by the timer after the review window (test
     clock), with `reverted` and the before value restored; a tightening one is escalated, never reverted; the
     emergency approver is excluded from the review;
   - post review: day-30 and day-90 instances start once each; a reverted request starts none;
   - sweep: a claim held by a user whose grant was revoked is released; a task with only excluded holders is parked
     and unparked when a new holder is granted;
   - inbox: counts equal list lengths for every view; a restricted task of `_dev_masked` is absent from lists and
     counts and its get and actions answer the masked 404 (the `task_restricted` probe set); `cargo bench` meets the
     section 3.7 budget;
   - offboarding: a user with claimed tasks and drafts -> tasks released or parked, drafts withdrawn, counts in the
     report.
5. Curl suite `tests/curl/phase04_change.sh` (dev users from `akr-ctl identity seed-dev`: `adm@bank.test`,
   `sec@bank.test`, `m@bank.test`, `chk@bank.test`, `ch@bank.test`, `cfg@bank.test`; codes from
   `oathtool`; the test clock of phase 00 for time travel):
   - Grant: as ADM `POST /api/v1/admin/users/<cfg id>/grants` with `Idempotency-Key: gr-0001` and
     `{"role":"CHK","scope":{"all_branches":true},"reason_code":"ACC_MOVER","reason_text":"..."}` -> 202 with
     `Location` and `"route":"cr_access"`; the same key and body -> byte-identical body with `x-idempotent-replay:
     true`. As SEC `POST /api/v1/change-requests/<id>/approve` with `If-Match` and no step-up -> 401
     `E_STEPUP_REQUIRED`; with one (`action_class: approval`) -> 200 `"status":"effective"`; `GET
     /api/v1/admin/users/<cfg id>` lists the grant with `change_request_id`.
   - Self-approval (the Cedar deny case): ADM proposes a governed setting change; ADM approves it -> 403
     `{"code":"E_SELF_APPROVAL",...}` and an `authz.denied` row with `deny_reason: self_approval`; as `chk@bank.test`
     `POST /api/v1/admin/tasks/<id>/reassign` -> 403 `E_SCOPE`.
   - Delegation: `chk@bank.test` delegates CHK to `cfg@bank.test` for a day (phase 03 route, step-up), then creates a
     minor `_dev_object` request (route `cr_minor`); `cfg@bank.test`, holding CHK only through that delegation,
     approves -> 403 `E_SELF_APPROVAL` (engine check, `acting_for` = the maker).
   - Material route: a `_dev_object` change `{"class":"material","volume_effect":"decrease"}` -> route `cr_material`;
     M and CHK approve in parallel; the CH step appears; CH approves -> `effective`.
   - Emergency: as ADM a `_dev_object` change `{"volume_effect":"decrease"}` with `emergency: true`, reason
     `CHG_SECURITY` and an `evidence_object` attached -> route `cr_emergency`; M approves -> `effective`,
     `review_outcome: pending`, a review instance on `cr_material` that excludes M; the clock moved 25 hours ->
     `reverted` and the old value back. The same with `"volume_effect":"increase"` and the clock moved 6 days ->
     `review_outcome: escalated`, value kept, a CH escalation task in `ch@bank.test`'s inbox. A `setting` emergency on
     `authz.shadow_sample_pct` (neutral) -> M approves -> `effective` with `review_due_at` 5 days out.
   - Stale base: a submitted `_dev_object` request; the dev row's version bumped directly in `psql`; approving -> 409
     `E_STALE_BASE`, the task still open.
   - Evidence: a request whose evidence is older than `change.evidence_max_age_days` (set to 1 in the test, clock
     moved 2 days) -> approve 422 `E_EVIDENCE_STALE`.
   - Claim race: two `POST /api/v1/tasks/<id>/claim` with the same `If-Match` in parallel -> one 200, one 409.
   - Policy: as SEC `PUT /api/v1/admin/policies` with the staged bundle -> 202, route `cr_access_adm`; ADM approves at
     once -> 422 `shadow_too_short`; with `change.policy_shadow_min_h` lowered to 1 by its own request and the clock
     moved -> 200 `effective`, `GET /api/v1/admin/policies` shows the new active version.
   - Workflow: ADM creates `_dev_flow`, `POST .../test-run` -> a trace naming eligible and excluded actors; `POST
     .../publish` -> 202 route `cr_workflow`; M approves -> version 1 published with `change_request_id`.
   - Masking: as ADM `GET /api/v1/tasks/<restricted dev task>` and `GET /api/v1/tasks/<random uuid>` -> both 404 with
     bodies equal after removing `correlation_id`; `GET /api/v1/inbox/counts` does not count it.
   - Rebuild: `akr-ctl workflow rebuild-instance <id> --dry-run` -> 0 divergences.
6. Swagger UI lists 32 new operations (86 with phases 01 and 03), each with `x-akr-action`; `make generate-api` twice
   gives zero diff.
7. Audit: after the curl run `audit_log` holds `change_request.created`, `.submitted`, `.effective`, `.reverted`,
   `.rejected`, `role_grant.created`, `setting.updated {source: change_request}`, `policy_bundle.activated
   {activation: change_request}`, `workflow_definition.published`, `task.claimed`, `task.approved` with `stepup_id`,
   and `authz.denied` rows with `deny_reason: self_approval`; `akr-ctl audit verify` passes.
8. No CBS connection in this phase: the read-only conformance suite is not affected.
9. Phase 00 to 03 tests still pass; the phase 03 test expecting 409 `governed` now expects 202.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 2, from tools/agent/gap1-run2.md; lead-verified (tools/agent/gap1-run2-verdict.md).

### 7.1 WorkflowSubject: check_before_decision
Section 2, trait `WorkflowSubject`, gains one method (phase 18 section 4.5 item 1 states it):

    async fn check_before_decision(&self, tx: &mut Tx, inst: &WorkflowInstance, actor: &AuthContext,
        task: &Task) -> Result<(), WorkflowError>;   // default body: Ok(())

Section 3.4 step 6 reads: "Subject check before decision: the subject's `check_before_decision` runs in the same
transaction, before the action applies; the change subject runs `ChangeTarget::check_before_decision` through it. A
refusal rolls the action back and the task stays open." Subjects with no extra rule keep the default
(`CaseWorkflowSubject`, phase 18; `RfiWorkflowSubject`, phase 19; `FreezeWorkflowSubject` and
`OpsTaskWorkflowSubject`, phase 20, implement it).

### 7.2 Evidence upload for change requests (DR-363)
The evidence owner kind `change_request` is registered in the phase 02 registry (owner id = the request id; phase 02
7.10). New route (gap item (d)):

| Method | Path | Scope | Description |
|-|-|-|-|
| POST | `/change-requests/{id}/evidence` | config.author, admin.users, admin.policies, admin.system | The maker attaches a file to their request while it is `draft`, or `submitted` with no decision recorded yet; `Idempotency-Key`; `If-Match` (request version); 201 with the evidence id |

Steps: only the maker (else 403 `E_FORBIDDEN`); status `draft`, or `submitted` with no deciding task of the request's instance in state `done` (a recorded decision: outcome `approved`, `rejected` or `signed`) in
the current pass (else 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `evidence_closed`); `EvidenceStore::stage` with owner
kind `change_request`; append `{kind: evidence_object, id}` to `change_requests.evidence`; version + 1. Audit
`change_request.evidence_added {evidence_id, kind}` (resource `change_request`, SIEM no). A request created through a
dedicated route (a governed setting, a grant, a policy bundle) receives its new documents the same way, before its
first decision; the freshness rule of section 3.2 applies at each deciding action as before.

### 7.3 Engine step 5 passes the declared step-up action
Section 3.4 step 5 reads: "Deciding steps and every `sign`: `StepUpService::consume(tx, ctx, header, action,
resource_ref = "task:<id>")` (phase 01 signature), where `action` is the declared `StepUpAction` of the act:
`task.approve` and `task.reject` (class `approval`), `task.sign` (class `signing`); on the change routes
`change.approve` and `change.reject` (class `approval`). A subject's own constants are passed the same way. 401
`E_STEPUP_REQUIRED` as before; the assertion id goes to `tasks.stepup_id`."

### 7.4 Holders rule 5 (case_acl)
Section 3.5 rule 5 reads: "`case_acl`: an active `casework.case_acl` row with `access = decide` on the instance's case;
when the instance's definition is `case_report_approval`, the principal also holds role `M` (phase 18 section 4.5
item 2). The inbox, the Task adapter and the engine read this one rule. Until phase 18 ships the case subject, a
definition that names `case_acl` is refused at publish (`E_ASSIGNEE_UNAVAILABLE`)."

### 7.5 Approver ids of open change requests (DR-92)
`ChangeRequestService` gains the read method `approver_ids(conn, object_kind, object_key) -> Vec<Uuid>`: for every
`submitted` request on that object, the users who may hold its open deciding task (`Holders::eligible`, section 3.5)
and the users who already decided in the current pass. The maker is not included (phase 11 admits the requester by
its own rule). Read only, no step-up, no audit row. Phase 11 sets the Cedar attribute `resource.approver_ids` of a
mapping replay from it (phase 11 Open doubts 7 closed).

### 7.6 A validation pass sets the inventory dates
Section 3.9 "Update" gains: "A validation pass (phase 24 section 3.6 step 8) calls `InventoryService::update` in the
validation transaction with the validator's context. It sets `last_validated_at` and `next_validation_due` and nothing
else; the status does not change, so no step-up applies. Audit `inventory_item.updated {fields, status_from,
status_to}` with `status_from = status_to`." The sentence "Next validation dates are entered by the model-risk officer
here" reads: "The model-risk officer enters a date by hand, or a passing validation sets it (phase 24); phase 27
computes due dates from the tier and lists overdue items."

### 7.7 role_grant revoke and the platform_release target (phase 25)
- Section 3.3, `role_grant` gains the operation `revoke` (phase 25 section 3.1, DR-234): maker SEC, route
  `cr_access_adm`, checker ADM. Apply: `UPDATE admin.role_grants SET revoked_at = now(), revoked_by = <checker> WHERE
  id = $grant AND revoked_at IS NULL`, then `invalidate_user`; the apply writes `role_grant.revoked` (phase 03 kind). A
  proposal for a grant that is not active: 422 `grant_not_active`, no request created. The phase 03 `DELETE
  /admin/users/{id}/grants/{grant_id}` is unchanged.
- New target kind `platform_release` (phase 25 section 4.5): route `cr_access`, object key
  `release:<release_version>`, `volume_effect = neutral`, evidence kind `package` with owner kind `release` (phase 02
  7.10), its `EvidenceResolver` registered with the target.
- The `change.route_map` seed of section 1 lists the kind in the second rule, so no later migration edits the map:
  `{"object_kind": ["role_grant", "policy_bundle", "task_assignment", "platform_release"], "route": "cr_access"}`. The
  first rule (maker SEC -> `cr_access_adm`) keeps its three kinds. Before phase 25 registers the target, the extra name
  matches nothing.

### 7.8 Notice requests from reminders, escalations and the notify effect (phase 22)
From phase 22, which registers the topic `notify.request` and its handler in the same release (phase 22 section 4.5
item 5):
- `remind` writes, in its transaction, the outbox row `notify.request {task_id, template_code: "task.overdue", to:
  "holders"}`; `escalate` writes `notify.request {task_id, template_code: "task.waiting", to: "escalation_target"}`.
- The terminal effect `notify` is `NotifyEffect` (all resource kinds) with params `{"template_code": <a closed code of
  phase 22 section 1.2>, "to": "initiator" | "subject_team"}`; it writes `notify.request {workflow_instance_id,
  template_code, to}`. Until then `notify` stays deferrable (section 3.4) and no row is written.

### 7.9 Volume effect is fixed at submit (DR-360)
A request's class and `volume_effect` are computed at submit (section 3.1) and are not recomputed. At each deciding
action, a target whose volume depends on a measurement (a replay, phase 16; a reader measure, phase 13) re-measures
in its `check_before_decision`; when the result differs from the stored `volume_effect`, the decision is refused with
422 `E_VALIDATION`, sub-code `volume_effect_changed`, and the maker withdraws and submits again, so the approvers
always decide on the class they see. No recompute entry point exists (phase 16 Open doubts 12 closed).

### 7.10 One setting caps an assessor grant (DR-361)
The cap of an `AS` grant is the phase 04 key `access.assessor_max_days` (prefix `access.`, route `cr_access`). Its seed
becomes default `30` with the floor `{"max": 30, "basis": "M-62 time-boxed read-only assessor grant"}`, and the
settings validation of section 3.10 reads 1-30 instead of 1-365. Phase 24's key `assessor.grant_max_days` is not created;
phases 24 and 25 read this key (their 7.x record the correction).

### 7.11 Families and routes added by later phases
Section 3.1 family list: phase 04's families are `access`, `setting`, `workflow` and `dev`. Later families are added
by the phase that owns them: `detection`, `mapping`, `kyc_model`, `reference` (as already written), `source`
(phase 05) and `reporting` (phase 07). Route rules named by later phases (phase 05 migration 0019, phase 09
`cr_authority`) are written by those phases. The nine rows of the shipped table are this phase's set; definitions
shipped by later phases (09, 15, 18 to 21, 23) are listed in those phases.

Gap pass 1 (2026-10-10), run 4b: items carried from run 4a and corrections found while checking phases 25 to 31 (tools/agent/gap1-run4b-verdict.md); lead-verified.

### 7.12 Model inventory items keep R9 (DR-392)
`admin.retention_assignment` is keyed by table, so one table cannot be R9 for some rows and R11 for others. The
retention row of `admin.inventory_items` stays R9 `retirement` for every row, model items included; the model evidence
is R11 through phase 27's own tables (`admin.models`, `admin.model_versions`, `admin.model_evaluations`). The note of
that retention row is corrected to say so.

Adversarial pass (2026-10-10), run 1 (four-eyes and self-approval), from tools/agent/adv-run1.md; lead-verified (tools/agent/adv-run1-verdict.md).

### 7.13 Emergency changes never touch access (ADV-1-01, HIGH; DR-407)
The first rule of `change.route_map` sends every `emergency` request to `cr_emergency` (one MLRO approval), ahead of the
access rules, and its review runs on the business family. That would let an access change escalate to a business role,
which the shipped definitions forbid ("Access changes never escalate to business roles"). Section 3.1 classify step 5 therefore refuses `emergency = true` (422 `E_VALIDATION`,
`emergency_not_allowed`) for the kinds `role_grant`, `policy_bundle` and `task_assignment`, and for a `setting` whose key
starts with `authz.`, `identity.`, `stepup.`, `session.`, `delegation.` or `access.`. These changes keep their access
route (`cr_access` or `cr_access_adm`), which is already one security approval. The emergency rule of the route map
then applies only to the other kinds and keys. Test: an emergency `setting` change on `stepup.max_failures` is 422
`emergency_not_allowed`.

### 7.14 Rework is a reject mode (ADV-1-10, LOW; DR-407)
Section 3.4 delta 8 allows `rework`: it is the `mode` of a definition's reject, not a task action. A reject with mode
`rework` returns the instance to the maker and increments `pass` on every later task. In each pass the maker stays
excluded and the per-pass unique deciders apply; an approver of an earlier pass may approve a later pass. The case
definitions of phase 18 use `mode: end` (phase 18 section 7+).

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.15 Names in the change-request signatures (V-04-01-a, LOW; DR-413)
`StepUpAssertionId` is the `id` (a UUID) of a phase 01 `StepUpAssertion`; the request and its task store it in
`stepup_id`. `ActorRef` in sections 2 and 3 is the phase 02 `Actor` enum (`User`, `System` or `Anonymous`). The `d`
argument of `decide` is the `TaskAction` of section 3.4 with the value Approve or Reject, not the phase 03 `Decision`
struct. `Activated` is a unit marker with no field; no step reads a value from it.

### 7.16 Inventory Create: order of checks and the owner refusal (V-04-02-a, MEDIUM; DR-413)
Section 3.9 Create runs its checks in this order, after the authorize layer:
1. The owner is an active human user, else 422 `E_VALIDATION` at `/owner_id`, reason `owner_not_active` (the reason
   phase 12 uses for the same check). The validator, when one is given, is an active human user, else 422
   `E_VALIDATION` at `/validator_id`, reason `validator_not_active`.
2. The validator is not the owner (422 `E_VALIDATION`, `validator_is_owner`); `user_roles` are role codes (422
   `E_VALIDATION` at `/user_roles`).
3. Insert with status `registered`; a second item for the same `(kind, object_id)` is 409 `E_NOT_ALLOWED_IN_STATE`,
   `already_registered`.
4. Audit `inventory_item.created`, as section 3.9 names it.

### 7.17 The change subject returns the target's refusal as a WorkflowError (V-04-07-a, LOW; DR-413)
The change subject's `check_before_decision` (7.1) returns a refusal of `ChangeTarget::check_before_decision` as
`WorkflowError::Change(ChangeError)`, a variant of the `thiserror` enum `WorkflowError` converted with `#[from]`.
`api::error` maps `Change` to the codes of section 3.2: 409 `E_STALE_BASE`, 422 `E_EVIDENCE_STALE` or 422
`E_VALIDATION`.

### 7.18 Evidence route: Cedar action and repository method (V-04-08-b, V-04-08-c, MEDIUM; DR-413)
The route `POST /change-requests/{id}/evidence` (7.2) is authorized by the Cedar action `change_request.evidence_add`
on ChangeRequest: `principal == resource.maker` and status `draft` or `submitted`; the action table of section 3.10
gains this row. The decided-task test of 7.2 stays a service check (409 `E_NOT_ALLOWED_IN_STATE`, `evidence_closed`).
`ChangeRequestRepository` gains `append_evidence(tx, id, item: EvidenceRef, expected_version)`: one UPDATE that
appends the item to `evidence` and sets `version = expected_version + 1` where `version = expected_version`; no row
matched is 409 `E_VERSION_CONFLICT`. The append step of 7.2 calls it. Migration 0014 (section 1): the column grant on
`admin.change_requests` includes `evidence` (V-04-08-a, HIGH).

### 7.19 Where a rework returns to (V-04-12-a, LOW; DR-413)
A reject with `mode: rework` (section 3.4 delta 8; 7.14) names its return step: the reject object carries `to`, the id
of an earlier step of the same definition, and the definition validator refuses `mode: rework` without it (`E_SCHEMA`); a `to` that is not a step of the definition from which the rejecting step can be reached is `E_DANGLING_REF`. The engine
writes `rework_returned {from_step, to_step, pass}` and creates the tasks of the `to` step with `pass + 1`. The words
"returns the instance to the maker" in 7.14 mean that step. Every shipped definition uses `mode: end`, so none ships a
return step.

### 7.20 Two effects may share a kind (V-15-01-d, HIGH; DR-413)
From phase 15 on, the effect registry resolves a terminal effect by its `kind` and the resource kind of the instance. Two effects may share a kind when their `resource_kinds` do not overlap (phase 15 registers `record_decision` for `change_request`, `kyc_review` and `exit_decision`; phase 18 for `case`); an overlap fails start-up. `E_EFFECT_UNAVAILABLE` is unchanged.

## What This Phase Does NOT Touch

- No detection, mapping, rating or reference kinds: their change targets, evidence resolvers (replay, below-the-line
  samples, shadow comparisons, impact analyses) and inventory gates register in phases 10, 14, 15 and 16; the
  `cr_material_validated` and `cr_mapping` routes exist but nothing selects them before then.
- No authority matrix and no bank calendar (phase 09): steps are held by role and office; working days are calendar
  days until `BankCalendarClock`; the other reason-code families and `POST /ref/reason-codes` are phase 09.
- No case, report, KYC or exit workflows (phases 15, 18, 21): `case_acl` assignees, `record_decision`,
  `release_for_filing` and `create_ops_task` arrive there.
- No notifications (phase 22) and no SSE (phase 17): reminders and escalations show in the inbox only.
- No workflow designer canvas (on no P0/P1 line, DR-402) and no inventory screens or validation calendar (phase 27); no findings
  register (phase 24) for post-review concerns.
- No screens (phase 08); no CBS connection; no compose change; no change to phase 02 tables; phase 03 tables gain only
  the two FKs.
