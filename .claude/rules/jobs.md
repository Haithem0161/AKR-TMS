---
paths:
  - "backend/crates/jobs/**"
  - "backend/worker/**"
  - "backend/ingestd/**"
  - "backend/crates/db/src/jobs.rs"
  - "backend/crates/db/src/outbox.rs"
  - "backend/crates/db/src/timers.rs"
---

# Background Job Processing Rules

Ported from the digital-banking-system repository (`.claude/rules/jobs.md`, read 2026-10-11) and adapted to
AKR-TMS. Differences from the DBS rule are marked "AKR". The DBS's own kinds (rendition, OCR, push, headline) are not
carried over; AKR's kinds are listed in DATA-MODEL.md and in each phase file's section 4.

Durability lives in **PostgreSQL**, not in Valkey. Three tiers:

1. **tokio tasks**: lightweight fire-and-forget work inside one process.
2. **`admin.jobs`**: durable, retryable jobs claimed with `FOR UPDATE SKIP LOCKED`.
3. **Transactional outbox** (`admin.outbox`) and **timers** (`admin.timers`): event fan-out and scheduled actions,
   both written in the same transaction as the domain change.

Valkey is cache, rate limits and SSE fan-out **only**, never a durable queue. If work must survive a restart, it is a
PostgreSQL row.

> Jobs have no session. A job's payload carries the ids it needs, and the handler runs as the system principal;
> every side effect it makes is still audited.

## When to use what

| Use case | Mechanism | Reason |
|----------|-----------|--------|
| Work that must complete: a scan, a render, a detection run, an export, a signature | `admin.jobs` row | Retryable, visible in the cockpit |
| Tell other parts of the product about a domain event | `admin.outbox` row in the SAME transaction | No lost and no phantom events |
| A deadline, a reminder, an escalation at a future time | `admin.timers` row | `fire_at` computed from the calendar, swept by the worker |
| A periodic task (a sweep, a check, a refresh) | a scheduled job kind: `.scheduled_by("schedule.<kind>")` | The cron is a setting; several replicas enqueue the slot once |
| Push a live update to connected browsers | Valkey pub/sub to the SSE bridge | Ephemeral by nature; clients refetch on reconnect |
| Recompute a cached value | `tokio::spawn` | Ephemeral; heals on the next read |
| Anything that must not be lost | A PostgreSQL row. Always. | Valkey and tokio tasks vanish on restart |

## The golden rule: same-transaction enqueue

A domain write and the job, outbox and timer rows it implies commit **atomically**:

```rust
let mut tx = pool.begin().await?;
// 1. the domain write
// 2. the audit event
// 3. JobQueue::enqueue(&mut tx, NewJob::new(&KIND)...) / outbox append / TimerService::schedule(&mut tx, ...)
tx.commit().await?;
```

AKR: the signature enforces it. `JobQueue::enqueue`, the outbox `append` and `TimerService::schedule` accept only a
transaction; there is no way to enqueue against the pool. NEVER enqueue after the commit (a crash loses the job) and
NEVER before the domain write is known to succeed.

## Four binaries, two job loops (AKR)

- **`akr-worker`** runs the harness over the product's kinds: the poll loop, the lease reaper, the scheduler, the
  outbox dispatcher and the timer sweep.
- **`akr-ingestd`** runs the same harness over its own kinds only: names that start `ingest_` or `discovery_`. It may
  also enqueue `map_` jobs for `akr-worker`. **Row-level security on `admin.jobs` enforces this for the role
  `akr_ingest`**: it sees and updates only `ingest_` and `discovery_` rows and may insert those and `map_` rows. A
  new kind for `akr-ingestd` therefore takes one of those prefixes, or it will never be claimed.
- **`akr-api`** enqueues and never runs a handler.
- **`akr-assess`** answers inside its latency budget and enqueues nothing on the request path.
- A binary claims only the kinds in its own registry. A queued job of a kind no running binary knows is a release
  error and is shown as one in the cockpit.

## Loops versus jobs (AKR)

A **job** is one unit of work with a beginning, an end and a row. A **loop** is work with no natural unit that a
binary runs for as long as it lives: the poll loop itself, the outbox dispatcher, the timer sweep, the scheduler, an
ingestion reader following a source, a listener on a PostgreSQL channel.

- A loop keeps its position in PostgreSQL (a watermark, a claimed row, a cursor), never only in memory, so a restart
  continues and does not repeat or skip.
- A loop stops on the binary's cancellation token and finishes the unit it holds before it returns.
- A loop is named in the phase file's section 4 with its interval and its owner binary, and it reports a metric so a
  stalled loop is visible. A loop that is part of readiness says so in `/readyz`.
- Do not write a loop where a scheduled job fits. If the work is "every N minutes do X to completion", it is a
  scheduled kind: it gets retries, a lease, a row in the cockpit and `job.dead` for free.

## Worker harness

- **Claim**: `status = 'queued' AND run_at <= now() AND kind = ANY($known)` ordered by `run_at`, `FOR UPDATE SKIP
  LOCKED`, up to the free permits of each kind. The poll runs every 500 ms and at once after `NOTIFY akr_jobs`.
