# Re-check, phase 24 (Haiku draft for lead review)

### R-24-1  HIGH   (fix F34)
- Problem: The fix gives the export file the classification `restricted`. The column `classification` of `admin.evidence_objects` is `admin.data_classification NOT NULL` (phase 02), and that enum has no value `restricted`, so the insert of the file fails and every register export job fails. No phase file adds the value (a grep for ADD VALUE finds none).
- Proof: docs/plan/phase-24.md:1348 "classification `restricted` (a file holds whatever rows the requester may read)"; docs/plan/phase-00.md:288 "('public', 'internal', 'personal', 'financial', 'detection-logic', 'secret-str', 'card');"
- Smallest fix: In docs/plan/phase-24.md:1348 change "classification `restricted`" to "classification `personal`" (the class section 7.13 uses for personal text), or add the value in this phase's migration and say so.

### R-24-2  MEDIUM   (fix F6)
- Problem: The fix states one state rule for every row of the review calendar: amber in the 30 days before `due_on`, red on and after `due_on`. The scenario bullet of section 3.4, which the fix does not replace, still gives scenario rows amber at one interval and red at two intervals of `review.rules_months`, so the same state column has two rules for scenario rows.
- Proof: docs/plan/phase-24.md:657 "`amber` in the 30 days before `due_on`, `red` on and after `due_on`, as 7.3 says"; docs/plan/phase-24.md:662 "against amber at one interval and red at two intervals of `review.rules_months`"
- Smallest fix: In docs/plan/phase-24.md:662 replace "against amber at one interval and red at two intervals of `review.rules_months`" with a due date for scenario rows (for example "`due_on` = activation instant plus one interval of `review.rules_months`") so that the rule of line 657 applies, or name the interval rule as an exception to line 657.

### R-24-3  MEDIUM   (fix F33)
- Problem: The export request takes `register` (a screen ID) and `filters`, and section 7.12 item 1 takes the filters from the screen's list route. For G4 the list route takes `kind` in its path (route 9), not in its filters, so nothing in the payload carries the kind that "one export per kind" needs. The implementer has to guess where the kind goes.
- Proof: docs/plan/phase-24.md:1478 "G4 is `GovernanceRegisterService::list` (one export per kind)"; docs/plan/phase-24.md:1339 "its filters are those of the screen's list route"; docs/plan/phase-24.md:555 "| GET | `/api/v1/registers/{kind}` | `dashboard.read`, `model.risk` |"
- Smallest fix: In docs/plan/phase-24.md:1478 add after "(one export per kind)": "The G4 export's `filters` carry `kind`, one of the five register kinds, which route 9 reads from its path."

### R-24-4  MEDIUM   (fix F33)
- Problem: The fix gives G5 no export entry because "G5 holds counts only". That premise is wrong: G5 also returns rows, the access history of one record (route 7 returns `access_event` rows, section 3.2 item 3) and the record picker of section 5.3 part (a). Section 5.3 labels G5 a REGISTER screen and section 7.12 says each REGISTER screen offers one export, so the stated reason does not support the decision.
- Proof: docs/plan/phase-24.md:1478 "G5 holds counts only and G6 is never exported (section 3.1 step 9), so neither has an entry."; docs/plan/phase-24.md:612 "Rows: `access_event` where `resource_type = kind` and `resource_id = id`"; docs/plan/phase-24.md:1197 "(a) Access history: a record picker (kind and id)"
- Smallest fix: In docs/plan/phase-24.md:1478 replace "G5 holds counts only" with the true reason (the access history is per record and the control report is counts only), and state whether the history rows get an export entry.

### R-24-5  MEDIUM   (fix F35)
- Problem: The fix says a zero-row UPDATE answers 404 when the row is outside the caller's party visibility, but the UPDATE's WHERE clause has no visibility predicate. An unexplained row outside the caller's visibility is updated and the call succeeds, so the stated 404 cannot occur. Lists apply the party visibility in SQL (section 7.13 item 4); the write does not, and no visibility-filtered read is named.
- Proof: docs/plan/phase-24.md:1483 "Zero rows updated is 409 `E_NOT_ALLOWED_IN_STATE` (404 `E_NOT_FOUND` when the row is absent or outside the caller's party visibility)"; docs/plan/phase-24.md:1424 "Lists and counts filter in SQL by the phase 18 section 3.10 party visibility"
- Smallest fix: In docs/plan/phase-24.md:1483 add the caller's visibility predicate to the UPDATE's WHERE clause as a bound parameter (as section 7.13 item 4 does for lists), or state that a visibility-filtered SELECT runs first in the same transaction and answers 404.

### R-24-6  LOW   (fix F14)
- Problem: The reference "phase 04's key, section 7.1" reads as phase 04 section 7.1, which is "WorkflowSubject: check_before_decision" and says nothing about the assessor cap. The key's text is in phase 04 section 7.10.
- Proof: docs/plan/phase-24.md:760 "no more than `access.assessor_max_days` (phase 04's key, section 7.1) after `valid_from`"; docs/plan/phase-04.md:1640 "### 7.1 WorkflowSubject: check_before_decision"
- Smallest fix: In docs/plan/phase-24.md:760 change "(phase 04's key, section 7.1)" to "(phase 04's key, phase 04 section 7.10)".

## Summary
- Fixes checked: 36 (F1 to F36).
- Findings by severity: HIGH 1, MEDIUM 4, LOW 1 (6 in total).
CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 R-24-2, F7 ok, F8 ok, F9 ok, F10 ok, F11 ok, F12 ok, F13 ok, F14 R-24-6, F15 ok, F16 ok, F17 ok, F18 ok, F19 ok, F20 ok, F21 ok, F22 ok, F23 ok, F24 ok, F25 ok, F26 ok, F27 ok, F28 ok, F29 ok, F30 ok, F31 ok, F32 ok, F33 R-24-3 R-24-4, F34 R-24-1, F35 R-24-5, F36 ok
