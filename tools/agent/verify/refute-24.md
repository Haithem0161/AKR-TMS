# Refuter, phase 24 (Haiku draft for lead review)

### V-24-01-a  STANDS  (first agent: LOW; refuter: MEDIUM)
- Claim: Step 4 lists `case_id` as a filter for both streams, but `audit.access_event` has no `case_id` column, so that filter has no defined meaning on the access_event stream.
- Evidence: phase-24.md:594 "Filters: `stream` (`audit_log`, `access_event` or both)" and "`case_id`, and for `access_event` only `decision` and `tier`". The access_event DDL (phase-02.md:142-168) has no case column; the only one is phase-02.md:122 "case_id UUID, -- filled from phase 17 for case-linked acts", in audit_log. `grep -n "ADD COLUMN" docs/plan/phase-*.md | grep -i access` finds nothing. Step 6 at phase-24.md:596 passes NULL for the case on access_event rows, so a reader can infer that the filter does not apply there, but step 4 never says so. Severity MEDIUM, not LOW, because the gap is inferable only from step 6.
- Fix: Phase 24, section 3.1 step 4: say that `case_id` filters `audit_log` only, since `access_event` has no case column.

### V-24-01-b  REFUTED  (first agent: MEDIUM; refuter: none)
- Claim: No struct for the search input (purpose, reason, from, to, the filters, limit, cursor); `AuditFilter` is used at phase-24.md:539 and never defined.
- Evidence: the brief says a type whose fields are stated in prose is not a gap. phase-24.md:592 "`other` requires `reason` of 1 to 200 characters"; phase-24.md:593 "`from` and `to` on `occurred_at`; default the last 31 days"; phase-24.md:594 "`actor_id`, `resource_type`, `resource_id`, `case_id`"; phase-24.md:596 "page size `min(limit, audit.search_page_size)`". All search inputs are named in steps 2 to 6. The name `AuditFilter` appears only at phase-24.md:539 in the plan, which is a naming point and not a missing field list.

### V-24-02-a  PARTLY  (first agent: MEDIUM; refuter: LOW for the part that stands)
- Claim: the step 2 refusals of `create_entry` (title without `ar` and `en`, severity outside the four, owner not an active governance user, `subject_key` missing) name no HTTP status and no problem code, and the 3.13 Validation sub-codes list none of them.
- Evidence (refuted part, status and problem code): phase-24.md:1057 "`Validation` | 422 `E_VALIDATION` | `window_too_long`, `reason_required`" and "`user_not_active`", so the owner-activity check has a sub-code. phase-00.md:499 "`Validation(Vec<ValidationError>)` | 422 | `E_VALIDATION`" gives the status and code for every validation refusal. Steps 1 and 3 of 3.4 name E_VALIDATION (phase-24.md:644 and 646); the grep for `E_` in lines 643-649 finds only those two, not step 2 itself.
- Evidence (part that stands, sub-codes): the three refusals of step 2 that no sub-code names are title bilingual (phase-24.md:645 "`title` has `ar` and `en`"), severity (phase-24.md:645 "`severity` one of the four") and `subject_key` (phase-24.md:645 "`subject_key` ... is required in `body`"). The error entries carry a `code` (phase-00.md:445-446 "`ValidationError {` / `pointer, code, params }`"), so each refusal needs one. `user_not_active` covers only the active check, not the governance-role check.
- Fix (standing part): Phase 24, section 3.13 Validation row: add one sub-code each for the title, severity and subject_key refusals, and one for the governance-role check of the owner.

### V-24-02-b  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `sync_post_review` names no title (the DDL needs `ar` and `en`), no body keys, and no join from the event to its change request.
- Evidence: phase-24.md:47 "title JSONB NOT NULL CHECK (jsonb_typeof(title) = 'object' AND title ? 'ar' AND title ? 'en')". phase-24.md:665 gives "one finding per concern: severity `high` for `rollback_needed`, `medium` for `concern`; owner = the change request's maker" and names no title or body key (grep "title\|body" on that line finds nothing). The change request has no bilingual title column (admin.change_requests in phase-04.md:121-150 has `reason_text`, not `title`; the `title` at phase-04.md:405 belongs to admin.tasks), so no source for the finding title is stated anywhere. The join is not named: phase-24.md has no `workflow_instances` reference, and `instance_id` appears only inside the source key at phase-24.md:665. Phase 04 gives two possible paths, `admin.workflow_instances.resource_id` (phase-04.md:331-332) and `admin.change_requests.instance_id` (phase-04.md:450), and phase 24 does not say which one the post-review instance uses. The event payload is `{day, outcome}` (phase-04.md:944), so a body could be built, but no body keys are named.
- Fix: Phase 24, section 3.4 sync_post_review: name the source of the bilingual title and of the body keys, and the join from `instance_events.instance_id` to the change request.

### V-24-02-c  STANDS  (first agent: LOW; refuter: LOW)
- Claim: `accepted_risk` names class `approval` but no `StepUpAction { name, class }` constant, and phase 01 says each phase declares its actions as constants.
- Evidence: phase-24.md:653 "`accepted_risk` needs step-up class `approval` (`StepUpService::consume`, resource ref = the row)". phase-01.md:711 "Each phase declares its actions as `StepUpAction { name, class }` constants". `grep -n "StepUpAction" docs/plan/phase-24.md` finds nothing, and no other phase declares a register action name (`grep -n "accepted_risk\|accept_risk" docs/plan/phase-*.md` finds only phase 24 lines). Other phases declare theirs (for example phase-22.md:548 "`StepUpAction { name: "branch.monthly.sign", class: signing }`").
- Fix: Phase 24, section 3.4 update: declare the register accept-risk action as a `StepUpAction` constant with class `approval`, as phase 01 requires.

