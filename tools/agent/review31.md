# Phase 31 review findings (lead, 2026-10-09)
Draft: docs/plan/phase-31.md (1088 lines; migrations 0096-0100; 9 tables + 1 view; 15 routes; 5 jobs; Cedar E0, E1,
E2; DR-338..355; 25 open doubts). Checked true against earlier phases: packs_kind_check already lists correspondent
and validator (phase 06 l.259, phase 24 l.238); alarms CHECK lists = phase 22 l.41-48 (latest); feature_flags INSERT
form (phase 29 l.660); retention clocks (phase 00 l.304-310); data_classification values; admin.pools (phase 01
l.102); alert columns triaged_at, priority_class, touch_due_at, state 'triaged' (phase 17); assignee_pool_id;
scn.replays / replay_results / would_alert (phase 16); matched_txn_ids; balance_lcy (phase 10); visibility 'normal'
(phase 06); is_calibration (phase 19); harness_runs recall_clean, passed_gate (phase 14); admin.dq_issues,
coverage_certificates (phase 12); casework.rfis recipient_ref, deadline_at; scope rfi.desk; outcomes no_report/utr/str/tfr
(phase 18); OffboardingService (phase 03 l.886).

1. MEDIUM a SCIM deactivate runs the phase 03 offboarding with no person and no step-up (s.3.11 step 5, s.3.12), and
   phase 03 offboarding is final ("a returning leaver is a new account", phase 03 l.844-845). One forged or mistaken
   directory event (or a compromised webhook secret, or an IdP admin error) permanently ends every linked account except
   the last ADM and SEC holders. Fix: a directory deactivate DISABLES the account through the phase 03 disable path
   (status `disabled`, reason `admin`, sessions and tokens revoked, audit `user.disabled` with `source: directory`),
   which is reversible; the final offboarding stays the phase 03 act of an administrator with step-up. The
   reconciliation lists linked accounts disabled by the directory for more than `directory.pending_alert_days` to ADM
   (count audit). `enable` stays refused while `directory_active` is false. Rename `offboard_directory` to
   `disable_from_directory` (extends phase 03 `UserAdminService::disable` or the real name; grep), outcome
   `offboarding_deferred` -> `disable_deferred`. Open doubts 16 becomes the lead decision.
2. LOW the edge (s.3.10 Edge, s.4.4) routes only `/sso/*` and `/scim/v2/*` to Polis, but the browser must also reach
   the Polis OAuth authorize endpoint and the SAML assertion consumer service (Kratos redirects the browser there), and
   Caddy's phase 00 rule `/api/*` -> akr-api exposes `POST /api/v1/directory/events` to the bank network (HMAC is the
   only guard). Fix: the edge exposes exactly the pinned Polis version's authorize, ACS and SCIM paths (listed from the
   pinned version, Open doubts 13), and Caddy refuses `/api/v1/directory/events` (Polis reaches akr-api directly on
   `z3_edge`, the only caller); s.6 item 24 checks the 404 at the edge.
3. LOW `train.practice_scores.passed_once` counts a pass made after the trainee has seen the known answer and the expert
   path of the same case (they are returned by the first attempt), and G9 shows it beside the register and admins may
   cite it as evidence. Fix: the view scores only each user's FIRST attempt per case (`passed_first`, `cases_passed_first`,
   `attempts`); later attempts are practice and stay in `practice_attempts`; the G9 column and DR-339 say so.
4. LOW the alert-to-pool attribution of the snapshot (s.3.3 step 1, "the phase 17 routing rule") is not a phase 17
   rule: phase 17 queues have `pool_id` and `filter`, cases have `assignee_pool_id`, alerts have no pool. Fix: state it as
   this phase's attribution rule (new in phase 31): an alert counts in the pool of the first active `alerts` queue by
   `code` whose filter admits it, once; an alert admitted by none is counted in a `unassigned` total shown without a
   pool; Open doubts 4 notes the bank may prefer another attribution.
5. LOW Cedar `directory.event.webhook` (s.3.17) has no `@role` annotation and permits `user.offboard`; with finding 1 the
   action becomes the phase 03 disable action. Fix with finding 1; keep the policy keyed to `principal.kind == "service"`
   and `context.via == "webhook_hmac"` (route scope `directory.sync` admits only `polis-directory`), annotate
   `@role("service")` as phase 29 did for the gateway principal (grep the phase 29 form).
Severity: 0 critical / 0 high / 1 medium (1) / 4 low (2-5) = 5.

Gap-pass items (not draft defects): phase 03 disable from the directory + enable check + `Ctx.via` `webhook_hmac` +
service principal `polis-directory` creation; phase 19 QA sampler surge rate and `AlarmService` kind row; phase 24
`TrainingRegisterService::list` practice column; phase 06 pack builders; phase 00 zones (Polis rows), AppRoles
`akr-polis` and `kv/polis/directory`; phase 01 `db provision` database `polis`; API.md section 2 scopes
`capacity.plan`, `directory.sync` and the 13 added routes; DATA-MODEL 9 tables + view, schema `train` in the phase 00
schema list; DESIGN-SYSTEM W12, G2, W8, R4, G1 panel, G9 column, A1 panel; roadmap services list.
