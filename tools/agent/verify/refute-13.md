# Refuter, phase 13 (Haiku draft for lead review)

### V-13-01-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: No repository method inserts or updates the `feat.segments` row that the segment activation writes.
- Evidence: docs/plan/phase-13.md:771 "Insert the `segments` row as `active` or update it (version + 1, `change_request_id`)". The five `SegmentRepository` methods at docs/plan/phase-13.md:473-477 (list, get_by_key, active_static, set_membership_batch, membership_at) include no segment write. The feature analogue is docs/plan/phase-13.md:463 "async fn activate_version(&self, tx: &mut Tx, d: &FeatureDefinition, cr: Uuid)". Greps: `grep -n 'async fn' docs/plan/phase-13.md` shows no segment write; `grep -ln SegmentRepository docs/plan/*.md` lists phase 13 only.
- Fix: phase 13, section 2 `SegmentRepository`: add one write method for the activation (insert or update the segment row with version + 1 and change_request_id), called from the segment target's `activate` (docs/plan/phase-04.md:652).

### V-13-01-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Propose step 1 refusals for the key pattern, level `cluster` and a repeated `sort_order` name no problem code; only `E_SEGMENT_MIN_SIZE` is named.
- Evidence: docs/plan/phase-13.md:642-643 "key pattern; level `static` or `risk_overlay`" and "`sort_order` unique among static segments"; line 643 names only "`E_SEGMENT_MIN_SIZE`". The 3.9 row at docs/plan/phase-13.md:799 gives 422 `E_VALIDATION` with "`errors[]` with DSL.md s.7 codes", and docs/API.md:985 says the same. No DSL s.7 code covers the sort-order clash; docs/DSL.md:1096 "`E_SCHEMA` | error | JSON Schema violation" applies only if step 1 uses `segment-definition.v1.json`, which it does not state.
- Fix: phase 13, section 3.3 step 1: name a code for each of the three refusals (E_SCHEMA if the segment schema is applied, otherwise a new phase 13 code listed in the section 3.9 table).

### V-13-02-a  PARTLY   (first agent: HIGH; refuter: HIGH for the standing part)
- Claim: No flag, tag or column marks government payroll or pension pool accounts or a government-source origin.
- Evidence: Refuted part: the account type column exists, "type TEXT NOT NULL" at docs/plan/phase-10.md:636, and the DSL names the value at docs/DSL.md:374 '"field": "account.type", "in": ["government_payroll_pool"]'. Standing part: no government-source flag and no payroll or pension batch tag exists anywhere in the plan; the requirement asks for them at research/27-transaction-monitoring-scenarios.md:1193 "**Payroll and pension batch tags** read from the CBS (employer, batch ID, payment code)". The `type` column has no CHECK or value list (docs/plan/phase-10.md:630-660). Greps: `grep -n -i 'payroll' docs/plan/*.md` (phase-13.md:27 and one phase-15 fixture); `grep -n -i 'employer\|payment_code\|government' docs/plan/phase-1[0-2].md` (none).
- Fix: phase 10, canonical account (or phase 13 section 1 for feat-only tags): add the government-source flag and the payroll and pension batch tags, and state the account type value that marks the payroll pool.

### V-13-02-b  PARTLY   (first agent: HIGH; refuter: MEDIUM for the standing part)
- Claim: No numbered step excludes government payroll pool accounts from feature values or segments, though the header says partly implemented.
- Evidence: Standing part: docs/plan/phase-13.md:27 "partly implemented: S-17 (the exclusion of government payroll pool accounts)", yet no step in 3.1 or 3.2 excludes an account. The phase 12 register has no account unit: docs/plan/phase-12.md:256 "unit IN ('branch', 'product', 'txn_type', 'channel', 'feed')". Refuted part: the exclusion mechanism for scenarios is carried by phase 16, docs/plan/phase-16.md:963 "a static exclusion carries an active `exclusion` reason code", and the S-17 text asks for tags and a flag rather than an exclusion (research/27-transaction-monitoring-scenarios.md:2366).
- Fix: phase 13, header line 27 and section 3.1 default `where`: either drop the exclusion claim for S-17 or add one numbered exclusion step for payroll pool accounts, and say which.

### V-13-02-c  STANDS   (first agent: LOW; refuter: LOW)
- Claim: The header reads S-17 as an exclusion, but the requirement asks for tags and a government-source flag so that public-salary accounts form peer segments.
- Evidence: research/27-transaction-monitoring-scenarios.md:2366 "| S-17 | Payroll and pension batch tags and a government-source flag so public-salary accounts form peer segments |"; docs/plan/phase-13.md:27 "S-17 (the exclusion of government payroll pool accounts)". The DSL A.7 example at docs/DSL.md:374 reads it as an exclusion, so the sources disagree as well. Grep: `grep -n -w 'S-17' docs/plan/*.md` (phase-13.md:27 only).
- Fix: phase 13, header line 27: state which S-17 reading this phase delivers, the tags and flag for peer segments or the exclusion.

