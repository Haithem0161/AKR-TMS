# Verification pass, phase 14 (Haiku draft for lead review)

Items selected before checking (12):
- Header "Implemented here" list (phase-14.md:10, 17 items; taken from the start, middle and end): V-14-01 L-36, V-14-02 U-02, V-14-03 VC-04, V-14-04 VC-24, V-14-05 VC-31.
- Gap pass blocks, one item from each of four blocks not headed "Adversarial pass": V-14-06 7.4 hold alerts (phase-14.md:1321), V-14-07 7.6 delisting by absence (phase-14.md:1335), V-14-08 7.9 access event of the name search (phase-14.md:1356), V-14-09 7.10 one-hour re-screen (phase-14.md:1367).
- Blocks under "Adversarial pass (2026-10-10), run 2" (phase-14.md:1401; only blocks 7.12 and 7.13 sit under it): V-14-10 7.12 fingerprint unique partial index, V-14-11 7.12 insert_candidates conflict rule, V-14-12 7.13 superseded-alert guard trigger.

## Checks

### V-14-01 L-36 freeze lists excluded from good-guy rules and bulk decisions; no amount threshold   (source: header "Implemented here" list, phase-14.md:10; result: GAPS 4)
- T1 SQL: PASS - phase-14.md:405 "CHECK (NOT (scope -> 'list_codes' ?| ARRAY['iq_national', 'un', 'ofac_sdn', 'eu', 'uk']))" (the five freeze codes; bank is kind internal). The bulk refusal needs no column (N/A). The absence of an amount threshold needs no column or setting (N/A).
- T2 Route: PASS - phase-14.md:834 "POST /screening/alerts/bulk-decision | screening.work" (screening.alert.bulk_decide); phase-14.md:840 "POST /screening/good-guys | screening.work" (screening.goodguy.propose). Cedar: phase-14.md:1089 and 1102 (bulk_decide), 1092 and 1105 (goodguy.propose).
- T3 Repository and types: GAP V-14-01-a (bulk service method not named). Named with types: phase-14.md:695 BulkDecisionRequest, phase-14.md:696 GoodGuyProposal, phase-14.md:753 GoodGuyRepository::insert_on_activation. Row type NewGoodGuy is undefined: systemic, see V-14-03-a.
- T4 Logic: GAP V-14-01-b and V-14-01-c. Good-guy refusal is stated: phase-14.md:1046 "list_codes must not include a freeze list (422 E_VALIDATION, freeze_list_excluded". Its rules are bullets, not numbered steps, but each has a code. Bulk: phase-14.md:1033 "the list is not a freeze list;" gives no refusal code.
- T5 Audit: PASS - phase-14.md:1033 "one batch audit screening.bulk_decided {count, entry_id}"; per alert, step 11 at phase-14.md:1031 names screening.alert_decided; phase-14.md:1172 names good_guy_rule.activated.
- T6 Cross-references: PASS for ListCode closed set (phase-14.md:672), admin.change_requests (phase-04.md:121, the table named by phase-14.md:398), E_VALIDATION (API.md section 6, row at API.md:985). GAP V-14-01-d for StepUpAction::Decision.
- T7 Consistency: PASS - phase-14.md:1046 (good-guy), phase-14.md:834 ("never freeze lists", bulk) and phase-14.md:1033 (bulk) agree; open doubt at phase-14.md:1455 states both are refused.
- Gaps:
  - GAP V-14-01-a | MEDIUM | T3 | The bulk-decide service method has no name and no parameters; 2.6 lists "bulk decide" as a capability only | section 3.6 (Bulk incorrect) and 2.6 | `grep -n "AlertService::" phase-14.md` (no hit); `grep -n "bulk_decide" phase-14.md` (route name 834, router 873, audit names 1033 and 1166, Cedar 1089 and 1102; no service method); research.md: DR-102 (line 113) only
  - GAP V-14-01-b | MEDIUM | T4 | The refusal status and code for a bulk decision on a freeze list; DR-102 (research.md:113) decides the exclusion but gives no code | section 3.6 (Bulk incorrect) and 3.14 | `grep -n "freeze_list_excluded" phase-14.md` (3.8, 3.14 and check 6 only; no bulk hit); `grep -n "bulk_not_identical" phase-14.md` (3.6 and 3.14 only); `grep -n "freeze_list_excluded" research.md` (none)
  - GAP V-14-01-c | MEDIUM | T4 | Good-guy freeze refusal: the Cedar forbid gives 403 E_FORBIDDEN (3.14) while the service gives 422 (3.8 and check 6); no order of checks is stated; the deny reason "freeze_list" has no row in 3.14 | sections 3.8, 3.11 and 3.14 | `grep -n 'freeze_list")' phase-14.md` (1142 only); `grep -n "E_FORBIDDEN" phase-14.md` (3.14 at 1181 and check 6 at 1284); `grep -n 'freeze_list")' research.md` (none). Phase 03 defines the @deny vocabulary as out_of_scope or tier (phase-03.md:704) and self_approval (phase-03.md:717); freeze_list is not in it.
  - GAP V-14-01-d | LOW | T6 | Step-up constant StepUpAction::Decision (3.6 step 1, used by the decision and bulk paths) is not declared by phase 01 or phase 14; phase 01 asks for declared StepUpAction { name, class } constants (phase-01.md:711) | section 3.6 step 1 and section 4.5 (phase 01 row) | `grep -rn "StepUpAction::Decision" docs/plan/` (phase-14.md:1021 only); `grep -n "StepUpAction::" phase-01.md` (none)

