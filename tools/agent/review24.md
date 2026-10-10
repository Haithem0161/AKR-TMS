# Phase 24 review findings (lead, 2026-10-09)
1. HIGH audit viewer tier: `audit.resource_tier` (s.1.3) resolves an alert's tier from `casework.effective_tier(party)`
   only and a case's from `casework.cases.tier`, missing phase 17's tier rule 2 (phase-17 l.1220: the family tier and
   `suspicion` when the alert's scenario is restricted) and an alert's own case (an alert row with no `case_id` in the
   audit row but linked to a suspicion case through `casework.case_alerts`). IA and SEC would see the audit rows of
   restricted-scenario alerts and of alerts inside suspicion cases. `access_event` rows are filtered on the tier stored
   at access time, so rows recorded before a raise stay visible. The function is SECURITY INVOKER as `akr_readonly`,
   which has no USAGE on schema `canon` (phase 00 l.70) and, by phase 17 l.565, no grant on the casework tables; the
   five column grants contradict that rule. Fix: `audit.resource_tier` is SECURITY DEFINER owned by `akr_migrate`
   (search_path pinned, EXECUTE to `akr_readonly` only) and mirrors phase 17 rule 2: case = the case tier and the
   family tier; alert = the highest of the party tier, the family tier of its case, the case tier through
   `case_alerts` and `suspicion` for a restricted scenario; `access_event` rows are filtered on the higher of the stored
   tier and the current tier; the column grants and the `effective_tier` grant to `akr_readonly` are dropped.
2. MEDIUM tier-only predicates for CH and IA: the decisions register (s.3.4: `cases.tier <> 'suspicion' OR M`) and the
   clocks board (s.3.8: suspicion rows excluded for all but M) admit restricted-tier cases to CH and IA, which phase 17
   rule 4 limits to A1, A2 and M with an `assigned_work` purpose. Fix: rows tied to a case or an alert pass phase 17's
   `services::visibility::build` (with phase 18's case access list and phase 19's QA grants); CH and IA see
   normal-tier rows and what their own grants admit; other subjects are counted (suppressed) on G1 only.
3. MEDIUM assessor trace unbounded: `GET /assessor/trace/{alert_id}` lets AS trace any alert id of any tier during the
   grant, so the designed exception (Open doubt 1) covers the whole alert register, not the recorded sample. Fix: AS
   traces an alert only when it is an item of a sample drawn under the same grant (or of a report in such a sample) or
   when the alert is normal tier; otherwise the masked 404.
4. MEDIUM assessor pack unreachable: `POST /packs`, `GET /packs/{id}` and the download carry scopes `dashboard.read` and
   `data.steward` (phase 06); AS holds `assessor.read` only, so the `pack.assessor.as` permit can never be reached
   (403 `E_SCOPE`). Fix: the three routes also accept `assessor.read`; Cedar keeps AS to kind `assessor`.
5. LOW settings floors point the wrong way: `fairness.review_ratio` `{"min": 1}` lets a bank set 100 and never review;
   `assessor.grant_max_days` `{"min": 1}` lets a 10-year "time-boxed" grant. Fix: `{"max": 1.25}` and `{"max": 30}`;
   `register.finding_due_days` gains `"max": 365` beside its min 30.
6. LOW retention clock: `admin.registers` R9 `age` on `updated_at` makes an untouched open finding due (Open doubt 21).
   Fix: `closure` on `closed_at` (an open row is never due).
7. LOW control report not suppressed: counts of tier raises and suspicion denies per month go to SEC without the
   small-number rule. Fix: the same suppression as the command centre.
8. LOW conventions and names: multi-role `@role` tags and `containsAny` (conventions: one role per `@role`; phase 21
   replaced `containsAny`); `context.pack_kind` read without `context has pack_kind` in `pack.board` and
   `pack.certification.create`; decisions view columns `case_ref` and `outcome` are not `casework.decisions` columns
   (`case_ref` comes from the case join; `outcome` dropped); `AuditVerifyService` appears only in AppState (the verify
   methods belong to `AuditViewerService`); classification uses `, unnest(...)` instead of `CROSS JOIN LATERAL unnest`.
Severity: 0 critical / 1 high (1) / 3 medium (2, 3, 4) / 4 low (5-8) = 8.
Gap-pass items (not draft defects): phase 06 step-up class `pack_download` is not in phase 01's closed list (use
`export`); phase 06 pack routes accept `assessor.read`; phase 02 evidence owner kinds `validation_pack`,
`training_record`; phase 03 MASKED_KINDS `audit_event`, `audit_target`; phase 00 AppRole akr-api KV `db/akr_readonly`;
phase 04 ChangeTarget `protected_attribute`, `InventoryService::update` called by a validation pass; phase 16
`check_before_decision` calls `ProtectedAttributeCheck`, the scenario activation column; phase 18 case review column for
clock `case.review`; canary records (research 30 s.8.6, 8.8) built by no phase; dev user `as@bank.test`; API.md
`GET /audit/events/{id}` needs `occurred_at`; phase 23 disposal handlers for the seven tables.
