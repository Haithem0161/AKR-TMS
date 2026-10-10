# Verification pass, phase 02 (Haiku draft for lead review)

Sample: 8 header items (SP-39, SP-11, TS-12, TS-39, RQ-11 from the "Requirements carried" list at start, middle and end; SP-40, VC-81, SP-58 from the same header). Phase 02 has no "Adversarial pass" blocks, so 3 extra header items were taken. Gap blocks used: 7.1, 7.3, 7.4, 7.7 (four different 7.x blocks). Section 1 greps: `grep -n` on docs/plan/phase-00.md, phase-01.md, phase-03.md, phase-04.md, phase-05.md, phase-06.md, phase-09.md, phase-10.md, phase-12.md, phase-13.md, phase-15.md, phase-16.md, phase-17.md, phase-18.md, phase-22.md, phase-23.md, phase-25.md, phase-29.md, docs/API.md, docs/MODULES.md, docs/plan/research.md.

### V-02-01 Access event writer, AccessScope and PgAccessLog (SP-39)   (source: header "Requirements carried"; result: GAPS 2)
- T1 SQL: PASS - `audit.access_event` with every column the logic uses (`actor_role`, `tier`, `purpose_code`, `reason`, `decision`, `correlation_id`) at phase-02.md:142-170; CHECK `tier = 'normal' OR decision <> 'allow' OR purpose_code IS NOT NULL` at phase-02.md:166.
- T2 Route: N/A - section 2 says "Route table: no routes" (phase-02.md:641). The only route named is the test route, see GAP b.
- T3 Repository and types: PASS for `AccessLog::record(events: Vec<AccessEvent>)` (phase-02.md:585-586 (`AccessLog::record`)) and `AccessEvent` fields (phase-02.md:566). `Access { resource_type, resource_id, subject_party, tier, action }` is given inline at phase-02.md:720. `AccessReceipt` is named at phase-02.md:637 and :730 but has no definition, see GAP a.
- T4 Logic: PASS - numbered steps 1-5 at phase-02.md:720-735; purpose refusal 422 `E_PURPOSE_REQUIRED` at phase-02.md:722; 503 `E_UNAVAILABLE` at phase-02.md:709-715 (`PgAccessLog::record` step 3); own transaction and lock at phase-02.md:712-714.
- T5 Audit: PASS - the mutation is the `access_event` insert, which is itself the record: "Access events are the `access_event` stream itself, not `audit_log` rows" (phase-02.md:982). Deny events for a missing purpose are named at phase-02.md:722.
- T6 Cross-references: PASS - `AuthContext` at phase-01.md:767; `RoleCode` at phase-01.md:424; purpose code list `admin.purpose_codes` at phase-03.md:689; Cedar decision feed at phase-03.md:715. Three checked.
- T7 Consistency: PASS - section 3 "Access events are always written in their own transaction" (phase-02.md:714) matches the header line "access events are written in their own transaction before the content is sent" (phase-02.md:1075).
- Gaps:
GAP V-02-01-a | LOW | T3 | `AccessReceipt` is returned by `AccessScope::record` and required by `Accessed::new`, but no fields or constructor are given | section 2 AccessScope bullet (phase-02.md:636-638) and section 3 AccessScope step 4 (phase-02.md:730) | greps: `grep -n AccessReceipt docs/plan/*.md` found only uses; no definition in phase-02, phase-03, phase-18
GAP V-02-01-b | LOW | T2 | The `cfg(test)` route `GET /api/v1/_test/sensitive/{id}` is used by the coverage test and by section 6 step 4 but is in no route table (no scope, no Cedar action) | section 2 Route table (phase-02.md:641-643) and section 6 access-log bullet (phase-02.md:1124) | greps: `grep -n "_test/sensitive" docs/plan/*.md docs/API.md` found only phase-02.md:643, :740 and :1124

