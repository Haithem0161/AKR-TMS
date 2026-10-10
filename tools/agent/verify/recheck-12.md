# Re-check, phase 12 (Haiku draft for lead review)

### R-12-1  LOW   (fix F15)
- Problem: Block 7.8 quotes block 7.5 as saying "the latest recon result for its object and field". Block 7.5 does not contain those words. It says "the issue's latest recon result for its object and field". The reference is clear, but a search for the quoted words finds nothing.
- Proof: docs/plan/phase-12.md:1355 "In block 7.5, "the latest recon result for its object and field" is this read" and docs/plan/phase-12.md:1331 "the delta of the issue's latest recon" (line 1332 continues "result for its object and field.").
- Smallest fix: On line 1355, replace the quoted words with "the issue's latest recon result for its object and field".

### R-12-2  MEDIUM   (fix F16)
- Problem: The fix makes block 7.5 say that `DqRepository::accepted_for` takes the field, but the base trait in section 2 still declares the method without a field, and the step 7 call still omits it. F16 is a name change inside block 7.5. It is not a new block that says it replaces the base text, so the 7.x precedence rule does not settle the conflict, and the phase states two signatures for one method.
- Proof: docs/plan/phase-12.md:1332 "`DqRepository::accepted_for` takes the field: `accepted_for(conn, source_id," and docs/plan/phase-12.md:532 "async fn accepted_for(&self, conn: &mut PgConnection, source_id: Uuid, object: &str, kind: DqKind)" and docs/plan/phase-12.md:678 "`accepted_for(source, <rung>:<object>, kind = recon_break)`".
- Smallest fix: Add `field: &str` after `object: &str` in the signature on line 532, and pass the group key as the field in the step 7 call on line 678.

## Summary
Fixes checked: 20 (F1 to F20).
Findings by severity: HIGH 0, MEDIUM 1, LOW 1.
CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok, F8 ok, F9 ok, F10 ok, F11 ok, F12 ok, F13 ok, F14 ok, F15 R-12-1, F16 R-12-2, F17 ok, F18 ok, F19 ok, F20 ok
