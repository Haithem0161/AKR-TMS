p='docs/plan/phase-31.md'
s=open(p).read()
def rep(a,b,n=1):
    global s
    assert s.count(a)==n,(s.count(a),a[:90])
    s=s.replace(a,b)

# ---- finding 1 and 5: a directory deactivation disables (reversible), never offboards
rep("Phase 03 (`OffboardingService`, `StepUpService` classes",
    "Phase 03 (`UserAdminService::disable` and `enable`, `OffboardingService`, `StepUpService` classes")
rep("`OffboardingService::offboard_directory` added)", "`UserAdminService::disable_from_directory` added)")
rep("local step-up always; SCIM deactivation runs offboarding; section 3.11)",
    "local step-up always; SCIM deactivation disables the account, reversibly, and the final offboarding stays an administrator's act; sections 3.11, 3.12)")
rep("'offboarding_deferred', 'unknown_user')),", "'disable_deferred', 'unknown_user')),")
rep("'Directory rows kept after offboarding (who was provisioned when)'",
    "'Directory rows kept after the account is retired (who was provisioned when)'")
rep("- `OffboardingService::offboard_directory` (extends phase 03 `OffboardingService`, section 4.5).",
    "- `UserAdminService::disable_from_directory` (extends phase 03 `UserAdminService::disable`, sections 3.12 and 4.5).")
rep("""   - `deactivate`: set `directory_active = FALSE`. If linked and the account is not offboarded: `OffboardingService::offboard_directory` (section 3.12). Outcome `applied`; `deferred` with reason `offboarding_deferred` when a Kratos step is deferred (phase 03 enqueues `offboard_finalize`); `refused` with reason `last_holder` when the account is the last ADM or SEC holder (then `directory_active` stays `FALSE`, the link stays, and audit `directory.deactivate_refused` is written; the reconciliation of step 9 lists it for SEC). A pending user is only marked inactive.""",
    """   - `deactivate`: set `directory_active = FALSE`. If linked and the account is active: `UserAdminService::disable_from_directory` (section 3.12), which disables the account reversibly; it never offboards. Outcome `applied` (also when the account is already disabled or offboarded: nothing more is done); `deferred` with reason `disable_deferred` when the Kratos step is deferred (phase 03 enqueues `offboard_finalize`); `refused` with reason `last_holder` when the account is the last ADM or SEC holder (then `directory_active` stays `FALSE`, the link stays, and audit `directory.deactivate_refused` is written; the reconciliation lists it for SEC). A pending user is only marked inactive.""")
rep("""1. Linked, `directory_active = FALSE`, account not offboarded: run `offboard_directory` again; audit `directory.reconciled {count}`.""",
    """1. Linked, `directory_active = FALSE`, account still active: run `disable_from_directory` again; audit `directory.reconciled {count}`. Linked accounts disabled by the directory (audit `user.disabled` with `source: directory`) for longer than `directory.pending_alert_days` and not yet offboarded: audit `directory.disabled_stale {count}`; the list is `GET /directory/users?state=linked&directory_active=false` for ADM, who decides the final offboarding (phase 03 `POST /admin/users/{id}/offboard`, step-up `identity_admin`). The job never offboards.""")
rep("### 3.12 Offboarding from a directory (extends phase 03 `OffboardingService`; DR-350)",
    "### 3.12 Disable from a directory (extends phase 03 `UserAdminService::disable`; DR-350)")
