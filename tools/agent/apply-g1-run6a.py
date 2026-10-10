#!/usr/bin/env python3
"""Apply gap pass 1, run 6a (coverage: L) after the lead's checks.

Source: tools/agent/gap1-run6a.md (Haiku draft); decisions: tools/agent/gap1-run6a-verdict.md.
Header lines only (all seven drafted gaps rejected); IDs are appended to an existing line of the same class.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'

IMPL = {
    '14': 'L-36 (no amount, materiality or exposure threshold on any screening; freeze lists excluded from good-guy rules '
          'and bulk decisions)',
    '16': 'L-26 (client-level and linked-party aggregation through `LinkedPartyResolver`)',
    '18': 'L-54 (decision four-eyes by Cedar, the workflow and a database CHECK; generic text refused; the QA queue is '
          'phase 19)',
}
PART = {
    '02': 'L-68 (the language tag on every evidence object)',
    '06': 'L-08 (per-input null share and source reference in the readiness view), L-25 and L-42 (the CBI baseline '
          'register with each item mapped to its scenarios)',
    '07': 'L-12 (pre-submission identifier and shape rules)',
    '10': 'L-06 (drift findings on new, changed and relabelled code values), L-07 (versioned mappings under change '
          'requests; schema and code-list drift), L-15 (the cash flag set by the code map), L-38 (quality flags on payment '
          'parties), L-39 (rejected and returned payment statuses), L-50 (the CBS CDD level on each customer)',
    '12': 'L-14 (exclusions with reasons and volume counts in the coverage ledger)',
    '13': 'L-49 (segment assignment from observed attributes)',
    '14': 'L-32 (the known-answer harness with its corpus and dated runs), L-33 (subject kinds on each screening alert), '
          'L-34 (the insufficient-data outcome), L-35 (pair-keyed good-guy rules with expiry, change request and the '
          'freeze-list refusal)',
    '15': 'L-09 (a customer without a rating shown as a control failure), L-28 (declared turnover and counts beside the '
          'actuals), L-37 (the geography block from nationality and residence), L-44 (the completeness cap and the '
          'degraded flag), L-45 (overdue reviews and the exit-candidate state)',
    '16': 'L-13 (the run ledger with its parameter set; matched transactions on each alert), L-20 (reach by segment in '
          'replays), L-21 (segment, risk and currency overrides of parameters), L-22 (unit and seeded cases that block '
          'activation), L-23 (the scenario record and its go-live gates), L-29 (per-scenario silence alarms), L-51 '
          '(bounded limit parameters), L-61 (the dated passing-test gate for scenarios)',
    '17': 'L-27 (the weighted score and priority), L-31 (the data latency clock on each alert), L-52 (the '
          'work-in-progress limit per analyst)',
    '18': 'L-55 (the opened-view record and access events)',
    '19': 'L-19 (the monthly closed-alert QA sample), L-46 (authority requests and internal reports linked, with the '
          're-rating trigger and overdue notices), L-48 (shared phone and address links as network edges), L-53 (reviewer '
          'rates and bulk reopen), L-62 (the authority-request register and its links)',
    '21': 'L-11 (the daily cash recount with control totals)',
    '24': 'L-24 (the review calendar with amber and red), L-30 (validator separation and replay results in validation '
          'packs), L-56 (the board pack from system facts), L-57 (registers, decisions and exclusions as read views), '
          'L-58 (the findings register with owner, status and repeat flag), L-59 (the warning register kind with owner '
          'and status), L-60 (pack kinds with manifests and the certification pack)',
}
NOTC = {
    '15': 'L-67 (staff and branch behaviour analytics, P2, DR-394)',
    '16': 'L-63 and L-66 (pass-through engine and front-company score, P2, DR-394), L-65 (typology exposure index, P2), '
          'L-40 (evasion-language search, P2, DR-397)',
    '24': 'L-64 (correspondent assertion cards, P2 correspondent pack)',
}

out = {}


def load(pp):
    p = PLAN / f'phase-{pp}.md'
    return p, out.get(p, p.read_text())


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
rs = r.read_text()
anchor = [l for l in rs.split('\n') if l.startswith('| DR-396 |')]
assert len(anchor) == 1
DR = ("| DR-397 | 2026-10-10 | Outside this release (P2): the daily value metric and alert-by-default for customers without "
      "a rating (L-09; the control failure itself is built), geofences and login or device location evidence (L-37), "
      "payment message-integrity analytics by operator, branch, customer and correspondent with cover-payment and repair "
      "views (L-38), evasion-language search (L-40), the low-tier account register with tier limits (L-50) and the "
      "cross-unit search workbench for authority requests (L-62). L-36 is met: no screening has an amount or materiality "
      "threshold; it never meant a zero name-match score | No MODULES or ROADMAP feature line puts them in P0 or P1; the "
      "run 6a draft argued from module tiers only (gap pass 1, run 6a coverage) |")
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + DR + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
