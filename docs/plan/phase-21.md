# Phase 21: Reporting to the Office

**Goal:** The bank files what the AML/CFT Office and the Central Bank require, and nothing else leaves the system unseen: a UTR, STR or TFR is drafted from its decided case, validated by the phase 07 engine, signed by the MLRO through the phase 18 approval, packaged with a SHA-256 manifest, filed by a named officer with the goAML reference and the portal capture kept, and closed only when the Office acknowledges it; rejections become field findings and a poor grade becomes a returned-for-completion task; the daily CTR batch is built at the CBI rate of the business date; the Committee and Office letters that phase 20 and phase 19 request are numbered, rendered, signed and recorded; and the CBI, liaison, board and MLRO periodic packs are generated from data.

**Dependencies:** Phase 00 (`admin.timers`, jobs, settings, cron schedules, the outbox), Phase 01 (`StepUpService` and the closed action classes `signing`, `filing`, `export`, `approval`; roles `M`, `A2`, `CH`, `IA`; `admin.role_grants`), Phase 02 (`EvidenceStore::stage`, `put_generated`, `open`, `RenderService::render`, the evidence owner registry, `admin.evidence_objects`, `AuditSink`), Phase 03 (Cedar bundle and schema, the purpose codes), Phase 04 (`WorkflowEngine::start` and `act`, `StartSpec`, `WorkflowSubject`, `TerminalEffect`, `Holders`, the registries, `admin.change_requests`, `admin.workflow_instances`), Phase 07 (the profile registry `rpt.goaml_profiles`, `CheckService`, the layers L1 to L5, the rule catalogue, `Scorer`, `RejectionParser` (a stub this phase completes), the regression corpus, the schema `rpt`), Phase 09 (`ref.fx_rates` and `rate_on`, `ref.governorates`, `ref.countries`, `BusinessClock::add` and `BankCalendarClock`, the closed `ClockRegistry`, `ref.reason_codes` families `decision` and `closure`, `PolicyPackService::scan` with scope `reason_text`), Phase 10 (canonical tables `canon.txn`, `canon.cash_event`, `canon.account`, `canon.account_party_role`, `canon.customer`, `canon.party`, `canon.party_name`, `canon.party_identifier`, `canon.party_contact`, `canon.balance_snapshot`), Phase 17 (`casework.cases`, `casework.effective_tier`, the visibility predicate), Phase 18 (`casework.proposals`, `casework.decisions`, `casework.case_acl`, the workflow definitions `case_report_approval` and `case_close_no_report`, `CaseWorkflowSubject`, `RecordDecisionEffect`, `ReleaseForFilingEffect` (the body replaced here), the close gate, the sealed-case rule, the lock order of section 3.16), Phase 19 (`casework.authority_requests`, `casework.internal_reports` with channel `liaison_monthly`, `AlarmService`, the post-filing watch of a UTR), Phase 20 (`LetterRequester` port and `RecordingLetterRequester` stub, which this phase replaces; the freeze and refrain rows that carry letter ids)

