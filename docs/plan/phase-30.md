# Phase 30: Profile M/L & High Availability

**Goal:** The phase 2 profile (and the rest of AKR-TMS) runs on a three-server K3s cluster (profile M) from the same images and the same catalogue as the profile S compose bundle, with zones enforced as NetworkPolicies rendered from `deploy/zones.yaml`, PostgreSQL failover with synchronous commit, OpenBao and the object store that restart without a human unseal, measured failover and restore drills recorded in the audit trail, and the profile S bundle left unchanged.

**Dependencies:** Phase 00 (compose bundle and its services, `deploy/zones.yaml`, `make zone-check`, `akr-ctl` commands, the job harness and loops, `SettingsService`, `backup_check`, readiness rules, bundle build and verify, `db migrate` as `akr_migrate`), Phase 01 (sessions and the Valkey limiter), Phase 02 (audit chain, stream lock, `create_partition` factories, evidence owner registry), Phase 03 (`OpsCockpit::snapshot()`, `ops_probe`, `GET /admin/ops`), Phase 05 and Phase 11 (`akr-ingestd` listeners and `admin.blind_periods`), Phase 16 (standby reads, its open item 26), Phase 17 (counts stream answers 503 without Valkey), Phase 23 (`akr-ctl retention replay` and the restore readiness gate), Phase 25 (`admin.releases`, `ReleaseService`, `release_verify`, A4, support bundles), Phase 29 (`akr-assess`, `assess.node_status`, `assess_blind_watch`, `PolicyCache`, `GatewayCache`, the decision path)

**Macro-phase:** P2 (optional per bank; profile S stays the default and is unchanged; no rail is assessed until the bank sets its mode, phase 29)
**Complexity:** M (0 routes, 0 new tables, 0 new Cedar files, 0 new job kinds, 0 new outbox topics, 0 new timers, 1 migration `0095` with 5 settings rows and one widened CHECK, 5 new audit event kinds, 1 reused phase 11 kind, 7 new akr-ctl commands (`deploy render`, `deploy upgrade`, `net check --cluster`, `ops record-failover-drill`, `restore verify`, `ha outage record`, `host check`), 21 DR numbers DR-317 to DR-337)
**Binaries touched:** akr-api (the `OpsCockpit` tiles of section 3.13; no route) | akr-worker (`ops_probe`, `backup_check`, `release_verify` extended; the job harness heartbeat and cluster-wide concurrency of section 3.5; the standby pool of section 3.7) | akr-ingestd (listener re-read on reconnect; still one replica) | akr-assess (readiness without Valkey, in-process limiter fallback, policy and gateway cache re-read on reconnect; two replicas on M) | akr-ctl (`deploy render`, `net check --cluster`, `ops record-failover-drill`, `restore verify`, `ha outage record`, `host check`; `bundle verify` gains the chart check)

**Requirements carried:**
- Implemented here: TS-29 (the Helm half of phase 00's TS-29: one catalogue and one generator for compose and the chart, a three-server K3s cluster with embedded etcd; sections 3.1 to 3.2), TS-24 (a node restart of the phase 2 path needs no manual unseal; section 3.6), TS-32 (the PostgreSQL HA proof-of-concept plan with pass criteria, a provisional default and what changes if the other tool wins; sections 3.4 and 6.2), TS-33 (profiles S, M and L as planning assumptions replaced by load tests; the M load test and the M failover drill; sections 3.15 and 6.3), TS-28 (cluster and database role metrics, replication lag, failover count, etcd health, certificate expiry, SLOs and alerts as code, A3 tiles; section 3.13), TS-30 (charts and K3s artifacts in the signed bundle, admission that refuses unsigned images; sections 3.11 and 3.12), TS-39 (failover drill and restore check recorded in the audit trail, monthly restore rehearsal on profile M; section 3.9), TS-40 (NTP on every node and a pre-flight check; section 3.2), SP-04 (zone rules rendered as NetworkPolicies from `deploy/zones.yaml`, a cluster zone check whose result equals the matrix; section 3.3), SP-83 (the proposed RTO and RPO per profile for the bank's BIA; section 3.8), SP-85 (a quarterly failover drill with six chaos cases and pass criteria; section 3.9), SP-86 (post-restore verification as one command, `akr-ctl restore verify`; section 3.9), SP-87 (AKR-TMS-down behaviour per function and the recorded database outage; section 3.8), SP-88 (ransomware restore steps, verified by the restore check; section 3.9)
- Connected to earlier phases (reused, not repeated): SP-84 (the phase 00 backup policy; the second repository at the DR site is added here, section 3.8), TS-27 (phase 00 metrics and redaction filter; section 3.13), TS-31 (one more CI gate, the render drift check of section 3.1)
- Not carried: TS-25 and SP-14 (SAML and SCIM, phase 31), the hosted multi-tenant option (research 32 section 17), the stretched two-site cluster of research 32 section 16.6 (rejected in DR-332), a sizing of profile L beyond its planning shape (section 3.15; the load test decides it), the read-only DR workspace of research 30 section 16.6 (Open doubts 31)
- Gap pass 1 (coverage), not carried: VA-44 (hosted shared service; P2, DR-402).

---

## 1. SQLx Migration Changes

One migration, `0095_ops_ha_settings.sql`, the next free number (phases 26 to 29 used `0083` to `0094`; a grep of docs/plan confirms `0095` is free). It creates no table, so it needs no classification row and no retention row (phase 00 requires them for tables only). It changes two things and nothing else:

1. Widens the CHECK of `admin.releases.refusal_code` (phase 25 section 1.2) by `chart_signature_invalid`, because the release verification of section 3.12 checks the Helm charts (DR-334).
2. Adds the failover drill settings that section 3.9 reads (DR-328) and the two standby read settings of section 3.7 (DR-327). Settings rows are the valid home here: the drill record is a setting written by a CLI command, exactly as phase 00 writes `ops.last_restore_drill_at`.
3. No change to `admin.blind_periods` (phase 11), whose CHECK already holds `akr_outage` and scope `system` (section 3.8), and no change to any other table.

```sql
-- migration 0095_ops_ha_settings.sql
-- Phase 30. Widens the phase 25 refusal code list by chart_signature_invalid, adds the failover drill settings
-- (written by akr-ctl ops record-failover-drill, the pattern of ops.last_restore_drill_at in phase 00) and the standby
-- read settings of scenario runs (phase 16 open item 26). No table is created. Idempotent.

ALTER TABLE admin.releases DROP CONSTRAINT IF EXISTS releases_refusal_code_check;
ALTER TABLE admin.releases ADD CONSTRAINT releases_refusal_code_check
    CHECK (refusal_code IS NULL OR refusal_code IN (
        'bundle_signature_invalid', 'image_signature_invalid', 'manifest_digest_mismatch',
        'fingerprint_mismatch', 'sbom_missing', 'rollback_plan_missing', 'notes_missing',
        'version_not_newer', 'policy_bundle_hash_mismatch', 'help_article_invalid', 'chart_signature_invalid'));

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('ops.last_failover_drill_at', 'null', 'timestamp', NULL, TRUE,
  '{"ar": "تاريخ آخر تمرين فشل تلقائي ناجح", "en": "Last passed failover drill"}'),
 ('ops.failover_drill_max_age_days', '92', 'integer',
  '{"max": 92, "basis": "SP-85: a failover drill at least once a quarter"}', TRUE,
  '{"ar": "أقصى مدة بين تمارين الفشل التلقائي (أيام)", "en": "Maximum days between failover drills"}'),
 ('ops.failover_max_s', '60', 'duration_s',
  '{"max": 300, "basis": "research 30 16.4: about five minutes RTO for profiles M and L; a larger budget hides a long gateway outage"}', TRUE,
  '{"ar": "أقصى زمن مقبول لتبديل قاعدة البيانات (ثوان)", "en": "Maximum accepted database failover time (seconds)"}'),
 ('ha.standby_reads', 'false', 'boolean', NULL, TRUE,
  '{"ar": "قراءة تشغيلات السيناريوهات من النسخة الاحتياطية (M و L)", "en": "Scenario runs read from the standby (profiles M and L)"}'),
 ('ha.standby_max_lag_s', '60', 'duration_s',
  '{"max": 300, "basis": "a run on a standby older than this reads data the primary has already changed; above it the run uses the primary"}', TRUE,
  '{"ar": "أقصى تأخر للنسخة الاحتياطية في تشغيلات السيناريوهات (ثوان)", "en": "Maximum standby lag for scenario reads (seconds)"}')
ON CONFLICT (key) DO NOTHING;
```

Checks on this SQL (section 6.1): the constraint name is the PostgreSQL default for the inline CHECK of phase 25, and `\d admin.releases` must show it once after the migration (if the name differs, the `DROP ... IF EXISTS` does nothing and the new list is added beside the old one, which the PoC catches). The settings keys are new: `grep -rn "ops.failover\|ha\.standby" docs/plan` finds nothing outside this phase. The `ha.` prefix is new in this phase; the bank reads it as the high-availability settings group.

## 2. DDD Implementation (Rust)

No new domain entity, repository or route. The phase adds four small units, each in the crate that already owns its behaviour (names checked with grep across docs/plan; none exists before this phase):

- **Crate placement.** `ctl` (phase 00 workspace): module `deploy` (`DeployRenderer`, the catalogue reader and the three output writers of section 3.1), module `net::cluster` (`ClusterZoneCheck`, section 3.3), module `restore` (`RestoreVerifier`, section 3.9), command `ops record-failover-drill` (`FailoverDrillRecorder`, section 3.9), command `ha outage record` (uses phase 11 `BlindPeriodService` connecting as `akr_app`, section 7.6; no new repository).
- **Worker** (`akr-worker`): `HaProbe` (inside phase 03 `ops_probe`; reads the cluster and database metrics of section 3.13 and the primary change of section 3.8), `StandbyPool` (a second `PgPool` for scenario runs, section 3.7). The job harness changes of section 3.5 are methods of the phase 00 `JobStore`: `heartbeat(id, locked_by)` and the cluster-wide concurrency of the claim.
- **Assess** (`akr-assess`): `LimiterFallback` (in-process token bucket used when Valkey does not answer; section 3.6). `PolicyCache` and `GatewayCache` (phase 29) gain `reload_on_reconnect` and the 30-second version check; no new type.
- **utoipa and OpenAPI:** none. **Router:** none. **Problem codes:** the API section 6 codes `E_UNAVAILABLE` (503) and `E_NOT_ALLOWED_IN_STATE` (409) only (section 3.16).

---

## 3. Business Logic

### 3.1 One catalogue for compose and Helm (DR-317; TS-29, TS-31)

Phase 00 writes the compose bundle by hand and phase 00 section 4 lists the services. This phase makes the list the input of two generated outputs, so the two deployment forms cannot diverge.

Inputs (both in `deploy/`, the second new in this phase):
- `deploy/zones.yaml` (phase 00): the zone matrix, the networks of every service, the edges, and the external addresses (CBS listeners, bank SMTP, SIEM, the proxy of phase 07, the payment hub CIDRs of section 3.3, and the entry `backup.repository`: the host and port of the pgBackRest repository on the backup server, which section 7.5 reads). It stays the only zone source (phase 00 rule: "no second matrix").
- `deploy/catalogue.yaml` (new): one entry per service with `name`, `image` (repository and pinned digest), `command`, `ports`, `env` (key, source: `literal`, `setting` or `secret_path`), `secret_paths` (OpenBao paths of the role the service uses), `networks` (names from `zones.yaml`), `profiles` (`s`, `m`, `l`, and the `phase2` flag of phase 00 for `akr-assess`), and `replicas` per profile.

Steps of `akr-ctl deploy render --profile s|m|l --out DIR`:
1. Read both files. Refuse a network name in the catalogue that `zones.yaml` does not define; refuse an image without a digest (`@sha256:` required).
2. Write `DIR/compose/` for profile `s` (services of profile `s`, the networks of phase 00 section 4 with `internal: true` where phase 00 says so). The output of the first render must equal the phase 00 bundle byte for byte: that is the acceptance test that profile S is unchanged.
3. Write `DIR/chart/akr/` for profile `m` or `l`: `Chart.yaml`, `values.yaml` (images and digests, replicas, resources of section 3.2, zone labels), and one template set per service kind (Deployment or StatefulSet, Service, PodDisruptionBudget, NetworkPolicies of section 3.3). Third-party charts (cert-manager, Kyverno, the PostgreSQL HA chart chosen in section 3.4, OpenBao, Valkey) are dependencies pinned by version and digest in `Chart.yaml`.
4. Environment keys and secret paths are written once per output from the catalogue. No value of a secret is ever written to a file: an AppRole secret id is a file mounted from a Kubernetes Secret that the operator creates (section 3.12).
5. The committed outputs under `deploy/compose/` and `deploy/chart/` are the reviewed form. CI runs `make deploy-render` and `git diff --exit-code deploy/` (the same pattern as the OpenAPI and Orval drift checks of phase 00 section 4), then `helm lint deploy/chart/akr` and a schema check of the rendered manifests with `kubeconform` against the pinned Kubernetes version of section 3.2.
6. CI also runs a structural comparison for every service that appears in both forms: the same digest, the same environment keys with the same sources, the same secret paths and the same network names. Equal by construction for generated files; the check exists for hand edits that bypass the generator.

Failure: any step refuses with a non-zero exit code and writes nothing (the output directory is written to a temporary name and renamed only when every file is rendered).

### 3.2 The K3s cluster on profile M (DR-318; TS-29, TS-40)

**Shape.** Three K3s servers on three hosts, each with the control plane, embedded etcd and the workloads. No taint on M, so all workloads may run on every server. Profile L (planning shape only, section 3.15) adds three agent nodes for the data workloads, labelled `akr.io/pool=data` and tainted `akr-data=true:NoSchedule`; the application servers carry `akr.io/pool=app`.

**Distribution settings (K3s, pinned to the release in the bundle; research 32 section 16.1 gives v1.37.1+k3s1 as the current line at 2026-09-30).**
- `--disable traefik --disable servicelb`. The edge is the phase 00 Caddy container as a Deployment with `hostPort` 443 on the three servers, behind the bank's virtual IP. Traefik would be a second TLS terminator with its own certificate handling; ServiceLB would publish the same ports a second way. Neither is needed.
- `--flannel-backend=wireguard-native` for encryption of node-to-node pod traffic (the K3s option); verified in the PoC by a capture on the node interface.
- The embedded network policy controller stays enabled (section 3.3 depends on it).
- Secrets encryption at rest enabled (`--secrets-encryption`), so cert-manager's CA key in its Secret is encrypted by the cluster.

