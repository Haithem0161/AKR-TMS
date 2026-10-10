# AKR-TMS — Tech Stack

The founding document of the system plan. It follows the digital-banking-system (DBS) stack and conventions, with the
three changes decided on 2026-10-07 (Valkey for Redis, OpenBao for Vault, MinIO kept as a pinned maintained build).
Evidence, versions and licences are in research file 32; this file records the choices. Versions are the planning
baseline read in October 2026; phase 0 pins them.

**Targets.** One installation per bank on the bank's own servers, inside its network. Read-only toward the core
banking system (CBS). Planning load: up to about 2 million accounts and 1 million postings a day for a large private
bank, more for a state bank (sized by load test). Analyst screens: queue and case first content within 1 s at 50
concurrent analysts, type-ahead p95 300 ms (UX-16). Phase-2 assessment API: p95 150 ms, p99 400 ms, inline name
matching under 25 ms at p99 (C-56, N-50). Data freshness per class F0-F5 with `data_as_of` on every screen (C-29).

## Backend

| Component | Choice | Notes |
|---|---|---|
| API framework | Rust + Axum 0.8 | As the DBS: `utoipa` + `utoipa-axum` (`OpenApiRouter`, `routes!`), problem+json errors, idempotency with byte-exact replay, `If-Match` |
| Service shape | Modular monolith, four binaries over shared crates | `akr-api` (analyst, MLRO, admin, reports; BFF), `akr-worker` (jobs), `akr-ingestd` (the only process that reaches the CBS), `akr-assess` (phase-2 assessment API). Separate binaries for security zoning, not scale; no inter-service RPC (they meet in PostgreSQL and the object store). DBS decision D14 kept in spirit |
| DB access | SQLx 0.8, compile-time checked (`query!`, `query_as!`, offline `.sqlx`) | For AKR-TMS's own database. **One written exception:** CBS reads are generated from the mapping, parsed with `sqlparser` to prove a single read-only `SELECT` over approved objects, and tested against fixtures (the CBS schema belongs to the bank) |
| Primary DB | PostgreSQL 17, primary + standby | Schemas `raw`, `canon`, `feat`, `scn`, `alert`, `casework`, `kyc`, `ref`, `rpt`, `assess`, `audit`, `admin`; roles `akr_app`, `akr_ingest`, `akr_assess`, `akr_audit_writer`, `akr_readonly`, `akr_migrate`; monthly range partitions; collation and ICU locale fixed at `initdb` and verified for Arabic |
| CBS drivers | One `SourceReader` port, feature-gated adapters | `sqlx` (PostgreSQL), `tiberius` (SQL Server), `oracle` crate with an `odbc-api` second backend (Oracle), `odbc-api` (Db2, Sybase). Vendor client libraries are the bank's, mounted read-only, never redistributed. Blocking calls on a bounded pool sized to the bank's signed load budget. Oracle 11.2 sources are read through OCI 19 (Instant Client 19; 21 and later need 12.1 or later) with client character set AL32UTF8, the Arabic round trip checked at connect (DR-36). A source may have no optimizer statistics (the discovery database ran rule-based plans), so reads are single-table key-range SELECTs and joins run in the AKR-TMS store (DR-40) |
| Change data capture | Only where the bank agrees | Debezium Server as the one JVM sidecar (Oracle, Db2), sinking over HTTP or NATS JetStream. No Kafka. A bank's own Kafka can be read as a source, never written |
| Queue and pipeline | PostgreSQL `jobs` (`FOR UPDATE SKIP LOCKED`) + transactional outbox + timers | The DBS harness. The raw landing table is the replay log. No message broker in the default design |
| Cache | Valkey 9 (BSD-3) | Replaces Redis (licence). Cache, rate limits and SSE fan-out only; never a queue |
| Object storage | MinIO, built from a pinned Silo release (AGPL-3.0), Chainguard's fork as second source | Kept by the owner's decision (2026-10-07). Behind an S3 guard; three credential kinds; COMPLIANCE object lock for evidence and audit anchors; data encrypted before upload with OpenBao-wrapped keys (no KES); WORM and security regression suites on every build; advisory watch. SeaweedFS and RustFS are tested exits behind the same S3 port |
| Scenario engine | Versioned JSON scenario documents (JSON Schema, `jsonschema` with no remote refs) | Aggregate logic from a closed operator set compiled to parameterised SQL over canonical partitions; event logic in CEL; profile logic over feature tables. No free SQL. Detail in docs/DSL.md |
| Screening | One Rust normalisation library + immutable in-memory index + Rust scorer | Same library for ingestion, batch screening and the assessment API, with a SQL twin kept in parity; `pg_trgm` for analyst search only |
| ML scoring | ONNX Runtime through `ort` | Models trained offline inside the bank, served as ONNX; gradient-boosted trees with monotonic constraints by default; TreeSHAP implemented in house with golden tests. Models rank rule alerts only in phase 1 |
| Language model runtime (optional) | llama.cpp server (`llama-server`) with one signed GGUF model, on premises | Phase 28 assistant pack, off by default. Own zone without egress, reached by the worker only; CPU by default, GPU an optional profile; a second `llama-eval` server for verification and evaluation; weights on a read-only volume, never downloaded at run time; release tag pinned at the phase 00 smoke test; the model is chosen by the bank's golden evaluation. No external AI API |
| Graph | PostgreSQL edge tables + recursive CTEs + `petgraph` | No graph database |
| Documents and PDF | Gotenberg (custom image with Arabic fonts), clamd | As the DBS: report packages and board packs rendered from HTML templates; antivirus on every upload |
| Regulatory output | goAML XML generated and validated against the Office's published XSD (profile registry, never hard-coded) | File 23 |

