# Gap pass 1 run 4a: lead verdict (2026-10-10; decisions final)

## Carried items (lead decisions, applied by apply-g1-run4a.py)

- Phase 20 heading fix (done inline 2026-10-10): its "### 7+. Gap Additions" was a level-3 heading; now "## 7+." like
  the other phases.
- Phase 22 section 4.5 item 9: the flag route is phase 03's (`PUT /admin/flags/{key}`, phase 03 route table), not
  "phase 06 section 3". Phase 06 seeds the flag (its migration) and its section 6 test toggles it. rep: "(phase 06
  section 3, `PUT /api/v1/admin/flags/branch.observations_live`)" -> "(phase 03 route `PUT /api/v1/admin/flags/{key}`;
  phase 06 seeds the flag)".
- Phase 22 Open doubts 14: phase 07's "No notifications (phase 22)" line keeps a new candidate on the screens, the
  cockpit and an alert rule; it promises no notice. Phase 10 section 3.5 says open drift shows on T3 ("notifications
  arrive with phase 22"): decided, no drift or candidate topic in this release; T3 and the cockpit are the notice.
  Block in phase 22 (closes item 14) + correction block in phase 10 (drift stays on T3; phase 22 defines no drift topic).
- Phase 22 "family `observation`" (section 4.5 item 4 and Open doubts 2): the hybrid scenario is a register scenario
  with `observational = true` (phase 06), DR-382; rep both sentences.
- Phase 19 Open doubts 9: closed by DR-375 (producer: phase 14 decision transaction from phase 15 on; suppression
  records: `alert.pair_memory` rows by `last_suppressed_at`). Block in phase 19.
- Phase 20 Open doubts 22: closed by DR-376 (phase 14 section 7+: delisting by absence, `delisted_on` from a publisher
  date only, drops guarded by `screening.max_count_drop_pct`). Block in phase 20.
- Phase 24 Open doubts 11: S-48 is P2 (DR-380); the rest of the list stays for the coverage runs. Open doubts 30:
  activation instant = `scn.scenario_versions.effective_from` (phase 16 section 7+). Block in phase 24.

## Per finding (Haiku draft: 31 findings, 0 HIGH, 19 MEDIUM, 12 LOW; all sources grepped)

- G1-17-01 ACCEPT (phase 19 section 4.5 item 4 confirmed verbatim, step 9 `BreachService::arm` included).
- G1-17-02 ACCEPT (phase 18 section 4.5 items 4, 5, 7 and section 1.1 confirmed).
- G1-17-03 ACCEPT, trimmed to the source ("on a declared conflict"; phase 19 closes phase 17 Open doubts 16).
- G1-17-04 ACCEPT (phase 25 section 3.2 Use steps 1, 2 and section 4.5 confirmed).
- G1-17-05 ACCEPT (phase 27 queue order, route 16, `model_band`, probe set confirmed).
- G1-18-01 ACCEPT (phase 19 section 4.5 item 5 confirmed).
- G1-18-02 ACCEPT; closes phase 18 Open doubts 11 (phase 19 says so).
- G1-18-03 ACCEPT (phase 21 section 4.5 items 1, 2, 4, 5; DR-170); closes phase 18 Open doubts 3; Open doubts 10 and 15
  already point to phase 21.
- G1-18-04 ACCEPT.
- G1-18-05 ACCEPT (phase 28 section 4.5 confirmed).
- G1-18-06 ACCEPT, corrected: closes phase 18 Open doubts 14 (Haiku wrote "phase 23 Open doubts 14"); DR-193.
- G1-18-07 DECIDE, DR-384: `case.review` needs no column: due = `casework.cases.opened_at` + the clock's days (registry
  value in force) for a case in `open` or `in_review`; closed and merged cases give no row. Blocks in phase 18 and
  phase 24 (closes phase 24 Open doubts 19; its clocks table row reads the rule).
