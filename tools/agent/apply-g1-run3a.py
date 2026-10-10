#!/usr/bin/env python3
"""Apply gap pass 1, run 3a (targets 05 to 10) after the lead's checks.

Source: tools/agent/gap1-run3a.md (Haiku draft); decisions: tools/agent/gap1-run3a-verdict.md.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'

HDR = ('Gap pass 1 (2026-10-10), run 3a, from tools/agent/gap1-run3a.md; lead-verified '
       '(tools/agent/gap1-run3a-verdict.md).')
CHDR = ('Gap pass 1 (2026-10-10), run 3a: corrections found while checking phases 05 to 10 '
        '(tools/agent/gap1-run3a-verdict.md); lead-verified.')

P05 = [
"""### 7.N discovery_seal_report: payload and the jobs it enqueues (DR-366)
The payload is exactly one of `{run_id}` or `{check_id}`. Section 3.3 step 9 enqueues `{check_id}`: the job renders
the grant check as canonical JSON, signs it with `akr-sign-certificate`, stores it as an evidence object owned by the
source and writes `admin.source_grant_checks.evidence_object_id` (a check with it set is skipped). Section 3.6 enqueues
`{run_id}`: the run report of section 3.6, written to `admin.discovery_runs.report_object_id` (a run with it set is
skipped). The job table of section 4 reads "payload `{run_id}` or `{check_id}`".
For a completed profile run only, the same transaction that writes `report_object_id` also enqueues `coverage_grade`
`{reason: "profile_run"}` (phase 06 section 3.3, coalesce key `coverage_grade`) and `drift_scan {source_id, run_id}`
(phase 10 section 3.5, coalesce key `drift_scan:<source_id>`). A catalogue run and a grant check enqueue neither.""",

"""### 7.N Calendar capture inside discovery_run (phase 09 section 3.3)
The adapter-pack hint schema (section 3.1 `patch`, section 3.6) gains the key `calendar`, which names two approved
objects of the ICS BANKS profile: the holiday table `HOLIDAY` and the currency table `CURRENCY` (the columns phase 09
section 3.3 lists). `discovery_run` gains one step at the end of a profile run: the calendar capture of phase 09
section 3.3. It reads each object whole under the row cap `calendar.capture_max_rows` (reaching the cap is the finding
`calendar_capture_truncated`, and the run is not sealed as complete), proves each statement with `SelectProof` (scope
`Data`), and refuses an object outside the approved list with `E_OBJECT_NOT_APPROVED`. Rows go to
`ref.cbs_calendar_captures` (phase 09 migration; `akr_ingest` holds INSERT) in the run's transaction. No customer,
account or posting table is read, and no new statement form or connection is added. The statements take
`StatementKind::Read` (DR-365).""",

"""### 7.N control_totals in the adapter pack (phase 12 section 4)
The hint schema gains the optional key `control_totals`: the object names, date columns and the branch, currency,
ledger and sign columns of the bank's control totals (data, not code; read by phase 12's control-totals reader). An
absent block declares that the pack has none; it is not an error here (phase 12 readiness warns). The pack loader
validates the block like the other keys, and every object it names must be approved before any read.""",

"""### 7.N StatementKind and the governor entry points (DR-365)
`StatementKind` is a closed enum with three variants in this phase:
- `Dictionary`: the catalogue, grant and round-trip statements of `ProofScope::Dictionary`. These are the only
  statements a `probe`, `health` or `resume` check runs while the source is not `active` (step 1).
- `Aggregate`: a data statement whose result is one aggregate row (the profile counts of section 3.6). It takes one
  token (step 4).
- `Read`: a data statement that returns rows (the code-list and calendar captures, and the loads of phase 11). It
  takes one token per row fetched (step 4).