### V-02-02 Signing key version recorded per anchor (SP-11)   (source: header "Requirements carried"; result: PASS)
- T1 SQL: PASS - `key_version INT NOT NULL CHECK (key_version > 0)` at phase-02.md:278; `key_name CHECK (key_name = 'akr-sign-anchor')` in the same table.
- T2 Route: N/A - no route in this phase; the key-version field is read by `audit keys export` (phase-02.md:935, `akr-ctl` additions).
- T3 Repository and types: PASS - the anchor insert uses the row struct of section 1; `AnchorError::OpenBao(String)` at phase-02.md:593.
- T4 Logic: PASS - step 8 "keep the version from `vault:vN:` as `key_version`" at phase-02.md:761; sign failure ends the run with `audit.anchor_failed {stage: sign}` at phase-02.md:761-762.
- T5 Audit: PASS - `audit.signing_key_published {key_version, fingerprint}` at phase-02.md:765 and in the events table at phase-02.md:969.
- T6 Cross-references: PASS - transit key `akr-sign-anchor` defined at phase-00.md:590; "sign on the four signing keys" AppRole line at phase-00.md:598 and the update at phase-00.md:925 (consistent with the phase 02 wording); `keys/...` PUT covered by the writer credential of phase-02.md:1035. Three checked.
- T7 Consistency: PASS - no 7.x block changes this item.

### V-02-03 Partition maintenance (TS-12)   (source: header "Requirements carried"; result: GAPS 2)
- T1 SQL: PASS - setting `audit.partition_ahead_months` seeded at phase-02.md:486; the partitioned parents `audit.audit_log`, `audit.access_event` at phase-02.md:130 and :167 (`PARTITION BY RANGE`); factory `audit.create_partition` at phase-02.md:206.
- T2 Route: N/A - job only (section 4 job table).
- T3 Repository and types: GAP for the method. `PartitionMaintenance` is named at phase-02.md:632 and :813 with no method signature, and `PartitionRegistry` has no entry type (only "each entry has its own factory", phase-02.md:1213), see GAP a.
- T4 Logic: prose, not numbered steps. Steps are readable (loop over registry, create months ahead, emit, gauge, alert), but failure handling is not stated, see GAP b.
- T5 Audit: PASS - `audit.partition_created {table, partition, inline: false}` at phase-02.md:815 and in the events table at phase-02.md:964.
- T6 Cross-references: PASS - each registry entry verified against its owning phase: canon factory at phase-10.md:859 (ten tables, phase-10.md:1531); feat at phase-13.md:225 with the same three tables as phase-13.md:235; kyc at phase-15.md:179; scn at phase-16.md:378; alert at phase-17.md:237 (`alert.alerts` partitioned at phase-17.md:172); assess at phase-29.md:127. Schedule `schedule.partition_maintenance` seeded at phase-02.md:498.
- T7 Consistency: PASS - section 4 time "daily 01:15" (phase-02.md:1016) matches the seed "0 15 1 * * *" (phase-02.md:498).
- Gaps:
GAP V-02-03-a | LOW | T3 | No signature for the maintenance method and no struct or trait for a `PartitionRegistry` entry (table, factory, ahead months) | section 2 "partition" module bullet (phase-02.md:569-571) and section 3 `PartitionMaintenance` (phase-02.md:813) | greps: `grep -n "PartitionRegistry" docs/plan/phase-02.md` gives no definition; phase-10.md:2169 and phase-13.md:937 only say "registers"
GAP V-02-03-b | LOW | T4 | Partition maintenance has no stated behaviour when one factory call fails (stop the run, or continue with the other tables) | section 3 `PartitionMaintenance` (phase-02.md:813-817) | grep "retry\|fails" near :813-820 found nothing

