import re
def edit(p, pairs):
    s=open(p).read()
    for a,b in pairs:
        assert s.count(a)==1,(p,s.count(a),a[:90]); s=s.replace(a,b)
    open(p,'w').write(s)

# ---- roadmap
edit('docs/plan/roadmap.md', [
("| 31 | Training, Capacity & Correspondent Packs | Training workspace with synthetic cases (W12), capacity workbench (G2), correspondent and validator packs, typology exposure index, RFIs to correspondents, SAML and SCIM through Ory Polis | L | P2 | 19, 24 |",
 "| 31 | Training, Capacity & Correspondent Packs | Training workspace in its own schema `train` with synthetic cases scored on the first attempt (W12; a display column on the phase 24 register, never a record); capacity workbench by team only from measured handling times, fluid forecast, sensitivity, `capacity_forecast` alarm, simulation on phase 16 replays, surge without pre-clear (G2; advice only, D21); correspondent and validator packs from system facts; typology exposure index (restricted scenarios excluded); correspondent list and pack attachment on the RFI desk; SAML through Ory Polis as the phase 25 OIDC upstream; SCIM creates no account and a deactivation disables, never offboards; five flags, all off | L | P2 | 19, 24 (also 00-04, 06, 09, 10, 12, 14, 16, 17, 21, 22, 25, 30) | Spec ready |"),
("| 31 | _(confirmed when the phase file is written; training and capacity data reuse cases and settings where possible)_ |",
 "| 31 | `train.practice_cases`, `train.practice_attempts`, view `train.practice_scores`; `casework.capacity_inputs`, `capacity_snapshots`, `capacity_runs`, `rfi_packs`; `rpt.typology_exposure`; `admin.directory_users`, `directory_events` |"),
("| 31 | `TrainingService`, `CapacityService`, `CorrespondentPackService` |",
 "| 31 | `TrainingService`, `CapacityService`, `CapacitySnapshotter`, `CapacityWatcher`, `SimulationHarness`, `ExposureService`, `CorrespondentPackService`, `DirectoryService`, `DirectoryWebhookVerifier`; `UserAdminService::disable_from_directory` (phase 03 extended); `RfiService::attach_pack`, `correspondent_list` (phase 19 extended) |"),
("| 31 | next numbers (from 0096) | 28 (when written) |",
 "| 31 | `0096_train_schema`, `0097_casework_capacity`, `0098_rpt_exposure`, `0099_admin_directory`, `0100_phase31_guards_grants_settings` | 28 |"),
])
s=open('docs/plan/roadmap.md').read()
m=re.search(r'^\| 0 \(B8 review: phase 30\) \|.*\n',s,re.M); assert m and 'B8 review: phase 31' not in s
row=("| 0 (B8 review: phase 31) | 2026-10-09 | 5 lead corrections to the Haiku draft | 0 | 0 | 1 | 4 | MEDIUM: a SCIM "
     "deactivation ran the final phase 03 offboarding with no person and no step-up, so one forged or mistaken directory event "
     "could end linked accounts for good (now a reversible phase 03 disable; offboarding stays an administrator's act). LOW: "
     "the edge missed the Polis authorize and assertion consumer paths and exposed the webhook route; practice scores counted "
     "a pass made after the answer was shown (first attempt only); the alert-to-pool attribution was credited to phase 17 "
     "(now this phase's rule, with an unassigned total); the Cedar webhook policy permitted `user.offboard` |\n")
s=s[:m.end()]+row+s[m.end():]
open('docs/plan/roadmap.md','w').write(s)