- G1-18-08 ACCEPT (24 sub-codes counted against phase 19 section 3.11; ranks from phase 19 section 3.15).
- G1-18-09 ACCEPT, rewritten: phase 18 names the party predicate of section 3.10 as `PartyReadPredicate::allows(principal,
  party_id, context) -> bool` (phase 26's three-argument form). The remark on phase 26's attribution is carried to run 4b.
- G1-19-01 ACCEPT, plus DECIDE DR-385 for the UNCLEAR part: phase 22 owns the read extension: the rows of `GET
  /internal-reports` whose internal report has a `casework.observations` row (`internal_report_id`) carry `observation
  {indicator_groups, escalation_criteria, notes}` for M only (notes decrypted for M). Note in phase 19, block in phase 22
  (closes phase 22 Open doubts 9 (a)). Phase 19's release payload already has `rfi_id` (`{rfi_id, kind}`); phase 22 adds
  `recipient_kind`.
- G1-19-02 ACCEPT (phase 28 section 4.5 and items 13, 14 confirmed). Draw seed: the weekly draw's existing seed rule of
  section 3.8 applies (no new seed).
- G1-19-03 ACCEPT, corrected: `ops_task` is phase 20's subject kind (0066); no claim on `stage` for later kinds.
- G1-19-04 ACCEPT with the two route rows named (`GET /rfis/correspondents`, `POST /rfis/{id}/packs`, phase 31).
- G1-19-05 ACCEPT (phase 31 cites the sampler as section 3.9; it is phase 19 section 3.8; carry the citation to 4b).
- G1-19-06 ACCEPT.
- G1-20-01 ACCEPT (phase 26 section 3.8 confirmed). Heading already fixed.
- G1-21-01 ACCEPT (`UpdateReportDraft` is phase 21's DTO).
- G1-22-01 ACCEPT, corrected: recipients come from phase 01 `GrantRepository::active_holders(role)` (Haiku invented
  `UserRoleQuery::holders`).
- G1-23-01 REWRITE as rules, not a 70-row table: each later phase that inserts `admin.retention_assignment` rows adds one
  handler per table, by class (R3/R8 owner rows: party batch; R6, R7, R11: `report_only`; a child of a `report_only`
  owner: `report_only`; R9 `kept`; R10 never; R12 `age`: the owner's named sweep (phase 22 `notification_sweep`, phase 29
  `assess_ledger_sweep`) or else the engine's `record_purge` by age; `retirement`: never due). `closure` on the table's
  own column: the row is its own owner. From phase 27: `alert.model_scores` rows are deleted before an `alert.alerts`
  partition drop. From phase 29: an `assess.decisions` partition is dropped only when no `assess.holds` row references it.
  DR-387.
- G1-23-02 DECIDE, DR-386: `akr_retention` holds DELETE on exactly the tables whose handler deletes rows
  (`record_purge`, including the later tables of G1-23-01); none on `report_only`, `kept`, `retirement` or `never`
  tables; partitions go through the drop function. Check 4a asserts grant = deleting handler. Carry to 4b: phases 27,
  28, 29, 31 "SELECT and DELETE on every new table" narrowed to the deleting tables.
- G1-24-01 ACCEPT.
- G1-24-02 ACCEPT (phase 26 tiles: counts only, its section 5.3).
- G1-24-03 ACCEPT.
- G1-24-04 ACCEPT.
- G1-24-05 ACCEPT.
- G1-24-06 DECIDE, DR-388: phase 24 defines the port `RegisterWriter` implemented by `GovernanceRegisterService`:
  `warn(tx, source, source_key, title_key, body) -> Option<Uuid>`, `ON CONFLICT (source_key) WHERE source_key IS NOT
  NULL DO NOTHING` on `uq_registers_source_key`; `title_key` is resolved to `title {ar, en}` from the message catalogue;
  `body` holds ids, codes and counts. Called only from jobs. Phase 26's name kept.

## Carried to run 4b (from the draft's summary and this check)

- Phase 26: the party predicate is phase 18's (`PartyReadPredicate`, phase 18 section 7+), not built by phase 17.
- Phase 27 Open item 9 (advance notice to MRO): decide with phase 22's dispatcher.
- Phase 30 still names the key `akr-sign-cert`; phase 23 section 7+ renamed it `akr-sign-deletion`.
- Phase 31 cites the QA sampler as phase 19 section 3.9 (it is 3.8).
- Phases 27, 28, 29, 31: `akr_retention` DELETE grants narrowed to tables whose handler deletes (DR-386).
- Phase 29 payment release status; phase 30 Open doubts 23; phase 09 typology "not here" line.
