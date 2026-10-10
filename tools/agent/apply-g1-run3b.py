#!/usr/bin/env python3
"""Apply gap pass 1, run 3b (targets 11 to 16, plus the unowned API.md rows) after the lead's checks.

Source: tools/agent/gap1-run3b.md (Haiku draft); decisions: tools/agent/gap1-run3b-verdict.md.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'

HDR = ('Gap pass 1 (2026-10-10), run 3b, from tools/agent/gap1-run3b.md; lead-verified '
       '(tools/agent/gap1-run3b-verdict.md).')
CHDR = ('Gap pass 1 (2026-10-10), run 3b: corrections found while checking phases 11 to 16 and the API.md route '
        'owners (tools/agent/gap1-run3b-verdict.md); lead-verified.')

P06 = [
"""### 7.N GET /ref/typologies (DR-381)
This phase owns the route. It joins the `catalogue` router (4 routes) with this row of the section 3.8 route table:

| Method | Path | Scope | Description |
|-|-|-|-|
| GET | /ref/typologies | self | The shared typology set: number, tag and label in Arabic and English (`typology.read`) |

The set is the 21 typologies of research file 27: B2.1 to B2.16 (numbers 1 to 16) and the five cross-cutting tags 17
to 21 named before the B2 table. `tools/build_scn_register.py` writes it as a third file,
`backend/catalogue/typologies.v1.json` (`[{number, tag, label: {ar, en}}]`), reviewed and committed like the two pack
files and signed by the release build. The loader refuses a file whose numbers are not exactly 1 to 21 or whose tags
repeat. Scenarios keep `typologies SMALLINT[]` (the numbers); the DSL `typology` strings are the `tag`s.
Cedar: `typology.read` on System, every active human principal, added to the `catalogue.everyone` policy beside
`cbi_catalogue.read` (`action in [Akr::Action::"cbi_catalogue.read", Akr::Action::"typology.read"]`). API.md 3.6
marks the row "(phase 06)".""",
]

P11 = [
"""### 7.N Tombstone check on every canonical write (phase 10 section 7+)
This phase writes canon only through phase 10 `CanonicalWriter::write`, so the tombstone check that phase 23 adds to
that writer (phase 10 section 7+) covers every row the load loop, catch-up and `ingest_resume` write. No check of its
own is added here.""",

"""### 7.N Quarantine extensions owned by phase 12 (phase 12 section 3.11)
Phase 12 extends this phase's quarantine and owns the extension: the Cedar entity `Quarantine` gains `gate: String`,
derived and not stored (a row held for a transaction-type code that is not mapped: reason `unmapped_code` with field
`txn_type`, DSL.md B.7); `QuarantineRepository` gains `assign` and `accept_conflicts`; `QuarantineFilter` gains
`owner_id`, `older_than_days` and the order `oldest`; the exclusion gains the refusals `gate_not_excludable` and
`card_guard_not_excludable`. The quarantine routes and actions of this phase do not change.""",

"""### 7.N No operator-planned backfill (DR-380)
The backfill rows are removed from API.md 3.5. A gap is covered by the re-read window (DR-37, overlap default 31 days)
and the automatic catch-up of this phase (DR-90); an explicit range beyond the overlap is P2. The sentence of
section 3.5 "the operator-planned backfill of API.md 3.5 (phase 12) is for explicit slices only" and the C-15 line
"the operator-planned backfill of phase 12 is not repeated here" now read: no phase builds an operator-planned backfill
in this release.""",
]

P12 = [
"""### 7.N Reconciliation runs after a restore (phase 30 section 3.9)
Phase 30's `restore verify` runs its steps in order and stops at the first failure: the retention replay (step 2)
completes before step 5 queues `recon_run`. A reconciliation run for a restore gap is therefore valid as it stands. A
business date that ingestion has not reloaded yet shows as a break, which is the intended result: the break is
reported, not waited for. No claim condition or ordering rule is added here.""",

"""### 7.N Backfill routes removed (DR-380)
The Open doubt "Backfill routes" is closed: the rows `POST /sources/{id}/backfills` and `GET /backfills/{id}` are
removed from API.md 3.5. The re-read window (DR-37) and phase 11's automatic catch-up (DR-90) cover gaps; an explicit
range beyond the overlap is P2.""",
]

P13 = [
"""### 7.N Compiler and comparator extensions (phase 16 section 3.2)
Phase 16 extends this phase and owns the extension: `FeatureCompiler` gains `compile_measure` and `compile_filter`,
and `ProfileComparator::plan` and `ProfileComparator::evaluate` become public. The feature definitions, their storage
and this phase's callers do not change.""",

