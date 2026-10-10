# Adversarial pass, run 6 (idempotency, audit chain and outbox replay; Haiku draft for lead review)

### ADV-6-01 Handler commit and idempotency completion are separate writes; a crash re-runs the mutation (verdict: FLAW; phase 00)
- Rule: "Every handler success ... store status, content type and sealed body with `completed_at = now()`" and "Claim: INSERT ... ON CONFLICT DO NOTHING" (phase-00.md:520, phase-00.md:528-529). The replay promise: "a replay returns a byte-identical body" (API.md:39). A mutation must happen once per key.
- Attack:
  1. Role: any authenticated principal with the route's scope (for example `POST /api/v1/cases/{id}/notes` with `Idempotency-Key: note-0001`).
  2. Request 1 runs. Step 4 commits the claim row (`completed_at` NULL) on its own statement (phase-00.md:520). The handler runs its business transaction and commits the note.
  3. The process is killed (OOM, pod restart, switchover) after the business commit and before step 5 (phase-00.md:528) writes `completed_at`. Nothing in the text binds step 5 to the business commit.
  4. Request 2, same key and body, arrives after `idempotency.in_flight_lease_s` (120 s, phase-00.md:242). Step 4's takeover (phase-00.md:527) sets `created_at = now()` and runs the handler again.
  5. The second run commits a second note. Both audit events exist (phase-02 `emit` in each business transaction), so the verifier sees two valid writes, and the replay promise is broken.
  6. Same result when step 5's store itself fails (database error after a committed business write): the text says "Any other outcome -> delete the claim", so the claim is deleted after the business commit and the retry runs the handler again.
- Guard search: the phase text has no transaction that spans steps 4 to 5 and the handler (grep "transaction" in phase-00.md section 3 finds only the job and timer rules; grep "UnitOfWork|with_transaction|tx wrapper" across docs/plan/phase-0*..phase-3*.md finds nothing). The claim is an auto-committed statement. Route-level uniqueness would stop a duplicate in some routes (for example `UNIQUE (proposal_id)` on decisions, phase-18.md:1230), but this is not general; a note, an evidence row or a CR evidence row has no such key. No guard found.
- Severity: CRITICAL (a mutation happens twice, with two audit events, and no error to the caller).
- Proposed 7.x text:
  "### 7.N Idempotency claim, handler writes and completion commit in one transaction (ADV-6-01, CRITICAL)
  In section 3 (`IdempotencyService`), the middleware opens one database transaction before step 4. The claim INSERT, the handler's writes (through the same transaction, handed to the handler as its request transaction) and the step 5 store (`response_status`, `response_content_type`, sealed `response_body`, `completed_at`) all run on it and commit together. A panic, a process death or an error before commit rolls every one of them back. The branch 'any other outcome -> delete the claim' becomes a rollback. A claim row can then exist only with its effects and its stored response. Step 4's takeover UPDATE is removed, because a crashed request leaves no committed claim to take over. Test: kill the process between the business commit point and the response (fault hook in the handler) and assert that a retry with the same key runs the handler once."

### ADV-6-02 A slow first attempt that is still running is taken over after the lease (verdict: FLAW; phase 00)
- Rule: "same hash, in flight and older (the first attempt's process died) -> take the claim over" (phase-00.md:526-527). The text defines takeover for a dead process; the statement does not check that the first attempt is dead.
- Attack:
  1. Role: any principal with the route's scope.
  2. Request 1 starts a long handler (for example a large case export or a Cedar-heavy decision). It runs for longer than 120 s, which the middleware does not cap.
  3. Request 2 with the same key and body arrives at 121 s. Step 4 sees `completed_at IS NULL` and `created_at` older than the lease, and runs the UPDATE of phase-00.md:527. The UPDATE is a plain statement: the claim row was committed by request 1's auto-commit INSERT and holds no row lock.
  4. Request 2 runs the handler. Request 1 is still running and commits its own business write. Both writes commit: two notes, two evidence rows, or a second CR evidence row.
  5. Both requests reply; the second response overwrites the stored body at step 5.