### V-02-04 Restore-drill record persists through the audit sink (TS-39)   (source: header "Requirements carried"; result: GAPS 1)
- T1 SQL: PASS - setting `ops.last_restore_drill_at` at phase-00.md:264 (the setting, not a phase 02 table).
- T2 Route: N/A - CLI `akr-ctl ops record-restore-drill` (phase-00.md:630), owned by phase 00.
- T3 Repository and types: N/A - the CLI calls `SettingsService::apply` (phase-00.md:630); no phase 02 type needed.
- T4 Logic: PASS - phase-00.md:630-632 writes the setting and emits `restore_drill.recorded`; phase 02 changes only the sink, `PgAuditSink::emit_standalone` (phase-02.md:684) with `RequestCtx::cli` (phase-02.md:561).
- T5 Audit: GAP - the kind is named (phase-00.md:653) but the table row has only "On" and "Resource". No detail keys, severity or SIEM flag exist for it anywhere in phase 00, phase 01 or phase 02, although phase-02.md:982 says "Every kind is in the catalogue with its detail keys, severity and SIEM flag", see GAP a.
- T6 Cross-references: PASS - `--evidence <file>` (phase-00.md:630) and the drill evidence step in phase-02.md:1147-1149; `ops_drill` owner kind registered by phase 30 (grep: phase-30.md contains `ops_drill`).
- T7 Consistency: PASS - phase 00 says the kind is "persisted from phase 02" (phase-00.md:632), and phase 02 step 7 expects it in `audit.audit_log` (phase-02.md:1149). Consistent.
- Gaps:
GAP V-02-04-a | MEDIUM | T5 | `restore_drill.recorded` has no detail keys, severity or SIEM flag; the phase-00 kinds are listed without them, and phase 02 does not supply them | phase 02 section 3 catalogue, or the phase 00 audit table (phase-00.md:650-655) | greps: `grep -n "restore_drill" docs/plan/*.md` found only phase-00.md:630, :632, :653, :821-822 and phase-02.md:1149; `grep -n "severity\|SIEM" phase-00.md phase-01.md` found no kind table with those columns

### V-02-05 Gotenberg render service and Arabic shaping check (RQ-11)   (source: header "Requirements carried"; result: GAPS 2)
- T1 SQL: N/A - no table; setting `render.timeout_s` at phase-02.md:522.
- T2 Route: N/A - no route; the service is called by workers.
- T3 Repository and types: GAP - `RenderService::render(RenderJob) -> RenderedPdf` (phase-02.md:897) and the port `PdfRenderer::render(RenderJob) -> Bytes` (phase-02.md:630) name `RenderJob`, and `RenderedPdf` is named but not defined. The step text gives some fields (`dir`, `lang`, the template, assets) but no struct, see GAP a. `RenderError` is defined (phase-02.md:597-598).
- T4 Logic: PASS for steps 1-4 at phase-02.md:897-914 (HTML pre-check, Gotenberg options, PDF body check, return with SHA-256). Error mapping: `Unavailable` and `Timeout` (phase-02.md:904); `Refused` and `FontMissing` have no stated handling, see GAP b.
- T5 Audit: N/A - the service writes nothing. Its output is stored through `put_generated`, which emits `evidence.stored` (phase-02.md:876-878).
- T6 Cross-references: PASS - Gotenberg image "with IBM Plex fonts (RQ-11)" at phase-00.md:702; `akr-worker -> gotenberg:3000` edge at phase-00.md:939; compose "Now used" at phase-02.md:1024-1030. Three checked.
- T7 Consistency: PASS - the HTML pre-check at phase-02.md:899-900 agrees with the Chromium deny list at phase-02.md:1028.
- Gaps:
GAP V-02-05-a | MEDIUM | T3 | `RenderJob` (its fields: template name and version, locale, direction, HTML, assets) and `RenderedPdf` (bytes and SHA-256) are not defined | section 2 "Ports" (phase-02.md:630) and section 3 `RenderService` (phase-02.md:897) | greps: `grep -n "RenderJob\|RenderedPdf" docs/plan/*.md` found only phase-02.md:630, :897
GAP V-02-05-b | LOW | T4 | No handling is given for `RenderError::Refused` and `RenderError::FontMissing` at the calling job (fail the job, or retry) | section 3 `RenderService` step 3 (phase-02.md:904-905) and the golden test (phase-02.md:911-912) | grep "Refused" in phase-02.md found only :597, :598 and no mapping