### V-14-02 U-02 a parse failure fails the version with an incident; the last good version stays   (source: header "Implemented here" list, phase-14.md:10; result: GAPS 2)
- T1 SQL: PASS - phase-14.md:99-101 failure_code enum includes "'parse_error'"; phase-14.md:107 file_sha256 BYTEA CHECK 32 bytes; phase-14.md:108 evidence_id; phase-14.md:97 status CHECK includes 'failed'.
- T2 Route: N/A - the parse runs as the worker job list_parse (phase-14.md:1207, payload {version_id}); no route. The state is read by GET /lists/{id}/versions (phase-14.md:820).
- T3 Repository and types: PASS - ListRepository::set_status(tx, version, expected, change) at phase-14.md:712; EvidenceStore::open named at phase-14.md:802 and defined at phase-02.md:881 (open(id) -> VerifiedReader). VersionChange row type undefined: systemic, see V-14-03-a.
- T4 Logic: GAP V-14-02-a. Step 2 gives the failure for a SHA-256 mismatch only ("sets failed with parse_error and an incident", phase-14.md:948). Step 3 (phase-14.md:949 "Parse by the closed feed_format set.") has no failure path. Step 4 (phase-14.md:950) reconciles against expected_count but gives no outcome for a mismatch, although count_mismatch is in the enum (phase-14.md:100). The last good version staying active is stated at phase-14.md:937 and phase-14.md:941.
- T5 Audit: GAP V-14-02-b. The "incident" of phase-14.md:948 has no kind, table, alarm or event. 3.13 names list.failed with failure_code (phase-14.md:1162) only. DR-164 (research.md:175) makes process failures incidents, but the phase 20 kinds (phase-20.md:453) do not cover a list failure.
- T6 Cross-references: PASS - EvidenceStore::open (phase-02.md:881). No other phase defines a list-parse incident (grep "incident" in phases 00, 02, 05, 10, 17, 19: no list or parse incident; phase 20 IncidentKind has five ops kinds only).
- T7 Consistency: PASS - failure of a SHA mismatch (phase-14.md:948) and the count-drop failure (phase-14.md:951) use the same failed state; no contradiction found.
- Gaps:
  - GAP V-14-02-a | MEDIUM | T4 | Parse error inside step 3 has no failure code or state, and step 4 gives no outcome for a count mismatch (count_mismatch is in the enum but no step writes it) | section 3.2, list_parse steps 3 and 4 | `grep -n "parse_error"` (100 DDL and 948 only); `grep -n "count_mismatch"` (phase-14.md:100 DDL only; phase-12 uses it for its own table)
  - GAP V-14-02-b | MEDIUM | T5 | The incident raised on a parse failure has no kind, alarm or audit event name; 3.13 names list.failed only | section 3.2 list_parse step 2 and section 3.13 | `grep -n -i "incident" phase-14.md` (10 header and 948 only); `grep -n -i "incident" phase-00.md` (none); phase-20.md:453 (five ops kinds only); research.md:175 (DR-164, process failures as ops incidents)

