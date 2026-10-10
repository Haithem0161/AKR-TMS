p='docs/plan/status.md'
s=open(p).read()
R=[
("phases 09 to 27 (batches B1 to B8; phases 19, 20, 21 and 23 are B6, phases 22, 24, 25 and 26 are B7, phase 27 is B8) reviewed 2026-10-09.",
 "phases 09 to 28 (batches B1 to B8; phases 19, 20, 21 and 23 are B6, phases 22, 24, 25 and 26 are B7, phases 27 and 28 are B8) reviewed 2026-10-09."),
("| 28 | Assistant | Planned | | | 1 | | | 2 |",
 "| 28 | Assistant | Spec ready | | | 4 | phase 19 weekly draw of `assistant_assisted`, source kind `assistant_draft` and the QA item view of an evaluation draft; phase 18 `RecordDecisionEffect` call for `ai_involvement` and the W3 `assistant_summary` affordance; phase 21 `ReportService::update_draft`; phase 23 purge of drafts before the case row and owner kind; phase 24 review-calendar row `assistant_kill_drill` and the `ValidationPackService` assistant branch; phase 25 SIEM and support-bundle exclusions and release import `registered` only; phase 03 flag service refuses `pack.assistant` off while a version is live; phase 00 zone `z2_model`, volumes and the `llama` and `llama-eval` images (gap pass 1) | 12 (3 owned, 9 added) | 9 (AssistantService, AssistantReads, AssistantKillSwitchService, AssistantVersionService, AssistantGenerator, AssistantMonitor, GroundingVerifier, PromptBuilder, AssistantEvaluationService) |"),
("| Domain tables (specified) | 0 | 245 |","| Domain tables (specified) | 0 | 249 |"),
("| Routes | 0 | 519 |","| Routes | 0 | 531 |"),
("| Phases specified | 0 | 28 |","| Phases specified | 0 | 29 |"),
("| 0 | 558 (some partial; `python3 tools/idcount.py 27`, phases 00-27:","| 0 | 580 (some partial; `python3 tools/idcount.py 28`, phases 00-28:"),
]
for a,b in R:
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
a=[l for l in s.split('\n') if l.startswith('| `model_label_gate` | 27 |')][0]+'\n'
jobs=("| `assistant_generate` | 28 | Enqueued by `POST /assistant/drafts`: re-checks switches and visibility, calls the live model, verifies, scans and seals the draft; no-op while `pack.assistant` is off |\n"
"| `assistant_evaluate` | 28 | Enqueued by `POST /assistant/evaluations`: golden drafts on `llama-eval` as the system with purpose `qa_review`; counts only; the human review stage finalises the run |\n"
"| `assistant_version_verify` | 28 | Enqueued by `POST /assistant/versions` for a model: manifest, signature and artifact SHA-256 checked on `llama-eval`; sets `registered` or `refused` |\n"
"| `assistant_monitor` | 28 | `schedule.assistant_monitor` (05:20): 30-day counts per task on live drafts, suppressed; a red breach opens one phase 24 finding per task and month (source `assistant_monitor`) |\n")
assert '`assistant_generate` | 28' not in s
s=s.replace(a,a+jobs)
open(p,'w').write(s)
print('ok')