### V-13-03-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: Cluster segment rows from the peer build leave `name`, `definition` and `created_by` unset and omit the `currency` the level-cluster CHECK needs, so the insert fails.
- Evidence: docs/plan/phase-13.md:717-718 "one `segments` row per cluster (level `cluster`, `status candidate`, key `peer.<currency lower>.<build_no>.<n>`, `peer_group_id`, `min_size`, `members_count`)". The DDL requires the columns: docs/plan/phase-13.md:113 "name JSONB NOT NULL", 115 "definition JSONB NOT NULL", 125 "created_by UUID NOT NULL", 130 "CHECK (level <> 'cluster' OR currency IS NOT NULL)". Greps: `grep -n "level \`cluster\`" docs/plan/phase-13.md` (718 only); `grep -n created_by docs/plan/phase-13.md` (no peer-build line). Currency and created_by can be read from the peer row, but name and definition cannot.
- Fix: phase 13, section 3.5 step 13: list the name (ar and en), definition, created_by and currency for each cluster row.

### V-13-03-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: No repository method loads the active `risk_overlay` segments or the segments of one peer build, and `SegmentRepository` has `active_static` only.
- Evidence: docs/plan/phase-13.md:475 "async fn active_static(&self, conn: &mut PgConnection)" (static only). The assignment step 1 needs "Load the active segments of the level in `sort_order`" (docs/plan/phase-13.md:654). The statistics step 2 needs "each active cluster segment of an active build" (docs/plan/phase-13.md:674). `PeerRepository::active` returns the build row (docs/plan/phase-13.md:481), not its segments. Grep `grep -n 'active_static\|active(' docs/plan/phase-13.md` shows no level or build loader.
- Fix: phase 13, section 2 `SegmentRepository`: add a load of the active segments by level and a load of the cluster segments of one `build_id`.

### V-13-03-c  PARTLY   (first agent: LOW; refuter: LOW for the standing part)
- Claim: `MembershipChange` and `MembershipSnapshot` are named in the trait but never defined; their fields are only implied by the table columns.
- Evidence: Refuted part (`MembershipChange`): the fields are named in prose at docs/plan/phase-13.md:661-662 "close that row (`valid_to = as_of`) and open a new row (`valid_from = as_of`)" for each customer whose effective segment changes; the rule allows the type without a struct. Standing part (`MembershipSnapshot`): returned by docs/plan/phase-13.md:477 "-> Result<MembershipSnapshot, RepoError>;", and no line names its fields; 3.4 step 2 (docs/plan/phase-13.md:676, "`valid_from <= M < coalesce(valid_to, infinity)`") gives a selection rule, not a field list. Grep: `grep -rln 'MembershipChange\|MembershipSnapshot' docs/plan/` (phase-13.md only).
- Fix: phase 13, section 2 `SegmentRepository`: add one line naming the snapshot's fields (customer, segment, valid_from, valid_to).

### V-13-03-d  PARTLY   (first agent: HIGH; refuter: HIGH for the standing part)
- Claim: The cluster assignment steps are missing: no step loads the centroids and scaling of `build_id`, computes each customer's nearest centroid, writes the membership rows, or defines the "every membership batch is written" test.
- Evidence: Refuted part: the nearest-centroid assignment of the whole population exists inside the peer build, docs/plan/phase-13.md:707 "9. Assign every customer of the population (not only the sample) to the nearest final centroid." Standing part: no step in 3.3 (heading at docs/plan/phase-13.md:653 lists `static | risk_overlay`) loads the build's centroids, writes cluster membership rows, or defines the completion test; docs/plan/phase-13.md:773 "the worker sets the build `active` when every membership batch is written" names the test without defining it. Greps: `grep -n 'build_id' docs/plan/phase-13.md` (773 and 905 only); `grep -n 'membership batch' docs/plan/phase-13.md` (773 only).
- Fix: phase 13, section 3.3: add numbered steps for level `cluster` (load the centroids and scaling of `build_id`, assign, write the membership rows) and define the completion test in 3.7.

### V-13-04-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `cash_share_1m` has no expression: the denominator and the direction are not stated, no `ratio` measures are written, and the shipped catalogue file is not in the plan.
- Evidence: docs/plan/phase-13.md:585 "| `cash_share_1m` | customer | ratio of cash-flagged count | calendar month | per currency | peer builds |". The only full expression in the seed is `credit_turnover_1m` (docs/plan/phase-13.md:524-526). The ratio rule at docs/plan/phase-13.md:548-549 says numerator and denominator come "from measures of the same definition", and no such measures are written. The requirement names both directions: research/21-technology-approaches.md:144 "**cash share** of in and out". Greps: `grep -n 'cash_share' docs/plan/*.md` (585 and 690 only); `grep -n 'cash_share' docs/DSL.md` (none).
- Fix: phase 13, section 3.1 seed table: give the numerator, denominator and direction of `cash_share_1m` as a ratio expression, and the expression of each of the other eight seed features.