### V-14-03 VC-04 versions staged, compared with the active version and gated before activation   (source: header "Implemented here" list, phase-14.md:10; result: GAPS 1)
- T1 SQL: PASS - status CHECK with 'loading' and 'validated' (phase-14.md:97); diff_summary JSONB (phase-14.md:111); alert.harness_runs.passed_gate (phase-14.md:337); grants UPDATE status, diff_summary, activated_at, activated_by, superseded_at, version for akr_app (phase-14.md:427-428) and lists.active_version_id (phase-14.md:422-423); guard transitions loading to validated and validated to active (phase-14.md:466-468).
- T2 Route: PASS - "POST /lists/{id}/versions | screening.work" (phase-14.md:821, screening_list.upload, SU); "POST /lists/{id}/versions/{v}/activate" (phase-14.md:823, SU, 4E, If-Match, screening_list.activate); "POST /screening/harness-runs" (phase-14.md:842, screening.harness.run). Cedar SS and M permits at phase-14.md:1093-1095 and 1106-1108.
- T3 Repository and types: GAP V-14-03-a (systemic). Named with parameters: ListRepository::insert_version, write_entries, set_status, activate, diff_counts (phase-14.md:709-714); DTOs UploadListVersion and ActivateListVersion (phase-14.md:689-690). The row types they take are undefined (listed in V-14-03-a).
- T4 Logic: PASS - list_parse steps 1 to 7 are numbered with failure codes (phase-14.md:947-953); list_activate steps (1) to (4) give 409 E_NOT_ALLOWED_IN_STATE and 422 harness_not_passed (phase-14.md:957); the comparison with the active version is step 7 ("against the active version by entry_key and entry_fp", phase-14.md:953); the gate rule is at phase-14.md:1055; "One transaction" at phase-14.md:953 and phase-14.md:957.
- T5 Audit: PASS - list.version_uploaded, list.fetched (phase-14.md:1161); list.parsed, list.failed, list.validated (phase-14.md:1162); list.activated, list.superseded (phase-14.md:1163); harness.run_completed (phase-14.md:1171).
- T6 Cross-references: PASS - EvidenceStore::stage (phase-02.md:839, owner kind and id, kind, content type, size); StepUpService::consume (phase-01.md:731); StepUpAction class "approval" (phase-01.md:710).
- T7 Consistency: PASS - "The version stays loading until the gate passes" (phase-14.md:953) agrees with "Pass sets the version validated" (phase-14.md:1055) and with the validated state (phase-14.md:639).
- Gaps:
  - GAP V-14-03-a | MEDIUM | T3 | Row and argument types used by the repository traits, the ports and the DTOs are not defined in any plan file: NewListVersion, NewEntry, VersionChange, DiffCounts, VersionFilter, NewAlert, NewDecision, NewPairMemory, PairReason, NewGoodGuy, NewHarnessRun, HarnessResult, AlertFilter, PairFilter, HarnessFilter, IdentifierInput, GoodGuyRuleInput, ScoringConfig, ScoreComponents, AlignedPair, IdentifierCheck, SubjectRow, NameInput, BankListRow, FeedError (the same gap applies to every item that passes one of these) | section 2.3 and section 2.5 (new structs), and section 2.4 (trait argument types) | `grep -rn "struct NewListVersion\|NewListVersion {" docs/plan/` (none); the same grep for each name finds only its trait or DTO line in phase-14.md (for example NewListVersion at 709 only)

