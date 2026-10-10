# Gap pass 1 run 6a: lead verdict (2026-10-10; decisions final, applied by apply-g1-run6a.py)

Haiku draft tools/agent/gap1-run6a.md: 56 L IDs; CARRIED 2, Partly 42, NOT-CARRIED 5, GAP 7 (MEDIUM 6, LOW 1).
Scope test: MODULES names only L-01..L-06, L-10, L-17, L-41 and L-47, and no L ID of this run is on a P0/P1 feature line
of MODULES or ROADMAP. The draft justified its gaps by the module's tier ("M10 is P1"), not by a feature line.

## Classification decisions
- CARRIED accepted: L-26 (16, `LinkedPartyResolver`), L-54 (18: decision four-eyes by Cedar, the workflow and
  `CHECK (decider_id <> proposer_id)`; generic text refused; the separate QA queue is phase 19).
- Partly accepted (42), with header text naming only the built part. Spot-checked: phase 02 `lang`, phase 10
  `admin.mapping_versions` and `quality_flags`, phase 12 `volume_count`, phase 14 section 3.9, phase 16 section 3.12 and
  `scenario.silence_business_days`, phase 17 `latency_s`, phase 19 `casework.qa_samples`, phase 24 section 3.6.
- NOT-CARRIED accepted: L-63, L-66 (P2, DR-394; phase 16), L-67 (P2, DR-394; phase 15), L-65 (P2, MODULES M11 line 434;
  phase 16, not 31), L-64 (P2, MODULES line 654 correspondent pack; phase 24, not 31).
- The draft's list of "missing parts P0 or P1 clearly needs" (about 35 Partly rows) names no P0/P1 line; none becomes a
  GAP in this pass.

## GAP findings (all 7 rejected)
- G1-15-01 L-09 REJECT: every `canon.account` row has a customer (`customer_id NOT NULL`), and a customer without a
  rating is shown as a control failure (phase 15, K-04). The daily value metric and alert-by-default are P2 (DR-397).
  L-09 Partly in phase 15. The draft also invented the attribute `rating.linked`.
- G1-14-01 L-36 REJECT, reclassified CARRIED in phase 14: research row L-36 means no minimum amount, materiality or
  exposure threshold. Phase 14 has none (every customer, counterparty and payment party is screened whatever the
  amount), freeze lists are excluded from good-guy rules and bulk "incorrect", and a lowered threshold is a material
  change. The draft read it as the name-match score and proposed zero score floors on freeze lists, which would alert on
  every shared token.
- G1-15-02 L-37 REJECT: the geography block (C1 nationality and residence) is built in phase 15; login, device and
  instruction-channel evidence are the optional channel links of S-20 (P2, DR-394); geofences are on no P1 line
  (DR-397). L-37 Partly in phase 15.
- G1-12-01 L-38 REJECT: payment parties already carry `quality_flags` (missing name, account, BIC; phase 10); analytics
  by operator, branch, customer and correspondent, cover payments and the repair queue are on no P1 line (DR-397).
  L-38 Partly in phase 10.
- G1-14-02 L-40 REJECT: evasion-language search is on no P1 line (DR-397). Not carried, phase 16.
- G1-15-03 L-50 REJECT: the CBS CDD level is read on every customer (`canon.customer.cdd_level`); a low-tier register
  with tier limits is on no P1 line (DR-397). L-50 Partly in phase 10.
- G1-19-01 L-62 REJECT: the authority-request register with its links is built (phase 19); the cross-unit search
  workbench is on no P1 line (DR-397). L-62 Partly in phase 19.

## Found while checking (not an L item)
- DESIGN-SYSTEM X2 (global search and command palette) is a P1 screen; phase 08 leaves it to P1 and no phase builds it.
  Logged as gap item (ab) in tools/agent/gap-items.md for run 7 (UX coverage) to place.

DR: DR-397 (the run 6a items outside this release).
