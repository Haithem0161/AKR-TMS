# Phase 25: Platform Growth & Releases

**Goal:** The bank can run AKR-TMS as its own platform: it reviews who holds which role every quarter and revokes what is no longer needed, opens a four-hour break-glass access to one case with two approvers and a 24-hour review, forwards security events to its SIEM without any case content, signs in through its own identity provider, approves and applies signed release bundles itself, lets the vendor in only through a time-boxed session with no data permissions, and reads the release notes and help articles that ship with each bundle.

**Dependencies:** Phase 00 (jobs, settings, cron schedules, the outbox (the reserved topic `siem.export` is not used, section 3.5), the network zone model and `deploy/zones.yaml`, OpenBao paths and AppRoles, `akr-ctl bundle verify` and the signed release bundle), Phase 01 (`StepUpService` with the closed classes `break_glass`, `approval`, `access_change`, `export`; `admin.users` with `kind` and `expires_at`; `admin.role_grants` with `valid_to` (altered here: source `support_session`); `ClientCredentials` service principals, the first consumer being the SIEM pull API; the Kratos and BFF session settings), Phase 02 (the audit writer, `audit.audit_log` and its chain, the catalogue with the `siem` flag, `audit.audit_anchors`, `EvidenceStore::put_generated`, the evidence owner registry), Phase 03 (`AccessMatrixBuilder::build(at)`, `OpsCockpit::snapshot()`, `MASKED_KINDS`, `admin.purpose_codes` (`break_glass`), the `system` and `user` entities of the Cedar schema, the `self` and `system_admin` action groups, the Cedar range 50 with `50_system.cedar`), Phase 04 (`admin.change_requests`, `ChangeTargetRegistry`, routes `cr_access` and `cr_access_adm`, the `role_grant` change target, `admin.evidence_objects` through phase 02), Phase 17 (`services::visibility::build`, the Cedar `Case` entity with `assignees`, `case.read` and the suspicion forbids of `90_alerts.cedar`), Phase 18 (`casework.case_acl` and its rule-5 clause, the case write and read actions of `91_cases.cedar`, sealed-case refusals with `E_CASE_SEALED`), Phase 19 (`casework.authority_requests`, the M-37 disclosure fields), Phase 22 (`notify.request`, the `NotificationDispatcher`, the closed `NoticeCode` list and the notice templates), Phase 24 (`audit.resource_tier(TEXT, UUID, UUID)` as SECURITY DEFINER, `GET /access/control-report`, the `pack.*` routes, the `G5` counts, `akr_readonly` rules)