### V-13-04-b  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: No repository trait method reads or writes `feat.segment_stats`, although the route and the job's rebuild need one.
- Evidence: docs/plan/phase-13.md:459-490 has five traits (FeatureRepository, FeatureValueWriter, SegmentRepository, PeerRepository, ProfileRepository) and none for statistics. `PgSegmentStatsRepository` appears only in the crate list at docs/plan/phase-13.md:431. The route at docs/plan/phase-13.md:508 returns the statistics, and the job "delete the period's rows ... and insert the new rows" (docs/plan/phase-13.md:679) names no method. Greps: `grep -n 'Stats' docs/plan/phase-13.md` (431, 495, 957 only); `grep -n 'trait ' docs/plan/phase-13.md` (five traits).
- Fix: phase 13, section 2 repository traits: add a statistics repository with the read used by the route and the replace-per-period write used by 3.4 step 5.

### V-13-04-c  PARTLY   (first agent: MEDIUM; refuter: MEDIUM for the standing part)
- Claim: The statistics read has no numbered steps, no error for an unknown segment id, and no source for `suppressed_periods`.
- Evidence: Refuted part (unknown id): docs/plan/phase-13.md:800 "| `NotFound` | 404 `E_NOT_FOUND` | also for an unknown party |" covers any unknown id. Standing part: the read is one sentence, "The API returns them only for the populations of section 3.4 step 3" (docs/plan/phase-13.md:683-684), which leaves the row shape and paging to guess; the response field `suppressed_periods` (docs/plan/phase-13.md:495-496) has no source, because the job "write[s] no row" and sends the count only to the audit: docs/plan/phase-13.md:677 "write no row (the suppressed count goes to the audit)".
- Fix: phase 13, section 3.4: add numbered read steps and say where the response count of suppressed periods comes from (or drop the field from the DTO).

### V-13-04-d  STANDS   (first agent: LOW; refuter: LOW)
- Claim: The route row says suppressed periods are "counted, never shown", but the response DTO and the section 6 test return `suppressed_periods` as an integer.
- Evidence: docs/plan/phase-13.md:508 "suppressed periods counted, never shown (`segment.stats.read`)"; docs/plan/phase-13.md:495-496 "`SegmentStatsView` (rows with `n`, `p50`, `p95`, `p99`, `mad`, and `suppressed_periods`)"; docs/plan/phase-13.md:1003-1004 "(`suppressed_periods` is an integer)". The text can be read as "never shown" meaning the periods, not their count, so this is a wording conflict only.
- Fix: phase 13, route row at docs/plan/phase-13.md:508: say the response carries a count of suppressed periods and no period values.

### V-13-04-e  PARTLY   (first agent: LOW; refuter: LOW for the standing part)
- Claim: The 3.10 heading says reads write access events, but the segment, feature and peer reads name none.
- Evidence: Refuted part: under phase 02 only sensitive routes write access events, and a route is sensitive only when it carries the extension: docs/plan/phase-02.md:735 "a route is sensitive when it carries the OpenAPI extension `x-akr-access: <resource_type>`". The phase 13 extension is on the profile read alone: docs/plan/phase-13.md:924 "`x-akr-access` on the profile read". So the segment reads correctly write no event. Standing part: the heading is wider than the rule, docs/plan/phase-13.md:807 "### 3.10 Audit events (one per mutation; reads write access events)".
- Fix: phase 13, section 3.10 heading: say that sensitive reads write access events, and name the profile read as the only one.

### V-13-05-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: No table or key holds counterparty profiles; `feat.profiles` is keyed by customer.
- Evidence: docs/plan/phase-13.md:208 "customer_id UUID NOT NULL, -- canon.customer logical id (no foreign key)" and 218 "PRIMARY KEY (customer_id, period, currency)". The header claims VB-05 as "implemented here" (docs/plan/phase-13.md:28), and the requirement asks for "Counterparty profiles for non-customers seen only in transactions" (research/12-vendor-ai-native-detection.md:702). Grep: `grep -n -i 'counterpart' docs/plan/phase-13.md` shows no profile table.
- Fix: phase 13, section 1: add a counterparty profile table (keyed by counterparty, period and currency) with its grant, classification and retention.

