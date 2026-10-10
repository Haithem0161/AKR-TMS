# Phase 25 review findings (lead, 2026-10-09)
1. MEDIUM break-glass open request per case: `uq_break_glass_open_case (case_id)` with `ON CONFLICT DO NOTHING` (s.3.2
   Request step 3) drops a second requester's request silently while step 4 writes `break_glass.requested` naming a grant
   that was never inserted; the second requester's own list then shows nothing, which tells them another request is open
   on that id (an existence and activity signal), and any A2 can hold the emergency path of a case shut by keeping a
   request open. Fix: unique `(case_id, requester_id)`; every request inserts its own row; the audit row names it.
2. MEDIUM read-only deny list incomplete: `bg.read_only` forbids 16 named case and pack actions, but the requester joins
   `Case.assignees`, which other team checks read (phase 19 `Rfi.assignees` is the case team: RFI drafting, and the
   later case-linked writes of phases 19-21). Fix: under purpose `break_glass` forbid every action outside a read
   allow-list (case.read, case.note.read, case.evidence.read, case.chain.read, case.link.read, case.work.read,
   case.access.read, alert.read, alert.snapshot.read, party.read, party.transactions.read, network.read, rfi.read,
   rfi.read.case, report.read).
3. MEDIUM SIEM leaks the tier: `audit.siem_page` returns column `tier` and the JSON line carries it, so the value
   `suspicion` or `restricted` leaves for the SIEM with the actor and time (phase 02 l.995: no suspicion-tier content;
   phases 17-20 viewer rows: "no tier value"). The SECURITY DEFINER function also has no `OWNER TO akr_migrate` and its
   search_path includes `audit`. Fix: drop `tier` from the function, `SiemLine`, the JSON line and the tests (a stripped
   row is recognisable only by its empty resource); owner `akr_migrate`; `SET search_path = pg_catalog`.
4. MEDIUM requesters who can never use a grant: phase 17 rule 1 admits only A1, A2 and M to any case or alert row, so a
   grant to SEC or CH opens nothing, and M needs none. Fix: requester role A2 only (Cedar `bg.request.a2`,
   `bg.end.a2`; the request and end permits for M and CH dropped; SEC keeps end and review); DR-242 and the test of
   s.6 (a2@ requests; a2@ approving is a Cedar 403) follow.
5. LOW visibility clause without the purpose: the SQL clause of s.3.2 Use step 1 admits the case for the requester under
   any purpose while the Cedar adapter adds the requester only under purpose `break_glass`, so the phase 17 pin test
   (list predicate equals Cedar) breaks and a list shows a case the single read masks. Fix: the clause also requires the
   request's purpose `break_glass` (rule 5 already takes the purpose as an input).
6. LOW step-up class: the support-session approval creates a role grant, which is phase 01 class `access_change`, not
   `approval`. Fix: `access_change`.
7. LOW verification expectations: `vs@` reading `GET /cases/{N1}` or `{S1}` is stopped by the scope gate (VS holds no
   `queue.work`), so both answer 403 `E_SCOPE`, not `E_FORBIDDEN` and the masked 404.
Severity: 0 critical / 0 high / 4 medium (1-4) / 3 low (5-7) = 7.
Gap-pass items (not draft defects): phase 17/18 case read path writes `break_glass.used` and the dedicated-log `used`
row, and takes the purpose-keyed clause; phase 19 `Rfi.assignees` must not take the break-glass requester; phase 01 dev
users `sec2@bank.test`, `vs@bank.test` and the key inventory entry `akr-sign-bundle` (SP-11); API.md s.2 scope
`siem.pull` (and what to do with `support.readonly`); phase 22 `NoticeCode` 15 -> 17 and the recipient rule "active
holders of a role"; phase 04 `role_grant` operation `revoke`, target `platform_release` with its `EvidenceResolver`;
phase 00 zones `z2_bank` (akr-worker egress to SIEM and TSA) and `z3_idp` (kratos egress to the issuer); phase 24 control
report gains two counts; phase 23 disposal handlers for the eleven tables; MODULES M3 "SEC + M" vs CH + SEC (bank).
