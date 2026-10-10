import re
p='docs/plan/phase-24.md'
s=open(p).read()
def rep(a,b,count=1):
    global s
    assert s.count(a)==count,(s.count(a),a[:80])
    s=s.replace(a,b)
# 1 HIGH: tier function
a=s.index("-- The tier of an audit row's resource, read at query time")
b=s.index("GRANT SELECT (id, sensitivity_tier, recorded_to, valid_from) ON canon.party TO akr_readonly;")
b=s.index("\n",b)+1
newfn='''-- The tier of an audit row's resource, read at query time (section 3.1). It mirrors phase 17's tier rule 2 (phase-17
-- section 3.10): the party tier (`casework.effective_tier`), the case family tier, the case tier, and `suspicion` for an
-- alert of a restricted scenario (`scn.scenarios.visibility`, phase 06). An alert takes the highest tier of every case
-- it is linked to. Unknown resource types answer 'suspicion': the viewer fails closed. SECURITY DEFINER owned by
-- akr_migrate: akr_readonly gains no read of casework, alert, scn or canon (phase 17 grants it nothing there) and learns
-- only the tier word of the one resource it asks about.
CREATE OR REPLACE FUNCTION audit.tier_rank(p_tier TEXT) RETURNS INT
LANGUAGE sql IMMUTABLE SET search_path = pg_catalog AS $$
    SELECT CASE p_tier WHEN 'suspicion' THEN 2 WHEN 'restricted' THEN 1 WHEN 'normal' THEN 0 ELSE 2 END
$$;
CREATE OR REPLACE FUNCTION audit.resource_tier(p_type TEXT, p_id UUID, p_case UUID) RETURNS TEXT
LANGUAGE sql STABLE SECURITY DEFINER SET search_path = pg_catalog AS $$
    WITH case_rank AS (
        SELECT c.id, GREATEST(audit.tier_rank(c.tier), audit.tier_rank(f.tier),
                              audit.tier_rank(casework.effective_tier(c.party_id))) AS r
          FROM casework.cases c JOIN casework.case_families f ON f.code = c.family
    ),
    r AS (
        SELECT CASE
            WHEN p_case IS NOT NULL OR p_type = 'case' THEN
                COALESCE((SELECT cr.r FROM case_rank cr WHERE cr.id = COALESCE(p_case, p_id)), 2)
            WHEN p_type = 'party' THEN audit.tier_rank(casework.effective_tier(p_id))
            WHEN p_type = 'customer' THEN
                COALESCE((SELECT max(audit.tier_rank(casework.effective_tier(cu.party_id)))
                            FROM canon.customer cu WHERE cu.id = p_id), 2)
            WHEN p_type = 'account' THEN
                COALESCE((SELECT max(audit.tier_rank(casework.effective_tier(apr.party_id)))
                            FROM canon.account_party_role apr WHERE apr.account_id = p_id), 2)
            WHEN p_type = 'alert' THEN
                COALESCE((SELECT GREATEST(audit.tier_rank(casework.effective_tier(a.party_id)),
                                          audit.tier_rank(f.tier),
                                          CASE WHEN sc.visibility = 'restricted' THEN 2 ELSE 0 END,
                                          COALESCE((SELECT max(cr.r) FROM casework.case_alerts ca
                                                      JOIN case_rank cr ON cr.id = ca.case_id
                                                     WHERE ca.alert_id = a.id), 0))
                            FROM alert.alerts a
                            JOIN casework.case_families f ON f.code = a.case_family
                            LEFT JOIN scn.scenarios sc ON sc.id = a.scenario_id
                           WHERE a.id = p_id), 2)
            WHEN p_type = 'pack' THEN
                COALESCE((SELECT CASE WHEN p.kind IN ('board', 'certification', 'assessor', 'cbi_gap', 'vendor_request',
                                                      'integration_profile') THEN 0 ELSE 2 END
                            FROM admin.packs p WHERE p.id = p_id), 2)
            WHEN p_type IN ('audit_log', 'access_log', 'anchor', 'setting', 'role_grant', 'user', 'branch',
                            'policy_bundle', 'change_request', 'register', 'validation_pack', 'training_record',
                            'assessor_sample', 'verify_run', 'fairness_metric', 'audit_stream', 'audit_partition',
                            'signing_key', 'database', 'antivirus', 'source', 'batch', 'inventory_item') THEN 0
            ELSE 2
        END AS rank
    )
    SELECT CASE rank WHEN 0 THEN 'normal' WHEN 1 THEN 'restricted' ELSE 'suspicion' END FROM r
$$;
ALTER FUNCTION audit.tier_rank(TEXT) OWNER TO akr_migrate;
ALTER FUNCTION audit.resource_tier(TEXT, UUID, UUID) OWNER TO akr_migrate;
REVOKE ALL ON FUNCTION audit.tier_rank(TEXT) FROM PUBLIC;
REVOKE ALL ON FUNCTION audit.resource_tier(TEXT, UUID, UUID) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION audit.tier_rank(TEXT) TO akr_readonly;
GRANT EXECUTE ON FUNCTION audit.resource_tier(TEXT, UUID, UUID) TO akr_readonly;
-- No table or column grant to akr_readonly on casework, alert, scn or canon (lead review): the function reads them as
-- its owner. The alert's own case_ref lookup and the access-history case resolution of section 3.2 call it the same way.
'''
s=s[:a]+newfn+s[b:]
rep("Note: `casework.effective_tier` is phase 17's function (`SECURITY INVOKER`, `EXECUTE` granted to `akr_app` only). Phase 24 adds the `akr_readonly` grant; it changes nothing of the function.",
    "Note: `casework.effective_tier` is phase 17's function; `audit.resource_tier` calls it as its owner, so phase 17's grants are unchanged (lead review: the draft granted `akr_readonly` five column reads and EXECUTE, against phase 17's rule that `akr_readonly` reads nothing in `casework`, and `akr_readonly` has no USAGE on `canon`).")
