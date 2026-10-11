# Owner-decision round, phase 19 (Haiku draft for lead review)

### O-19-1  MEDIUM   (section 7.21 item 1, check 4: owner and roles)
- Problem: The block names `akr_app` as the owner of the new table, but the platform rule makes `akr_migrate` the owner and bars application roles from owning tables. The migration runs as `akr_migrate` and phase 19 states that owner for its own migration, so the table ends up owned by `akr_migrate` whatever the label says.
- Proof: docs/plan/phase-19.md:1982 "owner `akr_app`):" and docs/plan/phase-00.md:180-181 "The table owner (`akr_migrate`) bypasses RLS; no application role is the owner." and docs/plan/phase-19.md:41 "Owner role: akr_migrate."
- Smallest fix: In the item 1 heading of section 7.21, replace "owner `akr_app`" with "owner `akr_migrate`".

### O-19-2  HIGH   (section 7.21 item 1 and item 11, checks 4 and 6: the DELETE check)
- Problem: Under the roles the block sets, the stated DELETE check cannot produce the trigger message: `akr_app` has no DELETE grant, and PostgreSQL checks table privileges before any row trigger runs, so the statement fails with "permission denied for table rfi_prefill_map". The guard message appears only for the owner, which is not `akr_app` (see O-19-1).
- Proof: docs/plan/phase-19.md:2144 "As `akr_app`: `DELETE FROM casework.rfi_prefill_map` fails with `pre-fill map rows are never deleted`" and docs/plan/phase-19.md:2024 "GRANT SELECT, INSERT ON casework.rfi_prefill_map TO akr_app;" and docs/plan/phase-19.md:2013 "RAISE EXCEPTION 'pre-fill map rows are never deleted'".
- Smallest fix: In the item 11 check, say that as `akr_app` the DELETE fails with "permission denied for table casework.rfi_prefill_map", that the guard message is seen when the same DELETE runs as `akr_migrate`, the owner, and that the UPDATE check sets only a granted column (`state`, `retired_at` or `retired_by_request`), since any other column is refused by the grant before the trigger runs.

### O-19-3  HIGH   (section 7.21 item 6 step 5 and item 11 curl check: the change request has no reason)
- Problem: `PrefillMapChange` has no `reason_code` or `reason_text`, and phase 04's `create` requires both and refuses an inactive code or one outside the target's family. Item 6 names only the family `change`, so the curl check that expects 201 for the body shown would get 422, and the developer must guess the code and the text.
- Proof: docs/plan/phase-19.md:2067 "DTOs: `PrefillMapChange { set: Vec<PrefillMapEntry>, remove: Vec<String> }`" and docs/plan/phase-19.md:2098 "(kind `rfi_prefill_map`, `object_key` `rfi_prefill_map`, reason family `change`)" and docs/plan/phase-04.md:748 "`CreateChangeRequest` | `object_kind`, `body` (the target's proposal), `reason_code`, `reason_text`" and docs/plan/phase-04.md:873 "reason code active, of a family the target accepts".
- Smallest fix: Add `reason_code` and `reason_text` to the body of `POST /rfi-prefill-map` (or name one fixed reason code of the `change` family in item 6 step 5) and say so in the curl check.

### O-19-4  HIGH   (section 7.21 item 6 steps 5 and 6, item 11 curl check: the change is never submitted)
- Problem: Step 5 creates the change request without `submit`, so it stays `draft` (phase 04 defaults `submit` to false), and approvals run only on a `submitted` request. The curl check "As CHK and M approve" cannot pass, and the map is never applied.
- Proof: docs/plan/phase-19.md:2098 "5. Create the change request through phase 04 `ChangeRequestService`" and docs/plan/phase-04.md:748 "`submit?` (default false)" and docs/plan/phase-04.md:821 "`draft` -> `submitted` -> `approved` -> `effective`" and docs/plan/phase-19.md:2146 "As CHK and M approve."
- Smallest fix: In item 6 step 5, say "with `origin = Dedicated` and `submit: true` (phase 04 section 3.2 items 1 and 6; this target has no generic create), so the request is `submitted` with its route in the same transaction", and keep 201 as the answer.