### V-24-03-a  PARTLY  (first agent: MEDIUM; refuter: LOW for the part that stands)
- Claim: the job `review_calendar_sweep` has no named method (section 4.3 gives only `review_sweep: Arc<GovernanceRegisterService>`, and 3.4 describes the sweep in one sentence).
- Evidence (refuted part): the calls the sweep makes are named. The rows come from the sources of 3.4 (phase-24.md:657-662) and the warning is written by the port `RegisterWriter::warn(tx, source, source_key, title_key: WarningTitle, body)`, which "this phase's own sweeps use it" (phase-24.md:1297 and 1301-1302). The source key of the sweep is given at phase-24.md:663 "`source_key = 'review:' || kind || ':' || subject_id || ':' || due_on`".
- Evidence (part that stands): the job's own entry method is not named anywhere. `grep -n "sweep" docs/plan/phase-24.md` finds lines 9, 475, 476, 663, 1093, 1129, 1143, 1156, 1251 and 1302, and none of them names a method; and `grep -n "sweep(" docs/plan/phase-24.md` finds nothing. The `review_calendar` route method takes a caller `ctx` (phase-24.md:657), which a job does not have; the plan does not say how the job reads the rows.
- Fix (standing part): Phase 24, section 3.4 sweep sentence or section 4.1 row: name the `GovernanceRegisterService` method the job runs, and say which row reader it uses without a caller context.

### V-24-03-b  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the calendar defines red two ways: "red past due" (3.4) and "red on the due date" for the 7.3 rows. The sweep opens a warning on red, so the boundary day decides whether a warning is written.
- Evidence: phase-24.md:657 "`state` (`ok`, `amber` within 30 days, `red` past due)" states the state for every row the calendar returns. phase-24.md:1277 "Amber 30 days before, red on the due date." is in 7.3, which adds three derived rows to "Review calendar (section 3.4)". 7.3 does not say that it replaces the 3.4 state rule. DBS decision 231 (phase-24.md:1471) "only red rows become warnings" does not fix the boundary. The scenario rule at phase-24.md:662 (red at two intervals) is a per-row rule and does not settle the general question. No 7.x block says which rule wins.
- Fix: Phase 24, section 3.4 `review_calendar` or 7.3: state one red boundary for the calendar and say whether 7.3 changes it for its three rows.

### V-24-03-c  STANDS  (first agent: LOW; refuter: LOW)
- Claim: `review_calendar(ctx, horizon_days)` takes `horizon_days` but no step and no route row uses it.
- Evidence: phase-24.md:657 "`review_calendar(ctx, horizon_days)` (`register.read`; route 13)" is the only use in the calendar text. Route 13 at phase-24.md:559 "`GET /api/v1/registers/review-calendar` | `dashboard.read` | Rules, exclusions, change-request reviews, readiness and scenario review dates" names no horizon parameter. The other hits (phase-24.md:468 "`command.clock_horizon_days`" and phase-24.md:736) belong to the clocks board and the command centre, not to this calendar. `grep -n "horizon" docs/plan/phase-24.md` confirms.
- Fix: Phase 24, section 3.4 `review_calendar`: say what `horizon_days` filters and its bounds, or drop it from the signature.

### V-24-04-a  PARTLY  (first agent: MEDIUM; refuter: LOW for the part that stands)
- Claim: no method is named for the board builder or for the aggregates it takes from 3.4 to 3.7; section 4.3 says only that the five kinds are registered through the phase 06 handler.
- Evidence (refuted part): section 2 names the readers. phase-24.md:541 "`BoardSource::periodic_final(period) -> Option<PeriodicReport>` (phase 21)" is the periodic source of the board. phase-24.md:540 "`CommandReadRepository` and `ClockReadRepository` (`akr_app`): one method per tile and per clock source of section 3.7 and 3.8" names the readers of the 3.7 aggregates. phase-24.md:533 "`RegisterRepository`: ... `list(conn, RegisterFilter, Keyset)`" names the register read. The `grep "Builder"` hit at phase-24.md:522 is the statement that no `PackBuilder` exists, which is a naming choice.
- Evidence (part that stands): the function that assembles the board payload from these readers has no name, and the aggregates of 3.4 to 3.7 are not mapped one by one to a method (phase-24.md:540 says only "one method per tile"). The predicate the builder applies is the subject of V-24-04-b.
- Fix (standing part): Phase 24, section 3.11 board row: name the payload builder and say which reader method feeds each aggregate.

