#!/usr/bin/env python3
"""Apply gap pass 1, run 4b (targets 25 to 31 and the system documents) after the lead's checks, with the carried items.

Source: tools/agent/gap1-run4b.md (Haiku draft); decisions: tools/agent/gap1-run4b-verdict.md.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'
DOCS = ROOT / 'docs'

HDR = ('Gap pass 1 (2026-10-10), run 4b, from tools/agent/gap1-run4b.md; lead-verified '
       '(tools/agent/gap1-run4b-verdict.md).')
CHDR = ('Gap pass 1 (2026-10-10), run 4b: items carried from run 4a and corrections found while checking '
        'phases 25 to 31 (tools/agent/gap1-run4b-verdict.md); lead-verified.')

P04C = [
"""### 7.N Model inventory items keep R9 (DR-392)
`admin.retention_assignment` is keyed by table, so one table cannot be R9 for some rows and R11 for others. The
retention row of `admin.inventory_items` stays R9 `retirement` for every row, model items included; the model evidence
is R11 through phase 27's own tables (`admin.models`, `admin.model_versions`, `admin.model_evaluations`). The note of
that retention row is corrected to say so.""",
]

P10C = [
"""### 7.N Canonical payment status values (DR-389)
`canon.payment_message.status` takes the canonical values `received`, `released`, `rejected`, `returned` and
`cancelled`, mapped from the CBS value through the `status` code map of this phase (kind `status` already exists in
`admin.code_maps`; an unmapped value follows the map's `gate`). The column of migration section 1 reads
`status TEXT NOT NULL CHECK (status IN ('received', 'released', 'rejected', 'returned', 'cancelled'))`; the values
appended to `status_history` use the same list. Phase 29's release control reads this column (phase 29 section 7+).""",
]

P24C = [
"""### 7.N Canary records are P2 (DR-391)
Open doubts 12 is closed: canary records (SP-30, research 30 sections 8.6 and 8.8) are P2 and no phase of this plan
builds them (phase 25 section 7+). The control report keeps `canary_not_built: true` and the "not built" line.""",
]

