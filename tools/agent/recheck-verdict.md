# Re-check verdict (lead)

Re-check of the 767 entries applied by the verification pass (DR-413). One Haiku agent per phase read
`tools/agent/verify/applied-NN.txt` and the fixed phase file and wrote `tools/agent/verify/recheck-NN.md`.
The lead decided every finding against the phase text. Format:
`R-NN-k | CONFIRMED severity | what | fix` or `R-NN-k | REJECTED | reason`. `L-NN-k` is a gap the lead found while
checking a finding.

Totals: 153 findings raised by the agents, all 153 confirmed and none rejected; 8 more found by the lead: 2 while deciding (L-17-1, L-21-1) and 6 in the second look at the applied fixes (L-04-1, L-06-1, L-11-1, L-15-1, L-18-1, L-24-1; five of them are route counts left stale by gap blocks that added routes). By lead severity: 7 HIGH, 43 MEDIUM, 111 LOW (the agents had rated 20 HIGH, 72 MEDIUM, 61 LOW). All 161 are fixed: 221 replacements in `tools/agent/recheck-fixes.txt`, applied by `tools/agent/recheck-fix.py`.

## Phase 00
- R-00-1 | CONFIRMED MEDIUM | the heartbeat is guarded by `locked_by` alone; the reaper re-queues without clearing it, so a late heartbeat still matches the row, against the block's own premise | guard is `status = 'running'` and `locked_by`
- R-00-2 | CONFIRMED LOW | `job.dead` is in the audit-event table of section 3, not section 4 | pointer

## Phase 01
- R-01-1 | CONFIRMED MEDIUM | step 1 of `client create` (7.9) inserts the service row without `display_name` (NOT NULL, both keys), and `--name` is never stored, so `rotate` and `disable` cannot find the row | `display_name` set from `--name`; the name is unique among service principals not disabled and is the lookup key
- R-01-2 | CONFIRMED LOW (raised MEDIUM) | 7.10 says the pinned list covers every route "but two" (heartbeat, logout), section 3 says the two public routes have no action | wording: two routes needed an action; the public ones need none

## Phase 02
- R-02-1 | CONFIRMED HIGH (raised HIGH) | the verifier reads as `akr_readonly`, and step 2 for the `case_event` stream reads `casework.case_events`, on which that role has no grant | phase 17 grants SELECT on the four columns `(case_id, seq, at, hash)` to `akr_readonly`; 7.17 names the grant
- R-02-2 | CONFIRMED MEDIUM (raised HIGH) | `ShredOutcome` is a struct in 7.16 and an enum in phase 23 section 7.8 | 7.16 takes the enum `{ Shredded, AlreadyShredded }`
- R-02-3 | CONFIRMED LOW (raised HIGH) | the metric label `{template}` cannot be filled, since `RenderJob` has no template name | the label is dropped in step 4 and section 4; 7.14 says so
- R-02-4 | CONFIRMED LOW (raised MEDIUM) | the 7.1 row shows a dash for `validation_pack` and `register_export`, for which phase 24 states R11 and R6 or R9 | the row is split in three
- R-02-5 | CONFIRMED MEDIUM (raised MEDIUM) | "SIEM when infected" cannot be a boolean flag | 7.13 names the three phase 02 kinds with the flag; `evidence.rejected` is exported for both outcomes and `reason` tells them apart

## Phase 03
- R-03-1 | CONFIRMED MEDIUM | block 7.15 adds three `dev.*` actions and `dev.cedar` but declares them neither in the schema nor in `Action::ALL`, so the inventory of section 3.1 step 3 fails in a `dev-routes` build | one sentence: declared under the feature in a schema fragment, with the `_dev_masked` entity
- R-03-2 | CONFIRMED LOW (raised MEDIUM) | 7.12 prints `roles` as a map, 7.16 as the `RoleActions` list | 7.12 prints the list
- R-03-3 | CONFIRMED LOW (raised MEDIUM) | step 3 of `build` has no key for the `sod.json` rules that step 1 and phase 25 name | key `sod` (`LoadedBundle.sod`)
- R-03-4 | CONFIRMED MEDIUM | 7.17 lets a kind outside `MASKED_KINDS` be refused as masked, but the access event, the same-work rule and the latency floor are tied to "a masked kind" | the kinds named by `@deny("masked")` forbids count as masked kinds for those three rules
- R-03-5 | CONFIRMED LOW | 7.17 points to section 3.6 for the `deny` access event; it is section 3.2 step 8 | pointer

