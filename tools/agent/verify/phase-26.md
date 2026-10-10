# Verification pass, phase 26 (Haiku draft for lead review)

Scope: docs/plan/phase-26.md (958 lines, read in full). Cross-checked against docs/plan/phase-00, 01, 02, 03, 04, 06, 09, 10, 11, 12, 16, 17, 18, 20, 21, 23, 24, research.md, docs/API.md, docs/DESIGN-SYSTEM.md, docs/DATA-MODEL.md. Line numbers below are phase-26.md unless a file is named.

## Items sampled (12)

The header "Implemented here" list has 16 IDs. Five were picked from the start, middle and end of that list (positions 1, 5, 9, 12 and 14). The phase has three "### 7.x" blocks (7.1, 7.2, 7.3), all from the gap pass, and no block headed "Adversarial pass" (grep -i adversarial: no match). So the fourth 7.x slot and the three adversarial slots go to header items, giving nine header items and three block items.

1. V-26-01: I-01, taxonomy and product code map (header position 1; sections 1.1, 3.1)
2. V-26-02: I-02, deal model, parties and deal read (header position 2; sections 1.1, 2, 3.2, 3.4)
3. V-26-03: I-04, third-party seller rule SCN-ISL-01 (header position 4; section 3.6)
4. V-26-04: I-05, possession evidence, routes and sweep (header position 5; sections 1.3, 2, 3.5)
5. V-26-05: I-08 (partly), SCN-ISL-06 and SCN-ISL-07 (header position 8; section 3.6)
6. V-26-06: I-10, pooled-fund freeze by account class (header position 9; section 3.8)
7. V-26-07: I-16, Sharia board aggregates and route (header position 10; section 3.9)
8. V-26-08: I-20, the eight library documents and pack install (header position 12; sections 3.6, 3.7)
9. V-26-09: I-26, data-quality rules (header position 14; section 3.2)
10. V-26-10: block 7.1, home of the keyed-hash purpose
11. V-26-11: block 7.2, register warning call (DR-388)
12. V-26-12: block 7.3, party predicate form

## Results by item

### V-26-01 Islamic contract type and product code map   (source: header I-01; result: GAPS 2)
- T1 SQL: PASS - the migration drops `product_islamic_contract_type_check`, which is the default name of the inline CHECK on `canon.product.islamic_contract_type` (phase-10.md:400). The twelve values match DR-247 (phase-26.md:33-34, 897).
- T2 Route: N/A - no route; the product map is read by the ingest path (phase-26.md:304-306).
- T3 Repository and types: PASS - the code map is phase 10's (`admin.code_maps` kind `product`, phase-10.md:130). No step calls a new method. The heading names `ProductTypeMap` (phase-26.md:302), but no step calls it, so this is not a gap.
- T4 Logic: GAP (see V-26-01-a) - the refusal test "whose family is Islamic" (phase-26.md:309) has no defined value. The `unmapped_code` path itself is complete: writer returns the kind, field = product (phase-26.md:309-311; phase-12.md:739).
- T5 Audit: N/A - the migration is not audited; code-map changes go through change request `cr_mapping` (phase-10.md:2034-2039). The refused row becomes a DQ issue, covered by V-26-09.
- T6 Cross-references: PASS for `admin.code_maps` kind product (phase-10.md:130), `unmapped_code` kind (phase-12.md:177, 739), `cr_mapping` (phase-04.md:1268) and `akr-ctl mapping lint` (phase-10.md:2178). GAP (see V-26-01-b) for `uq_code_map_entries_effective`, which the test in phase-26.md:315 says refuses a code mapped to two types.
- T7 Consistency: PASS - the DDL list (phase-26.md:33-34) and DR-247 (phase-26.md:897) agree. No 7.x block changes I-01.
- GAP V-26-01-a | MEDIUM | T4 | the family value (or code-map kind) that makes a product "Islamic" is not named, so the refusal in 3.1 step 2 and test 3(b) cannot be coded | section 3.1 step 2: name the family value or code-map kind that marks an Islamic product | greps: "family is Islamic" in phase-26.md only; canon.product.family is a bare TEXT column with no value list (phase-10.md:399); DATA-MODEL.md:387 lists no values; research.md has no product family rule
- GAP V-26-01-b | LOW | T6 | the test says uq_code_map_entries_effective refuses a code mapped to two types, but that UNIQUE is (code_map_id, source_list, source_code, valid_from) (phase-10.md:162), so only an equal valid_from is refused; phase 10 has no overlap rule for entries (overlap appears only as a source column (phase-10.md:58) and as the re-read window (phase-10.md:281, 288, 1840, 2149)) | section 3.1 step 3(d): state the rule that refuses two types, or restrict the test to equal valid_from | greps: "overlap" in phase-10.md (no entry-overlap rule); "two types" in phase-10.md (none)