### V-14-04 VC-24 pair memory and good-guy rules with approver, expiry and the QA sample; never on freeze lists   (source: header "Implemented here" list, phase-14.md:10; result: GAPS 2)
- T1 SQL: PASS - alert.pair_memory with approved_by (phase-14.md:371), expires_at (373), status (367), suppressions (377), last_suppressed_at (378); one open row per pair (phase-14.md:386-387); alert.good_guy_rules with expires_at (399), closed_reason (401) and the freeze CHECK (phase-14.md:405); grants INSERT and UPDATE (phase-14.md:443-446).
- T2 Route: GAP - revoke route missing (V-14-04-b). Other routes PASS: "POST /screening/pair-memory/{id}/approve" (phase-14.md:837, screening.pair.approve, SU, 4E); "DELETE /screening/pair-memory/{id}" (phase-14.md:838); "POST /screening/good-guys" (phase-14.md:840, screening.goodguy.propose).
- T3 Repository and types: PASS - PairMemoryRepository propose, approve, active_for, invalidate, expire_due, record_suppressions (phase-14.md:741-747); GoodGuyRepository insert_on_activation and close_expired (phase-14.md:753, 755); GoodGuyProposal (phase-14.md:696); PairMemoryService::propose named at phase-14.md:1029 (decide step 9). Row types NewPairMemory, PairReason and NewGoodGuy are undefined: systemic, V-14-03-a.
- T4 Logic: PASS - propose status and expiry cap (phase-14.md:1037: source alert must be incorrect; expiry_too_far at 1183); approve refused to the proposer (phase-14.md:1038; Cedar at 1134-1136); invalidation reasons (phase-14.md:1039); drift monitor (phase-14.md:1040); good-guy refusal and expiry (phase-14.md:1046-1050). The freeze wording is handled under T7 (V-14-04-a).
- T5 Audit: PASS - pair.proposed, pair.approved, pair.invalidated, pair.suppressed, pair.expired (phase-14.md:1169); good_guy.suppressed, good_guy.expired (phase-14.md:1170); good_guy_rule.activated (phase-14.md:1172).
- T6 Cross-references: PASS - qa_suppression_sample is a phase 19 job (phase-19.md:1659) and draws the rows whose last_suppressed_at falls in the ISO week (phase-19.md:1883-1884); phase-19.md:1715 states that phase 19 draws the pair-memory sample, not phase 14. Change-request route POST /change-requests (phase-04.md:778). Pair-memory cap of 12 months is the setting at phase-14.md:533-534.
- T7 Consistency: GAP V-14-04-a (freeze lists). PASS for the QA sample - the base sweep draws a sample (phase-14.md:1041) and 7.3 replaces it ("This phase's sweep keeps expiry and the counters and draws no sample", phase-14.md:1319), which phase 19 confirms (phase-19.md:1715).
- Gaps:
  - GAP V-14-04-a | MEDIUM | T7 | The header says VC-24 is "never on freeze lists" for pair memory, but DR-102 (research.md:113) excludes freeze lists only for good-guy rules and bulk incorrect, and 3.7 and 7.3 expect suppression rows on freeze-list entries; the two must agree | line 10 (header) versus sections 3.7 and 1.3 and DR-102 | `grep -n -i "freeze" phase-14.md` on lines 1037-1042 (1041 only); no refusal clause in the 1.3 DDL (lines 356-385; line 362 lists 'freeze' as a value only); `grep -n "never on a freeze" phase-14.md` (1436-1437 only: good-guy rules and bulk incorrect)
  - GAP V-14-04-b | MEDIUM | T2 | No route or request field says who proposes a good-guy revoke or how; the revoke body {"action": "revoke", "rule_id": ...} (phase-14.md:1047) fits neither GoodGuyProposal (phase-14.md:696) nor POST /screening/good-guys (phase-14.md:840); the generic change-request route (phase-04.md:778) needs scope config.author, which SS and M do not hold | sections 2.3, 2.8 and 3.8 | `grep -n "revoke"` (1047 and the closed_reason CHECK at 401 only); `grep -n '"action"'` (1047 in 3.8 only; none in section 2.3)

### V-14-05 VC-31 every closed alert a labelled pair; thresholds change only by change request   (source: header "Implemented here" list, phase-14.md:10; result: GAPS 2)
- T1 SQL: PASS - closed-alert columns exist: state (phase-14.md:272), decided_at (285), decision reason_codes (307) and list_kind (313). The labelled pair itself has no table or view: see V-14-05-a.
- T2 Route: N/A - no route is named for the labelled pair or for the MLRO acceptance into the corpus (phase-14.md:1057). The threshold change uses the phase 04 change-request routes (phase-04.md:778).
- T3 Repository and types: PASS - ListSettingTarget, ScreeningPolicyTarget and GoodGuyTarget (phase-14.md:806); their prepare, activate and revert methods (phase-14.md:1075); ListSettingProposal (phase-14.md:691).
- T4 Logic: GAP V-14-05-a for the labelled pair. Threshold change PASS: phase-14.md:1075 gives the dry run over at most 10,000 subjects, the volume_effect values, "unknown can never be approved as material", the write through SettingsService and the revert from the proposed diff.
- T5 Audit: PASS for the change events (phase-14.md:1172: list_setting.changed, screening_policy.changed, good_guy_rule.activated). The setting event name is wrong: see V-14-05-b.
- T6 Cross-references: GAP V-14-05-b (LOW). PASS for phase-04.md:50 (volume_effect enum with unknown), phase-04.md:121 (admin.change_requests), phase-04.md:139 (impact JSONB), phase-04.md:642 (ChangeTarget trait), docs/DSL.md:922 ("## D.3 Change classes") and phase-00.md:571 (SettingsService).
- T7 Consistency: PASS - "a lowered threshold is material" (phase-14.md:1071) agrees with "Loosening is material" (phase-14.md:1072) and with phase-04.md:827 ("any loosening is material").
- Gaps:
  - GAP V-14-05-a | MEDIUM | T4 | "Every closed alert becomes a labelled pair" has no record, view, query or steps; "the MLRO accepts entries into the corpus" has no route, table, job or change request | section 3.9 (Feedback) and section 2.8 | `grep -n "labelled" phase-14.md` (10 header, 1057 feedback, 1452 open doubt; no record or query); `grep -n "accepts" phase-14.md` (1057 only); research.md: no DR on labelled pairs or corpus acceptance ("corpus" appears only in DR-68, DR-71 and DR-100, none on labelled pairs)
  - GAP V-14-05-b | LOW | T6 | The setting audit event is named setting.changed "from phase 00", but phase 00 names setting.updated | sections 3.10 and 3.13 | `grep -n "setting.changed" docs/plan/phase-00.md` (none); phase-00.md:651 names setting.updated

