#!/usr/bin/env python3
"""Apply gap pass 1, run 5a (coverage: C, E, RP, S, TS) after the lead's checks.

Source: tools/agent/gap1-run5a.md (Haiku draft); decisions: tools/agent/gap1-run5a-verdict.md.
Header lines are added at the end of each phase's "Requirements carried" block, one line per class, so that
tools/coverage.py reads them (a line with the words "not carried" counts as Not carried).
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'

HDR = ('Gap pass 1 (2026-10-10), run 5a (coverage of C, E, RP, S and TS IDs), from tools/agent/gap1-run5a.md; '
       'lead-verified (tools/agent/gap1-run5a-verdict.md).')

IMPL = {
    '00': 'C-33 (the CBS egress rule: only akr-ingestd reaches the CBS listeners, host-enforced)',
    '05': 'TS-05 (generated SQL proved by `sqlparser` as one read-only SELECT over approved objects), TS-06 (blocking '
          'drivers on a pool sized by the load budget)',
    '10': 'C-17 (lifecycle events from initiated to cancelled), C-18 (booking, value and business dates, the source local '
          'timestamp and the bank zone), C-49 (monthly partitions, wrapped-key encryption, keyed match hashes)',
    '11': 'C-39 (the CT4 manifest contract with files, rows and checksums), C-14 and C-41 (section 7+), C-42 (the gateway '
          'database as a second source, section 7+)',
    '18': 'E-07 (suspicion data on a need-to-know grant, access events on every read)',
    '22': 'RP-13 (the internal chain: acknowledgement with the tipping-off reminder, decision notice, liaison report by '
          'the 10th), TS-21 (in-app and the bank SMTP relay only)',
}
PART = {
    '05': 'C-31 (the credential in OpenBao KV, read only by akr-ingestd), C-32 (TLS trust against the bank CA and the '
          '`tls_failed` check), C-40 (the catalogue run: tables, columns, partitions, character set, grants)',
    '06': 'S-19 (external needs per scenario and the connected-feed registry)',
    '07': 'RP-08 (several active profiles, one per code)',
    '08': 'TS-35 (the frontend stack, the table, the bidi helpers and the chart and graph spikes)',
    '09': 'S-14 (country risk levels with expiring overrides)',
    '10': 'C-37 (field references through the adapter pack dictionary), C-43 (the payment-link table and its mapping '
          'path; match rules and rung L4 are P2, DR-394), C-44 (IBAN checksum and decode), C-45 (the ISO-shaped payment '
          'model), C-53 (survivorship rules; merge and split are P2, DR-394), RP-10 (effective-dated code maps under '
          'change requests), S-11 (the counterparty table and its link on each transaction), S-16 (cleared and '
          'available balances), S-18 (the facility table), S-21 (cash events with denominations; the operator on each '
          'transaction)',
    '11': 'C-05 (ingest-lag measures and freshness classes), C-11 and TS-08 (the opt-in change-capture sidecar behind the '
          'signed agreement), C-12 (the change-capture agreement gate), C-19 (the closed business day gate; a late '
          'posting is a new row), TS-11 (the development fixture of the load path)',
    '13': 'S-17 (the exclusion of government payroll pool accounts), S-36 (the `dormant_days` feature)',
    '16': 'S-25 (CBI references on every rule), S-27 (the cash family and linked-party aggregation), S-29 (restricted '
          'visibility of the public-funds family)',
    '18': 'RP-12 (decision outcomes with reasons and the UTR-to-STR link), TS-20 (the per-case event chain and the note '
          'guard)',
    '19': 'E-01 (the authority-request register with its deadline), S-34 (shared-contact edges), TS-18 (the network edge '
          'table)',
    '20': 'E-02 (freeze tasks and verification levels from the canonical copy)',
    '21': 'E-03 (identifiers, names and the L0 completeness check), E-04 (filing interval metrics), E-05 (the FIU '
          'feedback register and the calibration sample), E-06 (CTR batches at USD 10,000 and the freeze letters)',
}
NOTC = {
    '05': 'C-34 (native audit on the read account is the bank DBA\'s and SIEM\'s work)',
    '09': 'S-15 (governed reference registers, P2, DR-380)',
    '11': 'S-20 (optional channel and card-processor links, P2, DR-394)',
    '16': 'S-28, S-30, S-33 (pass-through engine, front-company score, NPO scenarios: P2, DR-394)',
    '21': 'RP-35 (the officer registry and its notices are the bank\'s process, DR-394)',
    '24': 'S-35 (employee-risk view, P2, DR-394)',
}

P11 = [
"""### 7.N Source formats accepted from this phase (C-14, DR-393)
Phase 10's compile step 3 refuses every format but `relational` and leaves the others to this phase. From this phase
on it also accepts:
- `landing_file` for a source with `delivery = files`: the columns are the list pinned in the adapter pack (section
  3.4, extract files step 3);
- `t24_xmlrecord`: an approved object with one key column and one record-text column, both named in the mapping's
  `source` block; fields are read with phase 10's `XmlField` transform (DSL `xml_field`);