### V-02-06 Evidence owner registry (7.1)   (source: 7.1; result: PASS)
- T1 SQL: N/A - a code registry; the kind is stored in `admin.evidence_objects.owner_kind`, CHECK `owner_kind ~ '^[a-z][a-z_]*$'` at phase-02.md:356.
- T2 Route: N/A.
- T3 Repository and types: N/A - `stage` step 3 names the registry (phase-02.md:848).
- T4 Logic: PASS - registry refusal `OwnerUnknown { kind }` at phase-02.md:848 and in the error enum at phase-02.md:597.
- T5 Audit: N/A - no mutation.
- T6 Cross-references: checked 9 rows of the table, all PASS: `source` (phase-05.md:751, R9); `gap_report_run` (phase-06.md:853); `fx_rate` (phase-09.md:1082); `rfi_input` (phase-22.md:486, R6); `alert` (phase-17.md:973); `case` (phase-18.md:1137); `coverage_certificate` (phase-12.md:889, R6); `change_request` (phase-04.md:1652, the 7.2 block); `release` (phase-25.md:897, R10). Rows not checked: `goaml_profile`, `network_snapshot`, `ops_task`, `report` family, `legal_hold`, `islamic_deal`, `model_version`, `assess_extract`, `ops_drill` (only the presence of `ops_drill` in phase-30.md was grepped).
- T7 Consistency: PASS - 7.10 adds `change_request` and `release`; both are present in the table's source phases.

### V-02-07 Disposal methods and the deletion column (7.3)   (source: 7.3; result: GAPS 3)
- T1 SQL: PASS - `object_deleted_at TIMESTAMPTZ` added by phase 23 at phase-23.md:255 (migration 0072, phase-23.md:189); the column named in 7.3 at phase-02.md:1209 is the same column.
- T2 Route: N/A.
- T3 Repository and types: GAP for the return type. `shred(...) -> ShredOutcome` (phase-02.md:1202) names `ShredOutcome`, which is defined nowhere (phase 23 uses it at phase-23.md:500 without a definition). `delete_object` and `extend_lock` have signatures (phase-02.md:1205, :1207), see GAP a.
- T4 Logic: PASS - each method is one sentence with its precondition (`lock_until` passed, later date only). Lock and transaction: `shred` and `extend_lock` take `tx`; `delete_object` does not, and no transaction is needed for an object store call.
- T5 Audit: PASS with deferral - `shred` emits `retention.evidence_shredded` (phase-02.md:1202, and phase-23.md:810); `delete_object` emits `retention.object_deleted` (phase-23.md:811); `extend_lock` emits `evidence.lock_extended` (phase-23.md:815). Deferred to phase 23 (found at phase-23.md:810-815).
- T6 Cross-references: GAP b. The classification row is said to be "the one phase 23 section 1.1 defines" (phase-02.md:1210); phase 23 places it in section 1.2 (phase-23.md:189 starts 1.2; the row is at phase-23.md:449).
- T7 Consistency: GAP c. 7.3 says "the trigger of this section refuses a shorter one" (phase-02.md:1207). The trigger is in section 1 (`admin.evidence_objects_guard`, phase-02.md:395-420); 7.3 has no trigger.
- Gaps:
GAP V-02-07-a | LOW | T3 | `ShredOutcome` is returned by `shred` but never defined (fields or variants) | 7.3 (phase-02.md:1202); phase 23 uses it at phase-23.md:500 | greps: `grep -n "ShredOutcome" docs/plan/*.md` found only uses
GAP V-02-07-b | LOW | T6 | The classification row is attributed to phase 23 "section 1.1"; it is in phase 23 section 1.2 | 7.3 DDL bullet (phase-02.md:1209-1210) | phase-23.md:189 (1.2 heading) and :449
GAP V-02-07-c | LOW | T7 | 7.3 refers to "the trigger of this section" for the shorter-lock refusal; the trigger is in section 1 | 7.3 `extend_lock` bullet (phase-02.md:1207) | phase-02.md:395-420