## Phase 04
- R-04-1 | CONFIRMED LOW | the pointer "(section 3.1, DR-54)" in section 4 reads as a section of this phase; the statement is in phase 03 section 3.1 and in section 3.5 of phase 04 | pointer names both
- R-04-2 | CONFIRMED LOW | the section 1 comment and the section 6 check still say only lifecycle columns are updatable, while the grant now includes `evidence` (7.18) | both lines name `evidence`
- R-04-3 | CONFIRMED LOW | the evidence route closes at "no decision recorded yet" in the table and at outcome `approved` or `rejected` in the steps; a `signed` outcome is a decision too | steps say: no deciding task in state `done` in the current pass
- R-04-4 | CONFIRMED MEDIUM | block 7.19 names no refusal code for `mode: rework` without `to`, nor for a `to` that is not an earlier step | `E_SCHEMA` and `E_DANGLING_REF` named
- L-04-1 | CONFIRMED LOW (lead, second look) | the Complexity line and the status.md row still count 31 routes; the phase has 32 with the evidence route of 7.2 (R-07-1 fixed the three check places only) | both say 32

## Phase 05
- R-05-1 | CONFIRMED LOW | "Reused" line names `evidence.sealed`, an event no phase defines, and not `evidence.stored`, which the run report now emits | line corrected
- R-05-2 | CONFIRMED LOW | VD-02 is marked "partly" but sits in the "implemented here" list | moved to the partly-implemented list
- R-05-3 | CONFIRMED LOW (raised MEDIUM) | the retention seed note still calls `admin.source_pauses` "Coverage-gap evidence", against 7.12 | note reworded
- R-05-4 | CONFIRMED LOW | "no backfill planner (phase 11, DR-63)" points at a planner no phase builds (DR-380) | pointer corrected

## Phase 06
- R-06-1 | CONFIRMED LOW | 7.10 makes `code` and `name` of a linked entry optional but leaves `restricted` required on every entry | `restricted` is present only on a restricted entry
- R-06-2 | CONFIRMED HIGH | step 1 of `gap_report_build` ends on any run that is not `queued`, so the retry of a failed render stops at once and leaves the run `running`; the active-run index then refuses every new run | a `running` run is a retry and resumes
- R-06-3 | CONFIRMED HIGH | the cover reads `deploy/branding.json` "which akr-worker mounts", but section 4 says compose is unchanged and the phase 00 volume list has no such mount, so every run would fail | the mount is added in phase 06 section 4 and phase 00 section 7.8
- R-06-4 | CONFIRMED LOW | pointer: the file is named in phase 08 section 3.10, not section 4 | pointer
- R-06-5 | CONFIRMED LOW (raised MEDIUM) | `AttributeKey` is used and never defined | defined in the 7.x block
- L-06-1 | CONFIRMED LOW (lead, second look) | status.md counts 13 routes for phase 06; the phase has 14 (the typology route of section 7+) and says 14 itself | status.md says 14

## Phase 07
- R-07-1 | CONFIRMED MEDIUM (raised HIGH) | the running operation totals are one short from phase 04 on: phase 04 has 32 operations with the evidence route of 7.2, so 86, 107, 121 and 130, not 85, 106, 120 and 129 (lead counted the route rows: 22, 32, 32, 21, 14, 9) | six numbers corrected in phases 04 to 07

## Phase 08
- R-08-1 | CONFIRMED LOW (raised MEDIUM) | the G10 list has no reason-code column for the tag to sit beside | on the list the tag sits beside the class badge
- R-08-2 | CONFIRMED LOW (raised LOW) | "asks for a new code" ignores the WebAuthn proof | "a new proof (a new TOTP code or a fresh security-key touch)"
- R-08-3 | CONFIRMED LOW (raised MEDIUM) | section 1 says the phase writes `shortcuts` and `theme`; the screens write neither | the list is `density` and `prose_digits`
- R-08-4 | CONFIRMED LOW (raised LOW) | 7.2 says one `RailGroup` value per rail entry; 7.8 puts two entries in `reports` | "one value per rail group"
- R-08-5 | CONFIRMED MEDIUM (raised MEDIUM) | the source label of the transliteration is "the label of `source_field`", and no such label exists | the label is the word `typed`; `card`, `passport` and `generated` are not used in this plan

