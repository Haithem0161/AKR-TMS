#!/usr/bin/env python3
"""Apply gap pass 1, run 5b (coverage: SP and K) after the lead's checks.

Source: tools/agent/gap1-run5b.md (Haiku draft); decisions: tools/agent/gap1-run5b-verdict.md.
Header lines go at the end of each phase's "Requirements carried" block, one line per class; where run 5a already wrote
a line of the same class, the IDs are appended to it.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from g1lib import rep, add_section7  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PLAN = ROOT / 'docs' / 'plan'

HDR = ('Gap pass 1 (2026-10-10), run 5b (coverage of SP and K IDs), from tools/agent/gap1-run5b.md; '
       'lead-verified (tools/agent/gap1-run5b-verdict.md).')

IMPL = {
    '02': 'SP-40 (the verifier over chain, Merkle roots, signatures per key version and the locked copies; scheduled and '
          'on demand; a critical finding pages as P1)',
    '15': 'SP-29 (the relationship-conflict check, section 3.9), K-34 (the rating model as governed configuration; the '
          'model card and validation packs are phases 27 and 24)',
    '18': 'SP-28 (manual grants on suspicion-tier cases by the MLRO only, time-boxed and expiring automatically; '
          'section 7+)',
    '19': 'K-12 (the authority-request register and its re-rating trigger)',
    '22': 'SP-23 (branch-scoped window, the confidential channel to the MLRO desk, the liaison monthly reports)',
    '23': 'SP-57 (record purge of whole party trees for relationship-end and case clocks)',
    '24': 'SP-34 (the monthly tipping-off control report)',
    '29': 'SP-96 (no customer-facing surface; the customer sees a generic code only)',
}
PART = {
    '00': 'SP-10 (field encryption by class and keyed match hashes), SP-45 (the column classification registry and its '
          'coverage test)',
    '02': 'SP-41 (the audit kind catalogue with its SIEM flags, catalogue-listed keys without personal data, the '
          'correlation id, UTC partitions)',
    '03': 'SP-47 (purpose codes checked against roles), SP-49 (masking by surface; unmask as an access event), SP-79 (the '
          'inference test harness and its registry)',
    '10': 'K-32 (the family number, the migrated-record placeholder flag; name variants are phase 14 and Hijri dates '
          'phase 09)',
    '12': 'K-31 (the data-quality register with owners and ages)',
    '13': 'SP-46 (protected attributes blocked as inputs)',
    '15': 'K-19 (owner percentages entered with evidence; the coverage view, section 7+), K-25 (override analytics by '
          'proposer and branch with follow-on alerts and STRs, section 7+), K-36 (the level 1 row of the controls '
          'matrix with its CDD depth and monitoring profile; re-rating by triggers)',
    '16': 'K-20 (scenario outputs link parties by shared family number, phone, address, signatory and owner)',
    '17': 'SP-54 (the sealed evidence snapshot at case open)',
    '18': 'SP-26 (three tiers, the case access list, step-up, analyst networks; time-boxed manual grants in section 7+), '
          'SP-32 (step-up, purpose and an access event on every content download by the case team), SP-100 (recorded '
          'decisions with a named decider and coded reasons)',
    '21': 'K-23 (PEP items a to e in the CBI monthly pack, section 7+)',
    '23': 'SP-55 (plan, dry run, detach and deletion certificates), SP-61 (no erasure outside the retention classes; '
          'CBS-origin data is never written), SP-63 (the per-class retention report)',
    '24': 'SP-80 (the assessor pack), SP-99 (the protected-attribute register, the proxy scan and monthly outcome '
          'metrics), K-38 (the assessor workspace and packs; the rating distribution is phase 15)',
    '25': 'SP-90 (signed release bundles verified on import; no call-home)',
    '27': 'SP-91 (signed ONNX artifacts, the operator allow-list, the golden check with fallback; GGUF reading in '
          'DR-396), SP-92 (golden regression and the forced low-score sample)',
}
NOTC = {
    '00': 'SP-69 (vendor development practice, DR-396), SP-102 (the bank privacy office\'s records, DR-396)',
    '05': 'SP-82 (the CISO\'s exception register is the bank\'s, DR-396)',
    '15': 'K-41 (pre-purchase data pilot, a procurement step, DR-396)',
    '23': 'SP-62 (vendor contract exit export, DR-396), SP-97 (data-subject requests are the bank\'s process, DR-396)',
    '24': 'SP-02 (control crosswalk documents, vendor compliance work, DR-396)',
    '25': 'SP-01, SP-67, SP-68, SP-77 (vendor security programme and contract, DR-396), SP-50 (pseudonymised tuning '
          'sets and vendor bundles, P2, DR-396)',
    '26': 'K-39 (Islamic inputs to scores are forbidden, DR-249; F3 is withheld, DR-103)',
    '29': 'K-40 (payment-time rating lookup, outside this release, DR-396)',
    '31': 'SP-78 (vendor assurance programme, DR-396)',
}

P18 = [
"""### 7.N Time-boxed manual grants on suspicion-tier cases (SP-28, SP-26, DR-395)
The register of access requests is the set of manual grants in `casework.case_acl`; no new table. Section 1 carries the
column `expires_at`, the revoke CHECKs (a NULL `revoked_by` on a revoked row means the system expired the grant, as a
NULL `granted_by` means the system granted it), the guard that keeps `expires_at` fixed, the classification row and the
setting `casework.manual_grant_max_days` (governed, default 30).
1. `grant` (section 3.2), new step 3a: when the case's `tier` is `suspicion` and `reason = manual`, the caller holds role
   `M` (403 `E_FORBIDDEN` otherwise; the MLRO's approval is the grant and its note is the written reason) and the body
   carries `expires_at`, later than now and at most `casework.manual_grant_max_days` ahead (422 `E_VALIDATION` at
   `/expires_at`). On other tiers `expires_at` is optional with the same bound. Assignment, second-review and MLRO-desk
   grants carry none: they end with the assignment or the role.
2. Step 4: an active grant of the same principal whose `expires_at` has passed is first revoked by the system in the
   same transaction (step 3 of the handler below), so it does not block the new grant.
3. Step 6: a grant with `expires_at` schedules a phase 00 timer, kind `case_acl_expiry`, owner kind `case_acl`, owner id
   the grant id, `fire_at = expires_at`; a revocation cancels it.
4. Handler `timer_case_acl_expiry` (akr-worker): lock the case, re-read the grant; if it is still active and
   `expires_at <= now()`, set `revoked_at = now()` with `revoked_by` NULL; chain `case.acl_expired {principal_id}`; audit
   `case.acl_expired`; `version + 1`. A grant already revoked is left alone.
5. Job `case_acl_expiry_scan` (akr-worker, setting `schedule.case_acl_expiry_scan`, hourly at minute 15): the backstop
   for timers that did not fire; enqueues `timer_case_acl_expiry` per active grant with `expires_at <= now()`, coalesce
   key `case_acl_expiry:<grant id>`.
6. The rule-5 visibility clause (section 3.9) and `Holders` rule 5 also require `(a.expires_at IS NULL OR a.expires_at >
   now())`, so a grant past its time gives no access before the timer fires.
7. `list` returns `expires_at`. The audit table of section 3.18 gains `case.acl_expired` (access list change, case,
   chain yes). The counts for the CISO are phase 24's control report (from phase 24 on).
8. Tests: a manual grant on a suspicion case by A2 is 403; by M without `expires_at` is 422; a grant whose time passes
   is revoked by the timer with `revoked_by` NULL and the person gets 404 on the case; an UPDATE that changes
   `expires_at` fails in the guard.""",
]

P15 = [
"""### 7.N Ownership coverage view (K-19, DR-395)
| Method | Path | Scope | Description |
|---|---|---|---|
| GET | `/api/v1/ownership/coverage` | `dashboard.read` | Beneficial-ownership coverage by level, counts only (`rating.distribution.read`) |

`OwnershipService::coverage(ctx)` (Cedar `rating.distribution.read`, M, CH and IA; no new action): one SELECT over
`kyc.rating_current` joined to `kyc.ownership_results` (`superseded_at IS NULL`) by `party_id`, grouped by current
level: parties rated, parties with a current ownership result, results with `unknown_count > 0`, results with
`loops_detected`, results with `depth_truncated`, and results whose `as_of` is older than the party's KYC refresh
interval of the controls matrix. Suspicion-tier parties are counted with the rest and never listed; no party id is
returned. The registry import path of K-19 is P2 (DR-396). Shown on K3 as a header strip. Test: a fixture of three
parties (one result with an unknown owner, one without a result) gives the expected counts.""",

"""### 7.N Override analytics (K-25, DR-395)
| Method | Path | Scope | Description |
|---|---|---|---|
| GET | `/api/v1/rating/overrides/analytics` | `dashboard.read` | Override counts by proposer and branch, downward share, expiries, follow-on alerts and STRs (`rating.distribution.read`) |

`OverrideService::analytics(ctx, from, to)` (Cedar `rating.distribution.read`, M, CH and IA; no new action), counts
only, over `kyc.overrides` with `effective_from` in the window (at most 366 days):
1. Per `proposed_by`: total, `direction = 'down'`, `end_reason = 'expired'`; the downward share is down over total.
   User ids are returned, never names.
2. Per branch: the same counts, the branch being `canon.customer.branch_id` of the party's current customer rows
   (`recorded_to IS NULL`); a party with customer rows at two branches counts at each.
3. From phase 18 on: overall counts (not per person or branch) of overrides whose party has an `alert.alerts` row
   created within 90 days after `effective_from`, and of those whose party is on a case (`casework.case_parties`) with a
   `casework.decisions` row of type `str` decided within 180 days after it. Before phase 18 the two fields are null.
No audit event (a read). Test: fixture overrides by two proposers give the expected counts and share.""",
]

P21 = [
"""### 7.N PEP KPI items in the CBI monthly pack (K-23, DR-395)
No new pack kind: `cbi_monthly` (section 3.16) gains a PEP section with the items of the CBI circular item 9 that this
system holds (research 26, the CBI KPI report), counts for the month under the small-number rule of the pack:
- a. PEPs by category: confirmed `kyc.pep_register` entries by `tier`, current and former (`kyc.party_pep_state`);
- b. found by screening versus self-disclosed: confirmed entries by `source` (`pep_list` against `declaration`, with
  `cbs_class` and `manual` shown apart);
- c. relationships needing senior approval and their outcome: completed `kyc_edd_signoff` instances for parties with a
  confirmed entry, by outcome, and `kyc.exit_register` entries for those parties;
- d. data updates and re-assessments: `kyc.kyc_reviews` completed for those parties, by `kind`;
- e. STRs on PEPs and associates: `casework.decisions` of type `str` whose case (`casework.case_parties`) holds a party
  with a confirmed entry or a confirmed `kyc.pep_relatives` row.
Items f (internal-audit test results and remediation) and g (share of staff trained) are not held by AKR-TMS; the pack
cover says so, as for the RP-68 reports. Test: a fixture month with two PEPs (one by list, one declared) and one STR
gives the expected section.""",
]

P24 = [
"""### 7.N Manual case grants in the control report (SP-28, DR-395)
The control report (section 3.2) adds two counts for the month: manual grants on suspicion-tier cases made
(`casework.case_acl`, `reason = 'manual'`, case `tier = 'suspicion'`, `granted_at` in the month) and grants that expired
(`revoked_by IS NULL AND revoked_at` in the month). The small-number rule of the report applies. This is the CISO's view
of the access-request register of phase 18 section 7+.""",
]

TARGETS = [('18', HDR, P18), ('15', HDR, P15), ('21', HDR, P21), ('24', HDR, P24)]

out = {}


def load(pp):
    p = PLAN / f'phase-{pp}.md'
    return p, out.get(p, p.read_text())


for pp, hdr, blocks in TARGETS:
    p, s = load(pp)
    s = add_section7(s, hdr, blocks)
    out[p] = s

for table, label in ((IMPL, 'implemented here'), (PART, 'partly implemented'), (NOTC, 'not carried')):
    for pp, ids in table.items():
        if label != 'not carried':
            assert 'not carried' not in ids.lower(), ids
        p, s = load(pp)
        i = s.index('**Requirements carried:**')
        j = s.index('\n\n', i)
        blk = s[i:j]
        prefix = f'\n- Gap pass 1 (coverage), {label}: '
        if prefix in blk:
            k = s.index(prefix, i)
            e = s.index('\n', k + 1)
            assert e <= j and s[e - 1] == '.', s[k:e]
            s = s[:e - 1] + '; ' + ids + '.' + s[e:]
        else:
            s = s[:j] + prefix + ids + '.' + s[j:]
        out[p] = s


def edit(pp, pairs):
    p, s = load(pp)
    for a, b in pairs:
        s = rep(s, a, b)
    out[p] = s


edit('18', [
    ("NULL = the system (escalation to level 3)\n    granted_at    TIMESTAMPTZ NOT NULL DEFAULT now(),\n",
     "NULL = the system (escalation to level 3)\n    granted_at    TIMESTAMPTZ NOT NULL DEFAULT now(),\n"
     "    expires_at    TIMESTAMPTZ,                                              -- the time box of a manual grant (section 7+, SP-28)\n"),
    ("    CHECK ((reason = 'manual') = (note_id IS NOT NULL)),\n    CHECK ((revoked_at IS NULL) = (revoked_by IS NULL))\n);",
     "    CHECK ((reason = 'manual') = (note_id IS NOT NULL)),\n"
     "    CHECK (revoked_by IS NULL OR revoked_at IS NOT NULL),\n"
     "    CHECK (revoked_at IS NULL OR revoked_by IS NOT NULL OR (expires_at IS NOT NULL AND revoked_at >= expires_at)),   -- NULL revoked_by: the system expired it\n"
     "    CHECK (expires_at IS NULL OR expires_at > granted_at)\n);"),
    ("(NEW.id, NEW.case_id, NEW.principal_id, NEW.access, NEW.reason, NEW.note_id, NEW.granted_by, NEW.granted_at)",
     "(NEW.id, NEW.case_id, NEW.principal_id, NEW.access, NEW.reason, NEW.note_id, NEW.granted_by, NEW.granted_at, NEW.expires_at)"),
    ("(OLD.id, OLD.case_id, OLD.principal_id, OLD.access, OLD.reason, OLD.note_id, OLD.granted_by, OLD.granted_at)",
     "(OLD.id, OLD.case_id, OLD.principal_id, OLD.access, OLD.reason, OLD.note_id, OLD.granted_by, OLD.granted_at, OLD.expires_at)"),
    ("'granted_by', 'granted_at',\n                                 'revoked_by', 'revoked_at']),",
     "'granted_by', 'granted_at',\n                                 'expires_at', 'revoked_by', 'revoked_at']),"),
    (""" ('casework.txn_page_size', '100', 'integer', NULL, FALSE,
  '{"ar": "حجم صفحة المعاملات في صفحة الطرف", "en": "Transactions per page on the party transaction explorer"}')
ON CONFLICT (key) DO NOTHING;""",
     """ ('casework.txn_page_size', '100', 'integer', NULL, FALSE,
  '{"ar": "حجم صفحة المعاملات في صفحة الطرف", "en": "Transactions per page on the party transaction explorer"}'),
 ('casework.manual_grant_max_days', '30', 'integer', NULL, TRUE,
  '{"ar": "أقصى مدة لمنح وصول يدوي إلى قضية من فئة الاشتباه (أيام)", "en": "Longest manual grant on a suspicion-tier case (days; SP-28)"}')
ON CONFLICT (key) DO NOTHING;"""),
    (""" ('schedule.work_item_due_scan', '"0 5 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مسح بنود العمل المتأخرة (احتياطي لمهلة المؤقت)", "en": "Overdue work item backstop scan schedule (fallback for the timer)"}')
ON CONFLICT (key) DO NOTHING;""",
     """ ('schedule.work_item_due_scan', '"0 5 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مسح بنود العمل المتأخرة (احتياطي لمهلة المؤقت)", "en": "Overdue work item backstop scan schedule (fallback for the timer)"}'),
 ('schedule.case_acl_expiry_scan', '"0 15 * * * *"', 'cron', NULL, FALSE,
  '{"ar": "جدول مسح منح الوصول المنتهية (احتياطي للمؤقت)", "en": "Expired case grant backstop scan schedule (fallback for the timer)"}')
ON CONFLICT (key) DO NOTHING;"""),
])

edit('24', [
    ("and role grants that started or ended in the month (`admin.role_grants`, `valid_from`, `valid_to`, `revoked_at`);\n",
     "and role grants that started or ended in the month (`admin.role_grants`, `valid_from`, `valid_to`, `revoked_at`);\n"
     "- manual grants on suspicion-tier cases made and expired in the month (phase 18 `casework.case_acl`; section 7+);\n"),
])

# Phase 15 Open doubts 19: the K IDs now have a place.
p, s = load('15')
a = s.index('19. **K-IDs left to later phases.**')
b = s.index('Gap pass 1 assigns them.', a) + len('Gap pass 1 assigns them.')
s = s[:a] + ('19. **K-IDs placed by gap pass 1 (DR-395, DR-396).** K-12 phase 19; K-19, K-25 and K-36 here (section 7+ '
             'and the controls\n    matrix); K-20 phase 16; K-23 phase 21; K-31 phase 12; K-32 phase 10; K-34 here; '
             'K-38 phase 24; K-39, K-40 and K-41\n    are not carried. Closed.') + s[b:]
out[p] = s

m = ROOT / 'docs' / 'MODULES.md'
ms = m.read_text()
ms = rep(ms, "K-12, K-19, K-20, K-23, K-25, K-31, K-32, K-34, K-36 and K-38 to K-41 await later phases.",
         "Gap pass 1 placed the rest (DR-395, DR-396): K-12 phase 19; K-19, K-25, K-34 and K-36\nphase 15; K-20 phase 16; "
         "K-23 phase 21; K-31 phase 12; K-32 phase 10; K-38 phase 24; K-39 to K-41 are not carried.")
out[m] = ms

ap = ROOT / 'docs' / 'API.md'
s = ap.read_text()
row = '| GET | /rating/distribution | dashboard.read | Distribution, sanity bands, escapes |\n'
s = rep(s, row, row +
        '| GET | /ownership/coverage | dashboard.read | Beneficial-ownership coverage by level, counts only (phase 15 7+) |\n'
        '| GET | /rating/overrides/analytics | dashboard.read | Override counts by proposer and branch, downward share, '
        'expiries, follow-on alerts and STRs (phase 15 7+) |\n')
s = rep(s, '| POST | /cases/{id}/access | case.decide | Grant read, work or decide to a person (phase 18, added) |',
        '| POST | /cases/{id}/access | case.decide | Grant read, work or decide to a person; a manual grant on a '
        'suspicion-tier case is M only and time-boxed (phase 18, added; 7+) |')
out[ap] = s

for pp in ('15', '18', '21', '24'):
    p, s = load(pp)
    st = s.index('## 7+. Gap Additions')
    nx = s.find('\n## ', st + 5)
    tail = s[st:nx if nx != -1 else len(s)]
    for w in ('UNCLEAR', 'the lead decides', 'lead confirms'):
        assert w not in tail, (p.name, w)

r = PLAN / 'research.md'
rs = r.read_text()
anchor = [l for l in rs.split('\n') if l.startswith('| DR-394 |')]
assert len(anchor) == 1
DRS = [
    "| DR-395 | 2026-10-10 | Run 5b additions: a manual grant on a suspicion-tier case is made by the MLRO only and carries `expires_at` (at most `casework.manual_grant_max_days`), expired by a timer with a NULL `revoked_by`; the manual grants are the access-request register and the control report counts them (SP-28, SP-26). Phase 15 adds the ownership coverage view (K-19) and the override analytics (K-25), both counts only under `rating.distribution.read`. The PEP KPI items a to e go into the `cbi_monthly` pack, no new kind (K-23). K-36 is met by the level 1 row of the controls matrix and the trigger library; K-20 by the scenario link kinds | ROADMAP puts SP-26 to SP-36 in P0-P1 and M10 [P1] owns K-01..K-42; no new table, Cedar action or pack kind was needed (gap pass 1, run 5b coverage) |",
    "| DR-396 | 2026-10-10 | Outside this release: vendor programme and contract items (SP-01 threat model, SP-02 control crosswalk, SP-67 contract annex, SP-68 vendor ISMS, SP-69 SSDF, SP-77 rebuild and escrow, SP-78 assurance cycle, SP-62 exit export); the bank's processes (SP-82 exception register, SP-97 data-subject requests and restriction, SP-102 record of processing and impact assessment, the policy statement of SP-54); P2 (SP-50 pseudonymised tuning sets, the per-user watermark of SP-32, the registry import of K-19, the remaining K-20 scenarios, K-38's CBDDQ metrics, K-40 payment-time rating lookup); K-41 is a procurement step; K-39 is excluded by DR-249 and DR-103. SP-91's signed-ONNX rule is read as no code-bearing model format: the assistant's GGUF weights (DR-275) are a signed, data-only artifact on a read-only volume | MODULES and ROADMAP put none of them in P0 or P1, or a DR already decides them (gap pass 1, run 5b coverage) |",
]
rs = rep(rs, anchor[0] + '\n', anchor[0] + '\n' + '\n'.join(DRS) + '\n')
out[r] = rs

for p, s in out.items():
    p.write_text(s)
    print('wrote', p.relative_to(ROOT))
