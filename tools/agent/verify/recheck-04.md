# Re-check, phase 04 (Haiku draft for lead review)

### R-04-1  LOW   (fix F1)
- Problem: The fix moved the pointer for the not-added SoD pair to section 3.1, but section 3.1 (The change-request model) does not mention the pair or DR-54. The statement is in section 3.5 (Eligibility and four-eyes), which the old pointer named correctly.
- Proof: docs/plan/phase-04.md:1521 "the announced change-request pair is not added (section 3.1, DR-54)"; docs/plan/phase-04.md:1090 "The role-level SoD pair `["change_request.create", "change_request.decide"]` that phase 03 announced is **not**"
- Smallest fix: At line 1521 change "(section 3.1, DR-54)" back to "(section 3.5, DR-54)".

### R-04-2  MEDIUM   (fix F2)
- Problem: The grant now lets akr_app update `evidence`, but two lines of the same phase still say that only lifecycle columns are updatable: the section 1 comment says content columns are INSERT-only, and the section 6 check says UPDATE is on the lifecycle columns only. Neither line is replaced by a 7.x block (7.18 says only that the grant includes evidence), so the phase now contradicts itself.
- Proof: docs/plan/phase-04.md:202 "-- Content columns are INSERT-only: what the checker approves is byte for byte what the maker proposed. The"; docs/plan/phase-04.md:1553 "lifecycle columns only; `UPDATE admin.change_requests SET proposed = '{}'` as `akr_app` fails (permission); an"
- Smallest fix: At lines 202-203 say that `evidence` is the one content column appended by the route of 7.2 (7.18), and at line 1553 change "UPDATE on the lifecycle columns only" to "UPDATE on the lifecycle columns and `evidence` only".

### R-04-3  LOW   (fix F5)
- Problem: The steps of the evidence route now count only `approved` and `rejected` task outcomes as a decision, but the table row of the same block says evidence closes at "no decision recorded yet". A deciding `sign` task (outcome `signed`, tasks.deciding is true for sign steps) is a decision under the second wording and is not under the first. The shipped change-request definitions use only `approve` steps, so the effect today is nil, but the two lines give different rules.
- Proof: docs/plan/phase-04.md:1658 "The maker attaches a file to their request while it is `draft`, or `submitted` with no decision recorded yet"; docs/plan/phase-04.md:1660 "status `draft`, or `submitted` with no task of the request's instance whose `outcome` is `approved` or `rejected` in"
- Smallest fix: At line 1660 change "`approved` or `rejected`" to "`approved`, `rejected` or `signed`" (or reword the table row at line 1658 to match the steps), so both lines state one rule.

### R-04-4  MEDIUM   (fix F10)
- Problem: The fix says the definition validator refuses `mode: rework` without `to`, but names no refusal code. Every other refusal of the validator in section 3.8 is named by a code (line 1230 lists the catalogue codes, among them `E_SCHEMA` for the schema pass and `E_DANGLING_REF` for references), so the developer must choose one for this refusal. The block also names no code for a `to` that is a later step rather than an earlier one.
- Proof: docs/plan/phase-04.md:1800 "and the definition validator refuses `mode: rework` without it"; docs/plan/phase-04.md:1230 "the whole DBS catalogue (`E_SCHEMA`, `E_NO_START`, `E_BAD_START`, `E_CYCLE`, `E_DANGLING_REF`"
- Smallest fix: In the 7.19 block, name the code after "refuses": `E_SCHEMA` for a rework reject without `to`, and `E_DANGLING_REF` for a `to` that names no step of the definition; the block should also name the code for a `to` that is a later step.

## Summary

- Fixes checked: 12 (F1 to F12).
- Findings: HIGH 0, MEDIUM 2, LOW 2.
- CHECKED: F1 R-04-1, F2 R-04-2, F3 ok, F4 ok, F5 R-04-3, F6 ok, F7 ok, F8 ok, F9 ok, F10 R-04-4, F11 ok, F12 ok
