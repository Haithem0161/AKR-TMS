#!/usr/bin/env python3
"""Owner round 1 fixes: phase 10 (O-10-1..7), phase 26 (O-26-1..12), and the two base corrections in phases 11 and 12
(O-10-5), plus DATA-MODEL. Lead find L-3: the three column rules test mapped values, not the landed hash.
Verdicts: tools/agent/owner-verdict.md."""
import sys
sys.path.insert(0, "/home/haithem/Projects/AKR-TMS/tools/agent")
from ofix import Fixer, ROOT

f = Fixer(1)
P10, P26, P11, P12, DM = ("docs/plan/phase-10.md", "docs/plan/phase-26.md", "docs/plan/phase-11.md",
                          "docs/plan/phase-12.md", "docs/DATA-MODEL.md")

# ---------------------------------------------------------------- phase 10, section 7.26 (capture_source)
f.fix("O-10-7a", P10,
      "Grants, indexes and the retention row are unchanged.",
      "Indexes and the retention row are unchanged; one column grant is added (item 5).")
f.fix("O-10-7b", P10,
      "A mapping that leaves it out writes `typed` (the column default), which is what phase 08 showed",
      "A mapping that leaves it out writes `typed` (item 5), which is what phase 08 showed")
f.fix("O-10-1/6/7", P10,
      "5. **Writer** (section 3.7): the compiler adds the validation rule `enum(capture_source, [card, passport, typed, generated])` with `on_fail: dq_issue` to every `party_name` mapping that supplies the field. A row whose value fails it is written with `typed` and counted as a data-quality issue, as DSL.md B.5 says for `dq_issue`. A change of `capture_source` changes the `row_hash`, so the row is replaced as any changed child row is (step 6).",
      "5. **Executor and writer** (section 3.6 step 4 and section 3.7). The compiler adds the validation rule `enum(capture_source, [card, passport, typed, generated])` with `on_fail: dq_issue` to every `party_name` mapping that supplies the field. This block's rule for a value that fails it: the executor puts `typed` in the mapped row and counts the data-quality issue. (DSL.md B.5 alone says only that the row loads and the count rises; the failing value would then meet the column CHECK.) A mapped row with no value for the field, because the mapping leaves it out or its `expr` gives null, is written with `typed`: the writer always supplies the column and does not rely on the column default. `row_hash` is the hash of the landed source row (phase 11 section 3.2 step 2), so a mapping version that changes only this value leaves the hash as it is and step 3 writes nothing. The label is therefore tested on the mapped value: when the hash is equal and the stored `capture_source` of the current row differs from the mapped one, the writer calls `CanonRepository::set_capture_source(tx, id, capture_source)`, which sets that one column; when the hash differs, the row is replaced as step 6 says and the new row carries the mapped value. `CurrentRow` (section 7.17) holds `capture_source` for `canon.party_name`. `CanonRepository` gains `async fn set_capture_source(&self, tx: &mut Tx, id: Uuid, capture_source: &str) -> Result<u64, RepoError>;`, and migration `0030` (section 1.2) gains `GRANT UPDATE (capture_source) ON canon.party_name TO akr_ingest;`. A name already stored takes a new label when its row is mapped again: at the next load of its source row, or in the apply of a mapping replay (phase 11 section 3.8).")
f.fix("O-10-1b", P10,
      "No new route, event or action: the column travels with the name rows through the mapping change path (`mapping.*` events and the `mapping` change target of section 3.9).",
      "No new route, event or action: the value is set by the mapping, and a mapping changes only through its change path (`mapping.*` events and the `mapping` change target of section 3.9).")
f.fix("O-10-7c", P10,
      "the literal `'scan'` fails `validate` with `E_VALIDATION` at `/fields/capture_source`; `akr-ctl canon verify-catalogue` passes with the new column.",
      "the literal `'scan'` fails `validate` with `E_VALIDATION` at `/fields/capture_source`; a row whose `expr` gives null writes `typed`; a row whose `expr` reads `scan` from a source column writes `typed` and counts one data-quality issue; a stored name with `typed`, mapped again from the same landed row by a version that gives `passport`, has `passport` afterwards and no second row; `akr-ctl canon verify-catalogue` passes with the new column.")
