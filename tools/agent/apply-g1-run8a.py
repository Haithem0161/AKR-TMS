#!/usr/bin/env python3
"""Apply gap pass 1, run 8a (coverage: VA and VO market IDs) after the lead's checks.

Source: tools/agent/gap1-run8a.md (Haiku draft); decisions: tools/agent/gap1-run8a-verdict.md.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'
DOCS = ROOT / 'docs'

IMPL = {
    '06': 'VA-19 (the CBI catalogue as a versioned register with its coverage matrix on day one)',
    '08': 'VA-18 (Arabic and right-to-left first on every screen, English second)',
    '11': 'VA-03 (read-only extracts checked against control totals, with automatic catch-up), VA-04 (one runtime with '
          'jobs, secrets and alarms on late, partial or failed loads)',
    '12': 'VA-16 (the coverage ledger by product, channel and transaction type; exclusions with owner, reason and expiry)',
    '14': 'VA-11 (Arabic name matching with a score and coded reasons)',
    '17': 'VA-39 (one open case per party and family with its alerts as evidence)',
    '20': 'VA-05 (freeze and other measures as operations tasks with clock, owner and verification against the CBS copy)',
    '21': 'VA-14 (branch report, acknowledgement, MLRO decision, goAML report and the quality check), VA-30 (goAML from '
          'the profile registry with versions, field-linked errors, package, status and rejections), VO-11 (the goAML '
          'builder validated against the Office schema with the rejection path)',
    '24': 'VA-54 and VO-03 (the parallel run beside the bank\'s current system with the disagreement log, section 7+)',
}
PART = {
    '00': 'VA-21 (profile S: one server plus a backup, one runtime and one scheduler)',
    '01': 'VA-46 (personal access tokens with the user\'s roles and read scopes, audited)',
    '10': 'VA-31 (the product-agnostic canonical model with the product table)',
    '11': 'VA-17 (ICS BANKS on Oracle as the first adapter pack), VA-24 (the opt-in change-capture path beside the daily '
          'batch)',
    '12': 'VA-15 (the coverage ledger with dark units and the monitored share)',
    '14': 'VA-22 (versioned lists kept with their history, pair-keyed good-guy rules, proof of list age), VA-32 (Arabic '
          'keys on every name path with the known-answer harness), VO-10 (the dated known-answer harness)',
    '15': 'VA-36 (open, versioned rating models with factors, weights, bands and overrides)',
    '16': 'VA-06 (rule scenarios with the one-hop network), VA-07 (replay on own history with the go-live gate), VA-27 '
          '(the bilingual scenario card with its change history), VA-28 (replay rows per entity, with near misses), '
          'VA-29 (the seeded known-answer suite), VA-35 (versioned scenario documents with the change-request diff), '
          'VA-37 (draft, replay, approve and activate in the product), VA-43 and VO-09 (the shipped image with bank '
          'parameter sets and bank scenarios), VA-45 (family-number links and the mother\'s name), VA-55 and VO-02 '
          '(replay on as-of history with reach and overlap)',
    '18': 'VA-13 (case intake from every alert source, shipped workflow definitions, decision history), VA-48 (the '
          'shelved-decision register)',
    '19': 'VA-08 (hibernation only inside a case by a person, with trigger keys and expiry)',
    '21': 'VA-09 (per-scenario contribution figures), VO-01 (aggregate analytics with suppression)',
    '22': 'VA-42 (the branch window with internal reports, status and the monthly report)',
    '24': 'VA-12 (the inventory record of rules with owner and validation), VA-23 (the assessor pack from system facts)',
    '25': 'VA-25 (signed release bundles with a recorded rollback plan), VA-38 (the support session opened by the bank), '
          'VA-47 (in-product help and the bilingual glossary), VA-56 and VO-08 (offline signed bundles for the platform)',
    '26': 'VA-26 (Islamic contract types in the product list as scenario inputs)',
}
NOTC = {
    '05': 'VA-33 (vendor implementation timeline, DR-402)',
    '06': 'VA-51 (small-bank commercial package, DR-402)',
    '10': 'VA-41 (model-proposed mappings; P2, DR-402)',
    '11': 'VA-53 (migration kit from core-bundled AML modules, DR-402)',
    '21': 'VA-49 (published claims ledger, DR-402)',
    '23': 'VA-02 (vendor exit export, DR-396)',
    '25': 'VA-01, VA-50 and VO-05 (escrow and continuity terms, DR-396); VA-20 and VO-06 (pricing); VA-34 (support '
          'policy); VO-14 (sanctions clause in the contract); VO-13 (release-gate note; the ROADMAP P1 exit criteria '
          'are the gate); DR-402',
    '27': 'VA-52 (cross-bank signal sharing; P2, DR-402)',
    '28': 'VA-10 (generative assistant features), VO-12 (agent register); P2, DR-402',
    '29': 'VO-04 (import of payment-screening engine alerts; P2, DR-402)',
    '30': 'VA-44 (hosted shared service; P2, DR-402)',
    '31': 'VA-40 (synthetic data generator; P2, DR-402)',
}

HDR = ('Gap pass 1 (2026-10-10), run 8a (coverage of VA and VO market IDs), from tools/agent/gap1-run8a.md; '
       'lead-verified (tools/agent/gap1-run8a-verdict.md).')

P24 = [
"""### 7.N Parallel run beside the bank's current system (VA-54, VO-03; ROADMAP P1 "Parallel run"; DR-402)
ROADMAP P1 runs AKR-TMS beside the bank's current system for two to three months: every alert compared both ways, every
disagreement explained in a log the MLRO reviews weekly. This section builds the import, the comparison and the log.
The comparison is by party and date, not by scenario, because the two systems' rule lists differ.

