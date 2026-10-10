import re
# phase file: DR-298 and DR-307 follow the lead corrections
p='docs/plan/phase-29.md'
s=open(p).read()
def rep(a,b):
    global s
    assert s.count(a)==1,(s.count(a),a[:80]); s=s.replace(a,b)
rep("Advisory and enforcing create holds; a list hit gets one phase 14 alert",
    "Advisory and enforcing create holds; a freeze-list hit holds, a PEP or internal hit holds only when its kind is in `assess.hold_list_kinds` (default freeze only); a list hold gets one phase 14 alert")
rep("and writes exceptions for no decision, no key and a decision that was not a pass.",
    "and writes exceptions for no decision, no key, a pass that expired before the release (`expires_at` plus `assess.release_skew_s`) and a decision that was not a pass.")
open(p,'w').write(s)

p='docs/plan/research.md'
s=open(p).read()
rows=[
("DR-292","Phase 29 gateway identity: each gateway is a row of `assess.gateways` pinning the SHA-256 of its leaf certificate (bank CA chain required, the pin decides) and naming one `admin.users` service principal; revocation is a status change seen within `assess.gateway_cache_s` and needs two operators","C-55, T-43, SP-93; a gateway holds a certificate, not a rotatable secret, so the phase 01 client-credentials path is not used"),
("DR-293","Transport split: 14 people routes under `/api/v1/assess/*` on akr-api (sessions, Cedar, step-up); 4 gateway routes under `/assess/v1/*` on akr-assess (certificate only, no browser route, no cookie, no CORS)","ARCHITECTURE D4 and section 21; the phase 00 zone matrix"),
("DR-294","Decision signing: an Ed25519 seed wrapped under `akr-kek`, unwrapped once per node at start-up and used in process; transit key `akr-sign-decision` is reserved and unused (gap pass 1 amends phase 00); yearly rotation, verify keys published with a fingerprint read on a second channel","API 4.2 forbids an OpenBao call in the request path; TS-24, SP-11, the phase 27 verify-key pattern (DR-261)"),
("DR-295","Own replay ledger `assess.idempotency` keyed by gateway and key (not `admin.idempotency_keys`); a byte-exact replay returns the stored signed `response_bytes`; ledger swept after `assess.idempotency_hours` (48), `GET /assess/v1/decisions/{id}` serves later reconciliation","C-55, API 4.2; the phase 00 table is akr_app-only and keyed by a principal shared with people"),
("DR-296","The decision row is the record: no `audit.audit_log` row per payment; the audit stream holds the mutations only","The phase 02 stream lock per payment would be the throughput limit; research 25 15.3"),
("DR-297","Own parser for MT103 and pacs.008 subsets (DTDs and entities refused) plus `canonical` JSON for every rail; a parse failure answers 422 and writes no decision; `rtgs` and `ach` canonical only until the bank's formats are sampled","Research 25 15.1 and 15.3; phases 10 and 11 defer MT and MX parsing"),
("DR-298","Modes shadow, advisory and enforcing; shadow answers pass and keeps a `shadow_review` row with no alert; a freeze-list hit holds, a PEP or internal hit holds only when its kind is in governed `assess.hold_list_kinds` (default freeze only, else reason `LIST_NOTED` on a pass); a list hold gets one phase 14 alert, other holds are review holds with four eyes","Research 25 15.9, C-56, phase 14 section 3.6 (lead review: research 25 asks pre-release screening for sanctions only)"),
("DR-299","The release pass of a released hold is a `hold_release` decision signed by akr-assess at the first `GET /assess/v1/holds/{hold_ref}`, persisted before it is sent, recorded once on the hold","One process holds the decision key (DR-294); pull is the default of research 25 15.7"),
("DR-300","Fail policies with bands, rail modes and prohibited rules are phase 04 change targets on `cr_material`; database guards refuse direct writes; no emergency change for any of them (the gateway's own fail policy is the emergency lever)","C-56, C-57, phase 04 route map, phase 27 Open doubts 18"),
("DR-301","Only an active prohibited rule rejects without a person; its condition uses four keys (`corridor_in`, `currency_in`, `rail_in`, `amount_min`); a scenario flag `phase2_candidate` is never read by code and becomes a rule only by its own change request","C-65, research 25 15.3, S-26, GV-53"),
("DR-302","No model and no suppression in the decision path: the phase 14 index and deterministic rules only, `decisions.model_version_id` dropped, pair memory and good-guy rules not applied inline in this version","T-13 to T-15, GV-53, N-50, N-51; research 25 15.5 conservative default"),
("DR-303","akr-assess never calls akr-api and writes no outbox topic; a list hold gets its phase 14 alert from the hold sweep in akr-worker as a system `payment` run (`requested_by` NULL); the freeze starts when a person decides that alert `true`","Phase 20 freezes reference a screening alert; zone matrix (lead review: an `adhoc` run named no resolvable subject)"),
("DR-304","List freshness is phase 14 `ListRecord::stale` unchanged (no grace of this phase); a cross-border decision on a stale list holds with `LISTS_STALE`, a domestic one carries the degraded flag only; the stricter C-57 reading stays open","API 4.2, research 25 15.5 item 3, CBI 12-hour freeze-list rule (lead review: a 2-hour grace let a 14-hour-old freeze list pass)"),
("DR-305","The local fallback is a signed, time-limited extract of the freeze lists (`assess.fallback_extract_hours`, at most 12) read by the bank's own component; AKR-TMS runs no matcher outside its zone","Research 25 15.6 item 2, M19, zone rules"),
("DR-306","Node heartbeats and a minute watch open and close outage periods in `assess.blind_periods` (separate from phase 11 `admin.blind_periods`), the operator records missed outages with evidence; the watch is idle while `pack.assess` is off or no rail has an open mode","C-58, research 25 15.6 item 3 (lead review: an install without phase 2 must record no outage)"),
("DR-307","The release-without-decision control reads released outgoing payments of advisory and enforcing rails from `canon.payment_message` with a 30-minute lag, matches by UETR or a reference hash within `assess.release_window_min`, counts a pass only up to its `expires_at` plus `assess.release_skew_s` (60, max 120), and raises exceptions `no_decision`, `no_match_key`, `pass_expired`, `decision_not_pass` on the phase 24 register (source `assess_control`)","C-59, research 25 15.7, phase 10 canon (lead review: an expired pass counted as matched)"),
("DR-308","Control-test payments go through the real gateway weekly and are recorded by `admin.system` with `POST /api/v1/assess/control-tests`; the phase 24 calendar shows the next due test, a missed test is a warning","C-58, research 25 15.6 item 5, API 3.19"),
("DR-309","Tipping-off: the customer gets a generic code, the gateway reasons, SS and M names as phase 14 shows them; decision search for IA and M only, purpose required, phase 17 masking; aggregates below `report.small_number_threshold` suppressed","C-65, API 4.2, phase 17 rules, SP-93"),
("DR-310","Governance: fail-policy, mode and rule sets are Tier 1 inventory items with owner and validator apart and a phase 24 validation pack before the first move to advisory","GV-53, GV-03, GV-32, phase 27 tiering"),
("DR-311","Mode entry criteria are governed settings with floors the bank may raise but not lower (30 shadow days, 1,000 decisions, 0.9 review ratio, 14 advisory days, weekly control test), and the approved snapshot is kept in `rail_modes.readiness`","C-56, research 25 15.9, phase 24 floor pattern"),
("DR-312","A hold past `assess.hold_max_hours` (24, max 72) is `expired` and never passed afterwards; the gateway applies its fail policy","Research 25 15.7, API 4.2"),
("DR-313","Retention: decisions R7 by partition on `received_at` with an append-only trigger; holds and exceptions R7 by closure; outage periods R10; control tests R11; configuration R9; a decision partition is dropped only through `admin.retention_drop_partition` and not while a hold references it","ARCHITECTURE section 17, DATA-MODEL section 19, phase 23 clocks (DR-187)"),
("DR-314","Health surfaces show modes, list states, flag names, counts by rail and node status only; no customer, small counts suppressed","SP-93, phase 21 threshold, phase 24 G1 rule"),
("DR-315","Eighteen routes: the 12 owned rows of API 3.19, 2 added people routes (`GET /api/v1/assess/modes/{rail}/readiness`, `GET /api/v1/assess/blind-periods`) and the 4 gateway routes of API 4.1","Route budget L (15 to 25); the approver must see readiness, control owners need outages (C-58)"),
("DR-316","Cedar file `D0_assess.cedar` (phase 03 range D0); the gateway policy tests the service principal and `Ctx.via` value `gateway_cert` (added to phase 03 by gap pass 1); people policies one role each; purpose required for a decision search","Phase 03 Ctx, phase 14 and phase 25 idioms"),
]
anchor=re.search(r'^\| DR-291 \|.*\n',s,re.M)
assert anchor and '| DR-292 |' not in s
s=s[:anchor.end()]+''.join(f"| {d} | 2026-10-09 | {a} | {b} |\n" for d,a,b in rows)+s[anchor.end():]
open(p,'w').write(s)
print('ok',len(rows))
