p='docs/plan/phase-26.md'
s=open(p).read()
def rep(a,b,n=1):
    global s
    assert s.count(a)==n,(s.count(a),a[:90])
    s=s.replace(a,b)
# 6: close the 1.1 block, retention SELECT, kyc comment
rep("GRANT DELETE ON canon.islamic_deals, canon.deal_parties TO akr_retention;\n\n### 1.2",
    "GRANT SELECT, DELETE ON canon.islamic_deals, canon.deal_parties TO akr_retention;   -- phase 23's SELECT ON ALL TABLES predates these\n```\n\n### 1.2")
rep("GRANT DELETE ON kyc.possession_evidence TO akr_retention;   -- phase 23: the only DELETE in kyc",
    "GRANT SELECT, DELETE ON kyc.possession_evidence TO akr_retention;   -- phase 23 role; its SELECT ON ALL TABLES predates this table")
rep("| `canon.islamic_deals` | SELECT | SELECT, INSERT; UPDATE (recorded_to, valid_to) | DELETE | none |",
    "| `canon.islamic_deals` | SELECT | SELECT, INSERT; UPDATE (recorded_to, valid_to) | SELECT, DELETE | none |")
rep("| `canon.deal_parties` | SELECT | SELECT, INSERT; UPDATE (superseded_at) | DELETE | none |",
    "| `canon.deal_parties` | SELECT | SELECT, INSERT; UPDATE (superseded_at) | SELECT, DELETE | none |")
rep("| `kyc.possession_evidence` | SELECT, INSERT; UPDATE (status, reject_code, decided_by, decided_at, updated_at, version) | none | DELETE | none |",
    "| `kyc.possession_evidence` | SELECT, INSERT; UPDATE (status, reject_code, decided_by, decided_at, updated_at, version) | none | SELECT, DELETE | none |")
# 2: content route
rep("| POST | `/api/v1/islamic/deals/{id}/evidence/{evidence_id}/verify` (added) |",
    "| GET | `/api/v1/islamic/deals/{id}/evidence/{evidence_id}/content` (added) | `queue.work` | The verified bytes of one attachment through `EvidenceStore::open`; access event first; **SU** `record_open` on a suspicion-tier deal; `download=true` records `download` (`islamic.evidence.read`; phase 18 content pattern) |\n| POST | `/api/v1/islamic/deals/{id}/evidence/{evidence_id}/verify` (added) |")
rep("- **Route table** (scopes from API.md section 2; rows 1 to 4 are API.md 3.18, row 5 is added):",
    "- **Route table** (scopes from API.md section 2; four rows are API.md 3.18, the content and verify rows are added):")
rep("""   `evidence_status` from `kyc.deal_evidence_status`. No file name, no document reference, no evidence object id is returned: the
   file is opened only through phase 02's `GET /evidence/{id}` by a route that checks the owner (phase 18), so this view does not
   expose it.""",
"""   `evidence_status` from `kyc.deal_evidence_status`. No file name and no document reference are returned; each row carries its
   own `id`, and the file is opened through `GET /islamic/deals/{id}/evidence/{evidence_id}/content` (below), which repeats this
   read's checks (lead review: no generic `GET /evidence/{id}` exists; phase 18 serves content per owner).""")
rep("""9. Access event `islamic.deal.read` (phase 03 access event with purpose and tier). The read writes no audit row.""",
"""9. Access event (phase 02 `access_event`, action `view`, resource_type `islamic_deal`, with purpose and tier). The read writes no
   audit row.""")
