# Verification pass, phase 30 (Haiku draft for lead review)

## Items selected (step 1)

Phase 30 has no "Adversarial pass" blocks (grep finds none), so the extra items come from the header list. There are four 7.x blocks, so all four are used.

Header "Implemented here" (phase-30.md:12), 14 IDs in order; 8 picked, at positions 1, 3, 6, 7, 9, 11, 13 and 14:
- V-30-01 TS-29, one catalogue and one generator (position 1; sections 3.1 to 3.2)
- V-30-02 TS-32, PostgreSQL proof of concept (position 3; sections 3.4 and 6.2)
- V-30-03 TS-30, signed charts, K3s artifacts and admission (position 6; sections 3.11 and 3.12)
- V-30-04 TS-39, monthly restore rehearsal (position 7; section 3.9)
- V-30-05 SP-04, zones as NetworkPolicies and cluster zone check (position 9; section 3.3)
- V-30-06 SP-85, failover drill and its record (position 11; section 3.9)
- V-30-07 SP-87, database outage recorded as a blind period (position 13; section 3.8)
- V-30-08 SP-88, ransomware restore (position 14; section 3.9)

Gap pass blocks (section 7+, phase-30.md:560 onward), all four:
- V-30-09 7.1 pack flags in the catalogue (phase-30.md:564)
- V-30-10 7.2 deletion certificate key (phase-30.md:573)
- V-30-11 7.3 reconciliation after a restore (phase-30.md:577)
- V-30-12 7.4 who sees failover and restore reports (phase-30.md:581)

## Checked items

### V-30-01 TS-29 one catalogue and one generator for compose and chart   (source: Implemented here TS-29, sections 3.1 to 3.2; result: GAPS 1)
- T1 SQL: N/A - the render reads and writes files only, no table (phase-30.md:80-88).
- T2 Route: N/A - CLI command, no route: "Steps of `akr-ctl deploy render --profile s|m|l --out DIR`" (phase-30.md:80).
- T3 Repository and types: PASS - `DeployRenderer` and the catalogue reader are named (phase-30.md:63); catalogue fields listed (phase-30.md:78); flags given (phase-30.md:80).
- T4 Logic: PASS - steps 1 to 6 are numbered (phase-30.md:81-86); "any step refuses with a non-zero exit code and writes nothing" (phase-30.md:88); exit codes "with 0 on success and with 2 on a failed check" (phase-30.md:399).
- T5 Audit: N/A - writes files only, no database mutation.
- T6 Cross-references: PASS - "zones.yaml is the single statement of who reaches whom" (phase-00.md:717); compose bundle `deploy/compose/` (phase-00.md:688); networks `internal: true` (phase-00.md:712); akr-assess profile `phase2` (phase-00.md:708); OpenAPI and Orval drift check "make generate-api" (phase-00.md:757). The phrase "no second matrix" in phase-30.md:77 is not verbatim in phase 00; the substance is at phase-00.md:717.
- T7 Consistency: PASS - "byte for byte" in step 2 (phase-30.md:82) agrees with 7.1 (phase-30.md:567). Nothing in 7.1 gives akr-assess a pack flag, and phase-29.md:1341 keeps it in profile `phase2`.
- Gaps:
GAP V-30-01-a | LOW | T4 | the chart dependency versions (cert-manager, Kyverno, the PostgreSQL HA chart, OpenBao, Valkey) are not named; phase-30.md:83 says only "pinned by version and digest in Chart.yaml" | section 3.1 step 3 (name the pins, or the file that holds them) | greps: `grep -n -i 'cert-manager\|kyverno' docs/plan/phase-00.md` shows no chart version; phase-30.md names none