# ---- DATA-MODEL
edit('docs/DATA-MODEL.md', [
("letters R12 age on `created_at`; anchor time stamps R10 owner (RFC 3161, off by default).\n",
 "letters R12 age on `created_at`; anchor time stamps R10 owner (RFC 3161, off by default).\n\n"
 "### directory_users / directory_events (phase 31, DR-349 to DR-352)\n"
 "| id uuid pk | external_id text unique (the SCIM id) | user_name_enc bytea (sealed, personal) | display_name_enc bytea null (sealed) | state (pending, linked) | directory_active bool | linked_user_id fk null (unique) | linked_by fk null | linked_at null | created_at | updated_at | version |\n"
 "| id uuid pk | event_ref text unique (the Idempotency-Key) | directory_user_id fk null | op (create, update, deactivate, activate) | payload_sha256 bytea | outcome (applied, ignored, refused, deferred) | reason_code (last_holder, stale_event, reactivation_ignored, disable_deferred, unknown_user) null | received_at |\n\n"
 "Fed by Ory Polis through `POST /directory/events` (flag `pack.polis`). A directory row is not a sign-in account; the\n"
 "link is an administrator's act with step-up. A deactivation disables the linked account (phase 03, reversible), never\n"
 "offboards. Directory users R9 retirement; events R12 age on `received_at`, never updated.\n"),
("subject_kind (phase 20 adds freeze, refrain, ops_task; phase 21 adds report, report_task)",
 "subject_kind (phase 20 adds freeze, refrain, ops_task; phase 21 adds report, report_task; phase 22 adds liaison_report; phase 31 adds pool)"),
("phase 21 adds report_filing_overdue, report_rework_overdue), stage",
 "phase 21 adds report_filing_overdue, report_rework_overdue; phase 22 adds liaison_report_overdue; phase 31 adds capacity_forecast), stage"),
("built_batch_id, retired_at; edges from evidenced canon links only; `shared_device` unbuilt until a device source exists (phase 19, DR-154; renamed from entity_links by DR-145) | none | R3 |\n",
 "built_batch_id, retired_at; edges from evidenced canon links only; `shared_device` unbuilt until a device source exists (phase 19, DR-154; renamed from entity_links by DR-145) | none | R3 |\n"
 "| casework.capacity_inputs | id, pool_id, week_start (a Sunday), analysts_staffed (a count, no names), hours_per_day, shrinkage (max 0.600), occupancy_target, note_code, entered_by, entered_at, superseded_by, superseded_at, version; one live row per pool and week (partial unique index) (phase 31, DR-341) | none | R9 |\n"
 "| casework.capacity_snapshots | id, pool_id null (null = the unassigned total), bucket_start (hour), work_kind (alert, case), class_code, arrivals, decided, handling_minutes_sum, backlog_end, aged_over_sla, computed_at; unique nulls not distinct on (pool_id, bucket_start, work_kind, class_code); no person column (phase 31, DR-341) | none | R12 |\n"
 "| casework.capacity_runs | id, replay_id (scn.replays, completed), label, inputs jsonb, status (queued, running, completed, failed), result jsonb (estimates, no entity id or detection key), requested_by, requested_at, completed_at (phase 31, DR-342) | none | R12 |\n"
 "| casework.rfi_packs | rfi_id, pack_id (pk both), linked_by, linked_at; a pack attached to an outbound correspondent request in draft (phase 31, DR-347) | none | R6 |\n"),
("| periodic_reports | id, kind (cbi_monthly, liaison, board, mlro_annual, fatca, crs, analytics),",
 "| typology_exposure | id, period_month (first day), typology (research 27 number), scope_kind (bank, branch, correspondent), scope_ref null (bank), flows, value_lcy, total_flows, total_value_lcy, assets_lcy (LCY only), suppressed, computed_at; unique nulls not distinct on (period_month, typology, scope_kind, scope_ref); suppressed cells null; restricted scenarios excluded (phase 31, DR-346) | R7 |\n"
 "| periodic_reports | id, kind (cbi_monthly, liaison, board, mlro_annual, fatca, crs, analytics),"),
("mapped, never zero. In the ICS BANKS installation studied Murabaha agreements came from MUR_AGR, Qard Hasan had\npayment and schedule tables but no agreement table, and Mudaraba tables were empty.\n",
 "mapped, never zero. In the ICS BANKS installation studied Murabaha agreements came from MUR_AGR, Qard Hasan had\npayment and schedule tables but no agreement table, and Mudaraba tables were empty.\n\n"
 "## 20a. Training workspace (`train`, P2, phase 31)\n\n"
 "| Table | Columns | Class |\n|---|---|---|\n"
 "| train.practice_cases | id, case_code (PRAC-XX-NNN), course_code, path (l1, l2, mlro, rfi_desk, screening, tipping_off), typologies smallint[], title jsonb, facts jsonb (`synthetic: true` by CHECK), known_answer jsonb, expert_path jsonb (both detection-logic, shown only after the trainee's own attempt), pack_version, active, created_at; written only by `akr-ctl training seed` | R9 |\n"
 "| train.practice_attempts | id, case_id, user_id, decision (no_report, utr, str, tfr) or choice (1-9), reason_codes text[], decision_match, reasons_matched, passed, attempted_at; never updated; deleted only by the retention run | R9 |\n"
 "| train.practice_scores (view) | user_id, course_code, passed_first, cases_passed_first, cases_attempted, attempts, latest_first_attempt_at; scored on each user's first attempt per case only | - |\n\n"
 "No table, view or function outside schema `train` references it (DR-338); no count, register total, report, pack,\n"
 "export or goAML file reads it. The phase 24 training register shows `practice_scores` as a display column only\n"
 "(DR-339).\n"),
("| `akr_app` | Read and write on application schemas (`admin`, `alert`, `casework`, `kyc`, `scn`, `rpt`, `ref`, `feat`);",
 "| `akr_app` | Read and write on application schemas (`admin`, `alert`, `casework`, `kyc`, `scn`, `rpt`, `ref`, `feat`, `train`);"),
])
s=open('docs/DATA-MODEL.md').read()
m=re.search(r'^\| 27 \| `admin`: the widened `admin.releases.refusal_code`.*\n',s,re.M); assert m and '| 28 |' not in s[m.end():m.end()+10]
s=s[:m.end()]+("| 28 | `train`: practice cases, attempts, the view `practice_scores` (0096); `casework`: capacity inputs, snapshots, "
 "runs, RFI packs, the widened `alarms` kind and subject CHECKs (0097); `rpt`: typology exposure (0098); `admin`: directory "
 "users and events (0099); guards, grants, five flags, fourteen settings, classification and retention (0100) | 31 |\n")+s[m.end():]
