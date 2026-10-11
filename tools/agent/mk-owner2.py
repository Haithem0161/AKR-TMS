#!/usr/bin/env python3
"""Write the input files of a check round on the owner-decision fixes.

    python3 tools/agent/mk-owner2.py R1 owner2      # the round 1 fixes, input of round 2
    python3 tools/agent/mk-owner2.py R2 owner3      # the round 2 fixes, input of round 3

Reads tools/agent/owner-fixes.txt (written by tools/agent/ofix.py), takes the entries of the named round, groups them
by the phase they belong to and writes tools/agent/verify/<prefix>-fixes-<KEY>.txt, entries numbered F1, F2, ...
WHY is the lead's verdict line of tools/agent/owner-verdict.md for the id of the entry.

Keys: the phase number for a phase with owner blocks; "11" for phases 11 and 12 together; "L1" for the check-line
sweep over phases 06, 09, 15, 17, 20 and 21; "DR" for the DR-422 row. A fix in a system document goes to the phase
it follows. A key with more than MAX entries is split into parts a, b, ...
"""
import os
import re
import sys

ROOT = "/home/haithem/Projects/AKR-TMS"
SRC = os.path.join(ROOT, "tools/agent/owner-fixes.txt")
MAX = 20
ROUND, PREFIX = sys.argv[1], sys.argv[2]

CUSTOM_WHY = {
    "DOC-10": "System document in step with phase 10 section 7.26 item 5: migration 0030 grants `akr_ingest` UPDATE (capture_source) on canon.party_name, and DATA-MODEL.md listed the columns `akr_ingest` may update without it.",
    "DOC-26": "System document in step with phase 26 section 7.13: migration 0083 grants `akr_ingest` UPDATE (granted_on, row_hash) on canon.facility, and the DATA-MODEL.md grants paragraph named only the instalment table.",
    "O-18-1-note": "Agent note of round 1, not counted as a finding (the clipboard on a suspicion-tier party): the copied value is a UUID the caller already holds and names no person; one sentence is added to section 7.29 so nobody has to guess.",
    "DR-422": "DR-422 is the decision record that locks the design corrections of round 1. The row in docs/plan/research.md must say only what the phase files now say; each citing sentence in a phase block must name corrections that the block really carries.",
    "DR-423": "DR-423 is the decision record that locks the two design rules of round 2: phase 13 section 7.18 (a recompute resets the feature rows it no longer produces) and phase 04 section 7.21 (a rejection of a change request is not held by the subject check). The row must say only what those two sections say.",
    "L2-1": "Lead fix of round 2 (LOW), from an agent note: the SQL comment above feat.counterparty_links in phase 13 said \"Owner akr_app\"; every table is owned by `akr_migrate` and this one is written by `akr_app`.",
    "L2-2": "Lead find of round 2 (HIGH, a stated check failed): the month-end upsert of phase 13 section 3.1 never touched a row that a re-run no longer produces, and `akr_app` has no DELETE on feat.feature_values, so after a voided source tag or a replay the old value stayed. Fix as written (new section 7.18, DR-423): the write of a pair (feature, as_of) also resets, in the same statement, every stored row of the pair that holds a value and that the SELECT did not produce: `value` NULL, status `no_history` (`beyond_window` for the kind `days_since`), `basis` = the run's basis plus `reason: no_activity`. A reader takes a row without a value as an absent row. Only a row with status `value` is reset, so a second run writes nothing. The other L2-2 entries are pointers to section 7.18 from the base text, the test of section 7.17 and DATA-MODEL.md.",
}
DOC_KEY = {"L2-2": "13", "L2-4": "26"}
SWEEP = {"06", "09", "15", "17", "18", "20", "21"}
L1_KEY = {"06", "09", "15", "17", "20", "21"}


def parse():
    fixes, state, cur = [], None, None
    for raw in open(SRC, encoding="utf-8"):
        line = raw.rstrip("\n")
        if state is None:
            if line.startswith("@@ FIX "):
                _, _, rnd, rest = line.split(" ", 3)
                fid, path = rest.rsplit(" ", 1)
                cur = [rnd, fid, path, [], []]
                state = "old"
            continue
        if line == "@@ WITH" and state == "old":
            state = "new"
        elif line == "@@ END" and state == "new":
            fixes.append((cur[0], cur[1], cur[2], "\n".join(cur[3]), "\n".join(cur[4])))
            state = None
        else:
            cur[3 if state == "old" else 4].append(line)
    return fixes


