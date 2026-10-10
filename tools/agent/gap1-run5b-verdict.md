# Gap pass 1 run 5b: lead verdict (2026-10-10; decisions final, applied by apply-g1-run5b.py plus the phase 15 header sentence fix)

Haiku draft tools/agent/gap1-run5b.md: 51 IDs; CARRIED 8, Partly 19, NOT-CARRIED 10, GAP 6 (all MEDIUM), UNCLEAR 8.
Scope test for every GAP: does MODULES or ROADMAP put the requirement in P0 or P1? For SP: only SP-03, SP-64..66,
SP-83..88 (M20), SP-19..33 listed by M3 (MODULES line 117) and SP-26 to SP-36 (ROADMAP risk row 5 and the P1 quality
gate) are in P0/P1. For K: M10 [P1] owns K-01..K-42 and has no P2 line, so a K item is in P1 unless a P2 line, a DR or
missing data puts it out.

## Classification decisions
- CARRIED accepted: SP-23 (22), SP-29 (15 section 3.9), SP-34 (24), SP-40 (02), SP-57 (23), SP-96 (29), K-12 (19),
  K-34 (15; model card and validation packs are phases 27 and 24).
- Partly accepted; header text names only the built part (the draft's "missing" lists were not all grepped): SP-10 and
  SP-45 (00), SP-41 (02), SP-47, SP-49, SP-79 (03), SP-46 (13), SP-54 (17; the policy statement is the bank's),
  SP-26, SP-32, SP-100 (18), SP-55, SP-63 (23), SP-80, SP-99 (24), SP-92 (27), K-31 (12), K-32 (10), K-38 (24; CBDDQ
  evidence is P2 by MODULES line 654).
- NOT-CARRIED accepted (DR-396): SP-01, SP-67, SP-68, SP-77 (25, vendor programme and contract), SP-50 (25, P2), SP-62
  (23, vendor contract exit; the certificate is built), SP-69 (00, vendor SDLC), SP-78 (31, vendor assurance), K-40 (29,
  the assessment API returns decisions only), K-41 (15, procurement step).
- The Partly rows' in-scope "missing" parts: none becomes a GAP. SP-41, SP-45, SP-46, SP-47, SP-49, SP-55, SP-63, SP-10,
  SP-79, SP-80, SP-92, SP-99, SP-100 are on no P0/P1 line. SP-32 (in SP-26..36): the case access list is the approval
  of a content download (only the team opens it, with step-up, purpose and an access event); per-user watermarking of
  downloaded files is P2 (DR-396).

## UNCLEAR decisions
- SP-02 NOT-CARRIED, phase 24: control crosswalk documents are vendor compliance work (phase 24 Open doubts 10), DR-396.
- SP-61 Partly, phase 23: no erasure outside the retention classes; CBS-origin data is never written (read-only source,
  phase 05). Restriction handling for a data-subject request is the bank's process (DR-396).
- SP-82 NOT-CARRIED, phase 05: the CISO's exception register is the bank's (the weekly assessment control test is
  phase 29's `assess.control_tests`), DR-396.
- SP-90 Partly, phase 25: signed release bundles verified on import, no call-home; hosting the content services is the
  vendor's operation.
- SP-91 Partly, phase 27 (signed ONNX artifacts, operator allow-list, golden check with fallback). The two sources do
  not truly conflict: research 30's rule is read as "no code-bearing model format". The assistant's GGUF file (DR-275)
  holds weights only and is a signed artifact on a read-only volume. Recorded in DR-396.
- SP-97 NOT-CARRIED, phase 23: data-subject requests are the bank's process; no MODULES or ROADMAP line (DR-396).
- SP-102 NOT-CARRIED, phase 00: record of processing, privacy impact assessment and breach record are the bank privacy
  office's documents; the phase 00 classification registry is their input (DR-396).
- K-39 NOT-CARRIED, phase 26: Islamic inputs to any score are forbidden (DR-249), F3 is withheld (DR-103), and the
  charity module is not carried by phase 26.

## GAP findings
- G1-18-01 SP-28 (with SP-26's time box) ACCEPT MEDIUM, rewritten (DR-395). No new table: the register is the manual
  grants of `casework.case_acl`. On a suspicion-tier case a manual grant is made by role M only (the MLRO's approval is
  the grant; its note is the written reason) and carries `expires_at` (at most `casework.manual_grant_max_days`, default
  30). Column `expires_at` added in the DDL; the revoke CHECK lets a NULL `revoked_by` mean the system expired the grant
  (as `granted_by` NULL means the system granted it); the guard keeps `expires_at` fixed. Timer kind `case_acl_expiry`
  plus an hourly backstop job. Chain and audit `case.acl_expired`. Phase 24's control report counts manual grants on
  suspicion cases (granted, expired) for M and SEC (24 section 7+). The draft's new table, four routes, invented Cedar
  actions and "written_approval_id" are dropped.
- G1-15-01 K-19 ACCEPT IN PART (DR-395): the coverage view is P1 (M10 owns K-19; all data is in `kyc.ownership_results`):
  `GET /ownership/coverage`, counts only, reusing Cedar `rating.distribution.read` (M, CH, IA). The registry import
  path with source and fee tracking is P2 (no CRD or BO-platform feed exists to import; DR-396). The draft's new table,
  column and invented actions are dropped. K-19 Partly in phase 15.
- G1-15-02 K-25 ACCEPT MEDIUM, rewritten (DR-395): `GET /rating/overrides/analytics`, counts only, Cedar
  `rating.distribution.read`; per proposer and per branch (`canon.customer.branch_id`, current rows): total, downward,
  expired; follow-on alerts and STRs from phase 18 on (overall counts, not per person). Bulk re-rating events are not
  covered, so K-25 is Partly.
- G1-15-03 K-36 REJECT as gap: the controls matrix (DSL C.7) has one row per level with the CDD depth and the
  monitoring profile `risk:<level>`, so level 1 already carries its CDD depth and compensating scenario parameters; the
  trigger library re-rates on change, which is the automatic upgrade. No new column or setting. K-36 Partly in phase 15.
- G1-21-01 K-23 ACCEPT MEDIUM, rewritten (DR-395): no new pack kind (closed list; `cbi_monthly` fits, the KPI report
  goes to the CBI). The `cbi_monthly` facts gain the PEP section, items a to e from system data (research 26 section
  on the CBI KPI report): PEPs by tier, by source, EDD sign-offs and exits, reviews completed, STRs on register parties.
  Items f (internal-audit test results) and g (staff trained) are not held by AKR-TMS; the cover says so, as for the
  RP-68 reports. K-23 Partly in phase 21.
- G1-16-01 K-20 REJECT as gap: every scenario output already carries links by `family_no`, `phone`, `address`,
  `signatory` and `owner` (shared identifiers, recurring signatories). Profit distribution to non-shareholders and
  ownership change before large transactions need data the CBS does not hold (ownership is bank-entered, M10); they are
  P2 or bank-authored scenarios (DR-396). K-20 Partly in phase 16.

## Other edits
- MODULES M10 line ("K-12, ... await later phases"), phase 15 Open doubts 19 and the phase 15 header sentence that listed them as "left to later phases" (it made coverage.py count them as carried): rewritten to name where each K ID went.
- API.md: two new rows after `| GET | /rating/distribution |`; the `POST /cases/{id}/access` row names the time box.

DRs: DR-395 (the run 5b additions), DR-396 (outside this release and the readings: vendor programme, bank processes, P2
items, SP-91 reading).