### V-13-05-b  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: No counterparty feature is seeded and no step groups a feature by counterparty; the month-end SQL groups by customer.
- Evidence: The nine seed rows are customer or account, docs/plan/phase-13.md:581-589 (for example "| `dormant_days` | account |"). The month-end SQL groups by "GROUP BY a.customer_id, t.currency" (docs/plan/phase-13.md:566). `txn.counterparty_id` appears only in the links table, docs/plan/phase-13.md:1067-1091. Grep: `grep -n '| counterparty ' docs/plan/phase-13.md` (none). The DDL and enumerations allow entity `counterparty` (docs/plan/phase-13.md:56, 173), but no grouping for it is defined.
- Fix: phase 13, section 3.1 seed table and section 3.2: name the counterparty grouping (`txn.counterparty_id`) and seed at least one counterparty feature.

### V-13-05-c  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: No steps build counterparty profiles; the refresh and read in 3.6 are customer-only.
- Evidence: docs/plan/phase-13.md:728 "Customers: current `canon.customer` rows." The whole of 3.6 (docs/plan/phase-13.md:724-765) addresses customers and records of customers only. Grep: `grep -n -i 'counterpart' docs/plan/phase-13.md` returns no line in 3.6.
- Fix: phase 13, section 3.6: add the refresh and read steps for entity `counterparty`, or name a separate job for them.

### V-13-05-d  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: No repository trait or DTO exists for counterparty profiles; `ProfileRepository` takes customer ids and `Profile` rows.
- Evidence: docs/plan/phase-13.md:487 "async fn replace_period(&self, tx: &mut Tx, period: NaiveDate, rows: &[Profile])" and 488-489 "async fn for_party(&self, conn: &mut PgConnection, customer_ids: &[Uuid]". The view is customer-only: docs/plan/phase-13.md:498 "`PartyProfilesView` (per customer record: periods with `actual`, `declared` and `status`)".
- Fix: phase 13, section 2: add a counterparty profile repository trait and its view DTO.

### V-13-05-e  STANDS   (first agent: MEDIUM; refuter: LOW)
- Claim: No audit event names a counterparty profile write; `profile.refreshed` is customer-scoped.
- Evidence: docs/plan/phase-13.md:749 "audit `profile.refreshed {period, rows, complete, degraded, excluded_customers}`" and the 3.10 row at docs/plan/phase-13.md:821 "| `profile.refreshed` | end of a `profile_refresh` run | `profile` (period) |". The resource is named `profile`, so the event could cover counterparty profiles once they exist, but its `excluded_customers` field is customer-only. The gap is conditional on V-13-05-a: no counterparty profile write exists yet.
- Fix: phase 13, section 3.10: widen `profile.refreshed` to counterparty profiles when V-13-05-a is fixed, and give its excluded field a counterparty form.

### V-13-06-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The protected set is defined twice with different content: phase 13 hard-codes six attributes, and the phase 24 register has five other codes.
- Evidence: docs/plan/phase-13.md:535-536 "gender, religion, ethnicity, nationality, tribe and sect (DSL.md A.5". The same six list is docs/DSL.md:278 "Protected attributes (gender, religion, ethnicity, nationality, tribe, sect)". The register has codes nationality, residence_country, governorate, name_origin and sect_tribe_ethnicity, with no gender or religion, and nationality may be used for "'{fairness_metric,segmentation,reporting}'" (docs/plan/phase-24.md:441). A third list is in the requirement: research/30-security-privacy.md:1419 "a blocked-attribute list (religion, sect, ethnicity, blood type, biometrics, ID images)". Greps: `grep -n 'admin.protected_attributes' docs/plan/phase-13.md` (none).
- Fix: phase 13, section 3.1 step 2: name the governing list (the DSL A.5 list or the phase 24 register), and say whether the register's segmentation use covers detection segments.

### V-13-07-a  PARTLY   (first agent: MEDIUM; refuter: LOW for the standing part)
- Claim: For a customer with short history, step 5 says "a short-history fallback" but no step says what `deviation` and `expected` hold.
- Evidence: Refuted part: the designed behaviour is in the DSL, docs/DSL.md:366 "| `min_history` | Below it, `own_history` comparators are skipped and the peer fallback applies |". The peer comparator is computed for every customer in docs/plan/phase-13.md:740-744. Standing part: docs/plan/phase-13.md:738-739 "Otherwise a short-history fallback." is not restated, and docs/plan/phase-13.md:746-747 says the `expected` column holds "the own-history summary", which has no value in this case.
- Fix: phase 13, section 3.6 step 5: add one sentence saying that with short history the own-history summary and own z are absent and the peer comparator applies.

### V-13-07-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The read's range refusal (above twenty-four months, or `from` after `to`) has no problem code.
- Evidence: docs/plan/phase-13.md:758 "Range: `from` and `to` are periods; the default is the last twelve months and the maximum is twenty-four." No refusal is stated. The 3.9 row at docs/plan/phase-13.md:799 gives 422 `E_VALIDATION` with "`errors[]` with DSL.md s.7 codes", and docs/plan/phase-00.md:499 gives the same status. No DSL s.7 code covers a range, so the item-level code is missing even though the status can be inferred.
- Fix: phase 13, section 3.6 read step 4: name the refusal for a range above 24 months and for `from` after `to`, with the errors[] code.

