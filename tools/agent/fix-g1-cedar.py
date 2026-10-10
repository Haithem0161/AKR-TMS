#!/usr/bin/env python3
"""Gap pass 1, gap item (a): correct the Cedar role form and the aal comparison in place.

`principal in Akr::Role::"X"` is not how roles are modelled (phase 03: `principal.roles.contains("X")`), and
`context.aal` is a String ("aal1" | "aal2", phase 03 Ctx), so `context.aal >= 2` does not type-check.
Each phase gets a short 7.x note recording the in-place correction.
"""
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'
CHDR = ('Gap pass 1 (2026-10-10), gap item (a): Cedar form corrected in place '
        '(tools/agent/fix-g1-cedar.py); lead-verified.')

ROLE = r'principal in Akr::Role::"([A-Z0-9]+)"'


def fix_cedar(block):
    # 1. permit (principal in Akr::Role::"X", ... resource);  ->  permit (principal, ... resource)\nwhen {...};
    def head(m):
        role, rest = m.group(1), m.group(2)
        assert 'when' not in rest, rest[:60]
        return f'permit (principal, {rest}resource)\nwhen {{ principal.roles.contains("{role}") }};'
    block = re.sub(r'permit \(principal in Akr::Role::"([A-Z0-9]+)",\s*(.*?)resource\);', head, block, flags=re.S)
    # 2. a || chain of role tests  ->  containsAny
    def chain(m):
        roles = re.findall(ROLE, m.group(0))
        return 'principal.roles.containsAny([' + ', '.join(f'"{r}"' for r in roles) + '])'
    block = re.sub(ROLE + r'(?:\s*\|\|\s*' + ROLE + r')+', chain, block)
    # 3. negated single test
    block = re.sub(r'!\(' + ROLE + r'\)', lambda m: f'!principal.roles.contains("{m.group(1)}")', block)
    assert 'Akr::Role' not in block, block
    return block


out = {}
for pp in ('05', '10', '11', '12'):
    p = PLAN / f'phase-{pp}.md'
    s = p.read_text()
    n_before = s.count('in Akr::Role')
    s = re.sub(r'```cedar\n.*?```', lambda m: fix_cedar(m.group(0)), s, flags=re.S)
    assert s.count('in Akr::Role') == (1 if pp == '13' else 0), pp
    out[p] = (s, n_before)

# aal comparisons (phase 03 writes context.aal != "aal2")
p6 = PLAN / 'phase-06.md'
s6 = p6.read_text()
assert s6.count('&& context.aal >= 2 }') == 1
s6 = s6.replace('&& context.aal >= 2 }', '&& context.aal == "aal2" }')
s12, n12 = out[PLAN / 'phase-12.md']
assert s12.count('when { context.aal < 2 };') == 1 and s12.count('with `context.aal >= 2`') == 1
s12 = s12.replace('when { context.aal < 2 };', 'when { context.aal != "aal2" };')
s12 = s12.replace('with `context.aal >= 2`', 'with `context.aal == "aal2"`')
out[PLAN / 'phase-12.md'] = (s12, n12)

NOTE = """### 7.N Cedar form (gap item (a))
The Cedar policies of this phase are corrected in place: a role test is `principal.roles.contains("X")` (several
roles: `principal.roles.containsAny([...])`) in a `when` clause, never `principal in Akr::Role::"X"`, which phase 03's
schema does not model ({n} occurrences rewritten; the `@role` annotations are unchanged).{aal}"""
AAL = (' The AAL test compares the string: `context.aal == "aal2"` (or `!= "aal2"` in a forbid), as phase 03 writes '
       'it; `context.aal` is not a number.')

final = {}
for p, (s, n) in out.items():
    aal = AAL if p.name == 'phase-12.md' else ''
    final[p] = add_section7(s, CHDR, [NOTE.format(n=n, aal=aal)])
final[p6] = add_section7(s6, CHDR, ["""### 7.N AAL comparison
The policy `readiness.mlro_declares` is corrected in place: `context.aal == "aal2"` (phase 03 `Ctx.aal` is a string,
`"aal1"` or `"aal2"`), not `context.aal >= 2`."""])

for p, s in final.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