### O-19-5  LOW   (section 7.21 item 4, the type name of the change-request response)
- Problem: `ChangeRequestAccepted` is said to come from phase 04, which does not define it; phase 13 says it is "reused from phase 04 where it exists". The response the route returns is the phase 04 view `ChangeRequestView`.
- Proof: docs/plan/phase-19.md:2067 "`ChangeRequestAccepted` (phase 04, reused)" and docs/plan/phase-04.md:749 "`ChangeRequestView` | All columns except `proposed` and `impact`" (no `ChangeRequestAccepted` occurs in docs/plan/phase-04.md).
- Smallest fix: Replace "`ChangeRequestAccepted` (phase 04, reused)" in item 4 with "`ChangeRequestView` (phase 04)", the body of the 201 response.

### O-19-6  MEDIUM   (section 7.21 items 5, 6 and 11, check 2: the closed sub-code list of section 3.11)
- Problem: Section 3.11 says the sub-codes of this phase are a closed list. Six sub-codes used by the new text are not in that list, and no block adds them: `prefill_not_applicable`, `prefill_off`, `source_unknown`, `question_unknown`, `format_mismatch` and `not_mapped`.
- Proof: docs/plan/phase-19.md:1339 "The sub-codes of this phase (closed list, riding in `errors[].code` or in `sub`;" and docs/plan/phase-19.md:2084 "sub-code `prefill_not_applicable`" and docs/plan/phase-19.md:2096 "(sub-code `not_mapped`)".
- Smallest fix: Add `prefill_not_applicable`, `prefill_off`, `source_unknown`, `question_unknown`, `format_mismatch` and `not_mapped` to the list at line 1339.

### O-19-7  MEDIUM   (section 3.13 notes, check 2: a second line still states the old rule)
- Problem: The notes under the Cedar file say that every action of CFG and CHK is denied by default. Section 7.21 now permits CFG and CHK to read the map and CFG to propose a change, and the block does not say that the note is replaced.
- Proof: docs/plan/phase-19.md:1593 "every action for KYC, CFG, DS, CHK, MRO, ADM, SEC (except the alarm reads above)" and docs/plan/phase-19.md:2119 "@id(\"followup.prefill_map.read.cfg\") @role(\"CFG\")".
- Smallest fix: In the note at line 1593, add "(except `rfi_prefill_map.read` and `rfi_prefill_map.propose` of section 7.21)" after "SEC" and say which of CFG and CHK have the read.

### O-19-8  HIGH   (section 7.21 item 2, the source `party.pep_status`; checks 2 and 3, and the tipping-off claim of item 10)
- Problem: The source reads `canon.party.pep_status` through `CanonPartyReader`, but phase 15 says that column is never the product's PEP state, which is the view `kyc.party_pep_state` with four values, so the three-value mapping can answer `unknown` for a party the bank has confirmed as a PEP. Phase 18 section 3.14 item 3 reads the same column for the party page, so phases 15 and 18 disagree and the block follows phase 18 without saying so; the view is also fed by screening alerts (`pep_list_sweep`), which weakens item 10's claim.
- Proof: docs/plan/phase-19.md:2048 "`party.pep_status` | `{ answer }`: `yes` for `pep`, `no` for `not_pep`, `unknown` otherwise | `CanonPartyReader`" and docs/plan/phase-15.md:680 "-- stored status: canon.party.pep_status (phase 10) is CBS-derived and is never written by this phase, because the" and docs/plan/phase-15.md:1529 "and no tier (DR-38: a CBS PEP flag is read only when the bank declares it maintained; `canon.party.pep_status` is not" and docs/plan/phase-15.md:2276 "(the PEP state is the view `kyc.party_pep_state`)." and docs/plan/phase-18.md:871 "the current `canon.party` row (`recorded_to IS NULL`) for type, status, nationality, residence country and `pep_status`" and docs/plan/phase-15.md:1520 "`pep_list_sweep` (job, every ten minutes, section 4.1): reads `alert.screening_alerts`".
- Smallest fix: Read `party.pep_status` through the phase 15 view `kyc.party_pep_state` (map `pep` to `yes`, `former_pep` and `candidate` to `unknown` or a stated value, `none` to `no`), and re-check the claim of item 10 for this source.

