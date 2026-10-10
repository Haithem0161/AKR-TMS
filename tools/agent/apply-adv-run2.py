#!/usr/bin/env python3
"""Apply adversarial pass run 2 (alerts under concurrency, replay and late data) after the lead's checks.

Source: tools/agent/adv-run2.md (Haiku draft); decisions: tools/agent/adv-run2-verdict.md.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'

HDR = ('Adversarial pass (2026-10-10), run 2 (alerts under concurrency, replay and late data), from '
       'tools/agent/adv-run2.md; lead-verified (tools/agent/adv-run2-verdict.md).')

BLOCKS = {
    '11': [
        """### 7.N Every canonical write is a touch (ADV-2-10, ADV-2-11, ADV-2-13; CRITICAL; DR-408)
A batch can reach canon in several transactions: a pending batch mapped over several runs (`ingest.max_unit_rows`), a
quarantine retry, and a page of a mapping replay apply. The payload of `canon.touched` therefore gains `touch_id`
(UUIDv7, one per writing transaction): `{batch_id, touch_id, source_id, source_table, entity_kinds, business_date}`.
1. Section 3.3 step 1 writes a new `touch_id` in each mapping transaction, so a batch mapped over three runs is touched
   three times.
2. Section 3.3 step 2 (retries) writes `canon.touched` in the transaction that wrote a retried row, when the writer
   inserted or changed a row, with the row's own `batch_id` and business date.
3. Section 3.8 `ingest_mapping_replay_apply` step 1 writes it for each page under the same rule, with the page receipt
   as `batch_id`.
The consumers that key on the batch (phase 14 onboarding, phase 15 rating fan-out, phase 19 network edges) re-read the
batch and are idempotent; they ignore `touch_id`. Phase 16 keys its run ledger on it (phase 16 section 7+). Test: a
`row_failed` row retried after a mapping publication writes one `canon.touched` with a new `touch_id`.""",
    ],
    '16': [
        """### 7.N One run per touch, not per batch (ADV-2-10, ADV-2-11, ADV-2-13; CRITICAL; DR-408)
