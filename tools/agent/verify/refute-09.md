# Refuter, phase 09 (Haiku draft for lead review)

### V-09-01-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The Hijri variant name that must be stored with each Hijri label has no column or encoding rule; the DDL has only `hijri_label TEXT`.
- Evidence: docs/plan/phase-09.md:434 "`HijriVariant` (the variant label stored with every Hijri text)" and docs/plan/phase-09.md:724 "stored with its variant name" (the type and the storage rule exist, so the existence part is refuted). Stands: the only column is docs/plan/phase-09.md:128 "CHECK (hijri_label IS NULL OR length(hijri_label) <= 80)", and no text says whether the variant sits inside `hijri_label` or in its own column. Greps: "hijri_variant" and "variant_name" in docs/plan: none; research.md "variant": only DBS decision 365 (StatementKind), unrelated.
- Fix: Phase 09, section 1 (`ref.calendar_days`): one sentence saying whether the variant name is its own column or part of `hijri_label`.

### V-09-01-b  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The calendar refusals (weekend set, week floor, currency key, duplicate day, ramadan order, 400-row cap) have no HTTP status and no `errors[]` code.
- Evidence: The status is covered by a general rule: docs/plan/phase-00.md:499 "`Validation(Vec<ValidationError>)` | 422 | `E_VALIDATION`" (refutes the HTTP part). Stands: the rules are at docs/plan/phase-09.md:718 "the weekend lists are subsets of `1..=7` of size 1 to 3" and docs/plan/phase-09.md:722 "at most 400 per request"; the 3.8 row at docs/plan/phase-09.md:926 names no calendar code; docs/API.md:985 says `errors[]` codes are the DSL.md section 7 codes, and that section has no weekend, ramadan, duplicate or row-cap code. The nearest DSL code, `E_DANGLING_REF` (docs/DSL.md:1100), does not name currencies.
- Fix: Phase 09, section 3.8 error table: name an `errors[]` reason code for each calendar refusal (weekend size, week floor, unknown currency key, duplicate day, ramadan order, row cap).

### V-09-01-c  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: The CBS capture inserts `ref.cbs_calendar_captures` rows and names no audit event; phase 05 names only run-level kinds.
- Evidence: docs/plan/phase-09.md:671 "Capture (akr-ingestd, inside phase 05's `discovery_run`)." The capture is one step of that run, and the run's audit event is named: docs/plan/phase-05.md:1038 "audit `discovery_run.completed {objects, incomplete, statements, findings_by_severity}`". Phase 05 treats its other per-object discovery writes the same way: docs/plan/phase-05.md:1032 "writes progress after every object". Note for the lead: the completed event does not count the captured rows; that is a wording point, not a missing event.

### V-09-01-d  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The `BusinessClock` method is `add_working` in phases 00 and 04 but `add` in phase 09, and phase 09 section 4 does not list the rename.
- Evidence: The new name is designed: docs/plan/phase-09.md:459 "fn add(&self, from: DateTime<Utc>, n: u32, unit: BusinessUnit) -> Result<ClockResult, ClockError>;", and docs/plan/phase-17.md:2030 "Clocks use `BusinessClock::add` (phase 09), not" (a decision that names the winner). Stands: docs/plan/phase-00.md:567 and the two phase 04 calls (docs/plan/phase-04.md:923 and :1108) still use `add_working`. The section 4 phase 00 and phase 04 bullets (docs/plan/phase-09.md:1079-1103) do not mention the rename. Greps: "add_working" in docs/plan/phase-09.md: none.
- Fix: Phase 09, section 4 phase 00 and phase 04 bullets: one sentence saying the trait method is `add` and that the phase 00 trait line and the two phase 04 calls change to it.

### V-09-01-e  PARTLY   (first agent: LOW; refuter: LOW)
- Claim: `CaptureRun` (return type of `latest_run`) and `CaptureObject` (parameter of `rows`) are named but not defined.
- Evidence: CaptureObject's values are stated in a code block: docs/plan/phase-09.md:146 "CHECK (object_key IN ('holiday', 'currency_week'))" (refuted for the values). Stands: no text names the fields of `CaptureRun`; docs/plan/phase-09.md:530 "Result<Option<CaptureRun>, RepoError>" and docs/plan/phase-09.md:680 "Take the latest completed run of the source" give only its role. Greps: "CaptureRun" and "CaptureObject" in docs/plan: phase-09.md:530-531 only.
- Fix: Phase 09, section 2 `CaptureRepository`: one line naming the `CaptureRun` fields (for example run id, status, finished time, source id).

