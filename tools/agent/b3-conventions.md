# Conventions for AKR-TMS phase drafts (batch B3). Every item below was a real slip in earlier drafts.

## Method
- Write ONLY your own file docs/plan/phase-NN.md. Edit no other file. Never commit. No emojis. No sanctions-list
  personal names (use invented names like "Ahmed Kareem Hassan").
- Write section by section with `cat >> docs/plan/phase-NN.md <<'EOF' ... EOF` (the PC crashes; disk first).
- Read with line ranges (sed -n 'A,Bp'), never whole large files. `sed -n 0,Np` fails; use 1,Np.
- Sections in this exact order: header; 1 SQLx Migration Changes; 2 DDD Implementation (Rust); 3 Business Logic;
  4 Infrastructure Updates; 5 Frontend Impact; 6 Verification; 7+ Gap Additions (one line: "None yet.");
  "## Decisions" (numbered from the DR the prompt gives); "## Open doubts"; "## What This Phase Does NOT Touch" LAST.
- Number your own decisions only from the DR the prompt gives. Cite existing ones (DR-01..DR-413 in docs/plan/research.md) only
  where they really apply. Cite DBS decisions as "DBS decision NN", never "DBS DR-NN".
- Requirement IDs only from docs/MODULES.md (your module's lines) and research IDs already quoted there. Do not invent IDs.
- Final reply to the lead: under 15 lines: line count, migrations, tables, routes, jobs, Cedar file, open doubts count.

## SQL
- UUIDv7 ids generated app-side: `id UUID PRIMARY KEY` with NO default; never gen_random_uuid().
- Schema-qualified names everywhere; CREATE ... IF NOT EXISTS; TIMESTAMPTZ only; enums in
  `DO $$ BEGIN CREATE TYPE ... EXCEPTION WHEN duplicate_object THEN NULL; END $$;`.
- Migration file names `NNNN_snake_name.sql`, numbered from the number the lead gave you.
- GRANT for EVERY write path, including `GRANT UPDATE (col, col)` column lists for every column any service updates.
  Roles: akr_app (api + worker), akr_ingest, akr_readonly, akr_audit_writer, akr_migrate, akr_monitor.
- Every new table gets a classification row and a retention row. Copy the INSERT column lists EXACTLY from
  docs/plan/phase-18.md (grep -n "INSERT INTO admin.column_classification\|INSERT INTO admin.retention_assignment"). Tables:
  admin.column_classification (enum admin.data_classification; `encrypted` only for personal/financial/secret-str/card) and
  admin.retention_assignment (PK schema_name, table_name).
  Classes: public, internal, personal, financial, detection-logic, secret-str, card.
  Retention clock partition/age/relationship_end/closure/decision REQUIRES clock_column; retirement/owner/never
  REQUIRE clock_column NULL.
- Settings rows: copy the INSERT column list from phase-11 or phase-12 (grep -n "INSERT INTO admin.settings").
  value_type one of integer, decimal, boolean, string, duration_s, cron, timestamp, json. `floor` JSONB
  {"min": n, "basis": "..."} ONLY when governed = TRUE (phase 00 CHECK `floor IS NULL OR governed`); else floor NULL.
  Cron values look like '"0 20 3 * * *"' (JSON string, 6 fields with seconds).
- Before referencing ANY earlier table, column, enum value or setting, grep its real DDL in docs/plan/phase-00..31.md.
  Never assume a column exists. Never redefine a table another phase created; ALTER it and say so.

## Rust and API
- Route table format `| Method | Path | Scope | Description (cedar action) |`.
- Scopes come from docs/API.md section 2 (lines 76-110). READ SCOPES MUST ADMIT EVERY ROLE YOUR CEDAR POLICY ADMITS
  (a route may list several scopes, comma-separated, any one suffices). Key scopes: dashboard.read M,CH,IA;
  data.steward DS; config.author CFG,DS,MRO; config.approve CHK,M,CH,MRO; screening.work SS,M; kyc.work KYC,M;
  queue.work A1,A2,M; report.prepare A2,M; admin.system ADM; security.ops SEC; audit.read IA,SEC; model.risk MRO.
- Cedar in the phase 05 idiom (see docs/plan/phase-05.md, grep -n "permit (" ):
  `permit (principal in Akr::Role::"DS", action in [Akr::Action::"x.y", ...], resource);`
  Step-up is NOT a Cedar condition. The handler calls the phase 01 service
  `StepUpService::consume(tx, ctx, header, StepUpAction::<Class>, resource_ref)`. StepUpAction classes are CLOSED:
  token_create, credential_change, identity_admin, access_change, decision, approval, filing, export, backfill,
  break_glass, signing, record_open. Do not invent a class.
- Cedar files: phase 03 reserved ranges (00-50 platform, 55/56 change control and workflow, 60 audit, 70 sources and
  mapping, 80 detection, 90 casework, A0 reporting, B0 branch window, C0 governance). 80_detection.cedar belongs to
  phase 06: do not edit it, add your own file named in the lead's instruction.
- Change requests: register target kinds with the phase 04 ChangeTargetRegistry (grep -n "ChangeTargetRegistry\|trait ChangeTarget" docs/plan/phase-04.md).
  Routes cr_minor, cr_material (parallel MLRO + CHK; CH too when volume_effect is decrease), cr_mapping, cr_authority.
  volume_effect `unknown` can never be approved as material.
- Problem codes only from docs/API.md section 6 (grep -n "^| E_" docs/API.md). E_NOT_ALLOWED_IN_STATE is 409.
  E_VERSION_CONFLICT 409 with If-Match; E_PRECONDITION_REQUIRED 428; E_STEPUP_REQUIRED 401; E_SELF_APPROVAL 403.
  No E_SESSION_EXPIRED (use E_UNAUTHENTICATED).
- One audit event per mutation, named `<domain>.<verb>` with the resource; payloads carry names and ids, never personal values.
- Never reuse or redefine another phase's job kind, route, repository, service or Cedar action. If you need more from
  one, write "extends phase NN X with method Y" and specify only the addition.
- Jobs (Postgres jobs table, see phase 11/12 section 4): kind, binary, payload, coalesce key, concurrency, attempts,
  schedule or trigger. The `ingest_` prefix is only for akr-ingestd jobs. Detection/batch compute runs in akr-worker.
- Tipping-off review in section 3: for every screen, export, notice or task, state what a person outside the case's
  access list can learn (the answer must be "nothing").

## Frontend and verification
- Section 5: screens by catalogue ID from docs/DESIGN-SYSTEM.md 8.1 (line 657) and 8.3 specs; Orval hook names
  (`useGetXxx`, `usePostXxx`) for each route used.
- Section 6: numbered concrete checks; curl suite `tests/curl/phaseNN_<x>.sh` with dev users one per role
  `<role>@bank.test` (ds@, cfg@, chk@, m@, ch@, ia@, ss@, kyc@, a1@, a2@, a2b@, adm@, sec@, mro@); include a 403
  Cedar deny, a masked 404 for suspicion-tier resources (screening alerts and hits ARE suspicion tier), an
  idempotency replay (Idempotency-Key), an If-Match 428/409 case, and a self-approval refusal where CRs exist.

- (phase 16 review) Masking is a READ rule: store the entity id on every per-entity row and resolve the tier at request
  time from canon.party.sensitivity_tier; never write NULL ids or a frozen `masked` flag.
- (phase 16 review) Never insert admin.change_requests rows directly: phase 04 CHECKs need a submitted route, a workflow
  instance and checker_id <> maker_id. Go through the phase 04 create/submit flow, or do not use a change request.
- (phase 16 review) A release/seed must never replace a live version the bank approved or tuned: first install may go
  live, later versions install in shadow and go live by change request.
- (phase 16 review) Check every CREATE TABLE for duplicate column names (e.g. a business `version` and the optimistic
  lock `version`).
- (phase 16 review) A ChangeTarget that cites phase 04 evidence kinds must register their EvidenceResolver.
- (phase 17 review) Before adding a method or enum value to an earlier phase, grep for an existing one: phase 02 already has
  `EvidenceStore::put_generated` (worker only) for generated objects; `lock_mode` is the S3 object-lock mode
  (COMPLIANCE, GOVERNANCE), never a new value.
- (phase 17 review) Never reuse a struct name another phase already defines (phase 14 `AlertService`); grep `struct`/service
  names in docs/plan before naming yours.
- (phase 17 review) A route's scope decides which roles can reach it (API.md section 2): do not give a screen or queue to a
  role whose scopes do not include the route's scope (SS has screening.work, not queue.work).
- (phase 17 review) Never call a route or method of another phase that does not exist; define a port in your phase and
  say the other phase calls it (gap pass 1).
- (phase 17 review) Cedar files are copy-paste ready: no `/* placeholder */` action lists; one role per `@role`; the Cedar
  entity attributes must admit exactly what your SQL visibility predicate admits.
- (phase 17 review) Lock order: when two paths take an advisory lock and a row lock, both take them in the same order.
- (phase 17 review) Phase 04 `volume_effect` values are increase, neutral, decrease, unknown only.
- (phase 17 review) Clock arithmetic is `BusinessClock::add(from, n, BusinessUnit)` (phase 09 `BankCalendarClock`).
- (phase 17 review) Masked readers use `casework.effective_tier(party_id)` (phase 17), not `canon.party.sensitivity_tier` alone.
- (phase 18 review) Respect the workflow definitions of DSL E.4 exactly: who decides each definition (e.g. reports are signed
  by M from `case_acl`) is enforced in `Holders` eligibility, so the inbox, Cedar Task adapter and engine agree.
- (phase 18 review) A state that returns to `open` after a decision must still refuse a second decision (guard on outcome).
- (phase 18 review) Cedar forbids on a shared resource (phase 04 `Task`) bind every phase's instances; put a subject-only rule
  in that subject's service check (`WorkflowSubject::check_before_decision`) instead.
- (phase 18 review) `StepUpService::consume(tx, ctx, header, action: StepUpAction, resource_ref)` takes a declared action
  constant (`StepUpAction { name, class }`), never a bare class; a phase 04 task act is stepped up by the engine (step 5) once.
- (phase 18 review) Phase 02 evidence kinds are upload, render, snapshot, package, certificate, pack; `NewEvidence` fields:
  kind, owner_kind, owner_id, retention_class, content_type, file_name, classification, lang, bytes.
- (phase 18 review) `RatingSource::levels(customers, as_of)` is keyed by `canon.customer.id` (party -> customer via
  `canon.customer.party_id`). `canon.party_name` is current by `superseded_at IS NULL` (not bi-temporal). `canon.txn`
  key is `(id, booking_date)`: store the booking date with any txn reference. `admin.role_grants` is phase 01.
- (phase 18 review) Phase 09 already has `DecisionType` (authority-matrix keys like `case.decision`); phase 18 has
  `CaseOutcome`. Phase 18 tables: casework.case_acl (access read/work/decide), notes, proposals, decisions, case_seals,
  work_items, evidence_items, entity_links, opened_views; `casework.network_edges` is reserved for phase 19 (DR-145).
- (phase 19 review) A closed case is SEALED: phase 18 refuses every case write with E_CASE_SEALED (grant, note, work item,
  link). Anything that must happen after closure (QA read grant, watch, hibernation ledger) writes audit events and NO case
  event; design it as its own path, never through a phase 18 route that refuses closed cases.
- (phase 19 review) A decision (phase 18 RecordDecisionEffect) sets `outcome` and leaves the case OPEN; closing is a separate act.
- (phase 19 review) Alert intake (phase 17 3.1) steps: 4 snapshot, 5 open/find case, 6 insert alert, 7 link, 9 clocks,
  10 events, 12 outbox. A gate that hides an alert must run before step 5 and skip 5, 7, 9, 10, 12.
- (phase 19 review) Never leave a requirement unusable by refusing it "until the bank decides": pick a safe default
  (e.g. allow on normal tier only + MLRO signature) and list the choice in Open doubts.
- (phase 19 review) A setting key used in text must exist in a settings INSERT (grep it). Alert `measures` are `financial`.
  The authority matrix permit carries `second_review`; every decision path refuses it with `second_review_not_shipped`.
- Phase 19 tables: casework.rfis, rfi_questions, rfi_answers, authority_requests, internal_reports, watches, hibernations,
  hibernation_hits, alarms, network_snapshots, network_edges, qa_samples, qa_reviews, qa_findings; Cedar 92_followup.cedar;
  case_acl reason gains `qa_review`; phase 18 now emits outbox `case.decision`.
- (phase 20 review) Retention: R7 = screening and sanctions records (ARCHITECTURE s.17), R5 = alerts and dispositions, R6 = cases
  and STRs. A record that stays in force (a freeze, an open instruction) must use a closure clock (released_at, closed_at), never
  an age clock from its start; child rows follow their owner (clock `owner`, clock_column NULL).
- (phase 20 review) There is NO generic `POST /evidence`: uploads are per owner route (`POST /cases/{id}/evidence`,
  `POST /ops-tasks/{id}/evidence`) calling `EvidenceStore::stage` with a registered owner kind.
- (phase 20 review) `StartSpec { definition_key, resource: (kind, id), purpose, initiator, maker }` has NO params; anything the
  effect needs is frozen into the instance context by `WorkflowSubject::start_context` and read from `inst.context`.
- (phase 20 review) Never let a config value fail ANOTHER phase's decision transaction (e.g. a port called inside phase 14's
  true-match decision): fall back to the safe default and audit `<area>.config_fallback`.
