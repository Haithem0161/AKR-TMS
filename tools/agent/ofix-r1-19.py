#!/usr/bin/env python3
"""Owner-decision round 1 fixes: phase 19 (verdicts O-19-1..20 and L-1 in tools/agent/owner-verdict.md), with the
pointer in phase 18 and the DR-417 row of research.md."""
import sys
sys.path.insert(0, "/home/haithem/Projects/AKR-TMS/tools/agent")
from ofix import Fixer

f = Fixer(round_no=1)
P = "docs/plan/phase-19.md"

# O-19-1: who owns the table
f.fix("O-19-1", P,
      "the guard, grants, classification and retention rows to `0065_casework_followup_guards_grants.sql`; owner `akr_app`):",
      "the guard, grants, classification and retention rows to `0065_casework_followup_guards_grants.sql`; owner `akr_migrate`, written by `akr_app`):")

# O-19-13: the module follows the crate rule of the phase
f.fix("O-19-13", P,
      "(`PrefillSource`, a closed enum of `domain::rfi::prefill`;",
      "(`PrefillSource`, a closed enum of `domain::casework::prefill`;")

# O-19-16: the reader methods the sources use
f.fix("O-19-16", P,
      "| `relationship.start_date` | `{ date }`: the earliest `onboarded_on` of the party's current `canon.customer` rows | `CanonPartyReader` (phase 18 section 3.14) |",
      "| `relationship.start_date` | `{ date }`: the earliest `onboarded_on` of the party's current `canon.customer` rows | `CanonPartyReader::current_customers` (the reader of phase 18; the method is added in item 4) |")
f.fix("O-19-16", P,
      "| `relationship.status` | `{ records: [{ status, cdd_level }] }`, one per current customer row; no CIF | `CanonPartyReader` |",
      "| `relationship.status` | `{ records: [{ status, cdd_level }] }`, one per current customer row; no CIF | `CanonPartyReader::current_customers` |")

# O-19-8: the PEP answer comes from the bank's register first
f.fix("O-19-8", P,
      "| `party.pep_status` | `{ answer }`: `yes` for `pep`, `no` for `not_pep`, `unknown` otherwise | `CanonPartyReader` | `yes_no_unknown`, `value` | no |",
      "| `party.pep_status` | `{ answer, former? }`. The bank's PEP register decides first (view `kyc.party_pep_state`, phase 15): `pep` gives `yes`; `former_pep` gives `yes` with `former: true`; `candidate` gives `unknown`, because an unconfirmed lead is never stated. With no row, or the state `none`, the customer file decides (`canon.party.pep_status`): `pep` gives `yes`, `not_pep` gives `no`, anything else `unknown` | `PartyPepReader` (item 4), then `CanonPartyReader::current_party` | `yes_no_unknown`, `value` | no |")

# O-19-16, O-19-19, O-19-20: the signatory source is sensitive, has no flag, and names the legal name
f.fix("O-19-20", P,
      "| `accounts.signatories` | `{ parties: [{ role, name }], incomplete }`: the other parties with a current `joint` or `signatory` role on the accounts of `accounts.open`, each with its current primary name; a party the caller may not see under phase 18 section 3.10 is left out and `incomplete` is `true` | `CanonPartyReader` | `value`, `free_text` | no |",
      "| `accounts.signatories` | `{ parties: [{ role, name }] }`: the other parties with a current `joint` or `signatory` role on the accounts of `accounts.open`, each with its current `legal` name (`name_type = 'legal'`, `superseded_at IS NULL`; the Arabic or Kurdish form first). A party the caller may not see under phase 18 section 3.10 is left out, with no count and no flag | `CanonPartyReader::co_holders` (item 4) and `current_names` | `value`, `free_text` | yes (the names of other persons) |")

# O-19-17: the profile source is sensitive and goes through the phase 13 read rules
f.fix("O-19-17", P,
      "| `ProfileQuery` of phase 13 with its `PartyProfilesView` projection (`actual`, `declared` and `status`; never a deviation, an own-history or a peer value) | `value`, `free_text` | no |",
      "| `ProfileQuery` of phase 13 with its `PartyProfilesView` projection (`actual`, `declared` and `status`; never a deviation, an own-history or a peer value), called with the caller's context and purpose code, so the Cedar check `profile.read`, the purpose rule and the access events of phase 13 section 3.6 apply as on the party page | `value`, `free_text` | yes (the profile is restricted data that the desk may not read) |")
