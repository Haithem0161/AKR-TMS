# Gap pass 1 run 1: lead verdict (2026-10-10, applied)

Draft: tools/agent/gap1-run1.md (Haiku; 32 findings: 10 HIGH, 18 MEDIUM, 4 LOW; targets 00 14, 01 8, 02 10).
Read in full by the lead. Quality good; sources and checks spot-checked true so far (phase 00 l.553-556 outbox
reservation `notify` and `siem.export`; phase 22 uses `notify.request`; phase 00 l.584-598 KV layout and AppRoles;
key names by grep).

Decisions on the UNCLEAR items (to apply with the findings; new DR-356.. when applied):
1. KV mount (G1-00-05): one KV v2 mount `akr/` (phase 00 l.588). Every later path is relative to it: `kv/polis/directory`
   -> `polis/directory`, `kv/polis/keys` -> `polis/keys`, `kv/release/trust` -> `release/trust`, `kv/kratos/oidc/generic`
   -> `kratos/oidc/generic`, `secret/assistant/llama` -> `assistant/llama`, `secret/assistant/golden-hmac` ->
   `assistant/golden-hmac`; `ory/<x>` stays `ory/<x>` under the mount. Write it as one naming rule in phase 00 7.5 and
   correct the source phases (25, 28, 31) in their own 7.x (or in the run 4 apply). DR.
2. Key names (G1-00-06): `akr-sign-certificate` is the name (phases 00, 05, 12); phase 23's `akr-sign-cert` (l.498, 677,
   690, 917, 1022) is corrected to it. `akr-sign-bundle` (phase 25 l.947, 1391): STILL TO CHECK whether the bank
   installation signs with it or only verifies (vendor-side signing needs no bank transit key).
3. Pack download step-up (G1-01-08): `export` (phase 24, status). Phase 06 section 4.5 item 2 is corrected: no
   `pack_download` class. `readiness_declare`: STILL TO CHECK in phase 06 (what it gates; use an existing closed class,
   likely `approval`, or justify a new one).
4. Keyed-hash purpose `islamic.document_ref` (G1-02-10): the purposes belong to the phase 00 crypto port
   (`keyed_hash(purpose, data)`, phase 00 l.585; phase 01 added csrf, token, login_identifier). Phase 00 7.x gains the
   purpose list (collect every purpose named by phases 01-31 by grep "purpose" near keyed_hash/keyed hash); phase 26
   text corrected from "phase 02" to "phase 00".
5. Dormancy (G1-01-01): the expression IS stated in phase 03 (near l.848-852): `greatest(last_authenticated_at,
   enabled_at, created_at)`. Fill it in; drop the UNCLEAR.
6. `profile.read` (G1-02-09): STILL TO CHECK phase 13 l.934 and phase 15 l.1859 (access-event value vs Cedar action).
7. Feature tables (G1-02-04): STILL TO CHECK the three partitioned table names in phase 13 section 1.
8. Audit catalogue (G1-02-05): accept as written (rule: CATALOGUE is the union of each phase's audit table; each phase's
   audit section is normative). No copy of the kinds.
Accepted as written (pending the final read while applying): G1-00-01..04, 07..14; G1-01-02..07; G1-02-01..03, 06..08.
G1-02-08: also correct phase 24 section 4.5 item 1 "five kinds" (only `certification` is new in the CHECK).
Extra (not run 1): DATA-MODEL lacks `alert.model_scores` (phase 27) - add in the run 4 apply.

Apply method (next): tools/agent/apply-g1-run1.py: for each target phase, replace the "_None yet._" / "None yet." line
under "## 7+. Gap Additions" with the accepted 7.x blocks (header line "Gap pass 1 (2026-10-10), from
tools/agent/gap1-run1.md; lead-verified."), with the decisions above folded in. Then roadmap gap log row "1 (gap pass 1,
run 1: targets 00-02)" and status Gap Analysis Summary; _progress entry; then launch run 2 (targets 03, 04) with
tools/agent/p-g1-run1.txt adapted (targets, file name gap1-run2.md).

Final checks and changes made while applying (2026-10-10):
- `akr-sign-bundle`: signed at the bank by `akr-ctl support-bundle create` (phase 25 section 3.8 step 4): entered in the
  phase 00 key inventory under an operator policy.
- Decision 2 revised: phase 23's `akr-sign-cert` is a deliberately separate key (DR-194: own verifiers, rotation and
  custody), so it is renamed `akr-sign-deletion`, not merged into `akr-sign-certificate` (DR-357).
- `readiness_declare`: phase 01 lets each phase declare `StepUpAction { name, class }`; the declaration uses class
  `approval`, the pack download `export` (DR-359; phase 06 7.1, phase 08 7.1). Phase 01 gets no block for G1-01-08.
- `profile.read`: it is the Cedar action; the access_event CHECK holds verbs. No CHECK change; the profile read writes
  `view` on `customer` (DR-358; phase 02 7.9 rewritten; phase 13 7.1).
- Feature tables confirmed: `feat.feature_values`, `feat.segment_stats`, `feat.profiles`.
- G1-00-04 rewritten: `akr_monitor` is created by provisioning (phase 03, DR-22) and `akr_retention` by phase 23
  migration 0072; migration 0001 is unchanged (lead severity MEDIUM). G1-02-09 lead severity MEDIUM.
- G1-00-13: migration 0001 unchanged; phase 31 creates `train` itself.
- G1-02-08: phase 24's "five kinds" means the five builders, not CHECK values; no phase 24 correction.
- Keyed-hash purposes: phase 00 7.15 (rule plus the purposes named so far); phase 26 7.1 correction.
- Noticed for run 3 (target 14): phase 14 section 3 writes an access event `search`, not a CHECK value
  (`search_hit` exists).
- Timer table line numbers dropped (owning phases verified by grep).
