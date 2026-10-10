#!/usr/bin/env python3
"""Apply gap pass 1, run 7a (coverage: UX and W, plus gap item (ab) X2) after the lead's checks.

Source: tools/agent/gap1-run7a.md (Haiku draft); decisions: tools/agent/gap1-run7a-verdict.md.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'
DOCS = ROOT / 'docs'

IMPL = {
    '08': 'UX-03 (usability round 1 with Iraqi staff settles the range order, section 7+), UX-39 (the per-release '
          'accessibility gate: keyboard walkthroughs, 400 percent zoom in Arabic, screen-reader passes, section 7+)',
    '14': 'UX-58 (the screening hit screen of mockup 03), W-09 (weak-alias classes, patronymic alignment and the '
          'side-by-side hit screen)',
    '15': 'UX-59 (the risk profile of mockup 04; the open-case strip from phase 18 on)',
    '16': 'W-04 (below-the-line testing, champion and challenger, the governed scenario lifecycle)',
    '17': 'UX-05 (merged alerts as one case row with the merged count), UX-12 (the shortcut layer, section 7+), UX-16 '
          '(response budgets tested at 50 concurrent sessions, section 7+)',
    '18': 'UX-07 (the entity timeline of the case subject, section 7+), UX-29 (global search with kind detection and '
          'suspicion rows absent for non-members, section 7+)',
    '19': 'UX-10 and UX-61 (the network view with grouping above about 15 accounts, the node and link table, masked '
          'numbers and keyboard traversal)',
    '20': 'UX-69 (the operations-task inbox with the neutral instruction only)',
    '21': 'UX-62 (the report builder of mockup 07 with the filed-and-acknowledged step)',
    '24': 'UX-47 (dashboards in attention, period, detail order with definitions and data-as-of), UX-56 (the command '
          'centre of mockup 01)',
    '25': 'UX-40 (contextual help, the bilingual glossary and what-changed notices in one help centre)',
}
PART = {
    '08': 'UX-71 (the role-based rail and plain wording)',
    '10': 'W-14 (the canonical trade-instrument table)',
    '14': 'UX-43 (verdict sentence, identifiers, per-part table, frequency sentence, pair memory), W-07 (versioned lists '
          'and good-guy rules under change request), W-08 (customers, counterparties and payment parties screened), '
          'W-10 (the dated known-answer harness, look-backs and list diffs), W-18 (terrorist-list matches ranked first)',
    '15': 'W-15 (source-of-funds and source-of-wealth evidence; declared against actual figures), W-21 (the PEP register '
          'by position, confirmed relatives, the former state, pair memory), W-24 (the trigger library and the refresh '
          'clock)',
    '17': 'UX-11 (Next on one key; drafts kept in the tab), UX-37 (the pause control for live updates), UX-42 (three '
          'headline reasons beside their facts with mitigating factors)',
    '18': 'UX-09 (the evidence-opened gate before a decision), UX-17 (drafts kept in the tab across a time-out), UX-57 '
          '(the case workspace of mockup 02)',
    '19': 'UX-70 (the QA workspace), W-03 (network edges for shared contacts, ownership and kinship), W-23 (the RFI desk '
          'with deadline, KPIs, templates and four-eyes release)',
    '21': 'UX-51 (scheduled regulatory reports in both languages)',
    '24': 'UX-31 (CSV and XLSX register exports with injection guards, section 7+), UX-48 (the command-centre tiles), '
          'W-01 (productive alerts and FIU feedback), W-20 (the audit viewer and control report)',
}
NOTC = {
    '08': 'UX-52 (vendor usability benchmark, research only, DR-400)',
    '10': 'W-12 (payment-quality monitor, DR-400), W-13 (bundled and on-behalf-of payments, P2, DR-394)',
    '15': 'W-16 (adverse-media module; no media source contracted), W-17 (enterprise-wide risk assessment, DR-400)',
    '16': 'W-25 (PSP and MSB segment, DR-400)',
    '17': 'UX-04 (workspace telemetry for the pilot, DR-400)',
    '27': 'W-02 (supervised and unsupervised scoring), W-11 (model governance pack); P2, DR-400',
    '28': 'UX-46 (explanation usability protocol), W-05 (LLM assistants); P2, DR-400',
    '31': 'W-22 (correspondent and nested-relationship module, P2 correspondent pack)',
}

HDR = ('Gap pass 1 (2026-10-10), run 7a (coverage of UX and W IDs, gap item (ab)), from tools/agent/gap1-run7a.md; '
       'lead-verified (tools/agent/gap1-run7a-verdict.md).')

P18 = [
"""### 7.N Global search (X2; UX-29; SP-27, RP-61; DR-400)
One route serves the header search and the command palette (DESIGN-SYSTEM X2, MODULES M12 P1 screens). It searches
parties and cases only. It never searches reports (RP-61), screening alerts or pair memory (phase 14 owns the name
search, `screening.work`).

