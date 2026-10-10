# Verification pass, phase 24 (Haiku draft for lead review)

Scope: docs/plan/phase-24.md (1525 lines). Method: 12 sampled items (5 header items from the start, middle and end of
the "Requirements carried" list; 4 items from different 7.x blocks of the gap pass; 3 further header items, because the
phase has no "Adversarial pass (2026-10-10), run N" block: `grep -ci dversar docs/plan/phase-24.md` returns 0).

## Sample (listed before checking)

Header items, five spread over the list (first five) and three more (the phase has no adversarial block):
- V-24-01 SP-42 audit viewer: every search writes `audit.searched` (header line 12, start of list; section 3.1)
- V-24-02 GV-34 findings register with owner, due date, repeat flag, status (header line 12; section 3.x register)
- V-24-03 S-46 review calendar of rules, exclusions, change requests, readiness, scenarios (header line 13, middle; section 3.4)
- V-24-04 B-11 board pack builder with counts through each source predicate (header line 16, middle-late; section 3.11)
- V-24-05 VC-82 case and correspondent packs with hashes (header line 17, end of list; section 3.11)
- V-24-06 M-62 assessor workspace and time-boxed read-only grant (header line 12; sections 3.9)
- V-24-07 S-56 read-only assessor role with exports and a record of what was shown (header line 13; section 3.9)
- V-24-08 L-57 registers, decisions and exclusions as read views (header line 17; section 3.4)

Gap-pass 7.x items, four from four different blocks (no adversarial block exists):
- V-24-09 7.7 register warning writer `RegisterWriter::warn` (DR-388)
- V-24-10 7.11 manual case grants and expired grants counts in the control report (SP-28, DR-395)
- V-24-11 7.12 register exports: `POST /api/v1/exports`, `GET /api/v1/exports/{id}`, `GET /api/v1/exports/{id}/file`, job `register_export` (UX-31, DR-400)
- V-24-12 7.13 parallel run: `POST /api/v1/parallel-run/rows/{id}/explanation` (VA-54, VO-03, DR-402)

### V-24-01 SP-42 audit viewer search, every search writes audit.searched   (source: header line 12; result: GAPS 2)
- T1 SQL: GAP - the filter columns exist in phase-02.md:113-156 and the tier function at phase-24.md:255 matches its call in step 5 (phase-24.md:595). The `case_id` filter on the access_event stream has no column (see V-24-01-a). Settings `audit.search_page_size` and `audit.search_max_window_days` are seeded at phase-24.md:450 and 452.
- T2 Route: PASS - phase-24.md:547 `GET /api/v1/audit/events`, scope `audit.read`, Cedar `audit.read_events`; Cedar permits IA and SEC only (phase-24.md:855-863); API.md:100 gives `audit.read` to IA and SEC.
- T3 Repository and types: GAP - `search(ctx, query, cursor) -> AuditSearchPage` (phase-24.md:590) and `search_events(conn, AuditFilter, Keyset, viewer_tier_rule)` (phase-24.md:539) are named, but the query type is not (see V-24-01-b).
- T4 Logic: PASS - numbered steps 1 to 9 at phase-24.md:591-599: 403 `E_FORBIDDEN` (591), 422 `E_PURPOSE_REQUIRED` (592), 422 `window_too_long` (593), 404 masked (595), 503 `E_UNAVAILABLE` when the audit write fails (598). The audit write runs in its own transaction before the response (598).
- T5 Audit: PASS - `audit.searched {filters (names only), purpose, result_count, has_more}` at phase-24.md:598, listed at phase-24.md:1075.
- T6 Cross-references: PASS - `audit.searched` is reserved for this phase (phase-02.md:974); `AuditSink` trait (phase-02.md:576); purpose codes `internal_audit`, `supervisory_inspection`, `other` (phase-03.md:143-149); masked kind `audit_event` (phase-03.md:1277); problem codes in API.md section 6 (API.md:961-1002).
- T7 Consistency: PASS - header SP-42 (phase-24.md:12), step 9 (phase-24.md:599) and Open doubts 11 (phase-24.md:1487) agree that the export is not in this phase.
- GAP V-24-01-a | LOW | T1 | Step 4 lists `case_id` as a filter for both streams (phase-24.md:594). `audit.access_event` has no `case_id` column, so the filter on that stream is undefined; only `audit.audit_log` has it (phase-02.md:122). | 3.1 step 4 (phase-24.md:594) | grep case_id in phase-02.md lines 142-170 finds nothing.
- GAP V-24-01-b | MEDIUM | T3 | No struct is defined for the search input (`purpose`, `reason`, `from`, `to`, the filters, `limit`, `cursor`). `AuditFilter` is used at phase-24.md:539 and never defined; the DTO lists (phase-24.md:531 and 542) name only the response. | section 2 DTO lists (phase-24.md:531-542) | grep "AuditSearch\|AuditFilter\|AuditQuery" in phase-24.md finds only lines 539, 542 and 590.

