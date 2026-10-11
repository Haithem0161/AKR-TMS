# Owner-decision check, round 3, key 26 (Haiku draft for lead review)

### Q-26-1  HIGH   (fix F6, [L2-4])
- Problem: The sentence that F6 edits still says that no role has DELETE on `canon` except `akr_retention` on the party tree, and the pointer F6 adds covers only "the updates". Phase 26 gives `akr_retention` DELETE on `canon.islamic_deals`, `canon.deal_parties` and `canon.facility_instalments`, and the phase 26 paragraph of the same file says so. The three tables are not in the phase 23 party-tree list. The two rules cannot both hold.
- Proof: docs/DATA-MODEL.md:891-892 "no role has DELETE on `canon` except `akr_retention` on the party tree (phase 23; partitions are dropped only by"; docs/DATA-MODEL.md:958 "`akr_retention` has SELECT and DELETE on the three tables" (and, for `canon.facility_instalments`, "`akr_retention` has SELECT and DELETE; `akr_readonly` none (phase-26 section 7.13)"); docs/plan/phase-26.md:118 "GRANT SELECT, DELETE ON canon.islamic_deals, canon.deal_parties TO akr_retention;"; docs/plan/phase-26.md:1045 "GRANT SELECT, DELETE ON canon.facility_instalments TO akr_retention;"; the party-tree list at docs/plan/phase-23.md:374-376 names `canon.party` through `canon.kyc_document` and none of the three tables.
- Smallest fix: in docs/DATA-MODEL.md line 891, add "and on `canon.islamic_deals`, `canon.deal_parties` and `canon.facility_instalments` (phase 26 section 1.5)" after "`akr_retention` on the party tree", change the pointer to "the updates and the DELETE grants named in its own paragraph below".

### Q-26-2  LOW   (fix F6, [L2-4])
- Problem: The pointer "(phase 26 adds the updates named in its own paragraph below)" follows the `payment_message` clause, so it reads as if phase 26 adds the `status` and `status_history` updates of `payment_message`. Those updates are named by phase 10 section 7.18. Phase 26 names no `payment_message` table or update.
- Proof: docs/DATA-MODEL.md:891 "and `status` and `status_history` of `payment_message` (phase 26 adds the updates named in its own paragraph below)"; docs/plan/phase-10.md:2447 "`akr_ingest` may update the two columns" (section 7.18, heading at line 2446); no line of docs/plan/phase-26.md contains `payment_message`.
- Smallest fix: in docs/DATA-MODEL.md, move "(phase 26 adds the updates named in its own paragraph below)" from after `payment_message` on line 891 to directly after "`akr_ingest` may UPDATE only" on line 889.

## Summary
- Entries checked: 6 (F1 to F6).
- Findings: HIGH 1, MEDIUM 0, LOW 1 (2 findings in total).
- CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 Q-26-1, Q-26-2

