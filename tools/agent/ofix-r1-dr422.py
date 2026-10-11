#!/usr/bin/env python3
"""Owner-decision round 1: DR-422 (the design corrections of the round) in docs/plan/research.md, and one citing
sentence at the end of the opening paragraph of each owner block that it corrects."""
import sys
sys.path.insert(0, "/home/haithem/Projects/AKR-TMS/tools/agent")
from ofix import Fixer, ROOT

f = Fixer(round_no=1)

ROW = ("| DR-422 | 2026-10-11 | Design corrections of the check round on the owner decisions (round 1; every verdict is in "
       "tools/agent/owner-verdict.md). They change what DR-414 to DR-420 build, so they are locked here. "
       "(1) Writer, phases 10 and 26 (DR-419, DR-420): `capture_source`, `paid_on`, `paid_amount` and `granted_on` are "
       "compared as mapped values, also when the hash of the landed row is equal (`set_capture_source`, `pay_instalment`, "
       "`set_granted_on`, each with its column grant); the index on `(facility_id, seq)` is not unique; `paid_amount` may be "
       "set while `paid_on` is null. "
       "(2) RFI pre-fill, phase 19 (DR-417): four of the ten sources are sensitive and computed for the MLRO only "
       "(`accounts.closed`, `accounts.signatories`, `activity.declared_and_actual`, `rating.cbi_level`); `party.pep_status` "
       "answers from the bank's PEP register (`kyc.party_pep_state`) before the customer file; the activity figures are read "
       "through the phase 13 profile query under the purpose `investigation`; a hidden signatory is left out with no flag. "
       "(3) Source tags, phase 13 (DR-416): the activation closes the retire entries before it inserts the add entries; a "
       "second add with the same hash for one tag is refused (`already_tagged`); the default end date is the later of the "
       "row's `valid_from` and the last closed business date; a feature reads a tag as `tag.<name> = true` or `= false`; when "
       "a feature reads a changed tag, the activation enqueues `feature_compute {reopen_from}` under its own coalesce key. "
       "(4) Bank cases, phases 07 and 14 (DR-415): a stale intake is run again by the activation of the `goaml_profile` "
       "version, for `proposed` cases with a stored file, and not by the approval check; each deletion of an uploaded file "
       "writes `goaml_corpus_case.transient_deleted`; the mask keeps an Arabic letter inside the class of patch `P-01`; the "
       "screening harness report carries one result per case (`results`) in place of `unmet_ids`. "
       "(5) SIEM, phase 25 (DR-414, DR-418): the last CHECK of `admin.siem_dead_letters` is "
       "`((resolution = 'marked') IS TRUE) = (resolution_code IS NOT NULL)`; an access event recorded after the last "
       "off-run is sent when the export is switched on again. "
       "(6) Check lines, phases 06, 07, 09, 13, 14, 15, 17, 18, 19, 20 and 21: a statement by a role without the privilege "
       "fails with permission denied, and the guard message is expected as the owner `akr_migrate` "
       "| Found by the round-1 agents and the lead. Each would have made a stated check fail, or left an owner decision "
       "with no effect on rows already stored. Three points are written down and not redesigned, for the build: the replay "
       "apply against the landed-hash guard (phase 11 Open doubts 14), instalment rows a source replaces or deletes "
       "(phase 26 Open doubts 24), and the replay enqueue under the plain coalesce key (phase 13 Open doubts) |\n")

ANCHOR = "replaced by DR-414 to DR-420; the rest of DR-413 stands |\n"
f.fix("DR-422", "docs/plan/research.md", ANCHOR, ANCHOR + ROW)

# One citing sentence at the end of the opening paragraph of each corrected block.
CITES = [
    ("07", "### 7.6 Corpus cases the bank adds",
     " DR-422 locks the corrections of the check round of 2026-10-11: the intake is run again at profile activation, each deletion of an uploaded file is audited, and the mask keeps an Arabic letter inside the class of patch `P-01`."),
    ("10", "### 7.26 `capture_source` of a name",
     " DR-422 locks the correction of the check round of 2026-10-11: the label is compared as a mapped value, also when the hash of the landed row is equal (item 5)."),
    ("10", "### 7.27 `facility_instalment` and `facility.granted_on` in the writer",
     " DR-422 locks the corrections of the check round of 2026-10-11: the payment columns and the grant date are compared as mapped values, and the index on `(facility_id, seq)` is not unique."),
    ("13", "### 7.17 ",
     " DR-422 locks the corrections of the check round of 2026-10-11: the order of the activation, the refusal of a double add, the default end date, the form `tag.<name> = true` in a feature filter, and the recompute of months already computed."),
    ("14", "### 7.19 Known-answer cases the bank adds",
     " DR-422 locks the correction of the check round of 2026-10-11: the harness report carries one result per case (`results`, item 6)."),
    ("19", "### 7.21 ",
     " DR-422 locks the corrections of the check round of 2026-10-11: four sensitive sources, the PEP register as the first source of `party.pep_status`, and the purpose rule for the activity figures."),
    ("25", "### 7.23 Reads are exported to the SIEM",
     " DR-422 locks the correction of the check round of 2026-10-11 to item 6 (which events the off switch holds back)."),
    ("25", "### 7.24 Dead letters: resend and resolve",
     " DR-422 locks the correction of the check round of 2026-10-11 to the last CHECK of the table."),
    ("26", "### 7.13 ",
     " DR-422 locks the corrections of the check round of 2026-10-11: the plain index on `(facility_id, seq)`, `paid_amount` without `paid_on`, and what the grader does with the optional inputs."),
]
for nn, head, sentence in CITES:
    path = f"docs/plan/phase-{nn}.md"
    lines = f._load(path).split("\n")
    hits = [i for i, line in enumerate(lines) if line.startswith(head)]
    if len(hits) != 1:
        f.errors.append(f"DR-422 {path}: heading {head!r} found {len(hits)} times")
        continue
    para = lines[hits[0] + 1]
    if len(para) < 40 or para.startswith("#"):
        f.errors.append(f"DR-422 {path}: no opening paragraph under {head!r}")
        continue
    f.fix("DR-422", path, para, para + sentence)

f.done()
