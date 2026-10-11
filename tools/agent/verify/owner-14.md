# Owner-decision round, phase 14 (Haiku draft for lead review)

### O-14-1  MEDIUM   (section 4.2, checks 2 and 5)
- Problem: Section 4.2 still gives 25 OpenAPI operations, 17 of them `screening`. With the two routes of section 7.19 the phase has 27 operations and 19 `screening` operations, as section 6 item 10 and the 7.19 verification state.
- Proof: docs/plan/phase-14.md:1225 "tags `lists` (8 operations) and `screening` (17); 25 in all" and docs/plan/phase-14.md:1290 "Swagger lists 27 operations with `x-akr-action`".
- Smallest fix: In section 4.2 change "`screening` (17); 25 in all" to "`screening` (19); 27 in all".

### O-14-2  MEDIUM   (section 6 item 1, check 5)
- Problem: Section 6 item 1 checks "the ten new tables". Section 1 creates ten, and section 7.19 adds an eleventh, `alert.harness_cases`, so the schema check misses it.
- Proof: docs/plan/phase-14.md:1270 "the phase 00 schema checks pass for the ten new tables" and docs/plan/phase-14.md:1457 "CREATE TABLE IF NOT EXISTS alert.harness_cases (".
- Smallest fix: Change "for the ten new tables" to "for the eleven new tables (the ten of section 1 and `alert.harness_cases` of section 7.19)".

### O-14-3  MEDIUM   (section 4.3, check 2)
- Problem: Section 4.3 says it is a copy of section 1.5 and lists the ungoverned keys. Section 7.19 adds the ungoverned key `screening.harness_cases_max` to the migration, and the list omits it.
- Proof: docs/plan/phase-14.md:1227 "### 4.3 Settings and their registry (copy of section 1.5"; docs/plan/phase-14.md:1229 "Ungoverned: `screening.review_clock_hours`"; docs/plan/phase-14.md:1524 "('screening.harness_cases_max', '500', 'integer', NULL, FALSE,".
- Smallest fix: Add `screening.harness_cases_max` to the "Ungoverned:" list of section 4.3.

### O-14-4  MEDIUM   (section 4.5, check 2)
- Problem: Section 4.5 says phase 04 gets three change targets. Section 7.19 adds a fourth, `HarnessCaseTarget`, which phase 04's target registry must hold.
- Proof: docs/plan/phase-14.md:1246 "**Phase 04**: three change targets (`list_setting`, `screening_policy`, `good_guy_rule`)" and docs/plan/phase-14.md:1557 "the fourth change target `HarnessCaseTarget`".
- Smallest fix: Change the phase 04 line of section 4.5 to "four change targets (`list_setting`, `screening_policy`, `good_guy_rule` and `harness_case` of section 7.19)".

### O-14-5  MEDIUM   (section 7.16 against 7.19 item 3, check 2)
- Problem: Section 7.16 says `IdentifierInput` is never stored. Section 7.19 keeps the raw identifiers sealed in `query_enc`, which is storage. Item 3 says "everywhere else (section 7.16)", but 7.16 itself carries no exception, so a reader of 7.16 gets the old rule.
- Proof: docs/plan/phase-14.md:1433 "`IdentifierInput` is the raw form of `Identifiers` (section 2.1), hashed on receipt and never stored" and docs/plan/phase-14.md:1540 "a case is the one place that keeps it, sealed inside `query_enc`".
- Smallest fix: At the end of the `IdentifierInput` bullet of section 7.16 add "except the sealed query of a harness case (section 7.19)".

### O-14-6  MEDIUM   (7.19 item 11 frontend, read with items 3 and 6, check 6)
- Problem: The S3 "Bank cases" tab shows the result of the latest run for each case, including "Entry no longer on the list". The API cannot supply it: `HarnessCaseView` has no run result, `trial` is never updated (the UPDATE grant leaves it out), and `report.bank_cases` keeps only `unmet_ids`, so an absent case cannot be told from a met one.
- Proof: docs/plan/phase-14.md:1598 "the state and the result of the latest run in words"; docs/plan/phase-14.md:1588 "`report.bank_cases = {cases, met, unmet, absent, unmet_ids: [case ids]}`"; docs/plan/phase-14.md:1502 "GRANT UPDATE (state, retired_by_request, activated_at, closed_at) ON alert.harness_cases".
- Smallest fix: Store per-case results in the report (for example `results: [{case_id, result}]` with met, unmet or absent) and add the latest result to `HarnessCaseView`, or remove the per-case result from the tab.

