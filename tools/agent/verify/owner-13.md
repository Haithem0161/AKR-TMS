# Owner-decision round, phase 13 (Haiku draft for lead review)

### O-13-1  HIGH   (section 7.17 item 7 activation and item 1 table, check 4)
- Problem: The activation step sets `valid_to` for an `ended` row and nothing else, so `retired_at` and `retired_by_request` stay NULL, and PostgreSQL refuses the row because both CHECKs need them NOT NULL for any state other than `active`. No `ended` row can be written, so the closed-row check and the ended-row cargo test of item 14 cannot run.
- Proof: docs/plan/phase-13.md:1241 "CHECK ((state = 'active') = (retired_at IS NULL))" and 1242 "CHECK ((state = 'active') = (retired_by_request IS NULL))"; docs/plan/phase-13.md:1370 "close each `retire` as `ended` (with `valid_to`) or `void` (`retired_at = now()`, `retired_by_request`)".
- Smallest fix: in the item 7 activation cell, set `retired_at = now()` and `retired_by_request` for `ended` as well as `void` (`ended` also sets `valid_to`).

### O-13-2  HIGH   (section 7.17 item 6 step 3, check 4)
- Problem: `valid_from` may be today, but the default `valid_to` of an `end` is the last closed business date, which is always before today, so the default falls before `valid_from`. That breaks CHECK `valid_to >= valid_from` at activation and the range "between the row's valid_from and today" in the same sentence.
- Proof: docs/plan/phase-13.md:1359 "`valid_from` no later than today in bank time"; docs/plan/phase-13.md:1361 "`end` takes `valid_to` between the row's `valid_from` and today, and defaults to the last closed business date"; docs/plan/phase-13.md:1240 "CHECK (valid_to IS NULL OR valid_to >= valid_from),".
- Smallest fix: default `valid_to` to the later of `valid_from` and the last closed business date, and say so in step 3; or refuse a default that falls before `valid_from` with 422 `E_VALIDATION` at `/retire/<i>/valid_to`.

### O-13-3  HIGH   (section 7.17 item 14, first bullet, check 4 and check 6)
- Problem: The check expects the trigger message for a DELETE run as `akr_app`, but item 1 grants that role only SELECT and INSERT, so PostgreSQL refuses with "permission denied for table source_tags" before the trigger runs. The stated message never appears, so the check fails as written.
- Proof: docs/plan/phase-13.md:1426 "As `akr_app`: `DELETE FROM feat.source_tags` fails with `source tags are never deleted`"; docs/plan/phase-13.md:1278 "GRANT SELECT, INSERT ON feat.source_tags TO akr_app;".
- Smallest fix: expect "permission denied for table source_tags" for `akr_app`, and run the message check as the table owner.

### O-13-4  HIGH   (section 7.17 item 3, static segment predicate, check 1 and check 2)
- Problem: The text says a predicate with `value: false` is true for a customer that has a reach row, which is what `true` means. The compile rule gives only the EXISTS form, so a `false` predicate built from the text would select the payroll customers.
- Proof: docs/plan/phase-13.md:1328 "`{ "ref": "tag.payroll", "op": "=", "value": true }` (or `false`) is true for a customer with a `feat.customer_source_tags` row of that tag."
- Smallest fix: say that `value: false` is true for a customer with no row of that tag and compiles to `NOT EXISTS (...)` over the same subquery.

### O-13-5  HIGH   (section 7.17 item 6 step 2 and item 7 activation, check 4)
- Problem: Step 2 allows an add of an account whose active row is in the same change set's `retire`, which is the only way to change a payer name since no grant updates `payer_name`. The activation inserts the adds before closing the retires, so the partial unique index on `(tag, account_hk)` refuses the insert while the old row is still active.
- Proof: docs/plan/phase-13.md:1360 "An `active` row of the same tag whose `account_hk` or `alt_account_hk` equals one of the entry's hashes, and that is not in `retire`, is sub-code `already_tagged`"; docs/plan/phase-13.md:1370 "Insert each `add` as `active` with the hashes resolved again; close each `retire` as `ended`"; docs/plan/phase-13.md:1245 "CREATE UNIQUE INDEX IF NOT EXISTS uq_source_tags_active ON feat.source_tags (tag, account_hk) WHERE state = 'active';".
- Smallest fix: state in item 7 that the retires are closed first and the adds are inserted after, both in the approval transaction.

### O-13-6  MEDIUM   (section 7.17 item 6 step 2, check 6 refusal codes)
- Problem: Step 2 checks adds against active rows only, so two adds of one change set with the same `account_hk` for one tag pass, and the second insert at activation violates `uq_source_tags_active`. Step 2 never compares the adds with each other on `alt_account_hk`, and no status or sub-code is given for either case.
- Proof: docs/plan/phase-13.md:1360 "An `active` row of the same tag whose `account_hk` or `alt_account_hk` equals one of the entry's hashes"; docs/plan/phase-13.md:1359 "no pair `(tag, subject_kind, subject_id)` twice in `add`"; docs/plan/phase-13.md:1245 "WHERE state = 'active';".
- Smallest fix: add to step 2 that an add whose hash equals an earlier add's hash for the same tag is refused with sub-code `already_tagged` at its `/add/<i>/subject_id`.

### O-13-7  MEDIUM   (section 7.17 items 5 and 7, volume effect, check 6 completeness)
- Problem: The impact lists hold the readers of "a tag name of the change set", but a retire entry carries only `id`, `mode` and `valid_to`, and the text never says which tag it names. A change set that only ends or voids a tag that a segment reads can then get `neutral` and skip the Compliance Head.
- Proof: docs/plan/phase-13.md:1340 "`RetireSourceTag { id, mode: end | void, valid_to? }`"; docs/plan/phase-13.md:1363 "the two lists hold the keys of the active static segments and active feature definitions that read a tag name of the change set".
- Smallest fix: say that a retire entry's tag is the tag of the row it names (looked up by `id`), and that the impact counts those tags as well.