P25 = [
"""### 7.N Tables excluded from the support bundle and the SIEM (phases 27, 28, 29)
The support bundle (section 3.8 step 1) writes no row count and no row content for these tables, and the SIEM export
(section 3.5) reads the audit log only, so they reach it only through audit payloads, which carry ids and counts:
`alert.model_scores` (phase 27), `casework.assistant_drafts` and `admin.assistant_evaluations` (phase 28),
`assess.decisions`, `assess.holds`, `assess.hold_candidates` and `assess.release_exceptions` (phase 29). Test
(section 6): the bundle fixture's row-count list contains none of the seven names.""",

"""### 7.N Releases do not import models or assistant versions (DR-390)
The release path verifies the bundle only (section 3.6). Model and assistant artifacts that a bundle carries are not
imported by the verify job: people import them through phase 27 section 3.3 and phase 28's version import, which set
status `registered` and nothing else (GV-29). A release never sets a model or an assistant version live, shadow or
challenger, and its status does not depend on those imports.""",

"""### 7.N Policy matrix file hash in the release check
Verify job step 3 also stores the SHA-256 of the JSON output of `akr-ctl policy matrix --bundle <dir> --json` (the
bytes that `make generate-api` writes to `frontend/src/app/policy-matrix.json`, phase 03) as the key
`matrix_file_sha256` inside `policy_matrix_diff`. This is the hash phase 03 says the release check records. No new
column; a difference is shown in the diff and is not a refusal.""",

"""### 7.N Runbook: rotation of `keys/auth-hmac`
Phase 01 puts this runbook in phase 25. The KV entry `akr/keys/auth-hmac` (DR-356) holds the master of the HKDF
subkeys `csrf`, `token` and `login_identifier` (phase 01). Steps for the bank operator on the bank host:
1. Write a new KV version of `akr/keys/auth-hmac` with a fresh random value, under the operator's OpenBao policy.
2. Restart `akr-api` so every replica derives the subkeys from the new version.
3. Tell the holders of personal tokens that every token is invalid and must be created again.
4. Check: a request with a CSRF cookie minted before the rotation gets a new cookie; an old personal token is refused.
The effects are the ones phase 01 states: CSRF cookies are re-minted at the next request and every personal token
becomes invalid. Sessions are not claimed to end.""",

"""### 7.N Chart and K3s extensions owned by phase 30
Phase 30 owns these changes (its migration `0095` and section 3.12): the CHECK of `admin.releases.refusal_code`
(section 1.2) gains `chart_signature_invalid`; the verify job gains check (g), the Helm chart check, after check (f),
and the refusal order of step 4 ends at (g); `cosign_result` gains the chart count and the K3s version (no signature
bytes); A4 shows both facts; `release.refused` carries the new code.""",

"""### 7.N Rollout record on the apply
`POST /admin/releases/{id}/applied` takes an optional `rollout_evidence_id`. The operator first uploads the
`rollout.json` that phase 30's upgrade script writes, as evidence of kind `package`, owner kind `release` and owner id
this release (phase 02 evidence store). The route checks that the evidence has that owner (else `E_VALIDATION`). No
column is added: the evidence object's owner link is the record. Open doubts 14 keeps its point: the record is the
operator's file, not a proof of the apply.""",

"""### 7.N No cluster objects in the support bundle (phase 30)
Section 3.8 step 1 collects no Kubernetes object, event, pod log or cluster state (phase 30 section 3.14); the
application logs in the archive are the redacted logs of step 2 only.""",

"""### 7.N Read-only grant check reused by later phases
The database tests of section 6 assert `has_table_privilege('akr_readonly', t, 'SELECT')` is false for each table of
section 1, as a loop over a table list. Later phases add their tables to the same list (phase 26's read-only check of
`kyc.possession_evidence` is one).""",

"""### 7.N Scope of the one-year expiry
The one-year expiry of sections 3.5 and 4.5 applies to the SIEM pull principal only. The expiry of other service
principals, such as the gateway certificates of phase 29, is decided by their phase (phase 29 Open doubts 24).""",

"""### 7.N Canary records are P2 (DR-391)
SP-30 canary records (research 30 sections 8.6 and 8.8) are not built in this plan; they are P2. Phase 24's control
report keeps `canary_not_built: true` (phase 24 section 7+ closes its Open doubts 12).""",
]

P26 = [
"""### 7.N Register warning call (DR-388)
Section 3.5 step 4 calls `RegisterWriter::warn(tx, 'scan', 'islamic_evidence_gap:' || bank_date,
WarningTitle::IslamicEvidenceGap, body)` inside the sweep's transaction, with `body = {"missing": n, "late": n}`.
This phase adds the variant `IslamicEvidenceGap` to phase 24's closed enum `WarningTitle`, with the bilingual title in
the message catalogue. The insert-or-nothing on `source_key` keeps one row per bank day. The ports list of section 2
reads the phase 24 signature.""",

"""### 7.N Party predicate form
The party rule is phase 18's `PartyReadPredicate::allows(principal, party_id, context)` (phase 18 section 7+), not a
function built by phase 17. Section 3.4 step 7 calls it with the same three arguments as the ports list of section 2:
the caller, the party id and the deal read's context. The omitted-party rule of step 7 stays.""",
]

P27 = [
"""### 7.N Validation date of a model family
When the validation pack of a model gets verdict `pass` (phase 24 section 3.6 step 8), phase 24 calls phase 04
`InventoryService::update` on the inventory item to set `last_validated_at = now()` and `next_validation_due = today +
review.rules_months`. The phase 24 review calendar is read-only; it lists the item when the date nears. Section 2's
phase 04 line says the date is moved by a passing validation.""",

"""### 7.N Masking probe for the model-score route
Section 6 adds route 16 (`GET /alerts/{id}/model-score`) to the probe set of kind `alert` in phase 03's
`MASKED_KINDS` registry: an alert the caller may not see answers the masked 404, byte for byte the answer for an
unknown id, and the body carries no score, band or reason code.""",

"""### 7.N Inventory items keep R9 (DR-392)
`admin.inventory_items` keeps R9 for every row, model items included; model evidence is R11 through the retention rows
of section 1.1. Phase 04's note "model items move to R11 in phase 27" is corrected (phase 04 section 7+).""",
]

