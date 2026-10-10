p='docs/DATA-MODEL.md'
t=open(p).read()
def rep(a,b):
    global t
    assert t.count(a)==1,(a[:60],t.count(a))
    t=t.replace(a,b)
old_rows=[l for l in t.split('\n') if l.startswith('| ops_tasks |') or l.startswith('| ops_verifications |') or l.startswith('| freezes |')]
assert len(old_rows)==3
block='\n'.join(old_rows)
new='''| ops_tasks | id, task_ref (T-YYYY-NNNN, sequence `casework.ops_ref_seq`), case_id null (team navigation only), source_kind (freeze, refrain, exit), source_id, action (freeze, hold, restrict, exit, release), neutral_text_code (ops.restrict.account, ops.hold.account, ops.release.account), target_accounts uuid[], authority_ref, effective_at, due_at (mirrors the owner's clock; no timer_id), state (open, claimed, completed), claimed_by, claimed_at, cbs_reference null, completed_by null, completed_at null, verification_level (v0-v3) null, second_person_only, verified_at null, result (codes and counts), created_by, created_at, updated_at, version (phase 20, DR-158, DR-166) | R7 |
| ops_verifications | id, ops_task_id, kind (second_person, canonical_check, attestation), level, checked_at, actor_id, canon_as_of, accounts_checked, holds_matched, post_order_debits jsonb, evidence_id (attestation), workflow_instance_id (attestation); append-only (phase 20, DR-160) | R7 |
| freezes | id, freeze_ref, screening_alert_id (unique), case_id null, subject_kind, subject_id, list_id, list_code, entry_id, entry_key, listing_decision_at, listing_decided_by, freeze_due_at, freeze_limit_at, committee_report_due_at, ops_task_id, verification_level, frozen_at null, post_order_debit_count, committee_report_letter_id null, committee_report_at, second_report_due_at, second_report_letter_id null, second_report_at, referral_letter_id, referred_at, state (listed, frozen, released), released_at null, release_decision_id null, release_reason_code, delisting_list_version_id, created_at, updated_at, version (phase 20, DR-161, DR-162) | R7 |
| refrain_instructions | id, refrain_ref, office_reference, received_at, party_ids uuid[], target_accounts uuid[], case_id null, ops_task_id, refrain_due_at (7 working days), state (open, confirmed, referred, closed), confirmed_at, referral_letter_id, referred_at, closed_at, closed_by, closure_reason_code, registered_by, created_at, updated_at, version (phase 20, DR-163) | R7 |
| ops_incidents | id, kind (post_order_debit, hold_absent, limit_missed, report_missed, task_failed), subject_kind (ops_task, freeze, refrain), subject_id, raised_at, raised_by, state (open, closed), payload, closed_at, closed_by, closure_reason_code, version; one open incident per kind and subject (phase 20, DR-164) | R7 |

Retention clocks (phase 20, DR-166): `freezes` from `released_at`, `refrain_instructions` and `ops_incidents` from `closed_at`, `ops_tasks` and `ops_verifications` follow their owner, so a freeze in force is never due for deletion. `ops_verifications` is append-only by trigger.'''
rep(block,new)
rep("casework.alarms | id, kind (touch_breach, work_item_breach, chain_broken, conflict_declared, flat_line, qa_critical)",
    "casework.alarms | id, kind (touch_breach, work_item_breach, chain_broken, conflict_declared, flat_line, qa_critical; phase 20 adds post_order_debit, freeze_overdue, refrain_overdue)")
rep("subject_kind, subject_id, case_id, raised_at, supervisor_id",
    "subject_kind (phase 20 adds freeze, refrain, ops_task), subject_id, case_id, raised_at, supervisor_id")
g19=[l for l in t.split('\n') if l.startswith('Phase 19 grants:')][0]
rep(g19+'\n', g19+'''

Phase 20 grants: `akr_app` has SELECT and INSERT on `casework.ops_tasks`, `ops_verifications`, `freezes`, `refrain_instructions` and `ops_incidents` and USAGE on `casework.ops_ref_seq`, with UPDATE only on the state, claim, completion and verification columns of `ops_tasks` (state, claimed_by, claimed_at, cbs_reference, completed_by, completed_at, verification_level, second_person_only, verified_at, result, updated_at, version), the state, task, level, letter, release and delisting columns of `freezes`, `refrain_instructions (ops_task_id, state, confirmed_at, referral_letter_id, referred_at, closed_at, closed_by, closure_reason_code, updated_at, version)` and `ops_incidents (state, payload, closed_at, closed_by, closure_reason_code, version)`; `ops_verifications` is never updated; no DELETE; no grant to `akr_readonly` (phase-20 section 1.2).
''')
rep('| 18 | ops tasks and freezes | 20 |',
    '| 18 | `casework`: the ops reference sequence, ops tasks, ops verifications, freezes, refrain instructions, ops incidents, the `casework.alarms` kind and subject CHECKs widened (0066); guards, grants, settings, classification and retention (0067) | 20 |')
open(p,'w').write(t)
print('ok')