**Macro-phase:** P1
**Complexity:** XL (32 routes: 18 rows of API.md 3.13 owned here, 14 rows added; 12 tables and 2 sequences in 3 migrations; 1 workflow definition added and 2 terminal effects, 1 subject check and 1 phase 18 close gate changed; 10 job kinds; 2 timer kinds)
**Binaries touched:** akr-api (routers `reports`, `filings`, `ctr`, `letters`, `periodic`, `fiu_feedback`, `report_tasks`; `ReportService`, `FilingService` and `LetterService` in `AppState`; `LetterService` replaces `RecordingLetterRequester`) | akr-worker (jobs `report_validate`, `report_package`, `ctr_daily`, `ctr_fx_wait`, `ctr_package`, `letter_render`, `periodic_build`, `report_clock_scan`, `report_calibration`, `letter_backfill`; the two timer handlers of section 4.2; `GoamlPackager` and the XML build run here) | akr-ingestd and akr-assess unchanged
**Requirements carried:**
- Implemented here: RP-11 (report types STR, UTR, TFR and CTR; SAR and AIF where the active profile lists them, through the same builder), RP-14 (chronological transaction table with a per-transaction suspicious marker), RP-15 (content gates before approval), RP-16 (activity-based reports for SAR and for UTR and AIF without transactions), RP-17 (party, account and transaction mapping: Iraqi name splitting, mother's name, year-only birth dates, governorate to `state`), RP-19 (related reports as structured links, rendered in the narrative's first lines), RP-21 (Arabic-first builder with an English toggle and the 8,000-character limit of the profile), RP-22 (narrative templates with an auto-generated chronology), RP-24 (locked versions at every approval; corrections create linked versions), RP-25 (attachments: type and size checks, hashing, a list in the narrative), RP-26 (the UNKNOWN convention per field), RP-27 (validation layers L0 to L5 of the phase 07 engine plus L0 on the case snapshot), RP-29 (estimated grade on each draft; the monthly calibration sample), RP-32 (feedback register per report), RP-34 (report state machine of the phase 21 statuses; maker is not checker; only M signs for filing; approval bound to the manifest hash, step-up `signing`), RP-36 (the exception for a single compliance officer: a second named signatory for TFR and large STRs, recorded as an exception), RP-37 (roles of the reporting module; branch roles excluded), RP-38 (package: XML, attachments, bilingual PDF, SHA-256 manifest, validation report), RP-39 (no goAML credentials stored, no portal automation; the manual upload is recorded), RP-40 (controlled download with step-up `export`, purpose and audit; the watermark is an Open doubt), RP-41 (acknowledgement capture that locks the report), RP-42 (completes the phase 07 `RejectionParser`: findings against the stored XML, corrected version, rejection and fix retained), RP-43 (returned-for-completion task from a poor grade), RP-48 (suspicion-formed date on every report), RP-49 (regulatory calendar of recurring reports), RP-50 (daily CTR job with completeness check and control totals), RP-51 (threshold on the USD equivalent at the CBI rate of the day; aggregation default on), RP-52 (file splitting, reconciliation report, checker confirmation, correction files, exception log), RP-53 (CTR quality rules), RP-56 (the freeze report and second report letters: the generator and the channel setting; the clocks are phase 20's), RP-60 (every report object in the suspicion tier with case access lists and existence masking), RP-61 (exports with purpose and approval, logged, hidden from branch roles, excluded from search and counts), RP-63 (evidence pack per report: the sealed objects of section 3.9 and the approvals), RP-68 (CBI monthly, liaison, board and MLRO annual packs as generated documents from data; the bank edits the MLRO template), RP-79 (portal capture at filing time), RP-80 (status model mirroring goAML Web: the phase 21 statuses and the Office's grade), RP-84 (feedback-loop register), M-30 (ReportRequest and ReportResult contract with the builder; FIU grade creates a rework task and a feedback entry), M-31 (decision-to-filing clock with the 24-hour escalation, and the first-alert-to-filing distribution), M-41 (age of the underlying activity at filing and its share older than N months)
- Partly implemented (the rest is in Open doubts with its target phase): RP-18 (the indicator map is consumed; the 62-code map is an Open doubt), RP-20 (AIF through the builder; the AIF and statement builder is an Open doubt), RP-29 (the calibration sample is a monthly job; the Office's own grades are the comparison when they arrive), RP-31 (timeliness metrics: the intervals are computed; the explanation threshold is a setting), RP-44 (the Office's requests are registered by phase 19; the reply letter is here), RP-46 and RP-47 (the report clocks of section 3.18; the freeze clocks are phase 20's and the branch liaison clock is phase 22's), RP-57 (the CSV annex of the freeze letter is an Open doubt), RP-62 (phase 09 owns the library; this phase adds report notices to the inference test), RP-64 and RP-65 and RP-66 (traceability and the closed-without-filing report are analytics of section 3.17; the three views are the R1 and R2 screens), RP-67 (classes assigned here, the sweep is phase 23), RP-69 (FATCA and CRS periodic kinds exist; their sources are not in the canonical copy: Open doubts), RP-70 (correspondent pack: Open doubts), RP-76 (analytics of section 3.17; the small-number suppression is here)
- Not carried here: RP-23 (AI drafting, P2, phases 27 and 28), RP-33 (dry run: the Iraq profile keeps it off; no test environment exists), RP-45 and RP-55 and RP-58 and RP-59 (phase 20), RP-54 (phase 14), RP-71 to RP-75 and RP-77 (P3 regime packs and the discovery checklist), RP-78 (pilot letter, a document for the bank), RP-81 (semi-automation output: Open doubts), R-01 to R-61 (P3 regime packs), M-27 (phase 18 section 3.7; see Open doubts 26)
- Gap pass 1 (coverage), partly implemented: E-03 (identifiers, names and the L0 completeness check), E-04 (filing interval metrics), E-05 (the FIU feedback register and the calibration sample), E-06 (CTR batches at USD 10,000 and the freeze letters); K-23 (PEP items a to e in the CBI monthly pack, section 7+); L-11 (the daily cash recount with control totals); UX-51 (scheduled regulatory reports in both languages); VA-09 (per-scenario contribution figures), VO-01 (aggregate analytics with suppression); VB-43 (draft, approval, filing, feedback register and frozen versions), VD-40 (the yearly MLRO facts pack).
- Gap pass 1 (coverage), not carried: RP-35 (the officer registry and its notices are the bank's process, DR-394); VA-49 (published claims ledger, DR-402).
- Gap pass 1 (coverage), implemented here: UX-62 (the report builder of mockup 07 with the filed-and-acknowledged step); VA-14 (branch report, acknowledgement, MLRO decision, goAML report and the quality check), VA-30 (goAML from the profile registry with versions, field-linked errors, package, status and rejections), VO-11 (the goAML builder validated against the Office schema with the rejection path); VD-17 (goAML STR, TFR and CTR output validated with a quality grade).

---

## 1. SQLx Migration Changes

Three migrations, `0068_rpt_reports`, `0069_rpt_letters_periodic` and `0070_rpt_guards_grants`, numbered after phase 20's `0067` (no other phase uses 0068 to 0070). The schema `rpt` exists (phase 07); the tables created here are `rpt.ctr_batches`, `rpt.reports`, `rpt.report_versions`, `rpt.report_findings`, `rpt.report_packages`, `rpt.report_links`, `rpt.filings`, `rpt.fiu_feedback`, `rpt.report_tasks`, `rpt.ctr_items`, `rpt.letters`, `rpt.periodic_reports`, and the sequences `rpt.report_ref_seq` and `rpt.letter_no_seq`. No existing table is altered: phase 07's `rpt.goaml_*` tables are referenced (`rpt.goaml_profiles.id`) and not redefined.

**Reconciliation with DATA-MODEL section 14 (aligned in the same change, DR-169 and DR-176):**
- `reports`: DATA-MODEL's `id, type, case_id, profile_version_id, status, quality_score, office_grade, version, created_at`. Kept, with `profile_version_id` named `profile_id` (it is the phase 07 `rpt.goaml_profiles` row, which is the version). Added: `report_ref` (the RP-type-year-sequence number, DATA-MODEL section 14 names the format), `proposal_id` and `workflow_instance_id` (the approval), `ctr_batch_id`, `subject_party_id`, `current_version`, `frozen_version`, `suspicion_formed_on` (RP-48), `quality_grade` (estimated, DR-72), `office_grade_at`, the submit and sign pairs, `created_by`, `updated_at`. The DATA-MODEL `status` list is kept; its states `draft`, `validated`, `signed`, `packaged`, `filed`, `acknowledged`, `rejected`, `returned` are the only ones (RP-34 and RP-80 names that DATA-MODEL does not carry, such as "checker-approved" and "accepted with errors", are Open doubts 6).
- `report_versions` and `report_packages`: DATA-MODEL's columns. `content jsonb` becomes `content_enc` (sealed, because the draft holds party names and the narrative; the phase 18 `text_enc` pattern, AAD = the version row id). `xml_object_key` is not stored: the XML is rebuilt from the frozen content and checked against the manifest hash; the bytes live in the package ZIP. `object_key` becomes `package_evidence_id` (phase 02 evidence object, sealed under the retention class of the report).
- `filings`: DATA-MODEL's columns. `portal_capture_object_key` becomes `portal_capture_evidence_id` (RP-79). Added: `report_version`, `package_id`, `ack_by`, `ack_evidence_id`, `activity_age_months` (M-41).
- `fiu_feedback`: DATA-MODEL's columns. `findings jsonb` becomes `finding_ids uuid[]` (the findings are rows of `report_findings`), `changes_made` is kept as text limited to 2,000 characters and checked by the policy scan. Added: `filing_id`, `capture_evidence_id` (the Office text as received), `rule_change_id` (RP-84).
- `ctr_batches` and `ctr_items`: DATA-MODEL puts one customer, one day and the cash totals in one `ctr_batches` table. This phase splits it: `ctr_batches` is one row per business date and run (the GL tie-out, the control totals and the confirmation), and `ctr_items` is one row per customer per business day (or per transaction when aggregation is off, RP-51), with the USD and IQD totals and the rate date.
- `letters`: DATA-MODEL's `letters.kind` list is kept as it is: `freeze_report`, `freeze_second_report`, `office_reply`, `hold_reply`, `referral`. The phase 20 request kinds map as follows: `CommitteeFreezeReport` to `freeze_report`, `SecondFreezeReport` to `freeze_second_report`, `CommitteeReferral` to `referral`; the refrain referral is also `referral` with `source_kind = refrain`. `case_id`, `number` (now `letter_no`, from `rpt.letter_no_seq`) and `channel` are kept; `body jsonb` becomes `facts jsonb` (ids, codes, counts and dates only, so that the letter can be regenerated from its owner) and the rendered text lives in the PDF evidence object. `recipient` becomes `recipient_code`.
- `periodic_reports`: DATA-MODEL's columns, with `object_key` as `evidence_id`, the period bounds and `suppressed_cells` (RP-76). The kinds are DATA-MODEL's six and `analytics` (the reopen-rate, contribution and reason-code distribution reports that phases 18 and 19 name).
- Not in DATA-MODEL: `report_findings` (the validation panel and the Office findings, one row per finding per validation run), `report_links` (RP-19), `report_tasks` (the returned-for-completion task of RP-43, and the two tasks that need an action: a day without an FX rate, and the checker's control confirmation of a CTR batch). Each is explained in its DDL comment.
- Retention: section 1.3 gives the classes. The reports and STRs are R6 (ARCHITECTURE section 17: cases and STRs, seven years from the later of closure and report); the letters that concern a freeze or a refrain are R7 (screening and sanctions records, the class phase 20 uses for the freeze rows).

### 1.1 `0068_rpt_reports.sql`

```sql
-- migration 0068_rpt_reports.sql
-- Phase 21. Report records of the reporting module (M13): the daily CTR batches, the reports and their frozen versions, the
-- validation findings, packages, links, filings, Office feedback, the report tasks and the CTR items. Schema rpt (phase 07).
-- Ids are UUIDv7 generated by the application; no column has a database default for its id.

CREATE SEQUENCE IF NOT EXISTS rpt.report_ref_seq;

-- Daily CTR batches (RP-50, RP-52). One live batch per business date (the partial index below); a rerun creates the next run_no
-- and the earlier run is superseded, never deleted.
CREATE TABLE IF NOT EXISTS rpt.ctr_batches (
    id                  UUID PRIMARY KEY,
    business_date       DATE NOT NULL,
    run_no              INT NOT NULL CHECK (run_no >= 1),
    status              TEXT NOT NULL CHECK (status IN ('queued', 'waiting_source', 'waiting_fx', 'computed', 'confirmed',
                                                        'packaged', 'superseded', 'failed')),
    source_txn_count    INT NOT NULL DEFAULT 0 CHECK (source_txn_count >= 0),
    reportable_count    INT NOT NULL DEFAULT 0 CHECK (reportable_count >= 0),
    excluded_count      INT NOT NULL DEFAULT 0 CHECK (excluded_count >= 0),
    total_usd           NUMERIC(28,6) NOT NULL DEFAULT 0 CHECK (total_usd >= 0),
    control_totals      JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(control_totals) = 'object'),   -- counts and sums by currency and branch
    gl_tieout           JSONB NOT NULL DEFAULT '{"status": "not_available"}'::jsonb CHECK (jsonb_typeof(gl_tieout) = 'object'),
    created_by          UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- NULL = the daily job
    confirmed_by        UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    confirmed_at        TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    CONSTRAINT uq_ctr_batches_run UNIQUE (business_date, run_no),
    CHECK ((confirmed_by IS NULL) = (confirmed_at IS NULL)),
    CHECK (confirmed_by IS NULL OR created_by IS NULL OR confirmed_by <> created_by),
    CHECK (status NOT IN ('confirmed', 'packaged') OR confirmed_at IS NOT NULL)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_ctr_batches_live ON rpt.ctr_batches (business_date) WHERE status NOT IN ('superseded', 'failed');

-- Reports (DATA-MODEL s.14, adjusted: see the reconciliation above). A UTR, STR, TFR or SAR is one row per proposal (the
-- partial unique index); a CTR is one row per file of a CTR batch; an AIF is one row per Office request.
CREATE TABLE IF NOT EXISTS rpt.reports (
    id                    UUID PRIMARY KEY,
    report_ref            TEXT NOT NULL UNIQUE CHECK (report_ref ~ '^RP-(STR|SAR|TFR|UTR|CTR|AIF)-[0-9]{4}-[0-9]{4,}$'),
    type                  TEXT NOT NULL CHECK (type IN ('str', 'sar', 'tfr', 'utr', 'ctr', 'aif')),
    case_id               UUID REFERENCES casework.cases(id) ON DELETE RESTRICT,
    proposal_id           UUID REFERENCES casework.proposals(id) ON DELETE RESTRICT,
    workflow_instance_id  UUID REFERENCES admin.workflow_instances(id) ON DELETE RESTRICT,
    ctr_batch_id          UUID REFERENCES rpt.ctr_batches(id) ON DELETE RESTRICT,
    subject_party_id      UUID,                                  -- the main subject; canonical party, bi-temporal, no foreign key
    profile_id            UUID NOT NULL REFERENCES rpt.goaml_profiles(id) ON DELETE RESTRICT,   -- the pinned profile version (phase 07)
    status                TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'validated', 'signed', 'packaged', 'filed',
                                                                          'acknowledged', 'rejected', 'returned')),
    current_version       INT NOT NULL DEFAULT 1 CHECK (current_version >= 1),
    frozen_version        INT CHECK (frozen_version IS NULL OR frozen_version >= 1),
    suspicion_formed_on   DATE,                                  -- RP-48, separate from the transaction and filing dates
    quality_score         SMALLINT CHECK (quality_score IS NULL OR quality_score BETWEEN 0 AND 100),
    quality_grade         TEXT CHECK (quality_grade IN ('good', 'moderate', 'acceptable', 'poor')),   -- estimated (DR-72)
    office_grade          TEXT CHECK (office_grade IN ('good', 'moderate', 'acceptable', 'poor')),
    office_grade_at       TIMESTAMPTZ,
    submitted_at          TIMESTAMPTZ,
    submitted_by          UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    signed_at             TIMESTAMPTZ,
    signed_by             UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_by            UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    version               INT NOT NULL DEFAULT 1,
    CHECK (type = 'ctr' OR (case_id IS NOT NULL AND proposal_id IS NOT NULL AND ctr_batch_id IS NULL)),
    CHECK (type <> 'ctr' OR (case_id IS NULL AND proposal_id IS NULL AND ctr_batch_id IS NOT NULL)),
    CHECK ((submitted_at IS NULL) = (submitted_by IS NULL)),
    CHECK ((signed_at IS NULL) = (signed_by IS NULL)),
    CHECK (frozen_version IS NULL OR submitted_at IS NOT NULL),
    CHECK (status IN ('draft', 'validated') OR signed_at IS NOT NULL OR type = 'ctr'),   -- a CTR file is approved by the batch confirmation (section 3.14)
    CHECK (signed_by IS NULL OR submitted_by IS NULL OR signed_by <> submitted_by),   -- maker is not checker (RP-34)
    CHECK (quality_grade IS NULL OR quality_score IS NOT NULL)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_reports_proposal ON rpt.reports (proposal_id) WHERE proposal_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_reports_case      ON rpt.reports (case_id) WHERE case_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_reports_status    ON rpt.reports (status, updated_at);
CREATE INDEX IF NOT EXISTS idx_reports_type      ON rpt.reports (type, created_at);
CREATE INDEX IF NOT EXISTS idx_reports_batch     ON rpt.reports (ctr_batch_id) WHERE ctr_batch_id IS NOT NULL;

-- Versions (RP-24). The builder content is sealed (AAD = the version id). A version is frozen at submit and never changes
-- afterwards (guard in 0070). Versions of one report are numbered 1, 2, ...; a correction after a rejection is a new version.
CREATE TABLE IF NOT EXISTS rpt.report_versions (
    id                  UUID PRIMARY KEY,
    report_id           UUID NOT NULL REFERENCES rpt.reports(id) ON DELETE RESTRICT,
    version             INT NOT NULL CHECK (version >= 1),
    content_enc         BYTEA NOT NULL,                             -- the builder content, sealed (personal)
    content_sha256      BYTEA NOT NULL CHECK (octet_length(content_sha256) = 32),
    validation_summary  JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(validation_summary) = 'object'),   -- counts by layer and severity only
    validated_run_id    UUID,                                       -- the latest validation run (a job id; no foreign key)
    validated_at        TIMESTAMPTZ,
    frozen_at           TIMESTAMPTZ,
    created_by          UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_report_versions UNIQUE (report_id, version),
    CHECK ((validated_run_id IS NULL) = (validated_at IS NULL))
);

-- Validation findings (RP-27, RP-42). One row per finding per validation run; the panel reads the latest run of the version.
-- element_path and params never carry a value from the file or from the draft (phase 07 rule); field_key names the builder field.
CREATE TABLE IF NOT EXISTS rpt.report_findings (
    id                  UUID PRIMARY KEY,
    report_id           UUID NOT NULL REFERENCES rpt.reports(id) ON DELETE RESTRICT,
    report_version      INT NOT NULL,
    validation_run_id   UUID NOT NULL,
    source              TEXT NOT NULL CHECK (source IN ('validation', 'office')),
    layer               TEXT NOT NULL CHECK (layer IN ('L0', 'L1', 'L2', 'L3', 'L4', 'L5', 'office')),
    rule_code           TEXT NOT NULL CHECK (length(rule_code) BETWEEN 1 AND 40),
    severity            TEXT NOT NULL CHECK (severity IN ('block', 'warn', 'info')),
    element_path        TEXT CHECK (element_path IS NULL OR length(element_path) <= 300),
    field_key           TEXT CHECK (field_key IS NULL OR length(field_key) <= 120),
    line_no             INT CHECK (line_no IS NULL OR line_no >= 1),
    col_no              INT CHECK (col_no IS NULL OR col_no >= 1),
    params              JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(params) = 'object'),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT fk_report_findings_version FOREIGN KEY (report_id, report_version)
        REFERENCES rpt.report_versions (report_id, version) ON DELETE RESTRICT
);
CREATE INDEX IF NOT EXISTS idx_report_findings_run ON rpt.report_findings (report_id, report_version, validation_run_id);

-- Packages (RP-38). One package per frozen version. The ZIP holds the XML, the attachments and the manifest; the PDF is the
-- bilingual rendering. Both are sealed through put_generated (phase 02); the manifest holds the hashes of the XML and the attachments.
CREATE TABLE IF NOT EXISTS rpt.report_packages (
    id                    UUID PRIMARY KEY,
    report_id             UUID NOT NULL REFERENCES rpt.reports(id) ON DELETE RESTRICT,
    report_version        INT NOT NULL,
    manifest              JSONB NOT NULL CHECK (jsonb_typeof(manifest) = 'object'),   -- files, SHA-256 each, profile version, validation run
    manifest_sha256       BYTEA NOT NULL CHECK (octet_length(manifest_sha256) = 32),
    package_evidence_id   UUID NOT NULL REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    pdf_evidence_id       UUID NOT NULL REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    built_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_report_packages UNIQUE (report_id, report_version),
    CONSTRAINT fk_report_packages_version FOREIGN KEY (report_id, report_version)
        REFERENCES rpt.report_versions (report_id, version) ON DELETE RESTRICT
);

-- Links between reports (RP-19): previous, amending, superseding and the STR or SAR escalated from a UTR (which cites its number).
CREATE TABLE IF NOT EXISTS rpt.report_links (
    id                  UUID PRIMARY KEY,
    report_id           UUID NOT NULL REFERENCES rpt.reports(id) ON DELETE RESTRICT,
    related_report_id   UUID NOT NULL REFERENCES rpt.reports(id) ON DELETE RESTRICT,
    relation            TEXT NOT NULL CHECK (relation IN ('previous', 'amending', 'superseding', 'escalated_from_utr')),
    created_by          UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (report_id <> related_report_id),
    CONSTRAINT uq_report_links UNIQUE (report_id, related_report_id, relation)
);

-- Filings (M-31, M-41, RP-39, RP-41, RP-79). Written once by the named officer; the acknowledgement is written once later.
CREATE TABLE IF NOT EXISTS rpt.filings (
    id                          UUID PRIMARY KEY,
    report_id                   UUID NOT NULL REFERENCES rpt.reports(id) ON DELETE RESTRICT,
    report_version              INT NOT NULL,
    package_id                  UUID NOT NULL REFERENCES rpt.report_packages(id) ON DELETE RESTRICT,
    goaml_reference             TEXT NOT NULL UNIQUE CHECK (length(btrim(goaml_reference)) BETWEEN 1 AND 60),
    filed_by                    UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    filed_at                    TIMESTAMPTZ NOT NULL,
    portal_capture_evidence_id  UUID NOT NULL REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    activity_age_months         INT CHECK (activity_age_months IS NULL OR activity_age_months >= 0),
    ack_at                      TIMESTAMPTZ,
    ack_by                      UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    ack_evidence_id             UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    created_at                  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at                  TIMESTAMPTZ NOT NULL DEFAULT now(),
    version                     INT NOT NULL DEFAULT 1,
    CONSTRAINT uq_filings_version UNIQUE (report_id, report_version),
    CONSTRAINT fk_filings_version FOREIGN KEY (report_id, report_version)
        REFERENCES rpt.report_versions (report_id, version) ON DELETE RESTRICT,
    CHECK ((ack_at IS NULL) = (ack_by IS NULL)),
    CHECK ((ack_at IS NULL) = (ack_evidence_id IS NULL)),
    CHECK (ack_at IS NULL OR ack_at >= filed_at)
);
CREATE INDEX IF NOT EXISTS idx_filings_report ON rpt.filings (report_id);
CREATE INDEX IF NOT EXISTS idx_filings_open_ack ON rpt.filings (filed_at) WHERE ack_at IS NULL;

-- FIU feedback register (RP-32, RP-84). Append-only. A grade is one row; a rejection is one row per Office message; a session
-- (a quality session of the Office) and a request are rows without a report.
CREATE TABLE IF NOT EXISTS rpt.fiu_feedback (
    id                    UUID PRIMARY KEY,
    report_id             UUID REFERENCES rpt.reports(id) ON DELETE RESTRICT,
    filing_id             UUID REFERENCES rpt.filings(id) ON DELETE RESTRICT,
    kind                  TEXT NOT NULL CHECK (kind IN ('grade', 'rejection', 'session', 'request')),
    grade                 TEXT CHECK (grade IS NULL OR grade IN ('good', 'moderate', 'acceptable', 'poor')),
    finding_ids           UUID[] NOT NULL DEFAULT '{}',
    capture_evidence_id   UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,   -- the Office text as received (RP-79)
    changes_made          TEXT CHECK (changes_made IS NULL OR length(changes_made) BETWEEN 1 AND 2000),
    rule_change_id        UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    received_at           TIMESTAMPTZ NOT NULL,
    recorded_by           UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK ((kind = 'grade') = (grade IS NOT NULL)),
    CHECK (kind NOT IN ('grade', 'rejection') OR report_id IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_fiu_feedback_report ON rpt.fiu_feedback (report_id, received_at);

-- Report tasks (RP-43 returned for completion; the FX day waiting for a rate, section 3.13; the checker's control confirmation
-- of a CTR batch, section 3.14). The task is a row, not a workflow instance: it has no approval, only a claim and a close.
CREATE TABLE IF NOT EXISTS rpt.report_tasks (
    id                    UUID PRIMARY KEY,
    kind                  TEXT NOT NULL CHECK (kind IN ('returned_for_completion', 'fx_rate_missing', 'ctr_control_check')),
    report_id             UUID REFERENCES rpt.reports(id) ON DELETE RESTRICT,
    feedback_id           UUID REFERENCES rpt.fiu_feedback(id) ON DELETE RESTRICT,
    ctr_batch_id          UUID REFERENCES rpt.ctr_batches(id) ON DELETE RESTRICT,
    business_date         DATE,
    currency              CHAR(3) REFERENCES ref.currencies(code) ON DELETE RESTRICT,
    state                 TEXT NOT NULL DEFAULT 'open' CHECK (state IN ('open', 'claimed', 'done')),
    due_at                TIMESTAMPTZ,
    claimed_by            UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    claimed_at            TIMESTAMPTZ,
    closed_by             UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    closed_at             TIMESTAMPTZ,
    closure_reason_code   TEXT REFERENCES ref.reason_codes(code) ON DELETE RESTRICT,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    version               INT NOT NULL DEFAULT 1,
    CHECK ((kind = 'returned_for_completion') = (report_id IS NOT NULL AND feedback_id IS NOT NULL AND due_at IS NOT NULL)),
    CHECK ((kind = 'fx_rate_missing') = (business_date IS NOT NULL AND currency IS NOT NULL)),
    CHECK ((kind = 'ctr_control_check') = (ctr_batch_id IS NOT NULL)),
    CHECK (state <> 'claimed' OR claimed_by IS NOT NULL),
    CHECK ((state = 'done') = (closed_at IS NOT NULL)),
    CHECK ((closed_at IS NULL) = (closed_by IS NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_report_tasks_fx       ON rpt.report_tasks (business_date, currency)
    WHERE kind = 'fx_rate_missing' AND state <> 'done';
CREATE UNIQUE INDEX IF NOT EXISTS uq_report_tasks_feedback ON rpt.report_tasks (feedback_id)
    WHERE kind = 'returned_for_completion';
CREATE INDEX IF NOT EXISTS idx_report_tasks_open ON rpt.report_tasks (state, due_at) WHERE state <> 'done';

-- CTR items (RP-51, RP-53). One row per customer per business date when aggregation is on (the default); one row per
-- transaction when it is off. The rate is always the CBI rate of the business date (rate_date = business_date).
CREATE TABLE IF NOT EXISTS rpt.ctr_items (
    id                UUID PRIMARY KEY,
    batch_id          UUID NOT NULL REFERENCES rpt.ctr_batches(id) ON DELETE RESTRICT,
    grain             TEXT NOT NULL CHECK (grain IN ('customer_day', 'transaction')),
    customer_id       UUID,                                        -- the conductor's customer; canonical, bi-temporal, no foreign key; NULL = not identified
    business_date     DATE NOT NULL,
    txn_refs          JSONB NOT NULL CHECK (jsonb_typeof(txn_refs) = 'array' AND jsonb_array_length(txn_refs) >= 1),   -- [{id, booking_date}]
    deposits_usd      NUMERIC(28,6) NOT NULL CHECK (deposits_usd >= 0),
    withdrawals_usd   NUMERIC(28,6) NOT NULL CHECK (withdrawals_usd >= 0),
    total_usd         NUMERIC(28,6) NOT NULL CHECK (total_usd >= 0),
    total_lcy         NUMERIC(28,6) NOT NULL CHECK (total_lcy >= 0),          -- IQD
    rate_date         DATE NOT NULL,
    rate_source       TEXT NOT NULL CHECK (rate_source IN ('cbi', 'manual')),
    reportable        BOOLEAN NOT NULL,
    exclusion_code    TEXT REFERENCES ref.reason_codes(code) ON DELETE RESTRICT,
    report_id         UUID REFERENCES rpt.reports(id) ON DELETE RESTRICT,
    file_no           INT CHECK (file_no IS NULL OR file_no >= 1),
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (total_usd = deposits_usd + withdrawals_usd),
    CHECK (rate_date = business_date),
    CHECK (reportable OR exclusion_code IS NOT NULL),
    CHECK (reportable OR report_id IS NULL),
    CHECK ((report_id IS NULL) = (file_no IS NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_ctr_items_customer_day ON rpt.ctr_items (batch_id, customer_id)
    WHERE grain = 'customer_day' AND customer_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_ctr_items_batch ON rpt.ctr_items (batch_id, business_date);
```

### 1.2 `0069_rpt_letters_periodic.sql`

```sql
-- migration 0069_rpt_letters_periodic.sql
-- Phase 21. Official letters (the Committee and Office letters that phases 19 and 20 request; the Office replies) and the
-- periodic packs (CBI, liaison, board, MLRO annual, FATCA, CRS and the analytics pack).

CREATE SEQUENCE IF NOT EXISTS rpt.letter_no_seq;

-- Letters (RP-56, phase 20 section 3.6). A letter is requested inside the transaction of its owner (a freeze, a refrain, an
-- authority request or a report) and rendered by a worker job; it is numbered when requested. Its facts are ids, codes,
-- counts and dates: the text is rendered from the owner row when the letter is rendered, never stored in this table.
CREATE TABLE IF NOT EXISTS rpt.letters (
    id                  UUID PRIMARY KEY,
    letter_no           TEXT NOT NULL UNIQUE CHECK (letter_no ~ '^AKR-LTR-[0-9]{4}-[0-9]{6}$'),
    kind                TEXT NOT NULL CHECK (kind IN ('freeze_report', 'freeze_second_report', 'referral', 'office_reply', 'hold_reply')),
    source_kind         TEXT NOT NULL CHECK (source_kind IN ('freeze', 'refrain', 'authority_request', 'report')),
    source_id           UUID NOT NULL,                               -- freezes.id, refrain_instructions.id, authority_requests.id or reports.id; no foreign key
    case_id             UUID REFERENCES casework.cases(id) ON DELETE RESTRICT,
    recipient_code      TEXT NOT NULL CHECK (recipient_code IN ('committee', 'office', 'authority')),
    copy_to_office      BOOLEAN NOT NULL DEFAULT FALSE,
    facts               JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(facts) = 'object'),
    state               TEXT NOT NULL DEFAULT 'requested' CHECK (state IN ('requested', 'rendered', 'signed', 'delivered')),
    pdf_evidence_id     UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    requested_by        UUID REFERENCES admin.users(id) ON DELETE RESTRICT,     -- NULL = a timer, a service or the backfill
    requested_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    signed_by           UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    signed_at           TIMESTAMPTZ,
    channel             TEXT CHECK (channel IS NULL OR channel IN ('letter', 'email')),
    delivered_by        UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    delivered_at        TIMESTAMPTZ,
    delivery_reference  TEXT CHECK (delivery_reference IS NULL OR length(btrim(delivery_reference)) BETWEEN 1 AND 60),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    CHECK ((signed_at IS NULL) = (signed_by IS NULL)),
    CHECK ((delivered_at IS NULL) = (delivered_by IS NULL)),
    CHECK (state = 'requested' OR pdf_evidence_id IS NOT NULL),
    CHECK (state NOT IN ('signed', 'delivered') OR signed_at IS NOT NULL),
    CHECK (state <> 'delivered' OR (delivered_at IS NOT NULL AND channel IS NOT NULL AND delivery_reference IS NOT NULL)),
    CHECK (signed_by IS NULL OR requested_by IS NULL OR signed_by <> requested_by)
);
CREATE INDEX IF NOT EXISTS idx_letters_source  ON rpt.letters (source_kind, source_id);
CREATE INDEX IF NOT EXISTS idx_letters_case    ON rpt.letters (case_id) WHERE case_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_letters_pending ON rpt.letters (requested_at) WHERE state <> 'delivered';

-- Periodic packs (RP-68, RP-49, RP-76). Generated from data as documents; facts are aggregates with small-number suppression
-- (the cell is null and counted in suppressed_cells). One live pack per kind and period.
CREATE TABLE IF NOT EXISTS rpt.periodic_reports (
    id                  UUID PRIMARY KEY,
    kind                TEXT NOT NULL CHECK (kind IN ('cbi_monthly', 'liaison', 'board', 'mlro_annual', 'fatca', 'crs', 'analytics')),
    period              TEXT NOT NULL CHECK (period ~ '^[0-9]{4}(-(0[1-9]|1[0-2]))?$'),   -- YYYY-MM, or YYYY for annual packs
    period_start        DATE NOT NULL,
    period_end          DATE NOT NULL,
    status              TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'final', 'superseded')),
    template_version    TEXT NOT NULL CHECK (length(template_version) BETWEEN 1 AND 40),
    facts               JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(facts) = 'object'),
    suppressed_cells    INT NOT NULL DEFAULT 0 CHECK (suppressed_cells >= 0),
    evidence_id         UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    generated_by        UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    signed_by           UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    signed_at           TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    CHECK (period_end >= period_start),
    CHECK ((signed_at IS NULL) = (signed_by IS NULL)),
    CHECK (status <> 'final' OR (signed_at IS NOT NULL AND evidence_id IS NOT NULL)),
    CHECK (signed_by IS NULL OR generated_by IS NULL OR signed_by <> generated_by)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_periodic_live ON rpt.periodic_reports (kind, period) WHERE status <> 'superseded';
```

### 1.3 `0070_rpt_guards_grants.sql`

```sql
-- migration 0070_rpt_guards_grants.sql
-- Phase 21. Guards that the application cannot bypass, grants per write path, settings, reason codes, classification and
-- retention. Idempotent: triggers and functions are dropped before they are created.

-- Alarm register (phase 19 section 1.1, widened by phase 20 migration 0066). Two report kinds and two subject kinds are added;
-- the phase 20 values are repeated so that the constraint keeps every earlier kind valid. Drop by name, then add (idempotent).
ALTER TABLE casework.alarms DROP CONSTRAINT IF EXISTS alarms_kind_check;
ALTER TABLE casework.alarms ADD CONSTRAINT alarms_kind_check CHECK (kind IN (
    'touch_breach', 'work_item_breach', 'chain_broken', 'conflict_declared', 'flat_line', 'qa_critical',
    'post_order_debit', 'freeze_overdue', 'refrain_overdue', 'report_filing_overdue', 'report_rework_overdue'));
ALTER TABLE casework.alarms DROP CONSTRAINT IF EXISTS alarms_subject_kind_check;
ALTER TABLE casework.alarms ADD CONSTRAINT alarms_subject_kind_check CHECK (subject_kind IN (
    'alert', 'work_item', 'case', 'scenario', 'qa_review', 'freeze', 'refrain', 'ops_task', 'report', 'report_task'));

-- Append-only tables (no UPDATE, no DELETE).
CREATE OR REPLACE FUNCTION rpt.append_only() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION '% is append only', TG_TABLE_NAME;
END $$;
DROP TRIGGER IF EXISTS trg_report_findings_append ON rpt.report_findings;
CREATE TRIGGER trg_report_findings_append BEFORE UPDATE OR DELETE ON rpt.report_findings
    FOR EACH ROW EXECUTE FUNCTION rpt.append_only();
DROP TRIGGER IF EXISTS trg_report_packages_append ON rpt.report_packages;
CREATE TRIGGER trg_report_packages_append BEFORE UPDATE OR DELETE ON rpt.report_packages
    FOR EACH ROW EXECUTE FUNCTION rpt.append_only();
DROP TRIGGER IF EXISTS trg_report_links_append ON rpt.report_links;
CREATE TRIGGER trg_report_links_append BEFORE UPDATE OR DELETE ON rpt.report_links
    FOR EACH ROW EXECUTE FUNCTION rpt.append_only();
DROP TRIGGER IF EXISTS trg_fiu_feedback_append ON rpt.fiu_feedback;
CREATE TRIGGER trg_fiu_feedback_append BEFORE UPDATE OR DELETE ON rpt.fiu_feedback
    FOR EACH ROW EXECUTE FUNCTION rpt.append_only();

-- A frozen version is immutable (RP-24); versions are never deleted.
CREATE OR REPLACE FUNCTION rpt.versions_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN RAISE EXCEPTION 'report versions are never deleted'; END IF;
    IF NEW.id <> OLD.id OR NEW.report_id <> OLD.report_id OR NEW.version <> OLD.version THEN
        RAISE EXCEPTION 'the identity of a report version does not change';
    END IF;
    IF OLD.frozen_at IS NOT NULL THEN RAISE EXCEPTION 'a frozen report version is immutable'; END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_report_versions_guard ON rpt.report_versions;
CREATE TRIGGER trg_report_versions_guard BEFORE UPDATE OR DELETE ON rpt.report_versions
    FOR EACH ROW EXECUTE FUNCTION rpt.versions_guard();

-- Reports: the approval and the frozen version are written once per submission; a revise from rejected or returned back to draft clears the approval.
CREATE OR REPLACE FUNCTION rpt.reports_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN RAISE EXCEPTION 'reports are never deleted'; END IF;
    IF NEW.id <> OLD.id OR NEW.report_ref <> OLD.report_ref OR NEW.type <> OLD.type
       OR NEW.proposal_id IS DISTINCT FROM OLD.proposal_id OR NEW.profile_id <> OLD.profile_id THEN
        RAISE EXCEPTION 'the identity and the pinned profile of a report do not change';
    END IF;
    IF OLD.signed_at IS NOT NULL AND (NEW.signed_at IS DISTINCT FROM OLD.signed_at OR NEW.signed_by IS DISTINCT FROM OLD.signed_by)
       AND OLD.status NOT IN ('draft', 'validated') AND NOT (OLD.status IN ('rejected', 'returned') AND NEW.status = 'draft') THEN
        RAISE EXCEPTION 'a signature is written once';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_reports_guard ON rpt.reports;
CREATE TRIGGER trg_reports_guard BEFORE UPDATE OR DELETE ON rpt.reports
    FOR EACH ROW EXECUTE FUNCTION rpt.reports_guard();

-- Filings: identity fixed; the acknowledgement is written once.
CREATE OR REPLACE FUNCTION rpt.filings_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN RAISE EXCEPTION 'filings are never deleted'; END IF;
    IF NEW.id <> OLD.id OR NEW.report_id <> OLD.report_id OR NEW.report_version <> OLD.report_version
       OR NEW.package_id <> OLD.package_id OR NEW.goaml_reference <> OLD.goaml_reference
       OR NEW.filed_by <> OLD.filed_by OR NEW.filed_at <> OLD.filed_at
       OR NEW.portal_capture_evidence_id <> OLD.portal_capture_evidence_id THEN
        RAISE EXCEPTION 'the filing record is written once';
    END IF;
    IF OLD.ack_at IS NOT NULL AND (NEW.ack_at IS DISTINCT FROM OLD.ack_at OR NEW.ack_evidence_id IS DISTINCT FROM OLD.ack_evidence_id
                                   OR NEW.ack_by IS DISTINCT FROM OLD.ack_by) THEN
        RAISE EXCEPTION 'an acknowledgement is written once';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_filings_guard ON rpt.filings;
CREATE TRIGGER trg_filings_guard BEFORE UPDATE OR DELETE ON rpt.filings
    FOR EACH ROW EXECUTE FUNCTION rpt.filings_guard();

-- Letters: identity fixed; the state only moves forward; a signed letter keeps its document.
CREATE OR REPLACE FUNCTION rpt.letters_guard() RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE rank_old INT; rank_new INT;
BEGIN
    IF TG_OP = 'DELETE' THEN RAISE EXCEPTION 'letters are never deleted'; END IF;
    IF NEW.id <> OLD.id OR NEW.letter_no <> OLD.letter_no OR NEW.kind <> OLD.kind OR NEW.source_kind <> OLD.source_kind
       OR NEW.source_id <> OLD.source_id OR NEW.facts <> OLD.facts OR NEW.requested_at <> OLD.requested_at THEN
        RAISE EXCEPTION 'the identity of a letter does not change';
    END IF;
    rank_old := CASE OLD.state WHEN 'requested' THEN 1 WHEN 'rendered' THEN 2 WHEN 'signed' THEN 3 ELSE 4 END;
    rank_new := CASE NEW.state WHEN 'requested' THEN 1 WHEN 'rendered' THEN 2 WHEN 'signed' THEN 3 ELSE 4 END;
    IF rank_new < rank_old THEN RAISE EXCEPTION 'a letter state never moves back'; END IF;
    IF OLD.state IN ('signed', 'delivered') AND NEW.pdf_evidence_id IS DISTINCT FROM OLD.pdf_evidence_id THEN
        RAISE EXCEPTION 'a signed letter keeps its document';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_letters_guard ON rpt.letters;
CREATE TRIGGER trg_letters_guard BEFORE UPDATE OR DELETE ON rpt.letters
    FOR EACH ROW EXECUTE FUNCTION rpt.letters_guard();

-- CTR items of a confirmed batch are fixed, except the file number and report link set at packaging.
CREATE OR REPLACE FUNCTION rpt.ctr_items_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN RAISE EXCEPTION 'CTR items are never deleted'; END IF;
    IF EXISTS (SELECT 1 FROM rpt.ctr_batches b WHERE b.id = OLD.batch_id AND b.status IN ('confirmed', 'packaged'))
       AND (NEW.customer_id IS DISTINCT FROM OLD.customer_id OR NEW.txn_refs <> OLD.txn_refs
            OR NEW.deposits_usd <> OLD.deposits_usd OR NEW.withdrawals_usd <> OLD.withdrawals_usd
            OR NEW.total_usd <> OLD.total_usd OR NEW.total_lcy <> OLD.total_lcy OR NEW.reportable <> OLD.reportable
            OR NEW.exclusion_code IS DISTINCT FROM OLD.exclusion_code) THEN
        RAISE EXCEPTION 'CTR items of a confirmed batch are fixed';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_ctr_items_guard ON rpt.ctr_items;
CREATE TRIGGER trg_ctr_items_guard BEFORE UPDATE OR DELETE ON rpt.ctr_items
    FOR EACH ROW EXECUTE FUNCTION rpt.ctr_items_guard();

-- Grants per write path (akr_app is the API and the worker). Every UPDATE lists the columns its service writes.
GRANT SELECT, INSERT ON rpt.ctr_batches, rpt.reports, rpt.report_versions, rpt.report_findings, rpt.report_packages,
      rpt.report_links, rpt.filings, rpt.fiu_feedback, rpt.report_tasks, rpt.ctr_items, rpt.letters, rpt.periodic_reports TO akr_app;
GRANT UPDATE (confirmed_by, confirmed_at, status, source_txn_count, reportable_count, excluded_count, total_usd,
              control_totals, gl_tieout, updated_at, version) ON rpt.ctr_batches TO akr_app;
GRANT UPDATE (status, current_version, frozen_version, suspicion_formed_on, quality_score, quality_grade, office_grade,
              office_grade_at, submitted_at, submitted_by, signed_at, signed_by, updated_at, version) ON rpt.reports TO akr_app;
GRANT UPDATE (content_enc, content_sha256, validation_summary, validated_run_id, validated_at, frozen_at) ON rpt.report_versions TO akr_app;
GRANT UPDATE (ack_at, ack_by, ack_evidence_id, updated_at, version) ON rpt.filings TO akr_app;
GRANT UPDATE (state, claimed_by, claimed_at, closed_by, closed_at, closure_reason_code, due_at, updated_at, version)
      ON rpt.report_tasks TO akr_app;
GRANT UPDATE (report_id, file_no) ON rpt.ctr_items TO akr_app;
GRANT UPDATE (state, pdf_evidence_id, signed_by, signed_at, channel, delivered_by, delivered_at,
              delivery_reference, updated_at, version) ON rpt.letters TO akr_app;
GRANT UPDATE (status, facts, suppressed_cells, evidence_id, signed_by, signed_at, updated_at, version) ON rpt.periodic_reports TO akr_app;
GRANT USAGE ON SEQUENCE rpt.report_ref_seq, rpt.letter_no_seq TO akr_app;
-- No grant to akr_readonly, akr_ingest, akr_audit_writer or akr_monitor: report content is personal and is read through the API.
-- The phase 10 canonical tables and phase 09 reference tables are read by akr_app under their own grants.

-- Settings (copy of the phase 20 column list). floor is set only where governed = TRUE (phase 00 CHECK).
INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('ctr.aggregate_same_day', 'true', 'boolean', NULL, TRUE,
  '{"ar": "جمع معاملات النقد اليومية للعميل الواحد قبل اختبار حد العشرة آلاف دولار", "en": "Add up one customer''s cash on a business day before the USD 10,000 test"}'),
 ('ctr.file_max_transactions', '2000', 'integer', '{"min": 1, "basis": "Office limit not verified (research 23 s.8.11); the bank confirms it"}'::jsonb, TRUE,
  '{"ar": "أقصى عدد معاملات في ملف النقد اليومي", "en": "Maximum transactions per CTR file"}'),
 ('report.small_number_threshold', '5', 'integer', '{"min": 3, "basis": "RP-70 small-number suppression; never below 3"}'::jsonb, TRUE,
  '{"ar": "حد الكبت للأعداد الصغيرة في التقارير الدورية", "en": "Small-number suppression threshold in periodic packs"}'),
 ('report.letter_channel', '"letter"', 'string', NULL, TRUE,
  '{"ar": "قناة إرسال الخطابات الرسمية (خطاب أو بريد إلكتروني)", "en": "Delivery channel of official letters (letter or email)"}'),
 ('report.single_officer_exception', 'false', 'boolean', NULL, TRUE,
  '{"ar": "استثناء الموقّع الواحد: توقيع ثانٍ مسمّى للتقارير الكبيرة", "en": "Single-officer exception: a second named signatory for large reports"}'),
 ('report.large_str_usd', '0', 'decimal', '{"min": 0, "basis": "RP-36; the bank sets the amount; 0 means every STR needs the second signatory when the exception is on"}'::jsonb, TRUE,
  '{"ar": "حد المبلغ للتقرير الكبير (دولار)", "en": "Large report amount in USD (second signatory)"}'),
 ('report.activity_age_months_threshold', '12', 'integer', '{"min": 1, "basis": "M-41; the research gives no N; the bank sets it"}'::jsonb, TRUE,
  '{"ar": "عدد الأشهر لاعتبار النشاط قديماً عند الإبلاغ", "en": "Months after which the activity is old at filing (M-41)"}'),
 ('report.periodic_due_days', '10', 'integer', '{"min": 1, "basis": "CBI due dates are not verified (Open doubts); the bank confirms the value"}'::jsonb, TRUE,
  '{"ar": "أيام بعد نهاية الفترة لاستحقاق التقرير الدوري", "en": "Days after the period end when a periodic pack is due"}')
ON CONFLICT (key) DO NOTHING;

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('schedule.ctr_daily', '"0 30 19 * * 0-4"', 'cron', NULL, FALSE,
  '{"ar": "جدول مهمة النقد اليومي بعد إغلاق يوم العمل", "en": "Daily CTR job after the business-date close (bank time)"}'),
 ('schedule.ctr_fx_wait', '"0 15 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول إعادة محاولة أيام النقد المنتظرة لسعر الصرف", "en": "Retry of CTR days waiting for a CBI rate"}'),
 ('schedule.report_clock_scan', '"0 9 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول احتياط مؤقتات التقارير", "en": "Backstop schedule for the report clocks"}'),
 ('schedule.report_calibration', '"0 0 6 1 * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول عينة معايرة الدرجة التقديرية (أول كل شهر)", "en": "Monthly calibration sample of the estimated grade"}')
ON CONFLICT (key) DO NOTHING;

-- Reason codes (ref.reason_codes, phase 04; families closure and decision as phase 09 defines them). Seeded, never deleted.
INSERT INTO ref.reason_codes (code, family, label) VALUES
 ('CLS_CTR_EXCLUDED_CANCELLED', 'closure',
  '{"ar": "مُستبعد: معاملة ملغاة", "en": "Excluded: cancelled transaction"}'),
 ('CLS_CTR_EXCLUDED_REVERSED', 'closure',
  '{"ar": "مُستبعد: معاملة معكوسة", "en": "Excluded: reversed transaction"}'),
 ('CLS_REPORT_RESOLVED', 'closure',
  '{"ar": "أُغلقت المهمة بعد إعادة الإرسال", "en": "Closed after the corrected report was signed"}')
ON CONFLICT (code) DO NOTHING;

-- Classification (DATA-MODEL s.0 classes). content_enc is sealed (personal, encrypted); no free text of a party is stored in
-- the clear anywhere in this phase.
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'rpt', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('reports', 'internal', ARRAY['id', 'report_ref', 'type', 'case_id', 'proposal_id', 'workflow_instance_id', 'ctr_batch_id',
                                'subject_party_id', 'profile_id', 'status', 'current_version', 'frozen_version',
                                'suspicion_formed_on', 'quality_score', 'quality_grade', 'office_grade', 'office_grade_at',
                                'submitted_at', 'submitted_by', 'signed_at', 'signed_by', 'created_by', 'created_at',
                                'updated_at', 'version']),
  ('report_versions', 'personal', ARRAY['content_enc']),
  ('report_versions', 'internal', ARRAY['id', 'report_id', 'version', 'content_sha256', 'validation_summary', 'validated_run_id',
                                        'validated_at', 'frozen_at', 'created_by', 'created_at']),
  ('report_findings', 'internal', ARRAY['id', 'report_id', 'report_version', 'validation_run_id', 'source', 'layer', 'rule_code',
                                        'severity', 'element_path', 'field_key', 'line_no', 'col_no', 'params', 'created_at']),
  ('report_packages', 'internal', ARRAY['id', 'report_id', 'report_version', 'manifest', 'manifest_sha256', 'package_evidence_id',
                                        'pdf_evidence_id', 'built_at']),
  ('report_links', 'internal', ARRAY['id', 'report_id', 'related_report_id', 'relation', 'created_by', 'created_at']),
  ('filings', 'internal', ARRAY['id', 'report_id', 'report_version', 'package_id', 'goaml_reference', 'filed_by', 'filed_at',
                                'portal_capture_evidence_id', 'activity_age_months', 'ack_at', 'ack_by', 'ack_evidence_id',
                                'created_at', 'updated_at', 'version']),
  ('fiu_feedback', 'internal', ARRAY['id', 'report_id', 'filing_id', 'kind', 'grade', 'finding_ids', 'capture_evidence_id',
                                     'changes_made', 'rule_change_id', 'received_at', 'recorded_by', 'created_at']),
  ('report_tasks', 'internal', ARRAY['id', 'kind', 'report_id', 'feedback_id', 'ctr_batch_id', 'business_date', 'currency', 'state',
                                     'due_at', 'claimed_by', 'claimed_at', 'closed_by', 'closed_at', 'closure_reason_code',
                                     'created_at', 'updated_at', 'version']),
  ('ctr_batches', 'internal', ARRAY['id', 'business_date', 'run_no', 'status', 'source_txn_count', 'reportable_count',
                                    'excluded_count', 'created_by', 'confirmed_by', 'confirmed_at', 'created_at', 'updated_at', 'version']),
  ('ctr_batches', 'financial', ARRAY['total_usd', 'control_totals', 'gl_tieout']),
  ('ctr_items', 'internal', ARRAY['id', 'batch_id', 'grain', 'customer_id', 'business_date', 'txn_refs', 'rate_date', 'rate_source',
                                  'reportable', 'exclusion_code', 'report_id', 'file_no', 'created_at']),
  ('ctr_items', 'financial', ARRAY['deposits_usd', 'withdrawals_usd', 'total_usd', 'total_lcy']),
  ('letters', 'internal', ARRAY['id', 'letter_no', 'kind', 'source_kind', 'source_id', 'case_id', 'recipient_code', 'copy_to_office',
                                'facts', 'state', 'pdf_evidence_id', 'requested_by', 'requested_at', 'signed_by', 'signed_at',
                                'channel', 'delivered_by', 'delivered_at', 'delivery_reference', 'created_at', 'updated_at', 'version']),
  ('periodic_reports', 'internal', ARRAY['id', 'kind', 'period', 'period_start', 'period_end', 'status', 'template_version', 'facts',
                                         'suppressed_cells', 'evidence_id', 'generated_by', 'signed_by', 'signed_at', 'created_at',
                                         'updated_at', 'version'])
) AS v(t, cls, cols)
CROSS JOIN LATERAL unnest(v.cols) AS c
ON CONFLICT DO NOTHING;

-- Retention (phase 00 clocks; ARCHITECTURE s.17). R6 for reports, STRs, CTR and periodic packs; R7 for the letters that concern a
-- freeze or a refrain (the phase 20 class). A child row follows its owner (clock owner, clock_column NULL).
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('rpt', 'reports',            'R6', 'closure', 'created_at',    'Reports and STRs: seven years from the later of case closure and the filing (the sweep takes the later); a CTR file has no case'),
 ('rpt', 'report_versions',    'R6', 'owner',   NULL,            'Versions follow their report'),
 ('rpt', 'report_findings',    'R6', 'owner',   NULL,            'Findings follow their report version'),
 ('rpt', 'report_packages',    'R6', 'owner',   NULL,            'Packages follow their report'),
 ('rpt', 'report_links',       'R6', 'owner',   NULL,            'Links follow both reports'),
 ('rpt', 'filings',            'R6', 'owner',   NULL,            'Filing records follow their report'),
 ('rpt', 'fiu_feedback',       'R6', 'age',     'received_at',   'Office feedback register: seven years from receipt'),
 ('rpt', 'report_tasks',       'R6', 'closure', 'closed_at',     'Report tasks, from the close; an open task is never due'),
 ('rpt', 'ctr_batches',        'R6', 'age',     'business_date', 'Daily CTR batches: seven years from the business date'),
 ('rpt', 'ctr_items',          'R6', 'owner',   NULL,            'CTR items follow their batch'),
 ('rpt', 'letters',            'R7', 'closure', 'delivered_at',  'Letters concerning a freeze or a refrain (R7); a letter not yet delivered is never due'),
 ('rpt', 'periodic_reports',   'R6', 'age',     'period_end',    'Periodic packs: seven years from the period end')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

---

## 2. DDD Implementation (Rust)

Crate placement follows phases 18 to 20. The domain lives in `domain::reporting` (akr-core, no IO: the state machine of section 3.5, the quality bands, the CTR aggregation and threshold and the small-number rule are pure functions). The goAML XML builder `GoamlPackager` is a pure function in the phase 07 crate `akr-goaml`, as a new module `akr-goaml::build`; it reuses phase 07's profile types and element paths, so every finding maps back to a builder field. The repositories live in `infrastructure::reporting`, the services in `services::reporting`, the routers in `api::routes` (`reports`, `filings`, `ctr`, `letters`, `periodic`, `fiu_feedback`, `report_tasks`, `liaison`). The job handlers live in `akr-worker`.

**Domain entities** (`Debug, Serialize, Deserialize, FromRow, ToSchema` where they cross the API; the sealed content is never serialised):
- `Report { id, report_ref, report_type: ReportType, case_id, proposal_id, workflow_instance_id, ctr_batch_id, subject_party_id, profile_id, status: ReportStatus, current_version, frozen_version, suspicion_formed_on, quality_score, quality_grade, office_grade, office_grade_at, submitted_at, submitted_by, signed_at, signed_by, created_by, created_at, updated_at, version }`
- `ReportVersion { id, report_id, version, content_enc (not serialised), content_sha256, validation_summary: ValidationSummary, validated_run_id, validated_at, frozen_at, created_by, created_at }`
- `ReportFinding { id, report_id, report_version, validation_run_id, source: FindingSource, layer: FindingLayer, rule_code, severity: Severity, element_path, field_key, line_no, col_no, params: FindingParams, created_at }`. `Severity` (`block`, `warn`, `info`) and `FindingParams` (a closed typed struct with no free text) are phase 07's types.
- `ReportPackage { id, report_id, report_version, manifest: PackageManifest, manifest_sha256, package_evidence_id, pdf_evidence_id, built_at }`
- `Filing { id, report_id, report_version, package_id, goaml_reference, filed_by, filed_at, portal_capture_evidence_id, activity_age_months, ack_at, ack_by, ack_evidence_id, version }`
- `FiuFeedback { id, report_id, filing_id, kind: FeedbackKind, grade, finding_ids, capture_evidence_id, changes_made, rule_change_id, received_at, recorded_by }`
- `ReportTask { id, kind: ReportTaskKind, report_id, feedback_id, ctr_batch_id, business_date, currency, state: TaskState, due_at, claimed_by, closed_by, closed_at, closure_reason_code, version }`
- `CtrBatch { id, business_date, run_no, status: CtrBatchStatus, source_txn_count, reportable_count, excluded_count, total_usd, control_totals, gl_tieout, created_by, confirmed_by, confirmed_at, version }`
- `CtrItem { id, batch_id, grain: CtrGrain, customer_id: Option<Uuid>, business_date, txn_refs: Vec<TxnRef>, deposits_usd, withdrawals_usd, total_usd, total_lcy, rate_date, rate_source, reportable, exclusion_code, report_id, file_no }`
- `Letter { id, letter_no, kind: LetterKind, source_kind, source_id, case_id, recipient_code, copy_to_office, facts: LetterFacts, state: LetterState, pdf_evidence_id, requested_by, requested_at, signed_by, signed_at, channel, delivered_by, delivered_at, delivery_reference, version }`
- `PeriodicReport { id, kind: PeriodicKind, period, period_start, period_end, status, template_version, facts, suppressed_cells, evidence_id, generated_by, signed_by, signed_at, version }`
- `ReportContent` (sealed in `report_versions.content_enc`, AAD = the version id): `{ reason, suspicion_formed_on, origin, indicators: Vec<IndicatorCode>, action_taken, subjects: Vec<SubjectDraft>, accounts: Vec<AccountDraft>, transactions: Vec<TxnDraft>, narrative: Narrative, earlier_reports: Vec<RelatedRef>, attachments: Vec<AttachmentRef>, unknown: Vec<UnknownMark> }`. A `SubjectDraft` holds the canonical party id, the split name parts, the mother's name, the birth date and its precision, the nationality, the address with the governorate code, and the identifier references (`canon.party_identifier` ids, never the value). The sealed draft is the only place a party name is written in this phase.
- **Enums** (closed in code; the SQL CHECK lists are the same values): `ReportType { Str, Sar, Tfr, Utr, Ctr, Aif }`; `ReportStatus { Draft, Validated, Signed, Packaged, Filed, Acknowledged, Rejected, Returned }`; `QualityGrade { Good, Moderate, Acceptable, Poor }`; `FindingSource { Validation, Office }`; `FindingLayer { L0, L1, L2, L3, L4, L5, Office }`; `FeedbackKind { Grade, Rejection, Session, Request }`; `ReportTaskKind { ReturnedForCompletion, FxRateMissing, CtrControlCheck }`; `TaskState { Open, Claimed, Done }`; `CtrBatchStatus { Queued, WaitingSource, WaitingFx, Computed, Confirmed, Packaged, Superseded, Failed }`; `CtrGrain { CustomerDay, Transaction }`; `LetterKind { FreezeReport, FreezeSecondReport, Referral, OfficeReply, HoldReply }`; `LetterState { Requested, Rendered, Signed, Delivered }`; `PeriodicKind { CbiMonthly, Liaison, Board, MlroAnnual, Fatca, Crs, Analytics }`; `ReportLinkRelation { Previous, Amending, Superseding, EscalatedFromUtr }`. The serde values are the snake-case SQL values of section 1.
- **Value objects:** `ReportRef` (`RP-<TYPE>-<yyyy>-<seq>`), `GoamlReference` (1 to 60 characters after trimming), `LetterNo` (`AKR-LTR-<yyyy>-<seq6>`), `PeriodKey` (`YYYY` or `YYYY-MM`), `TxnRef { id, booking_date }`.

**Create and update DTOs.** `CreateReportDraft { suspicion_formed_on: Option<NaiveDate> }` (the body of `POST /cases/{id}/reports`; the type comes from the running proposal, never from the body). `UpdateReportDraft` (PATCH; every field `Option<T>`): `suspicion_formed_on`, `content_patch: Vec<FieldPatch>` (a field key and a value, checked against the builder schema of section 3.2). `ReviseReport { reason }`. `CreateCtrRun { business_date }`. `RecordFiling` (multipart: `goaml_reference`, `filed_at` optional, `activity_age_months` optional, part `portal_capture`). `RecordAcknowledgement` (multipart: part `receipt`, `acknowledged_at` optional). `RecordRejection` (`rejection_text`, at most 20,000 characters; optional part `capture`). `RecordFeedback { kind, grade?, changes_made?, rule_change_id?, received_at }`. `RequestLetter { kind, source_kind, source_id, copy_to_office, facts }` (manual requests: `office_reply` and `hold_reply` only; the phase 20 requests arrive through the port). `RecordLetterDelivery { channel, delivered_at, delivery_reference }`. `GeneratePeriodic { kind, period }`.

**Repository traits** (`domain::repositories::reporting`; the signatures carry the lock and the optimistic-version rules of section 3.24):
```rust
#[async_trait]
pub trait ReportRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, r: &NewReport) -> Result<Report, RepoError>;
    async fn lock(&self, tx: &mut Tx, id: Uuid) -> Result<Report, RepoError>;                            // FOR UPDATE
    async fn lock_by_proposal(&self, tx: &mut Tx, proposal: Uuid) -> Result<Option<Report>, RepoError>;   // FOR UPDATE; the approval path
    async fn update(&self, tx: &mut Tx, r: &Report, expected: i32) -> Result<Report, RepoError>;          // version + 1, compare and set
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<Report>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &ReportFilter, p: &Principal, page: Keyset) -> Result<Page<Report>, RepoError>;   // section 3.22 predicate, no total
    async fn insert_link(&self, tx: &mut Tx, l: &NewReportLink) -> Result<(), RepoError>;
    async fn links(&self, conn: &mut PgConnection, id: Uuid) -> Result<Vec<ReportLink>, RepoError>;
}
#[async_trait]
pub trait ReportVersionRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, v: &NewReportVersion) -> Result<(), RepoError>;
    async fn get(&self, conn: &mut PgConnection, report: Uuid, version: i32) -> Result<Option<ReportVersion>, RepoError>;
    async fn lock(&self, tx: &mut Tx, report: Uuid, version: i32) -> Result<ReportVersion, RepoError>;
    async fn set_content(&self, tx: &mut Tx, id: Uuid, content: &SealedContent) -> Result<(), RepoError>;   // refused by the trigger once frozen
    async fn set_validated(&self, tx: &mut Tx, id: Uuid, run: Uuid, summary: &ValidationSummary, at: DateTime<Utc>) -> Result<(), RepoError>;
    async fn freeze(&self, tx: &mut Tx, id: Uuid, at: DateTime<Utc>) -> Result<(), RepoError>;
    async fn summaries(&self, conn: &mut PgConnection, report: Uuid) -> Result<Vec<ReportVersionSummary>, RepoError>;
}
#[async_trait]
pub trait FindingRepository: Send + Sync {
    async fn insert_run(&self, tx: &mut Tx, rows: &[NewReportFinding]) -> Result<u64, RepoError>;   // one UNNEST batch per run
    async fn latest_run(&self, conn: &mut PgConnection, report: Uuid, version: i32, f: &FindingFilter) -> Result<Vec<ReportFinding>, RepoError>;
}
#[async_trait]
pub trait FilingRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, f: &NewFiling) -> Result<Filing, RepoError>;
    async fn lock(&self, tx: &mut Tx, id: Uuid) -> Result<Filing, RepoError>;
    async fn set_ack(&self, tx: &mut Tx, id: Uuid, expected: i32, by: Uuid, at: DateTime<Utc>, ev: Uuid) -> Result<Filing, RepoError>;
    async fn by_report(&self, conn: &mut PgConnection, report: Uuid) -> Result<Vec<Filing>, RepoError>;
    async fn ack_exists_for_proposal(&self, conn: &mut PgConnection, proposal: Uuid) -> Result<bool, RepoError>;   // section 3.10a
}
#[async_trait]
pub trait FeedbackRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, f: &NewFeedback) -> Result<FiuFeedback, RepoError>;   // append only
    async fn register(&self, conn: &mut PgConnection, f: &FeedbackFilter, page: Keyset) -> Result<Page<FiuFeedback>, RepoError>;
}
#[async_trait]
pub trait ReportTaskRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, t: &NewReportTask) -> Result<Option<Uuid>, RepoError>;   // None when the partial unique index refuses an open duplicate
    async fn lock(&self, tx: &mut Tx, id: Uuid) -> Result<ReportTask, RepoError>;
    async fn close(&self, tx: &mut Tx, id: Uuid, by: Option<Uuid>, at: DateTime<Utc>, reason: &str, expected: i32) -> Result<(), RepoError>;
    async fn open_fx(&self, conn: &mut PgConnection, date: NaiveDate) -> Result<Vec<ReportTask>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &TaskFilter, page: Keyset) -> Result<Page<ReportTask>, RepoError>;
}
#[async_trait]
pub trait CtrRepository: Send + Sync {
    async fn next_run_no(&self, tx: &mut Tx, date: NaiveDate) -> Result<i32, RepoError>;
    async fn insert_batch(&self, tx: &mut Tx, b: &NewCtrBatch) -> Result<CtrBatch, RepoError>;   // the partial unique index refuses a second live batch
    async fn lock_batch(&self, tx: &mut Tx, id: Uuid) -> Result<CtrBatch, RepoError>;
    async fn set_batch(&self, tx: &mut Tx, b: &CtrBatch, expected: i32) -> Result<CtrBatch, RepoError>;
    async fn insert_items(&self, tx: &mut Tx, rows: &[NewCtrItem]) -> Result<u64, RepoError>;   // UNNEST batches of 500
    async fn set_item_files(&self, tx: &mut Tx, batch: Uuid, files: &[(Uuid, Uuid, i32)]) -> Result<u64, RepoError>;   // (item, report, file_no)
    async fn items(&self, conn: &mut PgConnection, batch: Uuid, reportable: Option<bool>) -> Result<Vec<CtrItem>, RepoError>;
}
#[async_trait]
pub trait LetterRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, l: &NewLetter) -> Result<Letter, RepoError>;
    async fn lock(&self, tx: &mut Tx, id: Uuid) -> Result<Letter, RepoError>;
    async fn set_state(&self, tx: &mut Tx, l: &Letter, expected: i32) -> Result<Letter, RepoError>;
    async fn by_source(&self, conn: &mut PgConnection, source_kind: &str, source: Uuid) -> Result<Vec<Letter>, RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<Letter>, RepoError>;
    async fn next_letter_no(&self, tx: &mut Tx) -> Result<LetterNo, RepoError>;   // nextval of rpt.letter_no_seq
}
#[async_trait]
pub trait PeriodicRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, p: &NewPeriodic) -> Result<PeriodicReport, RepoError>;
    async fn lock(&self, tx: &mut Tx, id: Uuid) -> Result<PeriodicReport, RepoError>;
    async fn set(&self, tx: &mut Tx, p: &PeriodicReport, expected: i32) -> Result<PeriodicReport, RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &PeriodicFilter, page: Keyset) -> Result<Page<PeriodicReport>, RepoError>;
}
```

**Services** (`services::reporting`; each takes the transaction of its caller, or opens one, as the phase 18 services do):
- `ReportService`: drafts, edits, validation requests, submit, revise, attachments, links (sections 3.1 to 3.5a).
- `ReportApprovalService::sign` (section 3.6). The case subject of phase 18 is extended for `case_report_approval` (section 4.5), and `ReportWorkflowSubject` (new, resource kind `report`) serves `report_correction` (section 3.7).
- `ReleaseForFilingEffect` (phase 18's kind `release_for_filing`, body replaced; section 3.7) and `ReportCorrectionEffect` (kind `release_correction`, new; section 3.7).
- `PackageService` (request and job body, section 3.8) and `DownloadService` (section 3.9).
- `FilingService` (`record_filing`, `record_acknowledgement`; section 3.10); `RejectionService` (section 3.11, which calls phase 07's `RejectionParser`, completed here); `FeedbackService` (section 3.12).
- `CtrService` (`run_day`, `confirm`, `package`; sections 3.13 and 3.14); the FX wait is a method of `CtrService` (section 3.13 step 9).
- `LetterService` implements phase 20's `LetterRequester` and serves the manual letters; `LetterBackfill` runs once (section 3.15).
- `PeriodicReportService` (section 3.16) and `AnalyticsService` (section 3.17).
- `ReportClockHandlers` (section 3.18 and section 4.2).
- `ReportQuery` (the read model of R1, R2 and R3; the predicates of section 3.22).

**Error enum** `ReportError` (`thiserror`; section 3.19 maps each variant to an API section 6 code and its sub-codes).

**Route table** (base `/api/v1`; the rows marked "API" are API.md 3.13 rows from `POST /cases/{id}/reports` down, owned here; the rows marked "added" are new, and each is listed in Open doubts 9 for the API.md update; SU is the step-up of section 3.22; 4E is the four-eyes rule of phase 04):

| Method | Path | Scope | Description (cedar action) |
|-|-|-|-|
| POST | `/cases/{id}/reports` | `report.prepare` | Draft from the running proposal of type utr, str or tfr; 201, or 200 for the existing draft (`report.create`) [API] |
| GET | `/reports` | `report.prepare` | List, suspicion tier post-filtered; cursor, no total (`report.read`) [API] |
| GET | `/reports/{id}` | `report.prepare` | Draft or final with validation state and quality; **SU** (`report.read`) [API] |
| PATCH | `/reports/{id}` | `report.prepare` | Edit the draft; `If-Match` (`report.edit`) [API] |
| GET | `/reports/{id}/versions` | `report.prepare` | Versions with frozen and validated times; `?from=&to=` compares two versions (`report.read`) [added] |
| GET | `/reports/{id}/findings` | `report.prepare` | Findings of the latest run of the version, with `field_key` for the panel, `?layer=&severity=` (`report.read`) [added] |
| POST | `/reports/{id}/attachments` | `report.prepare` | Stage an attachment (multipart; `EvidenceStore::stage`) (`report.edit`) [added] |
| POST | `/reports/{id}/validate` | `report.prepare` | 202, job `report_validate`; `If-Match` (`report.validate`) [API] |
| POST | `/reports/{id}/submit` | `report.prepare` | Freeze the validated version; 422 `E_REPORT_INVALID` with blocks; `If-Match` (`report.submit`) [API] |
| POST | `/reports/{id}/revise` | `report.prepare` | New draft version from a rejected or returned report; `If-Match` (`report.edit`) [added] |
| POST | `/reports/{id}/sign` | `report.file` | Sign: acts on the `sign` task of `case_report_approval` or `report_correction`; **SU** (`signing`), **4E**; `If-Match` = instance version (`report.sign`) [API] |
| POST | `/reports/{id}/package` | `report.file` | 202, job `report_package` (`report.package`) [API] |
| GET | `/reports/{id}/package` | `report.file` | Download the ZIP; **SU** (`export`); `X-Access-Purpose` (`report.package_read`) [API] |
| POST | `/reports/{id}/filing` | `report.file` | Record the goAML reference and the portal capture (multipart); **SU** (`filing`) (`filing.record`) [API] |
| POST | `/filings/{id}/acknowledgement` | `report.file` | Record the Office receipt; **SU** (`filing`) (`filing.acknowledge`) [added] |
| POST | `/reports/{id}/rejections` | `report.file` | Record an Office rejection; findings; status `rejected` (`report.reject`) [API] |
| POST | `/reports/{id}/feedback` | `report.file` | Record a grade; a poor grade opens the returned task (`report.feedback`) [API] |
| GET | `/fiu-feedback` | `report.prepare` | The feedback register (R2), filters by kind, grade, period; suspicion post-filter (`feedback.read`) [added] |
| GET | `/report-tasks` | `report.prepare` | Open returned, FX and control tasks; cursor (`report.task.read`) [added] |
| GET | `/ctr/batches` | `report.prepare` | Daily CTR batches; counts and totals, no customer identity below M (`ctr.read_list`) [API] |
| GET | `/ctr/batches/{id}` | `report.prepare` | Batch with control totals and run numbers; no GL tie-out (the canonical copy has no general ledger; DR-174) (`ctr.read`) [API] |
| POST | `/ctr/batches` | `report.file` | Run a business date (manual run; the daily job runs it automatically); 202 (`ctr.run`) [added] |
| POST | `/ctr/batches/{id}/confirm` | `report.file` | The checker confirms the control totals; **SU** (`approval`); not the creator (`ctr.confirm`) [added] |
| POST | `/ctr/batches/{id}/package` | `report.file` | Build the CTR files: 202, job `ctr_package` (`ctr.package`) [API] |
| POST | `/letters` | `report.file` | A manual `office_reply` or `hold_reply` (`letter.create`) [API] |
| GET | `/letters/{id}` | `report.file` | Letter with state, number, PDF reference and delivery record (`letter.read`) [API] |
| POST | `/letters/{id}/sign` | `report.file` | Sign the letter; **SU** (`signing`) (`letter.sign`) [added] |
| POST | `/letters/{id}/delivery` | `report.file` | Record the delivery: channel, date and reference; no sending (`letter.deliver`) [added] |
| GET | `/reports/periodic` | `dashboard.read` | Periodic packs and their status (R3) (`periodic.read`) [API] |
| POST | `/reports/periodic` | `report.file` | Generate a pack for a kind and period; 202, job `periodic_build` (`periodic.generate`) [added] |
| GET | `/reports/periodic/{id}` | `dashboard.read` | Pack detail: aggregates, suppressed cell count (`periodic.read`) [added] |
| POST | `/reports/periodic/{id}/sign` | `report.file` | Sign and finalise the pack; **SU** (`signing`) (`periodic.sign`) [added] |

Count: 32 routes; 18 rows of API.md 3.13 owned here, 14 added. `GET /liaison-reports` and `POST /liaison-reports/{id}/review` (API.md 3.16) and screen R5 belong to phase 22, which owns the liaison monthly reports (lead review). The router is `pub fn router() -> Router<AppState>` in `api::routes::reports` (reports, periodic, attachments), `api::routes::filings`, `api::routes::ctr`, `api::routes::letters`, `api::routes::fiu_feedback` (feedback register and report tasks). The path `/reports/periodic` is registered with the reports router; `{id}` is a UUID in the path pattern, so `periodic` cannot match it.

## 3. Business Logic

### 3.1 Drafts (`ReportService::create_draft`; RP-11, RP-48)

Route `POST /cases/{id}/reports` (scope `report.prepare`; Cedar `report.create` on the case; `Idempotency-Key`; `X-Access-Purpose` required). Steps:
1. `lock(tx, case_id)` (phase 18 section 3.16 step 2). The case is `in_review` with a running proposal (`casework.proposals` with `ended_at IS NULL`) whose `type` is `utr`, `str` or `tfr`; otherwise 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `no_reportable_proposal`. A `no_report` proposal never creates a report.
2. The caller holds an active `decide` or `work` grant on the case (`casework.case_acl`, `revoked_at IS NULL`, lapsed grants excluded) and the role `A2` or `M`. Otherwise 403 sub-code `role_not_permitted`; for a case the caller cannot see, 404.
3. One report per proposal (`uq_reports_proposal`). A second request returns the existing report with 200 and the same body.
4. The active profile: the phase 07 `GoamlProfileRepository::active("IQ")`. None: 503 `E_UNAVAILABLE`. The row is pinned in `profile_id`.
5. `report_ref` from `rpt.report_ref_seq`, with TYPE from the proposal's outcome.
6. The first version is pre-filled from the proposal and the case: `reason` and `origin` and `earlier_reports` from the proposal's own columns; `indicators` from the proposal's `reason_codes` through the report-indicator map of RP-18 (Open doubts 2: until the map exists, the reason codes are the indicators and the analyst adds the rest); `transactions` from the proposal's `transactions` array (each with its booking date; the suspicious marker is `true` for every transaction the proposal names); `subjects` from the proposal's `subjects` array, each rebuilt from the canonical copy by section 3.3; `actions` from the proposal's `actions`. `suspicion_formed_on` from the body when given; otherwise empty, and L0 blocks validation until it is set (RP-48).
7. `subject_party_id` = `casework.cases.party_id`; `case_id`, `proposal_id` and `workflow_instance_id` = the proposal's columns; `status = draft`, `current_version = 1`, `created_by = actor`.
8. Insert the report and version 1 (`content_sha256` of the canonical JSON of the plaintext, `content_enc` sealed with AAD = the version id). Audit `report.created {report_id, report_ref, type, case_id, proposal_id}`. No chain event (the case chain is written by a decision only, phase 18) and no outbox event.
9. 201 with the detail view.

### 3.2 The six steps of the builder (RP-21, RP-22, RP-25, RP-26; screen R1)

The builder is one draft with six steps, in the order of the goAML structure (research file 23, section 8.5). Each step is a set of field keys (`reason`, `origin`, `subjects[n].*`, `accounts[n].*`, `transactions[n].*`, `narrative.*`, `attachments[n].*`, `indicators`, `action_taken`, `suspicion_formed_on`, `earlier_reports[n].*`) that `PATCH /reports/{id}` writes, each write with `If-Match` and the content of the current version is replaced in place while that version is not frozen (`set_content`); only revise (section 3.5a) creates a new version.
1. **Reason and indicators.** `reason` is required and not blank (BR-01, phase 07). `indicators` has at least one code for STR, SAR and TFR (BR-02). Each indicator is a code of the profile's indicator lookup (phase 07 lookups, RP-02); an unknown code is a block.
2. **Subjects.** One `SubjectDraft` per person or entity. A natural person has a mother's name field: an empty value is a warn (BR-05), and the field is marked UNKNOWN (RP-26) when the bank records it so.
3. **Accounts and transactions.** Each account is a `canon.account` id. Each transaction is a `TxnDraft { txn_id, booking_date, date, amount, currency, mode, from, to, suspicious }`. The chronological table of RP-14 and RP-22 is generated by sorting on `date`, then on `txn_id`. A transaction whose date lies outside the range the narrative states raises CONS-02 at validation.
4. **Narrative.** Arabic and English text (`narrative.ar`, `narrative.en`), each at most 8,000 characters (RP-21). The counter is shown on the screen, and a PATCH above the limit is 422 `E_VALIDATION` at `/narrative/ar` or `/narrative/en`. The RP-22 skeleton (who, what, when, where, why, how) is the template; the chronology is generated; each `RelatedRef` is rendered in the first lines with its number and date (RP-19). Every text write passes `PolicyPackService::scan` with scope `reason_text` (phase 09); a hit is 422 `E_VALIDATION` at the field, carrying the phase 09 reason.
5. **Validation** is not a PATCH: `POST /reports/{id}/validate` (section 3.4).
6. **Package** is not a PATCH: it follows the signature (section 3.8).

A field may be recorded as UNKNOWN (RP-26) with a reason code of the profile's closed list (`unknown_reason`, phase 07 lookups), and the quality score counts UNKNOWN apart from missing (section 3.4 step 6).

**Attachments** (RP-25): `POST /reports/{id}/attachments` (scope `report.prepare`, Cedar `report.edit`; the route is an added row, Open doubts 9) stages the file with `EvidenceStore::stage` (kind `upload`, owner kind `report`, owner id = the report, retention class R6). The type and size checks are the phase 07 profile limits (`max_attachments`, `attachment_types`, `max_file_bytes`). The narrative gets a numbered list of the attachments by their generated names `att-NNN.<ext>`; the source file name, which phase 02 seals, never reaches the XML.

### 3.3 Mapping from the canonical copy (`ReportMapper`; RP-17)

A pure function over the canonical rows of the subject, read in the caller's transaction under the phase 10 read grants. It never writes to the canonical copy.
1. Party: `canon.party` (current row) gives `type`, `nationality`, `birth_date`, `birth_precision`, `birth_place`, `residence_country`, `occupation_isic` and `legal_form`. `mother_name_enc` is decrypted inside the service only.
2. Name: `canon.party_name` rows with `superseded_at IS NULL` and `name_type = 'legal'`. The phase 10 `parts` array is used when it is present. Otherwise the raw name is split in the Iraqi quadripartite form: the first token is `first_name`, the second is the father's name, the third is the grandfather's name, and the last token is `last_name`. The connectors `bin`, `bint`, `ibn`, `abu`, `ابن`, `بن`, `بنت`, `ابو` and `أبو` stay with the token that follows them. A one-token name goes to `first_name`, and `last_name` takes the UNKNOWN marker (a block at L2 when the profile requires it). Apostrophes, dots, commas, parentheses and slashes are removed from person names, since the Iraqi pattern rejects them (research file 23, section 2.5); each removal is a warn finding `NAME-01` (new in phase 21) with `params.removed_count`.
3. Birth date: precision `day` gives the full date. Precision `year` gives `YYYY-01-01` with the month and day marked UNKNOWN (RP-26) and a warn `DATE-01` (new in phase 21): the XSD has no precision flag and the research gives no Office convention (Open doubts 10). Precision `month` or `unknown` is a block at L0 unless the field is marked UNKNOWN.
4. Address: the current `canon.party_contact` row of type `address` gives `city` and `district`. `governorate` must be a code of `ref.governorates` (`IQ-XX`, phase 09) and maps to the goAML `state` (RP-17); a code outside the table is a block at L0 (`governorate_unknown`, new in phase 21). Phone numbers take the country prefix 964.
5. Identifiers: `canon.party_identifier` rows with `superseded_at IS NULL` and `placeholder_flag = FALSE` map to `identification`, with the type code from the phase 10 code map and the value decrypted in the builder only. A placeholder maps to nothing and raises `ID-01` (new in phase 21, warn: the national ID is missing).
6. Entities: the legal name, the registration number (`canon.party.registration_no_enc`) and the legal form map to the entity block; `incorporation_country` is `residence_country`.
7. Output: a `SubjectDraft` in which each field is a value, the UNKNOWN marker or absent. The output carries no value into a log, an audit payload or a finding.

### 3.4 Validation (`ReportService::validate`, job `report_validate`; RP-27, RP-28, RP-29, RP-30, RP-82)

`POST /reports/{id}/validate` (scope `report.prepare`; Cedar `report.validate`; `If-Match` = the report version) requires status `draft` or `validated` and an unfrozen version. It enqueues `report_validate {report_id, version}` (coalesce key `report_validate:<report_id>`) and answers 202 with a `Location` to the report. The job (akr-worker):
1. Start: the version must still be the current one and unfrozen; otherwise the job completes with `superseded` and writes nothing. A new `run_id` (UUIDv7) is used for every finding of this run.
2. **L0, canonical completeness (new in phase 21).** On the mapped content and the canonical copy: each subject exists in `canon.party` (`subject_missing`, block); each account and transaction id exists (`canon_missing`, block); each transaction's booking date equals the stored `canon.txn.booking_date` (the table key is `(id, booking_date)`; `txn_booking_mismatch`, block); `suspicion_formed_on` is set for STR, SAR and TFR (`suspicion_date_missing`, block); the alert snapshot of the case (phase 17 `snapshot_object_id` of the alert that opened the case) still matches the transaction set of the draft, else `snapshot_drift` (warn with the count). L0 does not run for CTR files, whose source is the batch.
3. Build: `GoamlPackager::build(content, profile)` (akr-goaml, pure) gives the XML bytes and the map from element path to `field_key`. A value the XML cannot express is a block at L2 (`build_failed`, with the field key in `params`).
4. **L1, XSD**: `XsdValidator::validate` of phase 07 (`SandboxedXsdValidator`), the same call as for an upload. Findings keep the element path and the line; `field_key` comes from the map of step 3.
5. **L2, Iraqi business rules**: `RuleCatalogue` (phase 07 section 3.3) on the built tree. The rules that need the canonical copy are L0.
6. **L3, quality**: only when no block exists in L0 to L2, `Scorer::score` (phase 07 section 3.4) gives the score and the estimated grade. The Q criteria read the content, with UNKNOWN counted apart from missing (RP-26). The result is written to `reports.quality_score` and `quality_grade`, and the screen labels the grade "estimated".
7. **L4, consistency**: `ConsistencyChecks` (phase 07): narrative amounts equal the structured totals (CONS-01), dates lie inside the transaction range (CONS-02), the suspicious sum equals the reported amount (CONS-03), and every transaction's accounts appear among the subjects' accounts (`CONS-05`, warn, new in phase 21 for the RP-30 account rule; `CONS-04` is phase 07's chronological-order rule).
8. **L5, package and size**: `PackageChecks` and `PreScreen` (phase 07) on the built XML and the attachments, with the RP-82 limits (transactions per report, XML size, attachment count, size and type: `PKG-01` and `PKG-02`). An attachment whose phase 02 status is still `pending` is a block `PKG-07` (new in phase 21, "being checked").
9. Lock the report, then the version (section 3.24 step 3). When the version is no longer the current one, is frozen, or its `content_sha256` differs from the hash of the content this run read, the job completes with `superseded` and writes nothing. Otherwise write the findings (`insert_run`, one row per finding, `source = validation`, typed `params`) and `set_validated(run, summary, now)`. The summary holds counts by layer and severity only. The report becomes `validated` when no block exists, otherwise it stays `draft`. Audit `report.validated {report_id, version, run_id, block, warn, info, grade}`.
10. A block in any layer keeps the report from submission (section 3.5). A warn or an info never blocks: the analyst may submit with warns, and the warns are shown on the sign task.
11. Layer numbering follows phase 07, where L1 is XSD, L2 the rules, L3 the quality, L4 the consistency and L5 the package with the pre-screen. The research's six layers are described by seven names (research file 23, section 8.6); this phase keeps phase 07's numbering and Open doubts 1 records the mismatch.

The job is idempotent: `validated_run_id` is set once per run, and a retried job whose run already exists writes nothing.

### 3.5 Submit and the state machine (RP-34, RP-80)

`POST /reports/{id}/submit` (scope `report.prepare`; Cedar `report.submit`; `If-Match`; `Idempotency-Key`):
1. Lock the report and the version. The status must be `validated` (409 `E_NOT_ALLOWED_IN_STATE`, sub-code `not_validated`), `version` must equal `current_version` (sub-code `version_stale`), and the version must carry `validated_run_id`.
2. A block in the latest run of the version: 422 `E_REPORT_INVALID` with the findings (layer, rule code, `field_key`, `element_path`; never a value).
3. `freeze(version, now)`; `frozen_version = current_version`; `submitted_at = now`; `submitted_by = actor`. The trigger of section 1.3 makes the version immutable from here.
4. The approval: for STR, SAR, TFR and UTR the workflow instance `case_report_approval` already exists, because phase 18 started it at the proposal; the sign task is open and now has a frozen version to sign. For a correction the instance `report_correction` is started here (section 3.7).
5. Audit `report.submitted {report_id, version, workflow_instance_id}`.

The state machine of the report (the statuses of section 1.1). The services write only these transitions:
- `draft` to `validated`: validation without block (section 3.4 step 9).
- `validated` to `draft`: a PATCH after validation clears `validated_run_id` on the version.
- `validated` (submitted) to `signed`: the sign act (section 3.7).
- `signed` to `packaged`: the package job (section 3.8). For a CTR file, `validated` to `packaged` in the `ctr_package` job (section 3.14 step 3).
- `packaged` to `filed`: the filing (section 3.10).
- `filed` to `acknowledged`: the receipt (section 3.10); `filed` to `rejected`: the Office rejection (section 3.11).
- `acknowledged` or `filed` to `returned`: a poor grade (section 3.12 steps 1 and 2). `returned` or `rejected` to `draft`: revise (section 3.5a).
- `acknowledged` keeps its status when the grade is good, moderate or acceptable.

**3.5a Revise** (`POST /reports/{id}/revise`; scope `report.prepare`; Cedar `report.edit`; `If-Match`). Allowed from `rejected` or `returned` only (409 `not_revisable` otherwise). It creates version `current_version + 1` with the content of the frozen version (sealed again under the new version id), sets `status = draft`, clears `submitted_*`, `signed_*` and `frozen_version`, and writes audit `report.revised {report_id, version, from_version}`. Every correction is a new version, and its filing gets a new goAML reference (RP-42); the earlier version, its rejection and its filing stay in the record. A new report, rather than a correction, is used only when the Office asks for it (Open doubts 12).

### 3.6 Sign: the approval act (`ReportApprovalService::sign`; RP-34, RP-36, RP-37; route `POST /reports/{id}/sign`)

The route acts on the workflow task; it never writes a status directly. Steps:
1. Scope `report.file` (role `M`); Cedar `report.sign` on the report (section 3.22). `If-Match` = the workflow instance version (phase 04); `Idempotency-Key`. The body names `task_id`, an open `sign` task of the report's instance (otherwise 404 `E_NOT_FOUND`).
2. Step-up: the declared action `report.sign` with class `signing` (section 3.22, the action constants). The phase 04 engine consumes the assertion once, with `resource_ref = "task:<id>"` (its step 5). A missing, reused or wrong-class assertion is 401 `E_STEPUP_REQUIRED`.
3. `WorkflowEngine::act(ctx, task_id, TaskAction::Sign, if_match, idem, assertion)` runs the phase 04 steps. Eligibility is `Holders`: an active `case_acl` grant with `decide` on the case, and role `M` (phase 18 section 3.8 a). Four-eyes: the proposer of the instance, the submitter of the report and the earlier deciders are excluded (403 `E_SELF_APPROVAL`). Then `CaseWorkflowSubject::check_before_decision` (phase 18 section 3.8 d, extended in section 4.5): the report of the instance exists; `status = validated`; `frozen_version = current_version`; the latest run of the frozen version has no block; the case is not sealed. For a `report_correction` instance the subject is `ReportWorkflowSubject` (section 3.7): its holders are M at bank scope and it does not require an open case.
4. The terminal effects run in the same transaction in the order of the definition. `case_report_approval` runs `record_decision` (phase 18 `RecordDecisionEffect`: the decision row, the chain event `case.decided`, the tier raise to `suspicion`, the outbox `case.decision`) and then `release_for_filing` (section 3.7). `report_correction` runs only `release_correction`.
5. Two-person signature (RP-36). The bank may set `report.single_officer_exception` (governed, default FALSE) when it has one compliance officer. Then a TFR and a STR at or above `report.large_str_usd` (governed, default 0 which means "all STR"; Open doubts 5) require a second `sign` task, held by a second `M` on the case access list who is not the first signer. Without the exception the definition has one `sign` task.
6. Audit: phase 04 writes `task.signed`. The effect writes `report.signed {report_id, version, decision_id}` (section 3.7 step 4).
7. The response is the instance view. Refusals return the phase 04 problem codes, and the case sub-codes `not_validated`, `version_stale` (409), `case_sealed` (409 `E_CASE_SEALED`) and `role_not_permitted` (403).

**Route refusal (a phase 18 change, section 4.5).** The phase 18 route `POST /cases/{id}/decisions` with `action: sign` on a `case_report_approval` instance, and the generic phase 04 task route for a `sign` task of `case_report_approval` or `report_correction`, refuse with 409 `E_NOT_ALLOWED_IN_STATE` and sub-code `report_sign_required`. The report route passes the context marker `via = "report"`, which is the only way the subject check lets a report sign through.

### 3.7 Terminal effects (`ReleaseForFilingEffect` and `ReportCorrectionEffect`; DSL E.3 and E.4; DR-169, DR-175)

**`ReleaseForFilingEffect::run(tx, inst, params, actor)`** (phase 18 kind `release_for_filing`, resource kind `case`). Its body replaces phase 18 section 3.8 step g; the kind and the resource kind do not change.
1. Load the proposal by `inst.id` (`casework.proposals.workflow_instance_id`) and the report by `lock_by_proposal`. No report: `WorkflowError::Effect("no_report")`, and the whole approval rolls back (phase 04 rule).
2. The report must be `validated`, with `frozen_version = current_version` and `submitted_at` set (the sign checks already guarantee this; the effect checks it again).
3. Set `status = signed`, `signed_at = now()`, `signed_by = actor`.
4. Chain event `case.filing_released {report_id, report_ref, decision_id}` (the phase 18 event name is kept; its meaning is now "signed, ready to package"), and audit `report.signed {report_id, version, decision_id}`.
5. Filing is a person's act (DSL E.3). No job and no timer are created here except the filing target of section 3.18: `report_filing_target` at `signed_at + report.filing_target`.

**`ReportCorrectionEffect::run`** (kind `release_correction`, resource kind `report`; new in phase 21):
1. The report is taken from the instance context (`report_id` and `version`, frozen by `start_context`).
2. The same checks and the same status change as section 3.7 steps 2 to 4, without the chain event and without a decision: a correction is not a new case decision (phase 18 allows one decision per proposal), and no tier changes.
3. Audit `report.signed {report_id, version, correction: true}`.

**The correction instance** (section 3.5 step 4): `WorkflowEngine::start(tx, StartSpec { definition_key: "report_correction", resource: ("report", report_id), purpose: Decision, initiator: submitter, maker: submitter })`, with `start_context` of `ReportWorkflowSubject` freezing `{ report_id, version, case_id }`. The correction runs on the report, not on the case, because a poor grade or a late rejection usually arrives after the acknowledgement has let the case close, and phase 18 refuses every act on a sealed case (lead review). `ReportWorkflowSubject` (new; resource kind `report`) takes no case lock and writes no case event; its `check_before_decision` requires the report `validated`, `frozen_version = current_version`, no block in the latest run and `via = "report"`; its `Holders` are the users with role `M` at bank scope (scope `report.file`), with the report's submitter and earlier signers excluded, and no `case_acl` grant is required. The definition `report_correction` is shipped by a `cr_workflow` change request (DR-175): `sign` (M, step-up `signing`) then `release_correction`.

### 3.8 Package (`PackageService::request`, job `report_package`; RP-38, RP-63; route `POST /reports/{id}/package`)

Scope `report.file`; Cedar `report.package`; `Idempotency-Key`. Status `signed`, or `validated` for a CTR file (section 3.14); otherwise 409 `not_signed`. No package yet for this version. It enqueues `report_package {report_id}` (coalesce `report_package:<report_id>`) and answers 202. The job:
1. Open the frozen version (`content_enc` under AAD = the version id); the plaintext hash must equal `content_sha256` (else `E_HASH_MISMATCH`, and phase 02 opens the incident).
2. `GoamlPackager::build` again. The XML must hash to the XML hash recorded at validation; a difference is a block `build_drift`, the job fails, and the frozen version is not changed.
3. Attachments: each through `EvidenceStore::open` (verified reader), copied into the ZIP under its generated name.
4. Manifest: `{ profile_version, report_ref, report_version, validation_run_id, files: [{name, sha256, size}] }`. `manifest_sha256` is the SHA-256 of the canonical manifest bytes.
5. Bilingual PDF: `RenderService::render` (phase 02, Gotenberg, PDF/A-2b) of the template `report.pdf.v1` in Arabic and English, showing the report reference and the manifest hash on each page. `put_generated` with kind `render`, owner kind `report_package`, owner id = the new package id.
6. ZIP (XML, attachments, manifest, PDF): `put_generated` with kind `package`, owner kind `report_package`, owner id = the package id, retention class R6.
7. One transaction: insert `rpt.report_packages` (both evidence ids, the manifest and its hash), `status = packaged`, audit `report.packaged {report_id, version, manifest_sha256}`.
8. A crash between steps 5 and 7 leaves evidence objects with no package row. They are orphans, listed by phase 23's reconciliation (the rule of phase 02 `put_generated`); the retried job writes new objects.

### 3.9 Download (`DownloadService::download`; RP-40, RP-61; route `GET /reports/{id}/package`)

Scope `report.file`; Cedar `report.package_read`; step-up class `export` (declared action `report.package.download`); `X-Access-Purpose` required (422 `E_PURPOSE_REQUIRED` without it). The purpose is a code of the phase 03 purpose list; the reporting purposes are an Open doubt (Open doubts 13).
1. Visibility (section 3.22). A caller who cannot see the report gets 404 `E_NOT_FOUND`, byte-identical to a missing report.
2. The package of the current packaged version (none: 409 `not_packaged`).
3. `EvidenceStore::open` of the ZIP. The access event `download` is recorded through `AccessScope` before the open (phase 02 step 5).
4. Audit `report.package_downloaded {report_id, version, evidence_id, purpose}`. The file is served as stored, with no per-user watermark: a watermark would change the bytes the manifest hashes (Open doubts 14).
5. RP-61: the report register and the report counts are excluded from global search and from the counts that branch roles see (section 3.22).

### 3.10 Filing and acknowledgement (`FilingService`; RP-39, RP-41, RP-79, M-41; routes `POST /reports/{id}/filing` and `POST /filings/{id}/acknowledgement`)

**Filing.** `POST /reports/{id}/filing` (scope `report.file`; Cedar `filing.record`, role `M` only; step-up class `filing`, declared `report.filing.record`; `Idempotency-Key`; multipart: `goaml_reference`, `filed_at` (optional, default now, never in the future), `activity_age_months` (optional), part `portal_capture`, which is the on-screen receipt or the as-filed XML copy (RP-79)):
1. Lock the report. Status `packaged` (else 409 `not_packaged`); `current_version` equals the packaged version (`version_stale`).
2. `goaml_reference`: trimmed, 1 to 60 characters, and checked against the profile's `reference_pattern` when the profile has one. A reference already in the table is 409 `E_NOT_ALLOWED_IN_STATE` sub-code `reference_in_use`.
3. The portal capture: `EvidenceStore::stage(tx, NewEvidence { kind: upload, owner_kind: filing, owner_id: <filing id, generated here>, retention_class: R6, ... })` in the same transaction. A capture that is still `pending` is accepted and the filing points to it; the screen shows "being checked" until it is clean (the phase 18 rule). A rejected capture (type or macro) is 422 `E_UPLOAD_REJECTED`, and nothing is written.
4. `activity_age_months` (M-41): when the body omits it, the service computes whole months from the newest booking date of the report's transactions to `filed_at`; a report with no transaction gets NULL and is left out of the share of section 3.17 item 3. The share of reports older than the setting `report.activity_age_months_threshold` (governed; default 12, Open doubts 15) is an analytics output (section 3.17).
5. Insert `rpt.filings`; `status = filed`. Audit `report.filed {report_id, filing_id, goaml_reference, activity_age_months}`. The reference is an identifier, so it may appear in the audit payload.
6. Cancel the timer `report_filing_target` owned by the report (`TimerService::cancel_for_owner`).

**Acknowledgement.** `POST /filings/{id}/acknowledgement` (scope `report.file`; Cedar `filing.acknowledge`, role `M`; step-up class `filing`, declared `report.filing.acknowledge`; multipart: part `receipt`, `acknowledged_at` optional):
1. Read the filing's `report_id` without a lock, lock the report, then the filing (section 3.24 step 3). `ack_at IS NULL` (else 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `already_acknowledged`). The report is `filed`.
2. `EvidenceStore::stage` of the receipt (kind `upload`, owner kind `filing`, owner id = the filing, retention class R6), then `set_ack(expected, by, at, evidence)`; the trigger allows one write.
3. The report becomes `acknowledged`. Audit `report.acknowledged {report_id, filing_id}`. This is the record the close gate of section 3.10a reads.

**3.10a Close gate (replaces the phase 18 check; phase 18 section 3.12 step 5).** For `utr`, `str` and `tfr`, closure requires an acknowledged filing of the report of the decided proposal: `FilingRepository::ack_exists_for_proposal(proposal_id)`, the SQL `EXISTS (SELECT 1 FROM rpt.reports r JOIN rpt.filings f ON f.report_id = r.id WHERE r.proposal_id = $1 AND f.ack_at IS NOT NULL)`. Without it the close is 409 `E_NOT_ALLOWED_IN_STATE` with sub-code `filing_not_acknowledged`, which replaces `filing_not_released`. The chain event `case.filing_released` stays in the chain as the signature record but is no longer the check. The check is one read after the case lock (lock order, section 3.24).

**UTR watch.** A UTR decision starts the phase 19 watch (DR-152); filing does not start it, and this phase makes no change to phase 19's watch.

### 3.11 Rejections (`RejectionService::record`; RP-42, RP-79; route `POST /reports/{id}/rejections`)

Scope `report.file`; Cedar `report.reject`; `Idempotency-Key`; body `rejection_text` (at most 20,000 characters) and an optional part `capture`.
1. Lock the report. The status must be `filed` or `acknowledged` (a rejection after acknowledgement is possible: the Office can reject later). Otherwise 409 `not_filed`.
2. `RejectionParser::parse(text, tree, patterns)` (phase 07 section 3.7, completed here). The tree is the XML of the filed version, rebuilt by `GoamlPackager` and checked against the package manifest; if it does not match (`build_drift`), the parser reads the XML stored in the package through `EvidenceStore::open`. Each Office message becomes an `OFF-*` finding with its element path and line; a message that names a field maps to that field's `field_key` (section 3.4 step 3). The text is parsed in memory and never stored; the optional `capture` is staged as the Office text (kind `upload`, owner kind `fiu_feedback`, RP-79).
3. Write the findings (`source = office`, layer `office`, a new `validation_run_id`) and one `fiu_feedback` row, `kind = rejection`, with `finding_ids` and `capture_evidence_id`.
4. `status = rejected`. The filing row is kept, since its identity is written once. Audit `report.rejected {report_id, filing_id, findings_count, rule_codes}`: codes and counts only.
5. The correction is `POST /reports/{id}/revise` (section 3.5a), then validation, submit, sign (the correction instance of section 3.7), package and filing again, under a new goAML reference (RP-42).
6. The profile's `prev_rejected_ref_number` emit flag (phase 07, RP-05, default off) is set on the corrected file only when the profile enables it.

### 3.12 FIU feedback and the returned task (`FeedbackService::record`; RP-32, RP-43, RP-84, M-30; routes `POST /reports/{id}/feedback` and `GET /fiu-feedback`)

Scope `report.file`; Cedar `report.feedback`; `Idempotency-Key`; body `RecordFeedback`.
1. `kind = grade` needs `report_id` and `grade` (422 `E_VALIDATION` otherwise). The report must be `filed`, `acknowledged` or `returned` (409 `not_gradeable` otherwise). `office_grade` and `office_grade_at` are set on the report.
2. `poor`: the report becomes `returned`, and a `report_tasks` row `returned_for_completion` is created with `feedback_id` and `due_at = BusinessClock::add(now, report.rework, <working days>)` (the clock of section 3.18). The partial unique index allows one open task per feedback. Audit `report.returned {report_id, task_id, due_at}`; the timer `report_rework_due` is owned by the task (section 4.2).
3. `good`, `moderate` or `acceptable`: the status stays `acknowledged`; no task.
4. `kind = session` or `kind = request` carries no report. It records the Office's quality session or request with `changes_made` (RP-84: the common error or technical obstacle the Office named, the rule or template change made, and the `rule_change_id` when a change request exists: a phase 07 `goaml_profile` change or a template change).
5. `changes_made` passes `PolicyPackService::scan` with scope `reason_text` (phase 09); a hit is 422 `E_VALIDATION` at `/changes_made`.
6. Insert `fiu_feedback` (append only). Audit `feedback.recorded {feedback_id, kind, grade, report_id}`.
7. The case may be open or sealed. The feedback writes no case event and no case row (DR-179); when the case is sealed the audit event is the only trace.
8. The returned task is closed by the resubmission: a `returned_for_completion` task is closed with `CLS_REPORT_RESOLVED` when the corrected report is signed (`ReportTaskRepository::close`, section 3.7 step 3). Audit `report.task_closed`.
9. The first-time pass rate and the grade distribution of RP-84 are analytics (section 3.17).

### 3.13 The daily CTR run (`CtrService::run_day`, job `ctr_daily`; RP-50, RP-51, RP-52, RP-53)

The job runs after the business-date close (setting `schedule.ctr_daily`, Sunday to Thursday, bank time) for the business date that has just closed, and it runs by hand for any working day of the default calendar (phase 09) through `POST /ctr/batches` (422 `E_VALIDATION` at `/business_date` for a non-working day). Steps:
1. Coalesce key `ctr_daily:<business_date>`. If a live batch (status not `superseded` or `failed`) exists and the run is the scheduled one, the job returns without writing (idempotent). A manual run creates the next `run_no` and sets the earlier live batch to `superseded` in the same transaction (the partial unique index allows one live batch).
2. Completeness gate (RP-50, the replica check of research file 23, section 8.11): the business date must be covered for every source that feeds `canon.txn`: phase 12 `CoverageRepository::certificates_for_date(D, Some(source))` returns a current certificate with status `issued` or `attested` (a certificate `pending_attestation` waits). An incomplete day sets the batch to `waiting_source` and the job retries with back-off (five attempts); the register shows the day. Audit `ctr.waiting_source {batch_id, business_date}`.
3. Population: `canon.txn` rows with `business_date = D` and `cash_flag = TRUE`. Posted rows are counted; a cancelled row (`status = 'cancelled'`) is excluded with `CLS_CTR_EXCLUDED_CANCELLED`, and a reversed row (`reverses_id IS NOT NULL`, or `corrected_by_id IS NOT NULL` for the original) is excluded with `CLS_CTR_EXCLUDED_REVERSED`. Each excluded row is one `ctr_items` row with `reportable = FALSE` and its exclusion code, so the exclusions are listed with their reasons (RP-52).
4. Direction: `dr_cr = 'C'` is a deposit (money into the account), `dr_cr = 'D'` a withdrawal.
5. Conductor: the customer of the transaction's account (`canon.account.customer_id` of the current account row for `account_id`). No account or no customer: `customer_id` is NULL, the row is reportable on its own and carries a warn (the conductor is unidentified, RP-53, and the UNKNOWN convention applies).
6. FX (RP-51): for each currency in the population, `FxRateService::rate_on(currency, D)` (phase 09 section 3.2 step 5: the exact business date, `cbi` before `manual`). IQD has rate 1. A missing rate sets the batch to `waiting_fx`, opens one `report_tasks` row `fx_rate_missing {business_date, currency}` (the partial unique index keeps one open task per day and currency) and audit `ctr.waiting_fx {batch_id, business_date, currency}`. Nothing falls back to `canon.txn.amount_usd` (phase 10 stores that rate at the transaction's own date). The hourly job `ctr_fx_wait` (section 4.1) re-runs every `waiting_fx` batch whose currencies now have rates; the task closes with `CLS_REPORT_RESOLVED` when its rate arrives (`closed_by` NULL: the system).
7. Amounts: `usd = amount x rate_to_usd(currency, D)`; `lcy = amount x rate_to_iqd(currency, D)`. `rate_date = D` on every item (the database CHECK).
8. Threshold (RP-51). The fixed CBI value `CTR_THRESHOLD_USD = 10000` (a code constant mirroring the DSL section 9.1 pinned `limit`, `min = max`, basis CBI). With `ctr.aggregate_same_day = true` (the default, research Q9 and RP-51): the cash of one customer on the day is added up (deposits and withdrawals separately, and their sum is `total_usd`) and the group is reportable when `total_usd >= 10000`. A group below the threshold is not stored (the control totals count it); a group with a NULL customer is never aggregated and is tested per transaction. With aggregation off: each transaction is tested on its own USD amount, and the grain is `transaction`.
9. Control totals (RP-52): counts and sums by currency and by branch (the branch identifier and counts only, no names), the number of excluded rows by reason, the number of groups below the threshold, and the number of items per file to come. The batch's `total_usd` is the USD sum of the reportable items.
10. GL tie-out: the canonical copy holds no general-ledger table (phase 10). `gl_tieout` is `{"status": "not_available", "reason": "no_gl_source"}` until the bank names the GL control totals as a source (Open doubts 3). The reconciliation view shows the status; it is never shown as a match.
11. Insert the items (UNNEST batches of 500), set `reportable_count`, `excluded_count`, `total_usd`, `control_totals`, and set the batch to `computed`. Open one `report_tasks` row `ctr_control_check` for the batch (the checker's confirmation, section 3.14). Audit `ctr.computed {batch_id, business_date, run_no, reportable_count, excluded_count}`: counts only.
12. The CTR file quality rules (RP-53) run when the files are built (section 3.14): CTR-01 cash on one side and an account on the other, CTR-02 both countries domestic, CTR-03 amount sanity (an extreme or non-round amount is a warn; the bounds are the rule's parameters `max_usd` and `round_unit_iqd`, phase 07 section 7+). Rule codes CTR-01 to CTR-03 are new in phase 21 and enter the phase 07 rule catalogue in a release (section 4.5; DR-368).

The daily job's schedule is in `admin.settings` (`schedule.ctr_daily`); its time is an Open doubt (Open doubts 31).

### 3.14 CTR confirmation and the files (`CtrService::confirm`, `CtrService::package`, job `ctr_package`; RP-52, RP-53; routes `POST /ctr/batches/{id}/confirm` and `POST /ctr/batches/{id}/package`)

**Confirm.** Scope `report.file`; Cedar `ctr.confirm` (role `M`); step-up class `approval` (declared `ctr.confirm`); `If-Match`; `Idempotency-Key`:
1. Lock the batch. Status `computed` (else 409 `not_computed`). `confirmed_by <> created_by` (a manual run is confirmed by a second person: 403 `E_SELF_APPROVAL`). The confirmer confirms the control totals, not each row (RP-52).
2. Set `confirmed_by`, `confirmed_at`, `status = confirmed`; close the `ctr_control_check` task with `CLS_REPORT_RESOLVED`; audit `ctr.confirmed {batch_id, reportable_count, total_usd}`.

**Package.** `POST /ctr/batches/{id}/package` (scope `report.file`; Cedar `ctr.package`; `Idempotency-Key`): status `confirmed` (409 `not_confirmed` otherwise); enqueue `ctr_package {batch_id}` (coalesce `ctr_package:<batch_id>`); 202. The job:
1. Split the reportable items in order of `business_date`, `customer_id`, `id` into files of at most `ctr.file_max_transactions` transactions (each `txn_refs` entry counts once; the Office's own limit is an Open doubt, 24).
2. For each file: one `rpt.reports` row, `type = 'ctr'`, `ctr_batch_id`, its own `report_ref`, the profile pinned; a version whose content is the file's transactions and conductors, generated from the items and sealed; the version is validated by the section 3.4 steps 3 to 9 pipeline without L0 (its source is the batch), with the CTR rules CTR-01 to CTR-03 in L2; the version is frozen at once (`submitted_at = now`, `submitted_by = batch.confirmed_by`, `frozen_version = 1`).
3. The CTR file is not signed by an MLRO sign task: the confirmation of the batch is its approval. So the file goes from `validated` to `packaged` in the job itself (the database CHECK allows `packaged` without `signed_at` for `type = 'ctr'`), and the manifest records `batch_confirmed_by` and `batch_confirmed_at`. The package build is section 3.8 steps 3 to 7 without step 1's signature check.
4. Set `ctr_items.report_id` and `file_no` for each file (`set_item_files`, the trigger allows these two columns only).
5. Batch to `packaged`; audit `ctr.packaged {batch_id, files, manifest_sha256_list}`. Filing of each file uses section 3.10 (goAML upload by the named officer); the CTR filing is the same act as any other report's.

### 3.15 Letters (`LetterService`, job `letter_render`; RP-56; phase 20 section 3.6; routes `POST /letters`, `GET /letters/{id}`, `POST /letters/{id}/sign`, `POST /letters/{id}/delivery`)

**The port (phase 20 `LetterRequester::request`, implemented here).** The caller is phase 20's transaction, and the owner row (freeze or refrain) is already locked by it (phase 20 lock order, section 3.13 step 2):
1. Map the kind: `CommitteeFreezeReport` to `freeze_report` (recipient `committee`), `SecondFreezeReport` to `freeze_second_report` (recipient `committee`), `CommitteeReferral` to `referral` (recipient `committee`). `copy_to_office` is taken from the request, and phase 20 sends `TRUE` for the freeze report and the referral (its section 3.6).
2. `facts` must hold only the keys the phase 20 request names (ids, codes, counts, dates). Any other key is refused (`LetterError::Facts`, mapped to 500 `E_INTERNAL` with the key name in the log, and the owner's transaction rolls back).
3. `letter_no` from `rpt.letter_no_seq` (a rolled-back request uses a number that is never reused: the sequence is not gapless, and Open doubts 23 records it).
4. Insert `rpt.letters` (`state = requested`, `source_kind = freeze` or `refrain`, `source_id`, `case_id` from the owner, `requested_by` from the context or NULL for a timer) and enqueue `letter_render {letter_id}` (coalesce `letter_render:<letter_id>`) in the same transaction.
5. Audit `letter.requested {letter_id, letter_no, kind, source_kind}`. Return the letter id to phase 20, which stores it on the owner (`committee_report_letter_id`, `second_report_letter_id` or `referral_letter_id`).
6. A database error inside the port is `LetterError::Unavailable`, mapped to 503 `E_UNAVAILABLE` (phase 20 section 3.6): the owner keeps its time NULL and its timer retries.

**Render** (job `letter_render`): load the letter (state `requested`; otherwise done). Read the owner row (the freeze, the refrain, the authority request or the report) and fill the template `letter.<kind>.v1` in Arabic and English from the record, not from a stored text. `RenderService::render` (phase 02); `put_generated` (kind `render`, owner kind `letter`, owner id = the letter, retention class R7 for freeze and refrain letters and R6 otherwise); one transaction: `pdf_evidence_id`, `state = rendered` (the trigger refuses a move back). Audit `letter.rendered {letter_id, pdf_evidence_id}`.

**Sign** (`POST /letters/{id}/sign`; scope `report.file`; Cedar `letter.sign`, role `M`; step-up class `signing` (declared `letter.sign`); `If-Match`): state `rendered` (409 `not_rendered`); `signed_by` differs from `requested_by` (403 `E_SELF_APPROVAL`, enforced by the database CHECK too). Set `signed_by`, `signed_at`, `state = signed`; audit `letter.signed {letter_id}`.

**Delivery** (`POST /letters/{id}/delivery`; scope `report.file`; Cedar `letter.deliver`): state `signed` (409 `not_signed`). Body: `channel` (`letter` or `email`), `delivered_at` (not in the future), `delivery_reference` (the courier or message reference the officer types). No e-mail is sent by this phase: `email` records a delivery that happened outside the system (Open doubts 8). `state = delivered`; audit `letter.delivered {letter_id, channel}`. No owner is told: when the owner is a freeze, the phase 20 deadline handler reads the letter state through `LetterRequester::delivered` (section 4.5 item 9; 7.5).

**Manual letters** (`POST /letters`; Cedar `letter.create`; role `M`): `office_reply` and `hold_reply` only. The source is a phase 19 `authority_request` (its `id`, and a status of `approved` or later, else 409 `not_approved`) or a report. The facts are the claim ids and the RFI ids of the answer (phase 19 `response_rfi_id`) and the disclosure entry ids. The letter does not change the phase 19 status of the request; phase 19 owns that status (Open doubts 16). The sign and delivery steps are those above.

**Backfill** (`LetterBackfill`, job `letter_backfill`, one run, `akr-ctl letters backfill` in the install step of this phase, idempotent): phase 20's `RecordingLetterRequester` returned an id that the owner stored (`freezes.committee_report_letter_id`, `second_report_letter_id`, `referral_letter_id`; `refrain_instructions.referral_letter_id`) with no letter row behind it. The backfill reads those columns where no `rpt.letters` row has the id and inserts the letter under that same id through the port's steps 1 to 4 (kind from the column, `requested_at` from the owner's matching time column, `requested_by` NULL), so no owner row is changed and no phase 20 method is added (lead review); audit `letter.backfilled {letter_id, owner_kind, owner_id}`. The owner's text is never copied: the letter is rendered from the owner row.

### 3.16 Periodic packs (`PeriodicReportService`, job `periodic_build`; RP-49, RP-68, RP-70; routes `GET /reports/periodic`, `POST /reports/periodic`, `GET /reports/periodic/{id}`, `POST /reports/periodic/{id}/sign`)

Kinds and their period form: `cbi_monthly` (`YYYY-MM`), `liaison` (`YYYY-MM`), `board` (`YYYY-MM`), `analytics` (`YYYY-MM`), `mlro_annual` (`YYYY`), `fatca` (`YYYY`), `crs` (`YYYY`).

1. **Generate** (`POST /reports/periodic`; scope `report.file`; Cedar `periodic.generate`, role `M`; `Idempotency-Key`): the kind and the period are checked (422 `E_VALIDATION` at `/period`). A final pack for the same kind and period is 409 `E_NOT_ALLOWED_IN_STATE` sub-code `already_final`; a draft is superseded by the new draft in the same transaction (the partial unique index keeps one live pack). The row is inserted as `draft` with the period bounds and `generated_by` = the requesting user, and `periodic_build` is enqueued (coalesce `periodic_build:<id>`); 202.
2. **Build** (job): the facts of the kind (section 3.17 and the list below). Every count below the setting `report.small_number_threshold` (governed, minimum 3) and above zero is replaced by null and counted in `suppressed_cells`; a percentage whose numerator or denominator is suppressed is null too. Then `RenderService::render` of the template of the kind (`periodic.<kind>.v1`, with `template_version`), `put_generated` (kind `render`, owner kind `periodic_report`, owner id = the pack, retention R6), and the row update (`facts`, `suppressed_cells`, `evidence_id`; `generated_by` is set at the insert of step 1 and has no UPDATE grant). Audit `periodic.generated {periodic_id, kind, period, suppressed_cells}`.
   - `cbi_monthly`: the counts this system holds for the month (STRs, UTRs and TFRs by status; CTR files and items; freezes opened and released; letters by kind; alarms by kind). The three CBI standard reports named by RP-68 (account movements including staff accounts, the transfers register and the correspondent account activity) need data the canonical copy does not carry in the form the CBI asks for; they are Open doubts 18, and the pack says so on its cover.
   - `liaison`: liaison monthly reports received in the month from phase 19's `internal_reports` (channel `liaison_monthly`) by status, and how many arrived after the tenth of the following month.
   - `board`: reports signed, filed, acknowledged and rejected; grades; CTR files; open freezes; open alarms by kind. No names.
   - `mlro_annual`: the facts pack for the year (the counts of the other kinds over twelve months) and the bank's own template text (RP-68: the MLRO annual report has no set format, Open doubts 11).
   - `analytics`: section 3.17.
   - `fatca` and `crs`: no source of FATCA or CRS facts exists in the canonical copy (phase 15 has none; Open doubts 21). The job fails with `no_source`, and the pack stays `draft` with the reason; the kind is available once a source is configured.
3. **Sign** (`POST /reports/periodic/{id}/sign`; scope `report.file`; Cedar `periodic.sign`; step-up class `signing` (declared `periodic.sign`); `If-Match`): status `draft` with `evidence_id` (409 `not_built`); `signed_by` differs from `generated_by` (403 `E_SELF_APPROVAL`). Status `final`, `signed_at`; audit `periodic.signed`. A final pack is never edited (the database CHECK and the service).
4. **Due dates** (RP-49, the regulatory calendar): `due_on = period_end + setting report.periodic_due_days` (calendar days; the CBI dates are Open doubts 18). `GET /reports/periodic` returns `due_on` and `overdue` for each draft or missing pack of the calendar kinds. No timer: the overdue state is read, and it appears on the R3 screen.

### 3.17 Analytics (`AnalyticsService`; M-31, M-41, RP-31, RP-64, RP-65, RP-76, RP-84, DR-138)

Every metric is an aggregate over one period, computed by a read at build time, with the small-number rule of section 3.16. None carries a name, an account, a customer or a case reference.
1. Decision-to-filing (M-31, RP-31): for the reports signed in the period and filed, the hours from `signed_at` to `filed_at`: median, 90th percentile, maximum, and the count and share over `report.filing_target` (24 hours). The header clock of R1 reads the same figure.
2. First-alert-to-filing per scenario (M-31): from the alert `created_at` (phase 17) through the case opening (`casework.cases.opened_at`) to `filed_at`: median, 90th percentile and maximum per scenario code. Scenarios with fewer than the threshold reports are suppressed.
3. Activity age (M-41): the share of filings whose `activity_age_months` exceeds `report.activity_age_months_threshold`, by report type.
4. Timeliness (RP-31): the filings over the target and the count of explanations recorded (Open doubts 20 names the explanation field this phase does not yet store).
5. Feedback (RP-84): the grades received (`office_grade`) by type; the first-time pass rate (filed reports acknowledged without any rejection row, over filed reports); the rejection rule codes (`rpt.report_findings` with `source = office`, counted by `rule_code`); the sessions and requests by kind.
6. Closed without filing (RP-65): the cases closed in the period with outcome `no_report` (phase 18 `casework.decisions`), by the closure reason code and by the count per analyst (the analyst is shown to M and IA only, and only above the threshold).
7. Reopen rate (M-40, named by phase 18 and phase 19): the share of cases closed in the period that were reopened in the period, from phase 18's reopen record (the `case.reopened` chain event of phase 18 section 3.13; Open doubts 29).
8. Contribution (DR-138): the decisions of the period by their contribution outcome (phase 18's mapping: `no_report` to `explained`, `utr`, `str` and `tfr` to `not_explained`), and the reason-code distribution of the decisions.
9. Traceability (RP-64): not in this phase's pack. Each figure of a report links to its evidence snapshot rows through the phase 17 snapshot and the phase 02 hashes; the trace view for the D2 auditor is the G8 screen of phase 24 (Open doubts 19).

### 3.18 Clocks and timers (`ReportClockHandlers`; RP-46, RP-47; section 4.2)

The report clocks are entries of the phase 09 closed `ClockRegistry`, with a shipped value and bounds (phase 09 section 3.3). They are not settings, so a change goes through the registry's rules:
- `report.filing_target`: hours; shipped 24; bounds 1 to 24. The internal target of RP-47 (filing the same day as the decision, escalation at 24 hours); the legal text says "immediately" and gives no number (research file 33, row 5), so this value is internal and not a deadline claim. `fixed: false`.
- `report.rework`: working days; shipped 10; bounds 5 to 20. The rework period of RP-43 is not in the research; the shipped value is a proposal (Open doubts 7). `fixed: false`.
- The CBI-set value `CTR_THRESHOLD_USD = 10000` is a code constant of `domain::reporting`, not a registry entry (phase 09's registries hold clocks and switches only); it mirrors the DSL section 9.1 pinned `limit` (`min = max`), so no path can change it.

Timers (`admin.timers`, phase 00; `calendar_id` NULL for hours, the bank calendar for working days through `BusinessClock::add`):
1. `report_filing_target`, owner `report`, fires at `signed_at + report.filing_target hours`, scheduled by the sign act. Handler: if the report is `signed` or `packaged` (not filed), raise alarm `report_filing_overdue` (subject `report`) through `AlarmService::raise` (phase 19), which the supervisor sees on the alarm register; the escalation notice is in-app only and waits for phase 22. Cancelled by the filing.
2. `report_rework_due`, owner `report_task` (`returned_for_completion`), fires at `due_at`, scheduled by the poor grade. Handler: if the task is open, raise alarm `report_rework_overdue` (subject `report_task`) to M.
3. Backstop: `report_clock_scan` (hourly, section 4.1) enqueues the handler job of each due, unhandled timer; each handler re-reads its owner first and does nothing when the owner has moved on.
4. Periodic due dates are read, not timed (section 3.16 step 4), and the FX waits are the `ctr_fx_wait` job (section 3.13 step 6).

### 3.19 Errors (`ReportError`, `thiserror`; API section 6 codes only)

| Variant | Problem code | Status | Sub-codes and notes |
|-|-|-|-|
| `NotFound` | `E_NOT_FOUND` | 404 | also the masked answer of section 3.22 |
| `Forbidden` | `E_FORBIDDEN` | 403 | `role_not_permitted`; Cedar deny |
| `Scope` | `E_SCOPE` | 403 | the scope gate of API section 2 |
| `SelfApproval` | `E_SELF_APPROVAL` | 403 | maker, submitter, proposer, earlier decider, batch creator, letter requester, pack generator |
| `StepUpRequired` | `E_STEPUP_REQUIRED` | 401 | missing, reused or wrong class |
| `PurposeRequired` | `E_PURPOSE_REQUIRED` | 422 | the package download |
| `Validation` | `E_VALIDATION` | 422 | `errors[]` with `reference_format`, `too_long`, `scan_hit`, `unknown_code`, `narrative_limit`, `non_working_day`, `period` |
| `ReportInvalid` | `E_REPORT_INVALID` | 422 | submit with block findings; the findings listed |
| `UploadRejected` | `E_UPLOAD_REJECTED` | 422 | type, macro or antivirus refusal of a staged file |
| `NotAllowedInState` | `E_NOT_ALLOWED_IN_STATE` | 409 | `no_reportable_proposal`, `not_validated`, `version_stale`, `not_signed`, `not_packaged`, `not_filed`, `not_gradeable`, `not_revisable`, `not_computed`, `not_confirmed`, `not_rendered`, `not_built`, `already_final`, `already_acknowledged`, `reference_in_use`, `filing_not_acknowledged`, `report_sign_required`, `no_source` |
| `CaseSealed` | `E_CASE_SEALED` | 409 | a sign or a case-level act on a sealed case |
| `VersionConflict` | `E_VERSION_CONFLICT` | 409 | `If-Match` does not match; the current version in the body |
| `PreconditionRequired` | `E_PRECONDITION_REQUIRED` | 428 | versioned mutation without `If-Match` |
| `HashMismatch` | `E_HASH_MISMATCH` | 500 | a frozen version or a package failed its hash on serve (phase 02 incident) |
| `Unavailable` | `E_UNAVAILABLE` | 503 | the active profile, the renderer, the letter engine or the evidence store is down |
| `Internal` | `E_INTERNAL` | 500 | an invariant the code asserts; never a value in the message |

The idempotency codes (`E_IDEMPOTENCY_REQUIRED`, `E_IDEMPOTENCY_IN_FLIGHT`, `E_IDEMPOTENCY_REUSE`) are phase 00's and are used as they are.

### 3.20 Audit events (one per mutation; payloads carry ids, codes, counts and hashes; never a name, an amount in free text, a reason text or a value from a file)

| Event kind | On | Resource |
|-|-|-|
| `report.created` | draft created (3.1) | report |
| `report.validated` | validation run written (3.4 step 9) | report version |
| `report.submitted` | version frozen (3.5) | report version |
| `report.revised` | new draft version after rejection or return (3.5a) | report |
| `report.signed` | sign effect (3.7) and correction effect | report |
| `report.packaged` | package row written (3.8, 3.14) | report |
| `report.package_downloaded` | package served (3.9) | report |
| `report.filed` | filing row written (3.10) | report |
| `report.acknowledged` | receipt recorded (3.10) | report |
| `report.rejected` | Office rejection recorded (3.11) | report |
| `report.returned` | poor grade; task opened (3.12) | report |
| `report.task_closed` | returned task closed on resubmission (3.12 step 8) | report task |
| `report.attachment_staged` | attachment staged (3.2) | report |
| `report.linked` | link written (`POST /reports/{id}/links` is not a route: links come from the draft's `earlier_reports` and from the escalation; each write audited) | report |
| `feedback.recorded` | feedback row (3.12) | feedback |
| `report_task.created` | returned, FX or control task (3.12, 3.13) | report task |
| `ctr.waiting_source` | batch waits for the source (3.13 step 2) | ctr batch |
| `ctr.waiting_fx` | batch waits for a rate (3.13 step 6) | ctr batch |
| `ctr.computed` | items written, batch computed (3.13 step 11) | ctr batch |
| `ctr.confirmed` | checker confirmation (3.14) | ctr batch |
| `ctr.packaged` | files built (3.14) | ctr batch |
| `ctr.superseded` | a manual run replaced the live batch (3.13 step 1) | ctr batch |
| `letter.requested` | the port or a manual request (3.15) | letter |
| `letter.rendered` | PDF stored (3.15) | letter |
| `letter.signed` | signature (3.15) | letter |
| `letter.delivered` | delivery record (3.15) | letter |
| `letter.backfilled` | stub request re-created (3.15) | letter |
| `periodic.generated` | pack built (3.16) | periodic pack |
| `periodic.signed` | pack finalised (3.16) | periodic pack |
| `report.filing_overdue`, `report.rework_overdue` | alarm raised by the timers (3.18), through `alarm.raised` (phase 19) | alarm |

The phase 20 stub event `ops.letter_recorded` is the last event of that kind; this phase writes `letter.requested` in its place.

### 3.21 Tipping-off review (what a person outside the access list can learn; the answer must be "nothing")

- **Report list and detail (R2, R1).** A user without the case access list gets the masked 404 of section 3.22: the same bytes as for a missing report. Counts exclude suspicion reports for everyone not on the list (RP-61, RP-76).
- **Feedback register.** A row for a suspicion report appears only to the case team and M. A session row (no report) shows the Office's named error and the change made, and no case.
- **Packages and downloads.** The ZIP holds the party data and the narrative; download is step-up, purpose-bound and audited. The manifest and the file names are generated (`att-NNN`), so no source name reaches an outsider.
- **Letters.** The freeze letter names the subject and goes to the Committee and, by copy, to the Office: that is the reporting channel, which is its purpose, and the letter is visible only to M and to the case team. A letter with no case is visible to M alone. The phase 20 freeze screen shows only the letter's state and number to the operations user, never its content (the operations user sees the neutral view).
- **CTR (R2 counts).** The batch shows counts and totals to A2; the customer identity and the transaction references are visible to M only. The CTR file is a report: no outsider sees it.
- **Periodic packs (R3).** Aggregates only, with cells below the threshold suppressed. The board pack names no person.
- **Analytics.** Scenario and analyst counts below the threshold are suppressed; the analyst is shown to M and IA only.
- **Notices and alarms.** `report.filing_overdue` and `report.rework_overdue` are alarms with ids and kinds; the in-app notice text comes from the phase 09 neutral-wording library (RP-62) and names no case. The outbox carries ids only.
- **Errors.** Messages never contain a name, a value, or a reason text (section 3.19).

### 3.22 Visibility and Cedar (the file `A1_reports.cedar`; phase 03 bundle)

**Visibility predicates** (SQL, the same rules as the Cedar attributes, so that lists, counts and details agree):
- **V1 (reports).** `TRUE` for role `M`. Otherwise `r.case_id IS NOT NULL AND EXISTS (SELECT 1 FROM casework.case_acl a WHERE a.case_id = r.case_id AND a.principal_id = $principal AND a.revoked_at IS NULL)`. A CTR file (`case_id` NULL) is visible to M only. Phase 18's `case_acl` has no expiry column; a lapsed grant is a revoked one.
- **V2 (letters).** `TRUE` for `M`; for a letter with a `case_id`, the V1 predicate of its case; otherwise `FALSE` for everyone except `M`.
- **V3 (feedback register).** `TRUE` for `M`; for a row with a report, the V1 predicate of the report; a session row (no report) for `M` and `A2`.
- **V4 (CTR batches).** Batch counts and totals: `A2` and `M`. The items: `M` only.
- **V5 (periodic packs).** `dashboard.read` roles (`M`, `CH`, `IA`) read; generation and signature are `M`.

**Cedar entities** (appended to `schema.cedarschema` in the phase 03 form; `User` and `Ctx` are phase 03's, `Case` is phase 17's):

```cedar
entity Report = { report_type: String, status: String, tier: String, has_case: Bool, case_team: Set<User>, created_by?: User };
entity CtrBatch = { status: String };
entity Letter = { kind: String, state: String, has_case: Bool, case_team: Set<User>, requested_by?: User };
entity PeriodicPack = { kind: String, status: String };
entity ReportTask = { kind: String, state: String };
```

**Actions** (the `appliesTo` of each action; phase 20 section 3.12 is the idiom):

```cedar
action "report.create" appliesTo { principal: User, resource: Case, context: Ctx };
action "report.read", "report.edit", "report.validate", "report.submit", "report.sign", "report.package", "report.package_read",
       "report.reject", "report.feedback", "filing.record", "filing.acknowledge" appliesTo { principal: User, resource: Report, context: Ctx };