### V-26-02 Deal model, deal read and parties   (source: header I-02; result: GAPS 4)
- T1 SQL: PASS - `canon.islamic_deals` and `canon.deal_parties` (phase-26.md:38-96) hold every column the logic reads (3.4 step 2, 3.5, 3.9 step 2). Columns named from other phases exist: `canon.customer.party_id` and `recorded_to` (phase-10.md:599-629), `canon.party_name` with `superseded_at` (phase-10.md:495; grant at phase-10.md:776).
- T2 Route: PASS - `GET /islamic/deals` and `GET /islamic/deals/{id}` are in the route table (phase-26.md:289-290) with scope `queue.work` (A1, A2, M; API.md:85) and Cedar `islamic.deal.read` (phase-26.md:626-637).
- T3 Repository and types: PASS - `IslamicDealRepository` (phase-26.md:265-266) and `IslamicDealService::get(ctx, deal_id)` (phase-26.md:357) are named with parameters; `IslamicDeal`, `DealParty`, `IslamicDealView` and `IslamicDealSummary` are defined (phase-26.md:246-251, 281-282). The list route's service method is not named (see V-26-02-a).
- T4 Logic: PASS for the detail read, steps 1 to 10 (phase-26.md:360-378), with 404 (step 1, 2), 401 E_STEPUP_REQUIRED (step 6) and 422 E_PURPOSE_REQUIRED (phase-26.md:358). GAPS for the list route (V-26-02-a, V-26-02-c).
- T5 Audit: PASS for the read - access event `view` (phase-26.md:376), and "The read writes no audit row" (phase-26.md:377). The deal load writes canon rows (3.2 step 3); see V-26-02-d.
- T6 Cross-references: GAP (see V-26-02-b) for `StepUpService::consume`. PASS for `casework.effective_tier` (phase-17.md:530), `PartyReadPredicate::allows` (phase-18.md:1317), the `Party` entity with `tier` and `case_team` (phase-18.md:971), purposes `assigned_work` and `investigation` (phase-03.md:131, 133), access action `view` (phase-02.md:151) and `pack.islamic` (phase-06.md:318).
- T7 Consistency: PASS - 7.3 (phase-26.md:886-889) names the predicate that step 7 calls (phase-26.md:370) and that section 2 lists (phase-26.md:275).
- GAP V-26-02-a | MEDIUM | T4 | the list route `GET /islamic/deals?customer_id=` is one prose sentence: no numbered steps, no service method name, no error for a missing or malformed customer_id or cursor, and "each candidate customer" is unclear for one customer_id | section 3.4 list paragraph: numbered steps, the method name and the errors | greps: "list_for_customer" in phase-26.md (repository only, line 265); "candidate customer" in phase-26.md (line 380 only); "customer_id" in phase-26.md (lines 289, 380)
- GAP V-26-02-b | MEDIUM | T6 | step 6 passes the bare class `record_open`; phase 01 defines `consume(tx, ctx, header, action: StepUpAction, resource_ref)` (phase-01.md:731), and the phase declares its actions as `StepUpAction { name, class }` constants (phase-01.md:711), so phase 26 needs declared constants and passes none | section 3.4 step 6 and section 3 (declare the StepUpAction constants with name and class) | greps: "StepUpAction" in phase-26.md (none); "StepUpAction {" in phase-01, phase-06, phase-21, phase-22, phase-23 and phase-29 (declared there)
- GAP V-26-02-c | LOW | T4 | the display name is read from `canon.party_name` with no current-row filter, so a superseded name could be returned; the table carries `superseded_at` (phase-10.md:776) | section 3.4 step 7: add `superseded_at IS NULL` | greps: "superseded_at IS NULL" in phase-26.md (lines 107, 108, 110, 266, 369 are for deal_parties only); "party_name" in phase-26.md (3.4 step 7 only)
- GAP V-26-02-d | LOW | T5 | the deal load (3.2 step 3) writes `canon.islamic_deals` and `canon.deal_parties`; the audit event is phase 11's `batch.recorded` (phase-11.md:1191), which neither 3.2 nor 3.12 names | section 3.12: name the batch event for the deal load | greps: "batch.recorded" in phase-26.md (none)

### V-26-03 Third-party seller rule SCN-ISL-01   (source: header I-04; result: GAPS 2)
- T1 SQL: PASS - every field the row reads exists: `islamic_deal.contract_type` (phase-26.md:43), `deal_party.role` and `party_id` (phase-26.md:83-84), `party_contact.value_hk` (phase-10.md:552), `account_party_role.party_id` (phase-10.md:668-672), `account.number_hk` and `iban_hk` (phase-10.md:634-635), `party_relationship.type` (phase-10.md:572-575). `device_logs` is marked not held (phase-26.md:463) and is not read.
- T2 Route: N/A - library scenario, no route (phase-26.md:455-459).
- T3 Repository and types: N/A - no service or repository; the document runs in phase 16's engine (phase-26.md:712).
- T4 Logic: GAP (see V-26-03-a) - the rule is one phrase in the table: "Seller or supplier is the customer, his agent or a related party (hard rule)" (phase-26.md:463). It gives no match conditions.
- T5 Audit: N/A - the item writes no rows; scenario alerts are written by phases 16 and 17 (phase-26.md:485-486).
- T6 Cross-references: GAP (see V-26-03-b) - "agent" and "related party" are not `party_relationship.type` values. PASS for `visibility` (phase-06.md:90), `research_grade` (phase-06.md:87), `requires_flag` (phase-06.md:89) and `owner_id` (phase-06.md:93).
- T7 Consistency: PASS - no 7.x block changes SCN-ISL-01. The seller-is-customer condition is the further condition that 3.10 step 2 requires (phase-26.md:555-557).
- GAP V-26-03-a | MEDIUM | T4 | the SCN-ISL-01 match is not written: which deal_party roles count as seller or supplier, how "his agent" and "a related party" are matched, and what party_contact.value_hk and account_party_role are compared with | section 3.6 row SCN-ISL-01: numbered conditions, or the DSL document's conditions cited by name | greps: "relationship.*agent" in research.md and all phase files (none); research.md:262 (DR-251) gives the count only, not the rule text
- GAP V-26-03-b | MEDIUM | T6 | the rule needs an agent relation and a related-party relation, but the party_relationship type CHECK has neither value (owner, controller, director, signatory, attorney, guardian, family, employee_of, associate, unknown; phase-10.md:572-575, DATA-MODEL.md:378). The phase does not map them to these values or to deal_party.role = agent (phase-26.md:84-85) | section 3.6 row SCN-ISL-01: state the mapping | greps: "agent" in the phase-10 party_relationship CHECK (none); "related" in phase-10.md (none); "related" in DATA-MODEL.md only as a case-party role (line 630), not as a relationship type