Phase 12 adds `ControlTotals` (grouped aggregates, one token per row returned). Later phases extend, and do not change:
`SourceReader` gains `fetch_keyset`, `branch_dates` and `as_of` (phase 11 section 2), and `LoadGovernor::for_source(
source_id)` is the constructor that builds the governor of section 3.5 from the budget in force (phase 11
`ingest_load` step 1).""",

"""### 7.N SelectProof moves to crates/sqlproof (phase 10, DR-85)
Phase 10 moves `SelectProof`, `ProofScope` and `ProvenSelect` from `crates/sources` to the pure crate
`crates/sqlproof`; `crates/sources` re-exports them, so this phase's call sites do not change. Phase 10 also adds two
forms to the grammar of section 3.4: the keyset comparison (a row-value comparison over bound parameters only) and
`FETCH FIRST :n ROWS ONLY`. Both remain single-object, single-statement `SELECT`s proven before they run. The crate
layout of this phase is the state before phase 10.""",

"""### 7.N code_suggestions in discovery.json (phase 10 section 3.4)
The hint schema gains the optional key `code_suggestions`: a CBS code and its list mapped to a canonical code, read by
phase 10's code-map prefill. It is data; an absent key means no suggestions. (`deny_columns.json` is already part of
the pack, section 3.6.)""",

"""### 7.N Listener reload on reconnect (phase 30, DR-322)
When the `LISTEN akr_source` connection of akr-ingestd drops and is re-established, akr-ingestd rebuilds the
`LoadGovernor` of every source from one full read of the state, budget and access rows before it waits again. A
notification is a hint and carries nothing the reload depends on. The hourly re-read stays as the fallback.""",

"""### 7.N Resume of a halted source (phase 11)
Phase 11 extends `POST /sources/{id}/resume` and owns the extension: for a source in its `halted` state the route
enqueues `ingest_resume` (phase 11 job table) after a passing grant check, returns 202, and needs step-up class
`backfill`. A `paused` source resumes as section 3.1 states.""",

"""### 7.N Fixture additions (phase 11)
Phase 11 adds to `cbs-fixture-pg` a `PROCESS` table with a per-branch `BANK_DATE`, a `TRANSACT`-shaped table of about
one million rows over ten business days, and the `restore-to` mode of `akr-ctl fixtures cbs`. All values are
invented; nothing comes from `cbs-discovery/`.""",
]

P06 = [
"""### 7.N The AttributeCatalogue type (DR-367)
`AttributeCatalogue` (`domain::coverage`) is loaded from `backend/catalogue/canonical-attributes.v1.json` (phase 05
section 3.7). One entry per key with the fields of that file (`family`, `module`, `label`, `needs_declaration`,
`used_by`). Methods here: `get(key)`, `keys()`, `needs_declaration(key)`. Keys have the form `<namespace>.<name>`: the
file's keys are `<canon table>.<column>` and are checked against the canonical model by phase 10's test. Two
namespaces are registered in code by later phases and are not canonical columns, so that test skips them: `feat.`
(phase 13 adds `feature_keys()`, the active feature keys with their `requires`) and `rating.` (phase 15 adds
`register_rating_attributes()`: `rating.level`, `rating.cbi_level`). The grader of section 3.2 takes
`&AttributeCatalogue` unchanged. Phase 10's `AttributeService` is a different thing: the registry of bank-defined
attributes (`admin.attribute_values`), not this catalogue.""",

"""### 7.N packs seed and packs verify-data cover the goAML profiles (phase 07)
Phase 07 extends `akr-ctl packs seed` and `akr-ctl packs verify-data` with one step after the CBI pack: each
`backend/packs/goaml/*/` directory of the release manifest goes through phase 07's profile load (section 3.1 there).
The CBI pack steps of section 3.1 do not change.""",

"""### 7.N Scenario register and the routers (phase 16)
Phase 16 owns these changes (its section 4.4): `GET /scenarios` gains the live and shadow versions, the bank grade and
the open-declaration flag; `GET /scenarios/{id}` gains the versions, parameter sets and test summary, and its scopes
become `config.author, config.approve, dashboard.read`. `POST /cbi-catalogue/imports` joins the `catalogue` router and
`GET /coverage/matrix` the `coverage` router. The Cedar actions and the masked 404 of this phase do not change.""",

"""### 7.N Reason state unmapped (phase 10 section 3.8)
The reason-state list in the comment on `scn.readiness_grades.reasons` gains `unmapped`. The grader of section 3.2
reads phase 10's `MappingCompleteness { source_id, entity, column, published_version }`: a monitored attribute with no
published mapping for the source is `unmapped` and grades D, as `missing` does. Draft and testing versions do not
count. No column changes.""",

"""### 7.N Pack kinds, scopes and builders after phases 24 and 31
- Phase 24 migration `0078` replaces `packs_kind_check` with the list of this phase plus `certification`;
  `correspondent` and `validator` are already in the list.
