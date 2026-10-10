import re
# ---- research DR rows
p='docs/plan/research.md'
s=open(p).read()
rows=[
("DR-338","Phase 31: the training workspace lives in its own schema `train` (practice cases and attempts), written only by `TrainingService` and `akr-ctl training seed`; no object outside `train` references it","UX-41; a flag column on live tables needs an exclusion in every count, report, export and filing"),
("DR-339","Practice results reach the phase 24 register as a display column only (`train.practice_scores`, scored on each trainee's first attempt per case); a practice pass never creates a training record, an administrator may cite `practice:<attempt id>`","M-51 part; phase 24 `user_id <> recorded_by` (lead review: a pass after the answer is shown is not a pass)"),
("DR-340","Practice content comes from the typology library and the phase 16 seeded fixture shape, every value invented and marked `synthetic: true`, checked by the seed against live identifiers; the phase 28 golden set is not a source","Tipping-off and data protection; the golden set is live QA evidence"),
("DR-341","The capacity workbench measures teams, not people: inputs per pool and bank week, handling times measured from phase 17 timestamps in an hourly snapshot with no person column; no default handling time; alerts attributed to the first active `alerts` queue whose filter admits them (phase 31 rule), else an unassigned total","M-52, research 28 section 11 (lead review: the attribution is not a phase 17 rule)"),
("DR-342","The simulation harness reuses a completed phase 16 replay (daily would-alert counts, no entity id or detection key), splits by a stated class mix and runs a priority-ordered fluid queue; results are labelled estimates","M-76; a second replay engine would drift"),
("DR-343","Surge mode carries a temporary pool, a higher QC sample (`capacity.surge_qc_percent`) and hourly progress, switched by change request with the `capacity` tag; pre-clearing known false positives is not carried","M-54, ARCHITECTURE D21 (no capacity output suppresses an alert)"),
("DR-344","`capacity_forecast` alarm through phase 19 `AlarmService` (subject `pool`, supervisor M, notice `supervisor.alarm`); the growth, latency and silence alarms stay unbuilt","M-53 part; phase 19 Open doubts 10"),
("DR-345","Correspondent and validator packs are built by the phase 06 `pack_build` job from system facts with source and `data_as_of`; both kinds already in `packs_kind_check` (asserted by 0100); a case reference in a fact fails the build","W-06, GV-35; questionnaire text is the bank's (phase 19 DR-149)"),
("DR-346","Typology exposure index per month, scope (bank, branch, correspondent account) and typology: matched posted flows and LCY value against totals and month-end balances; restricted scenarios excluded; small numbers suppressed","S-31, SCN-CORR-07; T-04 never adds IQD and USD"),
("DR-347","The RFI desk gains the correspondent list and the attachment of a built pack to an outbound correspondent request in draft; no case detail shown; release stays the phase 19 external release","W8; phase 19 DR-148"),
("DR-348","SAML through Ory Polis as an optional catalogue entry (flag `pack.polis`) presenting the bank IdP as the phase 25 generic OIDC upstream; no authority from the assertion; own database and secrets; the edge exposes only the pinned authorize, assertion consumer and SCIM paths and refuses the webhook route","TS-25, research 32 section 14.3 (lead review: the edge paths)"),
("DR-349","SCIM creates no sign-in account: a create writes a `pending` directory row; an administrator creates the account (phase 03) and links it with step-up `identity_admin`; roles only by `cr_access`","SP-14; identity creation is an administrator act"),
("DR-350","A SCIM deactivation disables the linked account through the phase 03 disable (`UserAdminService::disable_from_directory`, `source: directory`, grants kept, reversible); it never offboards; the final offboarding stays an administrator's act with step-up, listed by the reconciliation","Phase 03 offboarding is final (lead review: an unattended external signal never triggers an irreversible act)"),
("DR-351","The directory is authoritative for deactivation only: a local enable is refused while the directory row is inactive, a local offboarding wins over an activation, groups never change grants, an e-mail change is a recorded mismatch; daily reconciliation with counts only","G-12; local authority stays with phase 03"),
("DR-352","`POST /directory/events` is an HMAC-verified webhook (pinned Polis signature format, secret `kv/polis/directory`, timestamp window `directory.event_max_age_s`); principal `polis-directory` with the one scope `directory.sync`, `Ctx.via = webhook_hmac`; every verified event answers 200 with its outcome","A retried webhook must not repeat a decision"),
("DR-353","Scopes `capacity.plan` (M) and `directory.sync` (the one service principal); Cedar files E0_directory, E1_training_capacity, E2_exposure_packs; route scopes admit every role the policies permit","Phase 03 Cedar ranges; phase 24 scope rule"),
("DR-354","Five flags, all off: `pack.training`, `pack.capacity`, `pack.exposure`, `pack.correspondent`, `pack.polis`; a route whose flag is off answers 404 `E_NOT_FOUND`","Phase 26 and 29 pattern; each part is a bank choice"),
("DR-355","Screens: W12 training workspace and G2 capacity workbench are new; W8, R4, G1, G9 and A1 gain panels, columns or kinds","DESIGN-SYSTEM rows; UX-41, M-52"),
]
anchor=re.search(r'^\| DR-337 \|.*\n',s,re.M)
assert anchor and '| DR-338 |' not in s
s=s[:anchor.end()]+''.join(f"| {d} | 2026-10-09 | {a} | {b} |\n" for d,a,b in rows)+s[anchor.end():]
open(p,'w').write(s)

