# Refuter, phase 30 (Haiku draft for lead review)

### V-30-01-a  REFUTED   (first agent: LOW; refuter: none)
- Claim: the chart dependency versions (cert-manager, Kyverno, the PostgreSQL HA chart, OpenBao, Valkey) are not named; the plan says only that they are pinned in Chart.yaml.
- Evidence: docs/plan/phase-30.md:83 "are dependencies pinned by version and digest in `Chart.yaml`". The plan uses the same convention for images and prints no image digest either: docs/plan/phase-30.md:78 "`image` (repository and pinned digest)"; docs/plan/phase-00.md:692 "PostgreSQL 17 (pinned digest)". The PostgreSQL HA chart is still open: docs/plan/phase-30.md:140 "Provisional default: Patroni 4.1.5 on PostgreSQL 17". The greps of the claim are right that no chart version appears in phase 00, but the design names the file that holds the pins.

### V-30-02-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the sequence behind `nextval` is not created; `drill.seq` is a table, so the writer needs a sequence with its own name and DDL.
- Evidence: docs/plan/phase-30.md:550 "It holds one table `drill.seq (id bigint primary key, written_at timestamptz)`." and "The writer inserts `id` every 10 milliseconds with `nextval`". The `id` column has no default, so the insert cannot use `nextval` as written. `grep -rn -i 'nextval\|create sequence\|drill\.seq' docs/plan/*.md` finds no sequence for the drill schema; the hits in docs/plan/phase-05.md (lines 904 and 1421) are validator rules, not DDL. Other phases declare their sequences, for example docs/plan/phase-20.md:47 "CREATE SEQUENCE IF NOT EXISTS casework.ops_ref_seq;".
- Fix: section 6.2 (docs/plan/phase-30.md:550): add the sequence that the writer's `nextval` uses to the test-only DDL.

### V-30-02-b  STANDS   (first agent: LOW; refuter: LOW)
- Claim: "the phase 00 stanza" is cited (phase-30.md:153), but phase 00 never names a pgBackRest stanza.
- Evidence: docs/plan/phase-30.md:153 "the phase 00 stanza is used unchanged". Phase 00 names the pgBackRest service and its policy only: docs/plan/phase-00.md:694 "WAL every 60 s, weekly full, nightly differential, four fulls". `grep -n -i 'stanza' docs/plan/phase-00.md` finds nothing. The claim holds.
- Fix: phase 00, section 4, the pgbackrest row (docs/plan/phase-00.md:694): name the stanza the service uses; section 3.4 criterion 5 then needs no change.

### V-30-02-c  PARTLY   (first agent: LOW; refuter: LOW, attribution only)
- Claim: "SQLx pool options of phase 00" (phase-30.md:146) are cited, but phase 00 defines only role settings; `max_lifetime`, `acquire_timeout`, `test_before_acquire` and the connect timeout are defined only in phase 30.
- Refuted part: the values are stated in this phase, in the same paragraph: docs/plan/phase-30.md:146 "`max_lifetime` 30 minutes". Nothing is missing from the plan.
- Stands part: the attribution is wrong. Phase 00 defines role settings only: docs/plan/phase-00.md:36 "`statement_timeout`, `idle_in_transaction_session_timeout`". `grep -n -i 'lifetime\|acquire\|test_before\|connect.timeout\|max_connections' docs/plan/phase-00.md` finds nothing.
- Fix: section 3.4 pool paragraph (docs/plan/phase-30.md:146): drop "of phase 00" from the cited name, because the values are this phase's own.

