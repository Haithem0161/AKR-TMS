import re
p='docs/plan/research.md'
s=open(p).read()
rows=[
("DR-246","Phase 26 deal tables are in `canon` (`canon.islamic_deals` bi-temporal, `canon.deal_parties`), written by `CanonicalWriter` through `akr_ingest` (migration 0083); possession evidence is analyst-written in `kyc.possession_evidence` (0084); the view `kyc.deal_evidence_status` yields `verified` only from an analyst's verification, `unknown` otherwise","`canon` is read-only for `akr_app`, so a CBS row or a mapping can never set `verified`"),
("DR-247","`canon.product.islamic_contract_type` widens by `tawarruq`, `diminishing_musharaka`, `wakala`, `wadiah` (twelve values, same list on the deal); sukuk, takaful and sarf are not deal contract types","Each added contract is named by research file 24 and I-01; the others are an instrument, insurance and an exchange"),
("DR-248","Missing or late possession evidence is a counts-only phase 24 register warning (source `scan`, column `body`) written by the daily `islamic_evidence_sweep`, gated by governed `islamic.evidence_tracking` (off until the bank declares the process live); no alert; a duplicate document number is a count for M only; verification is a second person's decision under step-up `decision` and CHECK `decided_by <> attached_by`; the verifier opens the file through the added content route","A process gap is not a finding about conduct; a register or audit row would show IA the document sits on another deal (lead review: closed register source list; no generic evidence route)"),
("DR-249","No Islamic factor enters any score, feature or fairness segment; an Islamic scenario uses the contract type only for applicability and needs a further condition (`packs verify-data` check `islamic_sole_trigger`)","I-30 and S-37; why I-28 is not carried"),
("DR-250","The Sharia board is not a role or a user; `GET /islamic/sharia-board/aggregates` (scope `dashboard.read`; M, CH, IA) returns counts by contract type, evidence status and gap rate, suppressed below `report.small_number_threshold`; no drill-down, no identifiers","MODULES s.0 has no Sharia role; counts only is the tipping-off answer"),
("DR-251","The Islamic library is eight documents SCN-ISL-01..08 (pack `islamic`, `requires_flag` `pack.islamic`), installed in shadow and taken live through G9 with `pack.akr_live` extended to `islamic` and a change request; the flag cannot turn off while an islamic version is live","Same route as any AKR library scenario; a release never replaces a version the bank approved"),
("DR-252","The pooled-fund freeze groups target accounts by class in phase 20 `FreezeService::start`: unrestricted `freeze`, restricted `restrict`, pooled investment `hold` (an ordinary hold task, no `second_person_only`); no Finance or Sharia task; the class map is setting `islamic.account_class_map`","The pool and other holders are never in the instruction; neither role exists (lead review: phase 20 CHECK allows `second_person_only` only at `v0`, so the insert rolled the freeze back)"),
("DR-253","A deal is read under its customer party's tier and team: Cedar actions `islamic.deal.read`, `islamic.evidence.read`, `islamic.evidence.attach`, `islamic.evidence.verify` on phase 18 `Party`; a refused deal is the unknown-id 404; other parties appear only when readable, with no count of the rest","One predicate for parties and deals"),
("DR-254","Islamic data-quality rules (`islamic.asset_missing`, `islamic.supplier_missing`, `islamic.settlement_ccy_missing`) are phase 12 kind `validation_rule` with the code in `detail.code`","The `dq_issues` kind CHECK stays unchanged"),
("DR-255","Cedar file `88_islamic.cedar` (detection range 80) repeats the three phase 18 party forbids for the Islamic actions; `sharia.aggregate.read` on `System`","Phase 18 forbids name their own action list and cannot be extended from another file"),
("DR-256","Phase 26 adds one job (`islamic_evidence_sweep`), five settings (`islamic.evidence_tracking`, `islamic.evidence_due_days`, `islamic.dq_failure_rate`, `islamic.account_class_map`, `schedule.islamic_evidence_sweep`) and no outbox topic; statutory amounts and periods are scenario parameters with min equal to max","No bank can loosen a statutory number by a setting"),
]
anchor=re.search(r'^\| DR-245 \|.*\n',s,re.M)
assert anchor
assert '| DR-246 |' not in s
add=''.join(f"| {d} | 2026-10-09 | {a} | {b} |\n" for d,a,b in rows)
s=s[:anchor.end()]+add+s[anchor.end():]
open(p,'w').write(s)
print('ok',len(rows))
