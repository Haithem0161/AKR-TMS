# Second re-check, phase 11 (Haiku draft for lead review)

### S-11-1  HIGH   (fix F7, [R-11-5b])
- Problem: The new paragraph says none of "the last three" callers may link a driver. The three named last are akr-ingestd, akr-worker and akr-ctl, and the same section says only akr-ingestd links a driver. The two rules cannot both hold for akr-ingestd. The intended set is akr-api, akr-worker and akr-ctl.
- Proof: docs/plan/phase-11.md:516 "only akr-ingestd links a driver (phase 05 rule)." and docs/plan/phase-11.md:519 "and none of the last three may link a driver."
- Smallest fix: in docs/plan/phase-11.md line 519, replace "and none of the last three may link a driver." by "and none of akr-api, akr-worker or akr-ctl may link a driver."

### S-11-2  MEDIUM   (fix F1, [R-11-1a])
- Problem: The verdict for R-11-1 says the payload of resume step 4 must name `as_of_marker`. Step 4 still lists only `{decision, tables, check_id}`, and F1 changed only the 7.13 sentence. Section 7.13 is headed "Section 3.5 resume step 3" and does not say it replaces the step 4 payload, so the two lines still disagree for a `false_alarm` after an `as_of_regressed` halt.
- Proof: docs/plan/phase-11.md:962 "audit `source.halt_resolved {decision, tables, check_id}`" and docs/plan/phase-11.md:1610 "`source.halt_resolved` carries it as `as_of_marker`, beside `decision`, `tables` and `check_id`."
- Smallest fix: in docs/plan/phase-11.md line 962, after "`{decision, tables, check_id}`" add "(plus `as_of_marker` on a `false_alarm` after an `as_of_regressed` halt, 7.13)", the same wording F2 gives the audit table row.

### S-11-3  MEDIUM   (fix F9, [R-11-7])
- Problem: F9 now takes the columns from the newest accepted manifest, but the next sentence of the same item still refuses a table only when it has no receipt. A table whose only receipt is a rejected batch (reason `column_check_failed`, section 7.20 item 2) has a receipt, so it is not refused, yet it has no accepted manifest to take columns from. The refusal test must be the same as the source test.
- Proof: docs/plan/phase-11.md:1701 "A table with no receipt yet is refused with `E_VALIDATION` at" and docs/plan/phase-11.md:1691 "Otherwise the batch is `rejected` with reason".
- Smallest fix: in docs/plan/phase-11.md line 1701, change "A table with no receipt yet is refused" to "A table with no accepted manifest yet is refused".

## Summary
- Entries checked: 14 (F1 to F14, from applied2-11.txt).
- Findings: HIGH 1, MEDIUM 2, LOW 0 (3 in total).
CHECKED: F1 S-11-2, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 S-11-1, F8 ok, F9 S-11-3, F10 ok, F11 ok, F12 ok, F13 ok, F14 ok