### V-30-02 TS-32 PostgreSQL proof of concept with pass criteria   (source: Implemented here TS-32, sections 3.4 and 6.2; result: GAPS 4)
- T1 SQL: GAP (see V-30-02-a) - the writer inserts "with `nextval`" (phase-30.md:550), but `drill.seq` is defined only as a table (phase-30.md:550). The setting `ops.failover_max_s` is seeded at phase-30.md:46.
- T2 Route: N/A - test harness only; `akr-pg-rw` is a Service, not a route (phase-30.md:146).
- T3 Repository and types: PASS - script and subcommand named: "tests/ha/phase30_failover.sh poc-postgres" (phase-30.md:148) and the script file (phase-30.md:554).
- T4 Logic: PASS - ten numbered pass criteria (phase-30.md:149-158) and the decision rule (phase-30.md:160). The writer and checker steps are in phase-30.md:550. Two-writer locking is N/A (test only). The contradiction in the rule is at T7.
- T5 Audit: N/A - "test-only schema drill", "never in a product migration" (phase-30.md:550).
- T6 Cross-references: PASS for `ops.failover_max_s` (phase-30.md:46); pgBackRest policy "WAL every 60 s, weekly full, nightly differential, four fulls" (phase-00.md:694); `db migrate` runs as `akr_migrate` (phase-00.md:624); partition factories `audit.create_partition` (phase-02.md:206), `canon.create_partition` (phase-10.md:859), `feat.create_partition` (phase-13.md:225), `kyc.create_partition` (phase-15.md:179), `assess.create_partition` (phase-29.md:127). Gaps b and c below.
- T7 Consistency: GAP (see V-30-02-d) - phase-30.md:148 says each candidate "must meet every pass criterion"; phase-30.md:160 lets CloudNativePG win on criteria 1 to 4 and 7 to 10; DR-320 (phase-30.md:590) says "passes the same proof-of-concept criteria".
- Gaps:
GAP V-30-02-a | MEDIUM | T1 | the sequence behind nextval is not created; drill.seq is a table, so a sequence needs its own name and DDL | section 6.2 (add the CREATE SEQUENCE of the writer to the test DDL) | greps: `grep -n -i 'sequence\|nextval' docs/plan/phase-30.md` gives lines 150 and 550 only; no CREATE SEQUENCE in phase-30.md
GAP V-30-02-b | LOW | T6 | "the phase 00 stanza" is cited (phase-30.md:153), but phase 00 never names a pgBackRest stanza | section 3.4 criterion 5 (name the stanza or the config file it uses) | greps: `grep -n -i 'stanza' docs/plan/phase-00.md` gives nothing
GAP V-30-02-c | LOW | T6 | "SQLx pool options of phase 00" (phase-30.md:146) are cited, but phase 00 defines only role settings (phase-00.md:36); max_lifetime, acquire_timeout, test_before_acquire and the connect timeout are defined only in phase 30 | section 3.4 pool paragraph (phase-30.md:146): state them as this phase's values | greps: `grep -n -i 'lifetime\|acquire\|test_before\|connect_timeout' docs/plan/phase-00.md` gives nothing
GAP V-30-02-d | MEDIUM | T7 | three statements disagree on what the winner must pass: phase-30.md:148 "must meet every pass criterion"; phase-30.md:160 CloudNativePG needs criteria 1 to 4 and 7 to 10 only; phase-30.md:590 DR-320 "the same proof-of-concept criteria" | section 3.4 decision rule and DR-320 (one statement of the winning condition) | greps: internal to phase-30.md (lines 148, 160, 590)

