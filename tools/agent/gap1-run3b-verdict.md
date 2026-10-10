# Gap pass 1 run 3b: lead verdict (2026-10-10; APPLIED by tools/agent/apply-g1-run3b.py and apply-g1-run3b2.py)

Draft: tools/agent/gap1-run3b.md (Haiku; 40 findings: 2 HIGH, 24 MEDIUM, 14 LOW; targets 11 2, 12 1, 13 3, 14 7,
15 5, 16 7, API 15). Read in full by the lead; sources and targets grepped.

Timing rule used throughout: `casework.effective_tier` exists from phase 17 (phase 17 migration, function at its DDL);
`ProtectedAttributeCheck` from phase 24; `RiskModifierSource` trait is defined by phase 15 (its section 2); the
`ScreeningCaseLinker` port by phase 17. A note in an earlier phase says "from phase NN on".

## Per finding (target phase: block to write)

- G1-11-01 ACCEPT note: phase 11 writes canon only through phase 10 `CanonicalWriter::write`, so the phase 23 tombstone
  check (phase 10 section 7+) covers it. Cite "phase 10 section 7+", not 7.4.
- G1-11-02 ACCEPT note (rewritten): phase 12 owns: Cedar entity `Quarantine` gains `gate: String` (derived: the row is
  held for an unmapped transaction-type code, reason `unmapped_code` with field `txn_type`, DSL.md B.7; no column);
  `QuarantineRepository` gains `assign` and `accept_conflicts`; `QuarantineFilter` gains `owner_id`, `older_than_days`
  and order `oldest`; exclusion refusals `gate_not_excludable`, `card_guard_not_excludable` (phase 12 section 3.11).
- G1-12-01 REWRITE: Haiku's claim condition is invented. Phase 30 section 3.9 `restore verify` is sequential and stops at
  the first failure: step 2 (retention replay) completes before step 5 queues `recon_run`. Note in phase 12: recon runs
  for a restore gap are valid; a date that ingestion has not reloaded yet shows as a break, which is the intended
  result; no claim condition. Closes phase 30 Open doubts 23 (carry the phase 30 wording to the run targeting 30).
- G1-13-01 ACCEPT note (phase 16 section 3.2 adds `FeatureCompiler::compile_measure`, `compile_filter`; makes
  `ProfileComparator::plan`, `evaluate` public).
- G1-13-02 ACCEPT (MEDIUM), DR-379: `check_before_decision` (section 3.7) calls `ReaderCatalogue::measure` again; a
  changed effect refuses `volume_effect_changed` (DR-360); NO recompute entry point in phase 04: the maker resubmits
  once the reader replay completes. Closes phase 16 Open doubts 12.
- G1-13-03 ACCEPT note (rewritten): phase 13 reads no tier; profile reads are restricted-tier reads. From phase 15 the
  party is loaded through the masked `customer` adapter (tier from phase 15, `casework.effective_tier` from phase 17).
  The access event keeps tier `restricted`.
- G1-14-01 ACCEPT HIGH: from phase 17 on, decision step 8 (section 3.6) calls `ScreeningCaseLinker::link_screening(tx,
  screening_alert_id, actor)` in the decision transaction for `subject_kind = 'party'` decided `true` or `potential`;
  counterparty and payment-party subjects are not linked (phase 18 DR-147). Before phase 17 no call.
- G1-14-02 ACCEPT, DR-375 (rewritten): `screening.hit` is written by the phase 14 decision transaction for a party
  subject decided `potential` or `true`, payload `{alert_id, party_id, list_kind, outcome}` (ids and codes only), FROM
  PHASE 15 ON (phase 15 registers the first handler, its TR-15 trigger to `TRG_REVIEW_REQUESTED`; phase 19 appends its
  handler in the ordered handler list of phase 00 section 4.5 item 10 / phase 22). Suppression records: phase 19's
  `qa_suppression_sample` population is the `alert.pair_memory` rows whose `last_suppressed_at` falls in the ISO week
  (5 per cent, plus every row on a freeze-list entry); phase 14's sweep keeps expiry and counters and draws no sample.
  Phase 15 correction block: `screening.hit` handler registered by phase 15 itself (producer precedes it), the other
  topics of its trigger table by their producers. Phase 19 Open doubts 9 closed (carry wording to run targeting 19).
- G1-14-03 ACCEPT HIGH as a note owned by phase 29 (phase 29 section 4.5 item 4 and Open doubts 29): phase 29 adds
  `ScreeningService::open_hold_alert(run, candidate)` (one caller `HoldSweepService`; system `payment` run,
  `requested_by` NULL, `trigger_ref` = the hold id; candidate via `AlertRepository::insert_candidates`); the
  `subject_id` and `trigger_ref` comments of `alert.screening_runs` read "adhoc runs and hold payment runs" / "payment
  runs: the hold id (phase 29)"; the subject view of a hold alert reads names from `assess.holds.party_names_enc` under
  phase 14 masking; `screening_payment` skips an entry already alerted on a hold whose decision has the same match key.
