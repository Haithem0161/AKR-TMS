# Gap pass 1 run 7a: lead verdict (2026-10-10; decisions final, applied by apply-g1-run7a.py)

Haiku draft tools/agent/gap1-run7a.md: 54 IDs (UX 31, W 23) plus gap item (ab) X2; CARRIED 13, Partly 20,
NOT-CARRIED 13, GAP 8 (HIGH 1, MEDIUM 5, LOW 2).
Scope sources: MODULES and ROADMAP P0/P1 lines, and for UX items the DESIGN-SYSTEM screen catalogue (P0 and P1 rows)
and rules. All 8 gaps cite one of them, and all 8 are accepted; four are rewritten.
Lead result: implemented 18, Partly 24, Not carried 12.

## GAP findings (all accepted)
- G1-18-01 X2 with UX-29, HIGH (DR-400).
  - In scope: MODULES line 501 lists X2 among the M12 P1 screens; DESIGN-SYSTEM X2 is P1.
  - Rewritten as `GET /search` in phase 18, groups `party` and `case`.
  - The verified names it uses: `services::visibility::build`, the section 3.10 party visibility, Cedar `party.read`
    and `case.read`, the `case_ref` format `CS-YYYY-NNNNNN`, the keyed hashes `iban_hk`, `number_hk`, `pan_hk` and
    `value_hk`.
  - One access event `search_hit` with resource type `global_search` (pattern of phase 14 section 7.9). A suspicion
    party outside the access list is absent. Reports and screening alerts are never searched.
  - The alert group is dropped: alerts are reached from the party.
- G1-18-02 UX-07, MEDIUM (DR-400).
  - In scope: DESIGN-SYSTEM 7.14 and W3 (P1, "entity timeline" in the evidence column).
  - Adds `GET /parties/{id}/timeline`, shown in the W3 evidence column for the case subject (the W4 tab list is
    unchanged).
  - Sources at phase 18: alerts, cases, decisions and rating changes. RFIs, authority requests and QA join from phase
    19, operations tasks from phase 20, observations from phase 22. The seasonal bands come from `ref.calendar_days`.
- G1-18-03 UX-59, LOW: phase 15 promised the open-case and watch strip on K1 "phase 18 adds it". Open cases come from
  `GET /parties/{id}` step 5. From phase 19 on, watches come from `GET /watches` with a `party_id` filter (phase 19
  section 7+).
- G1-17-01 UX-12, MEDIUM: the DESIGN-SYSTEM 7.18 shortcut layer (phase 08 says it is P1). It uses phase 01's
  preferences shape (`shortcuts: { enabled, modifier }`); no route.
- G1-17-02 UX-16, MEDIUM: the ROADMAP P1 quality gate. A release-pipeline load run with 50 concurrent sessions on
  synthetic data (SP-06), with budgets for the queue items, case header and next routes, and search from phase 18 on.
- G1-08-01 UX-03, LOW: ROADMAP track A ("Usability round 1 ... (UX-03)"). A process step whose record settles DS-O4.
  The draft's merge gate is dropped.
- G1-08-02 UX-39, MEDIUM: the ROADMAP P1 accessibility pack. Per-release keyboard walkthroughs of the six core tasks,
  400 percent zoom in Arabic, and NVDA and VoiceOver passes, recorded with the conformance statement (DS-O5). The axe
  CI run is phase 08's already.
- G1-24-01 UX-31, MEDIUM (DR-400).
  - In scope: the DESIGN-SYSTEM 8.1 REGISTER pattern ("an export (SU job)") on the P1 register screens. No phase had an
    export route.
  - Rewritten: register key = the screen ID. The job calls that screen's own list service as the caller, so scope,
    Cedar, visibility and masking are the list route's.
  - No new table: the export id is the job id, and the file is an evidence object of kind `render` with owner kind
    `register_export`.
  - Download uses step-up class `export` and the access event `export`. Per-user watermarking is P2 (DR-396, DR-172).
  - UX-31 is Partly.

## Classification decisions
- CARRIED accepted (11): UX-05 (17), UX-10, UX-61 (19), UX-40 (25), UX-47, UX-56 (24), UX-58, W-09 (14), UX-62 (21),
  UX-69 (20), W-04 (16).
  - W-04: the shadow minimum is the bank-set `scenario.shadow_min_days` of S-47 and DSL A.10, not a long parallel run.
    This is no conflict.
- Moved from CARRIED to Partly:
  - UX-70 (19, the QA workspace; the capacity workbench is P2 on phase 31).
  - W-03 (19, the network edges; entity merge is P2, DR-394).
- Implemented from the accepted gaps: UX-29, UX-07 (18), UX-59 (15, the strip from phase 18 on), UX-12, UX-16 (17),
  UX-03, UX-39 (08). UX-31 is Partly (24).
- Partly accepted as drafted: UX-09, UX-17, UX-57 (18); UX-11, UX-37, UX-42 (17); UX-43, W-07, W-08, W-10, W-18 (14);
  UX-48, W-01, W-20 (24); UX-51 (21; the watermark is P2, DR-396); UX-71 (08); W-15, W-21, W-24 (15); W-23 (19).
- W-14 moved from Not carried to Partly in phase 10 (the canonical trade-instrument table); its rules, profile, outliers
  and vessel screening are P2 (DR-400).
- NOT-CARRIED accepted (12): UX-04 (17), UX-46, W-05 (28), UX-52 (08), W-02, W-11 (27), W-12, W-13 (10), W-16, W-17
  (15; W-16 has no contracted media source, a ROADMAP external item), W-22 (31), W-25 (16). DR-400.

## Other edits
- Phase 24 Open doubts 15 said the capacity workbench is phase 28. It is phase 31 (G2).

DR: DR-400 (the run 7a additions and the items outside this release).