### V-30-03 TS-30 signed charts, K3s artifacts and admission   (source: Implemented here TS-30, sections 3.11 and 3.12; result: GAPS 2)
- T1 SQL: PASS - migration 0095 drops and re-adds `releases_refusal_code_check` with `chart_signature_invalid` (phase-30.md:33-34); the nine existing codes match phase-25.md:178-181; `cosign_result` is a JSONB column (phase-25.md:183).
- T2 Route: PASS - "POST /api/v1/admin/releases/{id}/applied" with scope `admin.system` (phase-25.md:683) and Cedar `release.applied` (phase-25.md:901). The chart check is a CLI and upgrade-script step with no route (phase-30.md:327).
- T3 Repository and types: PASS - `ReleaseService` (phase-25.md:876); job `release_verify` with `verify(&self, release: Uuid)` (phase-25.md:880); CLI `akr-ctl deploy upgrade --release <id>` (phase-30.md:307).
- T4 Logic: PASS - upgrade steps numbered 1 to 8 (phase-30.md:308-315); "A refused bundle stops the upgrade before any change" (phase-30.md:308); the chart refusal code is `chart_signature_invalid` (phase-30.md:324).
- T5 Audit: PASS - `release.refused` carries the new code (phase-30.md:412; phase-25.md:896); `release.verified` on success, with cosign_result filled (phase-25.md:897 and phase-25.md:1156).
- T6 Cross-references: GAP a and GAP b below. PASS for `akr-ctl bundle verify` and `bundle import` (phase-00.md:629); the bundle carries chart/ and k3s/ (phase-00.md:985-989, 7.12); `BAO_SECRET_ID_FILE` (phase-00.md:473).
- T7 Consistency: PASS - "Verify the bundle first (section 3.12)" (phase-30.md:308) agrees with the upgrade-script gate (phase-30.md:327) and 4.5 item 5 (phase-30.md:484); phase-00.md:989 says bundle verify also checks the chart signatures.
- Gaps:
GAP V-30-03-a | LOW | T6 | section 3.12 cites "phase 00 section 3, step 7" for the bundle, but the bundle steps are in phase 00 section 4 (phase-00.md:986 "step 7 of section 4"), as 4.5 item 5 also says | section 3.12 first bullet (phase-30.md:323) | greps: `grep -n 'step 7' docs/plan/phase-00.md` gives line 986 only
GAP V-30-03-b | LOW | T6 | section 3.12 cites "phase 25 section 3.6, step 1 (c)" for the chart check; (c) is the image signature check (phase-25.md:893), and the chart check is check (g) (phase-25.md:1393-1395) | section 3.12 second bullet (phase-30.md:324) | greps: `grep -n 'check (g)' docs/plan/phase-25.md` gives line 1395 only

### V-30-04 TS-39 monthly restore rehearsal on profile M   (source: Implemented here TS-39, section 3.9; result: PASS)
- T1 SQL: PASS - `ops.last_restore_drill_at` (phase-00.md:264) and `ops.restore_drill_max_age_days` = 31 (phase-00.md:266) exist, unchanged (phase-30.md:434); no new table.
- T2 Route: N/A - CLI commands only, no route (phase-30.md:293).
- T3 Repository and types: PASS - `RestoreVerifier` (phase-30.md:63); the record command with `--result passed|failed --evidence <file>` (phase-00.md:630), used as "`akr-ctl ops record-restore-drill --result passed --evidence FILE` (phase 00)" (phase-30.md:293).
- T4 Logic: PASS - "the restore runs ... followed by `restore verify`, then `akr-ctl ops record-restore-drill`" (phase-30.md:293) gives the order. The namespace has no network or admission rule; logged at V-30-05-a.
- T5 Audit: PASS (inherited) - phase 00 emits `restore_drill.recorded` (phase-00.md:632) with its event row (phase-00.md:653); the verify writes `restore_check.recorded` (phase-30.md:408).
- T6 Cross-references: PASS - `backup_check` runs daily (phase-00.md:258) and computes the restore age (phase-00.md:685); evidence owner `ops_drill` with R10 (phase-02.md:1189); pgBackRest backup (phase-00.md:694).
- T7 Consistency: PASS for the rehearsal text. Its namespace `akr-restore` conflicts with "Two namespaces" at phase-30.md:126; that conflict is logged once, at V-30-05-a.

### V-30-05 SP-04 zones as NetworkPolicies and cluster zone check   (source: Implemented here SP-04, section 3.3; result: GAPS 2)
- T1 SQL: N/A - the zone matrix lives in `deploy/zones.yaml`, not SQL (phase-30.md:77).
- T2 Route: N/A - CLI `akr-ctl net check --cluster` (phase-30.md:134), no route.
- T3 Repository and types: PASS - `ClusterZoneCheck` is named (phase-30.md:63); the command and its exit rule are given (phase-30.md:134).
- T4 Logic: PASS for the four rendered-policy rules (phase-30.md:129-132) and the known-bad probe (phase-30.md:136). GAP V-30-05-b (probe image and "observed" rule) below.
- T5 Audit: N/A - probe Jobs and a result table; no database mutation.
- T6 Cross-references: PASS - `make zone-check` and "deploy/zones.yaml is the single statement" (phase-00.md:717-718); zones `z1_cbs` (phase-00.md:707), `z2_store` (phase-00.md:697), `z4_gateway` (phase-00.md:708), `z5_mgmt` (phase-00.md:710), `z2_model` (phase-00.md:933); port 8443 on akr-assess (phase-29.md:1410).
- T7 Consistency: GAP V-30-05-a below.
- Gaps:
GAP V-30-05-a | MEDIUM | T7 | "Two namespaces" (phase-30.md:126), the rendered policies (phase-30.md:128), the cluster probe (phase-30.md:134) and admission (phase-30.md:326) cover only akr and akr-mgmt, but section 3.9 runs the rehearsal in a third namespace, akr-restore (phase-30.md:293) | section 3.3 (add akr-restore to the namespaces, the render and the zone check) and section 3.12 (admission scope) | greps: `grep -rn 'akr-restore' docs/plan/*.md` gives phase-30.md:293 only
GAP V-30-05-b | LOW | T4 | the probe Job's image and the rule for "observed" (connect succeeds, refused or timeout) are not named; probe Jobs are not catalogue entries, so the digest rule of phase-30.md:81 does not reach them | section 3.3 cluster zone check (phase-30.md:134) | greps: `grep -n -i 'probe' docs/plan/phase-30.md` names no probe image, timeout or connect rule

