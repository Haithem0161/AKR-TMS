# Gap pass 1 run 7b: lead verdict (2026-10-10; decisions final, applied by apply-g1-run7b.py)

Haiku draft tools/agent/gap1-run7b.md: 46 IDs (F 24, B 13, U 9); CARRIED 19, Partly 19, NOT-CARRIED 6, GAP 2 (MEDIUM 2).
Lead result: implemented 18, Partly 22, Not carried 6. GAP 1 accepted and rewritten (F-07), 1 rejected (U-01).

## GAP findings
- G1-16-01 F-07 ACCEPTED MEDIUM, rewritten and moved to phase 17 (DR-401).
  - In scope: MODULES M10 P1 "trigger library re-rating on events"; the shipped library TR-01 to TR-26 (DSL C.5) has
    TR-11 (declared-versus-actual breach, research 26 section 6c) on `profile.breach`. Phase 15 claims K-11, but its
    trigger table leaves `profile.breach` to "the phase that produces" it, and no phase writes it.
  - Why phase 17 and not 16: phase 16's sink only records; phase 17's `AlertIntakeService` replaces it (phase 16
    section 7.5) and creates the alert. The draft's "from phase 15 on" was backwards: phase 17 runs after 15, so it
    writes the topic and registers the first handler (the DR-375 pattern of `screening.hit`).
  - Phase 17 section 7.8: in section 3.1 step 12, outcome `New` of a scenario with `logic.kind = profile` and a
    `declared` expectation writes outbox `profile.breach {party_id}` (the case subject). Duplicates and amendments
    write nothing. The handler is phase 15's TR-11 trigger, stored with the neutral `TRG_REVIEW_REQUESTED` (an alert is
    suspicion data, phase 15 section 3.6).
  - Phase 15 section 7.6: the note "from phase 17 on".
  - F-07 is implemented in phase 15 (the clocks by level, plus the trigger from phase 17 on).
- G1-14-01 U-01 REJECTED.
  - No P0/P1 line names schema validation. M9 P1 names "list ingestion ... with versions, checksums and a daily
    known-answer harness", and phase 14 has them: fetch through the egress proxy, SHA-256, versions, publisher-count
    reconcile, the count-drop rule and the gate before activation.
  - U-01 is Partly in phase 14. The XSD check, the signed-redirect check and an hourly poll (the floor is 12 hours) are
    P2 (DR-401).

## Classification decisions
- CARRIED accepted (17): F-02, F-10, B-10, B-13 (15); F-05, B-02, B-05, B-07 (16); F-13, U-02, U-03, U-09 (14); F-16
  (18); F-17 (23); B-01 (12); B-04 (03); B-11 (24). With F-07 from the accepted gap: 18 implemented.
- Moved from CARRIED to Partly:
  - F-04 to phase 16: profile scenarios compare against declared figures, own history and peers; the comparison with
    the product's purpose is missing.
  - F-14 stays on phase 09: the country table with effective dates drives the floors and TR-10; blocking rules are
    payment-time checks (phase 29, P2).
- Partly accepted as drafted, header naming only the built part: F-03, F-08, F-09, F-22, B-03 (15); F-06, F-24, B-06
  (16); F-15, B-08 (18); F-19, F-21, F-23 (24); B-09, U-04, U-05, U-07, U-08 (14); U-06 (20). U-01 from the rejected
  gap (14).
  - F-08: AKR-TMS reads the CBS and sets no transaction limits (that is the bank's CBS); the completeness cap holds
    the level, so monitoring is tighter.
  - U-05: phase 14's harness report already has the list lag (publication to activation); a removal reopening a
    freeze stays a proposal (phase 20 section 3.5).
  - U-07: the 1518 designations come in the UN consolidated list; immediate family, ownership chains and transfer
    instructions are missing.
  - U-08: the UN designations and the PF scenarios are built; the Iran-nexus flag and the ownership test are missing.
    DSL A names a `regime` pack value "PF-Iran" while ROADMAP P3 names US, UK and EU regime packs; this run does not
    change the pack tiers (noted for pass 2).
  - The missing parts of all Partly rows are on no P0/P1 line (DR-401).
- NOT-CARRIED accepted: F-11, B-12 (31, the P2 correspondent pack); F-12 (29, payment-time checks, P2); F-18 (03,
  subsidiaries and group view; further banks are P3 installs); F-20 (27, ML models, P2).
- F-01 placed on phase 15 instead of 24, beside W-17 (enterprise-wide risk assessment, DR-400): the assessment is the
  bank's own document (ROADMAP P1: "alignment with the bank's own risk assessment").

## Found while checking (logged in gap-items.md for pass 2)
- (ac) Other trigger topics of DSL C.5 that no phase writes: `txn.new_corridor` (TR-12), `account.reactivated`
  (TR-14), `account.opened` (TR-24), `outreach.failed` (TR-20), `visit.recorded` (TR-19); `media.recorded` (TR-09)
  and `staff_link.recorded` (TR-25) are P2 sources. Phase 15 claims K-11 (the library).
- (ad) Phase 15 `pep_list_sweep` source 1 reads screening alerts with `list_kind = 'pep'`, but `ref.lists.code` is
  closed to six codes and none is seeded as kind `pep` (DR-100 "`pep` a kind only"), so that source never fires.

DR: DR-401 (the `profile.breach` producer in phase 17; the run 7b items outside this release).
