# Refuter, phase 15 (Haiku draft for lead review)

### V-15-01-a  PARTLY   (first agent: HIGH; refuter: MEDIUM)
- Claim: No repository trait for `kyc.exit_register` (insert, get, list) and no declared service or query type for the route. `ExitService` is named only in table 1.5, and `exit: Arc<ExitQuery>` has no definition.
- Evidence:
  - Refuted part (service named as owner): phase-15.md:884 "`ExitService` through `POST /exit-register` (M), from a completed `exit_decision` instance". The owner of the table and of the POST route is declared by name.
  - Stands part (no repository, no method list): `grep -rn "ExitRepository\|ExitRepo\|ExitService" docs/plan/` finds only line 884. The six traits of section 2 (phase-15.md:946, 956, 969, 980, 993, 1007) include no exit trait. `ExitQuery` appears only at phase-15.md:1817 with no definition; `RatingQuery` at 1815 is undefined too, so the query-type gap is shared.
  - Why MEDIUM and not HIGH: the table row, the route rows, `NewExitEntry` (940-941) and the step 8 insert give the shape, and the override trait shows the pattern (phase-15.md:970 "async fn insert(&self, tx: &mut Tx, o: &NewOverride, by: Uuid)"). A developer must guess the names, but can build the item.
- Fix: Phase 15, section 2, after the `OwnershipRepository` trait (line 1007): one sentence naming the exit repository's insert and keyset list, and the type that `ExitQuery` wraps.

### V-15-01-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `NewExitEntry { party_id, workflow_instance_id, reason_code }` has no `review_id`, but `kyc.exit_register.review_id` is NOT NULL and the audit uses it. The text never says where the value comes from.
- Evidence: greps that settle the rest.
  - `grep -n "review_id" docs/plan/phase-15.md`: the column is at phase-15.md:483 "review_id               UUID NOT NULL REFERENCES kyc.kyc_reviews(id) ON DELETE RESTRICT,". No line names a source for an exit entry's review id.
  - `grep -n "NewExitEntry"` finds 940 ("`NewExitEntry { party_id,") and 1049 only. Neither has a review id, and step 8 (1351-1356) does not name one.
  - The nearby hint is only indirect: phase-15.md:1339-1340 "`exit_candidate` opens the workflow instance `exit_decision`" (review completes at step 6). Nothing says the instance or the entry stores that review.
  - General rule checked: phase-04.md:338 "context JSONB NOT NULL DEFAULT '{}'::jsonb, -- subject attributes frozen at start" would carry the id, but phase 15 never names an `exit_decision` subject attribute (see V-15-06-a). So the source stays unstated.
  - Section 7.x blocks (2031 to 2255) contain no exit-register text about review ids (`grep -n -i "exit"` from line 2031 to the end lists 2031-2034, 2106-2118, 2165, 2179-2180, 2189 and 2195; none is about review ids).
- Fix: Phase 15, section 3.4 step 8 (or the `NewExitEntry` line at 940): one sentence saying whether `review_id` is the completed review that opened the `exit_decision` instance, and where the entry reads it.

### V-15-01-c  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: Step 8 requires an active `DEC_` reason code (1354), but gives no refusal code or status for a code that is inactive or of another family.
- Evidence:
  - phase-15.md:1587 "`Validation(Vec<Issue>)` | 422 `E_VALIDATION` | `errors[]` with the DSL.md s.7 Part C codes". This is the 3.10 row for input validation, so status 422 and code `E_VALIDATION` are fixed for any failed input check, the reason-code check included.
  - phase-15.md:1605 "No problem code is added. A DSL code that API.md section 6 does not list travels inside `errors[]` of `E_VALIDATION`." No new code is wanted for the reason-code check.
  - Platform rule: phase-00.md:499 "`Validation(Vec<ValidationError>)` | 422 | `E_VALIDATION` (`errors[]` with JSON pointers)".
  - The same check in 3.5 step 2 (phase-15.md:1367 "`reason_code` active in `ref.reason_codes` family") is covered by the same row.

### V-15-01-d  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: `record_decision`, the terminal effect of `exit_decision`, `kyc_override_down` and `kyc_edd_signoff`, is registered by phase 18 for resource kind `case` only, and phase 15 registers no effect for the kyc kinds.
- Evidence:
  - phase-18.md:1130 "`RecordDecisionEffect` (kind `record_decision`) and `ReleaseForFilingEffect` (kind `release_for_filing`) register with resource kind `case`." The registration covers `case` only.
  - phase-04.md:1245 "`E_EFFECT_UNAVAILABLE` | error | A terminal effect that is not registered and not deferrable, or that does not serve the family's resource kinds". So the three phase 15 definitions cannot publish as written.
  - phase-04.md:1771 "No case, report, KYC or exit workflows (phases 15, 18, 21): `case_acl` assignees, `record_decision`," so phase 04 expects phase 15 to bring the effect for KYC and exit. Phase 15 does not.
  - Phase 15 has no effect registration: `grep -n -i "regist" docs/plan/phase-15.md` has no effect among its hits (tables, indexes, change targets, masked kinds and evidence owner kinds), and `grep -n "record_decision"` gives 1347, 1351, 1389, 1794, 1797, 1798, 2032-2034 and 2143, none of which registers it.
  - No 7.x block of phase 15 changes this (7.2 at 2031-2034 names only `create_ops_task` for phase 20).
