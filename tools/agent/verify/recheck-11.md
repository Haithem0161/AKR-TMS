# Re-check, phase 11 (Haiku draft for lead review)

### R-11-1  MEDIUM   (fix F3)
- Problem: The false_alarm path (F3) writes the observed marker into `as_of_marker`, and section 7.13 says `source.halt_resolved` carries that marker, but the step 4 payload and the section 3.10 audit row list only decision, tables and check id, so the event as specified does not carry it.
- Proof: docs/plan/phase-11.md:959 "audit `source.halt_resolved {decision, tables, check_id}`" (and docs/plan/phase-11.md:1607 "and `source.halt_resolved` carries it."; docs/plan/phase-11.md:1196 "(decision, tables, check id)").
- Smallest fix: Add `as_of_marker` to the payload at line 959 and to the row at line 1196, or delete "and `source.halt_resolved` carries it" from line 1607.

### R-11-2  MEDIUM   (fix F4)
- Problem: `ensure_rows` inserts rows without `freshness_class`, so they take the column default F3 (docs/plan/phase-11.md:116), but section 3.7 says `cdc_position` watermarks are F1 and `full_compare` are F5, set on the first unit. No method sets the class (`WatermarkMove` at line 538 has no class field), so CDC and full-compare watermarks run with the wrong class from creation.
- Proof: docs/plan/phase-11.md:1004 "`cdc_position` is F1; `full_compare` is F5;" and docs/plan/phase-11.md:116 "TEXT NOT NULL DEFAULT 'F3'" (the insert is at docs/plan/phase-11.md:1617 "an insert with `value` NULL").
- Smallest fix: In the 7.14 text, state that `ensure_rows` sets `freshness_class` from `kind` (`cdc_position` F1, `full_compare` F5, other kinds F3), as section 3.7 requires.

### R-11-3  LOW   (fix F8)
- Problem: The job's cost_guard test in step 7 compares "by the same factor", but the phase defines no factor (the route's guard at line 1062 is an absolute threshold, `replay.estimate_max_rows`), so the job-side comparison that F8 relies on cannot be implemented as written.
- Proof: docs/plan/phase-11.md:1090 "`cost_guard` when the rows grew past the estimate by the same factor" and docs/plan/phase-11.md:1638 "`LandingStore::estimate` is called by the job only (akr-ingestd), before its first page, for".
- Smallest fix: In line 1090 replace "by the same factor" with the exact threshold the job compares against, or name the factor and the setting that holds it.

### R-11-4  HIGH   (fix F10)
- Problem: F10 says a replay receipt has its watermark fields NULL, but `admin.batches.watermark_kind` is NOT NULL and its CHECK lists no replay value, so read literally the insert is refused, and giving the column a value contradicts the same rule.
- Proof: docs/plan/phase-11.md:1119 "watermark fields NULL (a replay moves no" and docs/plan/phase-11.md:142 "TEXT NOT NULL CHECK (watermark_kind IN".
- Smallest fix: In line 1119 change "watermark fields NULL" to "watermark_from, watermark_to and as_of_marker NULL, and watermark_kind copied from the table's mapping".

### R-11-5  HIGH   (fix F18; F39 has the same callers)
- Problem: Section 3 places `BlindPeriodService` in the akr-ingestd crate, but akr-api holds it (line 1339) and the declaration route of F18 calls it, and the `ops_probe` job (akr-worker) and the `akr-ctl` callers of F39 need it too, so none of them can reach it inside akr-ingestd. Importing the ingestd crate would pull in akr-sources, which breaks the cargo tree check of section 6 item 3 and the phase 00 rule that only akr-ingestd may depend on sources (docs/plan/phase-00.md:389).
- Proof: docs/plan/phase-11.md:511 "ResumeHandler, BlindPeriodService" and docs/plan/phase-11.md:1339 "`blind: Arc<BlindPeriodService>` (declarations)" (and docs/plan/phase-11.md:1436 "`cargo tree -p akr-api` shows no `akr-sources` or driver crate").
- Smallest fix: Move `BlindPeriodService` out of the ingestd list at line 511 into a library crate that akr-api, akr-worker and akr-ctl all link (for example `db`), and keep only the close path in ingestd.

