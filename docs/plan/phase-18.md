# Phase 18: Cases, Decisions & Investigation

**Goal:** The case becomes the place where work is decided. Analysts add notes and evidence, link entities and work items, and propose an outcome (no report, UTR, STR or TFR) with coded reasons, a checked text and the evidence they relied on. The decision is a workflow task of phase 04 that a second person completes under step-up, after the decider has opened the evidence and panels the class requires; a case is escalated, closed and sealed with a hash in the audit chain, and a closed case is reopened only by a person; the case access list (`casework.case_acl`) becomes a visibility clause; screens W3 (case workspace), W4 (party page), W5 (transaction explorer) and W7 (decision dialogs) are built.

**Dependencies:** Phase 00 (jobs, outbox, timers, settings, cron schedules), Phase 01 (`StepUpService` and its closed action classes, `admin.role_grants`, preferences), Phase 02 (`EvidenceStore::stage`, `open`, `put_generated`; `PgAccessLog` and `AccessScope`; `AuditSink`; the canonical encoder; the evidence owner registry), Phase 03 (Cedar bundle and schema, masking registry, purpose codes, analyst networks), Phase 04 (`WorkflowEngine`, `WorkflowSubject`, `TerminalEffect`, `Holders`, the `case_acl` assignee kind, the `task.*` Cedar actions), Phase 09 (`BusinessClock`, reason-code families `decision` and `closure`, `PolicyPackService::scan`, `AuthorityMatrix::permits`), Phase 10 (canonical party, account, account-party role and transaction tables), Phase 14 (screening alerts, `alert.screening_alerts.case_id`, the decision service that calls `ScreeningCaseLinker`), Phase 15 (`RatingSource`, the `customer` masked kind, DR-103), Phase 17 (`casework.cases`, `casework.case_events` and `CaseChain`, `casework.case_alerts`, `casework.case_parties`, `casework.party_tiers`, `casework.effective_tier`, `alert.alerts`, `AssignmentService`, the visibility predicate)
**Macro-phase:** P1
**Complexity:** XL (28 product routes: the 25 of section 2, of which 9 are rows of API.md 3.12 owned here and 16 are added rows, and the three of sections 7.11 to 7.13; the decision engine with four-eyes, the evidence-opened gate, the hash-sealed closure and the case access list carry the weight; the template's 25-route boundary for XL is met, matching the roadmap)
**Binaries touched:** akr-api (routers `case_records`, `decisions`, `parties`, and the phase 17 `cases` router extended; the decision act runs `WorkflowEngine::act` and the terminal effects `record_decision` and `release_for_filing` in the request transaction; evidence upload through `EvidenceStore::stage`; evidence content through `EvidenceStore::open`) | akr-worker (job `timer_work_item_due`; the phase 17 outbox topic `case.event` is reused) | akr-ingestd and akr-assess unchanged
**Requirements carried:**
- Implemented here: M-01 (outcome set `no_report`, `utr`, `str`, `tfr`, each with a named decider and coded reasons; the UTR outcome is always available; the per-bank switch of research file 28 is not built), M-02 (origin of suspicion and earlier-report references stored on every proposal; the UTR-to-STR link is phase 21), M-04 (queryable shelved-decision register, `GET /shelved-decisions`; QA sampling of it is phase 19), M-09 (contribution outcome per alert and inheritance from the case decision, DR-138), M-19 (decision four-eyes by Cedar, the workflow exclusion and a database CHECK; L1 and L2 differ is phase 17), M-20 (delegated-authority check at decision through `AuthorityMatrix::permits`, DR-143), M-21 (required evidence panels opened before a decision for the configured classes, default P0 and P1, DR-136), M-26 (decision record fields of section 6.1 of research file 28, immutable), M-27 (governed reason codes in Arabic and English, generic text refused, DR-137; the distribution report is phase 21), M-28 (seven CBI escalation criteria as a checklist beside the decision, stored on the proposal and the decision), M-29 (UTR versus STR aid: definitions shown in W7, the answers recorded), M-38 (closing sequence: outcome fixed, work items done or waived by a second person, sealed, retention clock set), M-59 (notes and decisions immutable; amendments reference the original), M-61 (case sealing with a hash in the audit chain, re-seal on reopen with both seals kept, DR-139), M-74 (case aggregate state machine in Rust with property tests; the workflow runtime is phase 04), UX-06 (case workspace on one page with evidence panels and a sticky decision panel), UX-64 (actions separated from decisions), UX-65 (decision forms require coded reasons and refuse generic text, with the check shown)
- Partly implemented (the rest is in Open doubts with its target phase): M-40 (single reopen with reason and step-up; bulk reopen by reviewer is phase 19, reopen-rate reports phase 21), M-58 (evidence snapshot at case open is phase 17; "add to evidence" uploads are here, capture of windows and graphs is Open doubts), M-60 (vault rules come from phase 02 `stage` and `open`; the lock extension at closure is Open doubts), M-68 (case-level note thread under the access list and the reporter identity visible only to the MLRO desk here; generic notifications are phase 22), M-74 (the case aggregate here; the workflow runtime of phase 04)
- Not carried here (target phase in Open doubts): M-36 (exit proposals; proposed for phase 15, Open doubts 13), M-10, M-35, M-37, M-39, M-42 to M-54, M-55, M-76, T-21, T-22 (phase 19), M-30, M-31, M-41 (phase 21), M-25, M-66 (phase 22), M-05 (phase 23), M-56, M-62, M-63, M-64, M-69 (phase 24), M-70 to M-73 (P2)
- Gap pass 1 (coverage), implemented here: E-07 (suspicion data on a need-to-know grant, access events on every read); SP-28 (manual grants on suspicion-tier cases by the MLRO only, time-boxed and expiring automatically; section 7+); L-54 (decision four-eyes by Cedar, the workflow and a database CHECK; generic text refused; the QA queue is phase 19); UX-07 (the entity timeline of the case subject, section 7+), UX-29 (global search with kind detection and suspicion rows absent for non-members, section 7+); F-16 (need-to-know case access, the tipping-off review, contact only for normal-tier parties); VB-31 (escalation, report, freeze and exit decided by a named person).
- Gap pass 1 (coverage), partly implemented: RP-12 (decision outcomes with reasons and the UTR-to-STR link), TS-20 (the per-case event chain and the note guard); SP-26 (three tiers, the case access list, step-up, analyst networks; time-boxed manual grants in section 7+), SP-32 (step-up, purpose and an access event on every content download by the case team), SP-100 (recorded decisions with a named decider and coded reasons); L-55 (the opened-view record and access events); T-11 (the controlled closure-reason vocabulary); UX-09 (the evidence-opened gate before a decision), UX-17 (drafts kept in the tab across a time-out), UX-57 (the case workspace of mockup 02); F-15 (cases with clocks and the STR outcome at any amount), B-08 (profile, rating and transaction panels on the case); VA-13 (case intake from every alert source, shipped workflow definitions, decision history), VA-48 (the shelved-decision register); VB-03 (the case workspace with cases and alerts of the party), VB-06 (the closed outcome set), VB-32 (the seven-criteria checklist), VB-34 (the decision checklist per case).

---

## 1. SQLx Migration Changes

Two migrations, `0062_casework_decisions_records` and `0063_casework_decisions_guards_grants`, numbered after phase 17's `0061` (the roadmap chain has no other user of 0062 or 0063). The schema `casework` exists (phase 00); no new schema. Tables created here: `casework.case_acl`, `casework.notes`, `casework.proposals`, `casework.decisions`, `casework.case_seals`, `casework.work_items`, `casework.evidence_items`, `casework.entity_links`, `casework.opened_views`. Existing tables changed: `casework.cases` (one column, one CHECK), `alert.alerts` (one column, one CHECK) and the grants and classification rows of phase 17's tables. No table of phases 00 to 17 is recreated.

**Reconciliation with DATA-MODEL section 12 (aligned in the same change, see DR-135 and DR-145):**
- `casework.case_acl`: DATA-MODEL's columns plus `id` (primary key), `reason` (why the person is on the case), `note_id` (the note that gives the reason of a manual grant), `revoked_by`.
- `casework.decisions`: DATA-MODEL's `alert_id` is not a column (the alerts a decision covers are in the proposal's `alert_outcomes`); `type` is narrowed to `no_report`, `utr`, `str`, `tfr` because `close` and `escalate` are acts, not decisions (DR-135); `text` is `text_enc` (sealed); added `proposal_id`, `workflow_instance_id`, `second_review`.
- `casework.notes`: `body` is `body_enc` (sealed); `amends` as in DATA-MODEL.
- `casework.work_items`: DATA-MODEL's columns plus `description_enc`, `created_by`, `closed_by`, `waived_by`, `waiver_reason_code`, `updated_at`, `version`.
- `casework.evidence_items`: DATA-MODEL's `sha256`, `classification` and `lang` are not copied; the object row in `admin.evidence_objects` (phase 02) is the single source. The table keeps the link, the label and the adder.
- `casework.entity_links`: DATA-MODEL describes network edges (kinds `shared_contact`, `shared_device`, `transfer`, `ownership`, `kinship`, retention R3). Phase 18 uses the name for the case's own entity links (party, account, transaction, external; retention R6). The network edges move to phase 19 under the name `casework.network_edges` (DR-145, Open doubts 12).
- New tables not in DATA-MODEL: `casework.proposals` (the outcome proposed before a decision exists), `casework.case_seals` (every seal, so a reopened case keeps both), `casework.opened_views` (what a decider opened; the access log is not readable by `akr_app`, DR-136).
- Altered: `casework.cases` gains `escalation_level` and the outcome CHECK; `alert.alerts` gains `contribution_inherited` and the contribution CHECK. Phase 17 created `contribution_outcome` without a CHECK and said "written by phase 18 only".

### 1.1 `0062_casework_decisions_records.sql`

```sql
-- migration 0062_casework_decisions_records.sql
-- Phase 18. Column additions to phase 17 tables, and the case records: access list, notes, proposals, decisions, seals,
-- work items, evidence links, entity links and the opened views that the decision gate reads (DR-136).

-- Escalation level (DR-140): 1 analyst, 2 senior analyst, 3 MLRO desk. Only ever raised, one step at a time.
ALTER TABLE casework.cases ADD COLUMN IF NOT EXISTS escalation_level SMALLINT NOT NULL DEFAULT 1
    CHECK (escalation_level BETWEEN 1 AND 3);
-- The outcome is one of the four M-01 outcomes, and a closed case always has one (the closing sequence of M-38).
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'ck_cases_outcome') THEN
        ALTER TABLE casework.cases ADD CONSTRAINT ck_cases_outcome
            CHECK (outcome IS NULL OR outcome IN ('no_report', 'utr', 'str', 'tfr'));
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'ck_cases_closed_outcome') THEN
        ALTER TABLE casework.cases ADD CONSTRAINT ck_cases_closed_outcome
            CHECK (status <> 'closed' OR outcome IS NOT NULL);
    END IF;
END $$;

-- Contribution outcome per alert (M-09, DR-138). Phase 17 created the column without a CHECK.
ALTER TABLE alert.alerts ADD COLUMN IF NOT EXISTS contribution_inherited BOOLEAN NOT NULL DEFAULT FALSE;
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'ck_alerts_contribution') THEN
        ALTER TABLE alert.alerts ADD CONSTRAINT ck_alerts_contribution CHECK (
            contribution_outcome IS NULL
            OR contribution_outcome IN ('explained', 'not_explained', 'not_assessed', 'duplicate', 'data_error'));
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'ck_alerts_inherited') THEN
        ALTER TABLE alert.alerts ADD CONSTRAINT ck_alerts_inherited
            CHECK (contribution_inherited = FALSE OR contribution_outcome IS NOT NULL);
    END IF;
END $$;

-- Notes: posted, never edited. An amendment is a new note that names the old one (M-59). Body sealed (personal, AAD = id).
CREATE TABLE IF NOT EXISTS casework.notes (
    id          UUID PRIMARY KEY,
    case_id     UUID NOT NULL REFERENCES casework.cases(id) ON DELETE RESTRICT,
    author_id   UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    body_enc    BYTEA NOT NULL,
    amends      UUID REFERENCES casework.notes(id) ON DELETE RESTRICT,
    at          TIMESTAMPTZ NOT NULL,                 -- the case chain time of the note (section 3.7 of phase 17)
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (amends IS NULL OR amends <> id)
);
CREATE INDEX IF NOT EXISTS idx_notes_case   ON casework.notes (case_id, at);
CREATE INDEX IF NOT EXISTS idx_notes_author ON casework.notes (author_id);

-- The case access list (DR-133). A grant is a row; a revocation sets revoked_at once. One active grant per person and case.
CREATE TABLE IF NOT EXISTS casework.case_acl (
    id            UUID PRIMARY KEY,
    case_id       UUID NOT NULL REFERENCES casework.cases(id) ON DELETE RESTRICT,
    principal_id  UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    access        TEXT NOT NULL CHECK (access IN ('read', 'work', 'decide')),   -- the grant values of admin.tasks.acl_grant (phase 04)
    reason        TEXT NOT NULL CHECK (reason IN ('assignment', 'escalation', 'second_review', 'mlro_desk', 'manual')),
    note_id       UUID REFERENCES casework.notes(id) ON DELETE RESTRICT,   -- required for a manual grant
    granted_by    UUID REFERENCES admin.users(id) ON DELETE RESTRICT,      -- NULL = the system (escalation to level 3)
    granted_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at    TIMESTAMPTZ,                                              -- the time box of a manual grant (section 7+, SP-28)
    revoked_by    UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    revoked_at    TIMESTAMPTZ,
    CHECK ((reason = 'manual') = (note_id IS NOT NULL)),
    CHECK (revoked_by IS NULL OR revoked_at IS NOT NULL),
    CHECK (revoked_at IS NULL OR revoked_by IS NOT NULL OR (expires_at IS NOT NULL AND revoked_at >= expires_at)),   -- NULL revoked_by: the system expired it
    CHECK (expires_at IS NULL OR expires_at > granted_at)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_case_acl_active ON casework.case_acl (case_id, principal_id) WHERE revoked_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_case_acl_principal ON casework.case_acl (principal_id) WHERE revoked_at IS NULL;

-- A proposal is the outcome a person proposes before the decision. Its content is fixed at creation; the workflow instance
-- (phase 04) carries the approval. At most one proposal runs per case (partial UNIQUE). Content sealed where it names a person's
-- conduct (text_enc).
CREATE TABLE IF NOT EXISTS casework.proposals (
    id                    UUID PRIMARY KEY,
    case_id               UUID NOT NULL REFERENCES casework.cases(id) ON DELETE RESTRICT,
    workflow_instance_id  UUID NOT NULL UNIQUE REFERENCES admin.workflow_instances(id) ON DELETE RESTRICT,
    type                  TEXT NOT NULL CHECK (type IN ('no_report', 'utr', 'str', 'tfr')),
    definition_key        TEXT NOT NULL CHECK (definition_key IN ('case_close_no_report', 'case_report_approval')),
    reason_codes          TEXT[] NOT NULL CHECK (cardinality(reason_codes) BETWEEN 1 AND 5),
    text_enc              BYTEA NOT NULL,
    evidence_refs         JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(evidence_refs) = 'array'),
    alert_outcomes        JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(alert_outcomes) = 'array'),
    checklist             JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(checklist) = 'object'),
    origin                JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(origin) = 'object'),
    typology              TEXT CHECK (typology IS NULL OR length(typology) BETWEEN 1 AND 40),
    subjects              JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(subjects) = 'array'),
    transactions          JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(transactions) = 'array'),
    actions               JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(actions) = 'array'),
    earlier_reports       JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(earlier_reports) = 'array'),
    proposer_id           UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    ended_at              TIMESTAMPTZ,
    end_kind              TEXT CHECK (end_kind IN ('completed', 'rejected', 'withdrawn', 'failed')),
    CHECK ((ended_at IS NULL) = (end_kind IS NULL)),
    CHECK ((type = 'no_report') = (definition_key = 'case_close_no_report'))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_proposals_running ON casework.proposals (case_id) WHERE ended_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_proposals_case ON casework.proposals (case_id, created_at);

-- Decisions (DR-134). Written only by the terminal effect record_decision of the workflow (never by a route, never directly).
-- Immutable; a correction after a reopen is a new decision with `supersedes`.
CREATE TABLE IF NOT EXISTS casework.decisions (
    id                    UUID PRIMARY KEY,
    case_id               UUID NOT NULL REFERENCES casework.cases(id) ON DELETE RESTRICT,
    proposal_id           UUID NOT NULL UNIQUE REFERENCES casework.proposals(id) ON DELETE RESTRICT,
    workflow_instance_id  UUID NOT NULL REFERENCES admin.workflow_instances(id) ON DELETE RESTRICT,
    type                  TEXT NOT NULL CHECK (type IN ('no_report', 'utr', 'str', 'tfr')),
    reason_codes          TEXT[] NOT NULL CHECK (cardinality(reason_codes) BETWEEN 1 AND 5),
    text_enc              BYTEA NOT NULL,
    decider_id            UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    proposer_id           UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    decided_at            TIMESTAMPTZ NOT NULL,
    evidence_refs         JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(evidence_refs) = 'array'),
    opened_views          JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(opened_views) = 'array'),
    checklist             JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(checklist) = 'object'),
    origin                JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(origin) = 'object'),
    typology              TEXT CHECK (typology IS NULL OR length(typology) BETWEEN 1 AND 40),
    subjects              JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(subjects) = 'array'),
    transactions          JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(transactions) = 'array'),
    actions               JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(actions) = 'array'),
    earlier_reports       JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(earlier_reports) = 'array'),
    alert_outcomes        JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(alert_outcomes) = 'array'),
    ai_involvement        JSONB NOT NULL DEFAULT '{"used": false}'::jsonb CHECK (jsonb_typeof(ai_involvement) = 'object'),
    second_review         BOOLEAN NOT NULL DEFAULT FALSE,
    versions              JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(versions) = 'object'),
    stepup_id             UUID,                             -- no foreign key (the assertion ledger is pruned; phase 04 convention)
    supersedes            UUID REFERENCES casework.decisions(id) ON DELETE RESTRICT,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (decider_id <> proposer_id),                      -- four-eyes, second layer under Cedar and the workflow (M-19)
    CHECK (supersedes IS NULL OR supersedes <> id)
);
CREATE INDEX IF NOT EXISTS idx_decisions_case    ON casework.decisions (case_id, decided_at);
CREATE INDEX IF NOT EXISTS idx_decisions_type    ON casework.decisions (type, decided_at);       -- shelved register (M-04)
CREATE INDEX IF NOT EXISTS idx_decisions_decider ON casework.decisions (decider_id, decided_at);

-- Seals (M-61). One row per closure; a reopened case keeps the earlier seals here and clears the seal fields on casework.cases.
CREATE TABLE IF NOT EXISTS casework.case_seals (
    id           UUID PRIMARY KEY,
    case_id      UUID NOT NULL REFERENCES casework.cases(id) ON DELETE RESTRICT,
    seq_no       INT NOT NULL CHECK (seq_no >= 1),
    outcome      TEXT NOT NULL CHECK (outcome IN ('no_report', 'utr', 'str', 'tfr')),
    reason_code  TEXT NOT NULL REFERENCES ref.reason_codes(code) ON DELETE RESTRICT,
    closed_at    TIMESTAMPTZ NOT NULL,
    last_seq     INT NOT NULL CHECK (last_seq >= 1),
    head_hash    BYTEA NOT NULL CHECK (octet_length(head_hash) = 32),
    seal_hash    BYTEA NOT NULL CHECK (octet_length(seal_hash) = 32),
    sealed_by    UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    sealed_at    TIMESTAMPTZ NOT NULL,
    UNIQUE (case_id, seq_no)
);

-- Work items: case to-dos that are not approval steps (approvals are workflow tasks). Waiving needs a second person (DR-144).
CREATE TABLE IF NOT EXISTS casework.work_items (
    id                  UUID PRIMARY KEY,
    case_id             UUID NOT NULL REFERENCES casework.cases(id) ON DELETE RESTRICT,
    kind                TEXT NOT NULL CHECK (kind IN ('obtain_document', 'contact_branch', 'review_evidence', 'other')),
    description_enc     BYTEA,
    assignee_id         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    pool_id             UUID REFERENCES admin.pools(id) ON DELETE RESTRICT,
    due_at              TIMESTAMPTZ NOT NULL,
    timer_id            UUID REFERENCES admin.timers(id) ON DELETE RESTRICT,
    state               TEXT NOT NULL DEFAULT 'open' CHECK (state IN ('open', 'done', 'waived')),
    result              JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(result) = 'object'),
    created_by          UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_by           UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    closed_at           TIMESTAMPTZ,
    waived_by           UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    waiver_reason_code  TEXT REFERENCES ref.reason_codes(code) ON DELETE RESTRICT,
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    CHECK (assignee_id IS NULL OR pool_id IS NULL),
    CHECK ((state = 'open') = (closed_at IS NULL)),
    CHECK ((state = 'done') = (closed_by IS NOT NULL AND waived_by IS NULL)),
    CHECK ((state = 'waived') = (waived_by IS NOT NULL AND waiver_reason_code IS NOT NULL)),
    CHECK (waived_by IS NULL OR waived_by <> created_by)
);
CREATE INDEX IF NOT EXISTS idx_work_items_case ON casework.work_items (case_id, state);
CREATE INDEX IF NOT EXISTS idx_work_items_open_due ON casework.work_items (due_at) WHERE state = 'open';

-- Evidence links: the case's own evidence (the bytes are phase 02 evidence objects, owner kind `case`). Append-only.
CREATE TABLE IF NOT EXISTS casework.evidence_items (
    id          UUID PRIMARY KEY,
    case_id     UUID NOT NULL REFERENCES casework.cases(id) ON DELETE RESTRICT,
    object_id   UUID NOT NULL UNIQUE REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    label       TEXT NOT NULL CHECK (length(btrim(label)) BETWEEN 1 AND 120),
    added_by    UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    added_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_evidence_items_case ON casework.evidence_items (case_id, added_at);

-- Entity links of the case (DR-145): a party, an account, a transaction, or an external identifier (sealed value).
-- A link is retired once (retired_by, retired_at); it is never deleted.
CREATE TABLE IF NOT EXISTS casework.entity_links (
    id                  UUID PRIMARY KEY,
    case_id             UUID NOT NULL REFERENCES casework.cases(id) ON DELETE RESTRICT,
    entity_kind         TEXT NOT NULL CHECK (entity_kind IN ('party', 'account', 'transaction', 'external')),
    party_id            UUID,                         -- no foreign key (canonical party)
    account_id          UUID,                         -- no foreign key (canonical account, bi-temporal)
    txn_id              UUID,                         -- no foreign key (canonical txn, partitioned by booking date)
    txn_booking_date    DATE,                         -- the partition key of canon.txn, so the lookup prunes (DR-145)
    external_kind       TEXT CHECK (external_kind IN ('bank', 'wallet', 'card', 'phone', 'address', 'other')),
    external_value_enc  BYTEA,                        -- sealed (personal), AAD = id
    added_by            UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    added_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    retired_by          UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    retired_at          TIMESTAMPTZ,
    CHECK ((retired_at IS NULL) = (retired_by IS NULL)),
    CHECK ((txn_id IS NULL) = (txn_booking_date IS NULL)),
    CHECK (
           (entity_kind = 'party'       AND party_id IS NOT NULL AND account_id IS NULL AND txn_id IS NULL
                                        AND external_kind IS NULL AND external_value_enc IS NULL)
        OR (entity_kind = 'account'     AND account_id IS NOT NULL AND party_id IS NULL AND txn_id IS NULL
                                        AND external_kind IS NULL AND external_value_enc IS NULL)
        OR (entity_kind = 'transaction' AND txn_id IS NOT NULL AND party_id IS NULL AND account_id IS NULL
                                        AND external_kind IS NULL AND external_value_enc IS NULL)
        OR (entity_kind = 'external'    AND external_kind IS NOT NULL AND external_value_enc IS NOT NULL
                                        AND party_id IS NULL AND account_id IS NULL AND txn_id IS NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_entity_links_party   ON casework.entity_links (case_id, party_id)
    WHERE entity_kind = 'party' AND retired_at IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uq_entity_links_account ON casework.entity_links (case_id, account_id)
    WHERE entity_kind = 'account' AND retired_at IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uq_entity_links_txn     ON casework.entity_links (case_id, txn_id)
    WHERE entity_kind = 'transaction' AND retired_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_entity_links_party ON casework.entity_links (party_id) WHERE party_id IS NOT NULL;

-- Opened views (DR-136): what a person opened on a case (an evidence item, or a panel). Append-only. The decision gate reads it.
CREATE TABLE IF NOT EXISTS casework.opened_views (
    id          UUID PRIMARY KEY,
    case_id     UUID NOT NULL REFERENCES casework.cases(id) ON DELETE RESTRICT,
    actor_id    UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    view_kind   TEXT NOT NULL CHECK (view_kind IN ('panel', 'evidence')),
    view_key    TEXT NOT NULL CHECK (view_key ~ '^[a-z0-9_-]{1,64}$'),
    opened_at   TIMESTAMPTZ NOT NULL,
    CHECK (
           (view_kind = 'panel'    AND view_key IN ('alert_snapshot', 'party_profile', 'transactions', 'case_timeline',
                                                    'rating', 'screening'))
        OR (view_kind = 'evidence' AND view_key ~ '^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$'))
);
CREATE INDEX IF NOT EXISTS idx_opened_views_gate ON casework.opened_views (case_id, actor_id, view_kind, view_key, opened_at);
```

