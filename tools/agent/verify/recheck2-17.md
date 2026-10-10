# Second re-check, phase 17 (Haiku draft for lead review)

### S-17-1  HIGH   (fix F6, [R-17-5])
- Problem: The new step 3 in 7.25 sends every second worker who lacks `A2` or `M` to 422 `role_cannot_decide`, while the test row and the base text send the same request (L2 equal to L1, by the A1 user `a1`) to 403 `E_SELF_APPROVAL`. The replacement does not say which refusal is checked first, so one request gets two answers.
- Proof: docs/plan/phase-17.md:1693 "`POST /cases/{id}/assign` as `a1` (L1) then the same caller as L2 → 403 `E_SELF_APPROVAL`; `a2` as L2 → 200." ; docs/plan/phase-17.md:1971 "Step 3: a second worker must hold `A2` or `M`, else 422 `E_VALIDATION` at `/to` with the phase 18 sub-code `role_cannot_decide`" ; the same 403 rule at docs/plan/phase-17.md:1168 "the request is refused with 403 `E_SELF_APPROVAL`". The A1 role of `a1`: docs/plan/phase-28.md:839 "`a1@bank.test` (A1)".
- Smallest fix: In 7.25 step 3, after "sub-code `role_cannot_decide`;", add the sentence "The L2 self-approval refusal of section 3.8 step 4 is checked first." so that `a1` gets 403 as the row at line 1693 states (or change line 1693 to expect 422 `role_cannot_decide` for `a1`).

## Summary
Entries checked: 7 (F1 to F7). Findings: HIGH 1, MEDIUM 0, LOW 0.
CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 S-17-1, F7 ok

Notes outside the checks (not findings, not counted):
- docs/plan/phase-17.md:1798 "a reassignment revokes the previous assignee's grant (`revoked_by` is the actor)" is the 7.2 bullet that the F6 text (docs/plan/phase-17.md:1971, "7.2, second bullet:") changes to a revoke limited by reason. The old wording is still in the file; delete it if one rule is wanted.
- docs/plan/phase-18.md:649 "A reassignment revokes the previous assignee's grant (`revoked_by` is the actor)." is the same unconditional rule in phase 18. The F6 rule differs from it for grants of reason `manual`, `mlro_desk` or `escalation`.
- docs/plan/phase-01.md:743 "a `record_open` step-up proves presence and is never consumed" conflicts with the `StepUpService::consume` call for `record_open` at docs/plan/phase-17.md:1475, and with the same call in the sentence that F7 edits (docs/plan/phase-17.md:1240). This is base text, not part of the replacement.
- The F1 verdict says 7.17 names the grant. It does not: the grant is at docs/plan/phase-17.md:568, and docs/plan/phase-02.md:1334 points to it correctly. The F1 replacement itself is correct.