- Guard search: the claim INSERT is auto-committed (phase-00.md:520); no advisory lock or row lock is held during the handler (phase-00.md:520-531). The configured lease does not bound the handler (grep "timeout" in phase-00.md section 3 finds only the job and pool rules). Route-level uniqueness stops a double decision (phase-18.md:1230) but not the other mutations. No guard found for the general case.
- Severity: CRITICAL (a mutation happens twice with two audit events).
- Proposed 7.x text:
  "### 7.N A live first attempt keeps its claim; takeover is removed (ADV-6-02, CRITICAL)
  The fix of ADV-6-01 applies: the claim lives in the open request transaction, so a second request with the same key blocks on the row lock taken by the claim INSERT (READ COMMITTED: the INSERT ... ON CONFLICT waits for the uncommitted row), then reads `completed_at` and replays. The 409 `E_IDEMPOTENCY_IN_FLIGHT` is returned only when that wait exceeds `idempotency.in_flight_lease_s` (set the statement lock timeout to that value). Delete the takeover branch of step 4 (phase-00.md:526-527). Test: a handler that sleeps 3 s with a 1 s lease, two concurrent requests with one key: one handler run, the second answered by replay or 409 after the wait."

### ADV-6-03 E-mail and digest sends are not gated by a committed state, so a re-run mails twice (verdict: FLAW; phase 22)
- Rule: "On success: `sent`, `email_sent_at`, audit `notification.email_sent`" (phase-22.md:734) and the CHECK "`(email_state = 'sent') = (email_sent_at IS NOT NULL)`" (phase-22.md:174). A notice is one message to one recipient; a duplicate notice is a visible second notice (planning rule: HIGH).
- Attack:
  1. Any staff member's notice is created by an outbox handler (for example a `case.decision` notice, phase 22 section 3.8). Its `email_mode` is `immediate` and e-mail is on (`notify.email_enabled`, phase-22.md:733).
  2. The `notification_email` job (payload `{notification_id}`, coalesce key `notification_email:<id>`) loads the notice and calls `MailSender::send`. The relay accepts the message.
  3. The worker dies, or the connection to the database drops, before the job writes `sent` (phase-22.md:734 does the state write only after the send). The lease reaper re-queues the job (phase-00.md:546, job harness step 5).
  4. The coalesce key covers only a queued job (phase-00.md:537), and the job does not re-read `email_state`. The re-run sends again: two e-mails for one notice.
  5. Digest variant: `notification_digest` sends one message per user, then moves the included notices to `sent` "in one transaction per user" (phase-22.md:735). A crash between the send and that transaction re-sends the digest on the next hour.
- Guard search: grep "email_state" in phase-22.md finds only the CHECK, the index (phase-22.md:179-180), the grant and the mapping. No guard that tests `email_state = 'queued'` before the send. `dedupe_key` (phase-22.md:166) guards the notice row, not the e-mail. The SMTP path has no idempotency key (phase-22.md:736 "no retry inside `send`"). No guard found.
- Severity: HIGH (a second notice is visible to the recipient; the outcome is not a forbidden decision).
- Proposed 7.x text:
  "### 7.N At most one e-mail per notice (ADV-6-03, HIGH)
  `notification_email` and `notification_digest` claim the send in a committed short transaction before calling `MailSender::send`: `SELECT ... FROM admin.notifications WHERE id = $1 FOR UPDATE`; proceed only when `email_state = 'queued'` (digest: `'digest'`), then set `email_attempts = email_attempts + 1` and commit. The send then runs. A send that returns an error writes `email_state = 'queued'` back with the closed error code, for the next attempt. A re-run that finds the row already claimed for its attempt number and no outcome does not send again. Trade-off stated for the lead: the at-most-once rule can leave an e-mail unsent after a crash; the in-app notice (phase 22 section 3.8) is unaffected, and the reminder is not the record. Digest: the notices move to `sent` in the same committed transaction that claims the digest, before the send."