# ---- API.md
p='docs/API.md'
s=open(p).read()
def rep(a,b):
    global s
    assert s.count(a)==1,(s.count(a),a[:80]); s=s.replace(a,b)
rep("| `admin.users` | Users, grants, delegations, offboarding | ADM |\n",
    "| `admin.users` | Users, grants, delegations, offboarding, directory links | ADM |\n")
rep("| `assess.call` | `/assess/v1` only | Gateway client certificate |\n",
    "| `assess.call` | `/assess/v1` only | Gateway client certificate |\n"
    "| `capacity.plan` | Capacity inputs and simulation runs (phase 31, DR-353) | M |\n"
    "| `directory.sync` | `POST /directory/events` only; HMAC-verified webhook (phase 31, DR-352) | Service principal `polis-directory` only |\n")
rep("""| GET | /capacity/forecast | dashboard.read | Capacity workbench (arrivals, throughput, staffing) [P2] |
| GET | /typology-exposure | dashboard.read | Typology exposure index by bank, branch, correspondent [P2] |

Phase 24 owns the rows marked "phase 24" here and in 3.3, 3.4 and 3.15 (docs/plan/phase-24.md s.2; 31 rows, 28 new,
the three pack routes are phase 06's extended; DR-218 to DR-231).
""",
"""| GET | /training/practice/cases | queue.work | Active synthetic practice cases of the caller's paths, no answers; flag `pack.training` (phase 31, added) |
| GET | /training/practice/cases/{id} | queue.work | One practice case: facts, question, reason list; no known answer (phase 31, added) |
| POST | /training/practice/cases/{id}/attempts | queue.work | Submit a decision or choice; scored; returns the known answer and expert path; `Idempotency-Key` (phase 31, added) |
| GET | /training/practice/attempts | queue.work | The caller's own practice attempts (phase 31, added) |
| GET | /capacity/inputs | dashboard.read | Staffing inputs by pool and bank week (phase 31, added) |
| POST | /capacity/inputs | capacity.plan | Record or supersede the input of a pool and week; `If-Match`; `Idempotency-Key` (phase 31, added) |
| GET | /capacity/forecast | dashboard.read | Workload, backlog projection, sensitivity, surge progress; team-level only, `no-store` [P2] (phase 31) |
| POST | /capacity/simulations | capacity.plan | Simulation harness on a completed phase 16 replay; 202 (phase 31, added) |
| GET | /capacity/simulations/{id} | dashboard.read | Simulation status and result, estimates labelled (phase 31, added) |
| GET | /typology-exposure | dashboard.read | Typology exposure index by bank, branch, correspondent, with trend; suppressed cells null [P2] (phase 31) |

Phase 24 owns the rows marked "phase 24" here and in 3.3, 3.4 and 3.15 (docs/plan/phase-24.md s.2; 31 rows, 28 new,
the three pack routes are phase 06's extended; DR-218 to DR-231).

Phase 31 owns the `/training/practice`, `/capacity` and `/typology-exposure` rows here, the two correspondent rows in
3.12 and the three `/directory` rows in 3.2 (docs/plan/phase-31.md s.2; 15 rows, 13 added; DR-338 to DR-355). The
`correspondent` and `validator` kinds of `POST /packs` are built by phase 31 (flag `pack.correspondent`). Every phase 31
route is 404 `E_NOT_FOUND` while its flag is off. Capacity output is advice and never caps, raises or suppresses an
alert (D21).
""")
rep("| POST | /rfis/{id}/extend | rfi.desk | One extension by working days, two at most (phase 19, added) |\n",
    "| POST | /rfis/{id}/extend | rfi.desk | One extension by working days, two at most (phase 19, added) |\n"
    "| GET | /rfis/correspondents | rfi.desk | Correspondent requests in and out: deadline, progress, recipient account id; no case detail; flag `pack.correspondent` (phase 31, added) |\n"
    "| POST | /rfis/{id}/packs | rfi.desk | Attach a correspondent or validator pack to an outbound correspondent request in draft; `If-Match` (phase 31, added) |\n")
rep("| POST | /admin/access-reviews/{id}/sign | security.ops | Sign once every item is decided; signer is not the starter; **SU** `approval` (phase 25) |\n",
    "| POST | /admin/access-reviews/{id}/sign | security.ops | Sign once every item is decided; signer is not the starter; **SU** `approval` (phase 25) |\n"
    "| POST | /directory/events | directory.sync | Directory event from Ory Polis (create, update, deactivate, activate); HMAC-verified webhook, not reachable through the edge; a deactivation disables, never offboards; flag `pack.polis` (phase 31, added) |\n"
    "| GET | /directory/users | admin.users | Directory users with link state; names to ADM only, each read an access event (phase 31, added) |\n"
    "| POST | /directory/users/{id}/link | admin.users | Link a pending directory user to an active account with the same e-mail; **SU** `identity_admin`; `If-Match` (phase 31, added) |\n")
open(p,'w').write(s)

# ---- phase 31 s.4.5 item 13: where the rows went
p='docs/plan/phase-31.md'
s=open(p).read()
rep("the routes of section 2 of this phase are added to API.md (section 3.20, tags `training`, `capacity`, `exposure`, `directory`, and the two `rfis` rows).",
    "the routes of section 2 of this phase are added to API.md (training, capacity and exposure rows in section 3.16, the two `rfis` rows in 3.12, the three `directory` rows in 3.2).")
open(p,'w').write(s)
print('ok')