f.fix("O-19-14", P,
      "| `value`, `free_text` | yes (the bank's own view) |",
      "| `value`, `free_text` | yes (the bank's own risk label) |")
f.fix("O-19-17", P,
      "   No source reads an alert, a case, a decision, a report, a watch,",
      "   Four of the ten sources are sensitive: `accounts.closed`, `accounts.signatories`, `activity.declared_and_actual` and `rating.cbi_level`. A sensitive source is computed for a caller with role M only (item 5 step 5). No source reads an alert, a case, a decision, a report, a watch,")
f.fix("O-19-14", P,
      "linked documents and the bank's own view (question 22 of the Wolfsberg list).",
      "linked documents and the respondent's comfort with the relationship (question 22 of the Wolfsberg list).")

# O-19-3, O-19-5, O-19-9: the types
f.fix("O-19-3", P,
      "DTOs: `PrefillMapChange { set: Vec<PrefillMapEntry>, remove: Vec<String> }`,",
      "DTOs: `PrefillMapChange { set: Vec<PrefillMapEntry>, remove: Vec<String>, reason_code: String, reason_text: String }` (the two reason fields are the ones phase 04 `create` requires),")
f.fix("O-19-5", P,
      "`PrefillMapChange`, `ChangeRequestAccepted` (phase 04, reused), and `RfiPrefillView",
      "`PrefillMapChange`, the phase 04 `ChangeRequestView` (the answer of the POST), and `RfiPrefillView")
f.fix("O-19-17", P,
      "`withheld` (a sensitive source and the caller is not the MLRO) or `format_mismatch`.",
      "`withheld` (a sensitive source and the caller is not the MLRO, or phase 13 refused the profile read) or `format_mismatch`.")
f.fix("O-19-9", P,
      "    async fn insert(&self, tx: &mut Tx, row: &RfiPrefillMapRow) -> Result<(), RepoError>;",
      "    async fn insert(&self, tx: &mut Tx, row: &RfiPrefillMapRow) -> Result<RfiPrefillMapRow, RepoError>;   // RETURNING")
f.fix("O-19-16", P,
      "   AppState: `api` gains `rfi_prefill: Arc<RfiPrefillService>` and one more change target.",
      "   AppState: `api` gains `rfi_prefill: Arc<RfiPrefillService>` and one more change target.\n\n   Readers added for the registry. `CanonPartyReader` (phase 18 section 2) gains two methods: `current_customers(conn, party_id) -> Result<Vec<CustomerRow>, RepoError>`, the current `canon.customer` rows of the party (`recorded_to IS NULL`) with `onboarded_on`, `status` and `cdd_level`; and `co_holders(conn, account_ids) -> Result<Vec<CoHolder>, RepoError>`, the party id and role of every current `joint` or `signatory` row of `canon.account_party_role` on those accounts (`recorded_to IS NULL`). A new port `PartyPepReader` (domain layer; `PgPartyPepReader` in `db`) has one method, `pep_state(conn, party_id) -> Result<Option<String>, RepoError>`: the `pep_state` of the party's row in the view `kyc.party_pep_state`, on which phase 15 grants SELECT to `akr_app`. This phase reads the view and never writes the register.")

# O-19-10: the purpose rule is a rule of this route
f.fix("O-19-10", P,
      "   3. `X-Access-Purpose` holds a purpose code (422 `E_PURPOSE_REQUIRED` otherwise, phase 02).",
      "   3. `X-Access-Purpose` holds a purpose code (422 `E_PURPOSE_REQUIRED` otherwise). This is a rule of this route at every tier, as the profile read of phase 13 has one; the phase 02 rule alone covers restricted and suspicion data only.")
f.fix("O-19-10", P,
      "| `PurposeRequired` | `E_PURPOSE_REQUIRED` | 422 | a kinship read or a suspicion item read without `X-Access-Purpose` |",
      "| `PurposeRequired` | `E_PURPOSE_REQUIRED` | 422 | a kinship read, a suspicion item read or a pre-fill read (section 7.21) without `X-Access-Purpose` |")

# O-19-17: what a refusal of the profile read gives
f.fix("O-19-17", P,
      "otherwise compute the source (item 2), with `outcome = empty` when it returns nothing and `proposed` otherwise.",
      "otherwise compute the source (item 2), with `outcome = empty` when it returns nothing and `proposed` otherwise. For `activity.declared_and_actual` a refusal by phase 13 (for the MLRO that is a purpose other than `investigation`) also gives `withheld`, and the other proposals of the response are not affected.")
