---
phase: akr-tms-phases-00-31
verified: 2026-10-10T21:38:38+03:00
status: complete
score: 94/384 must-haves verified
gaps_confirmed: 641
gaps_by_severity: {high: 39, medium: 289, low: 313}
fixes_applied: true
recheck:
  done: 2026-10-10
  raised: 153
  confirmed: 153
  rejected: 0
  lead_finds: 8
  by_severity: {high: 7, medium: 43, low: 111}
  high: [R-02-1, R-06-2, R-06-3, R-10-1, R-13-2, R-24-1, R-27-7]
  fixes_applied: true
  clean_pass: false
recheck2:
  done: 2026-10-10
  scope: the 221 replacements of the first re-check
  raised: 32
  confirmed: 27
  rejected: 5
  lead_finds: 2
  by_severity: {high: 2, medium: 7, low: 20}
  high: [S-27-2, S-30-2]
  fixes_applied: true
  clean_pass: false
recheck3:
  done: 2026-10-10
  scope: the 29 replacements of the second re-check
  raised: 7
  confirmed: 3
  rejected: 4
  lead_finds: 2
  by_severity: {high: 0, medium: 1, low: 4}
  high: []
  fixes_applied: true
  clean_pass: false
recheck4:
  done: 2026-10-10
  scope: the 6 replacements of the third re-check
  raised: 0
  confirmed: 0
  rejected: 0
  lead_finds: 1
  by_severity: {high: 1, medium: 0, low: 0}
  high: [L4-27-1]
  fixes_applied: true
  clean_pass: false
recheck5:
  done: 2026-10-10
  scope: the 1 replacement of the fourth re-check
  raised: 0
  confirmed: 0
  rejected: 0
  lead_finds: 0
  by_severity: {high: 0, medium: 0, low: 0}
  high: []
  fixes_applied: true
  clean_pass: true
gaps:
  - V-00-12-a
  - V-04-08-a
  - V-07-01-a
  - V-07-08-a
  - V-10-09-a
  - V-10-09-b
  - V-11-02-a
  - V-11-03-a
  - V-11-03-b
  - V-11-04-a
  - V-11-04-d
  - V-11-07-a
  - V-12-10-a
  - V-13-03-d
  - V-15-01-d
  - V-15-06-a
  - V-17-01-a
  - V-19-05-a
  - V-20-12-a
  - V-21-03-a
  - V-21-05-a
  - V-21-08-a
  - V-23-01-a
  - V-23-08-a
  - V-24-10-a
  - V-25-02-b
  - V-25-07-a
  - V-25-07-d
  - V-25-07-f
  - V-25-08-a
  - V-25-12-a
  - V-26-06-a
  - V-27-02-a
  - V-27-08-b
  - V-29-02-a
  - V-29-03-a
  - V-29-04-a
  - V-29-04-f
  - V-30-05-a
---

# Verification pass, phases 00 to 31

This is the final pass that `.claude/rules/planning.md` asks for. It sampled 384 items, 12 from each of the 32 phase
files, and checked each one for five things: a complete SQL migration, a route table entry, a repository method, a
description of the business logic, and an audit event for every write.

## Result

- 94 of the 384 items passed with no gap. The other 290 had at least one.
- 641 gaps were confirmed: 39 high, 289 medium and 313 low. Another 8 claims repeated a gap already counted.
- Every confirmed gap is fixed in the phase files. The fixes are 549 text replacements and 218 new blocks under
  "7+. Gap Additions", recorded as DR-413 in `docs/plan/research.md`.
- The fixed files were then checked again (see "Re-check" below). That re-check found 161 more gaps: 7 high, 43
  medium and 111 low. All of them are fixed too.
- Four more re-checks followed, each on the text that the round before had changed. They found 29, 5, 1 and then 0
  true gaps (the sections below). Every gap known today is fixed, and the `gaps` list above is the record of the
  first pass, not a list of open items.
- The status is `complete` since the fifth re-check, which found no gap. The section "Fifth re-check" says what that
  shows and what it does not.

