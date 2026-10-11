# Owner-decision round: lead verdicts

The owner overturned seven scope decisions on 2026-10-10 (DR-414 to DR-420). The lead wrote the changes into ten phase
files. Round 1: one Haiku agent per changed phase read the new text (prompt tools/agent/p-owner.txt; input
tools/agent/verify/owner-diff-NN.txt; output tools/agent/verify/owner-NN.md). The lead decides every finding against
the phase text. Severity: HIGH = blocks building or makes a stated check fail; MEDIUM = one missing name, step,
refusal code or column, a wrong count, or a second line that still states the old rule; LOW = pointer, wording,
stale base text, field list or type.

Format: id | verdict | severity | what was wrong | fix.

## Round 1

### Phase 18 (raised 1; confirmed 1; rejected 0)
- O-18-1 | CONFIRMED | LOW | 7.29 says "section 3.14 step 5", but section 3.14 has a step 5 under each of its two routes and the first one lists cases | name the route: "step 5 of `GET /parties/{id}/transactions`".
- Agent note, not counted (clipboard on a suspicion-tier party): the copied value is a UUID the caller already holds and names no person; one sentence is added to 7.29 so nobody has to guess.
- Agent note, not counted (DR-416 rationale "copying the hashes"): the sentence is about the register, which copies the keyed hashes from canonical rows on the server (phase 13 section 7.17 item 1); it is not about the W5 copy actions. No change.

### Phase 02 (raised 0)
- FINDINGS: 0. The one changed place (the owner-kind row `goaml_corpus_case` of section 7.1) passed checks 1 to 6.
- Agent notes, not counted: three remarks on text the change did not touch (the `lang` of a generated object, the 7.10 sentence on the registry, the sniff rule for XML). No change.

### Phase 08 (raised 9; confirmed 8; rejected 1)
- O-08-1 | CONFIRMED | MEDIUM (agent: HIGH) | 7.11 items 3 and 4 link the maker to the change request on G10, but G10 lists no A2 and phase 04 gives A2 no scope for `GET /change-requests/{id}`, so an A2 maker lands on X6; phase 07 section 7.6 item 7 says only "its maker withdraws". Nothing blocks building and no stated check fails, so MEDIUM | 7.11: the G10 link is rendered for a role G10 lists (M); an A2 maker sees the reference as text and follows the request in the inbox tab "Waiting on others" (X3). Phase 07 item 7: an unmet case is closed by the approver's reject or the maker's withdraw (the Closed hook serves both); an A2 maker has no withdraw screen in P0, the same as the SS maker of a good-guy rule (phase 14).
- O-08-2 | CONFIRMED | MEDIUM | 7.11 item 3 gives the key rule without "starts with a lower-case letter" (phase 07 column check `^[a-z][a-z0-9-]{2,60}$`) | state the full rule in item 3.
- O-08-3 | CONFIRMED | MEDIUM | the two expected-outcome words have no key and no Arabic in item 5, while every other stored value has | add `corpus_case.expect.would_reject` and `corpus_case.expect.graded` to the table; items 2 and 3 name the keys.
- O-08-4 | CONFIRMED | LOW | the 7.9 paragraph kept the whole old text after "The text of the verification pass was:", old label rule included | keep only what stands (which row is the original, which the transliteration, no second line without a Latin row).
- O-08-5 | CONFIRMED | LOW | "the muted small style of DESIGN-SYSTEM 7.6": 7.6 says only "smaller" and defines no muted style | "smaller, as DESIGN-SYSTEM 7.6 says, in the secondary text colour (`--color-stone`)".
- O-08-6 | CONFIRMED | LOW | "Phase 14's back-transliteration ... is never shown under a name" reads as a claim about phase 14, which does not say it | state it as the rule of this component: `Name` shows only a Latin row the source gave and never a back-transliteration.
- O-08-7 | CONFIRMED | LOW | the add form does not send `profile_code` | item 3 sends the profile code the register is filtered to (`IQ` when no filter is set); item 1 says the same.
- O-08-8 | CONFIRMED | LOW | `FileDrop` checks size before upload, but no response carries `goaml.corpus_case_max_bytes` | item 3: the client checks the type only; the size is the server's 422 `file_too_large`, shown at the file field.
- O-08-9 | REJECTED | - | the agent wanted the panel on `/goaml-check/profiles/$id`. The version list is the register route `/goaml-check/profiles` (section 4 row E1p: REGISTER + detail; section 5: "E1p: versions, version detail"); `$id` is one version, where item 6 shows the corpus result | no change beyond the wording of O-08-7.

### Phase 19 (findings 1 to 18, decided while the agent was still writing; later ones are appended below)
- O-19-1 | CONFIRMED | LOW (agent: MEDIUM) | 7.21 item 1 says "owner `akr_app`"; tables are owned by `akr_migrate` and written by `akr_app` (phase 00) | "owner `akr_migrate`; written by `akr_app`". The same words in phase 13 section 7.17 item 1 are fixed with it.
- O-19-2 | CONFIRMED | MEDIUM (agent: HIGH) | the check "As `akr_app`: DELETE ... fails with the guard message" cannot give that message: `akr_app` has no DELETE grant, and PostgreSQL refuses on the privilege before a trigger runs | as `akr_app` the DELETE fails with permission denied; as the owner `akr_migrate` it fails with the guard message. LEAD FIND: the same wording stands in other blocks and in base text; all are fixed together (see "Lead finds").
- O-19-3 | CONFIRMED | HIGH | `PrefillMapChange` has no `reason_code` and `reason_text`, which phase 04 `create` requires, so the stated curl check (201) would get 422 | the DTO gains both fields; the curl body carries them. Same fix for `SourceTagChange` of phase 13 section 7.17.
- O-19-4 | CONFIRMED | MEDIUM (agent: HIGH) | item 6 step 5 does not say that the request is submitted. Phase 04 fixes the rule for every dedicated route (`create(origin = Dedicated)` with `submit = true`), so a developer has it, but the step must say it | step 5 names `ChangeRequestService::create` with origin Dedicated, so `submit` runs in the same transaction. Same fix in phase 13 section 7.17 item 6.
- O-19-5 | CONFIRMED | LOW | `ChangeRequestAccepted` is not a phase 04 type | the response is the phase 04 `ChangeRequestView`. Same fix in phase 13 section 7.17 item 5.
- O-19-6 | CONFIRMED | MEDIUM | six new sub-codes are missing from the closed list of section 3.11 | add them to the list.
- O-19-7 | CONFIRMED | LOW (agent: MEDIUM) | the note under the Cedar file still says every action of CFG and CHK is denied | add the exception for the two `rfi_prefill_map.*` actions.
- O-19-8 | CONFIRMED | HIGH | the source `party.pep_status` answers from the CBS-derived column `canon.party.pep_status`, which phase 15 says is not the PEP state (DR-38); the bank's PEP register (`kyc.party_pep_state`) is ignored, so a PEP the bank confirmed could be proposed as "unknown" or "no" | the source reads the register first: `pep` -> yes; `former_pep` -> yes, marked former; `candidate` -> unknown (an unconfirmed lead is never stated); no entry -> the customer-file value (`pep` yes, `not_pep` no, else unknown). New small reader over the view (SELECT is granted to `akr_app` by phase 15). Item 10 and the test of item 11 are reworded.
- O-19-9 | CONFIRMED | LOW | `insert` returns `()`, while the phase rule is `RETURNING` on every insert | `insert` returns the row.
- O-19-10 | CONFIRMED | LOW (agent: MEDIUM) | step 3 demands a purpose at every tier but points to phase 02, whose rule covers restricted and suspicion only | say it is a rule of this route at every tier, as the profile read of phase 13.
- O-19-11 | CONFIRMED | LOW (agent: MEDIUM) | the curl example maps a question key that no fixture defines (templates and question keys are pack content, Open doubts 2) | the curl check names the question as one of the correspondent template of the suite's fixture pack.
- O-19-12 | CONFIRMED | LOW | item 8 says "one more row of section 3.12" and the table has no such row | add the row to the table.
- O-19-13 | CONFIRMED | LOW | `domain::rfi::prefill` does not follow the crate rule of the phase (`domain::casework`) | `domain::casework::prefill`.
- O-19-14 | CONFIRMED | LOW | "the bank's own view" names two different things in two lines | the second becomes "the respondent's comfort with the relationship (question 22 of the Wolfsberg list)".
- O-19-15 | CONFIRMED | LOW | the 5.2 heading reads as cut off ("but for the panel ...") | "but P1 for the panel Proposed answers of section 7.21".
- O-19-16 | CONFIRMED | MEDIUM (agent: HIGH) | three sources are read "through `CanonPartyReader`", which has no method for the customer rows or for the other holders of an account | the block adds two reader methods and names them in the table.
- O-19-17 | CONFIRMED | HIGH | the source `activity.declared_and_actual` reads the phase 13 profile, which the RFI desk may not read (`profile.read` is A1, A2, KYC and M; the profile is restricted data with fixed purposes) | the source becomes sensitive: computed for the MLRO only, under `profile.read` and the purpose `investigation`, with the access events phase 13 writes; the desk sees "Needs the MLRO". No change to phase 13.
- O-19-18 | CONFIRMED | LOW (agent: MEDIUM) | the D3 tab is said to be read by RFI, but D3 is the studio of CFG and CHK, read by M, CH and IA (phase 09) | the tab is read by CHK and M; the desk reads the map through the API on W8.

