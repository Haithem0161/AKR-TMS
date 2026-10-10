# Refuter, phase 29 (Haiku draft for lead review)

### V-29-01-a  PARTLY   (first agent: MEDIUM; refuter: LOW for the part that stands)
- Claim: six types used in signatures (`GatewayIdentity`, `StoredResponse`, `LedgerBegin`, `SignError`, `FailPolicyHit`, `PolicySnapshot`) have no fields or variants in the text.
- Evidence:
  - REFUTED, `LedgerBegin`: docs/plan/phase-29.md:919 "The ledger answers `Created`, `Replay`, `InFlight` or `Reused`, and only `Created` continues." The rule of each answer is at :963-967.
  - REFUTED, `PolicySnapshot`: docs/plan/phase-29.md:1029 "an `ArcSwap<PolicySnapshot>` with the matrix, bands, modes, rules and `set_version`".
  - REFUTED, `StoredResponse`: docs/plan/phase-29.md:965 "its `response_bytes` are returned as they are" (the stored content is named).
  - REFUTED, `GatewayIdentity`: docs/plan/phase-29.md:1072 "the `gateway_key`, the service principal `user_id`", :1074 "`GatewayCache`, which holds the rows", :1075 the Cedar principal built from the same `admin.users` row.
  - REFUTED, `FailPolicyHit`: docs/plan/phase-29.md:1028 "Its `on_failure` is stored on the decision as `fail_policy`" (the `OnFailure` enum is at :824).
  - STANDS, `SignError`: `grep -n SignError docs/plan/phase-29.md` gives :902 only. No variant is named, and no step says what a failed signature answers. The generic row at docs/plan/phase-29.md:1175 "`Internal` | `E_INTERNAL` | 500 | correlation id only" is the only fit.
- Fix (`SignError` only): section 2 "Enums and closed sets" (:824): name the `SignError` variants, or say in section 3.16 that a failed signature answers `Internal`.

### V-29-01-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the hold and hold-candidate inserts of step 16, and the hold creation rule, name no audit event.
- Evidence: `grep -n "hold_created\|hold_opened\|hold_inserted"` finds nothing. The kinds at docs/plan/phase-29.md:1190-1193 are `assess.hold_proposed`, `assess.hold_decided`, `assess.hold_expired` and `assess.pass_issued_on_release`. "**Creation** (section 3.1 step 16, same transaction as the decision):" (:986) names no event. Section 3.17 says "one per mutation" (:1177). Its only exemption is "Per-payment decisions are **not** audited ... the decision row is the record (DR-296)" (:1179). DR-296 (docs/plan/research.md:307) says "the audit stream holds the mutations only", so a hold insert is a mutation with no named event.
- Fix: section 3.17 (:1179-1200): add a kind for hold creation (resource `assess_hold`), or state that the step 16 hold rows are covered by the decision row.

### V-29-01-c  REFUTED   (first agent: LOW; refuter: none)
- Claim: "phase 14 section 3.5 item 9" should be section 3.4 item 9.
- Evidence: docs/plan/phase-14.md:981 "9. **Outcome**: `Hit` when score" sits in section 3.4 "Scorer" (phase-14.md:970). The candidate outcome list exists. The cross-reference is off by one section, which the rules do not count as a gap.

### V-29-02-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: no step creates a rail or its first shadow row. `akr-ctl assess rail add` is named only in the header and in 4.3, and the first `rail_modes` row needs a change request that only a route on an existing rail can open.
- Evidence: `grep -n "rail add"` finds docs/plan/phase-29.md:9 and :1333 only. The rail row is required by the FK at :242 ("REFERENCES assess.rails(code) ON DELETE RESTRICT"). The guard at :520-524 refuses an INSERT without an approved `assess_mode` change request. The only route that opens one, `PUT /modes/{rail}`, needs the rail row. Section 3.7 says "The first mode of a rail is `shadow`" (:1035) with no step that writes it, so the text gives two rules for the same first row (no change request, and a change request).
- Fix: section 3.7 "Stages" (:1035): add the steps of `akr-ctl assess rail add` (rail row and first shadow change request) and say which of the two rules holds.

