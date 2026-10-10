# Re-check, phase 25 (Haiku draft for lead review)

### R-25-1  MEDIUM   (fix F2)
- Problem: The SP-36 entries get the fields window and threshold, but no value is given for any of the five rule groups the fix names, and research 30 section 9.3 gives no window or threshold either, so the shipped file cannot be written from this text. "Key operations" also names no exported kind: the signing-key kinds of phase 02 are not in its SIEM flag set, so a developer must choose the kinds.
- Proof: docs/plan/phase-25.md:1429 "as entries `{id, title, kinds, group_by: actor, window, threshold}`" and docs/plan/phase-25.md:1431 "and privileged bursts (role grants, policy activation, key operations)".
- Smallest fix: Give a window and a threshold for each rule group in the entries (or write that the bank sets them in its SIEM), and replace "key operations" in line 1431 by the exported kinds it means (for example `service_client.rotated`).

### R-25-2  LOW   (fix F10)
- Problem: One type name, SweepCounts, is given three different field sets. The access review sweep, whose signature also returns SweepCounts, has its counts only in section 3.1 step 3 and is missing from the list.
- Proof: docs/plan/phase-25.md:1444 "`SweepCounts` of the break-glass sweep is `{ lapsed, expired, review_overdue }`" and docs/plan/phase-25.md:749 "Return counts `{ superseded, overdue_opened }`." (signature at docs/plan/phase-25.md:710 "-> Result<SweepCounts, PlatformError>; // job access_review_sweep").
- Smallest fix: In 7.14 name one counts type per sweep (for example `AccessReviewSweepCounts { superseded, overdue_opened }`) and use it in the four sweep signatures at lines 710, 764, 925 and 926.

### R-25-3  LOW   (fix F15)
- Problem: The sentence added by F15 breaks off. The clause "as the catalogue name is:" is left over and the colon introduces an unrelated sentence.
- Proof: docs/plan/phase-25.md:846 "as the catalogue name is: `audit.siem_page` returns no severity column."
- Smallest fix: Replace ", as the catalogue name is:" with a full stop, so that "`audit.siem_page` returns no severity column." is its own sentence after "row's kind".

### R-25-4  LOW   (fix F19)
- Problem: The stated outcome is wrong. The tile counts dead letters created in the last 7 days, so it stays above zero for seven days after the last dead letter and does not clear once the cause is fixed.
- Proof: docs/plan/phase-25.md:1451 "so it clears once the cause is fixed" and docs/plan/phase-25.md:967 "count of `siem_dead_letters` created in the last 7 days".
- Smallest fix: Replace "so it clears once the cause is fixed" in line 1451 with "so it clears seven days after the last dead letter".

### R-25-5  MEDIUM   (fix F20)
- Problem: SiemStatus still names the old open-row count (`dead_letters_open`), which F20 replaced. The SIEM tiles read `status()`, so the tile count now comes from a field named for the superseded rule. The metric `akr_siem_dead_letters_open` has the same name.
- Proof: docs/plan/phase-25.md:663 "status() -> SiemStatus { lag_s, dead_letters_open, last_forwarded_at }"; docs/plan/phase-25.md:1459 "the SIEM tiles read `SiemRepository::status()`"; docs/plan/phase-25.md:1252 "`akr_siem_dead_letters_open`".
- Smallest fix: Rename the field to `dead_letters_7d` in line 663 and the metric to `akr_siem_dead_letters_7d` in line 1252, and define it as the count of rows created in the last 7 days.

### R-25-6  LOW   (fix F24)
- Problem: `invalidate_user` is defined by phase 01 (the session service), and phase 03 only calls it, so the attribution to phase 03 is wrong.
- Proof: docs/plan/phase-25.md:944 "after the commit phase 03's `invalidate_user`" and docs/plan/phase-01.md:605 "`invalidate_user(user_id)` =".
- Smallest fix: Change "after the commit phase 03's `invalidate_user`" in line 944 to "after the commit phase 01's `invalidate_user`".

