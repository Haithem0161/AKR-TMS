# Owner-decision check, round 2, key 19b (Haiku draft for lead review)

### P-19b-1  HIGH   (fix F5, [O-19-7])
- Problem: The default-deny note in section 3.13 gives CHK the `rfi_prefill_map.propose` permit, but section 7.21 item 9 gives CHK only `rfi_prefill_map.read` (only CFG holds propose). Read literally, the note and item 9 cannot both hold.
- Proof: docs/plan/phase-19.md:1594 "every action for KYC, CFG, DS, CHK, MRO, ADM, SEC (except the alarm reads above, and for CFG and CHK the `rfi_prefill_map.read` and `rfi_prefill_map.propose` permits of section 7.21)"; docs/plan/phase-19.md:2126-2128 "@id("followup.prefill_map.read.chk") @role("CHK") permit (principal, action == Akr::Action::"rfi_prefill_map.read", resource is Akr::System)"
- Smallest fix: In docs/plan/phase-19.md line 1594 replace "for CFG and CHK the `rfi_prefill_map.read` and `rfi_prefill_map.propose` permits of section 7.21" with "for CFG the `rfi_prefill_map.read` and `rfi_prefill_map.propose` permits and for CHK the `rfi_prefill_map.read` permit of section 7.21".

### P-19b-2  MEDIUM   (fix F10, [L-1])
- Problem: The owner's DELETE raises the guard message only when the table holds a row. The trigger is row-level and the migration inserts no pre-fill row, so on the table that `make migrate` leaves (where this check sits) the owner's DELETE affects zero rows and raises nothing. The UPDATE and unique-index checks name their rows ("a retired row", "a second active row"); the DELETE check does not.
- Proof: docs/plan/phase-19.md:2147 "`DELETE FROM casework.rfi_prefill_map` fails, as `akr_app` with permission denied (no DELETE grant) and as the owner `akr_migrate` with `pre-fill map rows are never deleted`"; docs/plan/phase-19.md:2022-2023 "BEFORE UPDATE OR DELETE ON casework.rfi_prefill_map" and "FOR EACH ROW EXECUTE FUNCTION casework.rfi_prefill_map_guard();"; the migration's only INSERTs are at docs/plan/phase-19.md:2028 and 2034, into `admin.column_classification` and `admin.retention_assignment`.
- Smallest fix: In docs/plan/phase-19.md line 2147 replace "and as the owner `akr_migrate` with `pre-fill map rows are never deleted`" with "and, once a row exists, as the owner `akr_migrate` with `pre-fill map rows are never deleted`".

### P-19b-3  MEDIUM   (fix F17, [O-19-18])
- Problem: The DESIGN-SYSTEM note on the D3 RFI pre-fill tab names CHK and M as its readers and leaves out CFG, the role that proposes the changes shown in that tab. Phase 19 section 7.21 item 11 says the tab renders for a D3 user who may call the GET, which is CFG, CHK and M, and the WHY of F17 in owner2-fixes-19b.txt gives that same final wording.
- Proof: docs/DESIGN-SYSTEM.md:849-850 ""Propose changes" (CFG) sets or removes rows through a change request. CHK and M read the tab; the RFI desk reads the map on W8."; docs/plan/phase-19.md:2145 "it renders for a D3 user who may call the GET, which is CFG, CHK and M."
- Smallest fix: In docs/DESIGN-SYSTEM.md line 850 replace "CHK and M read the tab" with "CFG, CHK and M read the tab".

## Summary
- Entries checked: 19 (F1 to F19).
- Findings by severity: HIGH 1, MEDIUM 2, LOW 0 (3 findings in total).
CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 P-19b-1, F6 ok, F7 ok, F8 ok, F9 ok, F10 P-19b-2, F11 ok, F12 ok, F13 ok, F14 ok, F15 ok, F16 ok, F17 P-19b-3, F18 ok, F19 ok