### V-13-07-c  REFUTED   (first agent: LOW; refuter: none)
- Claim: The Open doubts say T-08 to T-10 are "research IDs not carried here", while the header carries T-08 and T-09 and 7.6 closes them.
- Evidence: docs/plan/phase-13.md:1172 "T-07 is now carried (lead, review); T-08 and T-09 are decided in section 7+." The same bullet, lines 1169-1172, reconciles the statement the claim quotes, so the text is not in conflict. Section 7.6 (docs/plan/phase-13.md:1137-1140) closes T-08 and T-09 as the claim says.

### V-13-08-a  REFUTED   (first agent: HIGH; refuter: none)
- Claim: No table holds a per-segment baseline keyed by closed business day; the stores are keyed by month.
- Evidence: docs/plan/phase-13.md:1180-1181 "Features are month-end snapshots. ... this phase stores month-ends only." A daily table is therefore a stated non-goal, and the rule that a stated non-goal is not a gap applies. The requirement does not ask for a daily per-segment store either: research/12-vendor-ai-native-detection.md:711 "Per-customer behavioural baseline store usable offline (Phase 1) and online within a latency budget (Phase 2 scoring API)". The header wording that conflicts with this is covered by V-13-08-d.

### V-13-08-b  PARTLY   (first agent: HIGH; refuter: LOW for the standing part)
- Claim: No numbered step computes a per-segment robust baseline on closed days; section 3.4 is monthly and the Cadence doubt defers daily features to phase 16, which does not carry them.
- Evidence: Refuted part: phase 16 does evaluate daily event-time measures inline from `canon`: docs/plan/phase-16.md:995-996 "is compiled by `FeatureCompiler::compile_measure` (the measure and filter compiler of phase 13 section 3.1". Standing part: no step in either phase computes a per-segment baseline on closed days. Phase 16 uses only the monthly segment statistic: docs/plan/phase-16.md:1016 "`peer` through the segment statistic of the period before". The mismatch is with the header claim (docs/plan/phase-13.md:27), which V-13-08-d covers.
- Fix: phase 13, header line 27: restate VB-14 as the monthly and per-customer scope this phase delivers (see V-13-08-d); no new steps are needed unless the bank asks for a daily per-segment baseline.

### V-13-08-c  STANDS   (first agent: LOW; refuter: LOW)
- Claim: The header paraphrase of VB-14 ("the robust per-segment baseline on closed days") differs from the requirement, and its online part is neither delivered nor deferred in the text.
- Evidence: research/12-vendor-ai-native-detection.md:711 "Per-customer behavioural baseline store usable offline (Phase 1) and online within a latency budget (Phase 2 scoring API)"; docs/plan/phase-13.md:27 "VB-14 (the robust per-segment baseline on closed days)". Grep: `grep -n -i 'latency' docs/plan/phase-13.md` (none). The requirement itself places the online half in its own Phase 2, so the plan could name that deferral.
- Fix: phase 13, header line 27: restate VB-14 as the per-customer offline store, and say where the online half is deferred.

### V-13-08-d  STANDS   (first agent: LOW; refuter: LOW)
- Claim: The header wording "on closed days" contradicts the month-end design of sections 3.4 and 3.6 and the Cadence doubt.
- Evidence: docs/plan/phase-13.md:27 "robust per-segment baseline on closed days"; docs/plan/phase-13.md:672 "Require the month-end `M` of the period to be `computed`"; docs/plan/phase-13.md:1180 "Features are month-end snapshots." The wording is plural and suggests daily granularity; the design is monthly.
- Fix: phase 13, header line 27: replace "on closed days" with month-end wording.

### V-13-09-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The profile read's single access event does not say which `resource_id` it carries (the party, the first record, or one event per record).
- Evidence: docs/plan/phase-13.md:756-757 "One access event per response, tier `restricted`, `subject_party` = the party". The read covers several records: docs/plan/phase-13.md:752 "Resolve the party to its current customer records". phase 02 requires the column: docs/plan/phase-02.md:149 "resource_id UUID NOT NULL". The phase 02 list rule writes one event per row, docs/plan/phase-02.md:732 "Lists: `scope.record_hits(rows)` writes one `search_hit` per restricted or suspicion-tier row shown", and the text does not say why it does not apply. Grep: `grep -n 'resource_id' docs/plan/phase-13.md` (none).
- Fix: phase 13, section 3.6 read step 3: name the `resource_id` of the one event and say why no per-record event is written.

