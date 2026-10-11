# Owner-decision check, round 2, key 10 (Haiku draft for lead review)

### P-10-1  MEDIUM   (fix F3, [O-10-1/6/7])
- Problem: The sentence F3 added to 7.26 item 5 says a stored name takes its new label "at the next load of its source row". Phase 11 lands and maps only changed rows, and a later mapping publication does not re-map history, so for an unchanged source row the next load changes nothing and only the mapping replay reaches the name (the O-10-1 WHY says the same). The sentence does not carry that limit.
- Proof: docs/plan/phase-10.md:2487 "A name already stored takes a new label when its row is mapped again: at the next load of its source row"; docs/plan/phase-11.md:792-793 "A row is `changed` when its key is new or its hash differs; otherwise it" / "`unchanged` and is not landed."; docs/plan/phase-11.md:849 "A later publication does not re-map history: each receipt keeps the version it ran under".
- Smallest fix: In docs/plan/phase-10.md line 2487, change "at the next load of its source row" to "at the next load of its source row when that row has changed (phase 11 section 3.2 step 2)".

### P-10-2  LOW   (fix F16, [DR-422])
- Problem: The DR-422 sentence was inserted between "This phase's parts:" and the numbered list of the 7.27 items, so the colon now introduces a sentence about DR-422 and the list has no lead-in.
- Proof: docs/plan/phase-10.md:2493 "This phase's parts: DR-422 locks the corrections of the check round of 2026-10-11"; the list begins at docs/plan/phase-10.md:2495 "1. **Entity.** Phase 26 adds `facility_instalment`".
- Smallest fix: Move the sentence "DR-422 locks the corrections of the check round of 2026-10-11: the payment columns and the grant date are compared as mapped values, and the index on `(facility_id, seq)` is not unique." to before "This phase's parts:", so that the colon is followed directly by the list.

## Summary

- Entries checked: 16 (F1 to F16).
- Findings: HIGH 0, MEDIUM 1, LOW 1.
- CHECKED: F1 ok, F2 ok, F3 P-10-1, F4 ok, F5 ok, F6 ok, F7 ok, F8 ok, F9 ok, F10 ok, F11 ok, F12 ok, F13 ok, F14 ok, F15 ok, F16 P-10-2
