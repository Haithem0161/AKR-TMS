p='docs/DATA-MODEL.md'
s=open(p).read()
def rep(a,b):
    global s
    assert s.count(a)==1,(s.count(a),a[:80]); s=s.replace(a,b)
a=s.index("| fail_policies | id, rail, corridor, currency, amount_band, mode (shadow, advisory, enforcing)")
b=s.index("| holds | id, decision_id, decision_received_at, state, analyst_id null, resolved_at null, release_signal_at null | none | R7 |\n")
b+=len("| holds | id, decision_id, decision_received_at, state, analyst_id null, resolved_at null, release_signal_at null | none | R7 |\n")
new="""| rails | code (pk), label jsonb, payload_formats (mt103, pacs008, canonical), active, created_at, updated_at, version | none | R9 (retirement) |
| gateways | id, gateway_key (unique while active), user_id (service principal), cert_sha256 (unique), cert_subject, not_before, not_after, status (active, revoked), revoke_reason, registered_by, registered_at, revoked_at, revoked_by, version | none | R9 (retirement) |
| signing_keys | purpose (decision, extract), version, status (active, verify_only), public_key, fingerprint, wrapped_seed (akr-kek ciphertext, NULL once verify_only), created_by, created_at, retired_at; pk (purpose, version) | none | R9 (retirement) |
| decisions | id, received_at, kind (assessment, hold_release), gateway_id, rail, direction, payload_format, corridor, currency, amount_band, match_key (uetr or reference hash), request_sha256, decision (pass, hold, reject), mode, would_be (shadow only), fail_policy, policy_version, reason_codes text[], rule_id (a reject outside shadow), degraded text[], list_version_ids uuid[], matched_party_ids uuid[], hold_id (hold_release only), key_version, response_bytes, signature, expires_at, latency_ms; pk (id, received_at); append-only trigger | received_at (month; `assess.create_partition`) | R7 (partition) |
| idempotency | gateway_id, idem_key, request_sha256, decision_id, decision_received_at, state (in_flight, done), lease_until, created_at, completed_at; pk (gateway_id, idem_key) | none | R12 (age, created_at) |
| node_status | node_id (pk), started_at, last_seen_at, build_version, policy_version, mode_summary jsonb, version | none | R12 (age, last_seen_at) |
| blind_periods | id, started_at, ended_at, reason_code (no_heartbeat, operator_recorded), last_seen_at, recorded_by (system, operator), version; one open period | none | R10 (never) |
| policy_state | id (singleton 1), set_version, updated_at, version | none | R9 (retirement) |
| amount_bands | currency, band_code, lower_bound, upper_bound (NULL = unbounded), set_version, change_request_id; pk (currency, band_code); guard | none | R9 (retirement) |
| fail_policies | id, rail, corridor (`*` or ISO 2), currency (`*` or ISO 3), amount_band (`*` or code), on_failure (open, closed), set_version, change_request_id (`assess_policy`, approved or effective), activated_by, activated_at, superseded_at; one current row per key; guard | none | R9 (retirement) |
| rail_modes | id, rail, mode (shadow, advisory, enforcing), effective_from, superseded_at, change_request_id (`assess_mode`), activated_by, readiness jsonb; one open row per rail; guard | none | R9 (retirement) |
| prohibited_rules | rule_id, version, title jsonb, condition jsonb (keys corridor_in, currency_in, rail_in, amount_min only), basis, status (active, retired), change_request_id (`assess_rule`), activated_by, activated_at, retired_at; pk (rule_id, version); guard | none | R9 (retirement) |
| holds | id, hold_ref (opaque, unique), decision_id, decision_received_at, gateway_id, rail, mode, kind (list_match, review), reason_codes text[], state (held, advisory_open, shadow_review, released, rejected, expired, advisory_closed, shadow_closed), proposal (release, reject), proposal_reason_codes, proposed_by, proposed_at, decided_by (not the proposer), decided_at, alert_id (list_match only), subject_ref, party_names_enc (personal, enc), release_decision_id, release_received_at, expires_at, resolved_at, created_at, updated_at, version; guard (edges, never deleted, release pass once) | none | R7 (closure, resolved_at) |
| hold_candidates | hold_id, seq, entry_id, list_id, list_version_id, list_kind (freeze, pep, internal), outcome (hit, needs_more_data), score, evidence_bits, matching_major, subject_fp, entry_fp, explanation_enc (personal, enc), created_at; pk (hold_id, seq) | none | R7 (owner) |
| control_state | id (singleton 1), release_checkpoint_at, updated_at, version | none | R9 (retirement) |
| release_exceptions | id, payment_message_id, payment_received_at, match_key, rail, reason_code (no_decision, no_match_key, decision_not_pass, pass_expired), decision_id, found_at, register_id, closed_at, closed_by, close_reason (explained, bank_confirmed_manual, false_match), version; unique (payment_message_id, payment_received_at) | none | R7 (closure, closed_at) |
| control_tests | id, rail, test_ref (unique), sent_at, observed (hold, pass, reject, no_response), observed_decision_id, result (caught, missed; caught = hold), recorded_by, recorded_at, evidence_object_id | none | R11 (age, recorded_at) |
| mode_rollups | rail, day, mode, decisions, would_pass, would_hold, would_reject (from coalesce(would_be, decision)), reviews_closed, reviews_true_match, holds_released, holds_rejected, holds_expired, degraded_total, repeat_assessments, computed_at; pk (rail, day, mode) | none | R7 (age, day) |
| fallback_extracts | id, extract_version (unique), list_version_ids uuid[], evidence_object_id (sealed document), signing_key_version, row_count, built_at, valid_until (at most 12 hours) | none | R7 (age, built_at) |

Phase 29 (DR-292 to DR-316; migrations 0091 to 0094) replaces the earlier sketch: `decisions` drops
`model_version_id` (no model inline, DR-302), the replay ledger is `assess.idempotency` (not `admin.idempotency_keys`,
DR-295), `holds` is split into `holds` and `hold_candidates`, and fail policies, modes and rules are versioned change
targets. Phase 29 also widens `admin.registers.source` by `assess_control` (section 15) and adds flag `pack.assess`.
"""
s=s[:a]+new+s[b:]
rep("model_monitor (phase 27), assistant_monitor (phase 28)), source_key",
    "model_monitor (phase 27), assistant_monitor (phase 28), assess_control (phase 29)), source_key")