VER = open(os.path.join(ROOT, "tools/agent/owner-verdict.md"), encoding="utf-8").read().split("\n")


def base_ids(fid):
    """O-10-1/6/7 -> O-10-1, O-10-6, O-10-7; "O-10-3/4, O-26-3" -> three ids; O-08-3a -> O-08-3; L-3-doubt -> L-3."""
    out = []
    for part in re.split(r",\s*", fid):
        m = re.match(r"((?:O|P|Q|R)-(?:\d+[a-z]?|DR)-)(\d+)((?:/\d+)*)", part)
        if m:
            out.append(m.group(1) + m.group(2))
            out += [m.group(1) + x for x in m.group(3).split("/")[1:]]
            continue
        m = re.match(r"L\d*-\d+", part)
        if m:
            out.append(m.group(0))
    return out


def why(fid, path):
    if fid in CUSTOM_WHY:
        return CUSTOM_WHY[fid]
    lines = []
    for b in base_ids(fid):
        if b.startswith("L"):
            lines += [l[2:] for l in VER if l.startswith(f"- {b} ")]
            continue
        pat = re.compile(re.escape(b) + r"(?![0-9])")
        lines += [l[2:] for l in VER if l.startswith("- ") and pat.search(l[:70])]
    m = re.fullmatch(r"docs/plan/phase-(\d\d)\.md", path)
    if fid == "L-1" and m:
        nn = m.group(1)
        lines = [l for l in lines if not l.startswith("L-1 in phase") or l.startswith(f"L-1 in phase {nn}")]
        if nn in SWEEP:
            lines += [l[2:] for l in VER if l.startswith(f"- Phase {nn} section 6")]
    assert lines, (fid, path)
    seen, out = set(), []
    for l in lines:
        if l not in seen:
            seen.add(l)
            out.append(l)
    return " // ".join(out)


def key_of(fid, path):
    m = re.fullmatch(r"docs/plan/phase-(\d\d)\.md", path)
    if m:
        nn = m.group(1)
        if nn in L1_KEY:
            return "L1"
        return "11" if nn in ("11", "12") else nn
    if path == "docs/plan/research.md":
        return "DR"
    if fid in DOC_KEY:
        return DOC_KEY[fid]
    if fid == "DOC-10":
        return "10"
    if fid == "DOC-26":
        return "26"
    b = base_ids(fid)
    assert b and b[0][0] in "OPQR", (fid, path)
    return b[0].split("-")[1][:2]


TEXT = {}


def stands(path, new):
    if path not in TEXT:
        TEXT[path] = open(os.path.join(ROOT, path), encoding="utf-8").read()
    return new in TEXT[path]


by = {}
for rnd, fid, path, old, new in parse():
    if rnd != ROUND:
        continue
    by.setdefault(key_of(fid, path), []).append((fid, path, old, new))

written = []
for key, items in sorted(by.items()):
    parts = [items]
    if len(items) > MAX:
        n = -(-len(items) // MAX)
        size = -(-len(items) // n)
        parts = [items[i:i + size] for i in range(0, len(items), size)]
    for pi, part in enumerate(parts):
        name = key + ("abcdef"[pi] if len(parts) > 1 else "")
        out = [f"Replacements of round {ROUND[1:]} of the owner-decision check, key {name}. {len(part)} entries.",
               "Each replaced the OLD text by the NEW text in place, in the file named by FILE. The id in brackets is the",
               "lead's finding id. WHY is the lead's verdict line for that finding: id, verdict and severity, what was",
               "wrong, and what the fix was meant to do.", ""]
        for k, (fid, path, old, new) in enumerate(part, 1):
            out += [f"=== F{k} [{fid}]", "FILE: " + path, "WHY: " + why(fid, path)]
            if not stands(path, new):
                out.append("NOTE: a later entry of the same round changed part of this NEW text again. Skip check 1 "
                           "for the exact text; check the text as it stands in the file now.")
            out += ["OLD: " + old, "NEW: " + new, ""]
        fn = f"tools/agent/verify/{PREFIX}-fixes-{name}.txt"
        open(os.path.join(ROOT, fn), "w", encoding="utf-8").write("\n".join(out))
        written.append((name, len(part)))
print(" ".join(f"{n}({c})" for n, c in written))
print("keys:", len(written), "entries:", sum(c for _, c in written))
