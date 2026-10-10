p='docs/API.md'
s=open(p).read()
def rep(a,b):
    global s
    assert s.count(a)==1,(s.count(a),a[:80])
    s=s.replace(a,b)
rep("| GET | /islamic/deals | queue.work | Deals for a party (contract type, stage, evidence status) |\n"
"| GET | /islamic/deals/{id} | queue.work | Deal view: parties by role, asset, parallel contracts, possession evidence |\n"
"| POST | /islamic/deals/{id}/evidence | queue.work | Attach possession or supplier evidence |\n"
"| GET | /islamic/sharia-board/aggregates | dashboard.read | Aggregate figures only for the Sharia board |\n",
"| GET | /islamic/deals | queue.work | Deals of one customer (`?customer_id=`), cursor; only deals the caller may read under the phase 18 party rule (phase 26) |\n"
"| GET | /islamic/deals/{id} | queue.work | Deal view: readable parties by role, asset, evidence status and rows; **SU** `record_open` on a suspicion-tier deal; a refused deal is the unknown-id 404 (phase 26) |\n"
"| POST | /islamic/deals/{id}/evidence | queue.work | Attach possession evidence (multipart, staged through `EvidenceStore`); **IDEM** (phase 26) |\n"
"| GET | /islamic/deals/{id}/evidence/{evidence_id}/content | queue.work | The bytes of one attachment through `EvidenceStore::open`; access event first; **SU** `record_open` on a suspicion-tier deal (phase 26, added) |\n"
"| POST | /islamic/deals/{id}/evidence/{evidence_id}/verify | queue.work | Verify or reject an attachment made by another person (A2, M); **SU** `decision`; `If-Match`; **IDEM** (phase 26, added) |\n"
"| GET | /islamic/sharia-board/aggregates | dashboard.read | Counts for a bank month by contract type, evidence status and gap rate, suppressed below `report.small_number_threshold`; no identifiers (phase 26) |\n\n"
"Phase 26 owns these rows (docs/plan/phase-26.md s.2; 6 rows, 2 added; DR-246 to DR-256). While flag `pack.islamic` is\n"
"off every `/islamic` route answers 404 `E_NOT_FOUND` with the unknown-id body.\n")
open(p,'w').write(s)
print('ok')
