# Refuter, phase 18 (Haiku draft for lead review)

### V-18-01-a  PARTLY   (first agent: MEDIUM; refuter: LOW)
- Claim: No HTTP status or problem code for the refusals of step d(ii) (case not in_review, or proposal not running), d(iii) (case sealed) and d(vii) (evidence not clean, sub-code only) of section 3.8.
- Evidence: The status and code come from the phase's own error table. phase-18.md:912 "| `Sealed` | `E_CASE_SEALED` | 409 | closed and sealed: reopen first |" covers d(iii). phase-18.md:913 "| `NotAllowed { sub }` | `E_NOT_ALLOWED_IN_STATE` | 409 | the state does not allow the command; `sub` in the body |" covers d(ii). phase-18.md:1486 (DR-136) "The gate's refusals use the closed sub-codes of section 3.17 inside `E_VALIDATION`" covers d(vii); the table row at phase-18.md:916 gives 422 for `Validation`. docs/API.md:981-982 gives the same 409 codes. Standing part: d(ii) names no sub-code for its 409, although section 3.17 says `sub` is in the body. The only in_review sub-code is at phase-18.md:733 "Status `in_review` -> 409 `E_NOT_ALLOWED_IN_STATE` (sub-code `proposal_running`)", which is not repeated in step d(ii).
- Fix: Phase 18 section 3.8 step d(ii): name the sub-code for each branch (case not in review, proposal not running).

### V-18-02-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: No repository method is named for the alert.alerts update (contribution fields, version + 1) or the alert.alert_events insert in step 3.8 i (phase-18.md:773).
- Evidence: The update has a named owner method, so this part is refuted. phase-17.md:808 "async fn update(&self, tx: &mut Tx, a: &Alert, expected: i32)" (AlertRepository, comment "version + 1, CAS"). The trail write has no named writer in phase 17 or 18: `grep -n "alert_events"` over phase 18 finds phase-18.md:441, 773 and 942 only, and phase 17 defines the `AlertEvent` struct (phase-17.md:783) with no insert method. The only general rule is phase-17.md:1372 "are written in the same transaction as the audit event", which fixes timing, not the writer.
- Fix: Phase 18 section 3.8 step i: name the method that inserts the `alert.alert_events` row and point the alert row write at phase 17 `AlertRepository::update`.

### V-18-03-a  PARTLY   (first agent: HIGH; refuter: MEDIUM)
- Claim: The route `POST /cases/{id}/opened-views` (record that a panel was opened) has no numbered steps: no check of view_key, no 404, no insert, no transaction and no audit call for a panel.
- Evidence: The 404, insert, transaction and audit are designed. phase-18.md:950 "404 for a case the caller cannot see; nothing about other people's views". phase-18.md:797 "`casework.opened_views` is written in the caller's own transaction after the access event". phase-18.md:932 "an evidence item or panel opened (acknowledgement) | case | no (audit only)". The six panel keys are enforced by the table CHECK (phase-18.md:272-273), and the insert has its repository method (phase-18.md:580). Standing part: no step checks that an evidence view_key is an evidence id of this case, and no status is named for a key that the CHECK refuses. The evidence path at phase-18.md:703 is the pattern to follow, but it does not cover the panel key check. The route is a real write path (phase-18.md:1157 posts it on each panel expansion), so the gap is one input check, not a missing path.
- Fix: Phase 18 section 3.9 or 3.4: add numbered steps for `POST /cases/{id}/opened-views` that check view_key (a panel key, or an evidence id of this case) and name the status for a refused key.

