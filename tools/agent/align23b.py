p='docs/plan/phase-23.md'
s=open(p).read()
for a,b in [
("`latest_report(ctx) -> RetentionReportView` (section 3.6).","`latest_report(ctx) -> RetentionReportView` (section 3.10)."),
("from the latest completed run (section 3.6); `held` is `null` for IA and SEC |","from the latest completed run (section 3.10); `held` is `null` for IA and SEC |"),
]:
    assert s.count(a)==1,a; s=s.replace(a,b)
open(p,'w').write(s)
p='docs/API.md'
s=open(p).read()
old_start="| GET | /retention/holds | audit.read | Legal holds |\n"
i=s.index(old_start); j=s.index("| GET | /retention/certificates | audit.read | Deletion certificates |\n")+len("| GET | /retention/certificates | audit.read | Deletion certificates |\n")
new="""| GET | /retention/holds | audit.read | Legal holds, cursor page; for IA and SEC case holds are omitted and party holds show scope `restricted` (phase 23) |
| POST | /retention/holds | report.file | Place a legal hold (party, case or range); **SU** class `decision`; `Idempotency-Key` (phase 23) |
| GET | /retention/holds/{id} | audit.read | One hold; a case hold is `E_NOT_FOUND` for everyone but M (phase 23, added) |
| POST | /retention/holds/{id}/release | report.file | Start the workflow `legal_hold_release`; `If-Match`; the approval is the phase 04 task action with **SU** and **4E** (phase 23) |
| GET | /retention/report | audit.read | What is due by class and what is held, from the latest completed run; `held` is null for IA and SEC (phase 23) |
| POST | /retention/dry-runs | admin.system | Simulate a retention run; 202 with the run id; nothing deleted (phase 23) |
| GET | /retention/runs | audit.read | Runs, newest first, dry and real (phase 23, added) |
| GET | /retention/runs/{id} | audit.read | One run with its items; `held` is null for IA and SEC (phase 23, added) |
| GET | /retention/certificates | audit.read | Deletion certificates: run, class, kind, counts, digest, signature; never a clear key or value (phase 23) |

Phase 23 owns the rows marked "phase 23" (docs/plan/phase-23.md s.2; 9 routes, DR-184); the `/audit` and `/evidence`
rows above are phase 02's capture and phase 24's viewer and verifier.
"""
s=s[:i]+new+s[j:]
open(p,'w').write(s)
print('ok')