f.fix("O-19-17", P,
      "through the phase 02 access layer. No audit row: the read changes nothing.",
      "through the phase 02 access layer; the profile read of `activity.declared_and_actual` writes its own access events in phase 13, one per customer record. No audit row: the read changes nothing.")

# O-19-4: the creation step
f.fix("O-19-4", P,
      "   5. Create the change request through phase 04 `ChangeRequestService` (kind `rfi_prefill_map`, `object_key` `rfi_prefill_map`, reason family `change`), route `cr_material`, volume effect `neutral`.",
      "   5. Create the change request with phase 04 `ChangeRequestService::create` (origin Dedicated, so `submit` runs in the same transaction; kind `rfi_prefill_map`, `object_key` `rfi_prefill_map`, the caller's `reason_code` of family `change` and `reason_text`), route `cr_material`, volume effect `neutral`.")
f.fix("O-19-5", P,
      "   6. Respond 201 with the change request; `change_request.created` is written by phase 04.\n\n7. **Change target `rfi_prefill_map`**",
      "   6. Respond 201 with the `ChangeRequestView`; `change_request.created` is written by phase 04.\n\n7. **Change target `rfi_prefill_map`**")

# O-19-6: the six sub-codes join the closed list
f.fix("O-19-6", P,
      "`forbidden_term`, `rfi_open`, `not_critical_finding`.",
      "`forbidden_term`, `rfi_open`, `not_critical_finding`; and the six of section 7.21: `prefill_not_applicable`, `prefill_off`, `source_unknown`, `question_unknown`, `format_mismatch`, `not_mapped`.")

# O-19-12: the audit row is in the table
f.fix("O-19-12", P,
      "| `clock.warning` (outbox, stage 50) | breach | owner | no |\n",
      "| `clock.warning` (outbox, stage 50) | breach | owner | no |\n| `rfi_prefill_map.applied` (section 7.21) | a pre-fill map change applied | rfi_prefill_map | no |\n")

# O-19-7: the default-deny note names the exception
f.fix("O-19-7", P,
      "every action for KYC, CFG, DS, CHK, MRO, ADM, SEC (except the alarm reads above),",
      "every action for KYC, CFG, DS, CHK, MRO, ADM, SEC (except the alarm reads above, and for CFG and CHK the `rfi_prefill_map.read` and `rfi_prefill_map.propose` permits of section 7.21),")

# O-19-15: the heading
f.fix("O-19-15", P,
      "correspondents P2 for the desk, but for the panel Proposed answers of section 7.21)",
      "correspondents P2 for the desk, but P1 for the panel Proposed answers of section 7.21)")

# O-19-17, O-19-20: the tipping-off review counts four sensitive sources
f.fix("O-19-20", P,
      "ten facts of the customer file and the account list, none of which is derived from monitoring or from a case. The two facts that could let a reader infer more, a closed account and the bank's risk label, are computed for the MLRO only; the desk sees that a proposal exists and that it is withheld, nothing else.",
      "ten facts of the customer file, the account list and the KYC records (the PEP register, the rating label, the monthly figures), none of which is derived from monitoring or from a case. Four facts are computed for the MLRO only: a closed account and the bank's risk label, which could let a reader infer more; the names of the other holders of an account, which are other persons' data; and the declared and actual activity, which is restricted data that the desk may not read. The desk sees that a proposal exists and that it is withheld, nothing else. A party the MLRO may not see is left out of the list of holders with no count and no flag (section 3.14), so the list never says that someone is hidden. A PEP lead that is not confirmed is answered `unknown`, never `yes`.")

# O-19-17, O-19-18: the frontend lines
f.fix("O-19-17", P,
      "a withheld proposal says \"Needs the MLRO\"; nothing is recorded until the person submits the form.",
      "a withheld proposal says \"Needs the MLRO\" to the desk, and to the MLRO, for whom only the activity figures can be withheld, \"Choose the purpose Investigation to see this\"; a former PEP is rendered as such (`former`); nothing is recorded until the person submits the form.")
