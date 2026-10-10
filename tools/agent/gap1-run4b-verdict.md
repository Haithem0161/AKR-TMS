# Gap pass 1 run 4b: lead verdict (2026-10-10; decisions final, applied by apply-g1-run4b.py)

## Carried items (lead decisions, applied by apply-g1-run4b.py)

- Phase 26: `PartyReadPredicate` is phase 18's (phase 18 section 7+), not built by phase 17 `services::visibility::build`.
  rep the parenthesis at phase 26 section 2 ("phase 18 section 3.10 party rule, built by phase 17 ...") to "phase 18
  section 3.10 party rule (phase 18 section 7+)". Check the exact text before the rep.
- Phase 27 Open doubts 9 (advance notice, GV-29): decided, the notice is the audit record and the pending list on
  `GET /models`; no e-mail or in-app notice in this release (same rule as phase 22 Open doubts 14: the screens are the
  notice). Block in phase 27, no DR.
- Phase 30 section 3.x restore step 1: `akr-sign-cert` -> `akr-sign-deletion` (phase 23 section 7+, DR-357).
- Phase 31 section 4.5 item 9: "the sampler of phase 19 section 3.9" -> "section 3.8".
- Phases 27, 28, 29, 31: `akr_retention` DELETE only where the handler deletes (DR-386): a correction block in each
  naming its deleting tables (R12 `age` tables with no owner sweep; R5 `alert.model_scores` in phase 27; R3 owner rows in
  phase 26 stay); the others keep SELECT only. Grant lines: phase 27 line ~292 (list incl. alert.model_scores);
  phase 28 lines ~315-316 (R11/R10 tables and R6 drafts: none deletes in this release, R6 is report_only); phase 29
  lines ~632-638 (19 tables); phase 31 lines ~282-302 (practice_attempts R9 age = kept, so no DELETE yet; capacity
  snapshots/runs and directory_events R12 age = engine record_purge, DELETE stays; typology_exposure R7 = report_only).
  Delete-guards that admit akr_retention may stay (they admit, they do not grant).
- Phase 29 payment release status, DR-389 (draft decision): `canon.payment_message.status` takes the canonical values
  `received`, `released`, `rejected`, `returned`, `cancelled` through phase 10's `status` code map (kind `status`
  already exists in `admin.code_maps`); phase 10 adds the CHECK (correction block). Phase 29's release control
  (section 3.14 step 1) excludes rows whose status is `rejected` or `cancelled` when read; any other value is controlled
  (fail closed). Phase 29 block.
- Phase 30 Open doubts 23: closed by phase 12 section 7+ (restore verify runs in order; recon runs for a restore gap
  are valid, a date not yet reloaded shows as a break). Block in phase 30.
- Phase 09 out-of-scope line ("the typology set comes with the scenario register in phase 16"): now phase 06 (DR-381).
  rep "phases 13 and 14; the typology set comes with\n  the scenario register in phase 16" -> "phases 13 and 14; the
  typology set is phase 06's, DR-381". Check the exact text.

## Per finding (Haiku draft: 26 findings, 1 HIGH, 17 MEDIUM, 8 LOW, 5 UNCLEAR; sources spot-checked)

- G1-25-01 ACCEPT as a note in phase 25 (sources phase 27 4.5, phase 28 4.5, phase 29 4.5 item 8 confirmed): the
  support bundle's row-count list and the SIEM export exclude the seven tables by name; SIEM reads audit only.
- G1-25-02 REWRITE, DR-390: the release does NOT import models. Phase 25 verifies the bundle only; model and assistant
  artifacts a bundle carries are imported by people through phase 27 section 3.3 and phase 28's import, which set
  `registered` only (GV-29). Note in phase 25; phase 27's line citing "section 3.7" for the release import reads
  section 3.3 (rep in phase 27 if exact text found). Severity drops to MEDIUM.
- G1-25-03 ACCEPT (phase 03 says the release check records the hash): key `matrix_file_sha256` inside
  `policy_matrix_diff`; no column; not a refusal.
