import re
def edit(p, pairs):
    s=open(p).read()
    for a,b in pairs:
        assert s.count(a)==1,(p,s.count(a),a[:90]); s=s.replace(a,b)
    open(p,'w').write(s)

# status
edit('docs/plan/status.md', [
("phases 09 to 29 (batches B1 to B8; phases 19, 20, 21 and 23 are B6, phases 22, 24, 25 and 26 are B7, phases 27, 28 and 29 are B8)",
 "phases 09 to 30 (batches B1 to B8; phases 19, 20, 21 and 23 are B6, phases 22, 24, 25 and 26 are B7, phases 27 to 30 are B8)"),
("| 30 | Profile M/L & High Availability | Planned | | | 0 | 0 | 0 | infra |",
 "| 30 | Profile M/L & High Availability | Spec ready | | | 0 | phase 25 `admin.releases.refusal_code` CHECK widened by `chart_signature_invalid` (0095, with five settings); phase 00 job harness (lease heartbeat, cluster-wide concurrency, database-clock slots), listener reload on reconnect, `akr-assess` readiness without Valkey; phase 02 owner kind `ops_drill`; phase 16 `cost` keys | 0 | 7 (DeployRenderer, ClusterZoneCheck, RestoreVerifier, FailoverDrillRecorder, HaProbe, StandbyPool, LimiterFallback) |"),
("| Phases specified | 0 | 30 | 32 |", "| Phases specified | 0 | 31 | 32 |"),
("| 0 | 590 (some partial; `python3 tools/idcount.py 29`, phases 00-29:", "| 0 | 596 (some partial; `python3 tools/idcount.py 30`, phases 00-30:"),
])

# README
edit('docs/README.md', [
("Write phases 30-31 as Haiku agent drafts in batches (owner's choice, 2026-10-09), each reviewed, corrected and aligned by the lead; then the gap passes (phase-29 reviewed 2026-10-09, DR-292 to DR-316;",
 "Write phase 31 as a Haiku agent draft (owner's choice, 2026-10-09), reviewed, corrected and aligned by the lead; then the gap passes (phase-30 reviewed 2026-10-09, DR-317 to DR-337; phase-29 reviewed 2026-10-09, DR-292 to DR-316;"),
])

# tech-stack
edit('tech-stack.md', [
("| Profile M/L: K3s + Helm | Three K3s servers with embedded etcd; default-deny NetworkPolicies; cert-manager from the bank's CA | Banks that need the phase-2 API highly available; automated PostgreSQL failover (Patroni + pgBackRest or CloudNativePG, chosen in a proof of concept) |",
 "| Profile M/L: K3s + Helm | Three K3s servers with embedded etcd (Traefik and ServiceLB disabled, Caddy stays the edge, WireGuard flannel); default-deny NetworkPolicies rendered from `deploy/zones.yaml`; cert-manager from the bank's CA; Kyverno `verifyImages` admission (sigstore policy-controller the alternative); one catalogue renders compose and the chart | Banks that need the phase-2 API highly available; automated PostgreSQL failover with strict synchronous commit (Patroni + pgBackRest the provisional default, CloudNativePG only if it wins the proof of concept; phase 30 DR-320) |"),
])

# conventions
edit('tools/agent/b3-conventions.md', [
("(DR-01..DR-316 in docs/plan/research.md)", "(DR-01..DR-337 in docs/plan/research.md)"),
("grep its real DDL in docs/plan/phase-00..29.md.", "grep its real DDL in docs/plan/phase-00..30.md."),
("  `assess_control`; flag `pack.assess`; Cedar D0_assess.cedar (D0 taken); migrations 0091-0094 (next 0095).\n",
 "  `assess_control`; flag `pack.assess`; Cedar D0_assess.cedar (D0 taken); migrations 0091-0094 (next 0095).\n"
 "- (phase 30 review) Enforce a limit for every case by a rule, not for a hand-picked list: cluster-wide concurrency\n"
 "  listed twelve kinds and left the rest running once per replica.\n"
 "- (phase 30 review) A read-only copy cannot serve anything that writes on the way: sign-in (Kratos sessions) and\n"
 "  audited reads (`audit.access_event`) both write.\n"
 "- (phase 30 review) Write another phase's table through its owning service (phase 11 `BlindPeriodService`), which\n"
 "  carries rules such as never reaching back past the newest anchor.\n"
 "- (phase 30 review) A per-node fallback of a shared limit divides the limit by the replica count.\n"
 "- Phase 30: no table, no route; migration 0095 (refusal code `chart_signature_invalid`, settings `ops.last_failover_drill_at`,\n"
 "  `ops.failover_drill_max_age_days`, `ops.failover_max_s`, `ha.standby_reads`, `ha.standby_max_lag_s`); next 0096.\n"),
])

# gap items
s=open('tools/agent/gap-items.md').read()
assert '(z) Phase 30' not in s
s=s.rstrip('\n')+"\n" + (
"(z) Phase 30: phase 00 job harness (lease heartbeat every third of the kind's lease, cluster-wide concurrency under\n"
"`pg_advisory_xact_lock(hashtext(kind))`, database-clock scheduler slots; DR-337), listener reload on reconnect for\n"
"`akr_assess_policy`, `akr_assess_gateway`, `akr_source`; phase 00 and phase 29 readiness of `akr-assess` lose Valkey;\n"
"phase 01 limiter fails open with `akr_rate_limit_unavailable_total`; phase 02 owner kind `ops_drill` and five audit\n"
"kinds; phase 03 eight A3 tiles; phase 25 bundle `chart/` and `k3s/`, refusal code, support bundle excludes cluster\n"
"objects; phase 16 `cost` keys `read_source`, `replay_lag_s`; phase 00 AppRole `akr-migrate` and `zones.yaml` keys\n"
"`cluster.apiserver`, `cbs.listeners`, `gateway.cidrs`; ARCHITECTURE D19 and research 32 16.6 two-site split rejected\n"
"(DR-332, Open doubts 3); DR workspace not carried (Open doubts 31); MODULES M20 requirements now SP-83..SP-88.\n")
open('tools/agent/gap-items.md','w').write(s)
print('ok')
