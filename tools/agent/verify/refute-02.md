# Refuter, phase 02 (Haiku draft for lead review)

### V-02-01-a  STANDS  (first agent: LOW; refuter: LOW)
- Claim: `AccessReceipt` is returned by `AccessScope::record` and required by `Accessed::new`, but no fields or constructor are given.
- Evidence: docs/plan/phase-02.md:637 "it can be built only from the `AccessReceipt` that `AccessScope::record`"; docs/plan/phase-02.md:730 "Returns an `AccessReceipt`; `Accessed::new(receipt, body)` is the only way to build the response type of a". Greps: `grep -n AccessReceipt docs/plan/*.md` finds only these two uses, and no definition in phase 02, phase 03 or phase 18. The construction rule is stated (only `AccessScope::record` returns it); its contents are not.
- Fix: phase 02, section 2 Extractors bullet: say what the receipt holds (for example the ids of the access events it covers), or that it is an opaque token only `record` can build.

### V-02-01-b  PARTLY  (first agent: LOW; refuter: LOW)
- Claim: The `cfg(test)` route `GET /api/v1/_test/sensitive/{id}` is used by the coverage test and by section 6, but is in no route table and has no scope and no Cedar action.
- Evidence: Refuted in part. The route is named in the section 2 entry labelled "Route table": docs/plan/phase-02.md:643 "router mounts `GET /api/v1/_test/sensitive/{id}` to exercise `AccessScope` (section 6)." Its registration is stated at docs/plan/phase-02.md:740 "In this phase only the `cfg(test)` route `GET /api/v1/_test/sensitive/{id}` is registered". Stands: the route has no row in the `| Method | Path | Scope | Description |` table, and no scope or Cedar action. docs/plan/phase-02.md:987 "**Cedar actions:** none; this phase adds no route." contradicts the registered test route. Greps: `grep -n "_test/sensitive" docs/plan/*.md docs/API.md` finds only phase-02.md:643, :740 and :1124.
- Fix: phase 02, section 2 Route table and section 3 "Cedar actions": give the test route one table row with its scope, or state that a test-only route carries none, and make the Cedar line agree.

### V-02-03-a  PARTLY  (first agent: LOW; refuter: LOW)
- Claim: No signature is given for the maintenance method, and no struct or trait exists for a `PartitionRegistry` entry (table, factory, ahead months).
- Evidence: Refuted for the entry. docs/plan/phase-02.md:1213 "Each entry has its own factory and `ahead_months = audit.partition_ahead_months`.", and the list at docs/plan/phase-02.md:1214-1219 names the tables and factories of each entry. Stands for the method: docs/plan/phase-02.md:813 "**`PartitionMaintenance`** (job `partition_maintenance`, daily): for each entry of `PartitionRegistry`" gives the steps but no method signature. Greps: `grep -n PartitionRegistry docs/plan/phase-02.md` finds lines 570, 813 and 1212 only, none of them a definition.
- Fix: phase 02, section 3 `PartitionMaintenance`: one line giving the method signature and its error type.

### V-02-03-b  PARTLY  (first agent: LOW; refuter: LOW)
- Claim: Partition maintenance says nothing about a failing factory call (stop the run, or continue with the other tables).
- Evidence: Refuted in part by the job harness. docs/plan/phase-00.md:543 "`JobError::Retryable` -> `status = 'queued'`" and docs/plan/phase-00.md:788 "a handler error backs off and reaches `dead`" make a handler error end the attempt and retry the job until `max_attempts`. docs/plan/phase-02.md:1220 "A missing month is created once by the writer and retried" backs up a missed month at write time. Stands: no sentence says whether the tables after a failing one still get their months in the same run. `grep -n -i "retr\|fail"` over docs/plan/phase-02.md lines 813-830 finds nothing about a maintenance failure.
- Fix: phase 02, section 3 `PartitionMaintenance`: one sentence on whether a failing table stops the run or the other tables still get their months.

