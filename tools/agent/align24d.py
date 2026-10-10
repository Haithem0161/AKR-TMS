p='docs/plan/roadmap.md'
s=open(p).read()
R=[
("| 24 | Governance, Registers, Command Centre & Packs | Validation packs, registers (decisions, exclusions, findings, warnings), fairness metrics, model inventory views; command centre (G1), registers (G4), access and insider controls (G5), audit viewer and chain verifier (G6), assessor workspace (G8), training register (G9), board and evidence pack builder (R4) | XL | P1 | 18-23 | Planned |",
 "| 24 | Governance, Registers, Command Centre & Packs | Audit viewer (searches audited, `purpose` required), chain status, verify runs and access history through a new akr-api read-only pool, suspicion rows hidden from IA and SEC by a SECURITY DEFINER tier resolver mirroring phase 17 rule 2; findings and warnings register (append-and-close) with decisions and exclusions as read views; validation packs (written-once versions, computed verdict, validator never the owner); protected-attribute register and monthly fairness metrics; derived review calendar; live command centre and read-only clocks board with small counts suppressed; assessor workspace (time-boxed `AS` grant, signed D2 map, seeded samples, metadata trace); pack kinds board, case, report, assessor, certification; training register; screens G1, G4, G5, G6, G8, G9, R4 | XL | P1 | 18-23 | Spec ready |"),
("| 24 | `validation_packs`, `registers`, `fairness_metrics` |",
 "| 24 | `admin.registers`, `admin.validation_packs`, `admin.protected_attributes`, `admin.fairness_metrics`, `admin.training_records`, `admin.assessor_samples`, `admin.verify_runs` (`admin.packs` kind `certification`; functions `audit.tier_rank`, `audit.resource_tier`; models, scores and assistant drafts stay P2) |"),
("| 24 | `RegisterService`, `CommandCentreService`, `PackBuilder`, `AssessorService` |",
 "| 24 | `AuditViewerService`, `AccessHistoryService`, `GovernanceRegisterService` (phase 19 owns `RegisterService`), `FairnessService`, `ValidationPackService`, `CommandCentreService`, `ClockBoardService`, `AssessorService`, `TrainingRegisterService`, `ProtectedAttributeCheck`; pack kinds on phase 02's `PackService`; `readonly_pool` in akr-api (DR-218) |"),
("| 24 | next numbers (from 0076) | 22 |",
 "| 24 | `0076_admin_registers_validation`, `0077_admin_fairness_training_assessor`, `0078_admin_verify_viewer`, `0079_admin_guards_grants` | 22 |"),
("| 26 | next numbers | 23 |","| 26 | next numbers (from 0080) | 23 |"),
]
for a,b in R:
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
row22=[l for l in s.split('\n') if l.startswith('| 0 (B7 review: phase 22)')][0]
new=("| 0 (B7 review: phase 24) | 2026-10-09 | 8 lead corrections to the Haiku draft | 0 | 1 | 3 | 4 | HIGH: the audit viewer's tier "
"resolver used only the party tier and the case tier, missing phase 17 rule 2 (the case family tier, every case an alert belongs "
"to, `suspicion` for a restricted scenario), so IA and SEC would see audit rows of suspicion alerts; it ran as `akr_readonly` "
"with new column grants on casework tables that phase 17 keeps from that role (now SECURITY DEFINER `audit.resource_tier`, owner "
"`akr_migrate`, no table grant; `access_event` rows hidden on the higher of the stored and current tier). MEDIUM: the decisions "
"register and clocks board filtered on suspicion tier only, admitting restricted cases to CH and IA (now phase 17 "
"`services::visibility::build` with the access list and QA grants); the assessor could trace any alert of any tier (now a "
"suspicion alert only as an item of a sample drawn under the grant in force); the pack routes did not accept `assessor.read`, so "
"AS could never reach its pack. LOW: floors bounded the wrong side (`fairness.review_ratio` max 1.25, `assessor.grant_max_days` "
"max 30, finding due days max 365); register retention `closure` on `closed_at`; control report suppression; Cedar one role per "
"policy, no `containsAny`, `context has pack_kind`, no `outcome` column |")
assert 'B7 review: phase 24' not in s
s=s.replace(row22+'\n',row22+'\n'+new+'\n')
open(p,'w').write(s)
print('ok')
