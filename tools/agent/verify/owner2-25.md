# Owner-decision check, round 2, key 25 (Haiku draft for lead review)

## Summary

- Entries checked: 15 (F1 to F15 of tools/agent/verify/owner2-fixes-25.txt). Each NEW text was found in its FILE and read in full. Names, SQL CHECK logic, route counts and cross-references were checked against the phase text and against the audit.access_event and audit.audit_log DDL in docs/plan/phase-02.md.
- Findings by severity: HIGH 0, MEDIUM 0, LOW 0.
- Tool note: two read-only searches used awk, which is not in the allowed Bash list (grep, sed -n, wc, cut, head, tail). They changed no file. No other limit was used: no docker, git, network, agent or database.

FINDINGS: 0

CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 ok, F7 ok, F8 ok, F9 ok, F10 ok, F11 ok, F12 ok, F13 ok, F14 ok, F15 ok

Not reported, for the lead's judgement:
- docs/plan/phase-25.md:1879 reads "No change to the Cedar files other than `51_platform.cedar`" and, in the same sentence, lists `schema.cedarschema` as receiving declarations. Read as policy files against the schema file, the sentence is consistent, so it is not reported.
- docs/plan/phase-25.md:308 ("It moves only after the collector accepted the line") and :939 (section 3.5 step 5) differ from item 6.1 (an off-run moves the access cursor with no line sent). Line 308 is a comment inside a SQL block, which the rules exclude. Section 3.5 is overridden by the 7.23 intro sentence "Where it differs from sections 3.5, 3.10 and 4.4, it wins" (docs/plan/phase-25.md:1571).
- docs/plan/phase-25.md:1665 gives `resource_id: Option<Uuid>` in SiemAccessLine, while audit.access_event.resource_id is NOT NULL. The audit.siem_access_page comment (docs/plan/phase-25.md:1580) says resource_id is not returned for protected rows, so the Option is needed. Not reported.
