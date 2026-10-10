# Re-check, phase 14 (Haiku draft for lead review)

### R-14-1  HIGH   (fix F3)
- Problem: The block says each step-up constant's `name` is the Cedar action of its route row, and that the bulk decision uses the same constant, whose name is `screening.alert.decide`. The bulk route's row carries the Cedar action `screening.alert.bulk_decide`, so the two rules cannot both hold, and a bulk step-up would be bound to the single-decision action.
- Proof: docs/plan/phase-14.md:1424 "whose `name` is the Cedar action of its route row in section 2.8" and "the bulk decision uses the same constant."; second quote, docs/plan/phase-14.md:834 "(`screening.alert.bulk_decide`)" (the row for `/screening/alerts/bulk-decision`).
- Smallest fix: In line 1424 replace "the bulk decision uses the same constant." with "the bulk decision declares `StepUpAction { name: "screening.alert.bulk_decide", class: decision }`."

### R-14-2  MEDIUM   (fix F8)
- Problem: The revoke body is called "the second form of `GoodGuyProposal`", but section 2.3 defines `GoodGuyProposal` as one struct with no `action` or `rule_id` field, and no second form is defined anywhere in the phase. A developer cannot tell which type the revoke body deserializes into, or how the route tells a revoke from a proposal.
- Proof: docs/plan/phase-14.md:696 "`GoodGuyProposal { list_codes: Vec<ListCode>, rule: GoodGuyRuleInput, reason_codes, expires_on, reason_text }`"; docs/plan/phase-14.md:1047 "(the second form of `GoodGuyProposal`;"
- Smallest fix: In line 1047 replace "(the second form of `GoodGuyProposal`;" with "(a second DTO, `GoodGuyRevoke { action, rule_id, reason_codes, reason_text }`, declared in section 2.3 beside `GoodGuyProposal`;" and add that DTO to line 696.

### R-14-3  LOW   (fix F12)
- Problem: The new sentence claims that no feed format of this release carries a delisting date, and cites `list_parse` step 3 for it. Step 3 only says the `feed_format` set is closed and names the stop-gap format. It names no format field and no delisting date, so the pointer does not support the claim.
- Proof: docs/plan/phase-14.md:949 "3. Parse by the closed `feed_format` set."; second quote, docs/plan/phase-14.md:1338 "No feed format of this release carries one (section 3.2 `list_parse` step 3)"
- Smallest fix: In line 1338 delete "(section 3.2 `list_parse` step 3)" and mark the sentence as a decision, for example "Decision: no feed format of this release carries a delisting date."

## Summary
- Fixes checked: 17 (F1 to F17), all five checks run on each: present, no conflict, names exist, SQL and types, closed.
- Findings: HIGH 1, MEDIUM 1, LOW 1 (FINDINGS: 3).
- CHECKED: F1 ok, F2 ok, F3 R-14-1, F4 ok, F5 ok, F6 ok, F7 ok, F8 R-14-2, F9 ok, F10 ok, F11 ok, F12 R-14-3, F13 ok, F14 ok, F15 ok, F16 ok, F17 ok
