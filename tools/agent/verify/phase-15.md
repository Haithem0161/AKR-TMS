# Verification pass, phase 15 (Haiku draft for lead review)

Scope: docs/plan/phase-15.md (2255 lines). Read in full: header, sections 1 to 6, 7.x blocks, Decisions, Open doubts, Does NOT Touch. Outside reads (grep or cited lines only): docs/plan/phase-00, phase-04, phase-09, phase-10, phase-11, phase-14, phase-17, phase-18, phase-19, phase-20, phase-22; docs/API.md (sections 2 and 6); docs/DSL.md (lines 777-813, 838, 1052, 1457, 1469); research/26 lines 293 and 305 (TR-14 and TR-24, cited by 7.7); docs/plan/research.md (grep only). Not read: cbs-discovery/, test-poc/, research/01-33 apart from those two lines.

## Items picked (12)

Five header items, taken from the start, middle and end of the "Requirements carried" list (bullet order in the header):
1. K-13 exit register (bullet 1, start of the list) -> V-15-01
2. TR-10 country-level change fan-out (bullet 2, TR-01 to TR-26) -> V-15-02
3. T-32 overrides (bullet 6, "implemented here") -> V-15-03
4. VC-47 PEP register with tiers and relatives (bullet 7, "partly implemented", middle) -> V-15-04
5. VC-79 review queue with calendar and event due bases (bullet 7, last carried item) -> V-15-05

Four gap-pass items, one each from four different 7.x blocks not headed "Adversarial pass" (7.1, 7.3, 7.6 and 7.8 are not picked; 7.8 has no deliverable of its own):
6. 7.2 exit_decision and create_ops_task (phase-15.md:2031) -> V-15-06
7. 7.4 ownership coverage view (phase-15.md:2049) -> V-15-07
8. 7.5 override analytics (phase-15.md:2062) -> V-15-08
9. 7.7 trigger evaluation and account events (phase-15.md:2086) -> V-15-09

Three adversarial items, one from each run block (grep finds run headers at phase-15.md:2138 run 1, 2145 run 4 and 2155 run 6; the blocks under them are 7.9, 7.10 and 7.11; runs 2, 3 and 5 have no block in this phase):
10. 7.9 approver of a downward override (run 1) -> V-15-10
11. 7.10 absorbed event review keeps the earlier deadline (run 4) -> V-15-11
12. 7.11 trigger job applies once (run 6) -> V-15-12

### V-15-01 K-13 exit register (`POST` and `GET /exit-register`, 3.4 step 8)   (source: header Requirements carried, bullet 1; result: GAPS 4)
- T1 SQL: PASS - `CREATE TABLE IF NOT EXISTS kyc.exit_register` (phase-15.md:480-492) has `review_id` NOT NULL (483), `workflow_instance_id` FK to `admin.workflow_instances` (484), `reason_code` CHECK `^DEC_` (485); grants at 733-734; retention R5 at 866.
- T2 Route: PASS - `GET /api/v1/exit-register` (kyc.work, `exit.read`, phase-15.md:1082) and `POST /api/v1/exit-register` (kyc.work, `exit.record`, 1083); Cedar permit `kyc.exit.m` admits M only (1673-1675).
- T3 Repository and types: GAP - see V-15-01-a and V-15-01-b. Request type `NewExitEntry` (940-941) and response `ExitEntryView` (1049) are named.
- T4 Logic: GAP - see V-15-01-c. Step 8 (1351-1356) is numbered: a 409 for an instance not completed as approve and for a second entry; the reason-code refusal is missing.
- T5 Audit: PASS - "audit `exit.recorded {exit_id, review_id, reason_code}`" (phase-15.md:1355-1356), also in 3.11 at 1628.
- T6 Cross-references: GAP - see V-15-01-d (the terminal effect `record_decision`, 1351 and 1798). PASS: `DEC_EXIT_FINANCIAL_CRIME` is in family `decision` (phase-09.md:1294); `admin.workflow_instances` exists (phase-04.md:328).
- T7 Consistency: PASS - 7.2 (phase-15.md:2031-2036) changes the workflow's effect list only; 3.4 step 8 and 4.2 (1798-1799) agree that M records the entry.

GAP V-15-01-a | HIGH | T3 | No repository trait for `kyc.exit_register` (insert, get, list) and no declared service or query type for the route. `ExitService` is named only in table 1.5 (phase-15.md:884); `exit: Arc<ExitQuery>` (1817) has no definition | section 2 (repository traits, after line 1014) and section 4.3 (AppState, 1815-1820) | greps: `grep -n "ExitRepository\|ExitService\|ExitQuery"` in phase-15.md finds only 884 and 1817; research.md finds nothing.

