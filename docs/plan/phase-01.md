# Phase 01: Identity & Sessions

**Goal:** Standalone identity live end to end: Kratos and Hydra configured behind the BFF; the ar/en sign-in state
machine (password, a mandatory second factor with WebAuthn first and TOTP as the minimum, forced first login with the
confidentiality undertaking); the session cookie and CSRF filling the phase 00 auth slot; idle and absolute session
limits; the single-use step-up ledger bound to an action class; personal access tokens and client-credential service
principals; the principal mirror with role grants, pools and delegations; the bootstrap CLI for the first
administrator and security officer; and the CBI password and lockout minima as floors and the session limits as hard bounds (DR-34), with every identity
mutation naming its audit event.

**Dependencies:** Phase 00
**Macro-phase:** P0
**Complexity:** L (22 routes; the size is the Kratos contract, the session layer and the step-up ledger)
**Binaries touched:** akr-api (auth layer, `auth` and `me` routers) | akr-worker (`identity_sweep`; `maintenance_sweep`
extended) | `akr-ctl` (`identity` commands; `db provision` creates the Ory databases). akr-ingestd and akr-assess unchanged
**Requirements carried:** SP-14, SP-15, SP-16 (personal tokens and client-credential service principals with owner,
expiry and last use; gateway certificates in phase 29), SP-17 (dormancy and account expiry; the leave block arrives
with the leave ledger), SP-18 (sign-in and step-up events carry what the anomaly rules need; the SIEM rules in phase
25), SP-71 (unauthenticated-route coverage), SP-101 (acknowledgement at first login; staff-monitoring visibility in
phase 24), TS-25 (standalone Kratos and local step-up; OIDC upstream in phase 25, Ory Polis in phase 31), UX-35 and
UX-36 (server half; the dialogs in phase 08)
- Gap pass 1 (coverage), partly implemented: VA-46 (personal access tokens with the user's roles and read scopes, audited).

Parallel-track note: phase 02 runs beside this phase. Every event below goes through `audit::AuditSink`; until phase
02 merges they land on the `TracingSink` stub, afterwards they persist, and no call site changes.

## 1. SQLx Migration Changes

Three migrations, `backend/migrations/0005` to `0007` (canonical chain in roadmap.md, DR-23). Conventions as in phase
00: schema-qualified names, no database default on `id`, no reserved words (`user`, `from`, `to` and `end` are
avoided, hence `from_user`, `to_user`, `ended_at`), per-table grants with column lists where a role may change only
some columns, and each migration inserts the classification and retention rows for its own tables.

Kratos owns credentials and sessions in its own database (`kratos`), Hydra its clients and keys in `hydra`; both are
created by `akr-ctl db provision` (section 4) and are not AKR schemas, so the phase 00 schema tests do not cover them.
Account state (status, forced change, lockout, password age, acknowledgement, expiry) lives in `admin.users`: one
authority per fact (DR-30).

```sql
-- migration 0005_identity_principals_grants.sql
-- Phase 01. Principals (human, service, system), pools, role grants and delegations; the deferred FKs owed by phase
-- 00. admin.users is the FK target of every actor column in the product (DR-33).

DO $$ BEGIN
    CREATE TYPE admin.principal_kind AS ENUM ('human', 'service', 'system');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

DO $$ BEGIN
    CREATE TYPE admin.user_status AS ENUM ('active', 'disabled');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS admin.users (
    -- human: equals the Kratos identity id (Kratos mints v4; the one exception to UUIDv7, as in the DBS);
    -- service: UUIDv7 from the application; system: the seeded constant below. Never a database default.
    id                       UUID PRIMARY KEY,
    kind                     admin.principal_kind NOT NULL DEFAULT 'human',
    display_name             JSONB NOT NULL CHECK (display_name ? 'ar' AND display_name ? 'en'),
    email                    TEXT CHECK (email = lower(btrim(email))),   -- copy of the Kratos identifier trait
    status                   admin.user_status NOT NULL DEFAULT 'active',
    disabled_reason          TEXT CHECK (disabled_reason IN ('offboarded', 'dormant', 'expired', 'admin')),
    vetting_status           TEXT NOT NULL DEFAULT 'none'
                             CHECK (vetting_status IN ('none', 'pending', 'vetted', 'lapsed')),
    vetting_expires_on       DATE,
    branch_id                UUID,            -- FK to admin.branches added by phase 03 (deferred FK, DR-27)
    locale                   TEXT NOT NULL DEFAULT 'ar' CHECK (locale IN ('ar', 'en')),
    preferences              JSONB NOT NULL DEFAULT '{}'::jsonb,   -- PreferencesV1, validated in code (section 2)
    owner_user_id            UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- service principals only
    hydra_client_id          TEXT,            -- service principals that authenticate with client credentials
    expires_at               TIMESTAMPTZ,     -- account expiry (contractors, inspectors); required for service
    forced_change            TEXT CHECK (forced_change IN ('first_login', 'recovery')),
    temp_password_expires_at TIMESTAMPTZ,
    password_changed_at      TIMESTAMPTZ,
    failed_logins            INT NOT NULL DEFAULT 0 CHECK (failed_logins >= 0),
    locked_until             TIMESTAMPTZ,     -- 'infinity' in the administrator-unlock lockout mode
    last_authenticated_at    TIMESTAMPTZ,     -- human: last full sign-in; service: last token use (throttled)
    policy_ack_version       INT,
    policy_ack_at            TIMESTAMPTZ,
    created_at               TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at               TIMESTAMPTZ NOT NULL DEFAULT now(),
    version                  INT NOT NULL DEFAULT 1,   -- bumped only by If-Match changes (preferences, profile)
    CHECK ((kind = 'human') = (email IS NOT NULL)),
    CHECK ((kind = 'service') = (owner_user_id IS NOT NULL)),
    CHECK (kind <> 'service' OR expires_at IS NOT NULL),
    CHECK (hydra_client_id IS NULL OR kind = 'service'),
    CHECK ((status = 'disabled') = (disabled_reason IS NOT NULL)),
    CHECK ((forced_change IS NULL) = (temp_password_expires_at IS NULL)),
    CHECK ((policy_ack_version IS NULL) = (policy_ack_at IS NULL)),
    CHECK (kind = 'human' OR (forced_change IS NULL AND password_changed_at IS NULL AND policy_ack_version IS NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_users_email        ON admin.users (email) WHERE email IS NOT NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uq_users_hydra_client ON admin.users (hydra_client_id) WHERE hydra_client_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_users_status     ON admin.users (status, kind);
CREATE INDEX IF NOT EXISTS idx_users_created_id ON admin.users (created_at DESC, id DESC);   -- keyset (phase 03)
CREATE INDEX IF NOT EXISTS idx_users_owner      ON admin.users (owner_user_id);
CREATE INDEX IF NOT EXISTS idx_users_branch     ON admin.users (branch_id);

-- The system principal: the actor of jobs and sweeps. It cannot sign in (kind <> 'human'), holds no grants and is
-- never disabled. JobContext carries this id (phase 00).
INSERT INTO admin.users (id, kind, display_name, locale)
VALUES ('00000000-0000-0000-0000-000000000001', 'system', '{"ar": "النظام", "en": "System"}', 'ar')
ON CONFLICT (id) DO NOTHING;

CREATE TABLE IF NOT EXISTS admin.pools (
    id           UUID PRIMARY KEY,
    name         JSONB NOT NULL CHECK (name ? 'ar' AND name ? 'en'),
    kind         TEXT NOT NULL CHECK (kind IN ('alerts', 'screening', 'kyc', 'rfi', 'qa', 'ops')),
    branch_ids   UUID[] NOT NULL DEFAULT '{}',    -- checked against admin.branches by the service (phase 03)
    archived_at  TIMESTAMPTZ,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    version      INT NOT NULL DEFAULT 1
);
CREATE INDEX IF NOT EXISTS idx_pools_kind ON admin.pools (kind) WHERE archived_at IS NULL;

CREATE TABLE IF NOT EXISTS admin.role_grants (
    id                 UUID PRIMARY KEY,
    user_id            UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    role               TEXT NOT NULL CHECK (role IN ('A1', 'A2', 'SS', 'M', 'KYC', 'RFI', 'QA', 'OPS', 'CFG', 'CHK',
                                                     'DS', 'CH', 'MRO', 'IA', 'AS', 'L', 'ADM', 'SEC', 'VS')),
    -- {"all_branches": true} or {"branches": [uuid, ...], "pools": [uuid, ...]}; never an empty object
    scope              JSONB NOT NULL CHECK (scope ? 'all_branches' OR scope ? 'branches'),
    source             TEXT NOT NULL CHECK (source IN ('bootstrap', 'change_request')),
    change_request_id  UUID,             -- FK to admin.change_requests added by phase 04 (deferred FK)
    valid_from         TIMESTAMPTZ NOT NULL DEFAULT now(),
    valid_to           TIMESTAMPTZ,
    granted_by         UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    revoked_at         TIMESTAMPTZ,
    revoked_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK ((source = 'change_request') = (change_request_id IS NOT NULL)),
    CHECK (valid_to IS NULL OR valid_to > valid_from),
    CHECK ((revoked_at IS NULL) = (revoked_by IS NULL))
);
CREATE INDEX IF NOT EXISTS idx_role_grants_user       ON admin.role_grants (user_id, valid_from);
CREATE INDEX IF NOT EXISTS idx_role_grants_role       ON admin.role_grants (role) WHERE revoked_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_role_grants_granted_by ON admin.role_grants (granted_by);
CREATE INDEX IF NOT EXISTS idx_role_grants_revoked_by ON admin.role_grants (revoked_by);
CREATE INDEX IF NOT EXISTS idx_role_grants_cr         ON admin.role_grants (change_request_id);

CREATE TABLE IF NOT EXISTS admin.delegations (
    id           UUID PRIMARY KEY,
    from_user    UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    to_user      UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    scope        JSONB NOT NULL,          -- {"roles": [code, ...]}; phase 09 adds decision types (authority matrix)
    valid_from   TIMESTAMPTZ NOT NULL,
    valid_to     TIMESTAMPTZ NOT NULL,    -- every delegation ends
    reason_code  TEXT NOT NULL CHECK (reason_code IN ('annual_leave', 'sick_leave', 'official_travel', 'training',
                                                      'other_duty')),   -- a code, never free text (tipping-off)
    created_by   UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    ended_at     TIMESTAMPTZ,
    ended_by     UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    CHECK (from_user <> to_user),
    CHECK (valid_to > valid_from),
    CHECK ((ended_at IS NULL) = (ended_by IS NULL)),
    CHECK (CASE WHEN jsonb_typeof(scope -> 'roles') = 'array' THEN jsonb_array_length(scope -> 'roles') > 0
                ELSE FALSE END),
    -- Administration, security, audit, assessor and vendor roles are never delegated (independence, SP-37).
    CHECK (NOT ((scope -> 'roles') ?| ARRAY['ADM', 'SEC', 'IA', 'AS', 'VS']))
);
CREATE INDEX IF NOT EXISTS idx_delegations_to         ON admin.delegations (to_user, valid_to);
CREATE INDEX IF NOT EXISTS idx_delegations_from       ON admin.delegations (from_user, valid_to);
CREATE INDEX IF NOT EXISTS idx_delegations_created_by ON admin.delegations (created_by);
CREATE INDEX IF NOT EXISTS idx_delegations_ended_by   ON admin.delegations (ended_by);

-- Deferred FKs owed by phase 00. ADD CONSTRAINT has no IF NOT EXISTS: guarded like the roles in 0001.
DO $$ BEGIN
    ALTER TABLE admin.idempotency_keys ADD CONSTRAINT fk_idempotency_keys_principal
        FOREIGN KEY (principal_id) REFERENCES admin.users(id) ON DELETE RESTRICT;
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
-- No extra index: principal_id leads the primary key (principal_id, key).

DO $$ BEGIN
    ALTER TABLE admin.settings ADD CONSTRAINT fk_settings_updated_by
        FOREIGN KEY (updated_by) REFERENCES admin.users(id) ON DELETE RESTRICT;
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
CREATE INDEX IF NOT EXISTS idx_settings_updated_by ON admin.settings (updated_by);

DO $$ BEGIN
    ALTER TABLE admin.feature_flags ADD CONSTRAINT fk_feature_flags_updated_by
        FOREIGN KEY (updated_by) REFERENCES admin.users(id) ON DELETE RESTRICT;
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
CREATE INDEX IF NOT EXISTS idx_feature_flags_updated_by ON admin.feature_flags (updated_by);

-- Column lists: a role changes only what its services change. Grant and delegation facts are immutable once
-- written; only their end may be recorded. kind, id and hydra_client_id never change.
GRANT SELECT, INSERT ON admin.users TO akr_app;
GRANT UPDATE (display_name, email, status, disabled_reason, vetting_status, vetting_expires_on, branch_id, locale,
              preferences, owner_user_id, expires_at, forced_change, temp_password_expires_at,
              password_changed_at, failed_logins, locked_until, last_authenticated_at, policy_ack_version,
              policy_ack_at, updated_at, version)
    ON admin.users TO akr_app;
GRANT SELECT, INSERT, UPDATE ON admin.pools TO akr_app;
GRANT SELECT, INSERT ON admin.role_grants TO akr_app;
GRANT UPDATE (valid_to, revoked_at, revoked_by) ON admin.role_grants TO akr_app;
GRANT SELECT, INSERT ON admin.delegations TO akr_app;
GRANT UPDATE (ended_at, ended_by) ON admin.delegations TO akr_app;

-- akr-ingestd and akr-assess resolve no people. The read-only role renders actors in the audit viewer and builds
-- the access-matrix report (SP-20): names and grant facts, never contact or account-state columns.
GRANT SELECT (id, kind, display_name, status, branch_id) ON admin.users TO akr_readonly;
GRANT SELECT ON admin.pools, admin.role_grants, admin.delegations TO akr_readonly;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification, note) VALUES
 ('admin', 'users', 'display_name', 'personal', 'Staff name in Arabic and English'),
 ('admin', 'users', 'email',        'personal', 'Copy of the Kratos identifier trait')
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', v.t, c, 'internal'
FROM (VALUES
  ('users',       ARRAY['id','kind','status','disabled_reason','vetting_status','vetting_expires_on','branch_id',
                        'locale','preferences','owner_user_id','hydra_client_id','expires_at','forced_change',
                        'temp_password_expires_at','password_changed_at','failed_logins','locked_until',
                        'last_authenticated_at','policy_ack_version','policy_ack_at','created_at','updated_at',
                        'version']),
  ('pools',       ARRAY['id','name','kind','branch_ids','archived_at','created_at','updated_at','version']),
  ('role_grants', ARRAY['id','user_id','role','scope','source','change_request_id','valid_from','valid_to',
                        'granted_by','revoked_at','revoked_by','created_at']),
  ('delegations', ARRAY['id','from_user','to_user','scope','valid_from','valid_to','reason_code','created_by',
                        'created_at','ended_at','ended_by'])
) AS v(t, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'users',       'R9', 'retirement', NULL, 'Never deleted while an actor column refers to it; retirement = disabled'),
 ('admin', 'pools',       'R9', 'retirement', NULL, 'Retirement = archived'),
 ('admin', 'role_grants', 'R9', 'retirement', NULL, 'Grant history for access reviews (SP-20); retirement = revoked or past valid_to'),
 ('admin', 'delegations', 'R9', 'retirement', NULL, 'Retirement = ended or past valid_to')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

```sql
-- migration 0006_identity_stepup_tokens_passwords.sql
-- Phase 01. The single-use step-up ledger (DR-29), personal access tokens (DR-28) and the password history the CBI
-- minima need (DR-30).

CREATE TABLE IF NOT EXISTS admin.stepup_assertions (
    id                  UUID PRIMARY KEY,     -- the value sent as X-StepUp-Assertion; UUIDv7
    user_id             UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    session_id          UUID NOT NULL,        -- the Kratos session it was minted in
    action_class        TEXT NOT NULL CHECK (action_class ~ '^[a-z][a-z_]*$'),
    resource_ref        TEXT CHECK (resource_ref ~
                            '^[a-z][a-z_]*:[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$'),
    method              TEXT NOT NULL CHECK (method IN ('webauthn', 'totp')),
    issued_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at          TIMESTAMPTZ NOT NULL,
    consumed_at         TIMESTAMPTZ,
    consumed_by_action  TEXT CHECK (consumed_by_action ~ '^[a-z][a-z_]*(\.[a-z][a-z_]*)+$'),
    CHECK (expires_at > issued_at AND expires_at <= issued_at + interval '5 minutes'),
    CHECK ((consumed_at IS NULL) = (consumed_by_action IS NULL))
);
-- Freshness reads the newest issuance per (user, session); the sweep prunes by issued_at.
CREATE INDEX IF NOT EXISTS idx_stepup_session ON admin.stepup_assertions (user_id, session_id, issued_at DESC);
CREATE INDEX IF NOT EXISTS idx_stepup_issued  ON admin.stepup_assertions (issued_at);

CREATE TABLE IF NOT EXISTS admin.api_tokens (
    id             UUID PRIMARY KEY,          -- embedded in the presented token: akrpat_<id hex>_<secret>
    user_id        UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    name           TEXT NOT NULL CHECK (length(btrim(name)) BETWEEN 1 AND 100),
    token_hash     BYTEA NOT NULL CHECK (octet_length(token_hash) = 32),   -- HMAC-SHA-256 of the secret (DR-28)
    -- Read scopes and data.steward only: decisions are a person's interactive act (API.md section 2, D7).
    scopes         TEXT[] NOT NULL CHECK (cardinality(scopes) > 0 AND scopes <@
                       '{self,dashboard.read,audit.read,assessor.read,data.steward}'::text[]),
    expires_at     TIMESTAMPTZ NOT NULL,
    last_used_at   TIMESTAMPTZ,
    revoked_at     TIMESTAMPTZ,
    revoked_by     UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    revoke_reason  TEXT CHECK (revoke_reason IN ('owner', 'offboarded', 'dormant', 'expired_account', 'admin')),
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (expires_at > created_at AND expires_at <= created_at + interval '90 days'),
    CHECK ((revoked_at IS NULL) = (revoked_by IS NULL)),
    CHECK ((revoked_at IS NULL) = (revoke_reason IS NULL))
);
CREATE INDEX IF NOT EXISTS idx_api_tokens_user       ON admin.api_tokens (user_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_api_tokens_revoked_by ON admin.api_tokens (revoked_by);

CREATE TABLE IF NOT EXISTS admin.password_history (
    id           UUID PRIMARY KEY,
    user_id      UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    hash_sealed  BYTEA NOT NULL,   -- argon2id PHC string of a password the user set, sealed under `secret-str`
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_password_history_user ON admin.password_history (user_id, created_at DESC);

GRANT SELECT, INSERT, DELETE ON admin.stepup_assertions TO akr_app;   -- DELETE: maintenance_sweep past retention
GRANT UPDATE (consumed_at, consumed_by_action) ON admin.stepup_assertions TO akr_app;
GRANT SELECT, INSERT ON admin.api_tokens TO akr_app;
GRANT UPDATE (last_used_at, revoked_at, revoked_by, revoke_reason) ON admin.api_tokens TO akr_app;
GRANT SELECT, INSERT, DELETE ON admin.password_history TO akr_app;    -- DELETE: rows beyond auth.password_history
-- The token register for access reviews: everything but the hash. No read-only access to the other two tables.
GRANT SELECT (id, user_id, name, scopes, expires_at, last_used_at, revoked_at, revoke_reason, created_at)
    ON admin.api_tokens TO akr_readonly;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification, encrypted, note)
VALUES
 ('admin', 'api_tokens',       'token_hash',  'secret-str', FALSE, 'Keyed hash of a 256-bit secret; never serialised'),
 ('admin', 'password_history', 'hash_sealed', 'secret-str', TRUE,  'argon2id hash of a password the user set, sealed')
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', v.t, c, 'internal'
FROM (VALUES
  ('stepup_assertions', ARRAY['id','user_id','session_id','action_class','resource_ref','method','issued_at',
                              'expires_at','consumed_at','consumed_by_action']),
  ('api_tokens',        ARRAY['id','user_id','name','scopes','expires_at','last_used_at','revoked_at','revoked_by',
                              'revoke_reason','created_at']),
  ('password_history',  ARRAY['id','user_id','created_at'])
) AS v(t, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'stepup_assertions', 'R12', 'age',        'issued_at', 'stepup.retention_days; issue and use are also audit events (R10)'),
 ('admin', 'api_tokens',        'R9',  'retirement', NULL,        'Token register with owner, expiry and last use (SP-16); retirement = revoked or expired'),
 ('admin', 'password_history',  'R9',  'owner',      NULL,        'Only the newest auth.password_history rows per user are kept')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

```sql
-- migration 0007_identity_settings.sql
-- Phase 01. Identity, session, step-up and token settings. Floors carry the CBI minima (cyber-resilience controls,
-- June 2024, section 8.2): an administrator cannot weaken them (SP-14). Product limits are hard bounds in the
-- settings registry, not floors (DR-34).

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('auth.password_min_length',         '12', 'integer',
  '{"min": 8, "basis": "CBI cyber-resilience controls 2024, 8.2"}', TRUE,
  '{"ar": "الحد الأدنى لطول كلمة المرور", "en": "Minimum password length"}'),
 ('auth.password_min_classes',        '3', 'integer',
  '{"min": 3, "basis": "CBI cyber-resilience controls 2024, 8.2"}', TRUE,
  '{"ar": "عدد أنواع الأحرف المطلوبة في كلمة المرور", "en": "Character classes required in a password"}'),
 ('auth.password_max_age_days',       '90', 'integer',
  '{"max": 90, "basis": "CBI cyber-resilience controls 2024, 8.2"}', TRUE,
  '{"ar": "أقصى عمر لكلمة المرور (أيام)", "en": "Maximum password age (days)"}'),
 ('auth.password_history',            '3', 'integer',
  '{"min": 3, "basis": "CBI cyber-resilience controls 2024, 8.2"}', TRUE,
  '{"ar": "عدد كلمات المرور السابقة الممنوع تكرارها", "en": "Previous passwords that cannot be reused"}'),
 ('auth.lockout_max_failures',        '3', 'integer',
  '{"max": 3, "basis": "CBI cyber-resilience controls 2024, 8.2"}', TRUE,
  '{"ar": "عدد المحاولات الفاشلة قبل قفل الحساب", "en": "Failed sign-ins before lockout"}'),
 ('auth.lockout_mode',                '"timed"', 'string', NULL, TRUE,
  '{"ar": "طريقة فك القفل (بعد مدة أو من المسؤول)", "en": "Unlock mode (after a delay or by an administrator)"}'),
 ('auth.lockout_duration_s',          '900', 'duration_s', NULL, TRUE,
  '{"ar": "مدة القفل المؤقت (ثوانٍ)", "en": "Timed lockout duration (seconds)"}'),
 ('auth.temp_password_ttl_h',         '72', 'integer', NULL, TRUE,
  '{"ar": "صلاحية كلمة المرور المؤقتة (ساعات)", "en": "Temporary password validity (hours)"}'),
 ('auth.dormancy_days',               '60', 'integer', NULL, TRUE,
  '{"ar": "تعطيل الحساب غير المستخدم بعد (أيام)", "en": "Disable accounts unused for (days)"}'),
 ('auth.login_ip_rate_per_min',       '20', 'integer', NULL, FALSE,
  '{"ar": "محاولات الدخول المسموحة لكل عنوان في الدقيقة", "en": "Sign-in attempts per address per minute"}'),
 ('auth.policy_version',              '1', 'integer', NULL, TRUE,
  '{"ar": "إصدار تعهد السرية", "en": "Confidentiality undertaking version"}'),
 ('auth.policy_text',
  '{"ar": "ألتزم بالحفاظ على سرية كل ما أطّلع عليه في هذا النظام من بلاغات وقضايا وتنبيهات وبيانات عملاء، ولن أُبلغ أي عميل أو أي شخص من خارج المخوّلين بالقضية بأن بلاغاً قد قُدّم أو قد يُقدّم أو بوجود قضية (قانون مكافحة غسل الأموال وتمويل الإرهاب رقم 39 لسنة 2015). أستخدم هذا النظام لأداء مهامي المكلّف بها فقط، وجميع إجراءاتي فيه مسجّلة.", "en": "I will keep confidential everything I see in this system about reports, cases, alerts and customers. I will not tell any customer, or anyone outside the people authorised for a case, that a report has been or may be made or that a case exists (Anti-Money Laundering and Counter-Terrorism Financing Law No. 39 of 2015). I use this system only for my assigned duties, and my actions in it are recorded."}',
  'json', NULL, TRUE,
  '{"ar": "نص تعهد السرية", "en": "Confidentiality undertaking text"}'),
 ('session.idle_timeout_s',           '900', 'duration_s', NULL, TRUE,
  '{"ar": "مهلة الخمول قبل إنهاء الجلسة (ثوانٍ)", "en": "Idle time-out (seconds)"}'),
 ('session.absolute_lifetime_s',      '28800', 'duration_s', NULL, TRUE,
  '{"ar": "أقصى مدة للجلسة (ثوانٍ)", "en": "Maximum session lifetime (seconds)"}'),
 ('session.warning_s',                '60', 'duration_s', NULL, FALSE,
  '{"ar": "مدة التنبيه قبل انتهاء الجلسة (ثوانٍ)", "en": "Warning before the idle time-out (seconds)"}'),
 ('session.heartbeat_min_interval_s', '30', 'duration_s', NULL, FALSE,
  '{"ar": "أقل فاصل بين إشارات النشاط (ثوانٍ)", "en": "Minimum interval between activity signals (seconds)"}'),
 ('session.cache_ttl_s',              '30', 'duration_s', NULL, FALSE,
  '{"ar": "مدة التخزين المؤقت لبيانات الجلسة (ثوانٍ)", "en": "Session cache lifetime (seconds)"}'),
 ('rate.session_per_min',             '600', 'integer', NULL, FALSE,
  '{"ar": "الطلبات المسموحة لكل جلسة في الدقيقة", "en": "Requests per session per minute"}'),
 ('rate.token_per_min',               '120', 'integer', NULL, FALSE,
  '{"ar": "الطلبات المسموحة لكل رمز وصول في الدقيقة", "en": "Requests per access token per minute"}'),
 ('stepup.ttl_s',                     '300', 'duration_s', NULL, TRUE,
  '{"ar": "صلاحية تأكيد الهوية الإضافي (ثوانٍ)", "en": "Step-up assertion validity (seconds)"}'),
 ('stepup.fresh_window_s',            '600', 'duration_s', NULL, TRUE,
  '{"ar": "مدة اعتبار تأكيد الهوية الإضافي حديثاً (ثوانٍ)", "en": "Step-up freshness window (seconds)"}'),
 ('stepup.max_failures',              '5', 'integer', NULL, TRUE,
  '{"ar": "محاولات التأكيد الفاشلة قبل إنهاء الجلسة", "en": "Failed step-ups before the session is ended"}'),
 ('stepup.retention_days',            '30', 'integer', NULL, FALSE,
  '{"ar": "مدة الاحتفاظ بسجل تأكيدات الهوية (أيام)", "en": "Step-up ledger retention (days)"}'),
 ('pat.max_ttl_days',                 '90', 'integer', NULL, TRUE,
  '{"ar": "أقصى صلاحية لرمز الوصول الشخصي (أيام)", "en": "Maximum personal token lifetime (days)"}'),
 ('pat.max_per_user',                 '5', 'integer', NULL, TRUE,
  '{"ar": "أقصى عدد لرموز الوصول الشخصية لكل مستخدم", "en": "Personal tokens per user"}'),
 ('schedule.identity_sweep',          '"0 0 3 * * *"', 'cron', NULL, FALSE,
  '{"ar": "موعد مراجعة الحسابات اليومية", "en": "Daily account sweep time"}')
ON CONFLICT (key) DO NOTHING;

-- The dark palette is a proposal until the owner approves it (DS-O1); preferences refuse `dark` while this is off.
INSERT INTO admin.feature_flags (key, enabled, label) VALUES
 ('ui.dark_theme', FALSE, '{"ar": "المظهر الداكن", "en": "Dark theme"}')
ON CONFLICT (key) DO NOTHING;
```

Notes:
- The `scope` CHECK on `role_grants` rules out an empty object, so a grant never reads as "all branches" by accident;
  `{"all_branches": true}` is written explicitly (the bootstrap grants use it).
- `password_history` keeps a sealed copy of the current password's hash too (history includes the current password);
  the column is encrypted so a database dump or backup holds no crackable hash, and the table is granted to
  `akr_app` only.
- The governed keys cannot change until phase 04 (change requests); their defaults meet the minima. Phase 03 shows
  them read-only on A2 with the floor and its basis.

## 2. DDD Implementation (Rust)

Crate placement per the phase 00 workspace: entities, value objects and traits in `domain::identity`; Pg impls in
`db`; the Kratos and Hydra HTTP adapters in `common::ory` (an outbound adapter shared by `akr-api` and `akr-ctl`,
keeping `db` PostgreSQL-only); the Valkey limiter in `common::ratelimit`; the auth layer, extractors and routers in
`api`. Keyed hashes use the phase 00 crypto port with three new purposes, `csrf`, `token` and `login_identifier`, each
an HKDF-SHA-256 subkey of the OpenBao KV entry `keys/auth-hmac`.

- **Domain entities** (`domain`, derives `Debug, Serialize, Deserialize, FromRow, ToSchema`):
  - `User { id, kind, display_name: I18nText, email, status, disabled_reason, vetting_status, vetting_expires_on,
    branch_id, locale, preferences: PreferencesV1, owner_user_id, hydra_client_id, expires_at, forced_change,
    temp_password_expires_at, password_changed_at, failed_logins, locked_until, last_authenticated_at,
    policy_ack_version, policy_ack_at, created_at, updated_at, version }`
  - `RoleGrant { id, user_id, role: RoleCode, scope: GrantScope, source, change_request_id, valid_from, valid_to,
    granted_by, revoked_at, revoked_by, created_at }`
  - `Delegation { id, from_user, to_user, scope: DelegationScope, valid_from, valid_to, reason_code, created_by,
    created_at, ended_at, ended_by }`
  - `Pool { id, name: I18nText, kind: PoolKind, branch_ids, archived_at, created_at, updated_at, version }`
  - `StepUpAssertion { id, user_id, session_id, action_class, resource_ref, method, issued_at, expires_at,
    consumed_at, consumed_by_action }`
  - `ApiToken { id, user_id, name, scopes, expires_at, last_used_at, revoked_at, revoked_by, revoke_reason,
    created_at }` (`token_hash` lives only in a non-serialised `db` row struct and never leaves the crate)
- **Value objects:** `RoleCode` (the nineteen codes of MODULES.md section 0), `Scope` (API.md section 2),
  `GrantScope { all_branches, branches, pools }`, `DelegationScope { roles }`, `PoolKind`, `Aal { Aal1, Aal2 }`,
  `AuthVia { Session, Pat, ClientCredentials }`, `ForcedChange { FirstLogin, Recovery }`, `PendingStep {
  ChangePassword, EnrollMfa, AcknowledgePolicy }`, `ActionClass` and `StepUpAction { name: &'static str, class }`
  (section 3), `I18nText { ar, en }`, `PreferencesV1 { prose_digits: Western | ArabicIndic, density: Option<Compact |
  Standard | Relaxed> (None = the role default, DESIGN-SYSTEM.md 4.2), default_queue_id: Option<Uuid>, shortcuts: {
  enabled, modifier: None | Ctrl | Alt }, theme: Light | Dark | System }` (unknown keys refused).
- **`AuthContext`** (request extension read by every later phase): `{ principal_id, kind, via: AuthVia, session_id:
  Option<Uuid>, token_id: Option<Uuid>, aal, roles: Vec<EffectiveRole>, scopes: BTreeSet<Scope>, branch_scope:
  BranchScope, pending: Vec<PendingStep>, locale, session_issued_at: Option<DateTime<Utc>> }` with `EffectiveRole {
  code, scope: GrantScope, grant_id, delegation: Option<{ delegation_id, on_behalf_of }> }` and `BranchScope { All |
  Branches(Vec<Uuid>) }` (the union over effective roles). `on_behalf_of` is what phase 02 writes into
  `audit_log.on_behalf_of` for an act done under a delegated role.
- **Scope mapping** (`common::scopes::scopes_for_roles`): the API.md section 2 table encoded once (every principal
  holds `self`); a unit test per table row. Sessions get the scopes of their effective roles; tokens get the
  intersection of their own scopes with their owner's current scopes.
- **Create/Update DTOs** (`Deserialize, ToSchema`; Update fields all `Option<T>`): `UpdatePreferences { locale,
  prose_digits, density, default_queue_id, shortcuts, theme }`, `CreateTokenRequest { name, scopes, expires_at }`,
  `PolicyAckRequest { version }`, `PasswordChangeRequest { new_password }`.
- **Repository traits** (`domain::repositories`):
  - `UserRepository { get(id); by_email(email); by_hydra_client(client_id); insert(tx, NewUser);
    record_login_failure(tx, id, max_failures, lock_until) -> LockState; clear_login_failures(tx, id, at);
    set_forced_change(tx, id, Option<ForcedChange>, temp_expiry); set_password_changed(tx, id, at);
    acknowledge_policy(tx, id, version, at); update_preferences(tx, id, locale, prefs, expected_version) -> User;
    disable(tx, id, reason); touch_service_use(id, at); dormant_candidates(cutoff, limit);
    expired_candidates(now, limit) }`
  - `GrantRepository { effective_roles(user_id, at) -> Vec<EffectiveRole>; insert_bootstrap(tx, NewBootstrapGrant);
    active_holders(role) -> Vec<Uuid> }`
  - `DelegationRepository { active_to(user_id, at); active_from(user_id, at) }`
  - `PoolRepository { by_ids(ids) -> Vec<Pool> }`
  - `StepUpRepository { insert(tx, NewAssertion); consume(tx, ConsumeSpec) -> Option<StepUpAssertion>;
    latest_issue(user_id, session_id) -> Option<DateTime<Utc>> }`
  - `TokenRepository { insert(tx, NewTokenRow); list_for_user(user_id); find_for_auth(id) -> Option<TokenAuthRow>;
    count_active(user_id, now); revoke(tx, id, user_id, actor, reason) -> bool; revoke_all_for_user(tx, user_id,
    actor, reason) -> Vec<Uuid>; touch(id, at) }`
  - `PasswordHistoryRepository { recent(user_id, n) -> Vec<Sealed>; push(tx, user_id, Sealed, keep: n) }`
  - `IdentityProvider` (the Kratos facade): browser flows (SPA mode) `init_login(relay, refresh, aal) -> Flow`,
    `submit_password(flow, relay, identifier, password) -> PasswordOutcome { Session | SecondFactorRequired | Invalid
    }`, `submit_second_factor(flow, relay, Proof { Totp(code) | WebAuthn(assertion) })`, `init_settings(relay)`,
    `settings_password(flow, relay, new)`, `settings_totp_confirm(flow, relay, code)`,
    `settings_webauthn_register(flow, relay, credential, label)`, `settings_remove(flow, relay, CredentialRef)`,
    `whoami(cookie) -> KratosSession { id, identity_id, aal, issued_at, authenticated_at, expires_at }`; admin API
    `create_identity(traits, temp_password) -> Uuid`, `identity_with_credentials(id) -> IdentityCredentials`,
    `set_state(id, Active | Inactive)`, `set_password(id, password)`, `remove_second_factors(id)`,
    `list_sessions(id)`, `revoke_session(session_id)`, `revoke_all_sessions(id)`, `delete_identity(id)`
    (compensation only). `Relay` merges the browser's cookies with the flow's own cookies on every submit (DBS phase
    01 s.8.9 defect 2) and returns Kratos's `Set-Cookie` headers with `Domain` rewritten to the public host.
- **Repository impls** (`db`): `PgUserRepository` (`record_login_failure` is one `UPDATE ... SET failed_logins =
  failed_logins + 1, locked_until = CASE WHEN failed_logins + 1 >= $2 THEN $3 ELSE locked_until END RETURNING
  failed_logins, locked_until`), `PgGrantRepository` (`effective_roles` below), `PgDelegationRepository`,
  `PgPoolRepository`, `PgStepUpRepository` (`consume` is the single conditional UPDATE of section 3),
  `PgTokenRepository` (`find_for_auth` joins the owner's `kind`, `status` and `expires_at` in one query),
  `PgPasswordHistoryRepository`. `KratosClient` implements `IdentityProvider`; `InMemoryIdentityProvider` (tests)
  mints a new TOTP secret per settings flow, as Kratos does, so the flow-carrying contract is enforced in `cargo
  test` too (DBS s.8.9 defect 3).
- **Effective roles** (`PgGrantRepository::effective_roles`; a delegated role counts only while the delegator still
  holds it and is active):

```sql
SELECT g.id AS grant_id, g.role, g.scope, NULL::uuid AS delegation_id, NULL::uuid AS on_behalf_of
FROM admin.role_grants g
WHERE g.user_id = $1 AND g.revoked_at IS NULL AND g.valid_from <= $2 AND (g.valid_to IS NULL OR g.valid_to > $2)
UNION ALL
SELECT g.id, g.role, g.scope, d.id, d.from_user
FROM admin.delegations d
JOIN admin.users fu ON fu.id = d.from_user AND fu.status = 'active'
JOIN admin.role_grants g
  ON g.user_id = d.from_user AND (d.scope -> 'roles') ? g.role
 AND g.revoked_at IS NULL AND g.valid_from <= $2 AND (g.valid_to IS NULL OR g.valid_to > $2)
WHERE d.to_user = $1 AND d.ended_at IS NULL AND d.valid_from <= $2 AND d.valid_to > $2;
```

- **utoipa schemas:** `LoginRequest { identifier, password }`; `MfaRequest { flow_id, flow_csrf_token, method,
  totp_code?, webauthn_assertion? }`; `LoginOutcome { state: mfa_required | setup_required | authenticated, flow_id?,
  flow_csrf_token?, methods?, webauthn_request_options?, pending?, csrf_token? }`; `SessionView { principal_id, aal,
  issued_at, idle_expires_at, absolute_expires_at, warning_s, pending, roles, branch_scope, locale }`;
  `StepUpStartRequest { action_class, resource_ref? }`; `StepUpChallenge { challenge_id, flow_csrf_token,
  action_class, methods, webauthn_request_options?, expires_at }`; `StepUpVerifyRequest { challenge_id,
  flow_csrf_token, method, totp_code?, webauthn_assertion? }`; `StepUpAssertionView { assertion_id, action_class,
  resource_ref?, expires_at }`; `MeView { id, kind, display_name, email, locale, preferences, roles:
  [EffectiveRoleView], scopes, branch_id, pools: [PoolRef], delegations: { to_me, from_me }, mfa: MfaView, pending,
  policy: { required_version, acknowledged_version, text? }, password_expires_at, account_expires_at }` (`text` only
  while `acknowledge_policy` is pending); `MfaView { credentials: [CredentialView], min_factors }`; `CredentialView {
  id, method, label, created_at }`; `SessionRow { id, current, aal, authenticated_at, issued_at, expires_at, ip,
  user_agent }`; `TotpStart { flow_id, flow_csrf_token, secret, otpauth_qr_svg }`; `TotpConfirm { flow_id,
  flow_csrf_token, code }`; `WebauthnStart { flow_id, flow_csrf_token, creation_options }`; `WebauthnConfirm {
  flow_id, flow_csrf_token, credential, label }`; `TokenCreated { id, token, name, scopes, expires_at }`; `TokenView
  { id, name, scopes, expires_at, last_used_at, revoked_at, created_at }`.
- **Route table** (paths as served; "session" = cookie session only, a bearer gets 403 `E_SCOPE`):

| Method | Path | Scope | Description |
|-|-|-|-|
| POST | `/api/v1/auth/login` | none (pre-auth) | Password step; returns `LoginOutcome` (`mfa_required` with the second-factor flow, or `setup_required` for an identity with no factor yet) |
| POST | `/api/v1/auth/mfa` | none (flow) | Second factor (WebAuthn or TOTP) on the AAL2 flow; returns `authenticated` or `setup_required`; mints the CSRF cookie |
| GET | `/api/v1/auth/session` | self (session) | `SessionView`: AAL, idle and absolute expiry, pending steps, roles, branch scope; never extends the idle clock |
| POST | `/api/v1/auth/session/heartbeat` | self (session) | Records user activity (idle clock) at most once per `session.heartbeat_min_interval_s`; returns `SessionView` |
| POST | `/api/v1/auth/logout` | self (session) | Ends the current session and clears the cookies; 204 |
| POST | `/api/v1/auth/step-up` | self (session, AAL2) | Starts a step-up challenge for an action class (optional resource); WebAuthn options first |
| POST | `/api/v1/auth/step-up/verify` | self (session, AAL2) | Completes it; returns a single-use assertion id valid `stepup.ttl_s` |
| GET | `/api/v1/me` | self (session or token) | `MeView`: profile, roles with delegations, scopes, pools, factors, pending steps, undertaking |
| PATCH | `/api/v1/me/preferences` | self (session) | Language, prose digits, density, default queue, shortcuts, theme; `If-Match` |
| POST | `/api/v1/me/policy-acknowledgement` | self (session) | Acknowledge the current confidentiality undertaking version (SP-101) |
| GET | `/api/v1/me/sessions` | self (session) | Own active sessions, the current one flagged |
| DELETE | `/api/v1/me/sessions/{id}` | self (session) | Revoke one own session (the current one = logout); 404 for any other id |
| GET | `/api/v1/me/credentials` | self (session) | Own second factors with stable ids |
| POST | `/api/v1/me/credentials/password` | self (session) | Change password (forced first login, recovery, expiry, or voluntary) |
| POST | `/api/v1/me/credentials/totp` | self (session) | Start TOTP enrolment: settings flow, secret and QR (`Cache-Control: no-store`) |
| POST | `/api/v1/me/credentials/totp/confirm` | self (session) | Confirm with a code on the same flow |
| POST | `/api/v1/me/credentials/webauthn` | self (session) | Start security-key registration: settings flow and creation options |
| POST | `/api/v1/me/credentials/webauthn/confirm` | self (session) | Finish registration with the credential and a label |
| DELETE | `/api/v1/me/credentials/{id}` | self (session) | Remove one factor; **SU** (`credential_change`); 409 `E_MIN_FACTORS` if none would remain |
| GET | `/api/v1/me/tokens` | self (session) | Own personal access tokens (never the hash) |
| POST | `/api/v1/me/tokens` | self (session) | Create a token with a scope subset; **SU** (`token_create`); the secret is returned once |
| DELETE | `/api/v1/me/tokens/{id}` | self (session) | Revoke an own token; 204 |

  Twenty-two operations. `/me/notifications` and `/me/notifications/{id}/read` (API.md 3.1) belong with the
  notifications tables in phase 22. The administrator routes for users, unlock and credential reset are phase 03
  (API.md 3.2).
- **Idempotency exemptions** (filled into the phase 00 list, each with its reason, asserted by a test): `POST
  /auth/login` and `POST /auth/mfa` (no principal; Kratos flow ids give replay semantics); `POST /auth/logout` and
  `POST /auth/session/heartbeat` (naturally idempotent); `POST /auth/step-up` and `POST /auth/step-up/verify` (flow ids
  and the single-use ledger); `POST /me/credentials/password` (the body carries a password, and its fast hash must
  never be stored); `POST /me/credentials/totp` (the response carries a TOTP seed; the call persists nothing); `POST
  /me/tokens` (the response carries the token secret; the consumed step-up assertion makes a repeat impossible
  without a new step-up). Every other mutation above requires `Idempotency-Key` (DR-28).
- **Routers:** `api::routes::auth::router()` and `api::routes::me::router()` (`OpenApiRouter<AppState>`) merged in
  `build_app`; the auth layer (section 3) fills the phase 00 slot between metrics and the authz slot.

## 3. Business Logic

**The auth layer** (`api::auth`; runs on every `/api/v1` request, in this order):
1. Unsafe methods (POST, PUT, PATCH, DELETE): a request carrying a cookie must carry `Origin` equal to
   `AKR_PUBLIC_ORIGIN`, else 403 `E_CSRF_INVALID`; a body must be `application/json` (no form posts), else 400
   `E_BAD_REQUEST`. Bearer requests are not browsers and skip the `Origin` rule.
2. Credentials, at most one: `Authorization: Bearer akrpat_...` -> `TokenService::authenticate`; any other bearer ->
   `ClientCredentials::authenticate` (a JWT); else the `akr_session` cookie -> `SessionService::resolve`. A bearer
   together with a session cookie -> 400 `E_BAD_REQUEST`. A failed resolve on a public route is anonymous.
3. Anonymous requests reach only `PUBLIC_ROUTES` (`POST /api/v1/auth/login`, `POST /api/v1/auth/mfa`; infra routes
   live outside `/api/v1`); anything else -> 401 `E_UNAUTHENTICATED`. A test walks every operation of the OpenAPI
   document without credentials and expects 401 except for those two (SP-71).
4. CSRF: a cookie-authenticated unsafe request needs `X-CSRF-Token` equal to the `akr_csrf` cookie and to
   `base64url(HMAC(csrf subkey, session_id))`, compared in constant time, else 403 `E_CSRF_INVALID`. The cookie
   (`Secure`, `SameSite=Strict`, readable by the page, not `HttpOnly`) is minted wherever a usable session is handed
   out (DBS s.8.9 defect 1) and re-minted on any authenticated GET that arrives without a valid one (Kratos keeps the
   session id when the AAL rises, so this is a repair, not a rotation).
5. Confinement: with `pending` non-empty only the setup routes answer (`GET /auth/session`, `GET /me`, `POST
   /auth/logout`, `POST /auth/session/heartbeat`, `POST /me/credentials/password`, the four enrolment routes, `POST
   /me/policy-acknowledgement`); anything else -> 403 `E_CREDENTIAL_CHANGE_REQUIRED`.
6. Session-only routes reached with a token or a client credential -> 403 `E_SCOPE`.
7. Rate limits (`common::ratelimit`, GCRA in Valkey): per client IP on the two public routes
   (`auth.login_ip_rate_per_min`), per session (`rate.session_per_min`), per token or client (`rate.token_per_min`);
   429 `E_RATE_LIMITED` with `Retry-After`. The client IP comes from `X-Forwarded-For` only when the peer is in
   `AKR_TRUSTED_PROXIES` (the edge).
8. Activity: a cookie-authenticated unsafe request that passed 1-7 touches the idle clock; GETs and the event stream
   never do (DR-31).
9. `AuthContext` goes into the request extensions; the idempotency middleware keys on `principal_id`; handlers take
   the `Principal`, `SessionPrincipal` (cookie only) or `Aal2Principal` extractors.

**`SessionService`** (`domain::identity::session`, over Kratos and Valkey):

`resolve(cookie)`:
1. `kc:{sha256(cookie)}` -> session id and `sess:{session_id}` -> the cached `KratosSession` (`session.cache_ttl_s`);
   on a miss call `whoami`. No session, or Kratos's `session_aal2_required` (the identity has a factor and the session
   is still AAL1), -> unauthenticated.
2. Absolute limit: `now >= issued_at + session.absolute_lifetime_s` -> revoke at Kratos, delete the keys, audit
   `auth.session_expired {reason: absolute}`, 401.
3. Idle limit: `idle:{session_id}` missing, or older than `session.idle_timeout_s` -> the same with `reason: idle`. A
   missing key counts as expired, so a Valkey restart signs everyone out (DR-31).
4. Principal: `authctx:{session_id}:{gen}`, where `gen = authgen:{user_id}`. On a miss load the mirror row (kind
   human, status active, `expires_at` null or in the future, otherwise revoke the session, set the Kratos identity
   inactive if it is still active, and answer 401), the effective roles (section 2), pools, scopes, branch scope and
   pending steps, and cache the result for `session.cache_ttl_s`.
5. Pending steps, in this order: `change_password` when `forced_change` is set or `password_changed_at +
   auth.password_max_age_days` has passed; `enroll_mfa` when the identity has no second factor (decided on the
   credential content of the admin read with `include_credential`: TOTP when `totp_url` is present, WebAuthn when the
   credential list is non-empty; DBS s.8.4 and s.8.9 defect 4); `acknowledge_policy` when `policy_ack_version` is
   null or below `auth.policy_version`.
6. AAL: AAL1 is accepted only while `enroll_mfa` is pending (Kratos `highest_available` already refuses AAL1 to
   anyone holding a factor); otherwise the session must be AAL2.

`touch(session_id)` = `SET idle:{sid} <now> EX session.idle_timeout_s + 60`. `heartbeat(ctx)` touches when the last
touch is older than `session.heartbeat_min_interval_s` and returns the `SessionView`. `invalidate_user(user_id)` =
`INCR authgen:{user_id}`; every service that changes a principal's grants, status, factors, password or
acknowledgement calls it (phases 03 and 04 too), so cached contexts die at once rather than at the TTL.
`list(ctx)` reads Kratos's admin sessions for the identity (id, AAL, authenticated and issued times, expiry, newest
device IP and user agent). `revoke(ctx, id)`: the id must be one of the caller's sessions, else 404 `E_NOT_FOUND`;
revoke at Kratos, delete `sess:` and `idle:` keys, audit `session.revoked {by: self}`; the current session also gets
its cookies cleared.

**`IdentityService`** (`domain::identity`, over `IdentityProvider`, the repositories and `PasswordPolicy`):

`login(LoginRequest)`:
1. Normalise the identifier (trim, NFKC, lower case); over 254 characters -> 422 `E_VALIDATION`.
2. Per-IP bucket (layer step 7).
3. Lock check. Mirror row by email with `locked_until > now()` -> 429 `E_RATE_LIMITED` (`Retry-After` = the remaining
   seconds in `timed` mode, absent in `admin` mode). No row: `lockid:{hk(identifier)}` at or over
   `auth.lockout_max_failures` -> the same response, so an unknown identifier locks exactly like a real one. Audit
   `auth.login_failed {stage: locked}`.
4. Kratos: init the browser login flow in SPA mode (`Accept: application/json`) and submit `{method: password,
   identifier, password, csrf_token}` with the relay.
5. Kratos 400 -> failure (step 9). Kratos 200 -> an AAL1 session: the identity has no second factor. Kratos 422
   `browser_location_change_required` -> a factor is enrolled: init the `?aal=aal2` flow server-side; the redirect URL
   never reaches the browser (DBS s.8.2).
6. On a Kratos success, before any response: no mirror row for the email, or kind not human -> revoke the session,
   error log `identity.mirror_missing`, uniform 401; status disabled -> revoke, audit `auth.login_failed {stage:
   disabled}`, uniform 401; `expires_at` passed -> the same with `expired_account`; `forced_change` set with
   `temp_password_expires_at` passed -> the same with `temp_expired`; `LeavePolicy::blocks(user, now)` (always false
   until the leave ledger exists, SP-17) -> the same with `on_leave`.
7. Stash `mfaflow:{flow_id}` = `{user_id}` for the flow's lifetime (10 minutes); the second-factor step trusts only
   this, never a client-supplied identity.
8. Respond: the 422 path -> `{state: "mfa_required", flow_id, flow_csrf_token, methods, webauthn_request_options}`
   (methods are the identity's enrolled factors, WebAuthn first; the options come from the flow's WebAuthn trigger
   node). The AAL1 path -> mint the CSRF cookie, touch the idle clock, `{state: "setup_required", pending,
   csrf_token}`. Kratos cookies are relayed with `Domain` set to the public host.
9. Failure accounting (password or second factor): a known user -> `record_login_failure`; reaching
   `auth.lockout_max_failures` sets `locked_until = now() + auth.lockout_duration_s` (or `'infinity'` in `admin`
   mode) and emits `auth.account_locked`. Unknown -> `INCR lockid:{hk}` with the same duration as TTL (24 hours in
   `admin` mode). Audit `auth.login_failed {stage, identifier_hk}`: only the keyed hash, never the raw identifier,
   because people type passwords into the identifier field. One 401 `E_UNAUTHENTICATED` body for every stage.

`mfa(MfaRequest)`:
1. `mfaflow:{flow_id}` must exist -> else 401.
2. TOTP replay guard: `SET totp:{user_id}:{sha256(code)} NX EX 90`; already present -> a failed factor. Kratos
   accepts a reused code inside its window (DBS s.8.12), and a time-based code must be accepted once (DR-32).
3. Submit `{method: totp, totp_code}` or `{method: webauthn, webauthn_login: <assertion>}` to the AAL2 flow with the
   relay.
4. Failure -> failure accounting (login step 9) with `stage: mfa` and the method; 401.
5. Success: one transaction runs `clear_login_failures` (counter 0, lock cleared, `last_authenticated_at = now()`)
   and emits `auth.login_succeeded {method, aal: aal2}`; touch the idle clock; mint the CSRF cookie; respond
   `setup_required` with the pending steps, or `authenticated`.

`logout(ctx)`: revoke the current Kratos session, delete its keys, clear `akr_session`, the Kratos CSRF cookie and
`akr_csrf`, audit `auth.logout`; 204.

`change_password(ctx, new_password)`:
1. `PasswordPolicy::check`: at least `auth.password_min_length` and at most 128 characters; at least
   `auth.password_min_classes` of four classes (lower, upper, digit, other), where caseless letters (Arabic, Kurdish,
   Persian) count as lower and Arabic-Indic digits as digits; not similar to the identifier (Kratos
   `identifier_similarity_check_enabled`); not equal to any of the newest `auth.password_history` passwords (argon2id
   verify against the sealed hashes in `spawn_blocking`, at most two at once to cap memory). Failures -> 422
   `E_VALIDATION` at `/new_password` with `too_short`, `too_long`, `too_few_classes`, `similar_to_identifier` or
   `reused`.
2. Kratos settings flow: init and submit `{method: password, password}` with the relay. Kratos
   `session_refresh_required` (the session authenticated more than `privileged_session_max_age` ago) -> 401
   `E_STEPUP_REQUIRED` with action class `credential_change`: the client runs a step-up, which refreshes the session,
   and retries. A first-login session is fresh and passes.
3. Transaction: push the new argon2id hash (Kratos's parameters: 128 MB, 3 iterations, 4 lanes), sealed under
   `secret-str`, and delete the rows beyond the newest `auth.password_history`; `password_changed_at = now()`;
   `forced_change` and `temp_password_expires_at` cleared; audit `auth.password_changed {forced}`. Kratos's
   `revoke_active_sessions` hook ends the user's other sessions.
4. `invalidate_user`; return the new `SessionView`. If the transaction fails after Kratos succeeded, the password
   works and the pending step stays, so a retry is safe.

`enroll_totp_start(ctx)`: init a settings flow; read the `totp_secret_key` and `totp_qr` nodes; return the flow id and
flow CSRF token with the secret, because Kratos mints a new secret per flow and confirmation must use the same flow
(DBS s.8.9 defect 3). `enroll_totp_confirm`: submit `{method: totp, totp_code}` on that flow; a wrong code -> 422
`E_VALIDATION` at `/code`; success -> audit `auth.mfa_enrolled {method: totp, credential_id}`, `invalidate_user`
(Kratos raises the session to AAL2 in place, DBS s.8.5). `enroll_webauthn_start` and `enroll_webauthn_confirm`: the
same shape with the registration trigger's creation options and `{method: webauthn, webauthn_register,
webauthn_register_displayname}`; audit `auth.mfa_enrolled {method: webauthn, credential_id}`.

`list_credentials(ctx)`: the admin read with `include_credential=totp&include_credential=webauthn`, content-based as
in `resolve` step 5; stable ids (UUIDv5 over the identity id and `totp`, or over the WebAuthn credential id); never a
public key or secret; `min_factors = 1`.

`remove_credential(ctx, id, assertion)`: unknown id -> 404; it is the last factor -> 409 `E_MIN_FACTORS` (the account
would fall back to password only); then one transaction: `StepUpService::consume` (`credential.remove`, class
`credential_change`), the Kratos settings removal (`totp_unlink` or the WebAuthn remove node; a Kratos failure rolls
the transaction back, so the assertion is not spent), audit `auth.mfa_removed {method, credential_id, stepup_id}`;
`invalidate_user`. Losing the only device is the administrator credential reset of phase 03 (ADM and SEC), never
self-service (M1).

`acknowledge_policy(ctx, version)`: `version` must equal `auth.policy_version`, else 422 `E_VALIDATION` at `/version`
(`stale`); transaction: set `policy_ack_version` and `policy_ack_at`, audit `auth.policy_acknowledged {version}`;
`invalidate_user`. Raising `auth.policy_version` (by change request, phase 04) makes every user acknowledge again at
their next request.

`update_preferences(ctx, UpdatePreferences, If-Match)`: validate against `PreferencesV1` (`theme: dark` only while the
flag `ui.dark_theme` is on; `default_queue_id` must be null until queues exist in phase 17); the `If-Match` version
check -> 409 `E_VERSION_CONFLICT` with the current version, missing -> 428 `E_PRECONDITION_REQUIRED`; update with
`version + 1`; audit `user.preferences_updated {before, after}`; respond with `ETag`. Only `If-Match` changes bump
`version`; sign-in bookkeeping never does, so an open screen's ETag is not invalidated by a login elsewhere.

**`StepUpService`** (normative: every SU route of every later phase calls `consume`; DR-29):

Action classes, closed in code (`common::stepup::ActionClass`): `token_create`, `credential_change`, `identity_admin`,
`access_change`, `decision`, `approval`, `filing`, `export`, `backfill`, `break_glass`, `signing`, `record_open`. Each
phase declares its actions as `StepUpAction { name, class }` constants; this phase declares `token.create`
(`token_create`) and `credential.remove` (`credential_change`). A test asserts that every SU operation in the OpenAPI
document names a declared action.

`start(ctx, StepUpStartRequest)`:
1. Cookie session, AAL2, nothing pending; a token or client credential -> 403 `E_SCOPE` (they never step up).
2. Kratos browser login flow with `refresh=true&aal=aal2` (DBS s.8.2); stash `stepup:{flow_id}` = `{user_id,
   session_id, action_class, resource_ref}` for 10 minutes, so verification cannot switch class or session.
3. Return `{challenge_id: flow_id, flow_csrf_token, action_class, methods, webauthn_request_options, expires_at}`.

`verify(ctx, StepUpVerifyRequest)`:
1. The stash must exist and match the caller's user and session -> else 401 `E_STEPUP_REQUIRED`.
2. TOTP replay guard (as in `mfa`).
3. Submit the proof to the refresh flow with the relay.
4. Failure -> `INCR stepup_fail:{session_id}`, audit `stepup.denied {action_class, method}`; at `stepup.max_failures`
   revoke the session and audit `session.revoked {reason: stepup_failures}`; 401 `E_STEPUP_REQUIRED`.
5. Success -> insert the ledger row (`id` UUIDv7, `expires_at = now() + stepup.ttl_s`), delete the stash, audit
   `stepup.issued {assertion_id, action_class, method}`, return `{assertion_id, action_class, resource_ref,
   expires_at}`.

`consume(tx, ctx, header, action: StepUpAction, resource_ref: Option<&str>) -> StepUpReceipt`, inside the protected
action's own transaction:
1. `X-StepUp-Assertion` missing or not a UUID, or `ctx.via` not a session -> 401 `E_STEPUP_REQUIRED`.
2. `UPDATE admin.stepup_assertions SET consumed_at = now(), consumed_by_action = $5 WHERE id = $1 AND user_id = $2
   AND session_id = $3 AND action_class = $4 AND consumed_at IS NULL AND expires_at > now() AND (resource_ref IS NULL
   OR resource_ref = $6) RETURNING id, method`; zero rows (expired, spent, another session or user, another class or
   resource) -> 401 `E_STEPUP_REQUIRED`. The row lock settles concurrent consumers: exactly one wins.
3. Return `{assertion_id, method}`; the action's audit event records `stepup_id`. A failed action rolls the
   consumption back with its transaction, so the person can retry with the same assertion inside its five minutes.

`fresh_until(ctx) -> Option<DateTime<Utc>>` = the newest `issued_at` for the caller's user and session plus
`stepup.fresh_window_s`: the "fresh step-up within 10 minutes" for restricted and suspicion opens (SP-15). Phase 03
puts it in the Cedar context. Reading `fresh_until` spends nothing; an assertion of the class `record_open` is spent
by `consume` like one of any other class, in the handler of the open (phase 17 section 3.10).

**`TokenService`** (personal access tokens; DR-28):

Format `akrpat_<token id, 32 hex>_<base64url of 32 random bytes>`; the prefix lets secret scanning find a leaked token
(SP-72); only `HMAC-SHA-256(token subkey, secret)` is stored.

`create(ctx, CreateTokenRequest, assertion)`:
1. Cookie session only (a token cannot mint tokens), else 403 `E_SCOPE`.
2. Validate: name 1-100 characters; scopes non-empty, within the allow-list (`self`, `dashboard.read`, `audit.read`,
   `assessor.read`, `data.steward`) and within the caller's current scopes -> else 422 at `/scopes`; `expires_at` at
   most `pat.max_ttl_days` ahead (the default when absent) -> else 422 at `/expires_at`; fewer than `pat.max_per_user`
   active tokens -> else 422 (`limit_reached`).
3. Transaction: `consume` (`token.create`); insert the row; audit `token.created {token_id, name, scopes, expires_at,
   stepup_id}`.
4. 201 `{id, token, name, scopes, expires_at}` with `Cache-Control: no-store`. The route is outside the idempotency
   ledger, so the secret is stored nowhere; a lost response means revoking that token and creating another.

`authenticate(bearer)`:
1. Parse the format (malformed -> 401); `find_for_auth(id)`.
2. Compare the keyed hash in constant time; revoked, expired, owner disabled or owner expired -> 401, one body.
3. Scopes = the token's scopes intersected with `self` plus the owner's current role scopes: a token loses a scope the
   day its owner does.
4. `last_used_at` at most once a minute (`SET tokused:{id} NX EX 60`, then the update off the request path).
5. `AuthContext { via: Pat, aal: aal1, token_id, roles: the owner's effective roles, scopes }`: never SU, never a
   session-only route.

`list(ctx)` never returns the hash. `revoke(ctx, id)`: own token only, else 404; `revoked_at`, `revoked_by`,
`revoke_reason = owner`; audit `token.revoked {reason}`; 204.

**`ClientCredentials`** (service principals, SP-16, DR-33): Hydra issues JWT access tokens (`strategies.access_token:
jwt`, 15 minutes) by the client-credentials grant only, to clients registered with `akr-ctl identity client create`.
`authenticate(jwt)`: verify against Hydra's JWKS (cached 10 minutes; an unknown `kid` refetches once), `iss` =
`HYDRA_ISSUER`, `aud` contains `akr-api`, `exp` and `nbf` with 30 s leeway; `sub` (the client id) -> the users row of
kind service, active and unexpired; scopes = the token's `scp` within the token allow-list; `AuthContext { via:
ClientCredentials, aal: aal1 }`; `last_authenticated_at` throttled as for tokens. In this phase a service principal can
call only `GET /me`; the first real consumer is the SIEM pull API (phase 25).

**`akr-ctl identity`** (zone Z5; every command logs the OS user and PAM session into the audit sink, as in phase 00):
- `identity bootstrap --role ADM|SEC --email --name-ar --name-en`: allowed only while no active grant of that role
  exists; refuses SEC for a person who holds ADM and the reverse, so the first administrator and the first security
  officer are two people (maker and checker of every later grant, DR-33). Creates the Kratos identity with a
  generated temporary password that meets the policy, then in one transaction the mirror row (`forced_change =
  first_login`, `temp_password_expires_at = now() + auth.temp_password_ttl_h`) and the grant (`source = bootstrap`,
  `{"all_branches": true}`, `granted_by` = the system principal); a failed transaction deletes the Kratos identity.
  Prints the temporary password once, for hand-over in person. Audit `user.bootstrapped` and `role_grant.created
  {source: bootstrap}`. Once both roles exist the command refuses: every later grant is a change request (DR-15,
  phase 04).
- `identity unlock --email`: clears the lock; audit `user.unlocked`. The administrator route arrives in phase 03; the
  command stays for the day every administrator is locked out.
- `identity client create --name --owner <email> --scopes <list> --expires <date>` (scopes within the token
  allow-list; expiry required): the Hydra client and the service principal row; prints the secret once. `identity
  client rotate` and `identity client disable`. Audit `service_client.created`, `service_client.rotated`,
  `service_client.disabled`.
- `identity seed-dev` (refused when `AKR_ENV = production`): synthetic users from `fixtures/dev-users.yaml`, one per
  role, with fixed development passwords and TOTP seeds so the curl suite computes codes with `oathtool`, plus one
  development client.

**`identity_sweep`** (job; section 4):
1. Dormancy: active humans whose `coalesce(last_authenticated_at, created_at)` is older than `auth.dormancy_days` ->
   disable with `dormant` (SP-17).
2. Expiry: active principals with `expires_at <= now()` -> disable with `expired`.
3. Disabling: mirror status and reason; revoke the principal's tokens (`dormant` or `expired_account`);
   `invalidate_user`; audit `user.disabled {reason}` with the system principal as actor. The worker never calls
   Kratos: a disabled mirror row already refuses every session at its next request, and `resolve` step 4 then revokes
   the Kratos session and sets the identity inactive. Re-enabling is an administrator action (phase 03).
4. Batches of 200 with a pause; counts to `akr_identity_disabled_total{reason}`.

**Settings registry additions** (DR-34): the 26 keys of migration 0007 with typed getters, plus hard bounds refused
with 422 `E_VALIDATION`: `stepup.ttl_s` 60-300; `stepup.fresh_window_s` 60-600; `pat.max_ttl_days` 1-90;
`pat.max_per_user` 1-20; `session.idle_timeout_s` 300-3600; `session.absolute_lifetime_s` 3600-43200 (the Kratos
lifespan ceiling); `session.cache_ttl_s` 5-60; `auth.lockout_duration_s` 60-86400; `auth.temp_password_ttl_h` 1-168;
`auth.dormancy_days` 30-365; `auth.lockout_mode` in `timed` or `admin`. `floor` keeps the CBI minima with their basis.

**Error catalogue additions** (API.md section 6 gains the three new codes):

| Variant | HTTP | `code` |
|-|-|-|
| `CsrfInvalid` | 403 | `E_CSRF_INVALID` (missing or wrong CSRF token, or a foreign `Origin`) |
| `CredentialChangeRequired` | 403 | `E_CREDENTIAL_CHANGE_REQUIRED` (a confined session outside the setup routes) |
| `MinFactors` | 409 | `E_MIN_FACTORS` (removing the last second factor) |

Codes already in the catalogue used here: `E_UNAUTHENTICATED` (one body for every failed sign-in stage),
`E_STEPUP_REQUIRED`, `E_SCOPE`, `E_RATE_LIMITED` (lockout and buckets), `E_VALIDATION`, `E_NOT_FOUND` (a session,
factor or token that is not the caller's), `E_VERSION_CONFLICT`, `E_PRECONDITION_REQUIRED`, `E_BAD_REQUEST`.

**Audit events emitted** (through `AuditSink`; persisted once phase 02 lands):

| Event kind | On | Resource |
|-|-|-|
| `auth.login_succeeded` | second factor accepted (method, AAL) | user |
| `auth.login_failed` | any failed stage: password, mfa, locked, disabled, expired_account, temp_expired, on_leave (keyed hash of the identifier when no user matched) | user, or none |
| `auth.account_locked` | the failure that reaches the limit (failures, mode, until) | user |
| `auth.session_expired` | idle or absolute limit detected | user (session id) |
| `auth.logout` | logout | user (session id) |
| `auth.password_changed` | password change (forced or not) | user |
| `auth.mfa_enrolled` | TOTP confirmed or security key registered (method, credential id) | user |
| `auth.mfa_removed` | factor removed (method, credential id, step-up id) | user |
| `auth.policy_acknowledged` | undertaking acknowledged (version) | user |
| `user.preferences_updated` | preferences changed (before, after) | user |
| `session.revoked` | own revoke, or the step-up failure limit (by, reason) | user (session id) |
| `stepup.issued` | step-up verified (assertion id, action class, method) | user |
| `stepup.denied` | step-up proof refused (action class, method) | user |
| `token.created` | token created (name, scopes, expiry, step-up id) | api_token |
| `token.revoked` | token revoked (reason) | api_token |
| `user.bootstrapped` | `akr-ctl identity bootstrap` (role, OS user) | user |
| `role_grant.created` | bootstrap grant (source); phase 04 reuses it for change requests | role_grant |
| `user.unlocked` | `akr-ctl identity unlock` (OS user) | user |
| `user.disabled` | `identity_sweep` (reason: dormant or expired) | user |
| `service_client.created`, `service_client.rotated`, `service_client.disabled` | `akr-ctl identity client` | user (service principal) |

The sign-in and step-up events carry the client IP, user agent and session id in the request block, which is what the
anomaly rules of SP-18 need (unusual hour or workstation, repeated step-up failure, access from outside the analyst
network).

**Cedar actions:** none enforced in this phase; every route here has scope `self`, and phase 03 applies the scope gate
and Cedar to all of them. Names pinned now so that retrofit is mechanical: `session.read`, `session.revoke`,
`profile.read`, `preferences.update`, `policy.acknowledge`, `credential.list`, `credential.enroll`,
`credential.change_password`, `credential.remove`, `token.list`, `token.create`, `token.revoke`, `stepup.start`,
`stepup.verify`. The two public routes have no action.

**Tipping-off review.** This phase adds no case data. What a person could learn from what it adds:
- Sign-in answers: one 401 body for a wrong password, an unknown identifier, a disabled, expired or on-leave account;
  an unknown identifier locks exactly like a real one. Nothing reveals whether an account exists or why it was
  refused.
- `GET /me`, sessions, factors and tokens show the caller's own data only; another person's ids answer 404.
- The step-up ledger holds `resource_ref` (`case:<uuid>` style ids, never names) and is granted to `akr_app` only.
  The step-up dialog names the intent ("approve a change request"), never a customer or a reason.
- Delegations carry a reason code, never free text, so nothing about a case can be written into a record that
  administrators read.
- Logs and audit details never hold a raw identifier, password, TOTP code or token; the redaction layer gains those
  field names and the `akrpat_` pattern.
The answer for a person outside a case's access list is: nothing.

**Settings and configuration used:** the 26 keys and one flag of migration 0007 and the phase 00 keys. Environment
additions (validated by `Config::validate`, refused in production when missing or not `https`): `KRATOS_PUBLIC_URL`,
`KRATOS_ADMIN_URL`, `HYDRA_PUBLIC_URL`, `HYDRA_ISSUER`, `AKR_PUBLIC_ORIGIN`, `AKR_TRUSTED_PROXIES`; `akr-ctl` also
reads `HYDRA_ADMIN_URL`. The CSRF, token and identifier subkeys come from OpenBao KV `keys/auth-hmac`, never from the
environment.

## 4. Infrastructure Updates

- **Job kinds registered:** `identity_sweep` (daily, `schedule.identity_sweep`, 03:00 bank time; concurrency 1;
  section 3). `maintenance_sweep` (phase 00) also deletes `admin.stepup_assertions` rows older than
  `stepup.retention_days`. **Outbox topics:** none. **Timers:** none.
- **Kratos** (`deploy/compose/kratos/kratos.yml` and `identity.schema.json`; the contract the DBS measured on v1.2.0,
  re-measured on the pinned release in section 6):
  - Methods: `password` (`identifier_similarity_check_enabled: true`, `haveibeenpwned_enabled: false` because it is an
    outbound call), `totp` (issuer `AKR-TMS`), `webauthn` (`passwordless: false`; `rp.id` the public host, `rp.origins`
    the public origin). `lookup_secret`, `code`, `link` and `profile` are disabled.
  - Flows: registration, verification and recovery disabled (DR-30); settings `privileged_session_max_age: 15m`,
    `required_aal: highest_available`, after password `hooks: [{hook: revoke_active_sessions}]` (the plural `hooks`
    key; a bare `hook` fails schema validation, DBS s.8.8).
  - Sessions: `session.whoami.required_aal: highest_available`; `session.lifespan` 12 h as the ceiling; cookie
    `akr_session`, `SameSite=Strict`, `Secure`, not persistent, domain the public host.
  - `hashers.algorithm: argon2` (it runs argon2id: 128 MB, 3 iterations, 4 lanes); `secrets.cipher` exactly 32
    characters, `secrets.cookie` and `secrets.default` from OpenBao; the courier points at an unroutable SMTP URI and
    no enabled flow sends mail.
  - Identity schema: traits `email` (the identifier for password, and the account name for TOTP and WebAuthn),
    `display_name {ar, en}`, `locale`; no metadata (account state is in the mirror).
  - Kratos serves no browser traffic: the edge does not route to it, and `serve.public.base_url` is the public origin
    so cookie and WebAuthn origin checks match the BFF's host.
- **Hydra** (`deploy/compose/hydra/hydra.yml`): issuer `https://<public host>/`; the client-credentials grant only (the
  authorisation-code flow and a login and consent provider come with the first OIDC client, not in P0);
  `strategies.access_token: jwt`; `ttl.access_token: 15m`; `secrets.system` from OpenBao.
