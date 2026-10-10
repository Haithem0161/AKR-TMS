# Gap pass 1, run 7a (coverage: UX and W; Haiku draft for lead review)

## Part 1. Classification

| ID | Research row (file:line) | Requirement (at most 15 words) | Class | Evidence (phase-NN.md:LINE "quote" or source) | Proposed action |
|-|-|-|-|-|-|
| UX-03 | research/31-analyst-ux.md:903 | Usability rounds with Iraqi compliance staff, Arabic first, before and during build | GAP | ROADMAP.md:133 (P0 track A7: "Usability round 1 ... (UX-03)"); DESIGN-SYSTEM.md:974 (DS-O4 "First usability round with Iraqi staff (UX-03)"); no phase text names it | Finding G1-08-01 (LOW) |
| UX-04 | research/31-analyst-ux.md:904 | Instrument analyst workspace (panel time, switches, back navigations) for pilot; no league tables | NOT-CARRIED | grep "panel switch|time in panel|dwell" over docs/plan: none; no MODULES or ROADMAP feature line; pilot measurement sits in the parallel run (ROADMAP.md:160, bank's process) | add UX-04 to phase 17 header, line Not carried (pilot measurement is the bank's parallel run; no P0 or P1 line) |
| UX-05 | research/31-analyst-ux.md:905 | Merged alerts shown as one case row with merged count; any underlying alert openable | CARRIED | phase-17.md:1623 "merged-alert count" (section 5.2, W2); phase-17.md:296 `merged_into`; phase-18.md:1156 "merged-alert count" (W3 header) | add UX-05 to phase 17 header, line Implemented here |
| UX-07 | research/31-analyst-ux.md:907 | Entity timeline across alerts, windows, RFIs, decisions, tasks, QA; filterable; Hijri bands | GAP | phase-18.md:1164 W4 tabs are Profile, Accounts, Rating, Cases and alerts, Transactions (no timeline); API.md:533-535 has no /parties/{id}/timeline; API.md:488 only /cases/{id}/timeline; no Hijri band in any timeline | Finding G1-18-02 (MEDIUM) |
| UX-09 | research/31-analyst-ux.md:909 | Cognitive-forcing switches per priority class; evidence before decision; acceptance and override rates monitored | Partly | phase-18.md:597 `POST /cases/{id}/opened-views` (evidence-opened gate, DR-136); missing: per-priority-class switch and rate monitoring (phase-27.md:718 override rate is for models, P2) | add UX-09 to phase 18 header, line Partly implemented (evidence-opened gate built; per-class switch and rate monitoring missing) |
| UX-10 | research/31-analyst-ux.md:910 | Network view auto-groups above about 15 accounts; subject never grouped away; node and link table | CARRIED | phase-19.md:1732 "automatic grouping above about 15 accounts ... the subject is never grouped away"; phase-19.md:1734 "A node and link table is the equal alternative" | add UX-10 to phase 19 header, line Implemented here |
| UX-11 | research/31-analyst-ux.md:911 | Queue-run mode and one-key jump to oldest unassigned item; leaving never discards a draft | Partly | phase-17.md:1625 "Next" (key `n`) claims the first unclaimed visible item; phase-17.md:1623 order is class, score, age; phase-08.md:319-323 same-tab drafts; missing: oldest-first pick and run mode | add UX-11 to phase 17 header, line Partly implemented (oldest-first pick and run mode missing) |
| UX-12 | research/31-analyst-ux.md:912 | Shortcut layer on physical key codes, never in text fields, remappable, switchable off; cheat sheet on ? | GAP | DESIGN-SYSTEM.md:643-647 (7.18 Shortcuts, IR-91, IR-92, UX-12); phase-08.md:353-354 "The shortcut layer (7.18) is P1; P0 registers no single-key shortcuts"; phase-08.md:666 defers it; phase-17.md:1625 uses key `n` with no layer; no phase builds 7.18 | Finding G1-17-01 (MEDIUM) |
| UX-16 | research/31-analyst-ux.md:916 | Response budgets (first content 1 s at 50 analysts; type-ahead p95 300 ms) tested in CI | GAP | ROADMAP.md:232 (P1 quality gate "Response budgets under 50 concurrent analysts ... (UX-16)"); API.md:75 cites the budgets; no phase builds the 50-analyst test (grep "50 concurrent|synthetic data set": phase-02 chain test only); phase-04.md:1179 has a 300 ms inbox budget only | Finding G1-17-02 (MEDIUM) |
| UX-17 | research/31-analyst-ux.md:917 | Draft autosave, restore after time-out or step-up, one active editor, presence visible to access list | Partly | phase-18.md:1185 "for other cases the unsent text may be kept in the tab only"; phase-08.md:319-323 same-tab draft map survives a time-out; missing: presence (one active editor, grep "presence" empty), restore after step-up, autosave | add UX-17 to phase 18 header, line Partly implemented (presence and autosave missing) |
| UX-29 | research/31-analyst-ux.md:929 | Search detects input kind (IBAN, card, phone, case no., name), extended fold, mod-97 IBAN check, suspicion excluded for non-members | GAP | API.md has no global search route (grep "search": only /screening/search, phase-14, and /cases, phase-17.md:847 with no `q`); phase-17.md:1490 "the q parameter is not accepted (400)"; phase-17.md:1215 visibility predicate reusable | Finding G1-18-01 (HIGH; covers X2) |
| UX-31 | research/31-analyst-ux.md:931 | Exports: PDF via Gotenberg; XLSX with RTL sheet; CSV UTF-8 with BOM, ISO dates, formula-injection guard; audit entry | GAP | Gotenberg PDF is built (phase-06 gap report; phase-24.md:569 pack download); DESIGN-SYSTEM.md:8.1 REGISTER pattern requires "an export (SU job)" but no P1 phase builds a register export (grep "xlsx|csv" in phases 15-24: none); phase-24.md:599 "Export is not in this phase" (audit viewer only) | Finding G1-24-01 (MEDIUM); watermark conflict in Summary |
| UX-37 | research/31-analyst-ux.md:937 | Live updates announced in aggregate; pause control; lists never reorder under the cursor | Partly | phase-17.md:1632 "Pause live updates" stops the stream; phase-17.md:1632-1633 banner "Live updates paused" and "Refreshed every minute"; no aggregate-announcement or no-reorder rule in phase-08.md or phase-17.md | add UX-37 to phase 17 header, line Partly implemented (aggregate announcement and no-reorder rule missing) |
| UX-39 | research/31-analyst-ux.md:939 | Keyboard walkthroughs of six tasks, 400 percent zoom in Arabic, NVDA and VoiceOver passes | GAP | DESIGN-SYSTEM.md:908-909 (six core tasks per release; 400 percent zoom; NVDA and VoiceOver); ROADMAP.md:230-232 P1 accessibility pack; phase-08.md:601 and 628 cover P0 only; phase-17.md:1651 and phase-18.md:1197 per-screen keyboard checks only; no zoom or screen-reader pass in any phase | Finding G1-08-02 (MEDIUM) |
| UX-40 | research/31-analyst-ux.md:940 | Contextual help, bilingual compliance glossary, reason-code explanations, what-changed notices, one help location, no chatbot | CARRIED | phase-25.md:215 `admin.help_articles.kind` in article, glossary, what_changed; phase-25.md:1283 X5 tabs "Help", "Glossary", "What changed"; phase-08.md:349 `Help` popover from the offline glossary | add UX-40 to phase 25 header, line Implemented here |
| UX-42 | research/31-analyst-ux.md:942 | Explanation layers L0 to L3; three headline reasons beside facts; mitigating factors; disclosure by role; stored explanation | Partly | phase-17.md:1628 "at most three headline reasons each beside its fact", mitigating factors (same bullet); missing: layers L0 to L3, disclosure by role, stored explanation (grep "explanation" in phase-17.md: none); model contributions are P2 (MODULES.md:529 area, phase-27) | add UX-42 to phase 17 header, line Partly implemented (layers, role disclosure and stored explanation missing) |
| UX-43 | research/31-analyst-ux.md:943 | Screening hit screen: verdict sentence, side-by-side records, deciding identifiers, per-part table, frequency sentence, pair memory, cross-script note | Partly | phase-14.md:1256 S1 (verdict sentence, identifiers, per-part table with folds, frequency sentence, pair memory on the incorrect tile); missing: cross-script note (grep "cross-script" in docs/plan: none) | add UX-43 to phase 14 header, line Partly implemented (cross-script note missing) |
| UX-46 | research/31-analyst-ux.md:946 | Explanation usability protocol per release in Arabic with Iraqi analysts | NOT-CARRIED | ROADMAP.md:288 (P2 assistant "passes its evaluation gate in Arabic ... (UX-46)"); ROADMAP.md:255 starts P2 | add UX-46 to phase 28 header, line Not carried (P2 assistant evaluation gate, ROADMAP.md:288) |
| UX-47 | research/31-analyst-ux.md:947 | Dashboards: attention, then period, then detail; counts open lists; every figure has definition and data-as-of | CARRIED | phase-24.md:1187 (G1 order; "Each tile has a `?` help with its definition and formula, the `data_as_of` line and a link to its list") | add UX-47 to phase 24 header, line Implemented here |
| UX-48 | research/31-analyst-ux.md:948 | Command centre additions: decisions waiting, overdue clocks, arrivals vs throughput and capacity ratio, verification, QA, data incidents, tipping-off summary, productive scenario table | Partly | phase-24.md:1187 (decisions, clocks, verification, incidents, QA, tipping-off summary); phase-24.md:721 "Arrivals against throughput"; phase-24.md:723 "Productive alerts"; missing: capacity ratio, excluded by phase-24.md:1378 as P2 (that note names phase 28; the workbench is phase 31) | add UX-48 to phase 24 header, line Partly implemented (all tiles except the P2 capacity ratio) |
| UX-51 | research/31-analyst-ux.md:951 | Built-in regulatory and periodic reports with scheduling, watermark, Arabic and English renderings | Partly | ROADMAP.md:208 (R3 scheduling and watermarks); phase-21.md:532-536 scheduled jobs (`schedule.ctr_daily`, `schedule.report_clock_scan`); phase-21.md:891 and 1378 (DR-172, Open doubts 14): no per-user watermark; missing: watermark (conflict, see Summary) | add UX-51 to phase 21 header, line Partly implemented (watermark conflict open) |
| UX-52 | research/31-analyst-ux.md:952 | Competitive usability benchmark on a vendor demo; concurrent-user performance tests | NOT-CARRIED | no MODULES or ROADMAP feature line; vendor evaluation is research (research 31 section 9); the concurrency part is UX-16 (Finding G1-17-02) | add UX-52 to phase 08 header, line Not carried (vendor evaluation research, not a release feature) |
| UX-56 | research/31-analyst-ux.md:956 | Mockup 01 command centre: ageing, labelled stacked bars, defined productive, UTRs in STR tile, clickable tiles | CARRIED | phase-24.md:1187 (tiles link to their lists); phase-24.md:723 "Productive alerts" defined; phase-08.md:476 stacked bar with direct labels (chart spike) | add UX-56 to phase 24 header, line Implemented here |
| UX-57 | research/31-analyst-ux.md:957 | Mockup 02 case workspace: actions apart from decisions, coded reasons, seven-criterion checklist, four-eyes line, sentence-level sources, filterable timeline | Partly | phase-18.md:1156 header strip (class, clocks, tier, merged count); phase-18.md:1160 CBI escalation checklist (IR-32); phase-18.md:1161 four-eyes at point of action; missing: filterable timeline (phase-17.md:849 `GET /cases/{id}/timeline` takes no filter); sentence-level sources belong to the P2 assistant (MODULES.md:529) | add UX-57 to phase 18 header, line Partly implemented (timeline filter missing) |
| UX-58 | research/31-analyst-ux.md:958 | Mockup 03 screening hit: verdict sentence, deciding identifiers, Office states with consequences, pair memory on incorrect only, unticked | CARRIED | phase-14.md:1256 S1 ("A verdict sentence ... not a score ring"; Office tiles with consequence lines; "Remember this decision for this pair" only on the incorrect tile, unticked) | add UX-58 to phase 14 header, line Implemented here |
| UX-59 | research/31-analyst-ux.md:959 | Mockup 04 risk profile: missing-input share, refresh dates, rating-change reasons, open-cases strip | GAP | phase-15.md:1885-1888 K1 has missing-input share, refresh dates and rating-change reasons; phase-15.md:1887-1888 "the open-case and watch strip is hidden in this phase (phase 18 adds it within the user's access)"; phase-18.md has no such strip (header strip at 1156; W4 tabs at 1164) | Finding G1-18-03 (LOW) |
| UX-61 | research/31-analyst-ux.md:961 | Mockup 06 network: masked account numbers and Arabic names, grouping control, table alternative, keyboard traversal | CARRIED | phase-19.md:1732 grouping control; phase-19.md:1734 node and link table and keyboard traversal; phase-19.md:1737 masked account form and name toggle | add UX-61 to phase 19 header, line Implemented here |
| UX-62 | research/31-analyst-ux.md:962 | Mockup 07 report builder: type selector, origin of suspicion, seventh "Filed and acknowledged" step, filing reference, tipping-off check, decision-to-filing clock | CARRIED | phase-21.md:1277 (six steps; origin of suspicion; tipping-off phrase check; step 7 "Filed and acknowledged"; "Record filing reference"; decision-to-filing clock); sentence-level sources are P2 (MODULES.md:529) | add UX-62 to phase 21 header, line Implemented here |
| UX-69 | research/31-analyst-ux.md:969 | Minimal operations-task inbox showing only the neutral instruction and reference | CARRIED | phase-20.md:989 W9 OPS INBOX ("the neutral text ... No case, risk, reason, party name, list, or verification level is shown") | add UX-69 to phase 20 header, line Implemented here |
| UX-70 | research/31-analyst-ux.md:970 | Capacity workbench and QA workspace screens (file 28 sections 10 and 11) | CARRIED | QA workspace: phase-19.md:1760 section 5.5 G3 (eight criteria, rates, calibration, bulk reopen); capacity workbench G2 is P2: phase-31.md:928 section 5.2 (flag `pack.capacity`), ROADMAP.md:278 | add UX-70 to phase 19 header, line Implemented here (QA half; capacity half is P2 on phase 31) |
| UX-71 | research/31-analyst-ux.md:971 | Vendor lessons: fast kind-aware search; role-based simplicity; first-class reporting; plain Arabic words; in-case filing | Partly | phase-08.md:260-261 role-based rail and X6 for roles without a route; phase-21.md:1277 report built from the case; DESIGN-SYSTEM.md section 11 wording; missing: fast kind-aware search (Finding G1-18-01) | add UX-71 to phase 08 header, line Partly implemented (kind-aware search missing; see G1-18-01) |
| W-01 | research/04-basel-wolfsberg-egmont-un.md:1558 | Effectiveness metrics per scenario (alerts, escalations, STRs, FIU feedback), false-negative analysis, STR completeness score | Partly | phase-24.md:723 "Productive alerts" tile; phase-21.md:923 FIU feedback register (3.12); phase-24.md:13 S-53 partly (fairness only; productivity in Open doubts 11); phase-16.md:2131 targets S-53 to phase 24; missing: per-scenario escalations, misses (false-negative analysis), STR completeness score | add W-01 to phase 24 header, line Partly implemented (S-53 remainder is Open doubts 11) |
| W-02 | research/04-basel-wolfsberg-egmont-un.md:1559 | Hybrid detection: CBI rules, supervised and unsupervised scoring with reason codes; typology-to-feature coverage inventory | NOT-CARRIED | ROADMAP.md:269-272 (P2 models: ONNX, ranking rule alerts only); MODULES.md:597 (M15 P2 model registry); typology-to-feature inventory is named by no P0 or P1 line (phase-27.md:946 notes the mapping is absent) | add W-02 to phase 27 header, line Not carried (P2 models; ROADMAP.md:269) |
| W-03 | research/04-basel-wolfsberg-egmont-un.md:1560 | Entity resolution and network graph (related parties, BOs, counterparties, shared phone, address, family number); dynamic segmentation | CARRIED | phase-19.md:1206 `shared_contact` edges on phone and address (`value_hk`); phase-19.md:1009 family links; phase-13 segments; DR-398 counterparty links; device links are P2 (DR-394, S-20) | add W-03 to phase 19 header, line Implemented here (device links P2) |
| W-04 | research/04-basel-wolfsberg-egmont-un.md:1561 | Sandbox with below-the-line testing and champion-challenger; governed scenario lifecycle; no mandatory long parallel runs | CARRIED | phase-16.md:25 S-44 champion and challenger (shadow beside live version); phase-16.md:27 S-47 go-live gate; ROADMAP.md:193 (D2 below-the-line sample); conflict on "no mandatory long parallel runs" vs the 30-day shadow (phase-16.md:573, see Summary) | add W-04 to phase 16 header, line Implemented here |
| W-05 | research/04-basel-wolfsberg-egmont-un.md:1562 | LLMs only as assistants (summaries, narratives, Arabic media triage) with human review | NOT-CARRIED | ROADMAP.md:274 (P2 "Assistant for case summaries and report narratives"); MODULES.md:529 (P2) | add W-05 to phase 28 header, line Not carried (P2 assistant, ROADMAP.md:274) |
| W-07 | research/04-basel-wolfsberg-egmont-un.md:1564 | Screening configuration as governed, versioned object (matching mode, thresholds, exclusions, good-guy list), maker-checker, exportable documentation | Partly | phase-14.md:10-13 versioned lists with thresholds (`ref.lists`); phase-14.md:403 good-guy rules under change request (S4 "change request link"); missing: exportable documented configuration (no export route in phase 14) | add W-07 to phase 14 header, line Partly implemented (exportable configuration missing) |
| W-08 | research/04-basel-wolfsberg-egmont-un.md:1565 | Screen all relevant parties: customers, BOs, signatories, employees, suppliers, payment counterparties, free-text fields, vessels | Partly | phase-14.md:3 (goal: customers, counterparties and payment parties); grep "signator|employee|supplier|free.text" in phase-14.md: none; vessels are trade scope (see W-14) | add W-08 to phase 14 header, line Partly implemented (signatories, employees, suppliers, free-text fields missing) |
| W-09 | research/04-basel-wolfsberg-egmont-un.md:1566 | Weak-alias handling, Arabic and patronymic name intelligence, side-by-side alert screen with distinguishing attributes | CARRIED | phase-14.md:951 `alias_class` from UN and OFAC weak-alias criteria (N-27); phase-14.md:973 patronymic alignment penalty (N-25); phase-14.md:1256 S1 side by side | add W-09 to phase 14 header, line Implemented here |
| W-10 | research/04-basel-wolfsberg-egmont-un.md:1567 | Screening test harness with dated reports; lookback over the CBS replica; list diff with delta re-screen; list-update latency KPI | Partly | phase-14.md:1053 known-answer corpus with dated runs; phase-14.md:1055 look-back `POST /screening/lookbacks`; phase-14.md:1258 S3 diff by counts; MODULES.md:345 designation look-back on every list change; missing: delta re-screen (grep "re-screen" in phase-14.md: none beyond onboarding) and list-update latency KPI (phase-14.md:1290 measures match latency only) | add W-10 to phase 14 header, line Partly implemented (delta re-screen and list-update KPI missing) |
| W-11 | research/04-basel-wolfsberg-egmont-un.md:1568 | Model governance pack: model cards, validation, delta validation, drift and bias dashboards, purpose limits | NOT-CARRIED | ROADMAP.md:269 (P2 "Model registry and AI governance (D4)"); MODULES.md:597 (M15 models P2) | add W-11 to phase 27 header, line Not carried (P2 models, ROADMAP.md:269) |
| W-12 | research/04-basel-wolfsberg-egmont-un.md:1569 | Payment-quality monitor: missing or meaningless fields, name consistency with CDD name, structured addresses, over a canonical payment model | NOT-CARRIED | no MODULES or ROADMAP line names a payment-quality monitor; the P1 quality queue covers CBS tables (MODULES.md:292-296); payment match rules and payment links are P2 (DR-394; phase-10.md:34 C-43 partly) | add W-12 to phase 10 header, line Not carried (no P0 or P1 line; payment rules P2, DR-394) |
| W-13 | research/04-basel-wolfsberg-egmont-un.md:1570 | Detect bundled and on-behalf-of payments and domestic legs of cross-border payments | NOT-CARRIED | DR-394 (payment match rules and reconciliation rungs L2, L4, L5, L6, C-43 part: P2); phase-10.md:34 "match rules and rung L4 are P2, DR-394" | add W-13 to phase 10 header, line Not carried (payment match rules are P2, DR-394) |
| W-14 | research/04-basel-wolfsberg-egmont-un.md:1571 | Trade finance module: LC, LG and collection rules, declared-vs-actual trade profile, unit-price outliers, trade party and vessel screening | NOT-CARRIED | grep "trade" in MODULES.md and ROADMAP.md: no feature line names a trade module; vessel screening named by no P0 or P1 line; CBI TRADE scenarios (research/27-transaction-monitoring-scenarios.md:313) belong to the S- IDs of another run | add W-14 to phase 16 header, line Not carried (no P0 or P1 line names trade finance) |
| W-15 | research/04-basel-wolfsberg-egmont-un.md:1572 | Source of wealth and funds record with corroboration; actual-vs-declared monitoring; third-party funding detector; cash-intensity factor | Partly | phase-15.md:341 evidence kinds `source_of_funds`, `source_of_wealth`; phase-15.md:1179 B3 source of wealth checklist; phase-13.md:404 `GET /parties/{id}/profiles` (actual and declared figures); missing: third-party funding detector and cash-intensity factor (grep "cash.intensity" empty) | add W-15 to phase 15 header, line Partly implemented (third-party funding and cash intensity missing) |
| W-16 | research/04-basel-wolfsberg-egmont-un.md:1573 | Adverse-media module: Arabic and Kurdish sources, Iraq categories, credibility, snapshots, DOB tolerance, media hit as trigger | NOT-CARRIED | phase-18.md:1438 "Adverse-media panel. No source exists"; ROADMAP.md:361 external item (third-party risk data, "P1 PEP and adverse-media depth", Open); phase-15.md:1175 A5 "adverse media (the media feed, later phase)" | add W-16 to phase 15 header, line Not carried (media source not contracted; ROADMAP.md:361) |
| W-17 | research/04-basel-wolfsberg-egmont-un.md:1574 | Dynamic enterprise-wide risk assessment; risk appetite register; control rationalisation; profile-amendment monitoring; financial-inclusion lane | NOT-CARRIED | grep "enterprise|appetite" in MODULES.md, ROADMAP.md and phases: none; no P0 or P1 line names it; profile-change triggers are built (phase-19.md:261 `trigger_key` profile_change) | add W-17 to phase 15 header, line Not carried (no P0 or P1 line names an enterprise-wide assessment) |
| W-18 | research/04-basel-wolfsberg-egmont-un.md:1575 | Iraq TF typology pack, NPO segment, terrorist-list matches bypass the normal queue | Partly | phase-14.md:997 `screening.priority_map` gives `freeze_identifier` P0; research/27-transaction-monitoring-scenarios.md:313 TF family (7 scenarios) in the register; NPO scenarios P2 (DR-394, S-33); missing: bypass route for terrorist-list matches | add W-18 to phase 14 header, line Partly implemented (bypass route missing; NPO P2) |
| W-20 | research/04-basel-wolfsberg-egmont-un.md:1577 | Regulatory obligations register; control change log; STR usefulness indicators; audit workspace | Partly | phase-24.md:1199 G6 audit viewer (section 5.4); phase-24.md:1105 control report counts; phase-24.md:723 productive alerts; change log is the phase 04 change request history; missing: obligations register (grep "obligation" in phase-24.md: none) | add W-20 to phase 24 header, line Partly implemented (obligations register missing) |
| W-21 | research/04-basel-wolfsberg-egmont-un.md:1578 | Iraqi PEP register by position tier; relative and close-associate detection with confirmation; lifecycle incl. declassification; pair-keyed memory; PEP dashboard | Partly | phase-15.md:384 `position` on the PEP register; phase-15.md:413 relatives confirmed by a person; phase-15.md:830 `former`, `persistence_until`; phase-15.md:882 screening pair memory; missing: PEP dashboard (no PEP tile or route) | add W-21 to phase 15 header, line Partly implemented (PEP dashboard missing) |
| W-22 | research/04-basel-wolfsberg-egmont-un.md:1579 | Correspondent and nested-relationship module; respondent self-view | NOT-CARRIED | ROADMAP.md:278 (P2 "correspondent and validator packs"); ROADMAP.md:280 (P2 "RFIs to correspondents (W8)"); MODULES.md:655 (P2) | add W-22 to phase 31 header, line Not carried (P2 correspondent packs and RFIs, ROADMAP.md:278 and 280) |
| W-23 | research/04-basel-wolfsberg-egmont-un.md:1580 | RFI workspace: incoming RFIs with pre-filled answers, deadline clock, outreach tasks, tipping-off guard; outgoing RFIs with templates and payment restrictions; RFI KPIs | Partly | phase-19.md:1742 desk with deadline, overdue marker and `kpis` strip (incl. "missed by monitoring"); phase-19.md:1745 composer with six kinds and template preview; phase-19.md:1744 release four-eyes; missing: pre-filled draft answers, outreach task, payment restriction on outgoing RFIs | add W-23 to phase 19 header, line Partly implemented (pre-filled answers, outreach and payment restrictions missing) |
| W-24 | research/04-basel-wolfsberg-egmont-un.md:1581 | KYC refresh trigger engine, targeted refresh tasks, trigger effectiveness metrics, onboarding assurance level, optional digital-channel link | Partly | phase-15.md:12 TR-01 to TR-26 trigger library wired; phase-15.md:281 `due_basis` includes a KYC refresh clock; missing: trigger effectiveness metrics, onboarding assurance level (grep "assurance level" empty), digital-channel link (P2, DR-394) | add W-24 to phase 15 header, line Partly implemented (effectiveness metrics and assurance level missing) |
| W-25 | research/04-basel-wolfsberg-egmont-un.md:1582 | PSP and MSB segment: licence and flow-type records, activity review, PSP scenarios, questionnaire, unbundling, opaque-flow reporting | NOT-CARRIED | grep "PSP|MSB|money service" in MODULES.md, ROADMAP.md and phases: none; no P0 or P1 line names a PSP segment | add W-25 to phase 16 header, line Not carried (no P0 or P1 line names a PSP segment) |

