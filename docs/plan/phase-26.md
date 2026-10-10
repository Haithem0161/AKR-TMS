# Phase 26: Islamic Pack (optional)

**Goal:** A bank with an Islamic book (its own Islamic bank or Islamic windows) gets the Islamic deal model (contract type, deal, parties by role, possession evidence), the Islamic library scenarios with their basis labels, a pooled-fund freeze that respects account types, and a Sharia board view that shows counts only. Everything is off unless the installation's flag `pack.islamic` is on.

**Dependencies:** Phase 00 (jobs, settings, cron schedules, the network zone model), Phase 01 (`StepUpService` and the closed classes `record_open` and `decision`; `admin.users`), Phase 02 (`EvidenceStore::stage` and the evidence owner registry, `admin.evidence_objects`, `av_status`, the audit catalogue with the `siem` flag), Phase 03 (the flag service, `pack.islamic` in the settings, the Cedar bundle and range 80, the `Ctx` and `System` entities, access events), Phase 06 (`scn.scenarios` with `pack` value `islamic`, `requires_flag`, `visibility`, `evidence_label`; the coverage grade of a flagged scenario), Phase 10 (canonical conventions, `canon.product.islamic_contract_type`, `canon.account.islamic_book`, the product code map, `canon.party`, `canon.customer`, `canon.account_party_role`, the `CanonicalEntity` catalogue), Phase 11 (the `CanonicalWriter` call path and mapping replays), Phase 12 (validation rules that record `validation_rule` issues in `admin.dq_issues`), Phase 14 (`daily_base` screening of every party), Phase 15 (the rating model, which takes no Islamic factor), Phase 16 (library versions, `pack.akr_live`, gate G9, the pack seed, `scenario_sweep`), Phase 17 (`casework.effective_tier`, `services::visibility::build`, case families with `islamic`), Phase 18 (the party read predicate of section 3.10 and its Cedar forbids in `91_cases.cedar`, `Party` entity), Phase 20 (`FreezeService`, `casework.ops_tasks` with actions `freeze`, `restrict`, `hold`, `release`, `second_person_only`), Phase 21 (`report.small_number_threshold`), Phase 24 (`admin.registers` warnings with `source_key`, `dashboard.read` tiles of G1, `akr_readonly` rules)

**Macro-phase:** P1 (optional per bank; roadmap row 26)
**Complexity:** M by the template rule (6 routes: 4 rows of API.md 3.18 and two rows added (content, verify); 3 tables in 2 migrations `0083` and `0084`; 1 view; 1 altered CHECK on `canon.product`; 1 job kind; 5 settings rows (section 4.3); 8 library scenario documents; 4 audit kinds; 1 Cedar file). The brief said Size S; see Open doubts 22.
**Binaries touched:** akr-api (routers `islamic`, `sharia_board`; `IslamicDealService`, `PossessionEvidenceService`, `ShariaAggregateQuery` in `AppState`; the pooled branch of `FreezeService`) | akr-worker (job `islamic_evidence_sweep`) | akr-ingestd (unchanged code path: the `CanonicalWriter` library writes the two `canon` tables when phase 11 calls it) | akr-ctl (`packs seed --pack islamic`, `canon verify-catalogue`) | akr-assess unchanged

**Requirements carried:**
- Implemented here: I-01 (taxonomy as the closed `canon.product` list with the four additions of section 1.1; the product code map and its test, section 3.1), I-02 (deal model and parties; evidence status derived; disbursement and supplier-payment records are not built, Open doubts 2), I-03 (partly: suppliers, sellers, developers, partners and contractors are `canon` parties with deal roles; they reach screening through phase 14 without a new path), I-04 (third-party seller rule, scenario SCN-ISL-01), I-05 (possession-evidence tracker with ageing and the missing, late and duplicated warnings, section 3.5; Open doubts 1), I-06 (SCN-ISL-02, SCN-ISL-03, SCN-ISL-04), I-07 (SCN-ISL-05), I-08 (partly: SCN-ISL-06 cap and insiders, SCN-ISL-07 repeat loans; recipient abroad and the bank-level cap are not shipped, Open doubts 4), I-10 (pooled-fund freeze by account class, section 3.8), I-16 (Sharia board aggregates only, section 3.9), I-17 (book flag from phase 10 and the inter-book transfer scenario SCN-ISL-08; the per-product-line onboarding block is not built), I-20 (the eight scenarios as an optional extension with basis label, readiness and fixtures; back-testing and the AML/CFT Office agreement are the bank's, Open doubts 6), I-24 (partly: Islamic fields in the phase 12 and phase 16 coverage views; supplier master resolution is not built), I-26 (data-quality rules of section 3.2 and the gap rate of section 3.9), I-30 (every Islamic rule carries a basis label, section 3.6; no Islamic factor in any score), S-37 (the fairness test of section 3.10)
- Not carried here: I-09 (mudaraba and PSIA integrity checks; Open doubts 5), I-11 (margin engine), I-12 and I-13 (charity module and charity register), I-14 (Hijri baselines), I-15 (Sharia governance register), I-18 (bank-level exposure analytics), I-19 (shareholder screening), I-21 (evidence pack for the assessor, phase 24 owns the pack shape), I-22 (merger support), I-23 (Islamic versus conventional effectiveness metrics), I-25 (Arabic narratives for Islamic alerts), I-27 (alias scope for NPOs), I-28 (rating factors; forbidden by I-30 and DR-249), I-29 (respondent self-view)
- Gap pass 1 (coverage), not carried: K-39 (Islamic inputs to scores are forbidden, DR-249; F3 is withheld, DR-103).
- Gap pass 1 (coverage), partly implemented: VA-26 (Islamic contract types in the product list as scenario inputs).

---

## 1. SQLx Migration Changes

Two migrations, `0083_canon_islamic_deals` and `0084_kyc_possession_evidence`, numbered after phase 25's `0082` (the lead's range 0083 to 0085 allows a third; none is needed). The two tables of the deal model are in `canon` because they are CBS-sourced reference-and-deal facts written by the `CanonicalWriter` through `akr_ingest` (phase 10 conventions). The possession-evidence table is in `kyc`, not `canon`: an analyst writes it, and `canon` is read-only for `akr_app` (phase 10 section 3.7), so a verified status could never be stored on a canonical row. DATA-MODEL section 20 lists `possession_evidence` under `canon`; this phase moves it (DR-246). The only existing table altered is `canon.product` (phase 10), and only its CHECK, as stated in 1.1.

### 1.1 Migration `0083_canon_islamic_deals.sql`

```sql
-- migration 0083_canon_islamic_deals.sql (phase 26). Islamic deals and deal parties (DATA-MODEL s.20).
-- Written by the CanonicalWriter through akr_ingest (phase 10 section 1 conventions). The application never writes here.

-- 1. The contract-type list of canon.product (phase 10) gains four values. The phase 10 list is the base; the change is
--    stated in DR-247. The constraint name is the default name PostgreSQL gives the inline column CHECK of phase 10.
ALTER TABLE canon.product DROP CONSTRAINT IF EXISTS product_islamic_contract_type_check;
ALTER TABLE canon.product ADD CONSTRAINT product_islamic_contract_type_check CHECK (islamic_contract_type IN (
    'murabaha', 'musharaka', 'mudaraba', 'ijara', 'qard_hasan', 'istisna', 'salam', 'other',
    'tawarruq', 'diminishing_musharaka', 'wakala', 'wadiah'));

-- 2. Deals (R3). Bi-temporal like canon.account: one stable id per agreement, one row per version (phase 10 section 2).
--    One agreement is one deal and is never also a canon.facility row (DATA-MODEL s.20). outstanding stays null elsewhere.
CREATE TABLE IF NOT EXISTS canon.islamic_deals (
    id                  UUID NOT NULL,
    customer_id         UUID NOT NULL,                  -- no foreign key: canon.customer is bi-temporal (phase 10 section 2)
    account_id          UUID,                           -- financing or investment account; null when the source holds none
    product_id          UUID NOT NULL REFERENCES canon.product(id) ON DELETE RESTRICT,   -- contract type comes from here
    contract_type       TEXT NOT NULL CHECK (contract_type IN ('murabaha', 'musharaka', 'mudaraba', 'ijara', 'qard_hasan',
                                                               'istisna', 'salam', 'other', 'tawarruq',
                                                               'diminishing_musharaka', 'wakala', 'wadiah')),
    status              TEXT NOT NULL CHECK (status IN ('active', 'settled', 'closed', 'defaulted', 'cancelled', 'unknown')),
    asset_ref           TEXT CHECK (asset_ref IS NULL OR length(btrim(asset_ref)) BETWEEN 1 AND 80),   -- null = missing (I-26)
    asset_class         TEXT NOT NULL DEFAULT 'unknown'
                        CHECK (asset_class IN ('real_estate', 'vehicle', 'commodity', 'equipment', 'goods', 'other', 'unknown')),
    currency            CHAR(3) NOT NULL REFERENCES ref.currencies(code) ON DELETE RESTRICT,
    settlement_currency CHAR(3) REFERENCES ref.currencies(code) ON DELETE RESTRICT,   -- null = not in the source: unknown, never equal
    contract_amount     NUMERIC(28,6) CHECK (contract_amount IS NULL OR contract_amount >= 0),
    deferred_price      NUMERIC(28,6) CHECK (deferred_price IS NULL OR deferred_price >= 0),
    opened_on           DATE NOT NULL,
    break_even_on       DATE,                           -- real-estate two-year rule (SCN-ISL-05); null until mapped
    closed_on           DATE,                           -- the closure clock of retention (section 1.4)
    sharia_approval_ref TEXT CHECK (sharia_approval_ref IS NULL OR length(btrim(sharia_approval_ref)) BETWEEN 1 AND 60),
    valid_from          DATE NOT NULL,
    valid_to            DATE,
    recorded_from       TIMESTAMPTZ NOT NULL,
    recorded_to         TIMESTAMPTZ,
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (id, recorded_from),
    CHECK (valid_to IS NULL OR valid_to > valid_from),
    CHECK (recorded_to IS NULL OR recorded_to > recorded_from),
    CHECK (closed_on IS NULL OR closed_on >= opened_on),
    CHECK (status <> 'closed' OR closed_on IS NOT NULL)
);

-- 3. Deal parties (R3). Parties are the bi-temporal canon.party: no foreign key, as phase 10 section 2 requires.
--    Child rows are replaced by hash, never updated in place (phase 10 section 3.7 step 6): superseded_at marks the old row.
CREATE TABLE IF NOT EXISTS canon.deal_parties (
    id                  UUID PRIMARY KEY,
    deal_id             UUID NOT NULL,                  -- no foreign key: canon.islamic_deals is bi-temporal
    party_id            UUID NOT NULL,                  -- no foreign key: canon.party is bi-temporal
    role                TEXT NOT NULL CHECK (role IN ('customer', 'seller', 'buyer', 'agent', 'supplier', 'developer',
                                                     'contractor', 'partner', 'broker')),
    source_id           UUID NOT NULL REFERENCES admin.sources(id) ON DELETE RESTRICT,
    source_table        TEXT NOT NULL,
    source_pk           TEXT NOT NULL,
    source_version      TEXT,
    row_hash            BYTEA NOT NULL,
    batch_id            UUID NOT NULL,
    extracted_at        TIMESTAMPTZ NOT NULL,
    mapping_version_id  UUID NOT NULL REFERENCES admin.mapping_versions(id) ON DELETE RESTRICT,
    superseded_at       TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 4. Lineage and access-path indexes (phase 10 loop, written out for the two tables).
CREATE INDEX IF NOT EXISTS idx_islamic_deals_src      ON canon.islamic_deals (source_id, source_table, source_pk);
CREATE INDEX IF NOT EXISTS idx_islamic_deals_mv       ON canon.islamic_deals (mapping_version_id);
CREATE INDEX IF NOT EXISTS idx_islamic_deals_customer ON canon.islamic_deals (customer_id);
CREATE INDEX IF NOT EXISTS idx_islamic_deals_product  ON canon.islamic_deals (product_id);
CREATE UNIQUE INDEX IF NOT EXISTS uq_islamic_deals_current ON canon.islamic_deals (source_id, source_table, source_pk)
    WHERE recorded_to IS NULL;
CREATE INDEX IF NOT EXISTS idx_deal_parties_src       ON canon.deal_parties (source_id, source_table, source_pk);
CREATE INDEX IF NOT EXISTS idx_deal_parties_mv        ON canon.deal_parties (mapping_version_id);
CREATE INDEX IF NOT EXISTS idx_deal_parties_deal      ON canon.deal_parties (deal_id) WHERE superseded_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_deal_parties_party     ON canon.deal_parties (party_id) WHERE superseded_at IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uq_deal_parties_current ON canon.deal_parties (source_id, source_table, source_pk, role)
    WHERE superseded_at IS NULL;

-- 5. Grants. Writes come from akr_ingest only (the CanonicalWriter); akr_app reads; akr_readonly gets nothing (section 1.5).
GRANT SELECT, INSERT ON canon.islamic_deals, canon.deal_parties TO akr_ingest;
GRANT UPDATE (recorded_to, valid_to) ON canon.islamic_deals TO akr_ingest;
GRANT UPDATE (superseded_at) ON canon.deal_parties TO akr_ingest;
GRANT SELECT ON canon.islamic_deals, canon.deal_parties TO akr_app;
-- DELETE: akr_retention only, as phase 23 gives it for canon (section 1.4). No other role.
GRANT SELECT, DELETE ON canon.islamic_deals, canon.deal_parties TO akr_retention;   -- phase 23's SELECT ON ALL TABLES predates these
```

