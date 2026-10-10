p='docs/plan/phase-23.md'; s=open(p).read()
a="Two existing tables are altered (`admin.evidence_objects` gains one column; the rest of phase 02's table is unchanged). Two SECURITY DEFINER functions are added (`admin.retention_drop_partition` and its guard)."
b="One existing table is altered (`admin.evidence_objects` gains one column; the rest of phase 02's table is unchanged). One SECURITY DEFINER function (`admin.retention_drop_partition`) and two trigger functions (`admin.retention_append_only`, `admin.legal_holds_guard`) are added."
assert s.count(a)==1; s=s.replace(a,b); open(p,'w').write(s)

p='docs/DATA-MODEL.md'; s=open(p).read()
reps=[
("key_version, shredded_at, created_by, created_at, updated_at | follows owner |",
 "key_version, shredded_at, object_deleted_at (bytes deleted after the lock, phase 23), created_by, created_at, updated_at | follows owner |"),
("| legal_holds | id, scope jsonb (party, case, partition range), reason, placed_by, placed_at, released_by null, released_at null | R10 |\n| deletion_certificates | id, run_id, class, scope jsonb, rows bigint, objects bigint, signature, issued_at | R10 |\n| retention_runs | id, started_at, finished_at, dry_run bool, summary jsonb | R10 |\n",
 "| legal_holds | id, hold_ref (HLD-YYYY-NNNNNN, `admin.hold_ref_seq`), kind (party, case, range), party_id null, case_id null fk, range_table null, range_from null, range_to null, scope jsonb, reason_code (regulator_request, court_order, investigation, litigation, other), reason_enc (sealed), state (active, release_requested, released), release_proposed_by null, release_instance_id null, release_reason null, placed_by, placed_at, released_by null, released_at null, version, created_at, updated_at; the placer and the proposer never release (phase 23) | R10 |\n"
 "| legal_hold_copies | id, hold_id fk, run_id fk, source_table, partition_name, partition_rows, row_count, evidence_id fk (sealed package, owner legal_hold), sha256, hold_released_at null, copied_at (phase 23) | R6 |\n"
 "| retention_runs | id, trigger (schedule, dry_run_request, replay), dry_run, requested_by null, as_of, state (queued, running, completed, failed), error_code null, summary jsonb, started_at, finished_at, created_at; the API inserts dry-run requests only (phase 23) | R10 |\n"
 "| retention_run_items | id, run_id fk, schema_name, table_name, partition_name null, retention_class, disposal (partition_drop, record_purge, evidence_shred, copy_only, report_only, kept, sweep), eligible, deleted, held, deferred, copied, objects_shredded, state, reason_code null, created_at; append-only (phase 23) | R10 |\n"
 "| deletion_certificates | id, run_id fk, retention_class, kind (run, replay), scope jsonb, row_count, object_count, item_digest, signature (transit `akr-sign-cert`), key_version, worm_object_key, worm_version_id, issued_at; append-only (phase 23) | R10 |\n"
 "| retention_tombstones | schema_name, table_name, source_key_hmac (HMAC under the match key), match_key_version, run_id fk, purged_at; pk (schema_name, table_name, source_key_hmac); read by ingest to refuse a purged identity (phase 23) | R10 |\n"
 "| evidence_orphans | id, bucket, object_key, version_id, size_bytes, retention_class null, lock_until null, found_by (reconcile, counter), found_at, deleted_at null (phase 23) | R12 |\n"),
]
for a,b in reps:
    assert s.count(a)==1,a[:60]; s=s.replace(a,b)
# retention clock note after the packs row of s.18
a="| packs | id, kind (board, case, report, assessor, correspondent, validator, cbi_gap, integration_profile, vendor_request), scope jsonb, scope_key, lang, version, evidence_id fk, sha256, manifest jsonb, created_by, created_at | R6 |\n"
assert s.count(a)==1
b=a+"\nRetention clocks (phase 23, DR-184): holds, runs, items, certificates and tombstones `never`; `legal_hold_copies` closure on `hold_released_at` (shredded after `retention.held_copy_grace_days`); `evidence_orphans` age on `deleted_at` (a row not yet deleted is never due). `retention_run_items` and `deletion_certificates` are append-only by trigger; `legal_holds` rows are never deleted and a released hold is final (trigger `admin.legal_holds_guard`). Partitions leave only through the SECURITY DEFINER `admin.retention_drop_partition` (DR-187).\n"
s=s.replace(a,b)
# grants paragraph after Phase 21 grants
i=s.index("Phase 21 grants:"); j=s.index("\n",i)+1
g=("Phase 23 grants: the new NOLOGIN role `akr_retention` (LOGIN user `akr_retention_login`, akr-worker's second pool; member of `akr_audit_writer`) "
"is the only role with DELETE in `canon` and the KYC family: DELETE on canon.party, party_name, party_identifier, party_contact, party_relationship, customer, account, account_party_role, account_hold, card, facility, trade_instrument, kyc_document, admin.xref, admin.attribute_values, admin.quarantine_items and kyc.ratings, rating_current, rating_feed, overrides, kyc_reviews, review_documents, review_queries, ownership_inputs, ownership_results, declared_profiles, pep_register, pep_relatives; "
"EXECUTE on `admin.retention_drop_partition` (owned by `akr_migrate`; the only path for a partition drop); SELECT and INSERT on retention_runs, retention_run_items, legal_hold_copies, deletion_certificates, retention_tombstones and evidence_orphans, with UPDATE only on `retention_runs (state, started_at, finished_at, summary, error_code)`, `evidence_orphans (deleted_at)` and `evidence_objects (wrapped_dek, shredded_at, object_deleted_at, lock_until, updated_at)`; SELECT on the admin tables it plans from and on all tables of canon, feat, alert, kyc, casework, rpt, scn and ref; no DDL and no raw access. "
"`akr_app` has SELECT and INSERT on `legal_holds` (UPDATE only on state, release_proposed_by, release_instance_id, release_reason, released_by, released_at, version, updated_at) and `retention_runs` (dry-run requests only, by CHECK), USAGE on `admin.hold_ref_seq`, SELECT on retention_run_items, deletion_certificates, legal_hold_copies (UPDATE only `hold_released_at`) and evidence_orphans; `akr_ingest` has SELECT on `retention_tombstones`; no DELETE for any other role (phase-23 section 1.2).\n")
s=s[:j]+g+s[j:]
a="| 21 | retention (legal holds, certificates, runs) | 23 |"
assert s.count(a)==1
s=s.replace(a,"| 21 | `admin`: the hold reference sequence, legal holds, retention runs, held-row copies, run items, deletion certificates, tombstones, evidence orphans (0071); the retention role, append-only and hold guards, `admin.evidence_objects.object_deleted_at`, `admin.retention_drop_partition`, grants, settings, classification and retention (0072) | 23 |")
open(p,'w').write(s); print('ok')
