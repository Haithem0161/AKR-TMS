---
paths:
  - "docs/plan/**"
  - "backend/**"
  - "frontend/**"
  - "deploy/**"
---

# Implementation Flow: STRICT Rules

Ported from the digital-banking-system repository (`.claude/rules/implementation-flow.md`, read 2026-10-11) and
adapted to AKR-TMS. Differences from the DBS rule are marked "AKR".

How the phases get built. These are hard rules, not suggestions.

## Build approach

- **AKR: 32 phases, 00 to 31, on this dev machine, in the order of the dependency graph in
  `docs/plan/roadmap.md`.** A phase is not started until every phase it depends on is Done in `docs/plan/status.md`.
  Deployment to a bank comes later; build locally on the compose profile.
- **VERTICAL slices.** A phase delivers its backend AND the screens its Frontend Impact section names, together.
  AKR: phases 00 to 07 are platform phases; screens start with phase 08.
- **`main` is ALWAYS green.** Every commit compiles (`cargo check`), passes `make lint` (clippy with warnings as
  errors) and passes the FULL test suite. A red phase is never committed.
- **AKR: commits are made only when the owner asks**, on `main`, with paths staged one by one (never `git add -A`).
  No emojis. A request to commit covers that commit and no later one.
- **AKR: the phase file is the specification.** Code uses the plan's names exactly (tables, columns, routes,
  settings keys, job kinds, audit event kinds, metric names, error codes).

## The per-phase loop

1. **Weave.** Read the phase file whole, with its `7.x` gap additions. AKR: a decision in `research.md` (DR-NN) wins
   over a 7.x block, and a 7.x block wins over the base text.
2. **Research.** Check the current release and API of every crate and package the phase introduces, and pin it:
   one line in `[workspace.dependencies]` or an exact version in `frontend/package.json`, recorded under RQ-01 in
   `research.md`.
3. **Schema.** Write the migration from the phase file's SQL; `make migrate` (twice: the second run applies nothing);
   `make check`; `make prepare`.
4. **Implement (DDD order, per `ddd.md`):** entities, then repository traits (domain), then sqlx impls (db), then
   services, then utoipa handlers (api), then wire `AppState`, then register the routes.
5. **Frontend (phases with screens):** `make generate-api` (Orval), then build the screens per `DESIGN-SYSTEM.md`
   against the generated hooks. NEVER hand-write API types or hooks.
6. **Test: the full `testing.md` regime.** Deep, real, adversarial. Audit assertions on every mutation; concurrency
   tests on every "one winner" invariant. Tests may be larger than the implementation.
7. **Verify Section 6.** The phase file's own checklist is the gate: every step run, with the command and what it
   printed. AKR: a step that cannot run on this machine (a signing service, a bank's CBS, a runner) is reported as
   not run, with the reason; it is never reported as passed.
8. **BROWSER-CHECK EVERY SCREEN, ONE BY ONE (phases with screens; never skipped).** Start the API and the frontend
   and drive every screen the phase added or changed: open it, submit its forms, provoke its error states, walk its
   flows end to end, in BOTH Arabic right-to-left and English left-to-right and in both themes. Report what was
   actually seen. A build that compiles is not a screen that works (the DBS shipped a phase with every gate green and
   every request failing, because the client doubled a path prefix).
9. **Track.** Flip the phase to Done in `status.md` with its counts. Write what the build found back into the plan
   in the same change (see below).

**Definition of done per phase:** Section 6 run and reported + full test suite green + every screen driven in a
browser + plan and trackers updated + `main` compiles.

## A gap met while building (AKR)

The plan was written before the code. When the code shows that the plan is wrong, silent or contradicts itself:

1. Fix the phase file in the same change as the code: the base text when it was wrong, a new `7.x` block when
   something was missing.
2. A choice that later phases depend on is a dated decision in `research.md` (the next DR number).
3. A gap that belongs to a later phase is written into that phase's file and into Blockers and Notes of `status.md`,
   so it is not found a second time.
4. Never work around a gap in code and leave the plan as it was.

## External dependencies

Every external dependency sits behind a trait with an in-memory implementation, so every phase compiles and its
FULL test suite runs with no live credential and no bank system. AKR:

- **The CBS.** `SourceReader` (phase 05) has adapters per engine and a synthetic fixture source. No test and no
  fixture touches a real CBS, and `cbs-discovery/`, `test-poc/` and `from-banks-pc.zip` are never copied anywhere.
- **Secrets and signing.** `akr-crypto` has the OpenBao implementation and an in-memory one for tests.
- **The object store.** `akr-objectstore` runs against the development stand-in; the WORM conformance suite
  (`make store-conformance`) is what a store must pass before a bank uses it.
- **Lists, the screening API, goAML, the SIEM, the bank's identity provider.** Each is a port with a recorded or
  synthetic source for tests. Live wiring happens at installation, behind configuration.

## Nothing leaves the bank (AKR)

No customer data, no transaction and no case content is sent to a service outside the installation. No external AI
API is called from any binary, in any environment. A test or a tool that needs realistic data generates it.

## Sequencing

Phases 00 to 07 are the load-bearing base (platform, identity, audit and evidence, authorisation, change control,
sources, data quality, lists) and must be solid before detection is built on them. Do not start a phase whose
dependencies are not Done.
