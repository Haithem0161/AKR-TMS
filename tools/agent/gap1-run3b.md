# Gap pass 1, run 3b (targets 11 to 16 and the unowned API rows; Haiku draft for lead review)


## Phase 11 (target)

### G1-11-01 Tombstone check reaches phase 11 through the shared writer
- Severity: LOW
- Category: Missing Logic
- Target: phase-11.md section 3.3 (mapping pending and the `CanonicalWriter` call)
- Sources: phase-23.md:930 "Phase 10 and phase 11 canonical writers (akr-ingestd)"; phase-11.md:796 "`CanonicalWriter::write(tx, &mapped)` (phase 10 section 3.7)"; phase-11.md:1507 "no change to the `CanonicalWriter`"
- Check: grep -n "tombstone" docs/plan/phase-11.md -> none; phase 11 inserts canonical rows only through `CanonicalWriter` (lines 796, 827); the check itself is in phase 10 section 7.4, so the shared writer already covers phase 11.
- Proposed 7.2 text:
  ```
  ### 7.2 Tombstone check (phase 23)
  Phase 11 writes canonical rows only through phase 10's `CanonicalWriter::write`. The tombstone check of phase 10
  section 7.4 (from phase 23 on) therefore applies to every landing and every replay of this phase without change here.
  This phase adds no code and no DDL for it.
  ```

### G1-11-02 Quarantine `gate` attribute and the repository methods owned by phase 12
- Severity: LOW
- Category: Missing Fields
- Target: phase-11.md section 2 (`QuarantineRepository`, `QuarantineFilter`, the quarantine view)
- Sources: phase-12.md:987 "the phase 11 entity `Quarantine` gains `gate: String`"; phase-12.md:531 "Phase 11's QuarantineRepository (section 2 there) gains two methods here"; phase-12.md:577 "`QuarantineView` (id, source table, reason code, gate, status, owner"
- Check: grep -n "gate" docs/plan/phase-11.md -> no quarantine attribute; grep -n "accept_conflicts\|fn assign" docs/plan/phase-11.md -> none. The route, the two filters and the two exclusion refusals are already named in phase 11 section 4.4 (line 1389), so they are skipped.
- Proposed 7.2 text:
  ```
  ### 7.2 Quarantine gate attribute and repository methods owned by phase 12
  The quarantine view derives a `gate` word from the reason code: a row whose reason is an unmapped transaction type is a
  gate row (phase 12 section 3.11 shows it as "Blocks until mapped"); every other row has no gate. The Cedar policies of
  phase 12 read `gate` on the entity `Quarantine` (phase 12 section 3.12). Phase 12 adds to `QuarantineRepository` the
  methods `assign` and `accept_conflicts` and the filters `owner_id` and `older_than_days` to `QuarantineFilter`. This
  phase implements none of them; section 4.4 already names the route and refusals.
  ```

## Phase 12 (target)

### G1-12-01 Reconciliation for a restore gap while the restore replay runs
- Severity: LOW
- Category: Missing Logic
- Target: phase-12.md section 3.1 (the `recon_run` job, line 651, claim step)
- Sources: phase-30.md:615 "Phase 12 owns recon_run; its owner confirms that runs for a restore gap are valid while the replay is running"; phase-30.md:494 "restore verify queues phase 12 recon_run jobs for the gap"
- Check: grep -n -i "restore" docs/plan/phase-12.md -> no restore wording in section 3 (line 1347 is an exclusion of scope only); the claim step of `recon_run` (line 652) has no replay condition.
- Proposed 7.2 text:
  ```
  ### 7.2 Recon runs for a restore gap
  A `recon_run` whose business dates overlap a restore replay (phase 30 section 3.9) does not claim its run while that
  replay job is active. The job returns to the queue and writes no result until the replay completes; the ladder then
  reads the restored rows. A run for a date outside the replay window is unaffected.
  ```

## Phase 13 (target)

### G1-13-01 Compiler and comparator entry points named by phase 16
- Severity: LOW
- Category: Missing Logic
- Target: phase-13.md section 3.1 (`FeatureCompiler`) and section 3.6 (`ProfileComparator`)
- Sources: phase-16.md:2030 "Phase 13 gains two public entry points, `FeatureCompiler::compile_measure` and `compile_filter`"; phase-16.md:2031 "`ProfileComparator::plan` and `evaluate` become public"; phase-13.md:552 "`FeatureCompiler::compile(def, as_of)` is pure"
- Check: grep -n "compile_measure\|compile_filter" docs/plan/phase-13.md -> none; grep -n "ProfileComparator" docs/plan/phase-13.md -> module line only (424, 426), no public method list. Phase 16 says the additions are "made by this phase", so phase 16 owns the code.
- Proposed 7.2 text:
  ```
  ### 7.2 Public entry points added by phase 16
  Phase 16 (section 3.2 and its Decisions) adds to this phase's module, without redefining anything here: the public
  `FeatureCompiler::compile_measure` and `FeatureCompiler::compile_filter` (the scenario-side compilers), and makes
  `ProfileComparator::plan` and `ProfileComparator::evaluate` public. `FeatureCompiler::compile(def, as_of)` and the
  behaviour of this phase are unchanged.
  ```

### G1-13-02 Re-measure of the volume effect at decision
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-13.md section 3.7 (common rules, `check_before_decision`)
- Sources: phase-16.md:2060 "Phase 13 must call `ReaderCatalogue::measure` again in its `check_before_decision` (its section 3.7 does not say so)"; phase-16.md:1450 "Phase 13's `prepare` and `check_before_decision` call it again"; phase-13.md:774 "`check_before_decision` runs the validator again"
- Check: grep -n "measure" docs/plan/phase-13.md -> lines 645, 778, 787, 1043 only; the decision-time step of section 3.7 (line 774) runs the validator, not `ReaderCatalogue::measure`. Phase 04 DR-360 already refuses a changed measurement at decision (`volume_effect_changed`).
- Proposed 7.3 text:
  ```
  ### 7.3 Re-measure at decision
  `check_before_decision` (section 3.7, common rules) also calls `ReaderCatalogue::measure(object, proposal)` again for
  every kind whose volume effect is measured. When the new effect differs from the one stored at `prepare`, the decision
  refuses with `volume_effect_changed` (phase 04, DR-360) and the maker resubmits. A change whose effect is `unknown`
  stays not approvable. The recompute entry point for a waiting request belongs to phase 04 (phase 16 Open doubts 12).
  ```

