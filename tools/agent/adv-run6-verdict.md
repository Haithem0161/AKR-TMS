# Adversarial pass run 6: lead verdict (2026-10-10; decisions final, applied by apply-adv-run6.py)

Haiku draft tools/agent/adv-run6.md (replayed idempotency keys, audit-chain gaps, outbox replay and handler
idempotency): 17 candidates; FLAW 5 (CRITICAL 2, HIGH 3), DOUBT 3, GUARDED 9. About 193k tokens.
Lead result: 6 confirmed in 5 fixes (HIGH 2, MEDIUM 4), 1 LOW, 1 rejected as designed, 9 guarded accepted.

## Confirmed
- ADV-6-01 and ADV-6-02 HIGH (phase 00; supersedes the last clause of DR-21). The claim (section 3, IdempotencyService
  step 4) is its own committed statement, the handler commits its own transaction, and step 5 stores the response
  afterwards. (a) A crash between the business commit and step 5, or a failed step 5 ("any other outcome -> delete the
  claim"), leaves a retry free to run the handler again. (b) A live first attempt slower than
  `idempotency.in_flight_lease_s` is "taken over" while it still runs. Either way a create without a natural key (a note,
  an evidence row, an RFI) commits twice. Routes with `If-Match` refuse the second run (the version moved), so the
  damage is a visible duplicate: HIGH, not CRITICAL. Fix: one request transaction (`RequestTx`) opened by the
  middleware holds the claim, the handler's writes and the stored response, and commits once; a non-2xx outcome rolls
  it back. A second request with the same key waits on the uncommitted claim (`lock_timeout` = the lease) and then
  replays, or gets 409 `E_IDEMPOTENCY_IN_FLIGHT` on timeout. The takeover branch is removed. Writes a phase names as
  "in its own transaction" (decision rows, access events, opened views) stay separate: they record an attempt.
- ADV-6-05 MEDIUM (phases 00, 15). A `rating_compute` trigger job whose transaction committed but whose `done` was not
  written is re-queued by the lease reaper; if the review it opened was closed meanwhile, the re-run opens a second
  review (rule 5 absorbs into open reviews only). Haiku said HIGH and proposed a new table; the window is the lease and
  the duplicate is visible, so MEDIUM, and the fix is in the harness. Fix: `JobContext::complete_in(tx)` writes `done`
  in the handler's own transaction, guarded on the claim it holds (`status = 'running' AND locked_by AND locked_at`);
  0 rows (the reaper or another worker took the job) fails the handler and rolls its effects back. Single-transaction
  handlers use it; `rating_compute` does.
- ADV-6-08 MEDIUM (phase 19), the doubt resolved (reading A). Section 4.2 says a repeat trigger counts "only when the
  key is new", but step (iv) of the watch trigger tests nothing; a re-run of `watch_trigger_dispatch` (one transaction
  per watch, so `complete_in` does not fit) adds a second `trigger_count` and a second `review_evidence` work item. Fix:
  the job payload carries the outbox row's `created_at` (`event_at`); `last_trigger_at` stores `event_at`; step (iii)
  skips a watch whose `last_trigger_key` equals the key and whose `last_trigger_at >= event_at`. No new column.
- ADV-6-03 MEDIUM (phase 22). `notification_email` loads the notice and sends without testing `email_state`; a re-run
  after the state write committed (the job's own `done` lost) mails again. Haiku said HIGH and proposed at-most-once,
  which can lose a mail; not taken. Fix: the job loads the notice `FOR UPDATE` and sends only when `email_state =
  'queued'`. The residual window (the relay accepted, the commit failed) can send one duplicate of a generic text; that
  is the at-least-once rule, stated. The digest already selects `digest` notices only.
- ADV-6-04 MEDIUM (phase 00). A row at `outbox.max_attempts` is lost to its consumers (a KYC trigger, a notice, an
  alarm) and only the gauge `akr_outbox_stuck` and the phase 03 cockpit tile show it; no alert rule is named. Haiku said
  HIGH; the cockpit shows it critical, so MEDIUM. Fix: alert rules in `deploy/observability/` for `akr_outbox_stuck > 0`
  and for a rise of `akr_jobs_dead_total`, each a P2 page to the operator.

## LOW
- ADV-6-06 (phase 00), the doubt resolved (reading a, designed). A key is valid for `idempotency.ttl_days` (DR-21:
  only a storage window); a retry after the sweep is a new request. Fix: one sentence in phase 00 for the client rule.

## Rejected
- ADV-6-07 (a replay returns a stored body without an access event or a tier re-check): designed. DR-21 keys the ledger
  by principal and accepts that it holds copies of responses; a replay goes only to the principal who received those
  bytes, and only within the TTL. A path-addressed replay is still decided by the authorize layer first.

## Guarded (accepted as checked)
ADV-6-09 to ADV-6-17: key per principal and request hash; exempt routes refused on repeat; audit emit in the caller's
transaction; chain head under the advisory lock; audit partitions never dropped (superuser bypass designed:
tamper-evident); case chain under the case lock with the chain guard; outbox append takes a transaction and the
dispatcher marks dispatch in the handlers' transaction; notices deduped by `dedupe_key`, alarms by `uq_alarms_live`;
out-of-order delivery re-reads the source row.

DR: DR-412 (the run 6 fixes).
