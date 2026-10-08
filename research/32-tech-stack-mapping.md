# 32 — Tech-stack mapping: every AKR-TMS component against the digital-banking-system stack and conventions

Status: done (2026-10-06); updated 2026-10-07 for the owner's stack decisions: MinIO kept (section 12), OpenBao replaces Vault (section 13), Valkey replaces Redis (section 19)

Scope: AKR-TMS must follow the stack and conventions of /home/haithem/Projects/digital-banking-system (the "DBS": Rust, Axum 0.8, SQLx 0.8, PostgreSQL, Redis, MinIO, Vault, ORY Kratos and Hydra, Cedar, React 19, TanStack, Orval, Tailwind v4, Docker, K3s, Prometheus, Grafana, Loki, Gitea, Harbor). This file reads that repository (read only), maps each AKR-TMS component onto it, and lists where the DBS stack has nothing for the job, with a stack-consistent proposal and the reasons. Requirement IDs use the prefix TS. Cross-references: file 25 (integration patterns and CBS databases), file 21 (technology approaches, written by another agent), file 22 (Arabic names), file 29 (model governance), file 30 (security), file 31 (analyst UX).

## Contents

1. How the DBS was read and which conventions bind AKR-TMS
2. Component map: AKR-TMS component to DBS element (reuse, extend, new)
3. Workspace layout and bounded contexts for AKR-TMS
4. CBS drivers in Rust (SQLx limits; oracle, tiberius, odbc-api; sidecars; when Debezium is justified)
5. Ingestion workers and event streaming (what the DBS uses; what AKR-TMS needs)
6. Storage design on PostgreSQL (partitioning, retention, when a columnar store is justified)
7. The rules and scenario engine
8. ML inference in Rust and offline training
9. Graph analytics
10. Name-matching and screening index
11. Case management UI and workflow reuse (maker-checker)
12. Evidence vault on object storage (MinIO kept: maintained code line, hardening, advisory watch)
13. Secrets and keys (OpenBao replaces Vault; envelope encryption)
14. Identity, authorisation and RBAC (Kratos, Hydra, Cedar, directory federation)
15. Observability
16. Packaging: K3s on-prem, air-gapped updates through Harbor, supply chain
17. Single-tenant per bank versus a hosted multi-tenant option
18. Frontend: React 19, Orval, Tailwind v4, Arabic RTL and i18n, charts, graph views, tables
19. Licence review of the stack as a product sold to banks
20. Gap list and stack-consistent proposals
21. What it means for AKR-TMS
22. Derived requirements
23. Sources

Evidence marks used throughout: [V] read in a primary document, repository or registry during this research; [K] practitioner knowledge or inference to be confirmed; [S] secondary source; "measured" marks results produced for this file. Section 21 lists open items; section 20 is the gap list.

---

## 1. How the DBS was read and which conventions bind AKR-TMS

### 1.1 What was read (read only; nothing in the repository was changed)

`tech-stack.md`, `CLAUDE.md`, `PRODUCT.md`, `DESIGN.md`, `docs/ARCHITECTURE.md`, `docs/MODULES.md`, `docs/plan/research.md` (DR-96, DR-97, DR-107 to DR-113), `docs/plan/backup-restore.md`, `docs/plan/completion.md`, `.claude/rules/` (`ddd`, `rust-backend`, `auth`, `jobs`, `admin-managed`, `migrations`, `frontend`), the workspace and crate `Cargo.toml` files, migrations 0003, 0017 and 0019, the `audit`, `authz` and `common` crates, `domain/src/sensitive.rs`, the Cedar bundle `backend/policies/bundle-001`, `docker-compose.yml`, `deploy/prod/` (compose overlay, README, `vault.hcl`, `pgbackrest.conf`, `Caddyfile`, `jaeger.yaml`), `.github/workflows/ci.yml` and `frontend/package.json`.

### 1.2 What the DBS is

The DBS (shipping as WFIS at Asia Al-Iraq Islamic Bank) is a **modular monolith**: an Axum 0.8 `api` binary and a `worker` binary over shared crates, PostgreSQL 17 (the PGroonga image) through SQLx 0.8, Redis (cache, rate limits and SSE fan-out only), MinIO (documents and WORM anchor buckets), HashiCorp Vault (transit signing, field-key wrapping, KV), ORY Kratos and Hydra behind a backend-for-frontend, Cedar authorisation in process, a React 19 SPA (TanStack Router and Query, Orval-generated client, Tailwind v4, Untitled UI components on react-aria, i18next) and Capacitor shells for phones. Arabic is the default language and the design origin. It runs on the bank's own network, with default-deny egress and a rule that document content never leaves the bank.

### 1.3 Conventions that bind AKR-TMS

| Convention | Where it is written | What it means for AKR-TMS |
|---|---|---|
| Cargo workspace layered by crate: `domain` (no axum, no SQL), `db` (sqlx adapters), `authz`, `audit`, `common`, `api`, `worker`; layer = crate, bounded context = module | `.claude/rules/ddd.md` | Same layering; the compiler enforces it. New contexts follow the nine-step checklist |
| Compile-time checked SQL (`sqlx::query!`, `query_as!`, offline `.sqlx` cache); never runtime string queries | `CLAUDE.md` | Applies to AKR-TMS's **own** PostgreSQL. CBS reads cannot be compile-time checked (the schema is the bank's); section 4 records this as one deliberate, narrow exception |
| API-first: `#[utoipa::path]` and `OpenApiRouter::routes!`; the OpenAPI document is the contract; TypeScript types and hooks come only from Orval | `CLAUDE.md`, `api-design.md` | Unchanged, for the analyst API and the phase-2 assessment API |
| Errors are RFC 7807 problem+json with stable codes; 5xx never leaks detail; idempotency keys with byte-exact replay (`E_IDEMPOTENCY_REUSE`, `E_IDEMPOTENCY_IN_FLIGHT`) | `rust-backend.md`, migration 0003 | Reuse as is; the assessment API depends on it (file 25 section 15) |
| Background work: a Postgres `jobs` table polled with `FOR UPDATE SKIP LOCKED`, a transactional outbox and timers; handlers idempotent; Redis is never a queue | `.claude/rules/jobs.md` | The ingestion, reconciliation, scoring and alert pipeline runs on this harness |
| Authentication by Kratos sessions through a BFF (browser flows, CSRF bound to the session), step-up AAL2 as a single-use ledger row; authorisation by Cedar with two gates (token scope, then policy), every decision audited, existence masking for clearance denials | `.claude/rules/auth.md` | Reuse; add directory federation (section 14) |
| "Nothing is a fixture": what the bank may want different next month lives in the database behind a screen, audited, versioned where it governs history | `.claude/rules/admin-managed.md` | Scenarios, thresholds, code maps, mappings, fail policies, lists settings are admin-managed data |
| Migrations: one SQLx chain, never edit an applied migration, `TIMESTAMPTZ` only, UUID keys | `migrations.md` | Same |
| Audit: per-partition hash chain, Merkle anchors signed by a Vault transit key every five minutes, copy in a COMPLIANCE-locked bucket; reads of sensitive resources always logged | `ARCHITECTURE.md` section 8 | Reuse unchanged as the AKR-TMS audit trail; extend with customer-view logging (file 30) |
| Field-level envelope encryption: Vault wraps data keys, plaintext keys cached in process, AES-GCM locally; Vault is never in the request hot path | `ARCHITECTURE.md` section 10 | Reuse; the phase-2 API follows the same rule |
| i18n: every string ships in Arabic and English; logical CSS only (`ms-*`, `me-*`, `ps-*`, `text-start`); directional icons flip; Western digits; `YYYY-MM-DD HH:MM`; no `toLocaleString`; no `input type=date`; fonts vendored, no runtime font requests | `frontend.md`, `MODULES.md` 16.5, `PRODUCT.md` | Reuse as hard rules |
| Egress: internal network only, default-deny, one allow-listed egress; document content never leaves the bank; external AI APIs prohibited | `ARCHITECTURE.md` D6 and section 11 | Same; AKR-TMS may allow-list list providers only if the bank approves (file 01 section 8) |
| Testing regime: real curl tests, at least ten request cases per endpoint, audit assertions on every mutation, concurrency tests | `testing.md` | Same, plus reconciliation and mapping conformance suites (file 25) |
| Context7 lookup before using any crate; `cargo add` rather than hand-edited versions | `CLAUDE.md` | A development rule for the AKR-TMS repository too |

### 1.4 What the stack document says versus what the repository has built

The lead's brief says to follow the DBS stack. The documents and the repository do not fully agree, and AKR-TMS must plan for the difference:

| Area | `tech-stack.md` and `ARCHITECTURE.md` | The repository as built (October 2026) | Consequence |
|---|---|---|---|
| Orchestration | K3s on two servers, Helm | Production is **docker compose** with a Caddy edge and scripts (`deploy/prod/bootstrap.sh`, `push-images.sh`, `release.sh`, `rollback.sh`); the K3s and Helm layer "exists only in ARCHITECTURE.md" (`completion.md`, theme 1); DR-107 decides to build it | AKR-TMS must either build the K3s and Helm layer or ship a compose profile first. Section 16 proposes both from one set of images |
| CI and registry | Gitea Actions, Harbor with Trivy, Helm deploy | `.github/workflows/ci.yml`, manual dispatch only; images are built on a development machine and pushed by script | Build the Gitea and Harbor pipeline new; GitHub Actions YAML ports to Gitea Actions with little change |
| Metrics, dashboards, logs | Prometheus, Grafana, Loki | The API exposes `/metrics` through `metrics-exporter-prometheus`; spans go over OTLP to a Jaeger container; an in-app ops cockpit (MODULES M15) shows job queues, backups and anchors; no Prometheus, Grafana or Loki service appears in the compose files | Define the observability plane for banks (section 15) |
| Queue | Postgres queue, NATS "if throughput demands" | Postgres `jobs` only | Same choice for AKR-TMS; NATS only as a CDC sink (section 5) |
| Database HA | Primary plus hot standby, manual failover | pgBackRest with WAL archiving every minute, restic for buckets and Vault snapshots, a restore drill script; the standby is a funded decision (DR-108) | Add automated failover for three-node banks (section 16) |
| Object store | MinIO, object lock for finals | MinIO `RELEASE.2025-04-22` and `mc`, buckets created with `--with-lock` and a 3650-day COMPLIANCE default | **Both upstream repositories are now archived**; AKR-TMS keeps MinIO as a maintained code line (section 12) |
| Secrets | Vault | Vault 1.17 on raft, sealed after restart, unsealed by key holders | Licence is BUSL; AKR-TMS uses OpenBao (section 13) |
| Cache | Redis | `redis:7-alpine` (a floating tag that resolves to a release under RSALv2 or SSPLv1) | AKR-TMS uses Valkey (section 19) |
| Identity versions | Kratos, Hydra | Kratos v1.2.0 and Hydra v2.2.0; the current upstream release is v26.2.0 (Ory moved to calendar versions) | Plan the upgrade path before a product ships on old versions |
| Crate versions | | `cedar-policy` 4.7.1 (current 4.13.0), `jsonschema` 0.49.9 (0.58.6), `sqlx` 0.8 (0.9.0 exists; DBS DR-31 pins the CLI to 0.8 because the offline cache format changed) | AKR-TMS starts on the same majors and upgrades deliberately |

The practical reading: AKR-TMS can copy the **application architecture** (it is proven, documented and tested) almost whole. It cannot copy the **platform layer**, which has to be built, and it must replace one component (object storage) before it ships.

### 1.5 AKR-TMS hooks

- **Convention checklist as a repository file** (`CONVENTIONS.md`) listing the rows of 1.3 with a pointer to the DBS rule and a note where AKR-TMS deviates (for example CBS reads).
- **Version policy**: a pinned set of majors at project start, a quarterly dependency review, and `cargo deny` plus `cargo audit` in CI from the first commit (section 16).
- **Decision records** for the deliberate deviations in this file, in the DBS style (a numbered DR with the reason).

---

## 2. Component map: AKR-TMS component to DBS element

Verdicts: **Reuse** (copy the pattern or crate nearly unchanged), **Extend** (same machinery, new bounded contexts), **New** (nothing in the DBS; a stack-consistent choice is proposed).

| # | AKR-TMS component | DBS element it builds on | Verdict | Gap and proposal (section) |
|---|---|---|---|---|
| 1 | Analyst and admin API | `dbs-api`: Axum 0.8, `OpenApiRouter`, problem+json, idempotency, middleware stack | Reuse | None (3) |
| 2 | Ingestion daemon (`ingestd`): pollers, CDC consumer, file drops, reconciliation | `dbs-worker` poll loop, `jobs` harness, per-kind semaphores | Extend | New binary in its own network zone (3, 5) |
| 3 | CBS drivers | SQLx handles PostgreSQL only | New | `tiberius`, `odbc-api`, `oracle`, Debezium Server sidecar (4) |
| 4 | Canonical store | PostgreSQL 17, SQLx, migrations, `partition_maintenance` job | Reuse | Partitioning and retention classes (6) |
| 5 | Scenario and rules engine | `dsl` crate (JSON Schema validated definitions), timers, jobs | Extend | Scenario DSL with SQL and CEL (7) |
| 6 | Screening and name matching | PGroonga and `dbs_search_normalise`, `dbs_ocr::agreement::normalise` | Extend | In-memory index plus pg_trgm; measured (10) |
| 7 | ML scoring | None | New | ONNX Runtime through `ort`, offline training, model registry (8) |
| 8 | Graph analytics | None | New | Edge tables, recursive CTEs, `petgraph` (9) |
| 9 | Case management and approvals | Workflow engine (events, tasks, timers, delegation), notifications outbox, documents | Extend | Case aggregate plus reused approval flows (11) |
| 10 | Maker-checker and change control | Step-up ledger, Cedar, workflow approvals | Reuse | Generic change-request pattern (11, 14) |
| 11 | Evidence vault | S3 store, WORM buckets, SHA-256 on serve, antivirus (clamd), PDF stamping (Gotenberg) | Reuse | **MinIO kept as a maintained, hardened code line** (12) |
| 12 | Audit trail | `dbs-audit`: hash chain, Merkle anchors, Vault transit signature, WORM copy | Reuse | Add customer-view logging (file 30) |
| 13 | Secrets and keys | Vault: transit, KV, PKI; field-level envelope encryption | Reuse | **OpenBao replaces Vault** (13) |
| 14 | Authentication and authorisation | Kratos, Hydra, Cedar bundles in the database, simulator and dry-run | Reuse | Directory federation (14) |
| 15 | Notifications | Outbox, in-app, SSE | Extend | **No WhatsApp or external channel** (tipping-off); in-app and the bank's SMTP (11) |
| 16 | Regulatory reports (goAML XML, CBI reports, board packs) | Reports module, Gotenberg PDF with Arabic fonts, CSV and zip crates | Extend | XML generation and XSD validation (file 23; 20) |
| 17 | Phase-2 assessment API | Idempotency ledger, Cedar, transit key scheme (QR tokens) on OpenBao, timeouts | New binary | Separate `assess-api` (3; file 25 section 15) |
| 18 | Observability | `tracing`, OTLP to Jaeger, `/metrics`, ops cockpit | Reuse | Prometheus, Grafana, Loki not built (15) |
| 19 | Deployment | Compose bundle, Caddy, pgBackRest, restic, release and rollback scripts | Extend | K3s and Helm, air-gap bundle (16) |
| 20 | Frontend | React 19, TanStack, Orval, Tailwind v4, Untitled UI, i18next, RTL rules, SSE bridge | Reuse | Charts, large tables, graph views (18) |
| 21 | Mobile | Capacitor shells | Not needed | MLRO approvals on a phone are a later question |

---

## 3. Workspace layout and bounded contexts for AKR-TMS

### 3.1 Four binaries, one workspace

Four binaries over shared crates (the DBS has two). The extra two exist for **security zoning**, not for scale:

| Binary | Role | Network zone | Why separate |
|---|---|---|---|
| `akr-api` | Analyst, MLRO, admin and report API; SSE; BFF for Kratos | Analyst zone (behind the edge proxy) | Internet-style attack surface is the browser; holds no CBS credentials |
| `akr-worker` | Jobs: scenarios, scoring, features, alerts, reports, notifications, anchors, backups | Application zone | Same as the DBS worker |
| `akr-ingestd` | The **only** process with a route to the CBS: pollers, CDC consumer, file landing readers, reconciliation | CBS-facing zone (firewalled to listener ports) | Limits what a compromise of the web tier can reach; holds the read-only credentials; can be stopped by the bank's kill switch without touching analysts (file 25 sections 5.4 and 10.4) |
| `akr-assess` | Phase-2 synchronous assessment API (file 25 section 15) | Gateway zone | Own SLOs, own scaling, signed responses, no analyst traffic |

The DBS decision D14 ("modular monolith: one `api` binary and one `worker` binary, no microservices") is kept in spirit: one workspace, shared domain crates, no inter-service RPC. The four processes talk through PostgreSQL (jobs and outbox) and the object store.

### 3.2 Crates (layer = crate, context = module)

