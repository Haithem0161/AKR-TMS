# Owner-decision check, round 2, key DR (Haiku draft for lead review)

### P-DR-1  HIGH   (fix F1, [DR-422])
- Problem: Claim (4) of the row says each deletion of an uploaded file writes `goaml_corpus_case.transient_deleted`. Phase 07 section 7.6 step 5 deletes the staged upload when a proposal fails, and item 9 of the same section says that failed proposal writes no event. The two rules cannot both hold. (The 7.6 header at docs/plan/phase-07.md:1021, "each deletion of an uploaded file is audited", carries the same over-broad sentence; that line belongs to another key.)
- Proof: docs/plan/research.md:433 "each deletion of an uploaded file writes `goaml_corpus_case.transient_deleted`" and docs/plan/phase-07.md:1202 "The failed proposal of item 5 step 5 writes no event: no case row exists, as for a failed check request (section 3.2 step 4)."
- Smallest fix: In row DR-422 item (4), replace "each deletion of an uploaded file writes `goaml_corpus_case.transient_deleted`" with "each deletion of an uploaded file made by an intake run or the sweep writes `goaml_corpus_case.transient_deleted`; a failed proposal writes no event".

### P-DR-2  HIGH   (fix F1, [DR-422])
- Problem: Claim (6) says a statement by a role without the privilege fails with permission denied. Phase 13 section 6 item 1 says `akr_app` gets the guard message for an UPDATE of `expression`, but `akr_app` has no UPDATE grant on that column (only `state` and `retired_at`), so PostgreSQL refuses it with permission denied before the guard runs. The claim is not carried by this check line.
- Proof: docs/plan/phase-13.md:972-973 "Negative checks as `akr_app`: `DELETE FROM feat.feature_values` fails with permission denied; `UPDATE feat.feature_definitions SET expression = '{}'` fails with `feature definition content is immutable`"; GRANT at docs/plan/phase-13.md:317 "GRANT UPDATE (state, retired_at) ON feat.feature_definitions TO akr_app;".
- Smallest fix: In phase 13 section 6 item 1 (line 973), change "`UPDATE feat.feature_definitions SET expression = '{}'` fails with `feature definition content is immutable`" to "fails with permission denied (no UPDATE grant on `expression`), and as the owner `akr_migrate` with `feature definition content is immutable`".

## Summary
- Entries checked: 1 (F1, the row DR-422 of docs/plan/research.md). Its six claims were checked sentence by sentence against phases 06 to 26, its names against the owning phases, and its SQL against PostgreSQL 17 by reading.
- Findings by severity: HIGH 2, MEDIUM 0, LOW 0.
- Carried without a finding: claim 1 (phases 10 and 26), claim 2 (phase 19), claim 3 (phase 13), claim 4 (phases 07 and 14, except the deletion audit), claim 5 (phase 25), and claim 6 (phases 06 to 21, except phase 13 section 6).

CHECKED: F1 P-DR-1, P-DR-2