### V-18-04-a  STANDS   (first agent: LOW; refuter: LOW)
- Claim: The chain append is named `CaseChain::append_event(tx, case_id, kind, payload)` (phase-18.md:679 and 845; line 624 says "through `CaseChain`"), but phase 17 defines it as `CaseRepository::append_event(tx, case, NewCaseEvent)`, and `CaseChain` has `verify` only.
- Evidence: phase-18.md:845 "6. `CaseChain::append_event(tx, case_id, "case.closed", {reason_code, outcome})`". phase-17.md:820 "async fn append_event(&self, tx: &mut Tx, case: &mut Case, e: NewCaseEvent)". phase-17.md:1111 "Append (`CaseRepository::append_event`): under the case row lock". `grep -n "CaseChain::"` over phase 17 finds only 1115 and 1673, both `verify`.
- Fix: Phase 18 section 3 intro (line 624), section 3.3 step 5 (line 679) and section 3.12 step 6 (line 845): name `CaseRepository::append_event` with its phase 17 arguments.

### V-18-04-b  STANDS   (first agent: LOW; refuter: LOW)
- Claim: The `set_closure` call in step 9 does not match the section 2 signature (no tx argument, no outcome).
- Evidence: phase-18.md:581 "`set_closure(tx, id, closed_at, sealed_at, seal_hash, outcome)`". phase-18.md:848 "9. `set_closure(case, closed_at, sealed_at, seal_hash)`; `status = closed`; `version + 1`." `grep -n "set_closure"` finds these two lines only.
- Fix: Phase 18 section 3.12 step 9 or section 2 line 581: give the call and the signature the same argument list, saying whether outcome is passed.

### V-18-05-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The reject comment of the decide form has no generic-text check (`GenericTextGuard::check`), and no status or code is given for a missing comment.
- Evidence: The missing-comment refusal is inferable. phase-18.md:753 "a `reject` needs `comment`" gives no status, but the field-refusal row at phase-18.md:916 is 422 `E_VALIDATION`, and the closed list (phase-18.md:921) already holds `note_required`. Standing part: the generic-text check. research/31-analyst-ux.md:965 "Decision forms require coded reasons and refuse generic or capacity-driven text" (UX-65, the requirement). DR-137 covers only outcome reasons: phase-18.md:1488 "the guard enforces it everywhere the reason of an outcome is written". The reject comment is a reason for ending a proposal, not an outcome, and the decide dialog requires it (phase-18.md:1180, "Reject (comment required)").
- Fix: Phase 18 section 3.8 route step 1: say whether the reject comment passes `GenericTextGuard` and which refusal it gets.

### V-18-05-b  STANDS   (first agent: LOW; refuter: LOW)
- Claim: Section 5.4 refers to "the step-up prompt of 7.12", which is global search in this file, not the step-up dialog.
- Evidence: phase-18.md:1180 "the step-up prompt of 7.12 (class `decision`)". phase-18.md:1375 "### 7.12 Global search (X2; UX-29; SP-27, RP-61; DR-400)". The step-up dialog is docs/DESIGN-SYSTEM.md:583 "### 7.12 Step-up dialog and session warning". The text does not name that document. `grep -n "step-up prompt"` finds 1180 only.
- Note: it is unclear whether a wrong section number falls under the "name differs only by a section number" rule. Left at LOW for the lead.
- Fix: Phase 18 section 5.4 Decide bullet (line 1180): name the document (DESIGN-SYSTEM 7.12) for the step-up prompt.

### V-18-06-a  STANDS   (first agent: LOW; refuter: LOW)
- Claim: The sub-codes `report_sign_required` and `filing_not_acknowledged` added by 7.3 are not in the closed sub-code list of section 3.17, and the block does not say they join it.
- Evidence: phase-18.md:921 "The sub-codes of this phase are a closed list:", and the list has neither code. phase-18.md:1280 "sub-code `report_sign_required`, unless `via = "report"`". phase-18.md:1282 names `filing_not_acknowledged`. Section 7.8 does this for phase 19's codes: phase-18.md:1307 "the 24 sub-codes of phase 19 section 3.11 join the closed list".
- Fix: Phase 18 section 3.17 closed list (line 921): add `report_sign_required` and `filing_not_acknowledged`, as section 7.8 does for phase 19's codes.

