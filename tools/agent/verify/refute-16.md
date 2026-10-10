# Refuter, phase 16 (Haiku draft for lead review)

### V-16-01-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: No named insert for one bank scenario. Phase 06's bulk `insert_missing` would skip a duplicate POST silently, and no duplicate-code refusal is written.
- Evidence: Part refuted: the insert is named with its main columns, phase-16.md:1496 "insert `scn.scenarios` (`pack = bank`, `register_version` = the installed register, status `registered`)", and the other columns are the `NewScenario` fields at phase-16.md:728. Part stands: no single-row repository method exists for the scenario row (`grep -rn ScenarioRepository docs/plan/` hits only phase-06.md:401 and 521; phase-06.md:402 "async fn insert_missing(&self, tx: &mut Tx, s: &[NewScenario])" takes a slice). The code is unique (phase-06.md:74 "TEXT NOT NULL UNIQUE CHECK (code ~"), and the error catalogue has no unique-violation row (phase-00.md:485-505; `grep 23505` in phase-00, -03, -06 gives none). Greps in 3.15 for `duplicate`, `409`, `23505`: none.
- Fix: phase 16, section 3.15 step 2 and the section 2 repository list: name the one-row insert method and state the status and problem code a duplicate code gets.

### V-16-01-b  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The POST body `ScenarioCreate` is named without fields. `NewScenario` has no document field, yet step 2 installs the given document as draft 1.0.0.
- Evidence: Part refuted: the document is a field of the version struct, phase-16.md:729-730 "`NewScenarioVersion` (`logic_version`, `document`, `document_sha256`, `fixtures`", and step 2's draft goes through phase-16.md:739 "async fn insert_draft(&self, tx: &mut Tx, v: &NewScenarioVersion)". Part stands: `ScenarioCreate` appears only at phase-16.md:868 with no field list (`grep -rn ScenarioCreate docs/` gives that one line). The body can be derived: the `NewScenario` fields (phase-16.md:728) less `owner_id` (the caller, phase-16.md:1494) and `research_grade` (set to D), plus the document. No line says so, which is why the severity is MEDIUM (inferable from nearby text).
- Fix: phase 16, section 2 utoipa list (line 868): one sentence listing the `ScenarioCreate` fields, including the document.

### V-16-01-c  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The refusal `owner_not_active` has no HTTP status and no problem code.
- Evidence: Part refuted (status and code): a general rule gives field validation 422 `E_VALIDATION`, API.md:985 "Field or document validation; `errors[]` (DSL.md s.7 codes)", and phase-00.md:499 maps `Validation` to 422 `E_VALIDATION`. Phase-06.md:611 and phase-12.md:758 give this same refusal that status (422 `E_VALIDATION`). Part stands (the errors[] entry): the 3.17 list restricts `errors[]` to DSL codes, phase-16.md:1575-1576 "`E_VALIDATION` (422; `errors[]` with the DSL codes of section 3.1", and `owner_not_active` is not one. Phase 16 gives the refusal no status and no code of its own (`grep -rn owner_not_active docs/plan/phase-16.md`: line 1494 only).
- Fix: phase 16, section 3.15 step 1 and the 3.17 list: name the `errors[]` code of `owner_not_active` under the 422 `E_VALIDATION` answer.

### V-16-03-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: A reason code that is inactive or outside family `tuning` or `change` has no refusal code at drafting. Phase 04 refuses it with 422 `reason_not_allowed`, but only at change-request create, after drafting.
- Evidence: Part refuted: the same check exists for the proposal. phase-04.md:872-873 "reason code active, of a family the target accepts, and listing the kind in `object_kinds` when that is set -> else 422 (`reason_not_allowed`)", and phase-16.md:1105 "Proposing: `POST /change-requests` (phase 04 generic create" sends every parameter set through that create. Part stands: the drafting step names the rule without its refusal, phase-16.md:1100 "4. Reason: `reason_code` is an active code of family `tuning` or `change`". Step 5 (phase-16.md:1103) inserts the draft whatever the answer, so an inactive code gets a saved draft and is refused only at proposal. Greps: `reason_not_allowed` in phase-16.md (none); no status or code in 3.5 steps 1-6.
- Fix: phase 16, section 3.5 step 4: state the status and problem code the drafting route answers for a reason code that is inactive or of the wrong family.

