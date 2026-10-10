p='docs/DATA-MODEL.md'
s=open(p).read()
def rep(a,b):
    global s
    assert s.count(a)==1,(s.count(a),a[:80])
    s=s.replace(a,b)
rep("## 20. Islamic pack (`canon`, optional)\n\n| Table | Columns | Class |\n|---|---|---|\n"
"| islamic_deals | id, customer_id, contract_type, status, asset_ref, deferred_price, evidence_status, [lineage] | R3 |\n"
"| deal_parties | deal_id, party_id, role (seller, buyer, agent, supplier, developer, partner) | R3 |\n"
"| possession_evidence | id, deal_id, kind, evidence_object_id, verified_by, verified_at | R3 |\n",
"## 20. Islamic pack (`canon` and `kyc`, optional)\n\n| Table | Columns | Class |\n|---|---|---|\n"
"| canon.islamic_deals | id, customer_id, account_id null, product_id, contract_type (murabaha, musharaka, mudaraba, ijara, qard_hasan, istisna, salam, other, tawarruq, diminishing_musharaka, wakala, wadiah), status (active, settled, closed, defaulted, cancelled, unknown), asset_ref null, asset_class, currency, settlement_currency null, contract_amount null, deferred_price null, opened_on, break_even_on null, closed_on null, sharia_approval_ref null, [bitemp], [lineage]; pk (id, recorded_from) | R3 |\n"
"| canon.deal_parties | id, deal_id, party_id, role (customer, seller, buyer, agent, supplier, developer, contractor, partner, broker), [lineage], superseded_at | R3 |\n"
"| kyc.possession_evidence | id, deal_id, kind (invoice, delivery_note, title, registration, lease, valuation, shipping, other), evidence_object_id, document_ref_hk null, status (attached, verified, rejected), reject_code null, attached_by, attached_at, decided_by null, decided_at null, updated_at, version; CHECK decided_by <> attached_by | R3 |\n"
"| kyc.deal_evidence_status (view) | deal_id, evidence_status (verified, attached, unknown) over the current deals | - |\n")
rep("is not also loaded as a `facility` row. `evidence_status` is unknown for CBS-sourced deals until a possession source is\n"
"identified; it is never set to verified from the CBS.",
"is not also loaded as a `facility` row. Possession evidence is analyst-written in `kyc` (DR-246): the evidence status\n"
"is the view `kyc.deal_evidence_status`, `verified` only from an analyst's verification and `unknown` otherwise; it is\n"
"never set to verified from the CBS. Phase 26 (migrations 0083-0084) widens `canon.product.islamic_contract_type` by\n"
"tawarruq, diminishing_musharaka, wakala and wadiah (DR-247).")
rep("Phase 25 grants: ",
"Phase 26 grants: `akr_ingest` has SELECT and INSERT on canon.islamic_deals and canon.deal_parties, with UPDATE only on `islamic_deals (recorded_to, valid_to)` and `deal_parties (superseded_at)`; `akr_app` has SELECT on both, SELECT and INSERT on kyc.possession_evidence with UPDATE only on `(status, reject_code, decided_by, decided_at, updated_at, version)`, and SELECT on the view kyc.deal_evidence_status; `akr_retention` has SELECT and DELETE on the three tables (phase 23's `SELECT ON ALL TABLES` predates them); no grant to `akr_readonly` (phase-26 section 1.5).\n"
"Phase 25 grants: ")
rep("| 24 | Islamic pack | 26 (optional) |",
"| 24 | `canon`: islamic deals, deal parties, the widened `product.islamic_contract_type` CHECK, classification and retention (0083); `kyc`: possession evidence, the view `deal_evidence_status`, grants, classification and retention (0084) | 26 (optional) |")
open(p,'w').write(s)
print('ok')
