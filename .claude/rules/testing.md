---
paths:
  - "**/*test*"
  - "**/tests/**"
  - "tests/curl/**"
  - ".gitea/workflows/**"
  - "backend/fixtures/**"
---

# Testing Rules: STRICT, Deep, Real, Adversarial

Ported from the digital-banking-system repository (`.claude/rules/testing.md`, read 2026-10-11) and adapted to
AKR-TMS. Differences from the DBS rule are marked "AKR".

This is a bank's compliance system. A phase is NOT done when it works; it is done when it cannot be made to
misbehave. **Tests are allowed and expected to be larger than the implementation they cover.** If a mutation can be
raced, replayed, spoofed or leaked, there is a test that tries to.

## 0. Non-negotiables

1. **Real curl tests are first-class.** Every endpoint and every end-to-end flow gets a committed, runnable curl
   suite that hits the running stack and asserts on real HTTP responses (status, body through `jq`) AND on the
   resulting database and audit state.
2. **At least 10 distinct request cases per endpoint and per flow**, spanning the matrix in section 3.
3. **Every mutation asserts its audit event**: kind, actor, resource and correlation id. A mutating test with no
   audit assertion is incomplete.
4. **Every "exactly one winner" invariant has a concurrency test** that fires K simultaneous requests and asserts
   the invariant.
5. **`main` is always green**: `cargo check`, `make lint`, `make test-all`.
6. **AKR: a tipping-off test for every suspicion-tier artefact.** A person outside the case's access list learns
   nothing from a read, a list, a count, a search, an export, an SSE event, a notification or an error. The masked
   answer is compared with the answer for a resource that does not exist, byte for byte.
7. **AKR: synthetic data only.** No test, fixture or snapshot contains bank data, a real customer, a real account
   or a real name from a sanctions list. `cbs-discovery/`, `test-poc/` and `from-banks-pc.zip` are never copied into
   the repository's tests. Generated names are visibly invented.
8. **AKR: a check that did not run is reported as not run.** A skipped test, an ignored test and a step that needs
   something this machine lacks are named in the phase report with the reason. Nothing is called passed on the
   strength of code that was only read.

## 1. Test layers

| Layer | Where | Runs against | Covers |
|---|---|---|---|
| Unit | inline `#[cfg(test)]` | pure functions | validators, the DSL compilers, time and calendar arithmetic, hash chains, matching, serde round-trips |
| Integration | `<binary>/tests/`, `crates/<crate>/tests/`, with `#[sqlx::test]` | a throwaway PostgreSQL database, the real router and the real loops | CRUD, authorisation decisions, audit writes, races, state machines, grants and row-level security |
| Store and guard | `#[ignore]` tests, run by `make test-all` | the development stack (Valkey, the object store, the guard) | WORM behaviour, the guard's allow-list, rate limits, pub/sub |
| Curl (E2E) | `tests/curl/phaseNN_<area>.sh` | the running compose stack | real HTTP with database and audit assertions; whole flows |
| Conformance (AKR) | `akr-ctl store conformance`, the read-only source suite (phase 05) | a real store, a real source engine | what a bank's component must pass before it is used |
| Frontend | Vitest, `tsc -b`, `vite build`, Playwright with axe | components, generated hooks, the built bundle | states, both directions, both themes, accessibility |

## 2. Integration tests (`#[sqlx::test]`)

- Tests live PER CRATE. The attribute's path is relative to the crate root: `#[sqlx::test(migrations =
  "../migrations")]` in a binary, `"../../migrations"` in a library crate. Each test gets its OWN freshly migrated
  database.
- AKR: they need a superuser `DATABASE_URL` (the test creates databases) and `SQLX_OFFLINE=true`. `make test` sets
  both; do not hand-roll the command.
- Deterministic and order-independent: no shared state, no `sleep`. Wait on a condition, with a deadline.
- Drive the real stack: build the app with the test support of the binary and call it with `oneshot`, or run the
  real loop against the pool. The middleware (auth, idempotency, correlation, authorisation) is part of what is
  under test. A repository method tested alone proves little.
- Test files start with `#![allow(clippy::unwrap_used, clippy::expect_used)]`: the workspace denies both in
  production code.
- **AKR: test the database's own rules as the role that is bound by them.** A grant, a row-level-security policy
  or a "this role cannot delete" rule is tested by connecting as that role (`SET ROLE`) and expecting the refusal
  from PostgreSQL, not by reading the migration.
- Concurrency tests spawn K tasks against the SAME resource and assert the invariant:
  - a claim race: exactly one winner, K-1 conflicts
  - a double decision or a double approval: one transition, K-1 x 409
  - a maker approving their own change: refused whatever else races with it
  - a replayed batch: no second alert, the same fingerprint
  - numbering: K registrations give K sequential numbers, no duplicate and no gap
- Time is injected. A test that needs "31 days later" sets the clock port; it never waits and never edits rows to
  fake age unless the age column is what is under test.

## 3. The per-endpoint request matrix (the 10 and more)