### V-30-02-d  STANDS   (first agent: MEDIUM; refuter: HIGH, two texts contradict)
- Claim: three statements disagree on what the winning candidate must pass.
- Evidence: docs/plan/phase-30.md:148 "must meet every pass criterion"; docs/plan/phase-30.md:160 "CloudNativePG becomes the default only when it meets criteria 1 to 4 and 7 to 10"; docs/plan/phase-30.md:590 "CloudNativePG wins only if it passes the same proof-of-concept criteria". Line 160 lets CloudNativePG win without criterion 5 (pgBackRest integration) and criterion 6 (operator footprint), which line 148 requires of every candidate. No 7.x block and no Open doubts item (docs/plan/phase-30.md:615) says which text wins.
- Fix: section 3.4 decision rule (docs/plan/phase-30.md:160) and DR-320 (docs/plan/phase-30.md:590): state one winning condition, and make line 148 refer to it.
- Severity note: HIGH, because the database tool that wins depends on which text is followed.

### V-30-03-a  PARTLY   (first agent: LOW; refuter: LOW, citation only)
- Claim: section 3.12 cites "phase 00 section 3, step 7" for the bundle, but the bundle steps are in phase 00 section 4.
- Refuted part: the bundle step exists in section 4: docs/plan/phase-00.md:747 "7. Bundle (release branches): `akr-ctl bundle build` writes an OCI image layout", under "## 4. Infrastructure Updates" (docs/plan/phase-00.md:678). Phase 00's own section 7.12 says "(step 7 of section 4)" (docs/plan/phase-00.md:986).
- Stands part: the section number in the citation is wrong: docs/plan/phase-30.md:323 "phase 00 section 3, step 7, extended".
- Fix: section 3.12, first bullet (docs/plan/phase-30.md:323): cite phase 00 section 4, step 7.

### V-30-03-b  PARTLY   (first agent: LOW; refuter: LOW, citation only)
- Claim: section 3.12 cites "phase 25 section 3.6, step 1 (c)" for the chart check, but (c) is the image signature check and the chart check is check (g).
- Refuted part: the chart check is defined as check (g): docs/plan/phase-25.md:1395 "the verify job gains check (g), the Helm chart check, after check (f)" (phase 25 section 7.7, which hands the change to phase 30).
- Stands part: docs/plan/phase-25.md:893 "(c) every image signature is valid" is the image check, so the citation in docs/plan/phase-30.md:324 points at the wrong check.
- Fix: section 3.12, second bullet (docs/plan/phase-30.md:324): cite check (g) of phase 25 section 7.7.

### V-30-05-a  STANDS   (first agent: MEDIUM; refuter: HIGH, two texts contradict)
- Claim: the akr-restore namespace used by the monthly rehearsal has no zone render, no cluster zone check and no admission rule; the sections cover akr and akr-mgmt only.
- Evidence: docs/plan/phase-30.md:126 "Two namespaces: `akr` (every application and data workload) and `akr-mgmt`"; docs/plan/phase-30.md:293 "the restore runs in a separate namespace `akr-restore` on the same cluster"; docs/plan/phase-30.md:326 "for the namespaces `akr` and `akr-mgmt`". The render (docs/plan/phase-30.md:128 "For each namespace") and the cluster probe (docs/plan/phase-30.md:134) use the same two. `grep -rn 'akr-restore' docs/plan/*.md` finds phase-30.md:293 only, and phase 00 has no namespace at all.
- Fix: section 3.3 (namespace list, render and zone check) and section 3.12 (admission scope): add akr-restore, or move the rehearsal to the DR server and say so in section 3.9.
- Severity note: HIGH, because the text says two namespaces and then runs production-data restores in a third one with no policy and no admission check.

### V-30-05-b  PARTLY   (first agent: LOW; refuter: LOW, image and timeout only)
- Claim: the probe Job's image and the rule for "observed" are not named; probe Jobs are not catalogue entries, so the digest rule of phase-30.md:81 does not reach them.
- Refuted part: the observed rule is inherited from the phase 00 zone check: docs/plan/phase-00.md:718 "`make zone-check` starts a probe container", whose result "must equal the matrix", with "open" and "closed" examples (docs/plan/phase-00.md:720). A probe connect that succeeds is "open".
- Stands part: no probe image is named, and the digest rule does not reach it: docs/plan/phase-30.md:78 says the catalogue has "one entry per service with", and the digest rule at docs/plan/phase-30.md:81 is "refuse an image without a digest (`@sha256:` required)" for catalogue entries. The connect timeout is not stated (docs/plan/phase-30.md:134).
- Fix: section 3.3 cluster zone check (docs/plan/phase-30.md:134): name the probe image, by digest, and the connect timeout.