GAP V-15-01-b | MEDIUM | T3 | `NewExitEntry { party_id, workflow_instance_id, reason_code }` (940-941) has no `review_id`, but `kyc.exit_register.review_id` is NOT NULL (483) and the audit at 1356 uses it. The text never says where the value comes from (the completed review that opened the instance, or the instance context) | section 2 (`NewExitEntry`) and 3.4 step 8 | greps: `grep -n "review_id"` in phase-15.md shows no source for an exit entry; `grep -n "NewExitEntry"` finds 940 and 1049 only; research.md finds nothing.

GAP V-15-01-c | MEDIUM | T4 | Step 8 requires the reason code to be "an active code of family `decision`" (1354) but gives no refusal code or status for a code that is inactive or of another family | section 3.4 step 8 (1353-1355) and the 3.10 table (1585-1603) | greps: `grep -n -i "active code"` finds 1354 only; 3.10 has no row for it; research.md finds nothing.

GAP V-15-01-d | HIGH | T6 | The terminal effect `record_decision` that `exit_decision` (1351, 1798), `kyc_override_down` (1389, 1794) and `kyc_edd_signoff` (1347, 1797) end on is registered by phase 18 for resource kind `case` only ("register with resource kind `case`", phase-18.md:1130). Phase 04 says it registers later (phase-04.md:690) and that a definition naming an unregistered effect cannot be published (691). Phase 15 registers no effect | section 4.2 (shipped definitions) with a registration for the kyc resource kinds, or a named change to the phase 18 registration | greps: `grep -n "Effect" phase-15.md` finds no effect registration; phase-18.md:1130 registers it for the case kind only; research.md: DR-134 ties record_decision to phase 18 only.

### V-15-02 TR-10 country-level change fan-out (`rating_country_fanout`)   (source: header Requirements carried, bullet 2 (TR-01 to TR-26); result: GAPS 4)
- T1 SQL: GAP - see V-15-02-a. PASS for the other predicates: `canon.party.nationality` (phase-10.md:455) and `residence_country` (456); the owner's residence reads the owner party through `canon.party_relationship` (phase-10.md:570, `type` includes `owner`).
- T2 Route: N/A - a job with no route (phase-15.md:1272 and the job row at 1766).
- T3 Repository and types: GAP - see V-15-02-b. The job payload `{iso2, cause, effective_at}` is named (1766); the consumed topic payload is named (1272).
- T4 Logic: PASS - one job, coalesce key `rating_country:<iso2>`, concurrency 1 (1766); page size `rating.country_fanout_page` (settings, 765-766); each page enqueues `rating_compute {reason: "country"}` (1272). No refusal path; no second writer, because the job writes nothing.
- T5 Audit: PASS - "Each enqueues work and writes nothing itself" (phase-15.md:1267); the re-score's `rating.changed` and `rating.control_failure` are in 3.2 step 15 (1254-1261) and 3.11 (1615-1616).
- T6 Cross-references: GAP - see V-15-02-c and V-15-02-d. PASS: `ref.country_level_changed` is a phase 09 outbox topic (phase-09.md:1035, "no consumer until phase 15"); `ref.countries` exists (phase-09.md:37).
- T7 Consistency: PASS - no 7.x block changes TR-10 (`grep -n "country_level_changed"` in phase-15.md finds only 1272, 1766, 1789 and 1858); 3.3 and 4.1 agree on the job.

GAP V-15-02-a | HIGH | T1 | The fan-out selects parties by "registration country" (1272), but `canon.party` has no registration-country column. Its columns are `nationality`, `residence_country`, `registration_no_enc` and `registration_no_hk` (phase-10.md:451-466). Open doubts 10 (phase-15.md:2222-2223) defers "C2 (registration country)" until phase 10 maps it, and phase 10 does not | section 3.3 row 2 (1272), with a column in section 1 or a stated phase 10 change | greps: `grep -n -i "registration"` in phase-10.md finds 464, 465, 802 and 2179 only; `grep -n -i "country_of\|reg_country\|registered_in\|incorporat"` finds nothing; research.md: no "registration country".

GAP V-15-02-b | MEDIUM | T3 | No repository method or SQL is named for the paged party query: parties whose nationality, residence, registration country or owner's residence is `iso2`, keyset by party id, page of `rating.country_fanout_page` | section 2 (repository traits, 944-1014; none takes a country) | greps: `grep -n -i "country"` in section 2 lines 944-1014 finds no method; research.md finds nothing.

GAP V-15-02-c | MEDIUM | T6 | The job payload needs `effective_at` (1272, 1766), but the consumed event `ref.country_level_changed {iso2, from, to, cause}` has no date field, and the phase does not say where the date comes from | section 3.3 row 2 and section 4.2 "Outbox topics consumed" (1789-1790) | greps: `grep -n "effective_at"` in phase-09.md finds 444, 445 and 830 only (proposals), not the event at 1035; research.md: DR-58 only.

GAP V-15-02-d | LOW | T6 | DSL.md:1457 (and the list at DSL.md:807) names `country_table.changed` as TR-10's event, while phase 15 consumes phase 09's topic `ref.country_level_changed` and never names `country_table.changed` | section 3.3 row 2 and section 4.2 (state the name mapping) | greps: `grep -n "country_table.changed"` in phase-15.md finds nothing (only the `country_table.*` factor anchors at 1127 and 1216).

