p='docs/plan/roadmap.md'
s=open(p).read()
R=[
("| 28 | Assistant | Assistant drafts for case summaries and report narratives with sentence-level sources, evaluation gate, per-bank switch | M | P2 | 18, 21, 27 | Planned |",
 "| 28 | Assistant | Optional pack (flag `pack.assistant`, off by default) with two drafting tasks, case summary and report narrative; llama.cpp server with one signed GGUF model on premises in zone `z2_model` without egress, called by the worker only, plus `llama-eval` for verification and evaluation; generation as a job, placeholders filled by code, `GroundingVerifier` and the phase 09 tipping-off scan; nothing saved without one person accepting one draft (`ai_involvement` on the decision); golden evaluation on QA-confirmed cases with a human factual review; model and template go-live by `cr_material_validated`; per-task switches, kill switch by one MRO and release by change request; suppressed monitoring counts; QA sample of assisted decisions never below 10 per cent; no agent or tools; additions to W3, R1 and D4 | M | P2 | 18, 21, 27 (also 00, 01, 02, 03, 04, 09, 17, 19, 24, 26) | Spec ready |"),
("| 28 | `assistant_drafts` |",
 "| 28 | `admin.assistant_versions`, `admin.assistant_evaluations`, `admin.assistant_kill_state`, `casework.assistant_drafts` (phase 24 `registers.source` and phase 19 `qa_samples` population and source kind CHECKs widened) |"),
("| 28 | `AssistantService`, `EvaluationService` |",
 "| 28 | `AssistantService`, `AssistantReads`, `AssistantKillSwitchService`, `AssistantVersionService`, `AssistantGenerator`, `AssistantMonitor`, `GroundingVerifier`, `PromptBuilder`, `AssistantEvaluationService`, port `AssistantModelPort` (`LlamaServerClient`); crate `crates/llm` |"),
("| 28 | next numbers (from 0088) | 25b |\n| 29 | next numbers | 26 |",
 "| 28 | `0088_admin_assistant_registry`, `0089_casework_assistant_drafts`, `0090_assistant_guards_grants` | 25b |\n| 29 | next numbers (from 0091) | 26 |"),
]
for a,b in R:
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
row=[l for l in s.split('\n') if l.startswith('| 0 (B8 review: phase 27)')][0]
new=("| 0 (B8 review: phase 28) | 2026-10-09 | 9 lead corrections to the Haiku draft | 0 | 0 | 3 | 6 | MEDIUM: the only model "
"server served the live model, so a candidate could never be verified or evaluated and nothing could go live (added `llama-eval` "
"in profile `assistant-eval`, the job checks the artifact hash it reports); promotion required a `registered` model and retired "
"the whole live set, so a template-only change was impossible (a named live version stays live, only replaced versions retire, "
"`nothing_to_promote`); the golden human review had no item to review (phase 19 `source_kind` widened by `assistant_draft`, the QA "
"item view shows the evaluation draft, `eval_run_id` and `run_id` tie drafts, samples and stages). LOW: evaluation drafts had no "
"retrieval principal (system with purpose `qa_review`); the accept route's added `report.prepare` admitted nobody new (kept "
"`queue.work`); model version numbers were not unique (NULLS NOT DISTINCT); `akr-ctl net render` named for the model server "
"restart (`akr-ctl assistant serve`); the summary limit ignored the note's prefix line; the breach finding owner had no family "
"(the task's live template item owner) |")
assert 'B8 review: phase 28' not in s
s=s.replace(row+'\n',row+'\n'+new+'\n')
open(p,'w').write(s)
print('ok')