### V-18-06-b  STANDS   (first agent: LOW; refuter: HIGH)
- Claim: Base step e says "`sign` behaves as `approve` for `case_report_approval`" (phase-18.md:759). The 7.3 block makes the decision route refuse that sign unless via = "report", but does not name the sentence it replaces.
- Evidence: phase-18.md:759 "`sign` behaves as `approve` for `case_report_approval`." phase-18.md:1280 "`report_sign_required`, unless `via = "report"`". phase-18.md:1274 changes only the step-up class: "uses step-up class `signing`, not `decision`". `grep -n 'behaves as'` finds 759 only, and `via = "report"` appears at 1277 and 1280 only.
- Severity note: the two texts give opposite rules for sign on this route, and no 7.x block says which wins. The rubric puts a contradiction at HIGH. The intent of 7.3 is clear, so the fix is one sentence.
- Fix: Phase 18 section 3.8 step e: state that sign on `case_report_approval` through this route is refused unless the act carries via = "report".

### V-18-07-a  REFUTED
- Claim: The request DTO of the grant does not carry `expires_at`, though the block says the body does (section 2 `GrantAccess`, phase-18.md:566; `AccessView` fields, phase-18.md:569).
- Evidence: phase-18.md:1329-1330 "the body carries `expires_at`, later than now and at most `casework.manual_grant_max_days` ahead". The list output carries the field too: phase-18.md:1345 "`list` returns `expires_at`." The fields are stated in prose in the plan, and under the rule that a type whose fields are stated in prose needs no struct listing, this is not a gap. Section 2 (line 566) does not repeat the field. That is a tidy-up point, not a gap under the rule.

### V-18-07-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The timer kind `case_acl_expiry` (owner kind `case_acl`) is not in the phase 00 timer kind registry; section 4.5 item 13 registers only `work_item_due`.
- Evidence: phase-00.md:859-860 "a kind that is not in the registry is never claimed". phase-00.md:870 "| `work_item_due` | `work_item` | phase 18 |", with no `case_acl_expiry` row. phase-18.md:1111 "The kind is registered in the phase 00 timer kind registry by this phase." Section 7.10 does not say this for its kind. `grep -rn "case_acl_expiry" docs/plan/` finds phase-18.md only (lines 476, 1335, 1337, 1340-1342, 1444, 1454).
- Fix: Phase 18 section 4.2 and item 13 of section 4.5: register the timer kind `case_acl_expiry` (owner kind `case_acl`) in the phase 00 section 7.3 registry.

### V-18-07-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Job `case_acl_expiry_scan` (item 5) has no concurrency and no attempts, and the handler `timer_case_acl_expiry` is not registered in AppState (section 4.3 lists only `work_item_clock`).
- Evidence: phase-18.md:1340 "5. Job `case_acl_expiry_scan` (akr-worker, setting `schedule.case_acl_expiry_scan`, hourly at minute 15)" gives no concurrency or attempts. The section 4.1 table (phase-18.md:1102) has a "Concurrency, attempts" column filled for each row, and this job has no row. phase-00.md:539 "one semaphore per kind, sized by `JobKind::concurrency`" gives no default. phase-18.md:1118 "`akr-worker`: `work_item_clock: Arc<WorkItemClockHandler>`" is the only worker entry.
- Fix: Phase 18 section 4.1 table and section 4.3: add a row for `case_acl_expiry_scan` with its concurrency and attempts, and an akr-worker entry for `timer_case_acl_expiry`.

### V-18-07-d  STANDS   (first agent: LOW; refuter: LOW)
- Claim: Item 6 names section 3.9 for the rule-5 visibility clause; the clause is in section 3.10.
- Evidence: phase-18.md:1343 "6. The rule-5 visibility clause (section 3.9) and `Holders` rule 5 also require". phase-18.md:789 "### 3.9 The view gate (`ViewGate`; M-21, DR-136)". phase-18.md:801 "**Case rows** (phase 17 rules 1 to 5, with one clause added in rule 5)". The clause is findable by its name in 3.10.
- Note: it is unclear whether a wrong section number falls under the "name differs only by a section number" rule. Left at LOW for the lead.
- Fix: Phase 18 section 7.10 item 6: change the section reference from 3.9 to 3.10.

