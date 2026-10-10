# Phase 27 review findings (lead, 2026-10-09)
1. MEDIUM rollback and live exit broken by the version guard: the guard refuses any change of `shadow_criteria` once set,
   but the rollback path (s.3.7 step 6: `retired -> shadow` "with a new frozen criteria set, its own shadow period")
   needs new criteria and a new `shadow_started_at`, and the CHECK `(status = 'retired') = (retired_at IS NOT NULL)`
   needs `retired_at` cleared; also `live -> retired` is allowed without the promotion flag, which leaves
   `admin.models.live_version_id` pointing at a retired version (the pointer moves only under the flag). Fix: on the
   edges `retired -> shadow` and `retired -> challenger` only, `shadow_criteria`, `shadow_started_at` and `retired_at`
   may change; leaving `live` needs the flag too.
2. MEDIUM no way to put a bank-trained artifact into the store: s.4.4 says the `model-store` volume is written only by
   the phase 00 bundle import and the phase 25 release import, so a model trained by the bank's own team (TS-16, the
   main case) can never be imported. Fix: `akr-ctl model artifact add --file F` on the bank host (operator, writes
   `import/<sha256>.onnx` once, refuses an existing name, prints the sha256), the same host-side pattern as phase 25
   `akr-ctl support-bundle`; the route still checks only the manifest and signature.
3. MEDIUM shadow start unspecified and shadow scoring cannot insert: route 7 `POST /models/{id}/shadow` has no steps
   (s.3.1 points to s.3.6, which has none): no state check, no gate, no criteria validation; and `top_band_cut` is
   NULL until activation while `alert.model_scores.top_band` is NOT NULL and computed as `score >= top_band_cut`, so
   every shadow insert fails and the top-band share criterion has no value. Fix: `start_shadow` steps (version
   `registered` or `retired`, item `approved_for_use`, shadow gate met (200 bank-trained, 50 starter), criteria keys
   closed and at or above the floors, no other shadow or challenger of the role; provisional `top_band_cut` from the
   `model.top_band_percent` percentile of `reference_bins.score`; recomputed at activation).
4. MEDIUM override rate and label counts lose every report decision: s.3.6, s.3.8 and s.3.14 exclude suspicion-tier
   cases before counting, but phase 18 s.3 step 8 (l.759) raises the subject party to `suspicion` on every `utr`,
   `str` and `tfr` decision, so positives and override kind (b) are almost never counted (GV-21 and T-15 are not
   met). Phase 24 already counts STRs in window aggregates (`fairness_metrics.str_rate`) with suppression. Fix: the
   jobs count every decision and alert of the window (the worker reads them; no id leaves the job); reads show only
   window aggregates suppressed below `report.small_number_threshold`, as phase 24 does.
5. LOW kill switch on a scope row that may not exist: engage requires `If-Match` on the scope row but inserts the
   `model:<id>` row "if missing". Fix: the family's row is inserted (engaged false) when the family is created.
6. LOW `admin.models_guard` refuses every DELETE although `akr_retention` holds DELETE and the table is R11
   (retirement). Fix: the guard lets `akr_retention` delete, as the version guard does.
7. LOW route 16 requires an `assigned_work` purpose for every alert, stricter than `GET /alerts/{id}` (phase 17 rules
   4 and 5 need it only for restricted and suspicion tiers), so the W2 panel would fail on normal alerts. Fix: the same
   purpose and step-up rules as the alert's own read.
8. LOW feature lookup: `feat.feature_values` is keyed by `entity`, `entity_id`, `feature_id`, `currency` and month-end
   `as_of`; the draft gives only the customer and the alert time. Fix: the latest closed month-end on or before the
   alert's `created_at`, the currency each manifest feature names (`XXX` for currency-free), `status <> 'value'` is
   missing.
9. LOW phase 24 changes not stated in full: the five closure codes (RETRAINED, RECALIBRATED, RESTRICTED_USE,
   RULES_ONLY, JUSTIFIED_NO_ACTION) must join phase 24's closure-code list; the one-live-family unique index makes a
   second family's promotion fail with a raw unique violation. Fix: s.4.5 names the code-list change; `prepare` refuses
   `other_family_live` (409), and the open doubt names how a bank changes family.
Severity: 0 critical / 0 high / 4 medium (1-4) / 5 low (5-9) = 9.
Gap-pass items (not draft defects): DATA-MODEL s.15 rows (model_scores to `alert`, three new tables); API.md 3.15
meaning of `POST /models` and the 10 added rows (DR-271); phase 24 `registers.source` + `model_monitor`,
`fairness_metrics.metric` + `score_top_band_rate`, closure codes, review-calendar row kinds `model_kill_drill` and
starter recalibration, command-centre tile `model_governance`, `ValidationPackService` model branch; phase 23 purge
of `alert.model_scores` before alert partition drop (phase 17 children have the same FK pattern); phase 25 SIEM and
support bundle exclude scores, release import sets `registered` only; phase 02 owner kind `model_version`; phase 00
`model-store` volume and key purpose `keys/model-artifact`; phase 17 queue ORDER BY and `model_band`; kill switch
owner vs MRO scope (Open doubts 1); differencing of daily rolling-window points (an MRO learns that a report decision
happened, never which case).