### V-24-04-b  STANDS  (first agent: HIGH; refuter: HIGH)
- Claim: the board pack is read by M, CH and IA and its aggregates are "of 3.4 to 3.7", but several 3.7 tiles are visible to M only and the builder's predicate and tile list are not stated, so M-only counts would reach CH and IA.
- Evidence: phase-24.md:799 "plus aggregates of 3.4 to 3.7 | M | M, CH, IA". The M-only tiles: phase-24.md:723 "Productive alerts" with "| M |"; phase-24.md:724 "UTRs and STRs" with "| M |"; phase-24.md:730 "Tipping-off summary ... | M and SEC only; the tile is not sent to CH or IA"; phase-24.md:718 "(suspicion subjects: M)". The two texts that conflict: phase-24.md:1110 "nothing: suspicion counts are returned to M only" and phase-24.md:1112 "Board pack (3.11) | M, CH, IA | aggregates and periodic facts ... | nothing". The closest designed rule is DBS decision 227 (phase-24.md:1463) "a board reader who holds the pack learns the totals the MLRO's board already receives", but it says "aggregates only" without naming the tiles. `grep -n "predicate\|tile" docs/plan/phase-24.md` in lines 793-816 finds nothing. No 7.x block changes the board pack.
- Fix: Phase 24, section 3.11 board row: name the 3.7 tiles the board pack takes and the predicate its builder applies, so no M-only count reaches a CH or IA reader.

### V-24-05-a  PARTLY  (first agent: MEDIUM; refuter: MEDIUM for the part that stands)
- Claim: the case build (section 3.11, row `case`) has no refusal for an unknown case id, a case the caller cannot read, or a job failure reason for either; only the board has a named failure reason (phase-24.md:805).
- Evidence (refuted part): the failure audit is a general rule. phase-06.md:691-692 "Failures audit `pack.build_failed {kind, reason}`" covers the `pack_build` job, and phase-24.md:1091 lists `pack.build_failed` among the audit events of the five new kinds. A case the caller cannot read is refused at the Cedar gate: phase-24.md:800 "| `case` | `{case_id}` |" with "| M | M |" (create and read M only), and phase-24.md:1052 "Cedar deny on a resource the caller may know exists" is the 403 `E_FORBIDDEN` rule.
- Evidence (part that stands): POST /packs answers 202 and the build runs in the job (phase-06.md:683-685), so an unknown case id has no named synchronous refusal and no named status. The 404 row at phase-24.md:1055 "`NotFound` | 404 `E_NOT_FOUND`" is not tied to the case kind, and `grep -n "E_NOT_FOUND" docs/plan/phase-24.md` in lines 795-816 finds nothing. The failure reason value for a case build is not named.
- Fix (standing part): Phase 24, section 3.11 case row: say whether an unknown case id is refused at POST /packs (with its status) or fails in the job, and name the reason the failure audits.

### V-24-05-b  STANDS  (first agent: LOW; refuter: LOW)
- Claim: the read model is cited as "of phase 18" (phase-24.md:541, 800), but the route `GET /api/v1/cases/{id}` is phase 17's, phase 18 only extends its fields, and no function is named in either phase.
- Evidence: phase-17.md:849 "| GET | `/api/v1/cases/{id}` | `queue.work` | Case header: class, tier, status projection, alerts merged, clocks." phase-18.md:618 "Phase 17's `GET /cases/{id}` gains the fields listed in section 4.5, not a new row." phase-24.md:541 "`CaseFileSource` (the read model behind `GET /cases/{id}` of phase 18, not a new query)". `grep -n "CaseFileSource\|CaseReadModel\|case read model\|read model" docs/plan/phase-17.md docs/plan/phase-18.md` finds nothing, and no service method is named for `GET /cases/{id}` in either phase.
- Fix: Phase 24, section 2 and section 3.11 case row: cite phase 17 as the route owner and name the service method that `CaseFileSource` wraps.

### V-24-06-a  PARTLY  (first agent: LOW; refuter: LOW for the part that stands)
- Claim: 7.1 says `assessor.grant_max_days` is not created, but the base seed row still creates it, and 3.16 and the lead correction still name it, so the migration would create an unused governed setting.
- Evidence (refuted part): phase-24.md:1260 "The setting `assessor.grant_max_days` is not created: the cap of an `AS` grant is phase 04's". The 7.x block says what it replaces ("Section 1's seed row and section 3.9 read accordingly"), so under the gap-pass rule 7.1 wins over the base text and the migration does not create the key.
- Evidence (part that stands, stale text): the seed row still inserts it at phase-24.md:464 "('assessor.grant_max_days',". Section 3.16 still lists it as a phase 24 key at phase-24.md:1122 "`assessor.grant_max_days` (30; governed; floor max 30)". Section 3.9 still names it at phase-24.md:760 and the lead correction at phase-24.md:1473 still gives it a floor. A reader of section 1.4 alone would create it.
- Fix (standing part): Phase 24, section 1.4 seed rows and section 3.16: remove the `assessor.grant_max_days` row and name the phase 04 key `access.assessor_max_days` in its place.

