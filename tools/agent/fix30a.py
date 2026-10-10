p='docs/plan/phase-30.md'
s=open(p).read()
def rep(a,b,n=1):
    global s
    assert s.count(a)==n,(s.count(a),a[:90])
    s=s.replace(a,b)

# 1: no read-only DR workspace
rep("akr-api (the `OpsCockpit` tiles of section 3.13, and the read-only standby mode of section 3.8; no route)",
    "akr-api (the `OpsCockpit` tiles of section 3.13; no route)")
rep("SP-87 (AKR-TMS-down behaviour per function, the recorded database outage, the read-only DR workspace; section 3.8)",
    "SP-87 (AKR-TMS-down behaviour per function and the recorded database outage; section 3.8)")
rep("a sizing of profile L beyond its planning shape (section 3.15; the load test decides it)",
    "a sizing of profile L beyond its planning shape (section 3.15; the load test decides it), the read-only DR workspace of research 30 section 16.6 (Open doubts 31)")
rep("| The read-only DR workspace of this section, if the DR site is up; goAML is the Office's system |",
    "| Paper records of the bank, keyed in after recovery; goAML is the Office's system (no DR workspace before promotion, Open doubts 31) |")
rep("4. A read-only workspace: `akr-api` started with the bootstrap variable `AKR_STANDBY_READONLY=true` on a single server at the DR site. Every non-GET route answers 503 `E_UNAVAILABLE`; the readiness check reports `standby_readonly`; reads go to the replica. Cases, notes, screening lists and reports are readable; nothing is written. `akr-assess` is not deployed at the DR site until promotion.",
    "4. No workspace before promotion. A read-only `akr-api` on the DR replica cannot work: sign-in writes Kratos sessions to the `kratos` database (phase 01) and every read of personal data writes an `audit.access_event` row (phase 02), so nobody could sign in and every audited read would fail. `akr-api` and `akr-assess` are not deployed at the DR site until promotion (Open doubts 31).")
rep("for a database without a writable primary, for an evidence upload while the object store has no quorum, for a non-GET request on the read-only DR workspace; ``E_IDEMPOTENCY_IN_FLIGHT`",
    "for a database without a writable primary and for an evidence upload while the object store has no quorum; `E_IDEMPOTENCY_IN_FLIGHT`")
rep("- **akr-api**: no new service. `OpsCockpit` gains the tiles of section 3.13. The read-only standby mode of section 3.8 (`AKR_STANDBY_READONLY=true`, a bootstrap variable, phase 00 rule) is a startup branch: the router keeps only GET routes and answers 503 `E_UNAVAILABLE` on the rest.",
    "- **akr-api**: no new service. `OpsCockpit` gains the tiles of section 3.13.")
rep("The DR replica and the standby reads use `akr_app` with a read-only transaction setting (section 3.7).",
    "The standby reads use `akr_app` with a read-only transaction setting (section 3.7); nothing reads the DR replica before promotion.")
rep("`akr-api` ready: unchanged from phase 00; in the standby mode, readiness reports `standby_readonly` with status ready.",
    "`akr-api` ready: unchanged from phase 00.")
rep("never back-dated; the DR site is read-only until promoted).**", "never back-dated; the DR site serves nothing until promoted).**")
rep("The DR site holds an asynchronous replica, the second repository, the OpenBao copies and a read-only API mode; promotion is a bank runbook, and two sites never accept writes at once. Rationale: ARCHITECTURE section 18 and SP-87 require the outage on record with its real creation time, and research 30 section 16.6 asks for a read-only workspace on the standby.",
    "The DR site holds an asynchronous replica, the second repository and the OpenBao copies, and serves nothing before promotion; promotion is a bank runbook, and two sites never accept writes at once. Rationale: ARCHITECTURE section 18 and SP-87 require the outage on record with its real creation time. Research 30 section 16.6 asks for a read-only workspace on the standby; it is not carried, because sign-in and audited reads both write (Open doubts 31).")

