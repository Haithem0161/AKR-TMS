#!/usr/bin/env python3
"""Apply gap pass 1, run 8b (coverage: VB and VD market IDs) after the lead's checks.

Source: tools/agent/gap1-run8b.md (Haiku draft); decisions: tools/agent/gap1-run8b-verdict.md.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'
DOCS = ROOT / 'docs'

IMPL = {
    '05': 'VD-02 (each mapping from the bank\'s own dictionary, comments and samples, under change control), VD-05 '
          '(Db2 and Sybase through ODBC), VD-11 (PostgreSQL, SQL Server, Oracle and ODBC adapters; Oracle not assumed)',
    '07': 'VB-49 (the pre-submission goAML validator: schema and Office rules, failures block)',
    '10': 'VD-12 (bank fields and tables as extension attributes of the adapter pack)',
    '13': 'VB-05 (features and profiles for counterparties seen only in transactions)',
    '15': 'VD-36 (the rating model mapped to the CBI risk levels)',
    '16': 'VB-17 (the scenario studio with replay, approval and rollback), VB-23 (champion and challenger in shadow '
          'beside the live version), VB-24 (the replay of at least 12 months of history before go-live), VB-48 '
          '(scenarios as governed, versioned, tested documents)',
    '17': 'VB-33 (the evidence snapshot with producing versions and the decision record per alert)',
    '18': 'VB-31 (escalation, report, freeze and exit decided by a named person)',
    '21': 'VD-17 (goAML STR, TFR and CTR output validated with a quality grade)',
    '24': 'VD-18 (the D2 clause map with the read-only assessor grant and the assessor pack)',
}
PART = {
    '00': 'VB-25 (the profile S compose stack on premises, no big-data cluster), VB-28 (SBOMs with maintainer and '
          'country)',
    '05': 'VD-01 (ICS BANKS on Oracle as the first adapter pack), VD-06 (the character-set round trip per source), '
          'VD-33 (engine facts from read-only discovery)',
    '06': 'VB-38 (the CBI register and the coverage gap report), VD-39 (the versioned CBI register as the coverage '
          'report source)',
    '10': 'VD-10 (lineage on every canonical row), VD-21 (stable canonical ids and mapping change requests with '
          'replay), VD-25 (the channel table and ISO 20022 party fields)',
    '11': 'VD-04 (the record-text reader for Transact XML records), VD-13 (MT103 and pacs.008 messages mapped)',
    '13': 'VB-14 (the robust per-segment baseline on closed days)',
    '14': 'VB-01 (the known-answer harness with recall gates and the evidence level per hit), VB-37 (thresholds per '
          'list), VD-27 (Kurdish letters and the Kurdish lexicon), VD-32 (recall at each alert budget)',
    '16': 'VB-07 (replay with alert counts and overlap), VB-16 (signed scenario documents with owner and typologies), '
          'VB-36 (the seeded known-answer suite), VB-45 (the per-bank replay summary), VD-16 (the CBI pack with replay '
          'and shadow)',
    '18': 'VB-03 (the case workspace with cases and alerts of the party), VB-06 (the closed outcome set), VB-32 (the '
          'seven-criteria checklist), VB-34 (the decision checklist per case)',
    '19': 'VB-02 (canonical network edges), VB-35 (the forced low-score sample with an independent reviewer), VB-44 '
          '(RFI kinds with four-eyes release)',
    '21': 'VB-43 (draft, approval, filing, feedback register and frozen versions), VD-40 (the yearly MLRO facts pack)',
    '23': 'VD-19 (the seven-year floor, configurable upward)',
    '24': 'VB-18 (the import of the current system\'s alerts for the parallel run), VB-21 (the command-centre tiles), '
          'VD-22 (the parallel run beside the current system)',
    '25': 'VB-52 (on-premises runtime with signed offline bundles), VD-20 (no call-home; offline signed bundles), '
          'VD-38 (no licence server or call-home)',
}
NOTC = {
    '05': 'VD-07 (the bank\'s tool inventory meeting; bank process, DR-403)',
    '11': 'VD-03 and VD-34 (vendor read stores and warehouse sources, P3), VD-24 (payment-quality monitor, DR-400)',
    '13': 'VB-47 (digital-channel signals, DR-397)',
    '17': 'VB-22 (auto-close guardrails, DR-399)',
    '19': 'VB-12 (link-graph motif engine), VB-13 (shared fraud and AML mule layer); DR-403',
    '24': 'VB-51 (automation claims register; no automation feature in this release, DR-403)',
    '25': 'VB-26, VD-08, VD-09, VD-14, VD-15, VD-26, VD-28, VD-29, VD-30, VD-31, VD-35 and VD-37 (vendor, commercial and '
          'process items, DR-403)',
    '27': 'VB-04, VB-08, VB-11, VB-15, VB-19, VB-20 and VB-27 (model items; P2, DR-403)',
    '28': 'VB-09, VB-10, VB-29, VB-30, VB-39, VB-40, VB-41, VB-42, VB-46, VB-50 and VB-53 (assistant and agent items; '
          'P2, DR-403)',
    '29': 'VD-23 (remittance pre-check; P2, DR-403)',
}

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
anchor = [l for l in rs.split('\n') if l.startswith('| DR-402 |')]
assert len(anchor) == 1
DR = ("| DR-403 | 2026-10-10 | Run 8b: no gap. Outside this release: the model items (VB-04, VB-08, VB-11, VB-15, "
      "VB-19, VB-20, VB-27), the assistant and agent items (VB-09, VB-10, VB-29, VB-30, VB-39 to VB-42, VB-46, VB-50, "
      "VB-53), the link-graph motif engine (VB-12), a shared fraud and AML mule layer (VB-13), auto-close guardrails "
      "(VB-22), digital-channel signals (VB-47), vendor read stores and warehouse sources (VD-03, VD-34, P3), the "
      "remittance pre-check (VD-23, P2), vendor, commercial and process items (VB-26, VB-51, VD-07 to VD-09, VD-14, "
      "VD-15, VD-26, VD-28 to VD-31, VD-35, VD-37), and the missing parts of the run 8b Partly rows | Each is on no "
      "P0/P1 line or is P2 or P3 in ROADMAP (gap pass 1, run 8b) |")
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + DR + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