### V-24-07-a  PARTLY  (first agent: MEDIUM; refuter: MEDIUM for the part that stands)
- Claim: route 27 (`GET /api/v1/assessor/samples/{id}`, phase-24.md:573) returns the seed, frame hash and items, but names no access event or audit kind for the read, although S-56 asks for "a record of what was shown". Section 3.9 says only "every item later viewed is logged".
- Evidence (refuted part, the logging is designed): DBS decision 226 (phase-24.md:1461) "each item shown and each node of a trace is logged with purpose `supervisory_inspection`". Section 3.9 step 2 (phase-24.md:766) "every item later viewed is logged". S-56 is research/27-transaction-monitoring-scenarios.md:2405 "a record of what was shown". Route 27 is a sensitive read, so the general extractor `AccessScope` (phase-02.md:719) writes the access event before the response.
- Evidence (part that stands): the route row (phase-24.md:573) and section 3.14 (phase-24.md:1086-1088, `assessor.matrix_viewed`, `assessor.sample_drawn`, `assessor.trace_viewed`) name no sample-read event. `grep -n "sample_viewed\|sample_read" docs/plan/phase-24.md` finds nothing. Section 3.9 says "viewed" (phase-24.md:766) and DBS decision 226 says "shown" (phase-24.md:1461): two wordings of the trigger, and no 7.x block chooses between them.
- Fix (standing part): Phase 24, section 3.9 step 2 and the route 27 row: say the sample read writes one access event per item shown, with its action and purpose.

### V-24-07-b  STANDS  (first agent: MEDIUM; refuter: LOW)
- Claim: route 27 has no service method. `SampleRepository::get(conn, id)` is named (phase-24.md:537), but no `AssessorService` method calls it.
- Evidence: phase-24.md:537 "`SampleRepository`: `insert(tx, AssessorSample)`, `get(conn, id)`". The route row (phase-24.md:573) names no method. The service methods of 3.9 are `d2_matrix(ctx)` (phase-24.md:762), `draw(ctx, frame, from, to, sample_size, idem)` (phase-24.md:764) and `trace(ctx, alert_id)` (phase-24.md:775). `grep -n "samples/{id}" docs/plan/phase-24.md` finds 573 and 773 only, and `grep -n "sample(ctx\|samples(ctx"` finds nothing. The repository and the Cedar action (`assessor.sample`, phase-24.md:573) are named, so the missing piece is only the service method's name.
- Fix: Phase 24, section 3.9: name the `AssessorService` method for route 27 and say that it calls `SampleRepository::get`.

### V-24-08-a  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `GovernanceRegisterService` names `create_entry`, `update`, `review_calendar` and `sync_post_review` but no list or read method, and section 2 lists no repository for `casework.decisions` or `admin.exclusions`.
- Evidence: route 9 (phase-24.md:555) "`GET /api/v1/registers/{kind}` | `dashboard.read` | Registers `decisions` (read view), `exclusions` (read view), `findings`, `warnings`, `fairness`". Section 2 names a register repository (phase-24.md:533 "`list(conn, RegisterFilter, Keyset) -> Page<Register>`") but no service method calls it, and the repository list (phase-24.md:530-541) has no repository for `casework.decisions` or `admin.exclusions`. Section 3.4 describes the two read views (phase-24.md:637-638) by their columns, joins and predicate, but names no repository or method. `grep -n "list(ctx" docs/plan/phase-24.md` finds only the training register (phase-24.md:790).
- Fix: Phase 24, sections 2 and 3.4: name the service list method for route 9 and the repository that reads `casework.decisions` and `admin.exclusions` with the visibility predicate.

### V-24-08-b  STANDS  (first agent: LOW; refuter: LOW)
- Claim: an unknown `kind` on the GET is not refused. Route 9 lists the kinds (phase-24.md:555) and section 3.4 gives no answer for another value.
- Evidence: phase-24.md:555 lists five kinds, and phase-24.md:636-641 (section 3.4 "Kinds") lists four. No status or code is named for another value. `grep -n "unknown\|not a kind" docs/plan/phase-24.md` in lines 634-665 finds nothing. The general `NotFound` rule (phase-00.md:494 "`E_NOT_FOUND` (one body for missing and masked, phase 03)") does not name an unknown path value as a missing resource.
- Fix: Phase 24, section 3.4 Kinds: state the status and code for a `kind` value outside the list.

### V-24-08-c  PARTLY  (first agent: MEDIUM; refuter: MEDIUM for the part that stands)
- Claim: section 3.5 says `fairness.read` is "M, CH, IA and MRO" (phase-24.md:686), and 3.15 says registers and fairness are read by "M, CH, IA, MRO" (phase-24.md:1108). The Cedar policies `gov.read.m`, `gov.read.ch` and `gov.read.ia` admit M, CH and IA only.
- Evidence (refuted part): section 3.12 is the fine gate. phase-24.md:819 "**Actions and roles** (the fine gate; scopes of section 2 are the coarse gate):". So Cedar decides the access, and MRO is refused `fairness.read` by the policies at phase-24.md:898-911 ("`gov.read.m`", "`gov.read.ch`", "`gov.read.ia`", each with `fairness.read` and one role).
- Evidence (part that stands): the prose still gives MRO a read of fairness at phase-24.md:686 "`fairness.read` is M, CH, IA and MRO." and at phase-24.md:1108 "| Fairness (3.5) | M, CH, IA, MRO |". MRO reviews a metric at route 15 (phase-24.md:682 "`fairness.review`, roles M and MRO") and the Cedar policy `fairness.review.mro` (phase-24.md:940-942) gives the review action only, not the read. No 7.x block settles whether MRO reads fairness metrics.
- Fix (standing part): Phase 24, sections 3.5 and 3.15 and the 3.12 policies: decide whether MRO reads fairness metrics (add the read to the MRO policy or remove MRO from the prose) so the review in route 15 has the metric visible.