- G1-14-04 ACCEPT note (rewritten): phase 15 defines `RiskModifierSource::modifier(party)` (its section 2, setting
  `screening.rating_modifiers`, a json setting not a table); from phase 15 on the threshold step (section 3.4 step 8)
  adds it for a party subject within the existing bounds; 0 before phase 15 and for other subject kinds.
- G1-14-05 REWRITE, DR-376: delisting is by absence (phase 20 section 3.5 gate already reads absence); rows are NOT
  copied into the new version (versions are immutable after `loading`); `delisted_on` is set only when the publisher's
  file carries a delisting date; a silent drop is guarded by `screening.max_count_drop_pct` with
  `acknowledged_count_drop`, and `diff_summary.removed` counts drops. Closes phase 20 Open doubts 22 (carry wording).
- G1-14-06 ACCEPT, DR-378 (rewritten): column on `ref.lists`, not `ref.list_versions` (immutable after loading):
  `feed_checked_at TIMESTAMPTZ` in migration 0045, `GRANT UPDATE (feed_checked_at)` to akr_app in 0048 grant line,
  classification row (internal) in 0049. `list_fetch` step 2 `NotModified` sets it (with `list.feed_unchanged`).
  `ListRecord::stale`: `age_hours = now - max(active.activated_at, feed_checked_at)` (or `loaded_at` for
  `cbs_snapshot`). 12-hour cap unchanged. Closes phase 29 Open doubts 29 last sentence.
- G1-14-07 DECIDE, DR-377: screening alerts (freeze lists included) stay R7; the bank sets
  `retention.period_years.R7` (phase 23, default 7, min 7) to 10 where OFAC applies; no new class. Closes phase 14 Open
  doubt "Retention".
- G1-15-01 ACCEPT note: from phase 17 on the customer kind's `tier` is `casework.effective_tier(party_id)`; until then
  `canon.party.sensitivity_tier`.
- G1-15-02 REJECT: already in phase 15 section 4.4 (`register_rating_attributes()`).
- G1-15-03 ACCEPT note (rewritten): phase 15 ships `exit_decision` ending at `record_decision` (an unregistered effect
  fails publish); phase 20 registers `create_ops_task` and ships version 2 of `exit_decision` adding it after
  `record_decision` (DSL E.4).
- G1-15-04 MERGE into the phase 15 correction block of G1-14-02.
- G1-15-05 CARRY: K-prefix requirement owners (K-12, K-19, K-20, K-23, K-25, K-31, K-32, K-34, K-36, K-38 to K-41) go to
  the coverage runs; add prefix K to run 5's list in gap1-plan.md.
- G1-16-01 ACCEPT note: from phase 17 on, request-time tier reads (sections 3.9, 3.10, Decisions) use
  `casework.effective_tier(party_id)`.
- G1-16-02 ACCEPT note owned by phase 26 (G9 reads `pack: akr` or `pack: islamic`, refusal `pack_not_enabled` with
  `pack.islamic` off; seed skips a document whose `requires_flag` is off; `packs verify-data` refuses an ISL document
  without a further condition).
- G1-16-03 ACCEPT note (rewritten): from phase 24 on, the `scenario` change target's `check_before_decision` runs
  `ProtectedAttributeCheck` (phase 24 section 3.5) on the proposed version; a finding refuses the decision. Not in
  `GoLiveGate`.
- G1-16-04 ACCEPT note, DR-382: from phase 22, `DetectionPayload` gains `source_kind` (`scenario` | `observation`); the
  runner writes `scenario`. The scenario named by `branch.hybrid_scenario_code` is a register scenario with
  `observational = true` (phase 06), NOT a family `observation` (family is `^[A-Z]{2,5}$`, one of the 18). Carry the
  phase 22 wording ("family `observation`") to the run targeting 22.
- G1-16-05 ACCEPT note (phase 17 replaces the sink; audit kinds written by `AlertIntakeService`).
- G1-16-06 ACCEPT note: activation instant = `scn.scenario_versions.effective_from`. Closes phase 24 Open doubts 11/30
  (carry).
- G1-16-07 ACCEPT note (phase 30 DR-327 keys `read_source`, `replay_lag_s`).

## PART B (API rows), DR-380 and DR-381

routecheck fixed (tools/routecheck.py: a trailing "(NEW)"/"(added)" after the path is allowed): now 565 owned, 11 not
owned. API-10, 11, 14, 15 were owned all along.
- API-01..03: DELETE API.md rows 206-208 (`/cases/{id}/acl`): duplicates of the phase 18 rows already in API.md 3.12
  (`/cases/{id}/access`, lines ~523-525).
