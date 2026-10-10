#!/usr/bin/env python3
"""Apply gap pass 1, run 6b (coverage: T) after the lead's checks.

Source: tools/agent/gap1-run6b.md (Haiku draft); decisions: tools/agent/gap1-run6b-verdict.md.
Header lines (IDs appended to an existing line of the same class), the counterparty resolver (phase 13 section 7+,
phase 18 section 7+), the branch-first switch fixed at false (phase 09), Open doubts closed (phases 09, 13, 17),
DATA-MODEL, roadmap and API.md rows, DR-398 and DR-399.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'
DOCS = ROOT / 'docs'

IMPL = {
    '14': 'T-27 (Arabic-aware keys for comparison and blocking; no record merge, and a differing national ID gives a '
          'decision code)',
    '15': 'T-30 (versioned rating models with the factor catalogue, maker-checker, shadow runs and sensitivity), T-32 '
          '(overrides with reason code and expiry, a second approver for downward overrides, override analytics), T-34 '
          '(the per-customer rating explanation)',
    '16': 'T-01 (scenario documents compiled to parameterised SQL with incremental, sweep and replay runs), T-40 (the '
          'latency class on every scenario document; one definition run on the plan of its class)',
    '17': 'T-03 (the frozen evidence snapshot on every alert), T-47 (the evidence-first alert page; thresholds never shown '
          'to branch roles)',
    '19': 'T-24 (kinship edges hidden unless asked for and logged when shown), T-38 (hibernations tied to a four-eyes '
          'no-report decision with envelope and expiry, the hit ledger, QA sampling of hits and the regret flag; '
          'evaluation never skipped)',
    '22': 'T-06 (the branch observation log with the eight indicator groups)',
}
PART = {
    '11': 'T-41 (the watermark registry with the re-read window; rows the source deletes are kept)',
    '13': 'T-08 (deviations on monthly profiles against own history and peers), T-09 (the segment membership history), '
          'T-17 (month-end feature values keyed by as-of date), T-18 (robust median-MAD segment statistics), T-29 (the '
          'counterparty resolver for transfer counterparties with analyst confirmations, section 7.5), T-42 (daily '
          'event-time features compiled by the same compiler), T-65 (cash-share features and percentile statistics by '
          'segment, currency and period)',
    '14': 'T-28 (the known-answer harness with recall gates)',
    '15': 'T-31 (event triggers with a fall held until the review completes), T-33 (the distribution sanity band and '
          'sensitivity counts)',
    '16': 'T-16 (replay on as-of history with the look-ahead flag, precision and miss-rate estimates from samples), T-45 '
          '(the CBI coverage matrix), T-61 (above- and below-the-line sampling plans with seeded cases)',
    '17': 'T-35 (one open case per party with every contributing alert and scenario), T-36 (priority class and a score '
          'from displayed components, with an age clock)',
    '18': 'T-11 (the controlled closure-reason vocabulary)',
    '19': 'T-12 (QA samples of closures and MLRO decisions with calibration against a golden outcome), T-23 (typed '
          'network edges for shared contacts, devices, transfers, ownership and kinship)',
    '22': 'T-37 (branch RFIs routed to the liaison officer with escalation to the MLRO desk)',
    '24': 'T-05 (the productive-alert and top-scenario tiles), T-59 (tiles with definition, formula and role '
          'visibility), T-60 (period counts of alerts created and dispositioned, productive alerts and reports)',
}
NOTC = {
    '10': 'T-26 (persistent entity resolution with unmerge, P2, DR-394)',
    '16': 'T-39 (continuing-activity and auto-close switches; no auto-close in P1, DR-399)',
    '27': 'T-19 and T-20 (anomaly evaluation suite and graduation path), T-25 (graph neural networks), T-46 (explanation '
          'tests), T-52 to T-58 (cross-bank data model, sharing readiness, CBI reference designs, federation lab, public '
          'benchmarks, synthetic generator, leave-one-bank-out), T-62 (weak labels), T-64 (active-learning queue), T-66 '
          '(maturity ladder); not built by this phase, DR-399',
}

HDR = ('Gap pass 1 (2026-10-10), run 6b (coverage of T IDs), from tools/agent/gap1-run6b.md; lead-verified '
       '(tools/agent/gap1-run6b-verdict.md).')

P13 = [
"""### 7.N Counterparty resolver (T-29, S-11, DR-398)
MODULES M11 puts counterparty resolution in P1 and DSL A.6 names the resolver as a feature writer; this phase owns it.
`canon.counterparty` is mapped and append-only (phase 10), so its `party_id` holds only what the CBS itself links. The
resolver's links live in a table of their own (extends DATA-MODEL s.8), written by akr-worker only.