### V-29-02-b  PARTLY   (first agent: MEDIUM; refuter: HIGH for the part that stands)
- Claim: the refusal point of the entry criteria is not fixed. Section 3.7 and 3.11 put the check at `check_before_decision`, while item 29 and API.md:800 expect the PUT to refuse.
- Evidence:
  - REFUTED, item 29: docs/plan/phase-29.md:1429 "is refused at `check_before_decision` with 422 `E_VALIDATION` and `criterion_not_met`". Section 3.7 (:1050) says the same. Both phase 29 statements agree.
  - STANDS, API line: docs/API.md:800 "PUT | /assess/modes/{rail} ... refused when the entry criteria fail". Phase 29 `prepare` (docs/plan/phase-29.md:1090) does not check the criteria, and the criteria are checked only at approval (:1091). Phase 29 would create a change request for a move that API.md says is refused at the PUT. The two texts give different refusal points and no 7.x block says which wins. Severity is HIGH by the scale (two texts contradict).
- Fix: section 3.11 `prepare` (:1090) or section 3.7 "Change" (:1050): say whether the criteria are checked at the PUT, at approval, or both, and align docs/API.md:800.

### V-29-02-c  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: the writes of `activate` (four policy tables) have no repository method, since `PolicyRepository` names only reads and `bump_set_version`.
- Evidence: docs/plan/phase-29.md:851 "pub trait PolicyRepository: Send + Sync {   // akr-assess reads; akr-api writes through the change target". The design puts the writes in the change target's `activate`, which section 3.11 gives as one transaction (:1092). The same split covers V-29-03 and V-29-06.

### V-29-03-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the band history is contradictory. The guard message (:564) and the retention row (:802) assume that each band set is kept, but the primary key and the in-place activation keep one set.
- Evidence: docs/plan/phase-29.md:217 "PRIMARY KEY (currency, band_code)" has no set version. :1092 "bands: update or insert by key with the same set version". Against this, :564 "assess.amount_bands rows are replaced by a new set version, never deleted" and :802 "('assess', 'amount_bands', 'R9', 'retirement', NULL, 'Band sets of the fail policy')". The decision entity (docs/plan/phase-29.md:823) carries `amount_band` and `policy_version`, so after an activation a decision cannot be explained against the bounds it used.
- Fix: section 1.2 (:210-218) and section 3.11 "activate" (:1092): either add `set_version` to the key and keep superseded rows, or say that only the current bands are kept and change the retention row at :802.

### V-29-03-b  STANDS   (first agent: LOW; refuter: LOW)
- Claim: `FailPolicyRow` is named in `PolicySetProposal` but defined nowhere; the entity is `FailPolicy`.
- Evidence: docs/plan/phase-29.md:825 "`PolicySetProposal { matrix: Vec<FailPolicyRow>, bands: Vec<AmountBand>, basis: String }`". The entity is at :823 "`FailPolicy { rail, corridor, currency, a`". `grep -n FailPolicyRow docs/plan/phase-29.md` finds :825 only.
- Fix: section 2 "Create and update DTOs" (:825): use the entity `FailPolicy`, or define `FailPolicyRow` as the table row.

### V-29-03-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the `prepare` refusal "each rail has a row for `*`, `*`, `*`" names no error code, and no code in the 3.16 list fits.
- Evidence: docs/plan/phase-29.md:1027 "each rail has a row for `*`, `*`, `*`" names no code. The `Validation` row at :1171 has no catch-all code. `rail_not_configured` (:918, :1171) fits "every rail exists", not a missing `*` row. The convention in docs/API.md:985 takes `errors[]` codes from docs/DSL.md section 7 (:1070-1190), and no code there fits either.
- Fix: section 3.6 "Validation at proposal" (:1027) and the code list at :1171: name the `errors[]` code for a missing catch-all row.

### V-29-04-a  STANDS   (first agent: MEDIUM; refuter: HIGH: the closure cannot be built as written)
- Claim: the closure of step 6 cannot run. `closed_by` must name a user whenever `closed_at` is set, the job runs as the system, and step 5 never sets the `register_id` that links an exception to a finding.
- Evidence: docs/plan/phase-29.md:360 "CHECK ((closed_at IS NULL) = (closed_by IS NULL))" and :354 "UUID REFERENCES admin.registers(id) ON DELETE RESTRICT". Step 6 (:1138) says "The job performs the closure", and names no user for `closed_by`. Step 5 (:1137) inserts exceptions and opens "One register warning per run and rail", but sets no `register_id`. So no step gives an exception the finding that step 6 closes, and DR-307 (:1522) says exceptions "close when their finding is closed".
- Fix: section 3.14 step 5 (:1137): say which finding each exception opens and sets `register_id` to it; section 3.14 step 6 (:1138): say whose user id `closed_by` takes.