- (phase 20 review) An undo/lift task needs its own verification rule (the hold is GONE), checked even when its owner is
  released or closed.
- (phase 20 review) Copy phase 18 behaviour exactly: `second_review_not_shipped` is 409 E_NOT_ALLOWED_IN_STATE; the role refusal
  sub-code is `role_not_permitted`. Phase 15 exits: resource kind `exit_decision`, routes `/exit-register`.
- Phase 20 tables: casework.ops_tasks, ops_verifications, freezes, refrain_instructions, ops_incidents (+ casework.ops_ref_seq);
  casework.alarms kinds gain post_order_debit, freeze_overdue, refrain_overdue; Cedar 93_ops.cedar; port `LetterRequester`
  (phase 21 implements; recording stub `RecordingLetterRequester`); FreezeService replaces phase 14 RecordingFreezeTrigger.
- (phase 21 review) Cedar file ranges come from phase-03.md line 540 (60 audit, 70 sources and mapping, 80 detection,
  90 casework, A0 reporting, B0 branch window, C0 governance), NOT from the lead prompt; if the prompt's file number falls in
  another module's range, say so and pick the next free name in your own range.
- (phase 21 review) Work that happens after a case is closed (Office grades, rejections, corrections, letters, clocks) must
  NOT run on resource kind `case`: phase 18 refuses every act on a sealed case. Use your own resource kind and WorkflowSubject.