### R-11-6  LOW   (fix F24)
- Problem: The pointer "the extract file contract of this section" sits in 7.5, but the contract is in 7.20, so a reader of 7.5 cannot find it under that name.
- Proof: docs/plan/phase-11.md:1505 "(the extract file contract of this section, which corrects section" and docs/plan/phase-11.md:1679 "### 7.20 The extract file contract: file format, columns and the row reader".
- Smallest fix: In line 1505 replace "the extract file contract of this section" with "the extract file contract of section 7.20".

### R-11-7  MEDIUM   (fix F25)
- Problem: Item 4 takes the columns from the table's "newest receipt", but F24 (line 1505) takes them from the newest accepted manifest; a receipt rejected for a column or control-total failure would then supply the column list, which the accepted-manifest rule excludes.
- Proof: docs/plan/phase-11.md:1695 "against the `columns` of the table's newest receipt (its `manifest`" and docs/plan/phase-11.md:1505 "the columns are the `columns` of the table's newest accepted manifest".
- Smallest fix: In line 1695 change "the table's newest receipt" to "the table's newest accepted receipt", so it matches line 1505.

### R-11-8  LOW   (fix F25)
- Problem: The new receipt reason `column_check_failed` is missing from the list of receipt reasons that end a unit in section 3.10, which a developer would read as the closed set.
- Proof: docs/plan/phase-11.md:1687 "`column_check_failed` and audited as `file.rejected`" and docs/plan/phase-11.md:1181 "They end a unit with a receipt reason (`writer_failed,".
- Smallest fix: Add `column_check_failed` to the reason list at line 1182.

### R-11-9  LOW   (fix F29)
- Problem: "the late-arrival block of this section" names no block in 7.12, and the rule that the load unit writes the event is in 7.22 item 4, so a developer has to guess where it is.
- Proof: docs/plan/phase-11.md:1594 "written by the load unit when the tail grows (the late-arrival block of this section)" and docs/plan/phase-11.md:1727 "`ingest.late_arrival_near_window` is written by the unit, in its transaction".
- Smallest fix: In line 1594 replace "(the late-arrival block of this section)" with "(section 7.22, item 4)".

### R-11-10  MEDIUM   (fix F38)
- Problem: `previous_mapped_business_to` takes `source_table` but no source, so it returns the newest mapped batch of any source with that table name. Two sources may read a table with the same name (line 209), and phase 15 section 7.7 step 5 asks for the previous batch "of the same source table" (docs/plan/phase-15.md, 7.7 step 5).
- Proof: docs/plan/phase-11.md:1757 "It returns the `business_to` of the newest batch of `source_table` with status `mapped`" and docs/plan/phase-11.md:209 "so two sources may read a table with".
- Smallest fix: In line 1757 add a `source_id: Uuid` parameter after `conn` and change the text to "of `source_id` and `source_table`".

## Summary

- Fixes checked: 39 (F1 to F39; the 7.14 to 7.27 blocks located by title).
- Findings: HIGH 2, MEDIUM 4, LOW 4 (10 in total).
- CHECKED: F1 ok, F2 ok, F3 R-11-1, F4 R-11-2, F5 ok, F6 ok, F7 ok, F8 R-11-3, F9 ok, F10 R-11-4, F11 ok, F12 ok, F13 ok, F14 ok, F15 ok, F16 ok, F17 ok, F18 R-11-5, F19 ok, F20 ok, F21 ok, F22 ok, F23 ok, F24 R-11-6, F25 R-11-7 and R-11-8, F26 ok, F27 ok, F28 ok, F29 R-11-9, F30 ok, F31 ok, F32 ok, F33 ok, F34 ok, F35 ok, F36 ok, F37 ok, F38 R-11-10, F39 R-11-5
