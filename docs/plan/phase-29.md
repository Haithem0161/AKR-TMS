# Phase 29: Assessment API

**Goal:** A bank that switches the phase 2 option on gets a synchronous pre-release assessment: its payment gateway calls `akr-assess` over mTLS before it releases a payment, receives a signed, expiring pass, hold or reject with reason codes and no score, and sends holds to the screening queue; each rail moves through shadow, advisory and enforcing by change request, fail policies are bank data enforced by the gateway, and when AKR-TMS cannot answer the gateway's own fail policy applies and the outage and every release without a signed decision are recorded.

**Dependencies:** Phase 00 (roles `akr_assess` and schema `assess`, `admin.jobs`, `admin.settings` with cron values, `admin.feature_flags`, `admin.idempotency_keys` (not reused: the replay ledger of section 3.3), the OpenBao transit key `akr-kek` and KV `db/akr_assess`, zones `z4_gateway`, `z4_db`, `z4_cache`, compose profile `phase2`), Phase 01 (`admin.users` service principals with `owner_user_id` and required `expires_at`, `StepUpService` class `decision`, `admin.role_grants`), Phase 02 (audit sink and the `audit.audit_log` catalogue with the `siem` flag, `PartitionRegistry` and `create_partition` pattern, `admin.evidence_objects` and `EvidenceStore::put_generated` for the list extract, `akr_audit_writer` membership of `akr_assess`), Phase 03 (Cedar bundle and range D0, `Ctx` with `purpose` and `network`, `purpose_codes` `internal_audit`, `supervisory_inspection`, `other`), Phase 04 (`admin.change_requests`, `ChangeTargetRegistry`, `cr_material`, `activate_change`, `admin.inventory_items` kinds `parameter_set` and `rule`), Phase 09 (authority-matrix decision type `screening.decision`, reason-code family `decision`), Phase 10 (`canon.payment_message` read by the release-without-decision matcher), Phase 14 (`ScreeningIndexRegistry`, `ScreeningService`, `AlertService`, `Scorer`, `FreezeTrigger`, `alert.screening_runs`, `alert.screening_alerts`, `ref.lists`, `ref.list_versions`, `ref.list_entries`), Phase 06 (scenario column `phase2_candidate`, read only as described in section 3.9), Phase 17 (`services::visibility::build`, `casework.effective_tier`), Phase 20 (`FreezeService` as the `FreezeTrigger` implementation), Phase 23 (retention classes, `admin.retention_drop_partition`, `akr_retention`), Phase 24 (`admin.registers` and its `source` list, `review_calendar`, G1 tiles, `dashboard.read`), Phase 26 (optional-pack pattern: flag off means 404 `E_NOT_FOUND`)

**Macro-phase:** P2 (optional per bank; no rail is assessed until the bank sets its mode)
**Complexity:** L (18 routes: 4 gateway routes on `akr-assess` (API.md section 4.1) and 14 routes on `akr-api` (the 12 rows of API.md section 3.19 and 2 added rows); 19 new tables in schema `assess` in 4 migrations `0091` to `0094`; 1 widened CHECK of phase 24 (`admin.registers.source`); 6 new job kinds; 0 outbox topics; 0 timer kinds; 1 Cedar file `D0_assess.cedar`; 1 compose profile `phase2` of `akr-assess` (existing), 25 DR numbers, DR-292 to DR-316)
**Binaries touched:** akr-api (router `assess` with the 14 people routes; `HoldService`, `FailPolicyService`, `RailModeService`, `ProhibitedRuleService`, `AssessReadService`, `ControlTestService` in `AppState`; the three ChangeTargets registered in `ChangeTargetRegistry`) | akr-worker (jobs `assess_release_match`, `assess_hold_sweep`, `assess_blind_watch`, `assess_rollup_watch`, `assess_fallback_extract`, `assess_ledger_sweep`; `ReleaseControlService`, `HoldSweepService`, `OutageService`, `RollupService`, `FallbackExtractService`, `LedgerSweepService`) | akr-ingestd unchanged (no CBS read is added) | akr-assess (the 4 gateway routes and the health listener; `GatewayAuthenticator`, `AssessmentService`, `MessageParser`, `PolicyCache`, `DecisionSigner`, `GatewayRateLimiter`, `NodeHeartbeat`; the decision path reads the screening index of phase 14 in process) | akr-ctl (`assess gateway register|revoke`, `assess keys generate|rotate|export`, `assess rail add`, `assess policy export`, `assess verify-decision`)

**Requirements carried:**
- Implemented here: C-55 (synchronous assess endpoint over mTLS, identity bound by a pinned gateway certificate and a Cedar principal, idempotent byte-exact replay, signed decisions with expiry, generic customer codes; sections 3.1 to 3.3), C-56 (latency budget per step and the shadow, advisory and enforcing stages with maker-checker change requests; sections 3.1, 3.7, 3.11), C-57 (fail policies per rail, corridor, currency and amount as bank data changed by change request, degraded flags in every response, stale lists turn cross-border decisions into hold; sections 3.6, 3.11), C-58 (two stateless nodes, the signed list extract as the local fallback, blind periods recorded, weekly control-test payment; sections 3.13, 3.14), C-59 (release-without-decision control; section 3.14), C-60 (sanctions reject starts the freeze through the phase 14 port; section 3.8), C-65 (a score never rejects, deterministic prohibited rules with `rule_id`; sections 3.1 and 3.9), T-43 (decision vocabulary, timeouts, fail policy, decision log, replay tool `akr-ctl assess verify-decision`, shadow before holds, no ML in the path; sections 3.1, 3.4, 3.6, 3.7), N-49 and N-50 and N-51 (the inline screen reuses the phase 14 immutable index with its budget, `partial` flag and recorded policy; section 3.1 step 9), SP-93 (reasons and no score in the response; per-client rate limits in Valkey; repeat-assessment counter; sections 3.1, 3.10 and 3.12), GV-53 (Tier 1 inventory entries, latency and availability targets, decision log for every hold, no model in the path; section 3.22)
- Connected to earlier phases (reused, not repeated): C-60 freeze itself (phase 20 `FreezeService` through phase 14 `FreezeTrigger`), S-26 (phase 06 and 16 scenario list is not read for rejects; the prohibited-rule grammar of section 3.9 is the deterministic part), GV-32 (validator is never the owner, the inventory CHECK of phase 04), GV-03 (tiering answers stored in `inventory_items.tier_basis`)
- Not carried: C-61, C-62 and C-64 (source drivers, adapter packs and CBS time regression, outside this phase), C-63 (the sanctions duties of phases 14 and 20, carried there), SP-94 and GV-45 (phase 28), GV-49 and GV-50 (phase 28), T-13 to T-15 (phase 27: no model is used inline in this phase), the case creation that research 25 15.3 shows in its response (`case_ref`, Open doubts 3), the webhook release of research 25 15.7 (Open doubts 9), the RETURNED state of research 25 15.7 (a gateway state, not a route; Open doubts 10), the local matcher container of research 25 15.6 (replaced by the signed list extract, DR-305), the multi-corridor customer pre-confirmation channel of research 25 15.1 (later)
- Gap pass 1 (coverage), implemented here: SP-96 (no customer-facing surface; the customer sees a generic code only).
- Gap pass 1 (coverage), not carried: K-40 (payment-time rating lookup, outside this release, DR-396); F-12 (payment-transparency checks before payments leave; P2, DR-401); VO-04 (import of payment-screening engine alerts; P2, DR-402); VD-23 (remittance pre-check; P2, DR-403); VC-17, VC-18, VC-20 and VC-34 (payment-time screening, fail policies, name alignment and the Swift screening feed; P2, DR-404).

---

## 1. SQLx Migration Changes

Four migrations, numbered after phase 28's `0090` (phases 26 to 28 used `0083` to `0090`; grep of docs/plan confirms `0091` to `0094` are free). Nineteen tables in schema `assess`, all in the schema that phase 00 created for `akr_assess`. The decision ledger is the only partitioned table. No table of an earlier phase is redefined: the widened phase 24 CHECK is in `0094`, and no other earlier table changes. The DATA-MODEL sketch of section 19 is fixed as follows: `decisions` loses `model_version_id` (no model runs inline, DR-302), gains the columns the logic needs (section 1.1), and uses a `match_key` for the release control; `holds` is split into `holds` and `hold_candidates`; `fail_policies` becomes a versioned table with its own change-request link; `rail_modes`, `prohibited_rules`, `gateways`, `signing_keys`, `idempotency`, `node_status`, `blind_periods`, `release_exceptions`, `control_tests`, `control_state`, `mode_rollups`, `amount_bands` and `policy_state` are added.

**Why `assess` and not `admin` for the registry:** the gateway identity, keys and decision ledger are read and written only by `akr_assess` and `akr_app`; `admin` holds configuration for every module, and phase 00 grants `akr_assess` `USAGE` on `assess` and on nothing else that matters for writes.

### 1.1 `0091_assess_registry.sql`

```sql
-- migration 0091_assess_registry.sql
-- Phase 29. Rails, gateway identities (certificate pins), signing keys, the decision ledger (partitioned by received_at
-- month, R7), the replay ledger, node heartbeats and outage periods (schema assess). Idempotent.

CREATE TABLE IF NOT EXISTS assess.rails (
    code             TEXT PRIMARY KEY CHECK (code ~ '^[a-z][a-z0-9_]{1,15}$'),
    label            JSONB NOT NULL CHECK (label ? 'ar' AND label ? 'en'),
    payload_formats  TEXT[] NOT NULL DEFAULT '{}' CHECK (payload_formats <@ ARRAY['mt103', 'pacs008', 'canonical']::text[]),
    active           BOOLEAN NOT NULL DEFAULT TRUE,     -- a rail without a mode row is never assessed
    created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    version          INT NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS assess.gateways (
    id               UUID PRIMARY KEY,                                  -- UUIDv7, app-side
    gateway_key      TEXT NOT NULL CHECK (gateway_key ~ '^[a-z][a-z0-9_-]{2,40}$'),
    user_id          UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,   -- kind service, expires_at = not_after
    cert_sha256      BYTEA NOT NULL UNIQUE CHECK (octet_length(cert_sha256) = 32),  -- SHA-256 of the leaf certificate (DER)
    cert_subject     TEXT NOT NULL CHECK (length(cert_subject) BETWEEN 1 AND 300),
    not_before       TIMESTAMPTZ NOT NULL,
    not_after        TIMESTAMPTZ NOT NULL,
    status           TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'revoked')),
    revoke_reason    TEXT CHECK (revoke_reason IS NULL OR revoke_reason IN ('compromise', 'rotation', 'decommissioned')),
    registered_by    UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    registered_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    revoked_at       TIMESTAMPTZ,
    revoked_by       UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    version          INT NOT NULL DEFAULT 1,
    CHECK (not_after > not_before),
    CHECK ((status = 'revoked') = (revoked_at IS NOT NULL)),
    CHECK ((status = 'revoked') = (revoke_reason IS NOT NULL)),
    CHECK ((status = 'revoked') = (revoked_by IS NOT NULL))
);
-- One active certificate per gateway; a rotation is a new row with the same gateway_key and the old row revoked by `rotation`.
CREATE UNIQUE INDEX IF NOT EXISTS uq_assess_gateway_active_key ON assess.gateways (gateway_key) WHERE status = 'active';
CREATE INDEX IF NOT EXISTS idx_assess_gateways_user ON assess.gateways (user_id);

CREATE TABLE IF NOT EXISTS assess.signing_keys (
    purpose          TEXT NOT NULL CHECK (purpose IN ('decision', 'extract')),
    version          INT NOT NULL CHECK (version >= 1),
    status           TEXT NOT NULL CHECK (status IN ('active', 'verify_only')),
    public_key       BYTEA NOT NULL CHECK (octet_length(public_key) = 32),   -- Ed25519 verify key
    fingerprint      TEXT NOT NULL CHECK (fingerprint ~ '^[0-9a-f]{64}$'),   -- SHA-256 of public_key, read out on a second channel
    wrapped_seed     TEXT CHECK (wrapped_seed IS NULL OR length(wrapped_seed) BETWEEN 40 AND 2048),   -- OpenBao transit ciphertext of the seed (akr-kek); NULL once verify_only
    created_by       UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    retired_at       TIMESTAMPTZ,
    PRIMARY KEY (purpose, version),
    CHECK ((status = 'active') = (wrapped_seed IS NOT NULL)),
    CHECK ((status = 'verify_only') = (retired_at IS NOT NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_assess_signing_active ON assess.signing_keys (purpose) WHERE status = 'active';

-- The decision ledger. One row per assessment (kind assessment) and per release of a hold (kind hold_release).
-- Append-only by trigger (0093); rows leave only with their partition (phase 23, DR-187).
CREATE TABLE IF NOT EXISTS assess.decisions (
    id                  UUID NOT NULL,                       -- UUIDv7, app-side
    received_at         TIMESTAMPTZ NOT NULL,                -- first receipt of the request; partition key
    kind                TEXT NOT NULL CHECK (kind IN ('assessment', 'hold_release')),
    gateway_id          UUID NOT NULL REFERENCES assess.gateways(id) ON DELETE RESTRICT,
    rail                TEXT NOT NULL REFERENCES assess.rails(code) ON DELETE RESTRICT,
    direction           TEXT NOT NULL CHECK (direction IN ('in', 'out')),
    payload_format      TEXT NOT NULL CHECK (payload_format IN ('mt103', 'pacs008', 'canonical')),
    corridor            TEXT CHECK (corridor IS NULL OR corridor ~ '^[A-Z]{2}$'),       -- counterparty country
    currency            TEXT CHECK (currency IS NULL OR currency ~ '^[A-Z]{3}$'),
    amount_band         TEXT CHECK (amount_band IS NULL OR amount_band ~ '^[A-Z][A-Z0-9_]{1,15}$'),
    match_key           TEXT CHECK (match_key IS NULL OR match_key ~ '^(uetr:[0-9a-f-]{36}|ref:[0-9a-f]{64})$'),
    request_sha256      BYTEA NOT NULL CHECK (octet_length(request_sha256) = 32),
    decision            TEXT NOT NULL CHECK (decision IN ('pass', 'hold', 'reject')),   -- the answer the gateway receives
    mode                TEXT NOT NULL CHECK (mode IN ('shadow', 'advisory', 'enforcing')),
    would_be            TEXT CHECK (would_be IS NULL OR would_be IN ('pass', 'hold', 'reject')),   -- the computed decision, shadow only
    fail_policy         TEXT CHECK (fail_policy IS NULL OR fail_policy IN ('open', 'closed')),
    policy_version      INT NOT NULL CHECK (policy_version >= 0),
    reason_codes        TEXT[] NOT NULL DEFAULT '{}',
    rule_id             TEXT CHECK (rule_id IS NULL OR rule_id ~ '^[A-Z][A-Z0-9_]{5,80}$'),
    degraded            TEXT[] NOT NULL DEFAULT '{}',
    list_version_ids    UUID[] NOT NULL DEFAULT '{}',
    matched_party_ids   UUID[] NOT NULL DEFAULT '{}',    -- canonical party ids of a list hit, when the payment maps to one (personal)
    hold_id             UUID,
    key_version         INT NOT NULL CHECK (key_version >= 1),
    response_bytes      BYTEA NOT NULL,                  -- the exact signed JSON returned (replays return these bytes)
    signature           BYTEA NOT NULL CHECK (octet_length(signature) = 64),
    expires_at          TIMESTAMPTZ NOT NULL,
    latency_ms          INT NOT NULL CHECK (latency_ms >= 0),
    PRIMARY KEY (id, received_at),
    CHECK ((mode = 'shadow') = (would_be IS NOT NULL)),
    CHECK (mode <> 'shadow' OR decision = 'pass'),
    CHECK (decision <> 'reject' OR rule_id IS NOT NULL OR mode = 'shadow'),   -- a reject without a person names its rule (C-65)
    CHECK ((kind = 'hold_release') = (hold_id IS NOT NULL AND decision = 'pass')),
    CHECK (expires_at > received_at)
) PARTITION BY RANGE (received_at);

CREATE INDEX IF NOT EXISTS idx_assess_decisions_gateway ON assess.decisions (gateway_id, received_at);
CREATE INDEX IF NOT EXISTS idx_assess_decisions_match   ON assess.decisions (match_key, received_at) WHERE match_key IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_assess_decisions_hold    ON assess.decisions (hold_id) WHERE hold_id IS NOT NULL;

-- Monthly partitions through the factory (phase 13 pattern). akr_assess calls it once when a month is missing and retries.
CREATE OR REPLACE FUNCTION assess.create_partition(p_year INT, p_month INT) RETURNS TEXT
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog AS $$
DECLARE
    v_name TEXT := format('decisions_%s_%s', p_year, lpad(p_month::text, 2, '0'));
    v_from TIMESTAMPTZ;
    v_to   TIMESTAMPTZ;
BEGIN
    IF p_month < 1 OR p_month > 12 OR p_year < 2020 OR p_year > 2100 THEN
        RAISE EXCEPTION 'assess.create_partition: year % month % out of range', p_year, p_month;
    END IF;
    v_from := make_timestamptz(p_year, p_month, 1, 0, 0, 0, 'UTC');
    v_to   := v_from + interval '1 month';
    EXECUTE format('CREATE TABLE IF NOT EXISTS assess.%I PARTITION OF assess.decisions FOR VALUES FROM (%L) TO (%L)',
                   v_name, v_from, v_to);
    RETURN v_name;
END $$;
ALTER FUNCTION assess.create_partition(INT, INT) OWNER TO akr_migrate;
REVOKE ALL ON FUNCTION assess.create_partition(INT, INT) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION assess.create_partition(INT, INT) TO akr_assess, akr_app;

-- Replay ledger (not partitioned, section 3.3). Holds the key of a request and the decision it produced; the response
-- bytes live on the decision row. Deleted by assess_ledger_sweep after assess.idempotency_hours. No foreign key to the
-- decision: the row exists before the decision is written (in_flight).
CREATE TABLE IF NOT EXISTS assess.idempotency (
    gateway_id            UUID NOT NULL REFERENCES assess.gateways(id) ON DELETE RESTRICT,
    idem_key              TEXT NOT NULL CHECK (idem_key ~ '^[ -~]{8,255}$'),
    request_sha256        BYTEA NOT NULL CHECK (octet_length(request_sha256) = 32),
    decision_id           UUID NOT NULL,
    decision_received_at  TIMESTAMPTZ NOT NULL,
    state                 TEXT NOT NULL CHECK (state IN ('in_flight', 'done')),
    lease_until           TIMESTAMPTZ NOT NULL,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at          TIMESTAMPTZ,
    PRIMARY KEY (gateway_id, idem_key),
    CHECK ((state = 'done') = (completed_at IS NOT NULL))
);
CREATE INDEX IF NOT EXISTS idx_assess_idem_created ON assess.idempotency (created_at);

-- One row per node, written by akr-assess every 30 seconds (section 3.13).
CREATE TABLE IF NOT EXISTS assess.node_status (
    node_id         TEXT PRIMARY KEY CHECK (node_id ~ '^[a-z0-9][a-z0-9_-]{1,40}$'),
    started_at      TIMESTAMPTZ NOT NULL,
    last_seen_at    TIMESTAMPTZ NOT NULL,
    build_version   TEXT NOT NULL CHECK (length(build_version) BETWEEN 1 AND 60),
    policy_version  INT NOT NULL CHECK (policy_version >= 0),
    mode_summary    JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(mode_summary) = 'object'),   -- rail to mode, no counts
    version         INT NOT NULL DEFAULT 1
);

-- Outage periods of the assessment service (section 3.13). Distinct from phase 11 admin.blind_periods, which records
-- blindness of a CBS source by source_id.
CREATE TABLE IF NOT EXISTS assess.blind_periods (
    id              UUID PRIMARY KEY,
    started_at      TIMESTAMPTZ NOT NULL,
    ended_at        TIMESTAMPTZ,
    reason_code     TEXT NOT NULL CHECK (reason_code IN ('no_heartbeat', 'operator_recorded')),
    last_seen_at    TIMESTAMPTZ,
    recorded_by     TEXT NOT NULL CHECK (recorded_by IN ('system', 'operator')),
    version         INT NOT NULL DEFAULT 1,
    CHECK (ended_at IS NULL OR ended_at >= started_at)
);
-- At most one open period at a time (the expression is constant for open rows).
CREATE UNIQUE INDEX IF NOT EXISTS uq_assess_blind_open ON assess.blind_periods ((ended_at IS NULL)) WHERE ended_at IS NULL;
```

The `make_timestamptz` call and `format` are in `pg_catalog`, so `SET search_path = pg_catalog` is sufficient. The `assess.decisions` partitions are created by `assess.create_partition`, which phase 02 `PartitionRegistry` calls as `ahead_months` (section 4.4).

### 1.2 `0092_assess_policy_holds.sql`

