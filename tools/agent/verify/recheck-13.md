# Re-check, phase 13 (Haiku draft for lead review)

### R-13-1  MEDIUM   (fix F1)
- Problem: The fix names `EXCL_POOL_ACCOUNT` as the phase 16 static exclusion for the payroll pool account. That name occurs nowhere else in docs/plan. Phase 16 describes a static exclusion by its active `exclusion` reason code and defines no name for it.
- Proof: docs/plan/phase-13.md:27 "the exclusion of the payroll pool account is the phase 16 static exclusion `EXCL_POOL_ACCOUNT`"; docs/plan/phase-16.md:963 "a static exclusion carries an active `exclusion` reason code"
- Smallest fix: In phase-13.md:27, replace "is the phase 16 static exclusion `EXCL_POOL_ACCOUNT`" with the reason code and the exclusion row that phase 16 actually uses, or add `EXCL_POOL_ACCOUNT` to the phase 16 exclusion list.

### R-13-2  HIGH   (fix F9)
- Problem: The F9 seed expression for `distinct_counterparties_3m` takes its `field` from `txn.counterparty_id`, which phase 10 classes as `personal`. Section 3.1 step 2 refuses any CanonRef in `field` that is a class `personal` column, and the seed writes nothing when any feature fails validation. The two rules cannot both hold, so the nine-feature seed fails, and so does the peer build, which uses this feature by default.
- Proof: docs/plan/phase-13.md:1177 "`distinct_counterparties_3m` | `{"kind": "distinct_count", "field": "txn.counterparty_id", "where": "P"}`"; docs/plan/phase-13.md:535 "is not a class `personal`, `secret-str` or `card` column"; docs/plan/phase-10.md:1162 "('txn', 'personal', ARRAY['narrative_raw','narrative_norm','account_id','counterparty_id','document_key'])"; docs/plan/phase-13.md:592 "validate every feature; any failure exits 1 and writes nothing"
- Smallest fix: In docs/plan/phase-13.md:535 add an exception for a `txn.counterparty_id` that is the `field` of a `distinct_count` (a surrogate id, counted only), or move `counterparty_id` from the `personal` list to `internal` at docs/plan/phase-10.md:1162.

### R-13-3  MEDIUM   (fix F8)
- Problem: Step 3 of the cluster assignment writes rows only for a customer whose open cluster row of the currency "names another segment", and says that an unchanged customer writes nothing. A customer with no open cluster row of that currency matches neither clause, so the literal step writes no membership for them. At the first build of a currency there are no open rows, so that build would activate with no memberships at all. Step 4 does not cover this case either.
- Proof: docs/plan/phase-13.md:1165 "a customer whose open cluster row of the build's currency names another segment has that row closed"; docs/plan/phase-13.md:1165 "an unchanged customer writes nothing"
- Smallest fix: In docs/plan/phase-13.md:1165 add after "names another segment" the words "or has no open row (then only the new row is opened)".

### R-13-4  MEDIUM   (fix F18)
- Problem: The fix gives the sub-code `too_long` and says the sub-codes come from the phase 11 `E_VALIDATION` row. That row lists `range_too_long`, not `too_long`, so the code the fix names is not in the row it cites. A developer would have to choose between the two names.
- Proof: docs/plan/phase-13.md:758 "A longer range is 422 `E_VALIDATION` at `/from` with the sub-code `too_long`"; docs/plan/phase-11.md:1173 "`range_inverted`, `range_too_long`, `table_not_in_version`"
- Smallest fix: In docs/plan/phase-13.md:758 replace the sub-code `too_long` with `range_too_long`.

### R-13-5  LOW   (fix F5)
- Problem: The last sentence of the `active_by_level` paragraph reads as if `active_static` serves risk overlays: "it" points at `active_static`. Section 3.3 step 1 names no method; it loads the segments of a level, which is `active_by_level`.
- Proof: docs/plan/phase-13.md:1150 "`active_static` is its `static` case; section 3.3 step 1 uses it for `risk_overlay`."
- Smallest fix: In docs/plan/phase-13.md:1150 replace "section 3.3 step 1 uses it for `risk_overlay`" with "section 3.3 step 1 calls `active_by_level` for `risk_overlay`".

### R-13-6  LOW   (fix F22)
- Problem: The fix gives the plan signature `plan(expected, combine, min_history)` but the profile_refresh call names only the comparators and `min_history`. The value of `combine` for profile_refresh is not given. Phase 16 defines `combine` as `any` or `all`, and section 3.6 step 8 implies `all` for `complete`, but the fix does not say which value applies.
- Proof: docs/plan/phase-13.md:1199 "`profile_refresh` calls both with the fixed plan of section 3.6: declared, own history, peer, with `min_history`"; docs/plan/phase-16.md:958 "`combine` is `any` or `all`"
- Smallest fix: In docs/plan/phase-13.md:1199 add "and `combine` = `all`" after "peer," (or say that profile_refresh does not use `combine`).

## Summary
- Fixes checked: 31 (F1 to F31, from tools/agent/verify/applied-13.txt).
- Findings: HIGH 1, MEDIUM 3, LOW 2 (6 in total).
- CHECKED: F1 R-13-1, F2 ok, F3 ok, F4 ok, F5 R-13-5, F6 ok, F7 ok, F8 R-13-3, F9 R-13-2, F10 ok, F11 ok, F12 ok, F13 ok, F14 ok, F15 ok, F16 ok, F17 ok, F18 R-13-4, F19 ok, F20 ok, F21 ok, F22 R-13-6, F23 ok, F24 ok, F25 ok, F26 ok, F27 ok, F28 ok, F29 ok, F30 ok, F31 ok