### V-15-03 T-32 overrides: reason code, expiry, second approver for downward, analytics   (source: header Requirements carried, "Gap pass 1 (coverage), implemented here", T-32; result: GAPS 1)
- T1 SQL: PASS - `CREATE TABLE IF NOT EXISTS kyc.overrides` (phase-15.md:108-136): `reason_code` CHECK `^OVR_` (115), `expires_at` (124), `approved_by` (122), CHECK "approved_by <> proposed_by" (131); partial unique index `uq_overrides_open` (138); grant at 709. `admin.change_requests` exists (phase-04.md:121); family `override` is in phase-09.md:1295.
- T2 Route: PASS - `POST /api/v1/parties/{id}/overrides` (kyc.work, `override.propose`, phase-15.md:1061) and `GET /api/v1/overrides` (kyc.work, `override.read`, 1062); both are in the Cedar permit at 1659.
- T3 Repository and types: GAP - see V-15-03-a. `OverrideRepository` (968-977), `NewOverride` (932-934) and `OverrideView` (1044) are named.
- T4 Logic: PASS - upward steps 1-4 (1365-1374) and downward steps 1-7 (1376-1395) are numbered, each with its refusal: 422 `E_VALIDATION`, 409 `E_NOT_ALLOWED_IN_STATE`, 403 `E_FORBIDDEN` or masked 404, 422 `E_BELOW_FLOOR`, 202 with `Location`. Two writers meet at `uq_overrides_open` (138), and approval re-runs the floor check (1394-1395).
- T5 Audit: PASS - `override.proposed` (1387), `override.applied` upward (1373) and downward (1391-1392), `override.ended` (1399); all in 3.11 (1617-1619).
- T6 Cross-references: GAP (shared) - the downward route ends on `record_decision` (1389, 1794), the unregistered effect of V-15-01-d; not counted again here. PASS: `E_BELOW_FLOOR` (docs/API.md:989) and `E_SELF_APPROVAL` (docs/API.md:971) are section 6 codes; `ChangeRequestService` exists (phase-04.md:854-857).
- T7 Consistency: PASS with a note - 7.9 (phase-15.md:2140-2143) replaces the approver of 3.5 step 5 (1388-1389) and of 4.2 (1794-1795) with one step, role CHK, and an escalation to M. The block says "is shipped as", so it wins. The Cedar permit still admits M (see V-15-10-a). 7.5 adds a route only.

GAP V-15-03-a | MEDIUM | T3 | `OverrideService` is named at 876, 904, 1363, 1815 and 2067 but has no method signatures. The upward proposal, the downward proposal and the approval handler (which the route and `OverrideTarget::activate` call) are described in steps only | section 3.5 (before line 1365) or section 2 | greps: `grep -n "OverrideService"` in phase-15.md finds only those lines, none with a signature; research.md finds nothing.

### V-15-04 VC-47 PEP register with tiers and relatives   (source: header, "Gap pass 1 (coverage), partly implemented", VC-47; result: GAPS 3)
- T1 SQL: PASS - `CREATE TABLE IF NOT EXISTS kyc.pep_register` (phase-15.md:378-407) has `tier` CHECK P1 to P3 (383), `status` (382), `former` (389), `persistence_until` (390), `assessment_documented` (391), `source` (393) and `source_alert_id` (394); `kyc.pep_relatives` (416-436) has `relation` and `degree` 1 to 2 (424); `country` references `ref.countries(iso2)` (385; phase-09.md:37); `canon.party_relationship` has `type` including `family` and `degree` (phase-10.md:570-577).
- T2 Route: PASS - four routes at 1076-1079: `GET` and `POST /api/v1/pep/register` (`pep.read`, `pep.write`), `PATCH /api/v1/pep/register/{id}` (`pep.write`), `POST .../relatives` (`pep.relative.confirm`); all are in the Cedar permit at 1658-1662.
- T3 Repository and types: GAP - see V-15-04-a. `PepRepository` (993-1004) covers insert_candidate, insert, set_confirmed, update, list, get and upsert_relative; `NewPepEntry`, `PepPatch` and `RelativeInput` (936-939) and `PepEntryView`, `PepRelativeView` (1047-1048) are named.
- T4 Logic: GAP - see V-15-04-b. Sources 1 and 2 are numbered (1519-1529); confirmation runs through the `pep_entry` change request (1533-1539, 1409).
- T5 Audit: PASS - `pep.candidate_created` (1523-1524), `pep.proposed` (1531), `pep.confirmed` (1539), `pep.updated` (1542), `pep.ended` (1543-1545), `pep.relative_suggested` and `pep.relative_confirmed` (1552); all in 3.11 (1629-1632).
- T6 Cross-references: GAP - see V-15-04-c. PASS: `alert.screening_alerts` with `list_kind` and `subject_kind` (phase-14.md:251-259); list kind `pep` (phase-14.md:69); decision state `true` (272); `PairMemoryService::propose` (phase-14.md:1035-1037); `screening.pair_memory_max_months` (phase-14.md:533); `pep.persistence_months` and `pep.related_tier` (DSL.md:777-779).
- T7 Consistency: PASS - 7.8 (phase-15.md:2130-2136) says no list has kind `pep` in this release, so source 1 finds nothing. The block states the consequence itself, and the base text (1518-1525) follows it.

