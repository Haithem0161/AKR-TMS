# Second re-check, phase 00 (Haiku draft for lead review)

## Summary

Entries checked: 4 (F1 to F4).
Findings by severity: HIGH 0, MEDIUM 0, LOW 0.
FINDINGS: 0
CHECKED: F1 ok, F2 ok, F3 ok, F4 ok

Basis (line numbers are docs/plan/phase-00.md unless marked):
- F1: NEW text at line 848. The lease reaper (lines 546-547) moves rows to `queued` and never touches `locked_by`, so the status test stops a late heartbeat. No other heartbeat guard appears in the phase. `complete_in` is defined in 7.18 (line 1041), `jobs.lease_s` at line 244, the `status` enum at line 97, at-least-once in section 3 at line 547.
- F2: NEW text at line 1090. `job.dead` is row 654 of the section 3 audit-event table (section 3 runs from line 469 to 677). No other line sends `job.dead` to section 4.
- F3: NEW text at lines 951-952. Phase 06 line 850, under "## 4. Infrastructure Updates" (line 839), states the same file, read-only mount and target path, and says the phase 00 volume list names it. `model-store` and `AKR_MODEL_STORE` are defined in phase 27 lines 772 and 774. No base line of this phase says the compose stack has no volume change.
- F4: NEW text at line 694. No other line of this phase states the pgBackRest repository as unencrypted. The pgBackRest option name `repo-cipher-type` and value `aes-256-cbc` were not checked against pgBackRest itself; this check used the plan text only.
