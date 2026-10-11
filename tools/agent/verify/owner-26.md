# Owner-decision round, phase 26 (Haiku draft for lead review)

### O-26-1  HIGH   (section 1.1 part 6 and section 7.13 item 1, check 4)
- Problem: The UNIQUE on (facility_id, seq) is not the writer's insert target, so a second source key with the same facility and seq (a rescheduled schedule, for example) raises unique_violation and rolls back the whole batch; no rule in 7.13 or phase 10 says what the writer does instead.
- Proof: docs/plan/phase-26.md:145 "CONSTRAINT uq_facility_instalments_seq UNIQUE (facility_id, seq)"; docs/plan/phase-10.md:2496 "Its insert target is `uq_facility_instalments_source`."; docs/plan/phase-10.md:1976 "inside the batch transaction that also inserts the receipt (C-20)".
- Smallest fix: In 7.13 item 3 (and phase 10 section 7.27 item 3) add a rule that a row whose facility and seq belong to another source key is a RowConflict checked before the insert, or drop `uq_facility_instalments_seq` from both copies of the SQL.

### O-26-2  HIGH   (section 3.6 SCN-ISL-07 row and section 7.13 items 2, 5 and 8, check 2)
- Problem: 7.13 makes `paid_on` and `facility.granted_on` monitored, and phases 06 and 10 grade D any monitored attribute that a source leaves unmapped, so an agreement-only bank with no facility mapping would grade D (phase 16 gate G6 blocks D), not the A that item 5 promises. The B that the 3.6 cell and item 8 give a schedule source before its family is published follows from neither rule, because phase 06 gives optional needs no effect on the grade.
- Proof: docs/plan/phase-26.md:1060 "`paid_on` and `facility.granted_on` are `monitored`"; docs/plan/phase-26.md:1074 "so a bank with agreements only is not graded down for having no schedule mapping"; docs/plan/phase-06.md:1014-1015 "a monitored attribute with no published mapping for the source is `unmapped` and grades D"; docs/plan/phase-10.md:2032 "it grades D as `absent` does"; docs/plan/phase-06.md:564 "no effect on the grade"; docs/plan/phase-16.md:1077 "C and D block, S-47".
- Smallest fix: In 7.13 item 2 remove the `monitored` flag from `paid_on` and `facility.granted_on` (or state that phase 06 section 7.6 does not grade on them), then make the 3.6 readiness cell, item 5 and item 8 state the one grade that results.

### O-26-3  MEDIUM   (section 7.13 item 2 and the CHECK in section 1.1 part 6, check 4 and 6)
- Problem: `paid_amount` has no catalogue flag, although phase 10 section 3.8 makes the money columns of a fact `required` and the column is nullable, and nothing says it is null for a part payment, so a mapped `paid_amount` on a part payment violates the CHECK and the insert fails.
- Proof: docs/plan/phase-26.md:143 "CHECK (paid_amount IS NULL OR paid_on IS NOT NULL)"; docs/plan/phase-26.md:1060 "a part payment leaves `paid_on` null"; docs/plan/phase-10.md:2019 "the money and date columns of a fact".
- Smallest fix: In 7.13 item 2 state the flag of `paid_amount` and that it is set only together with `paid_on` (a part payment leaves both null), as the CHECK requires.

### O-26-4  MEDIUM   (section 6 check 2, and section 7.13 item 8, check 2)
- Problem: Check 2 names three tables only, so it does not cover `canon.facility_instalments` or `canon.facility.granted_on`, yet item 8 says the classification test of check 2 passes with the new rows.
- Proof: docs/plan/phase-26.md:859 "every column of `canon.islamic_deals`, `canon.deal_parties` and `kyc.possession_evidence`"; docs/plan/phase-26.md:1080 "the classification test of check 2 passes with the new rows".
- Smallest fix: In section 6 check 2 add `canon.facility_instalments` and `canon.facility.granted_on` to the tables and columns the query covers.

### O-26-5  MEDIUM   (section 6 check 5, check 5 and check 2)
- Problem: Check 5 verifies only "the two new entities", but this phase also adds `facility_instalment` and `facility.granted_on` to `canonical-model.v1.json` (phase 10 section 7.27 item 6), and no check in the block covers them.
- Proof: docs/plan/phase-26.md:865 "the two new entities match the migrated columns"; docs/plan/phase-10.md:2499 "`akr-ctl canon verify-catalogue` therefore passes before phase 26".
- Smallest fix: In section 6 check 5 change "the two new entities" to "the three new entities and `canon.facility.granted_on`".

