# Refuter, phase 17 (Haiku draft for lead review)

## V-17-01 Quarantine of payloads that cannot form an alert

### V-17-01-a  STANDS   (first agent: MEDIUM; refuter: HIGH)
- Claim: The UPDATE grant on alert.quarantine omits `reason_detail`, which the retry job writes (`valid_now`).
- Evidence: phase-17.md:551 "GRANT UPDATE (status, resolution_code, resolved_by, resolved_at, released_alert_id, revalidated_at, updated_at, version)"; phase-17.md:963 "gets `reason_detail.valid_now = true`"; phase-17.md:543 "Every write path, with the UPDATE column lists the services use." `grep -n reason_detail docs/plan/phase-*.md` returns lines 215, 648, 940 and 963 only; no grant names the column. The grant is to akr_app, the role the worker's job tables use (phase-00.md:154). Severity HIGH: a missing column grant makes the retry's UPDATE fail on every valid row.
- Fix: phase 17, section 1.5 (the 0061 GRANT block): add `reason_detail` to the UPDATE column list of `alert.quarantine`.

### V-17-01-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: No repository trait for alert.quarantine (insert, lock, update) and no QuarantineService signatures for release, reject, retry.
- Evidence: phase-17.md:800 "**Repository traits (domain, selected methods):**"; the trait block (lines 800-823) holds AlertRepository and CaseRepository only. `grep -n QuarantineRepository docs/plan/phase-17.md` finds nothing; `grep -n "release(\|reject(\|retry(" docs/plan/phase-17.md` finds nothing. The only QuarantineService method named is `QuarantineService::record_standalone(request)` (phase-17.md:938). The release steps (947-956) name the checks and their order, so the logic is inferable; the missing part is the signatures. Phase 11 has its own `QuarantineRepository` (phase-11.md:575) for the CBS ingest quarantine; it does not cover alert.quarantine.
- Fix: phase 17, section 2, after the CaseRepository trait: add a QuarantineRepository trait (insert, lock, update) and method signatures for QuarantineService release, reject and retry.

### V-17-01-c  STANDS   (first agent: LOW; refuter: LOW)
- Claim: The DTO field is `reason_code` but reject requires `resolution_code`.
- Evidence: phase-17.md:795 "`ResolveQuarantine { reason_code?: String }` (reject requires `resolution_code`, a `CLS_` code)"; phase-17.md:958 "requires `resolution_code`, a `CLS_`"; phase-17.md:1702 "without `resolution_code` → 422". The names differ and no line maps one to the other.
- Fix: phase 17, section 2 DTO list (line 795): rename the ResolveQuarantine field to `resolution_code`.

### V-17-01-d  PARTLY   (first agent: MEDIUM; refuter: MEDIUM for the lock part; 409 and 422 parts refuted)
- Claim: Reject has no row lock, no 409 for a row that is not open, no 422 for a missing or non-CLS_ code.
- Evidence (422 refuted): phase-17.md:1702 "without `resolution_code` → 422"; API.md:985 "| `E_VALIDATION` | 422 | Field or document validation". A non-CLS_ code is refused by the column CHECK at phase-17.md:218 "resolution_code ~ '^CLS_[A-Z0-9_]{2,60}$'" and by the field validation rule.
- Evidence (409 refuted): API.md:982 "| `E_NOT_ALLOWED_IN_STATE` | 409 | Action not allowed in the resource's status |"; the release step names the same check at phase-17.md:947 "Status must be `open`, else 409".
- Evidence (lock stands): phase-17.md:947 "Lock the row `FOR UPDATE`" is written for release only. `grep -n "FOR UPDATE"` over phase-17.md lines 958-965 finds nothing; the reject route row (phase-17.md:847) names only `If-Match`. The guard at phase-17.md:483 blocks a status change on a resolved row with a database error (check_violation), not a 409, so a reject racing the release job ends in an error.
- Fix (lock part): phase 17, section 3.2 Reject: add the row lock and the open-status check in the same words as release step 1.