Severity as the lead used it: high is a real defect that blocks building or makes a stated check fail (a statement
PostgreSQL refuses, a read or write with no grant, a missing route or method, a state that can never be left). Medium
is one missing name, step list, refusal code or column whose meaning is clear. Low is a pointer, wording, a field list
or old text that a later block already overrides.

## Method

1. One auditor agent per phase picked 12 items (requirements the phase says it implements, and its gap blocks) and
   wrote every gap it saw (`tools/agent/verify/phase-NN.md`).
2. One refuter agent per phase tried to disprove each claim against the phase text (`tools/agent/verify/refute-NN.md`).
3. One drafter agent per phase wrote a fix for each claim that stood (`tools/agent/verify/draft/NN.txt`).
4. The lead checked every standing claim against the phase text, set its severity, and wrote the fix
   (`tools/agent/verify/fixes/NN.txt`). The verdict for each claim is in `tools/agent/verify-verdict.md`.
5. `tools/agent/apply-verify.py` applied all fixes once. Ten edits outside the phase files were made by hand
   (`tools/agent/verify/fixes/MANUAL.txt`).

## Per phase

| Phase | Items passed | Gaps confirmed | High | Medium | Low | Rejected | Duplicates |
|-|-|-|-|-|-|-|-|
| 00 | 4/12 | 11 | 1 | 6 | 4 | 1 | 0 |
| 01 | 7/12 | 5 | 0 | 4 | 1 | 1 | 0 |
| 02 | 4/12 | 13 | 0 | 3 | 10 | 3 | 0 |
| 03 | 6/12 | 7 | 0 | 3 | 4 | 0 | 0 |
| 04 | 4/12 | 11 | 1 | 3 | 7 | 1 | 0 |
| 05 | 8/12 | 4 | 0 | 2 | 2 | 2 | 0 |
| 06 | 5/12 | 17 | 0 | 5 | 12 | 4 | 0 |
| 07 | 1/12 | 20 | 2 | 5 | 13 | 1 | 0 |
| 08 | 3/12 | 11 | 0 | 4 | 7 | 3 | 1 |
| 09 | 3/12 | 19 | 0 | 8 | 11 | 4 | 0 |
| 10 | 0/12 | 18 | 2 | 13 | 3 | 3 | 0 |
| 11 | 2/12 | 34 | 6 | 14 | 14 | 10 | 0 |
| 12 | 4/12 | 13 | 1 | 7 | 5 | 3 | 0 |
| 13 | 0/12 | 34 | 1 | 17 | 16 | 4 | 0 |
| 14 | 4/12 | 14 | 0 | 7 | 7 | 1 | 0 |
| 15 | 4/12 | 17 | 2 | 10 | 5 | 6 | 0 |
| 16 | 3/12 | 14 | 0 | 5 | 9 | 2 | 0 |
| 17 | 4/12 | 21 | 1 | 15 | 5 | 0 | 0 |
| 18 | 0/12 | 19 | 0 | 5 | 14 | 4 | 0 |
| 19 | 3/12 | 20 | 1 | 10 | 9 | 1 | 1 |
| 20 | 2/12 | 26 | 1 | 12 | 13 | 3 | 1 |
| 21 | 2/12 | 24 | 3 | 13 | 8 | 1 | 2 |
| 22 | 3/12 | 21 | 0 | 8 | 13 | 0 | 0 |
| 23 | 1/12 | 20 | 2 | 11 | 7 | 1 | 2 |
| 24 | 0/12 | 33 | 1 | 17 | 15 | 1 | 0 |
| 25 | 1/12 | 50 | 6 | 23 | 21 | 3 | 0 |
| 26 | 2/12 | 28 | 1 | 13 | 14 | 6 | 0 |
| 27 | 2/12 | 34 | 2 | 12 | 20 | 5 | 1 |
| 28 | 4/12 | 11 | 0 | 6 | 5 | 0 | 0 |
| 29 | 3/12 | 24 | 4 | 9 | 11 | 7 | 0 |
| 30 | 4/12 | 22 | 1 | 11 | 10 | 2 | 0 |
| 31 | 1/12 | 26 | 0 | 8 | 18 | 4 | 0 |
| All | 94/384 | 641 | 39 | 289 | 313 | 87 | 8 |