### V-02-08 PartitionRegistry entries (7.4)   (source: 7.4; result: PASS)
- T1 SQL: PASS - each listed table is partitioned in its owner: `feat.feature_values` (phase-13.md:172 onward), `kyc.ratings` (phase-15.md:145), `scn.replay_results` (phase-16.md:252, `PARTITION BY` at :270), `scn.shadow_results` (phase-16.md:309), `alert.alerts` (phase-17.md:106, `PARTITION BY` at :172), `assess.decisions` (phase-29.md:85).
- T2 Route: N/A.
- T3 Repository and types: N/A - factories are SQL functions; their signatures are in the owners.
- T4 Logic: PASS - the retry on SQLSTATE 23514 is stated (phase-02.md:1220-1221) and matches the writer rule at phase-02.md:674-676.
- T5 Audit: PASS - `audit.partition_created` is reused for every entry (phase-02.md:815, phase-10.md:2072).
- T6 Cross-references: PASS - factories verified: `canon.create_partition` phase-10.md:859; `feat.create_partition` phase-13.md:225 (its table list is phase-13.md:235, the same three tables); `kyc.create_partition` phase-15.md:179; `scn.create_partition` phase-16.md:378; `alert.create_partition` phase-17.md:237; `assess.create_partition` phase-29.md:127 (takes year and month only, no table name, which matches its single table). Five cross-references checked.
- T7 Consistency: PASS - "There is no DEFAULT partition" (phase-02.md:1221) matches phase-10.md:2171 ("no DEFAULT partition is created").

### V-02-09 case_event: one chain per case (7.7)   (source: 7.7; result: GAPS 1)
- T1 SQL: PASS - the `case_event` value in the anchor `stream` CHECK at phase-02.md:264-266 (the CHECK lists `case_event`); the per-case table is `casework.case_events` with `prev_hash` at phase-17.md:335-343.
- T2 Route: N/A.
- T3 Repository and types: N/A - the chain code is in phase 17 (`CaseChain`, phase-17.md:1106).
- T4 Logic: GAP. The anchor algorithm (section 3, steps 1-12) reads rows of the stream and signs a statement whose fields are fixed (phase-02.md:759, `{anchor_id, cv, entry_count, first_entry_id, key_name, last_entry_hash, last_entry_id, merkle_root, partition_name, prev_anchor_id, prev_merkle_root, range_from, range_to, stream}`). 7.7 says the `case_event` anchor "carries, in its payload, the digest of the heads of the cases that changed" (phase-02.md:1241-1242). No step and no statement field adds that digest, see GAP a.
- T5 Audit: PASS - case events are emitted by phase 17 (`case.alert_attached` at phase-17.md:904).
- T6 Cross-references: PASS - DR-122 at research.md:133; phase 17 section 3.7 at phase-17.md:1106 and :1957. Three checked.
- T7 Consistency: PASS - the base text "Each stream (`audit_log`, `access_event`; `case_event` from phase 17) is one hash chain" (phase-02.md:648) is replaced by 7.7 ("chained per case, not as one chain", phase-02.md:1239), which says what it replaces. One genesis per stream (phase-02.md:286) still holds, since the anchor chain stays per stream.
- Gaps:
GAP V-02-09-a | MEDIUM | T4 | The `case_event` anchor digest of case heads is described but has no step and no statement field; the statement list cannot carry it as written | section 3 `AnchorService::run` step 4 (phase-02.md:749) and step 7 (phase-02.md:759), or 7.7 (phase-02.md:1241-1242) | greps: `grep -n "digest\|heads" docs/plan/phase-02.md` found only :1243-1244; phase-17.md:1957 refers to it without a field name

