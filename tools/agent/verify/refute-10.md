# Refuter, phase 10 (Haiku draft for lead review)

### V-10-01-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM for the lock part only)
- Claim: Step 4 of validate (section 3.1) names no draft precondition (not_draft, 409) and no lock or compare-and-set on `revision` between the compile and the store; section 6 says "422 or 409" for the same refusal.
- Evidence:
  - Precondition, REFUTED: docs/plan/phase-10.md:1796 "moves `draft` -> `testing`; on failure returns 422" (the step's only state rule, a draft-to-testing move); docs/plan/phase-10.md:2051 "`E_NOT_ALLOWED_IN_STATE` (409, reasons `not_draft`, `not_testing`, `version_open`" (the reason is catalogued at 409); docs/plan/phase-10.md:2312 "`E_NOT_ALLOWED_IN_STATE` reason `not_draft`" (the validate test).
  - Lock or compare-and-set, STANDS: docs/plan/phase-10.md:1619 "async fn store_compiled(&self, tx: &mut Tx, id: Uuid, compiled: &Value" (no expected revision); step 4 at lines 1795-1796 names no compare-and-set. The general rule docs/API.md:43 "Mutations on versioned resources require" covers only the request-level If-Match, not the window between compile and store.
  - Section 6 wording, LOW: docs/plan/phase-10.md:2311 "-> 422 or 409 with" against docs/plan/phase-10.md:2051 (409 for `E_NOT_ALLOWED_IN_STATE`).
- Fix (lock part): section 3.1 step 4 and the `store_compiled` signature in section 2 (line 1619): pass the revision read at the start and make the store a compare-and-set on `revision` (409 `E_VERSION_CONFLICT` on a mismatch). Section 6 item 5 (line 2311): say 409 only.

### V-10-02-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: No step resolves the canonical ids of the payment message and the txn that a link names (`message_id`, `txn_id`); `xref` (step 2) covers only the row's own key; `xref_linked` (step 8) is undefined; the P1 `rule` and `confidence` values are not named.
- Evidence (greps that found nothing settling it):
  - Step 2 reads only the row's own key: docs/plan/phase-10.md:1979 "`xref` (source, key, entity) is read in one statement". No step looks up the key of a second entity or another source.
  - `xref_linked` is only a return field: docs/plan/phase-10.md:2005 "conflicts: Vec<RowConflict>, xref_linked }". Grep for `xref_linked` over docs/ and research/ finds no other text.
  - The only cross-reference rule is the validation rule `ref`: docs/DSL.md:598 "Referential check against a canonical or reference table". It checks a value, it does not give the canonical id for a link.
  - The P1 values are missing: docs/plan/phase-10.md:1090 "TEXT NOT NULL," (`rule` has no CHECK and no list), docs/plan/phase-10.md:1089 "confidence NUMERIC(5,4)" (range only). Grep for "declared" with link, and for "payment link" in research.md, finds no value. DR-394 (docs/plan/research.md:405) defers matching, not the declared link.
  - The FK targets need the ids: docs/plan/phase-10.md:1101 "CONSTRAINT fk_payment_link_message FOREIGN KEY (message_id, message_received_at)".
- Fix: section 3.7 step 2 (line 1979): add one sentence that resolves the message and the txn of a link through `xref` by their own source keys and entity, and name the P1 `rule` and `confidence` values for a declared link; section 3.7 step 8 (line 2005) must say what `xref_linked` counts.

### V-10-02-b  PARTLY   (first agent: MEDIUM; refuter: MEDIUM for account_hold, LOW for the other two)
- Claim: Step 3 classes (fact, bi-temporal, dimension) and step 6 (children) do not include `payment_link`, `payment_message_party` or `account_hold`, so the rule for a changed row of these three tables is not stated.
- Evidence:
  - The omission is real: the step 3 lists and step 6 name neither table (greps find them only in the entity order, docs/plan/phase-10.md:1976 "account_hold, balance_snapshot, txn, txn_event, payment_message, payment_message_party,").
  - Refuted for payment_link and payment_message_party: the general rule in docs/DATA-MODEL.md:45 "a canonical row whose source `row_hash` changes under the same key is not overwritten; the change opens" (a conflict, not an overwrite); step 3 cites the same rule at docs/plan/phase-10.md:1985 "the new version stays in `raw` (DATA-MODEL s.0, DR-37)".
  - Standing for account_hold: docs/DATA-MODEL.md:46-47 "Status changes a source makes after creation ... are kept as events or versions, never overwrites." Phase 10 gives versions only to the five bi-temporal tables (docs/plan/phase-10.md:1987 "new version. Step 4 closes the current one."), and account_hold has no bi-temporal columns (docs/plan/phase-10.md:693-713, `start_at`, `end_at`). A release (an `end_at` set on a held row) therefore has no defined outcome.
- Fix: phase 10, section 3.7 step 3 (lines 1981-1990): name the class of `account_hold` and what a release (a changed `end_at`) writes; add `payment_link` and `payment_message_party` to the fact list of the same step.

### V-10-03-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM for the class part, LOW for the refusal part)
- Claim: The class of a `Verify`-only change is not stated; refusals of invalid operations (a `Map` of a code with an effective entry, a `Reclassify` with no effective entry or an unmatched `supersedes_valid_from`, a `Verify` of an unknown entry) have no error reason; section 3.9 Prepare cites "the operations are valid (section 3.4)".
- Evidence:
  - Class, STANDS: docs/plan/phase-10.md:1893 "`minor` when every operation maps a previously unmapped code, `material` when any" (a `Verify` maps nothing, so the minor test fails, and it reclassifies nothing, so the material test fails). The class table has no `Verify` row: docs/DSL.md:932 "Code map: mapping a previously unmapped code | Minor" and docs/DSL.md:933 "Code map: reclassifying a mapped code | Material". Phase 04 takes the base class only from that table: docs/plan/phase-04.md:825 "the target's `Prepared.base_class` (DSL.md D.3 table". No default class is named.
  - Refusal, PARTLY REFUTED: the general code exists. docs/plan/phase-00.md:499 "`Validation(Vec<ValidationError>)` | 422 | `E_VALIDATION`" and phase 10 names it for "the reasons of sections 3.1 to 3.4" (docs/plan/phase-10.md:2050). The gap is the reason names only: section 3.4 gives one 409 reason for create (docs/plan/phase-10.md:1882 "reason `code_map_exists`"), and nothing for the three operation refusals. Phase 04 step 3 (docs/plan/phase-04.md:874 "`prepared = target.prepare(ctx, body)` (validation") gives no code either.
  - First-agent grep correction: `E_NOT_ALLOWED_IN_STATE` does appear in lines 1879-1899 (docs/plan/phase-10.md:1882, `code_map_exists`), so that grep result is wrong; it does not change the verdict.
- Fix: phase 10, section 3.4 step 4 and section 3.9 code_map Prepare cell (docs/plan/phase-10.md:2035): name the class of a `Verify`-only change, and name the reason and status (422 `E_VALIDATION` or 409 `E_NOT_ALLOWED_IN_STATE`) of each refused operation.

### V-10-03-b  PARTLY   (first agent: MEDIUM; refuter: MEDIUM for the target event, none for the request-level audit)
- Claim: The revert of a `code_map` change closes the `Map` entries it added (`valid_to`), a mutation, and names no audit event; phase 04 says `target.revert` writes its own audit event.
- Evidence:
  - Request-level audit, REFUTED: the revert of a change request is audited by phase 04 in both paths. docs/plan/phase-04.md:934 "audit `change_request.reverted {cause: review_rejected}`" and docs/plan/phase-04.md:936 "audit `change_request.reverted {cause:". So the mutation is not unaudited.
  - Target event, STANDS: docs/plan/phase-04.md:933 "`target.revert` (restores the base version, writing its own audit event)". Phase 10 names no such event: docs/plan/phase-10.md:2035 Revert cell "the entries are closed (`valid_to`) for the `Map` operations of the change"; the audit table lists only docs/plan/phase-10.md:2065 "`code_map.created`, `code_map.prefilled`" and docs/plan/phase-10.md:2066 "`code_map.entry_proposed`, `code_map.entry_activated`". Grep for `revert` in section 3.4 and section 3.10 finds no match.
- Fix: phase 10, section 3.10 audit table (after line 2066): name the event kind that the code-map revert writes for the entries it closes, and name it in the section 3.9 `code_map` Revert cell (line 2035).

### V-10-03-c  PARTLY   (first agent: LOW; refuter: LOW for the wrong citation)
- Claim: The section 3.4 heading cites DR-38 as a code map decision; DR-38 is the field-availability rule and no research decision covers code maps.
- Evidence:
  - Wrong citation, STANDS: docs/plan/research.md:49 "Field availability distinguishes four states per canonical attribute" (DR-38 is the field-availability rule, not a code-map rule). The heading at docs/plan/phase-10.md:1879 "C-50, DSL.md B.7, DR-38" cites it for code maps.
  - "No research decision covers code maps", REFUTED: the first agent's grep used "code map|code_map" and missed the hyphenated form. docs/plan/research.md:21 "a `gate` per code-map kind" (DR-10: the gate of each code-map kind, the default for `txn_type`) and docs/plan/research.md:99 "code-map prefills from the CBS dictionary are unverified rows that a change request verifies" (DR-88) both cover code maps.
- Fix: phase 10, section 3.4 heading (line 1879): cite DR-10 and DR-88 in place of DR-38.

### V-10-04-a  STANDS   (first agent: HIGH; refuter: MEDIUM)
- Claim: The mapping path of `cdd_level` is not stated. No code map kind, transform or validation rule converts the CBS CDD or risk value to `sdd`, `cdd` or `edd`, and the CHECK refuses any other value.
- Evidence (greps that found nothing settling it):
  - The CHECK admits only three values: docs/plan/phase-10.md:609 "cdd_level TEXT CHECK (cdd_level IN ('sdd', 'cdd', 'edd'))".
  - No code map kind: docs/plan/phase-10.md:132 "kind TEXT NOT NULL CHECK (kind IN ('txn_type', 'product'" (the ten kinds of docs/DSL.md B.7 contain no CDD or risk kind).
  - No transform: the `Transform` list at docs/plan/phase-10.md:1582 "closed enum: `ParseDate`" has no CDD, risk or level transform; the only generic one is `CodeMap(kind)`, which needs a kind. `ValidationRule::Enum` checks a value and does not translate it.
  - The CDD values in DSL.md are the rating model's own required levels, not a CBS translation: docs/DSL.md:1468 `"cdd": "sdd_or_cdd"` (controls of the computed level).
  - Why the severity is MEDIUM: the value is never a rating input until the bank declares it maintained (docs/DATA-MODEL.md:317 "the attribute is never used as a rating or scenario input until the bank declares"), and the CBS risk levels are all constant or empty (docs/plan/research.md:49 "all constant or empty on every customer"). Until a declaration, the column is null (docs/DATA-MODEL.md:424 "`cdd_level` ... are null when the source does not hold them"). The missing translation therefore blocks only the declared case, not the first build.
- Fix: phase 10, section 3.2 step 6 (transforms) or a code map kind in section 1 (lines 132-133): name the rule that maps each CBS risk value to `sdd`, `cdd` or `edd`, or state that the column stays null until the bank declares the field and then how the declared mapping works.

### V-10-05-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: `MappingExecutor::map(version, rows) -> MappedBatch` (the batch entry point that phase 11 and section 7.6 call), the row types `MappedRow`, `Rejected` and `MappedBatch`, and the reason codes of a row failure are not defined in phase 10.
- Evidence (greps that found nothing settling it):
  - Only names, no definition: docs/plan/phase-10.md:1950 "`MappingExecutor::apply(row) -> MappedRow | Rejected`" (the per-row function, which has a different name from the batch call); docs/plan/phase-10.md:1971 "`CanonicalWriter::write(tx, batch: &MappedBatch) -> Result<WriteOutcome, WriteError>`" (the writer input, whose type is not defined). Section 2 value objects (docs/plan/phase-10.md:1581-1591) list no `MappedRow`, `Rejected`, `MappedBatch` or `MappingExecutor`. Grep for `fn map` and `MappingExecutor::map(` in phase 10 finds nothing.
  - The batch entry point is called elsewhere but not defined: docs/plan/phase-11.md:796 "`MappingExecutor::map(version, rows)` (phase 10, pure transforms) returns `MappedBatch` with row failures". Section 7.6 relies on it: docs/plan/phase-10.md:2364 "with the same result type as `map`, for replays and retries."
  - Reason codes are deferred to phase 10 and not given: docs/plan/phase-11.md:797 "reason codes from phase 10". Phase 10 names only the counters of section 3.6 step 5 and the failure type of the transform function, docs/plan/phase-10.md:1859 "`apply(Transform, Value) -> Result<Value, TransformFailure>`", whose variants are not named.
  - The general rule does not cover it: the phase 00 mapping of `Validation` to 422 (docs/plan/phase-00.md:499) applies to request bodies, not to row outcomes.
- Fix: phase 10, section 2 value objects (after line 1590): define `MappingExecutor::map` with its return type, the three row types and their fields, and the row failure reason codes (also named in section 3.6 step 4, line 1950).

### V-10-06-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM for the writer lock, none for the purge lock claim)
- Claim: The tombstone check and the insert of a `canon.customer` or `canon.account` identity do not name the lock or transaction that makes them safe against the retention purge; the writer's only lock is the per-entity advisory lock of Open doubts 5, and the purge takes `akr.retention` instead.
- Evidence:
  - Purge lock, REFUTED in part: the purge takes a per-party lock as well as the run lock. docs/plan/phase-23.md:548 "`pg_try_advisory_lock(hashtext('akr.retention'))`" is the run-level lock, and docs/plan/phase-23.md:653 "`pg_advisory_xact_lock(hashtext('party:' || party_id))`" is taken per party in its own transaction (section 3.5 step 2). The phrase "the purge takes `akr.retention` instead" is therefore wrong.
  - Writer lock, STANDS: docs/plan/phase-10.md:2441 "serialises per entity id with an advisory lock" names no key. The party lock of phase 23 is keyed by party id (docs/plan/phase-23.md:886 "(`pg_advisory_xact_lock(hashtext('party:' || id))`, ascending ids)"), and the lock order of section 3.15 lists hold placement and the purge as its takers, not the writer (docs/plan/phase-23.md:886-888). A customer or account identity keyed by its own entity id does not match the party key, so the tombstone read and the insert can still interleave with the purge's commit.
  - Greps: `advisory` in phase 10 (lines 878, 858, 2280, 2441) shows no lock on the writer path; `tombstone` appears only in section 7.4 (lines 2354-2356).
- Fix: phase 10, section 7.4 (lines 2353-2356): say the writer takes the phase 23 party lock for the party of the identity before its tombstone read; phase 23 section 3.15 (line 886) lists the writer among the takers of that lock.

### V-10-07-a  PARTLY   (first agent: HIGH; refuter: HIGH for the result type only, same root as V-10-05-a)
- Claim: `MappingExecutor::map_payload` has no signature (payload type, version argument, row or batch result type); `map` is not defined in phase 10 and phase 11 gives no signature either.
- Evidence:
  - Inputs, REFUTED: the input is stated in prose: docs/plan/phase-10.md:2364 "the pure transforms of a version applied to one stored landing payload". The payload is the stored landing row (phase 11 `payload`, docs/plan/phase-11.md:240-246), and the version is a `MappingVersion`, so the two arguments are named and their types follow from the named tables.
  - Result type, STANDS: docs/plan/phase-10.md:2365 "with the same result type as `map`, for replays and retries." The result type is therefore that of `map`, which is defined nowhere in phase 10 (see V-10-05-a). Phase 11 gives only prose: docs/plan/phase-11.md:1387 "`MappingExecutor` gains `map_payload` (the pure transforms applied to a stored payload, used by replays and retries)." Grep for `map_payload` across docs/ and research/ finds only these four lines.
- Fix: phase 10, section 7.6 (line 2364): state the return type of `map_payload`, and define the result type it shares with `map` in section 2 (the same fix as V-10-05-a).

### V-10-08-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: No rule names the transform or mapping field that fills `light`, `folded` and `key` of `canon.party_name`. Section 3.3 names only `FoldArabic`; phase 14 says `key` is written through `SplitArabicName`, which phase 10 does not state.
- Evidence:
  - The general sentence does not bind the columns: docs/plan/phase-10.md:2380 "The transforms call the crate, so the columns" (7.9) says the columns come from the transforms as a set, not which transform or mapping field fills which column.
  - `FoldArabic` only: docs/plan/phase-10.md:1861 "`FoldArabic` is the folding of research file 22". No transform or field is named for `light`, and `SplitArabicName` appears only in the enum list (docs/plan/phase-10.md:1584).
  - Phase 14 is the only text that binds `key`: docs/plan/phase-14.md:1449 "Phase 10 writes `party_name.key` through `SplitArabicName`". Phase 10 does not say it.
  - Why MEDIUM and not HIGH: the meaning of the three forms is defined in research/22-arabic-name-matching.md:331 "**light** (steps N1 to N4 only: diacritics and tatweel removed, hamza kept)", so a developer knows what to compute. Only the binding of each column to a transform or mapping field is missing.
- Fix: phase 10, section 3.3 (after line 1861): one sentence per column that names the transform (or the mapping field) that writes `light`, `folded` and `key`.

### V-10-08-b  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: Two phases claim the crate `crates/names`: section 7.9 and DR-371 give phase 10 the creation, while phase 14 section 2 lists `crates/names/` as "NEW", so a workspace member would be added twice.
- Evidence:
  - The phase 14 base tree line is overridden by its own 7+ block, which says what it replaces: docs/plan/phase-14.md:1298 "`crates/names` is created by phase 10 (its section 7+, the folding behind `FoldArabic` and `SplitArabicName`)"; docs/plan/phase-14.md:1299 "extends it, so the tree line "NEW" reads "extended"".
  - The decision agrees: docs/plan/research.md:382 "Phase 10 creates the pure crate `crates/names` (fold and key tables); phase 14 extends it". Phase 10 section 7.9 says the same at docs/plan/phase-10.md:2379 "This phase creates the pure crate `crates/names`".
  - Note for phase 14 (not phase 10, LOW): the base text still says "Added to the phase 00 workspace members in the same change" at docs/plan/phase-14.md:652, which the 7.1 block does not retract in words. The first agent's grep for `crates/names` missed phase-14.md:1298.

### V-10-09-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: No write path for a status change. A changed source row is a `RowConflict` on a fact, so the writer never sets `status` or appends to `status_history`, though 7.12, the DDL comment and the grant assume it does. No repository method performs that update.
- Evidence:
  - The writer's rule for a changed fact: docs/plan/phase-10.md:1984 "not overwritten; a `RowConflict` is returned". `payment_message` is in the fact list (docs/plan/phase-10.md:1983).
  - The assumption that a status change is appended: docs/plan/phase-10.md:1025 "Status changes after creation are appended to status_history, never overwritten"; docs/plan/phase-10.md:2403 "appended to `status_history` use the same list"; docs/plan/phase-10.md:1147 "GRANT UPDATE (status, status_history) ON canon.payment_message TO akr_ingest;".
  - No repository method: docs/plan/phase-10.md:1674-1681 (`CanonRepository` has `current_by_source_key`, `insert_rows`, `close_versions`, `ensure_partition` only). No 7.x block says which rule wins, so the two texts conflict.
  - General rule that the status change must be kept (not that it becomes a conflict): docs/DATA-MODEL.md:46-47 "Status changes a source makes after creation ... are kept as events or versions, never overwrites."
  - Greps: `status_history` in phase 10 (lines 1025, 1041, 1147, 1173, 2222, 2403) has no step in section 3.7.
- Fix: phase 10, section 3.7 step 3 (lines 1983-1985): say what a changed status on `payment_message` does (append to `status_history` and set `status`, or record a conflict), and add the repository method that performs it to section 2 `CanonRepository`.

### V-10-09-b  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: The CHECK in 7.12 admits five values, but the unmapped-value rule stores `unknown` plus a data-quality count for a `dq_issue` gate; 7.12 says an unmapped value "follows the map's gate" and does not fix the gate of the `status` map, so every unmapped status fails the CHECK and the batch.
- Evidence:
  - The CHECK admits five values and not `unknown`: docs/plan/phase-10.md:2402 "CHECK (status IN ('received', 'released', 'rejected', 'returned', 'cancelled'))".
  - The default gate of the `status` map is `dq_issue` (every kind except `txn_type`): docs/plan/phase-10.md:1882 "`gate` defaults to `quarantine` for `txn_type` and `dq_issue` otherwise".
  - The unmapped value under that gate is `unknown`: docs/plan/phase-10.md:1899 "a value with no effective, verified entry is unknown with a data-quality count".
  - 7.12 leaves the gate open: docs/plan/phase-10.md:2401 "an unmapped value follows the map's `gate`".
  - The general rules point the other way: docs/DSL.md:634 "An unmapped value becomes `unknown` plus a data-quality issue, never a failed batch (C-50)."; docs/DATA-MODEL.md:39-40 "An unmapped code becomes `unknown` with a data-quality issue (DSL.md B.7), never a default." So the CHECK contradicts both, and no 7.x block says which wins.
- Fix: phase 10, section 7.12 (line 2401): name the gate of the `status` map and what an unmapped status writes, so that the CHECK and the batch outcome agree with DSL.md B.7 (line 634).

### V-10-09-c  STANDS   (first agent: LOW; refuter: LOW)
- Claim: The grant comment at line 1147 cites section 3.6 for "append-only in practice", but section 3.6 (the sampler) does not mention status or `status_history`; the write rule is in section 3.7 step 3.
- Evidence: docs/plan/phase-10.md:1147 "-- append-only in practice (section 3.6)". Grep for `status` in docs/plan/phase-10.md lines 1939-1968 (section 3.6) finds no match. The status write rule, such as it is, sits in section 3.7 step 3 (docs/plan/phase-10.md:1983-1985), and that rule conflicts with the grant (see V-10-09-a).
- Fix: phase 10, the grant comment at line 1147: cite section 3.7 step 3 (and 7.12) instead of section 3.6, once V-10-09-a is settled.

### V-10-10-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM for the undefined types)
- Claim: `CurrentRow` (returned by `current_by_source_key`) and `CanonRow` (taken by `insert_rows`) are not defined, and the field that the GREATEST rule reads (the current version's `valid_from`) is not named.
- Evidence:
  - Field named, REFUTED: section 7.13 names it in prose: docs/plan/phase-10.md:2412 "<the current version's valid_from>)`. When it equals the current version's `valid_from`". The comparison key is named in step 3: docs/plan/phase-10.md:1981 "`row_hash` is compared with the stored row for the same key".
  - Types, STANDS: docs/plan/phase-10.md:1678 "-> Result<HashMap<String, CurrentRow>, RepoError>;" and docs/plan/phase-10.md:1679 "async fn insert_rows(&self, tx: &mut Tx, e: CanonicalEntity, rows: &[CanonRow])". Grep for `CurrentRow` and `CanonRow` across docs/plan/ finds only these two lines. No field list follows them.
  - Partial cover for `CanonRow`: docs/plan/phase-10.md:1674 "each table's columns come from canonical-model.v1.json, so there is no per-entity CRUD." This names where the columns come from, but not the row type's fields.
  - Severity MEDIUM, not HIGH: the fields a developer needs for `CurrentRow` (id, `valid_from`, `recorded_from`, `row_hash`) can be read from steps 3 and 4 and 7.13; only the type definitions are missing.
- Fix: phase 10, section 2 `CanonRepository` (lines 1677-1679): name the fields of `CurrentRow` (the id, `valid_from`, `recorded_from` and `row_hash`) and state what `CanonRow` carries (the catalogue columns plus lineage).

### V-10-10-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Two sources for the date that GREATEST uses. Step 4 says the batch's business date of the row; 7.13 says the row's business date and that the raw row keeps it. Phase 11 stores the business date per batch and the raw landing table has no business-date column.
- Evidence:
  - Step 4 names the batch: docs/plan/phase-10.md:1990 "`valid_from` is the batch's business date of the row (DR-39)".
  - 7.13 names the row and says raw keeps it: docs/plan/phase-10.md:2411 "GREATEST(<the row's business date>," and docs/plan/phase-10.md:2414 "row keeps its own business date, and `recorded_from` records".
  - Phase 11 keeps one date per batch: docs/plan/phase-11.md:15 "the business date behind each batch is stored". The raw landing table has no date column: docs/plan/phase-11.md:240-246 (`id`, `source_id`, `batch_id`, `source_pk`, `extracted_at`, `row_hash`, `payload`).
  - No rule settles it: DR-39 (docs/plan/research.md:50) gives the CBS business date "per branch where the CBS keeps one" and names no row-level date. No 7.x block says which source wins.
  - Severity MEDIUM: for a batch that holds one business date, the two readings agree in value; the conflict is the statement that raw keeps a row date it does not store, and a batch that spans two dates has no defined value.
- Fix: phase 10, section 7.13 (line 2414) and section 3.7 step 4 (line 1990): use one source for the date (the batch's, as phase 11 stores it) and remove the claim that the raw row keeps its own date, or phase 11 adds the column.

### V-10-11-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `CanonRepository::close_versions` cannot express the equal case: its argument needs a `NaiveDate` valid_to for each row and its comment says it sets `valid_to`, while 7.13 requires that only `recorded_to` is set.
- Evidence:
  - The signature needs a date for every row: docs/plan/phase-10.md:1680 "async fn close_versions(&self, tx: &mut Tx, e: CanonicalEntity, ids: &[(Uuid, DateTime<Utc>, NaiveDate)])"; the comment says so too: docs/plan/phase-10.md:1681 "// sets recorded_to, valid_to".
  - The equal case needs no valid_to: docs/plan/phase-10.md:2412 "`close_versions` sets only" with the statement "its valid range is not shortened" (7.13, same line group, docs/plan/phase-10.md:2413).
  - The base step sets valid_to only in the later case: docs/plan/phase-10.md:1992 "`close_versions` sets `recorded_to` = the same time and `valid_to` = `valid_from - 1 day`... when its `valid_from` is earlier". The DDL allows a null valid_to on a current row: docs/plan/phase-10.md:620 `valid_to DATE` (no NOT NULL) and docs/plan/phase-10.md:625 "CHECK (valid_to IS NULL OR valid_to > valid_from)".
  - No general rule closes the gap: 7.13 changes the behaviour but leaves the signature unchanged.
- Fix: phase 10, section 2 `CanonRepository` (lines 1680-1681): make the valid_to of the close argument optional, or name a second method for the equal case, and make the comment match.

### V-10-12-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The writer does not say how `recorded_from` is made unique per id when two versions of one party arrive in one batch; the 7.13 test relies on it and the primary key refuses a duplicate. The order of the rows of one group, and the row that step 3 compares, are not stated.
- Evidence:
  - The mechanism gives one value per batch: docs/plan/phase-10.md:1991 "`clock_timestamp()` read once per batch, passed explicitly so it is unique per id". One reading per batch cannot separate two versions of one id in that batch.
  - The key refuses a duplicate: docs/plan/phase-10.md:624 "PRIMARY KEY (id, recorded_from)," (canon.customer; the same key shape on the other bi-temporal tables).
  - Open doubts 5 repeats one reading per batch: docs/plan/phase-10.md:2440 "`recorded_from` is one `clock_timestamp()` per batch".
  - Grouping names no order: docs/plan/phase-10.md:1974 "Group the mapped rows by `(entity, source_key)`". Step 3 compares "the stored row for the same key" (docs/plan/phase-10.md:1981), which is the stored row, not the earlier row of this batch.
  - No general rule covers it: phase 11 says nothing about one row per key in a batch (grep for "duplicate", "same key", "one row per key" in docs/plan/phase-11.md finds only unrelated lines).
- Fix: phase 10, section 3.7 steps 1 and 4 (lines 1974-1978 and 1991): name the order of the rows of one group and give the second version of an id in one batch its own `recorded_from` (or say that such a batch is refused).

### V-10-05-b  REFUTED   (first agent: LOW; refuter: none)
- Claim: The table for quarantined rows is not named in phase 10; docs/DSL.md says the row "goes to `admin.quarantine`", which phase 11 names `admin.quarantine_items`.
- Evidence:
  - Phase 10 writes no quarantine row, so it has no table to name: docs/plan/phase-10.md:1952 "No row value is written anywhere by this step." (the sampler, section 3.6 step 4). The production writer that lands quarantine rows is phase 11: docs/DATA-MODEL.md:280 "One transaction per unit writes the raw rows, the receipt, the canonical rows, the quarantine rows and the watermark".
  - The owner of the table is phase 11: docs/DATA-MODEL.md:268 "Phase 11 (moved from phase 05 with `batches` and `quarantine_items`"; docs/plan/phase-11.md:178 "CREATE TABLE IF NOT EXISTS admin.quarantine_items (". A table belongs to one phase, and later phases only use it.
  - Naming note (outside phase 10, LOW): docs/DSL.md:606 "(the row goes to" and docs/DSL.md:607 "`admin.quarantine` with the reason" name a different table. The DSL text is the one to align with phase 11, not phase 10.

### V-10-07-b  REFUTED   (first agent: LOW; refuter: none)
- Claim: The per-row function is called `apply` in phase 10 while phase 11 and section 7.6 call the batch function `map`; one name must be used.
- Evidence: the two names belong to two different functions, one per row and one per batch, so there is no naming conflict to unify.
  - Per row: docs/plan/phase-10.md:1950 "For each row, `MappingExecutor::apply(row) -> MappedRow | Rejected`" (one row in, one row out).
  - Per batch: docs/plan/phase-11.md:796 "`MappingExecutor::map(version, rows)` (phase 10, pure transforms) returns `MappedBatch`" (a set of rows in, a batch out); section 7.6 uses the same name for its result type (docs/plan/phase-10.md:2365 "with the same result type as `map`").
  - The real gap is that phase 10 never defines the batch function at all; that is V-10-05-a, not a naming question.

## Summary

- REFUTED: 3 (V-10-05-b, V-10-07-b, V-10-08-b)
- STANDS: 10
- PARTLY: 8
- Total: 21 claims, each Gap ID handled once. Quotes were checked against the cited lines (99 quotes, one fixed).

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-10-01-a | PARTLY | MEDIUM | The compile-to-store step has no compare-and-set on `revision`; the draft precondition is refuted (`not_draft` is catalogued) | section 3.1 step 4 (lines 1795-1796); `store_compiled` (line 1619); section 6 item 5 (line 2311) |
| V-10-02-a | STANDS | HIGH | No step resolves the `message_id` and `txn_id` of a link; `xref_linked` is undefined; the P1 `rule` and `confidence` values are not named | section 3.7 steps 2 and 8 (lines 1979, 2005) |
| V-10-02-b | PARTLY | MEDIUM | `account_hold` has no outcome for a changed row (a release); `payment_link` and `payment_message_party` are covered by DATA-MODEL s.0 but missing from the step 3 lists (LOW) | section 3.7 step 3 (lines 1981-1990) |
| V-10-03-a | PARTLY | MEDIUM | No class for a `Verify`-only change; the refusal reasons of the three operations are not named (LOW) | section 3.4 step 4 (lines 1892-1898); section 3.9 `code_map` Prepare cell (line 2035) |
| V-10-03-b | PARTLY | MEDIUM | The code-map revert names no target event, which phase 04 requires; the request-level `change_request.reverted` exists | section 3.9 `code_map` Revert cell (line 2035); section 3.10 audit table (after line 2066) |
| V-10-03-c | PARTLY | LOW | The section 3.4 heading cites DR-38 for code maps; the decisions meant are DR-10 and DR-88 | section 3.4 heading (line 1879) |
| V-10-04-a | STANDS | MEDIUM | No rule maps the CBS CDD or risk value to `sdd`, `cdd` or `edd`; the gap blocks only the declared case | section 3.2 step 6 or the code map kinds (lines 132-133) |
| V-10-05-a | STANDS | HIGH | `MappingExecutor::map`, `MappedRow`, `Rejected`, `MappedBatch` and the row failure reason codes are not defined | section 2 value objects (lines 1581-1591); section 3.6 step 4 (line 1950) |
| V-10-06-a | PARTLY | MEDIUM | The writer names no lock shared with the party purge; the purge takes a per-party lock (`party:` id), not only `akr.retention` | section 7.4 (lines 2353-2356); phase 23 section 3.15 (line 886) |
| V-10-07-a | PARTLY | HIGH | The result type of `map_payload` is undefined (it is `map`'s, see V-10-05-a); its inputs are stated in prose | section 7.6 (line 2364); section 2 value objects |
| V-10-08-a | STANDS | MEDIUM | No binding of `light`, `folded` and `key` to a transform or mapping field | section 3.3 (after line 1861) |
| V-10-09-a | STANDS | HIGH | No write path for a status change: a changed fact is a `RowConflict`, yet 7.12 and line 1025 assume an append to `status_history`; no repository method | section 3.7 step 3 (lines 1981-1985); section 2 `CanonRepository` (lines 1676-1684) |
| V-10-09-b | STANDS | HIGH | The status CHECK rejects `unknown`, which a `dq_issue` gate produces; the batch would fail, against DSL.md B.7 (line 634) | section 7.12 (line 2401); section 3.4 step 1 (line 1882) |
| V-10-09-c | STANDS | LOW | The grant comment cites section 3.6 for append-only status; section 3.6 does not mention status | grant comment (line 1147) |
| V-10-10-a | PARTLY | MEDIUM | `CurrentRow` and `CanonRow` are not defined; the field the GREATEST rule reads is named in 7.13 | section 2 `CanonRepository` (lines 1677-1679) |
| V-10-10-b | STANDS | MEDIUM | Two sources for the business date (batch in step 4, row in 7.13); the raw landing table keeps no date column | section 7.13 (line 2414); section 3.7 step 4 (line 1990) |
| V-10-11-a | STANDS | MEDIUM | `close_versions` cannot express the equal case, where `valid_to` must stay null | section 2 `CanonRepository` (lines 1680-1681) |
| V-10-12-a | STANDS | MEDIUM | `recorded_from` is not unique per id for two versions of one id in one batch; the order of rows in one group is not stated | section 3.7 steps 1 and 4 (lines 1974-1978, 1991) |