rep("""**Verify** (`POST /islamic/deals/{id}/evidence/{evidence_id}/verify`, `If-Match`, `Idempotency-Key`, body `DecideEvidence`):""",
"""**Content** (`GET /islamic/deals/{id}/evidence/{evidence_id}/content`; Cedar `islamic.evidence.read`; the phase 18 content pattern):

1. Steps 1 to 6 of 3.4 (flag, deal read, tier, Cedar, purpose, step-up `record_open` on a suspicion-tier deal); refusal 404 with the
   unknown-id body.
2. The evidence row must belong to this deal (404 otherwise; the same body).
3. Access event first (action `view`, or `download` with `download=true`; resource_type `possession_evidence`), then
   `EvidenceStore::open` (phase 02: decrypt, authenticate every segment, check the SHA-256). An object not `clean` is 409
   `E_NOT_ALLOWED_IN_STATE` (`evidence_not_clean`). `Cache-Control: no-store`.

**Verify** (`POST /islamic/deals/{id}/evidence/{evidence_id}/verify`, `If-Match`, `Idempotency-Key`, body `DecideEvidence`):""")
# 3: register insert
rep("""4. When either count is above zero, insert one register warning (`admin.registers`, kind `warning`, source `islamic_evidence`,
   `source_key = 'islamic_evidence_gap:' || bank_date`, `ON CONFLICT (source_key) DO NOTHING`, fixed bilingual title, detail
   `{"missing": n, "late": n}`).""",
"""4. When either count is above zero, insert one register warning (`admin.registers`, kind `warning`, source `scan` (phase 24's
   closed list; lead review), `source_key = 'islamic_evidence_gap:' || bank_date`, `ON CONFLICT (source_key) DO NOTHING`, fixed
   bilingual title, body `{"missing": n, "late": n}`).""")
# 7: business clock
rep("""3. Count current, `active` deals with `opened_on <= BusinessClock::add(today, -islamic.evidence_due_days, Days)` and""",
"""3. Count current, `active` deals whose due date `BusinessClock::add(opened_on, islamic.evidence_due_days, Days)` (phase 09; forward
   only) is before today and""")
rep("""4. Small cells (phase 21 `report.small_number_threshold`, default 5, minimum 3): a count from 1 to 4 is returned as null with one
   `suppressed_cells` for each;""",
"""4. Small cells (phase 21 `report.small_number_threshold`, governed, default 5, minimum 3): a count above zero and below the threshold
   is returned as null with one `suppressed_cells` for each;""")
# 1: pooled freeze
rep("""3. A `pooled` task sets `second_person_only = TRUE`: its verification needs a second person (phase 20 section 3.3). The task's payload
   carries the class code, never the account number.""",
"""3. A `pooled` task is an ordinary `hold` task: its verification follows phase 20 section 3.3 unchanged. `second_person_only` is not
   set at creation (phase 20 sets it only with verification level `v0`, and its CHECK refuses it earlier; lead review). The task's
   `result` carries the class code, never the account number. A two-person rule for pooled holds is Open doubts 9.""")
rep("pooled investment accounts `hold` with two-person verification, so that",
    "pooled investment accounts `hold` (phase 20 verification unchanged), so that")
rep("so this phase uses the neutral ops task with two-person verification (DR-252).",
    "so this phase uses the neutral `hold` task with phase 20's verification (DR-252); whether a pooled hold needs a second person before completion is the bank's call and needs a phase 20 rule, not `second_person_only`.")
rep("creates two ops tasks, `hold` (second_person_only TRUE) and\n     `freeze`;",
    "creates two ops tasks, `hold` (`second_person_only` FALSE) and\n     `freeze`;")
# 4: G1 tile
rep("""`unknown` with the evidence process live): visible to A2 and M; hidden while the flag is off.""",
"""`unknown` with the evidence process live): visible to M only (G1 is `dashboard.read`; A2 holds no such scope, lead review);
  hidden while the flag is off.""")
# hooks and counts
rep("`useGetIslamicDeals`, `useGetIslamicDealsId`, `usePostIslamicDealsIdEvidence`, `usePostIslamicDealsIdEvidenceEvidenceIdVerify`,\n`useGetIslamicShariaBoardAggregates`.",
    "`useGetIslamicDeals`, `useGetIslamicDealsId`, `usePostIslamicDealsIdEvidence`, `usePostIslamicDealsIdEvidenceEvidenceIdVerify`,\n`useGetIslamicShariaBoardAggregates`; the content read is hand-written like phase 18's `useEvidenceContent` (a blob, no cached URL).")
rep("**Complexity:** M by the template rule (5 routes: 4 rows of API.md 3.18 and one row added;",
    "**Complexity:** M by the template rule (6 routes: 4 rows of API.md 3.18 and two rows added (content, verify);")
rep("9. Swagger shows the five paths;","9. Swagger shows the six paths;")
rep("- OpenAPI: tags `islamic` (four paths) and `sharia-board` (one path); the verify path is added.",
    "- OpenAPI: tags `islamic` (five paths) and `sharia-board` (one path); the content and verify paths are added.")
open(p,'w').write(s)
print('ok')