### V-02-04-a  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: `restore_drill.recorded` has no detail keys, severity or SIEM flag; the four phase 00 kinds are listed without them, and phase 02 does not supply them.
- Evidence: docs/plan/phase-00.md:653 "`restore_drill.recorded` | `akr-ctl ops record-restore-drill` | setting `ops.last_restore_drill_at`" gives the trigger and the resource only; the other three phase 00 kinds in that table (lines 651-654) are the same. docs/plan/phase-02.md:1071 "phase 00 and 01 events persist; their kinds join the catalogue with detail keys, severity and SIEM flag" promises the values but gives none, and docs/plan/phase-02.md:960 counts "the 4 kinds of phase 00" in the catalogue without values. The rule that makes a per-phase audit table normative covers phases 03 to 31 only: docs/plan/phase-02.md:1225 "the audit section of each phase 03 to 31". Greps: `grep -n "restore_drill" docs/plan/*.md` (phase-00.md:630, :632, :653, :822; phase-02.md:1149) and `grep -n -i "siem\|severity\|detail_keys" docs/plan/phase-00.md docs/plan/phase-01.md` find no kind table that gives those values for the phase 00 kinds.
- Fix: phase 00, the "Audit events emitted" table (phase-00.md:647-654): add severity, SIEM flag and detail keys for its four kinds, or phase 02 section 4 gives them.

### V-02-05-a  PARTLY  (first agent: MEDIUM; refuter: LOW)
- Claim: `RenderJob` (template name and version, locale, direction, HTML, assets) and `RenderedPdf` (bytes and SHA-256) are not defined.
- Evidence: Refuted for `RenderedPdf`: docs/plan/phase-02.md:905 "Return the bytes and their SHA-256". Refuted in part for `RenderJob`: docs/plan/phase-02.md:898 "The caller renders HTML from a versioned template (minijinja, autoescape on) with `dir` and `lang` set" and docs/plan/phase-02.md:901 "(`index.html` plus assets)" name the inputs. The caller uses the template, so the job need not carry a template name or version. Stands: there is no field list for `RenderJob`. Greps: `grep -n "RenderJob\|RenderedPdf" docs/plan/*.md` finds only phase-02.md:630 and :897.
- Fix: phase 02, section 2 Ports: one sentence listing the fields of `RenderJob` and whether it carries the template name and version.

### V-02-05-b  STANDS  (first agent: LOW; refuter: LOW)
- Claim: No handling is given for `RenderError::Refused` and `RenderError::FontMissing` at the calling job (fail the job, or retry).
- Evidence: docs/plan/phase-02.md:904 "`Unavailable` or `Timeout` (the calling job retries)" is the only retry mapping. `Refused(String)` (docs/plan/phase-02.md:597) is raised by the URL refusal at docs/plan/phase-02.md:899 "refuses HTML that refers to any URL other than the bundled asset names". `FontMissing` (docs/plan/phase-02.md:598, :911) is raised only by the CI golden check. docs/plan/phase-00.md:543 defines `Retryable` and `Permanent` but does not assign these two errors to either. Greps: `grep -n "Refused\|FontMissing" docs/plan/phase-02.md` finds no job mapping.
- Fix: phase 02, section 3 `RenderService` step 3: one sentence on how the calling job treats each of the two errors.

### V-02-07-a  STANDS  (first agent: LOW; refuter: LOW)
- Claim: `ShredOutcome` is returned by `shred` but never defined (fields or variants).
- Evidence: docs/plan/phase-02.md:1202 "`shred(tx, evidence_id, reason) -> ShredOutcome`: erases `wrapped_dek` and sets `shredded_at` in the same transaction". The effects are stated; the returned value is not. docs/plan/phase-23.md:500 uses the name only. Greps: `grep -n ShredOutcome docs/plan/*.md` finds only these two lines.
- Fix: phase 02, section 7.3 `shred` bullet: one sentence naming what the outcome holds (for example the `shredded_at` instant and whether the object was already shredded).

### V-02-07-b  REFUTED  (first agent: LOW; refuter: none)
- Claim: The classification row is attributed to phase 23 "section 1.1"; it is in phase 23 section 1.2.
- Evidence: docs/plan/phase-23.md:189 "### 1.2 `0072_retention_guards_grants.sql`" and docs/plan/phase-23.md:449 "('evidence_objects', 'internal', FALSE, FALSE, ARRAY['object_deleted_at'])". The row exists in phase 23 under 1.2. Only the number is wrong, at docs/plan/phase-02.md:1210 ("the one phase 23 section 1.1 defines"). A name that differs only by a section number is not a gap.

### V-02-07-c  REFUTED  (first agent: LOW; refuter: none)
- Claim: 7.3 refers to "the trigger of this section" for the shorter-lock refusal; the trigger is in section 1.
- Evidence: docs/plan/phase-02.md:410 "RAISE EXCEPTION 'evidence %: lock_until can only be extended', OLD.id;" is the refusal, inside the section 1 guard trigger (docs/plan/phase-02.md:395-420). The refusal exists and is enforced by the database. Only the section reference at docs/plan/phase-02.md:1207 is off. A name that differs only by a section number is not a gap.

