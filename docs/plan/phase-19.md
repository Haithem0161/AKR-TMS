# Phase 19: RFIs, Watches, Network & QA

**Goal:** The case team asks for information and gets claims back: RFIs (six kinds, templates from the pinned policy pack, deadline clocks on the bank calendar, release only after a second signature under step-up and four-eyes, answers stored as claims that expire and are corroborated); the registers of internal reports and authority requests (clocks, reporter identity for the MLRO only, and the trigger that re-rates the linked customers, TR-17); UTR watches with closed triggers and an expiry review; hibernations that follow a no-report decision, suppress a scenario's display for one party inside its envelope and keep every suppressed hit in a ledger with a regret metric; the network view (W6) built only from evidenced canonical links, with kinship hidden by default and logged, snapshots kept as evidence, and network cases with one child per visible subject; QA sampling (stratified, seeded and reproducible, never the reviewer's own cases, MLRO decisions sampled by the compliance head or internal audit), scored reviews with findings, calibration, reviewer rates and a step-up bulk reopen by the MLRO after a critical finding; the breach actions of the clocks (50, 100 and 150 percent) with the supervisor alarm register (M-53) that phases 17 and 18 deferred. Screens W6, W8, W10, W11 and G3.

**Dependencies:** Phase 00 (jobs, outbox, timers and the timer kind registry, settings, cron schedules), Phase 01 (`StepUpService` and its closed action classes, `admin.role_grants`, `admin.users`), Phase 02 (`EvidenceStore::stage`, `open`, `put_generated`; the evidence owner registry; `casework.append_only()` of phase 17), Phase 03 (Cedar bundle and schema, `MASKED_KINDS`, purpose codes `qa_review`, `investigation`, `authority_request`, analyst networks), Phase 04 (`WorkflowEngine::start` and `act`, `WorkflowSubject`, `TerminalEffect`, `Holders` and `NamedOfficers`, `ChangeTargetRegistry`), Phase 09 (`BusinessClock::add`, `BankCalendarClock`, `PolicyPackService::scan` with scope `rfi`, the policy-pack clock `rfi.response` and switch `rfi.pre_fill`, `AuthorityMatrix::permits` with decision type `rfi.release`, reason-code families `closure` and `decision`, `ref.reason_codes`), Phase 10 (canonical party, contact, relationship, account-party role, payment link and transaction tables), Phase 11 (outbox topic `canon.touched`), Phase 14 (screening hits, pair-memory suppressions), Phase 15 (rating consumers of `authority_request.recorded` and `internal_report.recorded`, the `customer` masked kind, DR-103), Phase 16 (scenarios `scn.scenarios`, the linked-party resolver, detection counts per scenario), Phase 17 (`casework.cases` with `parent_case_id`, `casework.case_events` and `CaseChain`, `casework.case_families`, `casework.effective_tier`, `AlertIntakeService`, `AssignmentService`, `QueueService`, the visibility predicate, `casework.reopen_window_days`, `casework.touch_by_hours`, `casework.wip_limit`), Phase 18 (`casework.case_acl` (its reason list gains `qa_review`), `CaseService::close`, `CaseService::reopen`, `casework.decisions`, `casework.evidence_items`, `casework.work_items`, `WorkItemService`, `RecordDecisionEffect`, `CaseWorkflowSubject`, `ShelvedRegister`, `StepUpAction` `party.open` and `case.decide`, lock order of section 3.16)
**Macro-phase:** P1
**Complexity:** XL (32 product routes: 19 rows of API.md 3.12 owned here and 13 added rows; the RFI release workflow, the hibernation gate inside alert intake, the network edge builder and the QA draw with its independence rules carry the weight)
**Binaries touched:** akr-api (routers `rfis`, `watches`, `hibernations`, `authority_requests`, `internal_reports`, `qa`, `alarms`, the `parties` router of phase 18 extended by the graph and network-case routes; the RFI release and the watch and hibernation writes run in the request transaction; evidence of network snapshots through `EvidenceStore::put_generated` in akr-worker only, so the API writes the snapshot row through a job, section 4.1) | akr-worker (timers `rfi_clock`, `breach_stage`, `watch_expiry`, `hibernation_expiry`, `qa_grant_expiry`; scheduled jobs of section 4.1; outbox consumers `screening.hit`, `authority_request.recorded`, `canon.touched`, `alert.created`) | akr-ingestd and akr-assess unchanged
**Requirements carried:**
- Implemented here: M-10 (network cases: a parent with a subgraph snapshot and one child per visible subject; the parent is closed by a person once its children have outcomes, Decisions DR-154), M-11 (flat-line detector only: a scenario whose daily count is unchanged for `qa.flat_line_days` business days raises an alarm; the other data-latency clocks are phase 17), M-16 (the daily forced sample of low-score alerts, `qa.forced_low_score_daily`), M-22 (breach actions at 50, 100 and 150 percent of the touch window for alerts and of the due time for work items, section 3.10), M-35 (watch registry with envelope, triggers, expiry and the review at expiry), M-37 (authority-response workflow: register entry, draft from system facts, MLRO approval, disclosure-register fields), M-39 (hibernation only inside a case by a person, default 90 and maximum 180 days, stored envelope, hit ledger and regret metric, the scenario keeps running and logging, MLRO-only for suspicion-tier parties), M-40 (bulk reopen by the MLRO after a critical QA finding; reopen-rate reports are phase 21), M-42 (RFI object with six kinds and versioned templates, free text scanned for forbidden words), M-43 (branch RFIs to the liaison officer only and escalated to the MLRO desk; relationship managers only when the flag allows), M-44 (correspondent RFI desk: log, the pre-fill switch `rfi.pre_fill` only (the pre-fill itself is P2, section 7+), deadline clock, four-eyes release, tipping-off flag, missed-by-monitoring signal), M-45 (answers as claims with source, corroboration and expiry; the P0 and P1 closure gate of section 3.3), M-46 (RFI KPIs in the `kpis` block of `GET /rfis`), M-47 (sampling service: strata, random and targeted modes, sample-size calculator, immutable sample register), M-48 (QA workspace with the rubric of research 28 section 10.3, findings, coaching items, reviewer rates with Wilson intervals, bulk reopen on a critical finding), M-49 (QA independence: reviewer differs from the closer, MLRO decisions sampled by CH or IA, time-boxed per-case read grants with logging), M-50 (calibration kit: golden items and agreement; the gating of queue access by training is Open doubts 14), M-53 (alarm service: touch and work-item breach, conflict declared, chain broken, flat line, critical QA finding; supervisor task with notified and acknowledged times), M-67 (MLRO and CISO alerts of a declared conflict; the conflict check itself is phase 17), T-21 (entity graph: typed edges from canonical links, link weights, the supernode rule of section 3.7)
- Partly implemented (the rest is in Open doubts with its target phase): M-51 (feedback: coaching items and the critical re-review are here; the training register and the AI golden set are phase 24 and P2), M-54 (surge mode: Open doubts 10), M-55 (volume-change preview of a rule change: Open doubts 10), M-76 (simulation harness: Open doubts 10), T-22 (motif engine: Open doubts 10), M-52 (capacity workbench: Open doubts 10), M-53 (ageing, growth, latency, silence and capacity alarms: Open doubts 10; the missing-liaison-report alarm needs phase 22, Open doubts 11)
- Phase 18 open doubts closed here: 11 (breach actions and supervisor tasks for work items, section 3.10), 12 (the network edge table `casework.network_edges`, section 3.7). Phase 17 open doubts closed here: 16 (MLRO and CISO alerts of a declared conflict, section 3.10), 19 (flat-line detector and forced sample, section 3.8), 21 (breach actions and alarms, section 3.10), 22 (chain-broken alarm, section 3.10).
- Route IDs: API.md 3.12 rows of this phase (19 owned rows, section 2) and 13 added rows marked "phase 19, added" in API.md section 3.12 by this phase
- Gap pass 1 (coverage), partly implemented: E-01 (the authority-request register with its deadline), S-34 (shared-contact edges), TS-18 (the network edge table); L-19 (the monthly closed-alert QA sample), L-46 (authority requests and internal reports linked, with the re-rating trigger and overdue notices), L-48 (shared phone and address links as network edges), L-53 (reviewer rates and bulk reopen), L-62 (the authority-request register and its links); T-12 (QA samples of closures and MLRO decisions with calibration against a golden outcome), T-23 (typed network edges for shared contacts, devices, transfers, ownership and kinship); UX-70 (the QA workspace), W-03 (network edges for shared contacts, ownership and kinship), W-23 (the RFI desk with deadline, KPIs, templates and four-eyes release); VA-08 (hibernation only inside a case by a person, with trigger keys and expiry); VB-02 (canonical network edges), VB-35 (the forced low-score sample with an independent reviewer), VB-44 (RFI kinds with four-eyes release); VC-74 (shared contact, shared device and kinship edges), VC-75 (the shared-device edge).
- Gap pass 1 (coverage), implemented here: K-12 (the authority-request register and its re-rating trigger); T-24 (kinship edges hidden unless asked for and logged when shown), T-38 (hibernations tied to a four-eyes no-report decision with envelope and expiry, the hit ledger, QA sampling of hits and the regret flag; evaluation never skipped); UX-10 and UX-61 (the network view with grouping above about 15 accounts, the node and link table, masked numbers and keyboard traversal).
- Gap pass 1 (coverage), not carried: VB-12 (link-graph motif engine), VB-13 (shared fraud and AML mule layer); DR-403.

---

## 1. SQLx Migration Changes

Two migrations, `0064_casework_followup_records` and `0065_casework_followup_guards_grants`, numbered after phase 18's `0063` (the roadmap chain has no other user of 0064 or 0065; `grep` of docs/plan confirms). The schema `casework` exists (phase 00); no new schema. Fourteen tables are created here: `casework.authority_requests`, `casework.rfis`, `casework.rfi_questions`, `casework.rfi_answers`, `casework.internal_reports`, `casework.watches`, `casework.hibernations`, `casework.hibernation_hits`, `casework.alarms`, `casework.network_snapshots`, `casework.network_edges`, `casework.qa_samples`, `casework.qa_reviews`, `casework.qa_findings`. One row is added to the closed case-family list (`network`, the parent of a network case). One phase 18 constraint is widened: the closed `reason` list of `casework.case_acl` gains `qa_review`, the time-boxed read grant of a QA reviewer (section 3.8; lead review). No other existing table is altered: the phase 17 and phase 18 tables are read and written through the grants of section 1.2, and the extensions of their services are section 4.5.

**Reconciliation with DATA-MODEL section 12 (aligned in the same change, DR-157):**
- `casework.rfis / rfi_questions / rfi_answers`: the DATA-MODEL columns, plus `direction` (the correspondent kind has an in and an out direction), `contact_allowed` (the tipping-off flag), `authority_request_id`, `recipient_*`, `clock_unit` and `clock_days`, `extensions`, `workflow_instance_id` (the release instance, one per RFI), `proposer_id`, `released_by`. The `template` is not a table: the text of a template is in the pinned policy pack (phase 09 section 3.5), so the RFI stores `template_key` and `template_version`.
- `casework.internal_reports / authority_requests`: two tables, as DATA-MODEL lists them, with the register fields (`received_at`, `acknowledged_at`, `deadline_at`, `tier`). `authority_requests` also holds `deadline_source`, `subject_party_ids`, `linked_case_ids`, the sealed questions and the disclosure fields (`disclosed_at`, `disclosed_items`), which are the disclosure-register entry of M-37.
- `casework.watches`: the DATA-MODEL columns, plus `origin_decision_id` (one watch per UTR decision), `kind`, `triggers`, `period_days`, `review_due_at` and the review fields.
- `casework.hibernations`: the DATA-MODEL columns, plus `decision_id` (the no-report decision it follows), `period_days`, the end fields and `trigger_key`.
- **New, not in DATA-MODEL (DR-152, DR-153, DR-156):** `casework.hibernation_hits` (one row per suppressed alert: the ledger that the regret metric and the daily count read; the alert stays in `alert.alerts`, so the hit table is the only record of the suppression) and `casework.alarms` (the M-53 register: each alarm is also the supervisor-notification task, with the times at which the supervisor was informed and acknowledged it). Both are needed by requirements that DATA-MODEL section 12 does not list.
- `casework.network_snapshots`: DATA-MODEL's `object_key` is replaced by `object_id`, the phase 02 evidence object, as phase 18 does for `evidence_items` (the bytes, their hash and their encryption are the evidence store's).
- `casework.network_edges`: the DATA-MODEL columns (DR-145), plus `basis` (the canonical row references that back the edge), `retired_at` and `built_batch_id`. `shared_device` is a kind of the closed list with no canonical source in phase 10, so no edge of that kind is built (Open doubts 7).
- `casework.qa_samples / qa_reviews / qa_findings`: DATA-MODEL's `stratum`, `draw_seed`, `reviewer_id`, `rubric`, `findings` are columns of these three tables: one sample row per drawn item (the register of M-47), one review per sample, one finding per criterion that failed. The grant of a sampled case is a row of `casework.case_acl` (phase 18) with reason `qa_review`, so the QA read is the access list and needs no table of its own.
- Roadmap row 19 lists eleven of these tables and omits `rfi_questions`, `rfi_answers` and `network_edges`; the roadmap row gains the fourteen names (section 4.5 item 15).

### 1.1 `0064_casework_followup_records.sql`

```sql
-- migration 0064_casework_followup_records.sql
-- Phase 19. The authority and internal-report registers, RFIs with their questions and claimed answers, watches,
-- hibernations and their hit ledger, alarms, network snapshots and edges, and QA samples, reviews and findings.
-- The parent family of a network case is added to the closed family list. Owner role: akr_migrate.

INSERT INTO casework.case_families (code, label, tier, tf_class) VALUES
 ('network', '{"ar": "شبكة أطراف", "en": "Network (parent of a network case)"}', 'normal', FALSE)
ON CONFLICT (code) DO NOTHING;

-- The QA reviewer's time-boxed read grant is a case_acl row with its own reason (section 3.8; lead review). Phase 18's
-- inline CHECK is replaced by a named one with the wider list.
ALTER TABLE casework.case_acl DROP CONSTRAINT IF EXISTS case_acl_reason_check;
ALTER TABLE casework.case_acl DROP CONSTRAINT IF EXISTS ck_case_acl_reason;
ALTER TABLE casework.case_acl ADD CONSTRAINT ck_case_acl_reason
    CHECK (reason IN ('assignment', 'escalation', 'second_review', 'mlro_desk', 'manual', 'qa_review'));

-- Authority requests (M-37, DR-151). The register entry is the row; the disclosure-register entry is the disclosed_* fields.
-- Suspicion tier always: a request names the investigation of its subjects. The reference is secret-str (tipping-off).
CREATE TABLE IF NOT EXISTS casework.authority_requests (
    id                UUID PRIMARY KEY,
    authority_kind    TEXT NOT NULL CHECK (authority_kind IN ('fiu', 'court', 'prosecutor', 'cbi', 'other')),
    authority_ref     TEXT NOT NULL CHECK (length(btrim(authority_ref)) BETWEEN 1 AND 80),
    received_at       TIMESTAMPTZ NOT NULL,
    acknowledged_at   TIMESTAMPTZ,
    deadline_at       TIMESTAMPTZ NOT NULL,
    deadline_source   TEXT NOT NULL CHECK (deadline_source IN ('stated', 'bank_default')),
    clock_days        INT CHECK (clock_days IS NULL OR clock_days BETWEEN 1 AND 60),
    subject_party_ids UUID[] NOT NULL CHECK (cardinality(subject_party_ids) BETWEEN 1 AND 500),   -- canonical parties, no foreign key
    linked_case_ids   UUID[] NOT NULL DEFAULT '{}' CHECK (cardinality(linked_case_ids) <= 500),
    questions_enc     BYTEA NOT NULL,                         -- the authority's questions (personal, AAD = id)
    status            TEXT NOT NULL DEFAULT 'received' CHECK (status IN ('received', 'drafting', 'approved', 'answered', 'closed')),
    response_rfi_id   UUID,                                   -- the authority_response RFI; no foreign key (cycle with casework.rfis)
    approved_by       UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    approved_at       TIMESTAMPTZ,
    disclosed_at      TIMESTAMPTZ,
    disclosed_items   JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(disclosed_items) = 'array'),   -- ids and codes only
    tier              TEXT NOT NULL DEFAULT 'suspicion' CHECK (tier = 'suspicion'),
    registered_by     UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    timer_id          UUID REFERENCES admin.timers(id) ON DELETE RESTRICT,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    version           INT NOT NULL DEFAULT 1,
    UNIQUE (authority_kind, authority_ref),
    CHECK (deadline_at >= received_at),
    CHECK (acknowledged_at IS NULL OR acknowledged_at >= received_at),
    CHECK ((approved_by IS NULL) = (approved_at IS NULL)),
    CHECK (status NOT IN ('approved', 'answered', 'closed') OR approved_at IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_authority_requests_due ON casework.authority_requests (deadline_at)
    WHERE status IN ('received', 'drafting', 'approved');
CREATE INDEX IF NOT EXISTS idx_authority_requests_subjects ON casework.authority_requests USING GIN (subject_party_ids);

-- RFIs (M-42 to M-44, DR-148, DR-149). One RFI belongs to one case. The text of a template is in the pinned policy pack;
-- the row keeps the template key and the pack version that carried it. Release is a workflow of phase 04 (section 3.2).
CREATE TABLE IF NOT EXISTS casework.rfis (
    id                    UUID PRIMARY KEY,
    case_id               UUID NOT NULL REFERENCES casework.cases(id) ON DELETE RESTRICT,
    kind                  TEXT NOT NULL CHECK (kind IN ('branch_info', 'rm_info', 'unit_info', 'customer_outreach',
                                                        'correspondent', 'authority_response')),
    direction             TEXT CHECK (direction IN ('in', 'out')),
    template_key          TEXT NOT NULL CHECK (template_key ~ '^[a-z][a-z0-9_]{2,63}$'),
    template_version      TEXT NOT NULL CHECK (length(template_version) BETWEEN 1 AND 40),
    purpose_code          TEXT NOT NULL CHECK (purpose_code ~ '^[A-Z]{2,4}-[A-Z]{2,12}$'),   -- neutral RFI purpose, never STR or UTR
    legal_basis           TEXT NOT NULL CHECK (length(btrim(legal_basis)) BETWEEN 1 AND 200),
    recipient_kind        TEXT NOT NULL CHECK (recipient_kind IN ('liaison', 'relationship_manager', 'unit', 'customer',
                                                                  'correspondent', 'authority')),
    recipient_ref         UUID,                               -- branch, staff or unit id; from phase 31 on, the canon.account id of a correspondent (canonical; no foreign key)
    recipient_name_enc    BYTEA,                              -- correspondent or unit name, sealed (personal, AAD = id)
    party_id              UUID NOT NULL,                      -- the subject (canonical party; no foreign key)
    contact_allowed       BOOLEAN NOT NULL DEFAULT FALSE,     -- the tipping-off flag: a customer or RM is asked only when TRUE
    authority_request_id  UUID REFERENCES casework.authority_requests(id) ON DELETE RESTRICT,
    priority_class        TEXT NOT NULL CHECK (priority_class IN ('P0', 'P1', 'P2', 'P3')),
    tier                  TEXT NOT NULL CHECK (tier IN ('normal', 'restricted', 'suspicion')),
    status                TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'pending_release', 'released',
                                                                      'answered', 'closed', 'withdrawn', 'rejected')),
    clock_unit            TEXT CHECK (clock_unit IN ('rfi.response', 'internal', 'outreach', 'authority_stated',
                                                     'authority_default')),
    clock_days            INT CHECK (clock_days IS NULL OR clock_days BETWEEN 1 AND 60),
    deadline_at           TIMESTAMPTZ,
    extensions            SMALLINT NOT NULL DEFAULT 0 CHECK (extensions BETWEEN 0 AND 2),
    timer_id              UUID REFERENCES admin.timers(id) ON DELETE RESTRICT,
    workflow_instance_id  UUID UNIQUE REFERENCES admin.workflow_instances(id) ON DELETE RESTRICT,
    proposer_id           UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    released_by           UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    released_at           TIMESTAMPTZ,
    answered_at           TIMESTAMPTZ,
    closed_at             TIMESTAMPTZ,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    version               INT NOT NULL DEFAULT 1,
    CHECK ((kind = 'correspondent') = (direction IS NOT NULL)),
    CHECK ((kind = 'authority_response') = (authority_request_id IS NOT NULL)),
    CHECK ((kind = 'authority_response') = (recipient_kind = 'authority')),
    CHECK (released_by IS NULL OR released_by <> proposer_id),          -- second signature, second layer under the workflow
    CHECK ((released_at IS NULL) = (released_by IS NULL)),
    CHECK (status NOT IN ('released', 'answered', 'closed') OR released_at IS NOT NULL),
    CHECK (status NOT IN ('pending_release', 'released', 'answered', 'rejected') OR workflow_instance_id IS NOT NULL),
    CHECK (status = 'draft' OR clock_unit IS NOT NULL OR status IN ('withdrawn', 'rejected')),
    CHECK (status NOT IN ('released', 'answered') OR deadline_at IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_rfis_case ON casework.rfis (case_id, created_at);
CREATE INDEX IF NOT EXISTS idx_rfis_desk ON casework.rfis (status, deadline_at) WHERE status IN ('pending_release', 'released');
CREATE INDEX IF NOT EXISTS idx_rfis_party ON casework.rfis (party_id);

-- Questions of an RFI, from the template of the pack. The question text is not stored: question_key names it.
CREATE TABLE IF NOT EXISTS casework.rfi_questions (
    id              UUID PRIMARY KEY,
    rfi_id          UUID NOT NULL REFERENCES casework.rfis(id) ON DELETE RESTRICT,
    ordinal         SMALLINT NOT NULL CHECK (ordinal BETWEEN 1 AND 60),
    question_key    TEXT NOT NULL CHECK (question_key ~ '^[a-z][a-z0-9_.]{2,80}$'),
    answer_format   TEXT NOT NULL CHECK (answer_format IN ('yes_no_unknown', 'value', 'document', 'free_text')),
    required        BOOLEAN NOT NULL DEFAULT TRUE,
    state           TEXT NOT NULL DEFAULT 'open' CHECK (state IN ('open', 'answered', 'waived')),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (rfi_id, ordinal),
    UNIQUE (rfi_id, question_key)
);

-- Answers are claims (M-45, DR-150): the value is sealed, the source and the corroboration are stated, and the claim expires.
-- A correction is a new row that names the old one in `supersedes`; the old one becomes `superseded`.
CREATE TABLE IF NOT EXISTS casework.rfi_answers (
    id                  UUID PRIMARY KEY,
    rfi_id              UUID NOT NULL REFERENCES casework.rfis(id) ON DELETE RESTRICT,
    question_id         UUID NOT NULL REFERENCES casework.rfi_questions(id) ON DELETE RESTRICT,
    answer_kind         TEXT NOT NULL CHECK (answer_kind IN ('yes', 'no', 'unknown', 'value', 'document', 'free_text')),
    value_enc           BYTEA NOT NULL,                       -- the answer (personal, AAD = id)
    source_kind         TEXT NOT NULL CHECK (source_kind IN ('branch_officer', 'relationship_manager', 'unit',
                                                             'customer_via_branch', 'correspondent', 'authority', 'replica')),
    source_ref          TEXT CHECK (source_ref IS NULL OR length(source_ref) BETWEEN 1 AND 120),
    answered_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- the desk person who recorded it
    answered_at         TIMESTAMPTZ NOT NULL,
    corroboration       TEXT NOT NULL DEFAULT 'none' CHECK (corroboration IN ('none', 'partly', 'documented')),
    corroborated_by     UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    corroborated_at     TIMESTAMPTZ,
    evidence_object_id  UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    expires_at          TIMESTAMPTZ NOT NULL,
    claim_state         TEXT NOT NULL DEFAULT 'current' CHECK (claim_state IN ('current', 'superseded', 'expired', 'withdrawn')),
    supersedes          UUID REFERENCES casework.rfi_answers(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (expires_at > answered_at),
    CHECK ((corroboration = 'none') = (corroborated_by IS NULL)),
    CHECK ((corroborated_by IS NULL) = (corroborated_at IS NULL)),
    CHECK ((corroboration = 'documented') = (evidence_object_id IS NOT NULL)),
    CHECK (supersedes IS NULL OR supersedes <> id)
);
CREATE INDEX IF NOT EXISTS idx_rfi_answers_rfi ON casework.rfi_answers (rfi_id, claim_state);
CREATE INDEX IF NOT EXISTS idx_rfi_answers_expiry ON casework.rfi_answers (expires_at) WHERE claim_state = 'current';

-- Internal reports (M-66 side, M-68, DR-151). The reporter is shown to the MLRO only. A report is always suspicion tier.
CREATE TABLE IF NOT EXISTS casework.internal_reports (
    id                  UUID PRIMARY KEY,
    reporter_kind       TEXT NOT NULL CHECK (reporter_kind IN ('staff', 'liaison')),
    reporter_id         UUID NOT NULL,                        -- canonical staff or user id (no foreign key); MLRO only
    branch_id           UUID,                                  -- canonical branch (no foreign key)
    subject_party_id    UUID,                                  -- no foreign key; NULL when the report names no customer
    channel             TEXT NOT NULL CHECK (channel IN ('branch_window', 'liaison_monthly', 'direct', 'letter')),
    body_enc            BYTEA NOT NULL,                        -- the report text (personal, AAD = id)
    received_at         TIMESTAMPTZ NOT NULL,
    received_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- NULL when the branch window wrote the row
    ack_due_at          TIMESTAMPTZ NOT NULL,
    timer_id            UUID REFERENCES admin.timers(id) ON DELETE RESTRICT,
    acknowledged_at     TIMESTAMPTZ,
    acknowledged_by     UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    linked_case_id      UUID REFERENCES casework.cases(id) ON DELETE RESTRICT,
    status              TEXT NOT NULL DEFAULT 'received' CHECK (status IN ('received', 'acknowledged', 'linked', 'closed')),
    tier                TEXT NOT NULL DEFAULT 'suspicion' CHECK (tier = 'suspicion'),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    CHECK (ack_due_at >= received_at),
    CHECK ((acknowledged_at IS NULL) = (acknowledged_by IS NULL)),
    CHECK (status = 'received' OR acknowledged_at IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_internal_reports_due ON casework.internal_reports (ack_due_at) WHERE acknowledged_at IS NULL;

-- Watches (M-35, DR-152). A UTR decision starts one watch; the closed trigger keys are section 3.5.
CREATE TABLE IF NOT EXISTS casework.watches (
    id                  UUID PRIMARY KEY,
    kind                TEXT NOT NULL CHECK (kind IN ('utr')),
    party_id            UUID NOT NULL,                         -- canonical party (no foreign key)
    case_id             UUID NOT NULL REFERENCES casework.cases(id) ON DELETE RESTRICT,
    origin_decision_id  UUID NOT NULL UNIQUE REFERENCES casework.decisions(id) ON DELETE RESTRICT,
    envelope            JSONB NOT NULL CHECK (jsonb_typeof(envelope) = 'object'),   -- the measures at the start (financial, as alert.alerts.measures)
    triggers            JSONB NOT NULL CHECK (jsonb_typeof(triggers) = 'array'),    -- closed keys of section 3.5
    period_days         INT NOT NULL CHECK (period_days BETWEEN 1 AND 365),
    started_at          TIMESTAMPTZ NOT NULL,
    expires_at          TIMESTAMPTZ NOT NULL,
    review_due_at       TIMESTAMPTZ NOT NULL,                  -- the first working day on or after expiry (section 3.5)
    timer_id            UUID REFERENCES admin.timers(id) ON DELETE RESTRICT,
    state               TEXT NOT NULL DEFAULT 'open' CHECK (state IN ('open', 'under_review', 'closed', 'upgraded')),
    review_decision     TEXT CHECK (review_decision IN ('extend', 'close', 'upgrade_str')),
    reviewed_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    reviewed_at         TIMESTAMPTZ,
    review_note_enc     BYTEA,                                 -- the reason of the review (personal, AAD = id)
    trigger_count       INT NOT NULL DEFAULT 0 CHECK (trigger_count >= 0),
    last_trigger_at     TIMESTAMPTZ,
    last_trigger_key    TEXT CHECK (last_trigger_key IS NULL OR last_trigger_key IN ('list_hit', 'authority_request',
                                                                                   'profile_change', 'new_alert')),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    CHECK (expires_at > started_at),
    CHECK ((review_decision IS NULL) = (reviewed_at IS NULL)),
    CHECK ((reviewed_by IS NULL) = (reviewed_at IS NULL)),
    CHECK ((trigger_count = 0) = (last_trigger_at IS NULL))
);
CREATE INDEX IF NOT EXISTS idx_watches_open ON casework.watches (expires_at) WHERE state IN ('open', 'under_review');
CREATE INDEX IF NOT EXISTS idx_watches_party ON casework.watches (party_id, state);

-- Hibernations (M-39, DR-153). Only inside a case, only after a no-report decision of that case.
CREATE TABLE IF NOT EXISTS casework.hibernations (
    id              UUID PRIMARY KEY,
    party_id        UUID NOT NULL,                             -- canonical party (no foreign key)
    scenario_id     UUID NOT NULL REFERENCES scn.scenarios(id) ON DELETE RESTRICT,
    case_id         UUID NOT NULL REFERENCES casework.cases(id) ON DELETE RESTRICT,
    decision_id     UUID NOT NULL REFERENCES casework.decisions(id) ON DELETE RESTRICT,   -- a no_report decision (guard)
    envelope        JSONB NOT NULL CHECK (jsonb_typeof(envelope) = 'object'),   -- {"measures": {"n", "total"}, "margin", "scenario_version_id"}
    period_days     INT NOT NULL CHECK (period_days BETWEEN 1 AND 180),          -- maximum 180 (M-39), fixed here
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at      TIMESTAMPTZ NOT NULL,
    created_by      UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    timer_id        UUID REFERENCES admin.timers(id) ON DELETE RESTRICT,
    ended_at        TIMESTAMPTZ,
    ended_by        UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    ended_reason    TEXT CHECK (ended_reason IN ('expired', 'envelope_exceeded', 'trigger', 'ended_by_person')),
    trigger_key     TEXT CHECK (trigger_key IN ('adverse_media', 'list_change', 'authority_request', 'profile_change')),
    version         INT NOT NULL DEFAULT 1,
    CHECK (expires_at > created_at),
    CHECK ((ended_at IS NULL) = (ended_reason IS NULL)),
    CHECK (ended_by IS NULL OR ended_reason = 'ended_by_person'),
    CHECK (ended_reason IS DISTINCT FROM 'ended_by_person' OR ended_by IS NOT NULL),
    CHECK (ended_reason IS DISTINCT FROM 'trigger' OR trigger_key IS NOT NULL)
);
-- One active hibernation per party and scenario.
CREATE UNIQUE INDEX IF NOT EXISTS uq_hibernations_active ON casework.hibernations (party_id, scenario_id) WHERE ended_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_hibernations_expiry ON casework.hibernations (expires_at) WHERE ended_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_hibernations_case ON casework.hibernations (case_id);

-- The hit ledger (DR-153): one row per alert that a hibernation suppressed. The alert row itself is kept in alert.alerts
-- (partitioned by created_at: the pair alert_id, alert_created_at identifies it). A hit is hidden from queues and counts
-- until released.
CREATE TABLE IF NOT EXISTS casework.hibernation_hits (
    id                UUID PRIMARY KEY,
    hibernation_id    UUID NOT NULL REFERENCES casework.hibernations(id) ON DELETE RESTRICT,
    alert_id          UUID NOT NULL,                           -- alert.alerts(id) (partitioned: no foreign key)
    alert_created_at  TIMESTAMPTZ NOT NULL,
    hit_at            TIMESTAMPTZ NOT NULL,
    measures          JSONB NOT NULL CHECK (jsonb_typeof(measures) = 'object'),   -- n and total of the alert (financial, as alert.alerts.measures)
    released_at       TIMESTAMPTZ,
    release_reason    TEXT CHECK (release_reason IN ('expired', 'envelope_exceeded', 'trigger', 'ended_by_person')),
    report_after      BOOLEAN NOT NULL DEFAULT FALSE,          -- the party had a report outcome after the hit (regret, section 3.6)
    UNIQUE (alert_id, alert_created_at),
    CHECK ((released_at IS NULL) = (release_reason IS NULL)),
    CHECK (released_at IS NULL OR released_at >= hit_at)
);
CREATE INDEX IF NOT EXISTS idx_hibernation_hits_open ON casework.hibernation_hits (hibernation_id) WHERE released_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_hibernation_hits_hibernation ON casework.hibernation_hits (hibernation_id, hit_at);

-- Alarms (M-53, M-22, DR-156): ageing breaches of the clocks, a broken chain, a declared conflict (M-67), a flat line (M-11)
-- and a critical QA finding. Each open alarm is also the supervisor-notification task: notified_at records when the supervisor
-- first saw it in the list, acknowledged_at when it was acknowledged. Payloads carry ids, codes and counts only.
CREATE TABLE IF NOT EXISTS casework.alarms (
    id                  UUID PRIMARY KEY,
    kind                TEXT NOT NULL CHECK (kind IN ('touch_breach', 'work_item_breach', 'chain_broken',
                                                      'conflict_declared', 'flat_line', 'qa_critical')),
    stage               SMALLINT CHECK (stage IS NULL OR stage IN (100, 150)),
    subject_kind        TEXT NOT NULL CHECK (subject_kind IN ('alert', 'work_item', 'case', 'scenario', 'qa_review')),
    subject_id          UUID NOT NULL,                         -- no foreign key (alerts are partitioned; scenarios and reviews are other tables)
    case_id             UUID REFERENCES casework.cases(id) ON DELETE RESTRICT,
    raised_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    supervisor_id       UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    notified_at         TIMESTAMPTZ,
    acknowledged_at     TIMESTAMPTZ,
    acknowledged_by     UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    closed_at           TIMESTAMPTZ,
    closed_by           UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    closure_reason_code TEXT REFERENCES ref.reason_codes(code) ON DELETE RESTRICT,
    state               TEXT NOT NULL DEFAULT 'open' CHECK (state IN ('open', 'acknowledged', 'closed')),
    payload             JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(payload) = 'object'),
    version             INT NOT NULL DEFAULT 1,
    CHECK ((kind IN ('touch_breach', 'work_item_breach')) = (stage IS NOT NULL)),
    CHECK ((state = 'closed') = (closed_at IS NOT NULL)),
    CHECK ((closed_at IS NULL) = (closed_by IS NULL)),
    CHECK ((closed_at IS NULL) = (closure_reason_code IS NULL)),
    CHECK ((state = 'open') = (acknowledged_at IS NULL) OR state = 'closed'),
    CHECK (notified_at IS NULL OR notified_at >= raised_at)
);
-- At most one live alarm per kind, subject and stage (a repeat raises nothing).
CREATE UNIQUE INDEX IF NOT EXISTS uq_alarms_live ON casework.alarms (kind, subject_id, COALESCE(stage, 0)) WHERE state <> 'closed';
CREATE INDEX IF NOT EXISTS idx_alarms_supervisor ON casework.alarms (supervisor_id, state);

-- Network snapshots (DR-154): the subgraph of a network case, kept as an evidence object (kind snapshot, owner kind network_snapshot).
CREATE TABLE IF NOT EXISTS casework.network_snapshots (
    id              UUID PRIMARY KEY,
    case_id         UUID NOT NULL REFERENCES casework.cases(id) ON DELETE RESTRICT,   -- the network parent
    seed_party_id   UUID NOT NULL,                             -- canonical party (no foreign key)
    node_count      INT NOT NULL CHECK (node_count >= 1),
    edge_count      INT NOT NULL CHECK (edge_count >= 0),
    graph           JSONB NOT NULL CHECK (jsonb_typeof(graph) = 'object'),   -- the nodes and edges the creator could see (personal)
    object_id       UUID NOT NULL UNIQUE REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    edges_as_of     TIMESTAMPTZ NOT NULL,
    created_by      UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_network_snapshots_case ON casework.network_snapshots (case_id, created_at);

-- Network edges (DATA-MODEL section 12, renamed by DR-145; DR-154). Built only from canonical rows; retired, never deleted.
CREATE TABLE IF NOT EXISTS casework.network_edges (
    id                 UUID PRIMARY KEY,
    from_party         UUID NOT NULL,                          -- canonical party (no foreign key)
    to_party           UUID NOT NULL,
    kind               TEXT NOT NULL CHECK (kind IN ('shared_contact', 'shared_device', 'transfer', 'ownership', 'kinship')),
    hidden_by_default  BOOLEAN NOT NULL DEFAULT FALSE,         -- TRUE exactly for kinship (section 3.7)
    weight             INT NOT NULL CHECK (weight >= 1),       -- the number of canonical rows that back the edge
    basis              JSONB NOT NULL CHECK (jsonb_typeof(basis) = 'object'),   -- canonical row ids and link types (personal)
    first_seen         TIMESTAMPTZ NOT NULL,
    last_seen          TIMESTAMPTZ NOT NULL,
    built_batch_id     UUID,
    retired_at         TIMESTAMPTZ,
    CHECK (from_party <> to_party),
    CHECK (hidden_by_default = (kind = 'kinship')),
    CHECK (kind = 'ownership' OR from_party < to_party),       -- undirected kinds are stored once, smaller id first
    CHECK (last_seen >= first_seen)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_network_edges_live ON casework.network_edges (from_party, to_party, kind) WHERE retired_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_network_edges_to ON casework.network_edges (to_party) WHERE retired_at IS NULL;

-- QA samples (M-47, M-49, DR-155): one row per drawn item, the register of draws. Calibration rows are one per reviewer.
CREATE TABLE IF NOT EXISTS casework.qa_samples (
    id               UUID PRIMARY KEY,
    draw_id          UUID NOT NULL,
    population       TEXT NOT NULL CHECK (population IN ('closed_alerts', 'closed_cases', 'shelved_decisions',
                                                        'mlro_decisions', 'hibernation_hits', 'screening_suppressions',
                                                        'low_score_forced', 'calibration', 'targeted_repeat')),
    source_kind      TEXT NOT NULL CHECK (source_kind IN ('alert', 'case', 'decision', 'hibernation_hit',
                                                          'pair_memory_suppression')),
    source_id        UUID NOT NULL,                             -- no foreign key (alerts are partitioned)
    source_at        TIMESTAMPTZ NOT NULL,                      -- alert created_at, or decision decided_at (partition key of alerts)
    case_id          UUID REFERENCES casework.cases(id) ON DELETE RESTRICT,
    stratum          TEXT NOT NULL CHECK (length(stratum) BETWEEN 1 AND 80),
    draw_mode        TEXT NOT NULL CHECK (draw_mode IN ('stratified', 'random', 'targeted')),
    draw_seed        TEXT NOT NULL CHECK (length(draw_seed) BETWEEN 32 AND 128),   -- hex; the draw is reproducible from it
    population_size  BIGINT NOT NULL CHECK (population_size >= 1),
    sample_size      INT NOT NULL CHECK (sample_size >= 1),
    reviewer_id      UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    drawn_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- NULL for a scheduled draw
    drawn_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    due_at           TIMESTAMPTZ NOT NULL,
    state            TEXT NOT NULL DEFAULT 'open' CHECK (state IN ('open', 'reviewed', 'void')),
    grant_id         UUID REFERENCES casework.case_acl(id) ON DELETE RESTRICT,   -- the time-boxed read grant of a case item
    is_calibration   BOOLEAN NOT NULL DEFAULT FALSE,
    golden_outcome   TEXT CHECK (golden_outcome IN ('pass', 'pass_with_comment', 'minor', 'major', 'critical')),
    version          INT NOT NULL DEFAULT 1,
    CHECK ((golden_outcome IS NOT NULL) = is_calibration),
    CHECK (due_at > drawn_at),
    CHECK (source_kind NOT IN ('case', 'decision') OR case_id IS NOT NULL)
);
-- An item is reviewed once in the production sample; calibration items are reviewed by each calibrator.
CREATE UNIQUE INDEX IF NOT EXISTS uq_qa_samples_item ON casework.qa_samples (source_kind, source_id)
    WHERE is_calibration = FALSE AND state <> 'void';
CREATE UNIQUE INDEX IF NOT EXISTS uq_qa_samples_calibration ON casework.qa_samples (source_id, reviewer_id, draw_id)
    WHERE is_calibration = TRUE;
CREATE INDEX IF NOT EXISTS idx_qa_samples_reviewer ON casework.qa_samples (reviewer_id, state, due_at);
CREATE INDEX IF NOT EXISTS idx_qa_samples_draw ON casework.qa_samples (draw_id);

-- QA reviews: one per sample row, written once (append-only, DR-155). The reviewer is checked against the sample by a guard.
CREATE TABLE IF NOT EXISTS casework.qa_reviews (
    id              UUID PRIMARY KEY,
    sample_id       UUID NOT NULL UNIQUE REFERENCES casework.qa_samples(id) ON DELETE RESTRICT,
    reviewer_id     UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    rubric_version  TEXT NOT NULL CHECK (rubric_version = 'qa-rubric.v1'),
    scores          JSONB NOT NULL CHECK (jsonb_typeof(scores) = 'object'),   -- eight criteria, each pass to critical
    overall         TEXT NOT NULL CHECK (overall IN ('pass', 'pass_with_comment', 'minor', 'major', 'critical')),
    comment_enc     BYTEA,                                  -- personal, AAD = id
    completed_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_qa_reviews_reviewer ON casework.qa_reviews (reviewer_id, completed_at);

-- QA findings: one per failed criterion. Major: a coaching item (coaching_state). Critical: an alarm and a re-review list.
CREATE TABLE IF NOT EXISTS casework.qa_findings (
    id                    UUID PRIMARY KEY,
    review_id             UUID NOT NULL REFERENCES casework.qa_reviews(id) ON DELETE RESTRICT,
    criterion             TEXT NOT NULL CHECK (criterion IN ('evidence', 'reason', 'escalation', 'corroboration', 'timeliness',
                                                             'confidentiality', 'narrative', 'authority')),
    severity              TEXT NOT NULL CHECK (severity IN ('minor', 'major', 'critical')),
    target_kind           TEXT NOT NULL CHECK (target_kind IN ('alert', 'case', 'decision', 'rfi', 'hibernation')),
    target_id             UUID NOT NULL,                       -- no foreign key (alerts are partitioned)
    coaching_state        TEXT NOT NULL DEFAULT 'none' CHECK (coaching_state IN ('none', 'open', 'done')),
    coaching_due_at       TIMESTAMPTZ,
    re_review_sample_ids  UUID[] NOT NULL DEFAULT '{}' CHECK (cardinality(re_review_sample_ids) <= 50),
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK ((severity = 'major') = (coaching_state <> 'none')),
    CHECK ((coaching_state = 'none') = (coaching_due_at IS NULL))
);
CREATE INDEX IF NOT EXISTS idx_qa_findings_review ON casework.qa_findings (review_id);
CREATE INDEX IF NOT EXISTS idx_qa_findings_coaching ON casework.qa_findings (coaching_state, coaching_due_at) WHERE coaching_state = 'open';
```

