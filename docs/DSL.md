# Declarative Documents Specification — v1

Status: planning draft (2026-10-07). This document is the human reference for the four kinds of declarative document
that govern what AKR-TMS detects and how it reads the bank's data: **scenario documents** (Part A), **mapping
specifications** (Part B), the **rating model** (Part C) and **change requests** (Part D). Approvals around them reuse
the DBS workflow DSL (Part E). The machine reference is one JSON Schema per kind, checked in at
`backend/schemas/{scenario,mapping,rating-model,change-request}.v1.json` (generated from this spec in P1) and served
at `GET /api/v1/dsl/{kind}/schema`.

Sources: research 32 s.7 (engine), 27 Part B (library, B24 tuning, B26 catalogue and studio), 21 s.2, 3, 9, 10, 11
(rule families, segments, hibernation, latency classes, reason codes), 25 s.14 (mapping layer), 26 s.14 (rating
model), 29 s.9f (change management), 32 s.11 (maker-checker and workflow reuse); DBS `docs/DSL.md` for the shape.

Principles:

1. **Semantics only, no code.** No document holds SQL, scripts or URLs. Aggregates use a closed operator set; small
   conditions use a sandboxed CEL subset with a cost limit and no I/O; transforms come from a closed Rust library.
   Table and column names come from the canonical catalogue (DATA-MODEL s.5) or the source catalogue, never from the
   document text (S-51, C-50, C-54).
2. **Versioned and immutable.** A published version never changes. Alerts, ratings and batches record the versions
   that produced them, so any output can be reproduced (S-52, C-51, K-26).
3. **Validated twice, loudly.** JSON Schema first (`jsonschema` with `default-features = false`, so no remote `$ref`
   is fetched), then semantic checks that return structured errors (section 7) actionable from the path and code
   alone.
4. **Bilingual by construction.** Every human-visible string is an `I18nString`; Arabic is the original. Publishing
   with a missing `ar` or `en` is an error.
5. **Governed, not configured.** Nothing in a document takes effect until a change request (Part D) is approved by a
   second person with a fresh step-up. The maker never approves (D20, GV-53).
6. **Unknown is never low.** Every document kind states what happens when an input is missing, and the answer is
   never "treat as zero" or "skip silently" (S-10, K-04, D21).
7. **Never tuned to capacity.** No field can cap the number of alerts, cases or reviews (S-39). Capacity is handled
   by ranking, grouping and staffing (ARCHITECTURE s.8).

## 0. The document kinds

| Kind | Part | Stored in | Owner module | Who authors | Who approves (default) |
|---|---|---|---|---|---|
| Scenario | A | `scn.scenario_versions.document` + `scn.parameter_sets` | M11 | CFG (compliance analyst); implementer on request | MLRO + a second approver outside operations (Part E `cr_material`; `cr_material_validated` for CBI-baseline logic) |
| Mapping specification | B | `admin.mapping_versions.document` + `admin.code_map_entries` | M6 | DS (implementer or bank IT) | Bank compliance or IT officer (`cr_mapping`) |
| Rating model | C | `kyc.rating_models.document` | M10 | CFG with the MLRO | Validator, MLRO and a second approver (`cr_material_validated`) |
| Change request | D | `admin.change_requests` | M15 | Any maker with the authoring scope for the object | Routed by object kind and change class (Part E) |
| Workflow definition | E | DBS workflow tables in `admin` (ported in P0, docs/plan phase 04) | M15 | ADM | Two administrators |

Common envelope (every kind except the change request, which is a row):

```json
{
  "dsl": "scenario",
  "dsl_version": "1",
  "id": "SCN-CASH-02",
  "version": "1.2.0",
  "name": { "ar": "...", "en": "..." },
  "description": { "ar": "...", "en": "..." },
  "owner": "role:M",
  "changelog": { "ar": "...", "en": "..." },
  "metadata": { "tags": ["cash", "structuring"] }
}
```

| Field | Type | Req | Notes |
|---|---|---|---|
| `dsl` | enum | yes | `scenario`, `mapping`, `rating_model` |
| `dsl_version` | string | yes | `"1"` for this spec; unknown versions are refused (`E_UNKNOWN_DSL_VERSION`) |
| `id` | string | yes | Stable identifier of the family (format per kind) |
| `version` | semver or int | yes | Per kind: scenario logic semver (A.10), mapping integer, rating model semver |
| `name`, `description` | I18nString | yes / no | Shown in libraries, cards, alerts and the audit trail |
| `owner` | RoleRef | yes | A role code from MODULES s.0 (`role:M` for the MLRO, `role:CFG`) or a named user id; drives authoring scope in Cedar |
| `changelog` | I18nString | on publish | Shown in the version picker, the scenario card and to assessors |
| `metadata` | object | no | Free tags; never read by an engine |

## 1. Shared types

**I18nString** — `{ "ar": "...", "en": "..." }`. Both keys required on publish. Placeholders use `{name}` and must
be the same set in both languages (`E_I18N_PLACEHOLDERS`).

**RoleRef** — `"role:<code>"` with a code from MODULES s.0 (A1, A2, SS, M, KYC, RFI, QA, OPS, CFG, CHK, DS, CH, MRO,
IA, AS, L, ADM, SEC, VS), `"pool:<key>"` for a work pool, or `"user:<uuid>"` (discouraged, `W_USER_REF`).

**Duration** — `{ "value": 7, "unit": "days" }`. Units: `minutes`, `hours`, `days`, `months` (wall clock) and
`business_days`, `business_hours` (computed against the bank calendar in `ref.calendars`, M8). ISO 8601 strings
(`"PT10M"`) are accepted where a field says so (mapping change settings).

**Money** — `{ "amount": "10000", "currency": "USD" }`. Amount is a decimal string (never a float). A threshold with
no currency is refused (`E_MONEY_NO_CURRENCY`); IQD and USD are separate by default (T-04). Conversion, where a rule
asks for it, uses `amount_lcy` or `amount_usd` from the canonical row (DATA-MODEL conventions), never a rate in the
document.

**CanonicalRef** — `entity.attribute` from the canonical catalogue, for example `txn.amount_usd`, `txn.cash_flag`,
`customer.expected_profile.monthly_income`, `cash_event.depositor_id`. A trailing `?` marks the input optional: the
scenario still runs without it, in a degraded form that the readiness grade records (S-03). Unknown names are refused
(`E_UNKNOWN_ATTRIBUTE`). Feature values are `feat.<feature_key>` (a row of `feat.feature_definitions`), rating
values `rating.level`, `rating.cbi_level`, and list facts `screening.<fact>`.

**ParamRef** — `:name`, a parameter declared in the same document. Only parameters, literals and canonical refs may
appear in conditions; every parameter is bound at execution, never interpolated.

**Expr** — a CEL expression string over a typed context declared per use (A.6.2, B.1). The AKR subset: comparison,
arithmetic, logical operators, `in`, `has()`, `size()`, string `startsWith/endsWith/contains/matches` (RE2,
length-limited), `timestamp`/`duration` arithmetic, and the AKR functions `fold_ar(s)` (Arabic folding, file 22),
`within_band(x, lo, hi)`, `pct_of(x, y)`, `days_between(a, b)`, `max(a, b)`, `min(a, b)`. No user-defined functions, macros limited to
`all`, `exists`, `map`, `filter` over bounded lists. Each expression is compiled at validation and refused above a
cost bound (`E_EXPR_COST`); evaluation has a per-call budget and fails closed (the record is quarantined, never
skipped).

**Filter** — the restricted expression used in aggregate `where` and `having` clauses (A.6.1). It is a subset of
Expr that the compiler can translate to SQL: comparisons, `AND`, `OR`, `NOT`, `IN (literal list)`, `IS NULL`,
`BETWEEN`, arithmetic, and the functions `within_band`, `pct_of`. Anything outside the subset is refused
(`E_FILTER_NOT_COMPILABLE`).