### V-24-02 GV-34 findings register (create, update, post-review sync)   (source: header line 12; result: GAPS 3)
- T1 SQL: PASS - `admin.registers` (phase-24.md:42-74) has every column the logic writes: `severity` (line 49), `repeat_of` and `repeat_flag` (52-53), `status` (54-55), `closure_code`, `closed_by`, `closed_at`. `subject_key` is not a column; step 2 requires it in `body` (phase-24.md:645).
- T2 Route: PASS - POST `/api/v1/registers/{kind}/entries` (phase-24.md:556) scope `config.author`, `config.approve`, which admit M, CH and MRO (API.md:95-96); Cedar `register.write` for M, CH, MRO (phase-24.md:916-925). PATCH (558) uses `register.write` and `register.close` (M and the row owner, 928-934).
- T3 Repository and types: PASS - `create_entry(ctx, kind, body, idem)` (phase-24.md:643), `update(ctx, kind, id, patch, expected_version)` (651), `sync_post_review` (665), `RegisterRepository::insert` and `update` (533), `CreateRegisterEntry` and `UpdateRegisterEntry` (531).
- T4 Logic: GAP - the manual create (643-649), the state machine (652) and the version guard (651) are complete. Step 2 gives no codes and the sync gives no title or body (see a and b).
- T5 Audit: PASS - `register.created` (phase-24.md:649 and 665), `register.updated` and `register.closed` (655); all listed at phase-24.md:1080-1082.
- T6 Cross-references: PASS - `post_review_recorded {day, outcome}` (phase-04.md:944); outcomes `effective`, `concern`, `rollback_needed` (phase-04.md:942); `admin.instance_events` with `seq` and `payload` (phase-04.md:353-362); `maker_id` (phase-04.md:130); `PolicyPackService::scan(scope, text)` (phase-09.md:797); step-up class `approval` (phase-01.md:710).
- T7 Consistency: PASS - the finding and warning state machines (phase-24.md:652) match the DDL CHECKs (phase-24.md:65-66); DR-222 (phase-24.md:1453) agrees on the owner never accepting its own risk.
- GAP V-24-02-a | MEDIUM | T4 | Step 2 refusals (title without `ar` and `en`, severity outside the four, owner not an active governance user, `subject_key` missing) name no HTTP status and no problem code. The 3.13 Validation sub-codes (phase-24.md:1057) list none of them. | 3.4 create_entry step 2 (phase-24.md:645) | grep "E_" in phase-24.md lines 643-649 finds nothing; grep "owner\|subject_key" in lines 1050-1070 finds nothing.
- GAP V-24-02-b | MEDIUM | T4 | `sync_post_review` names no title, although the DDL requires `title ? 'ar' AND title ? 'en'` (phase-24.md:47); names no body keys; and gives no join from the event to its change request (`admin.workflow_instances.resource_id`, phase-04.md:331-332). | 3.4 sync_post_review (phase-24.md:665) | grep "title\|body" on line 665 finds nothing; grep "workflow_instances" in phase-24.md finds nothing.
- GAP V-24-02-c | LOW | T4 | `accepted_risk` names the class `approval` but no `StepUpAction { name, class }` constant; phase-01.md:711 says each phase declares its actions as such constants. | 3.4 update (phase-24.md:653) | grep "StepUpAction" in phase-24.md finds nothing.

