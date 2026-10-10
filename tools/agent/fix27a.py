p='docs/plan/phase-27.md'
s=open(p).read()
def rep(a,b):
    global s
    assert s.count(a)==1,(s.count(a),a[:90])
    s=s.replace(a,b)
# 1 guard
rep("""       OR NEW.signature IS DISTINCT FROM OLD.signature OR NEW.signing_key_version IS DISTINCT FROM OLD.signing_key_version
       OR (OLD.shadow_criteria IS NOT NULL AND NEW.shadow_criteria IS DISTINCT FROM OLD.shadow_criteria) THEN""",
"""       OR NEW.signature IS DISTINCT FROM OLD.signature OR NEW.signing_key_version IS DISTINCT FROM OLD.signing_key_version
       OR (OLD.shadow_criteria IS NOT NULL AND NEW.shadow_criteria IS DISTINCT FROM OLD.shadow_criteria
           AND NOT (OLD.status = 'retired' AND NEW.status IN ('shadow', 'challenger'))) THEN   -- rollback: new criteria""")
rep("""        IF NEW.status = 'live' AND current_setting('akr.model_promotion', true) IS DISTINCT FROM 'on' THEN
            RAISE EXCEPTION 'live is set only by the promotion change' USING ERRCODE = 'insufficient_privilege';""",
"""        IF (NEW.status = 'live' OR OLD.status = 'live')
           AND current_setting('akr.model_promotion', true) IS DISTINCT FROM 'on' THEN
            RAISE EXCEPTION 'live is entered and left only by the promotion change' USING ERRCODE = 'insufficient_privilege';""")
# 6 models guard
rep("""    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'model families are never deleted' USING ERRCODE = 'insufficient_privilege';""",
"""    IF TG_OP = 'DELETE' THEN
        IF current_user = 'akr_retention' THEN RETURN OLD; END IF;   -- phase 23 retention only, after its versions
        RAISE EXCEPTION 'model families are deleted only by retention' USING ERRCODE = 'insufficient_privilege';""")
# 5 kill row at creation
rep("insert `admin.models` with `next_kill_drill_due = today plus model.kill_drill_months months` (bank calendar), then call",
    "insert `admin.models` with `next_kill_drill_due = today plus model.kill_drill_months months` (bank calendar) and its `admin.model_kill_state` row (`scope = 'model:<id>'`, `engaged = false`), then call")