## Phase 09
- R-09-1 | CONFIRMED LOW | the rename block says section 4's phase 00 and phase 04 bullets include the rename; they do not | sentence corrected
- R-09-2 | CONFIRMED LOW (raised MEDIUM) | the block says `unknown_queue` and `pool_archived` are raised by the authority matrix, which has no queue or pool field | the two codes leave the section 3.8 row
- R-09-3 | CONFIRMED LOW (raised MEDIUM) | the Activation paragraph says a new pack version is used only by cases opened later, while the save-time scan always uses the active version | the exception is named
- R-09-4 | CONFIRMED LOW (raised MEDIUM) | the Keys paragraph calls the release schema closed over three table files, yet the default calendar and the reason-code file sit in the same folder | the calendar file's keys and the status of `reason-codes.v1.json` are stated

## Phase 10
- R-10-1 | CONFIRMED HIGH | the fact insert of section 3.7 step 5 names the conflict target `(source key, partition key)`; six fact tables have no unique index on it (`txn_event`, `cash_event`, `balance_snapshot`, `payment_message_party`, `payment_link`, `account_hold`), and PostgreSQL refuses such a target | step 5 names the target per table; two unique constraints are added in migrations 0030 and 0031
- R-10-2 | CONFIRMED LOW (raised MEDIUM) | section 3.3 says a card-guard hit "stops the field's load and opens a data incident" in phase 11; 7.16 and phase 12 make it a quarantine row and a data-quality issue | 3.3 reworded to the same rule
- R-10-3 | CONFIRMED MEDIUM | `RowFailure` (called closed) has no code for a row the `txn_type` gate holds, which phase 11 writes with reason `unmapped_code` | code added; `unmapped_codes` counts values
- R-10-4 | CONFIRMED LOW (raised MEDIUM) | 7.17 gives every `CurrentRow` the bi-temporal columns, which facts do not have | wording
- R-10-5 | CONFIRMED LOW (raised MEDIUM) | the folded form cites steps N1 to N6; the pipeline has N1 to N11 | range corrected
- R-10-6 | CONFIRMED LOW | backticks inside a code span break the formula | inner backticks removed

## Phase 11
- R-11-1 | CONFIRMED LOW (raised MEDIUM) | 7.13 says `source.halt_resolved` carries the marker; the payload of step 4 and the audit row do not list it | payload and row name `as_of_marker`
- R-11-2 | CONFIRMED MEDIUM | `ensure_rows` (7.14) inserts with no `freshness_class`, so every watermark starts as the default F3, against the class rule of section 3.7 | the caller passes the class
- R-11-3 | CONFIRMED LOW | job step 7 raises `cost_guard` "by the same factor"; no factor exists | the job's own estimate against `replay.estimate_max_rows`
- R-11-4 | CONFIRMED MEDIUM (raised HIGH) | a replay receipt is said to have its "watermark fields NULL"; `watermark_kind` is NOT NULL | the three nullable fields are named; the kind is copied
- R-11-5 | CONFIRMED MEDIUM (raised HIGH) | the crate tree puts `BlindPeriodService` in akr-ingestd, while akr-api holds it and phase 30 calls it from akr-worker and akr-ctl; linking akr-ingestd would pull in the drivers | the service is a module of `db`
- R-11-6 | CONFIRMED LOW | pointer "of this section" should name 7.20 | pointer
- R-11-7 | CONFIRMED LOW (raised MEDIUM) | 7.20 item 4 reads the columns from the newest receipt, 7.5 from the newest accepted manifest | 7.20 says accepted manifest
- R-11-8 | CONFIRMED LOW | `column_check_failed` is missing from the receipt-reason list of section 3.10 | added
- R-11-9 | CONFIRMED LOW | pointer "the late-arrival block of this section" should name 7.22 item 4 | pointer
- R-11-10 | CONFIRMED MEDIUM | `previous_mapped_business_to` takes no source, though two sources may read a table of one name | `source_id` added
- L-11-1 | CONFIRMED LOW (lead, second look) | after R-11-2 the text says `ensure_rows` takes the class as one more argument, and the signature printed beside it did not have it | signature gains `class: FreshnessClass`