"Rejected" counts the claims the verdict file lists as disproved.

## The 39 high gaps

| Claim | What was wrong | Fix |
|-|-|-|
| V-00-12-a | 7.18 guarded completion on the claim's `locked_at`, which the 7.2 heartbeat moves: every job longer than a third of its lease would roll back | REP 00 7.18: guard on `attempts` |
| V-04-08-a | 7.2 appends to `change_requests.evidence` but migration 0014 grants `akr_app` UPDATE on a |  |
| V-07-01-a | the profile reads list `report.prepare, config.author`; CHK holds neither, yet Cedar and section 6 let CHK read | 3 REPs 07 (both rows gain `config.approve`, the paragraph under the table); docs/API.md by hand (MANUAL.txt) |
| V-07-08-a | a retry of a `running` check run ended as done at step 1, and step 11 had no per-file record, so the run stayed `running` | 2 REPs 07: step 1, step 11 (NEW `GoamlCheckRepository::record_file`, one transaction per file) |
| V-10-09-a | no write path for a payment status change: step 3 made it a `RowConflict`, yet the table, the grant and 7.12 assume an append | block 10: `append_status` (NEW method); history entries keep the row hash so a re-read is `unchanged` |
| V-10-09-b | the status CHECK of 7.12 rejects `unknown`, which the `dq_issue` gate writes: the batch would fail | REP 10: the CHECK gains `unknown` |
| V-11-02-a | no step creates a watermark row; the first `advance` matches nothing | block 11: `ensure_rows` (NEW method) before the unit; NULL reads as sequence 0 for manifests |
| V-11-03-a | akr-api route called `LandingStore::estimate`, but akr-api never reads `raw` | REP 11 and block 11: `BatchRepository::rows_landed_in` (NEW); `LandingStore::estimate` is the job's |
| V-11-03-b | a replay receipt of a files table fails the `file_sha256` CHECK | block 11: CHECK of migration 0033 becomes `(delivery = 'files' AND NOT (manifest ? 'replay'))` |
| V-11-04-a | a declared period without `ended_at` could never close (and would cover every later date in phase 12) | REPs 11: `ended_at` is required; block 11 (declare steps) |
| V-11-04-d | `GET /coverage/blind-periods` scope shut out DS, who reads the register | REP 11: `dashboard.read, data.steward`; MANUAL API.md |
| V-11-07-a | the `landing_file` column list and the file format were pinned nowhere | block 11 (extract file contract) and two REPs 11 |
| V-12-10-a | `akr_app` had no UPDATE on `dq_issues.detail`; the lead also found `count` and `last_seen` missing for the recon run's upsert | REP 12: the grant of migration 0037 gains `detail, count, last_seen`; comment and inventory row |
| V-13-03-d | no steps for cluster membership; and (found by the lead) one open row per customer and level clashes with one live build per currency | block 13: `currency` in the membership key (migration 0040), `PeerRepository::get`, `insert_candidate`, four steps, activation at the last page, older candidate refused |
| V-15-01-d | no `record_decision` effect serves `change_request`, `kyc_review` or `exit_decision`; the three shipped definitions cannot publish | block 15: `KycRecordDecisionEffect` (NEW); block 04: effects resolve by kind and resource kind |
| V-15-06-a | no `WorkflowSubject` for `kyc_review` or `exit_decision` | block 15: two subjects (NEW) |
| V-17-01-a | the retry job writes `reason_detail` and the UPDATE grant on `alert.quarantine` did not name it | REP 17 (migration 0061 grant gains `reason_detail`) |
| V-19-05-a | IA and SEC had no `alarm.list` permit, so `GET /alarms` refused them against section 3.10 and check (p) | BLOCK 19 (permits `followup.ia.alarms`, `followup.sec.alarms`); REP 19 route row action `alarm.list` |
| V-20-12-a | the 7.3 deadline handler could not learn letter delivery; phase 21 named a phase 20 method that did not exist | BLOCK 20 (`LetterRequester::delivered`; no column); three REPs 21 (lead decision) |
| V-21-03-a | `reports_guard` refused the cleared signature of every revise | REP 21: the guard lets `rejected` or `returned` to `draft` clear it |
| V-21-05-a | the build wrote `generated_by` with no UPDATE grant | REP 21: the build does not write it |
| V-21-08-a | a PATCH during a validation run left changed content marked validated | REP 21: step 9 locks and re-checks the content hash |
| V-23-01-a | ADM lacked the scope for the run read that check 5a makes as `adm@`; Cedar permits ADM | eight REPs 23: the three reads take `audit.read` or `admin.system`; `held` is null for ADM too; MANUAL line for API.md |
| V-23-08-a | upper-case class keys fail the phase 00 settings CHECK; migration 0072 would abort | eleven REPs 23 and REP 14: lower case; MANUAL line for DR-377 |
| V-24-10-a | control report case counts ran on `readonly_pool`, which reads nothing in `casework` | BLOCK: read on the app pool with `CaseAccessRepository::manual_grant_counts` (no definer function) |
| V-25-02-b | session advisory lock never released, so every later run returns at step 1 | REP: unlock on the same connection on every exit path |
| V-25-07-a | `ON CONFLICT (source_key)` without the partial index predicate is refused by PostgreSQL | REP and BLOCK: `RegisterWriter::warn` (raised from MEDIUM) |
| V-25-07-d | the overdue guard needed an audit read `akr_app` lacks | REP and BLOCK: the audit is written only when `warn` returns an id |
| V-25-07-f | owner kind `access_review` not in the evidence registry, so the snapshot is refused | REP 02 |
| V-25-08-a | a malformed article had no outcome; section 6 used an undefined code | BLOCK and three REPs (25 and 30): refusal code `help_article_invalid` (lead decision) |
| V-25-12-a | no route uploads the rollout record | BLOCK: `POST /admin/releases/{id}/evidence`; MANUAL line |
| V-26-06-a | with group tasks the first verified group moved the whole freeze to `frozen` and silenced both freeze timers | BLOCK 26 (frozen only when every group task has a level; lowest level; freeze row lock) + REP 20 (pointer in 7.1) + REP 26 (step 8 follows fixes/20: release tasks on approval) |
| V-27-02-a | family and version rows were inserted before their inventory item, but `inventory_item_id` is NOT NULL with a non-deferrable foreign key | BLOCK 27 (item first, then the row; the resolver runs at the use gate) |
| V-27-08-b | a card never scanned left the version in `importing` for ever and blocked every later import of the family (3 attempts, about 14 seconds) | REP 27 (10 attempts; refusal at the last attempt) + REP 27 (job table row) |
| V-29-02-a | `assess rail add` is named but has no step, and no path creates the first mode row (the base version was a row id in an integer column) | block: `rail add` inserts the rail and audits `assess.rail_added`; the first row is the PUT route with `mode: shadow`; base version of a mode change = the rail's `rail_modes` row count; REP of the base version |
| V-29-03-a | bands were updated in place under a key without `set_version` while DELETE is refused, so a dropped band could never stop applying, and the guard and retention texts assume kept sets | block plus four REPs: key `(currency, band_code, set_version)`; the whole band set and the whole matrix are inserted per activation; current bands = greatest `set_version` |
| V-29-04-a | no step set `register_id`, and the closing job has no user for `closed_by`, so an exception could never close | block: link at step 5 (a closed warning of the day gets a second key); closure by the phase 24 route with the caller as `closed_by` |
| V-29-04-f | step 4 matched an advisory reject as expected while the next bullet makes it an exception, and the enforcing entry criterion reads that exception | REP: only an advisory hold is expected |
| V-30-05-a | the namespace that holds a restored copy of production data had no network policy, no zone check and no admission rule | block: default deny, the restore's own edges only, probed, Kyverno Enforce, deleted after the drill |