### V-30-06-a  STANDS   (first agent: MEDIUM; refuter: HIGH, two texts contradict)
- Claim: the CLI-only exemption is defined for "this one governed key"; section 3.9 uses it for a second key, and no phase 00 rule is amended for it.
- Evidence: docs/plan/phase-00.md:631 "under a CLI-only exemption for this one governed key"; docs/plan/phase-30.md:278 "the same CLI-only exemption phase 00 uses for `ops.last_restore_drill_at`"; docs/plan/phase-30.md:598 "through the CLI-only exemption that phase 00 uses for the restore drill". No item of section 4.5 (docs/plan/phase-30.md:478 to 496) mentions the exemption, and `grep -n -i 'exemption' docs/plan/phase-30.md` finds lines 278 and 598 only.
- Fix: section 4.5 (add a phase 00 item that widens the exemption to the second key), or section 3.9 step 1 (route the failover record through a change request instead).
- Severity note: HIGH, because the lead must decide whether a second governed key may skip the change request; the two texts give opposite answers.

### V-30-06-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: migration 0095 seeds `ops.last_failover_drill_at` as null, but phase 00 allows a null value only for keys the registry marks nullable, and names only `ops.last_restore_drill_at`.
- Evidence: docs/plan/phase-30.md:41 "'ops.last_failover_drill_at', 'null', 'timestamp'"; docs/plan/phase-00.md:665 "allowed only for keys the registry marks nullable (`ops.last_restore_drill_at` until the first drill)". Phase 30 never uses the word "nullable" (`grep -n -i 'nullable' docs/plan/phase-30.md` finds nothing), and no section 4.5 item adds the key to that list.
- Fix: section 4.5 (add a phase 00 item that marks `ops.last_failover_drill_at` nullable).
- Severity note: MEDIUM, because the intent is clear from the item and the fix is one registry flag.

### V-30-06-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `--result passed` with `--cases-passed` below 6 is not refused; step 1 writes no date, but steps 2 and 3 still store the report and audit a passed drill.
- Evidence: docs/plan/phase-30.md:278 "only when `--result passed` and `K = 6`" (the condition covers the date only); docs/plan/phase-30.md:279 "Stores the report (one document per drill" and docs/plan/phase-30.md:280 "Audit `failover_drill.recorded` (siem yes)" carry no condition. Section 3.16 gives exit 2 "on a failed check" (docs/plan/phase-30.md:399) but does not say that this input is a failed check.
- Fix: section 3.9 "Recording a drill" (docs/plan/phase-30.md:277): state the refusal for a passed drill below six cases, and its exit code, before step 2.

### V-30-07-a  PARTLY   (first agent: HIGH; refuter: HIGH, the stands part only)
- Claim: section 3.8 names no method and no authority for the probe's automatic system-scope open.
- Refuted part: section 3.8 names both: docs/plan/phase-30.md:230 "it opens a blind period through phase 11 `BlindPeriodService`" and "`declared_by` = the system principal".
- Stands part: phase 11 gives automatic opens for source scope only (docs/plan/phase-11.md:974 "**Automatic opens** (source scope;"). A system period comes only from a steward's declaration on POST /coverage/blind-periods (docs/plan/phase-11.md:702 "| POST | /coverage/blind-periods | data.steward |"), and Cedar `blind_period.declare` covers DS only (docs/plan/phase-11.md:1215). Phase 30 says phase 11 "owns the rules of a system period" (docs/plan/phase-30.md:230), but phase 11 gives no such rule for an automatic system open, so the probe path has no method of its own and no authority that phase 11 defines.
- Fix: section 3.8 item 1 (docs/plan/phase-30.md:230): name the system-scope open method of BlindPeriodService and the authority the system principal uses for it, or add both to phase 11 section 3.6.