### V-26-04 Possession evidence: attach, content, verify and sweep   (source: header I-05; result: GAPS 6)
- T1 SQL: PASS - `kyc.possession_evidence` with its indexes (phase-26.md:159-184), `kyc.deal_evidence_status` (phase-26.md:188-195) and grants (phase-26.md:198-201). `admin.evidence_objects` with `av_status` value `clean` (phase-02.md:350, 368) and `admin.users` (phase 01) exist elsewhere. Settings `islamic.evidence_tracking` and `islamic.evidence_due_days` are inserted in 4.3 (phase-26.md:724-726).
- T2 Route: PASS - POST attach, GET content and POST verify are in the table (phase-26.md:291-293) with scope `queue.work` (A1, A2, M; API.md:85). The Cedar actions attach, read and verify are in the file (phase-26.md:625-638), with verify narrowed to A2 and M.
- T3 Repository and types: GAP (see V-26-04-a) for the G1 tile counts, and GAP (see V-26-04-b) for missing service method signatures. PASS for `PossessionEvidenceRepository` (phase-26.md:267-269) and the request and response types `AttachEvidence`, `DecideEvidence` and `PossessionEvidenceView` (phase-26.md:260-263, 282-284).
- T4 Logic: PASS for attach steps 1 to 9 (phase-26.md:387-405), content steps 1 to 3 (phase-26.md:409-414), verify steps 1 to 9 (phase-26.md:418-429) and sweep steps 1 to 5 (phase-26.md:436-446). Errors are given per refusal (for example 422 E_VALIDATION at /kind, 413 E_PAYLOAD_TOO_LARGE, 409 E_NOT_ALLOWED_IN_STATE `already_decided`, 403 E_SELF_APPROVAL). Lock order is stated (phase-26.md:392, 422, 699-702).
- T5 Audit: PASS for the four named events `islamic.evidence.attached`, `verified`, `rejected` and `swept` (phase-26.md:590-593; steps 9 and 5). GAP (see V-26-04-c) for the events of the register insert and of the stage call.
- T6 Cross-references: PASS for `RegisterWriter::warn` (phase-24.md:1297-1298), `GET /api/v1/registers/{kind}` with dashboard.read (phase-24.md:555), `evidence.lock_days` with R3 at 2555 (phase-02.md:510-511), `NewEvidence` fields (phase-02.md:611-612), the access actions `view` and `download` (phase-02.md:151) and the API codes used (API.md:965-1002). GAP (see V-26-04-d) for the stage call's arguments. GAP (see V-26-04-e) for the section pointer.
- T7 Consistency: GAP (see V-26-04-f) for `duplicate_exists`. PASS for the 7.1 and 7.2 corrections, which replace the base text they name (phase-26.md:874-875, 880-884).
- GAP V-26-04-a | HIGH | T3 | no repository or service method computes the two G1 tile counts: "count of duplicate document references" (phase-26.md:797) and "deals awaiting possession evidence" (phase-26.md:794). `duplicate_exists` returns only a bool and is "never returned to the caller" (phase-26.md:269); the aggregate repository has no duplicate count | section 2 repository traits (add the count methods with their SQL predicate and visibility) and section 5.3 | greps: "duplicate count" in phase-26.md (Open doubts 11, line 931 only), "awaiting" in phase-26.md (5.3, line 794 only), "dup_count" (none); "Islamic" in phase-24.md (none); research.md (none)
- GAP V-26-04-b | MEDIUM | T3 | the service methods are not named with parameters: `PossessionEvidenceService` appears only as a heading (phase-26.md:384) with no attach, verify or sweep method; the sweep entry point in `worker::islamic` (phase-26.md:244) has no name | section 2 (name the service methods and the sweep function, with parameters) | greps: "PossessionEvidenceService" in phase-26.md (lines 9 and 384 only); "PossessionEvidenceService" in phase-02 and phase-24 (none)
- GAP V-26-04-c | LOW | T5 | the events of two mutations are not named by kind: the register insert (its kind is `register.created`, phase-24.md:1080) and the stage call (`evidence.staged`, phase-02.md:853). Section 3.12 names only "the register write of phase 24" (phase-26.md:597) | section 3.12: name both kinds | greps: "register.created" and "evidence.staged" in phase-26.md (none)
- GAP V-26-04-d | LOW | T6 | step 4 calls `EvidenceStore::stage(tx, NewEvidence { ... })` with no actor argument; phase 02 defines `stage(tx, NewEvidence, actor)` (phase-02.md:839) | section 3.5 step 4: add the actor argument | greps: "stage(tx, NewEvidence, actor)" in phase-02.md:839 (the signature the call must match)
- GAP V-26-04-e | LOW | T6 | the CBS-row test is cited as "section 6, check 13" (phase-26.md:432). Section 6 has checks 1 to 12 (phase-26.md:814-866); the test is check 6, sub-item l (phase-26.md:843-844) | section 3.5 closing paragraph: point to check 6 l | greps: "check 13" in phase-26.md (line 432 only)
- GAP V-26-04-f | MEDIUM | T7 | section 2 says `duplicate_exists` is "used only to write the register warning" (phase-26.md:269), but 3.5 step 7 uses it at attach for the G1 count (phase-26.md:399), and the sweep warning body is {"missing", "late"} only (phase-26.md:445) | section 2 (the duplicate_exists note) and 3.5 step 7: state one use | greps: "duplicate" in the sweep steps 3 to 5 (phase-26.md:440-446): no duplicate count

