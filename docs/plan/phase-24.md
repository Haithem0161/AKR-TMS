# Phase 24: Governance, Registers, Command Centre & Packs

**Goal:** The bank's governance functions can read and prove what the system did: IA and SEC search the audit trail and verify its chain (every search audited), anyone with the right role reads the registers of decisions, exclusions, findings, warnings and fairness outcomes, the MLRO and the CISO see the command centre and the clocks board with suppressed small numbers, the assessor works through a time-boxed read-only grant (the D2 matrix, seeded reproducible samples and an end-to-end trace), and every pack (board, case, report, assessor, annual certification) is an immutable, hash-verifiable version, with no case content in any pack a non-member can open.

**Dependencies:** Phase 00 (jobs, settings, cron schedules, timers, the outbox (no topic is added here), `akr-api` AppState and AppRole policies), Phase 01 (`StepUpService` with the closed classes `export`, `approval`, `record_open`; `admin.users`, `admin.role_grants` with `valid_to` (the time box of the assessor grant)), Phase 02 (the audit writer, `audit.audit_log`, `audit.access_event`, `audit.audit_anchors`, the access-event stream, `Verifier::run(scope)` and `VerifyReport`, the audit kind catalogue incl. the reserved `audit.searched`, `EvidenceStore`, `admin.evidence_objects`, `PackService::publish` and `admin.packs`), Phase 03 (`AccessScope`, the masking layer and `MASKED_KINDS`, `admin.purpose_codes` (`internal_audit`, `supervisory_inspection`, `other`), the Cedar range 60 and C0, `AccessMatrixBuilder`), Phase 04 (`admin.inventory_items`, `admin.change_requests`, `admin.instance_events` with the kind `post_review_recorded`, `ChangeTargetRegistry`, `Holders`, `admin.tasks`), Phase 06 (the `POST /packs` router, the `pack_build` job, the kinds `integration_profile` and `vendor_request` and the `Pack` Cedar entity, `C0_packs.cedar`, the `POST /packs/verify` reservation), Phase 09 (`ClockRegistry`, `PolicyPackService::scan` with scope `reason_text`, `admin.authority_matrix`, `BusinessClock`), Phase 10 (`canon.party` with `sensitivity_tier` and `recorded_to`, `canon.txn` with lineage, `admin.drift_events`), Phase 11 (`admin.quarantine_items`, the operations incident sources), Phase 12 (`admin.exclusions`, `admin.dq_issues`, `admin.coverage_certificates`, `report.small_number_threshold` is phase 21's), Phase 16 (scenario versions, parameter sets, runs, replays and seeded tests for the validation pack; the scenario review clock), Phase 17 (`alert.alerts`, `casework.cases` with `tier`, `casework.case_alerts`, `casework.effective_tier`, `casework.party_tiers`, the visibility predicate `services::visibility::build` built from the Cedar permits), Phase 18 (`casework.decisions`, `casework.case_acl`, the case read model, the sealed-case rule), Phase 19 (`casework.alarms`, `casework.rfis`, `casework.internal_reports`, `casework.authority_requests`, `casework.qa_samples`, `casework.qa_reviews`, `casework.qa_findings`; the internal-report register stays unreadable by `akr_readonly`), Phase 20 (`casework.freezes`, `casework.refrain_instructions`, `casework.ops_tasks`), Phase 21 (`rpt.reports`, `rpt.report_versions`, `rpt.filings`, `rpt.report_tasks`, `rpt.periodic_reports` with kinds `board` and `liaison`, `report.small_number_threshold`, the R3 periodic packs), Phase 22 (`casework.liaison_monthly_reports` with `due_on` and `status`), Phase 23 (its file `61_retention.cedar` is in the audit range 60 beside this phase's `60_audit.cedar`; phase 23 has no retention rows for this phase's tables, which 0079 adds)

**Macro-phase:** P1
**Complexity:** XL (39 routes: the eight of sections 7.12 and 7.13, and the 31 of section 2, which are 19 rows of API.md 3.3, 3.4, 3.15 and 3.16 owned or extended here, of which 3 are phase 06 routes whose kinds this phase adds (`POST /packs`, `GET /packs/{id}`, `GET /packs/{id}/download`); 12 rows added beyond API.md; 7 tables in 4 migrations (`0076` to `0079`); 1 altered CHECK (`admin.packs` kinds); 2 new `audit` functions (SECURITY DEFINER tier resolver, no new table grant to `akr_readonly`); 4 new job kinds and 1 extended (`pack_build`); no timer kind; no outbox topic)
**Binaries touched:** akr-api (routers `audit_viewer`, `access_history`, `registers`, `fairness`, `validation`, `command_centre`, `clocks`, `packs` (extended), `assessor`, `training`; `AuditViewerService`, `AccessHistoryService`, `GovernanceRegisterService`, `FairnessService`, `ValidationPackService`, `CommandCentreService`, `ClockBoardService`, `AssessorService`, `TrainingRegisterService` in `AppState`; a new `readonly_pool`) | akr-worker (jobs `audit_verify_run`, `register_post_review_sync`, `review_calendar_sweep`, `fairness_monthly`; extended `pack_build` for the five new kinds; `Verifier` of phase 02 called by `audit_verify_run`) | akr-ingestd and akr-assess unchanged

**Requirements carried:**
- Implemented here: SP-42 (the viewer: every search writes `audit.searched`, with the filters, the purpose code and the result count; section 3.1), SP-44 (partly: the time-boxed read-only assessor grant of M-62; the inspector account of phase 25 is not here), M-62 (assessor workspace: D2 map, seeded random sample, end-to-end trace, time-boxed read-only grant, pack), M-51 (training register; the AI golden set is P2), UX-49 (board pack from system facts, aggregates only), UX-50 (D2 evidence matrix, assessor workspace, chain verifier, pack download under a named time-boxed read-only account), GV-09 (validation pack record with the seven parts), GV-15 (per-bank delta validation part for rules), GV-22 (review calendar of rules, exclusions, change requests, readiness and scenarios, section 3.4), GV-34 (findings register with severity, owner, due date, repeat flag and status, fed by phase 04 post-change reviews and by hand), GV-35 (annual certification pack built from the phase 12 coverage certificates and this phase's validation and findings records), GV-36 (protected-attribute register and allowed-use matrix), GV-38 (monthly outcome metrics by customer group, suppression, review trigger)
- Partly implemented (the rest is in Open doubts with its target): S-46 (review calendar of section 3.4; the recalibration clock of phase 16 is Open doubts 11), S-53 (the fairness dimension only; productivity, uniqueness and the other eleven metrics are Open doubts 11), S-02 (the assessor pack links the phase 06 CBI gap report documents), S-56 (read-only assessor role with exports and a record of what was shown; the finding-to-task remediation generator is Open doubts 11), RP-63 (the `report` pack kind of section 3.11 holds the as-filed document, versions, reasons and approvals; the alert list and scoring snapshot are Open doubts 11), RP-64 (the trace of section 3.9 runs from a CBS row to a report; the figure-to-snapshot link is phase 21's Open doubts 19), M-63 (pack kinds, SHA-256 manifest and audited download; redaction, the MLRO certification with step-up and the requester watermark are Open doubts 11), GV-23 (the board pack carries the KPI set of its period; the quarterly committee cycle is Open doubts 10)
- Not carried here: GV-06 (the Model Oversight Committee pack and its calendar; Open doubts 10), GV-08 (annual self-assessment and control crosswalk; Open doubts 10), GV-10 (independent re-implementation; phase 16 S-48, Open doubts 11), GV-13 (behavioural-testing sample plan; Open doubts 10), GV-18 (rule-side monitoring thresholds; phase 17 and 19 alarms only, Open doubts 10), GV-19 (incident log with forced decision; the incident sources are read by the command centre only, Open doubts 10), SP-43 (SIEM export; phase 25), M-59 and M-61 (phases 17 and 18), S-41, S-43, S-48, S-54, S-55, S-57, S-60 and the card export of S-05 (Open doubts 11), UX-60 (mockup 05; phase 16 and phase 04)
- Gap pass 1 (coverage), not carried: S-35 (employee-risk view, P2, DR-394); SP-02 (control crosswalk documents, vendor compliance work, DR-396); L-64 (correspondent assertion cards, P2 correspondent pack); VB-51 (automation claims register; no automation feature in this release, DR-403).
- Gap pass 1 (coverage), implemented here: SP-34 (the monthly tipping-off control report); UX-47 (dashboards in attention, period, detail order with definitions and data-as-of), UX-56 (the command centre of mockup 01); B-11 (the board pack builder with counts through each source predicate); VA-54 and VO-03 (the parallel run beside the bank's current system with the disagreement log, section 7+); VD-18 (the D2 clause map with the read-only assessor grant and the assessor pack).
- Gap pass 1 (coverage), partly implemented: SP-80 (the assessor pack), SP-99 (the protected-attribute register, the proxy scan and monthly outcome metrics), K-38 (the assessor workspace and packs; the rating distribution is phase 15); L-24 (the review calendar with amber and red), L-30 (validator separation and replay results in validation packs), L-56 (the board pack from system facts), L-57 (registers, decisions and exclusions as read views), L-58 (the findings register with owner, status and repeat flag), L-59 (the warning register kind with owner and status), L-60 (pack kinds with manifests and the certification pack); T-05 (the productive-alert and top-scenario tiles), T-59 (tiles with definition, formula and role visibility), T-60 (period counts of alerts created and dispositioned, productive alerts and reports); UX-31 (CSV and XLSX register exports with injection guards, section 7+), UX-48 (the command-centre tiles), W-01 (productive alerts and FIU feedback), W-20 (the audit viewer and control report); F-19 (the command centre, the training register and the internal-audit role), F-21 (the review calendar with the scenario card and coverage matrix of phase 16), F-23 (the assessor pack and the productive-alert metric); VA-12 (the inventory record of rules with owner and validation), VA-23 (the assessor pack from system facts); VB-18 (the import of the current system's alerts for the parallel run), VB-21 (the command-centre tiles), VD-22 (the parallel run beside the current system); VC-38 (clock ageing on the command centre), VC-82 (case and correspondent packs with hashes).

---

## 1. SQLx Migration Changes

Four migrations, `0076_admin_registers_validation`, `0077_admin_fairness_training_assessor`, `0078_admin_verify_viewer` and `0079_admin_guards_grants`, numbered after phase 22's `0075` (the lead's range 0076 to 0079; no other phase uses these numbers). The schema is `admin` for every new table (registers, validation, fairness, training and assessor data are governance records, not case records); `audit` is unchanged apart from one read function. Seven tables are created: `admin.registers`, `admin.validation_packs`, `admin.protected_attributes`, `admin.fairness_metrics`, `admin.training_records`, `admin.assessor_samples` and `admin.verify_runs`. One CHECK constraint changes: the kinds of `admin.packs` (phase 02, widened by phase 06, gains `certification`).

**Reconciliation with DATA-MODEL section 15 (changed in the same change, DR-218):**
- `registers` keeps its name and schema. Its DATA-MODEL columns `id`, `kind` (`finding`, `warning`), `body`, `owner_id`, `status` and `version` are kept. `recurrence` becomes `repeat_flag` and `repeat_of` (GV-34). Added: `source`, `source_key` (the dedupe key of a system row), `title`, `severity` (findings only), `due_on`, `closed_by`, `closed_at`, `closure_code`, `created_by`, `created_at`, `updated_at`. DATA-MODEL's R9 class is kept.
- `validation_packs`: DATA-MODEL's `object_key` becomes `evidence_id` (the canonical JSON is an evidence object of phase 02, so its key and hash are the vault's). Added: `inventory_version`, `period_from`, `period_to`, `owner_id` (copied from the item at issue, for the GV-32 check), `sha256` and `version`. `parts`, `verdict`, `validator_id` and `issued_at` are kept.
- `fairness_metrics`: DATA-MODEL's `period`, `metric`, `group_ref`, `value` and `computed_at` are kept. Added: `group_kind`, `customers`, `events`, `reference_value`, `ratio`, `suppressed`, `status` and `register_id`.
- `models`, `model_versions`, `model_scores` and `assistant_drafts` are P2 (phases 27 and 28). They are not created here.
- New, not in DATA-MODEL: `admin.protected_attributes` (GV-36), `admin.training_records` (M-51), `admin.assessor_samples` (M-62) and `admin.verify_runs` (section 3.3).
- Decisions and exclusions are read views over `casework.decisions` (phase 18) and `admin.exclusions` (phase 12). No table is copied (section 3.4).

### 1.1 `0076_admin_registers_validation.sql`

```sql
-- migration 0076_admin_registers_validation.sql
-- Phase 24. Governance registers (findings and warnings, GV-34), the protected-attribute register (GV-36) and the
-- validation pack record (GV-09, GV-15). Ids are UUIDv7 from the application. Retention rows and grants: 0079.

-- Findings and warnings. A finding is opened by hand (M, CH, MRO) or by a job (post-change concern, fairness review,
-- review calendar); a warning is opened only by a job. Decisions and exclusions are NOT rows here (read views).
CREATE TABLE IF NOT EXISTS admin.registers (
    id                  UUID PRIMARY KEY,
    kind                TEXT NOT NULL CHECK (kind IN ('finding', 'warning')),
    source              TEXT NOT NULL CHECK (source IN ('manual', 'post_review', 'fairness', 'review_calendar', 'scan')),
    source_key          TEXT CHECK (source_key IS NULL OR length(source_key) BETWEEN 1 AND 200),   -- dedupe of a system row
    title               JSONB NOT NULL CHECK (jsonb_typeof(title) = 'object' AND title ? 'ar' AND title ? 'en'),
    body                JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(body) = 'object'),   -- ids, codes, counts; free text passes PolicyPackService::scan
    severity            TEXT CHECK (severity IN ('low', 'medium', 'high', 'critical')),
    owner_id            UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    due_on              DATE,
    repeat_of           UUID REFERENCES admin.registers(id) ON DELETE RESTRICT,   -- the earlier finding with the same subject
    repeat_flag         BOOLEAN NOT NULL DEFAULT FALSE,
    status              TEXT NOT NULL DEFAULT 'open'
                        CHECK (status IN ('open', 'in_progress', 'accepted_risk', 'acknowledged', 'closed')),
    closure_code        TEXT CHECK (closure_code IS NULL OR closure_code ~ '^[A-Z][A-Z0-9_]{2,39}$'),
    closed_by           UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    closed_at           TIMESTAMPTZ,
    created_by          UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- NULL for a job row
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    CHECK ((kind = 'finding') = (severity IS NOT NULL)),
    CHECK (kind <> 'finding' OR (owner_id IS NOT NULL AND due_on IS NOT NULL)),
    CHECK (kind <> 'finding' OR status IN ('open', 'in_progress', 'accepted_risk', 'closed')),
    CHECK (kind <> 'warning' OR status IN ('open', 'acknowledged', 'closed')),
    CHECK ((status = 'closed') = (closed_at IS NOT NULL)),
    CHECK ((closed_at IS NULL) = (closed_by IS NULL)),
    CHECK ((status = 'closed') = (closure_code IS NOT NULL)),
    CHECK (source = 'manual' OR created_by IS NULL),
    CHECK (source = 'manual' OR source_key IS NOT NULL),
    CHECK (repeat_of IS NULL OR repeat_of <> id),
    CHECK (repeat_flag = (repeat_of IS NOT NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_registers_source_key ON admin.registers (source_key) WHERE source_key IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_registers_open     ON admin.registers (kind, status, due_on) WHERE status <> 'closed';
CREATE INDEX IF NOT EXISTS idx_registers_owner    ON admin.registers (owner_id) WHERE owner_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_registers_repeat   ON admin.registers (repeat_of) WHERE repeat_of IS NOT NULL;

-- Protected-attribute register and allowed-use matrix (GV-36, GV-37). Seeded below; changed by change request in the
-- governance range (section 3.5). The sole-trigger flag is a CHECK: no attribute can be the only reason for an alert,
-- a hold or a score.
CREATE TABLE IF NOT EXISTS admin.protected_attributes (
    code                  TEXT PRIMARY KEY CHECK (code ~ '^[a-z][a-z_]{2,40}$'),
    label                 JSONB NOT NULL CHECK (jsonb_typeof(label) = 'object' AND label ? 'ar' AND label ? 'en'),
    attribute_class       TEXT NOT NULL CHECK (attribute_class IN ('nationality', 'residence', 'geography', 'name_derived',
                                                                   'sect_tribe_ethnicity')),
    source_column         TEXT NOT NULL CHECK (length(source_column) BETWEEN 1 AND 80),   -- canon column the attribute is read from
    allowed_uses          TEXT[] NOT NULL DEFAULT '{}' CHECK (allowed_uses <@ ARRAY['fairness_metric', 'segmentation', 'reporting']::text[]),
    combined_trigger_ok   BOOLEAN NOT NULL DEFAULT FALSE,   -- may sit beside another condition in a scenario
    sole_trigger_ok       BOOLEAN NOT NULL DEFAULT FALSE CHECK (sole_trigger_ok = FALSE),   -- GV-36: never the sole trigger
    prohibited_as_input   BOOLEAN NOT NULL DEFAULT FALSE,   -- GV-37: never a model or score input
    owner_id              UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- receives the findings of this attribute
    basis                 JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(basis) = 'object'),
    status                TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'retired')),
    updated_by            UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    version               INT NOT NULL DEFAULT 1
);

-- The validation pack record (GV-09 seven parts, GV-15 per-bank delta). One row per issued version of an inventory item.
-- Immutable: a rebuild is the next version (section 3.6). The validator is never the owner (GV-32, phase 04).
CREATE TABLE IF NOT EXISTS admin.validation_packs (
    id                    UUID PRIMARY KEY,
    inventory_item_id     UUID NOT NULL REFERENCES admin.inventory_items(id) ON DELETE RESTRICT,
    inventory_version     INT NOT NULL CHECK (inventory_version >= 1),   -- admin.inventory_items.version the pack was built on
    version               INT NOT NULL CHECK (version >= 1),              -- per inventory item
    period_from           DATE NOT NULL,
    period_to             DATE NOT NULL,
    parts                 JSONB NOT NULL CHECK (jsonb_typeof(parts) = 'object'),   -- the seven parts of section 3.6
    verdict               TEXT NOT NULL CHECK (verdict IN ('pass', 'pass_with_findings', 'fail')),
    owner_id              UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,   -- copied from the item at issue
    validator_id          UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    evidence_id           UUID NOT NULL REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,   -- canonical JSON (kind snapshot)
    sha256                BYTEA NOT NULL CHECK (octet_length(sha256) = 32),   -- equals the evidence object's sha256
    issued_at             TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (inventory_item_id, version),
    CHECK (period_to >= period_from),
    CHECK (validator_id <> owner_id)
);
CREATE INDEX IF NOT EXISTS idx_validation_packs_item      ON admin.validation_packs (inventory_item_id, version DESC);
CREATE INDEX IF NOT EXISTS idx_validation_packs_validator ON admin.validation_packs (validator_id);
CREATE INDEX IF NOT EXISTS idx_validation_packs_evidence  ON admin.validation_packs (evidence_id);
```

### 1.2 `0077_admin_fairness_training_assessor.sql`

```sql
-- migration 0077_admin_fairness_training_assessor.sql
-- Phase 24. Monthly outcome fairness metrics (GV-38), the training register (M-51) and the recorded assessor samples
-- (M-62). Ids are UUIDv7 from the application.

-- Outcome rates per customer group for one month (GV-38). Rates are events per 1,000 active customers of the group.
-- A cell below `report.small_number_threshold` (phase 21, never below 3) on either count is suppressed: value, ratio and
-- counts are NULL and the row says so. The metric is a measurement: nationality, residence and geography are never a
-- trigger (section 3.5).
CREATE TABLE IF NOT EXISTS admin.fairness_metrics (
    id                UUID PRIMARY KEY,
    period            TEXT NOT NULL CHECK (period ~ '^[0-9]{4}-(0[1-9]|1[0-2])$'),
    metric            TEXT NOT NULL CHECK (metric IN ('alert_rate', 'case_rate', 'str_rate', 'hold_rate')),
    group_kind        TEXT NOT NULL CHECK (group_kind IN ('nationality', 'residence_country', 'governorate', 'segment')),
    group_ref         TEXT NOT NULL CHECK (length(group_ref) BETWEEN 1 AND 40),
    customers         INT CHECK (customers IS NULL OR customers >= 0),
    events            INT CHECK (events IS NULL OR events >= 0),
    value             NUMERIC(12,6) CHECK (value IS NULL OR value >= 0),
    reference_value   NUMERIC(12,6) CHECK (reference_value IS NULL OR reference_value >= 0),
    ratio             NUMERIC(10,4) CHECK (ratio IS NULL OR ratio >= 0),
    suppressed        BOOLEAN NOT NULL,
    status            TEXT NOT NULL DEFAULT 'ok'
                      CHECK (status IN ('ok', 'review', 'justified', 'changed', 'suppressed')),
    register_id       UUID REFERENCES admin.registers(id) ON DELETE RESTRICT,   -- the finding or warning a review opened
    computed_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (period, metric, group_kind, group_ref),
    CHECK (suppressed = (value IS NULL)),
    CHECK ((status = 'suppressed') = suppressed),
    CHECK (status NOT IN ('review', 'justified', 'changed') OR register_id IS NOT NULL),
    CHECK (suppressed OR (customers IS NOT NULL AND events IS NOT NULL AND reference_value IS NOT NULL))
);
CREATE INDEX IF NOT EXISTS idx_fairness_period ON admin.fairness_metrics (period, metric);
CREATE INDEX IF NOT EXISTS idx_fairness_review ON admin.fairness_metrics (status) WHERE status = 'review';

-- The training register (G9, M-51). One row per course completion of a user. A correction is a new row and a void of
-- the old one; the void columns are the only ones ever updated. Nobody records their own training (section 3.10).
CREATE TABLE IF NOT EXISTS admin.training_records (
    id                  UUID PRIMARY KEY,
    user_id             UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    course_code         TEXT NOT NULL CHECK (course_code ~ '^[A-Z][A-Z0-9_]{2,30}$'),   -- the bank's course list (Open doubts 7)
    completed_on        DATE NOT NULL,
    expires_on          DATE,
    score_pct           SMALLINT CHECK (score_pct IS NULL OR score_pct BETWEEN 0 AND 100),
    evidence_id         UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,   -- the certificate file, when one exists
    evidence_ref        TEXT CHECK (evidence_ref IS NULL OR length(evidence_ref) BETWEEN 1 AND 120),   -- certificate number
    recorded_by         UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    recorded_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    voided_by           UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    voided_at           TIMESTAMPTZ,
    void_reason_code    TEXT CHECK (void_reason_code IN ('entered_in_error', 'superseded')),
    CHECK (user_id <> recorded_by),
    CHECK (evidence_id IS NOT NULL OR evidence_ref IS NOT NULL),
    CHECK (expires_on IS NULL OR expires_on > completed_on),
    CHECK ((voided_at IS NULL) = (voided_by IS NULL)),
    CHECK ((voided_at IS NULL) = (void_reason_code IS NULL))
);
CREATE INDEX IF NOT EXISTS idx_training_user   ON admin.training_records (user_id, course_code, completed_on DESC);
CREATE INDEX IF NOT EXISTS idx_training_expiry ON admin.training_records (expires_on) WHERE voided_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_training_by     ON admin.training_records (recorded_by);

-- Recorded assessor samples (M-62). The draw is seeded and reproducible: the seed (256 bits from the OS random source)
-- and the frame hash are stored, so any person with the frame can re-derive the same items (section 3.9). Immutable.
CREATE TABLE IF NOT EXISTS admin.assessor_samples (
    id              UUID PRIMARY KEY,
    drawn_by        UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    grant_id        UUID REFERENCES admin.role_grants(id) ON DELETE RESTRICT,   -- the time-boxed assessor grant in force
    frame           TEXT NOT NULL CHECK (frame IN ('alert_register', 'report_register')),
    frame_from      DATE NOT NULL,
    frame_to        DATE NOT NULL,
    frame_size      INT NOT NULL CHECK (frame_size >= 0),
    sample_size     INT NOT NULL CHECK (sample_size BETWEEN 1 AND 200),
    seed            BYTEA NOT NULL CHECK (octet_length(seed) = 32),
    algorithm       TEXT NOT NULL CHECK (algorithm = 'sha256_rank_v1'),   -- rank = SHA-256(seed || id bytes), ascending
    frame_hash      BYTEA NOT NULL CHECK (octet_length(frame_hash) = 32),  -- SHA-256 over the sorted frame ids
    items           JSONB NOT NULL CHECK (jsonb_typeof(items) = 'array'), -- [{"id": uuid, "rank": n}] in rank order
    drawn_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (frame_to >= frame_from),
    CHECK (sample_size <= frame_size),
    CHECK (jsonb_array_length(items) = sample_size)
);
CREATE INDEX IF NOT EXISTS idx_assessor_samples_by    ON admin.assessor_samples (drawn_by, drawn_at DESC);
CREATE INDEX IF NOT EXISTS idx_assessor_samples_grant ON admin.assessor_samples (grant_id) WHERE grant_id IS NOT NULL;
```

### 1.3 `0078_admin_verify_viewer.sql`

```sql
-- migration 0078_admin_verify_viewer.sql
-- Phase 24. Verify runs (the phase 02 Verifier as a job), the pack kind `certification` (GV-35), the tier function the
-- audit viewer uses (SECURITY DEFINER; no new table grant to akr_readonly).

-- Verify runs (POST /audit/verify-runs). The result keeps counts and finding kinds only: never an entry id, a partition
-- of a suspicion-tier row or a detail value (section 3.3). Retained for ever (R10): verification is audit evidence.
CREATE TABLE IF NOT EXISTS admin.verify_runs (
    id              UUID PRIMARY KEY,
    requested_by    UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    scope           JSONB NOT NULL CHECK (jsonb_typeof(scope) = 'object'),   -- {"streams": [...], "from": "YYYY-MM", "to": "YYYY-MM", "all": bool, "worm": bool}
    status          TEXT NOT NULL DEFAULT 'queued' CHECK (status IN ('queued', 'running', 'pass', 'findings', 'failed')),
    result          JSONB CHECK (result IS NULL OR jsonb_typeof(result) = 'object'),   -- VerifyReport summary (section 3.3)
    error_code      TEXT CHECK (error_code IS NULL OR error_code = 'E_INTERNAL'),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    started_at      TIMESTAMPTZ,
    finished_at     TIMESTAMPTZ,
    CHECK ((status IN ('pass', 'findings', 'failed')) = (finished_at IS NOT NULL)),
    CHECK ((status = 'failed') = (error_code IS NOT NULL)),
    CHECK (status = 'queued' OR started_at IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_verify_runs_created ON admin.verify_runs (created_at DESC, id DESC);
CREATE INDEX IF NOT EXISTS idx_verify_runs_status  ON admin.verify_runs (status) WHERE status IN ('queued', 'running');

-- Pack kind `certification` (GV-35, the annual certification pack). The phase 02 CHECK, widened by phase 06, is replaced
-- by the same list plus `certification`. No other kind is added (correspondent and validator stay P2).
ALTER TABLE admin.packs DROP CONSTRAINT IF EXISTS packs_kind_check;
ALTER TABLE admin.packs ADD CONSTRAINT packs_kind_check
    CHECK (kind IN ('board', 'case', 'report', 'assessor', 'correspondent', 'validator', 'cbi_gap',
                    'integration_profile', 'vendor_request', 'certification'));

-- The tier of an audit row's resource, read at query time (section 3.1). It mirrors phase 17's tier rule 2 (phase-17
-- section 3.10): the party tier (`casework.effective_tier`), the case family tier, the case tier, and `suspicion` for an
-- alert of a restricted scenario (`scn.scenarios.visibility`, phase 06). An alert takes the highest tier of every case
-- it is linked to. Unknown resource types answer 'suspicion': the viewer fails closed. SECURITY DEFINER owned by
-- akr_migrate: akr_readonly gains no read of casework, alert, scn or canon (phase 17 grants it nothing there) and learns
-- only the tier word of the one resource it asks about.
CREATE OR REPLACE FUNCTION audit.tier_rank(p_tier TEXT) RETURNS INT
LANGUAGE sql IMMUTABLE SET search_path = pg_catalog AS $$
    SELECT CASE p_tier WHEN 'suspicion' THEN 2 WHEN 'restricted' THEN 1 WHEN 'normal' THEN 0 ELSE 2 END
$$;
CREATE OR REPLACE FUNCTION audit.resource_tier(p_type TEXT, p_id UUID, p_case UUID) RETURNS TEXT
LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog AS $$
    WITH case_rank AS (
        SELECT c.id, GREATEST(audit.tier_rank(c.tier), audit.tier_rank(f.tier),
                              audit.tier_rank(casework.effective_tier(c.party_id))) AS r
          FROM casework.cases c JOIN casework.case_families f ON f.code = c.family
    ),
    r AS (
        SELECT CASE
            WHEN p_case IS NOT NULL OR p_type = 'case' THEN
                COALESCE((SELECT cr.r FROM case_rank cr WHERE cr.id = COALESCE(p_case, p_id)), 2)
            WHEN p_type = 'party' THEN audit.tier_rank(casework.effective_tier(p_id))
            WHEN p_type = 'customer' THEN
                COALESCE((SELECT max(audit.tier_rank(casework.effective_tier(cu.party_id)))
                            FROM canon.customer cu WHERE cu.id = p_id), 2)
            WHEN p_type = 'account' THEN
                COALESCE((SELECT max(audit.tier_rank(casework.effective_tier(apr.party_id)))
                            FROM canon.account_party_role apr WHERE apr.account_id = p_id), 2)
            WHEN p_type = 'alert' THEN
                COALESCE((SELECT GREATEST(audit.tier_rank(casework.effective_tier(a.party_id)),
                                          audit.tier_rank(f.tier),
                                          CASE WHEN sc.visibility = 'restricted' THEN 2 ELSE 0 END,
                                          COALESCE((SELECT max(cr.r) FROM casework.case_alerts ca
                                                      JOIN case_rank cr ON cr.id = ca.case_id
                                                     WHERE ca.alert_id = a.id), 0))
                            FROM alert.alerts a
                            JOIN casework.case_families f ON f.code = a.case_family
                            LEFT JOIN scn.scenarios sc ON sc.id = a.scenario_id
                           WHERE a.id = p_id), 2)
            WHEN p_type = 'pack' THEN
                COALESCE((SELECT CASE WHEN p.kind IN ('board', 'certification', 'assessor', 'cbi_gap', 'vendor_request',
                                                      'integration_profile') THEN 0 ELSE 2 END
                            FROM admin.packs p WHERE p.id = p_id), 2)
            WHEN p_type IN ('audit_log', 'access_log', 'anchor', 'setting', 'role_grant', 'user', 'branch',
                            'policy_bundle', 'change_request', 'register', 'validation_pack', 'training_record',
                            'assessor_sample', 'verify_run', 'fairness_metric', 'audit_stream', 'audit_partition',
                            'signing_key', 'database', 'antivirus', 'source', 'batch', 'inventory_item') THEN 0
            ELSE 2
        END AS rank
    )
    SELECT CASE rank WHEN 0 THEN 'normal' WHEN 1 THEN 'restricted' ELSE 'suspicion' END FROM r
$$;
ALTER FUNCTION audit.tier_rank(TEXT) OWNER TO akr_migrate;
ALTER FUNCTION audit.resource_tier(TEXT, UUID, UUID) OWNER TO akr_migrate;
REVOKE ALL ON FUNCTION audit.tier_rank(TEXT) FROM PUBLIC;
REVOKE ALL ON FUNCTION audit.resource_tier(TEXT, UUID, UUID) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION audit.tier_rank(TEXT) TO akr_readonly;
GRANT EXECUTE ON FUNCTION audit.resource_tier(TEXT, UUID, UUID) TO akr_readonly;
-- No table or column grant to akr_readonly on casework, alert, scn or canon (lead review): the function reads them as
-- its owner. The alert's own case_ref lookup and the access-history case resolution of section 3.2 call it the same way.
```

Note: `casework.effective_tier` is phase 17's function; `audit.resource_tier` calls it as its owner, so phase 17's grants are unchanged (lead review: the draft granted `akr_readonly` five column reads and EXECUTE, against phase 17's rule that `akr_readonly` reads nothing in `casework`, and `akr_readonly` has no USAGE on `canon`). A pack row takes the tier of its kind (`admin.packs.kind`, readable by `akr_readonly` since phase 02): a case or report pack is 'suspicion' for the viewer. Unknown types (for example `evidence` or `report` without a case id) fail closed to 'suspicion' (Open doubts 2).

### 1.4 `0079_admin_guards_grants.sql`

```sql
-- migration 0079_admin_guards_grants.sql
-- Phase 24. Guards the application cannot bypass, grants per write path, settings, classification and retention.
-- Idempotent: functions are replaced, triggers dropped before creation, inserts use ON CONFLICT DO NOTHING.

-- Generic immutability for tables written once: validation packs and assessor samples.
CREATE OR REPLACE FUNCTION admin.governance_immutable() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION '% rows are written once; a new version is a new row', TG_TABLE_NAME
        USING ERRCODE = 'insufficient_privilege';
END $$;
DROP TRIGGER IF EXISTS trg_validation_packs_immutable ON admin.validation_packs;
CREATE TRIGGER trg_validation_packs_immutable BEFORE UPDATE OR DELETE ON admin.validation_packs
    FOR EACH ROW EXECUTE FUNCTION admin.governance_immutable();
DROP TRIGGER IF EXISTS trg_assessor_samples_immutable ON admin.assessor_samples;
CREATE TRIGGER trg_assessor_samples_immutable BEFORE UPDATE OR DELETE ON admin.assessor_samples
    FOR EACH ROW EXECUTE FUNCTION admin.governance_immutable();

-- Registers: identity, text and author are fixed; a closed row is final; nothing is deleted.
CREATE OR REPLACE FUNCTION admin.registers_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'register rows are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.id <> OLD.id OR NEW.kind <> OLD.kind OR NEW.source <> OLD.source
       OR NEW.source_key IS DISTINCT FROM OLD.source_key OR NEW.title <> OLD.title OR NEW.body <> OLD.body
       OR NEW.created_by IS DISTINCT FROM OLD.created_by OR NEW.created_at <> OLD.created_at THEN
        RAISE EXCEPTION 'a register row keeps its identity, text and author' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.status = 'closed' THEN
        RAISE EXCEPTION 'a closed register row is final' USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_registers_guard ON admin.registers;
CREATE TRIGGER trg_registers_guard BEFORE UPDATE OR DELETE ON admin.registers
    FOR EACH ROW EXECUTE FUNCTION admin.registers_guard();

-- Protected attributes: never deleted; the code and the class are fixed; retirement is a status.
CREATE OR REPLACE FUNCTION admin.protected_attributes_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'protected attributes are retired, never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.code <> OLD.code OR NEW.attribute_class <> OLD.attribute_class OR NEW.source_column <> OLD.source_column THEN
        RAISE EXCEPTION 'the code, class and source column of a protected attribute are fixed' USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_protected_attributes_guard ON admin.protected_attributes;
CREATE TRIGGER trg_protected_attributes_guard BEFORE UPDATE OR DELETE ON admin.protected_attributes
    FOR EACH ROW EXECUTE FUNCTION admin.protected_attributes_guard();

-- Fairness metrics: the measured values are written once; only the status and the register link move.
CREATE OR REPLACE FUNCTION admin.fairness_metrics_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'fairness metrics are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.period <> OLD.period OR NEW.metric <> OLD.metric OR NEW.group_kind <> OLD.group_kind
       OR NEW.group_ref <> OLD.group_ref OR NEW.customers IS DISTINCT FROM OLD.customers
       OR NEW.events IS DISTINCT FROM OLD.events OR NEW.value IS DISTINCT FROM OLD.value
       OR NEW.reference_value IS DISTINCT FROM OLD.reference_value OR NEW.ratio IS DISTINCT FROM OLD.ratio
       OR NEW.suppressed <> OLD.suppressed OR NEW.computed_at <> OLD.computed_at THEN
        RAISE EXCEPTION 'a fairness metric is computed once; a changed input is a new period run' USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_fairness_metrics_guard ON admin.fairness_metrics;
CREATE TRIGGER trg_fairness_metrics_guard BEFORE UPDATE OR DELETE ON admin.fairness_metrics
    FOR EACH ROW EXECUTE FUNCTION admin.fairness_metrics_guard();

-- Training: a record is written once; only the void columns move, and a void is final.
CREATE OR REPLACE FUNCTION admin.training_records_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'training records are voided, never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.voided_at IS NOT NULL THEN
        RAISE EXCEPTION 'a voided training record is final' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.id <> OLD.id OR NEW.user_id <> OLD.user_id OR NEW.course_code <> OLD.course_code
       OR NEW.completed_on <> OLD.completed_on OR NEW.expires_on IS DISTINCT FROM OLD.expires_on
       OR NEW.score_pct IS DISTINCT FROM OLD.score_pct OR NEW.evidence_id IS DISTINCT FROM OLD.evidence_id
       OR NEW.evidence_ref IS DISTINCT FROM OLD.evidence_ref OR NEW.recorded_by <> OLD.recorded_by
       OR NEW.recorded_at <> OLD.recorded_at THEN
        RAISE EXCEPTION 'a training record is written once' USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_training_records_guard ON admin.training_records;
CREATE TRIGGER trg_training_records_guard BEFORE UPDATE OR DELETE ON admin.training_records
    FOR EACH ROW EXECUTE FUNCTION admin.training_records_guard();

-- Verify runs: the request is fixed; the status moves forward only (queued, running, one terminal state).
CREATE OR REPLACE FUNCTION admin.verify_runs_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'verify runs are kept' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.id <> OLD.id OR NEW.requested_by <> OLD.requested_by OR NEW.scope <> OLD.scope OR NEW.created_at <> OLD.created_at THEN
        RAISE EXCEPTION 'a verify run request is fixed' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.status IN ('pass', 'findings', 'failed') THEN
        RAISE EXCEPTION 'a finished verify run is final' USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_verify_runs_guard ON admin.verify_runs;
CREATE TRIGGER trg_verify_runs_guard BEFORE UPDATE OR DELETE ON admin.verify_runs
    FOR EACH ROW EXECUTE FUNCTION admin.verify_runs_guard();

-- Grants per write path (akr_app is the API and worker role; no other role writes these tables).
GRANT SELECT, INSERT ON admin.registers TO akr_app;
GRANT UPDATE (severity, owner_id, due_on, repeat_of, repeat_flag, status, closure_code, closed_by, closed_at, updated_at, version)
    ON admin.registers TO akr_app;
GRANT SELECT, INSERT ON admin.validation_packs TO akr_app;        -- written once; no UPDATE column grant exists
GRANT SELECT, INSERT ON admin.protected_attributes TO akr_app;
GRANT UPDATE (label, allowed_uses, combined_trigger_ok, owner_id, basis, status, updated_by, updated_at, version)
    ON admin.protected_attributes TO akr_app;
GRANT SELECT, INSERT ON admin.fairness_metrics TO akr_app;
GRANT UPDATE (status, register_id) ON admin.fairness_metrics TO akr_app;
GRANT SELECT, INSERT ON admin.training_records TO akr_app;
GRANT UPDATE (voided_by, voided_at, void_reason_code) ON admin.training_records TO akr_app;
GRANT SELECT, INSERT ON admin.assessor_samples TO akr_app;
GRANT SELECT, INSERT ON admin.verify_runs TO akr_app;
GRANT UPDATE (status, result, error_code, started_at, finished_at) ON admin.verify_runs TO akr_app;

-- Seed of the protected-attribute register (GV-36, GV-37). Owners are set by ADM with the bank MRO (Open doubts 14).
INSERT INTO admin.protected_attributes (code, label, attribute_class, source_column, allowed_uses, combined_trigger_ok, prohibited_as_input, basis) VALUES
 ('nationality',       '{"ar": "الجنسية", "en": "Nationality"}',                         'nationality',         'canon.party.nationality',       '{fairness_metric,segmentation,reporting}', TRUE,  FALSE, '{"basis": "GV-36; AI Act recital 42; never the sole trigger"}'),
 ('residence_country',  '{"ar": "بلد الإقامة", "en": "Country of residence"}',              'residence',           'canon.party.residence_country', '{fairness_metric,segmentation,reporting}', TRUE,  FALSE, '{"basis": "GV-36; never the sole trigger"}'),
 ('governorate',         '{"ar": "المحافظة", "en": "Governorate"}',                         'geography',           'canon.party_contact.address (key governorate)',     '{fairness_metric,segmentation,reporting}', TRUE,  FALSE, '{"basis": "GV-36 geography; never the sole trigger"}'),
 ('name_origin',         '{"ar": "أصل الاسم (مشتق من الاسم)", "en": "Name-derived origin"}',      'name_derived',       '(derived; no column)',          '{}',                                          FALSE, TRUE,  '{"basis": "GV-37: prohibited as a model input; proxy scan in validation"}'),
 ('sect_tribe_ethnicity', '{"ar": "الطائفة أو العشيرة أو الإثنية", "en": "Sect, tribe or ethnicity"}', 'sect_tribe_ethnicity', '(derived; no column)',      '{}',                                          FALSE, TRUE,  '{"basis": "GV-37: prohibited as a model input; proxy scan in validation"}')
ON CONFLICT (code) DO NOTHING;

-- Settings (copy of the phase 12 insert format). floor only when governed = TRUE (phase 00 CHECK).
INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('audit.search_page_size',        '100', 'integer', NULL, FALSE,
  '{"ar": "حجم صفحة البحث في سجل التدقيق", "en": "Audit search page size (rows)"}'),
 ('audit.search_max_window_days',  '366', 'integer', NULL, FALSE,
  '{"ar": "أقصى مدى زمني للبحث في سجل التدقيق (أيام)", "en": "Longest audit search window (days)"}'),
 ('audit.verify_max_rows',         '50000000', 'integer', NULL, FALSE,
  '{"ar": "أقصى عدد صفوف لتشغيل التحقق (حماية التكلفة)", "en": "Verify run cost guard (rows estimated)"}'),
 ('register.finding_due_days',     '90', 'integer', '{"min": 30, "max": 365, "basis": "GV-34 quarterly committee and yearly Board cycle"}'::jsonb, TRUE,
  '{"ar": "مهلة معالجة النتائج (أيام)", "en": "Finding due period (days from opening)"}'),
 ('fairness.review_ratio',         '1.25', 'decimal', '{"max": 1.25, "basis": "GV-38 internal review trigger; a bank may only tighten it"}'::jsonb, TRUE,
  '{"ar": "نسبة مراجعة الإنصاف لكل مجموعة", "en": "Group rate ratio that opens a fairness review"}'),
 ('review.rules_months',           '12', 'integer', NULL, TRUE,
  '{"ar": "فترة مراجعة القواعد (أشهر)", "en": "Rule review interval (months; GV-22 standard 12)"}'),
 ('validation.evidence_max_days',  '180', 'integer', NULL, TRUE,
  '{"ar": "أقصى عمر لأدلة التحقق قبل الإصدار (أيام)", "en": "Oldest replay or test evidence accepted for a validation pack (days)"}'),
 -- no 'assessor.grant_max_days' row: the cap of an AS grant is phase 04's access.assessor_max_days (section 7.1)
 -- (its label line is removed with it)
 ('assessor.sample_max_items',     '200', 'integer', NULL, FALSE,
  '{"ar": "أقصى حجم للعينة العشوائية", "en": "Largest assessor sample (items)"}'),
 ('command.clock_horizon_days',    '14', 'integer', NULL, FALSE,
  '{"ar": "أفق الساعات المستحقة في لوحة القيادة (أيام)", "en": "Clocks board horizon (days ahead)"}'),
 ('training.expiry_months',        '24', 'integer', NULL, FALSE,
  '{"ar": "مدة صلاحية التدريب (أشهر)", "en": "Default training validity (months)"}'),
 -- Schedules are settings of value_type cron (phase 00 convention; a JSON string of six fields with seconds).
 ('schedule.register_post_review_sync', '"0 */15 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مزامنة نتائج المراجعة بعد التغيير", "en": "Post-change review sync (every 15 minutes)"}'),
 ('schedule.review_calendar_sweep', '"0 0 6 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول فحص تقويم المراجعات", "en": "Review calendar sweep (06:00 bank time)"}'),
 ('schedule.fairness_monthly', '"0 30 4 2 * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مقاييس الإنصاف الشهرية", "en": "Monthly fairness metrics (day 2, 04:30 bank time)"}')
ON CONFLICT (key) DO NOTHING;

-- Column classification (copy of the phase 18 insert format). No column here is personal, financial, secret or card.
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('registers', 'internal', ARRAY['id', 'kind', 'source', 'source_key', 'title', 'body', 'severity', 'owner_id', 'due_on',
                                  'repeat_of', 'repeat_flag', 'status', 'closure_code', 'closed_by', 'closed_at',
                                  'created_by', 'created_at', 'updated_at', 'version']),
  ('protected_attributes', 'internal', ARRAY['code', 'label', 'attribute_class', 'source_column', 'allowed_uses',
                                             'combined_trigger_ok', 'sole_trigger_ok', 'prohibited_as_input', 'owner_id',
                                             'basis', 'status', 'updated_by', 'updated_at', 'version']),
  ('validation_packs', 'internal', ARRAY['id', 'inventory_item_id', 'inventory_version', 'version', 'period_from',
                                         'period_to', 'parts', 'verdict', 'owner_id', 'validator_id', 'evidence_id',
                                         'sha256', 'issued_at']),
  ('fairness_metrics', 'internal', ARRAY['id', 'period', 'metric', 'group_kind', 'group_ref', 'customers', 'events',
                                         'value', 'reference_value', 'ratio', 'suppressed', 'status', 'register_id',
                                         'computed_at']),
  ('training_records', 'internal', ARRAY['id', 'user_id', 'course_code', 'completed_on', 'expires_on', 'score_pct',
                                         'evidence_id', 'evidence_ref', 'recorded_by', 'recorded_at', 'voided_by',
                                         'voided_at', 'void_reason_code']),
  ('assessor_samples', 'internal', ARRAY['id', 'drawn_by', 'grant_id', 'frame', 'frame_from', 'frame_to', 'frame_size',
                                         'sample_size', 'seed', 'algorithm', 'frame_hash', 'items', 'drawn_at']),
  ('verify_runs', 'internal', ARRAY['id', 'requested_by', 'scope', 'status', 'result', 'error_code', 'created_at',
                                    'started_at', 'finished_at'])
) AS v(t, cls, cols) CROSS JOIN LATERAL unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

-- Retention (copy of the phase 23 insert format). Every row names a clock column except retirement and never.
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'registers',            'R9',  'closure',   'closed_at',   'Findings and warnings: from closure; an open row is never due'),
 ('admin', 'protected_attributes', 'R9',  'retirement', NULL,         'Register of protected attributes; retired rows are kept'),
 ('admin', 'validation_packs',     'R11', 'age',       'issued_at',   'Validation evidence: ten years (DATA-MODEL R11)'),
 ('admin', 'fairness_metrics',     'R9',  'age',       'computed_at', 'Outcome fairness metrics: kept with the configuration history'),
 ('admin', 'training_records',     'R9',  'age',       'recorded_at', 'Training evidence of staff; kept with the staff record'),
 ('admin', 'assessor_samples',     'R9',  'age',       'drawn_at',    'Assessor draws, seeds and ids; kept with the assessment file'),
 ('admin', 'verify_runs',          'R10', 'never',     NULL,          'Verification results are audit evidence: kept without limit')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

## 2. DDD Implementation (Rust)

- **Crate placement:** domain types and pure rules in `domain::governance` (akr-core, no IO: the register state machine, the fairness arithmetic and suppression, the seven-part validation rule, the sample rank, the tier mapping of section 3.1, the proxy scan of section 3.5); services in `services::governance` (akr-api and akr-worker); repositories in `db::governance` (sqlx, `akr_app`, plus the `akr_readonly` pool for the viewer only); the read-only SQL (the viewer, access history, verify-run estimate, pack verify) lives in `db::audit_viewer`; the phase 02 Verifier is the only other code on `readonly_pool`.
- **New names (checked with grep across docs/plan):** `AuditViewerService`, `AccessHistoryService`, `GovernanceRegisterService` (phase 19 already has `RegisterService`), `FairnessService`, `ValidationPackService`, `CommandCentreService`, `ClockBoardService`, `AssessorService`, `TrainingRegisterService`. Pack kinds use phase 02's `PackService`; no `PackBuilder` (phase 16 names one for another purpose).
- **Domain entities** (`Debug, Serialize, Deserialize, FromRow, ToSchema`):
  - `Register { id, kind: RegisterKind (finding | warning), source: RegisterSource, source_key, title: Bilingual, body: Value, severity: Option<Severity>, owner_id: Option<Uuid>, due_on: Option<NaiveDate>, repeat_of, repeat_flag, status: RegisterStatus, closure_code, closed_by, closed_at, created_by, created_at, updated_at, version }`
  - `ProtectedAttribute { code, label, attribute_class, source_column, allowed_uses: Vec<String>, combined_trigger_ok, sole_trigger_ok (always false), prohibited_as_input, owner_id, basis, status, version }`
  - `ValidationPack { id, inventory_item_id, inventory_version, version, period_from, period_to, parts: Value, verdict: Verdict, owner_id, validator_id, evidence_id, sha256, issued_at }`
  - `FairnessMetric { id, period, metric: FairnessMetricKind, group_kind, group_ref, customers: Option<i32>, events: Option<i32>, value: Option<Decimal>, reference_value: Option<Decimal>, ratio: Option<Decimal>, suppressed, status, register_id, computed_at }`
  - `TrainingRecord { id, user_id, course_code, completed_on, expires_on, score_pct, evidence_id, evidence_ref, recorded_by, recorded_at, voided_by, voided_at, void_reason_code }`
  - `AssessorSample { id, drawn_by, grant_id, frame: SampleFrame, frame_from, frame_to, frame_size, sample_size, seed: [u8; 32], algorithm, frame_hash: [u8; 32], items: Vec<SampleItem>, drawn_at }`
  - `VerifyRun { id, requested_by, scope: VerifyScope, status: VerifyStatus, result: Option<VerifySummary>, error_code, created_at, started_at, finished_at }`
- **Create and update DTOs:** `CreateRegisterEntry { kind, title, body, severity, owner_id, due_on, repeat_of }` (kinds `finding` and `warning` only; a warning is never created by a person); `UpdateRegisterEntry` (every field `Option<T>`: `status`, `owner_id`, `due_on`, `severity`, `closure_code`); `CreateValidationPack { inventory_item_id, period_from, period_to }`; `CreateTrainingRecord { user_id, course_code, completed_on, expires_on, score_pct, evidence_id, evidence_ref }`; `VoidTrainingRecord { void_reason_code }`; `DrawSample { frame, frame_from, frame_to, sample_size }`; `RequestVerify { scope }`; `CreatePack { kind, scope, lang }` (phase 06 DTO, kinds extended); `VerifyPackRequest { sha256 | pack_id }`; `ReviewFairnessMetric { outcome: justified | changed, register_entry: CreateRegisterEntry, comment_code }`.
- **Repository traits** (domain layer; each write takes `&mut Tx` and an expected version where the row is versioned):
  - `RegisterRepository`: `insert(tx, NewRegister) -> Register`; `update(tx, id, UpdateRegisterEntry, expected_version) -> Register` (the version in the `WHERE`); `find_by_source_key(conn, key) -> Option<Register>`; `list(conn, RegisterFilter, Keyset) -> Page<Register>`; `open_for_owner(conn, owner_id) -> Vec<Register>`.
  - `ValidationPackRepository`: `insert(tx, NewValidationPack)`, `next_version(conn, item_id) -> i32`, `list_for_item(conn, item_id, Keyset)`, `get(conn, id)`.
  - `FairnessRepository`: `upsert_period(tx, period, rows) -> u64` (insert only; a second run of a period is refused unless the rows are absent), `list(conn, period, metric, Keyset)`, `set_status(tx, id, status, register_id)`.
  - `TrainingRepository`: `insert`, `void(tx, id, by, reason)`, `list(conn, filter, Keyset)`, `expiring(conn, until) -> Vec<TrainingRecord>`.
  - `SampleRepository`: `insert(tx, AssessorSample)`, `get(conn, id)`, `frame_ids(conn, frame, from, to) -> Vec<Uuid>` (sorted, for the frame hash).
  - `VerifyRunRepository`: `insert`, `mark_running(tx, id)`, `finish(tx, id, status, result, error_code)`, `get`.
  - `AuditReadRepository` (`akr_readonly` only): `search_events(conn, AuditFilter, Keyset, viewer_tier_rule) -> Page<AuditRow>`, `chain_status(conn) -> Vec<ChainRow>`, `anchors(conn, Keyset) -> Page<AnchorRow>`, `history(conn, kind, id, Keyset) -> Page<AccessRow>`, `resource_tier(conn, type, id, case) -> Tier` (calls the SQL of section 1.3).
  - `CommandReadRepository` and `ClockReadRepository` (`akr_app`): one method per tile and per clock source of section 3.7 and 3.8; each method takes the caller's visibility predicate (phase 17, `services::visibility::build`) as a bound parameter, never as text.
- **Pack source readers** (ports, implemented in the same crate): `BoardSource::periodic_final(period) -> Option<PeriodicReport>` (phase 21), `CaseFileSource` (the read model behind phase 17's route `GET /cases/{id}`, `CaseReadService::header`, with the fields phase 18 adds; not a new query), `ReportFileSource` (phase 21 `rpt.reports` with versions and filings), `AssessorPackSource` (section 3.9, the D2 map, the authority matrix of phase 09, the coverage certificates of phase 12), `CertificationSource` (section 3.11).
- **utoipa schemas (response and request DTOs):** `AuditEventView` (stream, id, occurred_at, actor_id, action, resource_type, resource_id, case_id, decision, details, chain: `{prev_hash8, entry_hash8}`), `AuditSearchPage`, `ChainStatusView`, `AnchorView`, `VerifyRunView`, `AccessHistoryRow`, `ControlReportView`, `RegisterView`, `RegisterEntryView`, `ReviewCalendarRow`, `FairnessMetricView`, `ValidationPackView`, `CommandCentreView` (tiles array), `ClockRow`, `PackView` (phase 06 extended), `VerifyPackView`, `D2MatrixView`, `AssessorSampleView`, `TraceView`, `TrainingRecordView`.
- **Route table** (`| Method | Path | Scope | Description (Cedar action) |`; `[added]` is a row this phase adds to API.md 3.x; `[ext]` extends a phase 06 route):

| Method | Path | Scope | Description (cedar action) |
|-|-|-|-|
| GET | `/api/v1/audit/events` | `audit.read` | Audit search over `audit_log` and `access_event`, cursor, `purpose` required; every search written as `audit.searched` (`audit.read_events`) |
| GET | `/api/v1/audit/events/{id}` | `audit.read` | One event with its chain position; masked 404 for a suspicion row (`audit.read_events`) [added] |
| GET | `/api/v1/audit/chain-status` | `audit.read` | Chain and anchor status per partition and stream (`audit.chain_status`) |
| GET | `/api/v1/audit/anchors` | `audit.read` | Signed Merkle anchors, cursor (`audit.chain_status`) |
| POST | `/api/v1/audit/verify-runs` | `audit.read` | Queue a verify run over a range; `Idempotency-Key`; 202 (`audit.verify`) |
| GET | `/api/v1/audit/verify-runs/{id}` | `audit.read` | Status and result summary, counts and kinds only (`audit.verify`) |
| GET | `/api/v1/records/{kind}/{id}/access-history` | `audit.read` | Who read or changed a record, with purpose; masked 404 for a suspicion record outside M (`audit.access_history`) |
| GET | `/api/v1/access/control-report` | `dashboard.read`, `security.ops` | Monthly tipping-off control report, counts only (`access.control_report`) [added] |
| GET | `/api/v1/registers/{kind}` | `dashboard.read`, `model.risk` | Registers `decisions` (read view), `exclusions` (read view), `findings`, `warnings`, `fairness`; cursor (`register.read`) |
| POST | `/api/v1/registers/{kind}/entries` | `config.author`, `config.approve` | Add a finding (M, CH, MRO); `kind` other than finding is 422; `Idempotency-Key` (`register.write`) |
| GET | `/api/v1/registers/{kind}/entries/{id}` | `dashboard.read`, `model.risk` | One finding or warning with its history (`register.read`) [added] |
| PATCH | `/api/v1/registers/{kind}/entries/{id}` | `config.author`, `config.approve` | Owner, due date, severity, status, closure code; `If-Match`; step-up `approval` when `accepted_risk` (`register.write`, `register.close`) [added] |
| GET | `/api/v1/registers/review-calendar` | `dashboard.read`, `model.risk` | Rules, exclusions, change-request reviews, readiness and scenario review dates with amber and red (`register.read`) [added] |
| POST | `/api/v1/fairness/metrics/{id}/review` | `config.author`, `config.approve` | Justify or change a review-status group rate; `If-Match` (`fairness.review`) [added] |
| GET | `/api/v1/fairness/attributes` | `dashboard.read`, `model.risk` | The protected-attribute register and allowed-use matrix (`fairness.read`) [added] |
| POST | `/api/v1/inventory/{id}/validation-packs` | `model.risk` | Issue a validation pack (seven parts) for an inventory item; validator is the caller, never the owner; `Idempotency-Key` (`validation.issue`) [added] |
| GET | `/api/v1/inventory/{id}/validation-packs` | `model.risk`, `audit.read` | Versions of the item's validation packs, cursor (`validation.read`) [added] |
| GET | `/api/v1/validation-packs/{id}` | `model.risk`, `audit.read` | One validation pack with its parts and evidence hash (`validation.read`) [added] |
| GET | `/api/v1/dashboard/command-centre` | `dashboard.read` | Tiles with definition, formula, data-as-of and the list each opens (`command.read`) |
| GET | `/api/v1/dashboard/clocks` | `dashboard.read` | All clock instances from the clock sources, keyset by due instant (`clock.read`) |
| POST | `/api/v1/packs` | `dashboard.read`, `data.steward`, `assessor.read` | [ext] Build a pack (AS: kind `assessor` only, Cedar); the kinds `board`, `case`, `report`, `assessor`, `certification` are added (`pack.create`) |
| GET | `/api/v1/packs/{id}` | `dashboard.read`, `data.steward`, `assessor.read` | [ext] Pack metadata and build status (`pack.read`) |
| GET | `/api/v1/packs/{id}/download` | `dashboard.read`, `data.steward`, `assessor.read` | [ext] Download, audited, step-up class `export` (`pack.download`) |
| POST | `/api/v1/packs/verify` | `self` | Verify a pack or document hash against the vault and the audit chain; uniform answer for a masked pack (`pack.verify`) |
| GET | `/api/v1/assessor/d2-matrix` | `assessor.read` | D2 clause map with live evidence links and a status per clause (`assessor.d2`) |
| POST | `/api/v1/assessor/samples` | `assessor.read` | Draw a seeded, recorded random sample from a frame; `Idempotency-Key` (`assessor.sample`) |
| GET | `/api/v1/assessor/samples/{id}` | `assessor.read` | A recorded sample with its seed, frame hash and items (`assessor.sample`) [added] |
| GET | `/api/v1/assessor/trace/{alert_id}` | `assessor.read` | End-to-end trace: CBS row, transaction, alert, case, decision type, report; metadata only (`assessor.trace`) |
| GET | `/api/v1/training/register` | `dashboard.read` | Training register, cursor, filter by user and course (`training.read`) |
| POST | `/api/v1/training/register` | `admin.users` | Record a training completion; `Idempotency-Key`; never for oneself (`training.record`) |
| POST | `/api/v1/training/register/{id}/void` | `admin.users` | Void a record with a reason code (`training.void`) [added] |

- **Router functions:** `audit_viewer::router()` (rows 1 to 6), `access_history::router()` (7 and 8), `registers::router()` (9 to 13), `fairness::router()` (14, 15), `validation::router()` (16 to 18), `command_centre::router()` (19, 20), `packs::router()` (21 to 24; the phase 06 function gains the kinds, its mount does not change), `assessor::router()` (25 to 28), `training::router()` (29 to 31). Each built with `OpenApiRouter` and `authz_route!` (phase 03) so every route names its scope and Cedar action.
- **Route count:** 31 rows in this table (XL; sections 7.12 and 7.13 add eight more, three export routes and five parallel-run routes): 19 are rows of API.md that this phase owns or extends (3.3, 3.4, 3.15, 3.16), of which 3 (21 to 23) are phase 06 routes whose kinds this phase adds; 12 rows are added beyond API.md and marked `[added]`. The phase 02 Verifier is reached by the job `audit_verify_run` (section 4.1), not by a route.

## 3. Business Logic

Reads of `audit.audit_log` and `audit.access_event` are possible only as `akr_readonly` (phase 02: the application roles hold no column of the audit details or the access log). Phase 24 therefore runs every query of sections 3.1 to 3.3 on `readonly_pool` (DR-218). Every other read in this phase runs on the `akr_app` pool with the caller's visibility predicate.

### 3.1 The audit viewer (`AuditViewerService`; G6; SP-42)

**Who sees what.** IA and SEC only (`audit.read_events`, `audit.chain_status`, `audit.verify`). M has no audit viewer; M reads the access history of a record (section 3.2). Scope `audit.read` admits IA and SEC; the table in 3.12 is the fine gate.

**`search(ctx, query, cursor) -> AuditSearchPage`** (`audit.read_events`; route 1):
1. Cedar `audit.read_events` on `Akr::System`. Refusal: 403 `E_FORBIDDEN`.
2. Purpose: `purpose` is required and one of `internal_audit`, `supervisory_inspection`, `other` (phase 03 `admin.purpose_codes`). Missing: 422 `E_PURPOSE_REQUIRED`. `other` requires `reason` of 1 to 200 characters (422 `E_VALIDATION`, sub-code `reason_required`). SEC has no purpose code of its own: SEC uses `other` with a reason (Open doubts 23).
3. Window: `from` and `to` on `occurred_at`; default the last 31 days; a span longer than `audit.search_max_window_days` is 422 `E_VALIDATION` `window_too_long`. Partition pruning follows from the window.
4. Filters: `stream` (`audit_log`, `access_event` or both), `action` (exact, or a prefix ending in `.`), `actor_id`, `resource_type`, `resource_id`, `case_id` (a column of `audit_log` only: with `case_id` set the search reads `audit_log` alone, and the reads of a case are found with `resource_type = case` and `resource_id`), and for `access_event` only `decision` and `tier`.
5. A filter on `resource_id` or `case_id` resolves the tier first (`audit.resource_tier`, section 1.3). When the tier is `suspicion` and the caller is not M, the answer is 404 `E_NOT_FOUND` with the body of a missing id (the masked kind `audit_event`, phase 03 `MASKED_KINDS`). The caller is never M here, so every suspicion filter by id answers 404.
6. Query on `readonly_pool`, keyset on `(occurred_at, id)`, page size `min(limit, audit.search_page_size)`. The predicate of every row is the tier rule: `audit.resource_tier(resource_type, resource_id, case_id) <> 'suspicion'` for `audit_log`; for `access_event` the higher of the stored `tier` and the current `audit.resource_tier(resource_type, resource_id, NULL)` must not be `suspicion` (a record raised later hides its earlier reads too). Suspicion rows are not returned and not counted: the page carries `has_more` and no total, so the count cannot reveal them.
7. `details` is passed through only for keys of the phase 02 catalogue (ids, codes, counts, hashes). `request` (ip, session, correlation id) is returned to IA and SEC only.
8. Write `audit.searched {filters (names only), purpose, result_count, has_more}` through `AuditSink` in its own transaction (`akr_audit_writer`) BEFORE the response is sent. If the audit write fails the search fails with 503 `E_UNAVAILABLE`; no result leaves the viewer unaudited (SP-42).
9. Export is not in this phase (SP-42's export cap is Open doubts 11).

**`event(ctx, id, occurred_at)`** (`audit.read_events`; route 2): `occurred_at` is required (partition pruning). The same tier rule as step 5 applies to the one row (404 when suspicion). Audit `audit.event_read {event_id, stream}`.

**`chain_status(ctx)`** (`audit.chain_status`; route 3): one row per stream and partition: anchored rows (the sum of `audit_anchors.entry_count` of the partition), last anchored instant, anchor lag in seconds (`audit.anchor_lag_alert_s`), the result of the latest verify run that covered the partition (section 3.3). Volume only: the answer names no resource (Open doubts 3). Reads `audit.audit_anchors` and `admin.verify_runs` on `readonly_pool` and `akr_app`.

**`anchors(ctx, cursor)`** (`audit.chain_status`; route 4): signed anchors by stream, keyset `(anchored_at DESC, id)`: stream, partition, `range_from`, `range_to`, `entry_count`, `merkle_root` (hex), `statement_sha256` (hex), `key_name`, `key_version`, `anchored_at`, and whether `object_version_id` is present. The WORM object key is not returned.

### 3.2 Access history and the tipping-off control report (`AccessHistoryService`; G5)

**`history(ctx, kind, id, filters, cursor)`** (`audit.access_history`; route 7):
1. Resolve the case that holds the record: `case` gives its own id; `alert` through `casework.case_alerts`; `report` through `rpt.reports.case_id`; `evidence` through `casework.evidence_items.case_id`; anything else has no case. Then the tier is `audit.resource_tier(kind, id, case_id)`.
2. Cedar `audit.access_history` on `Akr::AuditTarget { kind, tier }`. M, SEC and IA are admitted. A `suspicion` record is refused for everyone but M (`audit.history.suspicion`): the answer is 404 `E_NOT_FOUND` with the body of a missing record (masked kind `audit_target`). This is the decision of DR-219: IA and SEC do not learn that a suspicion record exists through its access history.
3. Rows: `access_event` where `resource_type = kind` and `resource_id = id`, on `readonly_pool`, default window 90 days, keyset `(occurred_at, id)`: `occurred_at`, `actor_id`, `actor_role`, `on_behalf_of`, `action`, `purpose_code`, `decision`, `tier`, `client_ip`. The reason text (`reason`, sealed) is never returned.
4. The read is itself an access event (phase 03 `AccessScope`): `resource_type = 'access_history'`, `resource_id = id`, `action = view`, purpose `internal_audit` for IA, `other` with a reason for SEC and M.

**`control_report(ctx, period)`** (`access.control_report`; route 8; M and SEC; scope `dashboard.read` and `security.ops`): one month, counts only, computed on `readonly_pool`:
- denied and conflicting reads (`access_event.decision` in `deny`, `conflict`), split by the stored `tier`;
- break-glass uses (`purpose_code = 'break_glass'`), exports, prints and downloads (`action` in `export`, `print`, `download`), unmask actions (`action = 'unmask'`);
- tier raises (`audit_log` action `party.tier_raised`, phase 18) and role grants that started or ended in the month (`admin.role_grants`, `valid_from`, `valid_to`, `revoked_at`);
- manual grants on suspicion-tier cases made and expired in the month (phase 18 `casework.case_acl`; section 7+);
- canary triggers: the field is `null` with `canary_not_built: true` (no canary record exists in any phase; Open doubts 12).
Every count from 1 to `report.small_number_threshold` minus 1 is returned as `null` with `suppressed: true`, as on the command centre (lead review). Audit `access.control_report_generated {period}`.

### 3.3 Verify runs (the phase 02 Verifier as a job; route 5 and 6)

**`request(ctx, scope, idem)`** (`audit.verify`; IA and SEC):
1. Validate `scope`: `streams` (subset of `audit_log`, `access_event`, `case_event`), `from` and `to` as `YYYY-MM`, `all` (boolean), `worm` (boolean). `all = true` estimates the rows of the range from `pg_class.reltuples` on `readonly_pool`; above `audit.verify_max_rows` it is 422 `E_COST_GUARD` with the estimate in the body.
2. Insert `admin.verify_runs` (status `queued`, `requested_by` = the caller) and enqueue `audit_verify_run {run_id}` in the same transaction. Audit `audit.verify_requested {run_id, scope}`.
3. 202 with `Location`. A repeat with the same `Idempotency-Key` returns the same run.

**Job `audit_verify_run`** (akr-worker; section 4.1): mark `running` (`started_at`), call phase 02 `Verifier::run(scope)` (it reads as `akr_readonly` and emits `audit.verify_completed` with trigger `manual`), and map the `VerifyReport` to the stored summary: `partitions`, `rows`, `anchors`, `objects`, `findings_by_kind` (kind and count only). Entry ids, anchor ids, object keys and the `detail` text of a finding are not stored (DR-220). Status `pass` when no finding is an error or critical, otherwise `findings`. Any error after the last attempt sets `failed` with `error_code = E_INTERNAL`. A critical finding pages as the phase 02 P1 incident; this phase adds no second alert.

**`get(ctx, id)`** (`audit.verify`; route 6): status, the summary, `requested_by` display name, `finished_at`.

### 3.4 Registers, decisions and exclusions as read views, the review calendar (`GovernanceRegisterService`; G4)

**Kinds.** `decisions` and `exclusions` are READ VIEWS (no copy, DR-221):
- `decisions`: `casework.decisions` joined to `casework.cases` for the tier. Columns: decision id, `case_ref` (from `casework.cases`), `type`, `reason_codes`, `decided_at`, `decider_id`, `proposer_id`. `text_enc` is never selected. Predicate: phase 17's `services::visibility::build` for the caller (with phase 18's case access list and phase 19's QA grants as admitting clauses), never the tier alone (lead review: phase 17 rule 4 limits restricted cases to A1, A2 and M with an `assigned_work` purpose). CH and IA see normal-tier decisions and the decisions their own QA grants admit; M sees all. Other decisions are counted on G1 only, suppressed.
- `exclusions`: `admin.exclusions` (phase 12), ended and active, with `reason`, `expires_at`, `review_due`, `approved_by`. The reason is business text: phase 12 scans it on creation.
- `findings`, `warnings`: `admin.registers` (section 3.4 below).
- `fairness`: `admin.fairness_metrics` for one period (section 3.5).
Keyset `(created_at DESC, id DESC)` for rows of this phase; `(decided_at DESC, id)` for decisions.

**`create_entry(ctx, kind, body, idem)`** (`register.write`; route 10): roles M, CH, MRO (Cedar in 3.12).
1. `kind` other than `finding` is 422 `E_VALIDATION` with sub-code `kind_not_writable` (warnings are opened only by jobs; decisions and exclusions are views).
2. Validate: `title` has `ar` and `en`; `severity` one of the four; `owner_id` is an active user holding a governance role (M, CH, MRO, DS, CFG); `subject_key` (the scenario code, the inventory id or the change request id the finding is about) is required in `body`.
3. The text fields of `title` and `body` pass `PolicyPackService::scan` (phase 09, scope `reason_text`). A forbidden term is 422 `E_VALIDATION` with the term's code only.
4. `due_on` = today plus `register.finding_due_days` (governed, floor 30) unless a later date is given.
5. Repeat rule (GV-34): when another finding with the same `subject_key` exists (open or closed), `repeat_of` is set to the newest of them and `repeat_flag = true`.
6. Insert with `source = manual`, `created_by` = the caller. Audit `register.created {id, kind, source, severity, repeat_flag}`.

**`update(ctx, kind, id, patch, expected_version)`** (`register.write`, or `register.close` for a closing transition; route 12): `If-Match` required (428 `E_PRECONDITION_REQUIRED`), stale version 409 `E_VERSION_CONFLICT`.
- Finding state machine: `open` to `in_progress`, `open` or `in_progress` to `accepted_risk`, `in_progress` or `accepted_risk` to `closed`, `open` to `closed` (a finding closed at once carries a closure code). A warning: `open` to `acknowledged` to `closed`, or `open` to `closed`. Any other move is 409 `E_NOT_ALLOWED_IN_STATE` with sub-code `transition_not_allowed`; a closed row is 409 `E_NOT_ALLOWED_IN_STATE` with sub-code `register_closed`.
- `accepted_risk` needs step-up class `approval` (`StepUpService::consume`, resource ref = the row). The owner cannot accept the risk of their own finding: 403 `E_SELF_APPROVAL`.
- `closure_code` is one of the code list shipped in the release: `EFFECTIVE`, `RULE_CHANGED`, `COMMITTEE_ACCEPTED`, `NOT_REPRODUCED`, `DUPLICATE` (Open doubts 20). The value is a CHECK pattern in section 1.1 and a list in code.
- Audit `register.updated {id, fields, status_from, status_to, stepup_id?}`; a closing move also `register.closed {id, closure_code}`.

**`review_calendar(ctx)`** (`register.read`; route 13): read only. Rows, each with `kind`, `subject`, `due_on`, `state` (`ok`, `amber` in the 30 days before `due_on`, `red` on and after `due_on`, as 7.3 says):
- rules: `admin.inventory_items` with `next_validation_due` (phase 04); the interval of a rule is `review.rules_months` (governed, 12, GV-22);
- exclusions: `admin.exclusions.review_due` of the active rows (phase 12);
- change-request reviews: `admin.change_requests.review_due_at` where `review_outcome` is `pending` (phase 04);
- readiness declarations: `scn.readiness_grades.review_due` (phase 06);
- scenarios (S-46): the scenario's activation instant (phase 16 column, confirmed in gap pass 1, Open doubts 11) plus one interval of `review.rules_months` is its `due_on`, and the state follows the one rule above (amber in the 30 days before, red on and after).
The job `review_calendar_sweep` (section 4.1) opens one warning per row that turns `red`, with `source_key = 'review:' || kind || ':' || subject_id || ':' || due_on`; a repeated sweep finds the row and does nothing (the unique index of section 1.1).

**`sync_post_review`** (job `register_post_review_sync`, section 4.1): reads `admin.instance_events` with `kind = 'post_review_recorded'` (phase 04) in the last 30 days where `payload.outcome` is `concern` or `rollback_needed`. For each: `source_key = 'post_review:' || instance_id || ':' || seq`, `INSERT ... ON CONFLICT (source_key) DO NOTHING`, one finding per concern: severity `high` for `rollback_needed`, `medium` for `concern`; owner = the change request's maker (phase 04 column name confirmed in gap pass 1); due = `register.finding_due_days` from the event day. Audit `register.created {source: post_review}`. This job is the feed phase 04 names (its lines 945 and 1651).

### 3.5 Fairness: protected attributes, the proxy scan and monthly outcome metrics (`FairnessService`; GV-36, GV-37, GV-38)

**The register** (`admin.protected_attributes`, seeded in 1.4; route 15 reads it). The rules of GV-36 are in the data: `sole_trigger_ok` is always false (CHECK); `combined_trigger_ok` says whether an attribute may sit beside another condition; `prohibited_as_input` (GV-37) bars a name-derived or sect, tribe or ethnicity feature from any model or score. Changing an attribute's `allowed_uses` or owner is a change request in the `config.author` family (phase 04 ChangeTarget `protected_attribute`, registered in this phase); this phase adds the target, not the route.

**`ProtectedAttributeCheck`** (pure function in `domain::governance`): input is a scenario's condition tree (phase 16 DSL, read from the version row); output is a list of refusals. A refusal is raised when a trigger's condition set reduces to one leaf that reads a protected attribute (`sole_trigger`), or when a leaf reads a `prohibited_as_input` attribute at all. The check runs in two places: in the monthly job below (every active scenario, each run; refusals become warnings), and as a requested gate in phase 16's scenario change target `check_before_decision` (Open doubts 13).

**`run_period(period)`** (job `fairness_monthly`, section 4.1; period = the month before the run):
1. Customers per group: active customers of the period end (`canon.customer` rows current at the period end, as phase 10 defines currency), grouped by `nationality`, `residence_country` (both columns of `canon.party`), `governorate` (`canon.party_contact.address` key `governorate`, Open doubts 24) and `segment` (the customer's segment (phase 13 `feat.segments`)).
2. Events per group: alerts created in the period (`alert.alerts`) whose `party_id` is in the group; cases opened (`casework.cases.opened_at`); STR and TFR decisions (`casework.decisions.type`, decided in the period); freezes (`casework.freezes`, phase 20) started in the period.
3. Rates: events per 1,000 customers of the group (`metric` in `alert_rate`, `case_rate`, `str_rate`, `hold_rate`). The reference is the rate of all customers. `ratio = value / reference_value` (NULL when the reference is zero).
4. Suppression: when the group's customers or events are below `report.small_number_threshold` (phase 21, never below 3), the row is written with `suppressed = true`, `status = 'suppressed'`, and value, ratio and both counts NULL. Zero stays zero.
5. Review trigger (GV-38): when `ratio >= fairness.review_ratio` or `ratio <= 1 / fairness.review_ratio`, `status = 'review'` and a register row: a finding owned by the attribute's owner (`admin.protected_attributes.owner_id`) when one is set, otherwise a warning of source `fairness` (both keyed `fairness:<period>:<metric>:<group_kind>:<group_ref>`). The row's `register_id` is set in the same transaction.
6. The rows are inserted once per `(period, metric, group_kind, group_ref)`; a second run of the period inserts nothing (`ON CONFLICT DO NOTHING`), so a computed metric is never edited (the guard of section 1.4).
7. Audit `fairness.computed {period, rows, suppressed, review}`.

**Resolution of a review.** `POST /fairness/metrics/{id}/review` (route 14; `fairness.review`, roles M and MRO): `justified` (the difference has a documented reason) or `changed` (a change request was raised; its id is in the register body). Either needs a register row of kind finding, written in the same call; the metric row gets `status` and `register_id` (the only columns it updates). Audit `fairness.reviewed {id, outcome}`.

**Use of the attributes (the rule of GV-36, where it runs):** nationality, residence and geography are measured here and never used as a trigger by this phase. The proxy scan of the monthly run is the check; the scenario gate of phase 16 is the preventive check (Open doubts 13). Neither feeds a score: scores are P2.

**Tipping-off.** Metrics are aggregates over groups of at least the threshold; no row names a customer, a case or an alert id. `fairness.read` is M, CH, IA and MRO.

### 3.6 Validation packs (`ValidationPackService`; GV-09, GV-15, GV-32; route 16 to 18)

**`issue(ctx, inventory_item_id, period_from, period_to, idem)`** (`validation.issue`; MRO only; scope `model.risk`):
1. Lock order 1: `SELECT ... FROM admin.inventory_items WHERE id = $1 FOR UPDATE`. A `retired` item is 409 `E_NOT_ALLOWED_IN_STATE` with sub-code `item_retired`.
2. Cedar `validation.issue` on `Akr::ValidationPack { owner: item.owner_id, validator: caller }`. `resource.owner == principal` is refused (`validation.no_owner`): 403 `E_SELF_APPROVAL`. The validator is never the owner (GV-32; the phase 04 CHECK says the same).
3. Build the parts. Each part carries `data_as_of`, the source ids it read and counts or metrics only:
   - P1 conceptual soundness: the rule's version row (`scn.scenario_versions`, logic version, parameter set ids), the tier basis of the item (`admin.inventory_items.tier_basis`) and its `user_roles`. Documented, never free text.
   - P2 data: open `admin.dq_issues` and open `admin.drift_events` for the source tables the rule reads; reconciliation status of the period (`admin.recon_results`, phase 12).
   - P3 implementation: the replay results (`scn.replays`, `scn.replay_results`) and seeded tests (`scn.seeded_tests`) of the version, with pass and fail counts; the distinct logic versions on the rule's alerts in the period (`alert.alerts.logic_version`).
   - P4 outcomes: alerts, cases, decisions (STR, UTR, TFR) and dispositions of the period for the rule (counts and rates; no names).
   - P5 sensitivity: the parameter-variant replays of phase 16 for the version; `not_run` when none exists.
   - P6 ATL and BTL: QA samples and reviews of the rule's alerts (`casework.qa_samples`, `casework.qa_reviews`, phase 19): pass and fail counts; the below-the-line sample is the QA stratum, not a new sample (GV-13 is not built here).
   - P7 benchmark: the shadow results of the version against its predecessor (`scn.shadow_results`, phase 16).
   - GV-15 per-bank delta, carried as the eighth key `bank_delta`: the mapping versions in use (`alert.alerts.mapping_versions`), the threshold values against the shipped values, and the feedback settings of the item.
4. Freshness: a part whose evidence is older than `validation.evidence_max_days` makes the request 422 `E_EVIDENCE_STALE` with the part names in `errors[]`.
5. Verdict, computed by a pure function and never set by a person: `fail` when P3 shows the version does not beat its predecessor on the replay or any part is marked `failed` by its rule; `pass_with_findings` when an open register finding names the item; otherwise `pass`.
6. Canonical JSON of the whole pack → `EvidenceStore::put_generated` with `owner_kind = validation_pack` (registered by this phase as an evidence owner kind), `kind = snapshot`, retention R11, classification `internal`.
7. Insert `admin.validation_packs` with `version` = (the item's last version + 1), `inventory_version` = the item's version, `owner_id` and `validator_id`. Idempotent by `Idempotency-Key`.
8. When the verdict is `pass`, `InventoryService::update` (phase 04) sets `last_validated_at = now()` and `next_validation_due = today + review.rules_months` (Open doubts 9). The status stays as it was: approval for use is a separate step-up act of the validator's peer (phase 04 `PATCH /inventory/{id}`).
9. Audit `validation.issued {inventory_item_id, version, verdict, parts}` (part names only).

**Reads.** `GET /inventory/{id}/validation-packs` and `GET /validation-packs/{id}` (`validation.read`; MRO, IA, M) return the parts and the evidence hash. A pack is never edited: a rebuild is the next version (DR-223).

### 3.7 The command centre (`CommandCentreService::tiles`; G1; route 19)

Every tile has a key, a definition in the help text, a formula, `data_as_of` (the newest instant the tile read), the route it opens and its visibility. Counts pass through the source's own visibility predicate: the phase 17 predicate (`services::visibility::build`) for alerts and cases (generated from the Cedar permits of phase 17 section 3.17), the phase 19 predicate for RFIs and QA, the role predicates for decisions and reports. The predicate is a bound parameter (section 2), never text.

| Tile | Definition and formula | Data as of | Opens | Visible to |
|-|-|-|-|-|
| Decisions waiting for me | open or claimed `admin.tasks` whose step holders include the caller (phase 04 `Holders`) | now | `/inbox` | every dashboard role, own tasks only |
| Late and at-risk clocks | live `casework.alarms` by kind (phases 19 to 22) and clock rows overdue (3.8) | alarm raised instant | `/dashboard/clocks` | M, CH, IA (suspicion subjects: M) |
| Verification | status of the latest `admin.verify_runs` and days since the last `pass` covering the whole chain | run `finished_at` | `/audit` | M, CH, IA: status only; detail IA and SEC |
| Data and coverage incidents | open `admin.dq_issues` by severity; open `admin.drift_events` (phase 10); latest coverage certificates not `attested` (phase 12 status: `issued`, `pending_attestation`, `attested`) | source timestamp | `/data/quality`, `/data/recon` | every dashboard role (source level) |
| Arrivals against throughput | alerts created in the period against alerts dispositioned in the period (phase 17 `alert_events`) | now | `/queue` for the period | predicate of the caller |
| Backlog by elapsed age | open cases by opened-at band: 0 to 7, 8 to 30, 31 to 90, over 90 days | now | `/cases` by age | predicate of the caller |
| Productive alerts | alerts whose case has an STR, UTR or TFR decision in the period, over alerts in the period (the definition of S-53 is Open doubts 11) | decided instant | `/scenarios` | M |
| UTRs and STRs | decisions of the period by type (phase 18) | decided instant | `/reports` | M |
| Top scenarios with cases | the ten scenario codes with the most open cases (`casework.case_alerts`, `alert.alerts.scenario_code`) | now | `/scenarios/{code}` | predicate of the caller |
| QA status | open QA samples, reviews not completed, findings by severity (phase 19) | now | `/qa` | QA, M, CH, IA |
| Regulatory deadlines | reports overdue (phase 21 alarm `report_filing_overdue`), filings due in seven days, liaison reports still draft after `due_on` (phase 22); counts with the words of the deadline | now | `/reports`, `/liaison` | M, CH, IA (counts) |
| Branch liaison reports | signed or reviewed reports of the period over branches with a liaison (phase 22) | period end | `/liaison` | M, CH, IA |
| Governance open items | findings open by severity and overdue; warnings open; fairness reviews open (3.4, 3.5) | now | `/registers` | M, CH, IA (MRO reads the registers themselves, not the tile) |
| Tipping-off summary | denied and conflicting reads of suspicion records this month and break-glass uses (3.2) | now | `/access` | M and SEC only; the tile is not sent to CH or IA |

**Rules.** A count from 1 to `report.small_number_threshold` minus 1 is returned as `value: null, suppressed: true`; zero stays zero. No tile returns a case id, a name, narrative text or an amount. Responses are `Cache-Control: no-store`: the tiles are live. The capacity ratio of the mockup is not shown (P2, Open doubts 15).

### 3.8 The clocks board (`ClockBoardService::list`; route 20)

Rows come from the clocks of the phase 09 `ClockRegistry` and from the due columns of the sources that run them. Each row: `clock` (the registry key), `fixed` (the registry flag), `subject_kind`, `subject_id`, `due_at`, `state` (`overdue` when `due_at < now`, `at_risk` when due within `command.clock_horizon_days`, `ok` otherwise), the source table. Open rows only, keyset `(due_at, id)`.

| Clock (registry key) | Source column (owning phase) |
|-|-|
| `rfi.response` | `casework.rfis.deadline_at` (19) |
| `ops.task.due` | `casework.ops_tasks.due_at` (20) |
| `case.review` | `casework.cases.opened_at` plus the clock days, open and in-review cases (18, DR-384) |
| `refrain.seven_working_days` (fixed) | `casework.refrain_instructions.refrain_due_at` (20) |
| `freeze.task` / `freeze.committee_report` / `freeze.second_report` | `casework.freezes.freeze_due_at`, `committee_report_due_at`, `second_report_due_at` (20) |
| `internal_report.ack` | `casework.internal_reports.ack_due_at` (19) |
| `authority_request.deadline` | `casework.authority_requests.deadline_at` (19) |
| `report.task` | `rpt.report_tasks.due_at` (21) |
| `data.issue` | `admin.dq_issues.due_at` (12) |
| `change.review` | `admin.change_requests.review_due_at` where pending (4) |
| `rule.validation` | `admin.inventory_items.next_validation_due` (4) |
| `exclusion.review` | `admin.exclusions.review_due` (12) |
| `liaison.report` | `casework.liaison_monthly_reports.due_on` where draft (22) |
| `finding.due` | `admin.registers.due_on` (3.4) |
| `workflow.task` | `admin.tasks.due_at` (4) |

Visibility: a row tied to a case, an alert, an RFI, an internal report, a freeze, a refrain or an authority request passes the source's own predicate (phase 17 `services::visibility::build`, phase 19 and phase 20 predicates), never the tier alone; for CH and IA that admits normal-tier subjects and what their own grants admit; internal reports and suspicion subjects are M's (row and count). Cedar `clock.read` for M, CH and IA. The board only lists; it changes no clock (a clock moves only through its owner).

### 3.9 The assessor workspace (`AssessorService`; G8; M-62, UX-50, S-56, RP-64)

**The grant.** The assessor is a person with role `AS` in `admin.role_grants` with a `valid_to` no more than `access.assessor_max_days` (phase 04's key, phase 04 section 7.10) after `valid_from` (set by ADM through phase 01 and phase 04 as for any role). The time box is the grant's own `valid_to`: when it passes, the role is gone and Cedar refuses (403 `E_SCOPE`). No case access list entry is created (see the decision of DR-226: `casework.case_acl` has no assessor reason).

**`d2_matrix(ctx)`** (`assessor.d2`; AS and IA; route 25): reads the signed data file `assessor/d2_map.v1.json` shipped in the release bundle (its SHA-256 in the manifest, checked at start-up like the phase 06 pack files). Each row: `clause` (for example `D2.1(c)(1)`), `topic` (Arabic and English), `status` (`covered`, `partial`, `text_pending`), `evidence` (routes and screens), and live counts from the same sources the screens use (for `D2.1(c)(1)`, the number of retention assignments of phase 23 against the number of tables). Rows in the first version are the clauses the research set cites: D2.1(a)(2) programme monitoring and reporting, D2.1(b)(4) investigations by qualified staff, D2.1(c)(1) record keeping, and D2.1(a)(3), D2.1(b)(5), D2.1(b)(6) and D2.2(b)(5) as `text_pending` (Open doubts 6). The matrix is data, not code (S-56: kept as data). Audit `assessor.matrix_viewed` (the view of a matrix row is also an access event, purpose `supervisory_inspection`).

**`draw(ctx, frame, from, to, sample_size, idem)`** (`assessor.sample`; AS only; route 26):
1. Caller holds AS. `sample_size` between 1 and `assessor.sample_max_items`; `frame` one of `alert_register`, `report_register`; the window within `audit.search_max_window_days`.
2. Frame: `alert_register` is the ids of `alert.alerts` created in the window; `report_register` the ids of `rpt.reports` dated in the window. All tiers are in the frame: the time-boxed grant admits the suspicion tier for this act, and every item later viewed is logged (DR-226). An empty frame is 422 `E_VALIDATION` `frame_empty`.
3. Sort the frame ids ascending (16 bytes each). `frame_hash` = SHA-256 over the sorted ids.
4. `seed` = 32 bytes from the operating system random source (`getrandom`) on akr-api.
5. Rank: for each id, `rank = SHA-256(seed || id)` read as a big-endian 256-bit integer; sort ascending by rank; take `sample_size` items. The algorithm name `sha256_rank_v1` is stored with the row.
6. Insert `admin.assessor_samples` with `grant_id` (the grant in force) and `items` (`[{id, rank}]`). Audit `assessor.sample_drawn {sample_id, frame, frame_size, sample_size, frame_hash8}`: the seed is in the row, not in the event.
7. A repeat with the same `Idempotency-Key` returns the same sample (not a new draw).

**Reproduction.** Anyone holding the frame ids can re-derive the sample from `seed` and `frame_hash` with the algorithm above; the test in section 6 does this in SQL and in Rust. The `GET /assessor/samples/{id}` route returns the seed, the frame hash and the items (route 27, `assessor.sample`).

**`trace(ctx, alert_id)`** (`assessor.trace`; AS, and IA for a record that is not suspicion-tier; route 28). AS traces an alert only when it is normal tier or an item of a sample drawn under the caller's grant in force (`admin.assessor_samples.grant_id`; for a `report_register` sample, an alert of a sampled report's case); any other alert is the masked 404 (lead review: the designed exception covers the recorded sample, not the alert register). The trace has six nodes, each read once and each logged as an access event (purpose `supervisory_inspection`):
1. Alert: `alert.alerts` row (id, `scenario_code`, `created_at`, `reason_codes`, `severity`, the count and the ids of `matched_txn_ids`, `party_id` as a pseudonymous id).
2. Transactions: `canon.txn` rows of `matched_txn_ids`, with `booking_date` (the key of the table), `amount` and `dr_cr` (financial, shown to AS under the grant), and the lineage of each row: `source_id`, `source_table`, `source_pk`, `batch_id` (the CBS row the figure came from).
3. Case: `casework.case_alerts` to `casework.cases`: `case_ref`, `status`, `outcome`, `opened_at`, `closed_at`, `tier`.
4. Decisions of the case: `casework.decisions` (`type`, `decided_at`, `reason_codes`, `decider_id`); no text (`text_enc` is never selected).
5. Reports of the case: `rpt.reports` (`report_ref`, `type`, `status`, filing dates); no body.
6. Evidence: `admin.evidence_objects` of the case's snapshot evidence (`sha256`, `created_at`); no bytes.
For IA the trace is refused with 404 when the alert is suspicion-tier (Cedar `assessor.trace`, masked kind `audit_target`). The trace states nothing about narrative, names or the customer's account beyond the ids above. The figure-to-snapshot link of RP-64 (every figure of a report to its snapshot rows) is not built (Open doubts 11).

**Tipping-off.** AS learns, for a suspicion alert it samples or traces, the decision type, the report type and the dates of the case. That is the one designed exception to "nothing" in this phase: it is inside a time-boxed, read-only grant that the bank approves, every node is logged, and no narrative is returned (Open doubts 1).

### 3.10 The training register (`TrainingRegisterService`; G9; M-51)

- **`record(ctx, body, idem)`** (`training.record`; ADM; route 30): the caller is not the trainee (403 `E_SELF_APPROVAL`; Cedar `training.no_self`); the trainee is an active user (422 `E_VALIDATION`, sub-code `user_not_active`); `course_code` matches the pattern of section 1.2; `completed_on` is not in the future; `evidence_id` (a staged certificate, owner kind `training_record`) or `evidence_ref` (the certificate number) is given. Insert; audit `training.recorded {training_id, user_id, course_code}`.
- **`void(ctx, id, reason_code, idem)`** (`training.void`; ADM; route 31): sets `voided_by`, `voided_at` and `void_reason_code` only (the guard of section 1.4). A correction is a new record and a void of the old one. Audit `training.voided {training_id, reason_code}`.
- **`list(ctx, filter, cursor)`** (`training.read`; M, CH, IA; route 29): the latest valid record per user and course, the records expiring within 60 days, filters by user and course. Names come from `admin.users`.
- Who records: ADM (scope `admin.users`, as API.md 3.16). M, CH and line managers do not record in this phase (Open doubts 7).

### 3.11 Packs (`PackService` of phase 02, kinds added here; R4; GV-35, UX-49, M-63, S-02)

**Kinds added** (routes 21 to 23 of phase 06; the same `POST /packs` accepts them; the `pack_build` job of phase 06 builds them, section 4.1). Every pack is rendered in Arabic and English through `RenderService` (phase 02), its SHA-256 sits in the audit chain (`pack.created`), and it is immutable (a rebuild is the next version of the same scope and language).

| Kind | Scope | Built from | Cedar create | Cedar read and download |
|-|-|-|-|-|
| `board` | `{period}` | the FINAL `rpt.periodic_reports` row of kind `board` for the period (phase 21; 409 `E_NOT_ALLOWED_IN_STATE` `periodic_not_final` otherwise), plus aggregates of 3.4 to 3.7 | M | M, CH, IA |
| `case` | `{case_id}` | the case read model of phase 17's route `GET /cases/{id}` with the fields phase 18 adds (`CaseReadService::header`; no second query) | M | M |
| `report` | `{report_id}` | `rpt.report_versions`, `rpt.report_packages` hashes, `rpt.filings`, decision reason codes, the instance approvals (phase 04 `admin.instance_events`), the snapshot hash (phase 21) | M | M |
| `assessor` | `{sample_id}` | the sample record (frame, seed, frame hash, items as ids and ranks), the D2 matrix status, the authority matrix version and its content hash (phase 09 `admin.authority_matrix`, the matrix printed as M-20 asks), the latest coverage certificates (phase 12), the phase 06 gap report pack ids (S-02) | AS | AS, IA |
| `certification` | `{year}` | coverage certificates of the year (phase 12), validation packs issued (ids, verdicts, hashes), findings opened and closed by severity, fairness reviews, verify runs by status, D2 matrix version, retention runs of phase 23 (counts) (GV-35) | M | M, CH, IA |

**Board pack rule.** The pack payload holds the periodic facts (already aggregated and suppressed by phase 21) and the aggregates above. The builder refuses (`pack.build_failed {reason: case_reference}`) any payload value equal to a `casework.cases.id` or `rpt.reports.id`: a test runs this on every build. A board pack holds no case id, no name and no narrative (DR-227).

**Assessor pack rule.** Sample items appear as ids and ranks. A suspicion-tier item is never given its narrative; the decision type of section 3.9 is not in the assessor pack (only the trace shows it, under the grant).

**Pack versions and download.** `version` per `(kind, scope_key, lang)` (phase 02 `next_version`); `GET /packs/{id}/download` uses step-up class `export` (Open doubts 5) and records the access event `download` (phase 03) before `EvidenceStore::open`.

**`POST /packs/verify`** (route 24; `pack.verify`; scope `self`, any active user): input `sha256` (hex) or `pack_id`.
1. On `readonly_pool`: find the `admin.packs` row by its `sha256` or id; compare `admin.evidence_objects.sha256` (the vault) with the pack's `sha256`.
2. On `readonly_pool`: find the audit row `pack.created` whose `details.sha256` equals the hash (phase 02 catalogue); `anchored` is true when an anchor covers that row (`audit.audit_anchors` range).
3. Answer `{verified, anchored}`. The `kind`, `scope` and `version` are returned only when the caller holds `pack.read` for that kind.
4. The answer is the same for an unknown hash and for a pack the caller cannot read: `verified: false` (Open doubts 26). The check writes `pack.verified {result}` without the pack id unless the pack was readable.

### 3.12 Cedar: actions, roles and the two files (`60_audit.cedar`, `C0_governance.cedar`)

**Actions and roles** (the fine gate; scopes of section 2 are the coarse gate):

| Action | Resource | Roles |
|-|-|-|
| `audit.read_events`, `audit.chain_status`, `audit.verify` (pinned by phase 02) | `System` | IA, SEC |
| `audit.access_history` | `AuditTarget` | M, SEC, IA; suspicion tier: M only |
| `access.control_report` | `System` | M, SEC |
| `register.read`, `fairness.read`, `command.read`, `clock.read`, `training.read` | `System`, `Register`, `TrainingRecord` | M, CH, IA |
| `register.write` | `Register` (finding) | M, CH, MRO |
| `register.close` | `Register` | M, and the owner of the row |
| `fairness.review` | `FairnessMetric` | M, MRO |
| `validation.issue` | `ValidationPack` | MRO, not the owner |
| `validation.read` | `ValidationPack` | MRO, IA, M |
| `pack.create`, `pack.read`, `pack.download` (phase 06 actions) | `Pack` by kind | section 3.11 |
| `pack.verify` | `System` | any active principal |
| `assessor.d2` | `System` | AS, IA |
| `assessor.sample` | `Sample` | AS |
| `assessor.trace` | `AuditTarget` | AS for a normal-tier alert or an item of a sample drawn under the caller's grant; IA for a non-suspicion alert |
| `training.record`, `training.void` | `TrainingRecord` | ADM, not the trainee |

**Schema additions** (appended to `schema.cedarschema` in the phase 03 form; `User`, `System`, `Pack` and `Ctx` are phase 03 and phase 06):

```cedar
entity AuditTarget = { kind: String, tier: String, in_own_sample: Bool };   // in_own_sample: an item of a sample drawn under the caller's grant in force
entity Register = { kind: String, owner?: User, created_by?: User };
entity ValidationPack = { owner: User, validator: User };
entity Sample = { drawn_by: User };
entity TrainingRecord = { user: User, recorded_by: User };
```

**`60_audit.cedar`** (copy-paste ready; one role per policy and per `@role` tag, no `containsAny`; lead review):

```cedar
// 60_audit.cedar (phase 24). Audit viewer, chain status, verify runs, access history and the control report.
// One role per policy (conventions); the handler answers the masked 404 where a forbid fires on a suspicion record.

@id("audit.viewer.ia") @role("IA")
permit (principal, action in [Akr::Action::"audit.read_events", Akr::Action::"audit.chain_status",
                              Akr::Action::"audit.verify"], resource is Akr::System)
when { principal.roles.contains("IA") };

@id("audit.viewer.sec") @role("SEC")
permit (principal, action in [Akr::Action::"audit.read_events", Akr::Action::"audit.chain_status",
                              Akr::Action::"audit.verify"], resource is Akr::System)
when { principal.roles.contains("SEC") };

@id("audit.history.m") @role("M")
permit (principal, action == Akr::Action::"audit.access_history", resource is Akr::AuditTarget)
when { principal.roles.contains("M") };

@id("audit.history.sec") @role("SEC")
permit (principal, action == Akr::Action::"audit.access_history", resource is Akr::AuditTarget)
when { principal.roles.contains("SEC") };

@id("audit.history.ia") @role("IA")
permit (principal, action == Akr::Action::"audit.access_history", resource is Akr::AuditTarget)
when { principal.roles.contains("IA") };

// Suspicion-tier records: the access history is for the MLRO only (DR-219). The handler answers 404 (masked).
@id("audit.history.suspicion")
forbid (principal, action == Akr::Action::"audit.access_history", resource is Akr::AuditTarget)
when { resource.tier == "suspicion" && !principal.roles.contains("M") };

@id("control.report.m") @role("M")
permit (principal, action == Akr::Action::"access.control_report", resource is Akr::System)
when { principal.roles.contains("M") };

@id("control.report.sec") @role("SEC")
permit (principal, action == Akr::Action::"access.control_report", resource is Akr::System)
when { principal.roles.contains("SEC") };
```

**`C0_governance.cedar`** (copy-paste ready):

```cedar
// C0_governance.cedar (phase 24). Registers, fairness, validation, command centre, clocks, packs of this phase,
// assessor and training. Phase 06's C0_packs.cedar keeps cbi_gap, vendor_request and integration_profile.
// One role per policy (conventions).

@id("gov.read.m") @role("M")
permit (principal, action in [Akr::Action::"register.read", Akr::Action::"fairness.read",
                              Akr::Action::"command.read", Akr::Action::"clock.read",
                              Akr::Action::"training.read"], resource)
when { principal.roles.contains("M") };

@id("gov.read.ch") @role("CH")
permit (principal, action in [Akr::Action::"register.read", Akr::Action::"fairness.read",
                              Akr::Action::"command.read", Akr::Action::"clock.read",
                              Akr::Action::"training.read"], resource)
when { principal.roles.contains("CH") };

@id("gov.read.ia") @role("IA")
permit (principal, action in [Akr::Action::"register.read", Akr::Action::"fairness.read",
                              Akr::Action::"command.read", Akr::Action::"clock.read",
                              Akr::Action::"training.read"], resource)
when { principal.roles.contains("IA") };

@id("register.write.findings.m") @role("M")
permit (principal, action == Akr::Action::"register.write", resource is Akr::Register)
when { principal.roles.contains("M") && resource.kind == "finding" };

@id("register.write.findings.ch") @role("CH")
permit (principal, action == Akr::Action::"register.write", resource is Akr::Register)
when { principal.roles.contains("CH") && resource.kind == "finding" };

@id("register.write.findings.mro") @role("MRO")
permit (principal, action == Akr::Action::"register.write", resource is Akr::Register)
when { principal.roles.contains("MRO") && resource.kind == "finding" };

@id("register.close.m") @role("M")
permit (principal, action == Akr::Action::"register.close", resource is Akr::Register)
when { principal.roles.contains("M") };

@id("register.close.owner")
permit (principal, action == Akr::Action::"register.close", resource is Akr::Register)
when { resource has owner && resource.owner == principal };

@id("fairness.review.m") @role("M")
permit (principal, action == Akr::Action::"fairness.review", resource)
when { principal.roles.contains("M") };

@id("fairness.review.mro") @role("MRO")
permit (principal, action == Akr::Action::"fairness.review", resource)
when { principal.roles.contains("MRO") };

@id("validation.issue.mro") @role("MRO")
permit (principal, action == Akr::Action::"validation.issue", resource is Akr::ValidationPack)
when { principal.roles.contains("MRO") };

// GV-32: the validator is never the owner of the item.
@id("validation.no_owner")
forbid (principal, action == Akr::Action::"validation.issue", resource is Akr::ValidationPack)
when { resource.owner == principal };

@id("validation.read.mro") @role("MRO")
permit (principal, action == Akr::Action::"validation.read", resource is Akr::ValidationPack)
when { principal.roles.contains("MRO") };

@id("validation.read.ia") @role("IA")
permit (principal, action == Akr::Action::"validation.read", resource is Akr::ValidationPack)
when { principal.roles.contains("IA") };

@id("validation.read.m") @role("M")
permit (principal, action == Akr::Action::"validation.read", resource is Akr::ValidationPack)
when { principal.roles.contains("M") };

@id("pack.board.create.m") @role("M")
permit (principal, action == Akr::Action::"pack.create", resource)
when { principal.roles.contains("M") && context has pack_kind && context.pack_kind == "board" };

@id("pack.board.read.m") @role("M")
permit (principal, action in [Akr::Action::"pack.read", Akr::Action::"pack.download"], resource is Akr::Pack)
when { principal.roles.contains("M") && resource.kind == "board" };

@id("pack.board.read.ch") @role("CH")
permit (principal, action in [Akr::Action::"pack.read", Akr::Action::"pack.download"], resource is Akr::Pack)
when { principal.roles.contains("CH") && resource.kind == "board" };

@id("pack.board.read.ia") @role("IA")
permit (principal, action in [Akr::Action::"pack.read", Akr::Action::"pack.download"], resource is Akr::Pack)
when { principal.roles.contains("IA") && resource.kind == "board" };

@id("pack.case.m") @role("M")
permit (principal, action in [Akr::Action::"pack.create", Akr::Action::"pack.read", Akr::Action::"pack.download"], resource)
when { principal.roles.contains("M") && ((resource is Akr::Pack && resource.kind == "case") || (context has pack_kind && context.pack_kind == "case")) };

@id("pack.report.m") @role("M")
permit (principal, action in [Akr::Action::"pack.create", Akr::Action::"pack.read", Akr::Action::"pack.download"], resource)
when { principal.roles.contains("M") && ((resource is Akr::Pack && resource.kind == "report") || (context has pack_kind && context.pack_kind == "report")) };

@id("pack.assessor.as") @role("AS")
permit (principal, action in [Akr::Action::"pack.create", Akr::Action::"pack.read", Akr::Action::"pack.download"], resource)
when { principal.roles.contains("AS") && ((resource is Akr::Pack && resource.kind == "assessor") || (context has pack_kind && context.pack_kind == "assessor")) };

@id("pack.assessor.read.ia") @role("IA")
permit (principal, action in [Akr::Action::"pack.read", Akr::Action::"pack.download"], resource is Akr::Pack)
when { principal.roles.contains("IA") && resource.kind == "assessor" };

@id("pack.certification.read.m") @role("M")
permit (principal, action in [Akr::Action::"pack.read", Akr::Action::"pack.download"], resource is Akr::Pack)
when { principal.roles.contains("M") && resource.kind == "certification" };

@id("pack.certification.read.ch") @role("CH")
permit (principal, action in [Akr::Action::"pack.read", Akr::Action::"pack.download"], resource is Akr::Pack)
when { principal.roles.contains("CH") && resource.kind == "certification" };

@id("pack.certification.read.ia") @role("IA")
permit (principal, action in [Akr::Action::"pack.read", Akr::Action::"pack.download"], resource is Akr::Pack)
when { principal.roles.contains("IA") && resource.kind == "certification" };

@id("pack.certification.create.m") @role("M")
permit (principal, action == Akr::Action::"pack.create", resource)
when { principal.roles.contains("M") && context has pack_kind && context.pack_kind == "certification" };

@id("pack.verify")
permit (principal, action == Akr::Action::"pack.verify", resource is Akr::System)
when { principal.status == "active" };

@id("assessor.d2.as") @role("AS")
permit (principal, action == Akr::Action::"assessor.d2", resource is Akr::System)
when { principal.roles.contains("AS") };

@id("assessor.d2.ia") @role("IA")
permit (principal, action == Akr::Action::"assessor.d2", resource is Akr::System)
when { principal.roles.contains("IA") };

@id("assessor.sample.as") @role("AS")
permit (principal, action == Akr::Action::"assessor.sample", resource is Akr::Sample)
when { principal.roles.contains("AS") };

@id("assessor.trace.as") @role("AS")
permit (principal, action == Akr::Action::"assessor.trace", resource is Akr::AuditTarget)
when { principal.roles.contains("AS") && (resource.tier != "suspicion" || resource.in_own_sample) };

@id("assessor.trace.ia") @role("IA")
permit (principal, action == Akr::Action::"assessor.trace", resource is Akr::AuditTarget)
when { principal.roles.contains("IA") && resource.tier != "suspicion" };

@id("training.record.adm") @role("ADM")
permit (principal, action in [Akr::Action::"training.record", Akr::Action::"training.void"], resource is Akr::TrainingRecord)
when { principal.roles.contains("ADM") };

@id("training.no_self")
forbid (principal, action in [Akr::Action::"training.record", Akr::Action::"training.void"], resource is Akr::TrainingRecord)
when { resource.user == principal };
```

**Notes on the files.** The policy `pack.verify` has no `@role` tag: every active principal may hold it. Phase 03's `baseline.inactive` forbid already refuses inactive principals. Context attribute `pack_kind` is phase 06's. The policy `pack.assessor.read.ia` lets IA read and download an assessor pack, not create one; `context.pack_kind` is always read behind `context has pack_kind`.

### 3.13 Errors (`GovernanceError`, `thiserror`; API.md section 6 codes only)

| Variant | HTTP and code | Sub-code in `errors[]` (where used) |
|-|-|-|
| `Forbidden` | 403 `E_FORBIDDEN` | none (Cedar deny on a resource the caller may know exists, for example a board pack for CH on the create action) |
| `ScopeMissing` | 403 `E_SCOPE` | none (the route's scope gate; the assessor after the grant's `valid_to`) |
| `SelfApproval` | 403 `E_SELF_APPROVAL` | `validator_is_owner`, `trainee_is_recorder`, `owner_accepts_risk` |
| `NotFound` | 404 `E_NOT_FOUND` | masked suspicion record or pack (identical body to a missing id) |
| `PurposeRequired` | 422 `E_PURPOSE_REQUIRED` | none (audit search without a purpose code) |
| `Validation` | 422 `E_VALIDATION` | `window_too_long`, `reason_required`, `kind_not_writable`, `forbidden_term`, `frame_empty`, `user_not_active`, `unknown_pack_kind`, `sample_too_large`, `title_not_bilingual`, `severity_not_allowed`, `subject_key_required`, `owner_not_governance`, `unknown_register_kind` (a `kind` of route 9 outside the five), `explanation_code_not_allowed`, `note_too_long` |
| `EvidenceStale` | 422 `E_EVIDENCE_STALE` | names of the stale parts |
| `CostGuard` | 422 `E_COST_GUARD` | none (estimate in the body) |
| `IdempotencyReuse` | 422 `E_IDEMPOTENCY_REUSE` | none |
| `StepUpRequired` | 401 `E_STEPUP_REQUIRED` | none (missing, reused or wrong-class assertion: `export`, `approval`) |
| `PreconditionRequired` | 428 `E_PRECONDITION_REQUIRED` | none (`PATCH` of a register row without `If-Match`) |
| `VersionConflict` | 409 `E_VERSION_CONFLICT` | none (current version in the body) |
| `NotAllowedInState` | 409 `E_NOT_ALLOWED_IN_STATE` | `transition_not_allowed`, `register_closed`, `item_retired`, `periodic_not_final` |
| `IdempotencyInFlight` | 409 `E_IDEMPOTENCY_IN_FLIGHT` | none |
| `Unavailable` | 503 `E_UNAVAILABLE` | none (read-only pool or audit writer down; the viewer returns nothing) |
| `Internal` | 500 `E_INTERNAL` | none (correlation id only) |

Every mutation error writes no audit row (the transaction rolls back); the refused read of a masked resource writes its access event (`decision = deny`, phase 03).

### 3.14 Audit events emitted

| Event kind | On | Resource (key) |
|-|-|-|
| `audit.searched` | a viewer search (filters by name, purpose, result count, has_more) (SP-42) | `audit_log` (key) |
| `audit.event_read` | one event read (event id, stream) | `audit_log` (key) |
| `audit.verify_requested` | a verify run queued (run id, scope) | `verify_run` (key) |
| `access.control_report_generated` | a control report generated (period) | `access_control` (key) |
| `access.history_read` | the access history of a record read (kind, count); the row is also an access event | record (key) |
| `register.created` | a finding or warning created (id, kind, source, severity, repeat flag) | `register` (key) |
| `register.updated` | a register row changed (fields, status from and to, step-up id) | `register` (key) |
| `register.closed` | a register row closed (closure code) | `register` (key) |
| `fairness.computed` | a period computed (period, rows, suppressed, review) | `fairness_period` (key) |
| `fairness.reviewed` | a review resolved (metric id, outcome) | `fairness_metric` (key) |
| `validation.issued` | a validation pack issued (item id, version, verdict, part names) | `inventory_item` (key) |
| `assessor.matrix_viewed` | the D2 matrix read | `assessor` (key) |
| `assessor.sample_drawn` | a sample drawn (sample id, frame, sizes, frame hash prefix) | `assessor_sample` (key) |
| `assessor.trace_viewed` | a trace read (alert id, nodes read) | `audit_target` (key) |
| `training.recorded`, `training.voided` | a training record written or voided (ids, course, reason code) | `training_record` (key) |
| `pack.verified` | a verify request answered (result; pack id only when readable) | `pack` (key) |
| `pack.created`, `pack.requested`, `pack.downloaded`, `pack.build_failed` | the pack lifecycle of phases 02 and 06, now with the five new kinds | `pack` (key) |

Job events: `register.created` from `register_post_review_sync` and `review_calendar_sweep`; `fairness.computed` from `fairness_monthly`; `audit.verify_completed` (phase 02) from `audit_verify_run`. Payloads carry ids, codes, counts and hashes; no name, no narrative, no amount, no tier value.

### 3.15 Tipping-off review (what a person outside a case's access list can learn; the answer must be "nothing")

For each surface, the person is a viewer with no case access list entry. Quoted rows of earlier phases: phase 18 line 951 (audit viewer: "event kinds and ids; the seal hash; no text, no tier value beyond the source case id"), phase 19 line 1617 ("event kinds and ids, counts, the digest of a draw; no text, no name, no tier value"), phase 20 line 821 (IA and SEC: "event kinds, ids and counts; no account, no name, no amount").

| Surface | Who | What the outside person learns | Answer |
|-|-|-|-|
| Audit search (3.1) | IA, SEC | rows of normal and restricted resources with their ids and details; suspicion rows are absent, and a filter by a suspicion id is 404 | nothing about suspicion (the restricted rows are the insider control of research 30 section 9.2) |
| Chain status and anchors (3.1, 3.3) | IA, SEC | volume per partition and stream | volume only, no resource (Open doubts 3) |
| Verify runs (3.3) | IA, SEC | counts and finding kinds; no entry id | nothing about a resource |
| Access history (3.2) | M, SEC, IA | who read a normal or restricted record, when, purpose and decision; for a suspicion record, 404 to IA and SEC | nothing about suspicion for IA and SEC; M sees all (Open doubts 25) |
| Control report (3.2) | M, SEC | counts of denies, conflicts, break-glass, exports, tier raises | counts only; the suspicion-tier denies are a count, no id |
| Registers, decisions (3.4) | M, CH, IA | decision type, reason codes, date, decider id of the cases the phase 17 predicate admits for the caller | nothing about restricted or suspicion cases for CH and IA |
| Registers, exclusions, findings, warnings (3.4) | M, CH, IA, MRO | business reasons; findings carry the subject key of a rule, not a customer | nothing about a customer |
| Fairness (3.5) | M, CH, IA, MRO | group rates; small groups suppressed; no id | nothing about a person |
| Validation pack (3.6) | MRO, IA, M | counts, rates, rule version ids; no customer, no case | nothing about a person |
| Command centre (3.7) | M, CH, IA; the tipping-off tile M and SEC | counts through each source's predicate; small counts suppressed | nothing: suspicion counts are returned to M only |
| Clocks board (3.8) | M, CH, IA | due instants and subject ids of the subjects each source's predicate admits | nothing: restricted and suspicion subjects follow their source's predicate |
| Board pack (3.11) | M, CH, IA | aggregates and periodic facts; no case id (refused at build) | nothing |
| Case pack, report pack (3.11) | M | the case file and the report: the MLRO's own surfaces | not open to CH, IA, SEC, AS |
| Assessor pack and sample (3.9, 3.11) | AS, IA | sample ids and ranks, D2 status, authority matrix version, certificates; no narrative | nothing, except the designed exception of the trace below |
| Assessor trace (3.9) | AS; IA for normal and restricted alerts | the decision type and dates of a suspicion case, for AS only, for an item of a sample AS drew under the time-boxed grant, every node logged | the one exception; approval needed (Open doubts 1) |
| Certification pack (3.11) | M, CH, IA | counts and hashes per year | nothing |
| Training register (3.10) | M, CH, IA | staff course records | staff data, not customer data |
| `POST /packs/verify` (3.11) | any active principal | "verified" or "not verified" for a hash; the kind and scope only when the pack is readable | nothing about a case pack for a person without `pack.read` (Open doubts 26) |

### 3.16 Settings used (phase 24 keys in `0079`; reused keys from other phases)

Phase 24 keys: `audit.search_page_size` (100), `audit.search_max_window_days` (366), `audit.verify_max_rows` (50,000,000), `register.finding_due_days` (90; governed; floor min 30, max 365), `fairness.review_ratio` (1.25; governed; floor max 1.25), `review.rules_months` (12; governed), `validation.evidence_max_days` (180; governed), `assessor.sample_max_items` (200), `command.clock_horizon_days` (14), `training.expiry_months` (24).
Reused: `report.small_number_threshold` (phase 21; governed; never below 3), `bank.timezone` (phase 00), `audit.anchor_lag_alert_s` (phase 02, the anchor lag of 3.1).

### 3.17 Lock order

1. Inventory row (`admin.inventory_items`, `FOR UPDATE`) before any validation pack insert; the phase 04 `InventoryService::update` takes the same row and is called after it, never before.
2. Case row (phase 18 order) before the register rows of that case; no register path takes a case row.
3. `admin.registers` rows: the post-review sync and the sweep take their rows by `source_key` in the unique index, insert only; a status change takes the one row `FOR UPDATE`.
4. `admin.verify_runs` row: taken by the job and by the request, never both at once.
5. Pack builds: the version lock of the scope in `PackService::publish` (phase 02, `next_version` of `(kind, scope_key, lang)`) is the last lock taken; the sources are read before it.
6. Outbox and job claims (phase 00) are taken before any of the above and released at commit.
No path takes a lower-numbered lock after a higher-numbered one.

## 4. Infrastructure Updates

### 4.1 Job kinds (Postgres `jobs`; coalesce key in brackets)

| Kind | Binary | Payload | Concurrency, attempts | Schedule or trigger |
|-|-|-|-|-|
| `audit_verify_run` | worker | `{ run_id }` [`audit_verify_run:<run_id>`] | 1, 2; lease 2 h | enqueued in the request transaction of `POST /audit/verify-runs` (section 3.3); calls phase 02 `Verifier::run` |
| `register_post_review_sync` | worker | `{}` [`register_post_review_sync`] | 1, 3 | `schedule.register_post_review_sync` (every 15 minutes); section 3.4 |
| `review_calendar_sweep` | worker | `{}` [`review_calendar_sweep`] | 1, 3 | `schedule.review_calendar_sweep` (06:00 bank time); opens warnings for rows that turn red (section 3.4) |
| `fairness_monthly` | worker | `{ period }` [`fairness_monthly:<period>`] | 1, 3 | `schedule.fairness_monthly` (day 2, 04:30 bank time); the period is the month before; section 3.5 |
| `pack_build` (phase 06, extended) | worker | `{ kind, scope, lang?, requested_by }` [kind + canonical scope + lang] | 2 (unchanged), 3 | enqueued by `POST /packs` (phase 06 route); builds the five kinds of section 3.11 |

Every handler re-reads its owner first (a periodic board row, a verify run, an instance event). The `audit_verify_run` job reads through `readonly_pool` (phase 02 Verifier); the others use `akr_app`. No job of this phase reaches the core banking system (no connection to a source). The job names do not use the `ingest_` prefix.

### 4.2 Timers and outbox

No timer kind is added. No outbox topic is produced or consumed in this phase: the post-change feed is a scheduled read of `admin.instance_events` (section 3.4, Open doubts 18), so no second handler is registered on any topic (phase 22's rule).

### 4.3 AppState additions

- `akr-api`: `viewer: Arc<AuditViewerService>`, `access_history: Arc<AccessHistoryService>`, `registers: Arc<GovernanceRegisterService>`, `fairness: Arc<FairnessService>`, `validation: Arc<ValidationPackService>`, `command: Arc<CommandCentreService>`, `clocks: Arc<ClockBoardService>`, `assessor: Arc<AssessorService>`, `training: Arc<TrainingRegisterService>`, `d2_map: Arc<D2Map>` (the signed data file of section 3.9), and `readonly_pool: PgPool` (the role `akr_readonly`, KV `db/akr_readonly`, four connections, `statement_timeout` 10 s; used by the viewer, access history, pack verify and the verify-run estimate only).
- `akr-worker`: `verifier: Verifier` (phase 02, existing), `register_sync: Arc<GovernanceRegisterService>`, `review_sweep: Arc<GovernanceRegisterService>`, `fairness: Arc<FairnessService>`, and the five new kinds registered in the phase 02 `PackService` through the phase 06 `pack_build` handler.

### 4.4 Network zones, metrics, readiness, OpenAPI

- Network: `akr-api` reaches PostgreSQL as before. The new `readonly_pool` is a second credential to the same database, not a new network path (Open doubts 4).
- Metrics: `akr_audit_viewer_searches_total{result}`, `akr_audit_viewer_refusals_total{reason}`, `akr_registers_open{kind, severity}` (gauge, from a 60 s sample), `akr_fairness_review_open` (gauge), `akr_assessor_samples_total`. Labels never carry an id, a case or a party.
- Readiness: `akr-api` adds `audit_readonly` (a `SELECT 1` on `readonly_pool` and the `audit.audit_anchors` count), so a missing read-only credential fails readiness rather than the viewer at request time.
- OpenAPI tags: `audit`, `access`, `registers`, `fairness`, `validation`, `command`, `clocks`, `assessor`, `training`; the phase 06 `packs` tag gains the five kinds. The route order puts the literal `/registers/review-calendar` before `/registers/{kind}` so the calendar is not read as a kind.

### 4.5 Existing entity, schema, route and code updates (extensions of earlier phases)

1. **Phase 02** `admin.packs`: the kind CHECK gains `certification` (0078); the rest of the table is unchanged. `PackService::publish` is called with the five kinds; `RenderService` gains five templates (Arabic and English). `EvidenceStore` owner kinds gain `validation_pack` (3.6) and `training_record` (3.10), each with its `EvidenceResolver`. The Verifier is not changed.
2. **Phase 06** `POST /packs`, `GET /packs/{id}` and the download: accept the scope `assessor.read` beside `dashboard.read` and `data.steward` (AS holds no other scope; Cedar keeps AS to kind `assessor`); `POST /packs` accepts the five kinds; the `unknown_pack_kind` refusal of phase 06 section 3.6 narrows to the kinds still unbuilt (correspondent and validator, P2). The `pack_build` job gains the five kinds. `C0_packs.cedar` is unchanged. The step-up class of phase 06's download (`pack_download`) is replaced by `export` (Open doubts 5).
3. **Phase 03** `MASKED_KINDS` gains `audit_event` (probes: `GET /audit/events/{id}`, the filters `case_id` and `resource_id` of `GET /audit/events`) and `audit_target` (probes: `GET /records/{kind}/{id}/access-history`, `GET /assessor/trace/{alert_id}` for IA). Probes compare the body of a suspicion answer with the body of a missing id. `AccessScope` logs the reads of section 3.2. The files `60_audit.cedar` and `C0_governance.cedar` join the bundle manifest.
4. **Phase 00** AppRole policies (the list at line 594 of phase 00): `akr-api` gains KV read `db/akr_readonly`.
5. **Phase 04** `InventoryService::update` is called by section 3.6 step 8; the change target `protected_attribute` is registered with the reason family `change` (section 3.5). `GET /inventory` and the PATCH are unchanged. The post-review feed of phase 04 (lines 945 and 1651) is read by section 3.4.
6. **Phase 09** `PolicyPackService::scan` (scope `reason_text`) scans the text of every register row. `ClockRegistry` is read, not changed. `admin.authority_matrix` is read for the assessor pack.
7. **Phase 12** `admin.exclusions` is read as the exclusions view (3.4); `admin.coverage_certificates` feeds the assessor and certification packs. The phase 12 note that the annual certification pack is phase 24's now holds.
8. **Phase 16** the scenario change target's `check_before_decision` gains the `ProtectedAttributeCheck` (requested; Open doubts 13). The S-46 activation instant is read (Open doubts 30).
9. **Phase 17 and 18** `casework.effective_tier` is called by the SECURITY DEFINER `audit.resource_tier` (0078; no grant change); `casework.decisions` is read as a view (no change to phase 18 tables, routes or sealed-case rules).
10. **Phase 19** its `RegisterService` (internal reports and authority requests) is unchanged; this phase's `GovernanceRegisterService` is a different service.
11. **Phase 21** `rpt.periodic_reports` (kind `board`) and `rpt.report_versions` are read; the R3 routes are unchanged.
12. **Phase 23** the retention rows of this phase's seven tables are added in 0079; `61_retention.cedar` is unchanged.
13. **Phase 01** `admin.users` gains no column; training and reviews reference the user id only.

## 5. Frontend Impact

Screens by catalogue ID (DESIGN-SYSTEM section 8.1, rows G1, G4, G5, G6, G8, G9 and R4) and the Orval hooks named from the paths. The main bundle hosts them (not the branch bundle of phase 22). Every screen is a read of the API; the only writes are the forms of section 5.1 to 5.7 and go through the hooks.

### 5.1 G1 Command centre (`/command`; DASH; mockup 01; UX-47, UX-48, UX-56)

Order as DESIGN-SYSTEM line 780: decisions waiting and clocks at risk first, verification and data incidents next, then arrivals against throughput, QA, the tipping-off summary (M and SEC only), and the regulatory deadlines with words and the liaison reports submitted of total. Each tile has a `?` help with its definition and formula, the `data_as_of` line and a link to its list (section 3.7). Suppressed values read "fewer than 5" in the locale of the user. Hooks: `useGetDashboardCommandCentre` and `useGetDashboardClocks` (the clocks panel links to the board of 5.1a).

- 5.1a `/command/clocks`: the clocks board (3.8), a REGISTER list with the state colours of the design system and the fixed flag shown as a lock.

### 5.2 G4 Registers (`/registers`; REGISTER; UX-49 is the board pack of 5.7)

Tabs: Decisions (read view), Exclusions (read view), Findings, Warnings, Fairness, Review calendar. Hooks: `useGetRegistersKind` (kind in the path: `decisions`, `exclusions`, `findings`, `warnings`, `fairness`), `usePostRegistersKindEntries`, `useGetRegistersKindEntriesId`, `usePatchRegistersKindEntriesId` (sends `If-Match` from the row's version), `useGetRegistersReviewCalendar`, `usePostFairnessMetricsIdReview`, `useGetFairnessAttributes`. Detail of a finding shows its repeat link (`repeat_of`). Closing a finding asks for the closure code; accepting a risk asks for the second factor (step-up `approval`). Validation packs of an inventory item open from the rule's row: `useGetInventoryIdValidationPacks`, `usePostInventoryIdValidationPacks` (MRO), `useGetValidationPacksId`.

### 5.3 G5 Access and insider controls (`/access`; REGISTER)

Two parts. (a) Access history: a record picker (kind and id) that calls `useGetRecordsKindIdAccessHistory`; a suspicion record answers "not found" for IA and SEC, as designed. (b) Control report for the month (`useGetAccessControlReport`, M and SEC): counts only, with the "canary triggers: not built" line shown, not hidden.

### 5.4 G6 Audit viewer and chain verifier (`/audit`; REGISTER; IA and SEC)

A search form with the purpose field first (required; `other` shows the reason box), then the window, filters and the result list (cursor, "more rows", no total). Each search is announced as audited (the `audit.searched` notice). Hooks: `useGetAuditEvents`, `useGetAuditEventsId` (with `occurred_at`), `useGetAuditChainStatus`, `useGetAuditAnchors`, `usePostAuditVerifyRuns` (with `Idempotency-Key`), `useGetAuditVerifyRunsId` (polled every 3 s until a terminal state). The chain panel shows per-partition status; a verify result shows counts and finding kinds only.

### 5.5 G8 Assessor workspace (`/assessor`; DASH; AS, and IA for the matrix)

A banner names the read-only, time-boxed account and its expiry (the grant's `valid_to`). Four panels: the D2 matrix (`useGetAssessorD2Matrix`, rows with status and live evidence links), the sample (`usePostAssessorSamples` with the frame, window and size; the result shows the seed, the frame hash and the items; `useGetAssessorSamplesId` on reload), the trace of an alert (`useGetAssessorTraceAlertId`; six nodes, each shows its access event), and the assessor pack (R4, section 5.7). Suspicion items in a sample show their id and rank; no narrative is ever rendered.

### 5.6 G9 Training register (`/training`; REGISTER; M, CH, IA read; ADM records)

A list by user and course with the expiry column and the "expires within 60 days" filter. Hooks: `useGetTrainingRegister`, `usePostTrainingRegister` (ADM), `usePostTrainingRegisterIdVoid` (ADM). The record form refuses the caller's own user before the request.

### 5.7 R4 Board pack and evidence pack builder (`/packs`; STEPPER)

Steps: choose the kind (board, case, report, assessor, certification; the kinds a user cannot create are not offered), the scope (period, case id, report id, sample id or year), the language, then the build (`usePostPacks`, `Idempotency-Key`), then the status (`useGetPacksId`, polled until built). The download is a link to `GET /api/v1/packs/{id}/download` with the step-up prompt of class `export`. A verify box (`usePostPacksVerify`) accepts a hash or a pack id and shows "verified" or "not verified" with no further detail.

### 5.8 Generated hooks (Orval names from the paths)

Names follow the Orval naming of the conventions (`useGetXxx`, `usePostXxx`, `usePatchXxx`, built from the method and the path segments; the phase 00 pipeline generates them). The literal `review-calendar` and `verify-runs` segments keep their names; path parameters become `Id` or `Kind`. The download is not a hook (a file link). Regenerate with `make generate-api` in the same phase (section 6).

### 5.9 Browser and caching rules

Command centre, clocks, assessor and audit responses are `Cache-Control: no-store`; the client keeps them in React Query memory only (no `localStorage` of counts, no service worker cache). The sample draw and a sent verify request are not replayed on a network error without the same `Idempotency-Key`. Every screen in this phase meets WCAG 2.2 AA (DESIGN-SYSTEM section 9); the state colours always come with a text label.

## 6. Verification

1. `make migrate` applies `0076` to `0079` cleanly, and a second run is a no-op. `\d admin.registers`, `\d admin.validation_packs`, `\d admin.protected_attributes`, `\d admin.fairness_metrics`, `\d admin.training_records`, `\d admin.assessor_samples` and `\d admin.verify_runs` show the columns and CHECKs of section 1. `\d admin.packs` shows `packs_kind_check` with `certification`.
2. `cargo check`: the SQL of the repositories of section 2 compiles against the schema (sqlx offline data refreshed with `cargo sqlx prepare`).
3. `cargo clippy -- -D warnings`.
4. `cargo test`, including: (a) the tier predicate of `services::visibility` for the viewer, `audit.resource_tier` (section 1.3) and the Cedar file `60_audit.cedar` agree over generated principals and rows, one test per resource type of the catalogue; (b) the register state machine (every allowed move, every refused move with its sub-code); (c) the fairness arithmetic: suppression at 2, 4 and 5 with a threshold of 5, zero reference, the ratio bounds `1/1.25` and `1.25`; (d) the sample reproduction: the rank of fixed seed and ids gives the fixed order (test vectors computed once with the SQL of curl check 11; `bytea` order is the big-endian integer order); (e) the board payload refusal: a payload that contains a `casework.cases` id fails the build; (f) the verdict function over the seven parts and `bank_delta`; (g) every audit payload of section 3.14 is serialised against the phase 02 catalogue keys (phase 02 test).
5. Curl suite `tests/curl/phase24_governance.sh` (new file, written in the implementation phase; dev users `<role>@bank.test` as phase 22 lists, plus `as@bank.test`, role `AS`, grant valid for one day (Open doubts 27)). Each check states its expected answer:
   1. `ia@`: `GET /api/v1/audit/events?from=…&to=…&purpose=internal_audit` returns 200 with no row whose resource is a suspicion-tier record; `has_more` present, no total.
   2. `ia@`: the same call with `case_id` of a suspicion case returns 404 `E_NOT_FOUND`; its body is byte-identical to the body for a random UUID.
   3. `ia@`: the call without `purpose` returns 422 `E_PURPOSE_REQUIRED`.
   4. `ia@`: a search writes `audit.searched` (SP-42). Check as `akr_readonly`: `SELECT count(*) FROM audit.audit_log WHERE action = 'audit.searched' AND occurred_at > now() - interval '10 minutes'` returns at least 1.
   5. `ch@`: `GET /api/v1/audit/events` returns 403 `E_SCOPE` (no `audit.read` scope).
   6. Cedar deny: `ia@`: `POST /api/v1/packs` with kind `case` returns 403 `E_FORBIDDEN` (scope passes, Cedar refuses).
   7. `sec@`: `GET /api/v1/records/case/<suspicion id>/access-history` returns 404; `m@` on the same record returns 200.
   8. `sec@`: `POST /api/v1/audit/verify-runs` with `Idempotency-Key: k1` returns 202; the repeat with `k1` returns the same run id; `GET /api/v1/audit/verify-runs/{id}` reaches `pass` or `findings`, and its body holds no entry id.
   9. `m@`: `POST /api/v1/packs` with kind `board` and the period of a FINAL periodic board row returns 202; the download (step-up `export`) contains no `casework.cases` id (grep over the rendered JSON returns zero matches). With a DRAFT row the same request returns 409 `E_NOT_ALLOWED_IN_STATE` with sub-code `periodic_not_final`.
   10. `as@`: `POST /api/v1/assessor/samples` with frame `alert_register` returns 201; the repeat with the same `Idempotency-Key` returns the same sample id; the same key with a different body returns 422 `E_IDEMPOTENCY_REUSE`. `m@` on the same route returns 403 `E_FORBIDDEN`.
   11. `as@`: in `psql`, the items of the sample re-derive from the frame: `digest(seed || id, 'sha256')` (pgcrypto) ranked ascending gives the stored order, and the frame hash over the sorted frame ids equals the stored `frame_hash`.
   12. `adm@`: `POST /api/v1/training/register` with `user_id` of `adm@` returns 403 `E_SELF_APPROVAL`; with another active user returns 201; `POST …/void` with a reason code returns 200 and a second void returns 409 `E_NOT_ALLOWED_IN_STATE`.
   13. `mro@`: `POST /api/v1/inventory/{id}/validation-packs` for an item this user owns returns 403 `E_SELF_APPROVAL`; for another item returns 201 with a verdict; a second issue returns version 2; `usePatch` of the item is not called.
   14. `m@`: `PATCH /api/v1/registers/findings/entries/{id}` without `If-Match` returns 428; with a stale version 409 `E_VERSION_CONFLICT`; closing a closed row 409 `E_NOT_ALLOWED_IN_STATE` with `register_closed`.
   15. `ch@`: `POST /api/v1/registers/warnings/entries` returns 422 `E_VALIDATION` `kind_not_writable`.
   16. `ch@` against `m@` on `GET /api/v1/dashboard/command-centre`: CH receives no suspicion-tier value and no tipping-off tile; a count between 1 and 4 is returned as `value: null, suppressed: true` (threshold 5).
   17. `POST /api/v1/packs/verify` as `ds@`: the sha256 of the board pack returns `verified: true`; a random hash returns `verified: false`; the sha256 of a case pack as `ch@` returns `verified: false` (no `pack.read` for `case`).
   18. Read-only conformance: as `akr_readonly`, `INSERT INTO audit.audit_log …` is refused (permission denied); `SELECT` on `admin.role_grants` succeeds; `SELECT tier FROM casework.cases` is refused while `SELECT audit.resource_tier('case', <id>, NULL)` answers the tier word.
   19. Retention and classification: `SELECT count(*) FROM admin.retention_assignment WHERE schema_name = 'admin' AND table_name IN ('registers','protected_attributes','validation_packs','fairness_metrics','training_records','assessor_samples','verify_runs')` returns 7; every column of the seven tables has a row in `admin.column_classification`.
   20. Audit trail after the session holds `audit.searched`, `audit.verify_requested`, `assessor.sample_drawn`, `register.created`, `register.updated`, `validation.issued`, `training.recorded`, `pack.created`, `pack.verified`, `fairness.computed` (after the job run), `access.control_report_generated`.
6. Swagger UI lists the 31 routes of section 2 and the eight routes of sections 7.12 and 7.13 with their scopes; `make generate-api` regenerates the client with the hooks of section 5.8 and no other diff.
7. The job checks: `audit_verify_run` moves a queued run to `pass` or `findings` within its lease; `register_post_review_sync` run twice creates no second row for the same `post_review_recorded` event; `review_calendar_sweep` run twice creates no second warning; `fairness_monthly` run twice for one period inserts nothing the second time.
8. Phase 00 CBS conformance: not applicable (no connection to a source in this phase).
9. Existing tests (phases 00 to 23) still pass, with the phase 06 pack tests updated for the five new kinds and the phase 02 kind CHECK.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 2: corrections found while checking phases 03 and 04 (tools/agent/gap1-run2-verdict.md); lead-verified.

### 7.1 The assessor grant cap
The setting `assessor.grant_max_days` is not created: the cap of an `AS` grant is phase 04's
`access.assessor_max_days` (default 30, maximum 30; phase 04 7.10, DR-361). Section 1's seed row and section 3.9
read accordingly.

Gap pass 1 (2026-10-10), run 4a, from tools/agent/gap1-run4a.md; lead-verified (tools/agent/gap1-run4a-verdict.md).

### 7.2 Register sources, fairness metric and closure codes of later phases
The CHECKs are widened by the owners' migrations; this phase's DDL is not edited. `admin.registers.source` gains
`model_monitor` (phase 27), `assistant_monitor` (phase 28, `0088`) and `assess_control` (phase 29, DR-307).
`admin.fairness_metrics.metric` gains `score_top_band_rate` (phase 27). The closure codes of section 3.4 gain
`RETRAINED`, `RECALIBRATED`, `RESTRICTED_USE`, `RULES_ONLY` and `JUSTIFIED_NO_ACTION` (phase 27); for source
`model_monitor` a code outside these five is 422 `E_VALIDATION`, sub-code `closure_code_not_allowed`.

### 7.3 Review calendar rows and command-centre tiles of later phases
- Review calendar (section 3.4), derived rows, no table: `model_kill_drill` from `admin.models.next_kill_drill_due` and
  a starter family's `recalibration_due` (phase 27); `assistant_kill_drill` from
  `admin.assistant_kill_state.next_drill_due` (phase 28); `assess_control_test`, due `assess.control_test_max_days` after
  the last caught test per rail (phase 29). Amber 30 days before, red on the due date.
- Finding closure: the register route closes the release exceptions a finding of source `assess_control` names
  (phase 29 section 3.14 step 6).
- Command centre (section 3.7): the counts-only tiles `model_governance` (phase 27), the Assessment tile (phase 29) and
  the two tiles of phase 26.

### 7.4 Model and assistant branches of the validation pack
From phase 27 on, `ValidationPackService::issue` (section 3.6) has the model branch of phase 27; from phase 28 on, the
assistant branch for the kinds `llm_component` and `prompt_template`, with the seven parts of section 3.6 (phase 28
section 3.9). The signature, the verifier and the pack kinds do not change.

### 7.5 Two counts in the control report (phase 25)
From phase 25 on, `GET /access/control-report` gains two counts under this phase's small-number suppression:
break-glass grants activated in the month and access reviews overdue (phase 25 tables). Counts only.

### 7.6 Practice column in the training register (phase 31)
From phase 31 on, `TrainingRegisterService::list` (section 3.10) gains the display column `practice`, computed by
`TrainingService::practice_column` (phase 31 section 3.2 rule 4). `admin.training_records` is unchanged.

### 7.7 The register warning writer (DR-388)
This phase defines the port `RegisterWriter`, implemented by `GovernanceRegisterService`: `warn(tx, source,
source_key, title_key: WarningTitle, body) -> Option<Uuid>` inserts a row of kind `warning` with `ON CONFLICT
(source_key) WHERE source_key IS NOT NULL DO NOTHING` (index `uq_registers_source_key`; `None` when the key exists).
`WarningTitle` is a closed enum in `domain::governance` that maps each key to `title {ar, en}`; this phase's two variants are `ReviewCalendarRed` (source `review_calendar`; title {"ar": "حان موعد المراجعة", "en": "Review due date reached"}) and `FairnessRateReview` (source `fairness`; title {"ar": "معدل مجموعة يحتاج إلى مراجعة إنصاف", "en": "Group rate needs a fairness review"}); later phases add their
variants (phase 26, phase 29). `body` holds ids, codes and counts only. It is called only from jobs (section 4.1), and
this phase's own sweeps use it.

### 7.8 The case review clock (DR-384)
The clocks board row `case.review` reads `casework.cases.opened_at` plus the clock's days for cases in `open` or
`in_review` (phase 18 section 7+). This closes Open doubts 19; the clocks table of section 3.8 reads this rule.

Gap pass 1 (2026-10-10), run 4a: items carried from runs 3a and 3b and corrections found while checking phases 17 to 24 (tools/agent/gap1-run4a-verdict.md); lead-verified.

### 7.9 S-48 and the activation instant (DR-380)
Open doubts 11: S-48 (harness diff) is P2 (DR-380); the other items of that list go to the coverage runs of gap pass 1.
Open doubts 30 is closed: the activation instant is `scn.scenario_versions.effective_from` (phase 16 section 7+).

Gap pass 1 (2026-10-10), run 4b: items carried from run 4a and corrections found while checking phases 25 to 31 (tools/agent/gap1-run4b-verdict.md); lead-verified.

### 7.10 Canary records are P2 (DR-391)
Open doubts 12 is closed: canary records (SP-30, research 30 sections 8.6 and 8.8) are P2 and no phase of this plan
builds them (phase 25 section 7+). The control report keeps `canary_not_built: true` and the "not built" line.

Gap pass 1 (2026-10-10), run 5b (coverage of SP and K IDs), from tools/agent/gap1-run5b.md; lead-verified (tools/agent/gap1-run5b-verdict.md).

### 7.11 Manual case grants in the control report (SP-28, DR-395)
The control report (section 3.2) adds two counts for the month: manual grants on suspicion-tier cases made
(`casework.case_acl`, `reason = 'manual'`, case `tier = 'suspicion'`, `granted_at` in the month) and grants that expired
(`revoked_by IS NULL AND revoked_at` in the month). The small-number rule of the report applies. This is the CISO's view
of the time-boxed manual grants of phase 18 section 7.10.

Gap pass 1 (2026-10-10), run 7a (coverage of UX and W IDs, gap item (ab)), from tools/agent/gap1-run7a.md; lead-verified (tools/agent/gap1-run7a-verdict.md).

### 7.12 Register exports (UX-31; DESIGN-SYSTEM 8.1 REGISTER pattern; DR-400)
Each REGISTER screen of the catalogue offers one export, built as a job.

| Method | Path | Scope | Description |
|-|-|-|-|
| POST | `/api/v1/exports` | the list route's scope | Start an export `{register, filters, format: csv or xlsx}`; 202 with the export id; `Idempotency-Key` (New; the list route's Cedar action) |
| GET | `/api/v1/exports/{id}` | the list route's scope | Status of the caller's own export (New; requester only; the Cedar action of the list route the export names) |
| GET | `/api/v1/exports/{id}/file` | the list route's scope | The file; step-up class `export`; access event `export` first (New; requester only; the Cedar action of the list route the export names) |

1. `register` is a screen ID of the catalogue whose pattern is REGISTER; its filters are those of the screen's list
   route. The route checks that list route's scope and Cedar action for the caller; anything else is 422 at
   `/register`.
2. Job `register_export` (akr-worker; payload `{register, filters, format, requested_by}`): it calls the screen's own
   list service as the requester, so the visibility and masking are the list route's (suspicion rows absent for a
   non-member; the report register is never exported, RP-61). The export id is the job id; no table is added.
3. CSV: UTF-8 with a byte-order mark, ISO 8601 dates, Western digits. XLSX: right-to-left sheet view for Arabic content.
   In both, a text cell starting with `=`, `+`, `-`, `@`, a tab or a carriage return is prefixed with an apostrophe.
4. The file is stored through `EvidenceStore::stage` (phase 02) as kind `render`, owner kind `register_export`, owner id
   = the export id, retention class `R6` for the kind `decisions` and `R9` for every other register of this phase, classification `personal` (a file holds whatever rows the requester may read; the enum `admin.data_classification` of phase 00 has no value `restricted`), and is served only once clean: while the scan is pending or the file is not clean, the file route answers 404 `E_NOT_FOUND` and the status route shows the state.
5. Audit `export.requested {register, format}` and `export.downloaded {register, sha256}`. Per-user watermarking is P2
   (DR-396). PDFs stay on the pack path.
6. Tests: a fixture name starting with `=` is prefixed in both formats; a suspicion row is absent for a non-member and
   the export row count equals the list's; another user's export id answers 404.

Gap pass 1 (2026-10-10), run 8a (coverage of VA and VO market IDs), from tools/agent/gap1-run8a.md; lead-verified (tools/agent/gap1-run8a-verdict.md).

### 7.13 Parallel run beside the bank's current system (VA-54, VO-03; ROADMAP P1 "Parallel run"; DR-402)
ROADMAP P1 runs AKR-TMS beside the bank's current system for two to three months: every alert compared both ways, every
disagreement explained in a log the MLRO reviews weekly. This section builds the import, the comparison and the log.
The comparison is by party and date, not by scenario, because the two systems' rule lists differ.

**Migration.** `0077_admin_fairness_training_assessor` also creates the table below; `0079_admin_guards_grants` adds
`GRANT SELECT, INSERT, UPDATE ON alert.parallel_run_rows TO akr_app`, the classification (`internal` for ids, codes,
dates and outcome; `personal` for `incumbent_ref`, `incumbent_rule` and `note`), the retention row `('alert',
'parallel_run_rows', 'R5', 'age', 'created_at', 'Parallel-run comparison; kept with the alert class')` and the setting
`parallel_run.match_days` (integer, 3, bounds 0 to 14).

```sql
CREATE TABLE IF NOT EXISTS alert.parallel_run_rows (
    id                    UUID PRIMARY KEY,
    import_id             UUID NOT NULL,                  -- the job id of the import (no import table)
    party_id              UUID,                           -- no foreign key (canonical party, DR-87); NULL when unresolved
    outcome               TEXT NOT NULL CHECK (outcome IN ('both', 'akr_only', 'incumbent_only', 'unresolved')),
    incumbent_ref         TEXT CHECK (incumbent_ref IS NULL OR length(incumbent_ref) BETWEEN 1 AND 100),
    incumbent_rule        TEXT CHECK (incumbent_rule IS NULL OR length(incumbent_rule) BETWEEN 1 AND 200),
    incumbent_on          DATE,
    akr_alert_id          UUID,
    akr_alert_created_at  TIMESTAMPTZ,
    explanation_code      TEXT CHECK (explanation_code IN ('PRL_SCENARIO_GAP', 'PRL_PARAMETER', 'PRL_DATA',
                              'PRL_TIMING', 'PRL_INCUMBENT_NOISE', 'PRL_AKR_NOISE', 'PRL_OTHER')),
    note                  TEXT CHECK (note IS NULL OR length(note) BETWEEN 1 AND 2000),   -- passes PolicyPackService::scan
    explained_by          UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    explained_at          TIMESTAMPTZ,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT fk_parallel_akr_alert FOREIGN KEY (akr_alert_id, akr_alert_created_at)
        REFERENCES alert.alerts(id, created_at) ON DELETE RESTRICT,
    CHECK ((outcome = 'unresolved') = (party_id IS NULL)),
    CHECK ((outcome = 'akr_only') = (incumbent_ref IS NULL)),
    CHECK ((outcome IN ('both', 'akr_only')) = (akr_alert_id IS NOT NULL)),
    CHECK ((explanation_code IS NULL) = (explained_by IS NULL) AND (explained_by IS NULL) = (explained_at IS NULL)),
    CHECK (explanation_code IS DISTINCT FROM 'PRL_OTHER' OR note IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_parallel_rows_import ON alert.parallel_run_rows (import_id, outcome);
CREATE INDEX IF NOT EXISTS idx_parallel_rows_open ON alert.parallel_run_rows (created_at) WHERE explanation_code IS NULL
    AND outcome IN ('akr_only', 'incumbent_only');
```

| Method | Path | Scope | Description |
|-|-|-|-|
| POST | `/api/v1/parallel-run/imports` | `case.decide` | Upload the current system's alert extract (CSV); `Idempotency-Key`; 202 with `Location` (New; `parallel_run.write`) |
| GET | `/api/v1/parallel-run/imports/{id}` | `dashboard.read` | Import status with row, unresolved and outcome counts (New; `parallel_run.read`) |
| GET | `/api/v1/parallel-run/rows` | `dashboard.read` | Rows by `import_id`, `outcome` and `explained`; keyset cursor; no totals (New; `parallel_run.read`) |
| POST | `/api/v1/parallel-run/rows/{id}/explanation` | `case.decide` | Record the explanation code and note of one disagreement (New; `parallel_run.write`) |
| GET | `/api/v1/parallel-run/summary` | `dashboard.read` | Counts by ISO week and outcome, with the unexplained count (New; `parallel_run.read`) |

**Cedar** (`C0_governance.cedar`, one role per policy): `parallel_run.read` for M and CH; `parallel_run.write` for M
only. No existing action has exactly these roles. IA reads the outcome through the audit events.

`ParallelRunService`:
1. Import: the CSV has the columns `incumbent_ref`, `cif`, `alerted_on`, `rule` and `outcome`, UTF-8, at most
   `screening.feed_max_bytes`. The file goes through `EvidenceStore::stage` (phase 02; owner kind `parallel_import`,
   kind `upload`, retention R5), and job `parallel_import {evidence_id}` is enqueued. Its job id is the import id.
2. Job `parallel_import`: wait for the AV scan as `list_parse` does (phase 14 section 3.2). Resolve each `cif` to the
   party of the current `canon.customer` row; no match gives an `unresolved` row. Then compare in one transaction, over
   the window from the earliest to the latest `alerted_on`, widened by `parallel_run.match_days`:
   - `both`: an incumbent row whose party has an `alert.alerts` row created within `parallel_run.match_days` of
     `alerted_on`;
   - `incumbent_only`: an incumbent row without one;
   - `akr_only`: an `alert.alerts` row in the window (source kind `scenario`) that no incumbent row of its party
     matched.
   Audit `parallel_run.imported {import_id, rows, unresolved, both, akr_only, incumbent_only}`, counts only.
3. Explanation: the row must be `akr_only` or `incumbent_only` and not yet explained (else 409
   `E_NOT_ALLOWED_IN_STATE`); `PRL_OTHER` needs a note (422 `E_VALIDATION`). Set the code, note, `explained_by` and
   `explained_at`. Audit `parallel_run.explained {row_id, code}`.
4. Lists and counts filter in SQL by the phase 18 section 3.10 party visibility (`services::visibility::build`), so a
   suspicion-tier party outside the reader's access is absent and the counts come from the filtered query.
5. The summary is the weekly review: the MLRO reads it, with its unexplained count, each week, and before the go-live
   decision.

**Screen.** G4 gains the tab "Parallel run" for M and CH: the summary, the rows of an import, and the explanation
form (codes in the Office's words).

**Tipping-off review.** The rows and the summary are read only by M and CH, through the party visibility. Nothing is
written to `admin.registers`, the command centre, a notification or the case. The import file is an evidence object
of the same class.

**Tests.** A fixture extract with one shared, one incumbent-only and one unknown `cif` gives one `both`, one
`incumbent_only`, one `unresolved` and the `akr_only` rows of the window. An explained row refuses a second
explanation. A CH session cannot explain. A suspicion-tier party outside the reader's access is absent from the rows
and the counts.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.14 `sync_post_review`: the finding's title, body and join (V-24-02-b, MEDIUM; DR-413)
Section 3.4 `sync_post_review`, for each event. The join: the event's `instance_id` is an `admin.workflow_instances` row with `purpose = 'post_review'` and `resource_kind = 'change_request'`; its `resource_id` is the change request, and the finding's `owner_id` is that request's `maker_id` (this settles Open doubts 31). The `title` is fixed by the outcome: `concern` is {"ar": "ملاحظة من المراجعة بعد التغيير", "en": "Concern from the post-change review"}; `rollback_needed` is {"ar": "المراجعة بعد التغيير تطلب التراجع", "en": "Post-change review asks for a rollback"}. `body` is `{subject_key: <change request id>, outcome, day}` and nothing else; the notes of the review are not copied.

### 7.15 Step-up action constants of this phase (V-24-02-c, V-24-11-e, LOW; DR-413)
This phase declares two `StepUpAction { name, class }` constants with phase 01's closed classes: `register.accept_risk` (class `approval`) for the PATCH of section 3.4 when `status` becomes `accepted_risk`, and `export.download` (class `export`) for the file route of section 7.12. Pack downloads keep phase 06's `pack.download`.

### 7.16 Calendar sweep: the entry point (V-24-03-a, LOW; DR-413)
The job `review_calendar_sweep` (section 4.1) calls `GovernanceRegisterService::sweep_calendar(now)`. It runs the row query of `review_calendar` (section 3.4) with no caller: calendar rows hold no customer, case or narrative data (section 3.15). For each red row it calls `RegisterWriter::warn` (section 7.7) with the source key of section 3.4.

### 7.17 Board pack: which aggregates, under which predicate (V-24-04-a LOW, V-24-04-b MEDIUM; DR-413)
The "aggregates of 3.4 to 3.7" of the board row (section 3.11) are the command-centre tiles of section 3.7 that M, CH and IA all may see: Late and at-risk clocks, Verification (status only), Data and coverage incidents, Arrivals against throughput, Backlog by elapsed age, Top scenarios with cases, QA status, Regulatory deadlines, Branch liaison reports and Governance open items (the counts of 3.4 and 3.5). Not in the pack: Decisions waiting for me, Productive alerts, UTRs and STRs, and the Tipping-off summary. Report counts reach the board through the periodic row of phase 21 only.
`BoardSource::payload(period) -> Value` builds them. It calls the `CommandReadRepository` and `ClockReadRepository` methods of those tiles with the predicate of a principal that holds the role CH alone (no case access list entry, no QA grant), not the builder's own, so the pack shows no reader more than the command centre shows CH. The small-number rule of section 3.7 applies.

### 7.18 Case packs: an unknown case id (V-24-05-a, MEDIUM; DR-413)
`POST /packs` with kind `case`: the Cedar check comes first (a caller without M is 403 `E_FORBIDDEN` whatever the id). For M the case is looked up before the enqueue, and an unknown id is 404 `E_NOT_FOUND`. A case that can no longer be read when the job runs fails the build with `pack.build_failed {kind: case, reason: case_not_found}`.

### 7.19 Route 27: the method and its access events (V-24-07-a MEDIUM, V-24-07-b LOW; DR-413)
`AssessorService::sample(ctx, id) -> AssessorSampleView` (route 27, `assessor.sample`) reads `SampleRepository::get`. The answer lists ids and ranks, so the list rule of phase 02 applies: `scope.record_hits` writes one `search_hit` event for each restricted or suspicion-tier item, with purpose `supervisory_inspection`, before the response. An item is "viewed" (step 2 of `draw`, DR-226) when the assessor traces it: every node of the trace writes its `view` event (section 3.9). No audit kind is added.

### 7.20 Route 9: the list method and its reads (V-24-08-a, MEDIUM; DR-413)
`GovernanceRegisterService::list(ctx, kind, filter, cursor) -> Page<RegisterView>` (route 9, `register.read`) reads by kind: `findings` and `warnings` with `RegisterRepository::list`; `fairness` with `FairnessRepository::list`; `exclusions` with phase 12's `CoverageRepository::list_exclusions`; `decisions` with `CaseDecisionRepository::register_page(conn, predicate: &VisibilityPredicate, page: Keyset) -> Result<Page<DecisionRegisterRow>, RepoError>`, a method this phase adds to phase 18's repository. `DecisionRegisterRow` holds the columns section 3.4 lists and never `text_enc`. A `kind` outside the five is 422 `E_VALIDATION` `unknown_register_kind`.

### 7.21 MRO reads the registers and the fairness metrics (V-24-08-c, MEDIUM; DR-413)
Lead decision. Sections 3.5 and 3.15 say MRO reads the registers of exclusions, findings and warnings and the fairness metrics, and MRO writes findings and resolves fairness reviews; the action table and the policies of section 3.12 gave MRO no read. MRO gains it. Routes 9, 11, 13 and 15 accept the scope `model.risk` beside `dashboard.read`, and `60_audit.cedar` gains:
```cedar
@id("gov.read.mro") @role("MRO")
permit (principal, action in [Akr::Action::"register.read", Akr::Action::"fairness.read"], resource)
when { principal.roles.contains("MRO") };
```
The action table row reads: `register.read` and `fairness.read` for M, CH, IA and MRO; `command.read`, `clock.read` and `training.read` for M, CH and IA. For MRO the kind `decisions` of route 9 returns an empty page: its predicate is phase 17's, which admits no case for MRO. Test: `mro@` reads `GET /registers/findings` and `GET /registers/fairness` with 200, gets an empty page from `GET /registers/decisions`, and gets 403 `E_SCOPE` from `GET /dashboard/command-centre`.

### 7.22 Control report: the two case-grant counts are read on the app pool (V-24-10-a, HIGH; DR-413)
`akr_readonly` reads nothing in `casework` (section 1.3), so the two counts of 7.11 cannot run on `readonly_pool`. `AccessHistoryService::control_report` reads them on the app pool with `CaseAccessRepository::manual_grant_counts(conn, from, to) -> Result<(i64, i64), RepoError>` (made, expired), a method this phase adds to phase 18's repository; its query is the one 7.11 gives. Every other count of section 3.2 stays on `readonly_pool`. The sentence of section 3 that every query of sections 3.1 to 3.3 runs on `readonly_pool` reads with this one exception. Test: as `m@`, the report of a month with five manual grants on suspicion-tier cases returns `5`, and no query of the request fails with permission denied.

### 7.23 Export service, its job row and its owner kind (V-24-11-b, V-24-11-d, V-24-11-f, MEDIUM; DR-413)
- `ExportService` holds the three routes of section 7.12 (`request`, `status`, `file`) and the handler of the job `register_export`. It keeps a closed map in code from the screen ID to the list method of that screen; the phase that owns a REGISTER screen adds its entry, and a screen ID with no entry is 422 at `/register`. This phase's entries: G4 is `GovernanceRegisterService::list` (one export per kind: the request's `filters` carry `kind`, one of the five register kinds that route 9 takes from its path; a missing or unknown kind is 422 at `/filters/kind`) and G9 is `TrainingRegisterService::list`. G5 has no entry: its control report is counts only, and its access history is read per record and is not exported. G6 is never exported (section 3.1 step 9). The job rebuilds the requester's context from `requested_by` and calls the entry.
- Section 4.1 gains the row: `register_export` | worker | `{ register, filters, format, requested_by }` [`register_export:<export id>`] | 1, 3 | enqueued in the request transaction of `POST /exports`; the export id is the job id.
- Section 4.5 item 1 and Open doubts 29: `register_export` is a third owner kind of phase 02's evidence registry, beside `validation_pack` and `training_record`. Its `EvidenceResolver` is this phase's and admits the requester alone.

### 7.24 Explanation: the method, the guarded write and the note scan (V-24-12-a, V-24-12-b, V-24-12-c, MEDIUM; DR-413)
`ParallelRunService::explain(ctx, row_id, code, note)` (the explanation row of section 7.13, `parallel_run.write`) runs step 3 in this order: the note passes `PolicyPackService::scan` (phase 09, scope `reason_text`; a forbidden term is 422 `E_VALIDATION` `forbidden_term`), as the DDL comment says; then `ParallelRunRepository::explain(tx, row_id, code, note, by, at) -> Result<u64, RepoError>` runs one UPDATE with `WHERE id = $1 AND explanation_code IS NULL AND outcome IN ('akr_only', 'incumbent_only')`. Before the UPDATE, in the same transaction, `ParallelRunRepository::visible(tx, row_id, predicate) -> Result<bool, RepoError>` reads the row under the caller's party visibility (the predicate of section 7.13 item 4); a row that is absent or not visible is 404 `E_NOT_FOUND`, and nothing is written. Zero rows updated is then 409 `E_NOT_ALLOWED_IN_STATE`, so the second of two explainers is refused and nothing is overwritten. `ParallelRunRepository` is this phase's repository of `alert.parallel_run_rows`; it also holds `insert_rows(tx, rows)`, `rows(conn, filter, predicate, page)`, `import_status(conn, import_id, predicate)` and `summary(conn, predicate)` for steps 2, 4 and 5.

### 7.25 Parallel-run policies (V-24-12-e, LOW; DR-413)
Section 3.12 gains `entity ParallelRunRow = { outcome: String, explained: Bool };` and, in `C0_governance.cedar`, one role per policy:
```cedar
@id("parallel_run.read.m") @role("M")
permit (principal, action == Akr::Action::"parallel_run.read", resource) when { principal.roles.contains("M") };
@id("parallel_run.read.ch") @role("CH")
permit (principal, action == Akr::Action::"parallel_run.read", resource) when { principal.roles.contains("CH") };
@id("parallel_run.write.m") @role("M")
permit (principal, action == Akr::Action::"parallel_run.write", resource) when { principal.roles.contains("M") };
```

## Decisions

Numbered from DR-218 (the last number in the phase 22 and 23 decisions is DR-217). Each decision is one paragraph with its rationale. The lead adds them to `docs/plan/research.md`; this file does not edit it.

**DR-218 (the read-only pool and the DATA-MODEL section 15 changes).** The audit trail's details and the access log are readable only as `akr_readonly` (phase 02), and `akr-api` has no read-only pool today (phase 00). Phase 24 adds `readonly_pool` to `akr-api` with the KV path `db/akr_readonly`, used by four reads only (the viewer, the access history and control report, the pack verify and the verify-run estimate). In the same change DATA-MODEL section 15 changes as section 1 says: `registers` keeps its name with the columns of GV-34, `validation_packs` keeps its parts and verdict with the vault key, `fairness_metrics` gains the group and suppression columns. Models, model scores and assistant drafts stay P2. The reason for keeping one register table is that findings and warnings are the same kind of governance record with different sources.

**DR-219 (suspicion records are invisible to IA and SEC, including their access history).** The viewer's rows and the access history of a suspicion-tier record are refused to IA and SEC with the same 404 as a missing record, and the audit search leaves such rows out of its pages (no masked rows, no total). The M-only rule of phase 17 for suspicion tier is the source of the decision. Research file 30 section 8.8 wants the CISO and internal audit to see tipping-off attempts; this phase gives them the attempts as counts (the control report) and not the records. The bank may want the opposite for IA (Open doubts 25); the switch is one Cedar clause and one predicate.

**DR-220 (verify results keep counts and kinds only).** A verify run stores the `VerifyReport` as counts and finding kinds. Entry ids, anchor ids, object keys and detail text stay in the phase 02 report, the SIEM (phase 25) and the operator's CLI. A viewer of the verify run therefore learns that a break exists and where in time, never which row of a suspicion case.

**DR-221 (decisions and exclusions are read views; findings and warnings are the only register rows).** A copy of `casework.decisions` or `admin.exclusions` would be a second record of a decision that phase 18 and phase 12 already own and seal. The register page reads them through their own predicates. Findings and warnings are new rows because no earlier phase holds them: phase 04's post-review concerns and phase 16's review clocks have no place to live until this table exists.

**DR-222 (the register is an append-and-close record).** A finding keeps its identity, text and author for life; its status moves forward; a closed row is final; nothing is deleted (the guard of section 1.4). The owner closes the finding with a closure code; a risk accepted by M needs the second factor of class `approval`, and the owner of a finding never accepts its risk. This is the GV-34 tracker without the workflow engine, because the engine is for decisions and changes, not for findings.

**DR-223 (validation packs are versions; the verdict is computed).** A validation pack is written once and the next issue is the next version (GV-09, GV-15). The verdict is a function of the seven parts and the per-bank delta; no person sets it, so the validator cannot reach a `pass` by choice. The validator is the caller, never the owner (GV-32), enforced in Cedar and in the table. A `pass` moves the item's validation dates through phase 04; approval for use stays a separate step-up act (phase 04).

**DR-224 (the command centre is live, counted through each source's predicate, small counts suppressed).** The tiles read the sources on every request, with `Cache-Control: no-store`. Counts pass through the same predicate as the lists they open, so a tile can never show more than its list (the phase 17 rule). Counts from 1 to 4 are suppressed with the phase 21 threshold. Suspicion-tier counts are M's alone, and the tipping-off tile is not sent to CH or IA at all, because even its existence tells them that denied reads of suspicion records happened.

**DR-225 (the clocks board lists and never moves a clock).** A clock moves only through its owner, as phases 19 to 22 set it. The board joins the due columns that the owners already hold, so a clock cannot be shown at one date and enforced at another. Suspicion-tier subjects are M's alone, rows and counts alike.

**DR-226 (the assessor's grant: a role with a time box, no case access entry, and the suspicion tier in the designed sample).** The assessor is a person with role `AS` in `admin.role_grants`, bounded by `valid_to` (phase 01). `casework.case_acl` gets no assessor reason: an access-list row is a case team's grant, it is refused on sealed cases (phase 18), and the assessor needs metadata, not the team's work. The sample may hold suspicion-tier items because a sample that excludes them tests nothing; each item shown and each node of a trace is logged with purpose `supervisory_inspection`, and the trace returns no narrative. The bank approves the exception (Open doubts 1).

**DR-227 (the board pack carries no case reference).** The board pack is built from the signed periodic row and aggregates only, and its builder refuses a payload that carries a case or report id. Phase 21 suppresses small cells in the row, so the pack inherits the rule; a board reader who holds the pack learns the totals the MLRO's board already receives. Case packs and report packs are M's alone.

**DR-228 (the training register: ADM records, nobody records their own training).** Scope `admin.users` (API.md 3.16) and the Cedar `training.no_self` rule make the administrator the recorder. An administrator who is also a trainee is refused, and the register is readable by M, CH and IA as evidence for the staffing clause D2.1(b)(4) that research file 28 cites. A correction is a void and a new row, so the history is never rewritten.

**DR-229 (the D2 map is data, signed with the release).** The mapping of the CBI assessment guide's clauses to the bank's evidence is a JSON file in the release bundle, checked at start-up against its manifest hash (S-56: "the D2 test-script map kept as data"). Clauses the research set does not quote are `text_pending` and not invented: the bank's copy of the guide supplies their text (Open doubts 6).

**DR-230 (download step-up is `export`; verification answers the same for unknown and unreadable).** Pack downloads are exports, so the step-up class is the closed `export` of phase 03 (phase 06's `pack_download` is not a class). `POST /packs/verify` answers `verified: false` both for an unknown hash and for a pack the caller cannot read, so a non-member cannot tell a case pack from nothing (Open doubts 5, 26).

**DR-231 (the review calendar is derived; only red rows become warnings).** The calendar reads the due columns of rules, exclusions, change requests, readiness and scenarios and writes nothing. Only a row that turns red opens a warning, once (the unique `source_key`). Amber rows are shown, not written, so the warnings list stays a list of things somebody must act on (GV-22, S-46).

**Lead corrections (review 2026-10-09).** The audit viewer's tier resolver mirrors phase 17's tier rule 2 (case family tier, the tier of every case an alert belongs to, `suspicion` for a restricted scenario) and is SECURITY DEFINER, so `akr_readonly` gets no table or column grant in `casework`, `alert`, `scn` or `canon` (phase 17 grants it nothing there, and it has no USAGE on `canon`); `access_event` rows are hidden on the higher of the stored and the current tier. The decisions register and the clocks board use the sources' own predicates (phase 17 `services::visibility::build`, phase 18 access list, phase 19 QA grants), not the tier alone: phase 17 rule 4 keeps restricted cases from CH and IA. The assessor traces a suspicion alert only when it is an item of a sample drawn under the grant in force. `POST /packs`, `GET /packs/{id}` and the download accept `assessor.read`, or AS could never reach its pack. Floors bound the side that weakens the control (`fairness.review_ratio` `{"max": 1.25}`, phase 04's `access.assessor_max_days` at most 30, finding due days max 365). The register's retention clock is `closure` on `closed_at`. The control report suppresses small counts. Cedar has one role per policy and no `containsAny`; `context.pack_kind` is read behind `has`; `casework.decisions` has no `outcome` column; classification uses `CROSS JOIN LATERAL unnest`.

## Open doubts (for gap pass 1 and the bank)

1. **The assessor and the suspicion tier (DR-226).** A sample of suspicion items and a trace of a suspicion alert show the decision type, the report type and the case dates to the assessor. That is the one designed exception to the tipping-off rule of this phase. The bank approves it, or narrows the trace to counts and ids. Until then the trace is AS-only for suspicion items.
2. **Unknown audit resource types fail closed.** `audit.resource_tier` returns `suspicion` for a type it does not know, for example an `evidence` or `report` row with no case id. IA and SEC then do not see those rows. The bank confirms the mapping before the viewer is used for an insider review.
3. **Chain status shows volume.** Anchored row counts per partition reveal the volume of suspicion activity, not its resources. The bank's legal view decides whether volume is acceptable.
4. **A read-only credential in `akr-api`.** `akr-api` gains a second database credential (`akr_readonly`, KV `db/akr_readonly`). The alternative is to move the viewer to a separate read-only binary, which the architecture does not list. Security approves the credential and its rotation.
5. **The pack download step-up class.** Phase 06 names a step-up class `pack_download` that is not in the closed list of phase 03 and phase 22. This phase uses `export`. Phase 06 is aligned in its next revision.
6. **The D2 clauses.** The research set quotes D2.1(a)(2), D2.1(a)(3), D2.1(b)(4), D2.1(b)(5), D2.1(b)(6), D2.1(c)(1) and D2.2(b)(5) without the clause text. The first version of the map covers the clauses with text (D2.1(a)(2), D2.1(b)(4), D2.1(c)(1)); the others are `text_pending`. The bank supplies the guide's clause list and the expected evidence per clause.
7. **Training: the course list and the recorders.** The course codes are the bank's list (none is given by the research). Only ADM records; M, CH and line managers are not admitted in this phase. The bank chooses whether M records training of analysts, which needs a scope and a Cedar clause.
8. **The validation document and the validator kind.** The seven parts are stored and the canonical JSON is the evidence; the printable validation report is the `validator` pack kind, which the prompt places in P2. The bank accepts the JSON as the P1 record until then.
9. **Phase 04 extension for a passing validation.** A `pass` calls `InventoryService::update` to set `last_validated_at` and `next_validation_due`. Phase 04's CHECK requires a validator and `last_validated_at` before `approved_for_use`; the extension is a call and not a new column. Phase 04 is asked to accept the call.
10. **Not carried: GV items with a target.** GV-06 (committee pack and calendar), GV-08 (annual self-assessment and crosswalk), GV-13 (behavioural-testing sample plan), GV-18 (rule-side monitoring thresholds), GV-19 (incident log with forced decision), GV-23 (the quarterly committee cycle; the board pack carries the KPI set only) and GV-10 (phase 16 S-48, the independent re-implementation). Gap pass 1 assigns each; GV-06 and GV-23 suit phase 20 or phase 25.
11. **Not carried: S, M, UX and SP items.** S-41 (analyst-hour model), S-43 (tuning workbench, phase 16 or later), S-48 (harness diff, with the validation packs), S-54 and S-55 (risk statement and importer), S-57 (one-hour assessor pack of ten exhibits), S-60 (Office engagement pack), S-05's card export, the recalibration clock of S-46 (phase 16), the nine metrics of S-53 other than fairness, the remediation generator of S-56, the RP-64 figure link, the RP-63 alert list and scoring snapshot, the redaction and MLRO certification with requester watermark of M-63, and the export cap of SP-42 (no export route here). Gap pass 1 assigns each.
12. **Canary records.** Research file 30 sections 8.6 and 8.8 name canary records and their triggers. No phase builds them. The control report shows the line as "not built". Phase 25 or an amendment of phase 18 owns them.
13. **The scenario gate of the proxy scan.** `ProtectedAttributeCheck` runs monthly in this phase. The preventive gate belongs to phase 16's scenario change target, which must call the check in `check_before_decision`. The owner of phase 16 accepts the call.
14. **Owners of the protected attributes.** The register's `owner_id` values and the bank's `allowed_uses` per attribute need the bank's model risk officer. Until set, a fairness review opens a warning, not a finding.
15. **The capacity ratio tile.** Mockup 01 shows a capacity ratio beside the throughput. Capacity is P2 (phase 31 workbench, G2), so the tile is not in this phase.
16. **The source of the board pack.** The board pack is built from the final periodic row of kind `board` (phase 21). The bank confirms that the board sees this row and not a separate board report, and its cadence.
17. **Who builds the assessor pack.** This phase lets AS build it and AS and IA read it. The bank may want DS or M to build it for the assessor's visit.
18. **The post-review feed.** The sync reads `admin.instance_events` (phase 04) every 15 minutes. An outbox topic would be cleaner but adds a handler to a topic that phase 04 does not produce; the scan is the chosen route.
19. **The clock `case.review`.** Phase 09 names the clock; no phase names the column that holds the case review date. Until phase 18 names it, the board shows no rows for this clock.
20. **Closure codes and who accepts a risk.** The closure codes of 3.4 are a list in code (`EFFECTIVE`, `RULE_CHANGED`, `COMMITTEE_ACCEPTED`, `NOT_REPRODUCED`, `DUPLICATE`). The bank confirms the list and whether M alone may accept a risk or the committee must.
21. **The register's clock.** The register's retention clock is `closure` on `closed_at` (lead review), so an open finding is never due; the bank confirms the R9 period for closed findings.
22. **Verify-run results and the SIEM.** Verify results keep counts and kinds only (DR-220). Phase 25's SIEM export may need the entry ids for an incident; that export reads the phase 02 report, not this table.
23. **SEC's purpose code.** Phase 03's purpose list has no code for a CISO's review of the audit trail. SEC searches with `other` and a reason. The bank may want a code of its own; that is a row in `admin.purpose_codes`.
24. **A customer's governorate.** The fairness metric by governorate reads the `governorate` key of `canon.party_contact.address` (JSONB, phase 10). Whether that key is plain or inside the sealed value is not stated in phase 10; if sealed, the metric drops the governorate group.
25. **Access history for IA and SEC on suspicion records (DR-219).** The decision hides the suspicion record's access history from IA and SEC. Research file 30 section 8.8 may want the opposite. The bank decides.
26. **Uniform verification (DR-230).** A non-member who holds a case pack's hash is told "not verified". An honest holder without `pack.read` gets the same answer. The bank accepts that the verification of a case pack is an MLRO act.
27. **The new dev user `as@bank.test`.** The curl suite needs an assessor account (role `AS`, grant valid for one day). Phase 01's dev users do not include it; the implementation adds it to the dev seed.
28. **`GET /audit/events/{id}` needs `occurred_at`.** Both audit streams are partitioned by month; the id alone would scan every partition. The route takes `occurred_at` as a required query parameter. API.md 3.4 does not say so; the row is updated in the next API revision.
29. **The evidence owner kinds.** `validation_pack` and `training_record` are new owner kinds of phase 02's registry; their `EvidenceResolver` implementations are phase 24's. Gap pass 1 checks the registry's list.
30. **The scenario activation instant.** The review calendar's scenario rows read the activation instant of a scenario version. Phase 16 names the column in its own DDL; the name is confirmed in gap pass 1.
31. **The change request's maker column.** The post-review finding is owned by the change request's maker. Phase 04's column name is confirmed in gap pass 1.

## What This Phase Does NOT Touch

- No model registry, shadow or challenger runs, kill switch, model scores or GenAI assistant (P2, phases 27 and 28); `admin.models`, `admin.model_scores` and `admin.assistant_drafts` are not created.
- No SIEM export, no break-glass lifecycle, no access review lifecycle and no inspector accounts (phase 25); the access-review route of phase 03 is unchanged.
- No new case access list reason (`casework.case_acl` keeps its phase 19 list); no change to phase 17 or phase 18 tables, routes or sealed-case rules.
- No change to phase 19's `RegisterService`, RFIs, internal reports, alarms or QA tables; the QA counts of the command centre are reads.
- No change to phase 21's periodic packs, report routes, small-number rule or the `report.small_number_threshold` setting; the board pack reads the periodic row.
- No change to phase 22's liaison report, branch window or notifications; the liaison count is a read.
- No change to phase 23's retention engine, legal holds, `61_retention.cedar` or retention routes; this phase adds only its own rows (0079).
- No change to phase 06's gap report, CBI pack, its scenario catalogue or `C0_packs.cedar`; the `pack_build` job gains kinds.
- No change to phase 09's authority matrix, policy packs or clock registry; no clock is added or changed.
- No change to phase 04's change requests, workflow engine, `ChangeTargetRegistry` entries other than the protected-attribute target, or inventory routes beyond the validation call of section 3.6.
- No CBS connection and no CBS read; the only database credentials added are `akr_readonly` for `akr-api` (Open doubts 4).
- No outbox topic, no timer kind, no step-up class (the closed list of phase 03 is used as is).
- No file of the Cedar bundle other than `60_audit.cedar` and `C0_governance.cedar`; `80_detection.cedar`, `C0_packs.cedar`, `61_retention.cedar` and the phase 09 `C0_reference.cedar` and `C0_policy.cedar` are unchanged.
- No screen outside the seven catalogue rows of section 5 (G1, G4, G5, G6, G8, G9, R4); no branch bundle change.
- No export file of the viewer (SP-42's cap is Open doubts 11) and no PDF rendering of the validation pack (Open doubts 8).
