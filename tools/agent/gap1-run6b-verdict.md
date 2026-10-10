# Gap pass 1 run 6b: lead verdict (2026-10-10; decisions final, applied by apply-g1-run6b.py)

Haiku draft tools/agent/gap1-run6b.md: 50 T IDs; CARRIED 12, Partly 21, NOT-CARRIED 15, GAP 2 (HIGH 1, MEDIUM 1).
Scope test: MODULES names T-02, T-04, T-07, T-10, T-13..15, T-21, T-22, T-29, T-43, T-44 and T-48. The prompt's list
left out T-29: M11 lists it among its requirements, and its P1 feature line names "counterparty resolution".
Lead result: implemented 11, Partly 23, Not carried 16; GAP 1 accepted (T-29), 1 rejected (T-09).

## Classification decisions
- CARRIED accepted (11): T-01, T-40 (16); T-03, T-47 (17); T-06 (22); T-24, T-38 (19: hibernations are tied to a
  four-eyes no-report decision, with envelope and expiry; QA population `hibernation_hits`; regret flag); T-27 (14:
  there is no record merge in this release, and a differing national ID gives `DEC_SCR_ID_DIFFERS`); T-30, T-32, T-34
  (15). T-32: an upward override needs no second approver (DSL C.6), but a downward one does (CHECK
  `approved_by <> proposed_by`). That is the requirement where risk is lowered.
- Moved from CARRIED to Partly: T-65 (13). The percentiles are by segment, currency and period, but the thresholds have
  no calendar adjustment.
- Partly, moved out of the P2 phase 27 (the rule: a T item that only phases 27 or 28 could build is Not carried):
  - T-11 is Partly in phase 18 (the controlled `CLS_` closure-reason vocabulary); the label registry is P2.
  - T-16 is Partly in phase 16 (replay on as-of history with the look-ahead flag, above-the-line precision and
    below-the-line miss-rate estimates); the workload-at-fixed-recall curves are P2.
  - T-46 is Not carried (model explanation tests, phase 27).
- Partly accepted as drafted, header naming only the built part: T-05, T-59, T-60 (24); T-08, T-17, T-18, T-42 (13);
  T-12, T-23 (19); T-28 (14); T-31, T-33 (15); T-35, T-36 (17); T-41 (11); T-45, T-61 (16).
- T-37 is Partly in phase 22 (branch RFIs go to the liaison officer and escalate to the MLRO desk). See "Other edits"
  for the branch-first switch.
- NOT-CARRIED accepted: T-26 (10, P2, DR-394) and T-39 (16; DSL A.7 has no auto-close in P1, and the CBI position is to be
  sought).
- NOT-CARRIED placed in phase 27 instead of 13, 26 or 31: T-52 to T-58. Phase 26 is the Islamic pack, and cross-bank and
  synthetic-data work belongs with the model work (ROADMAP risk 9). T-19, T-20, T-25, T-46, T-62, T-64 and T-66 are also
  in phase 27.

## GAP findings
- G1-13-01 T-29 ACCEPTED HIGH, rewritten (DR-398).
  - Why it is in scope: M11 [P1] lists T-29 and its P1 line names counterparty resolution. DSL A.6 already calls the
    resolver a feature writer. Phase 16 Open doubts 21 says no phase writes it.
  - Why a new table: `canon.counterparty` is mapped and append-only, so its `party_id` is only what the CBS links.
  - Phase 13 section 7.5 adds:
    - migration `0044_feat_counterparty_links` (the number phase 13 left unused) with table `feat.counterparty_links`.
      Methods: `own_account`, `confirmed_account`, `name_key` (a suggestion only), `none`, `analyst_confirmed`,
      `analyst_rejected`. There is no case reference in the row, so the table says nothing about suspicion.
    - job `counterparty_resolve`, run hourly on a schedule (no outbox handler).
    - settings `counterparty.recheck_days` and `schedule.counterparty_resolve`.
  - Learning: an account hash already confirmed by an analyst links new counterparty rows with the same hash.
  - Phase 18 section 7.11 adds `POST /cases/{id}/counterparty-links`. It reuses the Cedar action `case.link.write`. The
    counterparty must be on a transaction linked to the case. The decision goes into the case chain, and
    `GET /cases/{id}/links` shows the open link for each transaction link.
  - Changes from the draft:
    - No new `entity_links` kind and no `case_id` in the feat table.
    - The `canon.touched` consumer is dropped (phase 14 registers the first handler).
    - `confidence` is dropped (the methods carry the meaning).
  - T-29 is Partly in phase 13: the depositor names of cash deposits are not captured (S-21, phase 16 Open doubts 2).
  - DATA-MODEL s.8 and the group 13 migration row, and roadmap rows 13 (entities and migrations), are updated. API.md
    gets one row.
- G1-15-01 T-09 REJECTED.
  - No MODULES or ROADMAP P0/P1 line names segment-migration alerts or a KYC refresh on a cluster change.
  - "Trigger library re-rating on events" (M10) names no segment source.
  - The membership history (`feat.segment_memberships`, `valid_from` and `valid_to`) is built. T-09 is Partly in
    phase 13, and the alert and refresh trigger are P2 (DR-399).
  - The phase 13 Open doubts that asked phase 17 to carry it are closed.

## UNCLEAR decisions
- T-37 and the branch-first switch: no P1 line names branch-first routing, and no phase defines its five conditions,
  although phase 09 says phases 17 and 22 define them. Phase 09 now ships the switch `fixed: true` at `false`, so the
  existing rule (`E_INVARIANT_LOCKED`) refuses `true`. The five conditions are P2 (DR-399). This fixes the text of
  phase 09 section 3.5 and its Open doubts, and phase 17 Open doubts 20 and 22.
- T-05 and T-60: the other S-53 metrics, unique catches, overlap, retire-or-keep advice and funnel stages are on no
  P0/P1 line (P2, DR-399).
- T-23: the graph stays one hop (DR-154). Degree, reach, hops to listed entities and neighbour-risk features are P2
  (DR-399).
- T-08: segment of one for large corporates and government bodies is P2 (DR-399). The phase 13 Open doubt is closed;
  until then such customers fall to the root segment.
- T-41: disclosing the CBS posting cadence and separate event-time and load-time columns are on no P1 line (DR-399).
- T-26: merge and unmerge stay P2 (DR-394). The persistent resolution of counterparties is the T-29 resolver.

## Other edits
- SP-30 (canary records, P2 by DR-391) was on no header line; it is now on the phase 25 not-carried line (inline fix
  after the script).

DRs: DR-398 (the counterparty resolver; the branch-first switch fixed at false), DR-399 (the run 6b items outside this
release).