### 1.2 Classification and retention rows (0083, copy of the phase 25 insert format)

```sql
-- Classification. Amounts are financial; the asset reference is personal (a plot or vehicle identifies a person).
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'canon', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('islamic_deals', 'internal', ARRAY['id', 'customer_id', 'account_id', 'product_id', 'contract_type', 'status',
                                      'asset_class', 'currency', 'settlement_currency', 'opened_on', 'break_even_on',
                                      'closed_on', 'sharia_approval_ref', 'valid_from', 'valid_to', 'recorded_from',
                                      'recorded_to', 'source_id', 'source_table', 'source_pk', 'source_version',
                                      'row_hash', 'batch_id', 'extracted_at', 'mapping_version_id', 'created_at']),
  ('islamic_deals', 'financial', ARRAY['contract_amount', 'deferred_price']),
  ('islamic_deals', 'personal', ARRAY['asset_ref']),
  ('deal_parties', 'internal', ARRAY['id', 'deal_id', 'party_id', 'role', 'source_id', 'source_table', 'source_pk',
                                     'source_version', 'row_hash', 'batch_id', 'extracted_at', 'mapping_version_id',
                                     'superseded_at', 'created_at'])
) AS v(t, cls, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

-- Retention. Deals are master data of the financing relationship (R3). The closure clock is the deal's closure; an open
-- deal (closed_on NULL) is never due. Parties follow their deal.
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('canon', 'islamic_deals', 'R3', 'closure', 'closed_on', 'Seven years after the deal closes; an open deal is never due'),
 ('canon', 'deal_parties',  'R3', 'owner',   NULL,        'Follows its deal; superseded rows follow the same owner')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

Note on the classification: the column list of `islamic_deals` does not repeat `asset_ref` in the `internal` list, and the
`VALUES` rows cover every column of the two tables; a test (section 6, check 2) compares the list with
`information_schema.columns` so a column added later without a row fails.

### 1.3 Migration `0084_kyc_possession_evidence.sql`

```sql
-- migration 0084_kyc_possession_evidence.sql (phase 26). Possession evidence written by analysts (I-05), and the derived
-- evidence status of a deal. canon stays read-only for akr_app; a CBS row can never carry a verified status (DR-246).

CREATE TABLE IF NOT EXISTS kyc.possession_evidence (
    id                  UUID PRIMARY KEY,               -- UUIDv7, app-side (DR-06)
    deal_id             UUID NOT NULL,                  -- canon.islamic_deals.id; no foreign key (bi-temporal parent)
    kind                TEXT NOT NULL CHECK (kind IN ('invoice', 'delivery_note', 'title', 'registration', 'lease',
                                                      'valuation', 'shipping', 'other')),
    evidence_object_id  UUID NOT NULL REFERENCES admin.evidence_objects(id) ON DELETE RESTRICT,   -- phase 02, owner kind islamic_deal
    document_ref_hk     BYTEA,                          -- HMAC of the document number (match key for duplicates); null if none given
    status              TEXT NOT NULL DEFAULT 'attached' CHECK (status IN ('attached', 'verified', 'rejected')),
    reject_code         TEXT CHECK (reject_code IN ('evidence_mismatch', 'evidence_unreadable', 'evidence_not_original')),
    attached_by         UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT,
    attached_at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    decided_by          UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    decided_at          TIMESTAMPTZ,
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    version             INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    CHECK ((status = 'attached') = (decided_by IS NULL AND decided_at IS NULL)),
    CHECK ((status = 'rejected') = (reject_code IS NOT NULL)),
    CHECK (decided_by IS NULL OR decided_by <> attached_by)   -- four eyes at the database too: the attacher never decides
);

CREATE INDEX IF NOT EXISTS idx_possession_deal       ON kyc.possession_evidence (deal_id);
CREATE INDEX IF NOT EXISTS idx_possession_object     ON kyc.possession_evidence (evidence_object_id);
CREATE INDEX IF NOT EXISTS idx_possession_attached   ON kyc.possession_evidence (attached_by);
CREATE INDEX IF NOT EXISTS idx_possession_decided    ON kyc.possession_evidence (decided_by) WHERE decided_by IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_possession_doc        ON kyc.possession_evidence (document_ref_hk)
    WHERE document_ref_hk IS NOT NULL AND status <> 'rejected';

-- Derived status of each current deal. Only an analyst's verified row gives 'verified'. A CBS-sourced deal with no
-- analyst row is 'unknown', never 'verified' and never 'missing' (missing is a register warning of section 3.5).
CREATE OR REPLACE VIEW kyc.deal_evidence_status AS
SELECT d.id AS deal_id,
       CASE WHEN EXISTS (SELECT 1 FROM kyc.possession_evidence p WHERE p.deal_id = d.id AND p.status = 'verified') THEN 'verified'
            WHEN EXISTS (SELECT 1 FROM kyc.possession_evidence p WHERE p.deal_id = d.id AND p.status = 'attached') THEN 'attached'
            ELSE 'unknown'
       END AS evidence_status
FROM canon.islamic_deals d
WHERE d.recorded_to IS NULL;