- Phase 24 adds the scope `assessor.read` to `POST /packs`, `GET /packs/{id}` and `GET /packs/{id}/download`
  (Cedar keeps AS to kind `assessor`), and its five kinds to `POST /packs`.
- Phase 31 adds the builders of `correspondent` and `validator` under the flag `pack.correspondent`.
Section 3.6 therefore reads: `POST /packs` accepts three kinds until phase 24 adds its five and phase 31 adds the last
two; any other kind is refused with `unknown_pack_kind`.""",
]

P07 = [
"""### 7.N CheckService::validate_built (phase 21 section 4.5 item 6, DR-171)
`CheckService::validate_built(bytes: &[u8], profile: &ProfileVersion, layers: &[Layer]) -> Result<CheckOutcome,
CheckError>` runs the per-file checks of the `goaml_check` job (section 3.2, from the package step on: package when
the bytes are a package, pre-screen, XSD, the rules of section 3.3 and the score of section 3.4) on XML that the
caller built and holds in memory. It writes no run row, stages nothing and stores nothing; the caller stores the
outcome. The upload path does not change. Phase 21 adds its own layer L0 before calling it.""",

"""### 7.N Rule codes added by phase 21 (DR-368)
The catalogue of section 3.3 gains these rows (columns: code, condition, severity, parameters). The codes arrive in a
release, because the catalogue is closed in code; a profile enables them as it enables the other rules.

| Code | Condition | Severity | Parameters |
|-|-|-|-|
| NAME-01 | an apostrophe, dot, comma, parenthesis or slash removed from a person name | warn | `removed_count` |
| DATE-01 | a birth date known only to the year (month and day UNKNOWN) | warn | |
| ID-01 | a placeholder national identifier (the national ID is missing) | warn | |
| PKG-07 | an attachment whose evidence is still being checked (`pending`) | block | |
| CTR-01 | a CTR transaction without cash on one side and an account on the other | block | |
| CTR-02 | a CTR transaction whose two countries are not both domestic | block | |
| CTR-03 | an extreme or non-round amount in a CTR transaction | warn | |

`CONS-04` is already in the catalogue (info).""",
]

P08 = [
"""### 7.N RailGroup values (DR-369)
`RailGroup` is a closed union, one value per rail entry of DESIGN-SYSTEM 8.2: `my_work`, `command`, `decisions`,
`alerts_cases`, `screening`, `customers`, `kyc_reviews`, `pep`, `rfi`, `qa`, `ops_tasks`, `scenarios`, `policy`,
`governance`, `data`, `reports`, `models`, `assessor`, `administration`, `access_insider`, `audit`,
`platform_health`, `help`. The branch window is a separate bundle and has no group. `screens.ts` lists every screen of
DESIGN-SYSTEM 8.1, including D3 (phase 09, group `policy`) and T3 (phase 10, group `data`), with `available` later
than the running release until their phase.""",

"""### 7.N The Name component (UX-25, DR-373)
The section 3.7 table gains the row `Name`: an original name and its transliteration, each in its own direction
(isolated with `dir`), and the quality flags in words (never colour alone). This phase builds it, since it carries
UX-25. Phase 14 extends it with the chain alignment (father and grandfather), the mother's name and the screening
flags for S1 and the customer screens.""",
]

P09 = [
"""### 7.N Clock registry entries of phases 20 and 21 (DR-370)
Registry entries carry a unit of type `ClockUnit` (`WorkingDays`, `Hours`, `Days`, `CalendarMonths`). The first
three are `BusinessUnit` values and are computed by `BusinessClock::add`. `CalendarMonths` is a label only:
`BusinessUnit` and `BusinessClock` do not change, and the month is computed by phase 20's `add_calendar_months`. The
clock list of section 3.5 gains:
- `freeze.limit`: hours, 24, `fixed: true` (phase 20 section 3.14)
- `freeze.committee_report`: hours, 48, `fixed: true` (phase 20)
- `freeze.second_report`: calendar months, 2, `fixed: true` (phase 20)
- `refrain.clock`: working days, 7, `fixed: true` (phase 20; the refrain clock already named in prose)
- `report.filing_target`: hours, 24, bounds 1 to 24 (phase 21 section 3.18)
- `report.rework`: working days, 10, bounds 5 to 20 (phase 21 section 3.18)""",

"""### 7.N Reason codes seeded for phases 14 and 15 (DR-372)
`reason-codes.v1.json` gains, inactive like the other seeded codes:
- Family `decision`, for screening decisions (phase 14): `DEC_SCR_NAME_DIFFERS`, `DEC_SCR_ID_DIFFERS`,
  `DEC_SCR_DOB_DIFFERS`, `DEC_SCR_FAMILY_NAME_ONLY`, `DEC_SCR_CONFIRMED`, `DEC_SCR_POTENTIAL_REVIEW`,
  `DEC_SCR_INSUFFICIENT_DATA`, `DEC_SCR_OTHER`.