f.fix("O-10-7d", P10,
      "GRANT UPDATE (superseded_at) ON canon.party_name, canon.party_identifier, canon.party_contact TO akr_ingest;\n",
      "GRANT UPDATE (superseded_at) ON canon.party_name, canon.party_identifier, canon.party_contact TO akr_ingest;\nGRANT UPDATE (capture_source) ON canon.party_name TO akr_ingest;   -- the label of a name, section 7.26 item 5\n")
# section 7.17: pointer for CurrentRow
f.fix("O-10-3a", P10,
      "and for `canon.account_hold` also `end_at`. `CanonRow` (taken by `insert_rows`)",
      "and for `canon.account_hold` also `end_at`; sections 7.26 and 7.27 add `capture_source` for `canon.party_name`, `granted_on` for `canon.facility`, and `paid_on` and `paid_amount` for `canon.facility_instalments`. `CanonRow` (taken by `insert_rows`)")
# ---------------------------------------------------------------- phase 10, section 7.27 (instalments)
f.fix("O-10-2", P10,
      "and registers the mapping family `<source>.facility_instalment`. In the dependency order",
      "and registers the mapping family `<source>.facility_instalment`. The catalogue entry of the entity names its table, `canon.facility_instalments` (an entry holds the table, the columns, the classification and the nullability: phase 26 section 3.2 item 2), as the entry of `islamic_deal` names `canon.islamic_deals`. In the dependency order")
f.fix("O-10-5a", P10,
      "a `RowConflict` of kind `parent_missing` is returned, and phase 11 records it as section 7.20 says.",
      "a `RowConflict` of kind `parent_missing` is returned, and phase 11 records it as section 7.20 says (reason `parent_missing`, phase 11 section 3.2 step 5).")
f.fix("O-10-3/4, O-26-3", P10,
      "3. **Fact rule** (section 3.7 steps 3 and 5). `facility_instalment` is a fact. Its insert target is `uq_facility_instalments_source`. One exception to the fact rule, like the end of a hold (section 7.19): when the stored `paid_on` is null and the mapped `paid_on` is set, the writer calls `CanonRepository::pay_instalment(tx, id, paid_on, paid_amount, row_hash)`, which sets the three columns; no conflict is returned. Any other difference is a `RowConflict`.",
      "3. **Fact rule** (section 3.7 steps 3 and 5). `facility_instalment` is a fact. Its insert target is `uq_facility_instalments_source`, the only unique constraint of the table beside the primary key, so a second source key for an instalment number the facility already holds is inserted like any new row. One exception to the fact rule, like the end of a hold (section 7.19): an instalment is open while its stored `paid_on` is null. For an open instalment whose mapped `paid_on` or `paid_amount` differs from the stored value (`IS DISTINCT FROM`), the writer calls `CanonRepository::pay_instalment(tx, id, paid_on, paid_amount, row_hash)`, which sets the three columns where `paid_on IS NULL`; `paid_on` stays null for a part payment. No conflict is returned, and the row counts as written for this batch: a difference in `seq`, `due_on`, `amount` or `currency` that arrives in the same version is not written and not reported. The test is on the mapped values, not on the hash, so it also runs when the hash is equal (the landed row is the same and the mapping changed, as in the apply of phase 11 section 3.8). In every other case step 3 stands: a different hash on a paid instalment, or on an open one whose two payment columns did not move, is the `RowConflict` of a fact. `CurrentRow` holds `paid_on` and `paid_amount` for this entity.")
f.fix("O-10-3b", P10,
      "4. **`granted_on` of a facility.** `facility` stays a dimension. One exception to the dimension rule of step 3: when the stored `granted_on` is null and the mapped one is set, the writer calls `CanonRepository::set_granted_on(tx, id, granted_on, row_hash)`, which sets the two columns; no conflict is returned. Any other difference is a `RowConflict` as before. The grants for both updates are in phase 26's migration `0083`.",
      "4. **`granted_on` of a facility.** `facility` stays a dimension. One exception to the dimension rule of step 3: when the stored `granted_on` is null and the mapped one is set, the writer calls `CanonRepository::set_granted_on(tx, id, granted_on, row_hash)`, which sets the two columns where `granted_on IS NULL`. No conflict is returned, and the row counts as written for this batch: a difference in another column that arrives in the same version is not written and not reported. As in item 3 the test is on the mapped value, not on the hash, so a mapping version that adds the field reaches the facilities already stored when they are mapped again. In every other case step 3 stands, and a stored `granted_on` is never changed. `CurrentRow` holds `granted_on` for `canon.facility`. The grants for both updates are in phase 26's migration `0083`.")
