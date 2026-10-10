#!/usr/bin/env python3
"""Apply gap pass 1, run 7b (coverage: F, B and U) after the lead's checks.

Source: tools/agent/gap1-run7b.md (Haiku draft); decisions: tools/agent/gap1-run7b-verdict.md.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'
DOCS = ROOT / 'docs'

IMPL = {
    '03': 'B-04 (roles for the three lines of defence; internal audit read-only; the MLRO always has access)',
    '12': 'B-01 (daily reconciliation of counts and sums against the CBS, breaks shown as data-quality issues)',
    '14': 'F-13 (customers, owners and payment parties screened against the UN, national and optional lists, refreshed '
          'within 12 hours, look-back on every list change, decisions with coded reasons), U-02 (a parse failure fails '
          'the version with an incident and the last good version stays), U-03 (alias quality classes, birth dates with '
          'precision, documents by type, name chains, original-script keys), U-09 (per-version fetch and load records, '
          'diff counts, look-back runs and alert outcomes)',
    '15': 'F-02 (every factor weighed and explained factor by factor, controls by level), F-07 (review clocks by level; '
          'the profile-breach trigger from phase 17 on), F-10 (PEP register for customers and owners with relatives and '
          'associates, EDD sign-off, source-of-wealth evidence, the former state), B-10 (CDD documents and every review '
          'outcome recorded), B-13 (the rating is a one-way feed that blocks nobody; the light check as compensating '
          'monitoring)',
    '16': 'F-05 (scenarios for complex, unusual and purposeless patterns), B-02 (every parameter with its basis, bounds '
          'and bank scenarios 901 to 999), B-05 (internal audit reads the scenario card, metrics and change history), '
          'B-07 (scenarios combine CDD data, risk-level overrides and register typologies)',
    '18': 'F-16 (need-to-know case access, the tipping-off review, contact only for normal-tier parties)',
    '23': 'F-17 (records kept after the relationship ends, with the evidence snapshot, decisions and notes)',
    '24': 'B-11 (the board pack builder with counts through each source predicate)',
}
PART = {
    '09': 'F-14 (the country table with effective dates driving floors and re-scores)',
    '14': 'B-09 (look-back re-screens on every list change), U-01 (fetch through the egress proxy, SHA-256, versions, '
          'diffs and the gate before activation), U-04 (one list model with kind, program and activation time), U-05 '
          '(look-back runs and the list lag from publication to activation), U-07 (the UN list with its 1518 '
          'designations and the national list), U-08 (UN designations, vessel entries and PF scenarios)',
    '15': 'F-03 (CDD depth set per level by the controls rows), F-08 (missing mandatory data caps the best level), F-09 '
          '(ownership and control chains with percentages and legal-arrangement parties), F-22 (the onboarding channel '
          'factor), B-03 (declared figures beside the actuals on the customer view)',
    '16': 'F-04 (profile scenarios against declared figures, own history and peers), F-06 (aggregation across linked '
          'parties, accounts and days), F-24 (TRADE, PF and TF scenario families), B-06 (product and channel codes in '
          'scenario filters)',
    '18': 'F-15 (cases with clocks and the STR outcome at any amount), B-08 (profile, rating and transaction panels on '
          'the case)',
    '20': 'U-06 (the freeze service, the operations task, the replica check and release after delisting)',
    '24': 'F-19 (the command centre, the training register and the internal-audit role), F-21 (the review calendar '
          'with the scenario card and coverage matrix of phase 16), F-23 (the assessor pack and the productive-alert '
          'metric)',
}
NOTC = {
    '03': 'F-18 (group view across subsidiaries; further banks are separate installs, DR-401)',
    '15': 'F-01 (the bank\'s own enterprise-wide risk assessment, DR-401)',
    '27': 'F-20 (explainable ML; P2, DR-401)',
    '29': 'F-12 (payment-transparency checks before payments leave; P2, DR-401)',
    '31': 'F-11 (correspondent-bank register), B-12 (correspondent annex); P2 correspondent pack, DR-401',
}

HDR = ('Gap pass 1 (2026-10-10), run 7b (coverage of F, B and U IDs), from tools/agent/gap1-run7b.md; '
       'lead-verified (tools/agent/gap1-run7b-verdict.md).')

P17 = [
"""### 7.N The profile.breach topic (F-07; TR-11, DSL.md C.5; DR-401)
No phase wrote `profile.breach`, the source of trigger TR-11 (declared against actual breach) in the shipped trigger
library; phase 15 section 3.3 leaves the topic to the phase that produces it. This phase writes it and registers its
first handler, the outbox rule of DR-375.
1. Section 3.1, step 12: when the outcome is `New`, the producing scenario version has `logic.kind = profile` and one
   of its `expected[]` entries has `source = declared` (phase 16 section 3.1), the same transaction also writes outbox
   `profile.breach {party_id}`, where `party_id` is the subject party of the case of step 5. `Duplicate` and `Amended`
   outcomes write nothing, so a replayed batch writes no second row.
