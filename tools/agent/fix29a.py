p='docs/plan/phase-29.md'
s=open(p).read()
def rep(a,b,n=1):
    global s
    assert s.count(a)==n,(s.count(a),a[:90])
    s=s.replace(a,b)

# 1: release control counts only an unexpired pass
rep("    reason_code         TEXT NOT NULL CHECK (reason_code IN ('no_decision', 'no_match_key', 'decision_not_pass')),\n    decision_id         UUID,                           -- the non-pass decision, for decision_not_pass",
    "    reason_code         TEXT NOT NULL CHECK (reason_code IN ('no_decision', 'no_match_key', 'decision_not_pass', 'pass_expired')),\n    decision_id         UUID,                           -- the non-pass decision (decision_not_pass) or the expired pass (pass_expired)")
rep("""  '{"ar": "نافذة مطابقة الإفراج بدون قرار (دقائق)", "en": "Window in which a release must match a decision (minutes)"}'),
""",
"""  '{"ar": "نافذة مطابقة الإفراج بدون قرار (دقائق)", "en": "Window in which a release must match a decision (minutes)"}'),
 ('assess.release_skew_s', '60', 'duration_s',
  '{"max": 120, "basis": "API.md 4.2: a pass is valid until expires_at; the skew covers clock drift between the gateway and the canonical copy, a larger value lets a stale pass release unseen"}', TRUE,
  '{"ar": "هامش انحراف الساعة عند مطابقة الإفراج (ثوان)", "en": "Clock skew allowed when a release is matched to a pass (seconds)"}'),
""")
rep("""   - a `pass` decision (any mode), or a decision in `advisory` mode (advisory releases are expected), or an enforcing `hold` followed by a `hold_release` pass: matched;
   - only a `reject`, or an enforcing `hold` with no later `hold_release` pass: exception `decision_not_pass`.""",
"""   - a `pass` decision (any mode), or a decision in `advisory` mode (advisory releases are expected), or an enforcing `hold` followed by a `hold_release` pass: matched, but only when the payment's `received_at` is at most the pass's `expires_at` plus `assess.release_skew_s` (60; max 120). A pass counts only while it is valid (API 4.2: a gateway releasing later must call again);
   - only passes that expired before the release: exception `pass_expired` (`decision_id` = the newest such pass);
   - only a `reject`, or an enforcing `hold` with no later `hold_release` pass: exception `decision_not_pass`.""")
rep("- Control: `assess.heartbeat_max_s` (90; max 120), `assess.release_window_min` (120; min 60),",
    "- Control: `assess.heartbeat_max_s` (90; max 120), `assess.release_window_min` (120; min 60), `assess.release_skew_s` (60; max 120),")

# 2: freshness is phase 14 ListRecord::stale
rep(""" ('assess.list_grace_hours', '2', 'integer',
  '{"max": 2, "basis": "CBI rule: freeze lists refresh at least every 12 hours; the grace may not widen the staleness window"}', TRUE,
  '{"ar": "مهلة تأخر القوائم قبل اعتبارها قديمة (ساعات)", "en": "Grace beyond a list refresh before it counts as stale (hours)"}'),
""", "")
rep("12. **Freshness** (section 3.6). For each enabled list the active version's `loaded_at` is compared with `ref.lists.refresh_hours` plus `assess.list_grace_hours`.",
    "12. **Freshness** (section 3.6). For each enabled list the phase 14 rule `ListRecord::stale` decides (phase 14 section 3.2: `now - active.activated_at` above `min(refresh_hours, screening.list_max_age_h)`, never above 12 hours for a freeze list); this phase adds no grace.")
rep("Stale means `now() - loaded_at > refresh_hours + assess.list_grace_hours`, with `refresh_hours` from `ref.lists` (at most 12 for freeze lists, phase 14 CHECK). The stale state is read from the index registry, not from a query per request.",
    "Stale is the phase 14 rule `ListRecord::stale` unchanged (phase 14 section 3.2: `age_hours = now - active.activated_at`, stale above `min(refresh_hours, screening.list_max_age_h)`; at most 12 hours for freeze lists, phase 14 CHECK and floor). This phase adds no grace and no setting of its own. The stale state is read from the index registry, not from a query per request.")
rep("- Lists: `assess.list_grace_hours` (2; max 2), `assess.fallback_extract_hours` (12; max 12).",
    "- Lists: `assess.hold_list_kinds` (`[\"freeze\"]`; must contain `freeze`), `assess.fallback_extract_hours` (12; max 12). Freshness uses phase 14 `screening.list_max_age_h` (section 3.6); there is no setting of this phase for it.")