### V-30-06 SP-85 failover drill and its record   (source: Implemented here SP-85, section 3.9; result: GAPS 3)
- T1 SQL: PASS - `ops.last_failover_drill_at` (seeded at phase-30.md:41) and `ops.failover_drill_max_age_days` (phase-30.md:43) are in migration 0095; evidence kind `package` is allowed (phase-02.md:355); owner kind `ops_drill` is registered (phase-02.md:1189).
- T2 Route: N/A - CLI command (phase-30.md:277); the age tile is read through `GET /admin/ops`, scope admin.system (phase-03.md:521).
- T3 Repository and types: PASS - `FailoverDrillRecorder` is named (phase-30.md:63); flags are given (phase-30.md:277); `SettingsService::apply(tx, key, value, expected_version, actor, source)` with `ChangeSource::Direct` (phase-00.md:574). The item does not pass `expected_version`; that is inferable.
- T4 Logic: GAP V-30-06-c below. PASS for steps 1 to 4 (phase-30.md:278-281) and the six-case table with pass criteria (phase-30.md:268-275); exit codes from phase-30.md:399.
- T5 Audit: PASS - `failover_drill.recorded`, SIEM yes (phase-30.md:407); listed in the catalogue by 4.5 item 7 (phase-30.md:486).
- T6 Cross-references: GAP V-30-06-a and GAP V-30-06-b below. PASS for the `ops record-restore-drill` pattern (phase-00.md:630) and `ops_drill` (phase-02.md:1189).
- T7 Consistency: PASS - the age tile reads the same two settings (phase-30.md:281; tile at phase-30.md:360); the floor "max 92" (phase-30.md:430) fits phase-00.md:208, which allows `{"max": n}` floors.
- Gaps:
GAP V-30-06-a | MEDIUM | T6 | the CLI-only exemption is defined for "this one governed key" (phase-00.md:631); section 3.9 uses it for ops.last_failover_drill_at (phase-30.md:278) and DR-328 says the same (phase-30.md:598), but 4.5 amends no phase 00 rule for it | section 4.5 (add a phase 00 item that widens the exemption to the second key) or section 3.9 step 1 | greps: `grep -rn -i 'CLI-only exemption' docs/plan/*.md` gives phase-00.md:631, phase-30.md:278 and phase-30.md:598 only
GAP V-30-06-b | MEDIUM | T6 | migration 0095 seeds ops.last_failover_drill_at as null (phase-30.md:41), but phase 00 allows a null value only for keys the registry marks nullable, and names ops.last_restore_drill_at (phase-00.md:665); 4.5 does not add the new key | section 4.5 (add a phase 00 nullable-list item) | greps: `grep -rn 'marks nullable' docs/plan/*.md` gives phase-00.md:665 only
GAP V-30-06-c | MEDIUM | T4 | "--result passed" with "--cases-passed" below 6 is not refused: step 1 writes no date, but steps 2 and 3 still store the report and audit a passed drill (phase-30.md:278-280); the inputs can disagree and no rule says what happens | section 3.9 "Recording a drill" (phase-30.md:277-281): state the refusal and its exit code from section 3.16 | greps: internal to phase-30.md (lines 277-281)

