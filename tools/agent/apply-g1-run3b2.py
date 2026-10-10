#!/usr/bin/env python3
"""Gap pass 1, run 3b addendum: the phase 14 name-search access event (lead check from the run 3b launch note)."""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'
CHDR = ('Gap pass 1 (2026-10-10), run 3b: correction found by the lead while checking phase 14 against phase 02 '
        '(tools/agent/gap1-run3b-verdict.md); lead-verified.')
B = ["""### 7.N Access event of the name search (DR-383)
Phase 02's `audit.access_event.action` is closed (`view`, `search_hit`, `export`, `print`, `download`, `unmask`) and has
no `search`. Section 3.12 step 5 writes one access event per search with action `search_hit`, resource type
`screening_search`, `resource_id` = the id of the `screening.searched` audit event (a fresh UUID v7, so the two rows
join), tier `restricted`, the purpose code from `X-Access-Purpose` and no `subject_party`. It is written through phase
02 `AccessScope` before the body (DR-46); a search with no candidate still writes it. The route carries
`x-akr-access: screening_search`. The list entries shown are not access events of their own (they are not bank
records)."""]

p = PLAN / 'phase-14.md'
s = add_section7(p.read_text(), CHDR, B)
s = rep(s, '5. The access event `search` is written before the body (DR-46).',
        '5. The access event (`search_hit`, resource type `screening_search`, section 7+) is written before the body (DR-46).')
r = PLAN / 'research.md'
rs = r.read_text()
anchor = [l for l in rs.split('\n') if l.startswith('| DR-382 |')]
assert len(anchor) == 1
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + "| DR-383 | 2026-10-10 | The phase 14 name search writes one access event per search: action `search_hit`, resource type `screening_search`, `resource_id` = the `screening.searched` audit event id, tier `restricted` | Phase 02's action list is closed and has no `search`; an existing value fits, and the query, not the public list entry, is what must be traced (gap pass 1, phase 14 section 7+) |\n")
p.write_text(s)
r.write_text(rs)
print('wrote phase-14.md, research.md')