### O-14-7  MEDIUM   (7.19 item 13, curl check, check 6)
- Problem: The curl check proposes a case "with a synthetic entry key" but names no `list_code` and no entry key. The suite loads a fixture version only for `iq_national`, and a proposal needs an active version of the named list (else 422 `unknown_list_code`), so the check cannot be run as written.
- Proof: docs/plan/phase-14.md:1605 "with a synthetic entry key, `expect: "hit"`"; docs/plan/phase-14.md:1275 "with a synthetic fixture version of `iq_national` loaded"; docs/plan/phase-14.md:1562 "the list of `list_code` is enabled and has an active version (else 422 `unknown_list_code`)".
- Smallest fix: Name the body `list_code: "iq_national"` and an `entry_key` that the active fixture version holds, and say that the check runs after that version is active.

### O-14-8  LOW   (7.19 item 4 step 4, check 3)
- Problem: Step 4 hashes the identifiers "as a search does (section 3.12 step 3)". Section 3.12 step 3 normalises the name only. Section 7.16 defines the hashing on receipt.
- Proof: docs/plan/phase-14.md:1563 "hash the identifiers as a search does (section 3.12 step 3)" and docs/plan/phase-14.md:1153 "3. Normalise the query with the names crate; a thin query returns `not_assessable`, never `clear`."
- Smallest fix: Replace "(section 3.12 step 3)" in step 4 with "(section 7.16)".

### O-14-9  LOW   (7.19 item 10, check 6)
- Problem: Item 10 names CH among the roles that "reach neither" action, but the bundle tests list CFG, CHK, IA and AS and leave CH out.
- Proof: docs/plan/phase-14.md:1596 "CFG, CHK, CH, IA and AS reach neither" and, on the same line, "Bundle tests: CFG, CHK, IA and AS are refused both actions".
- Smallest fix: Change the bundle-test list to "CFG, CHK, CH, IA and AS".

### O-14-10  LOW   (7.19 item 12, check 6)
- Problem: The tipping-off review says the change request holds the list code, the expectation and the trial result. The step 7 body also holds `entry_key`, `query_enc` and `query_hk`, which phase 04 keeps in `proposed`. Phase 04 also stores the free-text `reason_text` (plain text, up to 2000 characters) on the same request, which CHK reads. The review says nothing about either.
- Proof: docs/plan/phase-14.md:1600 "The change request, which CHK reads, and the audit events hold the list code, the expectation and the trial result."; docs/plan/phase-14.md:1566 "`{op: "add", case_id, list_code, entry_key, expect, trial, query_enc, query_hk}`"; docs/plan/phase-04.md:129 "proposed            JSONB NOT NULL"; docs/plan/phase-04.md:134 "reason_text         TEXT NOT NULL CHECK (length(btrim(reason_text)) BETWEEN 1 AND 2000)".
- Smallest fix: Add to item 12 that the body holds `entry_key`, `query_enc` and `query_hk`, and state the rule for `reason_text` (SS and M write no name in it, or the text is sealed).

### O-14-11  LOW   (7.19 item 8, check 1)
- Problem: Item 8 scores "each list the change touches". For `screening_policy` the base target covers the three global settings and names no lists, so the lists its dry run scores are not stated.
- Proof: docs/plan/phase-14.md:1592 "The dry run also scores the active `hit` cases of each list the change touches" and docs/plan/phase-14.md:1072 "| `screening_policy` | `screening_policy:<key>` (`doubt_policy`, `priority_map`, `harness_gate`) | the three governed JSON settings |".
- Smallest fix: Say which lists: every enabled list for `screening_policy`, and the list of the key for `list_setting`.

### O-14-12  LOW   (7.19 item 1, check 1)
- Problem: "as for the alert tables" is not accurate. Section 1.4 grants SELECT to `akr_readonly` on `alert.screening_runs` and `alert.harness_runs`, while `screening_alerts`, `screening_decisions`, `pair_memory` and `good_guy_rules` get no such grant.
- Proof: docs/plan/phase-14.md:1529 "`akr_readonly` gets no grant, as for the alert tables." and docs/plan/phase-14.md:447 "GRANT SELECT ON ref.lists, ref.list_versions, alert.screening_runs, alert.harness_runs TO akr_readonly;".
- Smallest fix: Replace "as for the alert tables" with "as for `screening_alerts`, `screening_decisions`, `pair_memory` and `good_guy_rules`".