## Phase 12
- R-12-1 | CONFIRMED LOW | block 7.8 quotes words of block 7.5 that are not there verbatim | quotation marks dropped, wording aligned
- R-12-2 | CONFIRMED LOW (raised MEDIUM) | the base trait and the step 7 call of `accepted_for` lack the `field` argument that block 7.5 gives the method | both lines gain it

## Phase 13
- R-13-1 | CONFIRMED MEDIUM | `EXCL_POOL_ACCOUNT` is only the example code of docs/DSL.md A.7 and its prefix breaks the phase 09 rule (`EXC_`); no phase defines it | phase 13 names a static exclusion of the scenario document with an active `EXC_` code; the DSL example becomes `EXC_POOL_ACCOUNT`
- R-13-2 | CONFIRMED HIGH (raised HIGH) | the seed feature `distinct_counterparties_3m` takes `txn.counterparty_id` as its field; that column is class `personal`, which step 2 of the validator refuses, so the nine-feature seed exits 1 | step 2 gains one exception: the field of a `distinct_count` may be `txn.counterparty_id`
- R-13-3 | CONFIRMED MEDIUM (raised MEDIUM) | step 3 of the cluster assignment opens a row only for a customer whose open row names another segment, so a first build writes no membership | a customer with no open row has the new row opened
- R-13-4 | CONFIRMED LOW (raised MEDIUM) | sub-code `too_long` for a range; the phase 11 row has `range_too_long` | renamed
- R-13-5 | CONFIRMED LOW (raised LOW) | "uses it" points at `active_static` | "calls `active_by_level`"
- R-13-6 | CONFIRMED LOW (raised LOW) | `combine` of the `profile_refresh` plan not given | `all`; it changes nothing the job stores

## Phase 14
- R-14-1 | CONFIRMED MEDIUM (raised HIGH) | 7.x says a step-up constant's name is the Cedar action of its route, and that the bulk decision reuses the constant named `screening.alert.decide`; the bulk route's action is `screening.alert.bulk_decide` | the bulk route declares its own constant
- R-14-2 | CONFIRMED LOW (raised MEDIUM) | the revoke body is called "the second form of `GoodGuyProposal`", a struct with one form | a second DTO `GoodGuyRevoke` is named in section 2.3 and at the route
- R-14-3 | CONFIRMED LOW | the claim that no feed format carries a delisting date cites a step that does not say so | marked as a decision of the phase

## Phase 15
- R-15-1 | CONFIRMED MEDIUM | 7.13 freezes `party_id` in the exit instance but lets the register body carry its own `party_id` unchecked | a body party that differs from the frozen one is 422
- R-15-2 | CONFIRMED LOW | the resource id the `exit_decision` instance is started with is not given | `("exit_decision", review_id)`
- R-15-3 | CONFIRMED LOW | neither new subject has the `title` that `SubjectContext` requires | two titles given
- R-15-4 | CONFIRMED LOW (raised MEDIUM) | row C3 still lists the registration country as wired; C2 and Open doubts 10 say it is pending | row corrected
- R-15-5 | CONFIRMED LOW | "the entry's `primary` row" does not name the column `alias_class` | column named
- R-15-6 | CONFIRMED LOW | 7.7 step 3 keeps the two-argument clock call that 7.17 replaces | call corrected in 7.7
- R-15-7 | CONFIRMED MEDIUM | 7.18 says `open_or_join` returns the old and the new `due_at`; the trait returns `(KycReview, bool)` and RETURNING cannot show the old value in PostgreSQL 17 | return type and the CTE that reads the old value are given
- L-15-1 | CONFIRMED MEDIUM (lead, second look) | check 9 says Swagger lists 32 new operations and the header, the OpenAPI line and status.md say 32; sections 7.4 and 7.5 add two routes, so the stated check would fail at 34 | the four places say 34 and name the two sections

## Phase 16
- R-16-1 | CONFIRMED LOW | "the four entry points of section 3.2": one of them is in section 3.6 | pointer
- R-16-2 | CONFIRMED LOW | 7.15 calls every set inside the band that does not loosen "minor"; a `capacity` reason is at least material | clause narrowed
- R-16-3 | CONFIRMED LOW (raised MEDIUM) | the route row and a column comment still say the test route runs unit and seeded cases; 7.17 makes it unit cases only | both lines corrected
- R-16-4 | CONFIRMED MEDIUM | `touch_id` on `scn.scenario_runs` exists only in the prose of 7.9; the CREATE TABLE and the unique index of migration 0055 do not have it | column, CHECK and index written into the migration text

