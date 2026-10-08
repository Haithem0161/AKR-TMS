# AKR-TMS — System Planning Documentation

AML and KYC risk and transaction-monitoring system for Iraqi banks (conventional and Islamic, private and
state-owned). Arabic-first, right to left, one installation per bank on the bank's own servers, a read-only
connection to the bank's core banking system (CBS). This directory is the system plan: nothing in code should
contradict it, and when reality wins an argument the document is updated in the same change.

Method and stack follow the digital-banking-system repository (DBS, read-only reference at
`/home/haithem/Projects/digital-banking-system`): the DBS planning set (`docs/`), its phase-file methodology
(`.claude/rules/planning.md`, ported here as [../.claude/rules/planning.md](../.claude/rules/planning.md)) and its stack, with the
three changes decided on 2026-10-07: Valkey instead of Redis, OpenBao instead of Vault, MinIO kept as a pinned
maintained build (Silo).

Planning started 2026-10-07. Requirements come from the research set in [../research/](../research/) (files 01-33;
file 33 is the synthesis). The leadership plan for banks is a separate deliverable in [../plan/](../plan/) (PDF,
English and Arabic) and is not changed by this directory.

## The documents

Written in this order (the DBS order). Status: Planned, Draft, Done.

| Doc | What it holds | Status |
|---|---|---|
| [../tech-stack.md](../tech-stack.md) | Stack choices, binaries, server profiles (the founding doc) | Done |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Locked decisions, components, zones, data flow from the CBS, engines (scenarios, rating, screening, scoring), case model, authorisation, audit and evidence, identity, notifications, i18n, backup and DR, SLOs, security, build phases, open items | Done (draft for review) |
| [MODULES.md](MODULES.md) | Modules M1-Mn: features by phase, flows, API sketches, screens, edge cases; frontend architecture; end-to-end walkthroughs; module x phase matrix; decision queue | Done (draft for review) |
| [DATA-MODEL.md](DATA-MODEL.md) | Table-by-table schema plan: conventions, all domains, canonical model, partitions and retention classes, grants, sizing, migration order | Done (draft for review) |
| [DSL.md](DSL.md) | The declarative documents: scenario DSL v1, mapping specification, rating model, change requests, approval workflows (DBS workflow DSL reused); validation error catalogue; complete examples | Done (draft for review) |
| [API.md](API.md) | API conventions (auth, step-up, existence masking, errors, idempotency, If-Match, cursors, data_as_of, affordances, SSE) + scopes + endpoint catalogue per module (about 366 endpoints) + workflow tasks; the phase-2 assessment API; problem codes | Done (draft for review) |
| [DESIGN-SYSTEM.md](DESIGN-SYSTEM.md) | The locked AKR-TMS identity (../plan/identity.md) as tokens with measured contrast, type scale, Tailwind v4 mapping, shared components, RTL rules, screen catalogue (61 screens) and key screen specs, accessibility, wording | Done (draft for review; dark palette proposed, needs owner approval) |
| [ROADMAP.md](ROADMAP.md) | Macro-phases P0-P3 with task checklists (P0 in two tracks: vendor build with the two early pieces first, and per-bank discovery), exit criteria, cross-phase invariants, risk register, external dependencies | Done (draft for review) |
| [plan/](plan/README.md) | Implementation phase files: roadmap (32 phases 00-31, dependency graph, entities, services, migration chain), status tracker, decisions log (DR-01..DR-26) and research queue, phase template, phase specs | Scaffolding done; phase 00 written (2026-10-07); 01-31 to write |

Macro-phases follow the leadership plan (section 11): **P0** discovery and foundations (including the two early
pieces decided 2026-10-07: the CBI coverage gap report and the goAML checker), **P1** core system for the first bank
(ends with a two-to-three-month parallel run), **P2** checks before payments are released, validated models and the
assistant, **P3** scale to further banks.

## Non-negotiables

1. **Read-only toward the CBS, and provably so.** No writes, triggers, jobs or stored code in the CBS; grants are
   checked on every connect and the connection fails closed on any write privilege; the CBS is never queried on
   demand by customer or case. Actions become tasks for bank staff.
2. **Bank data never leaves the bank.** One installation per bank on the bank's assets; only content services
   (lists, typology packs, signed release bundles) are hosted, and they carry no bank data; no external AI API.
3. **People decide.** A score alone never rejects; at most it holds. Reports, restrictions and exits need a named
   human decision; four-eyes is enforced by policy and by a database constraint.
4. **Tipping-off safe by construction.** A suspicion tier with per-case access lists and existence masking;
   neutral wording on every task and notice; no AML reason ever written to the CBS; no external message channel.
5. **The audit trail is append-only, hash-chained, anchored and separate from operational logs.** AML records
   follow the strictest retention (Iraq floor seven years); the audit trail itself is kept without limit.
6. **Never tuned to staff capacity; unknown is never low.** No parameter caps alerts; a missing value scores high
   or falls back with a "degraded" flag.
7. **Change is governed.** One maker-checker change request covers scenarios, thresholds, mappings, code maps,
   fail policies and roles; every alert carries the versions that produced it.
8. **Arabic is the original.** Status is never colour alone; Arabic and English on every string; logical CSS only;
   Western digits in data.
9. **Every mutation:** Idempotency-Key + If-Match + audit event.
10. **OpenBao never sits in the request hot path; a streaming replica is not a backup.**

## Next action

Write docs/plan/phase-01.md, then 02-08 one by one (phase 00 written 2026-10-07; the owner chose inline writing for P0, then a new question before 09-31). Progress and the resume point are kept in
[../research/_progress.md](../research/_progress.md).