### V-13-09-b  STANDS   (first agent: LOW; refuter: LOW)
- Claim: 7.1 voids "section 4.5's" access-layer bullet, but section 4 has no numbered subsections; the bullet is the "Phase 02" line of the existing-updates list.
- Evidence: docs/plan/phase-13.md:1034 'section 4.5's "the access layer gains the `profile.read` action" is void'. Section 4 (docs/plan/phase-13.md:897 "## 4. Infrastructure Updates") has no numbered subsections (grep `^### 4` finds none). The bullet is docs/plan/phase-13.md:937 "Phase 02: `PartitionRegistry` gains three feat entries; the access layer gains the `profile.read` action."
- Fix: phase 13, 7.1 wording (docs/plan/phase-13.md:1034): refer to the "Phase 02" bullet of the existing-updates list, not to "section 4.5".

### V-13-10-a  STANDS   (first agent: HIGH; refuter: MEDIUM)
- Claim: No signature exists for `compile_measure`, `compile_filter`, `ProfileComparator::plan` or `ProfileComparator::evaluate`, and phase 13 never defines the `ProfileComparator` type.
- Evidence: docs/plan/phase-13.md:429 "profiles (ProfileComparator)" is the only definition-level mention; the only compiler signature is docs/plan/phase-13.md:555 "`FeatureCompiler::compile(def, as_of)` is pure and returns `CompiledFeature { sql, binds }`". Section 7.2 names the four methods without signatures, docs/plan/phase-13.md:1040-1041. Phase 16 gives only partial forms: docs/plan/phase-16.md:1014 "The comparator plan is `ProfileComparator::plan(expected, combine," and docs/plan/phase-16.md:1162 "`ProfileComparator::evaluate` with `combine`". The behaviour is stated in prose (docs/plan/phase-13.md:533-553 and 737-744), so the severity is MEDIUM, not HIGH: a developer must add the signatures and the type's shape, but not the logic.
- Fix: phase 13, section 2 domain list or section 7.2: give the four signatures with return and error types, and define the `ProfileComparator` type in one place.

### V-13-10-b  PARTLY   (first agent: MEDIUM; refuter: LOW for the standing part)
- Claim: The owner of the four extension methods is stated two ways: phase 13 says phase 16 owns them, phase 16 says it consumes them "not defined here" and also adds them to phase 13's module.
- Evidence: Refuted part (owner decided): docs/plan/phase-13.md:1040 "Phase 16 extends this phase and owns the extension"; docs/plan/phase-16.md:2116 "additions to phase 13's module made by this phase, not redefinitions". Standing part: docs/plan/phase-16.md:864 "**Consumed ports** (not defined here): phase 13 `FeatureCompiler` (measure and filter compiler), `ProfileComparator`," is a stale wording of the same methods.
- Fix: phase 16, consumed-ports list (line 864): mark the four methods as extended by phase 16 in section 3.2, as section 7.2 of phase 13 and the phase 16 gap block already say.

### V-13-11-a  STANDS   (first agent: LOW; refuter: MEDIUM)
- Claim: The `ReaderCatalogue` port is written two ways: section 3.8 has no connection and no error type, while phase 16 implements a trait with `conn` and `Result<_, CatalogueError>`.
- Evidence: docs/plan/phase-13.md:790 "`ReaderCatalogue::readers(object) -> Vec<ReaderRef>` and `measure(object, proposal) -> Option<VolumeEffect>`". The trait phase 16 implements: docs/plan/phase-16.md:843-846 "pub trait ReaderCatalogue" ... "async fn readers(&self, conn: &mut PgConnection, object: &ObjectRef) -> Result<Vec<ReaderRef>, CatalogueError>;" ... "-> Result<Option<VolumeEffect>, CatalogueError>;". Phase 16 replaces the phase 13 null port (docs/plan/phase-16.md:1785), so the null port now needs the phase 16 signature. The return type differs (Option against Result of Option), which is more than a name difference.
- Fix: phase 13, section 3.8: give the port with the connection and the error type that phase 16 implements (docs/plan/phase-16.md:843-846).

### V-13-11-b  PARTLY   (first agent: MEDIUM; refuter: LOW for the standing part)
- Claim: Section 7.3 has no rule for `measure` returning `None` at decision, so a literal comparison would refuse every decision before phase 16.
- Evidence: Refuted part: the general volume-effect rule of section 3.7 already covers the no-reader case and the unmeasurable case: docs/plan/phase-13.md:780-782 "Volume effect: `neutral` when `ReaderCatalogue` reports no reader for the object, else the measured effect of `ReaderCatalogue::measure`, and `unknown` (not approvable) when it cannot be measured". Standing part: section 7.3 calls the port directly and does not say that the re-measure uses this rule: docs/plan/phase-13.md:1045 "`check_before_decision` (section 3.7) calls `ReaderCatalogue::measure` again on the proposed change."
- Fix: phase 13, section 7.3 (line 1045): add one sentence saying the re-measure uses the section 3.7 volume-effect rule.