## Decisions that narrow the scope

These are the lead's decisions where a requirement had no data source or map in the plan. Each one is carried in part
and the rest is marked P2. The product owner should confirm them.

1. **Staff-behaviour rule pack (SP-36, phase 01).** The rules that the exported audit kinds can feed ship in
   `deploy/siem/use_cases.yaml`. Rules that need allowed reads are P2, because allowed reads are not exported. The
   shipped rules carry proposed time windows and thresholds, and the bank tunes them in its own SIEM (phase 25).
2. **Test corpora (phases 07, 14).** The goAML rejection corpus and the screening known-answer corpus grow by release
   only. A bank cannot add to them in this version (N-44 is carried in part).
3. **Counterparty profiles and batch tags (phase 13).** VB-05 is carried as counterparty-keyed features only. The
   payroll and pension batch tags and the government-source flag of S-17 are P2: no core banking field feeds them.
4. **RFI pre-fill (M-44, phase 19).** P2. The plan holds no map from the Wolfsberg question keys to canonical fields.
5. **SIEM dead letters (phase 25).** Resolving a dead letter is P2. The tile counts the last seven days.
6. **Islamic scenarios (phase 26).** SCN-ISL-07 for a bank whose qard hasan sits in schedule tables is P2 until the
   schedule mapping names a repayment event. The related-person leg of SCN-ISL-06 reads the mapped flag or a family
   relation to a staff party.
