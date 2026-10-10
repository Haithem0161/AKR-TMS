# Refuter, phase 05 (Haiku draft for lead review)

### V-05-06-a  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: no evidence method is named for storing the signed check report; 7.2 says only "stores it as an evidence object".
- Evidence: phase-02.md:876 "`put_generated(tx, GeneratedEvidence, actor) -> EvidenceObject` (akr-worker; render output, snapshots, certificates," (phase 02 owns the worker's write method and names certificates in its scope). phase-05.md:815 "for akr-worker (it renders the check as canonical JSON, signs it with `akr-sign-certificate`, stores" (the seal job runs on the worker and produces a signed certificate). phase-02.md:355 "CHECK (kind IN ('upload', 'render', 'snapshot', 'package', 'certificate', 'pack'))" (the certificate evidence kind exists).

### V-05-06-b  STANDS   (first agent: MEDIUM; refuter: HIGH)
- Claim: base 3.6 names `EvidenceStore::stage` for a worker job; phase 02 gives `stage` to akr-api uploads and `put_generated` to akr-worker; 7.2 does not override.
- Evidence: phase-05.md:1141 "stores it through `EvidenceStore::stage` with owner kind `source`," (the worker writes the run report; the step starts at phase-05.md:1140). phase-02.md:839 "(akr-api; untrusted bytes from the upload routes of phases 17 and" against phase-02.md:876 "(akr-worker; render output, snapshots, certificates," (the generated-output path). phase-02.md:1050 gives the worker the full EvidenceStore, so the conflict is in the write path (quarantine and later scan, against an inline scan and a clean insert), not in permissions. Grep checked: `grep -n "stage\|put_generated\|EvidenceStore" docs/plan/phase-05.md` shows no line in 7.1 to 7.10 that names either method.
- Severity reasoning: HIGH under the rule that two texts contradict. Both texts name a method for the same worker write, and no 7.x block says which wins. A developer who follows 3.6 literally writes the report through the path that phase 02 describes for akr-api uploads. The first agent's MEDIUM understates this, because the worker's own text is the one that gets built.
- Fix: phase 05, section 3.6 (lines 1140-1141) or section 7.2: one sentence naming the EvidenceStore method the worker uses for the run report, and why.

### V-05-06-c  REFUTED   (first agent: MEDIUM; refuter: none)
- Claim: the check branch of the seal job (skip when evidence_object_id is set, render, sign, store, write the id) is prose, not numbered steps.
- Evidence: phase-05.md:1512 "the grant check as canonical JSON, signs it with `akr-sign-certificate`, stores it as an evidence object owned by the" and phase-05.md:1513 "(a check with it set is skipped)". Together they name the check, the skip rule and the order in one sentence, which the refute rules do not count as a gap. Failure handling is the job harness: phase-00.md:544 "`attempts >= max_attempts` or `JobError::Permanent` ->". The store call is `put_generated(tx, ...)` (phase-02.md:876), which takes the job's transaction, so the id write can share it.

### V-05-08-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the 7.8 listener reload has no numbered steps for the full read, the rebuild of each LoadGovernor, or the handling of permits in use.
- Refuted part (full read and per-source rebuild): phase-05.md:1563-1564 "rebuilds the `LoadGovernor` of every source from one full read of the state, budget and access rows before it waits again" names the read, the rebuild and the order in one sentence. The trigger sits in the base text at phase-05.md:941.
- Stands (permits in use during a rebuild): Grep checked: `grep -n -i "permit\|in use\|in-flight" docs/plan/phase-05.md` finds the acquire signature, step 3 and step 5 (lines 944, 952, 954, 959), the kill switch at line 983, an error-text hit at line 826 and Cedar hits at lines 1284 and 1292, and nothing about a rebuild. The one permit-release rule, phase-05.md:983 "`CancelHandle` of every permit out", belongs to the kill switch (a halt), not to a rebuild. Phases 10 and 11 use LoadGovernor for admission only (phase-10.md:1945, phase-11.md:769). The session rule, phase-05.md:954 "a `tokio::sync::Semaphore` of `max_sessions` permits", implies that the rebuilt governor must count sessions still open, but no text says so.
- Severity reasoning: MEDIUM. The session count can be inferred from the semaphore rule, so a developer is not blocked, but the corner case is not stated.
- Fix: phase 05, section 7.8: one sentence on how permits already out at a rebuild count against the rebuilt governor's session and rate gates.

### V-05-02-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: VD-02 is listed as implemented in phase 05, but the mapping-derivation steps and the change-control step are not written there; the owner is phase 10.
- Refuted part (change control): phase-10.md:116 "CHECK (status NOT IN ('approved', 'published') OR change_request_id IS NOT NULL)," makes a change request mandatory before a mapping version is approved or published, so the change-control step is designed in phase 10. Phase 05 also names phase 10 as the home of mapping authoring: phase-05.md:1115 "the mapping author needs to write a code mapping in phase 10."
- Stands (derivation): VD-02 asks to derive each mapping "from the bank's own DDL, column comments and a sample extract" (research/17-vendor-cbs-native-aml.md:453). Phase 05 reads only the inputs: the comment catalogue (phase-05.md:1045 "`ALL_TAB_COMMENTS`, `ALL_COL_COMMENTS`") and a candidate column taken from a hint (phase-05.md:1158 "Candidate column per attribute: the steward's hint"). Phase 10 has no derivation step; its DDL runs the other way, as the view DDL for the bank's DBA (phase-10.md:1760). Grep checked: `grep -n -i "DDL\|comment\|derive" docs/plan/phase-10.md` finds no derivation from DDL or comments. `grep -n "VD-02" docs/plan/*.md` names VD-02 only at phase-05.md:36, the gap-pass line in the header.
- Severity reasoning: MEDIUM. A developer can build the mapping workbench from phase 10, but the derivation itself is stated nowhere, and an ID-based coverage check would credit phase 05 for it.
- Fix: phase 10, section 3 (mapping authoring): one sentence that names VD-02, its inputs from phase 05 and the author's derivation steps; and phase 05 line 36 narrowed to the inputs that phase 05 reads.

### V-05-11-a  PARTLY   (first agent: MEDIUM; refuter: MEDIUM)
- Claim: the coverage gap (phase 11 blind_periods, phase 12) and the backfill (phase 11 ingest_backfill) are not built from admin.source_pauses, though phase 05 says they are; nothing links a pause row to a blind period row.
- Refuted part (backfill): Grep checked: `grep -n "ingest_backfill" docs/plan/*.md` finds no such job. phase-11.md:1497-1498 "no phase builds an operator-planned backfill in this release" (section 7.4) says no backfill is built, and a gap is covered by the re-read window and automatic catch-up (phase-11.md:1494). There is no backfill to link.
- Stands (coverage gap): phase 05 names the pause record as the source of the coverage gap (phase-05.md:44-45 "`source_pauses` (the record a coverage gap and a backfill are built from)"; phase-05.md:168-169 says phase 12 shows it as a coverage gap). Phase 12 reads the blind period table (phase-12.md:925). Phase 11 opens the period in the pause's transaction (phase-11.md:975 "A phase 05 pause (`POST /sources/{id}/pause`, any cause): cause `source_paused`, `started_at` = the pause's start,"; phase-11.md:1381). The two rows end at different events by design: the pause row closes at the move back to active (phase-05.md:706-707), while the blind period closes only after catch-up (phase-11.md:985 "closes an open source period when the source is `active` and no", and phase-11.md:12 on automatic catch-up on resume). Grep checked: `grep -n "pause_id\|source_pause_id" docs/plan/phase-*.md` finds no key that joins the two rows. C-15 (research/25-core-banking-integration.md:1148) asks for an open coverage gap shown to the MLRO and automatic backfill on resume, which phases 11 and 12 carry.
- Severity reasoning: MEDIUM. Phases 11 and 12 are explicit, so the gap can be built correctly. Phase 05 names the wrong record for an MLRO-facing figure, and that should be corrected.
- Fix: phase 05, lines 44-45 and 168-169: one sentence each saying that the coverage gap is the blind period that phase 11 opens in the same transaction as the pause, not a read of source_pauses. The lead decides whether a key link between the rows is wanted.

## Summary

- REFUTED: 2 (V-05-06-a, V-05-06-c)
- STANDS: 1 (V-05-06-b)
- PARTLY: 3 (V-05-08-a, V-05-02-a, V-05-11-a)

| Gap ID | Verdict | Severity | What stands | Fix place |
|-|-|-|-|-|
| V-05-06-b | STANDS | HIGH | Base 3.6 writes the worker's run report through `EvidenceStore::stage`, which phase 02 describes for akr-api uploads; generated output goes through `put_generated`; no 7.x block picks one | phase 05, section 3.6 (lines 1140-1141) or section 7.2 |
| V-05-08-a | PARTLY | MEDIUM | How permits already out at a listener rebuild count against the rebuilt governor; no text says so | phase 05, section 7.8 |
| V-05-02-a | PARTLY | MEDIUM | Derivation of a mapping from DDL, comments and samples is stated nowhere; VD-02 is named only in the phase 05 header | phase 10, section 3; phase 05 line 36 |
| V-05-11-a | PARTLY | MEDIUM | Phase 05 names source_pauses as the coverage-gap record; phases 11 and 12 build it from admin.blind_periods, opened in the pause's transaction | phase 05, lines 44-45 and 168-169 |