```sql
-- migration 0044_feat_counterparty_links.sql
-- Phase 13 (gap pass 1). Counterparty to party links. Owner akr_app; retention R2 (follows canon.counterparty).
CREATE TABLE IF NOT EXISTS feat.counterparty_links (
    id                  UUID PRIMARY KEY,
    counterparty_id     UUID NOT NULL REFERENCES canon.counterparty(id) ON DELETE RESTRICT,
    party_id            UUID,                                -- no foreign key: party is bi-temporal
    method              TEXT NOT NULL CHECK (method IN ('own_account', 'confirmed_account', 'name_key', 'none',
                                                        'analyst_confirmed', 'analyst_rejected')),
    decided_by          UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    valid_from          TIMESTAMPTZ NOT NULL,
    valid_to            TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK ((method = 'none') = (party_id IS NULL)),
    CHECK ((method IN ('analyst_confirmed', 'analyst_rejected')) = (decided_by IS NOT NULL)),
    CHECK (valid_to IS NULL OR valid_to >= valid_from)
);
CREATE UNIQUE INDEX IF NOT EXISTS uq_counterparty_link_open ON feat.counterparty_links (counterparty_id)
    WHERE valid_to IS NULL;
CREATE INDEX IF NOT EXISTS idx_counterparty_link_party ON feat.counterparty_links (party_id)
    WHERE valid_to IS NULL AND method IN ('own_account', 'confirmed_account', 'analyst_confirmed');

GRANT SELECT, INSERT ON feat.counterparty_links TO akr_app;
GRANT UPDATE (valid_to) ON feat.counterparty_links TO akr_app;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'feat', 'counterparty_links', c, v.cls::admin.data_classification
FROM (VALUES
  ('personal', ARRAY['party_id']),
  ('internal', ARRAY['id','counterparty_id','method','decided_by','valid_from','valid_to','created_at'])
) AS v(cls, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('feat', 'counterparty_links', 'R2', 'age', 'created_at', 'Counterparty links: kept as long as the counterparty')
ON CONFLICT (schema_name, table_name) DO NOTHING;

INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('counterparty.recheck_days', '30', 'integer', NULL, FALSE,
  '{"ar": "أيام إعادة مطابقة الطرف المقابل", "en": "Days before an unlinked or suggested counterparty is checked again"}'),
 ('schedule.counterparty_resolve', '"0 20 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "مطابقة الأطراف المقابلة", "en": "Counterparty resolution (hourly, bank time)"}')
ON CONFLICT (key) DO NOTHING;
```

A link counts (is read by later phases) only for `own_account`, `confirmed_account` and `analyst_confirmed`.
`name_key` is a suggestion shown to the case team and never read by detection; `none` and `analyst_rejected` mean no
link. The row has no case reference, so the table says nothing about suspicion; the case of an analyst decision is in
the case chain (phase 18 section 7+).

Job `counterparty_resolve` (akr-worker; payload `{}`; coalesce key `counterparty_resolve`; concurrency 1; schedule
`schedule.counterparty_resolve`):
1. Candidates, in chunks of 5,000 by id: counterparties with no row in `feat.counterparty_links`, and those whose open
   row is `none` or `name_key` with `valid_from` older than `counterparty.recheck_days`. An open `own_account`,
   `confirmed_account`, `analyst_confirmed` or `analyst_rejected` row is never replaced by the job.
2. `own_account`: `account_hk` equals `number_hk` or `iban_hk` of a current `canon.account` row (`recorded_to IS
   NULL`); the party is the account's current `owner` (`canon.account_party_role`). Exactly one party is required.
3. `confirmed_account`: `account_hk` equals the `account_hk` of another counterparty whose open row is
   `analyst_confirmed`; the party is that row's party. Exactly one party is required. This is how the resolver learns
   from analyst confirmations.
4. `name_key`: the key of a name in `names` (the `crates/names` function behind `canon.party_name.key`, DR-371) equals
   the `key` of current `canon.party_name` rows of exactly one party, and that party was never rejected for this
   counterparty (`analyst_rejected`, open or closed).
5. Otherwise, or when a step finds more than one party, `none` (counted as `ambiguous` when more than one).
6. One transaction per chunk: close the replaced open row (`valid_to = now()`) and insert the new row
   (`valid_from = now()`). A re-run that finds the same method and party writes nothing.
7. Audit one `counterparty.resolved {own_account, confirmed_account, name_key, none, ambiguous}` per job run (counts
   only).

