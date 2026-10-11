# Nothing Is A Fixture: Almost Everything Is Admin-Managed

Ported from the digital-banking-system repository (`.claude/rules/admin-managed.md`, read 2026-10-11; decided there
on 2026-09-06, by the bank) and adapted to AKR-TMS. Differences from the DBS rule are marked "AKR".

**If the bank could plausibly want it different next month, an administrator changes it on a screen. Not a fixture,
not a constant, not a script, not a redeploy.**

AKR: one installation serves one bank, and every bank is different: its CBS, its branches, its teams, its thresholds,
its lists. A fact about a bank that lives in code costs a release to change, and a release at a bank is an offline
bundle, a signature check and a change window.

## The test

Ask: **who is the authority on this?**

| The authority is | Then it lives in | Examples (AKR) |
|---|---|---|
| The bank | The database, edited on a screen, audited | who is on the investigation team, who covers for whom and until when, a rule's parameters inside their allowed range, which scenario runs on which segment, which list sources are switched on, a queue's routing, the length of an internal deadline, a `settings` value |
| The regulator | Code and the shipped packs, with the source cited | a CBI figure such as the USD 10,000 limit (DSL.md pins it: `min = max`), a legal retention floor, a report's legal deadline, the goAML schema |
| The product | Code, migrations, the DSL schema, the Cedar action vocabulary | what a rule kind computes, which actions exist, the classification levels, the retention classes, the shape of an audit event, the four-eyes rule on a case decision |

A fixture is acceptable only as a **seed**: the first value of something the bank will then own. The moment a fixture
is the only way to change a fact, it is a bug with a release attached.

AKR: a value the regulator owns is never offered on a screen as if the bank owned it. A setting may have a floor or a
ceiling that the product enforces (a retention period can be lengthened and never shortened below the legal floor).

## What this means in practice

- **A new team, queue or branch grouping is a screen, not a script.** The bank creates it, names it in both
  languages and puts people in it without anybody opening an editor.
- **Who may see or do something is data plus policy.** A person's roles, teams and case access are rows; what a role
  may do is the Cedar bundle. Neither is a constant in a handler.
- **Bounded facts carry their bounds.** A seat, a delegation, a team membership and a break-glass grant all take a
  window. "Covers the sanctions queue for one day" must be expressible without anybody remembering to undo it.
- **Seeding stays idempotent and safe to re-run.** A migration seeds a `settings` row, a classification row or a
  retention row with `ON CONFLICT ... DO NOTHING`, so applying it again never overwrites a value the bank changed. A
  seed that clobbers an administrator's edit is the failure this rule exists to prevent.
- **Synthetic data only (AKR).** A fixture never contains bank data, a real customer or a real name from a sanctions
  list. `cbs-discovery/`, `test-poc/` and `from-banks-pc.zip` are read for understanding and never copied into a
  fixture, a test or a seed.

## The two rules that do not bend

**Admin-managed is not admin-invented.** Everything an administrator changes is Cedar-gated, audited with an actor
and a reason, and versioned where it governs anything that has already happened. AKR: a change that moves detection
or access (a threshold, a rule version, a list source, a role grant) goes through a change request with a second
person (phase 04); nobody approves their own change. A screen that lets somebody widen their own authority silently
is worse than a fixture.

**A published version stays pinned.** An alert carries the version of the rule that raised it, a decision carries the
policy version it was taken under, and a report carries the template version it was rendered with. Changing the rule
makes a new version; it never rewrites what an existing alert, decision or report was based on.

## When adding a feature

Default to a table, a route, a policy action and a screen. If you are about to write a constant, a `const &[&str]`,
or a line in a fixture that names a person, a team, a branch, an amount or a deadline, stop and ask the test above.
The answer is almost always the bank, sometimes the regulator, and rarely the product.