7. **Payments with no UETR (phases 10, 29).** Matching a released payment needs the bank's mapping to fill
   `message_refs.internal_ref`. The gateway team names the column. Without it the payment is `no_match_key`.
8. **Backup and restore (phase 30).** The offline copy (SP-88) is a monthly procedure of the bank's operators, and no
   repository is object-locked. The credential rotation after a ransomware restore is an ordered runbook of existing
   commands. No route serves a stored drill or restore report: the operator keeps the file and the security officer
   sees the audit event.
9. **Label of a Latin name (phase 08).** A Latin name shown under an Arabic or Kurdish name is always labelled
   "typed". The labels "card", "passport" and "generated" of the design system are not used: no mapped field says
   that a name was read from a document, and the product generates no Latin form.

The other lead decisions are design choices inside a phase. DR-413 lists the main ones, and the verdict file gives
each one beside its claim.

## Checks after the fixes

- `python3 tools/idcheck.py`: 413 decisions defined, 0 problems.
- `python3 tools/routecheck.py tools/agent/gap1-routes.txt`: 581 rows in API.md, all owned by a phase route table.
  Five phase routes are not in API.md by design (`/healthz`, `/readyz`, `/metrics`, `/_echo`, `/_dev/detections`).
- `python3 tools/coverage.py tools/agent/gap1-coverage.txt`: 940 requirements carried, 268 named as not carried by a
  phase, 0 named nowhere. These numbers are the same as before the pass.

## Re-check

Done on 2026-10-10. The re-check looked at the 767 fixes of this pass, not at the whole plan again.

**Method.** One agent per phase read the list of fixes applied to its phase and the fixed phase file. It reported any
fix that was wrong, incomplete or in conflict with other text (`tools/agent/verify/recheck-NN.md`). The lead checked
every finding against the phase text, set its severity on the scale above, and wrote the fix. The verdicts are in
`tools/agent/recheck-verdict.md`. The fixes are exact text replacements in `tools/agent/recheck-fixes.txt`, applied
by `tools/agent/recheck-fix.py`. A copy of the files from before is in `tools/agent/verify/pre-recheck-backup.tgz`.

**Result.**

- The agents raised 153 findings. All 153 were true; none was rejected.
- The lead found 8 more: 2 while deciding the findings and 6 in a second look at the applied fixes.
- That is 161 gaps: 7 high, 43 medium and 111 low. The agents had rated 20 findings high. On the lead's scale 7 are
  high; most of the others were one missing name or a wording fault.
