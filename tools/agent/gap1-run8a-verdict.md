# Gap pass 1 run 8a: lead verdict (2026-10-10; decisions final, applied by apply-g1-run8a.py)

Haiku draft tools/agent/gap1-run8a.md: 69 market IDs (VA 56, VO 13); CARRIED 12, Partly 34, NOT-CARRIED 20, GAP 2 (one
finding, HIGH), UNCLEAR 1. The draft found that VA-27 to VA-56 are in files 09, 10 and 11, not 08 (the prompt was
wrong there). The prompt also wrongly called phase 26 P2: it is P1, optional per bank.
Lead result: implemented 13, Partly 36, Not carried 20. GAP accepted, rewritten and moved to phase 24.

## GAP finding
- G1-24-01 VA-54 and VO-03, parallel run against the bank's current system, ACCEPTED HIGH (DR-402).
  - In scope: ROADMAP P1 "Parallel run (two to three months)": "AKR-TMS runs beside the bank's current system on live
    data; every alert compared both ways; every disagreement explained in a log reviewed weekly with the MLRO". No
    phase imports the current system's alerts or keeps the disagreement log (grep "incumbent", "current system",
    "parallel run" in docs/plan: nothing).
  - Moved from phase 16 to phase 24: the comparison reads phase 17's `alert.alerts`, and phase 24 owns governance
    (registers screen G4).
  - Rewritten:
    - Not a `landing_file` source mapped into `scn`: mappings fill `canon`. Instead, a CSV upload staged through
      `EvidenceStore::stage` (owner kind `parallel_import`, kind `upload`), parsed by job `parallel_import`. The
      import id is the job id (no import table).
    - One table `alert.parallel_run_rows` (created by phase 24's `0077`, granted in `0079`): each row is a match
      (`both`), a disagreement (`akr_only`, `incumbent_only`) or an `unresolved` customer number. The explanation is on
      the row (closed `PRL_` codes plus an optional scanned note).
    - Matching is by party and date within `parallel_run.match_days` (3), not by scenario: the two systems' rule
      taxonomies differ.
    - Disagreements are not written to `admin.registers`. Register readers include CFG and IA, and a party id there
      would leak suspicion. The log is the table itself, readable by M and CH only: new Cedar actions
      `parallel_run.read` and `parallel_run.write` in `C0_governance.cedar` (no existing action has exactly M and CH),
      lists filtered by the phase 18 party visibility.
  - Routes: `POST /parallel-run/imports`, `GET /parallel-run/imports/{id}`, `GET /parallel-run/rows`,
    `POST /parallel-run/rows/{id}/explanation`, `GET /parallel-run/summary`. API.md rows added. DATA-MODEL section 12
    and the roadmap entity row 24 updated.

## Classification decisions
- CARRIED accepted (11): VA-03, VA-04 (11); VA-05 (20); VA-11 (14); VA-14,
  VA-30, VO-11 (21); VA-16 (12); VA-18 (08); VA-19 (06); VA-39 (17). With VA-54 and VO-03 from the accepted gap (24): 13.
- Moved from CARRIED to Partly: VA-46 (phase 01). Personal access tokens carry only read scopes at AAL1, so a bank's
  own agent can read under the user's roles but cannot act.
- Moved from NOT-CARRIED to Partly: VA-26 (phase 26, P1 optional per bank). The products are the closed product list
  with Islamic contract types, and the ISL scenarios use them; mudaraba and profit-distribution checks are not carried
  (phase 26, I-09).
- Partly accepted, header naming only the built part: VA-06, VA-07, VA-27, VA-28, VA-29, VA-35, VA-37, VA-43, VA-45,
  VA-55, VO-02, VO-09 (16); VA-08 (19); VA-09, VO-01 (21); VA-12, VA-23 (24); VA-13, VA-48 (18); VA-15 (12); VA-17,
  VA-24 (11); VA-21 (00); VA-22, VA-32, VO-10 (14); VA-25, VA-38, VA-47, VA-56, VO-08 (25); VA-31 (10); VA-36 (15);
  VA-42 (22). The missing parts are on no P0/P1 line (DR-402).
  - VA-35: change requests already show the diff (`GET /change-requests/{id}`); the export of definitions is missing.
  - VA-37 and VA-43: the 30-day shadow minimum is the bank-set `scenario.shadow_min_days` (S-47). This is no conflict.
- NOT-CARRIED accepted (19): VA-01, VA-20, VA-34, VA-50, VO-05, VO-06, VO-14 (25, contract and commercial terms);
  VA-02 (23, DR-396); VA-10, VO-12 (28); VA-33 (05); VA-40 (31); VA-41 (10); VA-44 (30); VA-49 (21); VA-51 (06);
  VA-53 (11); VO-04 (29).
- VA-52 (cross-bank signal sharing) moved from phase 19 to phase 27, beside the cross-bank items T-52 to T-58.
- VO-13 (the table stakes as the version 1 release gate) is Not carried on phase 25: a release-process note; the
  ROADMAP P1 exit criteria are the gate.

## Other edits
- Phase 04 said the workflow designer canvas is phase 26 and called it D3. Phase 26 is the Islamic pack, D3 is the
  policy studio (phase 09), and no P0/P1 line names a workflow designer. Both places now say so (DR-402).

DR: DR-402 (the parallel run in phase 24; the run 8a items outside this release).