- (phase 21 review) When replacing a recording stub, keep the ids the stub already returned and owners already store; insert
  the real rows under those ids. Do not add methods to the earlier phase to rewrite its columns.
- (phase 21 review) Reconciliation and coverage certificates are phase 12 (`CoverageRepository::certificates_for_date`);
  phase 09's registries hold clocks and switches only (a CBI-fixed amount is a code constant mirroring the DSL pinned limit).
- (phase 21 review) A retention clock column must be set for EVERY row of the table (e.g. `signed_at` is never set for a CTR
  file); prefer `created_at` with a note when the real clock is "the later of" two events.
- Phase 21 tables: rpt.ctr_batches, reports, report_versions, report_findings, report_packages, report_links, filings,
  fiu_feedback, report_tasks, ctr_items, letters, periodic_reports (+ rpt.report_ref_seq, rpt.letter_no_seq); casework.alarms
  kinds gain report_filing_overdue, report_rework_overdue (subjects report, report_task); Cedar A1_reports.cedar; LetterService
  replaces RecordingLetterRequester; ReportWorkflowSubject (resource kind report); close gate reads the acknowledgement
  (sub-code filing_not_acknowledged); /liaison-reports and R5 are phase 22's.
- (phase 23 review) A foreign key ON DELETE RESTRICT into a partitioned table blocks `DETACH`/`DROP` of any partition holding a
  referenced row (e.g. `casework.case_alerts` -> `alert.alerts`). Grep every FK into a table before planning its partition drop.