**Migration.** `0077_admin_fairness_training_assessor` also creates the table below; `0079_admin_guards_grants` adds
`GRANT SELECT, INSERT, UPDATE ON alert.parallel_run_rows TO akr_app`, the classification (`internal` for ids, codes,
dates and outcome; `personal` for `incumbent_ref`, `incumbent_rule` and `note`), the retention row `('alert',
'parallel_run_rows', 'R5', 'age', 'created_at', 'Parallel-run comparison; kept with the alert class')` and the setting
`parallel_run.match_days` (integer, 3, bounds 0 to 14).

```sql
CREATE TABLE IF NOT EXISTS alert.parallel_run_rows (
    id                    UUID PRIMARY KEY,
    import_id             UUID NOT NULL,                  -- the job id of the import (no import table)
    party_id              UUID,                           -- no foreign key (canonical party, DR-87); NULL when unresolved
    outcome               TEXT NOT NULL CHECK (outcome IN ('both', 'akr_only', 'incumbent_only', 'unresolved')),
    incumbent_ref         TEXT CHECK (incumbent_ref IS NULL OR length(incumbent_ref) BETWEEN 1 AND 100),
    incumbent_rule        TEXT CHECK (incumbent_rule IS NULL OR length(incumbent_rule) BETWEEN 1 AND 200),
    incumbent_on          DATE,
    akr_alert_id          UUID,
    akr_alert_created_at  TIMESTAMPTZ,
    explanation_code      TEXT CHECK (explanation_code IN ('PRL_SCENARIO_GAP', 'PRL_PARAMETER', 'PRL_DATA',
                              'PRL_TIMING', 'PRL_INCUMBENT_NOISE', 'PRL_AKR_NOISE', 'PRL_OTHER')),
    note                  TEXT CHECK (note IS NULL OR length(note) BETWEEN 1 AND 2000),   -- passes PolicyPackService::scan
    explained_by          UUID REFERENCES admin.users(id) ON DELETE RESTRICT,
    explained_at          TIMESTAMPTZ,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT fk_parallel_akr_alert FOREIGN KEY (akr_alert_id, akr_alert_created_at)
        REFERENCES alert.alerts(id, created_at) ON DELETE RESTRICT,
    CHECK ((outcome = 'unresolved') = (party_id IS NULL)),
    CHECK ((outcome = 'akr_only') = (incumbent_ref IS NULL)),
    CHECK ((outcome IN ('both', 'akr_only')) = (akr_alert_id IS NOT NULL)),
    CHECK ((explanation_code IS NULL) = (explained_by IS NULL) AND (explained_by IS NULL) = (explained_at IS NULL)),
    CHECK (explanation_code IS DISTINCT FROM 'PRL_OTHER' OR note IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_parallel_rows_import ON alert.parallel_run_rows (import_id, outcome);
CREATE INDEX IF NOT EXISTS idx_parallel_rows_open ON alert.parallel_run_rows (created_at) WHERE explanation_code IS NULL
    AND outcome IN ('akr_only', 'incumbent_only');
```

