# Re-check, phase 00 (Haiku draft for lead review)

### R-00-1  MEDIUM   (fix F6)
- Problem: The heartbeat is guarded by `locked_by` alone. The lease reaper only moves a running row back to `queued` with `last_error`; it does not clear `locked_by`, and the claim sets it (line 442). So a heartbeat from the worker whose job was re-queued still matches the row and refreshes `locked_at`. The fix's premise that such a heartbeat matches no row and changes nothing does not hold as written. It needs a status guard in the heartbeat, or a reaper step that clears `locked_by`.
- Proof: docs/plan/phase-00.md:848 "the update is guarded by `locked_by`. A heartbeat that matches no row (the reaper re-queued the job) changes nothing" and docs/plan/phase-00.md:547 "go back to `queued` with `last_error = 'lease expired'`" (the reaper step names no `locked_by` change).
- Smallest fix: In the 7.2 heartbeat text (line 848), state the guard as `WHERE id = $1 AND status = 'running' AND locked_by = $2` (the status guard that `complete_in` uses in 7.18), or add "and clears `locked_by`" to the reaper step at line 547.

### R-00-2  LOW   (fix F7)
- Problem: The `job.dead` event is listed in the "Audit events emitted" table of section 3. Section 4 never mentions it, so the pointer "(section 4)" sends the reader to the wrong section.
- Proof: docs/plan/phase-00.md:1089 "`job.dead` is the one harness event (section 4)" and docs/plan/phase-00.md:654 "| `job.dead` | a job reaching `dead` (kind, attempts; no payload) | job id |".
- Smallest fix: In the 7.24 text at line 1089, change "(section 4)" to "(section 3, the audit-event table)".

## Summary

- Fixes checked: 15 (F1 to F15 of tools/agent/verify/applied-00.txt).
- Findings: HIGH 0, MEDIUM 1, LOW 1.

CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 R-00-1, F7 R-00-2, F8 ok, F9 ok, F10 ok, F11 ok, F12 ok, F13 ok, F14 ok, F15 ok