### V-24-09-a  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the closed enum `WarningTitle` names no variant for this phase's warnings (the review rows at phase-24.md:663 and the fairness rows at phase-24.md:678) and no bilingual title. Phases 26 and 29 add variants to phase 24's enum, and the title must hold `ar` and `en` (phase-24.md:47).
- Evidence: phase-24.md:1300 "`WarningTitle` is a closed enum in `domain::governance` that maps each key to `title {ar, en}`", then "later phases add their variants (phase 26, phase 29)". No variant of phase 24 is listed. phase-24.md:663 "opens one warning per row that turns `red`" and phase-24.md:678 "a warning of source `fairness`" name no title key. The later phases confirm the gap: phase-26.md:882 "This phase adds the variant `IslamicEvidenceGap` to phase 24's closed enum `WarningTitle`" and phase-29.md:1480 "This phase adds seven variants to phase 24's closed enum `WarningTitle`". `grep -n "WarningTitle" docs/plan/phase-24.md` finds 1298 and 1300 only. DBS decision 388 (docs/plan/research.md:399) names the writer, not the variants. The DDL title CHECK at phase-24.md:47 requires `ar` and `en`.
- Fix: Phase 24, section 7.7: list the variants this phase uses, one per warning source, each with its bilingual title.

### V-24-09-b  STANDS  (first agent: LOW; refuter: LOW)
- Claim: section 7.7 says the port "is called only from jobs (section 1.1)" (phase-24.md:1301), but section 1.1 is the 0076 DDL and the job kinds are in section 4.1.
- Evidence: phase-24.md:1301 "It is called only from jobs (section 1.1), and". phase-24.md:33 "### 1.1 `0076_admin_registers_validation.sql`" is the DDL; phase-24.md:1137 "### 4.1 Job kinds (Postgres `jobs`; coalesce key in brackets)" holds the job kinds.
- Fix: Phase 24, section 7.7: change the cross-reference from section 1.1 to section 4.1.

### V-24-10-a  STANDS  (first agent: HIGH; refuter: HIGH)
- Claim: the control report is computed on `readonly_pool` and its manual-grant and expiry counts read `casework.case_acl` and `casework.cases`, but `akr_readonly` has no table or column grant on casework. No SECURITY DEFINER function or pool change is named for these counts.
- Evidence: phase-24.md:615 "one month, counts only, computed on `readonly_pool`:". phase-24.md:584 "Phase 24 therefore runs every query of sections 3.1 to 3.3 on `readonly_pool`". phase-24.md:619 "manual grants on suspicion-tier cases made and expired in the month (phase 18 `casework.case_acl`; section 7+)". phase-24.md:1324 "(`casework.case_acl`, `reason = 'manual'`, case `tier = 'suspicion'`, `granted_at` in the month)". The grant statement at phase-24.md:303 "No table or column grant to akr_readonly on casework, alert, scn or canon (lead review): the function reads them as its owner." The only definer named is the tier resolver (phase-24.md:256 "LANGUAGE sql STABLE SECURITY DEFINER", and the lead correction at phase-24.md:1473). `grep -n "SECURITY DEFINER" docs/plan/phase-24.md` finds 8, 217, 248, 256, 1175 and 1473 only. phase-17.md:568 and phase-18.md:443 both say "No grant to akr_readonly". The tier resolver cannot serve these counts, since it takes one resource, not a count.
- Fix: Phase 24, section 3.2 control report: name the SECURITY DEFINER function or the pool that runs the two case counts, since `akr_readonly` cannot read casework.

### V-24-10-b  STANDS  (first agent: LOW; refuter: LOW)
- Claim: section 7.11 cites "the access-request register of phase 18 section 7+" (phase-24.md:1326). The phrase is not in phase-18.md; the concept is section 7.10 "Time-boxed manual grants" (phase-18.md:1323).
- Evidence: phase-24.md:1326 "of the access-request register of phase 18 section 7+". `grep -n "access-request" docs/plan/phase-18.md` finds nothing. phase-18.md:1323 "### 7.10 Time-boxed manual grants on suspicion-tier cases". The phrase sits in the research files, not in phase 18: research/30-security-privacy.md:1049 and research/31-analyst-ux.md:670 (the first agent said it appears only in DBS decision 395 at docs/plan/research.md:406; that is slightly off, since the other two files also use it, but the wrong phase attribution stands).
- Fix: Phase 24, section 7.11: replace the phrase with a citation of phase 18 section 7.10.

### V-24-11-a  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the two GET rows of the export routes name no Cedar action. Step 1 has the route check the list route's scope and Cedar action, but does not name that check for either GET row.
- Evidence: phase-24.md:1336 "| GET | `/api/v1/exports/{id}` | the list route's scope | Status of the caller's own export (New; requester only) |" and phase-24.md:1337 "| GET | `/api/v1/exports/{id}/file` | the list route's scope | The file; step-up class `export`; access event `export` first (New; requester only) |". The POST row names its action: phase-24.md:1335 "(New; the list route's Cedar action)". Step 1 at phase-24.md:1340 says "The route checks that list route's scope and Cedar action for the caller". The general rule is phase-03.md:485-486, "every route has the Cedar action in the last column of section 3's action table", so each GET row needs one. `awk` over phase-24.md:1330-1352 finds "Cedar" at 1335 and 1340 only.
- Fix: Phase 24, section 7.12 route table: name the Cedar action for the status and file rows, as the POST row does.

