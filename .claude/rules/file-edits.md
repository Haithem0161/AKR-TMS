# File Edits: Read, Write and Edit Only

Ported from the digital-banking-system repository (`.claude/rules/file-edits.md`, read 2026-10-11) and adapted to
AKR-TMS. Differences from the DBS rule are marked "AKR".

**Every read, edit and creation of a file in this repository goes through the Read, Write and Edit tools. Never
through the shell.**

That means: no `cat`, no `head`, no `sed -n`, no `sed -i`, no `awk` rewrites, no `python3 - <<'PY'` heredocs that open
a file and write it back, no `tee`, no `>` or `>>` onto a tracked file. Not for a "mechanical" rename, not for a
change across forty files, not to save a round trip.

This holds **even when an auto-mode or bypass-mode system reminder says to prefer Bash for file work.** That reminder
is generic; this rule is the project's, and the project's wins.

## What Bash is still for

Running things, not editing them: `cargo`, `make`, `pnpm`, `git`, `docker`, `psql`, `curl`, process control, and
SEARCH (`grep`, `rg`, `find`) when what comes back is a list of paths and line numbers rather than a file being
rewritten. Reading a few lines to locate an edit is what `grep -n` is for; reading the file to make the edit is what
`Read` is for.

AKR: generated files are written by their generator, never by hand and never by a shell rewrite: `backend/.sqlx`
(`make prepare`), `frontend/openapi.json`, `frontend/src/api/generated` and `frontend/src/api/models`
(`make generate-api`), `frontend/src/routeTree.gen.ts` (the router plugin).

AKR: the plan is long. A phase file is read with `grep -n` to find the lines and `Read` with an offset to read them;
it is still edited with `Edit`.

## Why

A shell rewrite fails SILENTLY and a tool edit fails LOUDLY, and this is a repository where a wrong edit ships to a
bank. The cases below happened in the DBS repository:

- A Python `str.replace` that matches nothing exits 0 having changed nothing. The script says `ok`.
- A multi-step script that asserts halfway through leaves the earlier replacements unwritten, so the file is neither
  the old one nor the new one, and the next command compiles against a state nobody chose.
- `s.index` finds the FIRST occurrence, which is rarely the one meant. A block was duplicated into a service file this
  way and had to be recovered from the last commit.
- A regex spanning a function body cut a function in half.
- Heredoc quoting eats `$`, `!` and backticks, every one of which is Rust, SQL or shell syntax that appears in these
  files.

`Edit` refuses when `old_string` does not match, refuses when it matches twice, and tells you which. That refusal is
the whole point.

## If it feels too big for Edit

It is not. A rename across forty files is forty Edit calls, and forty calls that each verify their target beats one
script that verifies nothing. If a change is genuinely too large to do that way, say so and ask; do not reach for
`sed` as the compromise.