### V-30-07-b  STANDS   (first agent: MEDIUM; refuter: HIGH, a NOT NULL user reference has no value on the CLI path)
- Claim: the operator path (`akr-ctl ha outage record`) gives no comment and no declared_by user, although phase 11 requires both for a declaration.
- Evidence: docs/plan/phase-11.md:88 "UUID NOT NULL REFERENCES admin.users(id)"; docs/plan/phase-11.md:994 "a `comment` of 10 to 2,000 characters"; docs/plan/phase-11.md:702 makes system declarations a data.steward route. docs/plan/phase-30.md:231 names neither field. The audit actor is logged by docs/plan/phase-00.md:633 "Every command that changes an installation logs who ran it", but that goes to the audit sink and does not fill `declared_by`.
- Fix: section 3.8 item 2 (docs/plan/phase-30.md:231): name the admin user recorded as `declared_by` for a CLI run, and the comment the command requires.
- Severity note: HIGH, because the NOT NULL column has no value on this path, and the command also skips the steward authority that phase 11 requires.

### V-30-07-c  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the operator path names `blind_period.opened`, but a steward's system-scope declaration emits `blind_period.declared`.
- Evidence: docs/plan/phase-11.md:1199 "`blind_period.declared` | a steward's declaration"; docs/plan/phase-11.md:996 "`blind_period.declared {scope, cause, started_at, ended_at}`"; docs/plan/phase-30.md:231 "Audit `blind_period.opened`", and docs/plan/phase-30.md:410 "(the blind period itself is `blind_period.opened`, phase 11)". The operator path is a system declaration under phase 11 (cause `akr_outage`, data.steward route), so phase 11's declaration kind applies.
- Fix: section 3.8 item 2 and the section 3.17 row for `ha.outage_recorded` (docs/plan/phase-30.md:410): name `blind_period.declared` for the operator path, as phase 11 does for declarations.

### V-30-07-d  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: two writers can open overlapping system periods for one outage; no lock, unique index or overlap check is stated for system scope.
- Evidence: docs/plan/phase-11.md:97 "One open period per source: a second cause while one is open extends the statement, it does not open another." and docs/plan/phase-11.md:98 "CREATE UNIQUE INDEX IF NOT EXISTS uq_blind_open_source ON admin.blind_periods (source_id)", which applies only to source scope (its WHERE clause on the next line). Phase 11 has no system-scope rule, and `grep -n 'overlap' docs/plan/phase-11.md` shows only the ingestion re-read window. Phase 30 names the writers at docs/plan/phase-30.md:229 "Two writers, both on the primary", and a steward declaration is a third (docs/plan/phase-11.md:991).
- Fix: section 3.8, after item 2 (docs/plan/phase-30.md:231): state that a second open system period extends the first one's statement, as phase 11 does for sources, or name a unique index for system scope.

### V-30-07-e  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: "the akr-ctl database role of phase 00" (cited at phase-30.md:231) is not defined in phase 00.
- Evidence: the phrase is at docs/plan/phase-30.md:63 "through the `akr-ctl` database role of phase 00". Phase 00 defines no akr-ctl database role. Its LOGIN roles are akr_migrate (docs/plan/phase-00.md:33), akr_monitor and akr_retention_login (docs/plan/phase-00.md:886 to 889); its NOLOGIN roles start at docs/plan/phase-00.md:48 "CREATE ROLE akr_app NOLOGIN". The akr-ctl AppRole is "`akr-ctl` operator" with "policies issued per ceremony, never stored." (docs/plan/phase-00.md:600 to 601). Grants on `admin.blind_periods` go to akr_ingest and akr_app (docs/plan/phase-11.md:312 and 314 "GRANT SELECT, INSERT ON admin.blind_periods TO akr_app;"), so a role to inherit exists, but no section says which role the CLI uses.
- Fix: section 3.8 item 2 and section 4.4 (docs/plan/phase-30.md:472): name the database role the CLI connects as, and the grant it uses on `admin.blind_periods`.