### V-24-11-b  PARTLY  (first agent: MEDIUM; refuter: MEDIUM for the part that stands)
- Claim: no handler or service method is named for the POST, the two GETs or the job. The "screen's own list service" that the job calls has no named method. The request `{register, filters, format}` is not a named type.
- Evidence (refuted part, the request type): the fields are stated in the route row, phase-24.md:1335 "Start an export `{register, filters, format: csv or xlsx}`". The brief says a type whose fields are stated in prose is not a gap, so no request struct is needed.
- Evidence (part that stands): phase-24.md:1342-1343 "it calls the screen's own list service as the requester" names no method. `grep -n "ExportService\|export(" docs/plan/phase-24.md` finds nothing. No handler or service method is named for the POST (phase-24.md:1335), the GET rows (phase-24.md:1336-1337) or the job `register_export` (phase-24.md:1342).
- Fix (standing part): Phase 24, section 7.12: name the export service that holds the POST, the GET rows and the job, and the list method each REGISTER screen gives the job.

### V-24-11-c  PARTLY  (first agent: MEDIUM; refuter: MEDIUM for the part that stands)
- Claim: step 1 says "anything else is 422 at `/register`" (phase-24.md:1340-1341) with no problem code, and the file route (phase-24.md:1348) gives no response while the scan is pending or the file is not clean.
- Evidence (refuted part, the code): a 422 refusal at a body path is a validation error. phase-00.md:499 "`Validation(Vec<ValidationError>)` | 422 | `E_VALIDATION` (`errors[]` with JSON pointers)". API.md:37 "Validation errors carry `errors[]` with JSON-pointer paths". So the code is `E_VALIDATION` and the pointer is `/register`.
- Evidence (part that stands, the file route): phase-24.md:1348 "and is served only once clean." names no status and no problem code for a pending or unclean file. The file row (phase-24.md:1337) names none either.
- Fix (standing part): Phase 24, section 7.12 step 4: name the status and the problem code the file route returns while the scan is pending or the file is not clean.

### V-24-11-d  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `register_export` is not in the job table (phase-24.md:1139-1145) and has no coalesce key, concurrency, attempts or enqueue transaction (tools/agent/b3-conventions.md:61 lists them as required).
- Evidence: the table at phase-24.md:1139 has the columns "Concurrency, attempts" and the coalesce key in brackets. Its rows (phase-24.md:1141-1145) are `audit_verify_run`, `register_post_review_sync`, `review_calendar_sweep`, `fairness_monthly` and `pack_build`. No `register_export` row. `grep -n "register_export" docs/plan/phase-24.md` finds 1342 and 1347 only. The enqueue point is named for the other rows, for example phase-24.md:1141 "enqueued in the request transaction of `POST /audit/verify-runs`", but not for `register_export`. tools/agent/b3-conventions.md:61 "kind, binary, payload, coalesce key, concurrency, attempts, schedule or trigger".
- Fix: Phase 24, section 4.1: add a `register_export` row with its coalesce key, concurrency, attempts and the enqueue transaction of `POST /exports`.

### V-24-11-e  STANDS  (first agent: LOW; refuter: LOW)
- Claim: the file route needs step-up class `export` (phase-24.md:1337), but no `StepUpAction` constant is declared for it.
- Evidence: phase-24.md:1337 "The file; step-up class `export`; access event `export` first". `grep -n "StepUpAction" docs/plan/phase-24.md` finds nothing. phase-01.md:711 "Each phase declares its actions as `StepUpAction { name, class }` constants". The route-table rule of phase-03.md:485 says "SU" names the step-up action and class, so the action name is also required. The only constant for a pack download is phase 06's (phase-06.md:978, `pack.download`), which phase 24 does not declare.
- Fix: Phase 24, section 7.12: declare the file-download `StepUpAction` constant with class `export`.

### V-24-11-f  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the owner kind `register_export` is not in the evidence owner registry. Phase 02 requires each owner kind to be listed by its phase, and phase 24 section 4.5 item 1 and Open doubts 29 list only `validation_pack` and `training_record`.
- Evidence: phase-02.md:847 "`owner_kind` must be in the evidence owner registry (`pack` now; every owning phase adds its kind)". phase-24.md:1167 "`EvidenceStore` owner kinds gain `validation_pack` (3.6) and `training_record` (3.10), each with its `EvidenceResolver`." phase-24.md:1505 "`validation_pack` and `training_record` are new owner kinds of phase 02's registry". phase-24.md:1347 uses "owner kind `register_export`". `grep -n "register_export" docs/plan/phase-02.md docs/plan/research.md research/*.md` finds nothing.
- Fix: Phase 24, section 4.5 item 1 and Open doubts 29: add `register_export` to the owner kinds, with its EvidenceResolver.

### V-24-11-g  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `NewEvidence` requires `retention_class` and `classification`, and `admin.evidence_objects` has both NOT NULL. A retention class needs an `evidence.lock_days` entry. Step 4 gives only the kind, owner kind and owner id.
- Evidence: phase-02.md:358 "retention_class ... -- the owner's class" (the class comes from the owner, and `register_export` has none). phase-02.md:366 "classification    admin.data_classification NOT NULL,". phase-02.md:611-612 "NewEvidence { kind, owner_kind, owner_id, retention_class, content_type, file_name: Option<String>, classification, lang, bytes: Bytes }". phase-02.md:848 "`retention_class` must have an entry in `evidence.lock_days` -> `NoLockPolicy`". Step 4 at phase-24.md:1347-1348 names "kind `render`, owner kind `register_export`, owner id = the export id" and no class. No default class for kind `render` exists in phase 02 (`grep -n "'render'" docs/plan/phase-02.md` finds only the kind CHECK at line 355).
- Fix: Phase 24, section 7.12 step 4: name the retention class and the classification of the export file, and say whether that class already has an `evidence.lock_days` entry.

