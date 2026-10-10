p='docs/plan/roadmap.md'
s=open(p).read()
R=[
("| 25 | Platform Growth & Releases | OIDC upstream, SIEM forwarding, quarterly access review, break-glass with second approver, release and update screen (A4), support sessions, ops cockpit v1 | M | P1 | 04 | Planned |",
 "| 25 | Platform Growth & Releases | Quarterly access review (evidence snapshot, SEC and M decide, revocations by change request); break-glass (A2 requests, CH and SEC approve, read-only allow-list under purpose `break_glass`, four hours by CHECK, 24-hour review, dedicated log, no existence signal); SIEM forwarding and pull API through SECURITY DEFINER `audit.siem_page` (no case id, no tier); generic OIDC upstream as Kratos configuration; signed release import, verification, change-request approval and the operator's apply record; vendor support sessions (`VS` reads only its own profile) and support bundles; help centre shipped in the bundle; ops tiles; RFC 3161 option; screens A4, X5 | M | P1 | 04, 17-19, 22, 24 | Spec ready |"),
("| 25 | _(none expected — settings, audit and existing tables; confirmed when the phase file is written)_ |",
 "| 25 | `admin.access_reviews`, `admin.access_review_items`, `admin.break_glass_grants`, `admin.break_glass_log`, `admin.releases`, `admin.help_articles`, `admin.support_sessions`, `admin.support_bundles`, `admin.siem_cursors`, `admin.siem_dead_letters`, `admin.anchor_timestamps` (`admin.role_grants.support_session_id`; function `audit.siem_page`) |"),
("| 25 | `OidcUpstream`, `SiemForwarder`, `AccessReviewService`, `BreakGlassService`, `ReleaseService` |",
 "| 25 | `AccessReviewService`, `BreakGlassService`, `ReleaseService`, `SupportSessionService`, `HelpCentreService`, `SiemForwarder` (with `SiemPageReader`), `OidcUpstream` (configuration and `akr-ctl oidc upstream check`), `AnchorTimestamper`; tiles on phase 03's `OpsCockpit::snapshot()` |"),
("| 26 | next numbers (from 0080) | 23 |\n| 27-28 | next numbers | 24 |\n| 29 | next numbers | 25 |",
 "| 25 | `0080_admin_access_review_break_glass`, `0081_admin_releases_support_help`, `0082_admin_siem_guards_grants` | 23 |\n| 26 | next numbers (from 0083) | 24 |\n| 27-28 | next numbers | 25 |\n| 29 | next numbers | 26 |"),
("Phase 04 ──▶ Phase 25 (any time in P1)          Phases 10 + 16 ──▶ Phase 26 (optional)",
 "Phases 04 + 22 + 24 ──▶ Phase 25                 Phases 10 + 16 ──▶ Phase 26 (optional)"),
("phases 19, 20, 21 and 23 run in parallel after phase 18; phase 25 can run at any point.",
 "phases 19, 20, 21 and 23 run in parallel after phase 18; phase 25 follows phase 24 (break-glass reaches cases through\nphases 17 and 18, its notices use phase 22, its counts feed the phase 24 control report; DR-232)."),
]
for a,b in R:
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
row24=[l for l in s.split('\n') if l.startswith('| 0 (B7 review: phase 24)')][0]
new=("| 0 (B7 review: phase 25) | 2026-10-09 | 7 lead corrections to the Haiku draft | 0 | 0 | 4 | 3 | MEDIUM: a break-glass request "
"was unique per case with `ON CONFLICT DO NOTHING`, so a second requester's request was dropped while audited, their empty list "
"told them another request was open, and one A2 could hold a case's emergency path shut (now unique per case and requester); the "
"read-only rule forbade 16 named writes, but the requester joins `Case.assignees`, which later team checks read (phase 19 RFI "
"drafting), so it is now an allow-list of 15 reads under purpose `break_glass`; `audit.siem_page` returned the tier, so the label "
"`suspicion` would leave for the SIEM, and it had no owner and an `audit` search path (now no tier, owner `akr_migrate`, "
"`search_path = pg_catalog`); M, SEC and CH could request a grant that phase 17 rule 1 makes useless (requester now A2 only). "
"LOW: the visibility clause lacked the purpose `break_glass` that the Cedar adapter requires; the support approval step-up class "
"is `access_change`, not `approval`; `VS` case reads answer `E_SCOPE` |")
assert 'B7 review: phase 25' not in s
s=s.replace(row24+'\n',row24+'\n'+new+'\n')
open(p,'w').write(s)
print('ok')