### 1.2 `0063_casework_decisions_guards_grants.sql`

```sql
-- migration 0063_casework_decisions_guards_grants.sql
-- Phase 18. Guards (second layer under the grants), grants per write path with UPDATE column lists, settings, the
-- classification and the retention rows, and the sealed columns. Reuses casework.append_only() of phase 17 (0061).

-- Notes, decisions, seals, opened views and evidence links are append-only.
DROP TRIGGER IF EXISTS trg_notes_append_only ON casework.notes;
CREATE TRIGGER trg_notes_append_only BEFORE UPDATE OR DELETE ON casework.notes
    FOR EACH ROW EXECUTE FUNCTION casework.append_only();
DROP TRIGGER IF EXISTS trg_decisions_append_only ON casework.decisions;
CREATE TRIGGER trg_decisions_append_only BEFORE UPDATE OR DELETE ON casework.decisions
    FOR EACH ROW EXECUTE FUNCTION casework.append_only();
DROP TRIGGER IF EXISTS trg_case_seals_append_only ON casework.case_seals;
CREATE TRIGGER trg_case_seals_append_only BEFORE UPDATE OR DELETE ON casework.case_seals
    FOR EACH ROW EXECUTE FUNCTION casework.append_only();
DROP TRIGGER IF EXISTS trg_opened_views_append_only ON casework.opened_views;
CREATE TRIGGER trg_opened_views_append_only BEFORE UPDATE OR DELETE ON casework.opened_views
    FOR EACH ROW EXECUTE FUNCTION casework.append_only();
DROP TRIGGER IF EXISTS trg_evidence_items_append_only ON casework.evidence_items;
CREATE TRIGGER trg_evidence_items_append_only BEFORE UPDATE OR DELETE ON casework.evidence_items
    FOR EACH ROW EXECUTE FUNCTION casework.append_only();

-- A note that amends another names a note of the same case.
CREATE OR REPLACE FUNCTION casework.notes_insert_guard() RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE
    v_case UUID;
BEGIN
    IF NEW.amends IS NOT NULL THEN
        SELECT n.case_id INTO v_case FROM casework.notes n WHERE n.id = NEW.amends;
        IF NOT FOUND OR v_case <> NEW.case_id THEN
            RAISE EXCEPTION 'a note amends a note of the same case' USING ERRCODE = 'check_violation';
        END IF;
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_notes_insert_guard ON casework.notes;
CREATE TRIGGER trg_notes_insert_guard BEFORE INSERT ON casework.notes
    FOR EACH ROW EXECUTE FUNCTION casework.notes_insert_guard();

-- Decision rows are written only by the workflow terminal effect; the guard refuses a decision whose proposal has not ended
-- with `completed` in the same transaction (the effect sets it first, section 3.8 step f.2).
CREATE OR REPLACE FUNCTION casework.decisions_insert_guard() RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE
    v_end TEXT;
BEGIN
    SELECT p.end_kind INTO v_end FROM casework.proposals p WHERE p.id = NEW.proposal_id;
    IF v_end IS DISTINCT FROM 'completed' THEN
        RAISE EXCEPTION 'a decision needs its proposal completed in the same transaction' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_decisions_insert_guard ON casework.decisions;
CREATE TRIGGER trg_decisions_insert_guard BEFORE INSERT ON casework.decisions
    FOR EACH ROW EXECUTE FUNCTION casework.decisions_insert_guard();

-- Access grants: never deleted, never changed except the one revocation.
CREATE OR REPLACE FUNCTION casework.case_acl_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'access grants are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.revoked_at IS NOT NULL THEN
        RAISE EXCEPTION 'a revoked access grant does not change' USING ERRCODE = 'check_violation';
    END IF;
    IF (NEW.id, NEW.case_id, NEW.principal_id, NEW.access, NEW.reason, NEW.note_id, NEW.granted_by, NEW.granted_at, NEW.expires_at)
       IS DISTINCT FROM (OLD.id, OLD.case_id, OLD.principal_id, OLD.access, OLD.reason, OLD.note_id, OLD.granted_by, OLD.granted_at, OLD.expires_at) THEN
        RAISE EXCEPTION 'an access grant is revoked, never changed' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_case_acl_guard ON casework.case_acl;
CREATE TRIGGER trg_case_acl_guard BEFORE UPDATE OR DELETE ON casework.case_acl
    FOR EACH ROW EXECUTE FUNCTION casework.case_acl_guard();

-- Proposals: content fixed; the end is set once.
CREATE OR REPLACE FUNCTION casework.proposals_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'proposals are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.ended_at IS NOT NULL THEN
        RAISE EXCEPTION 'an ended proposal does not change' USING ERRCODE = 'check_violation';
    END IF;
    IF (NEW.id, NEW.case_id, NEW.workflow_instance_id, NEW.type, NEW.definition_key, NEW.reason_codes, NEW.text_enc,
        NEW.evidence_refs, NEW.alert_outcomes, NEW.checklist, NEW.origin, NEW.typology, NEW.subjects, NEW.transactions,
        NEW.actions, NEW.earlier_reports, NEW.proposer_id, NEW.created_at)
       IS DISTINCT FROM
       (OLD.id, OLD.case_id, OLD.workflow_instance_id, OLD.type, OLD.definition_key, OLD.reason_codes, OLD.text_enc,
        OLD.evidence_refs, OLD.alert_outcomes, OLD.checklist, OLD.origin, OLD.typology, OLD.subjects, OLD.transactions,
        OLD.actions, OLD.earlier_reports, OLD.proposer_id, OLD.created_at) THEN
        RAISE EXCEPTION 'a proposal is fixed at creation' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_proposals_guard ON casework.proposals;
CREATE TRIGGER trg_proposals_guard BEFORE UPDATE OR DELETE ON casework.proposals
    FOR EACH ROW EXECUTE FUNCTION casework.proposals_guard();

-- Work items: identity fixed; open -> done or waived, then terminal.
CREATE OR REPLACE FUNCTION casework.work_items_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'work items are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF (NEW.id, NEW.case_id, NEW.kind, NEW.description_enc, NEW.created_by, NEW.created_at)
       IS DISTINCT FROM (OLD.id, OLD.case_id, OLD.kind, OLD.description_enc, OLD.created_by, OLD.created_at) THEN
        RAISE EXCEPTION 'a work item identity does not change' USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.state <> 'open' AND (NEW.state IS DISTINCT FROM OLD.state OR NEW.result IS DISTINCT FROM OLD.result) THEN
        RAISE EXCEPTION 'a closed work item is terminal' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_work_items_guard ON casework.work_items;
CREATE TRIGGER trg_work_items_guard BEFORE UPDATE OR DELETE ON casework.work_items
    FOR EACH ROW EXECUTE FUNCTION casework.work_items_guard();

-- Entity links: identity fixed; retired once.
CREATE OR REPLACE FUNCTION casework.entity_links_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'entity links are retired, never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.retired_at IS NOT NULL THEN
        RAISE EXCEPTION 'a retired entity link does not change' USING ERRCODE = 'check_violation';
    END IF;
    IF (NEW.id, NEW.case_id, NEW.entity_kind, NEW.party_id, NEW.account_id, NEW.txn_id, NEW.txn_booking_date,
        NEW.external_kind, NEW.external_value_enc, NEW.added_by, NEW.added_at)
       IS DISTINCT FROM
       (OLD.id, OLD.case_id, OLD.entity_kind, OLD.party_id, OLD.account_id, OLD.txn_id, OLD.txn_booking_date,
        OLD.external_kind, OLD.external_value_enc, OLD.added_by, OLD.added_at) THEN
        RAISE EXCEPTION 'an entity link identity does not change' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_entity_links_guard ON casework.entity_links;
CREATE TRIGGER trg_entity_links_guard BEFORE UPDATE OR DELETE ON casework.entity_links
    FOR EACH ROW EXECUTE FUNCTION casework.entity_links_guard();

-- Grants. Every write path of this phase, with the UPDATE column lists its services use.
-- Phase 17 grants on casework.cases already cover status, version, updated_at and the projection; this phase adds the
-- closure, the seal, the outcome, the workflow version and the escalation level (DR-139, DR-140).
GRANT UPDATE (closed_at, sealed_at, seal_hash, outcome, workflow_version, escalation_level)
    ON casework.cases TO akr_app;
GRANT UPDATE (contribution_outcome, contribution_inherited) ON alert.alerts TO akr_app;   -- phase 17 grants version, updated_at
GRANT SELECT, INSERT ON casework.notes TO akr_app;                 -- no UPDATE, no DELETE: the trigger is the second layer
GRANT SELECT, INSERT ON casework.case_acl TO akr_app;
GRANT UPDATE (revoked_by, revoked_at) ON casework.case_acl TO akr_app;
GRANT SELECT, INSERT ON casework.proposals TO akr_app;
GRANT UPDATE (ended_at, end_kind) ON casework.proposals TO akr_app;
GRANT SELECT, INSERT ON casework.decisions TO akr_app;             -- written by the record_decision effect only
GRANT SELECT, INSERT ON casework.case_seals TO akr_app;
GRANT SELECT, INSERT ON casework.work_items TO akr_app;
GRANT UPDATE (assignee_id, pool_id, due_at, timer_id, state, result, closed_by, closed_at, waived_by, waiver_reason_code,
              updated_at, version) ON casework.work_items TO akr_app;
GRANT SELECT, INSERT ON casework.evidence_items TO akr_app;
GRANT SELECT, INSERT ON casework.entity_links TO akr_app;
GRANT UPDATE (retired_by, retired_at) ON casework.entity_links TO akr_app;
GRANT SELECT, INSERT ON casework.opened_views TO akr_app;
-- Existing grants used by this phase (phase 17): casework.case_events INSERT, casework.case_alerts INSERT and UPDATE(case_id,
-- duplicate_of), casework.case_parties INSERT, casework.party_tiers INSERT and UPDATE, alert.alert_events INSERT, casework.cases
-- UPDATE(status, priority_class, tier, group_id, merged_into, first_worker_id, second_worker_id, assignee_id, assignee_pool_id, last_seq, head_hash, updated_at, version), casework.case_ref_seq. akr_app keeps SELECT on canon.* (the SELECT grant block of phase 10).
-- No grant to akr_readonly, akr_ingest, akr_audit_writer or akr_monitor: these tables are personal and are read through the API.
-- The access log (audit.access_event) is written by the audit writer through AccessLog; akr_app does not read it (DR-136).

-- Settings and schedules (copy of the phase 17 column list; floor NULL throughout: no regulatory minimum of the {min} shape).
-- governed = TRUE only where a change must go through a change request (phase 04 `setting` target).
INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('casework.required_panels',
  '{"P0": ["alert_snapshot", "party_profile", "transactions", "case_timeline", "rating"], "P1": ["alert_snapshot", "party_profile", "transactions"], "P2": [], "P3": []}',
  'json', NULL, TRUE,
  '{"ar": "اللوحات التي يجب أن يفتحها صاحب القرار قبل القرار حسب فئة التنبيه (M-21)", "en": "Evidence panels the decider must open before a decision, per priority class (M-21)"}'),
 ('casework.decision_text_min_chars', '40', 'integer', NULL, TRUE,
  '{"ar": "الحد الأدنى لطول نص سبب القرار بعد التطبيع (حروف)", "en": "Minimum length of decision reason text after folding (characters)"}'),
 ('casework.contribution_defaults',
  '{"no_report": "explained", "utr": "not_explained", "str": "not_explained", "tfr": "not_explained"}',
  'json', NULL, TRUE,
  '{"ar": "نتيجة المساهمة الموروثة لتنبيهات القضية حسب نوع القرار (M-09)", "en": "Inherited contribution outcome of the case alerts per decision type (M-09)"}'),
 ('casework.work_item_due_hours', '72', 'integer', NULL, FALSE,
  '{"ar": "مهلة بنود العمل الافتراضية (ساعات عمل)", "en": "Default work item due time (working hours)"}'),
 ('casework.note_max_chars', '4000', 'integer', NULL, FALSE,
  '{"ar": "أقصى طول لملاحظة القضية (حروف)", "en": "Largest case note (characters)"}'),
 ('casework.decision_reason_max', '5', 'integer', NULL, FALSE,
  '{"ar": "أقصى عدد لرموز أسباب القرار", "en": "Largest number of reason codes on one proposal"}'),
 ('casework.txn_range_max_days', '366', 'integer', NULL, FALSE,
  '{"ar": "أقصى مدى زمني لاستعلام المعاملات في صفحة الطرف (أيام)", "en": "Largest date range of a party transaction query (days)"}'),
 ('casework.txn_page_size', '100', 'integer', NULL, FALSE,
  '{"ar": "حجم صفحة المعاملات في صفحة الطرف", "en": "Transactions per page on the party transaction explorer"}'),
 ('casework.manual_grant_max_days', '30', 'integer', NULL, TRUE,
  '{"ar": "أقصى مدة لمنح وصول يدوي إلى قضية من فئة الاشتباه (أيام)", "en": "Longest manual grant on a suspicion-tier case (days; SP-28)"}')
ON CONFLICT (key) DO NOTHING;

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('schedule.work_item_due_scan', '"0 5 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مسح بنود العمل المتأخرة (احتياطي لمهلة المؤقت)", "en": "Overdue work item backstop scan schedule (fallback for the timer)"}'),
 ('schedule.case_acl_expiry_scan', '"0 15 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مسح منح الوصول المنتهية (احتياطي للمؤقت)", "en": "Expired case grant backstop scan schedule (fallback for the timer)"}')
ON CONFLICT (key) DO NOTHING;

-- Classification (DATA-MODEL s.0 classes). Sealed columns are personal and encrypted (phase 02 personal seal, AAD = row id).
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'casework', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('notes', 'internal', ARRAY['id', 'case_id', 'author_id', 'amends', 'at', 'created_at']),
  ('notes', 'personal', ARRAY['body_enc']),
  ('case_acl', 'internal', ARRAY['id', 'case_id', 'principal_id', 'access', 'reason', 'note_id', 'granted_by', 'granted_at',
                                 'expires_at', 'revoked_by', 'revoked_at']),
  ('proposals', 'internal', ARRAY['id', 'case_id', 'workflow_instance_id', 'type', 'definition_key', 'reason_codes',
                                  'evidence_refs', 'alert_outcomes', 'origin', 'typology', 'subjects', 'actions',
                                  'proposer_id', 'created_at', 'ended_at', 'end_kind']),
  ('proposals', 'personal', ARRAY['text_enc', 'checklist', 'transactions']),
  ('proposals', 'secret-str', ARRAY['earlier_reports']),   -- goAML references of earlier reports (tipping-off)
  ('decisions', 'internal', ARRAY['id', 'case_id', 'proposal_id', 'workflow_instance_id', 'type', 'reason_codes',
                                  'decider_id', 'proposer_id', 'decided_at', 'evidence_refs', 'opened_views', 'origin',
                                  'typology', 'subjects', 'actions', 'alert_outcomes', 'ai_involvement', 'second_review',
                                  'versions', 'stepup_id', 'supersedes', 'created_at']),
  ('decisions', 'personal', ARRAY['text_enc', 'checklist', 'transactions']),
  ('decisions', 'secret-str', ARRAY['earlier_reports']),
  ('case_seals', 'internal', ARRAY['id', 'case_id', 'seq_no', 'outcome', 'reason_code', 'closed_at', 'last_seq', 'head_hash',
                                   'seal_hash', 'sealed_by', 'sealed_at']),
  ('work_items', 'internal', ARRAY['id', 'case_id', 'kind', 'assignee_id', 'pool_id', 'due_at', 'timer_id', 'state',
                                   'created_by', 'created_at', 'closed_by', 'closed_at', 'waived_by', 'waiver_reason_code',
                                   'updated_at', 'version']),
  ('work_items', 'personal', ARRAY['description_enc', 'result']),
  ('evidence_items', 'internal', ARRAY['id', 'case_id', 'object_id', 'added_by', 'added_at']),
  ('evidence_items', 'personal', ARRAY['label']),
  ('entity_links', 'internal', ARRAY['id', 'case_id', 'entity_kind', 'party_id', 'account_id', 'txn_id', 'txn_booking_date', 'external_kind',
                                     'added_by', 'added_at', 'retired_by', 'retired_at']),
  ('entity_links', 'personal', ARRAY['external_value_enc']),
  ('opened_views', 'internal', ARRAY['id', 'case_id', 'actor_id', 'view_kind', 'view_key', 'opened_at'])
) AS v(t, cls, cols)
CROSS JOIN LATERAL unnest(v.cols) AS c
ON CONFLICT DO NOTHING;

-- The sealed columns (phase 02 personal seal).
UPDATE admin.column_classification SET encrypted = TRUE
 WHERE (schema_name, table_name, column_name) IN (
   ('casework', 'notes', 'body_enc'),
   ('casework', 'proposals', 'text_enc'),
   ('casework', 'decisions', 'text_enc'),
   ('casework', 'work_items', 'description_enc'),
   ('casework', 'entity_links', 'external_value_enc'));

-- Retention (phase 00 clocks; R6 cases, their trail and the decision record, ARCHITECTURE s.17). Children read the parent's
-- closure (the engine reads casework.cases.closed_at); the clock column is the child's own timestamp, as in phase 17.
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('casework', 'notes',          'R6', 'closure', 'at',          'Case notes: follow the case'),
 ('casework', 'case_acl',       'R6', 'closure', 'granted_at',  'Access history of the case: follows the case'),
 ('casework', 'proposals',      'R6', 'closure', 'created_at',  'Proposals: follow the case; the decision is the record'),
 ('casework', 'decisions',      'R6', 'closure', 'decided_at',  'Decisions: seven years from the later of closure and report (research 28 s.6.1)'),
 ('casework', 'case_seals',     'R6', 'closure', 'sealed_at',   'Seals: kept with every closure of the case'),
 ('casework', 'work_items',     'R6', 'closure', 'created_at',  'Work items: follow the case'),
 ('casework', 'evidence_items', 'R6', 'closure', 'added_at',    'Evidence links: follow the case; the bytes follow phase 02 R6'),
 ('casework', 'entity_links',   'R6', 'closure', 'added_at',    'Entity links of the case: follow the case'),
 ('casework', 'opened_views',   'R6', 'closure', 'opened_at',   'What the decider opened: part of the decision record')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

## 2. DDD Implementation (Rust)

Crate placement follows phase 17: the domain in `domain::casework` (akr-core, no IO), the repositories in `infrastructure::casework`, the services in `services::casework` and `services::decisions`, the routers in `api::routes`. The sealed fields (`body_enc`, `text_enc`, `external_value_enc`, `description_enc`) are held in the domain as `Sealed<T>`: the plaintext exists only inside the service, `Sealed<T>` does not derive `Serialize`, and every response DTO is built from an explicit view struct, so a sealed value cannot reach a response by accident.

**Domain entities** (derives `Debug, Serialize, Deserialize, FromRow, ToSchema`, except `Sealed<T>` as above):
- `CaseNote { id, case_id, author_id, body: Sealed<String>, amends: Option<Uuid>, at }`
- `CaseAccess { id, case_id, principal_id, access: AccessGrant, reason: AccessReason, note_id: Option<Uuid>, granted_by: Option<Uuid>, granted_at, revoked_by: Option<Uuid>, revoked_at: Option<DateTime<Utc>> }`
- `Proposal { id, case_id, workflow_instance_id, proposal_type: CaseOutcome, definition_key: DefinitionKey, reason_codes: Vec<String>, text: Sealed<String>, evidence_refs: Vec<Uuid>, alert_outcomes: Vec<AlertOutcome>, checklist: CbiChecklist, origin: Origin, typology: Option<String>, subjects: Vec<Uuid>, transactions: Vec<TxnRef>, actions: Vec<ActionItem>, earlier_reports: Vec<EarlierReport>, proposer_id, created_at, ended_at: Option<DateTime<Utc>>, end_kind: Option<ProposalEnd> }`
- `Decision { id, case_id, proposal_id, workflow_instance_id, decision_type: CaseOutcome, reason_codes, text: Sealed<String>, decider_id, proposer_id, decided_at, evidence_refs, opened_views: Vec<OpenedViewRef>, checklist, origin, typology, subjects, transactions, actions, earlier_reports, alert_outcomes, ai_involvement: serde_json::Value, second_review: bool, versions: serde_json::Value, stepup_id: Option<Uuid>, supersedes: Option<Uuid> }`
- `CaseSeal { id, case_id, seq_no, outcome: CaseOutcome, reason_code, closed_at, last_seq, head_hash: [u8; 32], seal_hash: [u8; 32], sealed_by, sealed_at }`
- `WorkItem { id, case_id, kind: WorkItemKind, description: Option<Sealed<String>>, assignee_id, pool_id, due_at, timer_id, state: WorkItemState, result: serde_json::Map<String, Value>, created_by, created_at, closed_by, closed_at, waived_by, waiver_reason_code, updated_at, version: i32 }`
- `EvidenceItem { id, case_id, object_id, label, added_by, added_at }`
- `EntityLink { id, case_id, kind: EntityKind, party_id, account_id, txn_id, txn_booking_date: Option<NaiveDate>, external: Option<(ExternalKind, Sealed<String>)>, added_by, added_at, retired_by, retired_at }`
- `OpenedView { id, case_id, actor_id, kind: ViewKind, key: String, opened_at }`

**Enums** (closed in code; the SQL CHECK lists are the same values): `CaseOutcome { NoReport => "no_report", Utr => "utr", Str => "str", Tfr => "tfr" }` (named so as not to reuse phase 09's `DecisionType`, the closed authority-matrix vocabulary that this phase calls with `case.decision`); `DefinitionKey { CaseCloseNoReport, CaseReportApproval }` (the pairing with `CaseOutcome` is a unit-tested function, and the table CHECK repeats it); `AccessGrant { Read, Work, Decide }`; `AccessReason { Assignment, Escalation, SecondReview, MlroDesk, Manual }`; `WorkItemKind { ObtainDocument, ContactBranch, ReviewEvidence, Other }`; `WorkItemState { Open, Done, Waived }`; `ViewKind { Panel, Evidence }` with `PanelKey { AlertSnapshot, PartyProfile, Transactions, CaseTimeline, Rating, Screening }`; `ProposalEnd { Completed, Rejected, Withdrawn, Failed }`; `EntityKind { Party, Account, Transaction, External }`; `ExternalKind { Bank, Wallet, Card, Phone, Address, Other }`; `DecisionAction { Approve, Sign, Reject }`.

**Aggregate** (`CaseAggregate`, section 3.1): the pure state of one case (status, escalation level, outcome, sealed flag, open proposal, assignee set) with `apply(&self, command) -> Result<Vec<CaseEventDraft>, CaseError>`; no IO.

**Create, command and view DTOs** (`Deserialize, ToSchema`). Every change is a named command, so there is no generic Update DTO:
- `CreateNote { body: String (1 to casework.note_max_chars), amends: Option<Uuid> }`
- `UploadEvidence` (multipart): `file`, `label: String (1 to 120)`
- `OpenView { view_kind: ViewKind, view_key: String }`
- `CreateProposal { proposal_type, reason_codes: Vec<String> (1 to casework.decision_reason_max), text: String, evidence_refs: Vec<Uuid>, alert_outcomes: Vec<AlertOutcomeInput>, checklist: CbiChecklist, origin: Origin, typology: Option<String>, subjects: Vec<Uuid>, transactions: Vec<TxnRef { txn_id, booking_date }>, actions: Vec<ActionItem>, earlier_reports: Vec<EarlierReport> }`; query `dry_run: bool`
- `CbiChecklist { criteria: [CriterionAnswer; 7], notes: Option<String> }`, `CriterionAnswer` (`yes`, `no`, `not_applicable`) with `note: Option<String>` for `yes` and `not_applicable`
- `ActOnDecision { proposal_id: Uuid, task_id: Uuid, action: DecisionAction, comment: Option<String> }` (`comment` required for `reject`)
- `Escalate { to_level: u8, note: String }`; `CloseCase { reason_code: String, comment: Option<String> }`; `ReopenCase { reason_code: String }`
- `GrantAccess { principal_id: Uuid, access: AccessGrant, reason: AccessReason, note: Option<String> }` (`note` required when `reason` is `manual`)
- `CreateWorkItem { kind, description: Option<String>, assignee_id: Option<Uuid>, pool_id: Option<Uuid>, due_at: Option<DateTime<Utc>> }`; `CompleteWorkItem { result: serde_json::Map<String, Value> (at most 2048 bytes) }`; `WaiveWorkItem { reason_code: String }`
- `CreateEntityLink { entity_kind, party_id, account_id, txn_id, txn_booking_date, external_kind, external_value }` (`txn_booking_date` required with `txn_id`)
- Views: `CaseNoteView { id, author: PersonRef, body, amends, at }`; `ProposalView { ...fields of Proposal except sealed text unless the caller is on the case team, gate: GateReport, instance_version: i32, task: Option<TaskRef> }`; `GateReport { missing_panels: Vec<String>, missing_evidence: Vec<Uuid>, evidence_not_clean: Vec<Uuid> }`; `DecisionView`; `AccessView`; `SealView`; `WorkItemView`; `EvidenceItemView { id, label, added_by, added_at, av_status, lang, sha256_prefix }` (the sha256 is shown as its first 16 hex characters; the full value is in the open response headers); `EntityLinkView { id, kind, masked: bool, party_id, account_id, txn_id, external_kind, value: Option<String> }`; `PartyPage`; `PartyTxnRow`; `ShelvedRow { decision_id, case_ref, decided_at, reason_codes, decider: PersonRef, evidence_count }` (no text, no party names).

**Repository traits** (domain layer; all take `&mut PgConnection` or `&mut Tx` as their method says):
- `CaseNoteRepository`: `insert(tx, NewNote) -> CaseNote`; `list(conn, case_id, page) -> Page<CaseNote>`; `get(conn, id) -> Option<CaseNote>`.
- `CaseAccessRepository`: `active(conn, case_id) -> Vec<CaseAccess>`; `active_for(conn, case_id, principal) -> Option<CaseAccess>`; `insert(tx, NewAccess)`; `revoke(tx, id, by, at)`; `has_decide(conn, case_id, principal) -> bool`.
- `ProposalRepository`: `running(conn, case_id) -> Option<Proposal>`; `get(conn, id) -> Option<Proposal>`; `by_instance(conn, instance_id) -> Option<Proposal>`; `insert(tx, NewProposal) -> Proposal`; `end(tx, id, end_kind, at)`; `list(conn, case_id, page)`.
- `CaseDecisionRepository`: `insert(tx, NewDecision) -> Decision`; `list(conn, case_id) -> Vec<Decision>`; `shelved(conn, filter, page) -> Page<ShelvedRow>`; `latest_for_case(conn, case_id) -> Option<Decision>`.
- `CaseSealRepository`: `insert(tx, NewSeal) -> CaseSeal`; `count(conn, case_id) -> i32`; `list(conn, case_id)`.
- `WorkItemRepository`: `insert(tx, NewWorkItem)`; `get_for_update(tx, id) -> Option<WorkItem>`; `list(conn, case_id)`; `open_count(conn, case_id) -> i64`; `set_closed(tx, id, state, actor, at, reason_code)`; `set_timer(tx, id, timer_id)`.
- `EvidenceItemRepository`: `insert(tx, NewEvidenceItem)`; `list(conn, case_id) -> Vec<EvidenceItem>`; `get(conn, case_id, id) -> Option<EvidenceItem>`.
- `EntityLinkRepository`: `insert(tx, NewEntityLink)`; `list(conn, case_id)`; `retire(tx, id, by, at)`.
- `OpenedViewRepository`: `insert(tx, NewOpenedView)`; `since(conn, case_id, actor, proposal_created_at) -> Vec<OpenedView>` (the gate query, section 3.9).
- `CaseRepository` (phase 17) is extended with `set_closure(tx, id, closed_at, sealed_at, seal_hash, outcome)`, `set_reopened(tx, id, at)`, `set_status(tx, id, status, at)`, `set_escalation_level(tx, id, level, at)`, `set_workflow_version(tx, id, version)`. Its `lock(tx, id)` (phase 17, section 3.7) is the only way the services take the case row.
- `CanonPartyReader` (read only, SELECT on canon, phase 10): `current_party(conn, id)`, `current_names(conn, id)`, `current_accounts(conn, party_id)` (joins `canon.account_party_role` where `recorded_to IS NULL`), `transactions(conn, account_ids, filter, cursor)`.

**Pg implementations** (`PgCaseNoteRepository` and the others): `RETURNING` on every insert; the party reader reads `canon.party` where `recorded_to IS NULL` and `canon.party_name` where `superseded_at IS NULL` (the name table is not bi-temporal, phase 10); the transaction reader reads `canon.txn` with `booking_date` bounds (the partition key), so partition pruning applies. The shelved register joins `casework.decisions` to `casework.cases` for `case_ref` and to `admin.users` for the decider's display name.

**utoipa schemas**: every DTO and view above, with `ToSchema`; `Problem` (API section 6 envelope) is shared.

**Route table** (scopes from API.md section 2; "New" marks a row added to API.md 3.12 by this phase; "Owned" marks a row of API.md 3.12 that this phase builds; the markers are those of API.md section 3):

| Method | Path | Scope | Description (cedar action) |
|-|-|-|-|
| GET | `/api/v1/cases/{id}/notes` | `queue.work` | Notes of the case, oldest first, cursor (New; `case.note.read`) |
| POST | `/api/v1/cases/{id}/notes` | `queue.work` | Post a note; `amends` names a note of the same case; `Idempotency-Key`; no `If-Match` (Owned; `case.note.write`) |
| GET | `/api/v1/cases/{id}/evidence` | `queue.work` | Evidence items with their scan status (New; `case.evidence.read`) |
| POST | `/api/v1/cases/{id}/evidence` | `queue.work` | Upload evidence through `EvidenceStore::stage`; 422 `E_UPLOAD_REJECTED` on an antivirus or type refusal; `Idempotency-Key` (Owned; `case.evidence.add`) |
| GET | `/api/v1/cases/{id}/evidence/{evidence_id}/content` | `queue.work` | The verified bytes; access event first; step-up `record_open` on a suspicion case; `download=true` records `download` (New; `case.evidence.read`) |
| POST | `/api/v1/cases/{id}/opened-views` | `queue.work` | Record that a panel was opened (DR-136; an evidence item is recorded by its content read, section 3.4); `Idempotency-Key` (New; `case.read`) |
| GET | `/api/v1/cases/{id}/proposals` | `queue.work` | The running or last proposal, the decider's gate report, the decider's open task and the instance version as ETag (New; `case.read`) |
| POST | `/api/v1/cases/{id}/proposals` | `queue.work` | Propose no report, UTR, STR or TFR; `dry_run=true` runs the checks and writes nothing; `If-Match` (case version); `Idempotency-Key` (Owned; `case.propose`) |
| GET | `/api/v1/cases/{id}/decisions` | `queue.work` | Decisions with versions and `ai_involvement`; the text only to the case team; access event on a suspicion case (Owned; `case.read`) |
| POST | `/api/v1/cases/{id}/decisions` | `case.decide` | Approve, sign or reject the open proposal (acts on my task); **SU** (class `decision`); **4E**; `If-Match` (instance version); `Idempotency-Key` (Owned; `task.approve`, `task.sign`, `task.reject`, `case.decide`) |
| POST | `/api/v1/cases/{id}/escalate` | `queue.work` | Raise the escalation level by one with a note; `If-Match`; `Idempotency-Key` (Owned; `case.escalate`) |
| POST | `/api/v1/cases/{id}/close` | `case.decide` | The closing sequence: outcome fixed, work items done or waived, seal; **SU** (class `decision`); `If-Match`; `Idempotency-Key` (New; `case.close`) |
| POST | `/api/v1/cases/{id}/reopen` | `case.decide` | Reopen a closed case with a closure reason; **SU** (class `decision`); `If-Match`; late reopen is M only (Owned; `case.reopen`, `case.reopen_late`) |
| GET | `/api/v1/cases/{id}/access` | `queue.work` | The case access list with reasons and grant times (New; `case.access.read`) |
| POST | `/api/v1/cases/{id}/access` | `case.decide` | Grant read, work or decide to a person; `If-Match`; `Idempotency-Key` (New; `case.access.change`) |
| DELETE | `/api/v1/cases/{id}/access/{principal_id}` | `case.decide` | Revoke the grant; refused for the current assignee; `If-Match` (New; `case.access.change`) |
| GET | `/api/v1/cases/{id}/work-items` | `queue.work` | Work items of the case (New; `case.work.read`) |
| POST | `/api/v1/cases/{id}/work-items` | `queue.work` | Open a work item with a due time; `Idempotency-Key` (New; `case.work.write`) |
| POST | `/api/v1/cases/{id}/work-items/{work_item_id}/complete` | `queue.work` | Mark done with a result; `If-Match` (work item version) (New; `case.work.write`) |
| POST | `/api/v1/cases/{id}/work-items/{work_item_id}/waive` | `case.decide` | Waive by a second person with a closure code; `If-Match` (New; `case.work.waive`) |
| GET | `/api/v1/cases/{id}/links` | `queue.work` | Entity links of the case with masking applied (New; `case.link.read`) |
| POST | `/api/v1/cases/{id}/links` | `queue.work` | Link a party, account, transaction or external identifier; `Idempotency-Key` (New; `case.link.write`) |
| GET | `/api/v1/parties/{id}` | `queue.work` | Party page: profile, names, current accounts, rating level and label, the cases I may see (Owned; `party.read`) |
| GET | `/api/v1/parties/{id}/transactions` | `queue.work` | Transactions of the party's current accounts; date range at most 366 days; cursor (Owned; `party.transactions.read`) |
| GET | `/api/v1/shelved-decisions` | `case.decide` | The shelved-decision register: `no_report` decisions with codes, decider and evidence count; no text (New; `case.shelved.read`) |

Count: 25 product routes in this table (9 owned rows of API.md 3.12, 16 added rows); sections 7.11, 7.12 and 7.13 add one route each, so the phase has 28; no dev route. Phase 17's `GET /cases/{id}` gains the fields listed in section 4.5, not a new row.

**Router**: `pub fn router() -> Router<AppState>` in `api::routes::case_records` (notes, evidence, opened views, links, work items, access), `api::routes::decisions` (proposals, decisions, escalate, close, reopen, shelved register) and `api::routes::parties` (party page, transactions). The phase 17 `cases` router is extended, not replaced.

## 3. Business Logic

Every mutation below appends a case event through `CaseChain` (phase 17 section 3.7) under the case row lock, writes the audit event of section 3.18 in the same transaction, and bumps `casework.cases.version`. Reads of a suspicion-tier record write an access event (phase 02 `AccessScope`) before the body is sent. Sensitive writes carry `Idempotency-Key`; versioned writes carry `If-Match` (428 without it, 409 `E_VERSION_CONFLICT` with the current version otherwise).

### 3.1 The case aggregate (`CaseAggregate`; M-74, DR-134)

Pure state machine. Its transitions are the only rules the services apply to `casework.cases.status`, `escalation_level`, `outcome` and the seal fields:

| From | Command | To | Guard |
|-|-|-|-|
| `open` | start proposal | `in_review` | no running proposal (`uq_proposals_running`); not sealed; `outcome` is NULL (a decided case is closed or reopened, never decided twice) |
| `in_review` | proposal ends `rejected`, `withdrawn` or `failed` | `open` | the proposal is the running one |
| `in_review` | decision recorded (instance completed) | `open` | the proposal is the running one; `outcome` set |
| `open` | escalate | `open` | `escalation_level` < 3 (raised by one only) |
| `open` | close | `closed` | `outcome` set; no open work item; no running proposal; for `utr`, `str`, `tfr` a `case.filing_released` event exists (Open doubts 10) |
| `closed` | reopen | `open` | a closure reason; no other open case of the same party and family (the partial UNIQUE of phase 17) |
| `open`, `in_review` | merge (phase 17) | `merged` | terminal |

Invariants, each a property test (section 6, check 4): I1 `closed` implies `outcome` and `sealed_at` are set and `seal_hash` is 32 bytes; I2 `open` or `in_review` implies the seal fields are NULL; I3 at most one proposal is running per case; I4 a decision implies `in_review` before it and `open` after it; I5 `merged` absorbs every command; I6 `escalation_level` never falls, also across a reopen; I7 a sealed case accepts no note, evidence, link, work item, access change or proposal; I8 the number of chain events of a case equals `last_seq` after any command sequence, and the chain verifies (phase 17 `CaseChain::verify`).

`apply(&self, command) -> Result<Vec<CaseEventDraft>, CaseError>` returns the events that the service writes; the service never writes a status it did not get from `apply`.

### 3.2 The case access list (`CaseAccessService`; DR-133)

**Meaning.** A row says that a person is on the case, with an access level (`read` sees the team's work, `work` may work items and notes, `decide` may approve a proposal), a reason from the closed list and, for a manual grant, the note that gives the reason. The team of a case (for the visibility of phase 17 section 3.10) is: the assignees (`first_worker_id`, `second_worker_id`, `assignee_id`), the members of `assignee_pool_id`, and the active grants of this table. The approval eligibility of the `case_acl` assignee kind (phase 04) is an active grant with `decide`.

**Sources of grants.**
- Assignment (phase 17 `AssignmentService`, extended here): the assignee gets `work` when the holder is A1 and `decide` when the holder is A2 (reason `assignment`); the second worker gets `decide` (reason `second_review`). A reassignment revokes the previous assignee's grant of these two reasons (`revoked_by` is the actor); a grant of another reason stays, and an assignee who already holds a grant of the same or a wider access keeps it (phase 17 section 7.25).
- Escalation to level 3 (section 3.11): every active holder of role `M` (phase 01 `admin.role_grants`, `revoked_at IS NULL`) gets `decide` (reason `mlro_desk`, `granted_by` NULL).
- Manual grant: `POST /cases/{id}/access` with a note (reason `manual`).

**`grant(ctx, case_id, GrantAccess, if_match) -> AccessView`** (`case.access.change`; A2 and M):
1. Cedar `case.access.change` on Case; the forbids of section 3.19 apply.
2. `lock(tx, case_id)`; `If-Match` equals `version` (428 or 409); status `closed` -> 409 `E_CASE_SEALED`.
3. The principal is an active user (`admin.users.status = 'active'`, phase 01); `access = decide` requires the principal to hold `A2` or `M` (422 `E_VALIDATION` at `/access` with sub-code `role_cannot_decide`); `access = read` or `work` requires `A1`, `A2` or `M`.
4. An active grant of the same principal exists -> 409 `E_NOT_ALLOWED_IN_STATE` with sub-code `grant_exists`; the caller revokes first.
5. `reason = manual`: `note` is required and goes through `NoteService::post` (section 3.3) in the same transaction; the note id is stored on the grant.
6. Insert `casework.case_acl`; chain `case.acl_granted {principal_id, access, reason}`; audit `case.acl_granted`; `version + 1`.

**`revoke(ctx, case_id, principal_id, if_match)`** (`case.access.change`): steps 1 and 2 as above; the principal's active grant must exist (404 `E_NOT_FOUND` otherwise); a principal who is the current `first_worker_id`, `second_worker_id` or `assignee_id` cannot be revoked (409 `E_NOT_ALLOWED_IN_STATE`, sub-code `assignee_on_case`) until reassigned; the guard of the table sets `revoked_by` and `revoked_at` once; chain `case.acl_revoked {principal_id}`; audit `case.acl_revoked`.

**`list(ctx, case_id)`** (`case.access.read`): the active grants with `access`, `reason` and `granted_at`; the principal's display name; the revoked grants are not listed (the trail keeps them).

### 3.3 Notes and the generic-text guard (`NoteService`, `GenericTextGuard`; M-59, M-68, UX-65, DR-137)

**`GenericTextGuard::check(conn, text, min_chars) -> TextCheck`** (pure over the policy pack):
1. `PolicyPackService::scan(scope = "reason_text", text)` (phase 09 section 3.5, `ForbiddenWordScanner`) returns `Hit { term_id, start, end }` spans. The guard never returns the term; the author sees the span and the field.
2. Length: the folded text (`text::fold`, phase 09) must have at least `min_chars` letters or digits. `min_chars` is `casework.decision_text_min_chars` (40, governed) for decision text and for manual-grant notes, and 1 for an ordinary note.
3. Result: `Ok`, or `Refused { hits: Vec<Span>, too_short: bool }`. A refusal is 422 `E_VALIDATION` at the field, with sub-codes `forbidden_term` (spans in `errors[].spans`) and `text_too_short`.

The generic phrase "no suspicion" and the other forbidden terms of scope `reason_text` are the bank's list (phase 09). Capacity wording ("no time", "backlog", "workload") is not in the seed; the compliance owner adds it to the policy pack, with no code change (Open doubts 5).

**`post(ctx, case_id, CreateNote) -> CaseNoteView`** (`case.note.write`; A1, A2, M):
1. Cedar `case.note.write` on Case; the section 3.19 forbids apply.
2. `lock(tx, case_id)`; a closed case -> 409 `E_CASE_SEALED`; the body is at most `casework.note_max_chars` (422 `E_VALIDATION` at `/body`).
3. `amends`: a note of the same case (422 `E_VALIDATION` at `/amends`, sub-code `not_on_case`).
4. `GenericTextGuard::check(conn, body, min_chars)`: `min_chars` is 1 for an ordinary note and `casework.decision_text_min_chars` for the note of a manual grant (section 3.2 `grant` step 5). The minimum is an internal argument of `post`, not a field of `CreateNote`: the note route passes 1, and `grant` step 5 passes the setting.
5. `note_id` = UUIDv7; `CaseChain::append_event(tx, case_id, "case.note_posted", {note_id, amends})` returns the chain time `at`.
6. Insert `casework.notes` with `body_enc` sealed under AAD = `note_id` and `at` from step 5.
7. Audit `case.note_posted {note_id, amends}`.

A note is posted, never edited; an amendment is a new note with `amends` (DESIGN-SYSTEM 7.14). Drafts are kept in the browser and are never evidence (IR-24).

**`list(ctx, case_id, cursor)`** (`case.note.read`): oldest first; the body is decrypted in the service; a suspicion-tier case writes one access event per page (`action = view`, `resource_type = case`).

### 3.4 Evidence items (`EvidenceService`; M-58, M-60, DR-119 pattern of phase 17)

**Upload** (`POST /cases/{id}/evidence`; `case.evidence.add`):
1. Cedar `case.evidence.add`; the section 3.19 forbids apply.
2. `lock(tx, case_id)`; a closed case -> 409 `E_CASE_SEALED`.
3. `EvidenceStore::stage(tx, NewEvidence { kind: Upload, owner_kind: "case", owner_id: case_id, retention_class: R6, content_type, file_name, classification: personal, lang, bytes }, actor)` (phase 02 `stage`, steps 1 to 5: size, type, owner registry, PUT to `akr-quarantine`, the row with `av_status = pending`, the job `evidence_scan`). The owner kind `case` is registered by this phase (section 4.5). The PUT runs under the case lock; uploads are rare and the lock is held for the transfer only (Open doubts 25 on very large uploads).
4. `EvidenceError::TypeNotAllowed | Rejected | Infected | TooLarge | Empty` map to 422 `E_UPLOAD_REJECTED` (sub-code in `errors[]`) or to the phase 02 status; no row is written for a refusal.
5. `casework.evidence_items` insert (`object_id`, `label`, `added_by`); chain `case.evidence_added {evidence_id, object_id}`; audit `case.evidence_added`.
6. Response 201 with `av_status: pending`; W3 shows "being checked" until `evidence.stored` (phase 02 step 6).

**Content** (`GET /cases/{id}/evidence/{evidence_id}/content`; `case.evidence.read`):
1. Cedar `case.evidence.read`; the evidence item must belong to the case, else 404 `E_NOT_FOUND`.
2. Suspicion tier: `StepUpService::consume(tx, ctx, X-StepUp-Assertion, CASE_EVIDENCE_OPEN, Some("case:<id>"))` (phase 01; action `case.evidence.open`, class `record_open`); 401 `E_STEPUP_REQUIRED` otherwise.
3. The object must be clean and not shredded; otherwise 404 `E_NOT_FOUND` (phase 02 `NotClean` and `Shredded` reach callers as 404).
4. `AccessScope.record(Access { resource_type: "evidence", resource_id, subject_party: case.party_id, tier: effective tier of the case, action: view or download (query `download=true`) })` (the purpose comes from `X-Access-Purpose`, phase 02 step 2); `PgAccessLog::record` commits in its own transaction before anything else.
5. `EvidenceStore::open(object_id) -> VerifiedReader` (phase 02); a hash mismatch before the first byte is 500 `E_HASH_MISMATCH`, a mismatch after it resets the stream (phase 02).
6. In its own transaction: insert `casework.opened_views` (`view_kind = evidence`, `view_key = evidence_id`) and audit `case.view_opened {view_kind, view_key}`. The row is written after the access event, so the gate (section 3.9) never counts a view whose access was not recorded.

### 3.5 Entity links and masking (`EntityLinkService`, `EntityMask`; DR-142, DR-145)

**Add** (`POST /cases/{id}/links`; `case.link.write`): Cedar; `lock`; a closed case -> 409 `E_CASE_SEALED`; the referenced party, account or transaction must exist in the canonical store (party and account: current row, `recorded_to IS NULL`; transaction: `canon.txn` by `(id, booking_date)` from `txn_id` and `txn_booking_date`, so one partition is read); an active link of the same target -> 409 `E_NOT_ALLOWED_IN_STATE` with sub-code `link_exists`; an external value is sealed (AAD = link id) and its kind is from the closed list. Insert; chain `case.link_added {link_id, entity_kind}`; audit `case.link_added`.

**Retire** (not a route in this phase; the column pair exists so a wrong link can be retired through a later route, Open doubts 12): one-time update by the guard of section 1.2.

**`EntityMask::resolve(conn, link, principal, network, purpose) -> EntityLinkView`** (read time, never stored):
1. Owner party: a `party` link is its own party; an `account` link is the account's current `owner` party (`canon.account_party_role`, role `owner`, `recorded_to IS NULL`; no owner -> the account is shown without a party); a `transaction` link is the owner party of `canon.txn.account_id` (read by `(id, booking_date)`); an `external` link has no party.
2. Tier: `casework.effective_tier(owner party)` (phase 17; never `canon.party.sensitivity_tier` alone).
3. Shown when the tier is `normal`; for `restricted`, when the principal holds A1, A2 or M and the purpose is `assigned_work` or `investigation`; for `suspicion`, when the same rule as restricted holds, the network is `analyst`, and the principal is M or a member of the owner party's open-case team (section 3.10 applied to that party's cases).
4. Otherwise `masked: true` with the kind only: no party id, account id, transaction id or value. A masked link answers the same shape as a link the caller cannot see, so the count of links is not a leak.
5. An external link (no party) is shown to every principal with `case.link.read` on the case.

### 3.6 Work items (`WorkItemService`; M-38, DR-144)

**Open** (`POST /cases/{id}/work-items`; `case.work.write`): Cedar; `lock`; the case must be `open` or `in_review` (409 `E_CASE_SEALED` when closed); `kind` in the closed list; `assignee_id` must hold an active grant on the case (`work` or `decide`), or `pool_id` must be a pool of the case team, else 422 `E_VALIDATION` at `/assignee_id` with sub-code `assignee_not_on_case`; `assignee_id` and `pool_id` are not both set (the table CHECK); `due_at` defaults to `BusinessClock::add(now, casework.work_item_due_hours, BusinessUnit::Hours).fire_at` (phase 09). Insert with `state = open`; schedule the timer `work_item_due` at `due_at` (phase 00 `TimerService`, owner kind `work_item`); chain `case.work_item_opened {work_item_id, kind}`; audit `case.work_item_opened`.

**Complete** (`POST .../complete`; `case.work.write`; `If-Match` on the work item version): the caller is the assignee, or a grant holder of the case; `state = open` (409 otherwise); `result` is an object of at most 2048 bytes (422 `E_VALIDATION` at `/result`); sets `closed_by`, `closed_at`, `state = done`; chain `case.work_item_completed`; audit `case.work_item_completed`; cancels the timer (optimisation only; the handler re-checks).

**Waive** (`POST .../waive`; `case.work.waive`; A2 and M): `reason_code` is an active `closure` code (422 `E_VALIDATION` at `/reason_code`); the waiver is by a second person: a waiver by the creator is 403 `E_SELF_APPROVAL`; sets `waived_by`, `waiver_reason_code`, `state = waived`; chain `case.work_item_waived`; audit `case.work_item_waived`.

**Timer `work_item_due`** (job `timer_work_item_due`, section 4.1): re-reads the item; when `state = open` and `due_at <= now()`, writes chain `case.work_item_overdue {work_item_id}` and audit `case.work_item_overdue`. The timer fires once; a completed or waived item does nothing (phase 00 rule). Breach actions and supervisor tasks are phase 19.

### 3.7 Proposals (`DecisionService::propose`; M-01, M-02, M-26, M-27, M-28, UX-64, UX-65)

Route `POST /cases/{id}/proposals` (`case.propose`; A1 and A2 only: an M holder decides and does not propose, Cedar in section 3.19).

1. Cedar `case.propose` on Case; the section 3.19 forbids apply; `X-Access-Purpose` is required on a restricted or suspicion case (422 `E_PURPOSE_REQUIRED`).
2. `lock(tx, case_id)`; `If-Match` equals the case `version`. Status `in_review` -> 409 `E_NOT_ALLOWED_IN_STATE` (sub-code `proposal_running`); `closed` -> 409 `E_CASE_SEALED`; `merged` -> 409 `E_NOT_ALLOWED_IN_STATE`; `outcome` already set -> 409 `E_NOT_ALLOWED_IN_STATE` (sub-code `already_decided`: the next step is closure; a different outcome needs a reopen, which clears `outcome`, and the new decision names the old one in `supersedes`).
3. Validation, collecting every error in `errors[]` (422 `E_VALIDATION`):
   - `reason_codes`: 1 to `casework.decision_reason_max` codes; each in `ref.reason_codes` with `family = 'decision'` and `active = TRUE` (sub-code `reason_code_inactive` or `reason_family_mismatch`); a `closure` code is refused here.
   - `checklist`: exactly the seven criteria of the CBI circular of 26 August 2026 (research file 28 section 6.3), each answered `yes`, `no` or `not_applicable`; a `yes` or `not_applicable` answer needs a note (sub-code `note_required`). Criteria 1, 3 and 6 are pre-filled by the UI from the alerts and the graph; the stored answer is the analyst's.
   - `text`: `GenericTextGuard::check(conn, text, casework.decision_text_min_chars)` (section 3.3).
   - `evidence_refs`: each an evidence item of this case (sub-code `not_on_case`); each object clean (sub-code `evidence_not_clean`).
   - `alert_outcomes`: each a scenario alert attached to this case (sub-code `not_on_case`); `outcome` one of `explained`, `not_explained`, `data_error` (`duplicate` and `not_assessed` are not chosen by a person).
   - `origin.kind` in `scenario`, `list_match`, `staff_report`, `authority_request`, `rfi_answer`, `adverse_media`, with `origin.ref` optional (M-02).
   - `typology` at most 40 characters; `subjects` and `transactions` exist in the canonical store; `actions.kind` in `enhanced_monitoring`, `rating_change`, `restriction`, `hold_task`, `exit_proposal` (recorded only; the work they start is phases 15 and 20); `earlier_reports` each with `goaml_ref` (at most 64 characters), `date` and `type` in `utr`, `str` (M-02).
4. `dry_run=true`: return 200 with the check results (spans, not terms) and write nothing.
5. `WorkflowEngine::start(tx, StartSpec { definition_key: case_close_no_report (type no_report) or case_report_approval (utr, str, tfr), resource: ("case", case_id), purpose: Decision, maker: proposer })` (phase 04 step 2; `start_context` of section 3.8).
6. `proposal_id` = UUIDv7; insert `casework.proposals` with `text_enc` sealed under AAD = `proposal_id` and `workflow_instance_id` from step 5.
7. `set_status(case, in_review)`; chain `case.proposal_started {proposal_id, type}`; audit `case.proposal_started`; `version + 1`.
8. Response 201: `ProposalView` with the gate report (empty until the decider opens the items) and the instance version as ETag.

The authority matrix is checked at the decision, against the decider (section 3.8 step d.v), not here.

### 3.8 Decisions (`DecisionService::act`, `CaseWorkflowSubject`, `RecordDecisionEffect`, `ReleaseForFilingEffect`; M-19, M-20, M-09, DR-134, DR-141, DR-143)

**Route** (`POST /cases/{id}/decisions`; scope `case.decide`; Cedar `case.decide` on Case; SU class `decision`; `If-Match` = instance version from `GET /cases/{id}/proposals`; `Idempotency-Key`; `X-Access-Purpose` required):
1. The body names `proposal_id` (must be the running proposal of the case, else 409 `E_NOT_ALLOWED_IN_STATE`), `task_id` (an open task of the instance) and `action` (`approve`, `sign`, `reject`; a `reject` needs `comment`: a missing one is 422 `E_VALIDATION` at `/comment`, sub-code `note_required`, and it must pass `GenericTextGuard::check(conn, comment, casework.decision_text_min_chars)` (section 3.3), else 422 at `/comment` with `forbidden_term` or `text_too_short`).
2. Call `WorkflowEngine::act(ctx, task_id, TaskAction::{Approve | Sign | Reject}, if_match, idem, assertion)` (phase 04 `act`, envelope steps 1 to 12). The case-specific steps are these, in the order the engine runs them:
   - a. Eligibility: the task's holder is an active `case_acl` grant with `decide` on this case and, for an instance of `case_report_approval`, the holder also holds role `M` (DSL E.4: "MLRO from `case_acl`"; `Holders` rule 5 as extended in section 4.5 item 2, so the inbox of an A2 never shows the signature task of a report outcome); Cedar `task.approve`, `task.sign` or `task.reject` on the Task resource (phase 04 `56_workflow.cedar`, `resource.eligible`).
   - b. Four-eyes: the proposer (instance maker) and earlier deciders of the pass are excluded (phase 04 `excluded`); a refusal is 403 `E_SELF_APPROVAL`.
   - c. Step-up: the engine's own step 5 (phase 04 `act`) consumes the assertion with `resource_ref = "task:<id>"` and writes its id to `tasks.stepup_id`; this phase declares the action `case.decide` with class `decision` for `approve` and `sign` (section 4.5 item 14; Open doubts 3); 401 `E_STEPUP_REQUIRED` otherwise. The route does not consume a second assertion.
   - d. `CaseWorkflowSubject::check_before_decision` (extends phase 04 `WorkflowSubject`, see section 4.5): (i) `lock(tx, case_id)` (the case row is taken after the instance row: lock order of section 3.16); (ii) the case is `in_review` and the proposal of the instance is running (`ended_at IS NULL`); (iii) the case is not sealed; (iii-b) the instance is `restricted` (a restricted or suspicion case): the request comes from the analyst network and carries the purpose `assigned_work` or `investigation`, else 403 `E_FORBIDDEN` (sub-code `role_not_permitted`) or 422 `E_PURPOSE_REQUIRED`; this is a service check of the case subject, not a Cedar forbid on every restricted task, so the restricted instances of phases 15 and 21 keep their own rules; (iv) the act is not under a delegation (`acting_for` NULL), else 403 `E_FORBIDDEN` with sub-code `delegation_refused` (Open doubts 17); (v) `AuthorityMatrix::permits(actor.roles, "case.decision", case.priority_class)` (phase 09 section 3.6): a refusal is 403 `E_FORBIDDEN` with sub-code `role_not_permitted`; a permit with `second_review = true` is 409 `E_NOT_ALLOWED_IN_STATE` with sub-code `second_review_not_shipped` (DR-143); (vi) the view gate of section 3.9: a missing panel or evidence item is 422 `E_VALIDATION` with sub-codes `panel_not_opened` or `evidence_not_opened`; (vii) every cited evidence item is clean (sub-code `evidence_not_clean`).
   - e. `approve`: the instance completes and the terminal effect `record_decision` runs in the same transaction (step f). `reject`: the instance is rejected and `on_ended` runs (step h). `sign` behaves as `approve` for `case_report_approval` only when the act carries `via = "report"` (7.3); without it the sign is refused with 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `report_sign_required`.
   - f. `RecordDecisionEffect::run(tx, inst, params = {}, actor)`:
     1. Load the proposal by `workflow_instance_id` (`FOR UPDATE`); `ended_at` must be NULL.
     2. Set `ended_at = now()`, `end_kind = completed` (the proposal guard allows this once; this is the first write that the decision insert guard of section 1.2 requires).
     3. Decrypt `text_enc` (AAD = proposal id) and re-seal it under AAD = the new decision id: a sealed value is bound to its row id, so the bytes cannot be copied.
     4. `decision_id` = UUIDv7; insert `casework.decisions` with the proposal's fields, `decider_id = actor`, `proposer_id`, `decided_at = now()`, `second_review = FALSE`, `versions` = the workflow definition version, the workflow definition version id of the instance (phase 04 pin) and the policy pack version of the case (phase 09), `stepup_id` = the assertion id (the engine's step 5 wrote it to `tasks.stepup_id`), `ai_involvement = {"used": false}` (AI is P2), `supersedes` = `CaseDecisionRepository::latest_for_case` (NULL for the first decision; a reopened case's next decision names the one it replaces).
     5. `opened_views`: the decider's rows of `casework.opened_views` with `opened_at >= proposal.created_at`, copied as JSON into `decisions.opened_views`.
     6. Case: `outcome = type`, `status = open`, `workflow_version = definition version`; `set_status`.
     7. Contribution inheritance (step i).
     8. Tier: for `utr`, `str`, `tfr`, `casework.party_tiers` insert or raise the subject party to `suspicion` (`raised_by = actor`, `source_case_id = case_id`); audit `party.tier_raised` (DR-142).
     9. Chain `case.decided {decision_id, decision_type, reason_codes}`; audit `decision.recorded {decision_id, decision_type, reason_codes, proposal_id}`; outbox `case.decision {case_id, decision_id, decision_type, party_ids}` (the TR-15 source topic of DSL C.5 that phase 15 consumes; ids and codes only; added in the phase 19 review).
     10. For `utr`, `str`, `tfr`: `ReleaseForFilingEffect::run` (step g).
   - g. `ReleaseForFilingEffect::run`: chain `case.filing_released {decision_id}` and audit `case.filing_released`. It creates no report row; the report package and the filing record are phase 21 (DSL E.3 `release_for_filing`, narrowed here; Open doubts 15).
   - h. `on_ended(tx, inst, end)` for `rejected`, `withdrawn` and `failed` (`WorkflowSubject`, phase 04): `lock(tx, case_id)`; set the proposal `ended_at`, `end_kind` (`rejected`, `withdrawn`, `failed`); the case returns to `open` when it is `in_review`; chain `case.proposal_ended {proposal_id, end_kind}`; audit `case.proposal_ended`. A `completed` end is written by step f, not here.
   - i. Contribution inheritance (M-09, DR-138), in this order, on the attached scenario alerts of the case (`casework.case_alerts` with `source_kind = 'scenario'`): (1) each `alert_outcomes` entry of the proposal: `contribution_outcome = outcome`, `contribution_inherited = FALSE`; (2) each attached alert whose `contribution_outcome` is NULL or `not_assessed`: `duplicate` when its `state = 'duplicate'` (not inherited), otherwise the value of `casework.contribution_defaults` for the decision type, with `contribution_inherited = TRUE`; each change increments `alert.alerts.version` and writes `alert.contribution_set {outcome, inherited}` in `alert.alert_events` (phase 17 trail) and audit `alert.contribution_set`.
3. Audit of the act (phase 04 section 3.10) with the instance and task ids.
4. Response: `{instance_state, decision_id?}`; `decision_id` is present on `approve` and `sign`.

**`CaseWorkflowSubject` (phase 04 `WorkflowSubject`, `resource_kind = "case"`)**:
- `attribute_prefix = "case"`; `attributes`: `family` (text), `priority` (text), `tier` (text), `escalation_level` (integer), `outcome` (text, may be empty). No shipped condition reads them.
- `parties()`: none (`subject_party` is not used by the shipped definitions).
- `start_context(conn, case_id)`: the attributes above frozen; `classification = restricted` when the case tier is `restricted` or `suspicion`, otherwise `internal` (the definitions carry `classification_default = internal`; a subject may raise the classification, not lower it); `maker_id` = the proposer; `title` `{ar: "قرار بشأن ملف", en: "Case decision"}` and the step labels in the same neutral words (no party name, no typology).
- `visible_to(principal, case_id)`: the visibility predicate of section 3.10 for the case.
- `visibility_sql()`: `(i.resource_kind <> 'case' OR EXISTS (SELECT 1 FROM casework.cases c WHERE c.id = i.resource_id AND <case visibility predicate of section 3.10 over alias c, principal $1>))`, so the inbox and the counts of tasks use the same predicate as the case list.
- `party_sql`: none.

**Proposal reads.** `GET /cases/{id}/proposals` (`case.read`) returns the running proposal, or the last ended one, with `gate` (section 3.9), the caller's open task of the instance when the caller is eligible (`TaskRef`), and `instance_version` as ETag. A caller who is not eligible receives the proposal without the task; the sealed text is shown to the case team only.

**Decision reads.** `GET /cases/{id}/decisions` (`case.read`) returns the decisions with their versions and `ai_involvement`; the text is decrypted for the case team only; a suspicion-tier read writes an access event (`resource_type = decision`).

### 3.9 The view gate (`ViewGate`; M-21, DR-136)

`ViewGate::check(conn, proposal, decider, case) -> GateReport`, pure over three reads:
1. `required` = `casework.required_panels[case.priority_class]` (absent key -> no panel required).
2. `opened` = `OpenedViewRepository::since(conn, case.id, decider, proposal.created_at)`: the decider's rows of `casework.opened_views` opened at or after the proposal was created. A view from an earlier proposal or an earlier day does not count.
3. `missing_panels` = `required` minus the `panel` keys in `opened`; `missing_evidence` = `proposal.evidence_refs` minus the `evidence` keys in `opened`; `evidence_not_clean` = the refs whose object is not clean.
4. The decision refuses (section 3.8 step d.vi and d.vii) when any of the three lists is not empty; `GET /cases/{id}/proposals` returns the same report for the caller, so the W7 buttons stay disabled until the list is empty (DESIGN-SYSTEM 7.11).

Why a separate table: the access log of phase 02 (`audit.access_event`) is written by the audit writer and is not readable by `akr_app`; a gate that needs it would need a new privilege. `casework.opened_views` is written in the caller's own transaction after the access event (section 3.4), and it is the decision record's proof of what was opened (research file 28 section 6.1, "the log of which panels the decider opened").

### 3.10 Visibility and masking for the phase 18 surfaces (extends phase 17 section 3.10; DR-133, DR-142)

**Case rows** (phase 17 rules 1 to 5, with one clause added in rule 5): a suspicion-tier case is visible to M, to a principal in its team, and now also to a principal with an active row in `casework.case_acl` for the case:

```sql
-- added to the rule-5 disjunction of phase 17 section 3.10 (alias c = casework.cases, $1 = principal)
OR EXISTS (SELECT 1 FROM casework.case_acl a
            WHERE a.case_id = c.id AND a.principal_id = $1 AND a.revoked_at IS NULL)