### V-17-01-e  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The retry job writes rows the release job locks, but names no lock and no status re-check.
- Evidence: phase-17.md:962-964 (retry text): no lock, no re-check under a lock; `grep -n "FOR UPDATE"` over lines 962-965 finds nothing; phase-17.md:1527 (job row) "re-validates open rows of the four retryable reasons; never releases" names no lock. The guard at phase-17.md:483-484 blocks only status and resolution_code changes, so a retry write to reason_detail or revalidated_at can still land on a row released a moment earlier. The general rules do not cover it: phase-00.md:564-566 "Rule for every timer handler" covers timers only, and phase-00.md:547 "handlers are idempotent (at-least-once)" covers replays, not a race with the release job.
- Fix: phase 17, section 3.2 Retry job: lock each row and re-check that it is still open under the lock, as release step 1 does.

### V-17-01-f  PARTLY   (first agent: MEDIUM; refuter: MEDIUM for the part that stands)
- Claim: The 422 path of release writes revalidated_at and an audit event; the spec does not say that this transaction commits.
- Evidence (commit rule exists): phase-00.md:1027 "Any other outcome rolls the whole" (7.17, item 3: only a 2xx handler commits, and every other outcome rolls back). The release route carries `Idempotency-Key` (phase-17.md:846), so the rule applies to it. `grep -n "commit\|rollback"` over phase-17.md section 3.2 finds nothing, but the general rule answers the question.
- Evidence (part that stands): phase-17.md:948 "Still invalid: set `revalidated_at`, audit `alert.quarantine_revalidated`," then answer 422. Under phase-00.md:1027 these writes are rolled back; the only exemption is for steps a phase names "in its own transaction" (phase-00.md:1025, "keeps its own transaction"), and phase 17 does not name one here. So the revalidation record and its audit event are lost on that path, and the two texts disagree.
- Fix: phase 17, section 3.2 Release step 2: say whether the revalidation record runs in its own transaction (phase 00 7.17, item 2) or is not written on the 422 path.

## V-17-02 Alert and case statuses as events

### V-17-02-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: No method or type appends alert events: no append method on AlertRepository and no NewAlertEvent.
- Evidence: phase-17.md:804-812 (AlertRepository) lists resolve_id, insert, lock, update, queue_page, claim_next and count_wip; none appends. `grep -n "NewAlertEvent\|AlertEventRepository" docs/plan/phase-*.md` finds nothing; `AlertEvent` occurs only as an entity name (phase-17.md:783). Writers that name no method: phase-17.md:904 "`alert.recorded` (sequence 1, `alert_events`)"; phase-17.md:1139 "`alert_events` `alert.claimed {via: 'claim' | 'next'}`". Phase 18 also writes the alert trail with no method (phase-18.md:942 "alert trail (`alert.alert_events`)").
- Fix: phase 17, section 2: add an append method and a NewAlertEvent type to the AlertRepository trait, or to a separate alert-event repository.

### V-17-02-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: alert_events.seq is not allocated: alert.alerts has no last_seq, and no lock or max-plus-one rule is stated for the alert trail.
- Evidence: phase-17.md:194 "seq                 INT NOT NULL CHECK (seq >= 1)," and phase-17.md:201 "UNIQUE (alert_id, seq)". `grep -n last_seq docs/plan/phase-17.md` gives no hit inside the alert.alerts DDL (lines 106-174). The only seq rule is for cases: phase-17.md:1111 "under the case row lock (`lock(tx, id)`), `seq = last_seq + 1`". Intake writes sequence 1 only (phase-17.md:904). Claim step 8 (phase-17.md:1139) writes the next alert event with no seq rule, although claim step 4 locks the alert row (phase-17.md:1131), so a rule is inferable.
- Fix: phase 17, section 3.7: state the seq rule for alert_events (alert row lock, then max plus one, or a last_seq column on alert.alerts) and say that it covers every writer.