- G1-25-04 ACCEPT, trimmed (grep phase 01 ~920 before writing): runbook in phase 25 section 4: new KV version of
  `akr/keys/auth-hmac`; effects as phase 01 states (CSRF cookies re-minted, every personal token invalid); no session
  claim.
- G1-25-05 ACCEPT note (phase 30 owns: refusal code `chart_signature_invalid` in its migration, check (g), cosign_result
  chart count and K3s version, A4 facts).
- G1-25-06 REWRITE: `POST /admin/releases/{id}/applied` takes an optional `rollout_evidence_id`; the operator uploads
  `rollout.json` as evidence kind `package`, owner kind `release`, owner id this release; the route checks the owner
  (else `E_VALIDATION`). NO new column: the evidence's owner link is the record. Closes phase 25 Open doubts 14 as far
  as a record goes (still not a proof).
- G1-25-07 ACCEPT note.
- G1-25-08 DECIDE (a): no failover notice. Phase 30 correction (rep or block): failover and restore reports are viewable
  by `admin.system`; SEC sees their audit events (audit viewer, SIEM forward), no phase 25 notice. Block in phase 30.
- G1-25-09 ACCEPT note (loop over the table list in section 6 item 4; later phases add their tables).
- G1-25-10 ACCEPT note.
- G1-25-11 DECIDE, DR-391: canary records (SP-30, research 30 sections 8.6 and 8.8) are P2; phase 24's
  `canary_not_built: true` stays; block in phase 25 (not built, P2) and a note closing phase 24 Open doubts 12.
- G1-26-01 ACCEPT (variant `IslamicEvidenceGap`; call with `tx` and `body`).
- G1-26-02 ACCEPT (three-argument call in section 3.4 step 7) and, from run 4a carry, the section 2 port line no longer
  says "built by phase 17".
- G1-27-01 ACCEPT: a `pass` validation verdict calls phase 04 `InventoryService::update` to set `last_validated_at` and
  `next_validation_due = today + review.rules_months`; section 2 sentence "moved by the review calendar's existing
  route" -> "moved by a passing validation (section 7+)". Verify phase 24 ~702 wording before applying.
- G1-27-02 ACCEPT, trimmed: the probe set of kind `alert` gains route 16 (masked 404 identical to an unknown id; no
  score, band or reason in the body). No invented status codes.
- G1-27-03 DECIDE (a), DR-392: `admin.inventory_items` keeps R9 for every row; model evidence is R11 through phase 27's
  own tables. Phase 04's note "model items move to R11 in phase 27" is corrected (rep after grep).
- G1-28-01 REJECT (no change): MRO keeps `GET /assistant/monitoring` (counts only, suppressed, DR-286).
- G1-29-01 ACCEPT note (owner phase 30). G1-29-02 ACCEPT note (owner phase 30).
- G1-29-03 ACCEPT, lead names the variants (verify the four warnings in phase 29 section 3.12 first): `AssessKeyAge`,
  `AssessCertificateExpiry`, `AssessFallbackStale`, `AssessReleaseException`; also the blind-period warning if section
  3.12 has one (`assess:blind:` key seen at phase 29 ~1112): `AssessBlindPeriod`.
- G1-29-04 DECIDE: the circuit breaker is the bank gateway's; phase 29 note: its threshold is a bank setting that must
  not open before `ops.failover_max_s` (phase 30); no AKR setting. No DR.
- G1-30-01 ACCEPT (catalogue entry `pack:`; `--pack name=on|off`; off entries omitted; byte-identical with all off).
- G1-31-01 ACCEPT (`PgDirectoryLinkQuery`, wired when `pack.polis` is on).
- G1-DOC-01 ACCEPT: DESIGN-SYSTEM 8.1 row I1 (check the table's last-column format before inserting).
- G1-DOC-02 ACCEPT: two 8.3 paragraphs (S2 Holds tab, D3 Assessment tab), from phase 29 sections 5.1, 5.2.
- G1-DOC-03 ACCEPT: G1 paragraph gains the Assessment tile.

DRs planned: DR-389 payment status set and release control filter; DR-390 releases do not import models; DR-391 canary
records P2; DR-392 inventory items stay R9.
