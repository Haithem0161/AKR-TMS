#!/usr/bin/env python3
"""Apply the verification pass fixes after the lead's checks. Run ONCE (asserts no earlier run).

Sources: tools/agent/verify/phase-NN.md (first agents), tools/agent/verify/refute-NN.md (refuters);
decisions: tools/agent/verify-verdict.md. The fixes live in tools/agent/verify/fixes/NN.txt, one file per AUDITED
phase, in this form (TT = the phase file that receives the fix):

    @@ BLOCK TT
    ### 7.N Title (V-NN-KK-a, SEVERITY; DR-413)
    text ...
    @@ REP TT
    exact old text (must occur once in phase TT)
    @@ WITH
    new text
    @@ END

Lines before the first "@@" are comments. `--check` parses and asserts without writing.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'
FIX = ROOT / 'tools' / 'agent' / 'verify' / 'fixes'
CHECK = '--check' in sys.argv

HDR = ('Verification pass (2026-10-10), from tools/agent/verify/; lead-verified '
       '(tools/agent/verify-verdict.md).')

blocks = {}   # target phase -> [block text]
reps = {}     # target phase -> [(old, new, source file)]

for f in sorted(FIX.glob('[0-9][0-9].txt')):
    mode, tgt, buf, old = None, None, [], None

    def flush():
        global mode, tgt, buf, old
        text = '\n'.join(buf).strip('\n')
        if mode == 'BLOCK':
            assert text.startswith('### 7.N '), (f.name, text[:60])
            assert 'DR-413' in text.split('\n', 1)[0], (f.name, text[:80])
            blocks.setdefault(tgt, []).append(text)
        elif mode == 'WITH':
            assert old and text, (f.name, old)
            reps.setdefault(tgt, []).append((old, text, f.name))
        elif mode == 'REP':
            raise AssertionError((f.name, 'REP without WITH', text[:60]))
        buf = []

    for line in f.read_text().split('\n'):
        if line.startswith('@@ BLOCK '):
            flush()
            mode, tgt = 'BLOCK', line.split()[2]
        elif line.startswith('@@ REP '):
            flush()
            mode, tgt = 'REP', line.split()[2]
        elif line.strip() == '@@ WITH':
            assert mode == 'REP', (f.name, line)
            old = '\n'.join(buf).strip('\n')
            buf, mode = [], 'WITH'
        elif line.strip() == '@@ END':
            flush()
            mode = None
        elif line.startswith('@@'):
            raise AssertionError((f.name, line))
        elif mode:
            buf.append(line)
    flush()

out = {}
for tgt in sorted(set(blocks) | set(reps)):
    assert len(tgt) == 2 and tgt.isdigit(), tgt
    p = PLAN / f'phase-{tgt}.md'
    s = p.read_text()
    st = s.index('## 7+. Gap Additions')
    nx = s.find('\n## ', st + 5)
    assert 'Verification pass (2026-10-10)' not in s[st:nx if nx != -1 else len(s)], tgt
    for old, new, src in reps.get(tgt, []):
        assert s.count(old) == 1, (src, tgt, s.count(old), old[:80])
        s = rep(s, old, new)
    if tgt in blocks:
        s = add_section7(s, HDR, blocks[tgt])
    out[p] = s

r = PLAN / 'research.md'
rs = r.read_text()
anchor = [l for l in rs.split('\n') if l.startswith('| DR-412 |')]
assert len(anchor) == 1 and '| DR-413 |' not in rs
dr_file = FIX / 'DR-413.txt'
DR = dr_file.read_text().strip() if dr_file.exists() else ''
if not CHECK:
    assert DR.startswith('| DR-413 | 2026-10-10 |') and DR.endswith('|') and '\n' not in DR, DR[:80]
    out[r] = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + DR + '\n')

nb = sum(len(v) for v in blocks.values())
nr = sum(len(v) for v in reps.values())
print(f'{nb} blocks, {nr} replacements, {len(out)} files')
for tgt in sorted(set(blocks) | set(reps)):
    print(f'  phase {tgt}: {len(blocks.get(tgt, []))} blocks, {len(reps.get(tgt, []))} replacements')
if CHECK:
    print('check only: nothing written')
else:
    for p, s in out.items():
        p.write_text(s)
        print('wrote', p.relative_to(ROOT))