### V-17-02-c  PARTLY   (first agent: MEDIUM; refuter: MEDIUM for NewCaseEvent, LOW for CaseEventKind)
- Claim: NewCaseEvent (argument of append_event) and CaseEventKind (field of CaseEvent) are never defined.
- Evidence (part that stands, NewCaseEvent): phase-17.md:820 "e: NewCaseEvent". `grep -n "NewCaseEvent\|CaseEventKind" docs/plan/phase-*.md` returns only phase-17.md:741 and 820, so NewCaseEvent has no definition anywhere. Its fields can be inferred from CaseEvent without seq, prev_hash and hash: MEDIUM.
- Evidence (CaseEventKind is mostly covered by a near name): phase 02 defines the catalogue type `EventKind { name, resource_type, stream: Stream, ... }` (phase-02.md:549) and `Stream { AuditLog, AccessEvent, CaseEvent }` (phase-02.md:610), and every later phase appends its own kinds (phase-02.md:551). The case kinds are probably that catalogue entry for stream CaseEvent, but no text says so: LOW.
- Fix: phase 17, section 2, after the CaseEvent struct: define NewCaseEvent, and state whether CaseEventKind is phase 02's EventKind with stream CaseEvent or a new type.

## V-17-03 Assignment (push) of an alert

### V-17-03-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: No method signature for AssignmentService (alert and case assign), so the caller, target and version parameters are not named.
- Evidence: `grep -n AssignmentService docs/plan/phase-17.md` gives phase-17.md:1124 (the 3.8 heading), 1551 (AppState), 1795 and 1803 (7.2, 7.3); `grep -n "assign(\|fn assign"` finds nothing; section 2 has no AssignmentService trait. The assign steps (phase-17.md:1161-1169) are numbered and name their checks, so the logic can be inferred.
- Fix: phase 17, section 3.8: add method signatures for alert assign and case assign that name the caller, the target and the If-Match parameter.

### V-17-03-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Assign does not say which alert states it accepts; triaged or duplicate would become claimed and break the 0059 CHECKs, and a no_longer_matches alert would be revived.
- Evidence: the assign steps (phase-17.md:1161-1164) name no state check; `grep -n E_NOT_ALLOWED_IN_STATE` gives phase-17.md:947 (release), 1132 and 1172 (claim), 1158 (triage) and 1328 (the error table), and none in the assign steps (1161-1164); 7.12 (phase-17.md:1882-1891) names no state. The DDL CHECKs that an assign on these states would break: phase-17.md:168 "CHECK ((state = 'duplicate') = (duplicate_of IS NOT NULL))" and phase-17.md:169 "CHECK ((state = 'triaged') = (triaged_at IS NOT NULL))". The rule exists for unclaim (phase-17.md:1148 "A triaged or duplicate alert cannot be unclaimed") and triage, but not for assign.
- Fix: phase 17, section 3.8 Assign step 3: name the alert states that assign accepts and the 409 for the others.

### V-17-03-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Step 1 checks membership of "the queue's pool", but POST /alerts/{id}/assign has no queue and alert.alerts has no pool column.
- Evidence: phase-17.md:1161 "is a member of the queue's pool (role grant scope `pools`," ; the route row phase-17.md:844 path `/api/v1/alerts/{id}/assign` carries no queue id. The alert.alerts DDL (phase-17.md:106-174) has no pool or queue column; the only `queue` there is the partial index name at phase-17.md:174. Pools belong to queues (casework.queues.pool_id, phase-17.md:49), and step 3 says only that "an alert reaches a pool through the queue filter" (phase-17.md:1164), which does not say which queue applies.
- Fix: phase 17, section 3.8 Assign step 1: name the queue (and so the pool) that the membership check uses for an alert, and the rule for choosing it.

### V-17-03-d  STANDS   (first agent: MEDIUM; refuter: HIGH)
- Claim: The route row says "Assign to a user or pool"; step 3 and Decisions say a pool target is refused (422) and an alert goes to a user only.
- Evidence: phase-17.md:844 "Assign to a user or pool; conflict check; `If-Match`"; phase-17.md:793 "`AssignAlert { to: { user_id } | { pool_id }, reason_code?: String }`". Against phase-17.md:1164 "the target is a user only (a pool target is 422" and phase-17.md:2029 "An alert is assigned to a user only." No 7.x block changes it (7.3 at 1802-1806 and 7.12 at 1882-1891 do not). The requirement M-15 (research/28-case-management-workflow.md:1259) reads "pool pull for L1 and specialist queues, push for L2, MLRO and compliance-only", which supports the user-only rule. Two texts contradict, so HIGH under the scale.
- Fix: phase 17, the route row (line 844) and the AssignAlert DTO (line 793): accept a user target only, as step 3 says.

## V-17-04 Bulk assign and bulk mark duplicate