rep("22. Stale lists: with the test list's `loaded_at` moved back past `refresh_hours + assess.list_grace_hours`,",
    "22. Stale lists: with the test list's active version `activated_at` moved back past `min(refresh_hours, screening.list_max_age_h)` (phase 14 `ListRecord::stale`),")
rep("13. **DR-304 (list freshness: refresh plus a grace, cross-border holds, the stricter wording open).** A list is stale after its refresh interval plus `assess.list_grace_hours` (2, at most 2 under the freeze-list rule of the CBI).",
    "13. **DR-304 (list freshness: the phase 14 rule, cross-border holds, the stricter wording open).** A list is stale by the phase 14 rule `ListRecord::stale` (`now - active.activated_at` above `min(refresh_hours, screening.list_max_age_h)`, at most 12 hours for a freeze list under the CBI rule); this phase adds no grace.")

# 3: hold alert is a system payment run
rep("insert one `alert.screening_runs` row of kind `adhoc` (`subject_kind = 'payment_party'`, `subject_id = subject_ref`, `requested_by` = the gateway's `user_id`, status `completed`, `finished_at` now)",
    "insert one `alert.screening_runs` row of kind `payment` (a system run: `requested_by` NULL; `subject_kind = 'payment_party'`, `subject_id = subject_ref`, `trigger_ref` = the hold id, status `completed`, `finished_at` now)")
rep("it inserts an `adhoc` run and a candidate alert through `AlertRepository::insert_candidates`, and creates nothing else). The phase 14 `AlertService`, `FreezeTrigger` and masking are used unchanged.",
    "it inserts a system `payment` run (`requested_by` NULL, `trigger_ref` = the hold id) and a candidate alert through `AlertRepository::insert_candidates`, and creates nothing else). The phase 14 `AlertService`, `FreezeTrigger` and masking are used unchanged. Two requirements on phase 14 (Open doubts 29): the alert's subject view of a hold alert reads the names from `assess.holds.party_names_enc` (the hold's `subject_ref` is no canonical payment party); and `screening_payment` skips an entry already alerted on a hold whose decision has the same match key, so one match gives one alert.")
rep("the phase 14 queue shows one alert of run kind `adhoc` for the hold.",
    "the phase 14 queue shows one alert of run kind `payment` (`requested_by` NULL) for the hold, and no second alert for the same entry after the payment reaches `canon.payment_message`.")

# 4: drop a criterion that cannot fail
rep("| shadow to advisory | no release exception on the rail in the window | fixed (C-59 clean) |\n", "")

# 5 and 7: grants
rep("GRANT UPDATE (last_seen_at, build_version, policy_version, mode_summary, version) ON assess.node_status TO akr_assess;\nGRANT SELECT ON admin.users, admin.settings, admin.feature_flags TO akr_assess;",
    "GRANT UPDATE (started_at, last_seen_at, build_version, policy_version, mode_summary, version) ON assess.node_status TO akr_assess;\n"
    "GRANT SELECT (id, kind, status, expires_at) ON admin.users TO akr_assess;   -- the gateway's service principal; no names or emails in the gateway zone\n"
    "GRANT SELECT ON admin.settings, admin.feature_flags TO akr_assess;")

# 6: jobs idle while the pack is off
rep("- **Outage watch** (`OutageService::watch`, job `assess_blind_watch`, every minute):",
    "- **Outage watch** (`OutageService::watch`, job `assess_blind_watch`, every minute; it does nothing while `pack.assess` is off or while no rail has an open `assess.rail_modes` row, so an install without phase 2 records no outage):")
rep("The job names do not use the `ingest_` prefix. The decision path has no job: it runs in the request.",
    "The job names do not use the `ingest_` prefix. Each of the six jobs reads `pack.assess` first and returns without work while it is off; `assess_blind_watch` also needs at least one rail with an open mode row (section 3.13). The decision path has no job: it runs in the request.")

# 8: roll-up counts in every mode
rep("count `decisions`, `would_pass`, `would_hold` and `would_reject` (from `would_be`),",
    "count `decisions`, `would_pass`, `would_hold` and `would_reject` (from `coalesce(would_be, decision)`, so advisory and enforcing days count their decisions),")

# 9: PEP and internal hits hold only by setting
rep("Every list kind holds: freeze, PEP and internal.",
    "A freeze-list hit or review holds. A PEP or internal hit or review holds only when its list kind is in `assess.hold_list_kinds` (default `[\"freeze\"]`, which must contain `freeze`); otherwise it adds reason `LIST_NOTED` to a pass and no hold (research 25 15 asks pre-release screening for sanctions only).")