### G1-13-03 Tier of the profile read once the customer kind is masked
- Severity: LOW
- Category: Missing Logic
- Target: phase-13.md section 3.6 (the profile read) and section 7.1 (the access event of the profile read)
- Sources: phase-17.md:1907 "Masked readers of phases 13 to 16 ... must read `casework.effective_tier(party_id)`"; phase-13.md:1028 "The profile read's access event"; phase-15.md:1421 "The `customer` kind becomes masked."
- Check: grep -n "effective_tier" docs/plan/phase-13.md -> none. The profile read's tier is fixed at restricted in section 7.1; the masking of the customer kind is added by phase 15, which owns the tier source (G1-15-01).
- Proposed 7.4 text:
  ```
  ### 7.4 Tier of the profile read
  Once phase 15 masks the customer kind, the profile read takes its tier from the customer adapter, which reads
  `casework.effective_tier(party_id)` (phase 15 section 3.6, phase 17 Open doubts 6). The access event of section 7.1
  keeps the tier `restricted` for the event row itself. No route or table changes here.
  ```

## Phase 14 (target)

### G1-14-01 Case link after a party screening decision
- Severity: HIGH
- Category: Missing Integration
- Target: phase-14.md section 3.6, decision step 8 (the freeze port and the case hook)
- Sources: phase-17.md:1061 "`link_screening(tx, screening_alert_id, actor) -> CaseId`. Phase 14's decision service calls it"; phase-18.md:1127 "Phase 14 decision service: calls `ScreeningCaseLinker::link_screening`"; phase-14.md:1025 "Outcome `true` off freeze lists: state `true`; the case hook of phases 17 and 18 reads it"; phase-18.md:1278 "DR-147" (counterparty and payment-party hits stay without a case)
- Check: grep -n "link_screening\|ScreeningCaseLinker" docs/plan/phase-14.md -> none. Phase 14 only names a case hook that reads state; no call is written.
- Proposed 7.2 text:
  ```
  ### 7.2 Case link after a party screening decision
  Decision step 8 of section 3.6 gains a sub-step in the same decision transaction: when the alert's subject kind is
  `party` and the outcome is `potential` or `true`, the decision service calls
  `ScreeningCaseLinker::link_screening(tx, screening_alert_id, actor)` (port defined in phase 17, implemented in phase 18).
  Counterparty and payment-party subjects are not linked (phase 18 DR-147). The decision state machine, the four-eyes
  rule and the freeze call of step 8 are unchanged.
  ```

### G1-14-02 Screening hit topic and the suppression records named by phase 19
- Severity: MEDIUM
- Category: Missing Event
- Target: phase-14.md section 4.2 (outbox) and section 3.7 (the pair-memory sweep and its QA sample)
- Sources: phase-14.md:1221 "Outbox: no topic is produced by this phase"; phase-19.md:1677 "outbox topics consumed ... `screening.hit` (phase 14 ...)"; phase-19.md:1873 "Phase 14 does not name the table of its suppression records"; phase-14.md:1038 "each Monday it draws a QA sample of 5 per cent of the week's suppressions"; phase-19.md:1712 "the pair-memory weekly sample ... is drawn by this phase's job `qa_suppression_sample`"; DSL.md:807 lists `screening.hit` among trigger events
- Check: grep -n "screening.hit" docs/plan/phase-14.md -> none; grep -n "suppress" docs/plan/phase-14.md -> counters on `alert.pair_memory` (374-375) and `alert.good_guy_rules` (399), and the sweep line 1038 (which draws the QA sample in phase 14).
- Proposed 7.3 text:
  ```
  ### 7.3 Outbox topic `screening.hit` and the suppression records
  (a) The decision transaction of section 3.6 writes the outbox topic `screening.hit` for a party or payment-party alert
  decided `potential` or `true`, with the payload `{alert_id, subject_kind, subject_id, list_kind, outcome}` (ids and
  codes only, no names). Section 4.2 lists the topic as produced by this phase; its consumers are phase 19 and the
  watch triggers of DSL.md C.5.
  (b) The suppression records are the rows of `alert.pair_memory` and `alert.good_guy_rules` with `suppressions > 0`
  (columns `suppressions` and `last_suppressed_at`); no other table holds them.
  (c) The weekly QA sample of suppressions is drawn by phase 19's job `qa_suppression_sample` (phase 19 section 4.1). The
  Monday step of the sweep in section 3.7 keeps the counters only and draws nothing.
  ```