```

The Cedar `assignees` attribute of `Case` and `Alert` gains the same members (the entity adapter resolves them at request time; the attribute keeps its name, so the phase 17 policies are unchanged). A test pins the SQL clause and the Cedar attribute over generated principals (section 6, check 6).

**Reporter identity** (M-68): a `casework.case_parties` row with `role = 'reporter'` is returned to M only, on every read (`GET /cases/{id}`, the timeline and the party page). Other roles do not receive the row or its count.

**Party rows** (W4, W5, and the party links of section 3.5): a party `p` is visible to a principal `P` when every rule holds:
1. `P` holds A1, A2 or M.
2. The tier is `casework.effective_tier(p)`.
3. `normal`: no further rule.
4. `restricted`: a purpose code `assigned_work` or `investigation` (phase 03 purpose codes), and `P` is A1, A2 or M.
5. `suspicion`: `P` is M, or `P` is in the team of an open case of `p`: an assignee of the case, a member of its `assignee_pool_id` (the pool subquery of phase 17 section 3.10 rule 5), or an active row of `casework.case_acl`; plus the analyst network, a purpose code, and step-up `record_open` on the read.

The masked answer for a party the caller may not see is 404 `E_NOT_FOUND` with the body of a missing id (phase 03 section 3.6). The party kind is the phase 15 `customer` masked kind (phase 15 section 3.6), whose tier now reads `casework.effective_tier` (phase 17 Open doubts 6); this phase registers no new masked kind.

**Lists.** Every list of this phase (notes, evidence, work items, links, decisions, shelved rows, party cases) is filtered by the case predicate above, or by the party predicate for party rows, before the page is cut; no total is returned.

### 3.11 Escalation (`CaseService::escalate`; M-38, DR-140)

Route `POST /cases/{id}/escalate` (`queue.work`; Cedar `case.escalate`; `If-Match`; `Idempotency-Key`; body `Escalate { to_level, note }`).
1. Cedar `case.escalate` (A1, A2, M). `to_level = 3` requires A2 or M: an A1 caller gets 403 `E_FORBIDDEN` (sub-code `role_not_permitted`).
2. `lock(tx, case_id)`; `If-Match` equals `version`; status `in_review` -> 409 `E_NOT_ALLOWED_IN_STATE` (`proposal_running`); `closed` -> 409 `E_CASE_SEALED`.
3. `to_level` must be `escalation_level + 1` (422 `E_VALIDATION` at `/to_level`, sub-code `not_next_level`).
4. `note` goes through `NoteService::post` (section 3.3) in this transaction; the note id is kept on the event.
5. `set_escalation_level(case, to_level)`.
6. `to_level = 3`: for every active holder of role `M` (phase 01 `admin.role_grants`, `revoked_at IS NULL`) without an active grant on the case, insert `casework.case_acl` with `access = decide`, `reason = mlro_desk`, `granted_by = NULL`, and chain `case.acl_granted` per person.
7. Chain `case.escalated {from, to, note_id}`; audit `case.escalated`; outbox `case.decision {case_id, decision_type: "escalate", party_ids}` (TR-15, DSL C.5; added in the phase 19 review); `version + 1`.

An escalation never closes a case and never changes the status. The levels are: 1 analyst, 2 senior analyst (A2), 3 MLRO desk (Open doubts 7).

### 3.12 Closure and seal (`CaseSealer`; M-38, M-61, DR-139)

Route `POST /cases/{id}/close` (`case.decide`; Cedar `case.close`; SU class `decision`; `If-Match`; `Idempotency-Key`; body `CloseCase { reason_code, comment }`).
1. `reason_code` is an active `closure` code (422 `E_VALIDATION` at `/reason_code`, sub-code `reason_code_inactive`).
2. `lock(tx, case_id)`; `If-Match` equals `version`. Status `in_review` -> 409 `E_NOT_ALLOWED_IN_STATE` (`proposal_running`); `closed` -> 409 `E_NOT_ALLOWED_IN_STATE` (`already_closed`).
3. `outcome` is set (409 sub-code `no_outcome`).
4. `WorkItemRepository::open_count(case) = 0` (409 sub-code `work_items_open`): every item is done or waived by a second person (section 3.6).
5. For `utr`, `str` and `tfr`: a `case.filing_released` event exists in the chain (409 sub-code `filing_not_released`; phase 21 replaces the check with the filing record, Open doubts 10).
6. `CaseChain::append_event(tx, case_id, "case.closed", {reason_code, outcome})`: the closed event is part of the chain the seal commits to.
7. Seal: `seal_hash = sha256(canonical({case_id, case_ref, last_seq, head_hash, outcome, reason_code, closed_at}))` with the phase 02 canonical encoder (`cv = 1`); `last_seq` and `head_hash` are the values after step 6; `closed_at` = the chain time of step 6.
8. Insert `casework.case_seals` (`seq_no` = `CaseSealRepository::count + 1`, the seal fields, `sealed_by = actor`, `sealed_at = now()`).
9. `set_closure(tx, id, closed_at, sealed_at, seal_hash, outcome)`; `status = closed`; `version + 1`.
10. Audit `case.closed {reason_code, outcome}` and `case.sealed {case_id, seq_no, seal_hash}`. The audit chain of phase 02 carries the seal as one of its entries, so the seal is in the external anchor as well (phase 17 section 3.7 anchors the case heads). `case.sealed` is not a chain event: it commits to the head.
11. Response: `SealView { seq_no, closed_at, last_seq, head_hash, seal_hash }`.

A closed case is read-only (section 3.1 invariant I7). A closure does not close the alerts; their contribution outcomes were set by the decision (section 3.8 step i).

### 3.13 Reopen (`CaseService::reopen`; M-40, DR-140)

Route `POST /cases/{id}/reopen` (`case.decide`; Cedar `case.reopen` or `case.reopen_late`; SU class `decision`; `If-Match`; `Idempotency-Key`; body `ReopenCase { reason_code }`).
1. `reason_code` is an active `closure` code (422 `E_VALIDATION`).
2. The action is chosen before Cedar: `case.reopen` when `closed_at + casework.reopen_window_days >= now()` (phase 17 setting, 30 days), otherwise `case.reopen_late`. Cedar permits `case.reopen` to A2 and M, and `case.reopen_late` to M only; a late reopen by A2 is 403 `E_FORBIDDEN`.
3. `lock(tx, case_id)`; `If-Match`; status `closed` required (409 `E_NOT_ALLOWED_IN_STATE`).
4. No other open case of the same party and family (the partial UNIQUE of phase 17): when one exists, 409 `E_NOT_ALLOWED_IN_STATE` with sub-code `newer_open_case`; the reviewer merges instead (phase 17 section 3.6).
5. `set_reopened(tx, id, at)`, with `at = now()`: `status = open`, `closed_at`, `sealed_at`, `seal_hash` and `outcome` cleared (the CHECKs of phase 17 and section 1.1 hold); `escalation_level` is kept (invariant I6). The earlier seals stay in `casework.case_seals`.
6. Chain `case.reopened {reason_code, within_window}`; audit `case.reopened`; `version + 1`.

**Automatic reopen (phase 17 Open doubt 4).** No automatic reopen is built. A new alert of a closed case opens a new case beside it (phase 17 section 3.5, `case.linked_prior`), and a person decides whether to reopen (DR-140).

### 3.14 Party page and transactions (`PartyService`, `PartyTransactionService`; W4, W5; DR-142)

**`GET /parties/{id}`** (`party.read`):
1. Cedar `party.read` on Party (`tier` from `casework.effective_tier`, `case_team` from section 3.10); the tier rule of section 3.10 answers 404 `E_NOT_FOUND` when refused.
2. Suspicion: step-up action `party.open` (class `record_open`, `resource_ref = "party:<id>"`; 401 otherwise); restricted: purpose code required (422 `E_PURPOSE_REQUIRED`).
3. Reads (`CanonPartyReader`, SELECT only): the current `canon.party` row (`recorded_to IS NULL`) for type, status, nationality, residence country and `pep_status`; the current `canon.party_name` rows (`superseded_at IS NULL`), `script = 'ar'` first; the current `canon.account_party_role` rows joined to `canon.account` (product, currency, branch, status, the role). No account number, no identifier, no mother's name, no registration or tax number is returned (Open doubts 18).
4. Rating: `RatingSource::levels(&[customer_id], today)` (phase 13 port, phase 15 implementation) for the party's current `canon.customer` row (`party_id`); the level and its CBI label only; a party with no customer row shows no rating. Drivers, overrides and history stay with `GET /parties/{id}/rating*` of phase 15 (`kyc.work`). Suspicion-derived factors are withheld by phase 15 (DR-103), so the level shown here is the level phase 15 stores.
5. Cases: the party's cases the caller may see (section 3.10), with `case_ref`, status, priority class and tier.
6. `data_as_of`: the canonical batch time of the party row (phase 11 freshness).
7. Access event `view` (resource `party`, tier, purpose).

**`GET /parties/{id}/transactions`** (`party.transactions.read`):
1. The same tier rules as step 1 to 2 above.
2. Filters: `from`, `to` (default: the last 90 days; the range is at most `casework.txn_range_max_days`, else 422 `E_VALIDATION` at `/to`), `direction` (`D` or `C`), `currency` (`ref.currencies`), `min_amount`, `max_amount`, `txn_type` (a canonical code). No free-text filter (the search of phase 14 is not reused here).
3. Accounts: the party's current account rows with role `owner`, `joint` or `signatory` (`canon.account_party_role`, `recorded_to IS NULL`).
4. Query: `canon.txn` WHERE `account_id = ANY($accounts)` AND `booking_date BETWEEN $from AND $to` (the partition key bounds the scan), ordered `booking_date DESC, id DESC`, keyset cursor on `(booking_date, id)`, page size `casework.txn_page_size` (the page size plus one row decides the next cursor).
5. Row: `txn_id`, `booking_date`, `value_date`, `amount`, `currency`, `dr_cr`, `txn_type`, `status`, the channel and branch names, `account_id`, `counterparty_id`. No narrative, no counterparty name (Open doubts 20).
6. Access event per page: `view`, resource `party`, tier and purpose; a request without a purpose on a restricted party is 422 `E_PURPOSE_REQUIRED`.
7. No totals and no counts (M-17).

### 3.15 The shelved-decision register (`ShelvedRegister`; M-04, DR-134)

`GET /shelved-decisions` (`case.decide`; Cedar `case.shelved.read` on System; A2 and M):
1. Filters: `from`, `to` (default 90 days), `reason_code` (a `decision` code), `decider_id`, cursor.
2. Rows: `casework.decisions` WHERE `type = 'no_report'` joined to `casework.cases` and filtered by the case predicate of section 3.10 (the register is post-filtered, so a suspicion-tier case is absent for a principal outside its team, and the count does not change).
3. Columns: `decision_id`, `case_ref`, `decided_at`, `reason_codes`, the decider's display name, the count of `evidence_refs` and of `alert_outcomes`. No text, no party, no transaction.
4. Access event per page with the most sensitive tier of the rows (`view`, resource `decision`).
5. The supervisor or QA sample of the register is phase 19 (`qa.review`); this phase provides the register and its read.

### 3.16 Lock order (phase 17 section 3.7 extended; DR-141)

1. The workflow instance row and its tasks (`WorkflowEngine::act`, `claim`, `release`, `withdraw`; phase 04 invariant I1).
2. The case row: `CaseRepository::lock(tx, case_id)`. Taken by `check_before_decision`, by `RecordDecisionEffect`, by `on_ended` and by every case route.
3. Below the case row, in this order: `casework.proposals`, `casework.decisions`, `casework.case_acl`, `casework.work_items`, `alert.alerts` rows (contribution, and the claim rows of phase 17 when a case-level action touches them), `casework.party_tiers`, `casework.case_events` (the chain, under the lock of step 2).

No path takes step 2 and then step 1. A case route never takes an instance lock: the proposal start (section 3.7 step 5) inserts a new instance and locks no existing one. The decision act takes step 1 and then step 2, which is the only order the decision path uses, and no case route waits for an instance. Phase 17 claims lock the alert row and never the case row; a claim that ever takes both must take the case row first (Open doubts 21).

### 3.17 Errors (`CaseError`, `thiserror`; API section 6 codes only)

| Variant | Problem code | Status | Note |
|-|-|-|-|
| `NotFound` | `E_NOT_FOUND` | 404 | also the masked answer of 3.10 |
| `Forbidden { sub }` | `E_FORBIDDEN` | 403 | Cedar denial on a row the caller may know; `sub` in `errors[]` |
| `SelfApproval` | `E_SELF_APPROVAL` | 403 | proposer, earlier decider, or the creator of a work item who waives it |
| `StepUpRequired` | `E_STEPUP_REQUIRED` | 401 | missing, spent or wrong-class assertion |
| `PurposeRequired` | `E_PURPOSE_REQUIRED` | 422 | restricted or suspicion read or write without `X-Access-Purpose` |
| `Sealed` | `E_CASE_SEALED` | 409 | closed and sealed: reopen first |
| `NotAllowed { sub }` | `E_NOT_ALLOWED_IN_STATE` | 409 | the state does not allow the command; `sub` in the body |
| `VersionConflict { current }` | `E_VERSION_CONFLICT` | 409 | `If-Match` mismatch; the current version in the body |
| `PreconditionRequired` | `E_PRECONDITION_REQUIRED` | 428 | versioned command without `If-Match` |
| `Validation { errors }` | `E_VALIDATION` | 422 | one entry per field: `pointer`, `code` (sub-code), `spans` for forbidden terms |
| `UploadRejected { code }` | `E_UPLOAD_REJECTED` | 422 | phase 02 `TypeNotAllowed`, `Rejected`, `Infected`, `TooLarge`, `Empty` |
| `HashMismatch` | `E_HASH_MISMATCH` | 500 | evidence failed its hash on serve |
| `Unavailable` | `E_UNAVAILABLE` | 503 | audit or evidence store unavailable; no content is sent |

The sub-codes of this phase are a closed list: `reason_code_inactive`, `reason_family_mismatch`, `note_required`, `evidence_not_clean`, `not_on_case`, `forbidden_term`, `text_too_short`, `panel_not_opened`, `evidence_not_opened`, `role_cannot_decide`, `grant_exists`, `assignee_on_case`, `assignee_not_on_case`, `link_exists`, `not_next_level`, `proposal_running`, `already_decided`, `already_closed`, `no_outcome`, `work_items_open`, `filing_not_released`, `newer_open_case`, `delegation_refused`, `role_not_permitted`, `second_review_not_shipped`, `unknown_party`, `unknown_account`, `unknown_txn`, `not_clean`. They ride in `errors[].code` or in `sub` of the 403 and 409 bodies; no top-level code is added (Open doubts 22).

### 3.18 Audit events and tipping-off review (M-65, M-68; one audit event per mutation)

Each chain kind is also the audit kind, and each audit kind of a mutation is written in the mutation's transaction. Payloads carry ids, codes and counts, never names, values or text.

| Event kind | On | Resource | Chain |
|-|-|-|-|
| `case.note_posted` | a note | case | yes |
| `case.acl_granted`, `case.acl_revoked` | access list change | case | yes |
| `case.evidence_added` | an evidence upload staged | case | yes |
| `case.view_opened` | an evidence item or panel opened (acknowledgement) | case | no (audit only) |
| `case.proposal_started`, `case.proposal_ended` | proposal created, ended without decision | case | yes |
| `case.decided`, `decision.recorded` | the decision recorded | case, decision | yes, and audit |
| `case.filing_released` | `release_for_filing` | case | yes |
| `case.escalated` | escalation | case | yes |
| `case.work_item_opened`, `case.work_item_completed`, `case.work_item_waived`, `case.work_item_overdue` | work item change | case | yes |
| `case.link_added` | entity link | case | yes |
| `case.closed` | closure | case | yes |
| `case.sealed` | seal written | case | no (commits to the head) |
| `case.reopened` | reopen | case | yes |
| `alert.contribution_set` | contribution outcome set or inherited | alert | alert trail (`alert.alert_events`) |
| `party.tier_raised` (phase 17 kind) | a tier raised by a report outcome | party id | no |

**Tipping-off review** (what a person outside the case's access list can learn):

| Surface | Outside the access list, the person learns |
|-|-|
| `GET /cases/{id}/notes`, `/evidence`, `/evidence/{id}/content`, `/work-items`, `/links`, `/proposals`, `/decisions`, `/access`, `POST` of each | 404 `E_NOT_FOUND` (the case is masked, phase 17 section 3.10); nothing else |
| `POST /cases/{id}/opened-views` | 404 for a case the caller cannot see; nothing about other people's views |
| Gate report (`GET /cases/{id}/proposals`) | for a team member: which panels and items remain; for others the route is 404 |
| `GET /shelved-decisions` | rows of cases the caller can see; the count is taken after the predicate |
| `GET /parties/{id}`, `/transactions` | 404 for a refused party; a restricted party needs a purpose code, and the answer names it only to a holder of the purpose |
| `GET /parties/{id}` case list | the cases the caller can see, nothing about the others |
| Entity links | a masked link shows its kind and no id, value, party or account |
| Reporter identity | nothing outside M (M-68) |
| Decision text, notes, evidence labels | nothing outside the team (sealed; the access event of each read is on record) |
| Escalation and closure | the chain kinds and ids; the note text only to the team |
| Notifications and exports | none in this phase (phases 21 and 22); outbox payloads carry ids only |
| Audit viewer (phase 24) | event kinds and ids; the seal hash; no text, no tier value beyond the source case id |

### 3.19 Cedar actions and the file `91_cases.cedar` (phase 03 bundle, range 91)

The bundle gets one new file, `91_cases.cedar` (range 90 is phase 17's `90_alerts.cedar`, which this phase does not edit). Its entities and actions are appended to `schema.cedarschema` in the phase 03 form, as phase 14 and phase 17 did; the bundle hash changes with the file (phase 03 section 3.1). Roles are `principal.roles` (phase 03; no Role entity exists).

Schema additions (namespace `Akr`):

```cedar
// appended to schema.cedarschema by phase 18. `Case` and `Alert` are phase 17's; `assignees` of Case gains the case access list
// (section 3.10). `Party` is the party page resource: `tier` is casework.effective_tier, `case_team` the team of its open cases.
entity Party = { tier: String, case_team: Set<User> };