- Family `decision`, for the exit register (phase 15): `DEC_EXIT_FINANCIAL_CRIME`.
- Family `override` (phase 15 section 4.2): `OVR_UPWARD_ADVERSE_INFO`, `OVR_UPWARD_PEP_JUDGEMENT`,
  `OVR_UPWARD_EXPECTED_ACTIVITY`, `OVR_UPWARD_COMPLIANCE_JUDGEMENT`, `OVR_DOWN_SOURCE_DOCUMENTED`,
  `OVR_DOWN_STRUCTURE_VERIFIED`.
They become usable only through the family change request of section 3.7.""",

"""### 7.N The consumer of ref.country_level_changed (phase 15)
The sentences "no consumer until phase 15" (section 3.1 and the rating note of section 4) are closed by phase 15,
which registers the consumer `rating_country_fanout`. The outbox write of this phase does not change.""",

"""### 7.N Backward working-day walk (phase 16 section 3.8 step 5)
`BankCalendarClock::working_days_back(&self, d: NaiveDate, n: u32, calendar_id: Uuid) -> Result<NaiveDate,
ClockError>` walks back `n` working days from the local date `d` over the same day set and weekend pattern that `add`
uses forward (section 3.3). It is a pure method of `BankCalendarClock`, not of the `BusinessClock` trait, and has its
own unit test. A walk past the earliest day the calendar holds returns the same error as `add` past the horizon.""",

"""### 7.N D3 tab Assessment (phase 29 section 5.2)
Phase 29 adds the tab `Assessment` to D3 (panels "Fail policy", "Rail modes", "Prohibited rules"). It has no role
list of its own: like every section of a screen, it renders only when the caller may call one of its operations
(phase 08 `screens.ts` rule), and phase 29's Cedar decides who may.""",

"""### 7.N The segments router under /ref (phase 13)
The `reference` router mounts phase 13's `segments` router under `/ref`. No route row of this phase changes; the
out-of-scope note on segments covers their content, not the mount.""",
]