### V-18-07-e  REFUTED
- Claim: Item 2 says "(step 3 of the handler below)", but the handler in item 4 has no numbered steps.
- Evidence: phase-18.md:1337-1339 "4. Handler `timer_case_acl_expiry` (akr-worker): lock the case, re-read the grant; if it is still active and `expires_at <= now()`, set `revoked_at = now()` with `revoked_by` NULL". The handler is one sentence that names the checks and their order. Its third clause is the revoke, which item 2 cites. A one-sentence step that names the check, the error and the order needs no numbering, so this is not a gap.

### V-18-07-f  STANDS   (first agent: LOW; refuter: LOW)
- Claim: The grant method returns `CaseAccessView` (phase-18.md:653), which section 2 does not define; section 2 names `AccessView` (phase-18.md:569).
- Evidence: phase-18.md:653 "**`grant(ctx, case_id, GrantAccess, if_match) -> CaseAccessView`**". phase-18.md:569 lists `AccessView` among the views with no fields. `grep -n "CaseAccessView"` over docs/plan finds phase-18.md:653 only.
- Fix: Phase 18 section 2 views (line 569) or the section 3.2 grant signature (line 653): use one name for the access view and list its fields in one place.

### V-18-08-a  REFUTED
- Claim: The response of the decide route ("the new row") has no named type or fields, and the new fields of `GET /cases/{id}/links` (counterparty id, open link method, party) are not in `EntityLinkView` (line 569).
- Evidence: phase-18.md:1367 "Response 200 with the new row." The row is the phase 13 table, whose columns are in the DDL: phase-13.md:1065 "CREATE TABLE IF NOT EXISTS feat.counterparty_links (". phase-18.md:1369-1370 "`GET /cases/{id}/links` adds, for each `transaction` link, the counterparty id and its open link row (method and party)". Fields stated in prose, a table or a code block anywhere in the plan need no separate type under the rule.

### V-18-08-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The insert of the new counterparty link row does not set `valid_from`, which is NOT NULL with no default in phase 13 (phase-13.md:1072).
- Evidence: phase-13.md:1072 "valid_from          TIMESTAMPTZ NOT NULL," has no DEFAULT, while phase-13.md:1074 "created_at ... NOT NULL DEFAULT now()" does. phase-18.md:1365-1366 "insert `analyst_confirmed` or `analyst_rejected` with `decided_by`" names no valid_from. `grep -n "valid_from"` over phase 18 finds nothing. The value can be inferred from the close in the same step (`valid_to = now()`), so MEDIUM rather than HIGH.
- Fix: Phase 18 section 7.11 Decide steps: state the valid_from of the new row (the decision time).

### V-18-09-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The response type of `GET /api/v1/search` is not named, and its row fields (party and case rows, has_more, group names) are not given; the utoipa list of section 2 does not include it.
- Evidence (refuted part): phase-18.md:1382 "groups `party` and `case` with `has_more`, no totals" gives the group names and has_more, and phase-18.md:1473-1474 (7.20) uses has_more again. Standing part: the response type is not named, and the fields of the party row and the case row are stated nowhere. phase-18.md:586 "**utoipa schemas**: every DTO and view above, with `ToSchema`" does not reach this route. `grep -n "Search"` over phase 18 finds `SearchService` (line 1384) and `SearchKind` (line 1386) only.
- Fix: Phase 18 section 7.12: name the search response type and list the fields of its party row and case row.

