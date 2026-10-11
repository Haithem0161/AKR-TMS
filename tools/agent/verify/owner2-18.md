# Owner-decision check, round 2, key 18 (Haiku draft for lead review)

## Summary

Entries checked: 4 (F1, F2, F3, F4). Findings: HIGH 0, MEDIUM 0, LOW 0.

Evidence for the ok entries (all in docs/plan unless a path is given):
- F1 (O-18-1): phase-18.md line 1509 holds the NEW text. Section 3.14 has two routes, and step 5 of `GET /parties/{id}/transactions` (line 882) lists `account_id` and `counterparty_id`. Step 5 of `GET /parties/{id}` (line 873) lists cases, so the old unqualified reference was ambiguous and the naming fixes it.
- F2 (O-18-1-note): phase-18.md line 1509. Section 3.10 (lines 799 to 823) and steps 1 and 2 of `GET /parties/{id}` (lines 869 and 870, repeated at line 878) make the transaction rows visible only to callers who pass the party tier rules. Otherwise the answer is a 404. So the copied UUID is always in a response the caller already holds. `counterparty_id` and `account_id` are UUID columns of canon.txn (phase-10.md lines 956 and 978).
- F3 (O-19-16): phase-18.md line 582 holds the NEW text. phase-19.md line 2068 (item 4 of section 7.21, which begins at line 1980) and line 2083 add `current_customers(conn, party_id)` and `co_holders(conn, account_ids)` to CanonPartyReader. The signatures agree. Lines 2045 to 2052 name the same methods in the registry table.
- F4 (L-1, under the L1 role rule): phase-18.md line 1246 holds the NEW text. The only GRANTs of these tables to akr_app are phase-18.md line 436 (`evidence_items`, SELECT, INSERT) and line 439 (`opened_views`, SELECT, INSERT). No other phase grants either table to akr_app, and there is no ALTER DEFAULT PRIVILEGES (phase-00.md line 43). The triggers are at phase-18.md lines 297 and 300, and they reuse casework.append_only() (phase-17.md line 413; it raises '% is append-only' at line 415). The owner akr_migrate reaches that trigger. The NEW text does not quote the message, which matches the wording "(the guard raises)" in the same item 9 (line 1246), so no message is wrong.

CHECKED: F1 ok, F2 ok, F3 ok, F4 ok

FINDINGS: 0
