# Owner-decision check, round 4, key 13 (Haiku draft for lead review)

## Summary

- Entries checked: 2 (F1, F2).
- Findings by severity: HIGH 0, MEDIUM 0, LOW 0.
- Evidence, F1: the new text stands on docs/plan/phase-13.md line 976. The guard `feat.definition_guard` is a row trigger (lines 289-290, `FOR EACH ROW`), and its RAISE on line 284 gives the message in full. The seed of item 3 (section 3.1, line 596) inserts version-1 rows, so the owner's UPDATE has a row to meet. `akr_app` holds UPDATE only on (state, retired_at) (line 317), so `expression` is outside its column grant. No other line of phase 13 states the owner rule, and the only REVOKE in the file is on a function.
- Evidence, F2: the new text stands on docs/DATA-MODEL.md line 545. Section 7.18 item 1 (phase-13.md line 1441) gives `no_history`, and `beyond_window` for `days_since`, with `basis` reason `no_activity`. The status CHECK of `feat.feature_values` (phase-13.md line 179) allows value, no_history and beyond_window. Line 1193 agrees for `dormant_days`. No other line of DATA-MODEL names these statuses.

CHECKED: F1 ok, F2 ok
FINDINGS: 0
