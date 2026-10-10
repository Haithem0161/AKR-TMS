# Phase 28 review findings (lead, 2026-10-09)
1. MEDIUM a candidate model can never be evaluated, so nothing can go live: s.4.4 runs one `llama` container that
   serves only the live model, s.3.9 promotion needs a final passing evaluation of the candidate triple, and the verify
   step (b) loads the file "in the test profile" on an unnamed server. Before the first go-live there is no live model,
   so the evaluation job has no server that holds the candidate. Fix: a second container `llama-eval` (same image, zone
   `z2_model`, compose profile `assistant-eval`, never serves live drafts) started by `akr-ctl assistant eval-server
   --version <id>` with the registered artifact; `assistant_version_verify` (b) and `assistant_evaluate` call it through
   a second `AssistantModelPort` instance and first check that its reported SHA-256 equals the candidate's
   `artifact_sha256` (`eval_server_mismatch`: the run ends `passed = false`); production `llama` starts only when a live
   model exists.
2. MEDIUM a template-only change cannot be promoted: s.3.9 step 1 requires the model version `registered`, and the
   activation retires "the previous live model" and each previous live template; with the model already live (its sha is
   unique, so it cannot be re-imported) a new template can never go live, and a named template that is already live
   would be retired and then set live, an edge the guard refuses. Fix: the model may be `registered` or `live`; the
   activation retires only the versions it replaces and leaves a named version that is already live as it is; a
   request that changes nothing is 409 `nothing_to_promote`.
3. MEDIUM the human factual review of golden drafts has no item to review: the phase 19 population `assistant_golden`
   is added but `casework.qa_samples.source_kind` (alert, case, decision, hibernation_hit, pair_memory_suppression) is
   not widened, evaluation drafts are hidden from every route, and no column ties a draft to its run; the reviewer
   cannot read the text, finalisation can never find the run's samples, and the gate never passes. Fix: widen
   `source_kind` by `assistant_draft` (case_id required, as for case and decision); phase 19 `GET
   /qa/samples/{id}/item` renders the verified output of an evaluation draft with its cited records to its reviewer
   under the time-boxed grant (requirement on phase 19); `casework.assistant_drafts.eval_run_id` (NOT NULL exactly for
   mode `evaluation`) and `admin.assistant_evaluations.run_id` tie drafts, samples and both stages of a run; the
   `assistant_assisted` draw skips decisions already sampled (phase 19 unique `(source_kind, source_id)`).
4. LOW evaluation drafts have no defined requester or retrieval principal: `requested_by` is NOT NULL and retrieval is
   "what the requester may open", but the initiator is an MRO who may open no case. Fix: `requested_by` = the run's
   initiator (record only); in mode `evaluation` retrieval runs as the system principal with purpose `qa_review`
   (s.3.6); the sealed-case refusal of s.3.1 step 9 is for live drafts only (golden cases are sealed by definition).
5. LOW the accept route gains scope `report.prepare` (DR-280), which admits nobody new (A2 and M already hold
   `queue.work`) and changes an API row for nothing. Fix: keep `queue.work` alone; Cedar and phase 21 `report.edit`
   decide.
6. LOW model version numbers are not unique: `UNIQUE (kind, task, version_no)` never fires for models (task NULL,
   NULLs distinct). Fix: `UNIQUE NULLS NOT DISTINCT` (PostgreSQL 15+, the stack runs 17).
7. LOW the promotion names `akr-ctl net render` to restart `llama`, but that command renders the nftables zone rules
   (phase 00 l.710). Fix: `akr-ctl assistant serve` renders the `llama` command from the live version and restarts the
   container; until then the readiness mismatch refuses drafts with `model_unavailable` (fail closed).
8. LOW the case summary limit ignores the prefix line: s.3.5 step 8 checks the text against `casework.note_max_chars`
   but step 10 adds a first line, so a text near the limit fails inside `NoteService::post`. Fix: limit = note_max_chars
   minus the prefix length.
9. LOW the register finding owner "follows phase 27", which uses the family owner; the assistant has no family. Fix:
   the owner of the task's live template inventory item (the live model item's owner when the task has none).
Severity: 0 critical / 0 high / 3 medium (1-3) / 6 low (4-9) = 9.
Gap-pass items (not draft defects): tech-stack.md language-model runtime row (llama.cpp server, GGUF, CPU default,
pinned tag at the phase 00 smoke test); phase 03 flag service refuses `pack.assistant` off while a version is live (as
`pack.islamic`); phase 18 `RecordDecisionEffect` call and the `assistant_summary` affordance; phase 19 weekly draw of
`assistant_assisted`, the `assistant_draft` source kind and the item view; phase 21 names `ReportService::update_draft`;
phase 23 purge of drafts before the case row and owner kind; phase 24 review-calendar row `assistant_kill_drill`,
`ValidationPackService` assistant branch; phase 25 SIEM and support bundle exclusions, release import `registered` only;
phase 00 zone `z2_model`, volumes, `llama` and `llama-eval` images; report-narrative counts on the monitoring panel show
MRO the volume of report drafting (phase 24 shows report rates only to `dashboard.read`): decide whether MRO sees the
`report_narrative` row; kill switch engage by one MRO versus SP-95 two-person (Open doubts 8).