### 1.2 `0065_casework_followup_guards_grants.sql`

```sql
-- migration 0065_casework_followup_guards_grants.sql
-- Phase 19. Guards (second layer under the grants), grants per write path with UPDATE column lists, settings, schedules,
-- the classification and the retention rows, and the sealed columns. Reuses casework.append_only() of phase 17 (0061).

-- Append-only: QA reviews, QA findings and network snapshots.
DROP TRIGGER IF EXISTS trg_qa_reviews_append_only ON casework.qa_reviews;
CREATE TRIGGER trg_qa_reviews_append_only BEFORE UPDATE OR DELETE ON casework.qa_reviews
    FOR EACH ROW EXECUTE FUNCTION casework.append_only();
DROP TRIGGER IF EXISTS trg_qa_findings_append_only ON casework.qa_findings;
CREATE TRIGGER trg_qa_findings_append_only BEFORE UPDATE OR DELETE ON casework.qa_findings
    FOR EACH ROW EXECUTE FUNCTION casework.append_only();
DROP TRIGGER IF EXISTS trg_network_snapshots_append_only ON casework.network_snapshots;
CREATE TRIGGER trg_network_snapshots_append_only BEFORE UPDATE OR DELETE ON casework.network_snapshots
    FOR EACH ROW EXECUTE FUNCTION casework.append_only();

-- A review is written only by the sample's reviewer, and only while the sample is open (second layer of DR-155).
CREATE OR REPLACE FUNCTION casework.qa_reviews_insert_guard() RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE
    v_reviewer UUID;
    v_state    TEXT;
BEGIN
    SELECT s.reviewer_id, s.state INTO v_reviewer, v_state FROM casework.qa_samples s WHERE s.id = NEW.sample_id;
    IF NOT FOUND OR v_reviewer <> NEW.reviewer_id OR v_state <> 'open' THEN
        RAISE EXCEPTION 'a review is written by the sample reviewer while the sample is open' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_qa_reviews_insert_guard ON casework.qa_reviews;
CREATE TRIGGER trg_qa_reviews_insert_guard BEFORE INSERT ON casework.qa_reviews
    FOR EACH ROW EXECUTE FUNCTION casework.qa_reviews_insert_guard();

-- A hibernation follows a no-report decision of the same case (M-39).
CREATE OR REPLACE FUNCTION casework.hibernations_insert_guard() RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE
    v_type TEXT;
    v_case UUID;
BEGIN
    SELECT d.type, d.case_id INTO v_type, v_case FROM casework.decisions d WHERE d.id = NEW.decision_id;
    IF NOT FOUND OR v_type <> 'no_report' OR v_case <> NEW.case_id THEN
        RAISE EXCEPTION 'a hibernation follows a no-report decision of its case' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_hibernations_insert_guard ON casework.hibernations;
CREATE TRIGGER trg_hibernations_insert_guard BEFORE INSERT ON casework.hibernations
    FOR EACH ROW EXECUTE FUNCTION casework.hibernations_insert_guard();

-- An RFI answer is recorded only for a released RFI.
CREATE OR REPLACE FUNCTION casework.rfi_answers_insert_guard() RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE
    v_status TEXT;
BEGIN
    SELECT r.status INTO v_status FROM casework.rfis r WHERE r.id = NEW.rfi_id;
    IF v_status IS DISTINCT FROM 'released' AND v_status IS DISTINCT FROM 'answered' THEN
        RAISE EXCEPTION 'an answer is recorded for a released RFI' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_rfi_answers_insert_guard ON casework.rfi_answers;
CREATE TRIGGER trg_rfi_answers_insert_guard BEFORE INSERT ON casework.rfi_answers
    FOR EACH ROW EXECUTE FUNCTION casework.rfi_answers_insert_guard();

-- Claims: identity fixed; corroboration only rises; a claim that is not current is terminal; no delete.
CREATE OR REPLACE FUNCTION casework.rfi_answers_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'answers are claims and are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF (NEW.id, NEW.rfi_id, NEW.question_id, NEW.answer_kind, NEW.value_enc, NEW.source_kind, NEW.source_ref,
        NEW.answered_by, NEW.answered_at, NEW.expires_at, NEW.supersedes, NEW.created_at)
       IS DISTINCT FROM
       (OLD.id, OLD.rfi_id, OLD.question_id, OLD.answer_kind, OLD.value_enc, OLD.source_kind, OLD.source_ref,
        OLD.answered_by, OLD.answered_at, OLD.expires_at, OLD.supersedes, OLD.created_at) THEN
        RAISE EXCEPTION 'a claim does not change; a correction is a new row' USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.claim_state <> 'current' AND NEW.claim_state IS DISTINCT FROM OLD.claim_state THEN
        RAISE EXCEPTION 'a claim that is not current is terminal' USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.corroboration = 'documented' AND NEW.corroboration IS DISTINCT FROM 'documented' THEN
        RAISE EXCEPTION 'documented corroboration is kept' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_rfi_answers_guard ON casework.rfi_answers;
CREATE TRIGGER trg_rfi_answers_guard BEFORE UPDATE OR DELETE ON casework.rfi_answers
    FOR EACH ROW EXECUTE FUNCTION casework.rfi_answers_guard();

-- RFIs: identity fixed at drafting; status moves only along the lines of section 3.2; no delete.
CREATE OR REPLACE FUNCTION casework.rfis_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'RFIs are withdrawn, never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF (NEW.id, NEW.case_id, NEW.kind, NEW.direction, NEW.template_key, NEW.template_version, NEW.purpose_code,
        NEW.legal_basis, NEW.recipient_kind, NEW.recipient_ref, NEW.recipient_name_enc, NEW.party_id, NEW.contact_allowed,
        NEW.authority_request_id, NEW.proposer_id, NEW.created_at)
       IS DISTINCT FROM
       (OLD.id, OLD.case_id, OLD.kind, OLD.direction, OLD.template_key, OLD.template_version, OLD.purpose_code,
        OLD.legal_basis, OLD.recipient_kind, OLD.recipient_ref, OLD.recipient_name_enc, OLD.party_id, OLD.contact_allowed,
        OLD.authority_request_id, OLD.proposer_id, OLD.created_at) THEN
        RAISE EXCEPTION 'the identity of an RFI is fixed at drafting' USING ERRCODE = 'check_violation';
    END IF;
    IF NEW.status IS DISTINCT FROM OLD.status AND NOT (
            (OLD.status = 'draft'            AND NEW.status IN ('pending_release', 'withdrawn'))
         OR (OLD.status = 'pending_release'  AND NEW.status IN ('released', 'rejected', 'withdrawn'))
         OR (OLD.status = 'released'         AND NEW.status IN ('answered', 'closed'))
         OR (OLD.status = 'answered'         AND NEW.status = 'closed')) THEN
        RAISE EXCEPTION 'an RFI status move is not allowed' USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.status IN ('closed', 'withdrawn', 'rejected') AND NEW.status IS DISTINCT FROM OLD.status THEN
        RAISE EXCEPTION 'a terminal RFI does not change' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_rfis_guard ON casework.rfis;
CREATE TRIGGER trg_rfis_guard BEFORE UPDATE OR DELETE ON casework.rfis
    FOR EACH ROW EXECUTE FUNCTION casework.rfis_guard();

-- Questions: identity fixed; open -> answered or waived, once; no delete.
CREATE OR REPLACE FUNCTION casework.rfi_questions_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'questions are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF (NEW.id, NEW.rfi_id, NEW.ordinal, NEW.question_key, NEW.answer_format, NEW.required, NEW.created_at)
       IS DISTINCT FROM
       (OLD.id, OLD.rfi_id, OLD.ordinal, OLD.question_key, OLD.answer_format, OLD.required, OLD.created_at) THEN
        RAISE EXCEPTION 'a question is fixed at drafting' USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.state <> 'open' AND NEW.state IS DISTINCT FROM OLD.state THEN
        RAISE EXCEPTION 'a question that is answered or waived is terminal' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_rfi_questions_guard ON casework.rfi_questions;
CREATE TRIGGER trg_rfi_questions_guard BEFORE UPDATE OR DELETE ON casework.rfi_questions
    FOR EACH ROW EXECUTE FUNCTION casework.rfi_questions_guard();

-- Authority requests: identity fixed; status moves forward; approval once; no delete.
CREATE OR REPLACE FUNCTION casework.authority_requests_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'authority requests are kept in the register' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF (NEW.id, NEW.authority_kind, NEW.authority_ref, NEW.received_at, NEW.deadline_at, NEW.deadline_source,
        NEW.subject_party_ids, NEW.questions_enc, NEW.tier, NEW.registered_by, NEW.created_at)
       IS DISTINCT FROM
       (OLD.id, OLD.authority_kind, OLD.authority_ref, OLD.received_at, OLD.deadline_at, OLD.deadline_source,
        OLD.subject_party_ids, OLD.questions_enc, OLD.tier, OLD.registered_by, OLD.created_at) THEN
        RAISE EXCEPTION 'the identity of an authority request is fixed' USING ERRCODE = 'check_violation';
    END IF;
    IF NEW.status IS DISTINCT FROM OLD.status AND NOT (
            (OLD.status = 'received' AND NEW.status = 'drafting')
         OR (OLD.status = 'drafting' AND NEW.status = 'approved')
         OR (OLD.status = 'approved' AND NEW.status = 'answered')
         OR (OLD.status = 'answered' AND NEW.status = 'closed')) THEN
        RAISE EXCEPTION 'an authority request status move is not allowed' USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.approved_at IS NOT NULL AND (NEW.approved_at IS DISTINCT FROM OLD.approved_at OR NEW.approved_by IS DISTINCT FROM OLD.approved_by) THEN
        RAISE EXCEPTION 'an approval is kept' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_authority_requests_guard ON casework.authority_requests;
CREATE TRIGGER trg_authority_requests_guard BEFORE UPDATE OR DELETE ON casework.authority_requests
    FOR EACH ROW EXECUTE FUNCTION casework.authority_requests_guard();

-- Internal reports: identity fixed (the reporter included); acknowledgement once; no delete.
CREATE OR REPLACE FUNCTION casework.internal_reports_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'internal reports are kept' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF (NEW.id, NEW.reporter_kind, NEW.reporter_id, NEW.branch_id, NEW.subject_party_id, NEW.channel, NEW.body_enc,
        NEW.received_at, NEW.received_by, NEW.ack_due_at, NEW.tier, NEW.created_at)
       IS DISTINCT FROM
       (OLD.id, OLD.reporter_kind, OLD.reporter_id, OLD.branch_id, OLD.subject_party_id, OLD.channel, OLD.body_enc,
        OLD.received_at, OLD.received_by, OLD.ack_due_at, OLD.tier, OLD.created_at) THEN
        RAISE EXCEPTION 'the identity of an internal report is fixed' USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.acknowledged_at IS NOT NULL AND (NEW.acknowledged_at IS DISTINCT FROM OLD.acknowledged_at
                                             OR NEW.acknowledged_by IS DISTINCT FROM OLD.acknowledged_by) THEN
        RAISE EXCEPTION 'an acknowledgement is kept' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_internal_reports_guard ON casework.internal_reports;
CREATE TRIGGER trg_internal_reports_guard BEFORE UPDATE OR DELETE ON casework.internal_reports
    FOR EACH ROW EXECUTE FUNCTION casework.internal_reports_guard();

-- Watches: identity fixed; closed and upgraded are terminal; no delete.
CREATE OR REPLACE FUNCTION casework.watches_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'watches are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF (NEW.id, NEW.kind, NEW.party_id, NEW.case_id, NEW.origin_decision_id, NEW.envelope, NEW.triggers, NEW.period_days,
        NEW.started_at, NEW.created_at)
       IS DISTINCT FROM
       (OLD.id, OLD.kind, OLD.party_id, OLD.case_id, OLD.origin_decision_id, OLD.envelope, OLD.triggers, OLD.period_days,
        OLD.started_at, OLD.created_at) THEN
        RAISE EXCEPTION 'the identity of a watch is fixed' USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.state IN ('closed', 'upgraded') AND (NEW.state, NEW.expires_at, NEW.review_decision, NEW.reviewed_at)
       IS DISTINCT FROM (OLD.state, OLD.expires_at, OLD.review_decision, OLD.reviewed_at) THEN
        RAISE EXCEPTION 'a closed or upgraded watch is terminal' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_watches_guard ON casework.watches;
CREATE TRIGGER trg_watches_guard BEFORE UPDATE OR DELETE ON casework.watches
    FOR EACH ROW EXECUTE FUNCTION casework.watches_guard();

-- Hibernations: identity fixed; ended once; no delete.
CREATE OR REPLACE FUNCTION casework.hibernations_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'hibernations are kept in the ledger' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF (NEW.id, NEW.party_id, NEW.scenario_id, NEW.case_id, NEW.decision_id, NEW.envelope, NEW.period_days,
        NEW.created_at, NEW.expires_at, NEW.created_by)
       IS DISTINCT FROM
       (OLD.id, OLD.party_id, OLD.scenario_id, OLD.case_id, OLD.decision_id, OLD.envelope, OLD.period_days,
        OLD.created_at, OLD.expires_at, OLD.created_by) THEN
        RAISE EXCEPTION 'the identity of a hibernation is fixed' USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.ended_at IS NOT NULL AND (NEW.ended_at, NEW.ended_reason, NEW.ended_by) IS DISTINCT FROM
       (OLD.ended_at, OLD.ended_reason, OLD.ended_by) THEN
        RAISE EXCEPTION 'an ended hibernation does not change' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_hibernations_guard ON casework.hibernations;
CREATE TRIGGER trg_hibernations_guard BEFORE UPDATE OR DELETE ON casework.hibernations
    FOR EACH ROW EXECUTE FUNCTION casework.hibernations_guard();

-- Hit ledger: identity fixed; released once; report_after only rises; no delete.
CREATE OR REPLACE FUNCTION casework.hibernation_hits_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'hibernation hits are kept in the ledger' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF (NEW.id, NEW.hibernation_id, NEW.alert_id, NEW.alert_created_at, NEW.hit_at, NEW.measures)
       IS DISTINCT FROM
       (OLD.id, OLD.hibernation_id, OLD.alert_id, OLD.alert_created_at, OLD.hit_at, OLD.measures) THEN
        RAISE EXCEPTION 'the identity of a hibernation hit is fixed' USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.released_at IS NOT NULL AND (NEW.released_at, NEW.release_reason) IS DISTINCT FROM (OLD.released_at, OLD.release_reason) THEN
        RAISE EXCEPTION 'a released hit does not change' USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.report_after AND NOT NEW.report_after THEN
        RAISE EXCEPTION 'a regret mark is kept' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_hibernation_hits_guard ON casework.hibernation_hits;
CREATE TRIGGER trg_hibernation_hits_guard BEFORE UPDATE OR DELETE ON casework.hibernation_hits
    FOR EACH ROW EXECUTE FUNCTION casework.hibernation_hits_guard();

-- Alarms: identity fixed; closed is terminal; notified once; no delete.
CREATE OR REPLACE FUNCTION casework.alarms_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'alarms are kept' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF (NEW.id, NEW.kind, NEW.stage, NEW.subject_kind, NEW.subject_id, NEW.case_id, NEW.raised_at, NEW.payload)
       IS DISTINCT FROM
       (OLD.id, OLD.kind, OLD.stage, OLD.subject_kind, OLD.subject_id, OLD.case_id, OLD.raised_at, OLD.payload) THEN
        RAISE EXCEPTION 'the identity of an alarm is fixed' USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.state = 'closed' AND NEW.state IS DISTINCT FROM OLD.state THEN
        RAISE EXCEPTION 'a closed alarm is terminal' USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.notified_at IS NOT NULL AND NEW.notified_at IS DISTINCT FROM OLD.notified_at THEN
        RAISE EXCEPTION 'the notification time is kept' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_alarms_guard ON casework.alarms;
CREATE TRIGGER trg_alarms_guard BEFORE UPDATE OR DELETE ON casework.alarms
    FOR EACH ROW EXECUTE FUNCTION casework.alarms_guard();

-- Network edges: identity fixed; retired once; no delete.
CREATE OR REPLACE FUNCTION casework.network_edges_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'network edges are retired, never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF (NEW.id, NEW.from_party, NEW.to_party, NEW.kind, NEW.hidden_by_default, NEW.first_seen)
       IS DISTINCT FROM
       (OLD.id, OLD.from_party, OLD.to_party, OLD.kind, OLD.hidden_by_default, OLD.first_seen) THEN
        RAISE EXCEPTION 'the identity of a network edge is fixed' USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.retired_at IS NOT NULL AND NEW.retired_at IS DISTINCT FROM OLD.retired_at THEN
        RAISE EXCEPTION 'a retired edge does not change' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_network_edges_guard ON casework.network_edges;
CREATE TRIGGER trg_network_edges_guard BEFORE UPDATE OR DELETE ON casework.network_edges
    FOR EACH ROW EXECUTE FUNCTION casework.network_edges_guard();

-- QA samples: identity fixed; the reviewer changes only while open (a conflict reassignment); reviewed and void are terminal.
CREATE OR REPLACE FUNCTION casework.qa_samples_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'the sample register is never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF (NEW.id, NEW.draw_id, NEW.population, NEW.source_kind, NEW.source_id, NEW.source_at, NEW.case_id, NEW.stratum,
        NEW.draw_mode, NEW.draw_seed, NEW.population_size, NEW.sample_size, NEW.drawn_by, NEW.drawn_at, NEW.due_at,
        NEW.is_calibration, NEW.golden_outcome)
       IS DISTINCT FROM
       (OLD.id, OLD.draw_id, OLD.population, OLD.source_kind, OLD.source_id, OLD.source_at, OLD.case_id, OLD.stratum,
        OLD.draw_mode, OLD.draw_seed, OLD.population_size, OLD.sample_size, OLD.drawn_by, OLD.drawn_at, OLD.due_at,
        OLD.is_calibration, OLD.golden_outcome) THEN
        RAISE EXCEPTION 'a sample row is fixed at the draw' USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.reviewer_id IS DISTINCT FROM NEW.reviewer_id AND OLD.state <> 'open' THEN
        RAISE EXCEPTION 'the reviewer of a sample changes only while it is open' USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.state <> 'open' AND NEW.state IS DISTINCT FROM OLD.state THEN
        RAISE EXCEPTION 'a reviewed or void sample is terminal' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_qa_samples_guard ON casework.qa_samples;
CREATE TRIGGER trg_qa_samples_guard BEFORE UPDATE OR DELETE ON casework.qa_samples
    FOR EACH ROW EXECUTE FUNCTION casework.qa_samples_guard();
```

