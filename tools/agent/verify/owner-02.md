# Owner-decision round, phase 02 (Haiku draft for lead review)

## Not counted (seen while checking; not findings)

- `lang`: `GeneratedEvidence` is `NewEvidence` without `file_name` (docs/plan/phase-02.md:612-613), so it takes `lang`, and the column `lang` is NOT NULL (phase-02.md:367). Phase 07 item 6 step 7 (docs/plan/phase-07.md:1171) does not name it, and neither do the other put_generated calls (phase-07.md:488, docs/plan/phase-06.md:655, docs/plan/phase-25.md:805). The rule on comparable things applies.
- Section 7.10 (phase-02.md:1264) says the 7.1 registry "also holds" change_request, release and access_review, and the 7.1 table (lines 1167 to 1192) does not list them. This predates the diff, and the new row does not contradict it.
- Stage step 2 (phase-02.md:843) gives no sniff rule for XML or JSON, yet put_generated runs steps 1 to 3 with the generated-type list (phase-02.md:877). This predates the diff; the XSD puts of phase 07 (phase-07.md:488) already rely on it.

## Summary

- Changed places checked: 1 (docs/plan/phase-02.md:1173, the new row of the owner-kind table in section 7.1). The diff has one hunk, `@@ -1172,0 +1173 @@`, and line 1173 is the same text as the added line.
- Findings: HIGH 0, MEDIUM 0, LOW 0.
- CHECKED: 7.1 checks 1 to 6 ok (row at line 1173).
- FINDINGS: 0

Checks behind the result:

- Evidence kind `snapshot`: phase 07 item 6 step 7 (docs/plan/phase-07.md:1171) states it, and phase 02 allows it (phase-02.md:355, the CHECK on `kind`).
- Owner kind `goaml_corpus_case`: phase 07 states it at phase-07.md:1171 and 1174 ("joins the phase 02 registry"). It passes the owner-kind pattern (phase-02.md:356) and the registry rule (phase-02.md:847), and the row registers it.
- Retention R9: phase 07 states R9 at phase-07.md:865, 1092 (table row) and 1171. Phase 02 accepts R9 (phase-02.md:358) and gives it a lock period (phase-02.md:511, `"R9": 3650`). The rule at phase-02.md:949 requires every class an owner kind uses to be present, and R9 is present.
- Classification `secret-str`: phase 07 states it at phase-07.md:1171. The column is `admin.data_classification` (phase-02.md:366), and the enum holds 'secret-str' (docs/plan/phase-00.md:288). The table's own columns are 'internal' (phase-07.md:1085). Those cover the table's columns, not the evidence object, so there is no conflict.
- put_generated (phase 07 item 6 step 7): XML is on the generated-type list (phase-02.md:877). `application/xml` passes the content-type pattern (phase-02.md:364). The "no file name" of phase 07 agrees with `GeneratedEvidence`, which has no `file_name` field (phase-02.md:613). The owner id is a UUID (phase-02.md:357), and the case id is UUIDv7 (phase-07.md:1157).
- Phase 07 item 1 (phase-07.md:1022 and 1029): `evidence_id` references `admin.evidence_objects(id)`, which exists (phase-02.md:354).
- Same names elsewhere in phase 02 (grep for goaml_corpus_case, secret-str, R9, snapshot, DR-415, section 7.6): no line states a different rule for them. `secret-str` at line 459 is another column; R9 at lines 511, 1170, 1172 and 1187 has the same meaning.
- Other plan file: docs/DATA-MODEL.md:694 also says the masked copy is kind `snapshot`, `secret-str`.
- Layout: lines 1167 to 1192 (header, separator and 24 rows) each have four pipe characters, so three cells. The new row uses the same backtick style as `network_snapshot (kind `snapshot`)` (line 1181) and the "R6 (classification personal)" form of `rfi_input` (line 1184).
- Decision: DR-415 (docs/plan/research.md:426) says only the masked copy is kept as sealed evidence. The row carries that out.