## Auth

| Component | Choice | Notes |
|---|---|---|
| Identity | ORY Kratos (standalone by default) | Mandatory MFA (WebAuthn preferred, TOTP minimum); DBS offboarding flow; quarterly access review. Generic OIDC upstream as configuration (Entra ID, ADFS, Keycloak); Ory Polis optional for SAML and SCIM (phase 31: Polis is the OIDC upstream for a SAML IdP, flag `pack.polis`; SCIM creates no account and a deactivation disables, never offboards; own database, admin interface off) |
| Tokens / OIDC | ORY Hydra | Also the OIDC provider for platform tools (Gitea, Grafana) |
| Browser sessions | BFF in `akr-api` | As the DBS: Kratos browser flows, CSRF bound to the session |
| Step-up | AAL2 as a single-use ledger row | Enforced locally even with federated login: opening suspicion-tier records, decisions, approvals, exports |
| Authorisation | Cedar, embedded, deny by default | Two gates (token scope, then policy); roles + attributes (branch scope, clearance tier, case access list); segregation-of-duties pairs as `forbid` rules; existence masking for denied suspicion-tier resources; every decision audited; simulator and dry-run |
| Machine identities | mTLS client certificates (assessment API, one per gateway); scoped personal access tokens for scripts; Hydra client credentials internally | No shared service accounts |

## Frontend

| Need | Choice |
|---|---|
| Web app | React 19 + TypeScript, TanStack Router and Query, Orval-generated client (never hand-written API types), Tailwind CSS v4, react-aria components, i18next |
| Language and direction | Arabic first, right to left, mirrored for English; logical CSS only; Western digits in data; Hijri dates for display and entry only |
| Visual identity | The locked AKR-TMS identity (plan/identity.md, 2026-10-07): Lapis, Ishtar Blue, Glaze Teal, Babylon Gold on Clay; IBM Plex Sans Arabic, Plex Sans, Plex Mono; WCAG 2.2 AA screen tokens. Detail in docs/DESIGN-SYSTEM.md |
| Dense lists | TanStack Table + TanStack Virtual, server-side keyset pagination |
| Charts | Recharts by default (ECharts only for canvas performance), after an RTL spike |
| Network views | `@xyflow/react` for ego graphs; Sigma or Cytoscape behind a "large graph" switch |
| Forms and workbenches | DBS dynamic form renderer, react-hook-form + Zod; mapping and scenario editors are schema-driven |
| Branch window | A separate light bundle (about 300 KB) showing neutral statuses only |
| Mobile | Not in scope |

## Infra / Ops