### Phase 10 (raised 7; confirmed 7, one of them the same defect as O-26-1; rejected 0)
- O-10-1 | CONFIRMED | MEDIUM (agent: HIGH) | 7.26 item 5 says a change of `capture_source` changes the `row_hash`. The hash is the hash of the landed source row (phase 11 section 3.2 step 2), so a mapping edit that changes only the literal changes no stored name. No stated test fails (the tests are first writes) | item 5: a stored name changes when the bank's column behind the value changes; a mapping edit alone reaches stored names only when the rows are mapped again (the mapping replay of phase 11 section 3.8). Item 7 reworded: the value is set by the mapping, which changes only through its change path.
- O-10-2 | CONFIRMED | LOW (agent: HIGH) | the agent read "the writer addresses a table by its enum value" as: the table must be named `canon.facility_instalment`. The catalogue entry of a pack entity names its table (phase 26 section 3.2 item 2: "table, columns, classification, nullability"), as it does for `islamic_deal` -> `canon.islamic_deals` in verified text, so nothing fails; but 7.27 did not say it | 7.27 item 1 says that the catalogue entry names the table `canon.facility_instalments`.
- O-10-3 | CONFIRMED | MEDIUM | the writer tests the stored `paid_on` and `granted_on`, but `CurrentRow` (section 7.17) does not carry them | 7.27 states the fields `CurrentRow` holds for the two entities; 7.17 points to it.
- O-10-4 | CONFIRMED | same defect as O-26-1 | see O-26-1 | see O-26-1.
- O-10-5 | CONFIRMED | MEDIUM | 7.27 (and verified 7.20) say phase 11 records a `parent_missing` conflict, but the reason list of phase 11 section 3.2 step 5 and the mapping of phase 12 section 3.3 step 1 do not name it | phase 11 step 5 gains the reason `parent_missing`; phase 12 maps it to the issue kind `unresolved_pointer` (medium). Two small base corrections outside the ten changed phases; both go into round 2.
- O-10-6 | CONFIRMED | LOW | item 5 cites DSL.md B.5 for the replacement by `typed`; B.5 only says the row loads, and a failing value would then hit the column CHECK | item 5 states the replacement as this block's rule, done in the mapped row before the write.
- O-10-7 | CONFIRMED | MEDIUM | the column is NOT NULL; an `expr` that gives null, or a writer that inserts every catalogue column, would be refused by PostgreSQL | item 5: the writer writes `typed` when the mapped row has no value (absent or null); it does not rely on the column default.
- Lead note, not counted (outside the round): phase 11 section 3.8 says the replay apply opens new versions of changed values while the writer of phase 10 compares the landed-row hash only. How apply detects a changed mapped value is a point for the build of phases 10 and 11; it is listed in the report.

### Phase 26 (raised 12; confirmed 12, one of them in part; rejected 0)
- O-26-1 | CONFIRMED | HIGH | `uq_facility_instalments_seq UNIQUE (facility_id, seq)` is not the insert target of the fact write, so a second source key for the same instalment number (a rescheduled loan) raises a unique violation and rolls back the batch | the constraint becomes a plain index on `(facility_id, seq)`; 7.13 says why and that the bank's mapping leaves out superseded schedule rows. Both SQL copies and DATA-MODEL are changed.
- O-26-2 | CONFIRMED IN PART | HIGH | (a) rejected: the schedule inputs are optional (`?`), and phase 06 section 3.2 rule 4 gives an optional need no effect on the grade, so an agreement-only bank is not graded down. (b) confirmed: for the same reason "B until the family is published" cannot follow, and the stated test "grades B" would fail | the 3.6 cell, item 5 and the test of item 8 say what the grader really does: the unmapped optional inputs are listed in `reasons`, the grade does not change, and the facility legs are not evaluable until the inputs are published.
- O-26-3 | CONFIRMED | MEDIUM | `paid_amount` has no catalogue flag, and a part payment that carries it with a null `paid_on` violates `CHECK (paid_amount IS NULL OR paid_on IS NOT NULL)` | the CHECK is dropped: `paid_amount` is the amount paid so far, optional, neither `required` nor `monitored`. The writer rule (phase 10 section 7.27 item 3) is restated: an instalment may change in `paid_on` and `paid_amount` while its stored `paid_on` is null; any other change, or a change after it is paid, is a `RowConflict`.
- O-26-4 | CONFIRMED | MEDIUM | section 6 check 2 names three tables, so it does not cover the new table and column that item 8 says it covers | check 2 names `canon.facility_instalments` and `canon.facility.granted_on` too.
- O-26-5 | CONFIRMED | MEDIUM | check 5 says "the two new entities" | "the three new entities and `facility.granted_on`".
- O-26-6 | CONFIRMED | MEDIUM | section 3.2 items 1 and 2 still count two families and two entries | each gains the pointer to the third (section 7.13).
- O-26-7 | CONFIRMED | MEDIUM | section 1 and the trailer still say `canon.facility` is not changed | both name the added column.
- O-26-8 | CONFIRMED | LOW (agent: MEDIUM) | the note of section 1.2 says "the two tables" | "the three tables and the column `granted_on` of `canon.facility`".
- O-26-9 | CONFIRMED | MEDIUM | `not_evaluable_count` is used as if it existed | 7.13 item 4 introduces it: a measure alias of the SCN-ISL-07 document, an integer in the `measures` of the evaluation (phase 16).
- O-26-10 | CONFIRMED | LOW | "1 altered column list" against the header's "1 column added" | "1 column added".
- O-26-11 | CONFIRMED | LOW | "names which loan was not evaluable only by its count" contradicts itself | the evidence gives only the count.
- O-26-12 | CONFIRMED | LOW | "a later version that only sets `paid_on`" does not match the writer rule | restated with the rule of O-26-3.
- O-19-19 | CONFIRMED | LOW | "its current primary name": no column marks a primary name | the current `legal` name (`name_type = 'legal'`, `superseded_at IS NULL`), the Arabic or Kurdish form first.
- O-19-20 | CONFIRMED | HIGH | the flag `incomplete` of `accounts.signatories` tells the desk (and, through the answer, another bank) that a hidden party holds the account; the phase rule is "no count and no placeholder" (section 3.14) | the source becomes sensitive (names of other persons): computed for the MLRO only, a party the caller may not see is left out with no flag, and the flag is removed. Item 10 says so.
- Phase 19 total: raised 20; confirmed 20; rejected 0. By the lead's scale: HIGH 4 (O-19-3, O-19-8, O-19-17, O-19-20), MEDIUM 4 (O-19-2, O-19-4, O-19-6, O-19-16), LOW 12.

### Phase 14 (raised 15; confirmed 15; rejected 0)
- O-14-1 | CONFIRMED | MEDIUM | section 4.2 still counts 25 operations, 17 of them `screening` | 27 and 19.
- O-14-2 | CONFIRMED | MEDIUM | section 6 item 1 checks "the ten new tables" | eleven, naming `alert.harness_cases`.
- O-14-3 | CONFIRMED | MEDIUM | the ungoverned list of section 4.3 (a copy of 1.5) lacks `screening.harness_cases_max` | add it.
- O-14-4 | CONFIRMED | MEDIUM | section 4.5 says phase 04 gets three change targets | four, naming `harness_case`.
- O-14-5 | CONFIRMED | LOW (agent: MEDIUM) | section 7.16 says `IdentifierInput` is never stored; 7.19 item 3 states the exception, 7.16 does not | 7.16 gains the pointer to the exception.
- O-14-6 | CONFIRMED | MEDIUM | the S3 tab shows a per-case result of the latest run, which no response carries (`report.bank_cases` holds only the unmet ids) | the report holds `results: [{case_id, result}]` (`met`, `unmet`, `absent`) in place of `unmet_ids`; the tab joins it by case id from the harness result S3 already loads, and shows the trial result for a case with no run yet.
- O-14-7 | CONFIRMED | LOW (agent: MEDIUM) | the curl check names no list and no entry key | `list_code: "iq_national"` and an entry key of the fixture version of section 6, after that version is active.
- O-14-8 | CONFIRMED | LOW | "hash the identifiers as a search does (section 3.12 step 3)": that step normalises the name | "(section 7.16)".
- O-14-9 | CONFIRMED | LOW | the bundle tests leave out CH | add CH.
- O-14-10 | CONFIRMED | LOW | the tipping-off review does not say that the stored request body holds the entry key, the sealed query and its hash, nor that `reason_text` is plain text | item 12 says both; the form carries the hint to write no name in the reason.
- O-14-11 | CONFIRMED | LOW | "each list the change touches" is not defined for `screening_policy` | every enabled list for `screening_policy`; the list of the key for `list_setting`.
- O-14-12 | CONFIRMED | LOW | "`akr_readonly` gets no grant, as for the alert tables": two alert tables do grant it | name the comparable tables.
- O-14-13 | CONFIRMED | LOW | the DELETE check names no role; as `akr_app` it would give permission denied, not the guard message | part of the lead find on DELETE checks (see "Lead finds").
- O-14-14 | CONFIRMED | LOW | "a fourth row of the section 3.10 table": that table keeps three rows | "a fourth target".
- O-14-15 | CONFIRMED | LOW | no line says that `retire` and `reject` write `closed_at` | item 3 states what `activate`, `retire` and `reject` set.