### V-14-06 7.4 hold alerts from payment holds (ScreeningService::open_hold_alert, phase 29 owns the extension)   (source: block 7.4, phase-14.md:1321; result: PASS)
- T1 SQL: PASS - alert.screening_runs kind 'payment' (phase-14.md:225), subject_kind 'payment_party' (phase-14.md:228), trigger_ref UUID (phase-14.md:230), requested_by nullable and required only for adhoc (phase-14.md:241, 246), finished_at required for completed (phase-14.md:247); assess.holds.party_names_enc exists (phase-29.md:294).
- T2 Route: N/A - no route; the caller is HoldSweepService (phase-29.md:905 and phase-29.md:1016).
- T3 Repository and types: PASS - ScreeningService::open_hold_alert(run, candidate) at phase-14.md:1322 with its parameters; AlertRepository::insert_candidates (phase-14.md:722). Parameter types are part of the systemic gap V-14-03-a.
- T4 Logic: PASS - the numbered steps are in phase-29.md:1016 (step 2: run with requested_by NULL, trigger_ref = hold id, status completed, then insert_candidates). The skip rule at phase-14.md:1327 uses "match key", which phase 29 defines (phase-29.md:1131).
- T5 Audit: PASS - screening.run_started and screening.run_finished for each run (phase-14.md:1164); screening.alert_created for each candidate (phase-14.md:1165 and step 3 at phase-14.md:998).
- T6 Cross-references: PASS - phase-29.md:1351 (section 4.5 item 4) names the method; phase-29.md:905 (HoldSweepService); phase-29.md:294 (party_names_enc).
- T7 Consistency: PASS - the 7.4 comment for subject_id (phase-14.md:1325) updates the base comment "adhoc runs only" (phase-14.md:229); the CHECK at phase-14.md:246 applies to adhoc runs only, so payment runs are allowed.
- Gaps: none.

### V-14-07 7.6 delisting by absence, count-drop guard   (source: block 7.6, phase-14.md:1335; result: GAPS 1)
- T1 SQL: PASS - acknowledged_count_drop (phase-14.md:114); diff_summary (phase-14.md:111); delisted_on (phase-14.md:147); screening.max_count_drop_pct setting (phase-14.md:527); INSERT for akr_app and akr_ingest (phase-14.md:424-425) carries the acknowledgement; UPDATE of diff_summary (phase-14.md:427-430).
- T2 Route: PASS - "POST /lists/{id}/versions | screening.work" (phase-14.md:821); its body carries the acknowledgement (UploadListVersion, phase-14.md:689).
- T3 Repository and types: PASS - UploadListVersion (phase-14.md:689) and ListRepository::diff_counts (phase-14.md:714). DiffCounts is undefined: systemic, V-14-03-a.
- T4 Logic: GAP V-14-07-a (delisted_on source). Count-drop rule PASS: list_parse step 5 gives "fail with count_drop" unless acknowledged (phase-14.md:951); the 7.6 guard restates it (phase-14.md:1338).
- T5 Audit: PASS - a count-drop failure writes list.failed with failure_code (phase-14.md:1162); removed counts sit in diff_summary (phase-14.md:1339).
- T6 Cross-references: PASS - phase 20 reads absence as delisting: "The gate of section 3.5 treats a missing entry in the active version as delisting" (phase-20.md:1137, Open doubts item 22). DR-376 (research.md:387) decides the absence rule and the publisher-date rule for delisted_on.
- T7 Consistency: PASS - 7.6 (phase-14.md:1338) and step 5 (phase-14.md:951) use the same threshold and the same acknowledgement.
- Gaps:
  - GAP V-14-07-a | MEDIUM | T4 | Neither listed_on nor delisted_on has a source mapping in the parse; 7.6 sets delisted_on "when the publisher's file carries a delisting date" but names no field per feed_format; DR-376 (research.md:387) fixes the rule, not the field | section 3.2 list_parse step 6 (phase-14.md:952) and block 7.6 | `grep -n "listed_on" phase-14.md` (146, 147, 570, 571, 1337 only); `grep -n "delisted" phase-14.md` (147, 571, 1336, 1337 only); step 6 names no date field; `grep -n "listed_on" research.md` (DR-162 and DR-376 only, both through delisted_on)