### O-19-9  LOW   (section 7.21 item 4, the repository trait: check 4, method signatures)
- Problem: The phase states that every repository insert uses `RETURNING`, and that its methods return the row. The new `insert` returns nothing, so a developer cannot tell whether to return the stored row.
- Proof: docs/plan/phase-19.md:995 "each method takes `&mut PgConnection` or `&mut Tx` as its name says; `RETURNING` on every insert" and docs/plan/phase-19.md:2074 "async fn insert(&self, tx: &mut Tx, row: &RfiPrefillMapRow) -> Result<(), RepoError>;".
- Smallest fix: Change the return type of `insert` in item 4 to `Result<RfiPrefillMapRow, RepoError>` and say that it uses `RETURNING`.

### O-19-10  MEDIUM   (section 7.21 item 5 step 3 and item 11 curl check: the purpose rule by tier)
- Problem: Item 3 and step 3 require a purpose code on every call and cite phase 02, whose access rule requires one only for restricted and suspicion records, so a developer who follows the pointer would skip the check for a normal-tier party and fail the curl check "without X-Access-Purpose, 422". The block should state the rule as a route rule at every tier, as phase 13's step 3 does for its profile read.
- Proof: docs/plan/phase-02.md:722 "Tier `restricted` or `suspicion`: `X-Access-Purpose` must hold a purpose code, else 422 `E_PURPOSE_REQUIRED`" and docs/plan/phase-19.md:2085 "`X-Access-Purpose` holds a purpose code (422 `E_PURPOSE_REQUIRED` otherwise, phase 02)." and docs/plan/phase-13.md:756 "3. `X-Access-Purpose` is required (`E_PURPOSE_REQUIRED`, 422)".
- Smallest fix: In step 3 of item 5, say "a purpose code is required at every tier (a route rule; 422 `E_PURPOSE_REQUIRED` otherwise)", and cite phase 02 only for the access event of a restricted or suspicion record.

### O-19-11  MEDIUM   (section 7.21 item 6 step 2 and item 11 curl check: a question key the plan does not define)
- Problem: The curl check maps the question key `account.opened_on`, which occurs nowhere else in docs/plan. Step 2 needs it to be a question of a correspondent template in the active pack, but the plan lists no correspondent question keys or answer formats, so the expected 201 cannot be checked.
- Proof: docs/plan/phase-19.md:2095 "`question_key` is a question of an RFI template of kind `correspondent` in the active policy pack (phase 09; sub-code `question_unknown`)" and docs/plan/phase-19.md:2146 "`{\"set\":[{\"question_key\":\"account.opened_on\",\"source_key\":\"relationship.start_date\"}]}`".
- Smallest fix: Name in phase 09's pack seed the question keys (with `answer_format`) of the seed correspondent template, and use one of those keys in the curl example.

### O-19-12  LOW   (section 7.21 item 8, the audit table of section 3.12)
- Problem: Item 8 says `rfi_prefill_map.applied` is "one more row of section 3.12", but the table of section 3.12 has no such row.
- Proof: docs/plan/phase-19.md:2103 "8. **Audit event** (one more row of section 3.12): `rfi_prefill_map.applied`" and docs/plan/phase-19.md:1353 "| `alarm.raised`, `alarm.notified`, `alarm.acknowledged`, `alarm.closed` | alarms | alarm | no |".
- Smallest fix: Add the row "| `rfi_prefill_map.applied` | map change applied | rfi_prefill_map | no |" to the table of section 3.12 after line 1353.

### O-19-13  LOW   (section 7.21 item 2, the module path of the source registry)
- Problem: The registry is placed in `domain::rfi::prefill`, but the crate rule of this phase puts the domain in `domain::casework`.
- Proof: docs/plan/phase-19.md:2040 "a closed enum of `domain::rfi::prefill`" and docs/plan/phase-19.md:958 "the domain in `domain::casework` (akr-core, no IO".
- Smallest fix: Change `domain::rfi::prefill` in item 2 to `domain::casework::prefill`, the rule of line 958.