### V-30-07 SP-87 database outage recorded as a blind period   (source: Implemented here SP-87, section 3.8; result: GAPS 6)
- T1 SQL: PASS - `admin.blind_periods` has `scope` (phase-11.md:80), `cause` with `akr_outage` (phase-11.md:83), `declared_by` NOT NULL (phase-11.md:88), `statement` as an object (phase-11.md:90) and `ended_at >= started_at` (phase-11.md:93); `audit.audit_anchors.anchored_at` (phase-02.md:279); `ops.probe_stale_s` = 180 (phase-03.md:301).
- T2 Route: N/A - the operator path is a CLI (phase-30.md:231); the automatic path has no route.
- T3 Repository and types: GAP V-30-07-a below. PASS for the operator CLI inputs `--from T1 --to T2 --evidence FILE` (phase-30.md:231).
- T4 Logic: PASS for the automatic path's conditions and fields (phase-30.md:230). GAP V-30-07-b and GAP V-30-07-d below.
- T5 Audit: GAP V-30-07-c below. PASS for the automatic path: `blind_period.opened` is the kind phase 11 uses for automatic opens (phase-11.md:1198).
- T6 Cross-references: GAP V-30-07-e below. PASS for `BlindPeriodService` (phase-11.md:968), the system principal (phase-01.md:97-100), `ops_probe` every minute (phase-03.md:311), `assess.blind_periods` (phase-29.md:178) and DR-306 (phase-29.md:1521).
- T7 Consistency: GAP V-30-07-f below.
- Gaps:
GAP V-30-07-a | HIGH | T3 | no method for an automatic system-scope open: phase 11 opens periods automatically for source scope only (phase-11.md:974); system periods come from a steward's declaration (phase-11.md:991), with Cedar `blind_period.declare` for DS only (phase-11.md:1215). Section 3.8 names neither a method nor an authority for the probe (phase-30.md:230) | section 3.8 item 1 (name the method and the system authority), or extend phase 11 section 3.6 with that method | greps: `grep -n -i 'automatic' docs/plan/phase-11.md` lists source-scope opens and the source close only; `grep -n -i 'blind' docs/plan/phase-00.md docs/plan/phase-03.md` gives nothing
GAP V-30-07-b | MEDIUM | T4 | the operator path gives no comment (phase-11.md:994 requires 10 to 2,000 characters) and no declared_by user (phase-11.md:88, NOT NULL) for a CLI run (phase-30.md:231) | section 3.8 item 2 (phase-30.md:231) | greps: `grep -n 'comment\|declared_by' docs/plan/phase-30.md` shows declared_by only for the system principal, at line 230
GAP V-30-07-c | MEDIUM | T5 | the operator path names blind_period.opened (phase-30.md:231; 3.17 row at phase-30.md:410), but a steward's system-scope declaration emits blind_period.declared (phase-11.md:996 and phase-11.md:1199) | section 3.8 item 2 and the 3.17 row for ha.outage_recorded (phase-30.md:410) | greps: `grep -n 'blind_period.declared' docs/plan/phase-11.md` gives lines 996, 1199 and 1467 (curl audit list)
GAP V-30-07-d | MEDIUM | T4 | two writers can open overlapping system periods for one outage (the probe at phase-30.md:230, the CLI at phase-30.md:231, or a steward declaration); no lock, unique index or overlap check is stated, and phase-11.md:98 covers source scope only | section 3.8 (after item 2): state the rule for an open or overlapping system period | greps: `grep -n 'uq_blind' docs/plan/phase-11.md` gives line 98 and a test line 1434, both source scope
GAP V-30-07-e | MEDIUM | T6 | "the akr-ctl database role of phase 00" (phase-30.md:231) is not defined in phase 00, whose roles are akr_app, akr_ingest, akr_readonly, akr_audit_writer, akr_migrate, akr_monitor and akr_retention_login; the phase 11 grant of anchored_at goes to akr_app only (phase-11.md:407) | section 3.8 item 2 and section 4.4 (name the CLI's database role and its grants) | greps: `grep -n -i 'akr-ctl' docs/plan/phase-00.md | grep -i role` finds no database role for akr-ctl (lines 30 and 886 concern db provision and akr_monitor only)
GAP V-30-07-f | MEDIUM | T7 | phase 11 says a declaration "records the true start" (phase-11.md:994), but section 3.8 item 1 records an estimated started_at for the same kind of row and says phase 11 owns the rules of a system period (phase-30.md:230) | section 3.8 item 1 (state the departure from phase 11 section 3.6, or use the true start) | greps: `grep -n 'true start' docs/plan/phase-11.md` gives line 994 only

### V-30-08 SP-88 ransomware restore steps   (source: Implemented here SP-88, section 3.9; result: GAPS 3)
- T1 SQL: N/A - reuses `akr-ctl restore verify` (phase-30.md:283-289); no table of its own.
- T2 Route: N/A - CLI only.
- T3 Repository and types: N/A - the command is reused (phase-30.md:291); the copy and the clean environment name no object (GAP V-30-08-a).
- T4 Logic: GAP V-30-08-a and GAP V-30-08-b below.
- T5 Audit: GAP V-30-08-c below. PASS for the verify run: `restore_check.recorded` (phase-30.md:408).
- T6 Cross-references: PASS - the credentials of section 3.12 exist (phase-30.md:329); the rotation runbook in phase-25.md:1383 covers keys/auth-hmac only; phase-00.md:596 covers the KEK only.
- T7 Consistency: GAP V-30-08-a also. Section 3.8 says "the immutable copy is the offline copy of section 3.9 (SP-88)" (phase-30.md:259), but section 3.9 defines no such copy; the release row cites "offline media (section 3.11)" (phase-30.md:254), and section 3.11 has no offline media.
- Gaps:
GAP V-30-08-a | HIGH | T4 | the immutable or offline copy that the ransomware restore reads is not defined: no creation step, location, cadence, owner or retention; the clean environment is not named either (phase-30.md:291); Open doubts 22 (phase-30.md:636) leaves the choice to the bank and no other phase carries it | section 3.9 (a new paragraph: the copy, the job or runbook step that makes it, where it is kept, and the clean namespace or cluster) | greps: `grep -rn -i 'ransom' docs/plan/*.md` gives phase-30.md only; `grep -n -i 'immutable\|offline' docs/plan/phase-00.md docs/plan/phase-02.md docs/plan/phase-23.md` gives trigger and legal-hold wording, no copy definition
GAP V-30-08-b | MEDIUM | T4 | "the rotation of section 3.12's credentials" (phase-30.md:291) has no steps: section 3.12 creates the AppRole secret ids at install (phase-30.md:329) and says nothing about rotating them or the database passwords | section 3.9 ransomware paragraph or section 3.12 (list the credentials and the order of rotation) | greps: `grep -n -i 'rotat' docs/plan/phase-00.md` gives the KEK (line 596) and the deletion key (line 919) only; phase-30.md has no rotation step
GAP V-30-08-c | MEDIUM | T5 | the rotation changes credentials and names no audit event; restore_check.recorded (phase-30.md:408) covers the verify only; phase 00 requires a log only for commands that change an installation (phase-00.md:633) | section 3.9 ransomware paragraph (phase-30.md:291): name the event, or the operator log that records the rotation | greps: `grep -n -i 'rotat' docs/plan/phase-30.md` gives line 291 only

### V-30-09 7.1 pack flags in the catalogue   (source: 7.1 block, phase-30.md:564; result: PASS)
- T1 SQL: N/A - the render reads flags from the command line, not the database (phase-30.md:569).
- T2 Route: N/A - CLI option only.
- T3 Repository and types: PASS - `akr-ctl deploy render --pack <name>=on|off` (phase-30.md:566) and the `pack:` catalogue key (phase-30.md:565).
- T4 Logic: PASS - "An entry whose flag is off is left out of every output"; "a flag not given is off" (phase-30.md:566-567).
- T5 Audit: N/A - no mutation.
- T6 Cross-references: PASS - `pack.polis` is phase 31's flag (phase-31.md:311); the polis zone row is in phase 00 7.7 (phase-00.md:936); `pack.assess` is phase 29's flag (phase-29.md:666).
- T7 Consistency: PASS - "with all flags off the output is the phase 00 bundle byte for byte" (phase-30.md:567) agrees with phase-30.md:82. Nothing in 7.1 gives akr-assess a pack flag, and phase-29.md:1341 keeps it in profile `phase2`.

### V-30-10 7.2 deletion certificate key   (source: 7.2 block, phase-30.md:573; result: PASS)
- T1 SQL: N/A - a transit key name; no table.
- T2 Route: N/A - no route.
- T3 Repository and types: N/A - a name only; `CertificateSigner` is phase 23's port (phase-23.md:501).
- T4 Logic: PASS - restore step 1 names the key: "verifies the deletion certificates against akr-sign-deletion" (phase-30.md:284; block at phase-30.md:574).
- T5 Audit: N/A - verification only, no mutation.
- T6 Cross-references: PASS - phase-23.md:1002 renames the transit key to `akr-sign-deletion` (phase 00 7.6, DR-357); phase-00.md:918 lists `akr-sign-deletion`.
- T7 Consistency: PASS - the phase 23 7.1 block (phase-23.md:1001-1002) replaces the base name at phase-23.md:501, and the block wins. "the name akr-sign-cert is not used" (phase-30.md:574) agrees with it.

### V-30-11 7.3 reconciliation after a restore   (source: 7.3 block, phase-30.md:577; result: GAPS 2)
- T1 SQL: PASS - `recon_runs` and `coverage_ledger` are phase 12 tables (phase-12.md:53).
- T2 Route: PASS - `POST /recon/runs`, scope data.steward (phase-12.md:589); the restore check queues the job without calling the route (phase-30.md:288).
- T3 Repository and types: GAP V-30-11-a below.
- T4 Logic: GAP V-30-11-b below.
- T5 Audit: PASS - phase 12 inserts the run with `requested_by` (phase-12.md:640), audits `recon.requested` (phase-12.md:642) and `recon.completed` at run end (phase-12.md:973); the restore check audits `restore_check.recorded` (phase-30.md:408).
- T6 Cross-references: PASS - "Reconciliation runs after a restore (phase 30 section 3.9)" (phase-12.md:1309) and the 7.2 text that follows (phase-12.md:1310-1312); `certificates_for_date` (phase-12.md:516).
- T7 Consistency: PASS with a note. 7.3 says the runs are valid "while the replay runs" (phase-30.md:578); section 3.9 step 5 queues them after step 2 (phase-30.md:288), and phase-12.md:1310-1311 says the replay completes first. Both reach "valid".
- Gaps:
GAP V-30-11-a | LOW | T3 | the restore check queues "recon_run {run_id}" (phase-30.md:288), but phase 12 inserts the recon_runs row first, with requested_by the actor (phase-12.md:640); section 3.9 names neither the phase 12 method nor the actor | section 3.9 step 5 (phase-30.md:288): name the phase 12 method and the requested_by actor | greps: `grep -n 'requested_by' docs/plan/phase-30.md` gives nothing
GAP V-30-11-b | MEDIUM | T4 | phase 12 refuses a second active day run for the same source and date with 409 run_active (phase-12.md:641); step 5 says the command "waits for their results" and does not say what happens on that refusal, for example a day run already queued for that date | section 3.9 step 5 (phase-30.md:288) | greps: `grep -n 'run_active' docs/plan/phase-30.md` gives nothing

### V-30-12 7.4 who sees failover and restore reports   (source: 7.4 block, phase-30.md:581; result: GAPS 1)
- T1 SQL: N/A - a viewing rule; the evidence object is phase 02's table.
- T2 Route: GAP V-30-12-a below.
- T3 Repository and types: N/A - no viewing method; the header lists 0 routes (phase-30.md:8).
- T4 Logic: N/A - a visibility rule with no steps.
- T5 Audit: PASS - `failover_drill.recorded` and `restore_check.recorded`, SIEM yes (phase-30.md:407-408); SEC reads audit events through `audit.read`, which lists IA and SEC (docs/API.md:100).
- T6 Cross-references: PASS - `admin.system` on `GET /admin/ops` (phase-03.md:521); phase-22.md:5 names only `notify.request` as the notification topic, so "Phase 25 sends no notice" (phase-30.md:583) is consistent.
- T7 Consistency: PASS - 7.4 (phase-30.md:581-583) agrees with 3.17 (phase-30.md:407-408) and 3.18 (phase-30.md:418).
- Gaps:
GAP V-30-12-a | MEDIUM | T2 | "viewable by admin.system" (phase-30.md:418 and phase-30.md:583) has no route or screen that opens the report content: the header has 0 routes (phase-30.md:8), GET /admin/ops shows tiles only (phase-03.md:521), and docs/API.md:236 says there is no generic evidence route (DR-380) | section 3.9 or 3.18: name an owner-scoped route, as phase 18 does for case evidence (phase-18.md:596), or state how the bank's operators read the report | greps: `grep -rn 'ops_drill' docs/plan/*.md` gives phase-02.md:1189, phase-30.md, and the plan tables research.md:339 and status.md:45 only; `grep -n 'evidence' docs/API.md` shows no route for ops_drill

## Notes (not GAP lines)

- phase-30.md:14 and phase-30.md:15 use "not carried" inside the Requirements carried list (block at phase-30.md:11-15). tools/agent/b3-conventions.md says that phrase drops the whole line from tools/idcount.py, so the three IDs there (TS-25, SP-14, VA-44) may be left out of the count.
- phase-00.md:983 (7.11) cites "phase 30 section 3.2" for akr_failover_drill_age_days. The metric is defined in phase-30.md section 3.13 and set by 4.5 item 6 (phase-30.md:485).

## Summary

- Items checked: 12 (V-30-01 to V-30-12).
- Items fully PASS (no GAP line): 3 (V-30-04, V-30-09, V-30-10).
- GAP lines by severity: HIGH 2, MEDIUM 15, LOW 7 (24 in total).

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-30-01-a | LOW | T4 | chart dependency versions are not named | section 3.1 step 3 |
| V-30-02-a | MEDIUM | T1 | sequence behind nextval is not created | section 6.2 |
| V-30-02-b | LOW | T6 | "phase 00 stanza" is not defined in phase 00 | section 3.4 criterion 5 |
| V-30-02-c | LOW | T6 | SQLx pool options attributed to phase 00 but defined only in phase 30 | section 3.4 pool paragraph |
| V-30-02-d | MEDIUM | T7 | "must meet every pass criterion" conflicts with the decision rule and DR-320 | section 3.4 decision rule; DR-320 |
| V-30-03-a | LOW | T6 | cites phase 00 section 3 step 7; the steps are in section 4 | section 3.12 first bullet |
| V-30-03-b | LOW | T6 | cites phase 25 step 1 (c); the chart check is check (g) | section 3.12 second bullet |
| V-30-05-a | MEDIUM | T7 | akr-restore namespace has no render, zone-check or admission rule | sections 3.3 and 3.12 |
| V-30-05-b | LOW | T4 | cluster probe image and "observed" rule not named | section 3.3 cluster zone check |
| V-30-06-a | MEDIUM | T6 | CLI-only exemption is for one key; second key not amended | section 4.5 |
| V-30-06-b | MEDIUM | T6 | seeded null for a key not marked nullable in phase 00 | section 4.5 |
| V-30-06-c | MEDIUM | T4 | passed drill with fewer than six cases not refused | section 3.9 Recording a drill |
| V-30-07-a | HIGH | T3 | no method or authority for the automatic system-scope open | section 3.8 item 1 |
| V-30-07-b | MEDIUM | T4 | operator path gives no comment or declared_by | section 3.8 item 2 |
| V-30-07-c | MEDIUM | T5 | operator path names blind_period.opened; phase 11 uses blind_period.declared | section 3.8 item 2; 3.17 row |
| V-30-07-d | MEDIUM | T4 | two writers can open overlapping system periods | section 3.8 |
| V-30-07-e | MEDIUM | T6 | "akr-ctl database role of phase 00" does not exist | section 3.8 item 2; section 4.4 |
| V-30-07-f | MEDIUM | T7 | estimated started_at conflicts with phase 11 "true start" | section 3.8 item 1 |
| V-30-08-a | HIGH | T4 | immutable or offline copy and clean environment not defined | section 3.9 |
| V-30-08-b | MEDIUM | T4 | credential rotation has no steps | section 3.9 or 3.12 |
| V-30-08-c | MEDIUM | T5 | credential rotation names no audit event | section 3.9 |
| V-30-11-a | LOW | T3 | recon run row and phase 12 method not named | section 3.9 step 5 |
| V-30-11-b | MEDIUM | T4 | phase 12 run_active refusal not handled | section 3.9 step 5 |
| V-30-12-a | MEDIUM | T2 | no route or screen shows the failover and restore report content | section 3.9 or 3.18 |