### V-02-10 Verifier and WORM checks (SP-40)   (source: header "Implemented here"; result: GAPS 1)
- T1 SQL: PASS - every column read by the verifier exists: `statement_sha256`, `signature`, `key_version`, `object_key`, `object_version_id`, `anchored_at` (phase-02.md:263-285); the verifier reads `audit_log` and `access_event` as `akr_readonly`, granted at phase-02.md:311.
- T2 Route: N/A - CLI and job; the verify routes are phase 24 (phase-02.md:1266).
- T3 Repository and types: PASS - `Verifier::run(scope)` (phase-02.md:780), `VerifyReport` (phase-02.md:570) and the report shape at phase-02.md:804-805. The type of `scope` is inferred from the CLI flags (phase-02.md:933).
- T4 Logic: PASS - numbered steps 1-5 at phase-02.md:782-805; the findings and their severity are listed at phase-02.md:801-803; the pass rule is at phase-02.md:804.
- T5 Audit: PASS - `audit.verify_completed` at phase-02.md:804 and in the table at phase-02.md:970; chain findings also emit `audit.chain_break_detected` (phase-02.md:752-754 and the table at :977).
- T6 Cross-references: GAP. Step 3 needs a HEAD that shows COMPLIANCE and the retain-until date (`GetObjectRetention`), and a listing of versions to see a delete marker (`ListBucketVersions`), on `akr-anchors` and the published keys. The writer credential in section 4 grants only "GET and PUT on the three buckets, DELETE in akr-quarantine only, PUT retention" (phase-02.md:1035-1036). Phase 23 adds `ListBucketVersions` only on `akr-evidence` (phase-23.md:923). KV `db/akr_readonly` for the verifier pool is defined in phase 00 (cited in phase-02.md:1039). Three checked.
- T7 Consistency: PASS - the header "a critical finding pages as P1" matches phase-02.md:804-805 and the alert list at phase-02.md:1062.
- Gaps:
GAP V-02-10-a | MEDIUM | T6 | The verifier's WORM checks (retain-until via HEAD, delete-marker check via version listing) need S3 permissions that the worker credential of section 4 does not list | section 4 "Store" bullet (phase-02.md:1035-1036), or section 3 step 3 (phase-02.md:793-795) | greps: `grep -n "GetObjectRetention\|ListBucketVersions\|ListObjectVersions" docs/plan/*.md` found only phase-23.md:759 and :923 (for akr-evidence)