| Method | Path | Scope | Description |
|-|-|-|-|
| GET | `/api/v1/search` | `queue.work` | Global search: `q` 2 to 80 characters, `limit` at most 10 per group; groups `party` and `case` with `has_more`, no totals; purpose required (New; `party.read`, `case.read`) |

`SearchService::query(ctx, q, purpose, limit)`:
1. `X-Access-Purpose` holds a purpose code (else 422 `E_PURPOSE_REQUIRED`, phase 02).
2. `SearchKind::detect(q)` (pure): `case_ref` when `q` matches `^CS-[0-9]{4}-[0-9]{6}$`; `iban` when it has 23
   characters starting `IQ` and passes the mod-97 check; `card` when it has 13 to 19 digits and passes Luhn; `phone`
   when it normalises as phase 10 normalises contact values; otherwise `name`. A kind whose check fails is searched
   as `name`.
3. Parties: `iban` against `iban_hk` and `number_hk` of current `canon.account` rows (owner parties); `card` against
   `canon.card.pan_hk`; `phone` against `canon.party_contact.value_hk`; `name` through the `crates/names` key against
   current `canon.party_name.key`. Only keyed hashes and keys are compared. Each party passes Cedar `party.read` and
   the section 3.10 tier rule; a party the caller may not see is absent, so no count reveals it.
4. Cases: `case_ref` exact, or the cases of the matched parties, filtered by `services::visibility::build` (phase 17
   section 3.10), the predicate of the queues; Cedar `case.read`.
5. One access event `search_hit` with resource type `global_search`, `resource_id` = the id of the audit event
   `search.performed {kind, groups}`, the purpose code and no `subject_party` (the pattern of phase 14 section 7+), then
   `scope.record_hits(rows)` (phase 02) for the restricted and suspicion-tier rows shown. The query text is never
   logged.
6. Frontend: the X2 overlay opens from the header and on `/` (phase 17 section 7+ shortcut layer); results grouped by
   kind, Arabic name first.
7. Tests: each kind for each role of the visibility matrix; a suspicion party outside the access list is absent with the
   same body as no match; a card-like query that fails Luhn is searched as a name.""",
"""### 7.N Entity timeline of the case subject (UX-07; DESIGN-SYSTEM 7.14 and W3; DR-400)

| Method | Path | Scope | Description |
|-|-|-|-|
| GET | `/api/v1/parties/{id}/timeline` | `queue.work` | Events of the party, newest first; `kinds` filter; keyset cursor on (occurred_at, id); the 404 of section 3.10 for a masked party (New; `party.read`) |

`PartyTimelineService::list(ctx, party_id, kinds, cursor)`:
1. The tier rule of section 3.14 steps 1 and 2 (404, step-up, purpose).
2. Sources, each through its own visibility: alerts of the party (phase 17 predicate), the party's cases and their
   decisions (section 3.10), rating changes (`kyc.ratings`, the level only, when the caller holds `rating.read`). From
   phase 19 on: RFIs, authority requests and QA events; from phase 20 on: operations tasks; from phase 22 on:
   observations. A row whose source the caller may not see is absent.
3. Each row: `kind`, `occurred_at`, a role word for the actor, one neutral summary sentence, a link to its screen, and
   the seasonal band of its date (`ref.calendar_days` kinds `ramadan_hours`, `eid`, `arbaeen`, with `hijri_label`).