### V-17-04-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: No service method for the bulk dry run or the real run, with the caller, the ids and the dry_run flag named.
- Evidence: `grep -n "bulk(\|fn bulk" docs/plan/phase-17.md` finds nothing. `grep -n QueueService` gives phase-17.md:1186, 1189 and 1246 (queues, items, counts only). The bulk paragraph (phase-17.md:1207-1212) and the route row (phase-17.md:838, `POST /api/v1/queues/{id}/bulk`) name no method.
- Fix: phase 17, section 3.9 Bulk paragraph: name the service method for the dry run and for the real run, with the caller, the ids and the dry_run flag.

### V-17-04-b  STANDS   (first agent: MEDIUM; refuter: HIGH for the conflict check, MEDIUM for the order and pool points)
- Claim: Bulk checks are prose with no order and no pool or conflict reason; a bulk assign skips the conflict check (M-67) and the pool membership check unless the single-assign steps are said to apply.
- Evidence (conflict): the bulk reasons are "(`claimed_by_other`, `not_visible`, `p0_or_p1_duplicate`, `wip_limit`, `lower_refused`)" (phase-17.md:1208-1209). `grep -n conflict` over phase-17.md:1207-1212 finds nothing. Assign step 2 requires "target conflict check (step 7)" (phase-17.md:1163), but no bulk text refers to it. 7.12 (phase-17.md:1887-1889) lists the bulk checks as the WIP lock and the claimed_by re-read, with no conflict check. 7.14 (phase-17.md:1904-1905) closes the list at five reasons, none of them a conflict. The requirement M-67 is "Conflict service at assignment" (research/28-case-management-workflow.md:1311). A bulk assign is an assignment, so the omission is a control gap.
- Evidence (pool and order): `grep -n pool` over phase-17.md:1207-1212 finds nothing; the bulk paragraph gives no order for the item checks (no numbered steps).
- Fix: phase 17, section 3.9 Bulk: state that each bulk assign item runs the assign steps 1 and 2 of section 3.8 (pool membership, WIP and conflict) and give the order of the item checks.

### V-17-04-c  STANDS   (first agent: MEDIUM; refuter: HIGH)
- Claim: 7.11 adds reason `target_is_duplicate` for each bulk mark_duplicate item; 7.14 replaces the bulk reason list with a list that does not include it.
- Evidence: phase-17.md:1877-1878 (7.11) "refuse a target whose state is `duplicate` (422 `E_VALIDATION` at `/duplicate_of`, reason `target_is_duplicate`)". `grep -n target_is_duplicate docs/plan/phase-*.md` finds that line only. phase-17.md:1904-1905 (7.14) "`not_visible` leaves the list of bulk reasons, which becomes `not_found`, `claimed_by_other`, `p0_or_p1_duplicate`, `wip_limit`, `lower_refused`". Neither block says which list wins; 7.14 uses "becomes" but does not mention 7.11. Two texts give different reason lists for the same bulk result, so HIGH under the scale.
- Fix: phase 17, 7.14 (or 7.11): say whether `target_is_duplicate` belongs to the bulk reason list, and give one closed list.


## V-17-05 Evidence snapshot and the alert decision record

### V-17-05-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM for the part that stands)
- Claim: SnapshotService has no method signature (the store call, its tx and its inputs).
- Evidence (refuted part, the store call and its inputs): phase-17.md:973 "3. `EvidenceStore::put_generated(tx, GeneratedEvidence { owner_kind: 'alert', owner_id: alert id, kind: 'snapshot', bytes," with the full field list on the following lines; phase-17.md:979 "4. Return `(object_id, sha256)`." The store call, its tx, its inputs and its return are all named.
- Evidence (part that stands): `grep -n SnapshotService docs/plan/phase-17.md` gives only the heading at phase-17.md:966; `grep -n "fn store\|snapshot("` over the plan finds nothing for SnapshotService. The service itself has no signature (its name, the alert and payload it takes, the transaction and the return type).
- Fix: phase 17, section 3.3: add a SnapshotService method signature (tx, alert id, payload, the return `(object_id, sha256)`).

