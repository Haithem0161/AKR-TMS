# Phase 03: Authorisation, Users & Roles

**Goal:** Cedar becomes the authority on every `/api/v1` request: a bundle stored in the database, validated strictly,
activated once by the bootstrap command and afterwards only by change request, hot-reloaded in every `akr-api`
instance, evaluated after the scope gate with attributes resolved through a Valkey cache, every decision written to
the audit trail, and every refusal on a masked resource kind answered with the same 404 as a missing record; the
segregation-of-duties rules (exclusive roles, action pairs, no self-administration, four-eyes on credential resets)
as `forbid` rules plus service and database checks; the audited simulator and shadow evaluation; the administration
of users, branches, pools and delegations (A1), with unlock, suspension, re-enabling, one-action offboarding and the
four-eyes credential reset; the generated access matrix as the stub of the quarterly review; the purpose-code list
the access log needs; and the settings, flags, jobs and ops cockpit reads behind A2 and A3.

**Dependencies:** Phase 01, Phase 02
**Macro-phase:** P0
**Complexity:** XL (32 routes; roadmap row 03 said L, corrected here: the route count is over the 25-route line)
**Binaries touched:** akr-api (the `authz` crate replaces `AllowAllAuthorizer`; routers `org`, `admin_users`, `authz`,
`admin_system`; the phase 01 and 02 routes gain their actions) | akr-worker (`ops_probe`, `offboard_finalize`;
`identity_sweep` extended) | `akr-ctl` (`policy validate`, `policy install`, `policy activate --bootstrap`, `policy
shadow`, `org import-branches`). akr-ingestd and akr-assess unchanged
**Requirements carried:** SP-19 (deny by default, two gates, attributes, every decision audited), SP-20 (role
catalogue as policy and the generated access matrix), SP-21 (exclusive roles, action pairs and four-eyes as `forbid`
rules plus database checks, with a build-failing test), SP-25 (bundle governance: staged bundles, audited simulator,
shadow evaluation, the route coverage test in both directions; activation by change request from phase 04; the
quarterly review signature in phase 25), SP-27 (existence masking with byte-identical 404 bodies and a latency
floor; the tiered resources arrive from phase 15), SP-33 (the inference test harness and its registry; probes are
added by every phase that adds a masked kind), SP-35 (the purpose-code list, `deny` and `conflict` access events from
the policy layer), SP-14 (administrator provisioning and administrator recovery), SP-17 (re-enabling after dormancy),
SP-37 (no standing administrator access to business data: role exclusivity), SP-38 (vetting status recorded on the
user and exposed to policy; the suspicion-tier grant rule is enforced from phase 18), TS-26, TS-10 and TS-28 (the
cockpit tiles for jobs, backups, anchors, seal state and certificates), UX-53 (the `x-akr-action` extension that the
phase 08 route-to-policy test reads), M-43 (liaison officer and alternate per branch)
- Gap pass 1 (coverage), partly implemented: SP-47 (purpose codes checked against roles), SP-49 (masking by surface; unmask as an access event), SP-79 (the inference test harness and its registry).
- Gap pass 1 (coverage), implemented here: B-04 (roles for the three lines of defence; internal audit read-only; the MLRO always has access).
- Gap pass 1 (coverage), not carried: F-18 (group view across subsidiaries; further banks are separate installs, DR-401).

## 1. SQLx Migration Changes

Three migrations, `backend/migrations/0011` to `0013` (canonical chain in roadmap.md, DR-23). Conventions as in phases
00 to 02: schema-qualified names, no database default on `id`, no reserved words, per-table grants with column lists,
and each migration inserts the classification and retention rows for its own tables. Nothing here touches the
`audit` schema, so the DR-45 event triggers do not apply.

```sql
-- migration 0011_authz_bundles_purposes.sql
-- Phase 03. The Cedar bundle register (DR-47) and the purpose codes the access log checks (DR-52).

DO $$ BEGIN
    CREATE TYPE admin.bundle_status AS ENUM ('staged', 'active', 'superseded', 'refused');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS admin.policy_bundles (
    id                 UUID PRIMARY KEY,
    version            INT NOT NULL CHECK (version > 0),
    status             admin.bundle_status NOT NULL DEFAULT 'staged',
    source             TEXT NOT NULL CHECK (source IN ('release', 'authored')),   -- authored: PUT /admin/policies (phase 04)
    release_id         TEXT CHECK (release_id ~ '^[0-9A-Za-z._-]{1,64}$'),       -- the signed release it came in
    cedar_source       TEXT NOT NULL,       -- policy files in manifest order, each preceded by "// file: <name>"
    schema_text        TEXT NOT NULL,       -- the .cedarschema file as shipped
    schema_json        JSONB NOT NULL,      -- the same schema in Cedar's JSON form (what the simulator shows)
    sod                JSONB NOT NULL CHECK (sod ? 'exclusive_roles' AND sod ? 'action_pairs'),
    manifest           JSONB NOT NULL CHECK (manifest ? 'files'),   -- [{name, sha256}] in evaluation order
    sha256             BYTEA NOT NULL CHECK (octet_length(sha256) = 32),   -- over the canonical bundle (section 3)
    validation         JSONB NOT NULL,      -- {policies, actions, warnings[]} from strict validation at install
    activation         TEXT CHECK (activation IN ('bootstrap', 'change_request')),
    change_request_id  UUID,                -- FK to admin.change_requests added by phase 04 (deferred FK)
    shadow             BOOLEAN NOT NULL DEFAULT FALSE,   -- evaluated beside the active bundle (dry run, SP-25)
    installed_by       UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    installed_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    activated_by       UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    activated_at       TIMESTAMPTZ,
    superseded_at      TIMESTAMPTZ,
    refused_reason     TEXT CHECK (refused_reason ~ '^[a-z][a-z_]*$'),
    created_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK ((source = 'release') = (release_id IS NOT NULL)),
    CHECK ((status IN ('active', 'superseded')) = (activated_at IS NOT NULL)),
    CHECK ((activated_at IS NULL) = (activated_by IS NULL)),
    CHECK ((activated_at IS NULL) = (activation IS NULL)),
    CHECK ((activation = 'change_request') = (change_request_id IS NOT NULL)),
    CHECK ((status = 'superseded') = (superseded_at IS NOT NULL)),
    CHECK ((status = 'refused') = (refused_reason IS NOT NULL)),
    CHECK (NOT shadow OR status = 'staged')
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_policy_bundles_version ON admin.policy_bundles (version);
CREATE UNIQUE INDEX IF NOT EXISTS uq_policy_bundles_sha256  ON admin.policy_bundles (sha256);
-- At most one active and one shadow bundle.
CREATE UNIQUE INDEX IF NOT EXISTS uq_policy_bundles_active  ON admin.policy_bundles ((TRUE)) WHERE status = 'active';
CREATE UNIQUE INDEX IF NOT EXISTS uq_policy_bundles_shadow  ON admin.policy_bundles ((TRUE)) WHERE shadow;
CREATE INDEX IF NOT EXISTS idx_policy_bundles_installed_by ON admin.policy_bundles (installed_by);
CREATE INDEX IF NOT EXISTS idx_policy_bundles_activated_by ON admin.policy_bundles (activated_by);
CREATE INDEX IF NOT EXISTS idx_policy_bundles_cr           ON admin.policy_bundles (change_request_id);

-- Status moves only forward: staged -> active | refused, active -> superseded. Content columns cannot change (the
-- grants below hold no UPDATE on them; this trigger is the second layer, as for evidence in phase 02).
CREATE OR REPLACE FUNCTION admin.policy_bundles_guard() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog AS $$
BEGIN
    IF NEW.version <> OLD.version OR NEW.cedar_source <> OLD.cedar_source OR NEW.schema_text <> OLD.schema_text
       OR NEW.schema_json <> OLD.schema_json OR NEW.sod <> OLD.sod OR NEW.manifest <> OLD.manifest
       OR NEW.sha256 <> OLD.sha256 OR NEW.source <> OLD.source
       OR NEW.release_id IS DISTINCT FROM OLD.release_id THEN
        RAISE EXCEPTION 'policy bundle content is immutable' USING ERRCODE = 'check_violation';
    END IF;
    IF NEW.status <> OLD.status AND NOT (
           (OLD.status = 'staged' AND NEW.status IN ('active', 'refused'))
        OR (OLD.status = 'active' AND NEW.status = 'superseded')) THEN
        RAISE EXCEPTION 'policy bundle status % -> % not allowed', OLD.status, NEW.status
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;

DROP TRIGGER IF EXISTS trg_policy_bundles_guard ON admin.policy_bundles;
CREATE TRIGGER trg_policy_bundles_guard BEFORE UPDATE ON admin.policy_bundles
    FOR EACH ROW EXECUTE FUNCTION admin.policy_bundles_guard();

CREATE TABLE IF NOT EXISTS admin.purpose_codes (
    code             TEXT PRIMARY KEY CHECK (code ~ '^[a-z][a-z0-9_]{1,63}$'),   -- the format phase 02 checks
    label            JSONB NOT NULL CHECK (label ? 'ar' AND label ? 'en'),
    tiers            TEXT[] NOT NULL CHECK (cardinality(tiers) > 0 AND tiers <@ '{restricted,suspicion}'::text[]),
    reason_required  BOOLEAN NOT NULL DEFAULT FALSE,   -- X-Access-Reason must be present
    sort_order       INT NOT NULL DEFAULT 100,
    active           BOOLEAN NOT NULL DEFAULT TRUE,    -- codes are never deleted: access events cite them
    updated_by       UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- NULL = seeded
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    version          INT NOT NULL DEFAULT 1
);
CREATE INDEX IF NOT EXISTS idx_purpose_codes_updated_by ON admin.purpose_codes (updated_by);

-- Seed list (DR-52). Editing by change request arrives with the reason-code administration in phase 09.
INSERT INTO admin.purpose_codes (code, label, tiers, reason_required, sort_order) VALUES
 ('assigned_work',          '{"ar": "عمل مُسند (تنبيه أو قضية أو مهمة)", "en": "Assigned work (alert, case or task)"}',
  '{restricted,suspicion}', FALSE, 10),
 ('investigation',          '{"ar": "تحقيق في قضية", "en": "Case investigation"}',
  '{restricted,suspicion}', FALSE, 20),
 ('kyc_review',             '{"ar": "مراجعة اعرف عميلك", "en": "KYC review"}',
  '{restricted,suspicion}', FALSE, 30),
 ('report_preparation',     '{"ar": "إعداد بلاغ", "en": "Report preparation"}',
  '{suspicion}', FALSE, 40),
 ('authority_request',      '{"ar": "طلب من جهة مختصة", "en": "Request from a competent authority"}',
  '{restricted,suspicion}', TRUE, 50),
 ('qa_review',              '{"ar": "مراجعة الجودة", "en": "Quality review"}',
  '{restricted,suspicion}', FALSE, 60),
 ('internal_audit',         '{"ar": "تدقيق داخلي", "en": "Internal audit"}',
  '{restricted,suspicion}', FALSE, 70),
 ('supervisory_inspection', '{"ar": "تفتيش رقابي", "en": "Supervisory inspection"}',
  '{restricted,suspicion}', FALSE, 80),
 ('break_glass',            '{"ar": "وصول طارئ", "en": "Emergency access (break-glass)"}',
  '{restricted,suspicion}', TRUE, 90),
 ('other',                  '{"ar": "غرض آخر (مع ذكر السبب)", "en": "Other purpose (reason required)"}',
  '{restricted}', TRUE, 100)
ON CONFLICT (code) DO NOTHING;

-- akr-ctl runs as akr_app (phase 01), so installing and the bootstrap activation need no other role. Content columns
-- are INSERT-only; activation, shadow and refusal are the only updates.
GRANT SELECT, INSERT ON admin.policy_bundles TO akr_app;
GRANT UPDATE (status, activation, change_request_id, shadow, activated_by, activated_at, superseded_at,
              refused_reason)
    ON admin.policy_bundles TO akr_app;
GRANT SELECT ON admin.purpose_codes TO akr_app;
-- Internal audit reads the policy that applied at any time (with the decision rows of phase 02).
GRANT SELECT ON admin.policy_bundles, admin.purpose_codes TO akr_readonly;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', v.t, c, 'internal'
FROM (VALUES
  ('policy_bundles', ARRAY['id','version','status','source','release_id','cedar_source','schema_text','schema_json',
                           'sod','manifest','sha256','validation','activation','change_request_id','shadow',
                           'installed_by','installed_at','activated_by','activated_at','superseded_at',
                           'refused_reason','created_at']),
  ('purpose_codes',  ARRAY['code','label','tiers','reason_required','sort_order','active','updated_by','updated_at',
                           'version'])
) AS v(t, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'policy_bundles', 'R9', 'retirement', NULL,
  'Retirement = superseded or refused; never deleted while an audit decision cites its version'),
 ('admin', 'purpose_codes',  'R9', 'retirement', NULL, 'Retirement = inactive; access events cite the code')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

```sql
-- migration 0012_org_branches_resets.sql
-- Phase 03. The organisation's branches (DR-27) with the deferred FK owed by phase 01, the four-eyes credential
-- reset (DR-49), and the re-enable mark the dormancy sweep needs.

