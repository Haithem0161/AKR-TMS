p='docs/plan/phase-21.md'
t=open(p).read()
R=[]
def r(a,b,n=1): R.append((a,b,n))
# 3 liaison -> phase 22; counts
r("XL (33 routes: 18 rows of API.md 3.13 owned here, 15 rows added;","XL (32 routes: 18 rows of API.md 3.13 owned here, 14 rows added;")
r("`fiu_feedback`, `report_tasks`, `liaison` (read); `ReportService`","`fiu_feedback`, `report_tasks`; `ReportService`")
r("`letter_render`, `periodic_render`, `report_clock_scan`, `report_calibration`, `letter_backfill`; the three timer handlers of section 4.2;",
  "`ctr_package`, `letter_render`, `periodic_build`, `report_clock_scan`, `report_calibration`, `letter_backfill`; the two timer handlers of section 4.2;")
r("- `ReportQuery` (the read model of R1, R2, R3 and R5; the predicates of section 3.22).","- `ReportQuery` (the read model of R1, R2 and R3; the predicates of section 3.22).")
r("| GET | `/liaison-reports` | `case.decide` | Liaison monthly reports for review (R5): a read over phase 19's `internal_reports` with channel `liaison_monthly` (`internal_report.read`) [API; read here, Open doubts 16] |\n","")
r("Count: 33 routes; 18 rows of API.md 3.13 owned here, 15 added, one of them (`GET /liaison-reports`) an API.md row that no earlier phase owns.",
  "Count: 32 routes; 18 rows of API.md 3.13 owned here, 14 added. `GET /liaison-reports` and `POST /liaison-reports/{id}/review` (API.md 3.16) and screen R5 belong to phase 22, which owns the liaison monthly reports (lead review).")
r("`api::routes::fiu_feedback` (feedback register and report tasks) and `api::routes::liaison`.","`api::routes::fiu_feedback` (feedback register and report tasks).")
r("`fiu-feedback`, `report-tasks`, `liaison`. Schemas:","`fiu-feedback`, `report-tasks`. Schemas:")
r("10. **Phase 19**: `GET /internal-reports` gains the filter `channel` (R5 reads the `liaison_monthly` channel); `AlarmService::raise`","10. **Phase 19**: `AlarmService::raise`")
r("Screens are the DESIGN-SYSTEM section 8.1 catalogue rows R1, R2, R3 and R5. The DESIGN-SYSTEM specification exists for R1 only (section 8.3); R2, R3 and R5 have",
  "Screens are the DESIGN-SYSTEM section 8.1 catalogue rows R1, R2 and R3 (R5, liaison report review, is phase 22's). The DESIGN-SYSTEM specification exists for R1 only (section 8.3); R2 and R3 have")
i=t.index("### 5.4 R5 Liaison report review"); j=t.index("### 5.5 Generated hooks")
R.append((t[i:j],"",1))
r("### 5.5 Generated hooks","### 5.4 Generated hooks")
r("### 5.6 Browser and caching rules","### 5.5 Browser and caching rules")
r(", `usePostReportsPeriodicIdSign`, `useGetLiaisonReports`.",", `usePostReportsPeriodicIdSign`.")
r("Swagger UI shows the 33 routes of section 2 under the seven tags","Swagger UI shows the 32 routes of section 2 under the six tags")
r("every hook of section 5.5 exists","every hook of section 5.4 exists")
r("9. **API.md rows added here.** Fifteen rows of the route table are new (section 2), and `GET /liaison-reports` is an API.md row that no phase owns.",
  "9. **API.md rows added here.** Fourteen rows of the route table are new (section 2); the lead aligns API.md 3.13 in the same change.")
r("16. **R2, R3 and R5 detail.** The DESIGN-SYSTEM has no specification for R2, R3 and R5 beyond the catalogue. The liaison review action (`POST /liaison-reports/{id}/review` in API.md) has no owner; this phase reads the liaison reports and leaves the review to phase 19's acknowledgement.",
  "16. **R2 and R3 detail.** The DESIGN-SYSTEM has no specification for R2 and R3 beyond the catalogue. The liaison review (`GET /liaison-reports`, `POST /liaison-reports/{id}/review`, screen R5) is phase 22's; the periodic `liaison` kind reads phase 19's `internal_reports` (channel `liaison_monthly`) until phase 22's table exists.")
