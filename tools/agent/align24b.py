p='docs/API.md'
s=open(p).read()
R=[
("""| GET | /records/{kind}/{id}/access-history | audit.read | Who read or changed a record, with purpose |
""",
"""| GET | /records/{kind}/{id}/access-history | audit.read | Who read or changed a record, with purpose; masked 404 for a suspicion-tier record outside M (phase 24) |
| GET | /access/control-report | dashboard.read, security.ops | Monthly tipping-off control report, counts only, small counts suppressed (phase 24, added) |
"""),
("""| GET | /audit/events | audit.read | Audit events (filters, cursor); the auditor's own search is logged |
| GET | /audit/chain-status | audit.read | Hash-chain and anchor status per partition |
| GET | /audit/anchors | audit.read | Signed Merkle anchors |
| POST | /audit/verify-runs | audit.read | Verify chains and anchors over a range |
| GET | /audit/verify-runs/{id} | audit.read | Result |
""",
"""| GET | /audit/events | audit.read | Audit events (filters, cursor, `purpose` required); every search is logged as `audit.searched`; suspicion-tier rows are left out for IA and SEC (phase 24) |
| GET | /audit/events/{id} | audit.read | One event with its chain position; masked 404 for a suspicion-tier row; the key carries `occurred_at` (phase 24, added) |
| GET | /audit/chain-status | audit.read | Hash-chain and anchor status per partition and stream (phase 24) |
| GET | /audit/anchors | audit.read | Signed Merkle anchors, cursor (phase 24) |
| POST | /audit/verify-runs | audit.read | Queue a verify run over a range; `Idempotency-Key`; 202 (phase 24) |
| GET | /audit/verify-runs/{id} | audit.read | Status and result, counts and finding kinds only (phase 24) |
"""),
("""| GET | /registers/{kind} | dashboard.read | Registers: decisions, exclusions, findings, warnings, fairness |
| POST | /registers/{kind}/entries | config.author | Add an entry |
""",
"""| GET | /registers/{kind} | dashboard.read | Registers: `decisions` and `exclusions` (read views through their owners' predicates), `findings`, `warnings`, `fairness`; cursor (phase 24) |
| POST | /registers/{kind}/entries | config.author, config.approve | Add a finding (M, CH, MRO); any other kind is 422; `Idempotency-Key` (phase 24) |
| GET | /registers/{kind}/entries/{id} | dashboard.read | One finding or warning with its history (phase 24, added) |
| PATCH | /registers/{kind}/entries/{id} | config.author, config.approve | Owner, due date, severity, status, closure code; `If-Match`; **SU** class `approval` for `accepted_risk`, never by the finding's owner (phase 24, added) |
| GET | /registers/review-calendar | dashboard.read | Review dates of rules, exclusions, change requests, readiness and scenarios, amber and red; derived, writes nothing (phase 24, added) |
| POST | /fairness/metrics/{id}/review | config.author, config.approve | Justify or change a group rate in review status; `If-Match` (phase 24, added) |
| GET | /fairness/attributes | dashboard.read | Protected-attribute register and allowed-use matrix (phase 24, added) |
| POST | /inventory/{id}/validation-packs | model.risk | Issue a validation pack (seven parts, computed verdict); the validator is the caller, never the item owner; `Idempotency-Key` (phase 24, added) |
| GET | /inventory/{id}/validation-packs | model.risk, audit.read | Versions of an item's validation packs, cursor (phase 24, added) |
| GET | /validation-packs/{id} | model.risk, audit.read | One validation pack with its parts and evidence hash (phase 24, added) |
"""),
("""| GET | /dashboard/command-centre | dashboard.read | What is late, decisions waiting, clocks, arrivals against throughput, incidents |
| GET | /dashboard/clocks | dashboard.read | All regulatory and internal clocks with status |
| POST | /packs | dashboard.read, data.steward | Build a pack, a job (kinds `cbi_gap`, `vendor_request` and `integration_profile` [P0]; board, case, report, assessor, correspondent, validator [phase 24]) |
| GET | /packs/{id} | dashboard.read, data.steward | Pack metadata and build status [P0] |
| GET | /packs/{id}/download | dashboard.read, data.steward | Download (access event); **SU** [P0] |
| POST | /packs/verify | self | Verify a pack or document hash against the vault |
| GET | /assessor/d2-matrix | assessor.read | D2 test-script map with live evidence links |
| POST | /assessor/samples | assessor.read | Draw a random sample (recorded) |
| GET | /assessor/trace/{alert_id} | assessor.read | End-to-end trace: transaction, alert, case, report |
""",
"""| GET | /dashboard/command-centre | dashboard.read | Tiles with definition, formula, data-as-of and the list each opens; live, `no-store`, counts 1 to 4 suppressed (phase 24) |
| GET | /dashboard/clocks | dashboard.read | All clock instances from their owners' due columns, keyset by due instant; read-only (phase 24) |
| POST | /packs | dashboard.read, data.steward, assessor.read | Build a pack, a job (kinds `cbi_gap`, `vendor_request` and `integration_profile` [P0]; `board`, `case`, `report`, `assessor`, `certification` [phase 24]; `correspondent`, `validator` [P2]); AS builds kind `assessor` only |
| GET | /packs/{id} | dashboard.read, data.steward, assessor.read | Pack metadata and build status [P0; `assessor.read` phase 24] |
| GET | /packs/{id}/download | dashboard.read, data.steward, assessor.read | Download (access event); **SU** class `export` [P0; `assessor.read` phase 24] |
| POST | /packs/verify | self | Verify a pack or document hash against the vault and the audit chain; `verified: false` alike for unknown and unreadable (phase 24) |
| GET | /assessor/d2-matrix | assessor.read | D2 clause map (signed data file) with live evidence links and a status per clause (phase 24) |
| POST | /assessor/samples | assessor.read | Draw a seeded, recorded random sample from a frame; `Idempotency-Key` (phase 24) |
| GET | /assessor/samples/{id} | assessor.read | A recorded sample with its seed, frame hash and items (phase 24, added) |
| GET | /assessor/trace/{alert_id} | assessor.read | End-to-end trace, metadata only: CBS row, transaction, alert, case, decision type, report; a suspicion-tier alert only as an item of a sample drawn under the grant in force (phase 24) |
"""),
("""| GET | /training/register | dashboard.read | Training register |
| POST | /training/register | admin.users | Record training |
""",
"""| GET | /training/register | dashboard.read | Training register, cursor, filter by user and course (phase 24) |
| POST | /training/register | admin.users | Record a training completion; `Idempotency-Key`; never for oneself (phase 24) |
| POST | /training/register/{id}/void | admin.users | Void a record with a reason code (phase 24, added) |
"""),
("""| GET | /typology-exposure | dashboard.read | Typology exposure index by bank, branch, correspondent [P2] |
""",
"""| GET | /typology-exposure | dashboard.read | Typology exposure index by bank, branch, correspondent [P2] |

Phase 24 owns the rows marked "phase 24" here and in 3.3, 3.4 and 3.15 (docs/plan/phase-24.md s.2; 31 rows, 28 new,
the three pack routes are phase 06's extended; DR-218 to DR-231).
"""),
]
for a,b in R:
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
open(p,'w').write(s)
print('ok',len(R))
