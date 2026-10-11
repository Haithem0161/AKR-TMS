# Owner-decision check, round 3, key 18 (Haiku draft for lead review)

### Q-18-1  LOW   (fix F1, [L2-3])
- Problem: The NEW text gives the guard message `an access grant is revoked, never changed` for any `casework.case_acl` row. The guard raises that message only for a live grant. For a revoked row (`revoked_at` set) its first branch raises `a revoked access grant does not change` before the access comparison runs, so the owner gets a different message when a revoked row's `access` is changed. The clause does not say which row state it covers. The permission-denied part and the role naming are correct.
- Proof: docs/plan/phase-18.md:342 "IF OLD.revoked_at IS NOT NULL THEN" and docs/plan/phase-18.md:343 "RAISE EXCEPTION 'a revoked access grant does not change'"; against docs/plan/phase-18.md:1246 "as the owner `akr_migrate` the guard raises `an access grant is revoked, never changed`".
- Smallest fix: In docs/plan/phase-18.md line 1246, qualify the owner part to a live grant and name the second message, e.g. "...as the owner `akr_migrate` the guard raises `an access grant is revoked, never changed` on a live grant, and `a revoked access grant does not change` on a revoked one)."

## Summary

Entries checked: 1 (F1). Findings by severity: HIGH 0, MEDIUM 0, LOW 1.

CHECKED: F1 Q-18-1