# 4 Cedar
r("94_reporting.cedar","A1_reports.cedar",6)
r("**DR-183 (the reporting Cedar file is `A1_reports.cedar`, as requested; the range conflicts with phases 03 and 07).** Phase 03 reserves the range `A0` for reporting, and phase 07 already ships `A0_reporting.cedar` with the goAML checker's actions. This phase uses `A1_reports.cedar` as the lead instructed, so phase 07's file is not edited; the bundle order keeps the file after `93_ops.cedar`. Open doubts 22 asks the lead to confirm the range in phase 03's table before the bundle is built.",
  "**DR-183 (the reporting Cedar file is `A1_reports.cedar`).** Phase 03 reserves the range `A0` for reporting, and phase 07 already ships `A0_reporting.cedar` with the goAML checker's actions. This phase adds `A1_reports.cedar` in the same range, so phase 07's file is not edited and the bundle order keeps the reporting policies together (lead review: the lead prompt's `94` would have put reporting in the casework range).")
r("22. **The Cedar range.** `A1_reports.cedar` is used as the lead asked; phase 03 reserves `A0` for reporting, and phase 07 ships `A0_reporting.cedar`. The lead confirms the range and adds `94` to phase 03's list before the bundle is built.",
  "22. **The Cedar range.** Resolved in the lead review: `A1_reports.cedar`, in phase 03's reporting range `A0`, after phase 07's `A0_reporting.cedar`.")
# 8 containsAny
r('when { principal.roles.containsAny(["A2", "M"]) };','when { principal.roles.contains("A2") || principal.roles.contains("M") };')
r('when { principal.roles.containsAny(["M", "CH", "IA"]) };','when { principal.roles.contains("M") || principal.roles.contains("CH") || principal.roles.contains("IA") };')
# 1 correction subject
r("- `ReportApprovalService::sign` (section 3.6). No new `WorkflowSubject`: the case subject of phase 18 is extended (section 4.5).",
  "- `ReportApprovalService::sign` (section 3.6). The case subject of phase 18 is extended for `case_report_approval` (section 4.5), and `ReportWorkflowSubject` (new, resource kind `report`) serves `report_correction` (section 3.7).")
r("**`ReportCorrectionEffect::run`** (kind `release_correction`, resource kind `case`; new in phase 21):","**`ReportCorrectionEffect::run`** (kind `release_correction`, resource kind `report`; new in phase 21):")
r('**The correction instance** (section 3.5 step 4): `WorkflowEngine::start(tx, StartSpec { definition_key: "report_correction", resource: ("case", case_id), purpose: Decision, maker: submitter })`, with `start_context` (the phase 18 `CaseWorkflowSubject`) freezing `{ report_id, version }`. The definition `report_correction` is shipped by a `cr_workflow` change request (DR-175): `sign` (MLRO from `case_acl`, step-up) then `release_correction`.',
  '**The correction instance** (section 3.5 step 4): `WorkflowEngine::start(tx, StartSpec { definition_key: "report_correction", resource: ("report", report_id), purpose: Decision, initiator: submitter, maker: submitter })`, with `start_context` of `ReportWorkflowSubject` freezing `{ report_id, version, case_id }`. The correction runs on the report, not on the case, because a poor grade or a late rejection usually arrives after the acknowledgement has let the case close, and phase 18 refuses every act on a sealed case (lead review). `ReportWorkflowSubject` (new; resource kind `report`) takes no case lock and writes no case event; its `check_before_decision` requires the report `validated`, `frozen_version = current_version`, no block in the latest run and `via = "report"`; its `Holders` are the users with role `M` at bank scope (scope `report.file`), with the report\'s submitter and earlier signers excluded, and no `case_acl` grant is required. The definition `report_correction` is shipped by a `cr_workflow` change request (DR-175): `sign` (M, step-up `signing`) then `release_correction`.')
r("1. **Phase 18 `CaseWorkflowSubject::check_before_decision`** (phase 18 section 3.8 d) gains, for the instances of `case_report_approval` and `report_correction`, the report checks",
  "1. **Phase 18 `CaseWorkflowSubject::check_before_decision`** (phase 18 section 3.8 d) gains, for the instances of `case_report_approval`, the report checks")
r("5. **Phase 04 registries and DSL E.4**: effect `release_correction` (resource kind `case`) registered; definition `report_correction` published by a `cr_workflow` change request: `sign` (M from `case_acl`, step-up `signing`) then `release_correction`.",
  "5. **Phase 04 registries and DSL E.4**: subject `ReportWorkflowSubject` and effect `release_correction` (resource kind `report`) registered; definition `report_correction` published by a `cr_workflow` change request: `sign` (M at bank scope, step-up `signing`) then `release_correction`.")