- (phase 23 review) A partition clock (creation month) must never undercut a class floor that runs from a later event
  (disposition, report, relationship end): check the latest such instant of every row before dropping.
- (phase 23 review) Never block a short-lived class (R1 raw, R12 ops) on an unrelated hold: keep only what the hold scope names.
- (phase 23 review) Pool timeouts (statement_timeout 30 s) fail long statements: give a long operation its own timeout setting
  and lock the smallest object (the partition, SHARE) for counts; the parent only for the DDL, under lock_timeout.
- (phase 23 review) A CHECK listing codes (error_code, reason_code) must include every code any step writes.
- Phase 23 tables: admin.legal_holds, legal_hold_copies, retention_runs, retention_run_items, deletion_certificates,
  retention_tombstones, evidence_orphans (+ admin.hold_ref_seq; admin.evidence_objects.object_deleted_at); role akr_retention
  (the ONLY role with DELETE in canon and kyc; partitions only via admin.retention_drop_partition); Cedar 61_retention.cedar;
  workflow legal_hold_release (resource kind legal_hold); transit key akr-sign-cert; migrations 0071-0072 (next 0073).
- (phase 22 review) Phase 00's outbox routes each topic to ONE TopicHandler (one `dispatched_at` per row). Before consuming a
  topic, grep which phase already handles it; a second consumer needs the registry to become a handler list (gap item).
