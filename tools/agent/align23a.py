import re
p='docs/plan/research.md'
s=open(p).read()
rows=[
("DR-184","Phase 23 shape: migrations 0071-0072, 7 tables and 1 sequence in `admin`, `admin.evidence_objects.object_deleted_at` added, 9 routes (6 owned API.md 3.4 rows, 3 added), screen G7, jobs `retention_run`, `retention_lock_extend`, `evidence_reconcile`, Cedar `61_retention.cedar`, size M (heaviest logic of the milestone)","The audit viewer and verifier rows of API.md 3.4 are phase 24's"),
("DR-185","One engine and one plan: the dry run is the real run without its writes; each phase 00 `admin.retention_assignment` row maps to one disposal handler, checked by a unit test; no second assignment table","A new phase cannot forget its retention without the build failing"),
("DR-186","Per-class periods are governed settings with the Iraq floors as `floor.min`; the engine applies max(configured, floor) at run time; schedules and batch limits are not governed; jurisdiction profiles deferred","Research 30 s.12.3: a bank may lengthen, and shorten only to the floor"),
("DR-187","A partition is dropped only by the SECURITY DEFINER `admin.retention_drop_partition` (allow-list, range-hold check, party-hold copy check, raw batch check, DETACH and DROP); alert partitions also wait for open rows, case links and the disposition instant; audit partitions are never in the allow-list","One guarded path; `casework.case_alerts` is ON DELETE RESTRICT (lead review)"),
("DR-188","Copy before drop: a party or case hold on a dropped partition gets a sealed evidence copy (kind `package`, owner `legal_hold`, R6) with the row count compared under the partition lock; a range hold blocks the drop; raw landing is neither copied nor blocked by party or case holds; a parent with no party path is blocked while such a hold is active; copies shredded 30 days (governed) after release","No copy keeps a range; raw rows are in canon"),
("DR-189","Holds are placed by M under step-up class `decision`, never edited; three scope shapes matched live each run; release is workflow `legal_hold_release` with one approval by an M who is neither placer nor proposer; the drop's table lock closes the place-versus-drop race","Four-eyes on release; a released hold is final"),
("DR-190","R3 and R8 records are purged party by party on the relationship-end clock; a party named by an alert, case, case party or report is deferred; each purged customer and account leaves a keyed tombstone so the extract cannot reload it; only `akr_retention` holds DELETE","Phase 15 left relationship-end resolution to phase 23"),
("DR-191","R5 records outside partitions, R6 cases, reports and STR copies, R7 screening records, R9 and R11 history are reported as due, not deleted, in this phase; the report applies the later-of rule to reports","Purging cases needs a bank decision on the phase 17 and 18 append-only guards"),
("DR-192","Case holds are invisible to IA and SEC (omitted from lists and counts, E_NOT_FOUND on detail, no kind or id in the audit event); party holds are shown to them without the party id","A case hold reveals the case's existence (tipping-off)"),
("DR-193","Phase 23 owns the evidence lock extension at case closure: a job the seal transaction enqueues extends locks to `sealed_at` plus the class lock days, never shortening","Phase 18 Open doubt 14; M-60 is not in MODULES.md (reconciliation pass)"),
("DR-194","Certificates are signed with a new transit key `akr-sign-cert`; each batch writes a signed manifest to the COMPLIANCE bucket before its rows are deleted; identities are HMACs under the match key; certificate documents are plain JSON in WORM","Separate custody from the audit anchor key; replay must read them after a restore"),
("DR-195","After a restore, certificates newer than the restored database are verified and their manifests re-applied before the API reopens (`akr-ctl retention replay`, `verify-certificates`); the replay writes its own certificates","Restored rows that a certificate deleted must not come back"),
("DR-196","The API inserts dry-run requests only (CHECK on the trigger); `GET /retention/report` reads the latest completed run; a real run is the schedule or a host command","The plan's load stays in the nightly window"),
("DR-197","Only owner-following evidence is shredded here (KYC of a purged party, held copies after grace); byte deletion waits for the COMPLIANCE lock; orphans of phases 02, 17 and 21 are listed and deleted after their lock","An orphan's key was never committed"),
("DR-198","`akr_retention` is a NOLOGIN role with LOGIN user `akr_retention_login` in akr-worker's second pool: DELETE on the party tree and KYC family only, EXECUTE on the drop function, UPDATE column lists, SELECT on the planned schemas, no DDL, no raw access","Phase 10 named a retention role that did not exist"),
("DR-199","Batches of `retention.batch_rows` with a pause, run limit `retention.max_run_minutes`, 30 s statement timeout per batch (the drop uses `retention.drop_timeout_s`), 2 s lock timeout, one run at a time, schedule 03:45; a cut-short run is re-planned next night","Respect the database load"),
("DR-200","`61_retention.cedar` sits in phase 03's 60 audit range (phase 24 takes `60_audit.cedar`); one role per `@id`; the case forbid is a tipping-off deny answered E_NOT_FOUND; the release forbid is the four-eyes check","Phase 03 Cedar ranges"),
]
anchor=re.search(r'^\| DR-183 \|.*\n',s,re.M)
assert anchor
add=''.join(f"| {d} | 2026-10-09 | {a} | {b} |\n" for d,a,b in rows)
assert '| DR-184 |' not in s
s=s[:anchor.end()]+add+s[anchor.end():]
open(p,'w').write(s)
print('ok',len(rows))
