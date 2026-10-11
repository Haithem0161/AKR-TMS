#!/usr/bin/env python3
"""Owner-decision round 1 fixes: phase 14 (verdicts O-14-1..15, O-07-10 wording, L-1, L-4 in tools/agent/owner-verdict.md)."""
import sys
sys.path.insert(0, "/home/haithem/Projects/AKR-TMS/tools/agent")
from ofix import Fixer

f = Fixer(round_no=1)
P = "docs/plan/phase-14.md"

# O-14-1: operation counts of section 4.2
f.fix("O-14-1", P,
      "tags `lists` (8 operations) and `screening` (17); 25 in all, each with",
      "tags `lists` (8 operations) and `screening` (19: the 17 of section 2.8 and the 2 of section 7.19); 27 in all, each with")

# O-14-2: section 6 item 1
f.fix("O-14-2", P,
      "the phase 00 schema checks pass for the ten new tables (classification and retention rows present).",
      "the phase 00 schema checks pass for the eleven new tables, `alert.harness_cases` of section 7.19 among them (classification and retention rows present).")

# O-14-3: ungoverned list of section 4.3
f.fix("O-14-3", P,
      "`screening.search_per_min`, `screening.lookback_chunk`, and the four `schedule.*` keys. List thresholds",
      "`screening.search_per_min`, `screening.lookback_chunk`, `screening.harness_cases_max` (section 7.19), and the four `schedule.*` keys. List thresholds")

# O-14-4: four change targets (section 4.5, the crate listing and the services line)
f.fix("O-14-4", P,
      "- **Phase 04**: three change targets (`list_setting`, `screening_policy`, `good_guy_rule`); the route map is unchanged.",
      "- **Phase 04**: four change targets (`list_setting`, `screening_policy`, `good_guy_rule`, and `harness_case` of section 7.19); the route map is unchanged.")
f.fix("O-14-4", P,
      "AlertDedupe, FreezeTrigger port, the three change targets (pure logic, ports only)",
      "AlertDedupe, FreezeTrigger port, the four change targets (pure logic, ports only)")
f.fix("O-14-4", P,
      "# Pg repositories for the eleven tables of section 1\n",
      "# Pg repositories for the ten tables of section 1 and alert.harness_cases (section 7.19)\n")
f.fix("O-14-4", P,
      "`GoodGuyTarget` (the three change targets, section 3.10), `RecordingFreezeTrigger`",
      "`GoodGuyTarget` (the three change targets of section 3.10; `HarnessCaseService` and the fourth target `HarnessCaseTarget` are in section 7.19), `RecordingFreezeTrigger`")

# O-14-5: the exception named where the rule stands
f.fix("O-14-5", P,
      "is the raw form of `Identifiers` (section 2.1), hashed on receipt and never stored: `{ birth:",
      "is the raw form of `Identifiers` (section 2.1), hashed on receipt and never stored, except inside the sealed query of a known-answer case (section 7.19 item 3): `{ birth:")

# O-14-6: per-case results in the report; the tab says where its result comes from
f.fix("O-14-6", P,
      "`report.bank_cases = {cases, met, unmet, absent, unmet_ids: [case ids]}`; `absent` counts cases",
      "`report.bank_cases = {cases, met, unmet, absent, results: [{case_id, result}]}`: `results` has one element per case the run took, and `result` is `met`, `unmet` or `absent`; `absent` counts cases")
f.fix("O-14-6", P,
      "the state and the result of the latest run in words (\"As expected\", \"Not as expected\", \"Entry no longer on the list\"); \"Add a case\" (list, entry key, name, optional identifiers, expectation, reason) and \"Retire\". On S1,",
      "the state and the result in words (\"As expected\", \"Not as expected\", \"Entry no longer on the list\"); \"Add a case\" (list, entry key, name, optional identifiers, expectation, reason; the reason field carries the hint to write no name in it) and \"Retire\". The result of a case is its element of `report.bank_cases.results` in the newest `gate` or `oracle` run of the list's active version, joined by case id (`useGetScreeningHarnessRuns`, which S3 already calls); a case that no such run holds yet shows its trial result (`trial.met`). On S1,")
f.fix("O-14-6", P,
      "The harness report, which CFG, CHK, IA and AS read on S3, holds counts and case ids.",
      "The harness report, which CFG, CHK, CH, IA and AS read on S3, holds counts and, for each case id, one of three results.")

# L-4 (lead find while fixing): a trial is never `absent`, because step 3 refuses the key first
f.fix("L-4", P,
      "(`hit`, `below_line`, `needs_more_data`, `clear`) or `absent` when the version has no entry with the key.",
      "(`hit`, `below_line`, `needs_more_data`, `clear`). A trial is never `absent`: item 4 step 3 refuses a key that the active version does not hold, and `absent` is a result of a later harness run only (item 6).")