| Method | Path | Scope | Description |
|-|-|-|-|
| POST | `/api/v1/parallel-run/imports` | `case.decide` | Upload the current system's alert extract (CSV); `Idempotency-Key`; 202 with `Location` (New; `parallel_run.write`) |
| GET | `/api/v1/parallel-run/imports/{id}` | `dashboard.read` | Import status with row, unresolved and outcome counts (New; `parallel_run.read`) |
| GET | `/api/v1/parallel-run/rows` | `dashboard.read` | Rows by `import_id`, `outcome` and `explained`; keyset cursor; no totals (New; `parallel_run.read`) |
| POST | `/api/v1/parallel-run/rows/{id}/explanation` | `case.decide` | Record the explanation code and note of one disagreement (New; `parallel_run.write`) |
| GET | `/api/v1/parallel-run/summary` | `dashboard.read` | Counts by ISO week and outcome, with the unexplained count (New; `parallel_run.read`) |

**Cedar** (`C0_governance.cedar`, one role per policy): `parallel_run.read` for M and CH; `parallel_run.write` for M
only. No existing action has exactly these roles. IA reads the outcome through the audit events.

`ParallelRunService`:
1. Import: the CSV has the columns `incumbent_ref`, `cif`, `alerted_on`, `rule` and `outcome`, UTF-8, at most
   `screening.feed_max_bytes`. The file goes through `EvidenceStore::stage` (phase 02; owner kind `parallel_import`,
   kind `upload`, retention R5), and job `parallel_import {evidence_id}` is enqueued. Its job id is the import id.
2. Job `parallel_import`: wait for the AV scan as `list_parse` does (phase 14 section 3.2). Resolve each `cif` to the
   party of the current `canon.customer` row; no match gives an `unresolved` row. Then compare in one transaction, over
   the window from the earliest to the latest `alerted_on`, widened by `parallel_run.match_days`:
   - `both`: an incumbent row whose party has an `alert.alerts` row created within `parallel_run.match_days` of
     `alerted_on`;
   - `incumbent_only`: an incumbent row without one;
   - `akr_only`: an `alert.alerts` row in the window (source kind `scenario`) that no incumbent row of its party
     matched.
   Audit `parallel_run.imported {import_id, rows, unresolved, both, akr_only, incumbent_only}`, counts only.
3. Explanation: the row must be `akr_only` or `incumbent_only` and not yet explained (else 409
   `E_NOT_ALLOWED_IN_STATE`); `PRL_OTHER` needs a note (422 `E_VALIDATION`). Set the code, note, `explained_by` and
   `explained_at`. Audit `parallel_run.explained {row_id, code}`.
4. Lists and counts filter in SQL by the phase 18 section 3.10 party visibility (`services::visibility::build`), so a
   suspicion-tier party outside the reader's access is absent and the counts come from the filtered query.
5. The summary is the weekly review: the MLRO reads it, with its unexplained count, each week, and before the go-live
   decision.