### G1-14-03 Hold alerts for phase 29 (`ScreeningService::open_hold_alert` and its readers)
- Severity: HIGH
- Category: Missing Logic
- Target: phase-14.md section 3.5 (`ScreeningService`), section 1 DDL comments of `alert.screening_runs` (line 226-227), section 3.15 (subject view masking)
- Sources: phase-29.md:1346 "`ScreeningService::open_hold_alert(run, candidate)` (extends phase 14 with one caller: `HoldSweepService`)"; phase-29.md:1525 "the subject view of an alert whose run is a hold `payment` run ... reads the party names from `assess.holds.party_names_enc`"; phase-29.md:1525 "`screening_payment` skipping an entry already alerted on a hold with the same match key"; phase-14.md:226 "`subject_id` ... -- adhoc runs only"; phase-14.md:227 "`trigger_ref` ... -- list version (lookback), batch (onboarding)"
- Check: grep -n "open_hold_alert\|hold" docs/plan/phase-14.md -> no hold-alert method or wording; `alert.screening_runs.subject_id` comment says adhoc runs only; `trigger_ref` comment names no hold.
- Proposed 7.4 text:
  ```
  ### 7.4 Hold alerts for phase 29
  (a) `ScreeningService::open_hold_alert(run, candidate)` (section 3.5) is added with one caller, phase 29's
  `HoldSweepService`. It inserts one `alert.screening_runs` row of kind `payment` (`requested_by` NULL, `subject_kind`
  `payment_party`, `subject_id` the hold's subject reference, `trigger_ref` the hold id, status `completed`) and the
  candidate alert through `AlertRepository::insert_candidates`. It creates nothing else. The phase 14 `AlertService`,
  `FreezeTrigger` and masking are used unchanged.
  (b) The comment of `alert.screening_runs.subject_id` widens from "adhoc runs only" to "adhoc runs and hold payment runs".
  The comment of `trigger_ref` adds "payment runs: the hold id (phase 29)".
  (c) The subject view of an alert whose run is a hold `payment` run reads the party names from
  `assess.holds.party_names_enc` under the masking of section 3.15.
  (d) `screening_payment` (section 3.5, step 4) skips an entry already alerted on a hold with the same match key.
  ```

