# Re-check, phase 15 (Haiku draft for lead review)

### R-15-1  MEDIUM   (fix F2)
- Problem: The exit register row has two possible sources for party_id. F2 freezes `party_id` in the exit_decision context, but says only `review_id` is read from the context and that `NewExitEntry` keeps its fields (its body carries `party_id`). Nothing says the register party is the frozen party or that a different body `party_id` is refused, so an exit could be recorded against a party other than the one decided.
- Proof: docs/plan/phase-15.md:2175 "freezes `review_id` (the completed review whose outcome `exit_candidate` opened the instance, section 3.4 step 7 sub-step 6) and `party_id`"; docs/plan/phase-15.md:2176 "never from the body; `NewExitEntry` keeps its fields"; docs/plan/phase-15.md:940 "`NewExitEntry { party_id,"
- Smallest fix: In 7.13 add that the register's `party_id` is read from the instance context the same way as `review_id`, or that a body `party_id` differing from the frozen one is refused with 422 `E_VALIDATION`.

### R-15-2  LOW   (fix F2)
- Problem: No resource id is given for the exit_decision instance. `start_context(conn, id)` receives that id and F2 says it freezes `review_id`, but nothing says which id the instance is started with. Phase 15 gives no StartSpec for it; the comparable subjects of phases 18 and 20 name the resource in the start call.
- Proof: docs/plan/phase-15.md:1339 "`exit_candidate` opens the workflow instance" and docs/plan/phase-15.md:1340 "`exit_decision` (DSL.md E.4, shipped by this phase, section 4.2)"; docs/plan/phase-18.md:743 "resource: (\"case\", case_id)"
- Smallest fix: In 7.13 state the start, for example `resource: ("exit_decision", review_id)`, and that `start_context` reads `review_id` and `party_id` from that id.

### R-15-3  LOW   (fix F2)
- Problem: `SubjectContext` requires a `title` (neutral, bilingual). F2 gives no title for either new subject, while the comparable subjects of phases 18 and 19 give one.
- Proof: docs/plan/phase-15.md:2174 "`start_context` freezes both, sets `maker_id` to the reviewer"; docs/plan/phase-04.md:685 "pub title: I18nString"; docs/plan/phase-18.md:780 "`title` `{ar: ..., en: \"Case decision\"}`"
- Smallest fix: Add a neutral title to each subject in 7.13, for example English "KYC review" and "Exit decision", each with its Arabic form.

### R-15-4  MEDIUM   (fix F7)
- Problem: Factor C3 still lists the registration country as a wired input. Row C2 marks the registration country pending, and Open doubts 10 says it has no canonical column. F7 leaves registration out of the fan-out for that reason, so the old four-country set (nationality, residence, registration, beneficial-owner residence) is still stated as wired elsewhere in the phase.
- Proof: docs/plan/phase-15.md:1183 "C3 linked-country risk (35) | highest `CountryView::effective_level` over nationality, residence, registration, beneficial-owner residence | wired"; docs/plan/phase-15.md:2255 "C2 (registration country) and C4 (branch border status) stay pending until phase 10 maps them."
- Smallest fix: In row 1183 replace "nationality, residence, registration, beneficial-owner residence" with "nationality, residence, beneficial-owner residence (registration joins when mapped, Open doubts 10)".

### R-15-5  LOW   (fix F9)
- Problem: `primary` is a value of the column `alias_class` in `ref.list_entry_names` (phase 14), but the fix never names the column. "the entry's `primary` row" reads as if `primary` were a column.
- Proof: docs/plan/phase-15.md:1522 "display name from `ref.list_entry_names` (the list entry, not the customer: `ar` is the `raw` of the entry's `primary` row with `script = 'ar'`"; docs/plan/phase-14.md:178 "alias_class TEXT NOT NULL CHECK (alias_class IN ('primary', 'strong', 'weak', 'kunya', 'former'))"
- Smallest fix: Write "the row with `alias_class = 'primary'` and `script = 'ar'`" (and likewise `script = 'lat'` for `en`) in place of "primary row".

### R-15-6  LOW   (fix F11)
- Problem: Step 3 of 7.7 still states the two-argument call `BusinessClock::add(now, review_within)`, which is not the phase 09 signature `add(from, n, unit)`. F11 gives the three-argument call but does not say it replaces the line in 7.7, so two calls for the same due date stand in the phase.
- Proof: docs/plan/phase-15.md:2100 "`due_at = BusinessClock::add(now," and docs/plan/phase-15.md:2101 "`review_within)` on the bank calendar"; docs/plan/phase-15.md:2191 "The call is `BusinessClock::add(now, review_within.value, BusinessUnit::WorkingDays)`"; docs/plan/phase-09.md:459 "fn add(&self, from: DateTime<Utc>, n: u32, unit: BusinessUnit)"
- Smallest fix: Change the call in 7.7 step 3 to `BusinessClock::add(now, review_within.value, BusinessUnit::WorkingDays)`, or state in 7.17 that it replaces the call in step 3.

### R-15-7  MEDIUM   (fix F13)
- Problem: 7.18 says `open_or_join` returns the old and the new `due_at`, but gives no mechanism or signature. In PostgreSQL 17 the RETURNING clause of an INSERT ... ON CONFLICT DO UPDATE shows only the updated row, so the old value needs a read in the same statement (for example a leading CTE that reads the open row's `due_at`). The trait in section 2 still returns `(KycReview, bool)`, so the caller cannot receive the old value as written.
- Proof: docs/plan/phase-15.md:2194 "The method returns the old and the new `due_at`, and the caller moves the timer as section 7.10 says when the date moved."; docs/plan/phase-15.md:983 "async fn open_or_join(&self, tx: &mut Tx, r: &NewReview) -> Result<(KycReview, bool), RepoError>;"; docs/plan/phase-15.md:1020 "RETURNING ...`"
- Smallest fix: In 7.18 give the new return type (for example `(KycReview, bool, Option<DateTime<Utc>>)` carrying the old `due_at`) and the old-value read: a leading CTE that selects the open row's `due_at` before the insert, returned next to RETURNING.

## Summary
- Fixes checked: 13 (F1 to F13).
- Findings: HIGH 0, MEDIUM 3 (R-15-1, R-15-4, R-15-7), LOW 4 (R-15-2, R-15-3, R-15-5, R-15-6).
CHECKED: F1 ok, F2 R-15-1 R-15-2 R-15-3, F3 ok, F4 ok, F5 ok, F6 ok, F7 R-15-4, F8 ok, F9 R-15-5, F10 ok, F11 R-15-6, F12 ok, F13 R-15-7
