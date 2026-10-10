#!/usr/bin/env python3
"""Apply gap pass 1, run 8c (coverage: VC market IDs) after the lead's checks.

Source: tools/agent/gap1-run8c.md (Haiku draft); decisions: tools/agent/gap1-run8c-verdict.md.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'
DOCS = ROOT / 'docs'

IMPL = {
    '02': 'VC-81 (the evidence vault with object lock, per-object hash, signed anchors, retention by class, legal holds '
          'and access events)',
    '05': 'VC-85 (AKR-TMS never writes to the CBS; tasks, approvals and evidence live in AKR-TMS)',
    '14': 'VC-03 (the 12-hour freshness alarm and the one-hour re-screen measure, section 7+), VC-04 (versions staged, '
          'compared with the active version and gated before activation), VC-10 (fuzzy Arabic-to-Arabic token '
          'comparison with alignment), VC-11 (identifier adjustments only when comparable, no credit for a default '
          'date, versioned), VC-12 (the name-frequency evidence floor from the bank base), VC-24 (pair memory and '
          'good-guy rules with approver, expiry and the QA sample; never on freeze lists), VC-31 (every closed alert a '
          'labelled pair; thresholds change only by change request)',
    '15': 'VC-55 (the full shareholder table with percentages, evidence and dates), VC-59 (registry and licence '
          'documents tied to ownership inputs as evidence objects)',
}
PART = {
    '00': 'VC-36 (the licence allow-list in cargo deny)',
    '05': 'VC-86 (the read-only CBS connection and the egress allow-list)',
    '10': 'VC-70 (row lineage on every canonical row), VC-72 (generic data-quality rules and quarantine)',
    '11': 'VC-16 (ISO 20022 and MT parsers by element path or field tag with the raw message kept; fixtures for '
          'pacs.008 and MT103)',
    '12': 'VC-32 (the data-quality register and the placeholder-date rule)',
    '14': 'VC-01 (versioned entries, diffs, delisting by absence and look-backs), VC-02 (national list diffs and '
          'delisting), VC-05 (entry counts and the count-drop block), VC-06 (the look-back on every list change and '
          'changed parties against every active list), VC-07 (candidate retrieval and per-field scoring), VC-08 (the '
          'Iraqi name pipeline), VC-09 (back-transliteration through the lexicon), VC-13 (person documents and vessel '
          'identifiers), VC-14 (name and identifier fields), VC-15 (the UTF-8 check and the truncation flag), VC-23 '
          '(pair memory with change-driven reopening and good-guy expiry), VC-25 (the dry run counting alerts on a '
          'sample), VC-26 (the daily harness and the budget report), VC-28 (the OFAC-emulation oracle with its '
          'difference report), VC-29 (configuration versions per run and change-request approval), VC-30 (four-eyes '
          'and reason codes on decisions), VC-33 (runs, alerts and versions kept), VC-35 (pluggable official list '
          'feeds), VC-37 (the side-by-side Arabic-first alert screen), VC-42 (the cadence check for official lists), '
          'VC-56 (bank list versions with their evidence object), VC-71 (name forms and the mother\'s chain)',
    '15': 'VC-21 (the ownership chain at the 20 percent threshold), VC-39 (ownership inputs linked to evidence '
          'objects), VC-47 (the PEP register with tiers and relatives), VC-48 (kinship confirmed by a person), VC-57 '
          '(the evidence link and date per ownership edge), VC-66 (the controls matrix by level and the review '
          'interval), VC-67 (reviews on a level change), VC-68 (versioned EDD checklist templates), VC-69 (the '
          'checklist template and the completer recorded), VC-79 (the review queue with calendar and event due '
          'bases), VC-80 (triggers for screening, PEP and case events)',
    '19': 'VC-74 (shared contact, shared device and kinship edges), VC-75 (the shared-device edge)',
    '24': 'VC-38 (clock ageing on the command centre), VC-82 (case and correspondent packs with hashes)',
}
NOTC = {
    '02': 'VC-83 (OCR of evidence documents; DR-404)',
    '12': 'VC-19 (payment-format readiness report; DR-404)',
    '14': 'VC-22 (address screening), VC-43 (third-party list copy reconciliation, Q12), VC-44 (vendor delta feeds, '
          'Q12); DR-404',
    '15': 'VC-41 and VC-45 (vendor risk-data connectors and profile snapshots, Q12), VC-51 to VC-54 (adverse-media '
          'sources, feeds, delta runs and entity extraction, Q12), VC-58 (vendor ownership data, Q12), VC-73 and '
          'VC-78 (onboarding and identity-proofing fields from the CBS), VC-77 (recorded verification calls); DR-404',
    '25': 'VC-27, VC-40, VC-46, VC-49, VC-50, VC-60 to VC-65, VC-76 and VC-84 (vendor, commercial, contract and '
          'procurement items, DR-404)',
    '29': 'VC-17, VC-18, VC-20 and VC-34 (payment-time screening, fail policies, name alignment and the Swift '
          'screening feed; P2, DR-404)',
}

HDR = ('Gap pass 1 (2026-10-10), run 8c (coverage of VC market IDs), from tools/agent/gap1-run8c.md; '
       'lead-verified (tools/agent/gap1-run8c-verdict.md).')

P14 = [
    """### 7.N One-hour re-screen after a list activation (VC-03; ROADMAP M9 and its exit criterion; DR-404)