### V-18-09-b  PARTLY   (first agent: MEDIUM; refuter: LOW)
- Claim: No status or problem code for the two refusals in the route row: `q` shorter than 2 or longer than 80 characters, and `limit` above 10.
- Evidence (refuted part, `q` range): phase-00.md:499 "| `Validation(Vec<ValidationError>)` | 422 | `E_VALIDATION` (`errors[]` with JSON pointers) |" gives 422 E_VALIDATION for field validation, which covers `q` outside 2 to 80. Standing part: the text does not say whether a `limit` above 10 is refused or capped. phase-18.md:1382 says "`limit` at most 10 per group", and the general cap in docs/API.md:46 ("maximum 200") does not say what happens above it. The text is unclear, so LOW.
- Fix: Phase 18 section 7.12: say whether a `limit` above 10 is refused (422 at `/limit`) or capped at 10.

### V-18-10-a  STANDS   (first agent: LOW; refuter: LOW)
- Claim: Step 5 of section 3.13 calls `set_reopened(case)` with one argument; section 2 defines `set_reopened(tx, id, at)`.
- Evidence: phase-18.md:581 "`set_reopened(tx, id, at)`". phase-18.md:861 "5. `set_reopened(case)`: `status = open`, `closed_at`, `sealed_at`, `seal_hash` and `outcome` cleared". `grep -n "set_reopened"` finds lines 581 and 861 only.
- Fix: Phase 18 section 3.13 step 5: give the call the section 2 arguments (tx, id, at), as the fix for the `set_closure` call (section 3.12 step 9) also needs.

### V-18-11-a  STANDS   (first agent: LOW; refuter: LOW)
- Claim: The sub-code for "has an outcome" is `decided` in the 7.18 block (phase-18.md:1451) and in phase 17 (phase-17.md:1897-1898), but the closed list of section 3.17 has `already_decided`, which section 3.7 step 2 uses for the same condition.
- Evidence: phase-18.md:1451 "has an `outcome` (sub-code `decided`)". phase-18.md:733 "`outcome` already set -> 409 `E_NOT_ALLOWED_IN_STATE` (sub-code `already_decided`". The closed list at phase-18.md:921 holds `already_decided` and no `decided`.
- Fix: Phase 18 section 7.18: use `already_decided` from section 3.17, or add `decided` to that list and align the phase 17 section 7.13 wording.

### V-18-12-a  REFUTED
- Claim: `EntityMask` has no entry point for a target that is not a link yet; the block requires the rule on the owner party of a bare target.
- Evidence: phase-18.md:1461-1462 "the `EntityMask` rule of steps 1 to 3 runs on the target's owner party with the caller's principal, network and purpose". Steps 1 to 3 define the owner party for each target kind (phase-18.md:712 "a `party` link is its own party; an `account` link is the account's current `owner` party") and the rule on that party (phase-18.md:713 "`casework.effective_tier(owner party)`"). The rule is stated in prose for any owner party, and the rule covers methods stated in prose. The Rust signature (phase-18.md:711, `resolve` takes a link row) is a detail, not a gap.

### V-18-12-b  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The deny access event of the 7.19 refusal names no `action`, `resource_type` or `tier`; phase 02 requires all three.
- Evidence (refuted part, tier and resource): phase-03.md:711 "Masked read refusals (a masked kind and a read action): also an access event (`deny`, or `conflict`) through `AccessLog::record`... with the tier from the loaded attributes". Standing part: that rule covers read actions only. The 7.19 refusal sits on a write (phase-18.md:613, `POST /api/v1/cases/{id}/links`, `case.link.write`), so no action is named for its deny event. phase-02.md:151 "action TEXT NOT NULL CHECK (action IN ('view', 'search_hit', 'export', 'print', 'download', 'unmask'))". The tier and resource type can be inferred from section 3.5 step 2 (phase-18.md:713).
- Fix: Phase 18 section 7.19: name the action and the resource type of the deny event written when a link add is refused.

## Summary