action "report.task.read" appliesTo { principal: User, resource: System, context: Ctx };
action "feedback.read", "ctr.run", "ctr.read_list", "letter.create", "periodic.generate" appliesTo { principal: User, resource: System, context: Ctx };
action "ctr.read", "ctr.confirm", "ctr.package" appliesTo { principal: User, resource: CtrBatch, context: Ctx };
action "letter.read", "letter.sign", "letter.deliver" appliesTo { principal: User, resource: Letter, context: Ctx };
action "periodic.read", "periodic.sign" appliesTo { principal: User, resource: PeriodicPack, context: Ctx };
```

`ctr.read_list` is the list of batches (resource System); `ctr.read` is one batch. The list-level and the item-level checks are the SQL predicates V1 to V5 of this section, which the Cedar policies mirror.

**Step-up action constants** (declared in code as `StepUpAction { name, class }`, phase 01 classes only): `report.sign` (`signing`), `letter.sign` (`signing`), `periodic.sign` (`signing`), `report.filing.record` (`filing`), `report.filing.acknowledge` (`filing`), `report.package.download` (`export`), `ctr.confirm` (`approval`). Step-up is never a Cedar condition.

**The file** (`A1_reports.cedar`, copy-paste ready; one role per `@id`):

```cedar
// A1_reports.cedar (phase 21). Scope gates are API.md section 2 (report.prepare, report.file, dashboard.read, ctr.read through
// report.prepare); these policies are the fine gate. Step-up and the four-eyes rule are not conditions here: the handlers consume
// the step-up and the phase 04 engine applies the exclusions. The file name is the reporting range requested for phase 21; see Open doubts 22.