ROADMAP M9 asks for a full re-screen within one hour of a list update, and its exit criteria say "At most one hour from
a list update to a full re-screen". The 12-hour freshness rule of section 3.2 is unchanged; this adds the one-hour
measure and alarm. No table, column, job, timer or route is added.
1. The re-screen of a version is its `lookback` run (section 3.5 run kind 2, queued by `list_activate`): the
   `alert.screening_runs` row with `kind = lookback` and `trigger_ref` = the version id. Its re-screen time is the
   run's `finished_at` minus the version's `activated_at`. A look-back requested later through
   `POST /screening/lookbacks` for the same version does not count.
2. When that run completes, `ScreeningService` records the re-screen time in the histogram
   `akr_list_rescreen_seconds{list}` and adds `rescreen_seconds` (a number) to the payload of
   `screening.run_finished`.
3. `list_stale_sweep` (every 10 minutes, section 4.1) also selects each `active` version whose `activated_at` is more
   than one hour old and that has no `completed` look-back run of step 1. For each, it writes the audit event
   `list.rescreen_late {list_id, version_id}` (resource list_version) once per version, the same way `list.stale` is
   written once per transition, and sets the gauge `akr_list_rescreen_late{list}` to 1. The gauge returns to 0 when
   the look-back completes. A freeze list with the gauge at 1 raises a critical alert. With the 10-minute schedule the
   alarm comes at most 70 minutes after activation.
4. One hour is a fixed constant (`RESCREEN_TARGET`), not a setting: it is the ROADMAP exit criterion, not a bank
   choice.
5. The S3 list status shows the re-screen time of the active version (from its look-back run) next to the list lag
   that the harness report already gives (section 3.9).
6. Section 3.13 gains `list.rescreen_late`; section 4.4 gains the histogram, the gauge and the alert.""",
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

for pp, blocks in (('14', P14),):
    p, s = load(pp)
    st = s.index('## 7+. Gap Additions')
    nx = s.find('\n## ', st + 5)
    assert 'run 8c' not in s[st:nx if nx != -1 else len(s)], pp
    out[p] = add_section7(s, HDR, blocks)

r = PLAN / 'research.md'
p, rs = load(r)
anchor = [l for l in rs.split('\n') if l.startswith('| DR-403 |')]
assert len(anchor) == 1
DR = ("| DR-404 | 2026-10-10 | Run 8c: phase 14 measures the re-screen time from a list activation to the end of its "
      "look-back run and alarms when it passes one hour (`list.rescreen_late`, checked by `list_stale_sweep`; no new "
      "timer, table or route). Outside this release: payment-time screening, fail policies, name alignment and the "
      "Swift screening feed (VC-17, VC-18, VC-20, VC-34, P2), the payment-format readiness report (VC-19), address "
      "screening (VC-22), vendor list copies, delta feeds, risk-data and ownership connectors (VC-41, VC-43 to VC-45, "
      "VC-58, ROADMAP Q12), adverse-media sources and runs (VC-51 to VC-54, Q12), onboarding and identity-proofing "
      "fields and recorded calls (VC-73, VC-77, VC-78), OCR (VC-83), vendor, commercial, contract and procurement "
      "items (VC-27, VC-40, VC-46, VC-49, VC-50, VC-60 to VC-65, VC-76, VC-84), the OFAC 50 percent aggregation and "
      "derived-designation records (VC-21, VC-57), and the missing parts of the run 8c Partly rows | The one-hour "
      "re-screen is a ROADMAP M9 milestone and exit criterion; the existing sweep checks it, so no timer owner kind or "
      "cancel path is needed. Each other item is on no P0/P1 line, or is P2 or external in ROADMAP (gap pass 1, run "
      "8c) |")
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + DR + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
