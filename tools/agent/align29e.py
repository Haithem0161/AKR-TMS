p='docs/plan/status.md'
s=open(p).read()
R=[
("phases 09 to 28 (batches B1 to B8; phases 19, 20, 21 and 23 are B6, phases 22, 24, 25 and 26 are B7, phases 27 and 28 are B8) reviewed 2026-10-09.",
 "phases 09 to 29 (batches B1 to B8; phases 19, 20, 21 and 23 are B6, phases 22, 24, 25 and 26 are B7, phases 27, 28 and 29 are B8) reviewed 2026-10-09."),
("| 29 | Assessment API | Planned | | | 3 | | | 3 |",
 "| 29 | Assessment API | Spec ready | | | 19 | phase 24 `registers.source` CHECK widened by `assess_control`; phase 14 `ScreeningService::open_hold_alert`, the hold-alert subject view from `assess.holds.party_names_enc`, `screening_payment` skipping an entry already alerted on a hold, and `list.feed_unchanged` renewing freshness; phase 02 owner kind `assess_extract` and the PartitionRegistry entry; phase 03 `Ctx.via` value `gateway_cert`; phase 23 decision partitions dropped only after their holds; phase 24 review-calendar row `assess_control_test` and finding closure closing release exceptions; phase 25 SIEM and support-bundle exclusions; phase 00 `akr-assess` in `z1_bao`, `akr-sign-decision` reserved and unused, AppRole transit decrypt on `akr-kek` (gap pass 1) | 18 (12 owned, 2 added, 4 gateway) | 17 (AssessmentService, MessageParser, PolicyCache, DecisionSigner, GatewayAuthenticator, GatewayRateLimiter, HoldService, HoldSweepService, FailPolicyService, RailModeService, ProhibitedRuleService, ReleaseControlService, ControlTestService, RollupService, OutageService, FallbackExtractService, AssessReadService) |"),
("| Domain tables (specified) | 0 | 249 |","| Domain tables (specified) | 0 | 268 |"),
("| Routes | 0 | 531 |","| Routes | 0 | 549 |"),
("| Phases specified | 0 | 29 |","| Phases specified | 0 | 30 |"),
("| 0 | 580 (some partial; `python3 tools/idcount.py 28`, phases 00-28:","| 0 | 590 (some partial; `python3 tools/idcount.py 29`, phases 00-29:"),
]
for a,b in R:
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
a=[l for l in s.split('\n') if l.startswith('| `assistant_monitor` | 28 |')][0]+'\n'
jobs=("| `assess_release_match` | 29 | `schedule.assess_release_match` (every 15 minutes): released outgoing payments of advisory and enforcing rails from `canon.payment_message` matched to decisions; exceptions `no_decision`, `no_match_key`, `pass_expired`, `decision_not_pass` to the phase 24 register (source `assess_control`); idle while `pack.assess` is off |\n"
"| `assess_hold_sweep` | 29 | `schedule.assess_hold_sweep` (every minute): expires holds, opens the phase 14 alert of a list hold as a system `payment` run, syncs alert outcomes, enqueues the extract when the freeze-list versions change; idle while `pack.assess` is off |\n"
"| `assess_blind_watch` | 29 | `schedule.assess_blind_watch` (every minute): opens and closes outage periods from node heartbeats; idle while `pack.assess` is off or no rail has an open mode |\n"
"| `assess_rollup_watch` | 29 | `schedule.assess_rollup_watch` (00:40): daily counts per rail and mode (`coalesce(would_be, decision)`), hold counts and watch warnings (key age, certificate expiry, extract age, open outage, control test overdue, repeats); idle while `pack.assess` is off |\n"
"| `assess_fallback_extract` | 29 | `schedule.assess_fallback_extract` (05:30) and enqueued by the hold sweep: the signed freeze-list extract for the bank's fallback, valid at most 12 hours; idle while `pack.assess` is off |\n"
"| `assess_ledger_sweep` | 29 | `schedule.assess_ledger_sweep` (hourly): deletes replay-ledger rows older than `assess.idempotency_hours`; idle while `pack.assess` is off |\n")
assert '`assess_release_match` | 29' not in s
s=s.replace(a,a+jobs)
open(p,'w').write(s)
print('ok')