rep("1 new `audit` function and 5 column grants for `akr_readonly`","2 new `audit` functions (SECURITY DEFINER tier resolver, no new table grant to `akr_readonly`)")
rep("6. Query on `readonly_pool`, keyset on `(occurred_at, id)`, page size `min(limit, audit.search_page_size)`. The predicate of every row is the tier rule: `audit.resource_tier(resource_type, resource_id, case_id) <> 'suspicion'` for `audit_log`, `tier <> 'suspicion'` for `access_event` (the tier stored when the access happened).",
    "6. Query on `readonly_pool`, keyset on `(occurred_at, id)`, page size `min(limit, audit.search_page_size)`. The predicate of every row is the tier rule: `audit.resource_tier(resource_type, resource_id, case_id) <> 'suspicion'` for `audit_log`; for `access_event` the higher of the stored `tier` and the current `audit.resource_tier(resource_type, resource_id, NULL)` must not be `suspicion` (a record raised later hides its earlier reads too).")
rep("9. **Phase 17 and 18** `casework.effective_tier` is granted to `akr_readonly` (0078); `casework.decisions` is read as a view",
    "9. **Phase 17 and 18** `casework.effective_tier` is called by the SECURITY DEFINER `audit.resource_tier` (0078; no grant change); `casework.decisions` is read as a view")
# 2 MEDIUM: tier-only predicates
rep("Predicate: `cases.tier <> 'suspicion' OR caller is M`. CH and IA see the normal and restricted decisions; M sees all.",
    "Predicate: phase 17's `services::visibility::build` for the caller (with phase 18's case access list and phase 19's QA grants as admitting clauses), never the tier alone (lead review: phase 17 rule 4 limits restricted cases to A1, A2 and M with an `assigned_work` purpose). CH and IA see normal-tier decisions and the decisions their own QA grants admit; M sees all. Other decisions are counted on G1 only, suppressed.")
rep("Visibility: a row whose subject tier is `suspicion` is excluded for everyone but M (both the row and the count).",
    "Visibility: a row tied to a case, an alert, an RFI, an internal report, a freeze, a refrain or an authority request passes the source's own predicate (phase 17 `services::visibility::build`, phase 19 and phase 20 predicates), never the tier alone; for CH and IA that admits normal-tier subjects and what their own grants admit; internal reports and suspicion subjects are M's (row and count).")
rep("| Registers, decisions (3.4) | M, CH, IA | decision type, reason codes, date, decider id; suspicion decisions visible to M only | nothing about suspicion for CH and IA |",
    "| Registers, decisions (3.4) | M, CH, IA | decision type, reason codes, date, decider id of the cases the phase 17 predicate admits for the caller | nothing about restricted or suspicion cases for CH and IA |")
rep("| Clocks board (3.8) | M, CH, IA | due instants and subject ids of normal and restricted subjects | nothing: suspicion rows are excluded for everyone but M |",
    "| Clocks board (3.8) | M, CH, IA | due instants and subject ids of the subjects each source's predicate admits | nothing: restricted and suspicion subjects follow their source's predicate |")
# 3 MEDIUM: trace bounded
rep("**`trace(ctx, alert_id)`** (`assessor.trace`; AS, and IA for a record that is not suspicion-tier; route 28). The trace has six nodes,",
    "**`trace(ctx, alert_id)`** (`assessor.trace`; AS, and IA for a record that is not suspicion-tier; route 28). AS traces an alert only when it is normal tier or an item of a sample drawn under the caller's grant in force (`admin.assessor_samples.grant_id`; for a `report_register` sample, an alert of a sampled report's case); any other alert is the masked 404 (lead review: the designed exception covers the recorded sample, not the alert register). The trace has six nodes,")
