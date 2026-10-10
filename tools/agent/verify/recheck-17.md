# Re-check, phase 17 (Haiku draft for lead review)

### R-17-1  MEDIUM   (fix F15)
- Problem: F15 says `revalidated_at` is set by the retry job, but no retry step writes it. The retry steps in section 3.2 and in 7.19 set only `reason_detail.valid_now` and the audit event, and only for a valid row, so a developer cannot tell whether still-invalid rows are stamped.
- Proof: docs/plan/phase-17.md:948 "`revalidated_at` is set by the retry job and by step 4 only"; docs/plan/phase-17.md:1937 "When the row is valid, set `reason_detail.valid_now = true` and write audit `alert.quarantine_revalidated` in that transaction."
- Smallest fix: In the 7.19 retry sentence at line 1937, add "and set `revalidated_at = now()`" and state whether still-invalid rows get it too.

### R-17-2  MEDIUM   (fix F20)
- Problem: Step 1 of 7.22 tests only a user target of a case (a member of a pool). `AssignCase` also accepts `to: { pool_id }`, and section 3.8 step 4 allows `assignee_pool_id`, but no line says what a pool target must be. A case can therefore be given to any pool with no test and no 422 at `/to`, and the members of that pool then see the case (section 3.10 rule 5).
- Proof: docs/plan/phase-17.md:1950 "For a case, the target is a member of the case's `assignee_pool_id`, or of the pool of an active `alerts` queue"; docs/plan/phase-17.md:793 "`AssignCase { to: { user_id } | { pool_id }, reason_code?: String }`".
- Smallest fix: In 7.22 step 1, add "For a case with `to: { pool_id }`, the pool must be the case's `assignee_pool_id` or the pool of an active `alerts` queue whose filter admits the case's family and class; else 422 `E_VALIDATION` at `/to`."

### R-17-3  MEDIUM   (fix F21)
- Problem: 7.23 says its checks are the complete list, in order. For `mark_duplicate` they omit the rule that section 3.8 gives triage: the target must be an alert of the same case and must not be the item itself. A bulk `mark_duplicate` with one `duplicate_of` can therefore link alerts of different cases, and the closed reason list has no reason for that refusal.
- Proof: docs/plan/phase-17.md:1153 "`duplicate_of` must be an alert of the same case, different from this one"; docs/plan/phase-17.md:1960 "5. `mark_duplicate`: a P0 or P1 alert is `p0_or_p1_duplicate`; a target in state `duplicate` is `target_is_duplicate` (7.11)."
- Smallest fix: In 7.23 check 5, add "a target that is not an alert of the same case, or is the item itself, is `not_same_case`", and add `not_same_case` to the closed list on line 1963.

### R-17-4  LOW   (fix F21)
- Problem: `lower_refused` is in the closed bulk reason list, but no check in 7.23 can produce it, because bulk actions are `assign` and `mark_duplicate` only. It belongs to the single triage `raise_class` of section 3.8.
- Proof: docs/plan/phase-17.md:1963 "`conflict_refused`, `lower_refused`."; docs/plan/phase-17.md:1207 "`action` is `assign` or `mark_duplicate` only."
- Smallest fix: In the 7.23 closed list on line 1963, delete `lower_refused`.

### R-17-5  HIGH   (fix F25)
- Problem: 7.25 bypasses phase 18 `grant` and says its steps 3 to 5 are for manual grants. Steps 3 and 4 apply to every grant: `decide` needs A2 or M, and an active grant of the same principal is refused. So the assignment writes `decide` for a second worker of any role that holds `queue.work`, and A1 holds it (section 3.8 step 1 and the scope note). Phase 18 forbids `decide` for A1.
- Proof: docs/plan/phase-17.md:1970 "their steps 3 to 5 are for manual grants"; docs/plan/phase-18.md:656 "`access = decide` requires the principal to hold `A2` or `M`"; docs/plan/phase-17.md:825 "`queue.work` admits A1, A2, M".
- Smallest fix: In 7.25, replace "their steps 3 to 5 are for manual grants" with "step 5 is for manual grants only; AssignmentService applies steps 3 and 4 (the decide role check and the active-grant refusal) to each row it writes".

### R-17-6  HIGH   (fix F25)
- Problem: The insert in 7.25 has no test for an active grant of the same principal on the case. The partial unique index `uq_case_acl_active` allows one active row per case and principal, so an assignee who already holds a grant (a manual grant from phase 18 `POST /cases/{id}/access`, or the `decide` grant that every M holder gets at escalation) makes the assignment transaction fail with a unique violation, not with the 409 `grant_exists` of phase 18 step 4.
- Proof: docs/plan/phase-17.md:1970 "inserts and revokes the `casework.case_acl` rows itself, in the assignment transaction"; docs/plan/phase-18.md:102 "CREATE UNIQUE INDEX IF NOT EXISTS uq_case_acl_active ON casework.case_acl (case_id, principal_id) WHERE revoked_at IS NULL;" (phase 18 step 4, line 657: "An active grant of the same principal exists -> 409").
- Smallest fix: In 7.25, add before the insert "if the assignee already holds an active grant on the case, the assignment refuses with 409 `E_NOT_ALLOWED_IN_STATE`, sub-code `grant_exists`".

### R-17-7  LOW   (fix F25)
- Problem: "under the `case.assign` check of section 3.8" points to a section that names no Cedar check. `case.assign` is defined in section 3.17 and is named in the section 2 route row for `POST /cases/{id}/assign`.
- Proof: docs/plan/phase-17.md:1970 "under the `case.assign` check of section 3.8"; docs/plan/phase-17.md:1402 `Akr::Action::"case.assign"` (section 3.17).
- Smallest fix: In 7.25, change "under the `case.assign` check of section 3.8" to "under the `case.assign` check of section 3.17".

## Summary
- Fixes checked: 28 (F1 to F28 of tools/agent/verify/applied-17.txt; 12 blocks and 16 replacements).
- Findings: HIGH 2, MEDIUM 3, LOW 2 (7 in total).
- CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok, F8 ok, F9 ok, F10 ok, F11 ok, F12 ok, F13 ok, F14 ok, F15 R-17-1, F16 ok, F17 ok, F18 ok, F19 ok, F20 R-17-2, F21 R-17-3 R-17-4, F22 ok, F23 ok, F24 ok, F25 R-17-5 R-17-6 R-17-7, F26 ok, F27 ok, F28 ok