- **Compose** (both bundles):

| Service | Change |
|-|-|
| `kratos`, `kratos-migrate` | Config above; networks `z3_ory` only (phase 00's table listed `z3_edge` as well; corrected, because the browser never addresses Kratos); public 4433 reached by `akr-api`; admin 4434 by `akr-api` and `akr-ctl` |
| `hydra`, `hydra-migrate` | Config above; `z3_ory` and `z3_edge`; admin 4445 reached by `akr-ctl` only |
| `ory-secrets` (new, one-shot) | OpenBao Agent with AppRole `akr-ory` (KV read on `ory/*` only) renders the Ory environment files into a tmpfs volume mounted read-only by the four Ory containers; no secret in compose files or images (SP-12) |
| `caddy` | Routes `POST /oauth2/token`, `GET /.well-known/jwks.json` and `GET /.well-known/openid-configuration` to `hydra:4444`; nothing routes to Kratos or to either admin port |
| `akr-ctl` (one-shot service) | Networks `z2_app`, `z3_ory`, `z5_mgmt` |

- **Provisioning:** `akr-ctl db provision` also creates the databases `kratos` and `hydra`, each owned by a LOGIN role
  of the same name whose password lives in OpenBao (`ory/kratos`, `ory/hydra`); neither role has any privilege in the
  AKR database.
- **OpenBao:** KV `keys/auth-hmac` (HKDF subkeys `csrf`, `token`, `login_identifier`), `ory/kratos`, `ory/hydra`;
  AppRole `akr-ory`. The `akr-api` policy is unchanged (it already reads `keys/*`). Rotating `keys/auth-hmac` re-mints
  CSRF cookies at the next request and invalidates every personal token (they expire within 90 days in any case);
  the rotation runbook is part of phase 25.
- **Zone rules** (`deploy/zones.yaml`, checked by `make zone-check`): `akr-api` -> `kratos:4433`, `kratos:4434`,
  `hydra:4444`; the edge -> `hydra:4444` only; `akr-ctl` -> `kratos:4434`, `hydra:4445`, `postgres`; `kratos` and
  `hydra` -> `postgres` only. Nothing else reaches an Ory admin port; the edge cannot reach Kratos; `akr-worker` gains
  no edge (the sweep writes only the mirror).
- **AppState additions** (`api`): `identity: Arc<IdentityService>`, `sessions: Arc<SessionService>`, `stepup:
  Arc<StepUpService>`, `tokens: Arc<TokenService>`, `client_credentials: Arc<ClientCredentials>`, `users: Arc<dyn
  UserRepository>`, `kratos: Arc<dyn IdentityProvider>`, `limiter: RateLimiter`. `worker` registers `identity_sweep`.
- **OpenAPI registration:** tags `auth` and `me`; 22 operations; the cookie and bearer security schemes declared in
  phase 00 are referenced; the two SU operations reference the step-up scheme and also declare `X-StepUp-Assertion`
  as a header parameter, because a security scheme alone gives the generated client no way to send it (DBS s.8.10
  item 2; Orval `output.headers: true`); the exempt operations document that they take no `Idempotency-Key`.
- **Existing entity, schema or route updates:**
  - The phase 00 `X-Dev-Principal` stub is deleted; the dev-only `/api/v1/_echo` now needs a real principal.
  - The idempotency exemption list gains the eight routes of section 2.
  - `akr-api` readiness adds `kratos` (`/health/ready`) and `hydra_jwks`.
  - `maintenance_sweep` prunes the step-up ledger.
  - The settings registry gains hard bounds (DR-34).
  - The redaction layer gains `password`, `new_password`, `totp_code`, `token`, `secret` and the `akrpat_` pattern.
  - phase-00.md's compose table: Kratos is on `z3_ory` only.
  - The deferred FKs from phase 00 are added (migration 0005).

## 5. Frontend Impact

Backend phase for screens. `make generate-api` regenerates the client with the `auth` and `me` hooks
(`usePostAuthLogin`, `usePostAuthMfa`, `useGetAuthSession`, `usePostAuthSessionHeartbeat`, `usePostAuthLogout`,
`usePostAuthStepUp`, `usePostAuthStepUpVerify`, `useGetMe`, `usePatchMePreferences`, `usePostMePolicyAcknowledgement`,
`useGetMeSessions`, `useDeleteMeSessionsId`, `useGetMeCredentials`, `usePostMeCredentialsPassword`, the four
enrolment hooks, `useDeleteMeCredentialsId`, `useGetMeTokens`, `usePostMeTokens`, `useDeleteMeTokensId`). The Orval
mutator rules, from the DBS's defects: requests carry credentials; the base URL is not doubled with the spec's
`/api/v1` prefix (DBS s.8.14; a unit test pins it); every mutation sends `X-CSRF-Token` read from the `akr_csrf`
cookie and, outside the exemption list, an `Idempotency-Key`; the config leaves Orval's query and mutation defaults
alone (setting both inverts them, DBS s.8.10 item 1).

The screens are built in phase 08 on the shell: X1 sign-in (password; the second factor with WebAuthn first and
paste and autofill on code fields, UX-36; the first-login stepper: change password, enrol a factor, acknowledge the
undertaking), the shared `StepUpDialog`, `SessionWarning` (counts down from `idle_expires_at` minus `warning_s`;
"Extend session" sends the heartbeat; the client sends it only after real input, at most once per
`session.heartbeat_min_interval_s`, so an unattended screen times out, UX-35), and the X4 security section (sessions,
second factors, tokens). The one page this phase ships is the development harness `GET /_dev/webauthn` (mounted only
when `AKR_ENV != production`): a test fixture for section 6, not a screen.

## 6. Verification

1. `make migrate`: 0005-0007 apply; a second run is a no-op; the phase 00 schema checks pass for the seven new tables
   (classification and retention coverage, no `timestamp without time zone`, no `id` default, no reserved word);
   `\dp admin.role_grants` shows `akr_app` with UPDATE on (`valid_to`, `revoked_at`, `revoked_by`) only; `akr_readonly`
   has no privilege on `admin.stepup_assertions`, `admin.password_history` or `admin.api_tokens.token_hash`.
2. `make up`: `kratos` and `hydra` report ready; `akr-ctl db provision` created the `kratos` and `hydra` databases;
   `make zone-check` matches the updated matrix (from the edge's position `kratos:4433` is closed and `hydra:4444`
   open; from `akr-worker`'s position both Ory services are closed).
3. `cargo check` with `cargo sqlx prepare`; `cargo clippy --all-targets -- -D warnings`; `cargo deny check`.
4. `cargo test`, including:
   - step-up: two transactions consume one assertion -> exactly one succeeds; another class, session, user or
     resource, an expired row and a spent row -> `E_STEPUP_REQUIRED`; a failing action leaves the assertion unspent;
     five failed verifications -> the session is revoked;
   - tokens: `case.decide` and a 91-day expiry are refused by the service and by the table CHECKs; an owner losing
     `data.steward` -> the token loses it; revoked, expired and owner-disabled tokens -> 401 with one body; a token on
     a session-only route -> 403 `E_SCOPE`; after creation `admin.idempotency_keys` holds no row for the request;
   - sessions (test settings: idle 5 s): no activity -> 401 and the Kratos session revoked; a GET every second does
     not extend; the heartbeat does; the absolute limit holds; deleting `idle:{sid}` -> 401 (fail closed);
   - confinement: a setup session reaches exactly the setup routes (table-driven over the OpenAPI document);
   - lockout: three failures lock (timed: `Retry-After` equals the remainder; admin: no header until `akr-ctl
     identity unlock`); an unknown identifier gives the same status, headers and body after three tries; second-factor
     failures count; a successful second factor resets the counter;
   - password policy: an Arabic-letter password with digits and a symbol passes three classes; Arabic-Indic digits
     count; each of the last three passwords is refused and the fourth-oldest accepted; past
     `auth.password_max_age_days` -> `change_password` pending after the second factor;
   - TOTP: the same code twice -> the second refused, at sign-in and at step-up;
   - delegations: a delegated role appears with `on_behalf_of` and disappears when the delegator's grant is revoked,
     the delegation ends or the delegator is disabled; `ADM` in a delegation scope is refused by the CHECK;
   - `identity_sweep`: a user idle 61 days is disabled with tokens revoked; a passed `expires_at` disables; the system
     principal is untouched; a disabled user's live session is refused at its next request and the Kratos identity
     set inactive;
   - coverage: every operation without credentials -> 401 except the two public ones; every cookie mutation without
     `X-CSRF-Token` -> 403 `E_CSRF_INVALID`, and with a foreign `Origin` -> 403;
   - redaction: a sign-in with a password typed into the identifier field leaves that string in no log, trace,
     metric or audit detail; an `akrpat_` value is redacted.
5. The curl suite `tests/curl/phase01_identity.sh` against live Kratos and Hydra (the DBS lesson: a mock proves
   self-consistency; only the real service proves the protocol). TOTP codes from `oathtool`; distinct valid codes
   come from adjacent windows (one step of skew):
   - `akr-ctl identity bootstrap --role ADM --email adm@bank.test --name-ar ... --name-en ...` prints a temporary
     password; `curl -s -c jar -H 'Origin: https://akr.test' -H 'Content-Type: application/json' -X POST
     https://akr.test/api/v1/auth/login -d '{"identifier":"adm@bank.test","password":"<temp>"}'` -> 200
     `{"state":"setup_required","pending":["change_password","enroll_mfa","acknowledge_policy"],"csrf_token":"..."}`;
     the dev `POST /api/v1/_echo` with that session -> 403 `{"code":"E_CREDENTIAL_CHANGE_REQUIRED",...}`.
   - Password change -> TOTP start and confirm on the same flow -> acknowledgement (with `Idempotency-Key`) -> `GET
     /api/v1/auth/session` shows `"aal":"aal2"` and `"pending":[]`.
   - Logout; login again -> `{"state":"mfa_required","flow_id":"...","methods":["totp"],...}`; `POST
     /api/v1/auth/mfa` with a code -> `{"state":"authenticated",...}`; `GET /api/v1/me` lists role `ADM` and scope
     `admin.users`; the same code again on a new flow -> 401.
   - Step-up and token: `POST /api/v1/auth/step-up -d '{"action_class":"token_create"}'` -> challenge; verify ->
     `{"assertion_id":"...","expires_at":"..."}`; `POST /api/v1/me/tokens` without the assertion -> 401
     `E_STEPUP_REQUIRED`; with `X-StepUp-Assertion` -> 201 with `"token":"akrpat_..."`; the same assertion again -> 401;
     `curl -H "Authorization: Bearer akrpat_..." /api/v1/me` -> 200; that token on `POST /api/v1/me/tokens` -> 403
     `E_SCOPE`; `DELETE /api/v1/me/tokens/{id}` -> 204, then the token -> 401.
   - Idempotency replay: `PATCH /api/v1/me/preferences` with `If-Match: "1"` and `Idempotency-Key: pref-0001` twice ->
     byte-identical bodies, the second with `x-idempotent-replay: true`; the same key with another body -> 422
     `E_IDEMPOTENCY_REUSE`; no `If-Match` -> 428; `If-Match: "1"` after the change -> 409 `E_VERSION_CONFLICT`.
   - CSRF: `POST /api/v1/auth/logout` without `X-CSRF-Token` -> 403 `E_CSRF_INVALID`.
   - Lockout: three wrong passwords -> the fourth attempt 429 with `Retry-After`; the same for `nobody@bank.test`.
   - Sessions: two cookie jars -> `GET /api/v1/me/sessions` lists 2; `DELETE` the other -> 204 and that jar -> 401.
   - Client credentials: `akr-ctl identity client create --name recon-script --owner ds@bank.test --scopes
     self,data.steward --expires 2027-01-31`; `curl -u <id>:<secret> -d grant_type=client_credentials
     https://akr.test/oauth2/token` -> a JWT; `GET /api/v1/me` with it -> 200 `"kind":"service"`; after `identity
     client disable` -> 401.
   - No Cedar 403 or existence-masking 404 case exists yet (phase 03).
6. WebAuthn, end to end: a Playwright test in headless Chromium adds a virtual authenticator through the DevTools
   protocol (`WebAuthn.addVirtualAuthenticator`, CTAP2, internal transport, user verification), registers a key for a
   seeded user through `/_dev/webauthn`, signs in with password and key, and runs a step-up with the key; the sink
   shows `auth.mfa_enrolled {method: webauthn}` and `stepup.issued {method: webauthn}`. (The DBS could not verify
   WebAuthn; this closes that gap.)
7. Swagger UI lists 22 operations, with the step-up header on the two SU operations; `make generate-api` twice gives
   zero diff.
8. Audit: the tracing sink shows every event kind exercised in steps 5 and 6 (persisted rows from phase 02).
9. The Kratos contract re-measured on the pinned release and recorded in an implementation-notes section of this
   file, as the DBS did in its phase 01 section 8: the password submit (200, 422, 400), the AAL2 and refresh flows, the
   settings node names, the admin credential shapes with and without `include_credential`, the WebAuthn trigger
   nodes, and the rejected config values.
10. Phase 00 tests still pass.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 1, from tools/agent/gap1-run1.md; lead-verified (tools/agent/gap1-run1-verdict.md).

### 7.1 identity_sweep: resets, branch consistency and dormancy basis
- Step 1 (dormancy) measures from `greatest(last_authenticated_at, enabled_at, created_at)` instead of
  `coalesce(last_authenticated_at, created_at)`, so a re-enabled account gets a full `auth.dormancy_days` to sign in
  (`enabled_at` is added by phase 03 section 1; the expression is phase 03 section 3.7).
- Step 5 (pending credential resets): a `credential_resets` row in state `pending` with `expires_at <= now()` becomes
  `expired`, audited as `credential_reset.expired` with the system principal (phase 03 section 3.8).
- Step 6 (consistency check, no fix): pool branch lists and grant branch scopes that name an archived or unknown branch
  produce an error log and a cockpit count. Nothing is changed silently (phase 03 section 4).

### 7.2 akr-ctl db provision: additional logins, database and grants
`akr-ctl db provision` also does the following, in addition to the Ory databases above:
(a) creates the LOGIN role `akr_monitor` with the password in OpenBao `db/akr_monitor`; it is a member of
    `pg_monitor` only and has no privilege on any AKR schema (phase 03 section 4; phase 00 7.4);
(b) creates the LOGIN role `akr_retention_login` with the password in OpenBao `db/akr_retention_login`; it is a member
    of `akr_retention` (NOLOGIN, phase 23 migration 0072) and of `akr_audit_writer` (phase 23 sections 1.2 and 4.5);
(c) creates the database `polis`, owned by the LOGIN role `polis` with the password in OpenBao `ory/polis`; it runs only
    when `pack.polis` is switched on, not before (phase 31 section 4.5 item 2);
(d) runs `GRANT akr_audit_writer TO akr_app, akr_ingest, akr_assess` (INHERIT); `db check` asserts it (phase 02 section 4);
(e) the read-only pool needs no new role: `akr_readonly` already receives LOGIN in this provisioning step (phase 00 section 1).

### 7.3 admin.role_grants: support session source (documentation of migration 0081)
Phase 25 migration 0081 changes this table; its ALTER statements are in phase 25 section 1.2 and stay there (the
support session table `admin.support_sessions` is created by phase 25, so the foreign key cannot appear in this file).
After 0081 the `source` CHECK is `IN ('bootstrap', 'change_request', 'support_session')`, the column
`support_session_id` exists, and `role_grants_support_session_check` ties the two. Add a line under the DDL of this
section that says so, and change the `source` CHECK text above to show the three values with the note "(0081)".

### 7.4 Dev seed: fixed users beyond one per role
`identity seed-dev` creates one user per role (section 3) and these fixed extra users, all from `fixtures/dev-users.yaml`:
- `a2b@bank.test`, a second A2 user for the existence-masking test (phase 07 section 4);
- `sec2@bank.test`, a second SEC holder for four-eyes steps (phase 25 section 6);
- `vs@bank.test`, a vendor user with no role; the support session is its only grant (phase 25 section 6);
- `as@bank.test`, role AS, for the assessor pack and the training register (phase 24 section 6; grant valid one day).
Each has the fixed development password and TOTP seed of the other dev users.

### 7.5 Rate limiter when Valkey is unavailable
If Valkey does not answer, the limiter allows the request (fail open) and increments
`akr_rate_limit_unavailable_total`. The sign-in lockout (`auth.lockout_max_failures`, counted in `admin.users`) and the
step-up checks do not depend on the limiter and keep working. (Phase 30 section 4.5 item 9.)

### 7.6 OpenAPI extensions for the generated client
- Every step-up operation carries the extension `x-akr-stepup` with its action class name from the closed list (section 3).
- The document declares the schemas `ActionClass` (the closed list) and `ProblemCode` (the closed list of problem codes).
- The document carries `x-akr-role-scopes`: the role-to-scope table of `common::scopes::scopes_for_roles` (section 2), so
  the browser reads it instead of parsing descriptions.
- Every operation names its scopes in `security`.
The phase 08 client and the policy-matrix drift check read these (phase 08 section 4). Phase 03 adds `x-akr-action` to
the same operations.

### 7.7 default_queue_id rule
Replace the sentence "`default_queue_id` must be null until queues exist in phase 17" with: "`default_queue_id` is null or the
id of an active queue visible to the caller. Until phase 17 creates queues, only null is accepted; from phase 17 the id is
checked, and a refusal is 422 `E_VALIDATION` at `/default_queue_id`. The field stays inside the preferences object (no
column, no foreign key)." (Phase 17 section 4.5 item 4.)

Adversarial pass (2026-10-10), run 1 (four-eyes and self-approval), from tools/agent/adv-run1.md; lead-verified (tools/agent/adv-run1-verdict.md).

### 7.8 Decision step-ups name their resource (ADV-1-04, LOW; DR-407)
For the action classes `decision`, `approval`, `signing` and `filing`, `consume` step 2 matches the resource exactly:
`resource_ref IS NOT DISTINCT FROM $6` in place of `(resource_ref IS NULL OR resource_ref = $6)`. An assertion started
without a resource is then good only for an action that passes none. The other classes keep the current predicate.
Test: an assertion of class `decision` started without `resource_ref` is 401 `E_STEPUP_REQUIRED` on a task decision.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.9 Service client commands, step by step (V-01-02-a, MEDIUM; DR-413)
The three `akr-ctl identity client` commands of section 3 run as follows (the Hydra admin API through `common::ory`,
`HYDRA_ADMIN_URL`).
1. `client create`: validate (the owner is an active human; scopes within the token allow-list; `--expires` in the
   future, else exit 2 with the field named). Create the Hydra client (grant `client_credentials`, the scopes,
   audience `akr-api`, a generated secret). Then in one transaction insert the `admin.users` row (`kind = 'service'`,
   `display_name` with both keys set to `--name`, `hydra_client_id`, `owner_user_id`, `expires_at`) and audit `service_client.created {client_id, owner_user_id,
   scopes, expires_at}`. A failed transaction deletes the Hydra client. The secret is printed once; AKR-TMS stores no
   copy. `--name` is also the Hydra client name. A name held by a service principal that is not disabled is refused
   (exit 2), and `rotate` and `disable` find the principal by it (`kind = 'service'`, `status = 'active'` and
   `display_name ->> 'en'`), so a disabled principal that still carries the name is never the one rotated or disabled
   (the rerun of `disable` in step 3 is the one read of disabled principals by name: it only deletes their Hydra
   clients).
2. `client rotate --name`: Hydra replaces the client's secret with a new generated one. The old secret stops working at
   once; access tokens already issued run to their expiry (15 minutes). Audit `service_client.rotated {client_id}`;
   the new secret is printed once. No `admin.users` row changes.
3. `client disable --name`: in one transaction set the principal's `status = 'disabled'` with `disabled_reason =
   'admin'` and audit `service_client.disabled {client_id}`; after commit delete the Hydra client (the order phase 03
   uses when offboarding disables a user's service principals). `authenticate` already refuses a principal that is not
   active, so a failed Hydra call leaves no access and the command can be run again: when no active principal carries
   the name, `disable` reads the disabled service principals that carry it and repeats only the Hydra delete, for the
   `hydra_client_id` of each (a client that Hydra no longer has counts as deleted). The rerun changes no row and writes
   no second audit event.

### 7.10 Actions for heartbeat and logout (V-01-04-a, MEDIUM; DR-413)
The pinned list of section 3 left two routes that need an action without one (the two public routes need none).
`POST /auth/session/heartbeat` uses
`session.read` (it returns the `SessionView` of the caller's own session) and `POST /auth/logout` uses `session.revoke`
on the current session (the same act as `DELETE /me/sessions/{id}` for the current one). No action is added; the
phase 03 retrofit registers both `RouteAction` rows with resource `User(self)`.

### 7.11 The token limit is counted under a lock (V-01-05-a, MEDIUM; DR-413)
`TokenService::create`: the `pat.max_per_user` test of step 2 moves into the transaction of step 3. After `consume`
the transaction takes `pg_advisory_xact_lock(hashtext('pat:' || user_id))`, counts the user's active tokens and
answers the same 422 (`limit_reached`) before the insert. Two concurrent creates with two step-up assertions can
then not both pass the count. The other tests of step 2 stay where they are.

### 7.12 The Kratos admin port gains a traits update (V-03-05-a, MEDIUM; DR-413)
From phase 03 on the admin API of `IdentityProvider` (section 2) has `update_traits(id, traits)`, with the same
`traits` type as `create_identity`: Kratos `PUT /admin/identities/{id}` with the identity's current state and schema
and the new traits. `UserAdminService::update` (phase 03 section 3.7) calls it for an e-mail change before its commit;
a Kratos failure aborts the request. When a later step of the same request fails, the handler calls `update_traits`
again with the old traits before it returns the error (the compensation pattern `create` uses with
`delete_identity`).

### 7.13 SessionView carries the heartbeat interval (V-08-06-a, MEDIUM; DR-413)
`SessionView` (section 2) gains `heartbeat_min_interval_s` (seconds), the current value of the setting `session.heartbeat_min_interval_s`, as `warning_s` carries `session.warning_s`. `GET /auth/session` and `POST /auth/session/heartbeat` return it, so the client applies the heartbeat rule of phase 08 section 3.6 from the session view alone. The idle timeout reaches the client as `idle_expires_at`. No route is added and no setting changes.

## What This Phase Does NOT Touch

- No scope gate, Cedar policies or existence masking (phase 03); every route here is `self`.
- No administrator routes: user creation, profile edits, grants, unlock, credential reset and offboarding are phase
  03 (API.md 3.2); until then the bootstrap and unlock commands of `akr-ctl` are the only administration.
- No change requests (phase 04): `role_grants.change_request_id` has no FK yet and governed settings change only by
  migration.
- No branch table (phase 03, DR-27): `users.branch_id` and pool branch lists are unconstrained until then.
- No audit persistence (phase 02); `TracingSink` only.
- No notifications (phase 22); the two `/me/notifications` routes move there.
- No screens (phase 08) beyond the development harness.
- No OIDC upstream (phase 25), no Hydra authorisation-code flow or login and consent provider, no SAML or SCIM (phase
  31), no gateway certificates (phase 29), no leave ledger (the SP-17 leave block), no SIEM forwarding (phase 25).
- No CBS connection; no change to phase 00 tables beyond the deferred FKs and the `maintenance_sweep` addition.