```sql
-- migration 0092_assess_policy_holds.sql
-- Phase 29. Amount bands, fail policies, rail modes, the policy set counter, prohibited rules, holds and their candidates,
-- the release-control state and exceptions, control tests and the mode roll-ups (schema assess). Idempotent.
-- Every row of the policy tables names the change request that wrote it (guards in 0093).

CREATE TABLE IF NOT EXISTS assess.policy_state (
    id              SMALLINT PRIMARY KEY CHECK (id = 1),          -- singleton
    set_version     INT NOT NULL DEFAULT 0 CHECK (set_version >= 0),   -- bumped by every activation of assess_policy, assess_mode or assess_rule
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    version         INT NOT NULL DEFAULT 1
);
INSERT INTO assess.policy_state (id, set_version) VALUES (1, 0) ON CONFLICT (id) DO NOTHING;

CREATE TABLE IF NOT EXISTS assess.amount_bands (
    currency          TEXT NOT NULL CHECK (currency ~ '^[A-Z]{3}$'),
    band_code         TEXT NOT NULL CHECK (band_code ~ '^[A-Z][A-Z0-9_]{1,15}$'),
    lower_bound       NUMERIC(28,6) NOT NULL CHECK (lower_bound >= 0),
    upper_bound       NUMERIC(28,6) CHECK (upper_bound IS NULL OR upper_bound > lower_bound),   -- NULL = no upper bound
    set_version       INT NOT NULL CHECK (set_version >= 1),
    change_request_id UUID NOT NULL REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    PRIMARY KEY (currency, band_code, set_version)   -- band sets are kept: one row per band and set (section 7+)
);

-- Fail policy per rail, corridor, currency and amount band. '*' is the wildcard. The gateway enforces on_failure; the
-- decision path reads the most specific current row to record fail_policy and policy_version on the decision (section 3.1).
CREATE TABLE IF NOT EXISTS assess.fail_policies (
    id                UUID PRIMARY KEY,
    rail              TEXT NOT NULL REFERENCES assess.rails(code) ON DELETE RESTRICT,
    corridor          TEXT NOT NULL CHECK (corridor = '*' OR corridor ~ '^[A-Z]{2}$'),
    currency          TEXT NOT NULL CHECK (currency = '*' OR currency ~ '^[A-Z]{3}$'),
    amount_band       TEXT NOT NULL CHECK (amount_band = '*' OR amount_band ~ '^[A-Z][A-Z0-9_]{1,15}$'),
    on_failure        TEXT NOT NULL CHECK (on_failure IN ('open', 'closed')),
    set_version       INT NOT NULL CHECK (set_version >= 1),
    change_request_id UUID NOT NULL REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    activated_by      UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    activated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    superseded_at     TIMESTAMPTZ,
    CONSTRAINT uq_assess_fail_policy_set UNIQUE (rail, corridor, currency, amount_band, set_version)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_assess_fail_policy_current
    ON assess.fail_policies (rail, corridor, currency, amount_band) WHERE superseded_at IS NULL;

-- Mode per rail. A rail with no open row is not assessed. readiness is the shadow count snapshot the change was approved on.
CREATE TABLE IF NOT EXISTS assess.rail_modes (
    id                UUID PRIMARY KEY,
    rail              TEXT NOT NULL REFERENCES assess.rails(code) ON DELETE RESTRICT,
    mode              TEXT NOT NULL CHECK (mode IN ('shadow', 'advisory', 'enforcing')),
    effective_from    TIMESTAMPTZ NOT NULL,
    superseded_at     TIMESTAMPTZ,
    change_request_id UUID NOT NULL REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    activated_by      UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    readiness         JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(readiness) = 'object'),
    CHECK (superseded_at IS NULL OR superseded_at >= effective_from)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_assess_rail_mode_open ON assess.rail_modes (rail) WHERE superseded_at IS NULL;

-- Deterministic prohibited conditions (C-65). The condition uses a closed grammar: four keys only (checked below).
-- A row is a reject without a person; its rule_id is recorded on every decision it produces.
CREATE TABLE IF NOT EXISTS assess.prohibited_rules (
    rule_id           TEXT NOT NULL CHECK (rule_id ~ '^[A-Z][A-Z0-9_]{5,80}$'),
    version           INT NOT NULL CHECK (version >= 1),
    title             JSONB NOT NULL CHECK (title ? 'ar' AND title ? 'en'),
    condition         JSONB NOT NULL CHECK (jsonb_typeof(condition) = 'object' AND condition <> '{}'::jsonb
                                            AND (condition - ARRAY['corridor_in', 'currency_in', 'rail_in', 'amount_min']::text[]) = '{}'::jsonb),
    basis             TEXT NOT NULL CHECK (length(btrim(basis)) BETWEEN 1 AND 500),   -- the order or internal policy the rule implements
    status            TEXT NOT NULL CHECK (status IN ('active', 'retired')),
    change_request_id UUID NOT NULL REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    activated_by      UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    activated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    retired_at        TIMESTAMPTZ,
    PRIMARY KEY (rule_id, version),
    CHECK ((status = 'retired') = (retired_at IS NOT NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_assess_rule_active ON assess.prohibited_rules (rule_id) WHERE status = 'active';

-- Holds. A hold is the gateway-facing face of a decision that is not a pass. mode advisory and enforcing create holds;
-- shadow creates kind review rows with state shadow_review that touch no gateway and no alert (section 3.5).
CREATE TABLE IF NOT EXISTS assess.holds (
    id                    UUID PRIMARY KEY,                                  -- UUIDv7, app-side
    hold_ref              TEXT NOT NULL UNIQUE CHECK (hold_ref ~ '^[A-Za-z0-9_-]{22}$'),   -- 16 random bytes, base64url, opaque
    decision_id           UUID NOT NULL,
    decision_received_at  TIMESTAMPTZ NOT NULL,
    gateway_id            UUID NOT NULL REFERENCES assess.gateways(id) ON DELETE RESTRICT,
    rail                  TEXT NOT NULL REFERENCES assess.rails(code) ON DELETE RESTRICT,
    mode                  TEXT NOT NULL CHECK (mode IN ('shadow', 'advisory', 'enforcing')),
    kind                  TEXT NOT NULL CHECK (kind IN ('list_match', 'review')),
    reason_codes          TEXT[] NOT NULL CHECK (cardinality(reason_codes) >= 1),
    state                 TEXT NOT NULL CHECK (state IN ('held', 'advisory_open', 'shadow_review', 'released', 'rejected',
                                                         'expired', 'advisory_closed', 'shadow_closed')),
    proposal              TEXT CHECK (proposal IS NULL OR proposal IN ('release', 'reject')),   -- first of the two decisions
    proposal_reason_codes TEXT[] NOT NULL DEFAULT '{}',
    proposed_by           UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    proposed_at           TIMESTAMPTZ,
    decided_by            UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- the second person
    decided_at            TIMESTAMPTZ,
    alert_id              UUID REFERENCES alert.screening_alerts(id) ON DELETE RESTRICT,   -- list_match only; set by assess_hold_sweep
    subject_ref           UUID NOT NULL,                                     -- the payment subject; not a canonical party id
    party_names_enc       BYTEA,                                             -- sealed name forms (personal, encrypted)
    release_decision_id   UUID,
    release_received_at   TIMESTAMPTZ,
    expires_at            TIMESTAMPTZ NOT NULL,
    resolved_at           TIMESTAMPTZ,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    version               INT NOT NULL DEFAULT 1,
    FOREIGN KEY (decision_id, decision_received_at) REFERENCES assess.decisions (id, received_at) ON DELETE RESTRICT,
    FOREIGN KEY (release_decision_id, release_received_at) REFERENCES assess.decisions (id, received_at) ON DELETE RESTRICT,
    CHECK (alert_id IS NULL OR kind = 'list_match'),
    CHECK (mode <> 'shadow' OR state IN ('shadow_review', 'shadow_closed')),
    CHECK (state NOT IN ('released', 'rejected', 'expired', 'advisory_closed', 'shadow_closed') OR resolved_at IS NOT NULL),
    CHECK (proposed_by IS NULL OR decided_by IS NULL OR proposed_by <> decided_by),   -- four eyes (GV-32, 4E)
    CHECK ((release_decision_id IS NULL) = (release_received_at IS NULL))
);
CREATE INDEX IF NOT EXISTS idx_assess_holds_queue ON assess.holds (state, created_at)
    WHERE state IN ('held', 'advisory_open', 'shadow_review');
CREATE INDEX IF NOT EXISTS idx_assess_holds_alert ON assess.holds (alert_id) WHERE alert_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_assess_holds_decision ON assess.holds (decision_id);

-- The candidates of a hold (what the inline screen matched), written in the decision transaction. Names live only in
-- explanation_enc (sealed); the subject and entry fingerprints are the phase 14 values (section 3.5).
CREATE TABLE IF NOT EXISTS assess.hold_candidates (
    hold_id           UUID NOT NULL REFERENCES assess.holds(id) ON DELETE RESTRICT,
    seq               SMALLINT NOT NULL CHECK (seq >= 1),
    entry_id          UUID NOT NULL REFERENCES ref.list_entries(id) ON DELETE RESTRICT,
    list_id           UUID NOT NULL REFERENCES ref.lists(id) ON DELETE RESTRICT,
    list_version_id   UUID NOT NULL REFERENCES ref.list_versions(id) ON DELETE RESTRICT,
    list_kind         TEXT NOT NULL CHECK (list_kind IN ('freeze', 'pep', 'internal')),
    outcome           TEXT NOT NULL CHECK (outcome IN ('hit', 'needs_more_data')),
    score             NUMERIC(6,4) NOT NULL CHECK (score BETWEEN 0 AND 1),
    evidence_bits     NUMERIC(8,2) NOT NULL CHECK (evidence_bits >= 0),
    matching_major    SMALLINT NOT NULL CHECK (matching_major >= 1),
    subject_fp        BYTEA NOT NULL CHECK (octet_length(subject_fp) = 32),
    entry_fp          BYTEA NOT NULL CHECK (octet_length(entry_fp) = 32),
    explanation_enc   BYTEA NOT NULL,                                       -- sealed (personal): aligned pairs and checks
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (hold_id, seq)
);

-- Release-control checkpoint (singleton): the received_at up to which assess_release_match has read canon.
CREATE TABLE IF NOT EXISTS assess.control_state (
    id                      SMALLINT PRIMARY KEY CHECK (id = 1),
    release_checkpoint_at   TIMESTAMPTZ NOT NULL DEFAULT '1970-01-01T00:00:00Z',
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    version                 INT NOT NULL DEFAULT 1
);
INSERT INTO assess.control_state (id) VALUES (1) ON CONFLICT (id) DO NOTHING;

-- Released payments without a matching signed decision (C-59). One row per canonical payment and received_at.
CREATE TABLE IF NOT EXISTS assess.release_exceptions (
    id                  UUID PRIMARY KEY,
    payment_message_id  UUID NOT NULL,                  -- canon.payment_message.id (no foreign key: bi-temporal canon)
    payment_received_at TIMESTAMPTZ NOT NULL,
    match_key           TEXT CHECK (match_key IS NULL OR match_key ~ '^(uetr:[0-9a-f-]{36}|ref:[0-9a-f]{64})$'),
    rail                TEXT REFERENCES assess.rails(code) ON DELETE RESTRICT,
    reason_code         TEXT NOT NULL CHECK (reason_code IN ('no_decision', 'no_match_key', 'decision_not_pass', 'pass_expired')),
    decision_id         UUID,                           -- the non-pass decision (decision_not_pass) or the expired pass (pass_expired)
    found_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    register_id         UUID REFERENCES admin.registers(id) ON DELETE RESTRICT,
    closed_at           TIMESTAMPTZ,
    closed_by           UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    close_reason        TEXT CHECK (close_reason IS NULL OR close_reason IN ('explained', 'bank_confirmed_manual', 'false_match')),
    version             INT NOT NULL DEFAULT 1,
    CONSTRAINT uq_assess_release_exception UNIQUE (payment_message_id, payment_received_at),
    CHECK ((closed_at IS NULL) = (closed_by IS NULL)),
    CHECK (closed_at IS NULL OR close_reason IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_assess_release_open ON assess.release_exceptions (found_at) WHERE closed_at IS NULL;

-- Weekly control-test payments (section 3.14). result is caught when the gateway received hold.
CREATE TABLE IF NOT EXISTS assess.control_tests (
    id                    UUID PRIMARY KEY,
    rail                  TEXT NOT NULL REFERENCES assess.rails(code) ON DELETE RESTRICT,
    test_ref              TEXT NOT NULL UNIQUE CHECK (test_ref ~ '^CT-[0-9]{4}-[0-9]{2}-[0-9]{2}-[A-Z0-9]{4}$'),
    sent_at               TIMESTAMPTZ NOT NULL,
    observed              TEXT NOT NULL CHECK (observed IN ('hold', 'pass', 'reject', 'no_response')),
    observed_decision_id  UUID,
    result                TEXT NOT NULL CHECK (result IN ('caught', 'missed')),
    recorded_by           UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    recorded_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    evidence_object_id    UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    CHECK ((result = 'caught') = (observed = 'hold'))
);

-- Daily roll-up per rail, mode and day (section 3.12). Written by assess_rollup_watch as an upsert.
CREATE TABLE IF NOT EXISTS assess.mode_rollups (
    rail                    TEXT NOT NULL REFERENCES assess.rails(code) ON DELETE RESTRICT,
    day                     DATE NOT NULL,
    mode                    TEXT NOT NULL CHECK (mode IN ('shadow', 'advisory', 'enforcing')),
    decisions               BIGINT NOT NULL DEFAULT 0 CHECK (decisions >= 0),
    would_pass              BIGINT NOT NULL DEFAULT 0 CHECK (would_pass >= 0),
    would_hold              BIGINT NOT NULL DEFAULT 0 CHECK (would_hold >= 0),
    would_reject            BIGINT NOT NULL DEFAULT 0 CHECK (would_reject >= 0),
    reviews_closed          BIGINT NOT NULL DEFAULT 0 CHECK (reviews_closed >= 0),
    reviews_true_match      BIGINT NOT NULL DEFAULT 0 CHECK (reviews_true_match >= 0),
    holds_released          BIGINT NOT NULL DEFAULT 0 CHECK (holds_released >= 0),
    holds_rejected          BIGINT NOT NULL DEFAULT 0 CHECK (holds_rejected >= 0),
    holds_expired           BIGINT NOT NULL DEFAULT 0 CHECK (holds_expired >= 0),
    degraded_total          BIGINT NOT NULL DEFAULT 0 CHECK (degraded_total >= 0),
    repeat_assessments      BIGINT NOT NULL DEFAULT 0 CHECK (repeat_assessments >= 0),
    computed_at             TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (rail, day, mode)
);

-- Fallback list extracts (section 3.13). The document is evidence (owner kind assess_extract); this row is its index.
CREATE TABLE IF NOT EXISTS assess.fallback_extracts (
    id                   UUID PRIMARY KEY,
    extract_version      INT NOT NULL UNIQUE CHECK (extract_version >= 1),
    list_version_ids     UUID[] NOT NULL CHECK (cardinality(list_version_ids) >= 1),
    evidence_object_id   UUID NOT NULL REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,
    signing_key_version  INT NOT NULL CHECK (signing_key_version >= 1),
    row_count            INT NOT NULL CHECK (row_count >= 0),
    built_at             TIMESTAMPTZ NOT NULL,
    valid_until          TIMESTAMPTZ NOT NULL,
    CHECK (valid_until > built_at)
);

-- Seed the decision partitions: the current month and the three ahead (audit.partition_ahead_months, phase 02).
DO $$
DECLARE
    m DATE := date_trunc('month', now())::date;
    i INT;
BEGIN
    FOR i IN 0..3 LOOP
        PERFORM assess.create_partition(EXTRACT(YEAR FROM m + (i || ' months')::interval)::int,
                                        EXTRACT(MONTH FROM m + (i || ' months')::interval)::int);
    END LOOP;
END $$;
```

### 1.3 `0093_assess_guards_grants.sql`

```sql
-- migration 0093_assess_guards_grants.sql
-- Phase 29. Guards (append-only decisions, hold transitions, change-request-only policy tables) and grants per write
-- path. akr_retention gets SELECT on every new table and DELETE on assess.node_status only (DR-386). akr_readonly
-- gets nothing on assess.
-- Idempotent: functions are CREATE OR REPLACE, triggers are dropped and created, grants are repeatable.

-- Change-request guard. The policy tables are written only by an activation whose change request is approved or
-- effective and has the kind of the table (phase 04 object kinds assess_policy, assess_mode, assess_rule).
CREATE OR REPLACE FUNCTION assess.require_change(p_cr UUID, p_kind TEXT) RETURNS VOID
LANGUAGE plpgsql SET search_path = pg_catalog AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM admin.change_requests c
                    WHERE c.id = p_cr AND c.object_kind = p_kind AND c.status IN ('approved', 'effective')) THEN
        RAISE EXCEPTION 'change_request_required: a change request of kind % is needed', p_kind
            USING ERRCODE = 'check_violation';
    END IF;
END $$;
ALTER FUNCTION assess.require_change(UUID, TEXT) OWNER TO akr_migrate;
REVOKE ALL ON FUNCTION assess.require_change(UUID, TEXT) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION assess.require_change(UUID, TEXT) TO akr_app;

CREATE OR REPLACE FUNCTION assess.decisions_append_only() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog AS $$
BEGIN
    RAISE EXCEPTION 'assess.decisions is append-only; rows leave only with their partition (phase 29, DR-313)'
        USING ERRCODE = 'insufficient_privilege';
END $$;
DROP TRIGGER IF EXISTS trg_assess_decisions_append_only ON assess.decisions;
CREATE TRIGGER trg_assess_decisions_append_only BEFORE UPDATE OR DELETE ON assess.decisions
    FOR EACH ROW EXECUTE FUNCTION assess.decisions_append_only();

-- Holds: identity columns are fixed, rows are never deleted, and the state moves only along the edges of section 3.5.
CREATE OR REPLACE FUNCTION assess.holds_guard() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'assess.holds rows are never deleted' USING ERRCODE = 'check_violation';
    END IF;
    IF (NEW.id, NEW.hold_ref, NEW.decision_id, NEW.decision_received_at, NEW.gateway_id, NEW.rail, NEW.mode, NEW.kind,
        NEW.reason_codes, NEW.subject_ref, NEW.expires_at, NEW.created_at)
       IS DISTINCT FROM
       (OLD.id, OLD.hold_ref, OLD.decision_id, OLD.decision_received_at, OLD.gateway_id, OLD.rail, OLD.mode, OLD.kind,
        OLD.reason_codes, OLD.subject_ref, OLD.expires_at, OLD.created_at) THEN
        RAISE EXCEPTION 'assess.holds: identity columns are fixed' USING ERRCODE = 'check_violation';
    END IF;
    IF NEW.state IS DISTINCT FROM OLD.state AND NOT (
           (OLD.state = 'held' AND NEW.state IN ('released', 'rejected', 'expired'))
        OR (OLD.state = 'advisory_open' AND NEW.state = 'advisory_closed')
        OR (OLD.state = 'shadow_review' AND NEW.state = 'shadow_closed')) THEN
        RAISE EXCEPTION 'assess.holds: illegal transition % to %', OLD.state, NEW.state USING ERRCODE = 'check_violation';
    END IF;
    IF OLD.state IN ('released', 'rejected', 'expired', 'advisory_closed', 'shadow_closed')
       AND (NEW.proposal, NEW.proposal_reason_codes, NEW.proposed_by, NEW.proposed_at, NEW.decided_by, NEW.decided_at,
            NEW.alert_id, NEW.resolved_at)
           IS DISTINCT FROM
           (OLD.proposal, OLD.proposal_reason_codes, OLD.proposed_by, OLD.proposed_at, OLD.decided_by, OLD.decided_at,
            OLD.alert_id, OLD.resolved_at) THEN
        RAISE EXCEPTION 'assess.holds: a resolved hold is final' USING ERRCODE = 'check_violation';
    END IF;
    IF NEW.release_decision_id IS DISTINCT FROM OLD.release_decision_id
       AND NOT (OLD.state = 'released' AND OLD.release_decision_id IS NULL) THEN
        RAISE EXCEPTION 'assess.holds: the release pass is issued once, for a released hold' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_assess_holds_guard ON assess.holds;
CREATE TRIGGER trg_assess_holds_guard BEFORE UPDATE OR DELETE ON assess.holds
    FOR EACH ROW EXECUTE FUNCTION assess.holds_guard();

CREATE OR REPLACE FUNCTION assess.fail_policies_guard() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        PERFORM assess.require_change(NEW.change_request_id, 'assess_policy');
        RETURN NEW;
    END IF;
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'assess.fail_policies rows are superseded, never deleted' USING ERRCODE = 'check_violation';
    END IF;
    IF (NEW.id, NEW.rail, NEW.corridor, NEW.currency, NEW.amount_band, NEW.on_failure, NEW.set_version,
        NEW.change_request_id) IS DISTINCT FROM
       (OLD.id, OLD.rail, OLD.corridor, OLD.currency, OLD.amount_band, OLD.on_failure, OLD.set_version,
        OLD.change_request_id) THEN
        RAISE EXCEPTION 'assess.fail_policies: only superseded_at may change' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_assess_fail_policies_guard ON assess.fail_policies;
CREATE TRIGGER trg_assess_fail_policies_guard BEFORE INSERT OR UPDATE OR DELETE ON assess.fail_policies
    FOR EACH ROW EXECUTE FUNCTION assess.fail_policies_guard();

CREATE OR REPLACE FUNCTION assess.rail_modes_guard() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        PERFORM assess.require_change(NEW.change_request_id, 'assess_mode');
        RETURN NEW;
    END IF;
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'assess.rail_modes rows are superseded, never deleted' USING ERRCODE = 'check_violation';
    END IF;
    IF (NEW.id, NEW.rail, NEW.mode, NEW.effective_from, NEW.change_request_id, NEW.readiness)
       IS DISTINCT FROM (OLD.id, OLD.rail, OLD.mode, OLD.effective_from, OLD.change_request_id, OLD.readiness) THEN
        RAISE EXCEPTION 'assess.rail_modes: only superseded_at may change' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_assess_rail_modes_guard ON assess.rail_modes;
CREATE TRIGGER trg_assess_rail_modes_guard BEFORE INSERT OR UPDATE OR DELETE ON assess.rail_modes
    FOR EACH ROW EXECUTE FUNCTION assess.rail_modes_guard();

CREATE OR REPLACE FUNCTION assess.prohibited_rules_guard() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        PERFORM assess.require_change(NEW.change_request_id, 'assess_rule');
        RETURN NEW;
    END IF;
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'assess.prohibited_rules rows are retired, never deleted' USING ERRCODE = 'check_violation';
    END IF;
    IF (NEW.rule_id, NEW.version, NEW.title, NEW.condition, NEW.basis, NEW.change_request_id)
       IS DISTINCT FROM (OLD.rule_id, OLD.version, OLD.title, OLD.condition, OLD.basis, OLD.change_request_id) THEN
        RAISE EXCEPTION 'assess.prohibited_rules: only status and retired_at may change' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_assess_prohibited_rules_guard ON assess.prohibited_rules;
CREATE TRIGGER trg_assess_prohibited_rules_guard BEFORE INSERT OR UPDATE OR DELETE ON assess.prohibited_rules
    FOR EACH ROW EXECUTE FUNCTION assess.prohibited_rules_guard();

CREATE OR REPLACE FUNCTION assess.amount_bands_guard() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'assess.amount_bands rows are replaced by a new set version, never deleted' USING ERRCODE = 'check_violation';
    END IF;
    PERFORM assess.require_change(NEW.change_request_id, 'assess_policy');
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_assess_amount_bands_guard ON assess.amount_bands;
CREATE TRIGGER trg_assess_amount_bands_guard BEFORE INSERT OR UPDATE OR DELETE ON assess.amount_bands
    FOR EACH ROW EXECUTE FUNCTION assess.amount_bands_guard();

-- Gateway identities: a revocation is the only change after registration, and rows are never deleted.
CREATE OR REPLACE FUNCTION assess.gateways_guard() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'assess.gateways rows are never deleted' USING ERRCODE = 'check_violation';
    END IF;
    IF (NEW.id, NEW.gateway_key, NEW.user_id, NEW.cert_sha256, NEW.not_before, NEW.not_after, NEW.registered_by)
       IS DISTINCT FROM (OLD.id, OLD.gateway_key, OLD.user_id, OLD.cert_sha256, OLD.not_before, OLD.not_after, OLD.registered_by) THEN
        RAISE EXCEPTION 'assess.gateways: a certificate pin is fixed; register a new row' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS trg_assess_gateways_guard ON assess.gateways;
CREATE TRIGGER trg_assess_gateways_guard BEFORE UPDATE OR DELETE ON assess.gateways
    FOR EACH ROW EXECUTE FUNCTION assess.gateways_guard();

-- Grants per write path (UPDATE column lists are exact).
-- akr_assess: the decision path, the gateway identity read, the hold release pass, heartbeats. Nothing else.
GRANT SELECT ON assess.rails, assess.gateways, assess.signing_keys, assess.policy_state, assess.amount_bands,
                assess.fail_policies, assess.rail_modes, assess.prohibited_rules, assess.decisions, assess.idempotency,
                assess.holds, assess.hold_candidates, assess.node_status
    TO akr_assess;
GRANT INSERT ON assess.decisions, assess.idempotency, assess.holds, assess.hold_candidates, assess.node_status TO akr_assess;
GRANT UPDATE (state, lease_until, completed_at) ON assess.idempotency TO akr_assess;
GRANT DELETE ON assess.idempotency TO akr_assess;                   -- in_flight rows of a refused or unparsed request only
GRANT UPDATE (release_decision_id, release_received_at, updated_at, version) ON assess.holds TO akr_assess;
GRANT UPDATE (started_at, last_seen_at, build_version, policy_version, mode_summary, version) ON assess.node_status TO akr_assess;
GRANT SELECT (id, kind, status, expires_at) ON admin.users TO akr_assess;   -- the gateway's service principal; no names or emails in the gateway zone
GRANT SELECT ON admin.settings, admin.feature_flags TO akr_assess;
GRANT SELECT ON ref.lists, ref.list_versions, ref.list_entries, ref.list_entry_names TO akr_assess;   -- phase 14 grants, restated

-- akr_app: the people routes, the hold sweep, the release matcher, the change targets, the roll-ups and the sweeps.
GRANT SELECT ON assess.rails, assess.gateways, assess.signing_keys, assess.node_status, assess.blind_periods,
                assess.policy_state, assess.amount_bands, assess.fail_policies, assess.rail_modes,
                assess.prohibited_rules, assess.holds, assess.hold_candidates, assess.control_state,
                assess.release_exceptions, assess.control_tests, assess.mode_rollups, assess.decisions,
                assess.idempotency, assess.fallback_extracts
    TO akr_app;
GRANT INSERT ON assess.rails, assess.gateways, assess.signing_keys, assess.blind_periods, assess.control_tests,
                assess.release_exceptions, assess.mode_rollups, assess.fail_policies, assess.rail_modes,
                assess.prohibited_rules, assess.amount_bands, assess.fallback_extracts TO akr_app;
GRANT UPDATE (active, label, updated_at, version) ON assess.rails TO akr_app;
GRANT UPDATE (revoked_at, revoked_by, revoke_reason, status, version) ON assess.gateways TO akr_app;
GRANT UPDATE (status, wrapped_seed, retired_at) ON assess.signing_keys TO akr_app;
GRANT UPDATE (superseded_at) ON assess.fail_policies, assess.rail_modes TO akr_app;
GRANT UPDATE (status, retired_at) ON assess.prohibited_rules TO akr_app;
GRANT UPDATE (upper_bound, lower_bound, set_version, change_request_id) ON assess.amount_bands TO akr_app;
GRANT UPDATE (set_version, updated_at, version) ON assess.policy_state TO akr_app;
GRANT INSERT ON assess.holds, assess.hold_candidates TO akr_app;
GRANT UPDATE (alert_id, state, proposal, proposal_reason_codes, proposed_by, proposed_at, decided_by, decided_at,
              resolved_at, updated_at, version) ON assess.holds TO akr_app;
GRANT UPDATE (closed_at, closed_by, close_reason, register_id, version) ON assess.release_exceptions TO akr_app;
GRANT UPDATE (release_checkpoint_at, updated_at, version) ON assess.control_state TO akr_app;
GRANT UPDATE (ended_at, version) ON assess.blind_periods TO akr_app;
GRANT UPDATE (decisions, would_pass, would_hold, would_reject, reviews_closed, reviews_true_match, holds_released,
              holds_rejected, holds_expired, degraded_total, repeat_assessments, computed_at)
    ON assess.mode_rollups TO akr_app;
GRANT DELETE ON assess.idempotency TO akr_app;                      -- assess_ledger_sweep, rows older than the window
GRANT EXECUTE ON FUNCTION assess.create_partition(INT, INT) TO akr_app;
-- Insert of the signing key row by akr-ctl (akr_app) needs the admin.users read above; the seed is wrapped by OpenBao.

-- akr_retention: SELECT on every new table (phase 23 grants only tables that existed at its migration); DELETE only
-- on assess.node_status, the one table whose handler deletes rows (record_purge, R12 age; DR-386).
GRANT SELECT ON assess.rails, assess.gateways, assess.signing_keys, assess.decisions, assess.idempotency,
                        assess.node_status, assess.blind_periods, assess.policy_state, assess.amount_bands,
                        assess.fail_policies, assess.rail_modes, assess.prohibited_rules, assess.holds,
                        assess.hold_candidates, assess.control_state, assess.release_exceptions, assess.control_tests,
                        assess.mode_rollups, assess.fallback_extracts
    TO akr_retention;
GRANT DELETE ON assess.node_status TO akr_retention;

-- akr_readonly and akr_audit_writer receive nothing on schema assess (no widening, phase 24 rule).
-- Audit rows of akr_assess go through the audit sink as a member of akr_audit_writer (phase 02), not through a grant here.
```

