# Refuter, phase 26 (Haiku draft for lead review)

### V-26-01-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the family value (or code-map kind) that makes a product "Islamic" is not named, so the refusal in 3.1 step 2 and test 3(b) cannot be coded.
- Evidence: docs/plan/phase-26.md:309 "whose family is Islamic is refused for" (3.1 step 2). docs/plan/phase-26.md:559-560 "The Islamic product is a `canon.product` row of the same family with a type" (3.10 step 3), so family does not mark an Islamic product. docs/plan/phase-10.md:399 "family TEXT NOT NULL" (no value list). docs/DATA-MODEL.md:387 lists the column only. Greps with no hit: "family" in research/*.md and docs/DSL.md for a product family rule; "family" in phase-26.md other than 309, 455 (the ISL family) and 559-560.
- Fix: phase 26, section 3.1 step 2: name the value or code-map kind that marks an Islamic product, and make 3.10 step 3 agree with it.

### V-26-01-b  STANDS   (first agent: LOW; refuter: MEDIUM)
- Claim: test 3(d) says a code mapped to two types is refused by uq_code_map_entries_effective, but that UNIQUE is (code_map_id, source_list, source_code, valid_from), so only an equal valid_from is refused.
- Evidence: docs/plan/phase-10.md:162 "CONSTRAINT uq_code_map_entries_effective UNIQUE (code_map_id, source_list, source_code, valid_from)". docs/plan/phase-26.md:314-315 "(d) a code mapped to two types is refused by `uq_code_map_entries_effective` (phase 10)". Greps with no hit: "overlap" in phase-10.md is only a source column (58) and the re-read window (281, 288); "two types" and "already mapped" in phase-10.md: none. phase-10.md:1893 "`minor` when every operation maps a previously unmapped code" sets no refusal for a second Map of a mapped code.
- Fix: phase 26, section 3.1 step 3(d): restrict (d) to an equal valid_from, or name the refusal that a second Map of a mapped code gets (phase 10 has none).

### V-26-02-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM for the stands part)
- Claim: the list route `GET /islamic/deals?customer_id=` is one prose sentence: no numbered steps, no service method name, no error for a missing or malformed customer_id or cursor, and "each candidate customer" is unclear for one customer_id.
- Evidence (refuted part): docs/plan/phase-00.md:499 "`Validation(Vec<ValidationError>)` | 422 | `E_VALIDATION` (`errors[]` with JSON pointers)" covers a missing or malformed customer_id or cursor with the general 422. The repository method is named: docs/plan/phase-26.md:265 "`list_for_customer(customer_id, after, limit) -> Vec<IslamicDeal>`".
- Evidence (stands part): docs/plan/phase-26.md:380 "the same steps 1 to 6 for each candidate customer, then the rows whose customer is readable". Step 2 reads one deal by id (docs/plan/phase-26.md:361 "Read `canon.islamic_deals` where `id = $1 AND recorded_to IS NULL`"), so the steps cannot be applied to a list as written. No service method is named for the list (grep "list_for_customer" hits only line 265).
- Fix: phase 26, section 3.4 list paragraph: numbered steps for the list, the service method name, and the readable-row filter, replacing "the same steps 1 to 6".

### V-26-02-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: step 6 passes the bare class record_open, but phase 01 defines consume with a StepUpAction and says each phase declares its actions as constants; phase 26 declares none.
- Evidence: docs/plan/phase-26.md:367 "`StepUpService::consume(tx, ctx, header, record_open, "islamic_deal:<id>")`". docs/plan/phase-01.md:731 "`consume(tx, ctx, header, action: StepUpAction, resource_ref: Option<&str>)`". docs/plan/phase-01.md:711 "phase declares its actions as `StepUpAction { name, class }` constants". Grep "StepUpAction" in docs/plan/phase-26.md: none. The decision consume in 3.5 verify step 2 has the same gap.
- Fix: phase 26, section 3 (before 3.4): declare the StepUpAction constants the phase uses (a record-open action and a decision action, each with name and class), and pass them in 3.4 step 6 and 3.5 verify step 2.

### V-26-02-c  STANDS   (first agent: LOW; refuter: MEDIUM)
- Claim: the display name is read from canon.party_name with no current-row filter, so a superseded name could be returned; the table carries superseded_at.
- Evidence: docs/plan/phase-26.md:371 "A readable party shows its role, its id and its display name from `canon.party_name`." Grep "superseded_at IS NULL" in phase-26.md hits only the deal_parties lines (107, 108, 110, 266, 369). docs/plan/phase-10.md:777-778 "uq_party_name_current ... WHERE superseded_at IS NULL" defines the current name. docs/plan/phase-18.md:584 applies that filter to phase 18's own party reader, but phase 26 does not say its read is that reader. Severity is MEDIUM because the filter is inferable from the table's index, but it is not written.
- Fix: phase 26, section 3.4 step 7: add "where `superseded_at IS NULL`" to the display-name read.

### V-26-02-d  REFUTED   (first agent: LOW; refuter: none)
- Claim: the deal load (3.2 step 3) writes canon.islamic_deals and canon.deal_parties, and its audit event (phase 11 batch.recorded) is named in neither 3.2 nor 3.12.
- Evidence: docs/plan/phase-11.md:1191 "`batch.recorded` | every receipt (status, counts, watermark move; counts only) | batch". The deal load runs through the CanonicalWriter as a batch (docs/plan/phase-26.md:319 "### 3.2 Deal load (`CanonicalWriter`, phase 10 and phase 11)"), so each of its receipts emits batch.recorded under phase 11's rule.

### V-26-03-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the SCN-ISL-01 match is not written: which deal_party roles count as seller or supplier, how "his agent" and "a related party" are matched, and what party_contact.value_hk and account_party_role are compared with.
- Evidence: docs/plan/phase-26.md:463 "Seller or supplier is the customer, his agent or a related party (hard rule)" (3.6 row, with its field list only). research/24-islamic-banking-aml.md:633 gives the requirement, not the conditions: "detect shared owner, address, phone, email, signatory, account or device". Greps with no hit: "ISL-" and "seller" in docs/DSL.md. Grep "his agent", "related part" and "relationship.*agent" in docs/plan/*.md and research/*.md: the only SCN-ISL-01 hit is phase-26.md:463; phase-17.md:1954 and phase-18.md:1522 use "related parties" for other rules (linked-party rows, outcome tiers). Pointer only, not a refutation: research/01-iraq-regulatory-framework.md:1043 defines related parties by kinship degree in a digital-bank licensing context, which the phase does not cite.
- Fix: phase 26, section 3.6 row SCN-ISL-01: number the conditions (the seller and supplier roles, the customer match, the agent match, the related-party match, the contact and account comparisons), or cite a DSL scenario that states them.

### V-26-03-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the rule needs an agent relation and a related-party relation, but the party_relationship type CHECK has neither value, and the phase does not map them to those values or to deal_party.role = agent.
- Evidence: docs/plan/phase-10.md:574-575 "type TEXT NOT NULL CHECK (type IN ('owner', 'controller', 'director', 'signatory', 'attorney', 'guardian', 'family', 'employee_of', 'associate', 'unknown'))". docs/DATA-MODEL.md:378 lists the same ten values. "agent" in a CHECK appears only at docs/plan/phase-10.md:1061 (canon.payment_message_party, another table). docs/plan/phase-26.md:84 "('customer', 'seller', 'buyer', 'agent', 'supplier', 'developer'," is a deal role, not a relation between two parties.
- Fix: phase 26, section 3.6 row SCN-ISL-01: say which party_relationship type, or which deal_party role, stands for "his agent" and for "a related party".

### V-26-04-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: no repository or service method computes the two G1 tile counts (awaiting possession evidence; duplicate document references), and duplicate_exists returns only a bool.
- Evidence: docs/plan/phase-26.md:269 "`duplicate_exists(document_ref_hk, except_deal_id) -> bool` (used only to write the register warning, never returned to the caller)". The tiles are defined only in prose: docs/plan/phase-26.md:794 "Islamic deals awaiting possession evidence" and 797 "count of duplicate document references". docs/plan/phase-24.md:540 "one method per tile and per clock source of section 3.7 and 3.8", but the phase 26 tiles are not in the 3.7 table, and docs/plan/phase-24.md:1280-1281 only says "the two tiles of phase 26". Greps with no hit: "dup_count" and "duplicate count" as a method in docs/plan/phase-26.md (the only "duplicate count" hit is Open doubts 11, docs/plan/phase-26.md:931, which says the count "reaches M only and has no list route"); "Islamic" anywhere in docs/plan/phase-24.md. The duplicate count has no method and no defined aggregate, which makes it HIGH; the awaiting count has its predicate in 5.3 but still no method.
- Fix: phase 26, section 2 repository traits: add the two G1 count methods with their SQL predicates and the caller visibility parameter, as methods of phase 24's CommandReadRepository (named in section 5.3).

### V-26-04-b  PARTLY   (first agent: MEDIUM; refuter: LOW for the stands part)
- Claim: the service methods are not named with parameters: PossessionEvidenceService appears only as a heading, with no attach, verify or sweep method, and the sweep entry point in worker::islamic has no name.
- Evidence (refuted part): every method a step calls is named in section 2: docs/plan/phase-26.md:267 "`PossessionEvidenceRepository`: `insert(tx, NewPossessionEvidence) -> PossessionEvidence`; `get_for_update(tx, id)`", docs/plan/phase-26.md:266 "`lock_for_share(tx, deal_id)`" (the IslamicDealRepository line), and the phase 02 call in docs/plan/phase-26.md:393. The sweep entry point is covered by the general dispatch rule: docs/plan/phase-00.md:540 "Dispatch to the registered handler (`trait JobHandler { const KIND: &'static str; async fn run(&self, ctx:", with the job kind named at docs/plan/phase-26.md:710.
- Evidence (stands part): PossessionEvidenceService is named only at docs/plan/phase-26.md:9 and 384 (heading). Its attach, verify and content methods have no names or signatures, and the planning rule asks for method signatures in service specs (.claude/rules/planning.md, section 3). No developer is blocked, because the steps call only named methods.
- Fix: phase 26, section 2: one line per service method of PossessionEvidenceService (attach, verify, content, sweep) with its parameters, pointing to 3.5.

### V-26-04-c  REFUTED   (first agent: LOW; refuter: none)
- Claim: the register insert (register.created) and the stage call (evidence.staged) are not named by kind in 3.12; 3.12 names only "the register write of phase 24".
- Evidence: docs/plan/phase-26.md:597 "Mutations that this phase performs through other phases keep those phases' events: `scenario_version.seeded` (phase 16), the register write of phase 24". The register event is phase 24's: docs/plan/phase-24.md:1080 "`register.created` | a finding or warning created (id, kind, source, severity, repeat flag)". The stage event belongs to the stage call itself: docs/plan/phase-02.md:853 "emit `evidence.staged {owner_kind, owner_id, kind, content_type, size_bytes}`". Every mutation has its event named, so no gap; only the kind is not repeated in phase 26.

### V-26-04-d  STANDS   (first agent: LOW; refuter: LOW)
- Claim: step 4 calls EvidenceStore::stage(tx, NewEvidence { ... }) with no actor argument, but phase 02 defines stage(tx, NewEvidence, actor).
- Evidence: docs/plan/phase-26.md:393 "4. `EvidenceStore::stage(tx, NewEvidence { kind: upload, owner_kind: "islamic_deal", owner_id: deal_id, retention_class: R3, content_type," (no actor). docs/plan/phase-02.md:839 "`stage(tx, NewEvidence, actor) -> EvidenceObject`".
- Fix: phase 26, section 3.5 step 4: add the actor argument to the stage call, as phase 02 defines it.

### V-26-04-e  STANDS   (first agent: LOW; refuter: LOW)
- Claim: the CBS-row test is cited as "section 6, check 13", but section 6 has checks 1 to 12; the test is check 6, sub-item l.
- Evidence: docs/plan/phase-26.md:432 "so a test asserts both (section 6, check 13)". Section 6 numbers its checks 1 to 12 (docs/plan/phase-26.md:812-866; the last item is "12. Existing tests still pass"). The test is docs/plan/phase-26.md:843 "- l. CBS deal `D4` (loaded by the writer)". Grep "check 13" hits only line 432.
- Fix: phase 26, section 3.5 closing paragraph: point to "section 6, check 6 l".

### V-26-04-f  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: section 2 says duplicate_exists is used only to write the register warning, but 3.5 step 7 uses it at attach for the G1 count, and the sweep warning body is {"missing", "late"} only.
- Evidence: docs/plan/phase-26.md:269 "(used only to write the register warning, never returned to the caller)". docs/plan/phase-26.md:399 "A duplicate is recorded only in the G1 tile count of section 5 (for M)." The sweep body: docs/plan/phase-26.md:445 "body `{"missing": n, "late": n}`". Three other texts agree with 3.5 step 7: docs/plan/phase-26.md:682 "Duplicate document count (G1 tile) | M only", and check 6 m at docs/plan/phase-26.md:846 "the G1 tile count as `m@` is 1". The conflict is only the section 2 note, so MEDIUM.
- Fix: phase 26, section 2 (the duplicate_exists note): replace "used only to write the register warning" with "used by 3.5 step 7 to count duplicates for the G1 tile".

### V-26-05-a  PARTLY   (first agent: MEDIUM; refuter: LOW for the stands part)
- Claim: "above IQD 10,000,000 per loan" compares contract_amount in islamic_deal.currency with no currency rule: no conversion source, no skip and no refusal for a non-IQD deal.
- Evidence (refuted part): docs/DSL.md:83-84 "A threshold with no currency is refused (`E_MONEY_NO_CURRENCY`); IQD and USD are separate by default (T-04)." The threshold has a currency (IQD), and docs/DSL.md:235 "`separate` evaluates IQD and USD independently with their own thresholds (T-04)" keeps a USD deal away from the IQD threshold. docs/DSL.md:85 limits conversion to `amount_lcy` or `amount_usd` and the rule does not ask for it.
- Evidence (stands part): docs/plan/phase-26.md:52 "contract_amount NUMERIC(28,6)" has no lcy or usd companion (`amount_lcy` and `amount_usd` exist only on canon.txn, docs/plan/phase-10.md:959-960). Grep "conversion" in docs/plan/phase-26.md: none. Nothing in the phase says that SCN-ISL-06 reads IQD deals only. LOW, because the DSL default settles the behaviour and only the statement is missing.
- Fix: phase 26, section 3.6 row SCN-ISL-06 (and the parameter line at docs/plan/phase-26.md:477): say the rule reads IQD deals only, as the DSL default gives, or name the USD basis.

### V-26-05-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: party.related_to_staff is NULL until "a named derivation rule sets it", the phase names no such rule, and it does not say how SCN-ISL-06 treats a NULL.
- Evidence: docs/plan/phase-10.md:473 "related_to_staff BOOLEAN, -- null until a named derivation rule sets it, never false". Greps: "related_to_staff" in docs/plan/*.md hits only docs/plan/phase-10.md:473 and 804 and docs/plan/phase-26.md:468; "derivation" in docs/plan/phase-26.md hits only line 242 (the evidence-status derivation). NULL handling: docs/DSL.md has no rule on how a condition on a NULL evaluates (grep "not evaluable" finds nothing; grep "null" hits validation, default and window fields, for example docs/DSL.md:560 on mapping defaults only). The row at docs/plan/phase-26.md:468 lists `party.related_to_staff` and says nothing about NULL.
- Fix: phase 26, section 3.6 row SCN-ISL-06: name the derivation rule that sets related_to_staff (or say it is taken from the source), and say whether a NULL is not evaluable or counts as not related.

### V-26-05-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: SCN-ISL-07 does not define "full repayment" or "unpaid": which islamic_deal.status values, which date (closed_on), and which facility.events entries mark repayment.
- Evidence: docs/plan/phase-26.md:46 "status TEXT NOT NULL CHECK (status IN ('active', 'settled', 'closed', 'defaulted', 'cancelled', 'unknown'))" gives six values, none tied to repayment or unpaid. docs/plan/phase-26.md:469 "| SCN-ISL-07 | Repeat qard hasan within six months of full repayment, or while another is unpaid (hard rule)". docs/plan/phase-10.md:1320 "events JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(events) = 'array')" is untyped. Grep "repay" in docs/plan/phase-10.md, phase-03.md, docs/DATA-MODEL.md and docs/DSL.md: no hit.
- Fix: phase 26, section 3.6 row SCN-ISL-07: map which status values and which date mean full repayment and unpaid, and name the facility.events entry that records a repayment.

### V-26-05-d  PARTLY   (first agent: LOW; refuter: MEDIUM for the stands part)
- Claim: 3.1 step 2 says canon.facility.contract_type "is not used for this", but SCN-ISL-06 reads it to select qard hasan facilities, with no list of values.
- Evidence (refuted part, the contradiction): docs/plan/phase-26.md:308 "`canon.facility.contract_type` is free text and is not used for this". "This" is the deal's type, which 3.1 step 2 takes from the product row only. SCN-ISL-06 uses the field for a different job, selecting facilities for sources without agreements (docs/plan/phase-26.md:468). The two sentences do not contradict.
- Evidence (stands part): docs/plan/phase-10.md:1318 "contract_type TEXT," has no CHECK and no value list, and the logic uses it (docs/plan/phase-26.md:468 "`facility.contract_type` for sources without agreements"). Phase 26 never names the values that mark a qard hasan facility (grep "facility.contract_type": docs/plan/phase-26.md:308 and 468 only). This is a logic input with undefined values, so MEDIUM.
- Fix: phase 26, section 3.6 row SCN-ISL-06: list the facility.contract_type values that mark a qard hasan facility, or limit the test to the schedule source's product code.

### V-26-06-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: the freeze state does not wait for the group tasks. Phase 20's on_level moves the freeze to frozen on the first verified level, and 3.8 does not say whether the freeze waits for the restrict and hold groups.
- Evidence: docs/plan/phase-20.md:660 "(called by the verification of section 3.3)" and docs/plan/phase-20.md:661 "The freeze moves `listed` to `frozen` on the first level (V0 or better), with `frozen_at = now()`" (the on_level method, step 1). docs/plan/phase-20.md:608 "5. Owner: `on_level(v0)` (freeze: section 3.4...)" applies to any freeze-owned task. docs/plan/phase-26.md:520 "`freezes.ops_task_id` holds the `unrestricted` task when there is one, otherwise the first group's task." Greps with no hit in docs/plan/phase-26.md: "frozen" (no freeze-state rule anywhere in the phase), "every group", "all groups". The phase 20 7.1 block (docs/plan/phase-20.md:1050-1062) adds the groups but not the state rule.
- Fix: phase 26, section 3.8: state whether the freeze reaches frozen at the first verified group task or only when every group task has reached its first level.
- Severity: HIGH, because the frozen state is a legal state and would be set while accounts of another group are not yet restricted or held.

### V-26-06-b  REFUTED   (first agent: LOW; refuter: none)
- Claim: AccountClassMap is the parameter type of the pure rule (docs/plan/phase-26.md:280) and is defined nowhere.
- Evidence: the content of the type is stated in a code block and in prose: docs/plan/phase-26.md:730 "('islamic.account_class_map', '{"mudaraba": "pooled", "qard_hasan": "restricted"}', 'json', NULL, TRUE," and docs/plan/phase-26.md:739-740 "its values are validated by the handler against the three classes (`E_VALIDATION` at `/account_class_map`)". Section 3.8 step 1 (docs/plan/phase-26.md:510-511) gives its role. Under the rule that a type whose fields are stated anywhere is not a gap, the name alone is not a gap.

### V-26-06-c  STANDS   (first agent: LOW; refuter: LOW)
- Claim: 3.12 names `ops_task.created`, but phase 20 defines the kind as `ops.task_created`.
- Evidence: docs/plan/phase-26.md:598 "`ops_task.created` and `freeze.created` (phase 20, with the group counts of 3.8)". docs/plan/phase-20.md:541 "5. Audit `ops.task_created` (section 3.10)." and docs/plan/phase-20.md:781 "| `ops.task_created` | a task row (any owner) | ops_task |".
- Fix: phase 26, section 3.12: write the kind as `ops.task_created`.

### V-26-07-a  PARTLY   (first agent: MEDIUM; refuter: LOW for the stands part)
- Claim: AggregateCounts, the type that ShariaAggregateRepository::counts returns, is defined nowhere, and ShariaAggregateQuery appears only as a heading with no signature.
- Evidence (refuted part): the content of AggregateCounts is stated in prose: docs/plan/phase-26.md:537 "3. Cells: deals per contract type with the status split (active, settled, closed, other); deals per evidence status; accounts on the Islamic book; and three gap rates". The repository signature is named at docs/plan/phase-26.md:270 "`counts(period) -> AggregateCounts`". The query inputs are stated at docs/plan/phase-26.md:529-533 (the route with `period`, and step 1 "`period` defaults to the current bank month").
- Evidence (stands part): AggregateCounts has no field list in a type or code block, and the heading at docs/plan/phase-26.md:529 names no method or return type of its own. LOW, since a developer can build it from 3.9 step 3 and the section 2 signature.
- Fix: phase 26, section 2: list the AggregateCounts fields (the cells of 3.9 step 3) and name the query in 3.9 as the counts method.

### V-26-07-b  STANDS   (first agent: LOW; refuter: LOW)
- Claim: section 2 cites "section 3.4" for the counts query and for ShariaAggregateView, but both are defined in section 3.9.
- Evidence: docs/plan/phase-26.md:270 "`counts(period) -> AggregateCounts` (one query with grouped counts; section 3.4)." docs/plan/phase-26.md:284 "`ShariaAggregateView` (section 3.4)". Section 3.4 is the deal read (docs/plan/phase-26.md:355, "### 3.4 Deal read and the deal panel"); the aggregates are at docs/plan/phase-26.md:529 "### 3.9 Sharia board aggregates".
- Fix: phase 26, section 2 at lines 270 and 284: change "section 3.4" to "section 3.9".

### V-26-07-c  PARTLY   (first agent: MEDIUM; refuter: LOW for the stands part)
- Claim: a malformed period (not YYYY-MM, or month 13) has no refusal: step 1 gives 422 only for a future month, and 3.11 lists only period_future.
- Evidence (refuted part): a general rule gives the refusal: docs/plan/phase-00.md:499 "`Validation(Vec<ValidationError>)` | 422 | `E_VALIDATION` (`errors[]` with JSON pointers)", and docs/API.md:985 "`E_VALIDATION` | 422 | Field or document validation". A malformed period is therefore a 422 E_VALIDATION at /period by the general rule.
- Evidence (stands part): no malformed-period sub-code is named: docs/plan/phase-26.md:574 lists `kind_invalid`, `document_ref_too_long`, `reject_code_required`, `reject_code_not_allowed`, `period_future` for E_VALIDATION. Grep "period_invalid" and "period_malformed" in docs/plan/phase-26.md: none. LOW.
- Fix: phase 26, section 3.11 Validation row and section 3.9 step 1: add a sub-code for a malformed period beside `period_future`.

### V-26-07-d  STANDS   (first agent: LOW; refuter: LOW)
- Claim: the cited "section 6, check 17" does not exist; section 6 has checks 1 to 12, and the rating-factor grep and the Cedar scope test are not where the text says.
- Evidence: docs/plan/phase-26.md:554 "(section 6, check 17)" (3.10 step 1) and docs/plan/phase-26.md:670 "Checks of the file (section 6, check 17)" (3.13). Section 6 numbers its checks 1 to 12 (docs/plan/phase-26.md:812-866), and the Cedar scope test is check 10 (docs/plan/phase-26.md:863 "10. The Cedar bundle validates"). The rating-factor grep is described in 3.10 step 1 itself, so only the pointer is wrong.
- Fix: phase 26, section 3.13 (line 670): point the reference to check 10; section 3.10 step 1 (line 554): point to the grep test in that step, or add it as a numbered check in section 6.

### V-26-08-a  PARTLY   (first agent: LOW; refuter: LOW for the stands part)
- Claim: the --owner refusal (3.7 step 2) has no code, and the go-live refused without pack.akr_live is named only in check 6 r and in phase 16, not in step 3.
- Evidence (refuted part, go-live): 3.7 step 3 cites the gate, docs/plan/phase-26.md:497 "Gate G9 (phase 16 section 3.4) reads `pack: akr` or `pack: islamic`", and phase 16 carries the code: docs/plan/phase-16.md:1078 "| G9 pack | `pack: akr` only with `pack.akr_live` on | `E_NOT_ALLOWED_IN_STATE` `pack_not_agreed` |". Phase 16 section 7.2 extends the gate to islamic (docs/plan/phase-16.md:1951 "G9 reads `pack: akr` or `pack: islamic`").
- Evidence (stands part, --owner): docs/plan/phase-26.md:496 "`--owner` is required: an Islamic scenario without a named owner is refused." names no code and no exit status. LOW.
- Fix: phase 26, section 3.7 step 2: name the refusal (an error code or the akr-ctl exit status) for a missing --owner.

### V-26-08-b  REFUTED   (first agent: LOW; refuter: none)
- Claim: the flag change of step 4 is named only as "the flag change of phase 03" in 3.12, and phase 03 defines the kind as flag.updated.
- Evidence: docs/plan/phase-26.md:597-598 "Mutations that this phase performs through other phases keep those phases' events: ... and the flag change of phase 03." The kind is phase 03's: docs/plan/phase-03.md:1307 "Audit `flag.updated` is unchanged." The mutation has its event named by its owner, so no mutation lacks one.

### V-26-08-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the document file packs/scenarios/islamic.v1.0.0.json is not in the release manifest that phase 16 names, and section 4.5 does not add it, so "signed release data" is not met.
- Evidence: docs/plan/phase-26.md:455 "The pack has eight scenario documents in `packs/scenarios/islamic.v1.0.0.json` (signed release data, phase 16 section 3.15)". docs/plan/phase-16.md:1513 "the release manifest lists `packs/scenarios/documents.v1.0.0.json` with its SHA-256 and signature". docs/plan/phase-16.md:1769 "(release manifest, `packs/scenarios/documents.v1.0.0.json`, verified as in phase 06)". Section 4.5 item 7 (docs/plan/phase-26.md:761) adds no manifest entry. Greps with no hit: "islamic.v1" in docs/plan/phase-16.md and phase-06.md; "documents.v1.0.0" in docs/plan/phase-26.md.
- Fix: phase 26, section 4.5 item 7: add the islamic document file to the release manifest of phase 16 with its SHA-256 and signature.

### V-26-08-d  STANDS   (first agent: LOW; refuter: LOW)
- Claim: the text says phase 16 rule 15 applies to islamic as to akr, but rule 15 covers pack akr only, and the phase 16 extension does not add it.
- Evidence: docs/plan/phase-26.md:457 "phase 16 rule 15 applies to `islamic` as to `akr`". docs/plan/phase-16.md:969 "15. Evidence label: `evidence_label` is required for `pack: akr` (`E_SCHEMA`)." The extension (docs/plan/phase-16.md:1950-1953) lists G9, the seed skip and the verify-data check, not rule 15, and docs/plan/phase-26.md:761 does not add it. The requirement itself is stated in phase 26 (docs/plan/phase-26.md:456 "`evidence_label` required"), so only the mechanism is missing.
- Fix: phase 26, section 4.5 item 7: extend phase 16 rule 15 to `pack: islamic`.

### V-26-09-a  REFUTED   (first agent: LOW; refuter: none)
- Claim: the issue rows of the three rules come from phase 12's recorder, whose event is dq.recorded; section 3.2 step 4 and 3.12 do not name it.
- Evidence: docs/plan/phase-12.md:974 "| `dq.recorded` | `DataQualityRecorder` per batch with issues (kinds and counts) | source |". Section 3.2 step 4 names that recorder: docs/plan/phase-26.md:330 "(DSL B.5 rules with `on_fail: dq_issue`, phase 12 `DataQualityRecorder`)". The event is phase 12's rule for that writer, so the mutation has an audit event named.

### V-26-10-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: "normalised document_ref" is not defined. Section 2 says only "trimmed", and the duplicate match depends on the normalised value.
- Evidence: docs/plan/phase-26.md:396 "5. `document_ref_hk = HMAC-SHA256(key purpose "islamic.document_ref", normalised document_ref)`". docs/plan/phase-26.md:260 "`document_ref: Option<String> (max 60 characters, trimmed)`". Greps with no hit: "normalis" in docs/plan/phase-26.md other than line 396; in phase 00 (7.15, docs/plan/phase-00.md:1001-1005, purposes only) and phase 10 (the transforms list at docs/plan/phase-10.md:1585 `Trim`, `Upper`, `Lower` names no document rule). docs/plan/phase-02.md:698 "no Unicode normalisation (the bytes stored are the bytes hashed)" concerns the audit encoding, not this input.
- Fix: phase 26, section 3.5 step 5: state the normalisation of document_ref as a numbered rule covering case, inner spaces and separators.

### V-26-11-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM for the stands part)
- Claim: the insert names ON CONFLICT (source_key) DO NOTHING, but uq_registers_source_key is a partial unique index, so without the WHERE predicate PostgreSQL cannot infer it and the sweep's insert fails at run time.
- Evidence (refuted part, run-time failure): docs/plan/phase-26.md:880 "Section 3.5 step 4 calls `RegisterWriter::warn(tx, 'scan', 'islamic_evidence_gap:' || bank_date," (block 7.2). The warn SQL is phase 24's: docs/plan/phase-24.md:1298-1299 "inserts a row of kind `warning` with `ON CONFLICT" and "(source_key) WHERE source_key IS NOT NULL DO NOTHING`" (block 7.7, heading at docs/plan/phase-24.md:1296). If the sweep calls warn, the predicate is present.
- Evidence (stands part): the predicate-less clause is still in the base text and in the section 2 port: docs/plan/phase-26.md:444 "`ON CONFLICT (source_key) DO NOTHING`" (3.5 step 4, which describes a direct insert) and docs/plan/phase-26.md:277 "`ON CONFLICT (source_key) DO NOTHING`" (ports). The index is partial: docs/plan/phase-24.md:75 "CREATE UNIQUE INDEX IF NOT EXISTS uq_registers_source_key ON admin.registers (source_key) WHERE source_key IS NOT NULL;". Block 7.2 does not say that the inline clause is replaced.
- Fix: phase 26, section 3.5 step 4 and section 2 ports: drop the inline ON CONFLICT clause and cite phase 24's warn form (block 7.7).

### V-26-11-b  REFUTED   (first agent: LOW; refuter: none)
- Claim: the register insert's event register.created (phase-24.md:1080) is not named in 7.2 or 3.5, and 3.12 says only "the register write of phase 24".
- Evidence: docs/plan/phase-26.md:597 "Mutations that this phase performs through other phases keep those phases' events: ... the register write of phase 24". The event is phase 24's: docs/plan/phase-24.md:1080 "`register.created` | a finding or warning created (id, kind, source, severity, repeat flag) | `register` (key)". The warning is covered by that row, so the mutation has its event named.

### V-26-11-c  STANDS   (first agent: LOW; refuter: LOW)
- Claim: block 7.2 says "the bilingual title in the message catalogue", but phase 24 keeps the bilingual title in the closed WarningTitle map, and "message catalogue" is defined only for DSL messages.
- Evidence: docs/plan/phase-26.md:883 "the message catalogue. The insert-or-nothing on `source_key` keeps one row per bank day." (block 7.2, which also says the variant goes into WarningTitle). docs/plan/phase-24.md:1300 "`WarningTitle` is a closed enum in `domain::governance` that maps each key to `title {ar, en}`". docs/plan/phase-07.md:433 "rendered from the message catalogue" (DSL messages only). Grep "message catalogue" in docs/plan/phase-24.md and phase-00.md: none.
- Fix: phase 26, block 7.2: name the WarningTitle map of phase 24 as the place of the bilingual title.

## Summary

- REFUTED: 6 (V-26-02-d, V-26-04-c, V-26-06-b, V-26-08-b, V-26-09-a, V-26-11-b)
- STANDS: 20
- PARTLY: 8

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-26-01-a | STANDS | MEDIUM | The family value that marks an Islamic product is not named; 3.10 step 3 says the Islamic product shares its family with a conventional one | 3.1 step 2; 3.10 step 3 |
| V-26-01-b | STANDS | MEDIUM | Test 3(d) expects a refusal that the UNIQUE does not give (only an equal valid_from); no refusal for a second Map exists | 3.1 step 3(d) |
| V-26-02-a | PARTLY | MEDIUM | The list paragraph reuses steps 1 to 6, which read one deal by id; no list steps and no service method name | 3.4 list paragraph |
| V-26-02-b | STANDS | MEDIUM | Step 6 passes the class record_open; no StepUpAction constant is declared for the phase | section 3 (before 3.4); 3.4 step 6; 3.5 verify step 2 |
| V-26-02-c | STANDS | MEDIUM | Display name is read from canon.party_name without the superseded_at IS NULL filter | 3.4 step 7 |
| V-26-03-a | STANDS | MEDIUM | SCN-ISL-01 match conditions are not written (seller and supplier roles, agent and related-party match, contact and account comparisons) | 3.6 row SCN-ISL-01 |
| V-26-03-b | STANDS | MEDIUM | "His agent" and "a related party" have no party_relationship type and no deal role mapping | 3.6 row SCN-ISL-01 |
| V-26-04-a | STANDS | HIGH | No method computes the two G1 tile counts; the duplicate count has no method or defined aggregate | section 2 repository traits; 5.3 |
| V-26-04-b | PARTLY | LOW | PossessionEvidenceService methods (attach, verify, content, sweep) and their parameters are not written | section 2 |
| V-26-04-d | STANDS | LOW | The stage call in 3.5 step 4 omits the actor argument that phase 02 defines | 3.5 step 4 |
| V-26-04-e | STANDS | LOW | Pointer to "section 6, check 13", which does not exist (the test is check 6 l) | 3.5 closing paragraph |
| V-26-04-f | STANDS | MEDIUM | Section 2 says duplicate_exists only writes the register warning; 3.5 step 7 uses it for the G1 count | section 2 (duplicate_exists note) |
| V-26-05-a | PARTLY | LOW | The rule does not say that SCN-ISL-06 reads IQD deals only (the DSL default implies it) | 3.6 row SCN-ISL-06 |
| V-26-05-b | STANDS | MEDIUM | The derivation rule for related_to_staff is not named; NULL handling is not stated | 3.6 row SCN-ISL-06 |
| V-26-05-c | STANDS | MEDIUM | "Full repayment" and "unpaid" are not mapped to status values, a date or facility events | 3.6 row SCN-ISL-07 |
| V-26-05-d | PARTLY | MEDIUM | The values that mark a qard hasan facility (facility.contract_type, free text) are not listed | 3.6 row SCN-ISL-06 |
| V-26-06-a | STANDS | HIGH | No freeze-state rule for group tasks; the first verified group task moves the freeze to frozen under phase 20 | 3.8 |
| V-26-06-c | STANDS | LOW | 3.12 writes ops_task.created; phase 20 defines the kind as ops.task_created | 3.12 |
| V-26-07-a | PARTLY | LOW | The aggregate query has no name of its own or signature; AggregateCounts has no field list | section 2; 3.9 |
| V-26-07-b | STANDS | LOW | Section 3.4 is cited for the aggregates, which are defined in 3.9 | section 2 (lines 270 and 284) |
| V-26-07-c | PARTLY | LOW | No sub-code for a malformed period (the 422 itself comes from the general E_VALIDATION rule) | 3.11 Validation row; 3.9 step 1 |
| V-26-07-d | STANDS | LOW | Pointer to "section 6, check 17", which does not exist (the Cedar scope test is check 10) | 3.13 (line 670); 3.10 step 1 (line 554) |
| V-26-08-a | PARTLY | LOW | The --owner refusal names no error code or exit status | 3.7 step 2 |
| V-26-08-c | STANDS | MEDIUM | The islamic document file is not in the phase 16 release manifest | 4.5 item 7 |
| V-26-08-d | STANDS | LOW | Phase 16 rule 15 (evidence_label) is not extended to pack islamic | 4.5 item 7 |
| V-26-10-a | STANDS | MEDIUM | "Normalised document_ref" is not defined | 3.5 step 5 |
| V-26-11-a | PARTLY | MEDIUM | The predicate-less ON CONFLICT clause remains in 3.5 step 4 and in the section 2 ports, although block 7.2 routes the insert through phase 24's warn | 3.5 step 4; section 2 ports |
| V-26-11-c | STANDS | LOW | Block 7.2 places the register title in "the message catalogue"; the WarningTitle map holds it | block 7.2 |