### V-30-07-f  PARTLY   (first agent: MEDIUM; refuter: LOW, wording only)
- Claim: section 3.8 item 1 records an estimated started_at for a system row, but phase 11 says a declaration "records the true start".
- Refuted part: the estimate is a designed choice, stated in the item and in DR-331: docs/plan/phase-30.md:230 "`started_at` is an estimate that the statement names as one"; docs/plan/phase-30.md:601 "opened on recovery by the probe with its estimated start and the statement that says so".
- Stands part: the item says phase 11 "owns the rules of a system period" (docs/plan/phase-30.md:230), yet phase 11's rule for a system period is a declaration that "records the true start" (docs/plan/phase-11.md:994). The item does not say that the automatic path departs from that rule.
- Fix: section 3.8 item 1 (docs/plan/phase-30.md:230): add one sentence saying this automatic path departs from phase 11 section 3.6 on the start time, and why.

### V-30-08-a  STANDS   (first agent: HIGH; refuter: HIGH)
- Claim: the immutable or offline copy that the ransomware restore reads is not defined (no creation step, location, cadence, owner or retention), and the clean environment is not named.
- Evidence: SP-88 asks for it: research/30-security-privacy.md:1461 "restore into a clean environment from an immutable or offline copy". Phase 30 names the copy but defines none: docs/plan/phase-30.md:259 "the immutable copy is the offline copy of section 3.9 (SP-88) and Open doubts 22"; section 3.9 has no such copy (docs/plan/phase-30.md:283 to 291). The release row cites "on offline media (section 3.11)" (docs/plan/phase-30.md:254), and section 3.11 has no offline media (docs/plan/phase-30.md:305 to 319). Open doubts 22 leaves to the bank only "whether the offline monthly copy is its own (SP-88)" (docs/plan/phase-30.md:636); it does not leave the copy's existence or procedure open. The phase 00 bucket list has no immutable database copy (docs/plan/phase-00.md:616 to 618: `akr-evidence`, `akr-anchors` "COMPLIANCE 10 years", `akr-quarantine`, `akr-models`, `akr-exports`). `grep -rn -i 'ransom' docs/plan/*.md` finds phase 30 only.
- Fix: section 3.9 (a new paragraph): name the copy (what makes it, where it is kept, its retention) and the clean namespace or cluster the ransomware restore runs in.
- Severity note: HIGH, because the restore step reads an object that no step creates.

### V-30-08-b  STANDS   (first agent: MEDIUM; refuter: HIGH, a step is missing)
- Claim: "the rotation of section 3.12's credentials" (phase-30.md:291) has no steps, and section 3.12 does not say how any credential is rotated.
- Evidence: docs/plan/phase-30.md:291 "followed by the rotation of section 3.12's credentials before any node is opened to the bank's users"; docs/plan/phase-30.md:329 "are Kubernetes Secrets created by the operator from files". `grep -n -i 'rotat' docs/plan/phase-30.md` finds line 291 only. In phase 00 the rotation rules cover the KEK (docs/plan/phase-00.md:596) and the deletion key (docs/plan/phase-00.md:919), not the AppRole secret ids or the database passwords. SP-88 asks to "rotate every credential and key before service resumes" (research/30-security-privacy.md:1461), a wider scope than section 3.12.
- Fix: section 3.9 ransomware paragraph, or section 3.12: list the credentials the rotation covers and the order of its steps.
- Severity note: HIGH, because a named step on the ransomware path has no steps at all.

