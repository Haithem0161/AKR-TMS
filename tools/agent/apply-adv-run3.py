#!/usr/bin/env python3
"""Apply adversarial pass run 3 (watermarks, restores, failover, late batches, source pauses) after the lead's checks.

Source: tools/agent/adv-run3.md (Haiku draft); decisions: tools/agent/adv-run3-verdict.md.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'

HDR = ('Adversarial pass (2026-10-10), run 3 (watermarks, restores, failover, late batches, source pauses), from '
       'tools/agent/adv-run3.md; lead-verified (tools/agent/adv-run3-verdict.md).')

BLOCKS = {
    '11': [
        """### 7.N A paused source is stale (ADV-3-01 HIGH, ADV-3-07 LOW; DR-409)
While a source is paused nothing reads the CBS date, so rule e of section 3.7 compares `data_as_of` with a frozen date
and rule f never fires: the source looked fresh and every unit covered. `FreshnessPolicy::classify` therefore also takes
the source state, and a rule a2 follows rule a: a source in state `paused`, `paused_by_bank` or `refused` is stale with
`stale_reason = lag_ceiling` (no read is being made, so the lag has no ceiling). The coverage ledger then reads
`source_stale` (phase 12 section 3.7). The automatic blind-period opens of section 3.6 for a pause, a bank pause and a
refusal set `business_from` as the halt path does (the earliest `value_business_date` of the source plus one day), and
phase 12 states when a period covers an as-of date (phase 12 section 7+). Test: a source paused for three business days
on the test clock has every watermark stale and every unit dark.""",
        """### 7.N Sequence keys re-read a commit window (ADV-3-03, HIGH; DR-409)
Research 25 section 7.3: a key taken early and committed late is skipped by a reader that keeps only the highest value
seen. Section 3.1 planner rule a for `sequence_id` therefore starts at the last component of the stored key tuple minus
`ingest.sequence_overlap_keys` (migration `0035`: integer, default 10000, governed, `floor = {"min": 100, "basis":
"research 25 s.7.3 commit order"}`), not at the stored key. The window must exceed the keys the CBS can hand out during
its longest transaction; the change detector's hash guard absorbs the repeated keys. Test: key 500 committed after key
501 was read lands once, in the next unit.""",
        """### 7.N The high-water check before each unit (ADV-3-04, HIGH; DR-409)
