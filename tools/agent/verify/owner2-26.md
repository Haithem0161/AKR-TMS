# Owner-decision check, round 2, key 26 (Haiku draft for lead review)

### P-26-1  MEDIUM   (fix F7, O-26-2a)
- Problem: The text says "The three inputs marked `?`", but the SCN-ISL-07 row marks four fields with `?`: `facility.granted_on?` and the three `facility_instalment` fields. The count is wrong. "The three are listed in the `reasons`" is also wrong when only `facility.granted_on` is missing, because then one input is listed, not three.
- Proof: docs/plan/phase-26.md:513 "`facility.granted_on?`, `product.islamic_contract_type`, `facility_instalment.facility_id?`, `facility_instalment.due_on?`, `facility_instalment.paid_on?`"; docs/plan/phase-26.md:1075 "The three inputs marked `?` are optional inputs (DSL.md, CanonicalRef)".
- Smallest fix: In docs/plan/phase-26.md line 1075 replace "The three inputs marked `?`" with "The four inputs marked `?` (`facility.granted_on` and the three `facility_instalment` fields)", and replace "the three are listed in the `reasons`" with "the missing ones are listed in the `reasons`". The "three optional inputs" wording of F9 can stay, since it names the three family fields.

### P-26-2  LOW   (fix F14, O-26-4)
- Problem: "has a `admin.column_classification` row" uses "a" before a word that starts with a vowel sound. The article sits in the replaced sentence of check 2.
- Proof: docs/plan/phase-26.md:861 "has a `admin.column_classification` row (a query over `information_schema.columns`".
- Smallest fix: In docs/plan/phase-26.md line 861 change "has a `admin.column_classification` row" to "has an `admin.column_classification` row".

### P-26-3  MEDIUM   (fix F17, O-26-1b)
- Problem: Open doubts item 24 says that when a source changes an unpaid instalment in place, "the payment still lands, and a changed due date or amount is recorded as a conflict". Phase 10 section 7.27 item 3, which item 24 cites, says that in a version that also moves the payment, a changed `seq`, `due_on`, `amount` or `currency` is neither written nor reported. The conflict applies only when the payment columns did not move. The two statements cannot both hold for one version. The build is not blocked, because the writer follows phase 10 section 7.27 item 3 (its test in item 7 covers only the separate change). Rated MEDIUM as an open-doubt line; if the lead treats item 24 as a rule, it is HIGH (two rules that cannot both hold).
- Proof: docs/plan/phase-26.md:1138 "the payment still lands, and a changed due date or amount is recorded as a conflict (phase 10 section 7.27 item 3)"; docs/plan/phase-10.md:2497 "a difference in `seq`, `due_on`, `amount` or `currency` that arrives in the same version is not written and not reported".
- Smallest fix: In docs/plan/phase-26.md item 24 (line 1138) replace "a changed due date or amount is recorded as a conflict (phase 10 section 7.27 item 3)" with "a changed due date or amount with no payment change is recorded as a conflict; in a version that also moves the payment, phase 10 section 7.27 item 3 does not report it".

### P-26-4  LOW   (fix F20, DR-422)
- Problem: The sentence says DR-422 locks "what the grader does with the optional inputs". The DR-422 row in research.md has six numbered items, and none of them covers the grader or the optional inputs (the row contains none of the words "optional", "grader", "readiness" or "reasons"). The block does carry the rule (7.13 item 5), so the fault is only the attribution to DR-422.
- Proof: docs/plan/phase-26.md:1007 "and what the grader does with the optional inputs"; docs/plan/research.md:433 "the index on `(facility_id, seq)` is not unique; `paid_amount` may be set while `paid_on` is null" (the row's other items cover phases 06 to 25 and do not mention the grader).
- Smallest fix: Either delete ", and what the grader does with the optional inputs" from docs/plan/phase-26.md line 1007, or add a matching numbered item to the DR-422 row in docs/plan/research.md line 433.

## Summary
- Entries checked: 20 (F1 to F20), all with the six checks of the instruction, plus the extra rules for L1 (not applicable), DR (not applicable) and DR-422 (F20).
- Findings: HIGH 0, MEDIUM 2, LOW 2 (4 in total).
- CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 P-26-1, F8 ok, F9 ok, F10 ok, F11 ok, F12 ok, F13 ok, F14 P-26-2, F15 ok, F16 ok, F17 P-26-3, F18 ok, F19 ok, F20 P-26-4

Notes (not findings, not counted):
- docs/DATA-MODEL.md line 830 says "where it does not, the scenario is declared not covered". For a schedule bank without the family, the F7 and F8 wording (grade B, facility legs not evaluable, no grade change) contradicts it. DATA-MODEL is outside the same-file rule and line 830 is not a replacement, so it is not counted.
- docs/DATA-MODEL.md line 889 says `akr_ingest` may UPDATE only a list that does not include the DR-419 updates stated at line 958 (F19). Not counted: DR-419 is cited at line 958 for those grants, and a DR wins over base text. The list may still be updated.
- Side effect to clean up: a scratch file /tmp/dr422-row.txt was created by mistake (one copied line of docs/plan/research.md). It is not used, it is outside the write set, and it can be deleted.