old312 = s[s.index("`OffboardingService::offboard_directory(ctx: DirectoryCtx"):s.index("### 3.13 Errors")]
assert "Open doubts 16." in old312
s = s.replace(old312, """`UserAdminService::disable_from_directory(ctx: DirectoryCtx, user_id, reason_code = "directory_deactivated")`:
1. Target: a human account in status `active` (already `disabled` or offboarded: outcome `applied`, nothing done), not the last ADM or SEC holder (outcome `refused`, reason `last_holder`). The caller check of phase 03 (not the caller) does not apply: the caller is the directory, not a person.
2. One transaction, the phase 03 `disable` body: status `disabled`, reason `admin`; the user's tokens revoked (`admin`); grants, delegations and liaison assignments stay (a suspension, not an offboarding). No step-up is consumed: phase 03 `disable` consumes none, and the act is reversible. Audit `user.disabled {reason: admin, reason_code: "directory_deactivated", source: "directory"}` (phase 03 payload gains `source`).
3. After commit: `invalidate_user`, Kratos `revoke_all_sessions` and `set_state(inactive)`; a Kratos failure enqueues `offboard_finalize` (phase 03 section 4) and the outcome is `deferred` (reason `disable_deferred`).
4. The Cedar check is on the directory path only (section 3.17, E0: action `user.disable`).

Why a disable and not an offboarding: phase 03 offboarding is final (a returning leaver is a new account), and a directory event arrives with no person behind it. One forged or mistaken event, a leaked webhook secret or an IdP administrator's error would otherwise end every linked account for good. A disable removes access at once and can be undone by an administrator with step-up (phase 03 `enable`, refused while `directory_active` is `FALSE`). The final offboarding stays the phase 03 act of an administrator with step-up `identity_admin`; the reconciliation (section 3.11) lists the accounts waiting for it. Phase 03's last-holder rule still refuses the last ADM or SEC.

""")
rep("""   - `activate`: set `directory_active = TRUE`. A linked and offboarded account: `ignored` with reason `reactivation_ignored` (offboarding is final, phase 03 section 3.8 step 4). Otherwise `applied`. Activation never enables an account; enabling is the phase 03 act of an administrator.""",
    """   - `activate`: set `directory_active = TRUE`. A linked and offboarded account: `ignored` with reason `reactivation_ignored` (offboarding is final, phase 03 section 3.8 step 4). Otherwise `applied`. Activation never enables an account, including one the directory disabled; enabling is the phase 03 act of an administrator with step-up.""")
rep("| `directory.reconciled` | `directory` | false | `count` |",
    "| `directory.reconciled` | `directory` | false | `count` |\n| `directory.disabled_stale` | `directory` | true | `count` |")
rep("`user.offboarded` (phase 03; payload gains `source`), `user.updated` (phase 03)",
    "`user.disabled` (phase 03; payload gains `source`), `user.updated` (phase 03)")
rep("""// E0_directory.cedar (phase 31). The directory webhook is a service principal with the one scope directory.sync and
// Ctx.via = "webhook_hmac" (the value added to the phase 03 list by section 4.5). Account links are ADM acts.
@id("directory.event.webhook")
permit (principal, action in [Akr::Action::"directory.event", Akr::Action::"user.offboard"], resource)
when { principal.kind == "service" && context.via == "webhook_hmac" };""",
    """// E0_directory.cedar (phase 31). The directory webhook is a service principal with the one scope directory.sync and
// Ctx.via = "webhook_hmac" (the value added to the phase 03 list by section 4.5); the route scope admits only
// polis-directory. No @role: a service principal holds no role (the phase 29 gateway form). It may disable
// (reversible), never offboard. Account links are ADM acts.
@id("directory.event.webhook")
permit (principal, action in [Akr::Action::"directory.event", Akr::Action::"user.disable"], resource)
when { principal.kind == "service" && context.via == "webhook_hmac" };""")
rep("(4) the `admin.users` row (offboarding, phase 03 order)", "(4) the `admin.users` row (the phase 03 disable)")
rep("Offboarding keeps phase 03 order internally.", "The disable keeps phase 03 order internally.")
rep("""4. **Phase 03.** `OffboardingService::offboard_directory` (section 3.12) is added beside `offboard`; `user.offboarded` gains the payload key `source`.""",
    """4. **Phase 03.** `UserAdminService::disable_from_directory` (section 3.12) is added beside `disable`; `user.disabled` gains the payload key `source`. `OffboardingService` is unchanged: the directory never offboards.""")
rep("""- A linked account whose directory row is inactive shows the words "deactivated in the directory" and an explanation of what offboarding did; enabling it is disabled with that reason.""",
    """- A linked account whose directory row is inactive shows the words "deactivated in the directory" and an explanation: the account is suspended, its grants are kept, and an administrator decides whether to offboard it (the phase 03 action, with step-up). Enabling it is disabled with that reason.""")
rep("""14. **SCIM deactivate runs offboarding.** A `deactivate` event for a linked user who is not the last holder: 200 `outcome: applied`; `GET /admin/users/{id}` shows `disabled` with `disabled_reason: offboarded`; the user's sessions and tokens are gone (phase 03 counts); the audit row `user.offboarded` carries `source: "directory"` and `stepup_id: null`.""",
    """14. **SCIM deactivate disables, never offboards.** A `deactivate` event for a linked user who is not the last holder: 200 `outcome: applied`; `GET /admin/users/{id}` shows `disabled` with `disabled_reason: admin`; the user's sessions and tokens are gone (phase 03 counts); the grants are unchanged; the audit row `user.disabled` carries `source: "directory"`; no `user.offboarded` row exists. With the directory row set active again and an ADM `POST /admin/users/{id}/enable` with step-up: 200 (the account is back with its grants).""")