P27C = [
"""### 7.N Advance notice (Open doubts 9 closed)
The GV-29 notice is the audit record `model.version_imported` and the `pending_notices` list on `GET /models`. No
e-mail or in-app notice is sent in this release (the rule of phase 22 Open doubts 14: the screens are the notice).""",

"""### 7.N Release bundles import no model (DR-390)
A model that a phase 25 release carries is imported by people through section 3.3, as `registered` only; the release
itself imports nothing (phase 25 section 7+). The sentences of this phase that said "the release import" read so.""",

"""### 7.N Retention DELETE grant (DR-386)
`akr_retention` gets SELECT on the six new tables and DELETE on `alert.model_scores` only: its rows are deleted by
`record_purge` before an alert partition is dropped (DR-387). `admin.model_evaluations` (R11) is `report_only`,
`admin.model_monitor_points` (R9) is kept, the R11 `retirement` and R10 tables are never purged. The delete guards
that admit `akr_retention` stay; they admit, they do not grant.""",
]

P28C = [
"""### 7.N Retention DELETE grant (DR-386)
`akr_retention` gets SELECT only on the four new tables: no handler deletes their rows in this release (R11 and the
R6 drafts are `report_only`, R10 is never purged). The delete guards that admit `akr_retention` stay.""",

"""### 7.N Release bundles import no assistant version (DR-390)
A signed assistant artifact that a phase 25 release carries is imported by people through this phase's version
import, as `registered` only; the release itself imports nothing (phase 25 section 7+).""",
]

P29 = [
"""### 7.N Valkey outage (owner phase 30)
Phase 30 section 3.6 removes Valkey from the readiness list of `akr-assess` (section 4.4) and adds the in-process
`LimiterFallback` that answers the rate limit of section 3.1 step 3 while Valkey does not answer, with the capacity
phase 30 defines. No other rule of step 3 changes.""",

"""### 7.N Reconnect reload of the caches (owner phase 30)
On a reconnect of the `LISTEN` connection, `PolicyCache` and `GatewayCache` reload at once (phase 30). The 30-second
refresh of both caches stays as the bound when a notification is lost.""",

"""### 7.N Register warning titles (DR-388)
This phase adds seven variants to phase 24's closed enum `WarningTitle`, each with a bilingual title in the message
catalogue, and writes every warning with `RegisterWriter::warn(tx, 'assess_control', source_key, title, body)` from a
job (body: counts and ids only):
- `assess_rollup_watch` (section 3.12 step 3): `AssessKeyAge`, `AssessCertificateExpiry`, `AssessFallbackStale`,
  `AssessBlindPeriod` (an open outage period), `AssessControlTestStale`, `AssessRepeatAttempts`;
- `assess_blind_watch` (section 3.13): `AssessBlindPeriod` (`source_key = 'assess:blind:' || started_at`);
- `assess_release_match` (section 3.14 step 5): `AssessReleaseException`.""",

"""### 7.N Gateway circuit breaker
The circuit breaker of section 4.4 is the bank gateway's, not an AKR-TMS component. Its threshold is a bank setting
and must not open before `ops.failover_max_s` (phase 30 Open doubts 10). AKR-TMS adds no setting for it.""",
]

P29C = [
"""### 7.N Payment status in the release control (DR-389)
Phase 10 fixes the values of `canon.payment_message.status` (phase 10 section 7+). Section 3.14 step 1 skips rows
whose status is `rejected` or `cancelled` when they are read; any other value, an unexpected one included, is
controlled, so the control fails closed.""",

"""### 7.N Retention DELETE grant (DR-386)
`akr_retention` gets SELECT on the 19 tables and DELETE on `assess.node_status` only (R12 `age`, no owner sweep, so
the engine's `record_purge` deletes it, DR-387). `assess.idempotency` is purged by `assess_ledger_sweep` as `akr_app`;
the R7 tables are `report_only`; `assess.decisions` partitions leave through `admin.retention_drop_partition`; R9 and
R10 tables are kept. Sections 1.3 and 4.5 item 6 read so.""",
]

