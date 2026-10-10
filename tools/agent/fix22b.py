p='docs/plan/phase-22.md'
s=open(p).read()
R=[
("Phase 10 (`canon.account.org_branch_id`, `canon.party`)","Phase 10 (`canon.staff.branch_id`, `canon.account.branch_id`, `canon.branch.org_branch_id`, `canon.party`)"),
("`RfiService::release` and `RfiService::record_answers`,","`RfiService::release` and `RfiService::record_answers` (its inner body `record_answers_in_tx`, section 4.5 item 2),"),
("M-45 (branch answers are stored as claims through `RfiService::record_answers`)","M-45 (branch answers are stored as claims through `RfiService::record_answers_in_tx`, never self-corroborated)"),
("Answer the item as claims through `RfiService::record_answers`; `If-Match`","Answer the item as claims through `RfiService::record_answers_in_tx`; `If-Match`"),
("recorded through `RfiService::record_answers` with source kind `branch_officer`.","recorded through `RfiService::record_answers_in_tx` with source kind `branch_officer` (lead correction below)."),
]
for a,b in R:
    assert s.count(a)==1,a
    s=s.replace(a,b)
open(p,'w').write(s)
print('ok',len(R))
