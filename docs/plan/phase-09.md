# Phase 09: Reference Data, Calendars & Policy Pack

**Goal:** The reference tables that scenarios, ratings, reports and clocks read (countries with expiring risk overrides, governorates, currencies, CBI and manually recorded FX rates, calendars with per-currency weekends and holidays reconciled to the CBS calendar), the business clock that every working-day deadline uses, the reason-code families beyond change and tuning, the policy pack (routing, clocks, calendars, switches, templates, forbidden-word list) and the delegated-authority matrix, all changed only through change requests, with the phase 03 `NoMatrix` placeholder replaced by the matrix.

**Dependencies:** Phase 04 (change control: `ChangeTargetRegistry`, `reason_codes`, the route map, the workflow engine), Phase 03 (settings, flags, Cedar, the `DelegationAuthority` port), Phase 05 (CBS calendar capture and the discovery seal; needed by section 3.3 only)
**Macro-phase:** P1
**Complexity:** L (21 routes; the policy-pack validator, the business clock and the matrix permit logic carry the weight)
**Binaries touched:** akr-api (routers `reference`, `policy`, `authority`; `BankCalendarClock` as the API's `BusinessClock`) | akr-worker (jobs `fx_rate_import`, `ref_expiry_sweep`; the apply hooks of six change targets) | akr-ingestd (calendar capture step inside `discovery_run`, phase 05) | `akr-ctl` (`ref seed`, `ref verify-data`, `policy-pack seed`). akr-assess unchanged
**Requirements carried:** S-13 (calendar service: Hijri display, Ramadan, Eid, Arbaeen, pay days, used by windows and clocks), T-04 (calendar-aware windows; IQD and USD kept separate), T-10 (a governed calendar service used by thresholds, baselines and explanations), RP-50 (CBI rate of the day for the daily CTR job; the exact-date rule of section 3.2 is what phase 21 consumes), RP-46 (working-day clock engine with the Iraqi weekend and a manual holiday override), RP-62 (neutral-wording library and tipping-off inference test), TS-40 (the CBS calendar is the business-calendar authority), UX-22 (Hijri is display only), W-19 (country-risk table with versioning and overrides carrying approver and expiry; file 04), TR-10 (a country-risk table change re-scores affected customers; file 26), K-24 (override register: downward changes by compliance, expiry within twelve months; file 26), M-14 (versioned routing table), M-20 (delegated-authority matrix signed by the MLRO), M-23 (business calendars per bank, region and branch; holidays under maker-checker), M-27 (governed reason-code libraries), M-32 (neutral-wording library tested for forbidden words), M-42 (RFI templates and free-text scan for forbidden words), M-75 (policy pack: routing, clocks, calendars, delegated authority, reason codes, templates, switches; versioned, pinned per case, drained not migrated), T-44 (reason-code library with stable ids and bilingual reviewed templates), GV-44 (customer-facing explanation never reveals suspicion), SP-31 (neutral wording for tasks and notices), IR-31 (generic-text check on decisions), UX-65 (decision forms refuse generic or capacity-driven text), M-44 (correspondent RFI deadline clock, default ten business days), IR-56 (forbidden words for text that leaves the compliance zone), UX-68 (forbidden-word scan before release), GV-01 (an inventory record before a reference list or matcher is used; the first changes of a kind wait for it), C-38 (signed release data: reference releases and the bootstrap policy pack)
- Gap pass 1 (coverage), partly implemented: S-14 (country risk levels with expiring overrides); F-14 (the country table with effective dates driving floors and re-scores).
- Gap pass 1 (coverage), not carried: S-15 (governed reference registers, P2, DR-380).

No code-list row with its own ID exists in research files 04 and 26; the code-list part of the M8 requirement line is carried here by T-44 and TS-40.

---

## 1. SQLx Migration Changes

Three migrations, `0026_ref_reference_data`, `0027_policy_pack_authority` and `0028_reference_settings` (canonical chain
in roadmap.md). Eight tables in `ref` (seven of DATA-MODEL s.7 plus `ref.cbs_calendar_captures`, which holds the CBS capture of
section 3.3), two tables in `admin` (`policy_packs` and `authority_matrix`, placed in `admin` by DATA-MODEL s.1; the roadmap entity list names no
schema for them), the insert and update grants that phase 04 deferred on `ref.reason_codes`, and the foreign key on
`admin.timers.calendar_id` that phase 00 deferred to this phase.

Two deliberate departures from the id convention, both stated here so the gap pass can check them: the ISO-keyed reference
tables (`countries` by ISO 3166-1 alpha-2, `currencies` by ISO 4217 alpha, `governorates` by ISO 3166-2:IQ) use the code
as primary key, because the code is the identity every CBS mapping and every rating already uses; and `authority_matrix`
and `policy_packs` do not reference `admin.stepup_assertions` with a foreign key, because phase 01 prunes that table
(R12); the signature evidence is the audit event and the sealed document instead.

```sql
-- migration 0026_ref_reference_data.sql
-- Phase 09. Reference tables of the `ref` schema, the CBS calendar captures, and the grants on phase 04's
-- ref.reason_codes (MODULES M8, DATA-MODEL s.7, DR-39). Reference rows are never deleted: a row is ended, retired or
-- deactivated by a change request (phase 04); the guard triggers are the second layer under the grants.

CREATE TABLE IF NOT EXISTS ref.countries (
    iso2            CHAR(2) PRIMARY KEY CHECK (iso2 ~ '^[A-Z]{2}$'),
    name            JSONB NOT NULL CHECK (name ? 'ar' AND name ? 'en'),
    risk_level      TEXT NOT NULL CHECK (risk_level IN ('low', 'medium', 'high', 'very_high')),   -- base level, release data
    basis           JSONB NOT NULL CHECK (basis ? 'ar' AND basis ? 'en'),   -- list of record and date the level comes from
    release_id      TEXT NOT NULL CHECK (length(release_id) BETWEEN 1 AND 64),   -- the signed reference release
    version         INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- One row per override episode. The open row (ended_at null) is the override in force; ended rows are history.
CREATE TABLE IF NOT EXISTS ref.country_risk_overrides (
    id                  UUID PRIMARY KEY,
    iso2                CHAR(2) NOT NULL REFERENCES ref.countries(iso2) ON DELETE RESTRICT,
    level               TEXT NOT NULL CHECK (level IN ('low', 'medium', 'high', 'very_high')),
    direction           TEXT NOT NULL CHECK (direction IN ('raise', 'lower')),   -- against the level in force before
    reason              TEXT NOT NULL CHECK (length(btrim(reason)) BETWEEN 10 AND 2000),
    expires_at          TIMESTAMPTZ NOT NULL,
    ended_at            TIMESTAMPTZ,
    ended_reason        TEXT CHECK (ended_reason IN ('expired', 'superseded', 'reverted')),
    change_request_id   UUID NOT NULL REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    applied_by          UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,   -- the final approver
    applied_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK ((ended_at IS NULL) = (ended_reason IS NULL)),
    CHECK (ended_at IS NULL OR ended_at >= applied_at),
    CHECK (expires_at > applied_at)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_country_override_open ON ref.country_risk_overrides (iso2) WHERE ended_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_country_override_expiry  ON ref.country_risk_overrides (expires_at) WHERE ended_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_country_override_cr      ON ref.country_risk_overrides (change_request_id);
CREATE INDEX IF NOT EXISTS idx_country_override_applied ON ref.country_risk_overrides (applied_by);

CREATE TABLE IF NOT EXISTS ref.governorates (
    code            TEXT PRIMARY KEY CHECK (code ~ '^IQ-[A-Z]{2}$'),   -- ISO 3166-2:IQ
    name            JSONB NOT NULL CHECK (name ? 'ar' AND name ? 'en'),
    region          TEXT NOT NULL CHECK (length(region) BETWEEN 1 AND 40),
    release_id      TEXT NOT NULL CHECK (length(release_id) BETWEEN 1 AND 64),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS ref.currencies (
    code            CHAR(3) PRIMARY KEY CHECK (code ~ '^[A-Z]{3}$'),   -- ISO 4217 alpha; never a CBS internal code
    minor_unit      SMALLINT NOT NULL CHECK (minor_unit BETWEEN 0 AND 4),   -- ISO 4217; the CBS value is compared at discovery
    name            JSONB NOT NULL CHECK (name ? 'ar' AND name ? 'en'),
    is_active       BOOLEAN NOT NULL DEFAULT TRUE,
    release_id      TEXT NOT NULL CHECK (length(release_id) BETWEEN 1 AND 64),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Facts: an FX rate is never updated or deleted. A correction is a new row from another source (section 3.2).
CREATE TABLE IF NOT EXISTS ref.fx_rates (
    id              UUID PRIMARY KEY,
    currency        CHAR(3) NOT NULL REFERENCES ref.currencies(code) ON DELETE RESTRICT,
    rate_date       DATE NOT NULL,                                  -- the business date the rate applies to
    rate_to_iqd     NUMERIC(28,6) NOT NULL CHECK (rate_to_iqd > 0),  -- IQD per one unit of the currency
    rate_to_usd     NUMERIC(28,6) NOT NULL CHECK (rate_to_usd > 0),
    source          TEXT NOT NULL CHECK (source IN ('cbi', 'manual')),
    evidence_id     UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,   -- manual rows: the CBI notice
    recorded_by     UUID REFERENCES admin.users(id) ON DELETE RESTRICT,              -- manual rows: the steward
    fetched_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_fx_rates_day UNIQUE (currency, rate_date, source),
    CHECK ((source = 'manual') = (evidence_id IS NOT NULL AND recorded_by IS NOT NULL)),
    CHECK (currency <> 'IQD' OR rate_to_iqd = 1)
);
CREATE INDEX IF NOT EXISTS idx_fx_rates_lookup   ON ref.fx_rates (currency, rate_date DESC, source);
CREATE INDEX IF NOT EXISTS idx_fx_rates_evidence ON ref.fx_rates (evidence_id) WHERE evidence_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_fx_rates_recorded ON ref.fx_rates (recorded_by) WHERE recorded_by IS NOT NULL;

CREATE TABLE IF NOT EXISTS ref.calendars (
    id              UUID PRIMARY KEY,
    name            JSONB NOT NULL CHECK (name ? 'ar' AND name ? 'en'),
    -- {"weekend": {"default": [5, 6], "IQD": [5, 6]}}: ISO weekdays (1 Monday .. 7 Sunday). Working days are the rest.
    -- A currency key overrides "default" for the weekend that currency's business uses (DR-39).
    week            JSONB NOT NULL CHECK (jsonb_typeof(week -> 'weekend') = 'object'
                                          AND jsonb_typeof(week -> 'weekend' -> 'default') = 'array'
                                          AND jsonb_array_length(week -> 'weekend' -> 'default') BETWEEN 1 AND 3),
    is_default      BOOLEAN NOT NULL DEFAULT FALSE,
    version         INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_calendars_default ON ref.calendars ((true)) WHERE is_default;

CREATE TABLE IF NOT EXISTS ref.calendar_days (
    id                  UUID PRIMARY KEY,
    calendar_id         UUID NOT NULL REFERENCES ref.calendars(id) ON DELETE RESTRICT,
    on_date             DATE NOT NULL CHECK (on_date BETWEEN DATE '2000-01-01' AND DATE '2100-12-31'),
    kind                TEXT NOT NULL CHECK (kind IN ('holiday', 'ramadan_hours', 'eid', 'arbaeen', 'payday')),
    name                JSONB NOT NULL CHECK (name ? 'ar' AND name ? 'en'),
    -- ramadan_hours: {"from": "HH:MM", "to": "HH:MM"}; other kinds: {}
    attrs               JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(attrs) = 'object'),
    hijri_label         TEXT CHECK (hijri_label IS NULL OR length(hijri_label) <= 80),   -- display only (UX-22)
    active              BOOLEAN NOT NULL DEFAULT TRUE,                  -- deactivated, never deleted
    source              TEXT NOT NULL CHECK (source IN ('manual', 'cbs_reconciled', 'release')),
    change_request_id   UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    version             INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_calendar_days UNIQUE (calendar_id, on_date, kind),
    CHECK (kind <> 'ramadan_hours' OR (attrs ? 'from' AND attrs ? 'to'))
);
CREATE INDEX IF NOT EXISTS idx_calendar_days_active ON ref.calendar_days (calendar_id, on_date) WHERE active;
CREATE INDEX IF NOT EXISTS idx_calendar_days_cr     ON ref.calendar_days (change_request_id) WHERE change_request_id IS NOT NULL;

-- The CBS's own holiday and currency-week rows, captured whole by akr-ingestd during a discovery run (section 3.3).
-- Reference data only: nothing from a customer or an account is captured. Read by the reconciliation view.
CREATE TABLE IF NOT EXISTS ref.cbs_calendar_captures (
    run_id              UUID NOT NULL REFERENCES admin.discovery_runs(id) ON DELETE RESTRICT,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    object_key          TEXT NOT NULL CHECK (object_key IN ('holiday', 'currency_week')),
    row_key             TEXT NOT NULL CHECK (length(row_key) BETWEEN 1 AND 120),   -- holiday: CUR|FROM_DATE; currency: CUR
    cbs_currency_code   TEXT NOT NULL CHECK (length(cbs_currency_code) BETWEEN 1 AND 20),   -- the CBS internal code
    on_date             DATE,                                   -- holiday rows: FROM_DATE
    fst_work_date       DATE,                                   -- holiday rows: FST_WORK_DATE (first working day after)
    attrs               JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(attrs) = 'object'),
                        -- currency rows: {"iso_alpha": as read, "week_end": as read, "week_end_days": as read, "dec_pla": ...}
    captured_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (run_id, object_key, row_key),
    CHECK ((object_key = 'holiday') = (on_date IS NOT NULL)),
    CHECK (fst_work_date IS NULL OR on_date IS NULL OR fst_work_date >= on_date)
);
CREATE INDEX IF NOT EXISTS idx_cbs_calendar_source ON ref.cbs_calendar_captures (source_id);

-- No deletes on reference rows (DATA-MODEL conventions). fx_rates is a fact: no update either.
CREATE OR REPLACE FUNCTION ref.no_delete() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION 'reference rows are never deleted; end or deactivate the row (phase 09)'
        USING ERRCODE = 'insufficient_privilege';
END $$;

CREATE OR REPLACE FUNCTION ref.fx_append_only() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION 'an FX rate is a fact: a correction is a new row from another source (phase 09)'
        USING ERRCODE = 'insufficient_privilege';
END $$;

DO $$
DECLARE t TEXT;
BEGIN
    FOREACH t IN ARRAY ARRAY['countries', 'country_risk_overrides', 'governorates', 'currencies', 'calendars',
                             'calendar_days', 'cbs_calendar_captures'] LOOP
        BEGIN
            EXECUTE format('CREATE TRIGGER %I BEFORE DELETE ON ref.%I FOR EACH ROW EXECUTE FUNCTION ref.no_delete()',
                           'trg_' || t || '_no_delete', t);
        EXCEPTION WHEN duplicate_object THEN NULL;
        END;
    END LOOP;
END $$;

DO $$ BEGIN
    CREATE TRIGGER trg_fx_rates_append_only BEFORE UPDATE OR DELETE ON ref.fx_rates
        FOR EACH ROW EXECUTE FUNCTION ref.fx_append_only();
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

-- The CBS calendar is the authority for the business calendar (TS-40): the foreign key that phase 00 deferred.
DO $$ BEGIN
    ALTER TABLE admin.timers ADD CONSTRAINT fk_timers_calendar
        FOREIGN KEY (calendar_id) REFERENCES ref.calendars(id) ON DELETE RESTRICT;
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
CREATE INDEX IF NOT EXISTS idx_timers_calendar ON admin.timers (calendar_id) WHERE calendar_id IS NOT NULL;

-- Phase 04 gave akr_app SELECT on ref.reason_codes until this phase adds its administration. Only the lifecycle and
-- the revisable columns are writable; code and family never change (a new family is a new code).
GRANT INSERT ON ref.reason_codes TO akr_app;
GRANT UPDATE (tag, label, template, typology, cbi_refs, display_fields, object_kinds, active, version, updated_by,
              updated_at)
    ON ref.reason_codes TO akr_app;

GRANT SELECT, INSERT ON ref.countries, ref.governorates, ref.currencies TO akr_app;
GRANT UPDATE (name, risk_level, basis, release_id, version, updated_at) ON ref.countries TO akr_app;
GRANT UPDATE (name, region, release_id, updated_at) ON ref.governorates TO akr_app;
GRANT UPDATE (name, minor_unit, is_active, release_id, updated_at) ON ref.currencies TO akr_app;
GRANT SELECT, INSERT ON ref.country_risk_overrides TO akr_app;
GRANT UPDATE (ended_at, ended_reason) ON ref.country_risk_overrides TO akr_app;
GRANT SELECT, INSERT ON ref.fx_rates TO akr_app;
GRANT SELECT, INSERT ON ref.calendars TO akr_app;
GRANT UPDATE (name, week, is_default, version, updated_at) ON ref.calendars TO akr_app;
GRANT SELECT, INSERT ON ref.calendar_days TO akr_app;
GRANT UPDATE (kind, name, attrs, hijri_label, active, source, change_request_id, version, updated_at)
    ON ref.calendar_days TO akr_app;
GRANT SELECT ON ref.cbs_calendar_captures TO akr_app;
GRANT INSERT ON ref.cbs_calendar_captures TO akr_ingest;   -- written by the discovery run in akr-ingestd only
GRANT SELECT ON ref.countries, ref.country_risk_overrides, ref.governorates, ref.currencies, ref.fx_rates,
                ref.calendars, ref.calendar_days, ref.cbs_calendar_captures TO akr_readonly;

-- Classification (DATA-MODEL s.0). Country levels, overrides and calendars drive ratings and clocks: detection logic.
-- Reason-code templates describe what the bank monitors: detection logic (this changes a phase 04 classification).
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification, note)
SELECT 'ref', v.t, c, v.cls::admin.data_classification, v.note
FROM (VALUES
  ('countries',                'public',          ARRAY['iso2','name'],                       'Public names and codes'),
  ('countries',                'detection-logic', ARRAY['risk_level','basis'],                 'Levels that feed ratings (W-19)'),
  ('countries',                'internal',        ARRAY['release_id','version','updated_at'], NULL),
  ('country_risk_overrides',   'detection-logic', ARRAY['iso2','level','direction','reason'], 'Overrides feed ratings (K-24)'),
  ('country_risk_overrides',   'internal',        ARRAY['id','expires_at','ended_at','ended_reason',
                                                        'change_request_id','applied_by','applied_at'], NULL),
  ('governorates',             'public',          ARRAY['code','name','region'],               NULL),
  ('governorates',             'internal',        ARRAY['release_id','updated_at'],            NULL),
  ('currencies',               'public',          ARRAY['code','minor_unit','name'],           NULL),
  ('currencies',               'internal',        ARRAY['is_active','release_id','updated_at'], NULL),
  ('fx_rates',                 'financial',       ARRAY['rate_to_iqd','rate_to_usd'],         'Rates that value amounts (DATA-MODEL s.0)'),
  ('fx_rates',                 'internal',        ARRAY['id','currency','rate_date','source','evidence_id',
                                                        'recorded_by','fetched_at'],           NULL),
  ('calendars',                'internal',        ARRAY['id','name','week','is_default','version','created_at',
                                                        'updated_at'],                         NULL),
  ('calendar_days',            'internal',        ARRAY['id','calendar_id','on_date','kind','name','attrs',
                                                        'hijri_label','active','source','change_request_id',
                                                        'version','created_at','updated_at'],  NULL),
  ('cbs_calendar_captures',    'internal',        ARRAY['run_id','source_id','object_key','row_key',
                                                        'cbs_currency_code','on_date','fst_work_date','attrs',
                                                        'captured_at'],                        'CBS reference rows only')
) AS v(t, cls, cols, note), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification, note)
VALUES ('ref', 'reason_codes', 'template', 'detection-logic',
        'Detection text describes what the bank monitors (M-27, T-44); phase 09 reclassifies it')
ON CONFLICT (schema_name, table_name, column_name)
DO UPDATE SET classification = EXCLUDED.classification, note = EXCLUDED.note;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('ref', 'countries',              'R9', 'retirement', NULL,          'Base level replaced by a later release'),
 ('ref', 'country_risk_overrides', 'R9', 'retirement', NULL,          'Override history: the rating explanation cites it'),
 ('ref', 'governorates',           'R9', 'retirement', NULL,          'Replaced by a later release'),
 ('ref', 'currencies',             'R9', 'retirement', NULL,          'Deactivated, never deleted'),
 ('ref', 'fx_rates',               'R2', 'age',        'rate_date',   'Kept as long as the transactions valued at the rate (R2 floor)'),
 ('ref', 'calendars',              'R9', 'retirement', NULL,          'Configuration history'),
 ('ref', 'calendar_days',          'R9', 'retirement', NULL,          'Deactivated days stay for clocks already started'),
 ('ref', 'cbs_calendar_captures',  'R9', 'retirement', NULL,          'CBS reference capture; the reconciliation cites the run')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

```sql
-- migration 0027_policy_pack_authority.sql
-- Phase 09. The policy pack versions and the delegated-authority matrix versions (MODULES M2 and M15, DATA-MODEL s.1,
-- M-20, M-75). Both are append-only: a version row is inserted when its change request activates, and never changed.

CREATE OR REPLACE FUNCTION admin.append_only() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION '% versions are append-only (phase 09)', TG_TABLE_NAME
        USING ERRCODE = 'insufficient_privilege';
END $$;

CREATE TABLE IF NOT EXISTS admin.policy_packs (
    id                  UUID PRIMARY KEY,
    version             INT NOT NULL UNIQUE CHECK (version >= 1),
    -- The document (section 3.5). Keys are closed: an unknown key is refused by the validator and by this CHECK.
    document            JSONB NOT NULL CHECK (jsonb_typeof(document) = 'object'
                                              AND document ->> 'schema' = 'policy-pack.v1'),
    content_sha256      BYTEA NOT NULL UNIQUE CHECK (octet_length(content_sha256) = 32),   -- over canonical JSON
    change_request_id   UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,   -- NULL for the bootstrap version
    release_id          TEXT CHECK (release_id IS NULL OR length(release_id) BETWEEN 1 AND 64),
    approved_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- the final approver; NULL for bootstrap
    effective_from      TIMESTAMPTZ NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK ((change_request_id IS NULL) <> (release_id IS NULL)),      -- exactly one origin
    CHECK ((change_request_id IS NULL) = (approved_by IS NULL))
);
CREATE INDEX IF NOT EXISTS idx_policy_packs_cr          ON admin.policy_packs (change_request_id)
    WHERE change_request_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_policy_packs_approved_by ON admin.policy_packs (approved_by) WHERE approved_by IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_policy_packs_effective   ON admin.policy_packs (effective_from DESC);

CREATE TABLE IF NOT EXISTS admin.authority_matrix (
    id                  UUID PRIMARY KEY,
    version             INT NOT NULL UNIQUE CHECK (version >= 1),
    document            JSONB NOT NULL CHECK (jsonb_typeof(document) = 'object'
                                              AND document ->> 'schema' = 'authority-matrix.v1'),
    content_sha256      BYTEA NOT NULL UNIQUE CHECK (octet_length(content_sha256) = 32),
    change_request_id   UUID NOT NULL REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    signed_by           UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,   -- the MLRO's sign step
    -- Not a foreign key: admin.stepup_assertions is pruned after stepup.retention_days (phase 01, R12). The signature
    -- is proven by this value, the sign step's audit event and the content hash.
    stepup_id           UUID NOT NULL,
    effective_from      TIMESTAMPTZ NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_authority_cr        ON admin.authority_matrix (change_request_id);
CREATE INDEX IF NOT EXISTS idx_authority_signed_by ON admin.authority_matrix (signed_by);
CREATE INDEX IF NOT EXISTS idx_authority_effective ON admin.authority_matrix (effective_from DESC);

DO $$ BEGIN
    CREATE TRIGGER trg_policy_packs_append_only BEFORE UPDATE OR DELETE ON admin.policy_packs
        FOR EACH ROW EXECUTE FUNCTION admin.append_only();
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    CREATE TRIGGER trg_authority_matrix_append_only BEFORE UPDATE OR DELETE ON admin.authority_matrix
        FOR EACH ROW EXECUTE FUNCTION admin.append_only();
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

GRANT SELECT, INSERT ON admin.policy_packs, admin.authority_matrix TO akr_app;   -- insert on activation only
GRANT SELECT ON admin.policy_packs, admin.authority_matrix TO akr_readonly;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification, note) VALUES
 ('admin', 'policy_packs', 'document',       'detection-logic', 'Forbidden-word list, switches and clocks (M-75)'),
 ('admin', 'policy_packs', 'content_sha256', 'internal',        NULL),
 ('admin', 'policy_packs', 'version',        'internal',        NULL),
 ('admin', 'policy_packs', 'change_request_id', 'internal',     NULL),
 ('admin', 'policy_packs', 'release_id',     'internal',        NULL),
 ('admin', 'policy_packs', 'approved_by',    'internal',        NULL),
 ('admin', 'policy_packs', 'effective_from', 'internal',        NULL),
 ('admin', 'policy_packs', 'created_at',     'internal',        NULL),
 ('admin', 'policy_packs', 'id',             'internal',        NULL),
 ('admin', 'authority_matrix', 'document',   'internal',        'Who may decide what: staff-level, no customer data'),
 ('admin', 'authority_matrix', 'content_sha256', 'internal',    NULL),
 ('admin', 'authority_matrix', 'version',    'internal',        NULL),
 ('admin', 'authority_matrix', 'change_request_id', 'internal', NULL),
 ('admin', 'authority_matrix', 'signed_by',  'internal',        NULL),
 ('admin', 'authority_matrix', 'stepup_id',  'internal',        NULL),
 ('admin', 'authority_matrix', 'effective_from', 'internal',    NULL),
 ('admin', 'authority_matrix', 'created_at', 'internal',        NULL),
 ('admin', 'authority_matrix', 'id',         'internal',        NULL)
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'policy_packs',     'R9', 'retirement', NULL,
  'Every version kept; a case pins the version it opened under (phase 18)'),
 ('admin', 'authority_matrix', 'R9', 'retirement', NULL,
  'Every version kept; printed in the assessor pack (M-20)')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