### ADV-6-04 An outbox row that exhausts its attempts has no alert, only a gauge (verdict: FLAW; phase 00)
- Rule: "Rows at the attempt limit stay undispatched and are counted by `akr_outbox_stuck` for the cockpit" (phase-00.md:558-559), and the outbox "an undispatched row is evidence that someone was not told" (phase-00.md:346). The downstream effect of a stuck row (a KYC review, a notice, an alarm) is then lost while nobody is told.
- Attack:
  1. A producer (for example phase 14's `screening.hit` decision row, phase-15.md:2041-2042) writes an outbox row in its business transaction.
  2. The topic handler fails on every attempt (a code defect, or a referenced row removed). Each failure adds one to `attempts` with back-off (phase-00.md:555-556). After `outbox.max_attempts` (10, phase-00.md:250) the claim query (`attempts < outbox.max_attempts`) skips the row for good.
  3. The row keeps `dispatched_at` NULL. The only signal is the gauge `akr_outbox_stuck` (phase-00.md:643).
  4. No alert rule names `akr_outbox_stuck`: grep over docs/plan finds it only in the metric lists (phase-00.md:643, phase-22.md:584). The lost effect (no KYC review opened for a party with a screening hit, for example) is seen only if someone opens the cockpit.
- Guard search: grep "outbox_stuck" across docs/plan (only metric and cockpit text); grep "Alert rules" in phase-00.md (none; section 4 defers rules to `deploy/observability/`, phase-00.md:644-645). The cockpit (phase 03) has no outbox row list named. No guard found.
- Severity: HIGH (a lost downstream effect with no alert; the business record itself is kept, so this is not a lost detection).
- Proposed 7.x text:
  "### 7.N Stuck outbox rows page the operator (ADV-6-04, HIGH)
  Add the alert rule `akr_outbox_stuck > 0` for 5 minutes to `deploy/observability/` (phase 00 section 4): P2 incident. The cockpit (phase 03) lists undispatched rows with `attempts >= outbox.max_attempts`: `id`, `topic`, `created_at`, `attempts`, `last_error` (already redacted, phase 00), never the payload. Test: a topic handler that always fails; after `outbox.max_attempts` attempts the gauge reads 1 and the alert rule fires."

### ADV-6-05 A redelivered or re-run trigger job opens a second event review after the first is closed (verdict: FLAW; phase 15)
- Rule: "An open event review of the same party absorbs a new event (its `trigger_code` is updated; the review does not multiply)" (phase-15.md:1310-1311). The absorb rule covers open reviews only. "`event_ref` is never stored on a rating or a review" (phase-15.md:2092). Duplicate reviews are visible to analysts (planning rule: HIGH).
- Attack:
  1. A producer writes `screening.hit` for party P (phase 14 decision, phase-15.md:2041-2042). The dispatcher enqueues `rating_compute {reason: trigger, event: screening.hit, event_ref: alert A}` (phase-15.md:2090-2091) with coalesce key `rating_trigger:P:screening.hit:A`.
  2. The job runs `rescore_and_review` (phase-15.md:2100-2101): it opens event review R1 with `due_basis = trigger_sla`. The job then writes its `rating.trigger_applied` audit (phase-15.md:2104).
  3. The worker dies after the review and audit commit but before `complete(id)` (job harness step 4 of phase-00.md section 3 writes `done` in a separate statement). The lease reaper re-queues the job (phase-00.md:546). This is the designed at-least-once path ("handlers are idempotent (at-least-once)", phase-00.md:546).
  4. An analyst closes R1 (outcome recorded) before the re-run starts.
  5. The re-run finds no open event review for P. It opens R2 with a new deadline and writes a second `rating.trigger_applied`. Two reviews for one event are visible in the K2 queue.
- Guard search: `uq_kyc_reviews_open` (phase-15.md:309) covers open reviews only. The absorb rule (phase-15.md:1311) is for open reviews only. The coalesce key (phase-15.md:2090-2091) covers only a queued job, and the job has finished. "event_ref is never stored" (phase-15.md:2092) means no ledger row exists to test. No marker test was found in section 7.7 (phase-15.md:2083-2128). No guard found.
- Severity: HIGH (a duplicate review is visible to analysts; no forbidden decision is made, and the second review carries the same evidence).
- Proposed 7.x text:
  "### 7.N A trigger applies once per producer message (ADV-6-05, HIGH)
  New table `kyc.trigger_applications` with columns `outbox_id UUID NOT NULL`, `trigger_code TEXT NOT NULL`, `party_id UUID NOT NULL`, `review_id UUID`, `applied_at TIMESTAMPTZ NOT NULL DEFAULT now()` and `PRIMARY KEY (outbox_id, trigger_code)` (`party_id` is a plain column; the name of the party table is not needed for the key). The outbox row id is passed in the `rating_compute` payload as `source_message_id` (the dispatcher hands the row id to the topic handler, phase 00 `TopicHandler`). Step 3 of section 7.7: under the party advisory lock, `INSERT ... ON CONFLICT DO NOTHING`; zero rows means the trigger was applied, so the job skips the action and writes no review and no audit. The table holds ids and codes only and is classified and retained like `kyc_reviews` (phase 00 classification and retention rows). `event_ref` remains unstored on ratings and reviews. Test: run the same `rating_compute` payload twice with a closed review between: one review, one `rating.trigger_applied`."

### ADV-6-06 Idempotency window: a retry after the seven-day sweep runs the mutation again (verdict: DOUBT; phase 00, API)
- Rule: API.md:38-42 says "A replay returns the stored response byte for byte" and says nothing about a window. Phase 00 says "`maintenance_sweep` deletes rows older than `idempotency.ttl_days`" (phase-00.md:531) and the default is 7 (phase-00.md:240).
- Attack: an analyst's client issues `POST /api/v1/cases/{id}/notes` with key K. The response is lost. The sweep deletes the row after 7 days. The client retries K eight days later; the claim is new and the note is posted twice.
- Two readings: (a) the window is a storage limit, so a retry after 7 days is a new request (designed, but no text tells the client); (b) a key is good for the life of the request, so the sweep must not drop rows that a client may still retry.
- Guard search: the uniqueness that stops repeats applies to a few creates only (`POST /admin/users` by e-mail, phase-03.md:527-529; `POST /me/tokens` by step-up, phase-01.md:543-548). Notes, evidence and CR rows have no natural key. For the lead: state the window in API.md and the client rule, or confirm reading (a) as designed.
- Severity if a flaw: MEDIUM.

### ADV-6-07 A replayed create returns a sensitive body with no access event and no tier re-check (verdict: DOUBT; phase 00, phase 02, phase 03)
- Rule: "every access to a restricted or suspicion record is on record before a byte of the record leaves the server" (phase-02.md:715-717, `PgAccessLog`), and the access is written by `AccessScope::record` "after the handler has loaded the record" (phase-02.md:720). Replay: "open the sealed body and replay status, content type and body byte for byte" (phase-00.md:523), without calling the handler.
- Attack:
  1. Analyst A, tier-allowed, sends a sensitive create (a route with `x-akr-access`, for example one that returns a suspicion-tier case summary) with key K.
  2. Analyst A's access to the case (`case_acl`, phase 18) is removed. Analyst A then replays K.
  3. The authz layer loads only the path resource (phase-03.md:692, step 4 `PathId(kind, param)`). A collection create has no path id, so the tier and ACL checks do not run on the new resource. The idempotency layer runs next and returns the stored body. No handler runs, so `AccessScope` writes no `access_event`.
- Guard search: the authz layer runs before idempotency (phase-03.md:532-534), so path-addressed replays are checked. The access-event write is in the handler only (phase-02.md:720). Grep "replay" in phase-02.md and phase-03.md finds no replay access rule. DR-21 accepts that the ledger "holds copies of responses that may carry suspicion-tier content" (research.md:32).
- Two readings: (a) the caller created the record and gets back the bytes it already had, which DR-21 accepts; (b) a replay is a read of a sensitive record, so it needs the access event and the current tier check.
- Severity if a flaw: HIGH (a replay returns data the caller may no longer see, with no access event).
- Lead decision needed; if (b), the fix is to write an `access_event` of kind `replay` in the middleware for responses with `x-akr-access` and refuse the replay with 403 when the current ACL fails.

### ADV-6-08 Redelivered or re-run watch triggers: `trigger_count` and work items have no dedupe (verdict: DOUBT; phase 19)
- Rule: step (iv) of the watch trigger: "`record_trigger` sets `trigger_count + 1`, `last_trigger_at`, `last_trigger_key`" (phase-19.md:1165). Step (v): "`WorkItemService::open` ... creates a `review_evidence` work item" (phase-19.md:1165). The consumer summary: "every handler here is idempotent (a repeat trigger adds a `trigger_count` and an audit event only when the key is new within the same second)" (phase-19.md:1680).
- Attack: the same job re-run as ADV-6-05 (a `screening.hit` for a watched party, phase 15 job lease re-queue) runs `watch_trigger_dispatch` twice. Step (iv) increments `trigger_count` twice; step (v) opens a second `review_evidence` work item (`casework.work_items` has no unique key on kind or source per case: grep "work_items" in phase-18.md finds only the case and state indexes, phase-18.md:213-214).
- Guard search: `last_trigger_key` (phase-19.md:233) is recorded, not tested, in step (iv). The sentence at phase-19.md:1680 reads as a dedupe "only when the key is new within the same second", but no step tests a key or a time window.
- Reading A: the sentence at phase-19.md:1680 describes a dedupe that step (iv) omits. Reading B: the sentence is wrong and the trigger path is not idempotent. Lead to decide; if A, add the test `last_trigger_key = $key AND last_trigger_at > now() - 1 s` to step (iv) and state the key in step (iv).
- Severity if a flaw: HIGH (a duplicate work item in a case is visible).

### ADV-6-09 Replay of a key by another user, another route or another body (verdict: GUARDED; phase 00)
- Rule: "principal-scoped keys" (phase-00.md:198, `PRIMARY KEY (principal_id, key)`); the hash covers "method, the full original path ..., the principal id and the raw body bytes" (phase-00.md:518).
- Guard: another user's key is a different row (PK); another route or body changes `request_hash`, so step 4 returns 422 `E_IDEMPOTENCY_REUSE` (phase-00.md:522). DR-21 (research.md:32).

### ADV-6-10 Exempt mutations repeated by a retry (verdict: GUARDED; phase 01, phase 03)
- Rule: "Every other mutation above requires `Idempotency-Key`" (phase-01.md:548); the exempt routes "POST /admin/users" and "POST /me/tokens" (phase-03.md:527-529, phase-01.md:543-548).
- Guard: each exempt route is either refused on repeat by a unique e-mail (phase-03.md:528-529) or by a single-use step-up assertion consumed in the protected transaction (research.md:40, DR-29). `POST /auth/logout` and heartbeat are naturally idempotent (phase-01.md:544).

### ADV-6-11 Audit event and business change commit or fail together (verdict: GUARDED; phase 02)
- Rule: "the audit emit is the last statement before commit ... A failed emit fails the transaction: a change is never committed without its record" (phase-02.md:680-682).
- Guard: `emit(tx, event)` takes the caller's transaction (phase-02.md:660); a missing transaction is `AuditError::NotInTransaction` (phase-02.md:666). The authz decision row is written by `emit_standalone` before the handler (phase-03.md:707-709) and a write failure gives 503 (not a silent pass).

### ADV-6-12 Two writers take the audit chain head at once (verdict: GUARDED; phase 02)
- Rule: "Writers of a stream are serialised by a transaction-scoped advisory lock" (phase-02.md:651, chain model).
- Guard: `pg_advisory_xact_lock` on the stream key (phase-02.md:663); a check that the lock is held by this backend (phase-02.md:663-665); the head SELECT runs after the lock in its own statement (phase-02.md:668), so under READ COMMITTED it sees the other writer's committed row. Anchor settle point (phase-02.md:746-747) reads the head under the same lock.

### ADV-6-13 Audit partition dropped or detached under retention (verdict: GUARDED; phase 02, phase 23)
- Rule: "`audit.audit_log` and `audit.access_event` are never dropped" (phase-23.md:625); R10 `never` (phase-23.md:457-462).
- Guard: `ENABLE ALWAYS` row triggers and DDL event triggers refuse DROP, DETACH and DISABLE TRIGGER for the owner `akr_migrate` (phase-02.md:50-53, phase-02.md:61-62, phase-02.md:84-90). Residual: a superuser can bypass (designed; phase-02.md:52-53 says the trail is "tamper-evident rather than tamper-proof"). The anchors make a rewrite provable after the anchor is signed, so a rewrite before the next anchor (up to five minutes) is provable only through the chain's own continuity check. Lead may want this in Open doubts; not raised as a flaw here because the owner needs superuser to act.

### ADV-6-14 Case chain: order, fork or gap (verdict: GUARDED; phase 17)
- Rule: "the order of events is the order of time" (phase-17.md:1111-1112); `PRIMARY KEY (case_id, seq)` (phase-17.md:345).
- Guard: append under the case row lock, `seq = last_seq + 1`, `at = now()` bumped by one microsecond when not later than the previous (phase-17.md:1111-1112); `casework.case_events_chain_guard` refuses a row whose `prev_hash` is not the hash of `seq - 1` (phase-17.md:428-446); append-only trigger (phase-17.md:417-420). Note: `now()` is the transaction start, but the +1 us rule covers it.

### ADV-6-15 Outbox row written outside the business transaction, or handler effects without dispatch (verdict: GUARDED; phase 00)
- Rule: outbox rows are written by the business transaction (phase-00.md:435 `OutboxStore { append(tx, topic, payload) }`).
- Guard: the signature takes a transaction; the dispatcher runs the whole handler list and sets `dispatched_at` in one transaction, so a crash re-runs the list from a committed state (phase-00.md:836-838). Exception: work that leaves the database (the e-mail of ADV-6-03).

### ADV-6-16 Notices and alarms written twice on a replayed or concurrent event (verdict: GUARDED; phase 22, phase 19)
- Rule: "a replay of the same outbox row writes no new notice (`dedupe_key`)" (phase-22.md:593).
- Guard: `dedupe_key TEXT NOT NULL UNIQUE` (phase-22.md:166) and `INSERT ... ON CONFLICT (dedupe_key) DO NOTHING` (phase-22.md:403); live alarms by `uq_alarms_live` (phase-19.md:325; phase-19.md:1787 states the refusal).

### ADV-6-17 Out-of-order outbox delivery (verdict: GUARDED; phase 15, phase 22)
- Rule: a later message may dispatch before an earlier one after back-off (phase-00.md:555-556).
- Guard: notices compute recipients from the source row at dispatch (research.md:218, DR-207); the trigger job "re-reads the party (phase 00 rule), so a trigger for a party that has gone ... is a no-op" (phase-15.md:1281). The watch consumer checks the watch state before acting (phase-19.md:1165, step iii).

## Summary
- Counts: FLAW 5; DOUBT 3; GUARDED 9 (ADV-6-09 to ADV-6-17). Total 17.
- FLAW by severity: CRITICAL 2 (ADV-6-01 handler commit and idempotency completion split; ADV-6-02 live first attempt taken over after the lease). HIGH 3 (ADV-6-03 e-mail and digest re-sent after a crash; ADV-6-04 exhausted outbox row with no alert; ADV-6-05 closed event review followed by a re-run trigger that opens a second review). MEDIUM 0. LOW 0.
- DOUBT, one line each: ADV-6-06 the seven-day idempotency window (a retry after the sweep runs the mutation again; API.md is silent on the window). ADV-6-07 a replayed create returns a sensitive body with no access event and no tier re-check (reading A designed per DR-21, reading B a flaw). ADV-6-08 phase 19 step (iv) has no dedupe while line 1680 says a repeat adds a count only when the key is new within a second.
- Not reported again: runs 1 to 5 items (four-eyes, alerts, watermarks, clocks, masking).
- Coverage: phase 00 (ADV-6-01, 02, 04, 06, 09, 15, 17); phase 01 and 03 (ADV-6-10, 07); phase 02 (ADV-6-11 to 13); phase 15 (ADV-6-05, 17); phase 17 (ADV-6-14); phase 19 (ADV-6-08, 16); phase 22 (ADV-6-03, 16, 17).