**Storage.** No CSI driver on M. PostgreSQL (section 3.4), OpenBao raft (section 3.6) and the object store (section 3.6) each use statically created local PersistentVolumes on the bank's disks, pinned to their node by `nodeAffinity`. Replication is the application's job (Patroni, raft, erasure coding), so shared storage is not needed and would add a second failure domain. The K3s `local-path` provisioner is not used for data: it creates volumes without the bank's encryption and without a fixed disk per node. A CSI driver for a bank SAN is a later decision (Open doubts 2).

**Pod placement and budgets (M; L in brackets where it differs).**

| Workload | Replicas | Anti-affinity | PodDisruptionBudget | Requests and limits (planning, TS-33) |
|-|-|-|-|-|
| PostgreSQL (Patroni, section 3.4) | 3 | required on hostname | maxUnavailable 1 | 4 CPU and 24 GiB request and limit |
| OpenBao raft (section 3.6) | 3 | required on hostname | minAvailable 2 | 0.25 CPU and 512 MiB request, 1 CPU and 1 GiB limit |
| Valkey (section 3.6) | 1 | none | none (single instance) | 0.25 CPU and 512 MiB request, 1 CPU and 1 GiB limit |
| object store (section 3.6) | 3 [3] | required on hostname | minAvailable 2 | 1 CPU and 4 GiB request, 2 CPU and 8 GiB limit |
| akr-api | 3 | preferred on hostname | minAvailable 2 | 0.5 CPU and 512 MiB request, 2 CPU and 1 GiB limit |
| akr-worker | 3 | preferred on hostname | minAvailable 2 | 0.5 CPU and 1 GiB request, 4 CPU and 3 GiB limit |
| akr-ingestd | 1, strategy Recreate | none | none (one CBS connection by design) | 0.5 CPU and 1 GiB request, 2 CPU and 2 GiB limit |
| akr-assess | 2 (DR-318) | required on hostname | minAvailable 1 | 2 CPU and 2 GiB request and limit |
| Kratos and Hydra | 2 each | preferred on hostname | minAvailable 1 | 0.25 CPU and 256 MiB request, 1 CPU and 512 MiB limit |
| Caddy edge | 2 | preferred on hostname | minAvailable 1 | 0.1 CPU and 128 MiB request, 0.5 CPU and 256 MiB limit |
| observability (phase 00 list, `akr-mgmt`) | 1 each | none | none | per phase 00; no change |

Requests equal to limits for PostgreSQL and akr-assess (guaranteed class, so the kernel does not evict the database or the gateway path under node pressure). The values are planning assumptions: section 3.15 and the M load test replace them.

**Node time and pre-flight (DR-335).** Every server runs `chrony` against the bank's NTP sources (TS-40: `TIMESTAMPTZ` everywhere, the database clock orders the audit chain, the node clock sets decision expiry). The installer pre-flight `akr-ctl host check` (new) refuses the install unless: every server reports an offset under 100 ms against the bank's sources; the disks of section 3.2 exist with the expected size; the time to OpenBao and to the database answers; the certificate chain to the bank CA verifies; and the locale and character set checks of phase 00 pass. The check reruns daily from the `backup_check` job (section 4.5, item 6).

**Kubernetes version.** The NetworkPolicy, PodDisruptionBudget and Kyverno versions in the chart are the ones the pinned K3s release supports; `kubeconform` runs against that version (section 3.1 step 5).

### 3.3 Zones as NetworkPolicies (DR-319; SP-04)

**Namespaces and labels.** Two namespaces: `akr` (every application and data workload) and `akr-mgmt` (metrics, traces, log forwarder; phase 00 zone Z5). A pod is a member of each zone it has in `zones.yaml` through a label `akr.io/net-<network>: "true"` (for example `akr.io/net-z2_app`). A pod can be in several zones, which a namespace cannot express, so zones are labels and not namespaces. The bank's PAM, Harbor, the SIEM collector, the payment hub and the CBS are outside the cluster and are reached by `ipBlock` rules.