- API-04: DELETE API.md row 227 `GET /evidence/{id}`; fix the sentence after the 3.4 table ("... the `/audit` rows above
  are phase 24's and `GET /evidence/{id}` is phase 18's.") to say evidence content is served per owner (for example
  `GET /cases/{id}/evidence/{evidence_id}/content`, phase 18); no generic route (phase 26 states the same).
- API-05/06: DELETE rows 266-267 (backfills) and fix the sentence "the backfill routes are phase 12." in the 3.5 intro.
  The re-read window (DR-37, overlap default 31 days) and the automatic catch-up (DR-90) cover gaps; an explicit range
  beyond the overlap is P2. Notes in phase 11 (its line "the operator-planned backfill of API.md 3.5 (phase 12) is for
  explicit slices only") and phase 12 (Open doubt "Backfill routes") via 7.x blocks.
- API-07/08: DELETE rows 369-370 (`/ref/registers/...`): no scenario or phase reads these registers; P2 with the content
  that needs them. Phase 09 out-of-scope line already excludes them.
- API-09: OWNER phase 06 (DR-381): `GET /ref/typologies` on the `catalogue` router, scope self, Cedar
  `typology.read` (every authenticated principal, like `cbi_catalogue.read`): the 21 typologies of research file 27
  (B2.1 to B2.16 plus the five cross-cutting tags 17 to 21) shipped as `backend/catalogue/typologies.v1.json`
  `{number, tag, label {ar, en}}`, built by `tools/build_scn_register.py`; scenarios keep `typologies SMALLINT[]`
  numbers; the DSL `typology` strings are the `tag`s. API.md row 371 gets "(phase 06)". Phase 09's out-of-scope line
  says the set comes with phase 16: phase 09 correction not needed (carry: it is a "not here" line; fix in run 4 if
  touched).
- API-12: DELETE row 478 (`harness-diff`): S-48 (independent re-implementation by another author) is P2; phase 24 does
  not carry it. Phase 16 Open doubts 22 closed (note in phase 16).
- API-13: DELETE row 635 `POST /exits` (already marked superseded; phase 20 asked the lead to remove it).
After these, routecheck should show 0 not owned (row 371 owned once phase 06's 7.x route row exists).

## DRs (from DR-375; DR-374 is the last applied)

DR-375 screening.hit; DR-376 delisting by absence; DR-377 R7 retention; DR-378 feed_checked_at; DR-379 re-measure at
decision, no recompute; DR-380 API.md rows removed (acl, evidence, backfills, registers, harness-diff, exits); DR-381
typologies route phase 06; DR-382 observational scenario flag.

## Apply plan (next session)

tools/agent/apply-g1-run3b.py, pattern apply-g1-run3a.py (g1lib.add_section7; section-limited UNCLEAR check; HDR for
targets 11-16, CHDR correction blocks for 15 (screening.hit handler) and 06 (typologies route, as a run 3b block));
API.md row deletions with rep(); DRs after the DR-374 row; then idcheck, routecheck; logs (roadmap row "1 (gap pass 1,
run 3b: targets 11-16 + API rows)", status line "Run 3b", README "runs 1, 2, 3a, 3b", b3-conventions DR range,
_progress entry + NEXT, gap1-plan run log + add K to run 5). Then run 4 (targets 17-31 + system docs; include the
carried items: phase 22 items 9 and 14 and "family observation"; phase 29 payment release status; phase 30 Open doubts
23; phase 19 Open doubts 9; phase 20 Open doubts 22; phase 24 Open doubts 11/30).

## Applied (2026-10-10)

apply-g1-run3b.py: section 7 blocks in phases 06 (1), 11 (3), 12 (2), 13 (3), 14 (7), 15 (2 + 1 correction), 16 (8);
API.md rows removed and two sentences fixed; DR-375..382. Lead wording fixes against the verdict: the `screening.hit`
handler is phase 15's re-score trigger on that topic (TR-15 is the case-decision trigger, DSL.md C.5); the trigger table
is phase 15 section 3.3; DR-327's keys live in `scn.scenario_runs.cost`; phase 24 Open doubts 11 is the S-48 "not
carried" line (harness-diff), 30 the activation instant.
apply-g1-run3b2.py (lead finding, MEDIUM, from the launch note): phase 14 wrote the access action `search`, which phase
02's closed CHECK lacks; DR-383: one `search_hit` per search, resource type `screening_search`. idcheck 383 / 0;
routecheck 566 rows, 0 not owned (5 phase routes absent from API.md are health, metrics and dev routes).