### V-24-03 S-46 review calendar (rules, exclusions, change requests, readiness, scenarios)   (source: header line 13; result: GAPS 3)
- T1 SQL: PASS - `admin.inventory_items.next_validation_due` (phase-04.md:528); `admin.exclusions.review_due` and `ended_at` (phase-12.md:262 and 271); `admin.change_requests.review_due_at` and `review_outcome` (phase-04.md:154-155); `scn.readiness_grades.review_due` (phase-06.md:205); `scn.scenario_versions.effective_from` (phase-16.md:67-84), the activation column named by 7.9 (phase-24.md:1310-1312); `review.rules_months` (phase-24.md:460).
- T2 Route: PASS - GET `/api/v1/registers/review-calendar` (phase-24.md:559), scope `dashboard.read` (M, CH, IA, API.md:99), matching Cedar `register.read` for M, CH, IA (phase-24.md:898-914).
- T3 Repository and types: GAP - `review_calendar(ctx, horizon_days)` and `ReviewCalendarRow` are named (phase-24.md:657, 542); the sweep job's method is not (see a).
- T4 Logic: GAP - the state rules at 658-662 are explicit; the sweep and the unused `horizon_days` are not (see a and c).
- T5 Audit: PASS - warnings from the sweep carry `register.created` (phase-24.md:1093); the calendar read writes nothing.
- T6 Cross-references: PASS - the four tables and columns named in T1 exist as cited (phase-04.md:528, phase-12.md:262, phase-06.md:205, phase-16.md:84).
- T7 Consistency: GAP - the red state is defined two ways (see b).
- GAP V-24-03-a | MEDIUM | T3 | The job `review_calendar_sweep` has no named method. Section 4.3 gives `review_sweep: Arc<GovernanceRegisterService>` (phase-24.md:1156) and 3.4 describes the sweep in one sentence (phase-24.md:663). The warning title key for review rows is not named (see V-24-09-a). | 3.4 sweep (phase-24.md:663) and 4.3 (phase-24.md:1156) | grep "sweep" in phase-24.md finds 663, 1143 and 1156 only; grep "sweep(" finds nothing.
- GAP V-24-03-b | MEDIUM | T7 | The same calendar defines red two ways: "red past due" (phase-24.md:657) and "red on the due date" for the 7.3 rows (phase-24.md:1277). The sweep opens a warning on red, so the boundary day decides whether a warning is written. | 3.4 state rule (phase-24.md:657) and 7.3 (phase-24.md:1277) | grep "red" in phase-24.md finds these two rules and the scenario rule at 662 only.
- GAP V-24-03-c | LOW | T4 | `review_calendar(ctx, horizon_days)` takes `horizon_days` (phase-24.md:657), but no step and no route row (phase-24.md:559) uses it. The other horizon hits are `command.clock_horizon_days` (phase-24.md:468, 736). | 3.4 review_calendar (phase-24.md:657) | grep "horizon" in phase-24.md finds no use of the parameter.
### V-24-04 B-11 board pack builder (counts through each source predicate)   (source: header line 16; result: GAPS 2)
- T1 SQL: PASS - `admin.packs` kind CHECK includes `board` (phase-24.md:242-243); `rpt.periodic_reports` has kind `board` and status `final` (phase-21.md:344 and 349); `report.small_number_threshold` is seeded by phase 21 (phase-21.md:518).
- T2 Route: PASS - POST `/api/v1/packs` (phase-24.md:567), scope `dashboard.read` (M, CH, IA, API.md:99); Cedar `pack.board.create.m` (phase-24.md:965-967); read and download for M, CH, IA (phase-24.md:969-978).
- T3 Repository and types: GAP - the port `BoardSource::periodic_final(period)` is named (phase-24.md:541), but no builder method and no source for the aggregates of 3.4 to 3.7 is named (see a).
- T4 Logic: GAP - the board rule (phase-24.md:805) gives the id refusal only. The aggregate set and the predicate are not stated (see b).
- T5 Audit: PASS - `pack.created`, `pack.requested`, `pack.build_failed` (phase-24.md:1091); `pack.build_failed {reason: case_reference}` (phase-24.md:805).
- T6 Cross-references: PASS - `PackService::publish(tx, NewPack, actor) -> Pack` (phase-02.md:915); POST `/packs` (phase-06.md:488); `pack_build` job (phase-06.md:349); `rpt.periodic_reports` (phase-21.md:343).
- T7 Consistency: PASS - the readers in the board row (phase-24.md:799) match Cedar (phase-24.md:965-978) and 3.15 (phase-24.md:1112). The conflict with the 3.7 visibility rules is recorded under T4 (V-24-04-b).
- GAP V-24-04-a | MEDIUM | T3 | No method is named for the board builder or for the aggregates it takes from 3.4 to 3.7. Section 4.3 says only that the five kinds are registered through the phase 06 handler (phase-24.md:1156). | 3.11 board row (phase-24.md:799) and section 2 (phase-24.md:541) | grep "build" in phase-24.md lines 793-816 finds no method name; grep "Builder" finds only the statement at phase-24.md:522.
- GAP V-24-04-b | HIGH | T4 | The board pack is read by M, CH and IA (phase-24.md:799, 1112), and its aggregates are "aggregates of 3.4 to 3.7" (phase-24.md:799). Several 3.7 tiles are visible to M only (productive alerts at 723, UTRs and STRs at 724, the tipping-off summary at 730), and suspicion subjects are M's (718). The builder's predicate and tile list are not stated, so a pack built by M would carry M-only counts to CH and IA, against the "nothing" answer of 3.15. | 3.11 board row and board pack rule (phase-24.md:799 and 805) | grep "predicate" in phase-24.md lines 793-816 finds nothing; grep "tile" in the same lines finds nothing.

### V-24-05 VC-82 case and correspondent packs with hashes   (source: header line 17; result: GAPS 2)
- T1 SQL: PASS - `admin.packs` kind CHECK includes `case` (phase-24.md:242-243); the correspondent kind stays in the CHECK but is not built here (phase-24.md:1167-1168); the case file reads `casework.cases` (phase-17.md:287-300).
- T2 Route: PASS - POST `/api/v1/packs` (phase-24.md:567), GET `/api/v1/packs/{id}` (568) and the download (569) with scopes that include `dashboard.read` (M, API.md:99); Cedar `pack.case.m` (phase-24.md:981-983).
- T3 Repository and types: PASS - the case source is the named port `CaseFileSource` (phase-24.md:541); its method is not named, but the read it wraps is named (phase-24.md:800). The response is `PackView` (phase-24.md:542).
- T4 Logic: GAP - no refusal is stated for an unknown or unreadable case (see a).
- T5 Audit: PASS - `pack.created` (phase-24.md:1091; phase-02.md:980).
- T6 Cross-references: PASS - the correspondent part is deferred to phase 31 and carried there: `pack.correspondent` flag (phase-31.md:310), correspondent pack steps (phase-31.md:625-635); `PackService::publish` (phase-02.md:915); `RenderService::render` (phase-02.md:897). The read model is cited as phase 18 (see b).
- T7 Consistency: PASS - header VC-82 (phase-24.md:17) and 4.5 item 2 (phase-24.md:1168) agree that correspondent packs stay P2, and phase 31 builds them.
- GAP V-24-05-a | MEDIUM | T4 | The case build gives the scope `{case_id}` (phase-24.md:800) but no refusal for an unknown case id, a case the caller cannot read, or the job failure reason for either. Only the board has a named failure reason (phase-24.md:805). | 3.11 case row (phase-24.md:800) | grep "E_NOT_FOUND" in phase-24.md lines 795-816 finds nothing; grep "pack.build_failed" finds only lines 805 and 1091.
- GAP V-24-05-b | LOW | T6 | The read model is cited as "of phase 18" (phase-24.md:541, 800). The route `GET /api/v1/cases/{id}` is phase 17's (phase-17.md:849), and phase 18 only extends its fields (phase-18.md:618). No function is named in either phase. | 3.11 case row (phase-24.md:800) | grep "CaseFileSource\|CaseReadModel\|case read model" in phase-17.md and phase-18.md finds nothing.

