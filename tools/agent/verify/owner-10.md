# Owner-decision round, phase 10 (Haiku draft for lead review)

### O-10-1  HIGH   (section 7.26 items 5 and 7; check 2)
- Problem: Item 5 says a change of `capture_source` changes the `row_hash`, and item 7 says the column travels through the mapping change path, but a mapping edit that changes only the literal leaves the landed payload, the `row_hash` and every stored row unchanged, and the writer writes nothing for an unchanged hash. The cause is the base rule of section 3.7 step 3, which applies to every mapped child value, so the lead may decide the fix base-wide.
- Proof: docs/plan/phase-10.md:2486 "A change of `capture_source` changes the `row_hash`, so the row is replaced as any changed child row is (step 6)."; docs/plan/phase-10.md:1985 "same hash: no write (`unchanged`);"; docs/plan/phase-10.md:2434 "row_hash: RowHash, payload: Value }` is one landed or sampled row."; docs/plan/phase-11.md:790 "`row_hash` is SHA-256 over the canonical JSON of the remaining payload with sorted keys."; docs/DATA-MODEL.md:45 "a canonical row whose source `row_hash` changes under the same key is not overwritten".
- Smallest fix: In item 5, replace the sentence quoted above with "A stored row changes only when its source payload changes, because `row_hash` is the landed-payload hash", and delete "the column travels with the name rows through the mapping change path" from item 7 unless the lead first writes a value comparison for the child rows of section 3.7 step 6.

### O-10-2  HIGH   (section 7.27 items 1 and 6; check 3, and check 4 for the stated verify-catalogue result)
- Problem: The block names the entity `facility_instalment` (singular) while phase 26 creates the table `canon.facility_instalments` (plural), and section 2 says the writer and reader address a table by its enum value, so the writer would address a table that phase 26 never creates and `verify-catalogue` would fail, which makes item 6's claim false. Section 7.3 has the same mismatch for `islamic_deal` and `deal_party` against `canon.islamic_deals` and `canon.deal_parties`.
- Proof: docs/plan/phase-10.md:2494 "Phase 26 adds `facility_instalment` at the end of `CanonicalEntity`"; docs/plan/phase-26.md:1016 "CREATE TABLE IF NOT EXISTS canon.facility_instalments ("; docs/plan/phase-10.md:1676 "the writer and the reader address a table by its enum value"; docs/plan/phase-10.md:2025 "A drift between the two fails the build."; docs/plan/phase-10.md:2357 "Phase 26 adds `islamic_deal` and `deal_party` at the end of `CanonicalEntity`" against docs/plan/phase-26.md:38 "CREATE TABLE IF NOT EXISTS canon.islamic_deals (".
- Smallest fix: Name the entity `facility_instalments` in items 1, 3 and 6 and in the mapping family `<source>.facility_instalments`, or rename the table to `canon.facility_instalment` in phase 26 section 7.13; make the same choice for `islamic_deal` and `deal_party` in section 7.3 and in phase 26 in one change.

### O-10-3  MEDIUM   (section 7.27 items 3 and 4, with section 7.17; check 2)
- Problem: Items 3 and 4 test the stored `paid_on` and `granted_on`, and both exceptions need the stored values to judge "any other difference", but section 7.17 lists the row that `current_by_source_key` returns as `id` and `row_hash` plus only the extra fields of other entities, none for `facility_instalment` or `facility`. The writer therefore cannot read the tested values, though section 7.17 gave `account_hold` its `end_at` in exactly this way.
- Proof: docs/plan/phase-10.md:2440 "`CurrentRow` (returned by `current_by_source_key`) holds `id` and `row_hash`, ... and for `canon.account_hold` also `end_at`."; docs/plan/phase-10.md:2496 "when the stored `paid_on` is null and the mapped `paid_on` is set"; docs/plan/phase-10.md:2497 "when the stored `granted_on` is null and the mapped one is set".
- Smallest fix: Add to the `CurrentRow` sentence of section 7.17 "and for `facility_instalment` also `paid_on`, and for `facility` also `granted_on`", and say in item 3 how "any other difference" is found (the other mapped columns must be in the row type, or be compared in SQL).