### R-25-7  MEDIUM   (fix F42)
- Problem: F42 gives the critical cell "verified, proposed or approved", but the warn cell of the same row and the Currency paragraph of section 3.6 still say "verified or approved", so a proposed release is critical but never a warning. The clock of "waited unapplied" is not named, and the releases table has `verified_at` and `imported_at` but no `approved_at`.
- Proof: docs/plan/phase-25.md:968 "a newer verified, proposed or approved release has waited unapplied for more than 30 days" and docs/plan/phase-25.md:903 "the existence of a newer `verified` or `approved` release that is not yet applied".
- Smallest fix: In the warn cell of line 968 and in line 903, use "verified, proposed or approved", and name the clock of the 30 days (for example `verified_at`).

### R-25-8  MEDIUM   (fix F48)
- Problem: Section 3.9 still runs the article check inside the step 5 transaction, as part of `import`, and says it refuses. F48 moves the check before step 5 with the release ending `refused`. Read together, a bad article would roll back step 5 instead of ending the job as refused.
- Proof: docs/plan/phase-25.md:960 "runs inside the verification transaction of section 3.6 (step 5)" and docs/plan/phase-25.md:1471 "The verify job checks every article before step 5".
- Smallest fix: In section 3.9 change "runs inside the verification transaction of section 3.6 (step 5)" to "runs at step 5, after the article check of section 7.20 has passed", and drop the check and refusal clauses from that sentence.

### R-25-9  MEDIUM   (fix F54)
- Problem: The upload is specified as `EvidenceStore::stage` with no content type. `rollout.json` is JSON, and phase 02 step 2 refuses a declared type that is not in `evidence.allowed_types`, a governed setting whose default list has no `application/json`. On a default install a JSON upload is refused with `TypeNotAllowed`, and the fix does not say which type to declare.
- Proof: docs/plan/phase-25.md:1478 "Upload the `rollout.json` of the apply (multipart, `EvidenceStore::stage`)"; docs/plan/phase-02.md:843 "the declared type must be in `evidence.allowed_types`"; docs/plan/phase-02.md:507 "\"text/plain\", \"text/csv\", \"message/rfc822\"".
- Smallest fix: Add "content type `text/plain`" to the row in line 1478 (a default type that phase 02 sniffs as UTF-8 text), or state that `application/json` is added to `evidence.allowed_types` by change request and to the phase 02 sniffer.

### R-25-10  LOW   (fix F54)
- Problem: The new row sits in the 7.22 block, not in the section 2 route table that the block says it gains a row in, so the table and its counts are stale (21 rows, 21 routes, 9 added).
- Proof: docs/plan/phase-25.md:1477 "The route table of section 2 gains one row:"; docs/plan/phase-25.md:693 "**Route count**: 21 rows"; docs/plan/phase-25.md:8 "**Complexity:** M (21 routes:".
- Smallest fix: Insert the 7.22 row into the section 2 table after the `applied` row, and change the counts in line 8 and line 693 to 22 routes and 10 added.

## Summary
- Fixes checked: 57 (F1 to F57), all with the five checks of p-recheck.txt.
- Findings: HIGH 0, MEDIUM 5 (R-25-1, R-25-5, R-25-7, R-25-8, R-25-9), LOW 5 (R-25-2, R-25-3, R-25-4, R-25-6, R-25-10).
- CHECKED: F1 ok, F2 R-25-1, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok, F8 ok, F9 ok, F10 R-25-2, F11 ok, F12 ok, F13 ok, F14 ok, F15 R-25-3, F16 ok, F17 ok, F18 ok, F19 R-25-4, F20 R-25-5, F21 ok, F22 ok, F23 ok, F24 R-25-6, F25 ok, F26 ok, F27 ok, F28 ok, F29 ok, F30 ok, F31 ok, F32 ok, F33 ok, F34 ok, F35 ok, F36 ok, F37 ok, F38 ok, F39 ok, F40 ok, F41 ok, F42 R-25-7, F43 ok, F44 ok, F45 ok, F46 ok, F47 ok, F48 R-25-8, F49 ok, F50 ok, F51 ok, F52 ok, F53 ok, F54 R-25-9 and R-25-10, F55 ok, F56 ok, F57 ok.
