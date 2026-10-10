p='docs/API.md'
s=open(p).read()
def rep(a,b):
    global s
    assert s.count(a)==1,(s.count(a),a[:80]); s=s.replace(a,b)
old_start=s.index("| GET | /assess/fail-policies | config.author | Fail policy per rail")
old_end=s.index("| POST | /assess/control-tests | admin.system | Record the weekly control-test payment result |\n")+len("| POST | /assess/control-tests | admin.system | Record the weekly control-test payment result |\n")
new="""| GET | /assess/fail-policies | config.author, config.approve, dashboard.read | Fail policy matrix per rail, corridor, currency and amount band, with the amount bands |
| PUT | /assess/fail-policies | config.author | Propose a new policy set; **CR** `cr_material` (object `assess_policy`) |
| GET | /assess/modes | config.author, config.approve, dashboard.read | Mode per rail (shadow, advisory, enforcing) with effective-from and readiness snapshot |
| PUT | /assess/modes/{rail} | config.author | Propose a mode change; **CR** `cr_material` (object `assess_mode`); refused when the entry criteria fail |
| GET | /assess/modes/{rail}/readiness | config.author, config.approve | Entry criteria for the next mode, computed from the roll-ups (phase 29, added) |
| GET | /assess/holds | screening.work | Hold queue: held, advisory and shadow review rows |
| GET | /assess/holds/{id} | screening.work | One hold with its candidates, names for SS and M, and the proposal |
| POST | /assess/holds/{id}/claim | screening.work | Claim; `If-Match` |
| POST | /assess/holds/{id}/decision | screening.work | Release or reject: proposal, then approval by a second person; **SU** class `decision`, **4E** |
| GET | /assess/decisions | audit.read | Search of signed decisions; purpose required; party-linked rows masked |
| GET | /assess/release-without-decision | dashboard.read | Open release exceptions: no decision, no match key, pass expired, decision not a pass (C-59) |
| GET | /assess/blind-periods | dashboard.read | Outage periods of the assessment service with duration (C-58; phase 29, added) |
| GET | /assess/health | admin.system | Nodes, mode per rail, list freshness, degraded counters |
| POST | /assess/control-tests | admin.system | Record the weekly control-test payment result |

Phase 29 owns the `/assess` rows (docs/plan/phase-29.md s.2; 14 rows, 2 added; DR-292 to DR-316). Every route is 404
`E_NOT_FOUND` while flag `pack.assess` is off. Fail policies with bands, modes and prohibited rules are change targets
on `cr_material` with no emergency change (DR-300); prohibited rules have no route and are proposed through phase 04.
"""
s=s[:old_start]+new+s[old_end:]
rep("| GET | /assess/v1/health | assess.call | Mode per rail, list freshness, degraded flags |\n",
    "| GET | /assess/v1/health | assess.call | Mode per rail, list freshness, degraded flags |\n\n"
    "Phase 29 owns these rows (DR-292 to DR-316). The gateway is a service principal bound to a pinned certificate\n"
    "(DR-292); the routes answer 404 `E_NOT_FOUND` while flag `pack.assess` is off; `GET /assess/v1/decisions/{id}` and\n"
    "`GET /assess/v1/holds/{hold_ref}` answer only for the calling gateway.\n")
rep('  "format": "mt103" | "pacs008" | "rtgs" | "ach" | "canonical",',
    '  "format": "mt103" | "pacs008" | "canonical",     // rails rtgs and ach send canonical until samples exist')
rep('  "degraded": [],                                    // e.g. ["ml_unavailable", "lists_stale"]',
    '  "degraded": [],                                    // e.g. ["lists_stale", "screening_partial"]')
rep('  "customer_message_code": "PENDING_REVIEW"          // generic, never the reason\n}',
    '  "customer_message_code": "PENDING_REVIEW",         // generic, never the reason\n'
    '  "fail_policy": "open" | "closed",                 // the matching fail-policy row, for the gateway\'s own copy\n'
    '  "policy_version": 12                              // the policy set the decision used\n}')
rep("- **Idempotency.** The same key and payload return the stored response byte for byte;",
    "- **Idempotency.** Keys are scoped to the calling gateway. The same key and payload return the stored signed response\n  byte for byte (original `expires_at`);")
rep("  payload -> 422 `E_IDEMPOTENCY_REUSE`; the same key while the first call runs -> 409 `E_IDEMPOTENCY_IN_FLIGHT`.",
    "  payload -> 422 `E_IDEMPOTENCY_REUSE`; the same key while the first call runs -> 409 `E_IDEMPOTENCY_IN_FLIGHT`. After the\n  replay window (`assess.idempotency_hours`) the gateway reconciles with `GET /assess/v1/decisions/{id}`.")
rep("- **Expiry.** A `pass` is valid only until `expires_at` (default 10 minutes); a gateway releasing later must call\n  again.",
    "- **Expiry.** A `pass` is valid only until `expires_at` (default 10 minutes); a gateway releasing later must call\n  again. The release-without-decision control raises `pass_expired` for a release after `expires_at` plus\n  `assess.release_skew_s`.")
rep("- **Degraded.** Lists older than their refresh threshold turn cross-border decisions into `hold` with\n  `lists_stale`;",
    "- **Degraded.** Lists stale by the phase 14 freshness rule turn cross-border decisions into `hold` with\n  `lists_stale`;")
open(p,'w').write(s)
print('ok')