rep("\nPhase 28 grants: `akr_app` has SELECT and INSERT on admin.assistant_versions,",
    "\nPhase 29 grants: `akr_assess` has SELECT on the rails, gateways, signing keys, policy and mode tables, decisions, "
    "idempotency, holds, hold candidates and node status; INSERT on decisions, idempotency, holds, hold_candidates and "
    "node_status, with UPDATE only on `idempotency (state, lease_until, completed_at)`, `holds (release_decision_id, "
    "release_received_at, updated_at, version)` and `node_status (started_at, last_seen_at, build_version, policy_version, "
    "mode_summary, version)`, DELETE on idempotency (in-flight rows of a refused request), SELECT on `admin.users (id, kind, "
    "status, expires_at)` only, admin.settings, admin.feature_flags and the phase 14 `ref` list tables. `akr_app` has SELECT "
    "on all nineteen `assess` tables, INSERT on the configuration, hold, exception, test, roll-up and extract tables, "
    "UPDATE only on column lists (holds review columns, gateway revocation, key retirement, `superseded_at`, rule status, "
    "exception closure, the checkpoint, `blind_periods.ended_at`, the roll-up counts), DELETE on idempotency (the sweep) and "
    "EXECUTE on `assess.create_partition` and `assess.require_change`. `akr_retention` has SELECT and DELETE on every "
    "`assess` table. `akr_readonly` and `akr_audit_writer` get nothing on `assess`.\n"
    "Phase 28 grants: `akr_app` has SELECT and INSERT on admin.assistant_versions,")
rep("| 26 | `assess` | 29 |",
    "| 26 | `assess`: rails, gateways, signing keys, decisions (partitioned), `assess.create_partition`, idempotency, node status, blind periods (0091); policy state, amount bands, fail policies, rail modes, prohibited rules, holds, hold candidates, control state, release exceptions, control tests, mode roll-ups, fallback extracts (0092); guards and grants (0093); the widened `registers.source` CHECK, the `pack.assess` flag, settings, classification and retention (0094) | 29 |")
open(p,'w').write(s)
print('ok')
