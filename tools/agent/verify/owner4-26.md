# Owner-decision check, round 4, key 26 (Haiku draft for lead review)

## Summary

Entries checked: 1 (F1, docs/DATA-MODEL.md lines 891 and 892).
Findings: HIGH 0, MEDIUM 0, LOW 0.

Basis: each item of the sentence was checked against the GRANT statements. UPDATE for akr_ingest on canon: phase 10 lines 773 to 774, 775, 778, 779 and 1151, the account_hold grant that section 7.19 adds (line 2451), and phase 26 lines 114, 115, 150, 151, 1042 and 1043. DELETE on canon: phase 23 lines 374 to 381 (the party tree; the batch list at phase 23 line 654 defines it to include card, facility, trade_instrument and kyc_document), and phase 26 lines 118, 153 and 1045. The phase 26 paragraph at DATA-MODEL.md line 958 agrees with the new sentence. No other line of DATA-MODEL.md states a different rule for canon.

Note, not counted: the UPDATE grant on canon.account_hold is not in the section 1.2 SQL (phase 10 lines 364 to 849). It exists only in the section 7.19 text (line 2451: "Migration `0030` (section 1.2) gains ..."). The sentence cites 7.19, so it is right as written. A builder who reads section 1.2 alone would miss the grant.

CHECKED: F1 ok

FINDINGS: 0
