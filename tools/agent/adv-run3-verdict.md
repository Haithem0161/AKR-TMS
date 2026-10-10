# Adversarial pass run 3: lead verdict (2026-10-10; decisions final, applied by apply-adv-run3.py)

Haiku draft tools/agent/adv-run3.md (watermarks, restores, failover, late batches, source pauses): 22 candidates; FLAW 8
(HIGH 7, MEDIUM 1), DOUBT 3, GUARDED 11. About 242k tokens.
Lead result: 9 confirmed (HIGH 6, MEDIUM 1, LOW 2), 2 rejected, 11 guarded accepted as checked.

## Confirmed
- ADV-3-01 HIGH (phase 11, read by phase 12). `FreshnessPolicy::classify` has no rule on the source state. While a
  source is paused nothing reads the CBS date, so rule e compares with a frozen date and rule f never fires; the ledger
  shows every unit `monitored`. Fix: rule a2, a source in `paused`, `paused_by_bank` or `refused` is stale
  (`lag_ceiling`); the ledger then reads `source_stale`.
- ADV-3-07 LOW (phases 11, 12), the doubt resolved. A pause period carries no `business_from`, and "covers the as-of
  date" is not defined. Fix: automatic pause, bank-pause and refusal opens set `business_from` as the halt path does;
  phase 12 states the cover rule (`business_from` NULL or on or before the date, and `business_to` NULL or on or after
  it).
- ADV-3-03 HIGH (phase 11). Planner rule a gives `sequence_id` no overlap ("a sequence needs none"), against research 25
  section 7.3: a key taken early and committed late is skipped forever. The counts rung shows the break later, so HIGH.
  Fix: a key window `ingest.sequence_overlap_keys` behind the stored key; the hash guard absorbs repeats.
- ADV-3-04 HIGH (phase 11). Research 25 section 7.5 defines the regression as the source's high-water mark (highest key,
  row version or posting time) falling below AKR's watermark; phase 11 checks only the business date and the engine
  marker, and open doubt 8 leaves SQL Server, Db2 and Sybase markers unvalidated. A same-day restore on those engines is
  missed. Fix: one aggregate `MAX` read per table and branch before a unit; below the stored value halts
  (`as_of_regressed`, statement signal `high_water_regressed`). A legitimate delete of the newest rows is answered with
  `restored` at the observed maximum.
- ADV-3-05 HIGH (phase 11). DSL `lookback: auto` (C-21) measures late arrivals, alarms and widens the window; phase 11
  parses `lookback` and builds none of it. The 31-day window is DR-37 and stays; the missing part is the measure and the
  alarm. Haiku's fix (raise `recon.lookback_days`) does not reach rows past the window and is not taken. Fix: per receipt
  `late_days_max`, a histogram, an alarm near the window, and widening for `auto` up to `ingest.lookback_max_days`.
- ADV-3-06 HIGH (phase 10). Step 4 closes the current version only when its `valid_from` is earlier. A second change of
  a party on the same business date (two units in one day), or a back-dated re-read, inserts a second current row; the
  partial unique index refuses it, the batch fails as `writer_failed` on every retry and the table stops. Haiku's fix
  (record a conflict) would block ordinary master updates and is not taken. Fix: `valid_from = GREATEST(row business
  date, current valid_from)`; when equal, the current row is closed in recorded time only.
- ADV-3-10 HIGH (phase 12). `accepted_for(conn, source_id, object, kind)` has no group key and the acceptance stores no
  amount, so any later break of any size on the object is `confirmed_gap` and leaves the open count. The certificate
  still lists it, so HIGH, not CRITICAL. Fix: the acceptance stores `accepted_delta` in `detail`; `accepted_for` takes
  the field; a larger delta stays `open`.
- ADV-3-09 MEDIUM (phase 11), the doubt resolved. `false_alarm` moves no watermark, so after a failover to a standby
  with a lower marker every later unit halts again. Fix: `false_alarm` after an `as_of_regressed` halt writes the
  observed marker into `as_of_marker`.
- ADV-3-08 LOW (phase 11), the doubt resolved. For a `business_date` table `restored_to` is a date and the reset writes
  `value`, `value_business_date` and `business_date_moved_at`.

## Rejected
- ADV-3-02 (an endpoint change with a higher marker): the commits the standby lost were never read by AKR and no longer
  exist in any source AKR can read; the bank's own loss is not a read gap. The dangerous variant, a copy behind AKR's
  watermark, is caught by the ADV-3-04 high-water check (and by the marker where it is validated).
- ADV-3-11 (an exclusion outranks a dark cause in the ledger): designed. An excluded unit is approved not to be
  monitored (phase 12 section 3.6, "Why an exclusion is never a silent gap"); the source-level blind period and stale
  items are listed in the certificate and the statement.

## Guarded (accepted as checked)
ADV-3-12 to ADV-3-22: table lock before change detection; oldest-first units and the re-read window; extract file
sequence and hash checks; tier and delivery fixed; kill switch ends units whole; totals skew gives breaks, not passes;
grant check per session and pause blind periods; resolved conflicts re-checked by the next day run; rows the restored CBS
lost are kept; `stale` true when any watermark is stale; a table not read goes stale.

DR: DR-409 (the run 3 fixes).