P30 = [
"""### 7.N Pack flags in the catalogue
A catalogue entry may carry `pack: <flag name>`. `akr-ctl deploy render` takes `--pack <name>=on|off` for each flag the
catalogue names; a flag not given is off. An entry whose flag is off is left out of every output (compose, chart, env
keys and secret paths), so with all flags off the output is the phase 00 bundle byte for byte (section 3.1 step 2).
The `polis` entry and its flag `pack.polis` are phase 31's; this phase owns the mechanism. The render reads the flag
from the command line, not from the database.""",
]

P30C = [
"""### 7.N Deletion certificate key
Restore step 1 verifies the deletion certificates against `akr-sign-deletion` (phase 23 section 7+, DR-357); the name
`akr-sign-cert` is not used.""",

"""### 7.N Reconciliation after a restore (Open doubts 23 closed)
Phase 12 section 7+ settles it: restore verify runs in order, runs of `recon_run` for a restore gap are valid while the
replay runs, and a date not yet reloaded shows as a break.""",

"""### 7.N Who sees failover and restore reports
They are viewable by `admin.system`. SEC sees their audit events (`failover_drill.recorded`, `restore_check.recorded`,
SIEM yes) in the audit viewer and the SIEM forward. Phase 25 sends no notice for them.""",
]

P31 = [
"""### 7.N Directory link port (phase 03 section 7+)
Section 3.12 implements phase 03's `DirectoryLinkQuery::is_inactive(user_id) -> bool` as `PgDirectoryLinkQuery`: TRUE
when a row of `admin.directory_users` has `linked_user_id = user_id` and `directory_active = FALSE`, else FALSE. It is
wired into `AppState` when `pack.polis` is on; with the flag off, phase 03's `NoDirectory` stays. Test: enabling a
linked user whose directory row is inactive is refused as phase 03 states.""",
]

P31C = [
"""### 7.N Retention DELETE grants (DR-386)
`train.practice_attempts` is R9 and kept in this release (its period is Open doubts 8), so `akr_retention` gets SELECT
only until a period and its handler exist. `rpt.typology_exposure` is R7 `report_only`: SELECT only.
`casework.capacity_snapshots`, `casework.capacity_runs` and `admin.directory_events` are R12 `age` with no owner sweep,
so the engine's `record_purge` deletes them and their DELETE grant stays.""",

"""### 7.N QA sampler citation
Section 4.5 item 9: the sampler is phase 19 section 3.8.""",
]

TARGETS = [('04', CHDR, P04C), ('10', CHDR, P10C), ('24', CHDR, P24C), ('25', HDR, P25), ('26', HDR, P26),
           ('27', HDR, P27), ('27', CHDR, P27C), ('28', CHDR, P28C), ('29', HDR, P29), ('29', CHDR, P29C),
           ('30', HDR, P30), ('30', CHDR, P30C), ('31', HDR, P31), ('31', CHDR, P31C)]

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


def edit(name, pairs, base=PLAN):
    p = base / name
    s = out.get(p, p.read_text())
    for a, b in pairs:
        s = rep(s, a, b)
    out[p] = s


edit('phase-04.md', [
    ("'Retirement = retired; model items move to R11 in phase 27')",
     "'Retirement = retired; model items stay R9, model evidence is R11 in phase 27 tables (DR-392)')"),
])

edit('phase-09.md', [
    ("phases 13 and 14; the typology set comes with\n  the scenario register in phase 16)",
     "phases 13 and 14; the typology set is phase 06's,\n  DR-381)"),
])

