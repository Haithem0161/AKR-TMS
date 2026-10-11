# Owner-decision round, phase 25 (Haiku draft for lead review)

### O-25-1  HIGH   (section 1.3 DDL and section 7.24 item 1, check 4: CHECK against the open state)
- Problem: the last CHECK of `admin.siem_dead_letters` is FALSE for every row with `resolution` NULL, which is every open state: dead-lettering, a resend request and a failed resend. `(resolution = 'marked')` is NULL when resolution is NULL, and `NULL IS NOT DISTINCT FROM FALSE` is FALSE. Checked in PostgreSQL 17 (throwaway container) with the DDL text of section 1.3 as written: the open row and the resend-requested row both fail with `siem_dead_letters_check3`; the resent and the marked-with-code rows pass. With the CHECK written as `(((resolution = 'marked') IS TRUE) = (resolution_code IS NOT NULL))` all five states pass, and a marked row without a code and a resent row with a code are refused. Effect: the dead-letter INSERT of section 3.5 step 4 is in the same transaction as the cursor advance (step 6), so a refused line rolls the cursor back and the stream stops on that line, against DR-235 ("one bad line cannot block the stream").
- Proof: docs/plan/phase-25.md:340 "CHECK ((resolution = 'marked') IS NOT DISTINCT FROM (resolution_code IS NOT NULL))" and the same line in the 7.24 copy at docs/plan/phase-25.md:1726; docs/plan/phase-25.md:940 "in the same transaction as the dead-letter rows" (section 3.5 step 6).
- Smallest fix: in both copies replace the last CHECK with `CHECK (((resolution = 'marked') IS TRUE) = (resolution_code IS NOT NULL))`, so the two copies stay identical.

### O-25-2  MEDIUM   (section 2, "Route count" bullet; check 5 and the header)
- Problem: the route-count bullet of section 2 gives the size of the phase as L, while the header and the section 7.24 block give XL for 26 routes. The planning rule of .claude/rules/planning.md puts 25 routes or more in XL.
- Proof: docs/plan/phase-25.md:775 "The size of the phase is L." and docs/plan/phase-25.md:1743 "the size of the phase is XL; it was M with 22 routes" (header, line 8: "XL by the template rule of 25 routes or more").
- Smallest fix: in line 775 change "The size of the phase is L." to "The size of the phase is XL.".

### O-25-3  MEDIUM   (section 7.23 item 6.1 and item 12, forwarder test; check 1 and the stated test)
- Problem: the cursor of the access stream is moved only when an off-run executes, and the off-runs happen every 10 seconds. An access event written after the last off-run and before the setting is set back to true has an occurred_at later than the cursor, so the first on-run sends it. The rule "nothing read while the export was off is sent later" holds, but the test sentence "the events written meanwhile are not sent" fails for such an event, and a test that writes events after an off-run and then switches on at once would fail.
- Proof: docs/plan/phase-25.md:1697 "after it is set back to `true`, the events written meanwhile are not sent"; docs/plan/phase-25.md:1659 "Nothing read while the export was off is sent later."; docs/plan/phase-25.md:1314 "`schedule.siem_forward` (every 10 seconds)".
- Smallest fix: in item 12 change the sentence to "after one more off-run, the setting is set back to `true`, and the events written before that run are not sent", and add to item 6.1: "an event written after the last off-run is sent when the setting is set back to true".

### O-25-4  MEDIUM   (trailer "What This Phase Does NOT Touch", Cedar bullet; check 2)
- Problem: the trailer says that `schema.cedarschema` receives the action declarations of section 3.12 only. Section 7.24 item 3 adds two more action declarations (`siem_dead_letter.read`, `.resend`, `.resolve`) to the same file, so the trailer contradicts the new text.
- Proof: docs/plan/phase-25.md:1879 "`schema.cedarschema` receives the action declarations of section 3.12 only"; docs/plan/phase-25.md:1745 "`schema.cedarschema` gains `action "siem_dead_letter.read"`".
- Smallest fix: in line 1879 change "of section 3.12 only" to "of section 3.12 and of section 7.24 item 3 only".

