# Verification pass: lead verdict (2026-10-10; in progress)

Round 1: 32 Haiku agents, one per phase (prompt tools/agent/p-verify.txt), 12 items each, truths T1 to T7; outputs
tools/agent/verify/phase-NN.md. Round 2: a Haiku refuter per phase (prompt tools/agent/p-refute.txt), outputs
tools/agent/verify/refute-NN.md. The lead checks every STANDS and PARTLY line against the phase text and spot-checks the
REFUTED ones. A line below is the lead's decision.

Form: `V-NN-KK-a | CONFIRMED severity | what is missing | fix` or `V-NN-KK-a | REJECTED | the line that disproves it`.

## Phase 04 (first agent: 12 items, 3 pass, 13 claims; refuter: 1 refuted, 8 stand, 4 partly; lead: 12 confirmed)
- V-04-08-a | CONFIRMED HIGH | 7.2 appends to `change_requests.evidence` but migration 0014 grants `akr_app` UPDATE on a
  column list without `evidence`; the append would fail with a permission error | REP: the grant gains `evidence`.
- V-04-01-a | CONFIRMED LOW | `StepUpAssertionId`, `ActorRef`, `Activated` and the `d` of `decide` are not tied to
  their types | block.
- V-04-01-b | CONFIRMED LOW | `escalated` in 3.2, `task_escalated` in 3.4 | REP.
- V-04-02-a | CONFIRMED MEDIUM | inventory Create: no code for an inactive owner or validator, no order | block
  (`owner_not_active` reused from phase 12; `validator_not_active` new).
- V-04-03-a | CONFIRMED LOW | `post_review_recorded` missing from the 3.4 event list | REP.
- V-04-05-a | CONFIRMED LOW | `/inbox/counts` polled every 60 s (phase 04, 17) and every 30 s (phase 08) | REP in 08.
- V-04-06-a | CONFIRMED LOW (partly) | the reason tag is on no phase 08 change-request screen | phase 08 block.
- V-04-07-a | CONFIRMED LOW (partly) | no variant carries a `ChangeError` out of the subject | block.
- V-04-08-b, V-04-08-c | CONFIRMED MEDIUM | the evidence route has no Cedar action and no repository method | block
  (`change_request.evidence_add`, `append_evidence`).
- V-04-08-d | CONFIRMED MEDIUM | `evidence_closed` tests approve or reject events the engine never writes | REP: the
  task `outcome` values `approved`, `rejected`.
- V-04-12-a | CONFIRMED LOW (partly) | the return step of `mode: rework` is unnamed | block (lead decision: the reject
  names `to`, as the ported engine does; no shipped definition uses rework).
- V-04-10-a | REJECTED | 7.10 names the seed it replaces, so the block wins (DR-361).

## Phase 03 (first agent: 12 items, 6 pass, 7 claims; refuter: 0 refuted, 4 stand, 3 partly; lead: 7 confirmed)
- V-03-01-a | CONFIRMED LOW | step 6 of 3.2 names `Authorizer::is_authorized`; the section 2 trait has `authorize` |
  REP: step 6 runs inside `Authorizer::authorize`; `is_authorized` is the cedar-policy call.
- V-03-02-a | CONFIRMED LOW | phase 04 section 4 cites phase 03 "section 3.5" for the `sod.json` pair; it is in 3.1 |
  REP in phase 04.
- V-03-03-a | CONFIRMED MEDIUM (the action; the missing route-table row is designed, section 3.11) | the layer needs a
  `RouteAction` for every `/api/v1` route and no development route (`/_echo`, `/_dev/masked`, phase 17
  `/_dev/detections`) has one | phase 03 block: `dev.*` actions and `policies/dev.cedar` under the `dev-routes` feature.
- V-03-05-a | CONFIRMED MEDIUM (first agent HIGH: one method, meaning clear) | `update_traits` is in no Kratos port |
  phase 01 block: the admin API gains `update_traits(id, traits)` from phase 03 on.
- V-03-07-a | CONFIRMED LOW | "the alternate needs a liaison" has no code; `in_use` is not tied to the archive
  refusal; the transaction and order are covered (phase 00 `RequestTx`) | three REPs (`liaison_required`, `in_use`).
- V-03-11-a | CONFIRMED LOW | the no-database matrix build has a command but no method | phase 03 block.
- V-03-11-b | CONFIRMED MEDIUM | 7.12 derives the file from `@role` annotations, 3.5 step 1 by evaluation | same
  block: one derivation, `AccessMatrixBuilder::roles(bundle)`, used by both.

## Phase 01 (first agent: 12 items, 3 pass, 11 claims; refuter: 6 refuted, 3 stand, 2 partly; lead: 5 confirmed)
- V-01-01-a | CONFIRMED LOW | the Goal line calls session minima floors; DR-34 makes session limits hard bounds | REP.
- V-01-02-a | CONFIRMED MEDIUM | `akr-ctl identity client create`, `rotate`, `disable` have no order, transaction or
  failure path | phase 01 block.
- V-01-04-a | CONFIRMED MEDIUM | heartbeat and logout have no Cedar action, and the phase 03 layer needs one per route
  | phase 01 block: reuse `session.read` and `session.revoke`.
- V-01-05-a | CONFIRMED MEDIUM | the `pat.max_per_user` count runs before the transaction under no lock | phase 01
  block: advisory lock per user inside step 3.
- V-01-07-a | CONFIRMED MEDIUM (a coverage hole) | SP-36 (behaviour rule pack, P0-P1 by ROADMAP risk row 5) is carried
  by no phase; phase 25 points to phase 24, which has nothing | phase 25 block: the rules the exported kinds can feed
  ship in `deploy/siem/use_cases.yaml`; the rest is P2 because `authz.allowed` is not exported. TELL THE USER.
- V-01-08-a, V-01-09-a, V-01-09-b, V-01-10-a, V-01-10-b, V-01-11-a | REJECTED (the thing exists; only the section
  number or a line number is off) | corrected in passing by six REPs.

## Phase 05 (first agent: 12 items, 8 pass, 6 claims; refuter: 2 refuted, 1 stands, 3 partly; lead: 4 confirmed)
- V-05-06-a | REJECTED | phase 02 section 2 names `put_generated` for the worker's certificates.
- V-05-06-b | CONFIRMED MEDIUM (refuter HIGH: the right method exists, the base text names the wrong one) | section
  3.6 stores the worker's run report through `EvidenceStore::stage`, the akr-api upload path | REP to `put_generated`.
- V-05-06-c | REJECTED | 7.2 gives the check branch in one ordered sentence.
- V-05-08-a | CONFIRMED LOW (partly) | what a permit already out counts against after a governor rebuild | block.
- V-05-02-a | CONFIRMED LOW (partly; change control is in phase 10) | the header says VD-02 is implemented here, but
  the mapping is derived by its author in phase 10 | REP of the header item (lead decision B: no product-drafted
  mapping).
- V-05-11-a | CONFIRMED MEDIUM (partly; no backfill job exists by DR-380) | phase 05 says the pause row feeds the
  coverage gap and a backfill; phases 11 and 12 use `admin.blind_periods` | block states the one rule.

## Phase 00
V-00-02-a | CONFIRMED MEDIUM | SP-74 names frontend packages; CI step 5 listed crates and images only | REP 00 CI step 5
V-00-02-b | CONFIRMED MEDIUM | models are outside the release (DR-390) and the plan did not say so for the SBOM; the manifest had no publisher country (VO-07) | BLOCK 00 (SP-74), BLOCK 28 (publisher country)
V-00-02-c | CONFIRMED MEDIUM (refuter HIGH: a release artefact, not a build blocker) | no VEX statement | BLOCK 00 (SP-74); reuses `sbom_missing`
V-00-03-a | CONFIRMED MEDIUM | SP-84 WAL-gap alarm not carried | BLOCK 00, alert rule on the postgres exporter's archiver series (lead decision A)
V-00-05-a | CONFIRMED MEDIUM | crypto port had no error type; `CryptoError` used by phase 12 and defined nowhere | BLOCK 00
V-00-06-a | CONFIRMED MEDIUM | `TopicHandler` named in six phases, signature and error type nowhere | BLOCK 00 (reuses `JobError`)
V-00-07-a | REJECTED | kind name, concurrency, lease override and max attempts are all stated (refuter)
V-00-07-b | CONFIRMED LOW | heartbeat matching no row: reaction not stated | REP 00 7.2
V-00-09-a | CONFIRMED LOW | no stated rule for sweep deletes (a) and (b) | BLOCK 00: lead decision B, operational state is not audited
V-00-10-a | CONFIRMED LOW | same rule for timer rows | same BLOCK 00
V-00-12-a | CONFIRMED HIGH (raised by the lead) | 7.18 guarded completion on the claim's `locked_at`, which the 7.2 heartbeat moves: every job longer than a third of its lease would roll back | REP 00 7.18: guard on `attempts`
V-00-12-b | CONFIRMED LOW | pointer to a phase 19 marker that does not exist | REP 00 7.18

## Phase 02
V-02-01-a | CONFIRMED LOW | `AccessReceipt` contents not stated | BLOCK 02
V-02-01-b | CONFIRMED LOW | the cfg(test) route has no scope line and the Cedar line said no route is added | 2 REP 02
V-02-03-a | CONFIRMED LOW | `PartitionMaintenance` has no method signature | BLOCK 02
V-02-03-b | CONFIRMED LOW | effect of a failing table on later tables | same BLOCK 02
V-02-04-a | CONFIRMED MEDIUM | severity, SIEM flag and detail keys of the phase 00 kinds; the lead found the same for phase 01 | BLOCK 02 (one rule for phases 00, 01, 02)
V-02-05-a | CONFIRMED LOW | `RenderJob` fields | BLOCK 02
V-02-05-b | CONFIRMED LOW | job outcome for `Refused` and `FontMissing` | BLOCK 02
V-02-07-a | CONFIRMED LOW | `ShredOutcome` contents | BLOCK 02
V-02-07-b | REJECTED | the row exists; section number corrected in passing | REP 02
V-02-07-c | REJECTED | the refusal exists; pointer corrected in passing | REP 02
V-02-09-a | CONFIRMED MEDIUM | no steps for the case_event anchor; the section 3 steps assume one chain and a stream lock, and the anchor row needs ids the case events do not have | BLOCK 02 (heads as leaves, shared-lock settle), BLOCK 17, 2 REP 17
V-02-10-a | CONFIRMED MEDIUM | version listing and retention read on akr-anchors not in the worker credential | BLOCK 02; NEW `s3:GetObjectRetention`
V-02-11-a | REJECTED | `open` and its verified reader are specified (refuter, drafter DROP)
V-02-11-b | CONFIRMED LOW | stale generic evidence route path (DR-380) | 2 REP 02
V-02-11-c | CONFIRMED LOW | four row types without a field rule | BLOCK 02
V-02-11-d | CONFIRMED LOW | R12 cited for infected and rejected objects; R12 does not cover them | REP 02 (lead: kept, owner clock)

## Phase 16
V-16-01-a | CONFIRMED MEDIUM | no refusal for a taken scenario code; the draft's new `insert` method dropped, `insert_missing` returning 0 is enough | BLOCK 16
V-16-01-b | CONFIRMED LOW | `ScenarioCreate` fields | REP 16
V-16-01-c | CONFIRMED LOW | status of `owner_not_active` | BLOCK 16 (with V-16-03-a)
V-16-03-a | CONFIRMED MEDIUM | drafting reason check had no refusal | same BLOCK 16; reuses `reason_not_allowed`
V-16-03-b | CONFIRMED MEDIUM | two drafts could take the same `set_version` | REP 16 (lock the scenario row)
V-16-03-c | REJECTED | refuted by the refuter
V-16-03-d | CONFIRMED LOW | L-18 against DSL D.3 for minor sets | BLOCK 16: lead decision A, DSL D.3 holds
V-16-04-a | CONFIRMED LOW | reason for a version not in `testing` | BLOCK 16; NEW `not_testing`
V-16-05-a | CONFIRMED MEDIUM | no run steps for unit cases; automatic cases not carried | BLOCK 16
V-16-06-a | CONFIRMED LOW | `pack_not_enabled` missing from the 3.17 list | same BLOCK as V-16-04-a
V-16-07-a | CONFIRMED LOW | protected-attribute refusal had no code | REP 16; reuses `E_PROTECTED_ATTRIBUTE`
V-16-10-a | CONFIRMED MEDIUM | `touch_id` has no classification row | REP 16 (migration 0057 list)
V-16-10-b | CONFIRMED LOW | 4.1 payload rows and 3.6 step 1 stale against block 7.9 | 3 REP 16
V-16-11-a | REJECTED | refuted by the refuter
V-16-11-b | CONFIRMED LOW | wrong pointer to section 3.16 | 2 REP 16
V-16-12-a | CONFIRMED LOW | phase 17 did not mention the change phase 16 block 7.11 assigns to it | BLOCK 17 (pointer)