### V-17-05-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: TxRow (the rows returned by tx_rows and used for the snapshot body) is not defined.
- Evidence: `grep -n TxRow docs/plan/phase-*.md` gives phase-17.md:774 "`Result<Vec<TxRow>, RepoError>`" and phase-17.md:970 "(`TxRow` from `canon.txn` through `ReferenceReader::tx_rows`". No struct named TxRow exists, and phase-10.md has no `pub struct` at all (its canon.txn DDL is at phase-10.md:951). The fields are not named: phase-17.md:970-971 says only "the fields are those the scenario declares, never names or contact values", a dynamic subset of canon.txn. So the exemption for fields stated in prose does not apply, and the representation (typed struct or map) is not stated: MEDIUM.
- Fix: phase 17, section 2 next to ReferenceReader: define TxRow, its representation, and the canon.txn columns it may hold.

## V-17-06 Hibernation gate

### V-17-06-a  PARTLY   (first agent: LOW; refuter: LOW for the part that stands)
- Claim: The block passes `touch_hours` to `BreachService::arm`, but the phase defines a per-class array `casework.touch_by_hours[class]`, and `touch_hours` is defined nowhere.
- Evidence (refuted part, the window is defined): phase-19.md:1290 "The window is the touch window of an alert (`casework.touch_by_hours[class]`, phase 17 section 3.12, working hours)". So the value the call passes has a definition, phase 17 section 3.12 (phase-17.md:1276, the per-class `touch_by_hours[class]`).
- Evidence (part that stands): the name `touch_hours` at phase-17.md:1783 (7.1 step 9) matches no defined name; `grep -n touch_hours docs/plan/phase-*.md` finds it only there and as a different concept in phase-31.md:564. The name mismatch is LOW.
- Fix: phase 17, 7.1 step 9: pass `casework.touch_by_hours[class]` by its defined name.

## V-17-07 Case surfaces extended by phase 18

### V-17-07-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM for the part that stands)
- Claim: The assignment grant has no stated write path or authorization: grant() needs case.access.change (A2, M) and If-Match, but A1 may assign; the block does not say whether the rows are written directly or through grant(), or by which actor.
- Evidence (refuted part, the write path and actor): phase-17.md:1795 "`AssignmentService` (section 3.8) writes the `casework.case_acl` grants of phase 18 section 3.2 in the assignment transaction". That is a direct write in the assignment transaction, not a call to grant(). The actor is the assigner: phase-18.md:649 "A reassignment revokes the previous assignee's grant (`revoked_by` is the actor)".
- Evidence (part that stands): phase-18.md:653 "**`grant(ctx, case_id, GrantAccess, if_match) -> CaseAccessView`** (`case.access.change`; A2 and M)" and phase-17.md:1397-1402 gives A1 `case.assign` but no `case.access.change`. The block does not say that the assignment's direct write runs under case.assign and does not pass grant()'s A2/M-only gate, so a developer who routes the write through grant() would refuse every A1 assignment.
- Fix: phase 17, 7.2 second bullet: state that the assignment writes the rows directly under the assignment's case.assign check and does not call grant().

## V-17-08 Model rank in the queue order

### V-17-08-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The keyset cursor of the items query does not carry model_rank or the NULLS LAST position, so the next page of the new order is undefined.
- Evidence: 7.5 (phase-17.md:1814-1817) gives the new order "`priority_class, model_rank DESC NULLS LAST, score DESC NULLS LAST, created_at, id`" and says nothing about the cursor. The base item 3 (phase-17.md:1192) says only "Keyset cursor." `grep -n -i cursor docs/plan/phase-17.md` shows no cursor text in 1814-1822. phase-00.md:377 and phase-00.md:413 name a cursor module ("signed cursors") with no keyset rule. phase-27.md:406 is the only cursor text in phase 27 (the API row).
- Fix: phase 17, 7.5 (phase 27 owns the change): one sentence saying which sort keys the keyset cursor carries after model_rank, and where NULLS LAST places a null.

## Summary

- Claims checked: 21 (each Gap ID of tools/agent/verify/phase-17.md handled once).
- REFUTED: 0
- STANDS: 15
- PARTLY: 6