## Part 2. Findings (GAP rows)

Eight GAP IDs, eight findings. UX-29 and the required item X2 (global search and command palette) share G1-18-01.

### G1-18-01 Global search route with kind detection (X2; UX-29, UX-71)
- Severity: HIGH
- Category: Missing Endpoint
- Requirement: X2 (DESIGN-SYSTEM.md:682, P1; MODULES.md:501 lists X2 in the M12 P1 screens); UX-29 (research/31-analyst-ux.md:929); UX-71 (research/31-analyst-ux.md:971); SP-27 (research/30-security-privacy.md:1400); RP-61 (research/23-regulatory-reporting-goaml.md:811)
- Target: phase-18.md section 7.12
- Check: grep -n "global search|command palette|X2" docs/plan/*.md docs/API.md: phase-08.md:283 and :665 defer X2 to P1 and no P1 phase owns it; grep -n "search" docs/API.md: no global route (only /screening/search, phase 14, and GET /cases "Case search", API.md:486); phase-17.md:847 and :1490 GET /cases takes no q (400); phase-17.md:1215 and :1231 VisibilityPredicate is "the only list filter for alerts, cases, queue items, counts and bulk"; phase-18.md:799 section 3.10 party visibility; phase-02.md:731 scope.record_hits; phase-02.md:150 access verbs include search_hit; phase-10.md:1206 canon.card.pan_hk; phase-13.md:1115 canon.account number_hk and iban_hk; phase-19.md:1206 value_hk on canon.party_contact; phase-21.md:892 RP-61 exclusion stated in the download section only; phase-14 name search (screening.work) stays separate.
- Proposed 7.12 text:

"### 7.12 Global search (X2; UX-29, UX-71; SP-27, RP-61)
One route for the header search and the command palette. It searches cases, alerts and parties only. It never returns screening alerts or pair memory (phase 14 owns that search, screening.work), never report rows (RP-61) and never a suspicion-tier row the caller may not see (SP-27).

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /api/v1/search | queue.work | Global search. `q` 2 to 80 characters; `X-Access-Purpose` required (422 `E_PURPOSE_REQUIRED`; purposes `assigned_work` or `investigation`); `limit` at most 10 per group; response `{detected_kind, groups[{kind, items[], has_more}]}`; no totals; a masked row and a missing row give the same bytes (API.md section 6) |

`SearchService::query(ctx, q, purpose, limit)` (new in phase 18):
1. `SearchKind::detect(q)` (pure): `iban` when the value starts with IQ, has 23 characters and passes the mod-97 check; `card` when it has 13 to 19 digits and passes Luhn; `phone` when it normalises as phase 10 normalises `value_hk`; `case_ref` when it matches the reference format of `casework.cases.case_ref`; otherwise `name`. A kind whose check fails falls back to `name`.
2. `name`: the fold and key of `crates/names` (DR-371) against the canonical name keys of the party.
3. `iban`: keyed hash `iban_hk` (or `number_hk`) of `canon.account`; `card`: `pan_hk` of `canon.card`; `phone`: `value_hk` of `canon.party_contact`. Only keyed hashes are compared; the query text is never logged.
4. Cases, alerts: the rows that `services::visibility::build(principal, network)` (phase 17 section 3.10) admits, the same predicate the queues use; Cedar `case.read` and `alert.read`.
5. Parties: the phase 18 section 3.10 visibility, Cedar `party.read`. A suspicion-tier party outside the case access list is absent, not masked, so a count of matches cannot reveal it.
6. Reports and screening alerts are not queried.
7. For each restricted or suspicion-tier row returned: `scope.record_hits(rows)` (phase 02). One access event for the search with action `search_hit` (phase 02 list), resource type `global_search` (one new value in the phase 02 resource list) and resource_id = a fresh UUIDv7 event id, as phase 14 section 7.9 does.
8. Route tests: each kind through each role of the visibility matrix; the masked and missing bodies are byte-identical; a card-like query returns no row unless `pan_hk` matches.
9. Frontend (phase 18 section 5 addition): an overlay opened from the header and by the `/` key (phase 17 section 7.6 layer); groups by kind with the Arabic name first; the X2 entry joins the full shell form of phase 08 section 3.6 without a change to any screen.
10. API.md: one row in section 3.12 for GET /search, owned by phase 18."

### G1-18-02 Party entity timeline (UX-07)
- Severity: MEDIUM
- Category: Missing Endpoint
- Requirement: UX-07 (research/31-analyst-ux.md:907): entity timeline of alerts, windows, observations, RFIs, authority requests, rating changes, decisions, tasks and QA events; filter by type; Hijri bands
- Target: phase-18.md section 7.13
- Check: phase-18.md:1164-1168 W4 tabs (Profile, Accounts, Rating, Cases and alerts, Transactions): no timeline tab; API.md:533-535 has /parties/{id}, /transactions, /graph only; API.md:488 has only /cases/{id}/timeline; DESIGN-SYSTEM.md:611 the timeline component exists (7.14, "Timeline and notes"); phase-09.md:128 `hijri_label` on calendar days.
- Proposed 7.13 text:

"### 7.13 Party entity timeline (UX-07; W4 tab "Timeline")

| Method | Path | Scope | Description |
|---|---|---|---|
| GET | /api/v1/parties/{id}/timeline | queue.work | Events of the party, newest first; `kinds` filter (a list of the kinds in step 2); keyset cursor on (occurred_at, id); each row carries the `hijri_label` of its day (phase 09); 404 `E_NOT_FOUND` for a masked party, byte-identical to a missing one |

`PartyTimelineService::list(ctx, party_id, kinds, cursor)` (new in phase 18):
1. Party visibility (phase 18 section 3.10); a suspicion-tier party outside the case list answers 404.
2. Each source admits its rows through its own predicate: alerts through `services::visibility::build` (phase 17 section 3.10); RFIs and authority requests through the phase 19 visibility; rating changes through `rating.read`; decisions through the visibility of their case; operations tasks through the phase 20 visibility; QA samples to QA and M only (phase 19 section 5.5).
3. Merge the sources by `occurred_at` descending; no row without a visible source.
4. Each row: `kind`, `occurred_at`, `actor_role` (a role word, never a person's name for a restricted party), one summary sentence, a link to its screen, `hijri_label`.
5. A restricted or suspicion-tier page writes one access event `view` (resource type `party`) per page (phase 02).
6. Frontend: a Timeline tab on W4 with a filter bar by kind; the table alternative of DESIGN-SYSTEM 7.14 is the default.
7. API.md: one row in the party section (after GET /parties/{id}/graph, API.md:535), owned by phase 18."

### G1-18-03 Open-case and watch strip on K1 (UX-59)
- Severity: LOW
- Category: Missing Logic
- Requirement: UX-59 (research/31-analyst-ux.md:959): mockup 04 open-cases strip; phase 15 defers it to phase 18
- Target: phase-18.md section 7.14
- Check: phase-15.md:1887-1888 "the open-case and watch strip is hidden in this phase (phase 18 adds it within the user's access)"; phase-18.md has no such strip (header strip at :1156 is the case workspace only; W4 tabs at :1164); API.md:532 GET /watches exists without a party filter.
- Proposed 7.14 text:

"### 7.14 Open-case and watch strip on K1 (UX-59; closes the deferral of phase 15 section 5.1)
K1 (`/parties/:id/risk`) shows one strip above the drivers with two groups: the open cases of the party that the caller may see (reference, class word, status; from `GET /parties/{id}`, phase 18 section 3.14) and the UTR watches of the party (from `GET /watches`, phase 19). Add a `party_id` filter to `GET /watches` (API.md:532). The same visibility as W4 applies; a case the caller may not see is absent, with no count. Hooks: `useGetPartiesId`, `useGetWatches`. Empty state: "No open cases or watches for this customer"."

### G1-17-01 Shortcut layer for the queues (UX-12)
- Severity: MEDIUM
- Category: Missing Logic
- Requirement: UX-12 (research/31-analyst-ux.md:912); DESIGN-SYSTEM.md:643-647 (7.18: physical key codes, Latin keycap labels, never in text fields, switch off and remap with Ctrl or Alt, cheat sheet on ?)
- Target: phase-17.md section 7.6
- Check: phase-08.md:353-354 "The shortcut layer (7.18) is P1; P0 registers no single-key shortcuts"; phase-08.md:666 defers it; phase-17.md:1625 "Next" with key n and no layer; phase-01.md:428 PreferencesV1 has a `shortcuts` object; grep "cheat|KeyboardEvent|keycap" in docs/plan: none outside 7.18.
- Proposed 7.6 text:

"### 7.6 Shortcut layer for the queues (UX-12; DESIGN-SYSTEM 7.18)
Builds the shell component of DESIGN-SYSTEM 7.18 in `src/components/shortcuts/` (on the phase 08 layer). Used by W1, W2 and W3.
1. Bindings match on `KeyboardEvent.code`, never `key`; the Next binding of section 5.2 (`KeyN`) is the first.
2. Each binding shows its Latin keycap in the label, so the same keys work on an Arabic layout.
3. Inactive while the focus is in an input, textarea, select or contenteditable element; Escape closes overlays only.
4. `?` opens the cheat sheet: a dialog listing the bindings of the current screen.
5. The `shortcuts` object of PreferencesV1 (phase 01, `PATCH /me/preferences`) holds the on-off switch and the remaps; a remapped key needs Ctrl or Alt. The switch sits in the X4 profile (phase 08 section 3.6). No route and no audit event are added; the preference write is already audited by phase 01.
6. Tests: a key in a text field does nothing; a remap survives a reload; the switch off disables every binding; each binding has a keyboard case in the W1, W2 and W3 browser checks (phase 17 section 6)."

### G1-17-02 Response budgets for the queues (UX-16)
- Severity: MEDIUM
- Category: Missing Requirement
- Requirement: UX-16 (research/31-analyst-ux.md:916): first content within 1 s and type-ahead p95 300 ms at 50 concurrent analysts, tested in CI
- Target: phase-17.md section 7.7
- Check: ROADMAP.md:232 (P1 quality gate "Response budgets under 50 concurrent analysts on a synthetic data set (UX-16)"); API.md:75 cites the budgets; grep "50 concurrent|synthetic data set|first content" over docs/plan: phase-02 chain test (not a budget) only; phase-04.md:1179 has a p95 300 ms budget for GET /inbox only; no queue budget test in phases 17 or 18.
- Proposed 7.7 text:

"### 7.7 Response budgets for the queues (UX-16; ROADMAP P1 quality gate)
1. Budgets: `GET /queues/{id}/items` and `GET /cases/{id}` first content within 1 s; `POST /queues/{id}/next` p95 under 300 ms; the type-ahead of GET /search (phase 18 section 7.12) p95 under 300 ms (API.md:75).
2. Test: a seeded synthetic fixture of at least 10,000 alerts across 50 queues (SP-06: synthetic only) run with 50 concurrent sessions in the phase 00 CI harness at the release gate; the job fails when a p95 is over its budget, and the figures go to the release notes.
3. Jobs longer than about 10 s show the percent progress of the phase 08 Progress component (IR-73); the budget excludes them.
4. Phase 17 section 6 gains check 28: the fixture run above."

### G1-08-01 Usability round 1 gate (UX-03)
- Severity: LOW
- Category: Missing Requirement
- Requirement: UX-03 (research/31-analyst-ux.md:903): usability rounds with three to five participants per group, Arabic first, before and during the build
- Target: phase-08.md section 7.4
- Check: ROADMAP.md:133 (P0 track A7 "Usability round 1 ... (UX-03)"); DESIGN-SYSTEM.md:974 (DS-O4 closes with "First usability round with Iraqi staff (UX-03)"); grep "usability" in docs/plan: none.
- Proposed 7.4 text:

"### 7.4 Usability round 1 before the P0 screens (UX-03, DS-O4; ROADMAP track A7)
Round 1 runs with three to five Iraqi compliance staff per distinct group (MLRO team, analysts, branch liaison), Arabic first, on paper and clickable prototypes of X1, X6, W3 and E1, with a mixed-script checklist. Its output is a dated decision record in docs/plan/research.md, which closes DS-O4 (order of numeric ranges). Section 3.8 screens are merged only after that record exists. No code, route, table or setting is added."

### G1-08-02 Accessibility release gate for P1 (UX-39)
- Severity: MEDIUM
- Category: Missing Requirement
- Requirement: UX-39 (research/31-analyst-ux.md:939): keyboard walkthroughs of six tasks, 400 percent zoom in Arabic, NVDA and VoiceOver passes with mixed-script names
- Target: phase-08.md section 7.5
- Check: DESIGN-SYSTEM.md:906-910 (per-release walkthrough of six core tasks, 400 percent zoom, NVDA and VoiceOver passes, conformance statement); ROADMAP.md:230-231 (P1 accessibility pack); phase-08.md:601 axe on every route and :628 keyboard pass cover P0 only; phase-17.md:1651 and phase-18.md:1197 keyboard checks per screen only; grep "NVDA|VoiceOver|400 percent" in docs/plan: none.
- Proposed 7.5 text:

"### 7.5 Accessibility release gate (UX-39; DESIGN-SYSTEM section 9; ROADMAP P1 accessibility pack)
Added to the release job of section 4 (a gate per release, not a per-phase test):
1. axe on every route in both languages and every shipped theme (section 6 already).
2. Keyboard-only walkthrough of the six core tasks (open a case, record a disposition, request information, escalate, sign a report, answer an RFI), as a signed checklist per release.
3. 400 percent zoom (320 CSS px reflow) in Arabic on the screens of those six tasks.
4. Screen-reader passes with NVDA and VoiceOver on the same six tasks, including mixed-script names; the result goes into the conformance statement of the release (DS-O5).
A failed item blocks the release bundle of phase 25."

### G1-24-01 Register exports with CSV and XLSX (UX-31)
- Severity: MEDIUM
- Category: Missing Endpoint
- Requirement: UX-31 (research/31-analyst-ux.md:931): exports as CSV in UTF-8 with a byte-order mark, ISO dates, Western digits and formula-injection guards; XLSX with a right-to-left sheet view; access event and audit entry
- Target: phase-24.md section 7.12
- Check: DESIGN-SYSTEM.md:8.1 REGISTER pattern "an export (SU job)" (section 8.1 pattern table); phase-24.md:599 "Export is not in this phase" (audit viewer only); phase-24.md:569 and :809 pack download (PDF, step-up `export`) is built; grep "xlsx|csv|formula|byte-order" in phases 15-24: none for registers; phase-02.md:150 and phase-24.md:617 access verb `export` exists.
- Proposed 7.12 text:

"### 7.12 Register exports (UX-31; DESIGN-SYSTEM 8.1 REGISTER pattern)
Each REGISTER screen (W10, W11, S3, S4, G4 to G9, D1, K4) offers one export, built as a job.

| Method | Path | Scope | Description |
|---|---|---|---|
| POST | /api/v1/exports | the scope of the register screen | Starts a register export `{register, filters, format: csv or xlsx}`; 202 with the job id; step-up class `export`; `X-Access-Purpose` required (phase 03 purpose list); `Idempotency-Key` |
| GET | /api/v1/exports/{id} | the same scope, owner only | Status; the file link when done |
| GET | /api/v1/exports/{id}/file | the same scope, owner only | The file; access event `export` written first (phase 02 verb) |

1. The job kind `register_export` is registered in phase 00 section 4.1 (new kind, one line).
2. The writer applies the register's own visibility and masking (suspicion rows absent for non-members; the report register is never exported as a count, RP-61).
3. CSV: UTF-8 with a byte-order mark, ISO 8601 dates, Western digits; any text cell that starts with `=`, `+`, `-`, `@`, a tab or a carriage return is prefixed with an apostrophe.
4. XLSX: the same guard; sheets with Arabic content carry the right-to-left sheet view.
5. PDF stays on the pack path (section 3, Gotenberg, RQ-11).
6. The file's SHA-256 is written in the access event. No per-user watermark is added here (see the Summary, UNCLEAR).
7. Test: a formula cell in a fixture name is prefixed in both formats; a suspicion row is absent for a non-member and the export count equals the screen count."

## Summary

Counts by class (54 IDs):
- CARRIED: 13 (UX-05, UX-10, UX-40, UX-47, UX-56, UX-58, UX-61, UX-62, UX-69, UX-70, W-03, W-04, W-09)
- Partly: 20 (UX-09, UX-11, UX-17, UX-37, UX-42, UX-43, UX-48, UX-51, UX-57, UX-71; W-01, W-07, W-08, W-10, W-15, W-18, W-20, W-21, W-23, W-24)
- NOT-CARRIED: 13 (UX-04, UX-46, UX-52; W-02, W-05, W-11, W-12, W-13, W-14, W-16, W-17, W-22, W-25)
- GAP: 8 (UX-03, UX-07, UX-12, UX-16, UX-29, UX-31, UX-39, UX-59)

GAP findings by severity (8 findings; X2 is in G1-18-01 with UX-29):
- HIGH: G1-18-01 (global search, X2, UX-29)
- MEDIUM: G1-08-02 (UX-39), G1-17-01 (UX-12), G1-17-02 (UX-16), G1-18-02 (UX-07), G1-24-01 (UX-31)
- LOW: G1-08-01 (UX-03), G1-18-03 (UX-59)

UNCLEAR (for the lead):
1. UX-31 and UX-51, watermark: research/23 RP-61 says exports are watermarked per user and ROADMAP.md:208 says R3 has watermarks; phase-21 DR-172 and section 3.9 step 4 decide no per-user watermark (phase-21 Open doubts 14, bank decision). Left open in G1-24-01 and classed Partly for UX-51.
2. W-04, "no mandatory long parallel runs" (research/04) against the 30-day shadow of S-47 and DSL A.10 (phase-16.md:27, :573). Classed CARRIED; the conflict needs a decision.
3. UX-04 (analyst workspace telemetry): no P0 or P1 feature line; classed NOT-CARRIED on the parallel-run reason. Lead may prefer GAP.
4. W-12, W-13, W-14, W-17, W-25: NOT-CARRIED because no MODULES or ROADMAP P0 or P1 line names them and no DR decides them. Each is a scope call for the lead.
5. W-16 adverse media: ROADMAP.md:361 names "P1 PEP and adverse-media depth" as an open external item, while phase-18.md:1438 says no source exists. Classed NOT-CARRIED on the source.
6. W-14: research/27 line 313 counts 13 TRADE scenarios in the 161-scenario register; those belong to the S- IDs of another run, not checked here.
7. phase-24.md:1378 names phase 28 as the capacity workbench; the workbench is phase 31 (G2). Text fix for the lead.
8. UX-03 is a process step (usability round), classed GAP at LOW because ROADMAP.md:133 lists it in P0.