### V-24-12-a  PARTLY  (first agent: MEDIUM; refuter: MEDIUM for the part that stands)
- Claim: no method is named for the explanation. `ParallelRunService` is named with steps only (phase-24.md:1408-1427), and step 3 has no signature, no repository write and no named request type for `{code, note}`.
- Evidence (refuted part, the request type): step 3 names the fields and the write, phase-24.md:1422-1423 "Set the code, note, `explained_by` and `explained_at`", and the route row at phase-24.md:1402 says "Record the explanation code and note of one disagreement". The brief says a type whose fields are stated in prose is not a gap, so no `{code, note}` struct is needed.
- Evidence (part that stands): no method name is given for the explanation. `grep -n "explain" docs/plan/phase-24.md` finds no method. `grep -rn "ParallelRunService\|ParallelRunRepository" docs/plan/*.md` finds phase-24.md:1408 only, and section 2 (phase-24.md:519-545) names no repository for `alert.parallel_run_rows`, so the write has no named owner.
- Fix (standing part): Phase 24, section 7.13: name the service method for the explanation and the repository method that writes `alert.parallel_run_rows`.

### V-24-12-b  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: two explainers can race. Step 3 checks "not yet explained" and then sets the fields. No `FOR UPDATE`, `ON CONFLICT` or guarded `WHERE` appears in 7.13, and section 3.17 does not cover this table, so the second write silently replaces the first.
- Evidence: phase-24.md:1421 "and not yet explained (else 409", then phase-24.md:1422-1423 "Set the code, note, `explained_by` and `explained_at`." The awk search over phase-24.md:1356-1439 finds no `FOR UPDATE`, `ON CONFLICT` or guarded write (its only hit is the migration name `0079_admin_guards_grants`). Section 3.17 (phase-24.md:1125-1133) lists the inventory row, the case row, `admin.registers`, `admin.verify_runs`, the pack builds and the outbox, and not `alert.parallel_run_rows`. The table DDL in 7.13 has no version column, and the POST row (phase-24.md:1402) names no `If-Match`.
- Fix: Phase 24, section 7.13 step 3: make the write conditional on the row still being unexplained (a row lock or a guarded update) and name the error the second caller gets.

### V-24-12-c  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the DDL comment says the note "passes PolicyPackService::scan" (phase-24.md:1380), but step 3 does not call the scan.
- Evidence: phase-24.md:1380 "-- passes PolicyPackService::scan". Step 3 (phase-24.md:1421-1423) names no scan call. Other free-text writes of this phase name the call, for example phase-24.md:646 "The text fields of `title` and `body` pass `PolicyPackService::scan` (phase 09, scope `reason_text`)". `awk` over phase-24.md:1421-1423 finds no "scan". The only general scan rule found, phase-09.md:797 "through `PolicyPackService::scan(scope, text)` at save time", is for the texts phase 09 saves, so it does not cover this note.
- Fix: Phase 24, section 7.13 step 3: state the scan call on the note and the scope it uses.

### V-24-12-d  PARTLY  (first agent: LOW; refuter: LOW for the part that stands)
- Claim: refusals for a code outside the seven values and for a note over 2000 characters have no problem code. The CHECKs at phase-24.md:1372 and 1380 enforce them only in the database.
- Evidence (refuted part, the status and problem code): phase-00.md:499 "`Validation(Vec<ValidationError>)` | 422 | `E_VALIDATION`", and API.md:985 "`E_VALIDATION` | 422 | Field or document validation". So both refusals are 422 `E_VALIDATION` by the general mapping. The explanation CHECK is at phase-24.md:1378 (not 1372 as the first agent wrote) and the note CHECK at phase-24.md:1380 "CHECK (note IS NULL OR length(note) BETWEEN 1 AND 2000)".
- Evidence (part that stands, the sub-code): each error entry carries a `code` (phase-00.md:445-446 "`ValidationError {` ... `pointer, code, params }`"), and the 3.13 Validation row (phase-24.md:1057) lists no sub-code for either refusal. Step 3 names a sub-code for neither.
- Fix (standing part): Phase 24, section 3.13 Validation row: add one sub-code for an explanation code outside the list and one for a note over 2000 characters.

### V-24-12-e  STANDS  (first agent: LOW; refuter: LOW)
- Claim: the Cedar policy for `parallel_run.write` is prose only (phase-24.md:1405-1406). Section 3.12 has no policy text for it, and no parallel-run entity is declared in the schema additions (phase-24.md:817-1047).
- Evidence: phase-24.md:1405-1406 "**Cedar** (`C0_governance.cedar`, one role per policy): `parallel_run.read` for M and CH; `parallel_run.write` for M only." `awk` over phase-24.md:817-1047 finds no "parallel". The fine-gate rule at phase-24.md:819 "**Actions and roles** (the fine gate" makes the policy text the gate, and it is missing. The policy's resource type is not named either.
- Fix: Phase 24, section 3.12: add the two parallel-run policies and declare the resource entity the parallel-run actions use.