### V-09-02-a  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: The emergency revert inserts a fresh override row with no named audit event; section 3.8 names `country_override.applied` for CR activation only.
- Evidence: docs/plan/phase-09.md:638-639 "inserts a fresh row with that level and the old expiry" is one effect of the revert, and the revert is a named trigger of the event: docs/plan/phase-09.md:939 "| `country_override.ended` | supersede, revert or expiry sweep (reason) | country |". General rule: docs/plan/phase-04.md:933 "`target.revert` (restores the base version, writing its own audit event)". Note for the lead: the event names the revert, not the reinstated row in its detail keys.

### V-09-02-b  REFUTED   (first agent: LOW; refuter: none)
- Claim: `NewOverride` (insert input) and `EndReason` are named but not defined.
- Evidence: EndReason's values are stated: docs/plan/phase-09.md:487 "Ends the open row (ended_reason = superseded | reverted | expired)" and docs/plan/phase-09.md:56 "CHECK (ended_reason IN ('expired', 'superseded', 'reverted'))". NewOverride's fields are the columns of the table at docs/plan/phase-09.md:48-63 (level, direction, reason, expires_at, change_request_id, applied_by), and the apply hook names two of them: docs/plan/phase-09.md:633 "`insert_override` with `applied_by` = the final approver and `change_request_id`". The `NewOverride` of phase 15 (docs/plan/phase-15.md:933) is another module's type, not a gap for this item.

### V-09-03-a  PARTLY   (first agent: HIGH; refuter: HIGH)
- Claim: The routing part of `policy-pack.v1` has no keys, value shape, example or consumer.
- Evidence: The branch-first routing switch does have a key and a value shape: docs/plan/phase-09.md:775-776 "`branch_first_routing` (boolean, shipped `false` and `fixed: true` in this release" (refuted for the switch). Stands: the queue routing is named only as "Routing (the queues of phase 17 and the branch-first switch of M-14, referencing pools and queues that exist)" (docs/plan/phase-09.md:762-763), with no key name or shape, and the closed list names no separate routing key (docs/plan/phase-09.md:762-767). No consumer reads it: phase 17 takes its pool from the `casework.queues` filter (docs/plan/phase-17.md:1052-1053), and docs/plan/phase-17.md:1180 "Queues are created and changed only through the change target `queue_definition`" conflicts with a pack that holds the queues.
- Fix: Phase 09, section 3.5 "What a pack holds": name the routing key and its value shape, and say whether the pack holds queues or only references phase 17 queues (phase 17 says queues change only through `queue_definition`).

### V-09-04-a  PARTLY   (first agent: HIGH; refuter: HIGH)
- Claim: Phase 03 says `NoMatrix` passes and `window_limit_days` returns `Option<u32>`; phase 09 says delegation is refused and the port returns `Result`.
- Evidence: The refusal is designed and phase 09 says phase 03 changes: docs/plan/phase-09.md:874-875 "No current matrix -> `DelegationRefusal::NoMatrix`. Delegation is refused until the first matrix is signed (fail closed;" (refutes the NoMatrix-passes conflict; see also docs/plan/phase-09.md:1084 and :1271). Stands: the return type conflicts and no text picks a winner: docs/plan/phase-09.md:467 "fn window_limit_days(&self, roles: &[RoleCode]) -> Result<u32, DelegationRefusal>;" against the 7.6 block at docs/plan/phase-03.md:1315-1316 "window_limit_days(roles: &[RoleCode]) -> Option<u32>". The base text still gives the old rule at docs/plan/phase-03.md:941.
- Fix: Phase 03, section 7.6: replace the `Option<u32>` signature and the `None` path with phase 09 section 3.6's `Result<u32, DelegationRefusal>` form, and change the test that asserts `None`.