CREATE TABLE IF NOT EXISTS admin.branches (
    id                 UUID PRIMARY KEY,
    code               TEXT NOT NULL CHECK (code ~ '^[A-Za-z0-9_-]{1,32}$'),   -- the CBS branch code where one exists
    name               JSONB NOT NULL CHECK (name ? 'ar' AND name ? 'en'),
    governorate        TEXT CHECK (governorate ~ '^IQ-[A-Z]{2}$'),   -- ISO 3166-2:IQ; FK to ref.governorates in phase 09
    region             TEXT CHECK (length(btrim(region)) BETWEEN 1 AND 64),
    liaison_user_id    UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    alternate_user_id  UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    archived_at        TIMESTAMPTZ,
    created_by         UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    version            INT NOT NULL DEFAULT 1,
    CHECK (alternate_user_id IS NULL OR liaison_user_id IS NOT NULL),
    CHECK (alternate_user_id IS NULL OR alternate_user_id <> liaison_user_id),
    CHECK (archived_at IS NULL OR (liaison_user_id IS NULL AND alternate_user_id IS NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_branches_code       ON admin.branches (lower(code));
CREATE INDEX IF NOT EXISTS idx_branches_liaison          ON admin.branches (liaison_user_id);
CREATE INDEX IF NOT EXISTS idx_branches_alternate        ON admin.branches (alternate_user_id);
CREATE INDEX IF NOT EXISTS idx_branches_created_by       ON admin.branches (created_by);

-- The deferred FK owed by phase 01 (DR-27). Pool branch lists and grant branch scopes are arrays inside rows and are
-- checked by the services (section 3), with a nightly consistency check in identity_sweep.
DO $$ BEGIN
    ALTER TABLE admin.users ADD CONSTRAINT fk_users_branch
        FOREIGN KEY (branch_id) REFERENCES admin.branches(id) ON DELETE RESTRICT;
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
-- idx_users_branch exists since migration 0005.

-- Set when an administrator re-enables an account; the dormancy clock restarts from it (section 3).
ALTER TABLE admin.users ADD COLUMN IF NOT EXISTS enabled_at TIMESTAMPTZ;

CREATE TABLE IF NOT EXISTS admin.credential_resets (
    id                     UUID PRIMARY KEY,
    user_id                UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    reason_code            TEXT NOT NULL CHECK (reason_code IN ('forgotten_password', 'lost_device',
                                                               'compromise_suspected', 'admin_mode_lockout')),
    remove_second_factors  BOOLEAN NOT NULL,
    status                 TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'completed', 'expired')),
    initiated_by           UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    initiated_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    initiate_stepup_id     UUID NOT NULL,   -- no FK: the step-up ledger is pruned after stepup.retention_days
    expires_at             TIMESTAMPTZ NOT NULL,
    confirmed_by           UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    confirmed_at           TIMESTAMPTZ,
    confirm_stepup_id      UUID,
    expired_at             TIMESTAMPTZ,
    CHECK (expires_at > initiated_at),
    -- Four-eyes (DR-49): the confirmer is neither the initiator nor the subject; nobody resets their own account.
    CHECK (initiated_by <> user_id),
    CHECK (confirmed_by IS NULL OR (confirmed_by <> initiated_by AND confirmed_by <> user_id)),
    CHECK ((status = 'completed') = (confirmed_by IS NOT NULL)),
    CHECK ((confirmed_by IS NULL) = (confirmed_at IS NULL)),
    CHECK ((confirmed_by IS NULL) = (confirm_stepup_id IS NULL)),
    CHECK ((status = 'expired') = (expired_at IS NOT NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_credential_resets_pending ON admin.credential_resets (user_id)
    WHERE status = 'pending';
CREATE INDEX IF NOT EXISTS idx_credential_resets_due          ON admin.credential_resets (expires_at)
    WHERE status = 'pending';
CREATE INDEX IF NOT EXISTS idx_credential_resets_user         ON admin.credential_resets (user_id, initiated_at DESC);
CREATE INDEX IF NOT EXISTS idx_credential_resets_initiated_by ON admin.credential_resets (initiated_by);
CREATE INDEX IF NOT EXISTS idx_credential_resets_confirmed_by ON admin.credential_resets (confirmed_by);

GRANT SELECT, INSERT ON admin.branches TO akr_app;
GRANT UPDATE (name, governorate, region, liaison_user_id, alternate_user_id, archived_at, updated_at, version)
    ON admin.branches TO akr_app;
GRANT UPDATE (enabled_at) ON admin.users TO akr_app;
GRANT SELECT, INSERT ON admin.credential_resets TO akr_app;
GRANT UPDATE (status, confirmed_by, confirmed_at, confirm_stepup_id, expired_at)
    ON admin.credential_resets TO akr_app;
-- Branches are organisation facts for reports; resets are part of the access-review record (SP-20).
GRANT SELECT ON admin.branches, admin.credential_resets TO akr_readonly;
GRANT SELECT (enabled_at) ON admin.users TO akr_readonly;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', v.t, c, 'internal'
FROM (VALUES
  ('branches',          ARRAY['id','code','name','governorate','region','liaison_user_id','alternate_user_id',
                              'archived_at','created_by','created_at','updated_at','version']),
  ('credential_resets', ARRAY['id','user_id','reason_code','remove_second_factors','status','initiated_by',
                              'initiated_at','initiate_stepup_id','expires_at','confirmed_by','confirmed_at',
                              'confirm_stepup_id','expired_at']),
  ('users',             ARRAY['enabled_at'])
) AS v(t, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'branches',          'R9', 'retirement', NULL, 'Retirement = archived; never deleted while referenced'),
 ('admin', 'credential_resets', 'R9', 'retirement', NULL, 'Retirement = completed or expired; access-review record')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

```sql
-- migration 0013_authz_admin_settings.sql
-- Phase 03. Authorisation, delegation, credential-reset and ops-cockpit settings. Bounds are in the registry
-- (DR-34, section 3); none of these carries a regulatory floor.

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('authz.reload_poll_s',             '30', 'duration_s', NULL, FALSE,
  '{"ar": "فاصل التحقق من حزمة الصلاحيات (ثوانٍ)", "en": "Policy bundle check interval (seconds)"}'),
 ('authz.resource_cache_ttl_s',      '30', 'duration_s', NULL, FALSE,
  '{"ar": "مدة التخزين المؤقت لخصائص الموارد (ثوانٍ)", "en": "Resource attribute cache lifetime (seconds)"}'),
 ('authz.analyst_networks',          '[]', 'json', NULL, TRUE,
  '{"ar": "نطاقات شبكة المحللين (CIDR)", "en": "Analyst network ranges (CIDR)"}'),
 ('authz.shadow_sample_pct',         '100', 'integer', NULL, FALSE,
  '{"ar": "نسبة الطلبات المقيّمة بالحزمة التجريبية", "en": "Share of requests evaluated by the shadow bundle (%)"}'),
 ('authz.mask_floor_ms',             '40', 'integer', NULL, TRUE,
  '{"ar": "أدنى زمن لرد \"غير موجود\" على الموارد المحمية (مللي ثانية)", "en": "Minimum latency of a masked not-found answer (ms)"}'),
 ('identity.credential_reset_ttl_h', '24', 'integer', NULL, TRUE,
  '{"ar": "مهلة تأكيد إعادة تعيين بيانات الدخول (ساعات)", "en": "Credential reset confirmation window (hours)"}'),
 ('delegation.max_days',             '30', 'integer', NULL, TRUE,
  '{"ar": "أقصى مدة للتفويض (أيام)", "en": "Maximum delegation length (days)"}'),
 ('ops.probe_stale_s',               '180', 'duration_s', NULL, FALSE,
  '{"ar": "عمر لقطة الفحص قبل اعتبارها قديمة (ثوانٍ)", "en": "Probe snapshot age before it counts as stale (seconds)"}'),
 ('ops.cert_warn_days',              '30', 'integer', NULL, FALSE,
  '{"ar": "التنبيه قبل انتهاء الشهادة (أيام)", "en": "Certificate expiry warning (days)"}'),
 ('ops.disk_warn_pct',               '80', 'integer', NULL, FALSE,
  '{"ar": "نسبة امتلاء القرص للتنبيه", "en": "Disk use warning (%)"}'),
 ('ops.disk_critical_pct',           '90', 'integer', NULL, FALSE,
  '{"ar": "نسبة امتلاء القرص الحرجة", "en": "Disk use critical (%)"}'),
 ('ops.queue_age_warn_s',            '600', 'duration_s', NULL, FALSE,
  '{"ar": "عمر أقدم مهمة منتظرة قبل التنبيه (ثوانٍ)", "en": "Oldest queued job age before a warning (seconds)"}'),
 ('schedule.ops_probe',              '"0 * * * * *"', 'cron', NULL, FALSE,
  '{"ar": "موعد فحص حالة المنصة", "en": "Platform probe schedule"}')
ON CONFLICT (key) DO NOTHING;
```

Notes:
- `authz.analyst_networks` empty means the network attribute is `unknown` for every request, and the suspicion-tier
  permits of later phases require `analyst`: an installation that has not declared its analyst network cannot open a
  suspicion-tier record at all (fail closed). The install checklist sets it; it is governed, so after phase 04 only a
  change request moves it.
- No new role. The read-only role gains the four tables; `akr_ingest` gains nothing (phase 10 grants it `(id, code)`
  on `admin.branches` when `canon.branch` links to it).
- `policy_bundles.cedar_source` can be large (tens of kilobytes); it is read once per load, never per request.

## 2. DDD Implementation (Rust)

Crate placement per the phase 00 workspace: the Cedar engine, schema, action inventory, entity building, adapters,
decision record, simulator and shadow evaluation in `authz` (it depends on `domain`, `audit` and `cedar-policy`
4.x, never on `db`); entities, value objects, repository traits and the administration services in `domain::org`,
`domain::useradmin` and `domain::ops`; Pg impls in `db`; the middleware, extractors and routers in `api`. The
`AllowAllAuthorizer` stub of phase 00 is deleted.

- **Domain entities** (`domain`, derives `Debug, Serialize, Deserialize, FromRow, ToSchema`):
  - `PolicyBundle { id, version, status: BundleStatus, source: BundleSource, release_id, sha256: Hex32, sod:
    SodRules, manifest: BundleManifest, validation: Value, activation: Option<Activation>, change_request_id, shadow,
    installed_by, installed_at, activated_by, activated_at, superseded_at, refused_reason, created_at }` (the source
    text and schema are loaded by `PolicyBundleRepository::content(id)` and never serialised into lists).
  - `Branch { id, code, name: I18nText, governorate, region, liaison_user_id, alternate_user_id, archived_at,
    created_by, created_at, updated_at, version }`
  - `PurposeCodeDef { code: PurposeCode, label: I18nText, tiers: Vec<Tier>, reason_required, sort_order, active,
    updated_by, updated_at, version }`
  - `CredentialReset { id, user_id, reason_code: ResetReason, remove_second_factors, status: ResetStatus,
    initiated_by, initiated_at, initiate_stepup_id, expires_at, confirmed_by, confirmed_at, confirm_stepup_id,
    expired_at }`
  - `User` (phase 01) gains `enabled_at`; `Pool` and `Delegation` are phase 01's.
- **Value objects:** `BundleStatus`, `BundleSource { Release, Authored }`, `Activation { Bootstrap, ChangeRequest }`,
  `SodRules { exclusive_roles: Vec<RoleCode>, action_pairs: Vec<(ActionName, ActionName)> }`, `ResetReason`,
  `ResetStatus`, `OffboardReason { Resigned, Terminated, Transferred, ContractEnded, Other }`, `SuspendReason {
  Investigation, ExtendedAbsence, SecurityIncident, Other }` (codes only, never free text), `NetworkOrigin { Analyst,
  Other, Unknown }`, `TileState { Ok, Warn, Critical, Unknown }`. `Tier` and `PurposeCode` are phase 02's.
- **The `authz` crate** (pinned contracts; later phases extend the inventories, never the shapes):

```rust
#[async_trait]
pub trait Authorizer: Send + Sync {
    /// Gate 2. Evaluates the active bundle (and the shadow bundle when one is set), writes the decision row, and
    /// returns the decision. The scope gate (gate 1) has already run in the middleware.
    async fn authorize(&self, req: AuthzRequest<'_>) -> Result<Decision, AuthzError>;
    /// Drops the cached attributes of one resource; services call it after commit.
    async fn invalidate_resource(&self, kind: ResourceKind, id: Uuid);
}

pub struct AuthzRequest<'a> {
    pub ctx: &'a AuthContext,            // phase 01
    pub request: &'a RequestCtx,         // phase 02
    pub action: Action,
    pub resource: ResourceRef,
    pub purpose: Option<&'a PurposeCode>,
}

pub enum ResourceRef { System, User(Uuid), Branch(Uuid), Pool(Uuid), Delegation(Uuid), PolicyBundle(Uuid),
                       Setting(SettingKey), Flag(FlagKey), Job(Uuid), CredentialReset(Uuid) /* phases append */ }

pub struct Decision {
    pub effect: Effect,                  // Allow | Deny
    pub deny: Option<DenyReason>,
    pub policy_ids: Vec<String>,         // determining policies (@id annotations)
    pub actor_role: Option<RoleCode>,    // from the determining permit's @role (section 3)
    pub on_behalf_of: Option<Uuid>,      // set when actor_role is held only through a delegation
    pub attrs_hash: [u8; 32],
    pub bundle_version: i32,
    pub audit_id: Uuid,                  // the authz.allowed / authz.denied row
}

pub enum DenyReason { Role, Forbidden, OutOfScope, Tier, Conflict, Missing }
impl DenyReason {
    /// True when the answer must be the 404 of a missing record (section 3, masking).
    pub fn masks(self, kind: ResourceKind) -> bool {
        kind.masked() && matches!(self, Self::OutOfScope | Self::Tier | Self::Conflict | Self::Missing)
    }
}
```

  - `actions`: `Action` (one variant per action name, `as_str()` gives `"user.update"`), `ActionGroup`, and the route
    registry `ROUTES: &[RouteAction]` with `RouteAction { method, path_template, action, resource: ResourceSpec {
    System | PathId(ResourceKind, &'static str) | PathKey(ResourceKind, &'static str) }, scopes: &'static [Scope],
    logged: bool }` (`logged = false` only for the two public routes, which never reach the layer). Filled by the
    `authz_route!` macro beside each `#[utoipa::path]`, which also sets the OpenAPI extension `x-akr-action`.
  - `schema`: the shipped `.cedarschema` text and its parsed form; a test asserts that the schema's actions equal
    `Action::ALL` both ways.
  - `entities`: principal and resource entity builders that produce, side by side, the Cedar `Entities` and the JSON
    trace that `attrs_hash` digests and the simulator shows (one builder for both, so the trace is what was evaluated,
    the DBS rule).
  - `adapters`: `trait ResourceAdapter { fn kind(&self) -> ResourceKind; async fn load(&self, id: &ResourceId) ->
    Result<Option<ResourceAttrs>, AuthzError>; }` and the registry; this phase registers user, branch, pool,
    delegation, policy bundle, setting, flag, job and credential reset. `ResourceKind::masked()` is false for all of
    them; phase 15 onwards registers masked kinds (customer, alert, case, report, evidence, RFI, freeze).
  - `decision`: `DecisionRecord` conversion for `audit_log.decision` (`{policy_ids, effect, attrs_hash}`, phase 02)
    and `attrs_hash = SHA-256(Canon encoding of {action, context, principal, resource})` with the phase 02 canonical
    encoder, never `serde_json` (DBS lesson: an insertion-ordered map makes the hash depend on construction order).
  - `loader`: `LoadedBundle { id, version, sha256, policies: PolicySet, schema: Schema, sod: SodRules, role_of:
    HashMap<PolicyId, RoleTag> }` and `load(content) -> Result<LoadedBundle, Vec<BundleDiagnostic>>`.
  - `service`: `AuthzService` (implements `Authorizer`), `Simulator`, `AccessMatrixBuilder`, `BundleWatcher`.
  - `sod`: `SodRules::check_roles(&BTreeSet<RoleCode>) -> Result<(), SodViolation>` used by delegations now and by
    the grant change request of phase 04.
- **Create/Update DTOs** (`Deserialize, ToSchema`; Update fields all `Option<T>`): `CreateBranch { code, name,
  governorate?, region? }`, `UpdateBranch { name?, governorate?, region?, liaison_user_id?, alternate_user_id?,
  archived? }` (liaison and alternate use `Option<Option<Uuid>>` so `null` clears); `CreatePool { name, kind,
  branch_ids }`, `UpdatePool { name?, branch_ids?, archived? }`; `CreateDelegation { to_user, roles, valid_from,
  valid_to, reason_code }`; `CreateUser { email, display_name, locale?, branch_id?, vetting_status?,
  vetting_expires_on?, expires_at? }`, `UpdateUser { email?, display_name?, locale?, branch_id?, vetting_status?,
  vetting_expires_on?, expires_at? }`; `SuspendRequest { reason_code }`; `EnableRequest { expires_at? }`;
  `OffboardRequest { reason_code }`; `ResetRequest { reason_code, remove_second_factors }`; `UpdateSetting { value }`;
  `UpdateFlag { enabled?, scope? }`; `SimulateRequest` (section 3).
- **Repository traits** (`domain::repositories`; phase 01 traits gain the methods marked "+"):
  - `PolicyBundleRepository { active() -> Option<PolicyBundle>; active_id() -> Option<Uuid>; shadow() ->
    Option<PolicyBundle>; get(id); content(id) -> Option<BundleContent>; next_version(tx) -> i32; insert(tx,
    NewBundle) -> PolicyBundle; activate(tx, id, actor, Activation, cr: Option<Uuid>) -> PolicyBundle;
    set_shadow(tx, id, on: bool); refuse(tx, id, reason) }`. `activate` supersedes the previous active row and
    activates the new one in one transaction, then sends `NOTIFY akr_policy, '<id>'`.
  - `BranchRepository { list(BranchFilter) -> Vec<Branch>; get(id); by_code(code); insert(tx, NewBranch) -> Branch;
    update(tx, id, BranchPatch, expected_version) -> Branch; existing(ids) -> Vec<Uuid>; covering_liaison(user_id) ->
    Vec<Branch>; clear_liaison(tx, user_id) -> Vec<Uuid>; usage(id) -> BranchUsage { users, pools, grants } }`
  - `PoolRepository` + `{ list(PoolFilter); insert(tx, NewPool) -> Pool; update(tx, id, PoolPatch, expected_version)
    -> Pool; referenced_by_grants(id) -> i64 }`
  - `UserRepository` + `{ list(UserFilter, Cursor, limit) -> Page<UserRow>; detail(id) -> Option<UserDetail>;
    insert_admin(tx, NewAdminUser); update_profile(tx, id, UserPatch, expected_version) -> User; suspend(tx, id);
    enable(tx, id, expires_at) -> User; unlock(tx, id) -> bool; set_recovery(tx, id, temp_expiry); offboard(tx, id);
    owned_services(owner_id) -> Vec<Uuid> }`
  - `GrantRepository` + `{ list_for_user(user_id) -> Vec<RoleGrant>; revoke(tx, grant_id, user_id, actor) ->
    Option<RoleGrant>; revoke_all_for_user(tx, user_id, actor) -> Vec<Uuid>; active_holder_count(role, excluding:
    Option<Uuid>) -> i64; all_active() -> Vec<RoleGrant> }`
  - `DelegationRepository` + `{ list(DelegationFilter, Cursor, limit); get(id); insert(tx, NewDelegation) ->
    Delegation; end(tx, id, actor) -> Option<Delegation>; end_all_for_user(tx, user_id, actor) -> Vec<Uuid> }`
  - `CredentialResetRepository { insert(tx, NewReset) -> CredentialReset; get(id); pending_for(user_id); list_recent(
    since) -> Vec<CredentialReset>; complete(tx, id, actor, stepup_id) -> bool; expire_due(tx, now, limit) ->
    Vec<CredentialReset> }` (`complete` is `UPDATE ... WHERE id = $1 AND status = 'pending' AND expires_at > now()`)
  - `PurposeCodeRepository { list(active_only) -> Vec<PurposeCodeDef>; get(code) }`
  - `JobAdminRepository { list(JobFilter, Cursor, limit) -> Page<JobRow>; retry(tx, id) -> Option<JobRow>;
    counts_by_kind() -> Vec<KindCounts>; unknown_kinds(known: &[&str]) -> Vec<String> }`
  - `OpsRepository { outbox_counts(); timers_due(); anchor_status() -> Vec<AnchorTile>; partitions_ahead() -> i32;
    last_verify() -> Option<VerifySummary>; evidence_pending() -> EvidenceTile }` (all aggregate counts)
- **Repository impls** (`db`): `PgPolicyBundleRepository` (`activate` = `UPDATE ... SET status = 'superseded',
  superseded_at = now() WHERE status = 'active'`, then the activation update, then `pg_notify`), `PgBranchRepository`,
  `PgCredentialResetRepository`, `PgPurposeCodeRepository`, `PgJobAdminRepository` (`retry` = `UPDATE admin.jobs SET
  status = 'queued', attempts = 0, run_at = now(), finished_at = NULL, locked_at = NULL, locked_by = NULL WHERE id = $1
  AND status = 'dead' RETURNING ...`), `PgOpsRepository`; phase 01 impls gain the "+" methods. The user list is
  keyset-paginated on `(created_at DESC, id DESC)` (`idx_users_created_id`, migration 0005); `q` matches a prefix of
  the e-mail or a case-folded substring of either name (staff counts are in the thousands at most, so no trigram
  index).
- **Ports** (`domain::ports`): `ConflictCheck { check(principal, subject_party) -> ConflictOutcome { Clear, Conflict
  } }` (`NoConflictCheck` until phase 15 ships the relationship-conflict service, SP-29); `OffboardingStep` (section
  3); `DelegationAuthority { check(from, to, roles) -> Result<(), DelegationRefusal> }` (`NoMatrix` until the
  authority matrix of phase 09); `MetricsQuery` (the Prometheus HTTP API, worker only; adapter in `common`).
- **utoipa schemas:** `BranchView { id, code, name, governorate, region, liaison: Option<UserRef>, alternate:
  Option<UserRef>, archived_at, version }`; `PoolView { id, name, kind, branch_ids, archived_at, version }`;
  `DelegationView { id, from: UserRef, to: UserRef, roles, valid_from, valid_to, reason_code, ended_at, active }`;
  `UserRef { id, display_name }`; `UserListItem { id, kind, display_name, email, status, disabled_reason, branch_id,
  roles: [RoleCode], locked, last_authenticated_at, expires_at }`; `UserDetail { ...UserListItem, locale,
  vetting_status, vetting_expires_on, forced_change, password_changed_at, enabled_at, grants: [GrantView],
  delegations: { to_user, from_user }, tokens: [TokenRegisterRow], factors: { count, methods }, pending_reset:
  Option<ResetRef>, liaison_for: [BranchRef], version }`; `GrantView { id, role, scope, source, valid_from, valid_to,
  granted_by: UserRef, revoked_at, revoked_by }`; `UserCreated { user: UserDetail, temp_password,
  temp_password_expires_at }`; `OffboardReport { user_id, steps: [{ step, count }], kratos: done | deferred }`;
  `CredentialResetView { id, user: UserRef, reason_code, remove_second_factors, status, initiated_by: UserRef,
  initiated_at, expires_at, confirmed_by: Option<UserRef>, confirmed_at }`; `ResetCompleted { reset_id,
  temp_password, temp_password_expires_at }`; `PolicyBundleView { id, version, status, source, release_id, sha256,
  manifest, sod, activation, activated_at, activated_by: Option<UserRef>, schema_json, shadow:
  Option<ShadowView>, instances: [InstanceLoad] }`; `ShadowView { bundle_id, version, evaluated, differing, since }`;
  `InstanceLoad { instance_id, bundle_version, loaded_at }`; `SimulateRequest` and `SimulateResult` (section 3);
  `AccessMatrix` (section 3); `PurposeCodeView { code, label, tiers, reason_required }`; `SettingView { key, value,
  value_type, floor, governed, bounds, label, updated_by: Option<UserRef>, updated_at, version }`; `FlagView { key,
  enabled, scope, label, updated_by, updated_at, version }`; `JobView { id, kind, status, attempts, max_attempts,
  run_at, created_at, finished_at, last_error }` (never the payload); `OpsCockpit` (section 3).
- **Route table** (paths relative to `/api/v1`; "SU" names the step-up action and class; every route has the Cedar
  action in the last column of section 3's action table):

| Method | Path | Scope | Description |
|-|-|-|-|
| GET | `/org/branches` | self | Branches with governorate, region, liaison and alternate; `?archived=true` includes archived |
| POST | `/org/branches` | admin.users | Create a branch (code, names, governorate, region) |
| PATCH | `/org/branches/{id}` | admin.users | Names, governorate, region, liaison and alternate (L holders covering the branch), archive; `If-Match` |
| GET | `/org/pools` | self | Work pools with kind and branch list |
| POST | `/org/pools` | admin.users | Create a pool (name, kind, branch list) |
| PATCH | `/org/pools/{id}` | admin.users | Name, branch list, archive; members come from grant scopes (phase 04 change requests); `If-Match` |
| GET | `/org/delegations` | self | Delegations to and from me; `?all=true` lists every delegation for `admin.users` holders (cursor) |
| POST | `/org/delegations` | self | Delegate own roles during leave; **SU** (`delegation.create`, `access_change`) |
| DELETE | `/org/delegations/{id}` | self | End a delegation (delegator, delegate or an administrator) |
| GET | `/admin/users` | admin.users | Users (cursor; filters `status`, `kind`, `role`, `branch_id`, `locked`, `q`) |
| GET | `/admin/users/{id}` | admin.users | User detail: account state, grants, delegations, token register, factor count, pending reset |
| POST | `/admin/users` | admin.users | Create a person's account with a temporary password shown once; **SU** (`user.create`, `identity_admin`) |
| PATCH | `/admin/users/{id}` | admin.users | Names, e-mail, language, branch, vetting, account expiry; `If-Match` |
| DELETE | `/admin/users/{id}/grants/{grant_id}` | admin.users | Revoke a grant at once (no second approver); never the last ADM or SEC grant |
| POST | `/admin/users/{id}/unlock` | admin.users | Clear a sign-in lockout |
| POST | `/admin/users/{id}/disable` | admin.users | Suspend an account with a reason code (grants kept) |
| POST | `/admin/users/{id}/enable` | admin.users | Re-enable after dormancy, expiry or suspension; **SU** (`user.enable`, `access_change`) |
| POST | `/admin/users/{id}/offboard` | admin.users | Disable, revoke sessions and tokens, end grants and delegations, run the registered sweeps; **SU** (`user.offboard`, `identity_admin`) |
| POST | `/admin/users/{id}/reset-credentials` | admin.users | Request a credential reset (temporary password, optional second-factor removal); **SU** (`user.reset_credentials`, `identity_admin`) |
| GET | `/admin/credential-resets` | security.ops | Pending resets and those of the last 30 days |
| POST | `/admin/credential-resets/{id}/confirm` | security.ops | Confirm a reset; returns the temporary password once; **SU** (`credential_reset.confirm`, `identity_admin`); **4E** |
| GET | `/admin/policies` | admin.policies | Active bundle (version, hash, schema, SoD rules), shadow bundle and its differences, loaded version per instance |
| POST | `/admin/policies/simulate` | admin.policies | Evaluate a principal, action and resource against the active or a staged bundle, with the attribute trace; audited |
| GET | `/admin/access-matrix` | security.ops, audit.read | Generated access matrix: role to actions from the active bundle, and every principal's grants (SP-20) |
| GET | `/access/purpose-codes` | self | Active purpose codes with the tiers they apply to |
| GET | `/admin/settings` | admin.system | Settings with floors, bounds and the governed mark |
| PUT | `/admin/settings/{key}` | admin.system | Change a non-governed setting; a governed key answers 409 until phase 04 makes it a change request; `If-Match` |
| GET | `/admin/flags` | admin.system | Feature flags |
| PUT | `/admin/flags/{key}` | admin.system | Toggle a flag or change its scope; `If-Match` |
| GET | `/admin/jobs` | admin.system | Jobs by kind and status (cursor); never the payload |
| POST | `/admin/jobs/{id}/retry` | admin.system | Re-queue a dead job |
| GET | `/admin/ops` | admin.system | Ops cockpit v1: jobs, outbox, timers, backups and drills, anchors and verification, partitions, evidence scanning, disk, certificates, OpenBao seal, Valkey, bundle |

  Thirty-two operations. "security.ops, audit.read" means either scope passes gate 1. `PUT /admin/policies` (CR),
  `POST /admin/users/{id}/grants` (CR) and `PUT /org/authority-matrix` (CR) are phase 04 and 09; `GET
  /org/authority-matrix` is phase 09; the access-review lifecycle (`/admin/access-reviews`) is phase 25; case access
  lists, break-glass and access history (API.md 3.3) are phases 18, 25 and 24.
- **Idempotency exemptions** (added to the phase 00 list with their reasons, asserted by a test): `POST /admin/users`
  and `POST /admin/credential-resets/{id}/confirm` (the response carries a temporary password the ledger must not
  hold; the consumed step-up assertion makes a repeat impossible without a new step-up, and a repeated create fails
  on the unique e-mail); `POST /admin/policies/simulate` (changes nothing but the audit trail). Every other mutation
  above requires `Idempotency-Key`; every versioned resource requires `If-Match`.
- **Routers:** `api::routes::org::router()`, `admin_users::router()`, `authz::router()` and
  `admin_system::router()` (`OpenApiRouter<AppState>`), merged in `build_app`; the authz layer (section 3) fills the
  phase 00 slot between the auth layer and idempotency.

## 3. Business Logic

### 3.1 The bundle

**Layout in the repository** (`backend/policies/bundle/`, shipped inside every signed release, DR-47):
`schema.cedarschema`; `00_baseline.cedar`, `10_self.cedar`, `20_org.cedar`, `30_user_admin.cedar`,
`40_policy_admin.cedar`, `50_system.cedar` (later phases add files in their own number ranges: 55 and 56 change
control and workflow (phase 04), 60 audit, 70 sources and mapping, 80 detection, 90 casework, A0 reporting, B0 branch window, C0 governance); `sod.json`; `manifest.json`
(`{"version": n, "files": [{"name", "sha256"}]}` in evaluation order). The canonical bundle hash is SHA-256 over, for
each file in manifest order, `name || 0x00 || sha256(file) || 0x0A`; `akr-ctl policy validate` prints it and the
release notes carry it.

**Schema** (excerpt of `schema.cedarschema`; namespace `Akr`):

```cedar
namespace Akr {
  type Ctx = {
    via: String,            // "session" | "pat" | "client_credentials"
    aal: String,            // "aal1" | "aal2"
    stepup_age_s: Long,     // seconds since the newest step-up in this session; 2147483647 when none
    network: String,        // "analyst" | "other" | "unknown" (authz.analyst_networks)
    purpose: String,        // a validated purpose code, or ""
    local_hour: Long,       // bank time zone
    local_weekday: Long,    // 1 = Monday
  };
  entity Branch = { archived: Bool };
  entity Pool = { kind: String, archived: Bool, branches: Set<Branch> };
  entity User = {
    kind: String,                 // human | service | system
    status: String,               // active | disabled
    roles: Set<String>,           // every effective role code, direct and delegated
    delegated_roles: Set<String>, // the subset held only through a delegation
    all_branches: Bool,
    branches: Set<Branch>,
    pools: Set<Pool>,
    vetting: String,              // none | pending | vetted | lapsed (vetted with a passed expiry reads lapsed)
    sod_conflict: Bool,           // the role set breaks sod.json (section 3.4)
  };
  entity System;
  entity Delegation = { from_user: User, to_user: User };
  entity PolicyBundle = { status: String };
  entity Setting = { governed: Bool };
  entity Flag;
  entity Job = { status: String };
  entity CredentialReset = { subject: User, initiated_by: User, status: String };

  action "self", "org_admin", "user_admin", "policy_admin", "system_admin";   // groups
  action "user.update", "user.revoke_grant", "user.unlock", "user.disable", "user.enable", "user.offboard",
         "user.reset_credentials", "user.read" in ["user_admin"]
    appliesTo { principal: User, resource: User, context: Ctx };
  action "credential_reset.confirm" appliesTo { principal: User, resource: CredentialReset, context: Ctx };
  // ... every action of the table in section 3.10, each with its principal, resource and context types
}
```

**Baseline and examples** (`00_baseline.cedar`, `30_user_admin.cedar`):

```cedar
// An account that is not active can do nothing, whatever any permit says (DBS 00_deny_disabled).
@id("baseline.inactive")
forbid (principal, action, resource) when { principal.status != "active" };

// A role set that breaks the segregation rules of sod.json is refused everything until it is fixed (SP-21).
@id("baseline.sod_conflict")
forbid (principal, action, resource) when { principal.sod_conflict };

// Administration is an interactive act: a session at AAL2, never a token (D7).
@id("baseline.admin_session_only")
forbid (principal, action in [Akr::Action::"org_admin", Akr::Action::"user_admin", Akr::Action::"policy_admin",
                              Akr::Action::"system_admin"], resource)
when { context.via != "session" || context.aal != "aal2" };

@id("useradmin.adm") @role("ADM")
permit (principal, action in Akr::Action::"user_admin", resource is Akr::User)
when { principal.roles.contains("ADM") && resource.kind == "human" };

// Nobody administers their own account (SP-21).
@id("useradmin.no_self")
forbid (principal, action in Akr::Action::"user_admin", resource is Akr::User)
when { resource == principal && action != Akr::Action::"user.read" };

@id("reset.sec") @role("SEC")
permit (principal, action == Akr::Action::"credential_reset.confirm", resource is Akr::CredentialReset)
when { principal.roles.contains("SEC") };

// Four-eyes on credential resets (DR-49): the confirmer is neither the initiator nor the subject.
@id("reset.four_eyes")
forbid (principal, action == Akr::Action::"credential_reset.confirm", resource is Akr::CredentialReset)
when { resource.initiated_by == principal || resource.subject == principal };
```

**`sod.json`** (shipped default; the bank changes it with the bundle, by change request):

```json
{
  "exclusive_roles": ["ADM", "SEC", "IA", "AS", "VS"],
  "action_pairs": [["user.reset_credentials", "credential_reset.confirm"]],
  "basis": "research file 30 s.7.4: administrators and security never hold business roles; auditors and assessors are independent; vendor support holds nothing else"
}
```

An exclusive role must be the only role a person holds (directly or by delegation). An action pair may not both be
permitted to a principal that holds a single role (checked against the bundle by a test, section 6). A role-level
pair for change requests is not added: `MRO` and `ADM` must both propose and approve, so phase 04 enforces the
separation per request instead (DR-54, DR-57).

**`loader::load(content)`** (run at install, at activation and by every instance on reload):
1. Parse the schema; parse each policy file; policy ids must be unique and present (`@id`) on every policy.
2. `Validator::validate(policies, ValidationMode::Strict)`; any error refuses the bundle with the diagnostics.
3. Inventory: the schema's actions equal `Action::ALL` both ways; every action used by `ROUTES` exists; every `permit`
   carries `@role("<code>")` with a code from the nineteen (or `"*"` for the `self` group, which yields no
   `actor_role`); `00_baseline.cedar` holds the three baseline policies by id.
4. `sod.json` parses, its roles are known codes, its action names exist.
5. Compute `role_of` from the annotations and the canonical hash; return the `LoadedBundle`.

**`akr-ctl policy`** (zone Z5; the OS user and PAM session go into the audit event, as in phase 00):
- `policy validate <dir>`: runs `load` and prints diagnostics and the hash; no database.
- `policy install <dir> --release <id>`: validates, refuses a hash already present, inserts a `staged` row with
  `source = release`, `next_version`, the validation summary; audit `policy_bundle.installed {version, sha256,
  release_id}`.
- `policy activate --bootstrap <version>`: allowed only while no bundle has ever been active (the first install of an
  installation, DR-47, the same rule as the identity bootstrap of phase 01); activation `bootstrap`, `activated_by`
  = the system principal; audit `policy_bundle.activated {version, activation: bootstrap}`. After that the command
  refuses: activation is a change request (phase 04), whose approval calls `PolicyBundleRepository::activate` with
  `Activation::ChangeRequest(id)`.
- `policy shadow <version> on|off`: marks a staged bundle as shadow (or clears it); audit
  `policy_bundle.shadow_changed`. Phase 04 sets shadow automatically while a bundle's change request is open.
- A staged bundle that fails `load` at activation is marked `refused` with the first diagnostic code; the active one
  keeps serving.

**`BundleWatcher`** (in every `akr-api` instance; hot reload, M3 edge case "bundle with a syntax error"):
1. At start: load the active bundle; none, or it fails `load` -> the instance is not ready (readiness `authz_bundle`
   false) and every `/api/v1` request answers 503 `E_UNAVAILABLE`. There is no allow-all fallback anywhere.
2. `LISTEN akr_policy` on a dedicated connection, plus a poll of `active_id()` and the shadow id every
   `authz.reload_poll_s` (a missed notification is caught by the poll).
3. On a new id: fetch the content, `load`, verify that the stored hash equals the recomputed one, swap the
   `ArcSwap<LoadedBundle>` atomically (requests in flight finish on the old bundle). Failure -> keep the previous
   bundle, increment `akr_authz_bundle_load_failures_total`, audit `policy_bundle.load_failed {version, code}` once
   per instance and bundle, and set the ops tile to critical.
4. Report `instance_id`, version and load time to Valkey `authz:instances` (hash, 2-minute TTL per field) for the
   cockpit and `GET /admin/policies`.

### 3.2 The authz layer and `AuthzService::authorize`

The layer runs after the phase 01 auth layer and before idempotency, on every `/api/v1` route except the two public
ones:

1. Route lookup: the matched route template -> its `RouteAction`. No entry -> 500 `E_INTERNAL` and an error log (the
   coverage test makes this unreachable in a build).
2. **Gate 1, scope:** `AuthContext.scopes` must contain one of the route's scopes, else 403 `E_SCOPE` and an
   `authz.denied {gate: scope}` row. A token keeps only the intersection of its scopes and its owner's (phase 01), so
   a narrow token never borrows its owner's authority; Cedar then sees the owner's roles, which is why gate 1 comes
   first (DBS lesson).
3. Purpose: `X-Access-Purpose`, when present, must be an active code of `admin.purpose_codes` (cached 60 s), else 422
   `E_PURPOSE_REQUIRED`; it goes into the context. Whether a purpose is required is decided by the access layer of
   phase 02 for the tier of the record actually returned, and by policy for the masked kinds.
4. Resource: `ResourceSpec::System` -> the `System` entity; `PathId(kind, param)` -> the id from the path and
   `adapter(kind).load(id)` through the cache (section 3.3). Not found -> `DenyReason::Missing` (step 7).
5. Build entities and context: the principal from `AuthContext` (roles, delegated roles, branch scope, pools) plus the
   principal extension (`kind`, `status`, `vetting`), cached under `authz:pext:{user_id}:{authgen}` with the phase 01
   generation counter so `invalidate_user` also kills it; `sod_conflict` computed from the role set and the loaded
   `SodRules`; the context from `AuthContext` (via, AAL), `StepUpService::fresh_until` (phase 01), the client IP
   against `authz.analyst_networks`, the purpose, and the bank-local clock.
6. Evaluate the active bundle (inside `Authorizer::authorize` of section 2: the cedar-policy call `is_authorized`). Allow -> `actor_role` = the `@role` of the
   determining permits intersected with the principal's roles, the first in the MODULES s.0 order; `on_behalf_of` =
   the delegation's `from_user` when that role is in `delegated_roles` only. Deny -> `DenyReason`: a determining
   `forbid` -> `Forbidden`; no permit and the resource exists -> the adapter's classification of the attribute that
   failed (`OutOfScope` when a branch or pool test failed, `Tier` when a tier test failed, `Conflict` from the
   `ConflictCheck` port) or `Role`. Policies tag the attribute they test with `@deny("out_of_scope" | "tier")` on
   the matching `forbid`, so the classification reads annotations, never error strings.
7. **Decision row** (DR-48): `audit_log` event `authz.allowed` or `authz.denied` with the action, the resource
   reference, `actor_role`, `on_behalf_of`, `decision = {policy_ids, effect, attrs_hash}` and details `{gate,
   deny_reason, bundle_version}`, written by `AuditSink::emit_standalone` in its own short transaction **before** the
   handler runs (or before the refusal is sent). A write failure -> 503 `E_UNAVAILABLE`: an unrecorded decision is
   not acted on. A shadow evaluation (section 3.5) writes in the same transaction.
8. Masked read refusals (a masked kind and a read action): also an access event (`deny`, or `conflict`) through
   `AccessLog::record` in its own transaction (DR-46), with the tier from the loaded attributes. Missing records write
   no access event (there is no record to name).
9. Answer: allow -> put `Decided { decision, attrs }` in the request extensions and continue (handlers copy
   `decision.record()` into their own audit event and read `actor_role` for `AccessScope`); deny with
   `reason.masks(kind)` -> the masking response (section 3.6); other denies -> 403 `E_FORBIDDEN` (or
   `E_SELF_APPROVAL` when the determining policy carries `@deny("self_approval")`).

`authorize` is also callable from handlers for decisions on objects not named in the path (a list row, the second
resource of a merge); those calls write their decision rows the same way.

### 3.3 Attribute resolution and the cache

- Roles never come from a cache of their own: they come from the request's `AuthContext`, which phase 01 rebuilds
  when `authgen` moves, so a cached attribute can never grant an authority the person lost (DBS rule).
- Resource attributes: `authz:res:{kind}:{id}` in Valkey, JSON, TTL `authz.resource_cache_ttl_s`. Services that change
  an attribute call `invalidate_resource(kind, id)` after commit (user, branch, pool, delegation, reset, bundle in
  this phase). Valkey down -> load from PostgreSQL and continue (fails open on cache infrastructure); PostgreSQL down
  -> 503 (fails closed on facts), the DBS posture.
- A masked kind (phase 15 on) never caches attributes that would let a refused caller learn anything: the cache is
  keyed by resource, not by caller, and is never returned to a client.

### 3.4 Segregation of duties

- Role level (DR-50): `SodRules::check_roles` refuses a role set in which an exclusive role sits with any other role.
  Called by `DelegationService::create` (section 3.9) and by the role-grant change request of phase 04 at proposal
  and again at approval. At request time the `baseline.sod_conflict` forbid refuses everything to a principal whose
  effective roles break the rule (a grant made before a bundle tightened the rule, or a delegation racing a grant),
  and the cockpit lists such principals.
- Action pairs: a bundle test evaluates, for every role alone, every action of `sod.json` pairs; a role permitted both
  sides fails the build (SP-21).
- Resource level: `forbid` rules (no self-administration, the reset four-eyes) plus the database CHECKs on
  `credential_resets`; phase 04 adds the change-request and task pairs with their CHECKs.
- Bootstrap: phase 01 already refuses ADM and SEC for the same person; the rule is now general.

### 3.5 Simulator, shadow evaluation and the access matrix

**`Simulator::simulate(ctx, SimulateRequest)`** (SP-25; `POST /admin/policies/simulate`):
- Request: `{ principal: {user_id} | {synthetic: {roles, branches | all_branches, pools, vetting, kind}}, action,
  resource: {kind, id} | {kind, synthetic: {...attributes}}, context: {via?, aal?, stepup_age_s?, network?, purpose?,
  local_hour?}, bundle: "active" | <staged bundle id> }`.
1. The bundle must be active or staged (a refused or superseded one -> 422 at `/bundle`).
2. A real `resource.id` is accepted only for kinds that are not masked; a masked kind accepts synthetic attributes
   only (422 `E_VALIDATION` at `/resource/id`, `masked_kind`). The simulator can therefore never be used to learn
   whether a suspicion-tier record exists or who is on its access list.
3. Build entities with the same builders as section 3.2 (real principals load as they would at request time,
   `sod_conflict` included); evaluate; when `bundle` is staged also evaluate the active bundle.
4. Audit `authz.simulated {action, resource_kind, principal: user id or "synthetic", bundle_version, effect}`; no
   decision row (nothing was decided).
5. Return `SimulateResult { effect, deny_reason, masked, determining_policies: [{id, file, effect, role}], errors,
   principal_trace, resource_trace, context, attrs_hash, active_effect? }`.

**Shadow evaluation** (dry run before enforcement): while a bundle is marked shadow, `authorize` also evaluates it for
`authz.shadow_sample_pct` of requests (by a hash of the correlation id, so a sampled request is deterministic). A
different effect -> event `authz.shadow_diff {action, resource_kind, active_effect, shadow_effect, shadow_version,
attrs_hash}` in the decision row's transaction; counters `authz:shadow:{bundle_id}:{evaluated,differing}` in Valkey
and `akr_authz_shadow_diff_total`. The shadow result never changes the answer.

**`AccessMatrixBuilder::build(at)`** (SP-20; `GET /admin/access-matrix`; the stub the phase 25 review snapshots):
1. Roles: for each of the nineteen codes, a synthetic principal holding only that role, all branches, vetted, a
   session at AAL2 with a fresh step-up from the analyst network; evaluate every action against a synthetic resource
   of its type that satisfies every non-role condition. Result: `{role, actions: [action]}`; also the `sod.json`
   rules.
2. Principals: every user with kind, status, branch, vetting, last sign-in, active token count, and grants with
   scope, validity and source, plus active delegations.
3. `{generated_at, bundle_version, bundle_sha256, roles, sod, principals}` (`sod` is the `sod.json` rules of the
   loaded bundle, `LoadedBundle.sod`; `build` adds it, `roles` of 7.16 does not); `?format=csv` gives the principal table. Audit
   `access_matrix.generated {bundle_version, principals}` (the read itself is decided and logged as usual).

### 3.6 Existence masking and lists

**`MaskingLayer`** (DR-48, SP-27):
1. One function builds the 404: `AppError::NotFound` -> `{type, title: "Not Found", status: 404, code: "E_NOT_FOUND",
   correlation_id}`, the same headers (`Content-Type: application/problem+json`, `Cache-Control: no-store`) for a
   missing record and a masked refusal; no `detail`, no `instance`.
2. On a masked kind both paths do the same work before answering: one adapter read, one evaluation (a missing record
   is evaluated against a placeholder entity no permit matches) and one decision row.
3. Latency floor: a 404 on a masked kind is released no earlier than `authz.mask_floor_ms` after the request began
   (the remainder is slept), so the access-event write of a refusal cannot be timed against a miss.
4. Under the step-up and purpose rules, a request for a masked kind without a purpose or a fresh step-up is refused
   with the 404 too, never with `E_PURPOSE_REQUIRED` or `E_STEPUP_REQUIRED`, because those answers would confirm the
   record exists; the access event records the missing purpose. (Allowed callers learn the requirement from the
   record's tier in the list that led them there.)

**Lists:** a list route is decided once on its collection action (resource `System`); rows of a masked kind are then
filtered in SQL by a visibility predicate that is the policy's equivalent (branch scope, tier, access list), and a
test per masked kind pins the predicate to the policy over generated principals and rows (the DBS
`sql_equivalence` practice). Counts, cursors and totals come from the filtered query only; a list that can contain
masked rows returns no total. Phase 03's lists contain no masked kind.

**Inference harness** (`api/tests/inference/`, SP-33): a registry `MASKED_KINDS: &[MaskedKindProbes]` in which every
masked kind declares fixtures and probes for get, list count, search, export, error body, notification text, SSE
event and the audit viewer, plus the latency comparison (median of 200 masked refusals versus 200 misses differ by
less than 5 ms). A masked kind without a complete probe set fails the build. In this phase the registry holds the
development kind `_dev_masked` (section 3.11), which proves the machinery.

### 3.7 Users (`UserAdminService`, A1)

`list(filter, cursor)` and `detail(id)`: humans and service principals (the system principal is never listed); the
detail's token register and factor count come from the phase 01 repositories and the Kratos admin read; no
credential content, no session ids of other users.

`create(ctx, CreateUser, assertion)`:
1. Validate: e-mail normalised (trim, NFKC, lower case), unique among users -> else 422 `/email` (`taken`);
   `display_name` both languages, 1-120 characters each; `branch_id` an existing, unarchived branch; `expires_at` in
   the future when present; `vetting_status` in the set (default `none`).
2. Generate a temporary password that passes `PasswordPolicy` (phase 01).
3. Kratos `create_identity({email, display_name, locale}, temp)` -> the identity id.
4. Transaction: `StepUpService::consume` (`user.create`, `identity_admin`); insert the mirror row (id = the Kratos
   id, kind human, `forced_change = first_login`, `temp_password_expires_at = now() + auth.temp_password_ttl_h`);
   audit `user.created {branch_id, vetting_status, expires_at, stepup_id}` with the decision record. Any failure ->
   `delete_identity` (compensation) and the error.
5. 201 `UserCreated` with `Cache-Control: no-store`. The new person has no role: a grant is a change request (phase
   04). The administrator hands the temporary password over in person.

`update(ctx, id, UpdateUser, If-Match)`: target human and not the caller (Cedar); version check (409 / 428); e-mail
change -> uniqueness and Kratos `update_traits` before the commit (a Kratos failure aborts); branch must exist and
not be archived; then update, `invalidate_user`, `invalidate_resource(user)`; audit `user.updated {changed: [field
names], branch_id: {before, after}, vetting_status: {before, after}, expires_at: {before, after}, name_hash,
email_hash}` (names and addresses only as keyed hashes, phase 02 rule).

`revoke_grant(ctx, id, grant_id)`: the grant belongs to the user and is active, else 404; revoking the last active ADM
or SEC grant -> 409 `E_NOT_ALLOWED_IN_STATE` (`last_holder`). Transaction: `revoked_at`, `revoked_by`; when the grant
was `L` and the user is liaison or alternate of a branch the remaining grants no longer cover, clear that assignment
and audit `branch.updated {liaison: cleared}`; audit `role_grant.revoked {role, grant_id}`; then `invalidate_user`.
Delegated copies of the role disappear by themselves (the phase 01 effective-roles query requires the delegator to
hold the role).

`unlock(ctx, id)`: `locked_until > now()` -> clear the counter and the lock; else 409 `E_NOT_ALLOWED_IN_STATE`
(`not_locked`); audit `user.unlocked {source: admin}`.

`disable(ctx, id, SuspendRequest)`: active human or service principal, not the caller, not the last ADM or SEC
holder; transaction: status `disabled`, reason `admin`; revoke the user's tokens (`admin`); audit `user.disabled
{reason: admin, reason_code}`; after commit `invalidate_user`, Kratos `revoke_all_sessions` and `set_state(inactive)`
(a Kratos failure enqueues `offboard_finalize`, section 4; the mirror already refuses every session at its next
request). Grants stay.

`enable(ctx, id, EnableRequest, assertion)`: status `disabled` with reason `dormant`, `expired` or `admin`; `offboarded`
-> 409 (`offboarded`: a returning leaver is a new account). Reason `expired` (or a past `expires_at`) needs a new
future `expires_at` -> else 422 `/expires_at`. Transaction: `consume` (`user.enable`, `access_change`); status
`active`, reason null, `failed_logins = 0`, `locked_until` null, `enabled_at = now()`; audit `user.enabled
{previous_reason, expires_at, stepup_id}`. After commit: Kratos `set_state(active)`, `invalidate_user`. The dormancy
rule of `identity_sweep` now reads `greatest(last_authenticated_at, enabled_at, created_at)`, so a re-enabled account
gets a full `auth.dormancy_days` to sign in.

### 3.8 Credential reset (four-eyes) and offboarding

**`CredentialResetService`** (DR-30 recovery, DR-49 four-eyes; replaces "self-service recovery", which does not exist):

`request(ctx, user_id, ResetRequest, assertion)`:
1. Target: an active human who is not the caller (Cedar `useradmin.no_self`); a pending reset for the user -> 409
   (`reset_pending`).
2. Transaction: `consume` (`user.reset_credentials`, `identity_admin`, resource `user:<id>`); insert with `expires_at
   = now() + identity.credential_reset_ttl_h`; audit `credential_reset.requested {reset_id, reason_code,
   remove_second_factors, stepup_id}`.
3. 201 `CredentialResetView`. Security officers see it in `GET /admin/credential-resets` (and, from phase 22, a
   notification with generic text).

`confirm(ctx, reset_id, assertion)`:
1. The row is pending and unexpired, else 404 (expired or completed rows answer 409 `E_NOT_ALLOWED_IN_STATE`). Cedar
   has already refused the initiator and the subject (`reset.four_eyes`, 403 `E_SELF_APPROVAL`); the CHECK is the
   second layer.
2. Transaction: `consume` (`credential_reset.confirm`, `identity_admin`, resource `credential_reset:<id>`); generate
   a temporary password; Kratos `set_password(user, temp)`; when `remove_second_factors`, Kratos
   `remove_second_factors(user)`; `revoke_all_sessions(user)`; mirror: `forced_change = recovery`,
   `temp_password_expires_at`, `failed_logins = 0`, `locked_until` null; tokens revoked (`admin`) when the reason is
   `compromise_suspected` or `lost_device`; `complete` (zero rows -> another confirmer won the race -> roll back and 409);
   audit `credential_reset.confirmed {reset_id, factors_removed, sessions_revoked, tokens_revoked, stepup_id}`. A
   Kratos failure rolls the transaction back, so the assertion is not spent and the reset stays pending.
3. Commit; `invalidate_user`; 200 `ResetCompleted` with `Cache-Control: no-store`. The security officer hands the
   password over in person; at sign-in the person goes through the phase 01 first-login steps (new password, a
   second factor when removed, the undertaking if pending). The temporary password is not pushed into the password
   history (it is not a password the user set).
4. If the commit fails after Kratos succeeded, the password already works but the row stays pending and the person
   cannot tell the temporary password: a second confirmation with a new step-up generates a new one, which is safe.

Expiry: `identity_sweep` (phase 01) gains a step: pending resets past `expires_at` -> `expired`, audit
`credential_reset.expired` with the system principal.

**`OffboardingService::offboard(ctx, id, OffboardRequest, assertion)`** (one action, ARCHITECTURE s.13):
1. Target: a human or service principal, not the caller, not already offboarded (409 `offboarded`), not the last ADM
   or SEC holder (409 `last_holder`).
2. One transaction: `consume` (`user.offboard`, `identity_admin`); status `disabled`, reason `offboarded`; then every
   registered `OffboardingStep` in order, each returning a count:
   - `grants` (this phase): `revoke_all_for_user`;
   - `delegations` (this phase): end every delegation from or to the user;
   - `tokens` (this phase): revoke with `offboarded`;
   - `owned_services` (this phase): disable every service principal the user owns (`admin`), revoking their Hydra
     clients after commit, and list them in the report so the owner can be replaced by a new client;
   - `liaison` (this phase): clear the user as liaison or alternate; audit `branch.updated` per branch;
   - `credential_resets` (this phase): expire a pending reset for the user;
   - later phases register theirs: open tasks swept to the pool or the role queue (phase 04), case access-list
     removal (phase 18, research file 30 s.8.2 item 14), watch ownership (19), PAT-like gateway credentials (29).
   Audit `user.offboarded {reason_code, steps: {name: count}, stepup_id}`.
3. After commit: Kratos `revoke_all_sessions` and `set_state(inactive)`, Hydra client revocation for disabled
   service principals, `invalidate_user`; a failure enqueues `offboard_finalize` (retried until done) and the report
   says `kratos: deferred`. The mirror refuses the person at once either way.
4. 200 `OffboardReport`. Offboarding is final; history (grants, audit, actor columns) stays.

`trait OffboardingStep: Send + Sync { fn name(&self) -> &'static str; fn order(&self) -> u16; async fn run(&self, tx:
&mut Transaction<'_, Postgres>, user_id: Uuid, actor: Uuid) -> Result<u64, DomainError>; }`; the registry is built in
`AppState`, and a test asserts the phase 03 steps are present.

### 3.9 Organisation

**`BranchService`**:
- `create(ctx, CreateBranch)`: code unique ignoring case -> else 422 `/code` (`taken`); governorate pattern; audit
  `branch.created {branch_id, code}`.
- `update(ctx, id, UpdateBranch, If-Match)`: liaison or alternate must be an active human holding an active `L` grant
  whose scope is all branches or lists this branch -> else 422 (`/liaison_user_id`, `not_liaison_for_branch`); the
  alternate needs a liaison (else 422 at `/alternate_user_id`, `liaison_required`); `archived = true` -> refused while users, unarchived pools or active grant scopes refer to
  the branch (409 `E_NOT_ALLOWED_IN_STATE`, sub-code `in_use`, with the three counts), and clears liaison and alternate; audit `branch.updated {changed, liaison:
  {before, after}, alternate: {before, after}}`; `invalidate_resource(branch)`.
- `akr-ctl org import-branches <csv> --dry-run|--apply` (columns `code,name_ar,name_en,governorate,region`): the
  initial load from the bank's branch list or the discovery output (phase 05); existing codes are reported, never
  overwritten; one `branch.created` per row with source `cli`.
- The CBS branch merged or closed (M2 edge case): from phase 10 the drift detector raises it, and the administrator
  archives or renames here; history is kept.

**`PoolAdminService`**: `create` and `update` validate `branch_ids` against `admin.branches` (unknown or archived ->
422 `/branch_ids/<n>`); archiving is refused while active grants list the pool (409 with the count); audit
`pool.created`, `pool.updated {changed}`. Routing attributes (skills, tier, WIP limits) belong to the queues of phase
17.

**`DelegationService`** (M2 "delegations with windows"; SP-37 never-delegated roles):
- `create(ctx, CreateDelegation, assertion)`:
  1. Caller: a human on a session; `roles` non-empty, a subset of the caller's **direct** active grants (a delegated
     role cannot be passed on), none of ADM, SEC, IA, AS, VS (the phase 01 CHECK is the second layer).
  2. Window: `valid_from >= now() - 5 minutes`, `valid_to > valid_from`, length at most `delegation.max_days`.
  3. Delegate: an active human, not the caller; the delegate's current roles plus `roles` pass
     `SodRules::check_roles` -> else 422 `/to_user` (`sod_conflict`); `DelegationAuthority::check` (the authority
     matrix from phase 09; `NoMatrix` passes).
  4. Transaction: `consume` (`delegation.create`, `access_change`); insert; audit `delegation.created {delegation_id,
     to_user, roles, valid_from, valid_to, reason_code, stepup_id}`.
  5. After commit: `invalidate_user(to_user)` (the role appears at `valid_from`: the effective-roles query is
     time-bound, and the session cache TTL bounds the delay).
- `end(ctx, id)`: the delegator, the delegate, or an `admin.users` holder (Cedar `delegation.end`); already ended ->
  409; audit `delegation.ended {by: from | to | admin}`; `invalidate_user(to_user)`.
- `list(ctx, filter)`: own (to and from) by default; `all=true` needs action `delegation.list_all` (ADM).

### 3.10 Settings, flags, jobs and the ops cockpit (A2, A3)

- `SettingsAdmin::list` returns every registry key with value, floor and basis, registry bounds, governed mark and
  label; values are never secrets (phase 00 test). `update(ctx, key, value, If-Match)`: unknown key -> 404; governed
  -> 409 `E_NOT_ALLOWED_IN_STATE` (`governed`) until phase 04 replaces this branch with a change request (202); else
  `SettingsService::apply(Direct)` (type, bounds 422, floor `E_BELOW_FLOOR`, version, `setting.updated`).
- `FlagsAdmin::update(ctx, key, UpdateFlag, If-Match)`: scope branches must exist, roles must be known codes;
  `FlagsService::update` (`flag.updated`). Flags never gate security (phase 00 rule).
- `JobsAdmin::list`: filters `kind`, `status`, `since`; keyset on `(created_at DESC, id DESC)`; `JobView` without the
  payload (payloads carry ids and codes only, but an id of a case is still not an administrator's business).
  `retry(ctx, id)`: only a `dead` job -> else 409; `retry` resets attempts and queues it now; audit `job.retried {kind,
  attempts_before}`; a kind unknown to every binary -> 409 (`unknown_kind`, a release error shown on the cockpit).
- **`OpsCockpit::snapshot()`** (`GET /admin/ops`; TS-10, TS-28), every tile with a `state` (ok, warn, critical,
  unknown) and the threshold that set it:

| Tile | Source | Warn / critical |
|-|-|-|
| Jobs per kind: queued, running, dead, oldest queued age | `admin.jobs` (live) | oldest queued over `ops.queue_age_warn_s`; any dead |
| Unknown job kinds | `admin.jobs` against the registry | any (critical) |
| Outbox: pending, stuck | `admin.outbox` | stuck > 0 |
| Timers due and late | `admin.timers` | late over 5 minutes |
| Backups: age, restore-drill age | phase 00 metrics via the probe | over `ops.backup_max_age_h`; drill over `ops.restore_drill_max_age_days` |
| Audit: last anchor and lag per stream, unanchored rows, partitions ahead, last verify result | `audit` tables (phase 02) | lag over `audit.anchor_lag_alert_s`; verify failure critical |
| Evidence: pending scans, oldest pending, signature age | `admin.evidence_objects`, probe | over `evidence.av_max_signature_age_h` |
| Disk per mount | probe (node exporter) | `ops.disk_warn_pct`, `ops.disk_critical_pct` |
| Certificates: name, not-after, days left | probe (blackbox exporter, textfile collector) | under `ops.cert_warn_days`; expired critical |
| OpenBao: sealed, version | probe | sealed critical |
| Valkey | `PING` from the API | down warn (the product runs on, slower) |
| Database replication lag | probe (postgres exporter) | over 60 s warn, 300 s critical |
| Authorisation bundle: active version, instances and their version, load failures, SoD conflicts | Valkey `authz:instances`, metrics, query | mismatch warn; load failure critical |

  The probe snapshot older than `ops.probe_stale_s` turns its tiles `unknown`, never `ok`.
- **`ops_probe`** (job, every minute, worker; DR-51): OpenBao `sys/seal-status` and `sys/health`; Prometheus queries
  for `node_filesystem_avail_bytes` and size per mount, `pg_replication_lag_seconds`,
  `probe_ssl_earliest_cert_expiry` per target, `akr_backup_age_seconds`, `akr_restore_drill_age_days`, the ClamAV
  signature age; writes `ops:probe` (JSON with `taken_at`, TTL 10 minutes) to Valkey. Values are numbers and names of
  mounts and certificate targets only.

### 3.11 Purpose codes, the access layer and the retrofit

- `GET /access/purpose-codes` returns the active codes in sort order. `AccessScope` (phase 02) now checks a stated
  purpose against the list and its tiers (a code not listed for the record's tier -> 422 `E_PURPOSE_REQUIRED`; a code
  with `reason_required` and no `X-Access-Reason` -> the same), and takes `actor_role` and `on_behalf_of` from
  `Decided` instead of the phase 02 fallback.
- The policy layer writes the `deny` and `conflict` access events (section 3.2 step 8).
- Retrofit: every phase 01 route gets its pinned action (`session.read`, `session.revoke`, `profile.read`,
  `preferences.update`, `policy.acknowledge`, `credential.list`, `credential.enroll`, `credential.change_password`,
  `credential.remove`, `token.list`, `token.create`, `token.revoke`, `stepup.start`, `stepup.verify`) with resource
  `User(self)` and the `10_self.cedar` permits: every active human for all of them; a service principal for
  `profile.read` only. The phase 01 confinement (pending steps) still runs first, in the auth layer. The phase 02
  actions `audit.read_events`, `audit.chain_status`, `audit.verify` and `evidence.read` are declared in the schema
  with no permit yet (their routes are phases 24 and 18).
- Development kind `_dev_masked` (mounted only when `AKR_ENV != production`, like `/_echo`): `GET
  /api/v1/_dev/masked/{id}` over an in-memory fixture of two records (one `normal` tier, one `suspicion` tier with an
  access list of the seeded SEC user); masked, so a refused read is the 404 of section 3.6. It exists for the curl
  suite and the inference harness, and is not in the production OpenAPI document.

### 3.12 Errors, audit events, Cedar actions

**Settings registry additions** (DR-34 bounds, 422 `E_VALIDATION` outside them): `authz.reload_poll_s` 5-300;
`authz.resource_cache_ttl_s` 5-300; `authz.shadow_sample_pct` 1-100; `authz.mask_floor_ms` 10-500;
`authz.analyst_networks` a JSON array of valid CIDRs; `identity.credential_reset_ttl_h` 1-72; `delegation.max_days`
1-90; `ops.probe_stale_s` 60-3600; `ops.cert_warn_days` 1-180; `ops.disk_warn_pct` and `ops.disk_critical_pct`
50-99 with warn below critical; `ops.queue_age_warn_s` 60-86400.

**Error mapping.** No new codes. Used here: `E_SCOPE` (gate 1), `E_FORBIDDEN` (Cedar deny on a resource the caller may
know exists), `E_SELF_APPROVAL` (the reset four-eyes), `E_NOT_FOUND` (missing, masked, or a reset that is not
pending), `E_NOT_ALLOWED_IN_STATE` (`last_holder`, `not_locked`, `offboarded`, `reset_pending`, `governed`,
`not_dead`, `unknown_kind`, `in_use`), `E_STEPUP_REQUIRED`, `E_VALIDATION` (`taken`, `sod_conflict`,
`not_liaison_for_branch`, `liaison_required`, `masked_kind`, bounds), `E_PURPOSE_REQUIRED`, `E_VERSION_CONFLICT`,
`E_PRECONDITION_REQUIRED`, `E_BELOW_FLOOR`, `E_UNAVAILABLE` (no bundle loaded, decision row not written). `AuthzError`
(`BundleInvalid(Vec<BundleDiagnostic>)`, `NoActiveBundle`, `AttributeSource`, `AuditWrite`) maps to 503 except
`BundleInvalid`, which only `akr-ctl` sees.

**Audit events emitted** (catalogue additions, phase 02 `EventKind` with detail keys as listed):

| Event kind | On | Resource | SIEM |
|-|-|-|-|
| `authz.allowed` | every allowed evaluated request (action, gate, bundle version) | the request's resource | no |
| `authz.denied` | every refusal (gate, deny reason, bundle version) | the request's resource | yes |
| `authz.simulated` | simulator run (action, resource kind, principal or "synthetic", bundle, effect) | policy_bundle | yes |
| `authz.shadow_diff` | shadow bundle disagrees (both effects, shadow version) | the request's resource | no |
| `policy_bundle.installed` | `akr-ctl policy install` (version, hash, release id) | policy_bundle | yes |
| `policy_bundle.activated` | bootstrap activation (phase 04 adds change-request activation) | policy_bundle | yes |
| `policy_bundle.shadow_changed` | `akr-ctl policy shadow` | policy_bundle | yes |
| `policy_bundle.load_failed` | an instance refuses a bundle (code) | policy_bundle | yes |
| `branch.created` | branch created (API or CLI) | branch | no |
| `branch.updated` | branch changed, liaison cleared by a revocation or offboarding | branch | no |
| `pool.created`, `pool.updated` | pool created or changed | pool | no |
| `user.created` | account created (branch, vetting, expiry, step-up) | user | yes |
| `user.updated` | profile change (field names, branch, vetting, expiry, hashed name and e-mail) | user | yes |
| `user.unlocked` | (phase 01 kind) now also `source: admin` | user | yes |
| `user.disabled` | (phase 01 kind) now also `reason: admin` with the suspension code | user | yes |
| `user.enabled` | re-enabled (previous reason, expiry, step-up) | user | yes |
| `user.offboarded` | offboarding (reason code, step counts, step-up) | user | yes |
| `role_grant.revoked` | grant revoked at once | role_grant | yes |
| `credential_reset.requested` | reset requested (reason, factor removal, step-up) | credential_reset | yes |
| `credential_reset.confirmed` | reset confirmed (factors removed, sessions and tokens revoked, step-up) | credential_reset | yes |
| `credential_reset.expired` | pending reset expired (system) | credential_reset | yes |
| `delegation.created`, `delegation.ended` | delegation made or ended (by whom) | delegation | yes |
| `job.retried` | dead job re-queued (kind, attempts before) | job | no |
| `access_matrix.generated` | access matrix built (bundle version, principal count) | policy_bundle | yes |

`setting.updated` and `flag.updated` (phase 00) and `service_client.disabled` (phase 01, now also from offboarding)
are reused. Every mutation event carries the decision record of its request.

**Cedar actions touched** (new; principal `User`, context `Ctx`; the group in brackets):

| Action | Resource | Who (shipped bundle) |
|-|-|-|
| `branch.list`, `pool.list`, `purpose_code.list` [self] | System | every active principal |
| `branch.create`, `branch.update`, `pool.create`, `pool.update` [org_admin] | System, Branch, Pool | ADM |
| `delegation.list`, `delegation.create`, `delegation.end` [self] | System, Delegation | active humans; `end` when principal is `from_user` or `to_user`, or ADM |
| `delegation.list_all` [org_admin] | System | ADM |
| `user.list`, `user.create` [user_admin] | System | ADM |
| `user.read`, `user.update`, `user.revoke_grant`, `user.unlock`, `user.disable`, `user.enable`, `user.offboard`, `user.reset_credentials` [user_admin] | User | ADM; never on self (`user.read` excepted) |
| `credential_reset.list`, `credential_reset.confirm` | System, CredentialReset | SEC; confirm never by the initiator or subject |
| `policy.read`, `policy.simulate` [policy_admin] | System | ADM, SEC |
| `access_matrix.read` | System | SEC, IA |
| `setting.list`, `setting.update`, `flag.list`, `flag.update`, `job.list`, `job.retry`, `ops.read` [system_admin] | System, Setting, Flag, Job | ADM |
| the 14 phase 01 actions [self] | User (self) | active humans; `profile.read` also service principals |
| `audit.read_events`, `audit.chain_status`, `audit.verify`, `evidence.read` | declared only | permits arrive with their routes (24, 18) |

Attributes evaluated: `principal.status`, `roles`, `delegated_roles`, `kind`, `sod_conflict`, `branches` and
`all_branches` (branch scope of later business actions), `vetting`; `resource.kind`, `subject`, `initiated_by`,
`from_user`, `to_user`, `governed`; `context.via`, `aal`, `stepup_age_s`, `network`, `purpose`.

**Tipping-off review.** This phase adds administration screens' data, no case data:
- User administration shows staff accounts, grants, delegations (reason codes, never free text), token registers and
  reset records. Nothing in them names a customer, an alert or a case.
- The jobs view leaves out payloads; the cockpit shows counts, ages, names of mounts and certificate targets, never
  ids or names of business records. A job kind name (for example a freeze clock) reveals that such jobs exist, not
  for whom.
- The simulator refuses real ids of masked kinds, so it cannot answer "is this customer under suspicion" or "who is
  on this case's list"; its runs are audited for the security officer.
- The access matrix lists roles, actions and grants; it contains no record and no access list.
- Masking: one 404 body and header set for missing and refused, the same work on both paths, a latency floor, and no
  purpose or step-up prompt on a masked kind before access is granted; the harness proves it per kind from phase 15
  on.
- Audit details carry ids, codes, counts and hashes only; staff names and e-mail addresses only as keyed hashes.
The answer for a person outside a case's access list is: nothing.

**Settings and configuration used:** the 13 keys of migration 0013; phase 01 keys (`auth.temp_password_ttl_h`,
`auth.dormancy_days`, `stepup.fresh_window_s`); phase 02 keys for the cockpit thresholds; phase 00 `ops.*` keys.
Environment additions: `PROMETHEUS_URL` (worker only, refused in production when not `https` or not on `z5_mgmt`).

## 4. Infrastructure Updates

- **Job kinds registered:** `ops_probe` (worker; every minute, `schedule.ops_probe`; concurrency 1; section 3.10);
  `offboard_finalize` (worker; enqueued when the Kratos or Hydra step after an offboarding, suspension or reset fails;
  payload `{user_id, actions}`; retried with the phase 00 back-off; max attempts 20; it re-reads the mirror and does
  nothing when the user is active again). `identity_sweep` (phase 01) gains two steps: expire pending credential
  resets, and the consistency check (pool branch lists and grant branch scopes naming an archived or unknown branch ->
  an error log and a cockpit count, never a silent fix); its dormancy rule reads `enabled_at`. **Outbox topics:**
  none. **Timers:** none. **Notifications:** `NOTIFY akr_policy` (bundle activation) is a PostgreSQL notification,
  not a durable message; the poll is the guarantee.
- **Compose** (both bundles):

| Service | Change |
|-|-|
| `node-exporter` (new) | Pinned; host file systems read-only; network `z5_mgmt`; scraped by Prometheus |
| `postgres-exporter` (new) | Pinned; networks `z2_app` (to PostgreSQL) and `z5_mgmt`; connects as `akr_monitor` |
| `blackbox-exporter` (new) | Pinned; `z5_mgmt` plus the networks of its TLS targets (`z3_edge` for Caddy, `z1_bao` for OpenBao, `z2_store` for the guard); module `tls_connect` only |
| `prometheus` | Scrape jobs for the three exporters; `deploy/observability/tls-targets.yaml` lists the blackbox targets; the PostgreSQL server certificate expiry comes from the node exporter's textfile collector, written by the host timer `akr-cert-check` (PostgreSQL negotiates TLS inside its own protocol, which the blackbox prober cannot do) |
| `akr-worker` | Gains `z5_mgmt` reach to `prometheus:9090` (read API) |

- **Provisioning:** `akr-ctl db provision` creates the LOGIN role `akr_monitor` (password in OpenBao `db/akr_monitor`),
  member of `pg_monitor` only, with no privilege on any AKR schema (DR-22: role facts are provisioning).
- **OpenBao:** KV `db/akr_monitor` read by the `postgres-exporter` AppRole `akr-monitor` (rendered by an agent like
  `ory-secrets`); the `akr-worker` policy gains read on `sys/seal-status` and `sys/health` (both unauthenticated in
  OpenBao, listed for clarity). No transit change.
- **Zone rules** (`deploy/zones.yaml`): `akr-worker` -> `prometheus:9090`; `prometheus` -> the three exporters;
  `blackbox-exporter` -> `caddy:443`, `openbao:8200`, `s3guard:9000`; `postgres-exporter` -> `postgres:5432`. No new
  edge for `akr-api` (it reads the probe snapshot from Valkey); `akr-ingestd` and `akr-assess` unchanged.
- **AppState additions** (`api`): `authz: Arc<AuthzService>` (replacing `AllowAllAuthorizer`; the trait object stays
  `Arc<dyn Authorizer>` for the middleware), `bundles: Arc<dyn PolicyBundleRepository>`, `simulator: Arc<Simulator>`,
  `access_matrix: Arc<AccessMatrixBuilder>`, `user_admin: Arc<UserAdminService>`, `resets:
  Arc<CredentialResetService>`, `offboarding: Arc<OffboardingService>` (with its step registry), `branches:
  Arc<BranchService>`, `pools: Arc<PoolAdminService>`, `delegations: Arc<DelegationService>`, `purpose_codes:
  Arc<dyn PurposeCodeRepository>`, `jobs_admin: Arc<JobsAdmin>`, `ops: Arc<OpsCockpit>`. `worker` registers
  `ops_probe` and `offboard_finalize` and holds a `MetricsQuery` client.
- **OpenAPI registration:** tags `org`, `admin-users`, `authz`, `admin-system`; 32 operations; every operation (the
  phase 01 ones included) carries `x-akr-action: <action>` and its scopes in the description; the five SU operations
  reference the step-up scheme and declare `X-StepUp-Assertion`; the three exempt operations state that they take no
  `Idempotency-Key`; `_dev/masked` is excluded from the production document.
- **Readiness:** `akr-api` adds `authz_bundle` (a bundle loaded and its hash verified). `akr-ctl db check` reports a
  missing active bundle.
- **Metrics:** `akr_authz_decisions_total{effect, gate}`, `akr_authz_eval_seconds` (histogram),
  `akr_authz_bundle_version` (gauge per instance), `akr_authz_bundle_load_failures_total`,
  `akr_authz_shadow_diff_total`, `akr_authz_cache_hits_total{kind}`, `akr_authz_sod_conflicts` (gauge),
  `akr_ops_probe_age_seconds`. Alert rules: bundle version mismatch between instances for 5 minutes, any load failure,
  decision-row write failures, probe stale.
- **Existing entity, schema or route updates:**
  - `AllowAllAuthorizer` deleted; the phase 00 middleware slot now holds the authz layer.
  - Every phase 01 route gains its action; the 22-route coverage test of phase 01 now also checks `x-akr-action`.
  - `AccessScope` (phase 02) reads `Decided` and the purpose list; `audit_log.decision` is filled from here on.
  - `admin.users` gains the branch FK and `enabled_at`; `identity_sweep` reads it.
  - The idempotency exemption list gains three routes.
  - API.md 3.2 and 3.3 and section 2 are aligned in the same change: `GET /admin/users/{id}`, enable, disable and the
    two credential-reset routes, the access-matrix route and `GET /access/purpose-codes` are added; `POST
    /admin/users` and `POST /org/delegations` are marked **SU**; the reset row names its confirm route.
  - DATA-MODEL s.1 (`policy_bundles` columns, `purpose_codes`, `credential_resets`, `users.enabled_at`) and s.21
    (grants) are aligned; roadmap row 03 size becomes XL.

## 5. Frontend Impact

Backend phase for screens. `make generate-api` regenerates the client with the hooks `useGetOrgBranches`,
`usePostOrgBranches`, `usePatchOrgBranchesId`, `useGetOrgPools`, `usePostOrgPools`, `usePatchOrgPoolsId`,
`useGetOrgDelegations`, `usePostOrgDelegations`, `useDeleteOrgDelegationsId`, `useGetAdminUsers`,
`useGetAdminUsersId`, `usePostAdminUsers`, `usePatchAdminUsersId`, `useDeleteAdminUsersIdGrantsGrantId`,
`usePostAdminUsersIdUnlock`, `usePostAdminUsersIdDisable`, `usePostAdminUsersIdEnable`, `usePostAdminUsersIdOffboard`,
`usePostAdminUsersIdResetCredentials`, `useGetAdminCredentialResets`, `usePostAdminCredentialResetsIdConfirm`,
`useGetAdminPolicies`, `usePostAdminPoliciesSimulate`, `useGetAdminAccessMatrix`, `useGetAccessPurposeCodes`,
`useGetAdminSettings`, `usePutAdminSettingsKey`, `useGetAdminFlags`, `usePutAdminFlagsKey`, `useGetAdminJobs`,
`usePostAdminJobsIdRetry`, `useGetAdminOps`.

Phase 08 builds on them: A1 users, roles and leave (list with filters; detail with grants, delegations, token
register, the lock and reset state; create with the one-time password panel; suspend, re-enable, offboard and reset
with the `StepUpDialog`; the security officer's pending-reset list), A2 settings (floors and their basis shown;
governed keys read-only until phase 04 adds "propose a change"; flags), A3 platform health (the cockpit tiles with
their states; jobs with retry), and the X6 not-found page that a masked refusal lands on (it never says "no access" for
a masked kind, IR-70). The purpose dialog of later sensitive screens reads `GET /access/purpose-codes`. The
`x-akr-action` extension feeds the phase 08 route-to-policy test (UX-53): a screen whose route maps to no action the
role can pass is not rendered in the rail.

## 6. Verification

1. `make migrate`: 0011-0013 apply; a second run is a no-op; the phase 00 schema checks pass for the four new tables
   and the new column; `\dp admin.policy_bundles` shows `akr_app` with UPDATE on the eight activation columns only;
   an `UPDATE admin.policy_bundles SET cedar_source = 'x'` as the owner fails in the guard trigger; an active ->
   staged transition fails; `INSERT` of a second active row fails on `uq_policy_bundles_active`; a credential reset
   with `confirmed_by = initiated_by` fails the CHECK.
2. `cargo check` with `cargo sqlx prepare`; `cargo clippy --all-targets -- -D warnings`; `cargo deny check`.
3. `akr-ctl policy validate backend/policies/bundle` -> 0 diagnostics and the hash; a copy with a syntax error, an
   unknown attribute, a permit without `@role`, an action missing from the schema, or a `sod.json` naming an unknown
   role -> refused with the matching diagnostic.
4. `cargo test`, including:
   - coverage (SP-25, SP-71): every operation in the OpenAPI document has a `RouteAction` and an `x-akr-action`, and
     every `RouteAction` has an operation; every action is in the schema and the reverse; a live-traffic check in the
     curl run finds no request without a decision row;
   - decision rows: one `authz.allowed` or `authz.denied` per evaluated request, with policy ids and `attrs_hash`;
     a failing sink -> 503 and the handler never runs; the same request twice -> the same `attrs_hash`;
   - gate order: a personal token with `audit.read` only, whose owner holds ADM, on `GET /admin/users` -> 403
     `E_SCOPE`, Cedar never consulted;
   - hot reload: activating a new version in a test database switches every instance within `authz.reload_poll_s`
     (notification suppressed) or at once (notification delivered); a bundle that fails to load leaves the previous
     version serving, records `policy_bundle.load_failed` once, and readiness stays true; an instance started with no
     active bundle answers 503 and is not ready;
   - SoD: the bundle test over every role finds no role permitted both actions of a `sod.json` pair; a principal with
     ADM and A1 grants (inserted directly) is refused every action (`baseline.sod_conflict`); a delegation that would
     give a SEC holder the role M -> 422 `sod_conflict`; the reset confirmer equal to the initiator -> 403
     `E_SELF_APPROVAL` and, bypassing Cedar, the CHECK fails;
   - self-administration: ADM on `PATCH /admin/users/<own id>` -> 403 `E_FORBIDDEN`; on `GET` of their own detail ->
     200;
   - last holder: revoking or offboarding or suspending the only ADM -> 409 `last_holder`;
   - masking (`_dev_masked`): a refused read and a random id give byte-identical bodies apart from `correlation_id`,
     identical headers, and medians within 5 ms over 200 tries each; the refusal writes a `deny` access event, the
     miss none; a masked read without a purpose -> the same 404, not 422;
   - simulator: a real id of `_dev_masked` -> 422 `masked_kind`; a synthetic one -> a trace; every run writes
     `authz.simulated`; a staged bundle shows `active_effect` beside its own;
   - shadow: with a shadow bundle that denies `branch.list`, requests still succeed and `authz.shadow_diff` rows and
     counters appear at the sampled rate;
   - access matrix: ADM's row holds the `user_admin` actions and no business action; SEC holds `credential_reset.confirm`
     and not `user.reset_credentials`;
   - offboarding: one call leaves no active grant, delegation, token or liaison assignment; owned service principals
     are disabled; with Kratos down the call still returns 200 with `kratos: deferred` and `offboard_finalize`
     completes it later; the person's next request is refused;
   - credential reset: two security officers confirming at once -> one 200, one 409; the temporary password signs in
     to the phase 01 first-login steps; an expired reset cannot be confirmed; factor removal leads to `enroll_mfa`;
   - delegation: a delegated role appears at `valid_from` and disappears at `valid_to` or when ended; a delegated role
     cannot be delegated again; ADM, SEC, IA, AS and VS are refused by the service and by the CHECK;
   - re-enable: a dormant user re-enabled is not disabled again by the next `identity_sweep`;
   - settings: a governed key -> 409 `governed`; a non-governed key outside its bounds -> 422; inside -> 200 and
     `setting.updated` with the decision record;
   - jobs: retry of a `done` job -> 409; of a `dead` job -> queued with `attempts = 0`; `JobView` has no payload field;
   - cockpit: with the probe key deleted the probe tiles read `unknown`.
5. Curl suite `tests/curl/phase03_authz.sh` (dev users from `akr-ctl identity seed-dev`: `adm@bank.test`,
   `sec@bank.test`, `a1@bank.test`, `ia@bank.test`; codes from `oathtool`):
   - Bundle: `akr-ctl policy install backend/policies/bundle --release dev-1` and `akr-ctl policy activate --bootstrap
     1`; a second `--bootstrap` -> refused. `GET /api/v1/admin/policies` as ADM -> 200 `"version":1,"status":"active"`.
   - Branches: `POST /api/v1/org/branches` with `Idempotency-Key: br-0001` and `{"code":"BGD01","name":{"ar":"...","en":
     "Baghdad Main"},"governorate":"IQ-BG"}` -> 201; the same key and body again -> byte-identical body with
     `x-idempotent-replay: true`; another body with that key -> 422 `E_IDEMPOTENCY_REUSE`; `PATCH` with `If-Match:
     "1"` setting `liaison_user_id` to `a1@bank.test`'s id (who holds no `L`) -> 422 `not_liaison_for_branch`.
   - Users: `POST /api/v1/admin/users` without a step-up -> 401 `E_STEPUP_REQUIRED`; with one (`action_class:
     identity_admin`) -> 201 with `temp_password`; the new person signs in -> `setup_required`.
   - Cedar deny: `GET /api/v1/admin/users` as `a1@bank.test` -> 403 `E_SCOPE`; ADM `PATCH /api/v1/admin/users/<own
     id>` -> 403 `{"code":"E_FORBIDDEN",...}`.
   - Masking: as ADM `GET /api/v1/_dev/masked/<suspicion id>` and `GET /api/v1/_dev/masked/<random uuid>` -> both 404
     with bodies equal after removing `correlation_id`; as SEC with `X-Access-Purpose: investigation` and a fresh
     step-up -> 200 with `Cache-Control: no-store`.
   - Reset four-eyes: ADM requests a reset for `a1@bank.test` -> 201; ADM confirms -> 403 (scope: ADM lacks
     `security.ops`); SEC confirms with a step-up -> 200 `temp_password`; the old password -> 401.
   - Offboarding: ADM offboards the user created above with a step-up -> 200 report; that user's session -> 401.
   - Settings: `PUT /api/v1/admin/settings/auth.password_min_length` -> 409 `governed`; `PUT
     /api/v1/admin/settings/ops.cert_warn_days` with `{"value": 45}` and `If-Match` -> 200.
   - Jobs and cockpit: a forced dead `ops_probe` job -> `POST /api/v1/admin/jobs/<id>/retry` -> 200; `GET
     /api/v1/admin/ops` -> tiles with states.
   - Access matrix: as IA `GET /api/v1/admin/access-matrix` -> 200; as ADM -> 403 `E_SCOPE`.
6. Swagger UI lists 32 new operations (54 with phase 01), each with `x-akr-action`; `make generate-api` twice gives
   zero diff.
7. Audit: after the curl run `audit_log` holds `authz.allowed` and `authz.denied` rows for every request,
   `policy_bundle.installed`, `policy_bundle.activated`, `branch.created`, `user.created`,
   `credential_reset.requested`, `credential_reset.confirmed`, `user.offboarded`, `setting.updated`, `job.retried`,
   `access_matrix.generated`, and the mutation events carry `decision`; `access_event` holds the masked refusal as
   `deny` and the SEC read as `allow` with purpose `investigation`; `akr-ctl audit verify` passes.
8. No CBS connection in this phase: the read-only conformance suite is not affected.
9. Phase 00, 01 and 02 tests still pass, with the phase 01 routes now decided by Cedar.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 2, from tools/agent/gap1-run2.md; lead-verified (tools/agent/gap1-run2-verdict.md).

### 7.1 Masked kinds register by phase
The registry `MASKED_KINDS` is open: a phase registers each masked kind it introduces, with its complete probe set
(section 3.6), in the phase that first serves the record. Registered so far:

| Masked kind | Phase |
|-|-|
| `_dev_masked` (development only) | 03 |
| `task_restricted` | 04 |
| `screening_alert`, `pair_memory` | 14 |
| `customer`, `kyc_review`, `override`, `pep_entry` | 15 |
| `replay_sample` | 16 |
| `alert`, `case` | 17 |
| `rfi` | 19 |
| `branch_item`, `liaison_report` (phase 22 names them by their Cedar entities `BranchItem`, `LiaisonReport`) | 22 |
| `audit_event`, `audit_target` | 24 |

Section 2's list "(customer, alert, case, report, evidence, RFI, freeze)" and the header's "the tiered resources
arrive from phase 15" read "the masked kinds of this table": reports, evidence and freezes are masked through the
case rule, not as kinds of their own (phase 20). Phase 14's deviation note (Open doubts) is closed by this rule.

### 7.2 Purpose code for report filing (DR-362)
Migration 0011's purpose-code seed gains one row (phase 21 Open doubts 13):

    ('report_filing', '{"ar": "تقديم بلاغ", "en": "Report filing"}', '{restricted,suspicion}', FALSE, 45)

It is the purpose of the report package download (phase 21 section 3.9). Other reporting reads use the existing
codes (`report_preparation`, `qa_review`, `internal_audit`, `authority_request`).

### 7.3 Cockpit tiles and probe inputs added by later phases
The tile table of section 3.10 is extended by the tiles each phase registers through `OpsCockpit::snapshot()`, with
the thresholds and states that phase gives: parked tasks with their ages (phase 04 section 3.6); Sources (phase 05
section 3.10); the goAML line (phase 07 section 3.8); the three reference-data tiles (phase 09 section 3.8); the
tiles of phase 25 section 3.10; the eight tiles of phase 30 section 3.13 (the replication lag tile covers the DR
replica). `ops_probe` also computes the inputs and the outage detection of phase 30 section 3.8. A tile whose probe
snapshot is stale shows unknown (existing rule).

### 7.4 Flag guards and change hooks
`FlagsService::update` consults two registries, both empty in this phase:
- `FlagGuard` (one per key, registered by the phase that owns the flag): runs before the write; a refusal is 409
  `E_NOT_ALLOWED_IN_STATE` with the guard's sub-code. Phase 26 registers the `pack.islamic` guard (no FALSE while a
  `live` islamic version exists); phase 28 registers the same pattern for `pack.assistant`.
- `FlagChangeHook` (registered by the consuming phase): runs in the same transaction after the write. Phase 06
  registers the hook that enqueues `coverage_grade {reason: "flag"}` when `branch.observations_live` or
  `pack.islamic` changes.
Audit `flag.updated` is unchanged.

### 7.5 Ctx.via values
The comment on `Ctx.via` reads `// "session" | "pat" | "client_credentials" | "gateway_cert" | "webhook_hmac"`:
`gateway_cert` is a gateway listener with a pinned certificate (phase 29), `webhook_hmac` the directory webhook
(phase 31). The type is unchanged.

### 7.6 Window limit from the authority matrix
Port `DelegationAuthority` gains `window_limit_days(roles: &[RoleCode]) -> Option<u32>` (phase 09 section 3.6: the
smallest `max_days` of the roles); `NoMatrix` returns `None`. `DelegationService` step 2 reads: "length at most
`delegation.max_days` and, when the port returns `Some(n)`, at most `n` days." The phase 03 test that expects
`NoMatrix` asserts the `None` path.

### 7.7 One branch per liaison or alternate (phase 22, DR-205)
`BranchService::update` takes `pg_advisory_xact_lock` on each user it assigns and refuses a user who is already liaison
or alternate of another active branch: 422 `E_VALIDATION` at `/liaison_user_id` or `/alternate_user_id`, sub-code
`already_liaison_elsewhere`. Backstop in migration 0012, replacing the two plain indexes:

    CREATE UNIQUE INDEX IF NOT EXISTS uq_branches_liaison   ON admin.branches (liaison_user_id)
        WHERE liaison_user_id IS NOT NULL;
    CREATE UNIQUE INDEX IF NOT EXISTS uq_branches_alternate ON admin.branches (alternate_user_id)
        WHERE alternate_user_id IS NOT NULL;

(An archived branch has both columns NULL by its CHECK.) The case of one user as liaison of one branch and alternate
of another is caught by the service rule only.

### 7.8 UserRoleQuery port (phase 22)
Ports gain `UserRoleQuery { holders(role: RoleCode, scope: HolderScope) -> Vec<Uuid> }` over the effective roles
(direct grants and active delegations, the source of `User.roles` in the bundle), with `HolderScope { Any,
Branch(Uuid), Pool(Uuid) }`: `Any` is every active holder, `Branch(b)` a holder whose grant scope is
`all_branches` or lists `b`, `Pool(p)` a holder whose grant scope lists `p`. Read only, no audit row; used by the
phase 22 dispatcher.

### 7.9 Outbox topic for credential resets (phase 22)
From phase 22, which registers the topic and its handler in the same release, `CredentialResetService::request`
writes in its transaction the outbox row `credential_reset.requested {reset_id}`. Section 4 "Outbox topics" reads:
"`credential_reset.requested` (produced in section 3.8 from phase 22; handled by phase 22)."

### 7.10 Directory extensions to user administration (phase 31)
- Section 3.7 gains `UserAdminService::disable_from_directory` beside `disable` (its body and refusals are phase 31
  section 3.12): a reversible disable; grants, delegations and liaison assignments stay.
- `enable` refuses a linked account whose directory row has `directory_active = FALSE`: 409 `E_NOT_ALLOWED_IN_STATE`,
  sub-code `directory_inactive`. It reads the link through the port `DirectoryLinkQuery { is_inactive(user_id) ->
  bool }`, whose phase 03 implementation `NoDirectory` returns false; phase 31 supplies the real one.
- `user.disabled` gains the detail key `source`: `admin` for `disable`, `directory` for `disable_from_directory`.

### 7.11 staff_id in the user update (phase 15)
`UpdateUser` gains `staff_id?` (ADM only). The column, its unique index and the grant are phase 15's (its migration);
this phase only accepts the field. The `PATCH /admin/users/{id}` row keeps the `admin.users` scope and names the field
as ADM-only.

### 7.12 akr-ctl policy matrix (phase 08)
The `akr-ctl policy` commands of section 3.1 gain `akr-ctl policy matrix --bundle <dir> [--json]` (zone Z5):
`AccessMatrixBuilder` over the bundle directory with no database, printing `{bundle_hash, roles: [{role, actions}]}`
(the `RoleActions` list of 7.16) from the permits' `@role` annotations (phase 08 section 2). No audit row (no database); `make generate-api`
writes it to `frontend/src/app/policy-matrix.json`, and the release check of phase 25 records the hash it reads.

### 7.13 Development routes (DR-364)
Every development path (phase 00 `/_echo`, phase 03 `/_dev/masked`, phase 17 `/_dev/detections`, and the phase 04
subjects `_dev_object` and `_dev_masked`) is compiled only with the cargo feature `dev-routes`, which the release
build never enables, and in any build is mounted only when `AKR_ENV != production`. Section 6 gains a release-build
check: the inventory of development paths is listed in one file, and the release build answers 404 for each. Later
phases add their paths to that inventory.

Adversarial pass (2026-10-10), run 5 (existence masking leaks), from tools/agent/adv-run5.md; lead-verified (tools/agent/adv-run5-verdict.md).

### 7.14 Step-up and purpose answers to admitted callers (ADV-5-01, LOW; DR-411)
Section 3.6 step 4 (and DR-48's last clause) applies to a caller the policy refuses: such a caller gets the masked 404,
never 401 or 422, whatever header is missing. A caller the policy admits (from phase 17 on, the suspicion forbids already
include the analyst network and the purpose code, so a refused caller never reaches the handler) may get 401
`E_STEPUP_REQUIRED` for a missing fresh step-up, and on a restricted record 422 `E_PURPOSE_REQUIRED` (DR-46), as phases
17, 18, 19 and 21 state: those answers confirm nothing the caller may not know. The inference harness probes the refused
caller only; one more probe per masked kind checks that a caller outside the team without step-up gets the 404, not the
401.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.15 Development routes in the authorize layer (V-03-03-a, MEDIUM; DR-413)
The layer of section 3.2 needs a `RouteAction` for every `/api/v1` route, and the development routes had none. The
cargo feature `dev-routes` (7.13) adds them to `ROUTES` with actions of the namespace `dev.*` and compiles in the
policy file `policies/dev.cedar`, which `loader::load` appends to the active bundle whenever the feature is on
(`AKR_ENV` governs only the mount of the routes, 7.13). A release build has neither, so the shipped bundle and its hash do not change; the matrix
file of 7.12 is built without the feature. With the feature on, the three actions are members of `Action::ALL`
(`#[cfg(feature = "dev-routes")]`) and are declared in `policies/dev.cedarschema`, a schema fragment that
`loader::load` appends together with `dev.cedar`; the fragment also declares the entity type of `_dev_masked` with
its `tier` and `access_list` attributes. The permits of `dev.cedar` carry `@role("*")`, which step 3 of section 3.1
accepts for the `dev.*` actions as it does for the `self` group (no `actor_role`). So the inventory of section 3.1
step 3 holds in both builds, and the schema test of section 2 compares `Action::ALL` with the shipped text plus the
fragment when the feature is on.

| Route | Action | Resource | Permit in `dev.cedar` |
|-|-|-|-|
| `POST /api/v1/_echo` (phase 00) | `dev.echo` | System | every active principal |
| `GET /api/v1/_dev/masked/{id}` | `dev.masked_read` | `_dev_masked` (masked kind) | the `normal` record: every active principal; the `suspicion` record: only a principal on its access list, so a refused read is the 404 of section 3.6 |
| `POST /api/v1/_dev/detections` (from phase 17 on) | `dev.detection_emit` | System | every active principal |

The scope gate treats the three as "none (dev only)", as the phase 00 route table does. The coverage test of section 6
runs twice, with the feature and without it; a later phase that adds a development route adds its row here by the
same rule.

### 7.16 One derivation for the role part of the access matrix (V-03-11-a LOW, V-03-11-b MEDIUM; DR-413)
The role part of the matrix has one derivation, step 1 of `AccessMatrixBuilder::build` in section 3.5 (an evaluation
with a synthetic principal per role, so forbids count). It is the pure function
`AccessMatrixBuilder::roles(bundle: &LoadedBundle) -> Vec<RoleActions>` with `RoleActions { role, actions:
Vec<String> }`, both sorted; it reads no database. `build(at)` calls it for its `roles` part and adds the principals of
step 2. `akr-ctl policy matrix` (7.12) loads the directory with `loader::load` and prints the output of `roles`; the
words "from the permits' `@role` annotations" in 7.12 say only where a role's name comes from. For one bundle hash the
file and the `roles` part of `GET /admin/access-matrix` are therefore equal, and section 6 gains that comparison as a
test.

### 7.17 The masked deny tag (V-06-06-a, V-06-06-b, MEDIUM; DR-413)
Section 3.2 step 6 names two deny tags. From phase 06 on the policies use a third, `@deny("masked")`, on a `forbid` that hides a record by an attribute other than a branch, pool or tier test (a restricted scenario, a policy pack outside its readers, a report outside its case team). A determining forbid with this tag is classified `DenyReason::Masked`, a new variant for which `masks` returns true whatever the resource kind: the answer is the 404 `E_NOT_FOUND` body of a missing record, and the access event is the `deny` of section 3.2 step 8. The kind need not be in `MASKED_KINDS`: `loader::load` collects the resource kinds that such forbids name, and for section 3.2 step 8 and section 3.6 steps 2 and 3 (the access event, the same work and the latency floor) those kinds count as masked kinds, so a caller cannot tell a miss from a refusal on them by the answer or by its timing. A list route that can return such rows filters them in SQL by the same predicate (section 3.6, lists), and the phase that writes the forbid tests the 404.
Every other tag (`self_approval`, `state`, `aal` and the like) names the reason of a `Forbidden` deny; its problem code is the one the phase that writes the policy gives.

## What This Phase Does NOT Touch

- No change requests (phase 04): role grants, governed settings and bundle activation after the bootstrap wait for
  them; `PUT /admin/policies`, `POST /admin/users/{id}/grants` and the governed branch of `PUT /admin/settings/{key}`
  arrive there. `policy_bundles.change_request_id` has no FK yet.
- No tiered or masked business resources: sensitivity tiers, case access lists, the relationship-conflict check, the
  suspicion-tier grant rule on vetting, and canaries start in phases 15 and 18; break-glass and the quarterly access
  review lifecycle in phase 25; access history and the G5 screen in phase 24.
- No authority matrix and no leave ledger (phase 09; the delegation hook passes until then), no purpose-code editing
  (phase 09).
- No notifications (phase 22): security officers find pending resets on their list.
- No OIDC, SAML or SCIM (phases 25 and 31); no SIEM forwarding (phase 25).
- No screens (phase 08).
- No CBS connection; no change to phase 02 tables; phase 01 tables gain only the branch FK and `enabled_at`.
