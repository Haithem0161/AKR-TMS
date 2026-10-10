# Gap pass 1 run 5a: lead verdict (2026-10-10; decisions final, applied by apply-g1-run5a.py)

Haiku draft tools/agent/gap1-run5a.md: 59 IDs; CARRIED 11, Partly 35, NOT-CARRIED 1, GAP 12 (HIGH 2, MEDIUM 9, LOW 1).
Scope test used for every GAP: does MODULES or ROADMAP put the requirement in P0 or P1?

## Classification decisions
- CARRIED 11 accepted, except RP-08 moved to Partly (phase 07 allows several active profiles, one per code; per-profile
  clocks and numbering not read as built).
- Partly 35 accepted. Header text names only the built part (the draft's "missing" lists were not all checked; for
  example C-32's NTP is built by phase 30 chrony, DR-335), so the header claims nothing false.
- NOT-CARRIED C-34 accepted (phase 05, bank DBA and SIEM).
- Headers: one line per class at the end of each phase's "Requirements carried" block ("Gap pass 1 (coverage),
  implemented here / partly implemented / not carried: ...").

## GAP findings
- G1-11-01 C-14 ACCEPT, rewritten (DR-393): phase 11 accepts `landing_file`, `t24_xmlrecord`, `t24_multivalue`
  (transforms `XmlField`, `MultivalueSplit` exist in phase 10); phase 10's step 3 said phase 11 adds them and none did.
  MODULES M5 carries C-01..C-24.
- G1-11-02 C-41 ACCEPT HIGH, rewritten (DR-393): MODULES M5 P1 has payment messages (ISO 20022 and MT). No new column:
  the parser version is in the compiled mapping (`mapping_version_id` on every row). Parse failure is quarantine kind
  `row_failed`, reason `message_parse_failed` (phase 11 quarantine reason is a pattern, not a list). No crate path or
  migration invented.
- G1-11-03 C-42 REWRITE to CARRIED with a phase 11 note: a gateway database is a second `admin.sources` row with
  relational mappings to `canon.payment_message` (phase 10 catalogue); no invented adapter pack. Phase 11 trailer line
  corrected.
- G1-11-04 S-20 REJECT as gap: P2 (no system doc puts it in P1), DR-394; phase 06's pointer "device and channel data in
  26" corrected.
- G1-12-01 C-43 REJECT as gap: `canon.payment_link` carries NOT NULL lineage columns, so it is a mapped table, not a
  matcher's output; MODULES M7 P1 is four rungs; match rules and L4 (and L2, L5, L6) are P2, DR-394. Phase 12 block
  closes its Open doubts item on the deferred rungs. C-43 Partly in phase 10.
- G1-12-02 C-53 REJECT as gap: MODULES M6 omits C-53; merge and split are P2, DR-394; phase 10's pointer to phase 12
  corrected. C-53 Partly in phase 10 (survivorship).
- G1-09-01 S-15 REJECT: governed registers are P2 (DR-380 removed `/ref/registers`). Not carried in phase 09.
- G1-13-01 S-28, G1-16-01 S-30, G1-16-02 S-33 REJECT: P2 (phase 16 Open doubts 21 left them to later phases or P2; no
  P1 line names them), DR-394. Not carried in phase 16. ARCHITECTURE's rule for S-30 (a review, never an exit) stays.
- G1-21-01 RP-35 REJECT: the officer registry and its notices are the bank's process; holders of role M are in phase 01
  grants and the audit trail. DR-394; Not carried in phase 21.
- G1-24-01 S-35 REJECT: P2, DR-394 (the draft also guessed the meaning of role CH). Not carried in phase 24.

DRs: DR-393 (phase 11 owns the non-relational formats and the payment parsers), DR-394 (the items outside this release).
