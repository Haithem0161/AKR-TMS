# Phase 26 review findings (lead, 2026-10-09)
1. MEDIUM pooled freeze task cannot be created: s.3.8 step 3 sets `second_person_only = TRUE` when the `hold` task is
   created, but phase 20 `casework.ops_tasks` has CHECK `NOT second_person_only OR verification_level = 'v0'` and
   `verification_level` is NULL until completion, so the insert fails and the whole freeze start (a legal deadline) rolls
   back for any freeze that touches a pooled account. `second_person_only` is phase 20's V0 verification mark, not a
   request for two people. Fix: the pooled task is a normal `hold` task; phase 20 verification applies unchanged; a
   two-person rule for pooled holds is Open doubts 9. Curl q and DR-252 follow.
2. MEDIUM no way to open the evidence file: the deal view returns no evidence object id and cites phase 02
   `GET /evidence/{id}`, which no phase builds (phase 18 serves content per owner: `GET /cases/{id}/evidence/{evidence_id}/content`),
   so the verifier decides on a document nobody can open; action `islamic.evidence.read` is declared and used by no route.
   Fix: added route `GET /islamic/deals/{id}/evidence/{evidence_id}/content` (`queue.work`, `islamic.evidence.read`;
   access event first; step-up `record_open` on a suspicion-tier deal; `EvidenceStore::open`), the phase 18 pattern.
3. MEDIUM the sweep's register insert fails: `source = 'islamic_evidence'` is outside phase 24's closed CHECK (manual,
   post_review, fairness, review_calendar, scan) and the column is `body`, not `detail`. Fix: source `scan` (as phase 25
   used `review_calendar` and `scan`), `body`.
4. LOW G1 tile for A2: the "awaiting evidence" tile is shown to A2, who holds no `dashboard.read` (the G1 scope). Fix: M only.
5. LOW access event: phase 02 `access_event.action` is a closed list (view, search_hit, export, print, download, unmask);
   the deal read writes action `view` with resource_type `islamic_deal`, the content read `view` or `download`.
6. LOW grants: `akr_retention` got SELECT on ALL TABLES at phase 23 time, which does not cover tables created later; the
   three tables need `GRANT SELECT ... TO akr_retention`; the comment "the only DELETE in kyc" is wrong (phase 23 grants
   many). Also the 1.1 SQL block was not closed before 1.2.
7. LOW wording: suppression is "below `report.small_number_threshold`" (governed, min 3), not a fixed "1 to 4"; the sweep
   uses `BusinessClock::add(opened_on, islamic.evidence_due_days)` forward (its `n` is u32), not a negative add.
Severity: 0 critical / 0 high / 3 medium (1-3) / 4 low (4-7) = 7.
Gap-pass items (not draft defects): DATA-MODEL s.20 moves possession_evidence to `kyc` (DR-246) and the view; phase 02
owner kind `islamic_deal` and key purpose `islamic.document_ref`; API.md s.6 lacks the phase 02 codes E_UPLOAD_REJECTED,
E_PAYLOAD_TOO_LARGE, E_UNAVAILABLE (earlier phases use them too); phase 16 pack seed skips flagged documents and G9
reads `pack: islamic`; phase 03 flag service refuses `pack.islamic` off while a version is live; phase 10
`CanonicalEntity` +2 and `canonical-model.v1.json`; phase 20 `FreezeService::start` grouping by class; hard-rule
scenarios are `restricted` (suspicion-tier alerts, M or team only): the bank may prefer `normal` for product-rule breaches;
retention of superseded deal versions (closure clock on rows whose `closed_on` is NULL); I-09 not carried.
