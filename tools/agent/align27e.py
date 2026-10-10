p='docs/plan/status.md'
s=open(p).read()
R=[
("phases 09 to 26 (batches B1 to B7; phases 19, 20, 21 and 23 are B6, phases 22, 24, 25 and 26 are B7) reviewed 2026-10-09.",
 "phases 09 to 27 (batches B1 to B8; phases 19, 20, 21 and 23 are B6, phases 22, 24, 25 and 26 are B7, phase 27 is B8) reviewed 2026-10-09."),
("| 27 | Models & Model Governance | Planned | | | 3 | | | 3 |",
 "| 27 | Models & Model Governance | Spec ready | | | 6 | phase 24 `registers.source` (+`model_monitor`), `fairness_metrics.metric` (+`score_top_band_rate`), closure codes (+5) and `fairness_monthly`; phase 17 queue ORDER BY gains the model rank; phase 04 change targets `model_version` and `model_switch`; phase 25 release import sets `registered` only; phase 00 volume `model-store` and key purpose `keys/model-artifact`; phase 02 owner kind `model_version` (gap pass 1) | 16 (6 owned, 10 added) | 10 (ModelRegistry, ModelImportVerifier, ModelScorer, ExplanationService, ModelMonitor, ModelLabelGateService, ModelPromotion, ModelSwitch, ModelKillSwitchService, ModelReads) |"),
("| Domain tables (specified) | 0 | 239 |","| Domain tables (specified) | 0 | 245 |"),
("| Routes | 0 | 503 |","| Routes | 0 | 519 |"),
("| Phases specified | 0 | 27 |","| Phases specified | 0 | 28 |"),
("| Screens | 0 | 59 (X1, X3,","| Screens | 0 | 60 (X1, X3,"),
("I1 (proposed by phase 26; not yet in DESIGN-SYSTEM 8.1)) |","I1 (proposed by phase 26; not yet in DESIGN-SYSTEM 8.1), D4) |"),
("| 0 | 537 (some partial; `python3 tools/idcount.py 26`, phases 00-26:","| 0 | 558 (some partial; `python3 tools/idcount.py 27`, phases 00-27:"),
]
for a,b in R:
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
a=[l for l in s.split('\n') if l.startswith('| `islamic_evidence_sweep` | 26 |')][0]+'\n'
jobs=("| `model_import_verify` | 27 | Enqueued by `POST /models/{id}/versions`: artifact sha, operator allow-list, catalogue features, protected attributes, golden rows; sets `registered` or `refused` |\n"
"| `model_score_sweep` | 27 | `schedule.model_score_sweep` (every minute): scores new alerts with the live, challenger and shadow versions; skips everything while a kill switch is engaged |\n"
"| `model_monitor` | 27 | `schedule.model_monitor` (05:00): score and feature PSI, override rate, scored count for the previous bank day; a red point opens one phase 24 finding (source `model_monitor`) |\n"
"| `model_label_gate` | 27 | `schedule.model_label_gate` (Monday 04:00): QA-confirmed label counts over every tier against the 50/200/500 gates; one `label_gate` evaluation |\n")
assert '`model_score_sweep` | 27' not in s
s=s.replace(a,a+jobs)
open(p,'w').write(s)
print('ok')
