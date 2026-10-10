p='docs/plan/phase-23.md'
t=open(p).read()
R=[]
def r(a,b,n=1): R.append((a,b,n))
# 5: E_CLAIMED
r("error_code    TEXT CHECK (error_code IN ('E_UNAVAILABLE', 'E_INTERNAL')),",
  "error_code    TEXT CHECK (error_code IN ('E_CLAIMED', 'E_UNAVAILABLE', 'E_INTERNAL')),")
# 4: partition lock, not parent
r("(ROW EXCLUSIVE), which waits here; a row written into the parent (ROW EXCLUSIVE) waits for the parent lock below.",
  "(ROW EXCLUSIVE), which waits here; a row written into the partition (ROW EXCLUSIVE) waits for the partition lock below.")
r("""    -- 4. The parent is locked against writers, so the count below is final for this drop.
    EXECUTE format('LOCK TABLE %s IN EXCLUSIVE MODE', to_regclass(p_parent));""",
  """    -- 4. The partition (not the parent) is locked against writers, so the count below is final for this drop; writers of
    --    the other partitions go on. The caller sets statement_timeout from retention.drop_timeout_s (lead review).
    EXECUTE format('LOCK TABLE %s IN SHARE MODE', to_regclass(p_partition));""")
# 2: raw not blocked by party/case holds
r("""    --    A raw landing table has no party path (section 3.3): any party or case hold blocks its drop.
    SELECT count(*) INTO v_party_holds FROM admin.legal_holds h
     WHERE h.state <> 'released' AND h.kind IN ('party', 'case')
       AND (p_parent ~ '^raw\\.'
            OR NOT EXISTS (SELECT 1 FROM admin.legal_hold_copies c
                            WHERE c.hold_id = h.id AND c.run_id = p_run_id
                              AND c.source_table = p_parent AND c.partition_name = p_partition
                              AND c.partition_rows = v_rows));""",
  """    --    A raw landing table is neither copied nor blocked by party or case holds: its rows are mapped into canon in the
    --    same transaction (phase 11), the held copy is taken from canon, and step 6 keeps unmapped batches (lead review).
    SELECT count(*) INTO v_party_holds FROM admin.legal_holds h
     WHERE h.state <> 'released' AND h.kind IN ('party', 'case')
       AND p_parent !~ '^raw\\.'
       AND NOT EXISTS (SELECT 1 FROM admin.legal_hold_copies c
                        WHERE c.hold_id = h.id AND c.run_id = p_run_id
                          AND c.source_table = p_parent AND c.partition_name = p_partition
                          AND c.partition_rows = v_rows);""")
r("""    -- 7. Drop. The audit event is written by the caller in the same transaction.""",
  """    -- 7. Drop. DETACH takes the parent lock under the caller's lock_timeout (2 s). The audit event is written by the
    --    caller in the same transaction.""")
# 6: grants for quarantine_items, scn, ref
r("admin.xref, admin.attribute_values,\n", "admin.xref, admin.attribute_values, admin.quarantine_items,\n")
r("admin.evidence_objects, admin.workflow_instances, admin.column_classification TO akr_retention;",
  "admin.evidence_objects, admin.workflow_instances, admin.column_classification, admin.quarantine_items TO akr_retention;")
r("GRANT USAGE ON SCHEMA canon, feat, alert, kyc, casework, rpt TO akr_retention;",
  "GRANT USAGE ON SCHEMA canon, feat, alert, kyc, casework, rpt, scn, ref TO akr_retention;")
r("GRANT SELECT ON ALL TABLES IN SCHEMA canon, feat, alert, kyc, casework, rpt TO akr_retention;",
  "GRANT SELECT ON ALL TABLES IN SCHEMA canon, feat, alert, kyc, casework, rpt, scn, ref TO akr_retention;")
