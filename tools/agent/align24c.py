p='docs/DATA-MODEL.md'
s=open(p).read()
R=[
("""| validation_packs | id, inventory_item_id, parts jsonb (seven parts), validator_id, verdict, issued_at, object_key | R11 |
""",
"""| validation_packs | id, inventory_item_id, inventory_version, version, period_from, period_to, parts jsonb (seven parts), verdict (pass, pass_with_findings, fail; computed), owner_id, validator_id (never the owner), evidence_id, sha256, issued_at; written once (phase 24) | R11 |
"""),
("""| registers | id, kind (finding, warning), body jsonb, owner_id, recurrence, status, version | R9 |
| fairness_metrics | id, period, metric, group_ref, value, computed_at | R9 |
""",
"""| registers | id, kind (finding, warning), source (manual, post_review, fairness, review_calendar, scan), source_key (unique, system rows), title jsonb, body jsonb, severity (findings), owner_id, due_on, repeat_of, repeat_flag, status, closure_code, closed_by, closed_at, created_by, created_at, updated_at, version; append-and-close (phase 24) | R9 (closure, closed_at) |
| fairness_metrics | id, period, metric, group_kind, group_ref, customers, events, value, reference_value, ratio, suppressed, status (ok, review, justified, changed, suppressed), register_id, computed_at (phase 24) | R9 |
| protected_attributes | code, label jsonb, attribute_class, source_column, allowed_uses text[], combined_trigger_ok, sole_trigger_ok (always false), prohibited_as_input, owner_id, basis jsonb, status, updated_by, updated_at, version; seeded (phase 24, GV-36) | R9 (retirement) |
| training_records | id, user_id, course_code, completed_on, expires_on, score_pct, evidence_id, evidence_ref, recorded_by (never the user), recorded_at, voided_by, voided_at, void_reason_code (phase 24, M-51) | R9 |
| assessor_samples | id, drawn_by, grant_id, frame (alert_register, report_register), frame_from, frame_to, frame_size, sample_size, seed, algorithm, frame_hash, items jsonb, drawn_at; immutable (phase 24, M-62) | R9 |
| verify_runs | id, requested_by, scope jsonb, status (queued, running, pass, findings, failed), result jsonb (counts and kinds only), error_code, created_at, started_at, finished_at (phase 24) | R10 (never) |
"""),
("""| assistant_drafts | id, case_id, use_case, prompt_template_version, model_version_id, draft (enc), sources jsonb, grounding_result jsonb, accepted_by null, accepted_at null | R6 |
""",
"""| assistant_drafts | id, case_id, use_case, prompt_template_version, model_version_id, draft (enc), sources jsonb, grounding_result jsonb, accepted_by null, accepted_at null | R6 |

As planned in phase 24 (DR-218 to DR-231): `registers` replaces `recurrence` with `repeat_of` and `repeat_flag` and
gains the source, title, severity, due and closure columns; `validation_packs` replaces `object_key` with
`evidence_id` (the vault holds the key and hash); `fairness_metrics` gains the group and suppression columns;
`protected_attributes`, `training_records`, `assessor_samples` and `verify_runs` are new. Decisions and exclusions are
read views over `casework.decisions` and `admin.exclusions`, never copied (DR-221). Models, model versions, model
scores and assistant drafts stay P2 (phases 27 and 28). `admin.packs` kinds gain `certification`. Two read
functions are added: `audit.tier_rank` and `audit.resource_tier` (SECURITY DEFINER, owner `akr_migrate`; mirrors
phase 17 tier rule 2; DR-219).
"""),
("""Phase 22 grants: `akr_app` has SELECT and INSERT on casework.observations""",
"""Phase 24 grants: `akr_app` has SELECT and INSERT on admin.registers, validation_packs, protected_attributes, fairness_metrics, training_records, assessor_samples and verify_runs, with UPDATE only on `registers (severity, owner_id, due_on, repeat_of, repeat_flag, status, closure_code, closed_by, closed_at, updated_at, version)`, `protected_attributes (label, allowed_uses, combined_trigger_ok, owner_id, basis, status, updated_by, updated_at, version)`, `fairness_metrics (status, register_id)`, `training_records (voided_by, voided_at, void_reason_code)` and `verify_runs (status, result, error_code, started_at, finished_at)`; `validation_packs` and `assessor_samples` have no UPDATE grant and guard triggers refuse change; no table is deleted from (guards); `akr_readonly` gets only EXECUTE on `audit.tier_rank` and `audit.resource_tier` (SECURITY DEFINER, owner `akr_migrate`, `search_path` pinned), no table or column grant in casework, alert, scn or canon; akr-api gains a read-only pool (KV `db/akr_readonly`, DR-218) (phase-24 section 1.4).
Phase 22 grants: `akr_app` has SELECT and INSERT on casework.observations"""),
("""| 22 | governance (validation packs, registers, fairness) | 24 |
""",
"""| 22 | `admin`: registers, protected attributes, validation packs (0076); fairness metrics, training records, assessor samples (0077); verify runs, the `admin.packs` kind `certification`, `audit.tier_rank` and `audit.resource_tier` (0078); guards, grants, the protected-attribute seed, settings, classification and retention (0079) | 24 |
"""),
]
for a,b in R:
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
open(p,'w').write(s)
print('ok',len(R))