4. Access event `view` (resource `party`) per page.
5. Frontend: the W3 evidence column shows the subject's timeline with the kind filter and the table alternative of
   DESIGN-SYSTEM 7.14. The W4 tab list is unchanged.""",
"""### 7.N The open-case strip on K1 (UX-59; closes the phase 15 section 5 deferral)
K1 (`/parties/:id/risk`) shows one strip above the drivers: the party's open cases that the caller may see, from
`GET /parties/{id}` step 5 (reference, priority class, status), and from phase 19 on the party's UTR watches from
`GET /watches` with the `party_id` filter (phase 19 section 7+). A case the caller may not see is absent, with no
count. Empty state: "No open cases or watches for this customer".""",
]

P19 = [
"""### 7.N Party filter on the watches list (phase 18 section 7+, UX-59)
`GET /watches` accepts `party_id` (UUID; optional). The visibility, cursor and Cedar `watch.read` are unchanged.""",
]

P17 = [
"""### 7.N Shortcut layer (UX-12; DESIGN-SYSTEM 7.18)
The shell component of DESIGN-SYSTEM 7.18 (phase 08 left it to P1), first used by W1, W2 and W3:
1. Bindings match `KeyboardEvent.code`, never `key`, and show the Latin keycap, so an Arabic layout gives the same
   keys. The Next binding of section 5 (`KeyN`) is the first.
2. No binding fires while focus is in an input, textarea, select or contenteditable element.
3. `?` opens a cheat sheet of the current screen's bindings.
4. The phase 01 preference `shortcuts { enabled, modifier }` (`PATCH /me/preferences`) turns the layer off or requires
   Ctrl or Alt; the switch sits in X4. No route and no audit event are added.
5. Tests: a key in a text field does nothing; with `enabled: false` no binding fires; with `modifier: Ctrl` a bare key
   does nothing.""",
"""### 7.N Response budgets (UX-16; ROADMAP P1 quality gate)
A load run in the release pipeline (not on every push) with 50 concurrent sessions over a synthetic fixture (SP-06;
at least 10,000 alerts across the queues): first content of `GET /queues/{id}/items` and `GET /cases/{id}` within 1 s;
`POST /queues/{id}/next` p95 under 300 ms; from phase 18 on, `GET /search` p95 under 300 ms (API.md budgets). A budget
missed fails the release job, and the figures go to the release notes.""",
]

P08 = [
"""### 7.N Usability round 1 (UX-03; ROADMAP track A; DS-O4)
Usability round 1 runs with three to five Iraqi compliance staff per group (MLRO team, analysts, branch liaison), Arabic
first, on paper and clickable prototypes of the P0 screens. Its dated record in docs/plan/research.md settles DS-O4
(the order of numeric ranges); until then `formatRange()` keeps the left-to-right order. No code, route or setting is
added.""",
"""### 7.N Accessibility release gate (UX-39; ROADMAP P1 accessibility pack; DS-O5)
The axe run of section 6 stays in CI. Each release adds a signed checklist, stored with the conformance statement:
keyboard-only walkthroughs of the six core tasks (open a case, record a disposition, request information, escalate,
sign a report, answer an RFI); 400 percent zoom (320 CSS px reflow) in Arabic on those screens; NVDA and VoiceOver
passes on the same tasks with mixed-script names. A failed item blocks the release.""",
]

P24 = [
"""### 7.N Register exports (UX-31; DESIGN-SYSTEM 8.1 REGISTER pattern; DR-400)
Each REGISTER screen of the catalogue offers one export, built as a job.

| Method | Path | Scope | Description |
|-|-|-|-|
| POST | `/api/v1/exports` | the list route's scope | Start an export `{register, filters, format: csv or xlsx}`; 202 with the export id; `Idempotency-Key` (New; the list route's Cedar action) |
| GET | `/api/v1/exports/{id}` | the list route's scope | Status of the caller's own export (New; requester only) |
| GET | `/api/v1/exports/{id}/file` | the list route's scope | The file; step-up class `export`; access event `export` first (New; requester only) |

1. `register` is a screen ID of the catalogue whose pattern is REGISTER; its filters are those of the screen's list
   route. The route checks that list route's scope and Cedar action for the caller; anything else is 422 at
   `/register`.
