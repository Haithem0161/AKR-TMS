#!/usr/bin/env python3
"""Owner-decision round 3: the fixes for the confirmed findings of the 11 round-3 agents and the lead finds (L3-n).
Every verdict is in tools/agent/owner-verdict.md under "## Round 3". No design rule changes in this round."""
import sys
sys.path.insert(0, "/home/haithem/Projects/AKR-TMS/tools/agent")
from ofix import Fixer

f = Fixer(round_no=3)
P = "docs/plan/phase-%s.md"

# ---------------------------------------------------------------- phase 18: the guard message depends on the row state
f.fix("Q-18-1", P % "18",
      "as the owner `akr_migrate` the guard raises `an access grant is revoked, never changed`).",
      "as the owner `akr_migrate` the guard raises `an access grant is revoked, never changed` for a grant that is "
      "not revoked and `a revoked access grant does not change` for a revoked one).")

# ---------------------------------------------------------------- DATA-MODEL.md: the canon grants summary
f.fix("Q-26-1/2, L3-1", "docs/DATA-MODEL.md",
      "`loaded_last_at` of `party_contact`, and `status` and `status_history` of `payment_message` (phase 26 adds the "
      "updates named in its own paragraph below); no role has DELETE on\n`canon` except `akr_retention` on the party "
      "tree (phase 23; partitions are dropped only by `admin.retention_drop_partition`). `akr_readonly` reads a subset",
      "`loaded_last_at` of `party_contact`, `end_at` and `row_hash` of `account_hold` (phase 10 section 7.19), and "
      "`status` and `status_history` of `payment_message`; no role has DELETE on\n`canon` except `akr_retention` on "
      "the party tree (phase 23; partitions are dropped only by `admin.retention_drop_partition`). Phase 26 adds to "
      "both lists (its own paragraph below): UPDATE for `akr_ingest` on columns of `islamic_deals`, `deal_parties`, "
      "`facility_instalments` and `facility`, and DELETE for `akr_retention` on `islamic_deals`, `deal_parties` and "
      "`facility_instalments`. `akr_readonly` reads a subset")

# ---------------------------------------------------------------- phase 13: the owner's guard check needs a row
f.fix("Q-13-1", P % "13",
      "and, as the owner `akr_migrate`, with `feature definition content is immutable`;",
      "and, as the owner `akr_migrate` on a stored row (the guard is a row trigger, so the owner's statement needs "
      "a row: a fixture row, or a row of the seed of item 3), with `feature definition content is immutable; "
      "propose a new version`;")

f.fix("Q-13-2", "docs/DATA-MODEL.md",
      "is kept without a value: `no_history`, `basis.reason = no_activity`; phase 13 section 7.18)",
      "is kept without a value: `no_history` (`beyond_window` for the kind `days_since`), "
      "`basis.reason = no_activity`; phase 13 section 7.18)")

f.done()