# 2: cluster-wide concurrency for every kind
rep("the job harness heartbeat and cluster-exclusive claim of section 3.5", "the job harness heartbeat and cluster-wide concurrency of section 3.5")
rep("`heartbeat(id, locked_by)` and the cluster-exclusive claim.", "`heartbeat(id, locked_by)` and the cluster-wide concurrency of the claim.")
rep("| Kinds that must be single across the cluster are marked `cluster_exclusive` in code (`assess_hold_sweep`, `assess_release_match`, `assess_blind_watch`, `assess_rollup_watch`, `assess_fallback_extract`, `assess_ledger_sweep`, `retention_run`, `anchor_audit`, `partition_maintenance`, `recon_sweep`, `backup_check`, `maintenance_sweep`). The claim takes `pg_advisory_xact_lock` on a fixed key of the kind and skips the kind when a `running` row exists; the lock lasts for the claim transaction, and the `running` row is what the next claimant sees. |",
    "| The claim enforces every kind's concurrency across the cluster, with no per-kind flag and no list: in the claim transaction it takes `pg_advisory_xact_lock(hashtext(kind))`, counts the kind's `running` rows, and claims at most `JobKind::concurrency` minus that count. The lock lasts for the claim transaction; the committed `running` rows are what the next claimant counts. The per-process semaphore stays as the local bound. |")
rep("| The handler refreshes `locked_at` every `jobs.lease_s / 3` (`JobStore::heartbeat(id, locked_by)`, guarded by `locked_by`).",
    "| The handler refreshes `locked_at` every one third of the kind's lease (`jobs.lease_s`, or the per-kind override of the phase 00 registry that the reaper uses; `JobStore::heartbeat(id, locked_by)`, guarded by `locked_by`).")
rep("(the cluster-exclusive claim of section 3.5)", "(the cluster-wide concurrency of section 3.5)")
rep("No new job kind. Three existing jobs change, and six phase 29 jobs and the phase 00 and phase 02 jobs listed below gain the cluster-exclusive claim of section 3.5 (a property of the kind in code, not a schedule change):",
    "No new job kind. Three existing jobs change. Every kind gains the cluster-wide concurrency and the heartbeat of section 3.5 (harness changes; no per-kind flag, no schedule change):")
i=s.index("| `assess_release_match`, `assess_hold_sweep`, `assess_blind_watch`")
j=s.index("\n", s.index("| `retention_run`, `anchor_audit`, `partition_maintenance`, `recon_sweep`"))
s=s[:i].rstrip("\n")+s[j:]
rep("the runner refreshes `locked_at` every `jobs.lease_s / 3` while a handler runs, guarded by `locked_by`; (b) `JobKind` gains `cluster_exclusive: bool` and the claim of section 3.5 takes `pg_advisory_xact_lock` on a fixed key per kind and skips the kind when a `running` row exists;",
    "the runner refreshes `locked_at` every one third of the kind's lease (`jobs.lease_s` or the per-kind override) while a handler runs, guarded by `locked_by`; (b) the claim of section 3.5 enforces `JobKind::concurrency` across the cluster under `pg_advisory_xact_lock(hashtext(kind))`, counting the kind's `running` rows (no new field on `JobKind`);")
rep("no `cluster_exclusive` kind exceeds 1", "no kind exceeds its `JobKind::concurrency`")
rep("a kind marked `cluster_exclusive` runs once across the cluster", "every kind's concurrency holds across the cluster")
rep("(lease refresh, cluster-exclusive claim, database-clock slots)", "(lease refresh, cluster-wide concurrency, database-clock slots)")
rep("| unchanged (600); the heartbeat is every one third of it |", "| unchanged (600); the heartbeat is every one third of it, or of the kind's override |")

# 3: outage through BlindPeriodService
rep("command `ha outage record` (uses phase 11 `BlindPeriodRepository::open` through",
    "command `ha outage record` (uses phase 11 `BlindPeriodService` through")
rep("it opens a blind period: `scope = 'system'`, `cause = 'akr_outage'`, `started_at` = the previous snapshot's `taken_at` (the last moment the platform is known to have run),",
    "it opens a blind period through phase 11 `BlindPeriodService` (phase 11 section 3.6 owns the rules of a system period): `scope = 'system'`, `cause = 'akr_outage'`, `started_at` = the later of the previous snapshot's `taken_at` (the last moment the platform is known to have run) and the newest `audit_anchors.anchored_at` (the phase 11 rule: a period never reaches back past sealed evidence),")
rep("`akr-ctl ha outage record --from T1 --to T2 --evidence FILE` inserts the same row with",
    "`akr-ctl ha outage record --from T1 --to T2 --evidence FILE` opens the same row through `BlindPeriodService` (a `T1` earlier than the newest anchor is refused, the phase 11 rule) with")

# 4: no akr-ctl jobs run
rep("5. Reconciliation: `akr-ctl jobs run recon_sweep` (phase 12) is run for the business dates of the restore gap, and its result is compared with",
    "5. Reconciliation: the command lists the business dates of the restore gap, queues one phase 12 `recon_run` job per source and date (the job behind `POST /recon/runs`), waits for their results, and compares them with")
