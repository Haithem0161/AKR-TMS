#!/usr/bin/env python3
"""Apply adversarial pass run 6 (idempotency, audit chain and outbox replay) after the lead's checks.

Source: tools/agent/adv-run6.md (Haiku draft); decisions: tools/agent/adv-run6-verdict.md.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'

HDR = ('Adversarial pass (2026-10-10), run 6 (idempotency, audit chain and outbox replay), from '
       'tools/agent/adv-run6.md; lead-verified (tools/agent/adv-run6-verdict.md).')

BLOCKS = {
    '00': [
        """### 7.N One transaction for the claim, the handler and the stored response (ADV-6-01, ADV-6-02, HIGH; DR-412)
`IdempotencyService` (section 3) changes as follows; DR-412 replaces the takeover clause of DR-21.
1. After step 3 the middleware opens the request transaction `RequestTx` and runs `SET LOCAL lock_timeout` to
   `idempotency.in_flight_lease_s`. Step 4's claim INSERT runs on it.
2. The handler receives `RequestTx` as its business transaction (an extractor) and never commits it. A step that a
   phase names "in its own transaction" (the decision row of phase 03, access events of phase 02, the opened-view rows of
   phase 18) keeps its own transaction: those rows record an attempt, and a retry may write them again.
3. Handler success (2xx): the business audit emit is still the last business statement (phase 02 rule); the middleware
   then seals the body, writes step 5's columns on the claim row and commits once. Any other outcome rolls the whole
   transaction back, which replaces "delete the claim"; the transient failure stays retryable.
4. A second request with the same key and hash blocks on the uncommitted claim (READ COMMITTED: `INSERT ... ON CONFLICT
   DO NOTHING` waits for the other transaction). When the first commits it reads `completed_at` and replays; when the
   first rolls back its own INSERT succeeds and it runs the handler; when the wait passes `lock_timeout` it answers 409
   `E_IDEMPOTENCY_IN_FLIGHT`.
5. The takeover branch of step 4 is removed: a dead process leaves no committed claim.
Tests: (a) a fault hook that kills the request after the handler returns and before commit: a retry runs the handler
once and the case has one note; (b) a handler that sleeps 3 s with a 1 s lease and two requests with one key: one
handler run, the second gets 409 or the replay.""",
        """### 7.N Jobs complete in their own transaction (ADV-6-05, MEDIUM; DR-412)
`JobContext::complete_in(tx)` runs `UPDATE admin.jobs SET status = 'done', finished_at = now() WHERE id = $1 AND status
= 'running' AND locked_by = $2 AND locked_at = $3` in the handler's transaction (the values of this worker's claim). One
row: the effects and the completion commit together, and the harness's step 4 write is skipped. Zero rows (the lease
reaper re-queued the job or another worker holds it): the handler returns `JobError::Retryable` and its transaction
rolls back, so only one attempt's effects commit. A handler whose work is one transaction uses it; from phase 15 on
`rating_compute` does. Handlers that run one transaction per item keep a marker per item (phase 19 section 7+).""",
        """### 7.N Lost outbox rows and dead jobs page the operator; the key window (ADV-6-04 MEDIUM, ADV-6-06 LOW; DR-412)
1. `deploy/observability/` holds two alert rules: `akr_outbox_stuck > 0` for 5 minutes, and an increase of
   `akr_jobs_dead_total` over 15 minutes; each is a P2 page to the operator, in addition to the phase 03 cockpit tile. A
   row at `outbox.max_attempts` is lost to its consumers (a KYC trigger, a notice, an alarm) until someone acts, so it
   must not wait for someone to open the cockpit.
2. An `Idempotency-Key` is valid for `idempotency.ttl_days` (DR-21: a storage window). A retry after the sweep is a new
   request; the generated client retries within minutes and makes a new key for a new user action.""",
    ],
    '15': [
        """### 7.N A trigger job applies once (ADV-6-05, MEDIUM; DR-412)
Section 7.7: the `rating_compute` handler finishes with `JobContext::complete_in(tx)` (phase 00 section 7+) in the
transaction that writes the re-score, the review and the audit `rating.trigger_applied`. A re-run after a lost
completion therefore finds the job taken or done and writes nothing, so a review closed in between is not followed by a
second review for the same event. `event_ref` is still never stored.""",
    ],
    '19': [
        """### 7.N A watch trigger counts once per event (ADV-6-08, MEDIUM; DR-412)
The payload of `watch_trigger_dispatch` gains `event_at` (the `created_at` of the outbox row). Step (iii) of the watch
trigger also skips the watch when `last_trigger_key` equals this trigger's key and `last_trigger_at >= event_at`; step
(iv) stores `last_trigger_at = event_at`. A re-run of the job, or an older event of the same key delivered late, then
adds no `trigger_count`, no work item and no audit. This is the rule section 4.2 states ("only when the key is new");
its words "within the same second" are replaced by it. Test: run the same payload twice: `trigger_count` 1 and one
`review_evidence` work item.""",
    ],
    '22': [
        """### 7.N An e-mail is sent only for a queued notice (ADV-6-03, MEDIUM; DR-412)
Section 3.8.3 Immediate: `notification_email` loads the notice `FOR UPDATE` and sends only when `email_state =
'queued'`; any other state ends the job with no send. A re-run after the `sent` write committed therefore sends
nothing. The remaining window (the relay accepted the message and the state write failed) can send one more copy of the
generic text: delivery is at least once (DR-209 texts carry no data). The digest already selects notices in state
`digest` only and moves them to `sent` in one transaction per user.""",
    ],
}

out = {}


def load(p):
    if isinstance(p, str):
        p = PLAN / f'phase-{p}.md'
    return p, out.get(p, p.read_text())


for pp, blocks in BLOCKS.items():
    p, s = load(pp)
    st = s.index('## 7+. Gap Additions')
    nx = s.find('\n## ', st + 5)
    assert 'Adversarial pass (2026-10-10), run 6' not in s[st:nx if nx != -1 else len(s)], pp
    out[p] = add_section7(s, HDR, blocks)

r = PLAN / 'research.md'
p, rs = load(r)
anchor = [l for l in rs.split('\n') if l.startswith('| DR-411 |')]
assert len(anchor) == 1
DR = ("| DR-412 | 2026-10-10 | Adversarial run 6 (idempotency and replay): the idempotency claim, the handler's writes and "
      "the stored response commit in one request transaction; a second request waits on the claim and replays or gets "
      "409; the takeover of DR-21 is removed (phase 00). Jobs may complete in their own transaction "
      "(`JobContext::complete_in`, used by `rating_compute`, phases 00, 15); a watch trigger counts once per event by "
      "`event_at` (phase 19); an e-mail is sent only for a queued notice (phase 22); stuck outbox rows and dead jobs "
      "page the operator, and a key is valid for `idempotency.ttl_days` (phase 00) | A crash or a slow first attempt "
      "could commit a create twice, and job re-runs could duplicate reviews, work items and mails; replay of a stored "
      "body to its own principal stays designed (tools/agent/adv-run6-verdict.md) |")
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + DR + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