action "case.note.read" appliesTo { principal: User, resource: Case, context: Ctx };
action "case.note.write" appliesTo { principal: User, resource: Case, context: Ctx };
action "case.evidence.read" appliesTo { principal: User, resource: Case, context: Ctx };
action "case.evidence.add" appliesTo { principal: User, resource: Case, context: Ctx };
action "case.propose" appliesTo { principal: User, resource: Case, context: Ctx };
action "case.decide" appliesTo { principal: User, resource: Case, context: Ctx };
action "case.escalate" appliesTo { principal: User, resource: Case, context: Ctx };
action "case.close" appliesTo { principal: User, resource: Case, context: Ctx };
action "case.reopen" appliesTo { principal: User, resource: Case, context: Ctx };
action "case.reopen_late" appliesTo { principal: User, resource: Case, context: Ctx };
action "case.access.read" appliesTo { principal: User, resource: Case, context: Ctx };
action "case.access.change" appliesTo { principal: User, resource: Case, context: Ctx };
action "case.work.read" appliesTo { principal: User, resource: Case, context: Ctx };
action "case.work.write" appliesTo { principal: User, resource: Case, context: Ctx };
action "case.work.waive" appliesTo { principal: User, resource: Case, context: Ctx };
action "case.link.read" appliesTo { principal: User, resource: Case, context: Ctx };
action "case.link.write" appliesTo { principal: User, resource: Case, context: Ctx };
action "case.shelved.read" appliesTo { principal: User, resource: System, context: Ctx };
action "party.read" appliesTo { principal: User, resource: Party, context: Ctx };
action "party.transactions.read" appliesTo { principal: User, resource: Party, context: Ctx };
```

The file `91_cases.cedar` (every permit and forbid written out; one role per `@role`):

```cedar
// 91_cases.cedar (phase 18). Scope gates are API.md section 2 (queue.work, case.decide); these policies are the fine gate.
// The approval of a case decision is `task.approve`, `task.sign` and `task.reject` (phase 04 56_workflow.cedar), with the
// eligibility the workflow computes from the case access list (`case_acl` with `decide`; role M for report outcomes). The
// network and purpose rule of a restricted case instance is a service check of CaseWorkflowSubject (section 3.8 step d).