- All 161 are fixed, by 221 replacements in the phase files, `docs/plan/status.md` and `docs/DSL.md`. Their design
  decisions are in the DR-413 row.

**The 7 high gaps.**

| Gap | What was wrong | Fix |
|-|-|-|
| R-02-1 | the audit verifier reads case events as `akr_readonly`, and that role had no grant on `casework.case_events` | phase 17 grants SELECT on four columns (`case_id`, `seq`, `at`, `hash`); no payload and no actor column |
| R-06-2 | a retried gap-report job stopped at once on a run that was already `running`, so the run never left that state and blocked every new run | a `running` run is a retry and resumes from the step it reached |
| R-06-3 | the report cover reads `deploy/branding.json` in akr-worker, and no mount gave the worker that file | a read-only mount is added in phases 00 and 06 |
| R-10-1 | the fact insert named a conflict target for which six fact tables had no unique index, so PostgreSQL would refuse it | the target is named per table, and two unique constraints are added (`payment_message_party`, `account_hold`) |
| R-13-2 | the seed feature `distinct_counterparties_3m` counts `txn.counterparty_id`, a `personal` column that the validator refuses, so the seed would fail | one exception: the field of a `distinct_count` may be that surrogate id, which is counted and never stored or shown |
| R-24-1 | register export files were stored with classification `restricted`, a value the enum does not have, so every insert would fail | classification `personal` |
| R-27-7 | the model-score route answered 401 to any caller without step-up on a suspicion-tier alert, so a caller outside the case team learned that the alert exists (against DR-411) | the masked 404 comes first; only a caller the visibility rule admits gets 401 |

**Second look.** After the fixes were applied, the lead read the 81 replacements of the high and medium fixes in
place and found no conflict with the text around them. A count of the route rows of each phase against
`docs/plan/status.md` found stale route counts in phases 04, 06, 15, 18 and 24, where gap blocks had added routes
after the count was written. They are corrected. The route total in `status.md` is now 581, which is the sum of the
column and the number of rows in API.md.

**Checks after the re-check fixes.** The three checks of the section above were run again and give the same numbers:
413 decisions with 0 problems; 581 API.md rows, all owned; 940 requirements carried, 268 named as not carried and 0
named nowhere.

**Status.** `gaps_found` stays. The rule of `.claude/rules/planning.md` is that the plan is ready only when a pass
finds no gap. This re-check found 161, so it was not a clean pass. One more re-check, of the 221 replacements this
round made, was run next: the section below.

## Second re-check