### V-26-05 Qard hasan scenarios SCN-ISL-06 and SCN-ISL-07   (source: header I-08, partly; result: GAPS 4)
- T1 SQL: PASS - `facility.limit_amount`, `currency` and `contract_type` (phase-10.md:1310-1334), `facility.events` (a JSONB column of canon.facility, phase-10.md:1320), `party.staff_flag` and `party.related_to_staff` (phase-10.md:472-473), `customer.party_id` (phase-10.md:601), and the islamic_deal columns `contract_amount`, `opened_on`, `closed_on` and `status` (phase-26.md:46-54).
- T2 Route: N/A - library scenarios, no route (phase-26.md:455-459).
- T3 Repository and types: N/A - no new method; the scenarios run in phase 16's engine.
- T4 Logic: GAP (see V-26-05-a, V-26-05-b, V-26-05-c). PASS for the parameters, which carry basis tags and min and max (phase-26.md:474-479).
- T5 Audit: N/A - no mutation; alerts are written by phases 16 and 17 (phase-26.md:485-486).
- T6 Cross-references: PASS - every canonical field in the two rows exists (see T1). The scenario visibility and grade rules are phase 06 and 16 (phase-26.md:485-486; phase-06.md:87, 90).
- T7 Consistency: GAP (see V-26-05-d). 3.1 step 2 says `canon.facility.contract_type` "is not used for this" (phase-26.md:308). The SCN-ISL-06 row reads it (phase-26.md:468).
- GAP V-26-05-a | MEDIUM | T4 | "above IQD 10,000,000 per loan" compares `contract_amount` in `islamic_deal.currency` with no currency rule: no conversion source, no skip and no refusal for a non-IQD deal. `canon.txn` has amount_lcy and amount_usd (phase-10.md:959-960), but deals have none | section 3.6 row SCN-ISL-06: state the currency rule | greps: "qard_cap_iqd" and "conversion" in phase-26.md (no currency rule); E_FX_RATE_MISSING exists (API.md:983) but is not referenced by the phase
- GAP V-26-05-b | MEDIUM | T4 | `party.related_to_staff` is NULL until "a named derivation rule sets it" (phase-10.md:473). The phase names no such rule and does not say how SCN-ISL-06 treats a NULL (not evaluable, or not related). Readiness A or B (phase-26.md:468) does not cover it | section 3.6 row SCN-ISL-06: the NULL rule and the derivation rule's name | greps: "derivation" in phase-26.md (line 242 only, the evidence-status derivation); "related_to_staff" in phase-10.md and DATA-MODEL.md only
- GAP V-26-05-c | MEDIUM | T4 | SCN-ISL-07 does not define "full repayment" or "unpaid": which `islamic_deal.status` values (phase-26.md:46) and which date (`closed_on`) mark repayment, and which `facility.events` entries show repayments, when `events` is an untyped JSONB array (phase-10.md:1320) | section 3.6 row SCN-ISL-07: define the status, date and event rules | greps: "repay" in phase-10.md, phase-03.md and DATA-MODEL.md (none); "repayment" in phase-26.md (line 469 only)
- GAP V-26-05-d | LOW | T7 | section 3.1 step 2 says the free-text `canon.facility.contract_type` "is not used for this" (phase-26.md:308), but the SCN-ISL-06 row uses it to select qard hasan facilities (phase-26.md:468) with no list of values | section 3.1 step 2 and the SCN-ISL-06 row: say which values identify a qard hasan facility, or limit the test to schedule sources | greps: "facility.contract_type" in phase-26.md (lines 308 and 468 only); canon.facility.contract_type is TEXT with no CHECK (phase-10.md:1318)

