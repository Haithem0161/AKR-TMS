# Phase 22: Branch Window & Notifications

**Goal:** The liaison officers of the branches answer the MLRO's information requests, record behavioural observations and confidential concerns, and sign the monthly liaison report, all through a light separate bundle that shows only three neutral statuses and never reveals a case, a suspicion or a reporter to branch management; and every notice of the system (in-app, SSE and the bank's e-mail relay) says only that work is waiting, is sent by one dispatcher whose recipients are computed from the same access rules as the screens, and never carries a name, a case subject or an AML reason.

**Dependencies:** Phase 00 (jobs, the outbox (the topic reserved for this phase is `notify.request`, section 4.2), timers and the timer kind registry, settings, cron schedules, `bank.timezone`, the maintenance sweep), Phase 01 (`StepUpService` with the closed class `signing`, `admin.users` with `locale`, `admin.stepup_assertions`, `admin.role_grants`, roles `L`, `RFI`, `M`, `A2`, `CH`, `IA`, `SEC`, `DS`, `ADM`, the `/me` routes of which the two `/me/notifications` routes move here, `serve.public.base_url`, the Kratos courier that sends nothing), Phase 02 (`EvidenceStore::stage`, the evidence owner registry, `admin.evidence_objects`), Phase 03 (`admin.branches` with `liaison_user_id` and `alternate_user_id`, `BranchService` (extended here, section 4.5), `CredentialResetService`, `MASKED_KINDS`, the branch range `B0` of the Cedar bundle), Phase 04 (`WorkflowEngine::act`, the terminal effect registry, `TerminalEffect`, `Holders`, `ChangeTargetRegistry`, `admin.change_requests`, `remind` and `escalate` timeouts), Phase 06 (the flag `branch.observations_live`), Phase 09 (`BusinessClock::add` with `BusinessUnit::WorkingDays`, `BankCalendarClock`, `PolicyPackService::scan` with the scopes `notification`, `branch_form`, `rfi` and `reason_text`, the closed `ClockRegistry`), Phase 10 (`canon.staff.branch_id`, `canon.account.branch_id`, `canon.branch.org_branch_id`, `canon.party`), Phase 16 (`DetectionPayload` and the observational scenarios `OBS`), Phase 17 (`AlertIntakeService::emit`, `casework.cases.tier`, `casework.effective_tier`, `casework.case_events`, `CountsHub` and `GET /events/counts`, the masking of suspicion tier), Phase 18 (`casework.case_acl`, the sealed-case rule), Phase 19 (`casework.rfis`, `casework.rfi_questions`, `casework.rfi_answers`, `RfiService::release` and `RfiService::record_answers` (its inner body `record_answers_in_tx`, section 4.5 item 2), `casework.internal_reports`, `InternalReportService::receive`, `casework.alarms`, `AlarmService::raise`, the outbox topics `rfi.overdue`, `rfi.reminder`, `internal_report.overdue`, `authority_request.overdue`, `watch.triggered`, `watch.expired`, `alarm.raised`), Phase 20 (`alarm.raised` only, no topic of its own), Phase 21 (`report.small_number_threshold`, the periodic kind `liaison` that counts `internal_reports` with channel `liaison_monthly`, screen R3 links)

**Macro-phase:** P1
**Complexity:** L (17 routes: 11 rows of API.md 3.1, 3.16 and 3.17 owned here, 6 rows added; 6 tables in 3 migrations (2 in `casework`, 4 in `admin`) and 2 altered CHECK constraints on `casework.alarms`; 1 terminal effect registered (`notify`) and 1 change target kind (`notification_template`); 6 job kinds and 1 timer kind; 1 outbox dispatcher for 14 topics, 11 of them from earlier phases)
**Binaries touched:** akr-api (routers `branch`, `notifications`, `liaison_reports`; `BranchWindowService`, `ObservationService`, `LiaisonReportService` and `NotificationService` in `AppState`) | akr-worker (jobs `branch_monthly_draft`, `observation_hybrid`, `notification_digest`, `notification_email`; the timer handler `timer_liaison_due`; the `NotificationDispatcher` as the outbox handler of section 3.8; the SMTP relay sender) | akr-ingestd and akr-assess unchanged

**Requirements carried:**
- Implemented here: M-43 (branch RFIs go to the liaison officer only and escalate to the MLRO desk, never to branch management; the window shows the caller's branch only), M-44 (the branch's side of the RFI desk: the released item, the four-eyes release stays in phase 19), M-45 (branch answers are stored as claims through `RfiService::record_answers_in_tx`, never self-corroborated), L-47 (explanations as claims with source, corroboration and expiry, here the branch input), UX-02 (the separate light bundle of about 300 KB compressed, second Vite entry), UX-66 (branch flows: items with neutral statuses, input form, monthly report auto-draft and signature; B6 is an Open doubt for its content), UX-67 (observation form with the circular's eight groups as ticks with notes, routed to the MLRO, written acknowledgment shown at once), S-22 (observation log, the eight groups, companion ID, written acknowledgment with the tipping-off reminder, confidential channel to the MLRO, no visibility for branch management), S-23 (hybrid alert of an observation plus a transaction pattern; the seven-criteria checklist of the circular is recorded on the observation, the checklist inside every case is Open doubts 4), M-53 (the missing liaison report alarm and the supervisor notification with its record), M-66 (the neutral status mapper of the branch window and the neutral default text of the notice to a staff member who filed an internal report), M-68 (generic notifications: the reporter's identity is in no notice; phase 19 keeps the register side)
- Partly implemented: M-25 of research file 28 (the monthly liaison report: drafted, signed and reviewed here; the MLRO's overdue view on the command centre is phase 16's, Open doubts 13), UX-68 (the forbidden-word scan is called for every branch and notice text; the list is phase 09's)
- Not carried here: S-24 (automated restructuring, splitting and repeated-pattern scenarios are phase 16's; this phase adds none, DR-212), the seven-criteria checklist inside every case (Open doubts 4), the CBI indicator map of the branch's observations to reason codes (Open doubts 2), M-27 and M-37 (phases 18 and 19)
- Gap pass 1 (coverage), implemented here: RP-13 (the internal chain: acknowledgement with the tipping-off reminder, decision notice, liaison report by the 10th), TS-21 (in-app and the bank SMTP relay only); SP-23 (branch-scoped window, the confidential channel to the MLRO desk, the liaison monthly reports); T-06 (the branch observation log with the eight indicator groups).
- Gap pass 1 (coverage), partly implemented: T-37 (branch RFIs routed to the liaison officer with escalation to the MLRO desk); VA-42 (the branch window with internal reports, status and the monthly report).

---

## 1. SQLx Migration Changes

Three migrations, `0073_casework_branch_window`, `0074_admin_notifications` and `0075_branch_guards_grants`, numbered after phase 23's `0071` and `0072` (the lead's range 0073 to 0075; no other phase uses these numbers). The schemas `casework` and `admin` exist (phases 00 and 17). Tables created here: `casework.observations`, `casework.liaison_monthly_reports`, `admin.notifications`, `admin.notification_templates`, `admin.notification_template_versions`, `admin.user_notification_prefs`. Two existing objects are changed: the CHECK constraints of `casework.alarms` (kinds and subject kinds, widened the way phase 21 widened them) and the phase 19 table `casework.internal_reports` is written, never altered.

**Reconciliation with DATA-MODEL section 16 and 17 (aligned in the same change, DR-201 and DR-202):**
- `confidential_reports` is dropped. A confidential report is an internal report of phase 19 with channel `branch_window` (body sealed, reporter visible to M only, always suspicion tier). Its columns map as: `body` to `internal_reports.body_enc`, `received_at` kept, `case_id` to `internal_reports.linked_case_id`, `reporter_id` kept.
- `branch_inputs` is dropped. A branch answer is an RFI answer claim of phase 19 (`casework.rfi_answers`, `source_kind = branch_officer`, `answered_by` the liaison). Its columns map as: `explanation_claim` to a `free_text` answer, `documents_on_file` to a `document` answer whose sealed value lists the staged evidence ids (the claim is written `corroboration = none`; the desk raises it, section 3.2), `visit_result` to a `value` answer, `submitted_by` and `submitted_at` to `answered_by` and `answered_at`.
- `observations`: DATA-MODEL's columns are kept with these changes: `acknowledged_at` becomes `reporter_ack_at` (it is the reporter's written acknowledgment, not the MLRO's), `indicator_groups` is a closed code list, `notes` becomes `notes_enc` (sealed), `party_ref jsonb` becomes `party_id` (the canonical party, one party per observation), `alert_id` becomes `hybrid_alert_id` (set only when the hybrid rule of section 3.3 fired). Added: `companion_id_enc`, `escalation_criteria`, `reporter_ack_code`, `internal_report_id` (the register entry the observation creates), `hybrid_checked_at`.
- `liaison_monthly_reports`: DATA-MODEL's columns are kept (`branch_id`, `period`, `draft` as jsonb, `signed_by`, `signed_at`, `stepup_id`, `reviewed_by`). Added: `period_end`, `due_on`, `status` (`draft`, `signed`, `reviewed`), `drafted_at`, `internal_report_id` (the register entry written at the first signature), `reviewed_at`, `returned_at`, `returned_by`, `return_comment_enc`, `review_comment_enc`, `suppressed_cells` (the small-number rule, phase 21's `report.small_number_threshold`), `timer_id`.
- `notifications`: DATA-MODEL's columns are kept (`user_id`, `template_code`, `params`, `read_at`, `created_at`). Added: `template_version` (the version rendered), `dedupe_key` (a replayed outbox event never creates a second row), `source_topic`, and the e-mail state columns (`email_state`, `email_attempts`, `email_sent_at`, `email_error_code`). The column `params` is a closed object of ids, counts, codes and dates, checked in code and by the length CHECK.
- `notification_templates`: DATA-MODEL's `channel` column is removed (in-app is always on and e-mail is a user preference, section 3.8). The body is bilingual jsonb and the placeholders are a closed list in `params`.
- `user_notification_prefs`: DATA-MODEL's `channel` and `mode` columns become one `email_mode` (`immediate` or `digest`). In-app cannot be switched off (DESIGN-SYSTEM X3, ARCHITECTURE section 15).
- Not in DATA-MODEL: `admin.notification_template_versions` (a template edit is versioned, section 3.8; DR-210).

### 1.1 `0073_casework_branch_window.sql`

```sql
-- migration 0073_casework_branch_window.sql
-- Phase 22. Branch records of the liaison officers (casework schema, phase 19 conventions): the observation log and the
-- monthly liaison report. The register entries of both are phase 19 internal reports (channel branch_window and
-- liaison_monthly). Alarm kinds widened the way phase 21 migration 0070 widened them; the phase 21 values are repeated.

ALTER TABLE casework.alarms DROP CONSTRAINT IF EXISTS alarms_kind_check;
ALTER TABLE casework.alarms ADD CONSTRAINT alarms_kind_check CHECK (kind IN (
    'touch_breach', 'work_item_breach', 'chain_broken', 'conflict_declared', 'flat_line', 'qa_critical',
    'post_order_debit', 'freeze_overdue', 'refrain_overdue', 'report_filing_overdue', 'report_rework_overdue',
    'liaison_report_overdue'));
ALTER TABLE casework.alarms DROP CONSTRAINT IF EXISTS alarms_subject_kind_check;
ALTER TABLE casework.alarms ADD CONSTRAINT alarms_subject_kind_check CHECK (subject_kind IN (
    'alert', 'work_item', 'case', 'scenario', 'qa_review', 'freeze', 'refrain', 'ops_task', 'report', 'report_task',
    'liaison_report'));

-- Observation log (S-22, UX-67). One row per observation; its register entry is an internal report with channel
-- branch_window (phase 19), so the MLRO desk, the acknowledgment deadline and the suspicion-linked review are phase 19's.
-- Branch readers never read this table: the branch window returns the neutral receipt only (section 3.1).
CREATE TABLE IF NOT EXISTS casework.observations (
    id                    UUID PRIMARY KEY,
    branch_id             UUID NOT NULL REFERENCES admin.branches(id) ON DELETE RESTRICT,
    reporter_id           UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,   -- the liaison or alternate who recorded it
    party_id              UUID,                                -- canonical party with an account at the branch (no foreign key); NULL: no customer named (a walk-in, a third party; described in notes_enc)
    indicator_groups      TEXT[] NOT NULL CHECK (cardinality(indicator_groups) BETWEEN 1 AND 8),
    escalation_criteria   TEXT[] NOT NULL DEFAULT '{}',
    companion_id_enc      BYTEA,                               -- the companion's identity number, sealed (personal, AAD = id)
    notes_enc             BYTEA NOT NULL,                      -- the officer's notes, sealed (personal, AAD = id)
    reporter_ack_code     TEXT NOT NULL CHECK (reporter_ack_code = 'OBS_ACK_V1'),   -- the tipping-off reminder version shown
    reporter_ack_at       TIMESTAMPTZ NOT NULL,
    internal_report_id    UUID NOT NULL UNIQUE REFERENCES casework.internal_reports(id) ON DELETE RESTRICT,
    hybrid_alert_id       UUID,                                -- alert.alerts id when the hybrid rule fired (no foreign key: partitioned table)
    hybrid_checked_at     TIMESTAMPTZ,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    version               INT NOT NULL DEFAULT 1,
    -- The circular's eight groups: seven indicator groups and the escalation checklist (research file 01 section A).
    CHECK (indicator_groups <@ ARRAY['avoid_cdd', 'in_transaction', 'cash_physical', 'instructed_by_other',
                                     'staff_pressure', 'after_transaction', 'tf_indicator', 'escalation_check']::text[]),
    CHECK (escalation_criteria <@ ARRAY['esc_1', 'esc_2', 'esc_3', 'esc_4', 'esc_5', 'esc_6', 'esc_7']::text[]),
    CHECK (('escalation_check' = ANY (indicator_groups)) = (cardinality(escalation_criteria) >= 1)),
    CHECK (hybrid_alert_id IS NULL OR hybrid_checked_at IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_observations_branch ON casework.observations (branch_id, created_at);
CREATE INDEX IF NOT EXISTS idx_observations_party  ON casework.observations (party_id, created_at);
CREATE INDEX IF NOT EXISTS idx_observations_hybrid ON casework.observations (created_at) WHERE hybrid_checked_at IS NULL;

-- Monthly liaison report (M-25 of research file 28, UX-66). One row per branch and period. Drafted at month start, signed by
-- the liaison with step-up `signing` by the due day, reviewed by the MLRO on R5. A returned report goes back to draft; its
-- first signature stays in the register (DR-203).
CREATE TABLE IF NOT EXISTS casework.liaison_monthly_reports (
    id                    UUID PRIMARY KEY,
    branch_id             UUID NOT NULL REFERENCES admin.branches(id) ON DELETE RESTRICT,
    period                TEXT NOT NULL CHECK (period ~ '^[0-9]{4}-(0[1-9]|1[0-2])$'),
    period_end            DATE NOT NULL,                       -- last calendar day of the period (bank calendar)
    due_on                DATE NOT NULL,                       -- first business day on or after the configured due day of the next month (section 3.6)
    status                TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'signed', 'reviewed')),
    draft                 JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(draft) = 'object'),   -- neutral counts only
    suppressed_cells      SMALLINT NOT NULL DEFAULT 0 CHECK (suppressed_cells >= 0),
    drafted_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    signed_by             UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    signed_at             TIMESTAMPTZ,
    stepup_id             UUID REFERENCES admin.stepup_assertions(id) ON DELETE RESTRICT,
    internal_report_id    UUID UNIQUE REFERENCES casework.internal_reports(id) ON DELETE RESTRICT,
    returned_by           UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    returned_at           TIMESTAMPTZ,
    return_comment_enc    BYTEA,                               -- the MLRO's comment on a return (personal, AAD = id)
    reviewed_by           UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    reviewed_at           TIMESTAMPTZ,
    review_comment_enc    BYTEA,                               -- the MLRO's comment on acceptance (personal, AAD = id)
    timer_id              UUID REFERENCES admin.timers(id) ON DELETE RESTRICT,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    version               INT NOT NULL DEFAULT 1,
    UNIQUE (branch_id, period),
    CHECK (period_end = (date_trunc('month', (period || '-01')::date) + interval '1 month - 1 day')::date),
    CHECK (due_on > period_end),
    CHECK ((signed_at IS NULL) = (signed_by IS NULL)),
    CHECK ((signed_at IS NULL) = (stepup_id IS NULL)),
    CHECK (status = 'draft' OR signed_at IS NOT NULL),
    CHECK (status <> 'draft' OR returned_at IS NOT NULL OR signed_at IS NULL),
    CHECK ((reviewed_at IS NULL) = (reviewed_by IS NULL)),
    CHECK ((status = 'reviewed') = (reviewed_at IS NOT NULL)),
    CHECK ((returned_at IS NULL) = (returned_by IS NULL)),
    CHECK (internal_report_id IS NULL OR signed_at IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_liaison_reports_open ON casework.liaison_monthly_reports (status, due_on)
    WHERE status <> 'reviewed';
CREATE INDEX IF NOT EXISTS idx_liaison_reports_branch ON casework.liaison_monthly_reports (branch_id, period);
```

Note: `internal_report_id` is set at the first signature and never cleared, so a returned report (status `draft`, `returned_at` set, `signed_at` kept) still holds it. The last CHECK above allows that.

### 1.2 `0074_admin_notifications.sql`

