def edit(p,R,append=None):
    s=open(p).read()
    for a,b in R:
        assert s.count(a)==1,(p,s.count(a),a[:70]); s=s.replace(a,b)
    if append:
        assert append[:40] not in s; s=s.rstrip('\n')+'\n'+append
    open(p,'w').write(s)
edit('docs/MODULES.md',[(
"body (refused, idempotency); the gateway times out and releases (release-without-decision control raises an item).\n",
"body (refused, idempotency); the gateway times out and releases (release-without-decision control raises an item).\n"
"Phase file: docs/plan/phase-29.md (DR-292 to DR-316): an optional pack (flag `pack.assess`, default off, 404 on every\n"
"route); gateways bound by a pinned certificate and a service principal; decisions signed in process with an Ed25519\n"
"seed wrapped under `akr-kek`, persisted with their response bytes before the answer, replayed byte for byte from the\n"
"service's own ledger, and valid for ten minutes; MT103, pacs.008 and canonical input; the phase 14 index and freshness\n"
"rule inline, no model and no pair memory inline; freeze-list hits hold (PEP and internal by setting); modes shadow,\n"
"advisory and enforcing per rail with governed entry criteria; fail policies with bands, modes and prohibited rules\n"
"(closed grammar) as `cr_material` change targets with no emergency change; holds decided by two people, list holds as\n"
"phase 14 alerts and the freeze through phases 14 and 20; the release-without-decision control on the canonical copy,\n"
"including releases after a pass expired; outage periods, weekly control tests and a signed freeze-list extract for the\n"
"bank's own fallback (no matcher of this product beside the gateway). Left open: domestic payments on stale lists\n"
"(C-57 reading), raw RTGS and ACH formats, and pair memory inline (phase 29 Open doubts 4, 6 and 12).\n")])
edit('docs/README.md',[("Write phases 29-31 as Haiku agent drafts in batches (owner's choice, 2026-10-09), each reviewed, corrected and aligned by the lead; then the gap passes (phase-28 reviewed",
"Write phases 30-31 as Haiku agent drafts in batches (owner's choice, 2026-10-09), each reviewed, corrected and aligned by the lead; then the gap passes (phase-29 reviewed 2026-10-09, DR-292 to DR-316; phase-28 reviewed")])
edit('tools/agent/b3-conventions.md',[
("(DR-01..DR-291 in docs/plan/research.md)","(DR-01..DR-316 in docs/plan/research.md)"),
("grep its real DDL in docs/plan/phase-00..28.md.","grep its real DDL in docs/plan/phase-00..29.md."),
("  C2_assistant.cedar (C0_*, C1, C2 taken); migrations 0088-0090 (next 0091).\n",
"  C2_assistant.cedar (C0_*, C1, C2 taken); migrations 0088-0090 (next 0091).\n"
"- (phase 29 review) Reuse an earlier rule instead of restating it: list freshness is phase 14 `ListRecord::stale`; a\n"
"  restated rule with a grace drifted past the CBI 12-hour floor.\n"
"- (phase 29 review) A control that relies on an expiring token must check the expiry: a release matched to any pass\n"
"  missed releases made after the pass expired.\n"
"- (phase 29 review) A system-made record uses the run kind and requester the owning phase defines for system work\n"
"  (phase 14 `payment` run, `requested_by` NULL), and must not duplicate what the owning phase's own job creates later.\n"
"- (phase 29 review) A job or watch of an optional pack is idle while the pack flag is off; otherwise every install\n"
"  without the pack raises warnings forever.\n"
"- (phase 29 review) A readiness criterion must be able to fail in the state it is checked in; a column that is NULL\n"
"  outside one mode needs `coalesce` in roll-ups.\n"
"- Phase 29 tables: assess.rails, gateways, signing_keys, decisions (partitioned), idempotency, node_status,\n"
"  blind_periods, policy_state, amount_bands, fail_policies, rail_modes, prohibited_rules, holds, hold_candidates,\n"
"  control_state, release_exceptions, control_tests, mode_rollups, fallback_extracts; phase 24 `registers.source` +\n"
"  `assess_control`; flag `pack.assess`; Cedar D0_assess.cedar (D0 taken); migrations 0091-0094 (next 0095).\n")])
edit('tools/agent/gap-items.md',[],append=
"(y) Phase 29: phase 14 `ScreeningService::open_hold_alert` (system `payment` run, `trigger_ref` = hold id), the\n"
"hold-alert subject view from `assess.holds.party_names_enc`, the `subject_id` comment of `alert.screening_runs`\n"
"widened, `screening_payment` skipping an entry already alerted on a hold with the same match key, and\n"
"`list.feed_unchanged` renewing the freshness clock (phase 29 Open doubts 29); phase 00 `akr-assess` in `z1_bao`,\n"
"line 588 `akr-sign-decision` reserved and unused, AppRole transit decrypt on `akr-kek` and KV read `keys/assess-*`\n"
"(Open doubts 1-2); phase 02 owner kind `assess_extract` and the PartitionRegistry entry for `assess.decisions`; phase\n"
"03 `Ctx.via` value `gateway_cert`; phase 23 drop of a decision partition only after its holds are gone; phase 24\n"
"review-calendar row `assess_control_test` and finding closure closing release exceptions; phase 25 SIEM and support\n"
"bundle exclusions; phase 01 service-principal expiry versus certificate life (Open doubts 24); DESIGN-SYSTEM S2 tab\n"
"Holds, D3 tab Assessment, G1 tile Assessment.\n")
print('ok')
