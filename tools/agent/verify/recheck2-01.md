# Second re-check, phase 01 (Haiku draft for lead review)

### S-01-1  MEDIUM   (fix F2, [R-01-1b])
- Problem: The uniqueness rule covers only principals that are not disabled, so a disabled principal can keep the name and a new principal can then take it. The lookup has no status filter, so it matches two rows, and a `disable --name` that picks the old row would leave the live principal active.
- Proof: docs/plan/phase-01.md:1122 "A name held by a service principal that is not disabled is refused" and docs/plan/phase-01.md:1123 "find the principal by it (`kind = 'service'` and `display_name ->> 'en'`)".
- Smallest fix: In docs/plan/phase-01.md line 1123, change "find the principal by it (`kind = 'service'` and `display_name ->> 'en'`)" to "find the principal by it among principals that are not disabled (`kind = 'service'`, `status = 'active'` and `display_name ->> 'en'`)".

## Summary
- Entries checked: 3 (F1, F2, F3)
- Findings: HIGH 0, MEDIUM 1, LOW 0
CHECKED: F1 ok, F2 S-01-1, F3 ok