```sql
-- migration 0028_reference_settings.sql
-- Phase 09. Settings, schedules and the feature flag of the reference and policy surface. Bounds are in the registry
-- (common::settings_registry, DR-34); the governed keys change only through a change request (phase 04).

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('fx.rate_max_age_days', '3', 'integer', NULL, TRUE,
  '{"ar": "أقصى عمر لسعر صرف يُعرض عند غياب سعر اليوم (أيام)", "en": "Oldest rate shown for display when the day has none (days)"}'),
 ('country.override_max_days', '365', 'integer', NULL, TRUE,
  '{"ar": "أقصى مدة لتجاوز تصنيف الدولة (أيام)", "en": "Longest life of a country risk override (days)"}'),
 ('calendar.horizon_days', '365', 'integer', NULL, FALSE,
  '{"ar": "الأفق الذي يجب أن يغطيه التقويم (أيام)", "en": "Days ahead that the calendar must already hold"}'),
 ('calendar.capture_max_rows', '5000', 'integer', NULL, FALSE,
  '{"ar": "أقصى عدد صفوف يُلتقط من جدول العطل في النظام المصرفي", "en": "Most CBS calendar rows captured per object"}'),
 ('policy.dry_run_max_rows', '5000', 'integer', NULL, FALSE,
  '{"ar": "أقصى عدد صفوف لمحاكاة حزمة السياسات", "en": "Most history rows a policy dry run reads"}'),
 ('policy.dry_run_history_days', '365', 'integer', NULL, FALSE,
  '{"ar": "نافذة التاريخ لمحاكاة حزمة السياسات (أيام)", "en": "History window of a policy dry run (days)"}'),
 ('schedule.fx_rate_import', '"0 30 9 * * 0-4"', 'cron', NULL, FALSE,
  '{"ar": "جدول استيراد أسعار الصرف (أيام العمل)", "en": "CBI rate import schedule (working days)"}'),
 ('schedule.ref_expiry_sweep', '"0 10 0 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول إنهاء تجاوزات التصنيف المنتهية", "en": "Expired country override sweep schedule"}')
ON CONFLICT (key) DO NOTHING;

INSERT INTO admin.feature_flags (key, enabled, label) VALUES
 ('fx.cbi_feed_enabled', FALSE,
  '{"ar": "استيراد أسعار المصرف المركزي مفعّل", "en": "CBI rate import enabled"}')
ON CONFLICT (key) DO NOTHING;
```

## 2. DDD Implementation (Rust)

**Crate placement** (workspace pinned in phase 00; layer rules in `ddd.md`):

```
backend/
├── packs/reference/                  # signed reference releases (C-38): countries, governorates, currencies,
│   │                                 #   default calendar, reason-codes.v1.json (inactive until activated)
│   └── policy/policy-pack.v1.json    # the bootstrap policy pack (product default; compliance reviews before install)
├── schemas/
│   ├── reference-release.v1.json     # validates the release files of packs/reference
│   ├── policy-pack.v1.json           # closed document schema (section 3.5); include_str!
│   ├── authority-matrix.v1.json      # closed document schema (section 3.6)
│   └── reason-code.v1.json           # one reason code, both languages, template rules
├── workflows/shipped/cr_authority.json   # the new shipped approval family (section 4, phase 04 update)
├── crates/
│   ├── domain/   # + reference (Country, CountryRiskOverride, Governorate, Currency, FxRate, Calendar, CalendarDay,
│   │             #   CbsCalendarCapture, RiskLevel, RateSource, DayKind, WeekendRule; FxRateSource port;
│   │             #   CalendarReconciler (pure); BankCalendarClock (BusinessClock impl, pure over a day set);
│   │             #   ReasonCode and ReasonFamily with the template rules (pure));
│   │             #   policy (PolicyPackDocument, PolicyValidator, ForbiddenWordScanner, PolicyDryRun (pure),
│   │             #   HistorySource port, ClockRegistry, SwitchRegistry);
│   │             #   authority (AuthorityMatrixDocument, DecisionType (closed), Priority, AuthorityMatrix (pure),
│   │             #   AuthorityDelegation: DelegationAuthority impl)
│   ├── db/       # + Pg repositories for the ten tables (section 2, repository traits); the CBS capture writer is
│   │             #   used from akr-ingestd only
│   ├── authz/    # + resource adapters Country, Calendar, CalendarDay, ReasonCode, PolicyPack, AuthorityMatrix
│   └── jobs/     # + kinds fx_rate_import (worker), ref_expiry_sweep (worker)
├── api/          # + routers reference, policy, authority; the six ChangeTarget impls registered at start-up
├── worker/       # + FxRateService (CbiPublicationSource behind the feature flag), ExpirySweep, the six apply hooks
├── ingestd/      # + calendar capture step of discovery_run (phase 05 pipeline; SELECT only, approved objects only)
└── ctl/          # + ref seed, ref verify-data, policy-pack seed
```

