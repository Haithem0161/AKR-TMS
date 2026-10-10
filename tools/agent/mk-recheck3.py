#!/usr/bin/env python3
"""Write tools/agent/verify/applied3-NN.txt: the replacements of the second re-check per phase file.

Third re-check round. Only the entries of tools/agent/recheck-fixes.txt whose id starts with S- or L2- (the 29
replacements of the second re-check) are taken. They are grouped by the phase file they changed and numbered F1,
F2, ... in the order of the fixes file. WHY is the lead's verdict line of tools/agent/recheck2-verdict.md.
Run from the repository root:  python3 tools/agent/mk-recheck3.py
"""
import re

SRC = "tools/agent/recheck-fixes.txt"


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
        else:
            cur[2 if state == "old" else 3].append(line)
    return fixes


VER = open("tools/agent/recheck2-verdict.md", encoding="utf-8").read().split("\n")


def why(fid):
    base = re.match(r"(S|L2)-\d+-\d+", fid).group(0)
    lines = [l[2:] for l in VER if l.startswith(f"- {base} |") or f"same replacement as {base})" in l]
    assert lines, fid
    return " // ".join(lines)


by, other = {}, []
for fid, path, old, new in parse():
    if not re.match(r"(S|L2)-", fid):
        continue
    m = re.fullmatch(r"docs/plan/phase-(\d\d)\.md", path)
    (by.setdefault(m.group(1), []) if m else other).append((fid, path, old, new))
for nn, items in sorted(by.items()):
    out = [f"Replacements made in docs/plan/phase-{nn}.md by the second re-check (DR-413). {len(items)} entries.",
           "Each replaced the OLD text by the NEW text in place. The id in brackets is the lead's finding id; a finding",
           "of another phase can have a replacement in this file. WHY is the lead's verdict line for that finding:",
           "id, verdict and severity, what was wrong, and what the fix was meant to do.", ""]
    for k, (fid, _, old, new) in enumerate(items, 1):
        out += [f"=== F{k} [{fid}]", "WHY: " + why(fid), "OLD: " + old, "NEW: " + new, ""]
    open(f"tools/agent/verify/applied3-{nn}.txt", "w", encoding="utf-8").write("\n".join(out))
    print(nn, len(items))
print("other files:", [(f, p) for f, p, _, _ in other])