"""### 7.N Re-measure at decision (DR-379)
`check_before_decision` (section 3.7) calls `ReaderCatalogue::measure` again on the proposed change. If the measured
effect differs from the one stored on the change request, the decision is refused with `volume_effect_changed`
(DR-360). There is no recompute entry point: the maker resubmits once the reader replay completes. This closes phase
16 Open doubts 12.""",

"""### 7.N Tier of profile reads
This phase reads no tier: a profile read is a restricted-tier read, and its access event keeps tier `restricted`. From
phase 15 on the party is loaded through the masked `customer` adapter (the tier from phase 15, and
`casework.effective_tier` from phase 17); the access event does not change.""",
]

P14 = [
"""### 7.N Link to a case at decision (phase 17)
From phase 17 on, decision step 8 (section 3.6) calls `ScreeningCaseLinker::link_screening(tx, screening_alert_id,
actor)` in the decision transaction when `subject_kind = 'party'` and the decision is `true` or `potential`.
Counterparty and payment-party subjects are not linked (phase 18, DR-147). Phase 17 defines the port; before phase 17
no call is made.""",

"""### 7.N The screening.hit topic and the suppression sample (DR-375)
From phase 15 on, the decision transaction of section 3.6 writes the outbox topic `screening.hit` for a party subject
decided `potential` or `true`, payload `{alert_id, party_id, list_kind, outcome}` (ids and codes only, no name or list
text). Phase 15 adds the write together with the first handler of the topic (its re-score trigger on `screening.hit`,
DSL.md C.5, stored with the neutral `trigger_code` `TRG_REVIEW_REQUESTED`, phase 15 section 3.6); phase 19 appends its handler in the ordered handler list of phase 00 section 4.5 item 10.
Before phase 15 no row is written.
The suppression records that phase 19 samples are the `alert.pair_memory` rows: `qa_suppression_sample` takes the rows
whose `last_suppressed_at` falls in the ISO week (5 per cent, plus every row on a freeze-list entry). This phase's sweep
keeps expiry and the counters and draws no sample.""",

"""### 7.N Hold alerts from payment holds (phase 29)
Phase 29 extends this phase and owns the extension (phase 29 section 4.5 item 4): `ScreeningService::open_hold_alert(
run, candidate)`, with one caller, `HoldSweepService`. It opens a system `payment` run with `requested_by` NULL and
`trigger_ref` = the hold id, and writes the candidate through `AlertRepository::insert_candidates`. The comments on
`alert.screening_runs` read "adhoc runs and hold payment runs" (`subject_id`) and "payment runs: the hold id (phase
29)" (`trigger_ref`). The subject view of a hold alert reads the names from `assess.holds.party_names_enc` under this
phase's masking. `screening_payment` skips an entry already alerted on a hold whose decision has the same match key.""",

"""### 7.N Risk modifiers (phase 15)
Phase 15 defines `RiskModifierSource::modifier(party)` (its section 2), read from the json setting
`screening.rating_modifiers` (a setting, not a table). From phase 15 on, the threshold step (section 3.4 step 8) adds
the modifier for a party subject within the existing bounds. It is 0 before phase 15 and for the other subject
kinds.""",

"""### 7.N Delisting by absence (DR-376)
An entry that is missing from a new list version is delisted by its absence: phase 20 section 3.5 already reads
absence. Rows are not copied into the new version (a version is immutable after `loading`). `delisted_on` is set only
when the publisher's file carries a delisting date. A silent drop is guarded by `screening.max_count_drop_pct` with
`acknowledged_count_drop`, and `diff_summary.removed` counts the drops. This closes phase 20 Open doubts 22.""",

"""### 7.N Feed check time on ref.lists (DR-378)
`ref.lists` gains `feed_checked_at TIMESTAMPTZ` (NULL until the first check) in migration `0045`; the column goes on
`ref.lists`, not on `ref.list_versions`, which is immutable after loading. Migration `0048` adds `feed_checked_at` to
the column list of the `GRANT UPDATE ... ON ref.lists TO akr_app` line, and `0049` adds it to the `internal` row of
`ref.lists` in `admin.column_classification`. `list_fetch` step 2 sets it on `NotModified` (with the audit kind
`list.feed_unchanged`) and on every successful fetch. `ListRecord` gains the field, and its staleness is `age_hours =
now - max(active.activated_at, feed_checked_at)` (or `loaded_at` for `cbs_snapshot`). The 12-hour cap does not change.
This closes the last sentence of phase 29 Open doubts 29.""",

"""### 7.N Retention of screening alerts (DR-377)
Screening alerts, freeze-list alerts included, stay R7. Where OFAC applies, the bank sets `retention.period_years.R7`
(phase 23; default 7, minimum 7) to 10. No new retention class is added. This closes the Open doubt "Retention".""",
]