- (phase 22 review) Scope a record by the column that addresses it (`rfi.recipient_ref`, the assignee, the grantee), never by a
  proxy (the party's accounts): a proxy shows the record to everyone the proxy reaches. Grep the column's real DDL.
- (phase 22 review) A claim, approval or attestation must never be satisfied by its own maker: read the earlier phase's CHECKs
  (corroborated_by, documented iff evidence_object_id) before writing rows into its tables.
- (phase 22 review) A NOT NULL subject column (party_id) blocks real cases (walk-ins, third parties): ask whether the subject
  can be unknown before making it required.
- (phase 22 review) Service names clash across phases: grep "struct Name\|`Name`" in docs/plan/phase-*.md before naming one.
- Phase 22 tables: casework.observations, liaison_monthly_reports, admin.notifications, notification_templates,
  notification_template_versions, user_notification_prefs; casework.alarms kinds gain liaison_report_overdue (subject
  liaison_report); BranchWindowService (phase 03 keeps BranchService); NotificationDispatcher handles notify.request plus
  13 earlier topics; notification.new on phase 17 CountsHub; Cedar B0_branch.cedar; migrations 0073-0075 (next 0076).
- (phase 24 review) Any function or predicate that decides a record's tier must mirror phase 17 tier rule 2 in full: the
  party tier (casework.effective_tier), the case family tier, the tier of every case the alert belongs to (case_alerts) and
  `suspicion` when scn.scenarios.visibility = 'restricted'. A tier test alone is not phase 17 rule 4: restricted rows are
  for A1, A2 and M with a purpose; use phase 17 `services::visibility::build` for case and alert lists.
- (phase 24 review) Never widen akr_readonly (NOLOGIN, no USAGE on canon, nothing on casework): when it must learn a fact
  from those schemas, use a SECURITY DEFINER function owned by akr_migrate with `SET search_path = pg_catalog`, REVOKE
  from PUBLIC and EXECUTE to akr_readonly.
- (phase 24 review) A settings floor bounds the side that weakens the control: a ratio that triggers review gets a max, a
  time box gets a max, a deadline gets min and max.
- (phase 24 review) A route's scope list must admit every role its Cedar policies permit (AS holds `assessor.read` only);
  check each permit against the route's scopes.
- (phase 24 review) A designed exception (an assessor seeing suspicion items) is bounded to its recorded artefact (the
  sample drawn under the grant in force), never to the whole register.
- Phase 24 tables: admin.registers, validation_packs, protected_attributes, fairness_metrics, training_records,
  assessor_samples, verify_runs; admin.packs kind certification; audit.tier_rank, audit.resource_tier (SECURITY DEFINER);
  GovernanceRegisterService (phase 19 keeps RegisterService); readonly_pool in akr-api; Cedar 60_audit.cedar and
  C0_governance.cedar; migrations 0076-0079 (next 0080).
- (phase 25 review) A unique key that spans requesters (one open request per case) both leaks (the second requester
  learns a request exists) and blocks (one person holds the path shut); key by requester and subject, every request
  inserts its own row, and never ON CONFLICT DO NOTHING on a row the audit trail names.
- (phase 25 review) A read-only exception is an allow-list of the read actions under its purpose, never a list of
  forbidden writes: team membership feeds later team checks a deny list misses.
- (phase 25 review) Nothing that leaves the bank (SIEM, support bundle, notice) carries the tier value; a stripped row is
  recognisable only by its empty fields.
- (phase 25 review) Before granting a role access to case or alert rows, check phase 17 rule 1 (only A1, A2, M reach
  them): a grant to any other role opens nothing.
- (phase 25 review) A SQL list predicate and its Cedar adapter take the same inputs (purpose, step-up, time): a clause
  without the purpose shows in a list what the single read masks.
- Phase 25 tables: admin.access_reviews, access_review_items, break_glass_grants, break_glass_log, releases,
  help_articles, support_sessions, support_bundles, siem_cursors, siem_dead_letters, anchor_timestamps;
  admin.role_grants.support_session_id (source support_session); audit.siem_page (SECURITY DEFINER); notice codes
  security.break_glass_waiting, security.break_glass_opened (phase 22 list now 17); scope siem.pull; Cedar
  51_platform.cedar; migrations 0080-0082 (next 0083).
- (phase 26 review) Before setting a column at insert, read every CHECK on its table: phase 20 `ops_tasks` allows
  `second_person_only` only with `verification_level = 'v0'`, so setting it at creation failed and rolled back a freeze.
- (phase 26 review) A decision on a document needs a route that opens the document; no generic `GET /evidence/{id}`
  exists, content is served per owner (phase 18 `GET /cases/{id}/evidence/{evidence_id}/content` pattern).
- (phase 26 review) Register and catalogue sources are closed lists: phase 24 `admin.registers.source` is one of manual,
  post_review, fairness, review_calendar, scan, and its text column is `body`. Phase 02 `access_event.action` is one of
  view, search_hit, export, print, download, unmask.
- (phase 26 review) G1 (command centre) is `dashboard.read` (M, CH, IA); A1 and A2 never see a G1 tile.
- (phase 26 review) Phase 23 granted `akr_retention` SELECT ON ALL TABLES at its own migration time; every later table
  needs its own `GRANT SELECT, DELETE ... TO akr_retention`.
- Phase 26 tables: canon.islamic_deals (bi-temporal), canon.deal_parties (superseded_at), kyc.possession_evidence;
  view kyc.deal_evidence_status; canon.product.islamic_contract_type now twelve values; Cedar 88_islamic.cedar (80
  range: 80, 81, 82, 84, 86, 88 taken); migrations 0083-0084 (next 0085).
- (phase 27 review) A guard that freezes a column must still allow every edge the business logic documents (rollback by
  re-shadowing needed new criteria); list each allowed edge in the trigger and test it.
- (phase 27 review) A NOT NULL column computed from a nullable one fails at insert: check every computed value has a
  value in every state that inserts (shadow scores needed a provisional `top_band_cut`).
- (phase 27 review) Every route in the section 2 table needs its steps in section 3; a pointer to a section with no
  steps is a missing route.
- (phase 27 review) Phase 18 raises the subject party to `suspicion` on every utr, str and tfr decision: a count that
  excludes suspicion tier loses every report decision. Count every tier inside the job; let only suppressed window
  aggregates out (phase 24 `str_rate` pattern).
- (phase 27 review) A read-only store needs a documented writer for every source that fills it (host command, import).
- Phase 27 tables: admin.models, admin.model_versions, admin.model_evaluations, admin.model_kill_state,
  admin.model_monitor_points, alert.model_scores (FK into partitioned alert.alerts, RESTRICT); phase 24
  `registers.source` + `model_monitor`, `fairness_metrics.metric` + `score_top_band_rate`; Cedar C1_models.cedar (C0_*
  and C1 taken); migrations 0085-0087 (next 0088).
- (phase 28 review) A gate that evaluates a candidate needs a runtime that can hold the candidate: when production
  serves only the live version, add a separate evaluation server, or nothing can ever go live.
- (phase 28 review) A promotion of a set must allow changing one member: keep named versions that are already live and
  retire only the ones replaced; refuse a request that changes nothing.
- (phase 28 review) A new QA population needs an item a reviewer can open: widen phase 19 `source_kind`, say what
  `GET /qa/samples/{id}/item` renders, and tie samples to their run with a column.
- (phase 28 review) A job that reads case text for an evaluation runs as the system with a named purpose (`qa_review`),
  never with the visibility of the person who started it.
- (phase 28 review) A UNIQUE over a nullable column never fires for the NULL rows: use `UNIQUE NULLS NOT DISTINCT`.
- (phase 28 review) A text limit must count every prefix or suffix the code adds before the downstream check.
- (phase 28 review) The phrase "not carried" anywhere on a "Requirements carried" line drops the whole line from
  `tools/idcount.py`; write "left to a later task" inside an "Implemented here" line.
- Phase 28 tables: admin.assistant_versions, admin.assistant_evaluations, admin.assistant_kill_state,
  casework.assistant_drafts; phase 24 `registers.source` + `assistant_monitor`; phase 19 `qa_samples` population +
  `assistant_golden`, `assistant_assisted`, source_kind + `assistant_draft`; flag `pack.assistant`; Cedar
  C2_assistant.cedar (C0_*, C1, C2 taken); migrations 0088-0090 (next 0091).
- (phase 29 review) Reuse an earlier rule instead of restating it: list freshness is phase 14 `ListRecord::stale`; a
  restated rule with a grace drifted past the CBI 12-hour floor.
- (phase 29 review) A control that relies on an expiring token must check the expiry: a release matched to any pass
  missed releases made after the pass expired.
- (phase 29 review) A system-made record uses the run kind and requester the owning phase defines for system work
  (phase 14 `payment` run, `requested_by` NULL), and must not duplicate what the owning phase's own job creates later.
- (phase 29 review) A job or watch of an optional pack is idle while the pack flag is off; otherwise every install
  without the pack raises warnings forever.
- (phase 29 review) A readiness criterion must be able to fail in the state it is checked in; a column that is NULL
  outside one mode needs `coalesce` in roll-ups.
- Phase 29 tables: assess.rails, gateways, signing_keys, decisions (partitioned), idempotency, node_status,
  blind_periods, policy_state, amount_bands, fail_policies, rail_modes, prohibited_rules, holds, hold_candidates,
  control_state, release_exceptions, control_tests, mode_rollups, fallback_extracts; phase 24 `registers.source` +
  `assess_control`; flag `pack.assess`; Cedar D0_assess.cedar (D0 taken); migrations 0091-0094 (next 0095).
- (phase 30 review) Enforce a limit for every case by a rule, not for a hand-picked list: cluster-wide concurrency
  listed twelve kinds and left the rest running once per replica.
- (phase 30 review) A read-only copy cannot serve anything that writes on the way: sign-in (Kratos sessions) and
  audited reads (`audit.access_event`) both write.
- (phase 30 review) Write another phase's table through its owning service (phase 11 `BlindPeriodService`), which
  carries rules such as never reaching back past the newest anchor.
- (phase 30 review) A per-node fallback of a shared limit divides the limit by the replica count.
- Phase 30: no table, no route; migration 0095 (refusal code `chart_signature_invalid`, settings `ops.last_failover_drill_at`,
  `ops.failover_drill_max_age_days`, `ops.failover_max_s`, `ha.standby_reads`, `ha.standby_max_lag_s`); next 0096.
- (phase 31 review) An unattended external signal never triggers an irreversible act: a directory deactivation
  disables (reversible), the final offboarding stays a person's act with step-up.
- (phase 31 review) A score taken after the answer was shown is not a pass: score the first attempt only.
- (phase 31 review) A rule credited to another phase must exist there; grep it, or state it as this phase's rule.
- (phase 31 review) A webhook route meant for one internal caller must be refused at the edge, not only signed.
- Phase 31: tables train.practice_cases, train.practice_attempts (view train.practice_scores), casework.capacity_inputs,
  capacity_snapshots, capacity_runs, rfi_packs, rpt.typology_exposure, admin.directory_users, directory_events; flags
  `pack.training`, `pack.capacity`, `pack.exposure`, `pack.correspondent`, `pack.polis`; scopes `capacity.plan`,
  `directory.sync`; Cedar E0_directory, E1_training_capacity, E2_exposure_packs; migrations 0096-0100 (next 0101).