```text
backend/
  Cargo.toml                       # workspace, edition 2024, unsafe_code = "forbid" (as DBS)
  migrations/                      # one SQLx chain
  crates/
    common/       akr-common       config, errors/problem+json, correlation, idempotency, jobs harness,
                                    crypto helpers, vault client, objectstore, telemetry
    domain/       akr-domain       entities + ports; modules: party, account, txn, payment, case, scenario,
                                    screening, kyc, report, evidence, mapping, source, recon, assess
    db/           akr-db           Pg* adapters, partition helpers, read-only reporting pool
    authz/        akr-authz        Cedar facade (clearance, case ACLs, branch scope)
    audit/        akr-audit        hash chain, anchors (reuse of dbs-audit pattern)
    dsl/          akr-dsl          scenario and mapping documents, JSON Schema, CEL evaluation
    mapping/      akr-mapping      transform library, code maps, SQL generator, drift detection
    ingest/       akr-ingest       SourceReader trait, batch envelope, watermarks, reconciliation ladder
    drv-pg/ drv-mssql/ drv-oracle/ drv-odbc/   feature-gated CBS drivers (section 4)
    screening/    akr-screening    list ingestion, normalisation, in-memory index, scorer
    scoring/      akr-scoring      feature computation, ONNX inference wrapper, model registry client
    graph/        akr-graph        entity links, components, ego graphs
    reports/      akr-reports      goAML XML, CBI reports, board packs, PDF requests
    integrations/ akr-integrations SMTP, SIEM syslog/CEF export, list providers (allow-listed)
  api/  worker/  ingestd/  assess/ # the four binaries
```

Rules carried over from the DBS: domain has no `axum` and no SQL execution; contexts never import each other (cross-context needs go through a trait in `common` or `domain` wired in `main.rs`); adapters implement ports; handlers contain no business logic.

### 3.3 Bounded contexts

`source` (connections, profiles, load budgets, kill-switch state), `ingest` (batches, receipts, watermarks), `mapping` (specifications, code maps, versions), `canonical` (party, account, transaction, payment, product, branch, card, facility, trade), `recon` (ladder results, coverage certificates, gaps), `screening` (lists, entries, aliases, results, whitelists), `scenario` (definitions, versions, parameters, runs, backtests), `scoring` (features, models, scores), `alert` and `case` (alerts, cases, notes, decisions, STR packages), `kyc` (risk rating, reviews, EDD), `report` (regulatory, board), `evidence` (vault objects, legal holds), `admin` (settings, flags, roles, users, change requests), `assess` (decisions, fail policies). Each follows the DBS nine-step checklist.

### 3.4 AKR-TMS hooks

- **Workspace template** generated from the DBS layout (same Makefile targets: `make check`, `lint`, `migrate`, `generate-api`, `verify`).
- **Network policy per binary** shipped in the Helm chart and documented for compose (which container may reach which port).
- **One shared `akr-common` crate** extracted by copy from the DBS patterns (the DBS repository is read only and not a dependency); a later extraction into a private crate in the vendor's Gitea Cargo registry avoids long-term divergence.

---

## 4. CBS drivers in Rust

### 4.1 The constraint

SQLx, the DBS database layer, supports PostgreSQL, MySQL, MariaDB and SQLite. Its own README says "MSSQL was supported prior to version 0.7, but has been removed pending a full rewrite of the driver as part of our SQLx Pro initiative"; there is no Oracle or Db2 driver (SQLx 0.9.0 released 21 May 2026; the DBS stays on 0.8) [V, GitHub README]. SeaORM sits on the same drivers. AKR-TMS must reach Oracle, SQL Server, Db2, Sybase and others that Iraqi banks actually run (file 17), through read-only connections only (file 25).

### 4.2 Options, as published on 2026-10-06

