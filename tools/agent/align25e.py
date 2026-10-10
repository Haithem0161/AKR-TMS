p='docs/plan/status.md'
s=open(p).read()
R=[
("phases 09 to 24 (batches B1 to B7; phases 19, 20, 21 and 23 are B6, phases 22 and 24 are B7) reviewed 2026-10-09.",
 "phases 09 to 25 (batches B1 to B7; phases 19, 20, 21 and 23 are B6, phases 22, 24 and 25 are B7) reviewed 2026-10-09."),
("| 25 | Platform Growth & Releases | Planned | | | 0 | | | 5 |",
 "| 25 | Platform Growth & Releases | Spec ready | | | 11 | `admin.role_grants` (`support_session_id`, source `support_session`); function `audit.siem_page`; phase 04 `role_grant` op `revoke` and target `platform_release`; phase 22 `NoticeCode` 15 to 17; phase 24 control report gains two counts; phase 00 zones `z2_bank`, `z3_idp`; phase 03 `OpsCockpit` tiles; phase 17/18 case read path writes `break_glass.used` (gap pass 1) | 21 (12 owned, 9 added) | 8 (AccessReviewService, BreakGlassService, ReleaseService, SupportSessionService, HelpCentreService, SiemForwarder, OidcUpstream, AnchorTimestamper) |"),
("| Domain tables (specified) | 0 | 225 |","| Domain tables (specified) | 0 | 236 |"),
("| Routes | 0 | 476 |","| Routes | 0 | 497 |"),
("| Phases specified | 0 | 25 |","| Phases specified | 0 | 26 |"),
("| Screens | 0 | 56 (X1, X3,","| Screens | 0 | 58 (X1, X3,"),
("B1, B2, B3, B4, B5, B6, R5, G1, G4, G5, G6, G8, G9, R4) |","B1, B2, B3, B4, B5, B6, R5, G1, G4, G5, G6, G8, G9, R4, A4, X5) |"),
("| 0 | 506 (some partial; `python3 tools/idcount.py 24`, phases 00-24:","| 0 | 513 (some partial; `python3 tools/idcount.py 25`, phases 00-25:"),
]
for a,b in R:
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
a=[l for l in s.split('\n') if l.startswith('| `fairness_monthly` | 24 |')][0]+'\n'
jobs="""| `access_review_snapshot` | 25 | Enqueued by `POST /admin/access-reviews`: `AccessMatrixBuilder::build(at)` and the grants in force as one evidence object, one item per grant |
| `access_review_sweep` | 25 | `schedule.access_review_sweep` (06:10): one overdue warning in the phase 24 register (fixed source key) |
| `break_glass_sweep` | 25 | `schedule.break_glass_sweep` (every minute): lapses old requests, records expiries, flags reviews overdue after `break_glass.review_hours` |
| `siem_forward` | 25 | `schedule.siem_forward` (every 10 seconds): reads `audit.siem_page` after the cursor, sends CEF or JSON over TLS syslog, dead-letters refused lines |
| `support_session_sweep` | 25 | `schedule.support_session_sweep` (every minute): ends sessions at `ends_at`, revokes the `VS` grant, refuses requests not approved in time |
| `support_bundle_purge` | 25 | `schedule.support_bundle_purge` (03:40): deletes archives older than `support.bundle_retention_days`; the record stays |
| `release_verify` | 25 | Enqueued by `POST /admin/releases/verify`: phase 00 `bundle verify` code path; writes the result or the first failing check; help articles written once |
| `anchor_timestamp` | 25 | Enqueued by the phase 02 anchor job when `audit.rfc3161_enabled`: one RFC 3161 token per anchor |
"""
assert '`access_review_snapshot` | 25' not in s
s=s.replace(a,a+jobs)
open(p,'w').write(s)
print('ok')