rep("16. **Phase 12.** No change; `recon_sweep` is run by `restore verify` (section 3.9, step 5).",
    "16. **Phase 12.** No change; `restore verify` queues phase 12 `recon_run` jobs for the gap (section 3.9, step 5).")
rep("Phase 12 owns `recon_sweep`; its owner confirms that a sweep for a restore gap is valid",
    "Phase 12 owns `recon_run`; its owner confirms that runs for a restore gap are valid")
rep("and the reconciliation sweep for the restore gap", "and the reconciliation runs for the restore gap")

# 5: akr_policy is phase 03
rep("`akr_policy` (phase 04): a reconnect re-reads the active bundle version. | Reload on reconnect added to the four listeners (section 4.5). |",
    "`akr_policy` (phase 03): phase 03 already polls `active_id()` beside its listener, so a reconnect needs no change. | Reload on reconnect added to the three listeners `akr_assess_policy`, `akr_assess_gateway` and `akr_source` (section 4.5). |")
rep("`akr_source` (phase 05) and `akr_policy` (phase 04). Each listener's reload",
    "and `akr_source` (phase 05); `akr_policy` (phase 03) already polls `active_id()` and is unchanged. Each listener's reload")
rep("Reconnect reload added to `akr_assess_policy`, `akr_assess_gateway` (phase 29), and",
    "Reconnect reload added to `akr_assess_policy`, `akr_assess_gateway` (phase 29) and")
rep("The notification channels of phases 04, 05 and 29", "The notification channels of phases 03, 05 and 29")
rep("The four notification listeners reload their full state", "The three notification listeners without a poll reload their full state")

# 6: duration_s
rep(" ('ops.failover_max_s', '60', 'integer',", " ('ops.failover_max_s', '60', 'duration_s',")
rep("| `ops.failover_max_s` | new (0095) | 60; floor max 300 |", "| `ops.failover_max_s` | new (0095) | duration_s, 60; floor max 300 |")

# 7: limiter fallback split across replicas
rep("`LimiterFallback`: an in-process token bucket with the same refill and capacity, per node, used only when Valkey fails;",
    "`LimiterFallback`: an in-process token bucket per node whose capacity and refill are the gateway's setting divided by the `akr-assess` replica count of the catalogue (rounded up), so the nodes together stay within the gateway rate; used only when Valkey fails;")

# typos and layout
rep("the test is ``tests/ha/phase30_failover.sh poc-postgres`,", "the test is `tests/ha/phase30_failover.sh poc-postgres`,")
rep("\n\n---\n30. **Requirement line of MODULES M20.**", "\n30. **Requirement line of MODULES M20.**")
rep("to the phase 30 mapping of `docs/plan/roadmap.md`.\n",
    "to the phase 30 mapping of `docs/plan/roadmap.md`.\n"
    "31. **DR workspace (not carried).** Research 30 section 16.6 asks for a read-only workspace on the standby. A read-only `akr-api` on the DR replica fails at sign-in (Kratos writes sessions to its database, phase 01) and at every audited read (`audit.access_event`, phase 02). It needs a writable identity store and an access-event spool at the DR site that is merged into the chain on return, or a promoted copy (which is the promotion runbook). The bank decides whether case work during a primary-site loss justifies that design; until then the interim control is paper (section 3.8).\n")

# lead corrections
rep("\n\n---\n\n## Open doubts (for gap pass 1 and the bank)",
"""

**Lead corrections (review 2026-10-09).** The read-only DR workspace could not work: sign-in writes Kratos sessions and audited reads write `audit.access_event`, so it is dropped and goes to Open doubts 31. The cluster-exclusive claim covered a hand-picked list, so every other concurrency-1 kind still ran once per worker replica; the claim now enforces every kind's `JobKind::concurrency` across the cluster under `pg_advisory_xact_lock(hashtext(kind))` by counting `running` rows, with no flag and no list, and the heartbeat follows the kind's own lease. The outage writer inserted `admin.blind_periods` directly; it now goes through phase 11 `BlindPeriodService`, and its start never reaches back past the newest audit anchor. `akr-ctl jobs run recon_sweep` does not exist; the restore check queues phase 12 `recon_run` jobs per source and date. `akr_policy` is a phase 03 channel that already polls, so only three listeners change. `ops.failover_max_s` is `duration_s`. `LimiterFallback` divides the gateway rate by the replica count, so two nodes do not allow twice the rate.

---

## Open doubts (for gap pass 1 and the bank)""")

open(p,'w').write(s)
print('ok')