# O-14-7: the curl check names the list and the entry
f.fix("O-14-7", P,
      "- Curl (added to the phase suite): as SS `POST /api/v1/screening/harness-cases` with a synthetic entry key, `expect: \"hit\"` and `Idempotency-Key: hc-0001` -> 202",
      "- Curl (added to the phase suite, after its Lists step has activated the fixture version of `iq_national`): as SS `POST /api/v1/screening/harness-cases` with `list_code: \"iq_national\"`, the `entry_key` and the name of one entry of that fixture version, `expect: \"hit\"` and `Idempotency-Key: hc-0001` -> 202")

# O-14-8: the pointer for the identifiers
f.fix("O-14-8", P,
      "Normalise `name` with the names crate (section 3.1) and hash the identifiers as a search does (section 3.12 step 3).",
      "Normalise `name` with the names crate (section 3.1), as section 3.12 step 3 does for a search, and hash the identifiers into `Identifiers` on receipt (section 7.16).")

# O-14-9: CH in the bundle tests
f.fix("O-14-9", P,
      "Bundle tests: CFG, CHK, IA and AS are refused both actions;",
      "Bundle tests: CFG, CHK, CH, IA and AS are refused both actions;")

# O-14-10: what the stored request holds, and the reason text
f.fix("O-14-10", P,
      "The change request, which CHK reads, and the audit events hold the list code, the expectation and the trial result.",
      "The change request as its approvers read it (the diff of item 5) and the audit events hold the list code, the expectation and the trial result. The stored body of the request also holds the entry key, the sealed query and its keyed hash (item 4 step 7): the diff leaves all three out, and no reader of the request can open the sealed query. `reason_text` is plain text that the approvers read, so the form asks the maker to write no name in it (item 11).")

# O-14-11: which lists a governed change touches
f.fix("O-14-11", P,
      "The dry run also scores the active `hit` cases of each list the change touches, under the old and the new configuration.",
      "The dry run also scores the active `hit` cases of each list the change touches, under the old and the new configuration. A `list_setting` change touches the list of its object key; a `screening_policy` change touches every enabled list.")

# O-14-12: the comparable tables by name
f.fix("O-14-12", P,
      "`akr_readonly` gets no grant, as for the alert tables.",
      "`akr_readonly` gets no grant, as for `alert.screening_alerts`, `alert.screening_decisions`, `alert.pair_memory` and `alert.good_guy_rules`.")

# O-14-13 and L-1: a statement without the privilege never reaches the guard trigger
f.fix("L-1", P,
      "`DELETE` fails with `known-answer cases are never deleted`; an UPDATE of a `rejected` row fails;",
      "a `DELETE` fails, as `akr_app` with permission denied (no DELETE grant) and as the owner `akr_migrate` with `known-answer cases are never deleted`; as `akr_app` an UPDATE of the `state` of a `rejected` row fails with the guard's `is final` message;")
f.fix("L-1", P,
      "2. Negative DDL as `akr_app`: `DELETE FROM ref.list_versions` fails with `list versions are never deleted`; `UPDATE ref.list_entries` fails with permission denied;",
      "2. Negative DDL as `akr_app`: `DELETE FROM ref.list_versions` fails with permission denied (no role has a DELETE grant), and as the owner `akr_migrate` with `list versions are never deleted`; `UPDATE ref.list_entries` fails with permission denied;")
f.fix("L-1", P,
      "`UPDATE alert.screening_decisions` fails with `screening_decisions is append-only`;",
      "`UPDATE alert.screening_decisions` fails with permission denied (no UPDATE grant), and as `akr_migrate` with `screening_decisions is append-only`;")

# O-14-14: a fourth target, not a fourth row
f.fix("O-14-14", P,
      "5. **Change target `harness_case`** (a fourth row of the section 3.10 table; reason family `change`;",
      "5. **Change target `harness_case`** (a fourth target; the table of section 3.10 keeps its three rows; reason family `change`;")

# O-07-10: what a refusal at activation does on each path
f.fix("O-07-10", P,
      "`add`: `lock`; not `proposed` -> the target refuses and the request ends `failed`; `activate`;",
      "`add`: `lock`; not `proposed` -> the target refuses (phase 04 section 3.2: an immediate approval is rolled back with the error, and a scheduled activation ends `failed`); `activate`;")
f.fix("O-07-10", P,
      "`retire`: `lock`; not `active` -> refused; `retire` with the request id;",
      "`retire`: `lock`; not `active` -> refused in the same way; `retire` with the request id;")

# O-14-15: what the three state methods set
f.fix("O-14-15", P,
      "   `NewHarnessCase` holds the id, `list_id`, `entry_key`, `expect`, `query_enc`, `query_hk`, `trial`, `change_request_id` and `created_by`.",
      "   `NewHarnessCase` holds the id, `list_id`, `entry_key`, `expect`, `query_enc`, `query_hk`, `trial`, `change_request_id` and `created_by`. `activate` sets `state = 'active'` and `activated_at = at`; `retire` sets `state = 'retired'`, `retired_by_request = cr` and `closed_at = at`; `reject` sets `state = 'rejected'` and `closed_at = at`. These four columns are the whole UPDATE grant of item 1.")

f.done()
