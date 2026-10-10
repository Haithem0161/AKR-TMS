# Phase 06: CBI Catalogue, Readiness Grades & Gap Report

**Goal:** The first early piece: the CBI's 173 items (86 numbered scenarios and 87 product and behaviour indicators,
supervisory controls of 8 January 2025, chapter 6) installed as a register that is never pruned, the AKR library's 161
scenarios registered against them, a data-readiness grade A-D per scenario and per CBI item computed from the field
availability that phase 05 measured on the bank's own database, the bank's "not covered" declarations with an owner,
an interim route and a review date, draft field-mapping candidates for the attributes discovery could not place, and
the CBI coverage gap report as a sealed JSON document and signed PDF packs in Arabic and English (with a separate
vendor-request annex and the integration profile PDF promised by phase 05), shown on screen E2.

**Dependencies:** Phase 05, Phase 08 (the reduced shell and shared components for E2)
**Macro-phase:** P0
**Complexity:** M (14 routes; the register data and the grading rules carry the weight)
**Binaries touched:** akr-api (routers `catalogue`, `coverage`, `packs`; `FieldCandidateFinder` on the `discovery`
router) | akr-worker (jobs `coverage_grade`, `gap_report_build`, `pack_build`, `coverage_sweep`) | `akr-ctl` (`packs
seed`, `packs verify-data`). akr-ingestd and akr-assess unchanged
**Requirements carried:** S-01 (the CBI baseline pack as a versioned register of all 173 items, each mapped to its
implementing scenarios and a readiness grade; items never removed; default parameter sets arrive in phase 16), S-02
(the coverage report from the register, stamped with the pack version, as an assessor-ready export; the alert, case
and STR columns fill from phases 17-21), S-03 (readiness per scenario and per bank: grade, missing field named, vendor
ask written out, not-covered declarations with owner, reason, interim route and review date), S-04 (the AKR library
registered as an extension pack with typology tags and the phase-2 candidate flag; parameters and basis tags in phase
16), S-07 (one register as the single source of the coverage report; the studio, engine and scenario card read it from
phase 16), S-54 (the CBI-item and data-ready coverage measures; the risk-statement and outcome measures in phases 16
and 24), C-48 (coverage matrix operable, degraded, blocked from availability now and from mapping completeness from
phase 10), T-02 (the CBI minimum never retired by the product), L-02 (the not-covered declaration with owner, reason
and review date; the four-eyes exclusion register with expiry is phase 12)
- Gap pass 1 (coverage), partly implemented: S-19 (external needs per scenario and the connected-feed registry); L-08 (per-input null share and source reference in the readiness view), L-25 and L-42 (the CBI baseline register with each item mapped to its scenarios); VB-38 (the CBI register and the coverage gap report), VD-39 (the versioned CBI register as the coverage report source).
- Gap pass 1 (coverage), implemented here: VA-19 (the CBI catalogue as a versioned register with its coverage matrix on day one).
- Gap pass 1 (coverage), not carried: VA-51 (small-bank commercial package, DR-402).

---

## 1. SQLx Migration Changes

Three migrations, `0020_cbi_catalogue_register`, `0021_readiness_gap_report` and `0022_coverage_settings`. Five
tables, all in `scn` (DATA-MODEL s.10; `readiness_grades` moves there from s.6, and `gap_report_runs` is added, both
aligned in the same change). One table modified: the `kind` CHECK of `admin.packs` (phase 02) gains
`integration_profile` and `vendor_request`. The catalogue and register rows are not inserted by migrations: they are
installed from the signed pack files by `akr-ctl packs seed` (section 3.1), the same way phase 04 seeds workflow
definitions, so a release can ship a new catalogue version without a schema change.

```sql
-- migration 0020_cbi_catalogue_register.sql
-- Phase 06. The CBI catalogue, the scenario register and the links between them (MODULES M11, DATA-MODEL s.10,
-- DR-64, DR-65). Rows come from the signed pack files (akr-ctl packs seed); nothing here is ever deleted (T-02).

CREATE TABLE IF NOT EXISTS scn.cbi_catalogue_items (
    id                  UUID PRIMARY KEY,
    catalogue_version   TEXT NOT NULL CHECK (catalogue_version ~ '^[0-9]{4}-[0-9]{2}-[0-9]{2}(\.[0-9]+)?$'),
    ref                 TEXT NOT NULL CHECK (ref ~ '^Art\.([1-9]|1[0-4]) no\.[1-9][0-9]?$'),   -- "Art.1 no.6"
    article             SMALLINT NOT NULL CHECK (article BETWEEN 1 AND 14),
    number              SMALLINT NOT NULL CHECK (number BETWEEN 1 AND 99),
    kind                TEXT NOT NULL CHECK (kind IN ('scenario', 'indicator')),   -- articles 1-3 / 4-14
    chapter             TEXT NOT NULL,                        -- '6' for the 2025 controls
    title               JSONB NOT NULL CHECK (title ? 'ar' AND title ? 'en'),
    text                JSONB NOT NULL CHECK (text ? 'ar' AND text ? 'en'),   -- ar = the CBI text, en = translation
    source_doc          TEXT NOT NULL,                        -- issuance reference and page
    research_grade      CHAR(1) NOT NULL CHECK (research_grade IN ('A', 'B', 'C', 'D')),   -- reference baseline
    counter_only        BOOLEAN NOT NULL DEFAULT FALSE,       -- observational: only a person can record it
    status              TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'superseded')),
    superseded_by       UUID REFERENCES scn.cbi_catalogue_items(id) ON DELETE RESTRICT,
    installed_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_cbi_items_ref UNIQUE (catalogue_version, ref),
    CHECK ((kind = 'scenario') = (article <= 3)),
    CHECK ((status = 'superseded') = (superseded_by IS NOT NULL)),
    CHECK (NOT counter_only OR research_grade = 'D')
);
CREATE INDEX IF NOT EXISTS idx_cbi_items_superseded_by ON scn.cbi_catalogue_items (superseded_by)
    WHERE superseded_by IS NOT NULL;

CREATE TABLE IF NOT EXISTS scn.scenarios (
    id                  UUID PRIMARY KEY,
    code                TEXT NOT NULL UNIQUE CHECK (code ~ '^SCN-[A-Z]{2,5}-[0-9]{2,3}$'),
    family              TEXT NOT NULL CHECK (family ~ '^[A-Z]{2,5}$'),
    -- cbi: implements at least one CBI item (the always-on baseline); akr: extension; islamic: optional (phase
    -- 26); regime: a foreign regime's pack (ROADMAP P3, no phase here, DR-406); bank: the bank's own, codes
    -- 901-999 (phase 16).
    pack                TEXT NOT NULL CHECK (pack IN ('cbi', 'akr', 'islamic', 'regime', 'bank')),
    name                JSONB NOT NULL CHECK (name ? 'ar' AND name ? 'en'),
    summary             JSONB NOT NULL CHECK (summary ? 'ar' AND summary ? 'en'),   -- plain language, no thresholds
    typologies          SMALLINT[] NOT NULL DEFAULT '{}',     -- research file 27 B2 typology numbers
    -- [{"attribute": "txn.amount", "use": "required" | "fallback" | "optional"}] (section 3.2)
    needs               JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(needs) = 'array'),
    external_needs      TEXT[] NOT NULL DEFAULT '{}',          -- data outside the CBS (customs, hr, device_logs, ...)
    observational       BOOLEAN NOT NULL DEFAULT FALSE,        -- needs the branch observation log (phase 22)
    research_grade      CHAR(1) NOT NULL CHECK (research_grade IN ('A', 'B', 'C', 'D')),
    phase2_candidate    BOOLEAN NOT NULL DEFAULT FALSE,        -- could be enforced before release (assessment API)
    requires_flag       TEXT,                                  -- e.g. 'pack.islamic'; graded only when the flag is on
    visibility          TEXT NOT NULL DEFAULT 'normal' CHECK (visibility IN ('normal', 'restricted')),
    register_version    TEXT NOT NULL CHECK (register_version ~ '^[0-9]+\.[0-9]+\.[0-9]+$'),
    status              TEXT NOT NULL DEFAULT 'registered' CHECK (status IN ('registered', 'retired')),
    owner_id            UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- scenario owner (phase 16)
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    CHECK (code !~ '-9[0-9]{2}$' OR pack = 'bank'),
    CHECK (pack <> 'bank' OR code ~ '-9[0-9]{2}$'),
    CHECK (NOT (pack = 'cbi' AND status = 'retired'))         -- T-02: the CBI baseline is never retired
);
CREATE INDEX IF NOT EXISTS idx_scenarios_family ON scn.scenarios (family, code);
CREATE INDEX IF NOT EXISTS idx_scenarios_owner  ON scn.scenarios (owner_id) WHERE owner_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS scn.scenario_catalogue_links (
    scenario_id         UUID NOT NULL REFERENCES scn.scenarios(id) ON DELETE RESTRICT,
    catalogue_item_id   UUID NOT NULL REFERENCES scn.cbi_catalogue_items(id) ON DELETE RESTRICT,
    -- full: "implemented by"; partial: "also covered by"; proxy: covered only through a proxy signal (DR-64)
    coverage            TEXT NOT NULL CHECK (coverage IN ('full', 'partial', 'proxy')),
    note                JSONB,                                -- {"ar", "en"}: the replica limit, from the register
    register_version    TEXT NOT NULL,
    PRIMARY KEY (scenario_id, catalogue_item_id)
);
CREATE INDEX IF NOT EXISTS idx_scenario_links_item ON scn.scenario_catalogue_links (catalogue_item_id, coverage);

-- Never deleted, and only status, ownership and the register bookkeeping change (T-02, DR-65). The trigger is the
-- second layer under the grants: akr_app has no DELETE either.
CREATE OR REPLACE FUNCTION scn.register_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'register rows are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF TG_TABLE_NAME = 'cbi_catalogue_items'
       AND (NEW.ref <> OLD.ref OR NEW.catalogue_version <> OLD.catalogue_version OR NEW.text <> OLD.text
            OR NEW.kind <> OLD.kind) THEN
        RAISE EXCEPTION 'a catalogue item is immutable; install a new catalogue version'
            USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF TG_TABLE_NAME = 'scenarios' AND (NEW.code <> OLD.code OR NEW.pack <> OLD.pack) THEN
        RAISE EXCEPTION 'scenario code and pack never change' USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN NEW;
END $$;
DO $$ BEGIN
    CREATE TRIGGER trg_cbi_items_guard BEFORE UPDATE OR DELETE ON scn.cbi_catalogue_items
        FOR EACH ROW EXECUTE FUNCTION scn.register_guard();
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    CREATE TRIGGER trg_scenarios_guard BEFORE UPDATE OR DELETE ON scn.scenarios
        FOR EACH ROW EXECUTE FUNCTION scn.register_guard();
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN
    CREATE TRIGGER trg_scenario_links_guard BEFORE DELETE ON scn.scenario_catalogue_links
        FOR EACH ROW EXECUTE FUNCTION scn.register_guard();
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

GRANT SELECT, INSERT ON scn.cbi_catalogue_items TO akr_app;
GRANT UPDATE (status, superseded_by) ON scn.cbi_catalogue_items TO akr_app;
GRANT SELECT, INSERT ON scn.scenarios TO akr_app;
GRANT UPDATE (name, summary, typologies, needs, external_needs, observational, research_grade, phase2_candidate,
              requires_flag, visibility, register_version, status, owner_id, updated_at, version)
    ON scn.scenarios TO akr_app;
GRANT SELECT, INSERT ON scn.scenario_catalogue_links TO akr_app;
GRANT UPDATE (coverage, note, register_version) ON scn.scenario_catalogue_links TO akr_app;
GRANT SELECT ON scn.cbi_catalogue_items, scn.scenarios, scn.scenario_catalogue_links TO akr_readonly;

-- The register is detection logic: what the bank monitors and how (DR-64). Catalogue text is public regulation.
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'scn', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('cbi_catalogue_items', 'internal', ARRAY['id','catalogue_version','ref','article','number','kind','chapter','title',
                                            'text','source_doc','research_grade','counter_only','status',
                                            'superseded_by','installed_at']),
  ('scenarios', 'detection-logic',    ARRAY['id','code','family','pack','name','summary','typologies','needs',
                                            'external_needs','observational','research_grade','phase2_candidate',
                                            'requires_flag','visibility','register_version','status','owner_id',
                                            'created_at','updated_at','version']),
  ('scenario_catalogue_links', 'detection-logic', ARRAY['scenario_id','catalogue_item_id','coverage','note',
                                            'register_version'])
) AS v(t, cls, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('scn', 'cbi_catalogue_items',      'R9', 'retirement', NULL, 'Superseded items kept: old reports cite them'),
 ('scn', 'scenarios',                'R9', 'retirement', NULL, 'Parameter history at least seven years (S-58, phase 16)'),
 ('scn', 'scenario_catalogue_links', 'R9', 'retirement', NULL, 'As its scenario')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

```sql
-- migration 0021_readiness_gap_report.sql
-- Phase 06. Readiness grades per scenario and scope, the bank's not-covered declarations, and gap report runs
-- (MODULES M7, DATA-MODEL s.10, DR-64, DR-66, DR-67).

