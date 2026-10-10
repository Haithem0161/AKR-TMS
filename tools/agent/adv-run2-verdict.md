# Adversarial pass run 2: lead verdict (2026-10-10; decisions final, applied by apply-adv-run2.py)

Haiku draft tools/agent/adv-run2.md (alerts under concurrency, replay and late data): 13 candidates; FLAW 11
(CRITICAL 4, HIGH 3, MEDIUM 4), DOUBT 2, GUARDED 14. About 222k tokens.
Lead result: 12 confirmed (merged into 9 fixes: CRITICAL 2, HIGH 3, MEDIUM 3, LOW 1), 1 left open (ADV-2-13 second
path), 14 guarded accepted as checked. No candidate rejected outright; several severities changed.

## Confirmed
- ADV-2-01 CRITICAL (phase 16). `start_once` is `ON CONFLICT DO NOTHING` on `uq_scenario_runs_batch`, which has no
  status predicate, and run step 2 stops on `None`. Step 12 says a failed run retries (five attempts), and the phase 00
  lease reaper re-queues a run whose worker died; both retries stop at step 2, so the chunks after the failure are never
  evaluated. Fix: `start_once` takes over a row that is `failed`, or `running` with `started_at` older than the
  `scenario_run` lease; the restart begins at the first chunk; the sink's equal-digest `Duplicate` absorbs the
  detections already emitted.
- ADV-2-11 CRITICAL, ADV-2-10 HIGH, ADV-2-13 first path (phases 11, 16). `canon.touched` is written per mapping
  transaction, and a batch is touched several times: a pending batch mapped over several runs (`ingest.max_unit_rows`),
  a quarantine retry (section 3.3 step 2 writes no event at all), a mapping replay apply (section 3.8 writes no event).
  The phase 16 ledger is one run per (version, mode, batch), so a second touch is a no-op and rows mapped later are
  never evaluated by the incremental path. Fix: the payload gains `touch_id` (one per writing transaction); retries and
  replay-apply pages emit the event under the step 1 rule; the phase 16 ledger, the job coalesce keys and the fan-out key
  on `touch_id`. The run still reads the whole batch, so rows of an earlier touch stop at the sink as `Duplicate`.
  Other consumers (phase 14 onboarding, phase 15 rating fan-out, phase 19 network edges) re-read the batch and are
  idempotent; unchanged.
- ADV-2-06 HIGH (phase 16, from phase 17 on). `open_bucket` has no state or case filter, so a repeat within the dedupe
  span after the alert's case closed folds into the closed alert: an amendment and a flag on a sealed case, no new alert.
  This contradicts DSL "folds ... into the open alert" and DR-140 (a new alert of a closed case opens a case beside it).
  Haiku said CRITICAL; the flag is kept on the case, so HIGH. Fix: from phase 17 on, `open_bucket` skips a key whose
  alert's case is `closed`.
- ADV-2-08 HIGH (phase 14). `fingerprint ... UNIQUE` is a full constraint; a superseded row keeps blocking its
  fingerprint, so a name that changes and changes back inserts nothing, and the reverted name's match is never reviewed
  with its own forms. Haiku said CRITICAL; the (subject, entry) pair is still in front of an analyst through the newer
  alert, so HIGH. Fix: a partial unique index `WHERE superseded_at IS NULL` and the matching `ON CONFLICT` target.
- ADV-2-03 HIGH (phase 17). The amendment locks only the alert and reads `outcome` under READ COMMITTED; a decision
  holding the case lock and not yet committed is not seen, so the alert changes after the decision with no
  `case.alert_amended_after_decision`. Fix: the amendment locks the case row first (case, then alert: the order of
  attach and of the phase 18 decision), then reads `outcome`.
- ADV-2-02 MEDIUM (phase 17). Two `mark_duplicate` calls lock different alerts; each can name the other, and both leave
  the queue. Fix: lock both alerts in ascending id order; refuse a target whose state is `duplicate`.
- ADV-2-04 MEDIUM (phase 17). `CHECK ((state = 'duplicate') = (duplicate_of IS NOT NULL))` refuses the allowed move
  `duplicate` to `no_longer_matches` unless `duplicate_of` is cleared; section 3.13 does not clear it, so the update
  fails, the bucket aborts (`AlertSinkError::Db`) and the run fails on every retry. Fix: the move clears
  `alert.alerts.duplicate_of`; `casework.case_alerts.duplicate_of` keeps the history.
- ADV-2-05 MEDIUM with ADV-2-12 (phase 17). Assign counts the target's WIP without the `wip:` advisory lock, so a
  concurrent claim by the target exceeds the limit; the assign steps also name no alert row lock, and the bulk real run
  does not re-read `claimed_by`. ADV-2-12 resolved: a single assignment is a push (M-15) and may take over a live claim
  (`alert.assigned {from, to}` names the previous claimer); the bulk run refuses `claimed_by_other` as its dry run says,
  re-read under the row lock. Fix: target `wip:` lock first, then the alert `FOR UPDATE`, then If-Match and the count.
- ADV-2-07 MEDIUM (phase 16). `open_bucket` has a lower bound only, so a settled run of an older window folds into a
  newer bucket and overwrites its measures; the window columns do not follow, and later runs swap the evidence back.
  Fix: an upper bound `until = window_to`.
- ADV-2-09 LOW (phase 14). The transition table refuses a decision on a superseded alert, but no SQL guard holds it and
  the supersede step does not raise `version`. Haiku said HIGH; the rule is stated and step 3 applies the table, so LOW.
  Fix: the supersede update takes undecided, unsuperseded rows only and raises `version`; the 0048 state guard refuses a
  state change on a superseded row.

## Left open
- ADV-2-13 second path (phases 13, 16): `feature_compute` with `reopen_from` recomputes a closed month and writes no
  topic; the settled run of the touch can start before the recompute and read the old feature values. Recorded in phase
  16 section 7+ as an open point for the verification pass (a phase 13 topic or a settled-run delay).

## Guarded (accepted as checked)
Replays, seeded runs and shadow runs never call the sink; equal-digest `Duplicate` on a crash retry; `alert_index`
primary key; quarantine open-identity index; claim and Next locks with SKIP LOCKED; `alerts_guard` transitions; the case
chain guard; merge lock order; clock handlers re-read; snapshots never rewritten; `daily_base` backstop for screening;
terminal `true` and `incorrect`.

DR: DR-408 (the run 2 fixes).