Severity changes against the scale in the brief: V-17-01-a (MEDIUM to HIGH, a missing column grant that the retry job needs), V-17-03-d (MEDIUM to HIGH, two texts contradict on pool targets), V-17-04-b (MEDIUM to HIGH for the missing conflict check on bulk assign; order and pool points stay MEDIUM), V-17-04-c (MEDIUM to HIGH, two texts give different bulk reason lists). No claim was fully disproved: the strongest refutations are the partial ones (V-17-01-d, V-17-01-f, V-17-02-c, V-17-05-a, V-17-06-a, V-17-07-a).

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-17-01-a | STANDS | HIGH | Retry job writes reason_detail; no UPDATE grant names it on alert.quarantine | Phase 17, section 1.5 (0061 GRANT block) |
| V-17-01-b | STANDS | MEDIUM | No QuarantineRepository trait; no QuarantineService signatures for release, reject, retry | Phase 17, section 2, after CaseRepository |
| V-17-01-c | STANDS | LOW | DTO field reason_code, while reject requires resolution_code | Phase 17, section 2 DTO list (line 795) |
| V-17-01-d | PARTLY | MEDIUM | Reject has no row lock; the 409 and 422 parts are covered by API.md:982 and API.md:985 | Phase 17, section 3.2 Reject |
| V-17-01-e | STANDS | MEDIUM | Retry job has no row lock and no open-status re-check under a lock | Phase 17, section 3.2 Retry job |
| V-17-01-f | PARTLY | MEDIUM | Phase 00 7.17 rolls back the 422 path, so release step 2 writes and audit are lost; no own-transaction exemption is named | Phase 17, section 3.2 Release step 2 |
| V-17-02-a | STANDS | MEDIUM | No append method or NewAlertEvent type for alert events | Phase 17, section 2 (AlertRepository) |
| V-17-02-b | STANDS | MEDIUM | alert_events.seq has no allocation rule and alert.alerts has no last_seq | Phase 17, section 3.7 |
| V-17-02-c | PARTLY | MEDIUM (NewCaseEvent), LOW (CaseEventKind) | NewCaseEvent is defined nowhere; CaseEventKind is probably phase 02 EventKind, link not stated | Phase 17, section 2 after CaseEvent |
| V-17-03-a | STANDS | MEDIUM | No method signatures for AssignmentService (alert and case assign) | Phase 17, section 3.8 |
| V-17-03-b | STANDS | MEDIUM | Assign names no accepted alert states; triaged and duplicate would break the DDL CHECKs | Phase 17, section 3.8 Assign step 3 |
| V-17-03-c | STANDS | MEDIUM | "The queue's pool" has no queue on the assign route and no pool on alert.alerts | Phase 17, section 3.8 Assign step 1 |
| V-17-03-d | STANDS | HIGH | Route row (line 844) and DTO (line 793) accept a pool target; step 3 (line 1164) and Decisions (line 2029) say user only | Phase 17, route row line 844 and DTO line 793 |
| V-17-04-a | STANDS | MEDIUM | No service method for the bulk dry run or the real run | Phase 17, section 3.9 Bulk |
| V-17-04-b | STANDS | HIGH (conflict), MEDIUM (order, pool) | Bulk assign has no conflict check (M-67); bulk item checks have no order | Phase 17, section 3.9 Bulk |
| V-17-04-c | STANDS | HIGH | 7.11 adds target_is_duplicate to bulk; 7.14 bulk list of five omits it | Phase 17, 7.14 or 7.11 |
| V-17-05-a | PARTLY | MEDIUM | No SnapshotService signature of its own; the store call, tx, inputs and return are named at 3.3 steps 3 and 4 | Phase 17, section 3.3 |
| V-17-05-b | STANDS | MEDIUM | TxRow is undefined; its fields are only "those the scenario declares" | Phase 17, section 2 next to ReferenceReader |
| V-17-06-a | PARTLY | LOW | Name touch_hours does not match touch_by_hours[class]; the window itself is defined at phase-19.md:1290 | Phase 17, 7.1 step 9 |
| V-17-07-a | PARTLY | MEDIUM | Write path is stated (direct, in the assignment transaction); not stated that it bypasses grant()'s A2 and M gate | Phase 17, 7.2 second bullet |
| V-17-08-a | STANDS | MEDIUM | Keyset cursor does not carry model_rank or the NULLS LAST placement | Phase 17, 7.5 (phase 27 owns the change) |