### V-24-06 M-62 assessor grant and its time box   (source: header line 12; result: GAPS 1)
- T1 SQL: PASS - the cap is the phase 04 key `access.assessor_max_days`, seeded with default 30 and floor max 30 by 7.10 (phase-04.md:1725-1728); `admin.role_grants.valid_to` with `valid_to > valid_from` (phase-01.md:125 and 131).
- T2 Route: N/A - the grant is created through phase 04's `cr_access` change route (phase-04.md:568); phase 24 adds no grant route.
- T3 Repository and types: N/A - the grant has no method in phase 24.
- T4 Logic: PASS - the grant rule and time box are at phase-24.md:760; the refusal after `valid_to` is 403 `E_SCOPE` (phase-24.md:1053; API.md:969).
- T5 Audit: N/A - grant events belong to phases 01 and 04; the sample draw is audited at V-24-07.
- T6 Cross-references: PASS - `cr_access` (phase-04.md:568); DR-361 (research.md:372) fixes the one cap; `casework.case_acl` has no assessor reason (phase-18.md:90); DR-226 (phase-24.md:1461).
- T7 Consistency: GAP - the 7.1 block (phase-24.md:1259-1262) wins for the logic, but the base DDL still seeds the superseded key (see a).
- GAP V-24-06-a | LOW | T7 | 7.1 says `assessor.grant_max_days` "is not created" and that section 1 and 3.9 read the phase 04 key. The base seed row still creates it (phase-24.md:464), and 3.16 (phase-24.md:1122) and the lead correction (phase-24.md:1473) still name it, so the migration would create an unused governed setting. | section 1.4 seed rows (phase-24.md:464), 3.16 (phase-24.md:1122) | grep "assessor.grant_max_days" finds lines 464, 760, 1122, 1259, 1473 only; no removal appears in section 1.4.

### V-24-07 S-56 read-only assessor role, exports and a record of what was shown   (source: header line 13; result: GAPS 2)
- T1 SQL: PASS - `admin.assessor_samples` (phase-24.md:190-209) holds the seed, frame hash and items; `grant_id` references `admin.role_grants` (phase-24.md:192); the immutability trigger (phase-24.md:326) and the grant (phase-24.md:435) exist.
- T2 Route: PASS - routes 25 to 28 (phase-24.md:571-574) use scope `assessor.read`, which API.md:101 gives to AS and IA; Cedar `assessor.d2` for AS and IA (phase-24.md:1017-1022), `assessor.sample` for AS (1025-1026), `assessor.trace` for AS and IA (1029-1034). Exports are the assessor pack download (route 23, phase-24.md:569).
- T3 Repository and types: GAP - `d2_matrix`, `draw` and `trace` are named (phase-24.md:762, 764, 775); the method for route 27 is not (see b).
- T4 Logic: GAP - `draw` (764-771) and `trace` (775-782) have numbered steps and refusals; the logging of a sample view is missing (see a).
- T5 Audit: PASS - `assessor.matrix_viewed`, `assessor.sample_drawn`, `assessor.trace_viewed` (phase-24.md:1086-1088).
- T6 Cross-references: PASS - `admin.authority_matrix` (phase-09.md:300); `admin.coverage_certificates` (phase-12.md:312); `alert.alerts` with PK (id, created_at) (phase-17.md:106-163); `admin.evidence_objects` (phase-02.md:356).
- T7 Consistency: PASS - the D2 map as data (phase-24.md:762) agrees with DR-229 (phase-24.md:1467) and Open doubts 6 (phase-24.md:1482).
- GAP V-24-07-a | MEDIUM | T4 | `GET /api/v1/assessor/samples/{id}` (route 27, phase-24.md:573) returns the seed, the frame hash and the items. No access event or audit kind is named for the read, although S-56 asks for "a record of what was shown". 3.9 says only "every item later viewed is logged" (phase-24.md:766). | 3.9 after draw (phase-24.md:766) and Reproduction (phase-24.md:773); 3.14 (phase-24.md:1086-1088) | grep "sample_viewed\|sample_read" finds nothing; grep "assessor.sample" finds 573, 835, 1025, 1087 only; section 6 (phase-24.md:1223-1254) tests no sample read.
- GAP V-24-07-b | MEDIUM | T3 | Route 27 has no service method. `SampleRepository::get(conn, id)` is named (phase-24.md:537) but no `AssessorService` method calls it. | section 2 (phase-24.md:537) and 3.9 (phase-24.md:758-784) | grep "samples/{id}" finds 573 and 773 only; grep "sample(ctx\|samples(ctx" finds nothing.
### V-24-08 L-57 registers, decisions and exclusions as read views   (source: header line 17; result: GAPS 3)
- T1 SQL: PASS - `casework.decisions` has `type`, `reason_codes`, `text_enc`, `decider_id`, `proposer_id`, `decided_at` (phase-18.md:137-147); `case_ref` is on `casework.cases` (phase-17.md:292); `casework.cases.tier` (phase-17.md:300); `admin.exclusions` has `reason`, `approved_by`, `review_due`, `expires_at` and `ended_at` (phase-12.md:259-271).
- T2 Route: PASS - GET `/api/v1/registers/{kind}` (phase-24.md:555), scope `dashboard.read` (M, CH, IA, API.md:99), matching Cedar `register.read` for M, CH and IA (phase-24.md:898-914).
- T3 Repository and types: GAP - no service method is named for route 9, and no repository method reads `casework.decisions` or `admin.exclusions` (see a).
- T4 Logic: GAP - the predicate, columns and keyset for the views are complete (phase-24.md:636-641); the refusal for an unknown kind is not (see b).
- T5 Audit: N/A - a read with no mutation.
- T6 Cross-references: PASS - `services::visibility::build(principal, network)` (phase-17.md:1232); `casework.qa_samples` (phase-19.md:365); `admin.exclusions` (phase-12.md:253); `casework.decisions` (phase-18.md:137).
- T7 Consistency: GAP - MRO is listed as a reader of the registers and fairness, but the Cedar permits do not admit MRO (see c).
- GAP V-24-08-a | MEDIUM | T3 | `GovernanceRegisterService` names `create_entry`, `update`, `review_calendar` and `sync_post_review` (phase-24.md:643-665) but no list or read method. Section 2 lists no repository for `casework.decisions` or `admin.exclusions` (phase-24.md:531-541). | 3.4 Kinds (phase-24.md:636-641) and section 2 (phase-24.md:532-541) | grep "list(ctx" in phase-24.md finds only line 790 (training); grep "exclusions" in lines 531-541 finds nothing.
- GAP V-24-08-b | LOW | T4 | An unknown `kind` on the GET is not refused. Route 9 lists the kinds (phase-24.md:555) and 3.4 gives none for another value. | 3.4 Kinds (phase-24.md:636-641) | grep "unknown\|not a kind" in phase-24.md lines 634-665 finds nothing.
- GAP V-24-08-c | MEDIUM | T7 | 3.5 says `fairness.read` is "M, CH, IA and MRO" (phase-24.md:686) and 3.15 says the registers and fairness are read by "M, CH, IA, MRO" (phase-24.md:1107-1108). The Cedar policies `gov.read.m`, `gov.read.ch` and `gov.read.ia` admit M, CH and IA only (phase-24.md:898-914). | 3.12 Cedar (phase-24.md:898-914) or 3.5 and 3.15 (phase-24.md:686, 1107-1108) | grep "MRO" in phase-24.md lines 895-915 finds nothing.