```sql
-- migration 0074_admin_notifications.sql
-- Phase 22. The notification centre: closed templates (bilingual, ids and counts only), their versions, the in-app and
-- e-mail notices of users, and the per-user e-mail preference. The seeds are the closed template codes of section 3.8.

CREATE TABLE IF NOT EXISTS admin.notification_templates (
    code          TEXT PRIMARY KEY CHECK (code ~ '^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$'),
    body          JSONB NOT NULL CHECK (body ? 'ar' AND body ? 'en'),   -- {"ar": "...", "en": "..."}; placeholders from params only
    params        TEXT[] NOT NULL DEFAULT '{}',
    version       INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (params <@ ARRAY['count', 'due_date', 'branch_code', 'alarm_kind']::text[])
);

-- Every version of a template, append-only (DR-210). The change request is the governance record (phase 04).
CREATE TABLE IF NOT EXISTS admin.notification_template_versions (
    id                  UUID PRIMARY KEY,
    code                TEXT NOT NULL REFERENCES admin.notification_templates(code) ON DELETE RESTRICT,
    version             INT NOT NULL CHECK (version >= 1),
    body                JSONB NOT NULL CHECK (body ? 'ar' AND body ? 'en'),
    params              TEXT[] NOT NULL DEFAULT '{}',
    change_request_id   UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    approved_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (code, version)
);

-- The notices. One row per recipient and source; dedupe_key makes a replayed outbox event a no-op.
CREATE TABLE IF NOT EXISTS admin.notifications (
    id                UUID PRIMARY KEY,
    user_id           UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    template_code     TEXT NOT NULL REFERENCES admin.notification_templates(code) ON DELETE RESTRICT,
    template_version  INT NOT NULL CHECK (template_version >= 1),
    params            JSONB NOT NULL DEFAULT '{}'::jsonb
                      CHECK (jsonb_typeof(params) = 'object' AND octet_length(params::text) <= 512),
    dedupe_key        TEXT NOT NULL UNIQUE CHECK (length(dedupe_key) BETWEEN 1 AND 200),
    source_topic      TEXT NOT NULL CHECK (source_topic ~ '^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$'),
    read_at           TIMESTAMPTZ,
    email_state       TEXT NOT NULL DEFAULT 'none' CHECK (email_state IN ('none', 'digest', 'queued', 'sent', 'failed')),
    email_attempts    INT NOT NULL DEFAULT 0 CHECK (email_attempts >= 0),
    email_sent_at     TIMESTAMPTZ,
    email_error_code  TEXT CHECK (email_error_code IN ('relay_unavailable', 'relay_refused', 'address_missing', 'timeout')),
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK ((email_state = 'sent') = (email_sent_at IS NOT NULL)),
    CHECK (read_at IS NULL OR read_at >= created_at)
);
CREATE INDEX IF NOT EXISTS idx_notifications_user   ON admin.notifications (user_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_notifications_unread ON admin.notifications (user_id) WHERE read_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_notifications_mail   ON admin.notifications (email_state, created_at)
    WHERE email_state IN ('digest', 'queued', 'failed');

-- Per-user e-mail preference. In-app notices are always on and have no row here.
CREATE TABLE IF NOT EXISTS admin.user_notification_prefs (
    user_id       UUID PRIMARY KEY REFERENCES admin.users(id) ON DELETE RESTRICT,
    email_mode    TEXT NOT NULL DEFAULT 'digest' CHECK (email_mode IN ('immediate', 'digest')),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    version       INT NOT NULL DEFAULT 1
);

-- Seeds: the closed template codes of section 3.8. Each body is run through the phase 09 scanner (scope notification) by the
-- test of section 6, check 4. Counts are written as labels ("Overdue tasks: {count}"), so no plural form is needed.
INSERT INTO admin.notification_templates (code, body, params) VALUES
 ('task.waiting', '{"ar": "لديك مهمة بانتظارك. افتح صندوق المهام.", "en": "You have a task waiting. Open your inbox."}', '{}'),
 ('task.overdue', '{"ar": "المهام المتأخرة: {count}. افتح صندوق المهام.", "en": "Overdue tasks: {count}. Open your inbox."}', '{count}'),
 ('case.updated', '{"ar": "هناك تحديث في عمل تتابعه. افتح صندوق المهام.", "en": "A case you work on has an update. Open your inbox."}', '{}'),
 ('branch.item_waiting', '{"ar": "طلبات بانتظار فرعك: {count}.", "en": "Requests waiting for your branch: {count}."}', '{count}'),
 ('branch.monthly_due', '{"ar": "موعد التقرير الشهري لضابط الارتباط هو {due_date}.", "en": "The monthly liaison report is due on {due_date}."}', '{due_date}'),
 ('branch.monthly_returned', '{"ar": "أُعيد التقرير الشهري للتصحيح. افتح التقرير.", "en": "The monthly report was returned for correction. Open the report."}', '{}'),
 ('desk.internal_report_waiting', '{"ar": "تقارير بانتظار الإقرار: {count}.", "en": "Reports waiting to be acknowledged: {count}."}', '{count}'),
 ('desk.deadline_near', '{"ar": "مواعيد قريبة: {count}. افتح صندوق المهام.", "en": "Deadlines near: {count}. Open your inbox."}', '{count}'),
 ('desk.approval_waiting', '{"ar": "موافقات بانتظارك: {count}.", "en": "Approvals waiting: {count}."}', '{count}'),
 ('supervisor.alarm', '{"ar": "هناك إنذار يحتاج إلى متابعة. النوع: {alarm_kind}.", "en": "An alarm needs attention. Kind: {alarm_kind}."}', '{alarm_kind}'),
 ('liaison.report_received', '{"ar": "تقرير شهري لضابط الارتباط جاهز للمراجعة. الفرع: {branch_code}.", "en": "A liaison monthly report is ready for review. Branch: {branch_code}."}', '{branch_code}'),
 ('liaison.report_overdue', '{"ar": "تقرير شهري لضابط الارتباط متأخر. الفرع: {branch_code}.", "en": "A liaison monthly report is overdue. Branch: {branch_code}."}', '{branch_code}'),
 ('staff.report_decision', '{"ar": "تم فحص بلاغك وتسجيل القرار. واجب السرية ما زال قائماً.", "en": "Your report was examined and the decision recorded. The confidentiality duty continues."}', '{}'),
 ('security.reset_pending', '{"ar": "إعادة تعيين بيانات دخول بانتظار التأكيد: {count}.", "en": "Credential resets waiting for confirmation: {count}."}', '{count}'),
 ('data.drift_open', '{"ar": "تغييرات بيانات بانتظار المراجعة: {count}.", "en": "Data changes waiting for review: {count}."}', '{count}')
ON CONFLICT (code) DO NOTHING;
```

The seeds need `admin.change_requests` (phase 04) and `admin.stepup_assertions` (phase 01), both created before this migration. The CHECK on `notifications.email_error_code` lists the four codes that the relay sender can write (section 3.8); no server text is ever stored.

### 1.3 `0075_branch_guards_grants.sql`

