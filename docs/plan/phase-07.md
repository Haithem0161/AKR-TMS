# Phase 07: goAML Checker

**Goal:** The second early piece: an MLRO uploads (or pastes) goAML XML files the bank produced with its current
tool and gets, per file, what the Office's system would reject and why, in Arabic and English, with the field path,
the line and the fix for each finding, an estimated quality grade in the Office's four levels, and a printable result
page; behind it, the goAML profile registry that holds the Office's published XSD (5.0.2) with its patches, lookups,
business-rule parameters, limits and emit flags as versioned data changed only by change request, a watcher that
notices when the Office publishes a new schema, and the regression corpus every profile version must pass. No CBS
connection and no bank data beyond the uploaded files, which are checked and then deleted.

**Dependencies:** Phase 04 (the change target for profile changes; phases 02 and 03 through it), Phase 08 (the
reduced shell and the stepper for E1)
**Macro-phase:** P0
**Complexity:** M (9 routes; the XML safety, the rule catalogue and the profile loader carry the weight)
**Binaries touched:** akr-api (router `goaml`; change target `goaml_profile`) | akr-worker (jobs `goaml_check`,
`goaml_watch`, `goaml_corpus`; the helper executable `akr-xsdcheck` in the worker image) | `akr-ctl` (`goaml
import-xsd`, `goaml corpus`; `packs seed` installs the shipped profile). akr-ingestd and akr-assess unchanged
**Requirements carried:** RP-01 (the profile registry: XSD, patches, lookups, rules, report types, limits and emit
flags, versioned; the Iraqi profile `IQ-5.0.2` from the Office's XSD; clocks arrive in phase 21), RP-02 (lookups,
cardinalities, lengths and patterns read from the XSD, an unknown lookup a hard block), RP-03 (the watcher, the diff,
the corpus run and promotion by change request; reports in flight pinned from phase 21), RP-04 (the pattern
compatibility layer for the entity-name facet, the sanitiser warning and its open question), RP-05 (emit flags for the
5.0, 5.1 and 5.2 elements and the deprecated pairs, default off; the checker warns when a file uses one), RP-06 (the
regression corpus: the three validated samples and the 63 negative and edge cases; the FIU-answers corpus grows from
phase 21), RP-07 (the normaliser's rules as checks on uploaded files; the normaliser itself writes XML in phase 21),
RP-09 (the open-questions register per profile), RP-27 (validation layers L1 to L5 on uploaded XML; L0 needs the case
snapshot of phase 21), RP-28 (the business-rule catalogue with IDs, severities, versions and tests; changes by
change request), RP-29 (the quality score and the four-level grade, estimated; the monthly calibration sample is
phase 21), RP-30 (the consistency checks that an uploaded file allows), RP-42 (the rejection-message parser, a stub
with a pattern list in the profile), RP-82 (profile limits enforced on the uploaded files), RP-83 (the
business-rejection-rule corpus per profile, seeded and grown by profile change)
- Gap pass 1 (coverage), partly implemented: RP-08 (several active profiles, one per code); L-12 (pre-submission identifier and shape rules).
- Gap pass 1 (coverage), implemented here: VB-49 (the pre-submission goAML validator: schema and Office rules, failures block).

---

## 1. SQLx Migration Changes

Three migrations, `0023_goaml_profiles_watch`, `0024_goaml_check` and `0025_goaml_settings`. Four tables, all in
`rpt` (DATA-MODEL s.14, aligned in the same change: `goaml_profiles` becomes one row per profile version with the
columns below). No table of an earlier phase changes. The shipped profile is not inserted by a migration: `akr-ctl
packs seed` installs it with the XSD stored as evidence (section 3.1), as phase 06 installs the CBI catalogue.

```sql
-- migration 0023_goaml_profiles_watch.sql
-- Phase 07. The goAML profile registry and the Office schema watcher (MODULES M13, DATA-MODEL s.14, DR-68, DR-71).

DO $$ BEGIN
    CREATE TYPE rpt.goaml_profile_status AS ENUM ('candidate', 'active', 'retired', 'rejected');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

-- One row per profile version. A version's content never changes once it leaves `candidate`; an amendment is a new
-- candidate version. Reports (phase 21) and check runs pin the version row.
CREATE TABLE IF NOT EXISTS rpt.goaml_profiles (
    id                         UUID PRIMARY KEY,
    code                       TEXT NOT NULL CHECK (code ~ '^[A-Z]{2}(-[A-Z0-9]{1,8})?$'),   -- 'IQ'; 'AE' in a regime pack
    profile_version            INT NOT NULL CHECK (profile_version > 0),
    label                      TEXT NOT NULL,             -- 'IQ-5.0.2-2026-09'
    jurisdiction               CHAR(2) NOT NULL,          -- ISO 3166-1 alpha-2
    xsd_version                TEXT NOT NULL,             -- from goaml:schemaVersion, e.g. '5.0.2'
    xsd_sha256                 BYTEA NOT NULL CHECK (octet_length(xsd_sha256) = 32),   -- the Office's file as published
    xsd_evidence_id            UUID NOT NULL REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    effective_xsd_evidence_id  UUID NOT NULL REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,   -- patches applied
    source                     TEXT NOT NULL CHECK (source IN ('shipped', 'watcher', 'import', 'amendment')),
    source_ref                 JSONB NOT NULL DEFAULT '{}'::jsonb,   -- {url, published_at, title} or {file_sha256, note}
    -- [{id, xpath, facet, from, to, reason: {ar, en}, open_question?}]: changes to the XSD before validation (RP-04)
    patches                    JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(patches) = 'array'),
    -- Derived from the XSD at load: {lists: {<simpleType>: [{code, label}]}, report_codes: [...], counts: {...}}
    lookups                    JSONB NOT NULL CHECK (jsonb_typeof(lookups) = 'object'),
    -- {elements, flagged: {"5.0": [...], "5.1": [...], "5.2": [...]}, deprecated: [...], patterns: n, limits: n}
    schema_summary             JSONB NOT NULL CHECK (jsonb_typeof(schema_summary) = 'object'),
    -- {<rule code>: {enabled, severity: block | warn | info, params: {...}}} over the closed catalogue (section 3.3)
    rules                      JSONB NOT NULL CHECK (jsonb_typeof(rules) = 'object'),
    -- {criteria: {<code>: weight}, bands: {good: 85, moderate: 70, acceptable: 50}} (section 3.4)
    quality                    JSONB NOT NULL CHECK (jsonb_typeof(quality) = 'object'),
    -- {max_file_bytes, max_attachment_bytes, max_transactions, max_attachments, attachment_types, xml_per_zip}
    limits                     JSONB NOT NULL CHECK (jsonb_typeof(limits) = 'object'),
    emit_flags                 JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(emit_flags) = 'object'),
    -- [{id, pattern, rule_code, element?, message_key}] for Office rejection messages (RP-42)
    rejection_patterns         JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(rejection_patterns) = 'array'),
    -- [{id, question: {ar, en}, asked_at?, answer?: {ar, en}, answered_at?, source?}] (RP-09)
    open_questions             JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(open_questions) = 'array'),
    clocks                     JSONB NOT NULL DEFAULT '{}'::jsonb,   -- report clocks, filled by phase 21
    based_on                   UUID REFERENCES rpt.goaml_profiles(id) ON DELETE RESTRICT,   -- the version it came from
    diff_from_base             JSONB,                     -- section 3.6; NULL for the first version
    corpus_result              JSONB,                     -- {ran_at, passed, failed, cases: [...]}; NULL until run
    status                     rpt.goaml_profile_status NOT NULL DEFAULT 'candidate',
    change_request_id          UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,   -- the activation
    effective_from             TIMESTAMPTZ,
    retired_at                 TIMESTAMPTZ,
    created_by                 UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- NULL: system (seed, watcher)
    created_at                 TIMESTAMPTZ NOT NULL DEFAULT now(),
    version                    INT NOT NULL DEFAULT 1,
    CONSTRAINT uq_goaml_profiles_version UNIQUE (code, profile_version),
    CHECK ((status = 'active') = (effective_from IS NOT NULL AND retired_at IS NULL)),
    CHECK (status <> 'retired' OR retired_at IS NOT NULL),
    CHECK (source = 'shipped' OR status = 'candidate' OR status = 'rejected' OR change_request_id IS NOT NULL)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_goaml_profiles_active ON rpt.goaml_profiles (code) WHERE status = 'active';
CREATE INDEX IF NOT EXISTS idx_goaml_profiles_code       ON rpt.goaml_profiles (code, profile_version DESC);
CREATE INDEX IF NOT EXISTS idx_goaml_profiles_xsd        ON rpt.goaml_profiles (code, xsd_sha256);
CREATE INDEX IF NOT EXISTS idx_goaml_profiles_based_on   ON rpt.goaml_profiles (based_on) WHERE based_on IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_goaml_profiles_cr         ON rpt.goaml_profiles (change_request_id)
    WHERE change_request_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_goaml_profiles_created_by ON rpt.goaml_profiles (created_by) WHERE created_by IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_goaml_profiles_xsd_ev     ON rpt.goaml_profiles (xsd_evidence_id);
CREATE INDEX IF NOT EXISTS idx_goaml_profiles_eff_ev     ON rpt.goaml_profiles (effective_xsd_evidence_id);

-- A version's content is frozen once it is no longer a candidate; rows are never deleted.
CREATE OR REPLACE FUNCTION rpt.goaml_profiles_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'goAML profile versions are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.status <> 'candidate'
       AND (NEW.patches <> OLD.patches OR NEW.rules <> OLD.rules OR NEW.quality <> OLD.quality
            OR NEW.limits <> OLD.limits OR NEW.emit_flags <> OLD.emit_flags
            OR NEW.rejection_patterns <> OLD.rejection_patterns OR NEW.open_questions <> OLD.open_questions
            OR NEW.lookups <> OLD.lookups OR NEW.schema_summary <> OLD.schema_summary
            OR NEW.xsd_sha256 <> OLD.xsd_sha256
            OR NEW.effective_xsd_evidence_id <> OLD.effective_xsd_evidence_id) THEN
        RAISE EXCEPTION 'goAML profile %: content is frozen once the version leaves candidate', OLD.id
            USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.status IN ('retired', 'rejected') AND NEW.status <> OLD.status THEN
        RAISE EXCEPTION 'goAML profile %: % is final', OLD.id, OLD.status USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN NEW;
END $$;
DO $$ BEGIN
    CREATE TRIGGER trg_goaml_profiles_guard BEFORE UPDATE OR DELETE ON rpt.goaml_profiles
        FOR EACH ROW EXECUTE FUNCTION rpt.goaml_profiles_guard();
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

DO $$ BEGIN
    CREATE TYPE rpt.goaml_watch_status AS ENUM ('running', 'unchanged', 'changed', 'unreachable', 'failed');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS rpt.goaml_watch_runs (
    id                   UUID PRIMARY KEY,
    trigger              TEXT NOT NULL CHECK (trigger IN ('schedule', 'manual', 'import')),
    requested_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- NULL for schedule and akr-ctl
    profile_code         TEXT NOT NULL,
    source_url           TEXT,                          -- NULL for an import
    status               rpt.goaml_watch_status NOT NULL DEFAULT 'running',
    -- As the Office's API returned it: [{title, url, size, uploaded_at}]; no other content is kept
    attachment_list      JSONB,
    xsd_sha256           BYTEA CHECK (xsd_sha256 IS NULL OR octet_length(xsd_sha256) = 32),
    candidate_profile_id UUID REFERENCES rpt.goaml_profiles(id) ON DELETE RESTRICT,
    error_code           TEXT CHECK (error_code IN ('proxy_unreachable', 'tls', 'http_status', 'too_large',
                                                    'not_found_in_list', 'host_not_allowed', 'xsd_unparseable',
                                                    'internal')),
    detail               JSONB NOT NULL DEFAULT '{}'::jsonb,   -- http status, elapsed ms, orphaned patches
    started_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    finished_at          TIMESTAMPTZ,
    CHECK ((status = 'running') = (finished_at IS NULL)),
    CHECK ((status IN ('unreachable', 'failed')) = (error_code IS NOT NULL)),
    CHECK (status <> 'changed' OR candidate_profile_id IS NOT NULL),
    CHECK ((trigger = 'import') = (source_url IS NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_goaml_watch_running ON rpt.goaml_watch_runs (profile_code)
    WHERE status = 'running';
CREATE INDEX IF NOT EXISTS idx_goaml_watch_started   ON rpt.goaml_watch_runs (started_at DESC, id DESC);
CREATE INDEX IF NOT EXISTS idx_goaml_watch_req_by    ON rpt.goaml_watch_runs (requested_by) WHERE requested_by IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_goaml_watch_candidate ON rpt.goaml_watch_runs (candidate_profile_id)
    WHERE candidate_profile_id IS NOT NULL;

GRANT SELECT, INSERT ON rpt.goaml_profiles TO akr_app;
GRANT UPDATE (lookups, schema_summary, diff_from_base, corpus_result, status, change_request_id, effective_from,
              retired_at, patches, rules, quality, limits, emit_flags, rejection_patterns, open_questions,
              effective_xsd_evidence_id, version)
    ON rpt.goaml_profiles TO akr_app;
GRANT SELECT, INSERT ON rpt.goaml_watch_runs TO akr_app;
GRANT UPDATE (status, attachment_list, xsd_sha256, candidate_profile_id, error_code, detail, finished_at)
    ON rpt.goaml_watch_runs TO akr_app;
GRANT SELECT ON rpt.goaml_profiles, rpt.goaml_watch_runs TO akr_readonly;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'rpt', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('goaml_profiles',   'internal', ARRAY['id','code','profile_version','label','jurisdiction','xsd_version',
                                         'xsd_sha256','xsd_evidence_id','effective_xsd_evidence_id','source',
                                         'source_ref','patches','lookups','schema_summary','rules','quality','limits',
                                         'emit_flags','rejection_patterns','open_questions','clocks','based_on',
                                         'diff_from_base','corpus_result','status','change_request_id',
                                         'effective_from','retired_at','created_by','created_at','version']),
  ('goaml_watch_runs', 'internal', ARRAY['id','trigger','requested_by','profile_code','source_url','status',
                                         'attachment_list','xsd_sha256','candidate_profile_id','error_code','detail',
                                         'started_at','finished_at'])
) AS v(t, cls, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('rpt', 'goaml_profiles',   'R9',  'retirement', NULL, 'Filed reports cite the version they were validated under'),
 ('rpt', 'goaml_watch_runs', 'R12', 'age', 'started_at', 'Operational record; a change is kept in the profile row')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

```sql
-- migration 0024_goaml_check.sql
-- Phase 07. Checker runs and their findings (MODULES M13 P0, DATA-MODEL s.14, DR-69, DR-72). The uploaded files are
-- transient (section 3.2); these rows hold hashes, counts and value-free findings only.

DO $$ BEGIN
    CREATE TYPE rpt.goaml_check_status AS ENUM ('queued', 'running', 'completed', 'failed');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS rpt.goaml_check_runs (
    id                  UUID PRIMARY KEY,
    requested_by        UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    profile_id          UUID NOT NULL REFERENCES rpt.goaml_profiles(id) ON DELETE RESTRICT,   -- pinned at request
    status              rpt.goaml_check_status NOT NULL DEFAULT 'queued',
    file_count          SMALLINT NOT NULL CHECK (file_count BETWEEN 1 AND 50),
    -- [{n, sha256, size, content_type, kind: xml | zip | pasted, report_code?, transactions?, outcome?, score?,
    --   grade?, counts: {block, warn, info}}]; outcome: would_reject | graded | refused
    files               JSONB NOT NULL CHECK (jsonb_typeof(files) = 'array'),
    file_names          BYTEA,                         -- the uploaded names, sealed (`secret-str` family, AAD = id)
    with_rejection_text BOOLEAN NOT NULL DEFAULT FALSE,
    summary             JSONB,                         -- totals by layer and severity, grades
    error_code          TEXT CHECK (error_code IN ('signatures_stale', 'profile_unavailable', 'internal')),
    transient_deleted   BOOLEAN NOT NULL DEFAULT FALSE,   -- the staged files are gone (section 3.2)
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    started_at          TIMESTAMPTZ,
    finished_at         TIMESTAMPTZ,
    version             INT NOT NULL DEFAULT 1,
    CHECK ((status = 'failed') = (error_code IS NOT NULL)),
    CHECK ((status IN ('completed', 'failed')) = (finished_at IS NOT NULL))
);
CREATE INDEX IF NOT EXISTS idx_goaml_check_runs_req     ON rpt.goaml_check_runs (requested_by, created_at DESC, id DESC);
CREATE INDEX IF NOT EXISTS idx_goaml_check_runs_created ON rpt.goaml_check_runs (created_at DESC, id DESC);
CREATE INDEX IF NOT EXISTS idx_goaml_check_runs_profile ON rpt.goaml_check_runs (profile_id);
CREATE INDEX IF NOT EXISTS idx_goaml_check_runs_pending ON rpt.goaml_check_runs (created_at)
    WHERE NOT transient_deleted;

CREATE TABLE IF NOT EXISTS rpt.goaml_check_findings (
    id          UUID PRIMARY KEY,
    run_id      UUID NOT NULL REFERENCES rpt.goaml_check_runs(id) ON DELETE RESTRICT,
    file_no     SMALLINT NOT NULL CHECK (file_no >= 1),
    layer       TEXT NOT NULL CHECK (layer IN ('pre', 'xsd', 'compat', 'rule', 'quality', 'consistency', 'package',
                                               'office')),
    rule_code   TEXT NOT NULL CHECK (rule_code ~ '^[A-Z]{2,6}-[0-9]{2,3}$'),   -- 'XSD-04', 'BR-12', 'Q-03'
    severity    TEXT NOT NULL CHECK (severity IN ('block', 'warn', 'info')),
    -- Element path with positions only, e.g. /report/transaction[2]/t_from_my_client/from_person/first_name
    path        TEXT CHECK (path IS NULL OR (length(path) <= 1000 AND path ~ '^(/[A-Za-z_][A-Za-z0-9_.-]*(\[[0-9]+\])?)+$')),
    line        INT CHECK (line IS NULL OR line > 0),
    col         INT CHECK (col IS NULL OR col > 0),
    -- Codes, lengths, counts, offsets and expected sets only; never a value from the file (DR-69)
    params      JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(params) = 'object'),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_goaml_findings_run ON rpt.goaml_check_findings (run_id, file_no, severity, id);

GRANT SELECT, INSERT ON rpt.goaml_check_runs TO akr_app;
GRANT UPDATE (status, files, summary, error_code, transient_deleted, started_at, finished_at, version)
    ON rpt.goaml_check_runs TO akr_app;
GRANT SELECT, INSERT ON rpt.goaml_check_findings TO akr_app;
GRANT SELECT ON rpt.goaml_check_runs, rpt.goaml_check_findings TO akr_readonly;

-- The runs are about the bank's suspicion reports: secret-str, readable by the requester and the MLRO (section 3.8).
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification, encrypted, note)
VALUES ('rpt', 'goaml_check_runs', 'file_names', 'secret-str', TRUE, 'Uploaded file names can name a subject')
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'rpt', v.t, c, 'secret-str'::admin.data_classification
FROM (VALUES
  ('goaml_check_runs',     ARRAY['id','requested_by','profile_id','status','file_count','files',
                                 'with_rejection_text','summary','error_code','transient_deleted','created_at',
                                 'started_at','finished_at','version']),
  ('goaml_check_findings', ARRAY['id','run_id','file_no','layer','rule_code','severity','path','line','col','params',
                                 'created_at'])
) AS v(t, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('rpt', 'goaml_check_runs',     'R6', 'age', 'created_at', 'As report records; the files themselves are not kept'),
 ('rpt', 'goaml_check_findings', 'R6', 'age', 'created_at', 'As their run')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

```sql
-- migration 0025_goaml_settings.sql
-- Phase 07. Checker limits, the watcher, the bank's goAML identifiers.

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('goaml.check_max_files',   '20', 'integer', NULL, FALSE,
  '{"ar": "أقصى عدد ملفات في فحص واحد", "en": "Maximum files in one check"}'),
 ('goaml.check_max_bytes',   '52428800', 'integer', NULL, FALSE,
  '{"ar": "أقصى حجم للملف المفحوص (بايت)", "en": "Maximum size of a checked file (bytes)"}'),
 ('goaml.watch_enabled',     'false', 'boolean', NULL, TRUE,
  '{"ar": "مراقبة مخطط المكتب المنشور", "en": "Watch the Office''s published schema"}'),
 ('goaml.watch_url',         '"https://api.aml.iq/api/GoAmlSystem/attachments"', 'string', NULL, TRUE,
  '{"ar": "عنوان قائمة مرفقات المكتب", "en": "Office attachment list URL"}'),
 ('goaml.watch_hosts',       '["api.aml.iq", "aml-iq.fra1.cdn.digitaloceanspaces.com"]', 'json', NULL, TRUE,
  '{"ar": "المضيفات المسموح التنزيل منها", "en": "Hosts the watcher may download from"}'),
 ('goaml.watch_match',       '{"title_contains": ["goAML Schema"], "extension": ".xsd"}', 'json', NULL, FALSE,
  '{"ar": "كيفية التعرف على ملف المخطط في القائمة", "en": "How the schema file is recognised in the list"}'),
 ('goaml.watch_stale_days',  '7', 'integer', NULL, FALSE,
  '{"ar": "أيام دون مراقبة ناجحة قبل التنبيه", "en": "Days without a successful watch before an alert"}'),
 ('schedule.goaml_watch',    '"0 10 5 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مراقبة مخطط goAML", "en": "goAML schema watch schedule"}'),
 -- The bank's own identifiers for the my-client and rentity rules (BR-12, BR-23). Bank data; governed because phase
 -- 21 validates filed reports with them.
 ('goaml.bank_identifiers',  '{"rentity_id": null, "rentity_branches": [], "swift": [], "institution_codes": [], "gl_account_patterns": []}',
  'json', NULL, TRUE,
  '{"ar": "معرّفات المصرف في goAML", "en": "The bank''s goAML identifiers"}')
ON CONFLICT (key) DO NOTHING;
```

`secret-str` on the run and finding rows puts them under the suspicion-tier handling of phase 03 (masked reads,
access events on reads by anyone but the requester). The findings never hold a value from a file (section 3.2), so
the classification protects the fact that a report existed and what was wrong with it, not customer data.

## 2. DDD Implementation (Rust)

**Crate placement** (workspace pinned in phase 00; layer rules in `ddd.md`):

```
backend/
├── packs/goaml/iq-5.0.2-2026-09/      # signed release data (C-38), listed in the release manifest with sha256
│   ├── schema.xsd                     # the Office's file as published 2026-09-17 (217,567 bytes), unchanged
│   ├── profile.json                   # patches, rules, quality, limits, emit flags, rejection patterns, open questions
│   ├── corpus/                        # valid/*.xml (the three research samples, invented data) and cases/*.xml with
│   │                                  #   cases.json: {file, expect: {outcome, rules: [code]}} (63 cases)
│   └── README.md                      # provenance: URL, date, sha256, what each patch does and why
├── packs/goaml/messages.v1.json       # {<rule code>: {title: {ar, en}, explain: {ar, en}, fix: {ar, en}}}
├── schemas/goaml-profile.v1.json      # validates profile.json and every amendment
├── crates/
│   ├── goaml/     # akr-goaml (new, pure library, no libxml2): ProfileLoader (XSD facts with roxmltree), PatchApplier,
│   │              #   SchemaDiff, PreScreen (quick-xml), the rule catalogue, Scorer, ConsistencyChecks,
│   │              #   PackageChecks (zip), RejectionParser, the corpus runner
│   ├── domain/    # + goaml (GoamlProfile, ProfileStatus, CheckRun, CheckFinding, WatchRun, value objects, ports)
│   ├── db/        # + Pg repositories for the four tables
│   ├── authz/     # + resource adapters GoamlProfile, GoamlCheckRun
│   └── jobs/      # + kinds goaml_check, goaml_watch, goaml_corpus (akr-worker)
├── tools/xsdcheck/ # akr-xsdcheck: the only binary that links libxml2 (section 3.2); shipped in the akr-worker image
├── api/           # + router goaml; change target goaml_profile
├── worker/        # + the three job handlers, SandboxedXsdValidator, ProxySchemaFetcher
└── ctl/           # + goaml import-xsd, goaml corpus; packs seed installs the profile
```

`cargo deny` bans the `libxml` crate (and any `libxml2-sys`) everywhere but `tools/xsdcheck`, so no long-running
process links the C parser (DR-70). `akr-goaml` reads XML only through `quick-xml` (streaming pre-screen) and
`roxmltree` (read-only tree, no DTD support, which is what this phase wants) and the archive only through `zip` (read
mode, no extraction to disk).

- **Domain entities** (`Debug, Serialize, Deserialize, FromRow, ToSchema`; fields mirror section 1): `GoamlProfile`,
  `GoamlWatchRun`, `GoamlCheckRun`, `GoamlCheckFinding`; enums `ProfileStatus`, `WatchStatus`, `CheckStatus` with
  `#[sqlx(type_name = "rpt.goaml_profile_status")]` and the other two.
- **Value objects:** `ProfileCode`, `ProfileRef { code, profile_version }` (`Display` = `IQ v3`), `XsdFacts` (elements
  with cardinality, lengths, patterns, enumerations with their `xs:documentation` labels, `supportedIn` flags,
  deprecation notes), `Patch { id, xpath, facet, from, to, reason, open_question }`, `RuleCode` (`PRE`, `XSD`, `COMPAT`,
  `BR`, `NORM`, `EMIT`, `DEP`, `CONS`, `PKG`, `Q`, `OFF` prefixes), `RuleConfig { enabled, severity, params }`,
  `Severity` (`Block`, `Warn`, `Info`), `Layer`, `Quality { criteria: BTreeMap<RuleCode, u8>, bands: Bands }`, `Grade`
  (`Good`, `Moderate`, `Acceptable`, `Poor`), `FileOutcome` (`WouldReject`, `Graded`, `Refused`), `Limits`,
  `EmitFlags`, `RejectionPattern`, `OpenQuestion`, `FindingPath` (validated element path), `FindingParams` (a map
  whose values are numbers, booleans, rule codes, element names or lookup codes taken from the profile; the type has
  no constructor from a file's text, DR-69), `TransientRef` (phase 02 addition, section 4).
- **Create/Update structs:** `NewProfileVersion`, `ProfileAmendment` (`Deserialize, ToSchema`; all `Option`:
  `patches`, `rules`, `quality`, `limits`, `emit_flags`, `rejection_patterns`, `open_questions`), `NewCheckRun`,
  `NewFinding`, `NewWatchRun`. In `NewProfileVersion` the loader fills `lookups` and `schema_summary` (section 3.1 step 2); the caller, which holds the bytes, computes `xsd_sha256`, stores both XSDs as evidence and passes `xsd_evidence_id` and `effective_xsd_evidence_id` (section 3.1).
- **Repository traits** (domain layer; `Pg*` implementations in `db`):

```rust
#[async_trait]
pub trait GoamlProfileRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, p: &NewProfileVersion) -> Result<GoamlProfile, RepoError>;  // next profile_version
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<GoamlProfile>, RepoError>;
    async fn active(&self, conn: &mut PgConnection, code: &str) -> Result<Option<GoamlProfile>, RepoError>;
    async fn by_xsd(&self, conn: &mut PgConnection, code: &str, sha256: &[u8; 32]) -> Result<Vec<GoamlProfile>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &ProfileFilter, page: Keyset) -> Result<Page<GoamlProfile>, RepoError>;
    async fn set_corpus_result(&self, tx: &mut Tx, id: Uuid, r: &CorpusResult) -> Result<(), RepoError>;
    /// Candidate -> active and the previous active -> retired, in one statement pair under the code's advisory lock.
    async fn activate(&self, tx: &mut Tx, id: Uuid, cr: Uuid, at: DateTime<Utc>) -> Result<Activation, RepoError>;
    async fn reject(&self, tx: &mut Tx, id: Uuid) -> Result<(), RepoError>;
}

#[async_trait]
pub trait GoamlWatchRepository: Send + Sync {
    async fn start(&self, tx: &mut Tx, w: &NewWatchRun) -> Result<GoamlWatchRun, RepoError>;   // uq running -> 409
    async fn finish(&self, tx: &mut Tx, id: Uuid, end: &WatchEnd) -> Result<GoamlWatchRun, RepoError>;
    async fn list(&self, conn: &mut PgConnection, page: Keyset) -> Result<Page<GoamlWatchRun>, RepoError>;
    async fn last_success(&self, conn: &mut PgConnection, code: &str) -> Result<Option<DateTime<Utc>>, RepoError>;
}

#[async_trait]
pub trait GoamlCheckRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, r: &NewCheckRun) -> Result<GoamlCheckRun, RepoError>;
    async fn start(&self, tx: &mut Tx, id: Uuid) -> Result<Option<GoamlCheckRun>, RepoError>;
    async fn insert_findings(&self, tx: &mut Tx, rows: &[NewFinding]) -> Result<u64, RepoError>;   // UNNEST batches
    async fn complete(&self, tx: &mut Tx, id: Uuid, files: &[FileResult], summary: &CheckSummary)
        -> Result<GoamlCheckRun, RepoError>;
    async fn fail(&self, tx: &mut Tx, id: Uuid, code: CheckError) -> Result<GoamlCheckRun, RepoError>;
    async fn mark_transient_deleted(&self, tx: &mut Tx, id: Uuid) -> Result<(), RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<GoamlCheckRun>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &CheckRunFilter, page: Keyset) -> Result<Page<GoamlCheckRun>, RepoError>;
    async fn findings(&self, conn: &mut PgConnection, run: Uuid, f: &FindingFilter, page: Keyset)
        -> Result<Page<GoamlCheckFinding>, RepoError>;
    async fn undeleted_older_than(&self, conn: &mut PgConnection, t: DateTime<Utc>) -> Result<Vec<Uuid>, RepoError>;
}
```

- **Repository notes:** lists are keyset-paged (`(created_at DESC, id DESC)`; findings by `(file_no, severity
  order block-warn-info, id)`); `CheckRunFilter.requested_by` is forced to the caller unless Cedar allows
  `goaml_check.read_all` (section 3.8), so the filter is applied in SQL, never after the page is built; `insert`
  of a profile takes `pg_advisory_xact_lock(hashtext('goaml_profile:' || code))` and computes `max(profile_version)
  + 1` inside it.

- **Ports:**

```rust
/// L1. Implemented by SandboxedXsdValidator (akr-worker), which runs akr-xsdcheck (section 3.2).
#[async_trait]
pub trait XsdValidator: Send + Sync {
    async fn validate(&self, effective_xsd: &CachedXsd, xml: &[u8], limits: &XsdLimits)
        -> Result<Vec<XsdIssue>, XsdError>;
}
/// A libxml2 structured error reduced to what may be stored: no message text, no value.
pub struct XsdIssue { pub code: i32, pub domain: i32, pub element: Option<String>, pub facet: Option<XsdFacet>,
                      pub path: Option<FindingPath>, pub line: Option<u32>, pub col: Option<u32> }

/// The watcher's only network path: through the bank's declared proxy, to allow-listed hosts (DR-71).
#[async_trait]
pub trait SchemaFetcher: Send + Sync {
    async fn attachment_list(&self, url: &Url) -> Result<Vec<Attachment>, FetchError>;   // JSON, at most 1 MiB
    async fn download(&self, url: &Url, max_bytes: u64) -> Result<Bytes, FetchError>;   // host allow-list checked first
}
```

  Reused from phase 02: `EvidenceStore` (`put_generated` for the XSDs, the new transient methods of section 3.2),
  the clamd client used by `evidence_scan`.

- **utoipa schemas:** `CheckRequest` (multipart: `files[]`, `xml_text?`, `rejection_text?`, `profile_id?`),
  `CheckAccepted` (`run_id`, `Location`), `CheckRunView` (status, profile ref, files with outcome, score, grade and
  counts, summary; `file_names` unsealed only for the requester, section 3.8), `CheckFindingView` (layer, rule code,
  severity, path, line, column, params, and `title`, `explain`, `fix` rendered from the message catalogue in the
  request language), `ProfileView` (all columns except the XSD bytes; `lookups` collapsed to list names and counts
  unless `?lookups=true`), `ProfileSummary`, `ProfileChangeRequest` (`promote: bool` or a `ProfileAmendment`, plus the
  change fields `reason_code`, `reason_text`, `effective_at?`, `accept_corpus_failures?`), `WatchRunView`,
  `WatchAccepted`. Change-request responses reuse the phase 04 `ChangeRequestView` (202).

- **Route table** (base `/api/v1`; every route names its Cedar action, section 3.8; **CR** = answers 202 with a change
  request, phase 04):

| Method | Path | Scope | Description |
|-|-|-|-|
| POST | /goaml/check | report.prepare | Upload goAML XML or ZIP files (or paste one) for checking against the active profile; optional Office rejection text; 202 with the run id (`goaml_check.run`) |
| GET | /goaml/check-runs | report.prepare | Check runs, newest first: the caller's own, or all for the MLRO (`goaml_check.read`) |
| GET | /goaml/check-runs/{id} | report.prepare | Run status and per-file outcome, estimated score and grade, counts by severity (`goaml_check.read`) |
| GET | /goaml/check-runs/{id}/findings | report.prepare | Findings with path, line, explanation and fix in the request language: `file?`, `layer?`, `severity?` (`goaml_check.read`) |
| GET | /goaml/profiles | report.prepare, config.author, config.approve | Profile versions: `code?`, `status?` (`goaml_profile.read`) |
| GET | /goaml/profiles/{id} | report.prepare, config.author, config.approve | One version: patches, rules, quality, limits, emit flags, open questions, diff from its base, corpus result; `?lookups=true` (`goaml_profile.read`) |
| PUT | /goaml/profiles/{id} | config.author | Promote a candidate or propose an amendment of a version; **CR** `goaml_profile` (`change_request.create`, family `reporting`) |
| GET | /goaml/watch-runs | report.prepare, config.author | Watcher runs with the attachment list as returned and the candidate created (`goaml_watch.read`) |
| POST | /goaml/watch-runs | report.prepare, config.author | Run the watcher now (202); refused while the watcher is disabled (`goaml_watch.run`) |

API.md lists `GET /goaml/profiles` and the watch runs under `report.prepare` only; they now take `config.author` too,
because the configuration author proposes profile changes and must read what they change (aligned in the same change). The two profile reads also take `config.approve`, because the checker reads the version it decides (section 6); Cedar still limits them to A2, M, CFG and CHK.

- **Router function:** `goaml::router() -> OpenApiRouter<AppState>` (9 routes), merged in `build_app` under the authz
  layer. `POST /goaml/check` and `POST /goaml/watch-runs` take `Idempotency-Key`; `PUT /goaml/profiles/{id}` takes
  `If-Match` with the version row's `version` as the change request's base. `POST /goaml/check` has its own body
  limit: `goaml.check_max_files` x `goaml.check_max_bytes` plus multipart overhead, streamed to the transient store
  (never held whole in memory).

## 3. Business Logic

### 3.1 The shipped profile and the loader (`ProfileLoader`, `akr-ctl packs seed`; RP-01, RP-02, RP-04, RP-05, RP-09, DR-68)

`ProfileLoader::load(xsd: &[u8], profile: &ProfileJson) -> LoadedProfile` (pure, `akr-goaml`):
1. Parse the XSD with `roxmltree` (no DTD; a `<!DOCTYPE` refuses the file as `xsd_unparseable`). Read `goaml:schemaVersion` (absent ->
   `xsd_unparseable`).
2. Extract `XsdFacts`: every element with its type, `minOccurs`, `maxOccurs`, length and pattern facets and choice
   groups; every enumeration with its `xs:documentation` label (lookups; RP-02); the `supportedIn` annotations
   (`5.0`, `5.1`, `5.2`) and the deprecation notes. `lookups` and `schema_summary` are written from them (for the
   shipped file: 19 `report_code` values, 62 report-indicator codes, the 5.1 list of 25 distinct elements and the 5.2
   list of 29, three deprecation notes).
3. Apply the patches in order (`PatchApplier`): each `xpath` must select exactly one facet whose current value equals
   `from` (else `patch_orphaned`, kept in the result, the patch not applied). The shipped profile has one patch,
   `P-01` (RP-04): the entity `name` pattern `^[a-zA-Zء-ي\s'&-]*[a-zA-Zء-ي][a-zA-Zء-ي\s'&-]*$` is replaced by the same
   class without the literal `^` and `$`, because under XSD 1.0 rules no ordinary entity name can match the published
   facet (research file 23 s.2.5); the patch names open question `OQ-03` (how the Office's server treats it).
4. Serialise the effective XSD; validate the profile JSON against `goaml-profile.v1.json` and each rule's parameter
   schema (unknown rule code -> refused; the catalogue is closed, section 3.3); weights of `quality.criteria` sum to
   100 and bands descend (else `params_invalid`).

`akr-ctl packs seed` (phase 06 command, extended): after the CBI pack, for each `backend/packs/goaml/*/` directory in
the manifest, verify the files' SHA-256 against the manifest, `load`, and when no version with this `xsd_sha256` and
content hash exists for the code: `put_generated` the original and the effective XSD (evidence kind `snapshot`, owner
kind `goaml_profile`, retention R9, classification `internal`), insert the version (`source = shipped`), and, when the
code has no active version, activate it directly (`status = active`, `effective_from = now()`, no change request: a
release is the basis, and the audit event says so); a shipped version for a code that already has an active one
stays `candidate` and goes through a change request like any other. Enqueue `goaml_corpus {profile_id}`. Audit
`goaml_profile.seeded {code, profile_version, xsd_sha256, activated}`. `packs verify-data` adds: one active `IQ`
version; 19 report codes; 62 indicator codes; patch `P-01` applied.

**The shipped `profile.json`** holds: the patch above; `rules` with every catalogue code enabled at its default
severity (section 3.3); `quality` (section 3.4); `limits` (`max_file_bytes` 52,428,800, `max_attachment_bytes`
20,971,520, `max_transactions` 1,000, `max_attachments` 20, `attachment_types` PDF, JPEG, PNG, `xml_per_zip` 1; all
open question `OQ-05` until the Office states its own); `emit_flags` all `false` (`report_date`, `account_type`,
`related_persons`, `elements_5_1`, `elements_5_2`; RP-05); `rejection_patterns` (section 3.7); `open_questions`
`OQ-01` to `OQ-06` from research file 23 s.9 and RQ-08 (code-table meanings, report-date element, rejection messages
and purge period, TFR after a true match, the entity-name pattern on the server, the Office's limits).

### 3.2 Checking files (`CheckService`, job `goaml_check`; RP-27, DR-69, DR-70)

`POST /goaml/check` (akr-api):
1. Cedar `goaml_check.run`; `Idempotency-Key` required.
2. Parts: `files[]` (1 to `goaml.check_max_files`; declared type `application/xml`, `text/xml` or `application/zip`;
   each at most `goaml.check_max_bytes`, itself capped by the profile's `max_file_bytes`) or `xml_text` (one pasted
   file, at most 8 MiB, else `file_too_large`); `rejection_text` (at most 20,000 characters, else `file_too_large` at `/rejection_text`; only with exactly one file) and `profile_id`
   (default the active `IQ` version; a `candidate` only for M, to test a new schema against real files; CFG tests
   candidates through the corpus).
   Violations -> 422 `E_VALIDATION` (`too_many_files`, `file_too_large`, `type_not_allowed`, `rejection_needs_one_file`,
   `profile_not_usable`).
3. Insert the run (`queued`, the profile pinned, `files` with n, SHA-256, size, kind; `file_names` sealed under the
   `secret-str` family with AAD = run id).
4. Each part streams through `EvidenceStore::stage_transient(purpose = "goaml", owner = run_id, n: u8, stream)` (`n` is 1 to 50 for a file and 0 for the rejection text; `stream` is the part as `Stream<Item = Result<Bytes, io::Error>>`, never collected): sealed as AKRE1
   with the month's `evidence` family key and written to `akr-quarantine` at `transient/goaml/<run_id>/<n>` with the
   stager credential; no evidence row (the rejection text is part `0`). A failure rolls back the run and deletes what
   was written.
5. Enqueue `goaml_check {run_id}`; audit `goaml_check.requested {run_id, files, bytes, profile: ref, with_rejection}`;
   202 with `Location`.

`goaml_check` (akr-worker; concurrency 2; max attempts 3; per file a CPU budget, section 3.2 step 6):
1. `start` returns the run when it moved it from `queued` to `running` or when it is already `running` (a retry, which goes on from its first file without a recorded result, section 4), and `None` for a `completed` or `failed` run -> done. Load the pinned profile; the effective XSD comes from the worker's cache
   (tmpfs, keyed by SHA-256, filled by `EvidenceStore::open`); a profile that cannot be opened -> `fail
   (profile_unavailable)`.
2. For each file `n`: `open_transient`; SHA-256 must equal the row's (else block `PRE-01` `hash_mismatch` and skip).
3. **Antivirus** with the phase 02 clamd client (`INSTREAM`): stale signatures -> the job retries and finally fails
   `signatures_stale` (fail closed); `FOUND` or `Heuristics.Encrypted` -> outcome `refused`, block `PKG-05`, no
   parsing.
4. **Package (L5):** a ZIP is read in memory with `zip` (no extraction): at most `xml_per_zip` XML entries and
   `max_attachments` others; entry names without `..`, absolute paths or backslashes (`PKG-04`); declared and actual
   sizes within limits and a compression ratio of at most 100 per entry (`PKG-06`, a zip bomb); attachment types by
   magic bytes in `attachment_types`; each attachment scanned by clamd. The XML entry continues; a plain XML file
   continues as is. File size over `max_file_bytes` -> `PKG-01` block.
5. **Pre-screen (`PreScreen`, quick-xml, streaming):** UTF-8 (a BOM is accepted; any other encoding declaration ->
   `PRE-02` block); no `<!DOCTYPE`, no entity reference other than the five predefined and numeric ones, no
   processing instruction but the XML declaration (`PRE-03` block: the file is not parsed further, which removes
   entity expansion and external fetches before any parser sees them); depth at most 64, at most 2,000,000 elements
   and 64 attributes per element (`PRE-04` block); exactly one root `report` (`PKG-03`). It also counts
   transactions (`PKG-02` against `max_transactions`) and records element positions for later rules.
6. **XSD (L1):** `XsdValidator::validate` runs `akr-xsdcheck` once per file: the executable reads the effective XSD
   path and the XML on stdin, validates with libxml2 (`XML_PARSE_NONET`, no `XML_PARSE_NOENT`, no `XML_PARSE_DTDLOAD`,
   `XML_PARSE_HUGE` off), and writes one JSON line per structured error with code, domain, element, facet, node path
   (`xmlGetNodePath`) and line and column, never the message text (which quotes values). It runs as a child of
   akr-worker with no network namespace, `RLIMIT_CPU` 30 s, `RLIMIT_AS` 1 GiB, `RLIMIT_NOFILE` 16, a seccomp allow-list
   (read, write, mmap, brk, exit and the few calls libxml2 needs; no `socket`, `open` only of the XSD path given),
   and a wall clock of 60 s; a crash, a kill or a time-out is block `XSD-99` (`validator_failed`) for that file and the
   run continues. Each issue maps to a rule code by facet: `XSD-01` missing required element, `XSD-02` unexpected
   element or order, `XSD-03` enumeration (an unknown lookup code fails the whole file at the Office; RP-02), `XSD-04`
   pattern, `XSD-05` length, `XSD-06` date-time, `XSD-07` decimal or integer, `XSD-08` fixed value, `XSD-09` other;
   all `block`.
7. **Compatibility (L1a):** `COMPAT-01` (warn): an entity name that passes the patched facet but would fail the
   conservative reading (digits, dots, brackets, diacritics U+064B to U+0652, Kurdish and Persian letters outside
   U+0621 to U+064A), with the offsets of the characters, never the characters; open question `OQ-03`.
8. **Rules (L2)** and **consistency (L4)** over a `roxmltree` tree of the same bytes (section 3.3).
9. **Quality (L3):** only when the file has no `block` finding: `Scorer::score` (section 3.4). Otherwise the
   outcome is `would_reject` and no grade is given.
10. **Office message (layer `office`):** when part `0` exists, `RejectionParser::parse` (section 3.7).
11. In one transaction per file: insert its findings in batches and record the file result in the run's `files` entry with `GoamlCheckRepository::record_file(tx, id, n, &FileResult)`, so a retry never repeats a file's findings (`report_code`, transaction count, outcome, score, grade,
   counts).
12. One transaction: `complete` with the files and the summary (`CheckSummary`: counts per layer and per severity, block, warn and info; outcome counts; grade counts, good, moderate, acceptable and poor; the blocks and warns totals); audit `goaml_check.completed {run_id, outcomes:
   {would_reject, graded, refused}, grades: {...}, blocks, warns}`.
13. Delete every transient object of the run (`delete_transient`), then `mark_transient_deleted`. A failed run does
   the same in its failure path. `maintenance_sweep` (phase 00, extended) deletes transient objects older than
   `evidence.staging_ttl_h` whose run is final or missing, and sets the flag; `akr_goaml_transient_overdue` counts
   runs whose files outlive the TTL.

**What is kept** (DR-69): per file its SHA-256, size and kind, and the findings: rule code, severity, element path
with positions, line, column and `FindingParams` (lengths, counts, offsets, expected lookup lists from the profile,
element names). Never a value from the file, never the libxml2 message, never the rejection text. The requester
already holds the file; the finding tells them where to look and what to change. A test (section 6) feeds files whose
every text node is a unique marker string and asserts that no marker appears anywhere in the database, the logs, the
audit trail or the job payloads.

### 3.3 The rule catalogue (L2 and L4; RP-07, RP-28, RP-30, RP-83)

Rules are Rust functions in `akr-goaml`, closed and versioned with the release; a profile can enable or disable each,
set its severity and set its parameters (the shipped defaults below), never add code. Each has unit tests with
positive and negative fixtures, and every corpus case names the codes it must produce. Messages come from
`messages.v1.json` in both languages.

| Code | Check (on the uploaded XML) | Default | Parameters |
|-|-|-|-|
| BR-01 | `reason` present and not blank (after trimming) | block | |
| BR-02 | at least one `report_indicators/indicator` | block | |
| BR-03 | `action` present for STR, SAR, TFR (the Office's manual requires the action taken) | warn | `report_codes` |
| BR-04 | each transaction has date, amount, `transmode_code`, and a counterparty side | block | |
| BR-05 | every person node has a non-empty `mothers_name` (the XSD accepts an empty element) | warn | |
| BR-06 | `late_deposit` given exactly when `date_posting` is given | block | |
| BR-07 | no transaction date after the report or submission date | block | |
| BR-08 | transaction older than N days before submission: timeliness (the legal standard is "immediately") | warn | `days` 31 |
| BR-09 | account-number hygiene: no spaces, no leading `/` or `\`, not mostly letters, not equal to the amount | warn | |
| BR-10 | a bank GL account (by `gl_account_patterns`) appears as a party | warn | from `goaml.bank_identifiers` |
| BR-11 | my-client consistency: an account at the bank's SWIFT or institution code sent as not-my-client, or a my-client account at another institution | block | from `goaml.bank_identifiers` |
| BR-12 | an account with both an entity owner and signatories | warn | |
| BR-13 | CTR shape: cash mode on one side and an account on the other, both countries domestic, amount at least the threshold | warn | `threshold_usd` 10,000; `domestic` [IQ] |
| BR-14 | placeholder words in names, occupations or descriptions ("not found", "n/a", "dummy", "غير متوفر", "لا يوجد") | warn | `placeholders` |
| BR-15 | empty `id_number`, `occupation` or `source_of_wealth` on a my-client person | warn | |
| BR-16 | negative amount, or digits in a person name (both pass the XSD) | warn | |
| BR-17 | `rentity_id` or `rentity_branch` differs from the bank's | block | from `goaml.bank_identifiers`; skipped while unset (info `BR-17` once) |
| BR-18 | currency other than IQD and USD without a foreign-currency block | warn | `currencies` |
| NORM-01 | a date-time without time or before 1753-01-01 (the XSD also fails it; this names the fix) | block | |
| NORM-02 | Arabic presentation forms (U+FB50 to U+FDFF, U+FE70 to U+FEFF): NFKC changes the text | warn | |
| NORM-03 | bidirectional controls (U+200E, U+200F, U+202A to U+202E, U+2066 to U+2069) | warn | |
| NORM-04 | control or non-printable characters | block | |
| NORM-05 | non-ASCII digits in a numeric or date field | block | |
| NORM-06 | leading or trailing spaces in a value | info | |
| EMIT-01 | an element flagged 5.0, 5.1 or 5.2 used while its emit flag is off: the Office has not confirmed it | warn | |
| DEP-01 | a deprecated element (`submission_date`, `personal_account_type`, `signatory`) used | info | |
| CONS-01 | an amount stated in `reason` (Western or Arabic-Indic digits, with or without separators and currency words) that matches no transaction amount and no sum of them within 1 unit | warn | `tolerance` 1 |
| CONS-02 | a date stated in `reason` outside the transactions' date range by more than N days | warn | `days` 7 |
| CONS-03 | an STR, SAR or TFR whose `reason` is shorter than N characters | warn | `min_chars` 400 |
| CONS-04 | transactions not in chronological order by `date_transaction` | info | |

The business-rejection-rule corpus (RP-83) is the corpus under `corpus/cases/` plus the cases a profile amendment
adds (section 3.6): each case is a file, the rule codes it must produce and the Office message that revealed the rule,
when one did. Phase 21 adds the bank's own anonymised rejection cases (the FIU-answers corpus).

### 3.4 Quality score and estimated grade (`Scorer`; RP-29, DR-72)

Only a file with no `block` finding is scored. The score is 0 to 100, the sum of the weights of the criteria met
(`quality.criteria`; shipped weights in brackets), following research file 23 s.5.4:
`Q-01` structured party data complete: every person with first and last name, mother's name, birth date, ID and
nationality, every my-client entity with incorporation data (20, pro rata over parties); `Q-02` every transaction
complete: date, amount, currency, mode, counterparty (15, pro rata); `Q-03` narrative length at least
`CONS-03.min_chars` (10); `Q-04` narrative states amounts that match the structured data (`CONS-01` clean and at least
one amount stated) (10); `Q-05` narrative states dates inside the range (`CONS-02` clean and at least one date) (5);
`Q-06` action taken present (10); `Q-07` at least two indicators, or one plus a predicate-offence indicator (10);
`Q-08` previous report referenced when `fiu_ref_number` or `prev_rejected_ref_number` applies, or no reference needed
(5); `Q-09` attachments present in the ZIP when the narrative mentions documents (5); `Q-10` no warning from BR-05,
BR-14, BR-15 (10).

Bands (`quality.bands`): `good` at 85 and above, `moderate` 70-84, `acceptable` 50-69, `poor` below 50. The grade is
shown as "estimated" everywhere (API field `grade_basis: "estimated"`, screen label, printed page): the Office's own
grading form is not public (open question `OQ-01`), so the estimate is advice and never presented as the Office's
grade. The phase 21 calibration sample compares it with the grades the Office returns.

### 3.5 The Office schema watcher (`goaml_watch`; RP-03, DR-71)

Scheduled daily (`schedule.goaml_watch`, 05:10 bank time) when `goaml.watch_enabled` is true; manual through `POST
/goaml/watch-runs` (Cedar `goaml_watch.run`; disabled -> 409 `E_NOT_ALLOWED_IN_STATE` `watch_disabled`; one running
watch per code by index -> 409 `run_active`; audit `goaml_watch.requested`; 202). Steps:
1. `start` the run (`trigger`, `source_url` = `goaml.watch_url`).
2. `SchemaFetcher::attachment_list`: HTTPS through the egress proxy declared at install (`deploy/zones.yaml`
   `egress.proxy`, rendered into the worker's environment; no proxy declared -> `proxy_unreachable`), TLS verified
   against the system roots plus the bank's CA bundle, 30 s time-out, at most 1 MiB of JSON; the host must be in
   `goaml.watch_hosts` (else `host_not_allowed`). Store the list as returned (titles, URLs, sizes, dates).
3. Pick the schema entry by `goaml.watch_match` (none -> `not_found_in_list`); download it (at most 5 MiB, host
   allow-listed, redirects only to allow-listed hosts); SHA-256.
4. Equal to the active version's `xsd_sha256`, or to an existing candidate's -> `unchanged`.
5. New: `ProfileLoader::load` with the active version's profile content (patches, rules, quality, limits, flags,
   patterns, questions) over the new XSD (unparseable -> `failed` `xsd_unparseable`); store both XSDs as evidence;
   insert a `candidate` version (`source = watcher`, `based_on` = the active version, `source_ref` = URL, title and
   date); compute `diff_from_base` (section 3.6); enqueue `goaml_corpus`; `changed` with the candidate id; audit
   `goaml_profile.candidate_created {code, profile_version, xsd_sha256, orphaned_patches}` and `goaml_watch.changed`.
6. Network failures -> `unreachable` with the error code; anything else -> `failed`. Metric
   `akr_goaml_watch_last_success_timestamp`; alert rule when older than `goaml.watch_stale_days` while enabled.

`akr-ctl goaml import-xsd FILE [--note TEXT]` (zone Z5; OS user and PAM session recorded) does steps 4 and 5 from a
file the bank downloaded itself, for an installation with no egress (`trigger = import`, `source_ref = {file_sha256,
note}`), so an air-gapped bank follows the same promotion path. Nothing the watcher finds is active until a change
request promotes it.

### 3.6 Profile changes: diff, corpus and the `goaml_profile` change target (RP-03, RP-06, DR-68)

**`SchemaDiff::diff(base, new) -> ProfileDiff`** (pure): elements added and removed; cardinality, length and pattern
changes per element path; lookup values added and removed per list (with labels); `supportedIn` and deprecation
changes; patches orphaned; and for amendments the rule, quality, limit, flag and pattern changes field by field. Shown
on `GET /goaml/profiles/{id}` and in the change request's diff.

**`goaml_corpus`** (akr-worker; payload `{profile_id}`; concurrency 1): run every corpus file through steps 4-9 of
section 3.2 against the version (no transient store: the corpus is release data), compare with `cases.json` (outcome
and the exact set of block codes; warnings may differ and are listed), and `set_corpus_result {ran_at, passed,
failed, cases: [{file, expected, got}]}`; audit `goaml_profile.corpus_ran {code, profile_version, passed, failed}`.
`akr-ctl goaml corpus [--profile ID]` runs the same in the foreground with exit code 0 when all pass. CI runs it
against the shipped profile on every change to `akr-goaml`.

**Change target `goaml_profile`** (phase 04 `ChangeTarget`, registered in akr-api and akr-worker; family
`reporting`; `generic_create = false`; `emergency_allowed = false`; `reason_families = ["change"]`):

| Field | Value |
|-|-|
| Route | by class, `cr_material` (the shipped route map's class rule); maker CFG, checker per the route (CHK or M) |
| `object_key` | `goaml_profile:<code>` (one open change per profile code) |
| Base version | the active version's `profile_version` |
| Prepare | `promote`: the target is a `candidate` of this code with `corpus_result` present (else 422 `corpus_not_run`); failed cases need `accept_corpus_failures` with a reason of at least 20 characters (else 422 `corpus_failed`); `amend`: the body validates against `goaml-profile.v1.json` and the rules' parameter schemas, regexes compile under the `regex` crate (no backtracking) and are at most 500 characters, limits within the hard caps (`max_file_bytes` 300 MiB, `max_transactions` 100,000); a new `candidate` version is inserted (`source = amendment`, `based_on` = the target) and `goaml_corpus` runs before the checker can decide (`required_evidence = [validation_run]`, generated from `corpus_result`). Class `material`; volume effect `neutral`, except that disabling a `block` rule or lowering it to `warn` or `info` sets `loosening` and requires an answered open question or a reason of at least 50 characters |
| Activate | `GoamlProfileRepository::activate` (candidate -> `active`, previous active -> `retired`, `effective_from` = the request's `activated_at`); runs already queued keep their pinned version; audit `goaml_profile.activated {code, profile_version, from_version, change_request_id}` |
| Revert | not allowed (a new request promotes or amends again) |
| Closed | an `amendment` candidate -> `rejected`; a watcher or import candidate stays `candidate` (it is the Office's file; the bank may promote it later) |

### 3.7 Office rejection messages (`RejectionParser`, stub; RP-42)

`parse(text, tree, patterns) -> Vec<OfficeFinding>`: normalise the text (NFKC, whitespace folded); for each profile
pattern in order, a match yields a finding in layer `office` with the pattern's `rule_code`, and, when the pattern
captures an element name (`(?P<element>...)`), the paths of that element's occurrences in the file (at most 20), with
lines. Text no pattern matches gives one `OFF-99` info finding with the count of unmatched lines. The shipped patterns
are generic goAML schema-message shapes (`OFF-01` invalid child element, `OFF-02` value fails a pattern or
enumeration, `OFF-03` required element missing, `OFF-04` business rule with a named element), marked with open
question `OQ-04`: the Office's own message wording is not known (RQ-08). The text is never stored: only the findings.
Each real message the bank meets becomes a pattern through a profile amendment (and, from phase 21, a corpus case).

### 3.8 Errors, audit events, Cedar actions

**Error mapping.** No new codes. Used: `E_SCOPE`; `E_FORBIDDEN`; `E_NOT_FOUND` (also a masked run of another user);
`E_VERSION_CONFLICT`; `E_PRECONDITION_REQUIRED`; `E_STALE_BASE` (the change target); `E_IDEMPOTENCY_*` (phase 00);
`E_PAYLOAD_TOO_LARGE` (phase 00 body limit); `E_NOT_ALLOWED_IN_STATE` with reasons `watch_disabled`, `run_active`;
`E_VALIDATION` with reasons `too_many_files`, `file_too_large`, `type_not_allowed`, `rejection_needs_one_file`,
`profile_not_usable`, `corpus_failed`, `corpus_not_run`, `rule_unknown`, `params_invalid`, `pattern_invalid`, `limit_out_of_bounds`,
`loosening_needs_basis`. `GoamlError`, `CheckError`, `WatchError` and `XsdError` (`thiserror`) map in `api::error` or
end the job.

**Audit events emitted:**

| Event kind | On | Resource |
|-|-|-|
| `goaml_check.requested`, `.completed`, `.failed` | check lifecycle (run id, file count, bytes, profile ref, outcomes, grades, counts; never names or values) | goaml_check_run |
| `goaml_check.transient_deleted` | the run's staged files deleted (by the job or the sweep) | goaml_check_run |
| `goaml_profile.seeded` | `packs seed` installed a shipped version | goaml_profile |
| `goaml_profile.candidate_created` | watcher, import or amendment created a candidate | goaml_profile |
| `goaml_profile.corpus_ran` | corpus result recorded | goaml_profile |
| `goaml_profile.activated`, `goaml_profile.rejected` | the change target | goaml_profile |
| `goaml_watch.requested`, `.changed`, `.unchanged`, `.unreachable`, `.failed` | watcher runs | goaml_watch_run |

Reused: `change_request.*` (phase 04); `evidence.stored` (phase 02) for the XSDs; access events (phase 03) when the
MLRO reads another user's run.

**Cedar** (bundle file `A0_reporting.cedar`, the reporting range of phase 03). Entities added to
`schema.cedarschema`: `GoamlProfile { code: String, status: String }`, `GoamlCheckRun { requested_by: User, tier:
String }` (tier `suspicion` for every run); the `ChangeKind` family `reporting` (kind `goaml_profile`).

| Action | Resource | Who (shipped bundle) |
|-|-|-|
| `goaml_check.run` | System | A2, M |
| `goaml_check.read` | GoamlCheckRun | the requester; M (`read_all`, with an access event) |
| `goaml_check.read_all` | System | M |
| `goaml_check.use_candidate` | GoamlProfile | M |
| `goaml_profile.read` | System, GoamlProfile | A2, M, CFG, CHK |
| `goaml_watch.read` | System | A2, M, CFG |
| `goaml_watch.run` | System | M, CFG |
| `change_request.create` (family `reporting`) | ChangeKind | CFG |
| `change_request.read` (family `reporting`) | ChangeRequest | CFG, CHK, M, IA, or a member of `parties` |

```cedar
// A0_reporting.cedar (excerpt)
@id("goaml.check_preparers") @role("A2,M")
permit (principal, action == Akr::Action::"goaml_check.run", resource)
when { principal.roles.containsAny(["A2", "M"]) };

@id("goaml.check_own")
permit (principal, action == Akr::Action::"goaml_check.read", resource is Akr::GoamlCheckRun)
when { resource.requested_by == principal };

@id("goaml.check_mlro") @role("M")
permit (principal, action in [Akr::Action::"goaml_check.read", Akr::Action::"goaml_check.read_all", Akr::Action::"goaml_check.use_candidate"], resource)
when { principal.roles.contains("M") };

// Another preparer's run does not exist for the caller (existence masking).
@id("goaml.check_masked") @deny("masked")
forbid (principal, action == Akr::Action::"goaml_check.read", resource is Akr::GoamlCheckRun)
when { resource.requested_by != principal && !principal.roles.contains("M") };

@id("goaml.profile_readers") @role("A2,M,CFG,CHK")
permit (principal, action == Akr::Action::"goaml_profile.read", resource)
when { principal.roles.containsAny(["A2", "M", "CFG", "CHK"]) };

@id("goaml.watch_readers") @role("A2,M,CFG")
permit (principal, action == Akr::Action::"goaml_watch.read", resource)
when { principal.roles.containsAny(["A2", "M", "CFG"]) };

@id("goaml.watch_runners") @role("M,CFG")
permit (principal, action == Akr::Action::"goaml_watch.run", resource)
when { principal.roles.containsAny(["M", "CFG"]) };
```

Attributes evaluated: `principal.roles`, `delegated_roles`; `resource.requested_by`, `resource.status`. The MLRO's
read of another user's run records an access event (`view`, tier `suspicion`). CHK reads profiles to decide profile
changes and holds no `goaml_watch.read`; the watch routes refuse CHK at the scope gate (section 6, 403 `E_SCOPE`).
Bundle tests: A1, KYC, L, OPS, DS and VS reach none of these actions; A2 cannot
read another A2's run (404, byte-identical to a missing id); CFG cannot run a check; nobody but CFG can propose a
profile change.

**Tipping-off review.**
- The uploaded files are the bank's suspicion reports. They exist only sealed in the transient store for the length
  of the run (DR-69); nothing derived from them holds a value, a name or the narrative. The run rows are `secret-str`,
  readable by the requester and the MLRO only, masked for everyone else, and excluded from any count shown to other
  roles.
- No result leaves AKR-TMS: nothing is sent to the Office or to anyone else; the result page is printed by the user.
- The watcher's only outbound traffic is a GET of the Office's public attachment list and schema through the bank's
  proxy; it carries no bank data and no identifier beyond the bank's network address.
- Audit details carry ids, hashes, counts and codes; the file names stay sealed in the run row.
The answer for a person outside the requester and the MLRO: nothing, not even that a check ran.

**Settings and configuration used:** the nine keys of migration 0025 (registry bounds: `goaml.check_max_files` 1-50;
`goaml.check_max_bytes` 1 MiB to the profile's `max_file_bytes`; `goaml.watch_url` an `https` URL whose host is in
`goaml.watch_hosts`; `goaml.watch_hosts` at most 10 host names; `goaml.watch_stale_days` 1-60;
`schedule.goaml_watch` a cron at most hourly; `goaml.bank_identifiers` validated against its JSON schema, patterns
compiled with `regex`); the profile's `limits`; phase 02 `evidence.staging_ttl_h`; install-time `egress.proxy`.

## 4. Infrastructure Updates

- **Job kinds registered** (all akr-worker; the `goaml_` prefix belongs to `akr_app`):
  - `goaml_check` (payload `{run_id}`; on demand from `POST /goaml/check`; concurrency 2; max attempts 3; a retry
    restarts the run's files from the first file without a recorded result; payload and job error text carry ids and
    codes only).
  - `goaml_watch` (payload `{run_id}`; scheduled by `schedule.goaml_watch`, daily 05:10 bank time, only while
    `goaml.watch_enabled` is true; manual through `POST /goaml/watch-runs`; concurrency 1; max attempts 2; network
    failures end the run `unreachable` without retry).
  - `goaml_corpus` (payload `{profile_id}`; enqueued by `packs seed`, by every new candidate (watcher, import,
    amendment); coalesce key `goaml_corpus:<profile_id>`; concurrency 1; max attempts 2).
  - `maintenance_sweep` (phase 00) extended: transient `goaml` objects older than `evidence.staging_ttl_h` whose run is
    final or missing are deleted and the run flagged `transient_deleted` (audit `goaml_check.transient_deleted` with
    `by: sweep`).
- **Docker compose:**
  - The akr-worker image adds the `akr-xsdcheck` executable (built from `tools/xsdcheck`, statically linked against
    the pinned libxml2 2.13 release; the runtime image carries no other libxml2) and a seccomp profile file for it.
    The worker gets a `tmpfs` at `/run/akr/xsd` (64 MiB, `noexec`) for the effective-XSD cache.
  - New network `z2_egress` (not `internal`), attached to `akr-worker` only, and created only when
    `deploy/zones.yaml` declares `egress.proxy` (host and port). `akr-ctl net render` adds host nftables rules that let
    the worker's address on `z2_egress` reach that one host and port and nothing else; without a declared proxy the
    network is not created and the watcher reports `proxy_unreachable`. The proxy address reaches the worker as
    `AKR_EGRESS_PROXY` (rendered, not a setting: it is install-time infrastructure).
- **AppState additions:** `api`: `goaml: Arc<GoamlQuery>` (profiles, check runs, findings, watch runs),
  `goaml_check: Arc<CheckService>` (the request half), the change target `goaml_profile` in the target registry, the
  multipart limits from the settings. `worker`: the three handlers, `ProfileLoader`, `PatchApplier`, `SchemaDiff`,
  `PreScreen`, `SandboxedXsdValidator` (path to `akr-xsdcheck`, seccomp profile, limits), `RuleCatalogue`, `Scorer`,
  `RejectionParser`, `ProxySchemaFetcher`, `CorpusRunner`, the profile cache (by profile id, effective XSD path by
  SHA-256), the change target `goaml_profile` (activation side). `ctl`: `goaml import-xsd`, `goaml corpus`, `packs
  seed` and `packs verify-data` extended.
- **OpenAPI registration:** tag `goaml`, 9 operations, each with `x-akr-action`. `POST /goaml/check` documents
  `multipart/form-data` (`files[]` binary, `xml_text`, `rejection_text`, `profile_id`), `Idempotency-Key` and 202 with
  `Location`; `PUT /goaml/profiles/{id}` documents `If-Match` and 202 `ChangeRequestView` (phase 04); `POST
  /goaml/watch-runs` documents 202 and the 409 reasons. `GoamlProfileView` references `GoamlProfileV1` generated from
  `goaml-profile.v1.json`; `FindingView.params` is the closed `FindingParams` schema (no free-text field).
- **Network zones:** one new edge, akr-worker to the bank's declared HTTPS proxy over `z2_egress` (ARCHITECTURE zone
  Z2 lists the bank proxy as the only outbound path). akr-api, akr-ingestd and akr-assess gain nothing; nothing in this
  phase reaches a CBS; `akr-xsdcheck` runs with no network namespace at all.
- **Readiness:** akr-worker `/readyz` adds `xsdcheck` (at start the worker runs `akr-xsdcheck --self-test`, which
  validates a built-in sample against the shipped effective XSD inside the sandbox; failure -> not ready, the
  `goaml_check` handler is not registered and runs stay `queued`); akr-api `/readyz` adds `goaml_profile` (one active
  `IQ` version).
- **Metrics:** `akr_goaml_check_files_total{outcome}` (counter: `graded`, `would_reject`, `refused`),
  `akr_goaml_check_seconds` (histogram per file), `akr_goaml_xsdcheck_failures_total{reason}` (crash, time-out,
  CPU, memory), `akr_goaml_transient_overdue` (gauge), `akr_goaml_watch_last_success_timestamp` (gauge). Alert rules:
  `akr_goaml_transient_overdue > 0`; watcher success older than `goaml.watch_stale_days` while enabled; a `candidate`
  version waiting more than 14 days without a change request; any `akr_goaml_xsdcheck_failures_total` increase.
- **Ops cockpit:** a goAML line (phase 03 `GET /admin/ops`): watcher enabled, last success, last result, a candidate
  pending (version and age), transient objects overdue. No file or run detail.
- **Existing entity, schema or route updates:**
  - Phase 00: `deploy/zones.yaml` key `egress.proxy` and the `z2_egress` network in `akr-ctl net render`;
    `cargo deny` bans `libxml`, `libxml2-sys` and any crate linking libxml2 outside `tools/xsdcheck`;
    `maintenance_sweep` extended as above.
  - Phase 01: `fixtures/dev-users.yaml` gains a second A2 user (`a2b@bank.test`) for the existence-masking test.
  - Phase 02: `EvidenceStore::stage_transient(purpose, owner, n, stream)`, `open_transient(purpose, owner, n)` and
    `delete_transient(purpose, owner)` (sealed AKRE1 objects under `transient/<purpose>/` in `akr-quarantine`, no
    evidence row, s3guard policy for the prefix: stager credential writes, worker credential reads and deletes);
    evidence owner kind `goaml_profile` (R9).
  - Phase 03: Cedar file `A0_reporting.cedar`, entities `GoamlProfile` and `GoamlCheckRun` in `schema.cedarschema`,
    the cockpit line.
  - Phase 04: change target `goaml_profile`; `ChangeKind` family `reporting`; the route map's class rule sends it to
    `cr_material` with no new rule.
  - Phase 06: `packs seed` and `packs verify-data` extended to the goAML profile directories.
  - Planning docs aligned in the same change: API.md 3.13; DATA-MODEL s.14, s.21 and s.23; MODULES M13; ARCHITECTURE
    zone note; roadmap row 07.

## 5. Frontend Impact

Backend phase for screens; phase 08 builds them on this API. `make generate-api` regenerates the client with the
hooks `usePostGoamlCheck`, `useGetGoamlCheckRuns`, `useGetGoamlCheckRunsId`, `useGetGoamlCheckRunsIdFindings`,
`useGetGoamlProfiles`, `useGetGoamlProfilesId`, `usePutGoamlProfilesId`, `useGetGoamlWatchRuns`,
`usePostGoamlWatchRuns`.

Screen E1 (DESIGN-SYSTEM E1, route `/goaml-check`, STEPPER layout in the reduced shell) after phase 08:
- **Step 1, files:** a drop zone and file picker (XML or ZIP, the limits shown from the settings) or a paste box for
  one file; an optional "Office rejection message" box, enabled only with exactly one file; for M a profile
  picker that offers a `candidate` labelled "Not yet in force". "Check" sends the multipart request with a generated
  `Idempotency-Key`.
- **Step 2, progress:** polls `GET /goaml/check-runs/{id}` every 2 seconds until final; per file the stage reached,
  in words.
- **Step 3, result:** per file the outcome in words ("Would be rejected", "Accepted, estimated grade: Good",
  "Refused: not a safe file"), the estimated score and grade always with the word "estimated" and a note that the
  Office grades its own way; findings grouped by layer (structure, business rules, consistency, quality, Office
  message), each with severity word, the element path, line and column, the explanation and the fix in the user's
  language with the other language one tap away. No value from the file is shown (the API has none); the user opens
  their own file at the line given. A print stylesheet gives an A4 page per file with the profile version and the
  run time.
- **Earlier runs:** the caller's runs (all runs for M) with date, file count and outcomes; no file names in the list
  (the run detail shows them to the requester and M).
- **Profile page** (CFG, CHK, M, A2 read): versions with status, source and dates; for a version the patches, rules
  with severity and parameters, limits, emit flags and open questions; the diff from its base; the corpus result;
  "Propose change" (CFG) opens the phase 08 propose form with the promote or amend body; the watcher runs and "Check
  now" (M, CFG; hidden while the watcher is off, with the reason shown).
Until phase 08 the MLRO uses Swagger UI and `akr-ctl goaml`.

## 6. Verification

1. `make migrate`: 0023-0025 apply; a second run is a no-op; the phase 00 schema checks pass for the four new tables
   (classification and retention rows present). Negative checks: `DELETE FROM rpt.goaml_profiles` fails with
   `profile versions are never deleted`; changing `rules` on an `active` version fails; a second `active` version of
   `IQ` fails `uq_goaml_profiles_active`; any status change from `retired` or `rejected` fails; a second `running`
   watch run fails its index; a finding whose `path` holds a quote, `=` or a space fails the path CHECK; `akr_app` has
   no DELETE on any `rpt.goaml_` table.
2. `cargo check` with `cargo sqlx prepare`; `cargo clippy --all-targets -- -D warnings`; `cargo deny check` (libxml2
   linked only by `tools/xsdcheck`; a test crate that adds `libxml` elsewhere fails the check).
3. `akr-ctl packs seed` -> the `IQ` profile `5.0.2-2026-09` inserted `active`, a `goaml_corpus` job queued; a second
   run -> nothing added; `akr-ctl packs verify-data` -> exit 0 with 19 report codes, 62 indicator codes and `P-01`
   applied; `akr-ctl goaml corpus` -> exit 0: the three validated samples pass with no block finding and each of the
   63 cases gives exactly its expected block codes.
4. `cargo test`, including:
   - pre-screen: a DOCTYPE, the billion-laughs file, an external entity, depth 65, a processing instruction and a
     UTF-16 declaration are each refused with their `PRE-` code before `akr-xsdcheck` is started (the test asserts
     the validator mock was not called);
   - sandbox: a test XSD importing an `http://` schema fails closed with `XSD-99`; a validator stub that spins is
     killed by `RLIMIT_CPU` and gives `XSD-99` while the run continues with the next file; `akr-xsdcheck` output never
     holds a message string (schema test of its JSON lines);
   - marker test: three files whose every text node and attribute value is a unique marker string go through the full
     run; no marker appears in any database table, the worker and api logs, the audit details or the job payloads;
   - ZIP: a 10 KB entry inflating past ratio 100 -> `PKG-06`; an entry named `../x.xml` -> `PKG-04`; two XML entries
     -> refused by `xml_per_zip`; an EICAR attachment -> `PKG-05` `refused`;
   - each rule of section 3.3 with its positive and negative fixture; `P-01` (an ordinary Arabic entity name fails the
     published facet under the strict reading and passes the patched one; `COMPAT-01` flags digits with offsets);
   - scorer: each criterion and the band edges 84/85, 69/70, 49/50; a file with a block finding has no score;
   - loader and diff: a copy of the shipped XSD with one enumeration value removed, one `maxOccurs` changed and the
     entity pattern changed -> the diff lists exactly those three and `P-01` as orphaned;
   - watcher against a local fake server: unchanged list -> `unchanged`; a new schema -> `changed` with a `candidate`
     and a corpus job; a link to a host outside `goaml.watch_hosts` -> `host_not_allowed`; a 6 MiB schema -> refused;
     no proxy configured -> `proxy_unreachable`;
   - rejection parser: each shipped pattern; an unmatched text -> one `OFF-99`; the text is not stored.
5. Curl suite `tests/curl/phase07_goaml.sh` (dev users `a2@bank.test`, `a2b@bank.test` (a second A2, added to
   `fixtures/dev-users.yaml` by this phase), `m@bank.test`, `ds@bank.test`, `cfg@bank.test`, `chk@bank.test`; fixture
   files under `tests/fixtures/goaml/`):
   - As A2 `POST /api/v1/goaml/check` with the sample STR and `Idempotency-Key: gc-0001` -> 202 with `Location`;
     polling -> `completed`, the file `graded` with `grade_basis: "estimated"`; the same key and body -> the stored
     202 with `x-idempotent-replay: true`.
   - As A2 a case file with an apostrophe-and-digit first name and an unknown `transmode_code` -> `would_reject`, no
     grade, findings `XSD-03` and `XSD-04` with element path and line; `jq` finds no key holding the name or the code
     value.
   - As A2 a ZIP with 21 files -> 422 `too_many_files`; `rejection_text` with two files -> 422
     `rejection_needs_one_file`; A2 with a `candidate` `profile_id` -> 422 `profile_not_usable`.
   - As the second preparer (A2b) `GET /api/v1/goaml/check-runs/<A2's run>` -> 404 `E_NOT_FOUND`, byte-identical to an
     unknown id; A2b's run list does not include it; as M -> 200 with the file names, and `audit.access_event` holds
     the `view` row (`psql` as `akr_readonly`).
   - As DS `POST /api/v1/goaml/check` -> 403 `E_SCOPE`; as CFG `POST /api/v1/goaml/check` -> 403 `E_SCOPE`.
   - After the runs, `mc ls` (as the ops credential) of `akr-quarantine/transient/goaml/` is empty and each run has
     `transient_deleted = true`.
   - As CFG `PUT /api/v1/goaml/profiles/<IQ v1>` `{"op":"amend", ...}` lowering `BR-05` to `info` with `If-Match` ->
     202 `ChangeRequestView` on `cr_material`, a `candidate` version and a `goaml_corpus` job; the corpus result
     present; as CHK approve -> the candidate `active`, v1 `retired`, audit `goaml_profile.activated`; the same body
     disabling `BR-01` without a reason -> 422 `loosening_needs_basis`; as A2 the PUT -> 403 `E_SCOPE`.
   - As M `POST /api/v1/goaml/watch-runs` while `goaml.watch_enabled` is false -> 409 `watch_disabled`; as A2 `GET
     /api/v1/goaml/watch-runs` -> 200 (empty).
   - As CHK `GET /api/v1/goaml/profiles` -> 200; as CHK `GET /api/v1/goaml/watch-runs` -> 403 `E_SCOPE`.
6. Swagger UI lists 9 new operations (130 with phases 01 and 03 to 06), each with `x-akr-action`; `make generate-api`
   twice gives zero diff.
7. Audit: after the curl run `audit_log` holds `goaml_profile.seeded`, `goaml_profile.corpus_ran`,
   `goaml_check.requested`, `goaml_check.completed`, `goaml_check.transient_deleted`,
   `goaml_profile.candidate_created`, `change_request.*`, `goaml_profile.activated`, `authz.denied` (the DS, CFG and
   A2 attempts), and the masked `deny` access event for A2b; no audit detail holds a file name; `akr-ctl audit
   verify` passes.
8. No CBS connection is made by this phase: the conformance suite is unaffected, and the phase 05 statement journal
   shows no statement during the curl run.
9. Phase 00 to 06 tests still pass.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 3a, from tools/agent/gap1-run3a.md; lead-verified (tools/agent/gap1-run3a-verdict.md).

### 7.1 CheckService::validate_built (phase 21 section 4.5 item 6, DR-171)
`CheckService::validate_built(bytes: &[u8], profile: &GoamlProfile, layers: &[Layer]) -> Result<CheckOutcome,
CheckError>` runs the per-file checks of the `goaml_check` job (section 3.2, from the package step on: package when
the bytes are a package, pre-screen, XSD, the rules of section 3.3 and the score of section 3.4) on XML that the
caller built and holds in memory. It writes no run row, stages nothing and stores nothing; the caller stores the
outcome, which holds the `FileResult` of the file (section 3.2 step 11), its findings as `NewFinding` rows (section 2) and its `CheckSummary` (section 3.2 step 12). The upload path does not change. Phase 21 adds its own layer L0 before calling it.

### 7.2 Rule codes added by phase 21 (DR-368)
The catalogue of section 3.3 gains these rows (columns: code, condition, severity, parameters). The codes arrive in a
release, because the catalogue is closed in code; a profile enables them as it enables the other rules.

| Code | Condition | Severity | Parameters |
|-|-|-|-|
| NAME-01 | an apostrophe, dot, comma, parenthesis or slash removed from a person name (raised only by the phase 21 builder, never on an uploaded file) | warn | `removed_count` |
| DATE-01 | a birth date known only to the year (month and day UNKNOWN) | warn | |
| ID-01 | a placeholder national identifier (the national ID is missing) | warn | |
| PKG-07 | an attachment whose evidence is still being checked (`pending`) | block | |
| CTR-01 | a CTR transaction without cash on one side and an account on the other | block | |
| CTR-02 | a CTR transaction whose two countries are not both in `domestic` | block | `domestic` [IQ] |
| CTR-03 | a CTR transaction whose amount is above `max_usd` in USD, or is not a whole multiple of the cash unit (`round_unit_iqd` for IQD, 1 for every other currency) | warn | `max_usd` 1000000, `round_unit_iqd` 250 (integers) |
| CONS-05 | a transaction whose account is not among the accounts of the report's subjects (RP-30) | warn | |

`CONS-04` is already in the catalogue (info).

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.3 A corpus run that dies, and how the corpus grows (V-07-03-a, V-07-03-b, V-07-07-a, MEDIUM; DR-413)
When `goaml_corpus` goes dead (phase 00, `job.dead`), `corpus_result` stays NULL. A promote is then refused with 422 `corpus_not_run`, and an amendment request cannot be decided because its `validation_run` evidence is missing: the maker withdraws it (the candidate becomes `rejected`, section 3.6 Closed) or an operator retries the job (phase 03 job administration).
The corpus is release data (section 3.6): the files under `corpus/` and `cases.json`. A profile amendment adds rejection patterns, rule settings and limits; it adds no case file, and no table or route stores one. A rejection the bank meets becomes a corpus case when the bank gives the anonymised file, the codes it must produce and the Office message to the vendor, and the case ships in a later release, as the rule codes of section 7.2 do; `packs seed` then queues `goaml_corpus`. The header's RP-06 and RP-83 lines, the last paragraph of section 3.3 ("plus the cases a profile amendment adds", "Phase 21 adds the bank's own anonymised rejection cases") and section 3.7 ("and, from phase 21, a corpus case") read this way. Phase 21 adds no corpus.

### 7.4 Rule parameter types (V-07-05-a, LOW; DR-413)
The parameter schema of each rule (section 3.1 step 4) takes these JSON types for the Parameters column of section 3.3. `report_codes` (BR-03: STR, SAR, TFR), `placeholders` (BR-14: the five words quoted in that row), `currencies` (BR-18: IQD, USD) and `domestic` (BR-13, CTR-02: IQ) are arrays of strings. `days` (BR-08: 31; CONS-02: 7), `min_chars` (CONS-03: 400) and `threshold_usd` (BR-13: 10000) are integers. `tolerance` (CONS-01: 1) is a number. The shipped `profile.json` holds these values.

### 7.5 BR-13 beside CTR-01 and CTR-02 (V-07-12-b, MEDIUM; DR-413)
CTR-01 and CTR-02 block what DR-368 says is not a CTR. BR-13 keeps its warn for the amount: on a transaction for which CTR-01 or CTR-02 raised a finding, BR-13 raises none, so one fault gives one finding. A profile without the two CTR rules (before phase 21's release) keeps BR-13 as section 3.3 states it.

## What This Phase Does NOT Touch

- No report builder, report packages, filing workflow, deadline clocks, normaliser writing XML, L0 snapshot checks,
  FIU-answers corpus or monthly calibration sample (phase 21): this phase checks files the bank already produced.
- No CTR production or threshold job (phase 21); `BR-13` only checks the shape of an uploaded CTR.
- No code maps from CBS codes to goAML lookups (phases 10 and 21); lookups are read from the XSD only.
- No notifications (phase 22): a new candidate shows on the screens, the cockpit and an alert rule.
- No automation of the Office's portal, ever: nothing is uploaded to or fetched from goAML Web; the watcher reads the
  Office's public schema page only.
- No CBS access, no bank data other than the uploaded files (deleted after the run), and no change to the phase 00-06
  tables.
