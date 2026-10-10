# Gap pass 1, run 1 (targets 00, 01, 02; Haiku draft for lead review)


Scope: later phases 03 to 31 were read for every obligation on phases 00, 01 and 02. Sources: section 4 of each phase
(the "Existing entity ... updates" items and the zone, compose and AppRole text), every grep hit of
"phase 00/01/02" that describes a change, tools/agent/gap-items.md (a) to (aa), the "gap pass 1" rows of
docs/plan/research.md and the "Tables" notes of docs/plan/status.md. Line numbers are those of the cited files on
2026-10-10.

Severity rule used in this run. HIGH: the target lacks a runtime object that a later phase needs and that no later
phase's text creates (a role, an AppRole path, a zone edge, a timer or job or owner kind, a topic handler, a service
method, a CHECK value used at runtime). MEDIUM: a later migration already makes the change and only the target's text is
stale, or the target states a rule that a later phase changes. LOW: wording, or a list that is informative only.

Each Proposed 7.x block is written to be appended to the target's "## 7+. Gap Additions" section (currently "_None yet._")
with the number shown. Nothing here edits a phase file.

## Phase 00 findings

### G1-00-01 Outbox: one handler per topic, dotted notify name, siem.export release
- Severity: HIGH
- Category: Missing Logic
- Target: phase-00.md section 3, OutboxDispatcher paragraph (line 551) and the admin.outbox CHECK (line 124)
- Sources:
  - phase-22.md:824 "The TopicHandler registry becomes a list per topic" (handler list, run in one transaction)
  - phase-14.md:1221 "consumes canon.touched (phase 11) through a TopicHandler" (canon.touched has several consumers: 14, 15, 16, 19)
  - phase-22.md:795 "its admin.outbox CHECK requires a dotted topic" (so the name is notify.request)
  - phase-25.md:1238 "The reserved topic siem.export (phase 00) is released, not used"
- Check:
  - grep -n "TopicHandler" docs/plan/phase-00.md -> line 552 only, singular ("the registered TopicHandler"); one handler per topic is implied.
  - grep -n "notify" docs/plan/phase-00.md -> line 555 "notify (22)", no dot; the CHECK at line 124 requires a dot.
  - grep -n "siem.export" docs/plan/phase-00.md -> line 555, still reserved.

Proposed 7.1 text:
```markdown
### 7.1 Outbox handler lists and topic names
- `TopicHandler` registrations are a list per topic. `register(topic, handler)` appends. `OutboxDispatcher` runs the
  list for a row in order, inside one transaction, and sets `dispatched_at` only when every handler succeeded. If any
  handler fails, the row stays undispatched and the back-off of this section applies. Every handler is idempotent (the
  owner re-check rule of section 3).
- The reserved topic `notify` (phase 22) is named `notify.request`, because the `admin.outbox` CHECK requires a dotted
  name. Replace "`notify` (22)" in the reservation sentence with "`notify.request` (22)".
- Topic `siem.export` (phase 25) is released: no phase uses it. Remove it from the reservation sentence.
- Each topic is registered by the phase that produces or consumes it, through `register`. The registry is per topic,
  not per phase.
```

### G1-00-02 Job harness: lease heartbeat, cluster-wide concurrency, database clock, listener reload
- Severity: HIGH
- Category: Missing Logic
- Target: phase-00.md section 3, job claim and loops (around lines 530 to 550) and the LISTEN or NOTIFY text (line 535)
- Sources:
  - phase-30.md:479 "JobStore::heartbeat(id, locked_by): the runner refreshes locked_at every one third of the kind's lease"
  - phase-30.md:479 "pg_advisory_xact_lock(hashtext(kind))" (cluster-wide concurrency; DR-337)
  - phase-30.md:479 "(c) the scheduler computes slots from the database clock (now())"
  - phase-30.md:480 "Reconnect reload added to akr_assess_policy, akr_assess_gateway (phase 29) and akr_source (phase 05)"
  - research.md:348 DR-337 "Phase 00 sets locked_at once and its semaphore is per process"
- Check:
  - grep -n -i "heartbeat\|advisory\|hashtext" docs/plan/phase-00.md -> none in the job harness (line 257 and 683 mention a heartbeat metric or marker only).
  - grep -n "one semaphore per kind" docs/plan/phase-00.md -> line 536, per process.
  - grep -n "akr_jobs" docs/plan/phase-00.md -> line 535 only; no listener reconnect rule.

Proposed 7.2 text:
```markdown
### 7.2 Job harness: lease heartbeat, cluster-wide concurrency, database clock, listener reload
- Lease heartbeat: `JobStore::heartbeat(id, locked_by)` refreshes `locked_at` every third of the kind's lease
  (`jobs.lease_s` or the per-kind override) while a handler runs; the update is guarded by `locked_by`. The lease
  reaper re-queues only rows whose `locked_at` is older than the lease.
- Cluster-wide concurrency: the claim enforces `JobKind::concurrency` across all nodes. Inside the claim transaction it
  takes `pg_advisory_xact_lock(hashtext(kind))` and counts the kind's `running` rows. No new field on `JobKind`. The
  per-process semaphore stays as a local bound only.
- Scheduler: schedule slots are computed from the database clock (`now()`), never from the process clock.
- Listeners: a LISTEN connection reloads its state in full after every reconnect; the notification is a hint only.
  Channels in the system: `akr_jobs` (this section), `akr_policy` (phase 03; polls `active_id()`), `akr_source`
  (phase 05), `akr_assess_policy` and `akr_assess_gateway` (phase 29).
```

### G1-00-03 Timer kind registry: kinds registered by later phases
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-00.md section 3, TimerService paragraph (lines 558 to 562)
- Sources:
  - phase-19.md:1663 "registered in the phase 00 timer kind registry by this phase" (six kinds)
  - phase-20.md:939 "the kind registry gains five kinds"; phase-20.md:975 "Phase 00: the five timer kinds"
  - phase-21.md:1227 "the kind registry gains two kinds"; phase-21.md:1264 "the timer kind registry gains report_filing_target and report_rework_due"
  - phase-22.md:787 "the kind registry gains one kind"
  - phase-18.md:1102 "The kind is registered in the phase 00 timer kind registry by this phase"
- Check: grep -n -E "cr_activate|screening_alert_clock|kyc_review_clock|alert_touch|work_item_due|rfi_clock|freeze_due|report_filing_target|liaison_due" docs/plan/phase-00.md -> none. Phase 00 line 722 says the worker has "the timer kind registry", but lists no kinds.

Proposed 7.3 text:
```markdown
### 7.3 Timer kinds registered by later phases
The timer kind registry is a list of kind and owner kind pairs. The sweep enqueues the job `timer_<kind>`; a kind that
is not in the registry is never claimed (the job rule of this section). Kinds registered by later phases:

| Timer kind | Owner kind | Registered by |
|-|-|-|
| `cr_activate`, `cr_emergency_review`, `cr_post_review` | `change_request` | phase 04 section 3.6 (table, line 1103) |
| `task_timeout` | `task` | phase 04 (same table) |
| `screening_alert_clock` | `screening_alert` | phase 14 section 4.2 (line 1220) |
| `kyc_review_clock` | `kyc_review` | phase 15 section 4.2 (line 1780) |
| `alert_touch`, `alert_age` | `alert` | phase 17 section 4.2 (line 1533) |
| `work_item_due` | `work_item` | phase 18 section 4.2 (line 1102) |
| `rfi_clock` | `rfi` | phase 19 section 4.2 (line 1663) |
| `breach_stage` | `alert`, `work_item` | phase 19 section 4.2 |
| `watch_expiry` | `watch` | phase 19 section 4.2 |
| `hibernation_expiry` | `hibernation` | phase 19 section 4.2 |
| `qa_grant_expiry` | `qa_sample` | phase 19 section 4.2 |
| `intake_clock` | `internal_report`, `authority_request` | phase 19 section 4.2 |
| `freeze_due`, `freeze_limit`, `committee_report_due`, `second_report_due` | `freeze` | phase 20 section 4.2 (lines 943 to 946) |
| `refrain_due` | `refrain` | phase 20 section 4.2 (line 947) |
| `report_filing_target` | `report` | phase 21 section 4.2 (line 1231) |
| `report_rework_due` | `report_task` | phase 21 section 4.2 (line 1232) |
| `liaison_due` | `liaison_monthly_report` | phase 22 section 4.2 (line 791) |
```