### Phase 07 (raised 11; confirmed 11; rejected 0)
- O-07-1 | CONFIRMED | HIGH | `check_before_decision` enqueues `goaml_case_intake` and then issues `intake_stale`; the issue is a refusal, phase 04 rolls the action back, and a job enqueued in a rolled-back transaction leaves no row, so the stated check "then approval succeeds" cannot pass | the re-run is enqueued where it commits: the Activate of the `goaml_profile` target enqueues `goaml_case_intake` for every `proposed` case of the code; `check_before_decision` only issues `intake_stale`. (Verify phase 04 line ~1649 and phase 00 line ~788 when fixing.)
- O-07-2 | CONFIRMED | HIGH | the curl check expects 403 `E_SCOPE` for DS on the GET, but DS holds `config.author`, which the route accepts, so Cedar answers 403 `E_FORBIDDEN` | "as DS the GET -> 403 `E_FORBIDDEN`; as SS the GET -> 403 `E_SCOPE`". (Verify docs/API.md line ~95 when fixing.)
- O-07-3 | CONFIRMED | MEDIUM | the curl sequence approves before the intake job can have run | "after the intake job has completed (poll the GET until `intake_result` is set)".
- O-07-4 | CONFIRMED | MEDIUM | a case withdrawn before its intake keeps the uploaded file until the sweep | job step 1: a state other than `proposed` deletes the transient object and ends.
- O-07-5 | CONFIRMED | MEDIUM | the deletion of the uploaded file has no audit event, while the same deletion for a check run has `goaml_check.transient_deleted` | new kind `goaml_corpus_case.transient_deleted {case_id, by}` (seven kinds, not six), written after each `delete_transient` and by the sweep.
- O-07-6 | CONFIRMED | LOW (agent: MEDIUM) | "a key that names a person is refused at approval, as any reason text is" reads as a system step; none exists | the form asks for a code, not a name; the approver rejects a key that names a person; the system does not screen the key.
- O-07-7 | CONFIRMED | MEDIUM | the mask sends U+063B to U+0640 (the tatweel among them) to U+06A9, which the entity-name class of patch P-01 refuses, so a valid name can gain an XSD-04 block | "letter in U+0621 to U+064A" becomes a random letter of U+0621 to U+063A or U+0641 to U+064A.
- O-07-8 | CONFIRMED | MEDIUM | the base tipping-off review (section 3.8) still says nothing derived from an uploaded file holds a value | it gains the exception for the masked bank cases (codes, booleans, dates, the bank's own identifiers; nothing that identifies a person). Check the scope of the section 3.2 sentence (line ~572) when fixing.
- O-07-9 | CONFIRMED | LOW | Prepare accepts any `rule_code` of a rejection pattern as `office_code`; the column CHECK admits only `OFF-nn` | Prepare also requires the `OFF-nn` form (else 422 `rule_unknown` at `/office_code`).
- O-07-10 | CONFIRMED | LOW | "the target refuses and the request ends `failed`": phase 04 rolls back an immediate approval and ends `failed` only on the scheduled path | say both. The same words in phase 14 section 7.19 item 5 (and any other owner block) are fixed with it. (Verify phase 04 lines ~1023 and ~1103.)
- O-07-11 | CONFIRMED | LOW | the crate listing of section 2 still names three job kinds, one change target and three handlers | add `goaml_case_intake`, `goaml_corpus_case` and "four".
- Linked change from O-08-1: item 7 sentence "its maker withdraws the request" becomes "the approver rejects the request or its maker withdraws it".

### Lead finds (round 1, not raised by an agent in that place)
- L-1 | MEDIUM | check lines of the form "As `akr_app`: `DELETE FROM <table>` fails with `<guard message>`" cannot give the guard message where `akr_app` has no DELETE grant (PostgreSQL refuses on the privilege before any trigger). Places to check and fix: grep "never deleted" over docs/plan (owner blocks of phases 07, 13, 14, 19; base text phase 14 line ~1271, phase 09 line ~1184, phase 07 line ~906). Fix wording: as `akr_app` the DELETE fails with permission denied; as the owner `akr_migrate` it fails with the guard message.

### Phase 13 (raised 12; confirmed 12; lead scale HIGH 1: O-13-5)
- O-13-1 | CONFIRMED | MEDIUM (agent: HIGH) | the activation cell of item 7 reads as if only `void` sets `retired_at` and `retired_by_request`; the two CHECKs of item 1 need both for `ended` too. The `close` method already takes the request id for both modes, so the value is at hand; the cell must say it | cell and trait comment: `retired_at = now()` and `retired_by_request` on both; `ended` also sets `valid_to`.
- O-13-2 | CONFIRMED | MEDIUM (agent: HIGH) | the default `valid_to` of an `end` (last closed business date) falls before `valid_from` for a row whose `valid_from` is today or the open business day, which the CHECK `valid_to >= valid_from` refuses at activation | the default is the later of the row's `valid_from` and the last closed business date.
- O-13-3 | CONFIRMED | MEDIUM (agent: HIGH) | same defect as L-1 (DELETE as `akr_app` is refused on the privilege, not by the guard) | L-1 wording.
- O-13-4 | CONFIRMED | MEDIUM (agent: HIGH) | "(or `false`) is true for a customer with a row" gives `false` the meaning of `true` | `value: true` -> EXISTS; `value: false` -> NOT EXISTS over the same subquery.
- O-13-5 | CONFIRMED | HIGH | step 2 allows an add whose active row is in `retire` (the only way to change a payer name), but the activation inserts the adds first, so `uq_source_tags_active` refuses the insert every time | the activation closes the retires first and inserts the adds after, in the approval transaction.
- O-13-6 | CONFIRMED | MEDIUM | two adds of one change set that share a hash for one tag (an account and a counterparty row of the same account) pass step 2 and the second insert breaks the unique index at activation | step 2: an add that shares a hash with an earlier add of the same tag is sub-code `already_tagged` at its `/add/<i>/subject_id`.
- O-13-7 | CONFIRMED | MEDIUM | the impact lists hold readers of "a tag name of the change set", but a retire entry carries no tag, so a change set that only ends a tag a segment reads could be `neutral` | a retire entry's tag is the tag of the row it names; the impact counts those tags too.
- O-13-8 | CONFIRMED | MEDIUM | a tag change moves what a tag-reading feature gives for months already computed, and nothing recomputes them (the hourly run skips a month whose basis did not change) | the `source_tag` activation, when `reading_features` is not empty, also enqueues `feature_compute {reopen_from}` with the month-end of the month of the earliest date the change set moves (the earliest `valid_from` of an add or of a voided row, the earliest `valid_to` of an end); section 3.2 step 5 then updates only rows whose value changed. Section 4 job row names the third enqueuer.
- O-13-9 | CONFIRMED | MEDIUM | base section 4 "Only the worker reads `canon`; the API reads `feat` only" is no longer true: the proposal of item 6 reads `canon.account` and `canon.counterparty` in the API | the base line names the exception (read only, as `akr_app`, which holds the SELECT grants).
- O-13-10 | CONFIRMED | LOW | "`screens.ts` lists the tab for CFG ... and for CHK, M and CH": a D3 tab has no role list (phase 09 section 7.5) | the tab renders when the caller may call the GET; "Propose changes" when the caller may call the POST.
- O-13-11 | CONFIRMED | LOW | "two more rows of section 3.10" and "two more rows of section 3.11": the 3.11 table gained one row for both actions and the two events are named in the sentence after the 3.10 table | wording corrected.
- O-13-12 | CONFIRMED | LOW | the owner note cites section 7.7, which says only that a counterparty profile is P2 | the note cites the header entry S-17 only.
- L-2 | LEAD FIND (from the agent's note) | MEDIUM | a feature `where` is a string under the DSL.md Filter subset, and item 3 never writes how a tag name stands in it | item 3: the name is written as a comparison with a boolean literal, `tag.payroll = true` or `tag.payroll = false`, as `txn.cash_flag = true` is in the shipped catalogue; `= true` compiles to the EXISTS, `= false` to NOT EXISTS; any other use is `E_FIELD_TYPE`. The fixture feature of item 14 uses `tag.payroll = true`.
- Carried from phase 19 (already decided there): O-19-1 (owner words of item 1), O-19-3 (`reason_code`, `reason_text` in `SourceTagChange` and the curl body), O-19-4 (`ChangeRequestService::create` with origin Dedicated), O-19-5 (`ChangeRequestView`), O-19-9 (`insert` returns the row).

### Phase 25 (raised 9; confirmed 9; lead scale HIGH 1: O-25-1)
- O-25-1 | CONFIRMED | HIGH | the last CHECK of `admin.siem_dead_letters` is false for every open row: `(resolution = 'marked')` is NULL while `resolution` is NULL, and NULL IS NOT DISTINCT FROM FALSE is false. The dead-letter insert runs in the cursor transaction, so one bad line would stop the stream | both copies: `CHECK (((resolution = 'marked') IS TRUE) = (resolution_code IS NOT NULL))`.
- O-25-2 | CONFIRMED | MEDIUM | the route-count bullet of section 2 still says the phase is L | XL.
- O-25-3 | CONFIRMED | MEDIUM | the forwarder test "the events written meanwhile are not sent" fails for an event written after the last off-run (the cursor stands at that run) | item 6.1 says that such an event is sent; the test writes its events before an off-run.
- O-25-4 | CONFIRMED | MEDIUM | the trailer says `schema.cedarschema` receives the declarations of section 3.12 only; 7.24 item 3 adds three more | "of section 3.12 and of section 7.24 item 3 only".
- O-25-5 | CONFIRMED | MEDIUM | docs/API.md calls the access lines "allowed reads of sensitive routes"; item 1 exports every access event (allow, deny, conflict) | API.md row reworded; the same words are searched for in phase 25.
- O-25-6 | CONFIRMED | LOW | `on_behalf_of` is nullable in `audit.access_event` but has no `Option` in `SiemAccessLine` | `on_behalf_of: Option<Uuid>`.
- O-25-7 | CONFIRMED | LOW | resolve step 1 names the missing `If-Match` with no status | "(428 `E_PRECONDITION_REQUIRED`)".
- O-25-8 | CONFIRMED | LOW | "the three kinds that the forwarder's own work produces": one of the three is written by the request | "the three kinds of the resend work".
- O-25-9 | CONFIRMED | LOW | the tipping-off list of item 11 leaves out the access id, the decision and the on-behalf-of id | the three are added to the list.
- Agent notes, no change: the sub-codes and audit kinds that only the 7.x blocks name (the same convention as section 7.17); section 7.13's "SP-36 is partly implemented" still holds for canary access and the DBA rules.
- Process note: the phase 25 agent started a throwaway `postgres:17` container to test the CHECK and removed it afterwards, and ran one read-only `git status`. The standing rule is "Docker: never rm or prune"; the user's containers are all still listed. Round 2 prompts forbid Docker and git.

### Lead finds made while fixing phases 10 and 26 (round 1)
- L-3 | MEDIUM | `row_hash` is the hash of the landed source row, so a mapping version that only adds or changes `capture_source`, `paid_on` or `granted_on` leaves the hash equal and step 3 of the writer writes nothing; phase 11 says "a later publication does not re-map history". The three owner decisions would then not reach rows already stored | the three column rules test mapped values, not the hash: `set_capture_source` (new, with a column grant in migration 0030), `pay_instalment` (now also for part payments, `paid_on` optional) and `set_granted_on` run when the mapped value differs, also with an equal hash (re-land or the apply of a mapping replay). `CurrentRow` carries the four columns.
- L-3 note (base text, outside the owner blocks): the apply of phase 11 section 3.8 runs the writer, whose guard compares the landed hash, so read literally an apply changes nothing. Recorded as phase 11 Open doubts 14 for the build of phases 10 and 11; reported to the user. Not fixed in this round: it needs a design for value comparison in the apply.
- O-26-2 final wording (differs from the first verdict line above): the cell is "readiness A (agreement source) or B (schedule source)", as the verified cell of SCN-ISL-06 and as the SCN-ISL-07 row read before the owner change; the `?` inputs are optional and never change the grade. The test of item 8 now checks only what phase 06 rule 4 guarantees (listed in `reasons`, same grade).
- O-26-1 addition: Open doubts 24 of phase 26 states the limit for a source that replaces or deletes schedule rows (rows already loaded stay unpaid in `canon`). No column was added: no schedule table has been studied.
- O-10-3 final design: `CurrentRow` holds `paid_on` and `paid_amount` for an instalment and `granted_on` for a facility (not `seq`, `due_on`, `amount`, `currency`): the pay rule depends only on the two payment columns.

### Lead finds made while fixing phase 14 (round 1)
- L-4 | LOW | section 7.19 item 1 says a trial outcome can be `absent`, but item 4 step 3 refuses a key that the active version does not hold, so a trial is never `absent` | the sentence now says so; `absent` is a result of a later harness run only.
- O-14-4 addition | LOW | the crate listing said "the eleven tables of section 1" (base text; section 1 has ten) | "the ten tables of section 1 and alert.harness_cases (section 7.19)".
- L-1 in phase 14: the base check line of section 6 item 2 was wrong for two statements (`DELETE FROM ref.list_versions`, `UPDATE alert.screening_decisions`): `akr_app` has neither privilege. Both now say permission denied as `akr_app` and the guard message as the owner `akr_migrate`.

### Lead finds made while fixing phase 07 (round 1)
- O-07-1 final design | the profile activation enqueues the intake only for `proposed` cases that have a stored file (new repository method `stored_proposed_for_code`): a case whose first intake was not met has no file left, and a re-run would only replace "not met" by the refusal `file_gone`. A case whose first intake is still queued runs under the new active version anyway (job step 2 reads the active version when it runs).
- O-07-4 and O-07-5 final design | the event `goaml_corpus_case.transient_deleted {case_id, by}` has `by` = `intake` or `sweep`. Job step 1 deletes and writes it only for a case with no `intake_result` (the only case in which a file can still be staged). The failed proposal (item 5 step 5) writes none, as a failed check request writes none.
- L-5 | LOW | the tipping-off review said that nothing personal stays in a masked file; dates and date-times stay, a date of birth among them, and so do enumeration codes (nationality, gender), because the rules read them | the review of item 12 and the base review now say so plainly. No route returns the file; it is `secret-str` evidence. For the user report: this is what the mask keeps.
- L-1 in phase 07: base check of section 6 item 1 (`DELETE FROM rpt.goaml_profiles`) now names the owner role; the block's DELETE check names both roles.
- O-07-6 addition | phase 08 section 7.11 item 3 gains the same hint at the key field.

### Lead finds made while fixing phase 13 (round 1)
- O-13-8 final design | the activation enqueues `feature_compute {reopen_from}` under the coalesce key `feature_compute:reopen:<reopen_from>`: under the plain key `feature_compute` a queued hourly run swallows the enqueue (phase 00 `ON CONFLICT DO NOTHING`) and the reopen is lost. The reopened months are computed again whatever their `basis` says. The replay enqueue of base section 3.2 has the same weakness; it is written down as a new entry of phase 13 "Open doubts" and not changed (base text; for the user report as a build point).
- L-6 | LOW | the base proposal DTOs of phase 13 (`NewFeatureDefinition`, `NewSegment`) do not name `reason_code` and `reason_text`, which phase 04 `create` requires (the same gap as O-19-3 in base text) | one sentence added to the DTO list of section 2.
- L-1 in phase 13: the block's check now gives permission denied for `akr_app` and the guard message for the owner.
- Process slip, corrected at once: the first wording of the curl fix put a 422 check in the middle of the 201 sentence; the sentence was restored and the 422 check moved beside the other 422 checks (the log entry was corrected to the final text).

- O-19-5 in phase 13, corrected by the lead while fixing: phase 13 section 2 defines `ChangeRequestAccepted` itself (the response of its two base proposal routes), so the source-tag POST keeps that schema and the block now says where it is defined. The change to `ChangeRequestView` holds for phase 19 only, where the block had called it a phase 04 type.

### Lead notes made while fixing phase 19 (round 1)
- O-19-8 final design | new port `PartyPepReader::pep_state(conn, party_id)` over the view `kyc.party_pep_state`; the view's fourth state `none` (a former entry whose persistence has passed) is treated as "no row", so the customer file decides. Facts are `{ answer, former? }`.
- O-19-17 final design | `ProfileQuery` is called with the caller's context and purpose code; for the MLRO a purpose other than `investigation` gives `withheld` for that one proposal, and the panel says "Choose the purpose Investigation to see this".
- O-19-18 final wording | the D3 tab renders for a D3 user who may call the GET (CFG, CHK, M); the desk keeps its Cedar permit on `rfi_prefill_map.read` and reads the map on W8.
- System documents in step: MODULES.md ("four of them for the MLRO only"), DESIGN-SYSTEM.md (D3 tab readers; the W8 note), research.md DR-417 row, phase 18 section 2 pointer to the two new reader methods.

### Lead notes made while fixing phase 25 (round 1)
- O-25-3 final wording | item 6.1 now says: an event recorded before an off-run is never sent; one recorded after the last off-run (at most 10 seconds before the switch back) is sent. The test writes two events, lets one off-run pass, switches on, and expects them not sent, while a later event is sent.
- O-25-5 addition | the same words stood in docs/MODULES.md and docs/DATA-MODEL.md ("allowed reads of sensitive routes are exported too"); both now say every access event (allow, deny, conflict). Phase 25 line "The owner's decision: allowed reads go to the bank's SIEM too" is the decision as the owner made it and stays; item 1 says what is exported.
- L-7 | LOW | base struct `SiemLine` (section 2) had `actor_id` and `on_behalf_of` with no `Option`; both columns of `audit.audit_log` are nullable (phase 02) | `actor_id: Option<Uuid>, on_behalf_of: Option<Uuid>`. In `SiemAccessLine` only `on_behalf_of` is optional (`access_event.actor_id` is NOT NULL).
- System documents in step: DATA-MODEL.md names `capture_source` of `party_name` among the columns `akr_ingest` may UPDATE (DR-420) and the UPDATE grant on `canon.facility (granted_on, row_hash)` (DR-419).

### L-1 sweep over the other phases (round 1)
Method: every check line that names `akr_app` (or "the owner role too") together with a guard message was read against the GRANT lines of its own phase. Where `akr_app` lacks the privilege, the line now says permission denied for `akr_app` and the guard message for the owner `akr_migrate`. Script: tools/agent/ofix-r1-L1.py (10 fixes).
- Phase 06 section 6 item 1: `DELETE FROM scn.cbi_catalogue_items` (akr_app: SELECT, INSERT, UPDATE of two columns).
- Phase 09 section 6 item 1: `DELETE FROM ref.countries`, `UPDATE ref.fx_rates`, `UPDATE admin.policy_packs` (akr_app: no DELETE on `ref`; SELECT, INSERT only on the two others).
- Phase 15 section 6 item 2: `UPDATE kyc.ratings` (akr_app: SELECT, INSERT only).
- Phase 17 section 6 item 11: the `case_events` payload change (no UPDATE grant; item 12 already said so); the superuser variant now says the append-only trigger is disabled for that statement, or it could not run.
- Phase 18 section 6 item 9: the five append-only casework tables (akr_app: SELECT, INSERT only).
- Phase 20 section 6 item 5: `UPDATE casework.ops_tasks SET task_ref` (`task_ref` is not in the column grant).
- Phase 21 section 6 item 10: `UPDATE rpt.filings SET goaml_reference` (not in the column grant) and `DELETE FROM rpt.fiu_feedback` (no DELETE grant); the message is `fiu_feedback is append only` (TG_TABLE_NAME has no schema name).
- Read and left as they are: phase 02 (already says permission denied), phase 04 line 1554 (says permission), phase 12 (says which columns are granted), phase 16 item 8 (`insufficient_privilege` is the SQLSTATE of both refusals; `document` is in the column grant, so the guard is reached), phase 15 `rating_models` (the column is granted, the guard is reached), phase 20 and 21 lines whose column is granted, phase 25 lines that name the guard and no role, phase 26 block (says no column grant).

### Round 1 totals (recounted from the lines above on 2026-10-11)
- Agents: 10 (phases 02, 07, 08, 10, 13, 14, 18, 19, 25, 26). Raised 96; confirmed 95 (one in part: O-26-2; one is the same defect as another: O-10-4 = O-26-1); rejected 1 (O-08-9).
- Confirmed, by the lead's scale: HIGH 10 (O-07-1, O-07-2, O-13-5, O-19-3, O-19-8, O-19-17, O-19-20, O-25-1, O-26-1, O-26-2), MEDIUM 39, LOW 45, and 1 counted with O-26-1.
- Per phase (HIGH/MEDIUM/LOW): 07 2/5/4; 08 0/3/5 (1 rejected); 10 0/4/2 (and 1 with O-26-1); 13 1/8/3; 14 0/5/10; 18 0/0/1; 19 4/4/12; 25 1/4/4; 26 2/6/4; 02 none.
- Lead finds: 7 (L-1 to L-7): MEDIUM 3 (L-1 check lines and privileges, L-2 tag syntax in a feature filter, L-3 mapped-value writer rules), LOW 4 (L-4, L-5, L-6, L-7).
- Fixes: 208 replacements logged in tools/agent/owner-fixes.txt (tag R1), made by the scripts tools/agent/ofix-r1-*.py. DR-422 locks the design corrections.
- Checks after the fixes: idcheck "DR defined: 422 problems: 0"; routecheck "API.md rows 595; owned 595; not owned 0; phase routes absent from API.md 5"; coverage "baseline 1208; carried 940; uncarried 268 (0 named nowhere)".

## Round 2

Round 2 checks the 208 replacements of round 1. Sixteen Haiku agents, one per key (prompt tools/agent/p-owner2.txt; input tools/agent/verify/owner2-fixes-<KEY>.txt, written by tools/agent/mk-owner2.py; output tools/agent/verify/owner2-<KEY>.md). Keys: 07a, 07b, 08, 10, 11 (phases 11 and 12), 13a, 13b, 14a, 14b, 18, 19a, 19b, 25, 26, DR (the DR-422 row), L1 (the check-line sweep of phases 06, 09, 15, 17, 20, 21). Finding ids are P-<KEY>-k.

### Key 18 (4 entries; raised 0)
- FINDINGS: 0. F1 to F4 passed.

### Key L1 (9 entries; raised 0)
- FINDINGS: 0. The nine check-line corrections of phases 06, 09, 15, 17, 20 and 21 agree with the GRANT lines and the trigger messages.

### Key 13a (16 entries; raised 0)
- FINDINGS: 0. F1 to F16 passed.
- Agent note, taken as a lead fix for the round 2 script (LOW, L2-1): docs/plan/phase-13.md line ~1071, SQL comment "Owner akr_app" for feat.counterparty_links (gap pass 1 text); the owner is `akr_migrate`, as O-19-1 corrected in section 7.17. To fix when no agent is reading: "Owner akr_migrate; written by akr_app" (check the grant lines first). Then grep "Owner akr_app\|owner `akr_app`" over docs/plan for the same words.

### Key 14b (13 entries; raised 1; confirmed 1)
- P-14b-1 | CONFIRMED | LOW | the pointer "phase 04 section 3.2" in the Activate row: the rollback of an immediate approval is phase 04 section 3.4 (line ~1023) and the scheduled failure is the `cr_activate` row of section 3.6 (line ~1103) | "(phase 04: an immediate approval is rolled back with the error, section 3.4; a scheduled activation ends `failed`, section 3.6)". The same words stand in phase 07 section 7.6 (line ~1193) and are fixed with it.

### Key 19a (19 entries; raised 0)
- FINDINGS: 0. F1 to F19 passed.

### Key 14a (13 entries; raised 0)
- FINDINGS: 0. F1 to F13 passed.

### Key 07a (16 entries; raised 1; confirmed 1)
- P-07a-1 | CONFIRMED | MEDIUM (agent: HIGH) | job step 1 of section 7.6 item 6 reads "A state other than `proposed` -> done; when the case has no `intake_result` ..., first `delete_transient`": the delete clause can be read as a rule for every case, a `proposed` first run among them, and "done" stands before the delete. The parenthesis gives the intended meaning, so no stated check fails under it, but a step must not need the parenthesis to be read right | step 1 is written in order: when the state is not `proposed`, a case with no `intake_result` first has its staged file deleted and the event written, then the job is done; a `proposed` case goes on with step 2.

### Key 11 (4 entries in phases 11 and 12; raised 3; confirmed 3)
- P-11-1 | CONFIRMED | MEDIUM | phase 12 section 3.3 step 1 now maps four conflict reasons, but the value object `ConflictKind` (section 2, line ~472) lists three and the section 6 test (line ~1226) counts "the three `ConflictKind` values" | `ConflictKind` gains `ParentMissing`; the test says four.
- P-11-2 | CONFIRMED | LOW | the opening of phase 12 section 3.3 says a conflict "disagrees with a settled one, so it is never retried"; a `parent_missing` row disagrees with nothing and is met again by the trailing re-read window (phase 10 section 7.20) | the sentence names the exception.
- P-11-3 | CONFIRMED | LOW | the table comment of `quarantine_items` in phase 11 section 1.1 (line ~175) describes `row_conflict` rows only as rows whose canonical value differs from a settled one. It is a SQL comment, which the round's rules do not count; it is copied into the migration, so it is brought in step | the comment names the parent case too.

### Key 25 (15 entries in phase 25, API.md, MODULES.md, DATA-MODEL.md; raised 0)
- FINDINGS: 0. F1 to F15 passed, the corrected CHECK of `admin.siem_dead_letters` among them.

### Key 10 (16 entries in phase 10 and DATA-MODEL.md; raised 2; confirmed 2)
- P-10-1 | CONFIRMED | MEDIUM | section 7.26 item 5 ends "takes a new label ... at the next load of its source row, or in the apply of a mapping replay". Phase 11 lands only changed rows (section 3.2 step 2: an unchanged row is not landed), so for an unchanged source row the next load does nothing and only the replay reaches the name | the sentence says both cases: a changed source row lands again and its name row is replaced with the mapped label; an unchanged source row is not landed again, so a mapping edit alone reaches it only in the apply of a mapping replay (phase 11 section 3.8).
- P-10-2 | CONFIRMED | LOW | the DR-422 sentence of section 7.27 was appended after "This phase's parts:", so the colon no longer leads to the list | the sentence moves before "This phase's parts:".

### Key 13b (15 entries; raised 4; confirmed 4; one lead find)
- P-13b-1 | CONFIRMED | MEDIUM (agent: HIGH) | the Network zones line of section 4 (line ~934) lets the API read `canon.account` and `canon.counterparty` "for the source-tag proposal of section 7.17 item 6 only", but item 7 runs `check_before_decision` (steps 2 and 3 again) and the activation ("hashes resolved again") in the approval transaction, which is in akr-api | "for the source-tag proposal, its decision check and its activation (section 7.17 items 6 and 7)". Nothing is blocked (the grants exist), so MEDIUM.
- P-13b-2 | CONFIRMED | MEDIUM | item 7 says "the hourly run does not take those months again". Section 3.2 step 1 takes every month-end within `features.catchup_months` (3) back from the newest on every run and step 3 runs the upsert of section 3.1, which updates changed values. So the hourly run does reach the last three months; the reopen is needed for older months only. The fixture of item 14 ("two months back") lies inside the window | the reason is restated (the hourly run reaches only the catch-up window); the fixture uses a `valid_from` five months back and says "those months".
- P-13b-3 | CONFIRMED | LOW | "a queued hourly run would swallow the enqueue": the phase 00 scheduler (section 3.x step 6, line ~549) enqueues a periodic kind with the key `<kind>:<slot ISO>`, so the hourly job never holds the plain key `feature_compute`. What swallows a plain-key enqueue is another plain-key job (the catch-up enqueue of a `feature_definition` activation, a replay enqueue) | item 7, the test of item 14 ("also while an hourly ... job is queued") and the Open doubts entry say "a queued `feature_compute` job with the plain key".
- P-13b-4 | CONFIRMED | LOW | "(a row ended on day D still holds on D, so the month of D is the first that can change)": the first date that can change is the day after D; taking the month of D is at most one month early and harmless | reworded.
- L2-2 | LEAD FIND while deciding P-13b-2 | HIGH (a stated check fails) | the upsert of section 3.1 never touches a row that the recompute no longer produces, and `akr_app` has no DELETE on `feat.feature_values` (deliberate, line ~314). After a tag is voided, a customer whose only matching credits came from that source produces no group, so the old row keeps its value for the reopened months and for the catch-up months. The test of item 14 ("the feature rows ... no longer count the credits") fails for such a customer. The same holds in base text for any re-run in which a group disappears (a replay that cancels the only posting) | DESIGN TO WRITE (section 7.17 item 7, extending section 3.2 step 5 and the comment of `FeatureValueWriter::write_month_end`): a write for a `(M, feature)` pair that already has rows also resets, in the same transaction, every row of that pair that the SELECT did not produce: kinds `count` and `sum` get `value = 0`, status `value` (absent means zero, section 3.2 step 4); every other kind gets `value = NULL`, status `no_history` (as a zero denominator does, section 3.1 step ~8, line ~551), `basis` as the run's; a row already in that reset form is not written again. No new grant: UPDATE (value, status, basis, computed_at) is granted. The test of item 14 gains the customer whose only credits were tagged (its row becomes 0). CHECK FIRST when writing: the list of kinds in section 3.1 (lines ~535 to 551) and that `rows_written` counts these updates.

### Key 08 (12 entries; raised 2; confirmed 2)
- P-08-1 | CONFIRMED | MEDIUM | section 7.x item 3 shows the change request as text for A2, whom G10 does not list, but the base X3 text (line ~403) says "For a change-request task the panel links to G10 detail" for every role, and the "Waiting on others" rows of an A2 maker open that panel (phase 04 section 3.7 `waiting`). The link leads A2 to a screen the role does not hold | line ~403: the panel links to G10 detail for a role that G10 lists (the screen table) and shows the reference as text for any other role.
- P-08-2 | CONFIRMED | LOW | "no response gives the size limit": `GET /admin/settings` gives it to ADM (phase 03). The form's users, A2 and M, cannot read it, so the behaviour is right and the wording is not | "no response open to A2 or M gives the size limit".

### Key DR (1 entry, the DR-422 row; raised 2; confirmed 2)
- P-DR-1 | CONFIRMED | MEDIUM (agent: HIGH) | claim (4) of the row says "each deletion of an uploaded file writes `goaml_corpus_case.transient_deleted`". Phase 07 section 7.6 item 5 step 5 deletes the staged file of a failed proposal and item 9 says that deletion writes no event (no case row exists). A DR wins over block text, so the two cannot both stand | the row says "each deletion of an uploaded file by an intake run or by the sweep ...; a failed proposal has no case row and writes no event". The citing sentence of block 7.6 is brought in step (P-07b-6).
- P-DR-2 | CONFIRMED | MEDIUM (agent: HIGH) | claim (6) names phase 13, but phase 13 section 6 item 1 (lines ~972 to 973, a sentence that wraps over two lines and was missed by the round 1 sweep) still expects `feature definition content is immutable` as `akr_app` for `UPDATE feat.feature_definitions SET expression = '{}'`, and `akr_app` holds UPDATE (state, retired_at) only (line ~317) | the check line says permission denied for `akr_app` and the guard message for the owner `akr_migrate`.
- Lead sweep made again for P-DR-2 (every check paragraph of phases 00 to 31 that expects a guard message, read against the GRANT lines; this time over wrapped lines): phase 13 `feat.peer_groups (state)`, phase 14 `ref.list_entries` INSERT and `alert.screening_alerts (state)`, phase 15 `kyc.rating_models (document, status)`, phase 16 `scn.scenario_versions (document)`, phase 20 `casework.ops_tasks (cbs_reference, verification_level)` and `casework.freezes (state)`, phase 21 `rpt.report_versions`, `rpt.filings (ack_at)` and `rpt.letters (state)` are all in the column grants, so their guard messages are right as written. One more clause is not exact: L2-3 below.
- L2-3 | LEAD FIND | LOW | phase 18 section 6 item 9 (line ~1246), last clause: "a `casework.case_acl` row cannot change its `access` (the guard raises)". `akr_app` holds UPDATE (revoked_by, revoked_at) only (line ~428), so for `akr_app` the statement is refused with permission denied; the guard message `an access grant is revoked, never changed` is reached as the owner. The clause names no role, so no stated check fails | the clause names both roles.

### Key 19b (19 entries in phase 19, API.md, DESIGN-SYSTEM.md; raised 3; confirmed 3)
- P-19b-1 | CONFIRMED | MEDIUM (agent: HIGH) | the default-deny note of section 3.13 (line ~1594) says "for CFG and CHK the `rfi_prefill_map.read` and `rfi_prefill_map.propose` permits". The Cedar text of section 7.21 item 9 gives CHK the read only. The Cedar text is exact and the tests follow it, so nothing fails; the prose is wrong | "for CFG the `rfi_prefill_map.read` and `rfi_prefill_map.propose` permits and for CHK the `rfi_prefill_map.read` permit of section 7.21".
- P-19b-2 | CONFIRMED | LOW (agent: MEDIUM) | the check "`DELETE FROM casework.rfi_prefill_map` fails ... as the owner `akr_migrate` with `pre-fill map rows are never deleted`" stands under `make migrate`, where the table is empty; the guard is a row trigger, so the owner's statement raises only when a row exists. The neighbouring checks of the same item name their rows, so a builder runs the item with fixture rows; the wording is made exact | "a `DELETE` of a row fails ...". The twin line of phase 13 section 7.17 item 14 (`DELETE FROM feat.source_tags`) gets the same words.
- P-19b-3 | CONFIRMED | LOW (agent: MEDIUM) | DESIGN-SYSTEM.md (line ~850) says "CHK and M read the tab"; phase 19 section 7.21 item 11 says CFG, CHK and M. CFG is named as the proposer in the sentence before, so no screen is built wrong | "CFG, CHK and M read the tab".

### Key 07b (16 entries; raised 7; confirmed 7, two of them the same defect as a finding of another key)
- P-07b-1 | CONFIRMED | MEDIUM | the tipping-off review (section 7.6 item 12) says the masked file is evidence "that only the corpus runner opens"; item 6 step 4 has the intake job open it again at each profile activation | "that only the corpus runner and the intake job (item 6 step 4) open".
- P-07b-2 | CONFIRMED | LOW | the same defect as P-14b-1 (the pointer "phase 04 section 3.2" in the Activate row of section 7.6 item 7) | fixed with P-14b-1.
- P-07b-3 | CONFIRMED | LOW | section 6 item 1: "(as `akr_app` it fails with permission denied, the last check of this item)" was meant as a pointer to the last check and reads as if this were the last check | "(as `akr_app` it fails with permission denied; see the last check of this item)".
- P-07b-4 | CONFIRMED | LOW | the same sentence quotes the guard message without its first word; the trigger raises `goAML profile versions are never deleted` (line ~113) | the full message.
- P-07b-5 | CONFIRMED | LOW | section 7.6 item 14: "retention row. a `DELETE` fails" starts a sentence with a small letter | "A `DELETE` fails".
- P-07b-6 | CONFIRMED | MEDIUM (agent: HIGH) | the same defect as P-DR-1, in the citing sentence of block 7.6 ("each deletion of an uploaded file is audited") | "each deletion of an uploaded file by an intake run or by the sweep is audited".
- P-07b-7 | CONFIRMED | HIGH | section 7.6 item 7 says a case whose intake did not meet the expectation "cannot be approved: the approver rejects the request or its maker withdraws it", and phase 08 closes an A2 maker's unwanted proposal by "the approver's reject" (an A2 maker has no withdraw screen). But phase 04 runs the subject check before every engine action, Approve or Reject (section 3.2 `decide`; section 3.4 step 6; section 7.1), the hook has no action argument, and the `check_before_decision` row of item 7 returns `intake_not_met` whenever `met` is false. As written the approver's rejection of an unmet case is refused with 422 and the case stays `proposed`, holding its key and a place under `goaml.corpus_bank_cases_max`. The same holds in phase 04 base text for every change target (a request with a stale base or stale evidence could not be rejected), against the offboarding rule "Submitted requests made by the user stay: their checkers still decide them" (section 3.6) | DESIGN (new phase 04 section 7.21, DR-423): the trait `WorkflowSubject` gains `fn holds_reject(&self) -> bool` with the default `true`; engine step 6 is skipped for a `Reject` when it is `false`; the change subject returns `false`, so a rejection of a change request runs neither the evidence-age check nor the target's check. Every other subject keeps the default. Phase 07 item 7 says that an issue refuses an approval and points to the rule.

### Key 26 (20 entries in phase 26, DSL.md, DATA-MODEL.md; raised 4; confirmed 4)
- P-26-1 | CONFIRMED | MEDIUM | section 7.13 item 5 says "The three inputs marked `?`"; the row marks four (`facility.granted_on` and the three `facility_instalment` fields). "the three are listed in the `reasons`" is wrong in the same way | "The four inputs marked `?`"; "the ones not published are listed".
- P-26-2 | CONFIRMED | LOW | "has a `admin.column_classification` row" | "has an".
- P-26-3 | CONFIRMED | MEDIUM | Open doubts 24 says a changed due date or amount "is recorded as a conflict (phase 10 section 7.27 item 3)". Item 3 reports it only when the payment columns did not move; in a version that also moves the payment it is neither written nor reported | the entry says both cases.
- P-26-4 | CONFIRMED | LOW | the citing sentence of block 7.13 says DR-422 locks "what the grader does with the optional inputs"; the DR-422 row does not hold that rule. The rule was a HIGH correction of round 1 (O-26-2), so it belongs in the row | group (1) of the row gains it.
- L2-4 | LEAD FIND from the agent's two notes | LOW | docs/DATA-MODEL.md line ~830 says that where the bank does not map the schedule "the scenario is declared not covered"; phase 26 section 7.13 item 5 says the grade does not change and the facility legs are not evaluable. Line ~889 says `akr_ingest` "may UPDATE only" a list that leaves out the phase 26 updates named further down | both lines are brought in step.

### Round 2 totals
- Raised by the 16 agents: 29. Confirmed 29 (P-07b-2 is the same defect as P-14b-1, and P-07b-6 the same as P-DR-1); rejected 0.
- By lead severity: HIGH 1 (P-07b-7), MEDIUM 13 (P-07a-1, P-07b-1, P-07b-6, P-08-1, P-10-1, P-11-1, P-13b-1, P-13b-2, P-19b-1, P-26-1, P-26-3, P-DR-1, P-DR-2), LOW 15.
- Keys with no finding: 13a, 14a, 18, 19a, 25, L1 (6 of 16).
- Lead finds: 4 (L2-1 LOW, L2-2 HIGH, L2-3 LOW, L2-4 LOW).
- Two design rules come out of the round and are locked as DR-423: a recompute resets the feature rows it no longer produces (L2-2, phase 13 section 7.18), and a rejection of a change request is not held by the subject check (P-07b-7, phase 04 section 7.21).
- Agent conduct: several agents used read-only `awk`, `tr`, `printf`, `echo`, `sort` or `uniq` outside the allowed command list; the key 26 agent wrote one scratch file under /tmp by mistake. No agent reported Docker, a database, git or the network.

### L2-2, the design as written (replaces the sketch in the L2-2 line above)
- While writing phase 13 section 7.18 the lead read how the values are consumed. Section 3.4 step 2 builds a segment statistic from the rows with `status = value`, so a row reset to 0 with status `value` would enter the population as a zero and move `n` and the percentiles; an absent row does not. The reset therefore takes the value away for every kind: `value` NULL, status `no_history` (`beyond_window` for the kind `days_since`), `basis` = the run's basis plus `"reason": "no_activity"`. A reader takes a row without a value as an absent row (sections 3.4 and 3.6 already read `status = value` only), so a `count` or `sum` feature still reads as zero under a `computed` run. Only a row with `status = 'value'` is reset, which keeps a second run at zero writes. The statement has four parts (`cur`, `had`, `up`, `gone`) and counts inserted and updated rows through `had`, with no system column.
- Round 2 fixes applied: tools/agent/ofix-r2.py, 46 replacements in 14 files (tag R2 in tools/agent/owner-fixes.txt). Checks after them: idcheck "DR defined: 423 problems: 0" (run over every plan file it names no undefined DR); routecheck 595 rows, 595 owned, 0 not owned, 5 absent; coverage 1208 / 940 / 268, 0 named nowhere; owner-sync: only the two known diffs (p18.md, p25.md).

## Round 3

Round 3 checks the 46 replacements of round 2. Eleven Haiku agents, one per key (prompt tools/agent/p-owner3.txt; input tools/agent/verify/owner3-fixes-<KEY>.txt, written by `python3 tools/agent/mk-owner2.py R2 owner3`; output tools/agent/verify/owner3-<KEY>.md). Keys: 04, 07, 08, 10, 11 (phases 11 and 12), 13, 14, 18, 19, 26, DR (the DR-422 and DR-423 rows). Finding ids are Q-<KEY>-k.

### Key 18 (1 entry; raised 1; confirmed 1)
- Q-18-1 | CONFIRMED | LOW (agent: LOW) | the clause of phase 18 section 6 item 9 gives the guard message `an access grant is revoked, never changed` for any `casework.case_acl` row. The guard (line ~342) first raises `a revoked access grant does not change` when `OLD.revoked_at` is set, so the quoted message is reached only on a grant that is not revoked; the clause names no row state | the clause says "a `casework.case_acl` row that is not revoked" and names the second message for a revoked row.

### Key 14 (1 entry; raised 0)
- FINDINGS: 0. F1 passed.

### Key 19 (3 entries in phase 19 and DESIGN-SYSTEM.md; raised 0)
- FINDINGS: 0. F1 to F3 passed.

### Key 10 (2 entries; raised 0)
- FINDINGS: 0. F1 and F2 passed.

### Key 11 (4 entries in phases 11 and 12; raised 0)
- FINDINGS: 0. F1 to F4 passed.

### Key 08 (2 entries; raised 0)
- FINDINGS: 0. F1 and F2 passed.

### Key 04 (2 entries, the new section 7.21 among them; raised 0)
- FINDINGS: 0. F1 and F2 passed.

### Key 07 (8 entries; raised 0)
- FINDINGS: 0. F1 to F8 passed.

### Key DR (3 entries in docs/plan/research.md: two sentences of the DR-422 row and the new DR-423 row; raised 0)
- FINDINGS: 0. F1 to F3 passed.

### Key 26 (6 entries in phase 26 and DATA-MODEL.md; raised 2; confirmed 2; one lead find)
- Q-26-1 | CONFIRMED | LOW (agent: HIGH) | docs/DATA-MODEL.md line ~891, the sentence that the round 2 fix L2-4 touched: "no role has DELETE on `canon` except `akr_retention` on the party tree (phase 23 ...)". Phase 26 also gives `akr_retention` DELETE on `canon.islamic_deals`, `canon.deal_parties` (line ~118) and `canon.facility_instalments` (lines ~153, ~1045), and the pointer L2-4 added speaks of "the updates" only. The rule itself holds (only `akr_retention` deletes in `canon`) and the phase 26 paragraph of the same file names the grants, so nothing is built wrong; the summary sentence is one list short. LOW, not HIGH: the GRANT lines of phase 26 are the source and no stated check reads this sentence | the sentence ends at the phase 10 and phase 23 state, and one new sentence names what phase 26 adds to both lists.
- Q-26-2 | CONFIRMED | LOW (agent: LOW) | the pointer "(phase 26 adds the updates named in its own paragraph below)" stands right after "`status` and `status_history` of `payment_message`" and can be read as if phase 26 added those two updates; they are phase 10 section 7.18 | the pointer is taken out of the list and becomes the new sentence of Q-26-1.
- L3-1 | LEAD FIND while checking Q-26-1 | LOW | the same "may UPDATE only" list leaves out `end_at` and `row_hash` of `account_hold`, which phase 10 section 7.19 grants to `akr_ingest` (`GRANT UPDATE (end_at, row_hash) ON canon.account_hold`, added to migration 0030 by the verification pass, DR-413). Checked: the UPDATE grants on `canon` in the phase files are phase 10 lines ~773 to ~779 and ~1151, phase 10 section 7.19, and phase 26 lines ~114, ~115, ~150, ~151; the DELETE grants on `canon` are phase 23 line ~374 and phase 26 lines ~118 and ~153 | the list gains "`end_at` and `row_hash` of `account_hold` (phase 10 section 7.19)".

### Key 13 (14 entries in phase 13 and DATA-MODEL.md, the new section 7.18 among them; raised 2; confirmed 2)
- Q-13-1 | CONFIRMED | LOW (agent: MEDIUM) | phase 13 section 6 item 1 (line ~976): the owner half of the `expression` check ("as the owner `akr_migrate`, with `feature definition content is immutable`") stands in the `make migrate` item, and `feat.feature_definitions` is filled only by the seed of item 3; `feat.definition_guard` is a row trigger (line ~290), so on the empty table the owner's UPDATE meets no row and raises nothing. Same class as P-19b-2 of round 2, and LOW for the same reason: the next clause of the item already names a fixture row ("on a `candidate` row"), and a guard test on an empty table fails at once, so nothing is built wrong | the clause says "on a stored row" and why, and gives the guard message in full as the trigger raises it (`feature definition content is immutable; propose a new version`, line ~284). The section 7.18 SQL and its four items passed (F12, F13).
- Q-13-2 | CONFIRMED | LOW (agent: LOW) | docs/DATA-MODEL.md line ~545, the `feature_values` row that L2-2 changed, gives `no_history` as the only status of a reset row; phase 13 section 7.18 item 1 gives `beyond_window` for the kind `days_since` | the row names both.
- Lead note on the class of Q-13-1 (not swept, by decision): the other check lines that give a guard message for the owner role (phase 03 line ~1180; phase 06 ~910; phase 07 ~912 and ~1230; phase 09 ~1184 and ~1189; phase 14 ~1271 and ~1603; phase 15 ~1931) also need a stored row, and their tables are filled by a seed command or by use, not by a migration. They are left as they stand: each stands in a list of negative checks whose other clauses name fixture rows ("on a live row", "of a `rejected` row", "an `active` version"), the key 07 and key 14 agents of this round read two of them under the row-trigger rule and passed them, and a test that expects a guard message from an empty table fails on its first run. Recorded as a build note for the report: a guard check is run on a fixture row.

### Round 3 totals
- Agents: 11 keys, 46 entries checked. Raised 5; confirmed 5; rejected 0.
- Severity on the lead's scale, for the 5 confirmed findings of this round: HIGH 0, MEDIUM 0, LOW 5 (Q-13-1, Q-13-2, Q-18-1, Q-26-1, Q-26-2). On the agents' own scale: HIGH 1 (Q-26-1), MEDIUM 1 (Q-13-1), LOW 3.
- Keys with no finding: 8 of 11 (04, 07, 08, 10, 11, 14, 19, DR). The two new sections (phase 13 section 7.18 with its SQL, phase 04 section 7.21) and the DR-423 row passed with no finding.
- Lead finds: 1 (L3-1 LOW, the `account_hold` columns in the DATA-MODEL.md grants summary).
- No design rule changes in round 3; no new DR.
- Fixes: tools/agent/ofix-r3.py, 4 replacements in 3 files (phase 13, phase 18, docs/DATA-MODEL.md); Q-26-1, Q-26-2 and L3-1 are one replacement of one sentence.
- Agent conduct: four agents said they used `echo` as a separator or a `cd` prefix in Bash, outside the allowed list; read-only, no file written. No agent reported Docker, a database, git or the network.
- Fixes applied 2026-10-11 (tools/agent/ofix-r3.py: "applied 4 fixes in 3 files"). Checks after the fixes: idcheck "DR defined: 423 problems: 0"; routecheck "API.md rows 595; owned 595; not owned 0; phase routes absent from API.md 5"; coverage "baseline 1208; carried 940; uncarried 268 (0 named nowhere)"; owner-sync: only the two known diffs (p18.md, p25.md).

## Round 4

Round 4 checks the 4 replacements of round 3. Three Haiku agents, one per key (prompt tools/agent/p-owner4.txt; input tools/agent/verify/owner4-fixes-<KEY>.txt, written by `python3 tools/agent/mk-owner2.py R3 owner4`; output tools/agent/verify/owner4-<KEY>.md). Keys: 13 (phase 13 and the `feature_values` row of DATA-MODEL.md), 18, 26 (the grants summary of DATA-MODEL.md). Finding ids are R-<KEY>-k.

### Key 18 (1 entry; raised 0)
- FINDINGS: 0. F1 passed: both guard messages are the ones the trigger raises, each for the row state the clause names.

### Key 13 (2 entries in phase 13 and DATA-MODEL.md; raised 0)
- FINDINGS: 0. F1 and F2 passed.

### Key 26 (1 entry in DATA-MODEL.md; raised 0)
- FINDINGS: 0. F1 passed: every item of the sentence agrees with the GRANT statements of phases 10, 23 and 26.
- Agent note, not counted and not a gap: the UPDATE grant on `canon.account_hold` stands only in the text of phase 10 section 7.19 ("Migration `0030` (section 1.2) gains ..."), not in the SQL of section 1.2. That is how every section 7 block adds to a migration: the block wins over base text and a builder reads both.

### Round 4 totals
- Agents: 3 keys, 4 entries checked. Raised 0; confirmed 0; rejected 0. Lead finds: 0.
- A clean round: nothing was changed, so no further round follows. Confirmed findings plus lead finds per round: 102, 33, 6, 0. Counted as distinct defects (a defect raised twice counted once: one pair in round 1, two pairs in round 2), the true gaps are 101, 31, 6, 0, which is 138 in all: HIGH 12 (10 in round 1; P-07b-7 and L2-2 in round 2), MEDIUM 54 (42 and 12), LOW 72 (49, 17 and 6). The report uses the distinct count.
- Checks (2026-10-11, after round 3's fixes, unchanged since): idcheck "DR defined: 423 problems: 0"; routecheck "API.md rows 595; owned 595; not owned 0; phase routes absent from API.md 5"; coverage "baseline 1208; carried 940; uncarried 268 (0 named nowhere)".
