# Third re-check, phase 18 (Haiku draft for lead review)

### T-18-1  MEDIUM   (fix F1, [L2-17-1b])
- Problem: The F1 replacement is correct where it stands (section 3.2, line 649), but section 4.5 item 4 of the same phase is base text that no 7.x block overrides, and it still states the old unconditional reassignment rule. The phase now gives two rules for the same revocation that cannot both hold.
- Proof: docs/plan/phase-18.md:1132 "and revokes the previous assignee's grant on reassignment." conflicts with docs/plan/phase-18.md:649 "a grant of another reason stays".
- Smallest fix: In docs/plan/phase-18.md line 1132, replace "and revokes the previous assignee's grant on reassignment." with "and revokes the previous assignee's grant of the two reasons `assignment` and `second_review` on reassignment (section 3.2; phase 17 section 7.25)."

## Summary
Entries checked: 1 (F1, [L2-17-1b]). Findings by severity: HIGH 0, MEDIUM 1, LOW 0.
F1 is correct at docs/plan/phase-18.md:649 (present, readable, cross-reference to phase 17 section 7.25 verified, names `revoked_by`, `assignment` and `second_review` exist in the case_acl table). The conflict is with the unchanged base text at docs/plan/phase-18.md:1132 (section 4.5 item 4), which no 7.x block overrides.
CHECKED: F1 T-18-1