### V-02-09-a  STANDS  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The `case_event` anchor digest of case heads is described but has no step and no statement field; the statement list cannot carry it as written.
- Evidence: The content is designed, so the design is not what is missing. docs/plan/phase-02.md:1241 "The anchor for `case_event` carries, in its payload, the digest of the heads of the cases that changed since the"; DR-122 (docs/plan/research.md:133) "the `case_event` anchor carries a digest of the changed case heads"; docs/plan/phase-17.md:1119-1120 "This phase extends its payload with the heads of the cases that changed since the previous anchor". Nothing settles the step: the twelve steps under docs/plan/phase-02.md:743 are written for one chain per stream (docs/plan/phase-02.md:752 "Continuity: the first row's `prev_hash` must equal `prev_last_hash`"), and no step reads heads or computes a digest. The statement has no key for it: docs/plan/phase-02.md:759 "7. Statement = canonical JSON of `{anchor_id, cv, entry_count, first_entry_id, key_name, last_entry_hash,". Phase 02 section 1 says `case_event` is "chained and anchored by the same code" (docs/plan/phase-02.md:530-531), which the steps do not support. Phase 17 cites "phase 02 section 3, Open doubts" (docs/plan/phase-17.md:1121), and phase 02 has no such section. The sources also differ on the content: phase 17 names a list of heads, while DR-122 and docs/plan/phase-02.md:1241 say digest. Greps: `grep -n "digest\|heads" docs/plan/phase-02.md` finds line 1241 only.
- Fix: phase 02, section 3 `AnchorService::run`: one step for the case_event branch (how the changed heads are read and digested) and one statement key for the result, with the list-or-digest question settled.

### V-02-10-a  PARTLY  (first agent: MEDIUM; refuter: MEDIUM)
- Claim: The verifier's WORM checks (retain-until via HEAD, delete-marker check via version listing) need S3 permissions that the worker credential of section 4 does not list.
- Evidence: Refuted in part. The S3 guard passes the reads the verifier needs: docs/plan/phase-00.md:609 "`akr-s3guard` passes only: PUT and copy object, GET, HEAD, list object versions", and docs/plan/phase-00.md:610 "get and put retention and legal hold". The writer credential is restricted on deletes and bucket writes only: docs/plan/phase-00.md:618 "writer (no delete except quarantine, no bucket-level writes)". Stands: docs/plan/phase-02.md:1035 "`akr-worker` keeps the writer credential (GET and PUT on the three" does not name version listing or retention reads for `akr-anchors`, and docs/plan/phase-23.md:923 "the worker's identity gains `s3:ListBucketVersions`" grants version listing on `akr-evidence` under `ev/` only. Greps: `grep -n "GetObjectRetention\|ListBucketVersions\|ListObjectVersions" docs/plan/*.md` finds only phase-23.md:759 and :923, both for akr-evidence; there is no grant on `akr-anchors`.
- Fix: phase 02, section 4 "Store" bullet: name the version-listing and retention-read permissions on `akr-anchors` for the worker credential, or state that the phase 00 writer policy covers them.

### V-02-11-a  REFUTED  (first agent: MEDIUM; refuter: none)
- Claim: `VerifiedReader` (the return of `EvidenceStore::open`, a byte stream after verification) is not defined; phase 18 names it too.
- Evidence: docs/plan/phase-02.md:881 "`open(id) -> VerifiedReader` (akr-api for the phase 18 route; akr-worker for pack assembly):". Its contract is stated in the steps: docs/plan/phase-02.md:886 "every segment is authenticated before release; the SHA-256 accumulates; at the last segment it must equal the row's", and the same step says a failure aborts the stream, which is reset and never completed. DR-44 (docs/plan/research.md:55) says the bytes are "served by `akr-api` streaming, decrypting and checking the hash". A reader whose behaviour is stated in prose is not a gap.

