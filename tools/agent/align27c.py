p='docs/DATA-MODEL.md'
s=open(p).read()
def rep(a,b):
    global s
    assert s.count(a)==1,(s.count(a),a[:80])
    s=s.replace(a,b)
rep("| models / model_versions | id, name, purpose, tier; version, artifact_object_key, signature, card jsonb, training_manifest jsonb (hashes only), status (registered, shadow, live, retired), label_gate jsonb | R11 |\n"
"| model_scores | alert_id, alert_created_at, model_version_id, score, shap jsonb, scored_at | R5 |\n",
"| models | id, name jsonb, purpose jsonb, starter, recalibration_due (starter only), inventory_item_id (unique; kind `model`), live_version_id null, pinned_version_id null, next_kill_drill_due, created_by, created_at, updated_at, version; at most one family with a live version (phase 27) | R11 (retirement) |\n"
"| model_versions | id, model_id, version_no, inventory_item_id (unique), status (importing, refused, registered, shadow, challenger, live, retired), refusal_code, artifact_key, artifact_sha256 (unique), manifest jsonb (signed, hashes and declared values only), manifest_sha256, signature (Ed25519), signing_key_version, card_evidence_id, feature_keys jsonb, monotone jsonb, runtime (ort, tract), opset jsonb, reference_bins jsonb (histogram counts), top_band_cut, shadow_started_at, shadow_criteria jsonb (frozen; changes only on `retired -> shadow` or `challenger`), promotion_requested_at, live_from, retired_at, created_by, created_at, updated_at, version; one shadow, one challenger and one live per family (phase 27) | R11 (retirement) |\n"
"| model_evaluations | id, model_id, model_version_id (null only for `kill_drill`), kind (benchmark, out_of_time, calibration, label_gate, shadow_summary, kill_drill, sensitivity), result jsonb (counts and metrics, no ids), passed, data_as_of, created_by, created_at; written once (phase 27) | R11 (age, created_at) |\n"
"| model_kill_state | scope pk (`global` or `model:<family id>`; the family row created with the family), engaged, reason_code (model_error, data_break, red_breach, regulator_request, other), engaged_by, engaged_at, updated_at, version (phase 27) | R10 (never) |\n"
"| model_monitor_points | id, model_version_id, day, metric (score_psi, feature_psi, override_rate, scored_count), feature_key, value null, population, band (green, amber, red, suppressed, not_run), register_id null, computed_at; unique (version, day, metric, feature); written once (phase 27) | R9 (age, computed_at) |\n"
"| `alert.model_scores` | alert_id, alert_created_at, model_version_id, role (live, challenger, shadow), score, top_band, reason_codes text[] (at most 3), shap jsonb (at most 5; live rows only), scored_at; pk (alert_id, alert_created_at, model_version_id); FK (alert_id, alert_created_at) into `alert.alerts` RESTRICT; never updated; in schema `alert` (DR-258) | R5 (owner) |\n")
rep("| registers | id, kind (finding, warning), source (manual, post_review, fairness, review_calendar, scan),",
"| registers | id, kind (finding, warning), source (manual, post_review, fairness, review_calendar, scan, model_monitor (phase 27)),")
rep("| fairness_metrics | id, period, metric,","| fairness_metrics | id, period, metric (alert_rate, case_rate, str_rate, hold_rate, score_top_band_rate (phase 27)),")
rep("Models, model versions, model\nscores and assistant drafts stay P2 (phases 27 and 28).",
"Phase 27 (DR-257 to DR-273) adds\n`models`, `model_versions`, `model_evaluations`, `model_kill_state`, `model_monitor_points` and `alert.model_scores`,\nand widens the `registers.source` and `fairness_metrics.metric` CHECKs; assistant drafts stay P2 (phase 28).")
rep("Phase 26 grants:",
"Phase 27 grants: `akr_app` has SELECT and INSERT on the five `admin` model tables and alert.model_scores, with UPDATE only on column lists of admin.models (name, purpose, recalibration_due, live_version_id, pinned_version_id, next_kill_drill_due, updated_at, version), admin.model_versions (lifecycle, runtime, bins, cut and shadow columns) and admin.model_kill_state (engaged, reason_code, engaged_by, engaged_at, updated_at, version); no UPDATE on evaluations, monitor points or scores; `akr_retention` has SELECT and DELETE on all six; no grant to `akr_readonly` (phase-27 section 1.3).\n\nPhase 26 grants:")
rep("| 25 | models, model_scores, assistant_drafts | 27-28 |",
"| 25 | `admin`: models, model versions, evaluations, kill state, monitor points, the widened `registers.source` and `fairness_metrics.metric` CHECKs (0085); `alert`: model scores (0086); guards, grants, settings, classification and retention (0087) | 27 |\n"
"| 25b | assistant_drafts (from 0088) | 28 |")
open(p,'w').write(s)
print('ok')
