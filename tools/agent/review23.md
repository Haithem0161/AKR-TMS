# Phase 23 review findings (lead, 2026-10-09)
1. HIGH alert partitions: (a) `casework.case_alerts` has FK (alert_id, alert_created_at) REFERENCES alert.alerts ON DELETE
   RESTRICT (phase 17), so DETACH of a partition holding a case-linked alert fails; the draft's "plain reference" claim is
   wrong. (b) The created_at partition clock can delete alerts before the R5 floor ("7 years from the report date"; an alert
   triaged or reported years after creation). Fix: the alert guard blocks with `referenced` while any alert of the partition
   is named by `casework.case_alerts` (the case is R6, not purged here, DR-191); otherwise the partition is due only when
   every row's disposition instant (`triaged_at`, or `updated_at` for duplicate and no_longer_matches) is on or before the
   cut-off; open rows block (`open_row`). Open doubt 3 rewritten.
2. MEDIUM raw landing: function step 5 blocks every raw partition while ANY party or case hold is active anywhere
   (`p_parent ~ '^raw\.'`), so R1 is never dropped while a hold lasts (holds last years): personal data kept far past its
   90 days. Fix: raw partitions are blocked only by range holds on that raw table and by unmapped batches; party and case
   holds are served by copies of the canonical rows (raw content is mapped into canon in the same transaction, phase 11).
3. MEDIUM party paths of the droppable parents are undefined (3.5 lists only the purge tree). canon.txn and
   balance_snapshot carry account_id only, card_txn card_id, feat.* entity/entity_id; txn_event, cash_event,
   cbs_user_activity, fx_deal, payment_message(_party) carry no party column. Without a path the copy of 3.4 cannot be
   built. Fix: a party-path table in 3.4 (account_id through account_party_role; card_id through canon.card.account_id;
   locker_access and alert.alerts party_id; feat entity party or account; txn_event follows its companion txn); a parent
   with no path is blocked by an active party or case hold (Open doubt 22).
4. MEDIUM drop count vs timeouts: the retention pool default statement_timeout is 30 s (4.3), but the function counts a
   whole partition under the PARENT's EXCLUSIVE lock (Open doubt 10: minutes for a large canon.txn month), so large drops
   always fail while every writer of the parent waits. Fix: lock the partition (SHARE) for the count, the parent only for
   DETACH (lock_timeout 2 s); the drop transaction sets its own statement_timeout from `retention.drop_timeout_s` (600).
5. LOW `retention_runs.error_code` CHECK allows only E_UNAVAILABLE and E_INTERNAL, but step 1 writes E_CLAIMED. Fix: add it.
6. LOW registry: `alert.detection_keys` does not exist (`scn.detection_keys`, phase 16, R5 age on first_seen_at);
   `admin.quarantine_items` (phase 11, R1 age on created_at) has no handler, so check 4a fails. Fix: rows added (quarantine
   items purged by age, DELETE grant to akr_retention; detection keys report_only).
7. LOW defaults: R6 and R7 default 10 years, but ARCHITECTURE s.17 gives 7 years for both (only R5 has 7/10; OFAC 10 for
   R7 is a profile, Open doubt 9). Fix: defaults 7.
Severity: 0 critical / 1 high (1) / 3 medium (2, 3, 4) / 3 low (5, 6, 7) = 7.
Gap-pass items (not draft defects): phase 18 seal transaction enqueues retention_lock_extend; phase 02 EvidenceStore gains
shred, delete_object, extend_lock and owner kind legal_hold; phase 10/11 CanonicalWriter tombstone read; phase 04 registries
LegalHoldWorkflowSubject, release_legal_hold, DSL E.4 legal_hold_release; phase 00 role akr_retention + provisioning;
roadmap row 22 migrations from 0073.