### O-10-4  MEDIUM   (section 7.27 item 3; check 4 and check 6)
- Problem: The insert names `uq_facility_instalments_source` as its only target, but phase 26 gives the table a second unique constraint on `(facility_id, seq)`, so a row with a new source key for an instalment number the facility already holds (after a rescheduling, say) raises an error that `ON CONFLICT` on the named target does not catch. Section 3.7 step 5 handles only a missing partition, so the refusal has no code or reason.
- Proof: docs/plan/phase-26.md:1036 "CONSTRAINT uq_facility_instalments_seq UNIQUE (facility_id, seq)"; docs/plan/phase-10.md:2496 "Its insert target is `uq_facility_instalments_source`."; docs/plan/phase-10.md:2004-2005 "a second missing" (2004) "partition is `E_INTERNAL` and the batch fails." (2005)
- Smallest fix: Add to item 3 what a row does when `(facility_id, seq)` is held by another source key (for example a `RowConflict` with a named kind, not an insert), and name that kind in the phase 11 reason list.

### O-10-5  MEDIUM   (section 7.27 item 2, with section 7.20; check 3)
- Problem: Item 2 says phase 11 records the `parent_missing` conflict as a `row_conflict` quarantine row with reason `parent_missing`, but phase 11's list of reasons for writer conflicts has three entries and no `parent_missing`. Section 7.20 makes the same claim, which this block repeats.
- Proof: docs/plan/phase-11.md:802-803 "Each `RowConflict` becomes a `quarantine_items` row of kind `row_conflict` with reason `fact_changed_in_source` (...), `value_change` or `label_change` (dimensions)."; docs/plan/phase-10.md:2495 "and phase 11 records it as section 7.20 says."
- Smallest fix: Add `parent_missing` to the reason list in docs/plan/phase-11.md section 3.2 step 5 (line 802) in the same change, so that the claim of item 2 holds.

### O-10-6  LOW   (section 7.26 item 5; check 3)
- Problem: Item 5 cites DSL.md B.5 for the replacement by `typed`, but B.5 says only that under `dq_issue` the row loads and a count rises; the replacement is this block's own rule and runs in the executor (section 3.6 step 4), not in the writer as the item's heading says. A builder who follows B.5 literally would load the failing value and hit the CHECK.
- Proof: docs/DSL.md:615 "`on_fail`: `dq_issue` (default: the row loads and an `admin.dq_issues` count rises)"; docs/plan/phase-10.md:2486 "5. **Writer** (section 3.7): the compiler adds the validation rule"; docs/plan/phase-10.md:1953 "`MappingExecutor::apply(row) -> MappedRow | Rejected`: transforms in order, validation rules,".
- Smallest fix: Change "as DSL.md B.5 says for `dq_issue`" to "(this block's rule; DSL.md B.5 says only that the row loads and the count rises)", and change the heading "Writer (section 3.7)" to "Executor (section 3.6 step 4) and writer (section 3.7)".

### O-10-7  MEDIUM   (section 7.26 items 3 and 5; check 4)
- Problem: The column is `NOT NULL` and not `required`, and item 3 allows an `expr` over a bank column that can be null for some rows, but the block never says what a null becomes. Unless the writer drops null columns from its INSERT (the base does not say it does), PostgreSQL refuses the batch.
- Proof: docs/plan/phase-10.md:506 "capture_source      TEXT NOT NULL DEFAULT 'typed' CHECK (capture_source IN ('card', 'passport', 'typed', 'generated')),"; docs/plan/phase-10.md:2484 "or an `expr` over the source row when a column of the bank tells them apart."; docs/plan/phase-10.md:2486 "A row whose value fails it is written with `typed`"; docs/DSL.md:569 "`default` | A literal used only when the source value is null".
- Smallest fix: Add to item 5 "a null value is written with `typed`, as a `default` of "typed" on the field would do (DSL.md B.3)", so that the column never receives a null.

## Summary
- Changed base places checked: 2 (section 1.2 DDL, line 506, and the `party_name` classification row, lines 808-809). Both agree with block 7.26: the column text is identical, it stands after `source_field`, `capture_source` is named once in the classification row, and the grants, indexes and retention row are unchanged.
- Note at line 2471 (owner decisions paragraph): checked, no finding.
- Count (check 5): 32 names = 29 base (section 2 "closed enum of the 29 names"; section 3.7 step 1 lists 29) + 2 (section 7.3, "31 names") + 1 (this block). Agrees with phase 26 section 7.13 item 2 ("32nd name").
- Findings: HIGH 2 (O-10-1, O-10-2), MEDIUM 4 (O-10-3, O-10-4, O-10-5, O-10-7), LOW 1 (O-10-6).
- CHECKED: 7.26 checks 1 to 6: findings O-10-1, O-10-6, O-10-7.
- CHECKED: 7.27 checks 1 to 6: findings O-10-2, O-10-3, O-10-4, O-10-5.
- Checks 1 (wording, code spans, doubled words, item numbers) and 4 (SQL of the 7.26 column, the grants of phase 26 for the 7.27 writes) found nothing beyond the findings above.