- **Domain entities** (`Debug, Serialize, Deserialize, FromRow, ToSchema`; fields mirror section 1; enums as Rust enums
  with `#[sqlx(type_name = ...)]` where the column is an enum, text with a `CHECK` otherwise): `Country { iso2,
  name, risk_level: RiskLevel, basis, release_id, version, updated_at }`, `CountryRiskOverride`, `Governorate`,
  `Currency`, `FxRate`, `Calendar`, `CalendarDay`, `CbsCalendarCapture`, `PolicyPack { id, version, document:
  PolicyPackDocument, content_sha256, change_request_id?, release_id?, approved_by?, effective_from, created_at }`,
  `AuthorityMatrixVersion { id, version, document: AuthorityMatrixDocument, content_sha256, change_request_id,
  signed_by, stepup_id, effective_from, created_at }`, `ReasonCode` (phase 04 table, now administered here).
- **Value objects:** `IsoCountry`, `IsoCurrency` (`CHAR(3)`), `GovernorateCode` (`IQ-XX`), `RiskLevel` (`Low < Medium
  < High < VeryHigh`, `Ord`), `RateSource` (`Cbi`, `Manual`), `WeekendRule { weekend: BTreeMap<CurrencyKey,
  BTreeSet<IsoWeekday>> }` with `IsoWeekday` in `1..=7` (Monday is 1), `DayKind` (`Holiday`, `RamadanHours`, `Eid`,
  `Arbaeen`, `Payday`; `DayKind::closes_day(self) -> bool` is true for `Holiday`, `Eid` and `Arbaeen` only),
  `BusinessUnit` (`WorkingDays`, `Hours`, `Days`), `HijriVariant` (the variant label stored with every Hijri text),
  `ReasonFamily` (`Detection`, `Decision`, `Tuning`, `Change`, `Closure`, `Override`, `Exclusion`), `ReasonTag`
  (`Risk`, `Capacity`), `Priority` (`Immediate` = class P0, `Urgent` = P1, `Standard` = P2, `Routine` = P3; `Ord`
  with Immediate highest), `DecisionType` (closed enum, section 3.6), `PolicyVersion(i32)`, `ClockKey` and `SwitchKey`
  (closed registries, section 3.5).
- **Create/Update DTOs:** `CountryOverrideProposal { level: Option<RiskLevel>, reason, expires_at }` (`None` ends the
  override), `CalendarCreate { name, week }`, `CalendarWeekChange { week }`, `CalendarDaysChange { days: Vec<DayInput> }`
  (at most 400 rows), `DayInput { on_date, kind, name, attrs?, active? }`, `FxManualRate { currency, rate_date,
  rate_to_iqd, evidence_id }`, `ReasonCodeChange { code, family?, tag?, label?, template?, typology?, cbi_refs?,
  display_fields?, object_kinds?, active?, base_version? }` (batch: an array, at most 50 per request, one family),
  `PolicyPackProposal { document, reason_text, effective_at? }`, `AuthorityMatrixProposal { document, reason_text,
  effective_at? }`.
- **Ports** (domain traits; implementations in `db`, `worker` and `api`):

```rust
/// One day's CBI publication. The adapter reaches the publication host through the bank's proxy only (section 4).
#[async_trait]
pub trait FxRateSource: Send + Sync {
    async fn fetch(&self, rate_date: NaiveDate) -> Result<Vec<RawRate>, FxSourceError>;   // empty = not published yet
}
pub struct RawRate { pub currency: IsoCurrency, pub rate_to_iqd: Decimal, pub rate_to_usd: Option<Decimal> }

/// Business-day arithmetic (phase 00 `BusinessClock`). Phase 09 replaces the phase 00 `CalendarDaysClock` in the API
/// and the worker with `BankCalendarClock`; the phase 00 type stays for tests of the unbound case.
pub trait BusinessClock: Send + Sync {
    fn add(&self, from: DateTime<Utc>, n: u32, unit: BusinessUnit) -> Result<ClockResult, ClockError>;
}
pub struct ClockResult { pub fire_at: DateTime<Utc>, pub calendar_id: Uuid, pub calendar_version: i32,
                         pub horizon_exceeded: bool }

/// The authority matrix behind the phase 03 port (replaces `NoMatrix`).
pub trait DelegationAuthority: Send + Sync {
    fn check(&self, from: Uuid, to: Uuid, roles: &[RoleCode]) -> Result<(), DelegationRefusal>;
    fn window_limit_days(&self, roles: &[RoleCode]) -> Option<u32>;   // phase 03 section 7.6; None when no matrix is current (check then refuses)
}
pub enum DelegationRefusal { NoMatrix, NotDelegable { role: RoleCode } }

/// The policy pack's history input for the dry run. Empty until phase 17 (alerts) and phase 18 (cases) exist.
#[async_trait]
pub trait HistorySource: Send + Sync {
    async fn sample(&self, conn: &mut PgConnection, window_days: u32, max_rows: u32)
        -> Result<HistorySample, RepoError>;   // NoHistory in this phase: rows = 0, source = "none_until_phase_17"
}
```

- **Repository traits** (domain layer; `Pg*` implementations in `db`):

```rust
#[async_trait]
pub trait CountryRepository: Send + Sync {
    async fn list(&self, conn: &mut PgConnection, f: &CountryFilter, page: Keyset) -> Result<Page<CountryView>, RepoError>;
    async fn get(&self, conn: &mut PgConnection, iso2: &str) -> Result<Option<Country>, RepoError>;
    async fn open_override(&self, conn: &mut PgConnection, iso2: &str) -> Result<Option<CountryRiskOverride>, RepoError>;
    /// Ends the open row (ended_reason = superseded | reverted | expired) and returns it; no row -> None.
    async fn end_open_override(&self, tx: &mut Tx, iso2: &str, reason: EndReason, at: DateTime<Utc>)
        -> Result<Option<CountryRiskOverride>, RepoError>;
    async fn insert_override(&self, tx: &mut Tx, o: &NewOverride) -> Result<CountryRiskOverride, RepoError>;
    /// Open rows with expires_at <= now, locked SKIP LOCKED, for the sweep.
    async fn due_for_expiry(&self, tx: &mut Tx, now: DateTime<Utc>, limit: i64) -> Result<Vec<CountryRiskOverride>, RepoError>;
}

#[async_trait]
pub trait FxRepository: Send + Sync {
    /// INSERT ... ON CONFLICT (currency, rate_date, source) DO NOTHING; returns the rows written.
    async fn insert_cbi_batch(&self, tx: &mut Tx, rows: &[NewFxRate]) -> Result<u64, RepoError>;
    async fn insert_manual(&self, tx: &mut Tx, row: &NewFxRate) -> Result<FxRate, RepoError>;
    /// The rate for one business date; 'cbi' before 'manual' when both exist (cannot happen through the API).
    async fn on_date(&self, conn: &mut PgConnection, currency: &str, date: NaiveDate) -> Result<Option<FxRate>, RepoError>;
    /// The newest row on or before `date` no older than `max_days`, for labelled display only.
    async fn newest_within(&self, conn: &mut PgConnection, currency: &str, date: NaiveDate, max_days: u32)
        -> Result<Option<FxRate>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &FxFilter, page: Keyset) -> Result<Page<FxRate>, RepoError>;
    async fn dates_present(&self, conn: &mut PgConnection, currency: &str, from: NaiveDate, to: NaiveDate)
        -> Result<Vec<NaiveDate>, RepoError>;
}

#[async_trait]
pub trait CalendarRepository: Send + Sync {
    async fn list(&self, conn: &mut PgConnection, page: Keyset) -> Result<Page<Calendar>, RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<Calendar>, RepoError>;
    async fn default_calendar(&self, conn: &mut PgConnection) -> Result<Option<Calendar>, RepoError>;
    async fn insert(&self, tx: &mut Tx, c: &NewCalendar) -> Result<Calendar, RepoError>;
    /// `If-Match`: updates only when `version = expected`; bumps `version`.
    async fn update_week(&self, tx: &mut Tx, id: Uuid, week: &WeekendRule, expected: i32) -> Result<Option<Calendar>, RepoError>;
    /// Days in [from, to] of the calendar with `active = true` and a kind in `kinds`; the clock's input.
    async fn days_between(&self, conn: &mut PgConnection, id: Uuid, from: NaiveDate, to: NaiveDate, kinds: &[DayKind])
        -> Result<Vec<CalendarDay>, RepoError>;
    async fn list_days(&self, conn: &mut PgConnection, id: Uuid, f: &DayFilter, page: Keyset) -> Result<Page<CalendarDay>, RepoError>;
    /// INSERT ... ON CONFLICT (calendar_id, on_date, kind) DO UPDATE; one statement per batch.
    async fn upsert_days(&self, tx: &mut Tx, id: Uuid, days: &[NewCalendarDay], cr: Uuid) -> Result<u64, RepoError>;
}

#[async_trait]
pub trait CaptureRepository: Send + Sync {
    /// akr-ingestd only. INSERT ... (run_id, object_key, row_key); a repeated row in one run is an error.
    async fn insert_batch(&self, tx: &mut Tx, rows: &[NewCapture]) -> Result<u64, RepoError>;
    async fn latest_run(&self, conn: &mut PgConnection, source_id: Uuid) -> Result<Option<CaptureRun>, RepoError>;
    async fn rows(&self, conn: &mut PgConnection, run_id: Uuid, object: CaptureObject) -> Result<Vec<CbsCalendarCapture>, RepoError>;
}

#[async_trait]
pub trait ReasonCodeRepository: Send + Sync {
    async fn list(&self, conn: &mut PgConnection, f: &ReasonFilter, include_inactive: bool) -> Result<Vec<ReasonCode>, RepoError>;
    async fn get(&self, conn: &mut PgConnection, code: &str) -> Result<Option<ReasonCode>, RepoError>;
    async fn family_version(&self, conn: &mut PgConnection, family: ReasonFamily) -> Result<i32, RepoError>;   // MAX(version)
    /// Insert for a new code; update of the revisable columns for an existing one (`version + 1`).
    async fn apply(&self, tx: &mut Tx, changes: &[ReasonCodeChange], by: Uuid, cr: Uuid) -> Result<u64, RepoError>;
}

#[async_trait]
pub trait PolicyPackRepository: Send + Sync {
    async fn current(&self, conn: &mut PgConnection) -> Result<Option<PolicyPack>, RepoError>;   // max(version)
    async fn get(&self, conn: &mut PgConnection, version: i32) -> Result<Option<PolicyPack>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, page: Keyset) -> Result<Page<PolicyPackSummary>, RepoError>;
    async fn insert(&self, tx: &mut Tx, p: &NewPolicyPack) -> Result<PolicyPack, RepoError>;   // version = max + 1
}

#[async_trait]
pub trait AuthorityRepository: Send + Sync {
    async fn current(&self, conn: &mut PgConnection) -> Result<Option<AuthorityMatrixVersion>, RepoError>;
    async fn get(&self, conn: &mut PgConnection, version: i32) -> Result<Option<AuthorityMatrixVersion>, RepoError>;
    async fn insert(&self, tx: &mut Tx, m: &NewAuthorityMatrix) -> Result<AuthorityMatrixVersion, RepoError>;
}
```

- **Repository notes:** every list is keyset-paged (countries by `iso2`; days by `(on_date, kind, id)`; FX by
  `(rate_date DESC, currency, source)`; reason codes by `(family, code)`; policy packs and matrix versions by
  `version DESC`). `open_override` reads the partial unique index; `end_open_override` and `insert_override` run in one
  transaction under `FOR UPDATE` on the country row, so two approvals on one country serialise. `upsert_days` writes
  `source`, `change_request_id` and `version + 1` only on rows whose content changed (`IS DISTINCT FROM`), so an
  unchanged day keeps its version. `days_between` is one query over `uq_calendar_days` with the calendar id leading.
  `dates_present` is one query over `uq_fx_rates_day`. No repository method issues a delete.
- **utoipa schemas:** `CountryView` (iso2, name, base level, effective level, override block with expiry and reason
  when open, basis, version), `CountryOverrideAccepted` (change request id, `Location`), `FxRateView`,
  `FxManualRateRequest`, `FxGapsView` (currency, missing dates, count, `business_days_checked`), `CurrencyView`,
  `GovernorateView`, `CalendarView` (id, name, week, is_default, version), `CalendarCreateRequest`, `CalendarWeekRequest`,
  `CalendarDayView` (on_date, kind, name, attrs, hijri label with variant, active, source), `CalendarDaysRequest`,
  `BusinessDayView` (`business_date`, `fire_at_bank_time`, `calendar_version`, `horizon_exceeded`), `ReconciliationView`
  (section 3.3), `ReasonCodeView` (template only for readers of section 3.8), `ReasonCodeChangeRequest`,
  `PolicyPackView` (version, document, sha256, origin, approved_by display name, effective_from), `PolicyPackSummary`,
  `PolicyPackProposal`, `PolicyDryRunView` (clock changes, template hits as counts, history source, rows read),
  `AuthorityMatrixView`, `AuthorityMatrixProposal`.

- **Route table** (base `/api/v1`; 21 operations; scopes from API.md section 2; Cedar actions in section 3.8; every
  mutation takes `Idempotency-Key`; **CR** = a change request routed by phase 04, answered 202 with `Location`):

