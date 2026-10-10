# Re-check, phase 21 (Haiku draft for lead review)

### R-21-1  HIGH   (fix F23)
- Problem: The 7.8 block says the manifest holds the PDF hash. Section 3.8 builds the manifest in step 4, before the PDF is rendered in step 5, and step 5 prints the manifest hash on the PDF. The two statements cannot both hold: the manifest would depend on the PDF hash while the PDF depends on the manifest hash. The base comment on `rpt.report_packages` repeats the same wrong claim. The operative rule (the sign binds to `content_sha256`) is not affected.
- Proof: docs/plan/phase-21.md:1407 "The manifest holds the PDF hash and is built after the sign (section 3.8 steps 4 and 5)"; docs/plan/phase-21.md:881 "showing the report reference and the manifest hash on each page"; docs/plan/phase-21.md:157 "their hashes are in the manifest".
- Smallest fix: In 7.8 replace "The manifest holds the PDF hash" with "The manifest holds the XML and attachment hashes", and in the comment at line 157 replace "their hashes are in the manifest" with "the XML and attachment hashes are in the manifest".

### R-21-2  MEDIUM   (fix F4)
- Problem: The "What This Phase Does NOT Touch" trailer still allows an exception for "the owner methods of section 4.5 item 9". After F3 and F4, item 9 adds no owner method: the only phase 20 change is the port method `LetterRequester::delivered`, and 7.5 says no phase 20 method records the delivery. The exception is the old rule left behind.
- Proof: docs/plan/phase-21.md:1503 "No change to phase 20's freeze, refrain or ops-task tables or routes, except the owner methods of section 4.5 item 9" and docs/plan/phase-21.md:1389 "no phase 20 method records the delivery".
- Smallest fix: In the trailer at line 1503, delete ", except the owner methods of section 4.5 item 9, which phase 20's columns already allow".

### R-21-3  LOW   (fix F5)
- Problem: The comment above the report guard trigger still says the approval is written once. F5 now lets a revise from `rejected` or `returned` to `draft` clear the signature, so the comment is out of date.
- Proof: docs/plan/phase-21.md:417 "-- Reports: the approval and the frozen version are written once." and docs/plan/phase-21.md:426 "AND NOT (OLD.status IN ('rejected', 'returned') AND NEW.status = 'draft') THEN".
- Smallest fix: Replace the comment at line 417 with "-- Reports: the approval and the frozen version are written once per submission; a revise from rejected or returned clears the approval."

### R-21-4  LOW   (fix F27)
- Problem: F27 says the phase 04 engine cannot add the drafter and the authors to the excluded set, and that the subject carries the rule. Section 7.4 item 1 still says the excluded set adds them. The people refused are the same, so only the mechanism is stale, but a builder could still try to implement item 1 through the engine.
- Proof: docs/plan/phase-21.md:1374 "The excluded set of the `sign` task adds the report's `created_by` and the `created_by` of every version of the"; docs/plan/phase-21.md:1419 "the phase 04 engine computes `excluded` from the instance maker and the earlier deciders only".
- Smallest fix: In 7.4 item 1, replace "The excluded set of the `sign` task adds the report's" with "The sign subject refuses (section 7.12) the report's".

### R-21-5  LOW   (fix F21)
- Problem: The comment on `rpt.report_links` describes the escalated link only for an STR. Fix F21 also gives `escalated_from_utr` to a SAR escalated from a UTR, so the two statements differ.
- Proof: docs/plan/phase-21.md:172 "previous, amending, superseding and the STR escalated from a UTR (which cites its number)" and docs/plan/phase-21.md:1394 "when this report is an STR or a SAR and the related report is a UTR".
- Smallest fix: In the comment at line 172, replace "the STR escalated from a UTR" with "the STR or SAR escalated from a UTR", or limit the 7.6 rule to STR.

## Summary
- Fixes checked: 29 (F1 to F29).
- Findings: HIGH 1, MEDIUM 1, LOW 3 (5 in total).
- CHECKED: F1 ok, F2 ok, F3 ok, F4 R-21-2, F5 R-21-3, F6 ok, F7 ok, F8 ok, F9 ok, F10 ok, F11 ok, F12 ok, F13 ok, F14 ok, F15 ok, F16 ok, F17 ok, F18 ok, F19 ok, F20 ok, F21 R-21-5, F22 ok, F23 R-21-1, F24 ok, F25 ok, F26 ok, F27 R-21-4, F28 ok, F29 ok
