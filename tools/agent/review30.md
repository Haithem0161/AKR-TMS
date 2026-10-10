# Phase 30 review findings (lead, 2026-10-09)
Draft: docs/plan/phase-30.md (641 lines; migration 0095_ops_ha_settings; no tables; no routes; no new job kinds; no
Cedar file; DR-317..337; 30 open doubts). Checked true against earlier phases: phase 25 refusal_code list (l.175),
phase 11 admin.blind_periods (scope system, cause akr_outage, declared_by, statement), phase 00 jobs.lease_s 600,
idempotency.in_flight_lease_s 120, per-kind semaphore (l.536), ops.last_restore_drill_at row shape (l.261), phase 23
`retention verify-certificates` and `retention replay --source worm` (l.689, 700), retention.max_run_minutes 120 (l.557),
scn.scenario_runs.cost JSONB (phase 16 l.191), phase 16 open item 26 (l.2097), job kinds partition_maintenance,
anchor_audit, maintenance_sweep, recon_sweep exist, phase 29 readiness includes Valkey (l.1337), Kyverno in research 32
(l.923), Kratos sessions in its own database (phase 01 l.33).

1. MEDIUM the read-only DR workspace (s.3.8 item 4, SP-87) cannot work as written: sign-in writes Kratos sessions to the
   `kratos` database (phase 01 l.33), and reads of personal data write `audit.access_event` rows (phase 02), so on a
   read-only replica nobody can sign in and every audited read fails or goes unlogged. Fix: drop the workspace from this
   phase; SP-87 is carried for the down-behaviour table and the outage record only; the workspace goes to "Not carried"
   and an Open doubt (needs a writable identity store and an access-event spool at the DR site, or a promoted copy);
   remove `AKR_STANDBY_READONLY` from s.3.8, s.4.3, s.4.4, s.3.16, header binaries line, DR-330/331 text.
2. MEDIUM the cluster-exclusive claim (s.3.5, s.4.1, s.4.5 item 1) covers only a hand-picked list of kinds; every other
   kind with `JobKind::concurrency` 1 (or N) still runs once per worker replica (the semaphore is per process, phase 00
   l.536). Fix: the claim enforces each kind's concurrency cluster-wide: under `pg_advisory_xact_lock(hashtext(kind))`
   it counts the kind's `running` rows and claims only up to `concurrency` minus that count; no per-kind flag, no list.
   Note the phase 00 per-kind lease override (l.543) beside the heartbeat.
3. LOW the automatic outage writer (s.3.8 item 1) inserts `admin.blind_periods` directly; phase 11 `BlindPeriodService`
   (s.3.6, l.965-990) owns the rules (system scope, never reaching back past the newest anchor, SP-87). Fix: open the
   period through `BlindPeriodService` with cause `akr_outage` and `started_at` = max(previous snapshot `taken_at`,
   newest `audit_anchors.anchored_at`); same for `akr-ctl ha outage record`.
4. LOW `akr-ctl jobs run recon_sweep` (s.3.9 restore check step 5) does not exist. Fix: step 5 lists the business dates
   of the gap and queues phase 12 `POST /recon/runs` per source and date (scope data.steward) or a `recon_run` job per
   source and date; the restore check waits for their results.
5. LOW `akr_policy` is a phase 03 channel (phase 03 l.427, 666), not phase 04 (s.3.5 table, s.4.5 item 2); phase 03
   already polls `active_id()`, so its "reload on reconnect" is existing behaviour. Fix the attribution.
6. LOW `ops.failover_max_s` is value_type `integer`; seconds use `duration_s` (as `assess.heartbeat_max_s`). Fix in
   0095 and s.3.19.
7. LOW `LimiterFallback` (s.3.6) gives each akr-assess node the full capacity, so two nodes allow twice the gateway
   rate during a Valkey outage. Fix: per-node capacity and refill = the setting divided by the configured replica count
   (from the catalogue), rounded up.
Severity: 0 critical / 0 high / 2 medium (1-2) / 5 low (3-7) = 7.

Gap-pass items (not draft defects): phase 00 job harness heartbeat, cluster-wide concurrency and DB-clock scheduler
(DR-337); phase 00 and phase 29 readiness lose Valkey; phase 01 limiter fail-open with metric; phase 02 owner kind
`ops_drill` and five audit kinds; phase 03 eight A3 tiles; phase 25 bundle `chart/` and `k3s/`, refusal code, support
bundle excludes cluster objects; phase 16 `cost` keys `read_source`, `replay_lag_s`; ARCHITECTURE D19 and research 32
s.16.6 two-site split rejected (DR-332, Open doubts 3); tech-stack rows (Kyverno, Patroni default pending PoC).