### O-25-5  MEDIUM   (docs/API.md row of the access-line pull route, against section 7.23 item 1; rule 6 row check)
- Problem: the API.md row for GET /siem/access-events describes the lines as "allowed reads of sensitive routes". Section 7.23 item 1 exports every row of audit.access_event, with decisions allow, deny and conflict. The method, path and scope of the row agree with section 2 and section 7.23 item 8 (checked); only the description is wrong, and it tells API readers that deny and conflict lines are not in the feed.
- Proof: docs/API.md:851 "access lines (allowed reads of sensitive routes) after a cursor"; docs/plan/phase-25.md:1573 "decisions `allow`, `deny` and `conflict`) is exported as an access line".
- Smallest fix: in docs/API.md line 851 replace "access lines (allowed reads of sensitive routes)" with "access lines (every access event: allow, deny and conflict)".

### O-25-6  LOW   (section 7.23 item 7, Rust struct; check 4 types)
- Problem: `on_behalf_of` is nullable in audit.access_event, and most access events have no on-behalf-of value. The struct declares it as Uuid without Option, so a NULL cannot be decoded (sqlx refuses a NULL for a non-Option type at run time).
- Proof: docs/plan/phase-25.md:1665 "on_behalf_of, resource_type, resource_id: Option<Uuid>"; docs/plan/phase-02.md:147 "on_behalf_of   UUID,".
- Smallest fix: declare the field as `on_behalf_of: Option<Uuid>` in the `SiemAccessLine` struct of section 7.23 item 7.

### O-25-7  LOW   (section 7.24 item 8, step 1; check 6, refusal status)
- Problem: step 1 of the resolve route names a refusal ("If-Match is required") without its status or code. Step 1 of the resend route (item 6) gives "428 E_PRECONDITION_REQUIRED", and the test of item 13 expects 428.
- Proof: docs/plan/phase-25.md:1779 "`If-Match` is required."; docs/plan/phase-25.md:1766 "(428 `E_PRECONDITION_REQUIRED`)".
- Smallest fix: in line 1779 change "`If-Match` is required." to "`If-Match` is required (428 `E_PRECONDITION_REQUIRED`)."

### O-25-8  LOW   (section 7.24 item 9, sentence after the audit table; wording of a fact)
- Problem: the sentence calls the three kinds that are not exported "the three kinds that the forwarder's own work produces". The kind `siem.dead_letter_resend_requested` is written by the request of item 6 (the API), not by the forwarder. Only `siem.dead_letter_resent` and `siem.dead_letter_resend_failed` come from the forwarder.
- Proof: docs/plan/phase-25.md:1793 "The three kinds that the forwarder's own work produces are not exported"; docs/plan/phase-25.md:1768 "audit `siem.dead_letter_resend_requested`".
- Smallest fix: replace "The three kinds that the forwarder's own work produces" with "The three kinds of the resend work (two of them written by the forwarder)".

### O-25-9  LOW   (section 7.23 item 11, tipping-off review; check 6)
- Problem: item 11 lists what a protected access line carries, but it leaves out three fields that item 5 puts on every line: the access id, the decision (allow, deny or conflict) and the on-behalf-of id. A tipping-off review should name every field the reader gets, so that the answer "nothing" is checked field by field.
- Proof: docs/plan/phase-25.md:1691 "carries the time, the actor, the role, the verb, the resource kind, the purpose code, the client address and three flags"; docs/plan/phase-25.md:1655 "`{"time", "access_id", "action", "decision", "actor_id", "actor_role", "on_behalf_of"`".
- Smallest fix: in item 11 add "the access id, the decision and the on-behalf-of id" to the list before "and three flags".

## Summary

Blocks checked: 7.23 (items 1 to 12) and 7.24 (items 1 to 13), in full. Base places checked: the 23 changed hunks of the diff outside the new blocks (header lines 8, 9 and 13; section 1.3 at the comment, table, CHECK, index, function, grant, setting and classification; section 2 at SiemRepository, the four route rows, the router and route-count bullet; section 3.10 tile; section 3.15 row; section 4.4 metrics; section 7.13 heading and item 3; section 7.16 heading and item; Decisions DR-414 and DR-418; the trailer bullet), plus the unchanged base the new text leans on (3.5 steps 1 to 7 and the four pull steps, 3.10, 3.11, 3.12 (siem.pull.service and the Cedar file pattern), 3.13, 3.14, 4.1, 4.5, 5.4, 5.5, 6, 7.12, 7.15, 7.17, 7.22, Open doubts), and the four route rows of docs/API.md (lines 851 to 854) and line 107.

Findings by severity: HIGH 1, MEDIUM 4, LOW 4 (9 in all).