### O-14-13  LOW   (7.19 item 13, migration check, check 6)
- Problem: The DELETE check expects the trigger message. `akr_app` has no DELETE privilege, so as `akr_app` the statement fails with "permission denied" before the trigger runs. Only a role with DELETE reaches the message. The check names no role. Section 6 item 2 has the same pattern.
- Proof: docs/plan/phase-14.md:1603 "`DELETE` fails with `known-answer cases are never deleted`"; docs/plan/phase-14.md:1501 "GRANT SELECT, INSERT ON alert.harness_cases TO akr_app;"; docs/plan/phase-14.md:1271 "Negative DDL as `akr_app`: `DELETE FROM ref.list_versions` fails with `list versions are never deleted`".
- Smallest fix: Say that the DELETE check runs as the table owner, or expect "permission denied" when it runs as `akr_app`.

### O-14-14  LOW   (7.19 item 5 heading, check 1)
- Problem: The heading calls `harness_case` "a fourth row of the section 3.10 table". That table keeps three rows, and section 3.10 says the fourth target is in section 7.19.
- Proof: docs/plan/phase-14.md:1571 "(a fourth row of the section 3.10 table; reason family `change`" and docs/plan/phase-14.md:1067 "A fourth target, `harness_case`, is in section 7.19."
- Smallest fix: Replace "a fourth row of the section 3.10 table" with "a fourth target (section 3.10 keeps three rows)".

### O-14-15  LOW   (7.19 item 5 Activate row, check 4)
- Problem: `retire(tx, id, cr, at)` takes a timestamp, and the CHECK requires `closed_at` on every retired row. No step says that `at` is written to `closed_at`; the Activate row says only "`retire` with the request id".
- Proof: docs/plan/phase-14.md:1473 "CHECK ((state IN ('retired', 'rejected')) = (closed_at IS NOT NULL))," ; docs/plan/phase-14.md:1552 "async fn retire(&self, tx: &mut Tx, id: Uuid, cr: Uuid, at: DateTime<Utc>)" ; docs/plan/phase-14.md:1581 "`retire` with the request id; audit `harness_case.retired".
- Smallest fix: In the Activate row write that `retire` sets `retired_by_request` to the request id and `closed_at` to `at`.

## Summary

- Blocks and places checked: 7.19 (items 1 to 13); the paragraph "Owner decisions (2026-10-10)" before 7.19 (line 1447); the paragraph "Owner decision (2026-10-10), DR-415" in "## Decisions" (line 1626); 5 changed base places (header Complexity, the section 2.8 route-count line, the section 3.9 Feedback item, the section 3.10 pointer, section 6 item 10). Stale places elsewhere in the phase that the new text makes wrong: section 4.2, section 4.3, section 4.5, section 6 item 1 and section 7.16.
- Findings by severity: HIGH 0, MEDIUM 7, LOW 8 (15 in all).
- CHECKED: 7.19 items 2, 7 and 9 pass checks 1 to 6. The phase has 27 route rows (25 in section 2.8 plus the 2 of item 2) with no duplicate; both new routes are on `screening.work` (SS and M), and the roles match the Cedar permits and the frontend item.
- 7.19 finding ids by item: item 1 O-14-12; item 3 O-14-5; item 4 O-14-8; item 5 O-14-14 and O-14-15; items 6 and 11 O-14-6; item 8 O-14-11; item 10 O-14-9; item 12 O-14-10; item 13 O-14-7 and O-14-13.
- CHECKED: changed base places (header, section 2.8, section 3.9, section 3.10, section 6 item 10): checks 1 to 6 ok. The section 3.9 Feedback item quotes its replaced sentence inside a change note that names section 7.19; I treated that as a change note, not a finding.
- CHECKED: "Owner decisions (2026-10-10)" (line 1447) and "Owner decision (2026-10-10), DR-415" in Decisions (line 1626): checks 1 to 6 ok. The Decisions text matches the DR-415 row of docs/plan/research.md (table, two routes, material target, reports that never decide the gate, at least material for a lost known hit), and "with phase 07 section 7.6" exists and covers the bank corpus cases.
- Names and rules checked against phases 00 and 04: classification values `internal` and `personal` and the enum; `seal` and `keyed_hash`; roles `akr_app` and `akr_readonly`; the `ChangeTarget` methods, the `Prepared` fields, `ChangeRequestService::create` and `accept_warnings`; the `cr_material` route and the route-map rule (no new rule needed). All match.
- SQL checked: each CHECK of `alert.harness_cases` is met by the states the steps write, except the `closed_at` point in O-14-15. The column-level UPDATE grant covers every column the steps update (state, activated_at, closed_at, retired_by_request), and no step updates `trial`. The 14 classification rows name each column once, and the retention row and the setting row fit the phase 00 tables. Bank cases stay out of `cases`, `passed`, the recalls, `failures` and `passed_gate` everywhere the block and section 3.9 mention them.