| Component | Choice | Notes |
|---|---|---|
| Containers | Docker images, one set for both profiles | Same images for compose and K3s |
| Profile S: compose bundle | One server + a backup server; Caddy edge; bootstrap, release and rollback scripts | The DBS production pattern; pilots and smaller banks |
| Profile M/L: K3s + Helm | Three K3s servers with embedded etcd (Traefik and ServiceLB disabled, Caddy stays the edge, WireGuard flannel); default-deny NetworkPolicies rendered from `deploy/zones.yaml`; cert-manager from the bank's CA; Kyverno `verifyImages` admission (sigstore policy-controller the alternative); one catalogue renders compose and the chart | Banks that need the phase-2 API highly available; automated PostgreSQL failover with strict synchronous commit (Patroni + pgBackRest the provisional default, CloudNativePG only if it wins the proof of concept; phase 30 DR-320) |
| Secrets / KMS | OpenBao (MPL-2.0) | Replaces Vault (licence). Shamir unseal or auto-unseal (second OpenBao, bank HSM through PKCS#11); transit KEK; DEKs per table family cached in process; HMAC match-key hashes; Ed25519 signing keys for anchors, certificates, decisions and packages; never in the request hot path |
| Encryption | TLS everywhere, mTLS between zones; AES-256 at rest; field-level envelope encryption for personal data | |
| Network zones | Z0-Z7, default deny | `akr-ingestd` alone in the CBS-facing zone with the read-only credentials; `akr-assess` in the gateway zone |
| Metrics | Prometheus + Alertmanager | `/metrics` on every binary |
| Traces | OpenTelemetry Collector + Jaeger | As the DBS |
| Logs | JSON to stdout, forwarded to the bank's SIEM (syslog / CEF) | Loki and Grafana optional (AGPL, unmodified, with notices) |
| Operators' first view | In-app ops cockpit | Job queues, dead jobs, ingest lag, coverage, backups, anchors, certificates |
| Backup | pgBackRest (WAL archiving every minute) + restic for buckets and OpenBao snapshots | Offsite copy; monthly restore drill; a streaming replica is not a backup |

## Server profiles (planning assumptions; discovery replaces them)

| Profile | Nodes | Indicative size | For |
|---|---|---|---|
| S | 1 server + 1 backup server | 16 cores, 64-128 GB RAM, 2-4 TB NVMe | Up to about 300,000 accounts and 100,000 postings a day |
| M | 3 nodes, one at a second site | 24-32 cores, 128-256 GB RAM each, NVMe | Up to about 2 million accounts; phase-2 HA |
| L | 3 application + 3 data nodes, GPU optional | By load test | State-bank scale, salary-day peaks |

Every bank needs a second site in either profile (CBI continuity controls). Failover is manual on profile S (promote,
flip DNS) and said so to the bank; automated on M and L.

## CI/CD and release

| Component | Choice |
|---|---|
| Git hosting + CI (vendor) | Gitea + Gitea Actions |
| Registry | Harbor (vendor and bank), Trivy scanning, content trust enforced |
| Signing and supply chain | cosign with a vendor key in KMS/HSM (no keyless signing offline); CycloneDX SBOMs (`cargo-cyclonedx`, Syft); provenance; `cargo-auditable` |
| Delivery | Signed OCI-layout release bundle (images, charts, adapter packs, migrations, docs, SBOMs, signatures) carried in by a bank-approved route, imported into the bank's Harbor, approved by the bank's change board, then Helm upgrade or the compose script. Nothing is pushed remotely |

### Pipeline stages

**Rust backend**
```
1. Lint/format   -> cargo fmt --check, cargo clippy -D warnings
2. Policy        -> cargo deny (advisories, licences, bans), cargo audit
3. Test          -> cargo test (throwaway PostgreSQL), sqlx prepare --check,
                    read-only conformance suite (drivers), golden name-normalisation file
4. Build         -> cargo build --release (cargo-auditable)
5. Image         -> docker build, SBOM, Trivy scan, cosign sign, push to vendor Harbor
6. Bundle        -> release bundle with SBOM diff, test evidence and rollback plan
```

**React (web)**
```
1. Lint/type check -> eslint, tsc --noEmit, OpenAPI/Orval drift check
2. Test             -> vitest, response-budget and glyph-coverage tests, Playwright with axe-core (every route,
                       both languages; DR-77), token contrast check, design lint
3. Build            -> vite build -> static assets -> image -> bundle
```

## Licences

Everything shipped is MIT, Apache-2.0, BSD, MPL-2.0 or the PostgreSQL licence, except three AGPL-3.0 components
that run as separate, unmodified programs: the MinIO code line (shipped with its Corresponding Source, LICENSE,
NOTICE and CREDITS, renamed per MinIO's trademark policy), and the optional Grafana and Loki. Counsel signs off on
the AGPL reading before the first bank. A third-party notices file and an SBOM ship with every release. Redis and
Vault no longer ship. Oracle and IBM client libraries are supplied by the bank and never redistributed.

## Open items to revisit

- Pin every version in phase 0 (cargo workspace, images, frontend packages); quarterly dependency review.
- Kratos and Hydra: start on the current calendar releases (the DBS runs v1.2.0 and v2.2.0).
- Patroni + pgBackRest versus CloudNativePG for automated failover (proof of concept).
- Chart and graph libraries in RTL (spike before choice).
- OpenBao parity with the DBS's Vault client calls (expected through the shared API; prove in phase 0).
- The Silo build under the WORM and security regression suites; SeaweedFS and RustFS as exits under the same suites.
- `ort` is still a release candidate; `tract` as fallback for small models; ONNX-ML tree operator coverage unverified.
- sqlparser's generic dialect for Db2 statements.
- Oracle change capture (Debezium Server): Debezium's Oracle connector does not support 11g (research/25), and the
  discovered CBS is Oracle 11.2 in ARCHIVELOG mode as a single primary; supplemental logging and a standby were not
  examined. Polling (DR-37) is the plan for 11.2 sources; prove capture read-only before it is offered for Oracle.
- Hardware sizes from each bank's discovery data and a load test on the target hardware.
