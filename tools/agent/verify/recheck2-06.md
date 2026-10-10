# Second re-check, phase 06 (Haiku draft for lead review)

### S-06-1  HIGH   (fix F6, [R-06-5])
- Problem: The new AttributeKey sentence limits the type to canonical columns (the CanonicalRef form, checked when built), but section 7.3 says catalogue keys also take the `feat.` and `rating.` namespaces, which are not canonical columns. The grader looks up register attributes in that catalogue, and phase 13 says a scenario grade reads `feat.<key>` as an attribute, so the two rules cannot both hold.
- Proof: docs/plan/phase-06.md:1055 "holds one canonical attribute reference (`<entity>.<field>`, the `CanonicalRef` form of DSL.md B.1)"; docs/plan/phase-06.md:993 "Keys have the form `<namespace>.<name>`"; docs/plan/phase-06.md:995 "are not canonical columns, so that test skips them"; docs/plan/phase-06.md:551 "for an attribute whose catalogue entry has `needs_declaration = true`"; docs/plan/phase-13.md:941 "so the readiness grade of a scenario reads `feat.<key>` as an attribute" (phase 13, outside this phase).
- Smallest fix: In docs/plan/phase-06.md line 1055, replace "holds one canonical attribute reference (`<entity>.<field>`, the `CanonicalRef` form of DSL.md B.1)" with "holds one attribute key of the form `<namespace>.<name>` that section 7.3 admits (a canonical `<entity>.<field>`, or a registered `feat.` or `rating.` key)". If canonical only is kept instead, section 7.3 (lines 993 to 995) and phase 13 line 941 must be changed to match.

### S-06-2  LOW   (fix F6, [R-06-5])
- Problem: The new sentence places `grade` in section 2, but the only `grade` signature that takes `AttributeKey` is in section 3.2. Section 2 has `Need` with `AttributeKey`, but no `grade` function.
- Proof: docs/plan/phase-06.md:1055 "used here and by `Need` and `grade` in section 2"; docs/plan/phase-06.md:544 "`grade(register, availability: &[SourceAvailability], maintained: &[AttributeKey]" (under the heading at line 542, "### 3.2 Readiness grading").
- Smallest fix: In docs/plan/phase-06.md line 1055, change "used here and by `Need` and `grade` in section 2" to "used here, by `Need` in section 2 and by `grade` in section 3.2" (this drops out if S-06-1 rewrites the sentence).

### S-06-3  LOW   (fix F6, [R-06-5])
- Problem: CanonicalRef is defined in DSL.md section 1 (Shared types), not in B.1. B.1 (the mapping document structure) does not name it, and phase 10 already cites DSL.md s.1 for the CanonicalRef examples.
- Proof: docs/plan/phase-06.md:1055 "the `CanonicalRef` form of DSL.md B.1"; docs/DSL.md:88 "**CanonicalRef** — `entity.attribute` from the canonical catalogue" (under "## 1. Shared types", docs/DSL.md:71); docs/plan/phase-10.md:2021 "`CanonicalRef` examples of DSL.md s.1".
- Smallest fix: In docs/plan/phase-06.md line 1055, change "the `CanonicalRef` form of DSL.md B.1" to "the `CanonicalRef` form of DSL.md s.1" (this drops out if S-06-1 rewrites the sentence).

## Summary
Entries checked: 6 (F1 to F6).
Findings: HIGH 1, MEDIUM 0, LOW 2.
CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 S-06-1 and S-06-2 and S-06-3
