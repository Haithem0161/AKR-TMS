p='docs/API.md'
t=open(p).read()
o=t
def rep(a,b):
    global t
    assert t.count(a)==1,(a,t.count(a))
    t=t.replace(a,b)
rep('| GET | /ops-tasks | ops.tasks | Neutral task list (authority, instruction, clock); no AML reason |',
    '| GET | /ops-tasks | ops.tasks, case.decide | Neutral task list (authority, instruction, clock); no AML reason; A2 and M get the team view (phase 20) |')
rep('| GET | /ops-tasks/{id} | ops.tasks | Task detail |','| GET | /ops-tasks/{id} | ops.tasks, case.decide | Task detail in the caller\'s view (phase 20) |')
rep('| POST | /ops-tasks/{id}/claim | ops.tasks | Claim |','| POST | /ops-tasks/{id}/claim | ops.tasks | Claim (phase 20) |')
rep('| POST | /ops-tasks/{id}/complete | ops.tasks | Complete with the CBS reference |','| POST | /ops-tasks/{id}/complete | ops.tasks | Complete with the CBS reference (phase 20) |')
for a in ['Verification level V0-V3 reached from the canonical copy','Record a V3 attestation with evidence; **SU**',
          'Freeze board (freeze, Committee report, second report clocks; post-order debits)','Freeze detail with letters',
          'Release after delisting on a recorded decision; **SU**, **4E**','Office refrain instructions with the 7-working-day clock',
          'Register an instruction; creates the operations task; **SU**']:
    rep(a+' |',a+' (phase 20) |')
rep('| POST | /exits | case.decide | Propose an exit with individual assessment; starts `exit_decision` |',
    '''| POST | /exits | case.decide | Superseded by phase 15 `POST /exit-register`; not built |
| POST | /ops-tasks/{id}/cannot-complete | ops.tasks | Record that the action cannot be done, with a closure code; raises an incident (phase 20, added) |
| POST | /ops-tasks/{id}/confirm | ops.tasks | Second-person confirmation for V0; refused to the completer (phase 20, added) |
| POST | /ops-tasks/{id}/verification/check | case.decide | Run the canonical-copy check now (phase 20, added) |
| POST | /ops-tasks/{id}/evidence | case.decide | Upload the V3 attestation screenshot (owner kind `ops_task`) (phase 20, added) |
| POST | /freezes/{id}/release-proposals | case.decide | Propose the release of a delisted freeze; starts `freeze_release` (phase 20, added) |
| POST | /freezes/{id}/referral | case.decide | Refer an unresolved match to the Committee's technical committee (letter request); **SU** (phase 20, added) |
| GET | /refrain-instructions/{id} | case.decide | Instruction detail with its task (phase 20, added) |
| POST | /refrain-instructions/{id}/referral | case.decide | Refer to the technical committee (letter request); **SU** (phase 20, added) |
| POST | /refrain-instructions/{id}/close | case.decide | Close on the Office's withdrawal; creates the release task; **SU** (phase 20, added) |
| GET | /incidents | case.decide | Failure incidents (M only) (phase 20, added) |
| POST | /incidents/{id}/close | case.decide | Close an incident with a closure code (M only) (phase 20, added) |

Phase 20 owns the rows marked "phase 20" and ships them with the rows marked "phase 20, added" (22 routes, DR-168). Exit proposals are phase 15's exit register (`/exit-register`, resource kind `exit_decision`).''')
open(p,'w').write(t)
print('ok')