### V-16-03-b  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Drafting reads `next_version` and then inserts, with no transaction or lock stated. Two drafts on one version meet at `uq_parameter_set_version`, and the unique violation has no mapped response.
- Evidence: Part refuted (transaction): every POST runs in one request transaction. API.md:38 "Every POST, PUT, PATCH and DELETE requires `Idempotency-Key`", and phase-00.md:1023 "The handler receives `RequestTx` as its business transaction"; phase 16 says the same at phase-16.md:734 (every write takes the caller's transaction). Part stands (race and response): `next_version` takes a plain connection (phase-16.md:756, `conn: &mut PgConnection`), so two drafts can read the same number. The second insert fails on the unique index at phase-16.md:132 (`uq_parameter_set_version`), and no status or code is mapped for it (phase-00.md:485-505 has no unique-violation row; `grep 23505` in phase-00 and in phase-16 section 3.5 gives none). No lock or retry is stated (greps `lock`, `FOR UPDATE` in phase-16.md:1093-1125: none). The index prevents corrupt data, so the gap is the response and the allocation rule.
- Fix: phase 16, section 3.5 step 5: say how `set_version` is allocated under two concurrent drafts (lock or retry) and what a unique violation answers.

### V-16-03-c  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: The capacity BTL refusal applies only to kinds whose `required_evidence` lists `btl_sample`. The parameter_set target row and its prepare steps do not name it.
- Evidence: phase-04.md:661 "// for the computed class (DSL.md D.3)" on the `required_evidence` field: the list follows the computed class. DSL.md:927 "Parameter value outside the band, or any loosening" is the material row, and it lists the sample (`Replay; below-the-line sample; for a capacity reason an approver outside operations`). Phase 16 cites that rule for its class at phase-16.md:1109 "**Change class** (`ParameterSetTarget::prepare`, phase 04 classification rules; DSL D.3):". Phase 16 also names the sample as change-request evidence at phase-16.md:1351. Greps: `btl_sample` in 3.5 and 3.14 only (none); the wiring is by the cited class rule, so the gap is not in the target row.

### V-16-03-d  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: L-18 requires below-the-line evidence for threshold changes in general. S-39 names it for capacity changes, and 3.5 step 4 applies it to capacity only. No Decision and no DR records the narrower scope for risk-tagged sets.
- Evidence: Part refuted (material risk-tagged sets): DSL.md:927 puts the below-the-line sample on every material parameter change, whatever the reason, and phase-16.md:1109 cites DSL D.3 for the class. So a material risk-tagged set already needs the sample. Part stands (minor risk-tagged sets): DSL.md:926 "Parameter value inside the approved band, not loosening | Minor | Replay" asks for no sample, while L-18 (research/20-enforcement-failures-lessons.md:2080) says "every change records reason code (risk-driven or capacity-driven), before-and-after alert and below-the-line evidence". Neither phase 16 nor DR-113 (docs/plan/research.md:124, which cites L-18 and DSL D.3 and says "the band decides minor or material") records which rule wins. The DR rows that mention below-the-line are DR-109 (docs/plan/research.md:120, go-live labels within tolerance) and DR-112 (docs/plan/research.md:123, sample size by the rule of three); neither gives a scope for parameter sets, and phase-16.md Decisions has no BTL scope entry.
- Fix: phase 16, section 3.5 step 4 or a Decisions entry: say whether a minor risk-tagged set needs a below-the-line sample (L-18 says every change; DSL D.3 says replay only inside the band).


### V-16-04-a  PARTLY   (first agent: MEDIUM; refuter: LOW)
- Claim: Two shadow-start refusals name no code: the version is not `testing`, and its unit cases did not pass (`tests_passed_at` NULL). The 3.17 reason list has no such reason.
- Evidence: Part refuted (status): a version outside `testing` answers 409 `E_NOT_ALLOWED_IN_STATE` by the general code list, API.md:982 "| `E_NOT_ALLOWED_IN_STATE` | 409 | Action not allowed in the resource's status |". Part refuted (unit cases): a `testing` row cannot have a NULL `tests_passed_at`, so the second refusal cannot arise after the status check (phase-16.md:97, `CHECK (status NOT IN ('testing', 'shadow', 'live') OR tests_passed_at IS NOT NULL)`; start step 1 at phase-16.md:1357). Part stands: the reason token for a version not in `testing` is not in the closed list, phase-16.md:1577 "`E_NOT_ALLOWED_IN_STATE` (409; `reason` one of `not_live`, `no_fallback`, `draft_open`". Greps: `not_testing`, `tests_missing`, `must be testing` in phase-16.md (none).
- Fix: phase 16, section 3.17 reason list: add the reason for a version that is not `testing`, and say the unit-case check is carried by the status check.

### V-16-05-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `POST .../test` has no run steps. Section 3.12 runs seeded cases only, no unit-case run, pass criterion or failure response is written, and the studio's automatic cases are not carried.
- Evidence: Part refuted (pass criterion and failure response): phase-16.md:1027 "section 3.1 rules 1 to 16 with zero errors; three positive and three negative unit cases" with `E_TESTS_MISSING`, and the route row at phase-16.md:890 (`draft` to `testing` on pass). Part stands (run steps): the only run procedure is for seeded rows, phase-16.md:1391 "**Run.** For each active case of the scenario:" with steps 1-4 on `scn.seeded_tests`; `grep -n -i unit` over phase-16.md:1384-1410 gives none. The unit-case format sits in the fixtures (DSL.md:418-435, a `tests.unit` array of fixture and expect pairs), and no step runs it. Part stands (automatic cases): DSL.md:431 "the studio's automatic checks add null, currency, reversal and late-data cases", and phase 16 has no such text (greps `null, currency`, `automatic checks`: none).
- Fix: phase 16, section 3.12 (or a step list beside it): one run procedure for unit cases and a line naming the four automatic case kinds that DSL A.9 requires.

### V-16-06-a  PARTLY   (first agent: MEDIUM; refuter: LOW)
- Claim: The refusal `pack_not_enabled` has no HTTP status and no problem code, and it is not in the 3.17 reason list.
- Evidence: Part refuted (status and code): gate failures on state conditions are `E_NOT_ALLOWED_IN_STATE`: phase-16.md:1054 "Any failure is `E_NOT_ALLOWED_IN_STATE` and the decision is refused.", and phase-16.md:1064-1066 says the same for state conditions with a reason. The G9 row uses the same code: phase-16.md:1078 "| G9 pack | `pack: akr` only with `pack.akr_live` on | `E_NOT_ALLOWED_IN_STATE` `pack_not_agreed` |". Part stands (reason token): `pack_not_enabled` is named at phase-16.md:1952 (block 7.2) and at phase-26.md:498, but it is not in the reason list at phase-16.md:1577-1578. Greps: `pack_not_enabled` in phase-16.md (line 1952 only).
- Fix: phase 16, section 3.17 reason list: add `pack_not_enabled` as a G9 reason under `E_NOT_ALLOWED_IN_STATE`.

### V-16-07-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: "a finding refuses the decision" names no status and no problem code.
- Evidence: Part refuted (status): phase-16.md:1465-1466 "`check_before_decision` re-runs them: a moved base or a changed document refuses the decision (`E_STALE_BASE` or `E_VALIDATION`)", and phase-16.md:1064-1066 make document failures `E_VALIDATION` with errors[]. A protected-attribute finding is a document check, so the status is E_VALIDATION by these rules. Part stands (finding code): the check's output is a list of refusals with no code (phase-24.md:671), and block 7.3 says only that a finding refuses the decision (phase-16.md:1957). Greps: `E_` codes in phase-24.md section 3.5 (none for the check); no protected-attribute code in phase-16.md or the 3.17 list, and the 3.17 list allows only DSL codes in `errors[]` (phase-16.md:1575-1576).
- Fix: phase 16, block 7.3: name the errors[] code of a protected-attribute finding inside the E_VALIDATION answer.


### V-16-10-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `touch_id` has no `admin.column_classification` row. The 0057 list for `scn.scenario_runs` classifies the other columns and not `touch_id`.
- Evidence: Nothing refutes it. The column is defined (block 7.9 says migration 0055 adds it, phase-16.md:1986 "Migration `0055` adds `touch_id UUID` to `scn.scenario_runs`"), and the 0057 list for the table has no `touch_id`, phase-16.md:634 "('scenario_runs', 'internal', ARRAY['id','scenario_version_id','mode','batch_id'" (the list continues to `version` without `touch_id`). The rule that makes the row mandatory is phase-00.md:797-798, "every column of every table in the twelve AKR schemas has a `column_classification` row", so the schema check would fail. No 7.x block adds the row. Greps: `touch_id` in phase-16.md lines 621-668 (none); `column_classification` for `touch_id` (none). Severity stays MEDIUM because the rule is in phase 00 and the list pattern is adjacent.
- Fix: phase 16, section 1.4 migration 0057, scenario_runs `internal` list: add `touch_id`.

### V-16-10-b  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The fan-out input does not name `touch_id`. The 3.6 fan-out step 1 and the 4.1 payload of `scenario_touch_fanout` list `batch_id, source_id, entity_kinds, business_date`. The `scenario_run` payload has no `touch_id`.
- Evidence: Part refuted: block 7.9 names `touch_id` as a job input: phase-16.md:1988 "The fan-out passes `touch_id` into every `scenario_run` job it enqueues", and the fan-out's own key at phase-16.md:1989 "coalesce keys become `scenario_fanout:<touch_id>`". The event carries it: phase-11.md:1543-1544 "The payload of `canon.touched` therefore gains `touch_id`". Part stands: the base payload rows were not updated: phase-16.md:1130 (step 1 reads `batch_id`, `source_id`, `entity_kinds`, `business_date`), phase-16.md:1749 (the `scenario_touch_fanout` payload has the same four) and phase-16.md:1750 (the `scenario_run` payload has no `touch_id`). Block 7.9 changes the coalesce keys and says nothing about the payloads, so the table contradicts the block's own keys. Greps: `touch_id` in phase-16.md outside the block (none).
- Fix: phase 16, section 4.1 table rows for `scenario_touch_fanout` and `scenario_run` and section 3.6 step 1: add `touch_id` to both payloads.

### V-16-11-a  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: The takeover moves the run from `failed` or an expired `running` back to `running` through an UPDATE, and names no audit event. 3.17 says "one per mutation" and lists no run-start event.
- Evidence: Covered by run-level events. Every attempt ends in an audited event: phase-16.md:1171 "Audit `scenario.run_completed {run_id, version_id,", and the failure event at phase-16.md:1173. Block 7.10 keeps the earlier attempt's event: phase-16.md:2005-2006 "The earlier attempt's `scenario.run_failed` stays in the audit." The base text has no run-start event either (`grep -n "run_start"` in phase-16.md: none), so the takeover follows the phase's own design. The lease takeover is job-harness housekeeping: phase-00.md:546-547 (rows `running` past `jobs.lease_s` go back to `queued`), and the phase 00 audit catalogue lists only `job.dead` (phase-00.md:654). Residual, not a gap: the 3.17 mutation rule (phase-16.md:1585) does not say whether ledger transitions count.

### V-16-11-b  PARTLY   (first agent: LOW; refuter: LOW)
- Claim: The scenario shows "not operating as intended" (section 3.16), but section 3.16 has no such rule. The phrase is used at phase-16.md:1084, 1158 and 1239, and DSL A.3 is its source.
- Evidence: Part refuted (the rule exists): DSL.md:207 'shown as "not operating as intended" and are never scored low because of it' sits in DSL A.3 (heading at DSL.md:199), and the card rule is at DSL.md:202-203. Phase 16 cites that source at phase-16.md:1084 "(DSL A.3; phase 06 edge case)" and at phase-16.md:1239 (DSL A.3). Part stands (the cross-reference): phase-16.md:1158 cites section 3.16 for the phrase, and block 7.10 (phase-16.md:2006-2007) does the same; section 3.16 (phase-16.md:1536-1571) does not contain the phrase (`grep -n "not operating"` in that range: none).
- Fix: phase 16, section 3.6 (line 1158) and block 7.10: replace the reference to section 3.16 with DSL A.3.

### V-16-12-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The closed-case skip is assigned to phase 17 ("From phase 17 on"), but phase 17 has no `open_bucket` text and no fold or intake step that skips a closed case.
- Evidence: Nothing refutes it. phase-16.md:2014 "From phase 17 on, the query also skips a key whose alert" names phase 17 as the owner. Phase 17 supplies the tables (phase-17.md:181 `alert.alert_index`, phase-17.md:298 `status IN ('open', 'in_review', 'closed', 'merged')`), but no step uses them for the fold: `grep -n open_bucket docs/plan/phase-17.md` gives none. The intake identity step (phase-17.md:882, `insert_or_get`) has no skip. The case side is covered (phase-17.md:1034 "When a closed and sealed case of the same party and family exists"), but the alert side is not. Greps: 'closed' in the bucket lines of phase-17.md (883, 920, 978, 1337, 1574, 1862): none.
- Fix: phase 17, section 3.1 step 2: one sentence that the identity lookup skips a key whose alert sits in a closed case, as block 7.11 of phase 16 states.

## Summary
- Claims checked: 16 (one per GAP line in tools/agent/verify/phase-16.md; each Gap ID handled once).
- REFUTED: 2 (V-16-03-c, V-16-11-a)
- STANDS: 2 (V-16-10-a, V-16-12-a)
- PARTLY: 12 (V-16-01-a, V-16-01-b, V-16-01-c, V-16-03-a, V-16-03-b, V-16-03-d, V-16-04-a, V-16-05-a, V-16-06-a, V-16-07-a, V-16-10-b, V-16-11-b)

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-16-01-a | PARTLY | MEDIUM | No one-row repository method for the scenario insert; no duplicate-code refusal (`code` is UNIQUE) | 3.15 step 2; section 2 repository list |
| V-16-01-b | PARTLY | MEDIUM | `ScenarioCreate` lists no fields; the document field is in `NewScenarioVersion` but the body is not stated | section 2 utoipa schemas (line 868) |
| V-16-01-c | PARTLY | MEDIUM | 422 `E_VALIDATION` is the general answer, but `owner_not_active` is not a DSL code allowed in `errors[]` | 3.15 step 1; 3.17 list |
| V-16-03-a | PARTLY | MEDIUM | Drafting step 4 states the reason rule with no refusal; the draft is saved and refused only at proposal | 3.5 step 4 |
| V-16-03-b | PARTLY | MEDIUM | `set_version` race between two drafts is not handled; the unique violation has no mapped response (transaction part is covered by phase 00 block 7.17) | 3.5 step 5 |
| V-16-03-d | PARTLY | MEDIUM | Minor risk-tagged sets: L-18 says every change needs a sample, DSL D.3 says replay only inside the band; no decision picks one | 3.5 step 4 or Decisions |
| V-16-04-a | PARTLY | LOW | Reason token for a version not in `testing` is missing from the closed 3.17 list (status code is general) | 3.17 reason list |
| V-16-05-a | PARTLY | MEDIUM | No run procedure for unit cases; the studio's automatic null, currency, reversal and late-data cases (DSL.md:431) are not carried | 3.12 or a step list beside it |
| V-16-06-a | PARTLY | LOW | Reason token `pack_not_enabled` missing from the 3.17 list (status is E_NOT_ALLOWED_IN_STATE by the general gate rule) | 3.17 reason list |
| V-16-07-a | PARTLY | MEDIUM | The protected-attribute finding has no `errors[]` code; status is E_VALIDATION by the common rule | block 7.3 |
| V-16-10-b | PARTLY | MEDIUM | 4.1 payload rows for `scenario_touch_fanout` and `scenario_run` omit `touch_id` that block 7.9 uses | 4.1 table; 3.6 step 1 |
| V-16-11-b | PARTLY | LOW | Cross-reference to section 3.16 for "not operating as intended"; the rule is in DSL A.3 | 3.6 (line 1158); block 7.10 |
| V-16-10-a | STANDS | MEDIUM | `touch_id` has no `column_classification` row in the 0057 list | 1.4 migration 0057, scenario_runs internal list |
| V-16-12-a | STANDS | MEDIUM | Phase 17 carries no step for the closed-case skip that block 7.11 assigns to it | phase 17 section 3.1 step 2 |