edit('phase-26.md', [
    ("(phase 18 section 3.10 party rule, built by phase 17\n  `services::visibility::build` for party rows), "
     "`RegisterWriter::warn(source, source_key, title_key, detail)` (phase 24, upsert with\n"
     "  `ON CONFLICT (source_key) DO NOTHING`).",
     "(phase 18 section 3.10 party rule, phase 18 section 7+),\n"
     "  `RegisterWriter::warn(tx, source, source_key, title_key, body)` (phase 24 section 7+, DR-388; insert-or-nothing with\n"
     "  `ON CONFLICT (source_key) DO NOTHING`)."),
    ("`PartyReadPredicate::allows(ctx, party_id)` (phase 18 party rule)",
     "`PartyReadPredicate::allows(principal, party_id, context)` (phase 18 party rule)"),
    ("new screen, catalogue id to be assigned by the design owner, proposed `I1`, new in phase 26",
     "new screen, catalogue id `I1` (DESIGN-SYSTEM 8.1, gap pass 1), new in phase 26"),
])

edit('phase-27.md', [
    ("Phase 25 (release import, which may carry model artifacts but never sets a model live)",
     "Phase 25 (release bundles, which may carry model artifacts; the release imports none and never sets a model live, DR-390)"),
    ("A phase 25 release that carries a model imports it through section 3.3 with status `registered` and nothing else.",
     "A model that a phase 25 release carries is imported by people through section 3.3 with status `registered` and "
     "nothing else (DR-390)."),
    ("The e-mail notice to MRO is deferred (Open doubts 9);",
     "No e-mail or in-app notice is sent in this release (Open doubts 9, closed in section 7+);"),
    ("It is written by the signed bundle import (phase 00 `bundle import`) and the release import of phase 25 (starter models), and by",
     "It is written by the signed bundle import (phase 00 `bundle import`, which also brings the starter models a phase 25 "
     "release carries; the release itself imports no model, DR-390), and by"),
    ("`next_validation_due` of the family is set at creation and moved by the review calendar's existing route.",
     "`next_validation_due` of the family is set at creation and moved by a passing validation (section 7+)."),
    ("the release import of section 3.7 imports a model artifact as `registered` only.",
     "a model that a release carries is imported by people through section 3.3 as `registered` only (DR-390)."),
    ("a phase 25 release imports model artifacts as `registered` only;",
     "a model that a phase 25 release carries is imported as `registered` only (by people, DR-390);"),
    ("and the release import must set `registered` only. Neither is designed in phase 25 yet.",
     "and the release imports no model (DR-390). Both are stated in phase 25 section 7+."),
    ("-- Retention: own SELECT and DELETE on every new table (phase 23 grants SELECT on ALL TABLES only at its own time).\n"
     "GRANT SELECT, DELETE ON admin.models, admin.model_versions, admin.model_evaluations, admin.model_monitor_points,\n"
     "    admin.model_kill_state, alert.model_scores TO akr_retention;",
     "-- Retention: SELECT on every new table (phase 23 grants SELECT on ALL TABLES only at its own time); DELETE only on\n"
     "-- alert.model_scores, deleted by record_purge before an alert partition drop (DR-386, DR-387).\n"
     "GRANT SELECT ON admin.models, admin.model_versions, admin.model_evaluations, admin.model_monitor_points,\n"
     "    admin.model_kill_state TO akr_retention;\n"
     "GRANT SELECT, DELETE ON alert.model_scores TO akr_retention;"),
])

edit('phase-28.md', [
    ("-- Retention: own SELECT and DELETE on every new table (phase 23 grants SELECT on ALL TABLES only at its own time).\n"
     "GRANT SELECT, DELETE ON admin.assistant_versions, admin.assistant_evaluations, admin.assistant_kill_state TO akr_retention;\n"
     "GRANT SELECT, DELETE ON casework.assistant_drafts TO akr_retention;",
     "-- Retention: SELECT on every new table (phase 23 grants SELECT on ALL TABLES only at its own time). No DELETE: no\n"
     "-- handler of these tables deletes rows in this release (R11 and R6 report_only, R10 never; DR-386).\n"
     "GRANT SELECT ON admin.assistant_versions, admin.assistant_evaluations, admin.assistant_kill_state TO akr_retention;\n"
     "GRANT SELECT ON casework.assistant_drafts TO akr_retention;"),
    ("the release import may carry a signed assistant artifact, which is `registered` only (Open doubts 15).",
     "a release bundle may carry a signed assistant artifact, which people import through this phase's version import "
     "as `registered` only; the release imports none (DR-390, Open doubts 15)."),
    ("and a release imports an assistant artifact as `registered` only. Neither is designed in phase 25 yet.",
     "and the release imports no assistant artifact (DR-390). Both are stated in phase 25 section 7+."),
])