open('docs/DATA-MODEL.md','w').write(s)

# ---- MODULES
edit('docs/MODULES.md', [
("or the 8 h lifetime (phase 25 Open doubts 4). SAML and SCIM: phase 31.\n",
 "or the 8 h lifetime (phase 25 Open doubts 4). SAML and SCIM: phase 31.\n"
 "Phase file: docs/plan/phase-31.md (DR-348 to DR-353): SAML through Ory Polis (flag `pack.polis`) as the phase 25\n"
 "generic OIDC upstream, no authority from the assertion; the edge exposes only the pinned Polis paths. SCIM through an\n"
 "HMAC-verified webhook (`POST /directory/events`, service principal `polis-directory`): a create writes a pending\n"
 "directory row and never a sign-in account; the link is an ADM act with step-up; a deactivation disables the linked\n"
 "account (phase 03, reversible) and never offboards; groups never grant.\n"),
("are phase 17), replays with reproducible blind samples, seeded known-answer cases, the CBI importer; S-48 and the\noutcome measures of S-53 are phase 24.\n",
 "are phase 17), replays with reproducible blind samples, seeded known-answer cases, the CBI importer; S-48 and the\noutcome measures of S-53 are phase 24.\n"
 "Phase file: docs/plan/phase-31.md (DR-346): the typology exposure index per month, bank, branch and correspondent\n"
 "account (matched posted flows and LCY value against totals and balances; restricted scenarios excluded; small numbers\n"
 "suppressed; flag `pack.exposure`).\n"),
("RFIs, watches, hibernations, network and QA are phase 19.\n",
 "RFIs, watches, hibernations, network and QA are phase 19.\n"
 "Phase file: docs/plan/phase-31.md (DR-338 to DR-344, DR-347): the training workspace in schema `train` (synthetic\n"
 "cases, scored on the first attempt, screen W12); the capacity workbench by team only (measured handling times, fluid\n"
 "forecast, sensitivity, `capacity_forecast` alarm, simulation on phase 16 replays, surge QC rate; screen G2; advice\n"
 "only, D21); the correspondent list and pack attachment on the RFI desk (W8). Not carried: M-50, M-55, the surge\n"
 "pre-clear, the growth, latency and silence alarms.\n"),
("(recorded by ADM, never for oneself); screens G1, G8, G9, R4. Not carried: the capacity workbench, correspondent and\nvalidator packs, the typology exposure index (P2), redaction and the requester watermark (Open doubt 11).\n",
 "(recorded by ADM, never for oneself); screens G1, G8, G9, R4. Not carried: the capacity workbench, correspondent and\nvalidator packs, the typology exposure index (P2), redaction and the requester watermark (Open doubt 11).\n"
 "Phase file: docs/plan/phase-31.md (DR-339, DR-345, DR-346, DR-355): correspondent and validator packs from system\n"
 "facts (flag `pack.correspondent`); the typology exposure panel on G1 (not a tile); the practice column on the G9\n"
 "register (display only, never a record or a count); the capacity forecast route of 3.16.\n"),
])
print('ok')