GAP V-15-04-a | MEDIUM | T3 | `PepService` is named at 882, 904, 1516, 1816 and 1821 with no method signature for the candidate, confirmation, update and relative routes. The route handlers call methods that no section defines | section 3.8 header (1516) or section 2 | greps: `grep -n "PepService"` in phase-15.md finds only those lines, none with a signature; research.md finds nothing.

GAP V-15-04-b | MEDIUM | T4 | `POST /pep/register` logic (3.8 step 3, lines 1530-1531) is one sentence with no numbered steps and no refusal code. The 422 for a `confirmed` status on create appears only in the curl test (1993-1994) | section 3.8 step 3 | greps: `grep -n -i "confirmed"` near 1530 finds no refusal; research.md finds nothing.

GAP V-15-04-c | MEDIUM | T6 | Source 1 takes "the designation's display name from `ref.list_entry_names`" (1521-1522). That table has per-slot `raw` names with a `script` and no display-name column (phase-14.md:166-180), and `kyc.pep_register.name` must hold both `ar` and `en` keys (phase-15.md:381). A one-script designation therefore cannot satisfy the CHECK, and the mapping is not stated | section 3.8 source 1 (1521-1522) | greps: `grep -n -i "display" phase-14.md` with "name" finds nothing; `grep -n "display name"` in phase-15.md finds 1522 only (3.8); research.md finds nothing.

### V-15-05 VC-79 review queue with calendar and event due bases   (source: header, "Gap pass 1 (coverage), partly implemented", VC-79; result: GAPS 0)
- T1 SQL: PASS - `CREATE TABLE IF NOT EXISTS kyc.kyc_reviews` (phase-15.md:274-307) has `kind`, `due_at`, `due_basis` (CHECK list 281-283, which includes `trigger_sla` and `level_change`), `date_unknown`, `status` and `assignee_id`; queue index `idx_kyc_reviews_queue` (311-312).
- T2 Route: PASS - `GET /api/v1/kyc/reviews` (kyc.work, `kyc_review.read`, filters `due_before` and `assignee`, phase-15.md:1066); the Cedar permit `kyc.work.kyc_m` lists `kyc_review.read` (1660).
- T3 Repository and types: PASS - `KycReviewRepository::list(conn, f: &ReviewFilter, page: Keyset) -> Page<KycReview>` (988); `KycReviewView` (1047).
- T4 Logic: PASS - the queue order and masking are stated: open and in-progress reviews by `due_at`, NULLs first, `date_unknown` shown first, filters and the suspicion-tier exclusion in SQL (1358-1361). The due-date rules per clock and per basis are at 1288-1294.
- T5 Audit: N/A - a read; party-linked reads write access events (phase-15.md:1849-1850, 4.4).
- T6 Cross-references: PASS - `BankCalendarClock` exists (phase-09.md:692, "The business clock"); `kyc.work` admits KYC and M (docs/API.md:88); the due-basis words are in section 5 (1894).
- T7 Consistency: PASS - 7.10 (absorbed review keeps the earlier `due_at`, 2147-2153) and 7.7 (`due_basis = trigger_sla`, 2100) do not change the queue rule, and the CHECK list (281-283) already admits both bases.

### V-15-06 7.2 exit_decision and create_ops_task   (source: 7.2, phase-15.md:2031-2036; result: GAPS 4)
- T1 SQL: PASS - the block adds no table. `admin.workflow_instances.resource_kind` is a pattern CHECK (phase-04.md:331), so `exit_decision` needs no DDL.
- T2 Route: N/A - no route; the instance opens when the review completes (3.4 step 6, 1339-1341).
- T3 Repository and types: GAP - see V-15-06-a.
- T4 Logic: PASS - the sequence is numbered across 3.4 steps 6 and 8 (1339-1341, 1351-1356): `exit_candidate` opens the instance and completes the review; `record_decision` stores the outcome; M records the entry.
- T5 Audit: PASS - `exit.recorded` (1628); the instance start and completion are audited by phase 04 as `workflow_instance.started` and `.completed` (phase-04.md:1387).
- T6 Cross-references: GAP - see V-15-06-b and V-15-06-c; the `record_decision` gap is V-15-01-d (not counted again). PASS: `create_ops_task` is registered by phase 20 (phase-20.md:974).
- T7 Consistency: GAP - see V-15-06-d. 7.2 overrides 4.2 (1798-1799) as intended, but DSL.md:1052 still lists `create_ops_task` in `exit_decision`, and no DSL change is recorded (2254-2255).

