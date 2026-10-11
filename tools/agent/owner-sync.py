#!/usr/bin/env python3
"""Check that every block source tools/agent/owner/pNN*.md stands word for word in docs/plan/phase-NN.md."""
import glob, os, re
ROOT = "/home/haithem/Projects/AKR-TMS"
bad = 0
for src in sorted(glob.glob(os.path.join(ROOT, "tools/agent/owner/p*.md"))):
    nn = re.match(r"p(\d\d)", os.path.basename(src)).group(1)
    block = open(src, encoding="utf-8").read().strip("\n")
    text = open(os.path.join(ROOT, f"docs/plan/phase-{nn}.md"), encoding="utf-8").read()
    if block in text:
        print(f"ok   {os.path.basename(src)}")
        continue
    bad += 1
    bl, tl = block.split("\n"), set(text.split("\n"))
    miss = [l for l in bl if l not in tl]
    print(f"DIFF {os.path.basename(src)}: {len(miss)} of {len(bl)} lines differ")
    for l in miss[:4]:
        print("     " + l[:160])
print("block sources out of step:", bad)