rep("@id(\"assessor.trace.as\") @role(\"AS\")\npermit (principal, action == Akr::Action::\"assessor.trace\", resource is Akr::AuditTarget)\nwhen { principal.roles.contains(\"AS\") };",
    "@id(\"assessor.trace.as\") @role(\"AS\")\npermit (principal, action == Akr::Action::\"assessor.trace\", resource is Akr::AuditTarget)\nwhen { principal.roles.contains(\"AS\") && (resource.tier != \"suspicion\" || resource.in_own_sample) };")
rep("entity AuditTarget = { kind: String, tier: String };","entity AuditTarget = { kind: String, tier: String, in_own_sample: Bool };   // in_own_sample: an item of a sample drawn under the caller's grant in force")
# 4 MEDIUM: assessor scope on pack routes
rep("| POST | `/api/v1/packs` | `dashboard.read`, `data.steward` | [ext] Build a pack;","| POST | `/api/v1/packs` | `dashboard.read`, `data.steward`, `assessor.read` | [ext] Build a pack (AS: kind `assessor` only, Cedar);")
rep("| GET | `/api/v1/packs/{id}` | `dashboard.read`, `data.steward` | [ext]","| GET | `/api/v1/packs/{id}` | `dashboard.read`, `data.steward`, `assessor.read` | [ext]")
rep("| GET | `/api/v1/packs/{id}/download` | `dashboard.read`, `data.steward` | [ext]","| GET | `/api/v1/packs/{id}/download` | `dashboard.read`, `data.steward`, `assessor.read` | [ext]")
rep("2. **Phase 06** `POST /packs`: accepts the five kinds;","2. **Phase 06** `POST /packs`, `GET /packs/{id}` and the download: accept the scope `assessor.read` beside `dashboard.read` and `data.steward` (AS holds no other scope; Cedar keeps AS to kind `assessor`); `POST /packs` accepts the five kinds;")
# 5 LOW floors
rep("'{\"min\": 30, \"basis\": \"GV-34 quarterly committee cycle; never less than 30 days\"}'::jsonb","'{\"min\": 30, \"max\": 365, \"basis\": \"GV-34 quarterly committee and yearly Board cycle\"}'::jsonb")
rep("'{\"min\": 1, \"basis\": \"GV-38 internal review trigger; never below parity\"}'::jsonb","'{\"max\": 1.25, \"basis\": \"GV-38 internal review trigger; a bank may only tighten it\"}'::jsonb")
rep("'{\"min\": 1, \"basis\": \"M-62 time-boxed read-only assessor grant\"}'::jsonb","'{\"max\": 30, \"basis\": \"M-62 time-boxed read-only assessor grant\"}'::jsonb")
rep("`register.finding_due_days` (90; governed; floor 30), `fairness.review_ratio` (1.25; governed; floor 1),","`register.finding_due_days` (90; governed; floor min 30, max 365), `fairness.review_ratio` (1.25; governed; floor max 1.25),")
rep("`assessor.grant_max_days` (30; governed; floor 1)","`assessor.grant_max_days` (30; governed; floor max 30)")
# 6 LOW retention
rep(" ('admin', 'registers',            'R9',  'age',       'updated_at',  'Findings and warnings: the last change dates the row; an open row is never older than its opening'),",
    " ('admin', 'registers',            'R9',  'closure',   'closed_at',   'Findings and warnings: from closure; an open row is never due'),")
rep("21. **The register's clock.** The register's retention clock is `updated_at` (an open row is never older than its opening). Ten years of silence on an open finding would leave it to the engine's floor; the bank accepts this.",
    "21. **The register's clock.** The register's retention clock is `closure` on `closed_at` (lead review), so an open finding is never due; the bank confirms the R9 period for closed findings.")
# 7 LOW control report suppression
rep("Audit `access.control_report_generated {period}`.","Every count from 1 to `report.small_number_threshold` minus 1 is returned as `null` with `suppressed: true`, as on the command centre (lead review). Audit `access.control_report_generated {period}`.")
# 8 LOW conventions and names
rep("Columns: decision id, `case_ref`, `type`, `reason_codes`, `decided_at`, `decider_id`, `proposer_id`, `outcome`.","Columns: decision id, `case_ref` (from `casework.cases`), `type`, `reason_codes`, `decided_at`, `decider_id`, `proposer_id`.")
rep("`verify: Arc<AuditVerifyService>`, ","")
rep(") AS v(t, cls, cols), unnest(v.cols) AS c",") AS v(t, cls, cols) CROSS JOIN LATERAL unnest(v.cols) AS c")
open(p,'w').write(s)
print('part1 ok')