2. Job `register_export` (akr-worker; payload `{register, filters, format, requested_by}`): it calls the screen's own
   list service as the requester, so the visibility and masking are the list route's (suspicion rows absent for a
   non-member; the report register is never exported, RP-61). The export id is the job id; no table is added.
3. CSV: UTF-8 with a byte-order mark, ISO 8601 dates, Western digits. XLSX: right-to-left sheet view for Arabic content.
   In both, a text cell starting with `=`, `+`, `-`, `@`, a tab or a carriage return is prefixed with an apostrophe.
4. The file is stored through `EvidenceStore::stage` (phase 02) as kind `render`, owner kind `register_export`, owner id
   = the export id, and is served only once clean.
5. Audit `export.requested {register, format}` and `export.downloaded {register, sha256}`. Per-user watermarking is P2
   (DR-396). PDFs stay on the pack path.
6. Tests: a fixture name starting with `=` is prefixed in both formats; a suspicion row is absent for a non-member and
   the export row count equals the list's; another user's export id answers 404.""",
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


for pp, blocks in (('18', P18), ('19', P19), ('17', P17), ('08', P08), ('24', P24)):
    p, s = load(pp)
    st = s.index('## 7+. Gap Additions')
    nx = s.find('\n## ', st + 5)
    assert 'run 7a' not in s[st:nx if nx != -1 else len(s)], pp
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

edit('24', [("Capacity is P2 (phase 28 workbench), so the tile is not in this phase.",
             "Capacity is P2 (phase 31 workbench, G2), so the tile is not in this phase.")])
edit('15', [("is hidden in this phase (phase 18 adds it within the user's access).",
             "is hidden in this phase (phase 18 section 7+ adds it within the user's access).")])

edit(DOCS / 'API.md', [
    ("| GET | /cases | queue.work | Case search (post-filter; no totals) |\n",
     "| GET | /cases | queue.work | Case search (post-filter; no totals) |\n"
     "| GET | /search | queue.work | Global search over parties and cases; kind detection; suspicion rows absent for "
     "non-members; never reports or screening (phase 18 7+) |\n"),
    ("| GET | /parties/{id}/graph | queue.work | Network around the party (kinship hidden by default; showing it is logged) (phase 19) |\n",
     "| GET | /parties/{id}/graph | queue.work | Network around the party (kinship hidden by default; showing it is logged) (phase 19) |\n"
     "| GET | /parties/{id}/timeline | queue.work | Entity timeline of the party, each source through its own visibility "
     "(phase 18 7+) |\n"),
    ("| POST | /packs/verify | self |",
     "| POST | /exports | list route's scope | Start a CSV or XLSX export of one register screen (phase 24 7+) |\n"
     "| GET | /exports/{id} | list route's scope | Status of the caller's own export (phase 24 7+) |\n"
     "| GET | /exports/{id}/file | list route's scope | The export file; **SU** class `export` (phase 24 7+) |\n"
     "| POST | /packs/verify | self |"),
])

r = PLAN / 'research.md'
p, rs = load(r)
anchor = [l for l in rs.split('\n') if l.startswith('| DR-399 |')]
assert len(anchor) == 1
DR = ("| DR-400 | 2026-10-10 | Run 7a additions: global search `GET /search` in phase 18 (parties and cases; never reports "
      "or screening; suspicion rows absent), the party timeline `GET /parties/{id}/timeline` shown in W3, the K1 "
      "open-case strip, the shortcut layer and response budgets in phase 17, usability round 1 and the accessibility "
      "release gate in phase 08, register exports in phase 24 (export id = job id, file = evidence object of kind "
      "`render`, no table). Outside this release: workspace telemetry (UX-04), the vendor benchmark (UX-52), model and "
      "assistant items (UX-46, W-02, W-05, W-11), the payment-quality monitor (W-12), trade-finance rules, profile, "
      "outliers and vessel screening (W-14), the enterprise-wide assessment (W-17), the PSP segment (W-25) | X2 is a "
      "MODULES M12 P1 screen; DESIGN-SYSTEM 7.14, 7.18, 8.1 and the ROADMAP P1 quality gates name the others; the "
      "excluded items are on no P0/P1 line (gap pass 1, run 7a) |")
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + DR + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