### V-26-06 Pooled-fund freeze by account class   (source: header I-10; result: GAPS 3)
- T1 SQL: PASS - `canon.account.product_id` (phase-10.md:630-647) and `canon.product.islamic_contract_type` (phase-10.md:400); `casework.ops_tasks` with `source_id` (phase-20.md:56), `neutral_text_code` CHECK including `ops.hold.account` and `ops.restrict.account` (phase-20.md:58), `second_person_only` CHECK (phase-20.md:84); `freezes.ops_task_id` (phase-20.md:131). The index on (source_kind, source_id) is not unique (phase-20.md:87), as 3.8 step 4 says.
- T2 Route: N/A - no route in this phase; the freeze start belongs to phase 20 (phase-20.md:640-646).
- T3 Repository and types: PASS for `OpsTaskService::create` (phase-20.md:536) and `FreezeService::propose_release` (phase-20.md:680). GAP (see V-26-06-b) for `AccountClassMap`.
- T4 Logic: GAP (see V-26-06-a). PASS for steps 1 to 8 otherwise (phase-26.md:510-527): the class mapping, the grouping, the action and neutral-code mapping, and the lock order (phase-26.md:701).
- T5 Audit: PASS - `freeze.created` with the group counts (phase-26.md:526; phase-20.md:791) and the release tasks under phase 20's `freeze.release_proposed` (phase-20.md:800). The task event kind is mis-named (see V-26-06-c).
- T6 Cross-references: GAP (see V-26-06-c). PASS for `FreezeService::start` step 6 (phase-20.md:646), section 3.3 verification (phase-20.md:574), section 3.13 lock order (phase-20.md:910) and the neutral codes (phase-20.md:58).
- T7 Consistency: PASS - DR-252 (phase-26.md:907-909), step 3 (phase-26.md:517-519) and the lead correction (phase-26.md:917) agree with the phase 20 CHECK (phase-20.md:84).
- GAP V-26-06-a | HIGH | T4 | the freeze state does not wait for the group tasks. Phase 20's `on_level` moves the freeze to `frozen` on the first verified level, and it is called by the verification of any task of the freeze (phase-20.md:660-661). Section 3.8 names only `freezes.ops_task_id` (phase-26.md:520-521) and does not say whether the freeze waits for the restrict and hold groups | section 3.8: state that the freeze is `frozen` only when every group task has reached its first level, or state the first-level rule per group | greps: "on_level" in phase-26.md (none); "every group" and "all groups" in phase-26.md (none); phase-20.md:133 says frozen_at is the first level "on the freeze task"
- GAP V-26-06-b | LOW | T3 | `AccountClassMap` is a parameter type of the pure rule (phase-26.md:280) and is defined nowhere | section 2 pure rules: define it as the parsed setting with the three class values | greps: "AccountClassMap" in phase-26.md (line 280 only); research.md and all phase files (none)
- GAP V-26-06-c | LOW | T6 | section 3.12 names `ops_task.created` (phase-26.md:598); phase 20 defines the kind as `ops.task_created` (phase-20.md:541, 781) | section 3.12: use `ops.task_created` | greps: "ops_task.created" in phase-20.md (none); "ops.task_created" in phase-20.md (lines 541, 781)

