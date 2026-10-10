p='docs/API.md'
s=open(p).read()
R=[
("""  `sse.reset`. Events carry ids only and go to the per-user channel of users who can see the resource; clients
  refetch through TanStack Query invalidation. Reconnect with `Last-Event-ID`.""",
"""  `sse.reset`. Events carry ids only and go to the per-user channel of users who can see the resource; clients
  refetch through TanStack Query invalidation. Reconnect with `Last-Event-ID`. As planned, the one hub is phase 17's
  `CountsHub` on `GET /events/counts` (3.12); `notification.new {notification_id}` travels on it (phase 22, DR-214);
  the path is settled in gap pass 1."""),
("""| GET | /me/notifications | self | In-app notifications (cursor) [P1, phase 22] |
| POST | /me/notifications/{id}/read | self | Mark read [P1, phase 22] |
""",
"""| GET | /me/notifications | self | In-app notifications, newest first, cursor, `unread` filter [P1, phase 22] |
| POST | /me/notifications/{id}/read | self | Mark one of my notices read; another user's id is 404 [P1, phase 22] |
| GET | /me/notifications/unread-count | self | My unread count [P1, phase 22, added] |
| GET | /me/notification-preferences | self | My e-mail mode (`immediate` or `digest`) [P1, phase 22, added] |
| PUT | /me/notification-preferences | self | Set my e-mail mode; `If-Match` [P1, phase 22, added] |
"""),
("""| GET | /liaison-reports | case.decide | Liaison monthly reports for review (phase 22) |
| POST | /liaison-reports/{id}/review | case.decide | Review with comments (phase 22) |
""",
"""| GET | /liaison-reports | case.decide | Liaison monthly reports for review, cursor; filters branch, status, period; Cedar restricts to M (phase 22) |
| POST | /liaison-reports/{id}/review | case.decide | Accept or return to draft with a comment; `If-Match` (phase 22) |
"""),
("""| GET | /branch/items | branch.window | Items for my branch with neutral statuses only |
| GET | /branch/items/{id} | branch.window | Item with the neutral question and deadline |
| POST | /branch/items/{id}/input | branch.window | Answer with evidence |
| POST | /branch/observations | branch.window | Record an observation (eight groups, companion ID, written acknowledgment) |
| POST | /branch/confidential-reports | branch.window | Confidential report to the MLRO; branch management cannot see it |
| GET | /branch/monthly-report | branch.window | Draft monthly liaison report |
| POST | /branch/monthly-report/sign | branch.window | Sign and send; **SU** |
""",
"""| GET | /branch/home | branch.window | My branch, my role, the due date, neutral counts (phase 22, added) |
| GET | /branch/items | branch.window | Items for my branch with neutral statuses only (phase 22) |
| GET | /branch/items/{id} | branch.window | Item with the neutral question and deadline (phase 22) |
| POST | /branch/items/{id}/evidence | branch.window | Stage one document for the item (multipart) (phase 22, added) |
| POST | /branch/items/{id}/input | branch.window | Answer as phase 19 claims, never self-corroborated (phase 22) |
| POST | /branch/observations | branch.window | Record an observation (eight groups, companion ID, written acknowledgment) (phase 22) |
| POST | /branch/confidential-reports | branch.window | Confidential report to the MLRO; branch management cannot see it (phase 22) |
| GET | /branch/monthly-report | branch.window | Draft monthly liaison report (phase 22) |
| POST | /branch/monthly-report/sign | branch.window | Sign and send; **SU** (phase 22) |

Phase 22 owns the rows marked "phase 22" here, in 3.1, 3.16 and 3.20 (docs/plan/phase-22.md s.2; 17 routes, DR-201 to
DR-217). Every `/branch/*` route answers 404, not 403 `E_SCOPE`, to a caller without `branch.window` (DR-206).
"""),
("""| GET | /admin/flags | admin.system | Feature flags |
""",
"""| GET | /admin/notification-templates | admin.system | Notice templates with their current version; edits by change request (phase 22, added) |
| GET | /admin/flags | admin.system | Feature flags |
"""),
]
for a,b in R:
    assert s.count(a)==1,a[:60]
    s=s.replace(a,b)
open(p,'w').write(s)
print('ok',len(R))