GAP V-15-06-a | HIGH | T3 | `exit_decision` needs a registered `WorkflowSubject` (party members, start context, check before decision), and so does `kyc_review` for `kyc_edd_signoff` (4.2). Phase 04 says these subjects follow in phase 15 (phase-04.md:667), and a definition whose resource kind has no registered subject is refused (phase-04.md:1199, `unknown_resource_kind`). Phase 15 defines none | section 4.2 or a new section 3 block for `ExitDecisionWorkflowSubject` and `KycReviewWorkflowSubject` | greps: `grep -n "WorkflowSubject"` in phase-15.md finds nothing (the lowercase hits at 1521, 1573, 1576, 1578 and 2043 are alert and conflict-check uses); research.md finds no `WorkflowSubject`.

GAP V-15-06-b | MEDIUM | T6 | 7.2 says phase 20 "ships version 2 of `exit_decision`, which adds it after `record_decision`" (2033-2034). Phase 20 only registers the `create_ops_task` effect and says the definition calls it (phase-20.md:13 and 974); it names no version 2 | 7.2 (the claim) or phase 20's shipped-definition list | greps: `grep -n "exit_decision"` in phase-20.md finds 7 lines, none naming a version 2; `grep -n -i "version 2"` finds nothing.

GAP V-15-06-c | MEDIUM | T6 | The approval of `exit_decision` is "per the authority matrix (phase 09)" (1798), but the closed decision-type list has no key for exit decisions (phase-09.md:841-858). Phase 09 says a new decision type is a code change released with the phase that needs it | section 4.2 (line 1798) and phase 09's decision-type list, or a named key | greps: `grep -n -i "exit"` in phase-09.md finds only the word "exits" (7, 1193, 1194) and a reason code (1294); `grep -n "DecisionType"` in phase-04.md finds nothing; research.md finds nothing.

GAP V-15-06-d | LOW | T7 | 7.2 makes `exit_decision` version 1 end at `record_decision`, which contradicts DSL.md:1052 ("`approve` per authority matrix -> `record_decision` -> `create_ops_task`"). The planning rule says a phase and a planning doc that disagree are resolved in the same change (.claude/rules/planning.md:17-18). The "does not touch" list (2254-2255) allows DSL changes only for C.2, C.3 and C.8, and Open doubts (2198-2236) does not list the difference | DSL.md:1052 or the DSL change list at 2254-2255 | greps: `grep -n "create_ops_task"` in phase-15.md finds 1799, 2031 and 2033 only.

### V-15-07 7.4 ownership coverage view   (source: 7.4, phase-15.md:2049-2060; result: GAPS 1)
- T1 SQL: PASS - `kyc.rating_current` (phase-15.md:224-237: `party_id`, `level`, `cbi_level`, `rating_status`, `computed_at`) and `kyc.ownership_results` (460-474: `party_id`, `as_of`, `unknown_count`, `loops_detected`, `depth_truncated`, `superseded_at`) carry every column the single SELECT reads.
- T2 Route: PASS - `GET /api/v1/ownership/coverage`, scope `dashboard.read`, Cedar `rating.distribution.read` named in the description (2052). The scope admits M, CH and IA (docs/API.md:99), the same roles as the permit at 1699-1701.
- T3 Repository and types: GAP - see V-15-07-a. `OwnershipService::coverage(ctx)` is named (2054); no response type or utoipa schema is named.
- T4 Logic: PASS - the one SELECT is specified: grouping by current level, the six counts, the suspicion-tier rule ("never listed", counted with the rest) and no party id returned (2054-2059).
- T5 Audit: PASS - read only, counts only; the party-linked access rule (1849-1850) does not apply to counts with no party id.
- T6 Cross-references: PASS - `rating.distribution.read` is in the 3.12 permit (1699-1701); `dashboard.read` (docs/API.md:99); the refresh interval is `controls[level].kyc_refresh` in DSL.md (1469, 1472).
- T7 Consistency: PASS - no other block changes the coverage view. Section 5 (K3, 1903-1908) does not mention the header strip that 7.4 adds (2059-2060); the block defines its content, so it is not a contradiction.

GAP V-15-07-a | MEDIUM | T3 | The coverage response has no type name, no struct fields and no utoipa schema. The six counts exist in prose only (2056-2058), and the section 2 schema list (1041-1051) has no coverage view | section 7.4 (a response type) and section 2 (utoipa schemas) | greps: `grep -n -i "coverage"` in phase-15.md finds 5, 16-18, 1166, 2047-2054 and 2078 only, with no DTO; research.md DR-395 names the item without a type.