### V-02-11 Evidence vault: staging, scan, store, open (VC-81)   (source: header "Implemented here"; result: GAPS 4)
- T1 SQL: PASS - `admin.evidence_objects` at phase-02.md:353-391 with `av_status` enum from `admin.av_status` (phase-02.md:350, DO block); `admin.packs` at phase-02.md:423; `admin.data_classification` from phase-00.md:287-288; `admin.users` for `created_by` from phase-01.md:52.
- T2 Route: GAP b. Section 2 says "`GET /api/v1/evidence/{id}` is phase 18 (it calls `EvidenceStore::open`)" (phase-02.md:641) and the "Does Not Touch" list repeats it (phase-02.md:1266). Phase 18 defines no such path; its content route is `GET /api/v1/cases/{id}/evidence/{evidence_id}/content`, scope `queue.work` (phase-18.md:596).
- T3 Repository and types: GAP a for `VerifiedReader` (returned by `open` at phase-02.md:881, and used at phase-18.md:702) which has no definition. GAP c for `StagedRow`, `FinalRow`, `EvidenceOpenRow` and `NewPackRow` (named at phase-02.md:616-624) with no fields; `CleanFacts` is given (phase-02.md:619-620). `NewEvidence`, `GeneratedEvidence`, `NewPack` are given (phase-02.md:611).
- T4 Logic: PASS - `stage` steps 1-6 (phase-02.md:839-869), `evidence_scan` steps 1-6 (phase-02.md:856-869), `put_generated` (phase-02.md:876-878), `open` steps 1-5 (phase-02.md:881-895). Errors mapped at phase-02.md:953-957. Lock and conditional updates: `WHERE av_status = 'pending'` (phase-02.md:624-626).
- T5 Audit: PASS - `evidence.staged` (phase-02.md:976), `evidence.stored` (phase-02.md:977), `evidence.rejected` (963), `evidence.hash_mismatch` (964 area, and in `open` step 4 at phase-02.md:890); the access event in the route is in `AccessScope` (phase-02.md:720).
- T6 Cross-references: GAP d. `evidence_scan` step 5 keeps infected and rejected objects in quarantine "(retention R12, phase 23)" (phase-02.md:866). Phase 23 assigns R12 to the orphan register and to tables that a sweep handles (phase-23.md:602 and :463), and does not name quarantined evidence objects. Other cross-references PASS: `akr-kek` transit key (phase-00.md:590); `evidence` DEK family (phase-00.md:593); owner kinds (V-02-06); `retention.evidence_shredded` (phase-23.md:810); `clamd` and Gotenberg (phase-00.md:702-703).
- T7 Consistency: PASS - `shredded_at`, `wrapped_dek` and lock rules in section 1 agree with 7.3 and with phase 23 section 3.6 (phase-23.md:664-676).
- Gaps:
GAP V-02-11-a | MEDIUM | T3 | `VerifiedReader` (the return of `EvidenceStore::open`, a byte stream after verification) is not defined; phase 18 names it too | section 2 "Services" or section 3 `open` (phase-02.md:881) | greps: `grep -n "VerifiedReader" docs/plan/*.md` found only phase-02.md:881 and phase-18.md:702
GAP V-02-11-b | LOW | T2 | The evidence read route is named `GET /api/v1/evidence/{id}` (phase 18), but phase 18 has `GET /api/v1/cases/{id}/evidence/{evidence_id}/content` | section 2 Route table (phase-02.md:641) and "Does Not Touch" (phase-02.md:1266) | phase-18.md:596 and :697 (no `/evidence/{id}` path in phase 18)
GAP V-02-11-c | LOW | T3 | Repository row types `StagedRow`, `FinalRow`, `EvidenceOpenRow`, `NewPackRow` have no fields | section 2 `EvidenceRepository` and `PackRepository` (phase-02.md:616-624) | greps: `grep -n "StagedRow\|FinalRow\|EvidenceOpenRow\|NewPackRow" docs/plan/*.md` found only uses
GAP V-02-11-d | LOW | T6 | Infected and rejected evidence objects are said to stay in quarantine under R12 "phase 23"; phase 23 does not assign that class to them | `evidence_scan` step 5 (phase-02.md:866) | phase-23.md:602 (R12 row), :463 (orphan register only)

### V-02-12 Evidence lock at the class floor (SP-58)   (source: header "Requirements carried"; result: PASS)
- T1 SQL: PASS - `evidence.lock_days` seeded with `floor` JSON and `governed = TRUE` at phase-02.md:510-516; `lock_until`, `lock_mode` columns at phase-02.md:372-373; floor CHECK `floor IS NULL OR governed` in phase-00.md:214 satisfied.
- T2 Route: N/A.
- T3 Repository and types: N/A.
- T4 Logic: PASS - stage step 3 refuses a class with no entry (`NoLockPolicy`, phase-02.md:848); scan step 6 computes `now + evidence.lock_days[class]` (phase-02.md:869-870); the registry refusal `E_BELOW_FLOOR` (phase-02.md:534-535 and :949-950).
- T5 Audit: PASS - `evidence.stored {sha256, size_bytes, lock_mode, lock_until, av_engine_version}` (phase-02.md:867-869, table :962).
- T6 Cross-references: PASS - M-05 is cited in MODULES.md (line 148, the requirement list of the module); extension at closure and holds is in phase 23 section 3.6 (phase-23.md:664-676) and `EvidenceShredder::extend_lock` at phase-23.md:675. Three checked.
- T7 Consistency: PASS - the floor applies to R5 to R8 (phase-02.md:511-512) and the prose says "the four classes M-05 names" (phase-02.md:534), which match.