edit('phase-29.md', [
    ("path. akr_retention gets SELECT and DELETE on every new table (phase 23). akr_readonly gets nothing on assess.",
     "path. akr_retention gets SELECT on every new table and DELETE on assess.node_status only (DR-386). akr_readonly\n"
     "-- gets nothing on assess."),
    ("-- akr_retention: SELECT and DELETE on every new table (phase 23 grants only tables that existed at its migration).\n"
     "GRANT SELECT, DELETE ON assess.rails,",
     "-- akr_retention: SELECT on every new table (phase 23 grants only tables that existed at its migration); DELETE only\n"
     "-- on assess.node_status, the one table whose handler deletes rows (record_purge, R12 age; DR-386).\n"
     "GRANT SELECT ON assess.rails,"),
    ("    TO akr_retention;\n\n-- akr_readonly and akr_audit_writer receive nothing",
     "    TO akr_retention;\nGRANT DELETE ON assess.node_status TO akr_retention;\n\n"
     "-- akr_readonly and akr_audit_writer receive nothing"),
    ("`akr_retention` gets SELECT and DELETE on the 19 tables of this phase (section 1.3).",
     "`akr_retention` gets SELECT on the 19 tables of this phase and DELETE on `assess.node_status` only (section 1.3, "
     "DR-386)."),
    ("1. **Scope**: canonical payments with `direction = 'out'` on a rail whose open mode is `advisory` or `enforcing`.",
     "1. **Scope**: canonical payments with `direction = 'out'` and a `status` other than `rejected` or `cancelled` "
     "(phase 10 section 7+, DR-389; any other value is controlled, so the control fails closed) on a rail whose open "
     "mode is `advisory` or `enforcing`."),
])

edit('phase-30.md', [
    ("are verified against `akr-sign-cert`.",
     "are verified against `akr-sign-deletion` (phase 23 section 7+, DR-357)."),
    ("Viewable by `admin.system` and by the SEC role through the phase 25 notices only.",
     "Viewable by `admin.system`; SEC sees their audit events (audit viewer and SIEM forward), and phase 25 sends no "
     "notice (section 7+)."),
])

edit('phase-31.md', [
    ("GRANT SELECT, DELETE ON train.practice_attempts TO akr_retention;   -- phase 23 retention run (the guard admits only this role)",
     "GRANT SELECT ON train.practice_attempts TO akr_retention;   -- R9 kept: no DELETE until a period and handler exist (DR-386)"),
    ("GRANT SELECT, DELETE ON rpt.typology_exposure TO akr_retention;",
     "GRANT SELECT ON rpt.typology_exposure TO akr_retention;   -- R7 report_only (DR-386)"),
    ("the sampler of phase 19 section 3.9 takes", "the sampler of phase 19 section 3.8 takes"),
])

# System documents.
edit('DESIGN-SYSTEM.md', [
    ("| G10 | Change requests | `/changes` | LIST + detail | P0 (phase 08) |\n",
     "| G10 | Change requests | `/changes` | LIST + detail | P0 (phase 08) |\n"
     "| I1 | Sharia board aggregates | `/islamic/sharia-board` | DASH | P1 (phase 26, optional pack) |\n"),
    ("**Count:** 62 screens (59 from research file 31 plus E1, E2 and G10, the change-request screen behind the rail's\n"
     "\"Governance (change requests)\" entry, added by phase 08, DR-74), 8 with",
     "**Count:** 63 screens (59 from research file 31 plus E1, E2, G10, the change-request screen behind the rail's\n"
     "\"Governance (change requests)\" entry, added by phase 08, DR-74, and I1, the Sharia board view of phase 26, added by\n"
     "gap pass 1), 8 with"),
    ("and opens its list. Open work reads \"1,284 alerts in 911 cases\". Stacked bars gapped and labelled.\n",
     "and opens its list. Open work reads \"1,284 alerts in 911 cases\". Stacked bars gapped and labelled. From phase 29,\n"
     "the Assessment tile (phase 29 section 5.3): modes per rail, list freshness, open blind periods and open release\n"
     "exceptions, counts only; visible to M, CH and IA; it opens the Assessment tab of D3 for M and CH and the\n"
     "release-without-decision list for IA.\n"),
], base=DOCS)