| Method | Path | Scope | Description |
|-|-|-|-|
| GET | /ref/countries | self | Countries with base level, effective level, open override and basis; `risk?`, `q?` (`country.list`) |
| PUT | /ref/countries/{iso2}/override | config.author | Propose a bank override (`level`, `reason`, `expires_at`; `level` null ends it); **CR** `country_override` (route by direction); `If-Match` on the country version; 202 (`country_override.propose`) |
| GET | /ref/fx-rates | self | Rates by date: `currency?`, `source?`, `from?`, `to?`, cursor (`fx_rate.list`) |
| POST | /ref/fx-rates | data.steward | Record a rate when the CBI feed has none for the day; `evidence_id` (kind `upload`); 201 (`fx_rate.record`) |
| GET | /ref/fx-rates/gaps | data.steward | Business dates in `from..to` (default calendar, working days) with no CBI or manual rate for `currency` (`fx_rate.gaps`) |
| GET | /ref/currencies | self | ISO currencies with minor unit and active flag (`currency.list`) |
| GET | /ref/governorates | self | Iraqi governorates with region (`governorate.list`) |
| GET | /ref/calendars | self | Calendars with the default flag, week and version (`calendar.list`) |
| POST | /ref/calendars | config.author | Propose a new calendar (name, week); **CR** `calendar`; 202 (`calendar.propose`) |
| PUT | /ref/calendars/{id}/week | config.author | Propose a change of the weekend, per currency where it differs; **CR** `calendar`; `If-Match`; 202 (`calendar.propose`) |
| GET | /ref/calendars/{id}/days | self | Days in `from..to` with kind, names, Hijri label and variant, active; `kind?` (`calendar.read`) |
| POST | /ref/calendars/{id}/days | config.author | Propose adds and changes of days (at most 400); **CR** `calendar_day` (material); `If-Match` on the calendar version; 202 (`calendar_day.propose`) |
| GET | /ref/calendars/{id}/business-days | self | The business date reached after `n` units from `from`; the clock preview used by the editor and the dry run (`calendar.read`) |
| GET | /ref/calendars/{id}/reconciliation | data.steward | Differences between the latest captured CBS calendar and this calendar (`calendar.reconcile`) |
| GET | /ref/reason-codes | self | Reason codes by family; `family?`, `include_inactive` (config readers only); `template` shown to config readers only (`reason_code.list`) |
| POST | /ref/reason-codes | config.author | Propose add or revise of up to 50 codes of one family; **CR** `reason_code` (family-level request); 202 (`reason_code.propose`) |
| GET | /policy-packs | config.author, config.approve, dashboard.read | Policy pack versions, newest first (`policy_pack.list`) |
| GET | /policy-packs/{id} | config.author, config.approve, dashboard.read | One version with its document; readers of section 3.8 only, others get the masked 404 (`policy_pack.read`) |
| PUT | /policy-packs/active | config.author | Propose the next policy pack version with the dry-run evidence; **CR** `policy_pack` (material); `If-Match` on the active version; 202 (`policy_pack.propose`) |
| GET | /org/authority-matrix | self | Current matrix, or `?version=n`: grants by decision type and delegation rules (`authority_matrix.read`) |
| PUT | /org/authority-matrix | admin.users | Propose a new matrix; **CR** `authority_matrix` (route `cr_authority`: MLRO signs, CHK approves); `If-Match` on the active version; 202 (`authority_matrix.propose`) |

- **Router functions:** `reference::router() -> OpenApiRouter<AppState>` (16 operations), `policy::router()` (3),
  `authority::router()` (2), each merged in `build_app` under the authz layer. `PUT /policy-packs/active` is a literal
  route, so it never collides with `GET /policy-packs/{id}`.

## 3. Business Logic

### 3.1 Countries, overrides, currencies and governorates (`ReferenceService`; W-19, K-24, TR-10)

Base levels come from a signed reference release, never from an edit: `akr-ctl ref seed` (section 4) installs
`countries`, `governorates` and `currencies` from `backend/packs/reference/*.json`, each with its `release_id` and the
SHA-256 listed in the release manifest. A release that changes a base level is the trigger for re-scoring (TR-10): the
seed writes the outbox event `ref.country_level_changed {iso2, from, to}`, and so does every override activation, expiry
and revert (section 3.1); the phase 15 rating engine consumes it and re-scores the affected customers, so no rating changes
silently. Until phase 15 the event has no consumer and is kept by the outbox.

`PUT /ref/countries/{iso2}/override` (`ReferenceService::propose_override`):
1. Cedar `country_override.propose` (CFG). Scope `config.author` is checked first (gate 1).
2. Load the country: missing -> 404 `E_NOT_FOUND`. `If-Match` required (428 `E_PRECONDITION_REQUIRED`) and equal to
   `countries.version` (409 `E_VERSION_CONFLICT`).
3. Validate: `level` null needs an open override (422 `E_VALIDATION`, `no_override_to_end`); `level` equal to the
   effective level (409 `E_NOT_ALLOWED_IN_STATE`, `no_change`); `expires_at` after now (422 `expiry_in_past`) and no
   later than now plus `country.override_max_days` (422 `expiry_too_far`).
4. `direction` = `raise` when the new level is above the effective level, else `lower`.
5. `ChangeRequestService::create(Dedicated, submit = true)` with `object_kind = country_override` (section 3.7). The
   route is chosen by the route map: a raise is `cr_minor`; a lower is `cr_material` (volume effect `decrease`, phase 04
   rule 2). Answer 202 with `Location`.

Apply hook (`CountryOverrideTarget::activate`, in the approval transaction, phase 04):
1. `end_open_override(iso2, superseded)` when an override is open; the new row takes its place.
2. `insert_override` with `applied_by` = the final approver and `change_request_id`; a level null only ends step 1's row.
3. Audit `country_override.applied` (section 3.8) and the outbox event `ref.country_level_changed` when the effective
   level moves (TR-10; consumed by phase 15).

Emergency (phase 04 DR-55): allowed only for a `raise` with reason `CHG_SANCTIONS_LIST` or `CHG_REGULATORY_ORDER`; the
target's `revert` ends the new row (`reverted`) and, if the row it replaced was still within its expiry, inserts a fresh
row with that level and the old expiry. The revert never leaves a lower level in force than the one before the change.

Expiry (`ref_expiry_sweep`, section 4): every open row with `expires_at <= now` is ended (`expired`) under
`FOR UPDATE SKIP LOCKED`. The effective level falls back to the base level, not to an older override: an override that
should outlive its expiry needs a new change request. Audit `country_override.ended {reason: expired}` and the outbox event `ref.country_level_changed`.

`effective_level(iso2, at)` is the open override when it has not expired at `at`, else `risk_level`. Ratings read it
through `CountryView`; the phase 15 rating model reads the same function.

### 3.2 FX rates (`FxRateService` in `ReferenceService`; RP-50, DR-39)

1. **Import** (`fx_rate_import`, section 4): when the flag `fx.cbi_feed_enabled` is off the job completes with
   `skipped: feed_disabled` and writes no audit row. When on: `FxRateSource::fetch(rate_date)`. An empty answer fails the
   attempt with `no_rates_published` (the job retries with back-off, up to five attempts).
2. Validate each row: the currency is an active ISO currency; `rate_to_iqd > 0`; `rate_to_usd` taken from the feed or
   derived as `rate_to_iqd(currency) / rate_to_iqd(USD)` from the same batch; a row whose currency is unknown is refused
   and counted, never stored under a default. IQD is stored with rate 1 (CHECK in section 1).
3. `insert_cbi_batch` in one transaction; audit one `fx_rate.imported {rate_date, rows, refused, source: cbi}` per run.
4. `POST /ref/fx-rates` (data.steward, Cedar `fx_rate.record`): the evidence object exists, has kind `upload` and owner
   kind `fx_rate` (phase 02), and is not shredded (else 422 `E_VALIDATION`, `evidence_unknown`). A `cbi` row for the same
   currency and date exists -> 409 `E_NOT_ALLOWED_IN_STATE`, `cbi_rate_exists`: a manual rate never replaces the CBI
   rate. Insert with `source = manual`, `recorded_by`; 201; audit `fx_rate.recorded`.
5. `rate_on(currency, date)`: the exact business date only, `cbi` before `manual`. Missing -> `FxRateMissing`. Reporting
   (phase 21 CTR and any valued amount) treats it as "wait and raise a task"; a request that would convert with a missing
   rate answers 409 `E_FX_RATE_MISSING` with `{currency, date}` in the body. Nothing falls back silently (M8 edge case).
6. `newest_within(currency, date, fx.rate_max_age_days)`: the nearest earlier row, returned with `age_days` and
   `stale = true`, for labelled display in screens only. It is never written into an amount or a `rate_date` column.
7. `gaps(currency, from, to)` (`GET /ref/fx-rates/gaps`): the business dates of the default calendar in the range (working
   days only, since the CBI publishes on its working days) that have no row for the currency; at most 366 dates per call.

### 3.3 Calendars, the business clock and the CBS reconciliation (`CalendarService`; S-13, T-04, T-10, M-23, RP-46, TS-40, DR-39, UX-22)