rep("else `hold` (`LIST_MATCH`, `LIST_REVIEW`, `MISSING_DATA`,",
    "else `hold` (`LIST_MATCH` or `LIST_REVIEW` of a holding list kind, `MISSING_DATA`,")
rep(""" ('assess.domestic_country', '"IQ"', 'string', NULL, TRUE,""",
""" ('assess.hold_list_kinds', '["freeze"]', 'json', NULL, TRUE,
  '{"ar": "أنواع القوائم التي يعلّق تطابقها الدفع", "en": "List kinds whose hits hold a payment (must contain freeze)"}'),
 ('assess.domestic_country', '"IQ"', 'string', NULL, TRUE,""")

# lead corrections and open doubts
rep("\n## Open doubts (for gap pass 1 and the bank)\n",
"""
**Lead corrections (review 2026-10-09).** The release control counted any pass as matched, so a gateway releasing after the ten-minute expiry was never seen; a pass now matches only up to its `expires_at` plus governed `assess.release_skew_s` (60, max 120), and a later release is exception `pass_expired`. List freshness was redefined with a two-hour grace, which let a cross-border payment pass on a freeze list up to 14 hours old; the phase 14 rule `ListRecord::stale` is used unchanged and setting `assess.list_grace_hours` is gone. The hold alert was an `adhoc` run (a person's name search in phase 14) on a subject phase 14 cannot resolve, and post-event screening alerted the same match again; it is now a system `payment` run with `requested_by` NULL, with two requirements on phase 14 (Open doubts 29). The shadow-to-advisory criterion "no release exception" could never fail (shadow rails are not controlled) and is dropped. `akr_assess` reads only `(id, kind, status, expires_at)` of `admin.users` and may update `node_status.started_at`. The six jobs do nothing while `pack.assess` is off, and the outage watch needs a rail with an open mode, so an install without phase 2 records no outage. The roll-ups count `coalesce(would_be, decision)`, so advisory and enforcing days are not zero. PEP and internal hits hold only when their list kind is in governed `assess.hold_list_kinds` (default freeze only); otherwise they add `LIST_NOTED` to a pass.

## Open doubts (for gap pass 1 and the bank)
""")
rep("1. **Zone for OpenBao.** Phase 00 gives `akr-assess` no OpenBao network; this phase needs `z1_bao` for the start-up unwrap and the key reads. The bank approves the change to `deploy/zones.yaml` and to the probe matrix of `make zone-check`, or the unwrap moves to a sidecar in `z1_bao` that hands the seed over a local socket (not designed here).",
    "1. **Zone for OpenBao (lead decision).** `akr-assess` joins `z1_bao` for the start-up unwrap and the key reads; gap pass 1 amends phase 00 (compose networks, `deploy/zones.yaml`, the probe matrix of `make zone-check`). The bank approves the zone change at installation.")
rep("2. **Phase 00 amendments.** The transit key `akr-sign-decision` (phase 00 line 588, \"used from phases 02, 06, 12 and 29\") is not used by this phase: the lead amends that line to say it is reserved and unused, and amends the `akr-assess` AppRole line (section 4.4). The bank's OpenBao policy file is changed by the bank's operator.",
    "2. **Phase 00 amendments (lead decision).** Gap pass 1 amends phase 00 line 588: the transit key `akr-sign-decision` is reserved and unused (decisions are signed in process by a seed wrapped under `akr-kek`, DR-294); and the `akr-assess` AppRole line gains transit decrypt on `akr-kek` and KV read on `keys/assess-*` (section 4.4). The bank's OpenBao policy file is changed by the bank's operator.")
rep("hold that is released after the alert became `potential` does not create a freeze. The bank confirms that a potential match on a hold is held at the gateway until the true or false decision.\n",
    "hold that is released after the alert became `potential` does not create a freeze. The bank confirms that a potential match on a hold is held at the gateway until the true or false decision.\n"
    "29. **Phase 14 requirements of the hold alert.** Gap pass 1 adds to phase 14: the subject view of an alert whose run is a hold `payment` run (`trigger_ref` = an `assess.holds` id) reads the party names from `assess.holds.party_names_enc` with the phase 14 masking; the `subject_id` comment of `alert.screening_runs` widens from adhoc runs to these runs; and `screening_payment` skips an entry already alerted on a hold whose decision has the same match key. Phase 14 also gains: an unchanged feed (`list.feed_unchanged`) renews the freshness clock, else an unchanged list goes stale and holds every cross-border payment.\n")

open(p,'w').write(s)
print('ok')
