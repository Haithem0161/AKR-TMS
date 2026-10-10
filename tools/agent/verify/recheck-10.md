# Re-check, phase 10 (Haiku draft for lead review)

### R-10-1  HIGH   (fix F6)
- Problem: F6 makes `payment_message_party`, `payment_link` and `account_hold` facts, so the fact insert of section 3.7 step 5 applies to them: `INSERT ... ON CONFLICT (source_id, source_table, source_pk, <partition key>)`. None of the three tables has a unique index on its source key. PostgreSQL refuses an ON CONFLICT target that no unique index matches (SQLSTATE 42P10), so the writer fails for these entities. `account_hold` and `payment_link` are not partitioned, and the block gives them no conflict target. The same gap already exists for the base facts `txn_event`, `cash_event` and `balance_snapshot` (their UNIQUE constraints are on other columns), so step 5 needs a conflict target per table.
- Proof: docs/plan/phase-10.md:1995 "5. **Facts.** `INSERT ... ON CONFLICT (source_id, source_table, source_pk, <partition key>) DO NOTHING`"; docs/plan/phase-10.md:1983 "different hash on a **fact** (txn, txn_event, payment_message, payment_message_party, payment_link, account_hold,"; no source-key unique on the three: docs/plan/phase-10.md:1076 "PRIMARY KEY (id, received_at)," (payment_message_party), docs/plan/phase-10.md:1100 "CONSTRAINT uq_payment_link_pair UNIQUE (message_id, txn_id)," (payment_link), docs/plan/phase-10.md:694 "id UUID PRIMARY KEY," (account_hold).
- Smallest fix: name the conflict target of each of the three in 7.18 or 7.19 (for example a UNIQUE on (source_id, source_table, source_pk, received_at) for canon.payment_message_party in migration 0031, `ON CONFLICT (message_id, txn_id)` for canon.payment_link, and a UNIQUE on (source_id, source_table, source_pk) for canon.account_hold in migration 0030).

### R-10-2  MEDIUM   (fix F8)
- Problem: The new 7.16 makes `card_guard` a `RowFailure` code that phase 11 writes as a quarantine reason, so a card-guard hit in phase 11 is written as a quarantined row. Section 3.3 of the same phase says that in phase 11 a card-guard hit stops the field's load and opens a data incident. The two rules differ, and 7.16 neither says it replaces 3.3 nor says that both apply.
- Proof: docs/plan/phase-10.md:1867 "in phase 11 it stops the field's load and opens a data incident."; second quote docs/plan/phase-10.md:2428 "`RowFailure` is a closed enum whose codes phase 11 writes as quarantine reasons".
- Smallest fix: in 7.16 remove `card_guard` from the RowFailure list and state that a card-guard hit in phase 11 stops the field's load, as section 3.3 says (or amend 3.3 to say that the row is quarantined).

### R-10-3  MEDIUM   (fix F8)
- Problem: `MappedBatch` has one count, `unmapped_codes`, and `RowFailure` has no gate outcome. Section 3.6 step 5 keeps "quarantined by the gate" and "unknown codes" apart. A `txn_type` value with no entry is held by the gate (section 3.4 step 5), and phase 11 section 7.3 (owned by phase 12) writes such a held row as a quarantine row with reason `unmapped_code`. The types in 7.16 cannot carry the gate-held rows or their count, and the enum that 7.16 calls closed has no `unmapped_code`.
- Proof: docs/plan/phase-10.md:1953 "5. Accumulate the result: rows read, mapped, rejected by rule, quarantined by the gate, unknown codes"; docs/plan/phase-10.md:2427 "`MappedBatch { rows: Vec<MappedRow>, rejected: Vec<Rejected>, unmapped_codes: u64 }`"; second quote docs/plan/phase-11.md:1488 "(a row held for a transaction-type code that is not mapped: reason `unmapped_code` with field".
- Smallest fix: add a field for the gate-held rows (for example `gated: Vec<Rejected>`, reason `unmapped_code`) to `MappedBatch` in 7.16 and say that `unmapped_codes` counts unknown values only.

### R-10-4  MEDIUM   (fix F9)
- Problem: The 7.17 CurrentRow sentence gives every CurrentRow `recorded_from` and `valid_from`, and says the same type serves `canon.payment_message` and `canon.account_hold`. Neither table has those columns: canon.payment_message (docs/plan/phase-10.md lines 1026-1054) has `received_at`, and canon.account_hold (lines 693-712) has `start_at` and `end_at`. In canon, the bi-temporal columns exist only on party, party_relationship, customer, account and account_party_role. The base trait comment says current_by_source_key filters `recorded_to IS NULL`, which a developer would copy for facts and which fails on those tables. The block does not say how a fact row is found.
- Proof: docs/plan/phase-10.md:2432 "`CurrentRow` (returned by `current_by_source_key`) holds `id`, `recorded_from`, `valid_from` and `row_hash`"; second quote docs/plan/phase-10.md:1678 "bi-temporal: recorded_to IS NULL".
- Smallest fix: in 7.17 replace "holds `id`, `recorded_from`, `valid_from` and `row_hash`" with "holds `id` and `row_hash`, and `recorded_from` and `valid_from` for a bi-temporal entity; a fact row is read by source key with no `recorded_to` filter".

### R-10-5  MEDIUM   (fix F16)
- Problem: Research file 22 section 6.4 defines the Arabic pipeline as steps N1 to N11 and says the folded form uses all steps. Phase 14 section 3.1 also lists N1 to N11. The 7.23 block cites "N1 to N6" for the folded form, so a developer who implements N1 to N6 would leave out N7 (digits and punctuation) and the later steps.
- Proof: docs/plan/phase-10.md:2455 "`folded` is `FoldArabic` run through all of its steps (research file 22 section 6.4, N1 to N6)."; second quote docs/plan/phase-14.md:908 "| N11 | Emit typed parts (`PartRole`), the four forms and the flags |".
- Smallest fix: in 7.23 change "N1 to N6" to "N1 to N11".

### R-10-6  LOW   (fix F18)
- Problem: The NEW text puts backticks inside a code span. The span that opens before `valid_from` closes at the backtick before `admin.batches.business_to`, so that name renders as plain text, and the next span runs from its closing backtick to the `)` on line 2412. The formula is split into code, plain text and code. The sentence is readable, but the REP check does not allow a broken code span.
- Proof: docs/plan/phase-10.md:2411 "Step 4 therefore sets the new version's `valid_from = GREATEST(<the batch's business date of step 4, phase 11's `admin.batches.business_to`>,"
- Smallest fix: write `admin.batches.business_to` without backticks in line 2411, so that the code span runs from `valid_from` to the `)` at the end of line 2412.

## Summary
- Fixes checked: 20 (F1 to F20).
- Findings: HIGH 1, MEDIUM 4, LOW 1 (6 in total).
CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 R-10-1, F7 ok, F8 R-10-2, R-10-3, F9 R-10-4, F10 ok, F11 ok, F12 ok, F13 ok, F14 ok, F15 ok, F16 R-10-5, F17 ok, F18 R-10-6, F19 ok, F20 ok
