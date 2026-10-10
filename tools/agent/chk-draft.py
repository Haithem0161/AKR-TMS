#!/usr/bin/env python3
"""Check one fixes or draft file of the verification pass without writing anything.

Usage: python3 tools/agent/chk-draft.py tools/agent/verify/draft/NN.txt [more files]
For every `@@ REP TT` entry: how often the old text occurs in docs/plan/phase-TT.md (must be 1) and whether it is
one line. For every `@@ BLOCK TT` entry: the header line and whether it has the "### 7.N " form with "DR-413".
With `-n NAME ...` after the files: also count each NAME (a plain string) in every phase file and print the hits.
"""
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'

args = sys.argv[1:]
names = []
if '-n' in args:
    i = args.index('-n')
    args, names = args[:i], args[i + 1:]

for a in args:
    f = Path(a)
    mode, tgt, buf, old = None, None, [], None
    n = 0

    def flush():
        global mode, buf, old, n
        text = '\n'.join(buf).strip('\n')
        if mode == 'BLOCK':
            n += 1
            head = text.split('\n', 1)[0]
            ok = head.startswith('### 7.N ') and 'DR-413' in head
            print(f'{f.name} #{n} BLOCK {tgt} {"ok " if ok else "BAD"} {head[:110]}')
        elif mode == 'WITH':
            n += 1
            s = (PLAN / f'phase-{tgt}.md').read_text()
            c = s.count(old)
            one = '\n' not in old
            flag = 'ok ' if c == 1 and one and text else 'BAD'
            print(f'{f.name} #{n} REP   {tgt} {flag} count={c} oneline={one} :: {old[:90]}')
        buf = []

    for line in f.read_text().split('\n'):
        if line.startswith('@@ BLOCK '):
            flush(); mode, tgt = 'BLOCK', line.split()[2]
        elif line.startswith('@@ REP '):
            flush(); mode, tgt = 'REP', line.split()[2]
        elif line.strip() == '@@ WITH':
            old = '\n'.join(buf).strip('\n'); buf, mode = [], 'WITH'
        elif line.strip() == '@@ END':
            flush(); mode = None
        elif mode:
            buf.append(line)
    flush()

for name in names:
    hits = []
    for p in sorted(PLAN.glob('phase-*.md')):
        c = p.read_text().count(name)
        if c:
            hits.append(f'{p.stem[-2:]}:{c}')
    print(f'NAME {name!r}: {" ".join(hits) if hits else "NOWHERE"}')