rep("Insert the scope row if missing (a model scope), lock it,","Lock the scope row (created with the family, section 3.1; an unknown scope is 404 `E_NOT_FOUND`),")
# 3 start_shadow
rep("**Shadow summary (computed at the promotion decision,",
"""**Start a shadow or challenger run (route 7, `ModelRegistry::start_shadow`, `model.shadow.start`, MRO; `Idempotency-Key`; `If-Match` on the version):**
1. Lock the family, then the version (lock order 2 and 3). The version is `registered` or `retired` (a rollback, section 3.7 step 6), else 409 `E_NOT_ALLOWED_IN_STATE` `not_registered`; its inventory item is `approved_for_use` (`inventory_not_approved`); no other version of the family holds the requested role (`role_taken`; the partial unique indexes of section 1.1 are the backstop); role `challenger` needs a live champion in the family (`no_champion`).
2. Gate: the latest `label_gate` evaluation of the version meets the shadow gate (`model.gate_shadow_positives`, or `model.gate_recalibrate_positives` for a starter family), else `gate_not_met`.
3. Criteria: exactly the closed keys of the shadow summary below, each at or beyond its floor (`min_days >= model.shadow_min_days`, `min_scored_alerts >= 1000`, `min_rank_correlation_with_live >= 0.5`, `top_band_share_change_max <= 0.05`), else 422 `E_VALIDATION` `criteria_invalid`.
4. One update: `status` = the role, `shadow_started_at = now()`, `shadow_criteria` (frozen; on a rollback the guard admits new criteria only on this edge), `retired_at = NULL` on a rollback, and a provisional `top_band_cut` = the `model.top_band_percent` percentile of `reference_bins.score` (the import manifest), so `top_band` is defined for every shadow row. Audit `model.shadow_started {role, min_days}`.

**Shadow summary (computed at the promotion decision,""")
# 4 suspicion exclusion
rep("- Outcomes not yet QA-confirmed do not count. Alerts whose case is in suspicion tier count in the stored totals only (section 3.14).",
"- Outcomes not yet QA-confirmed do not count. Every decided case counts, whatever its tier: phase 18 raises the subject to `suspicion` on every `utr`, `str` and `tfr` decision (phase 18 section 3, step 8), so excluding suspicion tier would remove almost every positive. The job reads the rows; only window totals leave it, suppressed below `report.small_number_threshold` (section 3.14).")
rep("Only `casework.decisions.type` and the alert flags are read; reason codes and text are not. The rate is overrides over decisions of the window with live-scored alerts. Overrides of suspicion-tier cases are excluded before the count (section 3.14).",
"Only `casework.decisions.type` and the alert flags are read; reason codes and text are not. The rate is overrides over decisions of the window with live-scored alerts. Decisions of every tier count (a report decision always raises the subject to `suspicion`, phase 18), as phase 24 counts `str_rate`; only the window rate leaves the job, suppressed below the threshold (section 3.14).")
rep("Live-role scores only, from `alert.model_scores`; alerts in suspicion tier are excluded before the count (section 3.14).",
"Live-role scores only, from `alert.model_scores`; alerts of every tier count, as phase 24 counts its other rates, and the cell is suppressed below the threshold (section 3.14).")
rep("| Counts and distributions only, over all live-scored alerts of the queue except suspicion-tier alerts, which are excluded before any count (the exclusion is not reported); every count under the small-number threshold is suppressed. Nothing about one case |",
"| Counts and distributions only, over all live-scored alerts and decisions of a 30-day window, every tier included (as phase 24 `str_rate`); every count under the small-number threshold is suppressed. Nothing about one case: no id, no party, no day-level row of one decision. A reader comparing two consecutive daily points can learn that some report decision was made in the window, never on which case (gap pass item) |")
rep("| A rate over decisions by type; suspicion-tier cases excluded; no reason text, no decider, no case id |",
"| A window rate over decisions of every tier, suppressed below the threshold; no reason text, no decider, no case id |")
rep("the monitoring, ladder and override reads are counts from stored points, with suspicion-tier alerts excluded before counting;",
"the monitoring, ladder and override reads are window counts from stored points over every tier, suppressed below the threshold (lead review);")
# 7 purpose
rep("**Single read (route 16, `GET /alerts/{id}/model-score`, `alert.model_score.read`, A1, A2, M; purpose code of the `assigned_work` family (phase 03 `purpose_codes`), else 422 `E_PURPOSE_REQUIRED`):**",
"**Single read (route 16, `GET /alerts/{id}/model-score`, `alert.model_score.read`, A1, A2, M; the same purpose and step-up rules as phase 17 `GET /alerts/{id}`: rule 4 for restricted, rule 5 for suspicion):**")
rep("A score is an AML/CFT output: `alert.model_score.read` requires the purpose code of the `assigned_work` family,",
"A score is an AML/CFT output: `alert.model_score.read` takes the alert's own purpose rules (phase 17 rules 4 and 5),")
rep("| A1, A2, M, under the alert's visibility, with an `assigned_work` purpose; suspicion tier answers masked 404 with step-up first |",
"| A1, A2, M, under the alert's visibility and the alert's own purpose rules; suspicion tier answers masked 404 with step-up first |")
rep("| `E_PURPOSE_REQUIRED` (422) | none | `GET /alerts/{id}/model-score` without an `assigned_work` purpose |",
"| `E_PURPOSE_REQUIRED` (422) | none | `GET /alerts/{id}/model-score` on a restricted or suspicion-tier alert without the purpose phase 17 requires |")
# 8 feature lookup
rep("from `feat.feature_values` by the subject customer (`canon.customer.id` of the alert's party, phase 18 rule), at the alert's `created_at` as the `as_of` bound.",
"from `feat.feature_values` by `entity = 'customer'` and `entity_id` = the subject customer (`canon.customer.id` of the alert's party, phase 18 rule), `feature_id` of the manifest's `(key, version)`, the currency the manifest names per feature (`XXX` for currency-free kinds, phase 13), and `as_of` = the latest closed month-end on or before the alert's `created_at` (no look-ahead); a row whose `status` is not `value` is passed as missing.")
# 2 artifact add
rep("It is written only by the signed bundle import (phase 00 `bundle import`) and the release import of phase 25, with the files of the bundle, never by a route.",
"It is written by the signed bundle import (phase 00 `bundle import`) and the release import of phase 25 (starter models), and by `akr-ctl model artifact add --file F` on the bank host for a bank-trained model (TS-16): the operator command writes `import/<sha256>.onnx` once, refuses an existing name, prints the sha256 for the manifest and writes no database row (the same host-side pattern as phase 25 `akr-ctl support-bundle`). No route writes it.")
# 9 closure codes + other family
rep("the register update refuses a closure code outside the five of section 3.8 for source `model_monitor` (422 `E_VALIDATION` `closure_code_not_allowed`);",
"the closure-code list of phase 24 (section 3.4 there: `EFFECTIVE`, `RULE_CHANGED`, `COMMITTEE_ACCEPTED`, `NOT_REPRODUCED`, `DUPLICATE`) gains `RETRAINED`, `RECALIBRATED`, `RESTRICTED_USE`, `RULES_ONLY` and `JUSTIFIED_NO_ACTION`, and the register update refuses a closure code outside these five for source `model_monitor` (422 `E_VALIDATION` `closure_code_not_allowed`);")
rep("no kill switch is engaged for the family or globally (`kill_engaged`);",
"no kill switch is engaged for the family or globally (`kill_engaged`); no other family holds a live version (`other_family_live`, the one-live-family index of section 1.1; Open doubts 22);")
rep("`pinned`, `load_refused`, `kill_engaged`, `not_engaged`, `breach_open` |",
"`pinned`, `load_refused`, `kill_engaged`, `not_engaged`, `breach_open`, `not_registered`, `role_taken`, `no_champion`, `other_family_live` |")
# open doubt 22 + lead corrections
rep("## Open doubts (for gap pass 1 and the bank)\n",
"""**Lead corrections (review 2026-10-09).** The version guard froze `shadow_criteria` for ever and let a version leave `live` without the promotion flag, so the rollback by re-shadowing could not run and a retired champion could stay the family's live pointer; the criteria now change only on the edges `retired -> shadow` and `retired -> challenger`, and leaving `live` needs the flag. A bank-trained artifact had no way into the read-only store; `akr-ctl model artifact add` writes it on the host. Route 7 had no steps and `top_band_cut` was NULL during shadow while `top_band` is NOT NULL, so every shadow insert failed; `start_shadow` is written out with a provisional cut from the reference bins. Phase 18 raises the subject to `suspicion` on every report decision, so excluding suspicion tier from the counts removed almost every positive label and every override of kind (b); the jobs now count every tier and only suppressed window aggregates leave them, as phase 24 `str_rate` does. The kill-switch row of a family is created with the family (engage takes `If-Match` on it); the family guard lets `akr_retention` delete; route 16 takes the alert's own purpose rules; the feature lookup names the month-end, currency and status keys of `feat.feature_values`; the five closure codes join phase 24's list; a second family's promotion is refused with `other_family_live`.

## Open doubts (for gap pass 1 and the bank)
""")
rep("21. **Model cards and the ten sections.**",
"22. **One live family.** The unique index allows one family with a live version per bank, so a bank that changes family retires the old champion (a promotion change of the new family cannot run while the old one is live). Whether a family switch should be one change request (retire A, activate B) is for the bank and gap pass 1.\n21. **Model cards and the ten sections.**")
open(p,'w').write(s)
print('ok')