P15 = [
"""### 7.N Tier of the customer kind (phase 17)
From phase 17 on, the `tier` of the masked `customer` kind is `casework.effective_tier(party_id)`; until then it is
`canon.party.sensitivity_tier`.""",

"""### 7.N exit_decision and create_ops_task (phase 20, DSL E.4)
This phase ships the workflow `exit_decision` ending at `record_decision`; an effect that is not registered fails
publish, so it does not name `create_ops_task`. Phase 20 registers the effect `create_ops_task` and ships version 2 of
`exit_decision`, which adds it after `record_decision`.""",
]

P15C = [
"""### 7.N The screening.hit handler is registered here (DR-375)
The last row of the trigger table (section 3.3), which lists `screening.hit` with the later topics as "not wired
here", is corrected for `screening.hit`: phase 14 runs before this phase, so this phase registers the topic's first
handler (its trigger on `screening.hit`, DSL.md C.5, stored with the neutral `trigger_code` `TRG_REVIEW_REQUESTED`,
section 3.6) and adds the outbox write to phase 14's decision transaction (phase 14 section 7+): payload `{alert_id,
party_id, list_kind, outcome}` for a party subject decided `potential` or `true`. The handler follows the rules under
the table (it enqueues, never re-scores in the consumer's transaction). The other topics of that row are registered by
the phases that produce them (17 to 19, 26).""",
]

P16 = [
"""### 7.N Tier of request-time reads (phase 17)
From phase 17 on, the request-time tier reads of sections 3.9 and 3.10 and of the decisions use
`casework.effective_tier(party_id)`.""",

"""### 7.N G9 and the Islamic pack (phase 26)
Phase 26 extends this phase and owns the extension: G9 reads `pack: akr` or `pack: islamic`, with the refusal
`pack_not_enabled` while `pack.islamic` is off; the seed skips a document whose `requires_flag` is off; `packs
verify-data` refuses an ISL document without a further condition.""",

"""### 7.N Protected-attribute check on a scenario change (phase 24)
From phase 24 on, the `scenario` change target's `check_before_decision` runs `ProtectedAttributeCheck` (phase 24
section 3.5) on the proposed version; a finding refuses the decision. The check is not part of `GoLiveGate`.""",

"""### 7.N DetectionPayload source kind (DR-382)
From phase 22 on, `DetectionPayload` gains `source_kind` (`scenario` or `observation`); the runner of this phase writes
`scenario`. The scenario named by phase 22's `branch.hybrid_scenario_code` is a register scenario with
`observational = true` (phase 06), not a scenario of a family `observation`: a family code matches `^[A-Z]{2,5}$` and is
one of the 18.""",

"""### 7.N The alert sink (phase 17)
Phase 17 replaces this phase's alert sink with `AlertIntakeService`, which writes the intake audit kinds. The
detection payload and the runner do not change.""",

"""### 7.N Activation instant
A scenario version's activation instant is `scn.scenario_versions.effective_from`; a reader that needs the moment a
version went live reads that column. This closes phase 24 Open doubts 30.""",

"""### 7.N Reader keys of the replay (phase 30, DR-327)
Phase 30 adds the keys `read_source` and `replay_lag_s` to `scn.scenario_runs.cost` (DR-327, standby reads for
scenario runs); the existing keys do not change.""",

"""### 7.N harness-diff removed (DR-380)
`GET /scenarios/{id}/harness-diff` is removed from API.md 3.11: S-48 (an independent re-implementation by another
author) is P2, and phase 24 does not carry it (its Open doubts 11 lists S-48 as not carried). This closes Open
doubts 22.""",
]

TARGETS = [('06', CHDR, P06), ('11', HDR, P11), ('12', HDR, P12), ('13', HDR, P13), ('14', HDR, P14),
           ('15', HDR, P15), ('15', CHDR, P15C), ('16', HDR, P16)]

out = {}
for pp, hdr, blocks in TARGETS:
    p = PLAN / f'phase-{pp}.md'
    s = out.get(p, p.read_text())
    s = add_section7(s, hdr, blocks)
    i = s.index('## 7+. Gap Additions')
    j = s.find('\n## ', i + 5)
    tail = s[i:j if j != -1 else len(s)]
    for w in ('UNCLEAR', 'the lead decides', 'lead confirms'):
        assert w not in tail, (p.name, w)
    out[p] = s