Depositor names of cash deposits are not resolved: `canon.cash_event` holds no depositor (S-21; phase 16 Open doubts
2), so T-29 is partly implemented. Verification: a fixture with three new counterparties, one whose account hash is an
own account, one whose name key matches one party and one with no match, gives `own_account`, `name_key` and `none`;
a second run writes nothing; after an analyst confirms the second (phase 18 section 7+), a new counterparty with the
same account hash gets `confirmed_account`.""",
"""### 7.N Open doubts on segments closed (T-08, T-09, DR-399)
The Open doubts on segment-of-one and on membership history are decided. Segment of one for large corporates and
government bodies is P2; such customers stay in the root segment. Segment-migration alerts and a KYC refresh on a
cluster change are P2. The membership history of `feat.segment_memberships` is the record a later phase reads.""",
]

P18 = [
"""### 7.N Counterparty link decisions (T-29, DR-398)
The analyst confirms or rejects a counterparty's party link from inside a case; phase 13 section 7+ owns the table and
the resolver.

| Method | Path | Scope | Description |
|-|-|-|-|
| POST | `/api/v1/cases/{id}/counterparty-links` | `queue.work` | Confirm or reject the party of a counterparty on a linked transaction; `Idempotency-Key` (New; `case.link.write`) |

**Decide** (`POST /cases/{id}/counterparty-links`, body `{counterparty_id, party_id, decision: confirm | reject}`;
Cedar `case.link.write`, unchanged): `lock`; a closed case -> 409 `E_CASE_SEALED`. The counterparty must be the
`counterparty_id` of a transaction with an active `transaction` link on this case (else 422 at `/counterparty_id`,
`not_on_case`). The party must have a current row (else 422 at `/party_id`, `unknown_party`). In one transaction: close
the counterparty's open `feat.counterparty_links` row (`valid_to = now()`) and insert `analyst_confirmed` or
`analyst_rejected` with `decided_by`; chain `case.counterparty_decided {counterparty_id, decision}`; audit
`counterparty.link_decided`. Response 200 with the new row. A replay with the same key returns the first response.