### O-26-6  MEDIUM   (section 3.2 items 1 and 2, check 2)
- Problem: Section 3.2 says the pack adds two mapping families and two catalogue entries, but with `facility_instalment` (7.13 item 2) the phase adds three of each.
- Proof: docs/plan/phase-26.md:364 "Two mapping families are added"; docs/plan/phase-26.md:368 "`canonical-model.v1.json` gets two entries"; docs/plan/phase-10.md:2494 "Phase 26 adds `facility_instalment` at the end of `CanonicalEntity` (32 names".
- Smallest fix: Change "Two mapping families are added" to "Three mapping families are added", with `<source>.facility_instalment` in the list, and "gets two entries" to "gets three entries".

### O-26-7  MEDIUM   (section 1 introduction, line 21, and the trailer "What This Phase Does NOT Touch", check 2)
- Problem: Line 21 says `canon.product` is the only existing table altered, and the trailer says there is no change to `canon.facility`, but section 1.1 part 6 adds a column to `canon.facility` and grants UPDATE on it.
- Proof: docs/plan/phase-26.md:21 "The only existing table altered is `canon.product` (phase 10), and only its CHECK"; docs/plan/phase-26.md:1140 "no change to `canon.facility`, `canon.account`, `canon.party` or the customer tables"; docs/plan/phase-26.md:123 "ALTER TABLE canon.facility ADD COLUMN IF NOT EXISTS granted_on DATE;".
- Smallest fix: In line 21 name `canon.facility` (column `granted_on`) as a second altered table, and in line 1140 drop `canon.facility` from the "no change to" list and say that `granted_on` (section 7.13) is its only change.

### O-26-8  MEDIUM   (section 1.2 note, check 2)
- Problem: The note says the VALUES rows cover every column of "the two tables", but the classification insert now also covers `canon.facility_instalments` and the column `canon.facility.granted_on`.
- Proof: docs/plan/phase-26.md:191 "`VALUES` rows cover every column of the two tables"; docs/plan/phase-26.md:174 "('facility_instalments', 'internal', ARRAY['id', 'facility_id', 'seq'".
- Smallest fix: In line 191 change "the two tables" to "the three tables and the column `granted_on` of `canon.facility`".

### O-26-9  MEDIUM   (section 7.13 items 4 and 6, check 3)
- Problem: `not_evaluable_count` is used as an existing field of the alert evidence and asserted by fixture four, but no line of docs/plan defines or introduces it.
- Proof: docs/plan/phase-26.md:1071 "(`not_evaluable_count` in the measures)"; docs/plan/phase-26.md:1076 "(negative, `not_evaluable_count` 1)"; a grep of docs/plan for `not_evaluable` returns only these two lines.
- Smallest fix: In item 4 name the phase 16 object whose `measures` gain `not_evaluable_count` and what it counts, or use a name that already exists.

### O-26-10  LOW   (section 7.13 item 1, last sentence, check 5)
- Problem: The count "1 altered column list" does not match the header's "1 column added to `canon.facility`".
- Proof: docs/plan/phase-26.md:1058 "The phase now has 4 tables in 2 migrations and 1 altered column list"; docs/plan/phase-26.md:8 "1 column added to `canon.facility` (`granted_on`, section 7.13)".
- Smallest fix: In line 1058 change "1 altered column list (`canon.facility.granted_on`)" to "1 column added (`canon.facility.granted_on`)".

### O-26-11  LOW   (section 7.13 item 4, check 1)
- Problem: The sentence says the alert evidence both names the loan and gives only its count, so a reader cannot tell what the alert shows.
- Proof: docs/plan/phase-26.md:1071 "The alert evidence names which loan was not evaluable only by its count".
- Smallest fix: Change that sentence to "The alert evidence gives only the count of loans that were not evaluable (`not_evaluable_count`), so a gap in the mapping is seen."