```sql
-- migration 0075_branch_guards_grants.sql
-- Phase 22. Guards that the application cannot bypass, grants per write path, settings, classification and retention.
-- Idempotent: triggers and functions are dropped before they are created.

-- Observations: identity, the record of what was observed and the register link are written once; only the hybrid
-- columns move (section 3.3).
CREATE OR REPLACE FUNCTION casework.observations_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN RAISE EXCEPTION 'observations are never deleted'; END IF;
    IF NEW.id <> OLD.id OR NEW.branch_id <> OLD.branch_id OR NEW.reporter_id <> OLD.reporter_id
       OR NEW.party_id IS DISTINCT FROM OLD.party_id OR NEW.indicator_groups <> OLD.indicator_groups
       OR NEW.escalation_criteria <> OLD.escalation_criteria
       OR NEW.notes_enc IS DISTINCT FROM OLD.notes_enc OR NEW.companion_id_enc IS DISTINCT FROM OLD.companion_id_enc
       OR NEW.reporter_ack_at <> OLD.reporter_ack_at OR NEW.internal_report_id <> OLD.internal_report_id THEN
        RAISE EXCEPTION 'an observation is written once';
    END IF;
    IF OLD.hybrid_alert_id IS NOT NULL AND NEW.hybrid_alert_id IS DISTINCT FROM OLD.hybrid_alert_id THEN
        RAISE EXCEPTION 'a hybrid alert link is written once';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_observations_guard ON casework.observations;
CREATE TRIGGER trg_observations_guard BEFORE UPDATE OR DELETE ON casework.observations
    FOR EACH ROW EXECUTE FUNCTION casework.observations_guard();

-- Monthly reports: the period and the due day are fixed; a signature is written once and is cleared only by a return;
-- a reviewed report is final.
CREATE OR REPLACE FUNCTION casework.liaison_reports_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN RAISE EXCEPTION 'liaison monthly reports are never deleted'; END IF;
    IF NEW.id <> OLD.id OR NEW.branch_id <> OLD.branch_id OR NEW.period <> OLD.period
       OR NEW.period_end <> OLD.period_end OR NEW.due_on <> OLD.due_on THEN
        RAISE EXCEPTION 'the identity and the due date of a liaison report do not change';
    END IF;
    IF OLD.status = 'reviewed' THEN RAISE EXCEPTION 'a reviewed liaison report is final'; END IF;
    IF OLD.signed_at IS NOT NULL AND OLD.status <> 'draft'
       AND (NEW.signed_at IS DISTINCT FROM OLD.signed_at OR NEW.signed_by IS DISTINCT FROM OLD.signed_by
            OR NEW.stepup_id IS DISTINCT FROM OLD.stepup_id) THEN
        RAISE EXCEPTION 'a signature is written once';
    END IF;
    IF OLD.internal_report_id IS NOT NULL AND NEW.internal_report_id IS DISTINCT FROM OLD.internal_report_id THEN
        RAISE EXCEPTION 'the register entry of a liaison report is written once';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_liaison_reports_guard ON casework.liaison_monthly_reports;
CREATE TRIGGER trg_liaison_reports_guard BEFORE UPDATE OR DELETE ON casework.liaison_monthly_reports
    FOR EACH ROW EXECUTE FUNCTION casework.liaison_reports_guard();

-- Notices: the text, the params and the source are fixed; only the read mark and the e-mail state move.
CREATE OR REPLACE FUNCTION admin.notifications_guard() RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE days INT;
BEGIN
    IF TG_OP = 'DELETE' THEN
        -- Deleted only by the R12 sweep of the maintenance job (section 4.1), and only past notify.retention_days.
        SELECT (s.value #>> '{}')::int INTO days FROM admin.settings s WHERE s.key = 'notify.retention_days';
        IF OLD.created_at > now() - make_interval(days => COALESCE(days, 90)) THEN
            RAISE EXCEPTION 'a notice is deleted only after notify.retention_days';
        END IF;
        RETURN OLD;
    END IF;
    IF NEW.id <> OLD.id OR NEW.user_id <> OLD.user_id OR NEW.template_code <> OLD.template_code
       OR NEW.template_version <> OLD.template_version OR NEW.params <> OLD.params
       OR NEW.dedupe_key <> OLD.dedupe_key OR NEW.source_topic <> OLD.source_topic OR NEW.created_at <> OLD.created_at THEN
        RAISE EXCEPTION 'a notice does not change its text, recipient or source';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_notifications_guard ON admin.notifications;
CREATE TRIGGER trg_notifications_guard BEFORE UPDATE OR DELETE ON admin.notifications
    FOR EACH ROW EXECUTE FUNCTION admin.notifications_guard();

-- Template versions are append-only (phase 09 function admin.append_only).
DROP TRIGGER IF EXISTS trg_notification_template_versions_append ON admin.notification_template_versions;
CREATE TRIGGER trg_notification_template_versions_append BEFORE UPDATE OR DELETE ON admin.notification_template_versions
    FOR EACH ROW EXECUTE FUNCTION admin.append_only();

-- Grants per write path (akr_app is the API and the worker). Every UPDATE lists the columns its service writes.
GRANT SELECT, INSERT ON casework.observations, casework.liaison_monthly_reports, admin.notifications,
      admin.notification_templates, admin.notification_template_versions, admin.user_notification_prefs TO akr_app;
GRANT UPDATE (hybrid_alert_id, hybrid_checked_at, updated_at, version) ON casework.observations TO akr_app;
GRANT UPDATE (draft, suppressed_cells, drafted_at, status, signed_by, signed_at, stepup_id, internal_report_id,
              returned_by, returned_at, return_comment_enc, reviewed_by, reviewed_at, review_comment_enc, timer_id,
              updated_at, version) ON casework.liaison_monthly_reports TO akr_app;
GRANT UPDATE (read_at, email_state, email_attempts, email_sent_at, email_error_code) ON admin.notifications TO akr_app;
GRANT DELETE ON admin.notifications TO akr_app;   -- the R12 sweep only (the guard refuses any other delete)
GRANT UPDATE (body, params, version, updated_at) ON admin.notification_templates TO akr_app;
GRANT UPDATE (email_mode, updated_at, version) ON admin.user_notification_prefs TO akr_app;
-- No grant to akr_readonly, akr_ingest, akr_audit_writer or akr_monitor: branch and notice content is personal and is read
-- through the API. Phase 10 canonical tables are read by akr_app under their own grants.

-- Settings (copy of the phase 20 and 21 column list). floor is set only where governed = TRUE (phase 00 CHECK).
INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('branch.liaison_due_day', '10', 'integer', '{"max": 10, "basis": "CBI ch. 11: by the 10th of the month; a bank may only set an earlier day"}'::jsonb, TRUE,
  '{"ar": "يوم استحقاق التقرير الشهري لضابط الارتباط من الشهر التالي", "en": "Day of the next month when the liaison monthly report is due"}'),
 ('branch.hybrid_window_days', '30', 'integer', '{"min": 1, "basis": "S-23: the window is the bank''s choice; the bank confirms"}'::jsonb, TRUE,
  '{"ar": "نافذة المعاملات بعد الملاحظة لإنشاء تنبيه مختلط (أيام)", "en": "Transaction window after an observation for a hybrid alert (days)"}'),
 ('branch.hybrid_scenario_code', '""', 'string', NULL, TRUE,
  '{"ar": "رمز سيناريو المعاملات الذي يُقرن بالملاحظة (فارغ: لا تنبيه مختلط)", "en": "Transaction scenario code paired with an observation (empty: no hybrid alert)"}'),
 ('branch.item_closed_days', '30', 'integer', '{"min": 1, "basis": "how long a closed branch request stays on the list; the bank confirms"}'::jsonb, TRUE,
  '{"ar": "مدة بقاء الطلب المغلق في قائمة الفرع (أيام)", "en": "Days a closed branch request stays on the list"}'),
 ('notify.email_enabled', 'false', 'boolean', NULL, TRUE,
  '{"ar": "إرسال الإشعارات بالبريد الإلكتروني عبر مرحّل البنك", "en": "Send notices by e-mail through the bank relay"}'),
 ('notify.smtp_host', '""', 'string', NULL, TRUE,
  '{"ar": "خادم مرحّل البريد في البنك", "en": "The bank SMTP relay host"}'),
 ('notify.smtp_port', '587', 'integer', '{"min": 1, "basis": "SMTP submission port range"}'::jsonb, TRUE,
  '{"ar": "منفذ مرحّل البريد", "en": "SMTP relay port"}'),
 ('notify.smtp_tls', '"starttls"', 'string', NULL, TRUE,
  '{"ar": "تشفير الاتصال بالمرحّل: starttls أو tls", "en": "Relay transport security: starttls or tls"}'),
 ('notify.smtp_from', '""', 'string', NULL, TRUE,
  '{"ar": "عنوان المرسل للإشعارات", "en": "From address of the notices"}'),
 ('notify.retention_days', '90', 'integer', '{"min": 30, "basis": "R12 operations class: 30 to 90 days (ARCHITECTURE section 17)"}'::jsonb, TRUE,
  '{"ar": "مدة الاحتفاظ بالإشعارات المقروءة (أيام)", "en": "Retention of read notices (days)"}'),
 ('schedule.branch_monthly_draft', '"0 30 0 1 * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول إنشاء مسودات التقرير الشهري لضابط الارتباط (أول كل شهر)", "en": "Monthly draft of the liaison reports (first day of the month)"}'),
 ('schedule.notification_digest', '"0 5 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول إرسال ملخص البريد للمستخدمين الذين اختاروا الملخص", "en": "Digest e-mail run for users who chose the digest"}'),
 ('schedule.notification_sweep', '"0 10 2 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول حذف الإشعارات المنتهية مدتها", "en": "Sweep of notices past their retention"}')
ON CONFLICT (key) DO NOTHING;

-- Classification (DATA-MODEL section 0 classes). Every sealed column is personal; the text of a template is internal because it
-- never holds a name (section 3.8).
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT v.s, v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('casework', 'observations', 'internal', ARRAY['id', 'branch_id', 'reporter_id', 'party_id', 'indicator_groups',
        'escalation_criteria', 'reporter_ack_code', 'reporter_ack_at', 'internal_report_id', 'hybrid_alert_id',
        'hybrid_checked_at', 'created_at', 'updated_at', 'version']),
  ('casework', 'observations', 'personal', ARRAY['companion_id_enc', 'notes_enc']),
  ('casework', 'liaison_monthly_reports', 'internal', ARRAY['id', 'branch_id', 'period', 'period_end', 'due_on', 'status',
        'draft', 'suppressed_cells', 'drafted_at', 'signed_by', 'signed_at', 'stepup_id', 'internal_report_id', 'returned_by',
        'returned_at', 'reviewed_by', 'reviewed_at', 'timer_id', 'created_at', 'updated_at', 'version']),
  ('casework', 'liaison_monthly_reports', 'personal', ARRAY['return_comment_enc', 'review_comment_enc']),
  ('admin', 'notifications', 'internal', ARRAY['id', 'user_id', 'template_code', 'template_version', 'params', 'dedupe_key',
        'source_topic', 'read_at', 'email_state', 'email_attempts', 'email_sent_at', 'email_error_code', 'created_at']),
  ('admin', 'notification_templates', 'internal', ARRAY['code', 'body', 'params', 'version', 'updated_at']),
  ('admin', 'notification_template_versions', 'internal', ARRAY['id', 'code', 'version', 'body', 'params', 'change_request_id',
        'approved_by', 'created_at']),
  ('admin', 'user_notification_prefs', 'internal', ARRAY['user_id', 'email_mode', 'updated_at', 'version'])
) AS v(s, t, cls, cols)
CROSS JOIN LATERAL unnest(v.cols) AS c
ON CONFLICT DO NOTHING;

-- Retention (phase 00 clocks; ARCHITECTURE section 17). An observation follows its register entry (R6, owner clock);
-- a monthly report is R6 from its creation; notices are R12 (operations), swept by the maintenance job of section 4.1;
-- templates and their versions and the preferences are configuration (R9).
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('casework', 'observations',                'R6',  'owner',      NULL,         'Observations follow their register entry (internal report, phase 19); the entry follows its case or the register'),
 ('casework', 'liaison_monthly_reports',     'R6',  'age',        'created_at', 'Monthly liaison reports: seven years from the draft'),
 ('admin', 'notifications',                  'R12', 'age',        'created_at', 'Notices: swept after notify.retention_days, read or unread (the guard allows a delete only past that age)'),
 ('admin', 'notification_templates',         'R9',  'retirement', NULL,         'Never deleted; retirement is not used, so the row outlives its versions'),
 ('admin', 'notification_template_versions', 'R9',  'owner',      NULL,         'Versions follow their template'),
 ('admin', 'user_notification_prefs',        'R9',  'owner',      NULL,         'Preferences follow the user (phase 01 offboarding)')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

The setting `branch.hybrid_scenario_code` ships empty, so no hybrid alert is produced until the bank names the scenarios (Open doubts 2). The settings `notify.smtp_*` are the bank relay's; the relay credential is an OpenBao item, never a setting (section 3.8).

## 2. DDD Implementation (Rust)

- **Crate placement**: the domain of this phase lives in `domain/branch` (observations, monthly reports, neutral status) and `domain/notify` (notices, templates, preferences, recipients), with the pure rules in `domain/*/rules.rs`; the repositories in `infrastructure/pg/branch.rs` and `infrastructure/pg/notify.rs`; the SMTP sender in `infrastructure/mail/smtp.rs` (worker only); the routers in `api/routes/branch.rs`, `api/routes/notifications.rs` and `api/routes/liaison_reports.rs`.
- **Enums** (closed in code; the SQL CHECK lists are the same values; serde renames as the codes shown):
  - `NeutralStatus { AwaitingYourInput => "awaiting_input", Received => "received", Closed => "closed" }` (M-66, section 3.1).
  - `IndicatorGroup` with eight variants: `AvoidCdd => "avoid_cdd"`, `InTransaction => "in_transaction"`, `CashPhysical => "cash_physical"`, `InstructedByOther => "instructed_by_other"`, `StaffPressure => "staff_pressure"`, `AfterTransaction => "after_transaction"`, `TfIndicator => "tf_indicator"`, `EscalationCheck => "escalation_check"`.
  - `EscalationCriterion { Esc1 => "esc_1", ..., Esc7 => "esc_7" }`: the seven criteria of the circular, in the order of research file 01 section A, item 8.
  - `LiaisonStatus { Draft => "draft", Signed => "signed", Reviewed => "reviewed" }`.
  - `NoticeCode`: the fifteen closed codes of section 3.8 (`TaskWaiting => "task.waiting"` and so on); a value outside this enum cannot reach the table.
  - `EmailMode { Immediate => "immediate", Digest => "digest" }`; `EmailState { None, Digest, Queued, Sent, Failed }`.
- **Domain entities** (`Debug, Serialize, Deserialize, FromRow, ToSchema` unless noted):
  - `Observation { id, branch_id, reporter_id, party_id, indicator_groups: Vec<IndicatorGroup>, escalation_criteria: Vec<EscalationCriterion>, reporter_ack_at, internal_report_id, hybrid_alert_id: Option<Uuid>, hybrid_checked_at: Option<DateTime<Utc>>, created_at, version }`. The sealed fields `notes_enc` and `companion_id_enc` are never selected into a struct that a route returns to a branch reader; `ObservationView` (for M only, 7.2) carries the decrypted notes through the phase 02 personal seal (AAD = the observation id, as section 3.3 step 5 applies it).
  - `LiaisonMonthlyReport { id, branch_id, period, period_end, due_on, status: LiaisonStatus, draft: serde_json::Value, suppressed_cells: i16, drafted_at, signed_by: Option<Uuid>, signed_at: Option<DateTime<Utc>>, stepup_id: Option<Uuid>, internal_report_id: Option<Uuid>, returned_at, reviewed_by, reviewed_at, version }`.
  - `Notice { id, user_id, template_code: NoticeCode, template_version: i32, params: NoticeParams, dedupe_key, source_topic, read_at, email_state: EmailState, created_at }`.
  - `NoticeParams { count: Option<i64>, due_date: Option<NaiveDate>, branch_code: Option<String>, alarm_kind: Option<String> }`: the only four keys; `Serialize` writes no other key (`skip_serializing_if` on each).
  - `NotificationTemplate { code, body: BilingualText { ar, en }, params: Vec<PlaceholderKey>, version, updated_at }`.
  - `BranchRef { id, code, name: I18nText, role: BranchRole }` with `BranchRole { Liaison, Alternate }` (resolved from `admin.branches` at request time, section 3.1).
- **Create and update structs**:
  - `ObservationCreate { party_id: Option<Uuid> (NULL when the person is not a customer of the branch; the branch is the caller's, never sent), indicator_groups: Vec<IndicatorGroup> (1 to 8, no duplicate), escalation_criteria: Vec<EscalationCriterion> (0 to 7), companion_id: Option<String> (max 20 characters, sealed), notes: String (1 to 4,000 characters), acknowledgment: AcknowledgmentInput { ack_code: String, accepted: bool } }`. `Deserialize, ToSchema`.
  - `ConfidentialReportCreate { body: String (1 to 4,000 characters), subject_party_id: Option<Uuid>, acknowledgment: AcknowledgmentInput }`.
  - `AnswerInput { question_key: String (pattern of phase 19), answer_kind: AnswerKind (`yes`, `no`, `unknown`, `value`, `document`, `free_text`), value: Option<String> (max 500 characters), evidence_ids: Vec<Uuid> (0 to 5), supersedes: Option<Uuid> }` (the phase 19 `AnswerInput` with `question_key` in place of `question_id`, the source kind fixed to `branch_officer`, corroboration `none` and no `corroborates`, which is the desk's (section 3.2 step 5); the route finds `question_id` through `casework.rfi_questions`, unique on `(rfi_id, question_key)`; the 500-character cap of `value` is this route's own, narrower than the 2000 of phase 19).
  - `MonthlyReportSign { }` (the step-up assertion is the `X-StepUp-Assertion` header; the body is empty but present as `{}`), `LiaisonReviewInput { outcome: ReviewOutcome (accepted, returned), comment: Option<String> (max 1,000 characters; required when returned) }`, `PrefsUpdate { email_mode: EmailMode }`.
  - `TemplateChange` is not a route: a template edit is a phase 04 change request whose body is `{ code, body, params }` (section 3.8).
- **Repository traits** (domain layer):
  - `BranchRepository`: `branches_of_user(conn, user_id) -> Vec<BranchRef>` (`admin.branches` where `liaison_user_id = $1 OR alternate_user_id = $1` and `archived_at IS NULL`); `branch_of_party_account(conn, party_id, branch_id) -> bool` (`canon.account a JOIN canon.branch b ON b.id = a.branch_id` with `b.org_branch_id = $2` and a role on `party_id` through `canon.account_party_role`; phase 10: `canon.account` has `branch_id`, and `canon.branch.org_branch_id` maps it to `admin.branches`); `recipient_in_branch(rfi.recipient_ref, branch_id)` (`canon.staff s JOIN canon.branch b ON b.id = s.branch_id WHERE s.id = rfi.recipient_ref AND b.org_branch_id = $2`: the RFI's addressed liaison works at the caller's branch).
  - `RfiBranchRepository` (a read port over phase 19 tables): `released_items(conn, branch_id, cursor) -> Vec<RfiRow>` with the three suspicion checks of section 3.1 step 2 in SQL; `item(conn, rfi_id, branch_id) -> Option<RfiRow>` (the same predicate, so a hidden item is `None`).
  - `ObservationRepository`: `insert(tx, NewObservation)`; `set_hybrid(tx, id, alert_id: Option<Uuid>, checked_at)`; `unchecked(conn, limit) -> Vec<Uuid>` (`hybrid_checked_at IS NULL`, oldest first).
  - `LiaisonReportRepository`: `by_branch_period(conn, branch_id, period)`, `get(conn, id)`, `insert_draft(tx, NewDraft)`, `set_draft(tx, id, draft, suppressed_cells, expected_version)`, `set_signed(tx, id, user, stepup, internal_report_id, expected_version)`, `set_returned(tx, id, user, comment_enc)`, `set_reviewed(tx, id, user, comment_enc)`, `list(conn, filter, cursor)`.
  - `NoticeRepository`: `insert_if_new(tx, NewNotice) -> bool` (`INSERT ... ON CONFLICT (dedupe_key) DO NOTHING`, the boolean says whether a row was written); `list(conn, user, cursor, unread_only)`; `unread_count(conn, user)`; `mark_read(tx, id, user) -> Option<()>` (`user_id = $2` in the predicate: another user's id is `None`); `digest_due(conn, limit)`; `set_email_state(tx, id, state, attempts, sent_at, error_code)`; `get_for_update(tx, id) -> Option<Notice>` (the notice row `FOR UPDATE`, read by the job `notification_email`); `sweep_read_older_than(tx, days, limit)`.
  - `TemplateRepository`: `get(conn, code)`, `get_version(conn, code, version) -> Option<NotificationTemplate>` (one stored version of `admin.notification_template_versions`, for the render of `NotificationService::list`), `all(conn)`, `put_version(tx, NewVersion)` (used by the change-request apply hook only).
  - `PrefsRepository`: `get(conn, user) -> EmailMode` (default `digest` when no row); `upsert(tx, user, mode, expected_version)`.
  - `MailSender` (port, worker): `send(message: OutgoingMail) -> Result<(), MailError>`; the only implementation is `SmtpRelaySender` (section 3.8.3).
- **Repository impl notes**: `released_items` joins `casework.rfis`, `casework.cases`, `casework.rfi_questions` (question keys only; no `value_enc` is selected: the own answers come from `rfi_answers` in `item`, with `value_enc` decrypted for the caller's own claims only); `insert_if_new` uses the `dedupe_key` unique index; `mark_read` returns the row with `RETURNING id`; `sweep_read_older_than` deletes in batches of 500 (section 4.1).
- **utoipa schemas**: `BranchHomeView { branch: BranchRef, neutral_counts: NeutralCounts, monthly: Option<MonthlySummary> }`, `BranchItemView { id, kind: "branch_info", status: NeutralStatus, due_at, question_keys: Vec<String>, answered_count, evidence_requested: bool }`, `BranchItemDetail` (the view plus the question list with the neutral question text key and the own answer claims), `ObservationReceipt { id, status: "received", acknowledged_at }`, `ConfidentialReceipt { id, status: "received" }`, `MonthlyReportView { id, period, due_on, status: LiaisonStatus, counts: NeutralCounts (null below the threshold), suppressed_cells, signed_at, returned: bool, can_sign: bool }`, `LiaisonReportListItem` (for M), `LiaisonReportReview` (for M), `NotificationView { id, template_code, text: { ar, en } (rendered from the template and the params), read_at, created_at }`, `UnreadCount { unread }`, `PrefsView { email_mode }`, `TemplateView { code, body, params, version, updated_at }`, `StagedEvidence { evidence_id, sha256, size_bytes }`.
- **Router**: `pub fn router() -> Router<AppState>` in `api::routes::branch` (the branch routes), `api::routes::notifications` (the two `/me` routes, the unread count, the preferences and `GET /admin/notification-templates`), and `api::routes::liaison_reports`.

**Route table** (scopes from API.md section 2; rows marked "added" are new to API.md 3.17, 3.16 and 3.1; the scope gate of every `/branch/*` route answers 404 instead of 403 when the caller lacks `branch.window`, DR-206):

| Method | Path | Scope | Description |
|-|-|-|-|
| GET | `/api/v1/me/notifications` | `self` | In-app notices of the caller, newest first, cursor; `unread=true` filter (phase 22, API.md 3.1) (cedar `notification.list`) |
| POST | `/api/v1/me/notifications/{id}/read` | `self` | Mark one of the caller's notices read; another user's id is 404 (phase 22, API.md 3.1) (`notification.mark_read`) |
| GET | `/api/v1/me/notifications/unread-count` | `self` | Unread count of the caller (added) (`notification.list`) |
| GET | `/api/v1/me/notification-preferences` | `self` | The caller's e-mail mode (added) (`notification.prefs_read`) |
| PUT | `/api/v1/me/notification-preferences` | `self` | Set the e-mail mode; `If-Match`; 428 without it (added) (`notification.prefs_write`) |
| GET | `/api/v1/admin/notification-templates` | `admin.system` | Templates with their current version; read only, edits go through change requests (added) (`notification_template.read`) |
| GET | `/api/v1/liaison-reports` | `case.decide` | Liaison monthly reports for review, cursor; filters `branch_id`, `status`, `period` (phase 22, API.md 3.16) (`liaison_report.list`) |
| POST | `/api/v1/liaison-reports/{id}/review` | `case.decide` | Accept (`reviewed`) or return (to `draft`) with a comment; `If-Match`, `Idempotency-Key` (phase 22, API.md 3.16) (`liaison_report.review`) |
| GET | `/api/v1/branch/home` | `branch.window` | Branch name, my role (liaison or alternate), the due date, neutral counts (added) (`branch.home_read`) |
| GET | `/api/v1/branch/items` | `branch.window` | Released branch requests of my branch, neutral statuses only, cursor (phase 22, API.md 3.17) (`branch.item_list`) |
| GET | `/api/v1/branch/items/{id}` | `branch.window` | One item: neutral question keys, deadline, neutral status, my own answers (phase 22, API.md 3.17) (`branch.item_read`) |
| POST | `/api/v1/branch/items/{id}/evidence` | `branch.window` | Stage one document for the item (multipart; `EvidenceStore::stage`, owner kind `rfi_input`) (added) (`branch.evidence_stage`) |
| POST | `/api/v1/branch/items/{id}/input` | `branch.window` | Answer the item as claims through `RfiService::record_answers_in_tx`; `If-Match` = RFI version (phase 22, API.md 3.17) (`branch.item_answer`) |
| POST | `/api/v1/branch/observations` | `branch.window` | Record an observation: eight groups, companion ID, written acknowledgment (phase 22, API.md 3.17) (`branch.observation_create`) |
| POST | `/api/v1/branch/confidential-reports` | `branch.window` | Confidential report to the MLRO; a neutral receipt only (phase 22, API.md 3.17) (`branch.confidential_create`) |
| GET | `/api/v1/branch/monthly-report` | `branch.window` | The caller's branch report for the open period (draft, signed or returned) (phase 22, API.md 3.17) (`branch.monthly_read`) |
| POST | `/api/v1/branch/monthly-report/sign` | `branch.window` | Sign and send; **SU** (`signing`); `If-Match`; `Idempotency-Key` (phase 22, API.md 3.17) (`branch.monthly_sign`) |

Count: 17 routes (11 rows of API.md owned here, 6 added). The Cedar actions of the table are declared in section 3.10; `liaison_report.list` and `liaison_report.review` are the only actions that use the scope `case.decide` and are restricted to `M` in Cedar (the scope admits A2 too, as API.md section 2 says).

**Errors** (`BranchError` and `NoticeError`, `thiserror`; problem codes of API.md section 6 only):

| Variant | Problem | Sub-code |
|-|-|-|
| `NotFound` (unknown id, another branch's item, a suspicion-tier item, a notice of another user, the 404 scope gate) | 404 `E_NOT_FOUND` | none (identical body) |
| `NotReleased` (answer to an item that is not `released`) | 409 `E_NOT_ALLOWED_IN_STATE` | `not_released` |
| `AlreadySigned` | 409 `E_NOT_ALLOWED_IN_STATE` | `already_signed` |
| `NotSigned` (review of a draft) | 409 `E_NOT_ALLOWED_IN_STATE` | `not_signed` |
| `VersionConflict` | 409 `E_VERSION_CONFLICT` | none (current version in the body) |
| `AcknowledgmentRequired` (no tick, or a code other than `OBS_ACK_V1`) | 422 `E_VALIDATION` | `acknowledgment_required` |
| `EscalationChecklistRequired` (group 8 without criteria, or criteria without group 8) | 422 `E_VALIDATION` | `escalation_checklist` |
| `PartyNotFound` (unknown party and a party with no account at the caller's branch: the same answer) | 422 `E_VALIDATION` | `party_not_found` |
| `ForbiddenTerm` (phase 09 scanner hit; the span and the field, never the term) | 422 `E_VALIDATION` | `forbidden_term` |
| `TooLarge` / `TypeNotAllowed` / `Rejected` (evidence) | 413 `E_PAYLOAD_TOO_LARGE` / 422 `E_UPLOAD_REJECTED` | none |
| `StepUpRequired` | 401 `E_STEPUP_REQUIRED` | none |
| `PreconditionRequired` | 428 `E_PRECONDITION_REQUIRED` | none |
| `Forbidden` (M-only routes called by A2 with the scope) | 403 `E_FORBIDDEN` | none |
| `IdempotencyRequired` | 400 `E_IDEMPOTENCY_REQUIRED` | none |

## 3. Business Logic

Branch identity is resolved on every request from `admin.branches` (phase 03), never from a session flag: a caller is a branch member when `liaison_user_id` or `alternate_user_id` is the caller and `archived_at IS NULL`. A caller holds at most one branch (DR-205). A branch item is a phase 19 RFI of kind `branch_info`; a branch record is a phase 19 internal report of channel `branch_window` or `liaison_monthly`; no branch route reads a case, a decision or a party's detail.

### 3.1 The neutral status mapper and the item list (`BranchWindowService::items`, `BranchWindowService::item`, `BranchStatusMapper`; M-66, UX-66, M-43)

**`BranchStatusMapper`** (pure, no IO; the only function that turns an internal state into a word a branch reader sees). Both maps are exhaustive `match` expressions with no wildcard arm, so a new state fails the build:

| Source | Internal state | Shown to the branch as | Labels (ar / en) |
|-|-|-|-|
| RFI (phase 19 `casework.rfis.status`) | `released` | `awaiting_input` | بانتظار مدخلاتكم / Awaiting your input |
| RFI | `answered` | `received` | مستلم / Received |
| RFI | `closed`, `withdrawn` | `closed` | مغلق / Closed |
| RFI | `draft`, `pending_release`, `rejected` | none: the item is not listed and its id is 404 | none |
| Internal report (phase 19 `status`) | `received`, `acknowledged`, `linked` | `received` | مستلم / Received |
| Internal report | `closed` | `closed` | مغلق / Closed |

**`BranchWindowService::items(ctx) -> Page<BranchItemView>`** (`GET /branch/items`; Cedar `branch.item_list` on System):
1. `branch = branches_of_user(caller)`; none: an empty page (a liaison with no branch yet).
2. Query `RfiBranchRepository::released_items(branch.id)`: `rfi.kind = 'branch_info'` AND `rfi.recipient_kind = 'liaison'` AND `rfi.released_at IS NOT NULL` AND `rfi.status IN ('released', 'answered', 'closed', 'withdrawn')` AND (`rfi.status IN ('released', 'answered')` OR `rfi.closed_at > now() - make_interval(days => branch.item_closed_days)`) AND `rfi.tier <> 'suspicion'` AND `c.tier <> 'suspicion'` (the case of the RFI, phase 17 `casework.cases.tier`, which only rises) AND `casework.effective_tier(rfi.party_id) <> 'suspicion'` AND `recipient_in_branch(rfi.recipient_ref, branch.id)` (the branch of the addressed liaison, phase 19 section 3.1 step 4 stores that liaison's `canon.staff` row in `recipient_ref`; never the branches where the party holds accounts, which would show an RFI addressed to one branch to every branch of the customer: M-43, lead review). The predicate is one SQL `WHERE` clause, so the list, the detail and the counts of section 3.9 cannot disagree.
3. Map each row with `BranchStatusMapper`; the view carries `id`, `status`, `due_at` (only while `awaiting_input`), `question_keys` (phase 19 `rfi_questions.question_key`) and `answered_count`. No case id, family, priority, tier, party, reason code or decision is selected.
4. Cursor on `(created_at, id)`; no total count (phase 17 rule for lists that can hold suspicion rows).

**`BranchWindowService::item(ctx, rfi_id)`** (`GET /branch/items/{id}`; Cedar `branch.item_read` on BranchItem):
1. Same predicate with `id = $rfi_id`; no row: `NotFound`. An item of another branch, a suspicion item, an unknown id and a withdrawn-after-period item give the identical 404 (the body is the phase 17 masked body, byte-identical; the response has `Cache-Control: no-store`).
2. The view adds the question list (key and answer format; the neutral question text is the pack's template text, scanned by phase 09 scope `rfi` at drafting) and the caller's own current answer claims (`rfi_answers` with `claim_state = current` and `answered_by = caller`), without `corroboration`, `source_kind`, `corroborated_by` or `expires_at`.

### 3.2 The input: answers as claims and evidence (`BranchWindowService::answer`, `BranchWindowService::stage_evidence`; L-47, M-45, UX-66)

**`BranchWindowService::stage_evidence(ctx, rfi_id, upload)`** (`POST /branch/items/{id}/evidence`; multipart, one file; Cedar `branch.evidence_stage`):
1. Load the item with section 3.1's predicate; not visible: `NotFound`; status not `released` or `answered`: 409 `E_NOT_ALLOWED_IN_STATE` sub-code `not_released`.
2. The file name is replaced before staging by `document-<rfi short id>-<n>` (the branch's file names can hold a customer's name; the stored name never does).
3. `EvidenceStore::stage(tx, NewEvidence { kind: upload, owner_kind: "rfi_input", owner_id: rfi_id, retention_class: "R6", content_type, file_name: neutral name, classification: personal, lang: und, bytes }, actor)` (phase 02 steps 1 to 4: size, type, owner registry, hash and seal). The owner kind `rfi_input` is registered in the phase 02 owner registry by this phase (section 4.5 item 3).
4. The phase 02 malware scan runs in the worker (`evidence_scan`); until its `av_status` is `clean` it cannot be used in an answer (section 3.2 step 5).
5. Audit `branch.evidence_staged {rfi_id, evidence_id}` (phase 02 audits `evidence.staged` too). Response 201 `StagedEvidence`.

**`BranchWindowService::answer(ctx, rfi_id, If-Match, Vec<AnswerInput>)`** (`POST /branch/items/{id}/input`; Cedar `branch.item_answer`; `If-Match` = the RFI version; `Idempotency-Key`):
1. Load the item with section 3.1's predicate (lock order of section 3.14 applies from step 3): not visible: `NotFound`. The RFI status must be `released` or `answered` (else 409 `not_released`).
2. `If-Match` absent: 428; not equal to `rfi.version`: 409 `E_VERSION_CONFLICT`.
3. Each `AnswerInput`: `question_key` belongs to the RFI and is `open` (or `answered` when it corrects a current claim through `supersedes`, phase 19 section 3.3 step 2; `supersedes` must name a current claim of this RFI whose `source_kind` is `branch_officer`, else 422 `E_VALIDATION` at `/answers/{i}/supersedes`); `answer_kind` is allowed by the question's `answer_format` (`yes_no_unknown` allows `yes`, `no`, `unknown`; `value` allows `value`; `document` allows `document`; `free_text` allows `free_text`); else 422 `E_VALIDATION` at `/answers/{i}`.
4. Text: each `value` of kind `free_text` or `value` goes through `PolicyPackService::scan(scope = "branch_form", text)` (phase 09). A hit is 422 `E_VALIDATION` `forbidden_term` with the field and the span, never the term.
5. Evidence: an `evidence_id` must be an `admin.evidence_objects` row with owner kind `rfi_input`, owner id `rfi_id`, `created_by` = the caller and `av_status = 'clean'` (`version_id` set); else 422 `E_VALIDATION` sub-code `evidence_pending` (not scanned yet) or `evidence_unknown` (anything else, identical for both owners). A `document` answer needs at least one evidence id (at most 5); the ids go into the answer's sealed value, never into `evidence_object_id`. Every branch claim is written with `corroboration = 'none'` and `evidence_object_id` NULL: phase 19's CHECKs tie a corroboration to `corroborated_by` and `documented` to one evidence object, and a claim corroborated by the person who gave it would pass the M-45 closure gate on the giver's word. The desk raises the corroboration through phase 19's `corroborates` path, naming one of the staged `rfi_input` objects (section 4.5 item 2; lead review).
6. Call the phase 19 inner method `RfiService::record_answers_in_tx(tx, actor, rfi_id, answers, source_kind)` (new in phase 19 by section 4.5 item 2: the body of `record_answers` without its `rfi.answer` Cedar check, which a liaison outside the case team would fail; this route authorises with `branch.item_answer`) with `source_kind = branch_officer`, `answered_by = caller`, `answered_at = now()`, `expires_at = answered_at + casework.rfi_claim_expiry_days` (phase 19 step 3). The port is the same method the desk uses; this phase adds no claim logic. The answer values are sealed by phase 19 (AAD = answer id).
7. Audit `branch.item_answered {rfi_id, answer_count, evidence_count}` (ids and counts only). Phase 19 writes its own `rfi.answer_recorded` and the case chain event `case.rfi_answered` (for the case team only).
8. Response: `BranchItemDetail` after the write. The status moves to `answered` when every required question is answered, which the branch sees as "Received".

### 3.3 The observation log (`ObservationService::create`; S-22, UX-67; job `observation_hybrid`, section 3.3.1)

**`ObservationService::create(ctx, ObservationCreate)`** (`POST /branch/observations`; Cedar `branch.observation_create`; `Idempotency-Key`):
1. `branch = branches_of_user(caller)`; none: `NotFound`.
2. Form rules: `indicator_groups` non-empty, no duplicates, each from the eight codes; `escalation_criteria` non-empty if and only if `escalation_check` is in the groups (422 `escalation_checklist`); `acknowledgment.ack_code = "OBS_ACK_V1"` and `accepted = true` (else 422 `acknowledgment_required`). The reminder text is the screen's string bound to `OBS_ACK_V1` in the glossary (X5 owns the wording); the server stores the code and the time, not the text.
3. Party (when given; a walk-in or a third party is recorded without one, described in the notes): `branch_of_party_account(party_id, branch.id)`; false: 422 `E_VALIDATION` sub-code `party_not_found` (the same answer for an unknown party, so a branch learns nothing about parties of other branches).
4. Notes: `PolicyPackService::scan(scope = "branch_form", notes)`; a hit: 422 `forbidden_term` with the span.
5. Seal `notes` and `companion_id` with the phase 18 field-sealing pattern (`text_enc`, AAD = the observation id, generated first as UUIDv7).
6. One transaction: (a) `InternalReportService::receive(tx, CreateInternalReport { reporter_kind: liaison, reporter_id: caller, branch_id, subject_party_id: party_id, channel: branch_window, body: notes, received_at: now() })` (phase 19 section 3.4: the body is sealed there, `ack_due_at` and timer `intake_clock` are set there, and `internal_report.recorded` is written there); (b) insert `casework.observations` with `reporter_ack_at = now()` and the register id; (c) enqueue job `observation_hybrid` (coalesce `observation_hybrid:<id>`); (d) audit `branch.observation_recorded {observation_id, branch_id, internal_report_id, group_codes, criteria_count}`.
7. Response 201 `ObservationReceipt { id, status: "received", acknowledged_at }`. Nothing else is returned: no alert id, no hybrid result, no status.

**Tipping-off in this flow.** The branch never reads the observation again (no route lists or reads it), so the only things it learns are its own input and the word "Received". The MLRO reads the register (phase 19 `GET /internal-reports`) and the structured groups through the rows that section 7.2 extends (Open doubts 9).

**3.3.1 The hybrid alert** (`HybridObservationJob::run`, job `observation_hybrid`; S-23; DR-212):
1. Read the observation (no lock). If `hybrid_checked_at IS NOT NULL`: no-op. If `party_id IS NULL`: set `hybrid_checked_at = now()` and stop (no party, no pattern).
2. If `branch.hybrid_scenario_code` is empty (the shipped value): set `hybrid_checked_at = now()` and stop. No alert is created until the bank names a scenario code (Open doubts 2).
3. Pattern alerts: `alert.alerts` rows with `source_kind = 'scenario'` (any transaction scenario, never an `observation` alert), `linked_party_ids` containing the observation's `party_id`, and `created_at` within `[reporter_ack_at - branch.hybrid_window_days, reporter_ack_at]` (the partitions of that window only). None: set `hybrid_checked_at = now()` and stop.
4. Build the payload (phase 16 `DetectionPayload` with `source_kind = observation`, section 4.5 item 4): `scenario_code = branch.hybrid_scenario_code`; `fingerprint` = SHA-256 of `observation|<observation_id>|<scenario code>`; `fingerprint_inputs {scenario_id, entity_kind: "party", entity_id: party_id, anchor_hk: observation_id, bucket: the observation day}`; `matched_txn_ids` = the first 500 transaction ids of the pattern alerts; `measures {groups: n, pattern_alerts: k}`; `case_family = "observation"` (phase 17 family, tier `normal`); `severity` from the scenario's outputs rule (DSL A.8).
5. One transaction: `AlertIntakeService::emit(tx, payload)` (phase 17 steps 1 to 13; the case is opened or found by phase 17 step 5). `Duplicate` (the fingerprint exists): set `hybrid_checked_at` only. `New`: `ObservationRepository::set_hybrid(tx, id, alert_id, now())`; audit `branch.observation_hybrid_alert {observation_id, alert_id}`.
6. Lock order: the case row (taken by phase 17 step 5) before the observation row (section 3.14). The job never holds the observation lock while it calls phase 17.
7. Failure: the job's back-off retries; after `max_attempts` the row keeps `hybrid_checked_at` NULL and the cockpit shows the job as dead (phase 00).

### 3.4 The confidential report (`ObservationService::confidential`; M-68, DR-202)

**`ObservationService::confidential(ctx, ConfidentialReportCreate)`** (`POST /branch/confidential-reports`; Cedar `branch.confidential_create`):
1. `branch` from the caller; none: `NotFound`. `acknowledgment` as in section 3.3 step 2 (`OBS_ACK_V1`).
2. If `subject_party_id` is given: `branch_of_party_account` (422 `party_not_found` otherwise).
3. `PolicyPackService::scan(scope = "branch_form", body)`.
4. One transaction: `InternalReportService::receive(tx, { reporter_kind: liaison, reporter_id: caller, branch_id, subject_party_id, channel: branch_window, body, received_at: now() })`; audit `branch.confidential_recorded {internal_report_id, branch_id}`.
5. Response 201 `ConfidentialReceipt { status: "received" }`. The receipt is the same whatever the body says; the branch has no route to the report.

### 3.5 The monthly liaison report: the draft (`LiaisonReportService::draft_month`, job `branch_monthly_draft`; M-25 of research 28, UX-66)

**Due date rule** (pure, `due_on(period, bank_calendar)`; DR-211): `d = branch.liaison_due_day` (10 seeded). `from` = the day `d - 1` of the month after the period at 23:59:59 in `bank.timezone`; `due_on` = the local date of `BusinessClock::add(from, 1, BusinessUnit::WorkingDays)` (phase 09; `add` does not count the day of `from`, so the result is the first working day on or after day `d`). Example for the seeded calendar: the 10th of October 2026 is a Saturday; with the default week (weekend Friday and Saturday) and no closed day in the window, `due_on` is Sunday 11 October 2026.

**`LiaisonReportService::draft_month(period)`** (job `branch_monthly_draft`, payload `{period}`; the default period is the month before the job's run date):
1. For each branch with a liaison or an alternate and `archived_at IS NULL` (`admin.branches`), in one transaction per branch:
   a. Skip if a row exists for `(branch_id, period)` (`UNIQUE`; `INSERT ... ON CONFLICT DO NOTHING`).
   b. `period_end` = the last calendar day of the period; `due_on` as above.
   c. Counts (section 3.9) for the period: items released in the period by neutral status and observations recorded in the period (counts only; confidential reports are never counted on a branch surface, because the alternate would learn that the liaison reported, M-68; M sees them on R5 and in the register). A count above zero and below `report.small_number_threshold` (phase 21 setting, governed, minimum 3) becomes `null` and adds one to `suppressed_cells`.
   d. Insert the row (status `draft`, `draft` = the counts, `drafted_at = now()`).
   e. Timer `liaison_due`: `TimerService::schedule(tx, NewTimer { owner_kind: "liaison_monthly_report", owner_id, kind: "liaison_due", fire_at: (due_on + 1 day) 00:00 bank time, calendar_id: the bank calendar, payload: {} })`; store `timer_id` (section 3.6 handles it).
   f. Outbox `liaison_report.drafted {report_id}` (the dispatcher sends `branch.monthly_due` with the due date to the liaison and alternate; section 3.8).
   g. Audit `branch.monthly_drafted {report_id, branch_id, period, suppressed_cells}`.
2. The job is idempotent by `(branch_id, period)`. A branch that gets its first liaison during the month receives its draft at the next run; no route creates it earlier (Open doubts 10).

### 3.6 Signing (`LiaisonReportService::sign`; route `POST /branch/monthly-report/sign`; step-up `branch.monthly.sign`, class `signing`; DR-203)

**`StepUpAction { name: "branch.monthly.sign", class: signing }`** is declared in this phase (phase 01 closed classes; no new class).

**`LiaisonReportService::sign(ctx, If-Match, step_up_header)`**:
1. Open report of the caller's branch: the latest row with `status <> 'reviewed'` (`period` descending). None: `NotFound`. `reviewed`: 409 `already_signed`. `signed` and not returned: 409 `already_signed`.
2. `If-Match` checks as in section 3.2 step 2 (version of the report).
3. `StepUpService::consume(tx, ctx, step_up_header, StepUpAction::BRANCH_MONTHLY_SIGN, resource_ref = report id)` (phase 01; a missing or replayed assertion is 401 `E_STEPUP_REQUIRED`).
4. Lock the report row `FOR UPDATE` (lock order rank 3, section 3.14).
5. Recompute the counts (section 3.5 step 1c) and write them into `draft` with `set_draft`, so the signed content is the content the MLRO will read.
6. If `internal_report_id IS NULL` (first signature): `InternalReportService::receive(tx, { reporter_kind: liaison, reporter_id: caller, branch_id, subject_party_id: NULL, channel: liaison_monthly, body: sealed JSON {report_id, period}, received_at: now() })`. The body carries no count and no text, so the register entry is a pointer; the content is read in R5 (section 3.7). A re-signature after a return reuses the id and writes no second entry (phase 21 counts one entry per branch and period).
7. `set_signed(report, caller, stepup_id, internal_report_id, expected_version)`: status `signed`, `signed_by`, `signed_at = now()`, `stepup_id`. The due timer is left in place: it re-reads the status when it fires.
8. Outbox `liaison_report.signed {report_id, branch_id, period}` (the dispatcher sends `liaison.report_received` to M with the branch code).
9. Audit `branch.monthly_signed {report_id, period, internal_report_id, stepup_id}`.
10. Response `MonthlyReportView`.

**Timer `liaison_due`** (handler job `timer_liaison_due`; owner `liaison_monthly_report`; phase 00 rule: re-read the owner first):
1. Re-read the report. Status `signed` or `reviewed`: no-op (a signature on the due day or later is accepted; a late signature is counted by phase 21's `liaison` pack).
2. Status `draft` (never signed, or returned and not yet re-signed): `AlarmService::raise(tx, NewAlarm { kind: liaison_report_overdue, subject_kind: liaison_report, subject_id: report_id, stage: NULL, payload: {branch_id, period} })` (phase 19 section 3.10; the supervisor is M, section 3.10 table); the alarm writes the phase 19 outbox `alarm.raised {alarm_id, kind}`, which the dispatcher turns into `liaison.report_overdue` for M.

### 3.7 The review (`LiaisonReportService::review`; routes `GET /liaison-reports`, `POST /liaison-reports/{id}/review` on R5; Cedar `liaison_report.review` (M); DR-203)

1. `GET /liaison-reports` (`liaison_report.list`, M): cursor by `(due_on, id)`; filters `branch_id`, `status`, `period`; the view shows the branch code and name, period, due date, status, counts with `suppressed_cells`, the signature time and the signer's display name (the liaison of the branch; the M desk is entitled to see who signed).
2. `POST /liaison-reports/{id}/review` (`If-Match`, `Idempotency-Key`):
   a. Load the report `FOR UPDATE`. Status `draft`: 409 `not_signed`. Status `reviewed`: 409 `E_NOT_ALLOWED_IN_STATE` sub-code `already_reviewed`.
   b. Self-review: `reviewed_by = signed_by` is refused: 403 `E_SELF_APPROVAL` (Cedar `liaison.self_review`).
   c. `comment` scanned with `PolicyPackService::scan(scope = "reason_text", comment)`; a hit: 422 `forbidden_term`.
   d. `accepted`: `set_reviewed(report, M, review_comment_enc)` (status `reviewed`); audit `branch.monthly_reviewed {report_id}`.
   e. `returned`: the comment is required (422 `comment_required`); `set_returned(report, M, return_comment_enc)` (status back to `draft`, `returned_at`, `returned_by`; the signature fields stay as history); outbox `liaison_report.returned {report_id, branch_id}` (the dispatcher sends `branch.monthly_returned`, which never carries the comment); audit `branch.monthly_returned {report_id}`.
3. The due timer of a returned report still fires; a report returned and not re-signed by the due day raises the overdue alarm. A return made after the timer has fired (the due day has passed) raises the same alarm in the return transaction, with the `AlarmService::raise` call of section 3.6 timer step 2; the live-alarm index keeps one alarm.

### 3.8 Templates, notices and the dispatcher (`NotificationDispatcher`, `NotificationService`, `TemplateService`; M-68, M-66, X3, ARCHITECTURE section 15)

**Closed codes.** The fifteen template codes of section 1.2 are the `NoticeCode` enum. A code outside the enum cannot be written: the table has no other values in the application. Parameters are the four keys of `NoticeParams` (`count`, `due_date`, `branch_code`, `alarm_kind`); a template may use only the placeholders listed in its `params` column.

**Template edits (DR-210).** A template changes only by a phase 04 change request: target kind `notification_template` (`NotificationTemplateTarget`, registered in the `ChangeTargetRegistry` by this phase; route `cr_material`, so CHK and M approve in parallel). Validation before the change is accepted: both languages present (`E_MISSING_I18N`); placeholders from the template's `params` only (`unknown_key`); no digit outside a placeholder (phase 09 rule `template_has_figures`); `PolicyPackService::scan(scope = "notification")` on both languages (`forbidden_term`, span only); the code exists (`unknown_key`). The apply hook, in the change request's transaction: lock the template row, insert `admin.notification_template_versions` (version + 1, the change request id and the approver), update `admin.notification_templates` (body, params, version, `updated_at`); audit `notification_template.changed {code, version, change_request_id}`. Notices keep the version they were written with, so an edit never rewrites a notice already sent.

**Dispatcher** (`NotificationDispatcher::handle(tx, outbox_row)`, the outbox handler of topic `notify.request` and of the topics of the table below; phase 00 back-off applies on error):
1. Route the topic to its mapping (the table below), a pure function. Unknown topic: error (the row stays undispatched, counted by `akr_outbox_stuck`).
2. Read the source row by the ids in the payload (no text is read from the payload; the payload carries ids and codes only).
3. Compute the resource's tier (`suspicion`, `restricted` or `normal`; phase 17 `casework.effective_tier` for a party, the case tier for a case, the RFI tier for an RFI).
4. Recipients: the mapping's query (roles from phase 03's effective-role read port, branch membership from `admin.branches`, case team from `casework.case_acl` with `revoked_at IS NULL`, the RFI desk from phase 19's desk predicate), then the visibility filter: a recipient is kept only if the recipient could read the resource (the phase 17 predicate for cases, the phase 19 desk predicate for RFIs, the audience of phase 19 section 3.10 for alarms). A recipient who cannot read it is dropped silently.
5. Template: the mapping's template; when the tier is `suspicion` and the template is not `task.waiting`, the template is `task.waiting` (the notice says only that work is waiting).
6. Counts (`count`) are computed per recipient over that recipient's visible set, never over the resource's full set.
7. For each recipient: `NoticeRepository::insert_if_new(tx, { dedupe_key: "<outbox_id>:<user_id>:<template code>", template_version, params, source_topic, email_state })`. `email_state` is `none` when e-mail is off or the recipient has no address; `digest` for a digest recipient; `queued` for an immediate recipient (and the job `notification_email` is enqueued in the same transaction).
8. After commit: publish `notification.new {notification_id}` to the recipient's channel (section 3.8.2, SSE).
9. Audit `notification.dispatched {outbox_id, topic, template_code, recipient_count, generic_for_suspicion}` (no user ids, no names).
10. A replay of the same outbox row writes no new notice (`dedupe_key`).

**The mapping** (topic or event; producer; template; recipients; the rule that matters):

| Topic (producer) | Source read | Template | Recipients at dispatch | Rule |
|-|-|-|-|-|
| `notify.request` (phase 04 effect `notify` and the reminders and escalations, section 4.5 item 5) | the task and the template code in the payload | `task.waiting` or `task.overdue` (count 1) | holders of the open task (phase 04 `Holders::eligible`); for an escalation, the escalation assignee | the producer names the code; this row is the task shape, and the topic has two more shapes (section 7+) |
| `case.event` (phase 17) | `casework.case_events` row by `(case_id, seq)` | `case.assigned` gives `task.waiting` to the assignee; `case.work_item_overdue` gives `task.overdue` to the work item's owner; any other kind gives no notice | the assignee or owner, if they can read the case | a notice never names the case |
| `case.decision` (phase 18) | the case and the decision type | `case.updated` | the case team (`case_acl`, `revoked_at IS NULL`) | the decision text is never read |
| `case.event` of kind `case.rfi_released` (phase 19, section 4.5 item 1) | the RFI | `branch.item_waiting` (count 1) | the liaison and the alternate of the RFI's branch, only when `recipient_kind = 'liaison'` | no notice for other recipient kinds |
| `rfi.overdue`, `rfi.reminder` (phase 19) | the RFI | `desk.deadline_near` (count per recipient) | users with role `RFI` and `M` | the branch sees its deadline in its list, not in a notice |
| `internal_report.overdue` (phase 19) | the internal report | `desk.internal_report_waiting` (count) | `M` | the reporter is never a recipient |
| `authority_request.overdue` (phase 19) | the request | `desk.deadline_near` | `M` | |
| `watch.triggered`, `watch.expired` (phase 19) | the watch and its case | `case.updated` | the case team | a closed case: the team only |
| `alarm.raised` (phase 19, 20, 21 and this phase) | the alarm | `liaison.report_overdue` for kind `liaison_report_overdue` (branch code, to `M`); `supervisor.alarm` (alarm kind) for every other kind | the supervisor of the kind (phase 19 section 3.10 table: `M`, `A2` of the pool for touch and work breaches, `IA` and `SEC` for chain alarms as the audience table says) | the kind is a code, never a text |
| `liaison_report.drafted` (this phase) | the report | `branch.monthly_due` (due date) | the liaison and the alternate of the branch | |
| `liaison_report.signed` (this phase) | the report | `liaison.report_received` (branch code) | `M` | |
| `liaison_report.returned` (this phase) | the report | `branch.monthly_returned` | the liaison and the alternate | the comment is never sent |
| `credential_reset.requested` (phase 03, section 4.5 item 6) | the reset | `security.reset_pending` (count of pending resets) | `SEC` (scope `security.ops`) | |

Topics not consumed here: `internal_report.recorded` and `authority_request.recorded` (phase 15 reads them), `authority_request.overdue` is consumed (table above); `hibernation.*`, `network.snapshot_requested` and `graph.link` belong to phases 19 and 15. **Several handlers per topic (lead review).** Phase 00's outbox row has one `dispatched_at` and its registry routes a topic to one `TopicHandler`, while `case.event` is already handled by phase 17 (counts) and `case.decision` by phase 15 (TR-15). Phase 00's registry therefore maps a topic to an ordered list of handlers (section 4.5 item 10): they run in registration order in the dispatch transaction, the row is marked dispatched only when every handler succeeded, and a failure retries the row for all of them, which is safe because every handler is idempotent (this dispatcher by `dedupe_key`, phase 17 by recomputing counts, phase 15 by its coalesce key). The dispatcher is appended after the earlier handlers of each topic. Two notices that earlier phases mention are not built and are Open doubts: the notice for `drift.opened` (phase 10 writes no outbox event for drift; Open doubts 14) and the notice for a new candidate rule (phase 07; Open doubts 14).

**The staff decision notice (M-66).** When the MLRO records that the decision on a staff member's internal report is made, the notice to the reporter is `staff.report_decision` with its default neutral text and no outcome. The template exists and is seeded; no route in phase 19 sends it, so the send path is Open doubts 8. The text says that the decision was recorded and that the confidentiality duty continues; the outcome sentence is the MLRO's configuration and is not built.

### 3.8.1 The in-app list, the count, the read mark and the preferences (`NotificationService`)

- **`NotificationService::list(ctx, cursor, unread_only)`** (`GET /me/notifications`): `user_id = caller` in SQL; newest first; each item renders the stored `template_version` from `admin.notification_template_versions` with the four params substituted, in both languages (`text: {ar, en}`). The client shows the language it needs.
- **`unread_count(ctx)`** (`GET /me/notifications/unread-count`): one count with `user_id = caller AND read_at IS NULL`.
- **`mark_read(ctx, id)`** (`POST /me/notifications/{id}/read`): `UPDATE ... SET read_at = now() WHERE id = $1 AND user_id = $2 AND read_at IS NULL RETURNING id`; no row: 404 `E_NOT_FOUND` (another user's id is indistinguishable from a missing one); audit `notification.read {notification_id}`.
- **Preferences** (`GET` and `PUT /me/notification-preferences`): `email_mode` in `immediate` or `digest`; default `digest` (no row). `PUT` needs `If-Match` (the prefs row version; 428 without it; 409 on mismatch when a row exists). Audit `notification.prefs_changed {user_id}`. In-app is always on: the body has no in-app field.

### 3.8.2 Real-time events (SSE; DR-214)

The notice reaches the open page through the phase 17 `CountsHub` (Valkey fan-out; the per-user channel), as the event `notification.new` with the payload `{notification_id}` only. The page refetches `GET /me/notifications` and the count (phase 17 pattern). No text, no template code and no count travels in the event. The stream route is phase 17's `GET /api/v1/events/counts` (API.md section 2 names `GET /api/v1/events`; Open doubts 7).

### 3.9 The branch counts (`BranchWindowService::neutral_counts`)

`neutral_counts(branch, period | open)`: items released in the period by neutral status (`awaiting_input`, `received`, `closed`), observations and confidential reports recorded (counts only). Each count is 0 or above the threshold, or it is `null` with `suppressed_cells` incremented (`report.small_number_threshold`, phase 21). The same function feeds `GET /branch/home`, the monthly draft and the monthly view. A count never reveals an item, a case or a party.

### 3.10 Cedar actions and the file `B0_branch.cedar`

Actions (the names of the route table of section 2): `branch.home_read`, `branch.item_list`, `branch.observation_create`, `branch.confidential_create` (resource `System`); `branch.item_read`, `branch.item_answer`, `branch.evidence_stage` (resource `BranchItem`); `branch.monthly_read`, `branch.monthly_sign` (resource `LiaisonReport`); `liaison_report.list`, `liaison_report.review` (resource `LiaisonReport`); `notification.list`, `notification.prefs_read`, `notification.prefs_write` (resource `System`, every human principal; rows filtered in SQL to the caller); `notification.mark_read` (resource `Notice`); `notification_template.read` (resource `System`).

Entities (appended to `schema.cedarschema` in the phase 03 form; `User` and `Branch` are phase 03's):

```cedar
entity BranchItem = { branch: Branch, tier: String, state: String };
entity LiaisonReport = { branch: Branch, status: String, signed_by?: User };
entity Notice = { owner: User };
```

Action declarations (the `appliesTo` of each action; the phase 21 idiom):

```cedar
action "branch.home_read", "branch.item_list", "branch.observation_create", "branch.confidential_create",
       "notification.list", "notification.prefs_read", "notification.prefs_write",
       "notification_template.read", "liaison_report.list" appliesTo { principal: User, resource: System, context: Ctx };
action "branch.item_read", "branch.item_answer", "branch.evidence_stage" appliesTo { principal: User, resource: BranchItem, context: Ctx };
action "branch.monthly_read", "branch.monthly_sign", "liaison_report.review" appliesTo { principal: User, resource: LiaisonReport, context: Ctx };
action "notification.mark_read" appliesTo { principal: User, resource: Notice, context: Ctx };
```

The file (`B0_branch.cedar`, copy-paste ready; one role per `@id`; the scope gates of API.md section 2 are the coarse gates, these policies are the fine gate):

```cedar
// B0_branch.cedar (phase 22). Scope gates: branch.window (L), case.decide (A2, M), self, admin.system (ADM).
// Step-up is not a condition here: the handler consumes StepUpAction branch.monthly.sign (phase 01 class signing).
// A branch item or report the caller may not read is a masked 404 (MASKED_KINDS gains BranchItem and LiaisonReport, section 4.5 item 7).

@id("branch.members") @role("L")
permit (principal, action in [Akr::Action::"branch.home_read", Akr::Action::"branch.item_list",
        Akr::Action::"branch.observation_create", Akr::Action::"branch.confidential_create"], resource)
when { principal.roles.contains("L") };

@id("notice.self_service") @role("self")
permit (principal, action in [Akr::Action::"notification.list", Akr::Action::"notification.prefs_read",
        Akr::Action::"notification.prefs_write"], resource)
when { principal.kind == "human" };

@id("branch.items_own_branch") @role("L")
permit (principal, action in [Akr::Action::"branch.item_read", Akr::Action::"branch.item_answer",
        Akr::Action::"branch.evidence_stage"], resource is Akr::BranchItem)
when { principal.roles.contains("L") && principal.branches.contains(resource.branch) };

@id("branch.items_suspicion_hidden") @deny("masked")
forbid (principal, action in [Akr::Action::"branch.item_read", Akr::Action::"branch.item_answer",
        Akr::Action::"branch.evidence_stage"], resource is Akr::BranchItem)
when { resource.tier == "suspicion" };

@id("branch.reports_own_branch") @role("L")
permit (principal, action in [Akr::Action::"branch.monthly_read", Akr::Action::"branch.monthly_sign"],
        resource is Akr::LiaisonReport)
when { principal.roles.contains("L") && principal.branches.contains(resource.branch) };

@id("liaison.review") @role("M")
permit (principal, action in [Akr::Action::"liaison_report.list", Akr::Action::"liaison_report.review"], resource)
when { principal.roles.contains("M") };

@id("liaison.self_review") @deny("self_approval")
forbid (principal, action == Akr::Action::"liaison_report.review", resource is Akr::LiaisonReport)
when { resource has signed_by && resource.signed_by == principal };

@id("notice.own") @role("self")
permit (principal, action == Akr::Action::"notification.mark_read", resource is Akr::Notice)
when { resource.owner == principal };

@id("template.read") @role("ADM")
permit (principal, action == Akr::Action::"notification_template.read", resource)
when { principal.roles.contains("ADM") };
```

Visibility predicates (SQL, the same rules as the Cedar attributes, so that lists, counts and details agree): **V-B1** (items): section 3.1 step 2. **V-B2** (monthly reports): `branch_id` of the caller's branch for `L`; every row for `M`. **V-B3** (observations and confidential reports): no branch reader; `M` reads through the register (phase 19). **V-B4** (notices): `user_id = caller`.

### 3.11 Errors and audit events

Errors: section 2 table. Audit events (one per mutation; payloads carry ids, codes, counts and hashes; never a name, a party's value, a note, a comment, a file name or a notice text):

| Event kind | On | Resource |
|-|-|-|
| `branch.item_answered` | an answer accepted (section 3.2 step 7) | RFI |
| `branch.evidence_staged` | a document staged for an item | RFI |
| `branch.observation_recorded` | an observation inserted | observation (internal report id) |
| `branch.observation_hybrid_alert` | the hybrid rule raised an alert | observation |
| `branch.confidential_recorded` | a confidential report received | internal report |
| `branch.monthly_drafted` | a monthly draft inserted | liaison report |
| `branch.monthly_signed` | a signature | liaison report |
| `branch.monthly_reviewed` | an acceptance | liaison report |
| `branch.monthly_returned` | a return | liaison report |
| `notification.dispatched` | an outbox row handled (section 3.8) | outbox row |
| `notification.read` | a notice marked read | notice |
| `notification.prefs_changed` | a preference change | user |
| `notification.email_sent` | a relay send accepted (section 3.8.3) | notice |
| `notification.email_failed` | a relay failure, with its code | notice |
| `notification.swept` | the R12 sweep deletes a batch | count only |
| `notification_template.changed` | a template change applied | template |

Phase 19 writes `internal_report.received` (via `receive`), `rfi.answer_recorded` (via `record_answers`), `alarm.raised` and the case chain events; phase 02 writes `evidence.staged`.

### 3.8.3 The e-mail relay (`SmtpRelaySender`, job `notification_email`, digest job `notification_digest`; DR-209)

- **Off by default.** `notify.email_enabled = false` (seeded). With it off, every notice is `email_state = none`. The job reads the setting at each run.
- **Immediate** (`notification_email`, payload `{notification_id}`, coalesce `notification_email:<id>`): load the notice; the recipient's address is `admin.users.email` (phase 01; personal; never logged). No address: `failed` with `address_missing`, no retry. Otherwise `MailSender::send(OutgoingMail { to, subject, text, html: none, attachments: none })`: subject `AKR-TMS` and the rendered notice text in the recipient's `locale` (the generic text only), and the link built from `AKR_PUBLIC_ORIGIN` (phase 01 environment variable). On success: `sent`, `email_sent_at`, audit `notification.email_sent`. On failure: `email_attempts + 1`, the error code from the closed list (`relay_unavailable`, `relay_refused`, `timeout`); after the job's `max_attempts` the state is `failed`; audit `notification.email_failed`.
- **Digest** (`notification_digest`, hourly, `schedule.notification_digest`): for each user with `email_mode = digest` and notices in `digest` state, one message "You have {n} new notices. Open the system: <link>" (the count is the only value), then every included notice moves to `sent` in one transaction per user.
- **`SmtpRelaySender`** (worker only): host and port from `notify.smtp_host` and `notify.smtp_port`; transport `starttls` or `tls` from `notify.smtp_tls` (no plain transport; the settings validator refuses any other value); sender from `notify.smtp_from`; connect timeout 10 s, total 20 s; no retry inside `send` (the job retries). Relay credentials, when the bank's relay requires them, are the OpenBao KV item `smtp/relay` (secret-str), read by `akr-worker` only, never a setting and never logged. The error mapping is closed: connection refused or DNS failure `relay_unavailable`; a 4xx or 5xx reply `relay_refused`; a timeout `timeout`.
- **No attachment, no link to a record**: the link is the system's entry page; the notice text is the template text; no name, no case, no reason.

### 3.12 Tipping-off review (what a person outside the access list can learn; the answer must be "nothing")

| Surface | What it shows | What a branch manager learns | What a liaison of another branch learns | What the MLRO learns |
|-|-|-|-|-|
| Every `/branch/*` route (B1 to B6) | a released branch request of the caller's branch, its neutral status, deadline, question keys; the own answers; the branch's own counts | every `/branch/*` route answers 404 (DR-206); the body and the timing equal those of an unknown route (the gate answers before any branch lookup) | a 404 identical to an unknown id for any item of another branch | (not a branch route) |
| An item that moves to suspicion | the item leaves the list; its detail is the masked 404 | nothing: the 404 is identical to a missing item | the same 404 | the case, in the case workspace |
| The observation receipt (B3) | "Received" and the acknowledgment time | nothing (no route) | nothing | the observation in the register with its groups and notes, through phase 19 |
| The confidential report receipt (B4) | "Received" | nothing (no route, the report is not listed) | nothing | the report in the register, the reporter's identity to M only (M-68) |
| The monthly report (B5) | counts by neutral status, suppressed cells, due date, status; the MLRO's return reads as the neutral sentence "The MLRO asked for a correction" | nothing (no route; L only) | nothing | counts, the signer, the review; R5 |
| The document check (B6) | the document requests of the caller's released items | nothing | nothing | the uploaded document (evidence owner `rfi_input`) |
| Evidence files | a neutral file name; the file is sealed (phase 02) | nothing | nothing | the file, through the case or the register |
| Notices (all templates) | fifteen closed texts; ids, counts, codes and dates in params; a suspicion-tier resource gives `task.waiting` | nothing: no branch manager is ever a recipient of a branch template, and no notice names a branch | a notice about a branch's item is never sent to a non-member | the desk notices of section 3.8 |
| SSE `notification.new` | the notice id on the recipient's channel | nothing | nothing | nothing beyond the id |
| E-mail | the generic text and the link; no attachment | nothing (sent only to the recipient's own address, only when the recipient passed the visibility filter) | nothing | nothing beyond the text |
| `GET /admin/notification-templates` | the template texts (no names) | nothing (`ADM` only) | nothing | nothing |
| Alarm notices (`supervisor.alarm`) | the alarm kind code | nothing | nothing | the kind; counts per kind on the cockpit (phase 19) |
| Staff decision notice (`staff.report_decision`) | the default neutral text, no outcome | nothing | nothing | the sending act is M's (Open doubts 8) |

Inference test (phase 17 rule, applied to every branch route): the difference between "the item does not exist", "the item is another branch's", "the item is suspicion tier" and "the caller is not a liaison" is nil in status, body, headers and order of checks that depend on the record.

### 3.13 Configuration and settings used

Settings (all in `0075`, copied to section 1.3): `branch.liaison_due_day` (10; governed; floor max 10), `branch.hybrid_window_days` (30; governed; floor 1), `branch.hybrid_scenario_code` (empty string: no hybrid alert; governed), `branch.item_closed_days` (30; governed; floor 1: closed items stay on the list this many days), `notify.email_enabled` (false), `notify.smtp_host`, `notify.smtp_port` (587; floor 1), `notify.smtp_tls` (`starttls`), `notify.smtp_from`, `notify.retention_days` (90; floor 30), `schedule.branch_monthly_draft` (`"0 30 0 1 * *"`), `schedule.notification_digest` (`"0 5 * * * *"`), `schedule.notification_sweep` (`"0 10 2 * * *"`).

Settings read from other phases: `report.small_number_threshold` (phase 21, governed, minimum 3), `casework.internal_report_ack_hours` (phase 19), `casework.rfi_claim_expiry_days` (phase 19), `evidence.max_bytes` (phase 02), `AKR_PUBLIC_ORIGIN` (phase 01 environment variable, not a settings row), `bank.timezone` (phase 00).

### 3.14 Lock order

1. The case row (phase 17 step 5 or phase 18), then the RFI row (phase 19 `record_answers`), the order phase 19 section 3.15 fixes. The branch answer (section 3.2) follows it.
2. The internal report row (phase 19 `receive` inserts; no lock held on another row).
3. The liaison monthly report row (sign, review, return, the timer handler).
4. The observation row (the hybrid job, after the case lock of step 1).
5. The template row (the change request apply hook; no other path takes it).
6. The outbox claim row (phase 00 `FOR UPDATE SKIP LOCKED`) is taken before any of the above by the dispatcher and released at commit. No path takes a lower-numbered lock after a higher-numbered one.

## 4. Infrastructure Updates

### 4.1 Job kinds (Postgres `jobs`; coalesce key in brackets)

| Kind | Binary | Payload | Concurrency, attempts | Schedule or trigger |
|-|-|-|-|-|
| `branch_monthly_draft` | worker | `{ period }` [`branch_monthly_draft:<period>`] | 1, 3 | `schedule.branch_monthly_draft` (first day of the month, 00:30 bank time); the period is the month before (section 3.5) |
| `timer_liaison_due` | worker | timer row (owner `liaison_monthly_report`, kind `liaison_due`) [report id] | 2, 3 | fired by the phase 00 timer sweep at `due_on` plus one day, 00:00 bank time (section 3.6) |
| `observation_hybrid` | worker | `{ observation_id }` [`observation_hybrid:<id>`] | 2, 5 | enqueued in the observation transaction (section 3.3) |
| `notification_email` | worker | `{ notification_id }` [`notification_email:<id>`] | 4, 5 (back-off) | enqueued by the dispatcher for an immediate recipient, in the dispatch transaction (section 3.8.3) |
| `notification_digest` | worker | `{}` [`notification_digest`] | 1, 3 | `schedule.notification_digest` (hourly) |
| `notification_sweep` | worker | `{}` [`notification_sweep`] | 1, 3 | `schedule.notification_sweep` (daily, 02:10 bank time); deletes notices older than `notify.retention_days`, read or unread, in batches of 500; audit `notification.swept` with the count |

Every handler re-reads its owner first. The release, the signature and the review are acts in the request transaction (akr-api) through the phase 19 and phase 04 services, as in phases 20 and 21. The prefix `ingest_` is not used (akr-ingestd only). The dispatcher is not a job: it is the outbox handler of section 3.8, run by the phase 00 `OutboxDispatcher` loop of akr-worker.

### 4.2 Timers (phase 00 `admin.timers`; the kind registry gains one kind)

| Timer kind | Owner kind | Fires at | Handler job | Action (after the re-read) |
|-|-|-|-|-|
| `liaison_due` | liaison_monthly_report | `due_on` plus one day, 00:00 `bank.timezone`; `calendar_id` = the bank calendar | `timer_liaison_due` | while the report is `draft`: alarm `liaison_report_overdue` (section 3.6); otherwise no-op |

Scheduled by the transaction that inserts the draft (section 3.5 step 1e); `cancel_for_owner` is not needed (the handler is a no-op after the signature). Observations use phase 19's `intake_clock` timer (owner `internal_report`), created by `InternalReportService::receive`; no timer kind is added for them.

**Outbox.** Topics produced here: `liaison_report.drafted {report_id}`, `liaison_report.signed {report_id, branch_id, period}`, `liaison_report.returned {report_id, branch_id}`. Each carries ids and a period only. The topic `notify.request` (phase 00 reserves `notify` for this phase; its `admin.outbox` CHECK requires a dotted topic, so the topic is `notify.request`) gets its handler: `NotificationDispatcher` (section 3.8). The other topics it consumes are listed in section 3.8 (eleven from earlier phases, one of them new in phase 03 by section 4.5 item 6).

### 4.3 AppState additions

- `akr-api`: `branch: Arc<BranchWindowService>`, `observations: Arc<ObservationService>`, `liaison_reports: Arc<LiaisonReportService>`, `notifications: Arc<NotificationService>`, `templates: Arc<TemplateService>` (the read of section 3.8 and the change-request target).
- `akr-worker`: `dispatcher: Arc<NotificationDispatcher>` (outbox handler for `notify.request` and the listed topics), `mail: Arc<dyn MailSender>` (the `SmtpRelaySender`), `hybrid: Arc<HybridObservationJob>`, `monthly: Arc<LiaisonReportService>` (the draft job and the timer handler), `digest: Arc<NotificationService>` (the digest and the sweep).
- Registries: `EffectRegistry` gains the terminal effect `NotifyEffect` (kind `notify`, all resource kinds; section 4.5 item 5); `ChangeTargetRegistry` gains `NotificationTemplateTarget`; the phase 02 owner registry gains `rfi_input`; the phase 03 `MASKED_KINDS` gains `BranchItem` and `LiaisonReport`.
- `akr-ctl`: `branch draft --period YYYY-MM` (a manual run of section 3.5 for one period, idempotent), `notify test --user <email>` (sends the generic test notice through the relay, audited; for the go-live check of section 6).

### 4.4 Network zones, metrics, readiness, OpenAPI

- **Zones.** `akr-worker` gains one outbound edge: the bank's SMTP relay host and port (`notify.smtp_host`, `notify.smtp_port`), and nothing else outside the cluster. `akr-api` reaches Valkey already (phase 17 `CountsHub`). No binary reaches the CBS: the branch window reads canonical copies only (phase 10), through the phase 19 and 03 tables.
- **Metrics.** `akr_branch_requests_total{route,outcome}` (outcome: `ok`, `not_found`, `error`), `akr_branch_drafts_total{outcome}` (`created`, `exists`, `failed`), `akr_notice_dispatch_total{topic,outcome}` (`dispatched`, `dropped_no_recipient`, `failed`), `akr_notice_email_total{outcome}` (`sent`, `failed`, `skipped_off`, `skipped_address`).
- **Readiness.** The worker's readiness includes the relay probe only when `notify.email_enabled` is true (a TCP connect to the relay; no mail is sent). With e-mail off, readiness is unchanged.
- **OpenAPI.** Tags `branch` (the nine `/branch/*` rows and the two `/liaison-reports` rows, `x-akr-access: branch` on the 404 gate of DR-206) and `notifications` (`/me/notifications*`, `/me/notification-preferences`, `/admin/notification-templates`). Schemas: section 2 utoipa list. `make generate-api` regenerates the client (section 5.4).

### 4.5 Existing entity, schema, route and code updates (extensions of earlier phases)

Each item is a call, a registration or a clause; no column of an earlier phase changes except the one named in item 6.

1. **Phase 19, `RfiService::release`.** The case event `case.rfi_released` (already written by phase 19's release, section 3.2 step 3) carries `{rfi_id, recipient_kind}` in its payload, so that the dispatcher can read the RFI by id. If phase 19's payload has no `rfi_id`, the release adds it (lead to confirm).
2. **Phase 19, `RfiService::record_answers`.** (a) The body is split into `record_answers_in_tx(tx, actor, rfi_id, answers, source_kind)`, called by the desk route after its `rfi.answer` check and by the branch route after `branch.item_answer`; the branch path always writes `corroboration = 'none'`. (b) Step 3's `corroborates` path (the desk raising a claim to `documented`) accepts an evidence object whose owner kind is `rfi_input` and whose owner id is the RFI itself (the branch's uploads), as well as an object of the case (owner kind `case`) (DR-201, lead review).
3. **Phase 02, evidence owner registry.** Register `rfi_input`: the owner is the RFI (checked by `BranchWindowService::stage_evidence` against the caller's branch before staging), retention class `R6`, classification `personal`.
4. **Phase 16 `DetectionPayload` and phase 17 `AlertIntakeService::emit`.** The payload gains `source_kind` with the values `scenario` (today) and `observation` (the hybrid rule of section 3.3.1). The `alert.alerts` CHECK already allows `observation` (phase 17 migration 0059). The observational scenario named by `branch.hybrid_scenario_code` is a register scenario with `observational = true` (phase 06, DR-382); the code is set by the bank (Open doubts 2). The step-by-step of phase 17 section 3.1 is unchanged.
5. **Phase 04.** (a) The terminal effect `notify` is registered as `NotifyEffect` (kind `notify`, resource kinds all) with params `{ "template_code": "<one of the closed codes of section 1.2>", "to": "initiator" | "subject_team" }` (the phase 04 text and DSL E.3 give no params shape; this phase defines it, Open doubts 12). It writes the outbox row `notify.request {workflow_instance_id, template_code, to}`; its `effect_deferred` event ends with the registration. (b) The phase 04 `remind` and `escalate` timeouts (section 3.6 of phase 04) write the outbox row `notify.request {task_id, template_code: "task.overdue" | "task.waiting", to: "holders" | "escalation_target"}` in their transaction. (c) `ChangeTargetRegistry` gains `notification_template` with route `cr_material` (section 3.8).
6. **Phase 03.** (a) `CredentialResetService::request` writes the outbox row `credential_reset.requested {reset_id}` in its transaction (the new topic; phase 03 section 3 says the notice arrives from phase 22). (b) `BranchService::update` refuses a user who is already liaison or alternate of another branch (422 `/liaison_user_id`, sub-code `already_liaison_elsewhere`; DR-205). (c) A read port `UserRoleQuery::holders(role, scope) -> Vec<UserId>` over the effective roles that phase 03 computes (direct and delegated), used by the dispatcher's recipient queries. No column of `admin.users` or `admin.branches` changes.
7. **Phase 03, `MASKED_KINDS`.** Gains `BranchItem` (the tier of its case and of its party, section 3.1) and `LiaisonReport` (the branch of the report). A read of a masked kind the caller may not see is the phase 03 masked 404.
8. **Phase 01.** The two `/me/notifications` rows of API.md 3.1 move to this phase (phase 01 section 2 already says so). `StepUpAction { name: "branch.monthly.sign", class: signing }` is declared in this phase, a constant of the closed class list.
9. **Phase 06, flag `branch.observations_live`.** No code of this phase writes the flag. ADM turns it on through the existing flag route (phase 03 route `PUT /api/v1/admin/flags/{key}`; phase 06 seeds the flag) after the bank records its branch go-live (Open doubts 9); the flag changes only the coverage grades of phase 06.
10. **Phase 00.** The outbox topic `notify.request` (phase 00 reserves the name `notify (22)`; its CHECK requires a dotted name) gets its handler. The `TopicHandler` registry becomes a list per topic (section 3.8, lead review): `register(topic, handler)` appends; `OutboxDispatcher` runs the list in order in one transaction and marks the row dispatched only when all succeed. The dispatcher is appended to `case.event` (after phase 17), `case.decision` (after phase 15) and every other topic of section 3.8.

## 5. Frontend Impact

Screens are the DESIGN-SYSTEM section 8.1 catalogue rows B1 to B6 (`/branch`, `/branch/items/:id`, `/branch/observe`, `/branch/report`, `/branch/monthly`, `/branch/check`), R5 (`/liaison`, M only) and the notification half of X3 (`/inbox`; the task half is phase 08). The browser entry for the branch window is a second Vite entry (`branch.html` with `src/branch/main.tsx`), built separately; the main bundle of phase 08 does not import it. The branch bundle is about 300 KB compressed and has no chart library, no graph library and no shared chart code (UX-02, DESIGN-SYSTEM section 8, B1 to B6 paragraph). A build check fails the build above 320 KB compressed for the branch entry (section 6, check 14).

### 5.1 B1 Branch home (`/branch`; INBOX; UX-66)

Header with the branch name, my role (liaison or alternate), the due date of the open monthly report, and the button "Report a concern" (B4), which is on every branch screen. The list of the branch's requests with the three neutral statuses and their due times (from `GET /branch/items`), then the counts of `GET /branch/home`. No reason codes, no outcomes, no AML words (DESIGN-SYSTEM section 11; the scanner of phase 09 runs on the branch's own form text, section 3.2 step 4).

### 5.2 B2 Branch review (`/branch/items/:id`; FORM; L-47, M-45)

The neutral question texts in order (`GET /branch/items/{id}`), each with the answer control allowed by its format (yes, no, unknown; a value; a free text; a document), the own answers already given, the document upload (`POST /branch/items/{id}/evidence`, with the file name replaced by the server), and the submit (`POST /branch/items/{id}/input`, `If-Match` = the item's version). A scan refusal shows the field and "This wording cannot be used here; please rephrase." (the span is highlighted, the term is never shown).

### 5.3 B3 Observation form (`/branch/observe`; FORM; UX-67, S-22)

The customer is chosen from the accounts of the branch (search by account number, limited to the branch: Open doubts 9). The eight groups are ticks with a notes field each; ticking group 8 ("Escalation checklist") shows the seven criteria as ticks. The companion identity field is optional. The written acknowledgment: the tipping-off reminder text bound to code `OBS_ACK_V1` is shown at once above the submit, with a required tick; the submit is refused without it. The receipt reads "Received". The form is one page and works on a slow link; it needs no chart.

### 5.4 B4 Confidential report (`/branch/report`; FORM)

The body (scanned), the optional subject party from the branch's accounts, the same `OBS_ACK_V1` acknowledgment, and the receipt "Received". No list of earlier reports exists on the branch side.

### 5.5 B5 Monthly liaison report (`/branch/monthly`; STEPPER; UX-66)

Three steps: the counts of the month by neutral status (suppressed counts read "fewer than 3"), the due date and the status (draft, signed, returned), and the sign step with step-up (`signing`, the step-up dialog of X1). A returned report shows the neutral sentence "The MLRO asked for a correction. Please review and sign again." and no comment text.

### 5.6 B6 Branch transactions and documents check (`/branch/check`; LIST)

The document requests of the caller's released items (from `GET /branch/items?kind=documents`), with their status. No transaction list and no balance: the branch window never reads a transaction (DR-215; Open doubts 6).

### 5.7 R5 Liaison report review (`/liaison`; LIST; M only)

A list of monthly reports by branch code and period with the due date, status and signature time (`GET /liaison-reports`). A report opens to its counts and `suppressed_cells`, with the two actions "Accept" (`POST /liaison-reports/{id}/review`, `comment` optional) and "Return for correction" (comment required, scanned). Step-up is not required for review.

### 5.8 X3 notifications (`/inbox`, notification half; LIST)

The bell in the header shows `GET /me/notifications/unread-count`; the list shows `GET /me/notifications` with the text of the caller's locale, the time and a read mark (`POST /me/notifications/{id}/read` when the notice is opened). The page refetches on `notification.new` (SSE, id only). The e-mail mode (immediate or digest) is a control in X4 (`/me`): `GET` and `PUT /me/notification-preferences`. The notice never links to a record: the link goes to the inbox.

### 5.9 Generated hooks (Orval names from the paths)

`useGetBranchHome`, `useGetBranchItems`, `useGetBranchItemsId`, `usePostBranchItemsIdEvidence`, `usePostBranchItemsIdInput`, `usePostBranchObservations`, `usePostBranchConfidentialReports`, `useGetBranchMonthlyReport`, `usePostBranchMonthlyReportSign`, `useGetLiaisonReports`, `usePostLiaisonReportsIdReview`, `useGetMeNotifications`, `usePostMeNotificationsIdRead`, `useGetMeNotificationsUnreadCount`, `useGetMeNotificationPreferences`, `usePutMeNotificationPreferences`, `useGetAdminNotificationTemplates`. The step-up header is set by the mutator (phase 08, `X-StepUp-Assertion`) on `usePostBranchMonthlyReportSign` only.

### 5.10 Browser and caching rules

No branch data in local storage, session storage or a service worker cache: a draft answer is saved by the server only on submit (no autosave in this phase). Every branch response carries `Cache-Control: no-store`. Every POST carries an `Idempotency-Key` (the client generates one per form submission and reuses it on a retry). The page title of the branch bundle is the neutral "Branch", never a case word.

## 6. Verification

1. `make migrate`: `0073_casework_branch_window.sql`, `0074_admin_notifications.sql` and `0075_branch_guards_grants.sql` apply cleanly, and a second run changes nothing (every table and index is `IF NOT EXISTS`; the two `casework.alarms` constraints, the triggers and the two functions are dropped by name before they are created). The seeds of `admin.notification_templates` and the settings insert with `ON CONFLICT DO NOTHING`.
2. `cargo check`: compile-time SQL passes for every query of sections 2 and 3, including the reads of `admin.branches`, `canon.account` and `canon.branch` (`org_branch_id`), `canon.staff`, `canon.account_party_role`, `casework.rfis`, `casework.rfi_questions`, `casework.rfi_answers`, `casework.cases`, `casework.case_acl`, `casework.case_events`, `admin.role_grants` (through `UserRoleQuery`), `alert.alerts` and `admin.evidence_objects`.
3. `cargo clippy -- -D warnings`.
4. `cargo test`, including these unit and property tests:
   - `BranchStatusMapper` is exhaustive: a compile-time test lists every RFI status and every internal report status and asserts the map; a grep test fails the build on a wildcard arm in `domain/branch/status.rs`.
   - The due-date rule: for `branch.liaison_due_day = 10` and the seeded calendar (weekend Friday and Saturday), the period October 2026 gives `due_on = 2026-10-11` (the 10th is a Saturday); a holiday on the 11th moves it to the 12th; `period_end = 2026-09-30` for the period `2026-09`; a period `2026-02` gives `period_end = 2026-02-28` in a non-leap year.
   - The scanner over the fifteen seeded templates: no hit in `notification` scope for either language; no digit outside a placeholder; every placeholder is listed in its `params` (the test reads the seed file and the migration SQL).
   - `NoticeParams` serialises only the four keys (a property test with every combination of `Some` and `None`).
   - The dispatcher mapping: for each topic of section 3.8 the template, the recipient rule and the suspicion rule (a suspicion resource gives `task.waiting`); an unknown topic is an error, not a silent drop.
   - The hybrid rule: no pattern alert gives no alert and sets `hybrid_checked_at`; an empty `branch.hybrid_scenario_code` gives no alert whatever the pattern; a replayed observation gives `Duplicate` and no second alert.
   - Counts: a value of 1 or 2 under `report.small_number_threshold = 3` becomes null and increments `suppressed_cells`; zero stays zero.
5. Curl suite `tests/curl/phase22_branch.sh` (dev users, one per role: `l@bank.test` (liaison of branch BR-01), `l2@bank.test` (alternate of BR-01), `lb@bank.test` (liaison of BR-02), `bm@bank.test` (branch manager of BR-01: a human user with `branch_id = BR-01` and no grant), `rfi@bank.test` (RFI desk), `m@bank.test` (M), `a2@bank.test` (A2 on the fixture case), `sec@bank.test` (SEC), `adm@bank.test` (ADM)). Fixture: a case of the party "Ahmed Kareem Hassan" (an invented name) with an account at BR-01, a released `branch_info` RFI addressed to `l@bank.test`'s branch, and a second such RFI for BR-02. Each step asserts the status and the body:
   - a. `GET /branch/home` as `bm@bank.test` returns 404 `E_NOT_FOUND`, body byte-identical to `GET /branch/nonexistent-route` as `bm@bank.test`; the same for `GET /branch/items` and `POST /branch/observations` (no body is read: the 404 is the gate).
   - b. `GET /branch/items` as `l@bank.test` lists the BR-01 item with status `awaiting_input`, no `case_id`, no `tier`, no `family`, no `reason_code` and no party name (the script greps the body for the party name, the case code and the words `suspicion`, `STR`, `UTR`, `case`: zero matches).
   - c. `GET /branch/items/{BR-02 item}` as `l@bank.test` returns 404, body identical to a random UUID's 404 (liaison of another branch; the masked 404).
   - d. `POST /branch/items/{id}/input` without `If-Match` returns 428 `E_PRECONDITION_REQUIRED`; with a wrong version 409 `E_VERSION_CONFLICT` (current version in the body); with the right version and a `value` containing a forbidden term 422 `E_VALIDATION` `forbidden_term` (the span, never the term); with a clean value 200 and the item status `awaiting_input` (until all required questions are answered).
   - e. Suspicion disappearance: M raises the case tier (a `_dev` fixture detection of a suspicion family through `POST /_dev/detections`, phase 17). Then `GET /branch/items` no longer lists the item, and `GET /branch/items/{id}` returns the same 404 as step c, byte-identical.
   - f. `POST /branch/items/{id}/evidence` by `l@bank.test` with a file named `Ahmed_Kareem_Hassan_statement.pdf` returns 201; the stored `file_name` in `admin.evidence_objects` is `document-<short id>-1` (the name never reaches the table; the test reads the row through the migrate role).
   - g. `POST /branch/observations` as `l@bank.test`: without the acknowledgment 422 `acknowledgment_required`; with a party of another branch 422 `E_VALIDATION` `party_not_found`; the same request with an unknown party returns the identical body; with `notes` containing a forbidden term 422 `forbidden_term` with the span; with `escalation_check` but no criteria 422 `escalation_checklist`; a valid request with the `Idempotency-Key` returns 201 with `status: received` and no other field; the same key and body replays the same response (no second `internal_reports` row); the same key with a changed body returns 422 `E_IDEMPOTENCY_REUSE`.
   - h. `POST /branch/confidential-reports` as `l2@bank.test` returns 201 `received`; `GET /internal-reports` as `m@bank.test` shows the report with `reporter_id` and `channel = branch_window`; as `a2@bank.test` the same register call returns 403 `E_FORBIDDEN` (the register is M's).
   - i. The monthly draft: run `branch_monthly_draft` for the previous period (`akr-ctl branch draft --period`); `GET /branch/monthly-report` as `l@bank.test` shows status `draft`, a due date, counts with null for any count below the threshold and `suppressed_cells` equal to their number; the same call as `bm@bank.test` is 404.
   - j. `POST /branch/monthly-report/sign` without `X-StepUp-Assertion` returns 401 `E_STEPUP_REQUIRED`; with a fresh assertion 200, status `signed`, and one new `internal_reports` row with `channel = liaison_monthly` (`GET /internal-reports` as `m@bank.test`); replaying the same assertion returns 401 `E_STEPUP_REQUIRED`; a second sign returns 409 `E_NOT_ALLOWED_IN_STATE` sub-code `already_signed`.
   - k. `POST /liaison-reports/{id}/review` as `a2@bank.test` returns 403 `E_FORBIDDEN`; as `m@bank.test` with `outcome: returned` and no comment returns 422 `comment_required`; with a comment, 200, status `draft`, `returned_at` set; the returned comment is absent from `GET /me/notifications` of `l@bank.test`.
   - l. Overdue alarm: fire `timer_liaison_due` for a draft whose due date passed (the `_dev` timer fire); `GET /alarms` as `m@bank.test` lists one `liaison_report_overdue` alarm with the subject id and no text; the alarm's outbox row produces one `liaison.report_overdue` notice for `m@bank.test` with the branch code and no other value.
   - m. Notices: `GET /me/notifications` as `l@bank.test` lists the drafted and returned notices with the rendered text in both languages; `GET /me/notifications/unread-count` returns the count; `POST /me/notifications/{id}/read` as `l2@bank.test` on `l@bank.test`'s notice returns 404; as the owner returns 200 and the count drops by one.
   - n. Notification text check: over every notice body and every e-mail text produced in the session, zero matches of the party name, the case code, the RFI id, any reason code, and the words of the forbidden list of phase 09 (`suspicion`, `suspicious`, `STR`, `UTR`, `money laundering`, `terrorist`, `freeze reason`, `sanction`); the check runs against `admin.notifications` through the migrate role and against the captured SMTP sink.
   - o. Replay: re-deliver one outbox row of `notify.request` (the `_dev` outbox replay); the count of `admin.notifications` with its `dedupe_key` stays at one.
   - p. E-mail: with `notify.email_enabled = true` and a test sink, a `digest` user receives one message per digest run with the count and the link only, and an `immediate` user receives one message per notice; with the relay stopped the notice is `failed` with `relay_unavailable` after the retries and the audit `notification.email_failed` is written; no relay credential appears in any log line (grep of the worker log).
   - q. Preferences: `GET /me/notification-preferences` returns `digest`; `PUT` without `If-Match` returns 428; with the version returns 200; a stale version returns 409 `E_VERSION_CONFLICT`.
   - r. Template change: a change request for `notification_template` `task.waiting` with a digit in the text is refused at validation (422 `E_VALIDATION`, sub-code `template_has_figures`); a clean change, approved by CHK and M, creates version 2 in `admin.notification_template_versions`, and an existing notice still renders version 1.
   - s. Cedar deny and masking: `ADM` on `POST /branch/items/{id}/input` returns 404 (no `L` role, no scope) and `sec@bank.test` on `GET /liaison-reports` returns 403 `E_SCOPE` (no `case.decide`).
6. Swagger shows the 17 routes of section 2 under the tags `branch` and `notifications`; `make generate-api` regenerates the client (section 5.9) and the hooks named there exist in `frontend/src/api/generated`.
7. The audit trail of the curl session contains the events of section 3.11 for each step that mutates (`branch.observation_recorded`, `branch.monthly_drafted`, `branch.monthly_signed`, `branch.monthly_returned`, `notification.dispatched`, `notification.read`, `notification_template.changed`), and no event carries a name, a note, a comment or a file name (a grep of `audit.audit_log` details for the fixture name returns none).
8. For phases that touch the CBS connection: not applicable; this phase reads no CBS table at run time.
9. Existing tests still pass, including the phase 19 RFI tests (the `record_answers` extension of section 4.5 item 2 keeps the owner-kind `case` path) and the phase 03 user and branch tests (section 4.5 item 6 changes).
10. Route-to-policy test (phase 08): every route of section 2 has a Cedar action of `B0_branch.cedar` or of the self rules, and every scope of section 2 admits every role the Cedar policy admits (the `case.decide` scope admits A2 and M, Cedar admits M).
11. A release-safety check for the 404 gate: `GET /api/v1/branch/home` as a user without `branch.window` returns the identical 404 for an existing and a non-existing branch id and for an unknown route under `/api/v1/branch/`.
12. Retention rows: `SELECT count(*) FROM admin.retention_assignment WHERE table_name IN (...)` returns six rows for this phase's tables, and every row of `admin.column_classification` for the six tables is present (the copy of the INSERT column lists is checked by the migrate test).
13. The monthly and notice jobs run once under two workers (the coalesce key and `SKIP LOCKED`): two parallel `branch_monthly_draft:<period>` enqueues produce one draft per branch.
14. The branch bundle (second Vite entry): the compressed script of `branch.html` is at most 320 KB (the 300 KB target plus margin), it imports nothing from the main entry except the shared design tokens, and `vite build` lists no chart or graph package in its chunks (grep).
15. The Arabic and English notice texts are reviewed by the bank's translator before go-live (Open doubts 20); the test of check 4 covers the forbidden-word list, not the translation.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 4a, from tools/agent/gap1-run4a.md; lead-verified (tools/agent/gap1-run4a-verdict.md).

### 7.1 Break-glass notice codes (phase 25)
From phase 25 on (migration `0082` seeds the two template rows), `NoticeCode` has seventeen codes: phase 25 adds
`security.break_glass_waiting` (to CH and SEC, parameter `count`) and `security.break_glass_opened` (to SEC and M,
parameter `count`). They use a new recipient rule of the dispatcher (section 3.8), "active holders of the role", read
at send time from phase 01 `GrantRepository::active_holders(role)`. Neither text names a case, party, requester or
reason, and neither links to the request.

Gap pass 1 (2026-10-10), run 4a: items carried from runs 3a and 3b and corrections found while checking phases 17 to 24 (tools/agent/gap1-run4a-verdict.md); lead-verified.

### 7.2 Observation fields in the internal-report register (DR-385)
This phase owns the read extension of Open doubts 9 (a): the rows of phase 19's `GET /internal-reports` whose internal
report has a `casework.observations` row (`internal_report_id`) carry `observation {indicator_groups,
escalation_criteria, notes}` for M only; `notes` is decrypted for M only, and other readers get no `observation` key.
Phase 19's route, scope and Cedar action do not change.

### 7.3 Drift and candidate notices (closes Open doubts 14)
Phase 07's line "No notifications (phase 22)" keeps a new candidate on the screens, the cockpit and an alert rule; it
promises no notice. Phase 10's open drift shows on T3. No drift or candidate topic is defined in this release; the
screens are the notice (phase 10 section 7+).

### 7.4 Flag route and the hybrid scenario (DR-382)
Section 4.5 item 9 reads the flag route as phase 03's (`PUT /api/v1/admin/flags/{key}`); phase 06 seeds the flag. The
hybrid scenario of section 4.5 item 4 and Open doubts 2 is a register scenario with `observational = true` (phase 06),
not a scenario of a family `observation`.

Gap pass 1 (2026-10-10), gap items (ac), (ad) and (ae) of tools/agent/gap-items.md (found in runs 7b and 8c); lead work, no Haiku draft.

### 7.5 The visit.recorded topic (TR-19; DR-405)
The branch RFI templates of the shipped pack carry the question key `visit_result` (answer format `yes_no_unknown`:
"the visit found the customer as declared"). When `RfiService::record_answers_in_tx` records `no` for `visit_result`
from a branch answer, the same transaction writes outbox `visit.recorded {party_id, rfi_id, result: "negative"}`. A
`yes` or `unknown` writes nothing. This phase registers phase 15's trigger handler for the topic (phase 15 section
7+). The topic is suspicion-linked (phase 15 section 3.6 rule 3), so the review carries `TRG_REVIEW_REQUESTED`, and
the branch view (section 3) shows nothing of it.

Adversarial pass (2026-10-10), run 5 (existence masking leaks), from tools/agent/adv-run5.md; lead-verified (tools/agent/adv-run5-verdict.md).

### 7.6 A released item that moves to suspicion reads as closed (ADV-5-07, HIGH; DR-411)
1. `RfiBranchRepository::released_items`: the clauses `c.tier <> 'suspicion'` and `casework.effective_tier(rfi.party_id)
   <> 'suspicion'` no longer remove an item. When either tier is `suspicion`, the item is listed with the neutral status
   `closed` (the word of any closed request), with no `due_at`: while the RFI is `released` or `answered`, and after phase
   19 closes it with its case (phase 19 section 7+), for `branch.item_closed_days` from `closed_at` like any closed item. `rfi.tier <> 'suspicion'`
   stays: an RFI created at suspicion tier is never listed. The other conditions (kind, recipient, branch) are unchanged.
2. `BranchStatusMapper` and `neutral_counts` follow step 1, so a count does not fall either. `GET /branch/items/{id}`
   returns the closed view; input and evidence on such an item answer 409 `E_NOT_ALLOWED_IN_STATE` sub-code
   `not_released`, as for a closed item.
3. The dispatcher (section 3.8 step 4) tests a liaison recipient with the open form of the predicate (the suspicion
   clauses kept), so no reminder or overdue notice reaches the liaison for an item shown as closed.
4. Section 3.12, row "An item that moves to suspicion": the item's own liaison sees it become `closed`, which a closed
   request also shows; test e of section 6 expects status `closed` and a 409 on input, and a liaison of another branch
   still gets the 404.

### 7.7 Who reads the internal-report register (Open doubts 27; ADV-5-04; DR-411)
Open doubts 27 is closed. Phase 19's register readers are A2 and M (phase 19 section 3.13, Cedar `followup.a2.report`):
they read the subject party and the body; the reporter's identity is M's only (M-68). A party named by a branch
observation or a confidential report is therefore known to the MLRO desk (A2 and M), never to branch staff or branch
management. The sentence "an M-only fact" in Open doubts 27 is replaced by this rule.

Adversarial pass (2026-10-10), run 6 (idempotency, audit chain and outbox replay), from tools/agent/adv-run6.md; lead-verified (tools/agent/adv-run6-verdict.md).

### 7.8 An e-mail is sent only for a queued notice (ADV-6-03, MEDIUM; DR-412)
Section 3.8.3 Immediate: `notification_email` loads the notice `FOR UPDATE` and sends only when `email_state =
'queued'`; any other state ends the job with no send. A re-run after the `sent` write committed therefore sends
nothing. The remaining window (the relay accepted the message and the state write failed) can send one more copy of the
generic text: delivery is at least once (DR-209 texts carry no data). The digest already selects notices in state
`digest` only and moves them to `sent` in one transaction per user.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.9 Alarm row of the liaison report (V-22-04-a, MEDIUM; DR-413)
Phase 19 section 7.3 has each owner add its row to the table of phase 19 section 3.10. This phase's row: `liaison_report_overdue` (no stage) | raised by the handler `timer_liaison_due` (section 3.6) and by a late return (section 3.7 step 3) | subject `liaison_report` | supervisor M | read by M.

### 7.10 The three payload shapes of `notify.request` (V-22-06-a, V-22-06-b, MEDIUM; DR-413)
The mapping row of section 3.8 describes one shape. The dispatcher tells the shapes apart by the keys of the payload.
1. `{task_id, template_code, to}` (phase 04 remind and escalate): the row as written. The code is `task.overdue` or `task.waiting`; `to` is `holders` or `escalation_target`.
2. `{workflow_instance_id, template_code, to}` (the `notify` effect, section 4.5 item 5 a): the source row is the instance and its resource, and the tier of step 3 is the resource's. `to: initiator` is the instance's initiator. `to: subject_team` is the case team (`casework.case_acl`, `revoked_at IS NULL`) when the resource kind is `case`, and the initiator alone for every other resource kind. The code is any closed code of section 1.2; steps 4 to 6 run as written, with `count` 1.
3. `{template_code, count}` (phase 25 section 3.3: `security.break_glass_waiting`, `security.break_glass_opened`): the notice names no resource, so steps 2 and 3 do not run and step 5 never swaps the template. The recipients are the active holders of the roles of 7.1 with no visibility filter, and `count` is the payload's value (step 6 does not run).
A payload of any other shape, or a code outside the closed list, is an error and the row stays undispatched (step 1).

### 7.11 `desk.approval_waiting` has no fixed producer (V-22-05-c, LOW; DR-413)
The seeded code `desk.approval_waiting` is sent by no step of this phase. It stays in the closed list as a code that a workflow definition's `notify` effect may name (payload shape 2 of the block above). An approver learns of an open deciding task from the phase 04 inbox and, where the step has a `remind` timeout, from `task.overdue` once the task is overdue (a step with an `escalate` timeout alone sends its approver no overdue notice); `task.waiting` goes to the escalation target only (phase 04 section 7.8).

### 7.12 Observation read for the register list (V-22-07-c, LOW; DR-413)
Section 2 `ObservationRepository` gains `by_internal_report(conn, internal_report_ids: &[Uuid]) -> Vec<ObservationView>`: one read of the observations whose `internal_report_id` is in the page of phase 19's register list, for the `observation` key of 7.2 (three fields: `indicator_groups`, `escalation_criteria`, `notes`). The key is set for M only. The method selects `notes_enc` as well and decrypts it through the phase 02 personal seal (AAD = the observation id); the register list of phase 19 calls it only when the caller holds M.

## Decisions

Numbered from DR-201 (the last number in the phase 23 decisions is DR-200). Each decision is one paragraph with its rationale. The lead adds them to `docs/plan/research.md`; this file does not edit it.

**DR-201 (a branch answer is a phase 19 claim; no `branch_inputs` table).** The branch's explanation, its documents on file and its visit result are answers of the RFI that the desk released to the liaison, recorded through `RfiService::record_answers_in_tx` with source kind `branch_officer` (lead correction below). Phase 19 already stores answers as claims with source, corroboration and expiry (L-47, M-45), so a second table would hold the same facts under a weaker rule. The branch's documents are owned by the RFI (owner kind `rfi_input`), because the branch has no case access; the one change this needs in phase 19 is named in section 4.5 item 2. DATA-MODEL section 16's `branch_inputs` is dropped and its columns map to the claim columns (section 1).

**DR-202 (confidential reports and observations are phase 19 internal reports).** The confidential report and the observation are both written by `InternalReportService::receive` with channel `branch_window`, so that the reporter's identity (visible to M only, M-68), the acknowledgment deadline, the timer and the suspicion-linked review are phase 19's and exist once. `casework.observations` holds the structure (groups, criteria, companion, the link to the register entry) and nothing the register does not hold. The table `confidential_reports` of DATA-MODEL section 16 is dropped: it would duplicate the register.

**DR-203 (the monthly report: one register entry per period, returns keep it).** The liaison signs the draft with step-up `signing`; the first signature writes one internal report with channel `liaison_monthly` whose sealed body is only the report id and the period. Phase 21's `liaison` pack counts exactly those entries, so the count is right without a change to phase 21. A return sends the report back to `draft` with the MLRO's comment, and a second signature reuses the entry: a second entry would count one month twice. The content is read by the MLRO on R5, from the report row, never from the register body.

**DR-204 (the neutral vocabulary is a closed, exhaustive map; M-66).** Only `awaiting_input`, `received` and `closed` reach the branch, and a state with no entry in the map is not listed at all. Both maps are `match` expressions without a wildcard arm, so a new RFI or internal report state fails the build until someone decides what the branch may see. Research file 28 section 13.2 gives the same three-way division ("Information requested from the branch", "Received by the central department", "Reviewed; no further action by the branch"), and the words are the bank's, from the glossary.

**DR-205 (a branch is identified by `admin.branches`; one branch per liaison).** A caller's branch is the `admin.branches` row where the caller is liaison or alternate (phase 03's own grants and the `L` role); a branch item's branch is the branch of the liaison it is addressed to (`rfi.recipient_ref`, a `canon.staff` row, through `canon.staff.branch_id` and `canon.branch.org_branch_id`), which phase 19 chose as a liaison of the subject's account branch (its section 3.1 step 4); a party with accounts at two branches does not make the item visible at the second. The session carries no branch. A user is liaison or alternate of one branch at most, enforced in phase 03 (section 4.5 item 6), so a branch view never shows two branches to one person and an item is never listed to two branches.

**DR-206 (the branch routes answer 404 to a caller without the scope).** API.md section 2 gates the `branch.window` scope with the usual 403 `E_SCOPE`. For the branch window that answer tells a branch manager that a branch window exists, which is itself a signal about the liaison's role. The branch routes therefore answer the same 404 as a missing record when the scope is absent, and the same 404 as a masked item when the item is not theirs. This is a deliberate exception to section 6 of API.md, marked on the routes (section 4.4) and listed in Open doubts 23.

**DR-207 (one dispatcher; recipients are computed at dispatch and filtered by visibility).** A notice is not addressed in advance: the dispatcher computes the recipients from the source row at the moment it runs (roles, branch membership, the case team, the desk predicate, the alarm audience), then keeps only the recipients who could read that source under the screens' own predicates. A stored recipient list would go stale when a grant changes and would disagree with the screens; a computed one cannot disagree with them. A replayed outbox row writes nothing new (`dedupe_key`).

**DR-208 (notices are closed texts with four parameters; a suspicion resource says only that work is waiting).** The fifteen template codes are an enum; their parameters are `count`, `due_date`, `branch_code` and `alarm_kind`, all ids, counts, codes or dates. A notice about a suspicion-tier resource is always `task.waiting`, so no notice says what kind of work it is. This is the rule of ARCHITECTURE section 15 and DESIGN-SYSTEM section 11, applied where the notice is built, not where it is displayed.

**DR-209 (e-mail is generic, relay-only, off by default, with a digest default).** The bank's SMTP relay is the only sender (ARCHITECTURE section 15, TS-21: no SMS, no push, no messaging app). The message has the generic text and the link to the entry page; it has no attachment and no record. The setting `notify.email_enabled` is false until the bank turns it on, the default per user is a digest, and the relay credential is an OpenBao item read by the worker only. The error codes stored are a closed list, never the relay's text.

**DR-210 (template text changes by change request, with append-only versions).** The wording of a notice is a bank decision with a legal reading (it is what the staff are told), so a change goes through phase 04's change request with CHK and M in parallel (`cr_material`), a validator that enforces the placeholder list and the scanner, and an append-only version. A notice keeps the version it was written with, so a later edit never changes what someone was told.

**DR-211 (the monthly due day, and the next working day when it falls on a closed day).** The due day is the setting `branch.liaison_due_day` (10), the next working day when it is closed (`BusinessClock::add` from the day before it, phase 09); the timer fires the next morning, once. A report signed late is accepted and counted by phase 21's pack, and the overdue alarm (M-53) stays until the supervisor acknowledges it, as every alarm of phase 19 does. The choice of the next working day rather than the previous one is the simpler reading of "by the 10th" and is listed in Open doubts 11.

**DR-212 (the hybrid alert is an observation alert built on an existing transaction alert).** S-23's hybrid alert is produced when an observation and an existing scenario alert of the same party fall within the window (both facts already exist in phases 16 and 17), through `AlertIntakeService::emit` with the source kind `observation`. The rule adds no transaction logic, so it cannot drift from the scenario engine. No S-24 scenario is written here (restructuring, splitting and repeated patterns are phase 16's, and the phase 16 text places S-22 to S-24 "with the branch window" only for the notification and the observation side); the hybrid rule is off until the bank names a scenario (`branch.hybrid_scenario_code`).

**DR-213 (the observation's groups are a closed list of eight codes; the acknowledgment is a code).** The circular's behavioural indicators (research file 01 section A, items 1 to 7) and its escalation criteria (item 8) are the eight codes, with the seven criteria as the checklist that group 8 requires. The written acknowledgment is the code `OBS_ACK_V1` with the wording in the glossary (X5), so that the wording can change without a schema change while the stored fact (that the reminder was shown and accepted, and when) stays exact.

**DR-214 (the real-time signal carries the notice id only, on the existing hub).** The bell and the page refetch when `notification.new {notification_id}` arrives on the phase 17 `CountsHub`. The hub is the one fan-out already in the system; a second hub (roadmap row 22 says `SseHub`) would be a second place where a user's channel is decided. The event carries no text, template or count, so a leaked event reveals only that a notice exists for the user.

**DR-215 (the branch window reads no transaction and no CBS table).** The branch window's data are the RFIs, the claims, the evidence of the branch's own uploads and its own counts. The document check (B6) is the list of documents the branch is asked for. A transaction list would be a CBS read from the branch's session and would show the branch's customers' activity to the branch, which the CBI rule that branch management has no part in the process does not allow.

**DR-216 (branch counts are suppressed below the threshold and computed over the reader's visibility).** The counts on B1, B5 and in the draft use phase 21's small-number rule, and a count is always computed by the same predicate as the list it summarises (section 3.9). A count that could be reconstructed from the suppressed list would reveal an item, so suppression applies to the drafts that leave the branch and to the counts the branch sees.

**DR-217 (retention of this phase's tables).** Observations follow their register entry (R6, owner clock), the monthly report is R6 from its creation (seven years), notices are R12 (swept after `notify.retention_days`, floor 30, as ARCHITECTURE section 17 says for operations), and templates, their versions and the preferences are R9 configuration. Notices are operations records, not evidence: the record of what was decided is in the case, the register and the report, not in the notice, which is why it is safe to sweep the notice.

**Lead corrections (review 2026-10-09).** A branch item is listed to the branch of the liaison it is addressed to (`rfi.recipient_ref`, a `canon.staff` row, through `canon.branch.org_branch_id`), not to every branch where the party holds an account; the draft's `canon.account.org_branch_id` does not exist (`canon.account.branch_id` joins `canon.branch`). Phase 00's outbox routes a topic to one handler and `case.event` and `case.decision` already have one (phases 17 and 15), so the registry becomes a list of handlers per topic (section 3.8, section 4.5 item 10). Branch claims are written with `corroboration = 'none'` and no `evidence_object_id` (phase 19's CHECKs; a claim corroborated by its own maker would pass the M-45 closure gate); the document ids sit in the sealed value and the desk raises the corroboration; the branch calls `record_answers_in_tx`, the phase 19 body without its `rfi.answer` check. An observation may name no customer (`party_id` nullable): S-22's counter observations are often about walk-ins and third parties. Confidential reports are not counted on any branch surface (the alternate would learn that the liaison reported). The due-day floor is `{"max": 10}`. This phase's service is `BranchWindowService` (phase 03 owns `BranchService`); the case event is `case.assigned`.

## Open doubts (for gap pass 1 and the bank)

1. **The `recipient_ref` of a branch RFI.** Phase 19 section 3.2 step 4 names an active `canon.staff` row with a liaison role as the `recipient_ref` of a `branch_info` RFI, and phase 19 Open doubts 4 says no liaison role code exists in `canon.staff.role`. Phase 03 names the liaison in `admin.branches.liaison_user_id`. This phase matches the item to the caller's branch through the addressed staff row (`canon.staff.branch_id` to `canon.branch.org_branch_id`, DR-205, lead review); the caller is identified through `admin.branches`. Until phase 19's liaison role code exists, the desk picks the staff row by hand; gap pass 1 decides whether phase 19 resolves the recipient from `admin.branches.liaison_user_id` and `admin.users.staff_id` (phase 15) instead.
2. **The hybrid scenario.** `branch.hybrid_scenario_code` ships empty, so S-23's hybrid half does not run. The bank names a register scenario with `observational = true` (phase 06, DR-382) and approves the `source_kind` extension of section 4.5 item 4. Not built here: S-24 (restructuring and splitting scenarios, phase 16).
3. **Eight groups.** Research file 01 section A has seven indicator groups and a separate list of seven escalation criteria (its instruction 8). The API (3.17) and UX-67 say "eight groups". This phase codes eight: seven indicator groups and `escalation_check`, which carries the criteria (DR-213). Compliance confirms the coding and the order of the criteria.
4. **The seven-criteria checklist inside every case (S-23, second half).** A checklist on the case workspace is a phase 18 screen. It is not built here, and the observation carries only the ticks it was given.
5. **Scanning the officer's own notes.** Sections 3.2 and 3.3 scan branch text with the phase 09 list (scope `branch_form`). A factual note that says "suspicious" is refused with its span. The bank decides whether notes that never reach the branch after submit are exempt. The default keeps the branch's vocabulary neutral.
6. **The content of B6.** UX-66 names "branch transactions and documents check" without a definition, and research file 28 section 13.2 does not describe it. This phase defines B6 as the document requests of released items (section 5.6) and no transaction list (DR-215). A transaction check on the branch side is a decision about CBS reads from the branch window, which the phase does not make.
7. **The stream path.** API.md section 2 names `GET /api/v1/events` for the SSE event `notification.new`; phase 17 serves `GET /api/v1/events/counts` on `CountsHub`. This phase reuses the hub and the phase 17 route (DR-214). The API owner decides whether one stream path is kept; the rename of phase 17's route is not made here.
8. **The staff decision notice.** M-66's neutral default text for a staff member who filed an internal report is seeded (`staff.report_decision`), but no phase 19 route records the MLRO's decision on a report and sends the notice. The route and its step-up class are for the lead to decide; until then the notice is not sent.
9. **Reading observations; party search; the go-live flag.** (a) The MLRO reads an observation's groups, criteria and notes in the rows of phase 19's `GET /internal-reports`, which section 7.2 of this phase extends with the observation fields for M only. (b) The party search of B3 is limited to the caller's branch accounts (section 3.3 step 3). (c) The flag `branch.observations_live` is turned on by ADM at the bank's recorded go-live (section 4.5 item 9); the sign-off is in the change log, not in this phase.
10. **A liaison assigned during the month.** The monthly draft is created by the job at the start of the next month for branches with a liaison at that time, or by `akr-ctl branch draft`. Phase 03's `BranchService::update` could call `ensure_open_draft` at assignment; it does not in this phase.
11. **The due day when it is a closed day.** DR-211 takes the next working day. The bank confirms that the CBI date is "by the 10th, or the next working day" and not the previous working day.
12. **The `notify` effect's params.** Phase 04 and DSL E.3 name the effect and its deferral but give no params. Section 4.5 item 5 defines `{template_code, to}`. The DSL owner confirms the shape and records it in DSL E.3.
13. **M-25 and the command centre.** Research file 28 M-25 (the monthly liaison report, overdue branches on the command centre) is not among the requirement IDs of MODULES M17. It is carried in the text only. The overdue view on the command centre is phase 16's (M16); this phase raises the alarm and sends the notices.
14. **Drift and candidate notices.** Phase 10 section 3.5 (drift notice to DS) and phase 07 line 967 (a new candidate shown to a person) promise notices from phase 22. Neither writes an outbox event. This phase defines no topic for them. Their owners add a topic (for example `drift.opened`) or the notices stay on the screens; the security reset of phase 03 is the one new topic defined here (section 4.5 item 6).
15. **Alarm closure after a late signature.** Phase 19 closes no alarm by itself. A liaison report signed after its due day leaves the overdue alarm open until the supervisor acknowledges it through phase 19's `POST /alarms/{id}/acknowledge`.
16. **One branch per liaison, and the alternate's grant.** DR-205 and section 4.5 item 6(b) refuse a second branch in phase 03. Phase 03 already requires an active `L` grant for the alternate; this phase relies on it.
17. **Relay, sender and digest operations.** The bank's SMTP relay host, port, TLS mode, sender address and whether it needs a login (OpenBao item `smtp/relay`) are the bank's. The network owner adds the one egress edge of section 4.4. The digest hour is fixed in the seed (hourly run) and the default e-mail is off.
18. **The small-number threshold for branch counts.** Section 3.9 reuses phase 21's `report.small_number_threshold` (governed, minimum 3). The bank may want a separate value for the branch.
19. **Retention periods.** Notices are R12 with 90 days (floor 30); the observation follows its register entry; the monthly report is R6 from its creation. The bank confirms the periods and whether the companion identity number (sealed, personal) follows the register entry or the case.
20. **Arabic and English notice texts.** The fifteen texts of section 1.2 are drafted for this phase. The Arabic wording needs the bank's translator, and a term new to the forbidden list needs the phase 09 `ar_term_unreviewed` review. The count labels avoid plural forms by design; full ICU plural forms, if the translator wants them, are a change by DR-210.
21. **The payload of `case.rfi_released`.** Section 4.5 item 1 needs `rfi_id` in phase 19's release event payload. The lead confirms that phase 19 writes it.
22. **The owner kind of a branch document.** Section 4.5 item 2 extends phase 19's step 4 so that a corroboration may name an `rfi_input` object of its own RFI (DR-201). This is the one change that lets a branch document corroborate a claim, so the lead confirms it against phase 19's rule that the owner kind is `case`.
23. **404 instead of 403 on the branch routes.** DR-206 departs from API.md section 6 (`E_SCOPE` 403). The API owner adds the marker `x-akr-access: branch` and the rule to section 6, or accepts the visible 403.
24. **Earlier planning records.** Roadmap row 22 lists seven tables (including `confidential_reports` and `branch_inputs`) and a `SseHub`; both are superseded by DR-201, DR-202 and DR-214 and by section 2. The roadmap, status and research files are reconciled by the lead; DR-201 to DR-217 are not yet in `research.md`.
25. **Migration numbers.** This phase uses 0073 to 0075, as the lead assigned them; phase 23 uses 0071 and 0072. The next free number after this phase is 0076 (phase 24 and later; the roadmap's own number budget is the lead's).
26. **The branch bundle entry.** Section 5 adds a second Vite entry `branch.html`. Phase 08's main bundle does not import it (phase 08 says "No branch window bundle (phase 22)"). The frontend owner confirms the entry and the 320 KB build check of section 6, check 14.
27. **Who sees a branch observation's party.** Phase 19 `internal_reports.subject_party_id` is read by the register readers. A party named by a branch observation is therefore an M-only fact in the register; the lead confirms that phase 19's register view hides the subject from every role but M.

## What This Phase Does NOT Touch

- No CBS connection and no CBS read: the branch window and the notices read canonical copies and phase 19 and 03 tables only (DR-215). No statement reaches the source from this phase's jobs.
- No change to phase 19's RFI, answer, release, internal report or alarm tables. The four changes of section 4.5 items 1 and 2 are calls and a payload field; the lead applies them in phase 19's text.
- No change to phase 17's alert tables, the scenario engine of phase 16, or phase 18's case tables, routes or decisions. The hybrid rule of section 3.3.1 calls `AlertIntakeService::emit` only.
- No change to phase 21's `liaison` periodic count, its tables or its routes; the monthly report is counted through the register entries it already reads.
- No change to phase 01's `admin.users` or sessions; the `/me/notifications` rows move here as phase 01 already says.
- No change to phase 03's tables, roles or Cedar files other than the `MASKED_KINDS` entries, the one-branch rule and the read port of section 4.5 items 6 and 7; `B0_branch.cedar` is the only new Cedar file.
- No change to phase 00's job, outbox or timer tables; the dispatcher uses the `notify.request` topic that phase 00's reservation names.
- No change to phase 08's main bundle and no chart library in the branch bundle.
- No outgoing SMS, WhatsApp, push or messenger channel (ARCHITECTURE section 15, TS-21); no customer contact of any kind: no customer notice exists in this phase.
- No new step-up class (the class `signing` is used as phase 01 declares it).
- No retention sweep of other tables and no legal hold (phase 23); the notice sweep of section 4.1 deletes only R12 notices past `notify.retention_days`.
- No AI assistance in the branch window or in the notices (P2, phases 27 and 28).
- No second copy of the forbidden-word list: the scanner and the list are phase 09's, and the notice texts are checked by them.