-- Grants. akr_app inserts an attachment and changes only its decision columns; nothing else is updatable.
GRANT SELECT, INSERT ON kyc.possession_evidence TO akr_app;
GRANT UPDATE (status, reject_code, decided_by, decided_at, updated_at, version) ON kyc.possession_evidence TO akr_app;
GRANT SELECT ON kyc.deal_evidence_status TO akr_app;
GRANT SELECT, DELETE ON kyc.possession_evidence TO akr_retention;   -- phase 23 role; its SELECT ON ALL TABLES predates this table
-- akr_readonly: no grant (never widened, phase 24 rule).

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'kyc', v.t, c, v.cls::admin.data_classification
FROM (VALUES
  ('possession_evidence', 'internal', ARRAY['id', 'deal_id', 'kind', 'evidence_object_id', 'status', 'reject_code',
                                            'attached_by', 'attached_at', 'decided_by', 'decided_at', 'updated_at', 'version']),
  ('possession_evidence', 'personal', ARRAY['document_ref_hk'])
) AS v(t, cls, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

-- Retention: the record follows its deal (owner clock). The evidence file keeps its own class (R3 lock, section 2).
INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('kyc', 'possession_evidence', 'R3', 'owner', NULL, 'Follows its deal; the evidence object is R3 under evidence.lock_days')
ON CONFLICT (schema_name, table_name) DO NOTHING;
```

### 1.4 Retention and holds

- `canon.islamic_deals` and `canon.deal_parties` are R3 (DATA-MODEL section 20; ARCHITECTURE R3). Their rows are deleted only by
  `akr_retention` after the closure clock (phase 23 engine). A legal hold of phase 23 that names a customer holds that customer's deal rows,
  as it holds the customer's accounts.
- `kyc.possession_evidence` follows its deal. Its evidence object is R3 and locked for `evidence.lock_days` (2555 days, phase 02),
  so the file outlives the row only by the lock; the row is not deleted before the object's lock ends.

### 1.5 Grants summary

| Table or view | akr_app | akr_ingest | akr_retention | akr_readonly |
|-|-|-|-|-|
| `canon.islamic_deals` | SELECT | SELECT, INSERT; UPDATE (recorded_to, valid_to) | SELECT, DELETE | none |
| `canon.deal_parties` | SELECT | SELECT, INSERT; UPDATE (superseded_at) | SELECT, DELETE | none |
| `kyc.possession_evidence` | SELECT, INSERT; UPDATE (status, reject_code, decided_by, decided_at, updated_at, version) | none | SELECT, DELETE | none |
| `kyc.deal_evidence_status` (view) | SELECT | none | none | none |
| `canon.product` (CHECK only) | unchanged | unchanged | unchanged | unchanged |

`akr_readonly` gets no new grant: the pack has no report and no assessor view of its own (I-21 is not carried).

## 2. DDD Implementation (Rust)

- **Crate placement** (phase 00 workspace): domain types and pure rules in `domain::islamic` (akr-core, no I/O: the contract-type
  list, the evidence-status derivation, the deal-party role list, the small-cell suppression call, the account-class mapping);
  repositories ports in the same module; implementations in `infrastructure::pg::islamic`; HTTP in `api::routes::islamic` and
  `api::routes::sharia_board`; the sweep in `worker::islamic`.
- **Domain entities** (`Debug, Serialize, Deserialize, FromRow, ToSchema`):
  - `IslamicDeal { id, customer_id, account_id: Option<Uuid>, product_id, contract_type: ContractType, status: DealStatus,
    asset_ref: Option<String>, asset_class: AssetClass, currency: String, settlement_currency: Option<String>,
    contract_amount: Option<Decimal>, deferred_price: Option<Decimal>, opened_on: NaiveDate, break_even_on: Option<NaiveDate>,
    closed_on: Option<NaiveDate>, sharia_approval_ref: Option<String>, valid_from, valid_to, recorded_from, recorded_to,
    source_id, source_table, source_pk, source_version, row_hash, batch_id, extracted_at, mapping_version_id, created_at }`
  - `DealParty { id, deal_id, party_id, role: DealRole, superseded_at }`
  - `PossessionEvidence { id, deal_id, kind: EvidenceKind, evidence_object_id, status: EvidenceStatus, reject_code: Option<RejectCode>,
    attached_by, attached_at, decided_by: Option<Uuid>, decided_at: Option<DateTime<Utc>>, version }`
  - `DealEvidenceStatus { deal_id, evidence_status: Unknown | Attached | Verified }` (read from the view `kyc.deal_evidence_status`;
    `missing` is never a stored or returned value)
  - Enums (Rust, `#[sqlx(type_name = ...)]` not needed: TEXT with CHECK): `ContractType` (the twelve values of section 1.1), `DealStatus`,
    `AssetClass`, `DealRole`, `EvidenceKind`, `EvidenceStatus` (`attached`, `verified`, `rejected`), `RejectCode`, `AccountClass`
    (`unrestricted`, `restricted`, `pooled`).
- **Create and decision DTOs** (`Deserialize, ToSchema`):
  - `AttachEvidence { kind: EvidenceKind, document_ref: Option<String> (max 60 characters, trimmed) }`, sent as multipart with the
    file part `file`.
  - `DecideEvidence { decision: "verified" | "rejected", reject_code: Option<RejectCode> }` (`reject_code` required when rejected,
    refused when verified; 422 `E_VALIDATION` at `/reject_code`).
- **Repository traits** (domain):
  - `IslamicDealRepository`: `current(id) -> Option<IslamicDeal>`; `list_for_customer(customer_id, after, limit) -> Vec<IslamicDeal>`;
    `parties(deal_id) -> Vec<DealParty>` (current, `superseded_at IS NULL`); `lock_for_share(tx, deal_id)`.
  - `PossessionEvidenceRepository`: `insert(tx, NewPossessionEvidence) -> PossessionEvidence`; `get_for_update(tx, id)`;
    `decide(tx, id, expected_version, decision) -> PossessionEvidence`; `list_for_deal(deal_id)`.
    No duplicate method lives here: the tile method `islamic_duplicate_refs` of section 7+ counts the shared references when G1 is read.
  - `ShariaAggregateRepository`: `counts(period) -> AggregateCounts` (one query with grouped counts; section 3.9).
- **Repository implementations** (`Pg*`): `PgIslamicDealRepository` reads `canon.islamic_deals` with `recorded_to IS NULL`;
  `PgPossessionEvidenceRepository` uses `UPDATE ... WHERE id = $1 AND version = $2 RETURNING *` (version check in SQL);
  `PgShariaAggregateRepository` reads the deal tables and the view in one statement with `GROUP BY` and no row ids in the result.
- **Ports** (defined here, implemented by the phases named): `EvidenceStore::stage` (phase 02, owner kind `islamic_deal`),
  `StepUpService::consume` (phase 01), `PartyReadPredicate::allows(principal, party_id, context)` (phase 18 section 3.10 party rule, phase 18 section 7+),
  `RegisterWriter::warn(tx, source, source_key, title_key, body)` (phase 24 section 7+, DR-388; insert-or-nothing with
  `ON CONFLICT (source_key) WHERE source_key IS NOT NULL DO NOTHING`).
- **Pure rules** (`domain::islamic`, unit-tested): `derive_evidence_status(rows) -> DealEvidenceStatus` (the view's rule, so the
  application and the SQL give the same answer; a test pins both over generated rows); `suppress_small(count, threshold) -> Option<i64>`;
  `account_class(product_type: Option<ContractType>, map: &AccountClassMap) -> AccountClass`.
- **utoipa schemas**: `IslamicDealView` (deal fields, `evidence_status`, `parties[]` with role and party id only for the parties the
  caller may read, `evidence[]`), `IslamicDealSummary` (list rows), `PossessionEvidenceView` (id, kind, status, reject code, attached
  and decided instants, `attached_by` and `decided_by` as ids; no file name, no document reference),
  `ShariaAggregateView` (section 3.9), `ApiError`.
- **Route table** (scopes from API.md section 2; four rows are API.md 3.18, the content and verify rows are added):

| Method | Path | Scope | Description |
|-|-|-|-|
| GET | `/api/v1/islamic/deals` | `queue.work` | Deals of one customer (`?customer_id=`), cursor; the rows the caller may read under the party rule (`islamic.deal.read`) |
| GET | `/api/v1/islamic/deals/{id}` | `queue.work` | Deal view: parties by role (only those the caller may read), asset, evidence status and evidence rows (`islamic.deal.read`; **SU** `record_open` on a suspicion-tier deal) |
| POST | `/api/v1/islamic/deals/{id}/evidence` | `queue.work` | Attach possession evidence: stage the file through `EvidenceStore::stage` and insert the attachment; `Idempotency-Key`, multipart (`islamic.evidence.attach`) |
| GET | `/api/v1/islamic/deals/{id}/evidence/{evidence_id}/content` (added) | `queue.work` | The verified bytes of one attachment through `EvidenceStore::open`; access event first; **SU** `record_open` on a suspicion-tier deal; `download=true` records `download` (`islamic.evidence.read`; phase 18 content pattern) |
| POST | `/api/v1/islamic/deals/{id}/evidence/{evidence_id}/verify` (added) | `queue.work` | Verify or reject an attachment made by another person; **SU** `decision`; `If-Match`; `Idempotency-Key` (`islamic.evidence.verify`) |
| GET | `/api/v1/islamic/sharia-board/aggregates` | `dashboard.read` | Aggregate counts for a bank month, small cells suppressed; no identifiers (`sharia.aggregate.read`) |

- **Router**: `pub fn router() -> Router<AppState>` in `api::routes::islamic` and `api::routes::sharia_board`. Both routers are always
  mounted and are guarded per request by the flag (section 3.7 step 5): while the flag is off every path answers 404 `E_NOT_FOUND` with
  the body of an unknown id.

## 3. Business Logic

### 3.1 Product map and contract type (I-01; `ProductTypeMap`, phase 10 code maps)

1. Each bank product code reaches `canon.product` through the `product` code map (`admin.code_maps` kind `product`, phase 10
   section 1, DSL B.7). The canonical product row carries `islamic_contract_type` (phase 10 `canon.product`, the list widened by
   section 1.1). A deal's `contract_type` is copied from its product row when the deal is written; the deal never sets a type of its own.
2. Where the CBS keeps no contract type on the agreement or the account (the ICS BANKS installation studied: Qard Hasan without an
   agreement table, empty Mudaraba tables; `canon.facility.contract_type` is free text and is not used for this), the type comes
   from the product row only. A product code whose row has `islamic_contract_type` NULL is refused for
   deals: the writer returns `unmapped_code` (phase 12 kind, `field` = `product`), the row is not written, and the count shows on the
   DQ register. A change of the type is a change request of the code map (phase 10 `cr_mapping`).
3. Test `islamic_product_map_is_complete` (`cargo test`, and `akr-ctl mapping lint` with the same check): (a) every source
   product code in the fixture maps to exactly one canonical product; (b) every product row that a fixture deal uses has a non-NULL type from the
   twelve values; (c) every value of the twelve has at least one fixture deal; (d) a code mapped to two types with the same `valid_from` is refused by
   `uq_code_map_entries_effective` (phase 10).
4. The taxonomy of I-01 ("risk class and rule pack per class") is carried by the scenario families (section 3.6), not by a new
   column: a risk class per contract type would be a rating input, which I-30 and DR-249 forbid.

### 3.2 Deal load (`CanonicalWriter`, phase 10 and phase 11) and data-quality rules (I-26)

1. Two mapping families are added: `<source>.islamic_deal` (target entity `islamic_deal`) and `<source>.deal_party` (target entity
   `deal_party`), one active family per source and entity (phase 10). `evidence_status` is not a catalogue column, so no mapping can
   write it. `source_pk` follows the phase 10 rule (key parts joined by `|`).
2. The `CanonicalEntity` closed enum (phase 10, 29 names) gains `islamic_deal` and `deal_party` at the end of the catalogue order.
   `canonical-model.v1.json` gets two entries (table, columns, classification, nullability). `akr-ctl canon verify-catalogue` compares
   the file with `information_schema` and fails on any column without an entry (section 6, check 2).
3. The writer follows phase 10 section 3.7: a changed hash closes the current `canon.islamic_deals` version (`recorded_to`) and
   inserts the next; a changed party row is marked `superseded_at` and a new row is inserted. A deal whose product has no type
   is refused (3.1 step 2).
4. Data-quality rules (DSL B.5 rules with `on_fail: dq_issue`, phase 12 `DataQualityRecorder`): the rule failures are recorded as
   `validation_rule` issues, `field` = the attribute name, with the rule code in `detail.code`. The `kind` list of phase 12 is
   unchanged (it is a CHECK). The codes are:

| Rule code (`detail.code`) | Test | Field | Severity |
|-|-|-|-|
| `islamic.asset_missing` | `asset_ref` NULL on a deal of a contract type in {murabaha, tawarruq, ijara, istisna, salam} | `asset_ref` | medium |
| `islamic.supplier_missing` | no current `deal_party` with role `supplier` or `seller` on a deal of the same types | `deal_party.role` | medium |
| `islamic.settlement_ccy_missing` | `settlement_currency` NULL on a murabaha deal | `settlement_currency` | low |

5. Missing possession evidence is not a data-quality rule: it is not a source field (the source holds no evidence) and it is handled by
   section 3.5. A high share of deals with missing asset, supplier or evidence is shown on the Sharia board view as a control failure
   (section 3.9), which is the I-26 presentation.

### 3.3 Parties and screening (I-03; phase 14 unchanged)

1. Every party a deal names is a `canon.party` row (the seller, supplier, developer, contractor, partner, broker or agent), with the
   phase 10 identifiers and contacts. A deal party is never a new subject kind.
2. Screening needs no new run: phase 14 `daily_base` is a keyset over every current party id of `canon.party` (chunks of 5,000) and
   the `onboarding` run queues from each mapped batch's `canon.touched` event. A supplier or seller added by a deal is therefore screened
   by the existing runs. The screening subject is the party, not the deal, so the screening alert, its decision and its freeze (phase 20)
   are unchanged.
3. Deal roles do not change any screening rule. A party of role `supplier` with a list hit is handled exactly as any party with a hit
   (phase 14 section 3.6); the deal panel shows the screening status only through phase 14's own party read rules.

### 3.4 Deal read and the deal panel (`IslamicDealService`; W3 and W4 panels)

`IslamicDealService::get(ctx, deal_id)` (route GET `/islamic/deals/{id}`), with `X-Access-Purpose` required (phase 03 purpose codes
`assigned_work` or `investigation`; 422 `E_PURPOSE_REQUIRED` without it):

1. Flag: read `pack.islamic`; when off, return 404 `E_NOT_FOUND` with the body of an unknown id (no access event for the flag).
2. Read `canon.islamic_deals` where `id = $1 AND recorded_to IS NULL`. Missing: 404 with the same body.
3. `customer_party = SELECT party_id FROM canon.customer WHERE id = deal.customer_id AND recorded_to IS NULL`.
4. `tier = casework.effective_tier(customer_party)` (phase 17). The deal's tier is its customer's party tier; a deal is never
   more visible than its customer.
5. Cedar `islamic.deal.read` on `Akr::Party { tier, case_team }` with `context { purpose, network }` (section 3.13). A refusal is written
   as a masked access event and answered 404 `E_NOT_FOUND` with the body of an unknown id (the phase 18 party rule, section 3.10).
6. Tier `suspicion`: `StepUpService::consume(tx, ctx, header, DEAL_OPEN, Some("islamic_deal:<id>"))`; a missing assertion is 401
   `E_STEPUP_REQUIRED`.
7. Parties: `SELECT ... FROM canon.deal_parties WHERE deal_id = $1 AND superseded_at IS NULL`. For each party, call
   `PartyReadPredicate::allows(principal, party_id, context)` (phase 18 party rule). A party the caller may not read is omitted from the list, and
   no count of omitted parties is returned. A readable party shows its role, its id and its display name from the current `canon.party_name` rows (`superseded_at IS NULL`).
8. Evidence: `kyc.possession_evidence` rows of the deal (status, kind, reject code, attached and decided instants, the two user ids);
   `evidence_status` from `kyc.deal_evidence_status`. No file name and no document reference are returned; each row carries its
   own `id`, and the file is opened through `GET /islamic/deals/{id}/evidence/{evidence_id}/content` (below), which repeats this
   read's checks (lead review: no generic `GET /evidence/{id}` exists; phase 18 serves content per owner).
9. Access event (phase 02 `access_event`, action `view`, resource_type `islamic_deal`, with purpose and tier). The read writes no
   audit row.
10. Response `IslamicDealView`. The deal panel of W3 and the tab of W4 use this one read.

`GET /islamic/deals?customer_id=` (list): the steps of the list block of section 7+, for the one customer named;
a non-readable customer returns the same empty page a customer without deals returns, so the list shows nothing the page would
refuse.

### 3.5 Possession evidence (I-05; `PossessionEvidenceService`, sweep `islamic_evidence_sweep`)

**Attach** (`POST /islamic/deals/{id}/evidence`, `Idempotency-Key` required):

1. Flag and deal read: steps 1 to 5 of 3.4 (a refused deal is 404). Cedar `islamic.evidence.attach` (A1, A2, M). No step-up: an attachment is
   a record, and its decision is a later separate act.
2. Validate `kind` (422 `E_VALIDATION` at `/kind`, sub-code `kind_invalid`) and `document_ref` (at most 60 characters after trim; sub-code
   `document_ref_too_long`).
3. One transaction. Lock order: the deal row `FOR SHARE` first, then the evidence rows.
4. `EvidenceStore::stage(tx, NewEvidence { kind: upload, owner_kind: "islamic_deal", owner_id: deal_id, retention_class: R3, content_type,
   file_name, classification: personal, lang, bytes }, ctx.user)` (phase 02 section 3). Refusals: `E_UPLOAD_REJECTED` (422, antivirus or type),
   `E_PAYLOAD_TOO_LARGE` (413). The owner kind `islamic_deal` is added to the phase 02 evidence owner registry (section 4.5).
5. `document_ref_hk = HMAC-SHA256(key purpose "islamic.document_ref", normalised document_ref)` (phase 00 `Crypto::keyed_hash`; normalised means the phase 10 transforms `Trim`, `DigitsWestern` and `Upper`, in that order, with inner characters kept as typed), or NULL when no
   reference is given.
6. Insert `kyc.possession_evidence` (`status = attached`, `attached_by = ctx.user`, `attached_at = now()`).
7. Duplicates: the attach makes no duplicate read and stores no mark. A reference that a row of another deal also holds (rows not rejected) shows only in the G1 tile
   count of section 5 (for M). It is not a register row, not a notification, not in the response and not in the audit detail: the
   attacher, the audit readers and internal audit must not learn that the same document number appears on another deal (3.14).
8. Late check: when `islamic.evidence_tracking` is on and `attached_at::date - deal.opened_on > islamic.evidence_due_days`, the row is
   counted as late in the next sweep (step 3 of the sweep); the attach response does not say so.
9. Audit `islamic.evidence.attached` (section 3.12). Return 201 `PossessionEvidenceView` with `status = attached`. The body is the same
   whether or not another deal holds the same reference.

**Content** (`GET /islamic/deals/{id}/evidence/{evidence_id}/content`; Cedar `islamic.evidence.read`; the phase 18 content pattern):

1. Steps 1 to 6 of 3.4 (flag, deal read, tier, Cedar, purpose, step-up `record_open` on a suspicion-tier deal); refusal 404 with the
   unknown-id body.
2. The evidence row must belong to this deal (404 otherwise; the same body).
3. Access event first (action `view`, or `download` with `download=true`; resource_type `possession_evidence`), then
   `EvidenceStore::open` (phase 02: decrypt, authenticate every segment, check the SHA-256). An object not `clean` is 409
   `E_NOT_ALLOWED_IN_STATE` (`evidence_not_clean`). `Cache-Control: no-store`.

**Verify** (`POST /islamic/deals/{id}/evidence/{evidence_id}/verify`, `If-Match`, `Idempotency-Key`, body `DecideEvidence`):

1. Flag and deal read (3.4 steps 1 to 5, refusal 404). Cedar `islamic.evidence.verify` (A2, M). An A1 caller on a visible deal gets
   403 `E_FORBIDDEN` (Cedar denied on a resource the caller may know exists).
2. `If-Match` absent: 428 `E_PRECONDITION_REQUIRED`. Step-up `EVIDENCE_DECIDE` (class `decision`) consumed with resource ref `possession_evidence:<id>` (401 on
   a missing or spent assertion).
3. One transaction. Lock order: the deal row `FOR SHARE`, then the evidence row `FOR UPDATE`.
4. The row must exist on this deal (404 otherwise). `version` differs from `If-Match`: 409 `E_VERSION_CONFLICT` with the current version.
5. `status` is not `attached`: 409 `E_NOT_ALLOWED_IN_STATE` (sub-code `already_decided`); a decided row is never decided again.
6. `ctx.user = attached_by`: 403 `E_SELF_APPROVAL`. The database CHECK `decided_by <> attached_by` is the second line.
7. The evidence object must be `av_status = 'clean'` (phase 02): otherwise 409 `E_NOT_ALLOWED_IN_STATE` (sub-code `evidence_not_clean`).
8. Decision `verified`: `status = verified`; decision `rejected`: `status = rejected` and `reject_code` required (422 `E_VALIDATION` at
   `/reject_code` when missing, and refused when the decision is `verified`). `decided_by`, `decided_at`, `version + 1`.
9. Audit `islamic.evidence.verified` or `islamic.evidence.rejected` (siem yes). Return 200 `PossessionEvidenceView`.

A verified row is the only input that makes `kyc.deal_evidence_status` return `verified`. A CBS row cannot reach that state: `akr_ingest`
has no write on `kyc` and no mapping target for `evidence_status` exists (section 3.2 step 1), so a test asserts both (section 6, check 6 l).

**Sweep** (`islamic_evidence_sweep`, daily, section 4.1):

1. Return at once when `islamic.evidence_tracking` is FALSE (the default). Until the bank declares its evidence process live by change request,
   `unknown` means "not known", not "missing", and no warning is written.
2. Return at once when `pack.islamic` is off.
3. Count current, `active` deals whose due date `BusinessClock::add(opened_on, islamic.evidence_due_days, Days)` (phase 09; forward
   only) is before today and
   `kyc.deal_evidence_status = 'unknown'` (missing), and attachments whose `attached_at` is later than their deal's due date (late). Only
   counts are kept.
4. When either count is above zero, insert one register warning (`admin.registers`, kind `warning`, source `scan` (phase 24's
   closed list; lead review), `source_key = 'islamic_evidence_gap:' || bank_date`, `ON CONFLICT (source_key) WHERE source_key IS NOT NULL DO NOTHING` (written by phase 24's `warn`, block 7.2), fixed
   bilingual title, body `{"missing": n, "late": n}`). One row per bank day, no deal id, visible to M, CH and IA through `GET /registers/warnings` (phase 24).
5. Audit `islamic.evidence.swept` with the two counts (siem no), once per run that wrote a row.

Why a register warning and not an alert (decision DR-248): an alert is a finding about a customer's conduct and goes to the case
queue and to the suspicion rules of phase 17. A missing document is a process gap, which the bank closes with the customer or the
branch. The warning is counts only, so it tells internal audit how many deals lack evidence without naming one (DR-219 of phase 24
gives IA counts only for suspicion matters). Open doubts 1 records the bank's option to make it an alert.

### 3.6 Library scenarios (I-06, I-07, I-08, I-17, I-04, I-20, I-30; the `ISL` family)

The pack has eight scenario documents in `packs/scenarios/islamic.v1.0.0.json` (signed release data, phase 16 section 3.15), family
`ISL`, `pack: islamic`, `requires_flag: "pack.islamic"`, `research_grade` per phase 06 (A to D), `evidence_label` required (S = supervisory
text, the CBI 2018 circular; N = analysis, to be validated on the bank's cases; phase 16 rule 15 applies to `islamic` as to `akr`). The
English names are given; the Arabic names and summaries are written by compliance (Open doubts 7). Each document carries
positive and negative fixtures (phase 16 section 3.1) and declares its `data_requirements` (every read is listed; DSL A.1).

| Code | Scenario (logic) | Requirement | Basis label (evidence label) | Canonical fields read (`data_requirements`) | External needs and readiness | Visibility | Latency |
|-|-|-|-|-|-|-|-|
| SCN-ISL-01 | Seller or supplier is the customer, his agent or a related party (hard rule) | I-04 | CBI 2018 item 6-2-3, contract void (S) | `islamic_deal.contract_type`, `deal_party.role`, `deal_party.party_id`, `party_relationship.type`, `party_contact.value_hk`, `account_party_role.party_id`, `account.number_hk`, `account.iban_hk` | `device_logs` is not held: the device match is not covered; the rest readiness A where mapped | restricted | L1 |
| SCN-ISL-02 | Foreign-currency murabaha: contract currency differs from settlement currency (hard rule) | I-06 | CBI 2018 p. 22 item 7, prohibited (S) | `islamic_deal.currency`, `islamic_deal.settlement_currency` (required; NULL is not evaluable and grades D) | none beyond the mapping | restricted | L1 |
| SCN-ISL-03 | 'Inah pattern: sale to the customer and buy-back of the same asset within the window at a lower price (hard rule) | I-06 | CBI 2018, prohibited (S) | `islamic_deal.asset_ref`, `islamic_deal.customer_id`, `islamic_deal.opened_on`, `islamic_deal.closed_on`, `islamic_deal.contract_amount`, `islamic_deal.deferred_price` | the buy-back leg is a second deal row; readiness B until the closure event maps | restricted | L2 |
| SCN-ISL-04 | Organised tawarruq: the bank is the customer's seller under an agency clause, or proceeds are credited the day of purchase (hard rule) | I-06 | CBI 2018 and IIFA resolution 179, prohibited (S) | `islamic_deal.contract_type`, `txn.booking_date`, `txn.account_id`, `txn.amount` | external `contract_clauses`: not held by any source in the studied installation; readiness D, shown as "not covered" | restricted | L1 |
| SCN-ISL-05 | Real estate sold or closed before two years after break-even without default (hard rule) | I-07 | CBI 2018 item 16 (S) | `islamic_deal.asset_class`, `islamic_deal.break_even_on`, `islamic_deal.closed_on`, `islamic_deal.status` | break-even date must be mapped; readiness C until then | restricted | L2 |
| SCN-ISL-06 | Qard hasan above IQD 10,000,000 per loan, or to a staff member or a related person (hard rules) | I-08 | CBI 2018 p. 59 (S) | `islamic_deal.contract_type`, `islamic_deal.contract_amount`, `islamic_deal.currency`; `facility.limit_amount`, `facility.currency`, `facility.product_id`, `product.islamic_contract_type` for sources without agreements (the mapping decides which; never both for one agreement); `party.staff_flag`, `party.related_to_staff`, `party_relationship.type`, `customer.party_id` | readiness A (agreement source) or B (schedule source) | restricted | L1 |
| SCN-ISL-07 | Repeat qard hasan within six months of full repayment, or while another is unpaid (hard rule) | I-08 | CBI 2018 pp. 58-59 (S) | `islamic_deal.contract_type`, `islamic_deal.opened_on`, `islamic_deal.closed_on`, `islamic_deal.status`; `facility.events` for schedule sources | readiness B; repayment dates come from the schedule tables, which the source maps later | restricted | L2 |
| SCN-ISL-08 | Transfer between the Islamic and conventional books within two business days (inter-book) | I-17 | analysis; IFSB glossary "separate funds" (N) | `account.islamic_book` on both accounts, `txn.account_id`, `txn.amount`, `txn.booking_date`, `customer.party_id` | readiness A once the book flag is mapped (phase 10) | normal | L1 |

Parameters (DSL A.5; each has a basis tag, with `DP` requiring `review_by`):

- SCN-ISL-03 `window_days`: default 90, min 30, max 365, basis `DP`, `review_by` 2027-04-30. A shorter window weakens the rule, so the
  minimum is the floor.
- SCN-ISL-05 `break_even_months`: 24, min 24, max 24, basis `CBI` (the two-year rule is fixed).
- SCN-ISL-06 `qard_cap_iqd`: 10000000, min 10000000, max 10000000, basis `CBI`. Any change is a logic change (DSL A.5), never a parameter set.
- SCN-ISL-07 `repeat_months`: 6, min 6, max 6, basis `CBI`.
- SCN-ISL-08 `window_business_days`: default 2, min 1, max 5, basis `BANK`. A longer window weakens the rule, so the maximum is the floor.
- SCN-ISL-01, 02 and 04 have no tunable parameter (hard rules, CBI text, DSL A.5 "no tuning").

The hard rules need no baseline. The Islamic-specific condition of every scenario is its applicability (`contract_type` or
`islamic_book`), and every scenario has at least one further condition (section 3.10).

Visibility `restricted` makes an alert of the scenario a suspicion-tier row (phase 17 rule 2). SCN-ISL-08 is `normal` because its
signal is the book, not a counterparty's conduct; it is still shown only to the phase 17 queue roles.

### 3.7 Pack install, flag and go-live (phase 06, 16 and 03 extended)

1. Register rows. Phase 06 `packs seed` (every release) inserts the eight `scn.scenarios` rows with `pack = islamic`,
   `requires_flag = 'pack.islamic'` and their visibility; `owner_id` stays NULL until the install of step 2 names the owner. While the flag is off each is graded D with reason
   `{"pack_not_enabled": "pack.islamic"}` (phase 06 section 3.2 rule 1) and is left out of the counts.
2. Versions. Phase 16 pack seed (section 3.15) is extended: a document with `requires_flag` is skipped while that flag is off, so no
   `scn.scenario_versions` row exists and the batch engine has nothing to run. With the flag on, the command
   `akr-ctl packs seed --pack islamic --owner <active M user>` installs each document in `shadow` (`shadow_days_required = 30`, its
   default parameter sets `proposed`). `--owner` is required: an Islamic scenario without a named owner is refused before any write, with exit 1, as a failed seed is (phase 16 section 3.15 step 1). No job is enqueued.
3. Go-live. Gate G9 (phase 16 section 3.4) reads `pack: akr` or `pack: islamic`: the version goes live only with `pack.akr_live` on, by
   a `scenario` change request (phase 16 section 3.3). With `pack.islamic` off a go-live is refused as `pack_not_enabled`.
4. Switching off. The phase 03 flag service refuses to set `pack.islamic` to FALSE while any `live` islamic version exists (409
   `E_NOT_ALLOWED_IN_STATE`, sub-code `islamic_versions_live`). The bank first retires each version (phase 16 `retire`, which sets
   `effective_to`, DSL A.10). The engine never reads the flag itself: it runs what is live, so this order keeps the flag true to the
   behaviour.
5. Route guard. Every `/islamic/*` route answers 404 with the body of an unknown id while the flag is off (section 3.8 of this phase and
   section 3.4 step 1).

### 3.8 Pooled-fund freeze (I-10; `FreezeService::start` step 6 of phase 20)

Phase 20 creates one freeze task with action `freeze` for every target account. This phase makes the task follow the account class:

1. `AccountClass` of each target account: the account's current product (`canon.account.product_id` to `canon.product`, current row)
   and its `islamic_contract_type`, mapped through the admin setting `islamic.account_class_map` (section 4.3; default
   `{"mudaraba": "pooled", "qard_hasan": "restricted"}`). Every other type, and an account with no type (every conventional account),
   is `unrestricted`. With the flag off the map is not read and every account is `unrestricted`, so phase 20 behaves as shipped.
2. Target accounts are grouped by class. One ops task per non-empty group, through `OpsTaskService::create` (phase 20 section 3.1), with
   `action`: `unrestricted` to `freeze`, `restricted` to `restrict`, `pooled` to `hold`. `neutral_text_code` follows the action (phase 20
   CHECK: `ops.restrict.account` for freeze and restrict, `ops.hold.account` for hold).
3. A `pooled` task is an ordinary `hold` task: its verification follows phase 20 section 3.3 unchanged. `second_person_only` is not
   set at creation (phase 20 sets it only with verification level `v0`, and its CHECK refuses it earlier; lead review). The class shows in
   the action (`hold`); no account number is added. A two-person rule for pooled holds is Open doubts 9.
4. `freezes.ops_task_id` holds the `unrestricted` task when there is one, otherwise the first group's task. The other group tasks are
   found by `casework.ops_tasks.source_id = freeze.id` (no schema change: `source_id` has no uniqueness).
5. A pooled hold stops withdrawals from the customer's own investment account only. The pool, its profit allocation and the other
   holders' shares are not in the instruction, and the platform does not compute them. The instruction text says so (neutral text).
6. No Finance task and no Sharia board task is created: neither role exists (MODULES s.0), and the Sharia board is not a user. The
   pooled case is the neutral task above with two-person verification; the routing to Finance and to the board is Open doubts 9.
7. Audit `freeze.created` (phase 20) gains `groups: {unrestricted: n, restricted: n, pooled: n}` (counts only).
8. Release (the terminal effect `release_freeze`, phase 20 section 3.5 step 5, on approval) uses the same groups, one release task per group; a proposal creates no ops task.

### 3.9 Sharia board aggregates (I-16; `ShariaAggregateQuery`)

`GET /islamic/sharia-board/aggregates?period=YYYY-MM` (scope `dashboard.read`; Cedar `sharia.aggregate.read`, see 3.13):

1. Flag on, else 404. `period` defaults to the current bank month; a future month is 422 `E_VALIDATION` at `/period` (sub-code `period_future`), and a value that is not `YYYY-MM` with a month from 01 to 12 is 422 `E_VALIDATION` at `/period` (sub-code `period_invalid`).
2. One read-only statement over `canon.islamic_deals` (current versions opened in the period or open at its end), `kyc.deal_evidence_status`
   and `canon.account` (`islamic_book`). The statement groups by `contract_type`, `status`, `evidence_status` and `asset_class`, and
   selects no id, no party, no customer, no `asset_ref`, no amount and no date finer than the month.
3. Cells: deals per contract type with the status split (active, settled, closed, other); deals per evidence status; accounts on the Islamic
   book; and three gap rates (no asset, no supplier or seller party, evidence unknown), each as numerator and denominator.
4. Small cells (phase 21 `report.small_number_threshold`, governed, default 5, minimum 3): a count above zero and below the threshold
   is returned as null with one `suppressed_cells` for each; a rate whose numerator or denominator is suppressed is null. Zero stays zero.
5. A gap rate above `islamic.dq_failure_rate` is returned with `control_failure: true` (the I-26 presentation). The value is shown
   beside the flag; nothing else changes.
6. Response `ShariaAggregateView { period, data_as_of, contract_types[], evidence[], gaps{...}, suppressed_cells, control_failure[] }`.
   It contains no alert, case, report, scenario hit, party or customer data, and no identifiers. It has no drill-down route.
7. Access event (phase 03) for the read, with no tier (the view has no tiered row); a dashboard read needs no purpose code (as G1).

Who the board sees: the Sharia board is not a user (I-16 and DR-250). The view is read by M, CH and IA, and the compliance head presents
its figures to the board. An export or a board pack for the board is not built in this phase (Open doubts 10).

### 3.10 Fairness: no Islamic penalty (S-37, I-30; phase 24 `FairnessService`)

1. No rating factor reads an Islamic attribute. The rating model of phase 15 has no input from `canon.product.islamic_contract_type`,
   `canon.account.islamic_book`, `canon.islamic_deals` or `kyc.possession_evidence`. A test greps the rating factor catalogue and the
   feature definitions (phase 13) for these names and fails on a hit (a `cargo test`, run with section 6 check 4).
2. Every ISL document has a further condition. `packs verify-data` (phase 16, extended) refuses an `ISL` document whose logic reads
   only `islamic_deal.contract_type` or `account.islamic_book` as its triggering conditions, with the check name `islamic_sole_trigger` (a `verify-data` failure, exit 2; not an API code). The
   applicability condition alone can select a deal, never flag it.
3. Fairness suite (phase 24 `FairnessService`, a pinned fixture): a conventional murabaha-like product and an Islamic product with identical
   facts (amount, parties, dates, currency, customer profile) produce the same rating level and the same alert set. The Islamic product
   is a `canon.product` row of the same family with a type. The test is part of the release gate of phase 24.
4. No `protected_attributes` row is added: its `attribute_class` is a closed list of personal classes (phase 24), and the contract type is
   a product attribute (Open doubts 8). No fairness metric has `group_kind` = product type for the same reason.

### 3.11 Errors (`IslamicError`, `thiserror`; API section 6 codes only)

| Variant | Code | HTTP | Sub-codes |
|-|-|-|-|
| `NotFound` | `E_NOT_FOUND` | 404 | flag off, unknown id, masked deal (identical body) |
| `Scope` | `E_SCOPE` | 403 | the gate of the route |
| `Forbidden` | `E_FORBIDDEN` | 403 | `role_not_permitted` (A1 verifying) |
| `SelfApproval` | `E_SELF_APPROVAL` | 403 | the attacher verifies |
| `StepUpRequired` | `E_STEPUP_REQUIRED` | 401 | `record_open`, `decision` |
| `PurposeRequired` | `E_PURPOSE_REQUIRED` | 422 | the deal read without `X-Access-Purpose` |
| `Validation` | `E_VALIDATION` | 422 | `kind_invalid`, `document_ref_too_long`, `reject_code_required`, `reject_code_not_allowed`, `period_future`, `period_invalid` |
| `UploadRejected` | `E_UPLOAD_REJECTED` | 422 | phase 02 antivirus or type refusal |
| `PayloadTooLarge` | `E_PAYLOAD_TOO_LARGE` | 413 | `evidence.max_bytes` |
| `NotAllowedInState` | `E_NOT_ALLOWED_IN_STATE` | 409 | `already_decided`, `evidence_not_clean`, `islamic_versions_live` (flag switch) |
| `VersionConflict` | `E_VERSION_CONFLICT` | 409 | `If-Match` differs; the current version in the body |
| `PreconditionRequired` | `E_PRECONDITION_REQUIRED` | 428 | verify without `If-Match` |
| `IdempotencyRequired` | `E_IDEMPOTENCY_REQUIRED` | 400 | attach or verify without the key |
| `IdempotencyReuse` | `E_IDEMPOTENCY_REUSE` | 422 | same key, different body |
| `Unavailable` | `E_UNAVAILABLE` | 503 | the key store or the evidence store is down |

A refused Islamic read answers `E_NOT_FOUND` with the unknown-id body in every case where the deal's tier or the caller's team hides it.

### 3.12 Audit events emitted (one per mutation; payloads carry ids, codes and counts only)

| Event kind | On | Resource | SIEM |
|-|-|-|-|
| `islamic.evidence.attached` | attach (3.5) | `islamic_deal` | no |
| `islamic.evidence.verified` | verify, decision `verified` (3.5) | `possession_evidence` | yes |
| `islamic.evidence.rejected` | verify, decision `rejected` (3.5) | `possession_evidence` | yes |
| `islamic.evidence.swept` | sweep run that wrote its register row (3.5 step 5) | `system` | no |

The kinds are entries of the phase 02 `EventKind` catalogue (Rust constants with the `siem` flag). The `details` of
`islamic.evidence.attached` carry `evidence_id` and `kind`; never the document reference, the file name or the duplicate result.
Mutations that this phase performs through other phases keep those phases' events: `scenario_version.seeded` (phase 16), the register
write of phase 24, `ops.task_created` and `freeze.created` (phase 20, with the group counts of 3.8), and the flag change of phase 03.
Reads write access events (phase 03, with purpose and tier), never audit rows.

### 3.13 Cedar: actions, roles and the file `88_islamic.cedar` (phase 03 range 80)

**Schema additions** (appended to `schema.cedarschema` as phases 17, 18, 24 and 25 did; no existing entity or attribute changes). The
deal actions use the phase 18 `Party` entity, so a deal is read under the same tier and team attributes as its customer:

```cedar
namespace Akr {
  action "islamic.deal.read", "islamic.evidence.read", "islamic.evidence.attach", "islamic.evidence.verify"
    appliesTo { principal: User, resource: Party, context: Ctx };
  action "sharia.aggregate.read"
    appliesTo { principal: User, resource: System, context: Ctx };
}
```

**File `88_islamic.cedar`** (phase 03 range 80, free: 80, 81, 82, 84 and 86 are taken). Every policy is written out; one role per `@role`;
`principal.roles` is the phase 03 form; no `containsAny`. `context.purpose` and `context.network` are required `Ctx` fields and need no `has`.
The forbids repeat the three phase 18 party forbids for the new actions, because phase 18's forbids name their own action list and
cannot be edited here (DR-255).

```cedar
// 88_islamic.cedar (phase 26). Roles are principal.roles (phase 03 form). Step-up is a handler check, not a condition.
// Deal reads and evidence actions use the customer's Party resource (phase 18), so they inherit its tier and case team.

// Analysts attach and read; the MLRO and the A2 senior analyst verify (a verification by the attacher is refused by the handler).
@id("islamic.work.a1") @role("A1")
permit (principal, action in [Akr::Action::"islamic.deal.read", Akr::Action::"islamic.evidence.read",
        Akr::Action::"islamic.evidence.attach"], resource is Akr::Party)
when { principal.roles.contains("A1") };

@id("islamic.work.a2") @role("A2")
permit (principal, action in [Akr::Action::"islamic.deal.read", Akr::Action::"islamic.evidence.read",
        Akr::Action::"islamic.evidence.attach", Akr::Action::"islamic.evidence.verify"], resource is Akr::Party)
when { principal.roles.contains("A2") };

@id("islamic.work.m") @role("M")
permit (principal, action in [Akr::Action::"islamic.deal.read", Akr::Action::"islamic.evidence.read",
        Akr::Action::"islamic.evidence.attach", Akr::Action::"islamic.evidence.verify"], resource is Akr::Party)
when { principal.roles.contains("M") };

// Sharia board aggregates: counts only. The route scope is dashboard.read (M, CH, IA), which these three permits match exactly.
@id("islamic.aggregate.m") @role("M")
permit (principal, action == Akr::Action::"sharia.aggregate.read", resource is Akr::System)
when { principal.roles.contains("M") };

@id("islamic.aggregate.ch") @role("CH")
permit (principal, action == Akr::Action::"sharia.aggregate.read", resource is Akr::System)
when { principal.roles.contains("CH") };

@id("islamic.aggregate.ia") @role("IA")
permit (principal, action == Akr::Action::"sharia.aggregate.read", resource is Akr::System)
when { principal.roles.contains("IA") };

// Suspicion-tier party (and so its deals): the MLRO or a member of the open case team, from the analyst network, with a purpose.
@id("islamic.forbid.suspicion")
forbid (principal, action in [Akr::Action::"islamic.deal.read", Akr::Action::"islamic.evidence.read",
        Akr::Action::"islamic.evidence.attach", Akr::Action::"islamic.evidence.verify"], resource is Akr::Party)
when { resource.tier == "suspicion" && !(principal.roles.contains("M") || resource.case_team.contains(principal)) };

@id("islamic.forbid.network")
forbid (principal, action in [Akr::Action::"islamic.deal.read", Akr::Action::"islamic.evidence.read",
        Akr::Action::"islamic.evidence.attach", Akr::Action::"islamic.evidence.verify"], resource is Akr::Party)
when { resource.tier == "suspicion" && context.network != "analyst" };

@id("islamic.forbid.purpose")
forbid (principal, action in [Akr::Action::"islamic.deal.read", Akr::Action::"islamic.evidence.read",
        Akr::Action::"islamic.evidence.attach", Akr::Action::"islamic.evidence.verify"], resource is Akr::Party)
when { resource.tier != "normal" && !(context.purpose == "assigned_work" || context.purpose == "investigation") };
```

Checks of the file (section 6, check 10): every route scope admits every role the file permits for that route (queue.work admits A1,
A2 and M; dashboard.read admits M, CH and IA; the verify route is narrowed by Cedar to A2 and M), no role outside phase 17 rule 1 reaches
a deal (A1, A2 and M only), and the forbids match the phase 18 forbids for the party actions word for word, apart from the action list.
The test that the SQL list predicate of section 3.4 and this file agree (phase 17 section 3.10 method) runs over generated principals.

### 3.14 Tipping-off review (what a person outside the case's access list can learn; the answer must be "nothing")

| Surface | Who can see it | What the viewer learns | Answer |
|-|-|-|-|
| Deal panel (`GET /islamic/deals/{id}`, W3 and W4 tab) | A1, A2, M on a readable customer | the deal, its contract type, its parties the viewer may read | nothing about a suspicion-tier party: the deal is refused (404) and parties the viewer may not read are omitted with no count |
| Deal list (`GET /islamic/deals`) | same | the deals of readable customers only | nothing: an unreadable customer gives the empty page of a customer with no deals |
| Attach response | the attacher | the attachment and its status | nothing about a duplicate document: the body is identical with or without a duplicate (3.5 step 7) |
| Duplicate document count (G1 tile) | M only | a count of duplicate document references | a count, never a deal id; below `report.small_number_threshold` it is suppressed |
| Verify response | A2, M | the decision | nothing about the attacher's identity beyond the user id the caller already sees on the row |
| Register warning (sweep, 3.5) | M, CH, IA (phase 24 `dashboard.read`) | two counts for a bank day | counts only, no deal id: IA learns how many deals lack evidence, as it may for suspicion matters (DR-219 of phase 24) |
| Audit event `islamic.evidence.attached` | IA, SEC | the evidence id and kind | nothing about the document, the duplicate or the deal's tier |
| Scenario alerts SCN-ISL-01 to 08 | the phase 17 queue roles (A1, A2, M) under the alert's tier | the alert, through phase 17 | a restricted scenario's alert is a suspicion row, visible only under phase 17 rule 5; a branch user has no alert access at all |
| Sharia board aggregates (3.9) | M, CH, IA | counts by contract type, evidence status and gap rate | nothing about a party, a customer, a case, an alert, a report or a scenario hit; small cells suppressed |
| Pooled freeze task (3.8) | OPS (neutral text) | the neutral instruction and the account class code | nothing about the freeze reason, the screening list or the party; the case id is not in the OPS view (phase 20) |
| Flag-off answer (every `/islamic/*` route) | every caller | an unknown-id 404 | nothing about whether the pack exists |

### 3.15 Settings used (admin-managed; section 4.3 holds the inserts)

`pack.islamic` (phase 06), `pack.akr_live` (phase 16), `report.small_number_threshold` (phase 21), `evidence.max_bytes` and
`evidence.lock_days` (phase 02), `ops.*` freeze settings (phase 20), and the five settings of section 4.3. Every threshold a bank may
change is a setting or a scenario parameter, never a constant of the code (the constants are the statutory numbers of section 3.6).

### 3.16 Lock order

1. The deal row (`canon.islamic_deals`, `FOR SHARE`) before any evidence row, in attach, verify and the sweep.
2. The evidence row (`kyc.possession_evidence`, `FOR UPDATE`) after the deal row; a verify never locks two evidence rows.
3. The freeze row before its ops tasks (phase 20 section 3.13); the pooled grouping (3.8) takes no lock of its own.
4. The register insert takes no lock beyond its unique index (`ON CONFLICT`).

## 4. Infrastructure Updates

### 4.1 Job kinds (Postgres `jobs`; coalesce key in brackets)

| Kind | Binary | Payload [coalesce] | Concurrency, attempts | Trigger |
|-|-|-|-|-|
| `islamic_evidence_sweep` | worker | `{}` [`islamic_evidence_sweep`] | 1, 3 | `schedule.islamic_evidence_sweep` (04:30 bank time); no-op unless `pack.islamic` and `islamic.evidence_tracking` are on (3.5) |

No other job, timer or outbox topic. The library scenarios run in phase 16's `scenario_sweep` (L2) and batch path; the pooled freeze
runs inside phase 20's freeze start; the Sharia view is a read.

### 4.2 Timers and outbox

No timer kind. No outbox topic: no event of this phase is consumed by another phase, and the sweep and the routes write the only rows.

### 4.3 Settings (copy of the phase 25 INSERT format; `floor` only where `governed` is TRUE)

```sql
-- Settings of phase 26. Copy of the phase 25 insert format (key, value, value_type, floor, governed, label).
INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('islamic.evidence_tracking', 'false', 'boolean', NULL, TRUE,
  '{"ar": "تتبع أدلة الحيازة للعقود الإسلامية مفعّل", "en": "Possession-evidence tracking is live (missing evidence is reported)"}'),
 ('islamic.evidence_due_days', '30', 'integer', '{"max": 60, "basis": "I-05 ageing; analysis: a longer period weakens the control"}'::jsonb, TRUE,
  '{"ar": "المدة المسموحة لإرفاق دليل الحيازة (أيام)", "en": "Days allowed to attach possession evidence"}'),
 ('islamic.dq_failure_rate', '0.05', 'decimal', '{"max": 0.05, "basis": "I-26; a high gap rate is a control failure; a rate that triggers review has a maximum"}'::jsonb, TRUE,
  '{"ar": "نسبة النقص في بيانات العقود التي تُعد إخفاقاً في الضبط", "en": "Gap rate shown as a control failure on the Sharia view"}'),
 ('islamic.account_class_map', '{"mudaraba": "pooled", "qard_hasan": "restricted"}', 'json', NULL, TRUE,
  '{"ar": "تصنيف حسابات العقود الإسلامية للتجميد", "en": "Account class of Islamic products for freezes"}'),
 ('schedule.islamic_evidence_sweep', '"0 30 4 * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدولة مسح أدلة الحيازة", "en": "Schedule of the possession-evidence sweep"}')
ON CONFLICT (key) DO NOTHING;
```

`islamic.evidence_tracking` and `islamic.evidence_due_days` are governed: a change is a change request (phase 03 governed settings). The
floors bound the side that weakens the control: the due period has a maximum (a longer period weakens the control), and the gap-rate
threshold has a maximum (a higher threshold calls fewer gaps a failure). `islamic.account_class_map` has no floor: it is an enumeration, and its values
are validated by the handler against the three classes (`E_VALIDATION` at `/account_class_map`).

### 4.4 Network zones, metrics, readiness, OpenAPI

- No network zone change. Evidence bytes go to `akr-quarantine` through phase 02 `stage`, unchanged.
- Metrics: counter `akr_islamic_evidence_total{outcome}` (attached, verified, rejected); gauge `akr_islamic_evidence_unverified` (count of
  attached rows older than `islamic.evidence_due_days`, from the sweep). No label carries a deal or party id.
- Readiness: unchanged (the flag does not affect the readiness endpoint).
- OpenAPI: tags `islamic` (five paths) and `sharia-board` (one path); the content and verify paths are added. `make generate-api` regenerates the client.

### 4.5 Existing entity, schema, route and code updates (extensions of earlier phases)

1. **Phase 02**: the evidence owner registry gains `islamic_deal` (retention class R3, the lock of `evidence.lock_days`); the audit
   `EventKind` catalogue gains the four kinds of 3.12.
2. **Phase 03**: the flag service refuses `pack.islamic` FALSE while a live islamic version exists (3.7 step 4); the Cedar bundle gains
   `88_islamic.cedar` and the two action declarations of 3.13; `ChangeTargetRegistry` is unchanged.
3. **Phase 06**: the eight `scn.scenarios` register rows of 3.7 step 1 are seeded by `packs seed`; no change to `scn.scenario_versions`.
4. **Phase 10**: `canon.product` CHECK widened (section 1.1); `CanonicalEntity` gains two names; `canonical-model.v1.json` gains two
   entries; mapping families `islamic_deal` and `deal_party` registered in the mapping DSL catalogue.
5. **Phase 12**: no change to the `dq_issues` kind list; three rule codes of 3.2 are recorded in `detail.code`.
6. **Phase 15**: no factor added (DR-249). The feature catalogue of phase 13 is unchanged; the test of 3.10 pins that.
7. **Phase 16**: the pack seed skips flagged documents while their flag is off (3.7 step 2); the G9 gate reads `pack: akr` or `pack: islamic`
   (3.7 step 3); `packs verify-data` refuses the check `islamic_sole_trigger` (3.10 step 2); `akr-ctl packs seed` takes `--pack` and `--owner`.
8. **Phase 17**: `services::visibility::build` is unchanged; the deal read calls its party rule through `PartyReadPredicate`.
9. **Phase 20**: `FreezeService::start` step 6 groups target accounts by class (3.8); `freeze.created` gains the group counts.
10. **Phase 24**: the G1 tiles of section 5.3 (counts only); the register warning of 3.5 uses the existing register table.
11. **Settings**: the five rows of 4.3.

## 5. Frontend Impact

### 5.1 W3 case workspace (`/cases/:id`, WORK; phase 18 screen) and W4 party page (`/parties/:id`, WORK tabs): the deal panel

- W3: an "Islamic deal" panel in the evidence column of the case, shown when the flag is on and the case's party has at least one deal the
  caller may read. It lists the deals of the subject party (contract type, status, evidence status) and opens the deal view.
- W4: a tab "Islamic deals" beside the existing tabs, with the same list and the deal view. Both use `useGetIslamicDeals` and
  `useGetIslamicDealsId`. A refused deal does not appear; the panel never says that a deal was hidden.
- The deal view shows parties by role with the display name the API returns (only readable parties), the asset reference, the contract
  type in words (bilingual label from the API, IR-54), the evidence status in words ("verified", "attached", "not yet known"), and the
  evidence rows. "Not yet known" is the word for `unknown`; the UI never says "missing" for an evidence status (only the register does).
- Attach (`usePostIslamicDealsIdEvidence`): a file, a kind from a short list, an optional document reference. The button is shown to A1,
  A2 and M. The response is the same whatever the duplicate result (3.5).
- Verify (`usePostIslamicDealsIdEvidenceEvidenceIdVerify`): shown to A2 and M only, and not on a row the viewer attached (the button is
  hidden; the API still refuses). It needs `If-Match` (the row version) and a step-up dialog (`decision`). The reject choice needs a reason code.

### 5.2 I1 Sharia board aggregates (`/islamic/sharia-board`; DASH; new screen, catalogue id `I1` (DESIGN-SYSTEM 8.1, gap pass 1), new in phase 26)

- A read-only page for M, CH and IA (`dashboard.read`): a month selector, the counts by contract type, the evidence split, the three
  gap rates with the control-failure flag, and the suppressed-cell note ("fewer than 5", in the user's locale).
- The page carries no link to a deal, a party, a case or an alert. It is not in the G1 navigation for the board.
- Hook: `useGetIslamicShariaBoardAggregates` with the `period` query.
- Neutral wording (DESIGN-SYSTEM section 11): no word "suspicion" or "suspicious" on this page; the gap rates are described as data gaps.

### 5.3 G1 command centre (`/command`; DASH; phase 24): two tiles, no route

- "Islamic deals awaiting possession evidence" (count of deals the caller may read whose evidence status is `attached` but not verified, or
  `unknown` with the evidence process live): visible to M only (G1 is `dashboard.read`; A2 holds no such scope, lead review);
  hidden while the flag is off.
- "Islamic evidence documents reused across deals" (count of duplicate document references): visible to M only; below the small-number
  threshold it reads "fewer than 5".
- Hook: the existing `useGetDashboardCommandCentre` (no new hook). Each tile links to W2 or to the W4 tab of the deal's party, through the
  same visibility rules.

### 5.4 Generated hook summary

`useGetIslamicDeals`, `useGetIslamicDealsId`, `usePostIslamicDealsIdEvidence`, `usePostIslamicDealsIdEvidenceEvidenceIdVerify`,
`useGetIslamicShariaBoardAggregates`; the content read is hand-written like phase 18's `useEvidenceContent` (a blob, no cached URL). The screens of 5.1 and 5.2 and the tiles of 5.3 are the only consumers.

### 5.5 Browser and caching rules

The deal view and the aggregates are `no-store` responses (phase 17 rule for suspicion-capable reads). The aggregates page refreshes
on navigation only; it has no polling.

## 6. Verification

1. `make migrate`: `0083_canon_islamic_deals` and `0084_kyc_possession_evidence` apply cleanly, twice (idempotent); the `canon.product`
   CHECK accepts `tawarruq` and rejects `riba`.
2. `cargo check`; then the classification check: every column of `canon.islamic_deals`, `canon.deal_parties` and `kyc.possession_evidence`
   has a `admin.column_classification` row (a query over `information_schema.columns` against `admin.column_classification` returns no
   missing column).
3. `cargo clippy -- -D warnings`.
4. `cargo test`: `islamic_product_map_is_complete` (3.1 step 3); `derive_evidence_status` equals the view over generated rows; `suppress_small`
   for counts 0, 1, 4, 5; `account_class` for the three classes and for NULL.
5. `akr-ctl canon verify-catalogue`: the two new entities match the migrated columns.
6. Curl suite `tests/curl/phase26_islamic.sh` (dev users `a1@`, `a2@`, `m@`, `ch@`, `ia@`, `sec@`, `cfg@`, `ds@`, `ss@`, `kyc@`, `adm@`,
   `mro@`, `a2b@`, one per role on the bank test domain; fixtures: deals `D1` (customer C1, normal tier), `D2` (customer C2 with a suspicion
   party), `D3` (qard hasan with a staff recipient), and a CBS-loaded deal `D4`). The suite:
   - a. with the flag off (`pack.islamic` FALSE): `GET /islamic/deals/<D1>` as A1, `POST /islamic/deals/<D1>/evidence`, `GET /islamic/sharia-board/aggregates`
     answer 404 with a body byte-identical to `GET /islamic/deals/<unknown uuid>` (`cmp`).
   - b. with the flag on: `GET /islamic/deals/<D1>` as A1 with `X-Access-Purpose: assigned_work` returns 200 with `evidence_status` `unknown`.
   - c. without `X-Access-Purpose`: 422 `E_PURPOSE_REQUIRED`.
   - d. Cedar deny (403): `POST /islamic/deals/<D1>/evidence/<E1>/verify` as A1 with `If-Match` and step-up `decision` returns 403 `E_FORBIDDEN`.
   - e. masked deal (404): `GET /islamic/deals/<D2>` as A1 not on the case team returns 404 whose body equals the body for an unknown id
     (`cmp`); the access event `deny` exists (`psql` as the audit reader role).
   - f. the suspicion read as the team member without `record_open` step-up returns 401 `E_STEPUP_REQUIRED`; with it, 200.
   - g. attach idempotency: `POST /islamic/deals/<D1>/evidence` twice with `Idempotency-Key: ev-0001` and the same multipart body returns the
     same evidence id; the second response carries `x-idempotent-replay: true`; one row exists (`psql`).
   - h. the same key with a different `kind`: 422 `E_IDEMPOTENCY_REUSE`; no key: 400 `E_IDEMPOTENCY_REQUIRED`.
   - i. self-verification refused: A2 `a2@` attaches `E2` and then verifies it: 403 `E_SELF_APPROVAL`.
   - j. verify without `If-Match`: 428; with a stale version: 409 `E_VERSION_CONFLICT`; as A1 with a valid version: 403 `E_FORBIDDEN`.
   - k. verify by `m@` (another person) with step-up `decision` and the current version: 200; `GET /islamic/deals/<D1>` shows `evidence_status` `verified`;
     a second verify of the same row: 409 `E_NOT_ALLOWED_IN_STATE` (`already_decided`).
   - k2. content: `GET /islamic/deals/<D1>/evidence/<E1>/content` as `m@` with `X-Access-Purpose` returns the bytes with
     `Cache-Control: no-store` and one `view` access event; the same path with an evidence id of `D3` answers 404 with the unknown-id body.
   - l. CBS deal `D4` (loaded by the writer): its `evidence_status` is `unknown` after a load; `UPDATE kyc.possession_evidence` as `akr_ingest`
     fails with `permission denied`; no mapping target `evidence_status` exists (`mapping lint` refuses a mapping that writes it).
   - m. duplicate document: attach the same `document_ref` to `D1` and to `D3` in turn: both responses are byte-identical in shape (`jq 'keys'`
     equal), no `duplicate` key; the G1 tile as `m@` reads `suppressed: true` (one shared reference is below the threshold; the tile read zero before the second attach), and as `ch@` and `ia@` the tile is absent (`a2@` holds no `dashboard.read` and gets 403 `E_SCOPE` on the G1 route).
   - n. aggregates: `GET /islamic/sharia-board/aggregates?period=<month>` as `m@`, `ch@`, `ia@` returns 200; as `a1@` and `sec@` 403 `E_SCOPE`;
     no UUID appears in the body (`grep -E` over the body is empty); a cell of 2 deals reads `null` and `suppressed_cells` is 1.
   - o. future period: `period=2099-01`: 422 `E_VALIDATION` at `/period`.
   - p. sweep: with `islamic.evidence_tracking` FALSE the job writes no register row; set TRUE by change request; the job writes one
     `islamic_evidence_gap:<date>` warning with counts; a second run the same day writes none (`ON CONFLICT`).
   - q. pooled freeze: a freeze with one `mudaraba` account and one murabaha account creates two ops tasks, `hold` (`second_person_only` FALSE) and
     `freeze`; a `qard_hasan` account gives `restrict`; `ops.hold.account` is the neutral code of the hold.
   - r. scenario seed: flag off: `scn.scenarios` has eight `pack = islamic` rows and zero `scn.scenario_versions` rows for them; flag on and
     `packs seed --pack islamic --owner m@`: eight shadow versions; go-live without `pack.akr_live`: refused `pack_not_agreed`.
   - s. flag switch: setting `pack.islamic` FALSE while a live version exists: 409 `E_NOT_ALLOWED_IN_STATE` (`islamic_versions_live`).
   - t. audit: the trail holds `islamic.evidence.attached`, `islamic.evidence.verified` and `islamic.evidence.rejected` for the session, and the
     SIEM pull (phase 25) returns the verified and rejected kinds and not the attached kind.
7. `packs verify-data` (phase 16, extended) passes on the eight documents and fails on a fixture document whose only condition is
   `islamic_deal.contract_type` (check `islamic_sole_trigger`).
8. Fairness suite (phase 24) pins the pair of section 3.10 step 3: identical facts, identical rating level and alert set.
9. Swagger shows the six paths; `make generate-api` regenerates the client, and the five hooks of 5.4 exist in the generated module.
10. The Cedar bundle validates (`akr-ctl policy validate` prints the new bundle hash with `88_islamic.cedar`); the generated-principal test pins
    the list predicate of 3.4 to the Cedar file. A policy test holds the three checks of the file that section 3.13 states: the route
    scopes against the roles the file permits, the role reach (A1, A2 and M only), and the forbid text against the phase 18 forbids.
11. Read-only conformance (phase 25 check set): `akr_readonly` cannot read `kyc.possession_evidence` (permission denied); its grants are unchanged.
12. Existing tests still pass, including phase 14's screening fixture (a deal's supplier is screened by `daily_base` with no new run kind).

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 1: corrections found while checking phases 00 to 02 (tools/agent/gap1-run1-verdict.md); lead-verified.

### 7.1 Home of the keyed-hash purpose
The purpose `islamic.document_ref` is declared in the phase 00 crypto port (`Crypto::keyed_hash`, phase 00 7.15), not
in a phase 02 list. Section 3's "(phase 02 `KeyedHash`)" reads "(phase 00 `Crypto::keyed_hash`)". Open doubts 15 is
closed.

Gap pass 1 (2026-10-10), run 4b, from tools/agent/gap1-run4b.md; lead-verified (tools/agent/gap1-run4b-verdict.md).

### 7.2 Register warning call (DR-388)
Section 3.5 step 4 calls `RegisterWriter::warn(tx, 'scan', 'islamic_evidence_gap:' || bank_date,
WarningTitle::IslamicEvidenceGap, body)` inside the sweep's transaction, with `body = {"missing": n, "late": n}`.
This phase adds the variant `IslamicEvidenceGap` to phase 24's closed enum `WarningTitle`, with the bilingual title in
the same enum's `title {ar, en}` map ({"ar": "نواقص مستندات الحيازة في العقود الإسلامية", "en": "Possession evidence gaps on Islamic deals"}). The insert-or-nothing on `source_key` keeps one row per bank day. The ports list of section 2
reads the phase 24 signature.

### 7.3 Party predicate form
The party rule is phase 18's `PartyReadPredicate::allows(principal, party_id, context)` (phase 18 section 7+), not a
function built by phase 17. Section 3.4 step 7 calls it with the same three arguments as the ports list of section 2:
the caller, the party id and the deal read's context. The omitted-party rule of step 7 stays.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.4 List of one customer's deals (V-26-02-a, MEDIUM; DR-413)
`IslamicDealService::list_for_customer(ctx, customer_id, cursor) -> Page<IslamicDealSummary>` serves `GET /islamic/deals?customer_id=`, with `X-Access-Purpose` required as on the deal read (422 `E_PURPOSE_REQUIRED` without it):
1. Flag: as section 3.4 step 1.
2. `customer_id` missing or not a UUID, or a malformed cursor: 422 `E_VALIDATION` (phase 00 cursor rules).
3. The customer's party and its tier: section 3.4 steps 3 and 4. No current customer row gives the empty page.
4. Cedar `islamic.deal.read` on that party, as section 3.4 step 5. A refusal writes the masked access event and answers 200 with the empty page, the body of a customer with no deals.
5. No step-up on the list (lead decision). A row is a summary; each deal is opened by its id, and section 3.4 step 6 applies there. The three forbids of section 3.13 already hold the list of a suspicion-tier customer to M and the case team, on the analyst network, with a purpose.
6. Read `IslamicDealRepository::list_for_customer(customer_id, after, limit)` (current rows). `IslamicDealSummary` is `{ id, contract_type, status, opened_on, closed_on, evidence_status }`: no party, no amount and no asset.
7. Access events: `scope.record_hits(rows)` (phase 02 list rule), one `search_hit` per row when the customer's tier is restricted or suspicion, and nothing for the normal tier. No audit row.

### 7.5 Step-up actions of this phase (V-26-02-b, MEDIUM; DR-413)
This phase declares two `StepUpAction { name, class }` constants in `domain::islamic`, with phase 01's closed classes (both classes exist, so phase 01 does not change):
- `DEAL_OPEN` is `{ "islamic.deal.open", record_open }`. It is consumed in section 3.4 step 6 and in step 1 of the content read of section 3.5, both with the resource ref `islamic_deal:<id>`.
- `EVIDENCE_DECIDE` is `{ "islamic.evidence.decide", decision }`. It is consumed in step 2 of the verify of section 3.5, with the resource ref `possession_evidence:<id>`.

### 7.6 Conditions of SCN-ISL-01 (V-26-03-a, V-26-03-b, MEDIUM; DR-413)
The row of section 3.6 gives the fields; this is the logic the document carries. For one current deal, C is the party of role `customer` and S is each party of role `seller` or `supplier` (current `canon.deal_parties` rows). The rule fires when any of these holds for one S:
1. Same party: S is C.
2. Agent: S also holds the role `agent` on the same deal, or a current `canon.party_relationship` row of type `attorney` joins S and C in either direction. Phase 10 has no other agency type, and none is added.
3. Related party: a current `party_relationship` row of any other type joins S and C in either direction, or S and C are both joined to one third party by rows of type `owner`, `controller`, `director` or `signatory` (the shared owner and the shared signatory of I-04).
4. Shared contact: S and C have an equal `party_contact.value_hk` of the same `type` among `mobile`, `landline`, `email` and `address`. A NULL hash is never compared.
5. Shared account: S and C both hold a current `account_party_role` row on one account, or hold accounts with an equal `number_hk` or `iban_hk`.
The alert's evidence names which of the five matched, as a code, and never a contact value or an account number. The device match of I-04 is not covered, as the row says.

### 7.7 Command centre tile counts (V-26-04-a, V-26-04-f, MEDIUM; DR-413)
Phase 24's `CommandReadRepository` (`akr_app`) gains one method for each tile of section 5.3. Each takes the caller's visibility predicate as a bound parameter, as phase 24 section 2 requires, and applies it to the deal's customer party; for M, the only role that sees the two tiles, it admits every tier. Each reads current deal rows (`recorded_to IS NULL`) and returns one count.
1. `islamic_awaiting_evidence(conn, predicate) -> Result<i64, RepoError>`: the deals whose `kyc.deal_evidence_status` is `attached`, plus those where it is `unknown` while `islamic.evidence_tracking` is TRUE.
2. `islamic_duplicate_refs(conn, predicate) -> Result<i64, RepoError>`: the number of distinct `document_ref_hk` values that rows of two or more different deals hold, among rows with `document_ref_hk IS NOT NULL AND status <> 'rejected'` (the predicate of `idx_possession_doc`). One shared reference counts once.
Both are called only while `pack.islamic` is on. Both pass through the small-number rule of phase 24 section 3.7 like every tile: a count from 1 to the threshold minus 1 is `value: null, suppressed: true`.
The attach of section 3.5 makes no duplicate read and stores no duplicate mark, so its response and its timing are the same with or without a shared reference. `PossessionEvidenceRepository` therefore has no `duplicate_exists` method.

### 7.8 PossessionEvidenceService methods (V-26-04-b, LOW; DR-413)
`PossessionEvidenceService` has four methods, each run by the steps of its part of section 3.5:
- `attach(ctx, deal_id, AttachEvidence, file) -> PossessionEvidenceView` (Attach).
- `verify(ctx, deal_id, evidence_id, if_match, DecideEvidence, header) -> PossessionEvidenceView` (Verify; `header` is the step-up assertion header).
- `content(ctx, deal_id, evidence_id, download: bool, header) -> VerifiedReader` (Content; phase 02 `open`; `header` is the step-up assertion header that step 6 of section 3.4 consumes on a suspicion-tier deal). `IslamicDealService::get` of section 3.4 takes the same `header` parameter.
- `sweep(now) -> EvidenceSweepCounts` is the body of the job `islamic_evidence_sweep` (Sweep); `EvidenceSweepCounts { missing, late }` holds the two counts of its step 3.

### 7.9 Conditions of SCN-ISL-06 and SCN-ISL-07 (V-26-05-a, LOW; V-26-05-b, V-26-05-c, V-26-05-d, MEDIUM; DR-413)
SCN-ISL-06:
- A qard hasan is a deal with `contract_type = 'qard_hasan'`, or, for a source without agreements, a `canon.facility` row whose product row has `islamic_contract_type = 'qard_hasan'` (`facility.product_id`). `facility.contract_type` is free text and is not read, as section 3.1 step 2 says.
- Cap leg: `contract_amount` (or `limit_amount`) above `qard_cap_iqd`, compared only when the currency is `IQD`. A loan in another currency is not compared by this leg: the document keeps the DSL default `currency_mode: separate` with an IQD threshold only, and a deal row holds no IQD-equivalent amount. The two legs below still apply to it.
- Staff leg: the recipient party has `staff_flag` TRUE.
- Related-person leg: the recipient party has `related_to_staff` TRUE, or a current `party_relationship` row of type `family` joins the recipient and a party whose `staff_flag` is TRUE, in either direction. `related_to_staff` is filled only by the source's mapping, when the CBS holds such a flag; no job derives it and it is never written FALSE. A NULL flag with no such row does not fire the leg.
SCN-ISL-07 (deals of one customer with `contract_type = 'qard_hasan'`; N is the new deal and `d` its `opened_on`):
- Repeat leg: another deal with status `settled` or `closed` and a `closed_on` in the `repeat_months` months before `d`. `closed_on` is the date of full repayment; a `settled` deal with a NULL `closed_on` is not evaluable for this leg.
- Unpaid leg: another deal opened before `d` whose status is `active` or `defaulted` on `d`.
- `cancelled` and `unknown` deals feed neither leg.
- Schedule sources: `canon.facility.events` has no event kind for a repayment in this plan, so the facility leg is not evaluable until the schedule mapping names one. The document grades B on such a source, as the row says, and its facility leg is P2.

### 7.10 Freeze state with group tasks (V-26-06-a, HIGH; DR-413)
Section 3.8 gains the state rule. Phase 20's `on_level` moves a freeze to `frozen` at the first verified level of its task. With several group tasks that would call the freeze frozen while the accounts of another group are unverified, and `timer_freeze_due` and `timer_freeze_limit` would then do nothing for them.
When a freeze has more than one group task, `FreezeService::on_level(tx, freeze, level)` holds the freeze row `FOR UPDATE` (phase 20 lock order: the owner before the task) and then reads `verification_level` of every group task: the rows of `casework.ops_tasks` with `source_kind = 'freeze'`, `source_id = freeze.id` and `action IN ('freeze', 'restrict', 'hold')`.
1. Any group task has a NULL level: the freeze stays `listed`. Nothing is written on the freeze, `freeze.frozen` is not audited, and both timers keep their meaning.
2. Every group task has a level: the freeze moves to `frozen` with `frozen_at = now()`, and `freezes.verification_level` is the lowest level of the group tasks. A later rise sets it to the new lowest; it only rises, because each task's level only rises. Audit `freeze.frozen` once, with that level. `request_committee_report(reason = verified)` runs at this point, and at the deadline in any case.
The lock makes two group tasks verified at the same moment run one after the other, so the second one sees the first one's level. A freeze with one task behaves as phase 20 ships.

### 7.11 Aggregate counts and their query (V-26-07-a, LOW; DR-413)
`AggregateCounts`, the return of `ShariaAggregateRepository::counts(period)`, holds the cells of section 3.9 step 3: `by_contract_type` (per contract type: active, settled, closed, other), `by_evidence_status`, `islamic_book_accounts` and `gaps` (three pairs of numerator and denominator: no asset, no supplier or seller party, evidence unknown). `counts` runs the one statement of section 3.9 step 2.
`ShariaAggregateQuery::view(ctx, period) -> ShariaAggregateView` runs steps 1 and 4 to 7 of section 3.9 around that call.

### 7.12 Islamic document in the release manifest, and rule 15 (V-26-08-c, MEDIUM; V-26-08-d, LOW; DR-413)
Section 4.5 item 7 gains two points of the phase 16 extension that this phase owns (phase 16 section 7.2):
- The release manifest also lists `packs/scenarios/islamic.v1.0.0.json` with its SHA-256 and signature. `akr-ctl packs seed` checks it as it checks `packs/scenarios/documents.v1.0.0.json` (phase 06 section 3.1 step 1), whether the flag is on or off.
- Phase 16 rule 15 applies to `pack: islamic` as to `pack: akr`: a document without `evidence_label` is refused with `E_SCHEMA`.

## Decisions

Numbered from DR-246, the first number the lead assigned after DR-245. Each decision is one paragraph with its rationale. The lead adds them to `docs/plan/research.md`; this file does not edit it.

**DR-246 (where the deal and evidence tables live).** The deal tables are in `canon` (`canon.islamic_deals`, `canon.deal_parties`), because they are CBS-sourced facts written by the `CanonicalWriter` through `akr_ingest`, with the phase 10 conventions (lineage, bi-temporal deals, no foreign key into bi-temporal parents). The possession-evidence table is in `kyc`, not `canon`, as `kyc.possession_evidence`: an analyst writes it, and `canon` is read-only for `akr_app`, so a verified status on a canonical row could never be set by the application. The evidence status is therefore a view over the analyst rows (`kyc.deal_evidence_status`) that yields `verified` only from an analyst's verification and `unknown` otherwise; a CBS row cannot carry `verified`, and no mapping can write the status. DATA-MODEL section 20 lists the evidence table under `canon`; this decision moves it and the DATA-MODEL line should follow.

**DR-247 (the contract-type list: the phase 10 list, widened by four values).** The product list of phase 10 is the base. Four values are added to `canon.product.islamic_contract_type` (`tawarruq`, `diminishing_musharaka`, `wakala`, `wadiah`), because each is a contract the research names (file 24 section 3 and I-01) and a bank may hold it. The deal's list is the same twelve values. Sukuk, takaful and sarf (I-01) are not deal contract types: a sukuk is an instrument, takaful is insurance and sarf is an exchange transaction, so they stay out of this table (Open doubts 2). The product code map (phase 10 kind `product`) gives the type through the product row, so the CBS need not keep a type on the agreement or the account.

**DR-248 (possession evidence: a register warning for gaps, an analyst verification, and no alert).** Missing and late evidence are a process gap, not a finding about conduct, so they are a counts-only warning in the phase 24 register, written by a daily sweep and gated by `islamic.evidence_tracking`, which stays off until the bank declares the evidence process live (until then `unknown` is not `missing`). A duplicate document number is recorded only as a count for the MLRO, because a register or audit row would tell internal audit that the same document sits on another deal. Verification is a second person's decision under step-up `decision`, enforced in the handler and by the table CHECK `decided_by <> attached_by`. Open doubts 1 gives the bank the choice of an alert.

**DR-249 (no Islamic factor in any score; Islamic types are applicability, never a trigger).** The contract type and the book flag never enter the rating model (phase 15), the feature catalogue (phase 13) or a fairness segment. An Islamic scenario may use the type to decide that it applies, and must have a further condition (`packs verify-data` refuses a document without one). This follows I-30 and S-37 and is why I-28 is not carried.

**DR-250 (the Sharia board sees counts through dashboard.read; the board is not a user).** The Sharia board is not a role (MODULES s.0 has none) and its members are not users. The aggregate view is read by M, CH and IA, which the scope `dashboard.read` already gives, and it returns counts only: by contract type, evidence status and gap rate, with small cells suppressed by the phase 21 threshold. It has no drill-down, no alert, case, report, party or scenario count, and no identifiers. The compliance head presents the figures; the board pack is Open doubts 10.

**DR-251 (the Islamic library is a pack of eight documents that install in shadow and go live through the existing gate).** The eight documents (SCN-ISL-01 to 08, section 3.6) cover the hard rules of I-04, I-06, I-07, I-08 and the inter-book alert of I-17, with the basis labels of research file 24 section 8 and the evidence labels S and N. They install only while `pack.islamic` is on, in shadow, and go live through G9 with `pack.akr_live` (extended to `pack: islamic`) and a change request, the same route as any AKR library scenario, so a release never replaces a version the bank approved (phase 16 section 3.15). Turning the flag off waits until no islamic version is live. The register rows exist while the flag is off, graded D by phase 06, so the coverage view shows the pack as not enabled rather than hiding it.

**DR-252 (the pooled-fund freeze groups accounts by class and never freezes the pool).** Phase 20's freeze creates one task. For I-10 the task follows the account class: unrestricted accounts get `freeze`, restricted accounts `restrict`, and pooled investment accounts `hold` (phase 20 verification unchanged), so that the customer's own account is held while the pool's profit allocation and the other holders' shares are not in the instruction. Neither a Finance nor a Sharia board task is created, because neither role exists; the neutral task path of phase 20 is the only one, and the routing is an open doubt (Open doubts 9). The class mapping is an admin setting, because the bank decides which product is restricted.

**DR-253 (the deal read reuses the phase 18 party rule and adds only the deal actions).** A deal is read under the tier and team of its customer's party: `islamic.deal.read` is a Cedar action on the phase 18 `Party` entity, and its forbids repeat the three phase 18 party forbids for the new actions, because phase 18's forbids name their own action list. A refused deal is a 404 with the unknown-id body, as a party is (phase 18 section 3.10). Other parties of a deal appear only when the viewer may read them, with no count of the rest. This keeps one predicate for parties and deals.

**DR-254 (data-quality rules use the phase 12 kind `validation_rule` and carry their codes in `detail`).** The three rules of section 3.2 (asset, supplier and settlement currency missing) are phase 12 validation failures, so the `dq_issues` CHECK on `kind` does not change. The codes are in `detail.code`. Possession evidence is not a source field and is not a data-quality rule; its gaps are the warning of DR-248.

**DR-255 (the Cedar file is `88_islamic.cedar` in the 80 range, with copied forbids).** The file takes the first free number of the detection range (80, 81, 82, 84 and 86 are taken). Phase 18's forbids cannot be extended from another file, so the three party forbids are repeated for the Islamic actions. Phase 03's `Ctx` fields `purpose` and `network` are required, so the file needs no `has`.

**DR-256 (no new job except the evidence sweep; no new settings beyond five; no new outbox topic).** The library runs in phase 16's own jobs, the freeze in phase 20's start, and the Sharia view is a read; the single job is the daily gap sweep. The five settings are the two governed evidence settings, the gap-rate maximum, the account-class map and the sweep schedule. The statutory numbers (IQD 10 million, two years, six months) are scenario parameters with min equal to max, not settings, so no bank can loosen them by a setting.

**Lead corrections (review 2026-10-09).** A pooled hold task is created without `second_person_only`: phase 20 sets that flag only with verification level `v0`, and its CHECK refused the insert, which rolled back the whole freeze start. The evidence file is opened through the added route `GET /islamic/deals/{id}/evidence/{evidence_id}/content` (phase 18 content pattern): no generic `GET /evidence/{id}` exists, so the verifier could not see the document it decided on. The sweep's register warning uses source `scan` and column `body` (phase 24's closed list and real column). The G1 evidence tile is M's only, since G1 is `dashboard.read`. Access events use phase 02's closed actions (`view`, `download`). `akr_retention` gets SELECT on the three tables, because phase 23's `SELECT ON ALL TABLES` covers only tables that existed then. Suppression is below the governed threshold, not a fixed 1 to 4.

## Open doubts (for gap pass 1 and the bank)

1. **Alert or register for missing evidence.** I-05 says "alert". This phase writes counts-only register warnings for missing and late evidence (DR-248) and a count tile for duplicates. If the bank wants alerts, each needs a scenario version the bank approves, and a per-deal alert of a restricted scenario would be a suspicion row. Decide with the MLRO.
2. **Disbursement and supplier-payment records (I-02).** The three tables hold the deal, its parties and the evidence; the disbursement and supplier-payment records that I-02 links to the deal are not built. SCN-ISL-04 and SCN-ISL-07 use `txn` rows through the account and need the bank's mapping of the disbursement link; without it their readiness stays at B or D. Sukuk, takaful and sarf are not deal types here (DR-247).
3. **Deals versus facilities for qard hasan.** A qard hasan with an agreement table is a deal; one with only schedule tables is a `canon.facility` row (SCN-ISL-06 reads both, the mapping chooses, never both for one agreement). The bank confirms which source tables hold agreements per product.
4. **I-08 items not shipped.** Qard hasan to a recipient abroad (IS-Q2), the repeat check across banks (part of IS-Q3, which ships only the in-bank form), the bank-level 10 percent aggregate cap (IS-Q5), and the class-mismatch check (IS-Q6) are not among the eight documents. Their documents need a ninth to twelfth slot; the lead decides whether the pack grows by M18's "pack updates" (P3) or now.
5. **I-09 (mudaraba and PSIA integrity checks)** is not in the lead's list and not carried. It needs the pool, weightage and profit allocation tables, which no installation studied maps. The bank confirms whether it is wanted for this phase.
6. **Attestation of the library's basis.** The basis labels are the research file's (CBI 2018 page or "analysis"). The AML/CFT Office agreement that research file 27 asks for, and the back-test on the bank's own history, are the bank's steps before go-live; the phase makes no claim that they are done.
7. **Arabic names and summaries.** DSL A.1 requires `name` and `summary` in Arabic and English. The English names are given here; the Arabic text is written by compliance (X5 glossary), not by this draft.
8. **Protected attributes.** `islamic_contract_type` and `islamic_book` are not rows of `admin.protected_attributes`, because that table's `attribute_class` is a closed list of personal classes (phase 24). A new class `product_type` (with `sole_trigger_ok` FALSE) would make the fairness suite test them directly; the lead decides whether the list grows.
9. **Finance and Sharia board routing of the pooled freeze.** I-10 names Finance and the Sharia board for pooled funds. Neither is a role (MODULES s.0), so this phase uses the neutral `hold` task with phase 20's verification (DR-252); whether a pooled hold needs a second person before completion is the bank's call and needs a phase 20 rule, not `second_person_only`. A Finance or board task needs a role decision, a Cedar permit and a task path of phase 04 (the lead decides; not invented here).
10. **Board pack.** The Sharia board receives figures through the compliance head. A PDF or board pack is not built; the periodic `board` kind of phase 21 is the bank board's, and a Sharia board pack would need its own kind and an export step-up.
11. **Duplicate-document routing.** The duplicate count reaches M only and has no list route. M needs a way to see which deals share a reference; a list route would be a sixth route (and needs its own tipping-off review). Decide whether M's list of duplicates is wanted.
12. **Account-class defaults.** `islamic.account_class_map` defaults `mudaraba` to pooled and `qard_hasan` to restricted. The bank decides the classes, and whether a restricted investment product (for example a fund for a named purpose) is also in the restricted group.
13. **The three-value book flag (I-17).** Phase 10 keeps `canon.account.islamic_book` as a boolean. I-17 names three values (conventional, Islamic window, Islamic bank). The bank-level kind is an installation setting, not an account column; the per-product-line onboarding block is not built.
14. **Evidence-object lock.** The R3 lock of `evidence.lock_days` (2555 days) is longer than some deals; the bank confirms that the file's lock is the intended keep period for possession records.
15. **Duplicate key naming.** The keyed hash of the document reference uses the phase 02 purpose `islamic.document_ref`; the purpose list of phase 02 gains it. Confirm the purpose name with the key inventory (phase 00).
16. **Sharia approvals (IS-X2).** `sharia_approval_ref` is stored, but no scenario checks that a product code has a current approval: the approval register is phase 15's I-15 and is not carried.
17. **Scenario SCN-ISL-04 readiness.** Its clause data is not held by any studied source; it ships at readiness D and shows as "not covered". The bank may decide to drop it until clause data is mapped.
18. **Latency of SCN-ISL-02 and 03.** They are L1 (per batch) because the currency and the buy-back are known at posting; the bank may prefer L2 to reduce load.
19. **Visibility of SCN-ISL-08.** It is `normal` because its signal is the book. A bank that treats an inter-book movement as a conduct signal would make it `restricted`.
20. **Deal-party display names.** The deal view shows display names of readable parties. A stricter bank may want ids only, leaving names to the party page (W4).
21. **Retention of superseded deal rows.** Superseded versions of a deal keep the closure clock of the current row; the bank confirms that superseded versions are kept as long as the current row.
22. **Size.** The brief says S; the template rule gives M for six routes (the content and verify routes are added beyond API.md 3.18; both are needed for four-eyes verification). Kept at M.
23. **Routes that depend on the flag.** The flag is checked per request, which makes the router mount independent of it. A bank that never uses the pack may prefer the routes not mounted at all (a build flag); the behaviour required here is 404 either way.

## What This Phase Does NOT Touch

- No change to the Islamic product rows already loaded by phase 10 beyond the widened CHECK; no change to `canon.facility`, `canon.account`, `canon.party` or the customer tables; no change to phase 10's `contract_type` free text.
- No change to the screening runs, lists or scorer of phase 14, and no new screening subject kind or run kind; no change to the rating model, the features or the fairness metric set of phases 13, 15 and 24 beyond the test of section 3.10.
- No change to the scenario engine, the detection payload, the alert identity or the suspicion rules of phase 16 and 17; the Islamic scenarios run through them unchanged. The pack seed and G9 are extended as stated in 4.5 only.
- No change to the case access list (`casework.case_acl`), the party predicate, the case and party Cedar files `90_alerts.cedar` and `91_cases.cedar`, or any other Cedar file; `88_islamic.cedar` and the two action declarations are the only additions.
- No change to phase 20's freeze trigger, the screening freeze, the freeze release workflow or the neutral texts; only the task grouping of section 3.8 and the group state rule of section 7.10.
- No new role (no Finance, no Sharia board, no board member as a user), no new step-up class (`record_open` and `decision` are used as they are), no new problem code (the codes of section 3.11 are API section 6 codes) and no new outbox topic.
- No retention job and no change to phase 23's engine; the tables only take their classes and grants.
- No CBS connection, no CBS write and no new source grant: the deal tables are loaded by the phase 11 path under the existing `akr_ingest` grants.
- No change to the audit chain, anchors or signing (phase 02); no SIEM use-case rule beyond the `siem` flags of section 3.12.
- No report, filing, goAML field or CTR for Islamic deals (phase 21 is unchanged); no PDF or export of the aggregates.
- No change to phase 24's register table or its routes; the warnings use its existing insert pattern.
- No `akr_readonly` grant.