rep("""15. **SCIM activate after offboarding.** An `activate` event for that user: 200 `outcome: ignored, reason_code: reactivation_ignored`. `POST /admin/users/{id}/enable` on that user: 409 `E_NOT_ALLOWED_IN_STATE` (phase 03 rule `offboarded`).""",
    """15. **SCIM activate after an administrator's offboarding.** ADM offboards a directory-disabled user (phase 03 route, step-up `identity_admin`). An `activate` event for that user: 200 `outcome: ignored, reason_code: reactivation_ignored`. `POST /admin/users/{id}/enable` on that user: 409 `E_NOT_ALLOWED_IN_STATE` (phase 03 rule `offboarded`). The reconciliation run against a user disabled by the directory more than `directory.pending_alert_days` ago writes `directory.disabled_stale {count: 1}` and changes no account.""")
rep("and the phase 03 offboarding suite: no regression.", "and the phase 03 disable, enable and offboarding suites: no regression.")
rep("""**DR-350 (SCIM deactivation runs offboarding, without a step-up).** A directory deactivation of a linked account calls `OffboardingService::offboard_directory`, which runs the phase 03 steps in the phase 03 order and records `source: directory`. No step-up is consumed, because the operation only removes sessions, tokens, grants, delegations and liaison roles, and a removal cannot give anyone more access; the step-up classes stay closed. Phase 03's last-holder rule still refuses the last ADM or SEC. Reactivation never re-enables an offboarded account, because offboarding is final. The stricter alternative (a second approver for every deactivation) is Open doubts 16.""",
    """**DR-350 (SCIM deactivation disables the account; it never offboards).** A directory deactivation of a linked account calls `UserAdminService::disable_from_directory`, the phase 03 disable with `source: directory`: status `disabled` with reason `admin`, tokens and sessions revoked, grants kept. No step-up is consumed, as for every phase 03 disable, because the act is reversible. An unattended external signal never triggers an irreversible act: phase 03 offboarding is final, so one forged or mistaken event would otherwise end accounts for good. The final offboarding stays the phase 03 act of an administrator with step-up, and the daily reconciliation lists accounts disabled by the directory longer than `directory.pending_alert_days`. Phase 03's last-holder rule still refuses the last ADM or SEC, and an enable is refused while the directory row is inactive.""")
rep("A daily reconciliation repeats a failed offboarding, lists pending rows older than `directory.pending_alert_days` to the security officer,",
    "A daily reconciliation repeats a failed disable, lists pending rows and directory-disabled accounts older than `directory.pending_alert_days` (the second list to the administrator, who decides the offboarding),")
rep("""16. **Deactivation without step-up.** Section 3.12 removes access without a second approver. The bank may want a four-eyes rule on each directory deactivation; that would be a step-up class or a change request, and the closed class list of phase 03 would need a decision.""",
    """16. **Deactivation without step-up (lead decision 2026-10-09).** A directory deactivation disables the account (reversible, no step-up, like every phase 03 disable) and never offboards; the final offboarding is the phase 03 act of an administrator with step-up. The bank may still want automatic offboarding after a number of days; that would make an external signal final and is refused by default.""")
rep("24. **Phase 03 extensions.** The offboarding method `offboard_directory`, the payload key `source`,",
    "24. **Phase 03 extensions.** The disable method `disable_from_directory`, the payload key `source` of `user.disabled`,")

# ---- finding 2: edge paths
rep("Bank IdP (SAML) -> Polis assertion consumer service (`/sso/...` through Caddy)",
    "Bank IdP (SAML) -> Polis assertion consumer service (through Caddy, the pinned path)")
rep("- **Edge.** Caddy routes exactly `/sso/*` and `/scim/v2/*` to Polis. Every other Polis path answers 404 at the edge.",
    "- **Edge.** Caddy routes to Polis exactly the paths the browser and the bank IdP must reach in the pinned Polis version: the OAuth authorize endpoint (Kratos redirects the browser there), the SAML assertion consumer service and the SCIM 2.0 base path. The list is taken from the pinned version (Open doubts 13) and written in the catalogue entry; every other Polis path answers 404 at the edge. Caddy refuses `/api/v1/directory/events` (404): Polis reaches `akr-api` directly on `z3_edge` and is the route's only caller. Kratos reaches the Polis token and discovery endpoints on `z3_ory`, never through the edge.")
