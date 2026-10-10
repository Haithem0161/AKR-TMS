# Gap pass 1 run 8c: lead verdict (2026-10-10; decisions final, applied by apply-g1-run8c.py)

Haiku draft tools/agent/gap1-run8c.md: 86 market IDs (VC 1-86, research files 14, 15 and 16); CARRIED 11, Partly 42,
NOT-CARRIED 32, GAP 1 (MEDIUM). About 226k tokens.
Lead result: implemented 11, Partly 43, Not carried 32. GAP accepted, rewritten.

## GAP finding
- G1-14-01 VC-03, the one-hour re-screen after a list activation, ACCEPTED MEDIUM (DR-404).
  - In scope: ROADMAP M9 "full re-screen within one hour of a list update" and the exit criterion "At most one hour
    from a list update to a full re-screen". Phase 14 has the 12-hour freshness rule (section 3.2) but nothing measures
    or alarms on the time from activation to the end of the look-back (grep "one hour", "rescreen" in phase 14:
    nothing).
  - Rewritten: no new timer kind. The draft's timer `list_rescreen_clock` would need a new timer owner kind and a
    cancel path; the existing `list_stale_sweep` (every 10 minutes) checks instead. The re-screen of a version is its
    `lookback` run (`alert.screening_runs.trigger_ref` = the version). Duration = the run's `finished_at` minus the
    version's `activated_at`, kept as a histogram and in the `screening.run_finished` payload. A version active for
    more than one hour with no completed look-back writes `list.rescreen_late` once (the `list.stale` transition
    pattern), sets a gauge and, for a freeze list, a critical alert. No degraded flag on later runs (the draft's
    `rescreen_late` flag): the look-back is the late thing, not the later runs. One hour is a fixed constant (it is the
    exit criterion, not a bank choice). No new table, column, job, timer or route.
  - Phase 14 section 7.10. VC-03 goes on the Implemented line.

## Classification decisions
- CARRIED accepted (10): VC-04, VC-10, VC-11, VC-12, VC-24, VC-31 (14); VC-55, VC-59 (15); VC-81 (02); VC-85 (05).
  With VC-03 from the accepted gap (14): 11. Spot checks: VC-11 (section 3.4 step 6, a default date earns no credit;
  identifiers count only when comparable on both sides), VC-24 (pair memory with proposer, approver, expiry and the
  Monday QA sample; good-guy rules by change request, never on freeze lists), VC-55 (`kyc.ownership_inputs` one row
  per owner with `share_pct`, evidence and `valid_from`; the 20 percent threshold applies to the result only).
- Moved from CARRIED to Partly: VC-06 (14). The look-back screens the version's added and amended entries against
  "all current subjects" and onboarding screens changed parties against every active list, but the row's "recent
  counterparties" are not named in the look-back's subject set.
- Moved from phase 10 to phase 11: VC-16. Phase 11 section 7.6 owns the ISO 20022 and MT parsers (by element path or
  field tag, the raw message kept); fixtures exist for pacs.008 and MT103 only, and no per-format adapter (MT202 COV
  underlying customer, the 700 series, pacs.009, pacs.004, camt) is built. MODULES M5 names "payment messages (ISO
  20022 and MT)" without a format list.
- Partly accepted, header naming only the built part: VC-01, VC-02, VC-05, VC-06, VC-07, VC-08, VC-09, VC-13, VC-14,
  VC-15, VC-23, VC-25, VC-26, VC-28, VC-29, VC-30, VC-33, VC-35, VC-37, VC-42, VC-56, VC-71 (14); VC-21, VC-39, VC-47,
  VC-48, VC-57, VC-66, VC-67, VC-68, VC-69, VC-79, VC-80 (15); VC-70, VC-72 (10); VC-16 (11); VC-32 (12); VC-74, VC-75
  (19); VC-36 (00); VC-38, VC-82 (24); VC-86 (05). The missing parts are on no P0/P1 line, or are external (ROADMAP
  Q9, Q11, Q12) or P2 (DR-404).
- NOT-CARRIED accepted (32): VC-17, VC-18, VC-20, VC-34 (29, payment-time P2); VC-19 (12); VC-22, VC-43, VC-44 (14);
  VC-41, VC-45, VC-51, VC-52, VC-53, VC-54, VC-58, VC-73, VC-77, VC-78 (15); VC-83 (02); the vendor, commercial,
  contract and procurement items VC-27, VC-40, VC-46, VC-49, VC-50, VC-60, VC-61, VC-62, VC-63, VC-64, VC-65, VC-76,
  VC-84 (25).
- UNCLEAR answered:
  - VC-21 and VC-57: the OFAC 50 percent aggregation and derived-designation records are on no P0/P1 line (MODULES
    M10 names only the 20 percent chain; grep "50 percent" in MODULES and ROADMAP: nothing). They stay Partly.
  - VC-16: Partly on phase 11 (above).
  - VC-19: stays Not carried on phase 12 (no P0/P1 line names a payment-format readiness report).
  - VC-03: the one-hour target is a ROADMAP M9 milestone and exit criterion, so it is in scope (gap accepted).
  - VC-47: DR-395 stands (KPI items a to e in `cbi_monthly`); the rest is Partly.

DR: DR-404 (the one-hour re-screen measure in phase 14; the run 8c items outside this release).