### V-15-08 7.5 override analytics   (source: 7.5, phase-15.md:2062-2076; result: GAPS 2)
- T1 SQL: PASS - `kyc.overrides` (phase-15.md:108-136: `direction`, `proposed_by`, `end_reason`, `effective_from`); `canon.customer.branch_id` (phase-10.md:603) and `recorded_to` (phase-10.md:622); `alert.alerts` has `party_id` (phase-17.md:120) and `created_at` (phase-17.md:160); `casework.case_parties` has `party_id` and `role` (phase-17.md:372-378); `casework.decisions` has `type` with `str` (phase-18.md:142) and `decided_at` (147).
- T2 Route: PASS - `GET /api/v1/rating/overrides/analytics`, scope `dashboard.read`, Cedar `rating.distribution.read` in the description (2065).
- T3 Repository and types: GAP - see V-15-08-a. `OverrideService::analytics(ctx, from, to)` is named (2067); the route row does not name `from` and `to`, and no response type is named.
- T4 Logic: GAP - see V-15-08-b. Steps 1 to 3 are numbered with their counts (2068-2075), and the null rule before phase 18 is stated (2075); the window refusal is missing.
- T5 Audit: PASS - "No audit event (a read)" (phase-15.md:2076). The response carries user ids and branches, not party ids (2070-2072), so no party-linked access event applies.
- T6 Cross-references: PASS - `casework.case_parties` (phase-17.md:372) and `casework.decisions` (phase-18.md:137) exist with the columns used; `alert.alerts` (phase-17.md:106); `dashboard.read` (docs/API.md:99); "From phase 18 on" matches phase 18 creating `casework.decisions`.
- T7 Consistency: PASS - no conflict with 3.5 (expiry ends as `expired`, 1399; CHECK at 0050 line 118) or with 7.4.

GAP V-15-08-a | MEDIUM | T3 | No response type for the analytics (per-proposer rows, per-branch rows, overall counts, nulls before phase 18) and no utoipa schema. The query parameters `from` and `to` are not named in the route row (2065) | section 7.5 (response type and parameters) and section 2 (utoipa list, 1041-1051) | greps: `grep -n -i "analytics"` in phase-15.md finds only the header list (16-18) and 2062-2067; research.md finds nothing.

GAP V-15-08-b | MEDIUM | T4 | The window rule "at most 366 days" (2068) has no refusal: no status and no problem code for a longer window or for `from` after `to` | section 7.5 (2068), with the code from section 6 of docs/API.md | greps: `grep -n "366"` in phase-15.md finds 2068 only; research.md finds nothing.

### V-15-09 7.7 trigger evaluation and account events   (source: 7.7, phase-15.md:2086-2128; result: GAPS 2)
- T1 SQL: PASS - `kyc.exit_register` with status `recorded` (phase-15.md:486) backs `party.on_exit_register`; `canon.account` has `opened_on`, `status` with `active` and `dormant` (phase-10.md:640-642) and bi-temporal `recorded_from` and `recorded_to`; `admin.batches` has `status` of type `admin.batch_status` (phase-11.md:73) and `business_to` (phase-11.md:148).
- T2 Route: N/A - no route; the block adds job steps and triggers only.
- T3 Repository and types: GAP - see V-15-09-a. The `rating_compute` payload fields `event` and `event_ref` are named (2089-2091); `RatingService::rescore` with reason `trigger` (2093) names an existing method.
- T4 Logic: PASS - steps 1 to 8 are numbered (2089-2128) and state the order of checks: triggers in id order, the first holding trigger acts, later ones are listed in the audit payload (2093-2097); the party's advisory lock (3.2 step 1) and the coalesce key (2090-2092).
- T5 Audit: PASS - `rating.trigger_applied {party_id, trigger_code, action, review_id?}` is named in step 3 (2104-2105) and added to 3.11 by step 8 (2127-2128); an event review's open is `kyc_review.opened` (1620).
- T6 Cross-references: GAP - see V-15-09-b. PASS: TR-14 and TR-24 match research/26 lines 293 and 305 (dormant reactivation; exited customer opens an account); `account.reactivated` and `account.opened` are in DSL.md:807; the actions are in DSL.md:809; producers `screening.hit` (phase-14.md:1311-1314), `profile.breach` (phase-17.md:1842-1854), `outreach.failed` (phase-19.md:1893-1895) and `visit.recorded` (phase-22.md:947-950) exist; `canon.touched` (phase-11.md:833-834); `BankCalendarClock` (phase-09.md:692).
- T7 Consistency: PASS - step 8 says 4.1 and 3.11 gain the new entries; the last row of the 3.3 table ("not wired here", 1278) is replaced as the block says (2088); no conflict with 3.6 rule 3 (`TRG_` codes, 1459-1461) or with 7.10.

GAP V-15-09-a | MEDIUM | T3 | Step 5 needs "the business date of the previous mapped batch of the same source table" (2114-2116), but no read is named: no repository method or query over `admin.batches` (status `mapped`, same `source_table`, the latest `business_to`) | section 7.7 step 5 (a named read) | greps: `grep -n -i "previous mapped\|previous batch"` in phase-15.md finds 2115 only; phase-11.md defines `admin.batches` (136) and uses `mapped` only in an index predicate (171); research.md finds nothing.