rep("and to `z3_edge` (Caddy reaches it on `/sso/*` and `/scim/v2/*`; it reaches `akr-api:8080` for `POST /directory/events` only)",
    "and to `z3_edge` (Caddy reaches it on the pinned paths of section 3.10; it reaches `akr-api:8080` for `POST /directory/events` only)")
rep("- **Edge** (Caddy, phase 00): `/sso/*` and `/scim/v2/*` to `polis` while the profile is rendered; every other path of the Polis host answers 404 at the edge.",
    "- **Edge** (Caddy, phase 00): the pinned authorize, assertion consumer and SCIM paths of section 3.10 to `polis` while the profile is rendered; every other path of the Polis host answers 404 at the edge; `/api/v1/directory/events` answers 404 at the edge in every profile (the phase 00 rule `/api/*` -> akr-api gains this one exclusion).")
rep("With it on, the render adds the `polis` service and the two Caddy paths and nothing else.",
    "With it on, the render adds the `polis` service and the pinned Caddy paths and nothing else. `curl -s -o /dev/null -w '%{http_code}' -X POST https://$EDGE/api/v1/directory/events` answers 404 with the profile on and off.")
rep("and only the `/sso` and `/scim/v2` paths at the edge, with its admin interface disabled.",
    "and only the pinned authorize, assertion consumer and SCIM paths at the edge, with its admin interface disabled; the webhook route is not reachable through the edge.")
rep("The exact header format of the directory-sync webhook signature is checked against the pinned version before the verifier is written; this section states no header name on purpose.",
    "The exact header format of the directory-sync webhook signature, and the authorize, assertion consumer and SCIM paths that the edge exposes, are checked against the pinned version before the verifier and the Caddy rules are written; this section states no header name or path on purpose.")

# ---- finding 3: practice score counts first attempts only
rep("""-- Per trainee and course: has the trainee passed a practice case of the course? Read by the training register (phase 24
-- section 3.10) as a display column only. It is a view inside schema train, so it cannot be read by any count of the
-- live schemas.
CREATE OR REPLACE VIEW train.practice_scores AS
SELECT a.user_id,
       c.course_code,
       bool_or(a.passed)     AS passed_once,
       max(a.attempted_at)   AS last_attempt_at,
       count(*)              AS attempts
FROM train.practice_attempts a
JOIN train.practice_cases c ON c.id = a.case_id
GROUP BY a.user_id, c.course_code;""",
    """-- Per trainee and course: practice results scored on each trainee's FIRST attempt per case only (the first attempt
-- returns the known answer and the expert path, so a later pass of the same case is practice, not a result). Read by
-- the training register (phase 24 section 3.10) as a display column only. It is a view inside schema train, so it
-- cannot be read by any count of the live schemas.
CREATE OR REPLACE VIEW train.practice_scores AS
WITH first_attempt AS (
    SELECT DISTINCT ON (a.user_id, a.case_id) a.user_id, a.case_id, a.passed, a.attempted_at
    FROM train.practice_attempts a
    ORDER BY a.user_id, a.case_id, a.attempted_at, a.id
)
SELECT f.user_id,
       c.course_code,
       bool_or(f.passed)                      AS passed_first,
       count(*) FILTER (WHERE f.passed)       AS cases_passed_first,
       count(*)                               AS cases_attempted,
       (SELECT count(*) FROM train.practice_attempts x JOIN train.practice_cases y ON y.id = x.case_id
         WHERE x.user_id = f.user_id AND y.course_code = c.course_code) AS attempts,
       max(f.attempted_at)                    AS first_attempt_at
FROM first_attempt f
JOIN train.practice_cases c ON c.id = f.case_id
GROUP BY f.user_id, c.course_code;""")
rep("reads `train.practice_scores` for the users on the page and returns `passed_once`, `attempts` and `last_attempt_at`.",
    "reads `train.practice_scores` for the users on the page and returns `passed_first`, `cases_passed_first`, `cases_attempted` and `attempts`. Only each user's first attempt per case is scored: the first attempt returns the known answer and the expert path, so a later pass of the same case is practice and stays in `train.practice_attempts` without counting.")
rep("""- One column "Practice" beside each course record: "passed" or "not passed" or "no attempt", with the date of the last attempt.""",
    """- One column "Practice" beside each course record: "passed on first attempt" with the number of cases passed first time out of the cases tried, "not passed on first attempt", or "no attempt"; repeat attempts are shown as a count labelled "practice repeats, not scored".""")