**Screen.** G4 gains the tab "Parallel run" for M and CH: the summary, the rows of an import, and the explanation
form (codes in the Office's words).

**Tipping-off review.** The rows and the summary are read only by M and CH, through the party visibility. Nothing is
written to `admin.registers`, the command centre, a notification or the case. The import file is an evidence object
of the same class.

**Tests.** A fixture extract with one shared, one incumbent-only and one unknown `cif` gives one `both`, one
`incumbent_only`, one `unresolved` and the `akr_only` rows of the window. An explained row refuses a second
explanation. A CH session cannot explain. A suspicion-tier party outside the reader's access is absent from the rows
and the counts.""",
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


for pp, blocks in (('24', P24),):
    p, s = load(pp)
    st = s.index('## 7+. Gap Additions')
    nx = s.find('\n## ', st + 5)
    assert 'run 8a' not in s[st:nx if nx != -1 else len(s)], pp
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

edit('04', [
    ("designer canvas (D3) and the inventory screens wait for phases 26 and 27; until then ADM authors definitions as JSON",
     "designer canvas is on no P0/P1 line (DR-402) and the inventory screens wait for phase 27; ADM authors definitions as JSON"),
    ("- No workflow designer canvas (phase 26) and no inventory screens",
     "- No workflow designer canvas (on no P0/P1 line, DR-402) and no inventory screens"),
])

edit(DOCS / 'API.md', [
    ("| POST | /packs/verify | self |",
     "| POST | /parallel-run/imports | case.decide | Upload the current system's alert extract for the parallel run (phase 24 7+) |\n"
     "| GET | /parallel-run/imports/{id} | dashboard.read | Import status and outcome counts (phase 24 7+) |\n"
     "| GET | /parallel-run/rows | dashboard.read | Comparison rows by import, outcome and explained; M and CH; party visibility (phase 24 7+) |\n"
     "| POST | /parallel-run/rows/{id}/explanation | case.decide | Explain one disagreement; M only (phase 24 7+) |\n"
     "| GET | /parallel-run/summary | dashboard.read | Counts by week and outcome with the unexplained count (phase 24 7+) |\n"
     "| POST | /packs/verify | self |"),
])

dm_anchor = [l for l in (DOCS / 'DATA-MODEL.md').read_text().split('\n') if l.startswith('| alert.quarantine |')]
assert len(dm_anchor) == 1
edit(DOCS / 'DATA-MODEL.md', [
    (dm_anchor[0] + '\n', dm_anchor[0] + '\n'
     "| alert.parallel_run_rows | id, import_id (the import job id), party_id null, outcome (both, akr_only, incumbent_only, "
     "unresolved), incumbent_ref, incumbent_rule, incumbent_on, akr_alert_id, akr_alert_created_at (FK alert.alerts), "
     "explanation_code (closed `PRL_` list), note, explained_by, explained_at, created_at; read by M and CH only "
     "(phase 24 section 7+, DR-402) | none | R5 |\n"),
])

edit(PLAN / 'roadmap.md', [
    ("`admin.verify_runs` (`admin.packs` kind `certification`;",
     "`admin.verify_runs`, `alert.parallel_run_rows` (gap pass 1) (`admin.packs` kind `certification`;"),
])

r = PLAN / 'research.md'
p, rs = load(r)
anchor = [l for l in rs.split('\n') if l.startswith('| DR-401 |')]
assert len(anchor) == 1
DR = ("| DR-402 | 2026-10-10 | Run 8a: the parallel run in phase 24 (CSV import of the current system's alerts, staged "
      "as evidence; comparison by party and date within `parallel_run.match_days`; table `alert.parallel_run_rows` with "
      "the explanation on the row; Cedar `parallel_run.read` for M and CH, `parallel_run.write` for M; nothing in "
      "`admin.registers`). The workflow designer canvas is on no P0/P1 line. Outside this release: vendor commercial "
      "and contract terms (VA-01, VA-20, VA-33, VA-34, VA-49, VA-50, VA-51, VO-05, VO-06, VO-14), the legacy migration "
      "kit (VA-53), model, assistant, synthetic-data, cross-bank, hosted and payment-engine items (VA-10, VA-40, VA-41, "
      "VA-44, VA-52, VO-04, VO-12), and the missing parts of the run 8a Partly rows | ROADMAP P1 names the parallel run "
      "with its disagreement log and no phase built it; the rest is on no P0/P1 line (gap pass 1, run 8a) |")
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + DR + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
