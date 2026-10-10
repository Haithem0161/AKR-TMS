# Second re-check, phase 14 (Haiku draft for lead review)

### S-14-1  MEDIUM   (fix F3, [R-14-2b])
- Problem: The NEW text of F3 types the `rule` field of `GoodGuyProposal` as `GoodGuyRuleInput`. That name occurs nowhere else in docs/plan and no line of phase 14 defines it or gives its fields. Section 3.8 describes the rule only in prose, so a developer cannot write this DTO from the phase text.
- Proof: docs/plan/phase-14.md:696 "GoodGuyProposal { list_codes: Vec<ListCode>, rule: GoodGuyRuleInput, reason_codes, expires_on, reason_text }"
- Smallest fix: In section 7.16, add one line that defines `GoodGuyRuleInput` with its fields (the folded name tokens that must all be present, and the optional identifier conditions), as section 3.8 describes the rule object.

## Summary
- Entries checked: 4 (F1 to F4).
- Findings by severity: HIGH 0, MEDIUM 1, LOW 0.
CHECKED: F1 ok, F2 ok, F3 S-14-1, F4 ok
