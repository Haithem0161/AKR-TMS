p='docs/DATA-MODEL.md'
t=open(p).read()
def rep(a,b):
    global t
    assert t.count(a)==1,a[:80]
    t=t.replace(a,b)
# s.12 alarms
rep("phase 20 adds post_order_debit, freeze_overdue, refrain_overdue), stage",
    "phase 20 adds post_order_debit, freeze_overdue, refrain_overdue; phase 21 adds report_filing_overdue, report_rework_overdue), stage")
rep("subject_kind (phase 20 adds freeze, refrain, ops_task), subject_id",
    "subject_kind (phase 20 adds freeze, refrain, ops_task; phase 21 adds report, report_task), subject_id")
# s.14 rows
start=t.index("| reports | id, type (str, sar, tfr, utr, ctr, aif), case_id null,")
endmark="| periodic_reports | id, kind (cbi_monthly, liaison, board, mlro_annual, fatca, crs), period, status, object_key, signed_by | R6 |\n"
end=t.index(endmark)+len(endmark)
new='''| reports | id, report_ref (RP-TYPE-YYYY-NNNN, sequence `rpt.report_ref_seq`), type (str, sar, tfr, utr, ctr, aif), case_id null, proposal_id null (unique), workflow_instance_id, ctr_batch_id null (CTR only), subject_party_id, profile_id (the pinned `goaml_profiles` version), status (draft, validated, signed, packaged, filed, acknowledged, rejected, returned), current_version, frozen_version, suspicion_formed_on (RP-48), quality_score, quality_grade (estimated), office_grade null, office_grade_at, submitted_at, submitted_by, signed_at, signed_by (not the submitter), created_by, created_at, updated_at, version (phase 21, DR-169) | R6 |
| report_versions | id, report_id, version, content_enc (sealed, AAD the row id), content_sha256, validation_summary, validated_run_id, validated_at, frozen_at (frozen by trigger), created_by, created_at; the XML is rebuilt from the frozen content, not stored (phase 21, DR-172) | R6 |
| report_findings | id, report_id, report_version, validation_run_id, source (validation, office), layer (L0-L5, office), rule_code, severity, element_path, field_key, line_no, col_no, params (value-free), created_at; append-only (phase 21, DR-171) | R6 |
| report_packages | id, report_id, report_version (unique), manifest, manifest_sha256, package_evidence_id, pdf_evidence_id, built_at; append-only (phase 21, DR-172) | R6 |
| report_links | id, report_id, related_report_id, relation (previous, amending, superseding, escalated_from_utr), created_by, created_at (phase 21, RP-19) | R6 |
| filings | id, report_id, report_version (unique), package_id, goaml_reference (unique), filed_by, filed_at, portal_capture_evidence_id (RP-79), activity_age_months (M-41), ack_at null, ack_by, ack_evidence_id, created_at, updated_at, version; the close gate reads `ack_at` (phase 21, DR-173) | R6 |
| fiu_feedback | id, report_id null, filing_id null, kind (grade, rejection, session, request), grade null, finding_ids uuid[], capture_evidence_id, changes_made (max 2,000, policy scan), rule_change_id (RP-84), received_at, recorded_by, created_at; append-only (phase 21, DR-179) | R6 |
| report_tasks | id, kind (returned_for_completion, fx_rate_missing, ctr_control_check), report_id, feedback_id, ctr_batch_id, business_date, currency, state (open, claimed, done), due_at, claimed_by, claimed_at, closed_by, closed_at, closure_reason_code, created_at, updated_at, version (phase 21, RP-43) | R6 |
| ctr_batches | id, business_date, run_no (one live run per date), status (queued, waiting_source, waiting_fx, computed, confirmed, packaged, superseded, failed), source_txn_count, reportable_count, excluded_count, total_usd, control_totals, gl_tieout (`not_available`: no general ledger in the canonical copy), created_by null (the job), confirmed_by (not the creator), confirmed_at, created_at, updated_at, version (phase 21, DR-174) | R6 |
| ctr_items | id, batch_id, grain (customer_day, transaction), customer_id null, business_date, txn_refs, deposits_usd, withdrawals_usd, total_usd, total_lcy, rate_date (= business_date), rate_source (cbi, manual), reportable, exclusion_code, report_id null, file_no null, created_at (phase 21, RP-51) | R6 |
| letters | id, letter_no (AKR-LTR-YYYY-NNNNNN, sequence `rpt.letter_no_seq`), kind (freeze_report, freeze_second_report, referral, office_reply, hold_reply), source_kind (freeze, refrain, authority_request, report), source_id, case_id null, recipient_code (committee, office, authority), copy_to_office, facts (ids, codes, counts, dates only), state (requested, rendered, signed, delivered), pdf_evidence_id, requested_by null, requested_at, signed_by, signed_at, channel (letter, email), delivered_by, delivered_at, delivery_reference, created_at, updated_at, version (phase 21, DR-176) | R7 |
| periodic_reports | id, kind (cbi_monthly, liaison, board, mlro_annual, fatca, crs, analytics), period (YYYY-MM or YYYY), period_start, period_end, status (draft, final, superseded), template_version, facts, suppressed_cells (RP-76), evidence_id, generated_by, signed_by (not the generator), signed_at, created_at, updated_at, version (phase 21, DR-177) | R6 |

Retention clocks (phase 21): `reports` closure clock on `created_at` (the sweep takes the later of case closure and filing; a CTR file has no case), versions, findings, packages, links, filings and items follow their owner, `fiu_feedback` from `received_at`, `report_tasks` from `closed_at`, `ctr_batches` from `business_date`, `letters` (R7) from `delivered_at`, `periodic_reports` from `period_end`. `report_findings`, `report_packages` and `fiu_feedback` are append-only by trigger. The single `ctr_batches` table of this section was split into `ctr_batches` (one row per date and run) and `ctr_items` (phase-21 section 1).
'''
t=t[:start]+new+t[end:]
# s.21 grants
anchor="no grant to `akr_readonly` (phase-20 section 1.2).\n"
rep(anchor, anchor+"\nPhase 21 grants: `akr_app` has SELECT and INSERT on the twelve `rpt` tables of section 14 created by phase 21 and USAGE on `rpt.report_ref_seq` and `rpt.letter_no_seq`, with UPDATE only on the status, count and confirmation columns of `ctr_batches` (confirmed_by, confirmed_at, status, source_txn_count, reportable_count, excluded_count, total_usd, control_totals, gl_tieout, updated_at, version), the status, version, quality, grade, submit and sign columns of `reports`, `report_versions (content_enc, content_sha256, validation_summary, validated_run_id, validated_at, frozen_at)` (a frozen version is refused by trigger), `filings (ack_at, ack_by, ack_evidence_id, updated_at, version)`, the state, claim, close and due columns of `report_tasks`, `ctr_items (report_id, file_no)`, the state, PDF, sign and delivery columns of `letters` and `periodic_reports (status, facts, suppressed_cells, evidence_id, signed_by, signed_at, updated_at, version)`; `report_findings`, `report_packages`, `report_links` and `fiu_feedback` are never updated; no DELETE; no grant to `akr_readonly`, `akr_ingest`, `akr_audit_writer` or `akr_monitor` (phase-21 section 1.3).\n")
rep("| 19 | `rpt` full (reports, versions, packages, filings, feedback, CTR, letters, periodic) | 21 |",
    "| 19 | `rpt`: the report reference sequence, CTR batches, reports, versions, findings, packages, links, filings, FIU feedback, report tasks, CTR items (0068); the letter number sequence, letters, periodic reports (0069); guards, the `casework.alarms` kind and subject CHECKs widened, grants, settings, classification and retention (0070) | 21 |")
open(p,'w').write(t);print('ok')
