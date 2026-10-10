p='docs/MODULES.md'
s=open(p).read()
def rep(a,b):
    global s
    assert s.count(a)==1,(s.count(a),a[:80])
    s=s.replace(a,b)
# M1
rep("""local session (session revoked at next check); MFA device lost (recovery only through ADM + SEC, audited); clock skew
on TOTP (window of one step).
""","""local session (session revoked at next check); MFA device lost (recovery only through ADM + SEC, audited); clock skew
on TOTP (window of one step).
Phase file: docs/plan/phase-25.md (DR-239): the generic OIDC upstream (Entra ID, ADFS, Keycloak) as Kratos
configuration; a federated person is linked to a local identity only by an audited admin command, never by e-mail;
roles never come from the upstream and its `amr` never satisfies AAL2, step-up or `record_open`. Stated limit: an
upstream disable reaches the platform only through unlink or offboarding, so a live session ends at the idle timeout
or the 8 h lifetime (phase 25 Open doubts 4). SAML and SCIM: phase 31.
""")
# M2
rep("""admin decision, history kept).
""","""admin decision, history kept).
Phase file: docs/plan/phase-25.md (DR-234): the quarterly access review snapshots `AccessMatrixBuilder::build(at)`
and the grants in force as one evidence object; SEC decides every item and M the business-role items, nobody their
own grant; a revocation is a phase 04 change request (`role_grant` op `revoke`, route `cr_access_adm`); the signer is
not the starter (step-up `approval`); overdue opens one phase 24 register warning. Inspector accounts reuse role `AS`
with `valid_to` (DR-241).
""")
# M3 approvers
rep("two approvers (SEC + M)","two approvers (CH + SEC; M notified; phase 25 DR-242)")
rep("""(re-checked at download); policy bundle with a syntax error (refused, previous bundle stays).
""","""(re-checked at download); policy bundle with a syntax error (refused, previous bundle stays).
Phase file: docs/plan/phase-25.md (DR-232, DR-233, DR-242, DR-243): break-glass is its own table, not a `case_acl`
row; the requester is A2, the approvers one CH and one SEC (never the requester; step-up `break_glass`); the requester
reaches the case through the team clause only under purpose `break_glass`, read-only (Cedar allow-list of 15 read
actions), at most four hours (database CHECK, predicate stops at `expires_at`), every use in the audit trail and the
dedicated log, review within 24 hours, two count-only notice codes to SEC, CH and M; every request answers the same
202, so nothing tells whether a case exists. Screen G5 gains the break-glass list and the access-review tiles.
""")
# M4
rep("""for IA and SEC through the SECURITY DEFINER `audit.resource_tier`, which mirrors phase 17 tier rule 2; screen G6.
""","""for IA and SEC through the SECURITY DEFINER `audit.resource_tier`, which mirrors phase 17 tier rule 2; screen G6.
Phase file: docs/plan/phase-25.md (DR-235, DR-241, DR-245): SIEM forwarding (CEF or JSON over TLS syslog from
akr-worker, zone `z2_bank`) and the pull API `GET /siem/events` (scope `siem.pull`, service principals), both through
SECURITY DEFINER `audit.siem_page`: `siem`-flagged kinds only, no case id, no tier value, resource fields dropped for
every non-normal row; a cursor that survives restarts, dead letters by id; the RFC 3161 time stamp per anchor (off by
default).
""")
# M20
rep("""floors); vendor support asking for case data (no route).
""","""floors); vendor support asking for case data (no route).
Phase file: docs/plan/phase-25.md (DR-236 to DR-240, DR-244): release bundles imported and verified by the phase 00
`bundle verify` code path, approved by change request `platform_release` on `cr_access`, applied by the operator and
recorded (no call-home); vendor support sessions (at most 8 h, SEC approval with step-up `access_change`, the `VS`
grant can only read its own profile, automatic end); support bundles built by `akr-ctl support-bundle` (allow-list,
per-bundle pseudonym key, signed, deleted after 30 days, disclosure record kept); help articles and "what changed"
shipped in the signed bundle; ops tiles for SIEM lag and dead letters, release currency, open support sessions and
active break-glass grants; screens A4 and X5.
""")
open(p,'w').write(s)
print('ok')