P10 = [
"""### 7.N Islamic contract types (phase 26)
Phase 26's migration widens the CHECK on `canon.product.islamic_contract_type` with `tawarruq`,
`diminishing_musharaka`, `wakala` and `wadiah`. The base list of this phase does not change.""",

"""### 7.N The two entities of the Islamic pack (phase 26 section 3.2)
Phase 26 adds `islamic_deal` and `deal_party` at the end of `CanonicalEntity` (31 names) and of
`canonical-model.v1.json`, and registers the mapping families `<source>.islamic_deal` and `<source>.deal_party`.
Phase 26 creates both tables. A deal party is never a new subject kind.""",

"""### 7.N Tombstone check in the canonical writer (phase 23 section 4.5 item 3)
From phase 23 on, before `CanonicalWriter` inserts a `canon.customer` or `canon.account` identity, it reads
`admin.retention_tombstones` for the identity's keyed hash (phase 23 section 3.5). A hit skips the row and increments
`akr_ingest_tombstoned_total`. Phase 23 creates the table and grants `akr_ingest` SELECT on it; the writer's other
statements do not change.""",

"""### 7.N Batch indexes (phase 16)
Phase 16 migration `0056` creates `idx_canon_txn_batch` on `canon.txn (batch_id)` and `idx_canon_txn_event_batch` on
`canon.txn_event (batch_id)`. They are physical indexes only; the index list of this phase does not change.""",

"""### 7.N MappingExecutor::map_payload (phase 11)
Phase 11 adds `MappingExecutor::map_payload`: the pure transforms of a version applied to one stored landing payload,
with the same result type as `map`, for replays and retries. `map` does not change.""",

"""### 7.N Volume effect of a mapping change from its replay (phase 11, DR-92)
When a completed replay of the proposed version exists (phase 11), the `mapping` change target takes `volume_effect`
from it. The first publication of a family keeps `neutral`; without a replay the rule of the change-target table
applies. Per DR-360 the value is fixed when the request is submitted.""",

"""### 7.N Address keys of canon.party_contact (DR-374)
The `address` object of a `canon.party_contact` row of type `address` holds at most three keys: `governorate` (a code
of `ref.governorates`, `IQ-XX`, phase 09), `district` and `city` (text as the CBS holds it). The full address stays in
`value_enc`. The mapping fills the keys; a key the source does not hold is absent, never invented. Phases 21 and 24
read these keys. `canonical-model.v1.json` documents them; no constraint changes.""",

"""### 7.N The names crate starts here (DR-371)
This phase creates the pure crate `crates/names` (no IO, no database) with the fold and key tables of research file
22 and the functions behind `FoldArabic` and `SplitArabicName`. The transforms call the crate, so the columns
`light`, `folded` and `key` of `canon.party_name` come from it. Phase 14 extends the same crate (normaliser, forms,
parser, SQL twin, parity corpus) and stays the only place where the rules live. Screening recomputes its forms from
`raw` and never reads the stored columns; when phase 14 changes a table, the stored columns are refreshed by a mapping
replay (phase 11), not by a separate backfill.""",

"""### 7.N T3 in screens.ts (DR-369)
The T3 entry of section 5 uses `rail.group: 'data'` (phase 08 `RailGroup`), the next free `order` in that group, and
the mapping icon of `IconName`.""",
]

P14 = [
"""### 7.N The names crate and the Name component exist before this phase (DR-371, DR-373)
`crates/names` is created by phase 10 (its section 7+, the folding behind `FoldArabic` and `SplitArabicName`); this phase
extends it, so the tree line "NEW" reads "extended". The Open doubt "Stored forms in canon" is closed: phase 10's
transforms call the crate, and screening keeps ignoring the stored forms. The `Name` component is built by phase 08
(UX-25); this phase extends it, which closes the Open doubt "Name component".""",
]

P21 = [
"""### 7.N Rule codes enter phase 07 in a release (DR-368)
Section 3.13 step 12 and section 4.5 item 6: the CTR codes, like the other new codes, enter phase 07's catalogue in a
release (the catalogue is closed in code), not by change request. `CONS-04` is not new: phase 07 already has it
(info). CTR-01 and CTR-02 are block, CTR-03 warn (phase 07 section 7+).""",
]

TARGETS = {'05': (HDR, P05), '06': (HDR, P06), '07': (HDR, P07), '08': (HDR, P08), '09': (HDR, P09),
           '10': (HDR, P10), '14': (CHDR, P14), '21': (CHDR, P21)}

out = {}
for pp, (hdr, blocks) in TARGETS.items():
    p = PLAN / f'phase-{pp}.md'
    s = add_section7(p.read_text(), hdr, blocks)
    i = s.index('## 7+. Gap Additions')
    j = s.find('\n## ', i + 5)
    tail = s[i:j if j != -1 else len(s)]
    for w in ('UNCLEAR', 'the lead decides', 'lead confirms'):
        assert w not in tail, (p.name, w)
    out[p] = s

# phase 21: the step 12 sentence that says "by change request"
p21 = PLAN / 'phase-21.md'
out[p21] = rep(out[p21], 'enter the phase 07 rule catalogue by change request (section 4.5)',
               'enter the phase 07 rule catalogue in a release (section 4.5; DR-368)')