### V-26-07 Sharia board aggregates   (source: header I-16; result: GAPS 4)
- T1 SQL: PASS - the deal fields (phase-26.md:38-69), `kyc.deal_evidence_status` (phase-26.md:188-195), `canon.account.islamic_book` (phase-10.md:648) and the setting `islamic.dq_failure_rate` (phase-26.md:728) all exist.
- T2 Route: PASS - `GET /islamic/sharia-board/aggregates` with scope `dashboard.read` (phase-26.md:294; API.md:99 admits M, CH, IA) and Cedar `sharia.aggregate.read` for M, CH and IA (phase-26.md:640-651).
- T3 Repository and types: GAP (see V-26-07-a) for `AggregateCounts`, and GAP (see V-26-07-b) for the section pointers. PASS for `ShariaAggregateRepository::counts(period)` as a named method with a parameter (phase-26.md:270).
- T4 Logic: GAP (see V-26-07-c) for a malformed period. PASS for steps 2 to 7 (phase-26.md:534-545): the one read-only statement, the suppression rule with its setting (phase-26.md:539-540) and the control-failure flag (phase-26.md:541-542).
- T5 Audit: PASS - read only: an access event (phase-26.md:545), and reads write no audit row (phase-26.md:599).
- T6 Cross-references: PASS for `report.small_number_threshold` (phase-21.md:518: governed, minimum 3) and `dashboard.read` (API.md:99). GAP (see V-26-07-d) for the cited check.
- T7 Consistency: PASS - step 1 and section 3.11 agree on `period_future` (phase-26.md:533, 574). Section 3.14 and step 4 agree on suppression (phase-26.md:687, 539-540). DR-250 (phase-26.md:903) and step 6 (phase-26.md:543-544) agree that no identifiers are returned.
- GAP V-26-07-a | MEDIUM | T3 | `AggregateCounts`, the type `ShariaAggregateRepository::counts` returns (phase-26.md:270), is defined nowhere; `ShariaAggregateQuery` appears only as a heading (phase-26.md:529) with no signature | section 2: define AggregateCounts (or name ShariaAggregateView's fields as its source) and give the query signature in 3.9 | greps: "AggregateCounts" and "ShariaAggregateQuery" in phase-26.md only; research.md and all phase files (none)
- GAP V-26-07-b | LOW | T3 | section 2 cites "section 3.4" for the counts query (phase-26.md:270) and for `ShariaAggregateView` (phase-26.md:284); both are defined in section 3.9 (phase-26.md:529, 543) | section 2, lines 270 and 284: change to section 3.9 | greps: "section 3.4" in section 2 (lines 270, 284); section 3.4 is the deal read (phase-26.md:355)
- GAP V-26-07-c | MEDIUM | T4 | a malformed period (not YYYY-MM, or month 13) has no refusal: step 1 gives 422 only for a future month (phase-26.md:533), and section 3.11 lists only `period_future` (phase-26.md:574) | section 3.9 step 1 and the 3.11 sub-codes: add the malformed-period sub-code and its status | greps: "period_invalid" and "period_malformed" in phase-26.md (none); E_VALIDATION is 422 (API.md:985)
- GAP V-26-07-d | LOW | T6 | the cited check "section 6, check 17" does not exist; section 6 has checks 1 to 12 (phase-26.md:814-866). The text cites it at phase-26.md:554 (rating-factor grep, which has no check) and phase-26.md:670 (Cedar file). The Cedar scope test is in check 10 (phase-26.md:863-864) | section 3.13 (point to check 10) and section 3.10 step 1 (add a check for the rating-factor grep) | greps: "check 17" in phase-26.md (lines 554 and 670 only)

### V-26-08 Eight library documents and pack install   (source: header I-20; result: GAPS 4)
- T1 SQL: PASS - `scn.scenarios` columns `pack`, `requires_flag`, `visibility`, `research_grade` and `owner_id` (phase-06.md:59, 87-93); `scn.scenario_versions` with `shadow_days_required` and its 30-day floor (phase-16.md:82).
- T2 Route: N/A - no route in this phase; the go-live gate is G9 (phase-16.md:1061-1078) and the pack seed is a command.
- T3 Repository and types: PASS - `akr-ctl packs seed` (phase-16.md:1513; phase-26.md:495, 762), the G9 gate `GoLiveGate::check` (phase-16.md:1061) and `packs verify-data` (phase-16.md:708, 1950-1953) are named. The `--owner` refusal has no code (see V-26-08-a).
- T4 Logic: GAP (see V-26-08-a). PASS for steps 1 to 5 of 3.7 (phase-26.md:490-504), including `E_NOT_ALLOWED_IN_STATE` with sub-code `islamic_versions_live` (phase-26.md:499-500). The logic of the individual documents is covered at V-26-03 and V-26-05.
- T5 Audit: GAP (see V-26-08-b). PASS for `scenario_version.seeded` (phase-16.md:1592).
- T6 Cross-references: GAP (see V-26-08-c and V-26-08-d). PASS for `pack_not_enabled` (phase-06.md:557), `pack_not_agreed` (phase-16.md:1078), `pack.akr_live` (phase-16.md:618), the G9 extension (phase-16.md:1950-1953) and the flag guard that phase 03 registers for this phase (phase-03.md:1302).
- T7 Consistency: PASS - step 3 (phase-26.md:497-498) agrees with the phase 16 extension (phase-16.md:1950-1953). DR-251 (phase-26.md:905) agrees with step 4 on turning the flag off (phase-26.md:499-502). Open doubts 17 and 19 agree with the rows (phase-26.md:466, 470, 937, 939).
- GAP V-26-08-a | LOW | T4 | refusal codes are missing: the `--owner` refusal (phase-26.md:496) has no code, and the go-live refused without `pack.akr_live` is named only in check 6 r (phase-26.md:855) and phase 16 (phase-16.md:1078), not in step 3 (phase-26.md:497-498) | section 3.7 steps 2 and 3: name the code and status | greps: the --owner refusal at phase-26.md:496 names no code; "pack_not_agreed" in phase-26.md (line 855 only)
- GAP V-26-08-b | LOW | T5 | the flag change of step 4 is named only as "the flag change of phase 03" (phase-26.md:598); phase 03 defines the kind as `flag.updated` (phase-03.md:1307) | section 3.12: name `flag.updated` | greps: "flag.updated" in phase-26.md (none)
- GAP V-26-08-c | MEDIUM | T6 | the document file `packs/scenarios/islamic.v1.0.0.json` (phase-26.md:455) is not in the release manifest that phase 16 names (`packs/scenarios/documents.v1.0.0.json`, phase-16.md:691, 1513, 1769). Section 4.5 does not add it, so "signed release data" (phase-26.md:455) is not met | section 3.6 first paragraph and section 4.5 item 7: add the file to the manifest and to the pack seed | greps: "islamic.v1" in phase-16.md and phase-06.md (none); "documents.v1.0.0" in phase-26.md (none)
- GAP V-26-08-d | LOW | T6 | the text says "phase 16 rule 15 applies to `islamic` as to `akr`" (phase-26.md:457). Phase 16 rule 15 covers `pack: akr` only (phase-16.md:969), and the phase 16 extension block does not add it (phase-16.md:1950-1953) | section 4.5 item 7: add rule 15 for `pack: islamic` | greps: "evidence_label" in phase-16.md (line 969 only)

### V-26-09 Data-quality rules for Islamic deals   (source: header I-26; result: GAPS 1)
- T1 SQL: PASS - `admin.dq_issues` has `object`, `field`, `kind`, `severity` and `detail` (phase-12.md:171-200). The three rule fields are real columns: `canon.islamic_deals.asset_ref` and `settlement_currency` (phase-26.md:47, 51) and `canon.deal_parties.role` (phase-26.md:84).
- T2 Route: N/A - ingest rules, no route.
- T3 Repository and types: PASS - `DataQualityRecorder::record_batch` (phase-12.md:731) is the recorder; the item calls no method of its own.
- T4 Logic: PASS - each of the three rules has a test, a field and a severity (phase-26.md:334-338), and the rule failure is a `validation_rule` issue (phase-26.md:330-332).
- T5 Audit: GAP (see V-26-09-a). The rows are written by phase 12's recorder, and the event `dq.recorded` (phase-12.md:974) is not named in 3.2 or 3.12.
- T6 Cross-references: PASS - `dq_issues.kind` includes `validation_rule` and `unmapped_code`, and `severity` includes `medium` and `low` (phase-12.md:171-200).
- T7 Consistency: PASS - DR-254 (phase-26.md:911) and 4.5 item 5 (phase-26.md:759) both say the kind list is unchanged. The gap rates of 3.9 (phase-26.md:537-538) match the three rules (phase-26.md:336-338).
- GAP V-26-09-a | LOW | T5 | the issue rows written by the three rules come from phase 12's recorder, whose event is `dq.recorded` (phase-12.md:974); section 3.2 step 4 and section 3.12 do not name it | section 3.2 step 4 or section 3.12: name `dq.recorded` | greps: "dq.recorded" in phase-26.md (none)

### V-26-10 Home of the keyed-hash purpose   (source: block 7.1; result: GAPS 1)
- T1 SQL: N/A - the block has no SQL. The column it feeds, `kyc.possession_evidence.document_ref_hk`, exists (phase-26.md:165).
- T2 Route: N/A - no route.
- T3 Repository and types: PASS - `Crypto::keyed_hash(purpose, data) -> [u8; 32]` (phase-00.md:588) is the call that 3.5 step 5 needs.
- T4 Logic: GAP (see V-26-10-a). "normalised document_ref" (phase-26.md:396) has no rule; section 2 gives only "trimmed" (phase-26.md:260).
- T5 Audit: N/A - the block has no mutation.
- T6 Cross-references: PASS - phase-00.md:1001-1004 (block 7.15) declares `islamic.document_ref` in the purpose list. Phase 02 gets no purpose entry from this phase (4.5 item 1, phase-26.md:752-753).
- T7 Consistency: PASS - the block says it replaces "(phase 02 `KeyedHash`)" in section 3 (phase-26.md:873-875), and closes Open doubts 15 (phase-26.md:875). The base text at phase-26.md:396 still says KeyedHash, which the block resolves. 4.5 agrees.
- GAP V-26-10-a | MEDIUM | T4 | "normalised document_ref" is not defined. Section 2 says only "trimmed" (phase-26.md:260), and the duplicate match (3.5 step 7) depends on the normalised value, so case, inner spaces and separators are not settled | section 3.5 step 5: state the normalisation as a numbered rule | greps: "normalis" in phase-26.md (line 396 only); research.md (DR-100 names only, a different domain); phase-00.md and phase-10.md (no document-reference rule); phase-02.md:698 normalises evidence bytes only

### V-26-11 Register warning call for the evidence sweep   (source: block 7.2; result: GAPS 3)
- T1 SQL: PASS - `admin.registers` has `kind`, `source`, `source_key`, `title` and `body` (phase-24.md:42-75); `source` includes `scan` (phase-24.md:45) and `body` must be an object (phase-24.md:48).
- T2 Route: N/A - job-only call. The warnings list is phase 24's `GET /api/v1/registers/{kind}` with `dashboard.read` (phase-24.md:555).
- T3 Repository and types: PASS - `RegisterWriter::warn(tx, source, source_key, title_key: WarningTitle, body) -> Option<Uuid>` (phase-24.md:1297-1298; DR-388 at research.md:399).
- T4 Logic: GAP (see V-26-11-a) for the conflict clause. PASS otherwise: one row per bank day through the key (phase-26.md:444-445; 7.2 at phase-26.md:883), and the body keys (phase-26.md:445).
- T5 Audit: GAP (see V-26-11-b).
- T6 Cross-references: GAP (see V-26-11-c). PASS for `WarningTitle` as a closed enum into which later phases add variants (phase-24.md:1300-1301, which names phase 26).
- T7 Consistency: PASS - section 2 (phase-26.md:276-277) and 7.2 (phase-26.md:880-884) give the same signature. The conflict clause differs (see V-26-11-a).
- GAP V-26-11-a | MEDIUM | T4 | the insert names `ON CONFLICT (source_key) DO NOTHING` (phase-26.md:277, 444), but `uq_registers_source_key` is a partial unique index (`WHERE source_key IS NOT NULL`, phase-24.md:75). Phase 24 block 7.7 requires `ON CONFLICT (source_key) WHERE source_key IS NOT NULL DO NOTHING` (phase-24.md:1298-1299). Without the predicate PostgreSQL cannot infer the index, so the sweep's insert fails at run time | section 3.5 step 4 and section 2 ports: add the WHERE predicate | greps: "WHERE source_key" in phase-26.md (none); the predicate form is at phase-24.md:1298-1299 (block 7.7)
- GAP V-26-11-b | LOW | T5 | the register insert's event `register.created` (phase-24.md:1080) is not named in 7.2 or 3.5; section 3.12 says only "the register write of phase 24" (phase-26.md:597) | section 3.12: name `register.created` | greps: "register.created" in phase-26.md (none)
- GAP V-26-11-c | LOW | T6 | "the bilingual title in the message catalogue" (phase-26.md:882). Phase 24 keeps the bilingual title in the closed `WarningTitle` map (phase-24.md:1300), and "message catalogue" is defined only for DSL messages (phase-07.md:433) | block 7.2: name the WarningTitle map instead | greps: "message catalogue" in phase-24.md and phase-00.md (none)

### V-26-12 Party predicate form   (source: block 7.3; result: PASS)
- T1 SQL: N/A - the block has no SQL.
- T2 Route: N/A - no route; the deal read's route is covered by V-26-02.
- T3 Repository and types: PASS - `PartyReadPredicate::allows(principal, party_id, context) -> bool` (phase-18.md:1317-1318). The context is the deal read's context `{ purpose, network }` (phase-26.md:365, 370).
- T4 Logic: PASS - step 7 calls the predicate once per party; a party the caller may not read is omitted and no count is returned (phase-26.md:369-371).
- T5 Audit: N/A - a read; the access event is covered by V-26-02.
- T6 Cross-references: PASS - phase 18 block 7.9 (phase-18.md:1316-1318) and phase 18 section 3.10 (phase-18.md:799) define the rule that 3.4 step 7 relies on.
- T7 Consistency: PASS - section 2 ports (phase-26.md:275), step 7 (phase-26.md:370-371) and 4.5 item 8 (phase-26.md:763) all say the predicate is phase 18's and not phase 17's.

## Summary

- Items checked: 12 (9 header items, 3 gap-pass blocks).
- Items fully PASS: 1 (V-26-12). The other eleven each have at least one GAP line.
- GAP lines by severity: HIGH 2, MEDIUM 15, LOW 17; total 34.
- Items with GAP lines: V-26-01 (2), V-26-02 (4), V-26-03 (2), V-26-04 (6), V-26-05 (4), V-26-06 (3), V-26-07 (4), V-26-08 (4), V-26-09 (1), V-26-10 (1), V-26-11 (3).

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-26-01-a | MEDIUM | T4 | The family value that makes a product Islamic is not named | 3.1 step 2 |
| V-26-01-b | LOW | T6 | uq_code_map_entries_effective does not refuse a second type with a different valid_from | 3.1 step 3(d) |
| V-26-02-a | MEDIUM | T4 | List route has no steps, method name or errors for customer_id or cursor | 3.4 list paragraph |
| V-26-02-b | MEDIUM | T6 | StepUpService::consume gets a bare class; no StepUpAction constant declared | 3.4 step 6 |
| V-26-02-c | LOW | T4 | Display name read from canon.party_name without superseded_at IS NULL | 3.4 step 7 |
| V-26-02-d | LOW | T5 | Deal load audit event (phase 11 batch.recorded) not named | 3.2 step 3, 3.12 |
| V-26-03-a | MEDIUM | T4 | SCN-ISL-01 match conditions not written | 3.6 row SCN-ISL-01 |
| V-26-03-b | MEDIUM | T6 | Agent and related-party relations have no party_relationship.type value | 3.6 row SCN-ISL-01 |
| V-26-04-a | HIGH | T3 | No method computes the duplicate or awaiting-evidence G1 tile counts | section 2, 5.3 |
| V-26-04-b | MEDIUM | T3 | Service methods (attach, verify, sweep) not named with parameters | section 2 |
| V-26-04-c | LOW | T5 | Register insert and stage events not named by kind | 3.12 |
| V-26-04-d | LOW | T6 | EvidenceStore::stage call omits the actor argument | 3.5 step 4 |
| V-26-04-e | LOW | T6 | Pointer to "section 6, check 13" (no such check) | 3.5 closing paragraph |
| V-26-04-f | MEDIUM | T7 | duplicate_exists described as used only for the warning, but used at attach | section 2, 3.5 step 7 |
| V-26-05-a | MEDIUM | T4 | IQD cap has no rule for deals in other currencies | 3.6 row SCN-ISL-06 |
| V-26-05-b | MEDIUM | T4 | related_to_staff NULL handling and its derivation rule not named | 3.6 row SCN-ISL-06 |
| V-26-05-c | MEDIUM | T4 | "Full repayment" and "unpaid" not defined for SCN-ISL-07 | 3.6 row SCN-ISL-07 |
| V-26-05-d | LOW | T7 | facility.contract_type called "not used" in 3.1 but read by SCN-ISL-06 | 3.1 step 2, 3.6 row |
| V-26-06-a | HIGH | T4 | Freeze state follows the first verified task, not all group tasks | 3.8 |
| V-26-06-b | LOW | T3 | AccountClassMap type not defined | section 2 pure rules |
| V-26-06-c | LOW | T6 | Event named ops_task.created; phase 20 defines ops.task_created | 3.12 |
| V-26-07-a | MEDIUM | T3 | AggregateCounts and ShariaAggregateQuery not defined | section 2, 3.9 |
| V-26-07-b | LOW | T3 | Section 2 cites section 3.4 for the aggregate types (defined in 3.9) | section 2, lines 270 and 284 |
| V-26-07-c | MEDIUM | T4 | Malformed period has no refusal or code | 3.9 step 1, 3.11 |
| V-26-07-d | LOW | T6 | Cites section 6 check 17, which does not exist | 3.10 step 1, 3.13 |
| V-26-08-a | LOW | T4 | Refusal codes missing for --owner and for go-live without pack.akr_live | 3.7 steps 2, 3 |
| V-26-08-b | LOW | T5 | Flag change event named only by owner, not by kind (flag.updated) | 3.12 |
| V-26-08-c | MEDIUM | T6 | Document file not in phase 16's release manifest | 3.6, 4.5 item 7 |
| V-26-08-d | LOW | T6 | Rule 15 extension to islamic not in phase 16 (or in 4.5) | 3.6, 4.5 item 7 |
| V-26-09-a | LOW | T5 | dq.recorded (phase 12) not named for the issue rows | 3.2 step 4, 3.12 |
| V-26-10-a | MEDIUM | T4 | "normalised document_ref" not defined | 3.5 step 5 |
| V-26-11-a | MEDIUM | T4 | ON CONFLICT lacks the partial-index predicate; the insert would fail | 3.5 step 4, section 2 |
| V-26-11-b | LOW | T5 | register.created not named | 3.12, 7.2 |
| V-26-11-c | LOW | T6 | "Message catalogue" does not hold register titles; WarningTitle map does | 7.2 |

