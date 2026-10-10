p='docs/DATA-MODEL.md'
s=open(p).read()
def rep(a,b):
    global s
    assert s.count(a)==1,(s.count(a),a[:80])
    s=s.replace(a,b)
rep("| role text (19 codes, MODULES s.0) | scope jsonb (all_branches, or branches and pools) | source text (bootstrap, change_request) | change_request_id fk null |",
    "| role text (19 codes, MODULES s.0) | scope jsonb (all_branches, or branches and pools) | source text (bootstrap, change_request, support_session) | change_request_id fk null | support_session_id fk null (phase 25; `VS` only from a session, with `valid_to`, DR-237) |")
new='''### access_reviews / access_review_items (phase 25, DR-234)
| id uuid pk | status text (building, open, signed, failed) | snapshot_at | evidence_id fk null | evidence_sha256 bytea null | item_count int | due_on date | started_by fk | started_at | signed_by fk null (never the starter) | signed_at null | closed_at null | created_at | updated_at | version |
| id uuid pk | review_id fk | grant_id fk (role_grants) | user_id fk | role text | decision text (pending, confirmed, revoke_proposed, flagged, superseded) | decision_note_code text null | decided_by fk null (never the grant holder) | decided_at null | change_request_id fk null (the `role_grant` revoke) | version | unique (review_id, grant_id) |

The snapshot of `AccessMatrixBuilder::build(at)` and the grants in force is one phase 02 evidence object; items hold
ids only. R9 (closure on `closed_at`; items owner).

### break_glass_grants / break_glass_log (phase 25, DR-232, DR-233, DR-242)
| id uuid pk | case_id uuid (no FK) | requester_id fk (A2) | reason_code text (mlro_unavailable, case_unowned, regulator_deadline, stuck_work, other) | status text (requested, active, ended, expired, lapsed) | requested_at | request_expires_at | first_approver_id fk null | first_approver_role text null (CH, SEC) | first_approved_at null | second_approver_id fk null | second_approver_role text null | second_approved_at null | granted_at null | expires_at null (at most granted_at + 4 h, CHECK) | ended_at null | ended_by fk null | reviewed_at null | reviewed_by fk null | review_outcome_code text null (appropriate, escalate, no_action) | version |
| id uuid pk | grant_id fk | seq int | kind text (requested, approved, activated, used, ended, expired, lapsed, reviewed, review_overdue) | actor_id fk null | detail jsonb | occurred_at | unique (grant_id, seq) |

Unique open request per (case_id, requester_id); approvers are two different people of different roles, neither the
requester. Read-only under purpose `break_glass`. R10 (grants closure on `reviewed_at`; log owner).

### releases / help_articles (phase 25, DR-236, DR-240)
| id uuid pk | release_version text | channel text (stable, security) | bundle_ref text | manifest_sha256 bytea | status text (imported, verifying, verified, refused, proposed, approved, applied, superseded) | refusal_code text null | key_fingerprint text null | cosign_result jsonb null | sbom_diff jsonb null | notes jsonb null (ar, en) | rollback_plan_sha256 bytea null | policy_bundle_hash bytea null | policy_matrix_diff jsonb null | evidence_id fk null | change_request_id fk null (`platform_release`, `cr_access`) | imported_by fk | imported_at | verified_at null | applied_by fk null | applied_at null | created_at | version | unique (release_version, manifest_sha256) |
| id uuid pk | release_id fk | code text | kind text (article, glossary, what_changed) | title jsonb | body jsonb | sha256 bytea | sort_order int | created_at | unique (release_id, code) |

Releases R10 never; help articles R9 retirement (class `public`).

### support_sessions / support_bundles (phase 25, DR-237, DR-238)
| id uuid pk | vendor_user_id fk | requested_by fk | approved_by fk null (never the requester) | reason_code text (defect_reproduction, upgrade_assist, incident_bridge) | ticket_ref text | requested_hours int (1-8) | status text (requested, active, ended, expired, refused_timeout) | requested_at | approved_at null | starts_at null | ends_at null | ended_at null | ended_by fk null | version |
| id uuid pk | session_id fk | created_by fk | created_at | manifest_sha256 bytea | archive_sha256 bytea | signature bytea | signing_key_version int | pseudonym_key_ref text (OpenBao path, never the key) | size_bytes bigint (at most 200 MiB) | confirmed_by fk | confirmed_at | archive_deleted_at null | status text (confirmed, archive_deleted) | version |

The support-bundle disclosure record (SP-52); the phase 19 M-37 register is not used. Sessions R9 closure on
`ended_at`; bundles R9 age on `created_at`.

### siem_cursors / siem_dead_letters / anchor_timestamps (phase 25, DR-235, DR-241)
| consumer text pk | last_occurred_at | last_audit_id uuid | forwarded_total bigint | updated_at | version |
| id uuid pk | audit_id uuid | occurred_at | action text | reason_code text (format_failed, unknown_kind, collector_refused, oversize) | attempts int | created_at | resolved_at null | resolved_by fk null |
| anchor_id uuid pk fk (audit.audit_anchors) | tsa_name text | token bytea | token_sha256 bytea | requested_at | verified_at null |

The forwarder and the pull API read `audit.audit_log` only through SECURITY DEFINER `audit.siem_page` (owner
`akr_migrate`; no case id, no tier, resource fields dropped for every non-normal row). Cursors R12 retirement; dead
letters R12 age on `created_at`; anchor time stamps R10 owner (RFC 3161, off by default).

## 2. Audit (`audit`)'''
rep("\n## 2. Audit (`audit`)", "\n"+new)
grants=("Phase 25 grants: `akr_app` has SELECT and INSERT on admin.access_reviews, access_review_items, break_glass_grants, break_glass_log, releases, help_articles, support_sessions, support_bundles, siem_cursors, siem_dead_letters and anchor_timestamps, with UPDATE only on `access_reviews (status, evidence_id, evidence_sha256, item_count, signed_by, signed_at, closed_at, updated_at, version)`, `access_review_items (decision, decision_note_code, decided_by, decided_at, change_request_id, version)`, the approval, grant, end and review columns of `break_glass_grants`, the verification, approval and applied columns of `releases`, `support_sessions (status, approved_by, approved_at, starts_at, ends_at, ended_at, ended_by, version)`, `support_bundles (status, archive_deleted_at, version)`, `siem_cursors (last_occurred_at, last_audit_id, forwarded_total, updated_at, version)`, `siem_dead_letters (resolved_at, resolved_by)` and `anchor_timestamps (verified_at)`; `break_glass_log` and `help_articles` are append-only (guards); `akr_app` gets EXECUTE on `audit.siem_page` (SECURITY DEFINER, owner `akr_migrate`, `search_path = pg_catalog`); `admin.role_grants` gains `support_session_id` and the source `support_session`; no grant to `akr_readonly`, `akr_ingest` or `akr_audit_writer` (phase-25 section 1.3).\n")
rep("Phase 24 grants: ", grants+"Phase 24 grants: ")
rep("| 23 | Islamic pack | 26 (optional) |",
    "| 23 | `admin`: access reviews, review items, break-glass grants, break-glass log (0080); releases, help articles, support sessions, support bundles, `role_grants.support_session_id` and the `support_session` source (0081); SIEM cursors, dead letters, anchor time stamps, `audit.siem_page`, guards, grants, the two notice codes, settings, classification and retention (0082) | 25 |\n| 24 | Islamic pack | 26 (optional) |")
rep("| 24 | models, model_scores, assistant_drafts | 27-28 |","| 25 | models, model_scores, assistant_drafts | 27-28 |")
rep("| 25 | `assess` | 29 |","| 26 | `assess` | 29 |")
open(p,'w').write(s)
print('ok')