Per block:
- 7.23: finding ids O-25-3, O-25-5, O-25-6, O-25-9. Checks 3 and 5 ok; the SQL of item 2 runs as written (check 4), and the one type problem is O-25-6.
- 7.24: finding ids O-25-1, O-25-4, O-25-7, O-25-8. Checks 3 and 5 ok.
- Base (header and section 2, trailer): finding ids O-25-2 and O-25-4.

Checks that passed (evidence):
- Both copies of the `admin.siem_dead_letters` DDL with its indexes (section 1.3 lines 321 to 344 and section 7.24 lines 1707 to 1730) are identical line for line. The two copies of `audit.siem_access_page` (section 1.3 lines 394 to 453 and section 7.23 lines 1578 to 1637) are identical, 60 lines each.
- The column-level UPDATE grant lists every column that the resend request, resend success, resend failure and resolve steps update (`resend_requested_at`, `resend_requested_by`, `resolved_at`, `resolved_by`, `resolution`, `resolution_code`, `reason_code`, `attempts`, `version`); no step updates `action`, `audit_id`, `occurred_at` or `stream`.
- The classification row names the 15 columns of the table once each (set comparison; no duplicate, none missing).
- Routes: section 2 has 26 distinct routes (lines 769 to 772 are the four added rows), 12 marked owned and 14 marked added. The repeated rows of 7.23 and 7.24 match section 2 word for word. The row repeated in 7.22 (base text, not changed in this round) has the same method, path and scope but a longer description; not reported. The four new routes have the same method, path and scope in docs/API.md (lines 851 to 854).
- Names used from other phases exist: `audit.access_event` (columns, the six action values and the three decisions match the CHECKs of phase 02), `casework.effective_tier` (phase 17), `casework.case_acl` (phase 18), `casework.case_parties` and `alert.alerts` with `claimed_by` and state `claimed` (phase 17), `canon.party` with `staff_flag`, `related_to_staff`, `recorded_to`, `valid_from` (phase 10), `audit.resource_tier` (phase 24), `case.decided` and `case.closed` exported (phase 18), `authz.allowed` with SIEM "no" (phase 03), `ClientCredentials` and kind `service` (phase 01), `admin.purpose_codes`, the roles ADM and SEC, the scopes `admin.system` and `security.ops`, the `system_admin` group (phase 03), `AuditSink` (phase 02).
- No place of the new text gives a resource id, a subject party or a tier for a protected access line, except the incomplete list of O-25-9. The function returns NULL for both ids on protected rows, and the CEF and JSON lines use "or empty" or null for them.

Checks run in PostgreSQL 17 (postgres:17 container, removed afterwards; scratch SQL kept in the session scratchpad, not in the project):
- Dead-letter table with the exact DDL of section 1.3: as written, the open row and the resend-requested row are refused by `siem_dead_letters_check3`, and the resent and marked-with-code rows are accepted. With the proposed CHECK, all five states are accepted, and a marked row without a code and a resent row with a code are refused (O-25-1).
- `audit.siem_access_page` as written, with stub tables that follow the phase files (access_event DDL of phase 02, effective_tier of phase 17, case_acl, case_parties, alerts, canon.party), run as akr_app: it compiles; the 10 fixture events give the protected, work_link and subject_staff values that item 12 states; after P3 is raised to suspicion, its line has both ids NULL; a LIMIT of 0 and of 5001 raise the "siem page size out of range" error; akr_readonly is refused; akr_app reads no table directly; the 15 output columns contain no tier.

Notes (not findings, for the lead):
- 7.17 (earlier round) and 7.24 item 9 say that sections 3.11 (the closed sub-code list) and 3.13 (the audit table) gain entries that exist only in the blocks. Section 4.5 builds the CATALOGUE from section 3.13, so the four dead-letter kinds reach the catalogue only when the table is read with the block. The same convention is used in 7.17, so I did not report it; the lead may prefer to copy the rows into the base tables. Likewise the hook summary of section 5.5 does not list the three hooks that 7.24 item 11 names, and section 4.5's DESIGN-SYSTEM bullet does not mention the dead-letter table on A3.
- Section 7.13 keeps the sentence "SP-36 is partly implemented here and the rest is P2"; with the owner decision it still holds for canary access and the DBA rules, so I did not report it.
- Sections 4.1 ("read the audit log through `audit.siem_page` only") and 7.3 ("reads the audit log only") use the audit log in the phase's own sense, which is the audit_log table; the access log (access_event, read by `audit.siem_access_page`) is named separately in section 1.3 and section 7.23. Read that way they are not in conflict, so I did not report them.

FINDINGS: 9 (HIGH 1, MEDIUM 4, LOW 4)
