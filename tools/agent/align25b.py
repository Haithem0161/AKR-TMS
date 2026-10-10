p='docs/API.md'
s=open(p).read()
def rep(a,b):
    global s
    assert s.count(a)==1,(s.count(a),a[:80])
    s=s.replace(a,b)
# section 2 scopes
rep("| `support.readonly` | A support session opened by the bank (time-boxed, redacted) | VS |\n",
"| `support.readonly` | A support session opened by the bank (time-boxed); no route requires it: `VS` reads only its own profile (`self`; phase 25, DR-237) | VS |\n"
"| `siem.pull` | `GET /siem/events` only; the route is not mounted for a client without it (phase 25, DR-245) | Service principals (`ClientCredentials`) only |\n")
# 3.2
rep("| GET | /admin/access-reviews | security.ops | Quarterly access reviews |\n| POST | /admin/access-reviews | security.ops | Start a review (snapshot of grants) |\n| POST | /admin/access-reviews/{id}/sign | security.ops | Sign the review; **SU** |\n",
"| GET | /admin/access-reviews | security.ops, dashboard.read | Quarterly access reviews, newest first; `dashboard.read` admits M, who decides business-role items (phase 25) |\n"
"| GET | /admin/access-reviews/{id} | security.ops, dashboard.read | Review detail: state, snapshot identity, items with their decisions, paged (phase 25, added) |\n"
"| POST | /admin/access-reviews | security.ops | Start a review; 202, the snapshot of `AccessMatrixBuilder::build(at)` is built by job `access_review_snapshot` as an evidence object (phase 25) |\n"
"| POST | /admin/access-reviews/{id}/items/{item_id} | security.ops, dashboard.read | Confirm, flag or propose the revocation of one grant; never one's own grant; a revocation is a **CR** on `cr_access_adm` (`role_grant` op `revoke`); `If-Match` (phase 25, added) |\n"
"| POST | /admin/access-reviews/{id}/sign | security.ops | Sign once every item is decided; signer is not the starter; **SU** `approval` (phase 25) |\n")
# 3.3
rep("| POST | /access/break-glass | self | Request break-glass with reason and scope; **SU** |\n| POST | /access/break-glass/{id}/approve | security.ops | Approve (two approvers, at most four hours); **SU**, **4E** |\n| POST | /access/break-glass/{id}/end | self | End early |\n",
"| GET | /access/break-glass | self, security.ops, dashboard.read | Requests and grants with status and review state; a requester sees own rows only; no case id outside the requester's own rows (phase 25, added) |\n"
"| POST | /access/break-glass | self | Request read-only access to one case with a reason code (A2 only); **SU** `break_glass`; the same 202 whether or not the case exists (phase 25) |\n"
"| POST | /access/break-glass/{id}/approve | security.ops, dashboard.read | Approve as CH or SEC (one of each, neither the requester; at most four hours); **SU** `break_glass`; **4E** (phase 25) |\n"
"| POST | /access/break-glass/{id}/end | self | End early (the requester or SEC); `If-Match` (phase 25) |\n"
"| POST | /access/break-glass/{id}/review | security.ops | Record the review within 24 hours of the end, outcome code (phase 25, added) |\n")
# 3.20
rep("| GET | /admin/releases | admin.system | Imported release bundles [P1] |\n| POST | /admin/releases/verify | admin.system | Verify a bundle (signature, SBOM diff, notes, rollback plan) [P1] |\n| POST | /admin/support-sessions | admin.system | Open a time-boxed support session for vendor support [P1] |\n| POST | /admin/support-sessions/{id}/approve | security.ops | Approve it; **SU** [P1] |\n| DELETE | /admin/support-sessions/{id} | admin.system | End it [P1] |\n| GET | /help/articles | self | Bilingual help articles and \"what changed\" [P1] |\n",
"| GET | /admin/releases | admin.system, security.ops | Imported release bundles with verification result, approval and applied record (phase 25) |\n"
"| POST | /admin/releases/verify | admin.system | Import and verify a bundle (phase 00 `bundle verify` code path: signature, SBOM diff, notes, rollback plan, policy matrix diff); 202 with the job id (phase 25) |\n"
"| POST | /admin/releases/{id}/propose | admin.system | Propose a verified release for approval: **CR** target `platform_release` on `cr_access` (maker ADM, checker SEC); 202 (phase 25, added) |\n"
"| POST | /admin/releases/{id}/applied | admin.system | Record that the bank operator applied the approved release; no call-home, no remote push; **SU** `approval` (phase 25, added) |\n"
"| GET | /admin/support-sessions | admin.system, security.ops | Vendor support sessions with state and times (phase 25, added) |\n"
"| POST | /admin/support-sessions | admin.system | Request a time-boxed vendor support session (at most `support.session_max_hours`) (phase 25) |\n"
"| POST | /admin/support-sessions/{id}/approve | security.ops | Approve; the `VS` grant starts with the session; approver is not the requester; **SU** `access_change`; **4E** (phase 25) |\n"
"| DELETE | /admin/support-sessions/{id} | admin.system | End it now; the `VS` grant and the vendor's sessions and tokens are revoked (phase 25) |\n"
"| POST | /admin/support-sessions/{id}/bundles | admin.system | Register a support bundle built and confirmed with `akr-ctl support-bundle` (hashes, size, key version); **SU** `export` (phase 25, added) |\n"
"| GET | /help/articles | self | Bilingual help articles and \"what changed\" entries of the installed release, shipped in the signed bundle; `?kind=` (phase 25) |\n"
"| GET | /siem/events | siem.pull | SIEM pull API for a service principal: `siem`-flagged events after a cursor, allow-listed fields, no case id, no tier (phase 25, added) |\n\n"
"Phase 25 owns the rows marked \"phase 25\" here and in 3.2 and 3.3 (docs/plan/phase-25.md s.2; 21 rows, 9 added;\n"
"DR-232 to DR-245). `/admin/ops` gains tiles, not routes.\n")
open(p,'w').write(s)
print('ok')