## Phase 17
- R-17-1 | CONFIRMED MEDIUM (raised MEDIUM) | section 3.2 says the retry job sets `revalidated_at`, and no retry step writes it | 7.19 sets it with `valid_now`; a still-invalid row is not written
- R-17-2 | CONFIRMED MEDIUM (raised MEDIUM) | no test for a pool target of a case | the pool must be that of an active `alerts` queue that admits the case, or the case's own pool; else 422 at `/to`
- R-17-3 | CONFIRMED MEDIUM (raised MEDIUM) | the bulk `mark_duplicate` checks omit the same-case rule of triage | check 5 gains `not_same_case` (one answer for a missing and a hidden target), added to the closed list
- R-17-4 | CONFIRMED LOW (raised LOW) | `lower_refused` is in the bulk reason list and no bulk action can give it | removed from the list
- R-17-5 | CONFIRMED MEDIUM (raised HIGH) | 7.25 skips step 3 of the phase 18 grant, so an A1 second worker would get `decide` | a second worker must hold A2 or M (422 `role_cannot_decide`); A2 or M assignee gets `decide`, A1 gets `work`
- R-17-6 | CONFIRMED MEDIUM (raised HIGH) | an assignee who already holds an active grant breaks the unique index `uq_case_acl_active` | a grant of the same or wider access is kept; a narrower one is revoked and replaced; a reassignment revokes only grants of reason `assignment` or `second_review` (same replacement as R-17-5)
- R-17-7 | CONFIRMED LOW (raised LOW) | `case.assign` is of section 3.17, not 3.8 | pointer fixed (same replacement as R-17-5)
- L-17-1 | CONFIRMED LOW (lead) | section 3.10 says the step-up answer comes "before any tier answer"; read literally that gives 401 to a caller outside the team, which DR-411 (phase 03 section 7.14) forbids, and it misled a fix in phase 27 | the sentence names the admitted caller (401) and the refused caller (masked 404)

