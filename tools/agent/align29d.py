p='docs/plan/roadmap.md'
s=open(p).read()
SV="`AssessmentService`, `MessageParser`, `PolicyCache`, `DecisionSigner`, `GatewayAuthenticator`, `GatewayRateLimiter`, `HoldService`, `HoldSweepService`, `FailPolicyService`, `RailModeService`, `ProhibitedRuleService`, `ReleaseControlService`, `ControlTestService`, `RollupService`, `OutageService`, `FallbackExtractService`, `AssessReadService`"
R=[
("| 29 | Assessment API | `akr-assess` in the gateway zone: mTLS per gateway, `/assess/v1` pass/hold/reject, Ed25519-signed expiring decisions, fail policies and modes, holds, inline screening on the shared index | L | P2 | 14, 16, 20 | Planned |",
 "| 29 | Assessment API | Optional pack (flag `pack.assess`, off by default); `akr-assess` in the gateway zone: gateways bound by a pinned certificate and a service principal, `/assess/v1` pass/hold/reject with byte-exact replay from its own ledger, Ed25519 decisions signed in process (seed wrapped under `akr-kek`) and expiring after 10 minutes, persisted before the answer; inline screening on the phase 14 index with the phase 14 freshness rule; modes shadow, advisory and enforcing per rail with governed entry criteria; fail policies with amount bands, modes and prohibited rules (closed grammar) as `cr_material` change targets; holds decided by two people, list holds as phase 14 alerts (system `payment` runs), freeze through phase 14 and 20; release-without-decision control on `canon.payment_message` (expired passes raise `pass_expired`); outage watch, weekly control tests, signed list extract for the bank's fallback | L | P2 | 14, 16, 20 (also 00-04, 09, 10, 17, 23, 24, 26) | Spec ready |"),
("| 29 | `assess.fail_policies`, `assess.decisions`, `assess.holds` |",
 "| 29 | `assess.rails`, `gateways`, `signing_keys`, `decisions` (partitioned), `idempotency`, `node_status`, `blind_periods`, `policy_state`, `amount_bands`, `fail_policies`, `rail_modes`, `prohibited_rules`, `holds`, `hold_candidates`, `control_state`, `release_exceptions`, `control_tests`, `mode_rollups`, `fallback_extracts` (phase 24 `registers.source` CHECK widened by `assess_control`) |"),
("| 29 | `AssessmentService`, `FailPolicyService`, `DecisionSigner` |",
 "| 29 | "+SV+" |"),
("| 29 | next numbers (from 0091) | 26 |",
 "| 29 | `0091_assess_registry`, `0092_assess_policy_holds`, `0093_assess_guards_grants`, `0094_assess_settings_classes` | 26 |\n| 30 | next numbers (from 0095, only if needed) | 27 |"),
]
for a,b in R:
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
row=[l for l in s.split('\n') if l.startswith('| 0 (B8 review: phase 28)')][0]
new=("| 0 (B8 review: phase 29) | 2026-10-09 | 9 lead corrections to the Haiku draft | 0 | 0 | 3 | 6 | MEDIUM: the release "
"control counted any pass as matched, so a release after the ten-minute expiry went unseen (governed `assess.release_skew_s`, "
"exception `pass_expired`); list freshness was redefined with a two-hour grace beyond the CBI 12-hour rule (phase 14 "
"`ListRecord::stale` unchanged, `assess.list_grace_hours` removed); the hold alert was an `adhoc` run on a subject phase 14 "
"cannot resolve and post-event screening alerted the same match again (system `payment` run, two phase 14 requirements in "
"Open doubts 29). LOW: a readiness criterion that could never fail (dropped); `akr_assess` read all of `admin.users` (column "
"grant); every install without phase 2 would record a permanent outage (jobs idle while `pack.assess` is off, the watch needs "
"a rail with a mode); no grant for `node_status.started_at`; roll-ups counted zero outside shadow (`coalesce(would_be, "
"decision)`); every PEP and internal hit held a payment (governed `assess.hold_list_kinds`, default freeze only) |")
assert 'B8 review: phase 29' not in s
s=s.replace(row+'\n',row+'\n'+new+'\n')
open(p,'w').write(s)
print('ok')
