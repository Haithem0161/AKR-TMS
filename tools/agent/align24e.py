p='docs/plan/status.md'
s=open(p).read()
R=[
("phases 09 to 23 (batches B1 to B7; phases 19, 20, 21 and 23 are B6, phase 22 opens B7) reviewed 2026-10-09.",
 "phases 09 to 24 (batches B1 to B7; phases 19, 20, 21 and 23 are B6, phases 22 and 24 are B7) reviewed 2026-10-09."),
("| 24 | Governance, Registers, Command Centre & Packs | Planned | | | 3 | | | 4 |",
 "| 24 | Governance, Registers, Command Centre & Packs | Spec ready | | | 7 (models, scores and assistant drafts stay P2) | `admin.packs` kind `certification`; functions `audit.tier_rank`, `audit.resource_tier`; phase 06 pack routes accept `assessor.read` and download step-up `export`; phase 00 AppRole akr-api KV `db/akr_readonly`; phase 02 evidence owner kinds `validation_pack`, `training_record`; phase 03 MASKED_KINDS; phase 04 ChangeTarget `protected_attribute`; phase 16 `check_before_decision` calls `ProtectedAttributeCheck` (gap pass 1) | 28 (+ 3 phase 06 pack routes extended) | 10 (AuditViewerService, AccessHistoryService, GovernanceRegisterService, FairnessService, ValidationPackService, CommandCentreService, ClockBoardService, AssessorService, TrainingRegisterService, ProtectedAttributeCheck) |"),
("| Domain tables (specified) | 0 | 218 |","| Domain tables (specified) | 0 | 225 |"),
("| Routes | 0 | 448 |","| Routes | 0 | 476 |"),
("| Phases specified | 0 | 24 |","| Phases specified | 0 | 25 |"),
("| Screens | 0 | 49 (X1, X3,","| Screens | 0 | 56 (X1, X3,"),
("B1, B2, B3, B4, B5, B6, R5) |","B1, B2, B3, B4, B5, B6, R5, G1, G4, G5, G6, G8, G9, R4) |"),
("| 0 | 493 (some partial; `python3 tools/idcount.py 23`, phases 00-23:","| 0 | 506 (some partial; `python3 tools/idcount.py 24`, phases 00-24:"),
]
for a,b in R:
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
a=[l for l in s.split('\n') if l.startswith('| `notification_sweep` | 22 |')][0]+'\n'
jobs="""| `audit_verify_run` | 24 | Enqueued by `POST /audit/verify-runs`: phase 02 `Verifier::run` over the range through the read-only pool; stores counts and finding kinds only |
| `register_post_review_sync` | 24 | `schedule.register_post_review_sync` (every 15 minutes): phase 04 post-change review concerns become findings (unique `source_key`) |
| `review_calendar_sweep` | 24 | `schedule.review_calendar_sweep` (06:00): one warning for each review-calendar row that turns red |
| `fairness_monthly` | 24 | `schedule.fairness_monthly` (day 2, 04:30): group rates of the month before with suppression; a ratio above `fairness.review_ratio` opens a review |
"""
assert '`audit_verify_run` | 24' not in s
s=s.replace(a,a+jobs)
open(p,'w').write(s)
print('ok')