### V-14-08 7.9 access event of the name search   (source: block 7.9, phase-14.md:1356; result: GAPS 1)
- T1 SQL: PASS - audit.access_event (phase-02.md:148-156): resource_type regex accepts "screening_search"; action CHECK includes search_hit (phase-02.md:151); tier includes restricted (phase-02.md:153); subject_party nullable (phase-02.md:150); decision "deny" allowed (phase-02.md:156). Setting screening.search_per_min (phase-14.md:529).
- T2 Route: PASS - "POST /screening/search | screening.work" (phase-14.md:829, screening.search); Cedar SS and M permits at phase-14.md:1088 and 1101.
- T3 Repository and types: PASS - SearchRequest (phase-14.md:692), SearchResponse (phase-14.md:810), AccessScope (phase-02.md:719). IdentifierInput is undefined: systemic, V-14-03-a.
- T4 Logic: PASS - 3.12 steps 1 to 5 (phase-14.md:1151-1155): 422 E_PURPOSE_REQUIRED (1151; 3.14 at 1182); 429 E_RATE_LIMITED with Retry-After (1152; 3.14 at 1184); 503 E_UNAVAILABLE for an unloaded index (3.14 at 1185); at most 50 candidates (phase-14.md:692 and 1154).
- T5 Audit: PASS - screening.searched on every search (phase-14.md:1155 and 1168); the access event is written before the body (phase-14.md:1361).
- T6 Cross-references: GAP V-14-08-a. PASS for DR-46 (docs/plan/research.md:57), DR-383 (research.md:394), the GCRA limiter (phase-01.md:573) and the access-event action (phase-02.md:151).
- T7 Consistency: PASS - the deny access event of 3.12 step 1 (phase-14.md:1151) matches phase-02.md:723 ("written as a deny access event first").
- Gaps:
  - GAP V-14-08-a | MEDIUM | T6 | The route carries the marker x-akr-access: screening_search, but phase 02 requires the operation to be listed in api::access::SENSITIVE with a fixture request, and an extension without a fixture fails phase 02's first coverage test; neither the block nor section 4.5 adds the entry | block 7.9 (phase-14.md:1362) and section 4.5 (phase-14.md:1240-1251) | `grep -n "SENSITIVE" phase-14.md` (none); `grep -n "fixture" phase-14.md` (1275, a curl fixture only); `grep -n "SENSITIVE" research.md` (none); phase-02.md:735-741 states the rule

### V-14-09 7.10 one-hour re-screen after a list activation   (source: block 7.10, phase-14.md:1367; result: GAPS 2)
- T1 SQL: PASS - alert.screening_runs kind 'lookback' (phase-14.md:225), status 'completed' (226), trigger_ref (230), finished_at (243); ref.list_versions.activated_at (phase-14.md:116).
- T2 Route: N/A - the block adds "No table, column, job, timer or route" (phase-14.md:1370).
- T3 Repository and types: GAP V-14-09-b (S3 field has no response type or route).
- T4 Logic: GAP V-14-09-a (once-per-version rule). Steps 1, 2 and 4 PASS: the look-back run and its time (phase-14.md:1371-1374), the histogram and payload (phase-14.md:1375-1377), RESCREEN_TARGET as a constant (phase-14.md:1384-1385).
- T5 Audit: PASS - list.rescreen_late named in step 3 (phase-14.md:1380) and added to 3.13 (phase-14.md:1388); screening.run_finished gains rescreen_seconds (phase-14.md:1377); gauge akr_list_rescreen_late (phase-14.md:1381).
- T6 Cross-references: PASS - list_stale_sweep is a job of 4.1 (phase-14.md:1205) on schedule.list_stale_sweep "0 */10 * * * *" (phase-14.md:553). The ROADMAP exit criterion at docs/ROADMAP.md:246 ("At most one hour from a list update to a full re-screen") matches the block.
- T7 Consistency: PASS - "The 12-hour freshness rule of section 3.2 is unchanged" (phase-14.md:1369) agrees with the freshness rule at phase-14.md:937.
- Gaps:
  - GAP V-14-09-a | MEDIUM | T4 | "Once per version" for list.rescreen_late has no mechanism (no stored flag, no query); "the same way list.stale is written once per transition" points to a mechanism that the base text (phase-14.md:937) does not give either | block 7.10 step 3 (phase-14.md:1380-1381) | `grep -n "once per" phase-14.md` (937, 1380 and 1381 are the rule; 973 and 990 are unrelated); the ref.list_versions DDL (phase-14.md:92-128) has no stale or rescreen column; research.md has no DR on the once-per rule
  - GAP V-14-09-b | MEDIUM | T3 | The S3 re-screen time has no response type or route: ListRecordView and ListVersionView are named (phase-14.md:810) but not defined, and the routes at phase-14.md:818-820 name no re-screen field; DR-404 (research.md:415) allows no new route, so an existing response must carry it | block 7.10 step 5 (phase-14.md:1386-1387) and section 2.7 | `grep -n "rescreen" phase-14.md` (7.10 lines 1376-1388 only); `grep -n "ListVersionView" phase-14.md` (810 only)