### G1-00-04 Roles: akr_monitor and akr_retention
- Severity: HIGH
- Category: Missing Policy
- Target: phase-00.md section 1, migration 0001 (lines 39 to 49) and the provisioning text (lines 28 to 34)
- Sources:
  - phase-03.md:1113 "akr-ctl db provision creates the LOGIN role akr_monitor (password in OpenBao db/akr_monitor), member of pg_monitor only"
  - phase-23.md:931 "akr_retention (NOLOGIN, 0072) and the LOGIN role akr_retention_login"
  - phase-23.md:919 "the LOGIN role akr_retention_login with its password in OpenBao KV db/akr_retention_login"
- Check: grep -n "akr_monitor\|akr_retention" docs/plan/phase-00.md -> none. Lines 45 to 49 create akr_app, akr_ingest, akr_assess, akr_audit_writer and akr_readonly only.

Proposed 7.4 text:
```markdown
### 7.4 Monitoring and retention roles
Add to migration 0001, after the `akr_readonly` line (49):
    DO $$ BEGIN CREATE ROLE akr_monitor NOLOGIN; EXCEPTION WHEN duplicate_object THEN NULL; END $$;
    DO $$ BEGIN CREATE ROLE akr_retention NOLOGIN; EXCEPTION WHEN duplicate_object THEN NULL; END $$;
Provisioning (section 1, after the migrations): LOGIN with the OpenBao password is granted to `akr_monitor` (member of
`pg_monitor` only, no privilege on any AKR schema, password `db/akr_monitor`) and to the LOGIN role `akr_retention_login`
(member of `akr_retention` and of `akr_audit_writer`, password `db/akr_retention_login`). Phase 23 section 1.2 keeps its
table grants and does not create these roles again.
```

### G1-00-05 AppRole policies and KV paths for later phases
- Severity: HIGH
- Category: Missing Policy
- Target: phase-00.md section 3, AppRole policies (lines 594 to 598) and the KV layout (lines 587 to 589)
- Sources:
  - phase-03.md:1115 "KV db/akr_monitor read by the postgres-exporter AppRole akr-monitor"; phase-03.md:1116 "akr-worker policy gains read on sys/seal-status and sys/health"
  - phase-30.md:471 "One new AppRole akr-migrate (KV read db/akr_migrate only)"
  - phase-31.md:897 "the akr-polis AppRole is new"; phase-31.md:891 "kv/polis/directory ... read by the akr-api AppRole, which gains this one path"
  - phase-24.md:1166 "akr-api gains KV read db/akr_readonly"
  - phase-02.md:1036 "akr-worker policy gains ... KV read db/akr_readonly (verifier pool); akr-api gains KV read s3/stager"
  - phase-29.md:1333 "akr-assess: transit encrypt and decrypt on akr-kek; KV read db/akr_assess, keys/assess-decision/*, keys/assess-extract/*"
  - phase-23.md:919 "LOGIN akr_retention_login ... password in OpenBao KV db/akr_retention_login"
  - phase-28.md:755 "KV secret/assistant/llama"; phase-28.md:759 "secret/assistant/golden-hmac"
  - phase-25.md:1243 "bundle_trust ... read from OpenBao path kv/release/trust"
- Check: grep -n "db/akr_" docs/plan/phase-00.md -> db/akr_app, db/akr_ingest and db/akr_assess only (lines 594 to 597). grep -n "akr-polis\|akr-migrate\|akr-monitor\|s3/stager" docs/plan/phase-00.md -> none. grep -n "akr-assess" docs/plan/phase-00.md -> line 597 "KV read db/akr_assess (grows in phase 29)".

