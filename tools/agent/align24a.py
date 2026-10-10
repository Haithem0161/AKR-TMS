import re
p='docs/plan/research.md'
s=open(p).read()
rows=[
("DR-218","`akr-api` gains `readonly_pool` (KV `db/akr_readonly`) for four reads only: the audit viewer, the access history and control report, the pack verify and the verify-run estimate; DATA-MODEL s.15 `registers`, `validation_packs` and `fairness_metrics` change as phase 24 s.1 says; models, model scores and assistant drafts stay P2","The audit details and access log are readable only as `akr_readonly` (phase 02); one register table because findings and warnings are the same kind of governance record"),
("DR-219","Suspicion-tier records, their audit rows and their access history are refused to IA and SEC with the masked 404 and left out of search pages (no masked rows, no total); the tier comes from SECURITY DEFINER `audit.resource_tier`, which mirrors phase 17 tier rule 2, and `access_event` rows are hidden on the higher of the stored and current tier; `akr_readonly` gets no casework, alert, scn or canon grant","Phase 17's M-only rule; IA and SEC get tipping-off attempts as counts in the control report (lead review: the draft resolver missed the family, case and scenario tiers)"),
("DR-220","A verify run stores the `VerifyReport` as counts and finding kinds only; entry ids, anchors, object keys and detail stay in phase 02's report, the SIEM and the CLI","A viewer learns that a break exists and when, never which row of a suspicion case"),
("DR-221","Decisions and exclusions are read views through their owners' predicates (phase 17 `services::visibility::build`, phase 18 access list, phase 19 QA grants); findings and warnings are the only `admin.registers` rows","A copy would be a second record of a sealed decision; phase 17 rule 4 keeps restricted cases from CH and IA (lead review)"),
("DR-222","The register is append-and-close: identity, text and author fixed, status moves forward, closed rows final, no delete; risk accepted by M needs step-up `approval`; a finding's owner never accepts its risk","GV-34 without a workflow engine"),
("DR-223","Validation packs are versions written once; the verdict is computed from the seven parts and the bank delta; the validator is never the item owner (Cedar and table); a `pass` moves validation dates through phase 04, approval stays a separate act","GV-09, GV-15, GV-32: nobody reaches `pass` by choice"),
("DR-224","The command centre reads its sources live (`no-store`), counts through each list's own predicate, suppresses counts 1 to 4 (phase 21 threshold); suspicion counts and the tipping-off tile are M's alone","A tile must never show more than its list"),
("DR-225","The clocks board joins the owners' due columns and never moves a clock; subjects pass the sources' predicates; suspicion subjects are M's alone, rows and counts","A clock cannot be shown at one date and enforced at another"),
("DR-226","The assessor is role `AS` in `admin.role_grants` bounded by `valid_to` (`assessor.grant_max_days`, floor max 30); no `case_acl` reason; samples may hold suspicion items, each view logged with purpose `supervisory_inspection`; the trace returns no narrative and reaches a suspicion alert only as an item of a sample drawn under the grant in force; pack routes accept `assessor.read`, Cedar keeps AS to kind `assessor`","A sample without suspicion items tests nothing; the exception is bounded to the recorded sample (lead review)"),
("DR-227","The board pack is built from the signed periodic row and aggregates only; the builder refuses a payload carrying a case or report id","Phase 21 already suppresses small cells; case and report packs are M's alone"),
("DR-228","Training records are written by ADM (scope `admin.users`); Cedar `training.no_self` refuses recording one's own training; M, CH and IA read; a correction is a void plus a new row","Evidence for D2.1(b)(4); history never rewritten"),
("DR-229","The CBI assessment-guide clause map is a JSON data file in the release bundle, checked against its manifest hash at start-up; unquoted clauses are `text_pending`","S-56 keeps the D2 map as data; clause text is not invented"),
("DR-230","Pack download step-up class is `export` (phase 06's `pack_download` is not a phase 01 class); `POST /packs/verify` answers `verified: false` for unknown and for unreadable packs alike","A non-member cannot tell a case pack from nothing"),
("DR-231","The review calendar is derived from the due columns of rules, exclusions, change requests, readiness and scenarios and writes nothing; only a row turning red opens one warning (unique `source_key`)","GV-22, S-46: the warnings list holds only things someone must act on"),
]
anchor=re.search(r'^\| DR-217 \|.*\n',s,re.M)
assert anchor
add=''.join(f"| {d} | 2026-10-09 | {a} | {b} |\n" for d,a,b in rows)
assert '| DR-218 |' not in s
s=s[:anchor.end()]+add+s[anchor.end():]
open(p,'w').write(s)
print('ok',len(rows))
