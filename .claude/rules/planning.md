---
paths:
  - "docs/**"
  - "**/roadmap.md"
  - "**/phase-*.md"
  - "**/status.md"
  - "**/*VERIFICATION*"
---

# Development Plan Writing Rules

Ported from the digital-banking-system repository (`.claude/rules/planning.md`, read 2026-10-07) and adapted to
AKR-TMS. Differences from the DBS rule are marked "AKR".

For non-trivial features, write a structured plan before implementing. Plans live in `docs/plan/` and consist of the
files below. The system plan (docs/ARCHITECTURE.md, MODULES.md, DSL.md, DATA-MODEL.md, API.md, DESIGN-SYSTEM.md,
ROADMAP.md) is the source of requirements; phase files translate it into copy-paste-ready implementation specs. When
a phase file and a planning doc disagree, resolve the conflict in the same change; never let them drift silently.

AKR: behind the system plan sits the research set (`research/01-33`), whose requirement IDs (`C-02`, `S-01`,
`RP-55`, ...; 1,208 unique IDs, counted as unique `PREFIX-NN` excluding FM, FL, TR, G and IR) are the coverage
baseline. Every module in MODULES.md and every phase file names the requirement IDs it carries, so a gap pass can
check coverage mechanically.

## Plan Structure

| File | Purpose |
|------|---------|
| `roadmap.md` | Master blueprint: phase table, dependency graph, entity/service inventories, gap-analysis log |
| `research.md` | Implementation research + dated decisions log (DR-NN: date / decision / rationale) |
| `phase-XX.md` | Individual phase specs, the core deliverable (template: `phase-template.md`) |
| `status.md` | Living tracker: phase status table, cumulative totals, blockers |
| `PHASES-X-Y-VERIFICATION.md` | Verification reports with YAML frontmatter (score, status, per-truth pass/fail) |

The frontend consumes the backend through the Orval-generated client: any phase that changes the API updates the
OpenAPI registration and regenerates the client in the same phase, never batched.

## Roadmap.md Sections (in order)

1. **Header**: title, start date, target description, scope with hard numbers (tables, routes, services, jobs,
   binaries)
2. **Phase Overview Table**: #, Phase Name, Scope, Size (S/M/L/XL), Macro-phase (ROADMAP.md P0-P3), Depends On,
   Status
3. **Dependency Graph**: ASCII art showing phase relationships and parallel tracks
4. **New Entities by Phase**: phase to new database tables (must reconcile with DATA-MODEL.md)
5. **New Services by Phase**: phase to new domain services
6. **Gap Analysis Additions**: running log updated after each pass (count, categories, distribution)

## Phase File Template (SQLx/Axum, DBS conventions)

Every `phase-XX.md` MUST have these sections in this exact order (starter: `docs/plan/phase-template.md`):

### Header
```
# Phase N: <Name>

**Goal:** <One sentence describing what this phase delivers>

**Dependencies:** Phase X, Phase Y (or "None")
**Macro-phase:** P0 | P1 | P2 | P3   (from docs/ROADMAP.md)
**Complexity:** S | M | L | XL
**Binaries touched:** akr-api | akr-worker | akr-ingestd | akr-assess
**Requirements carried:** <research IDs, e.g. C-02, C-03, S-01>
```

### Section 1: SQLx Migration Changes
- New tables: full SQL migration blocks, copy-paste ready
- Modified tables: ALTER TABLE statements with exact column definitions
- New enum types: `DO $$ BEGIN CREATE TYPE ... END $$;` blocks
- Indexes: `CREATE INDEX IF NOT EXISTS` statements
- All SQL idempotent (`IF NOT EXISTS`); `TIMESTAMPTZ` (never `TIMESTAMP`)
- **Ids are UUIDv7 generated app-side** (DATA-MODEL.md conventions): `id UUID PRIMARY KEY` with no DB default; do
  not use `gen_random_uuid()`
- Every table names its schema, its owning role and its retention class (DATA-MODEL.md conventions) (AKR)
- Audit tables follow DATA-MODEL's audit section exactly (partitioning, grants, no UPDATE/DELETE)

### Section 2: DDD Implementation (Rust)
- **Domain entity**: struct definition with derives (`Debug, Serialize, Deserialize, FromRow, ToSchema`)
- **Create/Update structs**: with `Deserialize, ToSchema` derives, Update fields all `Option<T>`
- **Repository trait**: method signatures with parameter types (domain layer)
- **Repository impl**: `PgXxxRepository` notes on joins, RETURNING usage (infrastructure layer)
- **utoipa schemas**: list of request/response DTOs and their purpose
- **Route table**: `| Method | Path | Scope | Description |` format (scope = API.md scopes section)
- **Router function**: `pub fn router() -> Router<AppState>`
- Layer rules per `ddd.md`; crate placement per the workspace layout pinned in phase-00

### Section 3: Business Logic
- Domain service structs with method signatures and trait bounds
- `thiserror` error enum variants for this domain (+ problem+json `code` values)
- Step-by-step logic for each method (numbered steps, not prose)
- **Audit events emitted** (kind + resource): every mutation names its audit event here; a mutation without one is
  a plan gap by definition
- **Cedar actions touched**: new/changed policy actions and the attributes they evaluate
- **Tipping-off review** (AKR): for every screen, task, notice, export or CBS-facing artefact the phase adds, state
  what a person outside the case's access list can learn from it (the answer must be "nothing")
