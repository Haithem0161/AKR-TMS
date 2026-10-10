# Fifth re-check, phase 27 (Haiku draft for lead review)

## Summary
Entries checked: 1 (F1)
Findings: HIGH 0, MEDIUM 0, LOW 0
Basis for F1 (all line numbers from grep output): the NEW text is at docs/plan/phase-27.md:830 and reads correctly with balanced code spans. Section 3.4 step 1 is docs/plan/phase-27.md:505 (the Single read block). Route 16 is the 16th route row, docs/plan/phase-27.md:421 (`GET /alerts/{id}/model-score`; 16 route rows in total). The three permits of `alert.model_score.read` at docs/plan/phase-27.md:671, 675 and 679 test `principal.roles.contains` only, and the comment at line 669 says visibility is applied in the handler. Phase 17 check 6 at docs/plan/phase-17.md:1711 pins the section 3.10 predicate to the Cedar decision for `alert.read`, and docs/plan/phase-17.md:1233 says that predicate comes from `services::visibility::build(principal, network)`. Phase 17 route row docs/plan/phase-17.md:840 is `GET /api/v1/alerts/{id}`. No phase file names `alert.model_score.read` except phase 27, and the phase 17 forbids (docs/plan/phase-17.md:1437 to 1467) name explicit action lists without it. The phase 03 forbids at docs/plan/phase-03.md:596 and 600 are principal-level (status, SoD conflict), not tier-level. No other line of phase 27 states that the Cedar permit and the visibility predicate agree.
CHECKED: F1 ok
FINDINGS: 0