- Fix: Phase 15, section 4.2 (shipped definitions): one sentence stating which phase registers `record_decision` for `kyc_review` and `exit_decision` (phase 15 itself, or an added kind in the phase 18 registration at line 1130).

### V-15-02-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: The fan-out selects parties by "registration country" (1272), but `canon.party` has no registration-country column. The Open doubts defer it to phase 10, and phase 10 does not map it.
- Evidence:
  - The use: phase-15.md:1272 "pages of `rating.country_fanout_page` parties whose nationality, residence or registration country". The factor C2 needs it too (research/26 line 723: "C2 | Registration and ownership countries (20%) | Iraq only 10; foreign registration by country-table score").
  - The columns: phase-10.md:464-465 "registration_no_enc BYTEA," and "registration_no_hk  BYTEA,". No country column on `canon.party` (phase-10.md:455-456 has nationality and `residence_country` only).
  - The deferral: phase-15.md:2222-2223 "C2 (registration country) and C4 (branch border status) stay pending until phase 10 maps them." Phase 10 does not carry it: `grep -n -i "registration\|country" docs/plan/phase-10.md` shows no mapping. The deferral therefore does not refute the claim (the rule covers only deferral to a phase that carries the work).
  - One candidate exists but is not named: phase-10.md:529 "country             CHAR(2) REFERENCES ref.countries(iso2)" on `canon.party_identifier`, whose `type` (phase-10.md:524) is free text with no registration value defined. Phase 15 does not connect the two.
  - `grep -rn -i "registration country\|registration_country\|country_of_reg" docs/ research/`: apart from phase 15, the only hits are research 01 line 911 (a list of screening inputs, "country, registration country, border branch") and research 26 line 137 ("nationality, residence, work country, registration country", the CBI's named factors). No DBS decision fixes the source.
  - The attribute registry could carry a bank-specific value (phase-10.md:1836 "**Extension attributes.** `attribute: true` requires a definition in the registry for this entity"), but no attribute is named for the registration country anywhere in the plan, so this does not settle the gap.
- Fix: Phase 10, section 1 (`canon.party`, around line 464) or phase 15, section 3.3 row 2: one sentence naming where the registration country is stored (a new column, or the `country` of a registration row in `canon.party_identifier`) and which phase adds it.

### V-15-02-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: No repository method or SQL is named for the paged party query (parties whose nationality, residence, registration country or owner's residence is `iso2`, keyset by party id, page of `rating.country_fanout_page`).
- Evidence:
  - The query is described only in prose: phase-15.md:1272 "pages of `rating.country_fanout_page` parties whose nationality, residence or registration country". The page size setting is at phase-15.md:765 and 1749.
  - `grep -n -i "countr" docs/plan/phase-15.md` restricted to lines 942-1040 (section 2, the repository traits) returns no line.
  - No owner section has it: phase-10.md:1676-1682 (`CanonRepository`) has no country query. Its note "every list is keyset-paged" (phase-10.md:1686-1687) covers only its own lists.
  - Reading the step as "stated in prose" does not settle it: the method is called by the step at 1272 and no section names it, which is the rule in the gap list.
  - Part of the query (the registration-country arm) cannot be written until V-15-02-a is settled.
- Fix: Phase 15, section 2, after the `PartyFactsSource` port (about line 1029): one sentence naming the repository method that returns the keyset page of party ids for a country, and the four country columns it filters on.

### V-15-02-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The job payload needs `effective_at` (1272, 1766), but the consumed event `ref.country_level_changed {iso2, from, to, cause}` has no date field, and the phase does not say where the date comes from.
- Evidence:
  - The payload: phase-15.md:1766 "`rating_country_fanout` | akr-worker | `{iso2, cause, effective_at}`". The event: phase-09.md:1035 "`ref.country_level_changed {iso2, from, to, cause: release | override | expiry | revert}`", written "in the transaction that changes the effective level" (phase-09.md:1036). No date is in the payload.
  - `grep -n "effective_at" docs/plan/phase-09.md` finds 444, 445 and 830 only (proposals and a policy-pack timer), none on the country event.
  - Only a general timestamp exists: phase-00.md:129 "created_at     TIMESTAMPTZ NOT NULL DEFAULT now()," on `admin.outbox`. Phase 00 does not say consumers receive that column (`grep -n -i "consumer" docs/plan/phase-00.md` finds only 1049, a retry note), so the rule does not settle the source.
  - The K-42 requirement (research/26 line 945) names no effective date, so the requirement does not settle it either.
  - Phase 15 never says where the date comes from: `grep -n -i "effective date\|effective_at"` in phase-15.md finds 252, 258, 559, 815, 1044, 1084, 1272, 1475 and 1766; none of them names the source of the country-change date.
- Fix: Phase 15, section 3.3 row 2 (line 1272): one sentence stating where `effective_at` comes from (the outbox row's `created_at`, or a date field phase 09 adds to the event).

### V-15-02-d  STANDS   (first agent: LOW; refuter: LOW, see note)
- Claim: DSL.md:1457 (and the `on` list at DSL.md:807) names `country_table.changed` as TR-10's event, while phase 15 consumes phase 09's topic `ref.country_level_changed` and never names `country_table.changed`.
- Evidence:
  - DSL.md:1457 `{ "id": "TR-10", "on": "country_table.changed", "when": "true", "action": "rescore",` and DSL.md:807 lists `country_table.changed` among the `on` values.
  - `grep -n "country_table" docs/plan/*.md` finds only the factor anchors at phase-15.md:1127 and 1216. Phase 09 does not name the DSL event either. No text maps one name to the other.
  - No general rule maps trigger `on` names to outbox topics. Phase 15 section 3.3 lists the unwired DSL names (phase-15.md:1278) without `country_table.changed`.
- Note for the lead (not a new claim): phase-15.md:2093 "read the triggers of the live model whose `on` equals `event`" matches triggers by DSL name. The fan-out instead enqueues `rating_compute {reason: "country"}` (1272), so TR-10 is never matched and its review (DSL.md:1458 "review_within": { "value": 1, "unit": "business_days" }, "owner": "role:KYC") has no stated path. That is why the lead may want to weigh this above LOW.
- Fix: Phase 15, section 3.3 row 2 and section 4.2: one sentence stating that `ref.country_level_changed` is the topic that carries DSL `country_table.changed` (TR-10), and whether TR-10's review is opened.

### V-15-03-a  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: `OverrideService` is named at 876, 904, 1363, 1815 and 2067 with no method signatures. The upward proposal, the downward proposal and the approval handler are described in steps only.
- Evidence:
  - The three flows are fully stated in prose steps, which the rule on prose-stated methods accepts: phase-15.md:1365 "**Upward (`POST /parties/{id}/overrides`, `direction` derived from `to_level > computed_level`; SU `decision`):**" with steps 1-4 (1366-1372); phase-15.md:1376 "**Downward (same route; compliance only; one level; twelve months; evidence; four eyes):**" with steps 1-6 (1377-1390).
  - The approval runs through named parts: phase-15.md:1388 "5. Route `kyc_override_down` (shipped by this phase, section 4.2): one step, `approve`", and phase-15.md:1390 "6. Activation (`OverrideTarget::activate`, in the approval transaction)".
  - Every persistence call the steps make is named in section 2: phase-15.md:970 "async fn insert(&self, tx: &mut Tx, o: &NewOverride, by: Uuid)" and 971 "async fn set_status(&self, tx: &mut Tx, id: Uuid, expected: i32, to: OverrideStatus".
  - The route is in the route table: phase-15.md:1061 "| POST | `/api/v1/parties/{id}/overrides` | `kyc.work` | Upward: active at once (201); downward: a change request".
  - Correction to the claim's wording: one signature does exist, `OverrideService::analytics(ctx, from, to)` at phase-15.md:2067 (7.5).

### V-15-04-a  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: `PepService` is named at 882, 904, 1516, 1816 and 1821 with no method signature for the candidate, confirmation, update and relative routes. The route handlers call methods that no section defines.
- Evidence:
  - The four flows are stated in prose in 3.8: phase-15.md:1516 "### 3.8 The PEP register, relatives and the pair-memory cap (`PepService`, `PepRules` pure; MODULES M10; phase 14)", with the candidate sources at 1519-1532, "**Confirmation.**" at 1533, "**Changes and ends.**" (PATCH) and "**Relatives and close associates.**" (POST relatives) after it.
  - Every persistence call of those flows is a named method of `PepRepository` in section 2: phase-15.md:995 "async fn insert_candidate(&self, tx: &mut Tx, c: &NewPepCandidate)" (step 1, `ON CONFLICT DO NOTHING`), 996 "async fn insert(", 997 "async fn set_confirmed(", 999 "async fn update(", 1000 "async fn due_persistence(" (`pep_persistence_sweep`) and 1003 "async fn upsert_relative(" (relatives).
  - The routes are in the route table (`/api/v1/pep/register/{id}` PATCH and `/relatives` POST), so no route lacks a stated flow.
  - So the handler calls land on named repository methods; a service method name is not needed to build the item.

### V-15-04-b  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: `POST /pep/register` logic (3.8 step 3, lines 1530-1531) is one sentence with no numbered steps and no refusal code. The 422 for a `confirmed` status on create appears only in the test (1993-1994).
- Evidence:
  - The refusal code is stated, in the acceptance test: phase-15.md:1993-1994 "PEP: as KYC `POST /api/v1/pep/register` with `status: "confirmed"` -> 422 `E_VALIDATION`; without a status (a candidate) -> 201".
  - The general mapping gives the same status and code: phase-15.md:1587 "`Validation(Vec<Issue>)` | 422 `E_VALIDATION`" (3.10).
  - The rule that makes the refusal necessary is stated next to step 3: phase-15.md:1533 "**Confirmation.** A candidate becomes `confirmed` only through change request `pep_entry` (3.5)".
  - Step 3 names the check it needs (a manual or declared candidate, `source` values) and the audit event (`pep.proposed`). A single sentence that leaves nothing to guess is accepted by the rule, so the missing numbered list is not a gap.

### V-15-04-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Source 1 takes "the designation's display name from `ref.list_entry_names`" (1521-1522). That table has per-slot raw names with one `script` per row and no display-name column, and `kyc.pep_register.name` must hold both `ar` and `en` keys. A one-script designation cannot satisfy the CHECK, and the mapping is not stated.
- Evidence:
  - The CHECK: phase-15.md:381 "name JSONB CHECK (name IS NULL OR (name ? 'ar' AND name ? 'en'))".
  - The source table: phase-14.md:176 "script              TEXT NOT NULL CHECK (script IN ('ar', 'lat', 'ku', 'mixed', 'other'))," and phase-14.md:180 "UNIQUE (entry_id, name_slot)". There is no display column in phase-14.md:166-181.
  - The phrase "display name" is defined nowhere for list entries. `grep -n -i "display name" docs/plan/*.md` hits users, parties and decisions (for example phase-09.md:573, phase-18.md:891, phase-26.md:371) and the claim's own line phase-15.md:1522, with no definition. The `name_slot` hits (phase-14.md:170, 180, 574) show per-slot rows only.
  - No transliteration or script-mapping rule exists: `grep -n -i "transliterat\|translit\|both languages\|'lat'"` in phase-14, phase-15, phase-01 and phase-00 gives no rule for filling the missing language key.
  - Severity stays MEDIUM, not HIGH: the slot and script columns make the natural mapping inferable (the `ar` row and the `lat` row), but a designation with one script has no stated outcome.
- Fix: Phase 15, section 3.8 source 1 (1521-1522): one sentence naming which `ref.list_entry_names` row fills `ar` and which fills `en`, and what the sweep does when one script is missing.

### V-15-06-a  STANDS   (first agent: HIGH; refuter: HIGH, one item overstated)
- Claim: `exit_decision` needs a registered `WorkflowSubject` (party members, start context, check before decision), and so does `kyc_review` for `kyc_edd_signoff` (4.2). Phase 04 says these follow in phase 15 (phase-04.md:667), and an unregistered resource kind is refused (phase-04.md:1199). Phase 15 defines none.
- Evidence:
  - Phase 04 expects phase 15 to define them: phase-04.md:667 "case (18), report (21), kyc_review (15), exit_decision (15) follow."
  - The refusal: phase-04.md:1199 "Every `resource_kinds` entry is a registered `WorkflowSubject` (422 `unknown_resource_kind`)." The 4.2 shipped definitions name `kyc_review` and `exit_decision` (phase-15.md:1793).
  - `grep -n -i "subject" docs/plan/phase-15.md` finds no WorkflowSubject, no `ExitDecisionWorkflowSubject` and no `KycReviewWorkflowSubject`. The hits are the alert `subject_id` (1521), conflict-check parameters (1573-1578) and a party subject in an audit payload (2043).
  - Overstatement: `check_before_decision` has a default body, so the KYC subjects do not need it: phase-04.md:1644 "// default body: Ok(())" and phase-04.md:1648 "Subjects with no extra rule keep the default". The required methods without defaults (`start_context`, `party_members`, `attributes`, `parties`, `visible_to`, `visibility_sql`, `party_sql`, `on_ended`, `resource_kind`, `attribute_prefix`) are still undefined for both kinds.
- Fix: Phase 15, section 4.2 or a new section 3 block: one sentence per subject (`kyc_review`, `exit_decision`) naming its resource kind, attribute prefix and the methods it implements, with `check_before_decision` left at its default.

### V-15-06-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: 7.2 says phase 20 "ships version 2 of `exit_decision`, which adds it after `record_decision`" (2033-2034). Phase 20 registers only `create_ops_task` and says the definition calls it (phase-20.md:13 and 974). It names no version 2.
- Evidence:
  - The claim in 7.2: phase-15.md:2033-2034 "Phase 20 registers the effect `create_ops_task` and ships version 2 of `exit_decision`, which adds it after `record_decision`."
  - Phase 20's own shipped list has two definitions: phase-20.md:1108 "`freeze_release` and `ops_attestation` are shipped workflow definitions, and `release_freeze`, `record_verification` and `create_ops_task` are terminal effects". DBS decision 168 (research.md:179) says the same. No `exit_decision` version appears in phase 20.
  - Phase 20 treats the call as already present in the phase 15 definition: phase-20.md:13 "this phase only registers the `create_ops_task` effect that the `exit_decision` definition calls" and phase-20.md:974 "`create_ops_task` is registered here and called by `exit_decision` (DSL E.4)". That contradicts 7.2, which says version 1 does not name the effect.
  - `grep -n -i "version 2"` in phase-20.md finds no exit_decision version; the "v2" hits are verification levels (for example 69, 91, 97).
- Fix: Phase 15, 7.2 (2033-2034) or phase 20, shipped-definition list (line 1108): one sentence stating which phase ships `exit_decision` with `create_ops_task`, and which version, so that the two texts agree.

### V-15-06-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The approval of `exit_decision` is "per the authority matrix (phase 09)" (1798), but the closed decision-type list has no key for exit decisions. Phase 09 says a new decision type is a code change released with the phase that needs it.
- Evidence:
  - The use: phase-15.md:1798 "`exit_decision`: `approve` per the authority matrix (phase 09) -> `record_decision`". No decision-type key or approving role is named in phase 15 (`grep -n "authority matrix\|authority_matrix"` in phase-15.md finds 1798 only).
  - The closed list: phase-09.md:850 "| `kyc.decision` | kyc.work | KYC, M |" is the nearest key, but it also admits KYC, while the exit register is M-only (phase-15.md:1737). Nothing in phase 15 or phase 09 chooses it for exit decisions.
  - Phase 09 names the exit register only for the reason code, not for a decision type: phase-09.md:1294 "- Family `decision`, for the exit register (phase 15): `DEC_EXIT_FINANCIAL_CRIME`."
  - The rule: phase-09.md:856 "A new decision type is a code change released with the phase that needs it, never a document entry."
  - Phase 04 gives no matrix path for approvals: phase-04.md:1769 "No authority matrix and no bank calendar (phase 09): steps are held by role and office".
  - Severity stays MEDIUM: a developer must choose a key or add one, but the approver set is not stated for an M-only register; no rule in phases 00 to 04 decides it.
- Fix: Phase 15, section 4.2 (line 1798): one sentence naming the decision-type key the exit approval uses (an existing key, such as `kyc.decision` with its roles, or a new key that phase 15 releases) and which roles may approve.

### V-15-06-d  STANDS   (first agent: LOW; refuter: HIGH, see note)
- Claim: 7.2 makes `exit_decision` version 1 end at `record_decision`. That contradicts DSL.md:1052 ("`approve` per authority matrix -> `record_decision` -> `create_ops_task`"). The planning rule says a phase and a planning doc that disagree are resolved in the same change. The does-not-touch list allows DSL changes only for C.2, C.3 and C.8, and Open doubts does not list the difference.
- Evidence:
  - DSL.md:1052 "| `exit_decision` | Exit candidates | `approve` per authority matrix -> `record_decision` -> `create_ops_task` |".
  - 7.2 says the opposite for version 1: phase-15.md:2033 "publish, so it does not name `create_ops_task`. Phase 20 registers the effect `create_ops_task` and ships version 2 of".
  - A second text inside phase 15 also places the step in version 1: phase-15.md:1799 "-> `create_ops_task` (phase 20; a no-op until phase 20 is live)" in 4.2. Neither 7.2 nor 4.2 says it replaces the other, so no 7.x block says which wins.
  - The planning rule: .claude/rules/planning.md:18 "a phase file and a planning doc disagree, resolve the conflict in the same change; never let them drift silently."
  - The does-not-touch list allows no change to DSL.md:1052: phase-15.md:2254 "No DSL change beyond the wording the lead made at review (DSL.md C.2 withheld factors, C.3 PEP inputs, C.8 normalised formula)." Open doubts (2198-2236) do not list it.
  - `grep -n "create_ops_task" docs/plan/phase-15.md` finds 1799 and 2031-2034 only, which matches the first agent's search.
- Note on severity: the rubric lists "two texts contradict" as HIGH. Here three texts disagree (DSL.md:1052 and 4.2 line 1799 against 7.2 line 2033), and the phase's own 4.2 text conflicts with the publish rule of phase 04 (phase-04.md:690-691: an unregistered effect cannot be published). That is why the refuter rates it HIGH, not LOW.
- Fix: Phase 15, 4.2 (line 1799) and 7.2 (2033-2034): one sentence in 7.2 saying which text wins for version 1 and that the DSL.md:1052 difference is a recorded phasing decision (or a named DSL change).

### V-15-07-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The coverage response has no type name, no struct fields and no utoipa schema. The six counts exist in prose only (2056-2058), and the section 2 schema list (1041-1051) has no coverage view.
- Evidence:
  - Refuted part (fields): the six counts are named in prose, which the rule accepts: phase-15.md:2056-2058 "parties rated, parties with a current ownership result, results with `unknown_count > 0`, results with `loops_detected`, results with `depth_truncated`, and results whose `as_of` is older than". The route row is at phase-15.md:2052 ("| GET | `/api/v1/ownership/coverage` | `dashboard.read` |").
  - Stands part (type name and schema): the method is "`OwnershipService::coverage(ctx)`" (phase-15.md:2054) and returns no named type. `grep -n -i "coverage" docs/plan/phase-15.md` finds no DTO, and the utoipa list at phase-15.md:1041-1051 has no coverage view (grep count 0 for "coverage" in that range).
  - The counts are inferable from the prose, so this is not HIGH.
- Fix: Phase 15, 7.4 and section 2 (utoipa list, 1041-1051): one sentence naming the coverage response type and adding it to the schema list.

### V-15-08-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: No response type for the analytics (per-proposer rows, per-branch rows, overall counts, nulls before phase 18) and no utoipa schema. The query parameters `from` and `to` are not named in the route row (2065).
- Evidence:
  - Refuted part (row shapes and nulls): the three row kinds and the nulls are stated in prose: phase-15.md:2069 "1. Per `proposed_by`: total, `direction = 'down'`, `end_reason = 'expired'`", 2071 "2. Per branch: the same counts", and 2073-2075 "3. From phase 18 on: overall counts ... Before phase 18 the two fields are null."
  - Refuted part (parameters): the method names them, phase-15.md:2067 "`OverrideService::analytics(ctx, from, to)`". So the HTTP names `from` and `to` are inferable, though the route row (2065) does not list them.
  - Stands part (type and schema): no response type name appears in 7.5 (2062-2076), and `grep -n -i "analytics" docs/plan/phase-15.md` finds 2062-2067 and the header lists only. The utoipa list at phase-15.md:1041-1051 contains no analytics view (grep count 0).
- Fix: Phase 15, 7.5 and section 2 (utoipa list, 1041-1051): one sentence naming the analytics response type (its per-proposer, per-branch and overall parts) and adding it to the schema list; state the `from` and `to` query parameter names in the 7.5 route row.

### V-15-08-b  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: The window rule "at most 366 days" (2068) has no refusal: no status and no problem code for a longer window or for `from` after `to`.
- Evidence:
  - The problem-code rule of this phase: phase-15.md:1583 "### 3.10 Errors and problem codes (`KycError`, thiserror; API.md section 6 only)", and the validation row at phase-15.md:1587 "`Validation(Vec<Issue>)` | 422 `E_VALIDATION` | `errors[]` with the DSL.md s.7 Part C codes".
  - API.md section 6 gives the code for a bad window: docs/API.md:985 "| `E_VALIDATION` | 422 | Field or document validation; `errors[]` (DSL.md s.7 codes) |". A window longer than 366 days, or `from` after `to`, is a field validation, so status 422 and code `E_VALIDATION` are fixed.
  - The analytics method belongs to `OverrideService` (phase-15.md:2067 `OverrideService::analytics(ctx, from, to)`), and the 3.10 table already covers the override refusals ("a second open override", phase-15.md:1591), so the same error enum applies.
  - Precedent in phase 18 uses the same 366-day rule with no separate code: phase-18.md:615 "date range at most 366 days", which confirms the convention is field validation.

### V-15-09-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Step 5 needs "the business date of the previous mapped batch of the same source table" (2114-2116), but no read is named: no repository method or query over `admin.batches` (status `mapped`, same `source_table`, the latest `business_to`).
- Evidence:
  - The need: phase-15.md:2115 "business date of the previous mapped batch of the same source table. The first batch of a source table derives".
  - The table: phase-11.md:136 "CREATE TABLE IF NOT EXISTS admin.batches (" with `source_table` and `business_to` (phase-11.md:148). `mapped` appears only in an index predicate: phase-11.md:171 "WHERE status IN ('mapped', 'partial');".
  - The closest named read does not fit: phase-11.md:571 "/// data_as_of: the newest business date of a mapped batch per branch." is per source and branch, not per source table.
  - `BatchRepository::list` takes a `BatchFilter` (phase-11.md:568), and `BatchFilter` is defined nowhere in phase 11 (`grep -n "BatchFilter"` finds 568 only), so it cannot be known to filter by `source_table` and status.
  - No general rule covers it: `grep -n "previous batch\|latest_mapped"` finds nothing in phase 00, 02 or 03.
- Fix: Phase 15, 7.7 step 5 (line 2115), or phase 11 `BatchRepository`: one sentence naming the read that returns the latest `business_to` of mapped batches for one `source_table`.

### V-15-09-b  STANDS   (first agent: LOW; refuter: LOW)
- Claim: `due_at = BusinessClock::add(now, review_within)` (2100-2101) is called with two arguments and treated as a timestamp. Phase 09 defines `add(from, n, unit: BusinessUnit) -> Result<ClockResult, ClockError>` (phase-09.md:459), and `ClockResult` carries `fire_at`, `calendar_id` and `calendar_version`.
- Evidence:
  - The call: phase-15.md:2100-2101 "an open event review absorbs it) with `due_basis = trigger_sla` and `due_at = BusinessClock::add(now," / "review_within)` on the bank calendar."
  - The definition: phase-09.md:458-459 "pub trait BusinessClock: Send + Sync {" / "fn add(&self, from: DateTime<Utc>, n: u32, unit: BusinessUnit) -> Result<ClockResult, ClockError>;" and phase-09.md:461 "pub struct ClockResult { pub fire_at: DateTime<Utc>, pub calendar_id: Uuid, pub calendar_version: i32,".
  - The method is named and defined in its owner, so no gap of the "method not named" kind. What is missing is only the shape in the step: the unit (the DSL `review_within` is in `business_days`, DSL.md:1458) and the `fire_at` field that becomes `due_at`.
  - Step 3 gives no other clock call, and no general rule in phase 00 settles the shape.
- Fix: Phase 15, 7.7 step 3 (line 2100): one sentence naming the unit (working days, from `review_within`) and the `ClockResult` field (`fire_at`) used as `due_at`.

### V-15-10-a  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: Who may approve a downward override. The 7.9 block makes the task role CHK only (2142), but the Cedar permit admits CHK and M (1677-1679), and section 6 item 8 expects M's approval attempt to fail with `E_SELF_APPROVAL` (1983). The block does not change 3.12 or section 6.
- Evidence:
  - The block keeps M in the flow, as the escalation target: phase-15.md:2142 "`approve` role CHK, not the proposer, not the relationship owner, timeout `esc` M after 3 business days -> `record_decision`." M holds the task only after the timeout.
  - The engine applies the role of the task: phase-04.md:1074 "1. `role`: the role is among the principal's effective roles (direct or delegated, phase 01)". Before the timeout M does not hold a CHK task, so the Cedar permit at phase-15.md:1677-1679 (`@role("CHK,M")`, `override.approve_down`) is an outer gate and does not open the task early. The permit is needed for the escalated task.
  - Section 6 item 8 tests the proposer, not the role: phase-15.md:1983 "As M approve the downward change request it proposed -> 403 `E_SELF_APPROVAL`". That expectation holds under the 7.9 design and needs no change.
  - So the two texts do not give different rules for who decides: the workflow rule (CHK, then M by escalation) is the one the engine applies, and the permit is consistent with it.
- Note for the lead (not a new claim): the plan does not say in one sentence that the engine, not the Cedar permit, keeps M out until the timeout. A clarifying sentence in 7.9 would close the question, but it is not a gap.

### V-15-11-a  STANDS   (first agent: MEDIUM; refuter: HIGH, see note)
- Claim: The section 2 absorb statement updates only `trigger_code` ("DO UPDATE SET trigger_code = EXCLUDED.trigger_code", phase-15.md:1020). 7.10 needs `due_at` to move earlier and the timer to move with it (2148-2150). The block names 3.4 rule 5 (1308-1311), not the section 2 statement, so the statement as written never moves the deadline.
- Evidence:
  - The statement: phase-15.md:1020 "'awaiting_answer') DO UPDATE SET trigger_code = EXCLUDED.trigger_code RETURNING ...`". `grep -n "LEAST(\|DO UPDATE SET"` in phase-15.md finds 1020 only.
  - The rule the block adds: phase-15.md:2148 "Section 3.4 rule 5 and the trigger actions: when an open event review absorbs a new event, `due_at` becomes the earlier". 3.4 rule 5 itself (phase-15.md:1310-1311) says only "its `trigger_code` is updated; the review does not multiply".
  - No replacement wording: the 7.10 block (2147-2154) does not say that it replaces the section 2 statement. `grep -n -i "replac\|supersed\|instead"` in 2146-2152 finds nothing.
  - So two texts give two rules for the same operation (the statement changes only trigger_code; 7.10 moves due_at), and no 7.x block says which wins. The statement is the SQL a developer would copy.
- Note on severity: the first agent rated MEDIUM. The rubric lists "two texts contradict" as HIGH, and this is that case: the section 2 statement, read literally, never moves the deadline that 7.10 requires. That is why the refuter rates it HIGH.
- Fix: Phase 15, section 2 (`open_or_join`, line 1020): one sentence stating that the conflict update also sets `due_at` to the earlier value and re-arms the timer (7.10), or 7.10 naming the statement as replaced.

## Summary

- Gap IDs handled: 23 (V-15-01-a to V-15-11-a), each once.
- REFUTED: 6 (V-15-01-c, V-15-03-a, V-15-04-a, V-15-04-b, V-15-08-b, V-15-10-a)
- STANDS: 14
- PARTLY: 3
- Severity changed from the first agent: V-15-01-a HIGH to MEDIUM; V-15-06-d LOW to HIGH; V-15-11-a MEDIUM to HIGH (two texts contradict); V-15-02-d stays LOW but the lead may weigh it higher (see its note).

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-15-01-a | PARTLY | MEDIUM | No `ExitRepository` trait and no `ExitQuery` definition; `ExitService` has no method list (the service name is declared at 884) | Phase 15 section 2, after `OwnershipRepository` (line 1007) |
| V-15-01-b | STANDS | MEDIUM | `review_id` of an exit entry has no stated source | Phase 15 section 3.4 step 8 (1351-1356) or `NewExitEntry` (940-941) |
| V-15-01-d | STANDS | HIGH | `record_decision` is registered for `case` only; no effect is registered for `kyc_review` or `exit_decision` | Phase 15 section 4.2 (1794-1799), or the phase 18 registration (line 1130) |
| V-15-02-a | STANDS | HIGH | No registration-country source: no column on `canon.party`, and the phase 10 mapping the Open doubts point to does not exist | Phase 10 section 1 (`canon.party`, around line 464), or phase 15 section 3.3 row 2 (1272) |
| V-15-02-b | STANDS | MEDIUM | No repository method for the country fan-out page query | Phase 15 section 2, after `PartyFactsSource` (line 1029) |
| V-15-02-c | STANDS | MEDIUM | Source of `effective_at` is not stated; the consumed event carries no date | Phase 15 section 3.3 row 2 (line 1272) |
| V-15-02-d | STANDS | LOW | DSL event `country_table.changed` is not reconciled with `ref.country_level_changed`; TR-10 review has no stated path | Phase 15 section 3.3 row 2 and section 4.2 |
| V-15-04-c | STANDS | MEDIUM | "Display name" from `ref.list_entry_names` has no column; `ar` and `en` keys are not mapped for one-script designations | Phase 15 section 3.8 source 1 (1521-1522) |
| V-15-06-a | STANDS | HIGH | No `WorkflowSubject` for `kyc_review` or `exit_decision`; those definitions are refused at publish (`check_before_decision` has a default and is not needed) | Phase 15 section 4.2, or a new section 3 block |
| V-15-06-b | STANDS | MEDIUM | 7.2 says phase 20 ships `exit_decision` version 2; phase 20 ships no such version | Phase 15 7.2 (2033-2034), or phase 20 shipped list (line 1108) |
| V-15-06-c | STANDS | MEDIUM | The exit approval names no decision-type key and no approving roles | Phase 15 section 4.2 (line 1798) |
| V-15-06-d | STANDS | HIGH | `create_ops_task` in version 1: DSL.md:1052 and phase 15 4.2 (1799) include it, 7.2 (2033) excludes it; no block says which wins | Phase 15 section 4.2 (1799) and 7.2 (2033-2034) |
| V-15-07-a | PARTLY | MEDIUM | No response type name and no utoipa schema for the coverage view | Phase 15 7.4 and section 2 utoipa list (1041-1051) |
| V-15-08-a | PARTLY | MEDIUM | No analytics response type name and no utoipa schema; `from` and `to` not in the route row | Phase 15 7.5 and section 2 utoipa list (1041-1051) |
| V-15-09-a | STANDS | MEDIUM | No named read of the latest mapped `business_to` for one `source_table` | Phase 15 7.7 step 5 (line 2115), or phase 11 `BatchRepository` |
| V-15-09-b | STANDS | LOW | The `BusinessClock::add` call in step 3 omits the unit and the `ClockResult.fire_at` field | Phase 15 7.7 step 3 (line 2100) |
| V-15-11-a | STANDS | HIGH | The section 2 absorb statement never moves `due_at`; 7.10 requires it and names no replacement | Phase 15 section 2 `open_or_join` (line 1020) |