@id("report.preparers") @role("A2,M")
permit (principal, action in [Akr::Action::"report.create", Akr::Action::"report.read", Akr::Action::"report.edit",
        Akr::Action::"report.validate", Akr::Action::"report.submit", Akr::Action::"report.task.read",
        Akr::Action::"feedback.read", Akr::Action::"ctr.read", Akr::Action::"ctr.read_list"], resource)
when { principal.roles.contains("A2") || principal.roles.contains("M") };

@id("report.case_team_read") @role("A2")
permit (principal, action == Akr::Action::"report.read", resource is Akr::Report)
when { principal.roles.contains("A2") && resource.has_case && resource.case_team.contains(principal) };

@id("report.masked") @deny("masked")
forbid (principal, action == Akr::Action::"report.read", resource is Akr::Report)
when { !principal.roles.contains("M") && !(resource.has_case && resource.case_team.contains(principal)) };

@id("report.filers") @role("M")
permit (principal, action in [Akr::Action::"report.sign", Akr::Action::"report.package", Akr::Action::"report.package_read",
        Akr::Action::"report.reject", Akr::Action::"report.feedback", Akr::Action::"filing.record",
        Akr::Action::"filing.acknowledge", Akr::Action::"ctr.run", Akr::Action::"ctr.confirm", Akr::Action::"ctr.package",
        Akr::Action::"letter.create", Akr::Action::"letter.read", Akr::Action::"letter.sign", Akr::Action::"letter.deliver",
        Akr::Action::"periodic.generate", Akr::Action::"periodic.sign", Akr::Action::"periodic.read"], resource)