### V-24-09 7.7 register warning writer RegisterWriter::warn   (source: 7.7 block; result: GAPS 2)
- T1 SQL: PASS - the unique partial index `uq_registers_source_key` (phase-24.md:75) has the predicate `WHERE source_key IS NOT NULL` used by the ON CONFLICT target; `title` requires `ar` and `en` (phase-24.md:47); the `source` CHECK admits the sources used (phase-24.md:45).
- T2 Route: N/A - the port has no route; it is called from jobs only.
- T3 Repository and types: GAP - `warn(tx, source, source_key, title_key: WarningTitle, body) -> Option<Uuid>` is named (phase-24.md:1297-1299), but the variants of `WarningTitle` for this phase's warnings are not (see a).
- T4 Logic: PASS - one insert with `ON CONFLICT (source_key) WHERE source_key IS NOT NULL DO NOTHING`, returning `None` when the key exists (phase-24.md:1298-1299).
- T5 Audit: PASS - a created warning is a "finding or warning created" under `register.created` (phase-24.md:1080); the job list at phase-24.md:1093 names no register event for fairness_monthly, but the generic row at 1080 covers any warning created.
- T6 Cross-references: GAP - the reference to section 1.1 for the jobs is wrong (see b). Phases 26 and 29 use the port as stated: phase-26.md:276 and 880-882; phase-29.md:1480-1481. DR-388 (research.md:399) agrees.
- T7 Consistency: PASS - the keys and `source` values of 7.7 match the review sweep (phase-24.md:663) and the fairness warnings (phase-24.md:678).
- GAP V-24-09-a | MEDIUM | T3 | The closed enum `WarningTitle` names no variant for this phase's warnings (the review rows at phase-24.md:663 and the fairness rows at phase-24.md:678), and no bilingual title. Phases 26 and 29 add their own variants to "phase 24's closed enum" (phase-26.md:882; phase-29.md:1480). The title must hold `ar` and `en` (phase-24.md:47). | 7.7 (phase-24.md:1300) | grep "WarningTitle" in phase-24.md finds only lines 1298 and 1300; research.md mentions it only in DR-388 (line 399).
- GAP V-24-09-b | LOW | T6 | 7.7 says the port "is called only from jobs (section 1.1)" (phase-24.md:1301). Section 1.1 is the 0076 DDL (phase-24.md:33-78); the job kinds are in section 4.1 (phase-24.md:1139-1145). | 7.7 (phase-24.md:1301) | grep "4.1" in phase-24.md lines 1296-1302 finds nothing.

