# Third re-check, phase 30 (Haiku draft for lead review)

### T-30-1  LOW   (fix F3, [S-30-3])
- Problem: Section 3.1 says the phase 00 file `deploy/zones.yaml` already holds the entry `backup.repository`, but the F3 bullet of section 4.4 says phase 30 makes zones.yaml gain that key. No phase 00 line names `backup.repository` (grep of docs/plan/phase-*.md finds it only in phase 30), so the two lines disagree on when the entry is added. The renderer reads it either way, so the effect is small.
- Proof: docs/plan/phase-30.md:77 "and the entry `backup.repository`: the host and port of the pgBackRest repository on the backup server, which section 7.5 reads" and docs/plan/phase-30.md:471 "gains four keys that the renderer reads (`cluster.apiserver`, `cbs.listeners`, `gateway.cidrs`, and `backup.repository` of section 7.5)".
- Smallest fix: In line 77, delete the clause "and the entry `backup.repository`: the host and port of the pgBackRest repository on the backup server, which section 7.5 reads", and in line 471 add "(the host and port of the pgBackRest repository on the backup server)" after "`backup.repository` of section 7.5", so only section 4.4 says when the entry is added.

## Summary
- Entries checked: 3 (F1, F2, F3).
- Findings by severity: HIGH 0, MEDIUM 0, LOW 1.
CHECKED: F1 ok, F2 ok, F3 T-30-1