### V-30-08-c  PARTLY   (first agent: MEDIUM; refuter: MEDIUM, for the part no named rule covers)
- Claim: the credential rotation changes credentials and names no audit event; `restore_check.recorded` covers only the verify, and phase 00 requires a log only for commands that change an installation.
- Refuted part: a general rule covers an akr-ctl command that changes an installation: docs/plan/phase-00.md:633 "Every command that changes an installation logs who ran it (the OS user and the PAM session id from the environment) into the audit sink."
- Stands part: phase 30 does not make the rotation an akr-ctl command, and operator-run credential changes sit outside that rule, because phase 00 issues the akr-ctl operator policies "per ceremony" (docs/plan/phase-00.md:600 to 601). No event is named for the rotation (docs/plan/phase-30.md:291, and docs/plan/phase-30.md:408 covers the verify only). The pattern exists elsewhere: docs/plan/phase-01.md:852 names "`service_client.rotated`".
- Fix: section 3.9 ransomware paragraph (docs/plan/phase-30.md:291): make the rotation an akr-ctl command, so the phase 00 rule logs it, or name its audit event.

### V-30-11-a  STANDS   (first agent: LOW; refuter: MEDIUM, a NOT NULL value is undetermined)
- Claim: the restore check queues "recon_run" without naming the phase 12 insert or the requested_by actor.
- Evidence: docs/plan/phase-12.md:640 "Insert `recon_runs` (status `queued`, `requested_by` the actor)"; docs/plan/phase-12.md:79 "UUID NOT NULL REFERENCES admin.users(id) ON DELETE RESTRICT", with the comment "the system principal for sweeps". Phase 30 names only the job: docs/plan/phase-30.md:288 "queues one phase 12 `recon_run` job per source and date (the job behind `POST /recon/runs`)". `grep -c 'requested_by' docs/plan/phase-30.md` gives 0.
- Fix: section 3.9 step 5 (docs/plan/phase-30.md:288): name the phase 12 insert, and the admin user or principal recorded as `requested_by` for a restore run.
- Severity note: MEDIUM, not LOW, because the NOT NULL column has no value for this caller, so the insert cannot be written as the text stands.

### V-30-11-b  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: phase 12 refuses a second active day run for the same source and date with 409 `run_active`; step 5 of section 3.9 does not say what happens on that refusal.
- Evidence: docs/plan/phase-12.md:641 "409 `E_NOT_ALLOWED_IN_STATE` reason `run_active` with the active run id in `errors[]`"; docs/plan/phase-30.md:288 "waits for their results". Phase 12's section 7.2 adds no rule for it: docs/plan/phase-12.md:1313 "No claim condition or ordering rule is added here." `grep -c 'run_active' docs/plan/phase-30.md` gives 0.
- Fix: section 3.9 step 5 (docs/plan/phase-30.md:288): say whether the check waits for the active run named in `errors[]` or fails the restore step.

### V-30-12-a  STANDS   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: "viewable by admin.system" has no route or screen that opens the content of the failover and restore reports.
- Evidence: docs/plan/phase-30.md:418 "Viewable by `admin.system`", while the header lists 0 routes (docs/plan/phase-30.md:8). The cockpit shows drills only as an age tile: docs/plan/phase-03.md:521 lists "backups and drills", and docs/plan/phase-03.md:971 "Backups: age, restore-drill age". docs/API.md:236 says "there is no generic evidence route (DR-380)", and evidence is "served by its owner", for example docs/plan/phase-18.md:596 "`/api/v1/cases/{id}/evidence/{evidence_id}/content`". Phase 30 names no owner route for the ops_drill evidence.
- Fix: section 3.9 or 3.18: name an owner-scoped route for the report content, as phase 18 does for case evidence, or state how the bank's operators read the report.

## Summary