### V-09-04-b  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Authority matrix validator refusals (role outside the 19, `max_days` range, `max_priority`, `second_review`) have no `errors[]` code.
- Evidence: The unknown-role refusal fits a DSL code, and API.md says `errors[]` uses those codes: docs/DSL.md:1100 "Reference to an unknown reason code, segment, calendar, code map, template, feature, role or pool" and docs/API.md:985 "`errors[]` (DSL.md s.7 codes)" (refutes the role part). Stands: the `max_days` range (docs/plan/phase-09.md:864; the nearest DSL code, `E_PARAM_OUT_OF_BOUNDS` at docs/DSL.md:1116, covers scenario parameters only), the `max_priority` rule (:865) and the `second_review` rule (:866) have no code; the 3.8 row at docs/plan/phase-09.md:926 names none for them, and no matrix schema file is named (docs/plan/phase-09.md:399-400 names only the policy-pack and reference-release schemas).
- Fix: Phase 09, section 3.8 error table: name an `errors[]` code for the `max_days` range, `max_priority` and `second_review` refusals.

### V-09-04-c  REFUTED   (first agent: LOW; refuter: none)
- Claim: `NewAuthorityMatrix` (insert input), `Permit` and `Refused` (result of `permits`) are named but not defined.
- Evidence: The result of `permits` is stated: docs/plan/phase-09.md:868 "a grant with a role in `roles`" with docs/plan/phase-09.md:869 "The result carries `second_review`." `Refused` is the negation of that rule. The insert input's fields are the columns of the table at docs/plan/phase-09.md:300-313 (for example `signed_by` at :307). `Permit` also names a type of phase 05 (docs/plan/phase-05.md:944), a different module; that is a name clash, not a missing definition.

### V-09-05-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Policy pack validator refusals in steps 1 to 7 (unknown keys, bounds, calendar, reason code, matrix version, template keys and placeholders, term ids, priority labels, schema types) have no code.
- Evidence: Covered by a named code: an unknown key is `unknown_key` (docs/plan/phase-09.md:801 "An unknown key -> `E_VALIDATION`, `unknown_key`."); a missing calendar fits `E_DANGLING_REF` (docs/DSL.md:1100 "Reference to an unknown reason code, segment, calendar"); a one-language text fits `E_MISSING_I18N` (docs/DSL.md:1098 "`ar` or `en` missing in a human-visible string"). Schema types and lengths fit `E_SCHEMA` (docs/DSL.md:1096). Stands: the clock bounds (docs/plan/phase-09.md:802; the nearest DSL code, `E_PARAM_OUT_OF_BOUNDS` at docs/DSL.md:1116, covers scenario parameters only), the inactive reason code and the missing `authority_matrix_version` (docs/plan/phase-09.md:806), the placeholder allow-list and duplicate template keys (docs/plan/phase-09.md:807), and the duplicate term ids and empty forbidden list (docs/plan/phase-09.md:810). No DSL section 7 code names a matrix or an allow-list.
- Fix: Phase 09, section 3.8 error table: add one row naming an `errors[]` code for the stand items (bounds, inactive reason code, missing matrix version, placeholder allow-list, duplicates, empty forbidden list).

### V-09-05-b  STANDS   (first agent: LOW; refuter: LOW)
- Claim: The dry run cites "E_COST_GUARD ... (phase 04 pattern)", but phase 04 has no cost-guard pattern.
- Evidence: docs/plan/phase-09.md:818 "with the estimate in the body (phase 04 pattern)." Greps "cost", "guard" and "estimate" in docs/plan/phase-04.md: no cost-guard pattern (the only "guard" hit is docs/plan/phase-04.md:1222, about a rework loop). The code is defined in docs/API.md:992 "`E_COST_GUARD` | 422 | Replay or query estimate above the cost guard (estimate in the body)" and first used for replays in docs/plan/phase-11.md:1062.
- Fix: Phase 09, section 3.5 dry run step 1: point the citation at docs/API.md section 6 (or phase 11 section for replays) instead of "phase 04 pattern".