Research 25 section 7.5 names the regression signal: the source's high-water mark falls below AKR's watermark. The
business-date and marker signals of section 3.5 miss a same-day restore where the marker is unvalidated (Open doubts 8).
Before the first page of a unit of kind `sequence_id`, `timestamp_column`, `t24_date_time` or `rowversion`, the unit reads
`MAX(<watermark column>)` of the table and branch in one approved aggregate statement under the governor (the form of
the phase 12 control-totals reads, proven by `sqlproof`). A maximum below the stored `value` is the signal
`HighWaterRegressed` of `RestoreDetector`: the halt path runs with `halted_reason = as_of_regressed` (the closed list)
and `statement.signal = high_water_regressed`, cause `failover_suspected` when the endpoint changed, else
`restore_suspected`. The check works on every engine. A legitimate delete of the newest rows is answered with
`restored` and `restored_to` = the observed maximum (no later than the stored value); `false_alarm` moves no watermark
and would halt again on the next unit. Test: lowering the fixture's maximum key below the stored value halts the source.""",
        """### 7.N Late arrivals are measured and the auto lookback widens (ADV-3-05, HIGH; C-21; DR-409)
DSL B.2 `lookback: auto` derives the window from measured late arrivals and widens it; this phase parsed `lookback` only.
1. Each receipt records `manifest.late_days_max`: the largest number of days between a new or changed row's business
   date (or its watermark timestamp) and the receipt's extraction date. Unchanged repeats of the re-read window do not
   count. Histogram `akr_ingest_late_arrival_days{source, table}`.
2. The freshness job (section 3.7) reads, per table, the largest `late_days_max` of the last 90 days. Above
   `ingest.late_alarm_pct` percent of the table's window: audit `ingest.late_arrival_near_window {source_id,
   source_table, late_days, window_days}` once per table and day, and the alert rule of section 4.3.
3. For a table whose mapping says `lookback: auto`, planner rule a uses the larger of `overlap` and 1.25 times that
   largest lag, capped at `ingest.lookback_max_days`; a change audits `ingest.lookback_widened {source_id, source_table,
   from_days, to_days}`. A fixed `lookback` duration is used as written.
Migration `0035` adds `ingest.late_alarm_pct` (integer, default 80) and `ingest.lookback_max_days` (integer, default 93),
both governed. A row later than the window stays unread: that bound is DR-37, and the alarm is how a person learns the
tail is near it. Test: a row landed 28 days late against a 31-day window writes `ingest.late_arrival_near_window`.""",
        """### 7.N What the resume decisions write (ADV-3-08 LOW, ADV-3-09 MEDIUM; DR-409)
Section 3.5 resume step 3:
- `restored` on a `business_date` table: `restored_to` is a date, and `reset_and_clear` writes `value`,
  `value_business_date` and `business_date_moved_at = now()`, so the planner re-reads from the restored date.
- `false_alarm` after an `as_of_regressed` halt writes the marker observed by the halting unit (its rejected receipt)
  into `as_of_marker` of every watermark of the source, in the same transaction, and `source.halt_resolved` carries it.
  The next unit compares with that marker, so a standby the bank accepts does not halt the source again. A
  `business_date_regressed` halt cleared by `false_alarm` halts again while the date is still behind: the bank fixes the
  date or decides `restored`.
Test: `false_alarm` after a lower standby marker lets the next unit run.""",
    ],
    '10': [
        """### 7.N Versions with an equal or older business date (ADV-3-06, HIGH; DR-409)
Section 3.7 step 4 closed the current version only when its `valid_from` was earlier. A second change of a party on the
same business date (two units in one day), or a changed row met again in the re-read window with an older business
date, then inserted a second current row; `uq_<table>_current` refused it, the batch failed as `writer_failed` on every
attempt and the table stopped. Step 4 therefore sets the new version's `valid_from = GREATEST(<the row's business date>,
<the current version's valid_from>)`. When it equals the current version's `valid_from`, `close_versions` sets only
`recorded_to` on the current row (its valid range is not shortened); when it is later, step 4 runs as written. The raw
row keeps its own business date, and `recorded_from` records when AKR learned the value. Test: two changes of one
party on one business date give two versions with the same `valid_from`, the first closed in recorded time.""",
    ],
    '12': [
        """### 7.N Known adjustments bind the group and the amount (ADV-3-10, HIGH; DR-409)
`activate` of the `known_adjustment` target also writes `detail.accepted_delta`: the delta of the issue's latest recon
result for its object and field. `DqIssueRepository::accepted_for` takes the field: `accepted_for(conn, source_id,
object, field, kind)`. Section 3.1 step 7 returns `confirmed_gap` only for an accepted issue of the same object and
field whose `abs(accepted_delta)` is at least `abs(delta)`; a larger break stays `open` and step 8 writes a new issue.
Test: a break of 50,000,000 IQD in a group accepted for 1,000 IQD stays `open`.""",
        """### 7.N When a blind period covers the as-of date (ADV-3-07, LOW; DR-409)
Section 3.7 step 5: a blind period covers the as-of date `d` when its `business_from` is NULL or on or before `d`, and its
`business_to` is NULL or on or after `d`. Phase 11 sets `business_from` on every automatic source open (phase 11
section 7+). A paused source is dark through `source_stale` (phase 11 section 7+), because the as-of date is the last
date read, which the pause does not cover.""",
    ],
}

out = {}


def load(p):
    if isinstance(p, str):
        p = PLAN / f'phase-{p}.md'
    return p, out.get(p, p.read_text())


for pp, blocks in BLOCKS.items():
    p, s = load(pp)
    st = s.index('## 7+. Gap Additions')
    nx = s.find('\n## ', st + 5)
    assert 'Adversarial pass (2026-10-10), run 3' not in s[st:nx if nx != -1 else len(s)], pp
    out[p] = add_section7(s, HDR, blocks)

r = PLAN / 'research.md'
p, rs = load(r)
anchor = [l for l in rs.split('\n') if l.startswith('| DR-408 |')]
assert len(anchor) == 1
DR = ("| DR-409 | 2026-10-10 | Adversarial run 3 (watermarks and sources): a paused, bank-paused or refused source is "
      "stale and its blind period has a business_from, with a stated cover rule (phases 11, 12); sequence keys re-read "
      "a key window; a high-water MAX check before each unit halts a source whose data fell below the watermark; late "
      "arrivals are measured and alarmed and `lookback: auto` widens up to a cap; `restored` writes the business date "
      "and `false_alarm` rebases the marker (phase 11); a same-date or back-dated bi-temporal version no longer stalls "
      "the writer (phase 10); known adjustments bind the field and the amount (phase 12) | Each closes a path where "
      "source data was skipped, a period looked covered, a table stalled or a break hid; the endpoint-only halt and "
      "the exclusion precedence were rejected (tools/agent/adv-run3-verdict.md) |")
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + DR + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
