# Phase 21 review findings (lead, 2026-10-09)
1. HIGH correction after closure impossible: `report_correction` runs on resource kind `case` through phase 18
   `CaseWorkflowSubject::check_before_decision`, which refuses a sealed case. A poor grade or a late rejection arrives after
   the acknowledgement, which is exactly what lets the case close (3.10a), so a returned or rejected report of a closed case
   could never be corrected. Fix: `report_correction` runs on resource kind `report` with a new `ReportWorkflowSubject`
   (no case lock, no case event; checks the report state; Holders: role M at bank scope, the report's submitter and earlier
   signers excluded; no case_acl requirement); `release_correction` registers on resource kind `report`.
2. MEDIUM letter backfill: the draft re-creates stub letters under NEW ids and rewrites phase 20 owner columns through new
   phase 20 methods (`set_letter_id`), matching audit events by (kind, subject). The owner columns already hold the stub ids.
   Fix: backfill reads the owner columns (freezes committee/second/referral letter ids; refrain referral letter id) that have
   no rpt.letters row and inserts the letter under THAT id; no phase 20 method, no audit-event parsing.
3. MEDIUM liaison ownership: GET /liaison-reports and screen R5 are taken as a read over phase 19 internal_reports, leaving
   API.md's POST /liaison-reports/{id}/review unowned; roadmap phase 22 owns `liaison_monthly_reports` and the monthly liaison
   reports. Fix: both rows and R5 go to phase 22; phase 21 has 32 routes and screens R1, R2, R3; the periodic `liaison` kind
   reads phase 22's table (until then phase 19 internal_reports channel liaison_monthly).
4. MEDIUM Cedar range: the prompt's 94 was wrong (90 range = casework). Phase 03 reserves A0 for reporting and phase 07 ships
   A0_reporting.cedar. Fix: `A1_reports.cedar`; DR-183 and Open doubts 22 rewritten.
5. MEDIUM CTR completeness gate cites "phase 09's reconciliation view": reconciliation and coverage certificates are phase 12.
   Fix: `CoverageRepository::certificates_for_date(D, source)` for every source feeding canon.txn (a certificate of status
   issued or attested; pending_attestation waits).
6. LOW StartSpec lacked `initiator` (phase 04 fields: definition_key, resource, purpose, initiator, maker).
7. LOW CTR_THRESHOLD_USD put in phase 09 ClockRegistry, which holds clocks and switches only; it is a code constant that mirrors
   the DSL 9.1 pinned `limit` (min = max).
8. LOW Cedar `containsAny` breaks the house idiom `principal.roles.contains("X")` (two policies).
9. LOW reports retention clock column `signed_at`: a CTR file is never signed, so it would never fall due. Fix: clock column
   `created_at` with the note that the sweep takes the later of the case closure and the filing.
Severity: 0 critical / 1 high (1) / 4 medium (2, 3, 4, 5) / 4 low (6, 7, 8, 9) = 9.