-- Current state, recomputed in place by coverage_grade; history lives in the sealed run documents. A declaration is
-- the bank's answer to a grade C or D and survives regrading.
CREATE TABLE IF NOT EXISTS scn.readiness_grades (
    id                  UUID PRIMARY KEY,
    scenario_id         UUID NOT NULL REFERENCES scn.scenarios(id) ON DELETE RESTRICT,
    scope               TEXT NOT NULL CHECK (scope IN ('bank', 'source')),
    source_id           UUID REFERENCES admin.sources(id) ON DELETE RESTRICT,
    grade               CHAR(1) NOT NULL CHECK (grade IN ('A', 'B', 'C', 'D')),
    -- [{"attribute": ..., "state": "absent" | "unpopulated" | "unassessed" | "degraded" | "declaration_required"
    --   | "fallback", "source_ref": ...}, {"external": "customs"}, {"observational": true}]
    reasons             JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(reasons) = 'array'),
    missing_fields      TEXT[] NOT NULL DEFAULT '{}',          -- canonical attribute keys, for filters and the report
    basis               JSONB NOT NULL,                        -- profile run ids, catalogue and register versions, flags
    assessed_at         TIMESTAMPTZ NOT NULL,
    declaration_status  TEXT NOT NULL DEFAULT 'none' CHECK (declaration_status IN ('none', 'open', 'expired',
                                                                                   'lapsed')),
    interim_route       TEXT CHECK (interim_route IN ('observation_log', 'manual_review', 'screening',
                                                      'external_feed', 'request_for_information', 'risk_accepted')),
    interim_detail      TEXT CHECK (interim_detail IS NULL OR length(interim_detail) <= 2000),
    owner_id            UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    reason              TEXT CHECK (reason IS NULL OR length(reason) <= 2000),
    review_due          DATE,
    declared_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    declared_at         TIMESTAMPTZ,
    version             INT NOT NULL DEFAULT 1,
    CHECK ((scope = 'source') = (source_id IS NOT NULL)),
    CHECK (scope = 'bank' OR declaration_status = 'none'),
    CHECK ((declaration_status = 'none')
           = (interim_route IS NULL AND owner_id IS NULL AND reason IS NULL AND review_due IS NULL
              AND declared_by IS NULL AND declared_at IS NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_readiness_bank   ON scn.readiness_grades (scenario_id) WHERE scope = 'bank';
CREATE UNIQUE INDEX IF NOT EXISTS uq_readiness_source ON scn.readiness_grades (scenario_id, source_id)
    WHERE scope = 'source';
CREATE INDEX IF NOT EXISTS idx_readiness_grade        ON scn.readiness_grades (scope, grade);
CREATE INDEX IF NOT EXISTS idx_readiness_source       ON scn.readiness_grades (source_id) WHERE source_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_readiness_owner        ON scn.readiness_grades (owner_id) WHERE owner_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_readiness_declared_by  ON scn.readiness_grades (declared_by)
    WHERE declared_by IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_readiness_review_due   ON scn.readiness_grades (review_due)
    WHERE declaration_status = 'open';

DO $$ BEGIN
    CREATE TYPE scn.gap_report_status AS ENUM ('queued', 'running', 'completed', 'failed');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS scn.gap_report_runs (
    id                  UUID PRIMARY KEY,
    status              scn.gap_report_status NOT NULL DEFAULT 'queued',
    requested_by        UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    catalogue_version   TEXT,
    register_version    TEXT,
    source_ids          UUID[] NOT NULL DEFAULT '{}',          -- the sources whose availability was used
    profile_run_ids     UUID[] NOT NULL DEFAULT '{}',          -- the discovery runs behind the grades
    data_as_of          JSONB,                                 -- business date per branch, newest posting date
    -- counts by grade (items and scenarios), coverage measures (S-54), undeclared and expired declarations
    summary             JSONB,
    document_object_id  UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,   -- the sealed JSON report
    pack_ids            UUID[] NOT NULL DEFAULT '{}',          -- cbi_gap ar/en and vendor_request ar/en
    error_code          TEXT CHECK (error_code IN ('no_profile_run', 'register_missing', 'render_failed',
                                                   'internal')),
    started_at          TIMESTAMPTZ,
    finished_at         TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    CHECK ((status = 'failed') = (error_code IS NOT NULL)),
    CHECK ((status IN ('completed', 'failed')) = (finished_at IS NOT NULL)),
    CHECK (status <> 'completed' OR (document_object_id IS NOT NULL AND cardinality(pack_ids) = 4))
);
-- One run at a time: two runs would publish the same pack version twice.
CREATE UNIQUE INDEX IF NOT EXISTS uq_gap_report_runs_active ON scn.gap_report_runs ((true))
    WHERE status IN ('queued', 'running');
CREATE INDEX IF NOT EXISTS idx_gap_report_runs_created  ON scn.gap_report_runs (created_at DESC, id DESC);
CREATE INDEX IF NOT EXISTS idx_gap_report_runs_req_by   ON scn.gap_report_runs (requested_by);
CREATE INDEX IF NOT EXISTS idx_gap_report_runs_document ON scn.gap_report_runs (document_object_id)
    WHERE document_object_id IS NOT NULL;

-- Phase 02 packs: two more kinds (the integration profile of phase 05 and the vendor-request annex).
ALTER TABLE admin.packs DROP CONSTRAINT IF EXISTS packs_kind_check;
ALTER TABLE admin.packs ADD CONSTRAINT packs_kind_check
    CHECK (kind IN ('board', 'case', 'report', 'assessor', 'correspondent', 'validator', 'cbi_gap',
                    'integration_profile', 'vendor_request'));

GRANT SELECT, INSERT ON scn.readiness_grades TO akr_app;
GRANT UPDATE (grade, reasons, missing_fields, basis, assessed_at, declaration_status, interim_route,
              interim_detail, owner_id, reason, review_due, declared_by, declared_at, version)
    ON scn.readiness_grades TO akr_app;
GRANT SELECT, INSERT ON scn.gap_report_runs TO akr_app;
GRANT UPDATE (status, catalogue_version, register_version, source_ids, profile_run_ids, data_as_of, summary,
              document_object_id, pack_ids, error_code, started_at, finished_at, version)
    ON scn.gap_report_runs TO akr_app;
GRANT SELECT ON scn.readiness_grades, scn.gap_report_runs TO akr_readonly;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'scn', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('readiness_grades', 'detection-logic', ARRAY['id','scenario_id','scope','source_id','grade','reasons',
                                                'missing_fields','basis','assessed_at','declaration_status',
                                                'interim_route','interim_detail','owner_id','reason','review_due',
                                                'declared_by','declared_at','version']),
  ('gap_report_runs',  'detection-logic', ARRAY['id','status','requested_by','catalogue_version','register_version',
                                                'source_ids','profile_run_ids','data_as_of','summary',
                                                'document_object_id','pack_ids','error_code','started_at',
                                                'finished_at','created_at','version'])
) AS v(t, cls, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('scn', 'readiness_grades', 'R9', 'retirement', NULL, 'Current state; as its scenario'),
 ('scn', 'gap_report_runs',  'R9', 'age', 'created_at', 'Run records; the sealed document and packs are evidence (R6)')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

```sql
-- migration 0022_coverage_settings.sql
-- Phase 06. Coverage settings, schedules and the observation-log flag that phase 22 turns on.

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('coverage.declaration_review_max_days', '180', 'integer', NULL, TRUE,
  '{"ar": "أقصى مدة لمراجعة إعلان عدم التغطية (أيام)", "en": "Longest review period of a not-covered declaration (days)"}'),
 ('coverage.report_max_age_days',         '30', 'integer', NULL, FALSE,
  '{"ar": "عمر تقرير الفجوات الذي يُعد عنده قديماً (أيام)", "en": "Age at which the gap report is shown as out of date (days)"}'),
 -- DR-38: the CBS-held judgement fields (PEP flag, risk level, expected activity) the bank declares maintained.
 -- Governed: adding a key makes a CBS value a rating and scenario input, so it goes through a change request.
 ('coverage.maintained_fields',           '[]', 'json', NULL, TRUE,
  '{"ar": "حقول يصرّح المصرف بأنها محدّثة", "en": "Judgement fields the bank declares maintained"}'),
 ('coverage.candidates_max',              '5', 'integer', NULL, FALSE,
  '{"ar": "أقصى عدد للحقول المرشحة لكل سمة", "en": "Maximum field candidates per attribute"}'),
 ('schedule.coverage_sweep',              '"0 40 2 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مراجعة إعلانات عدم التغطية", "en": "Not-covered declaration sweep schedule"}')
ON CONFLICT (key) DO NOTHING;

INSERT INTO admin.feature_flags (key, enabled, label) VALUES
 ('branch.observations_live', FALSE, '{"ar": "سجل ملاحظات الفروع مفعّل", "en": "Branch observation log live"}'),
 ('pack.islamic',             FALSE, '{"ar": "حزمة الصيرفة الإسلامية", "en": "Islamic banking pack"}')
ON CONFLICT (key) DO NOTHING;
```

`detection-logic` is the phase 00 classification for what the bank monitors and how (DR-64): readable only by the
audiences of section 3.8, never by `self`. `admin.packs` keeps its phase 02 grants; the new kinds change nothing else.
The `packs_kind_check` name is PostgreSQL's default for the inline CHECK of phase 02 (`<table>_<column>_check`); the
migration test asserts the constraint exists under that name before dropping it, so a renamed constraint fails loudly.

## 2. DDD Implementation (Rust)

**Crate placement** (workspace pinned in phase 00; layer rules in `ddd.md`):

```
backend/
├── packs/                            # signed release data (C-38): listed in the release manifest with sha256
│   ├── cbi/catalogue.2025-01-08.json        # the 173 items (section 3.1)
│   └── scenarios/register.v1.0.0.json       # the 161 scenarios and their links to the items
├── schemas/
│   ├── cbi-catalogue.v1.json, scenario-register.v1.json   # validate the two pack files
│   └── cbi-gap-report.v1.json                # the sealed report document (section 3.5)
├── templates/                        # minijinja, autoescape on (phase 02 RenderService)
│   ├── gap_report.ar.html.j2, gap_report.en.html.j2
│   ├── vendor_request.ar.html.j2, vendor_request.en.html.j2
│   └── integration_profile.ar.html.j2, integration_profile.en.html.j2
├── crates/
│   ├── domain/    # + coverage (CatalogueItem, Scenario, ScenarioLink, Need, Use, Grade, ReadinessGrade,
│   │              #   Declaration, GapReportRun, ReadinessGrader (pure), FieldCandidateFinder (pure), the pack-file
│   │              #   model and its validation); discovery::IntegrationProfileBuilder (phase 05) gains a caller
│   ├── db/        # + Pg repositories for the five tables; PgAvailabilitySource over phase 05 rows
│   ├── authz/     # + resource adapters Scenario, ReadinessGrade, GapReportRun, Pack
│   └── jobs/      # + kinds coverage_grade, gap_report_build, pack_build, coverage_sweep (all akr-worker)
├── api/           # + routers catalogue, coverage, packs; GET /discovery/field-candidates on the discovery router
├── worker/        # + the four job handlers; GapReportService, the integration profile render
└── ctl/           # + packs seed, packs verify-data
tools/build_scn_register.py           # builds the two pack files once from research file 27; output committed
```

`tools/build_scn_register.py` reads `research/27-transaction-monitoring-scenarios.md` (Part A family table, the
scenario entries of B4-B21, and the B22 mapping table between `MAP-START` and `MAP-END`, 173 rows in the form
`| Art.X no.N | text | Implemented by | Also covered by | Grade | note |`) and writes the two JSON files. It runs once
per research revision, by a developer; its output is reviewed and committed, and the release build only signs and
lists the files. The script fails when a row count differs from 173, when a code in a mapping row has no scenario
entry, or when a scenario entry names a family outside the 18.

- **Pack file shapes:**
  - Catalogue: `{catalogue_version: "2025-01-08", chapter: "6", source_doc: "...", items: [{ref, article, number,
    kind, title: {ar, en}, text: {ar, en}, page, research_grade, counter_only}]}`.
  - Register: `{register_version: "1.0.0", families: [{code, name: {ar, en}}] (18: CASH, XFER, TRADE, ISL, DIGI, LOAN,
    ACCT, PEP, PUB, CORR, TF, PF, INS, DORM, CHQ, FX, OTH, OBS), scenarios: [{code, family, name, summary,
    typologies, needs: [{attribute, use}], external_needs, observational, research_grade, phase2_candidate,
    visibility, requires_flag}], links: [{ref, scenario, coverage, note?}]}`. Rules applied by the builder and
    checked again by `packs seed`: every scenario of family `PUB` is `visibility = restricted`; every `ISL` scenario
    has `requires_flag = "pack.islamic"`; every `OBS` scenario is `observational`; an "Implemented by" code becomes a
    `full` link, an "Also covered by" code a `partial` link, and Art.3 no.9 (no implementing scenario in the
    research) gets its only link, to SCN-DIGI-14, as `proxy`.
- **Domain entities** (`Debug, Serialize, Deserialize, FromRow, ToSchema`; fields mirror section 1): `CatalogueItem`,
  `Scenario`, `ScenarioLink`, `ReadinessGrade`, `GapReportRun` (status as `#[sqlx(type_name =
  "scn.gap_report_status")]`).
- **Value objects:** `CatalogueRef` (parses `Art.N no.M`, ordered by article then number), `ScenarioCode`, `Family`,
  `Pack` (`Cbi`, `Akr`, `Islamic`, `Regime`, `Bank`), `Need { attribute: AttributeKey, use_: Use }`, `Use`
  (`Required`, `Fallback`, `Optional`), `Coverage` (`Full`, `Partial`, `Proxy`), `Grade` (`A`, `B`, `C`, `D`;
  `Ord` with D worst; `Grade::worst(iter)`, `Grade::cap(self, Grade)`), `Term` (`Operable` for A, `Degraded` for B,
  `Blocked` for C and D; C-48), `Reason` (the `reasons` items of section 1), `Scope` (`Bank`, `Source(Uuid)`),
  `InterimRoute`, `DeclarationStatus`, `Declaration { interim_route, interim_detail, owner_id, reason, review_due }`,
  `GradeBasis { profile_run_ids, catalogue_version, register_version, flags: {observations_live, islamic},
  maintained_fields }`.
- **Create/Update structs:** `NewCatalogueItem`, `NewScenario`, `ScenarioRegisterUpdate` (the granted columns, all
  `Option`), `NewLink`, `GradeUpsert { scenario_id, scope, grade, reasons, missing_fields, basis }`,
  `DeclarationInput` (`Deserialize, ToSchema`: `interim_route`, `interim_detail?`, `owner_id`, `reason`,
  `review_due`), `NewGapReportRun`.
- **Repository traits** (domain layer; `Pg*` implementations in `db`):

```rust
#[async_trait]
pub trait CatalogueRepository: Send + Sync {
    async fn insert_missing(&self, tx: &mut Tx, items: &[NewCatalogueItem]) -> Result<u64, RepoError>;  // ON CONFLICT DO NOTHING
    async fn supersede(&self, tx: &mut Tx, old_version: &str, new_version: &str) -> Result<u64, RepoError>;
    async fn active(&self, conn: &mut PgConnection) -> Result<Vec<CatalogueItem>, RepoError>;   // ordered by ref
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<CatalogueItem>, RepoError>;
}

#[async_trait]
pub trait ScenarioRepository: Send + Sync {
    async fn insert_missing(&self, tx: &mut Tx, s: &[NewScenario]) -> Result<u64, RepoError>;
    /// Only when `register_version` is newer (semver compare in the statement's WHERE); bumps `version`.
    async fn update_from_register(&self, tx: &mut Tx, code: &str, u: &ScenarioRegisterUpdate) -> Result<bool, RepoError>;
    async fn upsert_links(&self, tx: &mut Tx, links: &[NewLink]) -> Result<u64, RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &ScenarioFilter, page: Keyset) -> Result<Page<Scenario>, RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<Scenario>, RepoError>;
    async fn links_for_scenario(&self, conn: &mut PgConnection, id: Uuid) -> Result<Vec<ScenarioLink>, RepoError>;
    async fn all_with_links(&self, conn: &mut PgConnection) -> Result<RegisterSnapshot, RepoError>;   // the grader input
}

#[async_trait]
pub trait ReadinessRepository: Send + Sync {
    /// Upsert on the partial unique index of the scope; never touches the declaration columns.
    async fn upsert_grades(&self, tx: &mut Tx, rows: &[GradeUpsert], assessed_at: DateTime<Utc>) -> Result<u64, RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &ReadinessFilter, page: Keyset) -> Result<Page<ReadinessGrade>, RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<ReadinessGrade>, RepoError>;
    async fn bank_scope(&self, conn: &mut PgConnection) -> Result<Vec<ReadinessGrade>, RepoError>;
    async fn declare(&self, tx: &mut Tx, id: Uuid, d: &Declaration, by: Uuid, expected_version: i32)
        -> Result<ReadinessGrade, RepoError>;                     // 409 on a version race
    /// `open` rows whose grade is now A or B -> `lapsed`; returns the ids for the audit events.
    async fn lapse_resolved(&self, tx: &mut Tx) -> Result<Vec<Uuid>, RepoError>;
    /// `open` rows with `review_due < today` -> `expired`.
    async fn expire_due(&self, tx: &mut Tx, today: NaiveDate) -> Result<Vec<Uuid>, RepoError>;
}

#[async_trait]
pub trait GapReportRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, r: &NewGapReportRun) -> Result<GapReportRun, RepoError>;   // uq active -> 409
    async fn start(&self, tx: &mut Tx, id: Uuid) -> Result<Option<GapReportRun>, RepoError>;
    async fn complete(&self, tx: &mut Tx, id: Uuid, c: &RunCompletion) -> Result<GapReportRun, RepoError>;
    async fn fail(&self, tx: &mut Tx, id: Uuid, code: GapReportError) -> Result<GapReportRun, RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<GapReportRun>, RepoError>;
    async fn latest_completed(&self, conn: &mut PgConnection) -> Result<Option<GapReportRun>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, page: Keyset) -> Result<Page<GapReportRun>, RepoError>;
}
```

- **Repository notes:** lists are keyset-paged (scenarios by `code`; readiness by `(grade DESC, scenario code)`, D
  first; runs by `(created_at DESC, id DESC)`); `upsert_grades` is one `INSERT ... ON CONFLICT (scenario_id) WHERE
  scope = 'bank'` statement per scope (the partial index is the arbiter) and bumps `version` only when the grade,
  reasons or missing fields changed, so a regrade that changes nothing leaves `If-Match` values valid;
  `all_with_links` loads the 161 scenarios and their links in two statements (no N+1).

- **Ports:**

```rust
/// The latest field availability per non-disabled source (phase 05 rows of the newest completed profile run).
#[async_trait]
pub trait AvailabilitySource: Send + Sync {
    async fn latest(&self, conn: &mut PgConnection) -> Result<Vec<SourceAvailability>, RepoError>;
}
pub struct SourceAvailability { pub source_id: Uuid, pub source_code: String, pub run_id: Uuid,
                                pub finished_at: DateTime<Utc>, pub rows: Vec<FieldAvailability> }

/// External feeds the bank has connected (customs, HR, device logs, ...). Empty in P0: no feed exists until the
/// phases that bring one (screening feeds in 13; device and channel data are P2, S-20, DR-394); the grader asks it by name.
pub trait ExternalFeedRegistry: Send + Sync {
    fn connected(&self, feed: &str) -> bool;
}
```

- **utoipa schemas:** `CatalogueItemView` (ref, article, number, kind, chapter, title, text, source_doc, status,
  `scenarios: [code]` of the full and partial links whose scenario is `normal` visibility; no grade, no coverage
  kind, no restricted code), `ScenarioView` (all register columns, links with item refs, the bank-scope grade),
  `ScenarioSummary` (list row), `ReadinessView` (grade, term, reasons with attribute labels, missing fields,
  declaration, `scenario: {code, family, name}`), `DeclarationInput`,
  `GapReportView` (the sealed document, section 3.5, plus `stale: bool` from `coverage.report_max_age_days`),
  `GapReportRunView`, `GapReportRunAccepted` (`run_id`, `Location`), `PackRequest` (`kind`, `scope`, `lang?`),
  `PackView` (kind, scope, lang, version, sha256, size, created_by, created_at, `status` of the build job: `queued`,
  `ready`, `failed`), `PackAccepted`, `FieldCandidateView` (`attribute`, `candidates: [{source_ref, score,
  reasons: [code]}]`).

- **Route table** (base `/api/v1`; every route names its Cedar action, section 3.8; **SU** = step-up assertion
  required):

| Method | Path | Scope | Description |
|-|-|-|-|
| GET | /cbi-catalogue | self | The 173 CBI items with their implementing scenario codes; no grade, no restricted code (`cbi_catalogue.read`) |
| GET | /scenarios | config.author | Scenario register: `pack?`, `family?`, `grade?`, `q?` (code or name prefix) (`scenario.list`) |
| GET | /scenarios/{id} | config.author | Detail: needs, external needs, links with coverage kind, bank and source grades (`scenario.read`) |
| GET | /coverage/readiness | config.author, dashboard.read | Readiness rows: `scope=bank` (default) or `source` with `source_id`; `grade?`; `declaration_status?` (`readiness.read`) |
| PUT | /coverage/readiness/{id}/declaration | config.approve | Record or replace the not-covered declaration on a bank-scope row graded C or D; `If-Match`; **SU** (`readiness.declare`) |
| GET | /coverage/cbi-gap-report | dashboard.read, data.steward | The latest completed report document, or `?run_id=`; `stale` flag (`gap_report.read`) |
| POST | /coverage/cbi-gap-report/runs | dashboard.read, data.steward | Regrade and build the report and its four packs; 202 (`gap_report.run`) |
| GET | /coverage/cbi-gap-report/runs | dashboard.read, data.steward | Runs, newest first (`gap_report.read`) |
| GET | /coverage/cbi-gap-report/runs/{id} | dashboard.read, data.steward | One run: status, summary, pack ids, error code (`gap_report.read`) |
| POST | /packs | dashboard.read, data.steward | Build a pack: `integration_profile {source_id}` or `cbi_gap` / `vendor_request` rebuild `{run_id}`; 202 (`pack.create`) |
| GET | /packs/{id} | dashboard.read, data.steward | Pack metadata and build status (`pack.read`) |
| GET | /packs/{id}/download | dashboard.read, data.steward | Stream the PDF through `EvidenceStore::open`; access event `download`; **SU** (`pack.download`) |
| GET | /discovery/field-candidates | data.steward | Ranked candidate columns for attributes discovery could not place: `source_id`, `attribute?` (`discovery.read`) |

API.md lists `POST /coverage/cbi-gap-report/runs` under `data.steward` and the pack routes under `dashboard.read`;
both now take either scope, because the steward builds the report and the MLRO reads and presents it (DS holds no
`dashboard.read`). Cedar narrows each pack kind to its audience (section 3.8).

- **Router functions:** `catalogue::router()` (4 routes), `coverage::router()` (6), `packs::router()` (3), each
  `-> OpenApiRouter<AppState>`, merged in `build_app` under the authz layer; `discovery::router()` gains one route
  (9). `POST /coverage/cbi-gap-report/runs` and `POST /packs` take `Idempotency-Key` (phase 00); the declaration takes
  `If-Match` with the row's `version`. `GET /packs/{id}/download` sets `Content-Type: application/pdf`,
  `Content-Disposition: attachment; filename="<kind>-<lang>-v<version>.pdf"`, `Cache-Control: no-store` and
  `X-Content-SHA256`.

## 3. Business Logic

### 3.1 Installing the catalogue and the register (`CatalogueInstaller`, `akr-ctl packs seed`; S-01, S-04, T-02, DR-65)

`akr-ctl packs seed [--dry-run]` (zone Z5; the OS user and PAM session recorded as in phase 00; run by the installer
after `db migrate` and `db harden`, and again by every upgrade):
1. Read the release manifest; for each of the two pack files, compare its SHA-256 with the manifest entry and verify
   the manifest signature (phase 00 release signing). A mismatch -> exit 1, nothing written.
2. Validate each file against its schema (`cbi-catalogue.v1.json`, `scenario-register.v1.json`) and the register
   rules of section 2 (family set, `PUB` restricted, `ISL` flag, `OBS` observational, every link names a known
   `ref` and code, no code ending 901-999). A failure -> exit 1, nothing written.
3. One transaction (as `akr_app`):
   1. `CatalogueRepository::insert_missing` with UUIDv7 ids: items already present for this `catalogue_version` are
      left as they are (the guard trigger forbids changing their text).
   2. A catalogue version newer than the newest installed one: `supersede(old, new)` sets `status = superseded`
      and `superseded_by` per `ref` on the old rows (a `ref` absent from the new version stays `active` and is listed
      as a warning: the CBI does not withdraw items silently, so a person must look).
   3. `ScenarioRepository::insert_missing`; pack assignment: a scenario with at least one `full` or `partial` link
      is `cbi` (99 in v1.0.0), else `akr` (62). `ISL` scenarios keep `cbi` or `akr` by the same rule; the flag, not the
      pack, decides whether they are graded.
   4. `update_from_register` for each existing code when the file's `register_version` is newer: name, summary,
      typologies, needs, external needs, observational, research grade, phase-2 flag, requires flag, visibility. A
      code present in the database but missing from the file is never retired by the seed (a `cbi` one cannot be;
      an `akr` one waits for phase 16's retirement through a change request) and is reported as a warning.
   5. `upsert_links` (the link set only grows; a link the file drops keeps its row and is reported).
   6. Audit `cbi_pack.seeded {catalogue_version, register_version, items, scenarios, links, added: {items,
      scenarios, links}, warnings}` with the system principal and the operator's OS user and host in the details.
   7. Enqueue `coverage_grade {reason: "seed"}` (coalesce key `coverage_grade`).
4. A second run with the same files writes nothing and audits `added` as zeros (idempotent; the verification runs it
   twice). `--dry-run` prints the counts and warnings and rolls back.

`akr-ctl packs verify-data` asserts the installed data, exit 2 on any failure: 173 active items (86 `scenario` in
articles 1-3, 87 `indicator` in articles 4-14); 161 scenarios in 18 families; 99 `cbi` and 62 `akr`; every active
item except Art.3 no.9 has at least one `full` link; Art.3 no.9 has exactly one link, `proxy`, to SCN-DIGI-14;
research grades of the items A 47, B 80, C 26, D 20; every `PUB` scenario restricted; every `counter_only` item graded
D in the research. akr-api's readiness check `cbi_pack` runs the count part (173, 161) at start-up; a deployment
without the seed is not ready (the gap report would grade nothing).

### 3.2 Readiness grading (`ReadinessGrader`, domain, pure; S-03, C-48, DR-38, DR-64)

`grade(register, availability: &[SourceAvailability], maintained: &[AttributeKey], feeds: &dyn
ExternalFeedRegistry, flags: Flags, catalogue_attrs: &AttributeCatalogue) -> Grading` returns the scenario grades for
every scope and the item grades for the bank scope. No IO; the job of section 3.3 gathers the inputs.

**Attribute state per scope.** Source scope: the source's row for the attribute in its latest completed profile run.
Bank scope: the best quality over every non-disabled source (order `good` > `partial` > `poor` > `unpopulated` >
`unassessed` > `absent`), keeping the `source_ref` of the source that supplied it. An attribute with no row anywhere
is `absent`. Then, for an attribute whose catalogue entry has `needs_declaration = true` and which is not in
`coverage.maintained_fields`, the state is `declaration_required` whatever the quality (DR-38: the value is unknown,
never low).

**Scenario grade** (per scope; each rule adds a `reasons` item; the grade is the worst of the rules that fire, A when
none does):
1. `requires_flag` set and the flag off -> grade D with the single reason `{"pack_not_enabled": "<flag>"}`; the
   scenario is left out of every count and of the item grades (section 3.5 lists it apart). No further rule runs.
2. Each `required` need: `good` -> nothing; `partial` or `poor` -> B, state `degraded`; `unpopulated`, `absent`,
   `unassessed` -> D, state as is, and the attribute joins `missing_fields`; `declaration_required` -> D, state
   `declaration_required`, joins `missing_fields`.
3. Each `fallback` need (a segment-percentile or peer fallback exists, S-10): `good` -> nothing; anything else -> B,
   state `fallback` (the attribute joins `missing_fields` when it is missing, not when it is only degraded).
4. Each `optional` need that is not `good`: listed in `reasons`, no effect on the grade.
5. Each `external_needs` entry the registry does not report connected -> C, reason `{"external": "<feed>"}`.
6. `observational` and the flag `branch.observations_live` off -> D, reason `{"observational": true}`.

Terms (C-48): A operable, B degraded, C and D blocked. From phase 10 a missing or draft mapping for a needed
attribute also degrades or blocks (the mapping-completeness half of C-48); this phase reads availability only.

**Item grade** (bank scope only; the catalogue is bank-wide):
1. Candidates: the item's links whose scenario is not excluded by rule 1. Each candidate's grade is capped by the
   link: `full` as computed; `partial` at best B; `proxy` at best C (DR-64). The item grade is the best capped grade;
   no candidate -> D, reason `no_scenario`.
2. `counter_only` items (only a person can observe them) are D until `branch.observations_live` is on, whatever the
   candidates say.
3. The item's `missing_fields` are those of the candidate that gave the grade, and, when the grade is C or D, the
   union over all candidates (the vendor annex needs the whole list).

**Consistency test** (unit test over the shipped pack files): with every attribute in the catalogue `good` on one
source, all three judgement fields maintained, no external feed and both flags off, every item graded C or D in the
research computes C or D, every A item computes A, and every B item computes A or B, B exactly when its best link is
`partial` or a needed attribute is `fallback`-only; the full list of computed grades is compared with the committed
golden `backend/packs/scenarios/grade-golden.v1.0.0.json`, so a register or rule change shows up as a diff a person
reviews.

### 3.3 The `coverage_grade` job (akr-worker; DR-64)

Payload `{reason: "seed" | "profile_run" | "flag" | "setting" | "report"}`; coalesce key `coverage_grade` (bursts
collapse into one run); concurrency 1. Enqueued by: `packs seed`; phase 05 `discovery_seal_report` after a profile
run's report is sealed; the phase 03 flag service when `branch.observations_live` or `pack.islamic` changes; the phase
04 activation of the setting `coverage.maintained_fields`. Steps:
1. Load the register (`all_with_links`), `AvailabilitySource::latest`, the setting, the flags, the attribute
   catalogue.
2. `ReadinessGrader::grade`.
3. One transaction: `upsert_grades` for the bank scope and each source scope (source rows only for sources that have
   a completed profile run); then `lapse_resolved`: every `open` declaration whose row is now A or B becomes `lapsed`
   (kept for history), audit `readiness.declaration_lapsed {scenario_code, grade}` each.
4. Audit `readiness.graded {reason, scope_counts: {bank: {A, B, C, D}, sources: n}, items: {A, B, C, D},
   profile_run_ids}`; gauge `akr_readiness_items{grade}`.
5. A source that is disabled keeps its old source-scope rows (history; the bank scope no longer uses it).

### 3.4 Not-covered declarations and the sweep (`DeclarationService`; S-03, L-02, DR-66)

`PUT /coverage/readiness/{id}/declaration` (akr-api):
1. Cedar `readiness.declare` (M only, section 3.8); a single-use step-up assertion of class `readiness_declare`
   (phase 01 `consume`, header `X-StepUp-Assertion`) else 401 `E_STEPUP_REQUIRED`; `If-Match` required (428 `E_PRECONDITION_REQUIRED`).
2. Load the row: missing -> 404; `scope = source` or grade A or B -> 409 `E_NOT_ALLOWED_IN_STATE` reason
   `not_blocked`.
3. Validate: `interim_route` in the enum; `reason` 10-2,000 characters; `owner_id` an active human user (not a
   service principal, not offboarded) -> else 422 `E_VALIDATION` `owner_not_active`; `review_due` after today and at
   most today + `coverage.declaration_review_max_days` in bank time -> else 422 `review_too_far` (or
   `review_in_past`); `interim_route = risk_accepted` needs `interim_detail` (what the bank accepts and why) -> else
   422 `detail_required`.
4. `ReadinessRepository::declare` with the version (409 `E_VERSION_CONFLICT` on a race): `declaration_status = open`,
   `declared_by`, `declared_at = now()`. Replacing an open, expired or lapsed declaration overwrites it; the audit
   event of the earlier declaration keeps the previous values, and the sealed report of each run keeps what was in force then.
5. Audit `readiness.declared {scenario_code, grade, interim_route, owner_id, review_due, replaced: bool}`.

The `coverage_sweep` job (daily, `schedule.coverage_sweep`, concurrency 1): `expire_due(today)` moves every `open`
declaration past its `review_due` to `expired` and audits `readiness.declaration_expired {scenario_code, owner_id}`
each. The report cover counts undeclared C and D items and expired declarations as open findings (section 3.5); the
phase 18 inbox picks up expiry as a task for the owner. A declaration is not an exclusion: excluding a population from
monitoring with four-eyes and an expiry is the phase 12 exclusion register (L-02's other half).

### 3.5 The CBI coverage gap report (`GapReportService`, job `gap_report_build`; S-02, S-54, DR-67)

`POST /coverage/cbi-gap-report/runs` (akr-api): Cedar `gap_report.run`; insert a `queued` run with `requested_by`
(the active-run index answers a second request with 409 `E_NOT_ALLOWED_IN_STATE` reason `run_active` and the active
run's id in `errors[]`); enqueue `gap_report_build {run_id}`; audit `gap_report.requested`; 202 with `Location`.

`gap_report_build` (akr-worker; concurrency 1; max attempts 3; a failed render retries, a data failure does not):
1. `start` (a progress marker of the job with no audit event; the lifecycle events of section 3.8 cover the run): a `queued` run becomes `running`; a run already `running` is a retry of this job and resumes from the step it reached (section 4); a `completed` or `failed` run -> done.
2. `AvailabilitySource::latest` empty -> `fail(no_profile_run)`; register empty -> `fail(register_missing)`.
3. Run the section 3.3 grading inline (same code, `reason: "report"`), so the report never shows grades older than
   the newest profile.
4. Build the document (`cbi-gap-report.v1.json`; ids, codes, labels and counts only; no customer value):
   - `cover`: bank name (both languages, from the installation file `deploy/branding.json` of phase 08 section 3.10, which the akr-worker service mounts read-only; an unreadable file is `fail(internal)`), catalogue and register versions, sources (code, name, profile run id and
     finished time), `data_as_of` per source (the business date per branch and the newest posting date, from phase 05
     profile rows), `generated_at`, `run_id`, open findings (`undeclared_blocked`, `declarations_expired`).
   - `summary`: items by grade and by term; scenarios by grade (excluded `pack_not_enabled` counted apart); the
     research baseline beside the computed counts (A 47, B 80, C 26, D 20); S-54 measures: `items_with_scenario` (172
     full or partial, 1 proxy, as counts and share of 173) and `items_data_ready` (A or B, count and share).
   - `items[]` in `ref` order: ref, kind, title and text in both languages, item grade and term, `linked`: [{scenario
     code and name, coverage, grade}] (a restricted scenario appears as `{restricted: true, coverage, grade}` with no
     code or name), missing attributes with label, CBS `module`, state, `null_pct` and `source_ref`, and the
     declaration (route, detail, owner name, review date, status) when the item's blocking scenarios carry one.
   - `bank_request`: the judgement fields that need the bank's declaration (`declaration_required`), with what the
     bank must do: declare the field maintained (a change request on `coverage.maintained_fields`) or run a KYC
     refresh first.
   - `vendor_annex`: the attributes missing or degraded anywhere, grouped by CBS `module`, each with label, state,
     `null_pct`, `source_ref` and the object it was looked for in. No item, scenario or count of dependants: the
     core-banking vendor learns which fields to supply, not where detection is blind.
   - `outcomes`: alert, case and STR columns per item, `null` until phases 17-21 fill them (S-02).
5. Validate against the schema; `put_generated` the canonical JSON (evidence kind `snapshot`, owner kind
   `gap_report_run`, owner id the run, retention R6, classification `detection-logic`).
6. Render four PDFs through phase 02 `RenderService` (templates of section 2; Arabic right to left, English left to
   right; the Arabic catalogue text is the CBI's own, the English text is marked as a translation): the report in
   Arabic and English, and the vendor request (cover letter plus the annex) in Arabic and English.
7. `PackService::publish` four times in one transaction: `cbi_gap` ar and en with scope `{run_id}`, `vendor_request`
   ar and en with scope `{run_id}`; each emits `pack.created {kind, version, lang, sha256}`. "Signed" means exactly
   this: the pack's SHA-256 sits in the anchored audit chain (phase 02) and `POST /packs/verify` (phase 24) checks a
   file against it.
8. `complete` with `document_object_id`, `pack_ids`, `summary`, `source_ids`, `profile_run_ids`, versions and
   `data_as_of`; audit `gap_report.completed {run_id, items: {A, B, C, D}, packs: 4}`; histogram
   `akr_gap_report_build_seconds`.
9. A render failure after the last attempt -> `fail(render_failed)`; any other error -> `fail(internal)`; audit
   `gap_report.failed {run_id, error_code}`. Nothing half-built is published: steps 7-8 share one transaction, and an
   evidence object stored before a failure stays as an unreferenced object for the phase 23 reconciliation.

`GET /coverage/cbi-gap-report` returns the latest completed run's sealed document (read through `EvidenceStore::open`,
so a tampered object fails closed) with `stale = true` when `finished_at` is older than `coverage.report_max_age_days`
or a profile run finished after it; no completed run -> 404 `E_NOT_FOUND`.

### 3.6 Packs on request: the integration profile PDF and rebuilds (`PackRequestService`, job `pack_build`; C-07, DR-67)

`POST /packs` (akr-api) accepts three kinds until phase 24 adds the rest; any other kind -> 422 `E_VALIDATION`
`unknown_pack_kind`:
- `integration_profile` with `{source_id}`: Cedar `pack.create` on the kind; the source must have a completed
  catalogue run (else 409 `E_NOT_ALLOWED_IN_STATE` `no_catalogue_run`).
- `cbi_gap` or `vendor_request` with `{run_id, lang?}`: a completed run; rebuilds the PDF from the run's sealed JSON
  (template fixes, a lost download), giving the next pack version. The document is not regraded.
Enqueue `pack_build {kind, scope, lang?, requested_by}` (coalesce key = kind + canonical scope + lang); audit
`pack.requested {kind, scope}`; 202 with a `PackAccepted` whose `Location` is the build's status (the pack id is known
once built; `GET /packs/{id}` of the pending id answers `status: queued`).

`pack_build` (akr-worker; concurrency 2): for `integration_profile`, call `IntegrationProfileBuilder` (phase 05,
`domain::discovery` over `db` repositories, so akr-worker builds the same document as the API without linking
`akr-sources`), validate it, render both languages from the `integration_profile` templates (the bilingual
cover, the checklist first, the minimum grants text as a monospace block kept left to right), publish two packs with
scope `{source_id, run_id}`; for a rebuild, open the sealed JSON and render. Failures audit `pack.build_failed {kind,
reason}`.

`GET /packs/{id}`: Cedar `pack.read` on the pack's kind. `GET /packs/{id}/download`: Cedar `pack.download`; a single-use step-up
assertion of class `pack_download` (401 `E_STEPUP_REQUIRED`); record the access event `download` through `AccessScope` (phase 03) before calling
`EvidenceStore::open`; audit `pack.downloaded {kind, version, lang}`. This brings forward, for these three kinds only,
the audited download that phase 02 assigned to phases 16 and 24.

### 3.7 Field candidates (`FieldCandidateFinder`, domain, pure; C-09, C-10)

`GET /discovery/field-candidates?source_id=&attribute=` (akr-api; Cedar `discovery.read`). For each attribute in the
catalogue whose source-scope state is `absent` or `unassessed` with the note `column_missing` (or the one attribute
asked for), rank the columns of the latest completed catalogue run of that source:
1. Exclude: objects that are copy, scratch, log or staging candidates (phase 05 `candidate` classification), columns
   on the deny list of the adapter pack, and objects not approved when an approved list exists (a candidate outside
   it is shown only with the reason `needs_approval`).
2. Score (0-100), each part recorded as a reason code: `alias` (the column or object name is in the adapter pack's
   alias list for the attribute, 40), `dictionary` (token overlap between the CBS field dictionary description, Arabic
   and English, folded by `text::fold`: diacritics and tatweel removed, alef, ya and ta marbuta forms unified,
   digits normalised; phase 13 extends it for names; and the attribute label, up to 25), `name`
   (column-name tokens against the attribute key, up to 15), `type` (compatible type: dates for dates, numeric with
   scale for amounts, short text for codes, 10; an incompatible type caps the score at 20), `liveness` (the object's
   phase 05 liveness score, up to 10).
3. Return the top `coverage.candidates_max` with score and reasons. Nothing is written: the steward accepts a
   candidate by setting the hint with `PATCH /sources/{id}` (phase 05), and the next profile run grades it. No values
   are read; the finder uses only catalogue and profile rows already stored.

### 3.8 Errors, audit events, Cedar actions

**Error mapping.** No new codes. Used: `E_SCOPE`; `E_FORBIDDEN`; `E_NOT_FOUND` (also the masked restricted scenario);
`E_VERSION_CONFLICT`; `E_PRECONDITION_REQUIRED`; `E_STEPUP_REQUIRED` (declaration, download); the replay
behaviour of phase 00 on the two POSTs; `E_NOT_ALLOWED_IN_STATE` with reasons `not_blocked`, `run_active`,
`no_catalogue_run`, `run_not_completed`; `E_VALIDATION` with reasons `review_too_far`, `review_in_past`,
`owner_not_active`, `detail_required`, `unknown_pack_kind`, `scope_invalid`. `CoverageError`, `GapReportError` and
`PackError` (`thiserror`) map in `api::error` or end the job; `no_profile_run` and `register_missing` are run error
codes, not HTTP answers (the run is accepted and fails visibly).

**Audit events emitted:**

| Event kind | On | Resource |
|-|-|-|
| `cbi_pack.seeded` | `akr-ctl packs seed` (versions, counts, added, warnings) | system |
| `readiness.graded` | each `coverage_grade` run (reason, counts by scope and grade, profile run ids) | system |
| `readiness.declared` | declaration recorded or replaced (scenario code, grade, route, owner, review date) | readiness_grade |
| `readiness.declaration_lapsed` | grade improved to A or B under an open declaration | readiness_grade |
| `readiness.declaration_expired` | sweep, past the review date | readiness_grade |
| `gap_report.requested`, `.completed`, `.failed` | run lifecycle (run id, counts, error code) | gap_report_run |
| `pack.requested`, `pack.build_failed`, `pack.downloaded` | pack routes and the build job (kind, scope, version, lang) | pack |

Reused: `pack.created`, `evidence.stored` (phase 02); access events (phase 03) on download.

**Cedar.** Two bundle files: `80_detection.cedar` (the range phase 03 reserved for detection: catalogue, scenarios,
readiness, the gap report) and `C0_packs.cedar` (governance range: packs, which later phases extend by kind).
Entities added to `schema.cedarschema`: `Scenario { pack: String, family: String, visibility: String }`,
`ReadinessGrade { scope: String, grade: String, scenario: Scenario }`, `GapReportRun { requested_by: User, status:
String }`, `Pack { kind: String, created_by: User }`; `PackKind` context attribute `context.pack_kind` on
`pack.create` (the pack does not exist yet).

| Action | Resource | Who (shipped bundle) |
|-|-|-|
| `cbi_catalogue.read` | System | every active human principal |
| `scenario.list`, `scenario.read` | System, Scenario | CFG, DS, MRO; a `restricted` scenario only CFG and MRO (others: 404) |
| `readiness.read` | System, ReadinessGrade | CFG, DS, MRO; M, CH, IA (the MLRO declares on these rows; added while writing phase 08); restricted rows masked as above |
| `readiness.declare` | ReadinessGrade | M |
| `gap_report.read` | System, GapReportRun | M, CH, IA, DS |
| `gap_report.run` | System | DS, M |
| `pack.create`, `pack.read`, `pack.download` | Pack (by kind) | `cbi_gap`, `vendor_request`: M, CH, IA, DS; `integration_profile`: DS, M, IA |
| `discovery.read` (field candidates) | System, Source | DS (phase 05) |

```cedar
// 80_detection.cedar (excerpt)
@id("catalogue.everyone")
permit (principal is Akr::User, action == Akr::Action::"cbi_catalogue.read", resource)
when { principal.kind == "human" && principal.status == "active" };

@id("scenario.register_readers") @role("CFG,DS,MRO")
permit (principal, action in [Akr::Action::"scenario.list", Akr::Action::"scenario.read",
        Akr::Action::"readiness.read"], resource)
when { principal.roles.containsAny(["CFG", "DS", "MRO"]) };

// Restricted scenarios (the PUB family) are read by the scenario authors and model risk only; others get 404.
@id("scenario.restricted") @deny("masked")
forbid (principal, action in [Akr::Action::"scenario.read", Akr::Action::"readiness.read"], resource)
when { (resource is Akr::Scenario && resource.visibility == "restricted"
        || resource is Akr::ReadinessGrade && resource.scenario.visibility == "restricted")
       && !principal.roles.containsAny(["CFG", "MRO"]) };

@id("readiness.oversight_readers") @role("M,CH,IA")
permit (principal, action == Akr::Action::"readiness.read", resource)
when { principal.roles.containsAny(["M", "CH", "IA"]) };

@id("readiness.mlro_declares") @role("M")
permit (principal, action == Akr::Action::"readiness.declare", resource is Akr::ReadinessGrade)
when { principal.roles.contains("M") && resource.scope == "bank" && context.aal == "aal2" };

@id("gap_report.readers") @role("M,CH,IA,DS")
permit (principal, action == Akr::Action::"gap_report.read", resource)
when { principal.roles.containsAny(["M", "CH", "IA", "DS"]) };

@id("gap_report.runners") @role("DS,M")
permit (principal, action == Akr::Action::"gap_report.run", resource)
when { principal.roles.containsAny(["DS", "M"]) };
```

```cedar
// C0_packs.cedar (excerpt)
@id("pack.coverage") @role("M,CH,IA,DS")
permit (principal, action in [Akr::Action::"pack.create", Akr::Action::"pack.read", Akr::Action::"pack.download"],
        resource)
when { principal.roles.containsAny(["M", "CH", "IA", "DS"])
       && ((resource is Akr::Pack && ["cbi_gap", "vendor_request"].contains(resource.kind))
           || (context has pack_kind && ["cbi_gap", "vendor_request"].contains(context.pack_kind))) };

@id("pack.integration_profile") @role("DS,M,IA")
permit (principal, action in [Akr::Action::"pack.create", Akr::Action::"pack.read", Akr::Action::"pack.download"],
        resource)
when { principal.roles.containsAny(["DS", "M", "IA"])
       && ((resource is Akr::Pack && resource.kind == "integration_profile")
           || (context has pack_kind && context.pack_kind == "integration_profile")) };
```

Attributes evaluated: `principal.roles`, `delegated_roles`, `kind`, `status`; `resource.visibility`, `scope`, `kind`;
`context.aal`, `context.stepup` (declaration, download), `context.pack_kind`. The restricted masking is a `masked` deny,
so the refusal is 404 and an access event `deny` (phase 03). Bundle tests: no role but M can declare; A1, A2, KYC, L
and OPS reach none of these actions except the catalogue; DS cannot read a restricted scenario; IA cannot run the
report; a delegated M (phase 03 delegation) can declare only when the delegation names `readiness.declare`.

**Tipping-off review.**
- The register, the grades and the report are detection logic (DR-64): which typologies the bank can and cannot see.
  They are readable only by the roles above; front-line, branch, operations and KYC roles never see them. The
  catalogue route open to everyone shows the CBI's public text and the codes of implementing scenarios, never a
  grade, a coverage kind, a declaration or a restricted code.
- The vendor annex names fields, not blind spots: no item, scenario or dependent count, so the core-banking vendor and
  the bank's IT, who receive it, learn what to supply and nothing about which detection is off.
- No customer, account, alert or case data appears anywhere in this phase: the inputs are availability rows (counts
  and percentages) and the register. Field candidates read stored catalogue rows only.
- Audit details carry codes, counts and ids; declaration text (`reason`, `interim_detail`) stays in the row and the
  sealed report, never in the audit detail.
The answer for a person outside a case's access list is: nothing, because no case exists here; for a person outside
the roles above, the only thing visible is the CBI's own public catalogue.

**Settings and configuration used:** the five keys of migration 0022 (registry bounds:
`coverage.declaration_review_max_days` 30-365, governed; `coverage.report_max_age_days` 1-365;
`coverage.candidates_max` 1-20; `coverage.maintained_fields` a JSON array whose items are catalogue keys with
`needs_declaration = true`, governed; `schedule.coverage_sweep` a cron at most hourly); the flags
`branch.observations_live` and `pack.islamic`; phase 01 `stepup.ttl_s`; phase 02 `evidence.lock_days` (R6),
`render.timeout_s`; phase 05 `discovery.*` only through the stored rows.

## 4. Infrastructure Updates

- **Job kinds registered** (all akr-worker; jobs RLS unchanged, the `coverage_`, `gap_report_` and `pack_` prefixes
  belong to `akr_app`):
  - `coverage_grade` (payload `{reason}`; coalesce key `coverage_grade`; concurrency 1; max attempts 5).
  - `gap_report_build` (payload `{run_id}`; concurrency 1, and one active run by `uq_gap_report_runs_active`; max
    attempts 3; a retry resumes from the step the run reached: a stored document is reused by its `owner_id`, packs
    are published only in the final transaction).
  - `pack_build` (payload `{kind, scope, lang?, requested_by}`; coalesce key kind + scope key + lang; concurrency 2;
    max attempts 3).
  - `coverage_sweep` (scheduled, `schedule.coverage_sweep`, daily 02:40 bank time; concurrency 1).
- **Docker compose:** one change: akr-worker gains a read-only bind mount of the installation file `deploy/branding.json` at `/etc/akr/branding.json` (the cover of section 3.5 reads the bank name from it; the phase 00 volume list names it). Gotenberg (phase 02) renders the PDFs; no new service, no new network edge.
- **AppState additions:** `api`: `catalogue: Arc<CatalogueQuery>`, `coverage: Arc<CoverageService>`
  (`ReadinessQuery`, `DeclarationService`, `GapReportQuery`), `packs: Arc<PackRequestService>`, `field_candidates:
  FieldCandidateFinder` on the discovery state; the evidence owner kind `gap_report_run`; step-up action classes
  `readiness_declare` and `pack_download`. `worker`: the four handlers, `GapReportService`, `IntegrationProfileBuilder`
  (the phase 05 domain service), `ExternalFeedRegistry` (empty implementation), the template set. `ctl`: `packs seed`, `packs
  verify-data`.
- **OpenAPI registration:** tags `catalogue` (4 operations), `coverage` (6), `packs` (3), and one more on `discovery`:
  14 in all, each with `x-akr-action`. The declaration and the download declare `X-StepUp-Assertion` and reference the
  step-up scheme; the declaration requires `If-Match`; the two POSTs document 202 with `Location` and
  `Idempotency-Key`; the download documents `application/pdf` and the `X-Content-SHA256` header; `GapReportView`
  references `CbiGapReportV1` generated from `cbi-gap-report.v1.json`.
- **Network zones:** no change. Only akr-worker calls Gotenberg (phase 02); nothing in this phase reaches a CBS.
- **Readiness:** akr-api `/readyz` adds `cbi_pack` (173 items and 161 scenarios installed, section 3.1).
- **Metrics:** `akr_readiness_items{grade}` and `akr_readiness_scenarios{grade}` (gauges, bank scope),
  `akr_gap_report_build_seconds` (histogram), `akr_declarations{status}` (gauge). Alert rule: a declaration
  `expired` for more than 7 days, a gap report run `failed`.
- **Ops cockpit:** nothing new (coverage is the MLRO's subject, not an operations one; the command centre of phase
  24 shows it).
- **Existing entity, schema or route updates:**
  - Phase 01: two step-up action classes (`readiness_declare`, `pack_download`).
  - Phase 02: `admin.packs` kind CHECK widened (migration 0021); evidence owner kind `gap_report_run` registered
    (R6); the audited pack download for three kinds arrives here, before phases 16 and 24.
  - Phase 03: Cedar files `80_detection.cedar` and `C0_packs.cedar`, entities in `schema.cedarschema`; the flag
    service enqueues `coverage_grade` when `branch.observations_live` or `pack.islamic` changes.
  - Phase 04: the setting change target enqueues `coverage_grade` on activating `coverage.maintained_fields`.
  - Phase 05: `discovery_seal_report` enqueues `coverage_grade` after a profile run; `IntegrationProfileBuilder` sits in
    `domain::discovery` so akr-worker can call it (phase 05 section 3.8 states this).
  - Planning docs aligned in the same change: API.md 3.5, 3.7, 3.11 and 3.16; DATA-MODEL s.6, s.10 and s.23; MODULES
    M7 and M11; roadmap row 06.

## 5. Frontend Impact

Backend phase for screens; phase 08 builds them on this API. `make generate-api` regenerates the client with the
hooks `useGetCbiCatalogue`, `useGetScenarios`, `useGetScenariosId`, `useGetCoverageReadiness`,
`usePutCoverageReadinessIdDeclaration`, `useGetCoverageCbiGapReport`, `usePostCoverageCbiGapReportRuns`,
`useGetCoverageCbiGapReportRuns`, `useGetCoverageCbiGapReportRunsId`, `usePostPacks`, `useGetPacksId`,
`useGetPacksIdDownload`, `useGetDiscoveryFieldCandidates`.

Screen E2 (DESIGN-SYSTEM E2, route `/discovery`, DASH layout in the reduced shell) after phase 08:
- **Sources band** (phase 05 hooks): each source's state, last profile run, field availability by family as a gapped,
  labelled bar (words on every segment).
- **Coverage grid:** the 173 items in `ref` order, grouped by article, each with its grade chip and term word
  ("Operable", "Degraded", "Blocked"), the linked scenarios (restricted ones as "Restricted scenario"), and the
  missing fields; filters by grade, kind and declaration status. Data-table rules of DESIGN-SYSTEM apply (sticky
  header, keyboard row navigation, Arabic numerals per the user's setting).
- **Declaration drawer** (M only; hidden for other roles, not disabled): interim route, detail, owner picker (active
  humans), reason, review date bounded by the setting; submit opens the `StepUpDialog` (phase 08 component) and sends
  `If-Match`.
- **Field candidates panel** (DS): for each missing attribute, the ranked candidates with reason chips and "Use as
  hint", which calls `PATCH /sources/{id}` (phase 05).
- **Report bar:** "Generate gap report" (DS, M), the run status by polling `GET /coverage/cbi-gap-report/runs/{id}`
  every 3 seconds until final, the stale banner, and the four pack downloads (step-up each) plus "Integration profile
  PDF" per source.
Until phase 08 the steward and the MLRO use Swagger UI and `akr-ctl`.

## 6. Verification

1. `make migrate`: 0020-0022 apply; a second run is a no-op; the phase 00 schema checks pass for the five new tables
   (classification and retention rows present). Negative checks: `DELETE FROM scn.cbi_catalogue_items` (any row, as
   the owner role too) fails with `register rows are never deleted`; changing an item's `ref` or `text` fails;
   changing a scenario's `code` or `pack` fails; `UPDATE scn.scenarios SET status = 'retired'` on a `cbi` scenario
   fails its CHECK; a `source` scope row with a declaration fails its CHECK; a second `queued` gap report run fails
   `uq_gap_report_runs_active`; a `completed` run with three pack ids fails its CHECK; an `admin.packs` row of kind
   `vendor_request` inserts; `akr_app` has no DELETE on any `scn` table.
2. `cargo check` with `cargo sqlx prepare`; `cargo clippy --all-targets -- -D warnings`; `cargo deny check` (akr-worker
   has no `akr-sources` dependency although it builds the integration profile).
3. `akr-ctl packs seed` -> `added: {items: 173, scenarios: 161, links: n}`; a second run -> all zeros;
   `akr-ctl packs verify-data` -> exit 0 with the counts of section 3.1; a tampered pack file (one byte) -> exit 1,
   nothing written; `--dry-run` writes nothing.
4. `cargo test`, including:
   - grader (`domain/coverage/tests`): each rule of section 3.2 in isolation (required good, partial, poor,
     unpopulated, absent, unassessed; fallback missing; optional missing; external need; observational with the flag
     off and on; `pack_not_enabled`; `declaration_required` before and after `coverage.maintained_fields` names the
     key); worst-of combination; bank scope takes the best source per attribute; the item caps (a `partial`-only item
     never above B, the `proxy` item never above C); `counter_only` items D until the flag is on;
   - the consistency test against `grade-golden.v1.0.0.json`;
   - declarations: lapse when a regrade lifts the grade to B; expiry on the sweep's test clock; the bound on
     `review_due`;
   - gap report document validates against `cbi-gap-report.v1.json`; the vendor annex has no key named `ref`,
     `scenario`, `code` or `item` at any depth (a recursive `jq` walk); restricted scenarios appear with no code or
     name; the render golden for one Arabic page of the report (phase 02 `render_golden_arabic` method);
   - field candidates: on the fixture catalogue, `customer.date_of_birth` ranks the fixture's `BIRTH_DATE` column
     first with `alias` and `type`; a copy table's column is never offered.
5. Curl suite `tests/curl/phase06_coverage.sh` (dev users `ds@bank.test`, `m@bank.test`, `ia@bank.test`,
   `cfg@bank.test`, `a1@bank.test`; the phase 05 suite has left `CBS1` active with a completed profile run):
   - As A1 `GET /api/v1/cbi-catalogue` -> 200 with 173 items; `jq` finds no key `grade`, `coverage` or
     `declaration` and no code of a `PUB` scenario.
   - As A1 `GET /api/v1/coverage/cbi-gap-report` -> 403 `E_SCOPE`; as CFG -> 403 `E_SCOPE`.
   - As DS `GET /api/v1/coverage/readiness?scope=bank&grade=D` -> 200, rows for the `party.pep_status` scenarios with
     `declaration_required`; `GET /api/v1/scenarios/<a PUB scenario id>` as DS -> 404 `E_NOT_FOUND`, byte-identical to
     an unknown id; as CFG -> 200.
   - As DS `POST /api/v1/coverage/cbi-gap-report/runs` with `Idempotency-Key: gap-0001` -> 202; a second key at once
     -> 409 `run_active`; polling the run -> `completed`, `pack_ids` of length 4; the same key and body -> the
     stored 202 with `x-idempotent-replay: true`.
   - As M `GET /api/v1/coverage/cbi-gap-report` -> 200, `summary.items` sums to 173, `stale: false`, `vendor_annex`
     present; as IA -> 200.
   - As M `GET /api/v1/coverage/readiness?grade=D` -> 200 with the row's `ETag`; as A1 -> 403 `E_SCOPE`.
   - As M `PUT /api/v1/coverage/readiness/<a D row>/declaration` with `If-Match` and no step-up -> 401
     `E_STEPUP_REQUIRED`; with step-up and `review_due` 400 days ahead -> 422 `review_too_far`; with 90 days -> 200,
     `declaration_status: open`; the same on an A row -> 409 `not_blocked`; as DS -> 403 `E_SCOPE`.
   - As DS `GET /api/v1/packs/<cbi_gap ar id>/download` without step-up -> 401; with it -> 200, `%PDF-` prefix, the
     `X-Content-SHA256` equal to the pack row's sha256; `audit.access_event` (phase 02; `psql` as `akr_readonly`) holds the
     `download` row for the pack.
   - As DS `POST /api/v1/packs {"kind":"integration_profile","scope":{"source_id":"<CBS1>"}}` -> 202, then two packs
     (ar, en); `{"kind":"board"}` -> 422 `unknown_pack_kind`; as CH an `integration_profile` pack -> 403 `E_FORBIDDEN`.
   - As DS `GET /api/v1/discovery/field-candidates?source_id=<CBS1>` -> 200, at most 5 candidates per attribute, each
     with reasons.
   - As ADM `PUT /api/v1/admin/flags/branch.observations_live` `{"enabled": true}` with `If-Match` -> a
     `coverage_grade` run; the `counter_only` items leave
     D only where their linked scenarios grade better.
6. Swagger UI lists 14 new operations (121 with phases 01, 03, 04 and 05), each with `x-akr-action`; `make
   generate-api` twice gives zero diff.
7. Audit: after the curl run `audit_log` holds `cbi_pack.seeded`, `readiness.graded`, `readiness.declared`,
   `gap_report.requested`, `gap_report.completed`, `pack.created` (six: four from the run, two from the integration
   profile), `pack.requested`, `pack.downloaded`, `authz.denied` (the CH pack and DS declaration attempts), and the
   masked `deny` access event for the restricted scenario; `akr-ctl audit verify` passes.
8. No CBS connection is made by this phase: the conformance suite is unaffected, and the statement journal of phase 05
   shows no statement between the start and end of the curl run.
9. Phase 00 to 05 tests still pass.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 1: corrections found while checking phases 00 to 02 (tools/agent/gap1-run1-verdict.md); lead-verified.

### 7.1 Step-up classes of the declaration and the pack download (DR-359)
Phase 01's closed list of action classes is not extended. This phase declares two step-up actions with existing
classes: `StepUpAction { name: "readiness.declare", class: approval }` (section 3.4 step 1) and
`StepUpAction { name: "pack.download", class: export }` (section 3.6; DR-230 of phase 24). Every mention of a class
`readiness_declare` or `pack_download` (sections 3.4, 3.6 and 4) reads as these action names with those classes, and
section 4's "Phase 01: two step-up action classes" is void: phase 01 needs no change.

Gap pass 1 (2026-10-10), gap item (a): Cedar form corrected in place (tools/agent/fix-g1-cedar.py); lead-verified.

### 7.2 AAL comparison
The policy `readiness.mlro_declares` is corrected in place: `context.aal == "aal2"` (phase 03 `Ctx.aal` is a string,
`"aal1"` or `"aal2"`), not `context.aal >= 2`.

Gap pass 1 (2026-10-10), run 3a, from tools/agent/gap1-run3a.md; lead-verified (tools/agent/gap1-run3a-verdict.md).

### 7.3 The AttributeCatalogue type (DR-367)
`AttributeCatalogue` (`domain::coverage`) is loaded from `backend/catalogue/canonical-attributes.v1.json` (phase 05
section 3.7). One entry per key with the fields of that file (`family`, `module`, `label`, `needs_declaration`,
`used_by`). Methods here: `get(key)`, `keys()`, `needs_declaration(key)`. Keys have the form `<namespace>.<name>`: the
file's keys are `<canon table>.<column>` and are checked against the canonical model by phase 10's test. Two
namespaces are registered in code by later phases and are not canonical columns, so that test skips them: `feat.`
(phase 13 adds `feature_keys()`, the active feature keys with their `requires`) and `rating.` (phase 15 adds
`register_rating_attributes()`: `rating.level`, `rating.cbi_level`). The grader of section 3.2 takes
`&AttributeCatalogue` unchanged. Phase 10's `AttributeService` is a different thing: the registry of bank-defined
attributes (`admin.attribute_values`), not this catalogue.

### 7.4 packs seed and packs verify-data cover the goAML profiles (phase 07)
Phase 07 extends `akr-ctl packs seed` and `akr-ctl packs verify-data` with one step after the CBI pack: each
`backend/packs/goaml/*/` directory of the release manifest goes through phase 07's profile load (section 3.1 there).
The CBI pack steps of section 3.1 do not change.

### 7.5 Scenario register and the routers (phase 16)
Phase 16 owns these changes (its section 4.4): `GET /scenarios` gains the live and shadow versions, the bank grade and
the open-declaration flag; `GET /scenarios/{id}` gains the versions, parameter sets and test summary, and its scopes
become `config.author, config.approve, dashboard.read`. `POST /cbi-catalogue/imports` joins the `catalogue` router and
`GET /coverage/matrix` the `coverage` router. The Cedar actions and the masked 404 of this phase do not change.

### 7.6 Reason state unmapped (phase 10 section 3.8)
The reason-state list in the comment on `scn.readiness_grades.reasons` gains `unmapped`. The grader of section 3.2
reads phase 10's `MappingCompleteness { source_id, entity, column, published_version }`: a monitored attribute with no
published mapping for the source is `unmapped` and grades D, as `absent` does. Draft and testing versions do not
count. No column changes.

### 7.7 Pack kinds, scopes and builders after phases 24 and 31
- Phase 24 migration `0078` replaces `packs_kind_check` with the list of this phase plus `certification`;
  `correspondent` and `validator` are already in the list.
- Phase 24 adds the scope `assessor.read` to `POST /packs`, `GET /packs/{id}` and `GET /packs/{id}/download`
  (Cedar keeps AS to kind `assessor`), and its five kinds to `POST /packs`.
- Phase 31 adds the builders of `correspondent` and `validator` under the flag `pack.correspondent`.
Section 3.6 therefore reads: `POST /packs` accepts three kinds until phase 24 adds its five and phase 31 adds the last
two; any other kind is refused with `unknown_pack_kind`.

Gap pass 1 (2026-10-10), run 3b: corrections found while checking phases 11 to 16 and the API.md route owners (tools/agent/gap1-run3b-verdict.md); lead-verified.

### 7.8 GET /ref/typologies (DR-381)
This phase owns the route. It joins the `catalogue` router (4 routes) with this row of the section 2 route table:

| Method | Path | Scope | Description |
|-|-|-|-|
| GET | /ref/typologies | self | The shared typology set: number, tag and label in Arabic and English (`typology.read`) |

The set is the 21 typologies of research file 27: B2.1 to B2.16 (numbers 1 to 16) and the five cross-cutting tags 17
to 21 named before the B2 table. `tools/build_scn_register.py` writes it as a third file,
`backend/catalogue/typologies.v1.json` (`[{number, tag, label: {ar, en}}]`), reviewed and committed like the two pack
files and signed by the release build. The loader refuses a file whose numbers are not exactly 1 to 21 or whose tags
repeat. Scenarios keep `typologies SMALLINT[]` (the numbers); the DSL `typology` strings are the `tag`s.
Cedar: `typology.read` on System, every active human principal, added to the `catalogue.everyone` policy beside
`cbi_catalogue.read` (`action in [Akr::Action::"cbi_catalogue.read", Akr::Action::"typology.read"]`). API.md 3.8
marks the row "(phase 06)".

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.9 The CatalogueQuery method (V-06-01-a, MEDIUM; DR-413)
`CatalogueQuery` (AppState, section 4) has one method, `list(&self, conn: &mut PgConnection) -> Result<Vec<CatalogueItemView>, RepoError>`, which serves `GET /cbi-catalogue`. It returns each item of `CatalogueRepository::active` with its `scenarios` codes (the `full` and `partial` links whose scenario has `visibility = 'normal'`, as `CatalogueItemView` defines them), ordered by `ref`. It reads no grade, coverage kind or restricted code.

### 7.10 Required and optional fields of cbi-gap-report.v1.json (V-06-03-b, LOW; DR-413)
Section 3.5 step 4 names the keys of the document. Required: the six top-level keys `cover`, `summary`, `items`, `bank_request`, `vendor_annex` and `outcomes`, and every key that step 4 names inside them, except these optional keys: an item's `declaration` (present only when its blocking scenarios carry one; its `detail` is null when no `interim_detail` was given), and the `code` and `name` of a `linked` entry (absent for a restricted scenario, which carries `restricted: true`); `restricted` itself is present, as true, only on such an entry.
Value types: ids are UUID strings; timestamps are RFC 3339 strings; counts are integers of 0 or more; shares are numbers from 0 to 1; `title`, `text` and labels are `{ar, en}` objects of strings; `items`, `linked`, `bank_request` and `vendor_annex` are arrays. The values of `outcomes` are nullable (null until phases 17 to 21 fill them).

### 7.11 The fields of Grading (V-06-04-a, LOW; DR-413)
`Grading { scenarios: Vec<GradeUpsert>, items: Vec<ItemGrade> }`. `scenarios` holds one row per scenario and scope: the bank scope and each source scope that has a completed profile run (the rows that section 3.3 step 3 writes). `items` holds one row per catalogue item for the bank scope, `ItemGrade { catalogue_item_id: Uuid, grade: Grade, reasons: Vec<Reason>, missing_fields: Vec<AttributeKey> }`: `reasons` carries `no_scenario` when item rule 1 gives D, and `missing_fields` follows item rule 3. `AttributeKey`, used here, by `Need` in section 2 and by `grade` in section 3.2, is a newtype over `String` that holds one key of the `AttributeCatalogue` (section 7.3: the form `<namespace>.<name>`, which is a canonical `<table>.<column>` or a registered `feat.` or `rating.` key); the form is checked when it is built. The term follows from the grade (`Term`).

### 7.12 The DeclarationService method (V-06-08-a, LOW; DR-413)
`DeclarationService::declare(&self, ctx: &AuthContext, id: Uuid, input: DeclarationInput, expected_version: i32) -> Result<ReadinessGrade, AppError>` is the service's only method. The handler has checked that `If-Match` is present (428 `E_PRECONDITION_REQUIRED` otherwise) and passes its version; the method consumes the step-up assertion (step 1) and does steps 2 to 5 of section 3.4 in one transaction. The `coverage_sweep` job calls `ReadinessRepository::expire_due` itself and does not go through the service.

### 7.13 The grader's mapping input (V-06-11-a, MEDIUM; DR-413)
From phase 10 on, `grade` (section 3.2) takes one more parameter after `catalogue_attrs`, `mapping: &[MappingCompleteness]`, with one row per source and monitored attribute, as phase 10 section 3.8 defines the type; section 3.3 step 1 loads these rows with its other inputs. The sentence "this phase reads availability only" in section 3.2 holds until then. Section 7.6 gives the rule.

## What This Phase Does NOT Touch

- No scenario versions, DSL, parameter sets, replay or engine (phase 16): the register holds identity, needs and
  links only; `owner_id` stays empty until phase 16 assigns owners.
- No alert, case or STR counts (phases 17-21): the report's outcome columns are `null`.
- No exclusion register with four-eyes and expiry (phase 12; the other half of L-02), no blind periods.
- No catalogue importer for new CBI issuances (S-06, phase 16); a new catalogue version arrives as a signed pack file.
- No branch observation log (phase 22): the flag `branch.observations_live` only changes grades.
- No mapping (phase 10): readiness reads availability; mapping completeness joins the grade in phase 10.
- No pack kinds other than `cbi_gap`, `vendor_request` and `integration_profile`, and no `POST /packs/verify` (phase
  24).
- No CBS access, no change to phase 05 tables, and no change to the phase 00-04 tables beyond the `admin.packs` kind
  CHECK.
