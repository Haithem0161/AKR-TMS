# Gap pass 1 run 2: lead verdict (2026-10-10, applied)

Draft: tools/agent/gap1-run2.md (Haiku; 24 findings: 2 HIGH, 20 MEDIUM, 2 LOW; targets 04 11, 03 13). Read in full;
every source line and target check re-read by the lead. Applied by tools/agent/apply-g1-run2.py (helper g1lib.py).
Lead severity: 1 HIGH (G1-P04-02), 21 MEDIUM, 2 LOW (G1-P04-01 lowered: phase 18 states the extension itself).

Decisions on the UNCLEAR items:
- G1-P04-02 evidence route: `POST /change-requests/{id}/evidence`, maker only, while `draft` or `submitted` with no
  decision in the current pass (dedicated-route requests need it too); audit `change_request.evidence_added`;
  owner kind `change_request`; API.md row added (DR-363). Closes gap item (d).
- G1-P04-05: method `approver_ids(conn, object_kind, object_key)`; eligible holders of the open deciding task plus
  deciders of the current pass; maker excluded (phase 11 admits the requester separately).
- G1-P04-07: the route-map seed of phase 04 lists `platform_release` in the `cr_access` rule (no later migration);
  owner kind `release` added to phase 02 (7.10; run 1 missed it).
- G1-P04-08 and G1-P03-09: outbox rows are written only from phase 22, which registers the topic handler (phase 00:
  an unregistered topic fails and sticks).
- G1-P04-09: volume effect fixed at submit; changed measurement refuses the decision (`volume_effect_changed`) (DR-360).
- G1-P04-10: one key `access.assessor_max_days`, default 30, max 30; phase 24 key dropped (DR-361); phases 24, 25 7.x.
- G1-P03-02: one code `report_filing` (DR-362).
- G1-P03-04: two registries `FlagGuard` and `FlagChangeHook` (phases 26, 28, 06 register).
- G1-P03-07: service rule plus unique partial indexes replacing the plain ones.
- G1-P03-08: `HolderScope { Any, Branch, Pool }` from the role_grants scope shape.
- G1-P03-10: port `DirectoryLinkQuery` (`NoDirectory` in phase 03); `source` = `admin` | `directory`; the body of
  `disable_from_directory` is left to phase 31 section 3.12 (the draft's Kratos details dropped).
- G1-P03-13: cargo feature `dev-routes` plus the runtime gate (DR-364).
Edited from the draft: G1-P04-01 drops the unverified phase 21 sub-code; G1-P03-01 uses snake_case `branch_item`,
`liaison_report` and states that reports, evidence and freezes are not kinds; G1-P03-03 cites sections instead of
tile names. Source correction: phase 27 section 4.5 says "Phase 03" for the ChangeTargetRegistry (phase 27 7.1).
Noticed for run 3 (gap item (a)): phase 11 l.1246 uses `principal in Akr::Role::"M"` (wrong Cedar form).