GAP V-15-09-b | LOW | T6 | `due_at = BusinessClock::add(now, review_within)` (2100-2101) is called with two arguments and treated as a timestamp. Phase 09 defines `add(from, n, unit: BusinessUnit) -> Result<ClockResult, ClockError>` (phase-09.md:459), and `ClockResult` carries `fire_at`, `calendar_id` and `calendar_version` | section 7.7 step 3 (name the unit, working days, and the fields used) | greps: `grep -n "BusinessClock::add"` in phase-15.md finds 2100 only; `grep -n "fn add\|BusinessClock::add"` in phase-09.md finds 459 and 1279 only.

### V-15-10 7.9 approver of a downward override   (source: 7.9 under adversarial run 1, phase-15.md:2138-2143; result: GAPS 1)
- T1 SQL: PASS - `kyc.overrides` has `approved_by` with CHECK "approved_by <> proposed_by" (phase-15.md:131) and a downward row must carry `change_request_id` (129), so the four-eyes rule the step relies on exists in SQL.
- T2 Route: N/A - the approval is a phase 04 task decision in the inbox, with no phase 15 route. The Cedar action `override.approve_down` is at 1677-1684.
- T3 Repository and types: PASS - `OverrideRepository::set_status(..., approved_by, ...)` (971-972); `RelationshipConflictCheck` (3.9, 1572-1581); the `Holders` trait (phase-04.md:702).
- T4 Logic: PASS - the block fixes the step: one step `approve` role CHK, "not the proposer, not the relationship owner", timeout `esc` M after 3 business days (2142-2143). Refusals: `E_SELF_APPROVAL` 403 (docs/API.md:971); the relationship owner refusal is `DenyReason::Conflict`, 403 or masked 404 (1579-1580). The `esc M 3bd` grammar matches phase-04.md:1264.
- T5 Audit: PASS - `override.applied {direction: down}` at activation (1391-1392; 3.11 line 1618); the task decision is audited by phase 04 (task events, phase-04.md:982).
- T6 Cross-references: GAP (shared) - the step's terminal effect `record_decision` is unregistered for this kind (V-15-01-d; not counted again). PASS: the CHK role exists in the phase 04 shipped-definition grammar (phase-04.md:1264).
- T7 Consistency: GAP - see V-15-10-a. The block makes the task role CHK only (2142), but the Cedar permit still admits CHK and M (1677-1679) and the section 6 step expects M's attempt to return `E_SELF_APPROVAL` (1983-1984). Neither the permit nor the test is updated, so two rules remain for who may approve.

GAP V-15-10-a | MEDIUM | T7 | Who may approve a downward override: the block makes the task role CHK only (2142), but the Cedar permit admits CHK and M (1677-1679) and section 6 item 8 expects M's approval attempt to fail with `E_SELF_APPROVAL` (1983). The block does not change 3.12 or section 6 | section 3.12 (permit at 1677-1679) and section 6 item 8 (1983-1984) | greps: `grep -n "approve_down"` in phase-15.md finds 1647, 1677-1683 and 1707 only, none naming the task role; `grep -n "CHK or M\|CHK,M"` finds 1388, 1677, 1794 and 2141; research.md finds nothing.

### V-15-11 7.10 absorbed event review keeps the earlier deadline   (source: 7.10 under adversarial run 4, phase-15.md:2147-2153; result: GAPS 1)
- T1 SQL: PASS - `kyc.kyc_reviews` has `due_at` (phase-15.md:280) and `escalated_at` (292); the grant at 718 lists both as updatable.
- T2 Route: N/A - timer and absorb path only.
- T3 Repository and types: PASS - the absorb path is `KycReviewRepository::open_or_join(tx, r: &NewReview)` (983); `TimerService::cancel_for_owner` and `schedule` exist (phase-00.md:437, 561-562).
- T4 Logic: PASS - the rules are stated: the earlier of the two due dates wins, the cancel and reschedule run in the same transaction, the handler re-arms and writes `escalated_at` and `kyc_review.overdue` only once (2148-2153).
- T5 Audit: PASS - `kyc_review.overdue` is named (2152-2153) and listed in 3.11 (1621).
- T6 Cross-references: PASS - `TimerService::cancel_for_owner(tx, owner_kind, owner_id, kind)` (phase-00.md:437, 562) matches the call in 7.10; timer kind `kyc_review_clock` is registered by phase 15 (phase-00.md:868).
- T7 Consistency: GAP - see V-15-11-a.

GAP V-15-11-a | MEDIUM | T7 | The section 2 absorb statement updates only `trigger_code`: "DO UPDATE SET trigger_code = EXCLUDED.trigger_code" (phase-15.md:1020). 7.10 needs `due_at` to move earlier and the timer to move with it (2148-2150). The block names 3.4 rule 5 (1308-1311), not the section 2 statement, so the statement as written never moves the deadline | section 2, `open_or_join` (1019-1021) | greps: `grep -n "DO UPDATE"` in phase-15.md finds 1020 only; `grep -n "LEAST("` finds nothing; research.md finds nothing.