ds = DOCS / 'DESIGN-SYSTEM.md'
s = out[ds]
S2 = ("**S2 Screening queue, tab Holds (phase 29 section 5.1).** A tab beside the alert queue lists payment holds\n"
      "(`GET /api/v1/assess/holds`) with the rail, the mode label (\"held at the gateway\", \"advisory: not held at the\n"
      "gateway\", \"shadow: no gateway effect\"), reason codes, age, the countdown to expiry and the proposal state. A\n"
      "suspicion-tier subject the caller may not see is not listed. Claim, propose release or reject, and approval by a\n"
      "second person follow the alert rules of S2, each with step-up class `decision`.\n")
D3 = ("**D3 Policy studio, tab Assessment (phase 29 section 5.2).** Three panels: Fail policy (the matrix as a table and\n"
      "the bands per currency), Rail modes (one row per rail with its mode, effective-from instant and readiness criteria\n"
      "with observed and required values) and Prohibited rules (read-only list with \"Propose a rule\"). Each panel says\n"
      "that a change is material and needs MLRO and CHK, and CH for a decrease.\n")
for lead, para in (('**S1 Screening review (mockup 03', S2), ('**D2 Scenario studio (mockup 05', D3)):
    assert s.count(lead) == 1
    i = s.index(lead)
    j = s.index('\n\n', i)
    s = s[:j + 2] + para + '\n' + s[j + 2:]
out[ds] = s

edit('MODULES.md', [
    ("view's screen id is proposed `I1` and is not yet in the DESIGN-SYSTEM catalogue.",
     "view's screen id is `I1` in the DESIGN-SYSTEM catalogue (gap pass 1)."),
], base=DOCS)
edit('README.md', [("screen catalogue (62 screens)", "screen catalogue (63 screens)")], base=DOCS)
edit('roadmap.md', [("62 screens (DESIGN-SYSTEM.md 8.1; G10 added by phase 08)",
                     "63 screens (DESIGN-SYSTEM.md 8.1; G10 added by phase 08, I1 by gap pass 1)")])

r = PLAN / 'research.md'
rs = r.read_text()
anchor = [l for l in rs.split('\n') if l.startswith('| DR-388 |')]
assert len(anchor) == 1
DRS = [
    "| DR-389 | 2026-10-10 | `canon.payment_message.status` takes `received`, `released`, `rejected`, `returned`, `cancelled` through phase 10's `status` code map (CHECK in phase 10); phase 29's release control skips `rejected` and `cancelled` and controls every other value | The release control read a status no phase defined; failing closed keeps an unknown status under control (gap pass 1, phase 10 and 29 section 7+) |",
    "| DR-390 | 2026-10-10 | A phase 25 release verifies the bundle only and imports no model or assistant version; people import them through phase 27 section 3.3 and phase 28's import, as `registered` only | Phases 27 and 28 assumed a release import phase 25 never designed; a person-driven import keeps GV-29 and the four eyes (gap pass 1, phase 25 section 7+) |",
    "| DR-391 | 2026-10-10 | Canary records (SP-30, research 30 sections 8.6 and 8.8) are P2; phase 24 keeps `canary_not_built: true` | No phase built them and phase 24 named two possible owners; the control report already shows the gap (gap pass 1, phase 24 and 25 section 7+) |",
    "| DR-392 | 2026-10-10 | `admin.inventory_items` stays R9 for every row; model evidence is R11 through phase 27's tables | A retention class is per table, so phase 04's per-row move to R11 could not be built (gap pass 1, phase 04 and 27 section 7+) |",
]
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + '\n'.join(DRS) + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