f.fix("O-19-18", P,
      "(rows to set, rows to remove; the server's answer per row; `Idempotency-Key`; 201 and a link to G10). CHK, M and RFI read the tab.",
      "(rows to set, rows to remove, a reason code and a reason text; the server's answer per row; `Idempotency-Key`; 201 and a link to G10). The tab has no role list of its own (phase 09 section 7.5): it renders for a D3 user who may call the GET, which is CFG, CHK and M. D3 is not a screen of the RFI desk; the desk reads the map through the same GET on W8, where the panel names the source of each proposal.")

# L-1 (O-19-2): the DELETE never reaches the guard as akr_app
f.fix("L-1", P,
      "As `akr_app`: `DELETE FROM casework.rfi_prefill_map` fails with `pre-fill map rows are never deleted`; an `UPDATE` of a retired row fails with `pre-fill map row is retired`;",
      "`DELETE FROM casework.rfi_prefill_map` fails, as `akr_app` with permission denied (no DELETE grant) and as the owner `akr_migrate` with `pre-fill map rows are never deleted`. As `akr_app`: an `UPDATE` of the `state` of a retired row fails with `pre-fill map row is retired`;")

# O-19-8, O-19-17, O-19-20: the tests
f.fix("O-19-8", P,
      "`party.pep_status` gives `yes`, `no` and `unknown` for the three stored values; `accounts.signatories` leaves out a party the caller may not see and sets `incomplete`; a sensitive source is not computed for a caller without role M (the reader is not called).",
      "`party.pep_status` gives `yes` for a register state `pep`, `yes` with `former: true` for `former_pep` and `unknown` for `candidate`, also when the customer file says `not_pep`; with no register row it gives `yes`, `no` and `unknown` for the three customer-file values; `accounts.signatories` leaves out a party the caller may not see and the response holds no count and no flag that tells it; `PrefillSource::sensitive()` is true for exactly four keys; a sensitive source is not computed for a caller without role M (the reader is not called); as M with a purpose other than `investigation` the activity proposal is `withheld` and the other proposals are returned.")

# O-19-3, O-19-11: the curl body
f.fix("O-19-11", P,
      "as CFG `POST /api/v1/rfi-prefill-map` with `{\"set\":[{\"question_key\":\"account.opened_on\",\"source_key\":\"relationship.start_date\"}]}` and `Idempotency-Key: pfm-0001` gives 201",
      "as CFG `POST /api/v1/rfi-prefill-map` with `{\"set\":[{\"question_key\":\"<key>\",\"source_key\":\"relationship.start_date\"}],\"reason_code\":\"<a code of family change>\",\"reason_text\":\"...\"}`, where `<key>` is a question key of the correspondent template of the suite's fixture pack, and `Idempotency-Key: pfm-0001` gives 201")
f.fix("O-19-3", P,
      "with `source_key: \"case.status\"` gives 422 with `source_unknown`;",
      "the same body without `reason_code` gives 422 `E_VALIDATION`; with `source_key: \"case.status\"` gives 422 with `source_unknown`;")

# The DR-417 paragraph of "## Decisions" and the research.md row say four
f.fix("O-19-17", P,
      "the two sources that could reveal more (a closed account, the bank's risk label) are computed for the MLRO only; and keeping the write",
      "four sources are computed for the MLRO only (a closed account and the bank's risk label, which could reveal more; the names of the other holders of an account; the declared and actual activity, which is restricted data); the PEP answer reads the bank's PEP register before the customer file; and keeping the write")
f.fix("O-19-17", "docs/plan/research.md",
      "the two sources that could reveal more (a closed account, the bank's risk label) are computed for the MLRO only; keeping the write",
      "four sources are computed for the MLRO only (a closed account and the bank's risk label, which could reveal more; the names of the other holders of an account; the declared and actual activity, which is restricted data); the PEP answer reads the bank's PEP register before the customer file; keeping the write")

# O-19-16: the pointer in phase 18
f.fix("O-19-16", "docs/plan/phase-18.md",
      "`current_accounts(conn, party_id)` (joins `canon.account_party_role` where `recorded_to IS NULL`), `transactions(conn, account_ids, filter, cursor)`.",
      "`current_accounts(conn, party_id)` (joins `canon.account_party_role` where `recorded_to IS NULL`), `transactions(conn, account_ids, filter, cursor)`. Phase 19 section 7.21 item 4 adds `current_customers(conn, party_id)` and `co_holders(conn, account_ids)` for the RFI pre-fill.")

f.done()