### V-15-12 7.11 trigger job applies once   (source: 7.11 under adversarial run 6, phase-15.md:2157-2161; result: GAPS 0)
- T1 SQL: N/A - the block adds no table; the job is an `admin.jobs` row of phase 00.
- T2 Route: N/A.
- T3 Repository and types: PASS - `JobContext::complete_in(tx)` exists (phase-00.md:1039).
- T4 Logic: PASS - the handler completes the job in the transaction that writes the re-score, the review and `rating.trigger_applied` (2158-2160); a re-run after a lost completion finds the job done and writes nothing (2160). The coalesce key and the advisory lock are in 4.1 (1764) and 3.2 step 1.
- T5 Audit: PASS - `rating.trigger_applied` is written in the same transaction (7.7 step 3, 2104-2105).
- T6 Cross-references: PASS - `complete_in` sits in phase 00's section 7+ (heading at phase-00.md:831; method at 1039), as the block says.
- T7 Consistency: PASS - matches 7.7 step 1 (`event_ref` never stored, 2092 and 2161) and the 4.1 coalesce key (1764); no other block changes this job.

## Summary

- Items checked: 12 (V-15-01 to V-15-12).
- Items fully PASS: 2 (V-15-05 VC-79 review queue; V-15-12 7.11 trigger job applies once).
- GAP lines by severity: HIGH 4, MEDIUM 16, LOW 3 (23 in all).
- The `record_decision` gap (V-15-01-d) is shared by V-15-03, V-15-06 and V-15-10; it is counted once, under V-15-01.

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-15-01-a | HIGH | T3 | No ExitRepository; ExitService and ExitQuery never declared | section 2 after line 1014; section 4.3 (1815-1820) |
| V-15-01-b | MEDIUM | T3 | NewExitEntry has no review_id; its source is not stated | section 2 (940-941); 3.4 step 8 (1356) |
| V-15-01-c | MEDIUM | T4 | No refusal code for an inactive or other-family reason code | 3.4 step 8 (1353-1355); 3.10 table |
| V-15-01-d | HIGH | T6 | record_decision registered for case only; no kyc or exit registration | section 4.2 (1794-1799); section 4.3 |
| V-15-02-a | HIGH | T1 | Registration country: no such column in canon.party | 3.3 row 2 (1272); section 1 or a phase 10 change |
| V-15-02-b | MEDIUM | T3 | No repository method for the paged party-by-country query | section 2 (944-1014) |
| V-15-02-c | MEDIUM | T6 | effective_at is not in the consumed event payload | 3.3 row 2 (1272); 4.2 (1789-1790) |
| V-15-02-d | LOW | T6 | DSL TR-10 event name country_table.changed not reconciled | 3.3 row 2; 4.2 |
| V-15-03-a | MEDIUM | T3 | OverrideService propose and approve methods have no signatures | section 3.5 (before 1365) or section 2 |
| V-15-04-a | MEDIUM | T3 | PepService methods have no signatures | 3.8 header (1516) or section 2 |
| V-15-04-b | MEDIUM | T4 | POST /pep/register logic is one sentence; no refusal code | 3.8 step 3 (1530-1531) |
| V-15-04-c | MEDIUM | T6 | Display name from ref.list_entry_names: no column; ar and en keys | 3.8 source 1 (1521-1522) |
| V-15-06-a | HIGH | T3 | No WorkflowSubject for exit_decision or kyc_review | section 4.2 or a new section 3 block |
| V-15-06-b | MEDIUM | T6 | Phase 20 ships no version 2 of exit_decision | 7.2 (2033-2034) or phase 20 (13, 974) |
| V-15-06-c | MEDIUM | T6 | No decision type for exit approval in the authority matrix | 4.2 (1798); phase 09 (841-858) |
| V-15-06-d | LOW | T7 | Contradicts DSL.md:1052; neither DSL nor Open doubts records it | DSL.md:1052; 2254-2255 |
| V-15-07-a | MEDIUM | T3 | No response type or utoipa schema for the coverage view | 7.4 (2054-2058); section 2 (1041-1051) |
| V-15-08-a | MEDIUM | T3 | No response type; the from and to parameters are not named | 7.5 (2065-2067); section 2 (1041-1051) |
| V-15-08-b | MEDIUM | T4 | No refusal for a window over 366 days or from after to | 7.5 (2068) |
| V-15-09-a | MEDIUM | T3 | No named read of the previous mapped batch | 7.7 step 5 (2114-2116) |
| V-15-09-b | LOW | T6 | BusinessClock::add called with two arguments; phase 09 signature differs | 7.7 step 3 (2100-2101) |
| V-15-10-a | MEDIUM | T7 | Cedar permit and section 6 still admit M as approver | 3.12 (1677-1679); section 6 item 8 (1983-1984) |
| V-15-11-a | MEDIUM | T7 | The open_or_join statement never moves due_at | section 2 (1019-1021) |