# 7 + drop timeout setting
r("('retention.period_years.R6', '10',", "('retention.period_years.R6', '7',")
r("('retention.period_years.R7', '10',", "('retention.period_years.R7', '7',")
r("""  '{"ar": "أقصى مدة لتشغيل الاحتفاظ بالدقائق", "en": "Maximum duration of one retention run in minutes"}')
ON CONFLICT (key) DO NOTHING;""",
  """  '{"ar": "أقصى مدة لتشغيل الاحتفاظ بالدقائق", "en": "Maximum duration of one retention run in minutes"}'),
 ('retention.drop_timeout_s', '600', 'duration_s', NULL, FALSE,
  '{"ar": "مهلة معاملة حذف القسم الواحد بالثواني", "en": "Time-out of one partition drop transaction in seconds"}')
ON CONFLICT (key) DO NOTHING;""")
r("| R6 | `closure` | 7 years from the later of closure and report | 10 years |", "| R6 | `closure` | 7 years from the later of closure and report | 7 years |")
r("| R7 | `closure`, `decision`, `age` | 7 years | 10 years |", "| R7 | `closure`, `decision`, `age` | 7 years | 7 years (10 under an OFAC profile, Open doubts 9) |")
# 6 registry
r("`alert.quarantine`, `alert.detection_keys` |", "`alert.quarantine`, `scn.detection_keys` (phase 16, `first_seen_at`) |")
r("| `partition_drop` | `raw.<source>` landing partitions (phase 11) |\n",
  "| `partition_drop` | `raw.<source>` landing partitions (phase 11) |\n| R1 | `age` on `created_at` | 30 days (product bound) | 90 days | `record_purge` by age in batches (lead review) | `admin.quarantine_items` (phase 11; partitioned by `extracted_at`, purged by row because its assignment clock is `age`) |\n")
# 1 alert guard
r("""**The alert partition guard (`alert.alerts`).** Before the call, the engine checks the partition for a row whose `state` is `queued` or `claimed` (the open states of phase 17). If one exists, the function is not called and the item is `blocked` with `open_row`. `triaged`, `duplicate` and `no_longer_matches` are the terminal states. Alerts of a month are created in that month, so the check cannot race with a new alert in an old partition. A drop of an alert partition does not damage a case: the case holds a snapshot of its evidence (phase 02, ARCHITECTURE s.17) and `casework.case_alerts` keeps the alert id as a plain reference.""",
  """**The alert partition guard (`alert.alerts`; lead review).** Before the call, the engine checks the partition, in order: (1) a row whose `state` is `queued` or `claimed` (the open states of phase 17): `blocked` with `open_row`; (2) a row named by `casework.case_alerts` (its foreign key `(alert_id, alert_created_at)` is `ON DELETE RESTRICT`, so the `DETACH` would fail, and the case is R6 and kept, DR-191): `blocked` with `referenced`; (3) a row whose disposition instant (`triaged_at` for `triaged`, `updated_at` for `duplicate` and `no_longer_matches`) is after the R5 cut-off: `skipped` with `not_due`. Only a partition that passes all three is dropped, so no alert leaves before seven years from its own disposition, whatever its `created_at`. Alerts of a month are created in that month, so the check cannot race with a new alert in an old partition; an attach to a case after the check waits on the partition lock of the function.""")
