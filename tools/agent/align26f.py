p='docs/MODULES.md'
s=open(p).read()
def rep(a,b):
    global s
    assert s.count(a)==1,(s.count(a),a[:80])
    s=s.replace(a,b)
rep("evidence tracker; freeze workflow involving Finance and the Sharia board for pooled funds; Sharia board aggregate",
"evidence tracker; pooled-fund freeze by account class (phase 26 DR-252: no Finance or Sharia board task, since neither\n  role exists); Sharia board aggregate")
rep("""fairness suite); a pooled-fund freeze (Finance and Sharia board tasks added).
""","""fairness suite); a pooled-fund freeze (the customer's pooled investment account gets an ordinary hold task, the pool and
the other holders are never in the instruction).
Phase file: docs/plan/phase-26.md (DR-246 to DR-256): deals and deal parties in `canon` (migration 0083, which also
widens the product contract-type list by four values), analyst-written possession evidence in `kyc` with the view
`kyc.deal_evidence_status` (0084); six routes (API.md 3.18); the deal read follows the phase 18 party rule
(`88_islamic.cedar`); missing evidence is a counts-only phase 24 register warning from job `islamic_evidence_sweep`, not
an alert (DR-248); no Islamic factor in any score (DR-249); library SCN-ISL-01..08 in shadow, live through G9
(DR-251); the Sharia board is not a user and reads counts through M, CH and IA (`dashboard.read`, DR-250). The Sharia
view's screen id is proposed `I1` and is not yet in the DESIGN-SYSTEM catalogue. Not carried by phase 26: I-09,
I-11..I-15, I-18, I-19, I-21..I-23, I-25, I-27..I-29 (phase 26 header; gap pass 1).
""")
open(p,'w').write(s)
print('ok')
