# Gap pass 1 run 3a: lead verdict (2026-10-10)

Draft: tools/agent/gap1-run3a.md (Haiku; 35 findings: 6 HIGH, 22 MEDIUM, 7 LOW; targets 05 9, 06 8, 07 2, 08 2,
09 6, 10 8). Read in full by the lead; every source line and target grepped.

## Per finding

- G1-05-01 ACCEPT. Sources confirmed (phase 06 section 3.3 coalesce key `coverage_grade`, reason `profile_run`; phase 10
  `drift_scan {source_id, run_id}`, coalesce `drift_scan:<source_id>`). Merged with UNCLEAR 2 (payload, DR-366).
- G1-05-02 ACCEPT HIGH. Object names confirmed in phase 09 section 3.3 (`HOLIDAY`, `CURRENCY`). Phase 05's hint schema
  (section 3.1 patch, section 3.6) gains the key `calendar`.
- G1-05-03 ACCEPT HIGH (phase 12 section 4.5). Hint schema key `control_totals`.
- G1-05-04 ACCEPT, decided: `StatementKind` = `Dictionary | Aggregate | Read`; phase 12 adds `ControlTotals` (DR-365).
  `LoadGovernor::for_source` and the phase 11 port methods recorded as later extensions.
- G1-05-05 ACCEPT (phase 10 section 4.5, DR-85). Text note: phase 10 moves the proof to `crates/sqlproof`.
- G1-05-06 ACCEPT; `deny_columns` already in phase 05. Hint key `code_suggestions` (phase 10).
- G1-05-07 ACCEPT (phase 30 section 4.5 item 2, DR-322).
- G1-05-08 ACCEPT as a note: `ingest_resume` and the `halted` resume belong to phase 11 (its job table).
- G1-05-09 ACCEPT as a note: the fixture additions are phase 11's.
- G1-06-01 ACCEPT HIGH, decided key form (DR-367). Phase 10 `AttributeService` is a different type (the bank-defined
  attribute registry, `admin.attribute_values`), not the catalogue.
- G1-06-02 ACCEPT as a note (phase 07 owns the extension).
- G1-06-03, G1-06-04 merged into one note (phase 16 section 4.5 owns the route changes).
- G1-06-05 ACCEPT; `MappingCompleteness` is phase 10's own name (phase 10 section 3.8), confirmed.
- G1-06-06, G1-06-07, G1-06-08 merged into one note on packs (phase 24 section 4.5 item 2, migration 0078 confirmed;
  phase 31 section 4.5 item 5).
- G1-07-01 ACCEPT HIGH; step wording rewritten against phase 07 section 3.2 (the per-file steps of `goaml_check`).
- G1-07-02 ACCEPT HIGH. NAME-01 IS defined (phase 21 section 3.x step 2 of the person split: warn, punctuation removed,
  `params.removed_count`); Haiku missed it. CTR-01 and CTR-02 decided `block` (DR-368). CONS-04 already exists (info).
  Phase 21 gets a correction block (CONS-04 not new; CTR codes arrive in a release, not by change request).
- G1-08-01 ACCEPT, decided `RailGroup` values from DESIGN-SYSTEM 8.2 (DR-369); D3 and T3 are already covered by the
  "every 8.1 screen is listed" rule of phase 08. Phase 10 gets a correction (T3 group `data`).
- G1-08-02 ACCEPT, decided: phase 08 builds the base `Name` component (it carries UX-25); phase 14 extends it (DR-373).
  Phase 14 gets a correction block.
- G1-09-01 ACCEPT HIGH, decided `ClockUnit` with the label `CalendarMonths` (DR-370). Phase 21 section is 3.18
  (confirmed).
- G1-09-02 ACCEPT, decided the eight `DEC_SCR_` code names (DR-372); phase 15 codes copied from phase 15 section 4.5.
- G1-09-03 ACCEPT (note).
- G1-09-04 ACCEPT; signature decided on `BankCalendarClock` (not the `BusinessClock` trait), returns `Result`.
- G1-09-05 ACCEPT; no role list of its own: the phase 08 rendering rule (operations the caller may call).
- G1-09-06 ACCEPT (note).
- G1-10-01, G1-10-02 ACCEPT (notes; phase 26 owns the DDL and enum change).
- G1-10-03 ACCEPT (note; phase 23 creates the table and the `akr_ingest` SELECT grant, confirmed at its grants).
- G1-10-04 ACCEPT (note; index names `idx_canon_txn_batch`, `idx_canon_txn_event_batch` from phase 16 migration 0056).
- G1-10-05, G1-10-06 ACCEPT (notes; phase 11 section 4.5).
- G1-10-07 ACCEPT, decided the key list (DR-374).
- G1-10-08 ACCEPT, decided: phase 10 creates `crates/names`; phase 14 extends it (DR-371). Closes phase 14 Open
  doubt "Stored forms in canon".

## UNCLEAR items of the draft's Summary

1. StatementKind: DR-365.
2. discovery_seal_report payload: exactly one of `{run_id}` or `{check_id}` (DR-366).
3. AttributeCatalogue key form: DR-367.
4. NAME-01 defined in phase 21; CTR-01/02 block; release, not change request: DR-368.
5. RailGroup: DR-369.
6. freeze.second_report: DR-370.
7. names crate order: DR-371.
8. Phase 07 "No notifications" already says the candidate stays on screens; phase 22 item 14 misreads it. No change
   here. CARRY to the run that targets phase 22 (wording).
9. Phase 22 item 9 citation (`PUT /admin/flags/...` is phase 03). CARRY to the run that targets phase 22.
10. Payment release status (phase 29 vs `canon.payment_message.status` free text). CARRY to the run that targets
    phase 29 (decide the value there, phase 10 correction then).
11. CONS-04 wording in phase 21: correction block applied now (with DR-368).

## DRs

DR-365 StatementKind; DR-366 seal payload; DR-367 catalogue keys; DR-368 rule codes; DR-369 RailGroup; DR-370
ClockUnit; DR-371 names crate; DR-372 DEC_SCR codes; DR-373 Name component; DR-374 address keys.
