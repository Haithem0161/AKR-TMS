p='docs/plan/status.md'
s=open(p).read()
R=[
("phases 09 to 25 (batches B1 to B7; phases 19, 20, 21 and 23 are B6, phases 22, 24 and 25 are B7) reviewed 2026-10-09.",
 "phases 09 to 26 (batches B1 to B7; phases 19, 20, 21 and 23 are B6, phases 22, 24, 25 and 26 are B7) reviewed 2026-10-09."),
("| 26 | Islamic Pack (optional) | Planned | | | 3 | | | 1 |",
 "| 26 | Islamic Pack (optional) | Spec ready | | | 3 | `canon.product.islamic_contract_type` CHECK (+4 values); phase 20 `FreezeService::start` groups by account class; phase 16 pack seed and G9 read `pack: islamic`; phase 03 flag `pack.islamic` refused off while a version is live; phase 10 `CanonicalEntity` +2; phase 02 owner kind `islamic_deal` (gap pass 1) | 6 (4 owned, 2 added) | 3 (IslamicDealService, PossessionEvidenceService, ShariaAggregateQuery) |"),
("| Domain tables (specified) | 0 | 236 |","| Domain tables (specified) | 0 | 239 |"),
("| Routes | 0 | 497 |","| Routes | 0 | 503 |"),
("| Phases specified | 0 | 26 |","| Phases specified | 0 | 27 |"),
("| Screens | 0 | 58 (X1, X3,","| Screens | 0 | 59 (X1, X3,"),
("G8, G9, R4, A4, X5) |","G8, G9, R4, A4, X5, I1 (proposed by phase 26; not yet in DESIGN-SYSTEM 8.1)) |"),
("| 0 | 513 (some partial; `python3 tools/idcount.py 25`, phases 00-25:","| 0 | 537 (some partial; `python3 tools/idcount.py 26`, phases 00-26:"),
]
for a,b in R:
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
a=[l for l in s.split('\n') if l.startswith('| `anchor_timestamp` | 25 |')][0]+'\n'
jobs="| `islamic_evidence_sweep` | 26 | `schedule.islamic_evidence_sweep` (04:30): no-op unless `pack.islamic` and `islamic.evidence_tracking`; one counts-only phase 24 register warning (source `scan`, key `islamic_evidence_gap:<date>`) |\n"
assert '`islamic_evidence_sweep` | 26' not in s
s=s.replace(a,a+jobs)
open(p,'w').write(s)
print('ok')