`GET /cases/{id}/links` adds, for each `transaction` link, the counterparty id and its open link row (method and
party), the party resolved through `EntityMask` as for a party link. No other route shows a counterparty link, so the
`name_key` suggestions reach the case team only.""",
]

P09 = [
"""### 7.N The branch-first switch is fixed at false (M-14, DR-399)
No phase defines the five conditions that turning on `branch_first_routing` needs; they are P2. The switch is
registered `fixed: true` with the value `false` (section 3.5), so validation step 3 refuses `true` with
`E_INVARIANT_LOCKED`. A later phase that defines the conditions unfixes it.""",
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


for pp, blocks in (('13', P13), ('18', P18), ('09', P09)):
    p, s = load(pp)
    st = s.index('## 7+. Gap Additions')
    nx = s.find('\n## ', st + 5)
    assert 'run 6b' not in s[st:nx if nx != -1 else len(s)], pp
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

# Phase 09: the switch text and the Open doubt.
edit('09', [
    ("`branch_first_routing` (boolean,\n  shipped `false`; turning it on needs the five conditions of M-14, referenced by "
     "id and defined in phases 17 and 22),",
     "`branch_first_routing` (boolean,\n  shipped `false` and `fixed: true` in this release: the five conditions of M-14 "
     "that turning it on needs are P2, section 7+),"),
    ("- **The five branch-first conditions (M-14)** are referenced by id here and defined in phases 17 and 22; this phase "
     "does not define them.",
     "- **The five branch-first conditions (M-14).** Decided in gap pass 1: they are P2 and the switch is fixed at "
     "`false` (section 7+, DR-399)."),
])

# Phase 13: section 1 migration list and the Open doubts.
edit('13', [
    ("Four migrations, `0040_feat_definitions_segments` to `0043_feat_settings_schedules`; `0044` is not used (the shipped",
     "Four migrations, `0040_feat_definitions_segments` to `0043_feat_settings_schedules`; `0044` was not used here and "
     "now holds the counterparty links of section 7+ (the shipped"),
    ("here. T-07 is now carried (lead, review); T-08 and T-09 stay open for phases 16 and 17.",
     "here. T-07 is now carried (lead, review); T-08 and T-09 are decided in section 7+."),
    ("root segment and their statistics are the bank-wide figures. Confirm or assign.",
     "root segment and their statistics are the bank-wide figures. Decided: P2 (section 7+, DR-399)."),
    ("Confirm that the\n  segment-migration alert of T-09 is in phase 17.",
     "Decided: the\n  segment-migration alert of T-09 is P2 (section 7+, DR-399)."),
])

# Phase 17: Open doubts 20 and the later-phase list.
edit('17', [
    (", and the branch-first routing flag with its five\n    conditions belong to phase 22 (branch) and the admin of phase "
     "03 (leave); not built here.",
     " belong to the admin of phase 03 (leave); not built here. The\n    branch-first routing flag stays fixed at `false`; "
     "its five conditions are P2 (phase 09 section 7+, DR-399)."),
    ("- Phase 22 (branch and notifications): M-25, M-66, and the branch routing of M-14.",
     "- Phase 22 (branch and notifications): M-25, M-66 (the branch routing of M-14 is P2, DR-399)."),
])

# Phase 16: Open doubts 21 names the resolver as unowned.
edit('16', [
    ("S-11 and T-29 (counterparty\n    resolution: no phase writes the resolver; the DSL calls it a feature writer, target "
     "phase 13 successor or gap pass 1);",
     "S-11 and T-29 (counterparty\n    resolution: phase 13 section 7+ writes the resolver and phase 18 section 7+ takes "
     "the analyst decisions, DR-398);"),
])

# DATA-MODEL: the feat table and the group 13 migration row.
dm = DOCS / 'DATA-MODEL.md'
p, s = load(dm)
a = [l for l in s.split('\n') if l.startswith('| profiles | customer_id, period, currency')]
assert len(a) == 1
s = rep(s, a[0] + '\n', a[0] + '\n'
        + '| counterparty_links | id, counterparty_id, party_id null, method (own_account, confirmed_account, name_key, '
          'none, analyst_confirmed, analyst_rejected), decided_by null, valid_from, valid_to null, created_at; one open row '
          'per counterparty; no case reference (phase 13 section 7+, DR-398) | none | R2 |\n')
s = rep(s, "feature, segment, peer and profile settings and schedules (0043) | 13 |",
        "feature, segment, peer and profile settings and schedules (0043); counterparty_links with its settings (0044, gap "
        "pass 1) | 13 |")
out[dm] = s

# Roadmap: entities and migrations of phase 13.
rm = PLAN / 'roadmap.md'
edit(rm, [
    ("`segment_stats`, `peer_groups`, `profiles` (schema `feat`; three monthly-partitioned through `feat.create_partition`) |",
     "`segment_stats`, `peer_groups`, `profiles`, `counterparty_links` (gap pass 1) (schema `feat`; three "
     "monthly-partitioned through `feat.create_partition`) |"),
    ("`0043_feat_settings_schedules` (0044 unused) | 13 |",
     "`0043_feat_settings_schedules`, `0044_feat_counterparty_links` (gap pass 1) | 13 |"),
])

# API.md: one row after the case links POST.
api = DOCS / 'API.md'
edit(api, [
    ("| POST | /cases/{id}/links | queue.work | Link a party, account, transaction or external identifier (phase 18, added) |\n",
     "| POST | /cases/{id}/links | queue.work | Link a party, account, transaction or external identifier (phase 18, added) |\n"
     "| POST | /cases/{id}/counterparty-links | queue.work | Confirm or reject the party of a counterparty on a linked "
     "transaction (phase 18 7+) |\n"),
])

r = PLAN / 'research.md'
p, rs = load(r)
anchor = [l for l in rs.split('\n') if l.startswith('| DR-397 |')]
assert len(anchor) == 1
DR = ("| DR-398 | 2026-10-10 | Counterparty resolver (T-29): phase 13 adds `feat.counterparty_links` (migration 0044) and "
      "the hourly job `counterparty_resolve` (own account hash, an account hash already confirmed by an analyst, a "
      "name-key suggestion that detection never reads); phase 18 adds `POST /cases/{id}/counterparty-links` (Cedar "
      "`case.link.write`, the counterparty on a linked transaction). The link row has no case reference | MODULES M11 "
      "lists T-29 and names counterparty resolution in P1; `canon.counterparty` is mapped and append-only, so the links "
      "need their own table; keeping the case in the case chain keeps the table free of suspicion (gap pass 1, run 6b) |\n"
      "| DR-399 | 2026-10-10 | Outside this release (P2): branch-first routing (its five M-14 conditions; the policy-pack "
      "switch is fixed at false), segment of one (T-08), segment-migration alerts and the KYC refresh on a cluster change "
      "(T-09; the membership history is built), the other S-53 metrics, unique catches, overlap, retire-or-keep advice and "
      "funnel stages (T-05, T-60), multi-hop graph features (T-23; the graph is one hop, DR-154), the CBS posting-cadence "
      "disclosure and separate event-time and load-time columns (T-41), continuing-activity and auto-close switches "
      "(T-39), and the model, cross-bank and synthetic-data items T-19, T-20, T-25, T-46, T-52 to T-58, T-62, T-64 and "
      "T-66 | No MODULES or ROADMAP P0/P1 feature line names them; DSL A.7 has no auto-close in P1; ROADMAP risk 9 puts "
      "labels, synthetic data and leave-one-bank-out in P2 (gap pass 1, run 6b) |")
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + DR + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