**BasisTag** — why a parameter value is what it is (file 27 B24.2): `CBI` (a CBI number as written), `CBI-2016` (the
2016 FX caps, status unverified), `MER` (MENAFATF mutual evaluation), `KYC` (a ratio to the declared profile), `PCT`
(a segment percentile), `DP` (design proposal: time-limited, must carry a review date, S-40), `BANK` (the bank's own
risk assessment), `REG` (another regime's rule the bank adopted). Every parameter carries one.

**CbiRef** — `{ "article": 1, "no": 6, "rel": "implements" }` for the CBI catalogue of 8 January 2025 (articles 1-3
numbered scenarios, articles 4-14 indicators; DATA-MODEL `scn.cbi_catalogue_items`). `rel`: `implements`, `extends`,
`proxy` (a partial implementation that the coverage report shows as such). Display form "Art.1 no. 6". References
that do not resolve to a catalogue item are refused (`E_CBI_REF_UNKNOWN`).

**ReasonCodeRef** — a stable reason code id from the reason-code library (`ref.reason_codes`, family `detection`),
for example `CASH02_SPLIT_BELOW_LIMIT`. The library holds the bilingual templates, the typology tag, the CBI mapping
and the fields to display (T-44); documents reference codes and pass facts into their placeholders.

**SegmentRef** — `segment:<key>` (a row of the admin-managed segment register, DATA-MODEL `feat.segments`), a risk
level `risk:<1-5>`, or a currency `ccy:<ISO>`. Overrides combine them (A.5).

**Classification** — `public`, `internal`, `confidential`, `restricted`. A scenario marked `restricted` (for example
the public-funds family, S-29) produces alerts only the suspicion tier and the named scenario owners can see.

**EffectiveWindow** — `{ "from": "2026-11-01T00:00:00+03:00", "to": null }`. Times carry the bank's zone (Iraq UTC+3)
and are stored as UTC (C-18). `from` is set by the activation of the approved change request, never typed by the
maker for a material change.

---

# Part A — Scenario documents

A scenario is the declarative definition of one detection pattern (file 27 B26.4; requirements S-51, T-01). The
scenario register (`scn.scenarios` + versions + parameter sets) is the single source for the studio, the engine, the
scenario card, the CBI coverage report and the assessor pack (S-07).

## A.1 Document structure

```json
{
  "dsl": "scenario", "dsl_version": "1",
  "id": "SCN-CASH-02", "version": "1.2.0",
  "name": { "ar": "...", "en": "..." },
  "family": "CASH",
  "pack": "akr",
  "typology": ["cash_placement_structuring"],
  "cbi_refs": [ { "article": 1, "no": 2, "rel": "implements" } ],
  "owner": "role:M",
  "classification": "confidential",
  "evidence_label": "P",
  "phase2_candidate": false,
  "latency_class": "L1",
  "data_requirements": ["txn.cash_flag", "txn.amount_usd", "cash_event.depositor_id?"],
  "aggregation": { "level": "customer", "linked_parties": ["family_no", "depositor"], "net_reversals": true,
                   "posted_only": true, "settle_delay": { "value": 2, "unit": "minutes" } },
  "parameters": { "<name>": { "...": "..." } },
  "logic": { "kind": "aggregate", "...": "..." },
  "exclusions": { "...": "..." },
  "suppression": { "...": "..." },
  "outputs": { "...": "..." },
  "tests": { "...": "..." },
  "changelog": { "ar": "...", "en": "..." }
}
```

| Field | Type | Req | Notes |
|---|---|---|---|
| `id` | string | yes | `SCN-<FAMILY>-NN`. Families from file 27 B4-B21: CASH, XFER, TRADE, ISL, DIGI, LOAN, ACCT, PEP, PUB, CORR, TF, PF, INS, DORM, CHQ, FX, OTH, OBS. Bank scenarios use `NN` 901-999 (`E_NAMESPACE`, S-05) |
| `family` | enum | yes | Must match the id prefix |
| `pack` | enum | yes | `cbi` (baseline, never removed), `akr` (library), `islamic`, `regime` (PUB, PF-Iran, TF district), `bank` |
| `typology` | string[] | yes | Tags from the shared typology set used by the risk assessment and the STR typology field (S-55) |
| `cbi_refs` | CbiRef[] | no | Required for `pack: cbi`. A bank scenario may implement CBI items too |
| `classification` | Classification | yes | `restricted` limits alert visibility to the suspicion tier and named owners |
| `evidence_label` | enum | yes for `akr` | `P` (published case evidence), `S` (supervisory text), `N` (inferred, to validate on the first bank's cases, S-59) |
| `phase2_candidate` | bool | yes | Marks scenarios that may run in the pre-release API later (S-26); no effect in P1 |
| `latency_class` | enum | yes | `L1` near-real-time after each batch, `L2` intraday or daily, `L3` periodic (T-40). `L0` is reserved for phase-2 checks |
| `data_requirements` | CanonicalRef[] | yes | Every attribute the logic, exclusions or outputs read must be listed; the compiler refuses unlisted reads (`E_UNDECLARED_INPUT`). Feeds readiness grades and the coverage matrix (S-03, C-48) |
| `aggregation` | object | aggregate and profile kinds | A.4 |
| `parameters` | map | yes | A.5 |
| `logic` | object | yes | One of three kinds (A.6) |
| `exclusions`, `suppression` | object | no | A.7 |
| `outputs` | object | yes | A.8 |
| `tests` | object | from `testing` on | A.9 |

## A.2 Identity and lineage

- The id is stable for the life of the scenario. A new logic version keeps the id; tuning keeps the id and the logic
  version and creates a new parameter-set version (file 27 B26.1).
- `pack: cbi` scenarios ship in the CBI baseline pack and are never retired or deleted by the product or the bank
  (S-01, T-02). A bank may disable an `akr` extension scenario and retire its own `bank` scenarios only.
- Each version records the CBI pack version it was approved under (for example "CBI-2025.01 + CBI-2026.08 circulars,
  mapping r3"); the coverage report states the CBI pack version it was produced under (S-02, S-52).

## A.3 Data requirements and readiness

- Required inputs (no `?`) must be mapped and above their minimum completeness for the scenario to reach `shadow` or
  `live` (S-47). When a required input later falls below its minimum, the scenario stays live but is shown as "not
  operating as intended" on the card, the readiness view and the command centre; it is never switched off silently.
- Optional inputs (`?`) degrade the scenario when absent. The readiness grade (A-D, `admin.readiness_grades`) is
  computed from the requirements and the field availability of the bank's sources; grade D produces a "not covered"
  declaration with owner, reason, interim route and review date (S-03).
- The studio picks data fields from the readiness view; a field that is not available at the bank is shown with the
  vendor request text.

## A.4 Aggregation level and counting rules

```json
"aggregation": {
  "level": "customer",
  "linked_parties": ["family_no", "phone", "address", "depositor", "companion"],
  "net_reversals": true,
  "posted_only": true,
  "settle_delay": { "value": 2, "unit": "minutes" },
  "currency_mode": "separate"
}
```

| Field | Values | Notes |
|---|---|---|
| `level` | `account`, `customer`, `counterparty`, `branch`, `staff` | The grouping key of the measures |
| `linked_parties` | subset of `family_no`, `phone`, `address`, `depositor`, `companion`, `signatory`, `owner` | Aggregates across parties linked by the shared linked-party service (S-08). Kinship links (`family_no`) are excluded from automated scores unless validator-approved (T-24); for scenarios they widen the group, and the alert names every link used |
| `net_reversals` | bool, default `true` | Reversals net against their original (S-09) |
| `posted_only` | bool, default `true` | `false` only for scenarios about attempts (rejected, cancelled, pending; C-17); the logic must then name `txn_event.kind` |
| `settle_delay` | Duration | Evaluation waits this long after the batch so late rows of the same business moment are included (C-22) |
| `currency_mode` | `separate` (default), `usd_equivalent`, `lcy_equivalent` | `separate` evaluates IQD and USD independently with their own thresholds (T-04) |

Counting rules that the engine applies to every scenario and that no document can switch off: amounts are positive
with a debit/credit flag; system-generated entries (`txn.system_generated`) are excluded from customer behaviour
measures unless the scenario names them; late rows recompute affected windows, and an alert already raised is
**amended with an explanation**, never silently changed (S-09, C-22).

## A.5 Parameters

```json
"parameters": {
  "min_deposits": { "type": "int", "default": 3, "min": 2, "max": 10, "basis": "DP",
                    "review_by": "2027-04-30", "band": [3, 5] },
  "window": { "type": "duration", "default": { "value": 7, "unit": "days" },
              "allowed": [ { "value": 7, "unit": "days" }, { "value": 14, "unit": "days" },
                           { "value": 30, "unit": "days" } ], "basis": "DP", "review_by": "2027-04-30" },
  "total": { "type": "money", "default": { "amount": "10000", "currency": "USD" },
             "min": { "amount": "3000", "currency": "USD" }, "basis": "CBI",
             "overrides": [ { "when": ["segment:cash_retailer"], "value": { "amount": "15000", "currency": "USD" },
                              "basis": "PCT" } ] },
  "just_below": { "type": "range", "default": [0.70, 0.999], "min": 0.5, "max": 0.999, "basis": "DP",
                  "review_by": "2027-04-30" }
}
```

| Field | Notes |
|---|---|
| `type` | `int`, `decimal`, `money`, `ratio` (0-1 decimal), `range` (two numbers), `duration`, `bool`, `enum` (with `options`), `list` (of literals, max 200) |
| `default` | Required. Must sit inside `min`/`max` or `allowed` |
| `min`, `max`, `allowed` | Hard bounds. A value outside them is refused at validation; changing the bounds is a logic change (minor version) |
| `band` | The approved band. A parameter-set change inside the band is a **minor** change; outside it, or any loosening, is **material** (file 29 s.9f; Part D) |
| `basis` | BasisTag, required. `DP` requires `review_by` (S-40); expired `DP` values raise `W_DP_EXPIRED` on the card and a review task |
| `overrides` | Ordered list; the first matching `when` wins. `when` combines SegmentRefs (segment, risk level, currency). Each override has its own `basis`. Tighter values for higher risk levels are expected (file 27 B24.3) |
| `description` | I18nString, optional; shown in the studio and on the card |

Rules:

- No parameter may limit the number of alerts produced (no `max_alerts`, `top_n`, `limit`). The schema has no such
  field, and a parameter whose name or use suggests one is refused (`E_ALERT_CAP`, S-39). Ranking-based output for
  anomaly detectors (T-18) belongs to the model registry (M15), not to scenarios.
- A parameter set is stored separately (`scn.parameter_sets`): `{ scenario_version_id, version, values, basis,
  reason_code, change_request_id }`. The document's `default` values are the shipped values; the bank's values live in
  the parameter set and are what the engine binds. The card shows both.
- Protected attributes (gender, religion, ethnicity, nationality, tribe, sect) may not appear in a segment, override
  condition, filter or grouping; age band and occupation only where a CBI text names them, which the document
  shows with a `cbi_refs` entry (`E_PROTECTED_ATTRIBUTE`, S-38).

## A.6 Logic

Exactly one of three kinds (research 32 s.7.3). Specialised engines (the pass-through matcher of S-28, the motif
engine of T-22, the counterparty resolver of T-29) are not logic kinds: they write **features** (`feat.*`), which
event and profile logic then read like any other input. This keeps the document language closed while letting the
product add engines.

### A.6.1 `aggregate` — measures over a window

```json
"logic": {
  "kind": "aggregate",
  "where": "txn.cash_flag = true AND txn.dr_cr = 'C' AND within_band(txn.amount_usd, :just_below, 10000)",
  "window": { "kind": "rolling", "length": ":window", "calendar": "default" },
  "group_by": "aggregation.level",
  "measures": [
    { "op": "count", "as": "n" },
    { "op": "sum", "field": "txn.amount_usd", "as": "total" },
    { "op": "distinct_count", "field": "txn.branch_id", "as": "branches" }
  ],
  "having": "n >= :min_deposits AND total >= :total"
}
```

| Field | Notes |
|---|---|
| `where` | Filter over the canonical row and the scenario's parameters |
| `window.kind` | `rolling` (ends at the evaluated event or the sweep date) or `calendar` (calendar day, week, month, quarter; business days use the calendar) |
| `window.length` | Duration or a ParamRef to a duration parameter. Longer than the hot data window of the canonical store is refused (`E_WINDOW_TOO_LONG`) |
| `window.calendar` | `default` or a named calendar from `ref.calendars`; the choice is shown in the explanation (T-04) |
| `group_by` | `aggregation.level` (default), or a canonical key (`txn.counterparty_id`, `txn.branch_id`) for scenarios that group below the level |
| `measures` | Closed operator set: `count`, `sum`, `avg`, `max`, `distinct_count`, `ratio` (`{ "op": "ratio", "num": "<alias>", "den": "<alias>" }`). `sum`, `avg` and `max` need a numeric field (`E_FIELD_TYPE`). Each measure may carry its own `where` (filtered measure) |
| `having` | Filter over measure aliases and parameters |

Compilation: the compiler turns the document plus a parameter set into one parameterised SQL statement over the
canonical partitions (table and column names from the catalogue, partition pruning by the window, an `(account_id,
booking_date)` or equivalent index path). It is checked with `EXPLAIN` against a cost guard before it is stored
(`scn.scenario_versions.compiled_sql`) and runs with a `statement_timeout`, on the standby when one exists. No
document text reaches the SQL string; parameters are bound.

### A.6.2 `event` — a condition on one event

```json
"logic": {
  "kind": "event",
  "on": "txn.posted",
  "context": ["txn", "account", "customer", "feat.dormant_days", "feat.p99_single_credit_usd_segment"],
  "condition": "feat.dormant_days >= 180 && txn.amount_usd >= max(:min_usd, :seg_multiple * feat.p99_single_credit_usd_segment)"
}
```

| Field | Notes |
|---|---|
| `on` | `txn.posted`, `txn.event` (any lifecycle event), `account.status_changed`, `party.changed`, `screening.hit`, `observation.recorded`, `rating.changed` |
| `context` | The typed context the condition may read: entity names and feature keys. Reads outside it are refused (`E_UNDECLARED_INPUT`) |
| `condition` | Expr. Must return bool (`E_EXPR_TYPE`) |

Event scenarios run on the fast path right after each batch (L1). The context is assembled by the engine from the
canonical store and the feature tables; the condition cannot query anything.

### A.6.3 `profile` — expected against actual, own history, peers

```json
"logic": {
  "kind": "profile",
  "measure": { "op": "sum", "field": "txn.amount_lcy", "where": "txn.dr_cr = 'C'" },
  "period": { "kind": "calendar", "length": { "value": 1, "unit": "months" } },
  "expected": [
    { "source": "declared", "field": "customer.expected_profile.monthly_income", "test": "ratio_above", "value": ":declared_ratio", "min_excess": ":min_excess" },
    { "source": "own_history", "lookback": { "value": 12, "unit": "months" }, "stat": "median", "test": "ratio_above", "value": ":own_multiple" },
    { "source": "peer", "peer_group": "aggregation.segment", "stat": "p99", "test": "above", "fallback_only": true }
  ],
  "combine": "any",
  "min_history": { "value": 3, "unit": "months" }
}
```

| Field | Notes |
|---|---|
| `measure` | One aggregate measure (A.6.1 operators) over the period |
| `period` | Calendar or rolling |
| `expected[]` | Each comparator has a `source`: `declared` (KYC profile), `own_history` (robust statistics: median, MAD, percentile from `feat.feature_values`), `peer` (segment statistics from the baseline service, S-12). `test`: `above`, `ratio_above`, `robust_z_above`, `pct_rank_above` |
| `fallback_only` | The comparator is used only when an earlier one cannot be computed (missing declared value, short history). When it is used, the alert and the customer are marked **degraded** (S-10) |
| `combine` | `any` or `all` |
| `min_history` | Below it, `own_history` comparators are skipped and the peer fallback applies |

A missing declared value never means "no expectation": the peer comparator applies and the case shows "declared
income missing" as a data gap.

## A.7 Exclusions and suppression

```json
"exclusions": { "register": true, "static": [ { "field": "account.type", "in": ["government_payroll_pool"], "reason_code": "EXCL_POOL_ACCOUNT" } ] },
"suppression": { "hibernation": { "allowed": true, "default_period": { "value": 90, "unit": "days" },
                                  "envelope": ["n", "total"], "margin": 0.2 },
                 "dedupe": { "same_entity_within": { "value": 7, "unit": "days" } } }
```

- `exclusions.register: true` means the governed exclusion register (`admin.exclusions`: owner, reason, expiry,
  change request) applies to this scenario. Static exclusions in the document are logic (minor version) and need a
  reason code each. An excluded record is still counted in the reach metric.
- **Hibernation** (T-38) suppresses the **display**, never the evaluation: the scenario still runs and logs, the
  suppressed result goes to the ledger, and the daily report counts it. Hibernation ends at expiry, when the measures
  named in `envelope` exceed the recorded values by `margin`, or on a trigger (new adverse media, list change,
  authority request, profile change). For `pack: cbi` scenarios hibernation is allowed but the evaluation result is
  always kept for coverage reporting (`E_HIBERNATION_SKIPS_EVALUATION` if a document tries otherwise).
- `dedupe` folds repeats of the same pattern for the same entity into the open alert (ARCHITECTURE s.8: consolidate,
  never drop); the alert fingerprint is `scenario_id + entity + window bucket`.
- Auto-close does not exist in P1 (file 21 s.9b). The schema has no auto-close field.

## A.8 Outputs

```json
"outputs": {
  "severity": { "base": "medium",
                "uplift": [ { "when": "branches >= 2", "to": "high", "reason_code": "CASH02_MULTI_BRANCH" },
                            { "when": "feat.limit_enquiry_logged_7d == true", "to": "high", "reason_code": "CASH02_LIMIT_ENQUIRY" } ] },
  "reason_codes": [ { "code": "CASH02_SPLIT_BELOW_LIMIT", "facts": { "n": "n", "total": "total", "days": ":window" } } ],
  "case_family": "cash",
  "evidence": { "transactions": "all_in_window", "fields": ["txn.branch_id", "cash_event.depositor_id?", "txn.channel_id"] },
  "template": "cash_structuring_v1"
}
```

| Field | Notes |
|---|---|
| `severity` | `base` and ordered `uplift` rules (`low`, `medium`, `high`, `critical`). Severity feeds the priority class; it never decides an outcome |
| `reason_codes` | At least one (`E_NO_REASON_CODE`). `facts` bind template placeholders to measures, parameters or canonical refs so the analyst sees the actual value next to the threshold (T-44). Branch-tier roles see reason codes without thresholds (T-47) |
| `case_family` | The case family the alert groups into (one open case per party and family, M12) |
| `evidence` | What the alert freezes into its snapshot (T-03): the transactions in the window, the listed fields, the parameter values bound, the versions and `data_as_of` |
| `template` | The alert page template id (bilingual, in the case policy pack) |

Every alert records: scenario id and version, parameter-set version, mapping versions, reference-data versions, CBI
pack version, `data_as_of`, the window, the matching measures and the source transaction ids (S-52, T-03). This is
what makes the "why was this an alert" and "why was this not an alert" what-if runs possible (research 32 s.7.5).

## A.9 Tests

```json
"tests": {
  "unit": [
    { "name": "three deposits under the limit in 7 days", "fixture": "fx/cash02_pos_1.json", "expect": { "alerts": 1, "severity": "medium" } },
    { "name": "reversal nets out", "fixture": "fx/cash02_neg_rev.json", "expect": { "alerts": 0 } }
  ],
  "seeded": ["SEED-CBI-A1-02-P", "SEED-CBI-A1-02-N"]
}
```

- At least 3 positive and 3 negative unit cases before `testing` can move on (file 27 B26.3; `E_TESTS_MISSING`).
  Fixtures are synthetic canonical rows; the studio's automatic checks add null, currency, reversal and late-data
  cases.
- `seeded` names known-answer cases in `scn.seeded_tests`; every CBI item has at least one positive and one negative
  case, run on every release and daily in production (S-49).
- Replay, above- and below-the-line sampling and shadow results are not in the document: they are evidence records
  (`scn.replays`, `scn.shadow_results`) attached to the change request.

## A.10 Versioning and lifecycle

Statuses: `draft` -> `testing` -> `shadow` -> `live` -> `retired` (DATA-MODEL `scn.scenario_versions.status`).

| Rule | Detail |
|---|---|
| Logic version (semver) | Major: new operator, window or aggregation level. Minor: new optional parameter, exclusion type or evidence field, bound change. Patch: text, reason-code wording, evidence template (file 27 B26.2) |
| Parameter set | Integer version per scenario and bank, effective-dated, with basis, reason code and change request |
| One live version | At most one `live` version per scenario; others may be `shadow` (`E_TWO_LIVE`) |
| Shadow | A new version runs beside the live one for at least 30 days (two cycles if seasonal) and creates no alerts; the comparison is evidence for approval (S-44, S-47) |
| Kill switch | Falls back to the previous approved version, never to "off" (S-52) |
| Retirement | `bank` scenarios only, on productivity and unique-catch data; a temporary switch-off of an `akr` scenario needs an end date; `cbi` scenarios are never retired (`E_BASELINE_RETIRE`) |
| Library releases | `akr` pack releases (for example "AKR-LIB 1.2") are pinned per bank and run in shadow against the pinned version for one cycle with a difference report before the bank accepts them |
| Go-live gate | Complete card, passing unit tests, replay inside the stated range, below-the-line sample within tolerance, inputs mapped above minimum completeness, maker-checker approval, 30-day shadow (S-47) |

The studio stages (S-50) map to statuses: author and unit-test in `draft`; replay and above/below-the-line in
`testing`; shadow in `shadow`; approval by change request moves the version to `live` at the effective time.

## A.11 Execution contract

| Mode | Trigger | What runs | Output |
|---|---|---|---|
| Incremental | Outbox row after each mapped batch naming the accounts touched | Event logic at once; aggregate logic for the touched entities over indexed ranges; a settled-time re-evaluation after `settle_delay` | Alerts (M12) |
| Sweep | Timer: daily or monthly per `latency_class` | Set-based statement per scenario over recent partitions (not a loop per account) | Alerts |
| Replay | Studio request or change-request evidence | Draft version and a parameter set over an as-of period on the bi-temporal store (S-41) | `scn.replays` + `scn.replay_results`; no alerts |
| Shadow | Version in `shadow` | Same as incremental and sweep, beside the live version | `scn.shadow_results`; no alerts |

Each run writes `scn.scenario_runs` (mode, window, accounts, alerts, cost). A run that would exceed the cost guard is
refused with the estimate (MODULES M11 edge case). Runs are deterministic for a given data-as-of, versions and
parameter set; the independent functional-test harness re-implements each scenario from its document and diffs
"simulated versus deployed" (S-48).

## A.12 Plain-language rendering

The studio and the scenario card render the logic as Arabic and English sentences generated from the document (for
example "three or more cash deposits by the customer or linked parties within 7 days, each between 70 and 99.9
percent of USD 10,000, totalling at least USD 10,000"). The text is generated, never typed, so it cannot drift from
the logic (S-45, S-50). The card itself is generated from the register, test results and monitoring data (file 27
B24.7).

---

# Part B — Mapping specifications

A mapping specification is the declarative document that turns rows of one source object into one canonical entity
(research 25 s.14; C-50, C-51). It is admin-managed data edited in the mapping workbench (T3), seeded from an adapter
pack (C-38) and governed like a scenario. The engine, the transform library and the SQL dialects are product code; the
mapping is the bank's data.

## B.1 Document structure

```json
{
  "dsl": "mapping", "dsl_version": "1",
  "id": "ics_core.txn", "version": 3,
  "name": { "ar": "...", "en": "..." },
  "owner": "role:DS",
  "entity": "txn",
  "source": { "source": "ics_core", "object": "AKR_V_TXN", "format": "relational", "key": ["TXN_ID"] },
  "change": { "strategy": "poll", "watermark": { "kind": "timestamp_column", "field": "LAST_UPD" },
              "overlap": "PT10M", "settle": "PT2M", "lookback": "auto" },
  "filter": [ { "expr": "row.GL_ONLY != 'Y'", "reason_code": "MAP_EXCL_GL_INTERNAL" } ],
  "fields": { "<canonical attribute>": { "from": "...", "do": ["..."] } },
  "validate": [ { "rule": "not_null", "field": "account_id", "on_fail": "quarantine" } ],
  "survivorship": null,
  "approved_catalogue_hash": "sha256:...",
  "changelog": { "ar": "...", "en": "..." }
}
```

| Field | Type | Req | Notes |
|---|---|---|---|
| `id` | string | yes | `<source key>.<canonical entity>`; one active mapping per source and entity |
| `version` | int | yes | Increments on every publish |
| `entity` | enum | yes | A canonical entity from DATA-MODEL s.5 (`party`, `party_name`, `customer`, `account`, `txn`, `txn_event`, `counterparty`, `cash_event`, `payment_message`, ...). Unknown entities are refused (`E_TARGET_UNKNOWN`) |
| `source.source` | string | yes | Key of a registered source (`admin.sources`) |
| `source.object` | string | yes | A bank-owned view (CT2) or replica object (CT1) on the source's approved object list (the footprint register); anything else is refused (`E_OBJECT_NOT_APPROVED`, C-30) |
| `source.format` | enum | yes | `relational` (columns), `t24_xmlrecord` (Transact record text decoded in AKR-TMS, never in the CBS, C-14), `t24_multivalue`, `landing_file` (CT4 contract with manifest, C-39), `iso20022`, `mt` (raw message parsed by a versioned parser, C-41) |
| `source.key` | string[] | yes | The source primary key; lands as `(source, table, pk)` for idempotent loading (C-24) |
| `change` | object | yes | B.2 |
| `filter` | list | no | Expr over the source row; each filter needs a reason code and appears in the coverage statement as an exclusion, so no row is dropped silently (C-27) |
| `fields` | map | yes | B.3. Every attribute the canonical entity marks as required must be mapped (`E_REQUIRED_TARGET_UNMAPPED`) |
| `validate` | list | no | B.5 |
| `survivorship` | object | no | Multi-source attribute rules (B.6) |
| `approved_catalogue_hash` | string | set by the system | Hash of the source columns the mapping reads, taken at approval; compared on every run (B.8) |

## B.2 Change strategy and watermarks

| Field | Values | Notes |
|---|---|---|
| `strategy` | `poll` (default), `stream`, `file` | The change-source interface (C-06): a bank moves between tiers CT1-CT4 by configuration. `stream` only where the bank agreed to CDC (Debezium sidecar, CT3, C-11) |
| `watermark.kind` | `timestamp_column`, `rowversion`, `sequence_id`, `t24_date_time`, `business_date`, `cdc_position`, `manifest` | Per table class (file 25 s.7.2). `none` is refused for fact tables; small reference tables may use `full_compare` with a key-set reconciliation |
| `overlap` | ISO 8601 duration | Re-read window behind the watermark (commit-order problem, file 25 s.7.3) |
| `settle` | ISO 8601 duration | Rows younger than this wait for the next poll |
| `lookback` | `auto` or a duration | `auto` derives the window from measured late-arrival percentiles and widens it automatically (C-21) |

Every batch records the as-of marker the engine offers (SCN, row version, LSN, timestamp or none), the mapping
version and the watermark move, landed with the follow-up jobs in one transaction (C-20).

## B.3 Field mappings

```json
"fields": {
  "booking_date":  { "from": "BOOK_DT", "do": ["parse_date:yyyy-MM-dd"] },
  "amount":        { "from": "AMT", "do": ["parse_decimal", "abs"] },
  "dr_cr":         { "from": "AMT", "do": ["dr_cr_from_sign"] },
  "currency":      { "from": "CCY", "do": ["code_map:ics_currency"] },
  "txn_type":      { "from": "TRN_CODE", "do": ["code_map:ics_txn_type"] },
  "narrative_raw": { "from": "NARR", "do": ["digits_western", "trim"] },
  "cash_flag":     { "expr": "row.TRN_CODE in ['CDEP', 'CWDL']" },
  "branch_id":     { "from": "BR_CODE", "do": ["code_map:ics_branch"] },
  "mothers_name":  { "from": "local:MOTHER.NAME", "attribute": true }
}
```

| Key | Notes |
|---|---|
| `from` | `COLUMN` (view or replica column), `field:NAME` (dictionary field, resolved to its position through the bank's own dictionary at generation, C-37), `local:NAME` (local reference field), `meta:source`, `meta:batch_as_of` |
| `do` | Ordered list of transforms from the closed library (B.4), written `name` or `name:arg` |
| `expr` | Expr over `row` (the source row) for small computed fields; used instead of `from` |
| `default` | A literal used only when the source value is null; a default on a field that feeds a scenario threshold is refused (`E_DEFAULT_ON_MONITORED`) so a missing value never becomes a plausible one |
| `attribute` | `true` stores the value as an extension attribute (`admin.attribute_values`) instead of a canonical column; the attribute must be defined in `admin.attribute_definitions` with a classification |

Classification is fixed by the canonical catalogue (`admin.column_classification`): a mapping cannot lower it. A
personal field that the catalogue marks `enc` or `hk` is encrypted or keyed-hashed on write whatever the mapping says.

## B.4 Transform library (closed, versioned, unit-tested Rust)

| Transform | Argument | Effect |
|---|---|---|
| `parse_date` | format | Text to date; invalid input is a validation failure, never a guessed date |
| `parse_timestamp` | format, zone | Text to timestamp in the bank's zone, stored with derived UTC (C-18) |
| `tz_convert` | zone | Re-zone a timestamp |
| `parse_decimal` | none | Text or engine numeric to decimal |
| `decimal_scale` | n | Divide by 10^n (implied decimals) |
| `currency_minor_units` | none | Scale by the currency's minor unit from `ref.currencies` |
| `abs`, `dr_cr_from_sign` | none | Split a signed amount into a positive amount and a debit/credit flag |
| `xml_field` | path | Read a field from Transact record text (decoded in AKR-TMS, C-14) |
| `multivalue_split` | separator | Split a multivalue field into a list |
| `code_map` | map key | Bank code to canonical code (B.7) |
| `normalise_phone` | country prefix (`+964`) | E.164 normalisation |
| `split_arabic_name` | none | Name parts with roles (file 22) |
| `digits_western` | none | Arabic-Indic and Persian digits to Western digits |
| `trim`, `upper`, `lower` | none | Text clean-up (Latin only for case changes) |
| `iban_validate` | none | 23-character Iraqi IBAN checksum and bank and branch decode (C-44) |
| `mask_pan` | none | First six and last four only; required before landing for any card number (B.5) |
| `keyed_hash` | key purpose | HMAC with an OpenBao-wrapped key for match keys |
| `coalesce`, `concat` | field list | Combine source values |

Adding a transform is a product release with property tests and a conformance-suite run, never a bank
configuration. Each transform has a version; a mapping records the library version it was tested with.

## B.5 Validation rules and landing guards

| Rule | Arguments | Notes |
|---|---|---|
| `not_null` | field | |
| `ref` | field, table | Referential check against a canonical or reference table |
| `range` | field, min, max | |
| `enum` | field, values | |
| `iban_checksum` | field | |
| `not_placeholder_date` | field | Refuses sentinel dates such as 1900-01-01 or 9999-12-31 |
| `no_mojibake` | field | Detects broken Arabic encodings (C-10, C-28) |
| `unique_key` | fields | Within the batch |

`on_fail`: `dq_issue` (default: the row loads and an `admin.dq_issues` count rises) or `quarantine` (the row goes to
`admin.quarantine` with the reason and is re-tried after a fix; the batch still completes). Key fields default to
`quarantine`.

**Card guard** (C-36): before landing, every text field of a batch is scanned for Luhn-valid card numbers. A field
that carries one without `mask_pan` stops that field's load and opens a data incident; a mapping that maps a card
number without `mask_pan` is refused at validation (`E_PAN_UNMASKED`).

## B.6 Survivorship (multi-source banks)

```json
"survivorship": { "party.residence_country": { "rule": "source_priority", "order": ["ics_core", "islamic_core"] },
                  "party_contact.value": { "rule": "most_recent" } }
```

Rules: `most_recent`, `source_priority`, `longest_verified`. The chosen value records which rule and which source won,
so it is explainable; party merge and split across sources are maker-checker actions and always reversible (C-53).

## B.7 Code maps

Code maps are separate admin data (`admin.code_maps`, `admin.code_map_entries`): effective-dated rows `source_code ->
canonical_code`, each added or changed by change request. Kinds: `txn_type`, `product`, `channel`, `country`,
`id_type`, `branch`, `purpose`, `currency`, `status`.

- An unmapped value becomes `unknown` plus a data-quality issue, never a failed batch (C-50).
- For kinds marked `gate: quarantine` (default for `txn_type`), a row with an unmapped code is quarantined until the
  code is mapped and is then replayed, so it is never monitored as "other" (MODULES M6 edge case).
- A reclassified code takes effect forward from its `valid_from`; history keeps the old classification.

## B.8 Generated SQL, drift and the read-only contract

- The SQL generator builds only `SELECT` statements from the mapping, per dialect plugin (Oracle, SQL Server,
  PostgreSQL, generic ODBC for Db2 and Sybase, C-61), with keyset pagination on the key and watermark, bind
  variables and fetch sizes from the signed load budget (C-13).
- Before any statement is sent, a SQL parser verifies it is a single read-only `SELECT` over approved objects
  (C-54). A failure is a product defect: the statement is refused and an incident opened.
- **Drift** (C-52): each run compares the catalogue hash of the columns used with `approved_catalogue_hash`. On a
  difference only the affected table pauses (bulkhead), a `drift_events` row and a DS task open, and the other tables
  keep running. Dictionary-driven packs propose a regenerated mapping for approval.
- No mapping can make AKR-TMS query the CBS by customer, account or case (C-35); the generator has no per-entity
  filter.

## B.9 Lifecycle

Statuses: `draft` -> `testing` -> `approved` -> `published` -> `retired` (DATA-MODEL `admin.mapping_versions`).

1. **Draft** in the workbench; every edit is audited with actor and reason.
2. **Test** against a read-only sample of real source rows (inside the load budget) and the pack's golden data;
   canonical output shown beside the source; automatic checks for types, ranges, nulls, references and row counts.
3. **Impact analysis**: rows whose canonical value changes against the published version, scenarios that read the
   changed fields, and the change in the coverage matrix.
4. **Approve** by change request (`cr_mapping`): the implementer proposes, a bank compliance or IT officer approves.
5. **Publish and pin**: the version applies from the next batch boundary; every batch records the mapping version it
   ran under, so history is never re-interpreted silently.
6. **Replay** (optional, explicit) over the raw landing layer for a date range, with a diff reviewed before it is
   applied (C-24).
7. **Rollback**: activate the previous version; replay if needed.

---

# Part C — The rating model document

The rating model is the declarative configuration of the customer risk rating engine (research 26 s.14; K-01). It
holds the levels, factors, blocks, weights, floors, triggers, overrides policy and controls matrix. The engine is
product code; the model is governed data with one live version per bank.

## C.1 Document structure

```json
{
  "dsl": "rating_model", "dsl_version": "1",
  "id": "RM-IQ", "version": "1.3.0",
  "name": { "ar": "...", "en": "..." },
  "owner": "role:M",
  "regulatory_profile": "iraq_cbi",
  "levels": [ { "level": 1, "from": 0, "to": 15, "cbi": "low" }, { "level": 2, "from": 15, "to": 35, "cbi": "low" },
              { "level": 3, "from": 35, "to": 55, "cbi": "medium" }, { "level": 4, "from": 55, "to": 75, "cbi": "high" },
              { "level": 5, "from": 75, "to": 100, "cbi": "high" } ],
  "blocks": [ { "id": "A", "name": { "ar": "...", "en": "Customer and structure" }, "weight": 25 } ],
  "factors": [ { "id": "A1", "...": "..." } ],
  "unknown_score": 60,
  "peak_term": 0.35,
  "floors": [ { "id": "FL-01", "...": "..." } ],
  "completeness_cap": { "...": "..." },
  "triggers": [ { "id": "TR-15", "...": "..." } ],
  "stickiness": { "rise": "immediate", "fall": "at_review", "cross_floor": false },
  "overrides": { "...": "..." },
  "controls": [ { "level": 1, "...": "..." } ],
  "sanity_bands": { "high_share": { "min": 0.01, "max": 0.20 } },
  "changelog": { "ar": "...", "en": "..." }
}
```

Bands are half-open (`from` inclusive, `to` exclusive; the top band includes 100).

| Field | Notes |
|---|---|
| `regulatory_profile` | `iraq_cbi` (default) or `eu_amlr`; selects review-clock limits and validation rules (K-09, K-37) |
| `levels` | Five levels covering 0-100 without gap or overlap, each mapped to the CBI's three labels (K-02) |
| `blocks` | Six blocks A-F (customer and structure, business and wealth, geography, products, channel, behaviour); weights sum to 100 |
| `factors` | C.2 |
| `unknown_score` | Score of an applicable factor whose value is unknown or stale beyond its freshness limit. Default 60 (K-04) |
| `peak_term` | p in the inherent score `(1 - p) x weighted blocks + p x highest block`; 0 to below 0.5 so no single block dominates (file 26 s.14b) |
| `floors` | C.3 |
| `completeness_cap` | C.4 |
| `triggers` | C.5 |
| `stickiness` | Rises apply at once; falls wait for the next review or an approved downward override and never cross a floor (K-30) |
| `overrides` | C.6 |
| `controls` | C.7 |
| `sanity_bands` | Distribution monitor: a High-plus share outside the band opens a review. A band is never a quota (K-29) |

## C.2 Factors

```json
{ "id": "A1", "block": "A", "weight": 30,
  "name": { "ar": "...", "en": "Customer type" },
  "applies_to": ["person", "entity"],
  "mandatory_for": ["entity"],
  "input": "party.legal_form",
  "anchors": { "kind": "categorical",
               "map": { "employee": 10, "pensioner": 10, "student": 10, "self_employed": 30, "sole_proprietor": 35,
                        "llc": 45, "jsc_unlisted": 40, "jsc_listed": 15, "government_body": 5, "state_enterprise": 35,
                        "waqf_supervised": 30, "npo_registered": 55, "association_unregistered": 70,
                        "foreign_branch": 60, "dnfbp": 75, "exchange_or_epayment": 85 } },
  "freshness": { "value": 36, "unit": "months" },
  "basis": "EBA guidelines; AMLR Annex III; MER para 695; CBI controls 2025" }
```

| Field | Notes |
|---|---|
| `block`, `weight` | Weight inside the block; the factors of a block sum to 100 |
| `applies_to` | `person`, `entity`, `arrangement`. A factor that does not apply is left out of the block average |
| `mandatory_for` | Party types for which the factor is mandatory (feeds the completeness cap) |
| `input` | A CanonicalRef, feature or derived fact (`pep.tier`, `ownership.layers`, `country_table.max_level`) |
| `anchors.kind` | `categorical` (value to score map), `bands` (numeric thresholds: `[{ "below": 1000000, "score": 10 }, ...]`), `country_table` (country-risk level to score: low 10, medium 40, high 70, very high 100), `expr` (Expr returning 0-100) |
| `freshness` | Older than this, the value counts as unknown and a refresh trigger fires (TR-23) |
| `basis` | Source text shown in the explanation panel (K-03) |

The shipped model has 27 factors in six blocks (file 26 s.14d). Factors may not use protected attributes: names,
tribe, religion, sect, region of origin and gender are refused (`E_PROTECTED_ATTRIBUTE`, K-33). Nationality and
residence are allowed because the CBI controls name them as customer factors. No commercial data (revenue, product
profitability) is a rating input (K-17).

## C.3 Floors

```json
{ "id": "FL-03", "when": "pep.tier == 'P3' || (pep.former && !has(pep.assessment_documented))", "level": 4,
  "basis": "CBI PEP circular 26 Aug 2026 items 1, 8; AMLR art. 45-46",
  "decay": null, "exception": null }
```

| Field | Notes |
|---|---|
| `id` | FL-01 to FL-15 for the shipped floors; bank floors use FL-9xx (901 onwards) |
| `when` | Expr over the party's facts |
| `level` | Minimum level, 1-5 |
| `decay` | Duration after which the floor stops applying (FL-08 24 months, FL-09 36 months, FL-07 12 to 36 months by severity). Expiry creates a review, never an automatic downgrade (K-30) |
| `exception` | Optional governed exception (for example FL-05 allows a documented sub-segment such as a registered domestic NPO with local donors to drop to 3 with EDD and senior approval kept); the exception is itself a change request |
| `action` | Optional process action (`block_onboarding` for FL-11, `remediation_task` for FL-12) |

Floors cannot be overridden per customer; the schema has no field that would allow it (K-07). A sanctions near-match
is a hold and freeze workflow (M9, M14), not a floor.

## C.4 Completeness cap

```json
"completeness_cap": {
  "mandatory": { "person": ["B1", "C1", "A4"], "entity": ["A1", "A3", "B1", "A6"] },
  "one_or_more_missing": { "min_level": 3 },
  "two_or_more_missing": { "action": "escalate_review", "block_rating": true }
}
```

A customer with any mandatory factor unknown cannot be rated below Level 3; with two or more, the review escalates
and no rating is produced until the data is supplied (shown as a control failure, never as Low; K-04).

## C.5 Triggers

```json
{ "id": "TR-15", "on": "case.decision", "when": "decision.type in ['str', 'tfr', 'escalate']",
  "action": "rescore_and_review", "review_within": { "value": 5, "unit": "business_days" },
  "owner": "role:KYC", "calibrated_on": "2026-09-30" }
```

| Field | Notes |
|---|---|
| `on` | `party.changed`, `relationship.changed`, `screening.hit`, `pep.changed`, `media.recorded`, `country_table.changed`, `profile.breach`, `txn.new_corridor`, `account.reactivated`, `case.decision`, `authority_request.recorded`, `internal_report.recorded`, `visit.recorded`, `outreach.failed`, `graph.link`, `dq.stale`, `account.opened`, `staff_link.recorded`, `islamic.signal` |
| `when` | Expr over the event and the party's facts |
| `action` | `rescore`, `rescore_and_review`, `refresh_task`, `pep_workflow`, `hold_and_escalate`, `block_and_escalate`, `reclassify` |
| `review_within` | SLA for the resulting review or task, on the bank calendar |
| `owner` | RoleRef |

The shipped library is TR-01 to TR-26 (file 26 s.6c; K-11). A country-table change (TR-10) re-scores every affected
customer the same day and lists them with the effective date (K-42). Floors FL-07 to FL-11 apply from the moment the
fact is recorded.

## C.6 Overrides policy

```json
"overrides": {
  "up":   { "roles": ["role:KYC", "role:M", "role:A2"], "reason_required": true },
  "down": { "roles": ["role:M", "role:KYC"], "max_levels": 1, "max_expiry": { "value": 12, "unit": "months" },
            "maker_checker": true, "exclude_relationship_owner": true, "evidence_required": true },
  "never_below": ["floor", "completeness_cap"]
}
```

Upward overrides are easy; downward overrides are compliance-only, one level per decision, expire within 12 months,
need a reason code, evidence and a second approver, and never cross a floor or the completeness cap (K-24). The
relationship owner can neither propose nor approve a downward override of their own customer (K-17).

## C.7 Controls matrix

```json
{ "level": 4, "cdd": "edd", "approver": "position:senior_management_and_mlro",
  "monitoring_profile": "enhanced",
  "classification_review": { "value": 12, "unit": "months" },
  "kyc_refresh": { "value": 12, "unit": "months" },
  "light_check": { "value": 3, "unit": "months" },
  "edd_playbook": "edd_entity_v1" }
```

One row per level (file 26 s.14g; K-08). `approver` is a position key from the bank's authority matrix
(`position:branch_officer`, `position:senior_management_and_mlro`), not a role code, because these approvers sit in
the business lines. Under `iraq_cbi`, no classification review interval may exceed 24 months,
the CBI's written maximum (`E_REVIEW_INTERVAL_LIMIT`). The monitoring profile names the scenario parameter overrides
that apply (`risk:<level>` in A.5), so the rating and the scenarios stay consistent.

## C.8 Computation contract

For each party, nightly and immediately on a trigger (K-01, K-26):

1. Extract each applicable factor's value; a value older than its `freshness` counts as unknown.
2. Score each factor from its anchors; an unknown applicable factor scores `unknown_score`.
3. Block score = weighted average of the applicable factors.
4. Inherent score = `(1 - p) x sum(block weight x block score) + p x max(block score)`.
5. Score level from `levels`.
6. Level = max(score level, highest floor that fires).
7. Apply the completeness cap.
8. Apply a valid override (never below a floor or the cap); apply stickiness against the previous rating.
9. Map to the CBI label; attach the controls row.
10. Store the rating (`kyc.ratings`: level, CBI label, score, drivers with refresh dates, missing share, degraded
    flag, trigger code, model version) and, if the level changed, open the review task by the clock and write the
    one-way feed row for the bank (K-35).

The computation is deterministic for a given model version, configuration and data snapshot, so any past rating can
be reconstructed (K-26). The explanation panel (K-27) is rendered from the stored drivers.

## C.9 Lifecycle

Statuses: `draft` -> `shadow` -> `live` -> `retired`. Any change to levels, weights, anchors, floors, triggers,
completeness or overrides policy is a new model version and a **material** change. A shadow run on production data
with disagreement lists, tier migration and sensitivity results is required evidence before approval (K-06, K-28).
Country-risk tables and PEP data are reference data with their own effective dates, not model versions.

---

# Part D — Change requests

One mechanism governs every admin-managed object that affects detection, reporting or access: scenarios, parameter
sets, mappings, code maps, the rating model, policy packs, fail policies, list settings, goAML profiles, role grants,
settings, models and workflow definitions (research 32 s.11.4; file 29 s.9f; D20). A change request is a row
(`admin.change_requests`), not a document, but its body has a fixed shape and its own validation.

## D.1 Request body

```json
{
  "object_kind": "parameter_set",
  "object_id": "0192f3a4-...",
  "base_version": 4,
  "proposed": { "kind": "parameter_set", "draft_id": "0192f3b0-..." },
  "reason_code": "TUNE_RECALIBRATION_PCT",
  "reason_text": "Monthly P95 recomputed; cash-retailer segment above 200 customers since September.",
  "evidence": [ { "kind": "replay", "id": "0192f3c1-..." }, { "kind": "btl_sample", "id": "0192f3c7-..." } ],
  "effective_at": null,
  "emergency": false
}
```

| Field | Notes |
|---|---|
| `object_kind` | `scenario`, `parameter_set`, `mapping`, `code_map`, `rating_model`, `policy_pack`, `fail_policy`, `list_setting`, `goaml_profile`, `role_grant`, `setting`, `model`, `workflow_definition` |
| `base_version` | The object version the change was made against. If the object moves before activation, the request goes stale and must be rebased (`E_STALE_BASE`) |
| `proposed` | A reference to the draft version (scenario version, parameter set, mapping version, model version), or an inline patch for small objects (setting, role grant) |
| `reason_code` | From `ref.reason_codes` family `tuning` or `change`. Every code is tagged `risk` or `capacity`; a `capacity` reason needs below-the-line evidence and an approver outside operations (S-39) |
| `reason_text` | Free text in the maker's language |
| `evidence` | References to replay, below/above-the-line samples, shadow comparisons, validation records, test runs, impact analyses. Each item must be younger than `settings.cr_evidence_max_age_days` when the checker approves (`E_EVIDENCE_STALE`) |
| `effective_at` | Optional future time; null means "on approval". Material changes cannot be back-dated |
| `emergency` | D.3 |

## D.2 Computed fields (set by the system, never by the maker)

| Field | How it is computed |
|---|---|
| `change_class` | `minor`, `material` or `emergency` from the rules in D.3. The maker may raise the class, never lower it (`E_CLASS_LOWERED`) |
| `volume_effect` | `increase`, `neutral`, `decrease` or `unknown`, from the attached replay (alerts before and after over the same period). A scenario or parameter change with no replay is `unknown` and cannot be approved as material |
| `impact` | Alert volume and coverage before and after, scenarios and coverage cells touched (mappings), tier migration (rating model) |
| `route` | The workflow definition that will carry the approval (Part E), chosen from `object_kind`, `change_class` and `volume_effect` |

## D.3 Change classes

| Change | Class | Evidence required |
|---|---|---|
| Parameter value inside the approved band, not loosening | Minor | Replay |
| Parameter value outside the band, or any loosening (`volume_effect: decrease`) | Material | Replay; below-the-line sample; for a `capacity` reason an approver outside operations |
| Scenario logic: patch | Minor | Unit tests |
| Scenario logic: minor or major | Material | Unit tests, replay, below/above-the-line, shadow comparison (S-47) |
| New bank scenario, or enabling an `akr` extension scenario as part of the bank's set | Material | As above, plus the Office-sharing card export (S-05) |
| Mapping version | Material | Test run, impact analysis |
| Code map: mapping a previously unmapped code | Minor | Sample rows |
| Code map: reclassifying a mapped code | Material | Impact analysis |
| Rating model version | Material | Shadow run, disagreement list, tier migration (K-06) |
| Segment definition | Material | Alert-volume impact by segment |
| goAML profile, policy pack, fail policy | Material | Validation run (goAML checker), policy test cases |
| Role grant, setting, list setting | Minor (raised to material when the setting carries a CBI floor) | None beyond the diff |
| Sanctions-list driven or regulator-ordered change | Emergency | The order or list reference |

**Emergency**: applied on the approval of one named officer (MLRO or deputy) who is not the maker, then reviewed on
the material route within 5 working days (file 27 B26.2). If the review rejects, a rollback task opens. The maker
still never approves.

## D.4 Invariants

- **Maker is never checker**: enforced in Cedar and by the database check `checker_id <> maker_id`
  (DATA-MODEL `admin.change_requests`; GV-53).
- **Fresh step-up**: the approving action needs AAL2 from the single-use step-up ledger.
- **Atomic activation**: at `effective_at` a job writes the new object version's status, the audit event and the
  outbox event in one transaction (the DBS policy-bundle activation pattern). `If-Match` on the object version is
  checked at submit and again at activation.
- **Versions on outputs**: every later alert, rating or batch records the versions it ran under (S-52).
- **Post-change review**: timers open review tasks for the approver at 30 and 90 days after a material change
  (file 29 s.9f; S-46).
- **Volume-reducing changes** are flagged on the request and routed to a second approver in the compliance function
  (research 32 s.11.4).

Statuses: `draft` -> `submitted` -> `approved` | `rejected` | `withdrawn` -> `effective` (DATA-MODEL).

---

# Part E — Approval workflows (the DBS workflow DSL, reused)

AKR-TMS adopts the DBS workflow DSL v1 (DBS `docs/DSL.md`, ported in P0 with its engine, docs/plan phase 04) for approvals and
routed tasks that are not case state (ARCHITECTURE D8). Investigation state lives in the case aggregate (M12); the
workflow engine carries the decisions around it.

## E.1 What is reused, what is not

| DBS element | AKR use |
|---|---|
| Document structure, `I18nString`, `Duration`, timers, versioning and pinning, validation response shape | Unchanged |
| Step types `form`, `approve`, `sign`, `review`, `parallel`, `terminal` | Used |
| Step types `reply`, `attach`, `acknowledge`, `execute` | Not used in P1. Requests for information use the case RFI model (M12); actions on the CBS are operations tasks (M14) |
| Assignee kinds `position`, `org_unit`, `relative` | Replaced by AKR kinds (E.2) |
| `auto_approve`, `auto_reject` timer action | Forbidden on any AKR definition (`E_AUTO_DECISION`): people decide (D7) |
| Terminal side effects (DBS letters, registers, staff transfers) | Replaced by the AKR set (E.3) |
| Layout block, canvas builder | Reused for the policy studio (D3) |

## E.2 AKR assignee kinds and decision rules

```json
{ "kind": "role", "role": "CHK", "scope": "bank" }
{ "kind": "pool", "pool": "pool:central_l2" }
{ "kind": "named_officer", "officer": "mlro" }
{ "kind": "case_acl", "grant": "decide" }
```

- `role` resolves through `admin.role_grants` (with delegations); `case_acl` only to principals on the case's access
  list with the `decide` grant, so a suspicion-tier approval never reaches someone outside the case.
- Every `approve` and `sign` step excludes the maker of the change, the proposer of the decision and earlier deciders
  of the same instance. The exclusion cannot be switched off (`E_FOUR_EYES_OFF`).
- Deciding steps require step-up (AAL2); `sign` steps always do (DBS `E_STEPUP_DISABLED`).
- Definitions for suspicion-tier work carry `classification_default: "restricted"`: the instance and its tasks are
  visible only to the case access list, and task titles shown in shared inboxes use neutral wording. Human-visible
  strings in definitions that branch-tier roles can see are checked against the admin-managed tipping-off term list
  (`E_TIPPING_OFF_TERM`).
- Timers use the bank calendar; decision steps carry a timeout with `notify_superior` or `reassign`, never an
  automatic outcome.

## E.3 AKR terminal effects

| Effect | What it does |
|---|---|
| `activate_change` | Activates the approved change request (Part D, atomic activation) |
| `record_decision` | Writes the case decision (`casework.decisions`, with `decider_id <> proposer_id`) and its case event |
| `release_for_filing` | Marks an STR, TFR or other report package ready for the named officer to file; filing itself is a person's act (D9) |
| `create_ops_task` | Creates an operations task for bank staff with neutral wording (M14); never a CBS write |
| `open_review` | Opens a follow-up review task (emergency changes, post-change reviews) |
| `notify` | In-app notification only; no external channel for suspicion-tier content |

## E.4 Shipped definitions

Seeded as data at install; a bank changes them through a `workflow_definition` change request approved by two
administrators. The bank's authority matrix (`admin.authority_matrix`) supplies who may sit at each step.

| Key | Used for | Steps (summary) |
|---|---|---|
| `cr_minor` | Minor change requests | `approve` (CHK, not the maker) -> `activate_change` |
| `cr_material` | Material change requests | `parallel` [`approve` MLRO, `approve` second approver outside operations (CHK)] -> if `volume_effect = decrease`: `approve` compliance head (CH) -> `activate_change` |
| `cr_material_validated` | Material changes to CBI-baseline scenario logic and to the rating model | `approve` independent validator (MRO, or an outside firm entered as a named user) -> then as `cr_material`. An `approve`, not a `review`: a DBS `review` step does not stop the request |
| `cr_emergency` | Emergency changes | `approve` (MLRO or deputy) -> `activate_change` -> `open_review` on the material route within 5 working days |
| `cr_mapping` | Mapping and code-map changes | `approve` (bank compliance or IT officer, not of the implementer's organisation) -> `activate_change` |
| `cr_access` | Role grants and delegations | `approve` (ADM) -> `approve` (SEC) -> `activate_change` |
| `case_close_no_report` | Closing an alert or case without a report | `approve` (A2 or M from `case_acl`, not the proposer) -> `record_decision` |
| `case_report_approval` | STR, SAR, TFR, UTR decisions | `sign` (MLRO from `case_acl`, step-up) -> `record_decision` -> `release_for_filing` |
| `kyc_override_down` | Downward rating override | `approve` (compliance, not the relationship owner) -> `record_decision` |
| `kyc_edd_signoff` | Level 4 and 5 onboarding and continuation | `approve` per controls matrix approver (C.7) -> `record_decision` |
| `exit_decision` | Exit candidates | `approve` per authority matrix -> `record_decision` -> `create_ops_task` |

Each instance pins its definition version; publishing a new version never moves in-flight instances.

---

## 6. Versioning and lifecycle summary

| Kind | Version format | Statuses | Pinned where | Rollback |
|---|---|---|---|---|
| Scenario logic | semver | draft, testing, shadow, live, retired | `alert.alerts.scenario_version_id` | Kill switch to the previous approved version |
| Parameter set | int | (follows the change request) | `alert.alerts.parameter_set_id` | Previous parameter set by change request |
| Mapping | int | draft, testing, approved, published, retired | Batch record and `alert.alerts.mapping_versions` | Activate the previous version, replay if needed |
| Code map entry | effective-dated rows | (follows the change request) | Canonical row lineage | A new entry effective forward |
| Rating model | semver | draft, shadow, live, retired | `kyc.ratings.model_version_id` | Activate the previous version by change request |
| Workflow definition | int | draft, published, archived | Instance | Publish the previous content as a new version |
| Change request | none | draft, submitted, approved, rejected, withdrawn, effective | Each activated version names its change request | Rollback is a new change request |

## 7. Validation errors

Response shape (RFC 7807 extension, the same as the DBS):

```json
{
  "type": "https://akr-tms.local/errors/dsl-validation",
  "status": 422,
  "errors": [
    { "path": "/parameters/total/overrides/0/value", "code": "E_PARAM_OUT_OF_BOUNDS",
      "message": "value 2500 USD is below min 3000 USD", "severity": "error" },
    { "path": "/logic/measures/1/field", "code": "E_FIELD_TYPE",
      "message": "sum needs a numeric field; txn.narrative_raw is text", "severity": "error" }
  ]
}
```

Every message must be actionable from the path and the code alone. Drafts may be saved with errors and warnings;
moving past `draft` (scenario to `testing`, mapping to `testing`, rating model to `shadow`) and submitting a change
request require zero errors. Warnings are listed on the change request and need `"accept_warnings": true` through the
API.

**Common**

| Code | Severity | Meaning |
|---|---|---|
| `E_SCHEMA` | error | JSON Schema violation (path points at the offending key) |
| `E_UNKNOWN_DSL_VERSION` | error | `dsl_version` not supported |
| `E_MISSING_I18N` | error | `ar` or `en` missing in a human-visible string (on leaving draft) |
| `E_I18N_PLACEHOLDERS` | error | The Arabic and English texts use different placeholder sets |
| `E_DANGLING_REF` | error | Reference to an unknown reason code, segment, calendar, code map, template, feature, role or pool |
| `E_UNKNOWN_ATTRIBUTE` | error | A CanonicalRef not in the canonical catalogue or a feature key not defined |
| `E_EXPR_COMPILE` / `E_EXPR_TYPE` / `E_EXPR_COST` | error | Expr does not compile, returns the wrong type, or exceeds the cost bound |
| `E_MONEY_NO_CURRENCY` | error | A money value without a currency |
| `E_PROTECTED_ATTRIBUTE` | error | A protected attribute used in a filter, grouping, segment, override condition or rating factor (A.5, C.2) |
| `W_USER_REF` | warning | A named user where a role or pool is expected |

**Scenario (Part A)**

| Code | Severity | Meaning |
|---|---|---|
| `E_NAMESPACE` | error | Id prefix and family differ, or a `bank` scenario outside 901-999 |
| `E_CBI_REF_UNKNOWN` | error | A CbiRef that is not in the CBI catalogue register |
| `E_CBI_REFS_REQUIRED` | error | A `pack: cbi` scenario without CBI references |
| `E_UNDECLARED_INPUT` | error | Logic, exclusions or outputs read an attribute missing from `data_requirements` or the event `context` |
| `E_UNKNOWN_PARAM` | error | A ParamRef to an undeclared parameter |
| `E_PARAM_OUT_OF_BOUNDS` | error | A default, override or parameter-set value outside `min`/`max`/`allowed` |
| `E_DEFAULT_OUTSIDE_BAND` | error | The shipped default is outside the approved band |
| `E_DP_NO_REVIEW` | error | A `DP` basis without `review_by` (S-40) |
| `E_ALERT_CAP` | error | A parameter or field that limits the number of alerts (S-39) |
| `E_FILTER_NOT_COMPILABLE` | error | A `where` or `having` outside the compilable Filter subset |
| `E_FIELD_TYPE` | error | `sum`, `avg` or `max` over a non-numeric field; `ratio` naming unknown aliases |
| `E_WINDOW_TOO_LONG` | error | A window longer than the hot window of the canonical store |
| `E_COST_GUARD` | error | Compiled statement over the cost guard for the bank's data (at compile or replay) |
| `E_NO_REASON_CODE` | error | No reason code in `outputs` |
| `E_HIBERNATION_SKIPS_EVALUATION` | error | A suppression rule that stops evaluation instead of display |
| `E_TESTS_MISSING` | error | Fewer than 3 positive and 3 negative unit cases when leaving `testing` |
| `E_TWO_LIVE` | error | A second `live` version for one scenario |
| `E_BASELINE_RETIRE` | error | Retiring, disabling or deleting a `pack: cbi` scenario |
| `W_OUTSIDE_BAND` | warning | A parameter-set value inside the hard bounds but outside the approved band; the change request becomes material |
| `W_DEGRADED_INPUT` | warning | An optional input whose readiness grade is C or D at this bank |
| `W_DP_EXPIRED` | warning | A `DP` value past its review date |
| `W_LOOSENING` | warning | A parameter-set change that lowers alert volume in the attached replay; the change request is flagged |

**Mapping (Part B)**

| Code | Severity | Meaning |
|---|---|---|
| `E_TARGET_UNKNOWN` | error | `entity` not in the canonical model |
| `E_OBJECT_NOT_APPROVED` | error | `source.object` not on the source's approved object list |
| `E_UNKNOWN_SOURCE_COLUMN` | error | `from` names a column or dictionary field not in the source catalogue |
| `E_UNKNOWN_TRANSFORM` / `E_TRANSFORM_ARGS` | error | A transform not in the library, or wrong arguments |
| `E_REQUIRED_TARGET_UNMAPPED` | error | A required canonical attribute has no mapping |
| `E_WATERMARK_KIND` | error | No watermark for a fact table, or a kind the source engine cannot provide |
| `E_PAN_UNMASKED` | error | A card number mapped without `mask_pan` |
| `E_DEFAULT_ON_MONITORED` | error | A `default` on an attribute that feeds a scenario threshold or a rating factor |
| `E_FILTER_NO_REASON` | error | A filter without a reason code |
| `E_CODE_MAP_UNKNOWN` | error | `code_map:<key>` names no code map for this source |
| `W_DRIFT_SINCE_TEST` | warning | The source catalogue changed after the last test run |
| `W_SAMPLE_SMALL` | warning | The test sample is below the configured minimum row count |

**Rating model (Part C)**

| Code | Severity | Meaning |
|---|---|---|
| `E_WEIGHTS_SUM` | error | Block weights, or factor weights within a block, do not sum to 100 |
| `E_PEAK_TERM_RANGE` | error | `peak_term` outside 0 to below 0.5 |
| `E_LEVEL_BANDS` | error | Level bands with a gap, an overlap, or not covering 0-100 |
| `E_CBI_MAPPING` | error | CBI labels not monotonic with levels |
| `E_UNKNOWN_IS_LOW` | error | `unknown_score` below the lowest score of the first level mapped to CBI Medium (35 with the shipped bands) |
| `E_FLOOR_LEVEL` | error | A floor level outside 1-5 |
| `E_DECAY_MISSING` | error | FL-07, FL-08 or FL-09 (or a bank floor based on a dated fact) without `decay` |
| `E_TRIGGER_INCOMPLETE` | error | A trigger without `action`, `owner` or `review_within` |
| `E_CONTROLS_MISSING` | error | No controls row for a level |
| `E_REVIEW_INTERVAL_LIMIT` | error | A classification review interval above the regulatory profile's maximum (24 months under `iraq_cbi`) |
| `E_OVERRIDE_LIMITS` | error | Downward overrides allowing more than one level, expiry above 12 months, or no maker-checker |
| `W_SHADOW_NOT_RUN` | warning | No shadow run attached for this version yet |
| `W_SANITY_BAND` | warning | The shadow distribution falls outside `sanity_bands` |

**Change request (Part D)**

| Code | Severity | Meaning |
|---|---|---|
| `E_CLASS_LOWERED` | error | The maker set a lower class than the computed one |
| `E_EVIDENCE_MISSING` | error | Evidence required for the class (D.3) is not attached |
| `E_EVIDENCE_STALE` | error | Evidence older than `settings.cr_evidence_max_age_days` at approval |
| `E_CAPACITY_NO_BTL` | error | A `capacity` reason without a below-the-line sample |
| `E_STALE_BASE` | error | The object moved past `base_version`; rebase the draft |
| `E_BACKDATED` | error | `effective_at` in the past for a material change |
| `E_SELF_APPROVAL` | error | The maker tried to approve (also refused by Cedar with 403 and by the database check) |

**Workflow definitions (Part E)** — the DBS catalogue applies; AKR adds:

| Code | Severity | Meaning |
|---|---|---|
| `E_AUTO_DECISION` | error | `auto_approve` or an `auto_reject` timer action on an AKR definition |
| `E_FOUR_EYES_OFF` | error | A deciding step that does not exclude the maker, the proposer or earlier deciders |
| `E_TIPPING_OFF_TERM` | error | A string visible to branch-tier roles contains a term from the tipping-off list |
| `E_ACL_ASSIGNEE_REQUIRED` | error | A `restricted` definition with a deciding step not assigned through `case_acl` |

## 8. Authoring API (summary; the full catalogue is in API.md)

| Call | Purpose |
|---|---|
| `GET /api/v1/dsl/{kind}/schema` | The JSON Schema for `scenario`, `mapping`, `rating_model`, `change_request` |
| `POST /api/v1/scenarios`, `POST /api/v1/scenarios/{id}/versions` | Create a bank scenario; open a new draft version |
| `PUT /api/v1/scenarios/{id}/versions/{v}` | Replace the draft content |
| `POST /api/v1/scenarios/{id}/versions/{v}/validate` | Schema and semantic validation; returns the error list without changing state |
| `POST /api/v1/scenarios/{id}/versions/{v}/test` | Run the unit cases and seeded cases |
| `POST /api/v1/scenarios/{id}/replays` | Replay a version with a parameter set over an as-of period |
| `POST /api/v1/scenarios/{id}/shadow` | Start a shadow run of a version |
| `GET /api/v1/scenarios/{id}/card` | The generated scenario card |
| `POST /api/v1/mappings/{id}/versions`, `POST .../validate`, `POST .../preview` | Mapping drafts, validation, canonical preview on sample rows |
| `POST /api/v1/rating-models/versions`, `POST .../validate`, `POST .../shadow` | Rating model drafts, validation, shadow run |
| `POST /api/v1/change-requests`, `POST /api/v1/change-requests/{id}/submit` | Create and submit a change request (class, volume effect and route computed) |
| `POST /api/v1/change-requests/{id}/approve`, `.../reject`, `.../withdraw` | Decisions (step-up, never the maker) |

Every mutation carries `Idempotency-Key` and `If-Match` and writes an audit event (README non-negotiable 9).

## 9. Complete examples

### 9.1 Aggregate — SCN-CASH-02 structuring (matches file 27 B26.4)

```json
{
  "dsl": "scenario", "dsl_version": "1",
  "id": "SCN-CASH-02", "version": "1.0.0",
  "name": { "ar": "إيداعات نقدية متكررة دون حد الإبلاغ",
            "en": "Structuring: repeated cash deposits just under the limit" },
  "family": "CASH", "pack": "akr",
  "typology": ["cash_placement_structuring"],
  "cbi_refs": [ { "article": 1, "no": 2, "rel": "implements" }, { "article": 1, "no": 12, "rel": "implements" },
                { "article": 1, "no": 4, "rel": "extends" } ],
  "owner": "role:M", "classification": "confidential", "evidence_label": "P",
  "phase2_candidate": false, "latency_class": "L1",
  "data_requirements": ["txn.cash_flag", "txn.dr_cr", "txn.amount_usd", "txn.booking_date", "txn.branch_id",
                        "cash_event.depositor_id?", "party_relationship.type?", "feat.limit_enquiry_logged_7d?"],
  "aggregation": { "level": "customer", "linked_parties": ["family_no", "depositor"], "net_reversals": true,
                   "posted_only": true, "settle_delay": { "value": 2, "unit": "minutes" }, "currency_mode": "separate" },
  "parameters": {
    "min_deposits": { "type": "int", "default": 3, "min": 2, "max": 10, "band": [3, 5], "basis": "DP", "review_by": "2027-04-30" },
    "window": { "type": "duration", "default": { "value": 7, "unit": "days" },
                "allowed": [ { "value": 7, "unit": "days" }, { "value": 14, "unit": "days" }, { "value": 30, "unit": "days" } ],
                "basis": "DP", "review_by": "2027-04-30" },
    "limit": { "type": "decimal", "default": 10000, "min": 10000, "max": 10000, "basis": "CBI",
               "description": { "ar": "حد الإبلاغ بالدولار", "en": "Reporting limit in USD" } },
    "total": { "type": "decimal", "default": 10000, "min": 3000, "max": 50000, "band": [8000, 12000], "basis": "CBI",
               "overrides": [ { "when": ["segment:cash_retailer"], "value": 15000, "basis": "PCT" },
                              { "when": ["risk:4"], "value": 7000, "basis": "BANK" },
                              { "when": ["risk:5"], "value": 5000, "basis": "BANK" } ] },
    "just_below": { "type": "range", "default": [0.70, 0.999], "min": 0.5, "max": 0.999, "basis": "DP", "review_by": "2027-04-30" }
  },
  "logic": {
    "kind": "aggregate",
    "where": "txn.cash_flag = true AND txn.dr_cr = 'C' AND within_band(txn.amount_usd, :just_below * :limit, :limit)",
    "window": { "kind": "rolling", "length": ":window", "calendar": "default" },
    "group_by": "aggregation.level",
    "measures": [ { "op": "count", "as": "n" },
                  { "op": "sum", "field": "txn.amount_usd", "as": "total" },
                  { "op": "distinct_count", "field": "txn.branch_id", "as": "branches" } ],
    "having": "n >= :min_deposits AND total >= :total"
  },
  "exclusions": { "register": true },
  "suppression": { "hibernation": { "allowed": true, "default_period": { "value": 90, "unit": "days" },
                                    "envelope": ["n", "total"], "margin": 0.2 },
                   "dedupe": { "same_entity_within": { "value": 7, "unit": "days" } } },
  "outputs": {
    "severity": { "base": "medium",
                  "uplift": [ { "when": "branches >= 2", "to": "high", "reason_code": "CASH02_MULTI_BRANCH" },
                              { "when": "feat.limit_enquiry_logged_7d == true", "to": "high", "reason_code": "CASH02_LIMIT_ENQUIRY" } ] },
    "reason_codes": [ { "code": "CASH02_SPLIT_BELOW_LIMIT", "facts": { "n": "n", "total": "total", "days": ":window" } } ],
    "case_family": "cash",
    "evidence": { "transactions": "all_in_window", "fields": ["txn.branch_id", "txn.channel_id", "cash_event.depositor_id?"] },
    "template": "cash_structuring_v1"
  },
  "tests": {
    "unit": [
      { "name": "three deposits of 9,500 in five days", "fixture": "fx/cash02_pos_1.json", "expect": { "alerts": 1, "severity": "medium" } },
      { "name": "deposits at two branches", "fixture": "fx/cash02_pos_2.json", "expect": { "alerts": 1, "severity": "high" } },
      { "name": "linked depositor completes the pattern", "fixture": "fx/cash02_pos_3.json", "expect": { "alerts": 1 } },
      { "name": "two deposits only", "fixture": "fx/cash02_neg_1.json", "expect": { "alerts": 0 } },
      { "name": "one deposit reversed", "fixture": "fx/cash02_neg_rev.json", "expect": { "alerts": 0 } },
      { "name": "IQD deposits only", "fixture": "fx/cash02_neg_iqd.json", "expect": { "alerts": 0 } }
    ],
    "seeded": ["SEED-CBI-A1-02-P", "SEED-CBI-A1-02-N", "SEED-CBI-A1-12-P", "SEED-CBI-A1-12-N"]
  },
  "changelog": { "ar": "الإصدار الأول", "en": "First version" }
}
```

The `limit` parameter is pinned (`min = max`) because the USD 10,000 figure is a CBI number; the bank cannot move it,
only the band below it.

### 9.2 Event — SCN-DORM-01 reactivation with a large movement

```json
{
  "dsl": "scenario", "dsl_version": "1",
  "id": "SCN-DORM-01", "version": "1.0.0",
  "name": { "ar": "حركة كبيرة عند إعادة تنشيط حساب راكد", "en": "Reactivation with a large movement" },
  "family": "DORM", "pack": "akr", "typology": ["dormant_account_abuse"],
  "cbi_refs": [ { "article": 3, "no": 1, "rel": "implements" }, { "article": 1, "no": 6, "rel": "implements" } ],
  "owner": "role:M", "classification": "confidential", "evidence_label": "S",
  "phase2_candidate": true, "latency_class": "L1",
  "data_requirements": ["txn.amount_usd", "account.type", "feat.dormant_days", "feat.p99_single_txn_usd_segment"],
  "parameters": {
    "dormant_days": { "type": "int", "default": 180, "min": 180, "max": 180, "basis": "CBI" },
    "min_usd": { "type": "decimal", "default": 5000, "min": 1000, "max": 50000, "band": [3000, 7000], "basis": "DP", "review_by": "2027-04-30" },
    "seg_multiple": { "type": "decimal", "default": 1.0, "min": 0.5, "max": 3.0, "band": [0.8, 1.5], "basis": "PCT" }
  },
  "logic": {
    "kind": "event", "on": "txn.posted",
    "context": ["txn", "account", "feat.dormant_days", "feat.p99_single_txn_usd_segment"],
    "condition": "account.type == 'current' && feat.dormant_days >= :dormant_days && txn.amount_usd >= max(:min_usd, :seg_multiple * feat.p99_single_txn_usd_segment)"
  },
  "outputs": {
    "severity": { "base": "high", "uplift": [] },
    "reason_codes": [ { "code": "DORM01_LARGE_FIRST_MOVE", "facts": { "days": "feat.dormant_days", "amount": "txn.amount_usd" } } ],
    "case_family": "dormant",
    "evidence": { "transactions": "triggering_and_previous_30d", "fields": ["account.dormant_since", "txn.channel_id"] },
    "template": "dormant_reactivation_v1"
  },
  "tests": { "unit": [], "seeded": ["SEED-CBI-A3-01-P", "SEED-CBI-A3-01-N"] }
}
```

Saved as a draft with `tests.unit` empty, it validates; leaving `testing` without six cases fails with
`E_TESTS_MISSING`. The six-month rule applies to current accounts only (file 27 B17), hence the account-type test.

### 9.3 Profile — SCN-ACCT-01 income above the declared profile

```json
{
  "dsl": "scenario", "dsl_version": "1",
  "id": "SCN-ACCT-01", "version": "1.0.0",
  "name": { "ar": "دخل أو حركة تتجاوز الملف المصرّح به", "en": "Income or turnover above the declared profile" },
  "family": "ACCT", "pack": "akr", "typology": ["profile_deviation"],
  "cbi_refs": [ { "article": 1, "no": 15, "rel": "implements" }, { "article": 1, "no": 17, "rel": "implements" },
                { "article": 1, "no": 14, "rel": "extends" } ],
  "owner": "role:M", "classification": "confidential", "evidence_label": "S",
  "phase2_candidate": false, "latency_class": "L2",
  "data_requirements": ["txn.amount_usd", "txn.dr_cr", "customer.expected_profile.monthly_income?", "customer.segment"],
  "aggregation": { "level": "customer", "net_reversals": true, "posted_only": true, "currency_mode": "usd_equivalent" },
  "parameters": {
    "declared_ratio": { "type": "decimal", "default": 1.5, "min": 1.1, "max": 3.0, "band": [1.3, 1.8], "basis": "KYC",
                        "overrides": [ { "when": ["risk:4"], "value": 1.2, "basis": "KYC" }, { "when": ["risk:5"], "value": 1.2, "basis": "KYC" } ] },
    "min_excess": { "type": "decimal", "default": 2000, "min": 500, "max": 20000, "basis": "KYC" },
    "own_multiple": { "type": "decimal", "default": 3.0, "min": 1.5, "max": 10, "basis": "DP", "review_by": "2027-04-30" }
  },
  "logic": {
    "kind": "profile",
    "measure": { "op": "sum", "field": "txn.amount_usd", "where": "txn.dr_cr = 'C'" },
    "period": { "kind": "calendar", "length": { "value": 1, "unit": "months" } },
    "expected": [
      { "source": "declared", "field": "customer.expected_profile.monthly_income", "test": "ratio_above", "value": ":declared_ratio", "min_excess": ":min_excess" },
      { "source": "own_history", "lookback": { "value": 12, "unit": "months" }, "stat": "median", "test": "ratio_above", "value": ":own_multiple" },
      { "source": "peer", "peer_group": "customer.segment", "stat": "p99", "test": "above", "fallback_only": true }
    ],
    "combine": "any",
    "min_history": { "value": 3, "unit": "months" }
  },
  "outputs": {
    "severity": { "base": "medium", "uplift": [] },
    "reason_codes": [ { "code": "ACCT01_ABOVE_DECLARED", "facts": { "actual": "measure", "declared": "customer.expected_profile.monthly_income" } } ],
    "case_family": "profile",
    "evidence": { "transactions": "credits_in_period", "fields": ["customer.expected_profile"] },
    "template": "profile_deviation_v1"
  },
  "tests": { "unit": [], "seeded": [] }
}
```

A customer without a declared income is compared with the segment P99 and the alert and customer are marked
degraded (S-10); the case shows "declared income missing" as a data gap.

### 9.4 Mapping — transactions from a bank-owned Oracle view

```json
{
  "dsl": "mapping", "dsl_version": "1",
  "id": "ics_core.txn", "version": 1,
  "name": { "ar": "حركات الحسابات", "en": "Account transactions" },
  "owner": "role:DS",
  "entity": "txn",
  "source": { "source": "ics_core", "object": "AKR_V_TXN", "format": "relational", "key": ["TXN_ID", "BOOK_DT"] },
  "change": { "strategy": "poll", "watermark": { "kind": "timestamp_column", "field": "LAST_UPD" },
              "overlap": "PT10M", "settle": "PT2M", "lookback": "auto" },
  "filter": [ { "expr": "row.GL_ONLY != 'Y'", "reason_code": "MAP_EXCL_GL_INTERNAL" } ],
  "fields": {
    "account_id":       { "from": "ACC_NO", "do": ["keyed_hash:account_number"] },
    "booking_date":     { "from": "BOOK_DT", "do": ["parse_date:yyyy-MM-dd"] },
    "value_date":       { "from": "VAL_DT", "do": ["parse_date:yyyy-MM-dd"] },
    "source_local_ts":  { "from": "POST_TS", "do": ["parse_timestamp:yyyy-MM-dd HH:mm:ss,Asia/Baghdad"] },
    "amount":           { "from": "AMT", "do": ["parse_decimal", "abs"] },
    "dr_cr":            { "from": "AMT", "do": ["dr_cr_from_sign"] },
    "currency":         { "from": "CCY", "do": ["code_map:ics_currency"] },
    "txn_type":         { "from": "TRN_CODE", "do": ["code_map:ics_txn_type"] },
    "channel_id":       { "from": "CHANNEL", "do": ["code_map:ics_channel"] },
    "branch_id":        { "from": "BR_CODE", "do": ["code_map:ics_branch"] },
    "status":           { "from": "STATUS", "do": ["code_map:ics_txn_status"] },
    "narrative_raw":    { "from": "NARR", "do": ["digits_western", "trim"] },
    "operator_id":      { "from": "MAKER_ID" },
    "authoriser_id":    { "from": "CHECKER_ID" },
    "reverses_id":      { "from": "REV_OF_TXN" },
    "cash_flag":        { "expr": "row.TRN_CODE in ['CDEP', 'CWDL', 'CEXC']" }
  },
  "validate": [
    { "rule": "not_null", "field": "account_id", "on_fail": "quarantine" },
    { "rule": "not_placeholder_date", "field": "booking_date", "on_fail": "quarantine" },
    { "rule": "ref", "field": "currency", "table": "ref.currencies" },
    { "rule": "no_mojibake", "field": "narrative_raw" }
  ],
  "changelog": { "ar": "الإصدار الأول", "en": "First version" }
}
```

The object name, columns and codes are illustrative; the real ones come from the bank's view DDL pack and dictionary.
The account number itself is never stored in clear here: the `account_id` link is resolved through the keyed hash to
the canonical account.

### 9.5 Rating model — excerpt

```json
{
  "dsl": "rating_model", "dsl_version": "1", "id": "RM-IQ", "version": "1.0.0",
  "name": { "ar": "نموذج تصنيف مخاطر العملاء", "en": "Customer risk rating model" },
  "owner": "role:M", "regulatory_profile": "iraq_cbi",
  "levels": [ { "level": 1, "from": 0, "to": 15, "cbi": "low" }, { "level": 2, "from": 15, "to": 35, "cbi": "low" },
              { "level": 3, "from": 35, "to": 55, "cbi": "medium" }, { "level": 4, "from": 55, "to": 75, "cbi": "high" },
              { "level": 5, "from": 75, "to": 100, "cbi": "high" } ],
  "blocks": [ { "id": "A", "weight": 25 }, { "id": "B", "weight": 15 }, { "id": "C", "weight": 20 },
              { "id": "D", "weight": 15 }, { "id": "E", "weight": 5 }, { "id": "F", "weight": 20 } ],
  "factors": [
    { "id": "A2", "block": "A", "weight": 5, "applies_to": ["entity"], "input": "party.entity_age_days",
      "anchors": { "kind": "bands", "bands": [ { "below": 90, "score": 100 }, { "below": 365, "score": 75 },
                                               { "below": 730, "score": 50 }, { "below": 1826, "score": 25 },
                                               { "otherwise": true, "score": 10 } ] },
      "freshness": { "value": 36, "unit": "months" }, "basis": "Wolfsberg RBA 2026 (newly formed entities)" },
    { "id": "C3", "block": "C", "weight": 35, "applies_to": ["person", "entity"], "input": "country_table.max_level",
      "anchors": { "kind": "country_table", "map": { "low": 10, "medium": 40, "high": 70, "very_high": 100 } },
      "freshness": { "value": 12, "unit": "months" }, "basis": "Country table" }
  ],
  "unknown_score": 60, "peak_term": 0.35,
  "floors": [
    { "id": "FL-01", "when": "pep.tier == 'P1'", "level": 5, "basis": "CBI PEP circular 26 Aug 2026; FATF R.12" },
    { "id": "FL-08", "when": "has(party.last_str_at)", "level": 4, "decay": { "value": 24, "unit": "months" },
      "basis": "EBA (subject of a past STR)" },
    { "id": "FL-11", "when": "exit_register.match", "level": 5, "action": "block_onboarding", "basis": "Starling (FCA 2024)" }
  ],
  "completeness_cap": { "mandatory": { "person": ["B1", "C1", "A4"], "entity": ["A1", "A3", "B1", "A6"] },
                        "one_or_more_missing": { "min_level": 3 },
                        "two_or_more_missing": { "action": "escalate_review", "block_rating": true } },
  "triggers": [
    { "id": "TR-10", "on": "country_table.changed", "when": "true", "action": "rescore",
      "review_within": { "value": 1, "unit": "business_days" }, "owner": "role:KYC" },
    { "id": "TR-15", "on": "case.decision", "when": "decision.type in ['str', 'tfr', 'escalate']",
      "action": "rescore_and_review", "review_within": { "value": 5, "unit": "business_days" }, "owner": "role:KYC" }
  ],
  "stickiness": { "rise": "immediate", "fall": "at_review", "cross_floor": false },
  "overrides": { "up": { "roles": ["role:KYC", "role:M", "role:A2"], "reason_required": true },
                 "down": { "roles": ["role:M", "role:KYC"], "max_levels": 1, "max_expiry": { "value": 12, "unit": "months" },
                           "maker_checker": true, "exclude_relationship_owner": true, "evidence_required": true },
                 "never_below": ["floor", "completeness_cap"] },
  "controls": [
    { "level": 1, "cdd": "sdd_or_cdd", "approver": "position:branch_officer", "monitoring_profile": "baseline",
      "classification_review": { "value": 24, "unit": "months" }, "kyc_refresh": { "value": 36, "unit": "months" },
      "light_check": { "value": 12, "unit": "months" } },
    { "level": 5, "cdd": "edd_plus", "approver": "position:board_committee", "monitoring_profile": "hold_on_match",
      "classification_review": { "value": 6, "unit": "months" }, "kyc_refresh": { "value": 12, "unit": "months" },
      "light_check": { "value": 1, "unit": "months" } }
  ],
  "sanity_bands": { "high_share": { "min": 0.01, "max": 0.20 } }
}
```

Shown abridged: the full shipped model lists all 27 factors, FL-01 to FL-15, TR-01 to TR-26 and five controls rows,
and only then passes `E_WEIGHTS_SUM` and `E_CONTROLS_MISSING`. The `position:` approvers in `controls` are bank-side
positions that the authority matrix maps to people (C.7).

### 9.6 Change request — a loosening parameter change, and the validator's answer

```json
POST /api/v1/change-requests
Idempotency-Key: 0192f3d0-...
{
  "object_kind": "parameter_set",
  "object_id": "0192f3a4-...",
  "base_version": 4,
  "proposed": { "kind": "parameter_set", "draft_id": "0192f3b0-..." },
  "reason_code": "TUNE_CAPACITY_BACKLOG",
  "reason_text": "Raise the total for cash retailers from 15,000 to 20,000 USD.",
  "evidence": [ { "kind": "replay", "id": "0192f3c1-..." } ]
}
```

```json
HTTP/1.1 422
{
  "type": "https://akr-tms.local/errors/dsl-validation", "status": 422,
  "errors": [
    { "path": "/proposed/values/total/overrides/0/value", "code": "W_OUTSIDE_BAND",
      "message": "20000 is inside min/max but outside the approved band [8000, 12000]; class is material", "severity": "warning" },
    { "path": "/evidence", "code": "E_CAPACITY_NO_BTL",
      "message": "reason TUNE_CAPACITY_BACKLOG is tagged capacity; attach a below-the-line sample", "severity": "error" },
    { "path": "/volume_effect", "code": "W_LOOSENING",
      "message": "replay 0192f3c1 shows 41 fewer alerts a month (-18%); routed to cr_material with a compliance approver", "severity": "warning" }
  ]
}
```

The request is refused only for the missing below-the-line sample. Outside the band is a warning about the class, not
an error: the value is inside the hard bounds, so once the sample is attached the change goes the material route.

### 9.7 Workflow definition — `cr_material`

```json
{
  "dsl_version": "1",
  "key": "cr_material",
  "name": { "ar": "اعتماد تغيير جوهري", "en": "Material change approval" },
  "classification_default": "confidential",
  "start": "approvals",
  "steps": {
    "approvals": {
      "type": "parallel",
      "branches": ["mlro_approval", "second_approval"],
      "join": "all",
      "next": [ { "if": { "field": "cr.volume_effect", "op": "eq", "value": "decrease" }, "to": "compliance_approval" },
                { "else": true, "to": "activate" } ]
    },
    "mlro_approval": {
      "type": "approve",
      "label": { "ar": "موافقة مسؤول الإبلاغ", "en": "MLRO approval" },
      "assignee": { "kind": "named_officer", "officer": "mlro" },
      "stepup": true,
      "timeout": { "after": { "value": 3, "unit": "business_days" }, "action": { "kind": "notify_superior" } },
      "reject": { "comment_required": true, "mode": "end" },
      "next": [ { "else": true, "to": "@join" } ]
    },
    "second_approval": {
      "type": "approve",
      "label": { "ar": "موافقة ثانية من خارج العمليات", "en": "Second approval outside operations" },
      "assignee": { "kind": "role", "role": "CHK", "scope": "bank" },
      "stepup": true,
      "timeout": { "after": { "value": 3, "unit": "business_days" }, "action": { "kind": "notify_superior" } },
      "reject": { "comment_required": true, "mode": "end" },
      "next": [ { "else": true, "to": "@join" } ]
    },
    "compliance_approval": {
      "type": "approve",
      "label": { "ar": "موافقة رئيس الامتثال", "en": "Compliance head approval" },
      "assignee": { "kind": "role", "role": "CH", "scope": "bank" },
      "stepup": true,
      "reject": { "comment_required": true, "mode": "end" },
      "next": [ { "else": true, "to": "activate" } ]
    },
    "activate": { "type": "terminal", "outcome": "approved", "effects": [ { "kind": "activate_change" } ] }
  }
}
```

The maker of the change request and anyone who already decided on this instance are excluded from every `approve`
step by the engine (E.2); the definition cannot switch that off. `cr_material_validated` is the same definition with
an `approve` step for the independent validator (MRO) placed before `approvals`. The authority matrix decides who
holds CHK and CH at each bank.
