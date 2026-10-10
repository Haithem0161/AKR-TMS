def edit(p, pairs):
    s=open(p).read()
    for a,b in pairs:
        assert s.count(a)==1,(p,s.count(a),a[:90]); s=s.replace(a,b)
    open(p,'w').write(s)

# status
edit('docs/plan/status.md', [
("phases 09 to 30 (batches B1 to B8; phases 19, 20, 21 and 23 are B6, phases 22, 24, 25 and 26 are B7, phases 27 to 30 are B8)",
 "phases 09 to 31 (batches B1 to B8; phases 19, 20, 21 and 23 are B6, phases 22, 24, 25 and 26 are B7, phases 27 to 31 are B8)"),
("| 31 | Training, Capacity & Correspondent Packs | Planned | | | tbd | | | 3 |",
 "| 31 | Training, Capacity & Correspondent Packs | Spec ready | | | 9 (and the view `train.practice_scores`) | phase 19 `casework.alarms` kind and subject CHECKs widened (`capacity_forecast`, `pool`); phase 03 `UserAdminService::disable_from_directory`, `user.disabled` key `source`, the enable check `directory_inactive`, `Ctx.via` `webhook_hmac`; phase 19 `RfiService` and the QA sampler surge rate; phase 24 register practice column; phase 06 pack builders; phase 00 zones and the edge exclusion of `/api/v1/directory/events` | 15 (13 added, 2 owned) | 9 (TrainingService, CapacityService, CapacitySnapshotter, CapacityWatcher, SimulationHarness, ExposureService, CorrespondentPackService, DirectoryService, DirectoryWebhookVerifier) |"),
("| Domain tables (specified) | 0 | 268 |", "| Domain tables (specified) | 0 | 277 |"),
("| Routes | 0 | 549 |", "| Routes | 0 | 564 |"),
("| Phases specified | 0 | 31 | 32 |", "| Phases specified | 0 | 32 | 32 |"),
("| Screens | 0 | 60 (", "| Screens | 0 | 62 ("),
("D4) | 62 (DESIGN-SYSTEM.md 8.1) |", "D4, W12, G2) | 62 (DESIGN-SYSTEM.md 8.1) |"),
("| `partition_maintenance` (extended) | 02 | Same schedule: `kyc.ratings` through `kyc.create_partition` |",
 "| `capacity_snapshot` | 31 | `schedule.capacity_snapshot` (hourly, minute 5): measured arrivals, decisions, handling minutes and backlog per pool, work kind and class, no person column; alerts no queue admits go to the unassigned row; idle while `pack.capacity` is off |\n"
 "| `capacity_watch` | 31 | `schedule.capacity_watch` (06:20): raises `capacity_forecast` through phase 19 `AlarmService` when the horizon backlog grows and required analysts exceed staffed |\n"
 "| `capacity_simulation` | 31 | Enqueued by `POST /capacity/simulations`: fluid queue over a completed phase 16 replay; estimates only |\n"
 "| `exposure_compute` | 31 | `schedule.exposure_compute` (2nd of the month, 01:30): typology exposure of the previous month, small numbers suppressed; idle while `pack.exposure` is off |\n"
 "| `directory_reconcile` | 31 | `schedule.directory_reconcile` (03:15): repeats a failed directory disable, counts stale pending rows, directory-disabled accounts awaiting an ADM decision and mismatches; never offboards; idle while `pack.polis` is off |\n"
 "| `pack_build` (extended) | 31 | `POST /packs` kinds `correspondent` and `validator` from system facts |\n"
 "| `partition_maintenance` (extended) | 02 | Same schedule: `kyc.ratings` through `kyc.create_partition` |"),
])

