#!/usr/bin/env python3
"""Owner-decision round 1, lead find L-1 swept over the check lines of the other phases.

A statement run by a role that lacks the privilege is refused by PostgreSQL before any trigger runs, so a check line
"as akr_app ... fails with <guard message>" is wrong wherever akr_app has no such grant. Each line below was read
against the GRANT lines of its own phase file (see the L-1 sweep note in tools/agent/owner-verdict.md)."""
import sys
sys.path.insert(0, "/home/haithem/Projects/AKR-TMS/tools/agent")
from ofix import Fixer

f = Fixer(round_no=1)

# Phase 06: akr_app has SELECT, INSERT and UPDATE (status, superseded_by) on scn.cbi_catalogue_items; no DELETE
f.fix("L-1", "docs/plan/phase-06.md",
      "`DELETE FROM scn.cbi_catalogue_items` (any row, as\n   the owner role too) fails with `register rows are never deleted`;",
      "`DELETE FROM scn.cbi_catalogue_items` (any row) fails, as\n   `akr_app` with permission denied (no DELETE grant) and as the owner `akr_migrate` with `register rows are never deleted`;")

# Phase 09: akr_app has no DELETE on ref.countries, and SELECT, INSERT only on ref.fx_rates and admin.policy_packs
f.fix("L-1", "docs/plan/phase-09.md",
      "Negative checks, each\n   as the owner role and as `akr_app`: `DELETE FROM ref.countries` fails with `reference rows are never deleted`;\n   `UPDATE ref.fx_rates SET rate_to_iqd = 1` fails with `an FX rate is a fact`; a second",
      "Negative checks, each\n   as the owner role `akr_migrate` and as `akr_app`: `DELETE FROM ref.countries` fails with `reference rows are never deleted`\n   and `UPDATE ref.fx_rates SET rate_to_iqd = 1` fails with `an FX rate is a fact` (the two messages as the owner; as `akr_app`,\n   which holds neither privilege, both statements fail with permission denied); a second")
f.fix("L-1", "docs/plan/phase-09.md",
      "`UPDATE admin.policy_packs SET document = document` fails with `versions are\n   append-only`; `akr_app` has no DELETE",
      "`UPDATE admin.policy_packs SET document = document` fails with `versions are\n   append-only` as the owner and with permission denied as `akr_app` (SELECT and INSERT only); `akr_app` has no DELETE")

# Phase 15: akr_app has SELECT, INSERT only on kyc.ratings
f.fix("L-1", "docs/plan/phase-15.md",
      "as `akr_app`: `UPDATE kyc.ratings SET level = 2` fails with `append-only`; `DELETE FROM kyc.ratings`\n   fails;",
      "as `akr_app`: `UPDATE kyc.ratings SET level = 2` fails with permission denied (no UPDATE grant; as the owner\n   `akr_migrate` it fails with `append-only`); `DELETE FROM kyc.ratings`\n   fails;")

# Phase 17: case_events has no UPDATE grant for akr_app (item 12 already says permission denied)
f.fix("L-1", "docs/plan/phase-17.md",
      "11. Case chain: a test-only superuser change to one `case_events` payload is refused for `akr_app` by the trigger (`case_events` has no UPDATE\n    grant); the same change made by a superuser in the test database makes",
      "11. Case chain: a change to one `case_events` payload is refused for `akr_app` with permission denied (`case_events` has no UPDATE\n    grant; item 12); the same change made by a superuser in the test database, with `trg_case_events_append_only` disabled for that statement, makes")

# Phase 18: akr_app has SELECT, INSERT only on the five append-only tables
f.fix("L-1", "docs/plan/phase-18.md",
      "`casework.opened_views` or `casework.evidence_items` (the append-only trigger raises);",
      "`casework.opened_views` or `casework.evidence_items` (permission denied: it holds SELECT and INSERT only; as the owner `akr_migrate` the append-only trigger raises);")

# Phase 20: task_ref is not in the UPDATE column grant of casework.ops_tasks
f.fix("L-1", "docs/plan/phase-20.md",
      "`UPDATE casework.ops_tasks SET task_ref = ...` fails with `the identity of an operations task does not change`;",
      "`UPDATE casework.ops_tasks SET task_ref = ...` fails with permission denied (`task_ref` is not in the column grant) and, as the owner `akr_migrate`, with `the identity of an operations task does not change`;")

# Phase 21: goaml_reference is not in the UPDATE column grant of rpt.filings; no DELETE grant on rpt.fiu_feedback
f.fix("L-1", "docs/plan/phase-21.md",
      "an `UPDATE rpt.filings SET goaml_reference` fails with `the filing record is written once`;",
      "an `UPDATE rpt.filings SET goaml_reference` fails with permission denied (`goaml_reference` is not in the column grant) and, as the owner `akr_migrate`, with `the filing record is written once`;")
f.fix("L-1", "docs/plan/phase-21.md",
      "a `DELETE FROM rpt.fiu_feedback` fails with `rpt.fiu_feedback is append only`.",
      "a `DELETE FROM rpt.fiu_feedback` fails with permission denied (no DELETE grant) and, as the owner `akr_migrate`, with `rpt.fiu_feedback is append only`.")

f.done()

# Follow-up (same run of work): the guard of phase 21 raises with TG_TABLE_NAME, which carries no schema name
g = Fixer(round_no=1)
g.fix("L-1", "docs/plan/phase-21.md",
      "as the owner `akr_migrate`, with `rpt.fiu_feedback is append only`.",
      "as the owner `akr_migrate`, with `fiu_feedback is append only` (the guard prints `TG_TABLE_NAME`, which has no schema name).")
g.done()