1. Happy path, typical input
2. Happy path, boundary-valid input (maximum length, minimum value, empty optional)
3. Happy path, a second realistic role where it applies
4. Missing `Idempotency-Key` on a mutation: 400 `E_IDEMPOTENCY_REQUIRED`
5. Idempotency **replay** (same key, same body): the stored response byte for byte, `x-idempotent-replay: true`, no
   second effect and no second audit event
6. Idempotency **reuse** (same key, different body): 422 `E_IDEMPOTENCY_REUSE`
7. `If-Match` missing: 428 `E_PRECONDITION_REQUIRED`; stale: 409 `E_VERSION_CONFLICT` with the current version
8. Authorisation **allow**
9. Authorisation **deny**, once per reason: scope (403 `E_SCOPE`), role (403 `E_FORBIDDEN`), segregation of duties
   (403 `E_SELF_APPROVAL`), and for a suspicion-tier resource the masked 404
10. Validation: each field rule broken once (type, required, range, enum, unknown field): 422 with `errors[]`
11. Audit: the event landed with the right actor, action, resource and correlation id
12. Pagination on lists: first page, follow `next_cursor`, the empty last page, a forged cursor
13. Step-up where the operation is marked SU: absent, replayed and wrong-class assertions all give 401
    `E_STEPUP_REQUIRED`
14. No credentials: 401, in problem+json

Then the phase's own adversarial cases. Ten is the floor, not the target.

## 4. Adversarial targets (AKR)

Every decision-adjacent and deadline-adjacent phase tests the ones that apply to it:

- four-eyes races on case decisions and report approval; a change request approved by its own maker
- claim races on alerts; duplicate alerts on a replayed batch; fingerprint collisions
- late data amending an alert that was already decided
- a watermark gap after a CBS restore or failover
- a freeze clock or a report clock firing on a closed case (the timer handler's re-check)
- existence masking leaking through a count, a search, a sort order, an export or a timing difference
- a replayed idempotency key; a replayed step-up assertion
- an audit-chain gap; an audit row changed after the fact
- a job delivered twice (run the handler twice and compare the state)
- a source granted a write privilege (refused at connect); a statement other than SELECT reaching a source

## 5. Flow (E2E) tests

Each end-to-end flow gets a curl script that runs the whole flow and asserts the state and the audit trail at EACH
step. A flow test that walks only the happy path is incomplete: inject the rejection, the recall, the race and the
unauthorised look.

## 6. Curl suite conventions (`tests/curl/`)

- One script per phase area: `tests/curl/phaseNN_<area>.sh`. Committed, executable, re-runnable.
- `set -euo pipefail`; a shared `tests/curl/_lib.sh` for signing in, `jq` assertions and `assert_audited`.
- Each script seeds its own synthetic fixtures, prints one pass or fail line per case and exits non-zero on ANY
  failure.
- Real requests only: nothing is mocked at the HTTP layer. Stand-ins live behind ports, never in the request path.
- Anything that touches identity flows, cookies or credential shapes is proven against the real Kratos and Hydra
  (`auth.md`).

## 7. What every phase MUST test

- [ ] At least 10 request cases per new endpoint (curl and integration)
- [ ] Every new flow has an E2E curl script with its rejection, recall and race branches
- [ ] Every mutation asserts its audit event
- [ ] Every "one winner" invariant has a concurrency test
- [ ] Every gated route tested for allow and for each deny reason (403 versus the masked 404)
- [ ] Idempotency replay and reuse on every mutation; `If-Match` conflict on every versioned mutation
- [ ] State machines: every legal transition once, a sample of illegal ones refused
- [ ] Every new job kind: payload check, idempotent handler, retry to `dead` with the `job.dead` event
- [ ] Every new table: grants tested as each role; classification and retention rows present (`make check`)
- [ ] Every new suspicion-tier artefact: the tipping-off test
- [ ] Every new setting: its bounds and its floor refused below (`E_BELOW_FLOOR`)
- [ ] `make lint`, `make test-all` and `make prepare-check` clean
- [ ] `make generate-api` twice gives zero difference
- [ ] The phase file's section 6, step by step, with what each step printed
- [ ] Every screen the phase touched driven in a real browser, in Arabic right-to-left and English left-to-right
      and in both themes (`implementation-flow.md`)

## 8. Running

```bash
make test            # everything that needs PostgreSQL only
make test-all        # also the ignored tests: Valkey, the object store, the guard (needs `make up`)
make lint            # cargo fmt --check and clippy with warnings as errors
make prepare-check   # backend/.sqlx is what the code needs
make store-conformance          # the WORM suite through the guard
make zone-check                 # the zone matrix against the running stack
```

To run one crate's tests by hand, copy the environment `make test` sets (`SQLX_OFFLINE=true` and the superuser
`DATABASE_URL` with `sslmode=verify-full`).

## 9. Frontend testing

- `tsc -b` and `vite build` are the baseline gate of every phase with screens.
- Vitest and React Testing Library for: validation messages in both languages, the loading, error, empty and data
  states, layout under `dir="rtl"`, optimistic versus awaited behaviour, SSE-driven invalidation.
- Never test the generated hooks' internals; test the components that use them.
- AKR: Playwright with axe against the development stack, and a check that records every request a session makes:
  none leaves the installation's origin.
