p='docs/plan/phase-25.md'
s=open(p).read()
R=[
# 1 unique per requester
("""-- One open grant per case at a time (requested or active).
CREATE UNIQUE INDEX IF NOT EXISTS uq_break_glass_open_case ON admin.break_glass_grants (case_id)
    WHERE status IN ('requested', 'active');""",
"""-- One open request or grant per requester and case (lead review: a per-case index let one requester hold the path shut
-- and told a second requester that a request was open).
CREATE UNIQUE INDEX IF NOT EXISTS uq_break_glass_open_case ON admin.break_glass_grants (case_id, requester_id)
    WHERE status IN ('requested', 'active');"""),
("""3. Insert the request (status `requested`, `request_expires_at = now() + break_glass.request_ttl_min`) with `ON CONFLICT DO NOTHING` on the open-grant unique index. The case id is stored as given, with no existence check and no foreign key (DR-233).
4. Audit `break_glass.requested` (resource: the grant; details: `reason_code`, `scope_kind = case`). The same audit row is written whether or not the case exists and whether or not an open grant already exists for the id.""",
"""3. Insert the request (status `requested`, `request_expires_at = now() + break_glass.request_ttl_min`). The unique index is per requester and case: a requester who already has an open request or grant for the id gets the same 202 and the existing row (an idempotent repeat); another requester's open request has no effect on this one. The case id is stored as given, with no existence check and no foreign key (DR-233).
4. Audit `break_glass.requested` (resource: the inserted or existing grant of this requester; details: `reason_code`, `scope_kind = case`). The same audit row is written whether or not the case exists."""),
("and by the one-open-request-per-case index,","and by the one-open-request-per-requester-and-case index,"),
# 2 allow-list
("""// A grantee cannot change, export or hand on a case opened under break-glass (SP-24 "no export"; section 3.2 "Use").
@id("bg.read_only")
forbid (principal, action in [Akr::Action::"case.assign", Akr::Action::"case.merge", Akr::Action::"case.note.write",
        Akr::Action::"case.evidence.add", Akr::Action::"case.propose", Akr::Action::"case.decide",
        Akr::Action::"case.escalate", Akr::Action::"case.close", Akr::Action::"case.reopen",
        Akr::Action::"case.reopen_late", Akr::Action::"case.access.change", Akr::Action::"case.work.write",
        Akr::Action::"case.work.waive", Akr::Action::"case.link.write", Akr::Action::"pack.create",
        Akr::Action::"pack.download"], resource)
when { context.purpose == "break_glass" };""",
"""// A grantee cannot change, export or hand on a case opened under break-glass (SP-24 "no export"; section 3.2 "Use").
// An allow-list of reads (lead review): team membership also feeds later team checks (phase 19 Rfi.assignees), so a
// list of forbidden writes would miss actions added later.
@id("bg.read_only")
forbid (principal, action, resource)
when { context.purpose == "break_glass" &&
       !(action in [Akr::Action::"case.read", Akr::Action::"case.note.read", Akr::Action::"case.evidence.read",
                    Akr::Action::"case.chain.read", Akr::Action::"case.link.read", Akr::Action::"case.work.read",
                    Akr::Action::"case.access.read", Akr::Action::"alert.read", Akr::Action::"alert.snapshot.read",
                    Akr::Action::"party.read", Akr::Action::"party.transactions.read", Akr::Action::"network.read",
                    Akr::Action::"rfi.read", Akr::Action::"rfi.read.case", Akr::Action::"report.read"]) };"""),
("""3. Read-only: file `51_platform.cedar` forbids every write, decision, access change, pack creation and pack download when `context.purpose == "break_glass"` (section 3.12).""",
"""3. Read-only: file `51_platform.cedar` forbids every action outside a list of fifteen read actions when `context.purpose == "break_glass"` (section 3.12), so writes, decisions, access changes, RFIs, packs and downloads are refused, including actions later phases add."""),
# 3 SIEM tier
("""    resource_key    TEXT,
    tier            TEXT,
    details         JSONB
)
LANGUAGE plpgsql STABLE SECURITY DEFINER SET search_path = pg_catalog, audit AS $$""",
"""    resource_key    TEXT,
    details         JSONB
)
LANGUAGE plpgsql STABLE SECURITY DEFINER SET search_path = pg_catalog AS $$"""),
("""           CASE WHEN t.tier = 'normal' THEN l.resource_key END,
           t.tier,
           CASE""","""           CASE WHEN t.tier = 'normal' THEN l.resource_key END,
           CASE"""),
("""REVOKE ALL ON FUNCTION audit.siem_page(TIMESTAMPTZ, UUID, INT, TEXT[]) FROM PUBLIC;""",
"""ALTER FUNCTION audit.siem_page(TIMESTAMPTZ, UUID, INT, TEXT[]) OWNER TO akr_migrate;
REVOKE ALL ON FUNCTION audit.siem_page(TIMESTAMPTZ, UUID, INT, TEXT[]) FROM PUBLIC;"""),
("-- every row whose resource tier is not normal, and never returns case_id.",
 "-- every row whose resource tier is not normal, and never returns case_id or the tier value (lead review)."),
("resource_id: Option<Uuid>, resource_key: Option<String>, tier: Tier, details }","resource_id: Option<Uuid>, resource_key: Option<String>, details }"),
("""`{"time", "audit_id", "action", "severity", "actor_id", "on_behalf_of", "resource_type", "resource_id", "resource_key", "tier", "details"}`; `resource_id`, `resource_key` and `details` are null or `{}` unless `tier = normal` (below).""",
"""`{"time", "audit_id", "action", "severity", "actor_id", "on_behalf_of", "resource_type", "resource_id", "resource_key", "details"}`; `resource_id`, `resource_key` and `details` are null or `{}` unless the resource tier is normal (below). The tier value itself is never sent."""),
("""no event carries a `case_id` key; every event whose `tier` is not `normal` has `resource_id` null and `details` `{}`;""",
 """no event carries a `case_id` or `tier` key; the fixture event on `S1` has `resource_id` null and `details` `{}`;"""),
("""returns no `case_id` column, returns `resource_id` NULL""","""returns no `case_id` or `tier` column, returns `resource_id` NULL"""),
# 4 requester roles
("""**Request** (`POST /access/break-glass`; Cedar `break_glass.request`, roles A2, M, SEC or CH):""",
 """**Request** (`POST /access/break-glass`; Cedar `break_glass.request`, role A2 only: phase 17 rule 1 admits only A1, A2 and M to a case row, M needs no grant, so a grant to SEC or CH would open nothing; lead review):"""),
("""4. The approver must not be the requester (`E_SELF_APPROVAL`, 403). The requester may be SEC or CH, which is why a requester of those roles is tested by this rule.""",
 """4. The approver must not be the requester (`E_SELF_APPROVAL`, 403): a person holding A2 and an approving role cannot approve their own request."""),
("""// Break-glass (SP-24, M3). The requester is A2, M, SEC or CH; approval is CH or SEC; the approver rule is in the handler.
@id("bg.request.a2") @role("A2")
permit (principal, action == Akr::Action::"break_glass.request", resource is Akr::System)
when { principal.roles.contains("A2") };

@id("bg.request.m") @role("M")
permit (principal, action == Akr::Action::"break_glass.request", resource is Akr::System)
when { principal.roles.contains("M") };

@id("bg.request.sec") @role("SEC")
permit (principal, action == Akr::Action::"break_glass.request", resource is Akr::System)
when { principal.roles.contains("SEC") };

@id("bg.request.ch") @role("CH")
permit (principal, action == Akr::Action::"break_glass.request", resource is Akr::System)
when { principal.roles.contains("CH") };
""",
"""// Break-glass (SP-24, M3). The requester is A2 (phase 17 rule 1); approval is CH or SEC; the approver rule is in the handler.
@id("bg.request.a2") @role("A2")
permit (principal, action == Akr::Action::"break_glass.request", resource is Akr::System)
when { principal.roles.contains("A2") };
"""),
("""@id("bg.end.requester") @role("A2")
permit (principal, action == Akr::Action::"break_glass.end", resource is Akr::System)
when { principal.roles.contains("A2") };

@id("bg.end.m") @role("M")
permit (principal, action == Akr::Action::"break_glass.end", resource is Akr::System)
when { principal.roles.contains("M") };

@id("bg.end.ch") @role("CH")
permit (principal, action == Akr::Action::"break_glass.end", resource is Akr::System)
when { principal.roles.contains("CH") };
""",
"""@id("bg.end.a2") @role("A2")
permit (principal, action == Akr::Action::"break_glass.end", resource is Akr::System)
when { principal.roles.contains("A2") };   // the handler admits the requester's own grant only
"""),
("""- New section "Emergency access" (A2, M, SEC and CH request; SEC and CH approve; SEC reviews)""",
 """- New section "Emergency access" (A2 requests; SEC and CH approve; SEC reviews)"""),
("""Requesters are `A2`, `M`, `SEC` and `CH`, so a requester who holds an approving role is refused as an approver by the rule that a requester never approves (the test of section 6 uses this case).""",
 """The requester is `A2` only: phase 17 rule 1 admits only A1, A2 and M to a case row, and M needs no grant, so a grant to SEC or CH would open nothing (lead review). A person who holds A2 and an approving role is refused as approver of their own request."""),
("""   - **Break-glass refused when the requester approves**: `sec@` calls `POST /api/v1/access/break-glass` with `case_id = N1` (step-up `break_glass`) → `202` `{"status":"requested"}` with no id; `sec@` calls `POST /api/v1/access/break-glass/{id}/approve` on the same request → `403` `E_SELF_APPROVAL`. The id is read from the database in the fixture.""",
 """   - **Break-glass refused when the requester approves**: `a2@` calls `POST /api/v1/access/break-glass` with `case_id = N1` (step-up `break_glass`) → `202` `{"status":"requested"}` with no id; `a2@` calls `POST /api/v1/access/break-glass/{id}/approve` on the same request → `403` (`E_SCOPE` at the gate, as A2 holds neither `security.ops` nor `dashboard.read`); `sec@` calling `POST /api/v1/access/break-glass` → `403` `E_FORBIDDEN` (no permit). The handler's `E_SELF_APPROVAL` for a person holding A2 and SEC is a unit test. The id is read from the database in the fixture."""),
# 5 purpose in SQL clause
("""`OR EXISTS (SELECT 1 FROM admin.break_glass_grants g WHERE g.case_id = c.id AND g.requester_id = $1 AND g.status = 'active' AND g.expires_at > now())`.""",
 """`OR ($purpose = 'break_glass' AND EXISTS (SELECT 1 FROM admin.break_glass_grants g WHERE g.case_id = c.id AND g.requester_id = $1 AND g.status = 'active' AND g.expires_at > now()))` (the purpose condition keeps the clause equal to the Cedar adapter of step 2, lead review)."""),
# 6 step-up class
("""**Approve** (`POST /admin/support-sessions/{id}/approve`; Cedar `support_session.approve`, role SEC; step-up class `approval`, action `support_session.approve`; 4E):""",
 """**Approve** (`POST /admin/support-sessions/{id}/approve`; Cedar `support_session.approve`, role SEC; step-up class `access_change` (it creates a role grant), action `support_session.approve`; 4E):"""),
("""| Approve; **SU** `approval`; the VS grant starts with the session;""","""| Approve; **SU** `access_change`; the VS grant starts with the session;"""),
# 7 vs@ expectations
("""`vs@` calls `GET /api/v1/cases/{N1}` → `403` `E_FORBIDDEN`; `vs@` calls `GET /api/v1/cases/{S1}` → `404` `E_NOT_FOUND`;""",
 """`vs@` calls `GET /api/v1/cases/{N1}` and `GET /api/v1/cases/{S1}` → `403` `E_SCOPE` for both (VS holds no `queue.work`; the gate answers before any lookup, so nothing tells the two apart);"""),
("""A case read by VS answers 403 `E_FORBIDDEN` for a normal case and the masked 404 for a suspicion case.""",
 """A case read by VS answers 403 `E_SCOPE` at the gate for every case id (VS holds no `queue.work`), so no answer tells a normal case from a suspicion case or a missing one."""),
]
for a,b in R:
    assert s.count(a)==1, a[:80]
    s=s.replace(a,b)
open(p,'w').write(s)
print('ok',len(R))