### O-13-8  MEDIUM   (section 7.17 item 8 step 3, check 6 completeness)
- Problem: A tag change enqueues `segment_assign` (and the activation enqueues only the refresh job), but base section 3.2 recomputes feature months only within `features.catchup_months` plus `reopen_from`. A tag that starts or stops for an older month therefore leaves that month's values of a tag-reading feature stale.
- Proof: docs/plan/phase-13.md:1397 "When a row was inserted or deleted and an active static segment reads a tag name, enqueue `segment_assign {level: static}`."; docs/plan/phase-13.md:604-605 "within `features.catchup_months` back from the newest, plus every month" (the sentence ends "from `reopen_from`" on line 605).
- Smallest fix: in step 3, also enqueue `feature_compute {reopen_from}` set to the earliest `valid_from` or `valid_to` of the changed rows when a feature definition reads a tag name.

### O-13-9  MEDIUM   (base section 4, Network zones, check 2)
- Problem: The base line says only the worker reads `canon` and the API reads `feat` only. The block gives the `api` service `SourceTagService::propose`, which reads `canon.account` and `canon.counterparty`, and does not say that it replaces the base line.
- Proof: docs/plan/phase-13.md:933 "Only the worker reads `canon` for this phase; the API reads `feat` only."; docs/plan/phase-13.md:1360 "Resolve each `add`. `account`: the current `canon.account` row of `subject_id`".
- Smallest fix: change the base line to say the API reads `canon` (read only) for the source-tag proposal of section 7.17 item 6; the API connects as `akr_app`, which already holds the SELECT grants on `canon.account`, `canon.counterparty` and `canon.txn` (phase 10 SELECT grant block).

### O-13-10  LOW   (section 7.17 item 13, check 1 and check 2)
- Problem: The text says `screens.ts` lists the tab with roles for CFG, CHK, M and CH, but in phase 08 `screens.ts` has one entry per screen with that screen's own `roles`, and no tab entries. Phase 09 section 7.5 says a D3 tab has no role list of its own.
- Proof: docs/plan/phase-13.md:1423 "`screens.ts` lists the tab for CFG (propose) and for CHK, M and CH (read)"; docs/plan/phase-08.md:243 "roles: Role[];" ("who may open it"); docs/plan/phase-09.md:1312-1313 "It has no role list of its own".
- Smallest fix: replace the sentence with "the tab renders when the caller may call one of its operations (GET or POST `/ref/source-tags`), as phase 09 section 7.5 says for D3 tabs".

### O-13-11  LOW   (section 7.17 items 9 and 10, wording)
- Problem: Item 10 says "two more rows of section 3.11", but the section 3.11 table gains one row (line 842) for both actions. Item 9 says "two more rows of section 3.10", but that table gains no row; the two events appear in the sentence after it (line 827).
- Proof: docs/plan/phase-13.md:1400 "(two more rows of section 3.10)"; docs/plan/phase-13.md:1402 "(two more rows of section 3.11; policies in `81_features.cedar`"; docs/plan/phase-13.md:842 "| `source_tag.read`, `source_tag.propose` (section 7.17 item 10) | System | CFG, CHK, M, CH read; CFG proposes |".
- Smallest fix: say "one more row of section 3.11 (both actions)" in item 10, and "the two events named after the section 3.10 table (line 827)" in item 9.

### O-13-12  LOW   (owner note at line 1213, pointer)
- Problem: The note cites section 7.7 for where the P2 decision on the tags was recorded, but section 7.7 says only that a counterparty profile is P2. The diff also has no hunk in section 7.7, although the change description says it was corrected.
- Proof: docs/plan/phase-13.md:1213 "(header, S-17; section 7.7)"; docs/plan/phase-13.md:1151 "and a counterparty profile is P2."
- Smallest fix: drop "; section 7.7" from the note at line 1213, so that it cites the header and S-17 only.

## Summary
- Blocks checked: 7.17 (items 1 to 14) and the owner note at line 1213 that precedes it.
- Changed base places checked: 20 (diff hunks at lines 18, 20, 27, 35, 43, 534, 635, 776, 827, 842, 901, 928, 930, 942, 955, 1021, 1454, 1488, 1497, 1501).
- Findings by severity: HIGH 5, MEDIUM 4, LOW 3 (12 in all).
- CHECKED: 7.17 finding ids O-13-1, O-13-2, O-13-3, O-13-4, O-13-5, O-13-6, O-13-7, O-13-8, O-13-9, O-13-10, O-13-11, O-13-12.
- Checked and consistent (no finding): the route count (13: the 11 rows at lines 505 to 515 and the two at lines 1335 and 1336); the two API.md rows (docs/API.md:365 and 366) for method, path and scopes; the classification of the 16 and 6 columns (each once; 22 rows); the 2 retention rows; the three settings and their floors; the two Cedar actions and their roles; the grants for the refresh job's upsert and delete; the compiled EXISTS for a tag name (every column it names exists); the references to phase 04, phase 10, phase 16 rule 6, DSL.md section 1, phase 18 section 7.29 and research DR-416 and DR-360.
- Not reported, for the lead to decide: a bare tag name in a feature `where` may sit outside the DSL.md Filter subset (DSL.md lines 113 to 116 list comparisons and `IS NULL`, not a bare boolean), and the text never writes its syntax. The base already uses `IS NOT TRUE`, which that list also omits, so I did not record it as a finding.