- Configuration and settings used (DB `settings` keys or env); anything the bank may want different next month is
  admin-managed data (`admin-managed.md`), not a constant

### Section 4: Infrastructure Updates
- Postgres `jobs` kinds / outbox events / timers introduced (or "No new jobs"); see `jobs.md`
- Docker compose changes (or "No infrastructure changes needed")
- AppState additions (new repos, services)
- OpenAPI registration (paths + schemas + tag)
- Network zone rules touched (which binary may reach which service) (AKR)
- Existing entity/schema/route updates required by this phase

### Section 5: Frontend Impact
- Screens/components added or changed (reference DESIGN-SYSTEM.md specs and the screen catalogue IDs)
- Generated-client surface consumed (hooks used)
- Or "Backend-only phase — no frontend impact"

### Section 6: Verification
Numbered checklist of concrete steps:
1. `make migrate`: migrations apply without error
2. `cargo check`: compile-time SQL verification passes
3. `cargo clippy -- -D warnings`: no lint errors
4. `cargo test`: all tests pass
5. curl-test each endpoint (specific examples with expected responses, including a 403 Cedar-deny case, a 404
   existence-masking case for suspicion-tier resources and an idempotency-replay case where applicable)
6. Swagger UI shows the new endpoints; `make generate-api` regenerates the frontend client cleanly
7. Audit trail shows the expected events for the curl session
8. For phases that touch the CBS connection (AKR): the read-only conformance suite passes (a source granted any
   write privilege is refused at connect; no statement other than SELECT reaches the source; the load budget is
   enforced)
9. Run existing tests: no regressions

### Section 7+: Gap Additions
Appended by gap-analysis passes. Numbered subsections (7.1, 7.2, ...) each containing migration additions (exact
SQL), new route tables, business-logic additions, and a reference to the gap ID and severity.

### Trailer: What This Phase Does NOT Touch
Explicit list of what the phase leaves alone ("No CBS connection", "No new jobs", "No changes to Phase N entities").

---

## Gap Analysis Methodology

Iterative passes are required for large plans. Do not mark a plan "ready for implementation" until a verification
pass finds 0 true gaps.

### Pass 1 (Initial)
After phase files are written, compare every requirement (entity, field, endpoint, business rule, state machine,
event, integration point, sourced from MODULES.md, DSL.md and the research requirement IDs) against the phase specs.
Log each gap with:
- **Severity:** CRITICAL / HIGH / MEDIUM / LOW
- **Category:** Missing Table, Missing Fields, Missing Endpoint, Missing Logic, Missing Report, Missing Integration,
  Missing Event, Missing Audit Event, Missing Policy, Missing Requirement (AKR), Incomplete Coverage
- **Target phase** for incorporation

Append gaps to the respective phase files as Section 7.x subsections. Update the `roadmap.md` gap log.

### Pass 2+ (Iterative)
Re-compare updated phase files against the requirements. Commonly missed: state machines, field completeness,
integration points, config endpoints, report drill-downs, cross-cutting engines, audit coverage, authz coverage,
retention classes, deadline clocks. Continue passes until one finds 0 true gaps.

### Adversarial correctness pass (required for decision-adjacent and deadline-adjacent phases)
Coverage passes confirm requirements are *present*; this pass attacks whether the *logic survives concurrent and
adversarial inputs*. AKR targets: four-eyes races on case decisions and report approval, claim races on alerts,
alert fingerprint collisions and duplicate alerts on replayed batches, late data amending alerts already decided,
watermark gaps after a CBS restore or failover, freeze and report clocks firing on a closed case, a change request
approved by its own maker, existence masking leaking through counts, search or exports, replayed idempotency keys,
audit-chain gaps. Verify each candidate flaw independently before logging it; resolve confirmed ones as locked
decisions in `research.md` + fixes in the phase files.

### Verification Pass (Final)
Audit N representative items across all phases (mix of severities). For each, verify it exists in the phase files
with complete SQL migration, route table entry, repository trait method, business-logic description, and
audit-event coverage.

Report as YAML frontmatter in `PHASES-X-Y-VERIFICATION.md`:
```yaml
---
phase: <plan-name>-phases-X-Y
verified: <ISO timestamp>
status: complete | gaps_found
score: N/M must-haves verified
gaps: [...]
---
```

## Status.md Sections

1. **Phase Status Table**: #, Phase, Status, Started, Completed, Tables Added, Tables Modified, Routes Added,
   Services Added
2. **Cumulative Totals**: Metric, Before, Current, Target
3. **Jobs by Phase**: job kind, phase, schedule/trigger
4. **Gap Analysis Summary**: per-pass summary with counts and category breakdown
5. **Blockers & Notes**: critical dependencies (external items from ROADMAP.md), parallel-track notes

## Rules

- SQL migrations MUST be copy-paste ready: exact column names, types, constraints, indexes
- Route tables MUST use `| Method | Path | Scope | Description |` format
- Service specs MUST include method signatures and numbered step-by-step logic
- Every mutation MUST name its audit event; every endpoint MUST name its Cedar action
- Every phase MUST name the research requirement IDs it carries (AKR)
- Each phase MUST state what it does NOT touch
- Verification steps MUST be concrete (specific commands, specific assertions), never vague
- Phase sizes: S (<5 routes), M (5-15 routes), L (15-25 routes), XL (25+ routes or complex engines)
- Gap severity: CRITICAL = blocks other phases, HIGH = missing core functionality, MEDIUM = missing enhancement,
  LOW = nice-to-have