f.fix("O-26-3b", P10,
      "`async fn pay_instalment(&self, tx: &mut Tx, id: Uuid, paid_on: NaiveDate, paid_amount: Option<Decimal>, row_hash: &[u8]) -> Result<u64, RepoError>;`",
      "`async fn pay_instalment(&self, tx: &mut Tx, id: Uuid, paid_on: Option<NaiveDate>, paid_amount: Option<Decimal>, row_hash: &[u8]) -> Result<u64, RepoError>;`")
f.fix("O-10-3c", P10,
      "a second version with `paid_on` set updates the row and returns no conflict; a second version with a different `amount` returns a `RowConflict`; a facility row that arrives again with `granted_on` set updates only `granted_on` and `row_hash`.",
      "a second version with `paid_on` set updates the row and returns no conflict; a second version with only `paid_amount` set (a part payment) updates `paid_amount` and leaves `paid_on` null; a second version of an open instalment with only a different `amount` returns a `RowConflict`, and so does any second version of a paid instalment; the same landed row mapped by a version that now gives `paid_on` updates the row; a second source key for an instalment number the facility already holds is inserted; a facility row that arrives again with `granted_on` set updates only `granted_on` and `row_hash`, and a later version with another `granted_on` leaves the stored date and returns the dimension `RowConflict`.")

# ---------------------------------------------------------------- phase 11 and phase 12 (O-10-5)
f.fix("O-10-5b", P11,
      "changed; the new version stays in raw), `value_change` or `label_change` (dimensions). Each",
      "changed; the new version stays in raw), `value_change` or `label_change` (dimensions), or `parent_missing` (a row\n   whose parent key has no `admin.xref` row yet, phase 10 sections 7.20 and 7.27; the row is not written). Each")
d13 = [l for l in open(ROOT + "/" + P11, encoding="utf-8").read().split("\n") if l.startswith("13. The outbox event `dq.stale`")]
assert len(d13) == 1
f.fix("L-3-doubt", P11, d13[0] + "\n",
      d13[0] + "\n14. The apply of section 3.8 runs the writer over the landed rows, and the writer's guard (phase 10 section 3.7 step 3) compares the hash of the landed row, which a mapping change does not move. Read literally, an apply finds every row unchanged. The writer needs a comparison of mapped values in the apply, or a stored hash that covers the mapped row; the choice is made when phases 10 and 11 are built. The three column rules of phase 10 sections 7.26 and 7.27 (`capture_source`, `paid_on` with `paid_amount`, `granted_on`) do not wait for it: they test mapped values. Found in the owner-decision round (2026-10-11).\n")
f.fix("O-10-5c", P12,
      "   (medium). Rule failures -> `validation_rule`,",
      "   (medium); `parent_missing` (a row whose parent key has not landed, phase 10 sections 7.20 and 7.27) ->\n   `unresolved_pointer` (medium). Rule failures -> `validation_rule`,")
f.fix("O-10-5d", P12,
      "(`fact_changed_in_source`, `value_change`, `label_change`; section 3.3).",
      "(`fact_changed_in_source`, `value_change`, `label_change`, `parent_missing`; section 3.3).")

# ---------------------------------------------------------------- phase 26: SQL, both copies (section 1.1 and 7.13)
f.fix("O-26-3a", P26,
      "    paid_amount         NUMERIC(28,6) CHECK (paid_amount IS NULL OR paid_amount >= 0),\n",
      "    paid_amount         NUMERIC(28,6) CHECK (paid_amount IS NULL OR paid_amount >= 0),   -- paid so far, when the source gives it\n", n=2)