### V-24-10 7.11 manual case grants and expired grants in the control report   (source: 7.11 block; result: GAPS 2)
- T1 SQL: PASS - `casework.case_acl` has `reason` with `manual` (phase-18.md:90), `granted_at` (93), `expires_at` (94), `revoked_by` and `revoked_at` (95-96); a NULL `revoked_by` with `revoked_at` is the system expiry (phase-18.md:99); `casework.cases.tier` (phase-17.md:300).
- T2 Route: PASS - GET `/api/v1/access/control-report` (phase-24.md:554), scope `dashboard.read` and `security.ops` (API.md:99, 105); Cedar `control.report.m` and `control.report.sec` (phase-24.md:882-887).
- T3 Repository and types: PASS - `AccessHistoryService::control_report(ctx, period)` (phase-24.md:615) and `ControlReportView` (phase-24.md:542).
- T4 Logic: GAP - the two counts read `casework.case_acl` and `casework.cases`, which the read-only pool cannot read (see a).
- T5 Audit: PASS - `access.control_report_generated {period}` (phase-24.md:621; listed at 1078).
- T6 Cross-references: PASS - `casework.case_acl` (phase-18.md:85-101); DR-395 (research.md:406) describes the same expiry model; `report.small_number_threshold` (phase-21.md:518); `admin.role_grants` (phase-01.md:125).
- T7 Consistency: PASS - 7.11 (phase-24.md:1322-1326) and the 3.2 bullet (phase-24.md:619) name the same two counts; 7.5 (phase-24.md:1288-1290) adds different counts.
- GAP V-24-10-a | HIGH | T4 | The report is "computed on `readonly_pool`" (phase-24.md:615), and every query of 3.1 to 3.3 runs there (phase-24.md:584). The manual-grant and expiry counts read `casework.case_acl` and `casework.cases`, and the phase states that `akr_readonly` has no table or column grant on casework (phase-24.md:303; phase-17.md:568; phase-18.md:443). No SECURITY DEFINER function or pool change is named for these counts (the only one is the tier resolver, phase-24.md:256). | 3.2 control_report (phase-24.md:615) and 7.11 (phase-24.md:1322-1326) | grep "SECURITY DEFINER" in phase-24.md finds lines 8, 217, 248, 256, 1175 and 1473 only, none for case counts.
- GAP V-24-10-b | LOW | T6 | 7.11 cites "the access-request register of phase 18 section 7+" (phase-24.md:1326). The phrase is not in phase-18.md; the concept is 7.10 "Time-boxed manual grants" (phase-18.md:1323) and the phrase appears only in DR-395 (research.md:406). | 7.11 (phase-24.md:1326) | grep "access-request" in phase-18.md finds nothing.

### V-24-11 7.12 register exports (POST /exports, GET status, GET file, job register_export)   (source: 7.12 block; result: GAPS 7)
- T1 SQL: PASS - the block adds no table (phase-24.md:1344: "no table is added"); the file goes through phase 02 `EvidenceStore::stage` with evidence kind `render`, which is valid (phase-02.md:355).
- T2 Route: GAP - the POST row names the Cedar action only by reference, "the list route's Cedar action" (phase-24.md:1335). The two GET rows name no Cedar action (phase-24.md:1336-1337) (see a).
- T3 Repository and types: GAP - no handler, service or repository method is named for the three routes or the job (see b).
- T4 Logic: GAP - refusal codes, the file response before the scan is clean, the job spec, and the step-up constant are missing (see c, d, e).
- T5 Audit: PASS - `export.requested {register, format}` and `export.downloaded {register, sha256}` are named in step 5 (phase-24.md:1349).
- T6 Cross-references: GAP - the owner kind and the evidence fields are not supplied (see f and g). `EvidenceStore::stage(tx, NewEvidence, actor)` is named (phase-02.md:839); the REGISTER pattern exists (DESIGN-SYSTEM.md:674); DR-396 covers the per-user watermark (research.md:407); DR-400 covers exports (research.md:411).
- T7 Consistency: PASS - "PDFs stay on the pack path" (phase-24.md:1350) agrees with 3.11; the report register stays out of exports (phase-24.md:1350-1351) as in 3.4.
- GAP V-24-11-a | MEDIUM | T2 | The GET rows for the export status and the file (phase-24.md:1336-1337) name no Cedar action. Step 1 (phase-24.md:1340-1341) has the route check the list route's scope and Cedar action, and does not name that check for either GET row. | 7.12 route table (phase-24.md:1333-1337) | grep "Cedar" in phase-24.md lines 1330-1352 finds 1335 and 1341 only.
- GAP V-24-11-b | MEDIUM | T3 | No handler or service method is named for the POST, the two GETs or the job. The list service the job calls, "the screen's own list service" (phase-24.md:1342-1343), has no named method (see V-24-08-a). The request `{register, filters, format}` (phase-24.md:1335) is not a named type. | 7.12 steps 1 to 2 (phase-24.md:1335-1343) | grep "ExportService\|export(" in phase-24.md finds nothing; grep "list service" finds line 1343 only.
- GAP V-24-11-c | MEDIUM | T4 | Refusal codes are missing: "anything else is 422 at `/register`" (phase-24.md:1340-1341) names no problem code, and the file route (phase-24.md:1348) gives no response while the scan is pending or the file is not clean. | 7.12 steps 1 and 4 (phase-24.md:1340-1341, 1348) | grep "E_" in phase-24.md lines 1330-1352 finds nothing.
- GAP V-24-11-d | MEDIUM | T4 | Job spec incomplete: `register_export` is not in the job table (phase-24.md:1139-1145) and has no coalesce key, concurrency, attempts or enqueue transaction (b3-conventions.md:61 lists them as required). | 4.1 (phase-24.md:1139-1145) and 7.12 step 2 (phase-24.md:1342-1344) | grep "register_export" in phase-24.md finds lines 1342 and 1347 only.
- GAP V-24-11-e | LOW | T4 | The GET file route needs step-up class `export` (phase-24.md:1337), but no `StepUpAction` constant is declared; phase-01.md:711 says each phase declares its actions as constants. | 7.12 (phase-24.md:1337) | grep "StepUpAction" in phase-24.md finds nothing.
- GAP V-24-11-f | MEDIUM | T6 | The owner kind `register_export` is not in the evidence owner registry. phase-02.md:847 requires each owner kind to be listed by its phase; 4.5 item 1 (phase-24.md:1167) and Open doubts 29 (phase-24.md:1505) list only `validation_pack` and `training_record`, with their EvidenceResolver. | 4.5 item 1 (phase-24.md:1167) and Open doubts 29 (phase-24.md:1505) | grep "register_export" in phase-24.md finds 1342 and 1347 only; grep "EvidenceResolver" finds 1167 and 1505 only, both for validation_pack and training_record; grep "register_export" in phase-02.md and research.md finds nothing.
- GAP V-24-11-g | MEDIUM | T6 | `NewEvidence` requires `retention_class` and `classification`, and admin.evidence_objects has both NOT NULL (phase-02.md:358 and 366; field list at phase-02.md:611). A retention class also needs an `evidence.lock_days` entry (phase-02.md:848). Step 4 gives only the kind, the owner kind and the owner id (phase-24.md:1347-1348). | 7.12 step 4 (phase-24.md:1347-1348) | grep "retention\|classification" in phase-24.md lines 1330-1352 finds nothing.