### O-19-14  LOW   (section 7.21 item 2, the phrase "the bank's own view" used for two facts)
- Problem: The phrase "the bank's own view" names the sensitive registry source `rating.cbi_level` and also a fact outside the registry (question 22 of the Wolfsberg list, the respondent's comfort with the relationship and its rationale). A reader cannot tell which fact the sensitive rule covers.
- Proof: docs/plan/phase-19.md:2053 "yes (the bank's own view)" and docs/plan/phase-19.md:2055 "and the bank's own view (question 22 of the Wolfsberg list)".
- Smallest fix: In the sentence at line 2055, replace "the bank's own view (question 22 of the Wolfsberg list)" with "the respondent's comfort and rationale (question 22 of the Wolfsberg list)".

### O-19-15  LOW   (section 5.2 heading, the priority word)
- Problem: The heading says "correspondents P2 for the desk, but for the panel Proposed answers of section 7.21". The priority of the panel is missing, so the sentence reads as cut off. Item 11 gives it as P1.
- Proof: docs/plan/phase-19.md:1744 "correspondents P2 for the desk, but for the panel Proposed answers of section 7.21" and docs/plan/phase-19.md:2141 "This panel is P1 by the owner decision".
- Smallest fix: In the heading of section 5.2, change "but for the panel" to "but P1 for the panel".

### O-19-16  MEDIUM   (section 7.21 item 2, the sources `relationship.start_date`, `relationship.status` and `accounts.signatories`: check 3, reader methods not named)
- Problem: The block reads three facts "through `CanonPartyReader`", but that reader has no method for the `canon.customer` rows or for the other holders of an account. Its listed methods read the party row, the names, the party's own accounts and the transactions, and the block introduces no new method.
- Proof: docs/plan/phase-18.md:582 "`CanonPartyReader` (read only, SELECT on canon, phase 10): `current_party(conn, id)`, `current_names(conn, id)`, `current_accounts(conn, party_id)`" and docs/plan/phase-18.md:880 "3. Accounts: the party's current account rows with role `owner`, `joint` or `signatory`" and docs/plan/phase-19.md:2044 "`relationship.start_date` | `{ date }`: the earliest `onboarded_on` of the party's current `canon.customer` rows | `CanonPartyReader` (phase 18 section 3.14)" and docs/plan/phase-19.md:2051 "`accounts.signatories` | `{ parties: [{ role, name }], incomplete }`".
- Smallest fix: Add to item 2 the two reader methods the sources need (for example `current_customers(conn, party_id)` on `canon.customer` with `recorded_to IS NULL`, and `current_co_holders(conn, account_ids)` on `canon.account_party_role` with roles `joint` and `signatory`), and list them in the reader list of phase 18 section 2.

### O-19-17  HIGH   (section 7.21 item 2, the source `activity.declared_and_actual`: a read with no grant, checks 3 and 6)
- Problem: The source is read through `ProfileQuery` of phase 13, and under phase 13's read rules the RFI desk is refused: it holds no `profile.read` (A1 and A2 within branch scope, KYC and M) and none of the profile purposes. The block does not say whether the pre-fill runs the read under the caller's Cedar check, which denies the desk, or bypasses it, which phase 13 does not allow.
- Proof: docs/plan/phase-13.md:841 "`profile.read` | Customer | A1 and A2 within branch scope; KYC; M" and docs/plan/phase-13.md:754 "2. Cedar `profile.read` on each `Customer` (section 3.11). A deny is a 403 `E_FORBIDDEN`" and docs/plan/phase-13.md:756 "3. `X-Access-Purpose` is required (`E_PURPOSE_REQUIRED`, 422): `assigned_work` for A1 and A2, `kyc_review` for KYC, `investigation` for M." and docs/plan/phase-19.md:2052 "`ProfileQuery` of phase 13 with its `PartyProfilesView` projection".
- Smallest fix: In item 2, say how the source is authorised: either add `profile.read` and a pre-fill purpose for role RFI in phase 13 section 3.11 and its purpose list, or state that the source reads the `PartyProfilesView` projection of `ProfileQuery` without the route's Cedar step and name the purpose the pre-fill records for it.

### O-19-18  MEDIUM   (section 7.21 item 11, the D3 tab "RFI pre-fill": the roles that read it)
- Problem: The block says CHK, M and RFI read the new D3 tab, but phase 09 gives the studio to CFG and CHK and reading to M, CH and IA, names no RFI role, and says the rail hides a tab from a caller the server would refuse. The two rules cannot both hold for the RFI desk.
- Proof: docs/plan/phase-19.md:2142 "CHK, M and RFI read the tab." and docs/plan/phase-09.md:1141 "phase 08. Its tabs follow the rail rules of DESIGN-SYSTEM 8.2 (CFG and CHK see the studio; M, CH and IA read it;" and docs/plan/phase-09.md:1143 "the rail and the route guard hide a tab the caller may not open".
- Smallest fix: In item 11, either remove RFI from "CHK, M and RFI read the tab" (the desk reads the map through `GET /rfi-prefill-map` on W8 instead, which the API allows) or add the tab's own rail rule for RFI to phase 09 section 5.

### O-19-19  LOW   (section 7.21 item 2, the source `accounts.signatories`: the name it returns)
- Problem: The source returns "its current primary name" for each other party, but no column of the party name table marks a primary name, and phase 18 selects the current names by script (`ar` first). A developer has to choose the name, and the choice changes what another bank receives.
- Proof: docs/plan/phase-19.md:2051 "each with its current primary name" and docs/plan/phase-10.md:504 "name_type TEXT NOT NULL CHECK (name_type IN ('legal', 'trade', 'alias', 'as_in_message', 'former'))" and docs/plan/phase-18.md:871 "the current `canon.party_name` rows (`superseded_at IS NULL`), `script = 'ar'` first".
- Smallest fix: Replace "its current primary name" with the rule the block means, for example "its current `legal` name (`name_type = 'legal'`, `superseded_at IS NULL`), the `ar` form first".

### O-19-20  HIGH   (section 7.21 item 2, the source `accounts.signatories`: the `incomplete` flag and the tipping-off review of item 10)
- Problem: The `incomplete` flag is `true` when a co-holder is left out because the caller may not see it, so the desk and, through the answer, another bank learn that a hidden party holds the account. The phase's rule for hidden neighbours is "no count and no placeholder", and item 10's tipping-off review does not mention this flag, so the rule and the output cannot both hold.
- Proof: docs/plan/phase-19.md:2051 "a party the caller may not see under phase 18 section 3.10 is left out and `incomplete` is `true`" and docs/plan/phase-19.md:1613 "an invisible neighbour is omitted with no count and no placeholder" and docs/plan/phase-19.md:1598 "### 3.14 Tipping-off review (M-43, M-44, M-68; the answer must be \"nothing\")".
- Smallest fix: Drop the `incomplete` field from `accounts.signatories` (leave the hidden parties out with no flag, as the graph read does), and add to item 10 that the signatories fact shows only the parties the caller may see.


## Summary

Blocks and base places checked (line numbers are those of docs/plan/phase-19.md): the new block 7.21 (items 1 to 11, lines 1979 to 2148); the owner-decision paragraph before it (line 1977); the 7.15 replacement (line 1951); and the DR-417 paragraph in Decisions (lines 2173 and 2174). Changed base places: the 10 hunks that remove base text (the header Complexity at line 7, the M-44 line at line 10, the route IDs at line 13, the section 1 intro at line 22, the section 2 count at line 1053, section 4.5 items 3 and 17 at lines 1701 and 1728, the 5.2 heading at line 1744, section 6 item 7 at line 1831 and the 7.15 text at line 1951), and the two added paragraphs. The route table counts (35 distinct routes: 32 in section 2 plus 3 in 7.21, matching the 19 owned and 16 added API.md rows), the 15 tables, the nine classification rows and the retention row, and the ten source keys in the SQL, the registry and the table were checked and agree. The two sensitive sources (`accounts.closed`, `rating.cbi_level`) are treated the same way in items 2, 5, 6, 10 and 11 and the Decisions paragraph: no finding.

Findings by severity: HIGH 6 (O-19-2, O-19-3, O-19-4, O-19-8, O-19-17, O-19-20), MEDIUM 7 (O-19-1, O-19-6, O-19-7, O-19-10, O-19-11, O-19-16, O-19-18), LOW 7 (O-19-5, O-19-9, O-19-12, O-19-13, O-19-14, O-19-15, O-19-19). Total 20.

Per block:
- 7.21 checks 1 to 6: finding ids O-19-1 to O-19-14 and O-19-16 to O-19-20. Items 7 (change target) and 9 (Cedar permits, including the scope gates against API.md section 2) have no finding of their own; O-19-7 is a base note that the item 9 permits contradict.
- Owner-decision paragraph (line 1977): CHECKED, no finding.
- 7.15 replacement (line 1951): CHECKED, no finding.
- DR-417 paragraph (lines 2173 and 2174): CHECKED, no finding.
- Base place, 5.2 heading (line 1744): finding O-19-15.
- Other base places (lines 7, 10, 13, 22, 1053, 1701, 1728 and 1831): CHECKED, no finding.

FINDINGS: 20 (6 HIGH, 7 MEDIUM, 7 LOW).