### G1-14-04 Risk-modifier port used by phase 15
- Severity: MEDIUM
- Category: Missing Integration
- Target: phase-14.md section 2.5 (ports) and section 3.4 (the scorer's threshold)
- Sources: phase-15.md:1558 "`RiskModifierSource::modifier(party)` returns the additive modifier"; phase-15.md:1861 "the risk-modifier hook implemented"; phase-14.md:1332 "Risk modifiers read phase 15 ratings; they are zero until phase 15."
- Check: grep -n "RiskModifier\|modifier" docs/plan/phase-14.md -> lines 977 (threshold wording), 1332 and 1347 only; no port is named; grep -n "rating_modifiers" docs/plan/phase-14.md -> none (phase 15 seeds it at phase-15.md:766).
- Proposed 7.5 text:
  ```
  ### 7.5 Risk-modifier port
  Section 2.5 gains the port `RiskModifierSource` with `modifier(party_id)`, which returns the additive modifier of the
  party's current rating level, or 0 when the party is unrated. The null implementation returns 0; phase 15 implements the
  port from `screening.rating_modifiers` (phase 15 section 3.8). The scorer (section 3.4) adds the modifier to the
  threshold of a party subject, within the bounds the thresholds already enforce (section 1 CHECK). Counterparty and
  payment-party subjects read no modifier.
  ```

### G1-14-05 Entries dropped from a list version keep a delisting record
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-14.md section 3.2 (activation, step 7 and the diff)
- Sources: phase-20.md:1106 "Absent from the list means delisted ... phase 14's validation ... sets `delisted_on` when a version omits an entry"; phase-20.md:680 "phase 14 `ref.list_entries.delisted_on`"; phase-14.md:144 "`delisted_on DATE`"
- Check: grep -n "delisted" docs/plan/phase-14.md -> the column (144) and its grant list (568) only; no activation step sets it. The count guard `screening.max_count_drop_pct` exists (phase-14.md:1226, governed setting).
- Proposed 7.6 text:
  ```
  ### 7.6 Delisting at activation
  When a version is activated (section 3.2, step 7), every entry of the previously active version whose `entry_key` is
  absent from the new version is recorded in the new version with `delisted_on` set to the activation date. A freeze-list
  gate (phase 20 section 3.5) therefore reads a row, not an absence. A version that drops entries above
  `screening.max_count_drop_pct` is refused by the existing count check, so a silent drop cannot activate.
  ```

### G1-14-06 An unchanged feed renews the freshness clock
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-14.md section 3.2 (freshness rule, `ListRecord::stale`) and section 1 DDL of `ref.list_versions` (migration 0045)
- Sources: phase-14.md:934 "`age_hours = now − active.activated_at` (or the last successful `loaded_at`...)"; phase-14.md:98 "`feed_unchanged`"; phase-14.md:938 "Call `ListFeedSource::fetch(list, prior_etag)`"; phase-29.md:1025 "Stale is the phase 14 rule `ListRecord::stale` unchanged"
- Check: grep -n "checked" docs/plan/phase-14.md -> no check timestamp; `ref.list_versions` (line 89) has `loaded_at` and `activated_at` only. An unchanged feed therefore never renews the clock.
- Proposed 7.7 text:
  ```
  ### 7.7 An unchanged feed renews freshness
  `ref.list_versions` gains `last_checked_at TIMESTAMPTZ` (in migration 0045, this phase's own DDL). A `list_fetch` that
  gets `feed_unchanged` sets it on the active version and writes no new version and no entry. `ListRecord::stale` uses
  `age_hours = now − max(activated_at, last_checked_at)` (or `loaded_at` for a `cbs_snapshot` list). The 12-hour cap of a
  freeze list is unchanged. This closes phase 29's open question on the freshness clock.
  ```

### G1-14-07 Retention class of freeze-list alerts (UNCLEAR)
- Severity: MEDIUM
- Category: Missing Policy
- Target: phase-14.md section 1.5 (retention rows) and its Open doubts (retention item)
- Sources: phase-14.md:1329 "Alerts are R7 ... ARCHITECTURE s.17 gives sanctions-tied records ten years under R5"; ARCHITECTURE.md:481 "R5 | Alerts and dispositions | ... (sanctions-tied 10)"; ARCHITECTURE.md:483 "R7 | Screening and sanctions records | 7 years"; phase-20.md:1081 "Retention is R7 (ARCHITECTURE section 17 gives R7 to screening and sanctions records)"
- Check: grep -n "R5" docs/plan/phase-14.md -> the open doubt only; section 1.5 gives `alert.screening_alerts` class R7 (line 625).
- UNCLEAR: ARCHITECTURE section 17 gives sanctions-tied alerts ten years under R5 and screening and sanctions records seven years under R7; phase 20 chose R7 and phase 14 keeps R7 with an open question. Two sources disagree. Proposed text: none. The bank decides the ten-year term for freeze-list alerts; the decision then goes to a DR.

## Phase 15 (target)

### G1-15-01 Masking reads the tier through casework.effective_tier
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-15.md section 3.6 (the `customer` kind becomes masked; the loaded `tier`)
- Sources: phase-15.md:1423 "the loaded attributes gain `tier` (from `canon.party.sensitivity_tier` of the customer's party"; phase-17.md:1907 "They read `canon.party.sensitivity_tier` only; they must read `casework.effective_tier(party_id)`"; phase-18.md:811 "whose tier now reads `casework.effective_tier` (phase 17 Open doubts 6)"
- Check: grep -n "effective_tier" docs/plan/phase-15.md -> none (only `effective_from` at lines 48, 120). The masking reads the ingest tier alone, which phase 17 says is wrong once casework raises a tier.
- Proposed 7.1 text:
  ```
  ### 7.1 Customer tier read through casework.effective_tier
  The `tier` loaded for the `customer` kind in section 3.6 is `casework.effective_tier(party_id)` (phase 17 Open doubts 6):
  the higher of `canon.party.sensitivity_tier` and `casework.party_tiers`. It is not `canon.party.sensitivity_tier` alone.
  Every masked read of the customer kind uses this value. The refusal, its access event and its decision row are
  unchanged.
  ```

### G1-15-02 Rating attributes registered in the attribute catalogue
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-15.md section 4.4 (existing updates)
- Sources: phase-06.md:991-992 "`rating.` (phase 15 adds `register_rating_attributes()`: `rating.level`, `rating.cbi_level`)"; phase-06.md:986 "### 7.3 The AttributeCatalogue type (DR-367)"
- Check: grep -n "register_rating_attributes\|rating.level\|cbi_level" docs/plan/phase-15.md -> the columns `cbi_level` (151, 169) only; no catalogue registration.
- Proposed 7.2 text:
  ```
  ### 7.2 Rating attributes in the attribute catalogue
  At start-up this phase calls `AttributeCatalogue::register_rating_attributes()` (phase 06 section 7.3, DR-367), which
  registers `rating.level` and `rating.cbi_level` in the `rating.` namespace. The phase 10 column test skips that
  namespace. The rating model and its columns are unchanged.
  ```

### G1-15-03 The exit_decision definition calls create_ops_task
- Severity: MEDIUM
- Category: Missing Integration
- Target: phase-15.md section 4.2 (shipped definitions)
- Sources: phase-20.md:972 "`create_ops_task` is registered here and called by `exit_decision` (DSL E.4)"; phase-20.md:957 "the `effects` registry gains ... `create_ops_task` (resource kind `exit_decision`)"; phase-15.md:1080 "Record an exit from a completed `exit_decision` instance"
- Check: grep -n "exit_decision" docs/plan/phase-15.md -> lines 476 (a table comment) and 1080 (the route) only; no definition text names an effect; grep -n "create_ops_task" docs/plan/phase-15.md -> none.
- Proposed 7.3 text:
  ```
  ### 7.3 exit_decision definition
  The shipped definition of `exit_decision` (DSL.md E.4) calls the effect `create_ops_task` (registered by phase 20,
  resource kind `exit_decision`) when the instance decides an exit, so that the exit has its operations task. The
  definition's other steps are as DSL.md E.4 states. `POST /exit-register` (section 2 route table) records the exit from a
  completed instance and is unchanged.
  ```

### G1-15-04 Producers of the topics this phase consumes
- Severity: LOW
- Category: Missing Event
- Target: phase-15.md section 4.2 (outbox topics consumed)
- Sources: phase-15.md:1786 "Outbox topics consumed: `canon.touched` (phase 11, section 3.3)"; phase-18.md:1103 "`case.decision` ... whose consumer is phase 15"; phase-19.md:1711 "the outbox topics `authority_request.recorded`, `internal_report.recorded` and `graph.link` are produced by this phase"
- Check: grep -n "producing\|produced by" docs/plan/phase-15.md -> none for the consumed topics; phase 15 names the producers of `canon.touched` only.
- Proposed 7.4 text:
  ```
  ### 7.4 Producers of the consumed topics
  Producers of the topics this phase consumes: `canon.touched` (phase 11), `case.decision` (phase 18, `RecordDecisionEffect`
  and the escalation), `authority_request.recorded`, `internal_report.recorded` and `graph.link` (phase 19 section 4.2),
  and `screening.hit` (phase 14 section 4.2 after gap pass 1). This phase registers the consumers only.
  ```

### G1-15-05 Requirement IDs K-12 to K-41 with no owner (UNCLEAR)
- Severity: LOW
- Category: Incomplete Coverage
- Target: phase-15.md section 7+ (owner decision) and docs/MODULES.md (the K1-K4 line)
- Sources: phase-15.md:2095 "K-12 (authority-request register), K-19 and K-20 ... have no phase yet. Gap pass 1 assigns them."; docs/MODULES.md K1-K4 line "K-12, K-19, K-20, K-23, K-25, K-31, K-32, K-34, K-36 and K-38 to K-41 await later phases"
- Check: grep -l "K-12" docs/plan/phase-*.md -> phase-15.md only; the same for K-19, K-20, K-23, K-25, K-31, K-32, K-34, K-36, K-38, K-41; K-39 and K-40 appear in no phase file.
- UNCLEAR: the owners are not decided, and the wording of each requirement lies in the research files, which this run did not read. Likely candidates from the phase text: K-12 near phase 19 (`authority_request.recorded`), K-23 near phase 24 (the PEP KPI pack). Proposed text: none until the lead assigns the IDs.

## Phase 16 (target)

### G1-16-01 Replay, sample and packet reads use casework.effective_tier
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-16.md section 3.9 (reading a replay, per-entity results), section 3.10 (blind labels and packets), and the Decisions entry at phase-16.md:2024
- Sources: phase-16.md:1301 "The tier is read at request time from the party (`canon.party.sensitivity_tier`, phase 10)"; phase-16.md:1326 "(`canon.party.sensitivity_tier`, phase 10); a suspicion-tier sample and its packet are M's alone"; phase-17.md:1907 "Masked readers of phases 13 to 16 ... must read `casework.effective_tier(party_id)`"
- Check: grep -n "effective_tier" docs/plan/phase-16.md -> none; grep -n "sensitivity_tier" docs/plan/phase-16.md -> lines 1302, 1326, 2024.
- Proposed 7.1 text:
  ```
  ### 7.1 Tier read through casework.effective_tier
  Every request-time tier read of this phase (the per-entity replay results of section 3.9, the sample labels and packets
  of section 3.10, and the tier rule of the Decisions) reads `casework.effective_tier(party_id)` (phase 17 Open doubts 6),
  the higher of `canon.party.sensitivity_tier` and `casework.party_tiers`. The tier stays read at request time and is never
  stored on the row.
  ```

### G1-16-02 Islamic pack hooks in the pack seed, G9 and verify-data
- Severity: MEDIUM
- Category: Missing Policy
- Target: phase-16.md section 3.4 (the G9 row of the gate table, line 1075) and section 3.15 (the pack seed and `packs verify-data`, line 1529)
- Sources: phase-26.md:495 "Gate G9 (phase 16 section 3.4) reads `pack: akr` or `pack: islamic`"; phase-26.md:491 "Phase 16 pack seed (section 3.15) is extended: a document with `requires_flag` is skipped while that flag is off"; phase-26.md:553 "`packs verify-data` (phase 16, extended) refuses an `ISL` document whose logic reads"; phase-16.md:1075 "G9 pack | `pack: akr` only with `pack.akr_live` on"
- Check: grep -n "requires_flag\|pack: islamic\|pack.islamic" docs/plan/phase-16.md -> none; G9 row reads `pack: akr` only (line 1075).
- Proposed 7.2 text:
  ```
  ### 7.2 Islamic pack hooks (phase 26 owns the content)
  (a) The G9 row of section 3.4 reads `pack: akr` (flag `pack.akr_live`) or `pack: islamic` (flag `pack.islamic`). A
  go-live with `pack.islamic` off is refused `pack_not_enabled`.
  (b) The pack seed of section 3.15 skips a document whose `requires_flag` is off (phase 26 section 3.7, step 2).
  (c) `akr-ctl packs verify-data` (section 3.15) refuses an `ISL` document that has no further condition beyond its
  basis (phase 26 section 3.10, item 2). The Islamic documents and their flags are phase 26's; this phase adds no content.
  ```

### G1-16-03 Protected-attribute check in the go-live gate
- Severity: MEDIUM
- Category: Missing Integration
- Target: phase-16.md section 3.4 (`GoLiveGate::check`, the conditions it returns)
- Sources: phase-24.md:1308 "The preventive gate belongs to phase 16's scenario change target, which must call the check in `check_before_decision`"; phase-24.md:667 "`ProtectedAttributeCheck` (pure function in `domain::governance`; input is a scenario's condition tree (phase 16 DSL"; phase-24.md:1170 "Phase 16 the scenario change target's `check_before_decision` gains the `ProtectedAttributeCheck`"
- Check: grep -n "ProtectedAttributeCheck\|protected" docs/plan/phase-16.md -> none; section 3.4 lists no protected-attribute condition.
- Proposed 7.3 text:
  ```
  ### 7.3 Protected-attribute condition in the go-live gate
  `GoLiveGate::check` (section 3.4) gains the condition `protected_attribute`: it runs `ProtectedAttributeCheck` (phase 24
  section 3.5, a pure function in `domain::governance`) on the version's condition tree and returns each refusal it gives.
  The condition blocks at `check_before_decision` and at activation, and reports at `prepare`, as the other conditions do.
  ```

### G1-16-04 Observation source for hybrid alerts and the observational seed
- Severity: MEDIUM
- Category: Missing Fields
- Target: phase-16.md section 2 (`AlertSink` port and `DetectionPayload`, line 833) and section 3.15 (bank scenarios and the seed)
- Sources: phase-22.md:818 "The payload gains `source_kind` with the values `scenario` (today) and `observation`"; phase-22.md:818 "The observational scenario named by `branch.hybrid_scenario_code` is seeded by the phase 16 scenario seed with family `observation`"; phase-17.md:1917 "a phase 16 payload extension, gap pass 1"
- Check: grep -n "source_kind" docs/plan/phase-16.md -> none; grep -n "observation" docs/plan/phase-16.md -> no family value `observation` in the seed.
- Proposed 7.4 text:
  ```
  ### 7.4 Source kind of a detection, and the observational seed
  `DetectionPayload` (section 2) gains `source_kind` with the values `scenario` (every scenario detection, the default)
  and `observation` (written by phase 22 section 3.3.1 only). This phase's runner always writes `scenario`. The scenario
  seed of section 3.15 carries the observational scenario named by the bank's `branch.hybrid_scenario_code` with family
  `observation`; the code is a bank setting (phase 22 Open doubts 2), so the seed row is added when the bank sets it.
  ```

### G1-16-05 Sink replacement and the detection audit kinds
- Severity: LOW
- Category: Missing Audit Event
- Target: phase-16.md section 3.7 (`RecordingAlertSink`) and section 3.17 (audit events, line 1600)
- Sources: phase-17.md:1573 "`RecordingAlertSink` is replaced in AppState; ... the detection audit kinds move from the phase 16 sink to `AlertIntakeService`"; phase-16.md:830 "Phase 16 ships RecordingAlertSink; phase 17 replaces it in AppState"; phase-16.md:1600 "`detection.recorded`, `detection.amended` | `RecordingAlertSink`"
- Check: grep -n "replaces it\|replaced" docs/plan/phase-16.md -> line 830 only; the audit table (1600) names the phase 16 sink as writer.
- Proposed 7.5 text:
  ```
  ### 7.5 Writer of the detection audit kinds after phase 17
  From phase 17, `AlertIntakeService` replaces `RecordingAlertSink` in AppState and writes the audit kinds
  `detection.recorded` and `detection.amended` (the table of section 3.17 keeps the phase 16 sink as writer until then).
  The keys and payloads are unchanged.
  ```

### G1-16-06 Activation instant column named
- Severity: LOW
- Category: Incomplete Coverage
- Target: phase-16.md section 1 DDL (`scn.scenario_versions`, line 81)
- Sources: phase-24.md:1325 "The scenario activation instant. ... Phase 16 names the column in its own DDL; the name is confirmed in gap pass 1."; phase-24.md:658 "the scenario's activation instant (phase 16 column, confirmed in gap pass 1, Open doubts 11)"
- Check: grep -n "effective_from" docs/plan/phase-16.md -> lines 81 (`scn.scenario_versions`), 92 (CHECK: live needs `effective_from`), 124.
- Proposed 7.6 text:
  ```
  ### 7.6 Activation instant
  The activation instant of a scenario version is `scn.scenario_versions.effective_from` (section 1 DDL), set in the
  activation transaction of section 3.3. Phase 24's review calendar reads this column for the S-46 rows. No DDL change.
  ```

### G1-16-07 Keys of the run cost JSON for standby reads
- Severity: LOW
- Category: Missing Fields
- Target: phase-16.md section 1 DDL comment of `scn.scenario_runs.cost` (line 191) and section 3.6 (the scenario job)
- Sources: phase-30.md:489 "`scn.scenario_runs.cost` gains the keys `read_source` and `replay_lag_s` (JSONB, no DDL)"; phase-30.md:621 "Phase 16's owner confirms that `scn.scenario_runs.cost` takes the two keys"; phase-16.md:191 "cost JSONB NOT NULL DEFAULT '{}'"
- Check: grep -n "read_source\|replay_lag_s" docs/plan/phase-16.md -> none.
- Proposed 7.7 text:
  ```
  ### 7.7 Cost keys of a scenario run
  `scn.scenario_runs.cost` (JSONB, no DDL change) carries the keys `read_source` (the node or pool the run read) and
  `replay_lag_s` (the standby lag in seconds) when the scenario job reads a standby (phase 30 DR-327). No other key changes.
  ```

## PART B: API.md rows that no phase route table owns

Each row was read in docs/API.md at the line given in tools/agent/gap1-routes.txt. "Present" means a phase route row with the same method and path already exists, so the routecheck tool missed it; no new route is drafted.

### G1-API-01 GET /cases/{}/acl (API.md line 206)
- Severity: MEDIUM
- Category: Incomplete Coverage
- Target: docs/API.md section 3.3 (line 206): change the path and scope to phase 18's row
- Sources: phase-18.md:596 "GET `/api/v1/cases/{id}/access` | `queue.work` | The case access list with reasons and grant times"; docs/API.md:206 "GET | /cases/{}/acl | case.decide"
- Check: grep -n "/acl" docs/plan/phase-18.md -> none; grep -n "/access" docs/plan/phase-18.md -> lines 596-598, the same operation under another path.
- Proposed text: API.md row becomes `GET | /cases/{id}/access | queue.work | The case access list with reasons and grant times (phase 18; Cedar case.access.read)`. The scope differs (API.md says case.decide; phase 18 says queue.work); the lead picks the scope. No new phase route.

### G1-API-02 POST /cases/{}/acl (API.md line 207)
- Severity: MEDIUM
- Category: Incomplete Coverage
- Target: docs/API.md section 3.3 (line 207)
- Sources: phase-18.md:597 "POST `/api/v1/cases/{id}/access` | `case.decide` | Grant read, work or decide to a person; `If-Match`"; docs/API.md:207 "POST | /cases/{}/acl | case.decide"
- Check: grep -n "POST.*/acl" docs/plan/phase-18.md -> none; the same operation is at `/cases/{id}/access` (line 597).
- Proposed text: API.md row becomes `POST | /cases/{id}/access | case.decide | Grant read, work or decide to a person; If-Match; Idempotency-Key (phase 18; Cedar case.access.change)`. The step-up marker (SU) of API.md is not in phase 18's row; the lead decides whether it stays.

### G1-API-03 DELETE /cases/{}/acl/{principal_id} (API.md line 208)
- Severity: MEDIUM
- Category: Incomplete Coverage
- Target: docs/API.md section 3.3 (line 208)
- Sources: phase-18.md:598 "DELETE `/api/v1/cases/{id}/access/{principal_id}` | `case.decide` | Revoke the grant"; docs/API.md:208 "DELETE | /cases/{}/acl/{principal_id} | case.decide"
- Check: grep -n "DELETE.*access" docs/plan/phase-18.md -> line 598 only.
- Proposed text: API.md row becomes `DELETE | /cases/{id}/access/{principal_id} | case.decide | Revoke the grant; refused for the current assignee; If-Match (phase 18; Cedar case.access.change)`.

### G1-API-04 GET /evidence/{} (API.md line 227)
- Severity: MEDIUM
- Category: Incomplete Coverage
- Target: docs/API.md section 3.4 (line 227): remove the row, or point it to the owner routes
- Sources: phase-26.md:373 "no generic `GET /evidence/{id}` exists; phase 18 serves content per owner"; phase-18.md:940 "`/evidence`, `/evidence/{id}/content`" (under the case path)
- Check: grep -n "GET.*/evidence/{id}\"\|GET | /evidence/" docs/plan/phase-*.md -> none; the content routes exist per owner (phase 18 under `/cases/{id}/evidence/{id}/content`, phase 26 under `/islamic/deals/{id}/evidence/{evidence_id}/content`).
- Proposed text: remove the API.md 3.4 row `GET /evidence/{}`. The lead records the decision as a DR (no generic evidence route; content is served by each owner's route). No phase route is added.

### G1-API-05 POST /sources/{}/backfills (API.md line 266) (UNCLEAR owner)
- Severity: MEDIUM
- Category: Incomplete Coverage
- Target: phase-12.md section 3.1 or phase-11.md section 3.5 (owner not decided)
- Sources: phase-11.md:1495 "API.md 3.5 says the resume of a source plans 'the backfill of the gap' from phase 11, while its backfill routes are phase 12; ... planned backfills stay in phase 12"; phase-12.md:1318 "neither this phase nor phase 11 (automatic catch-up, DR-90) builds an operator-planned backfill"
- Check: grep -n "backfills" docs/plan/phase-11.md docs/plan/phase-12.md -> phase 11 names the phase 12 owner (1495); phase 12 names no owner (1318).
- UNCLEAR: the two phases disagree. Phase 11 says the routes belong to phase 12; phase 12 says neither builds them. The lead decides the owner. Either phase 12 builds a plan job for explicit slices under the load budget (the load loop of phase 11 runs the slices), or the rows are dropped. Proposed text: none until the owner is decided.

### G1-API-06 GET /backfills/{} (API.md line 267) (UNCLEAR owner)
- Severity: MEDIUM
- Category: Incomplete Coverage
- Target: same owner decision as G1-API-05
- Sources: docs/API.md:267 "GET | /backfills/{} | data.steward | Plan and progress with checkpoints [owner open: gap pass 1]"; phase-12.md:1319 "owner or drops the rows"
- Check: grep -n "backfills/" docs/plan/phase-*.md -> none.
- UNCLEAR: same as G1-API-05. Proposed text: none until the owner is decided.

### G1-API-07 GET /ref/registers/{kind} (API.md line 369) (UNCLEAR owner and content)
- Severity: MEDIUM
- Category: Incomplete Coverage
- Target: docs/API.md section 3.8 (line 369); owner candidate phase-09.md (reference data, section 3)
- Sources: docs/API.md:369 "GET | /ref/registers/{}| config.author | Governed registers (dual-use goods, vessels, sector-goods map, flagged districts, crypto counterparties)"; phase-09.md:35-141 (its reference tables are countries, country risk overrides, governorates, currencies, fx rates, calendars and CBS calendar captures)
- Check: grep -n -i "dual-use\|flagged district\|crypto counterpart\|sector-goods" docs/plan/phase-*.md -> none; grep -n "registers" docs/plan/phase-09.md -> no route row. Phase 24's `/api/v1/registers/{kind}` is the governance register (decisions, exclusions, findings, warnings, fairness), a different resource under a different path.
- UNCLEAR: no phase defines the content of these registers or any table for them. The owner phase 09 is the lead's hint, but the data model is missing. Proposed text: none; the lead decides whether the registers are built (with a table and their content) or the rows are dropped.

### G1-API-08 POST /ref/registers/{kind}/entries (API.md line 370) (UNCLEAR owner and content)
- Severity: MEDIUM
- Category: Incomplete Coverage
- Target: same decision as G1-API-07
- Sources: docs/API.md:370 "POST | /ref/registers/{}/entries | config.author | Add or change entries; **CR**"; phase-24.md:553 "POST `/api/v1/registers/{kind}/entries` ... Add a finding (M, CH, MRO)" (a different resource)
- Check: grep -n "registers/{kind}/entries" docs/plan/phase-*.md -> phase 24 only, for findings; no reference-register entries anywhere.
- UNCLEAR: same as G1-API-07. Proposed text: none.

### G1-API-09 GET /ref/typologies (API.md line 371) (UNCLEAR owner)
- Severity: MEDIUM
- Category: Incomplete Coverage
- Target: phase-06.md section 3 (scenario register) or phase-16.md section 3.15; decision needed
- Sources: docs/API.md:371 "GET | /ref/typologies | self | Shared typology tag set"; phase-06.md:78 "`typologies SMALLINT[]` ... research file 27 B2 typology numbers"; phase-31.md:460 "GET /typology-exposure" (a different route)
- Check: grep -n "/ref/typologies\|typology tag" docs/plan/phase-*.md -> none; the typology numbers exist only as an array on scenarios.
- UNCLEAR: no phase holds a typology tag set table or file; the tag labels would come from research file 27 (not read in this run). Proposed text: none until the lead decides the source (a read of a catalogue file, or the scenario register's distinct values).

### G1-API-10 GET /samples/{}/packet (API.md line 471)
- Severity: LOW
- Category: Incomplete Coverage
- Target: docs/API.md section 3.11 (line 471): no change needed
- Sources: phase-16.md:895 "GET `/api/v1/samples/{id}/packet` (NEW) | queue.work | The blind packet"
- Check: grep -n "samples/{id}/packet" docs/plan/phase-16.md -> line 895 (route table of section 2).
- Present: owned by phase 16 section 2. Proposed text: none. The routecheck tool missed the row.

### G1-API-11 POST /scenarios/{}/retire-requests (API.md line 475)
- Severity: LOW
- Category: Incomplete Coverage
- Target: docs/API.md section 3.11 (line 475): no change needed
- Sources: phase-16.md:901 "POST `/api/v1/scenarios/{id}/retire-requests` (NEW) | config.author | Propose the retirement"
- Check: grep -n "retire-requests" docs/plan/phase-16.md -> line 901.
- Present: owned by phase 16 section 2. Proposed text: none.

### G1-API-12 GET /scenarios/{}/harness-diff (API.md line 478) (UNCLEAR owner)
- Severity: MEDIUM
- Category: Incomplete Coverage
- Target: phase-16.md section 2 (route table) or phase-24.md section 3.6 (validation packs); decision needed
- Sources: phase-16.md:2090 "`GET /scenarios/{id}/harness-diff`. A second implementation by a different author belongs with the validation packs of phase 24; the route row stays in API.md 3.11 with that target"; phase-24.md:1306 "Not carried: ... S-48 (harness diff, with the validation packs)"
- Check: grep -n "harness-diff\|harness diff" docs/plan/phase-24.md -> line 1306 (not carried) only; grep -n "harness-diff" docs/plan/phase-16.md -> lines 1843 and 2090, no route row in section 2.
- UNCLEAR: phase 16 says phase 24 owns the route; phase 24 says it does not carry S-48. Two sources disagree. Proposed text: none; the lead decides the owner (phase 24 with the validation packs, or phase 16 with the harness it already runs).

### G1-API-13 POST /exits (API.md line 635)
- Severity: MEDIUM
- Category: Incomplete Coverage
- Target: docs/API.md section 3.14 (line 635): replace the row
- Sources: phase-15.md:1080 "POST `/api/v1/exit-register` | `kyc.work` | Record an exit from a completed `exit_decision` instance"; phase-20.md:1101 "The row is superseded by phase 15's `POST /exit-register`. The lead removes it"
- Check: grep -n "/exits" docs/plan/phase-*.md -> phase 20 text only (lines 514, 972, 1101); the route is `/exit-register` in phase 15 (lines 881, 1080).
- Proposed text: API.md 3.14 row `POST /exits` is replaced by a pointer to `POST /exit-register` (phase 15, kyc.work, step-up decision, Cedar exit.record). No phase route is added; phase 20 does not build the row.

### G1-API-14 GET /islamic/deals/{}/evidence/{}/content (API.md line 778)
- Severity: LOW
- Category: Incomplete Coverage
- Target: docs/API.md section 3.18 (line 778): no change needed
- Sources: phase-26.md:290 "GET `/api/v1/islamic/deals/{id}/evidence/{evidence_id}/content` (added) | `queue.work`"
- Check: grep -n "evidence/{evidence_id}/content" docs/plan/phase-26.md -> line 290.
- Present: owned by phase 26 section 2. Proposed text: none.

### G1-API-15 POST /islamic/deals/{}/evidence/{}/verify (API.md line 779)
- Severity: LOW
- Category: Incomplete Coverage
- Target: docs/API.md section 3.18 (line 779): no change needed
- Sources: phase-26.md:291 "POST `/api/v1/islamic/deals/{id}/evidence/{evidence_id}/verify` (added) | `queue.work`"
- Check: grep -n "evidence/{evidence_id}/verify" docs/plan/phase-26.md -> line 291.
- Present: owned by phase 26 section 2. Proposed text: none.

## Summary

Findings by severity (40 in total):
- HIGH: 2 (G1-14-01, G1-14-03)
- MEDIUM: 24 (G1-13-02, G1-14-02, G1-14-04, G1-14-05, G1-14-06, G1-14-07, G1-15-01, G1-15-02, G1-15-03, G1-16-01, G1-16-02, G1-16-03, G1-16-04, G1-API-01 to G1-API-09, G1-API-12, G1-API-13)
- LOW: 14 (G1-11-01, G1-11-02, G1-12-01, G1-13-01, G1-13-03, G1-15-04, G1-15-05, G1-16-05, G1-16-06, G1-16-07, G1-API-10, G1-API-11, G1-API-14, G1-API-15)

Obligations already present (skipped, not written as findings): about 45. Examples: phase 11 canon.touched emission and its payload (matches phase 15); phase 11 blind-period routes, causes and scopes (used by phases 12 and 30); phase 11 section 4.4 names phase 12's quarantine route and filters; phase 12 admin.exclusions, coverage_certificates, recon_run and ControlTotals; phase 13 feature_keys(), bilingual names and the requires column; phase 14 counts_for, case_id without FK (phase 17 adds it), list_max_age_h, ListRecord::stale, ScreeningIndexRegistry, Scorer, insert_candidates, the freeze port, R7 retention, four-eyes on freeze true, the feed_unchanged event, PairMemoryService::propose, the 5,000-row daily base; phase 15 canon.touched consumer, the exit-register route, the rating feed cursor on seq; phase 16 AlertSinkError runner (lead sentence in 3.7), go-live gate, replay business_date, scenario_versions effective_from, the hibernation evaluation error; API rows owned by phase 16 (samples packet, retire requests) and phase 26 (Islamic evidence content and verify).

UNCLEAR (two sources disagree, or no owner or content exists; no text proposed):
- G1-14-07 freeze-list alert retention: R7 (phase 14, 20) versus ARCHITECTURE R5 sanctions-tied ten years.
- G1-15-05 requirement IDs K-12, K-19, K-20, K-23, K-25, K-31, K-32, K-34, K-36, K-38 to K-41 have no owner; K-39 and K-40 appear in no phase file.
- G1-API-05 and G1-API-06 backfill routes: phase 11 names phase 12 as owner, phase 12 says neither builds them.
- G1-API-07, G1-API-08 reference registers (dual-use goods, flagged districts, crypto counterparties, sector-goods map): no phase has the content or a table.
- G1-API-09 GET /ref/typologies: no tag set source.
- G1-API-12 harness-diff route: phase 16 names phase 24; phase 24 does not carry it.
- Gap item (l), "Phase 16 items" in tools/agent/gap-items.md, names no items, so it was not resolved here.
- Gap item (h), "freeze-list alert retention R7 vs R5" is covered by G1-14-07.
- Gap item (j), "rating feed cursor": phase 15 has the feed on `seq` (phase-15.md:238 and 241), so no finding.
- Gap item (i), masked readers: findings G1-13-03, G1-15-01 and G1-16-01. Phase 14 masks only fixed suspicion-tier kinds (screening alerts and pair memory) and reads no party tier, so no change there.