**Macro-phase:** P1
**Complexity:** XL by the template rule of 25 routes or more (26 routes: 12 rows of API.md 3.2, 3.3, 3.20 and the help row owned here, 14 rows added beyond API.md, four of them by the owner decisions of sections 7.23 and 7.24; 11 tables in 3 migrations (`0080` to `0082`); 1 altered table (`admin.role_grants`); 2 SECURITY DEFINER read functions in `audit` (`audit.siem_page`, and `audit.siem_access_page` of section 7.23); 8 new job kinds; no timer kind; no new outbox topic)
**Binaries touched:** akr-api (routers `access_reviews`, `break_glass`, `releases`, `support_sessions`, `help`, `siem_pull`, and `siem_admin` of section 7.24; `AccessReviewService`, `BreakGlassService`, `ReleaseService`, `SupportSessionService`, `HelpCentreService`, and `SiemDeadLetterService` of section 7.24, in `AppState`; the pull handler of `SiemForwarder` reads through `SiemRepository`) | akr-worker (jobs `access_review_snapshot`, `access_review_sweep`, `break_glass_sweep`, `siem_forward`, `support_session_sweep`, `support_bundle_purge`, `release_verify`, `anchor_timestamp`; `SiemForwarder` owns the push to the bank's SIEM) | akr-ctl (new subcommands `support-bundle`, `oidc upstream check` and `siem configure`, and `identity link-upstream` and `identity unlink-upstream` in the phase 01 `identity` group; no new binary) | akr-ingestd and akr-assess unchanged

**Requirements carried:**
- Implemented here: SP-24 (break-glass: reason code, two named approvers, never the requester, step-up class `break_glass`, four-hour limit enforced by the database, every use flagged, CISO and MLRO notices, no export, automatic end, review within 24 hours, dedicated log; section 3.2), SP-25 (the quarterly access review part: snapshot, confirm or revoke, sign-off with step-up, overdue warning; the policy-governance part is phase 03 and 04), SP-43 (CEF or JSON over TLS syslog and a signed pull API, the use-case catalogue as data, the phase 02 stripping rule; section 3.5), SP-44 (partly: the inspector accounts reuse the assessor role `AS` of phase 24 with a bank-approved time box (section 3.7); the RFC 3161 option on anchors, off by default), SP-52 (partly: the disclosure record of support bundles only; section 3.8), SP-64 (bank-opened, time-boxed vendor sessions, the `VS` role with no data permissions, recorded, ended automatically; section 3.8), SP-65 (`akr-ctl support-bundle`: allow-list redaction, per-bundle pseudonym key, manifest confirmed by the bank, signature, size cap, deletion after 30 days), SP-66 (signed bundles only, release notes, SBOM diff, rollback plan, the bank's four-eyes approval, the operator's apply record, no call-home; section 3.6), TS-25 (generic OIDC upstream as Kratos configuration, local MFA still enforced; section 3.4), TS-27 (SIEM export over syslog and the pull API; the tested redaction filter is phase 00), TS-28 (new ops tiles only, on `OpsCockpit::snapshot()`), TS-30 (release bundle import, verification, four-eyes approval and the operator's apply record; section 3.6), M1 (P1 generic OIDC upstream), M2 (quarterly access review), M3 (break-glass with two approvers and a four-hour limit), M20 (release bundle import and verification, support sessions, platform health tiles, help centre with "what changed")
- Partly implemented: SP-18 (the sign-in and step-up events the anomaly rules need are forwarded as kinds with the `siem` flag; the anomaly rules themselves are the SIEM rules of sections 7.13 and 7.23 (SP-36: by the owner decision of 2026-10-10 the access events are exported and the rule pack ships, except canary access (P2, DR-391) and the DBA and operator rules, which the bank feeds from pgAudit and its bastion)), SP-20 (the access-review snapshot reads the matrix of phase 03), SP-81 (not built here: the posture page is Open doubts 16), TS-34 (the single-tenant default is the installation model; the hosted cluster is phase 31 and Open doubts 17)
- Not carried here: SP-03, SP-07, SP-76 and the other vendor programme rows (outside the product), SP-89 (hosted model, Open doubts 17), SAML and SCIM (phase 31, TS-25 and SP-14), K3s, Helm and automated failover (phase 30, TS-29, TS-32), the general disclosure register of SP-52 beyond support bundles (Open doubts 9), the posture page of SP-81 (Open doubts 16)
- Gap pass 1 (coverage), partly implemented: SP-90 (signed release bundles verified on import; no call-home); VA-25 (signed release bundles with a recorded rollback plan), VA-38 (the support session opened by the bank), VA-47 (in-product help and the bilingual glossary), VA-56 and VO-08 (offline signed bundles for the platform); VB-52 (on-premises runtime with signed offline bundles), VD-20 (no call-home; offline signed bundles), VD-38 (no licence server or call-home).
- Gap pass 1 (coverage), not carried: SP-01, SP-67, SP-68, SP-77 (vendor security programme and contract, DR-396), SP-50 (pseudonymised tuning sets and vendor bundles, P2, DR-396); SP-30 (canary records, P2, DR-391); VA-01, VA-50 and VO-05 (escrow and continuity terms, DR-396); VA-20 and VO-06 (pricing); VA-34 (support policy); VO-14 (sanctions clause in the contract); VO-13 (release-gate note; the ROADMAP P1 exit criteria are the gate); DR-402; VB-26, VD-08, VD-09, VD-14, VD-15, VD-26, VD-28, VD-29, VD-30, VD-31, VD-35 and VD-37 (vendor, commercial and process items, DR-403); VC-27, VC-40, VC-46, VC-49, VC-50, VC-60 to VC-65, VC-76 and VC-84 (vendor, commercial, contract and procurement items, DR-404).
- Gap pass 1 (coverage), implemented here: UX-40 (contextual help, the bilingual glossary and what-changed notices in one help centre).

---

## 1. SQLx Migration Changes

Three migrations, `0080_admin_access_review_break_glass`, `0081_admin_releases_support_help` and `0082_admin_siem_guards_grants`, numbered after phase 24's `0079` (the lead's range 0080 to 0082; no other phase uses these numbers). The schema is `admin` for every new table: access reviews, break-glass grants, releases, support sessions and the SIEM state are governance and operations records, not case records, so no `casework` table is created. Eleven tables are created: `admin.access_reviews`, `admin.access_review_items`, `admin.break_glass_grants`, `admin.break_glass_log`, `admin.releases`, `admin.help_articles`, `admin.support_sessions`, `admin.support_bundles`, `admin.siem_cursors`, `admin.siem_dead_letters` and `admin.anchor_timestamps`. One table of phase 01 is altered (`admin.role_grants`, section 1.2). One SECURITY DEFINER function is added to `audit` (section 1.3). No table of `casework`, `alert`, `scn` or `canon` is altered, and `akr_readonly` receives no new grant.

**Reconciliation with the earlier phases (DR-232):**
- The break-glass grant is **not** a row of `casework.case_acl`. That table has a closed reason list, no expiry and refuses sealed cases (phase 18); a four-hour grant with its own approvers and log is a different record. The grant reaches the case through the team clause of phase 18 section 3.10 (section 3.2), not through the access list.
- The access-review snapshot is an evidence object of phase 02 (kind `snapshot`, owner kind `access_review`), so the matrix cannot drift after the review starts. Review items hold grant ids, not copies of grant rows.
- Release approval is a change request of phase 04 (route `cr_access`, section 3.6). Support-session approval is a route with step-up, not a change request, because it creates a role grant that phase 04 refuses by design (`role_not_grantable` for `VS`).

### 1.1 `0080_admin_access_review_break_glass.sql`

```sql
-- migration 0080_admin_access_review_break_glass.sql
-- Phase 25. Quarterly access reviews (SP-25, M2), break-glass grants and their dedicated log (SP-24, M3).
-- Ids are UUIDv7 from the application. Retention rows, classification, grants and guards: 0082.

-- One review of the role grants at one instant. The snapshot itself is an evidence object (phase 02, kind snapshot);
-- this row holds the state of the review and the hash of that object.
CREATE TABLE IF NOT EXISTS admin.access_reviews (
    id                  UUID PRIMARY KEY,
    status              TEXT NOT NULL CHECK (status IN ('building', 'open', 'signed', 'failed')),
    snapshot_at         TIMESTAMPTZ NOT NULL,                 -- the instant passed to AccessMatrixBuilder::build(at)
    evidence_id         UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    evidence_sha256     BYTEA CHECK (evidence_sha256 IS NULL OR octet_length(evidence_sha256) = 32),
    item_count          INT NOT NULL DEFAULT 0 CHECK (item_count >= 0),
    due_on              DATE NOT NULL,                        -- started date + access.review_interval_days
    started_by          UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    started_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    signed_by           UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    signed_at           TIMESTAMPTZ,
    closed_at           TIMESTAMPTZ,                          -- = signed_at; the retention clock
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    CHECK ((status = 'building') OR (evidence_id IS NOT NULL AND evidence_sha256 IS NOT NULL)),
    CHECK ((status = 'signed') = (signed_by IS NOT NULL)),
    CHECK ((signed_by IS NULL) = (signed_at IS NULL)),
    CHECK ((signed_at IS NULL) = (closed_at IS NULL)),
    CHECK (signed_by IS NULL OR signed_by <> started_by)   -- the signer is never the one who started the review
);
CREATE INDEX IF NOT EXISTS idx_access_reviews_due ON admin.access_reviews (due_on) WHERE status IN ('building', 'open');

-- One row per role grant in the snapshot. The decision is the reviewer's; the grant id is the live handle.
-- A grant that ends or changes before its decision is 'superseded' by the sweep, never silently confirmed.
CREATE TABLE IF NOT EXISTS admin.access_review_items (
    id                  UUID PRIMARY KEY,
    review_id           UUID NOT NULL REFERENCES admin.access_reviews(id) ON DELETE RESTRICT,
    grant_id            UUID NOT NULL REFERENCES admin.role_grants(id) ON DELETE RESTRICT,
    user_id             UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,   -- as at the snapshot
    role                TEXT NOT NULL CHECK (role IN ('A1', 'A2', 'SS', 'M', 'KYC', 'RFI', 'QA', 'OPS', 'CFG', 'CHK',
                                                     'DS', 'CH', 'MRO', 'IA', 'AS', 'L', 'ADM', 'SEC', 'VS')),
    decision            TEXT NOT NULL DEFAULT 'pending'
                        CHECK (decision IN ('pending', 'confirmed', 'revoke_proposed', 'flagged', 'superseded')),
    decision_note_code  TEXT CHECK (decision_note_code IS NULL OR decision_note_code IN
                                    ('still_needed', 'role_changed', 'left_post', 'other')),
    decided_by          UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    decided_at          TIMESTAMPTZ,
    change_request_id   UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    version             INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    UNIQUE (review_id, grant_id),
    CHECK ((decision IN ('pending', 'superseded')) OR (decided_by IS NOT NULL AND decided_at IS NOT NULL)),
    CHECK ((decision = 'revoke_proposed') = (change_request_id IS NOT NULL)),
    CHECK (decided_by IS NULL OR decided_by <> user_id)    -- nobody decides on a grant held by themselves
);
CREATE INDEX IF NOT EXISTS idx_access_review_items_review  ON admin.access_review_items (review_id, decision);
CREATE INDEX IF NOT EXISTS idx_access_review_items_grant   ON admin.access_review_items (grant_id);
CREATE INDEX IF NOT EXISTS idx_access_review_items_user    ON admin.access_review_items (user_id);
CREATE INDEX IF NOT EXISTS idx_access_review_items_cr      ON admin.access_review_items (change_request_id) WHERE change_request_id IS NOT NULL;

-- A break-glass grant: one case id, two approvers of different roles, a time box of at most four hours (DB floor; the
-- setting break_glass.max_hours may only be lower). Scope is a case only in this phase (party scope: Open doubts 1).
-- The case id has no foreign key on purpose: a request for an id that does not exist is recorded the same way as one for
-- a real case, so the response to the requester cannot tell the two apart (section 3.2, DR-233).
CREATE TABLE IF NOT EXISTS admin.break_glass_grants (
    id                    UUID PRIMARY KEY,
    case_id               UUID NOT NULL,                         -- deliberately no FK: an unknown id is accepted and never revealed (section 3.2)
    requester_id          UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    reason_code           TEXT NOT NULL CHECK (reason_code IN ('mlro_unavailable', 'case_unowned', 'regulator_deadline',
                                                              'stuck_work', 'other')),
    status                TEXT NOT NULL CHECK (status IN ('requested', 'active', 'ended', 'expired', 'lapsed')),
    requested_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    request_expires_at    TIMESTAMPTZ NOT NULL,                  -- requested_at + break_glass.request_ttl_min
    first_approver_id     UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    first_approver_role   TEXT CHECK (first_approver_role IN ('CH', 'SEC')),
    first_approved_at     TIMESTAMPTZ,
    second_approver_id    UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    second_approver_role  TEXT CHECK (second_approver_role IN ('CH', 'SEC')),
    second_approved_at    TIMESTAMPTZ,
    granted_at            TIMESTAMPTZ,                           -- the second approval
    expires_at            TIMESTAMPTZ,                           -- granted_at + at most four hours
    ended_at              TIMESTAMPTZ,
    ended_by              UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    reviewed_at           TIMESTAMPTZ,                           -- the retention clock (closure)
    reviewed_by           UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    review_outcome_code   TEXT CHECK (review_outcome_code IN ('appropriate', 'escalate', 'no_action')),
    version               INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    CHECK (expires_at IS NULL OR granted_at IS NOT NULL),
    CHECK (expires_at IS NULL OR expires_at <= granted_at + interval '4 hours'),
    CHECK (expires_at IS NULL OR expires_at > granted_at),
    CHECK (status <> 'active' OR (granted_at IS NOT NULL AND expires_at IS NOT NULL AND ended_at IS NULL)),
    CHECK (status NOT IN ('ended', 'expired') OR granted_at IS NOT NULL),
    CHECK (status NOT IN ('requested', 'lapsed') OR granted_at IS NULL),
    CHECK ((status IN ('ended', 'expired')) = (ended_at IS NOT NULL)),
    CHECK ((first_approver_id IS NULL) = (first_approver_role IS NULL AND first_approved_at IS NULL)),
    CHECK ((second_approver_id IS NULL) = (second_approver_role IS NULL AND second_approved_at IS NULL)),
    CHECK (first_approver_id IS NULL OR first_approver_id <> requester_id),
    CHECK (second_approver_id IS NULL OR second_approver_id <> requester_id),
    CHECK (second_approver_id IS NULL OR second_approver_id <> first_approver_id),
    CHECK (first_approver_role IS NULL OR second_approver_role IS NULL OR first_approver_role <> second_approver_role),
    CHECK (reviewed_at IS NULL OR status IN ('ended', 'expired')),
    CHECK ((reviewed_at IS NULL) = (reviewed_by IS NULL)),
    CHECK ((reviewed_at IS NULL) = (review_outcome_code IS NULL))
);
-- One open request or grant per requester and case (lead review: a per-case index let one requester hold the path shut
-- and told a second requester that a request was open).
CREATE UNIQUE INDEX IF NOT EXISTS uq_break_glass_open_case ON admin.break_glass_grants (case_id, requester_id)
    WHERE status IN ('requested', 'active');
CREATE INDEX IF NOT EXISTS idx_break_glass_active   ON admin.break_glass_grants (expires_at) WHERE status = 'active';
CREATE INDEX IF NOT EXISTS idx_break_glass_pending  ON admin.break_glass_grants (request_expires_at) WHERE status = 'requested';
CREATE INDEX IF NOT EXISTS idx_break_glass_review   ON admin.break_glass_grants (ended_at) WHERE reviewed_at IS NULL AND ended_at IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_break_glass_requester ON admin.break_glass_grants (requester_id, requested_at);

-- The dedicated break-glass log: every transition and every use, append-only (section 3.2). Details carry codes,
-- ids and counts only: never a case reference in a row that a SIEM or a notice may read.
CREATE TABLE IF NOT EXISTS admin.break_glass_log (
    id                UUID PRIMARY KEY,
    grant_id          UUID NOT NULL REFERENCES admin.break_glass_grants(id) ON DELETE RESTRICT,
    seq               INT NOT NULL CHECK (seq >= 1),
    kind              TEXT NOT NULL CHECK (kind IN ('requested', 'approved', 'activated', 'used', 'ended', 'expired',
                                                    'lapsed', 'reviewed', 'review_overdue')),
    actor_id          UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- NULL for the sweep
    detail            JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(detail) = 'object'),
    occurred_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (grant_id, seq)
);
CREATE INDEX IF NOT EXISTS idx_break_glass_log_actor ON admin.break_glass_log (actor_id, occurred_at) WHERE actor_id IS NOT NULL;

```

### 1.2 `0081_admin_releases_support_help.sql`

```sql
-- migration 0081_admin_releases_support_help.sql
-- Phase 25. Imported release bundles and their approval (SP-66, TS-30), the bilingual help articles shipped in a bundle
-- (M20, X5), support sessions for the vendor (SP-64) and the support-bundle register (SP-65, SP-52 part).
-- Ids are UUIDv7 from the application. The role_grants change at the end of this file is the only ALTER of phase 01.

-- An imported release bundle. The bundle directory is read from the bank's import volume by name; the row keeps the
-- name, never a path. Immutable once verified: the fields below change only through the status moves of section 3.6.
CREATE TABLE IF NOT EXISTS admin.releases (
    id                      UUID PRIMARY KEY,
    release_version         TEXT NOT NULL CHECK (release_version ~ '^[0-9]+\.[0-9]+\.[0-9]+(-[a-z0-9.]+)?$'),
    channel                 TEXT NOT NULL CHECK (channel IN ('stable', 'security')),
    bundle_ref              TEXT NOT NULL CHECK (bundle_ref ~ '^[A-Za-z0-9._-]{1,120}$'),   -- a directory name on the import volume
    manifest_sha256         BYTEA NOT NULL CHECK (octet_length(manifest_sha256) = 32),
    status                  TEXT NOT NULL CHECK (status IN ('imported', 'verifying', 'verified', 'refused', 'proposed',
                                                           'approved', 'applied', 'superseded')),
    refusal_code            TEXT CHECK (refusal_code IS NULL OR refusal_code IN (
                                'bundle_signature_invalid', 'image_signature_invalid', 'manifest_digest_mismatch',
                                'fingerprint_mismatch', 'sbom_missing', 'rollback_plan_missing', 'notes_missing',
                                'version_not_newer', 'policy_bundle_hash_mismatch', 'help_article_invalid')),
    key_fingerprint         TEXT CHECK (key_fingerprint IS NULL OR key_fingerprint ~ '^[0-9a-f]{64}$'),
    cosign_result           JSONB CHECK (cosign_result IS NULL OR jsonb_typeof(cosign_result) = 'object'),   -- counts and digests
    sbom_diff               JSONB CHECK (sbom_diff IS NULL OR jsonb_typeof(sbom_diff) = 'object'),           -- counts and names
    notes                   JSONB CHECK (notes IS NULL OR (jsonb_typeof(notes) = 'object' AND notes ? 'ar' AND notes ? 'en')),
    rollback_plan_sha256    BYTEA CHECK (rollback_plan_sha256 IS NULL OR octet_length(rollback_plan_sha256) = 32),
    policy_bundle_hash      BYTEA CHECK (policy_bundle_hash IS NULL OR octet_length(policy_bundle_hash) = 32),
    policy_matrix_diff      JSONB CHECK (policy_matrix_diff IS NULL OR jsonb_typeof(policy_matrix_diff) = 'object'),
    evidence_id             UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,   -- the verification report
    change_request_id       UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,    -- the approval (cr_access)
    imported_by             UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    imported_at             TIMESTAMPTZ NOT NULL DEFAULT now(),
    verified_at             TIMESTAMPTZ,
    applied_by              UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- the bank operator who applied it
    applied_at              TIMESTAMPTZ,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    version                 INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    UNIQUE (release_version, manifest_sha256),
    CHECK ((status = 'refused') = (refusal_code IS NOT NULL)),
    CHECK (status NOT IN ('verified', 'refused', 'proposed', 'approved', 'applied', 'superseded') OR verified_at IS NOT NULL),
    CHECK (status NOT IN ('verified', 'proposed', 'approved', 'applied', 'superseded') OR (cosign_result IS NOT NULL AND notes IS NOT NULL AND rollback_plan_sha256 IS NOT NULL AND policy_bundle_hash IS NOT NULL AND evidence_id IS NOT NULL)),
    CHECK (status NOT IN ('proposed', 'approved', 'applied') OR change_request_id IS NOT NULL),
    CHECK ((status = 'applied') = (applied_at IS NOT NULL)),
    CHECK ((applied_at IS NULL) = (applied_by IS NULL))
);
CREATE INDEX IF NOT EXISTS idx_releases_status   ON admin.releases (status, imported_at DESC);
CREATE INDEX IF NOT EXISTS idx_releases_applied  ON admin.releases (applied_at DESC) WHERE applied_at IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_releases_cr       ON admin.releases (change_request_id) WHERE change_request_id IS NOT NULL;

-- Help articles and "what changed" entries of a release, read from the signed bundle (X5). No article is fetched at run
-- time. Rows are written once by the import of their release and are never edited.
CREATE TABLE IF NOT EXISTS admin.help_articles (
    id              UUID PRIMARY KEY,
    release_id      UUID NOT NULL REFERENCES admin.releases(id) ON DELETE RESTRICT,
    code            TEXT NOT NULL CHECK (code ~ '^[a-z][a-z0-9_.-]{2,80}$'),
    kind            TEXT NOT NULL CHECK (kind IN ('article', 'glossary', 'what_changed')),
    title           JSONB NOT NULL CHECK (jsonb_typeof(title) = 'object' AND title ? 'ar' AND title ? 'en'),
    body            JSONB NOT NULL CHECK (jsonb_typeof(body) = 'object' AND body ? 'ar' AND body ? 'en'),
    sha256          BYTEA NOT NULL CHECK (octet_length(sha256) = 32),
    sort_order      INT NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (release_id, code)
);
CREATE INDEX IF NOT EXISTS idx_help_articles_kind ON admin.help_articles (kind, sort_order);

-- A vendor support session (SP-64): opened by the bank, approved by SEC with step-up, time-boxed, ended automatically.
-- The vendor's user holds the role VS only, for this session (role_grants.support_session_id, section 1.2).
CREATE TABLE IF NOT EXISTS admin.support_sessions (
    id                  UUID PRIMARY KEY,
    vendor_user_id      UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    requested_by        UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    approved_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    reason_code         TEXT NOT NULL CHECK (reason_code IN ('defect_reproduction', 'upgrade_assist', 'incident_bridge')),
    ticket_ref          TEXT NOT NULL CHECK (ticket_ref ~ '^[A-Z0-9-]{4,30}$'),   -- the bank's PAM or ticket reference
    requested_hours     INT NOT NULL CHECK (requested_hours BETWEEN 1 AND 8),    -- support.session_max_hours floor
    status              TEXT NOT NULL CHECK (status IN ('requested', 'active', 'ended', 'expired', 'refused_timeout')),
    requested_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    approved_at         TIMESTAMPTZ,
    starts_at           TIMESTAMPTZ,
    ends_at             TIMESTAMPTZ,
    ended_at            TIMESTAMPTZ,                           -- the retention clock (closure)
    ended_by            UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    version             INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    CHECK (approved_by IS NULL OR approved_by <> requested_by),
    CHECK ((approved_by IS NULL) = (approved_at IS NULL)),
    CHECK (ends_at IS NULL OR starts_at IS NOT NULL AND ends_at > starts_at),
    CHECK (ends_at IS NULL OR ends_at <= starts_at + make_interval(hours => requested_hours)),
    CHECK (status NOT IN ('active', 'ended', 'expired') OR approved_by IS NOT NULL),
    CHECK ((status IN ('ended', 'expired', 'refused_timeout')) = (ended_at IS NOT NULL)),
    CHECK (ended_by IS NULL OR ended_at IS NOT NULL)
);
-- A vendor has at most one open session (the VS role is exclusive, phase 03 sod.json).
CREATE UNIQUE INDEX IF NOT EXISTS uq_support_sessions_open ON admin.support_sessions (vendor_user_id)
    WHERE status IN ('requested', 'active');
CREATE INDEX IF NOT EXISTS idx_support_sessions_active ON admin.support_sessions (ends_at) WHERE status = 'active';

-- The disclosure record of one support bundle (SP-65, SP-52 part). The archive is a file on the bank's disk, built by
-- akr-ctl support-bundle; the row keeps its hash, size and confirmation. Deletion removes the archive file, not the row.
CREATE TABLE IF NOT EXISTS admin.support_bundles (
    id                      UUID PRIMARY KEY,
    session_id              UUID NOT NULL REFERENCES admin.support_sessions(id) ON DELETE RESTRICT,
    created_by              UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,   -- the bank operator who ran the CLI
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    manifest_sha256         BYTEA NOT NULL CHECK (octet_length(manifest_sha256) = 32),
    archive_sha256          BYTEA NOT NULL CHECK (octet_length(archive_sha256) = 32),
    signature               BYTEA NOT NULL CHECK (octet_length(signature) = 64),            -- Ed25519 over the manifest hash
    signing_key_version     INT NOT NULL CHECK (signing_key_version >= 1),
    pseudonym_key_ref       TEXT NOT NULL CHECK (length(pseudonym_key_ref) BETWEEN 1 AND 200),   -- OpenBao path, never the key
    size_bytes              BIGINT NOT NULL CHECK (size_bytes > 0 AND size_bytes <= 209715200),   -- hard cap 200 MiB
    confirmed_by            UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,    -- the operator who read the manifest
    confirmed_at            TIMESTAMPTZ NOT NULL,
    archive_deleted_at      TIMESTAMPTZ,
    status                  TEXT NOT NULL DEFAULT 'confirmed' CHECK (status IN ('confirmed', 'archive_deleted')),
    version                 INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    CHECK ((status = 'archive_deleted') = (archive_deleted_at IS NOT NULL))
);
CREATE INDEX IF NOT EXISTS idx_support_bundles_session ON admin.support_bundles (session_id, created_at);
CREATE INDEX IF NOT EXISTS idx_support_bundles_purge   ON admin.support_bundles (created_at) WHERE archive_deleted_at IS NULL;

-- The VS role and the session grant (phase 01 admin.role_grants is altered; DR-237). A support grant has source
-- 'support_session', names its session, has a valid_to, and is the only way a person holds VS (phase 04 refuses VS
-- on a change request with role_not_grantable).
ALTER TABLE admin.role_grants ADD COLUMN IF NOT EXISTS support_session_id UUID
    REFERENCES admin.support_sessions(id) ON DELETE RESTRICT;
ALTER TABLE admin.role_grants DROP CONSTRAINT IF EXISTS role_grants_source_check;
ALTER TABLE admin.role_grants ADD CONSTRAINT role_grants_source_check
    CHECK (source IN ('bootstrap', 'change_request', 'support_session'));
DO $$ BEGIN
    ALTER TABLE admin.role_grants ADD CONSTRAINT role_grants_support_session_check CHECK (
        (source = 'support_session') = (support_session_id IS NOT NULL)
        AND (source <> 'support_session' OR (valid_to IS NOT NULL AND role = 'VS'))
        AND (role <> 'VS' OR source = 'support_session')
    );
EXCEPTION WHEN duplicate_object THEN NULL; END $$;
CREATE INDEX IF NOT EXISTS idx_role_grants_support_session ON admin.role_grants (support_session_id) WHERE support_session_id IS NOT NULL;

```

### 1.3 `0082_admin_siem_guards_grants.sql`

```sql
-- migration 0082_admin_siem_guards_grants.sql
-- Phase 25. SIEM forwarder state and dead letters (SP-43), RFC 3161 time stamps of anchors (SP-44, off by default), the
-- SIEM read function, guards the application cannot bypass, grants per write path, settings, notice codes,
-- classification and retention. Idempotent: functions are replaced, triggers dropped before creation, inserts ON CONFLICT DO NOTHING.

-- The forwarder's position in the audit trail. One row per consumer; the cursor is the (occurred_at, id) of the last
-- audit row acknowledged by the collector. It moves only after the collector accepted the line.
CREATE TABLE IF NOT EXISTS admin.siem_cursors (
    consumer            TEXT PRIMARY KEY CHECK (consumer ~ '^[a-z][a-z0-9_]{2,40}$'),   -- 'cef_forwarder' in this phase
    last_occurred_at    TIMESTAMPTZ NOT NULL,
    last_audit_id       UUID NOT NULL,
    forwarded_total     BIGINT NOT NULL DEFAULT 0 CHECK (forwarded_total >= 0),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1 CHECK (version >= 1)
);

-- A line that the collector refused after siem.attempts_before_dead_letter tries, or that the formatter could not
-- render. It holds the audit id, the kind and a reason code, never the line itself. An operator asks for a resend or
-- marks it resolved with a code (section 7.24, owner decision 5).
CREATE TABLE IF NOT EXISTS admin.siem_dead_letters (
    id                  UUID PRIMARY KEY,
    stream              TEXT NOT NULL DEFAULT 'audit_log' CHECK (stream IN ('audit_log', 'access_event')),
    audit_id            UUID NOT NULL,                       -- the audit row id, or the access event id (section 7.23)
    occurred_at         TIMESTAMPTZ NOT NULL,
    action              TEXT NOT NULL CHECK (action ~ '^[a-z][a-z_]*(\.[a-z][a-z_]*)+$'),
    reason_code         TEXT NOT NULL CHECK (reason_code IN ('format_failed', 'unknown_kind', 'collector_refused', 'oversize')),
    attempts            INT NOT NULL CHECK (attempts >= 1),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    resend_requested_at TIMESTAMPTZ,
    resend_requested_by UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    resolved_at         TIMESTAMPTZ,
    resolved_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    resolution          TEXT CHECK (resolution IN ('resent', 'marked')),
    resolution_code     TEXT CHECK (resolution_code IN ('delivered_other_way', 'not_needed', 'cannot_be_sent')),
    version             INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    CHECK ((resolved_at IS NULL) = (resolved_by IS NULL)),
    CHECK ((resolved_at IS NULL) = (resolution IS NULL)),
    CHECK ((resend_requested_at IS NULL) = (resend_requested_by IS NULL)),
    CHECK (((resolution = 'marked') IS TRUE) = (resolution_code IS NOT NULL))   -- a code on a marked row only; an open row has neither
);
CREATE INDEX IF NOT EXISTS idx_siem_dead_letters_open ON admin.siem_dead_letters (created_at) WHERE resolved_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_siem_dead_letters_resend ON admin.siem_dead_letters (resend_requested_at)
    WHERE resend_requested_at IS NOT NULL AND resolved_at IS NULL;

-- RFC 3161 time stamp token of an anchor, requested by the anchor_timestamp job when audit.rfc3161_enabled is TRUE.
-- The anchor row itself stays immutable (phase 02 trigger); the token is a separate fact about it.
CREATE TABLE IF NOT EXISTS admin.anchor_timestamps (
    anchor_id           UUID PRIMARY KEY REFERENCES audit.audit_anchors(id) ON DELETE RESTRICT,
    tsa_name            TEXT NOT NULL CHECK (length(tsa_name) BETWEEN 1 AND 120),
    token               BYTEA NOT NULL CHECK (octet_length(token) BETWEEN 1 AND 65536),
    token_sha256        BYTEA NOT NULL CHECK (octet_length(token_sha256) = 32),
    requested_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    verified_at         TIMESTAMPTZ
);

-- SIEM read path (SP-43, SP-35). The forwarder and the pull API run as akr_app, which cannot read audit details or the
-- access log (phase 02 column grants). This function is the only read of audit rows for them: it returns the allow-listed
-- columns of the kinds named in p_kinds (the siem flag of the Rust catalogue), applies the phase 02 stripping rule to
-- every row whose resource tier is not normal, and never returns case_id or the tier value (lead review).
CREATE OR REPLACE FUNCTION audit.siem_page(p_after_at TIMESTAMPTZ, p_after_id UUID, p_limit INT, p_kinds TEXT[])
RETURNS TABLE (
    occurred_at     TIMESTAMPTZ,
    audit_id        UUID,
    action          TEXT,
    actor_id        UUID,
    on_behalf_of    UUID,
    resource_type   TEXT,
    resource_id     UUID,
    resource_key    TEXT,
    details         JSONB
)
LANGUAGE plpgsql STABLE SECURITY DEFINER SET search_path = pg_catalog AS $$
BEGIN
    IF p_limit IS NULL OR p_limit < 1 OR p_limit > 5000 THEN
        RAISE EXCEPTION 'siem page size out of range' USING ERRCODE = 'invalid_parameter_value';
    END IF;
    RETURN QUERY
    SELECT l.occurred_at, l.id, l.action, l.actor_id, l.on_behalf_of, l.resource_type,
           CASE WHEN t.tier = 'normal' THEN l.resource_id END,
           CASE WHEN t.tier = 'normal' THEN l.resource_key END,
           CASE WHEN t.tier = 'normal' THEN l.details ELSE '{}'::jsonb END
    FROM audit.audit_log l
    CROSS JOIN LATERAL (SELECT audit.resource_tier(l.resource_type, l.resource_id, l.case_id) AS tier) t
    WHERE (l.occurred_at, l.id) > (p_after_at, p_after_id)
      AND l.action = ANY (p_kinds)
    ORDER BY l.occurred_at, l.id
    LIMIT p_limit;
END $$;
ALTER FUNCTION audit.siem_page(TIMESTAMPTZ, UUID, INT, TEXT[]) OWNER TO akr_migrate;
REVOKE ALL ON FUNCTION audit.siem_page(TIMESTAMPTZ, UUID, INT, TEXT[]) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION audit.siem_page(TIMESTAMPTZ, UUID, INT, TEXT[]) TO akr_app;

-- Access lines for the SIEM (SP-36, owner decision 1). One row per access event after the cursor, in chain order.
-- A row is protected when its stored tier, the current tier of its resource or the current tier of its subject party
-- is not normal: then resource_id and subject_party are not returned. The tier value is never returned.
-- has_work: the actor holds a claimed alert on the subject party, or a live access-list grant on a case that names it.
-- is_staff: the subject party is a staff member or is marked as related to staff (phase 10).
CREATE OR REPLACE FUNCTION audit.siem_access_page(p_after_at TIMESTAMPTZ, p_after_id UUID, p_limit INT)
RETURNS TABLE (
    occurred_at     TIMESTAMPTZ,
    access_id       UUID,
    action          TEXT,
    decision        TEXT,
    actor_id        UUID,
    actor_role      TEXT,
    on_behalf_of    UUID,
    resource_type   TEXT,
    resource_id     UUID,
    subject_party   UUID,
    protected       BOOLEAN,
    purpose_code    TEXT,
    client_ip       INET,
    work_link       BOOLEAN,
    subject_staff   BOOLEAN
)
LANGUAGE plpgsql STABLE SECURITY DEFINER SET search_path = pg_catalog AS $$
BEGIN
    IF p_limit IS NULL OR p_limit < 1 OR p_limit > 5000 THEN
        RAISE EXCEPTION 'siem page size out of range' USING ERRCODE = 'invalid_parameter_value';
    END IF;
    RETURN QUERY
    SELECT e.occurred_at, e.id, e.action, e.decision, e.actor_id, e.actor_role, e.on_behalf_of, e.resource_type,
           CASE WHEN NOT t.is_protected THEN e.resource_id END,
           CASE WHEN NOT t.is_protected THEN e.subject_party END,
           t.is_protected, e.purpose_code, e.client_ip, w.has_work, s.is_staff
    FROM audit.access_event e
    CROSS JOIN LATERAL (
        SELECT (e.tier <> 'normal'
                OR audit.resource_tier(e.resource_type, e.resource_id, NULL) <> 'normal'
                OR (e.subject_party IS NOT NULL AND casework.effective_tier(e.subject_party) <> 'normal')) AS is_protected) t
    CROSS JOIN LATERAL (
        SELECT CASE WHEN e.subject_party IS NULL THEN NULL ELSE (
                   EXISTS (SELECT 1 FROM alert.alerts a
                            WHERE a.party_id = e.subject_party AND a.state = 'claimed' AND a.claimed_by = e.actor_id)
                OR EXISTS (SELECT 1 FROM casework.case_acl g
                             JOIN casework.case_parties cp ON cp.case_id = g.case_id
                            WHERE g.principal_id = e.actor_id AND g.revoked_at IS NULL
                              AND (g.expires_at IS NULL OR g.expires_at > e.occurred_at)
                              AND cp.party_id = e.subject_party)) END AS has_work) w
    CROSS JOIN LATERAL (
        SELECT CASE WHEN e.subject_party IS NULL THEN NULL ELSE COALESCE((
                   SELECT p.staff_flag OR COALESCE(p.related_to_staff, FALSE)
                     FROM canon.party p
                    WHERE p.id = e.subject_party AND p.recorded_to IS NULL
                    ORDER BY p.valid_from DESC LIMIT 1), FALSE) END AS is_staff) s
    WHERE (e.occurred_at, e.id) > (p_after_at, p_after_id)
    ORDER BY e.occurred_at, e.id
    LIMIT p_limit;
END $$;
ALTER FUNCTION audit.siem_access_page(TIMESTAMPTZ, UUID, INT) OWNER TO akr_migrate;
REVOKE ALL ON FUNCTION audit.siem_access_page(TIMESTAMPTZ, UUID, INT) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION audit.siem_access_page(TIMESTAMPTZ, UUID, INT) TO akr_app;

-- Guards. Break-glass log: append-only. Break-glass grants: identity fixed, status moves forward only, the time box
-- can only shrink, a reviewed grant is final. Access reviews: a signed review is final. Items: a decision is made once.
-- Releases: the bundle identity and the verification results are fixed once verified; the status moves forward only.
-- Support sessions: identity and time box fixed; status forward only. Support bundles: the record is fixed except the
-- send and deletion marks. Help articles: written once.
CREATE OR REPLACE FUNCTION admin.append_only_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION '% rows are append-only', TG_TABLE_NAME USING ERRCODE = 'insufficient_privilege';
END $$;
DROP TRIGGER IF EXISTS trg_break_glass_log_append_only ON admin.break_glass_log;
CREATE TRIGGER trg_break_glass_log_append_only BEFORE UPDATE OR DELETE ON admin.break_glass_log
    FOR EACH ROW EXECUTE FUNCTION admin.append_only_guard();
DROP TRIGGER IF EXISTS trg_help_articles_write_once ON admin.help_articles;
CREATE TRIGGER trg_help_articles_write_once BEFORE UPDATE OR DELETE ON admin.help_articles
    FOR EACH ROW EXECUTE FUNCTION admin.append_only_guard();

CREATE OR REPLACE FUNCTION admin.break_glass_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'break-glass grants are kept' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.id <> OLD.id OR NEW.case_id <> OLD.case_id OR NEW.requester_id <> OLD.requester_id
       OR NEW.requested_at <> OLD.requested_at OR NEW.reason_code <> OLD.reason_code THEN
        RAISE EXCEPTION 'a break-glass request is fixed' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.reviewed_at IS NOT NULL THEN
        RAISE EXCEPTION 'a reviewed break-glass grant is final' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.status IN ('ended', 'expired', 'lapsed') AND NEW.status <> OLD.status THEN
        RAISE EXCEPTION 'a closed break-glass grant does not reopen' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.status = 'requested' AND NEW.status NOT IN ('requested', 'active', 'lapsed') THEN
        RAISE EXCEPTION 'illegal break-glass move' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.status = 'active' AND NEW.status NOT IN ('active', 'ended', 'expired') THEN
        RAISE EXCEPTION 'illegal break-glass move' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.expires_at IS NOT NULL AND OLD.expires_at IS NOT NULL AND NEW.expires_at > OLD.expires_at THEN
        RAISE EXCEPTION 'a break-glass time box can only shrink' USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_break_glass_guard ON admin.break_glass_grants;
CREATE TRIGGER trg_break_glass_guard BEFORE UPDATE OR DELETE ON admin.break_glass_grants
    FOR EACH ROW EXECUTE FUNCTION admin.break_glass_guard();

CREATE OR REPLACE FUNCTION admin.access_review_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'access reviews are kept' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF TG_TABLE_NAME = 'access_reviews' THEN
        IF OLD.status = 'signed' THEN
            RAISE EXCEPTION 'a signed access review is final' USING ERRCODE = 'insufficient_privilege';
        END IF;
        IF NEW.id <> OLD.id OR NEW.snapshot_at <> OLD.snapshot_at OR NEW.started_by <> OLD.started_by THEN
            RAISE EXCEPTION 'an access review snapshot is fixed' USING ERRCODE = 'insufficient_privilege';
        END IF;
    ELSE
        IF OLD.decision <> 'pending' AND OLD.decision <> 'flagged' THEN
            RAISE EXCEPTION 'a decided access review item is final' USING ERRCODE = 'insufficient_privilege';
        END IF;
        IF NEW.grant_id <> OLD.grant_id OR NEW.user_id <> OLD.user_id OR NEW.review_id <> OLD.review_id THEN
            RAISE EXCEPTION 'an access review item is fixed' USING ERRCODE = 'insufficient_privilege';
        END IF;
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_access_reviews_guard ON admin.access_reviews;
CREATE TRIGGER trg_access_reviews_guard BEFORE UPDATE OR DELETE ON admin.access_reviews
    FOR EACH ROW EXECUTE FUNCTION admin.access_review_guard();
DROP TRIGGER IF EXISTS trg_access_review_items_guard ON admin.access_review_items;
CREATE TRIGGER trg_access_review_items_guard BEFORE UPDATE OR DELETE ON admin.access_review_items
    FOR EACH ROW EXECUTE FUNCTION admin.access_review_guard();

CREATE OR REPLACE FUNCTION admin.release_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'release records are kept' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.id <> OLD.id OR NEW.release_version <> OLD.release_version OR NEW.manifest_sha256 <> OLD.manifest_sha256
       OR NEW.bundle_ref <> OLD.bundle_ref OR NEW.imported_by <> OLD.imported_by THEN
        RAISE EXCEPTION 'a release identity is fixed' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.status IN ('verified', 'refused', 'proposed', 'approved', 'applied', 'superseded') THEN
        IF NEW.cosign_result IS DISTINCT FROM OLD.cosign_result OR NEW.sbom_diff IS DISTINCT FROM OLD.sbom_diff
           OR NEW.notes IS DISTINCT FROM OLD.notes OR NEW.policy_bundle_hash IS DISTINCT FROM OLD.policy_bundle_hash
           OR NEW.rollback_plan_sha256 IS DISTINCT FROM OLD.rollback_plan_sha256 THEN
            RAISE EXCEPTION 'verified release evidence is fixed' USING ERRCODE = 'insufficient_privilege';
        END IF;
    END IF;
    IF OLD.status = 'applied' AND NEW.status <> 'applied' AND NEW.status <> 'superseded' THEN
        RAISE EXCEPTION 'an applied release moves only to superseded' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.status = 'refused' AND NEW.status <> 'refused' THEN
        RAISE EXCEPTION 'a refused release does not reopen; import it again' USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_releases_guard ON admin.releases;
CREATE TRIGGER trg_releases_guard BEFORE UPDATE OR DELETE ON admin.releases
    FOR EACH ROW EXECUTE FUNCTION admin.release_guard();

CREATE OR REPLACE FUNCTION admin.support_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'support records are kept' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF TG_TABLE_NAME = 'support_sessions' THEN
        IF NEW.id <> OLD.id OR NEW.vendor_user_id <> OLD.vendor_user_id OR NEW.requested_by <> OLD.requested_by
           OR NEW.reason_code <> OLD.reason_code OR NEW.ticket_ref <> OLD.ticket_ref
           OR NEW.requested_hours <> OLD.requested_hours THEN
            RAISE EXCEPTION 'a support request is fixed' USING ERRCODE = 'insufficient_privilege';
        END IF;
        IF OLD.status IN ('ended', 'expired', 'refused_timeout') AND NEW.status <> OLD.status THEN
            RAISE EXCEPTION 'a closed support session does not reopen' USING ERRCODE = 'insufficient_privilege';
        END IF;
    ELSE
        IF NEW.id <> OLD.id OR NEW.session_id <> OLD.session_id OR NEW.manifest_sha256 <> OLD.manifest_sha256
           OR NEW.archive_sha256 <> OLD.archive_sha256 OR NEW.signature <> OLD.signature
           OR NEW.size_bytes <> OLD.size_bytes OR NEW.confirmed_by <> OLD.confirmed_by THEN
            RAISE EXCEPTION 'a support bundle record is fixed' USING ERRCODE = 'insufficient_privilege';
        END IF;
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_support_sessions_guard ON admin.support_sessions;
CREATE TRIGGER trg_support_sessions_guard BEFORE UPDATE OR DELETE ON admin.support_sessions
    FOR EACH ROW EXECUTE FUNCTION admin.support_guard();
DROP TRIGGER IF EXISTS trg_support_bundles_guard ON admin.support_bundles;
CREATE TRIGGER trg_support_bundles_guard BEFORE UPDATE OR DELETE ON admin.support_bundles
    FOR EACH ROW EXECUTE FUNCTION admin.support_guard();

-- Grants per write path (akr_app is the API and worker role). No grant to akr_readonly, akr_ingest or akr_audit_writer.
GRANT SELECT, INSERT ON admin.access_reviews TO akr_app;
GRANT UPDATE (status, evidence_id, evidence_sha256, item_count, signed_by, signed_at, closed_at, updated_at, version)
    ON admin.access_reviews TO akr_app;
GRANT SELECT, INSERT ON admin.access_review_items TO akr_app;
GRANT UPDATE (decision, decision_note_code, decided_by, decided_at, change_request_id, version)
    ON admin.access_review_items TO akr_app;
GRANT SELECT, INSERT ON admin.break_glass_grants TO akr_app;
GRANT UPDATE (status, first_approver_id, first_approver_role, first_approved_at, second_approver_id,
              second_approver_role, second_approved_at, granted_at, expires_at, ended_at, ended_by, reviewed_at,
              reviewed_by, review_outcome_code, version)
    ON admin.break_glass_grants TO akr_app;
GRANT SELECT, INSERT ON admin.break_glass_log TO akr_app;
GRANT SELECT, INSERT ON admin.releases TO akr_app;
GRANT UPDATE (status, refusal_code, key_fingerprint, cosign_result, sbom_diff, notes, rollback_plan_sha256,
              policy_bundle_hash, policy_matrix_diff, evidence_id, verified_at, change_request_id, applied_by,
              applied_at, version)
    ON admin.releases TO akr_app;
GRANT SELECT, INSERT ON admin.help_articles TO akr_app;
GRANT SELECT, INSERT ON admin.support_sessions TO akr_app;
GRANT UPDATE (status, approved_by, approved_at, starts_at, ends_at, ended_at, ended_by, version)
    ON admin.support_sessions TO akr_app;
GRANT SELECT, INSERT ON admin.support_bundles TO akr_app;
GRANT UPDATE (status, archive_deleted_at, version) ON admin.support_bundles TO akr_app;
GRANT SELECT, INSERT, UPDATE (last_occurred_at, last_audit_id, forwarded_total, updated_at, version)
    ON admin.siem_cursors TO akr_app;
GRANT SELECT, INSERT ON admin.siem_dead_letters TO akr_app;
GRANT UPDATE (resend_requested_at, resend_requested_by, resolved_at, resolved_by, resolution, resolution_code, reason_code, attempts,
              version) ON admin.siem_dead_letters TO akr_app;
GRANT SELECT, INSERT ON admin.anchor_timestamps TO akr_app;
GRANT UPDATE (verified_at) ON admin.anchor_timestamps TO akr_app;

-- Notice codes (phase 22 closed list, extended by two codes; DR-243). Counts only, no case, party or requester name.
INSERT INTO admin.notification_templates (code, body, params) VALUES
 ('security.break_glass_waiting', '{"ar": "طلبات وصول طارئ بانتظار الموافقة: {count}.", "en": "Emergency access requests waiting for approval: {count}."}', '{count}'),
 ('security.break_glass_opened', '{"ar": "وصول طارئ نشط ينتهي خلال أربع ساعات كحد أقصى. عدد الطلبات: {count}.", "en": "An emergency access grant is active for at most four hours. Grants: {count}."}', '{count}')
ON CONFLICT (code) DO NOTHING;

-- Settings (copy of the phase 24 insert format). floor only when governed = TRUE (phase 00 CHECK).
INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('access.review_interval_days', '92', 'integer', '{"max": 92, "basis": "SP-25 quarterly review; a bank may review more often, never less"}'::jsonb, TRUE,
  '{"ar": "فترة مراجعة صلاحيات الوصول (أيام)", "en": "Access review interval (days)"}'),
 ('break_glass.max_hours', '4', 'integer', '{"max": 4, "basis": "SP-24 time box of four hours"}'::jsonb, TRUE,
  '{"ar": "أقصى مدة للوصول الطارئ (ساعات)", "en": "Longest break-glass grant (hours)"}'),
 ('break_glass.review_hours', '24', 'integer', '{"max": 24, "basis": "SP-24 review within 24 hours"}'::jsonb, TRUE,
  '{"ar": "مهلة مراجعة الوصول الطارئ (ساعات)", "en": "Break-glass review deadline (hours after end)"}'),
 ('break_glass.request_ttl_min', '60', 'integer', NULL, FALSE,
  '{"ar": "مدة صلاحية طلب الوصول الطارئ قبل إغلاقه (دقائق)", "en": "Break-glass request lifetime before it lapses (minutes)"}'),
 ('support.session_max_hours', '8', 'integer', '{"max": 8, "basis": "SP-64 time-boxed vendor session"}'::jsonb, TRUE,
  '{"ar": "أقصى مدة لجلسة الدعم (ساعات)", "en": "Longest vendor support session (hours)"}'),
 ('support.bundle_retention_days', '30', 'integer', '{"max": 30, "basis": "SP-65 archives deleted after 30 days"}'::jsonb, TRUE,
  '{"ar": "مدة الاحتفاظ بأرشيف الدعم (أيام)", "en": "Support archive kept for (days)"}'),
 ('support.bundle_max_mb', '200', 'integer', NULL, FALSE,
  '{"ar": "أقصى حجم لأرشيف الدعم (ميغابايت)", "en": "Largest support archive (MiB)"}'),
 ('siem.batch_size', '500', 'integer', NULL, FALSE,
  '{"ar": "عدد الأحداث في كل دفعة إلى نظام السجلات", "en": "Events per SIEM batch"}'),
 ('siem.max_lag_s', '900', 'duration_s', NULL, FALSE,
  '{"ar": "أقصى تأخير مقبول لتصدير الأحداث (ثوان)", "en": "Longest accepted SIEM export lag (seconds)"}'),
 ('siem.attempts_before_dead_letter', '5', 'integer', NULL, FALSE,
  '{"ar": "محاولات قبل نقل الحدث إلى قائمة الأحداث المرفوضة", "en": "Attempts before a SIEM line is dead-lettered"}'),
 ('siem.export_access_events', 'true', 'boolean', NULL, TRUE,
  '{"ar": "تصدير سجل الاطلاع على السجلات إلى نظام السجلات الأمنية", "en": "Export record-access events to the SIEM"}'),
 ('release.currency_days', '180', 'integer', NULL, FALSE,
  '{"ar": "أقصى عمر للإصدار المطبق قبل التنبيه (أيام)", "en": "Age of the applied release before a warning (days)"}'),
 ('audit.rfc3161_enabled', 'false', 'boolean', NULL, TRUE,
  '{"ar": "تفعيل الختم الزمني RFC 3161 لمراسي السجل", "en": "RFC 3161 time stamps on audit anchors"}'),
 ('auth.upstream_mode', '"local_only"', 'string', NULL, TRUE,
  '{"ar": "نمط الدخول: محلي فقط أو عبر مزوّد هوية (OIDC)", "en": "Sign-in mode: local only or OIDC upstream"}'),
 ('schedule.break_glass_sweep', '"0 * * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول فحص الوصول الطارئ", "en": "Break-glass sweep (every minute)"}'),
 ('schedule.access_review_sweep', '"0 10 6 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول فحص مراجعات الصلاحيات", "en": "Access review sweep (06:10 bank time)"}'),
 ('schedule.support_session_sweep', '"30 * * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول فحص جلسات الدعم", "en": "Support session sweep (every minute)"}'),
 ('schedule.support_bundle_purge', '"0 40 3 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول حذف أرشيفات الدعم", "en": "Support archive purge (03:40 bank time)"}'),
 ('schedule.siem_forward', '"*/10 * * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول تصدير الأحداث إلى نظام السجلات", "en": "SIEM export (every 10 seconds)"}')
ON CONFLICT (key) DO NOTHING;

-- Column classification (copy of the phase 24 insert format). Ids, codes and counts only; no column is personal.
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('access_reviews', 'internal', ARRAY['id', 'status', 'snapshot_at', 'evidence_id', 'evidence_sha256', 'item_count',
                                       'due_on', 'started_by', 'started_at', 'signed_by', 'signed_at', 'closed_at',
                                       'created_at', 'updated_at', 'version']),
  ('access_review_items', 'internal', ARRAY['id', 'review_id', 'grant_id', 'user_id', 'role', 'decision',
                                            'decision_note_code', 'decided_by', 'decided_at', 'change_request_id', 'version']),
  ('break_glass_grants', 'internal', ARRAY['id', 'case_id', 'requester_id', 'reason_code', 'status', 'requested_at',
                                           'request_expires_at', 'first_approver_id', 'first_approver_role', 'first_approved_at',
                                           'second_approver_id', 'second_approver_role', 'second_approved_at', 'granted_at',
                                           'expires_at', 'ended_at', 'ended_by', 'reviewed_at', 'reviewed_by',
                                           'review_outcome_code', 'version']),
  ('break_glass_log', 'internal', ARRAY['id', 'grant_id', 'seq', 'kind', 'actor_id', 'detail', 'occurred_at']),
  ('releases', 'internal', ARRAY['id', 'release_version', 'channel', 'bundle_ref', 'manifest_sha256', 'status', 'refusal_code',
                                 'key_fingerprint', 'cosign_result', 'sbom_diff', 'notes', 'rollback_plan_sha256',
                                 'policy_bundle_hash', 'policy_matrix_diff', 'evidence_id', 'change_request_id',
                                 'imported_by', 'imported_at', 'verified_at', 'applied_by', 'applied_at', 'created_at', 'version']),
  ('help_articles', 'public', ARRAY['id', 'release_id', 'code', 'kind', 'title', 'body', 'sha256', 'sort_order', 'created_at']),
  ('support_sessions', 'internal', ARRAY['id', 'vendor_user_id', 'requested_by', 'approved_by', 'reason_code', 'ticket_ref',
                                         'requested_hours', 'status', 'requested_at', 'approved_at', 'starts_at', 'ends_at',
                                         'ended_at', 'ended_by', 'version']),
  ('support_bundles', 'internal', ARRAY['id', 'session_id', 'created_by', 'created_at', 'manifest_sha256', 'archive_sha256',
                                        'signature', 'signing_key_version', 'pseudonym_key_ref', 'size_bytes', 'confirmed_by',
                                        'confirmed_at', 'archive_deleted_at', 'status', 'version']),
  ('siem_cursors', 'internal', ARRAY['consumer', 'last_occurred_at', 'last_audit_id', 'forwarded_total', 'updated_at', 'version']),
  ('siem_dead_letters', 'internal', ARRAY['id', 'stream', 'audit_id', 'occurred_at', 'action', 'reason_code', 'attempts', 'created_at',
                                          'resend_requested_at', 'resend_requested_by', 'resolved_at', 'resolved_by',
                                          'resolution', 'resolution_code', 'version']),
  ('anchor_timestamps', 'internal', ARRAY['anchor_id', 'tsa_name', 'token', 'token_sha256', 'requested_at', 'verified_at'])
) AS v(t, cls, cols) CROSS JOIN LATERAL unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

-- Retention (copy of the phase 23 insert format). A clock column is named for every clock that needs one.
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'access_reviews',        'R9',  'closure',   'closed_at',    'Review and sign-off: from the signature; an open review is never due'),
 ('admin', 'access_review_items',   'R9',  'owner',     NULL,           'Items follow their review'),
 ('admin', 'break_glass_grants',    'R10', 'closure',   'reviewed_at',  'Break-glass evidence: from the review; an unreviewed grant is never due'),
 ('admin', 'break_glass_log',       'R10', 'owner',     NULL,           'The log follows its grant'),
 ('admin', 'releases',              'R10', 'never',     NULL,           'Release approvals and verification evidence are kept without limit'),
 ('admin', 'help_articles',         'R9',  'retirement', NULL,          'Articles of a release are kept with the release record'),
 ('admin', 'support_sessions',      'R9',  'closure',   'ended_at',     'Vendor access record: from the end of the session'),
 ('admin', 'support_bundles',       'R9',  'age',       'created_at',   'Disclosure record of a support bundle; the archive file goes after 30 days'),
 ('admin', 'siem_cursors',          'R12', 'retirement', NULL,          'Forwarder position; replaced in place'),
 ('admin', 'siem_dead_letters',     'R12', 'age',       'created_at',   'Operations record of refused lines; ids and codes only'),
 ('admin', 'anchor_timestamps',     'R10', 'owner',     NULL,           'The time stamp follows its anchor')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

## 2. DDD Implementation (Rust)

- **Crate placement** (phase 00 workspace): domain types in `domain/governance` (access reviews, break-glass, releases, support sessions, help articles, SIEM line), repository ports in the same module, implementations in `infrastructure/pg/platform`, HTTP in `api/routes/{access_reviews, break_glass, releases, support_sessions, help, siem_pull}`, the SIEM forwarder and its jobs in `worker/siem` and `worker/platform`. The pure rules (the four-hour arithmetic, the approver rule, the stripping rule, the CEF formatter) live in the domain and have no I/O.
- **Domain entities** (`Debug, Serialize, Deserialize, FromRow, ToSchema` unless noted):
  - `AccessReview { id, status: ReviewStatus, snapshot_at, evidence_id: Option<Uuid>, evidence_sha256: Option<[u8; 32]>, item_count, due_on: NaiveDate, started_by, started_at, signed_by: Option<Uuid>, signed_at: Option<DateTime<Utc>>, version }`
  - `AccessReviewItem { id, review_id, grant_id, user_id, role: RoleCode, decision: ReviewDecision, decision_note_code: Option<ReviewNote>, decided_by, decided_at, change_request_id, version }`
  - `BreakGlassGrant { id, case_id, requester_id, reason_code: BreakGlassReason, status: BreakGlassStatus, requested_at, request_expires_at, first_approver: Option<Approver>, second_approver: Option<Approver>, granted_at, expires_at, ended_at, ended_by, reviewed_at, reviewed_by, review_outcome_code, version }` where `Approver { user_id, role: ApproverRole }`
  - `BreakGlassLogEntry { id, grant_id, seq, kind: BreakGlassEventKind, actor_id, detail: serde_json::Map, occurred_at }`
  - `Release { id, release_version, channel, bundle_ref, manifest_sha256, status: ReleaseStatus, refusal_code: Option<RefusalCode>, key_fingerprint, cosign_result, sbom_diff, notes: Bilingual, rollback_plan_sha256, policy_bundle_hash, policy_matrix_diff, evidence_id, change_request_id, imported_by, imported_at, verified_at, applied_by, applied_at, version }`
  - `HelpArticle { id, release_id, code, kind: ArticleKind, title: Bilingual, body: Bilingual, sha256, sort_order }`
  - `SupportSession { id, vendor_user_id, requested_by, approved_by, reason_code: SupportReason, ticket_ref, requested_hours, status: SupportStatus, requested_at, approved_at, starts_at, ends_at, ended_at, ended_by, version }`
  - `SupportBundle { id, session_id, created_by, created_at, manifest_sha256, archive_sha256, signature: [u8; 64], signing_key_version, pseudonym_key_ref, size_bytes, confirmed_by, confirmed_at, sent_at, archive_deleted_at, status, version }`
  - `SiemLine { occurred_at, audit_id, action, severity: SiemSeverity, actor_id: Option<Uuid>, on_behalf_of: Option<Uuid>, resource_type, resource_id: Option<Uuid>, resource_key: Option<String>, details }` (the output of `audit.siem_page` plus the `severity` of the catalogue entry, section 3.5; never a `case_id` field)
  - `SiemCursor { consumer, last_occurred_at, last_audit_id, forwarded_total, version }`
- **Closed enums** (each a Rust enum with a fixed string per variant; a value outside it cannot be built): `ReviewDecision { Pending, Confirmed, RevokeProposed, Flagged, Superseded }`, `ReviewNote { StillNeeded, RoleChanged, LeftPost, Other }`, `BreakGlassReason { MlroUnavailable, CaseUnowned, RegulatorDeadline, StuckWork, Other }`, `BreakGlassStatus { Requested, Active, Ended, Expired, Lapsed }`, `ApproverRole { Ch, Sec }`, `BreakGlassEventKind` (the nine values of `break_glass_log.kind`), `ReleaseStatus { Imported, Verifying, Verified, Refused, Proposed, Approved, Applied, Superseded }`, `RefusalCode` (the nine codes of `0081`), `ArticleKind { Article, Glossary, WhatChanged }`, `SupportReason { DefectReproduction, UpgradeAssist, IncidentBridge }`, `SupportStatus { Requested, Active, Ended, Expired, RefusedTimeout }`, `SiemSeverity { Info, Notice, Warning, Critical }` (mirrors phase 02 `Severity`).
- **Create/Update DTOs**: `CreateAccessReview { }` (the snapshot instant is the server clock); `DecideAccessReviewItem { decision: ReviewDecision (confirmed, flagged, revoke_proposed), note_code: Option<ReviewNote> }` with `If-Match` on the item version; `CreateBreakGlassRequest { case_id, reason_code }`; `DecideBreakGlass { }` (approval only); `ReviewBreakGlass { outcome_code }`; `VerifyRelease { bundle_ref }`; `ProposeRelease { }`; `RecordReleaseApplied { rollout_evidence_id: Option<Uuid> }`; `CreateSupportSession { vendor_user_id, reason_code, ticket_ref, requested_hours }`; `RegisterSupportBundle { session_id, manifest_sha256, archive_sha256, signature, signing_key_version, pseudonym_key_ref, size_bytes }`. Update DTOs carry `version` for `If-Match`; no other update of these records exists.
- **Repository traits** (domain layer; `async`, all take a `&mut Transaction` for writes):
  - `AccessReviewRepository`: `insert(tx, &AccessReview)`, `lock(tx, id) -> AccessReview` (`SELECT ... FOR UPDATE`), `set_status(tx, id, ReviewStatus, expected_version)`, `items(id, page)`, `item_lock(tx, item_id)`, `decide_item(tx, item_id, decision, note, by, expected_version)`, `supersede_pending(tx, review_id, grant_ids)`, `open_reviews_due(now)`.
  - `BreakGlassRepository`: `open_for_case(tx, case_id) -> Option<BreakGlassGrant>`, `insert(tx, &BreakGlassGrant)`, `lock(tx, id)`, `set_approval(tx, id, Approver, at, expected_version)`, `activate(tx, id, granted_at, expires_at, expected_version)`, `end(tx, id, status, ended_by, at)`, `set_reviewed(tx, id, by, outcome, at)`, `active_for_principal_case(principal, case_id, now) -> bool` (read, used by the entity adapter), `active_case_ids_for(principal, now) -> Vec<Uuid>`, `due_sweep(now) -> Vec<BreakGlassGrant>`, `append_log(tx, &BreakGlassLogEntry)` (the next `seq` is `SELECT max(seq) + 1 ... FOR UPDATE` on the grant, inside the same transaction).
  - `ReleaseRepository`: `insert`, `lock`, `set_verification(tx, id, VerificationResult)`, `set_status(tx, id, from, to, expected_version)`, `record_applied(tx, id, by, at)`, `find_applied_latest()`, `newest_verified()`.
  - `HelpRepository`: `insert_many(tx, release_id, Vec<HelpArticle>)` (write once), `list(kind: Option<ArticleKind>, lang)`.
  - `SupportSessionRepository`: `insert`, `lock`, `set_approval(tx, id, by, at, starts_at, ends_at, expected_version)`, `end(tx, id, status, by, at)`, `open_for_vendor(vendor_user_id)`, `due_sweep(now)`, `insert_bundle(tx, &SupportBundle)`, `mark_bundle_purged(tx, id, at)`.
  - `SiemRepository`: `page(after: (DateTime<Utc>, Uuid), limit, kinds: &[&str]) -> Vec<SiemLine>` (calls `audit.siem_page`), `cursor_lock(tx, consumer)`, `advance(tx, consumer, at, id, n)`, `dead_letter(tx, &DeadLetter)`, `status() -> SiemStatus { lag_s, dead_letters_open, last_forwarded_at }` (`dead_letters_open` is the count of dead letters with `resolved_at IS NULL`, section 7.24).
- **Repository impl notes** (`Pg*Repository`, `akr_app` pool): every `UPDATE` names its column list from the grants of section 1.3 (a column outside the list is a permission error, which is a test); `RETURNING` is used for the version after each update; `set_status` is `UPDATE ... WHERE id = $1 AND version = $2 AND status = $3` and returns `E_VERSION_CONFLICT` on zero rows. `SiemRepository::page` reads only through `audit.siem_page`; the code has no `SELECT` on `audit.audit_log`.
- **utoipa schemas** (request and response DTOs, tag `platform`): `AccessReviewView`, `AccessReviewItemView` (no grant row copy; grant id, user id, role, decision), `AccessReviewStarted` (202), `BreakGlassRequestView` (no case title, no party; the case id is returned only to the requester, see section 3.2), `BreakGlassDetailView`, `ReleaseView` (notes bilingual; `refusal_code` closed list), `ReleaseVerifyAccepted` (202 with job id), `HelpArticleView`, `SupportSessionView` (vendor user id and ticket reference, no personal fields), `SupportBundleRegistered`, `SiemEventPage { events: Vec<SiemEventDto>, next: Option<SiemCursorDto> }`, `SiemEventDto` (the CEF fields as JSON keys, section 3.5), `Problem` (phase 00).
- **Route table** (scopes from API.md section 2; markers SU (step-up), CR (change request), 4E (four eyes), IDEM (`Idempotency-Key`), IF (`If-Match`); the Cedar action of each row is in section 3.12). A row marked *owned* is API.md 3.2, 3.3, 3.20 or the help row; *added* rows are new in this phase and are added to API.md in the same change.

| Method | Path | Scope | Description |
|-|-|-|-|
| GET | `/api/v1/admin/access-reviews` | `security.ops, dashboard.read` | Quarterly access reviews, newest first; the scope admits M, the reader of business items (owned; scope widened) |
| GET | `/api/v1/admin/access-reviews/{id}` | `security.ops, dashboard.read` | Review detail: state, snapshot identity, items with their decisions, paged (added) |
| POST | `/api/v1/admin/access-reviews` | `security.ops` | Start a review; 202 with the review id, the snapshot is built by job `access_review_snapshot`; IDEM (owned) |
| POST | `/api/v1/admin/access-reviews/{id}/sign` | `security.ops` | Sign the review once every item is decided; **SU** `approval`; IF; signer is not the starter (owned) |
| POST | `/api/v1/admin/access-reviews/{id}/items/{item_id}` | `security.ops, dashboard.read` | Confirm, flag or propose the revocation of one grant; IF on the item version; revocation creates a change request (added) |
| GET | `/api/v1/access/break-glass` | `self, security.ops, dashboard.read` | Break-glass requests and grants with status, approver roles and review state; no case id except in a requester's own rows; a requester sees own rows only (added) |
| POST | `/api/v1/access/break-glass` | `self` | Request break-glass on one case with a reason code; **SU** `break_glass`; IDEM; the same 202 whether or not the case exists (owned) |
| POST | `/api/v1/access/break-glass/{id}/approve` | `security.ops, dashboard.read` | Approve as CH or SEC; **SU** `break_glass`; IF; **4E** (two approvers of different roles, neither the requester) (owned) |
| POST | `/api/v1/access/break-glass/{id}/end` | `self` | End early (the requester or SEC); IF (owned) |
| POST | `/api/v1/access/break-glass/{id}/review` | `security.ops` | Record the review within 24 hours of the end: outcome code; IF (added) |
| GET | `/api/v1/admin/releases` | `admin.system, security.ops` | Imported release bundles with status, approval and applied record (owned) |
| POST | `/api/v1/admin/releases/verify` | `admin.system` | Import and verify a bundle by directory name; 202 with the job id; IDEM (owned) |
| POST | `/api/v1/admin/releases/{id}/propose` | `admin.system` | Propose the release for approval: change request `cr_access` (maker ADM, checker SEC); 202; IF; IDEM (added) |
| POST | `/api/v1/admin/releases/{id}/applied` | `admin.system` | Record that the bank operator applied the approved release; **SU** `approval`; IF; IDEM (added) |
| POST | `/api/v1/admin/releases/{id}/evidence` | `admin.system` | Upload the `rollout.json` of the apply as evidence of owner kind `release`; answers 201 with the evidence id; IDEM (added, section 7.22; Cedar `release.applied`) |
| GET | `/api/v1/admin/support-sessions` | `admin.system, security.ops` | Vendor sessions with state and times (added) |
| POST | `/api/v1/admin/support-sessions` | `admin.system` | Request a time-boxed vendor session; IDEM (owned) |
| POST | `/api/v1/admin/support-sessions/{id}/approve` | `security.ops` | Approve; **SU** `access_change`; the VS grant starts with the session; IF; **4E** (approver is not the requester) (owned) |
| DELETE | `/api/v1/admin/support-sessions/{id}` | `admin.system` | End the session now; IF (owned) |
| POST | `/api/v1/admin/support-sessions/{id}/bundles` | `admin.system` | Register a support bundle the operator built and confirmed (`akr-ctl support-bundle`); **SU** `export`; IDEM (added) |
| GET | `/api/v1/help/articles` | `self` | Bilingual help articles and "what changed" entries of the installed release; `?kind=` (owned) |
| GET | `/api/v1/siem/events` | `siem.pull` | Pull API for a service principal: events after a cursor, allow-listed fields only (added; section 3.5) |
| GET | `/api/v1/siem/access-events` | `siem.pull` | Pull API for a service principal: access lines after a cursor, the fields of section 7.23 item 5 (added; section 7.23) |
| GET | `/api/v1/admin/siem/dead-letters` | `admin.system, security.ops` | Dead letters, newest first; `?state=open\|resolved\|all` (default `open`), cursor, limit (added; section 7.24) |
| POST | `/api/v1/admin/siem/dead-letters/{id}/resend` | `admin.system` | Ask the forwarder to send the line again; 202; IF; IDEM (added; section 7.24) |
| POST | `/api/v1/admin/siem/dead-letters/{id}/resolve` | `admin.system` | Mark a dead letter resolved without sending it, with a resolution code; IF (added; section 7.24) |

- **Router**: `pub fn router() -> Router<AppState>` in each of the seven modules (`access_reviews`, `break_glass`, `releases`, `support_sessions`, `help`, `siem_pull`, and `siem_admin` of section 7.24), merged in `build_app`. The `siem_pull` router is mounted only when `siem.pull` is granted to a client in the token allow-list, and is otherwise absent (404 by the router, not by the handler).
- **Route count**: 26 rows, of which 12 are owned (API.md 3.2, 3.3, 3.20 and the help row) and 14 are added (the review detail, the review item decision, the break-glass list and review, the release propose and applied records, the rollout record upload of section 7.22, the support session list and bundle record, the SIEM pull route, the access-line pull route of section 7.23, and the three dead-letter routes of section 7.24). The size of the phase is XL (25 routes or more; section 7.24 item 2). The owned `GET /api/v1/admin/releases` row gains the scope `security.ops` (the SEC reader of section 3.12); the change is listed in section 4.5.

## 3. Business Logic

### 3.1 Quarterly access review (`AccessReviewService`; SP-25, M2)

Services and signatures (`thiserror` enum `PlatformError`, section 3.11):

```rust
pub struct AccessReviewService { reviews: Arc<dyn AccessReviewRepository>, grants: Arc<dyn RoleGrantReader>,
    matrix: Arc<AccessMatrixBuilder>, evidence: Arc<EvidenceStore>, cr: Arc<ChangeRequestService>,
    steps: Arc<StepUpService>, clock: Arc<BusinessClock>, settings: Arc<SettingsService>, audit: Arc<AuditSink>, jobs: Arc<JobQueue> }
impl AccessReviewService {
    pub async fn start(&self, ctx: &RequestCtx, idem: &IdemKey) -> Result<AccessReviewStarted, PlatformError>;
    pub async fn decide_item(&self, ctx: &RequestCtx, review: Uuid, item: Uuid, expected: i32, cmd: DecideAccessReviewItem) -> Result<AccessReviewItem, PlatformError>;
    pub async fn sign(&self, ctx: &RequestCtx, review: Uuid, expected: i32, header: StepUpHeader) -> Result<AccessReview, PlatformError>;
    pub async fn snapshot(&self, review: Uuid) -> Result<(), PlatformError>;          // job access_review_snapshot
    pub async fn sweep(&self, now: DateTime<Utc>) -> Result<SweepCounts, PlatformError>; // job access_review_sweep
}
```

**Start** (`POST /admin/access-reviews`; Cedar `access_review.start`, role SEC):
1. Refuse with `E_NOT_ALLOWED_IN_STATE` (sub-code `review_open`) when a review with status `building` or `open` exists (unique by rule, checked under `pg_advisory_xact_lock(hashtext('access_review'))`).
2. `due_on = BusinessClock::add(bank_today, settings.access.review_interval_days, Day)` (the floor caps the interval at 92 days).
3. Insert `admin.access_reviews` (status `building`, `snapshot_at = now()`, `started_by = actor`), enqueue job `access_review_snapshot {review_id}` in the same transaction, audit `access_review.started`, return 202. Idempotency: the `Idempotency-Key` replay returns the same review id (phase 00 table).

**Snapshot job** (worker; `access_review_snapshot`):
1. Lock the review row; a review not in `building` is a no-op (the job may run twice).
2. `matrix = AccessMatrixBuilder::build(snapshot_at)` (phase 03; roles to actions and the `sod.json` rules).
3. Read the grants in force at `snapshot_at`: `admin.role_grants` with `valid_from <= snapshot_at`, `revoked_at IS NULL OR revoked_at > snapshot_at`, `valid_to IS NULL OR valid_to > snapshot_at`. Support grants (`VS`) are included.
4. Canonical JSON `{ "matrix": ..., "grants": [{ "grant_id", "user_id", "role", "valid_from", "valid_to", "scope_sha256" }] }` (the scope is hashed, not copied). `EvidenceStore::put_generated` with kind `snapshot`, owner kind `access_review`, owner id = review id, retention class R9, classification `internal`, content type `application/json`, file name sealed (phase 02 rule). The returned evidence id and SHA-256 are the snapshot's identity.
5. Insert one `admin.access_review_items` row per grant (decision `pending`, `grant_id`, `user_id`, `role`). Items hold ids and the role code, never the grant's scope or dates: a reader who needs those reads the grant or the evidence object.
6. Update the review: status `open`, `evidence_id`, `evidence_sha256`, `item_count`; audit `access_review.snapshotted` (review, item count, evidence id).

**Decide an item** (`POST /admin/access-reviews/{id}/items/{item_id}`; Cedar `access_review.confirm`, `access_review.flag` or `access_review.revoke_propose`):
1. The review must be `open`; the item must be `pending` or `flagged` (a `flagged` item can still be proposed for revocation by SEC); otherwise `E_NOT_ALLOWED_IN_STATE`.
2. Compare `If-Match` with the item version: none is `E_PRECONDITION_REQUIRED` (428); a mismatch is `E_VERSION_CONFLICT` (409).
3. Lock the review, then the item (`FOR UPDATE`, lock order of section 3.16).
4. Check the grant is still in force (the same predicate as step 3 of the snapshot, at now). If not, set the item to `superseded` (no decider) and answer `E_NOT_ALLOWED_IN_STATE` with sub-code `grant_changed`. A grant that changed is never confirmed.
5. Decision by action: `confirmed` (SEC or M), `flagged` (SEC or M, with a note code), `revoke_proposed` (SEC only): the revocation is a change request of phase 04 (`ChangeRequestService::create`, target `role_grant`, operation `revoke` (extension below), key `user:<user_id>:role:<role>`, route `cr_access_adm` because the maker is SEC, checker ADM, `volume_effect` `neutral`). The item row takes `change_request_id` in the same transaction.
6. Set `decision`, `decision_note_code`, `decided_by`, `decided_at`, `version + 1`; audit `access_review.item_decided` (kind and note code only), or `access_review.revoke_proposed` for the proposal (`change_request_id`, `grant_id`).

Extension of phase 04 (`role_grant` change target; DR-234): one operation `revoke` is added to the target. Its apply is `UPDATE admin.role_grants SET revoked_at = now(), revoked_by = <checker> WHERE id = $grant AND revoked_at IS NULL`, then the phase 04 `invalidate_user`; the phase 03 audit kind `role_grant.revoked` is written by the apply. A revoke proposal for a grant that is already revoked fails validation with `duplicate_grant`-style code `grant_not_active` (422) and creates no request. The existing `DELETE /admin/users/{id}/grants/{grant_id}` (phase 03, at once, ADM) is unchanged and is not used by the review.

**Sign** (`POST /admin/access-reviews/{id}/sign`; Cedar `access_review.sign`, role SEC; step-up `access_review.sign` of class `approval`):
1. Consume the step-up assertion in the transaction (`StepUpService::consume`).
2. Lock the review; the status must be `open` (else `E_NOT_ALLOWED_IN_STATE`).
3. The signer must differ from `started_by` (`E_SELF_APPROVAL`, 403).
4. No item may be `pending` or `flagged`: otherwise `E_VALIDATION` (422) with `errors[].code = items_undecided` and the count. `revoke_proposed` items are allowed: the signature records that the review was done; the revocations complete through phase 04.
5. Set `status = signed`, `signed_by`, `signed_at`, `closed_at = signed_at`, `version + 1`; audit `access_review.signed` (counts by decision only).

**Own grants and business roles** (DR-234): a reviewer never decides on a grant held by themselves (Cedar `resource == principal` forbid, and the database check `decided_by <> user_id`). SEC decides on every item. M decides on items of users who hold no `SEC`, `ADM`, `IA`, `AS` or `VS` role (the Cedar permit reads the user entity's roles). The CH compliance head is not a reviewer in this phase (Open doubts 8).

**Sweep** (job `access_review_sweep`, cron `schedule.access_review_sweep`):
1. For every `pending` item whose grant is no longer in force, set `superseded` (once per item).
2. For every review in `building` or `open` with `due_on < bank_today`, open a warning through phase 24's `RegisterWriter::warn` and, only when it returns an id, write audit `access_review.overdue` (siem yes) in the same transaction; the statement `warn` runs is: `INSERT INTO admin.registers (id, kind, source, source_key, title, body, status) VALUES (..., 'warning', 'review_calendar', 'access_review_overdue:<review_id>', '{"ar": "...", "en": "..."}', '{"review_id": "<id>"}', 'open') ON CONFLICT (source_key) WHERE source_key IS NOT NULL DO NOTHING RETURNING id` (the unique index `uq_registers_source_key` of phase 24). The warning title is a fixed bilingual text with no counts that identify a person.
3. Return counts `{ superseded, overdue_opened }`.

### 3.2 Break-glass (`BreakGlassService`; SP-24, M3)

Services and signatures:

```rust
pub struct BreakGlassService { grants: Arc<dyn BreakGlassRepository>, steps: Arc<StepUpService>,
    notify: Arc<NotifyPort>, audit: Arc<AuditSink>, clock: Arc<BusinessClock>, settings: Arc<SettingsService> }
impl BreakGlassService {
    pub async fn request(&self, ctx: &RequestCtx, idem: &IdemKey, cmd: CreateBreakGlassRequest, header: StepUpHeader) -> Result<BreakGlassAccepted, PlatformError>;
    pub async fn approve(&self, ctx: &RequestCtx, id: Uuid, expected: i32, as_role: Option<ApproverRole>, header: StepUpHeader) -> Result<BreakGlassDetailView, PlatformError>;
    pub async fn end(&self, ctx: &RequestCtx, id: Uuid, expected: i32) -> Result<BreakGlassDetailView, PlatformError>;
    pub async fn review(&self, ctx: &RequestCtx, id: Uuid, expected: i32, outcome: ReviewOutcome) -> Result<BreakGlassDetailView, PlatformError>;
    pub async fn active_case_ids(&self, principal: Uuid, now: DateTime<Utc>) -> Result<HashSet<Uuid>, PlatformError>; // entity adapter and visibility
    pub async fn sweep(&self, now: DateTime<Utc>) -> Result<SweepCounts, PlatformError>; // job break_glass_sweep
}
```

No step of this phase asks whether a case exists: activation does not look at the case (step 8, DR-233), and the read path of phase 17 answers the masked 404 by its own predicate. There is no `CaseExistence` port.

**Request** (`POST /access/break-glass`; Cedar `break_glass.request`, role A2 only: phase 17 rule 1 admits only A1, A2 and M to a case row, M needs no grant, so a grant to SEC or CH would open nothing; lead review):
1. Step-up: `StepUpService::consume(tx, ctx, header, StepUpAction::BREAK_GLASS_REQUEST, grant_id)` (class `break_glass`), with the grant id generated first.
2. `reason_code` must be in the closed list (`E_VALIDATION`, `reason_code_invalid`). A `scope` of kind `party` is refused with `E_VALIDATION` and sub-code `party_scope_not_shipped` (Open doubts 1). A free-text reason is not accepted (Open doubts 3).
3. Insert the request (status `requested`, `request_expires_at = now() + break_glass.request_ttl_min`). The unique index is per requester and case: a requester who already has an open request or grant for the id gets the same 202 and the existing row (an idempotent repeat); another requester's open request has no effect on this one. The case id is stored as given, with no existence check and no foreign key (DR-233).
4. Audit `break_glass.requested` (resource: the inserted or existing grant of this requester; details: `reason_code`, `scope_kind = case`). The same audit row is written whether or not the case exists.
5. Enqueue the notice `notify.request {template_code: "security.break_glass_waiting", count}` to CH and SEC holders (section 3.3).
6. Answer 202 with `{ status: "requested" }` and no grant id. The response body is the same for an existing case, a missing case and a case with an open request. A requester who wants the status reads `GET /access/break-glass` (own requests only).

**Approve** (`POST /access/break-glass/{id}/approve`; Cedar `break_glass.approve`, roles SEC or CH; step-up class `break_glass` with action `break_glass.approve`):
1. Consume the step-up assertion (`StepUpService::consume`) in the transaction.
2. `pg_advisory_xact_lock(hashtext('break_glass:' || id))`, then `SELECT ... FOR UPDATE` on the grant.
3. The status must be `requested` and `request_expires_at > now()` (else `E_NOT_ALLOWED_IN_STATE`; sub-code `request_lapsed` when the sweep has not yet run).
4. The approver must not be the requester (`E_SELF_APPROVAL`, 403): a person holding A2 and an approving role cannot approve their own request.
5. The approver's role is `as_role` when given and held (CH or SEC), else the one role of the approver among CH and SEC. An approver holding both must name the role; one who names none is refused with 422 `E_VALIDATION`, sub-code `approver_role_required`.
6. First approval: set `first_approver_id`, `first_approver_role`, `first_approved_at`; log `approved`; answer 200 (still `requested`).
7. Second approval: the approver must differ from the first (`E_SELF_APPROVAL`) and hold the other role (`E_VALIDATION`, `approver_role_mismatch`). Then, in this order: set `second_approver_*`, `granted_at = now()`, `expires_at = granted_at + min(break_glass.max_hours, 4 hours)` (the table CHECK refuses anything longer), `status = active`; log `approved` and `activated`; audit `break_glass.activated` (resource: grant; details: the two approver roles and the expiry); enqueue `security.break_glass_opened` to SEC and M holders.
8. The activation does not look at the case. A request whose case id is missing activates like any other; the read path then answers the masked 404 for it. Approvers therefore learn nothing about whether a case exists. The requester learns that a case exists only by reading it under an active grant (Open doubts 6).

**Use** (the read path of an existing case; no new route):
1. Visibility: `services::visibility::build` (phase 17 section 3.10) gains one admitting clause in the team of rule 5, the phase 18 pattern: `OR ($purpose = 'break_glass' AND EXISTS (SELECT 1 FROM admin.break_glass_grants g WHERE g.case_id = c.id AND g.requester_id = $1 AND g.status = 'active' AND g.expires_at > now()))` (the purpose condition keeps the clause equal to the Cedar adapter of step 2, lead review). The clause sits in rule 5, the suspicion tier, which is the only tier a grant is needed for: the analyst network, the purpose code and step-up `record_open` of phase 17 still apply, so a grant never bypasses them.
2. Cedar entity adapter: the requester is added to `Case.assignees` (phase 18 mechanism) only when the request carries purpose code `break_glass` and an active grant exists for that case and principal (the grant's `expires_at` is compared with the request time, so the grant ends at its time box even if no sweep ran). Without the purpose code the adapter adds nothing, so the phase 17 and phase 18 team rules do not apply to the requester.
3. Read-only: file `51_platform.cedar` forbids every action outside a list of fifteen read actions when `context.purpose == "break_glass"` (section 3.12), so writes, decisions, access changes, RFIs, packs and downloads are refused, including actions later phases add. A grantee who sends another purpose gets no team membership (step 2), so writes are refused by phases 17 and 18 as for any non-member.
4. Every read writes its access event (phase 02, with purpose `break_glass`) and the audit event `break_glass.used` (siem yes; details: action kind and grant id; no case id). The dedicated log receives a `used` row on the first read and then at most one per 15 minutes per grant (volume control).
5. Sealed cases (phase 18, `E_CASE_SEALED` on writes): reads are allowed under the grant; no case event is written; the write refusal is unchanged.
6. Suspicion-tier cases: each read takes a fresh `record_open` step-up as phase 17 requires. The suspicion masking of phase 17 is not changed: a refused read is the masked 404.

**End** (`POST /access/break-glass/{id}/end`; self as requester, or SEC): status `active` becomes `ended`, `ended_at = now()`, `ended_by`; log `ended`; audit `break_glass.ended`. A request in `requested` may be ended by the requester (status `lapsed`, log `lapsed`, audit `break_glass.lapsed`). A grant in any other status is refused with 409 `E_NOT_ALLOWED_IN_STATE`.

**Review** (`POST /access/break-glass/{id}/review`; Cedar `break_glass.review`, role SEC): the grant must be `ended` or `expired` with `reviewed_at IS NULL`; the reviewer must not be the requester or an approver (`E_SELF_APPROVAL`); the outcome is one of `appropriate`, `escalate`, `no_action`; sets `reviewed_at`, `reviewed_by`, `review_outcome_code`; log `reviewed`; audit `break_glass.reviewed` (siem yes). A review after `ended_at + break_glass.review_hours` is still accepted and the sweep has already recorded `review_overdue`.

**Sweep** (job `break_glass_sweep`, every minute; no timer kind):
1. `requested` with `request_expires_at <= now()`: status `lapsed`, log `lapsed`, audit `break_glass.lapsed`.
2. `active` with `expires_at <= now()`: status `expired`, `ended_at = expires_at`, log `expired`, audit `break_glass.expired` (siem yes). The predicates of section 3.2 "Use" already refuse the grant from `expires_at`; the sweep records the end, so the four-hour limit holds even if nobody acts.
3. `ended` or `expired` with `reviewed_at IS NULL` and `ended_at + review_hours < now()`, once per grant: log `review_overdue`, audit `break_glass.review_overdue` (siem yes), and register a warning (`source = 'scan'`, `source_key = 'break_glass_review:<id>'`, body `{ "grant_id": ... }`).

**List** (`GET /access/break-glass`; Cedar `break_glass.read` for SEC, CH, M and, for the requester's own rows only (a handler filter), A2): for SEC, CH and M, the requests and grants with status, approver roles and times, and the review state; the case id is not in the list. A requester sees their own requests with the case id they supplied. IA sees counts only, from the control report of phase 24 (section 3.12).

**Masking before approval** (answer to the lead's question): until the second approval the requester has no grant, so every read of the case answers the masked 404 of phase 17 (an existing suspicion case and a missing one are identical). The request itself answers the same 202 whatever the case is. After activation, an existing case answers its normal read and a missing case answers the masked 404 again.

### 3.3 Notices of the break-glass path (phase 22 dispatcher; DR-243)

| Notice code (new, phase 22 `NoticeCode` extended by two variants and two seed rows in `0082`) | Producer | Recipients (active holders of the role at send time) | Parameter |
|-|-|-|-|
| `security.break_glass_waiting` | `break_glass.request` (step 5) and the sweep of waiting requests | CH and SEC | `count` (waiting requests) |
| `security.break_glass_opened` | `break_glass.approve` (step 7) | SEC (the CISO) and M (the MLRO and deputy) | `count` (active grants) |

The recipient rule "active holders of the role" is a new rule of phase 22's dispatcher for these two codes (extends phase 22 section 3.8, recorded in the phase 22 recipient table). Neither text names a case, a party, a requester or a reason. The notice does not link to the request; the recipient opens `/access`. A review-overdue warning goes to the register and to the command centre, not to a notice.

### 3.4 Generic OIDC upstream (`OidcUpstream`; TS-25, M1 P1)

`OidcUpstream` is the read-only configuration struct of section 4.3, not a service: it is the Kratos OIDC method and the BFF rules that read `auth.upstream_mode`. No code path of AKR-TMS parses an upstream token.

**Configuration** (deployment file `deploy/kratos/kratos.yml`, secrets from OpenBao path `kv/kratos/oidc/<provider>`):
1. `selfservice.methods.oidc.enabled: true` only when the bank runs `upstream_mode = oidc`; the provider list has exactly one entry with `provider: generic` (Entra ID, ADFS and Keycloak are configured as generic OIDC with their issuer URL). Providers `microsoft`, `google`, `github` and `apple` are refused by `akr-ctl oidc upstream check` (exit code 2).
2. `issuer_url`, `client_id` and the client secret (read from OpenBao; the file is mode 0400 in the container), scopes `openid email profile`, and a Jsonnet mapper that copies only the verified `email` claim and the subject; no other claim becomes a trait. `requested_claims` contains no group or role claim: roles are never taken from the upstream (DR-239).
3. `selfservice.flows.registration.enabled: false` (phase 01); an OIDC callback creates no identity. A human user's id is the Kratos identity id (phase 01), so the upstream subject is linked to an existing identity only by `akr-ctl identity link-upstream --user <id> --issuer <url> --subject <sub>` (zone Z5, the phase 01 Kratos admin path), which the phase 03 audit kind `user.updated` records with the field name `upstream_link`. An upstream login whose subject is not linked to an active local user is refused with a generic error. There is no automatic link by e-mail, which would let an upstream account take over a local one.
4. The Kratos route `/self-service/login/browser` shows the provider button only when `auth.upstream_mode = 'oidc'` (BFF reads the setting, cache 30 s).

**Local controls that still apply**:
- A federated user holds a local second factor (WebAuthn or TOTP, phase 01). The upstream's `amr` claim is ignored: it never satisfies AAL2, step-up or `record_open`. A federated user without a local factor is in the confined session of phase 01 (`E_CREDENTIAL_CHANGE_REQUIRED` outside the setup routes).
- Session limits are the phase 01 settings (`session.idle_timeout_s`, `session.absolute_lifetime_s`, default 8 hours). The upstream session length is not trusted.
- Sign-in lockout and password rules do not apply to the upstream; the upstream's own lockout is the bank's.

**Upstream disabled**: AKR-TMS does not receive the upstream's disable (no SCIM; phase 31). Effects: (a) a new upstream sign-in succeeds only while the subject is linked to an active local user, so the bank's runbook removes the link (`akr-ctl identity unlink-upstream`) when it disables a person upstream; (b) an existing session ends at the next of the phase 01 idle timeout (15 minutes) or absolute lifetime (8 hours); (c) the bank ends access at once with the phase 03 offboarding route (`POST /admin/users/{id}/offboard`, SU), which revokes sessions and tokens and ends grants. Until (c) or (b), a disabled upstream account can still hold a session; the runbook says so (Open doubts 4).

**Refused in this phase**: SAML (phase 31, Ory Polis), SCIM (phase 31), an upstream group or role mapping, a second upstream provider at once, an upstream `prompt=none` silent sign-in, and any token from the upstream sent to a downstream service.

**Settings and configuration used**: `auth.upstream_mode` (`local_only` or `oidc`, governed: changing it is a change request, phase 04), `session.*` (phase 01). Redeploy of the Kratos container is needed when the provider list changes; a mode switch needs one too: the flag of point 1 and the Kratos egress rule of section 4.4 follow the mode, so after the change request is activated the operator renders `kratos.yml` and the zone rules again and restarts Kratos (runbook step). Until then the BFF button of point 4 already follows the new mode.

### 3.5 SIEM forwarding and the pull API (`SiemForwarder`; SP-43, TS-27)

`SiemForwarder` owns the push to the bank's SIEM (akr-worker, job `siem_forward`); the pull route serves the same lines to a service principal through `SiemRepository::page`.

**Kinds exported**: the kinds whose catalogue entry has `siem = true` (phase 02 `EventKind`), named in the `kinds` argument of `audit.siem_page` from the Rust catalogue on every call (no table of kinds exists in the database). The use-case catalogue is data: `deploy/siem/use_cases.yaml` maps each exported kind to a use case (failed step-up, break-glass requested, activated, used, expired, policy activation, key operation, chain-verification failure, anchor lag, reconciliation failure, paused-by-bank, release refused, support session started, access review overdue). The file is validated in CI against the catalogue: an exported kind with no use case, or a use case naming a kind that is not exported, fails the build.

**Line format** (per audit row, from `audit.siem_page`):
1. CEF: `CEF:0|AKR|AKR-TMS|<release version>|<action>|<catalogue name>|<severity 0-10>|rt=<occurred_at ms> suid=<actor_id> cs1=<on_behalf_of> cs1Label=onBehalfOf cs2=<resource_type> cs2Label=resourceType cs3=<resource_id or empty> cs3Label=resourceId cs4=<audit id> cs4Label=auditId`. Severity: Info 3, Notice 5, Warning 7, Critical 9 (phase 02 `Severity`), read in the forwarder from the phase 02 catalogue entry of the row's kind, like the catalogue name, because `audit.siem_page` returns no severity column.
2. JSON (the alternative format of the bank's choice): `{"time", "audit_id", "action", "severity", "actor_id", "on_behalf_of", "resource_type", "resource_id", "resource_key", "details"}`; `resource_id`, `resource_key` and `details` are null or `{}` unless the resource tier is normal (below). The tier value itself is never sent.
3. Transport: syslog over TLS 1.2 or later (RFC 5425, port 6514 or the bank's), client certificate and key issued by the bank's own CA and held in the KV entry `siem/client` (written at install by `akr-ctl siem configure`; the `akr-worker` AppRole policy of phase 00 gains read on that path; no PKI engine is added), server certificate pinned by fingerprint given on a separate channel (`akr-ctl siem configure`).

**Stripping rule** (phase 02 section 3 rule, honoured here and made stricter for the restricted tier; DR-235): the SQL function `audit.siem_page` never returns `case_id`. For a row whose resource tier (phase 24 `audit.resource_tier(type, id, case_id)`) is not `normal`, it returns the row with `resource_id`, `resource_key` and `details` removed. The actor id and the action remain: they are staff ids and kinds, not case content. A row whose tier is `restricted` or `suspicion` therefore reaches the SIEM as kind, time, actor and an empty resource.

**Forwarder algorithm** (job `siem_forward`, concurrency 1, coalesce key `siem_forward`):
1. `pg_try_advisory_lock(hashtext('siem_forward'))`; if not acquired, return (another forwarder runs). Steps 2 to 7 run on the connection that took the lock, and the job releases it with `pg_advisory_unlock(hashtext('siem_forward'))` before the connection goes back to the pool, on every exit path including an error.
2. Read the cursor: `admin.siem_cursors WHERE consumer = 'cef_forwarder'` (a missing row starts at `1970-01-01`, nil id).
3. `rows = audit.siem_page(cursor.at, cursor.id, siem.batch_size, exported_kinds)`.
4. For each row: format (CEF or JSON per configuration). A row that cannot be formatted (unknown kind, oversize line over 8 KiB) goes to `admin.siem_dead_letters` with `reason_code` and the cursor moves past it.
5. Send the batch as one TLS write per line with a delivery acknowledgement from the collector (RFC 6587 framing). On a refusal or timeout, retry with back-off; after `siem.attempts_before_dead_letter` attempts on one line, dead-letter that line (`collector_refused`) and continue. The cursor moves only past lines the collector accepted or that were dead-lettered.
6. Advance the cursor (`last_occurred_at`, `last_audit_id`, `forwarded_total`, `version + 1`) in the same transaction as the dead-letter rows; the first advance inserts the cursor row (the `INSERT` grant of migration `0082`) and later advances update it.
7. Back-pressure: a batch that cannot be sent holds the cursor; the lag grows; nothing is dropped. The lag is the age of the first row that `audit.siem_page` returns after the cursor (`now()` minus its `occurred_at`), or zero when it returns none, and it is shown on the cockpit (section 3.10); above `siem.max_lag_s` the tile is `critical`. The audit log is the source of truth: a SIEM outage never blocks an audited action, and a forwarder restart resumes at the cursor.

**Pull API** (`GET /api/v1/siem/events?after_at=&after_id=&limit=`; scope `siem.pull`; Cedar `siem.pull`; the caller is a service principal of phase 01 `ClientCredentials`, kind `service`, with `siem.pull` in its token allow-list and an expiry within one year):
1. Calls `audit.siem_page` with the caller's cursor and the same `exported_kinds` list; `limit` is 1 to 500 (this route's own cap, above the 200 of API.md pagination, because the caller is a machine); a value outside it is 422 `E_VALIDATION`.
2. Returns `{ events, next: { after_at, after_id } }`; `next` is the last row's `(occurred_at, audit id)`; an empty page returns the same cursor.
3. The response carries the same fields as the push line. Replays are harmless: rows are immutable and the cursor is the caller's.
4. Refusals: a user token or a PAT answers `E_SCOPE` (403); a service principal without `siem.pull` never reaches the route (the router is not mounted for it, section 2).

**Network** (zone change, section 4.4): `akr-worker` gains one egress network `z2_bank`, not internal, whose only allowed destinations are the bank's SIEM collector and the RFC 3161 time-stamp authority (section 3.7), enforced by the rendered nftables rules of phase 00. The forwarder never reaches the Internet.

**Phase 00 topic reservation**: `siem.export` (phase 00 section 3.1) is not registered. The forwarder reads the audit log by cursor, so no outbox row is written per audit event (DR-235).

### 3.6 Signed release bundles (`ReleaseService`; SP-66, TS-30)

Services and signatures:

```rust
pub struct ReleaseService { releases: Arc<dyn ReleaseRepository>, cr: Arc<ChangeRequestService>, evidence: Arc<EvidenceStore>,
    audit: Arc<AuditSink>, jobs: Arc<JobQueue>, keys: Arc<BundleTrustRoots> }
impl ReleaseService {
    pub async fn import_and_verify(&self, ctx: &RequestCtx, idem: &IdemKey, cmd: VerifyRelease) -> Result<ReleaseVerifyAccepted, PlatformError>;
    pub async fn verify(&self, release: Uuid) -> Result<(), PlatformError>;                  // job release_verify
    pub async fn propose(&self, ctx: &RequestCtx, idem: &IdemKey, release: Uuid, expected: i32) -> Result<ReleaseView, PlatformError>;
    pub async fn record_applied(&self, ctx: &RequestCtx, idem: &IdemKey, release: Uuid, expected: i32, cmd: RecordReleaseApplied, header: StepUpHeader) -> Result<ReleaseView, PlatformError>;
    pub async fn list(&self, ctx: &RequestCtx) -> Result<Vec<ReleaseView>, PlatformError>;
}
```

**Import** (`POST /admin/releases/verify`; Cedar `release.verify`; no step-up: nothing changes on the platform):
1. `bundle_ref` must match the pattern of `0081` and name a directory in the import volume (read-only mount; the server never follows links out of it). A `bundle_ref` that names no directory is 422 `E_VALIDATION`, sub-code `bundle_not_found`; a directory without `manifest.json` is 422 `E_VALIDATION`, sub-code `manifest_missing`; both are answered before the 202.
2. Insert the row `imported` with the manifest digest read from `manifest.json` (the name only is sent by the client), `imported_by`; enqueue `release_verify {release_id}`; audit `release.imported` (siem yes); 202.
3. A bundle whose version and manifest digest already exist returns the existing row (unique key `(version, manifest_sha256)`).

**Verify job** (worker; `release_verify`, attempts 1: a verification is deterministic and is run again by a new import):
1. Status `verifying`. Run the phase 00 bundle verifier (`akr-ctl bundle verify` code path, not a copy): (a) the public key matches the fingerprint the bank received on a separate channel (`key_fingerprint`; mismatch: `fingerprint_mismatch`); (b) the manifest signature (cosign) is valid (`bundle_signature_invalid`); (c) every image signature is valid (`image_signature_invalid`); (d) every digest in the manifest matches its file (`manifest_digest_mismatch`); (e) the SBOM, the notes and the rollback plan are present (`sbom_missing`, `notes_missing`, `rollback_plan_missing`); (f) the version is newer than the installed one (`version_not_newer`; a security channel release may be older only when it is listed in the notes as a patch of the installed line).
2. Compute the SBOM diff against the installed release (`sbom_diff`: counts added, removed, changed, and the component names with their versions; no licence text).
3. Compute the policy bundle hash (SHA-256 over the canonical bundle, phase 03) and run `akr-ctl policy matrix --bundle <dir> --json` (phase 08 section on the release check) to produce `policy_matrix_diff` against the active bundle: roles and actions added or removed. A matrix that widens a role's actions is shown in the diff and needs the approval of section 3.6 step "Propose", which is the same four-eyes check.
4. On any failure: status `refused`, `refusal_code` (the first failing check in the order (a) to (f)), `verified_at = now()`; nothing else changes; audit `release.refused` (siem yes; details: code only). A refused bundle is never applied and the import is repeated after a new bundle arrives.
5. On success: write the verification report as evidence (`EvidenceStore::put_generated` with kind `package`, owner kind `release`, owner id the release id, R10, classification `internal`, content type `application/json`), fill `cosign_result` (key fingerprint, counts of verified signatures, no signature bytes), `notes` (bilingual, from the bundle's `notes.json`), `rollback_plan_sha256`, `policy_bundle_hash`, `policy_matrix_diff`, `evidence_id`, `verified_at`; status `verified`; audit `release.verified` (siem yes); insert the articles of the bundle (section 3.9) in the same transaction.

**Propose** (`POST /admin/releases/{id}/propose`; Cedar `release.propose`, role ADM): the release must be `verified` (else `E_NOT_ALLOWED_IN_STATE`). Create change request `ChangeRequestService::create` for target kind `platform_release` (new target of phase 04's `ChangeTargetRegistry`, key `release:<release_version>`, base version `release.version` (the row's optimistic-lock version), route `cr_access` (maker ADM, checker SEC, extension of the phase 04 route map for this target kind), `volume_effect = neutral`, evidence = the verification report of step 5). Status `proposed`, `change_request_id`; audit `release.proposed` (siem yes). The approval is the checker's decision in phase 04 (SEC), so the four eyes are the ADM maker and the SEC checker. The approved state is set by the phase 04 apply of the target (`status = approved`).

**Applied** (`POST /admin/releases/{id}/applied`; Cedar `release.applied`, role ADM; step-up class `approval` with action `release.applied`): the status must be `approved` (else 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `release_not_approved`). One transaction, in this order: consume the step-up assertion; lock the release; check the status; when `rollout_evidence_id` is given, read it with `EvidenceRepository::get` and refuse with 422 `E_VALIDATION` unless its owner kind is `release` and its owner id is this release (section 7.8); then write and audit. This records what the bank operator did; the platform does not apply a release, pull images, call home or push. Set `applied_by`, `applied_at`, status `applied`, and move the previously `applied` row to `superseded` in the same transaction; audit `release.applied` (siem yes).

**Currency**: the ops tile of section 3.10 shows the age of the `applied` release (`applied_at`) against `release.currency_days` and the existence of a newer `verified`, `proposed` or `approved` release that is not yet applied (its wait is counted from `verified_at`).

**Tipping-off**: a release record and its notes are vendor text and bank facts about the software; they carry no case, party or customer data (section 3.14).

### 3.7 Inspector accounts and the RFC 3161 option (SP-44, partly)

**Inspector accounts** (DR-241): a CBI inspector or an external assessor uses the assessor role `AS` of phase 24 (`admin.role_grants` with `valid_to`, the assessor workspace of `GET /access/control-report` and the assessor pack). No new role and no new scope are created. The account is a `human` user created by ADM with the phase 04 route `cr_access` (maker ADM, checker SEC), the grant's `valid_to` no later than the phase 24 setting `assessor.grant_max_days` (its floor is a maximum of 30 days). A longer inspection needs a new grant, not a longer one. Reads by an inspector carry purpose `supervisory_inspection` (phase 03 purpose codes). The inspector sees no suspicion-tier item except through the sample rule of phase 24 (its Open doubts 1).

**RFC 3161** (`audit.rfc3161_enabled`, default `false`, governed): when `true`, each new `audit.audit_anchors` row (phase 02, every five minutes) triggers job `anchor_timestamp {anchor_id}` (concurrency 1, attempts 5, back-off per phase 00). The job sends the anchor's `statement_sha256` to the time-stamp authority named by the deployment configuration (egress over `z2_bank`, section 3.5), stores the response token and its SHA-256 in `admin.anchor_timestamps` (the anchor row is immutable, phase 02), and verifies the token's signature chain against the bank's TSA certificate. A token that does not verify stays `verified_at` NULL and raises the cockpit tile `anchor_timestamp_unverified` (warning). The option does not change the Ed25519 anchor signature, which stays the primary proof (phase 02).

### 3.8 Vendor support sessions and support bundles (`SupportSessionService`; SP-64, SP-65, SP-52 part)

Services and signatures:

```rust
pub struct SupportSessionService { sessions: Arc<dyn SupportSessionRepository>, roles: Arc<dyn RoleGrantWriter>, steps: Arc<StepUpService>,
    audit: Arc<AuditSink>, clock: Arc<BusinessClock>, settings: Arc<SettingsService>, identity: Arc<dyn IdentityProvider>, tokens: Arc<dyn TokenRepository>, bundles: Arc<dyn BundleRecorder> }
impl SupportSessionService {
    pub async fn request(&self, ctx: &RequestCtx, idem: &IdemKey, cmd: CreateSupportSession) -> Result<SupportSessionView, PlatformError>;
    pub async fn approve(&self, ctx: &RequestCtx, id: Uuid, expected: i32, header: StepUpHeader) -> Result<SupportSessionView, PlatformError>;
    pub async fn end(&self, ctx: &RequestCtx, id: Uuid, expected: i32) -> Result<SupportSessionView, PlatformError>;
    pub async fn register_bundle(&self, ctx: &RequestCtx, idem: &IdemKey, session: Uuid, cmd: RegisterSupportBundle, header: StepUpHeader) -> Result<SupportBundleRegistered, PlatformError>;
    pub async fn sweep(&self, now: DateTime<Utc>) -> Result<SweepCounts, PlatformError>;   // job support_session_sweep
    pub async fn purge_bundles(&self, now: DateTime<Utc>) -> Result<SweepCounts, PlatformError>; // job support_bundle_purge
}
```

**Request** (`POST /admin/support-sessions`; Cedar `support_session.open`, role ADM):
1. The vendor user exists, is `human`, `active`, holds no other role (`sod.json` lists `VS` as exclusive; the check reads `admin.role_grants` for the user) and has no open session (unique index `uq_support_sessions_open`). Refusals: a vendor user that is missing, not `human` or not `active` is 422 `E_VALIDATION`, sub-code `vendor_user_invalid`; a user holding another role is 422 `E_VALIDATION`, sub-code `sod_conflict` (phase 04's code for the same check); an open session is 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `session_open`.
2. `requested_hours` ≤ `support.session_max_hours` (floor 8; `E_VALIDATION`, `hours_over_limit`); `reason_code` and `ticket_ref` from the closed list and the pattern of `0081`.
3. Insert status `requested`; audit `support_session.requested` (siem yes). No role is granted yet.

**Approve** (`POST /admin/support-sessions/{id}/approve`; Cedar `support_session.approve`, role SEC; step-up class `access_change` (it creates a role grant), action `support_session.approve`; 4E):
1. Lock the session. Status `requested` else `E_NOT_ALLOWED_IN_STATE`. The approver is not the requester (`E_SELF_APPROVAL`).
2. `starts_at = now()`, `ends_at = starts_at + requested_hours`.
3. Insert `admin.role_grants` with `user_id = vendor`, `role = 'VS'`,  `scope = {"branches": [], "pools": []}`(the phase 01 scope form with an empty `branches` list grants no branch data and no pool), `source = 'support_session'`, `support_session_id`, `valid_from = starts_at`, `valid_to = ends_at`, `granted_by = approver`. The check constraint of `0081` admits the row only with `role = 'VS'` and a `valid_to`.
4. Status `active`, `approved_by`, `approved_at`, `starts_at`, `ends_at`, `version + 1`; audit `support_session.started` (siem yes; details: the time box in seconds and the reason code).
5. Notify the vendor user: none. The vendor is informed through the bank's own channel (PAM), not by a platform notice.

**The VS role** (Cedar, section 3.12): VS has no permit for any action except the actions of group `self` (own profile, own sessions and tokens). Every other action is forbidden to a principal holding VS by a `forbid` that names all actions but those of `self`. A case read by VS answers 403 `E_SCOPE` at the gate for every case id (VS holds no `queue.work`), so no answer tells a normal case from a suspicion case or a missing one.

**End and expiry**: `DELETE /admin/support-sessions/{id}` (ADM) and the sweep (`support_session_sweep`, every minute) set the grant's `revoked_at` and `revoked_by` (the sweep as the system actor), revoke the vendor's sessions and tokens as phase 03's offboarding does (the tokens with phase 01's `TokenRepository::revoke_all_for_user` in the same transaction; after the commit phase 01's `invalidate_user` and Kratos `IdentityProvider::revoke_all_sessions`), set status `ended` or `expired`, `ended_at`, `ended_by`, and audit `support_session.ended` or `support_session.expired` (siem yes). A session still `requested` after one hour is set `refused_timeout` (`ended_at` set, no grant; audit `support_session.refused_timeout`). A `DELETE` on a session that is neither `active` nor `requested` is refused with 409 `E_NOT_ALLOWED_IN_STATE`.

**Support bundle** (`akr-ctl support-bundle create`, run by the bank operator on the bank host; `POST /admin/support-sessions/{id}/bundles` registers the result; Cedar `support_bundle.register`, role ADM; step-up class `export`, action `support_bundle.register`):
1. The CLI writes an archive with: versions and image digests with the SBOM id; configuration with secrets removed and `detection-logic` parameters removed unless the bank ticks the box `--include-detection-logic`; health, queue and reconciliation summaries; a metrics snapshot; redacted logs; normalised query statistics without literals; table row counts.
2. Redaction is an allow-list of log fields (the list is in the CLI's source and is tested by a fixture). Identifiers are pseudonymised with HMAC-SHA-256 under a per-bundle key from OpenBao (`pseudonym_key_ref` names the path; the key never leaves the bank). Case ids, party names, account numbers and free text are not on the allow-list.
3. The manifest (file list, digests, counts) is printed. The operator confirms it by typing its SHA-256 (`--confirm-manifest`); the confirmation is recorded as `confirmed_by`, `confirmed_at`.
4. The archive is signed with Ed25519 over the manifest hash, using the bank's transit key `akr-sign-bundle` (a new key name, Open doubts 9); the size cap `support.bundle_max_mb` is enforced before writing.
5. `register_bundle` stores the row (the four hashes, size, signature, key version, pseudonym key path) after the step-up, and audit `support_bundle.registered` (siem yes; details: session id, size, key version).
6. The vendor receives the archive through the bank's own transfer channel. The platform does not record the transfer: the bank's disclosure entry for it is the open doubt of section 3.8 (Open doubts 9).

**Purge** (job `support_bundle_purge`, daily): every archive file of a bundle older than `support.bundle_retention_days` (30) is deleted from the bank's disk; `archive_deleted_at` and status `archive_deleted` are set; the row and its hashes stay (the disclosure record, R9). Audit `support_bundle.archive_deleted` (no SIEM).

**Disclosure** (DR-238): the M-37 register of phase 19 holds authority requests (`casework.authority_requests`, fields `disclosed_at`, `disclosed_items`). Support bundles do not join it: that register is about case-related disclosures to authorities and has a case reference per row. Support bundles keep their own table (`admin.support_bundles`), which is the first part of the general disclosure register that SP-52 asks for (Open doubts 9).

### 3.9 Help centre and "what changed" (`HelpCentreService`; M20, X5)

`HelpCentreService::import(tx, release_id, bundle_dir)` runs inside the verification transaction of section 3.6 (step 5), after the article check of section 7.20 has passed. That check runs before step 5: it reads `help/*.json` from the verified bundle, tests each article against the schema (`code`, `kind`, `title` and `body` with `ar` and `en`) and refuses bidirectional-control characters outside the Arabic and Latin ranges (UX-20); a failure refuses the release. `import` then writes one row per article with its SHA-256 and inserts it once (`UNIQUE (release_id, code)`; the rows are write-once by trigger). Articles of the installed release are read; an older release keeps its own rows, so "what changed" lists the entries of the release that introduced each change. No article is fetched at run time and no article carries bank data (the articles are vendor text, class `public`). The vocabulary of the articles follows DESIGN-SYSTEM section 11 (the glossary of compliance staff, X5). `GET /help/articles?kind=` returns the articles of the installed release in the requester's language with the other language as fallback.

### 3.10 Ops cockpit tiles (extends phase 03 `OpsCockpit::snapshot()`; TS-28)

| Tile | Source | Warn | Critical |
|-|-|-|-|
| `siem_lag` | the lag of section 3.5 step 7 (the age of the oldest exported row after the cursor; 0 when there is none) | above `siem.max_lag_s / 2` | above `siem.max_lag_s` |
| `siem_dead_letters` | count of `siem_dead_letters` with `resolved_at IS NULL` (an operator resends or resolves them, section 7.24) | at least 1 | above 10 |
| `release_currency` | age of the applied release against `release.currency_days`; a newer verified, proposed or approved release not applied | either condition | a newer verified, proposed or approved release has waited unapplied for more than 30 days, counted from its `verified_at` |
| `support_sessions_open` | count of sessions `active` (and `requested` older than 1 hour) | at least 1 | none (count is information) |
| `break_glass_active` | count of grants `active` | at least 1 | a grant `active` past `expires_at` (must not happen; alarm) |
| `break_glass_review_overdue` | count of grants that have an `admin.break_glass_log` row of kind `review_overdue` and `reviewed_at IS NULL` | at least 1 | at least 1 for more than 24 hours |
| `access_review_overdue` | reviews `open` past `due_on` | at least 1 | more than 30 days past `due_on` |
| `anchor_timestamp_unverified` | `anchor_timestamps` with `verified_at IS NULL` (only when RFC 3161 is on) | at least 1 | more than 24 hours |

The tiles are read-only and named as above; the cockpit's own tile schema (phase 03) carries `state` (ok, warn, critical). The G5 access screen and the command centre read the break-glass and review counts from the same queries (section 5).


### 3.11 Errors (`PlatformError`, `thiserror`; API.md section 6 codes only)

| Variant | Problem code | Status | Sub-codes (closed list, `errors[].code` or `reason`) |
|-|-|-|-|
| `Validation` | `E_VALIDATION` | 422 | `reason_code_invalid`, `party_scope_not_shipped`, `approver_role_mismatch`, `items_undecided`, `hours_over_limit`, `grant_not_active`, `release_not_verified` |
| `NotAllowedInState` | `E_NOT_ALLOWED_IN_STATE` | 409 | `review_open`, `grant_changed`, `request_lapsed`, `not_approved`, `not_reviewable`, `session_not_requested`, `release_not_approved` |
| `SelfApproval` | `E_SELF_APPROVAL` | 403 | none (requester, first approver, starter of a review, requester of a support session) |
| `Forbidden` | `E_FORBIDDEN` | 403 | Cedar denial on a resource the caller may know exists (a normal case read by VS, a review item held by the reviewer) |
| `NotFound` | `E_NOT_FOUND` | 404 | missing or masked; identical body (the case read of section 3.2 "Masking") |
| `ScopeDenied` | `E_SCOPE` | 403 | the gate of the route |
| `StepUpRequired` | `E_STEPUP_REQUIRED` | 401 | missing, reused or wrong-class assertion (`break_glass`, `approval`, `access_change`, `export`) |
| `VersionConflict` | `E_VERSION_CONFLICT` | 409 | `If-Match` does not match; the current version in the body |
| `PreconditionRequired` | `E_PRECONDITION_REQUIRED` | 428 | versioned mutation without `If-Match` |
| `IdempotencyRequired` | `E_IDEMPOTENCY_REQUIRED` | 400 | mutation without `Idempotency-Key` |
| `IdempotencyReuse` | `E_IDEMPOTENCY_REUSE` | 422 | same key, different body |
| `Unavailable` | `E_UNAVAILABLE` | 503 | the pull API when PostgreSQL is down (the forwarder retries) |

No new problem code is added. A refused release bundle is not an HTTP error of the verify route (the route answers 202; the job records `refusal_code`, section 3.6); the refusal codes are fields of the release record.

### 3.12 Cedar: actions, roles and the file `51_platform.cedar` (phase 03 range 50)

**Schema additions** (appended to `schema.cedarschema` as phases 17, 18 and 24 did; no existing entity or attribute changes):

```cedar
namespace Akr {
  action "access_review.read", "access_review.start", "access_review.sign", "break_glass.read", "break_glass.request",
         "break_glass.approve", "break_glass.end", "break_glass.review", "release.read", "support_session.read",
         "help.read", "siem.pull"
    appliesTo { principal: User, resource: System, context: Ctx };
  action "access_review.confirm", "access_review.flag", "access_review.revoke_propose"
    appliesTo { principal: User, resource: User, context: Ctx };
  action "release.verify", "release.propose", "release.applied", "support_session.open", "support_session.approve",
         "support_session.end", "support_bundle.register" in ["system_admin"]
    appliesTo { principal: User, resource: System, context: Ctx };
}
```

Note: `support_session.approve` is declared with the `system_admin` group above; its approval is an interactive act of SEC, so the phase 03 baseline (AAL2 in a session) applies to it too. The `break_glass.*`, `access_review.*` and `release.read` actions are not in a group: their step-up is in the handler, and they are session-only by their scopes, because a personal access token never holds `security.ops` or an `admin.*` scope (API.md section 2).

**File `51_platform.cedar`** (every policy written out; one role per `@role`; `principal.roles` is the phase 03 form; no `containsAny`; `context.purpose` is a required `Ctx` field and needs no `has`):

```cedar
// 51_platform.cedar (phase 25). Roles are principal.roles (phase 03 form). Step-up is a handler check, not a condition.

// Quarterly access review (SP-25, M2). The reviewer never decides on a grant held by themselves.
@id("ar.sec.system") @role("SEC")
permit (principal, action in [Akr::Action::"access_review.read", Akr::Action::"access_review.start",
        Akr::Action::"access_review.sign"], resource is Akr::System)
when { principal.roles.contains("SEC") };

@id("ar.m.read") @role("M")
permit (principal, action == Akr::Action::"access_review.read", resource is Akr::System)
when { principal.roles.contains("M") };

@id("ar.sec.items") @role("SEC")
permit (principal, action in [Akr::Action::"access_review.confirm", Akr::Action::"access_review.flag",
        Akr::Action::"access_review.revoke_propose"], resource is Akr::User)
when { principal.roles.contains("SEC") };

@id("ar.m.items") @role("M")
permit (principal, action in [Akr::Action::"access_review.confirm", Akr::Action::"access_review.flag"], resource is Akr::User)
when { principal.roles.contains("M") && resource.kind == "human" && !resource.roles.contains("SEC")
       && !resource.roles.contains("ADM") && !resource.roles.contains("IA") && !resource.roles.contains("AS")
       && !resource.roles.contains("VS") };

@id("ar.no_own_grant")
forbid (principal, action in [Akr::Action::"access_review.confirm", Akr::Action::"access_review.flag",
        Akr::Action::"access_review.revoke_propose"], resource is Akr::User)
when { resource == principal };

// Break-glass (SP-24, M3). The requester is A2 (phase 17 rule 1); approval is CH or SEC; the approver rule is in the handler.
@id("bg.request.a2") @role("A2")
permit (principal, action == Akr::Action::"break_glass.request", resource is Akr::System)
when { principal.roles.contains("A2") };

@id("bg.approve.sec") @role("SEC")
permit (principal, action == Akr::Action::"break_glass.approve", resource is Akr::System)
when { principal.roles.contains("SEC") };

@id("bg.approve.ch") @role("CH")
permit (principal, action == Akr::Action::"break_glass.approve", resource is Akr::System)
when { principal.roles.contains("CH") };

@id("bg.end.a2") @role("A2")
permit (principal, action == Akr::Action::"break_glass.end", resource is Akr::System)
when { principal.roles.contains("A2") };   // the handler admits the requester's own grant only

@id("bg.end.sec") @role("SEC")
permit (principal, action in [Akr::Action::"break_glass.end", Akr::Action::"break_glass.review"], resource is Akr::System)
when { principal.roles.contains("SEC") };

@id("bg.read.a2") @role("A2")
permit (principal, action == Akr::Action::"break_glass.read", resource is Akr::System)
when { principal.roles.contains("A2") };

@id("bg.read.ch") @role("CH")
permit (principal, action == Akr::Action::"break_glass.read", resource is Akr::System)
when { principal.roles.contains("CH") };

@id("bg.read.m") @role("M")
permit (principal, action == Akr::Action::"break_glass.read", resource is Akr::System)
when { principal.roles.contains("M") };

@id("bg.read.sec") @role("SEC")
permit (principal, action == Akr::Action::"break_glass.read", resource is Akr::System)
when { principal.roles.contains("SEC") };

// A grantee cannot change, export or hand on a case opened under break-glass (SP-24 "no export"; section 3.2 "Use").
// An allow-list of reads (lead review): team membership also feeds later team checks (phase 19 Rfi.assignees), so a
// list of forbidden writes would miss actions added later.
@id("bg.read_only")
forbid (principal, action, resource)
when { context.purpose == "break_glass" &&
       !(action in [Akr::Action::"case.read", Akr::Action::"case.note.read", Akr::Action::"case.evidence.read",
                    Akr::Action::"case.chain.read", Akr::Action::"case.link.read", Akr::Action::"case.work.read",
                    Akr::Action::"case.access.read", Akr::Action::"alert.read", Akr::Action::"alert.snapshot.read",
                    Akr::Action::"party.read", Akr::Action::"party.transactions.read", Akr::Action::"network.read",
                    Akr::Action::"rfi.read", Akr::Action::"rfi.read.case", Akr::Action::"report.read"]) };

// Releases (SP-66, TS-30). Approval is a change request (phase 04, cr_access); the apply is the operator's record.
@id("rel.read.sec") @role("SEC")
permit (principal, action == Akr::Action::"release.read", resource is Akr::System)
when { principal.roles.contains("SEC") };

@id("rel.adm") @role("ADM")
permit (principal, action in [Akr::Action::"release.read", Akr::Action::"release.verify",
        Akr::Action::"release.propose", Akr::Action::"release.applied"], resource is Akr::System)
when { principal.roles.contains("ADM") };

// Vendor support (SP-64). The VS role holds nothing but the self group (section 3.8).
@id("ss.adm") @role("ADM")
permit (principal, action in [Akr::Action::"support_session.read", Akr::Action::"support_session.open",
        Akr::Action::"support_session.end", Akr::Action::"support_bundle.register"], resource is Akr::System)
when { principal.roles.contains("ADM") };

@id("ss.sec") @role("SEC")
permit (principal, action in [Akr::Action::"support_session.read", Akr::Action::"support_session.approve"], resource is Akr::System)
when { principal.roles.contains("SEC") };

@id("ss.vs_only_self")
forbid (principal, action, resource)
when { principal.roles.contains("VS") && !(action in Akr::Action::"self") };

// Help centre (X5) and the SIEM pull API (SP-43). The pull is for a service principal (phase 01 ClientCredentials).
@id("help.read.all")
permit (principal, action == Akr::Action::"help.read", resource is Akr::System);

@id("siem.pull.service")
permit (principal, action == Akr::Action::"siem.pull", resource is Akr::System)
when { principal.kind == "service" };
```

Checks of the file (section 6): the policy set is served by the phase 03 simulator for each role in the matrix; `akr-ctl policy matrix` lists every action of this file under the role that permits it; the route table is covered in both directions by the phase 03 coverage test: every route of section 2 names a scope that admits each role its permit names (`ar.*` admits SEC and M through `security.ops` and `dashboard.read`; `bg.request.*`, `bg.end.*` and `bg.read.a2` through `self`; `bg.approve.*` through `security.ops` and `dashboard.read` (CH holds `dashboard.read`); `release.*` through `admin.system` and `security.ops`; `support_session.approve` through `security.ops`).

**Attributes the entity adapter fills for this file** (no schema change): `User.roles`, `User.kind` (phase 03). `Case.assignees` gains the break-glass requester only under purpose `break_glass` (section 3.2 "Use" step 2). Nothing else.

### 3.13 Audit events emitted

Each mutation writes one event of the phase 02 catalogue (`<domain>.<verb>`). `SIEM` is the `siem` flag of the catalogue entry; the `resource` column names the resource type, and no event of this phase carries a case id, a party name or free text. Detail keys are ids, codes, counts and times only.

| Event kind | On | Resource | SIEM |
|-|-|-|-|
| `access_review.started` | review started (`POST /admin/access-reviews`) | access_review | no |
| `access_review.snapshotted` | snapshot built (job; item count, evidence id) | access_review | no |
| `access_review.item_decided` | confirm or flag (decision, note code) | user | no |
| `access_review.revoke_proposed` | revocation proposed (change request id, grant id) | user | yes |
| `access_review.signed` | review signed (counts by decision) | access_review | yes |
| `access_review.overdue` | review past `due_on` (once) | access_review | yes |
| `break_glass.requested` | request recorded (reason code; the same for any case id) | break_glass | yes |
| `break_glass.approved` | first or second approval (role of the approver) | break_glass | yes |
| `break_glass.activated` | second approval; time box (expires at) | break_glass | yes |
| `break_glass.used` | a read under the grant (action kind, grant id; no case id) | break_glass | yes |
| `break_glass.ended` | ended early by the requester or SEC | break_glass | yes |
| `break_glass.expired` | time box passed (sweep) | break_glass | yes |
| `break_glass.lapsed` | request not approved within its lifetime (sweep), or ended by its requester | break_glass | yes |
| `break_glass.reviewed` | review recorded (outcome code) | break_glass | yes |
| `break_glass.review_overdue` | review not recorded within `break_glass.review_hours` (sweep, once) | break_glass | yes |
| `release.imported` | bundle directory imported (version, manifest digest) | release | yes |
| `release.verified` | verification passed (key fingerprint, counts) | release | yes |
| `release.refused` | verification refused (refusal code) | release | yes |
| `release.proposed` | change request for approval created | release | yes |
| `release.applied` | operator's apply recorded (previous release superseded) | release | yes |
| `support_session.requested` | session requested (reason code, hours) | support_session | yes |
| `support_session.started` | approved; VS grant starts (time box in seconds) | support_session | yes |
| `support_session.ended` | ended by ADM | support_session | yes |
| `support_session.expired` | time box passed (sweep; grant revoked) | support_session | yes |
| `support_bundle.registered` | archive confirmed and registered (size, key version) | support_bundle | yes |
| `support_bundle.archive_deleted` | archive file deleted after its retention | support_bundle | no |
| `help.imported` | articles of a release written | release | no |
| `anchor.timestamped` | RFC 3161 token stored (TSA name) | audit_anchor | no |
| `siem.dead_lettered` | a line moved to the dead-letter table (reason code) | siem | no |
| `user.updated` (phase 03 kind) | upstream link set or removed (field name `upstream_link`) | user | yes |
| `role_grant.revoked` (phase 03 kind) | revocation applied by the change request (section 3.1) | role_grant | yes |
| `role_grant.created` (phase 04 kind) | not written for the VS grant: the start is `support_session.started` | role_grant | yes |
| `change_request.*` (phase 04 kinds) | the revoke, release and approval requests | change_request | yes (phase 04 section 3.14) |

The SIEM column is the flag of the new kinds; the kinds of phases 03 and 04 keep their flags. `siem.dead_lettered` is not exported, so a dead letter cannot loop into the stream. `support_bundle.archive_deleted` is not exported because it is an operations fact with no security consequence (a reviewer may change this, Open doubts 18).

### 3.14 Tipping-off review (what a person outside a case's access list can learn; the answer must be "nothing")

| Surface | What a reader outside the case's access list can learn | Answer |
|-|-|-|
| The SIEM stream and the pull API (3.5) | Kind, time, actor id and, for a restricted or suspicion resource, no id, no key, no details and no case id. A `break_glass.used` or `authz.denied` line shows that a masked read or refusal happened at a time by a named staff id, never which record. The SOC sees that a refusal happened, which SP-36 requires it to see. | Nothing about a case, party or record. Attempts are visible to the SOC as attempts only. |
| The break-glass request and approval (3.2) | The requester's 202 is the same for any case id, and the response to the approvers shows a request id, the reason code and the requester, never the case id. A requester learns the existence of a case only by reading it under two approvals. | Nothing to a person outside the grant. Approvers learn nothing about existence. |
| The break-glass notices (3.3) | Counts only, with no case, party, requester or reason text. | Nothing. |
| The break-glass list and the control report (3.2, phase 24 3.2) | The list omits the case id for all readers but the requester; IA sees counts. | Nothing to IA or SEC beyond counts. |
| The access-review snapshot and items (3.1) | Users, role codes, grant ids and decisions of staff. No case, party or suspicion-tier fact enters the snapshot: a case assignment is not a grant. | Nothing about cases. Staff information is shown to SEC and M only. |
| The release record and its notes (3.6, 3.9) | Vendor text about versions, SBOM names, refusal codes and articles. No bank data and no customer data. | Nothing. |
| The support session and bundle (3.8) | The vendor sees its own session and time box. The archive is an allow-list of operational fields with pseudonymised ids and no case id or party name; detection-logic parameters are excluded unless the bank ticks the box. | Nothing about a case or a party reaches the vendor. |
| The help articles (3.9) | Vendor text only. | Nothing. |
| The operations tiles (3.10) | Counts of grants, sessions, lines and reviews. The tile for break-glass shows a count with no case id. | Nothing beyond counts that the cockpit's readers (ADM, SEC) already see by role. |

### 3.15 Settings used

| Key | Value type | Governed | Floor | Used by |
|-|-|-|-|-|
| `access.review_interval_days` | integer | TRUE | `{"max": 92}` | 3.1 start |
| `break_glass.max_hours` | integer | TRUE | `{"max": 4}` | 3.2 approve (the table CHECK holds 4 hours) |
| `break_glass.review_hours` | integer | TRUE | `{"max": 24}` | 3.2 review and sweep |
| `break_glass.request_ttl_min` | integer | FALSE | none | 3.2 request |
| `support.session_max_hours` | integer | TRUE | `{"max": 8}` | 3.8 request |
| `support.bundle_retention_days` | integer | TRUE | `{"max": 30}` | 3.8 purge |
| `support.bundle_max_mb` | integer | FALSE | none | 3.8 CLI and register |
| `siem.batch_size` | integer | FALSE | none | 3.5 |
| `siem.max_lag_s` | duration_s | FALSE | none | 3.5 and tile |
| `siem.attempts_before_dead_letter` | integer | FALSE | none | 3.5 |
| `siem.export_access_events` | boolean | TRUE | none | 7.23 item 6 |
| `release.currency_days` | integer | FALSE | none | 3.10 |
| `audit.rfc3161_enabled` | boolean | TRUE | none | 3.7 |
| `auth.upstream_mode` | string | TRUE | none | 3.4 |
| `schedule.break_glass_sweep`, `schedule.access_review_sweep`, `schedule.support_session_sweep`, `schedule.support_bundle_purge`, `schedule.siem_forward` | cron | FALSE | none | section 4.1 |
| `session.idle_timeout_s`, `session.absolute_lifetime_s` (phase 01, governed) | duration_s | TRUE | phase 01 | 3.4 |
| `assessor.grant_max_days` (phase 24, governed, max 30) | integer | TRUE | `{"max": 30}` | 3.7 |

Settings are read through `SettingsService` with the phase 00 cache. A governed key changes by change request; the four-hour, 24-hour, 92-day, eight-hour and 30-day floors are the bounds of the side that weakens the control (a longer time box, a longer review interval, a longer archive).

### 3.16 Lock order

When two paths take the same locks, both take them in this order (phase 17 rule, conventions):
1. Advisory locks first: `access_review` (start), `break_glass:<grant id>`, `support:<vendor user id>`, `siem_forward`.
2. Then the parent row: the access review, the break-glass grant, the support session, the release, the SIEM cursor.
3. Then the child row: the review item; then the change request (phase 04 order: the change request row before the target row it changes).
4. Then the role grant (`admin.role_grants`, `FOR UPDATE` by id) and `invalidate_user` after commit.
Sweeps process one parent row per transaction in ascending id order, so a sweep and an interactive decision on the same row serialise on the row lock and the second one sees the new status.

## 4. Infrastructure Updates

### 4.1 Job kinds (Postgres `jobs`; coalesce key in brackets)

| Kind | Binary | Payload | Concurrency, attempts | Schedule or trigger |
|-|-|-|-|-|
| `access_review_snapshot` | worker | `{ review_id }` [`access_review_snapshot:<id>`] | 1, 3 | enqueued by `POST /admin/access-reviews` (section 3.1) |
| `access_review_sweep` | worker | `{}` [`access_review_sweep`] | 1, 3 | `schedule.access_review_sweep` (06:10 bank time) |
| `break_glass_sweep` | worker | `{}` [`break_glass_sweep`] | 1, 3 | `schedule.break_glass_sweep` (every minute); lapses, expiries and overdue reviews (section 3.2) |
| `siem_forward` | worker | `{}` [`siem_forward`] | 1, 5; lease 60 s | `schedule.siem_forward` (every 10 seconds); advisory lock, section 3.5 |
| `support_session_sweep` | worker | `{}` [`support_session_sweep`] | 1, 3 | `schedule.support_session_sweep` (every minute); ends and refuses sessions (section 3.8) |
| `support_bundle_purge` | worker | `{}` [`support_bundle_purge`] | 1, 3 | `schedule.support_bundle_purge` (03:40 bank time) |
| `release_verify` | worker | `{ release_id }` [`release_verify:<id>`] | 1, 1 | enqueued by `POST /admin/releases/verify`; deterministic, not retried (section 3.6) |
| `anchor_timestamp` | worker | `{ anchor_id }` [`anchor_timestamp:<anchor_id>`] | 1, 5 | enqueued by the anchor job of phase 02 only when `audit.rfc3161_enabled` is `true` (section 3.7) |

Every handler re-reads its owner first. No job reaches the core banking system. The job names do not use the `ingest_` prefix. The `siem_forward` job and the pull route read the audit log through `audit.siem_page` only.

### 4.2 Timers and outbox

No timer kind is added: the four-hour end is enforced by the predicate at read time (section 3.2) and recorded by the one-minute sweep, which is a cron job and not a timer. No outbox topic is registered or consumed by this phase. The reserved topic `siem.export` (phase 00) is released, not used (section 3.5, DR-235). This phase produces the existing topic `notify.request` for the two codes of section 3.3, which phase 22 consumes; no second handler is added to any topic.

### 4.3 AppState additions

- `akr-api`: `access_reviews: Arc<AccessReviewService>`, `break_glass: Arc<BreakGlassService>`, `releases: Arc<ReleaseService>`, `support_sessions: Arc<SupportSessionService>`, `help: Arc<HelpCentreService>`, `siem_pages: Arc<dyn SiemRepository>` (the pull route only), `oidc_config: Arc<OidcUpstream>` (read-only, for the BFF's `auth.upstream_mode`).
- `akr-worker`: the services above (jobs), `siem: Arc<SiemForwarder>`, `anchor_timestamps: Arc<AnchorTimestamper>` (only started when `audit.rfc3161_enabled`), `bundle_trust: Arc<BundleTrustRoots>` (the phase 00 verifier keys, read from OpenBao path `kv/release/trust`).
- `OpsCockpit` (phase 03) gains the tiles of section 3.10 through `snapshot()`.

### 4.4 Network zones, metrics, readiness, OpenAPI

- **Zone rules** (`deploy/zones.yaml`, checked by `make zone-check`; SP-04, DR-235): `akr-worker` -> `z2_bank` (new network, not internal, attached to `akr-worker` only, egress limited by the rendered nftables rules to the SIEM collector address and the TSA address declared at install with `akr-ctl siem configure`); `kratos` -> `z3_idp` (new network, not internal, attached to `kratos` only, egress limited to the bank's OIDC issuer address when `auth.upstream_mode = oidc`; the rule is absent in `local_only`); `akr-api` -> no new destination. Every other rule of phase 00 stands. The `log-forwarder` (Fluent Bit) keeps its JSON stdout path; the SIEM path of this phase is the forwarder's own and does not replace it.
- **Metrics** (phase 00 OTel and `/metrics`, labels without ids): `akr_siem_lag_seconds`, `akr_siem_forwarded_total`, `akr_siem_dead_letters_open`, `akr_break_glass_active`, `akr_break_glass_review_overdue`, `akr_access_reviews_overdue`, `akr_support_sessions_active`, `akr_release_applied_age_days`, `akr_anchor_timestamp_unverified`.
- **Readiness**: no change. The SIEM forwarder reports unready only when the lag of section 3.5 step 7 is above `siem.max_lag_s` for 3 consecutive checks (a warning on the tile, not a readiness failure for `akr-worker`, so a SIEM outage does not restart the worker).
- **OpenAPI**: tag `platform` for every route of section 2 except the pull route, which takes tag `siem`; `make generate-api` regenerates the Orval client in the same change (phase 00 drift check).

### 4.5 Existing entity, schema, route and code updates (extensions of earlier phases)

- **Phase 01 `admin.role_grants`** (migration `0081`, DR-237): `support_session_id` column; `source` CHECK widened to `support_session` (the constraint `role_grants_source_check` is replaced); a new CHECK `role_grants_support_session_check` (VS only from a session, with `valid_to`). The existing `change_request` CHECK is unchanged. No other phase 01 column or constraint changes.
- **Phase 01 sign-in**: `auth.upstream_mode` (governed) and the Kratos OIDC configuration of section 3.4. `akr-ctl identity link-upstream` and `unlink-upstream` are new subcommands of the phase 01 `akr-ctl identity` group. The SIEM pull principal is a phase 01 service principal (`admin.users.kind = 'service'`, `expires_at` within one year, token allow-list `siem.pull`).
- **Phase 02 audit catalogue**: the kinds of section 3.13 are appended to `CATALOGUE` with their `siem` flags; the stripping rule of section 3.5 is implemented in `audit.siem_page` (the phase 02 section 3 note is honoured). No phase 02 table, function or grant changes. The RFC 3161 token is a separate table (`admin.anchor_timestamps`), so the immutable anchor row is untouched.
- **Phase 03 `OpsCockpit::snapshot()`**: the tiles of section 3.10. The `break_glass` purpose code, the `system_admin` group and `AccessMatrixBuilder::build(at)` are used as they are. API.md 3.2 `GET /admin/access-reviews` (owned) gains the scope `dashboard.read`.
- **Phase 03 `role_grant.revoked`**: written by the revoke operation of section 3.1; the phase 03 "revocation at once" route `DELETE /admin/users/{id}/grants/{grant_id}` is unchanged.
- **Phase 04 `ChangeTargetRegistry`**: (a) `role_grant` gains the operation `revoke` (maker SEC, route `cr_access_adm`, checker ADM; apply as in section 3.1; this operation replaces the "not allowed" entry of the target for revoke); (b) a new target kind `platform_release` (route `cr_access`, key `release:<release_version>`, `volume_effect = neutral`, evidence kind `package` with owner kind `release`, and its `EvidenceResolver` registered in the same change, as phase 04 requires for evidence kinds); the route map of phase 04 section 3 gains `platform_release` beside `role_grant`.
- **Phase 17 visibility** (`services::visibility::build`, phase 17 section 3.10 rule 5) and **phase 18 case rows** (phase 18 section 3.10, the rule-5 clause): one admitting clause for an active break-glass grant of the requester (section 3.2 "Use" step 1). The Cedar `Case.assignees` adapter adds the requester under purpose `break_glass` only. A test pins the SQL clause and the adapter to each other (phase 17 section 6 check 6 is extended).
- **Phase 22 notifications**: `NoticeCode` gains `SecurityBreakGlassWaiting` and `SecurityBreakGlassOpened` with their two seed rows in `0082`; the dispatcher's recipient rule gains "active holders of CH or SEC" and "active holders of SEC or M" for those codes (DR-243). The fifteen-code list of phase 22 becomes seventeen codes.
- **Phase 24 control report and register**: `GET /access/control-report` (phase 24) gains two counts with the small-number suppression of phase 24: break-glass grants activated in the month, and access reviews overdue. The register receives the warnings of sections 3.1 and 3.2 (`source` `review_calendar` and `scan`) through its existing `uq_registers_source_key` index.
- **Phase 24 `pack.*`**: the pack routes refuse purpose `break_glass` through file 51 (section 3.12); no change to the pack services.
- **Phase 00 settings and jobs**: the keys of section 3.15, the cron keys of section 4.1 and the job kinds above. The `akr-ctl` command group gains `support-bundle create` and `support-bundle confirm`; the release job calls the phase 00 `bundle verify` code path, which is not changed.
- **API.md** (this phase owns its rows): the lead adds the rows of section 2 marked *added* to section 3 of `docs/API.md` (this phase does not edit API.md); the owned rows `GET /admin/access-reviews` and `GET /admin/releases` change their scope as stated in section 2; the lead adds the scope `siem.pull` to API.md section 2 (new in this phase, Open doubts 5).
- **DESIGN-SYSTEM**: screens A4, X5, G5 (as specified in section 5); A3 gains the support section and the tiles; no catalogue row is added.

## 5. Frontend Impact

Every screen below uses the Orval hooks named from the paths of section 2. The chrome, rail entries and bilingual labels follow DESIGN-SYSTEM sections 8 and 11. No screen shows a machine id, a case id or a party name in a notice or a list label.

### 5.1 A4 Release and updates (`/admin/releases`; STEPPER; ADM and SEC read)

- Steps: (1) import and verify: a directory name on the import volume, then the verification status with the refusal reason in words; (2) review: release notes in Arabic and English, the SBOM diff (counts and names), the policy matrix diff (roles and actions added or removed), the rollback plan and the key fingerprint; (3) propose: creates the change request and shows its link; (4) approval: the change request state (SEC approves in phase 04's queue; this screen only links to it); (5) applied: the operator records the apply with step-up and a note that the platform did not apply anything.
- Hooks: `useGetAdminReleases`, `usePostAdminReleasesVerify`, `usePostAdminReleasesIdPropose`, `usePostAdminReleasesIdApplied`. The list uses `useGetAdminReleases` with polling every 10 seconds while a release is `verifying`.
- Refusal and currency: a refused release shows its refusal reason and no next step; a currency banner shows the age of the applied release against `release.currency_days`.

### 5.2 X5 Help centre (`/help`; reading; every role)

- Tabs: "Help" (articles), "Glossary" (the terms of compliance staff in both languages), "What changed" (one entry per installed release, newest first).
- Hook: `useGetHelpArticles` with `kind` as the tab filter. Language follows the user's locale with the other language as fallback.

### 5.3 G5 Access and insider controls (`/access`; REGISTER; SEC, M, CH read; ADM none)

- New section "Access reviews" (SEC and M): the review list (`useGetAdminAccessReviews`), a detail with its items (`useGetAdminAccessReviewsId`), item decisions (`usePostAdminAccessReviewsIdItemsItemId` with `If-Match` on the item version), start (`usePostAdminAccessReviews`, SEC) and sign (`usePostAdminAccessReviewsIdSign`, SEC, step-up dialog). The "propose revocation" button is shown only to SEC and creates a change request; the screen shows the request's state.
- New section "Emergency access" (A2 requests; SEC and CH approve; SEC reviews): the list (`useGetAccessBreakGlass`), request (`usePostAccessBreakGlass`, shows the same confirmation whatever the case id; the form asks for the case id and the reason from a short list), approve (`usePostAccessBreakGlassIdApprove`, step-up), end (`usePostAccessBreakGlassIdEnd`), review (`usePostAccessBreakGlassIdReview`, SEC). The countdown uses the server `expires_at`, never the client clock. The section header uses the bilingual label "Emergency access (break-glass)" of DESIGN-SYSTEM section 11.
- Tiles fed to G1 and this screen: active grants, overdue reviews, overdue break-glass reviews (counts only).

### 5.4 A3 Platform health (`/admin/health`; DASH; ADM; additions only)

- Tiles of section 3.10: SIEM lag, SIEM dead letters, release currency, support sessions open, anchor time stamps unverified (when RFC 3161 is on).
- New section "Vendor support" (ADM requests and ends; SEC approves): list (`useGetAdminSupportSessions`), request (`usePostAdminSupportSessions`, reason code, ticket reference, hours up to 8), approve (`usePostAdminSupportSessionsIdApprove`, SEC, step-up), end (`useDeleteAdminSupportSessionsId`), and the bundle record (`usePostAdminSupportSessionsIdBundles`, step-up class `export`). The screen shows the `akr-ctl support-bundle` command for the operator to run on the host; it does not upload the archive.

### 5.5 Generated hook summary

`useGetAdminAccessReviews`, `useGetAdminAccessReviewsId`, `usePostAdminAccessReviews`, `usePostAdminAccessReviewsIdItemsItemId`, `usePostAdminAccessReviewsIdSign`, `useGetAccessBreakGlass`, `usePostAccessBreakGlass`, `usePostAccessBreakGlassIdApprove`, `usePostAccessBreakGlassIdEnd`, `usePostAccessBreakGlassIdReview`, `useGetAdminReleases`, `usePostAdminReleasesVerify`, `usePostAdminReleasesIdPropose`, `usePostAdminReleasesIdApplied`, `useGetAdminSupportSessions`, `usePostAdminSupportSessions`, `usePostAdminSupportSessionsIdApprove`, `useDeleteAdminSupportSessionsId`, `usePostAdminSupportSessionsIdBundles`, `useGetHelpArticles`. (The SIEM pull route has no UI hook.)

### 5.6 Browser and caching rules

- No `localStorage` or `sessionStorage` holds an approval, a grant, a decision or a session state. A draft review note may stay in memory only.
- Lists are never cached across a decision: the mutation's response replaces the row, and the list is invalidated.
- Every countdown reads `expires_at` from the server response; a stale page shows "updated at" and refreshes on focus.

## 6. Verification

1. `make migrate`: `0080` to `0082` apply cleanly on a database at `0079`, and a second run is a no-op. Then `\d admin.role_grants` shows `support_session_id` and the constraint `role_grants_source_check` with three values.
2. `cargo check`: compile-time SQL passes for the new queries, including `audit.siem_page` (called with `$1::timestamptz, $2::uuid, $3::int, $4::text[]`).
3. `cargo clippy -- -D warnings`.
4. `cargo test`, including the following database tests on a throwaway PostgreSQL 17 (run as `akr_migrate` for setup and `akr_app` for the assertions):
   - a `VS` grant without a session is refused (`role_grants_support_session_check`); a `support_session` grant without `valid_to` is refused;
   - a break-glass grant with `expires_at` 4 hours and 1 second after `granted_at` is refused by the CHECK; an `UPDATE` that extends `expires_at` is refused by `break_glass_guard`; the same row with `reviewed_at` set refuses every further update;
   - `UPDATE` and `DELETE` on `admin.break_glass_log` and `admin.help_articles` raise `insufficient_privilege`;
   - `SELECT` on `audit.audit_log` by `akr_app` is refused (column and table grants of phase 02 stand); `EXECUTE` of `audit.siem_page` by `akr_readonly` is refused;
   - `audit.siem_page` on a fixture with one `normal` row and one `suspicion` row returns no `case_id` or `tier` column, returns `resource_id` NULL and `details` `{}` for the suspicion row, and returns only the kinds named in `p_kinds`;
   - a `UPDATE` of `admin.releases.notes` on a verified row is refused by `release_guard`;
   - the column list of every `GRANT UPDATE` of section 1 matches the columns the repositories update (a test greps the repository SQL and compares).
5. curl suite `tests/curl/phase25_platform.sh`. Dev users: the conventions list (`ds@`, `cfg@`, `chk@`, `m@`, `ch@`, `ia@`, `ss@`, `kyc@`, `a1@`, `a2@`, `a2b@`, `adm@`, `sec@`, `mro@`) plus two users this suite adds: `sec2@bank.test` (a second SEC holder, needed for four-eyes steps) and `vs@bank.test` (a vendor user with no role, the support session subject). A suspicion-tier case `S1` and a normal case `N1` are seeded by the phase 17 fixtures. Each request sends `Idempotency-Key` where a mutation is listed; step-up is sent with the dev assertion helper of phase 01 for each class listed.
   - **403 Cedar deny (own grant)**: `m@` calls `POST /api/v1/admin/access-reviews/{id}/items/{own_item}` with `decision: confirmed` and the current `If-Match` → `403` `E_FORBIDDEN`.
   - **403 scope deny**: `ds@` calls `GET /api/v1/admin/access-reviews` → `403` `E_SCOPE`.
   - **Break-glass refused when the requester approves**: `a2@` calls `POST /api/v1/access/break-glass` with `case_id = N1` (step-up `break_glass`) → `202` `{"status":"requested"}` with no id; `a2@` calls `POST /api/v1/access/break-glass/{id}/approve` on the same request → `403` (`E_SCOPE` at the gate, as A2 holds neither `security.ops` nor `dashboard.read`); `sec@` calling `POST /api/v1/access/break-glass` → `403` `E_FORBIDDEN` (no permit). The handler's `E_SELF_APPROVAL` for a person holding A2 and SEC is a unit test. The id is read from the database in the fixture.
   - **Masking before approval**: `a2@` posts the same body for `S1` and for a random UUID; the two response bodies are byte-equal (`diff` in the script) and both `202`.
   - **Activation and read-only use**: `a2@` requests for `S1`; `ch@` approves (`200`, status `requested`); `sec@` approves (`200`, status `active`, `expires_at - granted_at` = 4 hours). `a2@` reads `GET /api/v1/cases/{S1}` with `X-Access-Purpose: break_glass` and the analyst network header → `200`; the same read without the purpose → `404` `E_NOT_FOUND`; `a2@` posts a case note with the purpose → `403` `E_FORBIDDEN` (file 51, `bg.read_only`).
   - **The grant ends at its time box**: the fixture sets the grant's `expires_at` to `now() + 2 seconds` through `akr_migrate`; after 3 seconds `a2@` reads `S1` with the purpose → `404`; the sweep (`akr-ctl jobs run break_glass_sweep`) sets `status = expired`; `GET /api/v1/access/break-glass` shows `expired` and the audit trail holds `break_glass.expired`.
   - **Review within 24 hours**: `sec@` posts `review` with `outcome_code = appropriate` → `200`; a second review → `409` `E_NOT_ALLOWED_IN_STATE`.
   - **Access review and idempotency replay**: `sec@` posts `POST /api/v1/admin/access-reviews` with key `K1` → `202`; the same request with `K1` returns the same review id and the replay header; the same key with another body → `422` `E_IDEMPOTENCY_REUSE`.
   - **Self-signature refused and If-Match**: with all items decided, `sec@` (the starter) signs → `403` `E_SELF_APPROVAL`; `sec2@` signs with the step-up `approval` → `200` `signed`. An item decision without `If-Match` → `428` `E_PRECONDITION_REQUIRED`; with a stale version → `409` `E_VERSION_CONFLICT`.
   - **Revocation through the change path**: `sec@` posts `revoke_propose` on a grant → `200` with `change_request_id`; `adm@` approves the change request in phase 04 (`cr_access_adm`, checker ADM) → the grant has `revoked_at` set and the audit trail holds `role_grant.revoked`; the grant row is unchanged except those columns.
   - **Support session and VS**: `adm@` posts `POST /api/v1/admin/support-sessions` for `vs@` (`requested_hours = 2`) → `202`; `sec@` approves with step-up → `200` `active`; `vs@` calls `GET /api/v1/me` → `200`; `vs@` calls `GET /api/v1/cases/{N1}` and `GET /api/v1/cases/{S1}` → `403` `E_SCOPE` for both (VS holds no `queue.work`; the gate answers before any lookup, so nothing tells the two apart); `adm@` calls `DELETE /api/v1/admin/support-sessions/{id}` → `200`; `vs@` calls `GET /api/v1/me` → `401` `E_UNAUTHENTICATED`.
   - **Support bundle needs step-up**: `adm@` posts the bundle record without step-up → `401` `E_STEPUP_REQUIRED`; with the `export` assertion → `201`.
   - **SIEM line with no case id**: `siem-pull` (a service principal of phase 01 with `siem.pull`) calls `GET /api/v1/siem/events?limit=500` → `200`; no event carries a `case_id` or `tier` key; the fixture event on `S1` has `resource_id` null and `details` `{}`; a user token calling the same route → `403` `E_SCOPE`.
   - **SIEM back-pressure and dead letter**: the collector stub is stopped for 2 minutes; the cursor does not move, `akr_siem_lag_seconds` grows and the tile is `warn`; on restart the forwarder catches up with no gap (the sequence of audit ids forwarded equals the audit log order); a fixture line the stub refuses is dead-lettered after `siem.attempts_before_dead_letter` attempts and the cursor moves past it.
   - **Release with a bad signature refused**: `adm@` posts `POST /api/v1/admin/releases/verify` with `bundle_ref = fixture-bad-signature` → `202`; after the job, `GET /api/v1/admin/releases` shows `status = refused`, `refusal_code = bundle_signature_invalid`; no `admin.help_articles` row exists for that release; the audit trail holds `release.refused`.
   - **Release approval and apply**: `adm@` verifies `fixture-good` (`verified`), proposes it (`proposed`, change request), `sec@` approves the change request in phase 04 (`approved`), `adm@` records the apply with step-up (`applied`, the earlier applied release `superseded`). A replay of the verify with the same key → the same id.
   - **Help centre**: `a1@` calls `GET /api/v1/help/articles?kind=what_changed` → `200` with the articles of the installed release; the import of `fixture-bidi` (a bidirectional control character in an article) ends the verify job with the release `refused` and `refusal_code = help_article_invalid`.
   - **Masked 404 for suspicion-tier resources**: `a1@` reads `S1` → `404` with the same body as an unknown id.
6. `make generate-api` regenerates the Orval client: the hook names of section 5.5 exist and the drift check is clean; Swagger UI lists the routes of section 2 under the tags `platform` and `siem`.
7. The audit trail contains, for the curl session: `access_review.started`, `access_review.snapshotted`, `access_review.item_decided`, `access_review.revoke_proposed`, `access_review.signed`, `break_glass.requested` (one for each of the three requests), `break_glass.approved`, `break_glass.activated`, `break_glass.used`, `break_glass.expired`, `break_glass.reviewed`, `support_session.requested`, `support_session.started`, `support_session.ended`, `release.imported`, `release.refused`, `release.verified`, `release.proposed`, `release.applied`, each once per operation, and no event carries a case id in its payload (a test greps `audit.audit_log.details` and `request` for the id of `S1` and `N1`).
8. Route coverage: the phase 03 coverage test reads the OpenAPI document and the router and fails on any route of section 2 without a Cedar decision, and on any Cedar permit whose route scope does not admit its role (section 3.12 check).
9. Existing tests still pass, including the phase 17 visibility-against-Cedar test (extended with the break-glass clause and the purpose-keyed assignee rule), the phase 22 notice-code test (seventeen codes), and the phase 24 control report suppression test.
10. For the SIEM: `akr-ctl policy matrix` and the release check of phase 08 run on the release bundle with the policy matrix diff of section 3.6 step 3; the zone check (`make zone-check`) shows `akr-worker` reaching only the collector and TSA addresses and `kratos` reaching only the issuer address.

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 1: corrections found while checking phases 00 to 02 (tools/agent/gap1-run1-verdict.md); lead-verified.

### 7.1 KV paths and the support-bundle key
- KV paths are relative to the one mount `akr/` (phase 00 7.5, DR-356): `kv/release/trust` is `release/trust` and
  `kv/kratos/oidc/<provider>` is `kratos/oidc/<provider>`.
- The transit key `akr-sign-bundle` is entered in the key inventory (phase 00 7.6, DR-357): signed by `akr-ctl` on the
  bank host under an operator policy. The key part of Open doubts 9 is closed; the disclosure-register part stays open.

Gap pass 1 (2026-10-10), run 2: corrections found while checking phases 03 and 04 (tools/agent/gap1-run2-verdict.md); lead-verified.

### 7.2 The assessor grant cap
Every mention of `assessor.grant_max_days` (the settings table and Open doubts 15) reads `access.assessor_max_days`
(phase 04 7.10, DR-361); the cap stays 30 days.

Gap pass 1 (2026-10-10), run 4b, from tools/agent/gap1-run4b.md; lead-verified (tools/agent/gap1-run4b-verdict.md).

### 7.3 Tables excluded from the support bundle and the SIEM (phases 27, 28, 29)
The support bundle (section 3.8 step 1) writes no row count and no row content for these tables, and the SIEM export
(section 3.5) reads the audit log only, so they reach it only through audit payloads, which carry ids and counts:
`alert.model_scores` (phase 27), `casework.assistant_drafts` and `admin.assistant_evaluations` (phase 28),
`assess.decisions`, `assess.holds`, `assess.hold_candidates` and `assess.release_exceptions` (phase 29). Test
(section 6): the bundle fixture's row-count list contains none of the seven names.

### 7.4 Releases do not import models or assistant versions (DR-390)
The release path verifies the bundle only (section 3.6). Model and assistant artifacts that a bundle carries are not
imported by the verify job: people import them through phase 27 section 3.3 and phase 28's version import, which set
status `registered` and nothing else (GV-29). A release never sets a model or an assistant version live, shadow or
challenger, and its status does not depend on those imports.

### 7.5 Policy matrix file hash in the release check
Verify job step 3 also stores the SHA-256 of the JSON output of `akr-ctl policy matrix --bundle <dir> --json` (the
bytes that `make generate-api` writes to `frontend/src/app/policy-matrix.json`, phase 03) as the key
`matrix_file_sha256` inside `policy_matrix_diff`. This is the hash phase 03 says the release check records. No new
column; a difference is shown in the diff and is not a refusal.

### 7.6 Runbook: rotation of `keys/auth-hmac`
Phase 01 puts this runbook in phase 25. The KV entry `keys/auth-hmac` (DR-356; the path is relative to the mount `akr/`, as in 7.1) holds the master of the HKDF
subkeys `csrf`, `token` and `login_identifier` (phase 01). Steps for the bank operator on the bank host:
1. Write a new KV version of `keys/auth-hmac` with a fresh random value, under the operator's OpenBao policy.
2. Restart `akr-api` so every replica derives the subkeys from the new version.
3. Tell the holders of personal tokens that every token is invalid and must be created again.
4. Check: a request with a CSRF cookie minted before the rotation gets a new cookie; an old personal token is refused.
The effects are the ones phase 01 states: CSRF cookies are re-minted at the next request and every personal token
becomes invalid. Sessions are not claimed to end.

### 7.7 Chart and K3s extensions owned by phase 30
Phase 30 owns these changes (its migration `0095` and section 3.12): the CHECK of `admin.releases.refusal_code`
(section 1.2) gains `chart_signature_invalid`; the verify job gains check (g), the Helm chart check, after check (f),
and the refusal order of step 4 ends at (g); `cosign_result` gains the chart count and the K3s version (no signature
bytes); A4 shows both facts; `release.refused` carries the new code.

### 7.8 Rollout record on the apply
`POST /admin/releases/{id}/applied` takes an optional `rollout_evidence_id`. The operator first uploads the
`rollout.json` that phase 30's upgrade script writes, as evidence of kind `package`, owner kind `release` and owner id
this release (phase 02 evidence store). The route checks that the evidence has that owner (else `E_VALIDATION`). No
column is added: the evidence object's owner link is the record. Open doubts 14 keeps its point: the record is the
operator's file, not a proof of the apply.

### 7.9 No cluster objects in the support bundle (phase 30)
Section 3.8 step 1 collects no Kubernetes object, event, pod log or cluster state (phase 30 section 3.14); the
application logs in the archive are the redacted logs of step 2 only.

### 7.10 Read-only grant check reused by later phases
The database tests of section 6 assert `has_table_privilege('akr_readonly', t, 'SELECT')` is false for each table of
section 1, as a loop over a table list. Later phases add their tables to the same list (phase 26's read-only check of
`kyc.possession_evidence` is one).

### 7.11 Scope of the one-year expiry
The one-year expiry of sections 3.5 and 4.5 applies to the SIEM pull principal only. The expiry of other service
principals, such as the gateway certificates of phase 29, is decided by their phase (phase 29 Open doubts 24).

### 7.12 Canary records are P2 (DR-391)
SP-30 canary records (research 30 sections 8.6 and 8.8) are not built in this plan; they are P2. Phase 24's control
report keeps `canary_not_built: true` (phase 24 section 7+ closes its Open doubts 12).

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.13 SP-36 behaviour rules: what ships and what does not (item 3 replaced by section 7.23) (V-01-07-a, MEDIUM; DR-413)
No phase carried SP-36 (the behaviour rule pack of research 30 section 9.3), and this phase's header named phase 24
for it. Decision: SP-36 is partly implemented here and the rest is P2.
1. Shipped: `deploy/siem/use_cases.yaml` (section 3.5) gains the correlation rules that the exported kinds can feed,
   as entries `{id, title, kinds, group_by: actor, window, threshold}`. The shipped `window` and `threshold` are
   proposed starting values that the bank tunes in its SIEM, not product settings: 10 minutes and 5 events for the
   denied, sign-in and step-up rules, 24 hours and 1 event for break-glass use, 1 hour and 3 events for privileged
   bursts. The rules: repeated denied or conflicting attempts
   (`authz.denied`), sign-in anomalies and repeated step-up failures (the SP-18 kinds of phase 01), break-glass use,
   and privileged bursts (role grants, policy activation, and service client and token changes: `role_grant.created`,
   `policy_bundle.activated`, the `service_client.*` kinds, `token.created` and `token.revoked`). The CI check of section 3.5 covers them: a
   rule may name exported kinds only. The rules run in the bank's SIEM; their recipients are the second line and
   security, never the actor's manager, and the actor is told nothing.
2. Built elsewhere as controls, not as alerts: purpose codes and the conflict check (phase 03), the volume-reduction
   flag and its second approver (phase 04), QA samples (phase 19), and the monthly counts of the control report
   (phase 24 section 3.2).
3. Replaced by section 7.23 (owner decision of 2026-10-10: the access events are exported and these rules ship). The text of gap pass 1 was: Not built (P2): the rules that need the allowed reads, which are not exported (`authz.allowed` has SIEM "no",
   phase 03): mass lookup, off-hours or off-network access, access with no work reason, snooping on staff or
   relatives, export spikes as an alert, and closure patterns; canary access is P2 already (7.12). A behaviour engine
   inside the product over `access_event` is that P2 item.

### 7.14 Step-up constants and sweep counts (V-25-01-e, V-25-01-g, LOW; DR-413)
- This phase declares its step-up actions as `StepUpAction { name, class }` constants with phase 01's closed classes: `BREAK_GLASS_REQUEST` is `{ "break_glass.request", break_glass }`; `BREAK_GLASS_APPROVE` is `{ "break_glass.approve", break_glass }`; `ACCESS_REVIEW_SIGN` is `{ "access_review.sign", approval }`; `RELEASE_APPLIED` is `{ "release.applied", approval }`; `SUPPORT_SESSION_APPROVE` is `{ "support_session.approve", access_change }`; `SUPPORT_BUNDLE_REGISTER` is `{ "support_bundle.register", export }`.
- `SweepCounts` is one type for the four sweeps: a map from a count name to a number (`BTreeMap<&'static str, i64>`). The access review sweep returns `superseded` and `overdue_opened` (section 3.1 sweep step 3); the break-glass sweep `lapsed`, `expired` and `review_overdue`, one count per step; the support session sweep `ended`, `expired` and `refused_timeout`; the bundle purge `archive_deleted`.

### 7.15 `SiemForwarder`: fields and entry point (V-25-02-d, LOW; DR-413)
`pub struct SiemForwarder { siem: Arc<dyn SiemRepository>, settings: Arc<SettingsService>, collector: Arc<dyn SiemCollector> }`. `SiemCollector` is the TLS syslog client of the transport paragraph of section 3.5: `send(line: &str) -> Result<(), PlatformError>` returns `Ok` only on the collector's delivery acknowledgement. The job `siem_forward` calls `pub async fn forward(&self) -> Result<(), PlatformError>`, which runs the steps of section 3.5.

### 7.16 Moves that write no audit event; dead letters (second item replaced by section 7.24) (V-25-02-f, V-25-04-e, LOW; V-25-02-g, MEDIUM; DR-413)
- The rule of section 3.13 has two exceptions, both technical markers and not business data. The advance of `admin.siem_cursors` writes no event, as anchoring writes none in phase 02: an event per batch would be forwarded by the forwarder itself. The move of a release to `verifying` writes no event: it is the job's own marker between `release.imported` and `release.verified` or `release.refused`.
- Replaced by section 7.24 (owner decision of 2026-10-10: an operator resends or resolves a dead letter, and the tile counts the open ones). The text of the verification pass was: Lead decision. Resolving a dead letter is P2. No route or method sets `resolved_at` or `resolved_by` in P1; the two columns, their CHECK, the index and the UPDATE grant of section 1 stay for it. The tile `siem_dead_letters` (section 3.10) therefore counts the dead letters created in the last 7 days, with the same warn and critical lines, so it clears seven days after the last dead letter. The rows stay as the record.

### 7.17 Sub-codes, audit kinds and methods the steps name (V-25-01-c, V-25-03-a, V-25-03-d, V-25-03-e, V-25-04-b, V-25-07-b, V-25-07-e, MEDIUM; V-25-06-d, V-25-07-c, LOW; DR-413)
- Section 3.11 gains the sub-codes `approver_role_required`, `vendor_user_invalid`, `sod_conflict` (phase 04's code, reused), `bundle_not_found`, `manifest_missing` under `E_VALIDATION`, and `session_open` under `E_NOT_ALLOWED_IN_STATE`.
- Section 3.13 gains two rows. `support_session.refused_timeout {session_id}`, SIEM yes, written by the sweep when a session still `requested` after one hour is refused. `access_review.item_superseded {item_id, review_id, grant_id}`, SIEM no, written for each item moved to `superseded` by decide step 4 or sweep step 1. In decide step 4 the move and its event are committed first, and the handler then answers the 409 `grant_changed`.
- `RoleGrantWriter` (the port the `SupportSessionService` struct names) has one method: `insert_support_grant(tx, user_id, support_session_id, scope, valid_from, valid_to, granted_by) -> Result<Uuid, RepoError>`, the insert of Approve step 3 (`role = 'VS'`, `source = 'support_session'`).
- `RoleGrantReader` (the port the `AccessReviewService` struct names) has one method: `in_force_at(conn, at: DateTime<Utc>) -> Result<Vec<RoleGrant>, RepoError>`, with the predicate of snapshot step 3.
- `AccessReviewRepository` gains `insert_items(tx, review_id, &[AccessReviewItem])` (snapshot step 5) and `set_snapshot(tx, id, evidence_id, evidence_sha256, item_count, expected_version)` (snapshot step 6: status `open` with the snapshot identity).
- Tile reads of section 3.10: `BreakGlassRepository::counts(now) -> BreakGlassCounts { active, active_past_expiry, review_overdue }`; `SupportSessionRepository::count_open(now) -> i64`; `AnchorTimestampRepository::unverified() -> (i64, Option<DateTime<Utc>>)` (the count and the oldest; the repository of `admin.anchor_timestamps`, which `AnchorTimestamper` also writes through); `AccessReviewRepository::open_reviews_due(now)` returns the open reviews past `due_on`; the SIEM tiles read `SiemRepository::status()`; `release_currency` reads `ReleaseRepository::find_applied_latest()` and `newest_verified()`, which returns the newest `verified`, `proposed` or `approved` release not yet applied.

### 7.18 The policy bundle hash is compared (V-25-04-a, MEDIUM; DR-413)
Verify job step 3: the hash computed over the canonical bundle is compared with the policy bundle hash that `notes.json` carries (phase 03: the release notes carry it). A difference refuses the release with `policy_bundle_hash_mismatch`, the code the CHECK of section 1.2 already lists. In the refusal order of step 4 this check comes after (f) and before the checks later phases add.

### 7.19 Register warnings go through `RegisterWriter::warn` (V-25-07-a, V-25-07-d, HIGH; DR-413)
Two defects of sweep step 2 of section 3.1: the unique index `uq_registers_source_key` is partial, so PostgreSQL refuses `ON CONFLICT (source_key)` without the predicate `WHERE source_key IS NOT NULL`; and the guard "no `access_review.overdue` event yet" needs a read of the audit trail that `akr_app` does not have. Both sweeps of this phase therefore open their warning with phase 24's `RegisterWriter::warn(tx, source, source_key, title_key, body)` (phase 24 section 7.7), which carries the predicate and returns `None` when the key exists:
- access review (section 3.1 sweep step 2): `warn(tx, review_calendar, 'access_review_overdue:<review_id>', WarningTitle::AccessReviewOverdue, {"review_id"})`; the audit `access_review.overdue` is written only on `Some(id)`, so it is written once.
- break-glass (section 3.2 sweep step 3): `warn(tx, scan, 'break_glass_review:<id>', WarningTitle::BreakGlassReviewOverdue, {"grant_id"})`; the log row `review_overdue` and the audit `break_glass.review_overdue` are written only on `Some(id)`.
This phase adds the two variants to phase 24's closed enum `WarningTitle`: `AccessReviewOverdue` {"ar": "مراجعة الصلاحيات الفصلية متأخرة", "en": "Quarterly access review is overdue"} and `BreakGlassReviewOverdue` {"ar": "مراجعة وصول الطوارئ متأخرة", "en": "Break-glass review is overdue"}. Test: the sweep run twice on an overdue review leaves one warning and one `access_review.overdue` event.

### 7.20 A malformed help article refuses the release (V-25-08-a, HIGH; DR-413)
Lead decision. The articles are part of a signed vendor bundle, so one that fails the schema or holds a refused bidirectional-control character (section 3.9) is a defect of the bundle. The verify job checks every article before step 5; a failure ends the job as step 4 does, with `refusal_code = help_article_invalid` (new value of the CHECK of section 1.2; the last check in the refusal order of this phase) and audit `release.refused`. No article of that bundle is inserted. It is not an HTTP error: the import route has already answered 202 (section 3.11).

### 7.21 Test of the excluded tables (V-25-10-a, LOW; DR-413)
Section 6 check 4 gains the database test that 7.3 cites: the row-count list of the support bundle fixture (section 3.8 step 1) holds none of the seven table names of 7.3.

### 7.22 Upload route of the rollout record (V-25-12-a, HIGH; DR-413)
Section 7.8 says the operator first uploads `rollout.json`; no route did that. The route table of section 2 gains one row:
| POST | `/api/v1/admin/releases/{id}/evidence` | `admin.system` | Upload the `rollout.json` of the apply (multipart, `EvidenceStore::stage`, declared content type `text/plain`: JSON is UTF-8 text, and `application/json` is not in `evidence.allowed_types`): kind `package`, owner kind `release`, owner id this release, retention R10, classification `internal`; answers 201 with the evidence id; IDEM (added; Cedar `release.applied`) |
The release must be `approved` or `applied` (else 409 `E_NOT_ALLOWED_IN_STATE`). Audit: the phase 02 evidence event of `stage`; no new kind.

Owner decisions (2026-10-10): the product owner overturned two lead decisions of this phase (sections 7.13 item 3 and 7.16); lead-written, checked in the owner-decision round (tools/agent/owner-verdict.md).

### 7.23 Reads are exported to the SIEM and the SP-36 rule pack ships (owner decision 1; DR-414)
The owner's decision: allowed reads go to the bank's SIEM too, so the behaviour rules of research 30 section 9.3 ship in this version. This block replaces item 3 of section 7.13 and extends its item 1. Where it differs from sections 3.5, 3.10 and 4.4, it wins. DR-422 locks the correction of the check round of 2026-10-11 to item 6 (which events the off switch holds back).

**1. What is exported.** Every row of `audit.access_event` (phase 02; decisions `allow`, `deny` and `conflict`) is exported as an access line, beside the audit lines of section 3.5. `authz.allowed` keeps its SIEM flag "no" (phase 03): it is the policy decision row of every request, and the read of a record is the access event (SP-35). No change to phases 02 and 03.

**2. Read function.** Migration `0082` gains this function after `audit.siem_page` (written into section 1.3 in the same change):

```sql
-- Access lines for the SIEM (SP-36, owner decision 1). One row per access event after the cursor, in chain order.
-- A row is protected when its stored tier, the current tier of its resource or the current tier of its subject party
-- is not normal: then resource_id and subject_party are not returned. The tier value is never returned.
-- has_work: the actor holds a claimed alert on the subject party, or a live access-list grant on a case that names it.
-- is_staff: the subject party is a staff member or is marked as related to staff (phase 10).
CREATE OR REPLACE FUNCTION audit.siem_access_page(p_after_at TIMESTAMPTZ, p_after_id UUID, p_limit INT)
RETURNS TABLE (
    occurred_at     TIMESTAMPTZ,
    access_id       UUID,
    action          TEXT,
    decision        TEXT,
    actor_id        UUID,
    actor_role      TEXT,
    on_behalf_of    UUID,
    resource_type   TEXT,
    resource_id     UUID,
    subject_party   UUID,
    protected       BOOLEAN,
    purpose_code    TEXT,
    client_ip       INET,
    work_link       BOOLEAN,
    subject_staff   BOOLEAN
)
LANGUAGE plpgsql STABLE SECURITY DEFINER SET search_path = pg_catalog AS $$
BEGIN
    IF p_limit IS NULL OR p_limit < 1 OR p_limit > 5000 THEN
        RAISE EXCEPTION 'siem page size out of range' USING ERRCODE = 'invalid_parameter_value';
    END IF;
    RETURN QUERY
    SELECT e.occurred_at, e.id, e.action, e.decision, e.actor_id, e.actor_role, e.on_behalf_of, e.resource_type,
           CASE WHEN NOT t.is_protected THEN e.resource_id END,
           CASE WHEN NOT t.is_protected THEN e.subject_party END,
           t.is_protected, e.purpose_code, e.client_ip, w.has_work, s.is_staff
    FROM audit.access_event e
    CROSS JOIN LATERAL (
        SELECT (e.tier <> 'normal'
                OR audit.resource_tier(e.resource_type, e.resource_id, NULL) <> 'normal'
                OR (e.subject_party IS NOT NULL AND casework.effective_tier(e.subject_party) <> 'normal')) AS is_protected) t
    CROSS JOIN LATERAL (
        SELECT CASE WHEN e.subject_party IS NULL THEN NULL ELSE (
                   EXISTS (SELECT 1 FROM alert.alerts a
                            WHERE a.party_id = e.subject_party AND a.state = 'claimed' AND a.claimed_by = e.actor_id)
                OR EXISTS (SELECT 1 FROM casework.case_acl g
                             JOIN casework.case_parties cp ON cp.case_id = g.case_id
                            WHERE g.principal_id = e.actor_id AND g.revoked_at IS NULL
                              AND (g.expires_at IS NULL OR g.expires_at > e.occurred_at)
                              AND cp.party_id = e.subject_party)) END AS has_work) w
    CROSS JOIN LATERAL (
        SELECT CASE WHEN e.subject_party IS NULL THEN NULL ELSE COALESCE((
                   SELECT p.staff_flag OR COALESCE(p.related_to_staff, FALSE)
                     FROM canon.party p
                    WHERE p.id = e.subject_party AND p.recorded_to IS NULL
                    ORDER BY p.valid_from DESC LIMIT 1), FALSE) END AS is_staff) s
    WHERE (e.occurred_at, e.id) > (p_after_at, p_after_id)
    ORDER BY e.occurred_at, e.id
    LIMIT p_limit;
END $$;
ALTER FUNCTION audit.siem_access_page(TIMESTAMPTZ, UUID, INT) OWNER TO akr_migrate;
REVOKE ALL ON FUNCTION audit.siem_access_page(TIMESTAMPTZ, UUID, INT) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION audit.siem_access_page(TIMESTAMPTZ, UUID, INT) TO akr_app;
```

The function is the second read of the audit schema for `akr_app`; like `audit.siem_page` it is owned by `akr_migrate` and reads as its owner, so no table grant changes. `work_link` and `subject_staff` are read when the line is paged, not when the record was read; the forwarder runs every 10 seconds, so the two differ only inside the forwarder's lag.

**3. Setting.** The settings insert of migration `0082` gains one row, and section 3.15 one line (`siem.export_access_events`, boolean, governed, no floor, used by item 6 below; both written in the same change):

```sql
 ('siem.export_access_events', 'true', 'boolean', NULL, TRUE,
  '{"ar": "تصدير سجل الاطلاع على السجلات إلى نظام السجلات الأمنية", "en": "Export record-access events to the SIEM"}'),
```

It is governed (a change request of phase 04): turning it off removes the data the behaviour rules need.

**4. Names.** The six access lines are named `access.view`, `access.search_hit`, `access.export`, `access.print`, `access.download` and `access.unmask` (the prefix `access.` and the verb of phase 02 section 7.9). They are the constant `ACCESS_LINE_KINDS` of the forwarder module. They are not `EventKind` values and are not added to the phase 02 catalogue. The CI check of section 3.5 counts them as exported kinds: each needs a use case in `deploy/siem/use_cases.yaml`, and a rule may name them.

**5. Line format.**
1. CEF: `CEF:0|AKR|AKR-TMS|<release version>|access.<verb>|Record access|<severity 0-10>|rt=<occurred_at ms> suid=<actor_id> spriv=<actor_role> src=<client_ip> outcome=<decision> cs1=<on_behalf_of> cs1Label=onBehalfOf cs2=<resource_type> cs2Label=resourceType cs3=<resource_id or empty> cs3Label=resourceId cs4=<access id> cs4Label=accessId cs5=<purpose_code or empty> cs5Label=purposeCode cs6=<subject_party or empty> cs6Label=subjectParty cn1=<protected 0 or 1> cn1Label=protected cn2=<work_link 0 or 1> cn2Label=workLink cn3=<subject_staff 0 or 1> cn3Label=subjectStaff`. `cn2` and `cn3` with their labels are left out when the value is null. Severity: 3 for `allow`, 5 for `deny`, 7 for `conflict`.
2. JSON: `{"time", "access_id", "action", "decision", "actor_id", "actor_role", "on_behalf_of", "resource_type", "resource_id", "subject_party", "protected", "purpose_code", "client_ip", "work_link", "subject_staff"}`; `action` is the name of item 4.
3. The transport, the framing and the 8 KiB limit are those of section 3.5.

**6. Forwarder.** After steps 2 to 6 of section 3.5 for the audit stream, and under the same advisory lock, the job runs an access pass:
1. Read `siem.export_access_events`. When it is `false`: set the cursor of consumer `cef_access_forwarder` to `(now(), nil id)` (insert or update, `forwarded_total` unchanged) and end the pass. An access event recorded before an off-run is never sent. The cursor moves only when a run executes, so an event recorded after the last off-run (at most one run interval, 10 seconds, before the setting is `true` again) is sent by the first run that finds the setting `true`.
2. Read the cursor `admin.siem_cursors WHERE consumer = 'cef_access_forwarder'`. A missing row starts at `(now(), nil id)`: the export starts with the first run, not with the oldest access event.
3. `rows = audit.siem_access_page(cursor.at, cursor.id, siem.batch_size)`.
4. Format, send, dead-letter and advance as steps 4 to 6 of section 3.5, with `stream = 'access_event'` on a dead letter (section 7.24), `audit_id` = the access id and `action` = the name of item 4.
5. Back-pressure is step 7 of section 3.5 for this cursor. The lag of the tile `siem_lag` (section 3.10) and of `akr_siem_lag_seconds` is the larger of the two streams' lags; `akr_siem_forwarded_total` gains the label `stream` (`audit_log`, `access_event`).

**7. Rust.** `SiemAccessLine { occurred_at, access_id, action, decision, actor_id, actor_role, on_behalf_of: Option<Uuid>, resource_type, resource_id: Option<Uuid>, subject_party: Option<Uuid>, protected: bool, purpose_code: Option<String>, client_ip: Option<IpAddr>, work_link: Option<bool>, subject_staff: Option<bool> }`. `SiemRepository` gains `access_page(after: (DateTime<Utc>, Uuid), limit) -> Vec<SiemAccessLine>` (calls `audit.siem_access_page`) and `set_cursor(tx, consumer, at, id)` (step 1 and the first insert of step 2). `cursor_lock`, `advance` and `dead_letter` are used as they are, with the consumer `cef_access_forwarder`. `SiemForwarder::forward` (section 7.15) runs both passes.

**8. Pull route.** The route table of section 2 gains one row (added; scope and Cedar action `siem.pull`, the policy `siem.pull.service` of section 3.12, no new action):

| Method | Path | Scope | Description |
|-|-|-|-|
| GET | `/api/v1/siem/access-events` | `siem.pull` | Pull API for a service principal: access lines after a cursor, the fields of section 7.23 item 5 (added; section 7.23) |

`GET /api/v1/siem/access-events?after_at=&after_id=&limit=` follows the four steps of the pull API of section 3.5, with `audit.siem_access_page` and the JSON fields of item 5. It is mounted with the `siem_pull` router. When `siem.export_access_events` is `false` it answers 200 with no events and the caller's cursor. utoipa: `SiemAccessEvent` and `SiemAccessPage { events, next }`.

**9. The rule pack.** `deploy/siem/use_cases.yaml` gains one use case for the six access kinds (`record_access`) and these correlation rules. An entry may now carry `where` (a list of `field = value` tests over the line fields of its kinds; the CI check refuses a field that a named kind does not carry) and `siem_lists` (names of lists that the bank keeps in its SIEM, not in the product). Windows and thresholds are proposed starting values, as in section 7.13.

| Rule id | Kinds | Where | Group, window, threshold | Research 30 rule |
|-|-|-|-|-|
| `ub_mass_lookup` | `access.view`, `access.search_hit` | `resource_type = customer`, `decision = allow`, `work_link = 0` | actor, 1 hour, 30 lines | Mass lookup |
| `ub_off_hours_protected` | the six access kinds | `protected = 1` | actor, 1 event, when the time is outside `working_hours` or `src` is outside `staff_networks` (`siem_lists`) | Off-hours or off-network access to restricted tiers |
| `ub_no_work_reason` | `access.view` | `resource_type = customer`, `decision = allow`, `work_link = 0` | actor, 24 hours, 10 lines; the SIEM samples them for review | Access without a work reason |
| `ub_staff_record` | the six access kinds | `subject_staff = 1` | actor, 1 event | Snooping on staff and relatives |
| `ub_conflict_read` | the six access kinds | `decision = conflict` | actor, 1 event | Snooping on staff and relatives (the conflict check of phase 15) |
| `ub_export_spike` | `access.export`, `access.print`, `access.download` | none | actor, 1 hour, 10 lines | Export spikes |
| `ub_closure_burst` | `case.decided`, `case.closed` (phase 18, exported) | none | actor, 1 hour, 15 events; also any event outside `working_hours` (`siem_lists`) | Alert closure patterns |

The rules run in the bank's SIEM. Their recipients are the second line and security, never the actor's manager, and the product tells the actor nothing (section 7.13 item 1 stands). The "Action" column of research 30 section 9.3 is not built as a product reaction: the rules alert, and the controls of section 7.13 item 2 stand. The access line carries no size, so `ub_export_spike` counts exports and does not weigh them.

**10. Still not built.** Canary access (P2, section 7.12, DR-391). DBA and operator anomalies: research 30 section 9.3 takes them from pgAudit and the bastion, which the bank forwards to its SIEM itself; the pack ships no rule for them. A behaviour engine inside the product stays out of this version.

**11. Tipping-off review** (extends section 3.14). An access line of a protected row carries the time, the access id, the decision, the actor, the role, the on-behalf-of id (a staff id, when the read used a delegated role), the verb, the resource kind, the purpose code, the client address and three flags. The access id is the id of the access event and opens nothing in the SIEM. It carries no resource id, no subject party and no tier, so a SIEM reader cannot tell whose record was read, and cannot tell a restricted record from a suspicion one. `subject_staff = 1` on a protected line says that a protected record about some staff member or relative was read; it names nobody. A line of a normal row carries the pseudonymous ids that the audit lines of section 3.5 already carry for normal resources. Purpose codes are codes of `admin.purpose_codes`, never free text; `access_event.reason` is never exported.

**12. Tests** (added to section 6):
- `audit.siem_access_page` on a fixture with one access event on a normal customer and one on a suspicion-tier customer: the first row has `resource_id` and `subject_party`, the second has both NULL and `protected` TRUE; no column is named `tier`. A normal-tier event whose party is raised to `suspicion` afterwards is returned with both ids NULL on the next call. `EXECUTE` by `akr_readonly` is refused.
- `work_link`: TRUE for an actor who holds a claimed alert on the party, TRUE for an actor with a live `casework.case_acl` grant on a case of the party, FALSE after the grant is revoked, NULL for an event with no subject party.
- `subject_staff`: TRUE for a party with `staff_flag` TRUE and for a party with `related_to_staff` TRUE, FALSE for a party with `related_to_staff` NULL.
- Forwarder: with the setting `true`, an access event written after the first run reaches the collector stub as one CEF line with `cs4` equal to its id; with the setting `false` no access line is sent; the test writes two access events, lets one more run pass with the setting `false`, sets it back to `true`, and the two events are not sent, while an event written after that off-run is sent.
- `siem-pull` calls `GET /api/v1/siem/access-events?limit=500` → 200; no event carries a `tier` key; a user token → 403 `E_SCOPE`.
- CI: `use_cases.yaml` with a rule whose `where` names `work_link` on the kind `case.closed` fails the build; an access kind with no use case fails the build.

### 7.24 Dead letters: resend and resolve (owner decision 5; DR-418)
The owner's decision: an operator can resend a dead letter or mark it resolved, and each act is audited. This block replaces the second item of section 7.16, the `siem_dead_letters` row of section 3.10 and the table DDL of section 1.3. DR-422 locks the correction of the check round of 2026-10-11 to the last CHECK of the table.

**1. Table.** In migration `0082` the table and its indexes read (section 1.3 carries this text since the same change, with the grant line and the classification row below):

```sql
CREATE TABLE IF NOT EXISTS admin.siem_dead_letters (
    id                  UUID PRIMARY KEY,
    stream              TEXT NOT NULL DEFAULT 'audit_log' CHECK (stream IN ('audit_log', 'access_event')),
    audit_id            UUID NOT NULL,                       -- the audit row id, or the access event id (section 7.23)
    occurred_at         TIMESTAMPTZ NOT NULL,
    action              TEXT NOT NULL CHECK (action ~ '^[a-z][a-z_]*(\.[a-z][a-z_]*)+$'),
    reason_code         TEXT NOT NULL CHECK (reason_code IN ('format_failed', 'unknown_kind', 'collector_refused', 'oversize')),
    attempts            INT NOT NULL CHECK (attempts >= 1),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    resend_requested_at TIMESTAMPTZ,
    resend_requested_by UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    resolved_at         TIMESTAMPTZ,
    resolved_by         UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    resolution          TEXT CHECK (resolution IN ('resent', 'marked')),
    resolution_code     TEXT CHECK (resolution_code IN ('delivered_other_way', 'not_needed', 'cannot_be_sent')),
    version             INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    CHECK ((resolved_at IS NULL) = (resolved_by IS NULL)),
    CHECK ((resolved_at IS NULL) = (resolution IS NULL)),
    CHECK ((resend_requested_at IS NULL) = (resend_requested_by IS NULL)),
    CHECK (((resolution = 'marked') IS TRUE) = (resolution_code IS NOT NULL))   -- a code on a marked row only; an open row has neither
);
CREATE INDEX IF NOT EXISTS idx_siem_dead_letters_open ON admin.siem_dead_letters (created_at) WHERE resolved_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_siem_dead_letters_resend ON admin.siem_dead_letters (resend_requested_at)
    WHERE resend_requested_at IS NOT NULL AND resolved_at IS NULL;
```

The grant line reads `GRANT UPDATE (resend_requested_at, resend_requested_by, resolved_at, resolved_by, resolution, resolution_code, reason_code, attempts, version) ON admin.siem_dead_letters TO akr_app;`. The classification row of the table lists the fifteen columns above, all `internal`. The retention row is unchanged (R12, `age`, `created_at`). A mark carries a code and no free text, as everywhere in this phase: `delivered_other_way` (the bank gave the event to its SIEM by hand), `not_needed` (the bank's security accepts the gap) or `cannot_be_sent` (the line can never be formatted or accepted).

**2. Routes.** The route table of section 2 gains three rows (added), in a new router module `siem_admin`:

| Method | Path | Scope | Description |
|-|-|-|-|
| GET | `/api/v1/admin/siem/dead-letters` | `admin.system, security.ops` | Dead letters, newest first; `?state=open\|resolved\|all` (default `open`), cursor, limit (added; section 7.24) |
| POST | `/api/v1/admin/siem/dead-letters/{id}/resend` | `admin.system` | Ask the forwarder to send the line again; 202; IF; IDEM (added; section 7.24) |
| POST | `/api/v1/admin/siem/dead-letters/{id}/resolve` | `admin.system` | Mark a dead letter resolved without sending it, with a resolution code; IF (added; section 7.24) |

With the row of section 7.23 the phase has 26 routes: 12 owned and 14 added. By the template rule (25 routes or more) the size of the phase is XL; it was M with 22 routes.

**3. Cedar.** `schema.cedarschema` gains `action "siem_dead_letter.read" appliesTo { principal: User, resource: System, context: Ctx };` and `action "siem_dead_letter.resend", "siem_dead_letter.resolve" in ["system_admin"] appliesTo { principal: User, resource: System, context: Ctx };`. `51_platform.cedar` gains:

```cedar
// SIEM dead letters (owner decision 5). ADM acts; SEC reads.
@id("siem.dl.adm") @role("ADM")
permit (principal, action in [Akr::Action::"siem_dead_letter.read", Akr::Action::"siem_dead_letter.resend",
        Akr::Action::"siem_dead_letter.resolve"], resource is Akr::System)
when { principal.roles.contains("ADM") };

@id("siem.dl.sec.read") @role("SEC")
permit (principal, action == Akr::Action::"siem_dead_letter.read", resource is Akr::System)
when { principal.roles.contains("SEC") };
```

The routes map to the actions in the order of the table: `siem_dead_letter.read`, `siem_dead_letter.resend`, `siem_dead_letter.resolve`.

**4. Rust.** `DeadLetter` gains `stream: SiemStream` (`AuditLog`, `AccessEvent`), `resend_requested_at`, `resend_requested_by`, `resolution: Option<DeadLetterResolution>` (`Resent`, `Marked`), `resolution_code: Option<DeadLetterResolutionCode>` (`DeliveredOtherWay`, `NotNeeded`, `CannotBeSent`) and `version`. `SiemRepository` gains `dead_letters(state, cursor, limit) -> Page<DeadLetter>` (keyset `(created_at, id)` descending), `find_dead_letter(conn, id)`, `request_resend(tx, id, by, expected_version)`, `resend_queue(conn, limit) -> Vec<DeadLetter>` (rows with `resend_requested_at IS NOT NULL AND resolved_at IS NULL`, oldest request first), `mark_resent(tx, id)`, `mark_resend_failed(tx, id, reason_code)` and `mark_resolved(tx, id, by, code, expected_version)`. `SiemStatus.dead_letters_7d` is renamed `dead_letters_open` (the count of rows with `resolved_at IS NULL`). `SiemDeadLetterService { siem: Arc<dyn SiemRepository>, audit: Arc<dyn AuditSink> }` holds the three methods below and is added to the `akr-api` AppState as `siem_dead_letters`. utoipa: `DeadLetterView { id, stream, audit_id, occurred_at, action, reason_code, attempts, created_at, resend_requested_at, resolved_at, resolved_by, resolution, resolution_code, version }`, `ResolveDeadLetterRequest { resolution_code }`.

**5. `list(ctx, state, cursor, limit)`.** Cedar `siem_dead_letter.read`. Returns the page. The view carries ids, kinds, codes and times only; it never carries the line.

**6. `request_resend(ctx, id, if_match)`.**
1. Cedar `siem_dead_letter.resend`. `If-Match` is required (428 `E_PRECONDITION_REQUIRED`).
2. Load the row; none → 404 `E_NOT_FOUND`. `resolved_at` set → 409 `E_NOT_ALLOWED_IN_STATE` (`already_resolved`). `resend_requested_at` set → 409 `E_NOT_ALLOWED_IN_STATE` (`resend_pending`).
3. In one transaction: `resend_requested_at = now()`, `resend_requested_by = ctx.actor`, `version + 1` where `version` is the expected one (no row → 409 `E_VERSION_CONFLICT`); audit `siem.dead_letter_resend_requested`.
4. Answer 202 with the view. No job is enqueued: the next `siem_forward` run (every 10 seconds) does the work.

**7. Resend pass of the forwarder** (job `siem_forward`, after the advisory lock of step 1 of section 3.5 and before step 2). For each row of `resend_queue(siem.batch_size)`:
1. Read the line again. For `stream = 'audit_log'`: `audit.siem_page(occurred_at - interval '1 microsecond', 'ffffffff-ffff-ffff-ffff-ffffffffffff', 1, ARRAY[action])`. For `stream = 'access_event'`: `audit.siem_access_page` with the same two cursor values and limit 1. `occurred_at` is strictly increasing within a stream (phase 02), so the call returns the row of the dead letter; the forwarder checks that the returned id equals `audit_id`. The stripping rule is applied again, with the tier of today.
2. Format and send the line once.
3. On the collector's acknowledgement, in one transaction: `resolved_at = now()`, `resolved_by = resend_requested_by`, `resolution = 'resent'`, `version + 1`; audit `siem.dead_letter_resent` with the actor `resend_requested_by`.
4. On any failure (the id does not match, the line cannot be formatted, the collector refuses or does not answer), in one transaction: `attempts + 1`, `reason_code` = the code of this failure (`format_failed` when the id does not match), `resend_requested_at` and `resend_requested_by` set to NULL, `version + 1`; audit `siem.dead_letter_resend_failed`. The row stays open and can be asked again.
The pass never moves a cursor. A resent line can reach the collector a second time when the first acknowledgement was lost; the line carries its audit id (`cs4`), so the SIEM can drop the copy.

**8. `resolve(ctx, id, resolution_code, if_match)`.**
1. Cedar `siem_dead_letter.resolve`. `If-Match` is required (428 `E_PRECONDITION_REQUIRED`). `resolution_code` is one of the three codes of item 1 (else 422 `E_VALIDATION`, `resolution_code_invalid`).
2. Load the row; none → 404 `E_NOT_FOUND`. `resolved_at` set → 409 `E_NOT_ALLOWED_IN_STATE` (`already_resolved`). `resend_requested_at` set → 409 `E_NOT_ALLOWED_IN_STATE` (`resend_pending`).
3. In one transaction: `resolved_at = now()`, `resolved_by = ctx.actor`, `resolution = 'marked'`, `resolution_code` = the code, `version + 1` where `version` is the expected one (else 409 `E_VERSION_CONFLICT`); audit `siem.dead_letter_resolved`.
4. Answer 200 with the view.

**9. Audit events** (rows added to section 3.13; resource `siem`; detail keys are ids and codes):

| Event kind | On | Resource | SIEM |
|-|-|-|-|
| `siem.dead_letter_resend_requested` | resend asked (`dead_letter_id`, `stream`) | siem | no |
| `siem.dead_letter_resent` | the collector accepted the line (`dead_letter_id`) | siem | no |
| `siem.dead_letter_resend_failed` | the resend failed (`dead_letter_id`, `reason_code`) | siem | no |
| `siem.dead_letter_resolved` | marked resolved without sending (`dead_letter_id`, `stream`, `reason_code`, `resolution_code`) | siem | yes |

The three kinds of the resend work (the request writes the first, the forwarder the other two) are not exported, for the reason `siem.dead_lettered` is not. `siem.dead_letter_resolved` is exported, with the use case `siem_delivery_gap_closed` in `use_cases.yaml`: a person closed a delivery gap without delivery, and security should see it. Section 3.11 gains the sub-codes `already_resolved` and `resend_pending` under `E_NOT_ALLOWED_IN_STATE`, and `resolution_code_invalid` under `E_VALIDATION`.

**10. Tile and metric.** The `siem_dead_letters` row of section 3.10 reads: source = count of `admin.siem_dead_letters` with `resolved_at IS NULL`; warn at least 1; critical above 10. The tile clears when every dead letter is resent or marked. The metric `akr_siem_dead_letters_7d` of section 4.4 is renamed `akr_siem_dead_letters_open`.

**11. Screen** (extends section 5.4, A3). The tile "SIEM dead letters" opens a table: time of the event, kind, stream, reason, attempts, state, and for a resolved row the resolver, the resolution and its code. Row actions for ADM on an open row: "Resend" and "Mark resolved" (a dialog with the three codes; one must be chosen). A row with a pending resend shows "Resend pending" and no action. SEC sees the table with no action. Hooks: `useGetAdminSiemDeadLetters`, `usePostAdminSiemDeadLettersIdResend`, `usePostAdminSiemDeadLettersIdResolve`.

**12. Tipping-off review** (extends section 3.14). The list shows the kind and the time of an event that was not delivered, the reason code and the audit id. It shows no case id, no resource, no party and no line content, and the audit id opens nothing for ADM: the audit viewer of phase 24 applies its own tier rule. A resent line passes the stripping rule again, so a resource that became protected after the first attempt is sent without its id.

**13. Tests** (added to section 6):
- A fixture line that the collector stub refuses is dead-lettered; the tile is `warn`. `adm@` posts `resend` with the row's version → 202; with the stub accepting, within two forwarder runs the row has `resolution = 'resent'`, the stub holds the line once, the tile is `ok`, and the audit trail holds `siem.dead_letter_resend_requested` and `siem.dead_letter_resent`.
- With the stub still refusing, the resend leaves the row open with `attempts` one higher and `resend_requested_at` NULL, and the audit trail holds `siem.dead_letter_resend_failed`.
- `adm@` posts `resolve` with no code → 422 `E_VALIDATION` (`resolution_code_invalid`); with `not_needed` → 200, `resolution = 'marked'`; a second `resolve` or a `resend` on the row → 409 `E_NOT_ALLOWED_IN_STATE` (`already_resolved`); the next SIEM page holds `siem.dead_letter_resolved`.
- `sec@` reads the list → 200 and posts `resend` → 403 `E_SCOPE`; `a1@` reads the list → 403 `E_SCOPE`.
- A stale `If-Match` → 409 `E_VERSION_CONFLICT`; no `If-Match` → 428.
- `UPDATE admin.siem_dead_letters SET action = 'x.y'` as `akr_app` is refused (no column grant).

## Decisions

Numbered from DR-232, as the lead assigned. Each decision is one paragraph with its rationale. The lead adds them to `docs/plan/research.md`; this file does not edit it.

**DR-232 (where the platform state lives and what it does not reuse).** The eleven new tables are in the `admin` schema (no `casework` table), in three migrations `0080` to `0082`. The break-glass grant is not a row of `casework.case_acl`: that table has a closed reason list, no expiry and refuses sealed cases (phase 18), and a four-hour grant with two approvers and its own log is a different record. The access-review snapshot is an evidence object of phase 02, so the matrix cannot drift; items hold ids. Release approval reuses the phase 04 change request, and support approval is a step-up route, because phase 04 refuses to grant `VS` by design. The reason for `admin` rather than `casework` is that none of these records has a case, so the case access rules never apply to them.

**DR-233 (break-glass reaches a case through the team clause, read-only, with no existence signal).** A grant is a row of `admin.break_glass_grants` with a case id and no foreign key. Its requester reaches the case through the same team clause that phase 18 added for `case_acl`, so the phase 17 and phase 18 rules (analyst network, purpose code, step-up `record_open`, sealed-case refusals) still apply and a grant never bypasses them. The Cedar adapter adds the requester to the case's assignees only under the purpose code `break_glass`, and file `51_platform.cedar` forbids every write, decision, access change, pack creation and download under that purpose. Four hours is a database CHECK, the sweep records the end and the predicate stops the read at `expires_at` even if no sweep runs. A request is answered with the same 202 for any case id, and activation does not look at the case, so no response tells a requester or an approver whether a suspicion case exists. The alternative of a `reason = break_glass` row in `case_acl` was rejected because it needs an expiry column on a table that refuses expiry by design and would reach sealed cases.

**DR-234 (the quarterly review: evidence snapshot, decisions by reviewers, revocations through the change path).** The review snapshots `AccessMatrixBuilder::build(at)` and the grants in force into one canonical JSON evidence object with its SHA-256; the items reference grant ids and never copy scope or dates, so nothing can drift from the evidence. SEC decides on every item, M on the items of business roles, and no one on a grant they hold. A revocation is a change request: SEC proposes it through `cr_access_adm` and ADM approves, which is the four-eyes split phase 04 already applies to SEC-made requests. Phase 04 gains one operation, `revoke` on `role_grant`, so the review uses the existing change path and no direct update of a grant. The phase 03 "revoke at once" route is an ADM act and is not used by the review, because the reviewer is SEC. The overdue warning is a row of the phase 24 register with a fixed source key, so the warning opens once.

**DR-235 (SIEM: the audit log is read by cursor through one function, and the stripping rule is stricter than phase 02's).** The forwarder and the pull API read through `audit.siem_page`, a SECURITY DEFINER function owned by `akr_migrate`, because the application role cannot read audit details (phase 02). The function returns the allow-listed columns of the kinds the catalogue flags, never a `case_id`, and drops `resource_id`, `resource_key` and `details` for every row whose tier is not `normal`; phase 02 required the stripping of suspicion rows only, and this phase extends it to restricted rows because a restricted customer is not a case reference either. No outbox row is written per audit event; the reserved topic `siem.export` is released. Back-pressure holds the cursor and never drops a line; a line the collector refuses after a fixed number of tries is dead-lettered by id, so one bad line cannot block the stream. The collector is reached from `akr-worker` over a new egress network with nftables rules rendered from `deploy/zones.yaml`, the same mechanism that restricts the CBS path, so the forwarder cannot reach any other destination.

**DR-236 (releases: the bank's verifier, four eyes through phase 04, and an operator's record of the apply).** The verification job calls the phase 00 `bundle verify` code path, so the platform and the bank's CLI cannot disagree. A refusal is a field of the release record with the first failing check, never a silent failure and never an applied bundle. Approval is a change request of a new target kind `platform_release` with the route `cr_access` (maker ADM, checker SEC), so the four eyes are the ones phase 04 already enforces and the approval appears in the same queue. The apply is recorded by the operator and is not performed by the platform: there is no call-home, no pull of images and no remote push (SP-66). The record therefore proves what the operator said, and the currency tile warns when the applied release grows old.

**DR-237 (vendor support: a time-boxed `VS` grant that can do nothing but read its own profile).** A `VS` grant exists only as a `support_session` grant: the phase 01 `role_grants` table gains `support_session_id` and the `source` value `support_session`, with a CHECK that makes `VS` possible only from a session with a `valid_to`. Phase 04 keeps refusing `VS` on a change request, so no other path grants it. The Cedar file forbids every action except the `self` group for a `VS` principal, which is a forbid that names what is allowed and therefore also covers actions added later. Sessions are at most eight hours (a floor), need SEC approval with step-up and a second person, and end by revocation of the grant and of the vendor's sessions and tokens, from the sweep or at once. Recording is the bank's PAM recording plus the phase 03 authorisation audit of every request the vendor makes.

**DR-238 (support bundles: built by the bank, confirmed by an operator, recorded as their own disclosure, deleted after thirty days).** The archive is built by `akr-ctl support-bundle create` on the bank's host, from an allow-list of fields with ids pseudonymised under a per-bundle key that stays in OpenBao; the operator reads the manifest and confirms it by its hash; the archive is signed with a bank transit key and the platform records its hashes, size and key version under step-up class `export`. The M-37 register of phase 19 is not used: it holds case-related authority disclosures, one row per request with a case reference, and a support bundle has none. The archive file is removed after `support.bundle_retention_days` (a floor of 30) and the disclosure record stays, which is what SP-52 asks of the register.

**DR-239 (the OIDC upstream is configuration, links are explicit, and the upstream adds no authority).** Generic OIDC is the Kratos method with one provider entry, configured per deployment; no code path parses an upstream token. A federated person is linked to an existing local identity by an administrator's command that the audit trail records, never by e-mail, because an automatic link would let an upstream account take over a local one. Roles are never read from the upstream, and its `amr` claim never satisfies AAL2, step-up or `record_open`, so the local second factor and the step-up rules of phase 01 hold unchanged. Disabling a person upstream does not reach the platform until the link is removed or the account is offboarded, and sessions end within the phase 01 idle timeout or the eight-hour absolute lifetime; this is a stated limit, not a hidden one (Open doubts 4). SAML and SCIM wait for phase 31.

**DR-240 (help centre content ships in the signed bundle and is never fetched).** Articles and "what changed" entries are data of the release bundle, written once by the verification transaction, so the help centre is as trustworthy as the release and works in a closed network. An older release keeps its own entries, so the history of changes is kept. Articles are vendor text with no bank data and class `public`.

**DR-241 (inspector accounts reuse the assessor role; the RFC 3161 option is off by default).** A CBI inspector or an external assessor needs read-only access for a bounded time. The assessor role `AS` of phase 24 already has a time box, a read-only scope and the assessor pack, so an inspector account is an `AS` grant with a `valid_to` and a bank-approved change request; no role is added to a closed list. The RFC 3161 time stamp is an additional, separately stored token per anchor, requested only when the bank turns it on, and the Ed25519 anchor signature stays the primary proof, because a time stamp authority is one more external dependency that a bank may not want on by default.

**DR-242 (break-glass approvers are compliance and security; the requester is A2).** Research file 30 section 7.6 asks for one approver from compliance and one from security; module M3 says "SEC + M". This phase follows the research: the compliance approver is the compliance head `CH` and the security approver is `SEC`, and the MLRO is notified rather than asked to approve, which keeps the approving pair independent of the case team. The requester is `A2` only: phase 17 rule 1 admits only A1, A2 and M to a case row, and M needs no grant, so a grant to SEC or CH would open nothing (lead review). A person who holds A2 and an approving role is refused as approver of their own request. The change to M3's wording is recorded in Open doubts 2.

**DR-243 (two notice codes are added to the closed phase 22 list, with counts only).** The break-glass notices need codes that say "an emergency access request is waiting" and "an emergency access grant is active", with a count and nothing else, and the closed list of phase 22 is the control on what a notice may say. The codes are added as two `NoticeCode` variants and two seed rows in `0082`, with the recipient rule of section 3.3. The list grows from fifteen to seventeen, and the neutral-wording rule of DESIGN-SYSTEM section 11 applies to both texts.

**DR-244 (new tiles and two control-report counts are reads of the sources they name).** The cockpit tiles of section 3.10 and the two counts added to the phase 24 control report are queries over the new tables and the audit trail; none stores a copy. The control report keeps its small-number suppression, so a count of one is not shown. Lag and dead letters are the operator's signals; break-glass counts are IA's signal, given as counts only, as DR-219 of phase 24 decided for suspicion records.

**DR-245 (the SIEM pull is a service principal's scope, mounted only for clients granted it).** The pull API is the first consumer of phase 01's `ClientCredentials`. Its scope is `siem.pull`, which exists in no list yet and is added to API.md section 2 by the lead (Open doubts 5). The route is not mounted for a client without the scope, so an unauthorised client gets a 404 from the router, which keeps the endpoint invisible; a user token is refused with `E_SCOPE`. Service principals hold no role and the Cedar permit reads `principal.kind == "service"`, so no role is invented for them.

**Lead corrections (review 2026-10-09).** A break-glass request is unique per requester and case, not per case: a per-case index dropped a second requester's request while auditing it, told that requester another request was open, and let one requester hold a case's emergency path shut. The read-only rule is an allow-list of fifteen read actions under purpose `break_glass`, because team membership also feeds later team checks (phase 19 `Rfi.assignees`) that a list of forbidden writes would miss. `audit.siem_page` returns no tier value (the label `suspicion` must not leave for the SIEM), is owned by `akr_migrate` and pins `search_path = pg_catalog`. The requester is A2 only: phase 17 rule 1 admits only A1, A2 and M to a case row, so a grant to SEC or CH opened nothing and M needs none. The visibility clause requires the purpose `break_glass`, as the Cedar adapter does, so the list predicate and Cedar stay equal. The support-session approval is step-up class `access_change` (it creates a role grant). A `VS` case read is refused by the scope gate (`E_SCOPE`) for every id.

**DR-414 (owner decision of 2026-10-10: reads are exported to the SIEM, and the behaviour rule pack ships).** The lead had kept the rules that need allowed reads as P2 (section 7.13). The owner overturned it. Every access event is exported as an access line through `audit.siem_access_page`, with its own cursor, the pull route `GET /api/v1/siem/access-events` and the governed switch `siem.export_access_events`. A protected line carries no resource id, no subject party and no tier. The pack gains seven rules. Canary access (DR-391) and the DBA and operator rules stay out (section 7.23).

**DR-418 (owner decision of 2026-10-10: dead letters are resent or resolved).** The lead had kept resolving a dead letter as P2 (section 7.16). The owner overturned it. ADM asks for a resend, which the forwarder does on its next run through the same stripping rule, or marks the dead letter resolved with one of three codes. Each act is audited, and the mark is exported to the SIEM. The tile counts the open dead letters (section 7.24).

## Open doubts (for gap pass 1 and the bank)

1. **Break-glass for a party.** Scope is a case only. A party grant needs the party entity of phase 15 and its visibility clause of phase 18 section 3.10 (rule 5) to take a grant as team membership, which is a phase 15 change. The request route refuses a party scope (`party_scope_not_shipped`) until the bank asks for it.
2. **Approvers.** Module M3 says "two approvers (SEC + M)"; research 30 section 7.6 says compliance and security. This phase uses CH and SEC (DR-242). The bank confirms the pair; the change is one role list in the request handler and the Cedar permits.
3. **Reason text.** A break-glass reason is a closed code. Research 30 section 7.6 says "with a reason"; a free-text reason would pass the phase 09 scanner but could carry case facts into a log that IA reads as counts. The bank says whether it wants text.
4. **Upstream disable.** Without SCIM (phase 31) the platform does not see an upstream disable. The runbook removes the link; a federated session lasts at most eight hours. The bank may want a shorter absolute lifetime for federated sessions (a governed setting of phase 01) or an upstream session check on each request, which needs the upstream's availability on every request and is not built.
5. **The scope `siem.pull`.** New in this phase. The lead adds it to API.md section 2 with its holders (service principals only). The scope `support.readonly` of API.md section 2 is held by no route of this phase: `VS` reads only its own profile (`self`). The lead decides whether the scope is removed or kept for a later support surface.
6. **What the approvers and the requester learn.** Approvers see the request id, the requester and the reason code, never the case id. The requester learns that a case exists only after two approvals, by reading it. The bank accepts this as the designed emergency path; the alternative is an approver view that shows the case id to CH and SEC, which would give the approvers an existence signal they do not otherwise have.
7. **Requests for unknown ids.** A request for an id that does not exist is accepted and recorded (DR-233). The records are bounded by the rate limit of phase 00 per user and by the one-open-request-per-requester-and-case index, but the bank may want unknown ids refused at the request, which would then reveal existence to the requester.
8. **Review roles.** SEC decides on all items and M on business-role items. The compliance head (CH) is not a reviewer. A second line may want CH to confirm SEC and M grants; that is one more Cedar permit and one more reviewer of the items.
9. **Disclosure register and key.** The general disclosure register of SP-52 is not built; support bundles have their own record (DR-238). The transit key `akr-sign-bundle` is new and must enter the key inventory of phases 00 and 01 (SP-11). The transfer of an archive to the vendor is not recorded by the platform; the bank records it in its own register until the general register exists.
10. **Bundle confirmation.** One operator confirms the manifest. The bank may require two people, which would need the record to take a second confirmer.
11. **Restricted-tier stripping.** The SIEM rule removes details and resource ids for restricted rows too, which is stricter than phase 02's suspicion-only rule (DR-235). Phase 02's owner confirms that the stricter rule is wanted.
12. **Staff ids in the SIEM.** `actor_id` is the bank's staff id. The bank may pseudonymise it before it reaches its SIEM with the phase 02 pseudonym key.
13. **Release ordering.** `version_not_newer` refuses an older version. A security patch on an older line is accepted only when its notes say so (section 3.6 step 1(f)); the rule for a rollback to an older release is the bank's rollback plan, not an import.
14. **The apply is a record.** The platform cannot prove that the operator applied the release; the record is the operator's statement under step-up. Phase 30's cluster tooling may give a machine record of the rollout.
15. **Inspector period.** `AS` grants are capped at `assessor.grant_max_days` (30 days). An inspection longer than that needs a new grant per period; the bank may want a longer cap for a named inspection.
16. **Posture page.** The security posture page of SP-81 is not built. The cockpit tiles and the access review are its first inputs; the page itself is a later phase.
17. **Hosted installations.** The hosted cluster per bank (TS-34, SP-89) is not built; the single-tenant default is the only installation model of this phase.
18. **Archive deletion is not exported.** `support_bundle.archive_deleted` is not in the SIEM stream. The bank may want it there.
19. **The `at` parameter of the matrix.** `AccessMatrixBuilder::build(at)` is phase 03's stub. The grant list of the snapshot is read directly from `admin.role_grants` at `snapshot_at`; if phase 03 later gives the matrix a historical view, the two must agree and the test of section 6 pins them.
20. **The RFC 3161 authority.** The bank names the time-stamp authority and its certificate in the deployment configuration. Until it does, the option stays off.
21. **Break-glass use log.** The dedicated log writes a `used` row on the first read and then once per 15 minutes. A bank that wants every read in the dedicated log, not only in the audit trail and the access events, needs the row written on every read, which is a volume decision.
22. **Kratos upgrade.** The upgrade from Kratos v1.2.0 to the version the OIDC method needs is part of the platform upgrade plan (TS-25), not of this phase; the configuration of section 3.4 is written for the version in the pinned image of phase 00.

## What This Phase Does NOT Touch

- No SAML and no SCIM (phase 31, Ory Polis); no K3s, Helm or automated failover (phase 30); no hosted multi-tenant model (phase 31 or later, Open doubts 17).
- No SIEM push of case, party or record content, no SIEM use-case rules that read suspicion content, and no change to the audit trail's chain, anchors or signing keys (phase 02); the RFC 3161 token is an extra row beside the anchor.
- No new role code, no new step-up class, no new problem code, no new outbox topic and no timer kind. `siem.export` is released, not used.
- No change to phase 02's access log, its chain or its grants, and no change to the SIEM flag of `authz.allowed` (phase 03): the access lines of section 7.23 are read through `audit.siem_access_page`. No behaviour engine inside the product, no canary records and no rule over database or bastion sessions (section 7.23 item 10).
- No change to the case access list `casework.case_acl`, its reasons or its expiry (phase 18); no change to `services::visibility::build` beyond the one clause of section 3.2, and no change to the suspicion forbids of `90_alerts.cedar` or `91_cases.cedar` (phases 17 and 18).
- No change to the Cedar files other than `51_platform.cedar`; `schema.cedarschema` receives the action declarations of section 3.12 and of section 7.24 item 3 only (the appended-declarations pattern of phases 17, 18 and 24); `50_system.cedar` (phase 03), `60_audit.cedar` and `C0_governance.cedar` (phase 24), `61_retention.cedar` (phase 23), `80_detection.cedar` (phase 06) and `90_alerts.cedar` (phase 17) are unchanged.
- No change to phase 03's access matrix, its routes, `DELETE /admin/users/{id}/grants/{grant_id}`, its purpose codes or its `sod.json`; no change to phase 04's workflow engine or to the `cr_access` and `cr_access_adm` routes beyond the two target entries of section 4.5.
- No change to phase 19's M-37 register or to phase 19's internal reports and RFIs; the support bundles do not join that register.
- No change to phase 21's reports, phase 22's notification templates other than the two codes of section 4.5, or phase 23's retention engine and holds (this phase adds its own retention rows only).
- No screen beyond A3, A4, X5 and the G5 sections of section 5; no change to G1 except its tiles; no PDF or export of the release notes or support bundle in the UI.
- No change to phase 00's compose stack beyond the two networks of section 4.4; `log-forwarder` (Fluent Bit) keeps its JSON stdout path.
- No call-home, no auto-update, no remote push and no pull of images or bundles by the platform (SP-66, TS-30).
- No new evidence kind; the snapshot and the verification report use phase 02 kinds `snapshot` and `package`.
- No CBS connection and no CBS read; the platform reads no core banking table in this phase.