### 1.4 `0094_assess_settings_classes.sql`

```sql
-- migration 0094_assess_settings_classes.sql
-- Phase 29. The widened phase 24 registers.source CHECK (new value assess_control, DR-307), the pack flag, settings
-- (copy of the phase 13 INSERT column list), column classification and retention. Idempotent.

-- Phase 24 CHECK widened (state here as phase 27 and 28 did). Current list: phase 27 added model_monitor and phase 28
-- added assistant_monitor; this phase adds assess_control.
ALTER TABLE admin.registers DROP CONSTRAINT IF EXISTS registers_source_check;
ALTER TABLE admin.registers ADD CONSTRAINT registers_source_check
    CHECK (source IN ('manual', 'post_review', 'fairness', 'review_calendar', 'scan', 'model_monitor',
                      'assistant_monitor', 'assess_control'));

-- The optional pack (phase 26 pattern: flag off means every /api/v1/assess route answers 404 E_NOT_FOUND and akr-assess
-- answers 404 on /assess/v1/payments). Default off: no rail is assessed until the bank switches the pack on.
INSERT INTO admin.feature_flags (key, enabled, scope, label) VALUES
 ('pack.assess', FALSE, '{}',
  '{"ar": "حزمة التقييم قبل الإطلاق", "en": "Pre-release assessment pack (phase 2)"}')
ON CONFLICT (key) DO NOTHING;

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('assess.payment_expiry_s', '600', 'duration_s',
  '{"max": 600, "basis": "API.md 4.2: a pass is valid for at most ten minutes; a longer window lets a stale pass release"}', TRUE,
  '{"ar": "مدة صلاحية القرار الموقّع (ثوان)", "en": "Lifetime of a signed decision (seconds)"}'),
 ('assess.idempotency_hours', '48', 'integer',
  '{"min": 24, "basis": "gateway retry window; replays after the window are answered from the decision ledger by GET"}', TRUE,
  '{"ar": "نافذة إعادة الطلب (ساعات)", "en": "Replay window of an idempotency key (hours)"}'),
 ('assess.hold_list_kinds', '["freeze"]', 'json', NULL, TRUE,
  '{"ar": "أنواع القوائم التي يعلّق تطابقها الدفع", "en": "List kinds whose hits hold a payment (must contain freeze)"}'),
 ('assess.domestic_country', '"IQ"', 'string', NULL, TRUE,
  '{"ar": "رمز البلد المحلي (لا يُعد الدفع عابراً للحدود)", "en": "Domestic country code (payments to it are not cross-border)"}'),
 ('assess.inline_timeout_ms', '250', 'integer',
  '{"max": 250, "basis": "research 22 N-51: request budget with the partial flag; a larger value only hides partial screens"}', TRUE,
  '{"ar": "مهلة الفحص المباشر (ملي ثانية)", "en": "Hard timeout of the inline screen (milliseconds)"}'),
 ('assess.rate_per_second', '50', 'integer', NULL, FALSE,
  '{"ar": "حد الطلبات لكل بوابة في الثانية", "en": "Requests per second per gateway"}'),
 ('assess.rate_burst', '500', 'integer', NULL, FALSE,
  '{"ar": "سعة الانفجار لكل بوابة", "en": "Burst size per gateway"}'),
 ('assess.gateway_cache_s', '30', 'duration_s',
  '{"max": 30, "basis": "a revoked certificate is refused within thirty seconds on every node"}', TRUE,
  '{"ar": "مدة تخزين هوية البوابة (ثوان)", "en": "Cache of gateway identities (seconds)"}'),
 ('assess.heartbeat_max_s', '90', 'duration_s',
  '{"max": 120, "basis": "an outage is recorded after this silence; a longer silence hides a blind period"}', TRUE,
  '{"ar": "أقصى صمت لنبض العقدة قبل تسجيل فترة العمى (ثوان)", "en": "Silence before a node counts as down (seconds)"}'),
 ('assess.hold_max_hours', '24', 'integer',
  '{"max": 72, "basis": "a payment held longer than the gateway return window is expired and never passed"}', TRUE,
  '{"ar": "أقصى مدة للتعليق (ساعات)", "en": "Longest time a hold stays open (hours)"}'),
 ('assess.release_window_min', '120', 'integer',
  '{"min": 60, "basis": "the release matcher must see the decision made before a release, with margin for the gateway queue"}', TRUE,
  '{"ar": "نافذة مطابقة الإفراج بدون قرار (دقائق)", "en": "Window in which a release must match a decision (minutes)"}'),
 ('assess.release_skew_s', '60', 'duration_s',
  '{"max": 120, "basis": "API.md 4.2: a pass is valid until expires_at; the skew covers clock drift between the gateway and the canonical copy, a larger value lets a stale pass release unseen"}', TRUE,
  '{"ar": "هامش انحراف الساعة عند مطابقة الإفراج (ثوان)", "en": "Clock skew allowed when a release is matched to a pass (seconds)"}'),
 ('assess.shadow_min_days', '30', 'integer',
  '{"min": 30, "basis": "entry criterion of advisory: a full month of shadow decisions (research 25 15.9)"}', TRUE,
  '{"ar": "أدنى مدة للوضع التجريبي قبل الإشعار (أيام)", "en": "Minimum days in shadow before advisory is requested"}'),
 ('assess.advisory_min_days', '14', 'integer',
  '{"min": 14, "basis": "entry criterion of enforcing: two weeks of advisory decisions with the analysts working the queue"}', TRUE,
  '{"ar": "أدنى مدة للوضع الاستشاري قبل الإنفاذ (أيام)", "en": "Minimum days in advisory before enforcing is requested"}'),
 ('assess.shadow_min_decisions', '1000', 'integer',
  '{"min": 500, "basis": "entry criterion of advisory: enough shadow decisions for a rate estimate (proposal, bank confirms)"}', TRUE,
  '{"ar": "أدنى عدد قرارات تجريبية قبل الإشعار", "en": "Minimum shadow decisions before advisory is requested"}'),
 ('assess.shadow_review_min_ratio', '0.9', 'decimal',
  '{"min": 0.8, "basis": "entry criterion: share of would-be holds (shadow) and of advisory holds closed by a person"}', TRUE,
  '{"ar": "أدنى نسبة مراجعة للتعليقات التجريبية والاستشارية", "en": "Minimum share of shadow and advisory holds closed by a person"}'),
 ('assess.control_test_max_days', '7', 'integer',
  '{"max": 7, "basis": "research 25 15.6: a weekly control-test payment"}', TRUE,
  '{"ar": "أقصى فترة بين اختبارين للرقابة (أيام)", "en": "Longest gap between control-test payments (days)"}'),
 ('assess.repeat_alert_count', '3', 'integer', NULL, FALSE,
  '{"ar": "عدد المحاولات للدفعة نفسها قبل تنبيه التكرار", "en": "Assessments of one payment key that raise a repeat warning"}'),
 ('assess.fallback_extract_hours', '12', 'integer',
  '{"max": 12, "basis": "CBI rule: freeze lists refresh at least every 12 hours; the signed extract follows the list"}', TRUE,
  '{"ar": "أقصى عمر للمستخرج الاحتياطي للقوائم (ساعات)", "en": "Longest age of the signed fallback extract (hours)"}'),
 ('schedule.assess_release_match', '"0 */15 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مطابقة الإفراج بدون قرار", "en": "Release-without-decision matcher schedule"}'),
 ('schedule.assess_hold_sweep', '"0 * * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول فحص التعليقات", "en": "Hold sweep schedule"}'),
 ('schedule.assess_blind_watch', '"0 * * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مراقبة نبضات العقد", "en": "Node heartbeat watch schedule"}'),
 ('schedule.assess_rollup_watch', '"0 40 0 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول الإحصاءات اليومية للوضع التجريبي والمراقبة", "en": "Daily roll-up and watch schedule"}'),
 ('schedule.assess_fallback_extract', '"0 30 5 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول المستخرج الاحتياطي للقوائم", "en": "Fallback list extract schedule"}'),
 ('schedule.assess_ledger_sweep', '"0 10 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول تنظيف سجل إعادة الطلب", "en": "Replay ledger sweep schedule"}')
ON CONFLICT (key) DO NOTHING;

-- Column classification (copy of the phase 27 and 28 format). Encrypted columns are personal and sealed (phase 02).
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'assess', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('rails', 'internal', ARRAY['code', 'label', 'payload_formats', 'active', 'created_at', 'updated_at', 'version']),
  ('gateways', 'internal', ARRAY['id', 'gateway_key', 'user_id', 'cert_sha256', 'cert_subject', 'not_before', 'not_after',
                                 'status', 'revoke_reason', 'registered_by', 'registered_at', 'revoked_at', 'revoked_by', 'version']),
  ('signing_keys', 'internal', ARRAY['purpose', 'version', 'status', 'public_key', 'fingerprint', 'created_by', 'created_at', 'retired_at']),
  ('signing_keys', 'secret-str', ARRAY['wrapped_seed']),
  ('decisions', 'internal', ARRAY['id', 'received_at', 'kind', 'gateway_id', 'rail', 'direction', 'payload_format', 'corridor',
                                  'currency', 'amount_band', 'request_sha256', 'decision', 'mode', 'would_be', 'fail_policy',
                                  'policy_version', 'reason_codes', 'rule_id', 'degraded', 'list_version_ids', 'hold_id',
                                  'key_version', 'response_bytes', 'signature', 'expires_at', 'latency_ms']),
  ('decisions', 'financial', ARRAY['match_key']),
  ('decisions', 'personal', ARRAY['matched_party_ids']),
  ('idempotency', 'internal', ARRAY['gateway_id', 'idem_key', 'request_sha256', 'decision_id', 'decision_received_at', 'state',
                                    'lease_until', 'created_at', 'completed_at']),
  ('node_status', 'internal', ARRAY['node_id', 'started_at', 'last_seen_at', 'build_version', 'policy_version', 'mode_summary', 'version']),
  ('blind_periods', 'internal', ARRAY['id', 'started_at', 'ended_at', 'reason_code', 'last_seen_at', 'recorded_by', 'version']),
  ('amount_bands', 'internal', ARRAY['currency', 'band_code', 'lower_bound', 'upper_bound', 'set_version', 'change_request_id']),
  ('fail_policies', 'internal', ARRAY['id', 'rail', 'corridor', 'currency', 'amount_band', 'on_failure', 'set_version',
                                      'change_request_id', 'activated_by', 'activated_at', 'superseded_at']),
  ('rail_modes', 'internal', ARRAY['id', 'rail', 'mode', 'effective_from', 'superseded_at', 'change_request_id', 'activated_by', 'readiness']),
  ('policy_state', 'internal', ARRAY['id', 'set_version', 'updated_at', 'version']),
  ('prohibited_rules', 'internal', ARRAY['rule_id', 'version', 'title', 'basis', 'status', 'change_request_id', 'activated_by',
                                         'activated_at', 'retired_at']),
  ('prohibited_rules', 'detection-logic', ARRAY['condition']),
  ('holds', 'internal', ARRAY['id', 'hold_ref', 'decision_id', 'decision_received_at', 'gateway_id', 'rail', 'mode', 'kind',
                              'reason_codes', 'state', 'proposal', 'proposal_reason_codes', 'proposed_by', 'proposed_at',
                              'decided_by', 'decided_at', 'alert_id', 'subject_ref', 'release_decision_id', 'release_received_at',
                              'expires_at', 'resolved_at', 'created_at', 'updated_at', 'version']),
  ('holds', 'personal', ARRAY['party_names_enc']),
  ('hold_candidates', 'internal', ARRAY['hold_id', 'seq', 'entry_id', 'list_id', 'list_version_id', 'list_kind', 'outcome', 'score',
                                        'evidence_bits', 'matching_major', 'subject_fp', 'entry_fp', 'created_at']),
  ('hold_candidates', 'personal', ARRAY['explanation_enc']),
  ('control_state', 'internal', ARRAY['id', 'release_checkpoint_at', 'updated_at', 'version']),
  ('release_exceptions', 'internal', ARRAY['id', 'payment_message_id', 'payment_received_at', 'rail', 'reason_code', 'decision_id',
                                           'found_at', 'register_id', 'closed_at', 'closed_by', 'close_reason', 'version']),
  ('release_exceptions', 'financial', ARRAY['match_key']),
  ('control_tests', 'internal', ARRAY['id', 'rail', 'test_ref', 'sent_at', 'observed', 'observed_decision_id', 'result',
                                      'recorded_by', 'recorded_at', 'evidence_object_id']),
  ('fallback_extracts', 'internal', ARRAY['id', 'extract_version', 'list_version_ids', 'evidence_object_id', 'signing_key_version',
                                          'row_count', 'built_at', 'valid_until']),
  ('mode_rollups', 'internal', ARRAY['rail', 'day', 'mode', 'decisions', 'would_pass', 'would_hold', 'would_reject', 'reviews_closed',
                                     'reviews_true_match', 'holds_released', 'holds_rejected', 'holds_expired', 'degraded_total',
                                     'repeat_assessments', 'computed_at'])
) AS v(t, cls, cols) CROSS JOIN LATERAL unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

UPDATE admin.column_classification SET encrypted = TRUE
 WHERE (schema_name, table_name, column_name) IN (
   ('assess', 'holds', 'party_names_enc'),
   ('assess', 'hold_candidates', 'explanation_enc'));

-- Retention (R7 decisions, holds and screening roll-ups; R10 outage evidence; R11 control tests; R12 operations; R9
-- configuration). A clock column is set exactly for partition, age, closure and decision (phase 00 CHECK). R10 is the
-- only class with clock never (phase 00 CHECK), so an unlimited record is R10 and a configuration record is R9.
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('assess', 'rails',              'R9',  'retirement', NULL,          'Rail codes; a retired rail keeps the decisions that name it'),
 ('assess', 'gateways',           'R9',  'retirement', NULL,          'Gateway identities; revoked rows kept (who was trusted when)'),
 ('assess', 'signing_keys',       'R9',  'retirement', NULL,          'Verify keys kept for every decision they signed'),
 ('assess', 'decisions',          'R7',  'partition',  'received_at', 'Phase 2 decisions: seven years (ARCHITECTURE s.17); partitions leave by admin.retention_drop_partition'),
 ('assess', 'idempotency',        'R12', 'age',        'created_at',  'Replay ledger: assess.idempotency_hours, swept by assess_ledger_sweep'),
 ('assess', 'node_status',        'R12', 'age',        'last_seen_at', 'Node heartbeats: operations evidence'),
 ('assess', 'blind_periods',      'R10', 'never',      NULL,          'Outage periods are control evidence kept without limit'),
 ('assess', 'amount_bands',       'R9',  'retirement', NULL,          'Band sets of the fail policy'),
 ('assess', 'fail_policies',      'R9',  'retirement', NULL,          'Fail policy history: the matrix each decision used'),
 ('assess', 'rail_modes',         'R9',  'retirement', NULL,          'Mode history with the readiness snapshot of each change'),
 ('assess', 'policy_state',       'R9',  'retirement', NULL,          'Singleton policy set counter'),
 ('assess', 'prohibited_rules',   'R9',  'retirement', NULL,          'Prohibited conditions with their basis and change request'),
 ('assess', 'holds',              'R7',  'closure',    'resolved_at', 'Held payments: seven years after resolution; an open hold is never due'),
 ('assess', 'hold_candidates',    'R7',  'owner',      NULL,          'Follows its hold'),
 ('assess', 'control_state',      'R9',  'retirement', NULL,          'Release-control checkpoint (singleton)'),
 ('assess', 'release_exceptions', 'R7',  'closure',    'closed_at',   'Control exceptions: seven years after closure; an open exception is never due'),
 ('assess', 'control_tests',      'R11', 'age',        'recorded_at', 'Control-test evidence: ten years (DATA-MODEL R11)'),
 ('assess', 'fallback_extracts',  'R7',  'age',        'built_at',    'Fallback extract index; the document itself is sealed evidence'),
 ('assess', 'mode_rollups',       'R7',  'age',        'day',         'Statistics derived from decisions and holds')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

---

## 2. DDD Implementation (Rust)

**Crate placement** (phase 00 workspace): pure domain in `domain/assess` (no driver, no I/O: modes, outcomes, reason codes, the hold state machine, the prohibited-rule condition grammar, the canonical decision bytes); repository ports in the same module; implementations in `infrastructure/pg/assess`; the gateway HTTP surface in `assess/src/routes/gateway`, the decision path in `assess/src/decision`, the parser in `assess/src/parse`, the signer in `assess/src/signing`. `akr-assess` depends on `domain/assess`, the phase 14 `names` and `screening` crates (the inline screen reuses them, N-49), `audit`, `crypto` and `ids`. It does not depend on `akr-sources` (phase 00 `deny.toml` check).

- **Domain entities** (`Debug, Clone, Serialize, Deserialize, FromRow` where stored; `ToSchema` where returned): `Rail { code, label, payload_formats, active }`; `Gateway { id, gateway_key, user_id, cert_sha256, not_after, status }`; `SigningKey { purpose, version, status, public_key, fingerprint }` (the wrapped seed is read only by `DecisionSigner::load`, never returned); `Decision { id, received_at, kind, gateway_id, rail, direction, payload_format, corridor, currency, amount_band, match_key, decision, mode, would_be, fail_policy, policy_version, reason_codes, rule_id, degraded, list_version_ids, matched_party_ids, hold_id, key_version, expires_at, latency_ms }` (`response_bytes` and `signature` are returned only by the replay path); `Hold { id, hold_ref, decision_id, decision_received_at, gateway_id, rail, mode, kind, reason_codes, state, proposal, proposal_reason_codes, proposed_by, proposed_at, decided_by, decided_at, alert_id, subject_ref, release_decision_id, expires_at, resolved_at, version }`; `HoldCandidate { seq, entry_id, list_id, list_version_id, list_kind, outcome, score, evidence_bits, matching_major, subject_fp, entry_fp }`; `FailPolicy { rail, corridor, currency, amount_band, on_failure, set_version }`; `AmountBand { currency, band_code, lower_bound, upper_bound, set_version }`; `RailMode { rail, mode, effective_from, superseded_at, change_request_id, readiness }`; `ProhibitedRule { rule_id, version, title, condition: RuleCondition, basis, status }`; `ReleaseException`; `ControlTest`; `BlindPeriod`; `ModeRollup`; `NodeStatus`.
- **Enums and closed sets**: `Mode { Shadow, Advisory, Enforcing }`; `Outcome { Pass, Hold, Reject }` (the answer); `DecisionKind { Assessment, HoldRelease }`; `OnFailure { Open, Closed }`; `HoldState { Held, AdvisoryOpen, ShadowReview, Released, Rejected, Expired, AdvisoryClosed, ShadowClosed }` with `HoldState::can_move(to)` mirroring the trigger of migration 0093; `ReasonCode { ListMatch, ListReview, ListsStale, ScreenPartial, MissingData, ProhibitedRule }` (the decision reasons; hold reasons use the phase 09 family `decision`); `DegradedFlag { ListsStale, ScreeningPartial, PartyUnnamed }` (no ML flag: no model runs, DR-302); `PayloadFormat { Mt103, Pacs008, Canonical }`; `ReleaseReason { NoDecision, NoMatchKey, DecisionNotPass, PassExpired }`; `BlindReason { NoHeartbeat, OperatorRecorded }`.
- **Create and update DTOs**: `NewDecision` (all columns of `assess.decisions` except `id` and `received_at`, which the caller supplies); `NewHold` and `NewHoldCandidate`; `HoldDecisionInput { outcome: release | reject, reason_codes: Vec<String>, text: Option<String> (at most 500 characters, sealed) }`; `PolicySetProposal { matrix: Vec<FailPolicyRow>, bands: Vec<AmountBand>, basis: String }` with `FailPolicyRow { rail, corridor, currency, amount_band, on_failure }` (the `set_version` of a proposed band is ignored; the activation sets it); `ModeProposal { mode: Mode }`; `RuleProposal { rule_id, title, condition, basis, status }`; `ControlTestRecord { rail, test_ref, sent_at, observed, observed_decision_id, evidence_object_id }`; `DecisionSearch { gateway_key, rail, decision, mode, from, to, uetr_hash, purpose, cursor, limit }` (limit at most 200).
- **Repository ports** (domain traits, `async_trait`, `conn: &mut PgConnection` for reads and `tx: &mut Tx` for writes):

```rust
pub trait AssessDecisionRepository: Send + Sync {
    async fn ledger_begin(&self, tx: &mut Tx, gw: Uuid, key: &str, sha: [u8; 32], decision_id: Uuid,
                          now: DateTime<Utc>, lease_until: DateTime<Utc>) -> Result<LedgerBegin, RepoError>;
    async fn ledger_take_over(&self, tx: &mut Tx, gw: Uuid, key: &str, now: DateTime<Utc>, lease_until: DateTime<Utc>) -> Result<bool, RepoError>;
    async fn ledger_done(&self, tx: &mut Tx, gw: Uuid, key: &str, now: DateTime<Utc>) -> Result<(), RepoError>;
    async fn ledger_forget(&self, tx: &mut Tx, gw: Uuid, key: &str) -> Result<(), RepoError>;   // in_flight rows only
    async fn insert_decision(&self, tx: &mut Tx, d: &NewDecision, response: &[u8], signature: &[u8; 64]) -> Result<(), RepoError>;
    async fn decision_bytes(&self, conn: &mut PgConnection, gw: Uuid, id: Uuid) -> Result<Option<StoredResponse>, RepoError>;
    async fn insert_holds(&self, tx: &mut Tx, holds: &[NewHold], candidates: &[NewHoldCandidate]) -> Result<(), RepoError>;
    async fn hold_by_ref(&self, conn: &mut PgConnection, gw: Uuid, hold_ref: &str) -> Result<Option<Hold>, RepoError>;
    async fn issue_release(&self, tx: &mut Tx, hold: Uuid, decision_id: Uuid, received_at: DateTime<Utc>) -> Result<bool, RepoError>;   // WHERE release_decision_id IS NULL
}
pub trait HoldRepository: Send + Sync {
    async fn get_for_update(&self, tx: &mut Tx, id: Uuid) -> Result<Option<Hold>, RepoError>;
    async fn page(&self, conn: &mut PgConnection, f: &HoldFilter, page: Keyset) -> Result<Page<Hold>, RepoError>;   // no total
    async fn claim(&self, tx: &mut Tx, id: Uuid, by: Uuid, expected: i32) -> Result<Hold, RepoError>;
    async fn set_proposal(&self, tx: &mut Tx, id: Uuid, expected: i32, p: &Proposal) -> Result<Hold, RepoError>;
    async fn apply_outcome(&self, tx: &mut Tx, id: Uuid, expected: i32, to: HoldState, by: Uuid, at: DateTime<Utc>) -> Result<Hold, RepoError>;
    async fn set_alert(&self, tx: &mut Tx, id: Uuid, alert: Uuid) -> Result<(), RepoError>;
    async fn expire_due(&self, tx: &mut Tx, now: DateTime<Utc>, limit: i64) -> Result<Vec<Uuid>, RepoError>;
    async fn unlinked_list_holds(&self, conn: &mut PgConnection, limit: i64) -> Result<Vec<Uuid>, RepoError>;
}
pub trait PolicyRepository: Send + Sync {   // akr-assess reads; akr-api writes through the change target
    async fn current_matrix(&self, conn: &mut PgConnection) -> Result<Vec<FailPolicy>, RepoError>;
    async fn current_bands(&self, conn: &mut PgConnection) -> Result<Vec<AmountBand>, RepoError>;
    async fn current_modes(&self, conn: &mut PgConnection) -> Result<Vec<RailMode>, RepoError>;
    async fn active_rules(&self, conn: &mut PgConnection) -> Result<Vec<ProhibitedRule>, RepoError>;
    async fn set_version(&self, conn: &mut PgConnection) -> Result<i32, RepoError>;
    async fn bump_set_version(&self, tx: &mut Tx, now: DateTime<Utc>) -> Result<i32, RepoError>;
}
pub trait ReleaseRepository: Send + Sync {
    async fn canon_releases(&self, conn: &mut PgConnection, from: DateTime<Utc>, to: DateTime<Utc>, limit: i64) -> Result<Vec<CanonRelease>, RepoError>;
    async fn decisions_for_keys(&self, conn: &mut PgConnection, keys: &[String], from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<DecisionKeyRow>, RepoError>;
    async fn record_exception(&self, tx: &mut Tx, e: &NewReleaseException) -> Result<bool, RepoError>;   // ON CONFLICT DO NOTHING on (payment_message_id, payment_received_at)
    async fn close_for_register(&self, tx: &mut Tx, register_id: Uuid, closed_by: Uuid, at: DateTime<Utc>) -> Result<Vec<Uuid>, RepoError>;   // closes the open exceptions of that register row (closed_at, closed_by, close_reason 'explained'); returns their ids
}
```

- **Repository implementations** (`infrastructure/pg/assess`): `PgAssessDecisionRepository`, `PgHoldRepository`, `PgPolicyRepository`, `PgReleaseRepository`, plus `PgNodeRepository`, `PgOutageRepository`, `PgRollupRepository`, `PgControlTestRepository`. Every write names its columns (no `SELECT *`, no `INSERT` without a column list). `canon_releases` reads `canon.payment_message` with `direction = 'out'` and a `received_at` range on the partition key, through `akr_app` only (akr-worker); `akr_assess` never reads canon. `decisions_for_keys` uses `idx_assess_decisions_match`.
- **utoipa schemas**: `AssessPaymentRequest { format, raw, canonical, context }`; `SignedDecision { decision_id, decision, mode, reasons, rule_id, hold_ref, degraded, expires_at, signature, customer_message_code, fail_policy, policy_version }`; `ReasonView { code, list, field, rule_id }`; `HoldStatusView { hold_ref, status, decision: Option<SignedDecision> }`; `GatewayHealth`; `AssessHealth { nodes, rails, lists, degraded, blind_periods_open, release_exceptions_open }`; `FailPolicyMatrix { set_version, rows, bands }`; `PolicySetProposal`; `ModeProposal`; `ModeReadiness { rail, current, next, criteria: Vec<Criterion { key, required, observed, passed }>, basis_days }`; `HoldView { id, hold_ref, rail, mode, kind, state, reason_codes, candidates, proposal, expires_at, party_names }`; `HoldDecisionRequest { outcome, reason_codes, text }`; `DecisionView { id, received_at, gateway_key, rail, decision, mode, reason_codes, rule_id, degraded, matched }`; `ReleaseExceptionView`; `BlindPeriodView`; `ControlTestRecord`; `RuleView`.
- **Route table** (`| Method | Path | Scope | Description (cedar action) |`; the people routes are on `akr-api` and their paths are relative to `/api/v1` (API.md section 1); each mutation requires `Idempotency-Key` (section 3.3); each versioned resource requires `If-Match`). The 12 rows of API.md section 3.19 are owned here; `readiness` and `blind-periods` are added (DR-315).

| Method | Path | Scope | Description (cedar action) |
|-|-|-|-|
| GET | /api/v1/assess/fail-policies | config.author, config.approve, dashboard.read | Current fail policy matrix and amount bands (`assess.fail_policy.read`) |
| PUT | /api/v1/assess/fail-policies | config.author | Propose a new policy set; **CR** `cr_material` (object `assess_policy`; `assess.fail_policy.propose`) |
| GET | /api/v1/assess/modes | config.author, config.approve, dashboard.read | Mode per rail with its effective-from instant and readiness snapshot (`assess.mode.read`) |
| PUT | /api/v1/assess/modes/{rail} | config.author | Propose a mode change; **CR** `cr_material` (object `assess_mode`; `assess.mode.propose`) |
| GET | /api/v1/assess/modes/{rail}/readiness | config.author, config.approve | Entry criteria for the next mode, computed from the roll-ups (`assess.mode.readiness`; added) |
| GET | /api/v1/assess/holds | screening.work | Hold queue: held, advisory and shadow review rows (`assess.hold.list`) |
| GET | /api/v1/assess/holds/{id} | screening.work | One hold with its candidates, names for SS and M, and the proposal (`assess.hold.read`) |
| POST | /api/v1/assess/holds/{id}/claim | screening.work | Claim; `If-Match` (`assess.hold.claim`) |
| POST | /api/v1/assess/holds/{id}/decision | screening.work | Proposal, then approval by a second person; **SU** class `decision`, **4E** (`assess.hold.decide`) |
| GET | /api/v1/assess/decisions | audit.read | Search of signed decisions; purpose required; party-linked rows masked (`assess.decision.search`) |
| GET | /api/v1/assess/release-without-decision | dashboard.read | Open release exceptions (`assess.release_exception.list`) |
| GET | /api/v1/assess/blind-periods | dashboard.read | Outage periods with duration (`assess.blind_period.list`; added) |
| GET | /api/v1/assess/health | admin.system | Nodes, mode per rail, list freshness, degraded counters (`assess.health.read`) |
| POST | /api/v1/assess/control-tests | admin.system | Record the weekly control-test result (`assess.control_test.record`) |
| POST | /assess/v1/payments | assess.call | Assess one payment; signed decision (`assess.payment.assess`) |
| GET | /assess/v1/decisions/{id} | assess.call | Replay this gateway's signed decision, byte for byte (`assess.decision.read_own`) |
| GET | /assess/v1/holds/{hold_ref} | assess.call | Hold status; a released hold carries its new signed pass (`assess.hold.status`) |
| GET | /assess/v1/health | assess.call | Mode per rail, list freshness and degraded flags for this gateway (`assess.health.gateway`) |

- **Routers**: `api::routes::assess::router() -> Router<AppState>` (mounts `config` and `holds` sub-routers, every path behind the `pack.assess` flag gate); `assess::routes::gateway::router() -> Router<AppState>` on `akr-assess`, behind `GatewayAuthenticator` and the rate limiter. The gateway router serves only the four rows above and `/livez`, `/readyz` and `/metrics` on a separate listener (no browser route, DR-293).

---

## 3. Business Logic

Service structs and their main methods (signatures are shown in the form the other phases use; the trait bounds are `Send + Sync`). Every step below is part of one request or one job; a job re-reads its owner before it acts.

- **`AssessmentService`** (akr-assess; the decision path): `async fn assess(&self, id: &GatewayIdentity, req: AssessPaymentRequest, idem: &str, now: DateTime<Utc>) -> Result<AssessResponse, AssessError>`, where `AssessResponse` is `Replayed(Vec<u8>)` or `Fresh(Vec<u8>)`.
- **`MessageParser`** (pure, `assess::parse`): `fn parse(format: PayloadFormat, body: &RequestBody) -> Result<ParsedPayment, ParseError>` (section 3.2).
- **`PolicyCache`** (akr-assess; reader of the policy tables): `fn snapshot(&self) -> Arc<PolicySnapshot>`, `async fn refresh(&self, conn: &mut PgConnection) -> Result<(), RepoError>`, `fn fail_policy(&self, rail: &str, corridor: &str, currency: &str, band: &str) -> FailPolicyHit`.
- **`DecisionSigner`** (akr-assess): `fn sign(&self, canonical: &[u8]) -> Result<(i32, [u8; 64]), SignError>`; `fn verify_key(&self, version: i32) -> Option<[u8; 32]>`; `async fn load(crypto: &dyn Crypto, conn: &mut PgConnection) -> Result<Self, SignError>` (section 3.4).
- **`GatewayAuthenticator`** and **`GatewayRateLimiter`** (akr-assess; section 3.10).
- **`HoldService`** (akr-api; the reviewer's act): `claim`, `decide`, `view`, `list` (section 3.5).
- **`HoldSweepService`** (akr-worker; job `assess_hold_sweep`): `run(&self, now)` (section 3.5).
- **`FailPolicyService`**, **`RailModeService`**, **`ProhibitedRuleService`** (akr-api; the proposals and the three change targets, section 3.11).
- **`ReleaseControlService`** (akr-worker; section 3.14), **`ControlTestService`** (akr-api; section 3.14).
- **`OutageService`** (akr-worker; section 3.13), **`FallbackExtractService`** (akr-worker; section 3.13), **`RollupService`** (akr-worker; section 3.12), **`LedgerSweepService`** (akr-worker; section 3.3).
- **`AssessReadService`** (akr-api; decision search, release exceptions, blind periods, health, readiness; sections 3.15 and 3.18).

### 3.1 The decision path (`AssessmentService::assess`; API 4.2; C-55, C-56, C-57, C-65, N-49, N-50, N-51)

The steps run in this order; a refusal at any step is answered with the code of section 3.16 and writes nothing except the ledger cleanup of step 5.

1. **Transport** (before the handler). TLS 1.3 on the gateway listener with a client certificate required, verified against the bank CA bundle (a file on the node, not a table). A handshake without a certificate, or with a chain outside the bank CA, is closed by the TLS layer and gets no HTTP response.
2. **Identity** (`GatewayAuthenticator::identify`, section 3.10): the SHA-256 of the leaf certificate (DER) is looked up in the gateway cache. Unknown, `revoked`, past `not_after`, or linked to a user row that is not `active`: 401 `E_UNAUTHENTICATED`. Then Cedar `assess.payment.assess` with `Ctx { via: "gateway_cert", network: "other", purpose: "", aal: "aal1", stepup_age_s: 2147483647 }` and the gateway principal; a Cedar deny is 403 `E_FORBIDDEN`. Budget 1 ms.
3. **Rate limit** (`GatewayRateLimiter`, Valkey token bucket keyed by `gateway_id`, `assess.rate_per_second`, `assess.rate_burst`; one bucket per gateway, shared by the nodes, section 3.10): an empty bucket is 429 `E_RATE_LIMITED` with `Retry-After`. Budget 1 ms.
4. **Gate and shape**. The flag `pack.assess` is read from the flag cache (30 seconds); off means 404 `E_NOT_FOUND`. The body is at most 64 KB (413 `E_PAYLOAD_TOO_LARGE`). `Idempotency-Key` must match the phase 00 key rule (8 to 255 printable ASCII) or 400 `E_IDEMPOTENCY_REQUIRED`. The body is parsed with unknown fields refused (400 `E_BAD_REQUEST`). The rail must be in `assess.rails` with `active = TRUE` and have an open `assess.rail_modes` row; otherwise 422 `E_VALIDATION` with `errors[].code = rail_not_configured`.
5. **Replay ledger** (section 3.3). `request_sha256` is the SHA-256 of the canonical request (format, body and context). `decision_id` is a new UUIDv7 and `received_at` is the current UTC instant. The ledger answers `Created`, `Replay`, `InFlight` or `Reused`, and only `Created` continues.
6. **Parse** (section 3.2). A parse failure deletes the in-flight ledger row and answers 422 `E_VALIDATION` with `errors[].code = parse_failed` and the field path. No decision is written (DR-297). Budget 5 ms.
7. **Policy and mode** (`PolicyCache`, sections 3.6 and 3.7). The mode of the rail (`shadow`, `advisory` or `enforcing`), the global `set_version` (stored as `policy_version`), the most specific fail policy row for the rail, corridor, currency and amount band (exact before `*`, corridor first), and its `on_failure` (stored as `fail_policy`). The amount band comes from `assess.amount_bands` for the instructed currency and amount. Budget 1 ms.
8. **Mandatory data.** For `mt103` and `pacs008` the value date, currency, amount, and a counterparty name or BIC must be present; a missing item adds `MISSING_DATA` (a hold, S-26 "missing mandatory data"). Budget under 0.1 ms.
9. **Inline screen** (N-49, N-50, N-51). For every name form of every party (ordering customer, beneficiary and their ultimate parties; banks contribute their names only), `ScreeningIndexRegistry` of phase 14 gives the candidates of the active index of every enabled list (`ref.lists.enabled`), and `Scorer` scores them with the inline gates of phase 14 section 3.4 (the thresholds of `ref.lists.thresholds`). A party without a name adds degraded `party_unnamed` and no hit. The screen stops at `assess.inline_timeout_ms` (250): an unfinished screen adds degraded `screening_partial` and the reason `SCREEN_PARTIAL`. Budget: p50 5 ms, p99 25 ms, p95 15 ms (N-50).
10. **Candidate outcome** (phase 14 section 3.4 item 9). `Hit` gives reason `LIST_MATCH`; `NeedsMoreData` gives `LIST_REVIEW`; `BelowLine` is counted in metrics only; `Clear` gives nothing. A freeze-list hit or review holds. A PEP or internal hit or review holds only when its list kind is in `assess.hold_list_kinds` (default `["freeze"]`, which must contain `freeze`); otherwise it adds reason `LIST_NOTED` to a pass and no hold (research 25 15 asks pre-release screening for sanctions only). There is no score in the decision and nothing adds to a reject from a score (C-65).
11. **Prohibited rules** (section 3.9): the active rules in `rule_id` order; the first match gives decision `reject` with `rule_id`. Budget 1 ms.
12. **Freshness** (section 3.6). For each enabled list the phase 14 rule `ListRecord::stale` decides (phase 14 section 3.2: `now - active.activated_at` above `min(refresh_hours, screening.list_max_age_h)`, never above 12 hours for a freeze list); this phase adds no grace. A stale list adds degraded `lists_stale` on every decision; on a cross-border decision (corridor other than `assess.domestic_country`, or corridor unknown) it adds reason `LISTS_STALE`, a hold. Domestic decisions are not held for staleness (research 25 15.5 item 3 and API 4.2; Open doubts 4 records the stricter C-57 wording).
13. **Decision** by precedence: `reject` (step 11), else `hold` (`LIST_MATCH` or `LIST_REVIEW` of a holding list kind, `MISSING_DATA`, `SCREEN_PARTIAL` or cross-border `LISTS_STALE`), else `pass`. The computed result is the **would-be** decision.
14. **Mode** (section 3.7). `shadow`: the answer is `pass` with `mode: shadow`, the would-be decision is stored, and a shadow review hold is created for a would-be hold (no gateway effect, no alert). `advisory`: the answer is the computed decision with `mode: advisory`; holds are created in state `advisory_open` and the gateway does not wait. `enforcing`: the answer is the computed decision; holds are created in state `held`. A `reject` is answered as a reject in advisory and enforcing and as a pass in shadow.
15. **Sign and expire** (section 3.4). `expires_at` is now plus `assess.payment_expiry_s` (600). The canonical response is signed with the active decision key. A failed signature (`SignError`) is answered as a failure of step 16: no decision row is written, the in-flight ledger row is deleted, and the answer is 503 `E_UNAVAILABLE` with no pass issued. A hold's `hold_ref` (22 base64url characters from 16 random bytes) is in the response.
16. **Persist before answer** (the rule of API 4.2: no `pass` without a row). One transaction: insert the `assess.decisions` row with `response_bytes` and `signature`; insert `assess.holds` and `assess.hold_candidates` for a hold (the decision row first, because of the composite foreign key); update the ledger row to `done`; commit. A missing partition (SQLSTATE 23514) is handled once by calling `assess.create_partition` and retrying; any other failure, or a second failure, deletes the in-flight ledger row and answers 503 `E_UNAVAILABLE`. No `pass` leaves the node without this commit. Budget 10 ms p95 (synchronous commit).
17. **Answer** with the stored bytes (the bytes committed are the bytes sent), status 200. Metrics: `akr_assess_step_seconds{step}` for steps 2 to 16, `akr_assess_decisions_total{rail,mode,outcome}`, `akr_assess_degraded_total{flag}`, `akr_assess_persist_failures_total`.

**Latency budget** (research 25 15.4 and C-56; service side; to be proven by the load test of section 6):

| Step | Budget (p95) | Note |
|-|-|-|
| 1 to 2 transport, identity, Cedar | 3 ms | certificate pin in cache; no database read on the path |
| 3 rate limit | 1 ms | Valkey, one round trip |
| 4 gate and shape | 1 ms | flags and rail cache |
| 5 ledger insert | 5 ms | one primary key insert |
| 6 parse | 5 ms | pure; research 25 15.4 gives 2 to 10 ms |
| 7 policy and mode | 1 ms | snapshot |
| 9 inline screen | 15 ms | N-50: p50 5 ms, p99 25 ms |
| 11 to 15 rules, freshness, decision, signing | 2 ms | Ed25519 signing is tens of microseconds |
| 16 persist | 10 ms | synchronous commit of one row and holds |
| **Total service side** | **about 45 ms** | target p95 150 ms and p99 400 ms (C-56), proven in section 6 |

### 3.2 Message parsing (`MessageParser`; new in phase 29, DR-297)

No parser exists in an earlier phase: phase 10 and phase 11 defer ISO 20022 and MT parsing (`phase-10.md` line 2375, `phase-11.md` line 1508), and the canonical payment copy of phase 10 is produced from the gateway database, not from messages. This phase adds a pure parser in `assess::parse`.

- **`mt103`**: the SWIFT MT103 subset of the block 4 fields `20`, `32A`, `33B`, `50A` or `50K`, `52A` or `52D`, `53A`, `56A`, `57A`, `59` or `59A`, `70`, `71A`, `121` (the UETR, when present). Other fields are ignored and not stored.
- **`pacs008`**: the ISO 20022 `pacs.008` subset `GrpHdr/MsgId`, `GrpHdr/CreDtTm`, `CdtTrfTxInf/PmtId/UETR`, `IntrBkSttlmAmt` (with `Ccy`), `IntrBkSttlmDt`, `Dbtr`, `DbtrAgt`, `Cdtr`, `CdtrAgt`, `UltmtDbtr`, `UltmtCdtr` and `RmtInf`. The namespace is matched by local name; a DTD or an entity declaration is refused (422 `parse_failed`, `dtd_refused`); the XML parser expands no entities; the size cap is 64 KB.
- **`canonical`**: a JSON object `{ "uetr"?, "internal_ref", "amount", "currency", "value_date", "counterparty_country", "parties": [{ "role", "names": [...], "bic"? }] }`.
- **`rtgs` and `ach`**: accepted in `canonical` form only in this version (DR-297); the bank's own raw formats for these rails are Open doubts 6, because no sample message exists in the plan.
- **Output** `ParsedPayment { uetr: Option<Uuid>, internal_ref: Option<String>, currency, amount: Decimal, value_date, corridor: Option<String> (ISO 3166-1 alpha-2 of the beneficiary side), parties: Vec<PartyNames { role, name_forms: Vec<String> }> }`. A party with no name form is kept with an empty list; it adds degraded `party_unnamed` (research 25 15.1: the API screens what the message carries and flags what it lacks).
- **Errors**: `parse_failed` (syntax, field path), `dtd_refused`, `unsupported_for_rail`, `too_large`. All map to 422 `E_VALIDATION` with `errors[]`.
- **Match key** (for the release control, section 3.14): `uetr:<lowercase UUID>` when a UETR is present, otherwise `ref:<SHA-256 hex of internal_ref>` when the canonical or message reference exists, otherwise none.

### 3.3 Replay ledger and byte-exact replay (`AssessDecisionRepository`; API 4.2 Idempotency; DR-295)

- **Why a separate ledger**: `admin.idempotency_keys` (phase 00) is granted to `akr_app` only, stores its response bodies sealed under the `idempotency` key family, and `akr_assess` holds no such key. The decision row carries the response bytes instead, so a replay returns the same bytes that were signed. The ledger is `assess.idempotency`, keyed by `(gateway_id, idem_key)`, so two gateways may use the same key.
- **Begin** (step 5 of section 3.1, one transaction): `INSERT INTO assess.idempotency (gateway_id, idem_key, request_sha256, decision_id, decision_received_at, state, lease_until) VALUES (..., 'in_flight', now() + interval '30 seconds') ON CONFLICT (gateway_id, idem_key) DO NOTHING`. If the insert succeeds the result is `Created`. Otherwise the row is read `FOR UPDATE` and:
  - `request_sha256` differs: `Reused`, 422 `E_IDEMPOTENCY_REUSE`.
  - `state = 'done'`: `Replay`. The stored decision row is read by `(decision_id, decision_received_at)` and its `response_bytes` are returned as they are, with header `x-idempotent-replay: true`. The rate limit (step 3) still applies; the list, the mode and the policy do not re-run, because the stored answer stands.
  - `state = 'in_flight'` and `lease_until > now()`: `InFlight`, 409 `E_IDEMPOTENCY_IN_FLIGHT`.
  - `state = 'in_flight'` and `lease_until <= now()`: take over (`UPDATE ... SET lease_until = now() + interval '30 seconds' WHERE state = 'in_flight' AND lease_until <= now()`) and reuse the stored `decision_id`. If a decision row with that id already exists (the node crashed after its commit), its bytes are returned and the ledger row becomes `done`; otherwise the request continues.
- **Cleanup**: a refusal in steps 4, 6 or 16 deletes the in-flight row, so a corrected retry with the same key is accepted. Error answers are never stored (API section 1: only successful responses are stored).
- **Expiry of a replayed pass**: a replay carries the original `expires_at`. After it, the gateway needs a new call with a new key (the gateway's key suffix is the attempt number).
- **Sweep** (`LedgerSweepService`, job `assess_ledger_sweep`, hourly): deletes `done` rows older than `assess.idempotency_hours` (default 48) and `in_flight` rows older than one hour. No decision row is touched.
- **No foreign key** from the ledger to the decision: the row exists before the decision is written.
- **Refused replays of expired keys** are not special: after the window the key is new, so the decision ledger's `GET /assess/v1/decisions/{id}` (section 2) is the reconciliation path for the gateway.

### 3.4 Canonical decision bytes and signing (`DecisionSigner`; API 4.2 Signature; DR-294)

- **Canonical form**: UTF-8 JSON, object keys sorted by Unicode code point, no insignificant whitespace, integers only (no floats), strings escaped as RFC 8259, timestamps as RFC 3339 UTC with `Z` and whole seconds. The signed object is `{"customer_message_code", "decision", "decision_id", "degraded", "expires_at", "fail_policy", "gateway_key", "key_version", "mode", "policy_version", "reasons", "request_sha256"}` plus `"hold_ref"` when present and `"rule_id"` when present. `reasons` entries are `{"code", "list"?, "field"?, "rule_id"?}`; no score exists.
- **Signature**: Ed25519 over the canonical bytes, with the in-memory key of the active version. The response field is `"signature": "<key_version>||<base64 signature>"`; the stored `response_bytes` are the full JSON, keys sorted, with that field. Signing is in process: OpenBao is never called per request (API 4.2).
- **Key lifecycle** (`akr-ctl assess keys generate --purpose decision`, run by an operator with `admin.users`): a new Ed25519 seed is generated in the process; the seed is wrapped with OpenBao transit `akr-kek` (`transit/encrypt`); a row `assess.signing_keys` is inserted with `status = 'active'`, the public key, its SHA-256 fingerprint and the wrapped seed; in the same transaction the previous active row becomes `verify_only` with `wrapped_seed` NULL and `retired_at` set (this is an `UPDATE (status, wrapped_seed, retired_at)` by `akr_app`, section 1.3). The fingerprint is printed for a second channel (the phase 27 pattern), and the public key is written to KV `keys/assess-decision/v<N>.pub`.
- **Start-up**: each node unwraps the active seed with `transit/decrypt` on `akr-kek` once, holds it in memory, and zeroises the wrapped copy it read. A restart needs OpenBao unsealed (auto-unseal, ARCHITECTURE section 21).
- **Rotation**: yearly (checked by `assess_rollup_watch`, section 3.12) and at once on compromise. Passes signed with a retired key stay verifiable with its published public key until their `expires_at` plus one day.
- **The phase 00 transit key `akr-sign-decision`** is listed in phase 00 (`phase-00.md` line 588, "used from phases 02, 06, 12 and 29"). This phase does not sign with it: a transit signature would sit in the request path, which API 4.2 forbids. The name stays reserved; Open doubts 2 asks the lead to amend phase 00.
- **Reference verifier** `akr-ctl assess verify-decision --pub FILE --response FILE [--now RFC3339] [--require-enforced]`: splits the signature, checks the key version, rebuilds the canonical bytes, verifies, refuses `expires_at <= now` (`expired`), and with `--require-enforced` refuses a `pass` whose `mode` is `shadow`. The curl suite uses it for the expired-pass case.

### 3.5 Holds: creation, review, sweep and the alert (`HoldService`, `HoldSweepService`; API 3.19 holds rows; C-58, C-60)

**Creation** (section 3.1 step 16, same transaction as the decision):

| Decision mode | Computed decision | Hold row | Kind | State |
|-|-|-|-|-|
| shadow | hold | yes | review, or list_match when a list candidate exists | `shadow_review` (no alert) |
| advisory | hold | yes | list_match or review | `advisory_open` (gateway does not wait) |
| enforcing | hold | yes | list_match or review | `held` (gateway holds) |
| any | pass | no | none | none |
| any | reject (rule) | no | none | none (the rule is final; `rule_id` recorded) |

- The subject of a hold is `subject_ref` = UUIDv5 over `gateway_id` and the match key (or the internal reference) in a namespace constant of the code. `party_names_enc` is the sealed name forms (personal envelope, phase 02); candidates are in `assess.hold_candidates` with sealed explanations.
- `expires_at` = creation plus `assess.hold_max_hours` (24).
- The hold's **single alert** is the highest-scoring candidate (ties by `entry_key`); the other candidates stay on the hold and do not create alerts.

**Review** (`POST /api/v1/assess/holds/{id}/decision`; scope `screening.work`; SU class `decision`; 4E; Cedar `assess.hold.decide`):

1. `StepUpService::consume(tx, ctx, header, StepUpAction { name: "assess.hold.decide", class: decision }, "assess_hold:<id>")` (phase 01); 401 `E_STEPUP_REQUIRED` otherwise.
2. `HoldRepository::get_for_update`; `If-Match` must equal `version` (428 `E_PRECONDITION_REQUIRED` when absent, 409 `E_VERSION_CONFLICT` otherwise).
3. State must be `held`, `advisory_open` or `shadow_review`; otherwise 409 `E_NOT_ALLOWED_IN_STATE`.
4. **First decision (proposal)** when `proposal` is NULL: the outcome `release` or `reject`; `reason_codes` validated against the phase 09 family `decision` (active codes); `text` at most 500 characters, sealed. Stores `proposal`, `proposal_reason_codes`, `proposed_by`, `proposed_at`; `version + 1`. Audit `assess.hold_proposed`.
5. **Second decision (approval)** when a proposal exists: the caller must differ from `proposed_by` (403 `E_SELF_APPROVAL`); the outcome must equal the proposal (409 `E_NOT_ALLOWED_IN_STATE`, sub-code `proposal_mismatch`). Then:
   - `advisory_open` gives `advisory_closed`; `shadow_review` gives `shadow_closed`. Both record the outcome for the roll-ups (section 3.12). For a list_match hold the phase 14 alert is decided by its own rules (step 6).
   - `held` review hold: `release` gives `released` (`resolved_at`), `reject` gives `rejected` (`resolved_at`).
   - `held` list_match hold: the alert (set by the sweep; absent gives 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `alert_pending`) is decided by phase 14 `AlertService::decide` with the approver as decider: `release` gives outcome `incorrect` (reason codes required by phase 14); `reject` gives outcome `potential`. The hold's state follows the alert (step 7). The `true` decision of a freeze list is a later act by a different person in phase 14 and starts the freeze (section 3.8).
6. Audit `assess.hold_decided` (`hold_id`, `outcome`, `state_to`; no names, no text). Every decision of a hold writes one access event for the names it shows (phase 02 `view`).
7. **Sync** (also run by the sweep): when the alert of a `held` list_match hold becomes `incorrect`, the hold becomes `released` and `resolved_at` is set; when it becomes `true`, the hold becomes `rejected` (section 3.8). `decided_by` and `decided_at` take the `decider_id` and the time of the alert's latest decision in `alert.screening_decisions` (phase 14), so a sync names the person who moved the alert. Each sync transition writes `assess.hold_decided` (`outcome` `release` or `reject`, `state_to`) in its own transaction. The sweep applies the same transition, so a decision made in the phase 14 queue alone also moves the hold.

**Sweep** (`HoldSweepService::run`, job `assess_hold_sweep`, every minute, batch 200; one transaction per hold):

1. **Expiry**: holds in `held` past `expires_at` become `expired` (`resolved_at`); `advisory_open` past it become `advisory_closed`; `shadow_review` past it become `shadow_closed`. Audit `assess.hold_expired`. An expired `held` hold is never released by a later pass (DR-312).
2. **Alerts for list holds**: for each `kind = 'list_match'` hold in mode `advisory` or `enforcing` with no alert, insert one `alert.screening_runs` row of kind `payment` (a system run: `requested_by` NULL; `subject_kind = 'payment_party'`, `subject_id = subject_ref`, `trigger_ref` = the hold id, status `completed`, `finished_at` now), then build the alert from the highest-scoring `hold_candidates` row and insert it through phase 14 `AlertRepository::insert_candidates` (the fingerprint, priority from `screening.priority_map` and the alert clock of phase 14 section 3.5 apply unchanged). This is the new method `ScreeningService::open_hold_alert` (section 4.5). Then `HoldRepository::set_alert`, in the same transaction. The link needs no kind of its own: the alert insert writes phase 14's `screening.alert_created`, and the run's `trigger_ref` is the hold id.
3. **Sync** (section 3.5 step 7).
4. **Freeze-list version check**: if the set of active freeze-list version ids differs from the `list_version_ids` of the newest row of `assess.fallback_extracts` (section 1.2), enqueue `assess_fallback_extract`.

**Queue and view**: `GET /api/v1/assess/holds` returns the held, advisory and shadow review rows (cursor, no total), sorted by `expires_at`. A list_match hold shows its alert's state and the candidate list codes; SS and M see the names as phase 14 shows them (phase 14 section 3.15 masking applies to the alert's subject). Responses are `Cache-Control: no-store`.

**Release pass** (`GET /assess/v1/holds/{hold_ref}`, akr-assess; DR-299). A hold in state `released` with no `release_decision_id` gets its pass on the first fetch. One transaction (lock order 4, section 3.20) inserts a decision of kind `hold_release` with decision `pass`, the hold's mode, `hold_id` and `expires_at` = now plus `assess.payment_expiry_s`, signed and persisted by the rule of section 3.1 step 16; then `issue_release` sets `release_decision_id` and `release_received_at` (once only, by the guard of section 1.3). A concurrent second fetch finds the id set and returns the stored bytes. The response is `{ "hold_ref", "status": "released", "decision": <the signed pass> }`, and the audit event is `assess.pass_issued_on_release`. A `held`, `rejected`, `expired` or closed hold returns its status with no decision. A `released` hold whose pass cannot be persisted answers 503 `E_UNAVAILABLE`, and the gateway fetches again.

### 3.6 Fail policies, amount bands and list freshness (`FailPolicyService`, `PolicyCache`; C-57)

- **Matrix** (`assess.fail_policies`): one row per (rail, corridor, currency, amount band) with `'*'` as the wildcard. The current set is the rows with `superseded_at IS NULL` of one `set_version`. A proposal carries the whole matrix and the bands; an approved activation (section 3.11) inserts the new rows with `set_version` = `assess.policy_state.set_version` + 1, the global counter after this activation's increment (section 3.11), and supersedes every current row in the same transaction. The set versions of the matrix and of the bands may therefore skip the numbers that a mode or rule activation took.
- **Validation at proposal** (`prepare`): every rail exists; corridor is ISO 3166-1 alpha-2 or `*`; currency ISO 4217 or `*`; each currency's bands cover from 0 to unbounded without a gap (`bands_gap`) or an overlap (`bands_overlap`); each rail has a row for `*`, `*`, `*` (otherwise 422 `E_VALIDATION` with `errors[].code = wildcard_row_missing`); a proposal that changes no row is 422 `E_VALIDATION` with `errors[].code = no_change` (phase 28 lesson).
- **Lookup** (step 7 of section 3.1): the row with the fewest wildcards, compared in the order corridor, currency, band. Its `on_failure` is stored on the decision as `fail_policy` and returned to the gateway (API 4.2 addition, Open doubts 13). The gateway enforces its own copy: it is exported with `akr-ctl assess policy export` as a document signed with the `extract` key (section 3.13). Only the gateway holds the message, so the gateway, not this product, applies open or closed (API 4.3).
- **Cache** (`PolicyCache`): an `ArcSwap<PolicySnapshot>` with the matrix, bands, modes, rules and `set_version`. It is rebuilt off the request path when `NOTIFY akr_assess_policy` arrives and every 30 seconds by reading `assess.policy_state.set_version`; a new snapshot replaces the old one atomically (the phase 14 pattern, N-49). `assess.node_status.policy_version` shows each node's version; the health route (section 3.15) shows any node that lags.
- **List freshness**: each enabled list has an `active` version (`ref.list_versions`, `loaded_at`) and the screening index of phase 14 records `loaded_at`. Stale is the phase 14 rule `ListRecord::stale` unchanged (phase 14 section 3.2: `age_hours = now - active.activated_at`, stale above `min(refresh_hours, screening.list_max_age_h)`; at most 12 hours for freeze lists, phase 14 CHECK and floor). This phase adds no grace and no setting of its own. The stale state is read from the index registry, not from a query per request.
- **Cross-border** means corridor different from `assess.domestic_country` (`IQ` by default) or corridor unknown.

### 3.7 Rail modes and readiness (`RailModeService`; C-56 stages, research 25 15.9)

- **Stages**: `shadow` (answers pass, records the would-be decision, creates shadow review holds, touches no gateway); `advisory` (answers the computed decision, holds are shown but the gateway does not wait); `enforcing` (the gateway holds and rejects). A rail with no open `rail_modes` row is not assessed (422 on the decision path, section 3.1 step 4). The first mode of a rail is `shadow`.
- **Allowed moves**: shadow to advisory; advisory to enforcing; and rollbacks: enforcing to advisory, advisory to shadow, enforcing to shadow. Rollbacks need no readiness.
- **Entry criteria** (`readiness`, computed from `assess.mode_rollups` and the decision and hold tables; each threshold is a governed setting, section 3.19):

| Move | Criterion | Setting |
|-|-|-|
| shadow to advisory | shadow days since `effective_from` at least the minimum | `assess.shadow_min_days` (30) |
| shadow to advisory | shadow decisions at least the minimum | `assess.shadow_min_decisions` (1,000) |
| shadow to advisory | closed shadow reviews at least the ratio of would-be holds | `assess.shadow_review_min_ratio` (0.9) |
| shadow to advisory | every day of the window has a roll-up (no blind day) | fixed |
| advisory to enforcing | advisory days since `effective_from` at least the minimum | `assess.advisory_min_days` (14) |
| advisory to enforcing | closed advisory holds at least the ratio of advisory holds | `assess.shadow_review_min_ratio` (0.9) |
| advisory to enforcing | no release exception with reason `decision_not_pass` | fixed |
| advisory to enforcing | a control test caught within the maximum gap | `assess.control_test_max_days` (7) |

- **Change**: a proposal (`PUT /api/v1/assess/modes/{rail}`) is a material change request (`cr_material`, section 3.11). `prepare` computes the volume effect: shadow to advisory is `neutral`, advisory to enforcing is `decrease` (fewer releases without a hold), rollbacks are `increase`. `prepare` computes the criteria too and refuses a proposal that does not meet them (422 `E_VALIDATION`, `errors[].code = criterion_not_met` with the failed criterion key), so the PUT is refused as the API.md row of this route says; `check_before_decision` re-computes them at approval with the same refusal. The readiness snapshot is stored on the new row (`readiness` JSON).
- **Gateway effect**: the mode is the bank's gateway setting too; the gateway reads `GET /assess/v1/health` and its own configuration. This product does not push a mode to a gateway.

### 3.8 Sanctions reject and the freeze (phase 14 `FreezeTrigger`, phase 20 `FreezeService`; C-60; DR-303)

- A freeze starts only from a phase 14 alert that a person decides `true` on a freeze list. Phase 14 section 3.6 requires a prior `potential` by one person and the `true` by another; the `true` calls `FreezeTrigger::start` in the same transaction (phase 14 decide step 8), and phase 20's `FreezeService` implements it and creates `casework.freezes` with `screening_alert_id` = that alert.
- This phase adds no call from `akr-assess` to akr-api, no outbox topic and no hand-off job of its own: the alert of a list hold is created by `HoldSweepService` (section 3.5 step 2), in the process where phase 14 and phase 20 already run. The hold follows the alert (section 3.5 step 7): `true` gives hold `rejected`.
- The gateway receives `rejected` on `GET /assess/v1/holds/{hold_ref}`, returns the payment to its own fail process, and never receives a pass (DR-312). Phase 20 then runs the freeze clocks and the verification of the block (C-60).
- A shadow true match has no alert and no freeze. It opens a register finding for the MLRO (source `assess_control`, section 3.12) so that the bank's sanctions process decides; the payment itself is outside this product's control in shadow (Open doubts 5).

### 3.9 Prohibited conditions (`ProhibitedRuleService`; C-65; S-26; DR-301)

- **Only deterministic conditions reject without a person.** A rule is a row of `assess.prohibited_rules` with a closed condition grammar: keys `corridor_in` (non-empty list of ISO 3166-1 alpha-2 codes), `currency_in` (ISO 4217 codes), `rail_in` (rail codes) and `amount_min` (decimal string in the instructed currency), combined with AND. The CHECK of section 1.2 refuses any other key; `domain/assess::rules` evaluates the condition as a pure function: `first_match(active: &[ProhibitedRule], rail: &str, corridor: Option<&str>, currency: &str, amount: Decimal) -> Option<&str>` returns the `rule_id` of the first rule in slice order whose keys all match (the caller passes the active rules in `rule_id` order; a missing corridor matches no `corridor_in`).
- **Proposal**: through phase 04 `POST /change-requests` with object kind `assess_rule` (the generic create of the target, section 3.11; no new route). `prepare` validates the grammar and the `basis` (a CBI order or an internal policy reference, 1 to 500 characters after trimming; otherwise 422 `E_VALIDATION` at `/basis`). `volume_effect` is `decrease` for an activation (fewer releases) and `increase` for a retirement; a rule change is material. A retirement is a rule proposal whose `status` is `retired` (the CHECK of section 1.2); its activation inserts that version with `status = 'retired'` and `retired_at` set, and retires the previous active version (section 3.11).
- **Reject**: the first matching active rule in `rule_id` order gives `reject` with `rule_id` (decision CHECK, section 1.1). A reject is final for the gateway in advisory and enforcing and is a pass in shadow (shadow records the would-be reject).
- **Scores never reject**: no score exists in this phase (no model, DR-302); a hold is the only other outcome.
- **Scenario flag**: the scenario register's `phase2_candidate` (the column of phase 06 section 1; the item S-26 of phase 16) is read by no code of this phase. A scenario becomes a prohibited rule only by its own rule change request, with a basis the MLRO signs (DR-301). This answers the "first list" of S-26 without automatic promotion.
- **Example used by the tests** (placeholder values, not a real order): rule `PROHIBITED_DESTINATION_TEST_01` with `corridor_in: ["ZZ"]`.

### 3.10 Gateway identity, transport and rate limits (`GatewayAuthenticator`, `GatewayRateLimiter`; C-55, SP-93, T-43)

- **Transport.** `akr-assess` serves TLS 1.3 on its gateway listener in zone `z4_gateway`, with a client certificate required and verified by the bank CA (the bank CA bundle is a file in the node's configuration, provisioned at install, changed by restart). The gateway listener serves only the four routes of section 2 and no others; there is no cookie, no CORS, no session and no CSRF surface, and no browser route (DR-293).
- **Identity table.** `assess.gateways` pins one certificate per row: `cert_sha256` (SHA-256 of the leaf DER, unique), the `gateway_key`, the service principal `user_id` (a row of `admin.users` with `kind = 'service'`, `owner_user_id` set to the registering operator and `expires_at` equal to `not_after`, phase 01), `not_before` and `not_after`, `status`. Rows are never deleted and their pin is never changed (guard, section 1.3).
- **Registration and revocation** are `akr-ctl` operations, not routes: `akr-ctl assess gateway register --key K --cert PEM --owner USER --confirm-by USER2` and `akr-ctl assess gateway revoke --gateway ID --reason compromise|rotation|decommissioned --confirm-by USER2`. Each requires two different operators with `admin.users` (the second confirms), writes one audit event (`assess.gateway_registered` or `assess.gateway_revoked`, SIEM true), and does not use a change request, because a compromised gateway must be refused at once (Open doubts 9 asks the bank to confirm the two-person rule). Rotation is a registration of a new row with the same `gateway_key`, then a revocation of the old row with reason `rotation`; the partial unique index allows one active row per key.
- **Identify** (`GatewayAuthenticator::identify(der)`): the SHA-256 of the DER is looked up in `GatewayCache`, which holds the rows and is refreshed every `assess.gateway_cache_s` (30 seconds), on one miss, and on `NOTIFY akr_assess_gateway` sent by the revocation. A revocation is therefore refused within thirty seconds on every node (DR-292).
- **Cedar principal.** The gateway is a Cedar `User` built from its `admin.users` row (kind `service`, status, roles empty) with `Ctx.via = "gateway_cert"` (value added to phase 03's `Ctx.via` list, section 3.21). The baseline forbid of phase 03 refuses an inactive principal.
- **Rate limit** (`GatewayRateLimiter`): a Valkey token bucket keyed `assess:rl:{gateway_id}` with refill `assess.rate_per_second` (50) and capacity `assess.rate_burst` (500), held in the shared Valkey, so the nodes draw on one bucket per gateway and the setting is that gateway's total rate whatever the node count (API 4.2: 50 requests a second per bank). The phase 30 fallback divides it by the replica count (section 7.1). The bank confirms the figure (Open doubts 10).
- **Repeat counter** (SP-93, anomaly): the match key of the payment with the attempt suffix removed from the idempotency key is counted per day in the roll-up (section 3.12); more than `assess.repeat_alert_count` (3) assessments of one payment raise a register warning.

### 3.11 Change targets and activation (`FailPolicyService`, `RailModeService`, `ProhibitedRuleService` as `ChangeTarget`s; phase 04; C-56, C-57)

Three targets are registered in `ChangeTargetRegistry` by akr-api at start-up (phase 04 trait `ChangeTarget`):

| Object kind | Carries | Route that creates the CR | `generic_create` | `inventory_kind` |
|-|-|-|-|-|
| `assess_policy` | the fail policy matrix and the amount bands (one set) | `PUT /api/v1/assess/fail-policies` | false | `parameter_set` |
| `assess_mode` | one rail's mode | `PUT /api/v1/assess/modes/{rail}` | false | `parameter_set` |
| `assess_rule` | one prohibited rule | phase 04 `POST /change-requests` (generic create) | true | `rule` |

- **Common rules**: `emergency_allowed = false` (no emergency change; the bank's lever in an emergency is its gateway fail policy, DR-300); the change class is `material` (`cr_material`, parallel MLRO and CHK; CH too when the volume effect is `decrease`, phase 04 route map); the inventory item of the target must be `approved_for_use` (phase 04 GV-01 gate), so a rail cannot leave shadow before its rule set and parameter set are approved. Owner and validator are different people (phase 04 CHECK).
- **`prepare`** validates the body (sections 3.6, 3.7, 3.9), sets `base_version` to the current `set_version` (policy), the number of `rail_modes` rows of the rail (mode; an integer, as phase 04 `base_version` is, and 0 for a rail with no row yet) or the rule's current version (rule), and sets `volume_effect` (policy: `increase` when a closed row becomes open, `decrease` when an open row becomes closed, `neutral` when only bands move; mode: section 3.7; rule: `decrease` on activation and `increase` on retirement). `unknown` is never approved as material (phase 04 rule), so a proposal whose effect cannot be computed is refused.
- **`check_before_decision`**: the base version is unchanged (409 `E_STALE_BASE` otherwise); for a mode move the entry criteria of section 3.7 hold; the basis text is present.
- **`activate`** (one transaction, in this order, section 3.20): lock `assess.policy_state`; policy: supersede every current row and insert the whole proposed matrix with `set_version + 1` and the change request id; bands: insert the whole proposed band set with the same set version (no band row is updated); mode: set `superseded_at` on the open row and insert the new row with the readiness snapshot; rule: insert the new version and retire the previous active version (`status`, `retired_at`); then `policy_state.set_version += 1` and `pg_notify('akr_assess_policy', set_version)`. Audit `assess.policy_activated`, `assess.mode_changed` or `assess.rule_activated` (section 3.17).
- **`revert`** is refused (`E_NOT_ALLOWED_IN_STATE`): a rollback is a new change request. **`on_closed`** does nothing.
- **Guards** (section 1.3) refuse any write of the policy tables that does not name an approved or effective change request of the matching object kind, so a direct SQL write fails (section 6, item 12).

### 3.12 Roll-ups, watch and repeat counter (`RollupService`, job `assess_rollup_watch`; C-56, GV-18 pattern)

Daily at 00:40 for the previous bank day (`day`). Counts only, no name, no amount, no customer id.

1. **Decisions**: group `assess.decisions` of the day by rail and mode and count `decisions`, `would_pass`, `would_hold` and `would_reject` (from `coalesce(would_be, decision)`, so advisory and enforcing days count their decisions), `degraded_total` (sum of degraded flags), `repeat_assessments`. Upsert into `assess.mode_rollups` (`ON CONFLICT (rail, day, mode) DO UPDATE`).
2. **Holds**: count `reviews_closed` (holds moved to `advisory_closed` or `shadow_closed` with the day's `resolved_at`), `reviews_true_match` (alerts of phase 14 decided `true` on those holds), `holds_released`, `holds_rejected` and `holds_expired`, into the same row.
3. **Watch** (warnings in the phase 24 register, source `assess_control`; `source_key` dedupes each condition per day, so a repeated run adds nothing):
   - an active decision key older than 365 days (`assess.signing_keys.created_at`);
   - a gateway certificate with `not_after` within 30 days;
   - the fallback extract older than `assess.fallback_extract_hours` (section 3.13);
   - an open outage period (section 3.13);
   - the last caught control test older than `assess.control_test_max_days` (section 3.14);
   - a repeat count above `assess.repeat_alert_count` for one payment key (step 1).
4. **Shadow truth**: a rail whose day has no roll-up row for a day inside its window is recorded as a blind day in `readiness`, so the entry criteria of section 3.7 fail.

The job runs as the system with no purpose code, because it reads no case and no customer record.

### 3.13 Node heartbeats, outages and the signed list extract (`OutageService`, `FallbackExtractService`; C-58, research 25 15.6; DR-305, DR-306)

- **Heartbeat.** Every akr-assess node upserts its `assess.node_status` row every 30 seconds (`INSERT ... ON CONFLICT (node_id) DO UPDATE`), with `last_seen_at`, `build_version`, `policy_version` and the mode per rail (no counts).
- **Outage watch** (`OutageService::watch`, job `assess_blind_watch`, every minute; it does nothing while `pack.assess` is off or while no rail has an open `assess.rail_modes` row, so an install without phase 2 records no outage):
  1. If no node has `last_seen_at >= now() - assess.heartbeat_max_s` and no open period exists, insert an `assess.blind_periods` row (`started_at` = the newest `last_seen_at` of the nodes, `reason_code = 'no_heartbeat'`, `recorded_by = 'system'`), audit `assess.blind_period_opened` (SIEM true) and one register warning (`source_key = 'assess:blind:' || started_at`).
  2. If an open period exists and a node is alive, set `ended_at = now()` and audit `assess.blind_period_closed` with `duration_s`.
- **Operator-recorded periods**: `akr-ctl assess outage record --from T1 --to T2 --reason operator_recorded --evidence FILE` inserts a closed period for an outage the watch missed (for example a gateway-side outage that the bank's circuit-breaker log shows). The evidence file is an evidence object (phase 02); no route is needed.
- **Visibility**: `GET /api/v1/assess/blind-periods` (section 2) and the G1 health tile (section 3.15). A blind period is never shown as "no decisions".
- **Fallback extract** (`FallbackExtractService::build`, job `assess_fallback_extract`; enqueued by `assess_hold_sweep` when the freeze-list version set changed, and daily at 05:30). The job builds one JSON document per freeze-list version set with, for each freeze entry, the list code, `entry_key`, list version id, the normalised name tokens of phase 14 and its birth-year and document fingerprints, the list's `loaded_at` and `valid_until = built_at + assess.fallback_extract_hours` (at most 12 hours, the CBI refresh rule). The document is stored with `EvidenceStore::put_generated` (phase 02; owner kind `assess_extract`, new, section 4.5; classification personal), signed with the `extract` signing key (Ed25519 over the SHA-256 of the document) and recorded in `assess.fallback_extracts` (section 1.2). Audit `assess.extract_built`.
- **Export to the bank's fallback** (`akr-ctl assess extract export --version N --out DIR`): writes the document and its signature for the bank's own fallback component beside the gateway, and audits `assess.extract_exported` (SIEM true, counts only). The extract is data, not a decision: the matching runs in the bank's fallback component under the bank's approved procedure, and AKR-TMS runs no second matcher outside its zone (DR-305; research 25 15.6 item 2 asks for a compiled index beside the gateway, which this phase replaces with the signed extract).
- **Gateway policy export**: `akr-ctl assess policy export --out FILE` writes the fail policy matrix and the bands, signed with the `extract` key, for the gateway team's configuration (the gateway enforces open or closed, section 3.6).

### 3.14 Release-without-decision control and control tests (`ReleaseControlService`, `ControlTestService`; C-59, C-58, research 25 15.7)

**The control** (job `assess_release_match`, every 15 minutes; reads `canon.payment_message` through `akr_app` in the worker, never through `akr_assess`):

1. **Scope**: canonical payments with `direction = 'out'` and a `status` other than `rejected` or `cancelled` (phase 10 section 7+, DR-389; any other value is controlled, so the control fails closed) on a rail whose open mode is `advisory` or `enforcing`. Rails in shadow are not controlled (no gateway behaviour is expected yet; their would-be decisions appear in the roll-ups).
2. **Window**: `received_at` in `(assess.control_state.release_checkpoint_at, now() - 30 minutes]`, read in keyset order of `(received_at, id)` in chunks of 5,000. The checkpoint advances only to the last instant whose rows were all processed, so no row is skipped; the 30-minute lag lets a decision committed just before the release be seen.
3. **Match key** (section 3.2): `uetr:<uuid>` when `canon.payment_message.uetr` is set; otherwise `ref:<SHA-256 of message_refs->>'internal_ref'>` when that reference exists; otherwise the payment has no match key and is an exception `no_match_key`.
4. **Match**: the decisions with the same match key and `received_at` within `assess.release_window_min` (120 minutes) of the payment's `received_at`, through `idx_assess_decisions_match`. Outcome:
   - no decision: exception `no_decision`;
   - a `pass` decision (any mode), or an `advisory` `hold` (the gateway does not wait for it, so its release is expected), or an enforcing `hold` followed by a `hold_release` pass: matched, but only when the payment's `received_at` is at most the pass's `expires_at` plus `assess.release_skew_s` (60; max 120). A pass counts only while it is valid (API 4.2: a gateway releasing later must call again);
   - only passes that expired before the release: exception `pass_expired` (`decision_id` = the newest such pass);
   - only a `reject`, or an enforcing `hold` with no later `hold_release` pass: exception `decision_not_pass`.
5. **Exceptions**: `INSERT ... ON CONFLICT (payment_message_id, payment_received_at) DO NOTHING` into `assess.release_exceptions`, once per payment. One register warning per run and rail (`source_key = 'assess:release:' || rail || ':' || run_date`, body with counts only), audit `assess.release_exception_raised` (SIEM true, `count` and `reason_code`).
6. **Closure**: an exception closes (`closed_at`, `closed_by`, `close_reason` `explained`) when the register warning of step 5 is closed by the phase 24 register route. That route performs the closure in its own transaction through `ReleaseRepository::close_for_register` (phase 24 section 7.3), with the caller as `closed_by`; the job closes nothing, and this phase adds no route.
7. **Checkpoint**: updated last, in the same transaction as the exception rows of its chunk.

**Control-test payments** (weekly; `ControlTestService::record`, `POST /api/v1/assess/control-tests`, scope `admin.system`, Cedar `assess.control_test.record`): the bank sends a synthetic payment that carries a known test reference through the real gateway to a rail in enforcing mode, and the gateway's answer is recorded: `observed` (`hold`, `pass`, `reject` or `no_response`), `observed_decision_id` (must be a decision of that rail, 422 otherwise), `sent_at`, `test_ref` (`CT-YYYY-MM-DD-XXXX`, unique) and an evidence object. `result` is `caught` exactly when `observed = 'hold'` (table CHECK). Audit `assess.control_test_recorded`. The control-test payment must route to a suspense account in the bank's gateway, a bank-side setting (Open doubts 11). The phase 24 review calendar gets a row of kind `assess_control_test` due `assess.control_test_max_days` after the last caught test per rail (section 4.5).

### 3.15 Health and the added routes (`AssessReadService`; API 3.19 and 4.1; GV-53 availability)

- **Gateway health** `GET /assess/v1/health` (scope `assess.call`, Cedar `assess.health.gateway`): `{ "status": "ok" | "degraded", "rails": [{ "rail", "mode" }], "lists": [{ "code", "state": "fresh" | "stale" }], "degraded_flags_15m": ["lists_stale", ...], "nodes_ok": true | false }`. No customer data and no count per customer; the flags are names only.
- **People health** `GET /api/v1/assess/health` (scope `admin.system`, Cedar `assess.health.read`): per node (`node_id`, seconds since the last heartbeat, `policy_version`, `build_version`); per rail (mode, the readiness status of the next move, last roll-up day); per list (fresh or stale, age in seconds); degraded counters of the last 15 minutes by flag; open blind periods; open release exceptions by rail (counts); the active decision key version and fingerprint; gateways active and certificates expiring within 30 days. No names, no customer ids.
- **Readiness** `GET /api/v1/assess/modes/{rail}/readiness` (added; scope `config.author`, `config.approve`; Cedar `assess.mode.readiness`): the criteria of section 3.7 with their observed values, the thresholds, and `next` (the move the criteria are for). The change request stores the same values as its snapshot.
- **Blind periods** `GET /api/v1/assess/blind-periods` (added; scope `dashboard.read`; Cedar `assess.blind_period.list`): periods with `started_at`, `ended_at`, `duration_s`, `reason_code` and `recorded_by`, newest first, cursor.
- **Small numbers**: any aggregate shown outside the control owners (CFG, CHK, MRO and the dashboard roles) is suppressed below `report.small_number_threshold` (phase 21, governed, minimum 3).
- **G1 tile** (phase 24 command centre): "Assessment" with the modes per rail, list freshness, open blind periods and open release exceptions; visible to M, CH and IA (`dashboard.read`); A1 and A2 never see it.

### 3.16 Errors (`AssessError`, `thiserror`; API section 6 codes only)

| Variant | Code | HTTP | When |
|-|-|-|-|
| `Unauthenticated` | `E_UNAUTHENTICATED` | 401 | unknown, revoked or expired certificate; inactive principal |
| `Forbidden` | `E_FORBIDDEN` | 403 | Cedar deny (gateway or people route) |
| `Scope` | `E_SCOPE` | 403 | a people token without the route's scope |
| `SelfApproval` | `E_SELF_APPROVAL` | 403 | approver is the proposer of a hold |
| `StepUpRequired` | `E_STEPUP_REQUIRED` | 401 | missing, reused or wrong-class assertion on a hold decision |
| `NotFound` | `E_NOT_FOUND` | 404 | pack off; unknown hold; a masked row (identical body) |
| `BadRequest` | `E_BAD_REQUEST` | 400 | malformed JSON or an unknown field |
| `IdempotencyRequired` | `E_IDEMPOTENCY_REQUIRED` | 400 | missing `Idempotency-Key` (people mutations and the gateway payment) |
| `PreconditionRequired` | `E_PRECONDITION_REQUIRED` | 428 | versioned mutation without `If-Match` |
| `VersionConflict` | `E_VERSION_CONFLICT` | 409 | `If-Match` differs; current version in the body |
| `NotAllowedInState` | `E_NOT_ALLOWED_IN_STATE` | 409 | sub-codes `proposal_mismatch`, `alert_pending`, `change_request_required`, `revert_refused` |
| `Claimed` | `E_CLAIMED` | 409 | hold claimed by another user |
| `InFlight` | `E_IDEMPOTENCY_IN_FLIGHT` | 409 | same key still running |
| `StaleBase` | `E_STALE_BASE` | 409 | change request base moved (policy set or mode row) |
| `IdempotencyReuse` | `E_IDEMPOTENCY_REUSE` | 422 | same key, different body |
| `Validation` | `E_VALIDATION` | 422 | `errors[]` codes `parse_failed`, `dtd_refused`, `rail_not_configured`, `no_change`, `bands_gap`, `bands_overlap`, `rule_condition_key`, `corridor_invalid`, `currency_invalid`, `criterion_not_met`, `wildcard_row_missing`, `reason_codes_unknown`, `purpose_required` (sent as 422 `E_PURPOSE_REQUIRED` on search) |
| `PayloadTooLarge` | `E_PAYLOAD_TOO_LARGE` | 413 | body above 64 KB |
| `RateLimited` | `E_RATE_LIMITED` | 429 | empty bucket; `Retry-After` |
| `Unavailable` | `E_UNAVAILABLE` | 503 | decision could not be recorded; no pass is issued |
| `Internal` | `E_INTERNAL` | 500 | correlation id only |

### 3.17 Audit events (one per mutation; payloads hold ids, codes and counts only; DR-296)

Per-payment decisions are **not** audited in `audit.audit_log`: the decision row is the record (DR-296). The hold and hold-candidate rows that step 16 of section 3.1 inserts belong to that record (the hold row names its decision) and write no audit row either; every later change of a hold's state has its kind below, and the alert link of section 3.5 step 2 is covered by phase 14's `screening.alert_created`. Operational writes are not audited: the node heartbeat (section 3.13), the release checkpoint (section 3.14 step 7), the replay ledger and the daily roll-up rows change no business decision. The phase 02 audit stream takes one advisory lock per insert, so one row per payment would serialise every decision of every node on that lock. Each kind is added to the phase 02 catalogue with its `resource_type` and detail keys:

| Kind | Resource type | SIEM | Detail keys |
|-|-|-|-|
| `assess.gateway_registered` | `assess_gateway` | true | `gateway_id`, `gateway_key` |
| `assess.gateway_revoked` | `assess_gateway` | true | `gateway_id`, `reason_code` |
| `assess.key_activated` | `assess_signing_key` | true | `purpose`, `version` |
| `assess.key_retired` | `assess_signing_key` | true | `purpose`, `version` |
| `assess.policy_activated` | `assess_policy` | false | `change_request_id`, `set_version`, `rows` |
| `assess.mode_changed` | `assess_mode` | false | `rail`, `from_mode`, `to_mode`, `change_request_id` |
| `assess.rule_activated` | `assess_rule` | false | `rule_id`, `version`, `change_request_id` |
| `assess.hold_proposed` | `assess_hold` | false | `hold_id`, `proposal` |
| `assess.hold_decided` | `assess_hold` | false | `hold_id`, `outcome`, `state_to` |
| `assess.hold_expired` | `assess_hold` | false | `hold_id` |
| `assess.pass_issued_on_release` | `assess_hold` | false | `hold_id`, `decision_id` |
| `assess.blind_period_opened` | `assess_blind_period` | true | `blind_period_id`, `reason_code` |
| `assess.blind_period_closed` | `assess_blind_period` | false | `blind_period_id`, `duration_s` |
| `assess.release_exception_raised` | `assess_release_exception` | true | `rail`, `reason_code`, `count` |
| `assess.release_exception_closed` | `assess_release_exception` | false | `exception_id`, `close_reason` |
| `assess.control_test_recorded` | `assess_control_test` | false | `control_test_id`, `rail`, `result` |
| `assess.extract_built` | `assess_extract` | false | `extract_version`, `rows` |
| `assess.extract_exported` | `assess_extract` | true | `extract_version`, `rows` |

Reads of personal content (hold names, decision search rows that show a party) write phase 02 `access_event` rows with action `view`, not audit rows.

### 3.18 Tipping-off review (what a person outside the case's access list can learn; the answer must be "nothing")

| Surface | Who can learn what | Verdict |
|-|-|-|
| `POST /assess/v1/payments` response | the gateway learns decision, reason codes (list code and field), `rule_id`, degraded flags and expiry; the customer sees only `customer_message_code` | reasons go to the gateway's operations view only; Open doubts 7 requires that view to stay away from customer channels |
| `GET /assess/v1/holds/{hold_ref}` | the gateway learns `held`, `released` (with its pass), `rejected` or `expired`; no reason | nothing beyond the status |
| Hold queue and hold view (`screening.work`) | SS and M see names and list codes, as phase 14 shows them; the masking of phase 14 section 3.15 applies to the alert's subject | a suspicion-tier subject answers 404 to every other role |
| `GET /api/v1/assess/decisions` (`audit.read`; Cedar IA and M only) | rows show reasons; a row that names a party runs the phase 17 rule for that party (`casework.effective_tier`); a suspicion-tier party answers 404 `E_NOT_FOUND` to IA | a purpose code is required (422 `E_PURPOSE_REQUIRED` otherwise); SEC has the scope but no Cedar permit |
| Release exceptions, blind periods, roll-ups, readiness, health | payment message ids and reason codes; time windows; counts per rail and mode | no name, no amount, no customer id; counts below the threshold suppressed outside the control owners |
| Fallback extract and gateway policy export | names and tokens of freeze entries, for the bank's fallback component only | sealed evidence; export audited with counts |
| SIEM and support bundles (phase 25) | none of the assess tables | phase 25 excludes `assess.decisions`, `assess.holds`, `assess.hold_candidates`, `assess.release_exceptions` by table name (Open doubts 8) |
| Notices and e-mail | none | this phase sends nothing to a customer |

### 3.19 Settings used (phase 00 `admin.settings`; floors only where governed)

- Decision path: `assess.payment_expiry_s` (600; max 600), `assess.inline_timeout_ms` (250; max 250), `assess.rate_per_second` (50), `assess.rate_burst` (500), `assess.gateway_cache_s` (30; max 30), `assess.domestic_country` (`IQ`).
- Ledger and holds: `assess.idempotency_hours` (48; min 24), `assess.hold_max_hours` (24; max 72).
- Lists: `assess.hold_list_kinds` (`["freeze"]`; must contain `freeze`), `assess.fallback_extract_hours` (12; max 12). Freshness uses phase 14 `screening.list_max_age_h` (section 3.6); there is no setting of this phase for it.
- Control: `assess.heartbeat_max_s` (90; max 120), `assess.release_window_min` (120; min 60), `assess.release_skew_s` (60; max 120), `assess.control_test_max_days` (7; max 7), `assess.repeat_alert_count` (3).
- Modes: `assess.shadow_min_days` (30; min 30), `assess.shadow_min_decisions` (1,000; min 500), `assess.shadow_review_min_ratio` (0.9; min 0.8), `assess.advisory_min_days` (14; min 14).
- Schedules (cron, six fields): `schedule.assess_release_match`, `schedule.assess_hold_sweep`, `schedule.assess_blind_watch`, `schedule.assess_rollup_watch`, `schedule.assess_fallback_extract`, `schedule.assess_ledger_sweep`.
- Flag: `pack.assess` (off). Phase 14 keys used: `screening.priority_map`, `ref.lists.thresholds`. Phase 21: `report.small_number_threshold`.

### 3.20 Lock order (phase 17 and 27 rule: the same order on every path)

1. `assess.policy_state` (the set counter) first, for any policy, mode or rule activation; then the rows of `assess.fail_policies`, `assess.amount_bands`, `assess.rail_modes` or `assess.prohibited_rules`; then the phase 04 `admin.change_requests` row (its engine takes it before `activate`, so the activation never takes it in a different order).
2. Decision path: the ledger row only (its primary key); no other lock. The decision, hold and candidate rows are inserted in one transaction after the ledger row.
3. Hold review: `assess.holds` row `FOR UPDATE`; then, for a list_match hold, the phase 14 alert row (`alert.screening_alerts`) as phase 14 `AlertService::decide` takes it. No other order exists.
4. Hold release pass (`GET /assess/v1/holds/{hold_ref}`): `assess.holds` row `FOR UPDATE`, then the insert of the pass decision, then the update of `release_decision_id`.
5. Sweeps: holds in batches with `FOR UPDATE SKIP LOCKED`, then their alerts; the release matcher takes no row lock except the exception inserts and the checkpoint row last.

### 3.21 Cedar: `D0_assess.cedar` and the `Ctx.via` value (phase 03 range D0 is free; grep of docs/plan confirms)

Phase 03 `Ctx.via` lists `session`, `pat` and `client_credentials`. This phase adds the value `gateway_cert` (documentation of the String value list; the type of `Ctx.via` does not change; DR-316). Roles are read as `principal.roles` (phase 03 form). One role per policy, `@role` annotated, as phase 14 does. Entities: no new entity type; resources are `Akr::System` for the route-level actions and plain `resource` elsewhere, as phase 14 does for its screening actions.

```cedar
// D0_assess.cedar (phase 29). The gateway is a service principal (admin.users kind service) that reaches the
// gateway listener with a pinned certificate; Ctx.via is "gateway_cert" (phase 03 value list, DR-316).
// Route scopes of section 2 admit every role below (checked row by row, section 6 item 14).

@id("assess.gateway.call")
permit (principal, action in [Akr::Action::"assess.payment.assess", Akr::Action::"assess.decision.read_own",
        Akr::Action::"assess.hold.status", Akr::Action::"assess.health.gateway"], resource)
when { principal.kind == "service" && context.via == "gateway_cert" };

@id("assess.config.cfg") @role("CFG")
permit (principal, action in [Akr::Action::"assess.fail_policy.read", Akr::Action::"assess.fail_policy.propose",
        Akr::Action::"assess.mode.read", Akr::Action::"assess.mode.propose", Akr::Action::"assess.mode.readiness"], resource)
when { principal.roles.contains("CFG") };

@id("assess.config.chk") @role("CHK")
permit (principal, action in [Akr::Action::"assess.fail_policy.read", Akr::Action::"assess.mode.read",
        Akr::Action::"assess.mode.readiness"], resource)
when { principal.roles.contains("CHK") };

@id("assess.config.ch") @role("CH")
permit (principal, action in [Akr::Action::"assess.fail_policy.read", Akr::Action::"assess.mode.read",
        Akr::Action::"assess.release_exception.list", Akr::Action::"assess.blind_period.list"], resource)
when { principal.roles.contains("CH") };

@id("assess.config.mro") @role("MRO")
permit (principal, action in [Akr::Action::"assess.fail_policy.read", Akr::Action::"assess.mode.read",
        Akr::Action::"assess.mode.readiness"], resource)
when { principal.roles.contains("MRO") };

@id("assess.m.config") @role("M")
permit (principal, action in [Akr::Action::"assess.fail_policy.read", Akr::Action::"assess.mode.read",
        Akr::Action::"assess.mode.readiness", Akr::Action::"assess.release_exception.list",
        Akr::Action::"assess.blind_period.list", Akr::Action::"assess.hold.list", Akr::Action::"assess.hold.read",
        Akr::Action::"assess.hold.claim", Akr::Action::"assess.hold.decide", Akr::Action::"assess.decision.search"], resource)
when { principal.roles.contains("M") };

@id("assess.ia.read") @role("IA")
permit (principal, action in [Akr::Action::"assess.fail_policy.read", Akr::Action::"assess.mode.read",
        Akr::Action::"assess.release_exception.list", Akr::Action::"assess.blind_period.list",
        Akr::Action::"assess.decision.search"], resource)
when { principal.roles.contains("IA") };

@id("assess.ss.holds") @role("SS")
permit (principal, action in [Akr::Action::"assess.hold.list", Akr::Action::"assess.hold.read",
        Akr::Action::"assess.hold.claim", Akr::Action::"assess.hold.decide"], resource)
when { principal.roles.contains("SS") };

@id("assess.adm.system") @role("ADM")
permit (principal, action in [Akr::Action::"assess.health.read", Akr::Action::"assess.control_test.record"], resource)
when { principal.roles.contains("ADM") };

// Purpose is required on a decision search: a search without one is refused whatever the role.
@id("assess.decision.search.purpose")
forbid (principal, action == Akr::Action::"assess.decision.search", resource)
unless { ["internal_audit", "supervisory_inspection", "other"].contains(context.purpose) };
```

Notes on the file: the gateway policy has no role check, so every gateway principal with a pinned certificate is admitted to the four gateway actions, and the baseline forbid of phase 03 refuses the inactive ones; no other role reaches a gateway action. The purpose forbid lists the three codes of phase 03 that IA and M use for a decision search (`other` carries a reason, phase 03 `reason_required`). Suspicion masking is a read rule of the service (section 3.18), not a Cedar condition.

### 3.22 Governance (GV-53; phase 04 inventory; phase 27 tiering answers)

- **Tier 1** by default for the decision path and the holds: a human decides every hold, and no model runs. The seven tiering answers of phase 27 section 3.1 are stored in `inventory_items.tier_basis` for the two inventory items, and the create call of each item (phase 04, `tier_basis`) carries all seven answers (kind `rule` for the prohibited rule set, kind `parameter_set` for the fail policy and mode set; phase 04 kinds, no new kind). The bank confirms the tier (Open doubts 15).
- **Targets**: latency p95 150 ms and p99 400 ms (section 3.1, section 6); availability target of the decision endpoint 99.9 per cent per month (proposed; the bank confirms); RTO 15 minutes; RPO zero for decisions (synchronous commit, research 25 15.6 item 4); two nodes.
- **Decision log**: the `assess.decisions` ledger (every decision, every mode) and the hold table (every hold, with its proposal and decision). Model risk: none in the path (DR-302).
- **Validation**: a phase 24 validation pack is issued for each inventory item before its first move to advisory (`ValidationPackService::issue`, parts of phase 24), and the rule path is tested with the example rule of section 3.9 (section 6, items 4 and 20) before a bank rule is activated.
- **Owner and validator** differ (phase 04 CHECK, GV-32). The bank names both (Open doubts 15).

---

## 4. Infrastructure Updates

### 4.1 Job kinds (Postgres `jobs`; coalesce key in brackets)

| Kind | Binary | Payload | Concurrency, attempts | Schedule or trigger |
|-|-|-|-|-|
| `assess_release_match` | worker | `{}` [`assess_release_match`] | 1, 3 | `schedule.assess_release_match` (every 15 minutes); section 3.14 |
| `assess_hold_sweep` | worker | `{}` [`assess_hold_sweep`] | 1, 3 | `schedule.assess_hold_sweep` (every minute); sections 3.5 and 3.8 |
| `assess_blind_watch` | worker | `{}` [`assess_blind_watch`] | 1, 3 | `schedule.assess_blind_watch` (every minute); section 3.13 |
| `assess_rollup_watch` | worker | `{ day }` [`assess_rollup_watch:<day>`] | 1, 3 | `schedule.assess_rollup_watch` (00:40 bank time), `day` = the previous bank day; section 3.12 |
| `assess_fallback_extract` | worker | `{ list_version_ids }` [`assess_fallback_extract`] | 1, 3 | `schedule.assess_fallback_extract` (05:30) and enqueued by `assess_hold_sweep` when the freeze-list version set changed; section 3.13 |
| `assess_ledger_sweep` | worker | `{}` [`assess_ledger_sweep`] | 1, 3 | `schedule.assess_ledger_sweep` (hourly); section 3.3 |

No job reads the core banking system (the worker reads `canon.payment_message` in the release matcher, which is the canonical copy of phase 10, not the CBS). The job names do not use the `ingest_` prefix. Each of the six jobs reads `pack.assess` first and returns without work while it is off; `assess_blind_watch` also needs at least one rail with an open mode row (section 3.13). The decision path has no job: it runs in the request. Every handler re-reads its owner first.

### 4.2 Timers and outbox

No timer kind and no outbox topic. Two PostgreSQL notification channels are added and are not outbox topics: `akr_assess_policy` (sent by the activation of section 3.11, received by every node's `PolicyCache`) and `akr_assess_gateway` (sent by a registration or revocation, received by `GatewayCache`). The freeze of section 3.8 uses the phase 14 alert chain and no outbox (DR-303). The alert clock of a list hold is the phase 14 timer of its alert, as for every other alert.

### 4.3 AppState additions

- **akr-api**: `HoldService`, `FailPolicyService`, `RailModeService`, `ProhibitedRuleService`, `AssessReadService`, `ControlTestService` (routers `assess::config` and `assess::holds` under `api::routes::assess`); the three change targets registered in `ChangeTargetRegistry` at start-up; `PgAssessDecisionRepository` is not in akr-api (only the reads and `HoldRepository` are).
- **akr-assess** (`AppState`): `pool` (as `akr_assess`), `valkey`, `settings`, `flags`, `crypto` (`akr-kek` wrapping and the personal family), `audit`, `index: Arc<ScreeningIndexRegistry>` (phase 14, one per process), `policy: Arc<PolicyCache>`, `gateways: Arc<GatewayCache>`, `signer: Arc<DecisionSigner>`, `limiter: GatewayRateLimiter`, `parser: MessageParser`, `config: Arc<AssessConfig>` (client CA path, node id, listener addresses). Its router is `assess::routes::gateway::router()`.
- **akr-worker**: `ReleaseControlService`, `HoldSweepService`, `OutageService`, `RollupService`, `FallbackExtractService`, `LedgerSweepService` in the job registry; `ReleaseControlService` takes the phase 10 canonical repository.
- **akr-ctl**: the subcommands of the header (`assess gateway`, `assess keys`, `assess rail add`, `assess policy export`, `assess extract export`, `assess outage record`, `assess verify-decision`).

### 4.4 Network zones, keys, compose, readiness, metrics, OpenAPI

- **Zones** (`deploy/zones.yaml`, `make zone-check`). `akr-assess` attaches to `z4_gateway` (the gateway listener on port 8443), `z4_db` (the same PostgreSQL), `z4_cache` (Valkey) and `z5_mgmt` (metrics and health), as phase 00 gives it. This phase adds `z1_bao` (OpenBao, internal, no egress) for the start-up unwrap of the decision and extract seeds and the reads of `keys/assess-*`; phase 00 gives `akr-assess` no OpenBao network, so this is a change to the zone matrix that the bank approves (Open doubts 1). Denied: `akr-assess` to `akr-api`, `akr-worker` and `z2_app`; `akr-api` and `akr-worker` to `z4_gateway`. `zone-check` probes each pair.
- **OpenBao policy for `akr-assess`** (amends the phase 00 AppRole line "akr-assess KV read db/akr_assess (grows in phase 29)"): transit encrypt and decrypt on `akr-kek` (seal wrapping and the personal DEK family), KV read `db/akr_assess`, `keys/assess-decision/*` and `keys/assess-extract/*`. Not granted: `transit/sign/akr-sign-decision` (DR-294).
- **Keys in KV**: `keys/assess-decision/v<N>.pub` and `keys/assess-extract/v<N>.pub` (verify-only, imported and fingerprinted as phase 27 `keys/model-artifact/v<N>.pub`). Seeds are never in KV: they are wrapped in `assess.signing_keys.wrapped_seed`.
- **Configuration files** on the node (not database rows): the bank CA bundle (`assess.client_ca`), the node id and the listener addresses. Provisioned at install and changed by restart.
- **Compose**: no new container. `akr-assess` stays in profile `phase2`, read-only root file system, `ulimits.core = 0`, non-root. The bank's local fallback component beside the gateway is outside this product (DR-305).
- **Readiness** (phase 00 rule): ready when the database answers, Valkey answers, the decision key is loaded, every enabled list's index is built, and the policy snapshot is loaded. Not ready makes the gateway's circuit open, which is the bank's fail policy.
- **Metrics** (Prometheus, `z5_mgmt`): `akr_assess_step_seconds{step}` (histogram, buckets from 0.5 ms to 400 ms), `akr_assess_decisions_total{rail,mode,outcome}`, `akr_assess_degraded_total{flag}`, `akr_assess_rate_limited_total{gateway}`, `akr_assess_replays_total`, `akr_assess_in_flight_refused_total`, `akr_assess_persist_failures_total`, `akr_assess_index_age_seconds{list}`, `akr_assess_policy_version`. The `gateway` label is the bounded `gateway_key`.
- **OpenAPI**: the people routes are in the akr-api document under tag `assess` (`/api/v1/assess/*`); the four gateway routes are in a separate document `assess-v1.json` exported by akr-assess for the bank's gateway team (API 4.2 "uses the OpenAPI document"). `make generate-api` regenerates the Orval client for `/api/v1/assess/*` only; the gateway document is not a frontend input.

### 4.5 Existing entity, schema, route and code updates (extensions of earlier phases; each is additive)

1. **Phase 03**: `Ctx.via` gains the value `gateway_cert` (documentation of the value list; the type does not change; DR-316).
2. **Phase 02**: 19 event kinds (the 18 rows of section 3.17 and `assess.rail_added` of section 7+) added to the catalogue with their `resource_type`, SIEM flag and detail keys; `assess_extract` added to the evidence owner registry; `PartitionRegistry` gains one entry for `assess.decisions` with `ahead_months = audit.partition_ahead_months` and the factory `assess.create_partition`.
3. **Phase 04**: three `ChangeTarget` implementations (`assess_policy`, `assess_mode`, `assess_rule`) registered in `ChangeTargetRegistry`; `inventory_items` kinds `rule` and `parameter_set` are used for them (no new kind).
4. **Phase 14**: the method `ScreeningService::open_hold_alert(run, candidate)` (extends phase 14 with one caller: `HoldSweepService`; it inserts a system `payment` run (`requested_by` NULL, `trigger_ref` = the hold id) and a candidate alert through `AlertRepository::insert_candidates`, and creates nothing else). The phase 14 `AlertService`, `FreezeTrigger` and masking are used unchanged. Two requirements on phase 14 (Open doubts 29): the alert's subject view of a hold alert reads the names from `assess.holds.party_names_enc` (the hold's `subject_ref` is no canonical payment party); and `screening_payment` skips an entry already alerted on a hold whose decision has the same match key, so one match gives one alert.
5. **Phase 20**: none (its `FreezeService` is called by phase 14 as before).
6. **Phase 23**: the partition clock of `assess.decisions` is `received_at` (section 1.4); `akr_retention` gets SELECT on the 19 tables of this phase and DELETE on `assess.node_status` only (section 1.3, DR-386). Phase 23 must not drop a decision partition while a hold references a row of it (the foreign key of `assess.holds` is `RESTRICT`), which is its own clock rule (Open doubts 16).
7. **Phase 24**: `admin.registers.source` CHECK widened by `assess_control` (section 1.4; DR-307); the review calendar gains the row kind `assess_control_test` (due `assess.control_test_max_days` after the last caught test per rail); the G1 command centre gains the "Assessment" tile (section 3.15). The register route of finding closure closes release exceptions (section 3.14 step 6).
8. **Phase 25**: the SIEM export and the support bundle exclude `assess.decisions`, `assess.holds`, `assess.hold_candidates` and `assess.release_exceptions` by table name (Open doubts 8).
9. **Phase 00**: the AppRole line of `akr-assess` (section 4.4); the zone matrix (`z1_bao` for `akr-assess`, section 4.4); the transit key `akr-sign-decision` stays reserved and unused (section 3.4; Open doubts 2).
10. **Phase 01**: the gateway principals are `admin.users` rows of kind `service` with `owner_user_id` and `expires_at`; they do not use `ClientCredentials` (Hydra), because the gateway authenticates by certificate (section 3.10).
11. **Phase 16 and 06**: the scenario `phase2_candidate` flag is read by no code of this phase (section 3.9); no change to the scenario tables.
12. **API.md** (the lead applies): section 3.19 rows changed to their final form (two rows added: `GET /assess/modes/{rail}/readiness` and `GET /assess/blind-periods`; the paths are relative to `/api/v1`); section 4.1 gateway rows unchanged; section 4.2 response gains `fail_policy` and `policy_version`; section 4.2 "Idempotency" states the gateway key scope and the replay of the stored bytes; the `format` values are `mt103`, `pacs008`, `canonical` (`rtgs` and `ach` are rails, not formats, section 3.2).
13. **DATA-MODEL section 19** (the lead applies): the sketch is replaced by section 1 of this phase (Open doubts 20).
14. **Phase 10 and 11**: none; the parser is new in phase 29 (section 3.2), and the release matcher reads the phase 10 table as it is.

---

## 5. Frontend Impact

The phase is operated by SS and M (holds), by M, CFG, CHK, MRO and CH (policy and modes), and by M, CH and IA (health and exceptions). The gateway has no screen.

### 5.1 S2 Screening queue (`/screening`; phase 14 screen S2; addition: tab "Holds")

- A tab "Holds" beside the alert queue: rows of `GET /api/v1/assess/holds` with the rail, the mode label ("held at the gateway", "advisory: not held at the gateway", "shadow: no gateway effect"), reason codes, age and the countdown to `expires_at`, and the proposal state ("proposed by {first decider} ... awaiting a second decider"). Columns and labels in Arabic and English (phase 00 i18n rule); logical CSS only.
- Hold view: candidates with their list code, the score bar of phase 14 and the explanation of phase 14 (the server renders it), the names as phase 14 shows them, and the single alert link for a list hold.
- Actions: claim (`If-Match`); propose release or reject (reason codes from family `decision`, text up to 500 characters); approve or refuse the proposal by a second person (the approve control is hidden from the proposer, with the reason "the first decider cannot approve"). Each action asks for the step-up assertion of class `decision` (phase 01 dialog).
- A suspicion-tier subject that the caller may not see is not listed (404 on open, as phase 14).

### 5.2 D3 Policy studio (`/policy`; phase 09 screen D3; addition: tab "Assessment")

- Panel "Fail policy": the matrix as a table (rail, corridor, currency, band, open or closed) and the bands per currency; "Propose a change" creates a change request (`cr_material`) with the whole matrix; the diff against the current set is shown before submission.
- Panel "Rail modes": one row per rail with its mode, its effective-from instant, the readiness criteria of section 3.7 with observed and required values (`GET /api/v1/assess/modes/{rail}/readiness`), and "Propose" for the next move (the proposal is refused in the UI when a criterion is not met, and the server refuses it again).
- Panel "Prohibited rules": read-only list of the active rules and their basis; "Propose a rule" opens the generic change-request form with object kind `assess_rule`.
- A note on each panel: the change is material, needs MLRO and CHK, and CH for a decrease.

### 5.3 G1 command centre (`/command`; phase 24 screen G1; addition: tile "Assessment")

- The tile shows the modes per rail, the list freshness, the open blind periods and the open release exceptions (counts). Visible to M, CH and IA; the tile opens `/policy` on the "Assessment" tab for M and CH, and the release list (`GET /api/v1/assess/release-without-decision`) for IA.

### 5.4 Generated hooks (Orval names from the paths)

`useGetAssessFailPolicies`, `usePutAssessFailPolicies`, `useGetAssessModes`, `usePutAssessModesRail`, `useGetAssessModesRailReadiness`, `useGetAssessHolds`, `useGetAssessHoldsId`, `usePostAssessHoldsIdClaim`, `usePostAssessHoldsIdDecision`, `useGetAssessDecisions`, `useGetAssessReleaseWithoutDecision`, `useGetAssessBlindPeriods`, `useGetAssessHealth`, `usePostAssessControlTests`.

### 5.5 Browser and caching rules

- Holds and decision rows are `Cache-Control: no-store`; the client keeps them in query state only, never in browser storage.
- Names are fetched when a hold is opened, and the open is one access event (section 3.17).
- A proposal or approval sends `Idempotency-Key` (a fresh UUIDv7 per click) and `If-Match` with the version shown; a 409 reloads the hold and shows the new state.

---

## 6. Verification

1. `make migrate`: `0091` to `0094` apply cleanly; a second run is a no-op; `\d+ assess.decisions` shows `PARTITION BY RANGE (received_at)` and four monthly partitions; `SELECT assess.create_partition(EXTRACT(YEAR FROM now())::int, EXTRACT(MONTH FROM now())::int)` returns the existing name without error.
2. `cargo check` for `akr-assess`, `akr-api`, `akr-worker` and `akr-ctl`: the SQLx compile-time checks pass for every query of sections 2 and 3, including the `UPDATE` column lists of section 1.3.
3. `cargo clippy -- -D warnings` on the four binaries and `domain/assess`.
4. `cargo test -p akr-assess -p domain-assess`: (a) canonical decision bytes against fixed vectors, with a key order and a timestamp format fixture; (b) parser fixtures for `mt103`, `pacs008` and `canonical`, a DTD refusal, a missing amount, and a party with no name (`party_unnamed`); (c) rule grammar: an unknown key refused; (d) hold state machine: every legal edge and a sample of illegal edges, mirroring the trigger of section 1.3; (e) ledger states with a fake clock: created, replay, in flight, lease takeover; (f) mode criteria at the thresholds (29 and 30 shadow days, 999 and 1,000 decisions); (g) `assess.decisions` append-only trigger refuses UPDATE and DELETE.
5. Curl suite `tests/curl/phase29_assess.sh` (section 6.1 below), run with the test CA and client certificates from `tests/curl/phase29/certs.sh` (a test CA, one valid gateway certificate `gw-a`, one revoked `gw-b`, one expired `gw-c`, one signed by another CA); dev users `cfg@`, `chk@`, `ch@`, `m@`, `ia@`, `ss@`, `adm@`, `mro@`, `ds@`, `a1@`, `sec@`.
6. `make generate-api` regenerates the client for `/api/v1/assess/*` with the hook names of section 5.4; the gateway document `assess-v1.json` is exported and not imported by the frontend.
7. The audit trail of the curl session contains the kinds of section 3.17 that the session triggers (`assess.gateway_registered`, `assess.key_activated`, `assess.policy_activated`, `assess.mode_changed`, `assess.hold_proposed`, `assess.hold_decided`, `assess.pass_issued_on_release`, `assess.release_exception_raised`, `assess.control_test_recorded`) and no `assess.decision` row (DR-296); `audit verify` passes.
8. `cargo deny check bans` and the phase 00 dependency-tree check: the dependency tree of `akr-assess` does not contain `akr-sources`, `tiberius`, `oracle` or `odbc-api`.
9. Existing tests pass (no regression), including the phase 00 readiness tests and the phase 27 and 28 registry tests that share `admin.registers`.
10. `make zone-check`: from the `z4_gateway` probe, port 8443 on `akr-assess` is open and port 5432 is closed; from the `akr-assess` probe, `openbao:8200` and `valkey` are open and `akr-api:8080` and `akr-worker` ports are closed; from the `akr-api` probe, `akr-assess:8443` is closed.
11. Partition path: insert a decision whose `received_at` is in a month without a partition through `AssessmentService`; the first attempt fails with SQLSTATE 23514, the factory runs once and the retry commits; a second failure answers 503 and leaves no row.
12. Guards: `UPDATE assess.decisions SET decision = 'pass'` and `DELETE FROM assess.holds` are refused with the trigger messages; `UPDATE assess.holds SET state = 'expired' WHERE state = 'released'` is refused; `UPDATE assess.holds SET release_decision_id = ... WHERE state = 'held'` is refused.
13. Cedar: `akr-ctl policy validate` accepts `D0_assess.cedar` and prints the new bundle hash; `POST /api/v1/admin/policies/simulate` returns the decisions of the matrix in section 6.2 for each role (the simulation changes nothing, phase 03).
14. Scope against Cedar: a script reads the route table of section 2 and, for each route, checks that every role in the Cedar permits of its actions holds one of the route's scopes (the scope list of API section 2: `dashboard.read` M, CH, IA; `config.author` CFG, DS, MRO; `config.approve` CHK, M, CH, MRO; `screening.work` SS, M; `audit.read` IA, SEC; `admin.system` ADM; `assess.call` gateway). Any role the Cedar admits and no scope admits fails the check.
15. Gateway refusals at the edge: a call without a client certificate fails the TLS handshake (curl exit 56 or 35, no HTTP status); a certificate signed by another CA fails the handshake; `gw-b` (revoked) gets 401 `E_UNAUTHENTICATED` within 30 seconds of the revocation on every node.
16. Byte-exact replay: the same body and `Idempotency-Key` twice; `cmp` of the two bodies is equal; the second carries `x-idempotent-replay: true`; the decision row count is one.
17. Same key, different body: 422 `E_IDEMPOTENCY_REUSE`; no second decision row.
18. In flight: a second request with the same key while a test hook holds the first one's lease: 409 `E_IDEMPOTENCY_IN_FLIGHT`.
19. Listed name: a payment with the test list entry gives `hold` with reason `LIST_MATCH` and `list` `iq_national` (the test list), and after `assess_hold_sweep` runs once the phase 14 queue shows one alert of run kind `payment` (`requested_by` NULL) for the hold, and no second alert for the same entry after the payment reaches `canon.payment_message`.
20. Prohibited destination: a payment whose corridor is in the active `PROHIBITED_DESTINATION_TEST_01` rule gets `reject` with `rule_id` `PROHIBITED_DESTINATION_TEST_01`. A score-only signal (a fuzzy name with a low-evidence match and no list hit) never gives `reject`: it gives at most `hold`.
21. Shadow: a rail in shadow answers `pass` with `mode: shadow`; the row has `would_be` set; for a would-be hold, `assess.holds` has state `shadow_review` and no alert exists.
22. Stale lists: with the test list's active version `activated_at` moved back past `min(refresh_hours, screening.list_max_age_h)` (phase 14 `ListRecord::stale`), a cross-border payment gets `hold` with `LISTS_STALE` and degraded `lists_stale`; a domestic payment gets `pass` with degraded `lists_stale` only.
23. Persistence failure: with the test fault `AKR_ASSESS_FAULT=persist` set on a test build, the answer is 503 `E_UNAVAILABLE`, no `assess.decisions` row exists for the request, the ledger row is removed, and no `pass` body was sent.
24. Signature: `akr-ctl assess verify-decision --pub keys/decision-v1.pub --response FILE` verifies a stored pass; the same file with `--now` set after `expires_at` is refused as `expired`; with `--require-enforced` a shadow pass is refused.
25. Hold released by two people: the first decider proposes `release`; the same person's approval gets 403 `E_SELF_APPROVAL`; a second person approves (with the step-up assertion); `GET /assess/v1/holds/{hold_ref}` then returns `status: released` with a signed pass whose `hold_id` matches; a second `GET` returns the same bytes; a missing `If-Match` gets 428 and a stale one 409.
26. Sanctions reject starts the freeze: the list hold's alert is decided `potential` by one person and `true` by another (phase 14 four-eyes); phase 20's `casework.freezes` gets one row with `screening_alert_id` set; the hold reads `rejected`; the gateway's `GET` returns `rejected` and no pass.
27. Release without decision: a canonical outgoing payment on an enforcing rail with no decision of its `match_key` appears in `GET /api/v1/assess/release-without-decision` after one run of `assess_release_match`; a payment with a `pass` decision in the window does not.
28. Fail-policy change bypassing the change request: a direct `INSERT INTO assess.fail_policies` without an approved change request is refused with `change_request_required`; the same change through `PUT /api/v1/assess/fail-policies` and its approval is accepted and bumps `set_version` once.
29. Mode change with a criterion not met: `PUT /api/v1/assess/modes/{rail}` to advisory on a rail with 29 shadow days is refused at the PUT (`prepare`) with 422 `E_VALIDATION` and `criterion_not_met`; `check_before_decision` repeats the check at approval.
30. Cedar deny and scope refusals: `a1@` on `GET /api/v1/assess/holds` gets 403 `E_SCOPE`; `ds@` (config.author) on `PUT /api/v1/assess/fail-policies` gets 403 `E_FORBIDDEN` (Cedar denies DS); `cfg@` on `POST /api/v1/assess/control-tests` gets 403 `E_SCOPE`; `sec@` on `GET /api/v1/assess/decisions` gets 403 `E_FORBIDDEN` (scope admitted, Cedar denies SEC).
31. Masking: `ia@` on `GET /api/v1/assess/decisions` for a decision that names a party of suspicion tier gets 404 `E_NOT_FOUND` with the same body as for an unknown id; `m@` with purpose `internal_audit` sees the row.
32. Purpose: `GET /api/v1/assess/decisions` without `X-Access-Purpose` gets 422 `E_PURPOSE_REQUIRED`.
33. Pack off: with `pack.assess` off, every `/api/v1/assess/*` route answers 404 `E_NOT_FOUND` and `POST /assess/v1/payments` answers 404.
34. Outage: stop `akr-assess` for three minutes; `assess_blind_watch` opens one blind period and raises one register warning; restarting closes it with `duration_s` near 180; `GET /api/v1/assess/blind-periods` shows it.
35. Fallback extract: after a freeze-list activation and one `assess_hold_sweep`, `assess_fallback_extract` writes one evidence object; its signature verifies with `keys/assess-extract/v<N>.pub`; after `assess.fallback_extract_hours` the watch raises the warning.
36. Load test: section 6.3.

### 6.1 Curl suite `tests/curl/phase29_assess.sh`

The suite covers, in order: a call without a client certificate refused at the edge; a call with the revoked certificate refused; a byte-exact idempotent replay; the same key with another body answered 422; a listed name gives `hold` with `LIST_MATCH`; a prohibited destination gives `reject` with `rule_id`; a score-only signal never gives `reject`; a shadow rail answers `pass` with `mode: shadow` and stores the would-be decision; stale lists turn a cross-border payment into `hold`; a persistence failure gives 503 and no `pass`; the signature verifies with the published key and an expired `pass` is refused by `akr-ctl assess verify-decision`; a hold released by two people yields a signed `pass` on the hold route; a sanctions reject starts the freeze; a released payment with no decision appears in the release-without-decision list; a Cedar deny gives 403; a fail-policy change that bypasses the change request is refused.

### 6.2 Cedar simulation matrix (item 13)

| Role | `assess.hold.decide` | `assess.fail_policy.propose` | `assess.decision.search` | `assess.health.read` | `assess.payment.assess` |
|-|-|-|-|-|-|
| SS | permit | deny | deny | deny | deny |
| M | permit | deny | permit (purpose required) | deny | deny |
| CFG | deny | permit | deny | deny | deny |
| CHK | deny | deny | deny | deny | deny |
| IA | deny | deny | permit (purpose required) | deny | deny |
| ADM | deny | deny | deny | permit | deny |
| gateway principal (`via gateway_cert`) | deny | deny | deny | deny | permit |
| A1 | deny | deny | deny | deny | deny |

### 6.3 Load-test plan (p95 and p99; no results recorded here)

- **Tool and place**: k6 (`tests/load/phase29_assess.js`) run from a host in `z4_gateway`'s test network against one `akr-assess` node, then against two nodes behind a balancer; mTLS with the test certificates of item 5; the list index loaded with 150,000 synthetic names (the N-50 size) and one enforcing rail with the fail policy and modes of the test set.
- **Profile A (steady)**: 50 requests per second for 15 minutes; 70 per cent `pacs008` cross-border payments with two party names each, 30 per cent `canonical` domestic; one in 200 requests a list hit; every request has a fresh key.
- **Profile B (burst)**: 500 requests per second for 20 seconds on one node, the rest of the mix unchanged.
- **Profile C (replays)**: 10 per cent of Profile A requests repeat an earlier key and body; each replay's bytes are compared with the first answer.
- **Pass criteria**: Profile A, server side (`akr_assess_step_seconds` summed per request): p95 at most 150 ms and p99 at most 400 ms; the inline screen alone p50 at most 5 ms and p99 at most 25 ms (N-50); zero 5xx; zero replay byte differences; zero duplicate `decision_id`. Profile B: every refusal is 429 `E_RATE_LIMITED` or a correct decision, zero 5xx, p99 of admitted calls at most 400 ms. Profile C: zero differences.
- **Not in the plan**: results, hardware figures and the recorded values belong to the test report, which is not part of this phase.

---

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 4b, from tools/agent/gap1-run4b.md; lead-verified (tools/agent/gap1-run4b-verdict.md).

### 7.1 Valkey outage (owner phase 30)
Phase 30 section 3.6 removes Valkey from the readiness list of `akr-assess` (section 4.4) and adds the in-process
`LimiterFallback` that answers the rate limit of section 3.1 step 3 while Valkey does not answer, with the capacity
phase 30 defines. No other rule of step 3 changes.

### 7.2 Reconnect reload of the caches (owner phase 30)
On a reconnect of the `LISTEN` connection, `PolicyCache` and `GatewayCache` reload at once (phase 30). The 30-second
refresh of both caches stays as the bound when a notification is lost.

### 7.3 Register warning titles (DR-388)
This phase adds seven variants to phase 24's closed enum `WarningTitle`, each with a bilingual title in the message
catalogue, and writes every warning with `RegisterWriter::warn(tx, 'assess_control', source_key, title, body)` from a
job (body: counts and ids only):
- `assess_rollup_watch` (section 3.12 step 3): `AssessKeyAge`, `AssessCertificateExpiry`, `AssessFallbackStale`,
  `AssessBlindPeriod` (an open outage period), `AssessControlTestStale`, `AssessRepeatAttempts`;
- `assess_blind_watch` (section 3.13): `AssessBlindPeriod` (`source_key = 'assess:blind:' || started_at`);
- `assess_release_match` (section 3.14 step 5): `AssessReleaseException`.

### 7.4 Gateway circuit breaker
The circuit breaker of section 4.4 is the bank gateway's, not an AKR-TMS component. Its threshold is a bank setting
and must not open before `ops.failover_max_s` (phase 30 Open doubts 10). AKR-TMS adds no setting for it.

Gap pass 1 (2026-10-10), run 4b: items carried from run 4a and corrections found while checking phases 25 to 31 (tools/agent/gap1-run4b-verdict.md); lead-verified.

### 7.5 Payment status in the release control (DR-389)
Phase 10 fixes the values of `canon.payment_message.status` (phase 10 section 7+). Section 3.14 step 1 skips rows
whose status is `rejected` or `cancelled` when they are read; any other value, an unexpected one included, is
controlled, so the control fails closed.

### 7.6 Retention DELETE grant (DR-386)
`akr_retention` gets SELECT on the 19 tables and DELETE on `assess.node_status` only (R12 `age`, no owner sweep, so
the engine's `record_purge` deletes it, DR-387). `assess.idempotency` is purged by `assess_ledger_sweep` as `akr_app`;
the R7 tables are `report_only`; `assess.decisions` partitions leave through `admin.retention_drop_partition`; R9 and
R10 tables are kept. Sections 1.3 and 4.5 item 6 read so.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.7 Adding a rail and its first mode (V-29-02-a, HIGH; DR-413)
`akr-ctl assess rail add --code C --label-ar T --label-en T --formats mt103,pacs008,canonical` (an operator with `admin.users`, as `keys generate`) inserts one `assess.rails` row in one transaction and audits `assess.rail_added {rail}` (resource `assess_rail`, SIEM false; a new kind in the table of section 3.17). It writes no mode row: the guard of section 1.3 refuses a `rail_modes` row without a change request.
The first mode row comes from `PUT /api/v1/assess/modes/{rail}` with `mode: shadow` on a rail that has no row. It is the one move besides those of section 3.7. The base version of a mode change is the number of `rail_modes` rows of the rail, so it is 0 here; `prepare` sets `volume_effect` to `neutral`, and no entry criterion applies. `prepare` also requires the rail's `*`, `*`, `*` row in the current matrix (422 `E_VALIDATION`, `errors[].code = wildcard_row_missing`), so step 7 of section 3.1 always finds a fail policy for an assessed rail; a policy set that names the new rail is activated first. `check_before_decision` requires the same row count (409 `E_STALE_BASE` otherwise). `activate` inserts the row with the readiness snapshot `{}` and supersedes nothing. A first proposal with another mode is 422 `E_VALIDATION` at `/mode`. The common rules of section 3.11 apply unchanged, the approved inventory item included. Until that activation the rail is not assessed (section 3.1 step 4).

### 7.8 Band sets and the matrix are replaced whole and kept by set version (V-29-03-a, HIGH; DR-413)
The primary key of `assess.amount_bands` is `(currency, band_code, set_version)` (section 1.2). A policy activation (section 3.11) inserts the whole band set of the proposal with the new `set_version` and updates no band row, so a band that the new set drops or renames stops applying without a DELETE, which the guard of section 1.3 refuses. The current bands (`PolicyRepository::current_bands`, step 7 of section 3.1 and the policy export) are the rows whose `set_version` is the greatest in the table. The older sets stay as the history that the guard text and the retention row of section 1.4 name, and the bands a decision used are the rows with the greatest `set_version` at or below its `policy_version`.
The matrix follows the same rule: the activation supersedes every current row of `assess.fail_policies`, not only the keys the new set names, and inserts the whole proposed matrix, so the current rows share one `set_version` (section 3.6).

### 7.9 The register link and the closure of a release exception (V-29-04-a, V-29-04-b, HIGH; DR-307; DR-413)
- **Link** (section 3.14 step 5). The warning of the run's rail and `run_date` is written before the exceptions, and every exception of that rail takes its id as `register_id`. When `RegisterWriter::warn` returns `None` (the key exists), the run reads the row with `RegisterRepository::find_by_source_key` (phase 24). When that row is already `closed`, the run writes a second warning whose key adds `':' || <run instant, RFC 3339>`, so a new exception always hangs on an open warning.
- **Closure** (step 6). The closing move of phase 24's register route on a row of source `assess_control` (phase 24 section 7.3) calls `ReleaseRepository::close_for_register(tx, register_id, closed_by, at)` in its own transaction: `closed_by` is the caller, `closed_at` the closing instant and `close_reason` `explained`, written together as the CHECK of section 1.2 needs. Each closed exception audits `assess.release_exception_closed`. The job `assess_release_match` closes nothing, and this phase adds no route. The values `bank_confirmed_manual` and `false_match` of the CHECK have no writer in this version.

### 7.10 The internal reference of the match key (V-29-04-d, MEDIUM; DR-413)
`ParsedPayment.internal_ref` (section 3.2) is field `20` of an `mt103`, `GrpHdr/MsgId` of a `pacs008` and `internal_ref` of a `canonical` body. On the canonical side the key `internal_ref` of `canon.payment_message.message_refs` holds the same reference: the bank's mapping of the payment source writes it (phase 10 section 7+) from the column of the gateway database that carries the reference the gateway sends. The bank's gateway team names that column at onboarding. A source whose mapping does not fill the key gives `no_match_key` for every payment without a UETR (section 3.14 step 3), so the gap shows in the exception counts and is never silent.

### 7.11 When the fallback limiter answers (V-29-09-a, MEDIUM; DR-413)
For section 7.1 and step 3 of section 3.1, Valkey "does not answer" when the limiter call returns a connection error or no reply within 5 ms (a constant of `GatewayRateLimiter`, not a setting). That request is decided by `LimiterFallback` (phase 30 section 3.6) and counted in `akr_assess_limiter_fallback_total`; the next request tries Valkey again. No request is refused or delayed further because Valkey is slow.

### 7.12 Source keys of the roll-up warnings (V-29-10-a, MEDIUM; DR-413)
Section 3.12 step 3 writes each warning with one of these keys, where `day` is the roll-up day (`YYYY-MM-DD`), so one condition raises at most one warning a day:
- `AssessKeyAge`: `'assess:key_age:' || version || ':' || day` (the active decision key's version);
- `AssessCertificateExpiry`: `'assess:cert_expiry:' || gateway_id || ':' || day`;
- `AssessFallbackStale`: `'assess:fallback_stale:' || day`;
- `AssessControlTestStale`: `'assess:control_test:' || rail || ':' || day`;
- `AssessRepeatAttempts`: `'assess:repeat:' || rail || ':' || day`, one warning per rail and day whose body holds the number of payment keys above the limit and no payment key;
- `AssessBlindPeriod`: the key of section 3.13 step 1 (`'assess:blind:' || started_at`), so the roll-up and the outage watch raise one warning per period.

## Decisions

1. **DR-292 (gateway identity: a pinned certificate and a service principal).** Each gateway is one row of `assess.gateways` that pins the SHA-256 of its leaf certificate and names one `admin.users` service principal. The bank CA must sign the chain, and the pin decides which gateway it is; a certificate outside the pin is refused even when the CA signs it. Revocation is a status change, effective within `assess.gateway_cache_s` on every node, and needs two operators because a compromised gateway must be refused at once. Rationale: C-55 and T-43 require mTLS with Cedar-bound client identities, SP-93 requires per-client limits, and the phase 01 `ClientCredentials` path (Hydra JWT) is not used because a gateway holds a certificate, not a secret that can be rotated by a token. The Cedar principal is the same `admin.users` row, so revocation of the principal and of the pin act on one identity.
2. **DR-293 (transport split: people on akr-api, gateways on akr-assess).** The fourteen people routes live under `/api/v1/assess/*` on `akr-api`, behind sessions, Cedar and step-up; the four gateway routes live under `/assess/v1/*` on `akr-assess`, behind the certificate only. `akr-assess` serves no browser route, sets no cookie and has no CORS, so a browser cannot reach the decision path even with a stolen session. Rationale: D4 of ARCHITECTURE (four binaries for security zoning), ARCHITECTURE section 21 (assessment nodes are stateless and in the gateway zone), and the phase 00 zone matrix.
3. **DR-294 (decision signing: an Ed25519 seed wrapped under akr-kek, held in memory).** The decision key is an Ed25519 seed that OpenBao wraps with the KEK `akr-kek` and each node unwraps once at start-up; signing happens in process. The transit key `akr-sign-decision` of phase 00 is not used for signing, because a transit signature would be an OpenBao call in the request path, which API 4.2 forbids, and transit keys for Ed25519 cannot be exported without weakening the key policy. Rotation is yearly and at once on compromise; verify keys are published in KV with a fingerprint read out on a second channel. Rationale: TS-24 and SP-11 (DEKs and wrapped keys cached in process memory), the phase 27 verify-key pattern (DR-261), and API 4.2 signature rules.
4. **DR-295 (a replay ledger of its own; the decision row holds the bytes).** The replay ledger is `assess.idempotency`, keyed by gateway and key, not `admin.idempotency_keys`: the phase 00 table is `akr_app`-only, stores sealed bodies under a key family `akr-assess` does not hold, and is keyed by a principal that the gateway would share with people. The byte-exact replay returns the `response_bytes` that were signed, so a replay and the original are the same bytes. The ledger is swept after `assess.idempotency_hours` (48), and `GET /assess/v1/decisions/{id}` serves the reconciliation after that window. Rationale: C-55 (byte-exact replay), API 4.2 (replay rules), and the partitioned decision table of DATA-MODEL section 19, whose unique key across partitions needs the partition column, which the ledger carries in `decision_received_at`.
5. **DR-296 (the decision row is the record; no audit row per payment).** Every decision is a row of `assess.decisions` with its signature and response bytes, and the audit stream holds the mutations (section 3.17). A row in `audit.audit_log` per payment would take the phase 02 stream lock once per payment, on every node, which is the throughput limit of the decision path. Rationale: C-55 and T-43 (decision log), the phase 02 audit design (one chain with a stream lock), and research 25 15.3 (the decision ledger is the evidence).
6. **DR-297 (a parser of its own, for the two message formats the bank sends; `canonical` for the rest).** `assess::parse` reads MT103 and pacs.008 subsets, refuses DTDs and entities, and accepts `canonical` JSON for every rail. A parse failure answers 422 and writes no decision, so nothing is passed unscreened, and the gateway's fail policy applies; the release control of section 3.14 finds any payment released after such a failure. `rtgs` and `ach` are accepted in canonical form only until the bank's formats are sampled (Open doubts 6). Rationale: research 25 15.1 and 15.3 (the API screens what the message carries and flags what it lacks), phase 10 and 11 deferring MT and MX parsing, and the no-DTD rule of secure XML handling.
7. **DR-298 (the modes and the hold kinds).** Shadow answers pass and records the would-be decision; a would-be hold becomes a review row in `shadow_review` that touches no gateway and no alert, so shadow adds no screening alert to the production queue. Advisory and enforcing create holds; a freeze-list hit holds, a PEP or internal hit holds only when its kind is in `assess.hold_list_kinds` (default freeze only); a list hold gets one phase 14 alert (the highest-scoring candidate) so that the four-eyes and freeze rules of phase 14 apply unchanged; every other hold is a review hold decided by `HoldService` with the same four-eyes rule. Rationale: research 25 15.9 (shadow first, advisory shows without holding, enforcing holds), C-56 (stages with maker-checker), and phase 14 section 3.6 (a freeze list hit cannot be decided by one person).
8. **DR-299 (the release pass is signed by akr-assess at fetch time).** When the hold is released, the next `GET /assess/v1/holds/{hold_ref}` inserts one `hold_release` decision (a pass, persisted before it is sent, signed with the one decision key) and records `release_decision_id` on the hold, once (guard, section 1.3). The signing process is therefore the one that holds the key, and akr-api never signs. Rationale: one place holds the decision key (DR-294), the gateway learns of a release by pull (research 25 15.7 names pull as the default), and a second person's approval is the recorded release.
9. **DR-300 (the three policy objects are change targets; material; no emergency).** Fail policies and bands, modes and prohibited rules are `ChangeTarget` kinds of phase 04, so their activation runs in the change engine with the four-eyes routes of the bank's route map, and the guards of section 1.3 refuse a direct write. Emergency changes are refused for all three: the emergency lever is the gateway's own fail policy, which the bank can change without this product. Rationale: C-56 and C-57 (maker-checker per rail and per policy), the phase 04 route map (`cr_material` with CH for a decrease), and the phase 27 experience (phase 27 Open doubts 18: no emergency change for a model).
10. **DR-301 (deterministic prohibited rules are the only rejects without a person; a closed grammar).** A reject without a person needs a `rule_id` of an active row, whose condition uses four keys only; a phase 06 scenario flag (`phase2_candidate`) is never read by code of this phase, and a scenario becomes a rule only through its own change request signed by the MLRO. Rationale: C-65 and research 25 15.3 ("REJECT is returned without a person only for deterministic conditions the bank has configured as prohibited"), S-26 (the first list of phase 2 candidates), and GV-53 (no ML-only holds).
11. **DR-302 (no model and no suppression inside the decision path).** The inline path runs the phase 14 index and the deterministic rules only. No model runs, so `decisions.model_version_id` of the DATA-MODEL sketch is dropped, and a score never leaves the path. Pair memory and good-guy rules (phase 14 sections 3.7 and 3.8) are not applied inline in this version: a known false positive is held and decided again by a person until the bank and the MLRO approve the inline application (Open doubts 12). Rationale: T-13 to T-15 and GV-53 (no model in a pre-release holding path before shadow evidence and a model-risk review), N-50 and N-51 (the budget and the partial flag), and the conservative default of research 25 15.5 for cross-border flows.
12. **DR-303 (the freeze hand-off goes through the phase 14 alert, not through akr-api).** `akr-assess` never calls into akr-api and writes no outbox topic. A list hold gets its phase 14 alert from `HoldSweepService` in akr-worker, and the freeze starts when a person decides that alert `true` (phase 14 `FreezeTrigger`, phase 20 `FreezeService`). Rationale: the phase 20 freeze requires a `screening_alert_id` that references the alert (its foreign key, `casework.freezes`), so a hand-off that skipped the alert could not create a valid freeze; and the zone matrix keeps `akr-assess` away from `akr-api` (section 4.4).
13. **DR-304 (list freshness: the phase 14 rule, cross-border holds, the stricter wording open).** A list is stale by the phase 14 rule `ListRecord::stale` (`now - active.activated_at` above `min(refresh_hours, screening.list_max_age_h)`, at most 12 hours for a freeze list under the CBI rule); this phase adds no grace. A cross-border decision on a stale list is a hold with `LISTS_STALE`; a domestic decision carries the degraded flag only. Rationale: API 4.2 "Degraded" and research 25 15.5 item 3 (cross-border), with C-57's "never answer RELEASE on stale lists" recorded as the stricter reading (Open doubts 4).
14. **DR-305 (the local fallback is a signed list extract the bank's own component reads; no second matcher).** The fallback beside the gateway receives a signed, time-limited extract of the freeze lists and runs under the bank's approved procedure. AKR-TMS builds and signs the extract and runs no matcher outside its zone. Rationale: research 25 15.6 item 2 asks for a local screen that answers without the central service, and the zone rules (no matcher of AKR-TMS in the gateway zone) and the M19 requirement of a local fallback screen beside the gateway are met by the extract; the extract is personal data and is sealed as evidence.
15. **DR-306 (outages are recorded by a watch and by the operator; a table of its own).** Nodes write a heartbeat; a minute watch opens and closes outage periods and raises a warning; the operator records outages the watch missed, with evidence. The table `assess.blind_periods` is separate from phase 11's `admin.blind_periods`, which is keyed by CBS source. Rationale: C-58 (a recorded pre-release blind period for any outage), research 25 15.6 item 3, and the difference between a gateway outage and a CBS source outage.
16. **DR-307 (the release control matches on a key, within a window, and raises exceptions on the register).** The matcher reads released outgoing payments of controlled rails from the canonical copy (phase 10) with a lag of 30 minutes, matches them to decisions by UETR or by a hash of the internal reference, and writes exceptions for no decision, no key, a pass that expired before the release (`expires_at` plus `assess.release_skew_s`) and a decision that was not a pass. Exceptions reach the phase 24 register with a new source `assess_control` (the CHECK widened in `0094`) and close when their finding is closed. Rationale: C-59 and research 25 15.7 (the release-without-decision control), phase 24 registers (every control result is a register finding), and the canonical copy as the source of releases (phase 10).
17. **DR-308 (control tests: recorded by admin.system; due on the review calendar).** The weekly control-test payment is sent through the real gateway by the bank; its result is recorded with `POST /api/v1/assess/control-tests` (scope `admin.system`, as the owned row of API 3.19), with the observed decision and evidence. The calendar of phase 24 shows the next test due, and a missed test is a warning. Rationale: C-58 and research 25 15.6 item 5 (a weekly control-test payment reported to the MLRO), and the API 3.19 row (`admin.system`).
18. **DR-309 (tipping-off: reasons only to the gateway; decision search for IA and M only, with purpose and masking).** The customer receives a generic code; the gateway receives reasons for its operations view; SS and M see names as phase 14 shows them; `GET /api/v1/assess/decisions` is refused to every other role, requires a purpose, and masks a suspicion-tier party exactly as phase 17 does. Aggregates are suppressed below `report.small_number_threshold`. Rationale: C-65 and API 4.2 (the customer sees a generic code), phase 17 rule 1 and 5 (suspicion-tier rows are masked), and SP-93 (no score to any surface).
19. **DR-310 (governance: Tier 1; the phase 04 inventory kinds; the validation pack before advisory).** Both policy sets and the rule set are inventory items with owner and validator apart, a validation pack from phase 24 before the first move to advisory, and the tiering answers of phase 27. Rationale: GV-53 (Tier 1 by default, latency and availability recorded, decision log for every hold), GV-03 and GV-32, and the phase 24 pack for each inventory item.
20. **DR-311 (entry criteria are governed settings with floors, and the snapshot is kept).** The criteria of shadow to advisory and advisory to enforcing are settings with minimums that the bank may raise but not lower (for example 30 shadow days and 14 advisory days), and each change stores the snapshot it was approved on in `rail_modes.readiness`. Rationale: C-56 (success criteria fixed in advance) and research 25 15.9 (each stage change is evidenced by volumes, latency and tests), with the floors as in the phase 24 pattern (a floor bounds the side that weakens the control).
21. **DR-312 (a hold expires and is never passed afterwards).** A hold past `assess.hold_max_hours` is `expired`; an expired hold is never released by a later pass, and the gateway applies its fail policy. Rationale: research 25 15.7 (`EXPIRED` is a state of a held payment) and API 4.2 (a pass is valid only until its expiry; a gateway releasing later calls again).
22. **DR-313 (retention and append-only: decisions R7 by partition, holds R7 by closure, outage evidence R10).** The decision ledger is R7 with the partition clock on `received_at` and an append-only trigger; holds and exceptions are R7 with the closure clock (an open record is never due); outage periods are R10 (never); control tests are R11 (ten years); configuration is R9. Partitions leave only through `admin.retention_drop_partition` (DR-187), which must not drop a partition that a hold references. Rationale: ARCHITECTURE section 17 (R7 for phase-2 decisions), DATA-MODEL section 19 (`decisions` partitioned by `received_at`), and the phase 23 clock rules.
23. **DR-314 (health: rates and states, never a customer).** The gateway health gives modes, list states and flag names; the people health gives counts by rail and node status; nothing names a customer, and small counts are suppressed. Rationale: SP-93 (no score or customer data outside the decision), phase 21 `report.small_number_threshold`, and the phase 24 G1 rule that a tile never shows a case count to A1 or A2.
24. **DR-315 (eighteen routes: twelve owned, two added, four on the gateway).** The owned rows of API 3.19 and 4 are kept with their scopes; two people routes are added, the entry criteria of a mode (`GET /api/v1/assess/modes/{rail}/readiness`, which the approver must see before a move) and the outage list (`GET /api/v1/assess/blind-periods`, which the control owners need for C-58). Rationale: section 2 and the lead's route budget (L: 15 to 25 routes).
25. **DR-316 (Cedar D0_assess and the gateway value of `Ctx.via`).** One file, `D0_assess.cedar`, in phase 03's range D0 (free). The gateway policy is keyed by the principal kind and the `gateway_cert` value of `Ctx.via`; people policies are one role per policy; the purpose is required for a decision search. Rationale: phase 03 (`Ctx` with required purpose and network; one role per policy in the phase 05 idiom), phase 14 (`84_screening.cedar` with `@role` annotations), and phase 25 (`siem.pull.service` uses the same service-principal test).

**Lead corrections (review 2026-10-09).** The release control counted any pass as matched, so a gateway releasing after the ten-minute expiry was never seen; a pass now matches only up to its `expires_at` plus governed `assess.release_skew_s` (60, max 120), and a later release is exception `pass_expired`. List freshness was redefined with a two-hour grace, which let a cross-border payment pass on a freeze list up to 14 hours old; the phase 14 rule `ListRecord::stale` is used unchanged and setting `assess.list_grace_hours` is gone. The hold alert was an `adhoc` run (a person's name search in phase 14) on a subject phase 14 cannot resolve, and post-event screening alerted the same match again; it is now a system `payment` run with `requested_by` NULL, with two requirements on phase 14 (Open doubts 29). The shadow-to-advisory criterion "no release exception" could never fail (shadow rails are not controlled) and is dropped. `akr_assess` reads only `(id, kind, status, expires_at)` of `admin.users` and may update `node_status.started_at`. The six jobs do nothing while `pack.assess` is off, and the outage watch needs a rail with an open mode, so an install without phase 2 records no outage. The roll-ups count `coalesce(would_be, decision)`, so advisory and enforcing days are not zero. PEP and internal hits hold only when their list kind is in governed `assess.hold_list_kinds` (default freeze only); otherwise they add `LIST_NOTED` to a pass.

## Open doubts (for gap pass 1 and the bank)

1. **Zone for OpenBao (lead decision).** `akr-assess` joins `z1_bao` for the start-up unwrap and the key reads; gap pass 1 amends phase 00 (compose networks, `deploy/zones.yaml`, the probe matrix of `make zone-check`). The bank approves the zone change at installation.
2. **Phase 00 amendments (lead decision).** Gap pass 1 amends phase 00 line 588: the transit key `akr-sign-decision` is reserved and unused (decisions are signed in process by a seed wrapped under `akr-kek`, DR-294); and the `akr-assess` AppRole line gains transit decrypt on `akr-kek` and KV read on `keys/assess-*` (section 4.4). The bank's OpenBao policy file is changed by the bank's operator.
3. **Case reference in the response.** Research 25 15.3 shows `case_ref` when a hold creates or joins a case. This phase does not create cases from holds (the phase 14 alert is the work item); the lead decides whether a hold should open a phase 17 alert instead.
4. **Stale lists: domestic payments.** C-57 says "never answer RELEASE on stale lists", which holds domestic payments too; API 4.2 and research 25 15.5 hold cross-border only. Section 3.1 step 12 follows the API. The bank and the MLRO decide; the change is one line of `prepare` and one setting.
5. **Shadow true matches.** A true match in shadow has no alert and no freeze (section 3.8). The bank decides whether a shadow true match must go to its sanctions process as a finding (this phase opens a register finding) or to a freeze by its own procedure.
6. **Parser formats for rtgs and ach.** No sample of the bank's RTGS or ACH message exists in the plan. The bank supplies samples, and a parser for each is a later change to `assess::parse`; until then these rails accept `canonical` only.
7. **Gateway operations view.** The gateway receives reason codes and list codes. The bank keeps that view away from any channel the customer can see (research 25 15.7 and file 30 section 8); the bank confirms the access rules of its gateway operators.
8. **SIEM and support bundles.** Phase 25 must exclude the four assess tables by name (section 4.5 item 8). Phase 25 has no such rule yet (requirement; the phase 25 owner confirms).
9. **Two-person rule for registration and revocation.** `akr-ctl assess gateway register|revoke` needs two operators (section 3.10). The bank confirms this, and the phase 01 step-up class to use for the CLI (the plan uses the phase 01 classes only; `identity_admin` is the candidate).
10. **Gateway rate limit.** The 50 requests per second and the 500 burst are one gateway's total across the nodes (section 3.10). The bank confirms the figure, and whether one limit per gateway is right for a bank with several gateways.
11. **Control-test routing.** The synthetic control payment must reach a suspense account in the bank's gateway and never a beneficiary. The bank sets that routing and confirms the control rail.
12. **Inline pair memory and good-guy rules.** Not applied inline (DR-302). A phase 14 extension would apply them to a payment subject; the phase 14 owner and the MLRO decide. Until then every known false positive is held.
13. **API 4.2 additions.** The response gains `fail_policy` and `policy_version`, and the gateway contract states the key scope of the replay; the lead applies both to API.md section 4.2.
14. **Fallback extract table and owner kind.** `assess.fallback_extracts` (section 1.2) and the evidence owner kind `assess_extract` (phase 02 registry, section 4.5 item 2) are new; the lead lists both in the DATA-MODEL and phase 02 changes.
15. **Tiers and inventory owners.** The bank confirms Tier 1 for the rule set and the parameter sets, names the owner and the validator of each, and confirms that the availability target (99.9 per cent a month) and the RTO and RPO of section 3.22 are acceptable.
16. **Partition drop and holds.** A decision partition is dropped only when no hold references it and its rows are past R7; phase 23's engine must check the composite foreign key of `assess.holds` before a drop, which its own rules do not yet state (requirement; the phase 23 owner confirms).
17. **Calendar and finding closure.** The phase 24 review calendar needs the new row kind `assess_control_test`, and the finding route of phase 24 must trigger the closure of release exceptions (section 3.14 step 6). Both are phase 24 changes that the phase 24 owner adds.
18. **Change classes of the mode moves.** The volume effect of each move is proposed in section 3.7 (`neutral`, `decrease`, `increase`). The phase 04 owner confirms the route map for a mode move that increases releases (rollback), which may need `CH` as for a decrease.
19. **Scope of the control test.** API 3.19 gives `admin.system` (ADM) to the control-test route; the control owner is the MLRO. The bank decides whether M should record the result, which would need a scope change in API section 2 (`screening.work` is wrong for it).
20. **DATA-MODEL and API edits.** `docs/DATA-MODEL.md` section 19 and `docs/API.md` sections 3.19 and 4.2 are changed by the lead to the final form of this phase, and `docs/DESIGN-SYSTEM.md` rows S2, D3 and G1 get their additions. This phase edits none of them.
21. **Release signal.** The release control assumes the canonical copy of outgoing payments (phase 10) shows a release by its presence. The bank confirms that its gateway feed writes a payment row only when the payment is sent, not when it is only queued.
22. **Hold names and suspicion tier.** A hold's names are shown to SS and M as phase 14 shows its subjects. The bank confirms that a payment subject without a canonical party (the usual case at the gateway) is shown to SS, and what masking applies when the matched party later gets a suspicion-tier case.
23. **Expiry of passes and queueing.** A pass expires after 10 minutes. The bank confirms that its gateway queues do not exceed that window in the normal case; a longer queue needs a longer `assess.payment_expiry_s` (governed, with the floor of section 1.4).
24. **Gateway service principals and SIEM.** Gateway principals are service principals with `expires_at` equal to the certificate's `not_after`; phase 25 rules that a service principal's expiry is within a year. The bank confirms whether a certificate of a longer life is accepted (the phase 25 rule would then need an exception for gateways).
25. **Gateway certificate authority.** The plan assumes one bank CA signs all gateway certificates. A second CA would need a second bundle; the bank confirms the trust model.
26. **Decision latency and availability.** The 99.9 per cent availability target and the p95 and p99 targets are proposals for the bank's SLO. The bank confirms them, and whether a degraded fail policy at the gateway is the accepted answer when the node is down.
27. **Synthetic labels.** The example rule `PROHIBITED_DESTINATION_TEST_01` and the list entries of the test fixtures use invented values; the production rule set is the bank's and is validated under phase 24 before its first activation.
28. **Freeze clock start.** The phase 20 freeze clocks start at the listing decision of the alert (phase 14 `true`); a hold that is released after the alert became `potential` does not create a freeze. The bank confirms that a potential match on a hold is held at the gateway until the true or false decision.
29. **Phase 14 requirements of the hold alert.** Gap pass 1 adds to phase 14: the subject view of an alert whose run is a hold `payment` run (`trigger_ref` = an `assess.holds` id) reads the party names from `assess.holds.party_names_enc` with the phase 14 masking; the `subject_id` comment of `alert.screening_runs` widens from adhoc runs to these runs; and `screening_payment` skips an entry already alerted on a hold whose decision has the same match key. Phase 14 also gains: an unchanged feed (`list.feed_unchanged`) renews the freshness clock, else an unchanged list goes stale and holds every cross-border payment.

## What This Phase Does NOT Touch

- No change to the core banking system, its read path or `akr-ingestd`; no source connection; no read of `raw` or of the CBS tables; the release matcher reads the canonical copy only (phase 10), and `akr_assess` reads no canonical table.
- No change to phase 14's matching, scoring, index, pair memory, good-guy rules, run kinds or S2 queue rules; the one addition is the method `ScreeningService::open_hold_alert` of section 4.5.
- No model, no score in the response, no ML in the path; no change to phase 27's families, tables, guards, kill switch or Cedar file `C1_models.cedar`.
- No change to the assistant of phase 28 or its tables, prompts or evaluation.
- No change to the phase 17 visibility predicate, the phase 18 case access list, the phase 19 QA grants or the phase 20 freeze and refrain logic beyond their existing ports.
- No new step-up class (phase 01 `decision` is used; `identity_admin` is a candidate in Open doubts 9); no change to the scope list of API section 2 (a scope change for the control-test route is Open doubts 19).
- No new outbox topic and no timer kind; the two PostgreSQL notification channels are not outbox topics.
- No change to the phase 00 readiness rule, the signed release bundle, the OpenBao layout beyond the `akr-assess` lines of section 4.4 and the `akr-sign-decision` reservation of section 3.4.
- No change to the object storage, the evidence vault's key families or the `idempotency` key family; the decision ledger does not use `admin.idempotency_keys`.
- No email, notice, export or report to a customer, and no customer-facing channel of any kind.
- No file of the plan other than `docs/plan/phase-29.md`: `docs/API.md`, `docs/DATA-MODEL.md`, `docs/DESIGN-SYSTEM.md`, `docs/plan/roadmap.md`, `docs/plan/research.md`, `docs/plan/status.md`, `tech-stack.md` and the phase files 00 to 28 are changed by the lead (Open doubts 2, 13, 14, 17 and 20).