## Summary

- Items checked: 12 (8 header items, 4 gap-pass items from blocks 7.1, 7.3, 7.4, 7.7; no adversarial blocks exist in phase 02).
- Items fully PASS: 4 (V-02-02 SP-11, V-02-06 owner registry 7.1, V-02-08 partition registry 7.4, V-02-12 SP-58 lock floor).
- GAP lines: 16 in total. HIGH 0, MEDIUM 5, LOW 11.
- Items with GAPS: V-02-01 (2), V-02-03 (2), V-02-04 (1), V-02-05 (2), V-02-07 (3), V-02-09 (1), V-02-10 (1), V-02-11 (4).
- Not checked in depth: 7.1 rows other than the nine listed in V-02-06; the phase 10 canon table list (only the count "ten" was checked); phase 01 and phase 00 kind tables beyond the four phase 00 kinds named in phase 00 section 9 of the event table.

| Gap ID | Severity | Truth | Missing | Where |
|-|-|-|-|-|
| V-02-01-a | LOW | T3 | `AccessReceipt` has no fields or constructor | section 2 AccessScope (phase-02.md:636-638), section 3 step 4 (:730) |
| V-02-01-b | LOW | T2 | Test route `GET /api/v1/_test/sensitive/{id}` is in no route table | section 2 Route table (:641-643); section 6 (:1124) |
| V-02-03-a | LOW | T3 | No signature for the maintenance method; no `PartitionRegistry` entry type | section 2 partition bullet (:569-571); section 3 (:813) |
| V-02-03-b | LOW | T4 | No stated behaviour when one factory call fails | section 3 PartitionMaintenance (:813-817) |
| V-02-04-a | MEDIUM | T5 | `restore_drill.recorded` has no detail keys, severity or SIEM flag anywhere | phase 02 section 3 catalogue (:982), or phase-00.md:650-655 |
| V-02-05-a | MEDIUM | T3 | `RenderJob` and `RenderedPdf` not defined | section 2 Ports (:630); section 3 RenderService (:897) |
| V-02-05-b | LOW | T4 | No handling of `RenderError::Refused` and `FontMissing` at the calling job | section 3 RenderService step 3 (:904) |
| V-02-07-a | LOW | T3 | `ShredOutcome` not defined | 7.3 (:1202); phase-23.md:500 |
| V-02-07-b | LOW | T6 | Classification row attributed to phase 23 "section 1.1"; it is in 1.2 | 7.3 DDL bullet (:1209-1210); phase-23.md:189, :449 |
| V-02-07-c | LOW | T7 | "the trigger of this section" refers to a trigger that is in section 1 | 7.3 extend_lock (:1207); section 1 (:395-420) |
| V-02-09-a | MEDIUM | T4 | `case_event` anchor digest has no step and no statement field | section 3 AnchorService step 4 (:749) and step 7 (:759); 7.7 (:1241-1242) |
| V-02-10-a | MEDIUM | T6 | Verifier needs GetObjectRetention and ListBucketVersions on the anchor bucket; the writer credential does not list them | section 4 Store (:1035-1036); section 3 step 3 (:793) |
| V-02-11-a | MEDIUM | T3 | `VerifiedReader` (return of `EvidenceStore::open`) not defined | section 3 open (:881); phase-18.md:702 |
| V-02-11-b | LOW | T2 | Evidence read route named `GET /api/v1/evidence/{id}`; phase 18 has `/cases/{id}/evidence/{evidence_id}/content` | section 2 Route table (:641); Does Not Touch (:1266); phase-18.md:596 |
| V-02-11-c | LOW | T3 | `StagedRow`, `FinalRow`, `EvidenceOpenRow`, `NewPackRow` have no fields | section 2 repositories (:616-624) |
| V-02-11-d | LOW | T6 | Infected or rejected objects assigned R12 "phase 23", which does not assign that class to them | evidence_scan step 5 (:866); phase-23.md:602, :463 |
