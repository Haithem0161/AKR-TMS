# Owner-decision check, round 2, key 11 (Haiku draft for lead review)

### P-11-1  MEDIUM   (fix F3, [O-10-5c])
- Problem: F3 adds `parent_missing` to the section 3.3 step 1 mapping, and section 2 says the mapping of `ConflictKind` to `DqKind` is section 3.3. The value object `ConflictKind` still lists three values, and the section 6 check still counts "the three `ConflictKind` values". The mapping now covers four reasons, so an implementer cannot tell whether `parent_missing` is a `ConflictKind` value.
- Proof: docs/plan/phase-12.md:472-473 "`ConflictKind` (`FactChangedInSource`, `LabelChange`, `ValueChange`; the mapping to `DqKind` is section 3.3)"; docs/plan/phase-12.md:1226 "the three `ConflictKind` values map to their kinds and severities"; new mapping at docs/plan/phase-12.md:739 "`parent_missing` (a row whose parent key has not landed, phase 10 sections 7.20 and 7.27) ->".
- Smallest fix: add `ParentMissing` after `ValueChange` in the `ConflictKind` list at phase-12.md line 473, and change "the three `ConflictKind` values" to "the four `ConflictKind` values" at line 1226.

### P-11-2  LOW   (fix F3, [O-10-5c])
- Problem: The definition at the start of section 3.3 says a conflict is a mapped row that disagrees with a settled one. After F3 the same section maps `parent_missing` rows, which have no settled row to disagree with (their parent has not landed), so the definition does not cover every conflict the section maps.
- Proof: docs/plan/phase-12.md:726-727 "A conflict is a mapped row that disagrees with a settled one, so it is never retried; it is closed when its issue is resolved"; docs/plan/phase-12.md:739 "`parent_missing` (a row whose parent key has not landed, phase 10 sections 7.20 and 7.27) ->".
- Smallest fix: in phase-12.md line 727 change "a mapped row that disagrees with a settled one," to "a mapped row that disagrees with a settled one or whose parent key has not landed,".

### P-11-3  LOW   (fix F1, [O-10-5b])
- Problem: The table comment in phase 11 describes `row_conflict` rows as rows whose canonical value differs from a settled one. After F1 the same kind also holds `parent_missing` rows, which have no canonical value to differ from, because their row is not written.
- Proof: docs/plan/phase-11.md:175 "rows whose canonical value differs from a settled one (`row_conflict`,"; docs/plan/phase-11.md:804-805 "or `parent_missing` (a row whose parent key has no `admin.xref` row yet".
- Smallest fix: in phase-11.md line 175 change "rows whose canonical value differs from a settled one (`row_conflict`," to "rows the writer returns as conflicts (`row_conflict`: a settled value that changed, or a parent key that has not landed),".

## Summary
- Entries checked: 4 (F1 to F4).
- Findings by severity: HIGH 0, MEDIUM 1, LOW 2.
- CHECKED: F1 P-11-3, F2 ok, F3 P-11-1 and P-11-2, F4 ok