### V-02-11-b  STANDS  (first agent: LOW; refuter: LOW)
- Claim: The evidence read route is named `GET /api/v1/evidence/{id}` (phase 18), but phase 18 has `GET /api/v1/cases/{id}/evidence/{evidence_id}/content`.
- Evidence: docs/plan/phase-18.md:596 "| GET | `/api/v1/cases/{id}/evidence/{evidence_id}/content` | `queue.work` | The verified bytes; access event first" is phase 18's route. The generic route was removed from the plan: DR-380 (docs/plan/research.md:391) lists "`GET /evidence/{id}`, the two backfill routes" among the API.md rows removed, and docs/API.md:236 says "there is no generic evidence route (DR-380)". Phase 02 still names the removed route: docs/plan/phase-02.md:641 "`GET /api/v1/evidence/{id}` is phase 18", and docs/plan/phase-02.md:1267 "`GET /evidence/{id}` (K3) is phase 18". Greps: phase 18 has no `/evidence/{id}` path.
- Fix: phase 02, section 2 Route table bullet and the first "Does Not Touch" bullet: replace the generic path with phase 18's content route.

### V-02-11-c  STANDS  (first agent: LOW; refuter: LOW)
- Claim: Repository row types `StagedRow`, `FinalRow`, `EvidenceOpenRow` and `NewPackRow` have no fields.
- Evidence: docs/plan/phase-02.md:616, :617, :619 and :622 use the four names without a field list. The columns can be read from the `admin.evidence_objects` DDL in section 1 and from the stage step at docs/plan/phase-02.md:852 "insert the row (`av_status = pending`, `file_name` sealed)"; `NewPackRow` is close to the `Pack` entity at docs/plan/phase-02.md:605. A developer can infer the columns, so the gap is minor. Greps: `grep -n "StagedRow\|FinalRow\|EvidenceOpenRow\|NewPackRow" docs/plan/*.md` finds only uses.
- Fix: phase 02, section 2 "Repository traits": one line per row type naming its columns, or saying it maps the whole table.

### V-02-11-d  STANDS  (first agent: LOW; refuter: LOW)
- Claim: Infected and rejected evidence objects are said to stay in quarantine under R12 "phase 23"; phase 23 does not assign that class to them.
- Evidence: docs/plan/phase-02.md:866 "The sealed object stays in quarantine for the security officer (retention R12, phase 23)." docs/plan/phase-23.md:602 gives R12 as age 30 to 90 days for database tables: "`kept` (`evidence_orphans`); other R12 tables by `maintenance_sweep` (phase 00)". No phase 23 row mentions infected or rejected objects (`grep -n -i "infected" docs/plan/phase-23.md` finds nothing). The behaviour is inferable as "kept", because docs/plan/phase-02.md:1022-1023 says the sweep "never deletes an infected or rejected object", but the cited class reads as a purge after 30 to 90 days.
- Fix: phase 23 retention table, or phase 02 `evidence_scan` step 5: name the class that holds infected and rejected objects, and make the two citations agree.

## Summary

Counts: REFUTED 3, STANDS 8, PARTLY 5 (16 claims; each Gap ID handled once).

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-02-01-a | STANDS | LOW | `AccessReceipt` has no contents | phase 02 s.2 Extractors |
| V-02-01-b | PARTLY | LOW | test route has no scope and no Cedar action; the Cedar line says no route is added | phase 02 s.2 Route table; s.3 Cedar actions |
| V-02-03-a | PARTLY | LOW | `PartitionMaintenance` has no method signature | phase 02 s.3 PartitionMaintenance |
| V-02-03-b | PARTLY | LOW | effect of a failing table on later tables not stated | phase 02 s.3 PartitionMaintenance |
| V-02-04-a | STANDS | MEDIUM | the four phase 00 kinds lack severity, SIEM flag and detail keys | phase 00 audit table (phase-00.md:647-654) |
| V-02-05-a | PARTLY | LOW | `RenderJob` has no field list | phase 02 s.2 Ports |
| V-02-05-b | STANDS | LOW | no job outcome given for `Refused` and `FontMissing` | phase 02 s.3 RenderService step 3 |
| V-02-07-a | STANDS | LOW | `ShredOutcome` has no contents | phase 02 s.7.3 shred bullet |
| V-02-09-a | STANDS | MEDIUM | no step computes the case_event heads digest; the statement has no key for it | phase 02 s.3 AnchorService::run |
| V-02-10-a | PARTLY | MEDIUM | version listing and retention reads on akr-anchors not named | phase 02 s.4 Store bullet |
| V-02-11-b | STANDS | LOW | stale route path `GET /api/v1/evidence/{id}` | phase 02 s.2 Route table; Does Not Touch |
| V-02-11-c | STANDS | LOW | four row types have no column list | phase 02 s.2 Repository traits |
| V-02-11-d | STANDS | LOW | infected and rejected objects cite R12, which does not cover them | phase 23 retention table, or phase 02 s.3 evidence_scan step 5 |