```sql
-- Grants. Every write path of this phase, with the UPDATE column lists its services use. Reads of the tables are through
-- the API (no grant to akr_readonly, akr_ingest, akr_audit_writer or akr_monitor: the registers are personal and case-bound).
GRANT SELECT, INSERT ON casework.authority_requests, casework.rfis, casework.rfi_questions, casework.rfi_answers,
    casework.internal_reports, casework.watches, casework.hibernations, casework.hibernation_hits, casework.alarms,
    casework.network_snapshots, casework.network_edges, casework.qa_samples, casework.qa_reviews, casework.qa_findings
    TO akr_app;
GRANT UPDATE (acknowledged_at, status, response_rfi_id, approved_by, approved_at, disclosed_at, disclosed_items,
              linked_case_ids, timer_id, updated_at, version) ON casework.authority_requests TO akr_app;
GRANT UPDATE (status, clock_unit, clock_days, deadline_at, extensions, timer_id, workflow_instance_id, released_by,
              released_at, answered_at, closed_at, updated_at, version) ON casework.rfis TO akr_app;
GRANT UPDATE (state) ON casework.rfi_questions TO akr_app;
GRANT UPDATE (corroboration, corroborated_by, corroborated_at, evidence_object_id, claim_state) ON casework.rfi_answers TO akr_app;
GRANT UPDATE (acknowledged_at, acknowledged_by, linked_case_id, status, timer_id, version) ON casework.internal_reports TO akr_app;
GRANT UPDATE (state, expires_at, review_due_at, timer_id, review_decision, reviewed_by, reviewed_at, review_note_enc,
              trigger_count, last_trigger_at, last_trigger_key, updated_at, version) ON casework.watches TO akr_app;
GRANT UPDATE (ended_at, ended_by, ended_reason, trigger_key, timer_id, version) ON casework.hibernations TO akr_app;
GRANT UPDATE (released_at, release_reason, report_after) ON casework.hibernation_hits TO akr_app;
GRANT UPDATE (state, supervisor_id, notified_at, acknowledged_at, acknowledged_by, closed_at, closed_by,
              closure_reason_code, version) ON casework.alarms TO akr_app;
GRANT UPDATE (weight, last_seen, basis, built_batch_id, retired_at) ON casework.network_edges TO akr_app;
GRANT UPDATE (state, reviewer_id, grant_id, version) ON casework.qa_samples TO akr_app;
GRANT UPDATE (coaching_state) ON casework.qa_findings TO akr_app;
-- Existing grants used by this phase: casework.cases INSERT (network children and parents, phase 17), casework.case_events
-- INSERT (phase 17), casework.case_acl INSERT and UPDATE(revoked_by, revoked_at) (phase 18; the QA grants, reason qa_review),
-- casework.decisions SELECT (the hibernation guard), casework.work_items INSERT and UPDATE
-- (phase 18, breach follow-ups), alert.alerts SELECT (phase 17), admin.timers INSERT (phase 00), admin.evidence_objects SELECT
-- (phase 02), admin.settings SELECT (phase 00), casework.cases UPDATE(parent-closure hook, phase 18 columns only).

-- Settings (copy of the phase 18 column list; floor NULL throughout: no regulatory minimum of the {min} shape).
-- governed = TRUE only where a change must go through a change request (phase 04 `setting` target).
INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('casework.watch_days', '90', 'integer', NULL, FALSE,
  '{"ar": "مدة مراقبة بلاغ الإجراء المؤقت بالأيام التقويمية (M-35)", "en": "Default length of a UTR watch in calendar days (M-35)"}'),
 ('casework.hibernation_default_days', '90', 'integer', NULL, FALSE,
  '{"ar": "المدة الافتراضية لتعليق السيناريو لطرف (أيام تقويمية، M-39)", "en": "Default hibernation period in calendar days (M-39)"}'),
 ('casework.hibernation_max_days', '180', 'integer', NULL, TRUE,
  '{"ar": "أقصى مدة للتعليق بالأيام التقويمية (M-39؛ القيد في القاعدة 180)", "en": "Largest hibernation period in calendar days (M-39; the database CHECK fixes 180)"}'),
 ('casework.hibernation_margin', '0.2', 'decimal', NULL, TRUE,
  '{"ar": "هامش المغلف: زيادة النسبة فوقه تنهي التعليق (DSL A.7)", "en": "Envelope margin: a rise above it ends a hibernation (DSL A.7)"}'),
 ('casework.rfi_internal_days', '2', 'integer', NULL, TRUE,
  '{"ar": "مهلة طلبات الفروع والوحدات والمدير المسؤول بأيام العمل (research 28 s.9.5)", "en": "Branch, unit and RM request deadline in working days (research 28 s.9.5)"}'),
 ('casework.rfi_outreach_days', '5', 'integer', NULL, TRUE,
  '{"ar": "مهلة التواصل مع العميل عبر الفرع بأيام العمل (research 28 s.9.5)", "en": "Customer outreach deadline in working days (research 28 s.9.5)"}'),
 ('casework.rfi_extension_days', '3', 'integer', NULL, FALSE,
  '{"ar": "مدة كل تمديد لطلب المعلومات بأيام العمل (حدان كحد أقصى)", "en": "Working days added by one RFI extension (two at most)"}'),
 ('casework.rfi_claim_expiry_days', '365', 'integer', NULL, TRUE,
  '{"ar": "مدة صلاحية الإجابة كادعاء بالأيام التقويمية (L-47)", "en": "Validity of an RFI answer as a claim in calendar days (L-47)"}'),
 ('casework.authority_default_days', '3', 'integer', NULL, TRUE,
  '{"ar": "مهلة طلب الجهة المختصة عندما لا تذكر جهة الطلب مهلة (بأيام العمل)", "en": "Deadline of an authority request that states none (working days)"}'),
 ('casework.internal_report_ack_hours', '8', 'integer', NULL, TRUE,
  '{"ar": "مهلة الإقرار بالتقرير الداخلي (ساعات، مقترح)", "en": "Hours to acknowledge an internal report (proposal; plain hours, Open doubts 3)"}'),
 ('casework.network_max_children', '25', 'integer', NULL, FALSE,
  '{"ar": "أقصى عدد من الأطراف الفرعية في حالة الشبكة", "en": "Largest number of subject children of one network case"}'),
 ('casework.network_transfer_min_confidence', '0.9', 'decimal', NULL, TRUE,
  '{"ar": "أدنى ثقة لربط التحويلات في الشبكة", "en": "Lowest payment-link confidence that builds a transfer edge"}'),
 ('casework.network_supernode_degree', '200', 'integer', NULL, FALSE,
  '{"ar": "درجة الطرف التي تُلخَّص عندها روابطه في عرض الشبكة (T-21)", "en": "Live degree above which a node is summarised, not listed (T-21)"}'),
 ('qa.confidence', '0.95', 'decimal', NULL, FALSE,
  '{"ar": "مستوى الثقة C في حساب حجم العينة", "en": "Confidence C of the sample-size calculator (M-47)"}'),
 ('qa.expected_defect_rate', '0.05', 'decimal', NULL, FALSE,
  '{"ar": "معدل الخطأ المتوقع p في حساب حجم العينة", "en": "Expected defect rate p of the sample-size calculator (M-47)"}'),
 ('qa.review_working_days', '3', 'integer', NULL, FALSE,
  '{"ar": "مهلة المراجعة بأيام العمل", "en": "Working days to complete a QA review"}'),
 ('qa.grant_days', '3', 'integer', NULL, FALSE,
  '{"ar": "مدة الإذن المؤقت للاطلاع على حالة مختارة بالأيام التقويمية (M-49)", "en": "Calendar days of the time-boxed read grant of a sampled case (M-49)"}'),
 ('qa.suppression_sample_percent', '5', 'decimal', NULL, TRUE,
  '{"ar": "نسبة العينة الأسبوعية من حالات كبح ذاكرة الأزواج (المرحلة 14)", "en": "Weekly sample share of pair-memory suppressions, per cent (phase 14)"}'),
 ('qa.forced_low_score_daily', '20', 'integer', NULL, FALSE,
  '{"ar": "عدد التنبيهات منخفضة الدرجة في العينة اليومية الإجبارية (M-16)", "en": "Low-score alerts in the daily forced sample (M-16)"}'),
 ('qa.min_per_stratum', '3', 'integer', NULL, FALSE,
  '{"ar": "أدنى عدد من كل طبقة في العينة الطبقية", "en": "Smallest number of items drawn from each stratum"}'),
 ('qa.min_reviewer_items', '20', 'integer', NULL, FALSE,
  '{"ar": "أدنى عدد من المراجعات لإظهار معدلات المراجع", "en": "Fewest reviews before a reviewer rate is shown"}'),
 ('qa.coaching_working_days', '10', 'integer', NULL, FALSE,
  '{"ar": "مهلة بند التدريب بعد نتيجة جسيمة بأيام العمل", "en": "Working days to complete a coaching item after a major finding"}'),
 ('qa.flat_line_days', '5', 'integer', NULL, FALSE,
  '{"ar": "عدد أيام العمل المتتالية بعدد ثابت قبل إنذار الخط المسطح (M-11)", "en": "Business days of an unchanged daily count before a flat-line alarm (M-11)"}'),
 ('qa.bulk_reopen_max', '25', 'integer', NULL, FALSE,
  '{"ar": "أقصى عدد من الحالات في إعادة الفتح الجماعية بعد نتيجة حرجة (M-40)", "en": "Largest number of cases in one bulk reopen after a critical finding (M-40)"}')
ON CONFLICT (key) DO NOTHING;

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('schedule.rfi_clock_scan', '"0 35 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مسح ساعات طلبات المعلومات وانتهاء الادعاءات (احتياطي)", "en": "RFI clock and claim-expiry backstop scan (fallback for the timers)"}'),
 ('schedule.network_edge_build', '"0 20 2 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول إعادة بناء روابط الشبكة من الأصل (احتياطي)", "en": "Daily network edge sweep from canonical links (backstop of canon.touched)"}'),
 ('schedule.flat_line_check', '"0 30 4 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول فحص الخط المسطح للسيناريوهات", "en": "Daily flat-line check of scenario counts (M-11)"}'),
 ('schedule.qa_forced_sample', '"0 10 3 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول العينة اليومية الإجبارية (M-16)", "en": "Daily forced sample of low-score alerts (M-16)"}'),
 ('schedule.qa_suppression_sample', '"0 0 6 * * 1"', 'cron', NULL, FALSE,
  '{"ar": "جدول عينة الكبح الأسبوعية (الاثنين)", "en": "Monday sample of pair-memory suppressions (phase 14)"}'),
 ('schedule.qa_grant_sweep', '"0 50 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مسح الإذن المؤقت المنتهي (احتياطي)", "en": "Expired QA grant backstop scan (fallback for the timers)"}'),
 ('schedule.hibernation_regret', '"0 45 4 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول وسم الندم في سجل التعليق", "en": "Daily regret marking of the hibernation ledger"}')
ON CONFLICT (key) DO NOTHING;

-- Classification (DATA-MODEL s.0 classes). Sealed columns are personal and encrypted (phase 02 personal seal, AAD = row id).
-- Every column of every new table has a row (the coverage test of phase 00 fails the build otherwise).
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'casework', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('authority_requests', 'internal', ARRAY['id', 'authority_kind', 'received_at', 'acknowledged_at', 'deadline_at', 'deadline_source',
                                           'clock_days', 'linked_case_ids', 'status', 'response_rfi_id', 'approved_by', 'approved_at',
                                           'disclosed_at', 'disclosed_items', 'tier', 'registered_by', 'timer_id', 'created_at',
                                           'updated_at', 'version']),
  ('authority_requests', 'secret-str', ARRAY['authority_ref']),
  ('authority_requests', 'personal', ARRAY['subject_party_ids', 'questions_enc']),
  ('rfis', 'internal', ARRAY['id', 'case_id', 'kind', 'direction', 'template_key', 'template_version', 'purpose_code', 'legal_basis',
                             'recipient_kind', 'recipient_ref', 'party_id', 'contact_allowed', 'authority_request_id', 'priority_class',
                             'tier', 'status', 'clock_unit', 'clock_days', 'deadline_at', 'extensions', 'timer_id',
                             'workflow_instance_id', 'proposer_id', 'released_by', 'released_at', 'answered_at', 'closed_at',
                             'created_at', 'updated_at', 'version']),
  ('rfis', 'personal', ARRAY['recipient_name_enc']),
  ('rfi_questions', 'internal', ARRAY['id', 'rfi_id', 'ordinal', 'question_key', 'answer_format', 'required', 'state', 'created_at']),
  ('rfi_answers', 'internal', ARRAY['id', 'rfi_id', 'question_id', 'answer_kind', 'source_kind', 'answered_by', 'answered_at',
                                    'corroboration', 'corroborated_by', 'corroborated_at', 'evidence_object_id', 'expires_at',
                                    'claim_state', 'supersedes', 'created_at']),
  ('rfi_answers', 'personal', ARRAY['value_enc', 'source_ref']),
  ('internal_reports', 'internal', ARRAY['id', 'reporter_kind', 'branch_id', 'subject_party_id', 'channel', 'received_at', 'received_by',
                                         'ack_due_at', 'timer_id', 'acknowledged_at', 'acknowledged_by', 'linked_case_id', 'status',
                                         'tier', 'created_at', 'version']),
  ('internal_reports', 'personal', ARRAY['reporter_id', 'body_enc']),
  ('watches', 'internal', ARRAY['id', 'kind', 'party_id', 'case_id', 'origin_decision_id', 'triggers', 'period_days', 'started_at',
                                'expires_at', 'review_due_at', 'timer_id', 'state', 'review_decision', 'reviewed_by', 'reviewed_at',
                                'trigger_count', 'last_trigger_at', 'last_trigger_key', 'created_at', 'updated_at', 'version']),
  ('watches', 'financial', ARRAY['envelope']),
  ('watches', 'personal', ARRAY['review_note_enc']),
  ('hibernations', 'internal', ARRAY['id', 'party_id', 'scenario_id', 'case_id', 'decision_id', 'period_days', 'created_at',
                                     'expires_at', 'created_by', 'timer_id', 'ended_at', 'ended_by', 'ended_reason', 'trigger_key',
                                     'version']),
  ('hibernations', 'financial', ARRAY['envelope']),
  ('hibernation_hits', 'internal', ARRAY['id', 'hibernation_id', 'alert_id', 'alert_created_at', 'hit_at', 'released_at',
                                         'release_reason', 'report_after']),
  ('hibernation_hits', 'financial', ARRAY['measures']),
  ('alarms', 'internal', ARRAY['id', 'kind', 'stage', 'subject_kind', 'subject_id', 'case_id', 'raised_at', 'supervisor_id',
                               'notified_at', 'acknowledged_at', 'acknowledged_by', 'closed_at', 'closed_by', 'closure_reason_code',
                               'state', 'payload', 'version']),
  ('network_snapshots', 'internal', ARRAY['id', 'case_id', 'seed_party_id', 'node_count', 'edge_count', 'object_id', 'edges_as_of',
                                          'created_by', 'created_at']),
  ('network_snapshots', 'personal', ARRAY['graph']),
  ('network_edges', 'internal', ARRAY['id', 'from_party', 'to_party', 'kind', 'hidden_by_default', 'weight', 'first_seen', 'last_seen',
                                      'built_batch_id', 'retired_at']),
  ('network_edges', 'personal', ARRAY['basis']),
  ('qa_samples', 'internal', ARRAY['id', 'draw_id', 'population', 'source_kind', 'source_id', 'source_at', 'case_id', 'stratum',
                                   'draw_mode', 'draw_seed', 'population_size', 'sample_size', 'reviewer_id', 'drawn_by', 'drawn_at',
                                   'due_at', 'state', 'grant_id', 'is_calibration', 'golden_outcome', 'version']),
  ('qa_reviews', 'internal', ARRAY['id', 'sample_id', 'reviewer_id', 'rubric_version', 'scores', 'overall', 'completed_at', 'created_at']),
  ('qa_reviews', 'personal', ARRAY['comment_enc']),
  ('qa_findings', 'internal', ARRAY['id', 'review_id', 'criterion', 'severity', 'target_kind', 'target_id', 'coaching_state',
                                    'coaching_due_at', 're_review_sample_ids', 'created_at'])
) AS v(t, cls, cols)
CROSS JOIN LATERAL unnest(v.cols) AS c
ON CONFLICT DO NOTHING;

-- The sealed columns (phase 02 personal seal).
UPDATE admin.column_classification SET encrypted = TRUE
 WHERE (schema_name, table_name, column_name) IN (
   ('casework', 'authority_requests', 'questions_enc'),
   ('casework', 'rfis', 'recipient_name_enc'),
   ('casework', 'rfi_answers', 'value_enc'),
   ('casework', 'internal_reports', 'body_enc'),
   ('casework', 'watches', 'review_note_enc'),
   ('casework', 'qa_reviews', 'comment_enc'));

-- Retention (phase 00 clocks; R6 cases, their trail and the decision record, ARCHITECTURE s.17; R5 alerts and dispositions;
-- R3 master data copy). Children use the child's own timestamp with the closure clock, as phase 18 does.
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('casework', 'authority_requests',  'R6', 'age',     'received_at',  'Authority register: seven years from receipt; sealed questions (Open doubts 16)'),
 ('casework', 'rfis',                'R6', 'closure', 'created_at',   'RFIs: follow the case'),
 ('casework', 'rfi_questions',       'R6', 'closure', 'created_at',   'RFI questions: follow the case'),
 ('casework', 'rfi_answers',         'R6', 'closure', 'answered_at',  'RFI claims: follow the case'),
 ('casework', 'internal_reports',    'R6', 'closure', 'received_at',  'Internal reports: follow the linked case or the register'),
 ('casework', 'watches',             'R6', 'closure', 'started_at',   'Watches: follow the case that started them'),
 ('casework', 'hibernations',        'R6', 'closure', 'created_at',   'Hibernations: follow the case'),
 ('casework', 'hibernation_hits',    'R6', 'closure', 'hit_at',       'Hibernation ledger: follows the case'),
 ('casework', 'alarms',              'R5', 'age',     'raised_at',    'Alarms about alerts and clocks: seven years from the alarm'),
 ('casework', 'network_snapshots',   'R6', 'closure', 'created_at',   'Network snapshots: follow the network case'),
 ('casework', 'network_edges',       'R3', 'age',     'last_seen',    'Canonical link copy: seven years after the last sighting'),
 ('casework', 'qa_samples',          'R6', 'closure', 'drawn_at',     'QA samples: follow the case or the alert they reviewed'),
 ('casework', 'qa_reviews',          'R6', 'closure', 'completed_at', 'QA reviews: follow the sample'),
 ('casework', 'qa_findings',         'R6', 'closure', 'created_at',   'QA findings: follow the sample')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

## 2. DDD Implementation (Rust)

Crate placement follows phase 18: the domain in `domain::casework` (akr-core, no IO; the RFI, watch, hibernation, alarm and QA rules are pure functions of their inputs, section 3), the repositories in `infrastructure::casework`, the services in `services::casework` (`rfi`, `watch`, `hibernation`, `network`, `qa`, `alarm`, `registers`), the routers in `api::routes`. The sealed fields (`value_enc`, `recipient_name_enc`, `body_enc`, `questions_enc`, `review_note_enc`, `comment_enc`) are `Sealed<T>` of phase 18: the plaintext exists only inside the service, `Sealed<T>` does not derive `Serialize`, and every response is an explicit view struct. The RFI view for the desk role has no case id, no family, no priority and no tier field (section 3.14).

**Domain entities** (derives `Debug, Serialize, Deserialize, FromRow, ToSchema`, except `Sealed<T>`):
- `Rfi { id, case_id, kind: RfiKind, direction: Option<RfiDirection>, template_key, template_version, purpose_code, legal_basis, recipient_kind: RecipientKind, recipient_ref: Option<Uuid>, recipient_name: Option<Sealed<String>>, party_id, contact_allowed, authority_request_id: Option<Uuid>, priority_class: PriorityClass, tier: Tier, status: RfiStatus, clock_unit: Option<RfiClockUnit>, clock_days: Option<i32>, deadline_at, extensions: i16, timer_id, workflow_instance_id, proposer_id, released_by, released_at, answered_at, closed_at, created_at, updated_at, version: i32 }`
- `RfiQuestion { id, rfi_id, ordinal, question_key, answer_format: AnswerFormat, required, state: QuestionState, created_at }`
- `RfiAnswer { id, rfi_id, question_id, answer_kind: AnswerKind, value: Sealed<String>, source_kind: SourceKind, source_ref: Option<String>, answered_by, answered_at, corroboration: Corroboration, corroborated_by, corroborated_at, evidence_object_id, expires_at, claim_state: ClaimState, supersedes, created_at }`
- `AuthorityRequest { id, authority_kind: AuthorityKind, authority_ref: String, received_at, acknowledged_at, deadline_at, deadline_source: DeadlineSource, clock_days, subject_party_ids: Vec<Uuid>, linked_case_ids: Vec<Uuid>, questions: Sealed<String>, status: AuthorityStatus, response_rfi_id, approved_by, approved_at, disclosed_at, disclosed_items: Vec<DisclosedItem>, registered_by, timer_id, created_at, updated_at, version }`
- `InternalReport { id, reporter_kind: ReporterKind, reporter_id: Uuid, branch_id, subject_party_id, channel: ReportChannel, body: Sealed<String>, received_at, received_by, ack_due_at, timer_id, acknowledged_at, acknowledged_by, linked_case_id, status: ReportStatus, created_at, version }`
- `Watch { id, kind: WatchKind, party_id, case_id, origin_decision_id, envelope: Measures, triggers: Vec<WatchTrigger>, period_days, started_at, expires_at, review_due_at, timer_id, state: WatchState, review_decision: Option<WatchReview>, reviewed_by, reviewed_at, review_note: Option<Sealed<String>>, trigger_count, last_trigger_at, last_trigger_key, created_at, updated_at, version }`
- `Hibernation { id, party_id, scenario_id, case_id, decision_id, envelope: HibernationEnvelope, period_days, created_at, expires_at, created_by, timer_id, ended_at, ended_by, ended_reason: Option<HibernationEnd>, trigger_key: Option<HibernationTrigger>, version }`
- `HibernationHit { id, hibernation_id, alert_id, alert_created_at, hit_at, measures: Measures, released_at, release_reason, report_after }`
- `Alarm { id, kind: AlarmKind, stage: Option<AlarmStage>, subject_kind: AlarmSubject, subject_id, case_id, raised_at, supervisor_id, notified_at, acknowledged_at, acknowledged_by, closed_at, closed_by, closure_reason_code, state: AlarmState, payload: serde_json::Map<String, Value>, version }`
- `NetworkSnapshot { id, case_id, seed_party_id, node_count, edge_count, graph: serde_json::Value, object_id, edges_as_of, created_by, created_at }`
- `NetworkEdge { id, from_party, to_party, kind: EdgeKind, hidden_by_default, weight, basis: serde_json::Value, first_seen, last_seen, built_batch_id, retired_at }`
- `QaSample { id, draw_id, population: SamplePopulation, source_kind: SampleSource, source_id, source_at, case_id, stratum, draw_mode: DrawMode, draw_seed, population_size, sample_size, reviewer_id, drawn_by, drawn_at, due_at, state: SampleState, grant_id, is_calibration, golden_outcome: Option<Level>, version }`
- `QaReview { id, sample_id, reviewer_id, rubric_version, scores: BTreeMap<Criterion, Level>, overall: Level, comment: Option<Sealed<String>>, completed_at, created_at }`
- `QaFinding { id, review_id, criterion: Criterion, severity: FindingSeverity, target_kind: TargetKind, target_id, coaching_state: CoachingState, coaching_due_at, re_review_sample_ids: Vec<Uuid>, created_at }`
- `Measures { n: i64, total: Decimal }` (the two measures of the DSL A.7 envelope; `total` is the sum of the amounts of the alert in the bank's functional currency, as the alert's `measures` stores it)

**Enums** (closed in code; the SQL CHECK lists are the same values): `RfiKind { BranchInfo => "branch_info", RmInfo => "rm_info", UnitInfo => "unit_info", CustomerOutreach => "customer_outreach", Correspondent => "correspondent", AuthorityResponse => "authority_response" }` (six kinds: research 28 section 9.2 has seven rows, of which `correspondent_in` and `correspondent_out` are one kind with a direction; the sixth kind `authority_response` is the response to an authority request, whose register is `casework.authority_requests`); `RfiDirection { In, Out }`; `RfiStatus { Draft, PendingRelease, Released, Answered, Closed, Withdrawn, Rejected }`; `RfiClockUnit { RfiResponse => "rfi.response", Internal => "internal", Outreach => "outreach", AuthorityStated => "authority_stated", AuthorityDefault => "authority_default" }`; `RecipientKind { Liaison, RelationshipManager, Unit, Customer, Correspondent, Authority }`; `AnswerFormat { YesNoUnknown, Value, Document, FreeText }`; `AnswerKind { Yes, No, Unknown, Value, Document, FreeText }`; `SourceKind { BranchOfficer, RelationshipManager, Unit, CustomerViaBranch, Correspondent, Authority, Replica }`; `Corroboration { None, Partly, Documented }`; `ClaimState { Current, Superseded, Expired, Withdrawn }`; `AuthorityKind { Fiu, Court, Prosecutor, Cbi, Other }`; `DeadlineSource { Stated, BankDefault }`; `AuthorityStatus { Received, Drafting, Approved, Answered, Closed }`; `ReporterKind { Staff, Liaison }`; `ReportChannel { BranchWindow, LiaisonMonthly, Direct, Letter }`; `ReportStatus { Received, Acknowledged, Linked, Closed }`; `WatchKind { Utr }`; `WatchState { Open, UnderReview, Closed, Upgraded }`; `WatchReview { Extend, Close, UpgradeStr }`; `WatchTrigger { ListHit => "list_hit", AuthorityRequest => "authority_request", ProfileChange => "profile_change", NewAlert => "new_alert" }`; `HibernationEnd { Expired, EnvelopeExceeded, Trigger, EndedByPerson }`; `HibernationTrigger { AdverseMedia => "adverse_media", ListChange => "list_change", AuthorityRequest => "authority_request", ProfileChange => "profile_change" }`; `AlarmKind { TouchBreach, WorkItemBreach, ChainBroken, ConflictDeclared, FlatLine, QaCritical }`; `AlarmStage { P100 = 100, P150 = 150 }`; `AlarmSubject { Alert, WorkItem, Case, Scenario, QaReview }`; `AlarmState { Open, Acknowledged, Closed }`; `EdgeKind { SharedContact, SharedDevice, Transfer, Ownership, Kinship }`; `SamplePopulation { ClosedAlerts, ClosedCases, ShelvedDecisions, MlroDecisions, HibernationHits, ScreeningSuppressions, LowScoreForced, Calibration, TargetedRepeat }`; `SampleSource { Alert, Case, Decision, HibernationHit, PairMemorySuppression }`; `DrawMode { Stratified, Random, Targeted }`; `SampleState { Open, Reviewed, Void }`; `Criterion { Evidence, Reason, Escalation, Corroboration, Timeliness, Confidentiality, Narrative, Authority }` (the eight criteria of research 28 section 10.3); `Level { Pass, PassWithComment, Minor, Major, Critical }`; `FindingSeverity { Minor, Major, Critical }`; `TargetKind { Alert, Case, Decision, Rfi, Hibernation }`; `CoachingState { None, Open, Done }`.

**Create, command and view DTOs** (`Deserialize, ToSchema`). Every change is a named command:
- `DraftRfi { kind: RfiKind, direction: Option<RfiDirection>, template_key: String (1 to 64), purpose_code: String, recipient_kind: RecipientKind, recipient_ref: Option<Uuid>, recipient_name: Option<String> (1 to 120, sealed), party_id: Uuid, contact_allowed: bool, authority_request_id: Option<Uuid> }`. The questions are not in the request: they are the question keys of the template in the pinned pack (section 3.1 step 3). There is no free-text question.
- `SubmitRfi {}` and `ExtendRfi { reason: String (5 to 200 characters, scope `reason_text`) }`
- `ReleaseRfi { task_id: Uuid, action: DecisionAction (`approve`, `reject`), comment: Option<String> }` (`comment` required for `reject`)
- `RecordAnswers { answers: Vec<AnswerInput { question_id, answer_kind, value: String (1 to 2000, sealed), source_kind, source_ref: Option<String> (1 to 120), corroboration: Corroboration, evidence_object_id: Option<Uuid>, answered_at: Option<DateTime<Utc>>, supersedes: Option<Uuid>, corroborates: Option<Uuid> }> (1 to 60 entries); `corroborates` names a current claim of this RFI whose corroboration is raised (section 3.3 step 3), `supersedes` names a claim this one corrects (step 2)`}`
- `CreateAuthorityRequest { authority_kind, authority_ref: String (1 to 80), received_at, stated_deadline: Option<DateTime<Utc>>, subject_party_ids: Vec<Uuid> (1 to 500), linked_case_ids: Vec<Uuid>, questions: String (1 to 5000, sealed) }`
- `CreateInternalReport { reporter_kind, reporter_id: Uuid, branch_id: Option<Uuid>, subject_party_id: Option<Uuid>, channel, body: String (1 to 4000, sealed), received_at }`; `AcknowledgeInternalReport {}`
- `StartWatch { decision_id: Uuid }`; `ReviewWatch { review: WatchReview, note: String (1 to 1000, sealed) }`
- `CreateHibernation { party_id, scenario_id, decision_id, period_days: i32 (1 to 180) }`; `EndHibernation { reason: String (5 to 200) }`
- `OpenNetworkCase { }` (the seed is the path party; the depth is one hop, section 3.7)
- `DrawQaSample { population: SamplePopulation, mode: DrawMode, strata_by: Vec<StratumKey> (at most 3), n: Option<i32>, confidence: Option<Decimal>, defect_rate: Option<Decimal>, since: Option<DateTime<Utc>>, until: Option<DateTime<Utc>>, reviewer_id: Option<Uuid> }`, where `reviewer_id` is only for the MLRO-decision population (a CH or IA reviewer, section 3.8)
- `ReviewQaSample { scores: BTreeMap<Criterion, Level> (all eight), overall: Level, comment: Option<String> (sealed, 1 to 2000), findings: Vec<FindingInput { criterion, severity, target_kind, target_id }> (0 to 20) }`
- `BulkReopen { analyst_id: Uuid, finding_id: Uuid, reason_code: String, case_ids: Vec<Uuid> (1 to `qa.bulk_reopen_max`) }`
- `CloseAlarm { reason_code: String }` (with `POST /alarms/{id}/acknowledge` and the field `close: true`)
- Views: `RfiDeskView { id, kind, status, deadline_at, clock_days, extensions, recipient_kind, questions: Vec<QuestionView>, answers: Vec<AnswerView>, overdue: bool }` (no case, family, priority or tier field); `RfiTeamView` (the case team's view: adds `case_id`, `priority_class`, `contact_allowed`, `released_by`); `RfiKpis { median_days, p90_days, share_by_deadline, follow_up_rounds, top_question_keys: Vec<(String, i64)>, missed_by_monitoring: i64 }` (M-46; `missed_by_monitoring` counts RFIs whose party had no alert in the prior 180 days); `AnswerView { id, question_key, answer_kind, value: Option<String>, source_kind, source_ref, answered_at, corroboration, expires_at, claim_state }` (`value` only to the desk and the team); `AuthorityView`; `InternalReportView { ..., reporter_id: Option<Uuid> }` (`reporter_id` is `None` except for M); `WatchView`; `HibernationView { ..., hits: i64, released_hits: i64, regret_share: Option<Decimal> }`; `GraphView { nodes: Vec<GraphNode { party_id, display_name: Option<String>, tier_label: Option<String> }>, edges: Vec<GraphEdge { from_party, to_party, kind, weight, hidden: bool }>, summary: String }`; `NetworkCaseView { parent_case_id, children: Vec<Uuid>, snapshot_id }`; `QaSampleView` (no reviewer for the reviewer's own row: a reviewer sees their own items, M sees all); `QaItemView { sample_id, source_kind, decision: Option<DecisionSummary>, alert: Option<AlertSummary>, reason_codes, evidence_labels: Vec<String>, opened_at }` (no party name, no account, no text except the decision text for an analyst network reader and, from phase 28 on, the verified output of an `assistant_draft` item, which may name parties of its case, section 7+); `QaRateView { reviewer_id, reviews, share_major_or_critical, wilson_low, wilson_high }`; `CalibrationView { golden_items, agreement: Vec<ReviewerAgreement { reviewer_id, matches, total, wilson_low, wilson_high }> }`; `AlarmView` (`payload` as stored: ids, codes and counts).

**Repository traits** (domain layer; each method takes `&mut PgConnection` or `&mut Tx` as its name says; `RETURNING` on every insert):
- `RfiRepository`: `insert(tx, NewRfi) -> Rfi`; `get(conn, id)`; `get_for_update(tx, id)`; `by_case(conn, case_id)`; `desk(conn, filter, page)` (the RFI desk query: `visibility_sql` of section 3.2 and the status filter); `set_status(tx, id, status, actor, at)`; `set_submitted(tx, id, instance_id, clock_unit, clock_days, deadline_at, timer_id)`; `set_released(tx, id, by, at)`; `set_extension(tx, id, extensions, deadline_at, timer_id)`; `kpi_rows(conn, window)`; `count_missed_outreach(conn, rfi_id) -> i64` (the `prior_failures` of 7.9: the party's `customer_outreach` RFIs created before this one whose `deadline_at` has passed with `answered_at` NULL).
- `RfiQuestionRepository`: `insert_many(tx, rfi_id, questions)`; `list(conn, rfi_id)`; `set_state(tx, id, state)`.
- `RfiAnswerRepository`: `insert(tx, NewAnswer) -> RfiAnswer`; `current_for_rfi(conn, rfi_id)`; `current_for_case(conn, case_id)`; `set_corroboration(tx, id, corroboration, by, at, evidence_object_id)`; `set_claim_state(tx, id, state)`; `expire_due(tx, now, limit) -> u64`.
- `AuthorityRepository`: `insert`; `get_for_update`; `list(conn, filter, page)`; `set_status`; `set_approved(tx, id, by, at)`; `set_disclosed(tx, id, at, items)`; `set_ack(tx, id, at)`; `set_response(tx, id, rfi_id)`; `for_party(conn, party_id, status)` (the watch and hibernation triggers, section 3.5).
- `InternalReportRepository`: `insert`; `get_for_update`; `list(conn, filter, page)`; `set_ack(tx, id, by, at)`; `set_link(tx, id, case_id)`.
- `WatchRepository`: `insert(tx, NewWatch)`; `get_for_update(tx, id)`; `open_for_party(conn, party_id)`; `list(conn, filter, page)`; `record_trigger(tx, id, key, at)`; `set_review(tx, id, review, note, by, at, expires_at, review_due_at)`; `due_expiry(conn, now, limit)`.
- `HibernationRepository`: `insert(tx, NewHibernation)`; `active_for(conn, party_id, scenario_id) -> Option<Hibernation>` (no lock, section 3.6); `get_for_update(tx, id)`; `end(tx, id, reason, by, at)`; `active_for_party(conn, party_id)`; `ended_expired(conn, now, limit)`; `list(conn, filter, page)`.
- `HibernationHitRepository`: `insert(tx, NewHit)`; `open_for_hibernation(conn, id)`; `open_for_alert(conn, alert_id, alert_created_at)`; `release(tx, id, reason, at)`; `mark_report_after(tx, hibernation_id, party_id, after, window)`; `counts(conn, hibernation_id) -> (i64, i64)`.
- `AlarmRepository`: `insert_live(tx, NewAlarm) -> Option<Alarm>` (the unique index of section 1.1 makes a repeat return `None`); `list_visible(conn, principal, filter, page)`; `get_for_update(tx, id, principal) -> Option<Alarm>` (one alarm under a row lock and the predicate of 7.11; `None` when the predicate hides it, which answers 404; acknowledge and close use it); `mark_notified(tx, id, at)`; `acknowledge(tx, id, by, at)`; `close(tx, id, by, at, reason_code)`.
- `NetworkEdgeRepository`: `upsert_live(tx, NewEdge)`; `retire_missing(tx, from_party, batch_id, at) -> u64`; `neighbours(conn, party_id) -> Vec<NetworkEdge>` (live edges only); `count_live(conn)`.
- `NetworkSnapshotRepository`: `insert(tx, NewSnapshot)`; `get(conn, id)`.
- `QaSampleRepository`: `insert_many(tx, rows)`; `get_for_update(tx, id)`; `list(conn, principal, filter, page)`; `population(conn, population, window, strata) -> Vec<PopulationRow>` (the reads of section 3.8); `open_for_reviewer(conn, reviewer_id)`; `set_state(tx, id, state)`; `set_reviewer(tx, id, reviewer_id)`; `set_grant(tx, id, grant_id)`; `due_grants(conn, now, limit)`.
- `QaReviewRepository`: `insert(tx, NewReview)`; `rates(conn, window) -> Vec<RateRow>`; `calibration(conn) -> Vec<AgreementRow>`.
- `QaFindingRepository`: `insert_many(tx, rows)`; `set_coaching(tx, id, state)`; `open_coaching_due(conn, now)`.
- `CanonNetworkReader` (read only, SELECT on canon, phase 10): `family_links(conn, batch_parties)` (equal `canon.party.family_no_hk`, current rows); `contact_links(conn, batch_parties)` (equal `canon.party_contact.value_hk` of types `mobile`, `landline`, `address`, `superseded_at IS NULL`); `ownership_links(conn, batch_parties)` (`canon.party_relationship` of types `owner`, `controller`, `director`, `signatory`, `attorney` with `recorded_to IS NULL`, `evidence <> '{}'`); `account_owners(conn, account_ids)` (`canon.account_party_role` with `role = 'owner'`, `recorded_to IS NULL`); `payment_pairs(conn, since, min_confidence)` (`canon.payment_link` rows with `confidence >= casework.network_transfer_min_confidence`, joined to `canon.txn` on `(txn_id, booking_date)` and to `canon.account_party_role` for both sides).

**Pg implementations**: `RETURNING` on every insert; the desk and team queries join `casework.cases` for the team and the tier (never a copy of the tier); the graph reader reads `canon.party_name` where `superseded_at IS NULL` (phase 10, not bi-temporal); `canon.txn` reads carry `booking_date` (the partition key).

**utoipa schemas**: every DTO and view above, with `ToSchema`; `Problem` (API section 6 envelope) is shared.

**Route table** (scopes from API.md section 2; "Owned" marks a row of API.md 3.12 that this phase builds (19 rows); "New" marks a row added to API.md 3.12 by this phase, marked "phase 19, added"; the markers are those of API.md section 3):

| Method | Path | Scope | Description (cedar action) |
|-|-|-|-|
| POST | `/api/v1/cases/{id}/hibernations` | `case.decide` | Hibernate a scenario for a party after a no-report decision of this case; `If-Match` (case version); `Idempotency-Key`; **SU** (class `decision`) (Owned; `hibernation.create`) |
| GET | `/api/v1/hibernations` | `queue.work` | Hibernation ledger: state, expiry, hits, released hits and the regret share; filters `case_id`, `state`; cursor (Owned; `hibernation.read`) |
| POST | `/api/v1/hibernations/{id}/end` | `case.decide` | End a hibernation by a person with a reason; `If-Match`; `Idempotency-Key` (New; `hibernation.end`) |
| GET | `/api/v1/watches` | `queue.work` | UTR watches visible to the caller, with triggers, expiry and review state; cursor (Owned; `watch.list`) |
| POST | `/api/v1/cases/{id}/watches` | `queue.work` | Start a UTR watch by hand on a UTR decision of the case (a decision starts one itself); `Idempotency-Key` (New; `watch.write`) |
| POST | `/api/v1/watches/{id}/review` | `case.decide` | Review at expiry: `extend`, `close` or `upgrade_str`; **SU** (class `decision`); `If-Match`; `Idempotency-Key` (New; `watch.review`) |
| GET | `/api/v1/parties/{id}/graph` | `queue.work` | Network around the party: visible nodes and edges, the summary; kinship only with `show_kinship=true`, a purpose code and **SU** `party.open`, and logged (Owned; `network.read`) |
| POST | `/api/v1/parties/{id}/network-cases` | `case.decide` | Open a network case: parent (family `network`), subgraph snapshot, one child per visible subject; `Idempotency-Key` (New; `network.case.open`) |
| POST | `/api/v1/cases/{id}/rfis` | `queue.work` | Draft an RFI of one of the six kinds from a template key of the pinned pack; `Idempotency-Key` (Owned; `rfi.draft`) |
| GET | `/api/v1/cases/{id}/rfis` | `queue.work` | The case's RFIs for the case team (New; `rfi.read.case`) |
| POST | `/api/v1/rfis/{id}/submit` | `queue.work` | The drafter submits for release: starts `rfi_release_internal` or `rfi_release_external`; `If-Match` (RFI version); `Idempotency-Key` (New; `rfi.draft`) |
| GET | `/api/v1/rfis` | `rfi.desk` | RFI desk: pending and released RFIs visible to the caller, with deadlines and the `kpis` block (M-46); cursor (Owned; `rfi.list`) |
| GET | `/api/v1/rfis/{id}` | `rfi.desk` | RFI with questions, deadline clock and claims; a masked 404 for a suspicion RFI that the caller may not see (Owned; `rfi.read`) |
| POST | `/api/v1/rfis/{id}/release` | `rfi.desk` | The second signature: acts on the RFI's release task; **SU** (class `approval`); **4E**; `If-Match` (instance version); `Idempotency-Key` (Owned; `rfi.release`, `task.approve`, `task.reject`) |
| POST | `/api/v1/rfis/{id}/extend` | `rfi.desk` | One extension by `casework.rfi_extension_days` working days, two at most; `If-Match`; `Idempotency-Key` (New; `rfi.extend`) |
| POST | `/api/v1/rfis/{id}/answers` | `rfi.desk` | Record answers as claims: source, corroboration, expiry; `If-Match` (RFI version); `Idempotency-Key` (Owned; `rfi.answer`) |
| GET | `/api/v1/authority-requests` | `case.decide` | Authority register (suspicion tier); cursor (Owned; `authority.read`) |
| POST | `/api/v1/authority-requests` | `case.decide` | Register a request with its deadline (stated, or the bank default); emits `authority_request.recorded` (TR-17); **SU** (class `decision`); `Idempotency-Key` (Owned; `authority.register`) |
| GET | `/api/v1/authority-requests/{id}` | `case.decide` | Detail: linked cases, subject customers, the response RFI and the disclosure fields (Owned; `authority.read`) |
| GET | `/api/v1/internal-reports` | `case.decide` | Internal reports register, the reporter shown to M only; cursor (Owned; `internal_report.read`) |
| POST | `/api/v1/internal-reports` | `case.decide` | Register a report received outside the branch window (letter, direct, liaison monthly); `Idempotency-Key` (New; `internal_report.write`) |
| POST | `/api/v1/internal-reports/{id}/acknowledge` | `case.decide` | Acknowledge receipt; stops the clock; `If-Match`; `Idempotency-Key` (Owned; `internal_report.acknowledge`) |
| GET | `/api/v1/qa/samples` | `qa.review` | Sample register: the caller's own items (QA) or all items (M); cursor (Owned; `qa.sample.read`) |
| POST | `/api/v1/qa/samples` | `qa.review` | Draw a sample (population, mode, strata, n or confidence and rate); reviewers are assigned and never a closer; `Idempotency-Key` (Owned; `qa.sample.draw`) |
| GET | `/api/v1/qa/samples/{id}/item` | `qa.review` | The sampled item for its reviewer; the time-boxed grant is checked; **SU** (action `qa.item.open`, class `record_open`) on a suspicion item (New; `qa.item.read`) |
| POST | `/api/v1/qa/samples/{id}/review` | `qa.review` | Score the eight criteria and record findings; only the sample's reviewer, while the sample is open; `If-Match` (sample version); `Idempotency-Key` (Owned; `qa.review.write`) |
| GET | `/api/v1/qa/rates` | `qa.review` | Reviewer rates with Wilson 95 per cent intervals, shown from `qa.min_reviewer_items` (New; `qa.rates.read`) |
| GET | `/api/v1/qa/calibration` | `qa.review` | Golden items and each calibrator's agreement, with Wilson intervals (Owned; `qa.calibration.read`) |
| POST | `/api/v1/qa/bulk-reopen` | `qa.review` | Reopen up to `qa.bulk_reopen_max` closed cases of one analyst after a critical finding; M only; **SU** (class `decision`); `Idempotency-Key` (Owned; `qa.bulk_reopen`) |
| GET | `/api/v1/qa/findings/{id}/reopen-candidates` | `qa.review` | Cases the closer of a critical finding closed in its 30-day window, for the bulk reopen dialog; at most 100 rows; M only (New, section 7+; `qa.bulk_reopen`) |
| GET | `/api/v1/alarms` | `case.decide`, `audit.read` | Alarms the caller may see, by kind; the supervisor's first list sets `notified_at` (New; `alarm.list`) |
| POST | `/api/v1/alarms/{id}/acknowledge` | `case.decide`, `audit.read` | Acknowledge an alarm; with `close: true` and a closure reason code, close it; `If-Match`; `Idempotency-Key` (New; `alarm.acknowledge`) |

Count: 32 product routes (19 owned rows of API.md 3.12, 13 added rows; the thirteenth is the reopen-candidates route of section 7.14, added after DR-157 counted thirty-one). No dev route. The route `POST /cases/{id}/rfis` of API.md 3.12 is drafting only; the release is `POST /rfis/{id}/release`.

**Routers**: `pub fn router() -> Router<AppState>` in `api::routes::hibernations` (hibernations, watches, the case watch start), `api::routes::rfis` (drafting, submit, desk, release, extend, answers, case RFIs), `api::routes::requests` (authority requests, internal reports), `api::routes::qa` (samples, item, review, rates, calibration, bulk reopen), `api::routes::alarms`, and `api::routes::parties` of phase 18 extended by the graph and the network-case routes (the phase 18 party router is extended, not replaced).

## 3. Business Logic

Every mutation below appends a case event through `CaseChain` (phase 17 section 3.7) under the case row lock when it concerns an open or in-review case (a closed case is sealed, phase 18 DR-139: the follow-up records of a closed case, its watch, its hibernation ledger and its QA grant, write their audit events and no case event, so the seal hash stays the hash of the chain at closure; lead review), writes the audit event of section 3.12 in the same transaction, and bumps the version of the row it changed. Reads of a suspicion-tier record write an access event (phase 02 `AccessScope`) before the body is sent. Sensitive writes carry `Idempotency-Key` (400 `E_IDEMPOTENCY_REQUIRED` without it); versioned writes carry `If-Match` (428 without it, 409 `E_VERSION_CONFLICT` with the current version otherwise). The lock order is section 3.15.

### 3.1 RFI drafting (`RfiService::draft`; M-42, M-43, M-44, DR-148, DR-149)

**Route** (`POST /cases/{id}/rfis`; scope `queue.work`; Cedar `rfi.draft` on Case; A1 and A2 only, since M decides and does not draft). Request `DraftRfi` (section 2).

1. The drafter is on the case team (`assignees`, the pool or the access list, phase 18 section 3.2) or the case is not visible to them (masked 404). The case is `open` or `in_review` and not sealed (409 `E_CASE_SEALED` otherwise).
2. The party is a party of the case (`casework.case_parties`, phase 17) or `party_id` is the case subject; else 422 `E_VALIDATION` with sub-code `not_on_case`.
3. The template is the one of the pack pinned to the case (`casework.policy_pack_version`, phase 17): `template_key` exists in its RFI templates (sub-code `template_unknown`); the template's kind equals `kind`; `purpose_code` is in the pack's purpose list (sub-code `purpose_not_in_pack`); `legal_basis` and the question list (`question_key`, `answer_format`, `required`, in the template's order) are copied from the pack. There is no free-text question: a question is a key of the pack, so the words of a question are the bank's templated words (DR-149).
4. Recipient rules by kind (`recipient_kind` must match the kind: `branch_info` to `liaison`, `rm_info` to `relationship_manager`, `unit_info` to `unit`, `customer_outreach` to `customer`, `correspondent` to `correspondent`, `authority_response` to `authority`):
   - `branch_info`: `recipient_ref` is an active `canon.staff` row whose `role` is the liaison code of the bank (Open doubts 4) and whose `branch_id` is the branch of the subject's account; a branch manager is refused (sub-code `recipient_not_liaison`). The RFI is never addressed to the branch manager (M-43).
   - `rm_info`: refused on a case whose tier is not `normal` (sub-code `tier_forbids_rm`); needs `contact_allowed = TRUE` (sub-code `contact_not_allowed`); the release is external (section 3.2).
   - `customer_outreach`: needs `contact_allowed = TRUE` (sub-code `contact_not_allowed`); the release is an external release (section 3.2).
   - `correspondent`: `direction` is required (`in` or `out`); the release is external.
   - `authority_response`: `authority_request_id` names a registered request whose status is `received` or `drafting` (sub-code `not_on_case` when the request links no case of this case); the request moves to `drafting` (section 3.4).
   - `contact_allowed` (the tipping-off flag; lead review): the drafter may set it `TRUE` only on a case whose tier is `normal` (else 422 `contact_not_allowed`); `rm_info` and `customer_outreach` require it, and the other kinds keep it `FALSE`. An RFI with `contact_allowed = TRUE` always takes the external release, so the MLRO's second signature confirms that asking the customer or the relationship manager tips no one off (DR-149).
5. Text scan: `recipient_name` (when given) and the response of the template are scanned with `PolicyPackService::scan(scope = "rfi", text)` (phase 09 section 3.5). A hit is 422 `E_VALIDATION` with the span and the field, never the term.
6. Insert `casework.rfis` (status `draft`, `priority_class` and `tier` copied from the case as they are now, `proposer_id` = actor, `template_version` = the pinned pack version) and one `casework.rfi_questions` row per template question (`ordinal` 1 to n).
7. Chain `case.rfi_drafted {rfi_id, kind}`; audit `rfi.drafted`. Response: `RfiTeamView`. The same `Idempotency-Key` replays the response.

### 3.2 Submission, the second signature and the release workflow (`RfiService::submit`, `RfiService::release`, `RfiWorkflowSubject`, `ReleaseRfiEffect`; M-42, M-44, DR-148)

**Shipped definitions** (new in phase 19; the DSL E.4 table has no RFI definition, so these two rows are added to DSL E.4 by a `cr_workflow` change after their seed, Open doubts 2):

| Key | Used for | Steps (summary) |
|-|-|-|
| `rfi_release_internal` | Branch and unit requests of a normal or restricted case, contact not allowed | `approve` (assignee `{kind: "role", role: "RFI", scope: "bank"}`, not the maker, step-up) -> `release_rfi` |
| `rfi_release_external` | RM requests, customer outreach, correspondent requests, authority responses, every RFI with `contact_allowed`, and every RFI of a suspicion case | `approve` (assignee `{kind: "named_officer", officer: "mlro"}`, not the maker, step-up) -> `release_rfi` |

Both definitions carry `classification_default: "restricted"` and timeouts `remind` (one working day before the deadline) and `escalate` to the MLRO (never `auto_*`, DSL E.2). `release_rfi` is a new terminal effect (`TerminalEffect`, kind `release_rfi`, resource kinds `["rfi"]`, phase 04 section 2, the `TerminalEffect` trait; section 4.5 item 1).

**`RfiService::submit`** (`POST /rfis/{id}/submit`; Cedar `rfi.submit` on Rfi; `If-Match` = RFI version):
1. Lock the case row, then the RFI row (lock order, section 3.15). Status must be `draft` (409 `E_NOT_ALLOWED_IN_STATE`).
2. Definition: `rfi_release_external` for kinds `rm_info`, `customer_outreach`, `correspondent`, `authority_response`, for any RFI with `contact_allowed = TRUE` and for any RFI whose tier is `suspicion`; `rfi_release_internal` otherwise.
3. Clock (`clock_unit`, `clock_days`, `deadline_at`), with `BusinessClock::add(now, n, BusinessUnit::WorkingDays)` (phase 09 section 3.3) unless stated:
   - `branch_info`, `rm_info`, `unit_info`: `internal`, `casework.rfi_internal_days` (2 working days in the seed).
   - `customer_outreach`: `outreach`, `casework.rfi_outreach_days` (5 working days).
   - `correspondent`: `rfi.response`, the pack clock (10 working days by default, bounds 5 to 20, phase 09 section 3.5).
   - `authority_response`: `authority_stated` when the request states a deadline (`deadline_at` of the request, `clock_days` NULL), else `authority_default` with `casework.authority_default_days`.
4. `WorkflowEngine::start(tx, StartSpec { definition_key, resource: ("rfi", rfi_id), purpose: Decision, maker: actor })` (phase 04 section 3.4). The instance and its single `approve` task are created in this transaction; the RFI gets `workflow_instance_id` and status `pending_release`.
5. Timers `rfi_clock` (owner `rfi`, calendar = bank calendar): correspondent requests at 50 and 80 percent of the window and at the deadline (payload `{stage: "reminder_50" | "reminder_80" | "due"}`); branch, RM and unit requests at one working day before the deadline and at the deadline; authority responses at the deadline. The handler (section 4.1) re-reads the RFI first.
6. Chain `case.rfi_submitted`; audit `rfi.submitted {rfi_id, definition_key, deadline_at}`.

**`RfiService::release`** (`POST /rfis/{id}/release`; scope `rfi.desk`; Cedar `rfi.release` on Rfi, and the engine's `task.approve` or `task.reject` on the Task, phase 04; SU class `approval`, action `rfi.release`, declared in section 4.5 item 11; 4E; `If-Match` = the instance version from `GET /rfis/{id}`):
1. `WorkflowEngine::act(ctx, task_id, Approve | Reject, if_match, idem, assertion)` runs the envelope of phase 04 section 3.4: the Task is eligible (role `RFI` or the MLRO by `NamedOfficers`), the proposer is excluded (4E: 403 `E_SELF_APPROVAL`), and the engine consumes the assertion once for `rfi.release` (401 `E_STEPUP_REQUIRED` otherwise).
2. `RfiWorkflowSubject::check_before_decision` (section 3.2 subject rules below): (i) the case row is locked (rank 2) and the RFI row FOR UPDATE; (ii) status `pending_release` and the instance is running; (iii) the case is not sealed; (iv) `AuthorityMatrix::permits(actor.roles, "rfi.release", rfi.priority_class)` (phase 09 section 3.6, decision type `rfi.release`, scope `rfi.desk`, roles RFI and M): refusal 403 `E_FORBIDDEN` with sub-code `role_not_permitted`; a permit with `second_review = true` is 409 `E_NOT_ALLOWED_IN_STATE` with sub-code `second_review_not_shipped`, as phase 18 DR-143 does; (v) for a `authority_response`, the request is `drafting` or `approved` (else 409 `E_NOT_ALLOWED_IN_STATE` with sub-code `not_on_case`).
3. On `approve`, the instance completes and `ReleaseRfiEffect::run(tx, inst, params = {}, actor)` runs in the same transaction: set `status = released`, `released_by = actor` (the completer of the task, CHECK `released_by <> proposer_id`), `released_at = now()`; the deadline of section 3.2 step 3 is already set; chain `case.rfi_released {rfi_id, kind}`; audit `rfi.released {rfi_id, kind, deadline_at}`. For `authority_response`: the request becomes `approved` (`approved_by`, `approved_at`); when the response's answers are complete, the disclosure fields of the request are set (section 3.4).
4. On `reject`, the instance is rejected and `on_ended(rejected)` runs: status `rejected` (terminal), chain `case.rfi_rejected`, audit `rfi.rejected`. A rejected RFI is not re-submitted: the drafter drafts a new one (the guard of section 1.2 makes `rejected` terminal).
5. `on_ended(withdrawn | failed)`: status `withdrawn`; audit `rfi.withdrawn`.

**`RfiWorkflowSubject`** (phase 04 `WorkflowSubject`, `resource_kind = "rfi"`, `attribute_prefix = "rfi"`; attributes `kind` (text), `tier` (text), `priority` (text), `external` (boolean: the instance runs `rfi_release_external`); `parties()` none):
- `start_context(conn, rfi_id)`: the attributes above frozen; `classification = restricted` when the RFI's tier is `restricted` or `suspicion`, otherwise `internal`; `maker_id` = the proposer; `title` `{ar: "طلب معلومات", en: "Information request"}` (no kind, no party, no case word).
- `visible_to(principal, rfi_id)` and `visibility_sql()`: a principal with an active `RFI` role grant sees the RFI when its case tier is not `suspicion`; `M` sees every RFI; a member of the case team (`assignees`, phase 18 section 3.2) sees the RFI of its case. The SQL is `(i.resource_kind <> 'rfi' OR EXISTS (SELECT 1 FROM casework.rfis r JOIN casework.cases c ON c.id = r.case_id WHERE r.id = i.resource_id AND ((c.tier <> 'suspicion' AND EXISTS (SELECT 1 FROM admin.role_grants g WHERE g.user_id = $1 AND g.role = 'RFI' AND g.revoked_at IS NULL AND (g.valid_to IS NULL OR g.valid_to > now()))) OR EXISTS (SELECT 1 FROM admin.role_grants m WHERE m.user_id = $1 AND m.role = 'M' AND m.revoked_at IS NULL AND (m.valid_to IS NULL OR m.valid_to > now())) OR $1 IN (c.first_worker_id, c.second_worker_id, c.assignee_id) OR EXISTS (SELECT 1 FROM casework.case_acl a WHERE a.case_id = c.id AND a.principal_id = $1 AND a.revoked_at IS NULL))))`. The tier is read from `casework.cases`, never copied.
- `on_ended` and the phase 04 `Holders` rules: section 3.2 steps 4 and 5 above.

**Decision reads and the RFI desk.** `GET /rfis` (`rfi.desk`) returns the RFIs of `visibility_sql` for the caller, with `RfiDeskView` (no case id, family, priority or tier); `GET /cases/{id}/rfis` (`queue.work`, team only) returns `RfiTeamView`. A suspicion RFI read by a caller who is neither M nor on the case team is a masked 404 (section 3.14).

### 3.3 Answers as claims, the closure gate and the RFI KPIs (`RfiService::record_answers`, `RfiService::extend`, `RfiService::kpis`; M-44, M-45, M-46, DR-150)

**`RfiService::record_answers`** (`POST /rfis/{id}/answers`; Cedar `rfi.answer` on Rfi; `If-Match` = RFI version):
1. Lock the case row, then the RFI row. Status `released` or `answered` (409 `E_NOT_ALLOWED_IN_STATE` otherwise).
2. For each `AnswerInput`: the question belongs to the RFI and is `open` (or `answered` when the claim corrects a current one through `supersedes`; the service sets the old claim `superseded`).
3. `corroborates` (a claim id of this RFI, optional): the service raises the corroboration of that claim (never lowers it; a `documented` claim is kept) and writes no new claim for that entry. Otherwise a new claim: `value` sealed (AAD = answer id) after `PolicyPackService::scan(scope "rfi", value)`; `expires_at = answered_at + casework.rfi_claim_expiry_days` (calendar days; `answered_at` defaults to now and is never in the future).
4. `corroboration = documented` needs an `evidence_object_id` that is an evidence object of this case (owner kind `case`, owner id = the case, phase 02 `admin.evidence_objects`; the upload is phase 18's `POST /cases/{id}/evidence`); else 422 `E_VALIDATION` at `/answers/{i}/evidence_object_id`.
5. Insert the claims (`claim_state = current`), set the questions `answered`; when every required question is `answered` or `waived`, the RFI becomes `answered` (`answered_at`), its reminder timers are cancelled, and `released` moves to `answered` (guard of section 1.2).
6. Chain `case.rfi_answered {rfi_id, count}`; audit `rfi.answer_recorded {rfi_id, answer_ids, corroboration_raised}` (ids and counts only).

**Claims expire.** `rfi_clock_scan` (hourly, section 4.1) sets `claim_state = expired` for `current` claims with `expires_at <= now()`; audit `rfi.claim_expired {answer_id}` per claim. An expired claim stays in the register.

**The closure gate (M-45; phase 18 section 3.12 extended, section 4.5 item 5).** `CaseService::close` of a case whose priority is `P0` or `P1` refuses (409 `E_NOT_ALLOWED_IN_STATE`, sub-code `uncorroborated_explanation`) when the case has at least one current RFI claim and no case evidence item (`casework.evidence_items`) and no current claim with `corroboration <> 'none'`. An explanation that rests on an uncorroborated answer alone cannot close a P0 or P1 case.

**`RfiService::extend`** (`POST /rfis/{id}/extend`; Cedar `rfi.extend` on Rfi; `If-Match`): the RFI is `released` and `extensions < 2`; `reason` is scanned with scope `reason_text`; `deadline_at = BusinessClock::add(deadline_at, casework.rfi_extension_days, WorkingDays)`; `extensions + 1`; the timers of section 3.2 step 5 are cancelled and rescheduled; chain `case.rfi_extended`; audit `rfi.extended {rfi_id, extensions, deadline_at}`.

**The KPIs** (`GET /rfis`, `kpis` block; M-46). Over the RFIs released in the last 180 days: `median_days` and `p90_days` of `answered_at - released_at` in calendar days (the working-day count is Open doubts 13); `share_by_deadline` (answered on or before `deadline_at` over answered); `follow_up_rounds` (sum of `extensions`); `top_question_keys` (the five most frequent `question_key` values of the window); `missed_by_monitoring` (RFIs whose party had no alert in the 180 days before the RFI was drafted). The block is computed on the RFIs the caller can see.

### 3.4 Internal reports and authority requests (`InternalReportService`, `AuthorityRequestService`; M-37, M-66 side, M-68, DR-151)

**Internal reports** (`POST /internal-reports`, `InternalReportService::receive`, scope `case.decide`, Cedar `internal_report.write` on System; also the port that the branch window of phase 22 calls with the same command):
1. `reporter_kind` and `reporter_id` (a canonical staff row or an admin user); `channel`; `body` sealed (AAD = id); `received_at` not in the future.
2. `ack_due_at = BusinessClock::add(received_at, casework.internal_report_ack_hours, BusinessUnit::Hours)` (phase 09: hours are plain addition, Open doubts 3).
3. Insert `casework.internal_reports` (status `received`, tier `suspicion`); timer `intake_clock` (owner `internal_report`, payload `{kind: "ack"}`, fire at `ack_due_at`); outbox `internal_report.recorded {internal_report_id, party_ids}` (the TR-15 and suspicion-linked consumer of phase 15, section 3.3 of phase 15, maps it to `TRG_REVIEW_REQUESTED`); audit `internal_report.received {internal_report_id, channel, reporter_kind}` (the reporter id is not in the payload).
4. `POST /internal-reports/{id}/acknowledge` (Cedar `internal_report.acknowledge`; `If-Match`): status `received` to `acknowledged`, `acknowledged_by`, `acknowledged_at`; the timer is cancelled; audit `internal_report.acknowledged`. A report not acknowledged at `ack_due_at` produces the outbox event `internal_report.overdue` at the timer's fire (section 4.1), for the MLRO desk notice of phase 22.
5. `reporter_id` is in every response to M only (`InternalReportView.reporter_id`); other readers receive `null`. The branch manager never sees a report (phase 22 rule; M-66).

**Authority requests** (`POST /authority-requests`, `AuthorityRequestService::register`; SU class `decision`, action `authority.register`; Cedar on System; suspicion tier only):
1. `authority_ref` is unique per `authority_kind` (409 `E_NOT_ALLOWED_IN_STATE`, sub-code `already_registered`); each subject is a canonical party (422 `unknown_party`); each linked case is an open or closed case the caller can see (422 `not_on_case` otherwise).
2. `deadline_at` = the stated deadline with `deadline_source = stated`, else `BusinessClock::add(received_at, casework.authority_default_days, WorkingDays)` with `deadline_source = bank_default`; `clock_days` recorded.
3. Insert the request (status `received`, `questions_enc` sealed, `tier` `suspicion`, `registered_by`); timer `intake_clock` (owner `authority_request`, payload `{kind: "deadline"}`, fire at `deadline_at`).
4. Chain `case.authority_linked {authority_request_id}` on each linked case (rank 2, ascending case id order).
5. Outbox `authority_request.recorded {authority_request_id, party_ids}` (at most 50 parties). This is the **TR-17 trigger**: the phase 15 consumer of section 3.3 of phase 15 enqueues an immediate `rating_compute` for each subject party with the neutral trigger code (phase 15 section 3.6 rule 3, `TRG_REVIEW_REQUESTED`). This phase writes no rating.
6. The same outbox event is consumed by this phase for the watch and hibernation triggers (section 3.5 and 3.6).
7. Audit `authority_request.registered {authority_request_id, authority_kind, deadline_source}`.
8. Draft of the response: `AuthorityRequestService::draft_response(ctx, id)` (the route is the RFI draft of kind `authority_response`, section 3.1 step 4): the request becomes `drafting`; the draft is built from system facts (the register fields and the pack template) and the questions are the sealed ones; audit `authority_request.drafted {authority_request_id, rfi_id}`.
9. The disclosure register entry (M-37): when the response RFI is released and every required question is answered, `ReleaseRfiEffect` writes `disclosed_at = released_at` and `disclosed_items` (the claim ids and question keys, ids and codes only) and the request becomes `answered`; `closed` is set by a person (route not in this phase: Open doubts 5).

**Deadlines.** The timer `intake_clock` of an internal report fires `internal_report.overdue` (outbox) when unacknowledged; the timer of an authority request fires `authority_request.overdue` (outbox) when the request is not `answered` or `closed`. Both are notices to the MLRO desk through phase 22; neither changes a status.

### 3.5 Watches (`WatchService`; M-35, DR-152)

**Start** (`WatchService::start_for_decision(tx, decision, case)`, called by `RecordDecisionEffect` of phase 18 section 3.8 step f for `utr` (section 4.5 item 5); and `POST /cases/{id}/watches` (Cedar `watch.write` on Case, scope `queue.work`) which starts a watch on the case's latest `utr` decision):
1. The case row is locked (rank 2). The decision type is `utr` (else 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `watch_needs_utr`). A watch exists for this decision (the unique `origin_decision_id`): replay.
2. The envelope: `measures = {n: sum of matched_count of the case's scenario alerts at decision time, total: the sum of the alerts' measure totals}` (the alert `measures` as phase 17 stores them), `source = "decision"`.
3. `triggers` = the four closed keys: `list_hit`, `authority_request`, `profile_change`, `new_alert`.
4. `started_at = now()`; `expires_at = BusinessClock::add(started_at, casework.watch_days, BusinessUnit::Days)`; `review_due_at = BusinessClock::add(expires_at - 1 day, 1, BusinessUnit::WorkingDays)` (the first working day on or after expiry, so a watch that expires on a holiday is reviewed the next working day).
5. Insert `casework.watches` (`period_days` = `casework.watch_days`); timer `watch_expiry` (owner `watch`, fire at `expires_at`, calendar = bank calendar).
6. Chain `case.watch_started {watch_id}`; audit `watch.started {watch_id, origin_decision_id}`.

**Triggers.** The outbox consumer job `watch_trigger_dispatch` (section 4.1) reads the topics of section 4.2 and, for each open or under-review watch of the party in the event, runs in its own transaction: (i) lock the case row (rank 2); (ii) lock the watch row FOR UPDATE (rank 3); (iii) state `open` or `under_review`; (iv) `record_trigger` sets `trigger_count + 1`, `last_trigger_at`, `last_trigger_key`; (v) when the case is open or in review, `WorkItemService::open` (phase 18 section 3.6) creates a `review_evidence` work item due in `casework.work_item_due_hours`; a closed case gets no work item and the trigger is recorded only; (vi) audit `watch.triggered {watch_id, trigger_key}`, outbox `watch.triggered {watch_id}`. The trigger keys map: `screening.hit` to `list_hit`, `authority_request.recorded` to `authority_request`, `canon.touched` (a watched party among the batch's changed party rows) to `profile_change`, `alert.created` (the party of the alert) to `new_alert`.

**Expiry.** `timer_watch_expiry` (owner `watch`): re-reads the watch; when `open` and `expires_at <= now()`: state `under_review`; work item `review_evidence` on the case (open case); outbox `watch.expired`; audit `watch.expired`. A closed or upgraded watch does nothing.

**Review** (`POST /watches/{id}/review`; scope `case.decide`; Cedar `watch.review` on Case; SU class `decision`; `If-Match`), served by `WatchService::review(ctx, id, ReviewWatch, if_match)`, which writes the outcome with `WatchRepository::set_review`: the watch is `under_review` (409 `E_NOT_ALLOWED_IN_STATE`, sub-code `not_due` otherwise); lock order: case, then watch.
- `extend`: `expires_at = BusinessClock::add(now, casework.watch_days, Days)`; state `open`; a new timer; `review_decision = extend`.
- `close`: state `closed`; `review_decision = close`.
- `upgrade_str`: state `upgraded`; `review_decision = upgrade_str`; a work item `other` on the case with the sealed description "review for a report outcome"; the watch does not decide anything: a report outcome is a new proposal by a person under phase 18 (DR-152).
- The reason is sealed (`review_note_enc`, AAD = watch id) and scanned with scope `reason_text`.
- Chain `case.watch_reviewed {watch_id, review}`; audit `watch.reviewed {watch_id, review}`.

### 3.6 Hibernations, the suppression gate and the regret metric (`HibernationService`, `HibernationGate`, `HibernationHitService`; M-39, DR-153)

**Create** (`POST /cases/{id}/hibernations`; scope `case.decide`; Cedar `hibernation.create` on Case (A2 for a non-suspicion party, M for any party; section 3.13); SU class `decision`; `If-Match` = case version; `Idempotency-Key`):
1. Lock the case row. The case is `open` with outcome `no_report` (phase 18 `RecordDecisionEffect` sets the outcome and leaves the case open until a person closes it; `decision_id` is that decision, and the guard of section 1.2 checks its type and case). A closed case is sealed: 409 `E_CASE_SEALED` (the W7 dialog offers the hibernation before closure, section 5.3). Any other state: 409 `E_NOT_ALLOWED_IN_STATE`.
2. The party is the case subject (`casework.cases.party_id`) or a case party; the scenario has an active version (`scn.scenario_versions`, phase 16) whose document has `suppression.hibernation.allowed = true` (DSL A.7); else 422 `E_VALIDATION`, sub-code `hibernation_not_allowed`.
3. `period_days` is at most `casework.hibernation_max_days` (180, the database CHECK is fixed at 180) and defaults to `casework.hibernation_default_days` (90).
4. The envelope: `{"measures": {"n": n_now, "total": total_now}, "margin": casework.hibernation_margin (0.2), "scenario_version_id": ...}` where `n_now` and `total_now` are the measures of the party's most recent alert of the scenario (the same measures the gate of step below compares).
5. No active hibernation exists for (party, scenario) (the partial unique index; 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `already_hibernating`).
6. The party's tier: `casework.effective_tier(party_id)` is `suspicion` only for M (Cedar; an A2 caller is refused 403 `E_FORBIDDEN`).
7. Insert `casework.hibernations`; timer `hibernation_expiry` (owner `hibernation`, fire at `expires_at = BusinessClock::add(created_at, period_days, Days)`).
8. Chain `case.hibernation_started {hibernation_id, scenario_id}`; audit `hibernation.started {hibernation_id, scenario_id, period_days}`.

**`HibernationGate::check`** (the port that phase 17's `AlertIntakeService` calls between its steps 4 and 5, after the snapshot and before the case is opened or found, section 4.5 item 4; DR-153): one read, no lock:
1. `h = active hibernation for (party_id, scenario_id)`; none gives `Show`.
2. The envelope test: `exceeded = measures.n > env.n × (1 + env.margin) OR measures.total > env.total × (1 + env.margin)`. When exceeded, the gate returns `Show` and the intake writes the outbox event `hibernation.envelope_exceeded {hibernation_id}` in its transaction: the hibernation is ended by its job, not by the intake transaction (which holds no hibernation lock, section 3.15).
3. Otherwise `Suppress(h.id)`: the intake inserts one `casework.hibernation_hits` row (`hit_at = now()`, `measures`), audit `hibernation.hit_recorded {hibernation_id, alert_id}`. The alert row stays in `alert.alerts`; it is hidden from queues and counts while its hit is open (section 4.5 item 4). No case is opened or joined by a suppressed alert (section 4.5 item 4).

**End** (`HibernationService::end`): by expiry (`timer_hibernation_expiry`: when `ended_at IS NULL AND expires_at <= now()`), by envelope (outbox `hibernation.envelope_exceeded`), by trigger (below) or by a person (`POST /hibernations/{id}/end`, scope `case.decide`, `reason` 5 to 200 characters, `ended_by` = actor, `ended_reason = ended_by_person`). The end locks the hibernation row only (rank 3); it sets `ended_at`, `ended_reason`, and writes the outbox `hibernation.ended {hibernation_id, reason, open_hits}`; audit `hibernation.ended {hibernation_id, reason}`.
- **Release of the hits** (job `hibernation_release`, section 4.1; one transaction per hit): lock the party's case of the alert (rank 2; the alert's case, or none), lock the hit row FOR UPDATE (rank 3), set `released_at = now()`, `release_reason` = the reason of the end, and call `AlertIntakeService::attach_released(alert_id, alert_created_at)` (phase 17 steps 5, 7, 9, 10 and 12 for a released alert, section 4.5 item 4); audit `hibernation.released {hit_id}`.
- **Triggers** (job `watch_trigger_dispatch`, the same consumer as the watches): an active hibernation of a party in `authority_request.recorded` ends with `trigger` and `trigger_key = authority_request`; in `screening.hit` (a list change) with `list_change`; in `canon.touched` (a profile change) with `profile_change`; the `adverse_media` key has no producer until a media source is contracted (Open doubts 6).

**Regret** (job `hibernation_regret`, daily): for each hit with `report_after = FALSE` whose party has a decision of type `utr`, `str` or `tfr` with `decided_at >= hit_at` and `decided_at <= hibernation.expires_at + 30 days`, set `report_after = TRUE` (rank 3 only; the guard keeps the mark). The ledger shows `regret_share` per hibernation (hits with `report_after` over hits) and overall. The metric is the share of suppressed alerts that a later report outcome shows were needed.

**Ledger** (`GET /hibernations`; scope `queue.work`; Cedar `hibernation.read` on System with the case visibility predicate of phase 17 section 3.10 over the hibernation's case): rows with `hits`, `released_hits`, `regret_share`, state, expiry; the count is taken after the predicate.

### 3.7 Network view, edges, snapshots and network cases (`NetworkService`, `EdgeBuilder`; M-10, T-21, DR-154)

**Edges come only from canonical rows** (`EdgeBuilder::rebuild(tx, batch_parties, batch_id, at)`, job `network_edge_build`, section 4.1). For each touched party `p` (the parties of the `canon.touched` batch, plus the sweep of section 4.1), recompute the live edges incident to `p` from these four sources and no other:

| Kind | Canonical source (current rows) | Stored as | Hidden by default |
|-|-|-|-|
| `shared_contact` | `canon.party_contact` rows of type `mobile` or `landline` (phone) or `address`, with equal non-null `value_hk` and `superseded_at IS NULL` | undirected, smaller id first | no |
| `kinship` | `canon.party.family_no_hk` equal, `recorded_to IS NULL` (phase 16 link `family_no`) | undirected | **yes** |
| `ownership` | `canon.party_relationship` of type `owner`, `controller`, `director`, `signatory` or `attorney`, `recorded_to IS NULL`, `evidence <> '{}'` | directed from the holder to the party (`from_party` is the holder) | no |
| `transfer` | `canon.payment_link` rows with `confidence >= casework.network_transfer_min_confidence`, joined to `canon.txn` on `(txn_id, booking_date)`, whose two sides are owners of the two accounts (`canon.account_party_role` `role = 'owner'`) | undirected | no |

- `shared_device`: no canonical source exists in phase 10; no edge of this kind is built (Open doubts 7).
- Shared accounts (`signatory`, `joint`, `owner` on one account) are phase 16's linked-party links but not network edges in this draft (Open doubts 6).
- Steps: (1) read the four sources for `p` (`CanonNetworkReader`, SELECT only); (2) upsert the live edges (`uq_network_edges_live`): `weight` = the count of canonical rows behind the edge, `basis` = the canonical row ids and link types (never values), `last_seen = now()`, `first_seen` kept; (3) retire the live edges incident to `p` that the sources no longer produce (`retired_at = now()`); (4) for each new edge whose endpoint has an open case, outbox `graph.link {party_id, kind}` (the phase 15 trigger of section 3.3 of phase 15); (5) audit `network.edges_rebuilt {party_id, added, retired}` once per batch job.
- **Supernode rule (T-21):** a node whose live degree exceeds `casework.network_supernode_degree` (200 in the seed; setting added with this draft's settings, section 1.2) is shown with a summary by kind and count; its edges are not listed in any graph response.
- The sweep job `network_edge_build` (daily, section 4.1) recomputes every party changed since its last build, so a missed `canon.touched` cannot leave an edge stale for more than a day.

**Graph** (`GET /parties/{id}/graph`; scope `queue.work`; Cedar `network.read` on Party; section 3.13):
1. The seed is visible to the caller under the phase 18 party predicate (section 3.10 of phase 18): otherwise masked 404 (`E_NOT_FOUND`).
2. The edges are the live edges incident to the seed (one hop). Each neighbour is included only when it is visible to the caller under the same predicate. An invisible neighbour and its edges are omitted without a count, a hint or a placeholder (tipping-off, section 3.14).
3. Kinship edges are omitted unless `show_kinship=true`. With it: `X-Access-Purpose` is required (`investigation` or `assigned_work`, else 422 `E_PURPOSE_REQUIRED`), **SU** `party.open` (class `record_open`, phase 18 section 4.5 item 14) is consumed, the read writes an access event (phase 02), and the audit event `network.kinship_shown {party_id, edge_count}` is written. A caller who does not show kinship gets no kinship edge and no count of them.
4. Nodes carry `party_id` and `display_name` (the phase 18 `PartyService` name of the party, `canon.party_name` where `superseded_at IS NULL`) for visible nodes only.
5. The summary `GraphView.summary` is computed on the visible nodes and edges only: for example "9 accounts, 214 senders, 96 percent to two exchange companies" is the UI text pattern of DESIGN-SYSTEM 7.16, computed by the service from the visible data.

**Snapshot and network case** (`POST /parties/{id}/network-cases`; scope `case.decide`; Cedar `network.case.open` on Party):
1. The seed is visible to the caller, and has at least one live non-kinship edge; else 422 `E_NOT_ALLOWED_IN_STATE` with sub-code `no_network`.
2. Subjects: the neighbours joined by a live non-kinship edge, visible to the caller, ordered by edge weight descending then id, at most `casework.network_max_children` (25).
3. The parent: a case of family `network` for the seed (`casework.cases` insert; `parent_case_id` NULL; `priority_class` the highest class of the seed's open cases or `P2`; `tier` the seed's effective tier; `policy_pack_version` the active pack). The partial unique index (party, family) allows one open network parent per seed.
4. The children: for each subject without an open `network` case, a case of family `network` for the subject with `parent_case_id` = the parent, `priority_class = P3`, `tier` the subject's effective tier. The parent and each child are locked in ascending id order (rank 2).
5. The snapshot: the route writes the outbox event `network.snapshot_requested {case_id, seed_party_id, viewer_id}`; the worker job `network_snapshot_seal` (section 4.1) builds the graph under the viewer's rules of the graph route (kinship excluded), calls `EvidenceStore::put_generated` (owner kind `network_snapshot`, kind `snapshot`, owner id the snapshot id, retention `R6`; phase 02 `put_generated` is worker only), inserts `casework.network_snapshots` and a `casework.evidence_items` row on the parent (label `Network snapshot`), and writes audit `network.snapshot_taken {snapshot_id, node_count, edge_count}`. The snapshot is evidence of the parent (M-10).
6. Chain `case.network_opened {children}` on the parent and `case.network_child_opened` on each child; audit `network.case_opened {parent_case_id, child_count}`.
7. **The parent closes when its children have outcomes**: `CaseService::close` of a child (phase 18 section 3.12, extended by section 4.5 item 5) locks the child and its parent in ascending id order, and when no open child remains writes `case.network_children_done` on the parent. The parent is then ready to close; closing stays a person's act (phase 18 DR-140 spirit; Open doubts 8).

### 3.8 QA sampling (`QaService::draw`, `QaSampler`; M-16, M-47, M-49, DR-155)

**Sample size** (`QaSampler::sample_size(C, p, N) -> u32`, pure): `n0 = ceil(ln(1 - C) / ln(1 - p))`. For a finite population of `N` items with `D = ceil(p × N)` defects, the smallest `n` in `1..=N` with `1 - Π_{i=0}^{n-1} (N - D - i) / (N - i) >= C` (the hypergeometric probability of at least one defect; a term with `N - D - i < 0` is 0). For `C = 0.95` and `p = 0.05` the infinite formula gives 59. The finite form is used when `N < 10 000`.

**Populations** (`QaSampleRepository::population`, read only; the window is the last business week for the weekly sample, the previous business day for the daily sample, otherwise `since` and `until`):

| Population | Rows | Stratum | Mode in the seed |
|-|-|-|-|
| `closed_alerts` | alerts `triaged` whose case closed in the window | priority class and family | stratified |
| `closed_cases` | cases closed in the window | family and outcome | stratified |
| `shelved_decisions` | `casework.decisions` of type `no_report` (phase 18 `ShelvedRegister` population) | first reason code family and `decider_id` | stratified |
| `mlro_decisions` | decisions whose decider holds `M` (`admin.role_grants`) | decision type | random; reviewer must hold `CH` or `IA` (M-49) |
| `hibernation_hits` | hits in the window | scenario | stratified |
| `screening_suppressions` | the phase 14 pair-memory suppressions of the week (5 per cent, `qa.suppression_sample_percent`; Open doubts 9) | list | random |
| `low_score_forced` | alerts created on the previous business day in state `queued` or `claimed` | none | targeted `lowest_score_day`, `qa.forced_low_score_daily` items (M-16) |
| `calibration` | golden items (`is_calibration`), one row per calibrator | none | random |
| `targeted_repeat` | the re-review set after a critical finding | none | targeted `same_closer_30d`, at most 10 |

**Targeted rules** (closed in code): `p0_closed_no_report`, `escalated_level_3`, `rfi_uncorroborated` (P0 or P1 cases closed with only uncorroborated RFI claims), `lowest_score_day`, `same_closer_30d`.

**Draw** (`POST /qa/samples`; scope `qa.review`; Cedar `qa.sample.draw` on System; the scheduled draws run the same function with no actor, `drawn_by` NULL):
1. Input: population, mode, `strata_by` (at most three of: priority class, family, outcome, scenario, reason family), `n` or the pair (`confidence`, `defect_rate`) from the settings, the window, and `reviewer_id` for `mlro_decisions` only.
2. Read the population rows. `N` is its size; `n = min(N, requested or QaSampler::sample_size)`.
3. Reviewer candidates: active users with an active `QA` grant (for `mlro_decisions`: active users with `CH` or `IA`, never `M`). Exclusions per item: the closers of the item (the decider, the proposer, the assignee, the first and second workers, the alert's `triaged_by` and `claimed_by`), the members of the case team, and any user with `sod_conflict = true` (the phase 03 user attribute). The reviewer with the fewest open samples is assigned; an item with no eligible reviewer is returned in `unassigned` and not drawn (sub-code `no_eligible_reviewer` on the item in the response).
4. Allocation: `stratified` allocates `a_s = max(qa.min_per_stratum, round(n × N_s / N))` per stratum with a largest-remainder correction to `n`; `random` and `targeted` select from the eligible rows.
5. Selection is reproducible: `draw_seed` is 32 bytes from the operating system's random source (stored as 64 hex characters); the rows are sorted by id and a seeded shuffle (ChaCha20 keyed by the seed) selects `a_s` items. The audit event carries the digest of the sorted ids (SHA-256), so a regulator can re-run the draw.
6. Insert `casework.qa_samples` (one per item; `due_at = BusinessClock::add(now, qa.review_working_days, WorkingDays)`; `is_calibration` for the calibration population).
7. For a case item (`source_kind` `case` or `decision`), a time-boxed read grant: lock the case (rank 2) and insert a `casework.case_acl` row (`access = read`, `reason = qa_review`, `granted_by` the drawer or NULL for a scheduled draw, no note) through `QaService::grant_review_access`. Phase 18's `CaseAccessService::grant` refuses a closed case with `E_CASE_SEALED` and most sampled cases are closed, so the QA grant is its own path: it never gives `work` or `decide`, it writes no case event on a closed case, and its audit event is `qa.grant_opened {sample_id, case_id}` (lead review). Then `qa_samples.grant_id`; timer `qa_grant_expiry` (owner `qa_sample`, fire at `now() + qa.grant_days` calendar days). Cases are locked in ascending id order (section 3.15).
8. Audit `qa.sample_drawn {sample_id, population, draw_id, reviewer_id}` per item, and the draw summary `qa.draw_summary {draw_id, population, n, unassigned, digest}`.
9. Response: the draw summary and the samples the caller may see (a reviewer sees their own rows; M sees all rows; no party or account value).

**Independence, in four layers (M-49):** the candidate filter (closers and team), the grant (read only, time-boxed, revoked at review or expiry), the database guard on the review (section 1.2: a review is written only by the sample's reviewer while the sample is open), and the Cedar `qa.review.write` (principal is the reviewer). A reviewer never exports; there is no export route.

### 3.9 QA reviews, findings, coaching, bulk reopen, calibration and rates (`QaService`; M-48, M-49, M-50, M-40, DR-155)

**Review** (`POST /qa/samples/{id}/review`; scope `qa.review`; Cedar `qa.review.write` on QaSample; `If-Match` = sample version):
1. Lock order: the case row (rank 2, when the item is a case item), the sample row FOR UPDATE (rank 3), the grant row (rank 4, section 3.15).
2. The sample is `open` (409 `E_NOT_ALLOWED_IN_STATE`, sub-code `sample_closed`); the caller is the reviewer (403 `E_FORBIDDEN`, sub-code `not_sample_reviewer`); the caller is not a closer of the item (sub-code `reviewer_is_closer`; the item is not reviewed, and the sample's reviewer is changed by the scheduled reassignment of a conflicted sample, which the guard allows while the sample is open).
3. Scores: all eight criteria of section 2 with a `Level`; `overall` is the most severe score (`Pass` < `PassWithComment` < `Minor` < `Major` < `Critical`), else 422 `E_VALIDATION`, sub-code `score_mismatch`.
4. Findings: one per failing criterion (`score >= Minor`); a finding on a passing criterion is 422 (sub-code `finding_criterion_passed`); `severity` equals the score.
5. Insert `casework.qa_reviews` (the comment sealed, AAD = review id) and `casework.qa_findings`; the sample becomes `reviewed`.
6. **Major** finding: `coaching_state = open`, `coaching_due_at = BusinessClock::add(now, qa.coaching_working_days, WorkingDays)`; audit `qa.coaching_opened {finding_id}`.
7. **Critical** finding: alarm `qa_critical` (subject kind `qa_review`, subject id the review, supervisor M, payload `{finding_ids, sample_id, closer_id}`, section 3.10); and the re-review: the targeted draw `same_closer_30d` (the items the same closer closed in the 30 days before the review, at most 10, reviewers are other QA users) whose ids are written to the finding's `re_review_sample_ids`.
8. The time-boxed grant of the item is revoked by `QaService` (`revoked_by = reviewer`, `revoked_at = now()`; the phase 18 CHECK requires a revoker), so the grant ends at review, not at expiry; audit `qa.grant_revoked {sample_id}`.
9. Audit `qa.reviewed {sample_id, overall}` and `qa.finding_recorded {finding_id, criterion, severity}` per finding.

**Grant expiry** (`timer_qa_grant_expiry`, and the backstop `qa_grant_sweep`): a grant still open after `qa.grant_days` is revoked with `revoked_by = reviewer_id`, the sample becomes `void` if not reviewed (guard of section 1.2), audit `qa.grant_revoked {sample_id}`.

**Bulk reopen** (`POST /qa/bulk-reopen`; scope `qa.review`; Cedar `qa.bulk_reopen` on System, M only; SU class `decision`, action `qa.bulk_reopen`; `Idempotency-Key`): `finding_id` is a critical finding; `case_ids` (at most `qa.bulk_reopen_max`, 25) are cases the analyst (`analyst_id`) closed in the window of the finding (the closer is the decider or the assignee; any other case is 422 `E_VALIDATION`, sub-code `not_bulk_case`). For each case in ascending id order (own transaction each) the phase 18 reopen rules run (section 3.13 of phase 18, with the batch assertion of this route in place of a second one: `CaseService::reopen_under_batch`, section 4.5 item 5), and the result is `reopened` or refused with the sub-code. The response lists each case with its result. Audit `qa.bulk_reopen {finding_id, analyst_id, reopened, refused}`; each reopen writes phase 18's `case.reopened`.

**Rates** (`GET /qa/rates`; scope `qa.review`; Cedar `qa.rates.read`, M only): for each reviewer with reviews in the window: `reviews`, `share_major_or_critical`, and the Wilson 95 per cent interval `(p̂ + z²/2n ± z·√(p̂(1-p̂)/n + z²/4n²)) / (1 + z²/n)` with `z = 1.96`. A reviewer with fewer than `qa.min_reviewer_items` reviews shows the count only (no rate).

**Calibration** (`GET /qa/calibration`; scope `qa.review`; Cedar `qa.calibration.read`, QA and M): the golden items (`is_calibration`, `golden_outcome` set), and per calibrator `matches` (review `overall` equal to `golden_outcome`), `total` and the Wilson interval. The agreement of calibrators on items without a golden outcome is a second use of the same rows and is Open doubts 14 (the gating of queue access by a recorded calibration result is not built).

### 3.10 Breach actions and alarms (`BreachService`, `AlarmService`; M-22, M-53, M-67, M-11, DR-156)

**Breach stages** (M-22: 50, 100 and 150 percent of the window). The window is the touch window of an alert (`casework.touch_by_hours[class]`, phase 17 section 3.12, working hours) or the due window of a work item (`casework.work_item_due_hours`, phase 18 section 3.6).
1. `BreachService::arm(tx, owner_kind, owner_id, start_at, window_hours)` (called by phase 17 intake step 9 for alerts, and by phase 18 `WorkItemService::open` for work items, section 4.5 items 4 and 5) schedules two timers of kind `breach_stage`: stage 50 at `BusinessClock::add(start_at, floor(window_hours / 2), Hours)` and stage 150 at `BusinessClock::add(start_at, ceil(window_hours × 3 / 2), Hours)` (plain hours, phase 09). Owner kind `alert` or `work_item`, payload `{stage}`.
2. The 100 percent stage is the existing deadline timer: `timer_alert_touch` of phase 17 (section 3.12) and `timer_work_item_due` of phase 18 (section 3.6). Their breach write calls `BreachService::breach(tx, owner, 100)`.
3. Stage 50 (`timer_breach_stage`, payload 50): re-reads the owner; when still open and not yet breached, the outbox event `clock.warning {owner_kind, owner_id, stage: 50}` (the SSE event of phase 00, API section 2); no alarm and no status change.
4. Stage 100: `BreachService::breach(tx, owner, 100)` raises the alarm of step 6 (supervisor: an `A2` of the alert's pool or of the work item's case team, the one with the fewest live alarms, else M).
5. Stage 150 (`timer_breach_stage`, payload 150): when the owner is still open and breached, `BreachService::breach(tx, owner, 150)` raises the alarm with supervisor M.
6. Alarm kinds: `touch_breach` (alert, stage 100 or 150) and `work_item_breach` (work item, stage 100 or 150).
7. Breach actions change no status: an alarm informs the supervisor; the alert and the work item keep their states, and the phase 17 and phase 18 handlers stay as they are.

**Alarm service** (`AlarmService::raise(tx, NewAlarm) -> Option<Alarm>`): `insert_live` (the unique index of section 1.1 on (kind, subject, stage) for a live alarm; a repeat returns `None` and writes nothing); the supervisor is chosen by kind (`touch_breach` and `work_item_breach` at stage 100: an `A2` of the pool or team, else M; `chain_broken`, `conflict_declared`, `flat_line`, `qa_critical`, and every stage 150: M).

| Kind | Raised by | Subject | Supervisor | Who reads it (Cedar, section 3.13) |
|-|-|-|-|-|
| `touch_breach` (100, 150) | phase 17 `timer_alert_touch` handler and breach stage 150 | alert | A2 of the pool, else M | M; A2 for touch breaches |
| `work_item_breach` (100, 150) | phase 18 `timer_work_item_due` handler and breach stage 150 | work item | A2 of the team, else M | M; A2 for work item breaches |
| `chain_broken` | phase 17 `case_chain_check` job (section 3.7 of phase 17) | case | M | M, SEC, IA |
| `conflict_declared` | phase 17 `AssignmentService` on a declared conflict (section 3.8 of phase 17) | case | M | M, SEC |
| `flat_line` | job `flat_line_check` | scenario | M | M |
| `qa_critical` | `QaService::review` (section 3.9 step 7) | QA review | M | M |

- The supervisor's first list (`GET /alarms`, scope `case.decide` or `audit.read`) sets `notified_at` on the alarms it returns for the first time (the "whether and when the supervisor was informed" record of M-53), with audit `alarm.notified`.
- `POST /alarms/{id}/acknowledge` (Cedar `alarm.acknowledge`; `If-Match`): state `open` to `acknowledged` with `acknowledged_by`; with `close: true` and a closure reason code (the `closure` family of phase 09, `ref.reason_codes`), state `closed`, `closed_by`, `closed_at`. Audit `alarm.acknowledged` or `alarm.closed`.
- Outbox `alarm.raised {alarm_id, kind}` at insert (phase 22 notifies the supervisor).
- **Phase 17 and 18 callers**: section 4.5 lists the calls added to the handlers of phase 17 and 18. A declared conflict and a broken chain are alarms here (phase 17 open doubts 16 and 22).

**Flat line** (job `flat_line_check`, daily, M-11): for each scenario with alerts, the daily count of alerts by the bank's local creation date over the last `qa.flat_line_days` (5) business days; a count that is unchanged and above zero on each of those days raises `flat_line` (subject scenario) once.

**Not built here:** the growth, latency, silence and capacity alarms of M-53 and the M-22 "maximum stay in awaiting-information" rule (Open doubts 10).

### 3.11 Errors (`FollowupError`, `thiserror`; API section 6 codes only)

| Variant | Problem code | Status | Note |
|-|-|-|-|
| `NotFound` | `E_NOT_FOUND` | 404 | also the masked answer of phase 18 section 3.10 for a suspicion RFI, a suspicion party and a network seed |
| `Forbidden { sub }` | `E_FORBIDDEN` | 403 | Cedar denial on a row the caller may know; `sub` in `errors[]` |
| `SelfApproval` | `E_SELF_APPROVAL` | 403 | proposer or earlier releaser of an RFI; reviewer who is a closer of the item (sub-code in the body) |
| `StepUpRequired` | `E_STEPUP_REQUIRED` | 401 | missing, spent or wrong-class assertion (`rfi.release`, `authority.register`, `hibernation.create`, `watch.review`, `qa.bulk_reopen`, `party.open`, `qa.item.open`) |
| `PurposeRequired` | `E_PURPOSE_REQUIRED` | 422 | a kinship read or a suspicion item read without `X-Access-Purpose` |
| `Sealed` | `E_CASE_SEALED` | 409 | the case is sealed: a RFI, a watch, a hibernation or a claim cannot change it |
| `NotAllowed { sub }` | `E_NOT_ALLOWED_IN_STATE` | 409 | the state does not allow the command; `sub` in the body |
| `VersionConflict { current }` | `E_VERSION_CONFLICT` | 409 | `If-Match` mismatch; the current version in the body |
| `PreconditionRequired` | `E_PRECONDITION_REQUIRED` | 428 | versioned command without `If-Match` |
| `IdempotencyRequired` | `E_IDEMPOTENCY_REQUIRED` | 400 | sensitive write without `Idempotency-Key` |
| `Validation { errors }` | `E_VALIDATION` | 422 | one entry per field: `pointer`, `code` (sub-code), `spans` for forbidden terms |
| `StaleBase` | `E_STALE_BASE` | 409 | a template base changed under a draft (the pack version pinned to the case moved) |
| `Unavailable` | `E_UNAVAILABLE` | 503 | evidence store, audit or workflow unavailable; no content is sent |
| `HashMismatch` | `E_HASH_MISMATCH` | 500 | a network snapshot failed its hash on serve |

The sub-codes of this phase (closed list, riding in `errors[].code` or in `sub`; the API owner adds them to the list of phase 18 section 3.17, Open doubts 15): `recipient_not_liaison`, `tier_forbids_rm`, `contact_not_allowed`, `template_unknown`, `purpose_not_in_pack`, `already_registered`, `watch_needs_utr`, `already_hibernating`, `hibernation_not_allowed`, `no_network`, `not_due`, `uncorroborated_explanation`, `no_eligible_reviewer`, `reviewer_is_closer`, `not_sample_reviewer`, `sample_closed`, `finding_criterion_passed`, `score_mismatch`, `not_bulk_case`, `no_linked_case`, `role_not_permitted`, `not_on_case`, `unknown_party`, `forbidden_term`, `rfi_open`, `not_critical_finding`.

### 3.12 Audit events (one per mutation; payloads carry ids, codes and counts, never names, values or text)

| Event kind | On | Resource | Case chain |
|-|-|-|-|
| `rfi.drafted`, `rfi.submitted`, `rfi.released`, `rfi.rejected`, `rfi.withdrawn` | RFI lifecycle | RFI | yes (`case.rfi_drafted`, `case.rfi_submitted`, `case.rfi_released`, `case.rfi_rejected`; withdrawal is audit only) |
| `rfi.answer_recorded`, `rfi.claim_expired`, `rfi.extended` | claims and deadlines | RFI, claim | yes for answers and extensions (`case.rfi_answered`, `case.rfi_extended`) |
| `internal_report.received`, `internal_report.acknowledged` | register | internal report | `case.internal_report_linked` when linked |
| `authority_request.registered`, `authority_request.drafted`, `authority_request.disclosed` | register | authority request | `case.authority_linked` on each linked case |
| `watch.started`, `watch.triggered`, `watch.expired`, `watch.reviewed` | watch lifecycle | watch | while the case is open (`case.watch_started`, `case.watch_reviewed`); audit only once it is closed |
| `hibernation.started`, `hibernation.hit_recorded`, `hibernation.released`, `hibernation.ended` | hibernation and ledger | hibernation, hit | the start only (`case.hibernation_started`; the case is open); the end, the hits and the releases are audit only (the end takes no case lock, section 3.15 (d)) |
| `network.edges_rebuilt`, `network.kinship_shown`, `network.case_opened`, `network.snapshot_taken` | network | party, case, snapshot | yes on the parent (`case.network_opened`, `case.network_children_done`) |
| `qa.sample_drawn`, `qa.draw_summary`, `qa.grant_opened`, `qa.reviewed`, `qa.finding_recorded`, `qa.coaching_opened`, `qa.grant_revoked`, `qa.bulk_reopen` | QA | sample, review, finding | no (the QA grant is a `case_acl` row of reason `qa_review` and writes no case event, section 3.8 step 7) |
| `alarm.raised`, `alarm.notified`, `alarm.acknowledged`, `alarm.closed` | alarms | alarm | no |
| `clock.warning` (outbox, stage 50) | breach | owner | no |

Chain kinds are the case event kinds of phase 17 `case_events` (`case.*`); audit kinds are the names above. Every mutation of section 3 appears in one of these rows.

### 3.13 Cedar actions and the file `92_followup.cedar` (phase 03 bundle, range 92)

The bundle gets one new file, `92_followup.cedar` (range 92 follows phase 18's `91_cases.cedar`; phase 17's `90_alerts.cedar` and phase 18's file are not edited). Its entities and actions are appended to `schema.cedarschema` in the phase 03 form (the bundle hash changes with the file, phase 03 section 3.1). Roles are `principal.roles` (phase 03; no Role entity exists). Step-up is never a Cedar condition: the handlers call `StepUpService::consume` with the declared constants of section 4.5 item 11.

Schema additions (namespace `Akr`; `Case`, `Party`, `System`, `User` and `Ctx` are phase 17, 18 and 03 entities):

```cedar
// appended to schema.cedarschema by phase 19. `Rfi.assignees` is the case team (assignees, pool members and the access list,
// phase 18 section 3.10). `QaSample.reviewer` is the assigned reviewer (a User). `Alarm.kind` is the alarm kind of section 3.10.
entity Rfi = { tier: String, kind: String, status: String, external: Bool, assignees: Set<User> };
entity Alarm = { kind: String, state: String };
entity QaSample = { reviewer: User, state: String, is_calibration: Bool };
entity InternalReport = { tier: String };
entity AuthorityRequest = { tier: String };

action "rfi.draft", "rfi.read.case", "watch.write", "watch.review", "hibernation.create", "hibernation.end"
    appliesTo { principal: User, resource: Case, context: Ctx };
action "rfi.submit", "rfi.read", "rfi.release", "rfi.answer", "rfi.extend"
    appliesTo { principal: User, resource: Rfi, context: Ctx };
action "rfi.list", "watch.list", "hibernation.list", "authority.register", "authority.list", "internal_report.write",
       "internal_report.list", "qa.sample.draw", "qa.sample.read", "qa.rates.read", "qa.calibration.read",
       "qa.bulk_reopen", "alarm.list"
    appliesTo { principal: User, resource: System, context: Ctx };
action "network.read", "network.case.open" appliesTo { principal: User, resource: Party, context: Ctx };
action "authority.read" appliesTo { principal: User, resource: AuthorityRequest, context: Ctx };
action "internal_report.read", "internal_report.acknowledge" appliesTo { principal: User, resource: InternalReport, context: Ctx };
action "qa.item.read", "qa.review.write" appliesTo { principal: User, resource: QaSample, context: Ctx };
action "alarm.read", "alarm.acknowledge" appliesTo { principal: User, resource: Alarm, context: Ctx };
```

The file (every permit and forbid written out; one role per `@role`; one permit per role and resource type, because a Cedar permit names one resource type):

```cedar
// 92_followup.cedar (phase 19). Scope gates are API.md section 2 (queue.work, case.decide, rfi.desk, qa.review, audit.read);
// these policies are the fine gate. The four-eyes rule of a release is the workflow's (phase 04, task.approve through the
// eligibility of Holders) and the database guard `released_by <> proposer_id`; not a Cedar condition on the case.

// --- A1: drafts on the case, reads the team's RFIs, starts watches, reads the network and the ledgers ---
@id("followup.a1.case") @role("A1")
permit (principal, action in [Akr::Action::"rfi.draft", Akr::Action::"rfi.read.case", Akr::Action::"watch.write"], resource is Akr::Case)
when { principal.roles.contains("A1") };

@id("followup.a1.rfi.submit") @role("A1")
permit (principal, action == Akr::Action::"rfi.submit", resource is Akr::Rfi)
when { principal.roles.contains("A1") && resource.assignees.contains(principal) };

@id("followup.a1.party") @role("A1")
permit (principal, action == Akr::Action::"network.read", resource is Akr::Party)
when { principal.roles.contains("A1") };

@id("followup.a1.system") @role("A1")
permit (principal, action in [Akr::Action::"watch.list", Akr::Action::"hibernation.list"], resource is Akr::System)
when { principal.roles.contains("A1") };

// --- A2: as A1, and decides the watch review, hibernations, the authority register, internal reports and the alarms of
//     touch and work-item breaches of the pool or team ---
@id("followup.a2.case") @role("A2")
permit (principal, action in [Akr::Action::"rfi.draft", Akr::Action::"rfi.read.case", Akr::Action::"watch.write",
        Akr::Action::"watch.review", Akr::Action::"hibernation.create", Akr::Action::"hibernation.end"], resource is Akr::Case)
when { principal.roles.contains("A2") };

@id("followup.a2.rfi.submit") @role("A2")
permit (principal, action == Akr::Action::"rfi.submit", resource is Akr::Rfi)
when { principal.roles.contains("A2") && resource.assignees.contains(principal) };

@id("followup.a2.party") @role("A2")
permit (principal, action in [Akr::Action::"network.read", Akr::Action::"network.case.open"], resource is Akr::Party)
when { principal.roles.contains("A2") };

@id("followup.a2.system") @role("A2")
permit (principal, action in [Akr::Action::"watch.list", Akr::Action::"hibernation.list", Akr::Action::"authority.register",
        Akr::Action::"authority.list", Akr::Action::"internal_report.write", Akr::Action::"internal_report.list",
        Akr::Action::"alarm.list"], resource is Akr::System)
when { principal.roles.contains("A2") };

@id("followup.a2.authority") @role("A2")
permit (principal, action == Akr::Action::"authority.read", resource is Akr::AuthorityRequest)
when { principal.roles.contains("A2") };

@id("followup.a2.report") @role("A2")
permit (principal, action in [Akr::Action::"internal_report.read", Akr::Action::"internal_report.acknowledge"],
        resource is Akr::InternalReport)
when { principal.roles.contains("A2") };

@id("followup.a2.alarm") @role("A2")
permit (principal, action in [Akr::Action::"alarm.read", Akr::Action::"alarm.acknowledge"], resource is Akr::Alarm)
when { principal.roles.contains("A2") && (resource.kind == "touch_breach" || resource.kind == "work_item_breach") };

// --- M: the MLRO. Decides RFI releases, external and suspicion RFIs included; reads every register; bulk reopen; rates ---
@id("followup.m.case") @role("M")
permit (principal, action in [Akr::Action::"rfi.read.case", Akr::Action::"watch.write", Akr::Action::"watch.review",
        Akr::Action::"hibernation.create", Akr::Action::"hibernation.end"], resource is Akr::Case)
when { principal.roles.contains("M") };

@id("followup.m.rfi") @role("M")
permit (principal, action in [Akr::Action::"rfi.read", Akr::Action::"rfi.release", Akr::Action::"rfi.answer",
        Akr::Action::"rfi.extend"], resource is Akr::Rfi)
when { principal.roles.contains("M") };

@id("followup.m.party") @role("M")
permit (principal, action in [Akr::Action::"network.read", Akr::Action::"network.case.open"], resource is Akr::Party)
when { principal.roles.contains("M") };

@id("followup.m.system") @role("M")
permit (principal, action in [Akr::Action::"rfi.list", Akr::Action::"watch.list", Akr::Action::"hibernation.list",
        Akr::Action::"authority.register", Akr::Action::"authority.list", Akr::Action::"internal_report.write",
        Akr::Action::"internal_report.list", Akr::Action::"qa.sample.draw", Akr::Action::"qa.sample.read",
        Akr::Action::"qa.rates.read", Akr::Action::"qa.calibration.read", Akr::Action::"qa.bulk_reopen",
        Akr::Action::"alarm.list"], resource is Akr::System)
when { principal.roles.contains("M") };

@id("followup.m.authority") @role("M")
permit (principal, action == Akr::Action::"authority.read", resource is Akr::AuthorityRequest)
when { principal.roles.contains("M") };

@id("followup.m.report") @role("M")
permit (principal, action in [Akr::Action::"internal_report.read", Akr::Action::"internal_report.acknowledge"],
        resource is Akr::InternalReport)
when { principal.roles.contains("M") };

@id("followup.m.alarm") @role("M")
permit (principal, action in [Akr::Action::"alarm.read", Akr::Action::"alarm.acknowledge"], resource is Akr::Alarm)
when { principal.roles.contains("M") };

@id("followup.m.qa.item") @role("M")
permit (principal, action == Akr::Action::"qa.item.read", resource is Akr::QaSample)
when { principal.roles.contains("M") };

// --- RFI desk (role RFI): the internal releases, the answers and the extensions; never a case word (section 3.14) ---
@id("followup.rfi.desk") @role("RFI")
permit (principal, action in [Akr::Action::"rfi.read", Akr::Action::"rfi.answer", Akr::Action::"rfi.extend"], resource is Akr::Rfi)
when { principal.roles.contains("RFI") };

@id("followup.rfi.release.internal") @role("RFI")
permit (principal, action == Akr::Action::"rfi.release", resource is Akr::Rfi)
when { principal.roles.contains("RFI") && !resource.external };

@id("followup.rfi.list") @role("RFI")
permit (principal, action == Akr::Action::"rfi.list", resource is Akr::System)
when { principal.roles.contains("RFI") };

// --- QA: draws and reads the samples, reviews the items drawn to them; calibration reads. A reviewer is the assigned
//     person whatever the role (CH and IA review MLRO decisions, M-49); the assignment is checked at the draw. ---
@id("followup.qa.draw") @role("QA")
permit (principal, action in [Akr::Action::"qa.sample.draw", Akr::Action::"qa.sample.read", Akr::Action::"qa.calibration.read"],
        resource is Akr::System)
when { principal.roles.contains("QA") };

// CH and IA review the MLRO decisions drawn to them (M-49; API.md section 2 gives them `qa.review`, lead review). They list
// their own samples; the service filters the list to the rows where they are the reviewer.
@id("followup.ch.samples") @role("CH")
permit (principal, action == Akr::Action::"qa.sample.read", resource is Akr::System)
when { principal.roles.contains("CH") };

@id("followup.ia.samples") @role("IA")
permit (principal, action == Akr::Action::"qa.sample.read", resource is Akr::System)
when { principal.roles.contains("IA") };

@id("followup.reviewer.item")
permit (principal, action == Akr::Action::"qa.item.read", resource is Akr::QaSample)
when { resource.reviewer == principal };

@id("followup.reviewer.write")
permit (principal, action == Akr::Action::"qa.review.write", resource is Akr::QaSample)
when { resource.reviewer == principal && resource.state == "open" };

// --- Forbids (the phase 17 and 18 rules applied to the new actions) ---

// A suspicion RFI is the MLRO's or a member of the case team's; the RFI desk never sees it (DR-149).
@id("followup.forbid.suspicion.rfi")
forbid (principal, action in [Akr::Action::"rfi.read", Akr::Action::"rfi.submit", Akr::Action::"rfi.release",
        Akr::Action::"rfi.answer", Akr::Action::"rfi.extend"], resource is Akr::Rfi)
when { resource.tier == "suspicion" && !(principal.roles.contains("M") || resource.assignees.contains(principal)) };

// Suspicion RFIs and cases: the analyst network only.
@id("followup.forbid.suspicion.network.rfi")
forbid (principal, action in [Akr::Action::"rfi.read", Akr::Action::"rfi.submit", Akr::Action::"rfi.release",
        Akr::Action::"rfi.answer", Akr::Action::"rfi.extend"], resource is Akr::Rfi)
when { resource.tier == "suspicion" && context.network != "analyst" };

// The RFI desk releases internal requests only; an external release (`resource.external`: RM request, customer outreach,
// correspondent, authority response, contact allowed,
// or any RFI of a suspicion case, which is external by the workflow) needs the MLRO.
@id("followup.forbid.external.release")
forbid (principal, action == Akr::Action::"rfi.release", resource is Akr::Rfi)
when { !principal.roles.contains("M") && resource.external };

@id("followup.forbid.suspicion.case")
forbid (principal, action in [Akr::Action::"rfi.draft", Akr::Action::"rfi.read.case", Akr::Action::"watch.write",
        Akr::Action::"watch.review", Akr::Action::"hibernation.create", Akr::Action::"hibernation.end"], resource is Akr::Case)
when { resource.tier == "suspicion" && !(principal.roles.contains("M") || resource.assignees.contains(principal)) };

@id("followup.forbid.suspicion.network.case")
forbid (principal, action in [Akr::Action::"rfi.draft", Akr::Action::"rfi.read.case", Akr::Action::"watch.write",
        Akr::Action::"watch.review", Akr::Action::"hibernation.create", Akr::Action::"hibernation.end"], resource is Akr::Case)
when { resource.tier == "suspicion" && context.network != "analyst" };

// A hibernation of a suspicion party is the MLRO's (section 3.6 step 6).
@id("followup.forbid.hibernation.suspicion")
forbid (principal, action == Akr::Action::"hibernation.create", resource is Akr::Case)
when { resource.tier == "suspicion" && !principal.roles.contains("M") };

// Party actions of the network: the phase 18 party rules, applied to network.read and network.case.open.
@id("followup.forbid.suspicion.party")
forbid (principal, action in [Akr::Action::"network.read", Akr::Action::"network.case.open"], resource is Akr::Party)
when { resource.tier == "suspicion" && !(principal.roles.contains("M") || resource.case_team.contains(principal)) };

@id("followup.forbid.suspicion.network.party")
forbid (principal, action in [Akr::Action::"network.read", Akr::Action::"network.case.open"], resource is Akr::Party)
when { resource.tier == "suspicion" && context.network != "analyst" };

@id("followup.forbid.purpose.party")
forbid (principal, action in [Akr::Action::"network.read", Akr::Action::"network.case.open"], resource is Akr::Party)
when { resource.tier != "normal" && !(context.purpose == "assigned_work" || context.purpose == "investigation") };

// QA items are read from the analyst network with the QA purpose only; a review is written from an open sample only.
@id("followup.forbid.qa.item.network")
forbid (principal, action == Akr::Action::"qa.item.read", resource is Akr::QaSample)
when { context.network != "analyst" || context.purpose != "qa_review" };

@id("followup.forbid.qa.review.closed")
forbid (principal, action == Akr::Action::"qa.review.write", resource is Akr::QaSample)
when { resource.state != "open" };

// Alarms of a chain or a conflict are read by the audit roles and the CISO only, in addition to the MLRO.
@id("followup.alarm.audit.chain")
permit (principal, action == Akr::Action::"alarm.read", resource is Akr::Alarm)
when { (principal.roles.contains("IA") || principal.roles.contains("SEC")) && resource.kind == "chain_broken" };

@id("followup.alarm.sec.conflict")
permit (principal, action == Akr::Action::"alarm.read", resource is Akr::Alarm)
when { principal.roles.contains("SEC") && resource.kind == "conflict_declared" };
```

Notes on the file:
- Default deny covers everything not permitted: a bulk reopen by A2 (`qa.bulk_reopen` is M only), a draft by M (`rfi.draft` is A1 and A2), an external release by the RFI desk, a review by a non-reviewer, every action for KYC, CFG, DS, CHK, MRO, ADM, SEC (except the alarm reads above), and for CH and IA beyond their own samples (IA also reads the chain alarm).
- The API scope `qa.review` admits QA, M, CH and IA (API.md section 2, widened in the phase 19 review for M-49; DR-155). CH and IA list their own samples and review the items drawn to them; the reviewer permits do not depend on a role.
- `resource.assignees` of `Rfi` is the team of the case, set by the entity adapter at request time (phase 18 section 3.10 rule); the phase 18 policies are unchanged.
- The visibility predicate of section 3.2 and the Cedar `Rfi` permits agree: a test pins them over generated principals (section 6, check 5).

### 3.14 Tipping-off review (M-43, M-44, M-68; the answer must be "nothing")

| Surface | What a person outside the case's access list can learn |
|-|-|
| `POST /cases/{id}/rfis`, `GET /cases/{id}/rfis`, `POST /rfis/{id}/submit`, `POST /rfis/{id}/extend` | 404 for a case the caller cannot see; nothing about other cases |
| `GET /rfis`, `GET /rfis/{id}` (RFI desk) | the RFI's kind, status, deadline, recipient kind and question keys. No case id, no family, no priority, no tier, no party, no decision text. A suspicion RFI is a masked 404 for every caller but M and the team |
| `POST /rfis/{id}/release` (RFI desk, MLRO) | the release decision of the task (the approver's id to the approver's own inbox only); the inbox title is neutral ("Information request") |
| `POST /rfis/{id}/answers` | the answer values to the desk and the team only (sealed); the claim's source kind and corroboration to the desk |
| The RFI text (template) | the pack's neutral templated words; the scan of scope `rfi` refuses a case word; the purpose code is a neutral code (`KYC-CLARIFY`, `DOC-CHECK`) |
| The branch's view (liaison) | the question keys and the deadline; no case, no reason code, no decision (phase 22 renders the branch view; the rule is recorded here) |
| `POST /internal-reports`, `GET /internal-reports`, `POST /internal-reports/{id}/acknowledge` | the reporter's identity to M only (M-68); the body sealed to the register readers |
| `POST /authority-requests`, `GET /authority-requests*` | the suspicion-tier register for A2 and M; the authority's reference is `secret-str`; no subject name in an RFI |
| `GET /watches`, `POST /cases/{id}/watches`, `POST /watches/{id}/review` | the watch and its trigger keys to the case team; a closed case shows the trigger count only to the team |
| `GET /hibernations`, `POST /cases/{id}/hibernations`, `POST /hibernations/{id}/end` | hit counts and regret shares only to the case team and M; the hibernated scenario's id to the team |
| Suppressed alerts | nothing in any queue, count, search or SSE event until the hit is released; the hit ledger is the team's and M's |
| `GET /parties/{id}/graph` | visible nodes and edges under the phase 18 party predicate; an invisible neighbour is omitted with no count and no placeholder; kinship only with the purpose and the step-up, logged |
| `POST /parties/{id}/network-cases` | the parent and the children the caller may see; a child of an invisible subject is not created or named |
| Network snapshot (evidence) | the team of the parent only; kinship excluded unless the viewer showed it (logged) |
| `GET /qa/samples`, `GET /qa/samples/{id}/item`, `POST /qa/samples/{id}/review` | the reviewer's own samples; the item shows the decision or alert summary, the reason codes and evidence labels, never a party name, an account or a contact (one exception from phase 28 on: the verified output of an `assistant_draft` item may name parties of its case, section 7+) |
| `GET /qa/rates`, `GET /qa/calibration` | reviewer rates to M and calibration to QA and M; no item identity to other reviewers |
| `POST /qa/bulk-reopen` | the reopened cases to M; each reopen is phase 18's audit of the case |
| `GET /alarms` and `POST /alarms/{id}/acknowledge` | the alarm kind, subject id and counts by kind; a chain alarm to IA, SEC and M; a conflict alarm to SEC and M; no case text |
| Outbox events of this phase (`authority_request.recorded`, `internal_report.recorded`, `rfi.overdue`, `watch.triggered`, `hibernation.ended`, `alarm.raised`, `network.snapshot_requested`, `graph.link`) | ids and codes only (phase 15 and phase 22 read them); no name, value or text |
| Audit viewer (phase 24) | event kinds and ids, counts, the digest of a draw; no text, no name, no tier value |

### 3.15 Lock order (phase 18 section 3.16 extended; DR-156 and DR-153)

1. The workflow instance row and its tasks (`WorkflowEngine::act`, `start`; phase 04 invariant I1).
2. The case row(s): `CaseRepository::lock(tx, case_id)`. When several cases are locked (a network parent and its children, the cases of a QA draw, a bulk reopen), they are locked in ascending `id`, as phase 17's merger does.
3. Below the case row, in this order: `casework.rfis`, `casework.rfi_questions`, `casework.rfi_answers`, `casework.authority_requests`, `casework.internal_reports`, `casework.watches`, `casework.hibernations` (the row lock for an end or a regret mark, never with a case row taken after it), `casework.hibernation_hits`, `casework.alarms`, `casework.qa_samples`.
4. Below the samples: `casework.case_acl` (the QA grant and its revocation), `casework.work_items`, `casework.case_events` (the chain under the lock of step 2), `casework.network_edges` and `casework.network_snapshots` (no case lock is taken under them).

Rules of the paths of this phase: (a) the RFI release is step 1 then step 2 then step 3 (the instance, the case, the RFI); (b) the claim, extension and submit paths take step 2 then step 3; (c) the gate of section 3.6 takes no lock (it reads the active hibernation and inserts a hit with a foreign key that takes a KEY SHARE lock on the hibernation row; no path holds a case row while it waits for an UPDATE of that row); (d) the hibernation end and the regret mark take step 3 only; the release of a hit (job `hibernation_release`) takes step 2 then step 3 in its own transaction, never the reverse; (e) the review of a sample takes the case row (2), the sample row (3), then the grant (4); the draw takes the cases in ascending id order (2) before the sample rows (3) and the grants (4); (f) the alarm raise takes no case lock (its `case_id` is a reference only) except when a case row is already held by the caller; (g) the watch trigger job takes the case (2) then the watch (3) then the work item (below the case).

No path takes a lower-ranked lock and then a higher one; the deadlock test of section 6 runs the two races that matter (a QA review against a closure of the same case, and a hibernation end against a hit release).

### 3.16 Configuration keys used (admin-managed data, never constants)

The keys are the rows of section 1.2 (`casework.watch_days`, `casework.hibernation_default_days`, `casework.hibernation_margin`, `casework.hibernation_max_days` (in the table of section 1.2 as the bound `period_days <= 180`), `casework.rfi_internal_days`, `casework.rfi_outreach_days`, `casework.rfi_extension_days`, `casework.rfi_claim_expiry_days`, `casework.authority_default_days`, `casework.internal_report_ack_hours`, `casework.network_max_children`, `casework.network_transfer_min_confidence`, `casework.network_supernode_degree`, `qa.confidence`, `qa.expected_defect_rate`, `qa.review_working_days`, `qa.grant_days`, `qa.suppression_sample_percent`, `qa.forced_low_score_daily`, `qa.min_per_stratum`, `qa.min_reviewer_items`, `qa.coaching_working_days`, `qa.flat_line_days`, `qa.bulk_reopen_max`) and the phase 17 and phase 09 keys already named (`casework.touch_by_hours`, `casework.work_item_due_hours`, `casework.reopen_window_days`, the pack clock `rfi.response` and switch `rfi.pre_fill`). The RFI template text and the question keys are in the pinned policy pack, not in a setting.


## 4. Infrastructure Updates

### 4.1 Job kinds (Postgres `jobs`; coalesce key in brackets)

| Kind | Binary | Payload | Concurrency, attempts | Schedule or trigger |
|-|-|-|-|-|
| `timer_rfi_clock` | worker | timer row (owner `rfi`, kind `rfi_clock`) [rfi id] | 2, 3 | fired by the phase 00 timer sweep at each stage of section 3.2 step 5; handler: re-read the RFI; `reminder_*` on a `released` RFI with open questions writes outbox `rfi.reminder`; `due` on a `released` RFI writes outbox `rfi.overdue` (the MLRO desk, never the branch manager, M-43); no status change |
| `timer_breach_stage` | worker | timer row (owner `alert` or `work_item`, kind `breach_stage`) [owner id and stage] | 4, 3 | armed by `BreachService::arm` (section 3.10 step 1); handler: stage 50 writes outbox `clock.warning`; stage 150 raises the alarm of section 3.10 when the owner is still breached |
| `timer_watch_expiry` | worker | timer row (owner `watch`, kind `watch_expiry`) [watch id] | 2, 3 | fired at `expires_at`, scheduled by `WatchService::start_for_decision` and by `extend`; handler: section 3.5 expiry |
| `timer_hibernation_expiry` | worker | timer row (owner `hibernation`, kind `hibernation_expiry`) [hibernation id] | 2, 3 | fired at `expires_at`, scheduled by `HibernationService::create`; handler: end with `expired` (section 3.6) |
| `timer_qa_grant_expiry` | worker | timer row (owner `qa_sample`, kind `qa_grant_expiry`) [sample id] | 2, 3 | fired at `now() + qa.grant_days`, scheduled by the draw (section 3.8 step 7); handler: revoke and void (section 3.9) |
| `timer_intake_clock` | worker | timer row (owner `internal_report` or `authority_request`, kind `intake_clock`) [owner id] | 2, 3 | fired at `ack_due_at` or `deadline_at`; handler: writes outbox `internal_report.overdue` or `authority_request.overdue` when the owner is still unacknowledged or unanswered; no status change |
| `watch_trigger_dispatch` | worker | outbox event (topic and id) [party id] | 4, 5 | outbox consumer of `screening.hit`, `authority_request.recorded`, `canon.touched` and `alert.created` (section 4.2); runs the triggers of sections 3.5 and 3.6 per party |
| `hibernation_release` | worker | `{hibernation_id}` [hit id] | 4, 5 | enqueued per open hit by the outbox event `hibernation.ended`; one transaction per hit (section 3.6, lock order 3.15 (d)) |
| `hibernation_envelope_check` | worker | `{hibernation_id}` [hibernation id] | 2, 3 | enqueued by the outbox event `hibernation.envelope_exceeded` (section 3.6); ends the hibernation with `envelope_exceeded` in its own transaction |
| `hibernation_regret` | worker | `{}` [`hibernation_regret`] | 1, 3 | `schedule.hibernation_regret` (daily 04:45); marks `report_after` (section 3.6) |
| `rfi_clock_scan` | worker | `{}` [`rfi_clock_scan`] | 1, 3 | `schedule.rfi_clock_scan` (hourly at minute 35); expires claims (section 3.3); re-arms the timers of released RFIs whose timer row is missing (backstop) |
| `network_edge_build` | worker | `{batch_id, party_ids}` [`network_edges:<batch_id>`] | 2, 3 | outbox `canon.touched` (phase 11) per batch; the sweep `schedule.network_edge_build` (daily 02:20) rebuilds every party changed since the last build (section 3.7) |
| `network_snapshot_seal` | worker | `{case_id, seed_party_id, viewer_id}` [`network_snapshot:<case_id>`] | 2, 3 | outbox `network.snapshot_requested` written by the network-case route (section 3.7 step 5); builds the graph under the viewer's rules and calls `EvidenceStore::put_generated` |
| `flat_line_check` | worker | `{}` [`flat_line_check`] | 1, 3 | `schedule.flat_line_check` (daily 04:30); section 3.10 |
| `qa_forced_sample` | worker | `{}` [`qa_forced_sample:<date>`] | 1, 3 | `schedule.qa_forced_sample` (daily 03:10); the population `low_score_forced` of section 3.8 (M-16) |
| `qa_suppression_sample` | worker | `{}` [`qa_suppression_sample:<iso week>`] | 1, 3 | `schedule.qa_suppression_sample` (Monday 06:00); the population `screening_suppressions` (5 per cent, phase 14 suppression records) |
| `qa_grant_sweep` | worker | `{}` [`qa_grant_sweep`] | 1, 3 | `schedule.qa_grant_sweep` (hourly at minute 50); revokes grants whose timer did not fire (section 3.9) |

Every handler re-reads its owner first (phase 00 rule). Nothing in this phase is a `decision` job: every decision, release, review and reopen runs in the request transaction through the workflow or the service (akr-api), so a decision never waits in a queue. Phase 19 adds no job prefix (`ingest_` is for akr-ingestd only). Seventeen job kinds: six timer jobs, eleven scheduled or event jobs.

### 4.2 Timers, outbox topics and the consumers

- **Timer kinds** (phase 00 `TimerService`; registered in the phase 00 timer kind registry by this phase, section 4.5 item 14): `rfi_clock` (owner `rfi`), `breach_stage` (owners `alert` and `work_item`), `watch_expiry` (owner `watch`), `hibernation_expiry` (owner `hibernation`), `qa_grant_expiry` (owner `qa_sample`), `intake_clock` (owners `internal_report` and `authority_request`). Each fires at an absolute instant; the bank calendar was applied when the instant was computed (`BusinessClock::add`, `calendar_id` and `calendar_version` stored on the timer).
- **Outbox topics produced** (phase 00 `TopicHandler` registry; payloads carry ids and codes only):

| Topic | Payload | Consumer (outside this phase) |
|-|-|-|
| `authority_request.recorded` | `{authority_request_id, party_ids}` | phase 15 (TR-17, immediate review of the subjects); this phase (section 3.5 and 3.6 triggers) |
| `internal_report.recorded` | `{internal_report_id, party_ids}` | phase 15 (suspicion-linked trigger `TRG_REVIEW_REQUESTED`, section 3.3 of phase 15) |
| `internal_report.overdue`, `authority_request.overdue`, `rfi.overdue`, `rfi.reminder` | ids and the stage | phase 22 (notice to the MLRO desk) |
| `watch.triggered`, `watch.expired` | `{watch_id, trigger_key}` | phase 22 |
| `hibernation.ended`, `hibernation.envelope_exceeded` | `{hibernation_id, reason}` | this phase (jobs `hibernation_release` and `hibernation_envelope_check`) |
| `alarm.raised` | `{alarm_id, kind}` | phase 22 (supervisor notice) |
| `network.snapshot_requested` | `{case_id, seed_party_id, viewer_id}` | this phase (job `network_snapshot_seal`) |
| `graph.link` | `{party_id, kind}` | phase 15 (section 3.3 of phase 15, trigger `graph.link`) |
| `outreach.failed` | `{party_id, rfi_id, prior_failures}` (7.9) | phase 15 (TR-20 trigger) |

- **Outbox topics consumed** (handlers registered in the phase 00 registry by this phase): `screening.hit` (phase 14; the event name of DSL.md C.5, whose producer is phase 14; Open doubts 9), `authority_request.recorded`, `canon.touched` (phase 11 outbox topic), `alert.created` (phase 17 section 4.2), and `hibernation.envelope_exceeded`. A failing handler backs off under the phase 00 rule; every handler here is idempotent (a repeat trigger adds a `trigger_count` and an audit event only when the key is new within the same second).
- **Clock event:** `clock.warning` is the existing SSE event of the API (API.md section 1, the SSE event list) carrying the breach stage 50 (section 3.10 step 3).

### 4.3 AppState additions

- `akr-api`: `rfis: Arc<RfiService>` (drafting, submit, release, answers, extend, KPIs), `workflow_subjects` (the phase 04 `subjects` registry gains `RfiWorkflowSubject`), `registers: Arc<RegisterService>` (`InternalReportService` and `AuthorityRequestService`), `watches: Arc<WatchService>`, `hibernations: Arc<HibernationService>` and `hibernation_gate: Arc<HibernationGate>` (the port of phase 17 intake), `network: Arc<NetworkService>` (graph, network cases; the `EdgeBuilder` is a worker type), `qa: Arc<QaService>`, `alarms: Arc<AlarmService>`, `breach: Arc<BreachService>`. The phase 04 `effects` registry gains `ReleaseRfiEffect` (kind `release_rfi`, resource kind `rfi`).
- `akr-worker`: `edge_builder: Arc<EdgeBuilder>`, `snapshot_sealer: Arc<NetworkSnapshotSealer>` (the phase 02 `EvidenceStore` already in worker state), `watch_dispatch: Arc<WatchTriggerDispatcher>`, `hibernation_release: Arc<HibernationReleaser>`, `qa_sampler: Arc<QaService>` (the scheduled draws), `flat_line: Arc<FlatLineDetector>`, and the handlers of the timer and job kinds of section 4.1.

### 4.4 Network zones, metrics, readiness, OpenAPI

- **Zones**: no new edge. The API uses PostgreSQL and the audit writer as before. Snapshots are sealed by the worker (the phase 02 worker edge to the evidence store); the API never calls `put_generated`.
- **Metrics**: `akr_rfi_releases_total{outcome}` (`released`, `rejected`, `withdrawn`), `akr_rfi_overdue_total{kind}`, `akr_watch_triggers_total{key}`, `akr_hibernation_hits_total`, `akr_hibernation_envelope_exceeded_total`, `akr_alarms_open{kind}`, `akr_qa_samples_drawn_total{population}`, `akr_qa_unassigned_total{population}`, `akr_network_edges_live`. No metric carries a case id, a party, a principal, a name, a tier value or a text.
- **Readiness**: no new check.
- **OpenAPI**: tags `rfis` (drafting, submit, desk, release, answers, extend), `watches`, `hibernations`, `requests` (authority requests and internal reports), `network` (graph and network cases, registered under the phase 18 `parties` router), `qa`, `alarms`. `make generate-api` regenerates the client in this change.

### 4.5 Existing entity, schema, route and code updates (extensions of earlier phases)

1. **Phase 04**: `WorkflowSubject` implementation `RfiWorkflowSubject` (section 3.2; the `check_before_decision` hook of phase 18 section 4.5 item 1); `TerminalEffect` implementation `ReleaseRfiEffect` (kind `release_rfi`, resource kinds `["rfi"]`); the seeded definitions `rfi_release_internal` and `rfi_release_external` (section 3.2 table) through `akr-ctl workflow seed` with `resource_kinds: ["rfi"]`, `classification_default: "restricted"` and the timeouts of section 3.2; DSL E.4 gains the two rows by a `cr_workflow` change after the seed (Open doubts 2).
2. **Phase 04 `Holders`**: no change (the `named_officer` rule 3 and the `role` rule 1 are used as they are).
3. **Phase 09**: no new clock or decision type: `rfi.response` (pack), `rfi.release` (authority matrix, scope `rfi.desk`) and the reason-code family `closure` (alarm closure) are used as they are. The pack document gains the RFI templates (the six kinds and the template `authority_response.v1`) and its purpose-code list; the switch `rfi.pre_fill` is `false` in the seed. The pack is a document of the bank's configuration (M-75), not a code change.
4. **Phase 17 extensions** (each is a call or a clause; no column of phase 17 changes):
   - `AlertIntakeService` (section 3.1): a new step 4b, between steps 4 and 5, calls `HibernationGate::check(conn, party_id, scenario_id, measures, now)`. On `Suppress(h)`: steps 6, 8 and 11 run and the hit row is inserted after step 6 (section 3.6); steps 5, 7, 9, 10 and 12 are skipped (no case is opened or joined, no clock runs, and no case event and no `alert.created` are written, so no watch, count or SSE event sees the alert). Step 9 calls `BreachService::arm(tx, "alert", id, created_at, touch_hours)`. New method `attach_released(tx, alert_id, alert_created_at)` runs steps 5, 7, 9, 10 and 12 for the existing alert (used by `hibernation_release`; the clocks start at the release, not at the detection).
   - The queue query (section 3.9) and the counts (section 3.11) and the visibility predicate (section 3.10) gain the clause `AND NOT EXISTS (SELECT 1 FROM casework.hibernation_hits h WHERE h.alert_id = a.id AND h.alert_created_at = a.created_at AND h.released_at IS NULL)`; `BreachService::breach` skips an alert with an open hit.
   - The touch handler `timer_alert_touch` (section 3.12) calls `BreachService::breach(tx, "alert", id, 100)` with its breach write.
   - `AssignmentService` (section 3.8) raises `conflict_declared` through `AlarmService::raise` on a declared conflict (phase 17 open doubt 16 closed here).
   - `case_chain_check` (section 4.1) raises `chain_broken` through `AlarmService::raise` for each break (phase 17 open doubt 22 closed here).
   - The merge validation of phase 17 section 3.6 refuses a source case with a `released` RFI (sub-code `rfi_open`, 7.10 step 3).
5. **Phase 18 extensions**:
   - `RecordDecisionEffect` (section 3.8 step f): after step 8 (the tier), for a `utr` decision, `WatchService::start_for_decision(tx, decision, case)` in the same transaction (the case row is already locked by the effect; section 3.5).
   - `CaseService::close` (section 3.12): the closure gate of section 3.3 (sub-code `uncorroborated_explanation`); and the network hook of section 3.7 step 7 (the child and its parent are locked in ascending id order).
   - New entry point `CaseService::reopen_under_batch(tx, case_id, reason_code, actor, finding_id)` for the bulk reopen of section 3.9: the reopen rules of section 3.13 of phase 18 with the batch assertion of `POST /qa/bulk-reopen` in place of a second one (the batch's step-up is consumed once by the route, phase 01 `StepUpService`).
   - `WorkItemService::open` (section 3.6) calls `BreachService::arm(tx, "work_item", id, now, casework.work_item_due_hours)`; the handler of `timer_work_item_due` calls `BreachService::breach(tx, "work_item", id, 100)` with its overdue write (phase 18 open doubt 11 closed here).
   - The lock order of section 3.16 of phase 18 is extended by section 3.15 of this phase (the rank 3 and rank 4 lists).
   - Sub-codes of section 3.11 of this phase are added to the closed list of phase 18 section 3.17 (Open doubts 15).
   - The phase 18 party routes gain the graph and network-case routes (`api::routes::parties`); the phase 18 `GET /cases/{id}` is unchanged.
6. **Phase 15**: the outbox topics `authority_request.recorded`, `internal_report.recorded` and `graph.link` are produced by this phase (section 4.2), as section 3.3 of phase 15 requires of the producing phase; the consumer code is phase 15's.
7. **Phase 14**: the pair-memory weekly sample (phase 14 section 3.7 sweep) is drawn by this phase's job `qa_suppression_sample` (section 4.1), not by a draw in phase 14 (one sentence changes in gap pass 1; the suppression records are phase 14's).
8. **Phase 11**: the `canon.touched` consumer (`network_edge_build`) and the watch's `profile_change` trigger are registered here; the payload of phase 11 is unchanged.
9. **Phase 02**: the evidence owner registry gains `network_snapshot` (the snapshot's `put_generated`, worker only, kind `snapshot`, retention `R6`). The authority and RFI evidence uses the phase 18 `case` owner kind (the upload route of phase 18).
10. **Phase 03**: the bundle gains `92_followup.cedar` and the entities and actions of section 3.13, appended to `schema.cedarschema`. `MASKED_KINDS` gains the masked kind `rfi` (the tier of its case; the probe set of section 6, check 9); no other kind is added.
11. **Phase 01**: the step-up action constants of this phase: `rfi.release` (class `approval`), `authority.register` (class `decision`), `hibernation.create` (class `decision`), `watch.review` (class `decision`), `qa.bulk_reopen` (class `decision`), `qa.item.open` (class `record_open`). The constants `party.open` and `case.decide` are phase 18's. The closed class list is unchanged.
12. **Phase 16**: no change. The scenario version's document field `suppression.hibernation.allowed` (DSL A.7) is read by the create route; the alert `measures` of phase 17 are read by the gate.
13. **Phase 10**: no change to any table; the reads of section 3.7 (`canon.party`, `canon.party_contact`, `canon.party_relationship`, `canon.account_party_role`, `canon.payment_link`, `canon.txn`) use the existing SELECT grants.
14. **Phase 00**: the six timer kinds, the seventeen job kinds (section 4.1), the seven schedules and the settings of section 1.2 are registered.
15. **DATA-MODEL section 12**: the fourteen tables of section 1, with the reconciliation of section 1 (DR-157); the grant paragraph of section 12 gains a "Phase 19 grants" paragraph that mirrors section 1.2. **Roadmap row 19** gains the tables `casework.rfi_questions`, `casework.rfi_answers`, `casework.hibernation_hits`, `casework.alarms` and `casework.network_edges` (named as DR-145 renamed it) and the services of section 2; the roadmap row 18 note "network edges are phase 19's" stays.
16. **API.md**: section 3.12 marks the 19 owned rows "phase 19" (the ownership sentence of section 3.12 already names phase 19) and gains the 13 added rows marked "(phase 19, added)"; section 2 gains CH and IA on `qa.review` (DR-155). API.md section 6 is unchanged.
17. **Routes and screens**: the screens of section 5 are catalogue entries of DESIGN-SYSTEM section 8.1 (W6, W8, W10, W11, G3 are already listed there); no screen id is added.

## 5. Frontend Impact

Screens (catalogue IDs from DESIGN-SYSTEM section 8.1; the layout rules of DESIGN-SYSTEM section 7 apply). The server decides every visibility (sections 3.13 and 3.14); the client renders what it receives and never reads a hidden field from a cache.

### 5.1 W6 Network view (`/parties/:id/network`, WORK; UX-10, UX-61, DESIGN-SYSTEM 7.16)

- Ego graph with `@xyflow/react` (tech-stack.md) from `GET /parties/{id}/graph`; automatic grouping above about 15 accounts with one-click expansion and a group summary; the subject is never grouped away.
- The summary sentence is the server's `summary` field (section 3.7 step 5), never a client sum.
- A node and link table is the equal alternative; keyboard traversal of nodes; dragging has buttons and keyboard moves (WCAG 2.5.7).
- Kinship is off by default. The "Show kinship" control opens a purpose choice and the step-up dialog (phase 01); the response is shown with the words "Kinship shown, this view is logged" and the purpose.
- "Open one case for the cluster" calls `POST /parties/{id}/network-cases` (after a confirmation that names the number of children); on success the screen links to the parent case in W3.
- The Arabic name toggle masks names by default; the node labels are the account numbers' masked form (never the full number).
- An invisible neighbour is absent: no placeholder, no count.

### 5.2 W8 RFI composer and RFI desk (`/rfi`, LIST + FORM; DESIGN-SYSTEM 8.1; correspondents P2 for the desk)

- **Desk** (role RFI and M): a list of pending and released RFIs with the deadline, the overdue marker (words, not colour alone), the kind label (neutral), the recipient kind; the `kpis` strip (median and 90th percentile days, share by deadline, follow-up rounds, top question keys, missed by monitoring). No case id, family, priority or tier is shown (section 3.14).
- **Detail:** the questions in template order with the "why this is asked" line of the pack; each answer as a claim: source, corroboration (none, partly, documented), expiry date; the "Record answer" form writes `POST /rfis/{id}/answers`; the "Extend deadline" button is shown while `extensions < 2`.
- **Release** panel: the second signature with step-up (`POST /rfis/{id}/release`) and the four-eyes message when the viewer drafted the RFI (the server refuses it; the client does not hide the refusal reason).
- **Composer** (inside W3 for the case team, `POST /cases/{id}/rfis`): kind chooser with the six neutral labels; the template preview from the pinned pack; the recipient picker lists liaison officers of the branch only (no branch manager); the contact flag can be set on a normal-tier case only, and the form says that an RFI with contact allowed is signed by the MLRO; the submit button is `POST /rfis/{id}/submit`.

### 5.3 W10 Watches and hibernations (`/watches`, REGISTER; DESIGN-SYSTEM 8.1)

- **Watches tab:** rows with the kind (UTR), the triggers as chips (list hit, authority request, profile change, new alert), the trigger count and the last trigger time, the expiry date and the review due date; a row under review opens the review dialog with three choices (extend, close, upgrade to a report outcome) and the step-up dialog. The upgrade choice explains in one sentence that a report outcome is a new proposal of the case.
- **Hibernations tab:** the ledger with the scenario label, the case link, the envelope (the measures and the margin in words), the expiry, the hits, the released hits and the regret share; the "End hibernation" button asks for a reason of 5 to 200 characters and is shown to A2 and M.
- The start of a hibernation is a button in the W7 decision dialog after a no-report decision (`POST /cases/{id}/hibernations`, step-up).
- Empty states name the next step ("No open watches"; "No hibernations in the last 180 days").

### 5.4 W11 Internal reports and authority requests (`/requests`, REGISTER; DESIGN-SYSTEM 8.1)

- **Authority requests tab:** the register with the authority kind, the deadline and its source ("stated by the authority" or "bank default: 3 working days"), the linked cases and customers; "Register request" is a form with the stated deadline optional; the detail shows the response RFI and its state; the disclosure entry after release.
- **Internal reports tab:** the register with the channel, the acknowledgement due time and state; the reporter is shown to M only (the field is absent for other roles); "Acknowledge" for A2 and M; "Register report" for a report received outside the branch window.
- **Alarms tab** (A2 for touch and work-item breaches, M for all kinds, IA and SEC for their kinds): the list with the kind, the stage, the raised time, the notified time (the record of M-53) and the acknowledgement; "Close" asks for a closure reason code from the closure family.

### 5.5 G3 QA workspace (`/qa`, LIST + WORK; DESIGN-SYSTEM 8.1)

- **Samples list** (QA and the MLRO; CH and IA reviewers see their own rows): reviewer, population, stratum, due time, state; a sample past its due time is marked by words.
- **Item panel:** `GET /qa/samples/{id}/item` (the decision or alert summary, the reason codes, the evidence labels; no party name, account or contact, except that the verified output of an `assistant_draft` item may name parties of its case, section 7+). A suspicion item asks for the QA purpose and the step-up (`qa.item.open`); the panel says that the view is logged.
- **Review form:** eight criteria (evidence, reason, escalation, corroboration, timeliness, confidentiality, narrative, authority) each with the five levels of research 28 section 10.3 (Pass, Pass with comment, Minor, Major, Critical) and the critical-when text of that table; the overall level is computed and shown; findings are added per failing criterion; the comment field is sealed server-side. A critical finding shows the words "A re-review of this analyst's related items will be drawn".
- **Rates tab** (M): reviewer counts and the Wilson interval, shown only from `qa.min_reviewer_items`.
- **Calibration tab** (QA and M): the golden items and each calibrator's agreement.
- **Bulk reopen** (M, step-up): a dialog lists the cases closed by the analyst in the window (the candidates of section 7.14, at most 100 rows, of which at most `qa.bulk_reopen_max`, 25, may be selected), the reason code from the closure family, and the per-case result after the call.

### 5.6 Generated hooks (Orval names from the paths)

`useGetPartiesIdGraph`, `usePostPartiesIdNetworkCases`, `useGetPartiesId` (phase 18); `useGetRfis`, `useGetRfisId`, `usePostRfisIdSubmit`, `usePostRfisIdRelease`, `usePostRfisIdExtend`, `usePostRfisIdAnswers`, `useGetCasesIdRfis`, `usePostCasesIdRfis`; `useGetWatches`, `usePostCasesIdWatches`, `usePostWatchesIdReview`, `useGetHibernations`, `usePostCasesIdHibernations`, `usePostHibernationsIdEnd`; `useGetAuthorityRequests`, `usePostAuthorityRequests`, `useGetAuthorityRequestsId`, `useGetInternalReports`, `usePostInternalReports`, `usePostInternalReportsIdAcknowledge`, `useGetAlarms`, `usePostAlarmsIdAcknowledge`; `useGetQaSamples`, `usePostQaSamples`, `useGetQaSamplesIdItem`, `usePostQaSamplesIdReview`, `useGetQaRates`, `useGetQaCalibration`, `usePostQaBulkReopen`.

Hand-written (not Orval): none. The step-up dialog of phase 01 is reused for every SU route; the purpose choice of the graph and the QA item reuses the phase 18 purpose component.

## 6. Verification

1. `make migrate`: migrations `0064_casework_followup_records` and `0065_casework_followup_guards_grants` apply cleanly, and a second run is a no-op (every object is `IF NOT EXISTS` or `DROP TRIGGER IF EXISTS` then `CREATE`).
2. SQL check on a throwaway container (`docker run --rm postgres:17`, with the phase 00 to 18 schema loaded): each assertion below raises the stated error, and each insert of section 1 succeeds on a valid row:
   - a release with `released_by = proposer_id` is refused by the CHECK (section 1.1, `rfis`);
   - a move `draft -> released` is refused by `casework.rfis_guard` (section 1.2);
   - an update of `value_enc` of a claim is refused by `casework.rfi_answers_guard`, and a second `documented` to `partly` move is refused;
   - an answer insert on a `pending_release` RFI is refused by `casework.rfi_answers_insert_guard`;
   - a hibernation whose `decision_id` is not a `no_report` decision of the same case is refused by `casework.hibernations_insert_guard`;
   - a second active hibernation for the same party and scenario is refused by `uq_hibernations_active`;
   - a hit row released twice is refused by `casework.hibernation_hits_guard`; a `report_after` set back to FALSE is refused;
   - a live alarm of the same kind, subject and stage is refused by `uq_alarms_live`, and a `closed` alarm cannot change;
   - a kinship edge with `hidden_by_default = FALSE` is refused by the CHECK; an undirected edge with `from_party > to_party` is refused by the CHECK;
   - a review inserted by a user other than the sample's reviewer, or on a `reviewed` sample, is refused by `casework.qa_reviews_insert_guard`; an update of a review is refused by `casework.append_only()`;
   - a second live sample of the same production item is refused by `uq_qa_samples_item`;
   - the classification coverage test of phase 00 finds a row for every column of the fourteen tables, and every `encrypted` column has `classification` personal or secret-str; the retention coverage test finds one row per table.
3. `cargo check`: compile-time SQL verification passes for the repositories of section 2.
4. `cargo clippy -- -D warnings`.
5. `cargo test` (unit and property tests, in `domain::casework::followup` and `services::casework`):
   - Cedar: `92_followup.cedar` validates against the phase 03 schema with the section 3.13 additions (`cedar-policy` 4.x, the validator of the Cedar engine), and the authorization fixtures of section 3.13 give the stated decisions: an RFI desk reader is allowed a normal RFI and denied a suspicion RFI; the MLRO releases an external RFI and the desk does not; a QA reviewer writes a review only on an open sample and reads an item only with the QA purpose; the IA role reads a chain alarm and not a touch breach. The fixtures are added to the phase 03 test set.

   - `QaSampler::sample_size(0.95, 0.05, u64::MAX)` is 59; for `N = 20`, `C = 0.95`, `p = 0.05` the finite result is the smallest `n` whose hypergeometric miss probability is at most 0.05 (a fixed table of values is pinned in the test).
   - Stratified allocation: the sum of `a_s` equals `n`; every stratum with `N_s >= 1` gets at least `min(qa.min_per_stratum, N_s)`.
   - Wilson interval: `(successes 0, n 10)` gives `[0, 0.2775]` at `z = 1.96`, to four decimals.
   - Watch review date: an expiry on a non-working day gives `review_due_at` on the next working day; an expiry on a working day gives that day (`BankCalendarClock`, phase 09 fixtures).
   - Hibernation envelope: `n = env.n * 1.2` is not exceeded; `n = env.n * 1.2 + 1` is exceeded; the same for `total`.
   - RFI status property test: from every status, every command either yields a legal next status of section 3.2 or is refused; no sequence reaches `released` without a `pending_release` step.
   - Claim correction: a correction of a current claim leaves exactly one current claim per question.
   - Network edge rules: one fixture per source (contact, kinship, ownership, transfer) gives the stated kind, hidden flag and direction; a shared device gives none; a party with more than `network_supernode_degree` live edges is summarised.
   - Visibility of the RFI desk: the SQL of section 3.2 and the Cedar `Rfi` permits agree over generated principals and rows (section 6, check 5).
   - Lock order: a test takes the locks of section 3.15 in the declared order on each path and fails on a reversed acquisition (a static check of the call sites plus two concurrent tests of section 6 check 11).
6. Curl suite `tests/curl/phase19_followup.sh` (dev users, one per role: `a1@`, `a2@`, `a2b@`, `m@`, `rfi@`, `qa@`, `ia@`, `sec@`, `ch@`; the fixture `dual@bank.test` holds `A2` and `RFI` for the four-eyes check; `rfi@`, `qa@` and `dual@` are new in this phase and are added to the phase 18 dev-user list). Each check states the expected status and the code:
   - (a) `a1@`: `POST /cases/{id}/rfis` with a `branch_info` template key returns 201; the same request with the same `Idempotency-Key` returns 201 with the same id.
   - (b) `a1@`: `POST /rfis/{id}/submit` without `If-Match` returns 428; with a stale ETag returns 409 `E_VERSION_CONFLICT`; with the current one returns 200 and status `pending_release`.
   - (c) `a1@`: `GET /rfis` returns 403 `E_SCOPE` (scope `rfi.desk`).
   - (d) `rfi@`: `POST /rfis/{id}/release` without the step-up header returns 401 `E_STEPUP_REQUIRED`; with the assertion and the instance ETag returns 200 and status `released`.
   - (e) `dual@` (A2 and RFI) drafts and submits an RFI, then `POST /rfis/{id}/release` returns 403 `E_SELF_APPROVAL`.
   - (f) `rfi@`: `GET /rfis/{id}` for a suspicion-tier RFI returns 404 `E_NOT_FOUND` (masked); for a normal RFI the body has no `case_id`, `priority_class` or `tier` key (`jq` check).
   - (g) `rfi@`: `POST /rfis/{id}/answers` with a forbidden word in `value` returns 422 `E_VALIDATION` with a span and without the word in the body.
   - (h) `a2@`: `POST /authority-requests` with a stated deadline returns 201 with `deadline_source` `stated`; a second request with the same reference returns 409 `E_NOT_ALLOWED_IN_STATE` (sub-code `already_registered`); `rfi@` `GET /authority-requests` returns 403 `E_SCOPE`.
   - (i) `m@`: `POST /cases/{id}/hibernations` after a `no_report` decision with the step-up returns 201; `a2@` on a suspicion party returns 403 `E_FORBIDDEN`; `period_days: 181` returns 422 `E_VALIDATION`.
   - (j) `a1@`: `GET /parties/{id}/graph` returns 200 with no kinship edge; `?show_kinship=true` without `X-Access-Purpose` returns 422 `E_PURPOSE_REQUIRED`; with the purpose and without step-up returns 401 `E_STEPUP_REQUIRED`.
   - (k) `a1@`: `POST /parties/{id}/network-cases` returns 403 `E_SCOPE`; `a2@` returns 201 with a parent id and the children the caller may see.
   - (l) `qa@`: `POST /qa/samples` with population `closed_cases`, mode `stratified`, `n: 10` returns 201 with `sample_size` 10 or fewer and an `unassigned` count; no sample row's reviewer is a closer of its case (SQL check).
   - (m) a QA user other than the reviewer: `POST /qa/samples/{id}/review` returns 403 `E_FORBIDDEN` (sub-code `not_sample_reviewer`); the reviewer with an `overall` that is not the worst score returns 422 `E_VALIDATION` (`score_mismatch`); the reviewer with a critical finding returns 200, and `GET /alarms` as `m@` lists an alarm of kind `qa_critical`.
   - (n) `a2@`: `POST /qa/bulk-reopen` returns 403 `E_SCOPE`; `m@` without the step-up returns 401; with it returns 200 with one result per case.
   - (o) `a2@`: `POST /watches/{id}/review` on a watch that is not `under_review` returns 409 `E_NOT_ALLOWED_IN_STATE` (sub-code `not_due`).
   - (p) `ia@`: `GET /alarms` returns 200 and only alarms of kind `chain_broken`; `a1@` returns 403 `E_SCOPE`.
   - (q) `a2@`: `POST /internal-reports` returns 201; the same `GET` as `a2@` shows `reporter_id: null`, and as `m@` shows the id.
   - (r) Any sensitive write without `Idempotency-Key` returns 400 `E_IDEMPOTENCY_REQUIRED`; a suspicion case not on the caller's team returns 404 on `GET /cases/{id}/rfis` for `a1@`.
   - (s) Hibernation hit: a detection of the hibernated scenario and party (the non-release `POST /_dev/detections` of phase 17) does not appear in `GET /queues/{id}/items` and `GET /hibernations` shows `hits` 1; after `POST /hibernations/{id}/end` the hit is released and the alert appears in the queue once the release job has run.
   - (t) Cedar deny: `sec@` on `POST /rfis/{id}/answers` returns 403 `E_SCOPE` (scope `rfi.desk` is not held).
7. `make generate-api`: the OpenAPI document lists the 32 routes of section 2 (the 19 owned and the 13 added); the Orval hooks of section 5.6 exist with the names given.
8. Audit trail: after the curl session the audit events of section 3.12 appear for each action, with payloads that carry ids, codes and counts only (a grep for the test names and personal values finds none).
9. Masking: the probe set of the masked kind `rfi` (phase 03 `MASKED_KINDS`) passes the inference harness: a missing RFI and an RFI the caller may not see return the same body, headers and latency floor.
10. Tipping-off: the RFI desk view and the graph view for a caller outside the case team contain no case id, family, priority, tier or party name of an invisible party (a fixture with one suspicion neighbour).
11. Concurrency: two tests on the same case, a QA review against a closure, and a hibernation end against a hit release, run to completion without deadlock under the order of section 3.15.
12. Read-only conformance (phase 05): unchanged; this phase makes no CBS connection and no write to `canon.*`.
13. Existing tests still pass, including the phase 17 queue and count tests (the new clause of section 4.5 item 4 changes no count of an alert without a hit) and the phase 18 decision tests.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 4a, from tools/agent/gap1-run4a.md; lead-verified (tools/agent/gap1-run4a-verdict.md).

### 7.1 Branch answers and the release event (phase 22)
Phase 22 extends this phase and owns the extension (its section 4.5 items 1 and 2):
- Section 3.2, `RfiService::release` step 3: the payload of `case.rfi_released` gains `recipient_kind` (`{rfi_id, kind,
  recipient_kind}`).
- Section 3.3: the body of `RfiService::record_answers` is split into `record_answers_in_tx(tx, actor, rfi_id,
  answers, source_kind)`, called by the desk route after its `rfi.answer` check and by phase 22's branch route after
  `branch.item_answer`; the branch path always writes `corroboration = 'none'`. The `corroborates` path also accepts an
  evidence object whose owner kind is `rfi_input` and whose owner id is the RFI (DR-201).
- `GET /internal-reports` carries the observation fields for M only (DR-385, phase 22 section 7+).

### 7.2 Assistant QA populations (phase 28)
From phase 28 on (migration `0089`), `casework.qa_samples.population` gains `assistant_golden` and
`assistant_assisted`, and `source_kind` gains `assistant_draft`. `assistant_assisted` is the decisions whose
`ai_involvement.used` is true, drawn by the weekly draw of section 3.8 at `assistant.qa_sample_percent` with the draw's
existing seed rule, skipping decisions already in a sample; `assistant_golden` is drawn by `POST /qa/samples` (phase 28
section 3.6). The QA review screen shows the text of an `assistant_golden` draft to the reviewers of that sample.

### 7.3 Alarm kinds of later phases
The CHECKs of `casework.alarms` are widened by the owners' migrations; this phase's DDL is not edited:
`post_order_debit`, `freeze_overdue`, `refrain_overdue` with subjects `freeze`, `refrain`, `ops_task` (phase 20,
`0066`); `report_filing_overdue`, `report_rework_overdue` with subjects `report`, `report_task` (phase 21, `0070`);
`liaison_report_overdue` with subject `liaison_report` (phase 22); `capacity_forecast` with subject `pool` (phase 31,
`0097`). Each owner adds its rows to the table of section 3.10 and raises through `AlarmService::raise`.

### 7.4 Correspondent routes (phase 31)
From phase 31 on, `RfiService` gains `attach_pack` and `correspondent_list`, and the `rfis` router gains `GET
/rfis/correspondents` (`rfi.correspondent.list`) and `POST /rfis/{id}/packs` (`rfi.attach_pack`). Phase 31 owns
them.

### 7.5 Surge sampling (phase 31)
From phase 31 on, while `capacity.surge_active` is true, the sampler of section 3.8 takes `max(its rate,
capacity.surge_qc_percent)`; no sample is removed by a surge. Open doubts 10 keeps the rest of M-54. "Its rate" is the sampled share `n / N` of a draw that takes its size from the settings (section 3.8 step 2 with no requested `n`). During a surge that draw uses `n = min(N, max(n, ceil(N * capacity.surge_qc_percent / 100)))`. The percent is a share of the population, not a defect rate, so `qa.expected_defect_rate` is unchanged, and so is a draw with a requested `n` (V-31-12-a; DR-413).

### 7.6 Break-glass and the RFI team (phase 25)
From phase 25 on, a break-glass grantee enters `Rfi.assignees` only under purpose `break_glass` (phase 25 section 3.2
step 2), and under that purpose `51_platform.cedar` refuses every RFI action (phase 25 section 3.12). The rules of
`RfiWorkflowSubject` do not change.

Gap pass 1 (2026-10-10), run 4a: items carried from runs 3a and 3b and corrections found while checking phases 17 to 24 (tools/agent/gap1-run4a-verdict.md); lead-verified.

### 7.7 Screening hits and suppression records (DR-375)
Open doubts 9 is closed. `screening.hit` is written by phase 14's decision transaction for a party subject decided
`potential` or `true`, from phase 15 on (phase 14 section 7+); this phase appends its handler after phase 15's in the
ordered handler list. The suppression records sampled by `qa_suppression_sample` are the `alert.pair_memory` rows whose
`last_suppressed_at` falls in the ISO week: a 5 per cent draw (`qa.suppression_sample_percent`) plus every row whose `list_kind` is `freeze`, whatever the draw gives (the rule of phase 14 section 7.3, which holds where the population row of section 3.8 says 5 per cent only).

Gap pass 1 (2026-10-10), run 7a (coverage of UX and W IDs, gap item (ab)), from tools/agent/gap1-run7a.md; lead-verified (tools/agent/gap1-run7a-verdict.md).

### 7.8 Party filter on the watches list (phase 18 section 7+, UX-59)
`GET /watches` accepts `party_id` (UUID; optional). The visibility, cursor and Cedar `watch.list` are unchanged.

Gap pass 1 (2026-10-10), gap items (ac), (ad) and (ae) of tools/agent/gap-items.md (found in runs 7b and 8c); lead work, no Haiku draft.

### 7.9 The outreach.failed topic (TR-20; DR-405)
`timer_rfi_clock` at stage `due` on a `released` RFI of kind `customer_outreach` that still has open questions also
writes outbox `outreach.failed {party_id, rfi_id, prior_failures}` in the same transaction as `rfi.overdue`.
`prior_failures` is the number of earlier `customer_outreach` RFIs of the party whose `deadline_at` passed with
`answered_at` null, so TR-20's `when` can escalate on a second failure. This phase registers phase 15's trigger
handler for the topic (phase 15 section 7+). The topic is suspicion-linked (phase 15 section 3.6 rule 3), so the
review carries `TRG_REVIEW_REQUESTED`. Section 4.2 gains the row.

Adversarial pass (2026-10-10), run 4 (clocks and timers on closed, merged or reopened items), from tools/agent/adv-run4.md; lead-verified (tools/agent/adv-run4-verdict.md).

### 7.10 RFIs of a closed or merged case (ADV-4-03, ADV-4-04, ADV-4-06; MEDIUM; DR-410)
1. `RfiService::record_answers` step 1: after the locks, a case with `sealed_at` set refuses with 409 `E_CASE_SEALED`
   (phase 18 I7: a closed case is read-only). No claim is written.
2. The handler `timer_rfi_clock` re-reads the case as well as the RFI: a `closed` or `merged` case writes no
   `rfi.reminder` and no `rfi.overdue`.
3. From this phase on, the merge validation of phase 17 section 3.6 also refuses a source with a `released` RFI (409
   `E_NOT_ALLOWED_IN_STATE`, sub-code `rfi_open`).
The chain write of a closed case is already suppressed by the rule at the head of section 3. Test: release an RFI,
close the case: its 50 percent and deadline stages send nothing, and an answer is 409 `E_CASE_SEALED`.

Adversarial pass (2026-10-10), run 5 (existence masking leaks), from tools/agent/adv-run5.md; lead-verified (tools/agent/adv-run5-verdict.md).

### 7.11 Breach alarms follow the visibility of their subject (ADV-5-05, HIGH; DR-411)
`AlarmRepository::list_visible` and the read and acknowledge of a single alarm: a `touch_breach` alarm is visible to M
and to an A2 who can see its alert (phase 17 section 3.10 predicate, generated by `services::visibility::build`); a
`work_item_breach` alarm to M and to an A2 who can see the work item's case (the phase 18 case predicate with
`casework.case_acl`). For any other caller the alarm is absent from the list and from the counts by kind, and its read
and acknowledge answer 404 `E_NOT_FOUND` as a missing alarm. The other kinds keep their named readers. The Cedar permit
`followup.a2.alarm` stays the role gate; the service applies the predicate in SQL. The audience of an alarm notice (phase
22 section 3.8 step 4) is the same set. Test: an A2 outside the pool of a suspicion alert with a breach does not see the
alarm, and `POST /alarms/{id}/acknowledge` gives the body of a random id.

Adversarial pass (2026-10-10), run 6 (idempotency, audit chain and outbox replay), from tools/agent/adv-run6.md; lead-verified (tools/agent/adv-run6-verdict.md).

### 7.12 A watch trigger counts once per event (ADV-6-08, MEDIUM; DR-412)
The payload of `watch_trigger_dispatch` gains `event_at` (the `created_at` of the outbox row). Step (iii) of the watch
trigger also skips the watch when `last_trigger_key` equals this trigger's key and `last_trigger_at >= event_at`; step
(iv) stores `last_trigger_at = event_at`. A re-run of the job, or an older event of the same key delivered late, then
adds no `trigger_count`, no work item and no audit. This is the rule section 4.2 states ("only when the key is new");
its words "within the same second" are replaced by it. Test: run the same payload twice: `trigger_count` 1 and one
`review_evidence` work item.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.13 Parties of a canonical batch (V-19-01-c, MEDIUM; DR-413)
Section 2, `CanonNetworkReader` gains `batch_parties(conn, batch_id: Uuid) -> Result<Vec<Uuid>, RepoError>` (SELECT only): the distinct party ids of the rows that carry this `batch_id` (the lineage column of phase 10) in `canon.party` (`id`), `canon.party_contact` (`party_id`), `canon.party_relationship` (`from_party` and `to_party`) and `canon.account_party_role` (`party_id`). The `canon.touched` handler calls it once per event: the result is the `party_ids` of the `network_edge_build` payload and the `batch_parties` of `EdgeBuilder::rebuild` (section 3.7), and `watch_trigger_dispatch` runs the `profile_change` trigger of sections 3.5 and 3.6 for each party in it.

### 7.14 Bulk reopen: the finding, its window and the candidate list (V-19-02-a, V-19-02-b, MEDIUM; V-19-02-c, LOW; DR-413)
Section 3.9 Bulk reopen:
1. `QaFindingRepository` gains `get(conn, id) -> Option<QaFinding>`. A missing finding is 404 `E_NOT_FOUND`. A finding whose severity is not `critical` is 422 `E_VALIDATION` at `/finding_id`, sub-code `not_critical_finding`, which joins the closed list of section 3.11.
2. The window of the finding is the 30 days that end at `completed_at` of the review that recorded it, the window of the `same_closer_30d` draw of the review's step 7. No setting is added.
3. `QaSampleRepository` gains `closed_by(conn, analyst_id, from, to, limit) -> Vec<ClosedCaseRow>`, with `ClosedCaseRow { case_id, case_ref, closed_at }`: the cases with `closed_at` in the window whose latest decision has `decider_id = analyst_id` or whose `assignee_id` is the analyst, newest first. The `not_bulk_case` test reads it.
4. The dialog of section 5.5 reads its list from a new route, which joins the route table of section 2 (the thirty-second product route; DR-157 counted thirty-one before it): `GET /api/v1/qa/findings/{id}/reopen-candidates?analyst_id=<uuid>`, scope `qa.review`, Cedar `qa.bulk_reopen` on System (M only; the action of the reopen, so no action is added). It applies the checks of item 1 and returns at most 100 rows of `ClosedCaseRow`, with no total.

### 7.15 The RFI pre-fill is P2 (V-19-03-b, MEDIUM; DR-413)
M-44 is carried in part. The desk, the log, the deadline clock, the four-eyes release, the tipping-off flag and the missed-by-monitoring signal are built here. The pre-fill of an incoming correspondent RFI from the replica is P2: it needs a map from each question key of the bank's template to canonical fields, and the plan holds none. In P1 the pack switch `rfi.pre_fill` stays `false` and no code reads it. An answer that the desk takes from AKR-TMS records is recorded by hand through `record_answers` with `source_kind = replica` (section 3.3).

### 7.16 QA grant revocation path (V-19-04-a, MEDIUM; DR-413)
Section 3.9 step 8 and the grant expiry revoke a QA grant with `QaService::revoke_review_access(tx, grant_id, revoked_by, at)`, the counterpart of `grant_review_access` (section 3.8 step 7). It runs in the caller's transaction, in the lock order of section 3.15 (case, sample, grant), and sets `revoked_by` and `revoked_at` with `CaseAccessRepository::revoke(tx, id, by, at)` (phase 18 section 2). It does not call phase 18 `CaseAccessService::revoke`, whose step 2 refuses a closed case with `E_CASE_SEALED`, so the revocation works on a sealed case. Its audit event is `qa.grant_revoked {sample_id}`; it writes no case event.

### 7.17 Alarm list for IA and SEC (V-19-05-a, HIGH; DR-413)
Section 3.13: `GET /alarms` is decided by `alarm.list` on System. Two permits are added: `@id("followup.ia.alarms") @role("IA") permit (principal, action == Akr::Action::"alarm.list", resource is Akr::System) when { principal.roles.contains("IA") };` and `@id("followup.sec.alarms") @role("SEC") permit (principal, action == Akr::Action::"alarm.list", resource is Akr::System) when { principal.roles.contains("SEC") };`. `AlarmRepository::list_visible` then returns the `chain_broken` alarms to IA and the `chain_broken` and `conflict_declared` alarms to SEC, the kinds that `followup.alarm.audit.chain` and `followup.alarm.sec.conflict` allow them to read. This is the readers column of section 3.10 and check (p) of section 6.

### 7.18 Assistant items: enums, the weekly draw and the item view (V-19-07-a, LOW; V-19-07-b, V-19-07-d, MEDIUM; V-19-07-c, LOW; DR-413)
From phase 28 on (migration `0089`), with 7.2:
- The enums of section 2 follow the widened CHECK lists: `SamplePopulation` gains `AssistantGolden` (`assistant_golden`) and `AssistantAssisted` (`assistant_assisted`); `SampleSource` gains `AssistantDraft` (`assistant_draft`).
- "The weekly draw" of 7.2 is the job `qa_suppression_sample` (section 4.1, Monday 06:00). In the same run it also draws the population `assistant_assisted` at `assistant.qa_sample_percent` (phase 28 section 1.3), with the draw of section 3.8 (no actor, `drawn_by` NULL) and the skip rule of 7.2. No job kind or setting is added.
- `QaItemView` gains `assistant: Option<AssistantItem>`, with `AssistantItem { draft_id: Uuid, output: String, source_ids: Vec<SourceRef>, cited_panels: Vec<String> }` (`SourceRef` and the two lists are phase 28 section 2). It is set only for an item of source kind `assistant_draft`: `output` is the verified output, decrypted from `output_enc` for the sample's reviewer, who is on the case's access list through the time-boxed grant.

### 7.19 An RFI closes with its case (V-22-10-a, MEDIUM; DR-413)
No step set an RFI to `closed` or wrote `closed_at`, and phase 22 lists a closed item for `branch.item_closed_days` from `closed_at`. `CaseService::close` (the extension of section 4.5 item 5), after the closure gate and in the same transaction, sets every `released` or `answered` RFI of the case to `closed` with `closed_at = now()` (`RfiRepository::set_status`; the guard of section 1.2 allows both moves), cancels the RFI's timers and writes audit `rfi.closed {rfi_id}` for each (new; no chain event, because the case is closing). The rows are locked in ascending id order after the case row. A closed RFI is final: a reopened case drafts a new RFI.

### 7.20 Assistant golden items in the QA draw (V-28-02-b, V-28-02-c, MEDIUM; DR-413)
From phase 28 on, `DrawQaSample` (section 2) gains `eval_run_id: Option<Uuid>`. It is required for population
`assistant_golden` and refused with 422 `E_VALIDATION` for any other population. The population is then the
`casework.assistant_drafts` rows whose `eval_run_id` is that value; each becomes an item with `source_kind`
`assistant_draft`, `source_id` the draft id, `case_id` the draft's case and `source_at` the draft's `requested_at`, and
the sample row keeps the run in `casework.qa_samples.eval_run_id` (the column phase 28 adds). Step 7 of the draw
(section 3.8) also applies to an `assistant_draft` item: the same time-boxed read grant on the draft's case, reason
`qa_review`, so `GET /qa/samples/{id}/item` can show the verified output and the records it cites.

## Decisions

Numbered from DR-148 (phase 17 used DR-117 to DR-132 and phase 18 used DR-133 to DR-147). Each decision is one paragraph with its rationale.

**DR-148 (the RFI object, its kinds and the release workflow).** An RFI belongs to one case and has one of six kinds: `branch_info`, `rm_info`, `unit_info`, `customer_outreach`, `correspondent` (with direction `in` or `out`) and `authority_response`. Research 28 section 9.2 has seven rows, but its correspondent rows are one kind with two directions. Its text, legal basis, purpose codes and question keys are in the policy pack pinned to the case (phase 09 M-75), so a template change is a bank document change with a dry run, never a code change; the RFI keeps the template key and the pack version. The second signature is a workflow of phase 04: the drafter submits, one `approve` task is held by the RFI desk for internal kinds and by the MLRO for external kinds and for every suspicion RFI, the approver is never the drafter (phase 04 exclusion and `released_by <> proposer_id`), step-up is consumed by the engine once, and the terminal effect `release_rfi` sets the release. The two shipped definitions are new (the DSL E.4 table had none), so they enter DSL E.4 by a change request after the seed. A rejected RFI is terminal and is replaced by a new draft, so no instance is reused.

**DR-149 (RFI tipping-off and visibility).** Every RFI text is a template of the pack, scanned with scope `rfi`; no free-text question exists, so a drafter cannot write a question that names a case. The RFI desk sees the kind, the status, the deadline and the question keys, never the case id, family, priority, tier, party or decision: the desk view is a different struct from the team view. A suspicion RFI is the MLRO's or the case team's, and a masked 404 for others. Branch requests go to a liaison officer of the branch only, and the branch manager is never a recipient (M-43). A relationship manager or a customer is contacted only when the tipping-off flag allows it: the drafter may set the flag on a normal-tier case only, and an RFI with the flag set is always released by the MLRO, whose signature confirms that the contact tips no one off (lead review). The release of a suspicion RFI is external by the workflow, so the desk never releases one.

**DR-150 (answers are claims).** An answer is a claim row: its value is sealed, its source kind and reference are stated, its corroboration is `none`, `partly` or `documented` (a `documented` claim names the evidence object of the case), and it expires after `casework.rfi_claim_expiry_days`. A correction is a new row that supersedes the old one; corroboration only rises; a claim that is not current is terminal. The guard refuses an answer on an RFI that is not released, so no claim exists before a second signature. The closure gate (section 3.3) refuses a P0 or P1 closure that rests on uncorroborated claims alone, which is the M-45 rule and the lesson of the cases in research 20 (FM-21).

**DR-151 (internal reports and authority requests as registers).** Both registers are tables with clocks, not workflows, because neither is a decision: the internal report is acknowledged by a person within `casework.internal_report_ack_hours`, the authority request has a deadline stated by the authority or the bank default of three working days (research 28 section 9.5). Both are suspicion tier, and the reporter identity of an internal report is shown to the MLRO only (M-68). An authority request registers the outbox event `authority_request.recorded`, which phase 15 turns into the immediate review of its subjects (TR-17, the trigger of DSL C.5). The disclosure register of M-37 is the request's own disclosure fields, written when the response is released: ids and codes only, so the register can be produced without the texts.

**DR-152 (watches).** A UTR decision starts one watch on the decided party: the envelope records the measures at the decision, the four closed triggers (list hit, authority request, profile change, new alert) raise a `review_evidence` work item on the open case or are recorded on a closed one, and the expiry at `casework.watch_days` (90 calendar days, M-35) moves the watch to review with the first working day on or after the expiry. The review is a person's choice among extend, close and upgrade. The upgrade to a report outcome is not a decision: it is a work item and a new proposal under phase 18, so a watch can never file a report. The watch is a case object, so the case's visibility and tier govern it.

**DR-153 (hibernations, the suppression gate and the regret metric).** A hibernation is created inside a case after a no-report decision of that case (the guard of section 1.2 checks it), by A2 for a non-suspicion party and by the MLRO for any party (M-39). It stores an envelope of the measures at the start with the margin of DSL A.7, and it ends by expiry, by a rise of the envelope above the margin, by one of the four triggers of DSL A.7, or by a person. The suppression is the display: a suppressed alert stays in `alert.alerts`, its hit is a row of `casework.hibernation_hits` (the ledger that counts the suppression, a table beyond DATA-MODEL's list), and its alert is hidden from queues and counts until the hit is released, when the alert is attached to its case as if it had arrived then. Phase 16 defines no read path for hibernation, so the gate is a port of this phase called by phase 17's intake: it takes no lock and never ends a hibernation in the intake transaction, because the end writes a row of rank 3 that must not be held before a case lock. The regret metric is the share of suppressed alerts whose party later had a report outcome within the hibernation and thirty days after it (`report_after`), which is the measure a bank can compare with the scenario's value. The 180-day maximum is a database CHECK, and the margin is a governed setting.

**DR-154 (the network: evidenced edges, hidden kinship, snapshots and network cases).** An edge exists only when a canonical row shows the link: a shared phone or address hash, a shared family number hash, a canonical ownership or control relationship with evidence, or a payment link above a governed confidence between owners of two accounts. The rule is the phase 16 linked-party resolver's list of link types, with the two it lacks (`companion`, `depositor`) and the device kind left unbuilt because phase 10 has no device source. Kinship is stored and hidden by default; showing it needs a purpose code, step-up `party.open` and an access event, because a family link is personal data the analyst may not need. The graph is one hop and shows only the nodes the caller may see, with no placeholder for the others. A network case is a parent of family `network` with a snapshot and one child per visible subject, each child a case of the subject's own tier; the parent's closure stays a person's act once the children have outcomes, and a snapshot is evidence sealed by the worker through `put_generated`, because phase 02 gives that call to the worker only.

**DR-155 (QA sampling, independence and reopening).** A sample is drawn from a named population with a stratified, random or targeted mode; the size comes from the finite-population formula of section 3.8 at the governed confidence and defect rate; the draw is reproducible from a stored seed and a digest of the sorted population. The reviewer is never a closer of the item, a team member, or a user with a separation conflict, and an MLRO decision is reviewed by the compliance head or internal audit (M-49). The reviewer reads a case only through a time-boxed read grant (phase 18's access list, a grant of the new reason `qa_review` written by the QA service because phase 18's grant route refuses a closed case; revoked at review, otherwise at expiry), and writes a review only while the sample is open (a guard, not only Cedar). The rubric is the eight criteria and five levels of research 28 section 10.3; a major finding opens a coaching item, a critical finding raises an alarm to the MLRO and draws a re-review of the closer's related items. Reopening after a critical finding is the MLRO's act under step-up, through the phase 18 reopen rules, at most 25 cases per call (M-40). Calibration uses golden items, and the gating of queue access by calibration is left open.

**DR-156 (breach actions and the alarm register).** The clocks of alerts and work items have three stages: 50 percent (a notice on the SSE stream, no alarm), 100 percent (the existing deadline; an alarm to the supervisor, an A2 of the pool or team or else the MLRO), and 150 percent (an alarm to the MLRO). An alarm changes no status: it informs the supervisor and records the time it was informed and the time it acknowledged. The register also takes the chain-broken alarm, the declared-conflict alarm (M-67), the flat-line alarm (M-11) and the critical QA alarm, so there is one place where a supervisor sees what needs attention (M-53). Duplicates are refused by a unique index on the live alarm, so a repeated detector run raises nothing new. This closes phase 18 open doubt 11 and phase 17 open doubts 16, 21 and 22 for the kinds listed; the other M-53 alarms are open.

**DR-157 (schema and route additions beyond DATA-MODEL and API.md).** Fourteen tables are created: the twelve of DATA-MODEL section 12 that this phase owns, plus `casework.hibernation_hits` (the suppression ledger the regret metric needs) and `casework.alarms` (the M-53 register). Thirty-one product routes are listed: nineteen owned rows of API.md 3.12 and twelve added rows (submit, extend, the case RFIs, the watch start, the watch review, the hibernation end, the internal report registration, the network case, the QA item, the QA rates, and the alarm list and acknowledgement). The routes that a decision makes need `case.decide` or `rfi.desk`, `qa.review` for the samples, and `audit.read` for the alarm list. Thirty-one routes is above the 25 of the XL boundary; the roadmap row, which rated the phase L, is raised to XL in the alignment (lead review).

**Lead corrections (review 2026-10-09).** QA reviewers could not read closed cases: phase 18's grant route refuses a sealed case, so the QA grant is now a `case_acl` row of a new reason `qa_review` written by `QaService` (migration 0064 widens the CHECK), with no note and no case event. A hibernation could be created on a closed case and watch and hibernation events were appended to a sealed chain: the hibernation now needs an open case with outcome `no_report`, and a closed case's follow-up records write audit events only. Two of the six RFI kinds could never be drafted (the contact flag was always refused): the flag is now allowed on a normal-tier case, and an RFI with it set is always released by the MLRO (`external` attribute in Cedar instead of kind lists). The hibernation gate sat after the case step of phase 17 intake; it is now step 4b, and a suppressed alert skips the case, clock, chain and `alert.created` steps until its release. `casework.bulk_cap` did not exist (now setting `qa.bulk_reopen_max`). The RFI release did not handle an authority-matrix permit with `second_review` (now `second_review_not_shipped`, as phase 18). `ReleaseRfiEffect::run` lacked the `params` argument. The RFI visibility SQL ignored lapsed grants (`valid_to`) and the case access list. Alert measures in the watch and hibernation envelopes and the hit ledger are `financial`, as phase 17 classifies them, not `personal`. CH and IA now hold `qa.review` (API.md section 2) so they can review MLRO decisions (M-49). Phase 18 gained the `case.decision` outbox topic that phase 15 waits for (TR-15).

## Open doubts (for gap pass 1 and the bank)

1. **Source of the tipping-off flag (resolved in review).** The drafter sets `contact_allowed` on a normal-tier case only, and every RFI with the flag set goes to the MLRO's external release (section 3.1 step 4, DR-149). The bank may still name a different owner of the flag.
2. **Release definitions and templates.** The two definitions `rfi_release_internal` and `rfi_release_external` (section 3.2) are new; DSL E.4 gains them by a change request. The RFI templates, the question keys, the purpose codes and the `authority_response.v1` template are pack content that the compliance owner writes (phase 09). The correspondent deadline is the pack clock; a deadline stated by the sender is not carried by the draft request (`stated_deadline` is not in `DraftRfi`), so the bank decides whether to add it.
3. **Hours in the internal report clock.** Phase 09 adds hours as plain hours. The acknowledgement window of `casework.internal_report_ack_hours` (8 in the seed) is therefore a plain window. If the bank means working hours, phase 09 needs a working-hours unit or the window is expressed in working days.
4. **The liaison role code.** `canon.staff.role` is free text in phase 10 and no liaison code is defined. The branch request needs the code of the liaison officers (section 3.1 step 4).
5. **Closing an authority request.** Section 3.4 leaves an authority request `answered` after its response is released. A `closed` status needs a route that a person uses after the disclosure is recorded. The route is not in this phase's table.
6. **Adverse media and shared accounts.** The hibernation trigger `adverse_media` has no producer until a media source is contracted (phase 18 open doubt 28). Two parties on one account (the phase 16 `signatory` and `owner` links) are not network edges in this draft; the bank decides whether co-ownership of an account is a network edge.
7. **Shared device.** DATA-MODEL lists `shared_device` as an edge kind. Phase 10 has no device source, so no edge is built and the kind is kept for the day a source exists.
8. **Automatic close of a network parent.** M-10 says the parent closes when every child has an outcome. This draft records the state (`case.network_children_done`) and leaves the closure to a person, as phase 18 does for every case. The bank confirms the rule.
9. **Source of the screening suppressions.** The weekly sample (`qa_suppression_sample`) reads the phase 14 suppression records. Phase 14 does not name the table of its suppression records in the text read for this phase; phase 14 names the population and phase 19 reads it. The table name and the producer of the `screening.hit` topic are confirmed in gap pass 1.
10. **Requirements not built here.** M-51 (the training register and the AI golden set), M-52 (the capacity workbench), M-53 (the growth, latency, silence and capacity alarms), M-54 (surge mode), M-55 (volume-change preview of a rule change), M-76 (the simulation harness) and T-22 (the motif engine) are not built. Their target phase is the lead's decision: M-52 and M-53 suit phase 20 (operations) or phase 24 (governance), M-54 and M-76 suit phase 20 or the policy phase, M-55 belongs with phase 04 change requests, and T-22 needs a phase of its own. The growth and latency alarms need the clocks of phase 17 (the latency per alert).
11. **Missing liaison report alarm.** M-53 names missing liaison reports. The liaison monthly report is a phase 22 product, so the alarm needs phase 22's report record. Target phase 22.
12. **Scope of the QA samples for CH and IA (resolved in review).** API.md section 2 now gives `qa.review` to CH and IA as well (M-49), and the phase 09 decision-type table follows; Cedar limits them to their own samples (section 3.13).
13. **Working days in the KPIs.** `median_days` and `p90_days` are in calendar days. Working days would need a working-day difference function in phase 09's `BankCalendarClock` (its `add` is the only function in the phase 09 text).
14. **Calibration gating and second labels.** Queue access gated by a calibration result (M-50) is not built. The second labels for inter-rater agreement are the calibration rows of this phase; the bank sets how many items are double-reviewed outside the calibration set (phase 16 open doubts 9 has the same question for scenarios).
15. **Sub-codes.** The sub-codes of section 3.11 (a list of twenty-four) are added to the closed list of phase 18 section 3.17 by the API owner, or the API owner names one top-level code for each, as phase 18 open doubt 22 asks.
16. **Retention of the authority register.** The clock is `age` from receipt (seven years), since a request is not bound to one closed case. The bank confirms whether the closure of the linked cases should set the clock instead.
17. **Step-up class of the second signature.** `rfi.release` uses class `approval`, and the authority registration, the hibernation, the watch review and the bulk reopen use `decision`. Phase 18 open doubt 3 asks whether a signature uses `signing`; the same question applies to `rfi.release`.
18. **Disclosure entry scope.** The disclosure fields (section 3.4 step 9) record claim ids and question keys. Whether the disclosure register must also record the document sent to the authority is a question for phase 21 (the report package) or the bank's legal review.
19. **Watch triggers and working days.** The watch expiry is in calendar days (90) and its review date is the next working day. A watch that should expire in working days needs `BusinessUnit::WorkingDays` and the bank's choice.
20. **Network depth.** The graph and the network case are one hop. A second hop (the relatives of a subject, for example) doubles the visibility checks and the edge reads; the bank decides whether the second hop is needed.
21. **Supernode degree.** `casework.network_supernode_degree` (200) is in the settings of section 1.2; the bank confirms the value.
22. **Internal and outreach RFI clocks and the pack bounds.** The branch, RM and unit clock is 2 working days in the seed, and the pack clock `rfi.response` has bounds of 5 to 20 (phase 09). The setting `casework.rfi_internal_days` therefore sits outside the pack registry. Widening `rfi.response` to 1 to 20, or keeping the internal clock as a setting, is a phase 09 decision.
23. **Regret window.** The regret metric counts report outcomes within the hibernation and thirty days after it. The thirty days are the draft's proposal; the MLRO confirms them with the hibernation rule.
24. **The RFI desk and branch scope.** The RFI role has no branch scope in the draft; a liaison of one branch cannot see another branch's RFIs only because the RFI desk is not a branch role. Whether the RFI role is branch-scoped is a phase 01 grant decision (`admin.role_grants.scope`).
25. **Hit release on an ended hibernation.** When a hibernation ends, its open hits are released and their alerts enter the queue at once (section 3.6). The bank may prefer that suppressed alerts stay in the ledger only; the choice changes one step of the release job.
26. **Batch reopen entry point.** `CaseService::reopen_under_batch` (section 4.5 item 5) is a new entry point of phase 18. Phase 18's reopen rules are the same; the question is whether the batch's single step-up is accepted by the bank as covering the cases of one call.
27. **The `case.decision` topic (resolved in review).** Phase 18 now writes `case.decision` from `RecordDecisionEffect` and from the escalation (phase 18 sections 3.8 and 3.11, section 4.2), the TR-15 source topic that phase 15 consumes.
28. **Reviewer candidates.** The QA candidates are the active users with an active `QA` grant (and CH or IA for MLRO decisions). A user whose grant lapses between the draw and the review keeps the sample; the review is still refused by the guard only when the reviewer is not the sample's reviewer. The rule for a lapsed grant is open.
29. **The API path of the case access list.** API.md section 2 lists `/cases/{id}/acl` (line 193) and phase 18 lists `/cases/{id}/access`. The API owner chooses one; this phase's grant of the QA reviewer goes through the service and does not depend on the path.
30. **Conventions file (resolved in review).** The lead's conventions file is corrected to `admin.column_classification` and `admin.retention_assignment`.
31. **Dev users.** `rfi@bank.test`, `qa@bank.test` and `dual@bank.test` (A2 and RFI) are new dev users for the curl suite; the conventions list of dev users (`<role>@bank.test`) has no RFI or QA user. The phase 03 dev fixture must create them.

## What This Phase Does NOT Touch

- No alert creation, no claim, no triage, no score and no queue rule (phase 17); the intake gains the gate call and the breach arm (section 4.5 item 4), and the queue and count clauses of section 4.5 item 4 are the only change to phase 17's queries.
- No decision, no proposal, no note, no evidence upload, no escalation and no closure logic beyond the two hooks of section 4.5 item 5 (the closure gate and the network hook) and the decision's watch start; no change to `casework.decisions`, `casework.proposals` or the workflow engine's invariants (phase 04 and 18).
- No report package, no goAML record, no filing and no filing acknowledgement (phase 21); no reopen-rate report (phase 21).
- No freeze, no operations task and no CBS-facing action (phases 14, 20); no write to any `canon.*` table (phase 10); no CBS write (phase 05).
- No screening decision, screening state or pair-memory write (phase 14); the weekly sample reads its records and writes none to them.
- No rating, override or feature write (phases 13 and 15); the trigger topics of this phase are produced for phase 15's consumers, and no suspicion-derived factor reaches a rating (DR-103).
- No notification, no in-app delivery and no exports (phases 21 and 22); outbox payloads carry ids and codes only, and the consumers are phase 22's.
- No AI assistant, no model score and no AI involvement (P2, M15 gate).
- No new role, no new step-up class (the phase 01 list is used as it is; the constants of section 4.5 item 11 use its classes), no new problem code (API section 6 only; the sub-codes ride in the bodies), no new Valkey database, no new masked kind other than `rfi` (the party page is phase 15's `customer`, the case is phase 17's `case`).
- No change to `90_alerts.cedar` (phase 17), `91_cases.cedar` (phase 18) or any other phase's Cedar file; `92_followup.cedar` is the only new file.
- No retention sweep and no legal hold (phase 23); the seals and the sealed texts follow their clocks, the sweep is phase 23's.
- No change to the reason-code families, the policy pack document or the authority matrix document; this phase uses them (phase 09).
- No motif engine, no capacity workbench, no surge mode and no simulation harness (Open doubts 10).
