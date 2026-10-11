# Phase 14: Screening Engine & Lists

**Goal:** The screening engine that compares customers, counterparties and payment parties against the freeze lists (UN, the Iraqi national list, and OFAC, EU and UK where the bank opts in) and the bank's own blacklist: one Arabic-first name normalisation library with a SQL twin and parity corpus, list versions with checksums, counts reconciled and a known-answer harness that must pass before activation, an immutable in-memory index per version, a deterministic scorer with secondary identifiers and a versioned doubt policy, daily and event-driven runs that never duplicate an alert, alerts and decisions in the Office's three terms (true, potential, incorrect) with four-eyes where the research demands, pair memory and expiring good-guy rules as change-controlled memory, and screens S1 to S4.

**Dependencies:** Phase 09 (reference data, the business clock, reason-code families `decision`, `closure` and `exclusion`), Phase 11 (production ingestion, `data_as_of` and freshness). Through them phases 00 to 05 and 10 are used: the timers and jobs harness, StepUpService, the evidence store, Cedar and masking, change requests, the `SourceReader` port and the canonical party tables.
**Macro-phase:** P1
**Complexity:** XL (27 routes: 25 of section 2.8 and 2 of section 7.19; four change-request targets, three of section 3.10 and `harness_case` of section 7.19; a normaliser with a SQL twin, a scorer and four-eyes rules carry the weight)
**Binaries touched:** akr-api (routers `lists`, `screening`; the index registry) | akr-worker (jobs `list_fetch`, `list_parse`, `harness_run`, `list_activate`, `screening_lookback`, `screening_daily_base`, `screening_onboarding`, `screening_payment`, `screening_adhoc`, `pair_memory_sweep`, `list_stale_sweep`, timer kind `screening_alert_clock`) | akr-ingestd (job `ingest_bank_list` only, the bank's own blacklist from the CBS through `SourceReader`; the one CBS-facing piece of this phase). akr-ctl gains `lists verify` only. akr-assess unchanged (phase 29 uses the same index).
**Requirements carried:** N-01 to N-59 (MODULES M9 owner; grouped in section 3.1 to 3.9 by what they govern), M-12, M-24, SP-98, RP-54, UX-44, C-63, L-41, S-32, TS-19, TS-36 (as MODULES M9 lists them; this phase carries them through the screens, the four-eyes rules and the audit events named in section 3)
- Gap pass 1 (coverage), implemented here: L-36 (no amount, materiality or exposure threshold on any screening; freeze lists excluded from good-guy rules and bulk decisions); T-27 (Arabic-aware keys for comparison and blocking; no record merge, and a differing national ID gives a decision code); UX-58 (the screening hit screen of mockup 03), W-09 (weak-alias classes, patronymic alignment and the side-by-side hit screen); F-13 (customers, owners and payment parties screened against the UN, national and optional lists, refreshed within 12 hours, look-back on every list change, decisions with coded reasons), U-02 (a parse failure fails the version with an incident and the last good version stays), U-03 (alias quality classes, birth dates with precision, documents by type, name chains, original-script keys), U-09 (per-version fetch and load records, diff counts, look-back runs and alert outcomes); VA-11 (Arabic name matching with a score and coded reasons); VC-03 (the 12-hour freshness alarm and the one-hour re-screen measure, section 7+), VC-04 (versions staged, compared with the active version and gated before activation), VC-10 (fuzzy Arabic-to-Arabic token comparison with alignment), VC-11 (identifier adjustments only when comparable, no credit for a default date, versioned), VC-12 (the name-frequency evidence floor from the bank base), VC-24 (pair memory and good-guy rules with approver, expiry and the QA sample; good-guy rules never on freeze lists (DR-102), and pair memory on a freeze-list entry is allowed, four-eyes approved and always in the weekly QA sample (sections 3.7 and 7.3)), VC-31 (every closed alert a labelled pair; thresholds change only by change request); VC-06 (the look-back of parties and unlinked counterparties on every list change, and changed parties against every active list, section 7+).
- Gap pass 1 (coverage), partly implemented: L-32 (the known-answer harness with its corpus and dated runs), L-33 (subject kinds on each screening alert), L-34 (the insufficient-data outcome), L-35 (pair-keyed good-guy rules with expiry, change request and the freeze-list refusal); T-28 (the known-answer harness with recall gates); UX-43 (verdict sentence, identifiers, per-part table, frequency sentence, pair memory), W-07 (versioned lists and good-guy rules under change request), W-08 (customers, counterparties and payment parties screened), W-10 (the dated known-answer harness, look-backs and list diffs), W-18 (terrorist-list matches ranked first); B-09 (look-back re-screens on every list change), U-01 (fetch through the egress proxy, SHA-256, versions, diffs and the gate before activation), U-04 (one list model with kind, program and activation time), U-05 (look-back runs and the list lag from publication to activation), U-07 (the UN list with its 1518 designations and the national list), U-08 (UN designations, vessel entries and PF scenarios); VA-22 (versioned lists kept with their history, pair-keyed good-guy rules, proof of list age), VA-32 (Arabic keys on every name path with the known-answer harness), VO-10 (the dated known-answer harness); VB-01 (the known-answer harness with recall gates and the evidence level per hit), VB-37 (thresholds per list), VD-27 (Kurdish letters and the Kurdish lexicon), VD-32 (recall at each alert budget); VC-01 (versioned entries, diffs, delisting by absence and look-backs), VC-02 (national list diffs and delisting), VC-05 (entry counts and the count-drop block), VC-07 (candidate retrieval and per-field scoring), VC-08 (the Iraqi name pipeline), VC-09 (back-transliteration through the lexicon), VC-13 (person documents and vessel identifiers), VC-14 (name and identifier fields), VC-15 (the UTF-8 check and the truncation flag), VC-23 (pair memory with change-driven reopening and good-guy expiry), VC-25 (the dry run counting alerts on a sample), VC-26 (the daily harness and the budget report), VC-28 (the OFAC-emulation oracle with its difference report), VC-29 (configuration versions per run and change-request approval), VC-30 (four-eyes and reason codes on decisions), VC-33 (runs, alerts and versions kept), VC-35 (pluggable official list feeds), VC-37 (the side-by-side Arabic-first alert screen), VC-42 (the cadence check for official lists), VC-56 (bank list versions with their evidence object), VC-71 (name forms and the mother's chain).
- Gap pass 1 (coverage), not carried: VC-22 (address screening), VC-43 (third-party list copy reconciliation, Q12), VC-44 (vendor delta feeds, Q12); DR-404.

Research basis: `research/22-arabic-name-matching.md` sections 6 (normalisation), 7 (Latin side and the consonant key), 8 (common names), 9 (identifiers), 12 (evaluation) and 13 (pipeline), and `research/14-vendor-screening-engines.md` section 12 (list management, good-guy lists). The thresholds and role weights in those sections are starting values that the research itself marks as unvalidated; this phase makes them governed data (section 3.10) and the harness (section 3.9) is what proves them per bank.

---

## 1. SQLx Migration Changes

Five migrations, `0045_ref_name_forms_lists` to `0049_screening_settings_classification`, in the group order of DATA-MODEL s.23 (group 14, screening, after `feat` of phase 13 and before `kyc`). They create the `ref` list registry, the `alert` screening tables, the name-key functions, the grants, the settings, schedules, classification rows and retention rows. Nothing here touches a phase 10 canonical table or a phase 11 table; the bank list is written by the `akr_ingest` role through the grants of migration 0048.

All ids are UUIDv7 generated in the application (no database default). The few fixed seed ids below are constants written once in the seed file, the same way phase 00 fixes the system principal. The schema `ref` exists from phase 09 and the schema `alert` from phase 00.

### 1.1 `0045_ref_name_forms_lists.sql`

The key function is the SQL twin of the Rust `names` crate (section 3.1). `k1` is the strict consonant key used for weighted comparison; `block` is the loose key used only for candidate generation, as research 22 section 7.3 measured (q, g and k merged; t and th merged; d and dh merged; w, y and ayn dropped). The function is plpgsql so that every step is named and testable; it uses only `normalize`, `translate`, `replace`, `regexp_replace` and `lower`, all immutable, under the database collation `C.UTF-8` (DR-26). Its parity with the Rust function is a test (section 6, check 3), not an assumption.

```sql
-- migration 0045_ref_name_forms_lists.sql
-- Phase 14. The name-key functions (k1 strict key, block loose key; the SQL twin of crates/names) and the list registry
-- (MODULES M9, DATA-MODEL s.9, DR-06, DR-20, DR-26). No extension is required and no pg_trgm index is created (N-53).

CREATE OR REPLACE FUNCTION ref.name_key_k1(p_raw TEXT) RETURNS TEXT
LANGUAGE plpgsql IMMUTABLE PARALLEL SAFE STRICT AS $$
DECLARE
    s TEXT;
BEGIN
    s := normalize(p_raw, NFKC);                                                        -- k1.1 compatibility form
    s := regexp_replace(s, '[​-‏‪-‮⁦-⁩﻿؜ـً-ٰٟ]', '', 'g');  -- k1.2 Cf, tatweel, harakat
    s := translate(s, '٠١٢٣٤٥٦٧٨٩۰۱۲۳۴۵۶۷۸۹', '01234567890123456789');                     -- k1.3 Eastern digits
    s := lower(s);                                                                       -- k1.4
    s := translate(s, 'àáâãäåçèéêëìíîïñòóôõöùúûüýÿ', 'aaaaaaceeeeiiiinooooouuuuyy');       -- k1.5 Latin accents (27)
    s := replace(replace(replace(replace(replace(s, 'æ', 'ae'), 'œ', 'oe'), 'þ', 'th'), 'ð', 'dh'), 'ß', 'ss');   -- k1.5b L1 letters
    s := translate(s, 'øłđħŋı', 'oldhni');                                                  -- k1.5c L1 single letters
    s := translate(s, 'أإآٱؤئىةیێېےکەھہۆۇۊۋڕڑڵڤپچژگ', 'ااااوييهييييكهههووووررلفبجزك');   -- k1.6 Arabic letter folds (28)
    s := replace(replace(replace(replace(replace(replace(replace(s,
                 'sh', 'C'), 'ch', 'C'), 'kh', 'X'), 'gh', 'G'), 'dj', 'J'), 'th', 'V'), 'dh', 'U');   -- k1.7 digraphs
    s := translate(s, '2356789', 'EEXTHGQ');                                             -- k1.8 Arabizi digits
    s := translate(s,                                                                    -- k1.9 letters to key symbols (54)
                   'بتطثجحهخدضذرزظسصشعغفقكلمنوي' || 'bptjhxkcdrzsfvqglmnwy' || '''ʻʿʼʾ’',
                   'BTTVJHHXDDURZZSSCEGFQKLMNWY' || 'BBTJHXKKDRZSFFQGLMNWYEEEEEE');
    s := regexp_replace(s, '[^A-Z]', '', 'g');                                           -- k1.10 vowels, spaces, unmapped letters
    s := regexp_replace(s, '([A-Z])\1+', '\1', 'g');                                     -- k1.11 doubled symbols collapse
    RETURN s;
END
$$;

CREATE OR REPLACE FUNCTION ref.name_key_block_k1(p_raw TEXT) RETURNS TEXT
LANGUAGE sql IMMUTABLE PARALLEL SAFE STRICT AS $$
    SELECT regexp_replace(translate(ref.name_key_k1(p_raw), 'QGVUWYE', 'KKTD'), '([A-Z])\1+', '\1', 'g')
$$;

-- The list registry. A list is a source of designations (freeze), of politically exposed persons (pep, the register itself
-- is phase 15) or of the bank's own decisions (internal). Rows are configuration (R9) and are never deleted.
CREATE TABLE IF NOT EXISTS ref.lists (
    id                  UUID PRIMARY KEY,
    code                TEXT NOT NULL UNIQUE CHECK (code IN ('iq_national', 'un', 'ofac_sdn', 'eu', 'uk', 'bank')),
    name                JSONB NOT NULL CHECK (name ? 'ar' AND name ? 'en'),
    kind                TEXT NOT NULL CHECK (kind IN ('freeze', 'pep', 'internal')),
    enabled             BOOLEAN NOT NULL DEFAULT FALSE,           -- ofac_sdn, eu and uk are optional per bank (CR list_setting)
    feed_kind           TEXT NOT NULL CHECK (feed_kind IN ('upload_only', 'http_feed', 'cbs_snapshot')),
    feed_format         TEXT CHECK (feed_format IN ('un_xml', 'ofac_xml', 'eu_xml', 'uk_xml', 'iq_xlsx_export', 'iq_office_feed',
                                                    'csv_upload', 'cbs_snapshot')),
    source_ref          JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(source_ref) = 'object'),
                                                                  -- http: {"host", "path", "publisher": {ar, en}} (allow-listed);
                                                                  -- cbs: {"source_id", "object"} (approved object only)
    refresh_hours       INT NOT NULL DEFAULT 12 CHECK (refresh_hours BETWEEN 1 AND 24),
    script_plan         TEXT NOT NULL DEFAULT 'both' CHECK (script_plan IN ('arabic', 'latin', 'both')),   -- N-15
    thresholds          JSONB NOT NULL CHECK (jsonb_typeof(thresholds) = 'object'
                                              AND thresholds ? 'name_only' AND thresholds ? 'with_identifiers'
                                              AND thresholds ? 'evidence_floor_bits' AND thresholds ? 'review_band'),
    active_version_id   UUID,                                     -- FK added below (circular)
    version             INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (kind <> 'freeze' OR refresh_hours <= 12),              -- the CBI floor: freeze lists refresh at least every 12 hours
    CHECK (feed_kind <> 'cbs_snapshot' OR code = 'bank'),
    CHECK (code <> 'bank' OR feed_kind = 'cbs_snapshot')
);

-- One row per loaded version. Content is immutable once the version leaves `loading` (guard trigger, migration 0048).
CREATE TABLE IF NOT EXISTS ref.list_versions (
    id                  UUID PRIMARY KEY,
    list_id             UUID NOT NULL REFERENCES ref.lists(id) ON DELETE RESTRICT,
    seq                 INT NOT NULL CHECK (seq >= 1),           -- per list, monotonic
    version_label       TEXT NOT NULL CHECK (length(version_label) BETWEEN 1 AND 80),
    status              TEXT NOT NULL DEFAULT 'loading' CHECK (status IN ('loading', 'validated', 'active', 'superseded', 'failed')),
    origin              TEXT NOT NULL CHECK (origin IN ('feed', 'upload', 'cbs_snapshot')),
    failure_code        TEXT CHECK (failure_code IS NULL OR failure_code IN (
                            'av_rejected', 'parse_error', 'count_mismatch', 'conflicting_duplicates', 'count_drop',
                            'harness_failed', 'feed_unreachable', 'too_large', 'feed_unchanged')),
    expected_count      INT CHECK (expected_count IS NULL OR expected_count >= 0),   -- the publisher's own row count
    record_count        INT CHECK (record_count IS NULL OR record_count >= 0),       -- distinct records after de-duplication
    duplicate_count     INT NOT NULL DEFAULT 0 CHECK (duplicate_count >= 0),        -- identical rows removed (N-45)
    conflict_count      INT NOT NULL DEFAULT 0 CHECK (conflict_count >= 0),         -- same key, different content
    checksum            BYTEA CHECK (checksum IS NULL OR octet_length(checksum) = 32),   -- SHA-256 over canonical entries
    file_sha256         BYTEA CHECK (file_sha256 IS NULL OR octet_length(file_sha256) = 32),
    evidence_id         UUID REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,   -- null for cbs_snapshot
    normaliser_version  TEXT NOT NULL,
    parser_version      TEXT NOT NULL,
    diff_summary        JSONB CHECK (diff_summary IS NULL OR jsonb_typeof(diff_summary) = 'object'),  -- {added, removed, amended}
    requested_by        UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- the uploader or the bank reviewer who asked
    activated_by        UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    acknowledged_count_drop BOOLEAN NOT NULL DEFAULT FALSE,
    loaded_at           TIMESTAMPTZ,
    activated_at        TIMESTAMPTZ,
    superseded_at       TIMESTAMPTZ,
    version             INT NOT NULL DEFAULT 1,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (list_id, seq),
    UNIQUE (list_id, version_label),
    CHECK (status <> 'failed' OR failure_code IS NOT NULL),
    CHECK (status <> 'active' OR activated_at IS NOT NULL),
    CHECK (status <> 'superseded' OR superseded_at IS NOT NULL),
    CHECK (origin <> 'upload' OR requested_by IS NOT NULL),
    -- four-eyes on activation of anything a person supplied or reviewed (research 22 section 6.5 and 13.8)
    CHECK (origin = 'feed' OR activated_by IS NULL OR requested_by IS NULL OR activated_by <> requested_by)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_list_versions_active ON ref.list_versions (list_id) WHERE status = 'active';
CREATE INDEX IF NOT EXISTS idx_list_versions_list_status ON ref.list_versions (list_id, status);

DO $$ BEGIN
    ALTER TABLE ref.lists ADD CONSTRAINT fk_lists_active_version
        FOREIGN KEY (active_version_id) REFERENCES ref.list_versions(id) ON DELETE RESTRICT DEFERRABLE INITIALLY DEFERRED;
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

-- The entries of a version. `entry_key` is the designation's identity across versions (the publisher's id, scoped to the
-- list); `entry_fp` fingerprints the identifying fields and drives pair-memory invalidation (section 3.7).
CREATE TABLE IF NOT EXISTS ref.list_entries (
    id                  UUID PRIMARY KEY,
    list_version_id     UUID NOT NULL REFERENCES ref.list_versions(id) ON DELETE RESTRICT,
    list_id             UUID NOT NULL REFERENCES ref.lists(id) ON DELETE RESTRICT,
    entry_key           TEXT NOT NULL CHECK (length(entry_key) BETWEEN 1 AND 120),
    type                TEXT NOT NULL CHECK (type IN ('person', 'entity', 'vessel', 'other')),
    program             TEXT CHECK (program IS NULL OR length(program) <= 120),
    listed_on           DATE,
    delisted_on         DATE,
    mother_name_enc     BYTEA,                                   -- sealed (personal, field encryption)
    mother_name_hk      BYTEA CHECK (mother_name_hk IS NULL OR octet_length(mother_name_hk) = 32),   -- exact-match twin
    birth               JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(birth) = 'array'),
                                                                  -- [{year, month?, day?, precision, default_flag}]
    nationality         TEXT[] NOT NULL DEFAULT '{}',
    documents           JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(documents) = 'array'),
                                                                  -- [{type, hk}] only; the number itself stays in raw_enc
    alias_count         INT NOT NULL DEFAULT 0 CHECK (alias_count >= 0),
    entry_fp            BYTEA NOT NULL CHECK (octet_length(entry_fp) = 32),
    row_hash            BYTEA NOT NULL CHECK (octet_length(row_hash) = 32),
    raw_enc             BYTEA NOT NULL,                          -- the publisher's record as read, sealed (AAD = id)
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (list_version_id, entry_key, row_hash)
);
CREATE INDEX IF NOT EXISTS idx_list_entries_version ON ref.list_entries (list_version_id);
CREATE INDEX IF NOT EXISTS idx_list_entries_identity ON ref.list_entries (list_id, entry_key);

-- The names of an entry: the primary name, each alias and each former name, with the four forms (N-07, N-15).
CREATE TABLE IF NOT EXISTS ref.list_entry_names (
    id                  UUID PRIMARY KEY,
    entry_id            UUID NOT NULL REFERENCES ref.list_entries(id) ON DELETE RESTRICT,
    list_version_id     UUID NOT NULL REFERENCES ref.list_versions(id) ON DELETE RESTRICT,
    name_slot           SMALLINT NOT NULL CHECK (name_slot >= 0),
    raw                 TEXT NOT NULL CHECK (length(raw) BETWEEN 1 AND 500),
    light               TEXT NOT NULL,
    folded              TEXT NOT NULL,
    key                 TEXT NOT NULL,                           -- ref.name_key_k1 of the folded form, per token and whole
    block_key           TEXT NOT NULL,                           -- ref.name_key_block_k1 (candidate generation only)
    script              TEXT NOT NULL CHECK (script IN ('ar', 'lat', 'ku', 'mixed', 'other')),
    parts               JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(parts) = 'array'),
    alias_class         TEXT NOT NULL CHECK (alias_class IN ('primary', 'strong', 'weak', 'kunya', 'former')),   -- N-27
    quality_flags       TEXT[] NOT NULL DEFAULT '{}',
    UNIQUE (entry_id, name_slot)
);
CREATE INDEX IF NOT EXISTS idx_list_entry_names_block ON ref.list_entry_names (list_version_id, block_key);
CREATE INDEX IF NOT EXISTS idx_list_entry_names_key   ON ref.list_entry_names (list_version_id, key);

-- Seeds: six lists. Ids are fixed constants (the phase 00 convention for seeded principals). Feed URLs are not in the
-- migration: a host and path enter by change request (list_setting) once the bank has approved the allow-list entry (3.2).
-- ofac_sdn, eu, uk and bank start disabled: the bank opts in by change request, and bank needs an approved CBS object.
INSERT INTO ref.lists (id, code, name, kind, enabled, feed_kind, feed_format, source_ref, refresh_hours, script_plan, thresholds) VALUES
 ('01970000-0000-7000-8000-00000000a001', 'iq_national',
  '{"ar": "القائمة الوطنية العراقية", "en": "Iraqi national list"}', 'freeze', TRUE, 'upload_only', 'iq_xlsx_export',
  '{"publisher": {"ar": "مكتب الإبلاغ عن غسل الأموال وتمويل الإرهاب", "en": "AML/CFT Office"}}', 12, 'both',
  '{"name_only": 0.95, "with_identifiers": 0.78, "evidence_floor_bits": 10, "review_band": 0.05, "former_weight": 0.9, "alias_weight_cap": 0.6, "fuzzy_gate_batch": [3, 7], "fuzzy_gate_inline": [5, 8]}'),
 ('01970000-0000-7000-8000-00000000a002', 'un',
  '{"ar": "قائمة مجلس الأمن الدولي الموحدة", "en": "UN Security Council consolidated list"}', 'freeze', TRUE, 'http_feed', 'un_xml',
  '{"publisher": {"ar": "مجلس الأمن الدولي", "en": "UN Security Council"}}', 12, 'both',
  '{"name_only": 0.95, "with_identifiers": 0.78, "evidence_floor_bits": 10, "review_band": 0.05, "former_weight": 0.9, "alias_weight_cap": 0.6, "fuzzy_gate_batch": [3, 7], "fuzzy_gate_inline": [5, 8]}'),
 ('01970000-0000-7000-8000-00000000a003', 'ofac_sdn',
  '{"ar": "قائمة مكتب مراقبة الأصول الأجنبية", "en": "OFAC SDN list"}', 'freeze', FALSE, 'http_feed', 'ofac_xml',
  '{"publisher": {"ar": "وزارة الخزانة الأمريكية", "en": "US Treasury (OFAC)"}}', 12, 'both',
  '{"name_only": 0.95, "with_identifiers": 0.78, "evidence_floor_bits": 10, "review_band": 0.05, "former_weight": 0.9, "alias_weight_cap": 0.6, "fuzzy_gate_batch": [3, 7], "fuzzy_gate_inline": [5, 8]}'),
 ('01970000-0000-7000-8000-00000000a004', 'eu',
  '{"ar": "قائمة الاتحاد الأوروبي للعقوبات", "en": "EU sanctions list"}', 'freeze', FALSE, 'http_feed', 'eu_xml',
  '{"publisher": {"ar": "الاتحاد الأوروبي", "en": "European Union"}}', 12, 'both',
  '{"name_only": 0.95, "with_identifiers": 0.78, "evidence_floor_bits": 10, "review_band": 0.05, "former_weight": 0.9, "alias_weight_cap": 0.6, "fuzzy_gate_batch": [3, 7], "fuzzy_gate_inline": [5, 8]}'),
 ('01970000-0000-7000-8000-00000000a005', 'uk',
  '{"ar": "قائمة المملكة المتحدة للعقوبات", "en": "UK sanctions list"}', 'freeze', FALSE, 'http_feed', 'uk_xml',
  '{"publisher": {"ar": "المملكة المتحدة", "en": "United Kingdom"}}', 12, 'both',
  '{"name_only": 0.95, "with_identifiers": 0.78, "evidence_floor_bits": 10, "review_band": 0.05, "former_weight": 0.9, "alias_weight_cap": 0.6, "fuzzy_gate_batch": [3, 7], "fuzzy_gate_inline": [5, 8]}'),
 ('01970000-0000-7000-8000-00000000a006', 'bank',
  '{"ar": "القائمة الداخلية للبنك", "en": "Bank internal blacklist"}', 'internal', FALSE, 'cbs_snapshot', 'cbs_snapshot',
  '{"source_id": null, "object": null}', 24, 'both',
  '{"name_only": 0.95, "with_identifiers": 0.78, "evidence_floor_bits": 10, "review_band": 0.05, "former_weight": 0.9, "alias_weight_cap": 0.6, "fuzzy_gate_batch": [3, 7], "fuzzy_gate_inline": [5, 8]}')
ON CONFLICT (code) DO NOTHING;
```

### 1.2 `0046_alert_screening_runs_alerts.sql`

```sql
-- migration 0046_alert_screening_runs_alerts.sql
-- Phase 14. Screening runs, alerts (party or counterparty or payment party against one list entry), decisions and the
-- known-answer harness runs (MODULES M9; DATA-MODEL s.9 extended, see section 2 and the Proposed decisions).

CREATE TABLE IF NOT EXISTS alert.screening_runs (
    id                  UUID PRIMARY KEY,
    kind                TEXT NOT NULL CHECK (kind IN ('daily_base', 'lookback', 'onboarding', 'payment', 'adhoc')),
    status              TEXT NOT NULL DEFAULT 'queued' CHECK (status IN ('queued', 'running', 'completed', 'failed', 'cancelled')),
    list_version_ids    UUID[] NOT NULL DEFAULT '{}',
    subject_kind        TEXT CHECK (subject_kind IN ('party', 'counterparty', 'payment_party')),
    subject_id          UUID,                                    -- adhoc runs only
    trigger_ref         UUID,                                    -- list version (lookback), batch (onboarding)
    data_as_of          DATE,                                    -- the CBS business date read (DR-39), never the server clock
    checkpoint          JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(checkpoint) = 'object'),
    config_versions     JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(config_versions) = 'object'),   -- N-55
    parties_screened    BIGINT NOT NULL DEFAULT 0 CHECK (parties_screened >= 0),
    hits                BIGINT NOT NULL DEFAULT 0 CHECK (hits >= 0),
    suppressed          BIGINT NOT NULL DEFAULT 0 CHECK (suppressed >= 0),
    needs_more_data     BIGINT NOT NULL DEFAULT 0 CHECK (needs_more_data >= 0),
    not_assessable      BIGINT NOT NULL DEFAULT 0 CHECK (not_assessable >= 0),   -- too thin and no candidate: never "clear"
    below_line          BIGINT NOT NULL DEFAULT 0 CHECK (below_line >= 0),       -- inside the review band, sampled
    degraded_flags      TEXT[] NOT NULL DEFAULT '{}',            -- truncated_names, stale_source, list_stale, feed_unreachable
    requested_by        UUID REFERENCES admin.users(id) ON DELETE RESTRICT,   -- null for system runs
    started_at          TIMESTAMPTZ,
    finished_at         TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1,
    CHECK (kind <> 'adhoc' OR (subject_id IS NOT NULL AND subject_kind IS NOT NULL AND requested_by IS NOT NULL)),
    CHECK (status <> 'completed' OR finished_at IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_screening_runs_kind_status ON alert.screening_runs (kind, status, created_at);

CREATE TABLE IF NOT EXISTS alert.screening_alerts (
    id                      UUID PRIMARY KEY,
    run_id                  UUID NOT NULL REFERENCES alert.screening_runs(id) ON DELETE RESTRICT,
    subject_kind            TEXT NOT NULL CHECK (subject_kind IN ('party', 'counterparty', 'payment_party')),
    subject_id              UUID NOT NULL,                       -- no foreign key: canonical tables are bi-temporal (DR-87)
    entry_id                UUID NOT NULL REFERENCES ref.list_entries(id) ON DELETE RESTRICT,
    list_id                 UUID NOT NULL REFERENCES ref.lists(id) ON DELETE RESTRICT,
    list_version_id         UUID NOT NULL REFERENCES ref.list_versions(id) ON DELETE RESTRICT,
    list_kind               TEXT NOT NULL CHECK (list_kind IN ('freeze', 'pep', 'internal')),
    fingerprint             BYTEA NOT NULL UNIQUE CHECK (octet_length(fingerprint) = 32),   -- section 3.5: the dedupe key
    subject_fp              BYTEA NOT NULL CHECK (octet_length(subject_fp) = 32),
    entry_fp                BYTEA NOT NULL CHECK (octet_length(entry_fp) = 32),
    matching_major          SMALLINT NOT NULL CHECK (matching_major >= 1),
    score                   NUMERIC(6,4) NOT NULL CHECK (score BETWEEN 0 AND 1),
    evidence_bits           NUMERIC(8,2) NOT NULL CHECK (evidence_bits >= 0),
    aligned                 JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(aligned) = 'array'),       -- personal
    identifiers             JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(identifiers) = 'array'),   -- personal
    components              JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(components) = 'object'),   -- detection-logic
    folds                   JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(folds) = 'array'),
    truncated               BOOLEAN NOT NULL DEFAULT FALSE,
    priority_class          TEXT NOT NULL CHECK (priority_class IN ('P0', 'P1', 'P2', 'P3')),
    state                   TEXT NOT NULL CHECK (state IN ('open', 'potential', 'true', 'incorrect', 'needs_more_data')),
    claimed_by              UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    claimed_at              TIMESTAMPTZ,
    clock_timer_id          UUID,                                -- no foreign key: fired timers are pruned (phase 00 timers.terminal_retention_days)
    escalated_at            TIMESTAMPTZ,
    superseded_by           UUID REFERENCES alert.screening_alerts(id) ON DELETE RESTRICT,
    superseded_at           TIMESTAMPTZ,
    case_id                 UUID,                                -- no foreign key until casework.cases (phase 17)
    algorithm_version       TEXT NOT NULL,
    lexicon_version         TEXT NOT NULL,
    normaliser_version      TEXT NOT NULL,
    config_version          JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(config_version) = 'object'),
    doubt_policy_version    INT NOT NULL CHECK (doubt_policy_version >= 1),
    decided_at              TIMESTAMPTZ,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    version                 INT NOT NULL DEFAULT 1,
    CHECK ((claimed_by IS NULL) = (claimed_at IS NULL)),
    CHECK (state IN ('open', 'needs_more_data') OR decided_at IS NOT NULL),
    CHECK (superseded_by IS NULL OR superseded_at IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_screening_alerts_queue ON alert.screening_alerts (state, priority_class, created_at)
    WHERE superseded_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_screening_alerts_subject ON alert.screening_alerts (subject_kind, subject_id);
CREATE INDEX IF NOT EXISTS idx_screening_alerts_version ON alert.screening_alerts (list_version_id);
CREATE INDEX IF NOT EXISTS idx_screening_alerts_entry ON alert.screening_alerts (entry_id);
CREATE INDEX IF NOT EXISTS idx_screening_alerts_run ON alert.screening_alerts (run_id);

-- Decisions are append-only (trigger and grants, migration 0048). `prior_decider_ids` carries the four-eyes history into
-- the row so that the database CHECK and the Cedar forbid read the same facts (section 3.6).
CREATE TABLE IF NOT EXISTS alert.screening_decisions (
    id                      UUID PRIMARY KEY,
    screening_alert_id      UUID NOT NULL REFERENCES alert.screening_alerts(id) ON DELETE RESTRICT,
    seq                     SMALLINT NOT NULL CHECK (seq >= 1),
    outcome                 TEXT NOT NULL CHECK (outcome IN ('true', 'potential', 'incorrect')),
    reason_codes            TEXT[] NOT NULL CHECK (cardinality(reason_codes) BETWEEN 1 AND 5),   -- family decision, DEC_SCR_*
    text_enc                BYTEA,                               -- free text, sealed, at most 500 characters before sealing
    decider_id              UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    decider_role            TEXT NOT NULL CHECK (decider_role IN ('SS', 'M')),
    stepup_id               UUID NOT NULL,                       -- no foreign key: step-up rows are pruned (DR-78)
    purpose_code            TEXT,                                -- X-Access-Purpose, cited without an FK
    list_kind               TEXT NOT NULL CHECK (list_kind IN ('freeze', 'pep', 'internal')),
    doubt_policy_version    INT NOT NULL CHECK (doubt_policy_version >= 1),
    prior_decider_ids       UUID[] NOT NULL DEFAULT '{}',
    remembered_pair_id      UUID,                                -- set only with an incorrect decision (no FK: pair rows close, not vanish)
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (screening_alert_id, seq),
    CHECK (NOT (decider_id = ANY (prior_decider_ids))),
    CHECK (remembered_pair_id IS NULL OR outcome = 'incorrect'),
    CHECK (outcome <> 'true' OR list_kind <> 'freeze' OR cardinality(prior_decider_ids) >= 1)   -- four-eyes on freeze true
);
CREATE INDEX IF NOT EXISTS idx_screening_decisions_alert ON alert.screening_decisions (screening_alert_id);

-- The known-answer harness: gate runs decide activation; oracle and canary runs report only (section 3.9).
CREATE TABLE IF NOT EXISTS alert.harness_runs (
    id                      UUID PRIMARY KEY,
    list_version_id         UUID NOT NULL REFERENCES ref.list_versions(id) ON DELETE RESTRICT,
    kind                    TEXT NOT NULL CHECK (kind IN ('gate', 'oracle', 'canary')),
    corpus_sha256           BYTEA NOT NULL CHECK (octet_length(corpus_sha256) = 32),
    config_version          JSONB NOT NULL CHECK (jsonb_typeof(config_version) = 'object'),
    cases                   INT NOT NULL CHECK (cases >= 0),
    passed                  INT NOT NULL CHECK (passed >= 0 AND passed <= cases),
    recall_clean            NUMERIC(6,5) CHECK (recall_clean IS NULL OR recall_clean BETWEEN 0 AND 1),
    recall_variant          NUMERIC(6,5) CHECK (recall_variant IS NULL OR recall_variant BETWEEN 0 AND 1),
    hard_negative_alerts    INT NOT NULL DEFAULT 0 CHECK (hard_negative_alerts >= 0),
    passed_gate             BOOLEAN,
    failures                JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(failures) = 'array'),   -- case ids and codes, never names
    report                  JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(report) = 'object'),   -- budget curve, oracle diff, list lag
    requested_by            UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    ran_at                  TIMESTAMPTZ NOT NULL DEFAULT now(),
    finished_at             TIMESTAMPTZ,
    CHECK (kind <> 'gate' OR passed_gate IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_harness_runs_version ON alert.harness_runs (list_version_id, kind, ran_at);
```


### 1.3 `0047_alert_pair_memory_good_guys.sql`

```sql
-- migration 0047_alert_pair_memory_good_guys.sql
-- Phase 14. Pair memory (pair-keyed, four-eyes, fingerprinted; research 22 section 13.8, N-35) and good-guy rules
-- (name-pattern, expiring, change-controlled; N-36). Neither table is ever deleted: rows close.

CREATE TABLE IF NOT EXISTS alert.pair_memory (
    id                      UUID PRIMARY KEY,
    subject_kind            TEXT NOT NULL CHECK (subject_kind IN ('party', 'counterparty', 'payment_party')),
    subject_id              UUID NOT NULL,                       -- no foreign key (DR-87)
    entry_id                UUID NOT NULL REFERENCES ref.list_entries(id) ON DELETE RESTRICT,
    list_id                 UUID NOT NULL REFERENCES ref.lists(id) ON DELETE RESTRICT,
    list_kind               TEXT NOT NULL CHECK (list_kind IN ('freeze', 'pep', 'internal')),
    scope                   TEXT NOT NULL CHECK (scope IN ('customer', 'payment', 'both')),
    subject_fp              BYTEA NOT NULL CHECK (octet_length(subject_fp) = 32),
    entry_fp                BYTEA NOT NULL CHECK (octet_length(entry_fp) = 32),
    matching_major          SMALLINT NOT NULL CHECK (matching_major >= 1),
    status                  TEXT NOT NULL DEFAULT 'proposed' CHECK (status IN ('proposed', 'active', 'rejected', 'invalidated', 'expired')),
    proposed_by             UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    proposed_from_alert_id  UUID REFERENCES alert.screening_alerts(id) ON DELETE RESTRICT,
    reason_codes            TEXT[] NOT NULL CHECK (cardinality(reason_codes) BETWEEN 1 AND 5),
    approved_by             UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    approved_at             TIMESTAMPTZ,
    expires_at              TIMESTAMPTZ NOT NULL,
    invalidated_reason      TEXT CHECK (invalidated_reason IN ('subject_changed', 'entry_changed', 'matching_version',
                                                               'expired', 'manual', 'rejected_on_review')),
    closed_at               TIMESTAMPTZ,
    suppressions            BIGINT NOT NULL DEFAULT 0 CHECK (suppressions >= 0),
    last_suppressed_at      TIMESTAMPTZ,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    version                 INT NOT NULL DEFAULT 1,
    CHECK (approved_by IS NULL OR approved_by <> proposed_by),                  -- four-eyes
    CHECK ((status = 'active') = (approved_at IS NOT NULL AND closed_at IS NULL)),
    CHECK (status NOT IN ('invalidated', 'expired', 'rejected') OR closed_at IS NOT NULL),
    CHECK (expires_at > created_at)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_pair_memory_open ON alert.pair_memory (subject_kind, subject_id, entry_id)
    WHERE status IN ('proposed', 'active');
CREATE INDEX IF NOT EXISTS idx_pair_memory_entry ON alert.pair_memory (entry_id) WHERE status = 'active';
CREATE INDEX IF NOT EXISTS idx_pair_memory_expiry ON alert.pair_memory (expires_at) WHERE status = 'active';

CREATE TABLE IF NOT EXISTS alert.good_guy_rules (
    id                      UUID PRIMARY KEY,
    scope                   JSONB NOT NULL CHECK (jsonb_typeof(scope) = 'object' AND scope ? 'list_codes'
                                                  AND jsonb_typeof(scope -> 'list_codes') = 'array'),
    rule                    JSONB NOT NULL CHECK (jsonb_typeof(rule) = 'object'),   -- name pattern and identifier conditions
    reason_codes            TEXT[] NOT NULL CHECK (cardinality(reason_codes) BETWEEN 1 AND 5),
    approved_by             UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    change_request_id       UUID NOT NULL REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    expires_at              TIMESTAMPTZ NOT NULL,
    closed_at               TIMESTAMPTZ,
    closed_reason           TEXT CHECK (closed_reason IN ('expired', 'revoked', 'superseded')),
    suppressions            BIGINT NOT NULL DEFAULT 0 CHECK (suppressions >= 0),
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- a good-guy rule never names a freeze list (research 14 section 12.2: the bank's own rules are where the NYDFS failure sat)
    CHECK (NOT (scope -> 'list_codes' ?| ARRAY['iq_national', 'un', 'ofac_sdn', 'eu', 'uk'])),
    CHECK ((closed_at IS NULL) = (closed_reason IS NULL)),
    CHECK (expires_at > created_at)
);
CREATE INDEX IF NOT EXISTS idx_good_guy_rules_open ON alert.good_guy_rules (expires_at) WHERE closed_at IS NULL;
```

### 1.4 `0048_screening_grants_guards.sql`

Every write path has its grant (conventions). `akr_app` (api and worker) writes the alert, memory and harness tables and the list rows it activates. `akr_ingest` writes only the bank list (`cbs_snapshot` origin) and the entries of its own versions. No role has DELETE on any table of this phase. The guard triggers are the second layer under the grants.

```sql
-- migration 0048_screening_grants_guards.sql
-- Phase 14 grants and guard triggers (DATA-MODEL s.21 pattern). No DELETE for any role.

-- Lists: the six rows are seeded by 0045 and changed by change request (list_setting) and activation only.
GRANT SELECT ON ref.lists TO akr_app, akr_ingest, akr_assess, akr_readonly;
GRANT UPDATE (enabled, feed_format, source_ref, refresh_hours, script_plan, thresholds, active_version_id, version,
              updated_at) ON ref.lists TO akr_app;
GRANT SELECT, INSERT ON ref.list_versions TO akr_app;
GRANT INSERT ON ref.list_versions TO akr_ingest;
GRANT SELECT ON ref.list_versions TO akr_ingest, akr_assess, akr_readonly;
GRANT UPDATE (status, failure_code, expected_count, record_count, duplicate_count, conflict_count, checksum, diff_summary,
              loaded_at, activated_at, activated_by, superseded_at, version) ON ref.list_versions TO akr_app;
GRANT UPDATE (status, failure_code, expected_count, record_count, duplicate_count, conflict_count, checksum, diff_summary,
              loaded_at, version) ON ref.list_versions TO akr_ingest;
GRANT SELECT, INSERT ON ref.list_entries, ref.list_entry_names TO akr_app, akr_ingest;
GRANT SELECT ON ref.list_entries, ref.list_entry_names TO akr_assess;

-- Alerts, decisions, memory, rules and harness runs.
GRANT SELECT, INSERT ON alert.screening_runs, alert.screening_alerts, alert.screening_decisions TO akr_app;
GRANT UPDATE (status, checkpoint, config_versions, parties_screened, hits, suppressed, needs_more_data, not_assessable,
              below_line, degraded_flags, data_as_of, started_at, finished_at, version) ON alert.screening_runs TO akr_app;
GRANT UPDATE (state, priority_class, claimed_by, claimed_at, clock_timer_id, escalated_at, superseded_by, superseded_at,
              case_id, decided_at, updated_at, version) ON alert.screening_alerts TO akr_app;
GRANT SELECT, INSERT ON alert.harness_runs TO akr_app;
GRANT UPDATE (passed, recall_clean, recall_variant, hard_negative_alerts, passed_gate, failures, report, finished_at)
    ON alert.harness_runs TO akr_app;
GRANT SELECT, INSERT ON alert.pair_memory, alert.good_guy_rules TO akr_app;
GRANT UPDATE (status, approved_by, approved_at, expires_at, invalidated_reason, closed_at, suppressions, last_suppressed_at, version)
    ON alert.pair_memory TO akr_app;
GRANT UPDATE (closed_at, closed_reason, suppressions) ON alert.good_guy_rules TO akr_app;
GRANT SELECT ON ref.lists, ref.list_versions, alert.screening_runs, alert.harness_runs TO akr_readonly;

-- Guard triggers.
CREATE OR REPLACE FUNCTION ref.list_version_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'list versions are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF NEW.origin IS DISTINCT FROM OLD.origin OR NEW.list_id IS DISTINCT FROM OLD.list_id
       OR NEW.seq IS DISTINCT FROM OLD.seq OR NEW.version_label IS DISTINCT FROM OLD.version_label
       OR NEW.normaliser_version IS DISTINCT FROM OLD.normaliser_version
       OR NEW.parser_version IS DISTINCT FROM OLD.parser_version
       OR NEW.file_sha256 IS DISTINCT FROM OLD.file_sha256 OR NEW.evidence_id IS DISTINCT FROM OLD.evidence_id
       OR NEW.requested_by IS DISTINCT FROM OLD.requested_by
       OR (OLD.status <> 'loading' AND (NEW.checksum IS DISTINCT FROM OLD.checksum
                                        OR NEW.record_count IS DISTINCT FROM OLD.record_count)) THEN
        RAISE EXCEPTION 'list version identity and content are frozen' USING ERRCODE = 'check_violation';
    END IF;
    IF NEW.status IS DISTINCT FROM OLD.status AND NOT (
           (OLD.status = 'loading'   AND NEW.status IN ('validated', 'failed'))
        OR (OLD.status = 'validated' AND NEW.status IN ('active', 'failed'))
        OR (OLD.status = 'active'    AND NEW.status = 'superseded')) THEN
        RAISE EXCEPTION 'illegal list version transition % to %', OLD.status, NEW.status USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;

CREATE OR REPLACE FUNCTION ref.list_entry_insert_guard() RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE
    v_status TEXT;
BEGIN
    SELECT status INTO v_status FROM ref.list_versions WHERE id = NEW.list_version_id;
    IF v_status IS DISTINCT FROM 'loading' THEN
        RAISE EXCEPTION 'entries are written only while their list version is loading' USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;

CREATE OR REPLACE FUNCTION alert.append_only_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    RAISE EXCEPTION '% is append-only', TG_TABLE_NAME USING ERRCODE = 'insufficient_privilege';
END $$;

CREATE OR REPLACE FUNCTION alert.screening_state_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF OLD.state IN ('true', 'incorrect') AND NEW.state IS DISTINCT FROM OLD.state THEN
        RAISE EXCEPTION 'a % alert is terminal', OLD.state USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END $$;

CREATE OR REPLACE TRIGGER trg_list_version_guard BEFORE UPDATE OR DELETE ON ref.list_versions
    FOR EACH ROW EXECUTE FUNCTION ref.list_version_guard();
CREATE OR REPLACE TRIGGER trg_list_entries_insert BEFORE INSERT ON ref.list_entries
    FOR EACH ROW EXECUTE FUNCTION ref.list_entry_insert_guard();
CREATE OR REPLACE TRIGGER trg_list_entry_names_insert BEFORE INSERT ON ref.list_entry_names
    FOR EACH ROW EXECUTE FUNCTION ref.list_entry_insert_guard();
CREATE OR REPLACE TRIGGER trg_screening_decisions_append_only BEFORE UPDATE OR DELETE ON alert.screening_decisions
    FOR EACH ROW EXECUTE FUNCTION alert.append_only_guard();
CREATE OR REPLACE TRIGGER trg_screening_alerts_terminal BEFORE UPDATE OF state ON alert.screening_alerts
    FOR EACH ROW EXECUTE FUNCTION alert.screening_state_guard();
```

### 1.5 `0049_screening_settings_classification.sql`

Settings copy the phase 11 and 12 column list. `floor` is NULL throughout: this phase has no regulatory minimum of the `{min}` shape (the 12-hour freshness rule is a maximum and is enforced in code, section 3.2). Cron values are JSON strings with six fields.

```sql
-- migration 0049_screening_settings_classification.sql
-- Phase 14 settings, schedules, classification and retention rows (DATA-MODEL s.1, phase 00 classes).

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('screening.review_clock_hours',      '24', 'integer', NULL, FALSE,
  '{"ar": "مهلة مراجعة التنبيه الفحصي (ساعات عمل)", "en": "Screening alert review clock (hours, starting value)"}'),
 ('screening.list_max_age_h',          '12', 'integer', NULL, TRUE,
  '{"ar": "أقصى عمر لقائمة الجهات المحظورة (ساعات، حد مصرف مركزي)", "en": "Longest age of a freeze list (hours; CBI 12-hour refresh)"}'),
 ('screening.feed_auto_activate',      'true', 'boolean', NULL, TRUE,
  '{"ar": "تفعيل تلقائي لنسخ القوائم من المصدر بعد اجتياز الاختبار", "en": "Activate feed versions automatically after the harness gate"}'),
 ('screening.feed_max_bytes',          '104857600', 'integer', NULL, FALSE,
  '{"ar": "أقصى حجم لملف قائمة (بايت)", "en": "Largest list file accepted (bytes)"}'),
 ('screening.max_count_drop_pct',      '1', 'integer', NULL, TRUE,
  '{"ar": "أقصى نسبة انخفاض في عدد السجلات دون إقرار", "en": "Largest fall in distinct records without acknowledgement (percent)"}'),
 ('screening.search_per_min',          '20', 'integer', NULL, FALSE,
  '{"ar": "حد البحث بالاسم لكل مستخدم في الدقيقة", "en": "Name searches per user per minute"}'),
 ('screening.lookback_chunk',          '500', 'integer', NULL, FALSE,
  '{"ar": "عدد الأطراف في كل دفعة من المراجعة العكسية", "en": "Subjects per look-back chunk"}'),
 ('screening.pair_memory_max_months',  '12', 'integer', NULL, TRUE,
  '{"ar": "أقصى مدة لذاكرة الزوج (أشهر)", "en": "Longest pair-memory expiry (months)"}'),
 ('screening.good_guy_max_days',       '180', 'integer', NULL, TRUE,
  '{"ar": "أقصى مدة لقاعدة الاستثناء بالنمط (أيام)", "en": "Longest good-guy rule expiry (days)"}'),
 ('screening.doubt_policy',
  '{"needs_more_data": "review", "truncated_name": "review_flag", "thin_query_min_tokens": 2, "common_token_ceiling": 200, "min_bits_single_token": 7.5, "family_plus_common_given_alone": "never_alert"}',
  'json', NULL, TRUE,
  '{"ar": "سياسة الشك للفحص (المراجعة والأسماء القصيرة والحد الأدنى للأدلة)", "en": "Doubt policy (review of thin and truncated queries, minimum evidence)"}'),
 ('screening.priority_map',
  '{"freeze_identifier": "P0", "freeze_name_only": "P1", "pep": "P2", "internal": "P3", "escalate_step": 1}',
  'json', NULL, TRUE,
  '{"ar": "ربط فئة الأولوية بنوع القائمة", "en": "Priority class by list kind and evidence"}'),
 ('screening.harness_gate',
  '{"recall_clean": 0.995, "recall_variant": 0.95, "hard_negative_alerts_max": 0, "self_canary_sample": 200}',
  'json', NULL, TRUE,
  '{"ar": "بوابة اختبار الإجابات المعروفة قبل التفعيل", "en": "Known-answer gate before activation"}'),
 ('schedule.screening_daily_base',     '"0 30 3 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول الفحص اليومي لكامل القاعدة", "en": "Daily whole-base screening (bank time)"}'),
 ('schedule.screening_payment',        '"0 */15 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول فحص أطراف المدفوعات", "en": "Payment-party screening (every 15 minutes)"}'),
 ('schedule.list_stale_sweep',         '"0 */10 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول فحص حداثة القوائم وجلب المصادر المستحقة", "en": "List freshness sweep and due feed fetches"}'),
 ('schedule.pair_memory_sweep',        '"0 10 3 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول انتهاء ذاكرة الأزواج والعينة الأسبوعية", "en": "Pair-memory expiry and weekly QA sample"}')
ON CONFLICT (key) DO NOTHING;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'ref', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('lists', 'internal', ARRAY['id', 'code', 'name', 'kind', 'enabled', 'feed_kind', 'feed_format', 'source_ref',
                              'refresh_hours', 'script_plan', 'active_version_id', 'version', 'created_at', 'updated_at']),
  ('lists', 'detection-logic', ARRAY['thresholds']),
  ('list_versions', 'internal', ARRAY['id', 'list_id', 'seq', 'version_label', 'status', 'origin', 'failure_code',
                                      'expected_count', 'record_count', 'duplicate_count', 'conflict_count', 'checksum',
                                      'file_sha256', 'evidence_id', 'normaliser_version', 'parser_version', 'diff_summary',
                                      'requested_by', 'activated_by', 'acknowledged_count_drop', 'loaded_at', 'activated_at',
                                      'superseded_at', 'version', 'created_at']),
  ('list_entries', 'internal', ARRAY['id', 'list_version_id', 'list_id', 'entry_key', 'type', 'program', 'listed_on',
                                     'delisted_on', 'alias_count', 'row_hash', 'created_at']),
  ('list_entries', 'personal', ARRAY['mother_name_enc', 'mother_name_hk', 'birth', 'nationality', 'documents', 'entry_fp',
                                     'raw_enc']),
  ('list_entry_names', 'internal', ARRAY['id', 'entry_id', 'list_version_id', 'name_slot', 'script', 'alias_class',
                                         'quality_flags']),
  ('list_entry_names', 'personal', ARRAY['raw', 'light', 'folded', 'key', 'block_key', 'parts'])
) AS v(t, cls, cols)
CROSS JOIN LATERAL unnest(v.cols) AS c
ON CONFLICT DO NOTHING;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'alert', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('screening_runs', 'internal', ARRAY['id', 'kind', 'status', 'list_version_ids', 'subject_kind', 'subject_id',
                                       'trigger_ref', 'data_as_of', 'checkpoint', 'config_versions', 'parties_screened',
                                       'hits', 'suppressed', 'needs_more_data', 'not_assessable', 'below_line',
                                       'degraded_flags', 'requested_by', 'started_at', 'finished_at', 'created_at', 'version']),
  ('screening_alerts', 'internal', ARRAY['id', 'run_id', 'subject_kind', 'subject_id', 'entry_id', 'list_id',
                                         'list_version_id', 'list_kind', 'fingerprint', 'matching_major', 'truncated',
                                         'priority_class', 'state', 'claimed_by', 'claimed_at', 'clock_timer_id',
                                         'escalated_at', 'superseded_by', 'superseded_at', 'case_id', 'algorithm_version',
                                         'lexicon_version', 'normaliser_version', 'config_version', 'doubt_policy_version',
                                         'decided_at', 'created_at', 'updated_at', 'version']),
  ('screening_alerts', 'personal', ARRAY['subject_fp', 'entry_fp', 'aligned', 'identifiers']),
  ('screening_alerts', 'detection-logic', ARRAY['score', 'evidence_bits', 'components', 'folds']),
  ('screening_decisions', 'internal', ARRAY['id', 'screening_alert_id', 'seq', 'outcome', 'reason_codes', 'decider_id',
                                            'decider_role', 'stepup_id', 'purpose_code', 'list_kind', 'doubt_policy_version',
                                            'prior_decider_ids', 'remembered_pair_id', 'created_at']),
  ('screening_decisions', 'personal', ARRAY['text_enc']),
  ('harness_runs', 'internal', ARRAY['id', 'list_version_id', 'kind', 'corpus_sha256', 'config_version', 'cases', 'passed',
                                     'passed_gate', 'failures', 'requested_by', 'ran_at', 'finished_at']),
  ('harness_runs', 'detection-logic', ARRAY['recall_clean', 'recall_variant', 'hard_negative_alerts', 'report']),
  ('pair_memory', 'internal', ARRAY['id', 'subject_kind', 'subject_id', 'entry_id', 'list_id', 'list_kind', 'scope',
                                    'matching_major', 'status', 'proposed_by', 'proposed_from_alert_id', 'reason_codes',
                                    'approved_by', 'approved_at', 'expires_at', 'invalidated_reason', 'closed_at',
                                    'suppressions', 'last_suppressed_at', 'created_at', 'version']),
  ('pair_memory', 'personal', ARRAY['subject_fp', 'entry_fp']),
  ('good_guy_rules', 'internal', ARRAY['id', 'scope', 'reason_codes', 'approved_by', 'change_request_id', 'expires_at',
                                       'closed_at', 'closed_reason', 'suppressions', 'created_at']),
  ('good_guy_rules', 'detection-logic', ARRAY['rule'])
) AS v(t, cls, cols)
CROSS JOIN LATERAL unnest(v.cols) AS c
ON CONFLICT DO NOTHING;

UPDATE admin.column_classification SET encrypted = TRUE
 WHERE (schema_name, table_name, column_name) IN (('ref', 'list_entries', 'mother_name_enc'),
                                                  ('ref', 'list_entries', 'raw_enc'),
                                                  ('alert', 'screening_decisions', 'text_enc'));
UPDATE admin.column_classification SET keyed_hash = TRUE
 WHERE (schema_name, table_name, column_name) IN (('ref', 'list_entries', 'mother_name_hk'),
                                                  ('ref', 'list_entries', 'documents'));

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('ref', 'lists',              'R9', 'never',   NULL,          'Configuration history; rows are never deleted'),
 ('ref', 'list_versions',      'R7', 'age',     'loaded_at',   'Screening and sanctions records (ARCHITECTURE s.17)'),
 ('ref', 'list_entries',       'R7', 'owner',   NULL,          'As its list version'),
 ('ref', 'list_entry_names',   'R7', 'owner',   NULL,          'As its entry'),
 ('alert', 'screening_runs',   'R7', 'age',     'finished_at', 'Run records; counts only, no subject names outside alerts'),
 ('alert', 'screening_alerts', 'R7', 'decision', 'decided_at',  'Decided alerts; undecided alerts stay as open work'),
 ('alert', 'screening_decisions', 'R7', 'decision', 'created_at', 'Decision records (the four-eyes history)'),
 ('alert', 'harness_runs',     'R7', 'age',     'ran_at',      'Validation evidence that an activation cites'),
 ('alert', 'pair_memory',      'R7', 'closure', 'closed_at',   'Closed pair memory; open rows stay'),
 ('alert', 'good_guy_rules',   'R7', 'closure', 'closed_at',   'Closed good-guy rules; the change request is R9')
ON CONFLICT DO NOTHING;
```

### 1.6 Migration notes and reconciliation

- **DATA-MODEL s.9 changes (aligned in the same change, section 4.4):** `ref.list_versions.status` gains `validated` (between a parsed load and an activation, so the harness gate has a state to read); `ref.list_entries` stores the publisher's record sealed in `raw_enc` instead of a plaintext `raw jsonb`; `ref.list_entries.entry_key` and `entry_fp` are added so that the identity of a designation survives a new version; `alert.screening_alerts.party_id` becomes `subject_kind` and `subject_id` so that counterparties and payment parties are screened by the same table; `alert.pair_memory` gains `proposed_by`, `status` and `closed_at` for four-eyes approval; `alert.screening_decisions` gains `prior_decider_ids` for the freeze four-eyes; `alert.good_guy_rules` gains `change_request_id` (already in DATA-MODEL s.9) with a freeze-list CHECK.
- **Dependencies on earlier DDL:** `admin.evidence_objects` (phase 02, migration 0009), `admin.timers` (phase 00, 0002), `admin.change_requests` (phase 04, 0014), `admin.users`, `admin.settings`, `admin.column_classification` and `admin.retention_assignment` (phase 00, 0004), `ref` schema (phase 09). The `ref.name_key_*` functions use no extension.
- **Order:** 0045 before 0046 (foreign keys); 0047 after 0046 (pair memory references alerts); 0048 after all four tables; 0049 last.

---

## 2. DDD Implementation (Rust)

**Crate placement** (workspace pinned in phase 00; layer rules in `ddd.md`):

```
backend/
├── crates/names/            # NEW, pure (no IO, no database): NameNormaliser, forms, parser, fold and key tables,
│                            #   the parity corpus loader; depends on unicode-normalization only; used by domain,
│                            #   db tests, ingestd and worker. Added to the phase 00 workspace members in the same change.
├── crates/domain/src/screening/   # ListRegistry values, ScreeningIndex, Scorer, SubjectFingerprints, DoubtPolicy,
│                                  #   AlertDedupe, FreezeTrigger port, the four change targets (pure logic, ports only)
├── crates/db/src/screening/       # Pg repositories for the ten tables of section 1 and alert.harness_cases (section 7.19)
├── crates/authz/src/resources/    # resource adapters ScreeningAlert, PairMemory, ListVersion, GoodGuyRule, ListRegistry
├── crates/jobs/                   # kinds of section 4 (enqueue takes &mut Tx)
├── api/src/routes/lists.rs, api/src/routes/screening.rs   # routers `lists`, `screening`
├── worker/src/screening/          # the job handlers, the feed fetcher (AKR_EGRESS_PROXY), the look-back runner
├── ingestd/src/bank_list.rs       # job ingest_bank_list (SourceReader from phase 05; only ingestd depends on it)
└── policies/bundle/84_screening.cedar   # section 3.11
```

`crates/names` is the only place where the normalisation rules live. Every other crate calls it. The SQL twin (section 1.1) is generated by hand from the same tables and proven equal by the parity corpus (section 6, check 3).

### 2.1 Value objects (`names` and `domain::screening`)

- `NameForms { raw, light, folded, key, block_key, script: Script, parts: Vec<NamePart>, alias_class: AliasClass, quality_flags: Vec<NameFlag>, folds: Vec<FoldApplied>, normaliser_version: &'static str }`
- `Script` (`Arabic`, `Latin`, `Kurdish`, `Mixed`, `Other`; mixed names are scored per script, research 22 section 13.11); `AliasClass` (`Primary`, `Strong`, `Weak`, `Kunya`, `Former`; `Ord` by weight); `NamePart { text, folded, key, role: PartRole, idf_bits: Option<f32> }`; `PartRole` (`Given`, `Father`, `Grandfather`, `GreatGrandfather`, `TribeOrNisba`, `Laqab`, `Kunya`, `Title`, `Particle`, `MotherChain`, `Unknown`).
- `NameFlag` (`Truncated35`, `Truncated140`, `MixedScript`, `Meaningless`, `DefaultDate`, `DigitsInName`, `EmptyAfterFold`); a flag never lowers a score (DATA-MODEL s.5, phase 10 `maybe_truncated`).
- `SubjectKind` (`Party`, `Counterparty`, `PaymentParty`); `ListCode` (closed: `IqNational`, `Un`, `OfacSdn`, `Eu`, `Uk`, `Bank`); `ListKind` (`Freeze`, `Pep`, `Internal`); `ListOrigin` (`Feed`, `Upload`, `CbsSnapshot`); `ListStatus` (`Loading`, `Validated`, `Active`, `Superseded`, `Failed`).
- `AlertState` (`Open`, `Potential`, `True`, `Incorrect`, `NeedsMoreData`), `DecisionOutcome` (`True`, `Potential`, `Incorrect`), `Priority` (`P0` Immediate to `P3` Routine, `Ord` with P0 highest; DR-16 wording), `RunKind` (`DailyBase`, `Lookback`, `Onboarding`, `Payment`, `Adhoc`).
- `Thresholds { name_only: f32, with_identifiers: f32, evidence_floor_bits: f32, review_band: f32, former_weight: f32, alias_weight_cap: f32, fuzzy_gate_batch: (u8, u8), fuzzy_gate_inline: (u8, u8) }` read from `ref.lists.thresholds`.
- `DoubtPolicy { version: i32, needs_more_data: DoubtAction, truncated_name: DoubtAction, thin_query_min_tokens: u8, common_token_ceiling: u32, min_bits_single_token: f32 }` read from `screening.doubt_policy`; `DoubtAction` (`Review`, `ReviewFlag`).
- `MatchingVersion { major: u16, algorithm: &'static str, lexicon: &'static str }`; the major number goes into every fingerprint and every pair-memory row.
- `Identifiers { birth: Vec<BirthDate { year: i16, month: Option<u8>, day: Option<u8>, precision: Precision, default_flag: bool }>, mother_chain: Option<NameForms>, documents: Vec<DocKey { doc_type: String, hk: [u8; 32] }>, nationality: Vec<IsoCountry>, gender: Option<Gender>, family_no_hk: Option<[u8; 32]> }`.
- `ScoreResult { score: f32, evidence_bits: f32, aligned: Vec<AlignedPair>, identifier_checks: Vec<IdentifierCheck>, folds: Vec<FoldApplied>, components: MatchComponents, outcome: MatchOutcome, truncated: bool }` with `MatchOutcome` (`Hit`, `BelowLine`, `NeedsMoreData`, `Clear`).

### 2.2 Entities (`Debug, Serialize, Deserialize, FromRow, ToSchema`)

- `ListRecord` (`ref.lists`): `id, code: ListCode, name: I18n, kind, enabled, feed_kind, feed_format: Option<FeedFormat>, source_ref: serde_json::Value, refresh_hours, script_plan, thresholds: Thresholds, active_version_id, version`. Serialised with `last_loaded_at`, `age_hours`, `stale` computed (section 3.2), never stored.
- `ListVersion` (`ref.list_versions`): all columns of the table; `status: ListStatus` (`#[sqlx(type_name = "text", rename_all = "snake_case")]`; the column is TEXT with a CHECK).
- `ListEntry` and `ListEntryName` (`ref.list_entries`, `ref.list_entry_names`). `ListEntry` never carries `raw_enc` in a serialised response; the detail route decrypts it inside the handler for SS and M only.
- `ScreeningRun`, `ScreeningAlert`, `ScreeningDecision`, `HarnessRun`, `PairMemory`, `GoodGuyRule`: the tables of section 1 one to one, with `affordances: Vec<&'static str>` on the alert and pair-memory responses (API cross-cutting rule).

### 2.3 Create, update and request DTOs

- `UploadListVersion { label: String, acknowledge_count_drop: bool, file: multipart }` (upload field `file`, at most `screening.feed_max_bytes`; the multipart body limit is that value plus 1 MiB).
- `ActivateListVersion { }` (If-Match carries the version's `version`; body empty).
- `ListSettingProposal { enabled?, refresh_hours?, script_plan?, thresholds?, feed_source? { host, path, publisher }, reason_code, reason_text, effective_at? }` with the same optional-field style as the update DTOs elsewhere (`Option<T>`).
- `SearchRequest { name: String (1 to 200 characters), identifiers: Option<IdentifierInput>, list_codes: Vec<ListCode> (default all enabled), limit: u8 (at most 50) }`.
- `RunRequest { subject_kind, subject_id, reason_text? }`, `LookbackRequest { list_version_id }`.
- `ClaimRequest {}`; `DecisionRequest { outcome: DecisionOutcome, reason_codes: Vec<String> (1 to 5, family decision), text: Option<String> (at most 500 characters), remember_pair: bool, remember_expires_on: Option<NaiveDate> }`.
- `BulkDecisionRequest { alert_ids: Vec<Uuid> (1 to 50), outcome: Incorrect, reason_codes, text? }` (only `Incorrect`; the route has no other outcome).
- `PairMemoryProposal { alert_id, reason_codes, expires_on }`; `GoodGuyProposal { list_codes: Vec<ListCode>, rule: GoodGuyRuleInput, reason_codes, expires_on, reason_text }`; `GoodGuyRevoke { action, rule_id, reason_codes, reason_text }` (the revoke body of `POST /screening/good-guys`).
- `HarnessRunRequest { list_version_id?, kind: gate | oracle | canary }`.

### 2.4 Repository traits (domain layer; `Pg*` implementations in `db`)

```rust
#[async_trait]
pub trait ListRepository: Send + Sync {
    async fn lists(&self, conn: &mut PgConnection) -> Result<Vec<ListRecord>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, code: ListCode) -> Result<Option<ListRecord>, RepoError>;
    async fn versions(&self, conn: &mut PgConnection, list: Uuid, f: &VersionFilter, page: Keyset) -> Result<Page<ListVersion>, RepoError>;
    async fn version(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<ListVersion>, RepoError>;
    async fn active_versions(&self, conn: &mut PgConnection) -> Result<Vec<ListVersion>, RepoError>;   // the index input
    async fn insert_version(&self, tx: &mut Tx, v: &NewListVersion) -> Result<ListVersion, RepoError>;  // status loading, seq = max + 1
    /// Copy-in of entries and names for a loading version (COPY, batches of 5,000); guard trigger checks the status.
    async fn write_entries(&self, tx: &mut Tx, version: Uuid, entries: &[NewEntry]) -> Result<u64, RepoError>;
    async fn set_status(&self, tx: &mut Tx, version: Uuid, expected: i32, change: &VersionChange) -> Result<ListVersion, RepoError>;   // If-Match on version
    async fn activate(&self, tx: &mut Tx, list: Uuid, version: Uuid, by: Option<Uuid>) -> Result<Vec<Uuid>, RepoError>;   // returns the superseded version id
    async fn diff_counts(&self, conn: &mut PgConnection, old: Uuid, new: Uuid) -> Result<DiffCounts, RepoError>;
    async fn entry_fingerprints(&self, conn: &mut PgConnection, version: Uuid) -> Result<Vec<(String, [u8; 32])>, RepoError>;   // (entry_key, entry_fp)
    async fn entry_detail(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<EntryDetail>, RepoError>;
}

#[async_trait]
pub trait AlertRepository: Send + Sync {
    /// INSERT ... ON CONFLICT (fingerprint) DO NOTHING; returns the ids actually inserted (section 3.5).
    async fn insert_candidates(&self, tx: &mut Tx, rows: &[NewAlert]) -> Result<Vec<Uuid>, RepoError>;
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<ScreeningAlert>, RepoError>;
    async fn get_for_update(&self, tx: &mut Tx, id: Uuid) -> Result<Option<ScreeningAlert>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &AlertFilter, page: Keyset) -> Result<Page<ScreeningAlert>, RepoError>;   // no total
    async fn claim(&self, tx: &mut Tx, id: Uuid, by: Uuid, expected: i32) -> Result<ScreeningAlert, RepoError>;
    async fn apply_state(&self, tx: &mut Tx, id: Uuid, expected: i32, to: AlertState, decided: bool) -> Result<ScreeningAlert, RepoError>;
    async fn supersede(&self, tx: &mut Tx, old: &[Uuid], by: Uuid) -> Result<u64, RepoError>;
    async fn schedule_clock(&self, tx: &mut Tx, id: Uuid, timer: Uuid) -> Result<(), RepoError>;
    async fn counts_for(&self, conn: &mut PgConnection, principal: Uuid) -> Result<AlertCounts, RepoError>;   // SSE queue.counts, filtered
}

#[async_trait]
pub trait DecisionRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, d: &NewDecision) -> Result<ScreeningDecision, RepoError>;   // seq = max + 1
    async fn history(&self, conn: &mut PgConnection, alert: Uuid) -> Result<Vec<ScreeningDecision>, RepoError>;
}

#[async_trait]
pub trait PairMemoryRepository: Send + Sync {
    async fn propose(&self, tx: &mut Tx, p: &NewPairMemory) -> Result<PairMemory, RepoError>;   // partial unique: one open row per pair
    async fn approve(&self, tx: &mut Tx, id: Uuid, by: Uuid, expected: i32, expires: DateTime<Utc>) -> Result<PairMemory, RepoError>;
    async fn active_for(&self, conn: &mut PgConnection, subject_kind: SubjectKind, subject: Uuid, entry: Uuid) -> Result<Option<PairMemory>, RepoError>;
    async fn invalidate(&self, tx: &mut Tx, ids: &[Uuid], reason: PairReason) -> Result<Vec<Uuid>, RepoError>;
    async fn active_for_entries(&self, conn: &mut PgConnection, entries: &[Uuid]) -> Result<Vec<PairMemory>, RepoError>;
    async fn expire_due(&self, tx: &mut Tx, now: DateTime<Utc>) -> Result<Vec<Uuid>, RepoError>;
    async fn record_suppressions(&self, tx: &mut Tx, counts: &[(Uuid, u64)], at: DateTime<Utc>) -> Result<(), RepoError>;
    async fn page(&self, conn: &mut PgConnection, f: &PairFilter, page: Keyset) -> Result<Page<PairMemory>, RepoError>;
}

#[async_trait]
pub trait GoodGuyRepository: Send + Sync {
    async fn insert_on_activation(&self, tx: &mut Tx, r: &NewGoodGuy) -> Result<GoodGuyRule, RepoError>;   // from the CR activation only
    async fn active_for_list(&self, conn: &mut PgConnection, list: ListCode) -> Result<Vec<GoodGuyRule>, RepoError>;
    async fn close_expired(&self, tx: &mut Tx, now: DateTime<Utc>) -> Result<u64, RepoError>;
}

#[async_trait]
pub trait HarnessRepository: Send + Sync {
    async fn start(&self, tx: &mut Tx, r: &NewHarnessRun) -> Result<HarnessRun, RepoError>;
    async fn finish(&self, tx: &mut Tx, id: Uuid, r: &HarnessResult) -> Result<HarnessRun, RepoError>;
    async fn latest_gate(&self, conn: &mut PgConnection, version: Uuid) -> Result<Option<HarnessRun>, RepoError>;
    async fn runs(&self, conn: &mut PgConnection, f: &HarnessFilter, page: Keyset) -> Result<Page<HarnessRun>, RepoError>;
}
```

### 2.5 Ports (domain traits; implementations in the crates named)

```rust
/// The one freeze hook. Phase 14 ships RecordingFreezeTrigger; phase 20 replaces it in AppState with FreezeService.
#[async_trait]
pub trait FreezeTrigger: Send + Sync {
    async fn start(&self, tx: &mut Tx, c: &FreezeCandidate) -> Result<FreezeStart, FreezeError>;
}
pub struct FreezeCandidate { pub alert_id: Uuid, pub subject_kind: SubjectKind, pub subject_id: Uuid, pub list: ListCode,
                             pub entry_key: String, pub decided_by: Uuid, pub first_decider: Uuid, pub decided_at: DateTime<Utc> }
pub enum FreezeStart { Recorded, Started { freeze_id: Uuid } }

/// Feed fetch through the bank's declared egress proxy only (phase 07 pattern; network z2_egress on akr-worker).
#[async_trait]
pub trait ListFeedSource: Send + Sync {
    async fn fetch(&self, list: &ListRecord, prior_etag: Option<&str>) -> Result<FeedFetch, FeedError>;   // NotModified is a result, not an error
}
pub enum FeedFetch { NotModified, Body { bytes: Bytes, etag: Option<String>, published_at: Option<DateTime<Utc>>, expected_count: Option<u32> } }

/// The bank list read (akr-ingestd only): one approved object through SourceReader and SelectProof (phase 05).
#[async_trait]
pub trait BankListReader: Send + Sync {
    async fn snapshot(&self, source: Uuid, object: &str) -> Result<Vec<BankListRow>, SourceError>;
}

/// Canonical subjects for the runs (phase 10 tables; read only). Bi-temporal rows: current versions only.
#[async_trait]
pub trait SubjectSource: Send + Sync {
    async fn parties_keyset(&self, conn: &mut PgConnection, after: Uuid, limit: u32) -> Result<Vec<SubjectRow>, RepoError>;
    async fn changed_parties(&self, conn: &mut PgConnection, batch: Uuid) -> Result<Vec<SubjectRow>, RepoError>;
    async fn new_payment_parties(&self, conn: &mut PgConnection, since: DateTime<Utc>, limit: u32) -> Result<Vec<SubjectRow>, RepoError>;
    async fn names_for(&self, conn: &mut PgConnection, subject: Uuid) -> Result<Vec<NameInput>, RepoError>;   // legal first, then aliases; quality flags
}
```

`BusinessClock` (phase 09) gives the alert clock its fire time in hours; `TimerService` (phase 00) schedules and cancels it; `EvidenceStore` (phase 02) stages the list file and `open` serves it to the parser. `SettingsService` (phase 00) reads every `screening.*` and `schedule.*` key; a settings change to a governed key arrives only through a change request (section 3.10).

### 2.6 Services (named once; their methods are in section 3)

`ListService` (versions, upload, activation, diff), `ListFetchService` (worker), `BankListService` (ingestd), `ScreeningIndexRegistry` (one `ArcSwap` per process holding the active index per list version), `ScreeningService` (runs, forward and reverse), `AlertService` (claim, decide, bulk decide), `PairMemoryService`, `GoodGuyService`, `KnownAnswerHarness`, `ListSettingTarget`, `ScreeningPolicyTarget`, `GoodGuyTarget` (the three change targets of section 3.10; `HarnessCaseService` and the fourth target `HarnessCaseTarget` are in section 7.19), `RecordingFreezeTrigger`, `SubjectFingerprinter`.

### 2.7 utoipa schemas

`ListRecordView`, `ListVersionView`, `ListEntryDetailView` (SS and M only), `ListDiffView`, `ScreeningRunView`, `ScreeningAlertView` (with `affordances`), `ScreeningAlertDetailView` (aligned parts, folds, identifiers compared, frequency context omitted in v1), `ScreeningDecisionView`, `PairMemoryView`, `GoodGuyRuleView`, `HarnessRunView`, `SearchResponse`, `AlertCountsView`, `ProblemCode` (existing enum). Every `Alert*` and `PairMemory*` view is documented with `Cache-Control: no-store`.

### 2.8 Route table

Scopes come from API.md section 2. Markers as API.md section 1: **SU** step-up, **4E** four-eyes, **CR** change request. Read scopes for the list status admit every role Cedar admits to `screening_list.read` (section 3.11): `config.author` (CFG, DS, MRO), `config.approve` (CHK, M, CH, MRO), `audit.read` (IA, SEC, M), `assessor.read` (AS, IA), `screening.work` (SS, M).

| Method | Path | Scope | Description (cedar action) |
|-|-|-|-|
| GET | `/lists` | `screening.work, config.author, config.approve, audit.read, assessor.read` | Lists with kind, enabled, refresh, active version, `age_hours` and `stale` (`screening_list.read`) |
| GET | `/lists/{id}` | same | Detail: thresholds, script plan, feed description, next due fetch (`screening_list.read`) |
| GET | `/lists/{id}/versions` | same | Versions with status, counts, checksum, failure code, origin (`screening_list.read`) |
| POST | `/lists/{id}/versions` | `screening.work` | Upload a national or bank list file (multipart); **SU** (class `approval`); 202 (`screening_list.upload`) |
| GET | `/lists/{id}/versions/{v}/diff` | same read scopes | Additions, removals and amendments by counts; entry rows for SS and M only (`screening_list.read`) |
| POST | `/lists/{id}/versions/{v}/activate` | `screening.work` | Activate a validated version; **SU** (class `approval`); **4E** for `upload` and `cbs_snapshot`; `If-Match` (`screening_list.activate`) |
| POST | `/lists/{id}/fetches` | `screening.work` | Fetch now (feed) or read the bank snapshot now (`bank`); 202 (`screening_list.fetch`) |
| PUT | `/lists/{id}/settings` | `config.author` | Propose list settings; **CR** `list_setting`; `If-Match`; 202 (`screening_list.configure`) |
| GET | `/screening/runs` | `screening.work` | Runs with kind, counts, degraded flags (`screening.run.read`) |
| POST | `/screening/runs` | `screening.work` | Adhoc run for one subject; purpose code; 202 (`screening.run`) |
| POST | `/screening/lookbacks` | `screening.work` | Re-run the look-back of a validated or active version; 202 (`screening.lookback`) |
| POST | `/screening/search` | `screening.work` | Name search; purpose code required; rate-limited; audited, text never stored (`screening.search`) |
| GET | `/screening/alerts` | `screening.work` | Alert queue; masked rows filtered; no total (`screening.alert.read`) |
| GET | `/screening/alerts/{id}` | `screening.work` | Alert with aligned parts, identifiers, folds and history; masked kind (`screening.alert.read`) |
| POST | `/screening/alerts/{id}/claim` | `screening.work` | Claim; `If-Match` (`screening.alert.claim`) |
| POST | `/screening/alerts/{id}/decision` | `screening.work` | True, potential or incorrect with reasons; **SU** (class `decision`); **4E** for freeze true; `If-Match` (`screening.alert.decide`) |
| POST | `/screening/alerts/bulk-decision` | `screening.work` | Incorrect on up to 50 alerts of identical evidence, never freeze lists; **SU** (class `decision`) (`screening.alert.bulk_decide`) |
| GET | `/screening/pair-memory` | `screening.work` | Remembered pairs with status and expiry; masked kind (`screening.pair.read`) |
| POST | `/screening/pair-memory` | `screening.work` | Propose a remembered pair from an incorrect alert; **4E** on approval; 201 (`screening.pair.propose`) |
| POST | `/screening/pair-memory/{id}/approve` | `screening.work` | Approve a proposal; **SU** (class `decision`); **4E**; `If-Match` (`screening.pair.approve`) |
| DELETE | `/screening/pair-memory/{id}` | `screening.work` | Invalidate (status `invalidated`, row kept); `Idempotency-Key`, `If-Match` (`screening.pair.invalidate`) |
| GET | `/screening/good-guys` | `screening.work, config.approve, audit.read` | Good-guy rules with expiry and suppression counts (`screening.goodguy.read`) |
| POST | `/screening/good-guys` | `screening.work` | Propose a rule; **CR** `good_guy_rule`; 202 (`screening.goodguy.propose`) |
| GET | `/screening/harness-runs` | `screening.work, config.author, config.approve, audit.read, assessor.read` | Known-answer runs per list version (`screening.harness.read`) |
| POST | `/screening/harness-runs` | `screening.work` | Run the harness (gate, oracle or canary); 202 (`screening.harness.run`) |

The route count is 25 (27 with the two routes of section 7.19). The table adds seven rows to API.md 3.9 (`GET /lists/{id}`, `POST /lists/{id}/versions/{v}/activate`, `POST /lists/{id}/fetches`, `PUT /lists/{id}/settings`, `POST /screening/runs`, `POST /screening/alerts/bulk-decision`, `POST /screening/pair-memory/{id}/approve`); the lead aligns API.md 3.9 in the same change (Proposed decisions).

### 2.9 Router

Two routers, one per module, each `pub fn router() -> Router<AppState>`:

```rust
// api/src/routes/lists.rs
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/lists", get(list_lists))
        .route("/lists/{id}", get(get_list))
        .route("/lists/{id}/settings", put(put_list_settings))          // CR list_setting proposal
        .route("/lists/{id}/versions", get(list_versions).post(upload_version))
        .route("/lists/{id}/versions/{v}/diff", get(version_diff))
        .route("/lists/{id}/versions/{v}/activate", post(activate_version))
        .route("/lists/{id}/fetches", post(fetch_now))
}

// api/src/routes/screening.rs
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/screening/runs", get(list_runs).post(adhoc_run))
        .route("/screening/lookbacks", post(start_lookback))
        .route("/screening/search", post(search))
        .route("/screening/alerts", get(list_alerts))
        .route("/screening/alerts/{id}", get(get_alert))
        .route("/screening/alerts/{id}/claim", post(claim_alert))
        .route("/screening/alerts/{id}/decision", post(decide_alert))
        .route("/screening/alerts/bulk-decision", post(bulk_decide))
        .route("/screening/pair-memory", get(list_pairs).post(propose_pair))
        .route("/screening/pair-memory/{id}", delete(invalidate_pair))
        .route("/screening/pair-memory/{id}/approve", post(approve_pair))
        .route("/screening/good-guys", get(list_good_guys).post(propose_good_guy))
        .route("/screening/harness-runs", get(list_harness).post(run_harness))
}
```

---

## 3. Business Logic

Coverage of N-01 to N-59 by section: N-01 to N-11, N-13 to N-17, N-46, N-47 and N-59 in 3.1 and 3.3; N-12 and N-28 in 3.1; N-18 to N-27 and N-34 in 3.4; N-19 and N-20 in 3.2 and 3.4; N-21 to N-23 in 3.2; N-24 in the open doubts; N-29 and N-30 in 3.3; N-31 and N-33 in 3.10 and 3.12; N-32, N-37, N-38 and N-40 to N-44 in 3.9; N-35 and N-36 in 3.7 and 3.8; N-39 in 3.9; N-45 in 3.2; N-48 in 4.4; N-49 to N-52 in 3.3 and 3.9; N-53 in 1.1; N-54 in 3.5; N-55 in 1.2 and 3.9; N-56 and N-57 in 3.2; N-58 in 3.5.

### 3.1 Name normalisation (`names` crate; N-01 to N-17, N-46, N-47, N-59)

`NameNormaliser::normalise(raw: &str, ctx: NameContext) -> NameForms` is pure and deterministic. `NameContext` carries `side` (`Party`, `Counterparty`, `ListEntry`), `source_field` and `truncation_cap` (35 for SWIFT line fields, 140 for ISO 20022 text, the source byte cap for CBS fields). The version constants are `NORMALISER_VERSION = "names-1.0.0"`, `LEXICON_VERSION = "lex-1.0.0"`, `KEY_VERSION = "k1"`, `UNICODE_VERSION = "16.0"` (pinned; a bump is a major version, section 3.9). The fold penalty is 0.03 per name (N-08; starting value inside the 0.02 to 0.05 range of research 22 section 6.4).

**Script detection** (before N1 and L1): a name with letters of both the Arabic and Latin blocks is `mixed` and is scored per script; a name in the Arabic block containing a Kurdish-specific letter (ۆ, ێ, ە, ڕ, ڵ, ڤ, ھ, ۇ, ۊ, ۋ, ژ, گ) is `ku`; otherwise `ar`, `lat` or `other`. The detected script selects the fold table (Persian and Kurdish letters apply to `ar` and `ku` alike) and the script plan of the list.

**Arabic steps (closed; each step has a rule id used in `folds` and in the golden file):**

| Rule | Operation | Note |
|-|-|-|
| N1 | Validate UTF-8, keep `raw`, trim | `raw` is never changed |
| N2 | NFKC; U+FDFC maps to the Arabic-yeh form of rial, U+FDFD to a space, then NFKC | research 22 section 6.2 |
| N3 | Remove Cf (ZWNJ, ZWJ, LRM, RLM, embeddings, isolates, BOM, ALM); Zs and controls to space; collapse spaces | |
| N4 | Remove tatweel U+0640; NFD and remove Mn (harakat, dagger alef, Quranic marks); `light` stops here (hamza kept) | |
| N5 | أ إ آ ٱ to ا; ؤ to و; ئ to ي; standalone ء dropped in `folded` only | `light` keeps ء and hamza seats |
| N6 | The letter table of N5 and N6 together (28 pairs: ة to ه; ى to ي; ی ێ ې ے to ي; ک to ك; ە ھ ہ to ه; ۆ ۇ ۊ ۋ to و; ڕ ڑ to ر; ڵ to ل; ڤ to ف; پ to ب; چ to ج; ژ to ز; گ to ك, with أ إ آ ٱ to ا, ؤ to و, ئ to ي); each applied fold adds to `folds` and the penalty | the same table as `ref.name_key_k1` step k1.6 |
| N7 | Eastern digits to ASCII; Arabic comma and punctuation to space | |
| N8 | Alias segments after المكنى, الملقب, المعروف, المدعو or a parenthesis become alias fields (`alias_class` kunya or weak by the list policy) | N-02 |
| N9 | Tokens classified: title, particle (بن, ابن, بنت, آل), kunya (أبو, ام), compound, article | lexicon `lex-1.0.0` |
| N10 | Strip the article ال from tokens longer than three letters (never from الله); join عبد + second element, al-Din compounds and kunya compounds from the closed lexicon | N-10 |
| N11 | Emit typed parts (`PartRole`), the four forms and the flags | matronymic chains (bar, beth, bint) are part of the chain (N-04) |

**Latin steps** (research 22 section 7.2; the L1 table is the same set the SQL key uses):

| Rule | Operation |
|-|-|
| L1 | NFKD, strip Mn; the closed table æ to ae, œ to oe, þ to th, ð to dh, ß to ss, ø to o, ł to l, đ to d, ħ to h, ŋ to n, ı to i; casefold; İ to i |
| L2 | Remove ʻ ʿ ʼ ʾ ' ’ and record the flag `ayn_marked`; the key maps these six to E (other marks are punctuation) |
| L3 | Hyphens, dots and commas to space; a comma marks family-name-first order (`order_flag`) |
| L4 | Titles removed: mr, mrs, dr, eng, sayyid, sayed, seyyed, sheikh, hajj, hajji, haj, mulla; suffix jr |
| L5 | Nasab markers (bin, ibn, ben, bint, binti, ould, walad) become particles with weight 0 |
| L6 | Article particles (al, el, ul, ar, as, ad, an, ash, at, az, adh, ath) stripped when followed by three or more letters |
| L7 | Abd compounds joined with the next token (abd, abdu, abdul, abdel, abdal, abdur, abder); the second element mapped through the compound lexicon |
| L8 | Kunya markers (abu, abou, abo, abi, umm, um, oum) and the next token go to the kunya field |
| L9 | Arabizi digits inside a token that has letters: 2 and 3 to E, 5 to X, 6 to T, 7 to H, 8 to G, 9 to Q (names fields only) |
| L10 | Emit the k1 key of each token and of the whole name, and the block key |

**Lexicons** (`backend/packs/names/lexicon.v1.json`, versioned as `lex-1.0.0`, reviewed by speakers of Kurdish, Turkmen, Syriac and Armenian names before use, research 22 section 14.3): tribes and families with variants; Abd and al-Din compounds including the Shia forms; titles, particles and kunya patterns; the given-name lexicon of about 3,000 names with Arabic spellings, Latin variants and keys (N-17, used for back-transliteration and as a test oracle). Entities use a legal-form and generic-word lexicon in Arabic and Latin with low weights, exact numbers and exact vessel identifiers (N-47). The lexicon never contains a religion, sect or ethnic label; name origin selects only fold and transliteration rules (N-05, recorded in the model-governance document of phase 27).

**Key `k1`** (N-13, N-14) is the strict consonant key (`ref.name_key_k1`, section 1.1) and **block key** is `ref.name_key_block_k1` (Q, G and K as one; V as T; U as D; W, Y and E dropped). Each token and the whole name have both. The substitution costs of research 22 section 7.4 are versioned in `lex-1.0.0` and re-estimated per bank under data governance. The Rust function is the reference implementation; the SQL twin must agree on the parity corpus of section 6, check 3. A Latin name is never compared with a Latin name by a phonetic key (N-13, N-16); Soundex and Metaphone are not used.

**Corpus and golden files** (N-06, N-38; cross-checked in the test pipeline only against nomenklatura and CAMeL Tools output, N-43): `backend/packs/names/parity-corpus.v1.jsonl` holds at least 300 cases (`raw`, `light`, `folded`, `key`, `block_key`, `folds`) covering every rule above, Kurdish ZWNJ repair (N-09), İ, Eastern digits, presentation forms and the research 22 section 6.1 pairs. The same file is loaded by the Rust test, by the SQL parity test and by the frontend `fold` test of phase 08 (section 5). Idempotence holds: `normalise(normalise(x).folded) == normalise(x).folded`.

**Name quality and profile** (N-12): the first `daily_base` run samples up to 20,000 names and writes the per-bank profile (share of presentation forms, ZWNJ, Persian and Kurdish letters, double spaces, parentheses, Latin letters in Arabic fields, Eastern digits, compound forms, hamza and ya variants) into `alert.screening_runs.checkpoint.name_profile`; S3 shows it. The profile sets nothing automatically; it is evidence for the threshold review.

**Frequency context** (N-28): token and chain frequencies come from the bank's own customer base, recomputed monthly from `canon.party_name` current rows, with the national-list tables as prior. They feed the explanation and the IDF switch (off by default, research 22 section 12.4).

### 3.2 List registry, feeds, uploads and the bank list (`ListService`, `ListFetchService`, `BankListService`; N-31, N-45, N-56, N-57, N-58)

**Freshness rule** (`ListRecord::stale`): `age_hours = now − active.activated_at` (or the last successful `loaded_at` for a `cbs_snapshot` list); a list is stale when `age_hours > min(refresh_hours, screening.list_max_age_h)`. A freeze list cannot be configured above 12 hours (the table CHECK) and `screening.list_max_age_h` cannot be raised above 12 (422 `E_VALIDATION`, `list_age_above_floor`). A stale list writes the audit event `list.stale` once per transition, raises the metric `akr_list_stale{list}` and marks every new run `degraded_flags += list_stale`; it never removes the list from screening (the last good version stays active, MODULES M9 edge case).

**`list_fetch` (akr-worker; schedule `schedule.list_stale_sweep`, on demand from `POST /lists/{id}/fetches`):**
1. Load the list. `upload_only` lists: no fetch; the sweep only raises the stale alarm.
2. Call `ListFeedSource::fetch(list, prior_etag)` through `AKR_EGRESS_PROXY` (phase 07 pattern; `z2_egress`; host and path from `source_ref`, allow-listed, size at most `screening.feed_max_bytes`). `NotModified` writes `list.feed_unchanged` and stops. A transport failure writes `list.feed_failed {list, status, reason}`, sets `feed_unreachable` on the next run, and stops: the last good version stays.
3. A body: compute the SHA-256; in one transaction call `EvidenceStore::stage` (owner kind `list_version`, kind `upload`, retention R7), insert the `ref.list_versions` row (`origin = feed`, `requested_by` null, `file_sha256`, `evidence_id`, `status = loading`), audit `list.fetched {list, version, bytes}`, and enqueue `list_parse`.

**Upload (`POST /lists/{id}/versions`, SU class `approval`):** size check (`E_PAYLOAD_TOO_LARGE`); `EvidenceStore::stage` (AV scan, type sniffing, the phase 02 rules); insert the version with `origin = upload`, `requested_by = principal`, `acknowledged_count_drop` from the body; audit `list.version_uploaded`; enqueue `list_parse`; 202 with `Location`. Only `iq_national` and `bank` accept uploads in this phase (other lists are fed).

**`list_parse` (akr-worker; payload `{version_id}`):**
1. If the evidence row is `pending` in the AV scan, re-enqueue with a 60-second delay. `rejected` or `infected` sets `failed` with `av_rejected`.
2. Open the evidence object; verify the SHA-256 against `file_sha256` (mismatch sets `failed` with `parse_error` and an incident). The XML parsers refuse DOCTYPE, entity references and processing instructions before any tree is built (the DR-70 pre-screen rules).
3. Parse by the closed `feed_format` set. `iq_office_feed` is a reserved name whose parser is written when the Office defines its feed (open doubt); until then `iq_xlsx_export` is the stop-gap (N-45).
4. Reconcile: parsed rows against `expected_count` (the publisher's own count where the format has one; otherwise null and the count-drop rule alone applies, with the warning `no_publisher_count` in `diff_summary`). Identical rows (same `entry_key` and `row_hash`) are removed and counted in `duplicate_count`. Same `entry_key` with different content is `conflict_count`; any conflict fails the version with `conflicting_duplicates` (zero tolerance; the bank decides by a new file).
5. Count drop: if the distinct count is below the active version's `record_count` by more than `screening.max_count_drop_pct` percent and `acknowledged_count_drop` is false, fail with `count_drop` (research 22 section 6.5: a silent fall is the defect to catch).
6. Normalise every name (3.1); UN alias quality and OFAC's weak-alias criteria set `alias_class` (N-27, N-46); parse identifiers (birth dates with precision and default flags N-19, documents by type with exact keyed hashes N-21, mother's chain N-20, nationality, gender from the lexicon N-23, place from the gazetteer N-22 as soft evidence), compute `entry_fp` over the identifying fields (names with aliases, birth, documents, nationality, mother chain, program) and `row_hash`.
7. One transaction: `write_entries` (batches of 5,000), set `record_count`, `checksum` (SHA-256 over the sorted pairs of `entry_key` and `row_hash`), `diff_summary` (added, removed and amended counts against the active version by `entry_key` and `entry_fp`), `loaded_at`. Enqueue `harness_run`. The version stays `loading` until the gate passes.

**`bank` list (akr-ingestd job `ingest_bank_list`; payload `{list_id}`; schedule none, on demand or by the bank's own rule):** the list's `source_ref` names an approved object of an active source (phase 05 `source_access` list). `BankListReader::snapshot` reads it through `SourceReader` with `SelectProof` (one statement, one approved object, the phase 05 load budget). Rows map through the approved mapping of the bank's own lists (the bank's column names, a bank-reviewed mapping document; no default for any column, N-57 variant table is a later governed object). The job writes a `cbs_snapshot` version with `source_as_of` (the CBS business date of the read, DR-39) in `diff_summary`, no evidence object, the rows and the checksum, and enqueues `harness_run`. The first version of the bank list requires the bank's review: its activation is a 4E step (`list_activate`, above). The discovery run of phase 05 supplies the fields, name keyboards and code lists that the mapping uses (N-56). A CBS snapshot changes the bank's list only through a new version; a removed row closes the pair memory of that entry (3.7).

**`list_activate` and `POST …/activate`** (4E for `upload` and `cbs_snapshot`): (1) the version is `validated` and its latest gate run passed (else 409 `E_NOT_ALLOWED_IN_STATE` or 422 `harness_not_passed`); (2) `If-Match` matches; (3) one transaction: `ListRepository::activate` moves the previous `active` version to `superseded` (`superseded_at`) first, so the partial unique index `uq_list_versions_active` holds at every statement, and then the new one to `active` (`activated_at`, `activated_by`, `lists.active_version_id`, version + 1); pair-memory rows are invalidated for every entry whose `entry_fp` moved or that left the version (`entry_changed`, 3.7); `screening_lookback` is queued; audit `list.activated {list, version, previous, diff_summary}`; (4) after commit, `NOTIFY akr_screening_index` makes each process build the new index and run its canary (3.9) before the swap. A `feed` version with `feed_auto_activate` on runs steps 1, 3 and 4 from `list_parse`'s gate result with no person as actor.

**Diff** (`GET /lists/{id}/versions/{v}/diff`): from `diff_summary` (counts) and, for SS and M only, the entry rows of the three classes: added (`entry_key` new), removed (old key absent), amended (same key, `entry_fp` differs).

### 3.3 Immutable index per version (`ScreeningIndex`; N-15, N-29, N-30, N-49, N-51)

- **Build**: from the rows of one `active` or `validated` version, in the process that needs it (akr-api, akr-worker, later akr-assess). Only the forms of the list's `script_plan` are indexed (N-15). Entries are sorted by `entry_key`; names are indexed under their folded tokens with document frequency, under each token's k1 key (for cross-script queries), under the whole block key, and birth years in buckets. Mother names and documents are decrypted into memory for the build only and zeroised after it (the `zeroize` crate).
- **Immutability and swap** (N-49): the index is never mutated. `ScreeningIndexRegistry` holds `ArcSwap<HashMap<Uuid, Arc<ScreeningIndex>>>`; a new version is built off the query path, a known-hit canary runs on it (N-52, 3.9), then it is swapped atomically. A `LISTEN akr_screening_index` notification triggers the rebuild in every process after `list_activate` commits.
- **Candidates** (N-29, N-30): query tokens are sorted by posting size; the two rarest intersect (galloping when the sizes differ by more than 32 times); one deletion expands tokens of 6 to 8 letters and two expand 9 or more (inline mode; batch mode uses 4 to 7 and 8 or more). Key-index expansion for Latin tokens keeps neighbours at 0.75 or more. The block key adds whole-name candidates. A cap of 100,000 candidates sets `truncated` on the result; it is never silent.
- **Thin queries**: fewer than `thin_query_min_tokens` tokens, or only tokens above `common_token_ceiling` with no usable identifier, return `NeedsMoreData` before candidate generation (3.4).
- **Determinism**: the hash maps are lookup tables only; candidates are sorted by `entry_key` before scoring; ties are broken by `entry_key`.
- **Budget** (N-50): the inline target is p50 under 5 ms and p99 under 25 ms at 150,000 names; the request budget is 250 ms with a `partial` flag and the recorded fail-open or fail-closed policy of the caller (N-51; the assessment caller of phase 29 decides).

### 3.4 Scorer (`domain::screening::Scorer`, pure; N-18 to N-34)

`score(query: &NameForms, q_ids: &Identifiers, cand: &IndexedEntry, cfg: &ScoringConfig) -> ScoreResult`, numbered:
1. **Token similarity** `s`: 1.0 for equal folded tokens; 0.97 for listed lexicon variants (Hussein spellings, Abd compounds); otherwise `1 − edit/len(longer)` when the edit is inside the gate (batch: 0 edits up to 3 letters, 1 up to 7, 2 above; inline: 0 up to 5, 1 up to 8, 2 above); cross-script pairs use the weighted key similarity capped at 0.95; each name that needed a fold loses 0.03 (once per name, not per token).
2. **Alignment**: one-to-one maximising `Σ s·w` (Hungarian for up to eight tokens, greedy above); a given name aligned to a tribe token takes a 0.02 tie penalty (roles are soft, N-25).
3. **Weights**: `w = role_weight` (given 1.0, father 0.9, grandfather 0.8, great-grandfather 0.7, tribe or nisba 0.9, kunya 0.5, laqab 0.5, title and particle 0; legal-form and generic entity tokens 0.1, a starting value, N-47). IDF is computed for evidence but does not weight the score (switch off, research 22 section 12.4).
4. **Name score** `= Σ s·w over aligned / (Σ aligned w + 0.8 · Σ unmatched query w + 0.2 · Σ unmatched list w)`; when the subject name is flagged truncated the unmatched list weight is 0 (research 22 section 13.11).
5. **Evidence bits** `E = Σ s · idf_bits(aligned token)`, plus identifier bits: birth year agrees +5; mother's full chain agrees +12 (first name only +8); a document number of the same type agrees +20. Contradictions carry 0 bits in this version (open doubt).
6. **Identifier score adjustments** (research 22 section 13.5; the mother's chain is aligned with the same aligner as the name, N-20): birth year 3 or more years apart −0.20, two years −0.08, one year −0.02; mother's chain agrees +0.08, disagrees −0.20; a document of the same type and number raises the score to at least 0.95 and escalates; same type, different number −0.30; nationality and place-of-birth contradictions −0.05 each; gender contradiction −0.10 unless the given name is unisex (both flagged unvalidated). A default date (1 January, 1 July) earns no credit (N-19).
7. **Minimum evidence** (N-26, N-27): at least two aligned tokens, or one whose `idf_bits` reach `doubt_policy.min_bits_single_token`; `E ≥ evidence_floor_bits`; a family or tribe token with one common given name never alerts alone; a kunya alone and a weak alias alone never alert. Weak aliases (`alias_class = weak`) cap their weight at `alias_weight_cap` and add evidence only from other fields; former names weigh `former_weight`.
8. **Threshold**: `with_identifiers` when a usable identifier is comparable on both sides (a birth year with precision year or better and not a default, a mother's chain, or a document), else `name_only`. Risk modifiers are additive and read phase 15 ratings; they are 0 in this phase (open doubt).
9. **Outcome**: `Hit` when score ≥ threshold, evidence ≥ floor and rule 7 passes; `BelowLine` when score lies in the review band below the threshold (counted and sampled, not an alert); `NeedsMoreData` for a thin query that has a candidate in the review band; `Clear` otherwise. A thin query with no candidate in the band is `not_assessable` in the run counts, never `Clear`.
10. **Output and explanation** (N-34): score rounded to four decimals, evidence to two; the result stores the algorithm, lexicon and normaliser versions and the threshold source (`name_only` or `with_identifiers`) so that the alert can be replayed (N-55). The explanation holds the aligned pairs with `s`, `w` and bits, the folds, the identifier checks (agree, differ or not comparable), the score components, the frequency context (N-28, when present) and a "would change if" sentence, computed by re-scoring with the deciding identifier removed; the server renders it in Arabic or English by `Accept-Language`.

### 3.5 Runs, subjects and alert creation (`ScreeningService`; N-54, N-58)

**Subjects and names.** A party is screened from its current `canon.party` row (type `person` or `entity`) and its `canon.party_name` rows: `legal` names first, then `alias` and `former` names with their alias class. A short field (any name type other than `legal` and `as_in_message`) is screened only when no full name exists; a name with `maybe_truncated` is screened with the truncation flag and never a lower score (DATA-MODEL s.9, phase 10 `quality_flags`). A multi-line SWIFT name is reassembled upstream by the phase 10 mapping and a name at 35 characters is flagged truncated (N-59). Counterparties come from `canon.counterparty.names`; payment parties from `canon.payment_message_party.name` (a null name is flagged and not screened). The forms are recomputed from `raw` with the current normaliser, never read from the stored canonical `key` columns (open doubt on phase 10).

**Run kinds** (each writes one `alert.screening_runs` row; `data_as_of` is the CBS business date of the newest mapped batch behind the subjects, DR-39; a stale source adds `stale_source` and never uses the server clock):
1. `daily_base` (`schedule.screening_daily_base`): keyset over party ids in chunks of 5,000, the checkpoint written after each chunk; every subject against every enabled active list.
2. `lookback` (queued by `list_activate`, or by `POST /screening/lookbacks`): the entries added or amended by the version, each queried against an in-memory index of all current subjects built once per run (chunks of `screening.lookback_chunk`); this is the designation look-back of MODULES M9 and research 22 section 13.10.
3. `onboarding` (queued from the phase 11 `canon.touched` event for each mapped batch; payload carries the batch id): the changed parties against every active list. A changed name is a re-screening trigger (N-58).
4. `payment` (`schedule.screening_payment`, every 15 minutes): payment parties and counterparties first seen since the last completed payment run (its checkpoint); post-event, phase 1 (research 22 section 13.6).
5. `adhoc` (`POST /screening/runs`, purpose code required): one subject against every active list; `requested_by` set.

**Alert creation** (for each candidate of outcome `Hit` or `NeedsMoreData`, and each `BelowLine` only as a counter):
1. Suppression first: an active pair-memory row for (subject, entry) with equal `subject_fp`, `entry_fp` and `matching_major` suppresses the candidate (3.7); an active good-guy rule for the list and the subject suppresses it (3.8). Both increment the run's `suppressed` count and the row's counter.
2. `fingerprint = SHA-256("sa1" ‖ subject_kind ‖ subject_id ‖ list_id ‖ entry_key ‖ subject_fp ‖ entry_fp ‖ matching_major)`. `subject_fp` covers the normalised name forms and identifiers the match used; `entry_fp` covers the list entry's identifying fields. Equal inputs give an equal fingerprint, so a rerun inserts nothing (`AlertRepository::insert_candidates`, section 2.4: `ON CONFLICT (fingerprint) DO NOTHING`).
3. Insert the new rows in one transaction. For each returned id: priority from `screening.priority_map` (`freeze_identifier` P0, `freeze_name_only` P1, `pep` P2, `internal` P3; `escalate_step` applies later), the alert clock (`TimerService::schedule` kind `screening_alert_clock`, `fire_at = BusinessClock::add(now, screening.review_clock_hours, Hours)`, owner `screening_alert`), the audit `screening.alert_created`.
4. An undecided alert for the same (subject, entry) with an older `subject_fp` is superseded (`superseded_by`, `superseded_at`); the queue hides it. A decided alert is never superseded.
5. Staged thresholds (N-54): the first `daily_base` after a list's first activation uses the identifier-backed threshold only; lowering the name-only threshold is a separate `list_setting` change request with its dry run (3.10).
6. `needs_more_data` is a state of an alert that exists; `not_assessable` is a run counter. A thin subject never produces `Clear`.

**Clock handler** (`timer_screening_alert_clock`; re-reads the alert and does nothing unless it is `open`, `potential` or `needs_more_data` and not superseded): raises the priority one step (never above P0), sets `escalated_at`, audit `screening.alert_escalated`, SSE `clock.warning` to SS and M only.

### 3.6 Decisions, claims and the freeze port (`AlertService`, `FreezeTrigger`; four-eyes)

**Transitions:**

| From | Outcome | To | Rule |
|-|-|-|-|
| `open`, `needs_more_data` | `incorrect` | `incorrect` | Reason codes required |
| `open`, `needs_more_data` | `potential` | `potential` | Escalates the priority one step |
| `open`, `needs_more_data` | `true` | `true` | Allowed off freeze lists; on a freeze list refused (422 `four_eyes_required`): a `potential` decision comes first |
| `potential` | `true` | `true` | On a freeze list the decider must not be a prior decider (Cedar forbid and the decision CHECK) |
| `potential` | `incorrect` | `incorrect` | Reason codes required |
| `true`, `incorrect`, or superseded | any | refused | 409 `E_NOT_ALLOWED_IN_STATE` (the terminal guard of migration 0048 enforces the first two) |

**Claim.** An unclaimed alert is claimed by the caller in the decision transaction; a claim held by another user is 409 `E_CLAIMED`. Claiming is `If-Match` on the alert version.

**Decide** (`POST /screening/alerts/{id}/decision`, SU class `decision`, resource `screening_alert:<id>`):
1. `StepUpService::consume(tx, ctx, header, StepUpAction::Decision, Some("screening_alert:<id>"))` (phase 01); 401 `E_STEPUP_REQUIRED` otherwise.
2. `get_for_update`; `If-Match` must equal `version` (428 or 409 otherwise).
3. Apply the transition table; refuse with 409 otherwise.
4. Cedar `screening.alert.decide` with `context.outcome` and the resource attributes `list_kind` and `excluded`. The forbid `screening.freeze_true_second_person` returns 403 `E_SELF_APPROVAL`. The delegated-authority matrix must also permit decision type `screening.decision` for the caller's roles at the alert's priority (phase 09 section 3.6; scope `screening.work`; roles SS and M).
5. Validate `reason_codes` against family `decision` (active codes of `ref.reason_codes`; phase 09) and `text` (at most 500 characters, sealed in `text_enc`).
6. Insert `alert.screening_decisions` (seq = the next number; `prior_decider_ids` = the earlier deciders; `purpose_code` from `X-Access-Purpose`).
7. `apply_state`: state, `decided_at`, claim fields, version + 1.
8. Outcome `true` on a freeze list: `FreezeTrigger::start(tx, FreezeCandidate { … })`. Phase 14 ships `RecordingFreezeTrigger`, which writes `screening.freeze_trigger_recorded` and returns `Recorded`; phase 20 replaces it in AppState (M14 freeze task, clocks and the Committee report). Outcome `true` off freeze lists: state `true`; the case hook of phases 17 and 18 reads it; PEP lists are read by phase 15.
9. Outcome `incorrect` with `remember_pair`: `PairMemoryService::propose` (3.7) and `remembered_pair_id` set on the decision.
10. `TimerService::cancel_for_owner(screening_alert, id, screening_alert_clock)`.
11. Audit `screening.alert_decided {alert, outcome, state_from, state_to, list_kind, reason_codes, decision_id}` (ids and codes only).

**Bulk incorrect** (`POST /screening/alerts/bulk-decision`, SU class `decision`): up to 50 alerts that share the same `entry_id` and the same identifier comparison signature (the same agree, differ and not-comparable results per identifier, and the same aligned-token count); the list is not a freeze list; every alert is decided as in steps 2 to 11 in one transaction, with one batch audit `screening.bulk_decided {count, entry_id}`. A mixed selection is refused for the whole batch (422 `E_VALIDATION`, `bulk_not_identical`), and so is a batch on a freeze list (422 `E_VALIDATION`, `freeze_list_excluded`, as in section 3.8).

### 3.7 Pair memory (`PairMemoryService`; N-35)

- **Propose** (`POST /screening/pair-memory`, 201, status `proposed`): the source alert must be `incorrect`. `expires_on` is at most `screening.pair_memory_max_months` months ahead (the PEP cap of 24 months is phase 15's). The partial unique index allows one open row per (subject, entry). Audit `pair.proposed`.
- **Approve** (`POST …/approve`, SU class `decision`, 4E): the approver is not the proposer (CHECK and Cedar `screening.no_self_approve_pair`); status `active`, `approved_at`. Audit `pair.approved`. Suppression applies from this commit.
- **Invalidate** (`DELETE` and the system paths; the row is kept, status `invalidated`, `closed_at` set): `subject_changed` (the subject's `subject_fp` moved, detected at onboarding or at the next run); `entry_changed` (list activation: the entry's `entry_fp` moved, or the designation left the active version); `matching_version` (a major bump); `expired` (sweep); `manual` (DELETE). Each writes `pair.invalidated {reason}`.
- **Drift monitor** (the "must be zero" rule of MODULES M9): after every activation and every daily run, count active rows whose current `entry_fp` or `subject_fp` differs from the stored value; the count is `akr_pair_memory_drift` and must be zero; a non-zero count is invalidated by the same transaction and raises an alarm.
- **Sweep** (`pair_memory_sweep`, daily): expires due rows; each Monday it draws a QA sample of 5 per cent of the week's suppressions, plus every suppression involving a freeze list, for S4. The sample creates no alert; a reviewer invalidates a row from S4.
- The memory is pair-keyed: it never propagates to another customer with the same name (research 22 section 13.8).

### 3.8 Good-guy rules (`GoodGuyService`, change target `good_guy_rule`; N-36)

- **Proposal** (`POST /screening/good-guys`, CR `good_guy_rule`): `list_codes` must not include a freeze list (422 `E_VALIDATION`, `freeze_list_excluded`: the service check runs first and is the answer of this route; the table CHECK repeats it at the insert; the Cedar forbid `screening.goodguy_no_freeze` is a third guard that matches only a `GoodGuyRule` resource, so it does not fire on this route, whose resource is System, and the policy tests alone exercise it); `expires_on` at most `screening.good_guy_max_days` ahead; `rule` is a name-pattern object (folded tokens that must all be present, and optional identifier conditions), never a single customer id.
- **Activation** (the CR approval, phase 04): one row in `alert.good_guy_rules` (`approved_by`, `change_request_id`, `expires_at`). A revoke is a CR proposed through the same route, `POST /screening/good-guys`, with the body `{"action": "revoke", "rule_id": …, "reason_codes": […], "reason_text": …}` (a second body type, `GoodGuyRevoke { action, rule_id, reason_codes, reason_text }`, which the route tells from `GoodGuyProposal` by the `action` field; the rule must be open, else 409 `E_NOT_ALLOWED_IN_STATE`; the checks of a new rule do not apply); it sets `closed_reason = revoked`. Nothing extends a rule silently.
- **Runtime**: applied before alert insert (3.5); the rule's `suppressions` count rises per run.
- **Report** (S4): count, age, approver, suppressions, and the number of rules whose tokens match any active freeze entry, which must be zero (research 22 section 12.2; the Habib Bank failure of research 14 section 12.2 is the reason for the rule).
- **Expiry**: `close_expired` (sweep) sets `closed_reason = expired`.

### 3.9 Known-answer harness (`KnownAnswerHarness`; N-37 to N-44, N-52)

- **Corpus**: `backend/packs/screening/known-answers.v1.jsonl`, synthetic and invented (never a real designated person; research 22 section 12.6, N-38). Three classes: `clean` (identical forms), `variant` (orthographic, reordered, truncated at 35 characters, one typo, four tokens), `hard_negative` (a family or tribe name alone, common names, near misses). Plus **self canaries**: up to `screening.harness_gate.self_canary_sample` entries of the candidate version, queried with their own names and identifiers.
- **Gate** (`kind = gate`, `harness_run` job, payload `{version_id}`): pass when `recall_clean ≥ 0.995` (N-39), `recall_variant ≥ 0.95` for cases with identifiers present, `hard_negative_alerts ≤ 0`, every self canary hits its own entry, and the names-crate golden test passes. Pass sets the version `validated`; fail sets `failed` with `harness_failed` and the failing case ids in `failures` (never names). Each run stores `corpus_sha256`, `config_version` and the report.
- **Below-the-line sampling** (N-41): each week one in twenty `BelowLine` candidates is written to the S3 report by ids; look-back over the replica is `POST /screening/lookbacks`.
- **Feedback** (N-44): every decided alert is a labelled pair as stored: its `alert.screening_decisions` rows, with outcome and reason codes, joined to the alert (ids, codes and scores only; no second store). A threshold review reads these tables; no route or report of this phase is added for it. The shipped corpus stays synthetic (N-38), so no real pair enters it. A bank adds its own known-answer cases, accepted by the MLRO through change control, in section 7.19 (owner decision of 2026-10-10, DR-415; this sentence replaced "it grows by release only, and a bank-side intake accepted by the MLRO and the bank-specific variant table (N-57) are P2"). The bank-specific variant table (N-57) stays P2.
- **Report** (all kinds): recall at each alert budget by variant class, the alert volume at each budget, the OFAC-emulation oracle difference (N-40, `kind = oracle`), the list lag (publication to activation), the release-to-release comparison with the previous run. Oracle runs never gate.
- **Start-up self-test** (N-52): the names golden file and the SQL twin canary (one key of each form) run at process start.
- **Snapshot canary** (`kind = canary`, N-52): on every index swap, five self canaries are queried; a miss keeps the previous index in that process, writes `list.index_canary_failed`, and raises the metric `akr_index_canary_failed`.
- **Shadow** (N-42): a candidate configuration runs as an `oracle` pass over live subjects, without alerts; the promotion is the change request, with the run as its evidence.
- **Versioning** (N-55): each run records the normaliser, lexicon and algorithm versions and the settings versions.


### 3.10 Governed configuration and change targets (N-31, N-33)

Settings are listed in section 1.5 and section 4.3. A governed key changes only through a change request (phase 04 `ChangeTargetRegistry`, DR-53). A fourth target, `harness_case`, is in section 7.19. The three targets of this section, each with reason family `change` (codes `CHG_*`; sanctions-driven changes take `CHG_SANCTIONS_LIST`):

| Target (`object_kind`) | Object key | Covers | Base class (route map DR-54) |
|-|-|-|-|
| `list_setting` | `list:<code>` | thresholds, `refresh_hours`, `script_plan`, `enabled`, `source_ref` | `enabled` true on a freeze list, a new `source_ref` and a raised `refresh_hours` are material; a lowered threshold is material; a raised threshold is minor. `emergency_allowed` for `enabled: true` only, reverted by setting `enabled` back |
| `screening_policy` | `screening_policy:<key>` (`doubt_policy`, `priority_map`, `harness_gate`) | the three governed JSON settings | Loosening is material; tightening is minor |
| `good_guy_rule` | `good_guy:<uuid>` | section 3.8 | Material (the rule suppresses, so `volume_effect` is `decrease`, and CH approves on the material route) |

`prepare` for the first two runs a dry run over a sample of at most 10,000 current subjects under the old and the new configuration and counts the alerts each would raise; `volume_effect` is `increase`, `neutral` or `decrease` from that count. An incomplete dry run is `unknown`, which can never be approved as material (DSL.md D.3). The dry run and its counts go into `impact` (`detection-logic`). `activate` writes the setting through `SettingsService` (governed write, audit `setting.updated` from phase 00) and, for a list, updates `ref.lists` with `version + 1`; a `revert` restores the previous value from the change request's `proposed` diff.

### 3.11 Cedar (`backend/policies/bundle/84_screening.cedar`)

Roles are read as `principal.roles` (phase 03 schema form; there is no Role entity). Entities added to `schema.cedarschema`: `ScreeningAlert { state: String, list_kind: String, excluded: Bool, superseded: Bool }`, `ListVersion { origin: String, status: String, requested_by_me: Bool }`, `PairMemory { list_kind: String, proposed_by_me: Bool }`, `GoodGuyRule { touches_freeze: Bool }`, `ScreeningList { kind: String }`. The resource adapters compute `excluded` (the caller is a prior decider of the alert, or acts for one) and the `*_by_me` flags per principal, as phase 04 computes `resource.excluded`. Step-up and purpose are not Cedar conditions; the handlers check them.

```cedar
// 84_screening.cedar (phase 14). Screening alerts are suspicion tier: the masking layer (phase 03) answers 404 to any
// caller outside these permits (the same body as a missing id).

@id("screening.work.ss") @role("SS")
permit (principal, action in [
        Akr::Action::"screening.run", Akr::Action::"screening.run.read", Akr::Action::"screening.lookback",
        Akr::Action::"screening.search", Akr::Action::"screening.alert.read", Akr::Action::"screening.alert.claim",
        Akr::Action::"screening.alert.decide", Akr::Action::"screening.alert.bulk_decide",
        Akr::Action::"screening.pair.read", Akr::Action::"screening.pair.propose", Akr::Action::"screening.pair.approve",
        Akr::Action::"screening.pair.invalidate", Akr::Action::"screening.goodguy.read",
        Akr::Action::"screening.goodguy.propose", Akr::Action::"screening.harness.run",
        Akr::Action::"screening.harness.read", Akr::Action::"screening_list.read",
        Akr::Action::"screening_list.upload", Akr::Action::"screening_list.activate",
        Akr::Action::"screening_list.fetch"], resource)
when { principal.roles.contains("SS") };

@id("screening.work.m") @role("M")
permit (principal, action in [
        Akr::Action::"screening.run", Akr::Action::"screening.run.read", Akr::Action::"screening.lookback",
        Akr::Action::"screening.search", Akr::Action::"screening.alert.read", Akr::Action::"screening.alert.claim",
        Akr::Action::"screening.alert.decide", Akr::Action::"screening.alert.bulk_decide",
        Akr::Action::"screening.pair.read", Akr::Action::"screening.pair.propose", Akr::Action::"screening.pair.approve",
        Akr::Action::"screening.pair.invalidate", Akr::Action::"screening.goodguy.read",
        Akr::Action::"screening.goodguy.propose", Akr::Action::"screening.harness.run",
        Akr::Action::"screening.harness.read", Akr::Action::"screening_list.read",
        Akr::Action::"screening_list.upload", Akr::Action::"screening_list.activate",
        Akr::Action::"screening_list.fetch"], resource)
when { principal.roles.contains("M") };

@id("screening.config_read") @role("CFG")
permit (principal, action in [Akr::Action::"screening_list.read", Akr::Action::"screening_list.configure",
        Akr::Action::"screening.harness.read"], resource)
when { principal.roles.contains("CFG") };

@id("screening.approver_read") @role("CHK")
permit (principal, action in [Akr::Action::"screening_list.read", Akr::Action::"screening.harness.read",
        Akr::Action::"screening.goodguy.read"], resource)
when { principal.roles.contains("CHK") || principal.roles.contains("CH") };

@id("screening.audit_read") @role("IA")
permit (principal, action in [Akr::Action::"screening_list.read", Akr::Action::"screening.harness.read"], resource)
when { principal.roles.contains("IA") };

@id("screening.assessor_read") @role("AS")
permit (principal, action in [Akr::Action::"screening_list.read", Akr::Action::"screening.harness.read"], resource)
when { principal.roles.contains("AS") };

// Four-eyes. The request-time SoD baseline (phase 03, principal.sod_conflict) also applies.
@id("screening.no_self_activate") @deny("self_approval")
forbid (principal, action == Akr::Action::"screening_list.activate", resource is Akr::ListVersion)
when { resource.origin != "feed" && resource.requested_by_me };

@id("screening.no_self_approve_pair") @deny("self_approval")
forbid (principal, action == Akr::Action::"screening.pair.approve", resource is Akr::PairMemory)
when { resource.proposed_by_me };

@id("screening.freeze_true_second_person") @deny("self_approval")
forbid (principal, action == Akr::Action::"screening.alert.decide", resource is Akr::ScreeningAlert)
when { resource.list_kind == "freeze" && context.outcome == "true" && resource.excluded };

@id("screening.goodguy_no_freeze") @deny("freeze_list")
forbid (principal, action == Akr::Action::"screening.goodguy.propose", resource is Akr::GoodGuyRule)
when { resource.touches_freeze };
```

Every action above has an `x-akr-action` entry in OpenAPI (section 4.2), and every route of section 2.8 maps to one of them (the route-to-policy test of phase 08). The scopes admit every role these permits admit: `config.author` admits CFG (and DS and MRO, which Cedar refuses), `config.approve` admits CHK, CH and M, `audit.read` admits IA, `assessor.read` admits AS.

### 3.12 Name search (`POST /screening/search`; N-33, audited, rate-limited)

1. `X-Access-Purpose` absent: 422 `E_PURPOSE_REQUIRED`, recorded as a `deny` access event (DR-46).
2. Rate bucket `screening.search_per_min` per user (GCRA in Valkey, phase 01 limiter): 429 with `Retry-After`.
3. Normalise the query with the names crate; a thin query returns `not_assessable`, never `clear`.
4. Search the in-memory indexes of the enabled lists (3.3), score (3.4), return at most 50 candidates with the list-side fields, score, evidence, outcome and the list status (`active` version, `stale`).
5. The access event (`search_hit`, resource type `screening_search`, section 7+) is written before the body (DR-46). The audit event `screening.searched {purpose_code, result_count, lists, query_hk}` carries a keyed hash of the normalised tokens, never the query text. Response `Cache-Control: no-store`.

### 3.13 Audit events (one per mutation; payloads carry ids, counts and codes, never names)

| Event | On | Resource |
|-|-|-|
| `list.version_uploaded`, `list.fetched`, `list.feed_failed`, `list.feed_unchanged` | upload, fetch outcome | list_version or list |
| `list.parsed`, `list.failed` (with `failure_code`), `list.validated` | parse and gate outcome | list_version |
| `list.activated`, `list.superseded`, `list.stale`, `list.index_canary_failed` | activation, freshness, canary | list_version or list |
| `screening.run_started`, `screening.run_finished` | each run | screening_run |
| `screening.alert_created`, `screening.alert_escalated`, `screening.alert_superseded` | alert lifecycle | screening_alert |
| `screening.alert_claimed`, `screening.alert_decided`, `screening.bulk_decided` | decisions | screening_alert |
| `screening.freeze_trigger_recorded` | a true freeze decision reaches the port | screening_alert |
| `screening.searched` | every search | screening (search) |
| `pair.proposed`, `pair.approved`, `pair.invalidated`, `pair.suppressed`, `pair.expired` | memory | pair_memory |
| `good_guy.suppressed`, `good_guy.expired` | rules | good_guy_rule |
| `harness.run_completed` | gate, oracle and canary | list_version |
| `setting.updated`, `list_setting.changed`, `screening_policy.changed`, `good_guy_rule.activated` | CR activation | setting or list or rule |

### 3.14 Errors (`ScreeningError`, `thiserror`)

| Variant | Status and code |
|-|-|
| `Masked` (alerts and pair rows outside the policy, and missing ids) | 404 `E_NOT_FOUND`, the same body and timing floor (phase 03) |
| `Claimed`, `VersionConflict`, `NotAllowedInState`, `Superseded` | 409 `E_CLAIMED`, `E_VERSION_CONFLICT`, `E_NOT_ALLOWED_IN_STATE` |
| `StepUpRequired` | 401 `E_STEPUP_REQUIRED` |
| `SelfApproval`, `Forbidden`, `Scope` | 403 `E_SELF_APPROVAL`, `E_FORBIDDEN`, `E_SCOPE` |
| `PurposeRequired` | 422 `E_PURPOSE_REQUIRED` |
| `Validation(codes)` | 422 `E_VALIDATION` with `errors[]` codes: `four_eyes_required`, `freeze_list_excluded`, `list_age_above_floor`, `reason_not_in_family`, `text_too_long`, `harness_not_passed`, `count_drop`, `expiry_too_far`, `bulk_not_identical`, `unknown_list_code` |
| `RateLimited` | 429 `E_RATE_LIMITED` |
| `IndexNotLoaded(list)` | 503 `E_UNAVAILABLE` (never a clear answer) |
| `PayloadTooLarge` | 413 `E_PAYLOAD_TOO_LARGE` |

No new problem code: the validation codes sit in `errors[]` (API.md section 6 is unchanged).

### 3.15 Masking and the tipping-off review

- **Masked kinds** registered with the phase 03 masking registry: `screening_alert` and `pair_memory`. Each has the complete probe set of the inference harness (get, list count, error body, SSE `queue.counts`, the audit viewer, the latency comparison). Phase 03 registers masked kinds "from phase 15"; this phase needs them earlier (open doubt).
- **List filters**: alert and pair lists filter rows in SQL by the policy's equivalent predicate (SS and M see every undecided and decided alert; no other role sees a row); no total is returned for these lists.
- **Tipping-off review**: for each screen, route, notification, export and SSE event of this phase, a person outside the policy learns nothing: alert and pair routes answer 404 identical to a missing id; S3 shows list status (names, counts, checksums) to configuration, approval, audit and assessor roles and no party; the harness report carries case ids and codes only; `screening.searched` records a keyed hash, and the search answer is list data, visible only to SS and M through a purpose-coded call; `queue.counts` goes to SS and M; no branch role holds any screening scope; this phase sends no notification.


---

## 4. Infrastructure Updates

### 4.1 Job kinds (Postgres `jobs`; coalesce key in brackets)

| Kind | Binary | Payload | Concurrency, attempts | Schedule or trigger |
|-|-|-|-|-|
| `list_stale_sweep` | worker | `{}` [`list_stale_sweep`] | 1, 3 | `schedule.list_stale_sweep`; raises stale alarms; enqueues due `list_fetch` |
| `list_fetch` | worker | `{list_id}` [list id] | 2, 5 | sweep, or `POST /lists/{id}/fetches` |
| `list_parse` | worker | `{version_id}` [version id] | 2, 3 | after `list_fetch`, upload or the bank snapshot; re-enqueues itself while the AV scan is pending |
| `harness_run` | worker | `{version_id, kind}` [version id, kind] | 2, 3 | after a successful parse; `POST /screening/harness-runs` |
| `list_activate` | worker | `{version_id, by}` [list id] | 1, 3 | feed auto-activation (setting) or the SU and 4E route |
| `screening_lookback` | worker | `{version_id}` [version id] | 1, 5 (checkpointed) | queued by `list_activate` and by `POST /screening/lookbacks` |
| `screening_daily_base` | worker | `{}` [`daily_base`] | 1, 5 (checkpointed) | `schedule.screening_daily_base` |
| `screening_onboarding` | worker | `{batch_id}` [batch id] | 2, 5 | outbox topic `canon.touched` (phase 11 emits; this phase adds the handler) |
| `screening_payment` | worker | `{}` [`payment`] | 1, 5 | `schedule.screening_payment` |
| `screening_adhoc` | worker | `{run_id}` [run id] | 4, 3 | `POST /screening/runs` |
| `pair_memory_sweep` | worker | `{}` [`pair_memory_sweep`] | 1, 3 | `schedule.pair_memory_sweep` |
| `timer_screening_alert_clock` | worker | timer row | 4, 3 | `admin.timers` kind `screening_alert_clock` (phase 00 sweep) |
| `ingest_bank_list` | akr-ingestd | `{list_id, source_id, object}` [list id] | 1, 3 | `POST /lists/{bank}/fetches`; the `ingest_` prefix is for akr-ingestd only |

Every handler re-reads its owner before it acts (phase 00 rule). `harness_run` and `list_activate` are idempotent on the version state: a second activation of an active version is a no-op.

### 4.2 Timers, outbox and OpenAPI

- **Timers**: `screening_alert_clock` (owner `screening_alert`), scheduled at alert creation, cancelled on a decision or a supersession.
- **Outbox**: no topic is produced by this phase. It consumes `canon.touched` (phase 11) through a `TopicHandler` (phase 00 registry).
- **OpenAPI**: tags `lists` (8 operations) and `screening` (19: the 17 of section 2.8 and the 2 of section 7.19); 27 in all, each with `x-akr-action` (the Cedar action of section 2.8), `x-akr-stepup` where SU applies, `x-akr-idempotency` (required on every POST, PUT and DELETE). `X-StepUp-Assertion` and `X-Access-Purpose` are documented headers. `GET /screening/alerts`, `GET /screening/alerts/{id}` and the pair routes document `Cache-Control: no-store`. The upload documents `multipart/form-data`. `ScreeningAlertView` and `PairMemoryView` reference the `affordances` schema.

### 4.3 Settings and their registry (copy of section 1.5; the floor is NULL throughout)

Governed: `screening.list_max_age_h`, `screening.feed_auto_activate`, `screening.max_count_drop_pct`, `screening.pair_memory_max_months`, `screening.good_guy_max_days`, `screening.doubt_policy`, `screening.priority_map`, `screening.harness_gate`. Ungoverned: `screening.review_clock_hours`, `screening.feed_max_bytes`, `screening.search_per_min`, `screening.lookback_chunk`, `screening.harness_cases_max` (section 7.19), and the four `schedule.*` keys. List thresholds and refresh are columns of `ref.lists`, changed by change request.

### 4.4 Metrics, readiness, zones, licences and AppState

- **Metrics**: `akr_list_age_seconds{list}`, `akr_list_stale{list}` (gauges), `akr_screening_alerts{state, priority}`, `akr_screening_run_seconds{kind}` and `akr_index_build_seconds{list}` (histograms), `akr_harness_recall{kind, class}`, `akr_pair_memory_drift` (must be 0), `akr_index_canary_failed` (counter), `akr_screening_suppressed_total{kind}`. Alerts: `akr_pair_memory_drift > 0` (critical), a freeze list stale for 30 minutes (critical), a failed index canary (critical).
- **Readiness**: akr-api `/readyz` adds `screening_index` (every enabled freeze list with an active version has a loaded index; degraded, not failed, otherwise) and `list_freshness` (informational).
- **Zones**: no new zone. `list_fetch` uses the phase 07 egress path (`AKR_EGRESS_PROXY`, network `z2_egress`, host allow-list in `source_ref`). `ingest_bank_list` is the only CBS-facing code of the phase and runs in akr-ingestd under `akr_ingest` through `SourceReader` (phase 05). Compose: no change.
- **Licence register** (N-48): the normaliser uses `unicode-normalization` (MIT or Apache-2.0); no OpenSanctions data, PyArabic or CAMeL code is linked; the lists are public publications and the UN, OFAC, EU and UK terms are recorded per list in `source_ref.publisher`.
- **`akr-ctl lists verify`** (read-only): checks the six rows, the threshold keys, the corpus and known-answer file checksums against the release manifest, and the gate of every active version; exit 2 on any failure.
- **AppState**: `api` gets `lists`, `screening`, `alerts`, `pairs`, `good_guys`, `harness`, `indexes: Arc<ScreeningIndexRegistry>` and `freeze: Arc<dyn FreezeTrigger>` (`RecordingFreezeTrigger` here). `worker` gets the handlers, `ListFeedSource` (HTTP through the proxy), `SubjectSource`. `ingestd` gets `BankListReader` and `BankListService`.

### 4.5 Existing entity, schema and route updates

- **DATA-MODEL s.9 and s.21 and s.23**: reconciled as section 1.6 lists; group 14 files are `0045` to `0049`.
- **Phase 00**: the `canon.touched` topic handler; the settings and schedule keys of section 1.5.
- **Phase 01**: no new step-up class (`approval` for upload and activation; `decision` for alert decisions, bulk decisions and pair approval).
- **Phase 03**: masked kinds `screening_alert` and `pair_memory`; Cedar file `84_screening.cedar` and the entities of section 3.11 in `schema.cedarschema`.
- **Phase 04**: four change targets (`list_setting`, `screening_policy`, `good_guy_rule`, and `harness_case` of section 7.19); the route map is unchanged.
- **Phase 09**: the reason family `decision` gains eight codes `DEC_SCR_*` (name differs, identifier differs, birth date differs, family name only, confirmed designation, potential needs review, insufficient data, other) in `backend/packs/reference/reason-codes.v1.json`; they stay inactive until the phase 09 family change request activates them.
- **Phase 11**: emits `canon.touched {batch_id, source_id, source_table, entity_kinds, business_date}` in the mapping transaction (phase 11 section 3.3, added at this review).
- **API.md 3.9**: seven rows added (section 2.8); **roadmap.md** row 14 is set to "XL, P1, depends on 09 and 11".

---

## 5. Frontend Impact

Screens of DESIGN-SYSTEM.md section 8.1, full shell (P1). Routes and layout patterns are those of the catalogue; the rail entry "Screening" shows for SS and M, and the list status for the configuration, approval, audit and assessor roles.

- **S1 Screening review** (`/screening/hits/:id`, COMPARE; DESIGN-SYSTEM 8.3). A verdict sentence with the evidence level and the applied threshold, not a score ring. Customer and list entry side by side, Arabic first with transliteration (the shared Name component). Between them the deciding identifiers, each agree, differ or not comparable: mother's name, birth date with precision, document numbers, place of birth, family number. A per-part table with the folds applied and the unmatched tokens; the frequency sentence; the list entry facts. Decision tiles in the Office's words with their consequence lines: التطابق الحقيقي (true match: freeze clock and Committee report), التطابق المحتمل (potential match: escalation, freezes nothing), التطابق غير الصحيح (incorrect match). "Remember this decision for this pair" appears only on the incorrect tile, unticked, with its expiry. A four-eyes notice on freeze lists; a coded reason is required; the decision uses `StepUpDialog` (phase 08) and sends `If-Match`.
- **S2 Screening queue** (`/screening`, LIST). Saved views "Open", "Potential", "Needs more data", "Mine", with counts from `queue.counts`; columns priority (code and word), subject name (Arabic first), list, top reason chip, score, age, next due (the clock), state; "Next" opens the oldest unclaimed item of the user's view; a bulk incorrect bar appears only when the selection is identical and never on freeze lists.
- **S3 List status and known-answer tests** (`/screening/lists`, REGISTER). A card per list: kind word, active version label, loaded and activated times, age against the limit with a stale banner, record count, checksum (first 12 hex digits), status words (Validated, Active, Failed with the failure word). Harness results as words and percentages with the user's numeral setting; the alert budget curve as a table. The upload action (SS, SU; the count-drop acknowledgement checkbox), "Fetch now", and the activate action with the four-eyes notice: the uploader does not see activate enabled. The national-list profile panel (N-12) and the diff by counts.
- **S4 Pair memory and exemption register** (`/screening/memory`, REGISTER). Tabs: "Pairs" (status, subject name, entry, approver, expiry, suppressions, last suppressed), "Good-guy rules" (list codes, pattern, approver, change request link, expiry, suppressions), "QA sample" (this week's draw), and "Freeze-match check" which must read zero.
- **Shared components**: the Name component (original and transliteration, chain alignment, mother's name, quality flags; research 31, section 4 hooks) is added to the phase 08 component layer for S1 and the customer screens. The i18n `fold` of phase 08 is tested against `parity-corpus.v1.jsonl` (section 6, check 3).
- **`screens.ts`**: `S1 /screening/hits/:id` (SS, M; `screening.alert.decide`), `S2 /screening` (SS, M; `screening.alert.read`), `S3 /screening/lists` (SS, CFG, M, CHK, CH, IA, AS; `screening_list.read`), `S4 /screening/memory` (SS, M, CHK, CH; `screening.pair.read`). The route-to-policy test maps each route to its Cedar action.
- **Generated hooks** (Orval): `useGetLists`, `useGetListsId`, `useGetListsIdVersions`, `usePostListsIdVersions`, `useGetListsIdVersionsVDiff`, `usePostListsIdVersionsVActivate`, `usePostListsIdFetches`, `usePutListsIdSettings`, `useGetScreeningRuns`, `usePostScreeningRuns`, `usePostScreeningLookbacks`, `usePostScreeningSearch`, `useGetScreeningAlerts`, `useGetScreeningAlertsId`, `usePostScreeningAlertsIdClaim`, `usePostScreeningAlertsIdDecision`, `usePostScreeningAlertsBulkDecision`, `useGetScreeningPairMemory`, `usePostScreeningPairMemory`, `usePostScreeningPairMemoryIdApprove`, `useDeleteScreeningPairMemoryId`, `useGetScreeningGoodGuys`, `usePostScreeningGoodGuys`, `useGetScreeningHarnessRuns`, `usePostScreeningHarnessRuns`.
- **Browser checks** (DR-77): axe on S1 to S4 in both languages and themes; the purpose dialog and the step-up dialog keyboard path; no alert data in the TanStack cache beyond the page (DR-75, `gcTime` 0 for the alert routes).

---

## 6. Verification

1. `make migrate`: 0045 to 0049 apply; a second run is a no-op; `SELECT code, enabled FROM ref.lists ORDER BY code` returns six rows (`bank`, `eu`, `iq_national`, `ofac_sdn`, `uk`, `un`) with `iq_national` and `un` enabled; the phase 00 schema checks pass for the eleven new tables, `alert.harness_cases` of section 7.19 among them (classification and retention rows present).
2. Negative DDL as `akr_app`: `DELETE FROM ref.list_versions` fails with permission denied (no role has a DELETE grant), and as the owner `akr_migrate` with `list versions are never deleted`; `UPDATE ref.list_entries` fails with permission denied; `INSERT INTO ref.list_entries` for an `active` version fails with `entries are written only while their list version is loading`; `UPDATE alert.screening_decisions` fails with permission denied (no UPDATE grant), and as `akr_migrate` with `screening_decisions is append-only`; a good-guy row with `list_codes ["un"]` fails its CHECK; a pair row with `approved_by = proposed_by` fails; a freeze `true` decision with an empty `prior_decider_ids` fails; `UPDATE alert.screening_alerts SET state = 'open'` on a `true` row fails with `a true alert is terminal`.
3. Parity: `cargo test -p names parity` runs `parity-corpus.v1.jsonl` (at least 300 cases) and the idempotence check; `cargo test -p db names_sql_parity` runs every case through `ref.name_key_k1` and `ref.name_key_block_k1` on PostgreSQL 17 under `C.UTF-8`; zero differences. `vitest` runs `i18n/fold.parity` over the same file. The three answers must agree on every case.
4. `cargo check`; `cargo sqlx prepare`; `cargo clippy --all-targets -- -D warnings`; `cargo deny check`. `cargo tree -p worker -i sources` shows no phase 05 `SourceReader` dependency; only `ingestd` depends on it.
5. `cargo test`, including: each rule of 3.4 in isolation (a family or tribe name with one common given name never alerts; a kunya alone never alerts; a weak alias caps at 0.6; birth year one year apart −0.02 and three years −0.20; a default date earns no credit; a document of the same type and number raises the score to 0.95; a thin query is `NeedsMoreData`, never `Clear`; a truncated subject leaves the list side unmatched weight at 0); the same input gives a byte-identical result twice; a candidate cap sets `truncated`; the gate fails when a self canary is removed from a candidate version (`harness_failed`); activation of a `loading` version is refused (409); a count drop without acknowledgement fails (`count_drop`); a feed failure leaves the last good version active; a second `screening_daily_base` run over the same subjects inserts no alert.
6. Curl suite `tests/curl/phase14_screening.sh` (dev users `ss@bank.test`, `m@bank.test`, `cfg@bank.test`, `chk@bank.test`, `ch@bank.test`, `ia@bank.test`, `a1@bank.test`, `ds@bank.test`; the lists are the seeded six with a synthetic fixture version of `iq_national` loaded by `akr-ctl lists verify`):
   - As A1 `GET /api/v1/screening/alerts` gives 403 `E_SCOPE`. As IA `GET /api/v1/screening/alerts` gives 403 `E_SCOPE`.
   - As SS `GET /api/v1/screening/alerts/<id>` with `X-Access-Purpose: assigned_work` gives 200 with `Cache-Control: no-store`; without the header gives 422 `E_PURPOSE_REQUIRED` (the caller is admitted, so nothing is masked); as CFG the same id gives 404 and a body byte-identical to a random UUID (diff empty).
   - As SS `POST /api/v1/screening/search` without purpose gives 422 `E_PURPOSE_REQUIRED`; with purpose, 25 calls in one minute give 429 with `Retry-After`.
   - As SS `POST /api/v1/screening/alerts/<id>/decision` (outcome `potential`): without step-up 401 `E_STEPUP_REQUIRED`; with step-up and no `If-Match` 428; with a stale version 409 `E_VERSION_CONFLICT`; with step-up and the current version 200.
   - On a freeze alert: SS (first decider) decides `true` 403 `E_SELF_APPROVAL`; a second SS decides `true` 200 and the audit `screening.freeze_trigger_recorded` appears; an open freeze alert decided `true` directly gives 422 `E_VALIDATION` with `four_eyes_required`.
   - Pair memory: SS proposes from an incorrect alert 201 `proposed`; the proposer approves 403 `E_SELF_APPROVAL`; another SS approves with step-up 200 `active`; the DELETE returns 200 with `status: invalidated`, and a replay with the same key gives `x-idempotent-replay: true`.
   - Lists: SS `POST /api/v1/lists/<iq_national>/versions` (multipart) without step-up 401, with step-up 202; the uploader's `POST …/activate` 403 `E_SELF_APPROVAL`; M activation while `loading` 409 `E_NOT_ALLOWED_IN_STATE`; M activation after the gate 200 and `GET /api/v1/lists/<iq_national>` shows `active`.
   - Good-guy: SS `POST /api/v1/screening/good-guys` with `list_codes: ["un"]` 422 with `freeze_list_excluded`; with `["bank"]` 202, and after CHK's approval a row exists in `alert.good_guy_rules`.
   - CFG `PUT /api/v1/lists/<un>/settings` with `If-Match` 202 (a CR); DS on the same route 403 `E_FORBIDDEN` (the scope admits DS, Cedar denies); without `If-Match` 428.
   - `POST /api/v1/screening/harness-runs` as SS 202; as IA `GET` 200; as A1 `GET` 403 `E_SCOPE`.
   - A repeated upload with the same `Idempotency-Key` and body returns the stored 202 with `x-idempotent-replay: true`; a different body gives 422 `E_IDEMPOTENCY_REUSE`.
7. Duplicates and drift: two `screening_daily_base` runs over the phase 05 sample: the second inserts zero alerts; `SELECT fingerprint FROM alert.screening_alerts GROUP BY 1 HAVING count(*) > 1` is empty; `akr_pair_memory_drift` is 0.
8. Audit: after the curl run `audit_log` holds `list.version_uploaded`, `list.validated`, `list.activated`, `screening.alert_created`, `screening.alert_decided`, `screening.freeze_trigger_recorded`, `screening.searched`, `pair.proposed`, `pair.approved`, `pair.invalidated`, `good_guy_rule.activated` and the `authz.denied` rows of the 403s; the search audit payload holds `query_hk` and no query text (a grep for the query string finds nothing); `akr-ctl audit verify` passes.
9. No CBS read outside `ingest_bank_list`: the phase 05 statement journal shows no statement during the search and decision steps; the phase 05 conformance suite is unaffected.
10. `make generate-api` twice gives zero diff; Swagger lists 27 operations with `x-akr-action` (the 25 of section 2.8 and the 2 of section 7.19); phases 00 to 12 tests still pass.
11. Latency (N-50): the inline benchmark on a synthetic 150,000-name list gives p50 under 5 ms and p99 under 25 ms (`cargo bench -p domain screening`).

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 3a: corrections found while checking phases 05 to 10 (tools/agent/gap1-run3a-verdict.md); lead-verified.

### 7.1 The names crate and the Name component exist before this phase (DR-371, DR-373)
`crates/names` is created by phase 10 (its section 7+, the folding behind `FoldArabic` and `SplitArabicName`); this phase
extends it, so the tree line "NEW" reads "extended". The Open doubt "Stored forms in canon" is closed: phase 10's
transforms call the crate, and screening keeps ignoring the stored forms. The `Name` component is built by phase 08
(UX-25); this phase extends it, which closes the Open doubt "Name component".

Gap pass 1 (2026-10-10), run 3b, from tools/agent/gap1-run3b.md; lead-verified (tools/agent/gap1-run3b-verdict.md).

### 7.2 Link to a case at decision (phase 17)
From phase 17 on, decision step 8 (section 3.6) calls `ScreeningCaseLinker::link_screening(tx, screening_alert_id,
actor)` in the decision transaction when `subject_kind = 'party'` and the decision is `true` or `potential`.
Counterparty and payment-party subjects are not linked (phase 18, DR-147). Phase 17 defines the port; before phase 17
no call is made.

### 7.3 The screening.hit topic and the suppression sample (DR-375)
From phase 15 on, the decision transaction of section 3.6 writes the outbox topic `screening.hit` for a party subject
decided `potential` or `true`, payload `{alert_id, party_id, list_kind, outcome}` (ids and codes only, no name or list
text). Phase 15 adds the write together with the first handler of the topic (its re-score trigger on `screening.hit`,
DSL.md C.5, stored with the neutral `trigger_code` `TRG_REVIEW_REQUESTED`, phase 15 section 3.6); phase 19 appends its handler in the ordered handler list of phase 00 section 4.5 item 10.
Before phase 15 no row is written.
The suppression records that phase 19 samples are the `alert.pair_memory` rows: `qa_suppression_sample` takes the rows
whose `last_suppressed_at` falls in the ISO week (5 per cent, plus every row on a freeze-list entry). This phase's sweep
keeps expiry and the counters and draws no sample.

### 7.4 Hold alerts from payment holds (phase 29)
Phase 29 extends this phase and owns the extension (phase 29 section 4.5 item 4): `ScreeningService::open_hold_alert(
run, candidate)`, with one caller, `HoldSweepService`. It opens a system `payment` run with `requested_by` NULL and
`trigger_ref` = the hold id, and writes the candidate through `AlertRepository::insert_candidates`. The comments on
`alert.screening_runs` read "adhoc runs and hold payment runs" (`subject_id`) and "payment runs: the hold id (phase
29)" (`trigger_ref`). The subject view of a hold alert reads the names from `assess.holds.party_names_enc` under this
phase's masking. `screening_payment` skips an entry already alerted on a hold whose decision has the same match key.

### 7.5 Risk modifiers (phase 15)
Phase 15 defines `RiskModifierSource::modifier(party)` (its section 2), read from the json setting
`screening.rating_modifiers` (a setting, not a table). From phase 15 on, the threshold step (section 3.4 step 8) adds
the modifier for a party subject within the existing bounds. It is 0 before phase 15 and for the other subject
kinds.

### 7.6 Delisting by absence (DR-376)
An entry that is missing from a new list version is delisted by its absence: phase 20 section 3.5 already reads
absence. Rows are not copied into the new version (a version is immutable after `loading`). `delisted_on` is set only
when the publisher's file carries a delisting date. No feed format of this release carries one (a decision of this phase: no parser of the closed `feed_format` set of section 3.2 reads such a field), so the column stays NULL and every delisting is by absence. A silent drop is guarded by `screening.max_count_drop_pct` with
`acknowledged_count_drop`, and `diff_summary.removed` counts the drops. This closes phase 20 Open doubts 22.

### 7.7 Feed check time on ref.lists (DR-378)
`ref.lists` gains `feed_checked_at TIMESTAMPTZ` (NULL until the first check) in migration `0045`; the column goes on
`ref.lists`, not on `ref.list_versions`, which is immutable after loading. Migration `0048` adds `feed_checked_at` to
the column list of the `GRANT UPDATE ... ON ref.lists TO akr_app` line, and `0049` adds it to the `internal` row of
`ref.lists` in `admin.column_classification`. `list_fetch` step 2 sets it on `NotModified` (with the audit kind
`list.feed_unchanged`) and on every successful fetch. `ListRecord` gains the field, and its staleness is `age_hours =
now - max(active.activated_at, feed_checked_at)` (or `loaded_at` for `cbs_snapshot`). The 12-hour cap does not change.
This closes the last sentence of phase 29 Open doubts 29.

### 7.8 Retention of screening alerts (DR-377)
Screening alerts, freeze-list alerts included, stay R7. Where OFAC applies, the bank sets `retention.period_years.r7`
(phase 23; default 7, minimum 7) to 10. No new retention class is added. This closes the Open doubt "Retention".

Gap pass 1 (2026-10-10), run 3b: correction found by the lead while checking phase 14 against phase 02 (tools/agent/gap1-run3b-verdict.md); lead-verified.

### 7.9 Access event of the name search (DR-383)
Phase 02's `audit.access_event.action` is closed (`view`, `search_hit`, `export`, `print`, `download`, `unmask`) and has
no `search`. Section 3.12 step 5 writes one access event per search with action `search_hit`, resource type
`screening_search`, `resource_id` = the id of the `screening.searched` audit event (a fresh UUID v7, so the two rows
join), tier `restricted`, the purpose code from `X-Access-Purpose` and no `subject_party`. It is written through phase
02 `AccessScope` before the body (DR-46); a search with no candidate still writes it. The route carries
`x-akr-access: screening_search`. The list entries shown are not access events of their own (they are not bank
records).

Gap pass 1 (2026-10-10), run 8c (coverage of VC market IDs), from tools/agent/gap1-run8c.md; lead-verified (tools/agent/gap1-run8c-verdict.md).

### 7.10 One-hour re-screen after a list activation (VC-03; ROADMAP M9 and its exit criterion; DR-404)
ROADMAP M9 asks for a full re-screen within one hour of a list update, and its exit criteria say "At most one hour from
a list update to a full re-screen". The 12-hour freshness rule of section 3.2 is unchanged; this adds the one-hour
measure and alarm. No table, job, timer or route is added; the once-only mark of step 3 is the `alarm_marks` column of `ref.lists` (section 7+, verification pass).
1. The re-screen of a version is its `lookback` run (section 3.5 run kind 2, queued by `list_activate`): the
   `alert.screening_runs` row with `kind = lookback` and `trigger_ref` = the version id. Its re-screen time is the
   run's `finished_at` minus the version's `activated_at`. A look-back requested later through
   `POST /screening/lookbacks` for the same version does not count.
2. When that run completes, `ScreeningService` records the re-screen time in the histogram
   `akr_list_rescreen_seconds{list}` and adds `rescreen_seconds` (a number) to the payload of
   `screening.run_finished`.
3. `list_stale_sweep` (every 10 minutes, section 4.1) also selects each `active` version whose `activated_at` is more
   than one hour old and that has no `completed` look-back run of step 1. For each, it writes the audit event
   `list.rescreen_late {list_id, version_id}` (resource list_version) once per version, the same way `list.stale` is
   written once per transition, and sets the gauge `akr_list_rescreen_late{list}` to 1. The gauge returns to 0 when
   the look-back completes. A freeze list with the gauge at 1 raises a critical alert. With the 10-minute schedule the
   alarm comes at most 70 minutes after activation.
4. One hour is a fixed constant (`RESCREEN_TARGET`), not a setting: it is the ROADMAP exit criterion, not a bank
   choice.
5. `ListVersionView` carries `rescreen_seconds` (null until the run of step 1 completes; the run's `finished_at` minus the version's `activated_at`, computed when the view is built), and the S3 list status shows it for the active version next to the list lag
   that the harness report already gives (section 3.9).
6. Section 3.13 gains `list.rescreen_late`; section 4.4 gains the histogram, the gauge and the alert.

Gap pass 1 (2026-10-10), gap items (ac), (ad) and (ae) of tools/agent/gap-items.md (found in runs 7b and 8c); lead work, no Haiku draft.

### 7.11 Subjects of the look-back (VC-06; gap item (ae); DR-406)
Section 3.5 run kind 2 (`lookback`) builds its in-memory index from "all current subjects". The subjects are:
1. every current party (`canon.party`, types `person` and `entity`), with the names of section 3.5;
2. every counterparty (`canon.counterparty`) whose `party_id` is null, from its `names` (a counterparty linked to a
   party is screened as that party).
Payment-message parties are not in the look-back: each is screened once by the payment run (run kind 4) when first
seen. A later list change reaches the sender or beneficiary through the counterparty row, when the mapping links one.
The daily base run (run kind 1) is unchanged (parties only).

Adversarial pass (2026-10-10), run 2 (alerts under concurrency, replay and late data), from tools/agent/adv-run2.md; lead-verified (tools/agent/adv-run2-verdict.md).

### 7.12 Superseded alerts free their fingerprint (ADV-2-08, HIGH; DR-408)
In migration `0046` the column is `fingerprint BYTEA NOT NULL CHECK (octet_length(fingerprint) = 32)` without
`UNIQUE`, and `CREATE UNIQUE INDEX IF NOT EXISTS uq_screening_alerts_fingerprint ON alert.screening_alerts (fingerprint)
WHERE superseded_at IS NULL;` holds the dedupe key. `AlertRepository::insert_candidates` uses `ON CONFLICT
(fingerprint) WHERE superseded_at IS NULL DO NOTHING`. A decided alert is never superseded, so a decided fingerprint
still blocks a rerun; a name that changes and changes back inserts a new alert for the restored name, and section 3.5
step 4 supersedes the undecided alert of the middle name. The section 6 duplicate check reads `SELECT fingerprint FROM
alert.screening_alerts WHERE superseded_at IS NULL GROUP BY 1 HAVING count(*) > 1`. Test: N1, then N2, then N1 again
leaves one open alert, carrying the N1 forms.

### 7.13 A superseded alert is refused in SQL (ADV-2-09, LOW; DR-408)
Section 3.5 step 4 supersedes with `WHERE decided_at IS NULL AND superseded_at IS NULL` and sets `version = version +
1`, so an `If-Match` read before the supersede is 409 `E_VERSION_CONFLICT`. Decide step 3 reads `superseded_at` in the
step 2 row. The guard `alert.screening_state_guard` (migration `0048`) adds: `IF OLD.superseded_at IS NOT NULL AND
(NEW.state IS DISTINCT FROM OLD.state OR NEW.decided_at IS DISTINCT FROM OLD.decided_at) THEN RAISE EXCEPTION 'a
superseded alert takes no decision' USING ERRCODE = 'check_violation'; END IF;`. Test: a decision with the pre-supersede
version is 409.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.14 Step-up actions of this phase (V-14-01-d, LOW; DR-413)
Section 4.5 (the phase 01 row): each step-up route of this phase declares one `StepUpAction { name, class }` constant (phase 01), whose `name` is the Cedar action of its route row in section 2.8 and whose `class` is the SU class of that row. `StepUpAction::Decision` is `{ name: "screening.alert.decide", class: decision }`; the bulk decision declares its own constant, `StepUpAction::BulkDecision`, `{ name: "screening.alert.bulk_decide", class: decision }`, consumed once for the batch with no resource reference.

### 7.15 A failed list version is the incident; count mismatch (V-14-02-a, V-14-02-b, MEDIUM; DR-413)
Section 3.2 `list_parse`: every failure (steps 1, 2, 4 and 5, and the gate of section 3.9) sets the version `failed` with its `failure_code`, writes `list.failed {list, version, failure_code}` (section 3.13) and leaves the last good version active. That audit event is the incident of step 2 and of U-02. No row of `casework.ops_incidents` is written (phase 20: that table holds the failures of the MLRO's orders). Step 4: a parsed row count, taken before identical rows are removed, that differs from a non-null `expected_count` fails the version with `count_mismatch`. Section 4.4 gains the counter `akr_list_version_failed_total{list, failure_code}` and the alert "a list version failed" (critical for a freeze list, warning otherwise).

### 7.16 Fields of the filter, input and result types (V-14-03-a, LOW; DR-413)
- `VersionFilter` and `PairFilter` are empty in this phase: `GET /lists/{id}/versions` and `GET /screening/pair-memory` take the keyset page only. The masking predicate of section 3.15 applies in SQL to the pair list.
- `AlertFilter { view: AlertView }`; `AlertView` is `Open`, `Potential`, `NeedsMoreData` or `Mine`, the four saved views of S2 (query parameter `view`, default `open`).
- `HarnessFilter { list_version_id: Option<Uuid> }` (query parameter `list_version_id`).
- `IdentifierInput` is the raw form of `Identifiers` (section 2.1), hashed on receipt and never stored, except inside the sealed query of a known-answer case (section 7.19 item 3): `{ birth: Vec<{ year, month?, day? }>, documents: Vec<{ doc_type, number }>, nationality: Vec<IsoCountry>, gender?, mother_chain?, family_no? }`.
- The `components` of `ScoreResult` are `MatchComponents { name_score: f32, identifier_adjustment: f32, risk_modifier: f32, threshold: f32, threshold_source: ThresholdSource }` (section 3.4 rules 4, 6 and 8); `ThresholdSource` is `NameOnly` or `WithIdentifiers`. The name `ScoreComponents` is left to phase 17, whose type of that name has other fields.
- `FeedError` is `Transport { status: Option<u16>, reason: String }` (writes `list.feed_failed`, section 3.2 step 2), `NotAllowed` (host or path outside the allow-list of `source_ref`) or `TooLarge` (body above `screening.feed_max_bytes`).
- `BankListRow` is one mapped row of the bank's list: `entry_key` and the identifying fields of `list_parse` step 6, each from the bank column that the approved mapping document names.

### 7.17 Registry entry of the name search (V-14-08-a, MEDIUM; DR-413)
This phase registers its one sensitive operation, `POST /api/v1/screening/search` (it carries `x-akr-access: screening_search`, section 7.9), in `api::access::SENSITIVE`, as phase 02 section 3 asks of each phase that adds a sensitive route. Its fixture request is that route as `ss@bank.test` with `X-Access-Purpose: assigned_work` and a name of the synthetic corpus of section 3.9 in the body. The fixture writes exactly one access event: action `search_hit`, resource type `screening_search`, tier `restricted`, no `subject_party`.

### 7.18 Once-only marks of the list alarms (V-14-09-a, MEDIUM; DR-413)
`list.stale` (section 3.2) and `list.rescreen_late` (section 7.10 step 3) are written once because the sweep keeps a mark. `ref.lists` gains `alarm_marks JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(alarm_marks) = 'object')` in migration `0045`; migration `0048` adds it to the column list of the `GRANT UPDATE ... ON ref.lists TO akr_app` line, and `0049` to the `internal` row of `ref.lists` in `admin.column_classification` (as section 7.7 does for `feed_checked_at`). `list_stale_sweep` is its only writer, in the transaction of the audit event:
- `stale_at`: when the list is stale and the key is absent, the sweep writes `list.stale` and sets the key to the current time; when the list is fresh and the key is present, it removes the key. One event is written per transition to stale.
- `rescreen_late_version`: when the active version meets the test of section 7.10 step 3 and the key does not hold its id, the sweep writes `list.rescreen_late` and sets the key to that version id.
The gauges are set from the computed state at every sweep and need no mark.

Owner decisions (2026-10-10): the product owner overturned the lead decision that the known-answer corpus grows by release only (section 3.9, Feedback); lead-written, checked in the owner-decision round (tools/agent/owner-verdict.md).

### 7.19 Known-answer cases the bank adds (owner decision 2; DR-415)
A bank adds its own known-answer cases through change control: a name, with identifiers when the bank has them, that must hit a named list entry (`hit`) or must not hit it (`no_hit`). This is the bank-side intake of N-44: a spelling the analysts want kept as a known hit, and a name that matched wrongly kept as a hard negative. The shipped corpus stays synthetic (N-38) and it alone decides the gate. The bank's cases run in every gate and oracle run and are reported beside it. They never block a list activation, because a list update is a legal duty (the 12-hour rule) and a case can go stale when the publisher changes an entry. A configuration change that would lose a known hit says so before it is approved (item 8). The bank-specific variant table (N-57) stays P2. DR-422 locks the correction of the check round of 2026-10-11: the harness report carries one result per case (`results`, item 6).

1. **Table** (extends DATA-MODEL s.9; the table and its guard join migration `0046_alert_screening_runs_alerts.sql` as its second part, the grants join `0048_screening_grants_guards.sql`, and the classification, retention and setting rows join `0049_screening_settings_classification.sql`):

```sql
-- migration 0046_alert_screening_runs_alerts.sql, part 2 (section 7.19)
-- Known-answer cases the bank adds (DR-415). No party id and no alert id: a case is a name and a list entry key.
CREATE TABLE IF NOT EXISTS alert.harness_cases (
    id                  UUID PRIMARY KEY,
    list_id             UUID NOT NULL REFERENCES ref.lists(id) ON DELETE RESTRICT,
    entry_key           TEXT NOT NULL CHECK (length(entry_key) BETWEEN 1 AND 120),
    expect              TEXT NOT NULL CHECK (expect IN ('hit', 'no_hit')),
    query_enc           BYTEA NOT NULL,                       -- sealed JSON {name, identifiers?} (personal, field encryption; AAD = id)
    query_hk            BYTEA NOT NULL CHECK (octet_length(query_hk) = 32),      -- keyed hash of the normalised tokens (section 3.12)
    trial               JSONB NOT NULL CHECK (jsonb_typeof(trial) = 'object'),   -- {list_version_id, outcome, met, ran_at}
    state               TEXT NOT NULL DEFAULT 'proposed' CHECK (state IN ('proposed', 'active', 'retired', 'rejected')),
    change_request_id   UUID NOT NULL REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    retired_by_request  UUID REFERENCES admin.change_requests(id) ON DELETE RESTRICT,
    created_by          UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    activated_at        TIMESTAMPTZ,
    closed_at           TIMESTAMPTZ,
    CHECK (state NOT IN ('active', 'retired') OR activated_at IS NOT NULL),
    CHECK ((state IN ('retired', 'rejected')) = (closed_at IS NOT NULL)),
    CHECK ((state = 'retired') = (retired_by_request IS NOT NULL))
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_harness_cases_open ON alert.harness_cases (list_id, entry_key, query_hk)
    WHERE state IN ('proposed', 'active');
CREATE INDEX IF NOT EXISTS idx_harness_cases_list       ON alert.harness_cases (list_id, state, created_at DESC, id DESC);
CREATE INDEX IF NOT EXISTS idx_harness_cases_cr         ON alert.harness_cases (change_request_id);
CREATE INDEX IF NOT EXISTS idx_harness_cases_retired_cr ON alert.harness_cases (retired_by_request)
    WHERE retired_by_request IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_harness_cases_created_by ON alert.harness_cases (created_by);

-- Rows are never deleted; a retired or rejected case never changes.
CREATE OR REPLACE FUNCTION alert.harness_cases_guard() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'known-answer cases are never deleted' USING ERRCODE = 'insufficient_privilege';
    END IF;
    IF OLD.state IN ('retired', 'rejected') THEN
        RAISE EXCEPTION 'known-answer case %: % is final', OLD.id, OLD.state USING ERRCODE = 'insufficient_privilege';
    END IF;
    RETURN NEW;
END $$;
DO $$ BEGIN
    CREATE TRIGGER trg_harness_cases_guard BEFORE UPDATE OR DELETE ON alert.harness_cases
        FOR EACH ROW EXECUTE FUNCTION alert.harness_cases_guard();
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

-- migration 0048_screening_grants_guards.sql, part 2 (section 7.19)
GRANT SELECT, INSERT ON alert.harness_cases TO akr_app;
GRANT UPDATE (state, retired_by_request, activated_at, closed_at) ON alert.harness_cases TO akr_app;

-- migration 0049_screening_settings_classification.sql, part 2 (section 7.19)
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'alert', 'harness_cases', c, v.cls::admin.data_classification
FROM (VALUES
  ('internal', ARRAY['id', 'list_id', 'entry_key', 'expect', 'trial', 'state', 'change_request_id',
                     'retired_by_request', 'created_by', 'created_at', 'activated_at', 'closed_at']),
  ('personal', ARRAY['query_enc', 'query_hk'])
) AS v(cls, cols)
CROSS JOIN LATERAL unnest(v.cols) AS c
ON CONFLICT DO NOTHING;
UPDATE admin.column_classification SET encrypted = TRUE
 WHERE (schema_name, table_name, column_name) = ('alert', 'harness_cases', 'query_enc');
UPDATE admin.column_classification SET keyed_hash = TRUE
 WHERE (schema_name, table_name, column_name) = ('alert', 'harness_cases', 'query_hk');

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('alert', 'harness_cases', 'R7', 'closure', 'closed_at', 'Closed known-answer cases; open rows stay')
ON CONFLICT DO NOTHING;

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('screening.harness_cases_max', '500', 'integer', NULL, FALSE,
  '{"ar": "أقصى عدد لحالات الاختبار التي يضيفها المصرف لكل قائمة", "en": "Maximum bank known-answer cases per list"}')
ON CONFLICT (key) DO NOTHING;
```

   The columns outside the UPDATE grant never change: a different name or expectation is a new case. `akr_readonly` gets no grant, as for `alert.screening_alerts`, `alert.screening_decisions`, `alert.pair_memory` and `alert.good_guy_rules`. `screening.harness_cases_max` is ungoverned, with registry bounds 1 to 5,000. `trial.outcome` is the scorer outcome of section 3.4 rule 9 in snake case (`hit`, `below_line`, `needs_more_data`, `clear`). A trial is never `absent`: item 4 step 3 refuses a key that the active version does not hold, and `absent` is a result of a later harness run only (item 6).

2. **Routes** (added to the `screening` router of section 2.9; the route count of this phase is now 27):

| Method | Path | Scope | Description (cedar action) |
|-|-|-|-|
| GET | `/screening/harness-cases` | `screening.work` | The bank's known-answer cases: `list_code?`, `state?`; the name and identifiers unsealed (`screening.harness_case.read`) |
| POST | `/screening/harness-cases` | `screening.work` | Propose a case, or the retirement of one; **CR** `harness_case`; `Idempotency-Key`; 202 (`screening.harness_case.propose`) |

   One POST takes two bodies, as `POST /screening/good-guys` does (section 3.8): `HarnessCaseProposal`, or `HarnessCaseRetire`, which the route tells apart by its `action` field.

3. **Types.** Entity `HarnessCase` (fields mirror item 1). Value objects `CaseExpect` (`Hit`, `NoHit`), `HarnessCaseState` (`Proposed`, `Active`, `Retired`, `Rejected`), `CaseTrial { list_version_id: Uuid, outcome: String, met: bool, ran_at: DateTime<Utc> }`, `CaseQuery { name: String, identifiers: Option<IdentifierInput> }` (the sealed JSON of `query_enc`). DTOs: `HarnessCaseProposal { list_code: ListCode, entry_key: String (1 to 120 characters), expect: CaseExpect, name: String (1 to 200 characters), identifiers: Option<IdentifierInput>, reason_code: String, reason_text: String }`; `HarnessCaseRetire { action: "retire", case_id: Uuid, reason_code: String, reason_text: String }`; `HarnessCaseFilter { list_code: Option<ListCode>, state: Option<HarnessCaseState> }`; `HarnessCaseView` (the columns of item 1 without `query_enc` and `query_hk`, with `list_code`, `name` and `identifiers`; documented with `Cache-Control: no-store`). `IdentifierInput` is hashed on receipt and never stored everywhere else (section 7.16); a case is the one place that keeps it, sealed inside `query_enc`, because the harness must run the same query again. Repository trait in the domain layer, `PgHarnessCaseRepository` in `db`:

```rust
#[async_trait]
pub trait HarnessCaseRepository: Send + Sync {
    async fn insert(&self, tx: &mut Tx, c: &NewHarnessCase) -> Result<HarnessCase, RepoError>;        // uq open -> Conflict
    async fn lock(&self, tx: &mut Tx, id: Uuid) -> Result<Option<HarnessCase>, RepoError>;            // FOR UPDATE
    async fn get(&self, conn: &mut PgConnection, id: Uuid) -> Result<Option<HarnessCase>, RepoError>;
    async fn list(&self, conn: &mut PgConnection, f: &HarnessCaseFilter, page: Keyset) -> Result<Page<HarnessCase>, RepoError>;
    async fn active_for_list(&self, conn: &mut PgConnection, list_id: Uuid) -> Result<Vec<HarnessCase>, RepoError>;
    async fn count_open(&self, conn: &mut PgConnection, list_id: Uuid) -> Result<i64, RepoError>;     // proposed + active
    async fn activate(&self, tx: &mut Tx, id: Uuid, at: DateTime<Utc>) -> Result<(), RepoError>;      // proposed -> active
    async fn retire(&self, tx: &mut Tx, id: Uuid, cr: Uuid, at: DateTime<Utc>) -> Result<(), RepoError>;   // active -> retired
    async fn reject(&self, tx: &mut Tx, id: Uuid, at: DateTime<Utc>) -> Result<(), RepoError>;        // proposed -> rejected
}
```

   `NewHarnessCase` holds the id, `list_id`, `entry_key`, `expect`, `query_enc`, `query_hk`, `trial`, `change_request_id` and `created_by`. `activate` sets `state = 'active'` and `activated_at = at`; `retire` sets `state = 'retired'`, `retired_by_request = cr` and `closed_at = at`; `reject` sets `state = 'rejected'` and `closed_at = at`. These four columns are the whole UPDATE grant of item 1. The list is keyset-paged by `(created_at DESC, id DESC)`. Services gain `HarnessCaseService` and the fourth change target `HarnessCaseTarget`; AppState `api` gains `harness_cases: Arc<HarnessCaseService>`.

4. **`HarnessCaseService::propose(ctx, body)`** (`POST /screening/harness-cases`):
   1. Cedar `screening.harness_case.propose`; `Idempotency-Key` required.
   2. A `HarnessCaseRetire` body: the case exists (else 404 `E_NOT_FOUND`); go to step 7 with `{op: "retire", case_id}`.
   3. A `HarnessCaseProposal` body: the list of `list_code` is enabled and has an active version (else 422 `unknown_list_code`); that version holds an entry with `entry_key` (else 422 `entry_not_found`); `count_open(list)` is below `screening.harness_cases_max` (else 422 `too_many_cases`).
   4. Normalise `name` with the names crate (section 3.1), as section 3.12 step 3 does for a search, and hash the identifiers into `Identifiers` on receipt (section 7.16). A thin query is refused (422 `thin_query`): it can never be assessed, so it can never be a known answer. `query_hk` is the keyed hash of the normalised tokens, computed as the `query_hk` of section 3.12 step 5; an open case with the same list, entry key and `query_hk` -> 422 `case_exists`.
   5. Trial: score the query against that entry in the active version's index under the live configuration (`ScreeningIndexRegistry`, section 3.3; `Scorer`, section 3.4). `met` is true when `expect` is `hit` and the outcome is `Hit`, or when `expect` is `no_hit` and the outcome is not `Hit`. An unmet case is accepted: a known miss and a name that still alerts are what the report is for.
   6. `case_id` = UUIDv7; seal `CaseQuery` as `query_enc` (AAD = `case_id`).
   7. `ChangeRequestService::create` (phase 04 section 3.2; origin = this dedicated route, so `submit` runs in the same transaction) with `object_kind = harness_case` and, for a proposal, the body `{op: "add", case_id, list_code, entry_key, expect, trial, query_enc, query_hk}`. `query_enc` travels as base64 ciphertext that no reader of the request can open, and the diff of item 5 leaves it and the entry key out.
   8. 202 with the `ChangeRequestView`.

   **`list(ctx, filter, page)`** (`GET /screening/harness-cases`): Cedar `screening.harness_case.read`; the repository list, each row unsealed into a `HarnessCaseView`.

5. **Change target `harness_case`** (a fourth target; the table of section 3.10 keeps its three rows; reason family `change`; `generic_create = false`; `emergency_allowed = false`; no inventory kind):

| Field | Value |
|-|-|
| Route | class `material` for a proposal and for a retirement, so the shipped route map sends both to `cr_material`: the MLRO accepts the pair and CHK the change. Volume effect `neutral`: a case changes no match and no alert. The route map gains no rule |
| `object_key` | `harness_case:<case id>` (one open change per case) |
| Base version | none (`current_version` returns `None`) |
| Prepare | `add`: the list, entry, cap and duplicate checks of item 4 steps 3 and 4 again on the body (the duplicate check by `query_hk`; the name is not in the body). `retire`: the case is `active` (else 409 `E_NOT_ALLOWED_IN_STATE`). `required_evidence` is empty |
| `on_submitted` | `add`: insert the row `proposed` (the unique index refusing it -> 422 `case_exists`); audit `harness_case.proposed {case_id, list_code, expect, met}`. `retire`: audit `harness_case.retire_proposed {case_id}` |
| `check_before_decision` | none |
| Activate | `add`: `lock`; not `proposed` -> the target refuses (phase 04: an immediate approval is rolled back with the error, section 3.4; a scheduled activation ends `failed`, section 3.6); `activate`; audit `harness_case.activated {case_id, change_request_id}`. `retire`: `lock`; not `active` -> refused in the same way; `retire` with the request id; audit `harness_case.retired {case_id, change_request_id}` |
| Revert | not allowed |
| Closed | `add`: `reject` (`proposed` -> `rejected`); audit `harness_case.rejected {case_id, how}`. `retire`: nothing |
| Diff | `add`: the list code, the expectation and the trial (outcome and `met`). `retire`: the case id, the list code and the expectation. Never the name, the identifiers or the entry key |

   The MLRO reads the name on S3 before deciding (item 11); CHK decides on what the diff shows.

6. **Harness runs** (section 3.9; job `harness_run`). A `gate` or `oracle` run of a version of list L also runs the `active` cases of L (`active_for_list`): for each case it unseals the query, finds the entry with the case's `entry_key` in that version and scores the pair under the run's configuration. The results go into `report.bank_cases = {cases, met, unmet, absent, results: [{case_id, result}]}`: `results` has one element per case the run took, and `result` is `met`, `unmet` or `absent`; `absent` counts cases whose entry the version does not hold (a delisting), and an absent case is neither met nor unmet. Bank cases are outside `cases`, `passed`, `recall_clean`, `recall_variant`, `hard_negative_alerts`, `failures` and `passed_gate`: the gate of section 3.9 reads the shipped corpus and the self canaries only, and `corpus_sha256` stays the shipped file's hash. `canary` runs do not run them. The metric `akr_harness_bank_cases{list, result}` (gauge; `met`, `unmet`, `absent`) joins section 4.4.

7. **Feedback** (N-44; replaces the last sentence of the Feedback item of section 3.9). Every decided alert stays a labelled pair as stored. A bank adds a pair to its known answers through this section: the person looking at a decided alert on S1 proposes the name and the entry key the screen already shows (item 11), and the API stores no link to the alert or to the party.

8. **Governed changes** (section 3.10, `prepare` of `list_setting` and `screening_policy`). The dry run also scores the active `hit` cases of each list the change touches, under the old and the new configuration. A `list_setting` change touches the list of its object key; a `screening_policy` change touches every enabled list. A case met under the old and unmet under the new is a lost known hit: `impact.known_hits_lost` lists the case ids, the request carries the warning `known_hit_lost` (phase 04: a warning must be accepted at submit), and the base class is at least `material`. `impact.no_hit_cases = {alerting_old, alerting_new}` counts the `no_hit` cases that alert under each configuration, for the approver's reading only.

9. **Audit events** (five more kinds in section 3.13, all on resource `harness_case`): `harness_case.proposed`, `harness_case.retire_proposed`, `harness_case.activated`, `harness_case.retired`, `harness_case.rejected`. Payloads carry ids, the list code, the expectation and the trial outcome, never the name, the identifiers or the entry key. `harness.run_completed` gains the counts of `report.bank_cases`.

10. **Cedar** (`84_screening.cedar`). Two actions, `screening.harness_case.read` and `screening.harness_case.propose`, join the action lists of the permits `screening.work.ss` and `screening.work.m` and of no other permit: SS and M read and propose; CFG, CHK, CH, IA and AS reach neither. Both have an `x-akr-action` entry. Bundle tests: CFG, CHK, CH, IA and AS are refused both actions; the maker of a case cannot decide its request (phase 04 four-eyes).

11. **Errors and masking.** No variant is added to section 3.14: `entry_not_found`, `thin_query`, `too_many_cases` and `case_exists` join the `errors[]` codes of `Validation(codes)`, and `unknown_list_code` is reused. No masked kind is added: the cases have no route by id, and the list route passes only SS and M at the scope gate and in Cedar, so every other caller gets 403 `E_SCOPE`, which says nothing about a subject. **Frontend**: S3 gains the tab **Bank cases**, rendered for SS and M only (`useGetScreeningHarnessCases`, `usePostScreeningHarnessCases`): the cases of the list with the name through the Name component, the entry key, the expectation in words ("Must match", "Must not match"), the state and the result in words ("As expected", "Not as expected", "Entry no longer on the list"); "Add a case" (list, entry key, name, optional identifiers, expectation, reason; the reason field carries the hint to write no name in it) and "Retire". The result of a case is its element of `report.bank_cases.results` in the newest `gate` or `oracle` run of the list's active version, joined by case id (`useGetScreeningHarnessRuns`, which S3 already calls); a case that no such run holds yet shows its trial result (`trial.met`). On S1, a decided alert offers "Propose as a known-answer case", which opens the same form filled with the subject name and the entry key the screen already shows, `Must not match` after `incorrect` and `Must match` after `true`. For every S3 reader the harness result of a version gains one line, "Bank cases: n as expected, n not as expected, n entry no longer on the list", from `report.bank_cases`.

12. **Tipping-off review.** A case holds a name. A `no_hit` case is usually a customer's name that matched wrongly; a `hit` case is a spelling of a listed party, which can be typed from any source, so it does not say that the bank had a true match. The name and the identifiers are sealed and one route returns them, to SS and M only. A case holds no party id, no alert id and no account, so nothing ties it to a customer or to an alert. The change request as its approvers read it (the diff of item 5) and the audit events hold the list code, the expectation and the trial result. The stored body of the request also holds the entry key, the sealed query and its keyed hash (item 4 step 7): the diff leaves all three out, and no reader of the request can open the sealed query. `reason_text` is plain text that the approvers read, so the form asks the maker to write no name in it (item 11). The harness report, which CFG, CHK, CH, IA and AS read on S3, holds counts and, for each case id, one of three results. A person outside screening learns that the bank keeps a number of test names for a list, and nothing about a customer, an alert or a decision. This section sends no notification.

13. **Verification** (added to section 6).
   - Migration: `alert.harness_cases` exists with 14 classification rows (`query_enc` encrypted, `query_hk` keyed hash) and its retention row; a `DELETE` fails, as `akr_app` with permission denied (no DELETE grant) and as the owner `akr_migrate` with `known-answer cases are never deleted`; as `akr_app` an UPDATE of the `state` of a `rejected` row fails with the guard's `is final` message; a second open row with the same list, entry key and `query_hk` fails `uq_harness_cases_open`; as `akr_app`, `UPDATE alert.harness_cases SET expect = 'hit'` fails with permission denied; `akr_readonly` cannot select the table.
   - `cargo test`: a `hit` case on a synthetic entry is `met` and counted in `report.bank_cases.met`; the same case after the entry leaves the version is `absent`; a gate run with one unmet bank case still passes (`passed_gate` true) and `failures` does not name the case; `cases` and `recall_clean` are equal with and without bank cases; a `list_setting` change that raises a threshold above a known hit's score carries `known_hit_lost`, lists the case in `impact.known_hits_lost` and is `material`; no audit detail, job payload or log line holds the name of a case (marker test with a unique name).
   - Curl (added to the phase suite, after its Lists step has activated the fixture version of `iq_national`): as SS `POST /api/v1/screening/harness-cases` with `list_code: "iq_national"`, the `entry_key` and the name of one entry of that fixture version, `expect: "hit"` and `Idempotency-Key: hc-0001` -> 202 `ChangeRequestView` on `cr_material`; the same key and body -> the stored 202 with `x-idempotent-replay: true`; the same body with a new key -> 422 with `case_exists`; an unknown entry key -> 422 with `entry_not_found`; a one-token common name -> 422 with `thin_query`; as M and as CHK approve -> the case `active`; as SS `GET /api/v1/screening/harness-cases` -> 200 with the name; as CFG and as IA the GET -> 403 `E_SCOPE`; as SS the retire body for the case -> 202, and for a `rejected` case -> 409 `E_NOT_ALLOWED_IN_STATE`.
   - Swagger UI lists 27 operations for this phase. Audit: `harness_case.proposed`, `harness_case.activated` and `harness_case.retire_proposed` appear, and none holds a name or an entry key.

## Decisions

Recorded in docs/plan/research.md: DR-100 (names and lists: one pure `names` crate with four forms and the closed fold
tables, the strict key `k1` and the loose block key, and the SQL twin `ref.name_key_k1` and `ref.name_key_block_k1`
held equal by the parity corpus under `C.UTF-8`; six seeded lists with optional lists disabled and `pep` a kind only;
the version lifecycle `loading`, `validated`, `active`, `superseded`, `failed` with zero tolerance for conflicting
duplicates and an acknowledged count drop; the known-answer gate before activation; feeds auto-activate after the gate
when the governed setting allows, uploads and the bank snapshot need a second person; the bank's own blacklist is read
only by akr-ingestd's `ingest_bank_list`), DR-101 (screening alerts: a deterministic scorer with the research 22
starting values governed per list and changed only by a dry-run change request; a fingerprint over subject, entry, both
content fingerprints and the matching major, so a rerun inserts nothing and a name change supersedes only undecided
alerts; a thin query is never `Clear`; `true` on a freeze list needs an earlier `potential` by another person and
reaches the `FreezeTrigger` port, recorded until phase 20; `screening_alert` and `pair_memory` are masked kinds from
this phase) and DR-102 (suppression memory: pair memory is pair-keyed, four-eyes, invalidated on subject, entry,
matching-major change and expiry, with a drift count that must be zero; good-guy rules are `material` change targets
(volume effect `decrease`), name patterns only, never on a freeze list, with a capped expiry; bulk `incorrect` only on
identical evidence and never on a freeze list).

Owner decision (2026-10-10), DR-415 (bank corpus cases; with phase 07 section 7.6): a bank adds its own known-answer
cases through change control (section 7.19): a name that must hit, or must not hit, a named list entry, kept sealed
with no link to a party or an alert, proposed by SS or M and approved on `cr_material`. The cases run in every gate and
oracle run and are reported; they never decide the gate, because a list update is a legal duty; a governed change that
would lose a known hit carries a warning and is at least `material`. The shipped corpus stays synthetic (N-38); the
bank-specific variant table (N-57) stays P2.

Lead corrections at review (2026-10-09): column grants added for `ref.lists.feed_format` and `source_ref` (the
`list_setting` activation writes them), `list_versions.expected_count` for akr_app and `diff_summary` for akr_ingest;
the `ListVersion` status mapping is TEXT, not a Postgres enum; the curl case for an admitted reader without a purpose is
422 `E_PURPOSE_REQUIRED`, not 404; phase 11 now emits `canon.touched` (the topic phase 00 assigned to it), which the
onboarding run consumes.

## Open doubts (for gap pass 1 and the bank)

- **National list feed.** Only the Office's Excel export has been read (research 22 section 6.5); the paging API misses about 29 per cent of the list. The formal feed's format, terms and identifiers are unknown; `iq_office_feed` is a reserved parser. Confirm with the Office before go-live.
- **Bank list object.** The ICS BANKS name `CUST_BLA` is one installation's fact (DATA-MODEL s.9); the approved object and mapping come from discovery (phase 05). The bank review of the first snapshot is a 4E step; confirm the reviewer roles.
- **Stored forms in canon.** Phase 10 writes `party_name.key` through `SplitArabicName`; screening recomputes forms from `raw` and ignores the stored keys. Phase 10 should bind its transform to `names` (or the forms should be dropped); confirm.
- **Retention.** Alerts are R7 (DATA-MODEL s.9); ARCHITECTURE s.17 gives sanctions-tied records ten years under R5. Decide whether freeze-list alerts move to a ten-year class.
- **Thresholds, clocks and priorities** (`screening.review_clock_hours` 24, the priority map, the 0.95 and 0.78 thresholds, the evidence floor 10 bits) are starting values; the alert budget must be agreed with the MLRO on the corpus, as research 22 section 13.6 requires.
- **Contradiction bits** (identifier disagreement carries 0 evidence bits in this version) and the unvalidated nationality, place and gender penalties need the bank's own labelled pairs.
- **Risk modifiers** read phase 15 ratings; they are zero until phase 15.
- **Pair-memory PEP cap** of 24 months is phase 15's; the cap here is 12.
- **Bulk incorrect and good-guy rules on freeze lists.** Both are refused or narrowed here; research 22 and 14 are silent on freeze-list suppression by rule; confirm.
- **Masked kinds** registered before phase 15 deviate from phase 03's text; confirm.
- **Auto-activation of feeds** (`screening.feed_auto_activate`) trades the 12-hour rule against a human check; the lead decides the default.
- **Assessor dev user.** No `as@bank.test` exists (as in phase 12), so the assessor reads are tested through `ia@` only.
- **Name component.** Phase 08's component list does not name it; this phase adds it to the layer.
- **Office terms.** The screen uses the 2026 Office terms (research 06 glossary); the CBI guidelines of 2023 use other words; the CBI's terms are the aliases in help.
- **Oracle effort.** The OFAC-emulation oracle (N-40) is non-gating here; its build size is not estimated.
- **N-24 (structured originator fields).** FATF R.16 (June 2025) requires structured originator birth and address fields by end-2030. Capture belongs to the phase 10 canonical payment tables, not to this phase; it is planned, not built, here.

## What This Phase Does NOT Touch

- No freeze tables, freeze clocks, Committee report or freeze release (phase 20); the port and its recording implementation only.
- No PEP register, PEP list semantics or PEP decisions (phase 15); `pep` is a list kind here.
- No risk rating or its modifiers (phase 15); no KYC review.
- No case creation from alerts, no queues, no triage (phases 17 and 18); no alert `case_id` is written here.
- No inline payment screening, no `/assess/v1` and no pre-release hold (phase 29); the index is built so that phase 29 reuses it.
- No scenario or detection logic (phases 13 and 16); no feature use of screening results.
- No change to the canonical tables or their grants (phase 10) beyond reading them; no change to phase 11 tables.
- No new step-up class, no new problem code, no new outbox topic, no new network zone, no compose change, no pg_trgm index (N-53).
- No CBS write and no CBS read other than the approved bank object (`ingest_bank_list`).
- No DR numbers are assigned here; no change to `change.route_map`.
