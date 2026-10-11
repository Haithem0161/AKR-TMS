# Owner-decision round, phase 18 (Haiku draft for lead review)

### O-18-1  LOW   (section 7.29, check 3)
- Problem: The pointer "section 3.14 step 5" does not name a route. Section 3.14 has a step 5 under each of its two routes. Under `GET /parties/{id}` step 5 lists the party's cases; the transaction row with the two ids is step 5 of `GET /parties/{id}/transactions`. A reader who searches for "step 5" meets the cases step first.
- Proof: docs/plan/phase-18.md:1509 "They copy the `account_id` and `counterparty_id` that the row already returns (section 3.14 step 5)"; docs/plan/phase-18.md:873 "5. Cases: the party's cases the caller may see"; docs/plan/phase-18.md:882 "5. Row: `txn_id`, `booking_date`, `value_date`, `amount`, `currency`, `dr_cr`, `txn_type`, `status`, the channel and branch names, `account_id`, `counterparty_id`."
- Smallest fix: In 7.29 change "(section 3.14 step 5)" to "(section 3.14 step 5 of `GET /parties/{id}/transactions`)".

## Summary

Blocks checked: the dated header paragraph (line 1503), 7.28 (lines 1505 to 1506) and 7.29 (lines 1508 to 1509). The diff is one pure insertion (`@@ -1502,0 +1503,8 @@`), so changed base places in phase 18: 0.

Base places read for the new text: section 2 (lines 569 and 582 to 584); section 3.10; section 3.14 (lines 866 to 884); sections 5.2, 5.3 and 5.6; Open doubts 18, 20 and 26 (lines 1566, 1568, 1574); phase 10 DDL of `canon.party_name` (lines 495 to 520) and `canon.txn` (lines 953 to 980) and section 7.26 (line 2473); phase 08 section 3.7 row `Name` (line 338) and section 7.10 (line 697); phase 13 section 7.17 (line 1215) items 12 to 14; research.md DR-416 (line 427) and DR-420 (line 431).

Findings by severity: HIGH 0, MEDIUM 0, LOW 1 (O-18-1).

CHECKED: header paragraph (line 1503) checks 1 to 6 ok.
CHECKED: 7.28 checks 1 to 6 ok.
CHECKED: 7.29 checks 1 to 6, finding O-18-1 (check 3).

Answers to the points the lead asked:
- Names shape: section 3.14 states no field list for name rows. Step 3 (line 871) gives the predicate `superseded_at IS NULL` and the order `script = 'ar'` first, and section 2 names `PartyPage` with no fields (line 569). The list in 7.28 is therefore the only shape stated in phase 18, and no line contradicts it. Its five fields are all columns of `canon.party_name` (phase 10 DDL, lines 495 to 520). Line 584 gives the party reader's predicate and no column list, so the new column needs no reader change.
- Transaction row ids: step 5 of `GET /parties/{id}/transactions` (line 882) returns `account_id` and `counterparty_id`. Both are columns of `canon.txn` (phase 10 lines 955 and 977). `canon.txn.account_id` carries the id that canonical versions of an account share (phase 10 line 48), which is the subject id that phase 13 section 7.17 names (line 1227). `counterparty_id` is nullable (line 977), so the "absent when null" rule can occur.
- Capture source: `canon.party_name.capture_source` is `NOT NULL DEFAULT 'typed'` with the four values of 7.28 (phase 10 line 506; section 7.26 item 1).

Not counted (questions for the lead, not rule conflicts):
- "its row menu" (7.29): section 5.3 (lines 1170 to 1175) defines no row menu, and Open doubts 26 (line 1574) leaves the W4 and W5 specifications to the design owner. DESIGN-SYSTEM lists menus as a surface type (lines 323 and 326), so the term can be built and is not counted.
- Clipboard (7.29 and 5.6): 7.29 does not say whether the two actions show on rows of a suspicion-tier party. Section 5.6 (line 1195) says "nothing of a suspicion-tier row stays after the page closes", while a copied id stays on the OS clipboard after the page closes. The literal scope of 5.6 is the browser cache, so this is not counted; one line in 7.29 would settle it.
- DR-416 rationale (research.md line 427): it says "copying the hashes from canonical rows", but the same row's decision text says "for the ids", and phase 13 section 7.17 item 13 and 7.29 copy ids. The block follows the decision text and phase 13; the rationale wording is for the lead to correct in research.md.

FINDINGS: 1 (O-18-1)