- **Concurrency is PER KIND and enforced**: one semaphore per kind, sized by `JobKind::concurrency`. A heavy kind gets
  a small number, set against the real ceiling behind it (the pool size, the scanner's threads, the renderer).
- **A kind is a `static`**: `JobKind::new("name")` with `.concurrency(n)`, `.max_attempts(n)`, `.lease(d)` and
  `.scheduled_by(setting_key)`. `NewJob::new(&KIND)` and `JobRegistry::register(&KIND, handler)` take it by
  reference, so a kind cannot be misspelt at one of its uses.
- **Outcomes**: `Ok(())` is `done`. `JobError::Retryable` goes back to `queued` with `run_at = now() + 2^attempts *
  2 s` (capped at one hour). `JobError::Permanent`, or the last attempt, is `dead`. A panic is a retryable failure.
- **`dead` is never silent**: the transaction that buries a job writes the audit event `job.dead` and adds to
  `akr_jobs_dead_total{kind}`, the cockpit lists the row with a retry action, and an alert fires.
- **Lease**: a `running` row whose lease has passed goes back to `queued` (or to `dead` on its last attempt). Delivery
  is therefore at-least-once, and **handlers MUST be idempotent**: key side effects on natural ids, or check before
  acting.
- **Coalescing**: `coalesce_key` makes a second enqueue of the same queued work a no-op. Scheduled kinds use
  `<kind>:<slot>`.
- **`last_error`** is redacted and cut to 500 characters. An error text never carries customer data.

## Payloads carry ids and codes only (AKR)

A job, outbox or timer payload is a JSON object of ids, codes, counts and instants. No name, no account number, no
amount, no free text: the payload is not field-encrypted and it is kept after the work is done. The handler reads
what it needs by id, under its own role's grants. `akr_jobs::payload::check` is the test helper for this, and a test
of every new kind calls it. The correlation id of the request that enqueued travels in `correlation_id`.

## Outbox

The dispatcher delivers one row per transaction: the handler's effects and `dispatched_at` commit together, and a
failure rolls the effects back, adds one to `attempts` and moves `available_at`. A row that reaches
`outbox.max_attempts` stays undispatched, is counted by `akr_outbox_stuck`, raises an alert and is never deleted by
the sweep: a lost event is somebody's missing alert or notice.

A topic handler is idempotent for the same reason a job handler is.

## Timers

`TimerService::schedule(tx, NewTimer { .. })` writes the timer with the change that starts the clock;
`cancel` and `cancel_for_owner` stop it. The sweep claims due timers and enqueues the job `timer_<kind>` in the same
transaction.

**Every timer handler re-reads its owner and does nothing if the owner's state no longer calls for it** (a closed
case, a completed task, a released freeze). Cancelling a timer is an optimisation; the re-check is the guarantee.

Deadlines in working days go through `BusinessClock`. `CalendarDaysClock` counts calendar days and is named so that
nobody mistakes it for the bank's calendar (phase 09 adds `BankCalendarClock`).

## tokio::spawn

```rust
let state = state.clone();
tokio::spawn(async move {
    if let Err(e) = state.some_svc.do_followup(id).await {
        tracing::error!(error = %e, %id, "follow-up failed");
    }
});
```

- NEVER `.await` the JoinHandle in the handler.
- ALWAYS clone what you need BEFORE the `move` closure.
- ALWAYS log errors inside the spawned task (they are otherwise dropped).
- NEVER use `tokio::spawn` for work that MUST complete: that is a job row.
- Use `tokio::task::spawn_blocking` for CPU-heavy work (hashing, parsing a large file).

## SSE fan-out

A domain event that needs a live update publishes ids to a Valkey channel after commit; the API's SSE endpoint
forwards typed events and clients refetch. Pub/sub is best-effort by design: correctness never depends on it,
because the data is already in PostgreSQL.

AKR: an SSE event carries an id and a kind and nothing a person outside the case's access list could learn from; the
refetch goes through the same authorisation as any read.

## Adding a kind

A new kind is: an entry in the phase file's section 4 and in DATA-MODEL.md, a `static JobKind`, a handler registered
in exactly one binary, a concurrency decision with its reason, a payload test, an idempotency test (run the handler
twice), and a `schedule.<kind>` setting when it is periodic.

## Common pitfalls

- **Enqueueing outside the domain transaction.** The original sin. Same transaction, always.
- **`tokio::spawn` for durable work.** A restart loses it.
- **Valkey as a queue.** We deliberately do not.
- **Non-idempotent handlers.** At-least-once delivery WILL replay a job.
- **Customer data in a payload or an error text.**
- **Unbounded concurrency.** One semaphore per kind.
- **An `akr-ingestd` kind without the `ingest_` or `discovery_` prefix.** Row-level security hides it from the binary
  that should run it.
- **A denormalised copy with more than one writer.** A value that can be computed from something authoritative is
  computed, or it has exactly ONE writer (the DBS learnt this with a task deadline copied from its timer).
