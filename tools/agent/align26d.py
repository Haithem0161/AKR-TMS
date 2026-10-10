p='docs/plan/roadmap.md'
s=open(p).read()
R=[
("| 26 | Islamic Pack (optional) | Islamic deals, deal parties, possession evidence; deal-centric scenarios and mapping extensions; switched on per installation for an Islamic CBS (the ICS BANKS installation studied was one, DR-36) | S | P1 | 10, 16 | Planned |",
 "| 26 | Islamic Pack (optional) | Islamic deals and deal parties in `canon` (contract-type list widened by four), analyst-written possession evidence in `kyc` with a derived status view (CBS never sets verified), deal read under the phase 18 party rule, evidence attach, open and second-person verify, missing evidence as a counts-only register warning (no alert), library SCN-ISL-01..08 in shadow and live through G9, pooled-fund freeze by account class, Sharia board counts only (M, CH, IA), no Islamic factor in any score; everything off unless `pack.islamic`; switched on per installation for an Islamic CBS (DR-36) | M | P1 | 10, 16 (also 18, 20, 21, 24) | Spec ready |"),
("| 26 | `islamic_deals`, `deal_parties`, `possession_evidence` |",
 "| 26 | `canon.islamic_deals`, `canon.deal_parties`, `kyc.possession_evidence` (view `kyc.deal_evidence_status`; `canon.product` CHECK widened) |"),
("| 26 | `IslamicDealService` |",
 "| 26 | `IslamicDealService`, `PossessionEvidenceService`, `ShariaAggregateQuery`; the pooled branch of phase 20 `FreezeService::start` |"),
("| 26 | next numbers (from 0083) | 24 |\n| 27-28 | next numbers | 25 |",
 "| 26 | `0083_canon_islamic_deals`, `0084_kyc_possession_evidence` | 24 |\n| 27-28 | next numbers (from 0085) | 25 |"),
("Phases 04 + 22 + 24 ──▶ Phase 25                 Phases 10 + 16 ──▶ Phase 26 (optional)",
 "Phases 04 + 22 + 24 ──▶ Phase 25                 Phases 10 + 16 (+ 18, 20, 24) ──▶ Phase 26 (optional)"),
]
for a,b in R:
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
row25=[l for l in s.split('\n') if l.startswith('| 0 (B7 review: phase 25)')][0]
new=("| 0 (B7 review: phase 26) | 2026-10-09 | 7 lead corrections to the Haiku draft | 0 | 0 | 3 | 4 | MEDIUM: the pooled-fund "
"hold task was created with `second_person_only`, which phase 20's CHECK allows only at verification level `v0`, so the insert "
"failed and rolled back the whole freeze start (now an ordinary hold task); the verifier had no route to open the evidence file "
"(no generic `GET /evidence/{id}` exists; added the per-owner content route of phase 18's pattern); the sweep's register warning "
"used a source outside phase 24's closed list and a column that does not exist (now source `scan`, column `body`). LOW: the G1 "
"evidence tile was shown to A2, who holds no `dashboard.read`; access events used actions outside phase 02's closed list; "
"`akr_retention` lacked SELECT on the new tables (phase 23's `ON ALL TABLES` predates them) and a SQL block was unclosed; "
"suppression is below the governed threshold, and the sweep adds business days forward |")
assert 'B7 review: phase 26' not in s
s=s.replace(row25+'\n',row25+'\n'+new+'\n')
open(p,'w').write(s)
print('ok')