### V-24-12 7.13 parallel run, explanation of a disagreement (POST /parallel-run/rows/{id}/explanation)   (source: 7.13 block; result: GAPS 5)
- T1 SQL: PASS - table `alert.parallel_run_rows` (phase-24.md:1368-1391) defines every column the explanation sets (`explanation_code`, `note`, `explained_by`, `explained_at`), and the CHECKs (phase-24.md:1386-1390) match step 3; the FK target `alert.alerts(id, created_at)` has the PK (phase-17.md:163).
- T2 Route: GAP - the route row names scope `case.decide` and Cedar `parallel_run.write` (phase-24.md:1402), and the scope admits M (API.md:86). The Cedar policy text and the resource type are missing (see e).
- T3 Repository and types: GAP - no method is named for the explanation (see a).
- T4 Logic: GAP - step 3 (phase-24.md:1421-1423) has no lock or guarded write, and the DDL's PolicyPackService::scan is not called (see b, c, d).
- T5 Audit: PASS - `parallel_run.explained {row_id, code}` is named in step 3 (phase-24.md:1423).
- T6 Cross-references: PASS - `case.decide` is A2 and M (API.md:86); `PolicyPackService::scan(scope, text)` (phase-09.md:797) exists; `admin.users` (phase-01.md); `E_NOT_ALLOWED_IN_STATE` and `E_VALIDATION` (API.md:982 and 985).
- T7 Consistency: PASS - "Nothing is written to admin.registers" (phase-24.md:1433) agrees with 3.4; the M and CH read scope and the M-only write (phase-24.md:1405-1406) agree with the tab rule (phase-24.md:1429).
- GAP V-24-12-a | MEDIUM | T3 | No method is named for the explanation. `ParallelRunService` is named with steps only (phase-24.md:1408-1427), and step 3 has no signature, no repository write and no named request type for `{code, note}`. | 7.13 ParallelRunService (phase-24.md:1408-1427) | grep "explain" in phase-24.md finds no method; grep "ParallelRunService" in docs/plan finds phase-24.md only.
- GAP V-24-12-b | MEDIUM | T4 | Two explainers can race. Step 3 checks "not yet explained" (phase-24.md:1421) and then sets the fields. No `FOR UPDATE`, `ON CONFLICT` or guarded `WHERE explanation_code IS NULL` appears in 7.13, and 3.17 (phase-24.md:1125-1133) does not cover this table, so the second write silently replaces the first. | 7.13 step 3 (phase-24.md:1421-1423); 3.17 (phase-24.md:1125-1133) | grep "FOR UPDATE\|lock\|guard\|ON CONFLICT" in phase-24.md lines 1356-1439 finds nothing relevant.
- GAP V-24-12-c | MEDIUM | T4 | The DDL comment says the note "passes PolicyPackService::scan" (phase-24.md:1380), but step 3 does not call the scan. | 7.13 step 3 (phase-24.md:1421-1423) | grep "scan" in phase-24.md lines 1421-1423 finds nothing.
- GAP V-24-12-d | LOW | T4 | Refusals for a code outside the seven values and for a note over 2000 characters have no problem code; the CHECKs at phase-24.md:1372 and 1380 enforce them only in the database. | 7.13 step 3 (phase-24.md:1421-1423) | grep "E_" in phase-24.md lines 1397-1431 finds only lines 1402 and 1422.
- GAP V-24-12-e | LOW | T2 | The Cedar policy for `parallel_run.write` is prose only (phase-24.md:1405-1406). Section 3.12 has no policy text for it, and no parallel-run entity is declared in the schema additions (phase-24.md:817-1047). | 7.13 Cedar (phase-24.md:1405-1406) and 3.12 (phase-24.md:817-1047) | grep "parallel" in phase-24.md lines 817-1047 finds nothing.
## Summary