Phase 11 now touches a batch once per writing transaction and names each touch by `touch_id` (phase 11 section 7+).
Migration `0055` adds `touch_id UUID` to `scn.scenario_runs` with `CHECK (touch_id IS NULL OR batch_id IS NOT NULL)`,
and `uq_scenario_runs_batch` becomes `ON scn.scenario_runs (scenario_version_id, mode, touch_id) WHERE touch_id IS NOT
NULL`. The fan-out passes `touch_id` into every `scenario_run` job it enqueues (incremental, settled, shadow); the
coalesce keys become `scenario_fanout:<touch_id>` and `scenario_run:<version>:<mode>:<touch_id or window_to>`. A second
delivery of the same event is still a no-op; a new touch of the same batch starts its own run. The run still reads
every row of the batch (fan-out step 2): rows of an earlier touch stop at the sink as `Duplicate` (equal digest), and a
row mapped late, a later slice or a retried row is evaluated for the first time. A touch whose rows have a booking date
before the newest evaluated window takes the late-data path of section 3.6 (the settled run). Test: a batch mapped in
two runs gives two `completed` incremental runs, and the second run's new detections are `New`.""",
        """### 7.N A retried run takes over its own row (ADV-2-01, CRITICAL; DR-408)
Run step 12 retries a failed run, and the phase 00 lease reaper re-queues a run whose worker died, but step 2 stopped on
any existing ledger row, so the chunks after the failure were never evaluated. `RunRepository::start_once` is therefore
`INSERT ... ON CONFLICT (scenario_version_id, mode, touch_id) WHERE touch_id IS NOT NULL DO UPDATE SET status =
'running', error_code = NULL, started_at = now(), finished_at = NULL, entities = 0, detections = 0, amendments = 0,
version = scn.scenario_runs.version + 1 WHERE scn.scenario_runs.status = 'failed' OR (scn.scenario_runs.status =
'running' AND scn.scenario_runs.started_at < now() - <the scenario_run lease>) RETURNING id`, where the lease is
`jobs.lease_s` or the kind's override in the registry (the bound of the phase 00 reaper). `None` now means the touch was
run (`completed`, `refused_cost`) or is running under a live lease. A takeover restarts at the first chunk; detections
already emitted stop at the sink as `Duplicate`, so no alert is written twice. The earlier attempt's
`scenario.run_failed` stays in the audit. After the last attempt the run stays `failed` and the scenario shows "not
operating as intended" (section 3.16). Test: a run failed at chunk 2 of 4 completes on the retry with every group
evaluated.""",
        """### 7.N The fold stays inside its window and never reaches a closed case (ADV-2-06 HIGH, ADV-2-07 MEDIUM; DR-408)
`DetectionKeyRepository::open_bucket` takes an upper bound: `open_bucket(tx, scenario, anchor_hk, since = window_to -
days + 1, until = window_to)` returns the newest key of the anchor whose bucket starts between `since` and `until`
inclusive. A settled run of an older window then gets its own bucket and no longer overwrites the evidence of a newer
alert. From phase 17 on, the query also skips a key whose alert (`alert.alert_index` to `alert.alerts`) sits in a case
(`casework.case_alerts` to `casework.cases`) with status `closed`: a repeat after closure gets a new bucket and a new
fingerprint, the sink returns `New`, and phase 17 section 3.5 opens a case beside the closed one with `case.linked_prior`
(DR-140). This is the DSL rule that repeats fold "into the open alert". Late rows for the window that created a closed
case's alert still reach that alert when its bucket is that window's end, and write `case.alert_amended_after_decision`
(phase 17 section 3.13). Tests: a repeat four days after the case closed opens a new case with `case.linked_prior`; a
settled run of 30 May beside a 1 June bucket leaves the 1 June alert unchanged.""",
        """### 7.N Features recomputed for a reopened month (ADV-2-13; open)
A replay of a closed month makes `feature_compute` reopen it (phase 13 section 3.2), and phase 13 writes no topic. The
settled run of the touch can start before the recompute and read the old feature values for that month. This is left
open for the verification pass: the two options are a phase 13 topic on a recomputed month that phase 16 consumes, or a
settled-run start after the next `feature_compute` slot.""",
    ],
    '17': [
        """### 7.N Amendments take the case lock (ADV-2-03, HIGH; DR-408)
Section 3.13, every bullet: under the bucket fold's advisory lock, resolve the alert's case through
`casework.case_alerts`, lock the case row `FOR UPDATE`, then lock the alert (case, then alert: the order of attach in
section 3.5 and of the phase 18 decision). `outcome` and the decision events are read after both locks. A decision that
committed first is therefore seen, and the amendment writes `case.alert_amended_after_decision`; a decision still open
waits on the case lock. Test: a decision and an amendment of one of its alerts started together always end with either
the flag or the amendment before the decision.""",
        """### 7.N Leaving the duplicate state clears the link (ADV-2-04, MEDIUM; DR-408)
The move `duplicate` to `no_longer_matches` (section 3.13) sets `alert.alerts.duplicate_of = NULL` in the same update,
as the CHECK `(state = 'duplicate') = (duplicate_of IS NOT NULL)` requires; the event `case.alert_no_longer_matches`
carries the earlier `duplicate_of`. `casework.case_alerts.duplicate_of` is kept, so the case history still shows the
mark. Matches again returns the alert to `queued` with `duplicate_of` NULL. Test: a duplicate alert whose detection no
longer matches moves to `no_longer_matches` and the run completes.""",
        """### 7.N A duplicate target is a live alert (ADV-2-02, MEDIUM; DR-408)
Triage `mark_duplicate` and each bulk `mark_duplicate` item lock both alerts `FOR UPDATE` in ascending `id` order, then
refuse a target whose state is `duplicate` (422 `E_VALIDATION` at `/duplicate_of`, reason `target_is_duplicate`). Two
analysts marking A and B as duplicates of each other are then serial, and the second is refused. Test: the two marks
started together leave one alert `duplicate` and one in the queue.""",
        """### 7.N Assignment locks and a live claim (ADV-2-05, ADV-2-12, MEDIUM; DR-408)
Assign (alert), in this order: `pg_advisory_xact_lock(hashtext('wip:' || target_user_id))` (the key of claim step 3),
then the alert `FOR UPDATE`, then `If-Match` on the alert version (the route row's `If-Match`), then the target's WIP
count and conflict check. A claim by the target and an assignment to the target are then serial and the limit holds. A
single assignment is a push (M-15): it may take over an alert another person has claimed, and `alert.assigned {from,
to}` names that person. The bulk real run takes the target's `wip:` lock once before its items, re-reads `claimed_by`
under each item's row lock and refuses `claimed_by_other` as the dry run does. Test: nine claimed alerts, an assignment
and a claim to the same analyst at once: one is 409 `E_WIP_LIMIT`.""",
    ],
    '14': [
        """### 7.N Superseded alerts free their fingerprint (ADV-2-08, HIGH; DR-408)
In migration `0046` the column is `fingerprint BYTEA NOT NULL CHECK (octet_length(fingerprint) = 32)` without
`UNIQUE`, and `CREATE UNIQUE INDEX IF NOT EXISTS uq_screening_alerts_fingerprint ON alert.screening_alerts (fingerprint)
WHERE superseded_at IS NULL;` holds the dedupe key. `AlertRepository::insert_candidates` uses `ON CONFLICT
(fingerprint) WHERE superseded_at IS NULL DO NOTHING`. A decided alert is never superseded, so a decided fingerprint
still blocks a rerun; a name that changes and changes back inserts a new alert for the restored name, and section 3.5
step 4 supersedes the undecided alert of the middle name. The section 6 duplicate check reads `SELECT fingerprint FROM
alert.screening_alerts WHERE superseded_at IS NULL GROUP BY 1 HAVING count(*) > 1`. Test: N1, then N2, then N1 again
leaves one open alert, carrying the N1 forms.""",
        """### 7.N A superseded alert is refused in SQL (ADV-2-09, LOW; DR-408)
Section 3.5 step 4 supersedes with `WHERE decided_at IS NULL AND superseded_at IS NULL` and sets `version = version +
1`, so an `If-Match` read before the supersede is 409 `E_VERSION_CONFLICT`. Decide step 3 reads `superseded_at` in the
step 2 row. The guard `alert.screening_state_guard` (migration `0048`) adds: `IF OLD.superseded_at IS NOT NULL AND
(NEW.state IS DISTINCT FROM OLD.state OR NEW.decided_at IS DISTINCT FROM OLD.decided_at) THEN RAISE EXCEPTION 'a
superseded alert takes no decision' USING ERRCODE = 'check_violation'; END IF;`. Test: a decision with the pre-supersede
version is 409.""",
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
    assert 'Adversarial pass (2026-10-10), run 2' not in s[st:nx if nx != -1 else len(s)], pp
    out[p] = add_section7(s, HDR, blocks)

r = PLAN / 'research.md'
p, rs = load(r)
anchor = [l for l in rs.split('\n') if l.startswith('| DR-407 |')]
assert len(anchor) == 1
DR = ("| DR-408 | 2026-10-10 | Adversarial run 2 (alerts): `canon.touched` carries a `touch_id` and is written by "
      "every canonical write, including quarantine retries and replay-apply pages (phase 11); the scenario run ledger "
      "keys on the touch and a retried run takes over its failed or expired row (phase 16); the bucket fold has an "
      "upper bound and never folds into an alert of a closed case (phase 16); amendments lock the case before the "
      "alert, leaving `duplicate` clears the link, a duplicate target must be live, and assignment takes the target's "
      "WIP lock and the row lock (phase 17); superseded screening alerts free their fingerprint and take no decision "
      "(phase 14); recomputed features of a reopened month left open | Each closes a path where a true detection was "
      "lost, a decided alert changed unseen, or a run failed on its own data; the rest were guarded "
      "(tools/agent/adv-run2-verdict.md) |")
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + DR + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