### V-29-04-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: who performs the closure is not clear. Step 6 says the job performs it, 4.5 item 7 says the register route closes the exceptions, and Open doubts 17 asks the phase 24 finding route to trigger it.
- Evidence: docs/plan/phase-29.md:1138 "The job performs the closure; there is no route for it." :1354 "The register route of finding closure closes release exceptions (section 3.14 step 6)." :1553 "the finding route of phase 24 must trigger the closure of release exceptions". The three can be read as one flow (the route enqueues the job), but no text says so, and the trigger is not named.
- Fix: section 3.14 step 6 (:1138): say how the closure starts (the phase 24 route writes it, or enqueues the job and how the job learns of it), and align 4.5 item 7 (:1354).

### V-29-04-c  PARTLY   (first agent: MEDIUM; refuter: LOW for the part that stands)
- Claim: `ReleaseRepository` has no method that closes exceptions, and `CanonRelease`, `DecisionKeyRow` and `NewReleaseException` have no fields in the text.
- Evidence:
  - REFUTED, `NewReleaseException`: its fields are the columns of `assess.release_exceptions` (docs/plan/phase-29.md:345-362: `payment_message_id`, `payment_received_at`, `match_key`, `rail`, `reason_code`, `decision_id`, `register_id`, `closed_at`, `closed_by`, `close_reason`).
  - REFUTED, `CanonRelease` and `DecisionKeyRow`: their fields are named in section 3.14 steps 1 to 4 (phase-29.md:1129 "`direction = 'out'` and a `status` other than", :1131 "`canon.payment_message.uetr`", :1132 "the decisions with the same match key and `received_at`").
  - STANDS, the closure method: step 6 (:1138) gives the closure's fields but names no method and no owner of the write. The owner question is V-29-04-b.
- Fix (method part only): section 2 "Repository ports" (:859-863): name the method that closes an exception, or name the job step that writes it.