### V-14-10 7.12 superseded alerts free their fingerprint   (source: block 7.12, "Adversarial pass (2026-10-10), run 2" header at phase-14.md:1401; result: PASS)
- T1 SQL: PASS - exact DDL in the block: column without UNIQUE and "CREATE UNIQUE INDEX IF NOT EXISTS uq_screening_alerts_fingerprint ON alert.screening_alerts (fingerprint) WHERE superseded_at IS NULL" (phase-14.md:1404-1405). The base 1.2 still says "fingerprint BYTEA NOT NULL UNIQUE" (phase-14.md:260); the block names migration 0046 and drops UNIQUE, so it wins (see T7).
- T2 Route: N/A - no route.
- T3 Repository and types: N/A - no repository type of its own; the method it changes is V-14-11.
- T4 Logic: PASS - 3.5 step 4 supersedes undecided alerts (phase-14.md:999); the block's rerun rules and test "N1, then N2, then N1 again leaves one open alert" (phase-14.md:1408-1411) follow from steps 2 and 4 (phase-14.md:997-999).
- T5 Audit: PASS - screening.alert_created for inserts and screening.alert_superseded for supersedes (phase-14.md:1165 and 998-999).
- T6 Cross-references: PASS - superseded_at and superseded_by columns (phase-14.md:277-278).
- T7 Consistency: PASS with notes. The block says the column is "without UNIQUE" in 0046 and that AlertRepository::insert_candidates uses the WHERE form (phase-14.md:1405-1407), so the block replaces the base text. Stale base text is left at phase-14.md:260 (UNIQUE), phase-14.md:721 and phase-14.md:997 (ON CONFLICT (fingerprint) with no WHERE), and phase-14.md:1287 (section 6 check 7, the unfiltered duplicate query that the block says is filtered at phase-14.md:1409-1410).
- Gaps: none.

### V-14-11 insert_candidates ON CONFLICT rule (7.12)   (source: block 7.12, phase-14.md:1406-1407, method at phase-14.md:722; result: PASS)
- T1 SQL: PASS - fingerprint (phase-14.md:260), superseded_at (278), decided_at (285) exist on alert.screening_alerts; the partial unique index is in the block (phase-14.md:1405).
- T2 Route: N/A - an internal repository method (called by the screening run and by HoldSweepService).
- T3 Repository and types: PASS - AlertRepository::insert_candidates(tx, rows) -> Result<Vec<Uuid>, RepoError> (phase-14.md:722); NewAlert is undefined: systemic, V-14-03-a.
- T4 Logic: PASS - 3.5 step 1 suppression first, step 2 fingerprint and the insert call, step 3 one transaction for each returned id with priority, clock and audit (phase-14.md:995-998).
- T5 Audit: PASS - screening.alert_created for each returned id (phase-14.md:998 and 1165).
- T6 Cross-references: PASS - TimerService::schedule (phase-00.md:561); BusinessClock::add (phase-09.md:459) and BusinessUnit with Hours (phase-09.md:434); screening.priority_map setting (phase-14.md:541).
- T7 Consistency: PASS with notes. The block (phase-14.md:1406-1407) gives the WHERE conflict clause and replaces "ON CONFLICT (fingerprint) DO NOTHING" at phase-14.md:721 and phase-14.md:997; the block wins.
- Gaps: none.

