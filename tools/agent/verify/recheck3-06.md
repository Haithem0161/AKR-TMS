# Third re-check, phase 06 (Haiku draft for lead review)

### T-06-1  MEDIUM   (fix F1, S-06-1)
- Problem: The column comment on `scn.readiness_grades.missing_fields` still says the stored keys are canonical. After F1 the column holds `AttributeKey` values, which can be registered `feat.` or `rating.` keys that section 7.3 says are not canonical columns (docs/plan/phase-13.md:941 says the readiness grade "reads `feat.<key>` as an attribute", and section 3.2 rule 2 adds a missing need to `missing_fields`), so the comment states the old rule for the same values. It has no build effect.
- Proof: docs/plan/phase-06.md:195 "-- canonical attribute keys, for filters and the report" and docs/plan/phase-06.md:995 "are not canonical columns, so that test skips them" (the new F1 text at docs/plan/phase-06.md:1055 says "holds one key of the `AttributeCatalogue`").
- Smallest fix: on docs/plan/phase-06.md line 195, change the comment "canonical attribute keys" to "AttributeCatalogue keys (section 7.3)".

## Summary
- Entries checked: 1 (F1).
- Findings: HIGH 0, MEDIUM 1, LOW 0.
- CHECKED: F1 T-06-1