# README
edit('docs/README.md', [
("Write phase 31 as a Haiku agent draft (owner's choice, 2026-10-09), reviewed, corrected and aligned by the lead; then the gap passes (phase-30 reviewed 2026-10-09, DR-317 to DR-337;",
 "All 32 phase files (00-31) are written and aligned. Next: gap pass 1, then the adversarial pass and the verification pass (the owner decides whether Haiku agents draft them or the lead does them inline) (phase-31 reviewed 2026-10-09, DR-338 to DR-355; phase-30 reviewed 2026-10-09, DR-317 to DR-337;"),
("Scaffolding done; phases 00-01 written (2026-10-07); 02-31 to write |",
 "Scaffolding done; phases 00-01 written (2026-10-07); 02-31 written and aligned (2026-10-09); gap passes next |"),
])

# tech-stack
edit('tech-stack.md', [
("Generic OIDC upstream as configuration (Entra ID, ADFS, Keycloak); Ory Polis optional for SAML and SCIM |",
 "Generic OIDC upstream as configuration (Entra ID, ADFS, Keycloak); Ory Polis optional for SAML and SCIM (phase 31: Polis is the OIDC upstream for a SAML IdP, flag `pack.polis`; SCIM creates no account and a deactivation disables, never offboards; own database, admin interface off) |"),
])

# conventions
edit('tools/agent/b3-conventions.md', [
("(DR-01..DR-337 in docs/plan/research.md)", "(DR-01..DR-355 in docs/plan/research.md)"),
("grep its real DDL in docs/plan/phase-00..30.md.", "grep its real DDL in docs/plan/phase-00..31.md."),
("  `ops.failover_drill_max_age_days`, `ops.failover_max_s`, `ha.standby_reads`, `ha.standby_max_lag_s`); next 0096.\n",
 "  `ops.failover_drill_max_age_days`, `ops.failover_max_s`, `ha.standby_reads`, `ha.standby_max_lag_s`); next 0096.\n"
 "- (phase 31 review) An unattended external signal never triggers an irreversible act: a directory deactivation\n"
 "  disables (reversible), the final offboarding stays a person's act with step-up.\n"
 "- (phase 31 review) A score taken after the answer was shown is not a pass: score the first attempt only.\n"
 "- (phase 31 review) A rule credited to another phase must exist there; grep it, or state it as this phase's rule.\n"
 "- (phase 31 review) A webhook route meant for one internal caller must be refused at the edge, not only signed.\n"
 "- Phase 31: tables train.practice_cases, train.practice_attempts (view train.practice_scores), casework.capacity_inputs,\n"
 "  capacity_snapshots, capacity_runs, rfi_packs, rpt.typology_exposure, admin.directory_users, directory_events; flags\n"
 "  `pack.training`, `pack.capacity`, `pack.exposure`, `pack.correspondent`, `pack.polis`; scopes `capacity.plan`,\n"
 "  `directory.sync`; Cedar E0_directory, E1_training_capacity, E2_exposure_packs; migrations 0096-0100 (next 0101).\n"),
])

# gap items
s=open('tools/agent/gap-items.md').read()
assert '(aa) Phase 31' not in s
s=s.rstrip('\n')+"\n" + (
"(aa) Phase 31: phase 03 `UserAdminService::disable_from_directory`, `user.disabled` key `source`, the enable check\n"
"`directory_inactive`, `Ctx.via` `webhook_hmac`, creation of the service principal `polis-directory`; phase 19 QA sampler\n"
"surge rate and the `AlarmService` kind row `capacity_forecast`; phase 24 `TrainingRegisterService::list` practice column\n"
"(first attempt only); phase 06 pack builders `correspondent` and `validator`; phase 00 zones (Polis rows), the Caddy\n"
"exclusion of `/api/v1/directory/events`, AppRoles `akr-polis` and `kv/polis/directory` for akr-api; phase 01 `db\n"
"provision` database `polis`; phase 00 schema list gains `train`; DESIGN-SYSTEM W12, G2 detail and the W8, R4, G1, G9,\n"
"A1 panels; the alert-to-pool attribution rule (phase 31 section 3.3, Open doubts 4); the Polis paths and webhook header\n"
"format from the pinned version (Open doubts 13).\n")
open('tools/agent/gap-items.md','w').write(s)
print('ok')
