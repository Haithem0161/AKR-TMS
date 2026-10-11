#!/usr/bin/env python3
"""Owner-decision round 1 fixes: phase 25 (verdicts O-25-1..9, L-7 in tools/agent/owner-verdict.md) and the system
documents that follow the owner blocks of phases 10, 25 and 26."""
import sys
sys.path.insert(0, "/home/haithem/Projects/AKR-TMS/tools/agent")
from ofix import Fixer

f = Fixer(round_no=1)
P = "docs/plan/phase-25.md"

# O-25-1 (HIGH): the CHECK was false for every open row; both copies (section 1.3 and section 7.24 item 1)
f.fix("O-25-1", P,
      "CHECK ((resolution = 'marked') IS NOT DISTINCT FROM (resolution_code IS NOT NULL))",
      "CHECK (((resolution = 'marked') IS TRUE) = (resolution_code IS NOT NULL))   -- a code on a marked row only; an open row has neither",
      n=2)

# O-25-2: the size in the route-count bullet
f.fix("O-25-2", P,
      "The size of the phase is L. The owned `GET /api/v1/admin/releases` row",
      "The size of the phase is XL (25 routes or more; section 7.24 item 2). The owned `GET /api/v1/admin/releases` row")

# O-25-3: what the off switch holds back, and a test that can pass
f.fix("O-25-3", P,
      "(insert or update, `forwarded_total` unchanged) and end the pass. Nothing read while the export was off is sent later.",
      "(insert or update, `forwarded_total` unchanged) and end the pass. An access event recorded before an off-run is never sent. The cursor moves only when a run executes, so an event recorded after the last off-run (at most one run interval, 10 seconds, before the setting is `true` again) is sent by the first run that finds the setting `true`.")
f.fix("O-25-3", P,
      "with the setting `false` no access line is sent and, after it is set back to `true`, the events written meanwhile are not sent.",
      "with the setting `false` no access line is sent; the test writes two access events, lets one more run pass with the setting `false`, sets it back to `true`, and the two events are not sent, while an event written after that off-run is sent.")

# O-25-4: the trailer names both places that declare actions
f.fix("O-25-4", P,
      "`schema.cedarschema` receives the action declarations of section 3.12 only (the appended-declarations pattern",
      "`schema.cedarschema` receives the action declarations of section 3.12 and of section 7.24 item 3 only (the appended-declarations pattern")

# O-25-5: every access event is exported, not only the allowed ones (API.md, MODULES.md, DATA-MODEL.md)
f.fix("O-25-5", "docs/API.md",
      "access lines (allowed reads of sensitive routes) after a cursor",
      "access lines (every access event: allow, deny and conflict) after a cursor")
f.fix("O-25-5", "docs/MODULES.md",
      "Owner decisions of 2026-10-10: allowed reads of sensitive routes are exported too, through SECURITY DEFINER",
      "Owner decisions of 2026-10-10: record reads are exported too (every access event: allow, deny and conflict), through SECURITY DEFINER")
f.fix("O-25-5", "docs/DATA-MODEL.md",
      "Owner decisions of 2026-10-10: allowed reads of sensitive routes are exported too (DR-414). They are read from",
      "Owner decisions of 2026-10-10: record reads are exported too, as every access event with the decision allow, deny\nor conflict (DR-414). They are read from")

# O-25-6: a nullable column needs Option
f.fix("O-25-6", P,
      "actor_id, actor_role, on_behalf_of, resource_type, resource_id: Option<Uuid>, subject_party: Option<Uuid>, protected: bool,",
      "actor_id, actor_role, on_behalf_of: Option<Uuid>, resource_type, resource_id: Option<Uuid>, subject_party: Option<Uuid>, protected: bool,")

# L-7 (lead find while fixing O-25-6): the same in the base struct; audit_log.actor_id and on_behalf_of are nullable
f.fix("L-7", P,
      "`SiemLine { occurred_at, audit_id, action, severity: SiemSeverity, actor_id, on_behalf_of, resource_type, resource_id: Option<Uuid>,",
      "`SiemLine { occurred_at, audit_id, action, severity: SiemSeverity, actor_id: Option<Uuid>, on_behalf_of: Option<Uuid>, resource_type, resource_id: Option<Uuid>,")

# O-25-7: the status of the missing If-Match on resolve
f.fix("O-25-7", P,
      "1. Cedar `siem_dead_letter.resolve`. `If-Match` is required. `resolution_code` is one of",
      "1. Cedar `siem_dead_letter.resolve`. `If-Match` is required (428 `E_PRECONDITION_REQUIRED`). `resolution_code` is one of")

# O-25-8: who writes the three kinds
f.fix("O-25-8", P,
      "The three kinds that the forwarder's own work produces are not exported,",
      "The three kinds of the resend work (the request writes the first, the forwarder the other two) are not exported,")

# O-25-9: every field of a protected line is named in the review
f.fix("O-25-9", P,
      "An access line of a protected row carries the time, the actor, the role, the verb, the resource kind, the purpose code, the client address and three flags.",
      "An access line of a protected row carries the time, the access id, the decision, the actor, the role, the on-behalf-of id (a staff id, when the read used a delegated role), the verb, the resource kind, the purpose code, the client address and three flags. The access id is the id of the access event and opens nothing in the SIEM.")

# System documents in step with the owner blocks of phases 10 and 26 (grants of akr_ingest)
f.fix("DOC-10", "docs/DATA-MODEL.md",
      "`superseded_at` of the three party child tables,\n`loaded_last_at` of `party_contact`, and",
      "`superseded_at` of the three party child tables,\n`capture_source` of `party_name` (DR-420; phase 10 section 7.26), `loaded_last_at` of `party_contact`, and")
f.fix("DOC-26", "docs/DATA-MODEL.md",
      "with UPDATE only on `(paid_on, paid_amount, row_hash)`; `akr_app` has SELECT;",
      "with UPDATE only on `(paid_on, paid_amount, row_hash)`, and UPDATE on `canon.facility (granted_on, row_hash)`; `akr_app` has SELECT;")

f.done()
