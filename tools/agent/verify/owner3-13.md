# Owner-decision check, round 3, key 13 (Haiku draft for lead review)

### Q-13-1  MEDIUM   (fix F9, [P-DR-2])
- Problem: The owner clause of the F9 check (section 6 item 1, run by `make migrate` on a fresh database) cannot pass as written, because the guard `feat.definition_guard` is a row trigger and no migration inserts `feat.feature_definitions` rows (the seed is `akr-ctl features seed`, item 3), so the owner's UPDATE matches no row and raises nothing. The lead fixed the same missing row condition for P-19b-2 (F10, LOW), so the lead may re-rate this to match.
- Proof: docs/plan/phase-13.md:976 "and, as the owner `akr_migrate`, with `feature definition content is immutable`;" and docs/plan/phase-13.md:290 "FOR EACH ROW EXECUTE FUNCTION feat.definition_guard();" (the check runs at docs/plan/phase-13.md:971 "`make migrate`: 0040 to 0043 apply"; the seed is at docs/plan/phase-13.md:981 "`akr-ctl features seed --dry-run` lists nine features").
- Smallest fix: In docs/plan/phase-13.md line 976, change "as the owner `akr_migrate`, with `feature definition content is immutable`" to "as the owner `akr_migrate` on an existing row (the guard is a row trigger, so the owner's statement needs one), with `feature definition content is immutable`".

### Q-13-2  LOW   (fix F14, [L2-2])
- Problem: The DATA-MODEL row gives `no_history` as the only status of a row that a recompute no longer produces, but section 7.18 item 1 gives `beyond_window` for the kind `days_since` (the case the L2-2 verdict names). The pointer to section 7.18 resolves it, so this is wording only.
- Proof: docs/DATA-MODEL.md:545 "a row that a recompute no longer produces is kept without a value: `no_history`, `basis.reason = no_activity`" and docs/plan/phase-13.md:1441 "`beyond_window` for the kind `days_since`, as section 7.11 says of an account".
- Smallest fix: In docs/DATA-MODEL.md line 545, change "`no_history`, `basis.reason = no_activity`" to "`no_history` (`beyond_window` for `days_since`), `basis.reason = no_activity`".

## Summary

Entries checked: 14 (F1 to F14, all in docs/plan/phase-13.md except F14, which is in docs/DATA-MODEL.md).
Findings: HIGH 0, MEDIUM 1, LOW 1.
CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok, F8 ok, F9 Q-13-1, F10 ok, F11 ok, F12 ok, F13 ok, F14 Q-13-2
