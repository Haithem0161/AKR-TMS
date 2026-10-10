#!/usr/bin/env python3
"""Apply the fixes of the re-check (verdicts: tools/agent/recheck-verdict.md).

Reads tools/agent/recheck-fixes.txt: '@@ FIX id path' / old text / '@@ WITH' / new text / '@@ END'.
The old text must occur exactly once. An entry whose new text is already in the file is skipped, so the
script can be run again after entries are added. A later entry may change text that an earlier entry wrote, so the
ids of the entries that are done are also kept in tools/agent/recheck-fixes.done and never applied again.
Run from the repository root:  python3 tools/agent/recheck-fix.py [--dry]
"""
import sys

DRY = "--dry" in sys.argv
SRC = "tools/agent/recheck-fixes.txt"
LEDGER = "tools/agent/recheck-fixes.done"


def parse():
    fixes, state, cur = [], None, None
    for raw in open(SRC, encoding="utf-8"):
        line = raw.rstrip("\n")
        if state is None:
            if line.startswith("@@ FIX "):
                _, _, fid, path = line.split(" ", 3)
                cur = [fid, path, [], []]
                state = "old"
            continue
        if line == "@@ WITH" and state == "old":
            state = "new"
        elif line == "@@ END" and state == "new":
            fixes.append((cur[0], cur[1], "\n".join(cur[2]), "\n".join(cur[3])))
            state = None
        elif line.startswith("@@ "):
            raise SystemExit(f"{cur[0]}: unexpected marker {line!r}")
        else:
            cur[2 if state == "old" else 3].append(line)
    assert state is None, f"{cur[0]}: entry not closed"
    return fixes


def main():
    fixes = parse()
    ids = [f[0] for f in fixes]
    assert len(ids) == len(set(ids)), "duplicate fix id"
    try:
        ledger = set(open(LEDGER, encoding="utf-8").read().split())
    except FileNotFoundError:
        ledger = set()
    texts, done, skipped, before = {}, 0, 0, 0
    for fid, path, old, new in fixes:
        if fid in ledger:
            before += 1
            continue
        t = texts.get(path)
        if t is None:
            t = open(path, encoding="utf-8").read()
        assert old and old != new, f"{fid}: empty or unchanged"
        n = t.count(old)
        if new in t:  # already applied (the new text may contain the old text, so this test comes first)
            skipped += 1
            texts[path] = t
            continue
        assert n == 1, f"{fid}: old text occurs {n} times in {path}"
        texts[path] = t.replace(old, new)
        done += 1
        print(f"  {fid}: {path}")
    if not DRY:
        for path, t in texts.items():
            open(path, "w", encoding="utf-8").write(t)
        with open(LEDGER, "a", encoding="utf-8") as f:
            for fid in ids:
                if fid not in ledger:
                    f.write(fid + "\n")
    print(f"{'DRY ' if DRY else ''}applied {done}, already there {skipped}, in the ledger {before}, entries {len(fixes)}")


main()