| Engine | Option | Facts | Verdict |
|---|---|---|---|
| PostgreSQL | `sqlx` | Pure Rust, rustls, 0.9.0 (2026-05-21), MIT or Apache-2.0 [V] | Use (AKR-TMS's own database, and CBS or channel databases on PostgreSQL) |
| MySQL, MariaDB | `sqlx` | Same; TLS with rustls | Use |
| SQL Server | `tiberius` 0.13.0 | Async TDS client; released 2026-09-25; tested on CI against SQL Server 2017, 2019 and 2022, "should work" on 2005 to 2016; TLS through native-tls (OpenSSL on Linux) or rustls; Kerberos integrated authentication through the `integrated-auth-gssapi` feature and NTLM through `sspi-rs`; pooling through bb8, mobc or deadpool [V, README] | Use. No Microsoft ODBC driver needed |
| Oracle | `oracle` 0.6.3 | Wrapper over ODPI-C; needs Oracle client libraries 11.2 or later **at run time** and a C compiler at build time; blocking API; UPL-1.0 or Apache-2.0; last release 2025-01-02, last push 2025-03-23, so maintenance is slow; 3.2 million downloads [V, GitHub and crates.io] | Default for Oracle polling, behind a trait, run on a bounded blocking pool |
| Oracle | `sibyl` 0.7.1 | Async OCI wrapper; MIT; 43 stars; releases through 2026-06 [V] | Watch only |
| Oracle, Db2, Sybase, others | `odbc-api` 29.2.0 | ODBC wrapper, MIT, released 2026-10-04; columnar bulk fetch; asynchronous only for one-shot queries (not prepared statements, connecting or metadata); needs unixODBC and the vendor's ODBC driver [V, README] | Default for Db2 (LUW and for i) and Sybase; second backend for Oracle |
| Oracle, Db2 (log-based CDC) | JVM sidecar: Debezium Server | Apache-2.0, 3.7 released 2026-09-29; Java 21 or later for the server [V] | Only where the bank agrees to log-based CDC (4.4) |

SQL Server is the one engine where the DBS stack has a clean pure-Rust path (`tiberius`) and SQLx has deliberately left; Oracle is the engine that most Iraqi banks run (ICS BANKS, file 17), and its Rust option is a thin wrapper over a native client.

### 4.3 Design

- **One port, several adapters.** `SourceReader` in `akr-ingest` with `capabilities()` (replica, snapshot isolation, keyset dialect, version), `probe()`, `read_batch(spec)` and `reconcile(spec)`. Adapters live in feature-gated crates (`drv-pg`, `drv-mssql`, `drv-oracle`, `drv-odbc`).
- **Dynamic SQL, not compile-time SQL.** `sqlx::query!` verification needs the real schema at build time; the CBS schema belongs to the bank. CBS reads are generated from the mapping (file 25 section 14), parsed with `sqlparser` (0.63.0, Apache-2.0; its source tree has dialect modules for Oracle, SQL Server, PostgreSQL, MySQL and others but none for Db2, where the generic or ANSI dialect must be used [V, repository]) to prove a single read-only `SELECT` over approved objects, and tested against fixtures. This is a narrow, written exception to the DBS rule; everything touching AKR-TMS's own database stays compile-time checked.
- **Dialect plugin per engine and version.** Row limiting differs: Oracle 12c and later `FETCH FIRST n ROWS ONLY`, Oracle 11g needs a `ROWNUM` wrapper (old estates, file 25 section 3.1); SQL Server `OFFSET ... FETCH` or `TOP`; Db2 `FETCH FIRST`; PostgreSQL `LIMIT`. Keyset predicates are generated, never `OFFSET`.
- **Blocking drivers on a bounded pool.** `oracle` and `odbc-api` calls run in `tokio::task::spawn_blocking` on a pool sized to the signed load budget (maximum sessions per bank), so a slow CBS cannot starve the runtime (the DBS rule: `spawn_blocking` for blocking and CPU work).
- **Vendor client libraries are the bank's, mounted, not redistributed.** Oracle client and IBM CLI or ODBC drivers are placed by the bank's DBA on a read-only volume (`/opt/drivers`); ODPI-C and unixODBC load them at run time. One `akr-ingestd` image therefore serves every bank, and AKR-TMS never redistributes Oracle or IBM binaries (the redistribution terms of Oracle Instant Client and the IBM driver were not verifiable in this research [K]; avoiding redistribution makes the question moot).
- **TLS and trust.** Pure-Rust drivers use rustls against the deployment's trust bundle (`SSL_CERT_FILE`, the DBS pattern); native clients use their own wallet or trust store, documented in the per-engine runbook (file 25 section 10).
- **A second Oracle backend on purpose.** Because the `oracle` crate has a slow release cadence, keep an `odbc-api` implementation of the Oracle adapter passing the same contract tests; switching is a configuration flag.

### 4.4 When a non-Rust CDC tool is justified

| Case | Decision | Reason |
|---|---|---|
| Oracle with an agreed log-based tier (R3) | Debezium Server sidecar | No maintained Rust crate reads Oracle redo or LogMiner; re-implementing transaction buffering, schema evolution and LOB handling is years of edge cases |
| Db2 LUW with IIDR licence | Debezium Server sidecar | Same, plus ASN control tables |
| SQL Server with CDC enabled by the bank | Rust reader of `cdc.*` change tables through `tiberius` | They are ordinary tables behind `fn_cdc_get_all_changes_*` |
| PostgreSQL logical decoding | Rust (Supabase ETL, Apache-2.0, "under active development", or a small protocol client) | The protocol is simple |
| Everything else | Polling | No JVM |

Deployment shape: Debezium Server is "a ready-to-use application that streams change events from a source database directly to messaging and storage systems without requiring Apache Kafka" and supports HTTP, NATS JetStream and Redis Stream sinks among others [V]. The simplest wiring for AKR-TMS is the **HTTP sink** into an mTLS endpoint of `akr-ingestd`, which lands events in the raw table with the commit position as a dedupe key (at-least-once delivery, idempotent landing). NATS JetStream is used only if the bank wants a durable buffer between the CBS-facing zone and the application zone (section 5). The JVM is a contained exception: one pinned image, in the bill of materials, upgraded with Debezium releases, never in the analyst or assessment processes.

### 4.5 AKR-TMS hooks

- **Driver conformance suite** run in CI against PostgreSQL, SQL Server and Oracle Free containers (Db2 and Sybase through a documented manual rig) to prove keyset reads, read-only enforcement, error taxonomy and character-set round trips for every adapter.
- **SQL safety gate** (`sqlparser` plus the catalogue-based read-only attestation) as a library used by every adapter and by the mapping workbench.
- **Driver volume contract** document: paths, expected library names, versions tested, and the health check that `akr-ingestd` runs at start and reports in the ops cockpit.

---

## 5. Ingestion workers and event streaming

### 5.1 What the DBS does

Durability lives in PostgreSQL: a `jobs` table polled with `FOR UPDATE SKIP LOCKED`, a transactional outbox, timers; per-kind concurrency semaphores; handlers idempotent; `dead` jobs surfaced in the ops cockpit. Redis carries cache, rate limits and SSE pub/sub only, "never a durable queue". Kafka is avoided "unless real replay or high-throughput streaming need emerges", NATS named as the step up (`tech-stack.md`, `jobs.md`). The DBS builds no broker today.

### 5.2 What AKR-TMS needs

| Flow | Mechanism | Delivery | Idempotency |
|---|---|---|---|
| CBS poll to raw landing | `akr-ingestd` loop per source; one transaction per batch writes rows, advances the watermark and enqueues follow-up jobs | At-least-once; replay-safe | Upsert on `(source, table, pk)` with version and hash guards (file 25 section 7.7) |
| CDC to raw landing | Debezium Server, HTTP sink (or NATS JetStream) to `akr-ingestd` | At-least-once | Commit position as dedupe key |
| R4 file or landing-table intake | `akr-ingestd` reads manifests and checksums from a bank-owned drop (shared volume, SFTP inside the bank, or a landing database) | At-least-once | Manifest id and row hashes |
| Raw to canonical mapping | `jobs` kind `map_batch`, concurrency per source | Exactly-once effect | Mapping version pinned per batch; replay tool |
| Canonical to scenarios and scoring | `jobs` kind `evaluate_accounts` fed by an outbox row listing the accounts a batch touched | At-least-once | Evaluation keyed on `(account, window, scenario_version, batch)` |
| Alert to case, notification | Outbox row in the same transaction as the alert | Exactly-once effect | Outbox id |
| Live updates to browsers | Valkey pub/sub to SSE; clients refetch on reconnect | Best effort by design | n/a |
| Phase-2 assessment | Synchronous; no queue; decision ledger row before the response | Idempotent by key | Byte-exact replay |

New job kinds follow the DBS rule (a phase-file entry, cockpit visibility, a concurrency decision): `poll_source`, `backfill_slice`, `map_batch`, `reconcile`, `coverage_certificate`, `evaluate_accounts`, `score_alerts`, `refresh_lists`, `rescreen_customers`, `generate_report`, `model_drift_check`, alongside the existing `anchor_audit`, `partition_maintenance` and `backup_check`.

### 5.3 Do the volumes justify a broker?

No, on the evidence available. A bank posting one million ledger lines a day averages about 12 per second; a tenfold peak is about 120 per second; even a salary-day burst of 1,000 per second is a small fraction of what a single PostgreSQL node absorbs with batched inserts [K; to be confirmed by a load test on the target hardware]. The CBI's 2013 paper reports about 3,000 RTGS transfers a month for the whole country (file 25 section 15). A broker would add a component, a failure mode and an operations burden in every bank for no throughput benefit. The raw landing table **is** the replay log: reprocessing is a re-read of raw rows, not a rewind of a topic.

A broker is justified in three narrow cases:

1. **CDC buffering across network zones**: NATS JetStream (server 2.15.0, Apache-2.0, released 2026-09-17; client `async-nats` 0.50.0 [V]) between a CBS-facing Debezium container and the application zone, when the bank's network design demands it.
2. **A bank that already runs Kafka** and offers topics as an R4 source: a consumer-only adapter behind a feature (`rdkafka` 0.39.0, MIT, librdkafka underneath [V]); AKR-TMS never produces to the bank's Kafka.
3. **Fan-out of canonical events to other bank systems** (SIEM, data warehouse): do it through the outbox to syslog or CEF export or an HTTP webhook rather than by exposing a broker.

Streams in Valkey are not used: the DBS rule is that Redis (Valkey in AKR-TMS) never holds work that must survive a restart.

### 5.4 Throughput design

- **Batch writes.** For backfills use `COPY` into a staging table then a set-based merge; for steady polling use multi-row `INSERT ... ON CONFLICT`. Staging tables are ordinary logged tables (PITR and replicas need the WAL).
- **Per-kind concurrency** through the DBS `KindPermits` pattern; `backfill_slice` runs with a small limit and obeys the load budget (file 25 section 5).
- **Back-pressure.** `evaluate_accounts` jobs are coalesced per account within a short window so a burst of postings produces one evaluation per account, not one per row.
- **Quarantine, not crash.** A batch that fails mapping validation is marked `rejected` with the reason and a retry button; the pipeline continues with other tables (bulkhead, file 25 section 14.4).

### 5.5 AKR-TMS hooks

- **Job-kind registry** extended and documented, with cockpit tiles for lag per source, batch rate, rejected batches and dead jobs.
- **Landing contract** for CDC events and R4 intake with a versioned envelope schema (`source_id`, `table`, `op`, `key`, `before`, `after`, `commit_ts`, `position`, `batch_id`, `checksum`).
- **A load-test rig** (the DBS uses k6 or oha for HTTP; add a synthetic CBS writer) that proves the 10x and 100x salary-day cases per hardware profile.

---

## 6. Storage design on PostgreSQL

### 6.1 Layout and roles

Schemas: `raw` (verbatim landing), `canon` (the canonical model, file 25 section 13), `feat` (features and aggregates), `scn` (scenario definitions, versions, runs), `alert`, `case`, `kyc`, `ref` (reference data), `audit`, `admin`. Database roles in the DBS style (grants listed, never `ALTER DEFAULT PRIVILEGES`; the DBS reporting role `dbs_readonly` holds `SELECT` and nothing else, with `statement_timeout`, `idle_in_transaction_session_timeout` and `default_transaction_read_only` as defence in depth [V, migration 0019]):

| Role | Reach |
|---|---|
| `akr_app` | Read and write on application schemas; no DDL |
| `akr_ingest` | Write `raw` and `canon` only; no access to `case` or `alert` text |
| `akr_assess` | Read lists, features and fail policies; write decisions only |
| `akr_readonly` | `SELECT` on a spelled-out list for the audit viewer and reports, on the standby when one exists |
| `akr_migrate` | DDL during releases only |

### 6.2 Partitioning and indexes

Range partitioning by booking month for `canon.txn`, `canon.txn_event` and `canon.payment_message`; by extraction day for `raw` tables; monthly for `audit_log` (as the DBS). BRIN indexes on time columns, B-tree on `(account_id, booking_date)` and on natural keys, GIN only where a search needs it. Partition creation and retention run as worker jobs, as the DBS `partition_maintenance` job does, rather than through `pg_partman` (5.5.0, PostgreSQL licence [V]): one fewer extension to certify at every bank.

### 6.3 Size model (planning assumptions, to be replaced by discovery data)

Rows per bank = postings per day times days kept. A row in `canon.txn` with its indexes is of the order of 300 to 600 bytes [K]. One million postings a day for five years (1.8 billion rows) is of the order of 0.5 to 1 TB including indexes and the raw layer; 100,000 a day is a tenth of that. NVMe storage and monthly partitions keep this unremarkable for PostgreSQL; the DBS growth estimate for its audit trail (about a billion rows over ten years) is the same order of magnitude and is described there as "unremarkable on NVMe". `wal_compression=zstd` and pgBackRest as in the DBS compose files.

### 6.4 Retention classes

Two mechanisms, chosen by the shape of the clock (the full class table is in file 30 section 12.3). High-volume time-series (raw landing, `canon.txn`, events, payment messages, features, alerts, screening and phase-2 decision records) are **partition-drop policies** keyed to the booking, extraction or decision month, with a legal-hold check that copies held rows into a hold store before a drop and blocks the drop if the copy fails. Low-volume customer-linked tables (party, account, relationship, KYC profile, rating history) carry **relationship-end clocks** (five years after the last account closes, Law 39 Article 11) and are purged record by record, because a partition cannot express a per-customer clock. Defaults: monitoring copy of transactions 10 years (floor 5), sanctions-related records and cases 10 years (OFAC rule of March 2025; file 05 section 1e), STR records at least five years from the report date, audit trail unlimited as the DBS decision D5. Opening a case freezes an evidence snapshot, so dropping old transaction partitions never damages a case.

### 6.5 When a columnar or analytical store is justified

Not at launch. Consider one when scenario windows run over more than about three to five billion ledger rows, or analysts need sub-second aggregates over more than twelve months with many concurrent users [K]. The first answer is a read-only standby for reports (as the DBS plans). The second is **ClickHouse** (Apache-2.0, 26.9 [V]) as a derived, read-only copy fed from the outbox, never as a system of record. For backtests on exports, **DataFusion** 55.1.0 (Apache-2.0) or **DuckDB** (`duckdb` crate 1.10506.0, MIT) over Parquet are embeddable without a server [V, crates.io]. TimescaleDB has a split licence (Apache core, Timescale-licensed features) and adds an extension to certify; not recommended.

### 6.6 Extensions and the Arabic collation decision

- **PGroonga** (4.0.9, PostgreSQL-style licence; Groonga itself is LGPL-2.1 [V]) stays for text search over case notes and documents, with the DBS Arabic fold function `dbs_search_normalise` (migration 0017). It is **not** the name-matching engine (section 10).
- **pg_trgm** and **unaccent** are contrib modules available in the base image [V, DBS DR-97]. **pgvector** is optional for case-similarity embeddings.
- **Collation.** The DBS images are Alpine (musl). In a throwaway PostgreSQL 17.11 Alpine container on 2026-10-06 the ICU provider offered 29 `ar*` collations and trigram extraction worked on Arabic text under `C.UTF-8` [measured]. Decide at `initdb` whether names sort under an ICU Arabic collation; changing a database's collation later is expensive. Glibc-based images (Debian) behave differently and were not measured.
- **Version.** PostgreSQL 17 for launch (the DBS version); plan 18 (images exist; Apache AGE's 1.8.0 release candidate targets 18).

### 6.7 AKR-TMS hooks

- **Retention engine** (admin-managed classes and jurisdiction profiles with floors, legal holds and a hold store, partition-drop and record-purge jobs with a dry run, maker-checker, signed deletion certificates and post-restore replay; file 30 section 12).
- **Size estimator** in the onboarding kit that prints expected raw, canonical and index sizes from the discovery numbers.
- **Collation and character-set check** in the installer (database encoding UTF8, ICU locale chosen, test strings round-trip).

---

## 7. The rules and scenario engine

### 7.1 What it has to do

Run the CBI's catalogue of 86 or more scenarios (file 27) and each bank's own, with parameters a compliance officer can change without a deployment; evaluate on events (near-real-time) and over windows (days, months); segment by customer type; explain every alert; replay history to test a change before it goes live; keep every version and approval (BCBS, NYDFS Part 504 and the CBI all ask for documented, tested, change-controlled scenarios; files 04 and 05). Detection method choices (rules, segments, ML, graphs) are argued in file 21; this section maps them to the stack.

### 7.2 Options

| Option | Fit | Verdict |
|---|---|---|
| Hand-written Rust rules | Fast; but every threshold change is a code release, which breaks "nothing is a fixture" | Reject for scenarios; keep only for fixed product mechanics |
| Embedded scripting (Rhai 1.26.1, MIT or Apache-2.0 [V]) | Flexible; Turing-complete, so it needs sandbox limits and is harder to prove safe when authored by implementers | Not for user-authored logic |
| **CEL** (Common Expression Language) through the `cel` crate 0.15.0, MIT, released 2026-10-06 [V]; the older `cel-interpreter` 0.10.0 is from July 2025 | Non-Turing-complete, side-effect-free, built for safe evaluation of admin-authored conditions | **Use for event-level conditions and computed fields** |
| **SQL on PostgreSQL** (window functions, partition-pruned scans) | The data is already there and indexed; set-based; auditable | **Use for window aggregates and sweeps** |
| DataFusion 55.1.0 (Apache-2.0) or DuckDB over Parquet [V] | Embeddable columnar engines | Backtests on exports; later optional for heavy windows |
| Cedar | Authorisation language, not detection (no aggregation or window semantics) | Reject for detection; keep for access control |
| JVM rule engines (Drools and similar) | Mature | Reject: outside the stack |

### 7.3 Design

A **scenario is a versioned JSON document** validated against a JSON Schema, following the DBS `dsl` crate pattern (strict schema, loud failure, and `default-features = false` on `jsonschema` so no remote `$ref` can be fetched; `jsonschema` is at 0.58.6 [V]):

- **Identity and lineage**: id, name in Arabic and English, typology, the CBI catalogue reference (for example "cash scenario 6"), owner, status (`draft`, `testing`, `shadow`, `live`, `retired`).
- **Data requirements**: the canonical attributes the scenario needs (feeds the coverage matrix, file 25 section 13.6).
- **Parameters**: typed, with bounds, defaults and per-segment overrides; each change is a new version.
- **Logic**, of one of three kinds: **aggregate** (a closed set of operators: count, sum, avg, max, distinct-count, ratio, over a window, grouped by account, customer, counterparty or branch; compiled to parameterised SQL over the canonical partitions, with table and column names taken from the canonical catalogue, never from the document); **event** (a CEL expression over a typed context of transaction, account, customer profile and features); **profile** (expected versus actual, peer deviation, using feature tables computed by jobs).
- **Outputs**: severity mapping, reason codes with Arabic and English text, suppression and hibernation rules, the alert template.

Safety rules: no free SQL in any document; bound parameters only; a `statement_timeout` per run; a cost guard from `EXPLAIN`; evaluation on the standby when one exists.

### 7.4 Execution model

1. **Incremental**: after each ingest batch an outbox row names the accounts touched; event scenarios run at once on the fast path; window scenarios run for those accounts over indexed ranges; a settled-time re-evaluation follows (file 25 section 7.4).
2. **Scheduled sweeps**: daily and monthly scenarios over all accounts (cash structuring over 30 days, dormancy, expected-versus-actual), as set-based queries per scenario, not loops per account.
3. **Backtests**: run a draft version over a historical period with alternative parameters; store alert counts, overlap with real STR outcomes and the volume change; this is the evidence for tuning approval (file 29).
4. **Shadow mode**: a new version runs beside the live one, produces no alerts, and is compared.

### 7.5 Segmentation, explanation, tuning governance

- **Segments** are admin-managed data; baseline statistics (percentiles, medians) are computed by a scheduled job into `feat.segment_stats`; thresholds can be absolute or relative to the segment.
- **Explanation**: every alert stores the version, parameter values at run time, the data window, the matching conditions and the ids of the source transactions; an analyst can ask "why was this not an alert" and get a what-if run.
- **Tuning governance**: every parameter or logic change is a change request with maker-checker approval (section 11), a backtest attached, and a flag when its main effect is to cut alert volume (the NYDFS concern, file 05 section 1f).

### 7.6 Performance expectations

Window queries on a partitioned table with an `(account_id, booking_date)` index are cheap per account; a daily sweep of a million accounts is a set-based scan of recent partitions and should complete in minutes to a few hours depending on scenario count and hardware [K; to be benchmarked with the synthetic CBS rig of section 5.5]. The incremental path keeps near-real-time work proportional to the accounts touched, not to the bank's size.

### 7.7 AKR-TMS hooks

- `akr-dsl` crate with the scenario and mapping schemas (shared with file 25 section 14), the CEL sandbox (evaluation cost limit, no I/O) and the SQL compiler.
- **Catalogue importer** loading the CBI scenarios from file 27 as draft documents with their data requirements.
- **Backtest runner** as a job kind with a report: alert counts by segment, overlap with past STRs, estimated analyst hours.

---

## 8. ML inference in Rust and offline training

### 8.1 What is needed

Alert prioritisation and SAR-propensity scoring, an anomaly detector as a second signal, customer risk scoring and entity-resolution scoring: tree ensembles as the default scorer, TreeSHAP for explanations, hand-built graph features as inputs, a streaming motif engine in Rust (file 21 sections 4 to 7, 11). Governance (inventory, validation, drift, human oversight) is file 29; this section covers where models are trained, how they are served and what the Rust ecosystem offers.

### 8.2 Training: offline, inside the bank's perimeter

The DBS has no ML training or serving. AKR-TMS adds a **model workbench**: a Python environment (scikit-learn, LightGBM or XGBoost, SHAP for validation) that runs on a bank-owned workstation or server **inside the bank's network**, reads de-identified feature extracts from the canonical store, and writes model artifacts. Training data never leaves the bank (DBS invariants: external AI services are prohibited, file 30). The vendor contributes code, feature definitions, notebooks and validation templates, and may train on **synthetic data** (file 25 section 13.7) on its own premises; a bank's model is trained on that bank's data by that bank's staff or under supervision.

### 8.3 Serving in Rust

| Component | Facts (2026-10-06) | Use |
|---|---|---|
| `ort` (ONNX Runtime bindings) | 2.0.0-rc.13, released 2026-07-28, **still a release candidate**; Apache-2.0 or MIT; ships prebuilt ONNX Runtime by default; can link a custom build through `ORT_LIB_PATH` or load it at run time with the `load-dynamic` feature [V, crate and docs] | Default engine for tree ensembles and small networks, with a pinned ONNX Runtime build stored in Harbor for air-gapped builds |
| `tract-onnx` | 0.23.8, released 2026-09-21; pure Rust; ONNX and NNEF loader; support for the ONNX-ML tree-ensemble operators was not confirmed [K] | Fallback for small neural models where a C++ dependency is unwelcome |
| `candle` | `candle-core` 0.11.0, released 2026-06-26; MIT or Apache-2.0; Hugging Face's Rust ML framework [V, crates.io] | Transformer inference (for example name-embedding or text-classification models), only if a model needs it |
| `lightgbm3` 1.0.11 | Active; wraps the LightGBM C++ library [V] | Alternative when ONNX conversion of a LightGBM model is awkward |
| `smartcore` 0.6.15, `linfa` 0.8.1 | Pure-Rust classical ML; active [V] | Isolation forest, clustering, simple baselines inside Rust if needed |
| `xgboost` crate 0.1.4 (2019), `gbdt` 0.1.3 (2024) | Stale [V] | Avoid |
| TreeSHAP | `shap-rs` 0.1.0 (August 2026, about 700 downloads) and `treeshap` 0.1.0 (July 2026, 21 downloads) exist and are brand new [V] | Treat as unproven: implement TreeSHAP in `akr-scoring` from the published algorithm over exported tree structures, with golden tests against Python `shap` |

### 8.4 Model lifecycle on the stack

1. **Registry**: a `model` table (name, version, training data window and hash, feature list, metrics, validation report, approvals) with the artifact in the evidence store, SHA-256 verified on load and signed with an OpenBao transit key (the DBS QR-token scheme).
2. **Promotion** is a maker-checker change request with a validation report attached; a new model runs in **shadow** (scores logged, not used) before it influences ranking.
3. **Serving** in `akr-worker` (batch and incremental scoring) and in `akr-assess` (real-time, in-process, no network hop); the model file is loaded once per process, inference is a few milliseconds for tree ensembles [K].
4. **Monitoring**: a `model_drift_check` job computes population-stability and score distributions per segment and writes to the ops cockpit; alerts when drift exceeds the approved threshold (file 29).
5. **Explanations** per alert: top contributing features by TreeSHAP, shown with facts and peer comparison (file 21 section 11).

### 8.5 Hardware

Tree ensembles need no GPU. A GPU matters only for optional self-hosted language-model features (case summaries, adverse-media triage; file 21 section 12). The DBS has declined to procure GPU capacity for OCR twice (DR-110), and the same hesitation applies here, so language-model features are an optional later phase with on-premises serving (llama.cpp or vLLM class), never an external API.

### 8.6 AKR-TMS hooks

- **Workbench package** and runbook; feature definitions as code and documentation; validation report template (file 29).
- **Model registry, signing and shadow-mode machinery** in `akr-scoring`; ONNX Runtime build pinned and vendored.
- **TreeSHAP implementation** with golden tests; explanation payload schema shared with the alert UI.

---

## 9. Graph analytics

### 9.1 What is needed

Entity links (shared phone, address, device, identifier, signatory, beneficial owner), counterparty flow graphs, cycles, fan-in and fan-out, connected components, neighbourhood risk, and ego-graph views for investigators (file 21 section 6; file 22 for the name-resolution side).

### 9.2 Options

| Option | Verdict |
|---|---|
| Dedicated graph database (Neo4j and similar) | Not in the stack; heavy to certify at every bank; reject |
| **Apache AGE** (Cypher on PostgreSQL), Apache-2.0; its 1.8.0 release candidate for PostgreSQL 18 was published 2026-07-09 [V] | Tracks PostgreSQL major versions closely and adds an extension to certify; optional, later |
| **Edge tables plus recursive CTEs** (bounded depth of 3 or fewer) | **Use** for ego graphs and neighbour lookups; indexed on both endpoints |
| **`petgraph` 0.8.3** (MIT or Apache-2.0; released 2025-09-30 [V]) in the worker | **Use** for components, strongly connected components, cycles and centrality over per-customer or per-component subgraphs loaded from edge tables |
| Streaming motif engine in Rust (in memory, windowed) | The approach file 21 recommends for fan-in, fan-out, cycles and scatter-gather features |

### 9.3 Design

- `canon.party_link` (party to party) and `canon.account_flow` (account to account over a window) as edge tables with a `kind`, weight, first and last seen, and the evidence ids; shared-contact links are derived by jobs from normalised phones, e-mails, addresses and device ids (file 25 section 13).
- Graph features (degree, component size, neighbour risk, cycle counts) are computed in `akr-graph`, written to `feat`, and used by scenarios and by the scorer, so no query-time graph engine is needed at analyst latency.
- **Visualisation** in the browser (section 18): `@xyflow/react` 12.12.0 (already in the DBS for the workflow builder) for small ego graphs; Sigma 3.0.3 with graphology 0.26.0 or Cytoscape 3.34.3 for large neighbourhoods [V, npm].

### 9.4 AKR-TMS hooks

- Edge-table schema and the link-derivation jobs; a component-id column so "everything connected to this customer" is one indexed lookup.
- A size guard: ego graphs are capped (nodes and depth) with a visible "more exists" marker.

---

## 10. Name-matching and screening index

Names are the screening problem in Iraq: long patronymic chains, tribal components, Arabic and Kurdish scripts, Latin transliterations that differ by clerk (file 22 sections 2 to 8). The DBS already has Arabic normalisation (`dbs_search_normalise` in SQL and a twin function in Rust, pinned by a parity test) and PGroonga for text search. What it lacks is a name-matching engine.

### 10.1 What was measured (throwaway PostgreSQL 17.11 on Alpine, `C.UTF-8`, synthetic data, 2026-10-06)

| Test | Result |
|---|---|
| Alef variants `أحمد` versus `احمد`, raw | trigram similarity **0.25**, below the default threshold of 0.3, so a plain `%` search **misses it** |
| Same pair after the DBS fold function | **1.00** |
| Ta marbuta and ha `فاطمة` versus `فاطمه`, raw, then folded | 0.50, then 1.00 |
| Four-part name with a one-letter difference (`حسين` versus `حسن`) | 0.79 |
| Same name parts reordered (`علي محمد حسين الجبوري` against `محمد علي حسين الجبوري`) | **1.00** (trigram sets ignore word order, which suits Iraqi name-order variation) |
| Three-part name against four-part name | 0.77; `word_similarity` of the short name inside the long one 0.71 |
| Latin variants `Mohammed Ali Hussein Al-Jubouri` against `Muhammad Ali Husain Jabouri` | 0.38: near the threshold; transliteration matching needs more than trigrams |
| Arabic against Latin `محمد علي` against `Mohammed Ali` | **0.00**: cross-script needs transliteration or phonetic keys |
| GIN trigram index, **100,000** synthetic names (list-sized) | `%` query in **2 to 8 ms** |
| GIN trigram index, **2,000,000** rows with 1.42 million distinct names (customer-sized) | index 85 MB for a 146 MB table, built in 10.9 s; a `%` query at threshold 0.6 for a name made of common words took **2.7 s** (699,000 candidates, 2,945 true matches); at threshold 0.8, 110 ms |
| Collations | 29 ICU `ar*` collations available on that image |

Caveats: synthetic data (a random-token tail on top of common Iraqi names), one run, a default container configuration, no concurrency, musl rather than glibc. The numbers show behaviour, not capacity.

### 10.2 Reading the results

- **pg_trgm works on Arabic**, but only after the same fold on both sides. Without it, common orthographic variants fall under the threshold (0.25 against 0.30).
- **It is fast on list-sized indexes and unpredictable on customer-sized ones.** With common-name tokens, a low threshold returns hundreds of thousands of candidates for the recheck. That is fine for batch re-screening and for back-office search; it cannot carry the 15 to 40 ms candidate-generation budget of the phase-2 API (file 25 section 15.4) over millions of customers.
- **The synchronous path searches the list, not the customers.** For a payment, the engine looks up the parties in the message against a few tens of thousands of list entries and aliases (UN, OFAC, EU, UK, the Iraqi national list of about 6,000 entries; files 04 and 05). That index fits in memory.

### 10.3 Options

| Option | Facts | Verdict |
|---|---|---|
| `pg_trgm` GIN | Contrib module; thresholds 0.3 and 0.6 by default [V, manual]; measured above | Batch re-screening, back-office name search, candidate generation on the list side where latency is not critical |
| PGroonga bigram (DBS) | Used for documents; index lives outside PostgreSQL relations and reports zero bytes to `pg_relation_size` (DBS DR-97) | Notes and documents; **not** names |
| `tantivy` 0.26.2 (MIT, Quickwit, active [V]) | Embeddable full-text engine in Rust; custom tokenizers and filters; memory-mapped segments | Candidate generation in the phase-2 path and for large re-screens, with Arabic-normalised tokens, n-grams and phonetic keys; or a purpose-built in-memory inverted index |
| Custom scorer in Rust | `strsim` 0.11.1 (MIT, last release 2024) and `rapidfuzz` 0.5.0 (December 2023) are stale [V]; Jaro-Winkler, Levenshtein and token-set ratios are short to implement and test | Own scorer with name-part alignment, token weights by frequency (file 22 section 8) and explainable features |
| `unicode-normalization` 0.1.25 (MIT or Apache-2.0 [V]) | NFKC and NFD | In the shared normalisation library (file 22 section 6.4) |

### 10.4 Design

Two stages, one normalisation:
1. **Normalise** with one Rust library used by ingestion, batch screening and the assessment API (file 22), with the SQL twin kept in parity by a test (the DBS pattern).
2. **Generate candidates** from an in-memory index of list entries (normalised tokens, bigrams, phonetic and transliteration keys); in batch, optionally from pg_trgm on a standby.
3. **Score** with the Rust scorer, return the matched fields, aligned name parts and the reason for each score; apply secondary identifiers (date of birth, mother's name, ID numbers) to cut false positives (file 22 section 9).
4. **Persist** the list entries, aliases, results, decisions and whitelists in PostgreSQL; rebuild the in-memory index on list change or restart (a few seconds for tens of thousands of entries [K]).

### 10.5 AKR-TMS hooks

- **Normalisation crate** with a golden test file and the SQL parity test.
- **List index service** built from PostgreSQL at start and on change, with a staleness clock that the assessment API enforces (file 25 section 15.5).
- **Benchmark harness** that regenerates the table in 10.1 on the bank's hardware and on glibc and musl images, and records precision and recall on the synthetic Iraqi corpus of file 22 section 12.

---

## 11. Case management UI, workflow reuse and maker-checker

### 11.1 What the DBS already provides

The DBS workflow engine is an event-sourced, DSL-driven system: versioned JSON definitions (steps `form`, `approve`, `sign`, `reply`, `attach`, `acknowledge`, `parallel`, `terminal`), instances pinned to the version they started on, an append-only `instance_events` stream with a `tasks` projection (the hot inbox table), timers with escalation, delegation with windows, break-glass overrides, optimistic version checks, an SSE bridge, and documents with antivirus, rendition and stamping. The frontend has the inbox, task screens, the audit timeline, the ops cockpit, the sensitive-text guard (`SensitiveDialog`: warns on a card number or a secret typed into free text, stores it masked) and a dynamic form renderer.

### 11.2 Mapping to AML concepts

| DBS concept | AKR-TMS use |
|---|---|
| Request (instance) | Alert, case, KYC review, STR package, rule-change request |
| Task and inbox | Analyst work queue, MLRO queue, branch liaison "window" |
| Step types (`approve`, `sign` with step-up) | Four-eyes decisions: close without STR, file STR, clear a hold, approve a rule change |
| Timers and escalation | Case SLAs, FIU and CBI deadlines, RFI response clocks (file 04 section 2p) |
| Delegation with windows | Cover during leave (the CBI's mandatory leave rule for staff, file 01) |
| Break-glass | Admin reassignment of a stuck case, always audited |
| Classification labels | Case confidentiality, with an STR tier visible to the MLRO group only (file 30 section 8) |
| QR and stamped PDF | Stamped STR package and evidence-pack verification |
| Sensitive-text guard | Keep full card numbers and secrets out of case notes (PCI scope, file 30) |

### 11.3 Recommendation

Do **not** model investigations as generic requests only. Use two layers:
- **A dedicated `case` aggregate** for investigation (alerts linked in, notes, evidence, entities, decisions, outcome), event-sourced with the same pattern and projections as the DBS, because investigations are long-lived, branch and merge, and need analyst-oriented screens.
- **The DBS workflow engine and DSL for approval flows** that wrap decisions (STR approval, closure approval, rule-change approval, KYC review sign-off), because approvals are exactly what that engine does well and are already tested, including step-up and delegation.

### 11.4 Maker-checker as a generic change request

One mechanism for every admin-managed object that governs detection or reporting: scenarios, parameters, thresholds, code maps, mappings, fail policies, list settings, roles. A `change_request` row (object type, before and after, reason, impact summary, backtest id) with: the maker cannot approve (enforced in Cedar and by a database constraint), approval needs a fresh second factor (the DBS single-use step-up ledger), and activation writes the new version and the audit entry in one transaction (the DBS policy-bundle activation pattern). A "volume-reducing change" flag requires a second approver in the compliance function.

### 11.5 Notifications

In-app (SSE) and e-mail through the bank's own SMTP relay. **No WhatsApp, no external push** for AML: the DBS WhatsApp channel (ARCHITECTURE D9) contradicts the tipping-off controls, because even a generic "you have a task" message tells a staff member's personal phone that compliance is looking at something. Notification text never names a customer or a case subject (file 30 section 8).

### 11.6 UI components needed beyond the DBS

A dense alert and case list (TanStack Table with virtualisation), a customer 360 page, a transaction timeline, a side-by-side name-match comparison with aligned name parts in RTL and LTR, a network view (section 9), a scenario and mapping workbench, the reconciliation and coverage dashboards, and report builders. File 31 (analyst UX) owns the screen design.

### 11.7 AKR-TMS hooks

- `case` bounded context and projections modelled on `instance_events` and `tasks`.
- Approval definitions shipped as seed DSL documents, editable on screens.
- `change_request` crate used by every admin screen.

---

## 12. Evidence vault on object storage (MinIO kept by decision, and how to run it safely)

Updated 2026-10-07 after the owner's decision to keep MinIO. The first version of this section (2026-10-06) recommended replacing MinIO with SeaweedFS; that recommendation is withdrawn. Working notes with every quote and download: `research/_minio-notes.md`.

### 12.1 What the vault holds

STR packages and their supporting documents, case evidence and screenshots, alert snapshots, signed coverage certificates and reconciliation reports, model artifacts and validation reports, mapping and scenario version exports, audit anchors, and customer KYC document copies where the bank chooses to hold them. Needed: write-once retention (five to ten years, file 30 section 12), legal hold, versioning, hash on serve, encryption at rest, access through short-lived presigned URLs after a Cedar decision and an audit row, antivirus on upload, and backups that preserve lock metadata.

### 12.2 What the DBS built on MinIO

`S3ObjectStore` over the `rust-s3` crate (0.38.0 today), buckets for documents, quarantine, finals and audit anchors; `dbs-finals` and `dbs-audit-anchors` are created with object lock and a default **COMPLIANCE** retention of 3650 days (`mc retention set --default COMPLIANCE 3650d`), because "object lock can only be enabled at creation time (`--with-lock`)" and COMPLIANCE is chosen over GOVERNANCE since "GOVERNANCE mode can be bypassed by a user holding the bypass permission, which defeats the purpose"; the finals bucket is the WORM copy that "cannot be altered or deleted for the retention period, by anybody, including an administrator holding root credentials" (comments in `docker-compose.yml`; `common/src/objectstore.rs` says the same of the anchors bucket: protected "by the storage layer itself"). Backups use restic over MinIO's storage so "content types, versions and object-lock retention" come back as they were (`backup-restore.md`). The images are pinned to MinIO `RELEASE.2025-04-22` and `mc` `RELEASE.2025-04-16`.

Two facts found on 2026-10-07 concern the DBS itself (reported to the owner; the DBS is read only for this research):

- Both images it pulls, `minio/minio:RELEASE.2025-04-22T22-12-26Z` and `minio/mc:RELEASE.2025-04-16T18-13-26Z` (`docker-compose.yml` lines 312 and 336), can no longer be pulled from Docker Hub: the Hub API answers "object not found" for both repositories and an anonymous registry token is refused ("authentication required"), while the same calls succeed for other images [V]. A fresh installation that pulls from Docker Hub fails unless the images are mirrored.
- The pinned server release predates the fix for CVE-2025-62506 and is exposed to the seven 2026 advisories in 12.3 [V].

### 12.3 The state of MinIO in October 2026

- **The upstream repository is archived.** `minio/minio` was archived after its last push on 2026-04-24 [V]. Its README opens "THIS REPOSITORY IS NO LONGER MAINTAINED", names "AIStor Free" and "AIStor Enterprise" as alternatives, says "The MinIO community edition is now distributed as source code only. We will no longer provide pre-compiled binary releases", and warns that "Production environments using compiled-from-source MinIO binaries do so at their own risk" and that "All usage of MinIO in your application stack requires validation against AGPLv3 obligations" [V]. The last release, `RELEASE.2025-10-15T17-29-55Z` (a security release for CVE-2025-62506), carries no binaries; the releases before it carried 48 [V]. The twelve commits after it are three small fixes plus documentation and README changes: "maintenance mode" (2025-12-03), "latest free license and enterprise option" (2026-01-06) and "clarify state of the project" (2026-02-12) [V]. The 2025-05-24 release had already taken the admin console out of the server ("Embedded UI Console is now deprecated"), including console logins through LDAP or OIDC, which "are now available as part of the AiStor Product"; the STS APIs stayed [V].
- **The companion repositories are archived too**: `minio/mc`, the client the DBS provisions buckets with (last release `RELEASE.2025-08-13`); `minio/kes`, the key service behind server-side encryption with a KMS (archived, last push 2025-05-27); and `minio/operator` for Kubernetes (last push 2026-03-20). The Go SDK `minio-go` (Apache-2.0) is still maintained [V, GitHub API].
- **Seven advisories were published after the last release, and the fixes exist upstream only in MinIO's proprietary AIStor.** Each advisory says the flaw affects "all MinIO releases through the final release of the minio/minio open-source project" [V, GitHub security advisories API]:

| Published | Advisory | Severity (CVSS 4.0) | Flaw | Precondition |
|---|---|---|---|---|
| 2026-03-19 | CVE-2026-33322 | Medium (9.2) | JWT algorithm confusion in OIDC: forged identity tokens with any policy, including `consoleAdmin` | OIDC configured and its client secret known |
| 2026-03-20 | CVE-2026-33419 | Medium (9.1) | LDAP STS user enumeration and no rate limit, so passwords can be brute-forced | LDAP configured |
| 2026-03-27 | CVE-2026-34204 | High (7.1) | `X-Minio-Replication-*` headers on an ordinary PUT inject encryption metadata; objects become "permanently unreadable" | Any key with `s3:PutObject` |
| 2026-04-07 | CVE-2026-39414 | High (7.1) | S3 Select CSV parser buffers a line without limit; a 2 MB compressed upload can crash the server | `s3:PutObject` and `s3:GetObject` |
| 2026-04-11 | CVE-2026-40344 | High (8.8) | Snowball auto-extract skips signature verification on the unsigned-trailer path: objects written "without knowing the secret key" | A known access key with write permission |
| 2026-04-14 | CVE-2026-41145 | High (8.8) | Same path with credentials in the query string: the signature is never checked on PUT and multipart uploads | Same |
| 2026-04-25 | CVE-2026-42600 | Medium (6.9) | Path traversal in the internode `ReadMultiple` endpoint reads files outside the drives | Multi-node only; a token signed with the root password |

- **Two of them matter most for an evidence store.** CVE-2026-40344 and CVE-2026-41145 need only a known access key, and an access key is not a secret in S3: it appears in every presigned URL. A COMPLIANCE lock still stops a forged request from overwriting or deleting a locked version, but it does not stop new objects or versions being planted, which breaks the guarantee that the vault holds only what AKR-TMS wrote.
- **Upstream disclosure has ended.** An archived repository cannot publish new advisories. Findings in the inherited code now appear only in the forks' own ledgers, and Silo's fork-local `SN-` identifiers are deliberately not written as CVEs ("there is no upstream maintainer to coordinate a CVE assignment with") [V]. A scanner that reads only CVE databases will not see them.
- **AIStor Free cannot be part of a product.** Its licence grants use "solely in standalone mode (single-node deployments without distributed clustering or high availability)", forbids modification and says "You may not distribute, sublicense, rent, lease, resell, or redistribute the Software (in whole or in part) to any third party"; the software "is proprietary to MinIO", the terms may be "periodically updated" with continued use counting as acceptance, and California law governs (agreement effective 2025-12-22, web version updated 2026-01-30) [V]. AIStor Enterprise is a commercial subscription. Both are excluded: AKR-TMS promises banks no foreign vendor licence.
- **A patched build may not carry the MinIO name.** MinIO's trademark policy (2025-08-29) lets anyone modify the code under the open-source licence "but [you] may not redistribute your modifications under any MinIO Trademarks"; such a product should be described as "based on MinIO technology" or "incorporating MinIO source code", and "You must also change the name of the products and binaries" [V]. Unchanged official binaries may be redistributed, but the newest official binaries (`RELEASE.2025-09-07T16-13-09Z`) are exposed to all eight advisories above and below.

### 12.4 What the decision keeps

Keeping MinIO keeps the MinIO code (AGPL-3.0), its S3 and admin APIs, its object-lock implementation, its on-disk format and the DBS's operating knowledge. COMPLIANCE mode protects a version "from write operations by *all* users, including the MinIO root user", the lock "cannot [be changed or removed] before the configured retention duration has passed" and it is "functionally identical to the S3 COMPLIANCE mode"; legal holds behave the same way and are set or lifted only with `s3:PutObjectLegalHold` [V, MinIO object-locking documentation as carried by Silo]. Cohasset Associates assessed MinIO object lock in COMPLIANCE mode against SEC 17a-4(f), FINRA 4511(c) and CFTC 1.31(c)-(d), but for "MinIO Object Storage, Release 172 ... running on MinIO qualified hardware" [V]: the design is sound, but no 2026 build is certified, so each AKR-TMS build is proved by its own tests (12.6, item 9).

The decision cannot keep the upstream maintainer, upstream binaries or images, or the MinIO name on a patched build. It therefore means running a **maintained MinIO code line** under another product name.

### 12.5 Maintained MinIO code lines (checked 2026-10-07)

| Code line | Licence and maintainer | Activity | The seven 2026 advisories | Assessment |
|---|---|---|---|---|
| Upstream `minio/minio` at `RELEASE.2025-10-15` | AGPL-3.0; MinIO, Inc.; archived | None since April 2026 | 0 fixed | Not acceptable as it is |
| **Silo** (`pgsty/silo`, delivered as `pgsty/minio` until August 2026) | AGPL-3.0-or-later; the Pigsty project (GitHub organisation location: Singapore, self-declared); no CLA, contributions under DCO | 11 releases since December 2025; latest `RELEASE.2026-09-16T00-00-00Z`; Go 1.27.1; releases "typically every one to two months, and at most a quarter apart"; 3,897 stars | **All seven fixed** (releases 2026-04-17 and 2026-06-18), plus its own advisories SN-2026-002 to SN-2026-014 and inherited defects | **Primary baseline** (12.6) |
| **Chainguard fork** (`chainguard-forks/minio`, Chainguard's EmeritOSS programme) | AGPL-3.0; Chainguard (United States); "Community contributions are not being accepted" | Tags up to `RELEASE.2026-09-22T19-25-18Z`; minimal-change policy | **All seven fixed** (2026-04-10 to 2026-06-04), plus the SigV4 unsigned-header fix (2026-09-22) | **Second source and cross-check**; the source is public, the images are a commercial offer |
| libreFS | AGPL-3.0; community | Last push 2026-05-04; 44 stars | Not checked | Forked at `RELEASE.2025-04-22`, before the CVE-2025-62506 fix; too small to depend on |
| Kypello | AGPL-3.0; community | Last push 2026-07-17; 4 stars | Not checked | Too small to depend on |
| `juicedata/minio` | Apache-2.0 (the pre-2021 code line) | Maintained for JuiceFS's gateway | n/a | Five years of upstream fixes missing |
| AIStor Free, AIStor Enterprise | Proprietary; MinIO, Inc. | Active | Fixed in AIStor | Excluded (12.3) |

What Silo publishes about itself, all verified on its site and releases [V]:

- **Compatibility.** "The S3 API, `MINIO_*` environment variables, `minio_*` metrics, `x-minio-*` headers, `/minio/*` routes, and the on-disk format (including `.minio.sys`) are preserved, and held in place by a CI compatibility check"; every release names its tested rollback target; its migration guide "includes how to leave Silo". A mixed MinIO and Silo cluster does not form, so a multi-node installation switches all nodes together.
- **No phone-home.** Its never-list includes "ship telemetry — the upstream phone-home paths (update checks, SUBNET, call-home) are removed outright, not merely disabled".
- **Release evidence.** "SHA-256 checksums, SPDX SBOMs, Sigstore-signed manifests, and GitHub build provenance"; GPG-signed packages; classic and distroless images for amd64 and arm64. The 2026-09-03 release was accepted on a four-node, 16-disk cluster through upgrade and rollback, node outage, quorum loss, drive rebuild and two-site replication of "versions, CORS, tags, quota, ILM, Object Lock, IAM, delete markers".
- **WORM-relevant repairs to inherited code**: replicated object-lock updates "ignored their timestamps", so "a stale replica could therefore turn a newer legal hold off or shorten a newer retention" (fixed in 2026-09-03); explicit version deletes needed only `s3:DeleteObject` (SN-2026-010); an object-only grant `bucket/*` authorised twelve bucket-level writes, including `PutBucketObjectLockConfiguration` and `DeleteBucket` (SN-2026-004); a presigned PUT could be turned into a copy that reads other data with an unsigned `x-amz-copy-source` header (SN-2026-011, fixed only in 2026-09-16).
- **Limits.** "We do not commit to a specific fix-time SLA." If maintenance stops for six months, the project will "say so publicly and archive the project". Its 2026-09-16 notes list open limitations in distributed operation: listings that can omit keys during rolling restarts with concurrent overwrites, delete-marker races across sites and multi-pool conditional writes.
- **Name.** "We renamed the project out of respect for upstream's trademarks."

### 12.6 How AKR-TMS runs MinIO safely

1. **Code line and build.** The store runs the MinIO code line as a **Silo build at a pinned release**, today `RELEASE.2026-09-16T00-00-00Z`. AKR-TMS compiles it from the signed tag in its own pipeline, checks the result against Silo's published checksums and provenance, signs the image with cosign, records it in the SBOM (maintainer and country, VO-07) and ships it in the offline bundle. Every advisory fixed in one maintained line is checked against the other (Silo against Chainguard). AKR-TMS carries its own patch only when neither line has fixed a confirmed finding within the patch deadline in item 9. Never used: upstream binaries or images, public registries at install time, AIStor.
2. **Name and licence.** The component is called "object store (MinIO code line, Silo build)" in AKR-TMS documents and may be described as "incorporating MinIO source code"; no build is branded MinIO. AKR-TMS talks to the store only over the standard S3 HTTP API, from separate processes and containers, and links none of its code. The AGPL treats such a combination as an "aggregate" that "does not cause this License to apply to the other parts", and the FSF's FAQ says that "pipes, sockets and command-line arguments are communication mechanisms normally used between two separate programs" [V]. Silo states the same reading: "using Silo through its S3 API does not make your application a derivative work" [V]. Two obligations still apply: delivering the store to a bank conveys object code, so each release ships the Corresponding Source of the exact build with LICENSE, NOTICE and CREDITS; and any AKR-TMS patch is published, because the AGPL's network clause requires a modified version to "offer all users interacting with it remotely ... an opportunity to receive the Corresponding Source" [V]. MinIO's own compliance note leaves the judgment to "your own legal counsel" [V], so counsel confirms this reading before the first delivery.
3. **Topology.** The default is one node with an erasure set across at least four drives. Most of the open distributed-mode limitations then do not apply, and evidence volumes are small (file 30 section 12). Banks that require high availability get four nodes in a single pool, upgraded together. AKR-TMS uses no multi-pool layouts and no site replication; disaster recovery uses backups (item 11), and replication is allowed only on Silo `RELEASE.2026-09-03` or later because of the lock-timestamp repair. An outage of the store delays evidence writes, which wait in the jobs table, but loses nothing.
4. **Smallest attack surface.** The console is off (`MINIO_BROWSER=off`). Identity federation is not configured: no LDAP or OIDC (so CVE-2026-33322 and CVE-2026-33419 cannot apply), only static service accounts created by the installer. SFTP and FTP, bucket notifications, Object Lambda, batch jobs, replication and the update API are off or unused. Egress is denied and no identity holds `admin:ServerUpdate`. The store runs as a non-root user with a read-only root filesystem (CVE-2026-42600 notes that containers running as UID 0 expose more). K3s network policy admits only the AKR-TMS services to the S3 port, only the nodes to each other's internode port, and only the installer job to the admin API.
5. **An S3 guard in front of the store.** A small allow-list proxy (a few hundred lines of Rust in the workspace) passes only the operations AKR-TMS uses: put, copy, get, head, list versions, multipart, delete in the quarantine bucket, retention and legal-hold reads and writes, and bucket creation by the installer. It rejects the request shapes behind the published flaws:

| Rejected | Advisory it neutralises |
|---|---|
| `select` query parameter (S3 Select) | CVE-2026-39414 |
| `X-Amz-Meta-Snowball-Auto-Extract` header | CVE-2026-40344 |
| `STREAMING-UNSIGNED-PAYLOAD-TRAILER` uploads (the AKR-TMS client does not use them) | CVE-2026-40344, CVE-2026-41145 |
| Query-string credentials on any method other than GET and HEAD (no presigned writes) | CVE-2026-41145, SN-2026-011, SN-2026-012 |
| Any client header beginning `X-Minio-`, and `X-Amz-Bucket-Replication-Status` | CVE-2026-34204, SN-2026-008 |
| `x-amz-*` headers absent from the signed-headers list | SN-2026-011 |
| Admin, STS, console and internode paths | CVE-2026-33322, CVE-2026-33419, CVE-2026-42600 |

   The guard checks no signatures; the store still does that. It shrinks what any future flaw can reach, and on a low-volume evidence path it adds little latency.
6. **Keys.** Three kinds of credential:
   - A writer key may put objects, set retention and legal holds, and read for verification. It may delete only in the quarantine bucket.
   - A presigning key may only get objects under the prefixes served to analysts. Its access-key ID appears in every URL, so it holds no write right, and flaws of the CVE-2026-40344 and CVE-2026-41145 kind gain nothing from it.
   - An installer and break-glass admin key is generated at install, held in OpenBao and never placed in an application's environment.

   Policies name both the bare bucket ARN and `bucket/*` explicitly, and they explicitly deny `s3:DeleteObjectVersion`, `s3:BypassGovernanceRetention` and every bucket-level write (SN-2026-004, SN-2026-010). Presigned URLs are GET only, live 60 to 300 seconds and are issued after a Cedar decision and an audit row (the DBS pattern).
7. **Encryption.** AKR-TMS encrypts evidence before upload: envelope encryption, with data keys wrapped by an OpenBao transit key, as for database fields (section 13). Disks use LUKS underneath. Server-side encryption through KES is not used, because KES is archived.
8. **Object lock and time.** The `akr-evidence` and `akr-anchors` buckets are created with object lock and a COMPLIANCE default retention (bucket map in 12.8). Retention runs on the store's clock, so NTP is monitored and the store's clock is compared with the platform's at every run of the conformance suite (file 30).
9. **Conformance suites and the advisory watch.**
   - **WORM conformance suite**, in CI and at every installation and upgrade:
     - create a locked bucket and write a version;
     - as the root credential, try to delete the version, overwrite it, shorten its retention and delete the bucket, all of which must fail;
     - set and clear a legal hold;
     - confirm that default retention is applied;
     - verify checksums and the expiry of presigned URLs;
     - restore from backup and confirm that retention and version identifiers survived;
     - check behaviour under clock skew.
   - **Security regression suite**, one test per published advisory: forged-signature unsigned-trailer writes, query-string credentials on unsigned-trailer uploads, Snowball extraction with a forged signature, replication headers on an ordinary PUT, an unsigned copy-source on a presigned PUT, an oversized S3 Select record, and so on. It runs against the store with the guard and again without it.
   - **Advisory watch.** The watch reads the upstream GitHub advisories (MinIO still listed advisories for the community range until the archive), Silo's advisory ledger including its SN entries, the Chainguard fork's commits, `govulncheck` over the pinned source, and Go releases. Findings are triaged within two working days. High and critical findings ship in a signed update within 14 days, others at the next monthly release, all through the bank's change approval (TS-30). The store certification report records the code line, tag, commit, image digest, advisories reviewed and suite results.
10. **Tested exits.** The S3 port keeps other stores runnable, and an exit drill runs once a year: the WORM suite against current SeaweedFS (Apache-2.0, 4.48 of 2026-09-28; its object-lock wiki says COMPLIANCE objects "cannot be deleted or modified by any user, including the root user") and RustFS (Apache-2.0, 1.0.1 of 2026-10-03; it describes itself as a MinIO reimplementation with a "byte-compatible on-disk format", a claim to test), plus a migration of a sample bucket with retention re-applied [V]. Ceph RGW and a bank's own WORM array (StorageGRID, ECS, Cloudian and similar [K]) remain deployment options behind the same port. Evidence manifests identify items by SHA-256, not by store version identifiers alone, so a migration that changes version identifiers keeps every item verifiable.
11. **Backups.** The DBS pattern stays: restic over the drive directories (the per-object metadata files carry retention), taken from a consistent snapshot of all drives. A quarterly restore drill confirms that retention and version identifiers came back, and records the result in the audit trail.

### 12.7 Residual risks

| Risk | Why it remains | Mitigation |
|---|---|---|
| Both maintained lines stop | Silo is one project with no fix-time SLA; the Chainguard fork is best effort | Two independent lines; Silo's six-month archive notice; AKR-TMS's own patch capacity (Go skills in the team); tested exits; evidence identified by hash |
| Findings without CVE numbers | Upstream disclosure ended with the archive; SN identifiers are not CVEs | The watch reads ledgers and commits, not only scanners |
| Regressions from fork changes | Silo changes a lot between releases (its renaming release alone touched 394 files) | Pinned release, both suites, staged rollout (test lab, pilot bank, others), rollback target per release |
| Licence interpretation | AGPL network clause | Separate processes over the S3 API, Corresponding Source shipped, patches published, counsel's opinion |
| Trademark | MinIO's policy for modified builds | No MinIO branding; descriptive mention only |
| Supply chain | Code from a Singapore-registered project and a US company | Build from source, verify provenance, maintainer and country in the SBOM (VO-07) |

### 12.8 AKR-TMS hooks

- `evidence` context: pack manifest (SHA-256 per item, size, type, classification), legal-hold flag, retention class, store version id; verification on serve; stamped PDF with a QR (the DBS pattern) for STR packages.
- **Store certification report** with each deployment profile: code line, tag, commit, image digest, WORM and security-suite results, advisories reviewed.
- **Bucket map**: `akr-evidence` (COMPLIANCE, retention by class), `akr-anchors` (COMPLIANCE 10 years, audit and coverage-certificate anchors), `akr-quarantine`, `akr-models`, `akr-exports`.
- **S3 guard** with a rule-to-advisory map, versioned with the store.
- **Advisory watch** tile in the ops cockpit: store version, days since the last review, open findings by severity.

---

## 13. Secrets and keys (OpenBao replaces Vault)

### 13.1 What the DBS does

Vault 1.17 on raft storage, sealed after any restart and unsealed by key holders (3 of 5), with separate narrow tokens for `api` and `worker` and no root token in production; transit engine keys for the audit-anchor signature (Ed25519), the QR-token signature (key versions embedded in tokens) and data-key wrapping (`dek-wrap`); KV for secrets; PKI planned; **Vault is never in the request hot path** (data keys are unwrapped at start and cached) (`ARCHITECTURE.md` 10 and 13, `deploy/prod/README.md`, `vault.hcl`, `backup-restore.md`).

### 13.2 Decision of 2026-10-07: OpenBao replaces Vault

HashiCorp's Vault has been under the Business Source License since version 1.15; the licensor is now IBM. The additional use grant allows production use "provided Your use does not include offering the Licensed Work to third parties on a hosted or embedded basis in order to compete with IBM Corp's paid version(s)". "Embedded" includes "packaging the competitive offering in such a way that the Licensed Work must be accessed or downloaded for the competitive offering to operate"; "Hosting or using the Licensed Work(s) for internal purposes within an organization is not considered a competitive offering" [V, LICENSE in `hashicorp/vault`]. An AML product that needs Vault is not a secrets-management product, so the grant most likely covers it, but the bank receives Vault as part of a paid product, which is exactly the situation the clause describes. That uncertainty, and the licence screen for products sold to banks, led the owner to replace Vault with OpenBao on 2026-10-07.

**OpenBao** (Linux Foundation, OpenSSF; MPL-2.0 [V]) is the open-source fork, and it exposes the Vault HTTP API for KV, transit and PKI [V, README]. Its CHANGELOG shows two maintained lines (2.7.1 and 2.6.4, both released 2026-10-01 with security fixes) and several features that Vault keeps for its Enterprise edition [V]:

- namespaces with tenant isolation since 2.3.1 (June 2025), and per-namespace Shamir sealing since 2.6.0;
- a static key unseal mechanism (2.4.0);
- auto-unseal through KMS plugins, including PKCS#11 (2.6.0). PKCS#11 and the cloud seals became external plugins in 2.7.0, when the separate HSM build was discontinued; the transit seal stays built in;
- "External Keys" (2.7.0): the Transit and PKI engines "can now use KMS plugins to perform cryptographic operations without storing key material in OpenBao", so a bank can keep AKR-TMS signing keys in its HSM.

Version 2.7.0 also adds ML-DSA (FIPS 204) signatures to Transit and PKI and post-quantum TLS key exchange, and version 2.7.1 removes the `file` storage backend (the DBS uses raft) [V]. Because the DBS talks to Vault over HTTP (`common/src/vault.rs`) and OpenBao keeps that API, the change is a server swap followed by proof: the **conformance checklist** runs against OpenBao in CI (transit Ed25519 sign and verify, key version listing for the QR scheme from `transit/keys/<key>`, rewrap, KV v2 versions, token policies, raft snapshots, Shamir unseal). A bank that already runs and licenses its own Vault may point AKR-TMS at it if the same checklist passes.

### 13.3 Key hierarchy for AKR-TMS

| Layer | Holder | Use |
|---|---|---|
| Unseal | Shamir 3 of 5 key holders in the bank (sealed envelopes in separate sites, the DBS ceremony); or auto-unseal through the bank's HSM (PKCS#11 KMS plugin), a second OpenBao server (transit seal) or a static key (2.4.0) [V] | Root of trust |
| KEK | Transit key (`aes256-gcm96`), versioned | Wraps data keys |
| DEK | Random 256-bit per table family or tenant, wrapped by the KEK, cached in process memory, AES-GCM locally (the `aes-gcm` crate) | Field-level encryption of personal data |
| Match-key hash key | A separate transit or KV key | HMAC-SHA256 keyed hashes of national id, phone, PAN token so records match without decryption |
| Signing keys | Transit Ed25519 (or a key held in the bank's HSM through External Keys), versioned, key version in every artifact | Audit anchors, coverage certificates, phase-2 decisions, model artifacts, STR package stamps |
| Internal TLS | PKI engine (or the bank's CA) with cert-manager | mTLS between components and from gateways |
| CBS credentials | KV, written by the bank's process (file 25 section 10.2) | Read-only database accounts |
| Backup key | Held outside the server (the DBS rule) | pgBackRest and restic repositories |

### 13.4 An availability hazard to design out

OpenBao, like Vault, restarts sealed. In the DBS, anything that needs a key (anchoring, QR signing, field decryption) waits until key holders unseal it. For AKR-TMS the phase-2 assessment service must **not** depend on an unseal at restart: a node that reboots at 03:00 with Vault sealed would fail closed for every cross-border payment until three key holders arrive. Mitigations, in order of preference: transit auto-unseal from a second OpenBao server in the bank's secure zone; auto-unseal through the bank's HSM with the PKCS#11 KMS plugin; a static key unseal where the platform's own secret store is trusted [V, OpenBao CHANGELOG]; and, as a documented fallback, running the assessment nodes with the signing key and data keys cached in process memory so that a restart of OpenBao, not of the node, does not interrupt service. The node restart case needs the first or second option.

### 13.5 AKR-TMS hooks

- **API conformance checklist** run against OpenBao in CI, and against a bank's own Vault when it brings one.
- **Key ceremony runbook** per bank (generation, custody, rotation, rewrap, destruction), and a **sealed-state drill** in the quarterly DR test.
- **Key-version discipline**: never raise `min_decryption_version` past a version that signed an artifact (the DBS rule for QR keys), because verification evidence would be lost.

---

## 14. Identity, authorisation and RBAC

### 14.1 What the DBS provides

Kratos holds credentials (TOTP and WebAuthn), Hydra issues tokens, and the API acts as a backend-for-frontend with session cookies and HMAC-bound CSRF. Roles come from Kratos identity traits and are mirrored into the request; Cedar bundles are stored in PostgreSQL, activated atomically, can be simulated ("can user X do action Y on resource Z", with a full attribute trace) and dry-run against live traffic before enforcement. Clearance is an attribute; insufficient clearance returns the same 404 as a missing record ("existence masking"); every decision, allowed or denied, is audited; a confined session that has not reached AAL2 may reach only a short allow-list; signing-grade actions require a single-use step-up assertion consumed inside the action's transaction (`auth.md`, `MODULES.md` M3).

### 14.2 AKR-TMS roles in Cedar terms (detail in file 30)

Analyst level 1 and 2, MLRO and deputy, branch liaison officer, compliance administrator, model-risk officer, internal auditor (read-only), external auditor or CBI inspector (read-only, time-boxed), bank IT operations (no business data), vendor support (break-glass only, time-boxed). Cedar attributes: branch scope, clearance tier (normal, restricted for staff and PEP customers, STR-secret), case ACL membership, step-up freshness. Policy files follow the DBS numbering (`00_deny_disabled.cedar` first, `forbid` for disabled accounts, one file per capability) and ship as a bundle row.

### 14.3 Bank directory federation: the one identity gap

DBS decision D2 chose standalone Kratos ("no AD federation") because it wanted to own identity and one-click offboarding. Banks buying AKR-TMS will ask for single sign-on and central deprovisioning. Facts:

- Kratos has a **generic OpenID Connect provider** ("any OpenID Connect-certified provider that doesn't require custom API calls"), which covers Microsoft Entra ID, ADFS and Keycloak as upstreams; its documentation does not mention SAML, LDAP or Active Directory [V, Ory docs].
- **Ory Polis** (formerly BoxyHQ Jackson; Apache-2.0; v26.2.0) "bridges or proxies a SAML login flow to OAuth 2.0 or OpenID Connect" and supports SCIM 2.0 directory sync for provisioning and de-provisioning [V, README].
- LDAP is not supported natively; a broker such as Keycloak or Dex would be needed [K].

Recommendation: **default to standalone Kratos** with mandatory WebAuthn or TOTP and the DBS offboarding flow plus quarterly access review (consistent, fewest moving parts); support **generic OIDC upstream** as configuration; offer **Polis** as an optional component for SAML and SCIM. Whatever the upstream, **step-up for sensitive actions is enforced locally** (AAL2, single-use ledger), and Cedar remains the authority for what a user may see (branch scope, clearance, case ACL).

### 14.4 Machine identities

DBS personal access tokens (`mpat_` with scopes always at `aal1`) for scripted clients; mTLS client certificates for the assessment API, one per gateway, mapped to a Cedar principal; Hydra client credentials for internal calls; no shared service accounts.

### 14.5 Version drift

The DBS pins Kratos v1.2.0 and Hydra v2.2.0; upstream is at v26.2.0 for both (Ory moved to calendar versioning). Plan the upgrade path and test the BFF flows (the DBS notes that five of six identity defects passed the mocked suite and only a real Kratos found them) before AKR-TMS ships on any version.

### 14.6 AKR-TMS hooks

- Cedar bundle for AKR-TMS roles with simulation and dry-run screens reused.
- Federation configuration screens (OIDC upstream) and an optional Polis chart.
- Access-review report (quarterly, per branch and role) generated by the system (the DBS compensating control).

---

## 15. Observability

### 15.1 What the DBS built

`tracing` with JSON logs and correlation ids, spans over OTLP to a Jaeger v2 container that keeps 20,000 traces in memory (its configuration says that is right for "why was that request slow this morning" and "wrong for anything that is evidence. Evidence is the audit trail, in Postgres"), a `/metrics` endpoint through `metrics-exporter-prometheus`, labels by matched route (never raw paths), and the in-app **ops cockpit** (job queue and dead jobs, backups and the last restore drill, audit anchor status, disk, certificate expiry). Prometheus, Grafana and Loki are specified for the K3s platform layer and not built (section 1.4).

### 15.2 A plane that fits banks

| Concern | Choice | Licence (2026-10-06) |
|---|---|---|
| Metrics | Prometheus v3.15.0 scraping `/metrics` on every binary, with Alertmanager | Apache-2.0 |
| Traces | OpenTelemetry Collector 0.162.0 to Jaeger 2.22.0 (as the DBS) | Apache-2.0 |
| Logs | JSON to stdout, collected by the node agent, kept short; forwarded to the **bank's SIEM** over syslog or CEF; Loki 3.7.8 optional | Loki AGPL-3.0 |
| Dashboards | Grafana 13.2.3 optional | AGPL-3.0 |
| Operators' first view | The in-app cockpit | Own code |

Licence note: Grafana, Loki and Tempo are AGPL-3.0. Used unmodified as separate services, an internal bank deployment does not trigger source-release duties for AKR-TMS, but a vendor that **ships** them must ship licence notices and an offer of source and should have counsel review. The conservative design ships Prometheus, the OTel Collector and Jaeger (Apache-2.0), exposes standard interfaces (Prometheus scrape, OTLP, JSON logs, syslog and CEF), makes Grafana and Loki optional, and lets a bank with Zabbix, Elastic or Splunk consume the same signals.

### 15.3 Signals that matter for AKR-TMS

Ingest lag and batch rate per source; reconciliation status per source per day; coverage percentage; paused-by-bank state; scenario run duration; alert and case queue age; assessment API latency percentiles, error and degraded-mode counts, list and model staleness; job queue depth and dead jobs; backup heartbeat and last restore drill; OpenBao seal state; object-store advisory status; certificate expiry; disk per volume. Each has an SLO, a burn-rate alert and a place in the cockpit.

### 15.4 Hygiene

No personal data or case content in logs or span attributes (the DBS never logs cookies, tokens, step-up assertions or secrets; AKR-TMS adds customer names and message text); identifiers only, with correlation ids to join to the audit trail. Telemetry stays in the bank; support bundles are redacted and leave only by a bank-approved route.

### 15.5 AKR-TMS hooks

- Metric catalogue with SLOs and alert rules shipped as code (Prometheus rules, cockpit tiles).
- A redaction filter on the tracing layer, with a test that fails if a configured sensitive field reaches a log.
- SIEM export of audit and security events in CEF and syslog (file 30 section 10).

---

## 16. Packaging: K3s on-prem, air-gapped updates through Harbor, supply chain

### 16.1 Two profiles from one set of images

| Profile | For | Shape |
|---|---|---|
| **P1 Compose bundle** | Pilots and smaller banks | The DBS production pattern: one server plus a backup server, Caddy edge, `bootstrap.sh`, pgBackRest and restic, release and rollback scripts. Proven in the DBS |
| **P2 K3s and Helm** | Banks that need the phase-2 API highly available | Three K3s servers with embedded etcd ("an odd number of server nodes... a minimum of three" [V, K3s docs]); default-deny NetworkPolicies; cert-manager issuing from the bank's CA; automated PostgreSQL failover (16.3) |
| P3 Hosted | Later, if the CBI cloud rules allow (file 01 section 8) | Section 17 |

Current versions: K3s v1.37.1+k3s1 (Kubernetes 1.37.1, 2026-09-30), cert-manager 1.21.2, Harbor v2.15.3, Gitea 28.0.0 [V, GitHub]. The DBS's own sizing assumes two servers; the CBI's continuity controls expect an alternate site far from the primary but reachable (file 30 section 16), so a bank needs at least a second site in either profile.

### 16.2 Air-gapped installation and upgrade

K3s documents three air-gap methods: a private registry configured through `registries.yaml`, image tarballs placed in `/var/lib/rancher/k3s/agent/images/`, and an embedded distributed registry mirror; the files to carry in are the K3s binary, the architecture-specific airgap images tarball, and the install script run with `INSTALL_K3S_SKIP_DOWNLOAD=true`; automated upgrades need the `k3s-upgrade` and `system-upgrade-controller` images in the private registry [V, K3s docs].

### 16.3 Release path from vendor to bank

```text
vendor: Gitea Actions -> build, test, scan -> Harbor (vendor)
        -> sign images and charts (cosign) + SBOM (CycloneDX) + provenance
        -> release bundle (OCI layout: images, Helm charts, adapter packs, migrations, docs,
           SBOMs, signatures, checksums)  [made with Hauler 2.1.1 or skopeo/oras]
transfer: bank-approved route (signed media, bank file gateway)
bank:   import into bank Harbor -> Harbor enforces content trust (signed before pull)
        + Trivy scan -> bank change board approves -> Helm upgrade (or compose script)
        -> health checks -> release marker; rollback = previous release + restore point
```

- **Harbor** supports Cosign and Notation; "a project administrator can configure a project to enforce content trust, making it required for all artifacts to be signed before they can be pulled", and Cosign signatures are stored as accessories next to the artifact [V, Harbor docs].
- **Signing without internet**: keyless signing needs online transparency services, so the vendor signs with a key held in its KMS or HSM and the bank pins the vendor public key in Harbor and in an admission policy (Kyverno 1.19.1 or sigstore policy-controller are the candidates [V repos; configuration not verified]).
- **Bank approval before change**: the CBI's 2026 outsourcing controls require a contract in which the provider makes no change to the service without the bank's prior approval (file 30 section 2), so every update ships with release notes, an SBOM diff, test evidence and a rollback plan, and nothing is applied by the vendor remotely.
- **Database migrations** follow the DBS rules (additive where possible, never edit an applied migration, a named restore point before migrating, `migrate --ignore-missing` for code-only rollback).

### 16.4 CI gates

`cargo fmt --check`, `clippy -D warnings`, tests against a throwaway PostgreSQL, `sqlx prepare --check`, `cargo deny` 0.20.2 (advisories, licences, bans), `cargo audit` 0.22.2, `cargo-auditable` 0.7.7 to embed dependency data in binaries, `cargo-cyclonedx` 0.5.9 for CycloneDX SBOMs, Syft 1.54.1 for image SBOMs, Trivy 0.75.0 scan, OpenAPI and Orval drift checks, frontend lint, `tsc` and vitest, signing with cosign v3.1.3. The DBS plan lists `cargo audit`, `cargo deny`, Trivy and signed commits; SBOMs, image signing and provenance are the additions (file 30 section 14). GitHub Actions workflow files port to Gitea Actions with little change.

### 16.5 PostgreSQL high availability on three nodes

The DBS accepts manual failover on two servers (promote the replica, flip DNS) and says so to leadership. For banks on P2, automate it: **CloudNativePG** v1.30.1 (Apache-2.0, CNCF) or **Patroni** v4.1.5 (MIT, with an external consensus store) [V, GitHub]. The deciding factor is backup consistency: the DBS runbooks are built on pgBackRest (2.59.3), and Patroni plus pgBackRest keeps them; CloudNativePG brings its own backup tooling [K]. Choose in a proof of concept; until then P1 keeps the DBS manual procedure.

### 16.6 Hardware profiles (planning assumptions, to be replaced by discovery)

| Profile | Nodes | Indicative size | Notes |
|---|---|---|---|
| S | 1 server plus 1 backup server | 16 cores, 64 to 128 GB RAM, 2 to 4 TB NVMe | Up to roughly 300,000 accounts and 100,000 postings a day [K] |
| M | 3 nodes, one at a second site | 24 to 32 cores, 128 to 256 GB RAM each, NVMe | Up to roughly 2 million accounts; phase-2 HA |
| L | 3 application plus 3 data nodes, GPU optional | By load test | State-bank scale; salary-day peaks |

### 16.7 AKR-TMS hooks

- Helm chart and compose bundle generated from one source of truth for image tags and environment.
- Release-bundle builder, bundle verifier (checks signatures, checksums, SBOM presence) and importer script for the bank's Harbor.
- Installer pre-flight (NTP, disk, character sets, driver volume, certificate chain, time to OpenBao).

---

## 17. Single-tenant per bank versus a hosted multi-tenant option

### 17.1 Default: single tenant per bank

One installation per bank, in the bank's data centre, as the DBS is installed at one bank. It matches banking secrecy (Banking Law 94/2004; file 01), the absence of a localisation law but also of cloud rules (file 01 section 8), the CBI's outsourcing expectations (the bank keeps ownership and control of data), and the read-only link to a CBS that sits on the bank's network.

### 17.2 If a hosted option is ever offered

| Model | Isolation | Cost | Verdict |
|---|---|---|---|
| **Cluster per bank** (own K3s, own database, own object store, own keys) in a vendor-run or Iraqi data centre | Strongest; the same artifacts as on-premises | Highest | The only hosted model to start with |
| Namespace per bank on a shared cluster, a dedicated PostgreSQL cluster per tenant, per-tenant buckets and credentials, per-tenant key mounts, default-deny NetworkPolicies and per-tenant ingress | Strong but shares a control plane and nodes | Lower | Acceptable later with isolation tests and the CBI's cloud rules known |
| Shared database with `tenant_id` and row-level security | Weak: noisy neighbour, one bug exposes every bank, one backup mixes tenants | Lowest | **Reject** for banks |

Vault's community edition has no namespaces (an Enterprise feature [K]); OpenBao has had namespaces since 2.3.1 and per-namespace sealing since 2.6.0 [V]. The cluster-per-bank model still runs one OpenBao per bank; namespaces matter only if a hosted model is ever offered. The CBI's cloud rules were still undefined in the research (file 01 section 8), so a hosted option waits for them.

### 17.3 Requirements for any hosted model

Per-tenant KEK and data keys; per-tenant backup and restore and an exit path (export of data and evidence in open formats); data residency inside Iraq; vendor support access only by just-in-time, bank-approved, recorded sessions (file 30 section 13); tenant isolation tests (cross-tenant access attempts at network, database, object store and key layers) in CI and annually by an external tester.

### 17.4 AKR-TMS hooks

- `tenant` concept limited to configuration (name, branding, language defaults), not to data separation; data separation is by deployment.
- Isolation test suite maintained from the first hosted pilot.

---

## 18. Frontend: React 19, Orval, Tailwind v4, Arabic RTL and i18n, charts, graph views, tables

### 18.1 What the DBS frontend gives AKR-TMS

`package.json` (2026-10): React 19.2, TanStack Router 1.170 and Query 5.101, TanStack Table 9.1, Orval 8.22, Tailwind 4.3 with the Vite plugin and no config file, `react-aria-components` over a vendored Untitled UI component set (the open `untitleduico/react` repository is MIT; `@untitledui/icons` is MIT [V]; check that no PRO-licensed component was vendored), i18next 26 with `react-i18next` 17, Zod 4 with react-hook-form, `@xyflow/react` 12 with `elkjs` (workflow canvas), tiptap, `motion`, `sonner`, a PDFium viewer, vitest 4. Fonts are vendored through Fontsource (Inter Variable, IBM Plex Sans Arabic, Cairo, Noto Sans Arabic, Carlito) with "no runtime font requests (egress rule D6)". Internationalisation code lives in `frontend/src/i18n/` (`direction.ts`, `bidi.ts`, `datetime.ts`, an Arabic `fold.ts` with a SQL parity test, `AriaLocale.tsx` to give react-aria the locale).

Hard rules from the DBS that carry over (`frontend.md`, `MODULES.md` 16.5, `PRODUCT.md`): `dir` set from the locale on `<html>`; logical properties only (`ms-*`, `me-*`, `ps-*`, `pe-*`, `text-start`; "a PR containing `ml-`/`mr-` fails review"); directional icons flip through a wrapper; Western digits and `YYYY-MM-DD HH:MM` in both languages; no `toLocaleString`, no `input type="date"`; `dir="ltr"` on a whole numeric expression, not its parts; every string in Arabic and English from the first commit; machine ids never printed to a reader (translate by rule and key, store i18n keys in state); a control that cannot work is not rendered (the API returns affordances); a refused request never looks like an empty list; status never by colour alone; WCAG AA in both themes.

### 18.2 What AKR-TMS adds, and what to choose

| Need | Options (2026-10-06 versions) | Recommendation |
|---|---|---|
| Dense alert, case and transaction lists with 100,000-row backing sets | TanStack Table 9.2.6 plus TanStack Virtual 3.14.13 (MIT) [V, npm] | Use, with server-side keyset pagination and sorting; pin columns by logical start and end |
| Dashboards (volumes, aging, coverage, SLA) | Recharts 3.10.1 (MIT, React components, SVG); ECharts 6.1.0 (Apache-2.0, canvas, heavy); visx `xychart` 4.0.0 (MIT) [V, npm] | **Recharts by default**; ECharts only if a chart needs canvas performance. **Run a short RTL spike first**: axis mirroring, tooltip direction, legend order and digit rendering are unverified for every option |
| Network views for investigators | `@xyflow/react` 12.12.0 for small ego graphs; Sigma 3.0.3 with graphology 0.26.0, or Cytoscape 3.34.3, or reagraph 4.32.0 (Apache-2.0) for large neighbourhoods [V, npm] | xyflow for the common case; Sigma or Cytoscape behind a "large graph" switch; labels use `<bdi>`; toolbars mirror, graph geometry does not |
| Name-match comparison (Arabic RTL beside Latin LTR, aligned name parts, character-level highlights) | New component | Per-token isolation with `dir="auto"` and `<bdi>`; highlight on the folded text and map offsets back to the original, as the DBS search does ("a best-effort highlight computed against it"); share `fold.ts` and its parity test |
| Forms and workbenches (mapping, scenarios, change requests) | DBS dynamic form renderer, react-hook-form with Zod | Reuse; the mapping and scenario editors are schema-driven forms |
| PDF and report output | Gotenberg 8.37.0 (MIT) with the DBS custom image carrying Arabic fonts; PDFium viewer | Reuse; STR packages and board packs rendered by HTML templates, never by pure-Rust PDF libraries (the DBS records that they cannot shape Arabic) |
| Live updates | SSE bridge with typed events from the Orval-generated enum | Reuse; subscribe deliberately and list ignored events |

### 18.3 Terms and quality

AML terminology in Arabic (الإبلاغ عن المعاملات المشبوهة, غسل الأموال, تمويل الإرهاب, العناية الواجبة المعززة and the rest) is a glossary owned by compliance staff, not machine-translated (file 31). Arabic alert templates are reviewed by the bank's compliance team. Nothing is fetched at run time: libraries, fonts and icons are bundled (egress rule), including any chart or graph library.

### 18.4 AKR-TMS hooks

- **RTL spike** (two days): render each candidate chart and graph library in Arabic with mixed-script labels and screenshot against a checklist; the result is recorded as a decision record.
- **Shared fold library** for client and server with a parity test (the DBS has the pattern).
- **Component catalogue** for the analyst UI (list, timeline, comparison, graph, evidence pack viewer) built on the vendored base components, documented in the design system for the new AKR-TMS identity.

---

## 19. Licence review of the stack as a product sold to banks

AKR-TMS is delivered to banks, so every component is redistributed or operated for them. Licence checks are legal work; this table records what was found and what needs counsel.

| Component | Licence (verified where marked) | Risk | Action |
|---|---|---|---|
| Rust crates in the DBS tree (axum, sqlx, tokio, cedar-policy, jsonschema, tiberius, odbc-api, ort, cel, tantivy, petgraph and others) | MIT, Apache-2.0 or both [V, crates.io] | Low | `cargo deny` licence policy in CI; third-party notices file |
| `oracle` crate | UPL-1.0 or Apache-2.0 [V] | Low | The Oracle **client** libraries it loads are proprietary and not redistributed (4.3) |
| PostgreSQL, pg_trgm, unaccent, pg_partman | PostgreSQL licence [V for pg_partman] | Low | None |
| PGroonga | PostgreSQL-style licence [V, COPYING]; Groonga is LGPL-2.1 [V] | Low to medium | Ship licence notices with the image; keep PGroonga's use to text search |
| **Redis** (DBS) | 7.2 and earlier BSD-3; **7.4 to 7.8 RSALv2 or SSPLv1; 8.x RSALv2, SSPLv1 or AGPLv3** [V, Redis README]. The DBS compose uses the floating tag `redis:7-alpine` | **Medium to high** | **Not shipped: Valkey replaces it** (decision 2026-10-07) |
| **Valkey** | BSD-3-Clause, 9.1.2 [V]; the `redis` Rust client is BSD-3 [V] | Low | Pin a release; notices |
| **HashiCorp Vault** (DBS) | BUSL-1.1, licensor IBM, from 1.15 [V] | Medium | **Not shipped: OpenBao replaces it** (decision 2026-10-07); a bank's own licensed Vault only through the conformance checklist |
| **OpenBao** | MPL-2.0 [V] | Low | Ship unmodified with notices (section 13) |
| **MinIO code line (Silo build)**, `mcli` in the installer only | AGPL-3.0-or-later [V]; upstream MinIO and `mc` archived | Medium | **Kept by decision** (section 12): separate process over the S3 API; ship the Corresponding Source of the exact build with LICENSE, NOTICE and CREDITS; publish any AKR-TMS patch; no MinIO branding (MinIO trademark policy); counsel to confirm |
| AIStor Free, AIStor Enterprise | Proprietary MinIO licences; the Free tier forbids redistribution and high availability [V] | n/a | Never used |
| SeaweedFS, RustFS | Apache-2.0 [V] | Low | Tested exits only (12.6, item 10) |
| Ory Kratos, Hydra, Polis | Apache-2.0 [V] | Low | Upgrade from v1.2.0 and v2.2.0 |
| Harbor, K3s, Helm, cert-manager, Caddy, CloudNativePG, Kyverno, cosign, Syft, Trivy | Apache-2.0 [V] | Low | None |
| Gitea | MIT [V] | Low | None |
| Patroni, restic, pgBackRest | MIT, BSD-2-Clause, MIT [V] | Low | None |
| Prometheus, OpenTelemetry Collector, Jaeger | Apache-2.0 [V] | Low | None |
| **Grafana, Loki, Tempo** | **AGPL-3.0** [V] | Medium if shipped | Make optional, ship unmodified with notices and source offer, counsel review |
| Debezium | Apache-2.0 [V] | Low | Java 21 runtime in the sidecar image |
| **OpenLogReplicator** | **AGPL-3.0** [V] | Medium | Not bundled by default |
| Oracle GoldenGate, IBM InfoSphere Data Replication, Oracle Active Data Guard | Commercial, customer-owned | n/a | Never assumed (file 25) |
| Oracle client libraries, IBM Db2 CLI and ODBC driver | Proprietary | n/a | Bank-supplied volume; redistribution terms not verified |
| ClamAV | GPL-2.0 [V]; separate service | Low | Separate container, notices |
| Gotenberg | MIT [V]; bundles Chromium and LibreOffice [K] | Low to medium | Notices for bundled components |
| ntfy | Apache-2.0 [V] | n/a | Not needed for AML |
| React, TanStack, Tailwind, i18next, Zod, react-hook-form, Recharts, Sigma, Cytoscape, xyflow | MIT [V, npm] | Low | Licence checker in the frontend build |
| react-aria-components, ECharts, reagraph | Apache-2.0 [V, npm] | Low | Notices |
| Untitled UI React (open repository), `@untitledui/icons` | MIT [V] | Low | Confirm that no PRO-licensed component or icon is vendored |
| Fonts: Inter, IBM Plex Sans Arabic, Cairo, Noto Sans Arabic, Carlito (Fontsource) | SIL Open Font Licence or equivalent [K] | Low | Notices |
| ONNX Runtime, LightGBM, XGBoost, SHAP | MIT, MIT, Apache-2.0, MIT [V] | Low | Pin versions |

Process: generate the third-party notices file and the SBOM at each release; legal sign-off on the AGPL components (the MinIO code line, the Grafana family and OpenLogReplicator) before the first bank delivery; Vault and Redis no longer ship; contract language that models trained on a bank's data belong to the bank (file 29).

---

## 20. Gap list and stack-consistent proposals

| # | Gap in the DBS stack | Why it matters | Proposal (consistent with the stack) | Section |
|---|---|---|---|---|
| G-01 | No driver for Oracle, SQL Server, Db2, Sybase (SQLx covers PostgreSQL, MySQL, SQLite; MSSQL removed) | AKR-TMS must read these engines | `tiberius`, `oracle`, `odbc-api` behind one port; bank-supplied client libraries; Debezium Server as the single JVM sidecar for agreed Oracle or Db2 CDC | 4 |
| G-02 | No streaming or CDC component | Latency and deletes for banks that agree to log-based capture | Debezium Server with HTTP or NATS JetStream sink; otherwise polling; no Kafka | 4, 5 |
| G-03 | **MinIO and mc archived**; seven post-archive advisories fixed upstream only in proprietary AIStor; AGPL | Evidence vault integrity and legal risk | MinIO kept (decision 2026-10-07): pinned Silo build compiled from source, Chainguard fork as second source, S3 guard, least-privilege keys, WORM and security regression suites, advisory watch; SeaweedFS, RustFS and bank WORM arrays as tested exits | 12 |
| G-04 | Vault is BUSL-licensed | Redistribution to banks | OpenBao (MPL-2.0) replaces Vault (decision 2026-10-07); keep the Vault-compatible HTTP client; conformance checklist in CI | 13 |
| G-05 | Redis licence (7.4 and later) | Same | Valkey (BSD-3) replaces Redis (decision 2026-10-07) | 19 |
| G-06 | K3s, Helm, Gitea, Harbor, observability exist only on paper | The bank deliverable is a platform, not a repository | Build the layer; ship the compose profile first; one source of truth for tags and environment | 1.4, 16 |
| G-07 | Manual PostgreSQL failover | The phase-2 API and the CBI's continuity expectations | Proof of concept: CloudNativePG versus Patroni with pgBackRest for the three-node profile | 16.5 |
| G-08 | No ML training or serving | Alert scoring and anomaly detection | Offline workbench in the bank; ONNX Runtime through `ort` (pinned build) with tract fallback; in-house TreeSHAP; signed registry; shadow mode | 8 |
| G-09 | No detection engine (the DSL is for workflows) | Scenarios are the product | Scenario DSL: JSON Schema, SQL from a closed operator set, CEL for event conditions, backtests, shadow, maker-checker | 7 |
| G-10 | No graph capability | Network features and investigator views | Edge tables, recursive CTEs, `petgraph` in workers, features in `feat`; AGE optional | 9 |
| G-11 | No name-matching engine (PGroonga is for text) | Screening quality and latency | Shared normalisation, in-memory list index, own scorer; pg_trgm for batch and back-office; benchmark harness | 10 |
| G-12 | Identity is standalone (no directory federation) | Banks expect SSO and central deprovisioning | Standalone by default; generic OIDC upstream; Ory Polis for SAML and SCIM; local step-up always | 14 |
| G-13 | Notification channel (WhatsApp) conflicts with tipping-off | Confidentiality | In-app and the bank's SMTP only; no subject names in text | 11.5 |
| G-14 | No charts, dense-grid or large-graph components | Analyst UI | TanStack Table plus Virtual; Recharts after an RTL spike; xyflow, Sigma or Cytoscape | 18 |
| G-15 | No SBOM, image signing or provenance in the planned CI | Supply-chain assurance for banks and the CBI | Syft and CycloneDX SBOMs, cosign signatures, Harbor content trust, admission policy, SLSA-style provenance | 16.3 to 16.4 |
| G-16 | Air-gapped update path undefined | Banks cannot pull from the internet | Release bundle (OCI layout) built with Hauler or skopeo, verified and imported into the bank's Harbor | 16.2 to 16.3 |
| G-17 | Version drift (Kratos v1.2.0, Hydra v2.2.0, cedar-policy 4.7.1, jsonschema 0.49.9, SQLx 0.8) | Shipping on old majors | Version policy and upgrade tests; start AKR-TMS on a deliberate set | 14.5, 1.4 |
| G-18 | goAML XML generation and XSD validation | STR filing (file 23) | No mature Rust XSD validator was found in this research [K]; validate with libxml2 bindings or a small sidecar; generate XML with `quick-xml` 0.42.0 | file 23 |
| G-19 | Object-store backup preserving lock metadata is store-specific | Evidence survives a disaster | Prove restore with the WORM suite for the MinIO line and for each tested exit | 12.6 |
| G-20 | Columnar analytics at very large scale | State-bank volumes | Read-only standby first; ClickHouse as derived copy only if measured need | 6.5 |
| G-21 | Four-binary zoning and network policies not in the DBS | Security of the CBS link | `ingestd` and `assess` as separate deployments with their own policies and credentials | 3.1 |
| G-22 | Product-level telemetry contract for bank monitoring stacks | Banks run Zabbix, Elastic, Splunk | Prometheus scrape, OTLP, JSON logs, syslog and CEF | 15 |

---

## 21. What it means for AKR-TMS

1. **The application architecture transfers; the platform layer does not.** Layered crates, bounded contexts, API-first with generated clients, problem+json and idempotency, the jobs and outbox harness, Cedar with simulation and dry-run, Kratos behind a BFF with single-use step-up, the audit hash chain with signed anchors and WORM copies, envelope encryption with the key service (OpenBao) out of the hot path, Arabic-first RTL rules and the ops cockpit are proven, documented and tested. The K3s, Helm, Harbor, Gitea and observability layer is only a plan in the DBS; AKR-TMS must build it, and a bank buying an AML product buys the platform too.
2. **MinIO stays, as a maintained and hardened code line, not the archived upstream.** The owner chose to keep MinIO (2026-10-07). Upstream is archived, and seven advisories published since its last release are fixed upstream only in MinIO's proprietary AIStor; two maintained AGPL forks (Silo and Chainguard's) have fixed all seven. AKR-TMS builds the store from a pinned Silo release, cross-checks against the Chainguard line, puts an S3 guard and least-privilege keys in front, proves every build with the WORM and security regression suites, watches the forks' advisory ledgers, and keeps SeaweedFS, RustFS and bank arrays as tested exits behind the same S3 port.
3. **Licence decisions: two taken, one for counsel.** OpenBao replaces Vault and Valkey replaces Redis (decided 2026-10-07). Counsel still confirms the AGPL reading for the MinIO code line, the Grafana family and OpenLogReplicator (separate programs; unmodified or with published source; optional where possible).
4. **The CBS drivers are the main new engineering.** `tiberius` and `sqlx` are clean; Oracle and Db2 need native clients that the bank supplies; one JVM sidecar (Debezium Server) is the justified exception, only where a bank agrees to log-based capture. CBS reads are the one place where compile-time SQL checking gives way to generated, parsed, read-only statements.
5. **No message broker.** PostgreSQL carries the pipeline and the raw landing table is the replay log; NATS JetStream appears only as a CDC buffer between zones. This keeps every bank's installation small and the failure modes few.
6. **Detection is mostly data and SQL.** Scenario documents compile to SQL over partitioned tables and CEL expressions; models are trained offline inside the bank and served as ONNX; TreeSHAP is implemented in house because the Rust crates are weeks old; graphs are edge tables plus `petgraph`; no graph database.
7. **Name matching is where the stack is weakest and the measurements most useful.** pg_trgm works on Arabic only after the same fold on both sides and is fast on list-sized indexes but unpredictable on customer-sized ones (2.7 s in a 2-million-row test), so the synchronous path searches an in-memory list index and pg_trgm serves batch and back-office search.
8. **Security by zoning.** Four binaries, the CBS-facing one alone holding credentials, signed phase-2 decisions, local step-up even with federated login, no WhatsApp, no per-customer CBS queries.
9. **Updates are a controlled bank process, not a vendor push.** A signed release bundle imported into the bank's Harbor, content trust enforced, bank change approval before any change, restore points and rollback: this satisfies the CBI's outsourcing controls and the air-gap reality.
10. **The frontend is mostly reuse, plus four new classes of component**: dense grids, charts, large graphs and bidi-aware name comparison, each needing an RTL check before choice.
11. **Suggested build order for the stack work**: `ingestd` with the poll path for ICS BANKS on Oracle (file 17), canonical store and reconciliation, the analyst API and case UI on the reused workflow engine, the scenario engine, screening with the in-memory index, then the phase-2 assessment API with the fail-policy matrix, then the K3s profile and air-gap pipeline.

### Open items and unverified points

- Oracle Instant Client and IBM driver redistribution terms (avoided by design); the Silo build under the WORM and security regression suites; OpenBao parity with the DBS's Vault client (expected through the shared API, to be proven); SeaweedFS and RustFS as exits under the same suites; chart and graph libraries in RTL; the ONNX-ML tree operator coverage of `tract`; CloudNativePG versus Patroni backup consistency; Untitled UI PRO components in the vendored set; the accuracy of sqlparser's generic dialect for Db2 statements.
- Web search capacity ran out during this research, so some facts rest on direct reads of repositories, registries and documentation pages; each is marked [V], [K] or [S].

---

## 22. Derived requirements

| ID | Requirement | Source |
|---|---|---|
| TS-01 | Adopt the DBS application architecture as AKR-TMS convention (layered crates, bounded contexts, Axum, utoipa, Orval, problem+json, idempotency, jobs and outbox, Cedar, Kratos BFF), and record every deviation as a numbered decision record | Section 1; DBS `CLAUDE.md`, `.claude/rules/` |
| TS-02 | Ship four binaries (`akr-api`, `akr-worker`, `akr-ingestd`, `akr-assess`) from one workspace, with per-binary network policy; only `akr-ingestd` holds CBS credentials or routes | Section 3.1; file 25 section 10 |
| TS-03 | Treat the DBS platform layer as unbuilt: plan K3s, Helm, Harbor, Gitea and observability work; ship the compose profile first and the three-server K3s profile from the same images | Section 1.4, 16; DBS `completion.md`, DR-107 |
| TS-04 | Reach CBS engines through one `SourceReader` port: `tiberius` for SQL Server, `sqlx` for PostgreSQL and MySQL, `oracle` with an `odbc-api` second backend for Oracle, `odbc-api` for Db2 and Sybase; feature-gated crates; bank-supplied client-library volume; no redistribution of Oracle or IBM binaries | Section 4; SQLx README, tiberius and odbc-api documentation |
| TS-05 | Generate CBS SQL from mappings, verify it with a SQL parser as a single read-only SELECT over approved objects, and keep compile-time SQL checking for AKR-TMS's own database only | Section 4.3; file 25 section 14 |
| TS-06 | Run blocking drivers on a bounded blocking pool sized by the signed load budget | Section 4.3; DBS `jobs.md` (`spawn_blocking`); file 25 section 5 |
| TS-07 | Run a driver conformance suite in CI (PostgreSQL, SQL Server, Oracle Free containers; a documented rig for Db2 and Sybase) covering keyset reads, read-only enforcement, error taxonomy and character-set round trips | Section 4.5 |
| TS-08 | Use Debezium Server as the only JVM sidecar, only for agreed Oracle or Db2 CDC, with an HTTP sink (or NATS JetStream) into `akr-ingestd`, a pinned image and an SBOM entry; do not use it where a Rust reader suffices | Section 4.4; Debezium documentation |
| TS-09 | Carry the pipeline on the PostgreSQL jobs and outbox harness; use no Kafka and no Redis queue; treat the raw landing table as the replay log; allow NATS JetStream only as a cross-zone CDC buffer and Kafka only as a consumer-only R4 adapter | Section 5; DBS `tech-stack.md`, `jobs.md` |
| TS-10 | Register the new job kinds with cockpit tiles, concurrency limits and coalescing of per-account evaluations | Section 5.2, 5.4 |
| TS-11 | Build a load-test rig with a synthetic CBS writer and prove the 10x and 100x salary-day cases for each hardware profile | Section 5.5, 16.6 |
| TS-12 | Organise PostgreSQL into the schemas and roles of section 6.1; partition fact tables by month with job-based partition maintenance; implement retention classes as partition drops for time-series and record-level purges for relationship-end clocks, with a hold store that preserves held rows before any drop | Section 6; file 30 section 12; DBS migration 0019, `partition_maintenance` |
| TS-13 | Decide the database collation and ICU locale at `initdb`, verify encoding and Arabic round trips in the installer, launch on PostgreSQL 17 and plan 18 | Section 6.6 |
| TS-14 | Implement scenarios as JSON-Schema-validated documents compiled to SQL from a closed operator set plus CEL for event conditions; no free SQL; bound parameters; statement timeouts; backtests and shadow mode; the CBI catalogue importer | Section 7; file 27 |
| TS-15 | Govern every scenario, threshold, mapping, code-map, fail-policy and role change through a generic maker-checker change request with a backtest or impact summary, a different approver, a fresh second factor and a volume-reduction flag | Section 7.5, 11.4; DBS step-up pattern; file 05 section 1f |
| TS-16 | Train models offline inside the bank (workbench), serve them as ONNX through `ort` with a pinned, vendored ONNX Runtime build (and `tract` as a fallback), implement TreeSHAP in-house with golden tests, keep a signed model registry, run new models in shadow before use, and monitor drift | Section 8; files 21 and 29 |
| TS-17 | Use no external AI API; any language-model feature is optional and served on premises | Section 8.5; DBS ARCHITECTURE section 11 |
| TS-18 | Model networks with edge tables, bounded recursive CTEs and `petgraph`, store graph features in `feat`, and use no graph database; keep Apache AGE optional | Section 9; file 21 section 6 |
| TS-19 | Use one shared Rust normalisation library with a SQL parity test, an in-memory list index for the synchronous path, an own explainable scorer, and pg_trgm only for batch and back-office search; keep the benchmark harness that reproduces the section 10.1 measurements | Section 10; files 22 and 25 |
| TS-20 | Model investigations as a dedicated event-sourced case aggregate and reuse the DBS workflow engine for approval flows; apply the sensitive-text guard to free-text notes | Section 11; DBS workflow engine, `sensitive.rs` |
| TS-21 | Notify only in-app and through the bank's SMTP relay; no WhatsApp or external push; no subject names in notification text | Section 11.5; file 30 section 8 |
| TS-22 | Put the evidence vault behind the S3 port; provision buckets through the S3 API with object lock and COMPLIANCE default retention (no `mc` at run time); run the MinIO code line as a pinned Silo build compiled from source by AKR-TMS and cross-checked against the Chainguard fork; certify every build with the WORM conformance suite (including restore with lock metadata) and a security regression suite with one test per published advisory | Section 12; GitHub advisories, Silo and Chainguard repositories, MinIO object-lock documentation |
| TS-23 | Use OpenBao as the secrets and key service; keep the Vault-compatible HTTP client; prove the DBS key operations against OpenBao with the conformance checklist in CI; accept a bank's own Vault only through the same checklist | Section 13; Vault LICENSE, OpenBao CHANGELOG |
| TS-24 | Define the key hierarchy (unseal, KEK, DEKs, match-key hash key, signing keys, PKI), keep OpenBao out of the hot path, and design the phase-2 service so a node restart does not need a manual unseal (transit or static-key auto-unseal, or the bank's HSM through the PKCS#11 plugin) | Section 13.3, 13.4 |
| TS-25 | Default to standalone Kratos with mandatory strong MFA; support generic OIDC upstream; offer Ory Polis for SAML and SCIM; enforce step-up locally; plan the upgrade from Kratos v1.2.0 and Hydra v2.2.0 | Section 14; Ory documentation |
| TS-26 | Express the AKR-TMS roles as a Cedar bundle with simulation, dry-run and existence masking for restricted tiers; machine clients use scoped tokens or per-gateway mTLS identities | Section 14; DBS `auth.md`, M3 |
| TS-27 | Ship Prometheus, the OpenTelemetry Collector and Jaeger; make Grafana and Loki optional; export audit and security events to the bank's SIEM over syslog or CEF; keep personal data out of logs and spans with a tested redaction filter | Section 15 |
| TS-28 | Define metrics and SLOs as code (ingest lag, reconciliation, paused-by-bank, assessment latency, staleness, queues, backups, OpenBao seal, certificates, object-store advisory status) with cockpit tiles | Section 15.3; DBS M15 |
| TS-29 | Produce a Helm chart and a compose bundle from one source of truth; use a three-server K3s with embedded etcd for the HA profile | Section 16.1; K3s documentation |
| TS-30 | Deliver updates as signed, verified OCI-layout release bundles imported into the bank's Harbor with content trust enforced, an admission policy, an SBOM diff and a rollback plan; never push changes remotely; require the bank's prior approval | Section 16.2, 16.3; CBI 2026 outsourcing controls; Harbor and K3s documentation |
| TS-31 | Run CI gates: format, clippy `-D warnings`, tests on throwaway PostgreSQL, `sqlx prepare --check`, `cargo deny`, `cargo audit`, `cargo-auditable`, CycloneDX SBOMs, image SBOMs, Trivy, signing, OpenAPI and Orval drift checks, frontend lint and tests | Section 16.4 |
| TS-32 | Choose PostgreSQL high-availability tooling for the three-node profile by proof of concept (CloudNativePG or Patroni with pgBackRest); keep the DBS manual failover for the compose profile | Section 16.5 |
| TS-33 | Treat hardware profiles S, M and L as planning assumptions, require a second site, and replace the figures with load-test results | Section 16.6; CBI controls (continuity) |
| TS-34 | Deploy single tenant per bank by default; offer hosted only as cluster per bank at first, with per-tenant keys, backups, exit path, JIT support access and isolation tests; reject a shared database with row-level security | Section 17 |
| TS-35 | Reuse the DBS frontend stack and RTL rules; add TanStack Table with Virtual and keyset pagination, Recharts as the default chart library (after an RTL spike), xyflow for small graphs with Sigma or Cytoscape for large ones, and a bidi-aware name-comparison component; fetch nothing at run time | Section 18 |
| TS-36 | Share the Arabic fold between server and client with a parity test; keep every user-facing string in Arabic and English | Section 18; DBS `fold.ts`, `frontend.md` |
| TS-37 | Maintain a third-party notices file and SBOM per release; ship Valkey and OpenBao, not Redis and Vault; obtain legal sign-off on the AGPL components (MinIO code line, Grafana family, OpenLogReplicator); ship the store's Corresponding Source and notices with every release; brand no build as MinIO; confirm no PRO-licensed UI component is vendored | Section 19; MinIO trademark policy |
| TS-38 | Keep a version policy (pinned majors, quarterly review, MSRV tracking) and keep bank-owned facts out of fixtures (the DBS admin-managed rule) | Section 1.5; DBS `admin-managed.md` |
| TS-39 | Back up with pgBackRest and restic as the DBS does, add object-store and OpenBao snapshot backups, run monthly restore drills and record them in the audit trail | Section 12.6, 16; DBS `backup-restore.md` |
| TS-40 | Require NTP on every node, `TIMESTAMPTZ` for all times, and the bank's CBS calendar as the business calendar authority | File 25 sections 6 and 10; DBS migration rules |
| TS-41 | Front the object store with an S3 guard that passes only the operations AKR-TMS uses and rejects the request shapes behind published advisories (S3 Select, Snowball auto-extract, unsigned-trailer uploads, presigned writes, unsigned `x-amz-*` headers, client `X-Minio-*` headers, admin, STS, console and internode paths); keep the console, identity federation, SFTP and FTP, notifications, replication and the update API off; run the store as a non-root user with egress denied | Section 12.6; MinIO and Silo advisories |
| TS-42 | Give the store three kinds of credential: a writer key without delete (except quarantine) or bucket-level rights, a GET-only presigning key (its ID is public in every URL), and an installer and break-glass admin key held in OpenBao; write policies with explicit bucket and object ARNs and explicit denies on version deletes, governance bypass and bucket-level writes | Section 12.6; Silo SN-2026-004 and SN-2026-010; CVE-2026-40344, CVE-2026-41145 |
| TS-43 | Run an object-store advisory watch: upstream GitHub advisories, the Silo ledger including SN identifiers that are not CVEs, the Chainguard fork's commits, `govulncheck` over the pinned source and Go releases; triage within two working days; ship high and critical fixes in a signed update within 14 days and others at the next monthly release; record each review in the store certification report; rehearse an exit to a second S3 store once a year | Section 12.6, 12.7 |

---

## 23. Sources

DBS repository (read only), under `/home/haithem/Projects/digital-banking-system/`
- `tech-stack.md`, `CLAUDE.md`, `PRODUCT.md`, `DESIGN.md`, `docker-compose.yml`, `Makefile`, `rust-toolchain.toml`
- `docs/ARCHITECTURE.md`, `docs/MODULES.md`, `docs/plan/research.md` (DR-96, DR-97, DR-107 to DR-113), `docs/plan/backup-restore.md`, `docs/plan/completion.md`
- `.claude/rules/` (`ddd.md`, `rust-backend.md`, `auth.md`, `jobs.md`, `admin-managed.md`, `migrations.md`, `frontend.md`)
- `backend/Cargo.toml` and crate manifests; `backend/crates/common/src/objectstore.rs`; `backend/crates/domain/src/sensitive.rs`; `backend/migrations/0003`, `0017`, `0019`; `backend/policies/bundle-001/`
- `deploy/prod/` (`docker-compose.prod.yml`, `README.md`, `vault.hcl`, `pgbackrest.conf`, `jaeger.yaml`, `Caddyfile`, `Dockerfile.postgres`); `.github/workflows/ci.yml`; `frontend/package.json`

Crates, packages and repositories (queried 2026-10-06)
- crates.io API for sqlx, tiberius, odbc-api, oracle, sibyl, sea-orm, ort, candle-core, tract-onnx, tantivy, petgraph, datafusion, polars, duckdb, cel, cel-interpreter, rhai, sqlparser, async-nats, redis, fred, rdkafka, quick-xml, cedar-policy, aws-sdk-s3, rust-s3, strsim, rapidfuzz, unicode-normalization, linfa, smartcore, lightgbm3, xgboost, gbdt, rust_decimal, jsonschema, deadpool-redis, cargo-cyclonedx, cargo-auditable, cargo-deny, cargo-audit, cargo-vet, sigstore: https://crates.io/
- npm registry for @tanstack/react-table, @tanstack/react-virtual, recharts, echarts, @visx/xychart, @xyflow/react, cytoscape, sigma, graphology, @react-sigma/core, reagraph, @antv/g6, vis-network, react-force-graph-2d, uplot, tailwindcss, react-aria-components, i18next, react-i18next, orval, zod, react-hook-form, @untitledui/icons: https://www.npmjs.com/
- SQLx README (MSSQL removed): https://github.com/launchbadge/sqlx
- tiberius README: https://github.com/tiberius-rs/tiberius ; odbc-api README: https://github.com/pacman82/odbc-api ; rust-oracle README: https://github.com/kubo/rust-oracle ; ort documentation (linking): https://ort.pyke.io/setup/linking ; tract: https://github.com/sonos/tract
- Supabase ETL: https://github.com/supabase/etl ; OpenLogReplicator: https://github.com/bersler/OpenLogReplicator ; Debezium releases and Server: https://debezium.io/releases/ , https://debezium.io/documentation/reference/stable/operations/debezium-server.html
- MinIO (archived notice): https://github.com/minio/minio ; mc (archived): https://github.com/minio/mc
- MinIO security advisories, releases, commits and forks (GitHub API, 2026-10-07): https://api.github.com/repos/minio/minio/security-advisories , https://api.github.com/repos/minio/minio/releases , https://api.github.com/repos/minio/minio/commits , https://api.github.com/repos/minio/minio/forks ; README and LICENSE (AGPL text): https://raw.githubusercontent.com/minio/minio/master/README.md , https://raw.githubusercontent.com/minio/minio/master/LICENSE , https://raw.githubusercontent.com/minio/minio/master/COMPLIANCE.md ; call-home and update code at the last tag: https://github.com/minio/minio/blob/RELEASE.2025-10-15T17-29-55Z/internal/config/callhome/callhome.go , https://github.com/minio/minio/blob/RELEASE.2025-10-15T17-29-55Z/cmd/update.go
- MinIO KES and operator (archived): https://github.com/minio/kes , https://github.com/minio/operator ; Docker Hub API: https://hub.docker.com/v2/repositories/minio/minio/
- AIStor Free Tier License Agreement: https://www.min.io/legal/aistor-free-agreement ; MinIO trademark policy: https://www.min.io/legal/trademark-policy ; download page: https://www.min.io/download ; Cohasset assessment: https://www.min.io/cohasset/introduction
- Silo (maintained fork): https://github.com/pgsty/silo , releases https://github.com/pgsty/silo/releases ; manifesto, licence, trademark notice, security advisories: https://silo.pgsty.com/about/manifesto/ , https://silo.pgsty.com/about/license/ , https://silo.pgsty.com/about/trademark/ , https://silo.pgsty.com/about/security-advisories/ ; object locking: https://silo.pgsty.com/administration/object-management/object-retention/
- Chainguard fork (EmeritOSS): https://github.com/chainguard-forks/minio ; other forks: https://github.com/libreFS/libreFS , https://github.com/kypello-io/kypello , https://github.com/juicedata/minio , https://github.com/openminio/openminio
- GPL FAQ on aggregation (gnu.org unreachable; Wayback copy): https://web.archive.org/web/2026/https://www.gnu.org/licenses/gpl-faq.html#MereAggregation
- OpenBao CHANGELOG: https://github.com/openbao/openbao/blob/main/CHANGELOG.md
- SeaweedFS README and Object Lock wiki: https://github.com/seaweedfs/seaweedfs , https://github.com/seaweedfs/seaweedfs/wiki/S3-Object-Lock-and-Retention ; RustFS README: https://github.com/rustfs/rustfs
- Vault licence: https://github.com/hashicorp/vault/blob/main/LICENSE ; OpenBao: https://github.com/openbao/openbao ; Vault database secrets engine: https://developer.hashicorp.com/vault/docs/secrets/databases
- Redis licensing: https://github.com/redis/redis ; Valkey: https://github.com/valkey-io/valkey
- Ory Kratos generic provider: https://www.ory.com/docs/kratos/social-signin/generic ; Ory Polis: https://github.com/ory/polis ; Kratos and Hydra: https://github.com/ory/kratos , https://github.com/ory/hydra
- K3s air-gap and HA documentation: https://docs.k3s.io/installation/airgap , https://docs.k3s.io/datastore/ha-embedded ; K3s: https://github.com/k3s-io/k3s
- Harbor content trust: https://goharbor.io/docs/main/working-with-projects/working-with-images/sign-images/ ; Harbor: https://github.com/goharbor/harbor ; Hauler: https://github.com/hauler-dev/hauler ; cosign: https://github.com/sigstore/cosign ; Syft: https://github.com/anchore/syft ; Trivy: https://github.com/aquasecurity/trivy ; Kyverno: https://github.com/kyverno/kyverno ; sigstore policy-controller: https://github.com/sigstore/policy-controller
- CloudNativePG: https://github.com/cloudnative-pg/cloudnative-pg ; Patroni: https://github.com/patroni/patroni ; pgBackRest: https://github.com/pgbackrest/pgbackrest ; restic: https://github.com/restic/restic ; PGroonga: https://github.com/pgroonga/pgroonga ; pg_partman: https://github.com/pgpartman/pg_partman ; Apache AGE: https://github.com/apache/age ; ClickHouse: https://github.com/ClickHouse/ClickHouse
- Gitea: https://github.com/go-gitea/gitea ; Gotenberg: https://github.com/gotenberg/gotenberg ; Prometheus, OpenTelemetry Collector, Jaeger, Grafana, Loki, Tempo, Caddy, cert-manager: GitHub release pages of each project
- NATS server: https://github.com/nats-io/nats-server ; async-nats: https://github.com/nats-io/nats.rs
- PostgreSQL pg_trgm manual: https://www.postgresql.org/docs/17/pgtrgm.html
- Untitled UI React (open repository): https://github.com/untitleduico/react

Measurements made for this file
- pg_trgm on Arabic names, PostgreSQL 17.11 Alpine container, 2026-10-06: scripts kept in the research scratchpad (`trgm-test*.sql`); synthetic data; results in section 10.1.

Companion research files: 01 (section 8), 03, 04, 05, 14, 17, 18, 21, 22, 23, 25, 27, 29, 30, 31.