**Capture (akr-ingestd, inside phase 05's `discovery_run`).** The adapter pack's hints gain a `calendar` block naming
two approved objects of the ICS BANKS profile: the holiday table (`HOLIDAY`: `CUR_CODE`, `FROM_DATE`, `FST_WORK_DATE`) and
the currency table (`CURRENCY`: internal code, ISO alpha as read, `WEEK_END`, `WEEK_END_DAYS`, `DEC_PLA`). The capture
reads each whole with the row cap `calendar.capture_max_rows` (reaching the cap is the finding
`calendar_capture_truncated`, and the run is not sealed as complete), with `SelectProof` (phase 05) on each statement and
`E_OBJECT_NOT_APPROVED` for an object outside the approved list. Rows go to `ref.cbs_calendar_captures` in the run's
transaction. Nothing from a customer, account or posting table is read.

**Reconciliation** (`CalendarReconciler`, pure; `GET /ref/calendars/{id}/reconciliation`):
1. Take the latest completed run of the source (`CaptureRepository::latest_run`); no run -> `ReconciliationView` with
   `status: no_capture` (never an empty "all matches").
2. Map each CBS currency code to ISO through the bank-confirmed code map of kind `currency` (phase 10). Until phase 10
   exists, only the IQD internal code confirmed at discovery is mapped (DATA-MODEL s.0); every other code is listed as
   `currencies_unmapped`, and its holidays are not compared.
3. For the mapped holidays: `cbs_only` (CBS holiday with no active `holiday` row in the calendar), `akr_only` (active
   calendar holiday, kinds `holiday`, `eid`, `arbaeen`, in the range the CBS table covers, with no CBS row).
4. Weekend: for each mapped currency, the CBS week-end days against `week.weekend[currency]` (or `default`):
   `weekend_differences`. The CBS weekend is read, never assumed: the calendar's default weekend is written from the
   captured data only at a steward's proposal (section 3.3, calendar changes).
5. The view never writes. A difference becomes a change request only when the steward proposes it (`POST .../days`).

**The business clock** (`BankCalendarClock`, pure over a day set; implements `BusinessClock`):
1. Input: `from` (UTC), `n` (1 to 3660), `unit`.
2. `hours` and `days`: plain addition (phase 04 rule); no calendar is read; `horizon_exceeded = false`.
3. `working_days`: convert `from` to the bank zone (`bank.timezone`, phase 00); keep the local date `d0` and the local
   time of day `t0`.
4. Load the default calendar's `week`, its closed days (kinds `holiday`, `eid`, `arbaeen`, active) from `d0` to `d0 + 3n +
   14` days, and `covered_until`, the latest active row date of any kind in the calendar. The closed days form a set. A
   working week of the default pattern has five working days, so `n` working days fit in that span with a fortnight of
   margin for closed days.
5. Walk forward one local date at a time from `d0` (the day of `from` itself is never counted): a date is working when
   its ISO weekday is not in `week.weekend.default` and it is not in the closed-day set. Count the working dates until
   `n` are reached.
6. The result is `d` at `t0` in the bank zone, converted to UTC. The time of day never moves; a `from` on a closed day
   counts from the next day.
7. If the walk passes `covered_until` before `n` is reached, the weekend pattern alone decides the remaining dates and
   `horizon_exceeded = true`. The clock still returns a time; the caller records the warning, and the sweep
   (section 4) reports the shortfall.
8. `ClockResult` carries `calendar_id` and `calendar_version`. The phase 00 timer stores both in its payload, so a later
   calendar change never moves a clock that is already running (drained, not migrated, as for the policy pack).
9. Hijri is never an input. The clock module imports no Hijri type, and a CI test greps it for the Hijri formatter
   (UX-22). Hijri labels are display text only.

Worked example for the test suite: from Thursday 2026-10-08 at 10:00 Baghdad time, two working days give Monday
2026-10-12 at 10:00, because Friday and Saturday are the default weekend (ISO 5 and 6) and Sunday is a working day.

**Calendar changes** (change targets `calendar` and `calendar_day`, section 3.7; base class `material` for both):
- `calendar.propose` (create, week): the weekend lists are subsets of `1..=7` of size 1 to 3; the week leaves at least three
  working days; a currency key names a currency in `ref.currencies`. Activation inserts or updates the row and audits
  `calendar.created` or `calendar.week_changed`. The default calendar is set by the seed only; a second default is
  refused by the unique index.
- `calendar_day.propose` (add or change days, at most 400 per request): each row has a unique `(on_date, kind)` within
  the request; `ramadan_hours` needs `from < to` in `HH:MM`; the names are both languages; the Hijri label is computed
  by the server with the Islamic-calendar variant the stack provides and stored in `hijri_label` with its variant name inside the label text (no separate column). Prepare computes
  `impact`: the number of open timers (`fired_at` and `cancelled_at` null) whose `fire_at` falls on a changed date, and
  whether a closing kind changed. Activation upserts the rows (`source = manual` for a steward's entry,
  `cbs_reconciled` when the request names the reconciliation, `change_request_id` set) and audits
  `calendar_day.changed`. Deactivation (`active = false`) is the only way a day leaves the clock; the row stays.

**Business-day preview** (`GET /ref/calendars/{id}/business-days?from=&n=&unit=`): the same function as the clock, with
the calendar given in the path, returning `business_date`, `fire_at_bank_time`, `calendar_version` and
`horizon_exceeded`. It is what the policy-pack editor and the dry run call.

### 3.4 Reason-code families beyond change and tuning (`ReferenceService`; M-27, T-44, IR-31, UX-65, GV-44)

Phase 04 seeded `change` and `tuning`. This phase administers the other five families of the phase 04 CHECK:
`detection` (the text of a detection reason, with a bilingual template), `decision` (the coded reasons of a decision form),
`closure` (the codes of a closed alert or case), `override` (the codes of an analyst override of a score or rating) and
`exclusion` (the codes of an exclusion register entry, phase 12).

- **Codes:** the prefixes `DET_`, `DEC_`, `CLS_`, `OVR_` and `EXC_` match the family. The code is immutable; a family
  change is a new code. `tag` is allowed for `tuning` and `change` only (phase 04 CHECK).
- **Seed:** `backend/packs/reference/reason-codes.v1.json` holds the drafted families. `akr-ctl ref seed` inserts them
  with `active = false`: a code is visible to the steward but usable on no form until a change request activates it.
  Activation is a family-level request (section 3.7). No code reaches a decision form unreviewed.
- **Template rules** (`ReasonCode::validate_template`, pure): the template exists only for `detection`; both
  languages; placeholders from the allow-list `{scenario_name}`, `{channel}`, `{period_label}` only; no digit outside a
  placeholder (a figure would be a threshold, and the liaison sees reason codes without thresholds, M-14); the folded text
  passes the forbidden-word scanner of section 3.5 for scope `reason_text` (a generic phrase such as "no suspicion" alone is
  a forbidden term of this scope, IR-31) and the neutral-wording rule of `notification` where the family is shown to
  branch roles.
- **Labels and typology:** labels in both languages; `typology` at most 40 characters; `cbi_refs` each of the form
  `Art.N no.M` and present in the catalogue of phase 06 when that catalogue is installed (422 `unknown_cbi_ref`
  otherwise).
- **Revision:** `base_version` is the family's `MAX(version)`; a stale base -> 409 `E_STALE_BASE` (phase 04). Activation
  writes `version + 1` per changed code and audits `reason_code.changed {code, family, version, active}` for each.
- **Reads:** `GET /ref/reason-codes` returns labels to everyone with `self`; `template` and inactive codes only to
  readers with `config.author` or `config.approve`.

### 3.5 The policy pack (`PolicyPackService`; M-75, M-14, M-32, M-42, RP-62, GV-44, SP-31, UX-68, IR-56)

**What a pack holds.** Routing (the queues of phase 17 and the branch-first switch of M-14, referencing pools and queues
that exist), clocks (the bank-tunable durations of the workflow clocks, within registry bounds), the calendar reference
(`calendar_id`, which must exist), the delegated-authority pin (`authority_matrix_version`, which must exist), the reason
codes enabled per family (references only), the templates (bilingual neutral texts for tasks, notices, RFI and branch
forms), the switches (closed registry), the forbidden-word list (closed structure, section below) and the four priority
labels of class P0 to P3 (DESIGN-SYSTEM 7.5). The document is closed: `policy-pack.v1` has exactly these keys.

**Closed registries** (in code, `ClockRegistry`, `SwitchRegistry`; each entry has its shipped value, bounds and whether it
is `fixed`):
- Clocks: `rfi.response` (working days, default 10, bounds 5 to 20, M-44 default), `ops.task.due` (working days, bounds
  1 to 10), `case.review` (days, bounds 30 to 365). A clock with `fixed: true` (a CBI-set deadline, for example the
  seven-working-day refrain clock, API.md 3.14) is listed in the registry with its shipped value; the document must carry
  that value for it, and any other value is refused with `E_INVARIANT_LOCKED`.
- Switches: `tipping_off_flag_default` (boolean, shipped `true`, `fixed: true`), `branch_first_routing` (boolean,
  shipped `false` and `fixed: true` in this release: the five conditions of M-14 that turning it on needs are P2, section 7+),
  `rfi.pre_fill` (boolean, `false`).
- Anything not in a registry cannot be set by a pack. An "invariant" is therefore a registry entry with `fixed: true`, not
  a convention.

**Forbidden-word list.** `forbidden_words: {version, terms: [{id, en, ar?, match: word | phrase, scopes: [...]}]}`.
Scopes: `customer_notice`, `notification`, `task`, `branch_form`, `rfi`, `reason_text`. A term with `ar: null` is enforced
in English and raises the warning `ar_term_unreviewed` on the proposal; the release does not ship Arabic terms that a
person has not reviewed (open doubt). The English seeds are the neutral-surface words of DESIGN-SYSTEM section 11 (Wording) (the
list of "suspicion", "suspicious", "STR", "UTR", "money laundering", "terrorist", "freeze reason", "sanction") plus the
generic phrase "no suspicion" of IR-31.

**`ForbiddenWordScanner`** (pure; no IO):
1. Fold the text with `text::fold` (diacritics and tatweel removed; alef forms, ya and ta marbuta unified; Western digits)
   and lower-case Latin letters; replace every run of punctuation and white space with one space.
2. `word` terms match on token boundaries on both sides; `phrase` terms match a token sequence. A match inside a longer
   token never counts (so "strategy" does not match "str").
3. The scanner returns `Hit { term_id, start, end }` (the span in the folded text, mapped back to the author's text).
   It never returns the term, and no response contains it. The author sees the span and the field.
4. A text is scanned for the scopes it appears in. Templates are scanned for every scope they are used in (`customer_notice`
   templates are the strictest). The RFI free text (phase 19) and the decision forms (phase 18) call the same scanner
   through `PolicyPackService::scan(scope, text)` at save time (M-42, UX-68, IR-56), always against the active pack version, whatever version the case pinned, so a new term binds at once; the HTTP API has no scan route.

**Document validation** (`PolicyValidator::validate(doc, current, registries, settings) -> Result<Validated, Vec<ValidationIssue>>`,
pure except for the reference checks it receives as data):
1. Schema `policy-pack.v1`: closed keys, types, lengths. An unknown key -> `E_VALIDATION`, `unknown_key`.
2. Clocks: each key in the registry; the value within bounds; a `fixed` clock with a different value -> `E_INVARIANT_LOCKED`;
   a value looser than the floor in settings where one exists -> `E_BELOW_FLOOR`.
3. Switches: each key in the registry; a `fixed` switch with a different value -> `E_INVARIANT_LOCKED`.
4. References: `calendar_id` exists and is not a fixed clock; every queue and pool named exists and is not archived
   (phase 01 pools); every reason code named is active; `authority_matrix_version` exists.
5. Templates: unique keys; both languages; placeholders from the allow-list; no digit outside a placeholder; the scanner
   for each declared scope -> `forbidden_term` with `{pointer, start, end}` and never the term; no `customer_notice`
   template containing a reason word of the forbidden list's `customer_notice` scope.
6. Forbidden-word list: unique term ids; `ar` warning as above; the list is never empty.
7. Priority labels: four keys, both languages.
8. Canonical JSON (RFC 8785 rules: sorted keys, no insignificant space) gives the content hash; an identical hash to the
   current version -> 409 `E_NOT_ALLOWED_IN_STATE`, `no_change`.
9. `changed_keys`: the clock, switch, template, term and routing keys whose value differs from the current version.

**Dry run** (`PolicyDryRun::run(current, proposed, history, window, max_rows)`, pure over `HistorySample`; synchronous):
1. Estimate rows: `history.sample(window_days, max_rows + 1)`; more than `policy.dry_run_max_rows` -> `E_COST_GUARD`
   with the estimate in the body (API.md section 6). In this phase the sample is always empty and the source is
   `none_until_phase_17`, which the view states.
2. Clock effects: for each changed clock, the number of open timers whose payload names the clock key (phase 00 timers
   payload `clock`), counted once for submit and again at decision time.
3. Template effects: each active template and each changed template scanned under the proposed list; the counts of hits
   per scope (never the text).
4. Routing and reason-code effects as counts of changed keys.
5. The report is `generated` evidence `policy_dry_run` (phase 04 EvidenceResolver, section 4), regenerated at every
   deciding action so it is never stale (phase 04 DR-58 pattern).

**Activation** (`PolicyPackTarget::activate`, in the approval transaction): `PolicyPackRepository::insert` with
`version = current + 1`, `document`, `content_sha256`, `change_request_id`, `approved_by` (the final approver),
`effective_from = now()` (a future `effective_at` is a `cr_activate` timer, phase 04), then audit `policy_pack.activated`.
Cases pin the version they open under (phase 18); the new version is used only by cases opened after `effective_from`, so
nothing migrates (M-75); the one exception is the save-time scan of forbidden terms (`PolicyPackService::scan`), which
always reads the active version. A rollback is a new change request; there is no revert hook (`revert` is not allowed).

**Bootstrap** (`akr-ctl policy-pack seed`): installs version 1 from `backend/packs/policy/policy-pack.v1.json` with
`release_id` and no change request, audit `policy_pack.seeded {version: 1, sha256}`. A person's authority is never created
by a release file, so the bootstrap is a product default that the bank's compliance team reviews before the first install,
and every later version goes through change control.

### 3.6 The delegated-authority matrix (`AuthorityMatrixService`, `AuthorityDelegation`; M-20, M2; never-delegated roles per DATA-MODEL s.1)

**Closed decision types** (`DecisionType`, the vocabulary the decision services of phases 17 to 21 call; each maps to one
API.md scope and to the roles that hold it, from API.md section 2):

| Key | Scope | Roles holding the scope |
|-|-|-|
| `alert.disposition` | queue.work | A1, A2, M |
| `case.decision` | case.decide | A2, M |
| `freeze.decision` | case.decide | A2, M |
| `screening.decision` | screening.work | SS, M |
| `kyc.decision` | kyc.work | KYC, M |
| `rfi.release` | rfi.desk | RFI, M |
| `report.file` | report.file | M |
| `qa.review` | qa.review | QA, M, CH, IA (widened by phase 19) |
| `ops.verify` | ops.tasks | OPS |

A new decision type is a code change released with the phase that needs it, never a document entry. The matrix validator
refuses a grant whose role does not hold the type's scope (422 `role_lacks_scope`), so an analyst cannot be granted a
decision the Cedar bundle would refuse them.

**Document** (`authority-matrix.v1`, closed): `grants: [{decision_type, role, max_priority, second_review}]` and
`delegation: [{role, delegable, max_days}]`.

**Validator** (pure): role codes are among the 19 of MODULES s.0; `ADM`, `SEC`, `IA`, `AS` and `VS` are never delegable
(`delegable: true` -> 422 `never_delegable`; the phase 01 CHECK is the second layer); `max_days` in `1..=delegation.max_days`
(phase 03); every closed decision type has at least one grant (422 `decision_type_unassigned`); `max_priority` is one of
the four classes; `second_review` is a boolean (the bank may choose blanket second review instead, M-20).

**Permit** (`AuthorityMatrix::permits(roles, decision_type, priority) -> Permit | Refused`): a grant with a role in `roles`
for the type and `max_priority >= priority` in the `Priority` order. The result carries `second_review`. Phases 17 to 21
call it in their decision services; Cedar still decides the scope. In this phase it is implemented and tested, and wired
only to the delegation check.

**`AuthorityDelegation`** (implements the phase 03 `DelegationAuthority` port; replaces `NoMatrix`):
1. No current matrix -> `DelegationRefusal::NoMatrix`. Delegation is refused until the first matrix is signed (fail closed;
   section 6 states the install consequence).
2. `check(from, to, roles)`: every role in `roles` has a `delegation` row with `delegable: true`; otherwise
   `NotDelegable { role }`.
3. `window_limit_days(roles)`: the smallest `max_days` of the roles. Phase 03's `DelegationService` caps the window at
   `min(delegation.max_days, window_limit_days)` (phase 03 update, section 4).

**Propose and sign** (`PUT /org/authority-matrix`):
1. Cedar `authority_matrix.propose` (ADM only, scope `admin.users`). `If-Match` on the active version (428, 409).
2. Validate (above), then `ChangeRequestService::create` with `object_kind = authority_matrix`, `Dedicated`, submit. The
   route map chooses `cr_authority`: a `sign` step for the MLRO named officer with step-up class `signing`, then an
   `approve` by role CHK, escalated to CH after three business days (section 4, shipped family). The maker cannot sign or
   approve (four-eyes, phase 04).
3. Activation (`AuthorityMatrixTarget::activate`): `insert` with `version = current + 1`, `signed_by` = the `sign` step's
   actor from the instance's `action_taken`, `stepup_id` from that step, `content_sha256` over canonical JSON,
   `effective_from = now()`; audit `authority_matrix.activated`.
4. The matrix is printed in the assessor pack (M-20) through phase 24; this phase stores it and serves it.

### 3.7 Change targets registered with phase 04 (`ChangeTargetRegistry`)

Every object of this phase that is governed registers one `ChangeTarget` (phase 04 port). The class is the base class the
target declares; the phase 04 classifier raises it (a loosening is `material`, a `decrease`). Each target declares its
inventory kind, so the phase 04 GV-01 gate applies: the bank registers the item (`POST /inventory`, phase 04) and a
validator approves it before the first change of that kind; the release does not approve them.

| Kind | Route (route map) | `object_key` | Base version | Base class | Inventory kind | Emergency | Revert | Closed |
|-|-|-|-|-|-|-|-|-|
| `country_override` | `cr_minor` for a raise, `cr_material` for a lower (rule 2) | `country:<iso2>` | country version | minor | `reference_list` | raise with a sanctions or regulatory reason only | end the new row, reinstate a still-valid predecessor | nothing |
| `calendar` | `cr_material` (class fallback) | `calendar:<id>` or `calendar:new:<uuid>` | calendar version (0 for new) | material | `reference_list` | no | not allowed | nothing |
| `calendar_day` | `cr_material` (class fallback) | `calendar:<id>:days` | calendar version | material | `reference_list` | no | not allowed | nothing |
| `reason_code` | `cr_material` (class fallback) | `reason_code_family:<family>` | family version (`MAX(version)`) | material | `reference_list` | no | not allowed | nothing |
| `policy_pack` | `cr_material` (class fallback) | `policy_pack:active` | active version | material | `matcher` (the forbidden-word scanner and template rules) | no | not allowed (a rollback is a new change) | nothing |
| `authority_matrix` | `cr_authority` (new route rule, section 4) | `authority_matrix:active` | active version | material | none (the matrix is the authority itself; the signature is the control) | no | not allowed | nothing |

`reason_families()` is `["change"]` for every target (a change to the decision vocabulary is a change, and phase 04 requires
a reason of a family the target accepts). `generic_create()` is false for all six: each has its dedicated route. `prepare`
runs the validators above and the dry run for `policy_pack`; `current_version` reads the version named in the table;
`check_before_decision` re-runs the validator against the current version, so a version that moved is caught at approval
(409 `E_STALE_BASE`). `diff` returns the `fields` format for scalar changes and the `json_paths` format for policy and
matrix documents.

### 3.8 Errors, audit events, Cedar actions, tipping-off review, settings

**Error mapping** (`thiserror` enums `RefError`, `PolicyError`, `AuthorityError`, `DelegationRefusal`; mapped in `api::error`):

| Condition | HTTP | `code` and reason |
|-|-|-|
| Unknown country, currency, calendar, code, version or matrix | 404 | `E_NOT_FOUND` (identical body for a masked policy pack) |
| `If-Match` missing or stale | 428 or 409 | `E_PRECONDITION_REQUIRED` or `E_VERSION_CONFLICT` |
| Base version moved under a change request | 409 | `E_STALE_BASE` (phase 04) |
| No override to end, no change, `cbi_rate_exists`, `no_matrix`, `route_unavailable` | 409 | `E_NOT_ALLOWED_IN_STATE` with the reason |
| No CBI or manual rate for a converted amount | 409 | `E_FX_RATE_MISSING` (new, section 4) with `{currency, date}` |
| Document, template, switch or expiry validation | 422 | `E_VALIDATION` with `errors[]`: `unknown_key`, `expiry_in_past`, `expiry_too_far`, `template_has_figures`, `forbidden_term` (span only), `unknown_cbi_ref`, `never_delegable`, `decision_type_unassigned`, `role_lacks_scope`, `calendar_fixed` |
| A fixed clock or switch changed | 422 | `E_INVARIANT_LOCKED` (new, section 4) |
| A clock looser than its floor | 422 | `E_BELOW_FLOOR` (phase 00 floors) |
| Dry run above the row cap | 422 | `E_COST_GUARD` with the estimate |
| Delegation refused by the matrix | 422 | `E_VALIDATION` at `/roles` with `not_delegable` or `no_matrix` (phase 03 delegation route) |
| Approval without the MLRO's step-up | 401 | `E_STEPUP_REQUIRED` (class `signing`) |

**Audit events emitted** (the change request's own events, `change_request.created` and `.submitted`, are phase 04's;
every mutation that changes state names its event here):

| Event kind | On | Resource |
|-|-|-|
| `country_override.applied` | CR activation (iso2, level, direction, expires_at, previous level, CR id) | country |
| `country_override.ended` | supersede, revert or expiry sweep (reason) | country |
| `fx_rate.imported` | job run, once per date (rows, refused, source) | fx_rate |
| `fx_rate.recorded` | manual rate (currency, date, evidence id) | fx_rate |
| `calendar.created`, `calendar.week_changed` | CR activation | calendar |
| `calendar_day.changed` | CR activation (count, date span, kinds, CR id) | calendar |
| `reason_code.changed` | CR activation, per code (code, family, version, active) | reason_code |
| `reason_codes.seeded` | `ref seed` (codes inserted inactive) | system |
| `ref.seeded` | `ref seed` (countries, governorates, currencies, default calendar; release id, counts) | system |
| `policy_pack.seeded` | `policy-pack seed` (version 1, sha256) | system |
| `policy_pack.activated` | CR activation (version, sha256, changed keys, CR id) | policy_pack |
| `authority_matrix.activated` | CR activation (version, sha256, signed_by, stepup id, CR id) | authority_matrix |
| `policy_pack.read_denied` | masked read (the access event of phase 03 is written too) | policy_pack |

Reused: `change_request.*`, `setting.updated`, `evidence.stored` (phase 02), `authz.denied` (phase 03), `delegation.created`
(phase 03, now with the matrix window).

**Cedar** (bundle files `C0_reference.cedar` for reference data and reason codes, `C0_policy.cedar` for the policy pack and
the matrix, both in the governance range of phase 03; phase 06 uses `C0_packs.cedar` for pack downloads, which this phase
does not touch). Entities added to `schema.cedarschema`: `Country { risk_level: String, has_override: Bool }`, `Calendar
{ is_default: Bool }`, `CalendarDay { kind: String }`, `ReasonCode { family: String, active: Bool }`, `PolicyPack { version:
Long, visibility: String }`, `AuthorityMatrix { version: Long }`.

| Action | Resource | Who (shipped bundle) |
|-|-|-|
| `country.list`, `currency.list`, `governorate.list`, `calendar.list`, `calendar.read`, `authority_matrix.read` | System, Calendar, AuthorityMatrix | every active human |
| `country_override.propose`, `calendar.propose`, `calendar_day.propose`, `reason_code.propose`, `policy_pack.propose` | Country, Calendar, CalendarDay, ReasonCode, PolicyPack | CFG (maker) |
| `fx_rate.record`, `fx_rate.gaps`, `calendar.reconcile` | System, FxRate, Calendar | DS |
| `fx_rate.list` | System | DS, CFG, CHK, M, CH, IA |
| `reason_code.list` | System, ReasonCode | every active human (the template is withheld by the service, section 3.4) |
| `policy_pack.list` | System | CFG, CHK, M, CH, IA |
| `policy_pack.read` | PolicyPack | CFG, CHK, M, CH, IA; all others masked (`@deny("masked")`, 404) |
| `authority_matrix.propose` | AuthorityMatrix | ADM |

```cedar
// C0_policy.cedar (excerpt)
@id("policy_pack.readers") @role("CFG,CHK,M,CH,IA")
permit (principal, action in [Akr::Action::"policy_pack.list", Akr::Action::"policy_pack.read"], resource)
when { principal.roles.containsAny(["CFG", "CHK", "M", "CH", "IA"]) };

// The document holds the forbidden-word list and the templates. Other readers get the same 404 as a missing version.
@id("policy_pack.masked") @deny("masked")
forbid (principal, action == Akr::Action::"policy_pack.read", resource is Akr::PolicyPack)
when { !principal.roles.containsAny(["CFG", "CHK", "M", "CH", "IA"]) };

@id("policy_pack.author") @role("CFG")
permit (principal, action == Akr::Action::"policy_pack.propose", resource is Akr::PolicyPack)
when { principal.roles.contains("CFG") };

@id("authority.propose") @role("ADM")
permit (principal, action == Akr::Action::"authority_matrix.propose", resource is Akr::AuthorityMatrix)
when { principal.roles.contains("ADM") };
```

Attributes evaluated: `principal.roles`, `principal.kind`, `principal.status`; `resource.visibility` (PolicyPack),
`resource.is_default`, `resource.kind` (CalendarDay). The masked forbid writes the phase 03 `deny` access event and answers
404. Bundle tests: DS and MRO receive the masked 404 on `GET /policy-packs/{id}` and the 403 `E_FORBIDDEN` on the list; only
CFG proposes calendars and reason codes; ADM proposes no calendar; no role but ADM proposes a matrix; CH and IA read the
pack and never propose.

**Tipping-off review.**
- The policy pack holds the forbidden-word list (what the bank avoids in outgoing and branch-facing text) and the templates.
  Both are readable only by compliance and oversight roles. The 404 for everyone else is the same body as a missing version,
  so the existence of a pack is not a signal.
- Customer-facing templates (`customer_notice`) are scanned at save; a template that reveals suspicion or an STR is refused
  (GV-44). Branch, ops and notification templates are scanned for the scopes they appear in (SP-31, RP-62).
- Reason codes: the labels are shown to the liaison and branch roles and contain no threshold (M-14); the detection templates
  are withheld from non-config readers; no label or template contains a customer, account or amount, and the scanner refuses a
  figure in a template (section 3.4).
- The calendar and the rates carry no customer data. The authority matrix names decision types and roles, not cases; it is
  readable by every active human, so it must never gain a case or party field (a CI test checks the document's keys).
- Audit details carry codes, counts, ids and hashes. Free text (`reason`, `reason_text`) stays in its row and the change
  request, never in an audit detail.
- The answer for a person outside the roles above, for every item in this phase, is nothing beyond the labels and the
  calendar.

**Settings and configuration used** (migration `0028_reference_settings`, bounds in `common::settings_registry`):
`fx.rate_max_age_days` 1 to 7 (governed); `country.override_max_days` 30 to 730 (governed; K-24 basis twelve months as the
default); `calendar.horizon_days` 90 to 1095; `calendar.capture_max_rows` 100 to 50000; `policy.dry_run_max_rows` 100 to 100000;
`policy.dry_run_history_days` 30 to 1095; `schedule.fx_rate_import` and `schedule.ref_expiry_sweep` (cron, bank time).
Flag `fx.cbi_feed_enabled` (default off). Phase 00 `bank.timezone` (the clock zone). Phase 03 `delegation.max_days`. Phase 00
floors where a clock has one. Phase 04 `change.route_map` (one rule added, section 4).

## 4. Infrastructure Updates

- **Postgres `jobs` kinds** (both akr-worker; the `ref_` and `fx_` prefixes belong to `akr_app`):
  - `fx_rate_import` (payload `{rate_date}`; coalesce key `fx_rate_import:<rate_date>`; concurrency 1; max attempts 5 with
    back-off; scheduled by `schedule.fx_rate_import`, Sunday to Thursday 09:30 bank time; a disabled flag completes with
    `skipped: feed_disabled`).
  - `ref_expiry_sweep` (payload `{}`; coalesce key `ref_expiry_sweep`; concurrency 1; max attempts 3; scheduled by
    `schedule.ref_expiry_sweep`, daily 00:10 bank time). It ends expired country overrides (section 3.1) and computes the
    calendar horizon and the override expiry window for the gauges of this section; it writes no other row.
  - No other job. The reconciliation is computed on request (section 3.3), and the policy dry run is synchronous with a
    row cap (section 3.5), so neither needs a queue.
- **Timers:** no new timer kind. The phase 00 `admin.timers.calendar_id` foreign key is added in section 1, and every
  working-day timer is scheduled with `BankCalendarClock` and stores `clock`, `calendar_id` and `calendar_version` in its
  payload (section 3.3 step 8). The phase 04 `cr_activate` and `cr_post_review` timers are unchanged.
- **Outbox events:** `ref.country_level_changed {iso2, from, to, cause: release | override | expiry | revert}` (section
  3.1, DR-84), written in the transaction that changes the effective level; no consumer until phase 15 (rating re-score).
- **Ingestion:** `akr-ingestd` gains one step inside `discovery_run` (the phase 05 pipeline): the calendar capture of section
  3.3, with the row cap of `calendar.capture_max_rows`. It uses the existing source connection and the existing read-only
  proofs; no new edge, no new credential, no write to the CBS.
- **Docker compose:** no change. The CBI publication request goes out through the bank's proxy on the existing `z2_egress`
  network attached to akr-worker (phase 07, DR-71). The rule is unchanged; it becomes live only with the flag
  `fx.cbi_feed_enabled`.
- **AppState additions:**
  - `api`: `reference: Arc<ReferenceService>`, `calendars: Arc<CalendarService>`, `policy: Arc<PolicyPackService>`,
    `authority: Arc<AuthorityMatrixService>`, `business_clock: Arc<BankCalendarClock>` (the API's `BusinessClock`),
    `authority_delegation: Arc<AuthorityDelegation>` (the phase 03 `DelegationAuthority`; replaces `NoMatrix`), the six
    `ChangeTarget` implementations in the registry.
  - `worker`: `FxRateService` with `CbiPublicationSource` (feature flag gated), `ExpirySweep`, `BankCalendarClock`, the six
    apply hooks, `PolicyDryRun` with `NoHistory` as its `HistorySource`.
  - `ingestd`: `CalendarCaptureStep` and `CaptureRepository::insert_batch` (write path, `akr_ingest`).
  - `ctl`: `ref seed`, `ref verify-data`, `policy-pack seed`.
- **OpenAPI registration:** tags `reference` (16 operations), `policy` (3) and `authority` (2): 21 operations, each with
  `x-akr-action` (section 3.8). The CR routes answer 202 with `Location: /api/v1/change-requests/{id}`. Every mutation
  documents `Idempotency-Key`; the versioned mutations document `If-Match`. No new step-up declaration: the approvals carry
  the phase 01 classes `approval` and `signing`, which exist.
- **Problem codes added to API.md section 6:** `E_FX_RATE_MISSING` (409; no CBI or manual rate for the currency and business
  date) and `E_INVARIANT_LOCKED` (422; a fixed clock or switch of the policy pack would change). Both are listed as additions
  in the API.md update below; no other new code is introduced.
- **Readiness:** akr-api `/readyz` adds `ref_seeded` (active currencies, governorates, the default calendar and at least one
  country exist) and `policy_pack` (version 1 exists). `authority_matrix` is reported as `degraded` while no matrix is
  signed; readiness still passes, and decisions and delegations are refused (section 3.6).
- **Metrics (new):** `akr_fx_rate_age_days{currency}` (age of the newest CBI rate, gauge), `akr_calendar_horizon_days{calendar}`
  (days the calendar already holds ahead, gauge), `akr_country_overrides{level, expiring_30d}` (gauge),
  `akr_policy_pack_version` and `akr_authority_matrix_version` (gauges), `akr_fx_import_refused_total` (counter).
  Alert rules: no CBI rate for a working day by 12:00 bank time while the flag is on; calendar horizon under 60 days; an
  override expired and not yet swept for more than one day.
- **Ops cockpit (phase 03 `OpsCockpit`):** three tiles on A3: the age of the newest rate per currency, the calendar horizon,
  and the overrides expiring within 30 days.
- **Evidence and inventory:** the generated evidence kind `policy_dry_run` joins the phase 04 `EvidenceResolver` (generated at
  submit and at every deciding action). The four inventory items (`reference_list`: country table, calendars and reason codes;
  `matcher`: the forbidden-word scanner and the template rules) are registered by the bank through `POST /inventory` (phase
  04) and approved by a validator, never by the release. The `authority_matrix` kind needs no inventory item.
- **Network zones:** no new zone and no new edge. akr-worker reaches the CBI publication host only through the proxy, and only
  while the flag is on. akr-ingestd reaches the CBS only for the approved calendar objects, through the phase 05 proofs.

**Existing entity, schema or route updates required by this phase** (each lands in the same change as this file, per
planning.md):

- **Phase 00:** `admin.timers.calendar_id` gains its foreign key to `ref.calendars(id)` (section 1, deferred by phase 00);
  the `BusinessClock` port's production implementation is `BankCalendarClock` from this phase (phase 00 keeps
  `CalendarDaysClock` for tests); the settings registry gains the bounds of section 3.8; `ref` schema already exists.
- **Phase 02:** the evidence owner kind `fx_rate` is registered for manual FX rates (evidence kind `upload`), and the owner
  registry lists it.
- **Phase 03:** `AuthorityDelegation` replaces `NoMatrix` in the `DelegationAuthority` wiring; `DelegationService` step 2
  caps the window at `min(delegation.max_days, window_limit_days)` and step 3 calls `check` with the matrix; the Cedar
  files `C0_reference.cedar` and `C0_policy.cedar` and the entities of section 3.8 are added to `schema.cedarschema`. The
  `ops cockpit` gains the three tiles above. The dev seed (`akr-ctl identity seed-dev`, one user per role) already has
  `mro@bank.test` (the only role
  with `model.risk`, needed to register the four inventory items in the curl suite).
- **Phase 04:**
  - the `change.route_map` setting gains one rule, placed directly after the emergency rule:
    `{"object_kind": ["authority_matrix"], "route": "cr_authority"}`. Its validation (every class has a fallback, every named
    family exists and lists `change_request`) is re-run by the seed;
  - the shipped definitions gain `cr_authority` (`backend/workflows/shipped/cr_authority.json`): `resource_kinds:
    ["change_request"]`, `classification_default: "internal"`, one `sign` step for the named officer `mlro` with step-up
    class `signing` and `content_sha256` recorded, then one `approve` step for role `CHK`; timeout `esc CH 3bd` on both
    steps; terminal effect `activate_change`; every deciding step `stepup: true` and `reject: {comment_required: true,
    mode: "end"}`;
  - the `EvidenceKind` enum gains `policy_dry_run` (generated);
  - the six change targets of section 3.7 register with the `ChangeTargetRegistry`;
  - the phase 04 working-day stand-in in the workflow timeouts (`CalendarDaysClock` in phase 04 section 3.6) becomes
    `BankCalendarClock` from this phase;
  - the phase 04 note "no authority matrix and no bank calendar (phase 09)" is resolved by this phase.
  - the phase 04 `ref.reason_codes` grants and the classification of `template` are changed in section 1 of this file.
- **Phase 05:** the adapter pack's `hints` gain a `calendar` block (the two approved objects, section 3.3); `discovery_run`
  gains the calendar capture step (ingestd); `discovery_seal_report` enqueues nothing new (the reconciliation is on request);
  the cockpit reads the capture's row count in the run summary.
- **Phase 06:** the `cbi_refs` check of reason codes reads the installed catalogue; a base-level change in a release seed
  writes the outbox event `ref.country_level_changed` (section 3.1); phase 15 consumes it for TR-10. Readiness grades
  (`coverage_grade`) do not depend on country levels and are not re-run.
- **Phase 07:** no change. Its proxy rule for the Office watcher is reused for the CBI publication host, with its own host
  entry.
- **Phase 08:** the screen D3 is built in this phase on the phase 08 layer (section 5); `screens.ts` already lists D3 at P1.
  No change to phase 08 files.
- **Phase 10:** the reconciliation of section 3.3 uses the bank-confirmed currency code map; until phase 10 lands, only the IQD
  mapping of section 3.3 is used.
- **API.md:**
  - section 3.8 gains `GET /ref/calendars/{id}/business-days`, `GET /ref/calendars/{id}/reconciliation`, `GET /ref/calendars/{id}/days`
    (the scope and description of section 2), `POST /ref/calendars`, `PUT /ref/calendars/{id}/week`, `GET /ref/fx-rates/gaps`,
    `GET /ref/currencies`, `GET /ref/governorates`, `GET /ref/calendars`, and the three `/policy-packs` rows; the
    `GET /ref/reason-codes` row notes the template masking;
  - section 3.2 keeps its `/org/authority-matrix` rows (scope and CR marker unchanged) and points them to this phase;
  - section 6 gains `E_FX_RATE_MISSING` and `E_INVARIANT_LOCKED`.
- **DATA-MODEL.md:** s.1 `policy_packs` gains `content_sha256`, `release_id`, `approved_by`, `change_request_id`;
  `authority_matrix` gains `change_request_id`, `content_sha256` and a non-foreign-key `stepup_id`; s.7 `calendars.week`
  takes the shape of section 1, `calendar_days` gains `attrs`, `active`, `source` and `change_request_id`, `fx_rates` gains
  `source`, `evidence_id` and `recorded_by`, and `ref.cbs_calendar_captures` is added; s.21 gains the phase 09 grants
  paragraph; s.23 group 9 names this phase's three migrations.
- **MODULES.md:** M8's **Requirements** line gains the IDs of this file's header (the research IDs found by grep, with W-19
  and TR-10 the country-risk rows of files 04 and 26); M2's P1 authority-matrix line names this phase; M8 states the
  calendar capture and the reconciliation as the M8 P1 bullet.
- **roadmap.md:** the migration chain gains 0026 to 0028; the row 09
  scope gains "reconciliation with the CBS calendar (phase 05 capture)" and the dependency on 05.
- **status.md:** the phase 09 row and totals (10 tables: 8 in ref and 2 in admin; 21 routes; 2 jobs).

## 5. Frontend Impact

Screen D3 (DESIGN-SYSTEM 8.1, route `/policy`, STUDIO pattern, P1) is built in this phase on the phase 08 layer (the
reduced shell, the shared components, `screens.ts`, the route-to-policy test, `DiffView`, `StepUpDialog`); it is not part of
phase 08. Its tabs follow the rail rules of DESIGN-SYSTEM 8.2 (CFG and CHK see the studio; M, CH and IA read it; ADM proposes
the matrix; DS sees the calendar reconciliation and the rate gaps). No screen is shown to a role that the server would refuse:
the rail and the route guard hide a tab the caller may not open, and a direct link to a masked resource shows X6.

- **Calendars tab:** the calendars with their default badge and the weekend of each currency as weekday words (never
  colour alone); the days in a chosen range with the kind word, both names and the Hijri label with its variant name, marked
  as display only; a business-day preview (from, n, unit) that shows the business date and the bank-time result from
  `GET /ref/calendars/{id}/business-days`; the reconciliation panel for DS (`cbs_only`, `akr_only`, weekend differences,
  unmapped currencies, and the capture status); "Propose days" for CFG, a table of at most 400 rows with validation shown per
  row, submitted with `Idempotency-Key`, answered 202 and linked to G10 detail.
- **Countries tab:** the table of every country returned by `GET /ref/countries` with base level, effective level,
  the open override with its expiry and reason, and the basis; "Propose override" for CFG (level, reason, expiry date bounded
  by `country.override_max_days` shown in the field), `If-Match` on the country version, answered 202 with a G10 link. The
  level is a word and a tone, never a colour alone.
- **FX tab:** rates by date with the source word (CBI or manual) and the age in business days; the gaps list for DS
  (`GET /ref/fx-rates/gaps`); "Record manual rate" for DS (currency, date, rate, evidence picked through the phase 02 upload
  control, the CBI notice as evidence). The screen states that a manual rate never replaces a CBI rate.
- **Reason codes tab:** the five families of this phase and the two of phase 04 as groups; each code with its label, active
  state and version; the template shown only to CFG and CHK; a batch editor (at most 50 codes of one family) for CFG,
  answered 202 and linked to G10.
- **Policy pack tab:** the current version as sections (routing, clocks, calendar, switches, templates, forbidden words,
  priority labels); the forbidden-word section visible to the readers of section 3.8 only; a `DiffView` of the proposed document
  against the current one; the dry-run block (clock effects, template hit counts, history source `none until phase 17`); "Propose
  version" for CFG with `If-Match` on the active version, answered 202 and linked to G10.
- **Authority matrix tab:** the grants as a grid (decision type by role, with the priority class word and the second-review
  word), the delegation rules, the signature (signed by, date, change request link); "Propose matrix" for ADM; a notice when no
  matrix is signed yet: "Delegations are refused until the MLRO signs the first matrix."

Generated hooks used (Orval, from the operation ids of section 2): `useGetRefCountries`, `usePutRefCountriesIso2Override`,
`useGetRefFxRates`, `usePostRefFxRates`, `useGetRefFxRatesGaps`, `useGetRefCurrencies`, `useGetRefGovernorates`,
`useGetRefCalendars`, `usePostRefCalendars`, `usePutRefCalendarsIdWeek`, `useGetRefCalendarsIdDays`,
`usePostRefCalendarsIdDays`, `useGetRefCalendarsIdBusinessDays`, `useGetRefCalendarsIdReconciliation`,
`useGetRefReasonCodes`, `usePostRefReasonCodes`, `useGetPolicyPacks`, `useGetPolicyPacksId`, `usePutPolicyPacksActive`,
`useGetOrgAuthorityMatrix`, `usePutOrgAuthorityMatrix`. Change-request detail and the approval actions are phase 08's G10 and
phase 04's task panel, unchanged.

Rules of this screen: Arabic first with right-to-left layout and Western digits in data (the user's numeral setting applies to
display); Hijri only as labelled display text (UX-22); every number that drives a clock is shown with its unit word; no
threshold appears in any reason-code text the screen shows to branch roles.

## 6. Verification

1. `make migrate`: migrations 0026 to 0028 apply after phases 00 to 06 and a second run is a no-op. Negative checks, each
   as the owner role `akr_migrate` and as `akr_app`: `DELETE FROM ref.countries` fails with `reference rows are never deleted`
   and `UPDATE ref.fx_rates SET rate_to_iqd = 1` fails with `an FX rate is a fact` (the two messages as the owner; as `akr_app`,
   which holds neither privilege, both statements fail with permission denied); a second open override for one `iso2` fails
   `uq_country_override_open`; a `calendar_days` row with `kind = 'ramadan_hours'` and empty `attrs` fails its CHECK; a second
   default calendar fails `uq_calendars_default`; `UPDATE admin.policy_packs SET document = document` fails with `versions are
   append-only` as the owner and with permission denied as `akr_app` (SELECT and INSERT only); `akr_app` has no DELETE on any `ref` or `admin` table of this phase; `akr_app` cannot UPDATE
   `ref.reason_codes.family` or `code`; `admin.timers` rejects a `calendar_id` that is not in `ref.calendars`.
2. `cargo check` with `cargo sqlx prepare` (the ten tables and their queries); `cargo clippy --all-targets -- -D warnings`;
   `cargo deny check`.
3. `akr-ctl ref seed` twice: the first run reports the release counts (countries, governorates, currencies, the default
   calendar) and the inactive reason codes of the five drafted families; the second run writes nothing and audits zero
   inserts. `akr-ctl ref verify-data` exits 0 with the counts; a release file with one changed byte exits 1 and writes nothing.
   `akr-ctl policy-pack seed` installs version 1 once (second run: already present, exit 0).
4. `cargo test`, including:
   - `BankCalendarClock` (`domain/reference/tests`): a Friday start counts from Sunday; a Thursday start with n = 2 gives the
     Monday of section 3.3; a holiday row skips a working day; an Eid row closes its day; `ramadan_hours` rows do not close a
     day; `hours` and `days` are plain addition; the horizon flag is set when the walk leaves the covered range; converting to the bank zone and back returns the same instant (Iraq keeps no daylight saving); the module imports no Hijri type (a test greps the source).
   - `CalendarReconciler`: `cbs_only` and `akr_only` on a fixture of ten holidays; an unmapped currency is listed and never
     compared; no run gives `no_capture`.
   - FX: `rate_on` prefers `cbi`; `newest_within` flags `stale` and never returns a row older than `fx.rate_max_age_days`; a
     manual insert on a day with a CBI row refuses `cbi_rate_exists`; the IQD row with rate other than 1 is refused by the CHECK.
   - `ForbiddenWordScanner`: folded matches across alef and ta marbuta variants; "strategy" does not match "str"; a phrase term
     matches across punctuation; a hit returns a span and never the term; an empty list never hits.
   - `PolicyValidator`: each rule of section 3.5 in isolation (unknown key, clock out of bounds, fixed clock changed, switch
     changed, template with a figure, template with a forbidden term, `ar: null` warning, identical hash refused); the canonical
     hash is stable across key order.
   - `AuthorityMatrix`: `never_delegable` for ADM, SEC, IA, AS and VS; `role_lacks_scope` for an A1 grant of `case.decision`;
     `permits` respects the priority order (Immediate permits Routine, not the reverse); `AuthorityDelegation` returns
     `NoMatrix` before a matrix exists and `NotDelegable` for a role without a delegable row.
   - Reason codes: a template on a `decision` code is refused (`template_not_allowed`); a figure in a `detection` template is
     refused; `cbi_refs` unknown to the catalogue is refused.
   - Change targets: each target's `prepare` on a valid and an invalid body; `check_before_decision` refuses a moved base
     (`E_STALE_BASE`).
5. Curl suite `tests/curl/phase09_reference.sh` (dev users from `akr-ctl identity seed-dev`: `cfg@bank.test`, `chk@bank.test`,
   `m@bank.test`, `ds@bank.test`, `ch@bank.test`, `ia@bank.test`, `adm@bank.test`, `a1@bank.test`, `a2@bank.test`, and
   `mro@bank.test` (all from the one-per-role fixture); step-up codes from `oathtool` as in phase 03; ids taken from the responses with `jq`):
   - As A1 `GET /api/v1/ref/countries` -> 200, every row has `iso2` and `risk_level`; the list contains no `template` key.
   - As A1 `POST /api/v1/ref/reason-codes` with `Idempotency-Key: rc-0001` -> 403 `E_SCOPE` (no `config.author`).
   - As DS `PUT /api/v1/ref/countries/IR/override` with `If-Match` and a valid body -> 403 `E_FORBIDDEN` (scope passes, Cedar
     allows CFG only).
   - As CFG `PUT /api/v1/ref/countries/IR/override` with `Idempotency-Key: cty-0001`, `If-Match: "<version>"`, body
     `{"level":"very_high","reason":"Test override for the curl suite","expires_at":"<now + 30 days>"}` -> 202 with `Location`
     and `route: cr_minor` (a raise is never routed by rule 2); the same key and body again -> 202 with `x-idempotent-replay: true` and the same body bytes; the same key with
     `level: "high"` -> 422 `E_IDEMPOTENCY_REUSE`; without `If-Match` -> 428.
   - As CHK approve that change request with step-up `approval` -> 200; `GET /api/v1/ref/countries` as A1 shows `IR` with the
     override `level: very_high`, `direction: raise`, and the expiry; `audit_log` holds `country_override.applied`.
   - As DS `GET /api/v1/policy-packs/<version 1 id>` -> 404 `E_NOT_FOUND`; as DS `GET /api/v1/policy-packs/<random uuid>`
     -> 404; the two bodies are byte-equal after `jq 'del(.correlation_id)'`; as DS `GET /api/v1/policy-packs` -> 403
     `E_FORBIDDEN`; as CH the version 1 detail -> 200 with the document; as A1 -> 403 `E_SCOPE` on the list.
   - As CFG `PUT /api/v1/policy-packs/active` with `If-Match` and a document whose `tipping_off_flag_default` is `false` -> 422
     `E_INVARIANT_LOCKED`; with a `customer_notice` template containing the word "suspicion" -> 422 `E_VALIDATION` with
     `errors[0].code = "forbidden_term"` and the pointer and span only; `grep -c suspicion` on the response body -> 0; with a
     stale `If-Match` -> 409 `E_VERSION_CONFLICT`; with the valid edit (one RFI clock value changed inside bounds) -> 202, and
     `GET /api/v1/change-requests/<id>` shows the generated `policy_dry_run` evidence with `history_source: none_until_phase_17`.
   - As M approve that policy change with step-up `approval`, then CHK approve it; the maker cannot approve (`E_SELF_APPROVAL`);
     `GET /api/v1/policy-packs` as CH -> version 2 at the top with `approved_by` set; `audit_log` holds `policy_pack.activated`.
   - As DS `POST /api/v1/ref/fx-rates` with `Idempotency-Key: fx-0001` and a body for USD on 2026-10-08 with `evidence_id` of an
     `upload` object -> 201 (the rate is a fixture for the suite, not a published rate); the same key and body -> 201 with
     `x-idempotent-replay: true`; the same key with another rate -> 422 `E_IDEMPOTENCY_REUSE`; with an `evidence_id` of kind
     `render` -> 422 `E_VALIDATION`.
   - As A1 `GET /api/v1/ref/fx-rates/gaps?currency=USD&from=2026-10-08&to=2026-10-12` -> 403 `E_SCOPE`; as DS the same call -> 200
     with `missing: ["2026-10-11", "2026-10-12"]` (Friday and Saturday are not business days; the 08th has the manual rate).
   - As DS `GET /api/v1/ref/calendars/<default id>/business-days?from=2026-10-08T10:00:00%2B03:00&n=2&unit=working_days` ->
     200, `business_date: "2026-10-12"`; with `from=2026-10-09T10:00:00%2B03:00&n=1` -> `2026-10-11`; with `unit=hours&n=2` ->
     plain addition.
   - As DS `GET /api/v1/ref/calendars/<default id>/reconciliation` -> 200 with `status` `captured` (after the suite's discovery
     run) or `no_capture`; as A1 -> 403 `E_SCOPE`.
   - As CFG `POST /api/v1/ref/calendars/<default id>/days` with 401 rows -> 422 `E_VALIDATION`; as DS the same call -> 403
     `E_FORBIDDEN` (scope `config.author` passes, Cedar allows CFG only).
   - As ADM `PUT /api/v1/org/authority-matrix` with a document that makes `ADM` delegable -> 422 `E_VALIDATION` with
     `never_delegable`; with a valid matrix -> 202 and `route: "cr_authority"`; as M the sign step without `X-StepUp-Assertion` ->
     401 `E_STEPUP_REQUIRED`; with the assertion of class `signing` -> 200; as CHK approve -> activation; `GET /api/v1/org/authority-matrix`
     -> version 1 with `signed_by` equal to `m@bank.test`'s id.
   - As A2 `POST /api/v1/org/delegations` before any matrix is signed (run first in the suite) -> 422 `E_VALIDATION` at `/roles`
     with `no_matrix`; after the matrix, delegating a delegable role for 3 days -> 201; delegating `ADM` -> 422 `not_delegable`.
   - As CFG `POST /api/v1/ref/reason-codes` with `Idempotency-Key: rc-0002` and a `decision` code carrying a `template` -> 422
     `template_not_allowed`; a `detection` template with the figure "15" -> 422 `template_has_figures`; a valid batch -> 202 with
     `Location`; as CFG `GET /api/v1/ref/reason-codes?family=decision` -> 200 with no `template` key.
   - Swagger UI lists 21 new operations (`reference` 16, `policy` 3, `authority` 2), each with `x-akr-action`; `make generate-api`
     twice gives zero diff and the hooks of section 5 exist.
6. Audit: after the curl run `audit_log` holds `country_override.applied`, `fx_rate.recorded`, `policy_pack.activated`,
   `authority_matrix.activated`, `authz.denied` for the A1 and DS refusals, and the `deny` access events for the masked policy
   reads; `akr-ctl audit verify` passes. `calendar_day.changed` and `reason_code.changed` are exercised by the unit and
   integration tests of item 4 (the suite does not approve a day change or a code batch).
7. The ref seed and the policy seed emit `ref.seeded`, `reason_codes.seeded` and `policy_pack.seeded` exactly once each on first run.
8. No CBS connection is made by the API or the worker in this phase. The calendar capture runs only inside a discovery run
   triggered by phase 05: its statements are single `SELECT`s on the two approved objects (the conformance suite of phase 05
   covers them), and `ref.cbs_calendar_captures` gets rows only from the ingestd role.
9. Phase 00 to 06 tests still pass, including phase 03's delegation tests, which now pass only with a matrix installed by their
   fixture (the phase 03 fixture gains a matrix seed; a phase 03 test that expected `NoMatrix` is changed in the same change).

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 3a, from tools/agent/gap1-run3a.md; lead-verified (tools/agent/gap1-run3a-verdict.md).

### 7.1 Clock registry entries of phases 20 and 21 (DR-370)
Registry entries carry a unit of type `ClockUnit` (`WorkingDays`, `Hours`, `Days`, `CalendarMonths`). The first
three are `BusinessUnit` values and are computed by `BusinessClock::add`. `CalendarMonths` is a label only:
`BusinessUnit` and `BusinessClock` do not change, and the month is computed by phase 20's `add_calendar_months`. The
clock list of section 3.5 gains:
- `freeze.limit`: hours, 24, `fixed: true` (phase 20 section 3.14)
- `freeze.committee_report`: hours, 48, `fixed: true` (phase 20)
- `freeze.second_report`: calendar months, 2, `fixed: true` (phase 20)
- `refrain.clock`: working days, 7, `fixed: true` (phase 20; the refrain clock already named in prose)
- `report.filing_target`: hours, 24, bounds 1 to 24 (phase 21 section 3.18)
- `report.rework`: working days, 10, bounds 5 to 20 (phase 21 section 3.18)

### 7.2 Reason codes seeded for phases 14 and 15 (DR-372)
`reason-codes.v1.json` gains, inactive like the other seeded codes:
- Family `decision`, for screening decisions (phase 14): `DEC_SCR_NAME_DIFFERS`, `DEC_SCR_ID_DIFFERS`,
  `DEC_SCR_DOB_DIFFERS`, `DEC_SCR_FAMILY_NAME_ONLY`, `DEC_SCR_CONFIRMED`, `DEC_SCR_POTENTIAL_REVIEW`,
  `DEC_SCR_INSUFFICIENT_DATA`, `DEC_SCR_OTHER`.
- Family `decision`, for the exit register (phase 15): `DEC_EXIT_FINANCIAL_CRIME`.
- Family `override` (phase 15 section 4.2): `OVR_UPWARD_ADVERSE_INFO`, `OVR_UPWARD_PEP_JUDGEMENT`,
  `OVR_UPWARD_EXPECTED_ACTIVITY`, `OVR_UPWARD_COMPLIANCE_JUDGEMENT`, `OVR_DOWN_SOURCE_DOCUMENTED`,
  `OVR_DOWN_STRUCTURE_VERIFIED`.
They become usable only through the family change request of section 3.7.

### 7.3 The consumer of ref.country_level_changed (phase 15)
The sentences "no consumer until phase 15" (section 3.1 and the rating note of section 4) are closed by phase 15,
which registers the consumer `rating_country_fanout`. The outbox write of this phase does not change.

### 7.4 Backward working-day walk (phase 16 section 3.8 step 5)
`BankCalendarClock::working_days_back(&self, d: NaiveDate, n: u32, calendar_id: Uuid) -> Result<NaiveDate,
ClockError>` walks back `n` working days from the local date `d` over the same day set and weekend pattern that `add`
uses forward (section 3.3). It is a pure method of `BankCalendarClock`, not of the `BusinessClock` trait, and has its
own unit test. The backward load reads the closed days from `d - (3n + 14)` days to `d`. A walk past the earliest day the calendar holds (the earliest active row date of any kind) is decided by the weekend pattern alone for the remaining dates, as `add` is past `covered_until` (section 3.3 step 7), and returns the date. `ClockError`, for both methods, has the variants `CalendarMissing` (no calendar for the id, or no default calendar) and `OutOfRange` (`n` outside 1 to 3660).

### 7.5 D3 tab Assessment (phase 29 section 5.2)
Phase 29 adds the tab `Assessment` to D3 (panels "Fail policy", "Rail modes", "Prohibited rules"). It has no role
list of its own: like every section of a screen, it renders only when the caller may call one of its operations
(phase 08 `screens.ts` rule), and phase 29's Cedar decides who may.

### 7.6 The segments router under /ref (phase 13)
The `reference` router mounts phase 13's `segments` router under `/ref`. No route row of this phase changes; the
out-of-scope note on segments covers their content, not the mount.

Gap pass 1 (2026-10-10), run 6b (coverage of T IDs), from tools/agent/gap1-run6b.md; lead-verified (tools/agent/gap1-run6b-verdict.md).

### 7.7 The branch-first switch is fixed at false (M-14, DR-399)
No phase defines the five conditions that turning on `branch_first_routing` needs; they are P2. The switch is
registered `fixed: true` with the value `false` (section 3.5), so validation step 3 refuses `true` with
`E_INVARIANT_LOCKED`. A later phase that defines the conditions unfixes it.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.8 Codes for the document refusals the section 3.8 table does not name (V-09-01-b, V-09-04-b, V-09-05-a, V-09-09-a, MEDIUM; DR-413)
A refusal of a calendar, authority matrix or policy pack document that the `E_VALIDATION` row of section 3.8 does not name takes a DSL code (DSL.md section 7) in `errors[]`, with the JSON pointer of the offending key or value:
- `E_SCHEMA`: a weekend list that is not a subset of `1..=7` of size 1 to 3; a week with fewer than three working days; a repeated `(on_date, kind)`, template key or term id; `from` not before `to`; more than 400 rows in one request; a `max_priority` outside the four classes; a `second_review` that is not a boolean; an empty forbidden list; a `fixed` clock or switch that the document omits.
- `E_DANGLING_REF`: an unknown currency; an inactive reason code; a matrix version that does not exist; a placeholder outside the allow-list.
- `E_PARAM_OUT_OF_BOUNDS`: a clock outside its registry bounds (section 3.5 step 2); a `max_days` outside `1..=delegation.max_days` (section 3.6).

### 7.9 The clock method name in phases 00 and 04 (V-09-01-d, LOW; DR-413)
This phase renames the `BusinessClock` method to `add(from, n, unit) -> Result<ClockResult, ClockError>` (section 2). The phase 00 trait line (`add_working`) and the two phase 04 calls (the emergency review date of phase 04 section 3.2 and the task due date of phase 04 section 3.6) change to `add`, and each call handles the `Result`. Section 4's phase 00 and phase 04 bullets do not repeat the rename; this paragraph is its record.

### 7.10 CaptureRun fields (V-09-01-e, LOW; DR-413)
`CaptureRun` (section 2, `CaptureRepository::latest_run`) is a read of the phase 05 `admin.discovery_runs` row with the fields `id`, `source_id`, `status` and `finished_at`.

### 7.11 The pack names no queue (V-09-03-a, MEDIUM; DR-413)
Section 3.5, "What a pack holds": the routing part of a pack is the switch `branch_first_routing` and nothing else. A pack names no queue and no pool. Queues are rows of `casework.queues`, created and changed only through phase 17's `queue_definition` change target, and phase 17 takes an alert's pool from them. So the queue and pool check of validation step 4 and the routing keys of step 9 apply to no key of `policy-pack.v1`, and the codes `unknown_queue` and `pool_archived` are raised by no check of this phase (the authority matrix has no queue or pool field either), so they leave the row of section 3.8.

### 7.12 PolicyPackSummary, Validated and ValidationIssue (V-09-05-c, LOW; DR-413)
`PolicyPackSummary` (the list rows of `PolicyPackRepository::list`) is the `PolicyPackView` of section 2 without `document`. `Validated`, the success result of `PolicyValidator::validate`, holds the canonical `document`, its `content_sha256` (validation step 8), the `changed_keys` (step 9) and the warnings of step 6. `ValidationIssue` is the phase 04 type that `Prepared.warnings` carries; the 422 maps each issue to a phase 00 `ValidationError`.

### 7.13 The object ids of the four inventory items (V-09-06-a, MEDIUM; DR-413)
The four inventory items of section 4 describe a whole register, not a row, so each has a fixed `object_id`: the UUIDv5 (URL namespace) of `urn:akr:inventory:countries`, `urn:akr:inventory:calendars`, `urn:akr:inventory:reason_codes` and `urn:akr:inventory:policy_pack`, held as constants of the `reference` module. A target's `prepare` returns its constant as `Prepared.object_id` (`country_override`: countries; `calendar` and `calendar_day`: calendars; `reason_code`: reason codes; `policy_pack`: its own), so the phase 04 gate finds the item by `(kind, object_id)`. The bank registers each item through `POST /inventory` with that `object_id`; `akr-ctl ref seed` prints the four ids.

### 7.14 The reference release: check, keys, later releases and owners (V-09-08-a, V-09-08-b, V-09-08-c, V-09-08-d, MEDIUM; DR-413)
Owners: `ref seed` and `ref verify-data` are operations of `ReferenceService` (section 3.1), which also holds the reason-code seed (section 3.4); `policy-pack seed` is an operation of `PolicyPackService`. The `ctl` commands of section 4 call them with the release directory.
Check: before any write the seed reads the release manifest (the phase 00 bundle manifest of SHA-256 digests), verifies its signature (phase 00 release signing) and compares the SHA-256 of each release file with its entry; a mismatch exits 1 and writes nothing, as the catalogue seed does (phase 06 section 3.1 step 1). `ref verify-data` makes the same two checks.
Keys: each file of `backend/packs/reference/` carries its `release_id` (1 to 64 characters) and the rows of one table (the calendar file, below, of two). A `countries` row carries `iso2`, `name`, `risk_level` and `basis`; a `governorates` row `code`, `name` and `region`; a `currencies` row `code`, `minor_unit`, `name` and `is_active`. `version` and `updated_at` are set by the seed. `reference-release.v1.json` is closed and validates exactly these keys for the three table files. The fourth release file, the default calendar, carries `release_id`, the columns of its one `ref.calendars` row and its `ref.calendar_days` rows (section 1), and the schema has a branch for it. `reason-codes.v1.json` is not a release file of this schema: `akr-ctl ref seed` reads it (section 3.4, Seed).
Later releases: a key the table does not hold is inserted. A row whose content differs is updated (`IS DISTINCT FROM`, the rule of `upsert_days`) with the new `release_id` and `updated_at`; a changed `countries` row also bumps `version`. An identical row is not written and keeps its `release_id`. A row the release no longer lists is left as it is and listed in the seed output: reference rows are never deleted (section 1).

## Decisions

Recorded in research.md: DR-78 (ISO keys, placement, no step-up foreign key, append-only), DR-79 (CBS calendar capture,
reconciliation, business clock, pinned versions, Hijri display only), DR-80 (FX rules and `E_FX_RATE_MISSING`), DR-81
(policy pack registries, `E_INVARIANT_LOCKED`, dry run, restricted forbidden-word list), DR-82 (reason-code families),
DR-83 (authority matrix, decision types, `cr_authority`), DR-84 (country overrides and the TR-10 outbox event). The Cedar
files `C0_reference.cedar` and `C0_policy.cedar` follow the phase 03 range convention and need no decision.

### Open doubts (for gap pass 1 and the bank)

- **Dependency on phase 05 (resolved by the lead).** Section 3.3 needs phase 05's discovery run and adapter-pack hint; the
  roadmap now lists phase 09 as depending on 04 and 05 (05 is P0 and lands first).
- **Conflicting weekend statements in the CBS discovery notes.** `cbs-discovery/CBS-DISCOVERY.md` section 5.6 lists Friday and Saturday
  as the weekend for IQD, USD, EUR, AED and JOD, and Saturday and Sunday for GBP, CHF and JPY; its section 13 summary reads "Friday-Saturday
  for IQD, Saturday-Sunday for others". The phase reads the CBS per currency and hard-codes nothing; the reconciliation view will show
  the difference. The bank should confirm the table before the first calendar change.
- **The CBI publication channel is unknown.** No feed, format, publication time or proxy rule for the CBI rate of the day exists in the
  plan. The import is off by default and its adapter is a port; RP-50 needs the channel before phase 21 runs in production.
- **Country base levels and their list of record.** The release file of base levels needs a named list of record and a cadence (FATF
  plenaries in February, June and October per file 26 TR-10). This phase defines the shape only; no level or basis text is set here.
- **Arabic forbidden-word terms.** The English seed comes from DESIGN-SYSTEM section 11 and IR-31. The Arabic terms need a compliance
  reviewer; until then the pack warns `ar_term_unreviewed` on every proposal.
- **The five branch-first conditions (M-14).** Decided in gap pass 1: they are P2 and the switch is fixed at `false` (section 7+, DR-399).
- **Clock bounds are proposals.** `rfi.response` (5 to 20 working days, default 10 from M-44), `ops.task.due` (1 to 10 working days) and
  `case.review` (30 to 365 days) are design proposals for compliance to confirm, not values from the research.
- **Governorate effective dates.** Research file 25 (section 11, item 9) asks for admin-managed reference data with effective dates; this
  phase keeps one current name per governorate and the release history only.
- **Authority matrix blocks delegation until signed.** This changes phase 03's leave behaviour at go-live. The bank must sign the first matrix
  before any delegation; the readiness reports it.
- **The dry run is weak until phases 17 and 18.** Its history source is empty, so its clock and template counts show the rules only, not
  the effect on live cases.
- **The first inventory registration.** The four inventory items must be registered and approved before the first change of their kind
  (GV-01). The first changes in a new installation therefore wait for a validator; this is intended, but the install checklist must say so.

## What This Phase Does NOT Touch

- No scenario register, scenario versions, parameter sets or the scenario studio (phase 16); the scenario review calendar of S-46 is phase 16.
- No segments, no governed registers under `/ref/registers/{kind}` and no `/ref/typologies` (phases 13 and 14; the typology set is phase 06's,
  DR-381). `/ref/segments` is not touched.
- No mapping, no currency code map and no canonical `canon` tables (phase 10): the CBS-to-ISO currency mapping is read, never written, here.
- No alert, case, decision, RFI, freeze or report record (phases 17 to 21). The decision services that call `AuthorityMatrix::permits` are phases
  17 to 21; this phase implements and tests the function and wires the delegation check only.
- No CTR job, no CTR batch and no goAML XML (phase 21); the exact-date FX rule is what phase 21 uses.
- No branch observation log, no branch calendar behaviour beyond the default calendar and the Ramadan hours rows (phase 22).
- No regulatory calendar of recurring reports (RP-49), no clock engine beyond the business clock and no per-profile clocks (phase 21 and
  the goAML profile of phase 07).
- No new Cedar range beyond `C0_reference.cedar` and `C0_policy.cedar`; no change to phase 06's pack downloads or `C0_packs.cedar`; no change
  to the phase 04 CR lifecycle, the workflow engine, the inbox or the task model.
- No CBS connection of its own: the capture is a step of the phase 05 discovery run, and no statement other than the two approved `SELECT`s
  runs. No write to the CBS, ever.
- No SSO, identity or step-up changes (phase 01); no change to the phase 03 Cedar bundle beyond the two files and the entities listed in section 3.8.
- No new settings beyond the eight keys and one flag of section 3.8; no change to phase 00's settings registry format.