Proposed 7.5 text:
```markdown
### 7.5 AppRole policy additions and KV paths
AppRole policies (section 3, line 594), in addition to the current lines:
- `akr-api`: KV read `db/akr_readonly` (read-only pool, phase 24), `s3/stager` (stager credential, phase 02), `kv/polis/directory`
  (webhook secret, phase 31).
- `akr-worker`: KV read `db/akr_readonly` (verifier pool, phase 02) and `db/akr_retention_login` (phase 23); read on
  `sys/seal-status` and `sys/health` (phase 03); KV read `secret/assistant/llama` and `secret/assistant/golden-hmac`
  (phase 28); KV read `kv/release/trust` (phase 25).
- `akr-ingestd`: unchanged (`db/akr_ingest`, `sources/*`).
- `akr-assess`: transit encrypt and decrypt on `akr-kek`; KV read `db/akr_assess`, `keys/assess-decision/*`,
  `keys/assess-extract/*` (phase 29 section 4.4).
- New AppRoles: `akr-monitor` (KV read `db/akr_monitor`, for the postgres exporter, phase 03); `akr-migrate` (KV read
  `db/akr_migrate` only, issued per release, phase 30); `akr-polis` (phase 31).
- KV items added: `s3/stager` (phase 02), `keys/model-artifact/v<N>.pub` (phase 27, verify-only), `db/akr_monitor`,
  `db/akr_retention_login`, `db/akr_migrate`, `db/akr_readonly`.
- Not proposed here (UNCLEAR, see the summary): the KV mount prefix. Line 588 says KV v2 mount `akr/`. Phases 25 and 31
  write `kv/...`, phase 28 writes `secret/...` and phases 01 and 31 write `ory/...`. The lead must choose one rule before
  these paths are added.
```

### G1-00-06 Signing key akr-sign-decision is reserved and unused
- Severity: MEDIUM
- Category: Missing Policy
- Target: phase-00.md section 3, OpenBao layout (lines 587 to 588) and the akr-worker sign list (line 595)
- Sources:
  - phase-29.md:1351 "the transit key akr-sign-decision stays reserved and unused (section 3.4; Open doubts 2)"
  - research.md:305 DR-294 "transit key akr-sign-decision is reserved and unused (gap pass 1 amends phase 00)"
  - phase-29.md:1498 "Gap pass 1 amends phase 00 line 588"
- Check: grep -n "akr-sign-decision" docs/plan/phase-00.md -> line 588, "(ed25519; used from phases 02, 06, 12 and 29)". Line 595 "sign on the four signing keys".

Proposed 7.6 text:
```markdown
### 7.6 Signing keys: akr-sign-decision reserved
Replace the key list of section 3 (OpenBao layout) with: transit keys `akr-kek` (aes256-gcm96, wraps DEKs),
`akr-sign-anchor`, `akr-sign-package` and `akr-sign-certificate` (ed25519). `akr-sign-decision` is reserved and unused:
decision signing uses a seed wrapped under `akr-kek`, unwrapped once per node at start-up (DR-294, phase 29 section 3.4).
In the akr-worker AppRole line, "sign on the four signing keys" becomes "sign on the three signing keys".
Not proposed here (UNCLEAR): phase 25 (section 4.5 item 9) adds a bundle key `akr-sign-bundle` and phase 23 (section 4.4)
adds `akr-sign-cert`; phases 00 and 12 name the certificate key `akr-sign-certificate`. The lead must settle the name
before either key is added.
```

### G1-00-07 Zone matrix: networks, zone rows and zones.yaml keys
- Severity: HIGH
- Category: Missing Integration
- Target: phase-00.md section 4, compose and zone table (lines 687 to 707), network rule (lines 708 to 709) and zone rules (lines 713 to 716)
- Sources:
  - phase-29.md:1332 "This phase adds z1_bao (OpenBao, internal, no egress) for the start-up unwrap" (akr-assess)
  - phase-07.md:811 "New network z2_egress (not internal), attached to akr-worker only, created only when deploy/zones.yaml declares egress.proxy"
  - phase-25.md:1248 "z2_bank (new network, not internal, attached to akr-worker only)" and "kratos -> z3_idp (new network, not internal)"
  - phase-28.md:754 "Zone (new network z2_model, internal: true, no egress). Members: akr-worker, llama and llama-eval only"
  - phase-22.md:806 "akr-worker gains one outbound edge: the bank's SMTP relay host and port"
  - phase-03.md:1118 "Zone rules: akr-worker -> prometheus:9090; prometheus -> the three exporters; blackbox-exporter -> caddy:443, openbao:8200, s3guard:9000; postgres-exporter -> postgres:5432"
  - phase-30.md:470 "deploy/zones.yaml gains three keys ... cluster.apiserver, cbs.listeners, gateway.cidrs"
  - phase-07.md:843 "deploy/zones.yaml key egress.proxy"
  - phase-31.md:888 "Polis is attached to z3_ory ... and to z3_edge"
- Check:
  - grep -n -E "z2_egress|z2_bank|z3_idp|z2_model|cluster.apiserver|cbs.listeners|gateway.cidrs|egress.proxy|smtp" docs/plan/phase-00.md -> none.
  - grep -n "akr-assess" docs/plan/phase-00.md -> line 705, networks z4_gateway, z4_db, z4_cache, z5_mgmt (no z1_bao).
  - grep -n "z1_bao" docs/plan/phase-00.md -> lines 693 (openbao), 704 (akr-ingestd) and 709 (network rule) only.

Proposed 7.7 text:
```markdown
### 7.7 Zone matrix rows, networks and zones.yaml keys
- Row `akr-assess` (line 705): networks `z4_gateway`, `z4_db`, `z4_cache`, `z5_mgmt` and `z1_bao` (phase 29 section 4.4).
- New networks: `z2_egress` (not internal; attached to `akr-worker` only; created only when `egress.proxy` is declared;
  phase 07 section 4). `z2_bank` (not internal; `akr-worker` only; egress only to the SIEM collector and TSA addresses
  declared with `akr-ctl siem configure`; phase 25 section 4.4). `z3_idp` (not internal; `kratos` only; egress to the
  OIDC issuer; phase 25 section 4.4). `z2_model` (`internal: true`; members `akr-worker`, `llama`, `llama-eval`; no
  egress; phase 28 section 4.4).
- New rows: `node-exporter` (`z5_mgmt`), `postgres-exporter` (`z2_app`, `z5_mgmt`), `blackbox-exporter` (`z5_mgmt`,
  `z3_edge`, `z1_bao`, `z2_store`; phase 03 section 4.4); `polis` (`z3_ory`, `z3_edge`; phase 31 section 4.4).
- Zone rules in `deploy/zones.yaml` (add to the rule list at line 713): `akr-worker` -> `prometheus:9090`; `prometheus`
  -> the three exporters; `blackbox-exporter` -> `caddy:443`, `openbao:8200`, `s3guard:9000`; `postgres-exporter` ->
  `postgres:5432` (phase 03); `akr-worker` -> `gotenberg:3000` and `clamd:3310` (phase 02 section 4); `akr-worker` ->
  `llama:8080` (phase 28); `akr-worker` -> the declared egress proxy (phase 07); `akr-worker` -> the bank's SMTP relay
  (`notify.smtp_host`, `notify.smtp_port`; phase 22 section 4.4); `akr-assess` -> `openbao:8200`.
- Keys read by the renderer, added to `deploy/zones.yaml`: `cluster.apiserver`, `cbs.listeners`, `gateway.cidrs`
  (phase 30 section 4.4) and `egress.proxy` (phase 07 section 4).
```

### G1-00-08 Compose services, volumes, dev profiles and the image lock file
- Severity: MEDIUM
- Category: Missing Integration
- Target: phase-00.md section 4, compose table (lines 685 to 707) and the image pinning text
- Sources:
  - phase-27.md:768 "a read-only volume model-store mounted into akr-worker at /var/akr/models"
  - phase-28.md:756 "Artifact store (assistant-store). A read-only volume mounted into llama and akr-worker, root /var/akr/assistant"
  - phase-28.md:760 "add the service llama ... add the service llama-eval (profile assistant-eval) ... add the test-only service llama-stub (profile test)"
  - phase-11.md:1319 "One host path AKR_INGEST_INBOX_ROOT (default /var/akr/inbox) mounted into akr-ingestd only"
  - phase-11.md:1322 "Compose profile cdc adds akr-cdc, a Debezium Server container ... not started unless ingest.cdc_enabled is on"
  - phase-05.md:1338 "Dev profile fixtures adds cbs-fixture-pg"; phase-05.md:1343 "Dev profile fixtures-oracle (opt-in)"; phase-05.md:1345 "dev profile fixtures-mssql"
  - phase-05.md:1346 "Images are pinned by digest in deploy/images.lock"; phase-05.md:1378 "deploy/images.lock gains the fixture images"
  - phase-03.md:1107 to 1109 "node-exporter (new)", "postgres-exporter (new)", "blackbox-exporter (new)"
- Check: grep -n -E "images.lock|model-store|assistant-store|AKR_INGEST_INBOX|akr-cdc|node-exporter|llama|fixtures" docs/plan/phase-00.md -> none. Phase 00 pins images only in the table text ("pinned digest").

Proposed 7.8 text:
```markdown
### 7.8 Compose services, volumes, profiles and the image lock
- Services added to the table of section 4: `node-exporter`, `postgres-exporter`, `blackbox-exporter` (phase 03,
  section 4.4, pinned, networks as in 7.7); `llama` (`akr/llama-server`, pinned and signed, `z2_model`, volume
  `assistant-store` read-only, bearer secret from a read-only file); `llama-eval` (same image, profile `assistant-eval`);
  `llama-stub` (test only, profile `test`, never in the production bundle) (phase 28 section 4.4).
- Volumes: `model-store`, read-only, mounted into `akr-worker` at `/var/akr/models` with `AKR_MODEL_STORE` set
  (phase 27 section 4.4); `assistant-store`, read-only, mounted into `llama` and `akr-worker` at `/var/akr/assistant`
  (phase 28 section 4.4); the inbox host path `AKR_INGEST_INBOX_ROOT` (default `/var/akr/inbox`), mounted into
  `akr-ingestd` only (phase 11 section 4.2).
- Profiles: `cdc` (opt-in, `akr-cdc`, not started unless `ingest.cdc_enabled`; phase 11 section 4.2); dev profiles
  `fixtures`, `fixtures-oracle` and `fixtures-mssql` (phase 05 section 4; images pinned by digest).
- `deploy/images.lock`: the file that pins every image by digest, one line per image (name, digest, source). Phases 05
  and 28 read it; section 4 must define it.
```

### G1-00-09 Readiness: akr-assess without Valkey, akr-ingestd checks
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-00.md section 2, readiness checks per binary (lines 455 to 457)
- Sources:
  - phase-30.md:481 "Valkey is removed from the readiness list of akr-assess (phase 00 section 2, the readiness checks)"
  - phase-29.md:1337 "ready when the database answers, Valkey answers, the decision key is loaded, every enabled list's index is built, and the policy snapshot is loaded"
  - phase-05.md:1367 "akr-ingestd's /readyz adds source_listener (the LISTEN akr_source connection is up) and adapters"
  - phase-05.md:1378 "Phase 00: akr-ingestd readiness checks above"
- Check: grep -n "akr-assess\` database, Valkey" docs/plan/phase-00.md -> line 457 (Valkey still listed). grep -n "source_listener" docs/plan/phase-00.md -> none.

Proposed 7.9 text:
```markdown
### 7.9 Readiness: akr-assess and akr-ingestd
- Replace "`akr-assess` database, Valkey" with: "`akr-assess` database, the decision key loaded, every enabled list
  index built, and the policy snapshot loaded. Valkey is not a readiness condition (DR-325)."
- `akr-ingestd` adds: `source_listener` (the `LISTEN akr_source` connection is up) and `adapters` (every compiled
  adapter's client library loads; a missing Oracle client makes that adapter `unavailable`, not the binary unready).
```

### G1-00-10 Caddy edge rules and the web pipeline
- Severity: MEDIUM
- Category: Missing Integration
- Target: phase-00.md section 4, caddy row (line 701) and section 6, verification (CI steps)
- Sources:
  - phase-08.md:524 "Caddy adds the security headers for /: Content-Security-Policy: default-src 'self'; script-src 'self'; ..."
  - phase-08.md:529 "/branding.json is served with no-cache"; the same line: "Cache-Control: no-store on index.html"
  - phase-08.md:532 "CI (web pipeline of tech-stack.md, extended): pnpm install --frozen-lockfile; pnpm licenses list check; tsc -b"
  - phase-08.md:541 "Phase 00: Caddy headers and /branding.json; the web pipeline steps"
  - phase-31.md:890 "/api/v1/directory/events answers 404 at the edge in every profile (the phase 00 rule /api/* -> akr-api gains this one exclusion)"
- Check: grep -n -E "Content-Security|branding|pnpm|lint-design|directory/events" docs/plan/phase-00.md -> none. Line 701 says only "/api/* to akr-api; / static frontend; /files/* to the guard".

Proposed 7.10 text:
```markdown
### 7.10 Caddy edge headers, exclusions and the web pipeline
- Caddy adds, for `/`: `Content-Security-Policy: default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:;
  font-src 'self'; connect-src 'self'; frame-ancestors 'none'; base-uri 'none'; form-action 'self'; object-src 'none'`;
  `Strict-Transport-Security`; `X-Content-Type-Options: nosniff`; `Referrer-Policy: no-referrer`; `Permissions-Policy`
  denying camera, microphone, geolocation, payment and USB. `Cache-Control: no-store` on `index.html`; hashed assets
  immutable for one year. `/branding.json` is served with `no-cache`. (Phase 08 section 4.)
- Caddy answers 404 for `/api/v1/directory/events` in every profile, before the `/api/*` rule (phase 31 section 4.4).
- Web pipeline steps (section 6): `pnpm install --frozen-lockfile`; `pnpm licenses list` check; `tsc -b`; `eslint`;
  `scripts/lint-design.mjs`; `scripts/gen-tokens.mjs` and `scripts/check-contrast.mjs`; the Orval, contract and
  policy-matrix drift check; `vitest run`; `vite build` with the bundle budget; Playwright with axe (phase 08 section 4).
```

### G1-00-11 maintenance_sweep and backup_check extensions
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-00.md section 4, job kinds `maintenance_sweep` and `backup_check` (lines 677 to 681)
- Sources:
  - phase-01.md:885 "maintenance_sweep (phase 00) also deletes admin.stepup_assertions rows older than stepup.retention_days"
  - phase-02.md:1018 "maintenance_sweep (phase 00) also deletes akr-quarantine objects under staging/ older than evidence.staging_ttl_h"
  - phase-07.md:804 "maintenance_sweep (phase 00) extended: transient goaml objects older than evidence.staging_ttl_h ... deleted"
  - phase-30.md:484 "The failover drill age metric and the daily host check of section 3.2" (metric akr_failover_drill_age_days at phase-30.md:343)
- Check: grep -n -E "stepup_assertions|staging|transient|failover" docs/plan/phase-00.md -> none. Lines 677 to 681 list only the four deletions and the backup marker.

Proposed 7.11 text:
```markdown
### 7.11 maintenance_sweep and backup_check additions
`maintenance_sweep` also deletes:
(a) `admin.stepup_assertions` rows older than `stepup.retention_days` (phase 01);
(b) objects under `staging/` in `akr-quarantine` older than `evidence.staging_ttl_h` that have no evidence row or whose row is
    clean; it never deletes an infected or rejected object (phase 02 section 3);
(c) goAML `transient/` objects older than `evidence.staging_ttl_h` whose run is final or missing; the run is flagged
    `transient_deleted` and audited as `goaml_check.transient_deleted` with `by: sweep` (phase 07 section 3.2).
`backup_check` also sets the gauge `akr_failover_drill_age_days` from `ops.last_failover_drill_at` (phase 30 section 3.2).
```

### G1-00-12 Release bundle: chart and K3s artifacts, chart signature check
- Severity: MEDIUM
- Category: Missing Integration
- Target: phase-00.md section 4, bundle build and verification (lines 744 to 752)
- Sources:
  - phase-30.md:483 "Bundle (phase 00 section 3, step 7, extended): the bundle carries chart/ and k3s/"
  - phase-30.md:483 "the bundle verify of phase 00 section 4 checks the chart signatures"
  - phase-30.md:322 "the bundle carries the Helm charts (chart/akr ...) and the K3s artifacts"
  - phase-25.md:1265 "the release job calls the phase 00 bundle verify code path, which is not changed"
- Check: grep -n -i "chart\|k3s" docs/plan/phase-00.md -> line 845 only ("No Helm chart, K3s or automated failover (phase 30)").

Proposed 7.12 text:
```markdown
### 7.12 Release bundle: charts and K3s artifacts
- `akr-ctl bundle build` (step 7 of section 4) adds `chart/` (the Helm chart `chart/akr` and the third-party charts pinned by
  digest) and `k3s/` (the K3s binary, the air-gap images tarball of the pinned release, the install script). The manifest
  lists them with SHA-256 digests, as the other files do.
- `akr-ctl bundle verify` also verifies the chart signatures. Nothing else in the verify path changes (phase 30 section 3.12).
```

### G1-00-13 Schema list: train
- Severity: LOW
- Category: Missing Table
- Target: phase-00.md section 1, migration 0001 schema list (lines 51 to 62)
- Sources:
  - phase-31.md:30 "CREATE SCHEMA IF NOT EXISTS train AUTHORIZATION akr_migrate"
  - phase-31.md:1021 "DR-338 (the training workspace lives in its own schema train)"
  - gap-items.md (aa) "phase 00 schema list gains train"
- Check: grep -n "train" docs/plan/phase-00.md -> no schema line. Lines 51 to 62 list admin, audit, raw, canon, feat, scn, alert, casework, kyc, ref, rpt, assess.

Proposed 7.13 text:
```markdown
### 7.13 Schema list: train
Add to migration 0001, after the `assess` line (62): `CREATE SCHEMA IF NOT EXISTS train;`. Phase 31 section 1 keeps its
`CREATE SCHEMA IF NOT EXISTS train AUTHORIZATION akr_migrate` and its grants; the two statements are compatible.
```

### G1-00-14 Requests from phase 02: Config::validate refusal and coverage tests
- Severity: LOW
- Category: Missing Logic
- Target: phase-00.md section 2 (Config::validate) and section 6 (coverage tests)
- Sources:
  - phase-02.md:1066 "phase 00 Config::validate refuses TracingSink in production"
  - phase-02.md:1064 "phase 00 schema checks skip partitions (relispartition = true) in the classification and retention coverage tests, and now run over audit too"
- Check: grep -n -E "TracingSink|relispartition" docs/plan/phase-00.md -> none.

Proposed 7.14 text:
```markdown
### 7.14 Config and coverage tests for the audit partitions
- `Config::validate` refuses `TracingSink` when `AKR_ENV = production` (phase 02 section 2).
- The classification and retention coverage tests of section 6 skip partitions (`relispartition = true`) and run over the
  `audit` schema as well (phase 02 section 4).
```

## Phase 01 findings

### G1-01-01 identity_sweep: credential reset expiry, branch consistency, dormancy basis
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-01.md section 3, `identity_sweep` (lines 800 to 807)
- Sources:
  - phase-03.md:1098 "identity_sweep (phase 01) gains two steps: expire pending credential resets, and the consistency check"
  - phase-03.md:883 "Expiry: identity_sweep (phase 01) gains a step: pending resets past expires_at -> expired, audit credential_reset.expired"
  - phase-03.md:1098 "its dormancy rule reads enabled_at"
  - phase-03.md:1143 "admin.users gains the branch FK and enabled_at; identity_sweep reads it"
- Check:
  - grep -n "enabled_at\|credential_reset\|branch" docs/plan/phase-01.md -> no sweep step names them; `enabled_at` is absent from phase 01 (only the branch FK is noted, line 63 and 94).
  - Lines 801 to 807 list four steps (dormancy on `coalesce(last_authenticated_at, created_at)`, expiry, disabling, batches). No reset step and no consistency step.

Proposed 7.1 text:
```markdown
### 7.1 identity_sweep: resets, branch consistency and dormancy basis
- Step 1 (dormancy) reads `admin.users.enabled_at` (added by phase 03 section 1). The exact expression is UNCLEAR in
  phase 03 (it states only that the rule reads `enabled_at`); the lead must write it before implementation.
- Step 5 (pending credential resets): a `credential_resets` row in state `pending` with `expires_at <= now()` becomes
  `expired`, audited as `credential_reset.expired` with the system principal (phase 03 section 3.8).
- Step 6 (consistency check, no fix): pool branch lists and grant branch scopes that name an archived or unknown branch
  produce an error log and a cockpit count. Nothing is changed silently (phase 03 section 4.5).
```

### G1-01-02 akr-ctl db provision: monitoring and retention logins, polis database, audit writer grants
- Severity: HIGH
- Category: Missing Policy
- Target: phase-01.md section 4, Provisioning bullet (lines 917 to 918)
- Sources:
  - phase-03.md:1113 "akr-ctl db provision creates the LOGIN role akr_monitor"
  - phase-23.md:919 "the LOGIN role akr_retention_login with its password in OpenBao KV db/akr_retention_login"; phase-23.md:931 "created by akr-ctl db provision"
  - phase-31.md:898 "akr-ctl db provision creates the database polis and the LOGIN role polis ... It runs when pack.polis is switched on"
  - phase-02.md:1039 "Database provisioning (akr-ctl db provision ...): GRANT akr_audit_writer TO akr_app, akr_ingest, akr_assess (INHERIT)"
- Check: grep -n "provision" docs/plan/phase-01.md -> line 917 lists the `kratos` and `hydra` databases only. No akr_monitor, akr_retention_login, polis or audit-writer grant.

Proposed 7.2 text:
```markdown
### 7.2 akr-ctl db provision: additional logins, database and grants
`akr-ctl db provision` also does the following, in addition to the Ory databases above:
(a) grants LOGIN to `akr_monitor` with the password in OpenBao `db/akr_monitor`; the role is a member of `pg_monitor`
    only and has no privilege on any AKR schema (phase 03 section 4.4; role created in phase 00 section 1, see 7.4 there);
(b) grants LOGIN to `akr_retention_login` with the password in OpenBao `db/akr_retention_login`; it is a member of
    `akr_retention` and of `akr_audit_writer` (phase 23 sections 1.2 and 4.4);
(c) creates the database `polis`, owned by the LOGIN role `polis` with the password in OpenBao `ory/polis`; it runs only
    when `pack.polis` is switched on, not before (phase 31 section 4.5 item 2);
(d) runs `GRANT akr_audit_writer TO akr_app, akr_ingest, akr_assess` (INHERIT); `db check` asserts it (phase 02 section 4);
(e) the read-only pool needs no new role: `akr_readonly` already receives LOGIN in this provisioning step (phase 00 section 1).
```

### G1-01-03 admin.role_grants: support_session source value
- Severity: MEDIUM
- Category: Missing Fields
- Target: phase-01.md section 1, `admin.role_grants` DDL (lines 114 to 136)
- Sources:
  - phase-25.md:1255 "Phase 01 admin.role_grants (migration 0081, DR-237): support_session_id column; source CHECK widened to support_session"
  - phase-25.md:280 to 292 (the ALTER statements of 0081, including `role_grants_support_session_check`)
- Check: grep -n "support_session" docs/plan/phase-01.md -> none. Line 121 reads `source TEXT NOT NULL CHECK (source IN ('bootstrap', 'change_request'))`.

Proposed 7.3 text:
```markdown
### 7.3 admin.role_grants: support session source (documentation of migration 0081)
Phase 25 migration 0081 changes this table; its ALTER statements are in phase 25 section 1.2 and stay there (the
support session table `admin.support_sessions` is created by phase 25, so the foreign key cannot appear in this file).
After 0081 the `source` CHECK is `IN ('bootstrap', 'change_request', 'support_session')`, the column
`support_session_id` exists, and `role_grants_support_session_check` ties the two. Add a line under the DDL of this
section that says so, and change the `source` CHECK text above to show the three values with the note "(0081)".
```

### G1-01-04 Dev users: fixed users added by later phases
- Severity: MEDIUM
- Category: Incomplete Coverage
- Target: phase-01.md section 3, `identity seed-dev` (lines 796 to 798) and section 6 (curl suite)
- Sources:
  - phase-07.md:846 "fixtures/dev-users.yaml gains a second A2 user (a2b@bank.test) for the existence-masking test"
  - phase-25.md:1318 "plus two users this suite adds: sec2@bank.test (a second SEC holder ...) and vs@bank.test (a vendor user with no role ...)"
  - phase-24.md:1317 "Phase 01's dev users do not include it [as@bank.test]; the implementation adds it" (role AS, grant valid one day)
- Check: grep -n "one per role" docs/plan/phase-01.md -> line 797 only. grep -n -E "a2b@|sec2@|vs@|as@" docs/plan/phase-01.md -> none.

Proposed 7.4 text:
```markdown
### 7.4 Dev seed: fixed users beyond one per role
`identity seed-dev` creates one user per role (section 3) and these fixed extra users, all from `fixtures/dev-users.yaml`:
- `a2b@bank.test`, a second A2 user for the existence-masking test (phase 07 section 4.5);
- `sec2@bank.test`, a second SEC holder for four-eyes steps (phase 25 section 6);
- `vs@bank.test`, a vendor user with no role; the support session is its only grant (phase 25 section 6);
- `as@bank.test`, role AS, for the assessor pack and the training register (phase 24 section 4.5; grant valid one day).
Each has the fixed development password and TOTP seed of the other dev users.
```

### G1-01-05 Valkey rate limiter fails open
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-01.md section 3, rate limiting (around lines 574 to 576) and section 2 (limiter field, line 405)
- Sources:
  - phase-30.md:487 "The Valkey rate limiter answers without Valkey by failing open, and counts each such answer in akr_rate_limit_unavailable_total (a requirement; phase 01 does not state the behaviour today)"
- Check: grep -n -i "fail\|unavailable" docs/plan/phase-01.md -> no rule for the limiter when Valkey is down. The lockout rule uses the database (`auth.lockout_max_failures`, line 337).

Proposed 7.5 text:
```markdown
### 7.5 Rate limiter when Valkey is unavailable
If Valkey does not answer, the limiter allows the request (fail open) and increments
`akr_rate_limit_unavailable_total`. The sign-in lockout (`auth.lockout_max_failures`, counted in `admin.users`) and the
step-up checks do not depend on the limiter and keep working. (Phase 30 section 4.5 item 9.)
```

### G1-01-06 OpenAPI: step-up and role-scope extensions
- Severity: MEDIUM
- Category: Missing Integration
- Target: phase-01.md section 4, OpenAPI registration (lines 931 to 933) and section 2 (OpenAPI schemas)
- Sources:
  - phase-08.md:545 "Phase 01: x-akr-stepup on SU operations; ActionClass and ProblemCode schemas; x-akr-role-scopes"
  - phase-08.md:546 "the security scopes on every operation"
  - phase-01.md:931 "the cookie and bearer security schemes declared in phase 00 are referenced" (the only extension named)
- Check: grep -n "x-akr" docs/plan/phase-01.md -> none. grep -n "ActionClass" docs/plan/phase-01.md -> line 426 and 708 (Rust types), not OpenAPI schemas.

Proposed 7.6 text:
```markdown
### 7.6 OpenAPI extensions for the generated client
- Every step-up operation carries the extension `x-akr-stepup` with its action class name from the closed list (section 3).
- The document declares the schemas `ActionClass` (the closed list) and `ProblemCode` (the closed list of problem codes).
- The document carries `x-akr-role-scopes`: the role-to-scope table of `common::scopes::scopes_for_roles` (line 436), so
  the browser reads it instead of parsing descriptions.
- Every operation names its scopes in `security`.
The phase 08 client and the policy-matrix drift check read these (phase 08 section 4). Phase 03 adds `x-akr-action` to
the same operations.
```

### G1-01-07 default_queue_id rule lifted by phase 17
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-01.md section 3, preferences rule (line 701)
- Sources:
  - phase-17.md:1580 "the preferences rule 'default_queue_id must be null until queues exist in phase 17' is lifted"
  - phase-17.md:1581 "the id must be an active queue visible to the caller (422 E_VALIDATION at /default_queue_id otherwise)"
- Check: grep -n "default_queue_id" docs/plan/phase-01.md -> line 701 says "must be null until queues exist in phase 17".

Proposed 7.7 text:
```markdown
### 7.7 default_queue_id rule
Replace the sentence "`default_queue_id` must be null until queues exist in phase 17" with: "`default_queue_id` is null or the
id of an active queue visible to the caller. Until phase 17 creates queues, only null is accepted; from phase 17 the id is
checked, and a refusal is 422 `E_VALIDATION` at `/default_queue_id`. The field stays inside the preferences object (no
column, no foreign key)." (Phase 17 section 4.5 item 4.)
```

### G1-01-08 Step-up class for pack download (UNCLEAR, no text proposed)
- Severity: LOW
- Category: Missing Policy
- Target: phase-01.md section 3, closed class list (lines 708 to 709)
- Sources:
  - phase-06.md:866 "Phase 01: two step-up action classes (readiness_declare, pack_download)"
  - phase-24.md:5 "Phase 01 (StepUpService with the closed classes export, approval, record_open ...)"; status.md phase 24 row "download step-up `export`"
  - gap-items.md (t) "phase 06 step-up class pack_download is not in phase 01's closed list (use export)"
- Check: grep -n -E "pack_download|readiness_declare" docs/plan/phase-01.md -> none. The closed list at lines 708 to 709 has no such class.
- Proposed text: UNCLEAR. Phase 06 asks phase 01 for two new classes. Phase 24, status.md and gap-items (t) use `export` for the pack download and need no new class. Propose nothing to phase 01 until the lead chooses; phase 06 section 4.5 item 2 is the text to correct if `export` is chosen.

## Phase 02 findings

### G1-02-01 Evidence owner kinds registered by later phases
- Severity: HIGH
- Category: Missing Logic
- Target: phase-02.md section 3, EvidenceStore `stage` step 3 (line 844) and a new table in the evidence owner registry text
- Sources: one row per owning phase (line of the registration sentence in that phase):
  - phase-05.md:1379 "evidence owner kind `source` registered (retention R9)"
  - phase-06.md:867 "evidence owner kind `gap_report_run` registered (R6)"
  - phase-07.md:850 "evidence owner kind `goaml_profile` (R9)"
  - phase-09.md:1080 "the evidence owner kind `fx_rate` is registered for manual FX rates (evidence kind upload)"
  - phase-10.md:2194 "evidence owner kind `mapping_run` registered"
  - phase-12.md:1125 "evidence owner kind `coverage_certificate` registered (R6)"
  - phase-15.md:1817 "evidence owner kinds `kyc_review`, `pep_register`, `ownership_input`, `override`, `rating_feed_export` registered"
  - phase-16.md:1789 "Evidence owner kinds (phase 02 registry): `replay_sample`, `cbi_import`"
  - phase-17.md:1575 "the owner kind `alert` is added to the evidence owner registry"
  - phase-18.md:1128 "owner kind `case` is added to the evidence owner registry"
  - phase-19.md:1714 "the evidence owner registry gains `network_snapshot` (... kind `snapshot`, retention `R6`)"
  - phase-20.md:973 "the evidence owner registry gains `ops_task`"
  - phase-21.md:1263 "the evidence owner registry gains `report`, `report_package`, `filing`, `letter`, `periodic_report` and `fiu_feedback`"
  - phase-22.md:817 "Register `rfi_input`: the owner is the RFI ... retention class R6, classification personal"
  - phase-23.md:1010 "a sealed evidence object (kind package, owner legal_hold, class R6)"
  - phase-24.md:1163 "EvidenceStore owner kinds gain validation_pack (3.6) and training_record (3.10)"
  - phase-26.md:750 "the evidence owner registry gains islamic_deal (retention class R3)"
  - phase-27.md:781 "the evidence owner registry gains model_version"
  - phase-29.md:1344 "assess_extract added to the evidence owner registry"
  - phase-30.md:278 "owner kind ops_drill (new in phase 02's registry), R10"
- Check: grep -n "owner kind\|owner_kind" docs/plan/phase-02.md -> line 353 (column), 844 ("pack now; every owning phase adds its kind"), and the pack example. No list of kinds exists in phase 02. Phase 24 item 29 says "Gap pass 1 checks the registry's list."

Proposed 7.1 text:
```markdown
### 7.1 Evidence owner registry: kinds and owning phases
The evidence owner registry holds the kinds below. `stage` refuses an owner kind that is not in the registry (step 3).
Each row is registered by the phase named; the retention class is the one that phase states, or the lead assigns it where
the owning phase does not state one.

| Owner kind | Owning phase | Retention |
|-|-|-|
| `pack` | 02 (existing) | per pack kind |
| `source` | 05 | R9 |
| `gap_report_run` | 06 | R6 |
| `goaml_profile` | 07 | R9 |
| `fx_rate` | 09 (manual rates, evidence kind `upload`) | not stated |
| `mapping_run` | 10 | not stated |
| `replay_sample`, `cbi_import` | 16 | not stated |
| `kyc_review`, `pep_register`, `ownership_input`, `override`, `rating_feed_export` | 15 | not stated |
| `alert` | 17 | not stated |
| `case` | 18 | the owner of the case uploads (phase 18 section 3.4) |
| `network_snapshot` (kind `snapshot`) | 19 | R6 |
| `ops_task` | 20 | not stated |
| `report`, `report_package`, `filing`, `letter`, `periodic_report`, `fiu_feedback` | 21 | R6 or R7 (phase 21 item 11) |
| `rfi_input` | 22 | R6 (classification personal) |
| `legal_hold` (kind `package`) | 23 | R6 |
| `validation_pack`, `training_record` | 24 | not stated |
| `islamic_deal` | 26 | R3 |
| `model_version` | 27 | not stated |
| `assess_extract` | 29 | not stated |
| `ops_drill` | 30 | R10 |
| `coverage_certificate` | 12 | R6 |
```

### G1-02-02 EvidenceStore: transient staging methods for goAML
- Severity: HIGH
- Category: Missing Logic
- Target: phase-02.md section 3, EvidenceStore (after the `stage` and `open` text, lines 834 to 900)
- Sources:
  - phase-07.md:847 "EvidenceStore: stage_transient(purpose, owner, n, stream), open_transient(purpose, owner, n) and delete_transient(purpose, owner)"
  - phase-07.md:848 "(sealed AKRE1 objects under transient/<purpose>/ in akr-quarantine, no evidence row, s3guard policy for the prefix: stager credential writes, worker credential reads and deletes)"
- Check: grep -n "transient" docs/plan/phase-02.md -> none. The `staging/` text (line 1018) is the only staging rule in phase 02.

Proposed 7.2 text:
```markdown
### 7.2 EvidenceStore transient objects (for goAML)
Three methods on `EvidenceStore`, with the signatures of phase 07 section 3.2 (the types of `n` and `stream` are those
defined there):
- `stage_transient(purpose, owner, n, stream)`: writes a sealed AKRE1 object under `transient/<purpose>/` in
  `akr-quarantine`. No evidence row is written.
- `open_transient(purpose, owner, n)`: opens such an object for the worker.
- `delete_transient(purpose, owner)`: deletes the objects of that purpose and owner.
S3 guard policy for the prefix: the stager credential writes, the worker credential reads and deletes (section 4).
The `transient/` prefix is swept by `maintenance_sweep` (proposed 7.11 for phase 00).
```

### G1-02-03 EvidenceStore: shredding, deletion and lock extension for retention
- Severity: HIGH
- Category: Missing Logic
- Target: phase-02.md section 1, `admin.evidence_objects` (DDL from line 350) and section 3, EvidenceStore (line 890, the shredding paragraph)
- Sources:
  - phase-23.md:497 "EvidenceShredder (phase 02 EvidenceStore): shred(tx, evidence_id, reason) -> ShredOutcome ... delete_object(evidence_id) -> () (only after lock_until ...); extend_lock(tx, evidence_id, until) (the guard of phase 02 accepts only a later date)"
  - phase-23.md:672 "lock_until = greatest(...) through EvidenceShredder::extend_lock"
  - phase-23.md:932 "admin.evidence_objects. One column added (object_deleted_at, 0072) and its classification row"
- Check:
  - grep -n -E "shred\(|delete_object|extend_lock|object_deleted_at" docs/plan/phase-02.md -> none as method or column. The DDL (lines 370 to 407) already allows `shredded_at` and a later `lock_until`.
  - Line 890: "Shredding (phase 23) erases wrapped_dek and sets shredded_at" describes the effect but names no method.

Proposed 7.3 text:
```markdown
### 7.3 EvidenceStore disposal methods and the deletion column
- `shred(tx, evidence_id, reason) -> ShredOutcome`: erases `wrapped_dek` and sets `shredded_at` in the same transaction;
  audit `retention.evidence_shredded` (phase 23 section 2 ports). A locked object is shredded, never deleted, before its
  lock ends.
- `delete_object(evidence_id) -> ()`: removes the stored object only after `lock_until` has passed; the S3 guard refuses
  an earlier delete.
- `extend_lock(tx, evidence_id, until)`: sets `lock_until` only to a later date; the trigger of this section refuses
  a shorter one.
- DDL: `admin.evidence_objects` gains `object_deleted_at TIMESTAMPTZ` (nullable), set only by `delete_object`; its
  classification row is the one phase 23 section 1.1 defines (migration 0072 adds the column; this text documents it).
```

### G1-02-04 PartitionRegistry: entries of later phases
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-02.md section 3, PartitionMaintenance (lines 810 to 813)
- Sources:
  - phase-10.md:2165 "registers the ten partitioned canon tables with the factory canon.create_partition"
  - phase-13.md:909 "this phase adds three entries with the factory feat.create_partition"
  - phase-15.md:1845 "kyc.ratings in the partition registry with the factory kyc.create_partition"
  - phase-16.md:1794 "scn.replay_results and scn.shadow_results with the factory scn.create_partition"
  - phase-17.md:1575-1576 "alert.alerts is added to the partition registry with the factory alert.create_partition"
  - phase-29.md:1344 "PartitionRegistry gains one entry for assess.decisions ... the factory assess.create_partition"
- Check: grep -n "create_partition" docs/plan/phase-02.md -> none. Line 810 says "later phases register their tables with their own factory", with `audit_log` and `access_event` as the only entries.

Proposed 7.4 text:
```markdown
### 7.4 PartitionRegistry: entries registered by later phases
Each entry has its own factory and `ahead_months = audit.partition_ahead_months`. Registered entries, by phase:
- phase 10: the ten partitioned `canon` tables (list in phase 10 section 1), factory `canon.create_partition`.
- phase 13: `feat.feature_values`, `feat.segment_stats`, `feat.profiles`, factory `feat.create_partition`.
- phase 15: `kyc.ratings`, factory `kyc.create_partition`.
- phase 16: `scn.replay_results`, `scn.shadow_results`, factory `scn.create_partition`.
- phase 17: `alert.alerts`, factory `alert.create_partition`.
- phase 29: `assess.decisions`, factory `assess.create_partition`.
A missing month is created once by the writer and retried (SQLSTATE 23514; phase 13 line 910, as phase 10 does). There is no
DEFAULT partition.
```
(Schema of phase 13 tables: phase 13 section 1 names them; confirm the schema prefix before adoption.)

### G1-02-05 Audit catalogue: kinds emitted by phases 03 to 31
- Severity: MEDIUM
- Category: Missing Audit Event
- Target: phase-02.md section 2, `catalogue` (EventKind and CATALOGUE, lines 546 to 549) and the catalogue note at line 957
- Sources: the audit event table of each phase (section and approximate row count of dotted kinds; the count includes kinds reused from earlier phases):
  - phase-03.md:1004 (section 3.12, about 35); phase-04.md:1358 (about 35); phase-05.md:1228 (about 23); phase-06.md:714 (about 15); phase-07.md:700 (about 16); phase-09.md:913 (about 30); phase-10.md:2043 (about 26); phase-11.md:1161 (about 30); phase-12.md:951 (about 25); phase-13.md:804 (about 18); phase-14.md:1154 (about 12); phase-15.md:1604 (about 38); phase-16.md:1570 (section 3.17, about 17); phase-17.md:1336 (section 3.16, about 37); phase-18.md:914 (section 3.18, about 15); phase-19.md:1337 (section 3.12, about 18); phase-20.md:775 (section 3.10, about 32); phase-21.md:1049 (section 3.20, about 30); phase-22.md:704 (section 3.11, about 29); phase-23.md:791 (section 3.12, about 20); phase-24.md:1067 (about 46); phase-25.md:1131 (section 3.13, about 49); phase-26.md:584 (section 3.12, about 12); phase-27.md:683 (section 3.13, about 16); phase-28.md:670 (section 3.12, about 16); phase-29.md:1172 (section 3.17, about 18); phase-30.md:400 (section 3.17, about 14); phase-31.md:721 (section 3.14, about 15)
- Check: grep -n "CATALOGUE" docs/plan/phase-02.md -> line 547 (the constant). Line 957 says the catalogue also lists "the 4 kinds of phase 00 and the 22 of phase 01" and nothing from phases 03 to 31. phase-16.md:1821 to 1823 and phase-25.md:1257 ask for catalogue additions.

Proposed 7.5 text:
```markdown
### 7.5 Audit catalogue: kinds emitted by later phases
`CATALOGUE` holds every kind that any phase emits. Each phase's audit table gives, for every kind, its resource type,
severity, SIEM flag and detail keys (the sections listed in the Sources of this finding). A kind that is not
listed is added with those values; a listed kind is reused. A test serialises a sample of every kind and fails on a
detail key outside the catalogue (phase 02 section 3, already stated).
Resource types added for phase 16 (section 4.4 of phase 16): `scenario`, `scenario_version`, `parameter_set`, `replay`,
`replay_sample`, `seeded_test`, `detection_key`, `catalogue_import`, `system`.
The kinds themselves are copied from those sections by the lead; they are not repeated here.
```

### G1-02-06 audit_anchors: column grant for akr_app
- Severity: MEDIUM
- Category: Missing Policy
- Target: phase-02.md section 1, grants for `audit.audit_anchors` (lines 304 to 308)
- Sources:
  - phase-11.md:1363 "audit.audit_anchors.anchored_at readable by akr_app (column grant in 0034, for the declaration check)"
  - phase-11.md:1364 "partition_maintenance unchanged" (no other change to phase 02)
- Check: grep -n "audit_anchors" docs/plan/phase-02.md -> line 304 grants INSERT to akr_audit_writer; line 307 SELECT to akr_audit_writer; line 308 SELECT to akr_readonly. No grant to akr_app.

Proposed 7.6 text:
```markdown
### 7.6 Column read for akr_app on audit_anchors
`GRANT SELECT (anchored_at) ON audit.audit_anchors TO akr_app;`
Used by the declaration check of phase 11 (section 4.4). Only this column is granted; the anchor rows stay unreadable to
`akr_app` otherwise.
```

### G1-02-07 case_event anchor: one chain or one chain per case
- Severity: MEDIUM
- Category: Missing Logic
- Target: phase-02.md section 2 (line 527, the `case_event` reservation) and section 3, chain model (lines 645 to 647)
- Sources:
  - phase-17.md:1819 "The phase 02 stream case_event is the anchor: its payload gains the heads of the cases that changed since the previous anchor"
  - phase-17.md:1820 "the chain is reserved for the case, where a decision is made" (alerts are not chained)
  - phase-17.md:1901 "the case_event anchor payload for per-case chains (DR-122; phase 02 reserved one chained stream)"
  - research.md:133 DR-122 "Case event chain (phase 17): one hash chain per case under the case row lock"
- Check: phase-02.md:645 "Each stream (audit_log, access_event; case_event from phase 17) is one hash chain that runs through its monthly partitions without a break".

Proposed 7.7 text:
```markdown
### 7.7 case_event: one chain per case
- The stream `case_event` is chained per case, not as one chain: each case has its own hash chain, written under the case
  row lock (DR-122, phase 17 section 3.7). Alert events are not chained.
- The anchor for `case_event` carries, in its payload, the digest of the heads of the cases that changed since the
  previous anchor, so the external anchor records what every case said at that time (phase 17 section 3.7).
- The chain rule of this section (one chain per stream, through the monthly partitions) applies to `audit_log` and
  `access_event` only.
```

### G1-02-08 admin.packs kind CHECK: final list
- Severity: MEDIUM
- Category: Missing Fields
- Target: phase-02.md section 1, `admin.packs` DDL (lines 420 to 423)
- Sources:
  - phase-06.md:257 to 261 "Phase 02 packs: two more kinds ... ALTER TABLE admin.packs ... CHECK (kind IN (... 'integration_profile', 'vendor_request'))"
  - phase-24.md:235 to 238 "Pack kind certification ... The phase 02 CHECK, widened by phase 06, is replaced by the same list plus certification"
  - phase-24.md:1163 "the kind CHECK gains certification (0078)"; phase-24.md:1163 also says "PackService::publish is called with the five kinds" (only certification is new in the CHECK; the other four kinds are already listed)
- Check: lines 422 to 423 read `kind IN ('board', 'case', 'report', 'assessor', 'correspondent', 'validator', 'cbi_gap')`.

Proposed 7.8 text:
```markdown
### 7.8 admin.packs: kind list after phases 06 and 24
The table DDL lists the kinds as they stand after migrations 0021 (phase 06) and 0078 (phase 24):
`kind IN ('board', 'case', 'report', 'assessor', 'correspondent', 'validator', 'cbi_gap', 'integration_profile', 'vendor_request', 'certification')`.
The ALTER statements of phases 06 and 24 stay in those phases. Phase 24 section 4.5 item 1 says "five kinds" are new; only
`certification` is new in the CHECK (lead to correct the wording in phase 24).
```

### G1-02-09 access_event.action CHECK: profile.read value
- Severity: HIGH
- Category: Missing Fields
- Target: phase-02.md section 1, `audit.access_event` DDL, column `action` (line 148), and section 2 (AccessAction, line 605)
- Sources:
  - phase-13.md:934 "Phase 02: PartitionRegistry gains three feat entries; the access layer gains the profile.read action"
  - phase-15.md:1859 "profile.read (GET /parties/{id}/profiles) gains the masking of the customer kind with no route change"
- Check: phase-02.md:148 reads `action TEXT NOT NULL CHECK (action IN ('view', 'search_hit', 'export', 'print', 'download', 'unmask'))`. No `profile.read`. A row with that action would fail the insert.
- Note: `profile.read` is also a Cedar action name in phase 01 (line 859) for the self profile. The lead should confirm that the access-event value is meant (phase 13 says so).

Proposed 7.9 text:
```markdown
### 7.9 access_event.action: profile.read
Add `profile.read` to the `action` CHECK of `audit.access_event`:
    action TEXT NOT NULL CHECK (action IN ('view', 'search_hit', 'export', 'print', 'download', 'unmask', 'profile.read'))
`AccessAction` (section 2) lists the same values. The value is written by the read of `GET /parties/{id}/profiles`
(phase 13 section 4.5; phase 15 section 4.4). Because `access_event` is append-only, make the change through the migration
that drops and re-adds the CHECK, as the audit DDL rule of section 4 says (`SET LOCAL akr.audit_ddl = ...`).
```

### G1-02-10 Keyed-hash purpose islamic.document_ref (UNCLEAR, no text proposed)
- Severity: LOW
- Category: Missing Policy
- Target: none determined (UNCLEAR)
- Sources:
  - phase-26.md:914 "The keyed hash of the document reference uses the phase 02 purpose islamic.document_ref; the purpose list of phase 02 gains it"
  - phase-00.md:582 "keyed_hash(purpose, data) -> [u8; 32]" (the crypto port, phase 00)
  - phase-01.md:405 "Keyed hashes use the phase 00 crypto port with three new purposes, csrf, token and login_identifier"
- Check: grep -n -i "purpose" docs/plan/phase-02.md -> `PurposeCode` (access purposes, admin data from phase 03) only. Phase 02 has no keyed-hash purpose list.
- Proposed text: UNCLEAR. Phase 26 names a phase 02 list that does not exist; keyed-hash purposes are a phase 00 crypto-port concept (phases 00 and 01). The lead should decide the home of the purpose list before any text is added. Gap item (v) in gap-items.md repeats the phase 02 wording.

## Summary

Findings by severity (32 in total):
- HIGH: 10 (G1-00-01, G1-00-02, G1-00-04, G1-00-05, G1-00-07, G1-01-02, G1-02-01, G1-02-02, G1-02-03, G1-02-09)
- MEDIUM: 18 (G1-00-03, G1-00-06, G1-00-08, G1-00-09, G1-00-10, G1-00-11, G1-00-12, G1-01-01, G1-01-03, G1-01-04,
  G1-01-05, G1-01-06, G1-01-07, G1-02-04, G1-02-05, G1-02-06, G1-02-07, G1-02-08)
- LOW: 4 (G1-00-13, G1-00-14, G1-01-08, G1-02-10)

Findings by target: phase 00 has 14, phase 01 has 8, phase 02 has 10. Each target's Section 7 numbering runs 7.1 to 7.N in
the order given above.

Obligations already present and skipped: about 50. The grouped list, so the lead can spot-check:
- Phase 00: FK of `admin.timers.calendar_id` deferred to phase 09; `ref` and `canon` schemas; job RLS (`jobs_app_all`,
  `jobs_ingest_own` with `ingest_` and `discovery_`); AppRole `akr-ingestd` on `sources/*`; keys `akr-sign-anchor`,
  `akr-sign-certificate`, `keys/cursor-hmac`, `keys/match-key`, `akr-kek`; roles `akr_readonly` and `akr_audit_writer`;
  zones `z1_cbs`, `z1_db`, `z1_bao`, `z2_app`, `z2_store`, `z3_ory`, `z3_edge`, `z4_cache`, `z4_gateway`, `z5_mgmt`;
  `gotenberg` and `clamd` on `z2_app`; `outbox.max_attempts`; `jobs.lease_s`; `maintenance_sweep` and `backup_check`
  schedules; the `frontend.md` and `form-templates.md` ports (stated as phase 08 work).
- Phase 01: `branch` FK deferred to phase 03; `identity_sweep` job; `admin.users` kind `service` with `owner_user_id`
  and `expires_at`; `siem.pull` token allow-list; `/me/notifications` rows moved to phase 22; the closed class list
  (`export`, `decision`, `approval`, `signing`, `record_open`, `backfill`, `break_glass`, `filing`, `access_change`,
  `identity_admin`); role `VS` in the role CHECK; the one-per-role dev seed including `mro@`; the Valkey limiter itself.
- Phase 02: `audit.partition_ahead_months`; `evidence.lock_days`; `evidence.staging_ttl_h`; `put_generated`, `stage`,
  `open`, `PgAccessLog`; SELECT on the audit tables for `akr_readonly`; `E_UPLOAD_REJECTED` and `E_PAYLOAD_TOO_LARGE`
  mapping; `case_event` reserved in the anchor CHECK; `admin.packs` CHECK with `correspondent`, `validator`, `cbi_gap`;
  `wrapped_dek`, `shredded_at` and the lock-extend-only trigger; `audit_anchors.anchored_at`; the partition registry
  mechanism; `purpose_code` admin list; owner kind `pack`.

UNCLEAR (no text proposed, or a choice for the lead):
1. G1-01-08: the pack download step-up class. Phase 06 section 4.5 asks for `pack_download` and `readiness_declare` in
   phase 01; phase 24 and status.md use `export`.
2. G1-02-10: the home of the keyed-hash purpose `islamic.document_ref`. Phase 26 names a phase 02 list that does not exist.
3. G1-00-05: the KV mount prefix. Phase 00 mounts `akr/`; phases 25 and 31 write `kv/...`, phase 28 writes `secret/...`,
   and phases 01 and 31 write `ory/...`.
4. G1-00-06: key names. Phase 23 uses `akr-sign-cert`, phase 25 uses `akr-sign-bundle` (its Open doubts 9 call it new), and
   phases 00 and 12 use `akr-sign-certificate`.
5. G1-01-01: the dormancy expression that reads `enabled_at` (phase 03 says only that it reads it).
6. G1-02-09: `profile.read` is an access-event value in phase 13 and also a Cedar action name in phase 01. The lead should
   confirm the access value before the CHECK changes.
7. G1-02-04: the schema prefix of the phase 13 feature tables (`feat.` assumed; confirm from phase 13 section 1).
8. G1-02-05: the audit kind names are not copied into this file. The lead copies them from the listed sections; the count
   per phase is approximate.

Not in scope but noticed (not phase 00, 01 or 02 text): phase 24 section 4.5 item 1 says "five kinds" where only one kind
(`certification`) is new in the CHECK; API.md section 6 lacks the phase 02 codes `E_UPLOAD_REJECTED`,
`E_PAYLOAD_TOO_LARGE` and `E_UNAVAILABLE` (gap item v); gap item (h) (freeze-list alert retention R5 or R7) is a bank
decision for phase 14.

No file other than tools/agent/gap1-run1.md was written. No subagents were used.