### V-14-12 7.13 a superseded alert is refused in SQL (guard alert.screening_state_guard)   (source: block 7.13, "Adversarial pass (2026-10-10), run 2" header at phase-14.md:1401; result: PASS)
- T1 SQL: PASS - guard function body at phase-14.md:490-496 (migration 0048) and trigger at phase-14.md:506-507; the block adds the superseded IF to that body (phase-14.md:1414-1419). The trigger is "BEFORE UPDATE OF state", so the decided_at clause fires only when state is also set; apply_state always sets state (phase-14.md:1027), so the service path is covered. Grants for state, decided_at, superseded_at and version (phase-14.md:438-439).
- T2 Route: N/A - no route; the decision route is at phase-14.md:833.
- T3 Repository and types: PASS - AlertRepository::apply_state(tx, id, expected, to: AlertState, decided) (phase-14.md:727) and supersede(tx, old, by) (phase-14.md:728).
- T4 Logic: PASS - decide step 2 is If-Match (phase-14.md:1022) and step 3 applies the transition table (phase-14.md:1023); the table gives 409 E_NOT_ALLOWED_IN_STATE for a superseded alert (phase-14.md:1016); the block says decide step 3 reads superseded_at from the step 2 row (phase-14.md:1415-1416). The 409 E_VERSION_CONFLICT for a stale version is the step 2 path, so both refusals have a code.
- T5 Audit: PASS - screening.alert_decided on each completed decision (phase-14.md:1031); refused decisions write no mutation.
- T6 Cross-references: N/A - the block names no name owned by another phase.
- T7 Consistency: PASS - the table row (phase-14.md:1016) says the terminal guard enforces the first two; the block adds superseded (phase-14.md:1416-1419). The two rules agree.
- Gaps: none.

## Summary

- Items checked: 12 (5 header items, 4 gap-pass blocks, 3 adversarial run 2 items).
- Items fully PASS: 4 (V-14-06 7.4 hold alerts; V-14-10 7.12 index; V-14-11 7.12 insert_candidates; V-14-12 7.13 guard). Items with GAP lines: 8 (V-14-01, 02, 03, 04, 05, 07, 08, 09).
- GAP lines: 15 in total. HIGH 0, MEDIUM 13, LOW 2.
- Systemic notes (counted once): V-14-03-a covers the undefined row and argument types used by items V-14-01, 02, 04, 06, 07, 08 and 11 (T3 references only, not counted again). Section 3 of phase 14 has no method signatures for AlertService, PairMemoryService, GoodGuyService and ScreeningService (the template asks for them), which is the cause of V-14-01-a.

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-14-01-a | MEDIUM | T3 | Bulk-decide service method has no name or parameters | Section 3.6 (Bulk incorrect) and 2.6 |
| V-14-01-b | MEDIUM | T4 | Refusal status and code for a bulk decision on a freeze list | Section 3.6 (Bulk incorrect) and 3.14 |
| V-14-01-c | MEDIUM | T4 | Good-guy freeze refusal: Cedar 403 versus service 422, no order of checks, deny reason freeze_list missing from 3.14 | Sections 3.8, 3.11 and 3.14 |
| V-14-01-d | LOW | T6 | Step-up constant StepUpAction::Decision not declared by phase 01 or phase 14 | Section 3.6 step 1 |
| V-14-02-a | MEDIUM | T4 | Parse error (step 3) and count mismatch (step 4) have no failure outcome | Section 3.2, list_parse steps 3 and 4 |
| V-14-02-b | MEDIUM | T5 | The incident raised on a parse failure has no kind, alarm or audit event | Section 3.2 list_parse step 2 and section 3.13 |
| V-14-03-a | MEDIUM | T3 | Row and argument types used by the traits, ports and DTOs are undefined (NewListVersion, NewEntry, VersionChange, DiffCounts and others) | Sections 2.3, 2.4 and 2.5 |
| V-14-04-a | MEDIUM | T7 | Header says "never on freeze lists" for pair memory; DR-102 and the base text exclude freeze lists only for good-guy rules and bulk | Header line 10 versus sections 3.7 and 7.3 and DR-102 |
| V-14-04-b | MEDIUM | T2 | No route or request field for the good-guy revoke proposal | Sections 2.3, 2.8 and 3.8 |
| V-14-05-a | MEDIUM | T4 | Labelled pair and MLRO corpus acceptance have no record, query, route or steps | Section 3.9 (Feedback) and section 2.8 |
| V-14-05-b | LOW | T6 | Audit event named setting.changed; phase 00 names setting.updated | Sections 3.10 and 3.13 |
| V-14-07-a | MEDIUM | T4 | No source mapping for listed_on or delisted_on in the parse | Section 3.2 list_parse step 6 and block 7.6 |
| V-14-08-a | MEDIUM | T6 | x-akr-access marker without the phase 02 SENSITIVE registry entry and fixture | Block 7.9 and section 4.5 |
| V-14-09-a | MEDIUM | T4 | "Once per version" for list.rescreen_late has no mechanism | Block 7.10 step 3 |
| V-14-09-b | MEDIUM | T3 | S3 re-screen time has no response type or route | Block 7.10 step 5 and section 2.7 |