rep("The register shows a `practice` display column read from `train.practice_scores`,",
    "The register shows a `practice` display column read from `train.practice_scores`, which scores only each trainee's first attempt per case (a pass after the known answer was shown is not a pass),")

# ---- finding 4: alert-to-pool attribution is this phase's rule
rep("`casework.touch_by_hours`, the queue routing rule)", "`casework.touch_by_hours`, the queue `pool_id` and `filter`)")
rep("""1. Alerts. The pool of an alert is the pool of the first active `alerts` queue, by `code`, whose filter admits the alert: the phase 17 routing rule (the same rule that gives a case its `assignee_pool_id`). An alert without an admitting queue is not counted in any team row.""",
    """1. Alerts. Phase 17 gives alerts no pool (queues have `pool_id` and `filter`; only cases have `assignee_pool_id`), so this phase sets its own attribution rule, new in phase 31 and used for capacity only: an alert counts, once, in the pool of the first active `alerts` queue, ordered by `code`, whose filter admits it. An alert admitted by no queue is counted in an `unassigned` row (`pool_id` NULL) that G2 shows as a total without a pool; the forecast, the sensitivity table and the alarm read pool rows only (Open doubts 4).""")
rep("""    pool_id               UUID NOT NULL REFERENCES admin.pools(id) ON DELETE RESTRICT,
    bucket_start          TIMESTAMPTZ NOT NULL,                  -- the hour, bank time (bank.timezone)""",
    """    pool_id               UUID REFERENCES admin.pools(id) ON DELETE RESTRICT,   -- NULL = the unassigned total (section 3.3)
    bucket_start          TIMESTAMPTZ NOT NULL,                  -- the hour, bank time (bank.timezone)""")
rep("CONSTRAINT uq_capacity_snapshot UNIQUE (pool_id, bucket_start, work_kind, class_code)",
    "CONSTRAINT uq_capacity_snapshot UNIQUE NULLS NOT DISTINCT (pool_id, bucket_start, work_kind, class_code)")
rep("7. **Phase 17.** None in code. The snapshotter reads the queue routing rule and `touch_due_at`;",
    "7. **Phase 17.** None in code. The snapshotter reads the queue `pool_id` and `filter` (the attribution rule of section 3.3 is this phase's, not phase 17's) and `touch_due_at`;")
rep("- No screen lists an analyst or a person. A row is a pool and a class.",
    "- No screen lists an analyst or a person. A row is a pool and a class. Alerts that no queue admits are shown as one \"unassigned\" total under the table, never inside a pool.")
rep("Each class's SLA is a bank setting in research; this phase counts the 60-day default only.",
    "Each class's SLA is a bank setting in research; this phase counts the 60-day default only. The alert-to-pool attribution (first active `alerts` queue by `code` whose filter admits the alert, else `unassigned`) is this phase's rule; the bank may prefer another, for example the pool of the claiming analyst's queue.")

# ---- lead corrections paragraph
rep("\n## Open doubts (for gap pass 1 and the bank)\n",
    "\n**Lead corrections (review 2026-10-09).** A SCIM deactivation ran the final phase 03 offboarding with no person and no step-up, so one forged or mistaken directory event could end every linked account for good; it now runs a reversible phase 03 disable (`UserAdminService::disable_from_directory`, `source: directory`, grants kept), the reconciliation lists directory-disabled accounts to ADM, and the final offboarding stays an administrator's act with step-up (Cedar `user.disable`, not `user.offboard`; outcome `disable_deferred`). The edge exposed only `/sso/*` and `/scim/v2/*`, which missed the Polis authorize and assertion consumer paths, and left `POST /api/v1/directory/events` open to the bank network under the phase 00 `/api/*` rule; the edge now exposes the pinned Polis paths and refuses the webhook route. `train.practice_scores` counted a pass made after the trainee had seen the answer; it now scores each trainee's first attempt per case only. The alert-to-pool attribution was credited to phase 17, which has no such rule; it is stated as this phase's rule, with an `unassigned` total (`pool_id` NULL, `UNIQUE NULLS NOT DISTINCT`).\n\n## Open doubts (for gap pass 1 and the bank)\n")

body=s[:s.index('**Lead corrections (review')]
for w in ('offboard_directory','offboarding_deferred','passed_once','/sso/*'): assert w not in body, w
assert True or 'offboard_directory' not in s and 'offboarding_deferred' not in s and 'passed_once' not in s and '/sso/*' not in s, 'leftover'
open(p,'w').write(s)
print('ok')