- REFUTED: 1 (V-30-01-a)
- STANDS: 16
- PARTLY: 7
- Total: 24 claims, each Gap ID handled once. Severity of the 23 STANDS and PARTLY claims: HIGH 7, MEDIUM 10, LOW 6.
- Severity raised from the first agent: V-30-02-d, V-30-05-a, V-30-06-a, V-30-07-b and V-30-08-b (MEDIUM to HIGH); V-30-11-a (LOW to MEDIUM). Severity lowered: V-30-07-f (MEDIUM to LOW, PARTLY).
- Searched: docs/plan/phase-30.md in full; phase 00, 01, 02, 03, 05, 11, 12, 17, 18, 20, 21, 23, 25 and 30 by grep and reading; docs/API.md; research/30-security-privacy.md (SP-88). The cbs-discovery/ and test-poc/ folders were not read.

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-30-02-a | STANDS | MEDIUM | the writer's `nextval` has no sequence behind it | section 6.2 |
| V-30-02-b | STANDS | LOW | the "phase 00 stanza" name is not defined in phase 00 | phase 00 section 4 (pgbackrest row); section 3.4 criterion 5 |
| V-30-02-c | PARTLY | LOW | the citation to phase 00 pool options is wrong; the values are in this phase | section 3.4 pool paragraph |
| V-30-02-d | STANDS | HIGH | the winning condition is stated three different ways | section 3.4 decision rule; DR-320 |
| V-30-03-a | PARTLY | LOW | the citation says phase 00 section 3; the bundle step is in section 4 | section 3.12 first bullet |
| V-30-03-b | PARTLY | LOW | the citation says step (c); the chart check is check (g) | section 3.12 second bullet |
| V-30-05-a | STANDS | HIGH | the akr-restore namespace has no render, zone check or admission rule | sections 3.3 and 3.12 |
| V-30-05-b | PARTLY | LOW | probe image and connect timeout are not named | section 3.3 cluster zone check |
| V-30-06-a | STANDS | HIGH | a second governed key uses the CLI-only exemption that phase 00 limits to one key | section 4.5 or section 3.9 step 1 |
| V-30-06-b | STANDS | MEDIUM | null seed for a key that phase 00 does not mark nullable | section 4.5 |
| V-30-06-c | STANDS | MEDIUM | a passed drill with fewer than six cases is stored and audited | section 3.9 "Recording a drill" |
| V-30-07-a | PARTLY | HIGH | no method or authority for an automatic system-scope open that phase 11 defines | section 3.8 item 1 |
| V-30-07-b | STANDS | HIGH | the operator path gives no declared_by user and no comment | section 3.8 item 2 |
| V-30-07-c | STANDS | MEDIUM | the operator path emits blind_period.opened; phase 11 declarations emit blind_period.declared | section 3.8 item 2; section 3.17 row |
| V-30-07-d | STANDS | MEDIUM | no overlap rule for system periods | section 3.8, after item 2 |
| V-30-07-e | STANDS | MEDIUM | the CLI's database role and its grant are not named | section 3.8 item 2; section 4.4 |
| V-30-07-f | PARTLY | LOW | "phase 11 owns the rules" points at a phase 11 rule the item departs from | section 3.8 item 1 |
| V-30-08-a | STANDS | HIGH | the immutable or offline copy and the clean environment are not defined | section 3.9 |
| V-30-08-b | STANDS | HIGH | the credential rotation is named but has no steps | section 3.9 ransomware paragraph or section 3.12 |
| V-30-08-c | PARTLY | MEDIUM | the rotation has no audit event and is not an akr-ctl command | section 3.9 ransomware paragraph |
| V-30-11-a | STANDS | MEDIUM | the requested_by value for a restore run is not named | section 3.9 step 5 |
| V-30-11-b | STANDS | MEDIUM | the run_active refusal is not handled | section 3.9 step 5 |
| V-30-12-a | STANDS | MEDIUM | no route or screen shows the report content | section 3.9 or 3.18 |