when { principal.roles.contains("M") };

@id("report.edit_drafts") @role("A2")
permit (principal, action in [Akr::Action::"report.edit"], resource is Akr::Report)
when { principal.roles.contains("A2") && resource.has_case && resource.case_team.contains(principal) && resource.status == "draft" };

@id("report.file_scope_deny") @deny("role_not_permitted")
forbid (principal, action == Akr::Action::"report.sign", resource is Akr::Report)
when { resource.report_type == "ctr" };

@id("letter.case_team") @role("A2")
permit (principal, action == Akr::Action::"letter.read", resource is Akr::Letter)
when { principal.roles.contains("A2") && resource.has_case && resource.case_team.contains(principal) };

@id("periodic.readers") @role("M,CH,IA")
permit (principal, action == Akr::Action::"periodic.read", resource is Akr::PeriodicPack)
when { principal.roles.contains("M") || principal.roles.contains("CH") || principal.roles.contains("IA") };

@id("report.write_masked") @deny("masked")
forbid (principal, action in [Akr::Action::"report.edit", Akr::Action::"report.validate", Akr::Action::"report.submit"], resource is Akr::Report)
when { !principal.roles.contains("M") && !(resource.has_case && resource.case_team.contains(principal)) };

@id("letter.requester_not_signer") @deny("self_approval")
forbid (principal, action == Akr::Action::"letter.sign", resource is Akr::Letter)
when { resource has requested_by && resource.requested_by == principal };
```

### 3.23 Settings, registries and the fixed values used here

- Settings (`admin.settings`, section 1.3): `ctr.aggregate_same_day`, `ctr.file_max_transactions`, `report.small_number_threshold`, `report.letter_channel`, `report.periodic_due_days`, `report.single_officer_exception`, `report.large_str_usd`, `report.activity_age_months_threshold`; schedules `schedule.ctr_daily`, `schedule.ctr_fx_wait`, `schedule.report_clock_scan`, `schedule.report_calibration`. Every governed setting names its floor in the migration; no setting holds a constant that the bank may want to change.
- Clock registry (phase 09, section 3.3): `report.filing_target` and `report.rework`, both `fixed: false`.
- Fixed CBI value (a code constant of `domain::reporting`, lead review): `CTR_THRESHOLD_USD = 10000`. The DSL pins the same figure (section 9.1, `limit`, `min = max`); a bank cannot move it.
- Profile clocks (phase 07 column `rpt.goaml_profiles.clocks`, "filled by phase 21"): the active profile carries the two report clocks as a copy of the registry values, written by the profile seed (section 4.5 item 7).
- Purpose codes: the phase 03 list is admin data; the reporting purposes are Open doubts 13.

### 3.24 Lock order (phase 18 section 3.16 extended; DR-169)

1. The workflow instance and its tasks (phase 04 `WorkflowEngine::act`, `start`, `claim`, `release`, `withdraw`; invariant I1).
2. The case row: `CaseRepository::lock(tx, case_id)` (phase 18 section 3.16 step 2). Taken by the sign act's subject check, by the decision and by every case route that creates a report.
3. The report row: `rpt.reports` (`lock`, `lock_by_proposal`), then its version (`report_versions`), then its package, filing, feedback, findings and links, in that order.
4. For a CTR batch: `rpt.ctr_batches` first, then its items.
5. For a letter: the owner row first (phase 20's freeze or refrain, which phase 20 locks before it calls the port), then `rpt.letters`. The port takes no report lock.
6. `rpt.report_tasks` after the row it refers to (the report, the batch or the feedback).
7. Below all of these: `admin.timers` (`TimerService`), the job queue (`enqueue`), `casework.alarms` (`AlarmService`), and the audit chain.

No path takes a lower step and then a higher one. The close gate of section 3.10a reads the filing after the case lock and takes no report lock. A letter request never waits for a report lock.

## 4. Infrastructure Updates

### 4.1 Job kinds (Postgres `jobs`; coalesce key in brackets)

| Kind | Binary | Payload | Concurrency, attempts | Schedule or trigger |
|-|-|-|-|-|
| `report_validate` | worker | `{ report_id, version }` [`report_validate:<report_id>`] | 2, 3 | `POST /reports/{id}/validate` (section 3.4) |
| `report_package` | worker | `{ report_id }` [`report_package:<report_id>`] | 2, 3 | `POST /reports/{id}/package` (section 3.8) |
| `ctr_daily` | worker | `{ business_date, manual }` [`ctr_daily:<business_date>`] | 1, 5 (back-off) | `schedule.ctr_daily` after the business-date close; `POST /ctr/batches` (section 3.13) |
| `ctr_fx_wait` | worker | `{}` [`ctr_fx_wait`] | 1, 3 | `schedule.ctr_fx_wait` (hourly); re-runs the `waiting_fx` and `waiting_source` batches (section 3.13 step 6) |
| `ctr_package` | worker | `{ batch_id }` [`ctr_package:<batch_id>`] | 1, 3 | `POST /ctr/batches/{id}/package` (section 3.14) |
| `letter_render` | worker | `{ letter_id }` [`letter_render:<letter_id>`] | 4, 5 | enqueued by the port in phase 20's transaction and by the manual letter route (section 3.15) |
| `periodic_build` | worker | `{ periodic_id }` [`periodic_build:<periodic_id>`] | 1, 3 | `POST /reports/periodic` (section 3.16) |
| `report_clock_scan` | worker | `{}` [`report_clock_scan`] | 1, 3 | `schedule.report_clock_scan` (hourly, minute 9); backstop for the two report timers (section 3.18) |
| `report_calibration` | worker | `{}` [`report_calibration`] | 1, 2 | `schedule.report_calibration` (monthly, first day, 06:00); draws the RP-29 calibration sample from the reports filed in the month |
| `letter_backfill` | worker | `{}` [`letter_backfill`] | 1, 1 | one-time; enqueued by `akr-ctl letters backfill` in the install step of this phase; idempotent (section 3.15) |

Every handler re-reads its owner first. The release and the signature are workflow acts in the request transaction (akr-api), as in phase 20. No new job prefix is used (`ingest_` is for akr-ingestd only).

### 4.2 Timers (phase 00 `admin.timers`; the kind registry gains two kinds)

| Timer kind | Owner kind | Fires at | Handler job | Action (after the re-read) |
|-|-|-|-|-|
| `report_filing_target` | report | `signed_at` plus `report.filing_target` hours | `timer_report_filing_target` | alarm `report_filing_overdue` while the report is `signed` or `packaged` (section 3.18) |
| `report_rework_due` | report_task | `due_at` (working days from the poor grade, `BusinessClock::add`) | `timer_report_rework_due` | alarm `report_rework_overdue` while the task is `open` or `claimed` |

Both are scheduled by the transaction that creates or changes the owner, with `calendar_id` NULL for hours and the bank calendar for working days. `cancel_for_owner` runs at filing (the filing target) and at the close of the task (the rework timer).

**Outbox.** No new topic. `alarm.raised` (phase 19) carries the two new alarm kinds with ids and kinds only.

### 4.3 AppState additions

- `akr-api`: `reports: Arc<ReportService>`, `approvals: Arc<ReportApprovalService>`, `downloads: Arc<DownloadService>`, `filings: Arc<FilingService>`, `rejections: Arc<RejectionService>`, `feedback: Arc<FeedbackService>`, `ctr: Arc<CtrService>`, `letters: Arc<LetterService>` (replaces `RecordingLetterRequester` as the `LetterRequester` port of phase 20), `periodic: Arc<PeriodicReportService>`, `analytics: Arc<AnalyticsService>`, `report_query: Arc<ReportQuery>`.
- `akr-worker`: `packager: Arc<PackageService>`, `ctr_jobs: Arc<CtrService>`, `letter_renderer: Arc<LetterService>`, `periodic_builder: Arc<PeriodicReportService>`, `clock_handlers: Arc<ReportClockHandlers>`, `letter_backfill: Arc<LetterBackfill>`, `calibration: Arc<AnalyticsService>`, and the `ReportService` validation body. The phase 07 `CheckService` and `SandboxedXsdValidator` are shared through the handlers' existing `Arc`s.
- `akr-ctl`: `letters backfill` (one-time), `ctr run --date YYYY-MM-DD` (a manual run for a day the job missed, audited as a manual run), `reports calibrate` (the calibration sample on demand). Each is idempotent.

### 4.4 Network zones, metrics, readiness, OpenAPI

- **Zones**: no new edge. The worker renders through the existing phase 02 Gotenberg path and stores through the phase 02 evidence store; the API reads the canonical copy (phase 10) and the evidence store. No binary reaches the Office, the CBI or any e-mail service: filing and delivery are a person's acts outside the system (RP-39, Open doubts 8).
- **Metrics**: `akr_reports_open{status}` (gauge), `akr_report_filings_total`, `akr_report_rejections_total`, `akr_report_returned_total`, `akr_report_decision_to_filing_hours` (histogram), `akr_ctr_batches{status}` (gauge), `akr_ctr_fx_waiting_days` (gauge), `akr_letters_open{kind}` (gauge), `akr_periodic_packs_total{kind}`. No metric carries a party, an account, a customer, a report reference or an amount.
- **Readiness**: the readiness check of phase 07 (an active profile exists) is reused; no new check.
- **OpenAPI**: tags `reports` (the report, periodic and attachment routes), `filings`, `ctr`, `letters`, `fiu-feedback`, `report-tasks`. Schemas: the views and commands of section 2 (the sealed content is never a schema). `make generate-api` regenerates the client in this change.

### 4.5 Existing entity, schema, route and code updates (extensions of earlier phases)

1. **Phase 18 `CaseWorkflowSubject::check_before_decision`** (phase 18 section 3.8 d) gains, for the instances of `case_report_approval`, the report checks of section 3.6 step 3: the report exists, `status = validated`, `frozen_version = current_version`, no block in the latest run, and the act carries `via = "report"` (otherwise the refusal of item 2). This phase does not edit phase 18's text; the gap pass applies this item.
2. **Phase 18 decision route** (`POST /cases/{id}/decisions`, action `sign` on `case_report_approval`) and the **phase 04 generic task route** for a `sign` task of `case_report_approval` or `report_correction` refuse with 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `report_sign_required`, unless `via = "report"`.
3. **Phase 18 close gate** (phase 18 section 3.12 step 5; its Open doubts 10): replaced by the acknowledged-filing check of section 3.10a. The sub-code `filing_not_released` is replaced by `filing_not_acknowledged`.
4. **Phase 18 `ReleaseForFilingEffect`** (section 3.8 step g): the body of section 3.7 of this phase. The kind `release_for_filing` and the resource kind `case` do not change; phase 18 said its body is completed here.
5. **Phase 04 registries and DSL E.4**: subject `ReportWorkflowSubject` and effect `release_correction` (resource kind `report`) registered; definition `report_correction` published by a `cr_workflow` change request: `sign` (M at bank scope, step-up `signing`) then `release_correction`. The step-up class of the sign step of `case_report_approval` is `signing` (DR-170), where phase 18 section 3.8 c declares `decision`; the DSL E.4 row is unchanged and the phase 18 declaration changes by the same gap pass.
6. **Phase 07 engine**: `CheckService::validate_built(bytes, profile, layers)` (the pipeline of section 3.4 for an already-built XML, with the same layers as an upload; the upload path is not changed); `RejectionParser` completed (section 3.11); rule codes `CONS-05`, `NAME-01`, `DATE-01`, `ID-01`, `PKG-07`, `CTR-01`, `CTR-02` and `CTR-03` added to the rule catalogue, which is closed in code and versioned with the release, so these arrive in a release, not in a profile amendment.
7. **Phase 07 profile**: the active `IQ` profile's `clocks` JSONB gets the two report clocks (section 3.23) through the profile seed (`akr-ctl packs seed`, extended by this phase's install step).
8. **Phase 09**: the closed `ClockRegistry` gains `report.filing_target` and `report.rework`. No change to the FX rules, the calendars or the policy pack structure.
9. **Phase 20**: `RecordingLetterRequester` is removed from `AppState` and kept as a test double; no phase 20 method is added for the backfill (section 3.15); this phase implements `LetterRequester::delivered(conn, letter_id)` (phase 20 section 7+), which the phase 20 deadline handler reads.
10. **Phase 19**: `AlarmService::raise` accepts the two new alarm kinds (the CHECK widening is in migration 0070 of this phase, section 1.3).
11. **Phase 02**: the evidence owner registry gains `report`, `report_package`, `filing`, `letter`, `periodic_report` and `fiu_feedback`; the retention classes R6 and R7 already have lock policies in `evidence.lock_days`.
12. **Phase 00**: the timer kind registry gains `report_filing_target` and `report_rework_due`.
13. **API.md section 3.13**: the 18 owned rows are kept as written, with the scopes and step-up of section 2 of this phase; the 15 added rows of the route table are added to the table (Open doubts 9).
14. **DATA-MODEL section 14**: aligned by section 1 of this phase (the reconciliation list). `letters.number` becomes `letter_no`; the other changes are listed in section 1.
15. **Phase 14**: no change. A TFR does not follow a true match (phase 14 has no TFR; Open doubts 27).

## 5. Frontend Impact

Screens are the DESIGN-SYSTEM section 8.1 catalogue rows R1, R2 and R3 (R5, liaison report review, is phase 22's). The DESIGN-SYSTEM specification exists for R1 only (section 8.3); R2 and R3 have their catalogue row (type, route, pattern) and nothing more, so their layout follows the catalogue patterns and Open doubts 16 records what is missing.

### 5.1 R1 Report builder (`/reports/:id`; STEPPER; mockup 07; UX-62)

Six steps as the builder of section 3.2 (type and subject; origin of suspicion and earlier reports; indicators as chips; narrative with the live counter and the tipping-off phrase check; validation panel with links to the fields by `field_key`; review and sign with step-up). Step 7 is "Filed and acknowledged" with the goAML reference, the time and the receipt, the button "Download goAML XML" (step-up, purpose asked), and "Record filing reference". The decision-to-filing clock of section 3.17 item 1 is in the header. A version comparison appears when the report has more than one version. The FIU feedback strip appears after filing. The sign panel shows the warns of the latest run and the estimated grade labelled "estimated".

### 5.2 R2 Report list and FIU feedback (`/reports`; LIST)

A list of reports (suspicion tier; a report the user cannot see does not exist for that user) with tabs "Reports", "Feedback register", "Returned", "Waiting for a rate" (FX tasks) and "Control confirmation" (CTR batches awaiting the checker). The feedback register shows the grade, the kind, the date and the rule change made.

### 5.3 R3 Regulatory and periodic reports (`/reports/periodic`; REGISTER)

A register of periodic packs by kind and period, with `due_on`, "overdue" words, the suppressed-cell count, and the actions generate (M) and sign (M, step-up). The CTR batches are on R2, not here.

### 5.4 Generated hooks (Orval names from the paths)

`usePostCasesIdReports`, `useGetReports`, `useGetReportsId`, `usePatchReportsId`, `useGetReportsIdVersions`, `useGetReportsIdFindings`, `usePostReportsIdAttachments`, `usePostReportsIdValidate`, `usePostReportsIdSubmit`, `usePostReportsIdRevise`, `usePostReportsIdSign`, `usePostReportsIdPackage`, `useGetReportsIdPackage`, `usePostReportsIdFiling`, `usePostFilingsIdAcknowledgement`, `usePostReportsIdRejections`, `usePostReportsIdFeedback`, `useGetFiuFeedback`, `useGetReportTasks`, `useGetCtrBatches`, `useGetCtrBatchesId`, `usePostCtrBatches`, `usePostCtrBatchesIdConfirm`, `usePostCtrBatchesIdPackage`, `usePostLetters`, `useGetLettersId`, `usePostLettersIdSign`, `usePostLettersIdDelivery`, `useGetReportsPeriodic`, `usePostReportsPeriodic`, `useGetReportsPeriodicId`, `usePostReportsPeriodicIdSign`.

### 5.5 Browser and caching rules

Report content is not kept in the browser: no local storage, no service-worker cache. A draft is saved by PATCH, so it survives a session timeout through the server, as phase 00's session rule requires. The package download is an attachment response with `Cache-Control: no-store`. Every POST of this section carries an `Idempotency-Key`, as phase 18 requires.

## 6. Verification

1. `make migrate`: `0068_rpt_reports.sql`, `0069_rpt_letters_periodic.sql` and `0070_rpt_guards_grants.sql` apply cleanly, and a second run changes nothing (every table and index is `IF NOT EXISTS`; the triggers and the two `casework.alarms` constraints are dropped by name before they are created).
2. `cargo check`: compile-time SQL passes for every query of sections 2 and 3, including the reads of `canon.txn`, `canon.account`, `canon.party`, `canon.party_name`, `canon.party_identifier`, `ref.fx_rates` and `ref.governorates`.
3. `cargo clippy -- -D warnings`.
4. `cargo test`, including these unit and property tests:
   - Name split: a proptest of at least 10,000 generated Iraqi names (one to five tokens, connectors, Arabic and Latin) gives the same parts when split twice, loses no token, and the serialised person name holds no apostrophe, dot, comma, parenthesis or slash.
   - CTR threshold: a USD total of 10,000.00 is reportable and 9,999.99 is not; two cash rows of 6,000 USD each for one customer on one day are reportable with aggregation on and not reportable per transaction with it off; a NULL customer is never aggregated.
   - FX: a day with a missing rate for one currency gives `waiting_fx` and one open `fx_rate_missing` task; no code path reads `canon.txn.amount_usd` for a CTR amount (a unit test with a stored value that differs from the rate, and a grep test on the module).
   - State machine: every transition of section 3.5 is accepted from its source status and every other pair of the eight statuses is refused (a table test).
   - Freeze: a version with `frozen_at` set refuses an update in the database (a `DO` block as `akr_app`), and the service never sends one.
   - Determinism: `GoamlPackager::build` twice on the same content gives identical bytes, and the manifest hash is stable across runs.
   - Suppression: a count of 1, 2 and 4 with threshold 5 is null; 0 and 5 are shown; a percentage with a suppressed denominator is null.
   - Letter facts: a key outside the phase 20 request keys is refused, and the owner's transaction rolls back.
   - Clock: a poor grade with ten working days gives the date the bank calendar (phase 09 section 3.3) gives; a holiday fixture inside the window moves it by one working day.
5. Curl suite `tests/curl/phase21_reports.sh` (dev users, one per role: `a2@bank.test` (A2 with a grant on the fixture case), `a2b@bank.test` (A2 with no grant), `m@bank.test` (M), `ch@bank.test` (CH), `ia@bank.test` (IA), `cfg@bank.test`). Each step prints the status and the problem code, and the script stops at the first mismatch.
   - a. `a2@` `POST /api/v1/cases/{utr_case}/reports` with `Idempotency-Key: k-draft-1`: 201. The same key and body again: 200 with the same `id`. The same key with another body: 422 `E_IDEMPOTENCY_REUSE`.
   - b. `a2b@` `GET /api/v1/reports/{id}`: 404 `E_NOT_FOUND`; the response file is byte-identical to `GET /api/v1/reports/00000000-0000-7000-8000-000000000000` (`cmp`).
   - c. `a2@` `PATCH /api/v1/reports/{id}` without `If-Match`: 428 `E_PRECONDITION_REQUIRED`; with a stale version: 409 `E_VERSION_CONFLICT`; with the current version and `suspicion_formed_on`: 200.
   - d. `a2@` `POST /api/v1/reports/{id}/validate` with `If-Match`: 202; after the job, `GET /api/v1/reports/{id}/findings` lists `suspicion_date_missing` only if the date was not set, and `GET /api/v1/reports/{id}` shows `validated` when no block exists.
   - e. `a2@` `POST /api/v1/reports/{id}/submit` with a block in the latest run (fixture: a transaction whose booking date differs from the canonical copy): 422 `E_REPORT_INVALID`, and the findings carry no value. After the fixture is corrected and revalidated: 200.
   - f. `m@` `POST /api/v1/reports/{id}/sign` without the step-up header: 401 `E_STEPUP_REQUIRED`. With a fresh `signing` assertion and the current instance version: 200 with the instance view; `GET /api/v1/cases/{id}/events` shows `case.decided` and `case.filing_released`.
   - g. `a2@` `POST /api/v1/cases/{utr_case}/decisions` with `action: sign` on the `case_report_approval` task: 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `report_sign_required`.
   - h. `m@` `POST /api/v1/reports/{id}/package` with `If-Match`: 202; after the job, `GET /api/v1/reports/{id}` shows `packaged`. `GET /api/v1/reports/{id}/package` without `X-Access-Purpose`: 422 `E_PURPOSE_REQUIRED`; with the purpose and without step-up: 401 `E_STEPUP_REQUIRED`; with both: 200, and the ZIP's manifest hashes match the files (`sha256sum -c` on the extracted copy).
   - i. `m@` `POST /api/v1/reports/{id}/filing` (multipart: `goaml_reference` `"GOAML-FIX-0001"`, a PNG capture, `Idempotency-Key`, the filing step-up): 200. A second call with the same reference: 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `reference_in_use`. `POST /api/v1/cases/{utr_case}/close` before the acknowledgement: 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `filing_not_acknowledged`.
   - j. `m@` `POST /api/v1/filings/{id}/acknowledgement` with the receipt and the filing step-up: 200; the report is `acknowledged`; `POST /api/v1/cases/{utr_case}/close` now passes its report check.
   - k. `m@` `POST /api/v1/reports/{id}/feedback` with `kind: grade`, `grade: poor` on a second fixture report: 200; `GET /api/v1/report-tasks` lists one `returned_for_completion` task with `due_at` ten working days ahead; `POST /api/v1/reports/{id}/revise` by `a2@`: 200, status `draft`, `current_version` 2.
   - l. CTR: `m@` `POST /api/v1/ctr/batches` for a fixture date whose USD rate is missing for one currency: 202; `GET /api/v1/ctr/batches/{id}` shows `waiting_fx`; `GET /api/v1/report-tasks` lists `fx_rate_missing`. The rate is added through phase 09 (`POST /api/v1/ref/fx-rates`, data steward) and the job `ctr_fx_wait` runs: the batch is `computed` and the task is `done`. `m@` confirms a batch it created: 403 `E_SELF_APPROVAL`; a second M confirms with the `approval` step-up: 200. `POST /api/v1/ctr/batches/{id}/package`: 202; `GET /api/v1/reports?type=ctr` lists the packaged files.
   - m. `a2@` `GET /api/v1/ctr/batches/{id}`: 200 with counts and totals and no customer identifier (`jq` asserts no key `customer_id` and no `txn_refs`); `a2@` `GET /api/v1/reports?type=ctr`: 200 with no row (M only).
   - n. Letters: the freeze fixture of phase 20 step j requests a committee report; `GET /api/v1/letters/{id}` shows `requested`, then `rendered` with a PDF reference after the job; `m@` `POST /api/v1/letters/{id}/sign` as the requester: 403 `E_SELF_APPROVAL`; as a second M with the `signing` step-up: 200; `POST /api/v1/letters/{id}/delivery` with `channel: letter` and a reference: 200, state `delivered`.
   - o. Periodic: `m@` `POST /api/v1/reports/periodic` with kind `cbi_monthly` and period `2026-09`: 202; after the job, a cell below the threshold is null in `GET /api/v1/reports/periodic/{id}` and `suppressed_cells` is at least 1. `m@` `POST /api/v1/reports/periodic/{id}/sign` with the `signing` step-up: 200. Kind `fatca` for `2026`: the job fails with `no_source`, and `GET` shows the reason.
   - p. `ch@` `GET /api/v1/reports/periodic`: 200 (`dashboard.read`). `a2@` `GET /api/v1/reports/periodic`: 403 `E_SCOPE`.
   - q. Tipping-off grep: the response files of steps b, d and m and the audit payloads of the run contain none of the invented names of the fixture (`Ahmed Kareem Hassan`, `Layla Nabil Sami`), no account number and no amount in free text; `grep -c` returns 0.
   - r. Audit: the audit log after the run holds `report.created`, `report.validated`, `report.submitted`, `report.signed`, `report.packaged`, `report.package_downloaded`, `report.filed`, `report.acknowledged`, `report.returned`, `ctr.waiting_fx`, `ctr.computed`, `ctr.confirmed`, `ctr.packaged`, `letter.requested`, `letter.rendered`, `letter.signed`, `letter.delivered`, `periodic.generated` and `periodic.signed`, and no payload holds a name.
6. Swagger UI shows the 32 routes of section 2 under the six tags; `make generate-api` regenerates the client cleanly, and every hook of section 5.4 exists in the generated module.
7. The audit chain verifies with the phase 02 chain verifier after the run, and every event of step 5 r is in it.
8. Read-only conformance (phase 05): the CTR and the validation paths issue no statement other than `SELECT` against the canonical copy; the test asserts the statement list of `ctr_daily` and `report_validate` against the allow-list.
9. Existing tests still pass, with two expected changes: the phase 18 close test asserts the acknowledged-filing gate (steps i and j), and the phase 20 letter stub test is replaced by the `LetterService` test.
10. Migration guard fixtures as `akr_app`: an `UPDATE rpt.report_versions` on a frozen version fails with `a frozen report version is immutable`; an `UPDATE rpt.filings SET goaml_reference` fails with permission denied (`goaml_reference` is not in the column grant) and, as the owner `akr_migrate`, with `the filing record is written once`; a second `ack_at` fails with `an acknowledgement is written once`; an `UPDATE rpt.letters SET state = 'requested'` on a signed letter fails with `a letter state never moves back`; a `DELETE FROM rpt.fiu_feedback` fails with permission denied (no DELETE grant) and, as the owner `akr_migrate`, with `fiu_feedback is append only` (the guard prints `TG_TABLE_NAME`, which has no schema name).

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 3a: corrections found while checking phases 05 to 10 (tools/agent/gap1-run3a-verdict.md); lead-verified.

### 7.1 Rule codes enter phase 07 in a release (DR-368)
Section 3.13 step 12 and section 4.5 item 6: the CTR codes, like the other new codes, enter phase 07's catalogue in a
release (the catalogue is closed in code), not by change request. `CONS-04` is not new: phase 07 already has it
(info, chronological order); the account rule of section 3.4 step 7 is `CONS-05` (warn). CTR-01 and CTR-02 are block, CTR-03 warn (phase 07 section 7+).

Gap pass 1 (2026-10-10), run 4a, from tools/agent/gap1-run4a.md; lead-verified (tools/agent/gap1-run4a-verdict.md).

### 7.2 Draft update method
`PATCH /reports/{id}` is served by `ReportService::update_draft(tx, report_id, UpdateReportDraft, actor, if_match)`
under the rules of this phase (draft only, the narrative limit of section 3.2, `validated` back to `draft` clears the
validation, section 3.5). Phase 28 calls the same method for an accepted narrative.

Gap pass 1 (2026-10-10), run 5b (coverage of SP and K IDs), from tools/agent/gap1-run5b.md; lead-verified (tools/agent/gap1-run5b-verdict.md).

### 7.3 PEP KPI items in the CBI monthly pack (K-23, DR-395)
No new pack kind: `cbi_monthly` (section 3.16) gains a PEP section with the items of the CBI circular item 9 that this
system holds (research 26, the CBI KPI report), counts for the month under the small-number rule of the pack:
- a. PEPs by category: confirmed `kyc.pep_register` entries by `tier`, current and former (`kyc.party_pep_state`);
- b. found by screening versus self-disclosed: confirmed entries by `source` (`pep_list` against `declaration`, with
  `cbs_class` and `manual` shown apart);
- c. relationships needing senior approval and their outcome: completed `kyc_edd_signoff` instances for parties with a
  confirmed entry, by outcome, and `kyc.exit_register` entries for those parties;
- d. data updates and re-assessments: `kyc.kyc_reviews` completed for those parties, by `kind`;
- e. STRs on PEPs and associates: `casework.decisions` of type `str` whose case (`casework.case_parties`) holds a party
  with a confirmed entry or a confirmed `kyc.pep_relatives` row.
Items f (internal-audit test results and remediation) and g (share of staff trained) are not held by AKR-TMS; the pack
cover says so, as for the RP-68 reports. Test: a fixture month with two PEPs (one by list, one declared) and one STR
gives the expected section.

Adversarial pass (2026-10-10), run 1 (four-eyes and self-approval), from tools/agent/adv-run1.md; lead-verified (tools/agent/adv-run1-verdict.md).

### 7.4 The drafter and the authors never sign (ADV-1-02, HIGH; DR-407)
Section 3.6 step 3 excludes the instance proposer, the submitter and the earlier deciders. The report's drafter
(`rpt.reports.created_by`) and the author of any version (`rpt.report_versions.created_by`) are also makers, so:
1. The `sign` task also refuses the report's `created_by` and the `created_by` of every version of the
   report (403 `E_SELF_APPROVAL`). The workflow subject carries this rule, not the engine's excluded set (section 7.12).
2. Cedar (section 3.22): `@id("report.sign_not_drafter") @deny("self_approval") forbid (principal, action ==
   Akr::Action::"report.sign", resource is Akr::Report) when { resource has created_by && resource.created_by ==
   principal };` (the entity already carries `created_by`).
3. Migration `0068` adds to `rpt.reports`: `CHECK (signed_by IS NULL OR created_by IS NULL OR signed_by <>
   created_by)`, next to the submitter CHECK.
A bank with one compliance officer has an A2 prepare the draft. Test: M1 creates the draft, an A2 edits and submits it,
M1's sign is 403 `E_SELF_APPROVAL`.

Adversarial pass (2026-10-10), run 4 (clocks and timers on closed, merged or reopened items), from tools/agent/adv-run4.md; lead-verified (tools/agent/adv-run4-verdict.md).

### 7.5 Freeze letters and the freeze timers (Open doubts 4; ADV-4-09; DR-410)
Phase 20's `committee_report_at` means requested, and the phase 20 deadline handler raises `report_missed` when the
letter is not `delivered` by `committee_report_due_at` (phase 20 section 7+). The handler reads the letter state through `LetterRequester::delivered`, which this phase implements (section
4.5 item 9); no phase 20 method records the delivery. Open doubts 4 is closed.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.6 Links are written at submit (V-21-01-a, MEDIUM; DR-413)
No step wrote `rpt.report_links`. Section 3.5 step 3, in the same transaction as the freeze: for each `earlier_reports` entry of the frozen content whose `goaml_ref` equals the `goaml_reference` of a filing of another report of this install (`FilingRepository::by_reference(conn, goaml_reference: &str) -> Result<Option<Filing>, RepoError>`, new), `ReportRepository::insert_link` writes one row from this report to that report; a row that `uq_report_links` already holds is skipped. The relation is `escalated_from_utr` when this report is an STR or a SAR and the related report is a UTR, and `previous` otherwise. Each new row is audited `report.linked {report_id, related_report_id, relation}` (section 3.20). An entry with no matching filing stays in the content only. The table is append only, so a link written for an earlier version stays. No act of this phase writes `amending` or `superseding` (Open doubts 6 and 12).

### 7.7 Row types of section 2 (V-21-01-b, V-21-02-a, V-21-06-b, V-21-10-b, LOW; DR-413)
Section 2 gains the fields of the rows its traits name.
- `ReportLink { id, report_id, related_report_id, relation: ReportLinkRelation, created_by: Option<Uuid>, created_at }` and `NewReportLink { id, report_id, related_report_id, relation: ReportLinkRelation, created_by: Option<Uuid> }`.
- `RelatedRef { goaml_ref: String, date: NaiveDate, type: ReportType }`, the fields of the phase 18 `EarlierReport`.
- `SealedContent { content_enc: Vec<u8>, content_sha256: [u8; 32] }` (sealed with AAD = the version id; the hash is of the plaintext canonical JSON) and `NewReportVersion { id, report_id, version: i32, content: SealedContent, created_by: Option<Uuid> }`.
- `ReportVersionSummary { version: i32, created_by: Option<Uuid>, created_at, validated_at: Option<DateTime<Utc>>, frozen_at: Option<DateTime<Utc>> }`, the row of `summaries` and of `GET /reports/{id}/versions`.
- `NewFiling { id, report_id, report_version: i32, package_id, goaml_reference: GoamlReference, filed_by, filed_at, portal_capture_evidence_id, activity_age_months: Option<i32> }`.
- `NewPeriodic { id, kind: PeriodicKind, period: PeriodKey, period_start: NaiveDate, period_end: NaiveDate, template_version: String, generated_by: Uuid }` and `PeriodicFilter { kind: Option<PeriodicKind>, period: Option<PeriodKey> }`.
- Every other `New*` row of section 2 (`NewReport`, `NewReportFinding`, `NewFeedback`, `NewReportTask`, `NewCtrBatch`, `NewCtrItem`, `NewLetter`) carries the columns of its section 1 table that the insert sets: every column without a default, and each nullable column that the step of section 3 names.

### 7.8 The approval binds to the frozen content hash (V-21-03-b, MEDIUM; DR-413)
RP-34 asks that the approval be bound to the manifest hash. The manifest holds the hashes of the XML and the attachments and is built after the sign (section 3.8 step 4; the PDF of step 5 prints the manifest hash, so the manifest cannot hold the PDF hash), so the sign binds to the frozen version instead: its `content_sha256` cannot change once the version is frozen (trigger of section 1.3). Both `report.signed` events of section 3.7 (the release and the correction) also carry `content_sha256` of the signed version. The package job checks the content against that hash before it builds (section 3.8 step 1), and the manifest names `report_version` and `validation_run_id`.

### 7.9 Generate: one at a time, and the supersede is audited (V-21-05-c, V-21-05-d, MEDIUM; DR-413)
Section 3.16 step 1: the transaction of `POST /reports/periodic` takes `pg_advisory_xact_lock(hashtext('periodic:' || kind || ':' || period))` before it reads the live pack, so two requests for the same kind and period run one after the other and the second supersedes the draft of the first. The lock is taken before any row of `rpt.periodic_reports`. A live draft that is replaced writes `periodic.superseded {periodic_id, kind, period, superseded_by}` in the same transaction (new; section 3.20 gains the row, resource periodic pack). The `periodic_build` job of a pack that is no longer `draft` completes and writes nothing.

### 7.10 Periodic service methods (V-21-05-e, MEDIUM; DR-413)
Section 2, services: `PeriodicReportService::generate(&self, tx: &mut Tx, principal: &Principal, req: &GeneratePeriodic) -> Result<PeriodicReport, ReportError>` (section 3.16 step 1), `PeriodicReportService::sign(&self, tx: &mut Tx, principal: &Principal, id: Uuid, if_match: i32, stepup_id: Uuid) -> Result<PeriodicReport, ReportError>` (step 3; the handler consumes the step-up of `periodic.sign` first) and `PeriodicReportService::build(&self, pool: &PgPool, periodic_id: Uuid) -> Result<(), ReportError>` (step 2, the body of the job `periodic_build`).

### 7.11 Draft edits are audited (V-21-08-c, MEDIUM; DR-413)
Section 3.2 and 7.2: each `PATCH /reports/{id}` writes `report.edited {report_id, version, field_keys}` in the transaction of the write (new; section 3.20 gains the row, resource report). `field_keys` lists the builder field keys the PATCH wrote and never a value: `suspicion_formed_on` appears as its key only.

### 7.12 The drafter, the authors and the submitter are refused by the subject check (V-21-10-a, MEDIUM; DR-413)
Section 3.6 step 3 and 7.4 item 1 name people the `sign` task excludes, but the phase 04 engine computes `excluded` from the instance maker and the earlier deciders only. The rule is therefore carried by the subject: `CaseWorkflowSubject::check_before_decision` (section 4.5 item 1), for an instance of `case_report_approval`, and `ReportWorkflowSubject::check_before_decision` (section 3.7), refuse the sign with 403 `E_SELF_APPROVAL` when the actor, or the user the actor acts for, is the report's `created_by`, the `created_by` of any version (`ReportVersionRepository::summaries`) or the report's `submitted_by`. The Cedar forbid of 7.4 item 2 and the two database CHECKs stay.

### 7.13 Foreign keys on the letter id columns of phase 20 (V-21-12-b, MEDIUM; DR-413)
Phase 20 leaves its letter id columns without a foreign key until this phase. Migration `0070` (section 1.3) adds four, each as `ALTER TABLE <table> DROP CONSTRAINT IF EXISTS <name>;` followed by `ALTER TABLE <table> ADD CONSTRAINT <name> FOREIGN KEY (<column>) REFERENCES rpt.letters (id) ON DELETE RESTRICT NOT VALID;`: `fk_freezes_committee_letter` on `casework.freezes (committee_report_letter_id)`, `fk_freezes_second_letter` on `casework.freezes (second_report_letter_id)`, `fk_freezes_referral_letter` on `casework.freezes (referral_letter_id)` and `fk_refrain_referral_letter` on `casework.refrain_instructions (referral_letter_id)`. `NOT VALID` is kept, because the stub ids of phase 20 have no letter row until the backfill of section 3.15 runs; the constraint still checks every row written from then on. The backfill ends by counting the ids in these four columns that have no `rpt.letters` row, and it fails when the count is not zero.

### 7.14 CTR-03 bounds and the account rule code (V-21-07-a, V-21-07-b, MEDIUM; DR-413)
Lead decision. CTR-03 had no bounds: it takes two rule parameters in the phase 07 catalogue (phase 07 section 7+), `max_usd` (shipped 1000000) and `round_unit_iqd` (shipped 250, the smallest note; every other currency must be a whole amount). They are shipped defaults of the profile, and a bank changes them by a `goaml_profile` change request, as for every rule parameter. The RP-30 account rule had the code of another rule: it is `CONS-05` (warn), and `CONS-04` stays phase 07's chronological-order rule (info). DR-171 named `CONS-04` for the account rule; this block replaces that name.

## Decisions

Numbered from DR-169 (the last number in the phase 20 decisions is DR-168). Each decision is one paragraph with its rationale.

**DR-169 (the draft starts from the running proposal; the sign is the decision).** A report is drafted from the running proposal of its case, before any decision row exists, and the MLRO's sign act of phase 18's `case_report_approval` is the decision. Drafting after the decision would make the MLRO sign a document nobody had built. The draft is pre-filled from the proposal's own columns and the canonical copy; the decision row is written at sign by phase 18 unchanged, and `release_for_filing` (the phase 18 kind, body replaced here) marks the report `signed` and the frozen version as the one signed. One report per proposal keeps the approval and the report in step; corrections are versions of the same report, and a correction after an Office rejection has its own approval (DR-175).

**DR-170 (the report sign is the workflow act; its step-up is `signing`).** The route `POST /reports/{id}/sign` acts on the open `sign` task of the instance through phase 04's `WorkflowEngine::act`, so that eligibility, four-eyes and the step-up are the engine's and the inbox agrees with the route. The step-up class is `signing` (the brief, and DSL E.4's "sign always steps up"), where phase 18 section 3.8 c declared `decision`; a report signature is a signature of a filing, and phase 01's closed list has the class for it. The phase 18 route and the generic task route refuse a report sign task unless the report route acts, because the report's checks (frozen version, no block, the case not sealed) must run on the same act.

**DR-171 (validation reuses the phase 07 engine; L0 is the only new layer).** The report validator is phase 07's pipeline run on the XML built from the frozen content, with the same XSD sandbox, rule catalogue, scorer and consistency checks, so that a file checked on upload and a report checked before filing cannot disagree. Phase 07 numbers the layers L1 to L5, and this phase adds L0 (the canonical completeness and the case snapshot, which phase 07 defers, as its section 3 says). New rule codes (`CONS-05`, `NAME-01`, `DATE-01`, `ID-01`, `PKG-07`, `CTR-01` to `CTR-03`) enter the catalogue in a release, because the catalogue is closed in code.

**DR-172 (frozen versions and packages are sealed, hashed and not watermarked).** A submitted version is frozen by the database and never changes; its content is sealed under the version id so that a copied row cannot be read. The package is a ZIP with a SHA-256 manifest and a bilingual PDF, both sealed through phase 02's `put_generated`, and the download is step-up `export`, purpose-bound and audited. A per-user watermark would change the bytes the manifest hashes, so the choice is to log the access and leave the file as it was filed (Open doubts 14).

**DR-173 (filing is a named officer's act; the close gate reads the acknowledgement).** Only the MLRO files (`filing.record` for role M), because the goAML upload is an act of the named officer (RP-39), and the portal capture is taken at the same moment (RP-79). The close of a UTR, STR or TFR now requires the Office's acknowledgement of the report, not a chain event: the chain event only records the signature, and a case cannot close on a file that was never accepted. This replaces the phase 18 check with a record, as phase 18's Open doubts 10 asked.

**DR-174 (CTR: the CBI threshold is fixed, the rate is the CBI rate of the day, the day waits for a rate).** The USD 10,000 threshold is a CBI figure the bank cannot move, so it is a code constant (`CTR_THRESHOLD_USD`), not a setting and not a registry entry (the DSL pins it the same way). Cash is added up per customer per business day by default (research Q9, RP-51), with a setting that turns it off. The test uses the CBI rate of the business date; a day with a missing rate waits and raises a task, and nothing falls back to the rate stored on the transaction at its own date. The batch is one per business date with run numbers, a second person confirms the control totals (RP-52), and no GL tie-out is claimed because the canonical copy has no general ledger (Open doubts 3).

**DR-175 (a correction is a new version under a new approval; it is not a new decision).** After an Office rejection or a poor grade, the corrected report is a new version, signed under the `report_correction` definition (published by a `cr_workflow` change request, section 4.5 item 5) with the effect `release_correction`. A second `case_report_approval` could not be started, because phase 18 allows one decision per proposal and the decision row is the record of the case. The correction runs on the resource kind `report` (`ReportWorkflowSubject`), because the case is often closed and sealed by the time the Office grades or rejects the file. The correction changes no case tier and writes no chain event, and each filing of a correction gets its own goAML reference (RP-42).

**DR-176 (letters are requested in the owner's transaction, generated from the record, signed by M, and delivered on record).** The port that phase 20 defined is implemented in the same transaction as the owner's change, so that a letter exists if and only if the owner's request exists. Its text is rendered from the owner row by a job, and its facts are ids, codes and dates, so the letter can be regenerated and the table holds no party name. The number comes from a sequence (not gapless; Open doubts 23). The letter is signed by M with step-up and delivered by a person, who records the channel and the reference; the system sends no e-mail in this phase, since phase 22's notifications do not exist yet and RP-39 keeps the Office outside the system. The stub requests of phase 20 become letters under the ids the owners already store, so no owner row and no phase 20 method changes.

**DR-177 (periodic packs are generated from data and suppressed; the FATCA and CRS kinds wait for a source).** Each pack is built from counts this system holds, with cells below the bank's threshold (never below three) suppressed, and it is final only when signed. The CBI standard reports, which need account movements the canonical copy does not carry in the CBI's layout, are not invented: the monthly pack says what it lacks. The FATCA and CRS kinds exist in the table, and their generation fails with `no_source` until the bank names the source (phase 15 has none; Open doubts 21).

**DR-178 (report clocks are registry entries and timers; the periodic due date is read, not timed).** The filing target is the internal 24-hour escalation of RP-47 (the legal text says "immediately" and gives no number, research file 33, row 5), and the rework period is a proposal of ten working days (Open doubts 7). Both are entries of phase 09's closed registry, so they change only through its rules. A timer raises an alarm and decides nothing: a late report is still filed by the officer. The periodic due date is a calendar computation from a setting, because a timer per pack would fire for every pack of every month before the bank has confirmed the CBI dates.

**DR-179 (the feedback register writes no case event, also after closure).** An Office grade, rejection or session is recorded for the report and the register; for a sealed case the audit event is the only trace, as phase 18's sealed-case rule and phase 19's rule for work after closure require. The grade is an observation about a file that was filed; a case event would write into a sealed chain a fact that does not change the case's decision.

**DR-180 (visibility: the case access list for reports and letters; counts for A2 and identities for M only).** A report is visible to M and to the case team, and every other reader gets the masked 404 of phase 18's rule (existence masking). A CTR file has no case, so it is visible to M only, and the batch shows counts to A2 so that analysts can see that a day was run without seeing the customers. A letter with no case is visible to M only. The Cedar file and the SQL predicates of section 3.22 state the same rules.

**DR-181 (analytics are aggregates with suppression, and name the measures of phases 17 to 19).** The decision-to-filing and first-alert-to-filing distributions (M-31), the activity age share (M-41), the first-time pass rate and the rejection rule codes (RP-84), the closed-without-filing count (RP-65), the reopen rate (M-40) and the contribution outcomes (DR-138) are computed from the records this phase and phases 17 to 19 write. Every cell below the threshold is suppressed, and analyst names are shown to M and IA only.

**DR-182 (the two-signatory exception is a setting that defaults to off).** RP-36 asks for a second named signatory for a TFR and a large STR when the bank has one compliance officer. The default is off, because a bank with a deputy MLRO has no need of it; a bank with one officer turns the exception on and names the amount, and the signature then needs a second `M` on the case access list. The amount and the switch are governed settings with their basis in the migration.

**DR-183 (the reporting Cedar file is `A1_reports.cedar`).** Phase 03 reserves the range `A0` for reporting, and phase 07 already ships `A0_reporting.cedar` with the goAML checker's actions. This phase adds `A1_reports.cedar` in the same range, so phase 07's file is not edited and the bundle order keeps the reporting policies together (lead review: the lead prompt's `94` would have put reporting in the casework range).

**Lead corrections (review 2026-10-09).** The correction instance runs on the resource kind `report` with a new `ReportWorkflowSubject` (holders M at bank scope; no case lock, no case event): on the resource kind `case`, phase 18 refuses the act once the case is sealed, and the acknowledgement is what lets the case close, so a returned or rejected report could never have been corrected. The letter backfill inserts each stub letter under the id the owner already stores, so phase 20 needs no new method. `GET /liaison-reports`, `POST /liaison-reports/{id}/review` and screen R5 are phase 22's (it owns the liaison monthly reports); this phase has 32 routes and screens R1, R2, R3. The Cedar file is `A1_reports.cedar` in phase 03's reporting range (the lead prompt's `94` was the casework range). The CTR completeness gate reads phase 12 coverage certificates (phase 09 has no reconciliation). `StartSpec` carries `initiator`. `CTR_THRESHOLD_USD` is a code constant mirroring the DSL pinned `limit`, not a ClockRegistry entry. The Cedar policies use `principal.roles.contains` only. The reports retention clock column is `created_at` (a CTR file is never signed).

## Open doubts (for gap pass 1 and the bank)

1. **Six layers or seven names.** Research file 23 lists six validation layers with seven names (canonical completeness, XSD, compatibility, Iraqi business rules, quality, consistency, package). Phase 07 numbers L1 to L5 with the pre-screen inside L5, and this phase adds L0. Gap pass 1 chooses one numbering for the UI labels.
2. **The report-indicator map (RP-18).** The 62 Iraqi indicator codes and the scenario-to-indicator map are not in any phase. Until a map exists, the proposal's reason codes are the indicators. The bank or the compliance owner supplies the map, as a code map of phase 10's kind (effective-dated, change request).
3. **The GL tie-out source.** The canonical copy holds no general-ledger table (phase 10). The daily CTR batch reports the tie-out as not available until the bank names the cash general-ledger control totals and a channel for them.
4. **Freeze letters and the freeze timers.** Phase 20 sets `committee_report_at` when the letter is requested; the freeze report is sent when it is delivered. Gap pass 1 decides whether phase 20's column means "requested" or "delivered", and DR-413 settles it: `committee_report_at` means requested, and the phase 20 handler reads the delivery through `LetterRequester::delivered` (7.5).
5. **The single-officer exception and the large-STR amount.** RP-36 needs the bank's number of compliance officers and the amount above which a second signatory is needed. Both are governed settings with a default of off and zero; the bank sets them, and zero with the exception on means every STR.
6. **Statuses of RP-80 and the states of RP-34.** The phase 21 statuses are DATA-MODEL's eight. RP-34 and RP-80 name more (checker-approved, accepted with errors, cancelled, superseded, the goAML Web set); this phase does not store them. Gap pass 1 decides whether `superseded` (a report replaced by a later one through `report_links`) is needed.
7. **The rework period.** The ten working days of `report.rework` are a proposal. Research file 23 gives no period for a returned report. The bank or the compliance owner sets it within the registry bounds (5 to 20).
8. **Sending letters and e-mail.** No e-mail is sent by this phase, and a letter delivery is recorded by a person. Phase 22 owns notifications; the channel `email` records an out-of-system delivery. The Office's own channel for the freeze report (letter, controlled report, IQTFS forms or goAML) is the bank's setting (`report.letter_channel`) and the Office's answer.
9. **API.md rows added here.** Fourteen rows of the route table are new (section 2); the lead aligns API.md 3.13 in the same change. The API.md update is part of gap pass 1 with the scopes of section 2 of this phase; `POST /reports/{id}/attachments` and `GET /report-tasks` are new as well.
10. **Year-only birth dates and the father's name element.** The goAML XSD has no precision flag for a birth date, and the research gives no Office convention for a year-only date; this phase emits `YYYY-01-01` with the UNKNOWN convention and a warning. The element name for the father's name is read from the XSD in phase 07's profile, which the lead confirms before the mapper is built.
11. **The MLRO annual report.** Research file 23 says there is no set format for the MLRO annual report (RP-68). The pack provides the facts and a template the bank edits; the bank supplies its own template text and sign-off.
12. **A correction or a new report.** The design makes a correction a new version of the same report. The Office may ask for a new report with a link to the old one (`amending`); the choice is the Office's and is decided case by case.
13. **Purpose codes for the reporting exports.** The phase 03 purpose list does not name the reporting purposes. The package download uses a code of the list; the lead adds the reporting codes (for example `filing` and `review`) as admin data.
14. **Watermark of the package.** Section 3.9 serves the file as stored. A per-user watermark is a choice for the bank's legal owner; it would change the hashed bytes and so the manifest.
15. **The activity-age threshold N (M-41).** The research gives the share of reports older than N months but no N. The shipped value is 12 and is governed.
16. **R2 and R3 detail.** The DESIGN-SYSTEM has no specification for R2 and R3 beyond the catalogue. The liaison review (`GET /liaison-reports`, `POST /liaison-reports/{id}/review`, screen R5) is phase 22's; the periodic `liaison` kind reads phase 19's `internal_reports` (channel `liaison_monthly`) until phase 22's table exists. Whether an authority reply letter (`office_reply`) moves phase 19's authority request status is phase 19's decision; this phase does not change it.
17. **Phase 20's stub audit payload.** `ops.letter_recorded` does not carry the id the stub returned, so the backfill of section 3.15 creates new letters for the stub requests and replaces the owners' ids. The stub is removed by this change, so the gap pass records the decision and not a fix.
18. **The CBI standard and monthly reports.** The three CBI standard reports of RP-68 (account movements including staff accounts, the transfers register and correspondent account activity) need account-level data in the CBI's layout. The bank and the CBI confirm the layouts; until then the monthly pack says what it lacks, and the due-date setting (`report.periodic_due_days`) is the bank's number.
19. **Traceability (RP-64).** The trace from each report figure to the evidence snapshot rows is not built here. The D2 trace view belongs to the assessor workspace (phase 24, G8).
20. **Timeliness explanation (RP-31).** The mandatory explanation for an interval over the threshold is not stored by this phase's filing record; a late filing is counted in the analytics, and whether to add an explanation column is the bank's decision.
21. **FATCA and CRS sources.** The canonical copy has no FATCA or CRS data (phase 15 has none). The kinds exist and fail with `no_source`. The bank confirms the source, or the decision not to report these kinds, before the first run.
22. **The Cedar range.** Resolved in the lead review: `A1_reports.cedar`, in phase 03's reporting range `A0`, after phase 07's `A0_reporting.cedar`.
23. **Numbering and capture gaps.** The letter number sequence is not gapless (a rolled-back request burns a number). If the bank's rule requires gapless numbers, the number is assigned at signature from a locked counter instead. Separately, a portal capture that is still `pending` at filing is accepted (section 3.10 step 3); the bank confirms that the capture is evidence of the act even while the scan is pending.
24. **The CTR file limit and the Office's rules.** The number of transactions per file (`ctr.file_max_transactions`, default 2,000) is research file 23's estimate; the Office's limit is not verified there. The replacement semantics of a corrected CTR file are also not verified for Iraq: this phase issues a new file, and marks it as a correction only when the bank confirms the Office's rule.
25. **Purge windows of the portal (RP-79).** Some portals purge reports and drafts after a period (research file 23 quotes another FIU's windows). The Iraqi portal's windows are unknown, so the module raises no warning; the portal captures are the protection, and the windows become a profile field when the Office gives them.
26. **M-27 is not phase 21's.** The brief said M-27 was assigned to phase 21 by phases 17 and 18. Phase 18 section 3.7 carries M-27 (proposals) and does not defer it, and phase 17 lists M-30, M-31 and M-41 for phase 21 only. This phase does not claim M-27; the gap pass confirms the owner.
27. **No TFR from a true match.** Phase 14 (the screening decision) creates a freeze and no TFR, and research file 23 (Q7) leaves open whether a TFR is expected for a true match. A TFR therefore exists only as a case outcome of phase 18. A rule that creates one automatically is not part of this phase.
28. **Lapsed case grants.** Phase 18's `case_acl` has no expiry column, and the report predicates treat a revoked grant as the only ending. If phase 19 or the bank adds an expiry, the predicate of section 3.22 changes with it.
29. **The reopen event name.** The reopen measure of section 3.17 counts phase 18's `case.reopened` chain event (phase 18 section 3.13). The gap pass confirms that the measure counts that event and not `case.reopen_late`.
30. **Aggregation across branches.** The CTR aggregation groups by the customer identifier. A customer with accounts in two branches is one customer; the bank confirms that the CTR rule counts across branches, since research Q9 does not say.
31. **The CTR run time.** The schedule `schedule.ctr_daily` (19:30 bank time, Sunday to Thursday) is a placeholder until the bank's end-of-day time is known. The completeness gate of section 3.13 step 2 makes an early run wait, so the schedule is safe but may be late.

## What This Phase Does NOT Touch

- No CBS write, no CBS action and no core-system connection: the canonical copy is read only (phases 05 and 10). No statement other than `SELECT` reaches the source from this phase's jobs.
- No change to phase 07's tables, to its `A0_reporting.cedar` file, to its upload path or to its goAML profile content, beyond the report clocks of section 4.5 item 7. No second validator is written: the report validator is phase 07's engine.
- No change to phase 18's files, routes or definitions by this phase's text: the changes of section 4.5 items 1 to 5 are listed for the gap pass and are applied there. No new case event: the phase 18 `case.filing_released` keeps its name and changes its meaning.
- No change to phase 19's watch, RFI, authority or alarm tables (its `alarms` CHECKs are widened here, as phase 20 did, with the kinds of section 1.3); the reply letter does not change an authority request's status.
- No change to phase 20's freeze, refrain or ops-task tables or routes; this phase only implements the phase 20 port `LetterRequester` (`request` in section 3.15, `delivered` in section 4.5 item 9). The freeze clocks are phase 20's.
- No e-mail, no SMS, no notification and no outbox topic (phase 22 notifies; the outbox carries ids only through `alarm.raised`).
- No goAML credentials, no portal automation and no upload to the Office by the system (RP-39).
- No retention sweep, no legal hold and no deletion certificate (phase 23); the classes R6 and R7 are assigned here.
- No AI drafting, no AI narrative and no model score in any report (P2, phases 27 and 28; RP-23).
- No regime packs other than the Iraqi profile (P3; R-01 to R-61 are not carried).
- No change to the reason-code families, the scopes of API section 2, the step-up classes or the roles of phase 01 (the classes `signing`, `filing`, `export` and `approval` are used as they are).
- No change to the authority matrix document, the policy pack structure or the calendars (phase 09); the registry entries and the fixed value of section 3.23 are the only additions.
- No canonical row is written, derived or back-filled into the copy; the mapper and the CTR job only read it.
