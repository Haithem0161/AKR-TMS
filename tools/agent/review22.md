# Phase 22 review findings (lead, 2026-10-09)
1. HIGH branch item scope: 3.1 step 2 and DR-205 list an item to every branch where the subject party has an account
   (`branch_of_party_account`, through `canon.account.org_branch_id`, which does not exist: canon.account has branch_id
   -> canon.branch.org_branch_id, phase 10 l.381/636). A party with accounts at two branches shows the RFI addressed to
   branch A's liaison to branch B's liaison, who can also answer it: M-43 (the addressed liaison only) broken and the RFI's
   existence leaked to another branch. Fix: the item's branch is the branch of its recipient: phase 19 stores in
   `recipient_ref` the canon.staff row of the liaison (phase 19 3.1 step 4), so the predicate is
   `rfi.recipient_kind = 'liaison' AND EXISTS (canon.staff s JOIN canon.branch b ON b.id = s.branch_id WHERE s.id =
   rfi.recipient_ref AND b.org_branch_id = :branch)`; the party-account check (observations, confidential reports) joins
   canon.account.branch_id -> canon.branch.org_branch_id. Open doubt 1 rewritten.
2. HIGH outbox consumers: phase 00's outbox row has one `dispatched_at` and the registry routes a topic to one
   `TopicHandler` (phase 00 l.550-555). `case.event` is already handled by phase 17 (counts), `case.decision` and
   `internal_report.recorded` by phase 15 (triggers); "each consumer keeps its own cursor" (3.8) does not exist, so the
   dispatcher cannot receive those topics without displacing the earlier handler. Fix: phase 00's registry maps a topic to
   an ordered list of handlers, run in registration order in the dispatch transaction (each under a savepoint is not
   needed: every handler is idempotent and a failure retries the row for all); the row is dispatched when every handler
   succeeded. The dispatcher is appended to `case.event`, `case.decision`, `alarm.raised` and the other topics; gap item
   for phase 00.
3. MEDIUM branch document answers: phase 19 `rfi_answers` CHECKs make `corroboration <> 'none'` require
   `corroborated_by`/`corroborated_at`, `documented` iff `evidence_object_id`, one evidence object per claim; the draft
   lets a branch `document` answer carry up to 5 evidence ids and become `documented` by its own maker, which also defeats
   the M-45 closure gate (an explanation corroborated by whoever gave it). `record_answers` is the Cedar `rfi.answer`
   route method (case lock, case team). Fix: branch claims are written with `corroboration = 'none'` and
   `evidence_object_id` NULL; a `document` answer's sealed value lists the staged evidence ids (1 to 5); the desk raises
   the corroboration through phase 19's `corroborates` path naming one of those objects (4.5 item 2 keeps the
   `rfi_input` owner-kind extension for that path); the branch calls a phase 19 inner method
   `RfiService::record_answers_in_tx(tx, actor, rfi_id, answers, source_kind)` without the `rfi.answer` Cedar check (the
   branch route authorises with `branch.item_answer`); gap item for phase 19.
4. MEDIUM observation subject: `casework.observations.party_id` is NOT NULL and must be a customer of the branch, so the
   counter observations S-22 is about (a walk-in depositor, a person instructing the customer, a companion) cannot be
   recorded. Fix: `party_id` nullable (NULL = no customer named; the person is described in the sealed notes and the
   companion ID); the hybrid job stops at once when `party_id` is NULL.
5. LOW the monthly draft and B1 counts (seen by the liaison and the alternate) include confidential reports recorded in
   the period: the alternate learns that the liaison reported (M-68: the reporter's identity is M's only). Fix: the
   branch counts carry items and observations only; confidential reports are counted on R5 for M.
6. LOW `branch.liaison_due_day` floor `{"min": 1}` lets a bank move the due day past the CBI's 10th; the phase 00 floor
   takes `max` (phase 01 uses it). Fix: floor `{"max": 10}`.
7. LOW names: `BranchService` is phase 03's service (phase 03 l.912, admin branches); this phase's branch window service
   is renamed `BranchWindowService` (phase 03's keeps its name in 4.5 item 6). The case event is `case.assigned`
   (phase 17 l.1166), not `case.assign`. The e-mail section numbered 3.9.1 after 3.11 becomes 3.8.3. The Cedar schema
   block is missing its closing fence.
Severity: 0 critical / 2 high (1, 2) / 2 medium (3, 4) / 3 low (5, 6, 7) = 7.
Gap-pass items (not draft defects): phase 00 multi-handler topics; phase 19 `record_answers_in_tx`, `corroborates` accepting
`rfi_input`, release chain event `case.rfi_released {rfi_id, recipient_kind}` (phase 19 writes none today), register
detail shows observation fields to M; phase 03 one-branch rule, `UserRoleQuery`, MASKED_KINDS, outbox
`credential_reset.requested`; phase 04 `notify` effect params (DSL E.3), remind/escalate write `notify.request`,
ChangeTargetRegistry `notification_template`; phase 02 owner kind `rfi_input`; phase 16/17 `DetectionPayload.source_kind`;
phase 23 registry handlers for the six new tables; API.md SSE path (`/events` vs phase 17 `/events/counts`); roadmap row 22
SseHub superseded.