### V-13-11-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The null case of `measure` contradicts across phases: phase 13 says it returns `None`, phase 16 calls `Some(Neutral)` for no readers "phase 13's null rule".
- Evidence: docs/plan/phase-13.md:790-791 "Null implementation: no readers (so the volume effect is `neutral`) and `measure` returns `None`." docs/plan/phase-16.md:1445 "1. No readers: `Some(Neutral)` (phase 13's null rule, now true because no scenario reads the object)." The two texts cannot both hold. The effect is limited to the null port, which phase 16 replaces (docs/plan/phase-16.md:1785), and to section 7.3 (V-13-11-b).
- Fix: phase 13, section 3.8 (lines 790-791) or phase 16, section 3.14 (line 1445): make the two null returns agree.

### V-13-12-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The retention row for `feat.counterparty_links` uses clock `age` on `created_at`, while the block and its note say the row follows the counterparty.
- Evidence: docs/plan/phase-13.md:1096 "'R2', 'age', 'created_at', 'Counterparty links: kept as long as the counterparty'"; the block says "retention R2 (follows canon.counterparty)" at docs/plan/phase-13.md:1064. The retention rule for a row that follows its owner is the owner clock: docs/plan/phase-23.md:576 "`owner`: no clock of its own; the row is deleted in the same transaction as its owner". The phase 10 counterparty uses `age` on `last_seen` (docs/plan/phase-10.md:1186), so the link row should not copy it.
- Fix: phase 13, section 7.5 retention row (line 1096): use the owner clock as phase 23 defines it, or change the note to match the age rule.

### V-13-12-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `feat.counterparty_links` has no owning service or repository method, and phase 18 writes it directly.
- Evidence: docs/plan/phase-13.md:1059-1060 "The resolver's links live in a table of their own (extends DATA-MODEL s.8), written by akr-worker only." The phase 18 decide route is an API route that writes the table: docs/plan/phase-18.md:1359 "| POST | `/api/v1/cases/{id}/counterparty-links` |" and docs/plan/phase-18.md:1365 "the counterparty's open `feat.counterparty_links` row (`valid_to = now()`)". Section 2 has no link method (docs/plan/phase-13.md:459-490). The owning-service convention itself is not in this repository (ddd.md is not present), so the verdict rests on the phase 13 text, which contradicts phase 18.
- Fix: phase 13, section 2: add a counterparty-link repository method for the analyst decision, and correct line 1060 so that it names the API route as a writer, or move the decision to the worker.

### V-13-12-c  STANDS   (first agent: LOW; refuter: LOW)
- Claim: The key function of step 4 is not named, so a developer cannot call it by name.
- Evidence: docs/plan/phase-13.md:1122 "the `crates/names` function behind `canon.party_name.key`". docs/plan/phase-10.md:2379-2380 names the crate and "the functions behind `FoldArabic` and `SplitArabicName`" and says the `key` column comes from the crate through the transforms. No function is named for `key`.
- Fix: phase 13, section 7.5 step 4 (line 1122): name the crates/names function that produces the key.

### V-13-12-d  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The job and the phase 18 analyst route both close the open row and insert a new one; the job takes no lock and does not re-read the open row in its chunk transaction, and the block does not say what a unique-index refusal does.
- Evidence: docs/plan/phase-13.md:1126 "close the replaced open row (`valid_to = now()`) and insert the new row"; docs/plan/phase-13.md:1079 "CREATE UNIQUE INDEX IF NOT EXISTS uq_counterparty_link_open ON feat.counterparty_links (counterparty_id)"; docs/plan/phase-18.md:1365 closes the same row inside the case lock. The job rule "never replaced by the job" (docs/plan/phase-13.md:1115-1116) is decided in step 1, before the chunk transaction. The harness retries failed jobs (docs/plan/phase-00.md:107 "max_attempts  INT NOT NULL DEFAULT 5"), but that does not settle whether the close is by the row read in step 1, so a silent overwrite of an analyst row is possible if the close is keyed by counterparty.
- Fix: phase 13, section 7.5 step 6 (lines 1126-1127): close the row read in step 1 by its id after a re-read in the chunk transaction, and say what a unique-index refusal does.

## Summary