# API.md: remove the unowned rows (DR-380), mark the typology row (DR-381), fix two sentences.
a = ROOT / 'docs' / 'API.md'
s = a.read_text()
for pref in ('| GET | /cases/{id}/acl |', '| POST | /cases/{id}/acl |', '| DELETE | /cases/{id}/acl/{principal_id} |',
             '| GET | /evidence/{id} |', '| POST | /sources/{id}/backfills |', '| GET | /backfills/{id} |',
             '| GET | /ref/registers/{kind} |', '| POST | /ref/registers/{kind}/entries |',
             '| GET | /scenarios/{id}/harness-diff |', '| POST | /exits |'):
    lines = [l for l in s.split('\n') if l.startswith(pref)]
    assert len(lines) == 1, (pref, len(lines))
    s = rep(s, lines[0] + '\n', '')
s = rep(s, '| GET | /ref/typologies | self | Shared typology tag set |',
        '| GET | /ref/typologies | self | Shared typology set: number, tag and label (phase 06, DR-381) |')
s = rep(s, "phase 24's and `GET /evidence/{id}` is phase 18's.",
        "phase 24's. Evidence content is served by its owner (for example\n"
        "`GET /cases/{id}/evidence/{evidence_id}/content`, phase 18); there is no generic evidence route (DR-380).")
s = rep(s, 'backfill routes are phase 12. **CR**',
        're-read window (DR-37) and the automatic catch-up (DR-90) cover gaps, and no phase builds an operator-planned\n'
        'backfill (DR-380). **CR**')
out[a] = s

r = PLAN / 'research.md'
rs = r.read_text()
anchor = [l for l in rs.split('\n') if l.startswith('| DR-374 |')]
assert len(anchor) == 1
DRS = [
    "| DR-375 | 2026-10-10 | `screening.hit` (payload `{alert_id, party_id, list_kind, outcome}`) is written by phase 14's decision transaction for a party decided `potential` or `true`, from phase 15 on; phase 15 registers the first handler, phase 19 appends; phase 19's suppression sample reads `alert.pair_memory` by `last_suppressed_at` | Phases 15 and 19 consumed a topic no phase wrote; the outbox rule writes a topic only once a handler exists (gap pass 1, phase 14 section 7+) |",
    "| DR-376 | 2026-10-10 | A list entry is delisted by its absence from the new version; rows are not copied; `delisted_on` only from a publisher date; drops guarded by `screening.max_count_drop_pct` | Versions are immutable after loading; phase 20 already reads absence (gap pass 1, phase 14 section 7+) |",
    "| DR-377 | 2026-10-10 | Screening alerts, freeze lists included, stay R7; the bank sets `retention.period_years.R7` to 10 where OFAC applies | One class with a bank setting covers the ten-year case without a new class (gap pass 1, phase 14 section 7+) |",
    "| DR-378 | 2026-10-10 | `ref.lists.feed_checked_at` (0045, grant 0048, classification 0049) is set on every fetch, `NotModified` included; staleness reads `max(activated_at, feed_checked_at)` | An unchanged feed made a list stale; `ref.list_versions` is immutable after loading (gap pass 1, phase 14 section 7+) |",
    "| DR-379 | 2026-10-10 | Phase 13's `check_before_decision` calls `ReaderCatalogue::measure` again and refuses `volume_effect_changed`; no recompute entry point, the maker resubmits | The stored effect can be stale by decision time (gap pass 1, phase 13 section 7+; closes phase 16 Open doubts 12) |",
    "| DR-380 | 2026-10-10 | API.md rows removed: `/cases/{id}/acl` (3, duplicates of `/cases/{id}/access`), `GET /evidence/{id}`, the two backfill routes (P2; DR-37 and DR-90 cover gaps), `/ref/registers` (2, P2), `harness-diff` (S-48 P2), `POST /exits` (superseded) | No phase owned them; each is a duplicate, superseded or later-release work (gap pass 1, API route owners) |",
    "| DR-381 | 2026-10-10 | Phase 06 owns `GET /ref/typologies` (`catalogue` router, `typology.read`, every active human), served from `backend/catalogue/typologies.v1.json` (21 typologies of research file 27) | Scenarios and the DSL used typology numbers and tags with no served set (gap pass 1, API route owners) |",
    "| DR-382 | 2026-10-10 | `DetectionPayload` gains `source_kind` (`scenario`, `observation`) from phase 22; the hybrid scenario is a register scenario with `observational = true`, not a family `observation` | Families are the 18 codes of `^[A-Z]{2,5}$` (gap pass 1, phase 16 section 7+) |",
]
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + '\n'.join(DRS) + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