**Method.** On the user's word, a second round checked only what the first re-check had changed: its 221
replacements. One Haiku agent per phase read the replacements made in that phase file (212 in all, each with the
lead's verdict line that says what the fix was meant to do) and tested six things for each: the new text stands and
reads right, no other line of the phase still states the old rule, every name it uses exists, its SQL and types agree
with the tables, its counts are right, and the fault it was written for is closed. The lead checked the 9
replacements in `status.md` and `DSL.md`. The lead then decided every finding against the phase text. The prompt is
`tools/agent/p-recheck2.txt`, the agents' files are `tools/agent/verify/recheck2-NN.md`, and the verdicts are in
`tools/agent/recheck2-verdict.md`.

**Result.** 13 of the 32 phases had no finding. The agents raised 32 findings in the other 19. The lead confirmed 27
and rejected 5: three named text that stood before the replacement, and two read a designed pattern as a conflict (a
route row that a gap block shows again, and a general rule with named exceptions). The lead found 2 more from the
side notes of one agent. That is 29 true gaps: 2 high, 7 medium and 20 low. All 29 are fixed, by 29 replacements
(ids `S-` and `L2-` in `tools/agent/recheck-fixes.txt`).

| Gap | Phase | What was wrong | Fix |
|-|-|-|-|
| S-27-2 | 27 | The first re-check made the model-score route answer the masked 404 to any caller the visibility rule refuses, but left test l and one error row expecting 422 `E_PURPOSE_REQUIRED` for a team member with no purpose code on a suspicion-tier alert. Rule 5 of phase 17 admits no caller without a purpose code, so the test and the rule could not both hold | Test l expects the masked 404; the error row gives the 422 to the restricted tier only |
| S-30-2 | 30 | The first re-check let the network allows of the restore namespace select every pod. The known-bad probe that must reach nothing runs in that namespace as an unlabelled pod, so it would have reached the repository, the object store and OpenBao, and the check before each restore rehearsal would fail | The allows select a label that only the restore Job and the scratch pod carry |

Both high gaps were faults of the lead's own first-round fixes. The 7 medium gaps: the lookup of a service
principal by name could match a disabled and an active one (phase 01); the development policy file named no
`@role` for its permits, and it was loaded under a narrower condition than its actions were compiled in (phase 03);
phase 08 still gave the old shape of the policy matrix file (phases 03 and 08); the definition of `AttributeKey`
left out the feature and rating keys (phase 06); the order of two refusals at case assignment was not stated
(phase 17); and one line still gave the old transaction unit of the scoring sweep (phase 27). The 20 low gaps are
pointers, stale sentences and wording.

**Points settled.** A `record_open` step-up is spent by `consume` like every other class (phase 01 said "never
consumed" in one clause, against its own steps and six later phases). The development policy file and its schema
fragment are loaded whenever the build has the feature; the environment decides only whether the routes are
mounted. The pods of the restore rehearsal carry their own membership label.

**Checks after the fixes.** The three checks give the same numbers: 413 decisions with 0 problems; 581 API.md rows,
all owned; 940 requirements carried, 268 named as not carried and 0 named nowhere. The fix applier now keeps a list
of the entries it has applied (`tools/agent/recheck-fixes.done`), because a later entry may change text that an
earlier entry wrote.

**Status.** `gaps_found` stays: this round found 29 true gaps, so it was not a clean pass. The numbers fall from
641 to 161 to 29, and the high gaps from 39 to 7 to 2. A third round, on the 29 new replacements only, was run next: the section below.

## Third re-check

**Method.** The same round again, smaller: one Haiku agent per touched phase file (17 files) read the replacements
that the second re-check made in it (29 in all, each with the lead's verdict line) and ran the same six tests. The
lead decided every finding against the phase text. The prompt is `tools/agent/p-recheck3.txt`, the agents' files are
`tools/agent/verify/recheck3-NN.md`, and the verdicts are in `tools/agent/recheck3-verdict.md`.

**Result.** 10 of the 17 phase files had no finding. The agents raised 7 findings in the other 7. The lead
confirmed 3 and rejected 4: a section 7 block that extends a step of base text (phase 03), a column comment with no
effect (phase 06), a general rule that no mapping line repeats (phase 22), and two lines that do not disagree
(phase 30). The lead found 2 more while checking the phase 30 finding. That is 5 true gaps: 0 high, 1 medium and 4
low. All 5 are fixed, by 6 replacements (ids `T-` and `L3-` in `tools/agent/recheck-fixes.txt`).

| Gap | Phase | Severity | What was wrong | Fix |
|-|-|-|-|-|
| T-01-1 | 01 | medium | A fault of the second round's fix: `client disable` finds the service principal among active ones only, but it sets the principal to disabled before it deletes the Hydra client and promised that the command can be run again after a failed Hydra call. The second run would find nothing, so the Hydra client could never be removed by the command | A second run that finds no active principal of the name only repeats the Hydra delete for the disabled principals that carry it; it changes no row and writes no second audit event |
| T-04-1 | 04 | low | The OpenAPI bullet called the evidence route a ninth operation of the tag `change-control` while still counting that tag as 8 | The tag count reads 9 |
| T-18-1 | 18 | low | Section 4.5 item 4 still said a reassignment revokes the previous assignee's grant with no condition: the third place of a sentence the second round corrected in two | The item names the two reasons (`assignment`, `second_review`) |
| L3-00-1 | 00 | low | The phase 00 list of the keys that phase 30 adds to `deploy/zones.yaml` named three; phase 30 names four | The list names `backup.repository` |
| L3-30-1 | 30 | low | Section 3.1 called the first of its two input files new; the second one is | "the second new in this phase" |

**Checks after the fixes.** The three checks give the same numbers: 413 decisions with 0 problems; 581 API.md rows,
all owned; 940 requirements carried, 268 named as not carried and 0 named nowhere.

**Status.** `gaps_found` stays: this round found 5 true gaps, so it was not a clean pass. The numbers fall from 641
to 161 to 29 to 5, and the high gaps from 39 to 7 to 2 to 0. A fourth round, on the 6 new replacements only, was
run next: the section below.

## Fourth re-check

**Method.** The same round again: one Haiku agent per touched phase file (00, 01, 04, 18, 30) read the 6
replacements of the third re-check and ran the same six tests. The prompt is `tools/agent/p-recheck4.txt`, the
agents' files are `tools/agent/verify/recheck4-NN.md`, and the verdicts are in `tools/agent/recheck4-verdict.md`.

**Result.** The agents raised no finding: all 6 replacements stand and are right. Before closing the round the lead
judged one open point that the lead had noted in an earlier round and left aside, because no round had raised it.
It is a true gap, so the round is not a clean pass.

| Gap | Phase | Severity | What was wrong | Fix |
|-|-|-|-|-|
| L4-27-1 | 27 | high | Check 6 of section 6 said the visibility predicate and "the Cedar permit of `alert.model_score.read`" agree on every row. The permits of that action are by role only, no forbid of phase 17 names the action, and the route applies visibility in its handler. So the Cedar decision allows every A1, A2 and M on every alert, the predicate does not, and the check as written could not pass | The check pins the handler: over the generated principals and rows of phase 17 check 6, the route answers the masked 404 on exactly the rows on which phase 17 `GET /alerts/{id}` answers it. The text says the permits are by role only |

The design did not change: the route was already safe, because its handler hides the same rows as the alert's own
read. Only the check was worded for a rule that Cedar does not carry for this action.

**Checks after the fix.** The three checks give the same numbers: 413 decisions with 0 problems; 581 API.md rows,
all owned; 940 requirements carried, 268 named as not carried and 0 named nowhere.

**Status.** `gaps_found` stays: this round found 1 true gap. True gaps per round: 641, 161, 29, 5, 1. A fifth round,
on the 1 new replacement only, was run next: the section below.

## Fifth re-check

**Method.** One Haiku agent read the one replacement of the fourth re-check (check 6 of phase 27) and ran the same
six tests. The prompt is `tools/agent/p-recheck5.txt`, the agent's file is `tools/agent/verify/recheck5-27.md`, and
the verdict is in `tools/agent/recheck5-verdict.md`.

**Result.** No finding. The agent confirmed each fact the new check rests on: the three permits of the action test
the role only, no forbid of phase 17 names the action, phase 03 has no tier forbid over every action, and phase 17
check 6 pins the predicate to the Cedar decision for `alert.read`. The lead has no open point left. That is 0 true
gaps: a clean round. Nothing was changed, so no further round follows.

**Checks.** The three checks give the same numbers: 413 decisions with 0 problems; 581 API.md rows, all owned; 940
requirements carried, 268 named as not carried and 0 named nowhere.

**Status.** `complete`, the value of `.claude/rules/planning.md` for a pass with no open gap. By that rule the plan
is ready for implementation.

**What this shows, and what it does not.**
- Every gap that the pass and its five re-checks found is fixed: 641, 161, 29, 5 and 1, which is 837 in all, 49 of
  them high.
- Every fix was read again by a later round, until a round found nothing to correct.
- The first pass was a sample: 384 items, 12 from each phase. It did not read every line. The four re-checks after
  the first read only the text that had changed. On the way the lead found 5 gaps in text that no round was reading
  (rounds 2, 3 and 4; one of them high). So a fresh read of the whole plan would probably still find some gaps, most
  of them small. This report does not claim that none are left.
- A gap met while building is fixed in the phase file in the same change, as `planning.md` asks.
- The nine decisions under "Decisions that narrow the scope" still wait for the user's word.