## Phase 18
- R-18-1 | CONFIRMED LOW (raised MEDIUM) | section 5.4 still says each opening posts `opened-views`; the route takes panels only (7.x block: an evidence item's row comes from the content read) | the dialog line says so
- R-18-2 | CONFIRMED LOW (raised MEDIUM) | section 3.7 step 3 calls `GenericTextGuard::check` with two arguments, without `conn` | call gains `conn`
- R-18-3 | CONFIRMED LOW (raised MEDIUM) | section 3.3 step 4 gives the note of a manual grant a higher minimum, but `post` takes `CreateNote` only, so nothing says how it learns the minimum | step 4 says the minimum is an internal argument of `post`
- R-18-4 | CONFIRMED LOW | block 7.25 says section 4.5 item 13 covers the grant-expiry timer kind and setting; item 13 names only the work-item ones | item 13 names both
- L-18-1 | CONFIRMED LOW (lead, second look) | the header, the Count sentence, check 7 and status.md count 25 routes; sections 7.11, 7.12 and 7.13 add one each | the header and status.md say 28; the Count sentence and check 7 name the three sections

## Phase 19
- R-19-1 | CONFIRMED MEDIUM (raised MEDIUM) | the route table has 32 rows (13 added) and the header, the count sentence, 4.5 item 16, check 7 and status.md still say 31 and 12 | the counts become 32 and 13; 7.14 says DR-157 counted thirty-one before the new route
- R-19-2 | CONFIRMED LOW (raised MEDIUM) | `not_critical_finding` is missing from the closed sub-code list of section 3.11 | appended
- R-19-3 | CONFIRMED LOW (raised MEDIUM) | the dialog lists "at most 25" cases and its route returns up to 100 | up to 100 candidates, at most `qa.bulk_reopen_max` selected
- R-19-4 | CONFIRMED LOW (raised MEDIUM) | three places say a QA item never shows a party name; the verified output of an `assistant_draft` item may name parties | the exception is written in the three places

## Phase 20
- R-20-1 | CONFIRMED MEDIUM (raised HIGH) | 7.4 has the seed publish version 2 of `exit_decision`; the phase 04 seed leaves an existing family alone and never changes a published definition | version 2 arrives as a draft through `--offer-updates` and the bank publishes it; a first install gets it as its first version
- R-20-2 | CONFIRMED LOW | pointer "(section 3.2)" for `source_id` should be the column comment of section 1.1 | pointer
- R-20-3 | CONFIRMED LOW (raised MEDIUM) | `NewVerification` omits `ops_task_id` (NOT NULL) | added to the list
- R-20-4 | CONFIRMED LOW (raised MEDIUM) | the derived values of `NewOpsTask` omit `authority_ref` on the freeze path | added
- R-20-5 | CONFIRMED LOW | doubled words "the reference and reference equal to" | wording
- R-20-6 | CONFIRMED LOW | a clause of the W9 line hangs after the inserted sentence | clause moved
- R-20-7 | CONFIRMED LOW | "W-OPS" is not a screen id; the screen is W9 | id corrected

## Phase 21
- R-21-1 | CONFIRMED LOW (raised HIGH) | 7.8 and the table comment say the manifest holds the PDF hash; the manifest is built before the PDF, which prints the manifest hash | the manifest holds the hashes of the XML and the attachments
- R-21-2 | CONFIRMED LOW (raised MEDIUM) | the trailer still excepts "owner methods" of 4.5 item 9, which adds none | the trailer names the one port method `LetterRequester::delivered`
- R-21-3 | CONFIRMED LOW (raised LOW) | the guard comment says the approval is written once; a revise clears it | comment corrected
- R-21-4 | CONFIRMED LOW (raised LOW) | 7.4 item 1 still says the engine's excluded set adds the drafter and the authors | the subject refuses them (7.12)
- R-21-5 | CONFIRMED LOW (raised LOW) | the comment on `rpt.report_links` names only the STR for an escalated link | "STR or SAR"
- L-21-1 | CONFIRMED MEDIUM (lead) | the seed gives `CLS_CTR_EXCLUDED_CANCELLED` and `CLS_CTR_EXCLUDED_REVERSED` the family `exclusion`; the prefix `CLS_` is the family `closure` (phase 09 section 3 codes rule), and phase 12 would accept them as register codes and then fail its `^EXC_` CHECK | family `closure` in the seed

## Phase 22
- R-22-1 | CONFIRMED LOW (raised MEDIUM) | 7.11 says an approver learns of an open deciding task through `task.waiting`; phase 04 sends that notice to the escalation target only | the sentence names the inbox and `task.overdue`
- R-22-2 | CONFIRMED LOW (raised MEDIUM) | `by_internal_report` returns `Observation`, which has no notes | it returns `ObservationView`, selects `notes_enc` and decrypts it; phase 19 calls it for M only

## Phase 23
- R-23-1 | CONFIRMED LOW (raised MEDIUM) | the party-path sentence can be read as giving `facility` no path | wording: `facility` and `trade_instrument` each through `customer_id`
- R-23-2 | CONFIRMED LOW | section 3.4 still lists `feat.feature_runs` among the droppable parents; 7.10 made it a record purge by age and does not say whether a hold applies | removed from 3.4; 7.10 says no hold applies (no party data)

## Phase 24
- R-24-1 | CONFIRMED HIGH (raised HIGH) | the export file gets classification `restricted`, which is not a value of the enum `admin.data_classification`; the insert of every export file would fail | classification `personal`
- R-24-2 | CONFIRMED MEDIUM (raised MEDIUM) | scenario rows of the review calendar keep a second state rule (amber at one interval, red at two) | a scenario row gets `due_on` = activation plus one interval, and the one state rule applies
- R-24-3 | CONFIRMED MEDIUM (raised MEDIUM) | nothing in the export request carries the register kind that the G4 export needs | `filters` carry `kind`; a missing or unknown kind is 422
- R-24-4 | CONFIRMED LOW (raised MEDIUM) | "G5 holds counts only" is the wrong reason: G5 also shows the access history of one record | the true reason is written: the history is read per record and is not exported
- R-24-5 | CONFIRMED MEDIUM (raised MEDIUM) | the explain UPDATE has no visibility test, so the stated 404 cannot happen and a hidden row can be written | a visibility read runs first in the same transaction and answers 404
- R-24-6 | CONFIRMED LOW (raised LOW) | "section 7.1" points at the wrong phase 04 section | "phase 04 section 7.10"
- L-24-1 | CONFIRMED LOW (lead, second look) | the header, the Route count bullet, check 6 and status.md count 31 routes (28 new and 3 extended); sections 7.12 and 7.13 add eight | the header says 39, status.md 36 new and 3 extended, and the bullet and check 6 name the two sections. The Routes total of status.md becomes 581, the sum of the column and the number of API.md rows

## Phase 25
- R-25-1 | CONFIRMED MEDIUM (raised MEDIUM) | the SP-36 rule entries have `window` and `threshold` and no value, and "key operations" names no exported kind | proposed starting values the bank tunes in its SIEM; the privileged-burst kinds are named
- R-25-2 | CONFIRMED LOW (raised LOW) | `SweepCounts` has three field sets and the access review sweep is missing | one map type for the four sweeps, with the count names of each
- R-25-3 | CONFIRMED LOW (raised LOW) | the severity sentence of section 3.5 reads broken | reworded
- R-25-4 | CONFIRMED LOW (raised LOW) | "clears once the cause is fixed" is wrong for a seven-day count | "clears seven days after the last dead letter"
- R-25-5 | CONFIRMED LOW (raised MEDIUM) | `SiemStatus.dead_letters_open` and the metric keep the name of the replaced rule | `dead_letters_7d` and `akr_siem_dead_letters_7d`
- R-25-6 | CONFIRMED LOW (raised LOW) | `invalidate_user` is phase 01's, not phase 03's | attribution fixed
- R-25-7 | CONFIRMED MEDIUM (raised MEDIUM) | the warn cell and the Currency paragraph leave out `proposed`, and the 30 days have no clock | both name the three states; the wait is counted from `verified_at`
- R-25-8 | CONFIRMED LOW (raised MEDIUM) | section 3.9 still runs the article check inside step 5; 7.20 moved it before step 5 | section 3.9 follows 7.20
- R-25-9 | CONFIRMED MEDIUM (raised MEDIUM) | the rollout upload names no content type, and `application/json` is not an allowed evidence type | declared type `text/plain`
- R-25-10 | CONFIRMED LOW (raised LOW) | the upload route is missing from the section 2 table and the counts say 21 and 9 | the row is added; counts 22 and 10 (phase 25 and status.md)

## Phase 26
- R-26-1 | CONFIRMED LOW (raised LOW) | "section 4.5 gains this phase 01 item" points at nothing; phase 01 does not change | "both classes exist, so phase 01 does not change"
- R-26-2 | CONFIRMED LOW (raised LOW) | a stray semicolon before the new sentence of the repository bullet | full stop
- R-26-3 | CONFIRMED MEDIUM (raised MEDIUM) | the test says the duplicate tile is absent for `a2@`, who gets 403 on G1 and never reads a tile | the test uses `ch@` and `ia@`; `a2@` gets 403 `E_SCOPE`
- R-26-4 | CONFIRMED LOW (raised MEDIUM) | `content` has no step-up header parameter though its step 1 consumes the step-up; `get` has the same gap | both take `header`
- R-26-5 | CONFIRMED LOW (raised MEDIUM) | the trailer says phase 20 changes only by the task grouping; 7.10 adds the group state rule | the trailer names both
- R-26-6 | CONFIRMED LOW (raised LOW) | `FreezeService::release_freeze` is not a method; it is a terminal effect | "the terminal effect `release_freeze`"
- R-26-7 | CONFIRMED LOW (raised LOW) | check 10 does not hold the three file checks that section 3.13 sends to it | check 10 gains them

## Phase 27
- R-27-1 | CONFIRMED LOW (raised LOW) | "the 7.x block on bins" is a placeholder | "section 7.10"
- R-27-2 | CONFIRMED LOW (raised LOW) | a share from 0 to 1 is compared with a setting in per cent | "`model.top_band_percent` / 100"
- R-27-3 | CONFIRMED LOW (raised LOW) | the 428 and 409 row leaves out shadow | "promote, pin, shadow, engage"
- R-27-4 | CONFIRMED LOW (raised LOW) | the `E_VALIDATION` row leaves out section 3.10 (pin) | added
- R-27-5 | CONFIRMED LOW (raised LOW) | the sweep scores in chunks of 50 and step 6 still writes one transaction per batch | one transaction per chunk; one audit per batch; a stopped batch keeps its committed chunks
- R-27-6 | CONFIRMED LOW (raised LOW) | "`revert` is not implemented" leaves the return value open | it returns `ChangeError::NotRevertible`, as in phase 16
- R-27-7 | CONFIRMED HIGH (raised MEDIUM) | section 3.3, 7.3 and test l give 401 to any caller without step-up on a suspicion-tier alert; a caller outside the team then learns that the alert exists, and the phase 03 probe of DR-411 (a caller outside the team gets 404, not 401) fails | the masked 404 comes first for a caller the visibility rule refuses; 401 only for a caller it admits; the two rows of sections 3.11 and 3.14 and test l follow

## Phase 28
- R-28-1 | CONFIRMED MEDIUM (raised HIGH) | the JSON escape of `<` in the data-block rule of section 3 is printed as a bare `<` (the escape was lost when the fixes file was generated), so the rule replaces the character by itself | the escape is written out as backslash u003c

## Phase 29
- R-29-1 | CONFIRMED LOW (raised LOW) | "every later change of a hold has its kind below" is not true of the alert link | the sentence names the state changes and the phase 14 kind that covers the link
- R-29-2 | CONFIRMED MEDIUM (raised MEDIUM) | a rail can get its first mode with no fail-policy row, and step 7 then finds no row | the first mode proposal needs the rail's wildcard row in the current matrix (422 `wildcard_row_missing`)
- R-29-3 | CONFIRMED LOW (raised LOW) | section 4.5 item 2 says 17 event kinds; there are 19 | "19"
- R-29-4 | CONFIRMED LOW (raised LOW) | "as API section 5 says" points at the codegen section | "as the API.md row of this route says"
- R-29-5 | CONFIRMED LOW (raised HIGH) | section 3.6 says `set_version = previous + 1`, which reads as a counter of its own; `activate` uses the global counter, so the reconstruction rule of 7.x holds | section 3.6 names the global counter and says numbers may be skipped

## Phase 30
- R-30-1 | CONFIRMED MEDIUM (raised HIGH) | the decision rule names two defaults when both candidates qualify | CloudNativePG becomes the default only when Patroni does not qualify; when both qualify Patroni stays
- R-30-2 | CONFIRMED LOW (raised MEDIUM) | `record_system` returns `None` on an overlap, yet both callers print the id of the row found | return `(BlindPeriod, bool)`: the row and whether this call wrote it
- R-30-3 | CONFIRMED LOW (raised MEDIUM) | section 2 says the command writes through "the `akr-ctl` database role of phase 00", which does not exist; block 7.6 says `akr_app` | section 2 says `akr_app`
- R-30-4 | CONFIRMED LOW | pointer: phase 11 section 1.1 defines the table, section 3.6 the anchor rule | pointer
- R-30-5 | CONFIRMED MEDIUM (raised MEDIUM) | the rotation runbook names `akr-ctl assess keys rotate`, a command no phase defines | `akr-ctl assess keys generate --purpose decision` (phase 29)
- R-30-6 | CONFIRMED LOW (raised LOW) | "the signed release bundle is already on offline media (section 3.11)"; section 3.11 says no such thing | the operators add the bundle of the running release to the same media
- R-30-7 | CONFIRMED MEDIUM (raised LOW) | "it is encrypted, phase 00": the phase 00 `pgbackrest` row states no encryption, though SP-84 asks for an encrypted repository | the phase 00 row states the pgBackRest cipher and the passphrase held off the server
- R-30-8 | CONFIRMED LOW (raised LOW) | "the key ceremony of phase 00" rotates nothing: the ceremony runbook creates the transit keys | the transit keys are rotated in OpenBao by the operators of that runbook
- R-30-9 | CONFIRMED LOW (raised MEDIUM) | section 3.9 still says "the immutable or offline copy" | "the offline copy (section 7.7)"
- R-30-10 | CONFIRMED MEDIUM (raised MEDIUM) | the restore pods' egress to the pgBackRest repository has no address in `zones.yaml`, and the rule names no pod selector | `zones.yaml` gains the entry `backup.repository`; the allows select every pod of `akr-restore`

## Phase 31
- R-31-1 | CONFIRMED MEDIUM (raised HIGH) | the e-mail mismatch count sits under the item for offboarded accounts, so a run with no such account would not record an e-mail mismatch of an active account | item 3 audits when either count is above zero; `email_count` covers every linked row
