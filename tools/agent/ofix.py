#!/usr/bin/env python3
"""Owner-decision round: exact text replacements, all or nothing, logged for the next check round.

Use from a fix script in tools/agent/:

    from ofix import Fixer
    f = Fixer(round_no=1)
    f.fix("O-13-5", "docs/plan/phase-13.md", "old text", "new text")        # old must occur exactly once
    f.fix("O-25-1", "docs/plan/phase-25.md", "old text", "new text", n=2)   # or exactly n times
    f.done()

Rules:
- Every `old` must occur exactly `n` times in the file as it stands after the earlier fixes of the same script.
  One failed count stops the script before any file is written.
- For docs/plan/phase-NN.md the same replacement is also made in every block source tools/agent/owner/pNN*.md
  that holds `old` (the 7.x blocks were inserted from those files), so the two stay identical.
- done() writes the files and appends each fix to tools/agent/owner-fixes.txt in the format of
  tools/agent/recheck-fixes.txt ("@@ FIX <round> <id> <path>", old, "@@ WITH", new, "@@ END").
"""
import glob
import os
import re
import sys

ROOT = "/home/haithem/Projects/AKR-TMS"
LOG = os.path.join(ROOT, "tools/agent/owner-fixes.txt")


class Fixer:
    def __init__(self, round_no):
        self.round_no = round_no
        self.text = {}      # path -> current text
        self.log = []       # (fid, path, old, new)
        self.errors = []
        self.src_hits = 0

    def _load(self, path):
        if path not in self.text:
            self.text[path] = open(os.path.join(ROOT, path), encoding="utf-8").read()
        return self.text[path]

    def fix(self, fid, path, old, new, n=1):
        assert old != new, fid
        s = self._load(path)
        c = s.count(old)
        if c != n:
            self.errors.append(f"{fid} {path}: found {c} times, expected {n}: {old[:90]!r}")
            return
        self.text[path] = s.replace(old, new)
        self.log.append((fid, path, old, new))
        m = re.fullmatch(r"docs/plan/phase-(\d\d)\.md", path)
        if m:
            for src in sorted(glob.glob(os.path.join(ROOT, f"tools/agent/owner/p{m.group(1)}*.md"))):
                rel = os.path.relpath(src, ROOT)
                t = self._load(rel)
                if old in t:
                    self.text[rel] = t.replace(old, new)
                    self.src_hits += 1

    def show(self, path, needle, width=400):
        """Print the lines of the pending text of `path` that hold `needle` (for a look before done())."""
        for i, line in enumerate(self._load(path).split("\n"), 1):
            if needle in line:
                k = line.index(needle)
                print(f"{path}:{i}: ...{line[max(0, k - width // 2):k + width // 2]}...")

    def done(self):
        if self.errors:
            print("NOTHING WRITTEN. Errors:")
            for e in self.errors:
                print("  " + e)
            sys.exit(1)
        for path, s in self.text.items():
            open(os.path.join(ROOT, path), "w", encoding="utf-8").write(s)
        with open(LOG, "a", encoding="utf-8") as out:
            for fid, path, old, new in self.log:
                out.write(f"@@ FIX R{self.round_no} {fid} {path}\n{old}\n@@ WITH\n{new}\n@@ END\n")
        files = sorted({p for _, p, _, _ in self.log})
        print(f"applied {len(self.log)} fixes in {len(files)} files; block-source copies updated: {self.src_hits}")
        for p in files:
            print("  " + p)