@id("cases.work.a1") @role("A1")
permit (principal, action in [
        Akr::Action::"case.note.read", Akr::Action::"case.note.write",
        Akr::Action::"case.evidence.read", Akr::Action::"case.evidence.add",
        Akr::Action::"case.propose", Akr::Action::"case.escalate",
        Akr::Action::"case.access.read", Akr::Action::"case.work.read", Akr::Action::"case.work.write",
        Akr::Action::"case.link.read", Akr::Action::"case.link.write",
        Akr::Action::"party.read", Akr::Action::"party.transactions.read"], resource)
when { principal.roles.contains("A1") };

@id("cases.work.a2") @role("A2")
permit (principal, action in [
        Akr::Action::"case.note.read", Akr::Action::"case.note.write",
        Akr::Action::"case.evidence.read", Akr::Action::"case.evidence.add",
        Akr::Action::"case.propose", Akr::Action::"case.decide", Akr::Action::"case.escalate",
        Akr::Action::"case.close", Akr::Action::"case.reopen",
        Akr::Action::"case.access.read", Akr::Action::"case.access.change",
        Akr::Action::"case.work.read", Akr::Action::"case.work.write", Akr::Action::"case.work.waive",
        Akr::Action::"case.link.read", Akr::Action::"case.link.write",
        Akr::Action::"case.shelved.read",
        Akr::Action::"party.read", Akr::Action::"party.transactions.read"], resource)
when { principal.roles.contains("A2") };

// M decides and does not propose (case.propose is not in this list); a late reopen is M only.
@id("cases.work.m") @role("M")
permit (principal, action in [
        Akr::Action::"case.note.read", Akr::Action::"case.note.write",
        Akr::Action::"case.evidence.read", Akr::Action::"case.evidence.add",
        Akr::Action::"case.decide", Akr::Action::"case.escalate",
        Akr::Action::"case.close", Akr::Action::"case.reopen", Akr::Action::"case.reopen_late",
        Akr::Action::"case.access.read", Akr::Action::"case.access.change",
        Akr::Action::"case.work.read", Akr::Action::"case.work.write", Akr::Action::"case.work.waive",
        Akr::Action::"case.link.read", Akr::Action::"case.link.write",
        Akr::Action::"case.shelved.read",
        Akr::Action::"party.read", Akr::Action::"party.transactions.read"], resource)
when { principal.roles.contains("M") };

// Suspicion-tier case: the team (assignees, pool members and the access list, all in resource.assignees) and the MLRO only.
@id("cases.forbid.suspicion.case")
forbid (principal, action in [
        Akr::Action::"case.note.read", Akr::Action::"case.note.write",
        Akr::Action::"case.evidence.read", Akr::Action::"case.evidence.add",
        Akr::Action::"case.propose", Akr::Action::"case.decide", Akr::Action::"case.escalate",
        Akr::Action::"case.close", Akr::Action::"case.reopen", Akr::Action::"case.reopen_late",
        Akr::Action::"case.access.read", Akr::Action::"case.access.change",
        Akr::Action::"case.work.read", Akr::Action::"case.work.write", Akr::Action::"case.work.waive",
        Akr::Action::"case.link.read", Akr::Action::"case.link.write"], resource is Akr::Case)
when { resource.tier == "suspicion" && !(principal.roles.contains("M") || resource.assignees.contains(principal)) };

// Suspicion-tier cases: the analyst network only.
@id("cases.forbid.suspicion.network.case")
forbid (principal, action in [
        Akr::Action::"case.note.read", Akr::Action::"case.note.write",
        Akr::Action::"case.evidence.read", Akr::Action::"case.evidence.add",
        Akr::Action::"case.propose", Akr::Action::"case.decide", Akr::Action::"case.escalate",
        Akr::Action::"case.close", Akr::Action::"case.reopen", Akr::Action::"case.reopen_late",
        Akr::Action::"case.access.read", Akr::Action::"case.access.change",
        Akr::Action::"case.work.read", Akr::Action::"case.work.write", Akr::Action::"case.work.waive",
        Akr::Action::"case.link.read", Akr::Action::"case.link.write"], resource is Akr::Case)
when { resource.tier == "suspicion" && context.network != "analyst" };

// Restricted and suspicion cases: a purpose code of the assigned_work family or the investigation purpose (phase 03).
@id("cases.forbid.purpose.case")
forbid (principal, action in [
        Akr::Action::"case.note.read", Akr::Action::"case.note.write",
        Akr::Action::"case.evidence.read", Akr::Action::"case.evidence.add",
        Akr::Action::"case.propose", Akr::Action::"case.decide", Akr::Action::"case.escalate",
        Akr::Action::"case.close", Akr::Action::"case.reopen", Akr::Action::"case.reopen_late",
        Akr::Action::"case.access.read", Akr::Action::"case.access.change",
        Akr::Action::"case.work.read", Akr::Action::"case.work.write", Akr::Action::"case.work.waive",
        Akr::Action::"case.link.read", Akr::Action::"case.link.write"], resource is Akr::Case)
when { resource.tier != "normal" && !(context.purpose == "assigned_work" || context.purpose == "investigation") };

// Party rows: suspicion-tier parties are visible to the MLRO and to the team of an open case of the party (case_team), from
// the analyst network; restricted and suspicion parties need the purpose codes above.
@id("cases.forbid.suspicion.party")
forbid (principal, action in [Akr::Action::"party.read", Akr::Action::"party.transactions.read"], resource is Akr::Party)
when { resource.tier == "suspicion" && !(principal.roles.contains("M") || resource.case_team.contains(principal)) };

@id("cases.forbid.suspicion.network.party")
forbid (principal, action in [Akr::Action::"party.read", Akr::Action::"party.transactions.read"], resource is Akr::Party)
when { resource.tier == "suspicion" && context.network != "analyst" };

@id("cases.forbid.purpose.party")
forbid (principal, action in [Akr::Action::"party.read", Akr::Action::"party.transactions.read"], resource is Akr::Party)
when { resource.tier != "normal" && !(context.purpose == "assigned_work" || context.purpose == "investigation") };