### V-09-05-c  PARTLY   (first agent: LOW; refuter: LOW)
- Claim: `NewPolicyPack`, `PolicyPackSummary`, `Validated` and `HistorySample` are named but not defined; `ValidationIssue` is not said to be a specific type.
- Evidence: HistorySample's fields are named: docs/plan/phase-09.md:475 "NoHistory in this phase: rows = 0" (refuted for that type). NewPolicyPack's fields are the columns of the table at docs/plan/phase-09.md:280-298 (origin CHECK at :292). Stands: `PolicyPackSummary` (docs/plan/phase-09.md:573) and `Validated` (docs/plan/phase-09.md:799) have no field list. `ValidationIssue` is defined by the DSL validator (docs/plan/phase-16.md:922 "`ValidationIssue { path, code, message, severity }`") and phase 04 re-exports it under another name (docs/plan/phase-04.md:768); phase 09 does not say which one it uses.
- Fix: Phase 09, section 2 `PolicyPackRepository` and section 3.5: give the fields of `PolicyPackSummary` and `Validated`, and name the `ValidationIssue` type the validator returns.

### V-09-06-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: The four kind-level inventory items cannot pass the GV-01 gate: phase 04 keys an item by a UUID `object_id` per `(kind, object_id)`, while phase 09 keys its targets by strings such as `country:<iso2>`.
- Evidence: docs/plan/phase-04.md:533 "CONSTRAINT uq_inventory_items_object UNIQUE (kind, object_id)" (one item per kind and UUID `object_id`); docs/plan/phase-04.md:658 "pub object_id: Option<Uuid>, pub object_key: String" (a change carries an optional UUID beside the string key); the gate at docs/plan/phase-04.md:886-887 "no `approved_for_use` inventory record for the object". Phase 09's object keys are strings (docs/plan/phase-09.md:901 `country:<iso2>`, :906 `authority_matrix:active`), and the registration rule gives no object_id (docs/plan/phase-09.md:1069-1070, and the open doubt at :1357). Greps: "object_id" in docs/plan/phase-09.md: none; "uuidv5" and "namespace" in phase 04 and phase 09: none.
- Fix: Phase 09, section 4 "Evidence and inventory": state the UUID `object_id` each kind-level item carries and how the gate maps an `object_key` to it, or give phase 04 section 3.2 step 3 a rule for kind-level items.

### V-09-07-a  PARTLY   (first agent: LOW; refuter: LOW)
- Claim: `PolicyPackService::scan(scope, text)` has no pack-version input, and the text does not say whether a scan reads the active version or the version a case is pinned to.
- Evidence: The pinning rule covers cases: docs/plan/phase-09.md:831 "Cases pin the version they open under (phase 18)" and docs/plan/phase-17.md:306 "pinned at opening (M-75)" (refuted for case reads). Stands: the scan has no version input (docs/plan/phase-09.md:797 "through `PolicyPackService::scan(scope, text)` at save time"), and its callers pass none (docs/plan/phase-18.md:668, docs/plan/phase-19.md:1074). No text says which version a save-time scan reads.
- Fix: Phase 09, section 3.5 ForbiddenWordScanner step 4: one sentence stating whether a scan reads the active version or the calling case's pinned version.

### V-09-08-a  PARTLY   (first agent: HIGH; refuter: MEDIUM)
- Claim: The reference seeds verify SHA-256 only; no step checks the release manifest signature before anything is written; phase 06 does this.
- Evidence: A general rule covers the bundle: docs/plan/phase-00.md:752-755 "verifies the manifest signature and every image signature" and "any failure refuses before anything changes" (refutes "no check at all"). Stands: the seed does not name it. docs/plan/phase-09.md:612 states a signed release but has no signature step, whereas phase 06 names the check in its seed (docs/plan/phase-06.md:510-511). The bundle list at docs/plan/phase-00.md:748 names adapter packs, not the reference packs. The `verify-data` command checks SHA-256 only (docs/plan/phase-09.md:1193). Greps: "cosign", "manifest signature" and "bundle verify" in docs/plan/phase-09.md: none.
- Fix: Phase 09, section 3.1 seed paragraph: add the manifest signature check as a first step, as phase 06 section 3.1 does, or state that the seed runs only after `bundle verify` and that the packs are in the signed bundle.

### V-09-08-b  PARTLY   (first agent: HIGH; refuter: HIGH)
- Claim: The release file shapes are not defined: `reference-release.v1.json` has no keys, the seed installs from `packs/reference/*.json` with no field list, and the bootstrap pack's keys are missing.
- Evidence: Stands: docs/plan/phase-09.md:399 "reference-release.v1.json     # validates the release files of packs/reference" names the schema without keys; docs/plan/phase-09.md:613 "from `backend/packs/reference/*.json`" gives no field list or manifest format; no release id rule (docs/plan/phase-09.md:42 gives only "length(release_id) BETWEEN 1 AND 64"). Refuted for the bootstrap pack: docs/plan/phase-09.md:767 "The document is closed: `policy-pack.v1` has exactly these keys." and the key groups named at docs/plan/phase-09.md:762-766.
- Fix: Phase 09, section 2 "Crate placement" or section 3.1: give the keys of `reference-release.v1.json` (the table columns will do), the manifest format and the release id rule in one place.

