# Owner-decision check, round 2, key 07b (Haiku draft for lead review)

### P-07b-1  MEDIUM   (fix F5, [L-5])
- Problem: The F5 sentence in 7.6 item 12 says the masked file is evidence "that only the corpus runner opens". Item 6 step 4 has the intake job open the same evidence object again on every later run (at profile activation), so the two statements cannot both hold. The intake job is the missing reader.
- Proof: docs/plan/phase-07.md:1225 "the file is `secret-str` evidence that only the corpus runner opens." and docs/plan/phase-07.md:1175 "`EvidenceStore::open(evidence_id)` gives the masked bytes."
- Smallest fix: In line 1225 replace "that only the corpus runner opens" with "that only the corpus runner and the intake job of item 6 step 4 open".

### P-07b-2  LOW   (fix F7, [O-07-10])
- Problem: The pointer "phase 04 section 3.2" does not carry the rollback. The statement that an effect error rolls back the approval is in phase 04 section 3.4 item 9. Section 3.2 has no "rolled back" wording.
- Proof: docs/plan/phase-07.md:1193 "(phase 04 section 3.2: an immediate approval is rolled back with the error, and a scheduled activation ends `failed`)" and docs/plan/phase-04.md:1023 "the approval is rolled back and answered" (the sentence goes on to "with the effect's error" on line 1024).
- Smallest fix: In line 1193 change "phase 04 section 3.2: an immediate approval is rolled back" to "phase 04 section 3.4 item 9: an immediate approval is rolled back".

### P-07b-3  LOW   (fix F14, [L-1])
- Problem: "the last check of this item" is wrong. Section 6 item 1 has six more checks after that parenthesis; the last of them is the akr_app no-DELETE check.
- Proof: docs/plan/phase-07.md:913 "(as `akr_app` it fails with permission denied, the last check of this item)" and docs/plan/phase-07.md:916 "`akr_app` has no DELETE on any `rpt.goaml_` table."
- Smallest fix: In line 913 delete ", the last check of this item" from the parenthesis.

### P-07b-4  LOW   (fix F14, [L-1])
- Problem: The quoted guard message drops the "goAML " prefix. The trigger raises "goAML profile versions are never deleted", so the quote is not the message the trigger raises, which the L-1 rule asks for. A substring check would still match, so this is LOW. The F15 entry quotes its message in full.
- Proof: docs/plan/phase-07.md:113 "RAISE EXCEPTION 'goAML profile versions are never deleted'" and docs/plan/phase-07.md:913 "`profile versions are never deleted` (as `akr_app` it fails with permission denied".
- Smallest fix: In line 913 change the quoted message to `goAML profile versions are never deleted`.

### P-07b-5  LOW   (fix F15, [L-1])
- Problem: The replacement starts a sentence with a lowercase "a" after a full stop, so the sentence does not read as correct English.
- Proof: docs/plan/phase-07.md:1230 "retention row. a `DELETE` fails, as `akr_app` with permission denied".
- Smallest fix: In line 1230 change "retention row. a `DELETE` fails" to "retention row. A `DELETE` fails".

### P-07b-6  HIGH   (fix F16, [DR-422])
- Problem: The DR-422 sentence says each deletion of an uploaded file is audited. Item 5 step 5 deletes the staged upload when a proposal fails, and item 9 says that failed proposal writes no event. The two rules cannot both hold, so the block does not carry this correction.
- Proof: docs/plan/phase-07.md:1021 "each deletion of an uploaded file is audited," and docs/plan/phase-07.md:1166 "A failure in step 4 deletes what step 3 wrote" with docs/plan/phase-07.md:1202 "The failed proposal of item 5 step 5 writes no event: no case row exists".
- Smallest fix: In line 1021 change "each deletion of an uploaded file is audited" to "each deletion of an uploaded file made by an intake run or the sweep is audited".

### P-07b-7  HIGH   (fix F13, [O-08-1])
- Problem: The F13 sentence says the approver rejects an unmet case. Phase 04 runs the subject check before every engine action, Approve or Reject, and the target hook has no action parameter. The check_before_decision row of item 7 refuses every decision when `met` is false, so as written the approver's rejection is refused with 422 E_VALIDATION and the task stays open. Phase 04 line 969 says check_before_decision refuses approval (for the bundle target), so the intent may be approval-only, but the block does not say so and a developer must guess.
- Proof: docs/plan/phase-07.md:1198 "the approver rejects the request or its maker withdraws it" and docs/plan/phase-07.md:1192 "an issue is a refusal, phase 04 rolls the action back"; docs/plan/phase-04.md:896 "`engine.act(task, Approve | Reject, if_match, assertion)`" and docs/plan/phase-04.md:652 "async fn check_before_decision(&self, tx: &mut Tx, cr: &ChangeRequest)".
- Smallest fix: In line 1192 add that the check_before_decision issues refuse an approval only and never a rejection (phase 04 section 3.2, line 897, must then skip the check for Reject), or else change line 1198 to "its maker withdraws the request".

## Summary
- Entries checked: 16 (F1 to F16, key 07b).
- Findings: 7 in total. HIGH 2, MEDIUM 1, LOW 4.
- CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 P-07b-1, F6 ok, F7 P-07b-2, F8 ok, F9 ok, F10 ok, F11 ok, F12 ok, F13 P-07b-7, F14 P-07b-3 and P-07b-4, F15 P-07b-5, F16 P-07b-6
