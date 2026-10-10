def edit(p, pairs):
    s=open(p).read()
    for a,b in pairs:
        assert s.count(a)==1,(p,s.count(a),a[:90]); s=s.replace(a,b)
    open(p,'w').write(s)

edit('docs/plan/roadmap.md', [
("| 30 | Profile M/L & High Availability | K3s with Helm charts, default-deny NetworkPolicies, automated PostgreSQL failover (Patroni or CloudNativePG by proof of concept), failover drills | M | P2 | 29 | Planned |",
 "| 30 | Profile M/L & High Availability | One catalogue renders the profile S compose bundle and the M/L Helm chart (drift check); three K3s servers (Traefik and ServiceLB disabled, Caddy edge); zones as pod labels with default-deny NetworkPolicies rendered from `zones.yaml` and a cluster zone check; PostgreSQL failover with strict synchronous commit (Patroni with pgBackRest provisional, CloudNativePG by a 10-criteria proof of concept); OpenBao automatic unseal; Valkey single instance with the gateway limiter falling back in process; job harness lease heartbeat and cluster-wide concurrency; outage recorded as a phase 11 blind period; quarterly failover drill and `restore verify`; RTO/RPO per profile; signed charts and Kyverno admission; no route, no table | M | P2 | 29 (also 00-03, 05, 11, 16, 23, 25) | Spec ready |"),
("| 30 | _(infrastructure)_ |",
 "| 30 | _(infrastructure)_ `DeployRenderer`, `ClusterZoneCheck`, `RestoreVerifier`, `FailoverDrillRecorder` (akr-ctl); `HaProbe`, `StandbyPool` (akr-worker); `LimiterFallback` (akr-assess); `JobStore::heartbeat` and the cluster-wide claim (phase 00 harness) |"),
("| 30 | next numbers (from 0095, only if needed) | none (infrastructure) |",
 "| 30 | `0095_ops_ha_settings` | 27 |\n| 31 | next numbers (from 0096) | 28 (when written) |"),
])
s=open('docs/plan/roadmap.md').read()
import re
m=re.search(r'^\| 0 \(B8 review: phase 29\) \|.*\n',s,re.M); assert m and 'B8 review: phase 30' not in s
row=("| 0 (B8 review: phase 30) | 2026-10-09 | 7 lead corrections to the Haiku draft | 0 | 0 | 2 | 5 | MEDIUM: the read-only DR "
     "workspace could not work, because sign-in writes Kratos sessions and audited reads write `audit.access_event` (dropped, "
     "Open doubts 31); the cluster-exclusive claim covered a hand-picked list of kinds (every kind's concurrency now holds "
     "cluster-wide under an advisory lock on the kind). LOW: outage writer through phase 11 `BlindPeriodService`; restore check "
     "used a non-existent `akr-ctl jobs run` (phase 12 `recon_run` jobs); `akr_policy` is phase 03; `ops.failover_max_s` is "
     "`duration_s`; `LimiterFallback` divides the rate by the replica count |\n")
s=s[:m.end()]+row+s[m.end():]
open('docs/plan/roadmap.md','w').write(s)

edit('docs/DATA-MODEL.md', [
("fallback extracts (0092); guards and grants (0093); the widened `registers.source` CHECK, the `pack.assess` flag, settings, classification and retention (0094) | 29 |\n",
 "fallback extracts (0092); guards and grants (0093); the widened `registers.source` CHECK, the `pack.assess` flag, settings, classification and retention (0094) | 29 |\n"
 "| 27 | `admin`: the widened `admin.releases.refusal_code` CHECK (`chart_signature_invalid`) and five settings (`ops.last_failover_drill_at`, `ops.failover_drill_max_age_days`, `ops.failover_max_s`, `ha.standby_reads`, `ha.standby_max_lag_s`) (0095); no table (phase 30 is infrastructure) | 30 |\n"),
])

edit('docs/MODULES.md', [
("**Requirements:** TS-01..TS-43 (platform, owner), SP-03, SP-64..SP-66, SP-83, SP-85, SP-87.",
 "**Requirements:** TS-01..TS-43 (platform, owner), SP-03, SP-64..SP-66, SP-83..SP-88."),
("shipped in the signed bundle; ops tiles for SIEM lag and dead letters, release currency, open support sessions and\nactive break-glass grants; screens A4 and X5.\n",
 "shipped in the signed bundle; ops tiles for SIEM lag and dead letters, release currency, open support sessions and\nactive break-glass grants; screens A4 and X5.\n"
 "Phase file: docs/plan/phase-30.md (DR-317 to DR-337; migration 0095, no table, no route): profile M/L on three K3s\n"
 "servers from one catalogue that also renders the profile S compose bundle; zones as NetworkPolicies rendered from\n"
 "`deploy/zones.yaml` with a cluster zone check; PostgreSQL failover with strict synchronous commit (Patroni with\n"
 "pgBackRest provisional, proof of concept decides); OpenBao automatic unseal; job harness lease heartbeat and\n"
 "cluster-wide concurrency; database outages recorded as phase 11 blind periods; quarterly failover drill, `akr-ctl\n"
 "restore verify`, RTO/RPO per profile; signed charts and Kyverno admission; eight A3 tiles. The read-only DR workspace\n"
 "is not carried (Open doubts 31).\n"),
])
print('ok')