f.fix("O-26-1a", P26,
      "    CHECK (paid_amount IS NULL OR paid_on IS NOT NULL),\n    CONSTRAINT uq_facility_instalments_source UNIQUE (source_id, source_table, source_pk),\n    CONSTRAINT uq_facility_instalments_seq UNIQUE (facility_id, seq)\n);\n",
      "    CONSTRAINT uq_facility_instalments_source UNIQUE (source_id, source_table, source_pk)\n);\n-- (facility_id, seq) is an index, not a unique constraint: a rescheduled loan can give a number under a second source key.\nCREATE INDEX IF NOT EXISTS idx_facility_instalments_facility ON canon.facility_instalments (facility_id, seq);\n", n=2)
# 7.13 item 1 closing paragraph
f.fix("O-26-10", P26,
      "The phase now has 4 tables in 2 migrations and 1 altered column list (`canon.facility.granted_on`).",
      "The phase now has 4 tables in 2 migrations and 1 column added (`canon.facility.granted_on`). The source key is the table's only unique constraint: the fact write of phase 10 has one conflict target, and a second unique rule on `(facility_id, seq)` would fail the batch of a rescheduled loan whose new rows repeat the numbers. `paid_amount` may be set while `paid_on` is null (a part payment), so no CHECK ties the two.")
# item 2
f.fix("O-26-3c", P26,
      "`paid_on` and `facility.granted_on` are `monitored` (inputs of SCN-ISL-07) and not `required`. One source row is one instalment. The mapping sets `paid_on` only when the source shows the instalment paid in full (a `filter` or an `expr` over the source's paid flag or amounts); a part payment leaves `paid_on` null.",
      "`paid_on` and `facility.granted_on` are `monitored` (inputs of SCN-ISL-07) and not `required`; `paid_amount` is neither: it is the amount paid so far, kept when the source gives it and read by no scenario. One source row is one instalment. The mapping sets `paid_on` only when the source shows the instalment paid in full (an `expr` over the source's paid flag or amounts); a part payment leaves `paid_on` null and may set `paid_amount`. Rescheduling: when the source keeps the rows that a new schedule replaced, the mapping leaves them out with a `filter`, so that an old unpaid row does not hold the facility unpaid. A replaced row that was loaded before the rescheduling stays in `canon`, since facts are not deleted; Open doubts 24 names this limit.")
# item 3
f.fix("O-26-12", P26,
      "the row is written as a fact, and a later version that only sets `paid_on` updates the row in place.",
      "the row is written as a fact, and while an instalment is unpaid a later version may set its `paid_on` and `paid_amount` in place; a paid instalment does not change.")
# item 4
f.fix("O-26-9/11", P26,
      "The alert evidence names which loan was not evaluable only by its count (`not_evaluable_count` in the measures), so a gap in the mapping is seen.",
      "The SCN-ISL-07 document gains the measure alias `not_evaluable_count`, an integer: the number of the customer's facilities that were not evaluable in the evaluation. It stands in the `measures` of the alert evidence (phase 16). The evidence gives this count only, not which facility, so a gap in the mapping is seen.")
# item 5 and the cell of section 3.6
f.fix("O-26-2a", P26,
      "The readiness cell reads: readiness A for an agreement source; for a schedule source A once `facility.granted_on` and the family `<source>.facility_instalment` are published, and B until then. The facility inputs are optional inputs (DSL A.3), so a bank with agreements only is not graded down for having no schedule mapping.",
      "The readiness cell reads: readiness A (agreement source) or B (schedule source), as for SCN-ISL-06. The three inputs marked `?` are optional inputs (DSL.md, CanonicalRef), and phase 06 section 3.2 rule 4 gives an optional input no effect on the grade. While `facility.granted_on` or the family `<source>.facility_instalment` is not published, the three are listed in the `reasons` of the readiness result and the facility legs are not evaluable (item 4); once they are published the facility legs run. A bank with agreements only is therefore not graded down for having no schedule mapping.")
f.fix("O-26-2b", P26,
      "| readiness A (agreement source); for a schedule source A once `facility.granted_on` and the family `<source>.facility_instalment` are published, B until then (section 7.13) |",
      "| readiness A (agreement source) or B (schedule source); the inputs marked `?` are optional and do not change the grade: until they are published the facility legs are not evaluable (section 7.13) |")