- Items checked: 12 (header items 01 to 08; gap-pass 7.x items 09 to 12). Phase 24 has no "Adversarial pass" block (`grep -ci dversar` returns 0), so the last three header items replace that slot.
- Items fully PASS: 0. Every sampled item has at least one GAP line.
- GAP lines by severity: HIGH 2, MEDIUM 21, LOW 11 (34 in total).
- Greps used for the "found nothing" claims are quoted on each GAP line. Files not read: cbs-discovery/, test-poc/, research/01-33 (one requirement lookup only, RP-61).

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-24-01-a | LOW | T1 | `case_id` filter on the access_event stream has no column | 3.1 step 4 (phase-24.md:594); phase-02.md:142-156 |
| V-24-01-b | MEDIUM | T3 | Search request type not defined (`AuditFilter` used, never defined) | section 2 (phase-24.md:531-542) |
| V-24-02-a | MEDIUM | T4 | Step 2 refusals give no HTTP status or problem code | 3.4 create_entry step 2 (phase-24.md:645) |
| V-24-02-b | MEDIUM | T4 | Post-review sync names no title, body keys or event-to-change-request join | 3.4 sync_post_review (phase-24.md:665) |
| V-24-02-c | LOW | T4 | No StepUpAction constant for `accepted_risk` | 3.4 update (phase-24.md:653) |
| V-24-03-a | MEDIUM | T3 | Review sweep method not named | 3.4 (phase-24.md:663); 4.3 (phase-24.md:1156) |
| V-24-03-b | MEDIUM | T7 | Red defined as "past due" (3.4) and "on the due date" (7.3) | phase-24.md:657, 1277 |
| V-24-03-c | LOW | T4 | `horizon_days` parameter unused | phase-24.md:657 |
| V-24-04-a | MEDIUM | T3 | Board builder and aggregate source not named | 3.11 (phase-24.md:799); section 2 (phase-24.md:541) |
| V-24-04-b | HIGH | T4 | Board predicate and tile list unstated; M-only counts would reach CH and IA readers | 3.11 (phase-24.md:799, 805); 3.7 (phase-24.md:718-730) |
| V-24-05-a | MEDIUM | T4 | No refusal for an unknown or unreadable case in the case pack | 3.11 case row (phase-24.md:800) |
| V-24-05-b | LOW | T6 | Read model cited as phase 18; the route is phase 17's | phase-24.md:541, 800; phase-17.md:849 |
| V-24-06-a | LOW | T7 | Base seed row, 3.16 and the lead correction still create or name `assessor.grant_max_days` | phase-24.md:464, 1122, 1473 |
| V-24-07-a | MEDIUM | T4 | Sample read (route 27) names no access event, so "a record of what was shown" is not met | phase-24.md:573, 766, 773; 3.14 (1086-1088) |
| V-24-07-b | MEDIUM | T3 | Route 27 has no service method | section 2 (phase-24.md:537); route row 573 |
| V-24-08-a | MEDIUM | T3 | No list method for route 9; no repository for decisions or exclusions reads | 3.4 (phase-24.md:636-665); section 2 (phase-24.md:531-541) |
| V-24-08-b | LOW | T4 | Unknown `kind` on the GET not refused | 3.4 Kinds (phase-24.md:636-641) |
| V-24-08-c | MEDIUM | T7 | MRO reads registers and fairness in 3.5 and 3.15 but not in Cedar | phase-24.md:686, 1107-1108; Cedar 898-914 |
| V-24-09-a | MEDIUM | T3 | This phase's own `WarningTitle` variants and bilingual titles not named | 7.7 (phase-24.md:1298-1300) |
| V-24-09-b | LOW | T6 | Jobs cited as section 1.1; they are in section 4.1 | 7.7 (phase-24.md:1301) |
| V-24-10-a | HIGH | T4 | Control-report counts read casework on `readonly_pool`, which has no casework grant | 3.2 (phase-24.md:584, 615); 7.11 (phase-24.md:1322-1326); phase-24.md:303 |
| V-24-10-b | LOW | T6 | "access-request register" phrase not in phase 18 | 7.11 (phase-24.md:1326) |
| V-24-11-a | MEDIUM | T2 | GET export status and file rows name no Cedar action | 7.12 route table (phase-24.md:1336-1337) |
| V-24-11-b | MEDIUM | T3 | No handler, service or repository method named; list service unnamed | 7.12 (phase-24.md:1335-1343) |
| V-24-11-c | MEDIUM | T4 | Refusal codes missing (422 at `/register`; file response before a clean scan) | 7.12 (phase-24.md:1340-1341, 1348) |
| V-24-11-d | MEDIUM | T4 | Job `register_export` absent from 4.1; no coalesce key, concurrency, attempts or enqueue transaction | 4.1 (phase-24.md:1139-1145); 7.12 (phase-24.md:1342-1344) |
| V-24-11-e | LOW | T4 | No StepUpAction constant for the export step-up | 7.12 (phase-24.md:1337) |
| V-24-11-f | MEDIUM | T6 | Owner kind `register_export` not in the evidence owner registry, nor its EvidenceResolver | 4.5 item 1 (phase-24.md:1167); Open doubts 29 (phase-24.md:1505); phase-02.md:847 |
| V-24-11-g | MEDIUM | T6 | Retention class and classification of the export file not given | 7.12 step 4 (phase-24.md:1347-1348); phase-02.md:358, 366, 611 |
| V-24-12-a | MEDIUM | T3 | No explain method, repository write or request type | 7.13 ParallelRunService (phase-24.md:1408-1427) |
| V-24-12-b | MEDIUM | T4 | No lock or guarded write for two explainers; the second write replaces the first | 7.13 step 3 (phase-24.md:1421-1423); 3.17 (phase-24.md:1125-1133) |
| V-24-12-c | MEDIUM | T4 | Note scan promised by the DDL comment is not in step 3 | 7.13 DDL (phase-24.md:1380); step 3 (phase-24.md:1421-1423) |
| V-24-12-d | LOW | T4 | No problem code for an unknown explanation code or a note over 2000 characters | 7.13 step 3 (phase-24.md:1421-1423) |
| V-24-12-e | LOW | T2 | Cedar policy for `parallel_run.write` is prose only; no resource entity declared | 7.13 Cedar (phase-24.md:1405-1406); 3.12 (phase-24.md:817-1047) |