r("party and case holds (copy required, or block for raw)", "party and case holds (copy required; raw is not copied, step 5)")
# 3 party paths
r("""Steps for one `(hold, partition)` pair of the run (case holds are resolved to their parties first, section 3.8):
1. Select the held rows of the partition in batches of `retention.batch_rows`, by the party path of the table (section 3.5, "party path"). A raw landing table has no party path: no copy is made and the drop is blocked (section 3.3).""",
  """**Party paths of the droppable parents (lead review).** `canon.txn` and `canon.balance_snapshot`: `account_id` through `canon.account_party_role` (`party_id`); `canon.card_txn`: `card_id` through `canon.card.account_id` and the roles; `canon.locker_access` and `alert.alerts`: `party_id`; `feat.feature_values` and `feat.profiles`: `entity_id` where `entity` is a party or an account (through the roles); `canon.txn_event`: the events of the held `canon.txn` rows (its companion). `canon.cash_event`, `canon.cbs_user_activity`, `canon.fx_deal`, `canon.payment_message`, `canon.payment_message_party`, `feat.feature_runs` and `feat.segment_stats` have no party path in this phase: no copy can be made, so their drop is blocked (`copy_failed`) while a party or case hold is active (Open doubts 22). Raw landing tables are not copied (section 3.3, step 5).

Steps for one `(hold, partition)` pair of the run (case holds are resolved to their parties first, section 3.8):
1. Select the held rows of the partition in batches of `retention.batch_rows`, by the party path above. A parent with no party path gets no copy and its drop is blocked.""")
r("A raw landing table has no party path (section 3.3).\n", "Raw landing tables are not copied (section 3.3).\n")
# timeouts text
r("`SET LOCAL statement_timeout = '30s'` per batch;", "`SET LOCAL statement_timeout = '30s'` per batch (a partition drop transaction uses `retention.drop_timeout_s`, default 600 s, because it counts the partition);")
r("default `statement_timeout = 30s`)", "default `statement_timeout = 30s`; a drop transaction sets `retention.drop_timeout_s`)")
r("then the **partition parent lock** (`EXCLUSIVE`)", "then the **partition lock** (`SHARE`) and, for the `DETACH` only, the parent (lock_timeout 2 s)")
# DRs
r("the party-hold copy check under the parent lock,", "the party-hold copy check under the partition lock,")
r("The alert partition additionally waits for its open alerts.", "The alert partition additionally waits for its open alerts, for every alert a case names and for the disposition instant of every row (lead review).")
r("A raw landing table has no party path, so a party or case hold blocks its drop (Open doubts 22 for the path).",
  "A raw landing table is neither copied nor blocked by a party or case hold, because its rows are in canon where the copy is taken; a parent with no party path is blocked while such a hold is active (Open doubts 22).")
r("with the partition's row count recorded and compared under the parent lock.", "with the partition's row count recorded and compared under the partition lock.")
# open doubts
r("""3. **The R5 clock.** The assignment says `partition` on `created_at`; the research says closure. This phase drops alert partitions by month with the open-state guard; a closure clock would need row-level purges of alerts that cases reference. Default: the partition clock.""",
  """3. **The R5 clock.** The assignment says `partition` on `created_at`; the research says seven years from the report date. This phase drops an alert partition only when no row is open, no row is named by a case and every row's disposition is past the cut-off (section 3.3, lead review), so a month with one case-linked alert stays until the case purge is decided (Open doubts 1). The bank confirms that the disposition instant is the start of the R5 clock for alerts that led to no report.""")
r("""10. **Cost of the partition count.** The drop counts the partition under the parent's lock. For a large `canon.txn` month this can take minutes. Default: accept it in the nightly window with `lock_timeout` 2 seconds; an estimate with a re-count is the alternative and weakens the check.""",
  """10. **Cost of the partition count.** The drop counts the partition under its own SHARE lock (writers of other partitions continue) inside a transaction bounded by `retention.drop_timeout_s` (600 s). For a very large month the count may still exceed it; the item is then `failed` and retried. An estimate with a re-count is the alternative and weakens the check.""")
r("""`canon.counterparty` and `canon.payment_link` are `report_only` until their references are checked.""",
  """`canon.counterparty` and `canon.payment_link` are `report_only` until their references are checked. The parents without a party path (section 3.4) are blocked while any party or case hold is active; the bank's first hold will show whether their paths (through `canon.txn` for cash events and payment messages) are needed.""")
for a,b,n in R:
    c=t.count(a)
    assert c==n,(a[:70],c,n)
    t=t.replace(a,b)
open(p,'w').write(t)
print('ok',len(R))