- REFUTED: 2 (V-13-07-c, V-13-08-a)
- STANDS: 26
- PARTLY: 10

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-13-01-a | STANDS | HIGH | No write method for the segment activation row | phase 13 section 2 `SegmentRepository` |
| V-13-01-b | STANDS | MEDIUM | Propose refusals for key pattern, cluster level and sort-order clash name no code | phase 13 section 3.3 step 1 |
| V-13-02-a | PARTLY | HIGH | No government-source flag, no payroll or pension batch tags, no value set for the account type | phase 10 canonical account, or phase 13 section 1 |
| V-13-02-b | PARTLY | MEDIUM | Header claims an exclusion of payroll pool accounts for features and segments; no step does it | phase 13 header line 27 and section 3.1 default `where` |
| V-13-02-c | STANDS | LOW | Header reads S-17 as an exclusion; the requirement asks for tags and a flag | phase 13 header line 27 |
| V-13-03-a | STANDS | MEDIUM | Cluster segment rows omit name, definition, created_by and currency | phase 13 section 3.5 step 13 |
| V-13-03-b | STANDS | MEDIUM | No loader for active risk_overlay segments or for the segments of one build | phase 13 section 2 `SegmentRepository` |
| V-13-03-c | PARTLY | LOW | `MembershipSnapshot` fields are not named (`MembershipChange` is named in 3.3 step 4) | phase 13 section 2 `SegmentRepository` |
| V-13-03-d | PARTLY | HIGH | Cluster membership steps missing: centroid load, membership writes, completion test | phase 13 section 3.3 (new cluster steps) and 3.7 |
| V-13-04-a | STANDS | MEDIUM | `cash_share_1m` has no numerator, denominator or direction; eight seed features have no expression | phase 13 section 3.1 seed table |
| V-13-04-b | STANDS | HIGH | No repository method for `feat.segment_stats` | phase 13 section 2 repository traits |
| V-13-04-c | PARTLY | MEDIUM | Statistics read is one sentence with no steps; `suppressed_periods` has no source | phase 13 section 3.4 |
| V-13-04-d | STANDS | LOW | Route row says "never shown"; DTO returns a count | phase 13 route row (line 508) |
| V-13-04-e | PARTLY | LOW | 3.10 heading says reads write access events, wider than the phase 02 rule | phase 13 section 3.10 heading |
| V-13-05-a | STANDS | HIGH | No counterparty profile table | phase 13 section 1 |
| V-13-05-b | STANDS | HIGH | No counterparty feature seeded; no counterparty grouping in any step | phase 13 section 3.1 seed table and section 3.2 |
| V-13-05-c | STANDS | HIGH | No counterparty profile refresh or read steps | phase 13 section 3.6 |
| V-13-05-d | STANDS | HIGH | No counterparty profile repository trait or DTO | phase 13 section 2 |
| V-13-05-e | STANDS | LOW | `profile.refreshed` is customer-scoped (conditional on V-13-05-a) | phase 13 section 3.10 |
| V-13-06-a | STANDS | MEDIUM | Two protected-attribute lists; no rule says which governs | phase 13 section 3.1 step 2 |
| V-13-07-a | PARTLY | LOW | Short-history case: own-history summary and own z value not stated | phase 13 section 3.6 step 5 |
| V-13-07-b | STANDS | MEDIUM | Range refusal has no errors[] code | phase 13 section 3.6 read step 4 |
| V-13-08-b | PARTLY | LOW | Header claims a per-segment baseline on closed days with no step; the event-time part is carried by phase 16 | phase 13 header line 27 |
| V-13-08-c | STANDS | LOW | VB-14 paraphrase is per-segment; the requirement is per-customer; online half not deferred in the text | phase 13 header line 27 |
| V-13-08-d | STANDS | LOW | "on closed days" wording against the month-end design | phase 13 header line 27 |
| V-13-09-a | STANDS | MEDIUM | Profile read's single access event has no `resource_id` rule | phase 13 section 3.6 read step 3 |
| V-13-09-b | STANDS | LOW | 7.1 cites "section 4.5", which does not exist | phase 13 section 7.1 wording (line 1034) |
| V-13-10-a | STANDS | MEDIUM | Four extension methods have no signatures; `ProfileComparator` type undefined | phase 13 section 2 domain list or 7.2 |
| V-13-10-b | PARTLY | LOW | Phase 16 line 864 says "not defined here", against 7.2 and its own gap block | phase 16 consumed-ports list (line 864) |
| V-13-11-a | STANDS | MEDIUM | `ReaderCatalogue` signature differs (no connection, plain Option instead of Result) | phase 13 section 3.8 |
| V-13-11-b | PARTLY | LOW | Section 7.3 calls `measure` directly without saying the 3.7 volume-effect rule applies | phase 13 section 7.3 |
| V-13-11-c | STANDS | MEDIUM | Null `measure` returns `None` (phase 13) against "phase 13's null rule" as `Some(Neutral)` (phase 16) | phase 13 section 3.8 or phase 16 section 3.14 |
| V-13-12-a | STANDS | MEDIUM | Link retention uses `age` on `created_at`; the note says it follows the counterparty | phase 13 section 7.5 retention row (line 1096) |
| V-13-12-b | STANDS | MEDIUM | Phase 18 API route writes the link table; phase 13 says worker only; no method named | phase 13 section 2 and section 7.5 (line 1060) |
| V-13-12-c | STANDS | LOW | Key function of step 4 not named | phase 13 section 7.5 step 4 |
| V-13-12-d | STANDS | MEDIUM | Job and analyst route race on the open row; no re-read or unique-violation rule | phase 13 section 7.5 step 6 |