```

Notes on the policies:
- Default deny applies to everything not permitted: a late reopen by A2 (`case.reopen_late`), a proposal by M (`case.propose`), and every action for IA, SEC, SS, KYC, CH, CHK, CFG, DS, MRO, ADM and QA. The IA and SEC chain read is `case.chain.read` of phase 17 (`90_alerts.cedar`), and the shelved register is `case.shelved.read`, granted to A2 and M only.
- The forbids are the phase 17 rules applied to the new actions: suspicion membership (`assignees`, which now includes the access list), the analyst network, and the purpose. The purpose rule uses the two codes of the `assigned_work` family and `investigation`, a stricter reading than phase 17's "a purpose code is present" for suspicion rows (DR-133).
- Four-eyes is not a Cedar condition on the case: the proposer and earlier deciders are excluded by the workflow (phase 04), and the database CHECK `decider_id <> proposer_id` is the second layer (section 1.1).
- Test: each permit and forbid has a fixture for every role of the matrix (section 6, check 4), and the visibility predicate of section 3.10 is compared with these clauses (section 6, check 6).

## 4. Infrastructure Updates

### 4.1 Job kinds (Postgres `jobs`; coalesce key in brackets)

| Kind | Binary | Payload | Concurrency, attempts | Schedule or trigger |
|-|-|-|-|-|
| `timer_work_item_due` | worker | timer row (owner `work_item`, kind `work_item_due`) [work item id] | 2, 3 | fired by the phase 00 timer sweep at `due_at`; scheduled at open (section 3.6); handler section 3.6 |
| `work_item_due_scan` | worker | `{}` [`work_item_due_scan`] | 1, 3 | `schedule.work_item_due_scan` (hourly at minute 5); backstop for open items whose timer did not fire: enqueues `timer_work_item_due` per overdue item with coalesce key `work_item_due:<id>`; the handler writes one overdue event per item (checked before writing) |

Every handler re-reads its owner first. Nothing in this phase is a `decision` job: decisions run in the request transaction through the workflow (akr-api), so a decision is never waiting in a queue. No new job prefix is used (`ingest_` is for akr-ingestd only).

### 4.2 Timers, outbox topics and the counts trigger

- **Timers** (phase 00 `TimerService`): kind `work_item_due`, owner kind `work_item`, owner id the work item id, fired at the instant `due_at` (absolute; the bank calendar was applied when `due_at` was computed, section 3.6). The kind is registered in the phase 00 timer kind registry by this phase.
- **Outbox topics**: one new topic, `case.decision {case_id, decision_id?, decision_type, party_ids}`, written by `RecordDecisionEffect` (section 3.8 step f.9) and by the escalation (section 3.11 step 7); it is the TR-15 source topic of DSL C.5, whose consumer is phase 15 (phase 15 section 4.1 says the producing phase registers it; added in the phase 19 review). `case.event` (phase 17, payload `{case_id, seq}`) is now also written by every chain append of this phase; its phase 17 handler (the counts trigger) is unchanged, and phases 19 and 22 subscribe to it later. Phase 17 section 4.2 is the contract.
- **Evidence scan**: the `evidence_scan` job of phase 02 runs for the uploads of section 3.4; this phase adds no scan logic.

### 4.3 AppState additions

- `akr-api`: `cases: Arc<CaseService>` (escalation, closure, seal, reopen), `decisions: Arc<DecisionService>` (propose, act, shelved register), `case_access: Arc<CaseAccessService>`, `notes: Arc<NoteService>`, `evidence: Arc<EvidenceService>` (uses the phase 02 `EvidenceStore` already in AppState), `work_items: Arc<WorkItemService>`, `links: Arc<EntityLinkService>`, `view_gate: Arc<ViewGate>`, `text_guard: Arc<GenericTextGuard>`, `parties: Arc<PartyService>`, `party_txns: Arc<PartyTransactionService>`, `shelved: Arc<ShelvedRegister>`. The phase 04 `subjects` registry gains `CaseWorkflowSubject`; the phase 04 `effects` registry gains `RecordDecisionEffect` and `ReleaseForFilingEffect`.
- `akr-worker`: `work_item_clock: Arc<WorkItemClockHandler>` (the timer handler and the backstop scan of section 4.1). The worker holds no decision service; the decision path is API-only.

### 4.4 Network zones, metrics, readiness, OpenAPI

- **Zones**: no new edge. The API uses the existing edges to PostgreSQL, the evidence store and OpenBao (phase 02 for `open`), and the audit writer (phase 02 for `AccessLog`). No binary gains a route to another service.
- **Metrics**: `akr_case_decisions_total{decision_type}`, `akr_case_gate_refusals_total{gate}` (`gate` one of `panel`, `evidence`, `matrix`, `text`, `step_up`, `delegation`), `akr_case_closed_total{outcome}`, `akr_case_reopened_total{within_window}`, `akr_work_items_overdue_total`. No metric carries a case id, a party, a principal, a tier or a text.
- **Readiness**: no new check.
- **OpenAPI**: tags `cases` (phase 17, extended), `case-records` (notes, evidence, opened views, links, work items, access), `decisions` (proposals, decisions, escalation, closure, reopen, shelved register), `parties` (party page and transactions). `make generate-api` regenerates the client in this change.

### 4.5 Existing entity, schema, route and code updates (extensions of earlier phases)

1. **Phase 04 `WorkflowSubject`**: extended with the method `check_before_decision(&self, tx: &mut Tx, inst: &WorkflowInstance, actor: &AuthContext, task: &Task) -> Result<(), WorkflowError>`, called at phase 04 `act` envelope step 6 (phase 04 section 3.4 names it for other subjects). `CaseWorkflowSubject` implements it (section 3.8 step d). `on_ended` and `visibility_sql` are used as phase 04 defines them.
2. **Phase 04 effects and validator**: `RecordDecisionEffect` (kind `record_decision`) and `ReleaseForFilingEffect` (kind `release_for_filing`) register with resource kind `case`. The phase 04 refusal of `case_acl` before phase 18 (E_ASSIGNEE_UNAVAILABLE, section 3.10 and the publish rule of section 3.4 step 6) is lifted for the `case` subject; `Holders::eligible` rule 5 becomes "an active `casework.case_acl` row with `access = decide` on the instance's case, and, when the instance's definition is `case_report_approval`, the principal holds role `M`" (DSL E.4; the inbox, the Task adapter and the engine read the same rule).
3. **Phase 04 shipped definitions**: the two documents of DSL E.4, `case_close_no_report` and `case_report_approval`, are seeded by this phase through `akr-ctl workflow seed` with `resource_kinds: ["case"]`, `classification_default: "internal"` (the subject raises it to `restricted` for restricted and suspicion cases, section 3.8), the deciding step's assignee `case_acl` with grant `decide`, and timeouts `remind` and `escalate` only (no automatic outcome). A bank change to them is a `cr_workflow` change request (DSL E.4).
4. **Phase 17 `AssignmentService`**: writes the access-list grants of section 3.2 in the assignment transaction, and on a reassignment revokes the previous assignee's grant of the reasons `assignment` and `second_review` (section 3.2).
5. **Phase 17 case read**: `GET /cases/{id}` (phase 17) gains the fields `escalation_level`, `outcome`, `sealed` (boolean), `open_proposal_id`, the `affordances` object (`propose`, `decide`, `close`, `reopen`, `escalate`, `grant_access`, from the caller's role and the state; the W3 buttons render only when the affordance is true, DESIGN-SYSTEM 7.11) and the reporter row of section 3.10 for M only. The route and its scope are unchanged.
6. **Phase 17 visibility predicate and entity adapter**: the rule-5 clause and the reporter rule of section 3.10; `Case.assignees` and `Alert.assignees` include the active access-list members (phase 17 section 3.17 comment updated).
7. **Phase 17 `CaseRepository`**: extended with the methods of section 2. No column of phase 17 changes except the two of section 1.1.
8. **Phase 14 decision service**: calls `ScreeningCaseLinker::link_screening` (phase 17 section 3.5; port defined in phase 17, implemented here) in its own transaction when a party screening alert is decided `true` or `potential` (phase 14 section 3.6 step 8 names the hook; the phase 14 file gets this sentence in gap pass 1). Counterparty and payment-party screening subjects keep no case shape (DR-147).
9. **Phase 02**: owner kind `case` is added to the evidence owner registry (the owner of the uploads of section 3.4); `R6` already has a `evidence.lock_days` entry (phase 02 settings). No new method: `stage`, `open`, `put_generated` and `PgAccessLog` are used as they are.
10. **Phase 03**: the bundle gains `91_cases.cedar` (section 3.19); `schema.cedarschema` gains the entity `Party` and the actions of section 3.19. `MASKED_KINDS` gains nothing: cases are the phase 17 kind `case`, and the party page is the phase 15 kind `customer` (section 3.10). The purpose codes are used as they are.
11. **Phase 15 masked kind `customer`** (phase 15 section 3.6): its tier read becomes `casework.effective_tier(party_id)` (phase 17 Open doubt 6; the gap item is listed for phases 13 to 16). Phase 15's `RatingSource` is read as it is for the party page.
12. **Phase 09**: `PolicyPackService::scan` with scope `reason_text` (section 3.3); `AuthorityMatrix::permits` with the decision type `case.decision` (section 3.8; the type is in the closed table of phase 09 section 3.6 with scope `case.decide`); the reason-code families `decision` and `closure` as they are.
13. **Phase 00**: the timer kinds `work_item_due` and `case_acl_expiry`, the job kinds of section 4.1 (with the two rows of 7.25) and the settings `schedule.work_item_due_scan` and `schedule.case_acl_expiry_scan`.
14. **Phase 01**: no change to the closed class list. This phase declares its `StepUpAction` constants (phase 01 section 3, the OpenAPI test): `case.decide` (`decision`; consumed by the phase 04 engine at the task act), `case.close` (`decision`), `case.reopen` (`decision`), `case.evidence.open` (`record_open`; evidence content of a suspicion case) and `party.open` (`record_open`; party page and transactions of a suspicion party).
15. **Phase 10**: none. `akr_app` reads canon with the existing SELECT grants (the SELECT grant block of phase 10).
16. **API.md section 3.12**: the nine owned rows are marked "phase 18" as the ownership sentence says, and the sixteen added rows of section 2 are added with the marker "(phase 18, added)". The route `GET /cases/{id}` gains the fields of item 5.
17. **DATA-MODEL section 12**: reconciled to migrations 0062 and 0063 in the same change (section 1 list). **Roadmap** row 18 gains the tables `casework.proposals`, `casework.case_seals` and `casework.opened_views`, and `casework.entity_links` is marked "case links (network edges are phase 19)".
18. **Migration chain**: 0062 and 0063 are phase 18's; phase 19 starts at 0064.

---

## 5. Frontend Impact

Four screens of the DESIGN-SYSTEM catalogue (section 8.1, lines 691 to 695), all P1: W3 `/cases/:id` (WORK), W4 `/parties/:id` (WORK, tabs), W5 `/parties/:id/transactions` (LIST) and W7 (FORM, a set of dialogs inside W3). Their entries go into `frontend/src/app/screens.ts` with the roles that see them: A1, A2 and M for W3, W4 and W5; A2 and M for the decision dialogs, the access dialog and the shelved register. The route-to-policy coverage test maps W3 and W4 to `case.read` and `party.read`, W5 to `party.transactions.read`, and W7 to `case.decide` for the decide dialog and `case.propose` for the propose dialog.

### 5.1 W3 Case workspace (`/cases/:id`, WORK; mockup 02; UX-06, UX-64, UX-65)

- **Header strip** (DESIGN-SYSTEM 8.3 W3): case reference, priority class (the phase 14 words for P0 and P3, the proposed P1 and P2 words of phase 17 Open doubts 8), clocks (age and the next due work item or touch clock), the tier badge (7.4), merged-alert count, status and escalation level, `data_as_of`.
- **Evidence column** (scrolling): the "why raised" reasons (`ReasonChip`s of phase 17 with their facts, never thresholds for branch roles); the evidence panels that expand in place: alert snapshot (`GET /alerts/{id}/snapshot`), party profile (`GET /parties/{id}`), transactions (summary, with the link to W5), case timeline (`GET /cases/{id}/timeline`), rating (the level from the party page), screening (phase 14 items linked to the case), evidence (list, upload, content). Expanding a panel posts `POST /cases/{id}/opened-views` once per expansion; the gate report of section 3.9 is refreshed after each post.
- **Notes**, **work items**, **links** and **access** (the team, with reasons from `GET /cases/{id}/access`) are sections of the evidence column, each with its own empty state.
- **Decision column** (sticky, 7.11; mirrors the end side in RTL): two groups with separate headings (UX-64, IR-30). **Actions**: add note, add evidence, open work item, link entity, escalate, grant access. **Decisions**: propose a decision, decide (approve, sign or reject), close, reopen. The buttons render only when `affordances` says so.
- **The CBI escalation checklist** (IR-32) sits beside the propose dialog, pre-filled for criteria 1, 3 and 6 where the system can compute them, and the answers are the analyst's.
- Four-eyes is explained at the point of action in the case's terms ("This decision needs approval by a second person who holds decide access to this case"), never as a blanket line (IR-34).
- Errors and states: loading (skeleton), empty per panel, the masked answer (the same "Not found or no access" text as X6), error, offline; a closed case shows its seal ("Sealed on ..., seal reference ...") and no action buttons except reopen for those allowed.

### 5.2 W4 Party page (`/parties/:id`, WORK tabs)

- Tabs: **Profile** (type, status, nationality, residence country, PEP status, names with the Arabic form first), **Accounts** (product, currency, branch, status, the caller's role; no account number), **Rating** (the CBI label and the level with the date of the stored rating; drivers are on the K1 screen of phase 15 for KYC), **Cases and alerts** (the cases and alerts the caller may see, with case reference, status, class and tier), **Transactions** (opens W5).
- A suspicion-tier party whose page the caller may not see answers the masked "Not found or no access" text, identical to a missing id (X6).
- The restricted tier shows a purpose banner naming the purpose the caller stated.

### 5.3 W5 Transaction explorer (`/parties/:id/transactions`, LIST)

- Filter bar: date range (default the last 90 days, at most 366 days), direction, currency, amount range, transaction type. No free-text search.
- Table: booking date, amount with its currency code (numerals by the user's setting, phase 01 preferences), direction ("Debit" and "Credit", Arabic words in the Arabic interface), transaction type, account label, channel, branch. No narrative column in this phase (Open doubts 20).
- "Load more" uses the keyset cursor; no total and no count; the page size is `casework.txn_page_size`.
- The purpose banner as in 5.2; a request that the server refuses for the purpose shows the refusal's words, not a retry loop.

### 5.4 W7 Decision dialogs (in W3; FORM)

- **Propose a decision** (`case.propose`): outcome choice of the four outcomes, each with its consequence line (no report: the shelved register; UTR: goAML and a watch after filing, phase 19; STR: the report builder, phase 21; TFR: the terrorist financing report, phase 21); reason codes (the active `DEC_` codes, at most five); the reasoning text, with the generic-text check: on blur the dialog calls `POST /cases/{id}/proposals?dry_run=true` and marks each refused span in the text, with the reason ("the phrase on its own is not a reason"; "too short"), never the term (UX-65, IR-31; Open doubts 6 for the live check); the CBI checklist (seven rows, yes, no, not applicable, a note for yes and not applicable); origin (kind and reference); typology from the bank's list; the alerts with their contribution outcome (duplicate shown read-only); evidence items (clean items only); actions; earlier reports (goAML references); subjects and transactions.
- **Decide** (`case.decide`, for the decider): the proposal in read-only form; the gate report as a checklist of the panels and items with the links that open them (a panel opening posts `opened-views`; opening an evidence item writes its own row through the content read, section 3.4 step 6); the authority line ("Your role may decide cases of this priority", from the matrix); the step-up prompt of DESIGN-SYSTEM 7.12 (class `decision`); buttons Approve, Sign (for report outcomes) and Reject (comment required). The buttons stay disabled until the gate report is empty.
- **Close** (`case.decide`): the closure reason (`CLS_` codes), the open work items (with the waiver action for a second person), and the statement that the case is sealed on closure and is reopened only by a person.
- **Reopen** (`case.decide`, closed cases): the reason; a late reopen (outside `casework.reopen_window_days`) is shown only to M, and an A2 sees why it is not offered.
- **Escalate** (`queue.work`): the level (2, or 3 for A2 and M), the note (required).
- **Access** (`case.decide`): person search limited to people with the role the access level needs; the access level; the reason (manual needs a note).
- Draft text of a decision or a note is not persisted for a suspicion-tier case (no local or session storage); for other cases the unsent text may be kept in the tab only.

### 5.5 Generated hooks (Orval names from the paths)

`useGetCasesId`, `useGetCasesIdTimeline`, `useGetCasesIdNotes`, `usePostCasesIdNotes`, `useGetCasesIdEvidence`, `usePostCasesIdEvidence`, `usePostCasesIdOpenedViews`, `useGetCasesIdProposals`, `usePostCasesIdProposals`, `useGetCasesIdDecisions`, `usePostCasesIdDecisions`, `usePostCasesIdEscalate`, `usePostCasesIdClose`, `usePostCasesIdReopen`, `useGetCasesIdAccess`, `usePostCasesIdAccess`, `useDeleteCasesIdAccessPrincipalId`, `useGetCasesIdWorkItems`, `usePostCasesIdWorkItems`, `usePostCasesIdWorkItemsWorkItemIdComplete`, `usePostCasesIdWorkItemsWorkItemIdWaive`, `useGetCasesIdLinks`, `usePostCasesIdLinks`, `useGetShelvedDecisions`, `useGetPartiesId`, `useGetPartiesIdTransactions`.

Hand-written (not Orval): `useEvidenceContent`, which fetches `GET /cases/{id}/evidence/{evidence_id}/content` with the session cookie and the purpose header, and offers the blob to the viewer (no URL is kept in the query cache); the step-up dialog of phase 01 is reused.

### 5.6 Browser and caching rules

- TanStack cache `gcTime: 0` for the case and party routes (DR-75 of phase 14): nothing of a suspicion-tier row stays after the page closes.
- No notes, decision text or evidence labels in local or session storage for suspicion-tier cases (5.4).
- axe on W3, W4 and W5 in Arabic and English, light and dark themes; keyboard only for every decision action; the focus returns to the trigger after each dialog.
- At 390 px width the decision column becomes a bottom sheet with the same two groups; the evidence column becomes a stack; a 16 px gutter; no horizontal page scroll (the wide table of W5 scrolls inside its container with a sticky first column).
- Numerals follow the user's numeral setting; amounts carry the currency code; dates use the product format (UX-07).

## 6. Verification

1. `make migrate`: `0062_casework_decisions_records.sql` and `0063_casework_decisions_guards_grants.sql` apply cleanly, and a second run changes nothing (every statement is idempotent; the two `ALTER ... ADD CONSTRAINT` blocks are guarded by `pg_constraint`).
2. `cargo check`: compile-time SQL passes for every query of sections 2 and 3, including the party reader and the transaction reader.
3. `cargo clippy -- -D warnings`.
4. `cargo test`, including these property and unit tests:
   - `CaseAggregate` invariants I1 to I8 of section 3.1 over at least 10,000 generated command sequences (proptest), each sequence followed by `CaseChain::verify`.
   - `CaseOutcome` and `DefinitionKey` pairing: `no_report` only with `case_close_no_report`; `utr`, `str`, `tfr` only with `case_report_approval`.
   - `GenericTextGuard` fixtures: "no suspicion" alone is refused; the same phrase inside a longer sentence is refused; a Latin-letter variant with diacritics and tatweel is refused after folding; "strategy" is not a hit of "str"; a text of 39 folded characters is refused and 40 is accepted; the refusal carries spans and never the term.
   - `ViewGate`: a view opened before `proposal.created_at` does not count; a view of another actor does not count; P2 and P3 need no panel; P0 needs all five panels of the default setting.
   - Contribution inheritance: explicit outcomes are kept with `inherited = FALSE`; `not_assessed` and NULL take the setting value with `inherited = TRUE`; a duplicate takes `duplicate`; a second run changes nothing.
   - Seal: `seal_hash` recomputed from the stored chain head equals the stored value (a golden vector in `audit/tests/vectors/`, shared with phase 02's canonical encoder); a reopen and a second closure keep two `case_seals` rows with `seq_no` 1 and 2.
   - Masking matrix of section 3.10 and 3.5: roles (A1, A2, M, SS, IA) by tier (normal, restricted, suspicion) by network (analyst, other, unknown) by purpose (none, `assigned_work`, `investigation`, `kyc_review`); the expected answer of each cell is generated from the rules, and the masked answer equals the missing-id answer byte for byte.
5. Curl suite `tests/curl/phase18_cases.sh` (users `a1@`, `a2@`, `a2b@`, `m@`, `ia@`, `ss@`, `ds@`, `ch@`; a fixture case of party P1 with two scenario alerts from `POST /_dev/detections` (phase 17), one with a suspicion-tier party P2 and team `a2`, and one clean test file from the phase 02 fixtures). Every row is one request with the stated answer:
   - `POST /cases/{id}/notes` as `a1` with `Idempotency-Key: k1` and body "Customer called the branch" → 201; the same key and body again → 201 or 200 with `x-idempotent-replay: true`, byte-identical; the same key with another body → 422 `E_IDEMPOTENCY_REUSE`; without `Idempotency-Key` → 400 `E_IDEMPOTENCY_REQUIRED`.
   - `POST /cases/{id}/notes` as `a1` with body "no suspicion" → 422 `E_VALIDATION` at `/body` with `errors[].code = forbidden_term` and a `spans` array; the response contains no term.
   - `POST /cases/{id}/notes` with `amends` naming a note of another case → 422 `E_VALIDATION` at `/amends`, `not_on_case`.
   - `POST /cases/{id}/proposals` as `a1` without `If-Match` → 428; with `If-Match: 0` → 409 `E_VERSION_CONFLICT` with the current version in the body.
   - `POST /cases/{id}/proposals` as `m` with a valid body and `If-Match` → 403 `E_FORBIDDEN` (M does not propose).
   - `POST /cases/{id}/proposals` as `a1` with `reason_codes: ["CLS_TEST"]` → 422 at `/reason_codes/0`, `reason_code_inactive`; with `text: "no suspicion"` → 422 at `/text`, `forbidden_term`; with `text: "ok"` → 422 at `/text`, `text_too_short`.
   - `POST /cases/{id}/proposals?dry_run=true` as `a1` with a clean text → 200 with the check results; `GET /cases/{id}/proposals` → 200 with `proposal: null` (no proposal row written).
   - `POST /cases/{id}/proposals` as `a1` with `evidence_refs` naming a pending evidence item → 422 at `/evidence_refs/0`, `evidence_not_clean`; after the scan is clean → 201 with `instance_version`; `GET /cases/{id}` as `a1` → `status: in_review`; a second proposal → 409 `E_NOT_ALLOWED_IN_STATE`, `proposal_running`.
   - On a second fixture case where `a2` proposed `no_report`: `POST /cases/{id}/decisions` as `a2` (the proposer, holding `decide` on the case) with `If-Match` the instance version and a step-up header → 403 `E_SELF_APPROVAL`.
   - On a third fixture case where `a1` proposed `str`: `GET /inbox` as `a2b` (on the access list with `decide`) does not list the signature task; `POST /cases/{id}/decisions` as `a2b` with `action: sign` → 403 (not eligible: report outcomes are signed by M, DSL E.4); as `m` after step-up and the gate → 200 with a `decision_id` and a `case.filing_released` event on the timeline.
   - `POST /cases/{id}/proposals` as `a1` on the first case after its decision (status `open`, `outcome` set) → 409 `E_NOT_ALLOWED_IN_STATE`, `already_decided`.
   - `POST /cases/{id}/decisions` as `a2b` (on the access list with `decide`) without `X-StepUp-Assertion` → 401 `E_STEPUP_REQUIRED`.
   - `POST /cases/{id}/decisions` as `a2b` with step-up and before any view is opened → 422 `E_VALIDATION` with `panel_not_opened` on each panel of the P0 setting and `evidence_not_opened` on the item.
   - `POST /cases/{id}/opened-views` as `a2b` for each panel and `GET /cases/{id}/evidence/{evidence_id}/content` → 200 bytes; `GET .../content` as `a1` for the same item → 200 (the item is on the case); then `POST /cases/{id}/decisions` as `a2b` with step-up, the instance version and `action: approve` → 200 with `instance_state: completed` and a `decision_id`; `GET /cases/{id}/decisions` as `a2b` shows `decider_id` = `a2b`, `proposer_id` = `a1`, and `ai_involvement.used = false`.
   - `POST /cases/{id}/decisions` as `a2b` again → 409 `E_NOT_ALLOWED_IN_STATE` (no running proposal).
   - Two parallel `POST /cases/{id}/decisions` on one proposal (same body, different idempotency keys) → one 200 and one 409; the decision table holds one row (`UNIQUE (proposal_id)`).
   - `POST /cases/{id}/access` as `m` with `reason: manual` and no `note` → 422 at `/note`; with a note → 201 and `case.acl_granted` on the timeline.
   - `DELETE /cases/{id}/access/{a2_principal}` as `m` for the current assignee → 409 `E_NOT_ALLOWED_IN_STATE`, `assignee_on_case`.
   - `POST /cases/{id}/escalate` as `a1` with `to_level: 3` → 403 `E_FORBIDDEN`; as `a2` with `to_level: 2` → 200, and a repeat → 422 at `/to_level`, `not_next_level`.
   - `POST /cases/{id}/close` as `a1` → 403 `E_SCOPE` (no `case.decide` scope); as `a2` with an open work item → 409 `E_NOT_ALLOWED_IN_STATE`, `work_items_open`; after the item is completed → 200 with `sealed_at` and `seal_hash` set; `GET /cases/{id}/chain` as `ia` → 200 with `result: ok`; `POST /cases/{id}/notes` as `a2` → 409 `E_CASE_SEALED`.
   - `POST /cases/{id}/reopen` as `m` with a `CLS_` code that is inactive → 422; with an active code and step-up → 200 `status: open`, the seal fields NULL, and `case_seals` keeps the first seal.
   - `POST /cases/{id}/reopen` as `a2` after `casework.reopen_window_days` has passed (test setting 0) → 403 `E_FORBIDDEN` (`case.reopen_late` is M only).
   - `GET /cases/{id}/notes` as `a1` on the suspicion case P2 (no team membership) → 404 `E_NOT_FOUND`, and the body equals that of `GET /cases/<unknown uuid>/notes`.
   - `GET /parties/{P2}` as `a1` → 404 with the body of an unknown id; as `m` → 200 after step-up `record_open`.
   - `GET /parties/{P1}/transactions` as `a1` without `X-Access-Purpose` on a restricted P1 → 422 `E_PURPOSE_REQUIRED`; with `investigation` → 200 with rows, no narrative field, a `next` cursor when more rows exist, and `from` more than 366 days before `to` → 422 at `/to`.
   - `GET /shelved-decisions` as `a2` → 200 with rows that have no `text` field; as `ds` → 403 `E_SCOPE`.
   - `GET /work` and `GET /queues` as `ss` → 403 `E_SCOPE` (no `queue.work`); `GET /parties/{P1}` as `ch` → 403 `E_SCOPE`.
   - `POST /cases/{id}/links` as `a1` with `entity_kind: external` and a value → 201; the value is absent from `GET /cases/{id}/links` for `a1` when P2 is the linked party of a masked link (masked kind, no id).
6. The visibility predicate of section 3.10 (with the rule-5 clause for `casework.case_acl`) and the Cedar policies of `91_cases.cedar` agree over generated principals (with the pool and access-list memberships), tiers, networks and purposes: the test fails on any row the two disagree on.
7. Swagger UI shows the 25 routes of section 2, the three routes of sections 7.11 to 7.13 and the extended `GET /cases/{id}`; `make generate-api` regenerates the client cleanly and the hooks of section 5.5 exist under those names.
8. The audit trail of the curl session contains the kinds of section 3.18 for each request (`case.note_posted`, `case.proposal_started`, `case.decided`, `decision.recorded`, `case.acl_granted`, `case.escalated`, `case.closed`, `case.sealed`, `case.reopened`, `case.view_opened`); `audit.access_event` holds `view` events for the evidence content and the party reads with their purpose codes.
9. Database tests (a role per layer): `akr_app` cannot UPDATE or DELETE `casework.notes`, `casework.decisions`, `casework.case_seals`, `casework.opened_views` or `casework.evidence_items` (the append-only trigger raises); an INSERT into `casework.decisions` while the proposal is not `completed` is refused by `decisions_insert_guard`; `decider_id = proposer_id` is refused by the CHECK; a second live `casework.proposals` row for a case is refused by `uq_proposals_running`; a `casework.case_acl` row cannot change its `access` (the guard raises).
10. Lock order test: a decision act and a concurrent close on the same case, each run 200 times on two connections, finish without a deadlock report; exactly one of the two commits.
11. Existing tests still pass, including phase 17's visibility tests (the extension adds one clause; the phase 17 fixtures without access-list rows are unchanged).
12. Phase 17 and phase 14 callers: a party screening alert decided `true` in phase 14 creates a case through `ScreeningCaseLinker` (the call of section 4.5 item 8), with `source_kind = screening` and the tier `suspicion`.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 4a, from tools/agent/gap1-run4a.md; lead-verified (tools/agent/gap1-run4a-verdict.md).

### 7.1 Watch start, closure gate, network hook and bulk reopen (phase 19)
Phase 19 extends this phase and owns the extension (its section 4.5 item 5):
- Section 3.8 step f (`RecordDecisionEffect`): after step 8 (the tier), for a `utr` decision,
  `WatchService::start_for_decision(tx, decision, case)` in the same transaction.
- Section 3.12 (`CaseService::close`): the closure gate of phase 19 section 3.3 (sub-code
  `uncorroborated_explanation`), and the network hook of phase 19 section 3.7 step 7 (a child and its parent are locked
  in ascending `id` order).
- Section 3.13: the new entry point `CaseService::reopen_under_batch(tx, case_id, reason_code, actor, finding_id)` for
  the bulk reopen of phase 19 section 3.9: the reopen rules of this section with the batch assertion of `POST
  /qa/bulk-reopen` in place of a second one (the route consumes the step-up once).

### 7.2 Work-item breach clock (phase 19)
From phase 19 on, `WorkItemService::open` (section 3.6) calls `BreachService::arm(tx, "work_item", id, now,
casework.work_item_due_hours)`, and the `timer_work_item_due` handler calls `BreachService::breach(tx, "work_item", id,
100)` with its overdue write. The stages and the alarm `work_item_breach` are phase 19 section 3.10. This closes Open
doubts 11.

### 7.3 Report checks on the case report approval (phase 21)
Phase 21 extends this phase and owns the extension (its section 4.5 items 1, 2, 4 and 5; DR-170):
- The sign step of `case_report_approval` uses step-up class `signing`, not `decision`. This closes Open doubts 3.
- `CaseWorkflowSubject::check_before_decision` (section 3.8 d), for an instance of `case_report_approval`, runs the
  report checks of phase 21 section 3.6 step 3 (the report exists, `status = validated`, `frozen_version =
  current_version`, no block in the latest run, and the act carries `via = "report"`).
- The decision route (`POST /cases/{id}/decisions`, action `sign` on `case_report_approval`) and the phase 04 generic
  task route for a `sign` task of `case_report_approval` or `report_correction` refuse with 409
  `E_NOT_ALLOWED_IN_STATE`, sub-code `report_sign_required`, unless `via = "report"`.
- `ReleaseForFilingEffect` (section 3.8 step g) takes the body of phase 21 section 3.7; the close gate of section 3.12
  step 5 is replaced by phase 21's acknowledged-filing check (`filing_not_acknowledged`), as Open doubts 10 says.

### 7.4 Break-glass clause in the case rows (phase 25)
From phase 25 on, the rule-5 clause of section 3.10 (case rows) gains the same purpose-keyed break-glass clause as
phase 17 section 3.10 (phase 17 section 7+), and the Cedar `Case.assignees` adapter takes the requester under purpose
`break_glass` only (phase 25 section 3.2). The masked 404 does not change.

### 7.5 AI involvement and the assistant summary (phase 28)
From phase 28 on, `RecordDecisionEffect` calls `AssistantInvolvement::for_case` once in the decision transaction
(phase 28 section 3.4) and writes its shape into `casework.decisions.ai_involvement`; the default `{"used": false}` is
unchanged. The `affordances` of `GET /cases/{id}` gain `assistant_summary` (computed on the server).

### 7.6 Lock extension on seal (phase 23, DR-193)
From phase 23 on, the seal transaction of section 3.12 enqueues `retention_lock_extend` with `{case_id}`. No route,
state or event of this phase changes. This closes Open doubts 14.

### 7.7 The case review clock (DR-384)
The clock `case.review` (phase 09 registry, days, bounds 30 to 365) needs no column: a case in `open` or `in_review` is
due at `casework.cases.opened_at` plus the clock's days in force; closed and merged cases have no due date. Phase 24's
clocks board computes it (phase 24 section 7+).

### 7.8 Lock order, sub-codes and party routes extended by phase 19
- Section 3.16: phase 19 section 3.15 adds rank 3 (`casework.rfis`, `rfi_questions`, `rfi_answers`,
  `authority_requests`, `internal_reports`, `watches`, `hibernations`, `hibernation_hits`, `alarms`, `qa_samples`) and
  rank 4 (`casework.case_acl`, `work_items`, `case_events`, `network_edges`, `network_snapshots`).
- Section 3.17: the 24 sub-codes of phase 19 section 3.11 join the closed list: `recipient_not_liaison`,
  `tier_forbids_rm`, `contact_not_allowed`, `template_unknown`, `purpose_not_in_pack`, `already_registered`,
  `watch_needs_utr`, `already_hibernating`, `hibernation_not_allowed`, `no_network`, `not_due`,
  `uncorroborated_explanation`, `no_eligible_reviewer`, `reviewer_is_closer`, `not_sample_reviewer`, `sample_closed`,
  `finding_criterion_passed`, `score_mismatch`, `not_bulk_case`, `no_linked_case`, `role_not_permitted`,
  `not_on_case`, `unknown_party`, `forbidden_term`.
- The party routes (`api::routes::parties`) gain phase 19's graph and network-case routes; `GET /cases/{id}` is
  unchanged.

### 7.9 The party read predicate
The party rule of section 3.10 (party rows) is exposed as `PartyReadPredicate::allows(principal, party_id, context) ->
bool`, implemented here and used by this phase's party routes and by phase 26 (deal reads take the customer's party
rule). It reads `casework.effective_tier(party_id)` (phase 17) and the team and access-list clauses of section 3.10.

Gap pass 1 (2026-10-10), run 5b (coverage of SP and K IDs), from tools/agent/gap1-run5b.md; lead-verified (tools/agent/gap1-run5b-verdict.md).

### 7.10 Time-boxed manual grants on suspicion-tier cases (SP-28, SP-26, DR-395)
The register of access requests is the set of manual grants in `casework.case_acl`; no new table. Section 1 carries the
column `expires_at`, the revoke CHECKs (a NULL `revoked_by` on a revoked row means the system expired the grant, as a
NULL `granted_by` means the system granted it), the guard that keeps `expires_at` fixed, the classification row and the
setting `casework.manual_grant_max_days` (governed, default 30).
1. `grant` (section 3.2), new step 3a: when the case's `tier` is `suspicion` and `reason = manual`, the caller holds role
   `M` (403 `E_FORBIDDEN` otherwise; the MLRO's approval is the grant and its note is the written reason) and the body
   carries `expires_at`, later than now and at most `casework.manual_grant_max_days` ahead (422 `E_VALIDATION` at
   `/expires_at`). On other tiers `expires_at` is optional with the same bound. Assignment, second-review and MLRO-desk
   grants carry none: they end with the assignment or the role.
2. Step 4: an active grant of the same principal whose `expires_at` has passed is first revoked by the system in the
   same transaction (step 3 of the handler below), so it does not block the new grant.
3. Step 6: a grant with `expires_at` schedules a phase 00 timer, kind `case_acl_expiry`, owner kind `case_acl`, owner id
   the grant id, `fire_at = expires_at`; a revocation cancels it.
4. Handler `timer_case_acl_expiry` (akr-worker): lock the case, re-read the grant; if it is still active and
   `expires_at <= now()`, set `revoked_at = now()` with `revoked_by` NULL; chain `case.acl_expired {principal_id}`; audit
   `case.acl_expired`; `version + 1`. A grant already revoked is left alone.
5. Job `case_acl_expiry_scan` (akr-worker, setting `schedule.case_acl_expiry_scan`, hourly at minute 15): the backstop
   for timers that did not fire; enqueues `timer_case_acl_expiry` per active grant with `expires_at <= now()`, coalesce
   key `case_acl_expiry:<grant id>`.
6. The rule-5 visibility clause (section 3.10) and `Holders` rule 5 also require `(a.expires_at IS NULL OR a.expires_at >
   now())`, so a grant past its time gives no access before the timer fires.
7. `list` returns `expires_at`. The audit table of section 3.18 gains `case.acl_expired` (access list change, case,
   chain yes). The counts for the CISO are phase 24's control report (from phase 24 on).
8. Tests: a manual grant on a suspicion case by A2 is 403; by M without `expires_at` is 422; a grant whose time passes
   is revoked by the timer with `revoked_by` NULL and the person gets 404 on the case; an UPDATE that changes
   `expires_at` fails in the guard.

Gap pass 1 (2026-10-10), run 6b (coverage of T IDs), from tools/agent/gap1-run6b.md; lead-verified (tools/agent/gap1-run6b-verdict.md).

### 7.11 Counterparty link decisions (T-29, DR-398)
The analyst confirms or rejects a counterparty's party link from inside a case; phase 13 section 7+ owns the table and
the resolver.

| Method | Path | Scope | Description |
|-|-|-|-|
| POST | `/api/v1/cases/{id}/counterparty-links` | `queue.work` | Confirm or reject the party of a counterparty on a linked transaction; `Idempotency-Key` (New; `case.link.write`) |

**Decide** (`POST /cases/{id}/counterparty-links`, body `{counterparty_id, party_id, decision: confirm | reject}`;
Cedar `case.link.write`, unchanged): `lock`; a closed case -> 409 `E_CASE_SEALED`. The counterparty must be the
`counterparty_id` of a transaction with an active `transaction` link on this case (else 422 at `/counterparty_id`,
`not_on_case`). The party must have a current row (else 422 at `/party_id`, `unknown_party`). In one transaction: close
the counterparty's open `feat.counterparty_links` row (`valid_to = now()`) and insert `analyst_confirmed` or
`analyst_rejected` with `decided_by`, both through phase 13 `CounterpartyLinkRepository::replace_open` (it sets `valid_from` of the new row to the instant that closes the old one); chain `case.counterparty_decided {counterparty_id, decision}`; audit
`counterparty.link_decided`. Response 200 with the new row. A replay with the same key returns the first response.

`GET /cases/{id}/links` adds, for each `transaction` link, the counterparty id and its open link row (method and
party), the party resolved through `EntityMask` as for a party link. No other route shows a counterparty link, so the
`name_key` suggestions reach the case team only.

Gap pass 1 (2026-10-10), run 7a (coverage of UX and W IDs, gap item (ab)), from tools/agent/gap1-run7a.md; lead-verified (tools/agent/gap1-run7a-verdict.md).

### 7.12 Global search (X2; UX-29; SP-27, RP-61; DR-400)
One route serves the header search and the command palette (DESIGN-SYSTEM X2, MODULES M12 P1 screens). It searches
parties and cases only. It never searches reports (RP-61), screening alerts or pair memory (phase 14 owns the name
search, `screening.work`).

| Method | Path | Scope | Description |
|-|-|-|-|
| GET | `/api/v1/search` | `queue.work` | Global search: `q` 2 to 80 characters, `limit` at most 10 per group (a larger value is 422 `E_VALIDATION` at `/limit`); groups `party` and `case` with `has_more`, no totals; purpose required (New; `party.read`, `case.read`) |

`SearchService::query(ctx, q, purpose, limit)`:
1. `X-Access-Purpose` holds a purpose code (else 422 `E_PURPOSE_REQUIRED`, phase 02).
2. `SearchKind::detect(q)` (pure): `case_ref` when `q` matches `^CS-[0-9]{4}-[0-9]{6}$`; `iban` when it has 23
   characters starting `IQ` and passes the mod-97 check; `card` when it has 13 to 19 digits and passes Luhn; `phone`
   when it normalises as phase 10 normalises contact values; otherwise `name`. A kind whose check fails is searched
   as `name`.
3. Parties: `iban` against `iban_hk` and `number_hk` of current `canon.account` rows (owner parties); `card` against
   `canon.card.pan_hk`; `phone` against `canon.party_contact.value_hk`; `name` through the `crates/names` key against
   current `canon.party_name.key`. Only keyed hashes and keys are compared. Each party passes Cedar `party.read` and
   the section 3.10 tier rule; a party the caller may not see is absent, so no count reveals it.
4. Cases: `case_ref` exact, or the cases of the matched parties, filtered by `services::visibility::build` (phase 17
   section 3.10), the predicate of the queues; Cedar `case.read`.
5. One access event `search_hit` with resource type `global_search`, `resource_id` = the id of the audit event
   `search.performed {kind, groups}`, the purpose code and no `subject_party` (the pattern of phase 14 section 7+), then
   `scope.record_hits(rows)` (phase 02) for the restricted and suspicion-tier rows shown. The query text is never
   logged.
6. Frontend: the X2 overlay opens from the header and on `/` (phase 17 section 7+ shortcut layer); results grouped by
   kind, Arabic name first.
7. Tests: each kind for each role of the visibility matrix; a suspicion party outside the access list is absent with the
   same body as no match; a card-like query that fails Luhn is searched as a name.

### 7.13 Entity timeline of the case subject (UX-07; DESIGN-SYSTEM 7.14 and W3; DR-400)

| Method | Path | Scope | Description |
|-|-|-|-|
| GET | `/api/v1/parties/{id}/timeline` | `queue.work` | Events of the party, newest first; `kinds` filter; keyset cursor on (occurred_at, id); the 404 of section 3.10 for a masked party (New; `party.read`) |

`PartyTimelineService::list(ctx, party_id, kinds, cursor)`:
1. The tier rule of section 3.14 steps 1 and 2 (404, step-up, purpose).
2. Sources, each through its own visibility: alerts of the party (phase 17 predicate), the party's cases and their
   decisions (section 3.10), rating changes (`kyc.ratings`, the level only, when the caller holds `rating.read`). From
   phase 19 on: RFIs, authority requests and QA events; from phase 20 on: operations tasks; from phase 22 on:
   observations. A row whose source the caller may not see is absent.
3. Each row: `kind`, `occurred_at`, a role word for the actor, one neutral summary sentence, a link to its screen, and
   the seasonal band of its date (`ref.calendar_days` kinds `ramadan_hours`, `eid`, `arbaeen`, with `hijri_label`).
4. Access event `view` (resource `party`) per page.
5. Frontend: the W3 evidence column shows the subject's timeline with the kind filter and the table alternative of
   DESIGN-SYSTEM 7.14. The W4 tab list is unchanged.

### 7.14 The open-case strip on K1 (UX-59; closes the phase 15 section 5 deferral)
K1 (`/parties/:id/risk`) shows one strip above the drivers: the party's open cases that the caller may see, from
`GET /parties/{id}` step 5 (reference, priority class, status), and from phase 19 on the party's UTR watches from
`GET /watches` with the `party_id` filter (phase 19 section 7+). A case the caller may not see is absent, with no
count. Empty state: "No open cases or watches for this customer".

Adversarial pass (2026-10-10), run 1 (four-eyes and self-approval), from tools/agent/adv-run1.md; lead-verified (tools/agent/adv-run1-verdict.md).

### 7.15 Reopen after a filing is M only (ADV-1-08, MEDIUM; DR-407)
A closed `utr`, `str` or `tfr` case always has `case.filing_released` in its chain (section 3.12 step 5). Section 3.13
step 2 therefore chooses `case.reopen_late` (M only) for such a case whatever the window: an A2 reopen is 403
`E_FORBIDDEN`. From phase 21 on, the same rule reads the filing record that replaces the chain check. The reopen
still clears `outcome`; the decision rows, the filing and the earlier seals stay.

### 7.16 No rework in the case definitions (ADV-1-10, LOW; DR-407)
Phase 04 rework is a reject mode (phase 04 section 7+). The case definitions of this phase use `mode: end`: a rejected
proposal ends the instance and the proposer proposes again. This closes Open doubts 16.

Adversarial pass (2026-10-10), run 4 (clocks and timers on closed, merged or reopened items), from tools/agent/adv-run4.md; lead-verified (tools/agent/adv-run4-verdict.md).

### 7.17 Grant expiry on a closed case (ADV-4-01, MEDIUM; DR-410)
Handler `timer_case_acl_expiry` (section 4.1): after the lock, a case with status `closed` gets the revocation
(`revoked_at = now()`, `revoked_by` NULL) and the audit `case.acl_expired` only: no chain event and no case `version`
bump, so the seal stays the hash of the chain at closure (DR-139). An open case is handled as written. Test: a grant
expiring the day after closure is revoked and the case's `last_seq` equals the seal's.

### 7.18 Merges and the work of this phase (ADV-4-06 HIGH, ADV-4-08 LOW; DR-410)
From this phase on, the merge validation of phase 17 section 3.6 also refuses (409 `E_NOT_ALLOWED_IN_STATE`) a source or
target that is `in_review` (sub-code `proposal_running`) or has an `outcome` (sub-code `already_decided`), and a source with an
open work item (`WorkItemRepository::open_count(source) > 0`, sub-code `work_items_open`, the test of the close in
section 3.12). A person completes or waives the items first. The timer handlers of this phase (`timer_work_item_due`,
`timer_case_acl_expiry`) do nothing on a `merged` case, and `timer_work_item_due` writes `case.work_item_overdue` only
when no such event exists for the item (the phase 17 `alert_touch` pattern), so a re-run writes once. Test: a merge of a
source with an open work item is 409 `work_items_open`.

Adversarial pass (2026-10-10), run 5 (existence masking leaks), from tools/agent/adv-run5.md; lead-verified (tools/agent/adv-run5-verdict.md).

### 7.19 A link target the caller may not see answers as a missing one (ADV-5-03, HIGH; DR-411)
Section 3.5 Add: after the existence read and before the `link_exists` check, the `EntityMask` rule of steps 1 to 3 runs on
the target's owner party with the caller's principal, network and purpose. A target the caller may not see is refused
exactly as a missing target: 422 `E_VALIDATION` with sub-code `unknown_party`, `unknown_account` or `unknown_txn`, the
same body, released no earlier than `authz.mask_floor_ms` after the request began, with a `deny` access event. No link is
written. An external link has no owner party and is unchanged. Test: adding a suspicion party outside the caller's
access and adding a random id give the same body; the case's link count is unchanged.

### 7.20 Party read order and the search page flag (ADV-5-02, ADV-5-06; LOW; DR-411)
1. Section 3.14 step 1 refuses on rules 1 and 5 of section 3.10 (party rows) with the masked 404. The purpose of rule 4
   (restricted) is answered by step 2 (422 `E_PURPOSE_REQUIRED`, DR-46): A1, A2 and M may know a restricted party
   exists, and the test of section 6 keeps its 422.
2. Section 7.12 step 3: the party predicate is in the SQL `WHERE` of each party query before `LIMIT limit + 1`, as
   section 3.10 "Lists" requires, so `has_more` comes from the filtered query. Test: with `limit` 1, two matches of
   which one is a hidden suspicion party give `has_more: false`, the same as one match.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.21 Sub-codes that join the closed list (V-18-01-a, V-18-06-a, LOW; DR-413)
Section 3.17, closed list: `no_running_proposal`, `report_sign_required` and `filing_not_acknowledged` join it, as 7.8 does for the sub-codes of phase 19. `no_running_proposal` is the sub-code of section 3.8 step d(ii) and of route step 1: the case is not `in_review`, or the proposal of the instance has ended (409 `E_NOT_ALLOWED_IN_STATE`). The other two are the sub-codes of 7.3.

### 7.22 Writers of the alert rows in the contribution step (V-18-02-a, LOW; DR-413)
Section 3.8 step i, for each attached alert it changes: `AlertRepository::lock` (after the case lock, the order of phase 17 section 7.9), `AlertRepository::update` with the version read under the lock, and `AlertRepository::append_event` (phase 17) with `NewAlertEvent { actor: the decider, kind: "alert.contribution_set", payload: {outcome, inherited} }`.

### 7.23 Steps of the opened-views route (V-18-03-a, MEDIUM; DR-413)
`POST /cases/{id}/opened-views`, body `{view_kind, view_key}`:
1. Cedar `case.read` on Case (the forbids of section 3.19 apply). A case the caller cannot see is 404 `E_NOT_FOUND` (section 3.10).
2. `view_kind` must be `panel` and `view_key` one of the six `PanelKey` values; else 422 `E_VALIDATION` at `/view_kind` or `/view_key`. The route does not take `evidence`: the row of an evidence item is written only by the content read (section 3.4 step 6), after its access event, so the gate of section 3.9 cannot be met without opening the item.
3. In the request transaction: `OpenedViewRepository::insert` with the caller as `actor_id` and `opened_at = now()`; audit `case.view_opened {view_kind, view_key}`; no chain event. Response 201.

### 7.24 The chain append is CaseRepository::append_event (V-18-04-a, LOW; DR-413)
Where the intro of section 3 names `CaseChain` as the writer, and where section 3.3 step 5 and section 3.12 step 6 write `CaseChain::append_event(tx, case_id, kind, payload)`, the call is `CaseRepository::append_event(tx, case, e)` of phase 17 section 2, with `e` a `NewCaseEvent` of that kind and payload. It runs under the case row lock and sets the sequence, the time and the head (phase 17 section 3.7). `CaseChain` is the verifier only (`CaseChain::verify`).

### 7.25 Jobs, timer kind and handler of the grant expiry (V-18-07-b, V-18-07-c, MEDIUM; DR-413)
Section 4.1 gains two rows: `timer_case_acl_expiry` | worker | timer row (owner `case_acl`, kind `case_acl_expiry`) [grant id] | 2, 3 | fired by the phase 00 timer sweep at `expires_at` (7.10 items 3 and 4); and `case_acl_expiry_scan` | worker | `{}` [`case_acl_expiry_scan`] | 1, 3 | `schedule.case_acl_expiry_scan` (hourly at minute 15), the backstop of 7.10 item 5.
Section 4.2: this phase also registers the timer kind `case_acl_expiry` (owner kind `case_acl`) in the phase 00 timer kind registry. Section 4.3, akr-worker, gains `case_acl_expiry: Arc<CaseAclExpiryHandler>` (the timer handler and the scan). Section 4.5 item 13 covers the kind, the two job kinds and the setting `schedule.case_acl_expiry_scan`.

### 7.26 Search response and its rows (V-18-09-a, LOW; DR-413)
7.12: the response of `GET /api/v1/search` is `SearchResponse { party: SearchGroup<PartyHit>, case: SearchGroup<CaseHit> }` (utoipa `ToSchema`), with `SearchGroup<T> { rows: Vec<T>, has_more: bool }` and no total. `PartyHit { party_id: Uuid, name: String }` holds the current name, Arabic first, read by `CanonPartyReader::current_names`. `CaseHit { case_id: Uuid, case_ref: String }`.

### 7.27 Deny event of a refused link add (V-18-12-b, LOW; DR-413)
7.19: the deny access event is written through `AccessLog::record` in its own transaction (phase 03 section 3.2 step 8), with `action = view` (the add reads the target to refuse it), `decision = deny`, `resource_type = customer`, `resource_id` and `subject_party` the owner party of the target, `tier` = `casework.effective_tier(owner party)` and the request's purpose code.

## Decisions

Numbered from DR-133 (the last number in `research.md` is DR-132). Each decision is one paragraph with its rationale.

**DR-133 (the case access list).** A person who is on a case without being its assignee, a pool member or the MLRO is recorded in `casework.case_acl` with an access level (`read`, `work`, `decide`), a closed reason and the note that gives a manual reason. The list is a visibility clause for suspicion-tier cases (phase 17 section 3.10, rule 5), part of the Cedar `assignees` attribute, and the eligibility of the `case_acl` assignee kind of phase 04. Grants and revocations are case events, so the list has a trail. This resolves the usability cost of phase 17 Open doubts 15 on the decision path: without the list an A2 could decide only on a case assigned to them. The purpose rule of the case policies is the two codes of the `assigned_work` family and `investigation`, a narrower reading than phase 17's "a purpose code is present"; the reason is that a suspicion-tier case is worked for one of those two purposes only.

**DR-134 (proposals and decisions).** A proposal is a row of its own, written with the workflow instance it starts, so the proposed content exists before anyone decides and the instance has a fixed subject. At most one proposal runs per case (a partial UNIQUE); a decision is written only by the terminal effect `record_decision` of phase 04, in the transaction that completes the approval, and a database guard refuses a decision whose proposal is not completed in that transaction. The decision row carries `decider_id <> proposer_id` as a CHECK, so the four-eyes rule of the workflow (phase 04 exclusion, Cedar `task.approve` through `resource.eligible`) has a second layer in the table. Decisions and notes are immutable; a correction after a reopen is a new decision with `supersedes`. The sealed text is re-sealed under the decision's own id when the decision is written, because phase 02 binds a sealed value to its row id.

**DR-135 (outcomes, closure and escalation are not one list).** The decision types are the four M-01 outcomes: `no_report`, `utr`, `str` and `tfr`. DATA-MODEL section 12 listed `close` and `escalate` as decision types; they are acts on the case, not decisions, because a decision needs a decider different from the proposer and closing or escalating does not. `no_report` runs the shipped definition `case_close_no_report` (one approval by A2 or M from the access list); the three report outcomes run `case_report_approval` (one signature by the MLRO from the access list, step-up, then `release_for_filing`). The closing sequence is a separate act with its own seal (DR-139). The DATA-MODEL row is aligned in the same change.

**DR-136 (the evidence-opened and panel gate).** A decision is refused unless the decider opened each cited evidence item and each panel that the case's priority class requires (M-21; default P0 with five panels and P1 with three; the setting is governed). The record of opens is `casework.opened_views`, written in the caller's transaction after the access event of phase 02 is recorded, and read by the gate for views opened at or after the proposal's creation. The access log itself (`audit.access_event`) is not used for the gate because `akr_app` has no read on it and phase 02 keeps it behind the audit writer; a new privilege for the gate would widen a table that holds every access in the bank. The gate's refusals use the closed sub-codes of section 3.17 inside `E_VALIDATION`, so no problem code is invented (API section 6).

**DR-137 (decision text, notes and the generic-text check).** The reasoning text of a proposal and the text of a note pass `GenericTextGuard`, which calls the phase 09 `ForbiddenWordScanner` through `PolicyPackService::scan` with scope `reason_text`, and then applies a minimum length to the folded decision text (40 characters, governed). The guard reports spans, never the term, as phase 09 requires. The guard does not add a scan route to the HTTP API (phase 09 forbids one): a dry run of the proposal route runs the same checks and writes nothing, and the W7 dialog calls it on blur. Capacity wording is a list the compliance owner keeps in the policy pack (M-27), not code. The rule "generic text is refused" is therefore the bank's list, and the guard enforces it everywhere the reason of an outcome is written.

**DR-138 (contribution outcomes, M-09).** A case decision writes a default contribution outcome on the alerts of the case, marked as inherited, and keeps every outcome a person set explicitly. The default for each outcome is a governed setting (`casework.contribution_defaults`), so the productivity figures that depend on the mapping can be changed by change request. A duplicate is never given an inherited outcome: its outcome is `duplicate`. Each write is an alert trail event and an audit event, so the per-scenario figures can be recomputed from the trail.

**DR-139 (seals, closure and reopen history).** Closure appends the closed event to the case chain first and seals the chain head, so the seal commits to the chain, the outcome, the reason and the closing time. The seal is one row of `casework.case_seals` (a sequence number per case), a `case.sealed` audit event, and the fields on `casework.cases`. The audit chain of phase 02 carries the seal as an entry, and phase 17's anchor carries the case heads, so the seal is in the external anchor. A reopen clears the fields on the case and keeps every earlier seal in `casework.case_seals`, so a reopened and re-closed case has two seals, both of which can be verified. A closed case accepts no write of the kinds listed in section 3.1.

**DR-140 (escalation levels, reopen and the automatic reopen).** A case has three escalation levels (analyst, senior analyst, MLRO desk), raised by one step at a time and never lowered. Level 3 gives `decide` on the access list to every active MLRO holder, because a case decision needs a decider on the list. A closed case is reopened by a person: the reason is a closure code and the step-up class is `decision`; the reopen of a case closed longer ago than `casework.reopen_window_days` is the MLRO's alone (`case.reopen_late`). No automatic reopen is built. This answers phase 17 Open doubts 4 for this phase: a new alert of a closed case opens a new case beside it, and a person decides whether the old one reopens.

**DR-141 (lock order for the decision path).** The decision act runs in the phase 04 engine, which locks the instance first; the case row is locked after it, inside the engine's transaction. No case route takes an instance lock, so the case row is never awaited by a holder of the instance lock who is itself waiting on a case route. The one order used by every path is therefore instance before case for decisions and case alone for everything else, and the lock-order test of section 6 runs the two races that matter (decision against closure, and decision against escalation). The alert rows of the contribution inheritance come after the case row, as the claim rows of phase 17 do not lock the case row.

**DR-142 (party tiers from report outcomes, party pages and entity links).** A decision of `utr`, `str` or `tfr` raises the subject party to `suspicion` in `casework.party_tiers`, as phase 17 raises a suspicion party when its family is suspicion. The party page and the transaction explorer read the tier through `casework.effective_tier`, and they register under the phase 15 `customer` kind, so no second masked kind exists for the same person. The page shows the rating level only (drivers stay with the phase 15 KYC screen, and phase 15 withholds suspicion-derived factors, DR-103). Entity links are masked at read time from the owner party's tier and carry no id when masked, so a link can never reveal a suspicion party to someone outside its team.

**DR-143 (the delegated-authority check at decision, M-20).** The decision act asks `AuthorityMatrix::permits(roles, "case.decision", priority)` of phase 09 for the decider, and refuses a decider whose role has no grant or whose grant does not reach the case's priority class. A grant with `second_review = true` is refused with a conflict until a two-step definition is shipped, because the shipped definitions have one deciding step and the bank has not chosen how a second review is recorded. The matrix is therefore enforced, and the bank's choice between a matrix and blanket second review is left open (Open doubts 2).

**DR-144 (work items and the waiver).** A work item is a to-do of the case with a kind from a closed list, an assignee or pool that holds access to the case, a due time from the bank calendar and a timer. A waiver needs a closure code and a second person: the creator cannot waive their own item (403 `E_SELF_APPROVAL`), which is the M-38 rule that actions are verified or waived with a second person. Closure of a case refuses while an item is open.

**DR-145 (entity links and network edges).** DATA-MODEL section 12 used `casework.entity_links` for network edges (shared contacts, devices, transfers, ownership and kinship, retention R3). This phase uses the name for the case's own entity links (parties, accounts, transactions and external identifiers, retention R6), because the analyst needs them in the case from the start. The network edges are phase 19's (the graph of W6) and move to a table of their own, `casework.network_edges`, which the lead confirms in DATA-MODEL and the roadmap (Open doubts 12). The reconciliation list of section 1 records the change.

**DR-146 (the route count and the API rows).** Twenty-five product routes: nine rows that API.md section 3.12 already gives to phase 18, and sixteen added rows (notes read, evidence read and content, opened views, the proposal read, closure, the access list read and changes, work items, links, the shelved register). Twenty-five routes is the boundary the template gives for XL, and the roadmap rates the phase XL, so the rating and the count agree. The routes that a person's decision makes (decisions, closure, reopen, access changes, waivers, the register) need `case.decide`, which is held by A2 and M only; A1 keeps the work routes.

**DR-147 (counterparty and payment-party screening subjects).** Phase 17 asked phase 18 to decide the case shape of counterparty and payment-party screening hits, which have no party case. A screening hit on a counterparty or a payment party stays in phase 14's queue and its decision, without a case. An analyst who needs the counterparty in a case links it as an external entity link, with its identifier sealed and its kind from the closed list. A party screening hit (of a customer or a party) opens or joins the party's case as phase 17 section 3.5 defines.

**Lead corrections (review 2026-10-09).** A report outcome (`utr`, `str`, `tfr`) could be signed by any A2 holding `decide`; DSL E.4 names the MLRO, so `Holders` rule 5 adds role `M` for `case_report_approval` and the inbox follows. A decided case went back to `open` with its outcome and could take a second proposal and a second decision; a proposal now needs `outcome` NULL (`already_decided`), and the decision after a reopen sets `supersedes`. The draft's Cedar forbids on every restricted Task would also have bound the KYC and report instances of phases 15 and 21; the network and purpose rule is now a check of `CaseWorkflowSubject`. The outcome enum is `CaseOutcome` (phase 09 already has `DecisionType`). Evidence uploads use the phase 02 kind `upload` and the real `NewEvidence` fields. The party page reads `RatingSource::levels` for the party's customer row (there is no `current`). Step-up uses declared `StepUpAction` constants (`case.decide`, `case.close`, `case.reopen`, `case.evidence.open`, `party.open`); the decision's assertion is consumed once, by the phase 04 engine. Transaction links and the transaction arrays carry the booking date, so a lookup reads one partition (Open doubts 19 resolved). Smaller fixes: `canon.party_name` is current by `superseded_at IS NULL`; `admin.role_grants` is phase 01's; the access record takes the purpose from the header; `earlier_reports` is `secret-str` in both tables; the self-approval curl row matched the fixture; the case-set network and purpose forbids of phase 17's `90_alerts.cedar` were written out.

## Open doubts (for gap pass 1 and the bank)

1. **Automatic reopen** (phase 17 Open doubts 4). Not built: reopen is a person's act (DR-140). The bank may want a reopen inside the window to happen by the system with a re-seal; that is a new route rule, decided by the MLRO.
2. **Second review.** The delegated-authority matrix (phase 09) has a `second_review` flag that this phase refuses until a two-step definition exists (DR-143). The MLRO chooses between a matrix with second review and blanket second review; a two-step variant of each shipped definition would then be a `cr_workflow` change.
3. **Class of the sign step.** The `sign` step of `case_report_approval` uses step-up class `decision`, as the brief asks. Phase 01 also has `signing` for signatures; the bank's security owner decides whether the signature of a report should use it.
4. **Panel gate.** The panel keys, the class defaults (P0 with five panels, P1 with three, P2 and P3 none) and the inclusion of transactions in P1 are proposals. The MLRO confirms them (M-21; DESIGN-SYSTEM 7.11 IR-22).
5. **Capacity wording.** The phase 09 seed of scope `reason_text` has "no suspicion" and the generic terms of DESIGN-SYSTEM section 11. Capacity wording ("no time", "backlog", "workload" and their Arabic forms) is not in it. The compliance owner adds the terms through the policy pack, with Arabic terms reviewed by a person (M-27; phase 09 Open doubts on Arabic terms).
6. **Live check while typing.** Phase 09 has no scan route. This phase offers a dry run of the proposal route, which the W7 dialog calls on blur. A check on every keystroke needs a scan route in phase 09, which the API owner decides.
7. **Escalation levels.** Level 2 is a senior analyst (A2) with no automatic grant. Level 3 grants `decide` to the MLRO holders active at the time; an MLRO appointed later needs a manual grant. The bank confirms both rules.
8. **Contribution defaults.** The mapping of the decision types to the contribution outcomes (`no_report` to `explained`; `utr`, `str` and `tfr` to `not_explained`) is a proposal. The productivity report of phase 21 depends on it (DR-138).
9. **Party tier from report outcomes.** A `utr`, `str` or `tfr` outcome raises the subject party to `suspicion`. The bank decides whether related parties of the case follow (phase 17 DR-123 gives the rule for the tier), and whether a `no_report` outcome lowers nothing (it does not in this phase).
10. **Filing gate for closure.** Closing a report outcome checks only for a `case.filing_released` event. Phase 21 replaces the check with the filing reference and its acknowledgement.
11. **Work item due time.** The default is 72 working hours. Work items have no breach action; the supervisor tasks and alarms are phase 19.
12. **Entity link names and retirement.** The DATA-MODEL table `entity_links` for network edges moves to phase 19 as `casework.network_edges` (DR-145). The route that retires a wrong entity link is not built; the columns and the guard exist for it.
13. **M-36 exit proposals.** Not carried here. The exit definition (`exit_decision`, DSL E.4) and the exit register belong to phase 15 (KYC), so phase 15 is the proposed owner; phase 17 had named phase 18. The lead decides.
14. **Lock extension at closure** (M-60). Phase 02 has no operation that extends an object lock at closure. The closing sequence does not extend it. Either phase 02 adds an operation (for example `evidence.extend_lock`) or phase 23 owns it.
15. **`release_for_filing`.** Phase 18 registers the effect with the narrowed body of section 3.8 (a chain event and an audit event). Phase 21 builds the package and the filing record; DSL E.3 describes the fuller effect.
16. **Rework.** The phase 04 task actions have no rework (the DSL refers to rework for cases). A rejected proposal ends the instance and the proposer proposes again. The lead decides whether phase 04 adds a rework action.
17. **Delegation at decision.** A decision under a delegation (`acting_for` set) is refused. Research file 28 allows the deputy of an absent MLRO to decide, recorded; the bank decides whether that deputy decides through a delegation or through a grant.
18. **Account numbers on the party page.** Not shown. `canon.account` holds only the sealed number and its keyed hash (phase 10), so a last-four display needs a masked column or an opened value. The bank decides the display rule.
19. **Transaction lookup by id.** Resolved in review: an entity link carries `txn_booking_date` with `txn_id` (the `canon.txn` key is `(id, booking_date)`), as phase 17's `case_alerts` carries `alert_created_at`. The `transactions` arrays of proposals and decisions hold `{txn_id, booking_date}` pairs for the same reason.
20. **Narrative in the explorer.** `canon.txn.narrative_norm` is personal and is not returned by W5 in this phase. The bank decides whether the explorer shows it, with the purpose and an access event per page.
21. **Lock order confirmation.** The decision path takes a case row inside the phase 04 transaction (DR-141). The lead confirms that the phase 04 `act` allows a terminal effect to lock a subject row, and that the phase 17 claim path locks no case row.
22. **Problem sub-codes.** The sub-codes of section 3.17 ride in `errors[].code` or in the `sub` field of a 403 or 409 body. The API owner adds them to API section 6 or to DSL section 7, or names one top-level code for each.
23. **Restricted tasks of other subjects.** The network and purpose rule for a restricted case instance is a service check in `CaseWorkflowSubject::check_before_decision` (section 3.8 step d.iii-b), because the Cedar Task entity has no resource kind and a forbid on every restricted task would also bind the KYC and report instances of phases 15 and 21. If the Task entity gains `resource_kind` (phase 03 or 04), the check can move to Cedar.
24. **Register visibility.** A2 and M see the shelved-decision register, rows of suspicion cases only where they are on the access list or are M. The bank may restrict the register to M.
25. **Large uploads under the case lock.** The `stage` transfer runs while the case row is locked (section 3.4). Uploads are limited by `evidence.max_bytes` (phase 02). A staged-then-attached flow would release the lock sooner; it is not built.
26. **Specifications of W3 to W7.** DESIGN-SYSTEM 8.3 has the W3 layout paragraph only. W4, W5 and the W7 dialogs have no full specification; the design owner writes them before build, as phase 17 requires for W1.
27. **Capture of windows and graphs** (IR-27: "add to evidence" with a window, a filter set or a graph definition). Not built; this phase accepts file uploads only.
28. **Adverse-media panel.** No source exists. The panel keys of section 1.1 have no adverse-media key until a source is contracted.
29. **Counterparty and payment-party screening** have no case shape (DR-147). The bank confirms that a counterparty hit does not open a case.
30. **AI involvement.** `ai_involvement` is `{"used": false}` in every decision until the P2 assistant (M15 gate).
31. **Priority words P1 and P2** are proposed in phase 17 Open doubts 8 and used here as they stand.
32. **Decision text minimum.** 40 folded characters (governed). The bank confirms the number.
33. **Notes and the note scope.** Notes and manual-grant reasons pass the `reason_text` scope of phase 09. A `note` scope in phase 09 would separate them; the bank and phase 09 decide.
34. **Distribution of reason codes** (M-27, reported to the second line). Phase 21.
35. **Decision type of phase 09.** `case.decision` (case.decide; A2 and M) is in the closed table of phase 09 section 3.6. The lead confirms the name and the holders.
36. **Chain events of phase 18 in the timeline.** The timeline of phase 17 (`GET /cases/{id}/timeline`) must show the kinds of section 3.18; the verification of its presentation belongs to the W3 specification (item 26).

Items not carried here (by M-ID) and their targets are listed in the requirements block of the header; the gap pass checks them.

## What This Phase Does NOT Touch

- No alert creation, no quarantine, no priority rule, no score, no claim, no queue, no counts and no triage (phase 17); the alert rows change only in `contribution_outcome` and `contribution_inherited` (section 1.1) and their version.
- No RFI, watch, hibernation, network (W6, `casework.network_edges`), QA sample, authority request, internal report or the graph route `GET /parties/{id}/graph` (phase 19).
- No report package, no goAML record, no filing, no filing acknowledgement and no report builder (phase 21); `release_for_filing` writes a chain event only.
- No freeze, no operations task, no CBS-facing action (phases 14, 20); `FreezeTrigger` and `RecordingFreezeTrigger` are not touched.
- No screening decision, screening state, list or pair memory (phase 14); the only phase 14 change is the decision service's call of `ScreeningCaseLinker` (section 4.5 item 8), which the phase 14 file receives in gap pass 1.
- No rating, override, segment, feature or feed write (phases 13 and 15); the party page reads the rating level only, and no suspicion-derived factor reaches a rating (DR-103).
- No write to `canon.*` (phase 10); no CBS write (the connection stays read-only, phase 05); no account number is returned.
- No new role, no new step-up class (the closed list of phase 01 is used as it is), no new problem code (API section 6 only), no new Valkey database, no new masked kind (the party page uses phase 15's `customer`; the case uses phase 17's `case`).
- No change to the workflow engine's invariants, routes or task states (phase 04); the two new effects and the subject hook are registered extensions.
- No change to the Cedar file of phase 17 (`90_alerts.cedar`) or to any other phase's Cedar file; `91_cases.cedar` is the only new file.
- No retention sweep and no legal hold (phase 23); the seal and the sealed text follow the R6 clock, the sweep is phase 23's.
- No notification, no outbox topic beyond `case.event` and `case.decision` (phase 22 subscribes later), no exports and no reports (phases 21 and 22).
- No AI assistant, no model score and no AI involvement other than the constant `{"used": false}` (P2).
- No change to the reason-code families, the policy pack document or the authority matrix document; this phase uses them (phase 09).