### V-29-04-d  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the matcher reads `message_refs->>'internal_ref'`, a key that no plan file defines. Phase 10 types `message_refs` only as an open JSONB object.
- Evidence: docs/plan/phase-29.md:1131 "`ref:<SHA-256 of message_refs->>'internal_ref'>`". `grep -rn internal_ref docs/` finds phase-29.md:954, :956, :958 and :1131 only (the first three are the parser's canonical input, not the stored column). docs/plan/phase-10.md:1033 "message_refs JSONB NOT NULL DEFAULT '{}'::jsonb" has no key list.
- Fix: section 3.14 step 3 (:1131), or phase 10 section 7+: name the key and the producer that writes it into `canon.payment_message.message_refs`.

### V-29-04-e  STANDS   (first agent: MEDIUM; refuter: LOW, the point is unclear)
- Claim: the checkpoint update of step 7 (`assess.control_state.release_checkpoint_at`) is a mutation with no audit event, and no exemption for operational writes is stated.
- Evidence: docs/plan/phase-29.md:1139 "**Checkpoint**: updated last, in the same transaction as the exception rows of its chunk." No kind for `control_state` at :1181-1200. The node heartbeat at :1115 is also unaudited, and no text exempts either write. The exemption for housekeeping in the rules applies only when the item's text says so. It is unclear whether a control cursor counts as business data.
- Fix: section 3.17 (:1179-1200): one sentence saying that the control checkpoint and the node heartbeat are operational writes that emit no audit event.

### V-29-04-f  STANDS   (first agent: MEDIUM; refuter: HIGH: step 4 gives two answers for an advisory reject)
- Claim: step 4 matches "a decision in `advisory` mode (advisory releases are expected)" whatever its decision, so a release after an advisory reject raises no exception and no `decision_not_pass`.
- Evidence: docs/plan/phase-29.md:1134 "a `pass` decision (any mode), or a decision in `advisory` mode (advisory releases are expected)" matches the release. The next bullet, :1136, says "only a `reject` ... exception `decision_not_pass`". For an advisory reject both bullets apply and they answer differently. DR-307 (:1522) says exceptions are written for "a decision that was not a pass". Step 14 (:928) answers an advisory reject as a reject.
- Fix: section 3.14 step 4 (:1134): limit the advisory match to `hold` and `pass`, so that an advisory reject raises `decision_not_pass`.

### V-29-05-a  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: the effect of a list alert on an advisory list hold is not stated, and it is unclear whether the alert may stay open after the hold closes.
- Evidence: docs/plan/phase-29.md:1007 "`advisory_open` gives `advisory_closed`; `shadow_review` gives `shadow_closed`" followed by "For a list_match hold the phase 14 alert is decided by its own rules (step 6)". The sync of step 7 (:1011) applies only "when the alert of a `held` list_match hold becomes `incorrect`" and "when it becomes `true`". So an alert outcome moves only a held hold, and an advisory alert is decided under phase 14's own rules. The pointer "(step 6)" is wrong, which is a LOW matter and not a gap.

### V-29-05-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `HoldRepository::apply_outcome` requires `by: Uuid`, but a sync is system work with no user, and no text says what `decided_by` holds after a sync.
- Evidence: docs/plan/phase-29.md:846 "`apply_outcome(&self, tx: &mut Tx, id: Uuid, expected: i32, to: HoldState, by: Uuid, at: DateTime<Utc>)`". The sync of step 7 (:1011) names no decider. The only inference is the held list-hold bullet at :1009, "decided by phase 14 `AlertService::decide` with the approver as decider", which the text does not extend to a sync. The other half of the claim does not stand: the type `Proposal` (:845) has its fields in the `Hold` entity (:823) and in the proposal step at :1005 (section 3.5 step 4).
- Fix: section 3.5 step 7 (:1011): say what `decided_by` holds after a sync (the alert's decider, or NULL with a changed signature).

### V-29-05-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the sync transitions to `released` and `rejected` (:1011) and the `set_alert` write (:1016) name no audit event.
- Evidence: docs/plan/phase-29.md:1011 "when the alert of a `held` list_match hold becomes `incorrect`, the hold becomes `released`" is a state change of business data. :1016 "Then `HoldRepository::set_alert`" links the alert. The kinds at :1190-1193 cover a person's decision (`assess.hold_decided`), expiry and the release pass. None covers a sync transition or the alert link. The sync is not a person's decision, so step 6 does not reach it.
- Fix: section 3.17 (:1179-1200): add a kind for the sync transition and the alert link, or say that `assess.hold_decided` covers them.

### V-29-06-a  PARTLY   (first agent: MEDIUM; refuter: LOW for the part that stands)
- Claim: the retirement of a rule is priced (`increase`) but has no defined request or activation.
- Evidence:
  - REFUTED, request and activation: docs/plan/phase-29.md:1063 "Proposal: through phase 04 `POST /change-requests` with object kind `assess_rule`" and "`increase` for a retirement"; the `RuleProposal` entity carries `status` (:825); the activation at :1092 says "rule: insert the new version and retire the previous active version (`status`, `retired_at`)".
  - STANDS, the row: no text says that a retirement request sets `status` to a retired value, or that the inserted version carries it.
- Fix: section 3.9 "Proposal" (:1063): one sentence saying that a retirement is a rule proposal whose `status` is the retired value.

### V-29-06-b  PARTLY   (first agent: MEDIUM; refuter: LOW for the part that stands)
- Claim: the refusal of an invalid `basis` (empty, or over 500 characters: DB CHECK at :261) names no error code.
- Evidence:
  - REFUTED as to the code family: docs/API.md:985 "`E_VALIDATION` | 422 | Field or document validation; `errors[]` (DSL.md s.7 codes)". docs/DSL.md:1096 "`E_SCHEMA` | error | JSON Schema violation (path points at the offending key)". A length breach of the CHECK at docs/plan/phase-29.md:261 is a schema violation.
  - STANDS, the exact code: :1063 and the code list at :1171 name no code for the `basis` refusal, and phase 29 uses its own `errors[]` list at :1171 that has no `basis` code.
- Fix: section 3.9 "Proposal" (:1063): name the `errors[]` code and JSON-pointer path for an empty or over-long `basis`.

### V-29-06-c  PARTLY   (first agent: MEDIUM; refuter: LOW for the part that stands)
- Claim: `RuleCondition` (:823) is named but defined nowhere, and the pure function `domain/assess::rules` (:1062) has no signature.
- Evidence:
  - REFUTED, `RuleCondition`: docs/plan/phase-29.md:1062 "keys `corridor_in` (non-empty list of ISO 3166-1 alpha-2 codes), `currency_in` (ISO 4217 codes), `rail_in` (rail codes) and `amount_min` (decimal string in the instructed currency), combined with AND". The four keys and their types are stated.
  - STANDS (LOW), the function: `domain/assess::rules` (:1062) has no Rust signature. Its input (the payment's corridor, currency, rail and amount) and its output (the first matching `rule_id`, :1064) are stated in prose.
- Fix: section 3.9 (:1062): one line giving the signature of `domain/assess::rules`.

### V-29-06-d  STANDS   (first agent: LOW; refuter: LOW)
- Claim: phase 16 is named as the owner of the scenario flag `phase2_candidate`, but the flag is defined only in phase 06, and the cited line 84 is wrong.
- Evidence: docs/plan/phase-29.md:5 "Phase 16 (scenario register `phase2_candidate`, read only as described in section 3.9)". docs/plan/phase-06.md:88 "phase2_candidate BOOLEAN NOT NULL DEFAULT FALSE". `grep -c phase2_candidate docs/plan/phase-16.md` gives 0. docs/plan/phase-16.md:2170 "S-26 (phase 2 flag, phase 29)" points to phase 29, a third owner.
- Fix: header (line 5) and section 3.9 (:1066): name phase 06 as the owner of the column (phase-06.md:88), and align phase 16 line 2170 with it.

### V-29-07-a  REFUTED   (first agent: LOW; refuter: none, cosmetic only)
- Claim: phase 14 gives the candidate query only as prose, with no method name, so phase 29 step 9 cannot name the call it makes.
- Evidence: docs/plan/phase-14.md:964 "`ScreeningIndexRegistry` holds `ArcSwap<HashMap<Uuid, Arc<ScreeningIndex>>>`" and :965 "**Candidates** (N-29, N-30): query tokens are sorted by posting size; the two rarest intersect". The candidate query is designed in prose. Only the Rust method name is absent, and a missing name is not a gap under the rules.

### V-29-08-a  PARTLY   (first agent: MEDIUM; refuter: LOW for the part that stands)
- Claim: no step creates the two inventory items (`rule` and `parameter_set`), sets their owner and validator, or writes the seven tier answers that section 3.22 says are stored.
- Evidence:
  - REFUTED, creation: docs/API.md:661 "POST | /inventory | model.risk | Register an item before use". docs/plan/phase-04.md:1339 "**Create**: `kind`, `object_id`, `name`, `tier`, `tier_basis?`, `owner_id`, `developer`, `validator_id?`".
  - REFUTED, owner, validator and the tier values: docs/plan/phase-29.md:1305 "The bank names both (Open doubts 15)"; :1551 "The bank confirms Tier 1 for the rule set and the parameter sets, names the owner and the validator of each".
  - STANDS (LOW): :1301 says the seven answers "are stored in `inventory_items.tier_basis`", but phase 04 makes `tier_basis` optional for these kinds (:1339), and phase 29 does not say the create call must carry it.
- Fix: section 3.22 (:1301): say that the create call of each item carries the seven answers in `tier_basis`.

### V-29-08-b  STANDS   (first agent: LOW; refuter: LOW)
- Claim: section 3.22 says the rule set is "tested with the golden fixtures of section 6", but section 6 names no golden fixtures.
- Evidence: docs/plan/phase-29.md:1304 "the rule set is tested with the golden fixtures of section 6". `grep -n golden` finds that line only. Section 6 item 4 (:1404) has "parser fixtures" and "fixed vectors", and its rule test at (c) covers the grammar only. The production rule set is validated under phase 24 (Open doubts 27, :1563).
- Fix: section 3.22 (:1304): name the rule-set cases in section 6 item 4, or point to the phase 24 validation pack.

### V-29-09-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the switch to the fallback has no trigger. "while Valkey does not answer" gives no timeout and no rule for a slow call, and phase 30 says "used only when Valkey fails".
- Evidence: docs/plan/phase-29.md:1472 "`LimiterFallback` that answers the rate limit of section 3.1 step 3 while Valkey does not answer". docs/plan/phase-30.md:203 "used only when Valkey fails". Step 3 (:917) has "Budget 1 ms" and no timeout. `grep -n -i timeout docs/plan/phase-00.md` gives only database and job timeouts, so no general dependency-timeout rule covers Valkey.
- Fix: section 3.1 step 3 (:917): state the Valkey call timeout, and say whether a timeout counts as "does not answer".

### V-29-09-b  STANDS   (first agent: MEDIUM; refuter: HIGH: two texts contradict)
- Claim: section 3.10 says the limit is "enforced per node" and "twice the per-node figure" with two nodes, but the key `assess:rl:{gateway_id}` is shared by every node, and phase 30 divides the gateway setting by the replica count.
- Evidence: docs/plan/phase-29.md:1076 "keyed `assess:rl:{gateway_id}` with refill `assess.rate_per_second` (50) and capacity `assess.rate_burst` (500), enforced per node". One key for all nodes means the Valkey path is one bucket, so "per node" is wrong for it. docs/plan/phase-30.md:203 "an in-process token bucket per node whose capacity and refill are the gateway's setting divided by the `akr-assess` replica count". Section 7.1 (docs/plan/phase-29.md:1472-1473) says "No other rule of step 3 changes", so the two paths give different totals. The bank's pending decision (docs/plan/phase-29.md:1546, Open doubts 10) covers the balancing, not the key scope.
- Fix: section 3.10 (:1076): state one scope for the bucket (one shared key, or one bucket per node), and align phase 30 section 3.6 (:203).

### V-29-09-c  REFUTED   (first agent: LOW; refuter: none)
- Claim: `LimiterFallback` has no method (take, retry-after) in either phase.
- Evidence: docs/plan/phase-29.md:1472 "`LimiterFallback` that answers the rate limit of section 3.1 step 3". Step 3 (:917) gives the method's behaviour: "an empty bucket is 429 `E_RATE_LIMITED` with `Retry-After`". The behaviour of take and retry-after is stated in prose. `GatewayRateLimiter` has no Rust signature either (:1076), so the omission is consistent and not a gap under the rules.

### V-29-10-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the `source_key` of `AssessKeyAge`, `AssessCertificateExpiry`, `AssessFallbackStale`, `AssessControlTestStale` and `AssessRepeatAttempts` is not given. "dedupes each condition per day" names no format.
- Evidence: docs/plan/phase-29.md:1102 "`source_key` dedupes each condition per day, so a repeated run adds nothing" gives no format. Section 7.3 (:1479-1486) names the titles but no key. The neighbouring warnings give a pattern: section 3.13 step 1 uses `'assess:blind:' || started_at`, and section 3.14 step 5 (:1137) uses `'assess:release:' || rail || ':' || run_date`. A developer can infer "per day", but not the format.
- Fix: section 3.12 step 3 (:1102) or section 7.3: give one format for the six titles that includes the day.

### V-29-11-a  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: the skip of `rejected` and `cancelled` rows is missing from the repository spec of `canon_releases` (:866), so the rule has no owner in the code.
- Evidence: docs/plan/phase-29.md:1129 "**Scope**: canonical payments with `direction = 'out'` and a `status` other than `rejected` or `cancelled`" names the owner: step 1 of the matcher job. Section 7.5 (:1495-1496) says "Section 3.14 step 1 skips rows whose status is `rejected` or `cancelled` when they are read". The repository returns the rows and the service applies the scope, which is a designed split.

### V-29-12-a  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: no disposal handler is registered for the 19 tables, which phase 23 section 7.2 requires, so `assess.node_status` is never purged.
- Evidence: docs/plan/phase-23.md:1010 says each later phase adds "one handler per table ... chosen by class and clock". Section 1.4 gives a class and a clock for each of the 19 tables (docs/plan/phase-29.md:795-813, 19 `admin.retention_assignment` rows). Section 7.6 (:1502-1503) states the dispositions: "the R7 tables are `report_only`", "R9 and R10 tables are kept", and the `node_status` row is deleted by the engine's `record_purge`. The handler follows from class and clock, so no table lacks one by design.

## Summary

- REFUTED: 7
- STANDS: 17
- PARTLY: 7

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-29-01-a | PARTLY | LOW | `SignError` has no variants, and no step says what a failed signature answers | section 2 "Enums and closed sets" (:824), or section 3.16 |
| V-29-01-b | STANDS | MEDIUM | the step 16 hold and hold-candidate inserts have no audit kind | section 3.17 (:1179-1200) |
| V-29-02-a | STANDS | HIGH | no step creates a rail or its first shadow row, and the text gives two rules for that first row | section 3.7 "Stages" (:1035) |
| V-29-02-b | PARTLY | HIGH | docs/API.md:800 says the PUT is refused when the criteria fail; phase 29 checks them only at approval (:1050, :1091) | section 3.11 `prepare` (:1090) or section 3.7 (:1050); docs/API.md:800 |
| V-29-03-a | STANDS | HIGH | `amount_bands` keeps one band set (:217, :1092), but the guard (:564) and the retention row (:802) assume kept sets | section 1.2 (:210-218) and section 3.11 "activate" (:1092) |
| V-29-03-b | STANDS | LOW | `FailPolicyRow` (:825) is undefined; the entity is `FailPolicy` (:823) | section 2 "Create and update DTOs" (:825) |
| V-29-03-c | STANDS | MEDIUM | a refused missing catch-all row has no code in the list at :1171 | section 3.6 "Validation at proposal" (:1027) |
| V-29-04-a | STANDS | HIGH | closure sets `closed_by` (NOT NULL pair, :360) but the job runs as the system, and no step sets `register_id` (:354) | section 3.14 steps 5 and 6 (:1137-1138) |
| V-29-04-b | STANDS | MEDIUM | the closure trigger is unstated: "the job performs" (:1138) against the phase 24 route (:1354, :1553) | section 3.14 step 6 (:1138) and section 4.5 item 7 (:1354) |
| V-29-04-c | PARTLY | LOW | no repository method closes an exception, and step 6 names no owner of the write | section 2 "Repository ports" (:859-863) |
| V-29-04-d | STANDS | MEDIUM | the key `internal_ref` in `message_refs` is read (:1131), but no text defines it or its producer | section 3.14 step 3 (:1131), or phase 10 section 7+ |
| V-29-04-e | STANDS | LOW | the control checkpoint (:1139) and the node heartbeat (:1115) are unaudited, and no text exempts operational writes | section 3.17 (:1179-1200) |
| V-29-04-f | STANDS | HIGH | step 4 (:1134) matches an advisory reject, while the next bullet (:1136) and DR-307 (:1522) give an exception | section 3.14 step 4 (:1134) |
| V-29-05-b | STANDS | MEDIUM | `apply_outcome` requires `by: Uuid` (:846), and no text says what `decided_by` holds after a sync | section 3.5 step 7 (:1011) |
| V-29-05-c | STANDS | MEDIUM | the sync transitions (:1011) and `set_alert` (:1016) have no audit kind | section 3.17 (:1179-1200) |
| V-29-06-a | PARTLY | LOW | no text says a retirement proposal sets `status` to the retired value | section 3.9 "Proposal" (:1063) |
| V-29-06-b | PARTLY | LOW | no error code is named for an empty or over-long `basis` (:1063, :1171) | section 3.9 "Proposal" (:1063) |
| V-29-06-c | PARTLY | LOW | no Rust signature for `domain/assess::rules` (:1062) | section 3.9 (:1062) |
| V-29-06-d | STANDS | LOW | `phase2_candidate` is attributed to phase 16 (header line 5), but the column is defined in phase 06 (phase-06.md:88) | header line 5 and section 3.9 (:1066) |
| V-29-08-a | PARTLY | LOW | `tier_basis` is not required on the create call for kinds `rule` and `parameter_set` (phase-04.md:1339) | section 3.22 (:1301) |
| V-29-08-b | STANDS | LOW | "golden fixtures of section 6" (:1304): section 6 names none | section 3.22 (:1304) or section 6 item 4 (:1404) |
| V-29-09-a | STANDS | MEDIUM | "while Valkey does not answer" (:1472) has no timeout or error rule | section 3.1 step 3 (:917) |
| V-29-09-b | STANDS | HIGH | "enforced per node" (:1076) conflicts with the shared key, and phase 30 divides the rate (phase-30.md:203) | section 3.10 (:1076) and phase 30 section 3.6 (:203) |
| V-29-10-a | STANDS | MEDIUM | the `source_key` format of the six roll-up warning titles is not given | section 3.12 step 3 (:1102) or section 7.3 (:1479-1486) |
