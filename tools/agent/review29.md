# Phase 29 review findings (lead, 2026-10-09)
Draft: docs/plan/phase-29.md (1,533 lines; 0091-0094; 19 tables; 18 routes; 6 jobs; D0_assess.cedar; DR-292..316).
Checked true against earlier phases: phase 14 list/alert/run DDL (screening_runs kind has `payment`), change_requests
statuses `approved`/`effective`, feature_flags has `scope`, value_type list, retention CHECKs, Cedar User.kind and
purpose codes, payment_message columns (direction, uetr, message_refs), admin.users columns. Scope vs Cedar rows OK.

1. MEDIUM the release control counts any `pass` as matched (s.3.14 step 4), even when the canonical release came after
   the pass's `expires_at`; the expiring pass is the control API 4.2 relies on ("a gateway releasing later must call
   again"), so a stale pass releases unseen. Fix: a pass matches only when the payment's `received_at` <= the pass's
   `expires_at` + `assess.release_skew_s` (new governed setting, default 60, max 120); otherwise exception
   `pass_expired` (release_exceptions.reason_code CHECK gains it; s.3.14, s.1.2, s.1.4, s.3.19).
2. MEDIUM list freshness is redefined (s.3.1 step 12, s.3.6: active `loaded_at` > `refresh_hours` +
   `assess.list_grace_hours` 2) instead of phase 14 `ListRecord::stale` (phase 14 l.934: `now - active.activated_at` >
   min(refresh_hours, screening.list_max_age_h), freeze lists never above 12 h); the 2-hour grace lets a cross-border
   payment pass on a freeze list up to 14 hours old, beyond the CBI 12-hour rule phase 14 enforces. Fix: call phase 14
   `ListRecord::stale` through the index registry; drop setting `assess.list_grace_hours` (s.1.4, s.3.19, DR-304).
   Gap item for phase 14: an unchanged feed (`list.feed_unchanged`) should renew the freshness clock, else an unchanged
   UN list goes stale and holds every cross-border payment.
3. MEDIUM the hold alert has no resolvable subject and duplicates post-event screening: s.3.5 sweep step 2 opens an
   `adhoc` run (a person's name search in phase 14) with `subject_id` = a UUIDv5 `subject_ref` that is no canonical
   payment party, so phase 14 views and masking find no subject; and when the payment lands in canon, phase 14 job
   `screening_payment` screens the same party and opens a second alert for the same entry. Fix: run kind `payment`,
   `requested_by` NULL (system run); the hold alert's subject view reads the names from `assess.holds.party_names_enc`
   (requirement on phase 14 view); `screening_payment` skips an entry already alerted on a hold whose decision has the
   same match key (requirement on phase 14; s.4.5 item 4 and Open doubts).
4. LOW readiness criterion "no release exception on the rail in the window" for shadow -> advisory (s.3.7) can never
   fail: the matcher skips shadow rails (s.3.14 step 1). Fix: drop it from the shadow -> advisory row.
5. LOW `akr_assess` gets SELECT on all of `admin.users` (s.1.3), names and emails included, in the gateway zone. Fix:
   column grant `(id, kind, status, expires_at)`.
6. LOW every install without phase 2 gets a permanent outage: `assess_blind_watch` opens a blind period and a
   register warning whenever no node beats, and `akr-assess` is not started by default (phase 00 profile `phase2`).
   Fix: the six jobs are no-ops while `pack.assess` is off; the blind watch also needs at least one rail with an open
   mode row.
7. LOW the heartbeat upsert sets `started_at` on a restart but `akr_assess` has no UPDATE on it. Fix: add `started_at`
   to the column grant.
8. LOW roll-up `would_pass`, `would_hold`, `would_reject` come from `would_be`, which is NULL outside shadow, so
   advisory and enforcing days count zero. Fix: count `coalesce(would_be, decision)`.
9. LOW every PEP and internal list hit holds a payment before release (s.3.1 step 10, "every list kind holds");
   research 25 s.15 asks pre-release screening for sanctions; holding PEP payments is not in the requirement. Fix: a
   freeze-list hit holds; a PEP or internal hit holds only when its list kind is in governed setting
   `assess.hold_list_kinds` (default `["freeze"]`, must contain `freeze`), otherwise it is a reason code on a pass.
Severity: 0 critical / 0 high / 3 medium (1-3) / 6 low (4-9) = 9.

Lead decisions on the draft's Open doubts 1 and 2 (gap pass 1 applies to phase 00): `akr-assess` joins `z1_bao` for
the start-up unwrap (phase 00 compose and zone matrix); phase 00 l.588 says `akr-sign-decision` is reserved and unused
(decisions are signed in process by a seed wrapped under `akr-kek`, DR-294); phase 00 AppRole line for akr-assess
gains transit decrypt on `akr-kek` and KV read `keys/assess-*`.
Gap-pass items (not draft defects): phase 23 drop of a decision partition only after its holds are gone; phase 24
review-calendar row `assess_control_test` and finding closure closing release exceptions; phase 25 SIEM and support
bundle exclusions; phase 02 owner kind `assess_extract` and the PartitionRegistry entry; phase 03 `Ctx.via` value
`gateway_cert`; API.md 3.19 scopes widened (GET fail-policies and modes add config.approve, dashboard.read) and 4.2
response gains `fail_policy` and `policy_version`, formats `mt103`, `pacs008`, `canonical`; DATA-MODEL s.19
replaced; DESIGN-SYSTEM S2, D3, G1 additions; phase 01 service-principal expiry versus certificate life (Open doubts 24).
A stopped container `p27test` (postgres:17) is left from the phase 27 agent; not removed (Docker rule).