### O-26-12  LOW   (section 7.13 item 3, check 1 and check 4)
- Problem: Item 3 says a later version that "only sets `paid_on`" updates the row, but phase 10 section 7.27 item 3 sets `paid_on`, `paid_amount` and `row_hash` together, so a version that also carries `paid_amount` is left unclear.
- Proof: docs/plan/phase-26.md:1062 "a later version that only sets `paid_on` updates the row in place"; docs/plan/phase-10.md:2496 "which sets the three columns".
- Smallest fix: In item 3 change "a later version that only sets `paid_on`" to "a later version that sets `paid_on` (and `paid_amount`, when mapped)".

## Summary

Base places checked: 8 changed (the diff hunks of phase 26 in base text: header lines 8 and 9; sections 1.1, 1.2 (two inserts), 1.5, 3.6, 7.9 and "## Decisions"). Unchanged base lines checked for conflict with the new text: 7 (lines 21, 191, 364, 368, 859, 865, 1140).

Findings by severity: HIGH 2, MEDIUM 7, LOW 3 (12 in total).

Per block:
- 7.13 items 1 to 8: findings O-26-1 (items 1 and 3), O-26-2 (items 2, 5 and 8), O-26-3 (item 2), O-26-4 (item 8), O-26-9 (items 4 and 6), O-26-10 (item 1), O-26-11 (item 4), O-26-12 (item 3). Item 7 has no finding.
- Owner paragraph above 7.13 (line 1003): CHECKED: 7.13 owner paragraph checks 1 to 6 ok.
- DR-419 paragraph in "## Decisions" (line 1110): CHECKED: checks 1 to 6 ok.
- 7.9 "Schedule sources" bullet (line 985): CHECKED: checks 1 to 6 ok (it says "replaced by section 7.13").
- SCN-ISL-07 row of 3.6 (line 512): O-26-2.
- Header lines 8 and 9: CHECKED: checks 1 to 6 ok (4 tables, 1 column, 3 canon tables of the pack).
- Section 1.1 part 6 (lines 120 to 153): O-26-1, O-26-3, O-26-7. The copy in 7.13 is identical (diff of the two line ranges is empty). Otherwise checked: PostgreSQL 17 syntax, UUID keys with no default, TIMESTAMPTZ times, foreign keys to `admin.sources`, `admin.mapping_versions` and `ref.currencies(code)` (all exist), the UNIQUE on the source key (the writer's target), and the grants against phase 10 section 7.27 items 3 and 4 (`paid_on`, `paid_amount`, `row_hash`; `granted_on`, `row_hash`).
- Section 1.2 classification and retention rows (lines 173 to 177 and 186): CHECKED: 17 columns in the CREATE TABLE, 17 names in the rows, each once; the retention row (R3, clock owner) is a valid class and clock under phase 23. The note at line 191 is O-26-8.
- Section 1.5 grant rows (lines 274 and 275): CHECKED: checks 1 to 6 ok.

Other requested checks:
- Entity count: `facility_instalment` is the 32nd name (29 base, 2 pack, 1 new), with the mapping family and the position agreeing with phase 10 sections 7.3 and 7.27; the flag gap is O-26-3.
- CHECK and UNIQUE of the new table: the source-key UNIQUE is the writer's target (ok); the seq UNIQUE is O-26-1; the paid_amount CHECK is O-26-3; the other CHECKs (seq, amount, paid_amount >= 0) are conditions on mapped values, not contradictions.
- P2 line check: no line outside "## Decisions" and "## Open doubts" states the facility leg of SCN-ISL-07 is P2 without a pointer to 7.13. Line 985 says "replaced by section 7.13"; line 1003 is a dated history note.
- Pointers to other phases: `ref.currencies(code)` is the primary key (phase 09 line 78); the `?` optional marker (phase 16 line 942, DSL A.3) and `repeat_months` (line 521) are used as defined; the retention row (R3, clock owner) and the owner rule match phase 23 (lines 576 and 605); the `akr_retention` SELECT and DELETE on the new table extend phase 23's grant list (lines 370 to 381) in the way the deal tables already do (1.1 part 5).
- Not reported (exempt, or outside the diff): Open doubts 2 (line 1115) still says SCN-ISL-07 uses `txn` rows (exempt). Section 3.2 item 2 points `verify-catalogue` to "section 6, check 2", but that is check 5 (base text, outside the diff).