### V-09-08-c  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: A later release is not numbered: insert, update (version bump), no-op and removed-entry rules and the audit counts are not stated.
- Evidence: Base-level change and its event are stated: docs/plan/phase-09.md:614 "A release that changes a base level is the trigger for re-scoring (TR-10): the" and docs/plan/phase-09.md:615 "seed writes the outbox event `ref.country_level_changed {iso2, from, to}`". Audit counts are stated for any seed run (docs/plan/phase-09.md:946). Removal is covered by a general rule (docs/plan/phase-09.md:34-35: reference rows are never deleted, only ended, retired or deactivated by a change request). Stands: how a later release inserts new rows, whether `countries.version` bumps on a changed level, and what an unchanged row does. Greps: "upgrade" in docs/plan/phase-09.md: none; the "second run" rule covers the same release only (docs/plan/phase-09.md:1192).
- Fix: Phase 09, section 3.1 seed paragraph: one sentence each for new, changed (with version bump), unchanged and removed entries of a later release.

### V-09-08-d  STANDS   (first agent: LOW; refuter: LOW)
- Claim: No type or service is named for the seed or for `verify-data`; a developer must invent the loader type.
- Evidence: Greps "ReferenceSeed", "ReleaseLoader" and "SeedService" in docs/plan: none; "upgrade" in docs/plan/phase-09.md: none. The commands are only listed: docs/plan/phase-09.md:420 "+ ref seed, ref verify-data, policy-pack seed" and docs/plan/phase-09.md:1051 "`ctl`: `ref seed`, `ref verify-data`, `policy-pack seed`."
- Fix: Phase 09, section 2 "Crate placement": name the owner of the seed and of `verify-data` (for example the existing `ReferenceService` and `PolicyPackService`).

### V-09-09-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: A document that omits a fixed clock has no rule or code; the four new fixed clocks are new keys that an existing bootstrap or version file would lack.
- Evidence: The rule exists: docs/plan/phase-09.md:773-774 "the document must carry that value for it, and any other value is refused with `E_INVARIANT_LOCKED`" (refutes "no rule"). Stands: the only row, docs/plan/phase-09.md:927 "A fixed clock or switch changed | 422 | `E_INVARIANT_LOCKED`", covers a changed value, and no refusal or code is named for an absent one. The new fixed clocks are listed at docs/plan/phase-09.md:1282 "`freeze.limit`: hours, 24, `fixed: true`" and :1283-1285. The closed schema that would mark them required is not defined (docs/plan/phase-09.md:400 names the file only).
- Fix: Phase 09, section 3.8 error table: add a row giving the status and `errors[]` code for a missing fixed clock or switch.

### V-09-11-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The 7.4 block contradicts the base text it extends: it says a walk past the earliest day returns "the same error as `add` past the horizon", while section 3.3 says `add` past the horizon returns a flagged time.
- Evidence: The two rules cover different ends of the calendar, so step 7 is not replaced: docs/plan/phase-09.md:706 "If the walk passes `covered_until` before `n` is reached" is the forward end. The earliest-day rule, docs/plan/phase-09.md:1308 "A walk past the earliest day the calendar holds returns the same error", already says the outcome is an error, not a flagged date (refutes "phase 16 cannot know"). Stands: the referent is wrong, since `add` past the horizon returns no error ("The clock still returns a time", docs/plan/phase-09.md:707), and no `ClockError` variant is named: docs/plan/phase-09.md:459 "Result<ClockResult, ClockError>".
- Fix: Phase 09, section 7.4: replace the "same error as `add`" reference with a named `ClockError` variant for the earliest-day case.