- REFUTED: 4 (V-18-07-a, V-18-07-e, V-18-08-a, V-18-12-a)
- STANDS: 12
- PARTLY: 7
- Refuter severity of the 19 STANDS and PARTLY claims: HIGH 1, MEDIUM 8, LOW 10. One claim (V-18-06-b) is raised from LOW to HIGH because two texts contradict on the same route; the lead may downgrade it.

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-18-01-a | PARTLY | LOW | d(ii) names no sub-code for its 409, though section 3.17 says the sub goes in the body | Phase 18 section 3.8 step d(ii) |
| V-18-02-a | PARTLY | MEDIUM | no named writer for the `alert.alert_events` insert (the alert row update is named in phase 17) | Phase 18 section 3.8 step i |
| V-18-03-a | PARTLY | MEDIUM | no check that an evidence `view_key` belongs to this case, no status for a refused key, no numbered steps for POST opened-views | Phase 18 section 3.9 or 3.4 (new steps for the route) |
| V-18-04-a | STANDS | LOW | `CaseChain::append_event` is named, but phase 17 defines `CaseRepository::append_event` with other arguments | Phase 18 section 3 intro (line 624), 3.3 step 5 (line 679), 3.12 step 6 (line 845) |
| V-18-04-b | STANDS | LOW | the step 9 call of `set_closure` differs from the section 2 signature | Phase 18 section 3.12 step 9 or section 2 (line 581) |
| V-18-05-a | PARTLY | MEDIUM | the reject comment passes no `GenericTextGuard` check, though UX-65 requires generic-text refusal on decision forms | Phase 18 section 3.8 route step 1 |
| V-18-05-b | STANDS | LOW | "the step-up prompt of 7.12" points to this file's global search; the step-up dialog is DESIGN-SYSTEM 7.12 | Phase 18 section 5.4 Decide bullet (line 1180) |
| V-18-06-a | STANDS | LOW | `report_sign_required` and `filing_not_acknowledged` are not in the closed sub-code list | Phase 18 section 3.17 closed list (line 921) |
| V-18-06-b | STANDS | HIGH | step e (sign behaves as approve) contradicts 7.3 (refused on this route unless via = "report") | Phase 18 section 3.8 step e (line 759) |
| V-18-07-b | STANDS | MEDIUM | timer kind `case_acl_expiry` is not in the phase 00 timer kind registry | Phase 18 section 4.2 and item 13 of section 4.5 |
| V-18-07-c | STANDS | MEDIUM | job `case_acl_expiry_scan` has no concurrency or attempts, and handler `timer_case_acl_expiry` is not in AppState | Phase 18 section 4.1 table and section 4.3 |
| V-18-07-d | STANDS | LOW | item 6 cites section 3.9 for the rule-5 clause, which is in section 3.10 | Phase 18 section 7.10 item 6 |
| V-18-07-f | STANDS | LOW | the grant method returns `CaseAccessView` (line 653), and section 2 names `AccessView` (line 569) | Phase 18 section 2 views or section 3.2 grant signature |
| V-18-08-b | STANDS | MEDIUM | the insert of the new counterparty link row omits `valid_from`, which is NOT NULL with no default in phase 13 | Phase 18 section 7.11 decide steps |
| V-18-09-a | PARTLY | MEDIUM | the search response type and the party and case row fields are not named (group names and has_more are given) | Phase 18 section 7.12 |
| V-18-09-b | PARTLY | LOW | whether a `limit` above 10 is refused or capped is not stated (the q range is covered by phase 00) | Phase 18 section 7.12 |
| V-18-10-a | STANDS | LOW | step 5 of 3.13 calls `set_reopened(case)` with one argument; section 2 defines three | Phase 18 section 3.13 step 5 (line 861) |
| V-18-11-a | STANDS | LOW | the merge refusal uses sub-code `decided`; the closed list has `already_decided` for the same condition | Phase 18 section 7.18 (line 1451) |
| V-18-12-b | PARTLY | MEDIUM | the deny event for a refused link add names no action (phase 03 step 8 covers read actions only) | Phase 18 section 7.19 (line 1464) |