## Phase 28
V-28-02-a | CONFIRMED LOW | `input_too_large` in no refusal group of the monitoring counts | REP 28 (group `model`)
V-28-02-b | CONFIRMED MEDIUM | the golden draw had no run input and the sample no run column (asked for in the phase 28 review, never written) | BLOCK 19, BLOCK 28 (DDL in migration 0089)
V-28-02-c | CONFIRMED MEDIUM | the time-boxed QA grant did not cover `assistant_draft` items | same BLOCK 19; pointer REP 28
V-28-03-a | CONFIRMED LOW | utoipa list named a DTO that does not exist | REP 28
V-28-04-a | CONFIRMED MEDIUM | no route returns the draft's provenance | BLOCK 28; NEW field `provenance`
V-28-05-a | CONFIRMED MEDIUM (refuter HIGH) | narrative accept took lock 4 before lock 3 | REP 28
V-28-07-a | CONFIRMED MEDIUM (refuter HIGH) | section 4.5 and Open doubts 24 contradicted 7.2 (DR-386) | 2 REP 28
V-28-07-b | CONFIRMED LOW | phase 23 class table lacked the `retirement` clock for R11 | REP 23 (lead decision A)
V-28-09-a | CONFIRMED LOW | no zone table row for `llama-eval` | REP 28
V-28-11-a | CONFIRMED LOW | canonical form not cited | REP 28
V-28-12-a | CONFIRMED MEDIUM (refuter HIGH) | the escape rule replaced `<` by itself | REP 28: JSON escape `<` (lead choice, not the draft's `&lt;`)

## Phase 06 (17 confirmed: 5 MEDIUM, 12 LOW; 4 rejected)
V-06-01-a | CONFIRMED MEDIUM | `CatalogueQuery` has no method for GET /cbi-catalogue | block 06: `CatalogueQuery::list`
V-06-01-b | CONFIRMED LOW | `principal.active` where the phase 03 User entity has `status` (also phases 09, 12) | 4 REPs (06 x2, 09, 12)
V-06-03-a | REJECTED | insert and completion values are named in section 3.5; ids are app-generated (phase 00)
V-06-03-b | CONFIRMED LOW | no required flags or value types for cbi-gap-report.v1.json | block 06
V-06-03-c | CONFIRMED LOW | `start` names no audit event | REP 06 step 1: a progress marker, no event (lifecycle events cover the run)
V-06-03-d | CONFIRMED MEDIUM | the cover's bank name has no readable source ("phase 01 settings" does not exist) | REP 06: read from `deploy/branding.json`, mounted read-only in akr-worker (lead decision: one source for the name, no new setting)
V-06-03-e | CONFIRMED LOW | `E_IDEMPOTENCY_REPLAY` is not a code | REP 06
V-06-04-a | CONFIRMED LOW | `Grading` has no field list | block 06 (NEW `ItemGrade`)
V-06-04-b | CONFIRMED MEDIUM | nothing enqueues `coverage_grade` on a change of `coverage.maintained_fields` | block 00: `SettingChangeHook` registry in `SettingsService::apply` (NEW; mirrors `FlagChangeHook`)
V-06-04-c | CONFIRMED LOW | reason state `missing` where the grader uses `absent` | 4 REPs (06 x2, 10 x2)
V-06-06-a | CONFIRMED LOW | "masked form" undefined | REP 06 (words removed)
V-06-06-b | CONFIRMED MEDIUM | `@deny("masked")` is used by 8 phases and phase 03 defines only `out_of_scope` and `tier` | block 03: the masked deny tag, `DenyReason::Masked` (NEW variant), other tags are Forbidden reasons. The drafter's change to `@deny("tier")` and the masked-kind registration were not taken (phase 16 says these are not masked kinds).
V-06-06-c | CONFIRMED LOW | `Keyset` and `Page<T>` defined nowhere | block 00
V-06-08-a | CONFIRMED LOW | `DeclarationService` has no method signature | block 06
V-06-08-b | REJECTED | registry bounds, not a floor (DR-34)
V-06-08-c | CONFIRMED LOW | section 6 expects `E_FORBIDDEN` for DS, the scope gate answers `E_SCOPE` | REP 06
V-06-08-d | CONFIRMED LOW | step 4 says the audit event keeps a previous-values hash; no such key | REP 06: the earlier declaration's event keeps the previous values (no new key)
V-06-09-a | REJECTED | exists; pointer "section 4.5" corrected to "section 4" by REP
V-06-11-a | CONFIRMED MEDIUM | the grader signature lacks the `MappingCompleteness` input of 7.6 | block 06 (from phase 10 on)
V-06-12-a | REJECTED | exists; pointer "API.md 3.6" corrected to "API.md 3.8" by REP
V-06-12-b | CONFIRMED LOW | counts still say 13 routes after 7.8 | 5 REPs in 06, the cumulative count in 07 (128 -> 129), and the "section 3.8 route table" pointer of 7.8

## Phase 07 (20 confirmed: 2 HIGH, 5 MEDIUM, 13 LOW; 1 rejected)
V-07-01-a | CONFIRMED HIGH | the profile reads list `report.prepare, config.author`; CHK holds neither, yet Cedar and section 6 let CHK read | 3 REPs 07 (both rows gain `config.approve`, the paragraph under the table); docs/API.md by hand (MANUAL.txt)
V-07-01-b | CONFIRMED LOW | the weights-and-bands and DOCTYPE refusals name no reason | 2 REPs 07: reuse `params_invalid` and `xsd_unparseable` (the drafter's NEW `quality_invalid` not taken)
V-07-01-c | CONFIRMED LOW | who fills which field of `NewProfileVersion` | REP 07
V-07-01-d | CONFIRMED LOW | freeze trigger omits `lookups`, `schema_summary` | REP 07 (trigger, with V-07-04-a)
V-07-02-a | REJECTED | the watch run table and its UPDATE grant exist
V-07-02-b | CONFIRMED LOW | the Cedar permit gives CHK `goaml_watch.read`, the action table does not (the access matrix would list CHK) | 4 REPs 07: split policy `goaml.watch_readers` (NEW policy id)
V-07-03-a | CONFIRMED MEDIUM | promote with no corpus result has no reason; a dead corpus run has no stated outcome | REP 07 `corpus_not_run` (NEW), reason list REP, block 07
V-07-03-b | CONFIRMED MEDIUM | phase 21 defines no FIU-answers corpus | block 07: the corpus grows by release only (lead decision, reported to the user)
V-07-04-a | CONFIRMED LOW | `open_questions` outside the freeze list | same trigger REP
V-07-05-a | CONFIRMED LOW | no JSON type for rule parameters | block 07
V-07-06-a | CONFIRMED LOW | no reason for the two size limits | 2 REPs 07: reuse `file_too_large` (the drafter's NEW `rejection_too_long` not taken)
V-07-07-a | CONFIRMED MEDIUM | no storage, field or route for cases a profile amendment adds | same block as V-07-03-b: an amendment adds no case file (section 3.6 already says the corpus is release data)
V-07-08-a | CONFIRMED HIGH | a retry of a `running` check run ended as done at step 1, and step 11 had no per-file record, so the run stayed `running` | 2 REPs 07: step 1, step 11 (NEW `GoamlCheckRepository::record_file`, one transaction per file)
V-07-08-b | CONFIRMED MEDIUM | CFG holds `goaml_check.use_candidate` in the table and Cedar; step 2 allows M only (CFG cannot run a check anyway) | 3 REPs 07
V-07-08-c | CONFIRMED LOW | `CheckSummary` has no field list | REP 07 step 12
V-07-08-d | CONFIRMED LOW | types of `n` and `stream` in `stage_transient` | REP 07 step 4
V-07-09-a | CONFIRMED LOW | `CheckOutcome` fields | REP 07 (7.1)
V-07-09-b | CONFIRMED LOW | `ProfileVersion` is not a type | REP 07 -> `GoamlProfile`
V-07-10-a | CONFIRMED LOW | NAME-01 cannot be raised on an upload | REP 07 (7.2 row)
V-07-12-a | CONFIRMED LOW | CTR-02 has no parameter for its domestic test | REP 07 (7.2 row, `domestic` [IQ])
V-07-12-b | CONFIRMED MEDIUM | BR-13 (warn) and CTR-01/02 (block) test the same shape | block 07: BR-13 raises nothing where a CTR rule already did

## Phase 08 (11 confirmed: 4 MEDIUM, 7 LOW; 3 rejected; 1 duplicate of a phase 04 finding)
V-08-01-a | CONFIRMED LOW | the sign body's comment flag is not stated | REP 04 (`DecideRequest`)
V-08-01-b | DUPLICATE (LOW) | inbox counts polled every 30 s here, 60 s in phases 04 and 17 | the REP in fixes/04.txt (same finding from phase 04) already sets 60 s; counted once, under phase 04
V-08-01-c | CONFIRMED LOW | `E_SELF_APPROVAL` grouped under 409 | REP 08
V-08-02-a | CONFIRMED LOW | whether a header sort is a server sort | REP 08: a header sort reorders loaded rows; no sort parameter (lead decision; API.md is cursor only)
V-08-02-b | REJECTED | refuted by the refuter
V-08-03-a | CONFIRMED LOW | the one-retry rule reaches verify | 2 REPs 08
V-08-05-a | CONFIRMED MEDIUM | phase 08 expects a refused resume; phase 05 answers 202 and keeps the source paused | 2 REPs 08
V-08-05-b | REJECTED | refuted by the refuter
V-08-06-a | CONFIRMED MEDIUM | `SessionView` lacks the heartbeat interval phase 08 reads | block 01 (NEW field `heartbeat_min_interval_s`)
V-08-07-a | CONFIRMED LOW | `locale` called a `PreferencesV1` field | 2 REPs 08
V-08-08-a | CONFIRMED LOW | code points of `scanControls` not written | block 08
V-08-08-b | CONFIRMED MEDIUM | UX-20 flagging has no owner | block 10: `bidi_control` and `zero_width` as `quality_flags` values set by the canonical writer (lead decision; no DDL, no register kind)
V-08-10-a | CONFIRMED LOW | the `board pack` rail entry has no `RailGroup` | block 08 (`reports`)
V-08-11-a | CONFIRMED MEDIUM | transliteration pairing undefined | block 08: the Latin `party_name` row of the same party and name type; nothing generated (lead decision)
V-08-11-b | REJECTED | refuted by the refuter

## Phase 09 (19 confirmed: 8 MEDIUM, 11 LOW; 4 rejected)
V-09-01-a | CONFIRMED LOW | where the Hijri variant name is stored | REP 09: inside `hijri_label`
V-09-01-b | CONFIRMED MEDIUM | no `errors[]` code for five calendar refusals | block 09: DSL codes `E_SCHEMA`, `E_DANGLING_REF`, `E_PARAM_OUT_OF_BOUNDS` reused (the drafter's 12 new codes not taken)
V-09-01-c | REJECTED | refuted by the refuter
V-09-01-d | CONFIRMED LOW | phases 00 and 04 still say `add_working` | block 09
V-09-01-e | CONFIRMED LOW | `CaptureRun` fields | block 09
V-09-02-a | REJECTED | refuted by the refuter
V-09-02-b | REJECTED | refuted by the refuter
V-09-03-a | CONFIRMED MEDIUM | the pack's queue routing has no shape and conflicts with phase 17's `queue_definition` | block 09: the pack names no queue (lead decision)
V-09-04-a | CONFIRMED MEDIUM | `window_limit_days` is `Result` here and `Option` in phase 03 7.6 | REP 09 -> `Option<u32>`. The drafter's block was not taken: "`NoMatrix` passes" in phase 03 is the stub adapter of phases 03 to 08, not a conflict with DR-83.
V-09-04-b | CONFIRMED LOW | codes for `max_days`, `max_priority`, `second_review` | same block as V-09-01-b
V-09-04-c | REJECTED | refuted by the refuter
V-09-05-a | CONFIRMED LOW | codes for six policy pack refusals | same block as V-09-01-b
V-09-05-b | CONFIRMED LOW | "phase 04 pattern" for `E_COST_GUARD` does not exist | REP 09 -> API.md section 6
V-09-05-c | CONFIRMED LOW | fields of `PolicyPackSummary`, `Validated`; which `ValidationIssue` | block 09
V-09-06-a | CONFIRMED MEDIUM | kind-level inventory items have no `object_id`, so the GV-01 gate cannot find them | block 09: four fixed UUIDv5 constants returned by `prepare`
V-09-07-a | CONFIRMED LOW | which pack version a save-time scan reads | REP 09: the active version (lead decision)
V-09-08-a | CONFIRMED MEDIUM | the seed names no manifest signature check | block 09 (reference release)
V-09-08-b | CONFIRMED MEDIUM | release file keys undefined | same block
V-09-08-c | CONFIRMED MEDIUM | later releases: new, changed, unchanged, removed rows | same block (a removed row is left and listed)
V-09-08-d | CONFIRMED LOW | owners of the seed and `verify-data` | same block
V-09-09-a | CONFIRMED LOW | a document that omits a fixed key | same block as V-09-01-b (`E_SCHEMA`)
V-09-11-a | CONFIRMED MEDIUM | 7.4 cites an error `add` does not return; no `ClockError` variant | REP 09 (NEW variants `CalendarMissing`, `OutOfRange`)
V-09-11-b | CONFIRMED LOW | earliest day and backward load window | same REP

## Phase 10 (18 confirmed: 2 HIGH, 13 MEDIUM, 3 LOW; 3 rejected)
V-10-01-a | CONFIRMED MEDIUM | compile-to-store has no compare-and-set on `revision` | block 10 (`store_compiled` gains `expected_revision`)
V-10-02-a | CONFIRMED MEDIUM | no step resolves a link's `message_id` and `txn_id`; `xref_linked` undefined; rule and confidence unnamed | block 10: both ends through `admin.xref`; a missing end is a `RowConflict` `parent_missing` (NEW kind), re-read later; `rule = 'declared'`, confidence 1 (lead decisions); `xref_linked` = xref rows inserted
V-10-02-b | CONFIRMED MEDIUM | a released `account_hold` had no outcome (it would stay open, and phase 20 reads it) | block 10: `end_hold` (NEW method) and `GRANT UPDATE (end_at, row_hash)` in migration 0030; REP 10 adds three tables to the fact list
V-10-03-a | CONFIRMED MEDIUM | class of a Verify-only change; refusal reasons of three operations | block 10: `minor`; 422 `operation_invalid` (NEW reason)
V-10-03-b | CONFIRMED MEDIUM | the code-map revert names no target event | same block: `code_map.entry_reverted` (NEW)
V-10-03-c | CONFIRMED LOW | section 3.4 heading cites DR-38 | REP 10 -> DR-10, DR-88
V-10-04-a | CONFIRMED LOW | no rule maps a CBS value to `cdd_level` | block 10: the column stays null in this plan (lead decision)
V-10-05-a | CONFIRMED MEDIUM | `MappingExecutor::map`, `MappedRow`, `Rejected`, `MappedBatch`, failure codes undefined | block 10 (NEW `SourceRow`, `RowFailure` with five codes; three are the reasons phase 11 already names in words)
V-10-05-b | REJECTED | phase 10 writes no quarantine row; phase 11 owns the table
V-10-06-a | CONFIRMED MEDIUM | the writer shares no lock with the party purge | block 10 and block 23: the writer takes the party lock before its tombstone read
V-10-07-a | CONFIRMED MEDIUM | result type of `map_payload` | same block as V-10-05-a
V-10-07-b | REJECTED | `apply` (per row) and `map` (per batch) are two functions
V-10-08-a | CONFIRMED MEDIUM | `light`, `folded`, `key` bound to no transform | block 10 (NEW `stage` argument of `FoldArabic`)
V-10-08-b | REJECTED | the crate has one creator (DR-371)
V-10-09-a | CONFIRMED HIGH | no write path for a payment status change: step 3 made it a `RowConflict`, yet the table, the grant and 7.12 assume an append | block 10: `append_status` (NEW method); history entries keep the row hash so a re-read is `unchanged`
V-10-09-b | CONFIRMED HIGH | the status CHECK of 7.12 rejects `unknown`, which the `dq_issue` gate writes: the batch would fail | REP 10: the CHECK gains `unknown`
V-10-09-c | CONFIRMED LOW | grant comment cites section 3.6 | REP 10
V-10-10-a | CONFIRMED MEDIUM | `CurrentRow`, `CanonRow` undefined | block 10 (row types)
V-10-10-b | CONFIRMED MEDIUM | two sources for the business date | 2 REPs 10 (7.13): the batch's `business_to` (lead decision)
V-10-11-a | CONFIRMED MEDIUM | `close_versions` cannot express the equal case | same block as V-10-10-a (`Option<NaiveDate>`)
V-10-12-a | CONFIRMED MEDIUM | two versions of one id in one batch collide on `(id, recorded_from)` | same block: a group keeps its newest row (lead decision)

## Phase 11 (34 confirmed: 6 HIGH, 14 MEDIUM, 14 LOW; 10 rejected)
Lead decisions: a declaration needs `ended_at` (no open declared period, no close route, no new grant); the replay estimate sums receipts (`rows_landed_in`), raw stays akr-ingestd's; replay receipts of a files table carry no file hash (CHECK of migration 0033 changed); extract files are RFC 4180 CSV with a header and the manifest names `columns`; `source.record` names the record or raw-message column; late-arrival events are written by the load unit on a new 90-day maximum, so no day is stored; a `cdc_position` table is due at every tick.
V-11-01-a | CONFIRMED LOW | which check id `source.halt_resolved` carries | REP 11: the fresh check of step 2
V-11-01-b | REJECTED | the authorize layer's `authz.allowed` row is the request record (phase 03 section 3.2)
V-11-01-c | CONFIRMED LOW | `Trigger::Resume` against phase 05's `CheckTrigger` | REP 11, and REP 05 for the same slip (`Trigger::Health`)
V-11-01-d | CONFIRMED LOW | base "no watermark moves" under 7.13's marker write | REP 11
V-11-02-a | CONFIRMED HIGH | no step creates a watermark row; the first `advance` matches nothing | block 11: `ensure_rows` (NEW method) before the unit; NULL reads as sequence 0 for manifests
V-11-02-b | CONFIRMED MEDIUM | planner signature lacks the inputs of 7.10 and 7.12; no read of the measured lag | REP 11 (signature) and the late-arrival block (`BatchRepository::late_stats`, NEW)
V-11-02-c | CONFIRMED LOW | `StaleChange`, `StaleTransition`, `WatermarkWithSource` undefined | block 11
V-11-03-a | CONFIRMED HIGH | akr-api route called `LandingStore::estimate`, but akr-api never reads `raw` | REP 11 and block 11: `BatchRepository::rows_landed_in` (NEW); `LandingStore::estimate` is the job's
V-11-03-b | CONFIRMED HIGH | a replay receipt of a files table fails the `file_sha256` CHECK | block 11: CHECK of migration 0033 becomes `(delivery = 'files' AND NOT (manifest ? 'replay'))`
V-11-03-c | CONFIRMED LOW | status of a replay page receipt | REP 11 (`mapped` or `partial` by section 3.2 step 6)
V-11-03-d | REJECTED | the replay types are described by the DDL comment and the steps (field lists)
V-11-03-e | REJECTED | run-record progress writes are not business mutations; the lifecycle events cover the replay
V-11-03-f | CONFIRMED MEDIUM | whether apply lands raw rows and what it reads | REP 11: step 5 only, over `read_window`
V-11-03-g | CONFIRMED LOW | failure path omits `finished_at` | REP 11
V-11-03-h | CONFIRMED LOW | open doubt 7 is closed by phase 04 section 7+ | REP 11
V-11-04-a | CONFIRMED HIGH | a declared period without `ended_at` could never close (and would cover every later date in phase 12) | REPs 11: `ended_at` is required; block 11 (declare steps)
V-11-04-b | CONFIRMED LOW | reasons of the declaration refusals | REP 11: `in_future` (NEW), `too_short`, `too_long`, `range_inverted`
V-11-04-c | CONFIRMED MEDIUM | `BlindPeriodService` has no method | block 11: `declare` (NEW method) with steps
V-11-04-d | CONFIRMED HIGH | `GET /coverage/blind-periods` scope shut out DS, who reads the register | REP 11: `dashboard.read, data.steward`; MANUAL API.md
V-11-04-e | REJECTED | phases 16 and 17 carry the blind period ids with the alert
V-11-05-a | CONFIRMED MEDIUM | `SpoolError` variants and the consumer entry unnamed | block 11: four variants, `read_unit` (NEW)
V-11-05-b | CONFIRMED MEDIUM | no first-unit rule or unit size for `cdc_position` | same block: first segment of the spool, snapshot `op` `r` as `c`, whole segments up to `ingest.max_unit_rows`
V-11-05-c | CONFIRMED MEDIUM | 60-minute idle rule against the 900-second F1 limit | same block: a `cdc_position` table is due at every tick
V-11-05-d | REJECTED | designed: section 4.4 of phase 11 changes the phase 05 refusal (DR-63)
V-11-05-e | CONFIRMED LOW | "a signed agreement" read as a check | REP 11: the audited flag change is the record; no step checks the paper
V-11-05-f | CONFIRMED LOW | gap halts would audit `source.restore_suspected` | REP 11: `source.state_changed` with cause `delivery_gap` only
V-11-06-a | CONFIRMED MEDIUM | values of `Quarantine.gate` | block 12: `txn_type`, `none`; card guard is known by its reason code
V-11-06-b | REJECTED | `QuarantineFilter` fields are in the route table and 7.3
V-11-07-a | CONFIRMED HIGH | the `landing_file` column list and the file format were pinned nowhere | block 11 (extract file contract) and two REPs 11
V-11-07-b | CONFIRMED MEDIUM | no DSL key for the record-text column | block 11: `source.record` (NEW key); MANUAL DSL.md
V-11-07-c | CONFIRMED MEDIUM | no row reader on `ExtractInbox` | contract block: `read_rows` (NEW)
V-11-07-d | CONFIRMED MEDIUM | a row that does not fit the columns had no outcome | contract block: batch `rejected`, reason `column_check_failed` (NEW)
V-11-08-a | CONFIRMED MEDIUM | no `from` grammar or parser signature for `iso20022` and `mt` | block 11 (path, tag, `sender`, `receiver`, `parse`)
V-11-08-b | CONFIRMED LOW | key of the parser version in the artefact | same block: `parser_version`
V-11-09-a | REJECTED | the gateway database sits in zone Z0 of ARCHITECTURE section 4
V-11-09-b | REJECTED | the tier is a registration input of every source
V-11-11-a | CONFIRMED MEDIUM | `business_from` of a source with no business date read | two REPs 11: `ingest.history_from`
V-11-11-b | REJECTED | designed: phase 11 section 4.4 adds the opens to the phase 05 transitions
V-11-12-a | CONFIRMED MEDIUM | the alert rule 7.12 cites is not in section 4.3 | block 11: gauge `akr_ingest_late_near_window` (NEW) and rule
V-11-12-b | CONFIRMED LOW | three settings without INSERT rows and labels | block 11
V-11-12-c | CONFIRMED MEDIUM | "once per table and day" had no storage; `ingest.lookback_widened` no writer | late-arrival block and REP 11 (no stored day; `manifest.lookback_days`)
V-11-12-d | CONFIRMED LOW | two kinds missing from the audit table | late-arrival block item 5
V-11-12-e | REJECTED | 7.12 item 3 uses a fixed lookback as written; "never narrower" is the rule of `auto`, which the planner keeps
V-11-12-f | CONFIRMED LOW | settings count and governed list predate 7.10 and 7.12 | four REPs 11

## Phase 12 (13 confirmed: 1 HIGH, 7 MEDIUM, 5 LOW; 3 rejected)
V-12-01-a | CONFIRMED MEDIUM | the issue upserts of a recon run had no audit event | REP 12: they run in the step 9 transaction and `recon.completed` (counts per rung) is their event; no new kind
V-12-01-b | REJECTED | claim and wait are the job's own status moves; the run has its named events
V-12-02-a | CONFIRMED MEDIUM | what proves "signed by the bank" | REP 12: document kind `exception_signed` (phase 05)
V-12-02-b | CONFIRMED LOW | `ChangeRequestService::submit` with create arguments | REP 12: `create(origin = Dedicated)`, which continues with `submit`
V-12-03-a | CONFIRMED LOW | the same for the exclusion proposal | REP 12
V-12-04-a | CONFIRMED MEDIUM | a card-guard stop wrote no quarantine row, so the refusal could not fire | covered by fixes/10 (`RowFailure` `card_guard`) and the gate block of fixes/11; REP 12 names the reason code
V-12-04-b | CONFIRMED LOW | the test expects a status `assigned` that no enum has | two REPs 12: assign sets the owner and keeps the status
V-12-05-a | REJECTED | the fields of `DataAsOf` and `BranchDate` are named where they are used
V-12-07-a | CONFIRMED LOW | restore path: the job is queued without the route checks; an unloaded date is `source_anomaly`, not a break | two REPs 12 (7.2)
V-12-08-a | REJECTED | phase 11 block 7.4 already replaces the stale backfill sentence
V-12-10-a | CONFIRMED HIGH | `akr_app` had no UPDATE on `dq_issues.detail`; the lead also found `count` and `last_seen` missing for the recon run's upsert | REP 12: the grant of migration 0037 gains `detail, count, last_seen`; comment and inventory row
V-12-10-b | CONFIRMED MEDIUM | no read returns the latest recon result of an object and group | block 12: `ReconRepository::latest_result` (NEW)
V-12-10-c | CONFIRMED MEDIUM | "latest" undefined; no-result case | same block: newest `finished_at`; None writes no `accepted_delta`
V-12-10-d | CONFIRMED LOW | `DqIssueRepository` for `DqRepository` | REP 12
V-12-11-a | CONFIRMED MEDIUM | the step 7 lookup key differs from the stored `<rung>:<object>` | three REPs 12
V-12-12-a | CONFIRMED MEDIUM | no read of the blind periods covering a date; `BlindFilter` undefined | block 12: `CoverageRepository::blind_periods_covering` (NEW); block 11: `BlindFilter` with `source_id?`

## Phase 13 (34 confirmed: 1 HIGH, 17 MEDIUM, 16 LOW; 4 rejected)
Lead decisions: VB-05 carried as counterparty-keyed features only (counterparty profiles P2; TELL THE USER); S-17 batch tags and government-source flag P2 (no CBS field; TELL THE USER); `suppressed_periods` dropped; cluster memberships keyed by currency (DDL change in migration 0040); one access event per customer record.
V-13-01-a | CONFIRMED MEDIUM | no write method for the segment activation row | block 13: `SegmentRepository::apply_version` (NEW)
V-13-01-b | CONFIRMED MEDIUM | propose refusals name no code | block 13: all three are `E_SCHEMA` with a pointer; no new code
V-13-02-a | CONFIRMED MEDIUM | S-17 tags and government-source flag carried nowhere | REP 13 header: P2, no CBS field known
V-13-02-b | CONFIRMED LOW | header claims a pool exclusion no step does | same REP: the exclusion is phase 16's `EXCL_POOL_ACCOUNT`
V-13-02-c | CONFIRMED LOW | header reads S-17 as an exclusion | same REP
V-13-03-a | CONFIRMED MEDIUM | cluster segment rows omit name, definition, currency, created_by | REP 13 step 13
V-13-03-b | CONFIRMED MEDIUM | no loader for risk-overlay segments or the segments of a build | block 13: `active_by_level` (NEW)
V-13-03-c | CONFIRMED LOW | `MembershipSnapshot` fields unnamed | same block; `Level`, `MembershipChange` named too
V-13-03-d | CONFIRMED HIGH | no steps for cluster membership; and (found by the lead) one open row per customer and level clashes with one live build per currency | block 13: `currency` in the membership key (migration 0040), `PeerRepository::get`, `insert_candidate`, four steps, activation at the last page, older candidate refused
V-13-04-a | CONFIRMED MEDIUM | eight seed expressions unwritten; `cash_share_1m` ambiguous | block 13: table of eight expressions; ratio holds `num` and `den` inline
V-13-04-b | CONFIRMED MEDIUM | no repository for `feat.segment_stats` | block 13: `SegmentStatsRepository` (NEW), `FeatureRunRepository` (NEW)
V-13-04-c | CONFIRMED MEDIUM | stats read has no steps; `suppressed_periods` has no source | same block: three steps; field dropped (four REPs)
V-13-04-d | CONFIRMED LOW | route row says "counted, never shown" | REP 13 route row
V-13-04-e | CONFIRMED LOW | 3.10 heading wider than the phase 02 rule | REP 13
V-13-05-a | CONFIRMED MEDIUM | header claims counterparty profiles; none exist | REP 13 header + block 13 (features only; profiles P2)
V-13-05-b | CONFIRMED LOW | no counterparty grouping rule | same block: grouping by `(t.counterparty_id, t.currency)`; no seed feature
V-13-05-c | CONFIRMED LOW | no counterparty profile steps | same header REP (not claimed any more)
V-13-05-d | CONFIRMED LOW | no counterparty profile trait or DTO | same header REP
V-13-05-e | REJECTED | conditional on a counterparty profile, which is P2
V-13-06-a | CONFIRMED MEDIUM | two protected-attribute lists, no rule | block 13: the DSL A.5 list in code governs; the phase 24 register does not widen it
V-13-07-a | CONFIRMED LOW | short-history case unstated | REP 13 step 5
V-13-07-b | CONFIRMED MEDIUM | range refusal has no code | REP 13: `too_long` at `/from`, `range_inverted`
V-13-07-c | REJECTED | refuted
V-13-08-a | REJECTED | refuted
V-13-08-b | CONFIRMED LOW | VB-14 header wording (per-segment, closed days) | REP 13 header
V-13-08-c | CONFIRMED LOW | VB-14 is per customer; online half not deferred in the text | same REP
V-13-08-d | CONFIRMED LOW | "on closed days" against the month-end design | same REP
V-13-09-a | CONFIRMED MEDIUM | access event has no `resource_id` rule | two REPs 13: one event per customer record
V-13-09-b | CONFIRMED LOW | 7.1 cites a section 4.5 that does not exist | REP 13
V-13-10-a | CONFIRMED LOW | extension points have no forms | block 13: `ComparatorPlan`, `ComparatorResult` (NEW); compiler entry points are phase 16's
V-13-10-b | CONFIRMED LOW | phase 16 says "not defined here" | REP 16
V-13-11-a | CONFIRMED MEDIUM | `ReaderCatalogue` signatures differ; `CatalogueError`, `ObjectRef` defined nowhere | REP 13 (full signature, `ObjectRef` defined), two REPs 16 (`RepoError`)
V-13-11-b | CONFIRMED LOW | 7.3 does not name the 3.7 rule | REP 13
V-13-11-c | CONFIRMED MEDIUM | null `measure` returns `None` against phase 16's `Some(Neutral)` | REP 13
V-13-12-a | CONFIRMED MEDIUM | link retention clock `age` against "follows the counterparty"; phase 23 has no handler | REP 13 (`owner`), block 23 (parent key)
V-13-12-b | CONFIRMED MEDIUM | phase 18 writes the table; no method | REP 13 + block 13: `CounterpartyLinkRepository::replace_open` (NEW)
V-13-12-c | REJECTED | the function is identified by the column it fills; pointer to phase 10 section 7.9 added in passing
V-13-12-d | CONFIRMED MEDIUM | job and analyst route race on the open row | same block: `FOR UPDATE`, conflict-safe insert, re-read once

## Phase 14 (14 confirmed: 0 HIGH, 7 MEDIUM, 7 LOW; 1 rejected)
Lead decisions: a failed list version is the incident (no phase 20 row); pair memory on freeze-list entries is allowed (DR-102), header corrected; N-44 carried in part (corpus grows by release only; TELL THE USER); once-only alarm marks in a new `ref.lists.alarm_marks` column.
V-14-01-a | REJECTED | refuted (the bulk method, input and handler are named)
V-14-01-b | CONFIRMED MEDIUM | bulk decision on a freeze list names no code | REP 14: `freeze_list_excluded`
V-14-01-c | CONFIRMED LOW | no order between the Cedar forbid and the service 422 | REP 14: service check answers; the forbid never fires on a System resource
V-14-01-d | CONFIRMED LOW | `StepUpAction::Decision` undeclared | block 14: name = Cedar action, class = SU class of the route
V-14-02-a | CONFIRMED MEDIUM | count mismatch has no outcome | block 14: fails with `count_mismatch`
V-14-02-b | CONFIRMED MEDIUM | the "incident" has no kind and no alarm | same block: `list.failed` is the incident; counter `akr_list_version_failed_total` (NEW) and alert
V-14-03-a | CONFIRMED LOW | eight types have no fields | block 14; `ScoreComponents` renamed `MatchComponents` (clash with phase 17), REP 14
V-14-04-a | CONFIRMED LOW | header says pair memory never on freeze lists, against DR-102, 3.7 and 7.3 | REP 14 header
V-14-04-b | CONFIRMED MEDIUM | no route for the good-guy revoke | REP 14: same proposal route, second body form
V-14-05-a | CONFIRMED MEDIUM | no sink for labelled pairs; no MLRO corpus acceptance | REP 14: labels are the stored decisions; corpus by release only; bank-side intake P2
V-14-05-b | CONFIRMED LOW | `setting.changed` for phase 00's `setting.updated` | two REPs 14
V-14-07-a | CONFIRMED LOW | `delisted_on` has no source field | REP 14: no format carries one; NULL, delisting by absence
V-14-08-a | CONFIRMED MEDIUM | sensitive route has no registry entry or fixture | block 14
V-14-09-a | CONFIRMED MEDIUM | "once per version" has no stored mark (nor `list.stale` once per transition) | block 14: `ref.lists.alarm_marks` (NEW column, migration 0045), REP 14 in 7.10
V-14-09-b | CONFIRMED LOW | no response carries the re-screen time | REP 14: `ListVersionView.rescreen_seconds`

## Phase 15 (17 confirmed: 2 HIGH, 10 MEDIUM, 5 LOW; 6 rejected)
Lead decisions: phase 15 registers its own `record_decision` effect (three resource kinds) and two workflow subjects; exit approval is one step role M; section 7.2 holds and phase 20 seeds `exit_decision` version 2; the registration-country arm waits for a canonical column.
V-15-01-a | CONFIRMED MEDIUM | no `ExitRepository`; `ExitQuery` undefined | block 15: `ExitRepository` (NEW), `ExitService::record`, `list`; REP 15 (`Arc<ExitService>`)
V-15-01-b | CONFIRMED MEDIUM | `review_id` of an exit entry has no source | block 15 (subjects): frozen in the instance context
V-15-01-c | REJECTED | refuted
V-15-01-d | CONFIRMED HIGH | no `record_decision` effect serves `change_request`, `kyc_review` or `exit_decision`; the three shipped definitions cannot publish | block 15: `KycRecordDecisionEffect` (NEW); block 04: effects resolve by kind and resource kind
V-15-02-a | CONFIRMED MEDIUM | fan-out names a registration country no column holds | REP 15: arm dropped until the column exists (Open doubts 10)
V-15-02-b | CONFIRMED MEDIUM | no method for the fan-out page query | block 15: `PartyFactsSource::parties_for_country` (NEW)
V-15-02-c | CONFIRMED MEDIUM | `effective_at` has no source | same block: `created_at` of the outbox row
V-15-02-d | CONFIRMED LOW | `country_table.changed` not tied to the topic | same block: the topic is the TR-10 event; action `rescore`
V-15-03-a | REJECTED | refuted
V-15-04-a | REJECTED | refuted
V-15-04-b | REJECTED | refuted
V-15-04-c | CONFIRMED MEDIUM | PEP candidate display name has no column rule | REP 15: `raw` of the `primary` rows by script; missing script takes the other
V-15-06-a | CONFIRMED HIGH | no `WorkflowSubject` for `kyc_review` or `exit_decision` | block 15: two subjects (NEW)
V-15-06-b | CONFIRMED MEDIUM | phase 20 ships no `exit_decision` version 2 | block 20
V-15-06-c | CONFIRMED MEDIUM | exit approval names no decision type or role | REP 15: one step role M, `kyc.decision`, not the reviewer
V-15-06-d | CONFIRMED LOW | section 4.2 base text against 7.2 | REP 15 (7.2 holds)
V-15-07-a | CONFIRMED LOW | coverage response has no type | block 15: `OwnershipCoverageView` (NEW)
V-15-08-a | CONFIRMED LOW | analytics response has no type; `from`, `to` not in the route | same block: `OverrideAnalyticsView` (NEW)
V-15-08-b | REJECTED | refuted
V-15-09-a | CONFIRMED MEDIUM | no read of the previous mapped batch's `business_to` | block 11: `BatchRepository::previous_mapped_business_to` (NEW)
V-15-09-b | CONFIRMED LOW | `BusinessClock::add` call omits the unit | block 15
V-15-10-a | REJECTED | refuted
V-15-11-a | CONFIRMED MEDIUM | the absorb statement never moves `due_at` | REP 15 (SET list) + block 15; no new audit kind

## Phase 17 (21 confirmed: 1 HIGH, 15 MEDIUM, 5 LOW; 0 rejected)

V-17-01-a | CONFIRMED HIGH | the retry job writes `reason_detail` and the UPDATE grant on `alert.quarantine` did not name it | REP 17 (migration 0061 grant gains `reason_detail`)
V-17-01-b | CONFIRMED MEDIUM | no repository trait and no service signatures for the alert quarantine; the service and AppState field also clashed with phase 11's `QuarantineService` and `quarantine` | BLOCK 17 (`AlertQuarantineRepository`, `AlertQuarantineService`, field `alert_quarantine`); six REPs 17 and one REP 16 rename
V-17-01-c | CONFIRMED LOW | DTO field `reason_code` while reject needs `resolution_code` | REP 17
V-17-01-d | CONFIRMED MEDIUM | reject had no row lock and no open-status check | BLOCK 17 (lock, 409 `E_NOT_ALLOWED_IN_STATE`, `If-Match`)
V-17-01-e | CONFIRMED MEDIUM | the retry job had no row lock or re-check | same BLOCK 17
V-17-01-f | CONFIRMED MEDIUM | release step 2 wrote `revalidated_at` and an audit event before a 422 that phase 00 7.17 rolls back | REP 17: a refused release writes nothing; the phase 03 decision row records the attempt (lead decision)
V-17-02-a | CONFIRMED MEDIUM | no append method or input type for alert events | BLOCK 17 (`AlertRepository::append_event`, `NewAlertEvent`)
V-17-02-b | CONFIRMED MEDIUM | `alert_events.seq` had no allocation rule | same BLOCK 17: `max(seq) + 1` under the alert lock, no column (lead decision)
V-17-02-c | CONFIRMED MEDIUM | `NewCaseEvent` and `CaseEventKind` defined nowhere | BLOCK 17: `CaseEventKind(&'static str)` checked by the table CHECK, not the phase 02 catalogue type (lead decision; the draft's catalogue option not taken)
V-17-03-a | CONFIRMED MEDIUM | no method signatures for assignment | BLOCK 17 (`assign_alert`, `assign_case`)
V-17-03-b | CONFIRMED MEDIUM | assign named no accepted alert states | same BLOCK 17 (`queued` or `claimed`; else 409)
V-17-03-c | CONFIRMED MEDIUM | "the queue's pool" had no queue on the assign routes | same BLOCK 17: pool of an active queue whose filter admits the item, or the case's `assignee_pool_id` (lead decision; no `queue_id` in the request)
V-17-03-d | CONFIRMED MEDIUM | route row and DTO accepted a pool target for an alert; step 3 says user only | two REPs 17 (down-rated from HIGH: wording against a clear rule)
V-17-04-a | CONFIRMED MEDIUM | no service method for bulk | BLOCK 17 (`QueueService::bulk`)
V-17-04-b | CONFIRMED MEDIUM | bulk items had no check order and no conflict check | same BLOCK 17 (seven ordered checks; NEW reasons `not_allowed_in_state`, `conflict_refused`; down-rated from HIGH)
V-17-04-c | CONFIRMED LOW | 7.14 list left out 7.11's `target_is_duplicate` | same BLOCK 17 (one closed list)
V-17-05-a | CONFIRMED LOW | no signature for the snapshot service | BLOCK 17 (`SnapshotService::seal`)
V-17-05-b | CONFIRMED MEDIUM | `TxRow` undefined | same BLOCK 17 (`TxRow { id, booking_date, fields }`); REP 17 `tx_rows` gains `fields`
V-17-06-a | CONFIRMED LOW | `touch_hours` is not a defined name | REP 17 (`casework.touch_by_hours[class]`)
V-17-07-a | CONFIRMED LOW | not said that assignment writes `case_acl` rows without `grant()`'s gate | BLOCK 17 (direct write under `case.assign`; chain and audit events of grant and revoke)
V-17-08-a | CONFIRMED MEDIUM | the keyset cursor did not carry the sort keys or the null placement | BLOCK 17 (keys per order, nulls last, a cursor of another order is 422)

## Phase 18 (19 confirmed: 0 HIGH, 5 MEDIUM, 14 LOW; 4 rejected)

V-18-01-a | CONFIRMED LOW | step d(ii) named no sub-code for its 409 | BLOCK 18 (NEW sub-code `no_running_proposal` joins the closed list)
V-18-02-a | CONFIRMED LOW | no named writer for the alert trail row of step i | BLOCK 18 (`AlertRepository::lock`, `update`, `append_event` of phase 17; the method itself is V-17-02-a)
V-18-03-a | CONFIRMED MEDIUM | no steps for `POST /cases/{id}/opened-views`; an evidence key was unchecked | BLOCK 18 (three steps) and REP 18 route row: the route records panels only (lead decision; evidence rows come from the content read); MANUAL line for API.md
V-18-04-a | CONFIRMED LOW | `CaseChain::append_event` named where phase 17 defines `CaseRepository::append_event` | BLOCK 18
V-18-04-b | CONFIRMED LOW | `set_closure` call differed from its signature | REP 18
V-18-05-a | CONFIRMED MEDIUM | the reject comment passed no generic-text check (UX-65) | REP 18 (decision-text guard, `note_required`); in passing REP 18 on `NoteService::post` step 4 (`conn`, the minimum of a manual-grant note)
V-18-05-b | CONFIRMED LOW | "7.12" meant DESIGN-SYSTEM 7.12 | REP 18
V-18-06-a | CONFIRMED LOW | `report_sign_required`, `filing_not_acknowledged` not in the closed list | same BLOCK 18 as V-18-01-a
V-18-06-b | CONFIRMED LOW | step e stale under 7.3 (sign needs `via = "report"`) | REP 18 (down-rated from HIGH: 7.3 already wins)
V-18-07-a | REJECTED | refuted by the refuter
V-18-07-b | CONFIRMED MEDIUM | timer kind `case_acl_expiry` not in the phase 00 registry | REP 00 (table row); BLOCK 18 (section 4.2)
V-18-07-c | CONFIRMED MEDIUM | no job rows and no worker handler for the grant expiry | BLOCK 18 (`timer_case_acl_expiry` 2, 3; `case_acl_expiry_scan` 1, 3; NEW `CaseAclExpiryHandler`)
V-18-07-d | CONFIRMED LOW | section pointer 3.9 for 3.10 | REP 18
V-18-07-e | REJECTED | refuted by the refuter
V-18-07-f | CONFIRMED LOW | `CaseAccessView` for `AccessView` | REP 18
V-18-08-a | REJECTED | refuted by the refuter
V-18-08-b | CONFIRMED MEDIUM | the new counterparty link row had no `valid_from` | REP 18: the decide route writes through phase 13 `CounterpartyLinkRepository::replace_open` (fixes/13)
V-18-09-a | CONFIRMED LOW | search response type and row fields unnamed | BLOCK 18 (`SearchResponse`, `SearchGroup`, `PartyHit`, `CaseHit`)
V-18-09-b | CONFIRMED LOW | over-cap `limit` undefined | REP 18 (422 `E_VALIDATION` at `/limit`; lead decision)
V-18-10-a | CONFIRMED LOW | `set_reopened(case)` for `(tx, id, at)` | REP 18
V-18-11-a | CONFIRMED LOW | sub-code `decided` for `already_decided` | REP 18 and REP 17 (7.13)
V-18-12-a | REJECTED | refuted by the refuter
V-18-12-b | CONFIRMED LOW | the deny event of a refused link add named no action | BLOCK 18 (`view`, `deny`, `customer`, owner party)

## Phase 19 (20 confirmed: 1 HIGH, 10 MEDIUM, 9 LOW; 1 rejected; 1 duplicate)

V-19-01-a | CONFIRMED LOW | `GET /watches` named `watch.read`; the list action is `watch.list` | two REPs 19
V-19-01-b | CONFIRMED LOW | no service method for the watch review | REP 19 (`WatchService::review`)
V-19-01-c | CONFIRMED MEDIUM | no read turned a `canon.touched` batch into party ids | BLOCK 19 (`CanonNetworkReader::batch_parties`)
V-19-02-a | CONFIRMED MEDIUM | no by-id finding read; the bulk-reopen dialog had no list source | BLOCK 19 (`QaFindingRepository::get`, `QaSampleRepository::closed_by`, route `GET /qa/findings/{id}/reopen-candidates` under `qa.bulk_reopen`); MANUAL line for API.md (lead decision)
V-19-02-b | CONFIRMED MEDIUM | "the window of the finding" undefined | same BLOCK 19: the 30 days before the review, as `same_closer_30d` (lead decision; no setting, no column)
V-19-02-c | CONFIRMED LOW | a non-critical finding had no refusal | same BLOCK 19 (422, NEW sub-code `not_critical_finding`)
V-19-03-a | CONFIRMED LOW | `GET /rfis` named `rfi.read`; the list action is `rfi.list` | REP 19
V-19-03-b | CONFIRMED MEDIUM | M-44 pre-fill listed as implemented with no step | REP 19 header and BLOCK 19: the pre-fill is P2, the switch stays off and unread (lead decision, USER)
V-19-04-a | CONFIRMED MEDIUM | the QA grant revocation named no method | BLOCK 19 (`QaService::revoke_review_access` over `CaseAccessRepository::revoke`)
V-19-05-a | CONFIRMED HIGH | IA and SEC had no `alarm.list` permit, so `GET /alarms` refused them against section 3.10 and check (p) | BLOCK 19 (permits `followup.ia.alarms`, `followup.sec.alarms`); REP 19 route row action `alarm.list`
V-19-05-b | CONFIRMED MEDIUM | no by-id alarm read under a lock | REP 19 (`AlarmRepository::get_for_update`)
V-19-05-c | REJECTED | refuted by the refuter
V-19-07-a | CONFIRMED LOW | Rust enums lacked the three assistant values | BLOCK 19
V-19-07-b | CONFIRMED MEDIUM | "the weekly draw" named no job | same BLOCK 19 (`qa_suppression_sample` also draws `assistant_assisted`)
V-19-07-c | CONFIRMED LOW | the item view lacked the cited records (the grant part is in fixes/28) | same BLOCK 19 (`AssistantItem`)
V-19-07-d | CONFIRMED MEDIUM | `QaItemView` said no text; 7.2 shows the draft output | same BLOCK 19 and REP 19 (down-rated from HIGH)
V-19-08-a | CONFIRMED MEDIUM | phase 14 samples every freeze-list suppression row; phase 19 said 5 per cent only | REP 19 in 7.7 (phase 14 rule holds; down-rated from HIGH)
V-19-09-a | CONFIRMED MEDIUM | no read for `prior_failures` | REP 19 (`RfiRepository::count_missed_outreach`)
V-19-09-b | CONFIRMED LOW | section 4.2 table lacked `outreach.failed` | REP 19 (row)
V-19-10-a | CONFIRMED LOW | `rfi_open` missing from the closed list | REP 19
V-19-10-b | CONFIRMED LOW | section 4.5 item 4 omitted the merge refusal | REP 19
V-19-11-a | DUPLICATE | the same missing method as V-19-05-b

## Phase 20 (26 confirmed: 1 HIGH, 12 MEDIUM, 13 LOW; 3 rejected; 1 duplicate)

V-20-01-a | CONFIRMED LOW | `NewOpsTask` had no field list | BLOCK 20 (input rows)
V-20-01-b | CONFIRMED LOW | `RefrainService::on_task_completed` undefined; "record" named no column | BLOCK 20 (hooks write nothing; signatures)
V-20-01-c | CONFIRMED LOW | cannot-complete named no refusal for another user's claim | REP 20 (409 `E_CLAIMED`)
V-20-01-d | CONFIRMED MEDIUM | no clock set `due_at` of the exit task and the refrain lift | BLOCK 20: pack clock `ops.task.due` through `BusinessClock::add` (lead decision)
V-20-01-e | REJECTED | refuted by the refuter
V-20-02-a | CONFIRMED MEDIUM | `FreezeError` defined nowhere | REP 20 (`pub type FreezeError = OpsError;`)
V-20-02-b | CONFIRMED MEDIUM | `listing_decided_by` had no value in Start | BLOCK 20 (input rows: `c.decided_by`)
V-20-02-c | CONFIRMED LOW | `NewFreeze` not tied to columns | same BLOCK 20
V-20-03-a | CONFIRMED MEDIUM | bounds 1 to 8 of `ops.freeze_task_hours` not in the settings registry | BLOCK 20
V-20-04-a | CONFIRMED MEDIUM | section 6 step had `a2@` register a refrain; Cedar gives it to M only | REP 20: the test uses `m@` and `a2@` gets 403 (Cedar is the rule; down-rated from HIGH)
V-20-04-b | CONFIRMED MEDIUM | close on a closed refrain had no refusal | BLOCK 20 (409, `already_closed` reused)
V-20-04-c | CONFIRMED LOW | refrain methods had no signatures | BLOCK 20 (signatures)
V-20-04-d | CONFIRMED LOW | `NewRefrain` not tied to `RegisterRefrain` | BLOCK 20 (input rows)
V-20-06-a | CONFIRMED MEDIUM | freeze referral had no state rule; a second referral overwrote the letter id | BLOCK 20 (states; NEW sub-code `already_referred`)
V-20-06-b | CONFIRMED LOW | no method for the referral routes | BLOCK 20 (`refer`)
V-20-07-a | CONFIRMED MEDIUM | `freeze.reason_code` said frozen but no attribute and no reader | REP 20: not carried in the instance; the effect writes the constant (down-rated from HIGH)
V-20-07-b | CONFIRMED MEDIUM | approving the release of a freeze with no task failed on the lift task | BLOCK 20: no lift task for a freeze with no task (lead decision)
V-20-07-c | CONFIRMED LOW | no sub-code for another active decision code | same BLOCK 20 (`reason_family_mismatch` reused; the draft's NEW code not taken)
V-20-07-d | CONFIRMED LOW | `propose_release` had no signature | BLOCK 20 (signatures)
V-20-07-e | REJECTED | refuted; the section pointer is corrected in passing (REP 20: phase 18 section 3.8 item d)
V-20-07-f | CONFIRMED MEDIUM | 7.1 created release tasks at proposal; step 5 creates them on approval | REP 20 (approval holds)
V-20-08-a | CONFIRMED MEDIUM | the check read without locks against section 3.13 | REP 20 (owner, then task, `FOR UPDATE`)
V-20-08-b | CONFIRMED MEDIUM | `n` undefined; `holds_matched` counted holds | REP 20 (count per account; `n` = `accounts_checked`)
V-20-08-c | CONFIRMED LOW | `NewVerification` not tied to step 7 | BLOCK 20 (input rows)
V-20-08-d | REJECTED | refuted by the refuter
V-20-09-a | CONFIRMED LOW | the Confirm condition could not be read from the neutral view | REP 20 (the server's `copy_matches` decides)
V-20-09-b | CONFIRMED LOW | the day window of completed tasks was in no rule | BLOCK 20: the server lists completed tasks of the current bank day in the neutral view (lead decision; no new filter)
V-20-10-a | DUPLICATE | the same contradiction as V-20-07-f
V-20-10-b | CONFIRMED LOW | group order unstated | REP 20
V-20-12-a | CONFIRMED HIGH | the 7.3 deadline handler could not learn letter delivery; phase 21 named a phase 20 method that did not exist | BLOCK 20 (`LetterRequester::delivered`; no column); three REPs 21 (lead decision)

## Phase 21 (24 confirmed: 3 HIGH, 13 MEDIUM, 8 LOW; 1 rejected; 2 duplicates)

V-21-01-a | CONFIRMED MEDIUM | no step wrote `rpt.report_links` | BLOCK 21: written at submit from the frozen `earlier_reports`, matched by goAML reference (`FilingRepository::by_reference`, new); down-rated from HIGH (table, method and audit row existed)
V-21-01-b | CONFIRMED LOW | `ReportLink` and `NewReportLink` fields not named | BLOCK 21 (row types)
V-21-02-a | CONFIRMED LOW | `NewReportVersion` and `SealedContent` fields not named | BLOCK 21 (row types)
V-21-03-a | CONFIRMED HIGH | `reports_guard` refused the cleared signature of every revise | REP 21: the guard lets `rejected` or `returned` to `draft` clear it
V-21-03-b | CONFIRMED MEDIUM | the approval was not bound to a hash (RP-34) | BLOCK 21: bound to the frozen `content_sha256`, carried in `report.signed` (the manifest is built after the sign)
V-21-03-c | CONFIRMED MEDIUM | the state list lacked the CTR `validated` to `packaged` edge and `filed` to `returned` | two REPs 21 (down-rated from HIGH: sections 3.12 and 3.14 state both)
V-21-04-a | CONFIRMED MEDIUM | the acknowledgement locked the filing before the report | REP 21: report, then filing
V-21-04-c | CONFIRMED LOW | the receipt had no retention class | REP 21: R6
V-21-05-a | CONFIRMED HIGH | the build wrote `generated_by` with no UPDATE grant | REP 21: the build does not write it
V-21-05-b | CONFIRMED MEDIUM | `generated_by` was never set from a user (the job has no actor) | REP 21: set at the insert of step 1 (down-rated from HIGH: one defect with V-21-05-a)
V-21-05-c | CONFIRMED MEDIUM | two generates at once had no order | BLOCK 21: advisory lock per kind and period
V-21-05-d | CONFIRMED MEDIUM | the supersede of a draft pack had no audit event | BLOCK 21: `periodic.superseded` (new)
V-21-05-e | CONFIRMED MEDIUM | `PeriodicReportService` methods not named | BLOCK 21: `generate`, `sign`, `build`; `NewPeriodic`, `PeriodicFilter` in the row types block
V-21-06-a | CONFIRMED LOW | no activity age for a report with no transaction | REP 21: NULL, left out of the share
V-21-06-b | CONFIRMED LOW | `NewFiling` fields not named | BLOCK 21 (row types)
V-21-07-a | CONFIRMED MEDIUM | CTR-03 had no bounds | REP 07 and BLOCK 21: rule parameters `max_usd` 1000000 and `round_unit_iqd` 250 (lead decision; the draft left it open)
V-21-07-b | CONFIRMED MEDIUM | the account rule used the code of phase 07's chronological rule | REP 07 and five REPs 21: `CONS-05` (new, warn)
V-21-08-a | CONFIRMED HIGH | a PATCH during a validation run left changed content marked validated | REP 21: step 9 locks and re-checks the content hash
V-21-08-c | CONFIRMED MEDIUM | a draft edit had no audit event | BLOCK 21: `report.edited` (new; field keys only)
V-21-08-e | CONFIRMED LOW | PATCH said "a new version" against the in-place `set_content` | REP 21
V-21-10-a | CONFIRMED MEDIUM | drafter, version authors and submitter were excluded by no mechanism | BLOCK 21: both subject checks refuse with `E_SELF_APPROVAL`
V-21-10-b | CONFIRMED LOW | `ReportVersionSummary` not defined | BLOCK 21 (row types)
V-21-12-a | DUPLICATE | of V-20-12-a (fixes/20.txt sends four REPs to phase 21)
V-21-12-b | CONFIRMED MEDIUM | the foreign keys phase 20 deferred were never added | BLOCK 21: four NOT VALID foreign keys in 0070; the backfill checks the count
V-21-12-d | REJECTED | no freeze write exists once the handler reads the letter state, so no freeze event is owed
V-21-12-e | CONFIRMED LOW | section 3.15 named "section 4.5 item 10" and a freeze method | REP 21: the sentence now names `LetterRequester::delivered`
V-21-12-f | DUPLICATE | of V-20-12-a (Open doubts 4 is settled by the REPs of fixes/20.txt)

## Phase 22 (21 confirmed: 0 HIGH, 8 MEDIUM, 13 LOW; 0 rejected)

V-22-02-a | CONFIRMED LOW | "scanned" is not an `av_status` value | two REPs 22: `clean`
V-22-02-b | CONFIRMED LOW | `lang: none` is not a phase 02 value | REP 22: `und`
V-22-02-c | CONFIRMED MEDIUM | a branch correction named `corroborates`, which writes no claim | two REPs 22: `supersedes`, limited to a branch officer's current claim (down-rated from HIGH: one wrong name)
V-22-02-d | CONFIRMED LOW | the `question_key` to `question_id` lookup was not stated | REP 22 (the `AnswerInput` line)
V-22-03-a | CONFIRMED LOW | `ReceiveCommand` is not a phase 19 name | REP 22: `CreateInternalReport`
V-22-03-b | CONFIRMED LOW | "register detail" names no route; Open doubts 9 said "not built here" against 7.2 | two REPs 22
V-22-04-a | CONFIRMED MEDIUM | no `liaison_report_overdue` row for the phase 19 alarm table | BLOCK 22
V-22-04-b | CONFIRMED MEDIUM | a return after the timer fired armed no alarm | REP 22: the return raises it at once (lead decision)
V-22-05-a | CONFIRMED MEDIUM | no read of one stored template version | REP 22: `get_version`
V-22-05-b | CONFIRMED LOW | template checks had no codes | two REPs 22: `E_MISSING_I18N`, `unknown_key`, `forbidden_term`
V-22-05-c | CONFIRMED LOW | `desk.approval_waiting` seeded with no producer | BLOCK 22: kept as a code the `notify` effect may name (lead decision)
V-22-05-d | CONFIRMED LOW | `serve.public.base_url` is a Kratos key, not a setting | two REPs 22: `AKR_PUBLIC_ORIGIN`
V-22-06-a | CONFIRMED MEDIUM | the `notify.request` row allowed two codes; two producers send others | REP 22 and BLOCK 22: three payload shapes (down-rated from HIGH)
V-22-06-b | CONFIRMED MEDIUM | a count-only notice has no source row for steps 2, 3 and 6 | same BLOCK 22 (down-rated from HIGH)
V-22-07-a | CONFIRMED LOW | section 2 gave `ObservationView` to the desk; 7.2 says M only | REP 22 (stale base under 7.2; down-rated from HIGH)
V-22-07-b | CONFIRMED LOW | "phase 02 sealing service" is not a phase 02 name | same REP 22: the personal seal
V-22-07-c | CONFIRMED LOW | no observation read for the register rows | BLOCK 22: `by_internal_report`
V-22-10-a | CONFIRMED MEDIUM | no step moved an RFI to `closed` | BLOCK 19: an RFI closes with its case; `rfi.closed` (new; lead decision; down-rated from HIGH)
V-22-10-b | CONFIRMED LOW | 7.6 spoke of a withdrawn request the list never shows | three REPs 22
V-22-11-a | CONFIRMED LOW | 7.7 cited section 3.14 for section 3.13 | REP 22
V-22-12-a | CONFIRMED MEDIUM | no locked read of a notice by id | REP 22: `get_for_update`

## Phase 23 (20 confirmed: 2 HIGH, 11 MEDIUM, 7 LOW; 1 rejected; 2 duplicates)

V-23-01-a | CONFIRMED HIGH | ADM lacked the scope for the run read that check 5a makes as `adm@`; Cedar permits ADM | eight REPs 23: the three reads take `audit.read` or `admin.system`; `held` is null for ADM too; MANUAL line for API.md
V-23-01-c | CONFIRMED LOW | `RunSummary` had no definition | REP 23
V-23-01-d | CONFIRMED MEDIUM | the failed-item audit event had no name | BLOCK 23: `retention.item_failed` (new)
V-23-01-e | CONFIRMED MEDIUM | the dry-run request wrote no audit event | BLOCK 23: `retention.dry_run_requested` (new)
V-23-02-a | CONFIRMED MEDIUM | `HoldView` clashed with phase 29's schema of the same name | two REPs 23: `LegalHoldView`
V-23-02-c | CONFIRMED LOW | the 500-character limit had no step | REP 23
V-23-02-d | CONFIRMED LOW | the DTO cited section 3.2 for the scan of section 3.8 | REP 23
V-23-03-a | CONFIRMED MEDIUM | `ShredOutcome` values and the repeat shred were not stated | BLOCK 23: two values; a repeat is a no-op (lead decision)
V-23-03-b | DUPLICATE | already fixed by fixes/02.txt (the same pointer in phase 02 section 7.3)
V-23-04-a | CONFIRMED MEDIUM | `facility` and `trade_instrument` have no `account_id` | REP 23: through `customer_id`
V-23-04-c | CONFIRMED LOW | `RelationshipEndResolver` had no signature | BLOCK 23
V-23-04-d | CONFIRMED LOW | Open doubts 22 said phase 10 has no index on `account_id` | REP 23: `idx_txn_account`
V-23-05-a | CONFIRMED MEDIUM | a kept R7 screening alert could name a purged party | two REPs 23: `alert.screening_alerts` joins the blocking list
V-23-06-a | CONFIRMED MEDIUM | `feat.feature_runs` is not partitioned but was listed for partition drop | three REPs 23, REP 13, BLOCK 23: purged by row on `as_of` (lead decision)
V-23-06-b | CONFIRMED MEDIUM | the WORM bucket was not named | two REPs 23: `akr-evidence` under `certificates/` (lead decision)
V-23-07-a | DUPLICATE | of V-23-01-a (the same scope rule on the other two reads)
V-23-07-c | CONFIRMED MEDIUM | the owner of the later-of count was stated two ways | REP 23: the engine computes, the report service reads
V-23-08-a | CONFIRMED HIGH | upper-case class keys fail the phase 00 settings CHECK; migration 0072 would abort | eleven REPs 23 and REP 14: lower case; MANUAL line for DR-377
V-23-09-a | CONFIRMED MEDIUM | `akr-ctl ops provision` is not a command and no step created the transit keys | REP 23 and BLOCK 00: the ceremony runbook and `make bao-dev` (lead decision)
V-23-10-a | CONFIRMED MEDIUM | `assess.decisions` was not in the drop function's allow-list | REP 23 and BLOCK 23: listed from migration 0072 on
V-23-10-b | CONFIRMED LOW | the R7 sentence had no pointer to the exception | same BLOCK 23
V-23-12-a | CONFIRMED LOW | the purge hold check did not name the `merged_into` path of 7.4 | REP 23 (stale base under 7.4; down-rated from HIGH)
V-23-12-b | REJECTED | the merge test is stated in 7.4; a 7.x test line extends section 6

## Phase 24 (33 confirmed: 1 HIGH, 17 MEDIUM, 15 LOW; 1 rejected)
V-24-01-a | CONFIRMED MEDIUM | `case_id` filter has no column on `access_event` | REP: `case_id` reads `audit_log` alone; case reads are found by `resource_type = case` and `resource_id`
V-24-01-b | REJECTED | refuted by the refuter (the search input fields are stated in steps 2 to 6)
V-24-02-a | CONFIRMED LOW | no sub-codes for the title, severity, subject key and owner refusals | REP: seven sub-codes in the 3.13 Validation row
V-24-02-b | CONFIRMED MEDIUM | `sync_post_review` had no title, body or join | BLOCK: join through the `post_review` instance to `maker_id`; fixed bilingual titles; body `{subject_key, outcome, day}`
V-24-02-c | CONFIRMED LOW | no step-up constant for `accepted_risk` | BLOCK: `register.accept_risk` (approval)
V-24-03-a | CONFIRMED LOW | sweep job entry method unnamed | BLOCK: `GovernanceRegisterService::sweep_calendar(now)`
V-24-03-b | CONFIRMED LOW | red boundary differs between 3.4 and 7.3 | REP: red on and after `due_on` (7.3 rules); down-rated from MEDIUM
V-24-03-c | CONFIRMED LOW | `horizon_days` unused | REP: parameter dropped (lead decision)
V-24-04-a | CONFIRMED LOW | board payload builder unnamed | BLOCK: `BoardSource::payload(period)`
V-24-04-b | CONFIRMED MEDIUM | tile list and predicate of the board pack unstated | BLOCK: the ten tiles M, CH and IA all see, under the predicate of a CH-only principal; down-rated from HIGH (the phase 21 board row already gives CH and IA the report counts)
V-24-05-a | CONFIRMED MEDIUM | unknown case id on a case pack | BLOCK: 404 before the enqueue; `pack.build_failed {reason: case_not_found}` in the job
V-24-05-b | CONFIRMED LOW | case read model cited to phase 18; method unnamed | two REPs in 24; BLOCK 17: `CaseReadService::header(ctx, case_id) -> CaseView`
V-24-06-a | CONFIRMED LOW | stale `assessor.grant_max_days` in base text | five REPs (7.1 already rules)
V-24-07-a | CONFIRMED MEDIUM | access events of the sample read unclear | BLOCK: phase 02 list rule (`record_hits`, `search_hit` for restricted and suspicion items); "viewed" is the trace
V-24-07-b | CONFIRMED LOW | no method for route 27 | same BLOCK: `AssessorService::sample`
V-24-08-a | CONFIRMED MEDIUM | no list method for route 9, no decisions read | BLOCK: `GovernanceRegisterService::list`; `CaseDecisionRepository::register_page`; `DecisionRegisterRow`
V-24-08-b | CONFIRMED LOW | unknown kind on route 9 | sub-code `unknown_register_kind` (REP of 02-a and the route 9 BLOCK)
V-24-08-c | CONFIRMED MEDIUM | prose gives MRO reads that Cedar and the scope deny | lead decision: MRO gains `register.read` and `fairness.read` (policy `gov.read.mro`, scope `model.risk` on routes 9, 11, 13, 15); tile cell and two route numbers corrected; MANUAL line for API.md
V-24-09-a | CONFIRMED MEDIUM | no `WarningTitle` variant of this phase | REP: `ReviewCalendarRed`, `FairnessRateReview` with titles
V-24-09-b | CONFIRMED LOW | wrong section pointer | REP: section 4.1
V-24-10-a | CONFIRMED HIGH | control report case counts ran on `readonly_pool`, which reads nothing in `casework` | BLOCK: read on the app pool with `CaseAccessRepository::manual_grant_counts` (no definer function)
V-24-10-b | CONFIRMED LOW | wrong phase 18 citation | REP: phase 18 section 7.10
V-24-11-a | CONFIRMED MEDIUM | export GET rows name no Cedar action | two REPs
V-24-11-b | CONFIRMED MEDIUM | no export service or list methods named | BLOCK: `ExportService` with a closed screen map (G4, G9)
V-24-11-c | CONFIRMED MEDIUM | file route while the scan is pending | REP: 404 `E_NOT_FOUND`, status route shows the state
V-24-11-d | CONFIRMED MEDIUM | `register_export` missing from the job table | BLOCK (same): the row
V-24-11-e | CONFIRMED LOW | no step-up constant for the export download | BLOCK: `export.download` (export)
V-24-11-f | CONFIRMED MEDIUM | `register_export` not an evidence owner kind | BLOCK (same) and REP 02 on the registry table
V-24-11-g | CONFIRMED MEDIUM | retention class and classification of the export file | REP: R6 for `decisions`, R9 otherwise; `restricted`
V-24-12-a | CONFIRMED MEDIUM | no explanation method or repository | BLOCK: `ParallelRunService::explain`, `ParallelRunRepository`
V-24-12-b | CONFIRMED MEDIUM | two explainers could overwrite | BLOCK (same): guarded UPDATE, zero rows is 409
V-24-12-c | CONFIRMED MEDIUM | note scan not called | BLOCK (same): `PolicyPackService::scan` first
V-24-12-d | CONFIRMED LOW | no sub-codes for the code and the note | REP of 02-a
V-24-12-e | CONFIRMED LOW | Cedar text for the parallel-run actions | BLOCK: entity and three policies

## Phase 25 (50 confirmed: 6 HIGH, 23 MEDIUM, 21 LOW; 3 rejected)
V-25-01-a | CONFIRMED MEDIUM | one sentence said activation calls `CaseExistence`; step 8 and DR-233 say it does not | two REPs: the port is removed (down-rated: DR-233 rules)
V-25-01-b | CONFIRMED MEDIUM | End had no refusal code | REP: 409 `E_NOT_ALLOWED_IN_STATE`
V-25-01-c | CONFIRMED MEDIUM | an approver holding both roles who names none | REP: 422 `approver_role_required`; lead decision: a named role not held keeps the fallback
V-25-01-d | CONFIRMED MEDIUM | requester's end of a request wrote no audit event | REP: `break_glass.lapsed` (reused), table row widened
V-25-01-e | CONFIRMED LOW | step-up constants not declared | BLOCK: six constants
V-25-01-f | CONFIRMED LOW | unused `cfg` field | REP
V-25-01-g | CONFIRMED LOW | sweep counts unnamed | BLOCK (same)
V-25-01-h | CONFIRMED LOW | wrong Open doubts number | REP
V-25-02-a | CONFIRMED MEDIUM | no step inserts the first cursor row | REP: the first advance inserts
V-25-02-b | CONFIRMED HIGH | session advisory lock never released, so every later run returns at step 1 | REP: unlock on the same connection on every exit path
V-25-02-c | CONFIRMED LOW | limit above 500 | REP: 1 to 500, else 422 (lead decision: the route's own cap)
V-25-02-d | CONFIRMED LOW | `SiemForwarder` fields and entry point | BLOCK: fields, `SiemCollector`, `forward`
V-25-02-e | CONFIRMED MEDIUM | CEF severity has no source column | two REPs: read from the catalogue in the forwarder (down-rated from HIGH)
V-25-02-f | CONFIRMED LOW | cursor advance writes no audit event | BLOCK: stated exception (lead decision)
V-25-02-g | CONFIRMED MEDIUM | dead-letter resolution has no route or method | BLOCK and REP: resolving is P2; the tile counts seven days (lead decision)
V-25-02-h | REJECTED | refuted by the refuter
V-25-02-i | CONFIRMED MEDIUM | PKI mount for the SIEM client certificate defined nowhere | REP: certificate and key in KV `siem/client`, no PKI engine (lead decision)
V-25-02-j | CONFIRMED LOW | header names `SiemPageReader` | REP
V-25-03-a | CONFIRMED MEDIUM | request refusals had no code | REP: `vendor_user_invalid`, `sod_conflict` (reused), `session_open`
V-25-03-b | REJECTED | refuted by the refuter
V-25-03-c | CONFIRMED MEDIUM | ending a session in another state | REP: 409
V-25-03-d | CONFIRMED MEDIUM | `refused_timeout` wrote no audit event | BLOCK: `support_session.refused_timeout {session_id}`
V-25-03-e | CONFIRMED MEDIUM | VS grant insert had no method | BLOCK: `RoleGrantWriter::insert_support_grant`
V-25-03-f | CONFIRMED LOW | `SessionRevoker` undefined | two REPs: phase 03's offboarding order
V-25-04-a | CONFIRMED MEDIUM | `policy_bundle_hash_mismatch` never raised | BLOCK: compared with the hash in `notes.json` (lead decision)
V-25-04-b | CONFIRMED MEDIUM | missing directory or manifest | REP: `bundle_not_found`, `manifest_missing`
V-25-04-c | CONFIRMED MEDIUM | report evidence write names no method | REP: `EvidenceStore::put_generated`
V-25-04-d | CONFIRMED LOW | `ReleaseCurrency` names no method | REP: port dropped; the tile reads the repository (lead decision)
V-25-04-e | CONFIRMED LOW | move to `verifying` writes no event | BLOCK: stated exception (down-rated: a technical marker)
V-25-04-f | CONFIRMED LOW | IF marker missing on two rows | two REPs; MANUAL line
V-25-05-a | CONFIRMED MEDIUM | Kratos flag follows the mode yet the switch "needs no redeploy" | REP: a switch renders the config and restarts Kratos (lead decision; down-rated from HIGH)
V-25-05-b | REJECTED | refuted by the refuter
V-25-05-c | CONFIRMED LOW | header omits two subcommands | REP
V-25-05-d | CONFIRMED LOW | `OidcUpstreamConfig` undeclared | two REPs: `OidcUpstream`
V-25-06-a | CONFIRMED MEDIUM | tile counts a status no grant has | REP: the log kind `review_overdue`
V-25-06-b | CONFIRMED MEDIUM | lag grows on a quiet system | three REPs: age of the oldest row not yet forwarded
V-25-06-c | CONFIRMED MEDIUM | critical condition cannot occur | REP: a newer release unapplied for more than 30 days (lead decision)
V-25-06-d | CONFIRMED LOW | four tiles name no count method | BLOCK: the methods
V-25-07-a | CONFIRMED HIGH | `ON CONFLICT (source_key)` without the partial index predicate is refused by PostgreSQL | REP and BLOCK: `RegisterWriter::warn` (raised from MEDIUM)
V-25-07-b | CONFIRMED MEDIUM | item insert and snapshot update had no method | BLOCK: `insert_items`, `set_snapshot`
V-25-07-c | CONFIRMED LOW | `RoleGrantReader` undefined; two type names differ | two REPs; BLOCK: `in_force_at`
V-25-07-d | CONFIRMED HIGH | the overdue guard needed an audit read `akr_app` lacks | REP and BLOCK: the audit is written only when `warn` returns an id
V-25-07-e | CONFIRMED MEDIUM | `superseded` wrote no audit event | BLOCK: `access_review.item_superseded`; committed before the 409
V-25-07-f | CONFIRMED HIGH | owner kind `access_review` not in the evidence registry, so the snapshot is refused | REP 02
V-25-08-a | CONFIRMED HIGH | a malformed article had no outcome; section 6 used an undefined code | BLOCK and three REPs (25 and 30): refusal code `help_article_invalid` (lead decision)
V-25-08-b | CONFIRMED MEDIUM | help folder and `notes.json` not in the bundle contents | BLOCK 00
V-25-08-c | CONFIRMED LOW | SP-70 cited for UX-20 | REP
V-25-08-d | CONFIRMED LOW | "Route 19" has no referent | REP
V-25-09-a | CONFIRMED LOW | KV path written with the mount prefix | two REPs
V-25-10-a | CONFIRMED LOW | 7.3 cites a test that does not exist | BLOCK: the test
V-25-12-a | CONFIRMED HIGH | no route uploads the rollout record | BLOCK: `POST /admin/releases/{id}/evidence`; MANUAL line
V-25-12-b | CONFIRMED MEDIUM | `rollout_evidence_id` not carried | three REPs
V-25-12-c | CONFIRMED LOW | order of the owner check | REP (same)
Note for phase 26: `ON CONFLICT (source_key) DO NOTHING` at two places of phase 26 has the same missing predicate.

## Phase 26 (28 confirmed: 1 HIGH, 13 MEDIUM, 14 LOW; 6 rejected)

V-26-01-a | CONFIRMED MEDIUM | 3.1 step 2 tested a product "family Islamic" that no value list defines | two REPs 26: no family test; a deal whose product row has no type is refused; test 3(b) reads the product rows the fixture deals use
V-26-01-b | CONFIRMED LOW | test 3(d) expected a refusal the UNIQUE gives only for an equal `valid_from` | REP 26
V-26-02-a | CONFIRMED MEDIUM | the list reused steps that read one deal by id; no list steps, no method, no summary fields | REP 26 + BLOCK 26 (`IslamicDealService::list_for_customer`, `IslamicDealSummary` fields, `record_hits`; lead decision: no step-up on the list)
V-26-02-b | CONFIRMED MEDIUM | no `StepUpAction` constant declared | BLOCK 26 (`DEAL_OPEN`, `EVIDENCE_DECIDE`) + two REPs 26
V-26-02-c | CONFIRMED MEDIUM | display name read without the current-row filter | REP 26
V-26-02-d | REJECTED | the deal load's event is phase 11's `batch.recorded`
V-26-03-a | CONFIRMED MEDIUM | SCN-ISL-01 match conditions not written | BLOCK 26 (five conditions on existing fields; no resolver)
V-26-03-b | CONFIRMED MEDIUM | "his agent" and "a related party" had no relation or role | same BLOCK 26 (deal role `agent` or relation `attorney`; no DDL)
V-26-04-a | CONFIRMED MEDIUM | no method for the two G1 tile counts (down-rated from HIGH: 5.3 and check 6 m give the meaning) | BLOCK 26 (`islamic_awaiting_evidence`, `islamic_duplicate_refs` on phase 24's `CommandReadRepository`) + REP 26 of check 6 m (one shared reference reads as suppressed)
V-26-04-b | CONFIRMED LOW | service methods not named | BLOCK 26 (`attach`, `verify`, `content`, `sweep`, `EvidenceSweepCounts`)
V-26-04-c | REJECTED | `register.created` and `evidence.staged` are their owners' events
V-26-04-d | CONFIRMED LOW | stage call lacked the actor argument | REP 26
V-26-04-e | CONFIRMED LOW | pointer to a check 13 that does not exist | REP 26 (check 6 l)
V-26-04-f | CONFIRMED MEDIUM | `duplicate_exists` returned a bool nothing stored or counted | three REPs 26: the attach makes no duplicate read; the method is dropped; the tile counts at read time
V-26-05-a | CONFIRMED LOW | cap leg did not say IQD only | BLOCK 26 (SCN-ISL-06 and 07 conditions)
V-26-05-b | CONFIRMED MEDIUM | no rule fills `related_to_staff` | same BLOCK 26 + REP 26 (mapped flag or a `family` relation to a staff party; no derivation job)
V-26-05-c | CONFIRMED MEDIUM | "full repayment" and "unpaid" not mapped to statuses | same BLOCK 26 (`settled` or `closed` with `closed_on`; `active` or `defaulted`; facility leg P2 until the schedule mapping)
V-26-05-d | CONFIRMED MEDIUM | row read free-text `facility.contract_type`, against 3.1 step 2 | REP 26 (`facility.product_id`, `product.islamic_contract_type`) + same BLOCK 26
V-26-06-a | CONFIRMED HIGH | with group tasks the first verified group moved the whole freeze to `frozen` and silenced both freeze timers | BLOCK 26 (frozen only when every group task has a level; lowest level; freeze row lock) + REP 20 (pointer in 7.1) + REP 26 (step 8 follows fixes/20: release tasks on approval)
V-26-06-b | REJECTED | `AccountClassMap` content is stated by the setting
V-26-06-c | CONFIRMED LOW | `ops_task.created` is phase 20's `ops.task_created` | REP 26
V-26-07-a | CONFIRMED LOW | `AggregateCounts` fields and the query method not named | BLOCK 26
V-26-07-b | CONFIRMED LOW | two pointers to 3.4 for the aggregates | two REPs 26 (3.9)
V-26-07-c | CONFIRMED LOW | no sub-code for a malformed period | two REPs 26 (`period_invalid`)
V-26-07-d | CONFIRMED LOW | two pointers to a check 17 that does not exist | two REPs 26
V-26-08-a | CONFIRMED LOW | `--owner` refusal had no exit status | REP 26 (exit 1, nothing written)
V-26-08-b | REJECTED | the flag change keeps phase 03's `flag.updated`
V-26-08-c | CONFIRMED MEDIUM | the islamic document file was not in the release manifest | BLOCK 26
V-26-08-d | CONFIRMED LOW | rule 15 not extended to `pack: islamic` | same BLOCK 26
V-26-09-a | REJECTED | the DQ rows keep phase 12's `dq.recorded`
V-26-10-a | CONFIRMED MEDIUM | "normalised document_ref" undefined | REP 26 (`Trim`, `DigitsWestern`, `Upper`; also the 7.1 pointer to `Crypto::keyed_hash`)
V-26-11-a | CONFIRMED LOW | base text kept `ON CONFLICT (source_key)` without the index predicate under block 7.2 (down-rated: 7.2 routes through `warn`) | two REPs 26
V-26-11-b | REJECTED | `register.created` is phase 24's event
V-26-11-c | CONFIRMED LOW | block 7.2 put the title in "the message catalogue" | REP 26 (the `WarningTitle` map, with the bilingual title)

## Phase 27 (34 confirmed: 2 HIGH, 12 MEDIUM, 20 LOW; 1 duplicate; 5 rejected)

V-27-01-a | CONFIRMED MEDIUM | no reader of `feat.feature_values` for the sweep or the feature PSI | BLOCK 13 (`FeatureValueReader::read`, `FeatureValueRow`)
V-27-01-b | REJECTED | refuted by the refuter
V-27-01-c | CONFIRMED MEDIUM | a per-row missing count "stored in `shap`", which holds five contributions | REP 27 (no per-row count; the batch total is `skipped_missing`; no new column)
V-27-01-d | CONFIRMED LOW | the kill-state read before each 50-alert chunk was not in the steps | REP 27
V-27-01-e | CONFIRMED LOW | header pointer 3.4 for the forced sample | REP 27 (3.6)
V-27-02-a | CONFIRMED HIGH | family and version rows were inserted before their inventory item, but `inventory_item_id` is NOT NULL with a non-deferrable foreign key | BLOCK 27 (item first, then the row; the resolver runs at the use gate)
V-27-02-b | CONFIRMED LOW | no sub-codes for the name, purpose and starter checks | REP 27 (`name_not_bilingual`, `recalibration_due_required`)
V-27-02-c | CONFIRMED MEDIUM | version items did not copy `tier_basis` | REP 27
V-27-02-d | CONFIRMED LOW | the item's name was not stated | same BLOCK 27 as V-27-02-a
V-27-03-a | CONFIRMED MEDIUM | no repository reads for the monitor windows | BLOCK 27 (`score_window`, `entered_window`, `override_window`)
V-27-03-b | CONFIRMED MEDIUM | the red-breach finding had no bilingual title | REP 27 (fixed title; `find_by_source_key` before `insert`)
V-27-03-c | CONFIRMED MEDIUM | the daily monitor run had no audit kind | same BLOCK 27 (`model.monitored`)
V-27-03-d | REJECTED | refuted by the refuter
V-27-03-e | CONFIRMED LOW | `scored_count` undefined for shadow and challenger | REP 27 (written for each role, from `shadow_started_at` or `live_from`)
V-27-03-f | CONFIRMED LOW | header pointer 3.7 for the monitors | REP 27 (3.8)
V-27-03-g | CONFIRMED LOW | header credited phase 18 with `casework.case_alerts` | REP 27
V-27-04-a | CONFIRMED LOW | nine route numbers off against the table | nine REPs 27
V-27-04-b | CONFIRMED LOW | scope sentence false for SEC on the six read routes | REP 27 (scope admits, Cedar refuses)
V-27-04-c | CONFIRMED LOW | `ModelSwitch` did not state the other `ChangeTarget` methods | BLOCK 27
V-27-04-d | CONFIRMED LOW | the finding lookup at release was not named | REP 27 (`RegisterRepository::list`)
V-27-05-a | CONFIRMED MEDIUM | the shadow-start UPDATE had no repository method | BLOCK 27 (`begin_shadow`)
V-27-05-b | CONFIRMED MEDIUM | no bin edges or percentile rule for the provisional cut | BLOCK 27 (`reference_bins` shape; ten equal-width score bins; lower edge, no interpolation)
V-27-05-c | CONFIRMED LOW | shadow route row lacked `If-Match`; no test | REP 27 + BLOCK 27 (check 8 item q)
V-27-05-d | CONFIRMED LOW | shadow route cited as route 7 | three REPs 27
V-27-06-a | CONFIRMED MEDIUM | `ModelPin` step-up constant not declared | REP 27 (`{name: "model.pin", class: approval}`)
V-27-06-b | CONFIRMED MEDIUM | no refusal for a pin target of another family or in a dead status | BLOCK 27 + REP 27 (`not_pinnable`)
V-27-06-c | CONFIRMED LOW | route row said "the live version" (down-rated: wording) | REP 27
V-27-07-a | CONFIRMED MEDIUM | masked 404 where phase 17 and check 8 l give 401 first | two REPs 27
V-27-07-b | CONFIRMED MEDIUM | `for_alert_live` filtered by the current live id against DR-273 | REP 27
V-27-08-a | CONFIRMED LOW | the import insert left four NOT NULL columns unnamed (down-rated: field list) | REP 27
V-27-08-b | CONFIRMED HIGH | a card never scanned left the version in `importing` for ever and blocked every later import of the family (3 attempts, about 14 seconds) | REP 27 (10 attempts; refusal at the last attempt) + REP 27 (job table row)
V-27-08-c | REJECTED | refuted by the refuter
V-27-08-d | CONFIRMED LOW | step 7 said the item is created at verification (down-rated: wording) | REP 27
V-27-08-e | REJECTED | refuted by the refuter
V-27-08-f | CONFIRMED LOW | import cited as route 4 | REP 27
V-27-08-g | CONFIRMED LOW | lock order number | REP 27
V-27-09-a | CONFIRMED LOW | `if_match` of the inventory update not stated in 7.2 | REP 27
V-27-10-a | DUPLICATE | of V-27-07-a (the same 401-first rule, in block 7.3); fixed by REP 27
V-27-10-b | CONFIRMED LOW | block 7.3 named section 6 for the probe set | REP 27
V-27-12-a | REJECTED | phase 23 section 7.2 takes later phases' rows by class and clock: R11 is `report_only`, a `retirement` clock is never due

## Phase 29 (24 confirmed: 4 HIGH, 9 MEDIUM, 11 LOW; 7 rejected)

V-29-01-a | CONFIRMED LOW | no step says what a failed signature answers | REP: a `SignError` is a failure of step 16 (503 `E_UNAVAILABLE`, ledger row deleted, no pass)
V-29-01-b | CONFIRMED LOW | the hold rows of step 16 have no audit kind and no stated exemption | REP in 3.17: the hold rows belong to the decision record (DR-296), no audit row in the decision path
V-29-01-c | REJECTED | locator only (phase 14 section 3.4, not 3.5); corrected in passing by REP
V-29-02-a | CONFIRMED HIGH | `assess rail add` is named but has no step, and no path creates the first mode row (the base version was a row id in an integer column) | block: `rail add` inserts the rail and audits `assess.rail_added`; the first row is the PUT route with `mode: shadow`; base version of a mode change = the rail's `rail_modes` row count; REP of the base version
V-29-02-b | CONFIRMED MEDIUM | docs/API.md says the mode PUT is refused when the criteria fail, phase 29 checked at approval only | two REPs: `prepare` checks them too; section 6 item 29
V-29-02-c | REJECTED | writes of `activate` sit in the change target by design
V-29-03-a | CONFIRMED HIGH | bands were updated in place under a key without `set_version` while DELETE is refused, so a dropped band could never stop applying, and the guard and retention texts assume kept sets | block plus four REPs: key `(currency, band_code, set_version)`; the whole band set and the whole matrix are inserted per activation; current bands = greatest `set_version`
V-29-03-b | CONFIRMED LOW | `FailPolicyRow` undefined | REP: `FailPolicyRow { rail, corridor, currency, amount_band, on_failure }`
V-29-03-c | CONFIRMED MEDIUM | a missing catch-all row had no code | two REPs: sub-code `wildcard_row_missing`
V-29-04-a | CONFIRMED HIGH | no step set `register_id`, and the closing job has no user for `closed_by`, so an exception could never close | block: link at step 5 (a closed warning of the day gets a second key); closure by the phase 24 route with the caller as `closed_by`
V-29-04-b | CONFIRMED MEDIUM | closure trigger stated two ways (the job, the phase 24 route) | same block; REP of step 6: the phase 24 register route closes, as phase 24 section 7.3 says
V-29-04-c | CONFIRMED LOW | no repository method closes an exception | REP: `ReleaseRepository::close_for_register`
V-29-04-d | CONFIRMED MEDIUM | `message_refs.internal_ref` had no definition and no producer | block in 29 (field 20, `GrpHdr/MsgId`, `internal_ref`; the bank's mapping fills the key) and block in 10
V-29-04-e | CONFIRMED LOW | heartbeat and checkpoint unaudited with no exemption | REP in 3.17 (operational writes)
V-29-04-f | CONFIRMED HIGH | step 4 matched an advisory reject as expected while the next bullet makes it an exception, and the enforcing entry criterion reads that exception | REP: only an advisory hold is expected
V-29-05-a | REJECTED | an alert outcome moves only a held hold; stated
V-29-05-b | CONFIRMED MEDIUM | `decided_by` after a sync not stated | REP: the decider and time of the alert's latest decision
V-29-05-c | CONFIRMED MEDIUM | sync transitions and the alert link had no audit kind | same REP: `assess.hold_decided` per sync transition; REP: the link is covered by `screening.alert_created` (no new kind)
V-29-06-a | CONFIRMED LOW | a retirement proposal's row state not stated | REP (draft)
V-29-06-b | CONFIRMED LOW | no answer for an empty or over-long `basis` | REP: 422 `E_VALIDATION` at `/basis` (no new code)
V-29-06-c | CONFIRMED LOW | no signature for the rule evaluator | REP: `first_match(...)`
V-29-06-d | CONFIRMED LOW | `phase2_candidate` attributed to phase 16 | two REPs (draft)
V-29-07-a | REJECTED | the candidate query is designed in prose; a missing method name only
V-29-08-a | CONFIRMED LOW | `tier_basis` not named on the create call | REP (draft)
V-29-08-b | CONFIRMED LOW | "golden fixtures of section 6" do not exist | REP: the example rule and items 4 and 20
V-29-09-a | CONFIRMED MEDIUM | "does not answer" had no rule | block: connection error or no reply within 5 ms (a constant)
V-29-09-b | CONFIRMED MEDIUM | "per node" against the shared Valkey key and phase 30's division | three REPs: one bucket per gateway shared by the nodes
V-29-09-c | REJECTED | limiter behaviour stated in prose
V-29-10-a | CONFIRMED MEDIUM | source keys of the roll-up warnings not given | block: six keys; the repeat warning is per rail and day and holds no payment key
V-29-11-a | REJECTED | the scope is applied by step 1 of the job by design
V-29-12-a | REJECTED | section 7.6 states the dispositions; phase 23 handles them by class and clock
In passing (not counted): `ReleaseReason` gains `PassExpired` (the CHECK has four reason codes).

## Phase 30 (22 confirmed: 1 HIGH, 11 MEDIUM, 10 LOW; 2 rejected)

V-30-01-a | REJECTED | chart pins are named in Chart.yaml by the same convention as images
V-30-02-a | CONFIRMED LOW | the test writer's `nextval` had no sequence | REP: `id bigserial`
V-30-02-b | CONFIRMED LOW | "the phase 00 stanza" is not a phase 00 name | REP: the stanza the `pgbackrest` service configures
V-30-02-c | CONFIRMED LOW | pool options cited to phase 00 | REP (draft)
V-30-02-d | CONFIRMED MEDIUM | the winning condition of the proof of concept stated three ways | REP: one rule (Patroni all ten; CloudNativePG 1 to 4 and 6 to 10 plus the bank's acceptance of another backup tool in place of criterion 5)
V-30-03-a | CONFIRMED LOW | phase 00 section number | REP (draft)
V-30-03-b | CONFIRMED LOW | phase 25 check cited as step 1 (c) | REP (draft): section 7.7, check (g)
V-30-05-a | CONFIRMED HIGH | the namespace that holds a restored copy of production data had no network policy, no zone check and no admission rule | block: default deny, the restore's own edges only, probed, Kyverno Enforce, deleted after the drill
V-30-05-b | CONFIRMED LOW | probe image and connect timeout unnamed | REP: the `akr-ctl` image, 3 seconds
V-30-06-a | CONFIRMED MEDIUM | phase 00 limits the CLI exemption to one key | block in phase 00 (two keys)
V-30-06-b | CONFIRMED MEDIUM | null seed for a key phase 00 does not mark nullable | same block
V-30-06-c | CONFIRMED MEDIUM | `--result passed` with fewer than six cases was stored and audited | REP: refused, exit 3, nothing written
V-30-07-a | CONFIRMED MEDIUM | no call without a session for the probe's system period | block in 30 and note block in 11: `BlindPeriodService::record_system`; two REPs
V-30-07-b | CONFIRMED MEDIUM | operator path named no `declared_by` and no comment | same block: system principal, `comment` NULL, OS user in the audit event
V-30-07-c | REJECTED | designed: `blind_period.declared` is the steward route's kind; a period written without a session is `blind_period.opened`, and the operator's act has `ha.outage_recorded`
V-30-07-d | CONFIRMED MEDIUM | no overlap rule for system periods | same block: advisory lock, overlap writes nothing (command exit 2)
V-30-07-e | CONFIRMED LOW | role and grant of the CLI named by pointer only | same block: `akr_app`, `SELECT, INSERT`
V-30-07-f | CONFIRMED LOW | "phase 11 owns the rules" while the start is an estimate | same block and REP
V-30-08-a | CONFIRMED MEDIUM | the offline copy and the clean environment were undefined | block: monthly offline copy by the bank's operators; clean environment from the offline bundle; REP in 3.8
V-30-08-b | CONFIRMED MEDIUM | the rotation had no steps | same block: ordered runbook of existing commands; no single command in this version
V-30-08-c | CONFIRMED LOW | no audit event for the rotation | same block: each step's existing kind, the evidence file, `record-restore-drill`; no new kind
V-30-11-a | CONFIRMED LOW | `requested_by` of a restore run | REP: inserted as `recon_sweep` step 1 does, system principal
V-30-11-b | CONFIRMED MEDIUM | `run_active` not handled | same REP: the active run is used
V-30-12-a | CONFIRMED MEDIUM | "viewable by `admin.system`" with no route | two REPs: no route in this version (P2); the operator keeps the file, SEC sees the audit event

## Phase 31 (26 confirmed: 0 HIGH, 8 MEDIUM, 18 LOW; 4 rejected)

V-31-01-a | CONFIRMED LOW | "passed on first attempt" beside a flag that is TRUE when one case passed | two REPs: the flag keeps `bool_or`; rule 4 and section 5.5 say what it means, the counts are the measure
V-31-02-a | CONFIRMED MEDIUM | `ClassAggregate` had no fields, and steps 1, 6, 9 and 10 of the forecast had no read | block: the fields; `snapshot_dates` and `recent_buckets` on `CapacityRepository`
V-31-02-b | CONFIRMED LOW | the hourly snapshot upsert named no audit event | REP: a derived rebuild, no audit row, `computed_at` is its record
V-31-02-c | CONFIRMED MEDIUM | `ForecastQuery` contradicted the fixed sensitivity grid | REP (draft): `forecast(ctx)`
V-31-02-d | CONFIRMED LOW | `NewAlarm` written with a subject tuple | REP (draft): `subject_kind`, `subject_id`
V-31-03-a | CONFIRMED LOW | F4 named `ref.lists` for columns of `ref.list_versions` | REP (draft)
V-31-03-b | CONFIRMED LOW | no body shape of `POST /packs` for the two kinds | REP: the phase 06 `PackRequest`, `lang` left out, one row per language
V-31-03-c | CONFIRMED LOW | Cedar pointer to section 3.16 | REP (draft): 3.17
V-31-04-a | CONFIRMED MEDIUM | "status built" has no column | REP: a row of `admin.packs` exists only once published
V-31-04-b | CONFIRMED MEDIUM | no repository method for the pack link or the correspondent list | block: `RfiPackRepository` (`insert`, `for_rfis`), `RfiRepository::correspondent_page`, the two service signatures, `CorrespondentRfiRow` fields
V-31-04-c | REJECTED | refuted
V-31-04-d | CONFIRMED LOW | a pack of another kind had no stated answer | same REP as 04-a: `pack_not_ready` for every case, an unknown id included
V-31-04-e | CONFIRMED LOW | `recipient_ref` of a correspondent differs from the phase 19 column comment | REP in phase 19: the comment names the `canon.account` id from phase 31 on
V-31-05-a | CONFIRMED MEDIUM | outcome `deferred` is known only after the commit, the event row is written inside it and never updated | five REPs: no outcome `deferred` and no reason `disable_deferred` (both CHECK lists, step 5, section 3.12 step 3, lead corrections); the outcome stays `applied`, a Kratos failure is the phase 03 `offboard_finalize` retry
V-31-05-b | CONFIRMED LOW | `DirectoryCtx` not defined | two REPs: `ctx` is the caller context; Cedar on the event path, the system context on the reconciliation path
V-31-05-c | CONFIRMED LOW | lead corrections named an outcome `disable_deferred` | same REP as 05-a on that line
V-31-06-a | CONFIRMED MEDIUM | no method returned the daily would-alert counts | REPs: `CapacityRepository::replay_daily_counts` (this phase reads `scn.replay_results`, so phase 16 stays unchanged), `run`, `start_run`; step 1 uses phase 16 `ReplayRepository::get`
V-31-06-b | REJECTED | refuted
V-31-06-c | CONFIRMED LOW | marking a run `running` named no audit event | same REP on step 1: the run-level pair covers it
V-31-06-d | CONFIRMED LOW | default `class_mix` pointed at a mix section 3.4 does not compute | REP: required, no default (DR-342); 422 `E_VALIDATION` at `/inputs/class_mix`
V-31-06-e | REJECTED | refuted
V-31-07-a | CONFIRMED LOW | pruning key named as business date | REP (draft): the BRIN index on `business_date`
V-31-07-b | CONFIRMED LOW | `ExposureRunSummary` had no fields | REP: the three audit detail values
V-31-08-a | REJECTED | refuted
V-31-08-b | CONFIRMED MEDIUM | "recorded as `directory.mismatch`" had no step, and an update did not say it stores the e-mail | three REPs: `update` replaces `user_name_enc` and never touches the account; reconciliation item 3 gains `email_count`; the 3.14 row gains the key
V-31-08-c | CONFIRMED LOW | grant route credited to phase 03 | REP (draft): phase 04
V-31-10-a | CONFIRMED LOW | block 7.2 said section 3.12 implements the port | REP on the block line: the `directory` module implements it
V-31-10-b | CONFIRMED LOW | the port was missing from the AppState list | REP (draft)
V-31-11-a | CONFIRMED LOW | item 23 read as an existing deletion path | REP (draft): SELECT only in this release
V-31-12-a | CONFIRMED MEDIUM | the base of the surge "rate" was undefined | REPs: phase 19 block 7.5 gets the formula (a floor of `ceil(N * percent / 100)` on a draw sized from the settings); phase 31 sections 3.6 and 4.5 item 9 follow it