# phase 05 job table payload
p05 = PLAN / 'phase-05.md'
out[p05] = rep(out[p05], '`discovery_seal_report` (payload `{run_id}`; concurrency 2;',
               '`discovery_seal_report` (payload `{run_id}` or `{check_id}`, DR-366; concurrency 2;')

# research.md DRs
r = PLAN / 'research.md'
rs = r.read_text()
anchor = [l for l in rs.split('\n') if l.startswith('| DR-364 |')]
assert len(anchor) == 1
DRS = [
    "| DR-365 | 2026-10-10 | `StatementKind` is closed: `Dictionary`, `Aggregate` (one token), `Read` (one token per row); phase 12 adds `ControlTotals` | Phase 05 named the type with no variants while phases 11 and 12 used them (gap pass 1, phase 05 section 7+) |",
    "| DR-366 | 2026-10-10 | `discovery_seal_report` takes `{run_id}` or `{check_id}`; only a completed profile run enqueues `coverage_grade` and `drift_scan`, in the seal transaction | Phase 05 enqueued both payloads but listed one; phases 06 and 10 relied on the enqueue (gap pass 1, phase 05 section 7+) |",
    "| DR-367 | 2026-10-10 | `AttributeCatalogue` keys are `<namespace>.<name>`: file keys `<canon table>.<column>`, plus the code-registered namespaces `feat.` (phase 13) and `rating.` (phase 15), which the phase 10 column test skips | Phases 13 and 15 extend a type phase 06 never defined; phase 10 `AttributeService` is the bank-defined registry, a different type (gap pass 1, phase 06 section 7+) |",
    "| DR-368 | 2026-10-10 | Phase 21's rule codes enter phase 07's catalogue in a release; CTR-01 and CTR-02 are block, CTR-03 warn; NAME-01 warn as phase 21 section 3.3 defines | The catalogue is closed in code; a CTR without the cash/account shape or with a foreign country is not a CTR (gap pass 1, phase 07 section 7+) |",
    "| DR-369 | 2026-10-10 | `RailGroup` has one value per rail entry of DESIGN-SYSTEM 8.2 (23 values; `policy` for D3, `data` for T3) | Phase 08 named the type without values; phase 10 needed a data group (gap pass 1, phase 08 section 7+) |",
    "| DR-370 | 2026-10-10 | Clock registry entries carry `ClockUnit` = `BusinessUnit` values plus the label `CalendarMonths`; months are computed by phase 20's `add_calendar_months`; `BusinessUnit` unchanged | `freeze.second_report` is two calendar months, which no business unit expresses (gap pass 1, phase 09 section 7+) |",
    "| DR-371 | 2026-10-10 | Phase 10 creates the pure crate `crates/names` (fold and key tables); phase 14 extends it; screening ignores the stored forms, refreshed by a mapping replay | Phase 10 runs before phase 14 yet writes folded names; one place for the rules (gap pass 1, phase 10 section 7+; closes a phase 14 Open doubt) |",
    "| DR-372 | 2026-10-10 | The eight screening decision codes are `DEC_SCR_NAME_DIFFERS`, `_ID_DIFFERS`, `_DOB_DIFFERS`, `_FAMILY_NAME_ONLY`, `_CONFIRMED`, `_POTENTIAL_REVIEW`, `_INSUFFICIENT_DATA`, `_OTHER` | Phase 14 described them without codes (gap pass 1, phase 09 section 7+) |",
    "| DR-373 | 2026-10-10 | Phase 08 builds the base `Name` component (UX-25); phase 14 extends it | Phase 08 carries UX-25 but listed no component; phase 14 said it adds it (gap pass 1, phase 08 section 7+) |",
    "| DR-374 | 2026-10-10 | `canon.party_contact.address` holds at most `governorate` (`IQ-XX`), `district` and `city`; the full address stays sealed in `value_enc` | Phases 21 and 24 read these keys, which phase 10 never named; plaintext limited to coarse location (gap pass 1, phase 10 section 7+) |",
]
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + '\n'.join(DRS) + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