## Summary

Counts: REFUTED 1, STANDS 22, PARTLY 11 (34 Gap IDs, each handled once). Severity in the table is the refuter's severity for the part that stands.

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-24-01-a | STANDS | MEDIUM | `case_id` filter on the access_event stream has no column | Phase 24, section 3.1 step 4 |
| V-24-02-a | PARTLY | LOW | Sub-codes for the title, severity, subject_key and governance-role refusals | Phase 24, section 3.13 Validation row |
| V-24-02-b | STANDS | MEDIUM | `sync_post_review` has no title source, no body keys and no event-to-change-request join | Phase 24, section 3.4 sync_post_review |
| V-24-02-c | STANDS | LOW | No StepUpAction constant for the accepted_risk step-up | Phase 24, section 3.4 update |
| V-24-03-a | PARTLY | LOW | The job's own entry method and the row reader it uses | Phase 24, section 3.4 sweep sentence and section 4.1 |
| V-24-03-b | STANDS | MEDIUM | Two boundaries for red (past due in 3.4; on the due date in 7.3) | Phase 24, section 3.4 review_calendar or 7.3 |
| V-24-03-c | STANDS | LOW | The `horizon_days` parameter has no use | Phase 24, section 3.4 review_calendar |
| V-24-04-a | PARTLY | LOW | The board payload builder is not named; no aggregate-to-reader map | Phase 24, section 3.11 board row |
| V-24-04-b | STANDS | HIGH | Board pack aggregates of 3.4 to 3.7 can carry M-only counts to CH and IA; predicate and tile list unstated | Phase 24, section 3.11 board row |
| V-24-05-a | PARTLY | MEDIUM | Unknown case id: no synchronous refusal and no failure reason for case packs | Phase 24, section 3.11 case row |
| V-24-05-b | STANDS | LOW | Read model attributed to phase 18 (the route is phase 17's); service function unnamed | Phase 24, sections 2 and 3.11 |
| V-24-06-a | PARTLY | LOW | Stale seed row, 3.9, 3.16 and lead correction still name assessor.grant_max_days | Phase 24, sections 1.4 and 3.16 |
| V-24-07-a | PARTLY | MEDIUM | Sample-read access event: trigger wording ("shown" vs "viewed") and action not named | Phase 24, section 3.9 step 2 and route 27 |
| V-24-07-b | STANDS | LOW | No AssessorService method for route 27 | Phase 24, section 3.9 |
| V-24-08-a | STANDS | MEDIUM | No list or read method for route 9; no repository for decisions or exclusions | Phase 24, sections 2 and 3.4 |
| V-24-08-b | STANDS | LOW | No status or code for an unknown kind on GET /registers/{kind} | Phase 24, section 3.4 Kinds |
| V-24-08-c | PARTLY | MEDIUM | Prose gives MRO a fairness read that Cedar denies; MRO review (route 15) needs the metric | Phase 24, sections 3.5, 3.15 and 3.12 |
| V-24-09-a | STANDS | MEDIUM | No WarningTitle variant or bilingual title for this phase's warnings | Phase 24, section 7.7 |
| V-24-09-b | STANDS | LOW | Cross-reference to section 1.1 should be section 4.1 | Phase 24, section 7.7 |
| V-24-10-a | STANDS | HIGH | Control report case counts run on readonly_pool with no casework grant and no definer function | Phase 24, section 3.2 control report |
| V-24-10-b | STANDS | LOW | Wrong phase 18 citation for the access-request register | Phase 24, section 7.11 |
| V-24-11-a | STANDS | MEDIUM | The GET export status and file rows name no Cedar action | Phase 24, section 7.12 route table |
| V-24-11-b | PARTLY | MEDIUM | No handler, service or list method named for the export routes and the job | Phase 24, section 7.12 |
| V-24-11-c | PARTLY | MEDIUM | File route response while the scan is pending or the file is unclean | Phase 24, section 7.12 step 4 |
| V-24-11-d | STANDS | MEDIUM | `register_export` missing from the job table (no coalesce key, concurrency, attempts or enqueue point) | Phase 24, section 4.1 |
| V-24-11-e | STANDS | LOW | No StepUpAction constant for the export file download | Phase 24, section 7.12 |
| V-24-11-f | STANDS | MEDIUM | `register_export` is not in the evidence owner registry | Phase 24, section 4.5 item 1 and Open doubts 29 |
| V-24-11-g | STANDS | MEDIUM | Retention class and classification of the export file are not given | Phase 24, section 7.12 step 4 |
| V-24-12-a | PARTLY | MEDIUM | No explanation service method and no repository write for `alert.parallel_run_rows` | Phase 24, section 7.13 |
| V-24-12-b | STANDS | MEDIUM | Two explainers can overwrite each other; no guarded write or row lock | Phase 24, section 7.13 step 3 |
| V-24-12-c | STANDS | MEDIUM | Note scan promised by the DDL comment is not called in step 3 | Phase 24, section 7.13 step 3 |
| V-24-12-d | PARTLY | LOW | No sub-code for an unknown explanation code or a note over 2000 characters | Phase 24, section 3.13 Validation row |
| V-24-12-e | STANDS | LOW | Cedar policy text and resource entity for `parallel_run.write` are missing | Phase 24, section 3.12 |