2. The payload carries the party id only: no alert id, scenario, measure or tier.
3. Handler (worker; phase 15 trigger table, section 3.3): the TR-11 trigger of the library, stored with the neutral
   `trigger_code` `TRG_REVIEW_REQUESTED` (phase 15 section 3.6). An alert is suspicion data, so the TR-11 id and its
   cause are never stored for a non-M reader. The handler enqueues and never re-scores in the consumer's transaction.
4. Section 4.2 gains the row `profile.breach` | `{party_id}` | the phase 15 trigger.
5. Tests: a profile scenario with a declared expectation creates one alert, one outbox row and one trigger job with
   `TRG_REVIEW_REQUESTED`; the replayed batch returns `Duplicate` and writes no second row; a profile scenario with
   peer expectations only writes none.""",
]
P15 = [
"""### 7.N The profile.breach handler (phase 17)
From phase 17 on, `profile.breach` (TR-11) is written by phase 17's alert intake, which registers this phase's
handler for it with the neutral `trigger_code` `TRG_REVIEW_REQUESTED` (phase 17 section 7+).""",
]


out = {}


def load(p):
    if isinstance(p, str):
        p = PLAN / f'phase-{p}.md'
    return p, out.get(p, p.read_text())


def edit(p, pairs):
    p, s = load(p)
    for a, b in pairs:
        s = rep(s, a, b)
    out[p] = s


for pp, blocks in (('17', P17), ('15', P15)):
    p, s = load(pp)
    st = s.index('## 7+. Gap Additions')
    nx = s.find('\n## ', st + 5)
    assert 'run 7b' not in s[st:nx if nx != -1 else len(s)], pp
    out[p] = add_section7(s, HDR, blocks)

for table, label in ((IMPL, 'implemented here'), (PART, 'partly implemented'), (NOTC, 'not carried')):
    for pp, ids in table.items():
        if label != 'not carried':
            assert 'not carried' not in ids.lower(), ids
        p, s = load(pp)
        i = s.index('**Requirements carried:**')
        j = s.index('\n\n', i)
        prefix = f'\n- Gap pass 1 (coverage), {label}: '
        if prefix in s[i:j]:
            k = s.index(prefix, i)
            e = s.index('\n', k + 1)
            assert e <= j and s[e - 1] == '.', s[k:e]
            s = s[:e - 1] + '; ' + ids + '.' + s[e:]
        else:
            s = s[:j] + prefix + ids + '.' + s[j:]
        out[p] = s

r = PLAN / 'research.md'
p, rs = load(r)
anchor = [l for l in rs.split('\n') if l.startswith('| DR-400 |')]
assert len(anchor) == 1
DR = ("| DR-401 | 2026-10-10 | Run 7b: phase 17's alert intake writes `profile.breach {party_id}` for a new alert of a "
      "profile scenario with a declared expectation and registers phase 15's TR-11 handler (neutral `trigger_code` "
      "`TRG_REVIEW_REQUESTED`). Outside this release: the bank's enterprise-wide assessment (F-01), the subsidiary group "
      "view (F-18), explainable ML (F-20), payment-transparency checks (F-12), the correspondent register and annex "
      "(F-11, B-12), and the missing parts of the Partly rows: SDD lifted on suspicion (F-03), the product-purpose "
      "comparison (F-04), occasional-customer aggregation (F-06), a pending-verification state (F-08), the senior "
      "managing official, nominee and verification fields (F-09), blocking rules (F-14), an attempted-transaction STR "
      "path (F-15), the identification method (F-22), a product-level view (B-06), a seeded PEP list (B-09), feed "
      "schema and redirect checks and hourly polling (U-01), cross-list de-duplication (U-04), the frozen amounts and "
      "exemptions (U-06), the 1518 family and ownership rules (U-07), the Iran-nexus flag (U-08) | No phase wrote the "
      "TR-11 topic while phase 15 claims the trigger library (MODULES M10 P1); the excluded parts are on no P0/P1 line "
      "(gap pass 1, run 7b) |")
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + DR + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