# item 6: the fixture wording follows item 4
# item 8 tests
f.fix("O-26-2c", P26,
      "the four fixtures of item 6 pass; a schedule source with the family published grades A and the same source without it grades B for SCN-ISL-07.",
      "a second row with a new source key and a `(facility_id, seq)` that the table already holds is inserted; a row with `paid_amount` set and `paid_on` null is accepted; the four fixtures of item 6 pass; for SCN-ISL-07 a source without the family `<source>.facility_instalment` has the three optional inputs in the `reasons` of its readiness result and the same grade as the same source with the family published.")
# ---------------------------------------------------------------- phase 26: base text
f.fix("O-26-7a", P26,
      "The only existing table altered is `canon.product` (phase 10), and only its CHECK, as stated in 1.1.",
      "The existing tables altered are `canon.product` (phase 10), only in its CHECK, as stated in 1.1, and `canon.facility` (phase 10), which gains the column `granted_on` (section 7.13).")
f.fix("O-26-8", P26,
      "`VALUES` rows cover every column of the two tables; a test (section 6, check 2)",
      "`VALUES` rows cover every column of the three tables and the column `granted_on` of `canon.facility`; a test (section 6, check 2)")
f.fix("O-26-6a", P26,
      "1. Two mapping families are added: `<source>.islamic_deal` (target entity `islamic_deal`) and `<source>.deal_party` (target entity\n   `deal_party`), one active family per source and entity (phase 10).",
      "1. Two mapping families are added: `<source>.islamic_deal` (target entity `islamic_deal`) and `<source>.deal_party` (target entity\n   `deal_party`), one active family per source and entity (phase 10). Section 7.13 adds a third, `<source>.facility_instalment`.")
f.fix("O-26-6b", P26,
      "   `canonical-model.v1.json` gets two entries (table, columns, classification, nullability).",
      "   `canonical-model.v1.json` gets two entries (table, columns, classification, nullability); section 7.13 adds a third entity,\n   `facility_instalment`, with its entry, and the column `facility.granted_on`.")
f.fix("O-26-4", P26,
      "2. `cargo check`; then the classification check: every column of `canon.islamic_deals`, `canon.deal_parties` and `kyc.possession_evidence`\n   has a `admin.column_classification` row",
      "2. `cargo check`; then the classification check: every column of `canon.islamic_deals`, `canon.deal_parties`, `canon.facility_instalments` and\n   `kyc.possession_evidence`, and the column `canon.facility.granted_on`, has a `admin.column_classification` row")
f.fix("O-26-5", P26,
      "5. `akr-ctl canon verify-catalogue`: the two new entities match the migrated columns.",
      "5. `akr-ctl canon verify-catalogue`: the three new entities (with `facility_instalment` of section 7.13) and `facility.granted_on` match the migrated columns.")
f.fix("O-26-7b", P26,
      "beyond the widened CHECK; no change to `canon.facility`, `canon.account`, `canon.party` or the customer tables;",
      "beyond the widened CHECK; no change to `canon.facility` beyond the column `granted_on` of section 7.13; no change to `canon.account`, `canon.party` or the customer tables;")
# open doubt 24
d23 = [l for l in open(ROOT + "/" + P26, encoding="utf-8").read().split("\n") if l.startswith("23. **Routes that depend on the flag.**")]
assert len(d23) == 1
f.fix("O-26-1b", P26, d23[0] + "\n",
      d23[0] + "\n24. **Rescheduled and cancelled instalments** (section 7.13; found in the owner-decision round, 2026-10-11). No schedule table of a bank has been studied. A source that changes an unpaid instalment in place is handled: the payment still lands, and a changed due date or amount is recorded as a conflict (phase 10 section 7.27 item 3). A source that replaces a schedule by new rows, or deletes rows, leaves the rows already loaded in `canon` as unpaid, so the unpaid leg of SCN-ISL-07 can fire for that facility and its repeat leg never sees it repaid. Until a schedule source is studied, the bank's mapping leaves replaced rows out from the first load; a column that marks an instalment as replaced is added when a source needs it.\n")

# ---------------------------------------------------------------- DATA-MODEL
f.fix("O-26-1c", DM,
      "paid_amount null; unique (facility_id, seq); one source row is one instalment;",
      "paid_amount null (the amount paid so far, when the source gives it); index on (facility_id, seq), not unique (a rescheduled loan can give a number under a second source key); one source row is one instalment;")
f.done()