**Rendered policies.** For each namespace, `akr-ctl deploy render` writes:
1. A default deny of ingress and egress that selects every pod (empty `podSelector`).
2. DNS egress to the cluster DNS on 53 (UDP and TCP) only.
3. One allow per edge of `zones.yaml`: from the pods with the source label to the pods with the destination label on the listed ports. An edge to a service outside the cluster is an `ipBlock` rule with the addresses from `zones.yaml`.
4. The specific rules that the matrix already states: egress to the CBS listener addresses (`cbs.listeners`) only from pods labelled `akr.io/net-z1_cbs` (that is `akr-ingestd`); ingress to `akr-assess` on 8443 only from `gateway.cidrs` (the bank's payment hub); no egress rule at all for `z2_store` (the object store) or `z2_model` (phase 28), so those pods have no egress; no route from `akr-api` to `z4_gateway`.

**Cluster zone check (`akr-ctl net check --cluster`; extends the phase 00 `make zone-check`).** The check creates one probe Job per source membership in `akr` and `akr-mgmt`, labelled with that membership only, which runs the `akr-ctl` image (signed and pinned like every image of the catalogue) and tries every port that the matrix names for every destination, including the ports that must stay closed. Each connect has a timeout of 3 seconds, and a connect that does not complete in that time is observed as closed. The probe's result is written as a table (source, destination, port, expected, observed) and the command exits 0 only when observed equals expected in every row. The matrix comes from `zones.yaml`; the check never keeps its own list. Two sources cannot be probed from inside the cluster and are probed from the bank's network: the payment hub CIDR (`z4_gateway`, from a host in that CIDR) and the CBS listener (`z1_cbs`, from the `akr-ingestd` node's network). Their rows are marked `external` in the table and are a checklist item of the installer, not a pod probe.

**Enforcement proof.** A default-deny that is not enforced passes every positive probe and fails no negative one, so the check also needs a known-bad probe: a probe pod with no membership labels must reach nothing, and the check fails if it does. This row catches a disabled network policy controller (section 3.2).

### 3.4 PostgreSQL high availability (DR-320, DR-321, DR-323; TS-32, SP-84)

**Provisional default: Patroni 4.1.5 on PostgreSQL 17, with pgBackRest 2.59.3.** The reason is backup consistency, which research 32 section 16.5 names as the deciding factor: the DBS runbooks, the phase 00 backup policy (WAL every 60 seconds, weekly full, nightly differential, four fulls, encrypted repository) and the restore drill of section 3.9 are all pgBackRest. CloudNativePG v1.30.1 brings its own backup tooling, so choosing it moves the backup policy and every restore runbook to another tool (section 3.4, the last paragraph).

**Topology.** Three PostgreSQL pods, one per server (section 3.2). Patroni keeps one primary and one synchronous standby (`synchronous_node_count: 1`) and one asynchronous standby. PostgreSQL settings: `synchronous_commit = on` (not `remote_apply`, not `local`), `synchronous_standby_names` as Patroni writes it. Patroni settings: `synchronous_mode: true` and `synchronous_mode_strict: true`. In strict mode the primary never turns asynchronous: when no synchronous standby is up, commits wait, and no failover goes to a copy that is behind. The DCS is the Kubernetes API for the PoC (Patroni's Kubernetes store, configured with the namespace's own ServiceAccount, RBAC limited to its own ConfigMaps and Endpoints); a separate three-node etcd is the alternative the PoC may choose (Open doubts 2). The egress edge `akr-pg -> kube-apiserver:6443` is one `ipBlock` rule in `zones.yaml` (`cluster.apiserver`), and no other edge is added.

**Recovery point.** With one failure (the primary or the synchronous standby), every commit acknowledged to a client exists on the new primary: RPO zero for committed rows. With two failures at once (the primary and the synchronous standby), strict mode keeps writes stopped until a synchronous standby returns, so no acknowledged row is lost and no write is accepted without a synchronous copy. The cost of that choice is availability, which section 3.8 and the gateway fail policy absorb.

**How the application finds the primary.** One Service `akr-pg-rw` selects the pod whose `role` label Patroni keeps as `primary` (Patroni's Kubernetes integration maintains that label). Every write path (`akr_app`, `akr_ingest`, `akr_assess`, `akr_retention`, `akr_audit_writer`, `akr_migrate`) uses this Service; no write path uses an address of a pod. Pool settings for every role (SQLx pool options, set by this phase): `max_lifetime` 30 minutes, `test_before_acquire` on, `acquire_timeout` 5 seconds, and a connection timeout of 3 seconds. A connection to a demoted node answers SQLSTATE 25006 (read-only transaction) on the first write; the pool closes that connection and the call returns `E_UNAVAILABLE` (503) after its retry budget of two attempts within 2 seconds (a code constant, not a bank setting: the budget is a safety bound, not a business rule). A retried write is safe only through the phase 00 idempotency key or the decision ledger of phase 29; a write without one is never retried by the pool.

**Proof-of-concept plan (TS-32; the test is `tests/ha/phase30_failover.sh poc-postgres`, run on a three-VM K3s and on a k3d cluster for the operator's footprint; no result is recorded in this document).** Each candidate (Patroni, CloudNativePG) runs the same steps and must meet every pass criterion:
1. Failover time: kill -9 of the primary's container and of its node (two runs each, five repetitions per run); the time from the kill to the first successful write from a client through `akr-pg-rw`, measured on the client side. Pass: p95 at or below `ops.failover_max_s` (60 seconds, DR-328) and no run above 120 seconds.
2. Data loss under synchronous commit: a writer inserts a numbered row every 10 milliseconds and records each id after its commit returns; the primary is killed during the run. Pass: every recorded id exists on the new primary (zero acknowledged rows lost), and the id sequence has no gap before the kill.
3. Two failures: kill the primary and the synchronous standby together. Pass: no write is accepted by any node until a synchronous standby returns; the first write after the return succeeds; zero acknowledged rows lost.
4. Partition: isolate the primary's node from the other two (drop 5432 and the apiserver port on that node only). Pass: at no time are two nodes writable (the probe of section 3.13 reads `pg_is_in_recovery()` on each node); the isolated node demotes; after healing it rejoins as a replica without a manual step.
5. pgBackRest integration: the stanza that the phase 00 `pgbackrest` service configures is used unchanged; phase 00 names the service and its policy, not the stanza (WAL archive continues after the switchover with a new timeline; `pgbackrest check` passes; a full backup taken on the new primary restores on a scratch PVC). Pass: all three; any change in the backup policy is a failed criterion.
6. Operator footprint: pods, CPU and memory at idle, the number of CRDs, and the minutes a trained operator needs to do a minor upgrade of PostgreSQL with the procedure written down. Pass: a footprint the bank's operators accept, recorded in the PoC report.
7. Air-gapped images: every image of the candidate mirrored into Harbor by digest and deployed with the registry mirror of section 3.12 and no other network. Pass: zero pulls from outside the mirror.
8. Licence and support: Patroni MIT, CloudNativePG Apache-2.0 (research 32 section 16.5); the bank's legal review of the chosen one is a checklist item of its acceptance.
9. Upgrade path: a rolling replacement of the three pods to the next minor PostgreSQL release, with the migrations of section 3.11 run first by the `akr_migrate` hook. Pass: no write refused for more than the failover time of criterion 1.
10. Migrations: `akr-ctl db migrate` runs as `akr_migrate` against `akr-pg-rw` (never as an application role; phase 00 rule). Pass: the migration Job completes against the current primary after a switchover made during the PoC.

**Decision rule.** One rule, which is also how "every pass criterion" above and DR-320 are read. Patroni is the default when it meets all ten criteria, criterion 6 being the acceptance of its footprint by the bank's operators. CloudNativePG becomes the default only when Patroni does not qualify, CloudNativePG meets criteria 1 to 4 and 6 to 10, and the bank accepts a backup tool other than pgBackRest; that acceptance takes the place of criterion 5, which CloudNativePG cannot meet. When both candidates qualify, Patroni stays the default, because it keeps the pgBackRest backups of phase 00 unchanged (research 32 section 16.5: backup consistency decides). If CloudNativePG wins, these change and are listed as work for the phase owner: the backup policy of phase 00 (barman-cloud replaces the `pgbackrest` service and its WAL archive), the restore procedures of section 3.9, the DR repository of section 3.8, and the SP-84 description; the failover behaviour, the Service, the pool settings and the migration Job do not change.

**Profile S.** Profile S keeps the phase 00 manual failover to `postgres-standby` (section 3.10). Synchronous replication on profile S is optional and the bank sets it; when `pack.assess` is on with profile S, the bank must enable it (Open doubts 9).

**Partitions and DDL.** The partition factories (`audit.create_partition`, `feat.create_partition`, `kyc.create_partition`, `canon.create_partition`, and `assess.create_partition` of phase 29) run only on the primary, through their daily maintenance job or the inline retry on SQLSTATE 23514. DDL is WAL-logged and is therefore replicated synchronously with the rows that need the partition, so a switchover never finds a row without its partition. Each factory keeps its own advisory lock (phase 02 uses a fixed key), which is per server; after a switchover the new primary takes it on its first call.

### 3.5 What a switchover does to each loop and to each durable record (DR-322, DR-337)

The table is the answer to "how each loop behaves with two or more replicas" that section 1 of the header asks for. It is checked against the real locking of phase 00 (job claim `FOR UPDATE SKIP LOCKED`, lease reaper, scheduler, outbox dispatch, timer sweep) and phase 02 (stream lock).

| Loop or record | Behaviour across a switchover (and with two or more replicas) | Change in this phase |
|-|-|-|
| Job poll and claim (phase 00 `PgJobStore::claim`) | Two replicas never claim the same row: `FOR UPDATE SKIP LOCKED`. A transaction in flight on the old primary is rolled back; its row is still `queued` on the new primary, because it was never committed as `running`. | None for claim safety. For cluster-wide concurrency see the next row. |
| Concurrency of a kind | `JobKind::concurrency` is a semaphore per process (phase 00 section 3). With two worker replicas, a kind with concurrency 1 can run in both at once. This is a gap of phase 00. | The claim enforces every kind's concurrency across the cluster, with no per-kind flag and no list: in the claim transaction it takes `pg_advisory_xact_lock(hashtext(kind))`, counts the kind's `running` rows, and claims at most `JobKind::concurrency` minus that count. The lock lasts for the claim transaction; the committed `running` rows are what the next claimant counts. The per-process semaphore stays as the local bound. |
| Running job after a crash or a switchover | The `running` row keeps its `locked_at` and is reaped after `jobs.lease_s` (600 seconds, phase 00 setting). A long job (`retention_run` runs up to 120 minutes, phase 23) would be reaped after 10 minutes and started a second time, because phase 00 sets `locked_at` once, at the claim. | The handler refreshes `locked_at` every one third of the kind's lease (`jobs.lease_s`, or the per-kind override of the phase 00 registry that the reaper uses; `JobStore::heartbeat(id, locked_by)`, guarded by `locked_by`). A worker that dies stops refreshing and is reaped at the lease. Handlers stay idempotent (at-least-once, phase 00). |
| Scheduler (phase 00 step 6) | Slots are computed from the cron of `schedule.<kind>` and the coalesce key `<kind>:<slot>`, with the unique index on queued rows. The slot must be computed from the database clock (`now()` in SQL), not from a replica's clock, so a replica with a fast clock cannot enqueue a slot early. | The slot computation reads `now()` from the database. |
| Outbox dispatch (phase 00 `OutboxDispatcher`) | Rows are locked by `FOR UPDATE SKIP LOCKED`; a switchover rolls back the uncommitted dispatch, so the row is dispatched again by the new primary. Delivery is at least once: every `TopicHandler` is idempotent on its outbox id. | None. |
| Timers (phase 00 `TimerService`) | Claim, `fired_at` and the enqueue of `timer_<kind>` are one transaction; a switchover before commit leaves the timer due, and the handler re-reads its owner (phase 00 rule). | None. |
| Replay ledgers (phase 29 `assess.idempotency`; phase 00 `admin.idempotency_keys`) | An in-flight row left by a switchover is taken over after its in-flight lease (phase 29 section 3.3; phase 00 `idempotency.in_flight_lease_s`, 120 seconds). A gateway that retries during the lease gets `E_IDEMPOTENCY_IN_FLIGHT` and retries again. | None. |
| Audit chain and its stream lock (phase 02 section 3) | The stream lock is a transaction-level advisory lock of one server. Locks die with the transaction of the old primary, so the new primary takes the lock on its first emit. The chain head is read from the new primary's `audit_log`, which holds every committed row (synchronous commit, section 3.4). `occurred_at = greatest(clock_timestamp(), head + 1 microsecond)` keeps the order if the old primary's clock was ahead. A row whose commit was never acknowledged is not in the chain on either side. | None. The verification after a switchover is `akr-ctl audit verify --all` (section 6). |
| Anchors (phase 02, `anchor_audit`) | The anchor object is written before its row; a second runner loses on `prev_anchor_id` (unique) and ends with `ChainMoved`. An object written just before the switchover without its row is an orphan that the verifier reports as information only. | None. |
| The phase 02 `anchor_fail:{stream}` marker (in Valkey) | Lost when Valkey restarts (section 3.6). Only the `audit.anchor_recovered` event is missed; the anchor itself runs on the next pass. | Documented; no change. |
| LISTEN and NOTIFY listeners | `akr_jobs` is a wake-up only (the 500 ms poll covers a lost notification). `akr_assess_policy` (phase 29 section 4.2) and `akr_assess_gateway`: notifications sent during the reconnect gap are lost, so each listener reloads its full state on every reconnect and checks the version every 30 seconds. `akr_source` (phase 05, read by `akr-ingestd`): a reconnect re-reads the source state at once; the phase 05 hourly re-read stays. `akr_policy` (phase 03): phase 03 already polls `active_id()` beside its listener, so a reconnect needs no change. | Reload on reconnect added to the three listeners `akr_assess_policy`, `akr_assess_gateway` and `akr_source` (section 4.5). |
| Object lock and evidence (phase 02, phase 23) | Object storage is outside PostgreSQL; a switchover does not change it. Evidence rows are written in the same transaction as the write that needs them (phase 02); a row without its object cannot be committed. | None. |
| Partitions (section 3.4) | DDL is replicated with the rows that need it. | None. |

**Read-after-failover rule.** Every reader that caches an answer from the database (the policy cache of phase 29, the authorisation bundle of phase 03, the settings cache of phase 00 with its 30-second TTL) has a bounded staleness that does not depend on the switchover; no reader needs a different database address.

### 3.6 OpenBao, Valkey, the object store and the identity services (DR-324, DR-325, DR-326; TS-24)

**OpenBao (DR-324).** Three raft members, one per server, in the `akr` namespace, zone `z1_bao` for the consumers and no egress. The seal is automatic, so a restarted member unseals without a human:
1. Preferred: the bank's HSM through the PKCS#11 seal of the OpenBao build the bank approves (research 32 section 13.4 names this option).
2. Otherwise: a transit seal backed by a second OpenBao in the bank's secure zone. The second server has its own seal, and its unseal is a ceremony of the bank (phase 00 Shamir runbook). This moves the human step to a host that restarts rarely; it does not remove it.
3. A static-key seal is not used: the platform's own secret store would then hold the means to unseal the vault.

Restart rule for the phase 2 path (TS-24): a restart of one `akr-assess` pod or one OpenBao member never waits for a human. A restarted `akr-assess` unwraps its decision seed and extract seed from OpenBao at start-up (phase 29 section 3.4), so it needs OpenBao to answer at start-up; when the seal is automatic and OpenBao has a quorum, that is true. While OpenBao cannot answer, a restarting node is not ready, the other node serves, and if both are down the gateway applies its fail policy (section 3.8). Running decisions keep the keys in memory (phase 29), so an OpenBao restart does not interrupt a node that is already running.

**Valkey (DR-325).** One instance, no replica set, restart on failure, on M and L. The reasons are in the table below; the decision is that no state in Valkey is needed for correctness, so losing it costs a sign-out, a counter reset and a cache refill, not a wrong answer.

| Valkey use (phase) | What happens when Valkey is down or restarts | Change in this phase |
|-|-|-|
| Session idle clock `idle:{session_id}` (phase 01) | A missing key counts as expired: every analyst signs in again after a Valkey restart (phase 01 DR-31). Safe (a sign-out, not a grant). | Documented as the cost of the single instance. |
| Rate limits, GCRA (phases 01 and 14) | Counters are lost and the limiter answers. Phase 01 does not state what the limiter does without Valkey (Open doubts 6). | Requirement on phase 01: fail open with a metric `akr_rate_limit_unavailable_total`. |
| Gateway rate limit of `akr-assess` (phase 29 step 3) | Without Valkey, step 3 cannot answer; phase 29 has no rule for it. | `LimiterFallback`: an in-process token bucket per node whose capacity and refill are the gateway's setting divided by the `akr-assess` replica count of the catalogue (rounded up), so the nodes together stay within the gateway rate; used only when Valkey fails; counted in `akr_assess_limiter_fallback_total`. |
| Readiness of `akr-assess` (phase 00 rule; phase 29 section 4.4) | Valkey down makes the node not ready, so a Valkey outage would stop the whole gateway path. | Valkey is removed from the `akr-assess` readiness list (amends phase 00 and phase 29). |
| SSE fan-out and counts (phase 17) | Clients see the stream end and reconnect; the counts stream answers 503 until Valkey answers (phase 17 rule); the snapshot on connect restores the counts. | None. |
| Authorisation bundle instances and resource cache (phase 03) | Resource attributes are loaded from PostgreSQL and the instance registry is refilled within 2 minutes (phase 03 fails open on cache infrastructure). | None. |
| Settings invalidation (phase 00) | The 30-second TTL is the bound. | None. |
| `ops:probe` snapshot (phase 03) | Tiles read `unknown` until the next probe (minute) refills it. | None. |
| Counts cache and per-user caches (phase 17) | Recomputed on the next read. | None. |

**Object store (DR-326).** The object store is the AKR Silo build behind the S3 port (phase 00). Provisional default on M and L: three members with erasure coding on their local volumes, if the store PoC shows that the build supports erasure coding and that `akr-ctl store conformance` (COMPLIANCE object lock, delete refused, versioning) passes with one member stopped. If the build does not support it, the object store runs as one member per site and the phase 00 restic copy every 15 minutes covers its loss; while that member is down, evidence uploads answer 503 `E_UNAVAILABLE` and nothing is written around the lock.

**Kratos and Hydra (DR-326).** Two replicas each, stateless apart from the database (phase 01 and phase 00 rows of the compose table), PodDisruptionBudget minAvailable 1. No change to their configuration.

**Caddy.** Two replicas (section 3.2), so the bank's virtual IP has a second target.

### 3.7 Standby reads for scenario runs (DR-327; phase 16 open item 26)

Phase 16 left "runs use the standby where one exists" (DSL section A.6.1) to this phase. Decision: carried for profiles M and L only, and only for scenario runs, which read history and never write the decision path.

1. `StandbyPool` (akr-worker) is a second pool to `akr-pg-ro` (a Service that selects the pods whose `role` label is `replica`), connecting as `akr_app` with `default_transaction_read_only = on` on the connection. It has no write grant beyond those of `akr_app`; the read-only setting is what keeps it read-only.
2. The scenario run job reads `ha.standby_reads` (governed; default false). When it is true, it measures the replication lag of the chosen replica (`pg_stat_replication` or `now() - pg_last_xact_replay_timestamp()` on the standby) once at the start of the run. Above `ha.standby_max_lag_s` (60 seconds; floor max 300), the run uses the primary.
3. The run records `read_source` (`standby` or `primary`) and `replay_lag_s` in the `cost` object of `scn.scenario_runs` (phase 16 column, JSONB; two keys added, no DDL). The run's result is therefore reproducible in its record: the reader can see which copy it read.
4. Screening, decisions, holds, cases, approvals and every write never use `akr-pg-ro`. The gateway decision path reads only the primary (phase 29), because a decision needs the newest list state and synchronous commit.
5. On profile S the setting has no effect (the pool is not created without the `m` or `l` profile flag in the catalogue).

### 3.8 AKR-TMS is down, and how the outage is recorded (DR-330, DR-331; SP-83, SP-87, SP-84)

**Recording a database outage.** Phase 11 already has the place: `admin.blind_periods` with scope `system` and cause `akr_outage` (phase 11 section 1 DDL), and the audit event `blind_period.opened` / `blind_period.closed` (phase 11). No new table. Two writers, both on the primary:
1. **Automatic, after recovery.** `ops_probe` (phase 03) keeps its last snapshot in Valkey with `taken_at` (a Valkey restart in the same window loses it, and the operator path below covers that case). When the probe runs on the new primary and the previous snapshot is older than `ops.probe_stale_s` (180 seconds, phase 03), or the writable node name changed (the `db_role` tile of section 3.13), it writes a blind period through `BlindPeriodService::record_system` (section 7+; phase 11 section 1.1 defines the table and its section 3.6 owns the anchor rule): `scope = 'system'`, `cause = 'akr_outage'`, `started_at` = the later of the previous snapshot's `taken_at` (the last moment the platform is known to have run) and the newest `audit_anchors.anchored_at` (the phase 11 rule: a period never reaches back past sealed evidence), `ended_at` = the first probe on the new primary, `declared_by` = the system principal, `statement` = `{ "detected_by": "ops_probe", "previous_taken_at": ..., "writable_node_changed": bool }`. The row is created now, so `created_at` is the real time; `started_at` is an estimate that the statement names as one. Audit `blind_period.opened`.
2. **Operator, for an outage the probe could not see.** `akr-ctl ha outage record --from T1 --to T2 --evidence FILE` writes the same row through `BlindPeriodService::record_system` (a `T1` earlier than the newest anchor is refused, the phase 11 rule) with `statement.detected_by = 'operator'` and the evidence file as an evidence object (phase 02, owner kind `ops_drill`, kind `package`, R10). Audit `blind_period.opened`. Used when the cluster event log, not the probe, shows the outage.
3. The gateway side is recorded by phase 29's own watch (`assess.blind_periods`, section 3.13 of phase 29): the nodes cannot write heartbeats while the database is down, so the watch sees the silence and opens its period at the newest `last_seen_at`. That period is separate by design (phase 29 DR-306).

**Behaviour per function while the platform is down (SP-87).** The phase 30 answer for each row of research 30 section 16.6, with what changes.

| Function | Effect in this phase | Interim control | Recovery |
|-|-|-|-|
| Transaction monitoring | No new alerts while the database is down; ingest pauses (phase 11 watermarks) | The bank's existing controls; the MLRO is told by the freshness alert of phase 11 | Backfill from the watermark; alerts for the gap are created late, flagged with the blind period, never back-dated (phase 11) |
| Case work and STR deadlines | Unavailable (`E_UNAVAILABLE`) | Paper records of the bank, keyed in after recovery; goAML is the Office's system (no DR workspace before promotion, Open doubts 31) | Entries made by hand are keyed in with their real times |
| Daily screening and list refresh | Missed days are caught up after recovery (phase 14) | The bank's other screening tool, manual checks on high-risk parties | Catch-up run; more than 24 hours is a compliance gap with a notice (phase 14) |
| Freeze tasks and the 48-hour clock | Clock jobs wait (their timers fire late) | Manual register kept by compliance | Tasks re-created from the audit trail; timer rule (phase 00): re-read owner before acting |
| Phase 2 decisions | 503 from `akr-assess` while no primary is writable; no `pass` leaves a node without its row (phase 29 step 16) | The gateway's fail policy (phase 29 section 3.6) and circuit breaker; the weekly control-test payment (phase 29) | Blind period (above); priority review of anything released in the window (phase 29 section 3.14 exceptions) |
| Audit trail | No state change proceeds without its audit row (phase 02) | Refusal, not silence | Resume when the stream lock can be taken |

**Recovery point and time objectives per profile (SP-83, the proposal for the bank's BIA).**

| Function | Profile S: RTO and RPO | Profile M and L: RTO and RPO | Mechanism |
|-|-|-|-|
| Analyst application, cases, MLRO workspace, evidence | RTO 1 hour (restore onto the second server, the DBS rehearsal); RPO 15 minutes (WAL within 60 seconds, files every 15 minutes) | RTO 5 minutes (automatic database failover within `ops.failover_max_s` and application restart); RPO zero for committed rows | pgBackRest with WAL; synchronous standby; Patroni (or the PoC winner) |
| Keys (OpenBao) | RTO 1 hour; RPO 15 minutes (snapshot) | RTO 5 minutes for a member restart; RTO 1 hour for loss of quorum (restore a snapshot); RPO zero with three members | raft snapshots sealed by OpenBao's barrier, copied to the DR site; unseal shares in sealed envelopes at separate sites (phase 00) |
| Ingestion | Freshness SLO breach at 4 hours; MLRO escalation at 24 hours; RPO none (watermark) | the same | restart and backfill (phase 11); gaps are blind periods |
| Phase 2 assessment | RTO 15 minutes; RPO zero for decisions only when the bank enables synchronous replication (section 3.4), otherwise the replication lag at the failure | RTO 60 seconds of gateway 503 (`ops.failover_max_s`), the gateway fail policy for that window; RPO zero | two nodes, synchronous standby, the gateway's local fallback (phase 29 DR-305) |
| Audit anchors and evidence | RPO 15 minutes; locked copies are independent of the database | RPO zero for the chain (synchronous), 15 minutes for the locked copies | WORM bucket and its replication to the DR site |
| Release bundle and Helm values | RTO 4 hours; RPO last release | the same | the signed bundle at the DR site and on offline media (section 7.7 item 1) |
| Valkey (cache, limits, fan-out) | RTO 5 minutes (restart); RPO none | the same | single instance, restart (section 3.6) |

**DR site (zone Z6, DR-331).** The DR site holds, and never serves unless promoted by the bank:
1. A PostgreSQL asynchronous replica, cascaded from the primary's synchronous set. It is not a synchronous member, so it never slows a commit. Its lag is alerted at 60 seconds and critical at 300 seconds (section 3.13); the RPO of a site loss is that lag.
2. The second repository of the pgBackRest stanza (SP-84), and the offsite copy the phase 00 `restic` service makes of buckets and the OpenBao snapshot. Neither is an object-locked repository in this design; the copy that an attacker on the network cannot change is the offline copy that section 7+ defines (SP-88); object lock is Open doubts 22.
3. The second OpenBao of section 3.6 and the snapshots of the raft members.
4. No workspace before promotion. A read-only `akr-api` on the DR replica cannot work: sign-in writes Kratos sessions to the `kratos` database (phase 01) and every read of personal data writes an `audit.access_event` row (phase 02), so nobody could sign in and every audited read would fail. `akr-api` and `akr-assess` are not deployed at the DR site until promotion (Open doubts 31).
5. Promotion is a runbook, never automatic: the bank decides; the steps are `pg_promote()` on the DR replica, the OpenBao restore and unseal, the object store copy, `akr-ctl restore verify` (section 3.9), the readiness gate of phase 23 (the API stays closed while a replay is pending), and the bank's DNS change. Two sites never accept writes at the same time: the DR replica is read-only until its promotion step runs.

### 3.9 Failover drill, restore check and the monthly restore (DR-328, DR-329; SP-85, SP-86, SP-88, TS-39)

**Failover drill (quarterly, DR-328).** Run on the test cluster before every production drill, and on the production cluster of each bank once a quarter inside the bank's change window. Six chaos cases; each has an expected effect and a pass criterion. The script `tests/ha/phase30_failover.sh` drives them (section 6.3).

| Case | Action | Expected effect | Pass criterion |
|-|-|-|-|
| C1 primary killed | kill -9 the PostgreSQL primary pod and its node's container runtime | Writes stop; a synchronous standby is promoted; the Service follows; `akr-assess` answers 503 while no writable primary exists | time to first successful write at most `ops.failover_max_s`; zero acknowledged rows lost (the writer's id check of section 6.2); the old primary rejoins as a replica without a manual step; `akr-assess` returns 200 with a new committed row after the window |
| C2 node drained | `kubectl drain` of one server | PodDisruptionBudgets respected; OpenBao keeps quorum; `akr-assess` keeps one node serving | drain completes; no 5xx from `akr-api` beyond the Service move (at most 30 seconds); the PostgreSQL primary moves at most once (C1 rules apply if it was on the node) |
| C3 network partition | drop 5432 and the apiserver port on the primary's node, both directions | The isolated primary demotes; the majority elects one primary | at no sample are two nodes writable (`pg_is_in_recovery()` on each node every second); no row is written on the isolated node after the partition starts (the writer's id check); after healing, the node is a replica |
| C4 OpenBao member lost | kill one raft member | Quorum of two continues; the restarted member auto-unseals | no sealed state reported by `ops_probe`; no manual unseal step; `akr-assess` decisions continue with cached keys (phase 29) |
| C5 Valkey lost | kill the Valkey pod | Sessions re-authenticate (section 3.6); counts stream answers 503; `akr-assess` uses `LimiterFallback` | `akr_assess_limiter_fallback_total` rises; no `pass` without a row; Valkey back within 5 minutes; the snapshot on reconnect restores the counts |
| C6 worker killed mid-job | kill a worker pod during a `retention_run` in its dry-run form | The run is reaped at the lease after its heartbeat stops and is planned again | the job completes once; no second concurrent run of the kind (the cluster-wide concurrency of section 3.5); the audit chain verifies |

**Recording a drill (DR-328).** `akr-ctl ops record-failover-drill --result passed|failed --measured-failover-s N --cases-passed K --cases-total 6 --evidence FILE` (the pattern of phase 00 `ops record-restore-drill`):
1. Writes `ops.last_failover_drill_at` through `SettingsService::apply` with `ChangeSource::Direct` and the same CLI-only exemption phase 00 uses for `ops.last_restore_drill_at`, only when `--result passed` and `K = 6`. A failed drill writes no date, so the age tile keeps counting from the last passed one. `--result passed` with `K` below 6 is refused before anything is written (exit 3, section 3.16): no setting, no evidence object and no audit event, so no drill is on record as passed with a failed case.
2. Stores the report (one document per drill, the six case rows with their expected and observed values) as an evidence object: kind `package`, owner kind `ops_drill` (new in phase 02's registry, section 4.5), R10, classification `internal`.
3. Audit `failover_drill.recorded` (siem yes).
4. The age tile (section 3.13) reads `ops.last_failover_drill_at` against `ops.failover_drill_max_age_days` (92, the bank may lower it). The review calendar of phase 24 is not used: a new row kind would be a second CHECK change of phase 24, and the tile plus the audit trail are enough for SP-85 (Open doubts 11).

**Restore check (`akr-ctl restore verify`, DR-329; SP-86, SP-88).** One command runs the post-restore checks in a fixed order and stops at the first failure. It runs on the restored database before the API is opened (the phase 23 readiness gate stays closed until the replay completes):
1. `akr-ctl retention verify-certificates` (phase 23 section 3.7): the deletion certificates newer than the restored database are verified against `akr-sign-deletion` (phase 23 section 7+, DR-357).
2. `akr-ctl retention replay --source worm` (phase 23 section 3.7): the disposals the certificates name are repeated.
3. `akr-ctl audit verify --all` (phase 02) against the locked anchors (`akr-anchors`): the chain and every anchor match. Offline verification (`--offline --pubkeys`) is used on the scratch restore.
4. Keys and clock: every signing key the database references (`assess.signing_keys`, the anchor and certificate keys) is present in OpenBao with the recorded fingerprint; the node offsets of section 3.2 are under 100 ms; the decision key version in use is the one `assess.signing_keys` marks active.
5. Reconciliation: the command lists the business dates of the restore gap, inserts one phase 12 day run per source and date as `recon_sweep` step 1 does (`requested_by` = the system principal) and queues its `recon_run` job (phase 12 section 7.2), uses the active day run where a source and date already has one (a second insert would fail the partial unique index of phase 12), waits for their results, and compares them with the coverage ledger (phase 12 `coverage_ledger`); the blind periods of the gap are listed.
6. Writes the report as evidence (owner `ops_drill`, kind `package`, R10) and audits `restore_check.recorded` (siem yes; details: the step results, counts only).

The ransomware variant (SP-88) is this command run on a clean environment from the offline copy (section 7.7), followed by the rotation of section 3.12's credentials before any node is opened to the bank's users. Its drill is the monthly restore below.

**Monthly restore rehearsal on profile M (TS-39).** Once a month, the restore runs in a separate namespace `akr-restore` on the same cluster (or the DR server), from the latest pgBackRest backup into a scratch PersistentVolume, followed by `restore verify`, then `akr-ctl ops record-restore-drill --result passed --evidence FILE` (phase 00). The phase 00 `backup_check` job keeps the restore age tile (`ops.restore_drill_max_age_days`, 31).

### 3.10 Profile S: the manual failover (unchanged)

Profile S keeps phase 00 exactly: one `postgres` and one `postgres-standby` (streaming replica), no automatic failover, no change to the compose file the phase 00 bundle ships. The runbook command that promotes the standby is:

```
docker compose exec -T postgres-standby psql -U <superuser> -c "SELECT pg_promote(wait => true, wait_seconds => 60);"
```

then the bank updates the database address of the application containers (the database variable of phase 00's bootstrap environment) and restarts `akr-api`, `akr-worker`, `akr-ingestd` and `akr-assess` in that order; then `akr-ctl restore verify` (section 3.9, steps 3 and 6 are enough when no replay is pending) and `akr-ctl ops record-failover-drill` if the drill is the test. The superuser name is the phase 00 bootstrap superuser, which phase 00 does not name (Open doubts 26). When `pack.assess` is on with profile S, the bank enables synchronous replication to `postgres-standby` before the switch-over is possible (Open doubts 9).

### 3.11 Upgrades and rollback on K3s (DR-333; phase 25 rollback plan)

**Order of one upgrade (`akr-ctl deploy upgrade --release <id>`, which runs `helm upgrade --install`):**
1. Verify the bundle first (section 3.12). A refused bundle stops the upgrade before any change.
2. Pre-upgrade hook Job `akr-migrate`: the `akr-ctl` image runs `akr-ctl db migrate --release <id>` as `akr_migrate`, against `akr-pg-rw`. It uses a new AppRole `akr-migrate` (KV read `db/akr_migrate` only), whose secret id the operator issues for this release and deletes after the hook (phase 00 lists AppRoles for the four services and `akr-ctl` only; this one is new). Before the first migration the hook creates a named restore point: `SELECT pg_create_restore_point('akr_release_<version>')`; the phase 00 pgBackRest backup taken before the upgrade is the restore base. The hook refuses production without an approved release id (phase 00 rule).
3. `akr-worker` (three replicas, rolling, `maxUnavailable 1`): first, so the job kinds that the new API enqueues are known to a worker before the API enqueues them. An old worker does not claim a kind it does not know (phase 00 rule), so the order is safe in both directions.
4. `akr-ingestd` (one replica, Recreate): a restart pauses ingestion; the watermark resumes it (phase 11). Not run in parallel with `akr-worker`.
5. `akr-api` (rolling, `maxSurge 1`, `maxUnavailable 0`).
6. `akr-assess` (two replicas, one at a time, `maxUnavailable 1`): each replica must be ready before the next is replaced, and PodDisruptionBudget minAvailable 1 keeps one serving.
7. The edge (Caddy), Kratos and Hydra (rolling).
8. The operator's rollout record: the upgrade script writes `rollout.json` (chart and image digests, Helm release revision, timestamps, per-step result) for the bank to attach to `POST /admin/releases/{id}/applied` (phase 25 section 3.6; phase 25 Open doubts 14 asked for a machine record of the rollout).

**Rollback.** `helm rollback` to the previous revision runs the previous images. The migrations of this release are additive (research 32 section 16.3: "additive where possible"), so the previous code runs on the migrated schema (the code-only rollback uses `migrate --ignore-missing`, as research 32 says). A data migration that cannot run backwards is recovered from the named restore point and the pgBackRest backup, which is an operator decision and is recorded in the release row by phase 25 (`refused` or superseded with a note).

**What a failed upgrade leaves.** Each migration runs in its own transaction (the SQLx default), so a failed migration leaves no partial DDL and is not recorded in `_sqlx_migrations`. Migrations that succeeded before the failure stay applied, and the previous code runs against them. Helm records the release as `failed`; the workers and the API keep the old revision until a step succeeds, because every later step waits for the one before it. The job harness and the outbox keep working across the mixed revisions, since job kinds are only claimed by binaries that know them.

### 3.12 Admission, supply chain and the release bundle (DR-334; TS-30)

- **Bundle (phase 00 section 4, step 7, extended):** the bundle carries the Helm charts (`chart/akr`, the third-party charts of section 3.1, pinned by digest) and the K3s artifacts (the K3s binary, the air-gap images tarball of the pinned release, the install script and its checksum) in directories `chart/` and `k3s/`. The manifest lists their SHA-256 digests like every other file, and cosign signs the chart artifacts the way it signs images (phase 00 step 5).
- **Verification (phase 25 section 7.7, check (g), the same code path as `akr-ctl bundle verify`):** every chart signature must verify; a failure is `chart_signature_invalid`, the new refusal code of migration `0095`. K3s artifacts are covered by the manifest digest check (`manifest_digest_mismatch`), so they need no new code.
- **Release verify job (`release_verify`, phase 25):** adds the chart count and the K3s version to `cosign_result` (counts and digests, no signature bytes) and shows them on A4 (section 5.2).
- **Admission (Kyverno):** Kyverno (the candidate of research 32 section 16.3; its choice is the PoC's, Open doubts 12) with a `verifyImages` policy in `Enforce` mode for the namespaces `akr` and `akr-mgmt`. It requires a cosign signature from the vendor key the bank pinned at install and a digest reference (no tag). A test image without a signature is refused at admission (section 6.1, check 3).
- **Charts at install and upgrade:** the upgrade script runs `cosign verify` on the chart reference before `helm upgrade`. Admission does not verify charts (it verifies container images), so this check is the chart's gate.
- **Air-gap install (research 32 section 16.2):** the K3s installer runs with `INSTALL_K3S_SKIP_DOWNLOAD=true`; `registries.yaml` points every registry name to the bank's Harbor mirror; every image and chart is pushed to Harbor by `akr-ctl bundle import` (phase 00). No node pulls from a public registry. The zone check of section 3.3 includes a deny probe for the public registry addresses of the nodes.
- **Secrets at install:** the AppRole secret ids (`akr-api`, `akr-worker`, `akr-ingestd`, `akr-assess`, `akr-migrate`) are Kubernetes Secrets created by the operator from files, mounted as files (phase 00 `BAO_SECRET_ID_FILE`); no secret value is in `values.yaml`, the catalogue, a ConfigMap or an environment variable of a pod spec.

### 3.13 Observability, SLOs and alerts (DR-336; TS-27, TS-28)

**Metrics added (Prometheus in `akr-mgmt`; scraped through the phase 00 metrics listeners and the cluster exporters).** Names are either the upstream names of the component or derived by `ops_probe` (phase 03), which computes the derived ones from the database and the cluster API:

| Metric | Source | Meaning |
|-|-|-|
| `akr_db_writable_primary` | `ops_probe`: count of PostgreSQL pods with `pg_is_in_recovery() = false` | 1 is correct; 0 is an outage; 2 is a split brain (critical at once) |
| `akr_db_sync_standbys` | `pg_stat_replication` (`sync_state = 'sync'` count), postgres exporter | 0 means writes wait (strict mode); critical after 60 seconds |
| `akr_db_failovers_total` | `ops_probe`: counter of changes of the writable node's name | the failover count |
| `pg_replication_lag_seconds` | postgres exporter (phase 03 name) | replica lag, per replica; the DR replica's lag is its own series |
| `etcd_server_has_leader`, `etcd_server_leader_changes_seen_total` | etcd's own metrics on the three servers | leader presence and churn |
| `certmanager_certificate_expiration_timestamp_seconds` | cert-manager | certificate expiry (phase 03 `cert` tile reads it) |
| `node_timex_offset_seconds` | node exporter (timex collector) | clock offset per node |
| `akr_failover_drill_age_days` | `backup_check` (section 4.5, item 6) | days since `ops.last_failover_drill_at` |
| `akr_assess_limiter_fallback_total` | `akr-assess` | requests limited in-process during a Valkey outage |
| `akr_rate_limit_unavailable_total` | `akr-api` (phase 01 requirement, section 4.5) | rate checks without Valkey |
| the OpenBao raft metrics | the pinned OpenBao build's `/metrics` | member count and leader; names taken from the build in the PoC, not invented here |

**Cockpit tiles (A3, DR-336; extends phase 03 `OpsCockpit::snapshot()`).** Added to the table of phase 03 section 3.10:

| Tile | Source | Warn | Critical |
|-|-|-|-|
| `db_role` | `akr_db_writable_primary` | never warn | not 1 (the value and the node names are shown, no data) |
| `db_sync` | `akr_db_sync_standbys` | 0 for 10 seconds | 0 for 60 seconds |
| `db_failovers` | `akr_db_failovers_total` in 24 hours | at least 1 | at least 3 in 24 hours |
| `replication_lag` | `pg_replication_lag_seconds` (phase 03 tile, now with the DR replica) | over 60 s | over 300 s |
| `etcd` | `etcd_server_has_leader` and churn | leader churn over 5 in 10 minutes | no leader |
| `certificates` | phase 03 tile, now with `certmanager_certificate_expiration_timestamp_seconds` | under `ops.cert_warn_days` (phase 03) | expired |
| `clock` | `node_timex_offset_seconds` | above 100 ms | above 500 ms |
| `failover_drill` | `ops.last_failover_drill_at` and `ops.failover_drill_max_age_days` | older than the maximum | older than twice the maximum |

The thresholds are proposals for the bank's review (Open doubts 15). The tiles are in the `GET /admin/ops` response (phase 03 route, scope `admin.system`); no route is added. A3 shows them under the existing platform health screen (section 5.1).

**SLOs as code (DR-336).** `deploy/observability/slo/*.yaml` (the directory phase 02 already uses for alert rules; the SLO files sit next to the alert rules, ARCHITECTURE section 19). The objectives, all proposals for the bank's SLO review (Open doubts 15):
- `akr-assess` availability 99.9 per cent a month, measured as the share of gateway requests that receive an answer within the gateway's timeout (phase 29 section 3.22 and its Open doubts 15).
- `akr-assess` latency: p95 150 ms and p99 400 ms end to end (ARCHITECTURE section 19; the load test of section 6.3).
- Database: writable primary count equals 1 for 99.95 per cent of minutes.
- Backups: `akr_backup_age_seconds` under 26 hours (phase 00); restore drill age under 31 days; failover drill age under 92 days.

**Alerts.** One rule per tile above at its critical threshold; the failover count alert is a ticket, not a page; the DR replica lag at 300 seconds pages the operator on the DR site.

**Logs and events (TS-27).** Application logs go through the phase 00 redaction filter and reach the SIEM through Fluent Bit (phase 00). The Kubernetes audit log of the API server is forwarded the same way with the audit policy that records requests without request bodies and without secret values. Pod names, node names and labels carry no name, account, case or customer value (section 3.14).

### 3.14 Who may see and touch the cluster; tipping-off (DR-336)

- **People with cluster access:** the bank's operators only, through the bank's PAM in zone Z5 (phase 00 zones). A `kubectl exec` is recorded by the bank's PAM session and by the Kubernetes audit log. The cluster RBAC binds the `akr-operators` group, which the bank's identity provider maps from its operators' groups; no vendor identity is bound to any role (zone Z7 has nothing standing, ARCHITECTURE section 4).
- **What a cluster event, log or description may hold:** resource names, node names, replica counts, image digests, error codes and counts. Not: a name, an account, a case, an alert, a payment reference, a customer or a party. `kubectl describe` of a pod shows the environment keys, never the values (the catalogue writes sources, section 3.1, not values).
- **Support bundles (phase 25 section 3.8):** the bundle does not collect Kubernetes objects, events or pod logs from the cluster. Logs of the application that a support session needs are the redacted bundle of phase 25 only.
- **Metrics:** labels are bounded (node name, gateway key of phase 29, rail code, list code); no label carries a customer, a party or a case.

### 3.15 Hardware profiles and the load test (DR-332; TS-33)

**Planning shapes (research 32 section 16.6, replaced by the load test).**

| Profile | Nodes | Shape | Use |
|-|-|-|-|
| S | 1 server and 1 backup server | compose bundle (phase 00) | pilots and smaller banks; the default |
| M | 3 servers at the primary site (section 3.2), 1 DR server at the DR site (section 3.8) | the cluster of section 3.2 | phase 2 highly available |
| L | 3 application servers and 3 data agents at the primary site, 1 DR server | the same chart with the pools of section 3.2 | a bank whose load test shows the M sizing is not enough |

The memory and CPU requests of section 3.2 are the starting point; the M load test sets the final numbers. The research 32 figures (24 to 32 cores and 128 to 256 GB per node on M) are the bank's hardware assumption to check.

**Two sites (DR-332).** Research 32 section 16.6 puts one of the three M nodes at a second site. This phase does not: a 2 plus 1 split loses the etcd majority (K3s control plane) and the Patroni DCS majority when the two-node site fails, and PostgreSQL then stops writing. The second site is the DR server and the DR replica of section 3.8, which are useful without a majority at the second site. The bank may still choose 2 plus 1 with the majority at the primary site (Open doubts 3).

**The load test on M (section 6.3).** Runs the phase 29 gateway profile against the M cluster at the target rate of phase 29 section 6.3, with the synthetic payments of phase 29 and the failover of case C1 during the run. Results go to the bank's acceptance record; no number is written into this document.

### 3.16 Errors (API section 6 only)

No new problem code. The HTTP-facing answers of this phase use existing codes only: `E_UNAVAILABLE` (503) for a database without a writable primary and for an evidence upload while the object store has no quorum; `E_IDEMPOTENCY_IN_FLIGHT` (409) during a switchover is phase 29's (section 3.5 above). The CLI commands of this phase exit with 0 on success and with 2 on a failed check (the convention of `akr-ctl lists verify`, phase 14, defined here for every new command), 3 when a precondition is missing (for example no evidence file), and print the first failing row.

### 3.17 Audit events emitted

Every mutation writes one audit event. Payloads hold ids, codes and counts only (phase 02 catalogue rule). New kinds are added to the phase 02 catalogue in code; the one reused kind is phase 11's `blind_period.opened`.

| Event kind | On | Resource | SIEM | Detail keys |
|-|-|-|-|-|
| `failover_drill.recorded` | `akr-ctl ops record-failover-drill` | setting `ops.last_failover_drill_at` | yes | `result`, `measured_failover_s`, `cases_passed`, `cases_total`, `evidence_id` |
| `restore_check.recorded` | `akr-ctl restore verify` (end) | evidence object | yes | `steps_passed`, `steps_total`, `first_failed_step`, `replayed_certificates`, `chain_rows` |
| `ha.failover_observed` | `ops_probe` on the new primary (the writable node changed) | `system` | yes | `previous_node`, `new_node`, `failovers_24h` (node names are infrastructure names, not personal data) |
| `ha.outage_recorded` | `akr-ctl ha outage record` | blind period | yes | `duration_s`, `evidence_id` (the blind period itself is `blind_period.opened`, phase 11) |
| `ha.standby_read_refused` | `scenario run` when the lag is above `ha.standby_max_lag_s` | scenario run | no | `replay_lag_s` |
| `release.refused` (phase 25) | with the new `chart_signature_invalid` | release | yes | `refusal_code` only (phase 25 rule) |

Five new kinds (`failover_drill.recorded`, `restore_check.recorded`, `ha.failover_observed`, `ha.outage_recorded`, `ha.standby_read_refused`); one existing kind with a new refusal value (`release.refused`, phase 25); one reused kind (`blind_period.opened`, phase 11). The evidence owner kind `ops_drill` is a registration in the phase 02 owner registry, not an event.

### 3.18 Tipping-off review (what a person outside the access list can learn; the answer must be "nothing")

- Failover and restore reports: counts, times, node names, check names. No case, party, account or payment reference. No route serves a stored report in this version: the owner kind `ops_drill` has no owner route (DR-380), and a download route is P2. The command prints its report and leaves the file with the operator for the acceptance record; SEC sees the audit event with the step results and counts (audit viewer and SIEM forward), and phase 25 sends no notice (section 7+).
- Blind periods: the existing phase 11 visibility (system scope, no party). Unchanged.
- The standby read setting and the scenario run `cost` keys: counts and lags. No party.
- Release chart counts (A4): counts and digests. Nothing about a case.
- Cluster events and logs: section 3.14.
- No new screen shows a customer, a party or a payment. No new notice or export.

### 3.19 Settings used and added

| Key | Status | Value and floor | Used in |
|-|-|-|-|
| `ops.last_failover_drill_at` | new (migration 0095) | timestamp, `null`, governed | section 3.9, 3.13 |
| `ops.failover_drill_max_age_days` | new (0095) | 92; floor max 92 | section 3.9, 3.13 |
| `ops.failover_max_s` | new (0095) | duration_s, 60; floor max 300 | section 3.4, 3.8, 3.9 |
| `ha.standby_reads` | new (0095) | false; governed | section 3.7 |
| `ha.standby_max_lag_s` | new (0095) | 60; floor max 300 | section 3.7 |
| `ops.restore_drill_max_age_days`, `ops.backup_max_age_h`, `ops.last_restore_drill_at` | phase 00 | unchanged | sections 3.9, 3.13 |
| `ops.probe_stale_s`, `ops.cert_warn_days` | phase 03 | unchanged | sections 3.8, 3.13 |
| `jobs.lease_s` | phase 00 | unchanged (600); the heartbeat is every one third of it, or of the kind's override | section 3.5 |
| `assess.*` | phase 29 | unchanged | sections 3.6, 3.8 |

Settings are not used for the Patroni, OpenBao or Kubernetes configuration, which are files of the chart (versioned, reviewed, rendered by section 3.1); those are deployment facts, not bank-changeable business rules (phase 00 admin-managed rule).

---

## 4. Infrastructure Updates

### 4.1 Job kinds (Postgres `jobs`; coalesce key in brackets)

No new job kind. Three existing jobs change. Every kind gains the cluster-wide concurrency and the heartbeat of section 3.5 (harness changes; no per-kind flag, no schedule change):

| Kind | Binary | Change | Schedule (unchanged) |
|-|-|-|-|
| `ops_probe` (phase 03) | worker | reads the cluster and database metrics of section 3.13, computes `akr_db_writable_primary`, `akr_db_sync_standbys`, `akr_db_failovers_total`; opens the outage blind period of section 3.8; emits `ha.failover_observed` | every minute (`schedule.ops_probe`) |
| `backup_check` (phase 00) | worker | sets `akr_failover_drill_age_days`; runs the daily host check of section 3.2 (clock offset, disks, time to OpenBao) and records a failure as a cockpit tile, not an audit event | daily (`schedule.backup_check`) |
| `release_verify` (phase 25) | worker | the chart check of section 3.12 and the chart count in `cosign_result` | on import (phase 25) |

Every handler of the kinds above re-reads its owner first (phase 00 and phase 29 rule); the heartbeat of section 3.5 applies to every kind.

### 4.2 Timers, outbox and notification channels

No timer kind and no outbox topic. The notification channels of phases 03, 05 and 29 are unchanged; their listeners reload their state on reconnect (section 3.5, and section 4.5 item 10).

### 4.3 AppState additions

- **akr-api**: no new service. `OpsCockpit` gains the tiles of section 3.13.
- **akr-worker**: `StandbyPool` (profiles M and L only, section 3.7) in `AppState` of the scenario job; `HaProbe` inside `ops_probe`.
- **akr-assess**: `LimiterFallback` next to the Valkey limiter of step 3 (phase 29); `PolicyCache` and `GatewayCache` gain the reconnect reload and the version check.
- **akr-ingestd**: the source listener re-reads the source state on reconnect (no new state).
- **akr-ctl**: `deploy render` (with `--profile s|m|l`, `--out`, and `deploy upgrade --release`), `net check --cluster`, `ops record-failover-drill`, `restore verify`, `ha outage record`, `host check`. `bundle verify` gains the chart check.

### 4.4 Network zones, keys, cluster, readiness, metrics, OpenAPI

- **Zones.** `deploy/zones.yaml` gains four keys that the renderer reads (`cluster.apiserver`, `cbs.listeners`, `gateway.cidrs`, and `backup.repository` of section 7.5), and the existing edges stay as they are. The zone matrix of phase 29 section 4.4 (`akr-assess` joins `z1_bao`) is accepted here (Open doubts 5). The namespaces and labels of section 3.3 are the cluster form of the same matrix. `make zone-check` (compose) and `akr-ctl net check --cluster` (cluster) read the same file.
- **Keys and roles.** One new AppRole `akr-migrate` (KV read `db/akr_migrate` only), issued per release (section 3.11). `akr_migrate` remains the migration role of phase 00; no application role owns a table. Every role's connection goes through `akr-pg-rw` (section 3.4). The standby reads use `akr_app` with a read-only transaction setting (section 3.7); nothing reads the DR replica before promotion.
- **Readiness.** `akr-assess` ready: the database answers, the decision key is loaded, every enabled list index is built, the policy snapshot is loaded (phase 29 rule). Valkey is no longer a condition (section 3.6; amends phase 00 and phase 29 section 4.4). `akr-api` ready: unchanged from phase 00.
- **Compose.** Profile S: no change. The phase 00 compose file is the first output of the renderer and must match it (section 3.1 step 2).
- **Metrics.** Section 3.13 (the table lists every metric and its source). The phase 29 `akr_assess_*` metrics are unchanged; `akr_assess_limiter_fallback_total` is added.
- **OpenAPI.** No new route, no change to the phase 29 `assess-v1.json` document, no change to the Orval output. `make generate-api` gives zero diff (section 6.1).

### 4.5 Existing entity, schema, route, job and code updates (extensions of earlier phases; each is additive or a stated fix)

1. **Phase 00 job harness (DR-337).** (a) `JobStore::heartbeat(id, locked_by)`: the runner refreshes `locked_at` every one third of the kind's lease (`jobs.lease_s` or the per-kind override) while a handler runs, guarded by `locked_by`; (b) the claim of section 3.5 enforces `JobKind::concurrency` across the cluster under `pg_advisory_xact_lock(hashtext(kind))`, counting the kind's `running` rows (no new field on `JobKind`); (c) the scheduler computes slots from the database clock (`now()`).
2. **Phase 00 listeners (DR-322).** Reconnect reload added to `akr_assess_policy`, `akr_assess_gateway` (phase 29) and `akr_source` (phase 05); `akr_policy` (phase 03) already polls `active_id()` and is unchanged. Each listener's reload is one full read of its state; the notification becomes a hint.
3. **Phase 00 readiness and phase 29 (DR-325).** Valkey is removed from the readiness list of `akr-assess` (phase 00 section 2, the readiness checks, and its section 6 readiness check; phase 29 section 4.4).
4. **Phase 00 zones and compose (DR-319, section 3.1).** `deploy/zones.yaml` keys of item 4.4 above; the compose bundle is generated (no change to its content).
5. **Phase 00 bundle (DR-334).** The bundle carries `chart/` and `k3s/` (section 3.12); the bundle verify of phase 00 section 4 checks the chart signatures.
6. **Phase 00 `backup_check` (DR-328, DR-335).** The failover drill age metric and the daily host check of section 3.2 (section 4.1).
7. **Phase 02 (DR-328, DR-329, section 3.17).** Owner kind `ops_drill` in the evidence owner registry; the five new audit event kinds of section 3.17 in the catalogue with their SIEM flags and detail keys; `audit.create_partition` unchanged.
8. **Phase 03 (DR-336, section 3.8).** `OpsCockpit::snapshot()` gains the eight tiles of section 3.13 and the replication lag tile covers the DR replica; `ops_probe` gains the metrics and the outage detection of section 3.8.
9. **Phase 01 (DR-325).** The Valkey rate limiter answers without Valkey by failing open, and counts each such answer in `akr_rate_limit_unavailable_total` (a requirement; phase 01 does not state the behaviour today, Open doubts 6).
10. **Phase 05 (DR-322).** `akr-ingestd` keeps one replica (no change in the code path); its listener reload is item 2. Phase 11 changes nothing: its `admin.blind_periods` is used as it is (section 3.8).
11. **Phase 16 (DR-327; its open item 26).** `scn.scenario_runs.cost` gains the keys `read_source` and `replay_lag_s` (JSONB, no DDL); the scenario job uses `StandbyPool` under the settings of section 3.7.
12. **Phase 23.** No change. `restore verify` runs the phase 23 commands (`verify-certificates`, `replay`) in the order phase 23 requires; the readiness gate of phase 23 is unchanged.
13. **Phase 25 (DR-334, migration 0095).** `admin.releases.refusal_code` gains `chart_signature_invalid`; `release_verify` checks the charts; A4 shows the chart count and the K3s version (section 5.2); the support bundle of section 3.8 of phase 25 collects no Kubernetes object, event or cluster log (section 3.14 of this phase).
14. **Phase 29 (DR-325, DR-318).** `akr-assess` runs at least two replicas on M with the placement and budget of section 3.2; `LimiterFallback` (section 3.6); `PolicyCache` checks its version every 30 seconds in addition to the notification; `GatewayCache` reloads on reconnect; the readiness list of section 4.4 loses Valkey.
15. **Phase 24.** No change: no review calendar row (section 3.9) and no G1 tile (section 5.3).
16. **Phase 12.** No change; `restore verify` queues phase 12 `recon_run` jobs for the gap (section 3.9, step 5).
17. **Migration numbering.** `0095_ops_ha_settings.sql` (section 1). The next free number after it is `0096`.

---

## 5. Frontend Impact

### 5.1 A3 Platform health (`/admin/health`; DASH; ADM; additions only)

The eight tiles of section 3.13 are added to the existing cockpit grid (`db_role`, `db_sync`, `db_failovers`, `replication_lag` (existing tile, now with the DR replica), `etcd`, `certificates` (existing tile, now with cert-manager expiry), `clock`, `failover_drill`). Each tile shows its state and the threshold that set it (phase 03 rule); no tile shows a name, a case or a customer; node names appear only in the `db_role` tile's detail, as infrastructure names. The G1 command centre is unchanged (section 5.3).

### 5.2 A4 Release and updates (`/admin/releases`; STEPPER; ADM and SEC read)

The release detail of phase 25 shows two additional facts from `cosign_result`: the number of Helm charts verified and the K3s release of the bundle (counts and versions only). The refusal label `chart_signature_invalid` is shown with its bilingual text (the phase 25 pattern for refusal codes). The step "Propose" and the four-eyes approval are unchanged.

### 5.3 G1 command centre (`/command`; phase 24 screen G1): no tile

No tile is added. G1 is read by M, CH and IA (`dashboard.read`) and is a compliance view; the platform's high-availability state is an operations fact for ADM and the bank's operators, shown on A3 (section 5.1), and the blind periods it may affect are already on G1 through phase 29 (`assess`) and phase 11.

### 5.4 Generated hooks (Orval names from the paths)

No new route, so no new hook. Used: `useGetAdminOps` (A3, `GET /admin/ops`), `useGetAdminReleases` and `usePostAdminReleasesVerify` (A4, phase 25 hook summary), `usePostAdminReleasesIdApplied` (A4, phase 25). The `make generate-api` drift check gives zero diff for the frontend.

### 5.5 Browser and caching rules

A3 refreshes its tiles on the phase 03 cadence; it reads `GET /admin/ops` only. A tile whose probe snapshot is older than `ops.probe_stale_s` shows `unknown` (phase 03 rule), never `ok`.

## 6. Verification

### 6.1 Checks (run on the test cluster of section 6.3 unless stated)

1. **Render drift.** `make deploy-render && git diff --exit-code deploy/` exits 0. On profile S, the first render of `deploy/compose/` equals the phase 00 compose bundle byte for byte (`diff -r` empty), and `docker compose config` of both is equal.
2. **Chart lint and schema.** `helm lint deploy/chart/akr` reports no error; `kubeconform -strict -kubernetes-version <pinned>` on `helm template` output reports no invalid object.
3. **Admission.** Three cases in namespace `akr`: (a) an image by digest with a valid vendor signature (the bundle's `akr-api`) is admitted; (b) a busybox image by digest with no signature is refused by the Kyverno policy with the policy name in the message; (c) the same signed image referenced by tag is refused (digest required).
4. **Cluster zone check.** `akr-ctl net check --cluster` exits 0 and its table equals the matrix of `deploy/zones.yaml` in every row; the known-bad probe (no membership labels) reaches nothing. Then the probe matrix is edited in a scratch copy with one edge removed and the check exits 2 on the row of that edge.
5. **Chart signature.** `akr-ctl bundle verify` on a bundle whose `chart/akr` file was changed by one byte refuses with `chart_signature_invalid`; the import row is `refused` with that code; the check constraint of section 1 accepts the value and rejects `chart_bad_value` (a direct `INSERT` through `akr_migrate` in a scratch database).
6. **Migration `0095`.** Applies cleanly twice; `SELECT key FROM admin.settings WHERE key IN ('ops.last_failover_drill_at','ops.failover_drill_max_age_days','ops.failover_max_s','ha.standby_reads','ha.standby_max_lag_s')` returns five rows; `\d admin.releases` shows `releases_refusal_code_check` once. The SQL of section 1 was checked against a throwaway `docker run --rm postgres:17` with a stub `admin.releases` and `admin.settings` (no production database).
7. **PostgreSQL proof of concept.** Criteria 1 to 10 of section 3.4 for each candidate; the report is attached to the bank's acceptance record. The chosen tool is named in DR-320 by the PoC report, not by this document.
8. **Failover drill.** Cases C1 to C6 of section 3.9 with their pass criteria; `akr-ctl ops record-failover-drill --result passed --measured-failover-s <n> --cases-passed 6 --cases-total 6 --evidence drill.json` sets `ops.last_failover_drill_at`; audit `failover_drill.recorded` exists; the age tile of section 3.13 reads `ok`.
9. **Gateway during a switchover.** During C1, every response of `akr-assess` is 200 with a `pass` or `hold` body whose `decision_id` has a row in `assess.decisions` with the same signature, or 503 `E_UNAVAILABLE`, or 409 `E_IDEMPOTENCY_IN_FLIGHT`; no 200 `pass` lacks its row. Count check: the number of 200 responses in the window equals the number of decision rows of that window (the same counter as the audit trail).
10. **Audit chain.** `akr-ctl audit verify --all` exits 0 after C1 and after C3; `SELECT count(*) FROM audit.audit_log` before and after the switchover has no gap in `occurred_at` order (the verifier's chain check); every anchor's `prev_anchor_id` chain is unbroken.
11. **No duplicate runs, no double dispatch.** During C1 and C6, a sampler runs `SELECT kind, count(*) FROM admin.jobs WHERE status = 'running' GROUP BY kind` every second and no kind exceeds its `JobKind::concurrency`; with two worker replicas, a scheduled kind runs once per slot (`coalesce_key` count is 1); each outbox row is dispatched once by its handler's idempotency record (a handler table in the test harness counts the outbox ids it applied).
12. **OpenBao without a human unseal.** C4 kills one raft member; the member restarts and reports unsealed without any `bao operator unseal` command (the install log and the audit of the bank's PAM show none); `akr-assess` answers 200 throughout with cached keys.
13. **Valkey.** C5 stops Valkey: `akr_assess_limiter_fallback_total` rises; `akr-assess` readiness stays ready; the counts stream answers 503 (phase 17) and recovers within 60 seconds of Valkey's return.
14. **Restore check and the monthly rehearsal.** `akr-ctl restore verify` on a scratch restore exits 0 and writes the report evidence (owner `ops_drill`) and the audit event `restore_check.recorded`; a second run after a deliberately removed deletion certificate fails at step 1 and names it; `akr-ctl ops record-restore-drill --result passed` after the monthly rehearsal (phase 00 pattern).
15. **Standby reads.** With `ha.standby_reads = true`: a run with injected replica lag of 90 seconds records `read_source = 'primary'` and writes `ha.standby_read_refused`; a run with lag of 5 seconds records `read_source = 'standby'` and `replay_lag_s`; with `ha.standby_reads = false` no `StandbyPool` query runs (the pool's statement log is empty). A write attempted through `StandbyPool` fails with SQLSTATE 25006.
16. **Observability.** `promtool check rules deploy/observability/slo/*.yaml` exits 0; each metric of section 3.13 is present at `/metrics` of its exporter; `akr_db_writable_primary` equals 1 in steady state; each tile of section 3.13 shows `ok` in steady state and the state its threshold predicts after the matching case.
17. **Tipping-off.** A seeded run with the fixture name `Ahmed Kareem Hassan` (an invented name, the conventions of phase 00) and a case reference: `kubectl get events -A -o json`, `kubectl describe` of every pod of `akr` and `akr-mgmt`, the Fluent Bit output and the support bundle contain none of the fixture name, the case reference or the account number of the fixture (a grep returns 0 for each).
18. **Settings floors.** `PUT /api/v1/admin/settings/{key}` (phase 03) on the governed key `ops.failover_drill_max_age_days` answers 409 (governed keys change only by change request, phase 04). The floor is enforced in `SettingsService::apply` for every source: a change request that sets 120 is refused at apply with 422 `E_BELOW_FLOOR` (the floor of 92 is a maximum), and one that sets 92 is applied; `ops.failover_max_s` = 400 is refused the same way.
19. **Profile S unchanged.** The phase 00 curl suite and `make zone-check` pass on profile S with the rendered bundle; no compose service changed.
20. **OpenAPI.** `make generate-api` gives zero diff; the gateway document `assess-v1.json` is unchanged.
21. **Load test on M.** The phase 29 gateway profile at the rate of phase 29 section 6.3 for the duration stated there, with case C1 in the middle of the run. Result: p95 at most 150 ms and p99 at most 400 ms outside the switchover window, and inside it the 503 share and the fail-policy answers only (no pass without a row). Results go to the acceptance record.

### 6.2 Writer and probes used by the drill and the PoC

A test-only schema `drill` is created by the test harness through `akr_migrate` on the test database (never in a product migration). It holds one table `drill.seq (id bigserial primary key, written_at timestamptz)`; the serial column's sequence is the one the writer's `nextval` reads. The writer inserts `id` every 10 milliseconds with `nextval`, and records `id` in a client-side log only after its commit returns. After a switchover the checker reads `drill.seq` on the new primary and verifies: every logged `id` exists (zero acknowledged rows lost, PoC criterion 2 and drill case C1), and the ids form one run with no gap before the kill point. A second probe reads `pg_is_in_recovery()` and `pg_stat_replication.sync_state` on each node every second, and the counts of section 3.13 are compared with the probe's own log.

### 6.3 The test script and the load test

`tests/ha/phase30_failover.sh` (new, in the repository after this phase is implemented; no file exists yet) takes one subcommand each: `render-check` (check 1), `zone-check` (check 4), `admission` (check 3), `chart-verify` (check 5), `poc-postgres` (section 3.4 criteria, check 7), `drill C1` to `drill C6` (checks 8 to 13), `restore-verify` (check 14), `standby` (check 15), `observability` (check 16), `tipping-off` (check 17), `load-m` (check 21). The script refuses to run against any kubeconfig whose context name is not on its allow-list of test clusters, and never runs drill cases on a production context: the production drill is run by the bank's operators from the bank's runbook, with the same script in its read-only mode for the checks.

The load test uses the phase 29 gateway profile and the targets of ARCHITECTURE section 19 (p95 150 ms, p99 400 ms). Its numbers are never written into this document.

---

## 7+. Gap Additions

Gap pass 1 (2026-10-10), run 4b, from tools/agent/gap1-run4b.md; lead-verified (tools/agent/gap1-run4b-verdict.md).

### 7.1 Pack flags in the catalogue
A catalogue entry may carry `pack: <flag name>`. `akr-ctl deploy render` takes `--pack <name>=on|off` for each flag the
catalogue names; a flag not given is off. An entry whose flag is off is left out of every output (compose, chart, env
keys and secret paths), so with all flags off the output is the phase 00 bundle byte for byte (section 3.1 step 2).
The `polis` entry and its flag `pack.polis` are phase 31's; this phase owns the mechanism. The render reads the flag
from the command line, not from the database.

Gap pass 1 (2026-10-10), run 4b: items carried from run 4a and corrections found while checking phases 25 to 31 (tools/agent/gap1-run4b-verdict.md); lead-verified.

### 7.2 Deletion certificate key
Restore step 1 verifies the deletion certificates against `akr-sign-deletion` (phase 23 section 7+, DR-357); the name
`akr-sign-cert` is not used.

### 7.3 Reconciliation after a restore (Open doubts 23 closed)
Phase 12 section 7+ settles it: restore verify runs in order, runs of `recon_run` for a restore gap are valid while the
replay runs, and a date not yet reloaded shows as a break.

### 7.4 Who sees failover and restore reports
No route serves the stored reports in this version (owner kind `ops_drill` has no owner route; a download route is P2), and the operator keeps the files. SEC sees their audit events (`failover_drill.recorded`, `restore_check.recorded`,
SIEM yes) in the audit viewer and the SIEM forward. Phase 25 sends no notice for them.

Verification pass (2026-10-10), from tools/agent/verify/; lead-verified (tools/agent/verify-verdict.md).

### 7.5 The restore namespace takes the cluster rules (V-30-05-a, HIGH; DR-413)
The monthly rehearsal of section 3.9 restores a copy of production data, so its namespace `akr-restore` is under the same controls as `akr` and `akr-mgmt` whenever it exists:
1. `akr-ctl deploy render` writes for `akr-restore` the default deny of ingress and egress and the DNS egress of section 3.3. The only allows are the ones the restore needs: the `restore verify` Job to the scratch PostgreSQL pod on 5432 inside the namespace, and egress from the two to the pgBackRest repository, the object store and OpenBao on the ports `zones.yaml` names for those destinations (the repository is its entry `backup.repository`). The allows select the pods that carry the label `akr.io/net-restore: "true"` (the label form of section 3.3; a membership of this namespace only, not a network of `zones.yaml`), which the Job and the scratch pod carry and no other pod does, so the unlabelled known-bad probe of item 2 reaches nothing. No rule lets a pod of `akr` or `akr-mgmt` reach the scratch database, and no rule lets `akr-restore` reach an application pod.
2. `akr-ctl net check --cluster` probes `akr-restore` with the same table and the known-bad probe of section 3.3, and its rows must pass before the restore starts.
3. The Kyverno `verifyImages` policy of section 3.12 covers `akr-restore` in `Enforce` mode.
4. The namespace and its scratch PersistentVolume are deleted when the drill is recorded, so no second copy of the data outlives the rehearsal.
The alternative of section 3.9 (the DR server) is outside the cluster and keeps the DR site rules of section 3.8.

### 7.6 The two writers of a system blind period (V-30-07-a, V-30-07-b, V-30-07-d, V-30-07-e, V-30-07-f, MEDIUM; DR-331; DR-413)
Neither writer of section 3.8 has a session, so neither can call `BlindPeriodService::declare` (phase 11: the steward route under Cedar `blind_period.declare`). This phase adds one method to that service and owns it: `BlindPeriodService::record_system(tx, started_at, ended_at, statement) -> Result<(BlindPeriod, bool), AppError>` (the row, and whether this call wrote it). Its two callers are `ops_probe` (item 1) and `akr-ctl ha outage record` (item 2). It has no Cedar action, as a job handler and an operator command have none. Both callers connect as `akr_app`, which holds `SELECT, INSERT` on `admin.blind_periods` (phase 11 section 1), so no grant is added. In the caller's transaction the method:
1. Takes `pg_advisory_xact_lock(hashtext('blind_period:system'))` and reads the rows of scope `system` and cause `akr_outage` whose span overlaps the new one. When one exists it writes nothing and returns that row with `false`: the probe logs the id of the row found and still writes `ha.failover_observed`, and the command exits 2 and prints that id. One outage is therefore one row.
2. Refuses a `started_at` earlier than the newest `audit.audit_anchors.anchored_at` and an `ended_at` that is missing, in the future or before `started_at`. The probe never meets the first refusal, because item 1 already takes the later of the two instants.
3. Inserts through `BlindPeriodRepository::open` with scope `system`, cause `akr_outage`, `declared_by` = the system principal (phase 01) and `comment` NULL: the reason is in `statement`, and for the command also in its evidence object. The command's audit event carries the OS user and the PAM session id of the phase 00 rule.
4. Audits `blind_period.opened`, the phase 11 kind for a period that no steward declared.
The probe's `started_at` is an estimate. That departs from the true start that a phase 11 declaration records, which is why `statement` names it as one.

### 7.7 The ransomware restore: offline copy, clean environment and rotation (V-30-08-a, V-30-08-b, V-30-08-c, MEDIUM; DR-413)
These rules complete the ransomware paragraph of section 3.9 and item 2 of the DR site list of section 3.8.
1. **Offline copy.** Once a month, after the rehearsal has passed, the bank's operators copy three things to media that are then kept off every network: the pgBackRest repository as it stands after the newest full backup (it is encrypted, phase 00), the newest restic snapshot of the buckets, and the newest OpenBao raft snapshot. The operators add the signed bundle of the running release to the same media. This is a bank procedure with a line in the rehearsal record, not an AKR-TMS job, and how long the media are kept is the bank's choice (Open doubts 22). No repository is object-locked in this design.
2. **Clean environment.** A host or cluster installed from the offline release bundle, with no route to the production zones until the rotation below is done. The namespace `akr-restore` on the production cluster is not clean after a ransomware event; the monthly rehearsal there proves the restore procedure only.
3. **Order.** Restore OpenBao from its snapshot and unseal it; restore the database from the offline repository; run `akr-ctl restore verify`; rotate; only then open a node to users.
4. **Rotation**, with the means the plan already has, in this order: (a) the AppRole secret ids of section 3.12, issued anew per ceremony (phase 00), with the old ids revoked in OpenBao; (b) the database role passwords, set again by `akr-ctl db provision` (phase 00); (c) the Hydra client secrets, with `akr-ctl identity client rotate` (phase 01); (d) when `pack.assess` is on, a new decision key (`akr-ctl assess keys generate --purpose decision`, phase 29) and new gateway certificates (`akr-ctl assess gateway register`, then `revoke` with reason `compromise`; phase 29). The transit keys are rotated in OpenBao (a new key version; the older versions stay for reading) by the operators of the OpenBao ceremony runbook (SP-12, phase 00), when the bank's security officer finds that OpenBao itself was exposed. One command that runs the whole rotation is not part of this version.
5. **Record.** Each `akr-ctl` step writes the audit event it already has (`service_client.rotated`, `assess.key_activated`, `assess.gateway_registered`, `assess.gateway_revoked`). The evidence file of the run lists the credential kinds rotated in OpenBao and in the database, and the run is recorded with `akr-ctl ops record-restore-drill` (phase 00). No audit kind is added.

## Decisions

1. **DR-317 (one catalogue and one generator for compose and the chart).** `deploy/catalogue.yaml` (new) lists each service once with its pinned digest, environment sources, secret paths, network names from `deploy/zones.yaml` and profile membership; `akr-ctl deploy render` writes the profile S compose bundle and the profile M and L Helm chart from it, and CI fails on any difference between the render and the committed outputs. Rationale: TS-29 asks for one source of truth for images, environment keys and secrets paths; a hand-written second file drifts silently, and the drift check of the OpenAPI pattern only catches it after the fact. The profile S output is the acceptance test, so the render cannot change the working bundle.
2. **DR-318 (the M cluster: three servers, no taints, Traefik and ServiceLB disabled, local volumes, two `akr-assess` replicas).** Three K3s servers host the control plane, embedded etcd and the workloads, with hard anti-affinity for the stateful and gateway pods and the budgets of section 3.2; profile L adds data agents by label and taint and changes nothing else. Traefik and ServiceLB are disabled because the phase 00 Caddy edge already terminates TLS with the bank's certificates. PostgreSQL, OpenBao and the object store use statically created local volumes, because replication belongs to the application and shared storage would add a failure domain. `akr-assess` runs as two replicas on M, one per server, with a PodDisruptionBudget of one, because the gateway needs a second node while one restarts, and the phase 29 index is held per process. Rationale: TS-29 and research 32 section 16.1, the cost of a third data copy on the bank's disks, and the phase 29 decision that no gateway call waits on a database that is not writable.
3. **DR-319 (zones are labels, not namespaces; the matrix stays in `zones.yaml`).** Workloads sit in two namespaces, `akr` and `akr-mgmt`; each pod carries one label per network it belongs to; the NetworkPolicies are rendered from `zones.yaml` with a default deny, DNS, one allow per edge and the stated `ipBlock` rules (CBS listeners only from `akr-ingestd`, gateway CIDRs only to `akr-assess` on 8443, no egress for the object store and the model zone). The cluster zone check probes every source membership and a known-bad probe, and fails on any difference. Rationale: SP-04 requires default-deny in both directions with a test that proves what each workload can reach; a pod can belong to several zones, which a namespace cannot express, and phase 00 forbids a second matrix. The known-bad probe makes a disabled enforcement visible.
4. **DR-320 (PostgreSQL HA: Patroni with pgBackRest by default; CloudNativePG by the proof of concept).** Three PostgreSQL pods, one primary, one synchronous standby and one asynchronous standby, with Patroni's synchronous mode in strict form, so a failover never goes to a copy that is behind and writes stop rather than degrade. Patroni is the provisional default because it keeps the pgBackRest backup policy and restore runbooks of phase 00 and the DBS, which research 32 section 16.5 names as the deciding factor; CloudNativePG wins only if it passes the same proof-of-concept criteria and the bank accepts a backup tool other than pgBackRest. Rationale: TS-32 asks for a choice by proof of concept, and the criteria of section 3.4 are fixed before the test so that the choice cannot be made by preference.
5. **DR-321 (the application reaches the primary through one Service; the pool discards demoted connections).** `akr-pg-rw` selects the pod that Patroni labels as primary; every write path uses it, and no write path uses a pod address. Pool connections have a maximum lifetime, are tested on acquire, and a SQLSTATE 25006 answer discards the connection; a write is retried only when the phase 00 idempotency key or the phase 29 ledger makes the retry safe. Rationale: the switchover must not require a configuration change in any binary, and a retried write without a key is a duplicate waiting to happen. The connection string names one host: `postgres://<role>@akr-pg-rw.akr.svc.cluster.local:5432/akr?sslmode=verify-full` (the TLS rule of SP-08), with the role's password read from OpenBao (phase 00).
6. **DR-322 (what a switchover does to each loop and record: the table of section 3.5, with the listener reload).** Every loop and every durable record is listed with its behaviour across a switchover and with two or more replicas, checked against the real locking of phase 00 and phase 02. The three notification listeners without a poll reload their full state on reconnect, because a notification sent in the reconnect gap is lost; the notification becomes a hint and the poll or version check is the guarantee. Rationale: the switchover is a normal event on M, and each loop's answer must be known before the drill, not discovered in it.
7. **DR-323 (synchronous commit with strict Patroni mode: zero acknowledged loss under one failure, stopped writes under two).** `synchronous_commit = on` with one synchronous standby. With one failure every acknowledged commit exists on the new primary; with two failures at once no node accepts a write until a synchronous standby returns. Rationale: the audit chain (phase 02) and the decision ledger (phase 29, no `pass` without a row) need every acknowledged row to survive; a stopped write is refused, not lost, and SP-87 requires that a state change that cannot be audited is refused. The availability cost is recorded in the RTO and RPO table (DR-330) and the gateway fail policy absorbs it.
8. **DR-324 (OpenBao: three raft members with automatic unseal; a restart never waits for a human).** The seal is the bank's HSM through PKCS#11 where the bank has one; otherwise a transit seal from a second OpenBao in the bank's secure zone. No static-key seal. A restarting `akr-assess` unwraps its seeds at start-up from a quorum, and a running node keeps its keys in memory, so an OpenBao restart does not interrupt the gateway path. Rationale: TS-24 and research 32 section 13.4 name the availability hazard of a sealed vault on a node restart; the phase 29 design keeps the decision path free of OpenBao calls, so the only exposure is the start-up of a node, and that is designed to need a quorum, not a person.
9. **DR-325 (Valkey: one instance with restart; the gateway limiter and readiness do not depend on it).** Valkey runs as one instance on M and L. Its losses cost a sign-out, a counter reset and a cache refill, never a wrong answer: the idle clock fails closed (a missing key signs the user out), the counts stream answers 503, and caches reload from PostgreSQL. `akr-assess` answers its rate limit in process while Valkey is down and stays ready, because a gateway outage caused by a cache is the largest avoidable outage in phase 29. Rationale: the phase 00 rule says Valkey has no persistence requirement; replicas would add Sentinel or cluster complexity for state that does not need to survive. The phase 01 limiter must fail open with a metric (requirement, Open doubts 6).
10. **DR-326 (object store, identity services and edge: their replica counts and failure answers).** The object store is three erasure-coded members on M and L if the store proof of concept supports erasure coding and passes the phase 00 conformance suite with one member down, otherwise one member per site with the restic copy; Kratos and Hydra run two replicas each; Caddy runs two. While the object store has no quorum, evidence uploads answer 503 and nothing is written around the object lock. Rationale: the COMPLIANCE lock must survive a node loss, which only the erasure-coded set guarantees; the fallback keeps the lock and refuses writes instead of weakening it.
11. **DR-327 (standby reads close phase 16 open item 26: scenario runs only, on M and L, governed).** `StandbyPool` reads through a read-only transaction setting; the scenario job reads the standby only when `ha.standby_reads` is true and the replica is no more than `ha.standby_max_lag_s` behind; the run records which copy it read and the lag. Screening, decisions, holds, cases and all writes never read the standby. Rationale: DSL section A.6.1 expects runs on the standby where one exists, and phase 16 deferred it to this phase; the reads that feed a decision must see the newest state, and a scenario run that records its source is reproducible.
12. **DR-328 (failover drill: quarterly, six chaos cases, recorded by a setting, an audit event and evidence; no calendar row).** The six cases C1 to C6 each have an expected effect and a pass criterion (section 3.9). A passed drill sets `ops.last_failover_drill_at` through the CLI-only exemption that phase 00 uses for the restore drill; the age tile reads it against `ops.failover_drill_max_age_days` (92, floor maximum 92). Rationale: SP-85 and the ROADMAP exit criterion "a failover drill passed"; the record mirrors the phase 00 pattern, which the bank's reviewers already know, and a calendar row would be a second CHECK change in phase 24.
13. **DR-329 (post-restore verification is one command with a fixed order, and it runs before the API opens).** `akr-ctl restore verify` runs the deletion-certificate verification and replay of phase 23, the audit chain check against the locked anchors, the key and clock check, and the reconciliation runs for the restore gap, and writes one report. The phase 23 readiness gate stays closed until the replay completes. Rationale: SP-86 asks for one post-restore job; the parts exist in phases 02, 12 and 23, so the phase builds the order and the report and nothing else, and the ransomware variant of SP-88 reuses it.
14. **DR-330 (RTO and RPO proposals per profile).** The table of section 3.8 gives, for each function, the recovery time and point for profiles S, M and L, with the mechanism that delivers it. The numbers are proposals for the bank's BIA (SP-83), built from research 30 section 16.4 and the phase 00 backup policy; the M figure for the phase 2 path is the failover budget of DR-328 plus the gateway's fail policy window. Rationale: the CBI expects an RTO and an RPO per system; the bank adjusts them, and the platform states the mechanism that makes each number true.
15. **DR-331 (an AKR-TMS outage is a blind period, recorded on recovery and never back-dated; the DR site serves nothing until promoted).** A database outage is an `admin.blind_periods` row with scope `system` and cause `akr_outage` (phase 11), opened on recovery by the probe with its estimated start and the statement that says so, or by an operator with evidence; the gateway's own silence is recorded by phase 29. The DR site holds an asynchronous replica, the second repository and the OpenBao copies, and serves nothing before promotion; promotion is a bank runbook, and two sites never accept writes at once. Rationale: ARCHITECTURE section 18 and SP-87 require the outage on record with its real creation time. Research 30 section 16.6 asks for a read-only workspace on the standby; it is not carried, because sign-in and audited reads both write (Open doubts 31).
16. **DR-332 (hardware profiles are planning shapes; no stretched cluster; the load test decides the size).** S, M and L are the shapes of research 32 section 16.6 with the requests and limits of section 3.2 as starting values. The second site is the DR server and replica, not a third K3s member, because a 2 plus 1 split loses the etcd and Patroni majorities when the two-node site fails. Rationale: TS-33 and the CBI's expectation of a second site; a majority that can be lost with a single site is a worse failure than a failover that needs the DR procedure.
17. **DR-333 (upgrades: migrations as a hook under `akr_migrate`, a fixed rollout order, rollback by revision with additive migrations).** The pre-upgrade hook runs the migrations as `akr_migrate` against the primary after a named restore point; the order is worker, ingestd, API, assess (one at a time), edge; the rollback is `helm rollback` and `migrate --ignore-missing`, with the restore point for any data migration that cannot run backwards. Each migration is its own transaction, so a failure leaves no partial DDL. Rationale: TS-30 and phase 25's rollback plan; the worker goes first because a job kind must be known before the API enqueues it, and an old worker never claims an unknown kind.
18. **DR-334 (signed charts and K3s artifacts in the bundle; admission refuses unsigned images; the chart gate is the upgrade script).** The bundle carries the charts and the K3s artifacts under the same manifest and signature rules as the images; `chart_signature_invalid` joins phase 25's refusal list (migration `0095`); Kyverno admits only signed images by digest in the two AKR namespaces; charts are verified by `cosign verify` in the upgrade script because admission does not check charts. Rationale: TS-30 requires content trust before any pull and an admission policy, and research 32 section 16.2 requires an air-gapped path that never pulls from a public registry.
19. **DR-335 (time: chrony on every node, a pre-flight refusal, an alert before the failure).** Each server syncs to the bank's sources; `akr-ctl host check` refuses an install with an offset above 100 ms and runs daily; the clock tiles warn above 100 ms and go critical above 500 ms. The database clock orders the audit chain (phase 02), and the node clock sets decision expiry (phase 29), so both are covered. Rationale: TS-40 and research 30 section 16.5 (no signing and no watermark while the clock is unhealthy); the in-process check of the gateway is left open (Open doubts 14), because an unreadable offset must not be treated as a healthy one.
20. **DR-336 (cluster access, logs and metrics carry no personal data; the bank's operators alone reach `kubectl exec`).** Operators reach the cluster through the bank's PAM in zone Z5; RBAC binds only the operators' group; no vendor identity has a role (zone Z7). Events, logs, labels and metrics carry names of resources and counts only, and the support bundle collects no cluster object. The cockpit tiles and SLOs are code in the repository, and their alerts sit beside them. Rationale: the tipping-off rule of phase 29 DR-309, SP-04 for Z5 and Z7; TS-27 and TS-28 for observability as code.
21. **DR-337 (the job harness gets three fixes that the switchover and the second worker expose).** A running job refreshes its lease; every kind's concurrency holds across the cluster; scheduler slots are computed from the database clock. Rationale: phase 00 sets `locked_at` once, so a long job would be reaped and started twice (`retention_run` runs up to 120 minutes against a 600-second lease), its semaphore is per process, so two worker replicas run a concurrency-1 kind twice, and a replica clock could enqueue a slot early. Each is a defect of the phase 00 harness that the phase owner accepts (Open doubts 18); the fixes are additive and change no working job's behaviour on one worker.

**Lead corrections (review 2026-10-09).** The read-only DR workspace could not work: sign-in writes Kratos sessions and audited reads write `audit.access_event`, so it is dropped and goes to Open doubts 31. The cluster-exclusive claim covered a hand-picked list, so every other concurrency-1 kind still ran once per worker replica; the claim now enforces every kind's `JobKind::concurrency` across the cluster under `pg_advisory_xact_lock(hashtext(kind))` by counting `running` rows, with no flag and no list, and the heartbeat follows the kind's own lease. The outage writer inserted `admin.blind_periods` directly; it now goes through phase 11 `BlindPeriodService`, and its start never reaches back past the newest audit anchor. `akr-ctl jobs run recon_sweep` does not exist; the restore check queues phase 12 `recon_run` jobs per source and date. `akr_policy` is a phase 03 channel that already polls, so only three listeners change. `ops.failover_max_s` is `duration_s`. `LimiterFallback` divides the gateway rate by the replica count, so two nodes do not allow twice the rate.

---

## Open doubts (for gap pass 1 and the bank)

1. **Patroni or CloudNativePG.** The provisional default (DR-320) is not a result. The proof of concept decides; the bank's operators decide whether the footprint of criterion 6 is acceptable. If CloudNativePG wins, section 3.4 lists the work.
2. **DCS and storage.** Patroni's distributed configuration store is the Kubernetes API for the PoC; a separate three-node etcd is the alternative (its own failure domain). CSI for a bank SAN is not designed here; the bank decides whether it wants one on M.
3. **Two sites.** The bank may prefer research 32 section 16.6's 2 plus 1 layout with the majority at the primary site. DR-332 gives the reason against it; the bank accepts or rejects it, and the DR server stays either way.
4. **OpenBao seal.** Whether the bank has an HSM with the PKCS#11 seal path, or will run a second OpenBao in its secure zone, which has its own unseal ceremony. The chain of seals is the bank's to approve.
5. **Zone for `akr-assess`.** Phase 29 Open doubts 1 (`z1_bao`) is accepted by this phase for the cluster form of the matrix; the bank approves the zone change at installation, as phase 29 says.
6. **Valkey and the limiter.** Phase 01 does not state what its rate limiter does without Valkey; DR-325 asks for fail-open with a metric. The bank confirms that a Valkey restart may sign analysts out, which is the phase 01 behaviour today.
7. **Erasure coding in the object store build.** Whether the AKR Silo build supports erasure-coded members with COMPLIANCE object lock, which the store proof of concept must show (DR-326). If not, the fallback of one member per site applies and evidence writes are refused while the member is down.
8. **K3s network enforcement and CNI.** The design assumes the embedded network policy controller of K3s with flannel and WireGuard enforces the NetworkPolicies; the cluster zone check proves it. A bank that requires another CNI (its choice) needs the same check and its own installer step.
9. **Synchronous replication on profile S.** When `pack.assess` is on with profile S, the bank must enable synchronous replication to `postgres-standby`, and accept that commits stop when the standby is down. Without it the decision RPO on S is the replication lag at the failure.
10. **Failover budget and the gateway.** `ops.failover_max_s` (60 seconds) and the gateway's circuit breaker threshold must agree: a breaker that opens faster than the failover stops nothing, and one that opens slower lets passes queue. The bank also confirms its gateway queue does not exceed the pass window (phase 29 Open doubts 23).
11. **Drill record.** The drill is recorded by a setting, an audit event and evidence (DR-328). Whether the bank wants a review calendar row in phase 24 instead is a phase 24 change that its owner decides; the age tile works either way.
12. **Admission controller.** Kyverno is the candidate of research 32 section 16.3; sigstore policy-controller is the alternative. The PoC picks one; the bank approves the add-on, its images and its licence.
13. **Cluster access.** The bank names the identity-provider groups for `akr-operators` and its PAM workflow for `kubectl exec`, and decides who may run the chaos cases on its production cluster (DR-336).
14. **Clock check inside the gateway.** The gateway does not read its node's offset in process; the alert and the pre-flight are the controls. The bank decides whether a readable offset should refuse signing (research 30 section 16.5 says so) and how the offset is read from the container.
15. **Objectives.** The availability and latency objectives (section 3.13 and DR-336) are proposals. The bank confirms them, and the RTO and RPO table (DR-330) for its BIA.
16. **Charts in the bank's Harbor.** The bank confirms that its Harbor accepts the OCI chart with a cosign signature, and its version (research 32 section 16.2 requires content trust on the artifacts pulled by the cluster).
17. **Standby reads.** The bank decides whether scenario runs may read a replica up to `ha.standby_max_lag_s` behind (default 60 seconds). A governed setting is a change request (phase 04); the default is off.
18. **Phase 00 harness.** The phase 00 owner accepts the three harness fixes of DR-337 (lease refresh, cluster-wide concurrency, database-clock slots) and the listener reload of DR-322, as additive changes; gap pass 1 applies them to phase 00 section 3.
19. **Patroni permissions.** The Kubernetes DCS needs a ServiceAccount that can read and write its own ConfigMaps and Endpoints; the bank's security review confirms that scope, because it is the only cluster permission the database needs.
20. **Hardware.** The memory and CPU requests of section 3.2 and the research 32 figures (24 to 32 cores and 128 to 256 GB per M node) are planning assumptions. The bank confirms its hardware or accepts the load test's result; L is a planning shape only.
21. **DR promotion.** The bank names the owner of the promotion runbook and the authority that starts it; this phase does not promote automatically.
22. **Immutable copies.** The pgBackRest repository and the restic copies are not object-locked in this design (research 30 section 16.5 records that the DBS backups are not immutable). The bank decides whether the DR repository uses object lock, and whether the offline monthly copy is its own (SP-88).
23. **Reconciliation after a restore.** Phase 12 owns `recon_run`; its owner confirms that runs for a restore gap are valid while the replay is running, or that `restore verify` must wait for it to finish.
24. **Refusal list.** Phase 25's owner confirms that `chart_signature_invalid` is the only new refusal code, and that the constraint name `releases_refusal_code_check` is the one PostgreSQL generates for phase 25's inline CHECK (checked by the lead's SQL test of 2026-10-09: PostgreSQL gives the inline CHECK that name).
25. **`akr-migrate` AppRole.** The bank approves a new AppRole whose only secret is a per-release secret id, deleted after the hook (section 3.11). Phase 00 lists the AppRoles of the four services and `akr-ctl` only.
26. **Superuser name in the profile S runbook.** Phase 00 does not name the bootstrap superuser; section 3.10 uses a placeholder until the name is written in the phase 00 bootstrap.
27. **K3s release.** Research 32 section 16.1 gives v1.37.1+k3s1 as the current line at 2026-09-30. The bank's approved version may differ; the bundle pins one and the PoC tests it.
28. **Third-party licences.** The licences of the third-party charts and images (Kyverno, the HA tool chosen, OpenBao, Valkey, cert-manager) are checked in the bundle's notices (phase 00 step 7) before the PoC, and the bank's legal review is a checklist item.
29. **Phase 16 cost keys.** Phase 16's owner confirms that `scn.scenario_runs.cost` takes the two keys of section 4.5 item 11 and that nothing in phase 16 reads the whole object as a fixed shape.
30. **Requirement line of MODULES M20.** `docs/MODULES.md` section M20 lists SP-83 and SP-85 and SP-87 but not SP-84, SP-86, SP-88 or TS-29 to TS-33, which this phase carries; the lead adds them to the M20 requirements line and to the phase 30 mapping of `docs/plan/roadmap.md`.
31. **DR workspace (not carried).** Research 30 section 16.6 asks for a read-only workspace on the standby. A read-only `akr-api` on the DR replica fails at sign-in (Kratos writes sessions to its database, phase 01) and at every audited read (`audit.access_event`, phase 02). It needs a writable identity store and an access-event spool at the DR site that is merged into the chain on return, or a promoted copy (which is the promotion runbook). The bank decides whether case work during a primary-site loss justifies that design; until then the interim control is paper (section 3.8).

## What This Phase Does NOT Touch

- No change to the profile S compose bundle, its services, networks, environment keys or secrets paths; the renderer must reproduce it byte for byte (section 3.1 step 2).
- No route, no Cedar file, no role, no step-up class, and no new problem code (API section 6 only).
- No new job kind, no new outbox topic, no new timer kind, no new notification channel. The listeners and the job kinds named above change behaviour only as section 4.5 states.
- No new table and no change to any existing table other than the CHECK of `admin.releases.refusal_code` (migration `0095`).
- No CBS connection, no change to `akr-ingestd`'s source access, no read of the CBS by any binary; the read-only conformance suite of phase 05 is unchanged.
- No change to the decision path of phase 29 (its steps, budgets, signing, ledger, holds and fail policies) beyond the readiness list, the limiter fallback, the version check and the reconnect reload of section 4.5 items 2 and 14; no model, no score.
- No change to the screening index, the scoring, the lists or the freeze (phases 14 and 20).
- No change to the retention engine or the deletion certificates of phase 23 beyond running its existing commands in order (section 3.9).
- No change to the audit chain format, the anchors, the stream lock keys or the verifier (phase 02); the new event kinds are catalogue entries.
- No change to the release bundle format beyond the chart and K3s entries of section 3.12 and the refusal value of item 13 in section 4.5; no change to the release states or the change request route of phase 25.
- No G1 tile, no review calendar row, no change to phase 24.
- No vendor remote access, no vendor standing account in any cluster, no call-home, no push of any change from the vendor (TS-30, Z7).
- No production drill is run by this document; the test cluster results and the bank's drill results are recorded outside it.
- No file other than `docs/plan/phase-30.md` is changed by this phase. The lead applies the amendments of section 4.5 to phases 00, 01, 02, 03, 04, 05, 16, 25 and 29 and to `docs/ARCHITECTURE.md`, `docs/API.md`, `docs/DATA-MODEL.md`, `docs/DESIGN-SYSTEM.md`, `docs/plan/roadmap.md`, `docs/plan/research.md`, `docs/plan/status.md` and `tech-stack.md`.