### V-09-11-b  STANDS   (first agent: LOW; refuter: LOW)
- Claim: "The earliest day the calendar holds" is not defined; section 3.3 step 4 defines `covered_until` but no earliest date, and the backward closed-day load window is not stated.
- Evidence: Greps "covered_from" and "earliest" in docs/plan/phase-09.md: only docs/plan/phase-09.md:1308. Section 3.3 step 4 gives the forward window only: docs/plan/phase-09.md:697 "from `d0` to `d0 + 3n +" (continued at :698 "14` days"). `CalendarRepository::days_between` (docs/plan/phase-09.md:519) takes a range, and no method returns the earliest date.
- Fix: Phase 09, section 7.4: one sentence defining the earliest day (the minimum `on_date` of the calendar's rows) and the backward load window (a mirror of section 3.3 step 4).

## Summary

- REFUTED: 4 (V-09-01-c, V-09-02-a, V-09-02-b, V-09-04-c)
- STANDS: 4 (V-09-05-b, V-09-06-a, V-09-08-d, V-09-11-b)
- PARTLY: 15 (the other 15 claims)
- Total: 23 unique Gap IDs (the first agent's count of 23 GAP lines is right; V-09-10 and V-09-12 carry none).

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-09-01-a | PARTLY | MEDIUM | Where the variant name is stored (own column or inside `hijri_label`) is not stated | Phase 09 section 1, `ref.calendar_days` |
| V-09-01-b | PARTLY | MEDIUM | No `errors[]` code for the five calendar refusals | Phase 09 section 3.8 error table |
| V-09-01-d | PARTLY | MEDIUM | Phase 00 and phase 04 still call `add_working`; section 4 does not list the rename | Phase 09 section 4, phase 00 and phase 04 bullets |
| V-09-01-e | PARTLY | LOW | `CaptureRun` has no field list | Phase 09 section 2 `CaptureRepository` |
| V-09-03-a | PARTLY | HIGH | Queue routing key has no name, shape or consumer; "holds" conflicts with phase 17 | Phase 09 section 3.5 "What a pack holds" |
| V-09-04-a | PARTLY | HIGH | `window_limit_days` return type conflicts (`Result` in phase 09, `Option` in phase 03 7.6) | Phase 03 section 7.6 |
| V-09-04-b | PARTLY | MEDIUM | No `errors[]` code for the `max_days`, `max_priority` and `second_review` refusals | Phase 09 section 3.8 error table |
| V-09-05-a | PARTLY | MEDIUM | No code for clock bounds, inactive reason code, missing matrix version, placeholder allow-list, duplicates, empty forbidden list | Phase 09 section 3.8 error table |
| V-09-05-c | PARTLY | LOW | Fields of `PolicyPackSummary` and `Validated`; which `ValidationIssue` is used | Phase 09 section 2 `PolicyPackRepository` and section 3.5 |
| V-09-07-a | PARTLY | LOW | `scan` has no version input; no rule says which version a save-time scan reads | Phase 09 section 3.5, ForbiddenWordScanner step 4 |
| V-09-08-a | PARTLY | MEDIUM | The seed does not name the manifest signature check (phase 06 does); packs are not named in the signed bundle | Phase 09 section 3.1 seed paragraph |
| V-09-08-b | PARTLY | HIGH | `reference-release` file keys, manifest format and release id rule are not defined | Phase 09 section 2 "Crate placement" or section 3.1 |
| V-09-08-c | PARTLY | MEDIUM | Later release: new rows, version bump, unchanged rows | Phase 09 section 3.1 seed paragraph |
| V-09-09-a | PARTLY | MEDIUM | No code or outcome for a document that omits a fixed clock | Phase 09 section 3.8 error table |
| V-09-11-a | PARTLY | MEDIUM | 7.4 names an error by reference to `add`, which returns none; no `ClockError` variant named | Phase 09 section 7.4 |
| V-09-05-b | STANDS | LOW | The "phase 04 pattern" cited for `E_COST_GUARD` does not exist in phase 04 | Phase 09 section 3.5, dry run step 1 |
| V-09-06-a | STANDS | HIGH | Kind-level inventory items have no UUID `object_id`, so the GV-01 gate cannot find them | Phase 09 section 4 "Evidence and inventory" |
| V-09-08-d | STANDS | LOW | No type or owner named for the seed and for `verify-data` | Phase 09 section 2 "Crate placement" |
| V-09-11-b | STANDS | LOW | "Earliest day" and the backward load window are not defined | Phase 09 section 7.4 |