# 2 backfill
r("**Backfill** (`LetterBackfill`, job `letter_backfill`, one run, `akr-ctl letters backfill` in the install step of this phase, idempotent): phase 20's `RecordingLetterRequester` wrote `ops.letter_recorded` audit events with the kind, the subject kind and the subject id, but not the id it returned (Open doubts 17). For each event without a `letter.backfilled` event: create a letter row through the port's steps 1 to 4 with a new id, then write the new id into the owner's matching column through phase 20's `FreezeService::set_letter_id` (or `RefrainService::set_letter_id`), replacing the stub id; audit `letter.backfilled {letter_id, owner_kind, owner_id}`.",
  "**Backfill** (`LetterBackfill`, job `letter_backfill`, one run, `akr-ctl letters backfill` in the install step of this phase, idempotent): phase 20's `RecordingLetterRequester` returned an id that the owner stored (`freezes.committee_report_letter_id`, `second_report_letter_id`, `referral_letter_id`; `refrain_instructions.referral_letter_id`) with no letter row behind it. The backfill reads those columns where no `rpt.letters` row has the id and inserts the letter under that same id through the port's steps 1 to 4 (kind from the column, `requested_at` from the owner's matching time column, `requested_by` NULL), so no owner row is changed and no phase 20 method is added (lead review); audit `letter.backfilled {letter_id, owner_kind, owner_id}`.")
r("9. **Phase 20**: `RecordingLetterRequester` is removed from `AppState` and kept as a test double; `FreezeService::set_letter_id(kind, letter_id)` and `RefrainService::set_letter_id(letter_id)` are added (phase 20's owner columns are already granted for these writes); `FreezeService::letter_delivered(letter_id, delivered_at)` is added (Open doubts 4).",
  "9. **Phase 20**: `RecordingLetterRequester` is removed from `AppState` and kept as a test double; no phase 20 method is added for the backfill (section 3.15); `FreezeService::letter_delivered(letter_id, delivered_at)` is proposed only if gap pass 1 decides Open doubts 4 that way.")
# 5 completeness
r("the business date must be complete in phase 09's reconciliation view for the canonical cash source.",
  "the business date must be covered for every source that feeds `canon.txn`: phase 12 `CoverageRepository::certificates_for_date(D, Some(source))` returns a current certificate with status `issued` or `attested` (a certificate `pending_attestation` waits).")
# 7 threshold constant
r("- The CBI-set fixed value `CTR_THRESHOLD_USD = 10000` is a fixed entry of the same registry (`fixed: true`, the value the DSL pins), so a change is refused with `E_INVARIANT_LOCKED`.",
  "- The CBI-set value `CTR_THRESHOLD_USD = 10000` is a code constant of `domain::reporting`, not a registry entry (phase 09's registries hold clocks and switches only); it mirrors the DSL section 9.1 pinned `limit` (`min = max`), so no path can change it.")
r("- Fixed CBI value (phase 09 closed registry, `fixed: true`): `CTR_THRESHOLD_USD = 10000`.","- Fixed CBI value (a code constant of `domain::reporting`, lead review): `CTR_THRESHOLD_USD = 10000`.")
r("8. **Phase 09**: the closed `ClockRegistry` gains `report.filing_target` and `report.rework`; the closed fixed-value list gains `CTR_THRESHOLD_USD = 10000`.","8. **Phase 09**: the closed `ClockRegistry` gains `report.filing_target` and `report.rework`.")
r("(DSL section 9.1 `limit`: pinned, `min = max`, basis CBI; a change is refused with `E_INVARIANT_LOCKED`)","(a code constant mirroring the DSL section 9.1 pinned `limit`, `min = max`, basis CBI)")
# 9 retention
r("('rpt', 'reports',            'R6', 'closure', 'signed_at',     'Reports and STRs: seven years from the later of case closure and the report date (the sweep takes the later of the two)'),",
  "('rpt', 'reports',            'R6', 'closure', 'created_at',    'Reports and STRs: seven years from the later of case closure and the filing (the sweep takes the later); a CTR file has no case'),")
for a,b,n in R:
    c=t.count(a); assert c==n,(a[:70],c,n)
    t=t.replace(a,b)
open(p,'w').write(t); print('ok',len(R))
