p='docs/plan/roadmap.md'
s=open(p).read()
R=[
("| 27 | Models & Model Governance | Models and versions, model scores, ONNX serving through `ort`, TreeSHAP, shadow then ranking-only promotion through `cr_material_validated`, drift and kill switch; screen D4 | L | P2 | 17, 24 | Planned |",
 "| 27 | Models & Model Governance | Model families and versions as inventory items (only `approved_for_use` scores), signed import in two steps (Ed25519 manifest in the API, artifact checks in the worker; `akr-ctl model artifact add` for bank-trained artifacts), ONNX through `ort` with `tract` fallback in the worker, scores in `alert.model_scores` ranking inside the priority class only, TreeSHAP reason codes (three shown), label gates 50/200/500 over every tier, frozen shadow criteria, validation through the phase 24 pack, promotion by `cr_material_validated`, PSI and override monitoring with red points as phase 24 findings, kill switch immediate by MRO and released by change request, pins and one live family; screen D4 and the W2 score panel | L | P2 | 17, 24 (also 00, 01, 02, 03, 04, 13, 16, 18, 19, 21, 25) | Spec ready |"),
("| 27 | `models`, `model_versions`, `model_scores` |",
 "| 27 | `admin.models`, `admin.model_versions`, `admin.model_evaluations`, `admin.model_kill_state`, `admin.model_monitor_points`, `alert.model_scores` (phase 24 `registers.source` and `fairness_metrics.metric` CHECKs widened) |"),
("| 27 | `ModelRegistry`, `ModelScorer`, `ExplanationService` |",
 "| 27 | `ModelRegistry`, `ModelImportVerifier`, `ModelScorer`, `ExplanationService`, `ModelMonitor`, `ModelLabelGateService`, `ModelPromotion` (change target `model_version`), `ModelSwitch` (`model_switch`), `ModelKillSwitchService`, `ModelReads`; crate `crates/ml` |"),
("| 27-28 | next numbers (from 0085) | 25 |",
 "| 27 | `0085_admin_models_registry`, `0086_alert_model_scores`, `0087_admin_models_guards_grants` | 25 |\n| 28 | next numbers (from 0088) | 25b |"),
]
for a,b in R:
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
row=[l for l in s.split('\n') if l.startswith('| 0 (B7 review: phase 26)')][0]
new=("| 0 (B8 review: phase 27) | 2026-10-09 | 9 lead corrections to the Haiku draft | 0 | 0 | 4 | 5 | MEDIUM: the version guard "
"froze `shadow_criteria` for ever and let a version leave `live` without the promotion flag, so the documented rollback by "
"re-shadowing could not run and a retired champion could stay the live pointer (criteria now change only on `retired -> shadow` "
"or `challenger`; leaving `live` needs the flag); the read-only `model-store` had no writer for a bank-trained artifact (added "
"`akr-ctl model artifact add`); route `POST /models/{id}/shadow` had no steps and `top_band_cut` was NULL during shadow while "
"`top_band` is NOT NULL, so every shadow insert failed (steps written; provisional cut from the reference bins); counts excluded "
"suspicion tier, but phase 18 raises the subject on every report decision, so almost no positive or override was counted (jobs "
"count every tier; reads show suppressed window aggregates). LOW: the family's kill-switch row did not exist for `If-Match`; the "
"family guard refused `akr_retention` deletes; the score route demanded a purpose on every alert; the feature lookup missed the "
"month-end, currency and status keys; phase 24 closure codes and a second family's promotion (`other_family_live`) were unstated |")
assert 'B8 review: phase 27' not in s
s=s.replace(row+'\n',row+'\n'+new+'\n')
open(p,'w').write(s)
print('ok')