- `t24_multivalue`: the same object shape, values split with phase 10's `MultivalueSplit`;
- `iso20022` and `mt` (next subsection).
The source read is the keyset statement of section 3.1 over the key column: no XML function and no predicate beyond
the bound reaches the CBS (C-14). The record text lands unchanged in the raw rows and is decoded in akr-ingestd. Any
other value stays refused with `format_not_in_phase`. Test: a fixture record decodes to the expected fields, and a
mapping with an unknown format is refused.""",

"""### 7.N Payment messages in ISO 20022 and MT (C-41, DR-393)
1. Source shape: an approved object, in the CBS or in the payment-gateway database (next subsection), with a key
   column and a raw message column. The mapping has `source.format = iso20022` or `mt` and entity `payment_message`;
   `payment_message_party` is its own mapping over the same object.
2. A pure parser module (no IO) beside phase 10's transform library parses the raw text into the elements the
   mapping's `from` entries name (an ISO 20022 element path, or an MT field tag). Each parser has golden fixtures and a
   fuzz target that CI runs for a bounded time.
3. Parser version: each parser has a version constant that the compiler writes into the compiled mapping, so the
   `mapping_version_id` of every canonical row names the parser version. No column is added; a parser upgrade is a new
   mapping version through phase 10's lifecycle.
4. The raw message is kept in `canon.payment_message.raw_enc` where the source holds it (phase 10).
5. A message that does not parse is held as a quarantine row of kind `row_failed`, reason `message_parse_failed`
   (section 3.3), never written to canon.
6. Tests: a pacs.008 and an MT103 fixture each give the expected `payment_message` and party rows; a truncated message
   lands in quarantine; the fuzz target runs in CI.""",

"""### 7.N Payment-gateway database as a second source (C-42)
A payment-gateway database is its own `admin.sources` row with its own grant proof and load budget (phase 05), its
own watermarks (section 3.1) and its own freshness state (section 3.7). Its approved objects map to
`canon.payment_message`, `canon.payment_message_party` and `canon.payment_link` through relational mappings (phase 10
catalogue) or through the formats of the subsection above. A gateway database on an engine phase 05 supports needs no
new adapter; a halt of the gateway source does not change the CBS source's freshness.""",
]

P12 = [
"""### 7.N Deferred rungs (Open doubts closed, DR-394)
MODULES M7 puts four rungs in P1 (receipt, counts, control totals, GL tie-out). Rungs L2, L4, L5 and L6 are P2. For L4,
the payment-link table of phase 10 is filled only by a mapping from a source that holds the link; match rules and the
routing of unmatched items to L4 (C-43) are P2. The Open doubts item on the deferred rungs is closed.""",
]

TARGETS = [('11', HDR, P11), ('12', HDR, P12)]

out = {}


def load(pp):
    p = PLAN / f'phase-{pp}.md'
    return p, out.get(p, p.read_text())


for pp, hdr, blocks in TARGETS:
    p, s = load(pp)
    s = add_section7(s, hdr, blocks)
    out[p] = s

for table, label in ((IMPL, 'implemented here'), (PART, 'partly implemented'), (NOTC, 'not carried')):
    for pp, ids in table.items():
        p, s = load(pp)
        i = s.index('**Requirements carried:**')
        j = s.index('\n\n', i)
        line = f'\n- Gap pass 1 (coverage), {label}: {ids}.'
        if label != 'not carried':
            assert 'not carried' not in line.lower(), line
        s = s[:j] + line + s[j:]
        out[p] = s


def edit(pp, pairs):
    p, s = load(pp)
    for a, b in pairs:
        s = rep(s, a, b)
    out[p] = s


edit('10', [
    ("(phase 11 and the parser phases add them)", "(phase 11 section 7+ adds them, DR-393)"),
    ("Party merge and split stay maker-checker actions of phase 12 (C-53).",
     "Party merge and split are P2 (C-53, DR-394)."),
])
edit('11', [
    ("- No ISO 20022 or MT parser and no payment-gateway source (phases 12 and later under C-41 and C-42).",
     "- No payment tie-out: match rules and rung L4 are P2 (C-43, DR-394). The payment formats and the gateway source "
     "are in section 7+."),
])
edit('06', [
    ("(screening feeds in 13, device and channel data in 26)",
     "(screening feeds in 13; device and channel data are P2, S-20, DR-394)"),
])

r = PLAN / 'research.md'
rs = r.read_text()
anchor = [l for l in rs.split('\n') if l.startswith('| DR-392 |')]
assert len(anchor) == 1
DRS = [
    "| DR-393 | 2026-10-10 | Phase 11 owns the non-relational source formats (`landing_file`, `t24_xmlrecord`, `t24_multivalue`, `iso20022`, `mt`) and the ISO 20022 and MT parsers; the parser version is part of the compiled mapping, so `mapping_version_id` records it on every row | Phase 10 left the formats to phase 11 and the parser phases, and no phase took them, while MODULES M5 puts payment messages in P1 (gap pass 1, run 5a, phase 11 section 7+) |",
    "| DR-394 | 2026-10-10 | Outside this release: payment match rules and reconciliation rungs L2, L4, L5, L6 (C-43, C-25 part); party merge and split (C-53); optional channel and card-processor links (S-20); the pass-through engine, front-company score and NPO scenarios (S-28, S-30, S-33); the employee-risk view (S-35): all P2. The officer registry and its notices (RP-35) are the bank's process | MODULES and ROADMAP put none of them in P0 or P1; they were named by no phase (gap pass 1, run 5a coverage) |",
]
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + '\n'.join(DRS) + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
