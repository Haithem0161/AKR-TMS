# Re-check, phase 23 (Haiku draft for lead review)

### R-23-1  MEDIUM   (fix F16)
- Problem: The F16 text gives no party path for `facility`. Section 3.2 (R3 row) puts `facility` on the record-purge list, and section 3.5 says every table of that list has a party path. `canon.facility` has `customer_id` (phase 10 DDL, line 1311), not `account_id`, so its path is through `customer_id` and `canon.customer.party_id`, as `trade_instrument` has. Without a path a facility row is never purged by this phase.
- Proof: docs/plan/phase-23.md:647 "`card` (through `account_id`), `facility`, `trade_instrument` (through `customer_id` and `canon.customer.party_id`)"; second quote docs/plan/phase-23.md:590 "the party family, `customer`, `account` (`closed_on`), `account_party_role`, `account_hold`, `card`, `facility`, `trade_instrument`"
- Smallest fix: In line 647 change "`facility`, `trade_instrument` (through" to "`facility` (through `customer_id` and `canon.customer.party_id`), `trade_instrument` (through", so that facility gets its own path.

### R-23-2  LOW   (fix F42)
- Problem: Section 3.4 still lists `feat.feature_runs` among the droppable parents whose drop is blocked while a party or case hold is active. F21 removed the table from the drop allow-list and 7.10 makes it a record purge by age, so the sentence describes a drop that cannot happen. 7.10 does not say whether an active hold blocks the record purge.
- Proof: docs/plan/phase-23.md:631 "`feat.feature_runs` and `feat.segment_stats` have no party path in this phase: no copy can be made, so their drop is blocked (`copy_failed`)"; second quote docs/plan/phase-23.md:1054 "Its R4 handler is `record_purge` by age on `as_of`, in batches, as for `admin.quarantine_items`."
- Smallest fix: In line 631 change "`feat.feature_runs` and `feat.segment_stats` have no party path" to "`feat.segment_stats` has no party path", and add one sentence to 7.10 saying whether an active party or case hold blocks the record purge of `feat.feature_runs`.

## Summary
- Fixes checked: 44 (F1 to F44, from tools/agent/verify/applied-23.txt).
- Findings: HIGH 0, MEDIUM 1, LOW 1.
- CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok, F8 ok, F9 ok, F10 ok, F11 ok, F12 ok, F13 ok, F14 ok, F15 ok, F16 R-23-1, F17 ok, F18 ok, F19 ok, F20 ok, F21 ok, F22 ok, F23 ok, F24 ok, F25 ok, F26 ok, F27 ok, F28 ok, F29 ok, F30 ok, F31 ok, F32 ok, F33 ok, F34 ok, F35 ok, F36 ok, F37 ok, F38 ok, F39 ok, F40 ok, F41 ok, F42 R-23-2, F43 ok, F44 ok
