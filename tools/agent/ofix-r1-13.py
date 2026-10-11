#!/usr/bin/env python3
"""Owner-decision round 1 fixes: phase 13 (verdicts O-13-1..12, L-1, L-2, L-6 and the carried O-19-1/3/4/5/9)."""
import sys
sys.path.insert(0, "/home/haithem/Projects/AKR-TMS/tools/agent")
from ofix import Fixer

f = Fixer(round_no=1)
P = "docs/plan/phase-13.md"

# O-13-12: the owner note cites the header entry only
f.fix("O-13-12", P,
      "the government-source flag to P2 (header, S-17; section 7.7); lead-written,",
      "the government-source flag to P2 (header, S-17); lead-written,")

# O-19-1 (carried): who owns the tables
f.fix("O-19-1", P,
      "added to migration `0044_feat_counterparty_links.sql` as its second part; owner `akr_app`):",
      "added to migration `0044_feat_counterparty_links.sql` as its second part; owner `akr_migrate`, written by `akr_app`):")

# L-2: how a tag name stands in a feature `where`
f.fix("L-2", P,
      "In the `where` of a feature definition (section 3.1) the name is true for a posting whose counterparty carries a row of that tag that holds on the posting's `booking_date`. The name says nothing about direction:",
      "In the `where` of a feature definition (section 3.1) the name is written as a comparison with a boolean literal, `tag.payroll = true` or `tag.payroll = false`, the form the Filter subset of DSL.md has (as `txn.cash_flag = true` in the shipped catalogue). `tag.payroll = true` holds for a posting whose counterparty carries a row of that tag that holds on the posting's `booking_date`; `tag.payroll = false` holds for every other posting. Any other use of the name in a `where` (the bare name, another operator, a value that is not a boolean) is `E_FIELD_TYPE`. The name says nothing about direction:")
f.fix("L-2", P,
      "`FeatureCompiler` compiles the name to the following, with the tag as a bound parameter:",
      "`FeatureCompiler` compiles `tag.<name> = true` to the following, with the tag as a bound parameter, and `tag.<name> = false` to `NOT` of the same expression:")
f.fix("L-2", P,
      "`cargo test`: the validator accepts `tag.payroll` in a `where` and refuses it in `field` (`E_UNKNOWN_ATTRIBUTE`);",
      "`cargo test`: the validator accepts `tag.payroll = true` and `tag.payroll = false` in a `where`, refuses the bare name there (`E_FIELD_TYPE`) and refuses the name in `field` (`E_UNKNOWN_ATTRIBUTE`);")
f.fix("L-2", P,
      "the compiled filter of a fixture feature `payroll_credit_1m` holds the `EXISTS` of item 3;",
      "the compiled filter of a fixture feature `payroll_credit_1m` (`where`: `txn.dr_cr = 'C' AND tag.payroll = true`) holds the `EXISTS` of item 3, and the same feature with `tag.payroll = false` holds `NOT EXISTS`;")

# O-13-4: what `value: false` means in a static segment
f.fix("O-13-4", P,
      "`{ \"ref\": \"tag.payroll\", \"op\": \"=\", \"value\": true }` (or `false`) is true for a customer with a `feat.customer_source_tags` row of that tag. Only `=` with a boolean is accepted (`E_FIELD_TYPE` otherwise). The assignment of section 3.3 step 1 compiles it to `EXISTS (SELECT 1 FROM feat.customer_source_tags s WHERE s.customer_id = cu.id AND s.tag = $n)`, where `cu` is the customer row of the page.",
      "`{ \"ref\": \"tag.payroll\", \"op\": \"=\", \"value\": true }` is true for a customer with a `feat.customer_source_tags` row of that tag; with `\"value\": false` it is true for a customer with no such row. Only `=` with a boolean is accepted (`E_FIELD_TYPE` otherwise). The assignment of section 3.3 step 1 compiles `true` to `EXISTS (SELECT 1 FROM feat.customer_source_tags s WHERE s.customer_id = cu.id AND s.tag = $n)`, where `cu` is the customer row of the page, and `false` to `NOT EXISTS` over the same subquery.")
f.fix("O-13-4", P,
      "a static segment with `tag.pension = true` compiles and a risk overlay with it is refused (`E_SEGMENT_INPUT_NOT_ALLOWED`);",
      "a static segment with `tag.pension = true` compiles to `EXISTS`, one with `tag.pension = false` to `NOT EXISTS`, and a risk overlay with either is refused (`E_SEGMENT_INPUT_NOT_ALLOWED`);")

# O-19-3, O-19-5, O-19-9 (carried) and O-13-1: the types
f.fix("O-19-3", P,
      "DTOs: `SourceTagChange { add: Vec<NewSourceTag>, retire: Vec<RetireSourceTag> }`,",
      "DTOs: `SourceTagChange { add: Vec<NewSourceTag>, retire: Vec<RetireSourceTag>, reason_code: String, reason_text: String }` (the two reason fields are the ones phase 04 `create` requires),")
f.fix("O-19-5", P,
      "`SourceTagChange` and `ChangeRequestAccepted` (reused). Repository trait",
      "and `SourceTagChange`; the POST answers the phase 04 `ChangeRequestView`. Repository trait")
f.fix("O-19-9", P,
      "    async fn insert(&self, tx: &mut Tx, row: &SourceTag) -> Result<(), RepoError>;",
      "    async fn insert(&self, tx: &mut Tx, row: &SourceTag) -> Result<SourceTag, RepoError>;   // RETURNING")
f.fix("O-13-1", P,
      "    /// Sets `ended` (with `valid_to`) or `void` on an `active` row; `Ok(false)` when the row is no longer active.",
      "    /// Sets `ended` (with `valid_to`) or `void` on an `active` row, and for both `retired_at = now()` and\n    /// `retired_by_request = cr`; `Ok(false)` when the row is no longer active.")

# O-13-6: two adds of one change set that share a hash
f.fix("O-13-6", P,
      "and that is not in `retire`, is sub-code `already_tagged`. Each is 422 `E_VALIDATION` at `/add/<i>/subject_id`.",
      "and that is not in `retire`, is sub-code `already_tagged`. So is an entry that shares a hash with an earlier entry of `add` for the same tag (an account and a counterparty row of that account, for example), at the later entry. Each is 422 `E_VALIDATION` at `/add/<i>/subject_id`.")

# O-13-2: the default end date never falls before valid_from
f.fix("O-13-2", P,
      "`end` takes `valid_to` between the row's `valid_from` and today, and defaults to the last closed business date (phase 11 `FreshnessQuery`);",
      "`end` takes `valid_to` between the row's `valid_from` and today, and defaults to the later of the row's `valid_from` and the last closed business date (phase 11 `FreshnessQuery`), so the default never falls before `valid_from`;")

# O-19-4 (carried) and O-13-7: the creation step and the tags a change set names
f.fix("O-19-4", P,
      "   5. Create the change request through phase 04 `ChangeRequestService` (kind `source_tag`, `object_key` `source_tags`, reason family `change`), route `cr_material`.",
      "   5. Create the change request with phase 04 `ChangeRequestService::create` (origin Dedicated, so `submit` runs in the same transaction; kind `source_tag`, `object_key` `source_tags`, the caller's `reason_code` of family `change` and `reason_text`), route `cr_material`.")
f.fix("O-13-7", P,
      "and the two lists hold the keys of the active static segments and active feature definitions that read a tag name of the change set.",
      "and the two lists hold the keys of the active static segments and active feature definitions that read a tag name of the change set. The tag names of a change set are the tags of its `add` entries and the tags of the rows that its `retire` entries name (read by `id`), so a change set that only ends or voids a tag has its readers counted too.")

# O-13-1, O-13-5, O-13-8: the activation
f.fix("O-13-5", P,
      "| Insert each `add` as `active` with the hashes resolved again; close each `retire` as `ended` (with `valid_to`) or `void` (`retired_at = now()`, `retired_by_request`); enqueue `source_tag_refresh`; audit `source_tag.applied` |",
      "| In this order: close each `retire` as `ended` (with `valid_to`) or `void`, both with `retired_at = now()` and `retired_by_request` = this request; then insert each `add` as `active` with the hashes resolved again; enqueue `source_tag_refresh`; when a feature reads a tag of the change set, enqueue `feature_compute {reopen_from}` (first point below); audit `source_tag.applied` |")
f.fix("O-13-8", P,
      "   - **Volume effect.** `neutral` when both impact lists are empty: nothing reads the tags yet.",
      "   - **Order, and months already computed.** The retires are closed before the adds are inserted, because an add may take the place of a row that the same change set retires (item 6 step 2; this is how a payer name is changed) and `uq_source_tags_active` allows one active row per tag and account. A tag change also moves what a tag-reading feature gives for months that are already computed, and the hourly run does not take those months again. So, when `reading_features` of the stored impact is not empty, the activation enqueues `feature_compute {reopen_from}`, where `reopen_from` is the month-end of the month of the earliest date the change set moves: the earliest of the `valid_from` of every add, the `valid_from` of every voided row and the `valid_to` of every ended row (a row ended on day D still holds on D, so the month of D is the first that can change). The job is enqueued with the coalesce key `feature_compute:reopen:<reopen_from>`, a key of its own: under the key `feature_compute` a queued hourly run would swallow the enqueue (phase 00 `ON CONFLICT DO NOTHING`) and the reopen would be lost; the kind's concurrency of 1 still runs the jobs one at a time. Section 3.2 step 1 then takes every month from `reopen_from`, computes it again whatever its `basis` says (the register is not part of the basis), and step 5 updates only the rows whose value changed.\n   - **Volume effect.** `neutral` when both impact lists are empty: nothing reads the tags yet.")
f.fix("O-13-8", P,
      "enqueued by a `feature_definition` activation (with `catchup`) and by a replay of phase 11 |",
      "enqueued by a `feature_definition` activation (with `catchup`), by a replay of phase 11 and by a `source_tag` activation that a feature reads (with `reopen_from` and a coalesce key of its own, section 7.17 item 7) |")
f.fix("O-13-8", P,
      "after the tag is voided the row is gone and `segment_assign` is enqueued when a segment reads `tag.payroll`.",
      "after the tag is voided the row is gone and `segment_assign` is enqueued when a segment reads `tag.payroll`. With the fixture feature `payroll_credit_1m` active, the activation of a change set that voids a tag whose `valid_from` lies two months back enqueues one `feature_compute` job with that month's month-end as `reopen_from`, also while an hourly `feature_compute` job is queued, and after it ran the feature rows of the two months no longer count the credits.")
f.fix("O-13-5", P,
      "a second `active` row with the same tag and `account_hk` fails on `uq_source_tags_active`.",
      "a second `active` row with the same tag and `account_hk` fails on `uq_source_tags_active`. A change set that retires a row and adds the same subject under the same tag with another payer name activates without error (retires first).")

# O-13-9: the API reads canon for the proposal
f.fix("O-13-9", P,
      "- **Network zones:** no change. Only the worker reads `canon` for this phase; the API reads `feat` only.",
      "- **Network zones:** no change. The worker reads `canon` for this phase; the API reads `feat`, and `canon.account` and `canon.counterparty` (read only, as `akr_app`, which holds the SELECT grants of phase 10) for the source-tag proposal of section 7.17 item 6 only.")

# O-13-10: a D3 tab has no role list
f.fix("O-13-10", P,
      "built in this phase on the phase 08 layer; `screens.ts` lists the tab for CFG (propose) and for CHK, M and CH (read).",
      "built in this phase on the phase 08 layer. The tab has no role list of its own (phase 09 section 7.5): it renders when the caller may call `GET /ref/source-tags` (CFG, CHK, M and CH in the shipped bundle), and \"Propose changes\" renders when the caller may call the POST (CFG).")

# O-13-11: wording of items 9 and 10
f.fix("O-13-11", P,
      "9. **Audit events** (two more rows of section 3.10):",
      "9. **Audit events** (the two events named after the table of section 3.10):")
f.fix("O-13-11", P,
      "10. **Cedar actions** (two more rows of section 3.11; policies in `81_features.cedar`;",
      "10. **Cedar actions** (one more row of section 3.11, for both actions; policies in `81_features.cedar`;")

# L-1: the DELETE never reaches the guard as akr_app
f.fix("L-1", P,
      "As `akr_app`: `DELETE FROM feat.source_tags` fails with `source tags are never deleted`; an `UPDATE` of an `ended` row fails with `source tag is closed`;",
      "`DELETE FROM feat.source_tags` fails, as `akr_app` with permission denied (no DELETE grant) and as the owner `akr_migrate` with `source tags are never deleted`. As `akr_app`: an `UPDATE` of the `state` of an `ended` row fails with `source tag is closed`;")

# O-19-3 (carried): the curl body carries the reason
f.fix("O-19-3", P,
      "as CFG `POST /api/v1/ref/source-tags` with one `add` on a counterparty and `Idempotency-Key: tag-0001` gives 201",
      "as CFG `POST /api/v1/ref/source-tags` with one `add` on a counterparty, a `reason_code` of family `change`, a `reason_text` and `Idempotency-Key: tag-0001` gives 201; the same body without `reason_code` gives 422 `E_VALIDATION`; the first body again gives 201")

# L-6 (lead find while fixing): the base proposal DTOs do not name the reason fields either
f.fix("L-6", P,
      "  min size, `base_version?`), `PeerBuildRequest { currency }`. No Update struct:",
      "  min size, `base_version?`), `PeerBuildRequest { currency }`. Both proposals also carry `reason_code` and\n  `reason_text`, which phase 04 `create` requires (a code of a family the target accepts, section 3.7). No Update struct:")

# O-13-8 (design note): the same swallowing can hit the replay enqueue of the base text
f.fix("O-13-8", P,
      "A tag is matched on the other side of a posting only, so the debits of a tagged account itself are not marked.",
      "A tag is matched on the other side of a posting only, so the debits of a tagged account itself are not marked.\n- **Reopen enqueues and the coalesce key (found in the owner-decision round).** An enqueue of `feature_compute {reopen_from}` under the coalesce key `feature_compute` is dropped while another `feature_compute` job is queued (phase 00: `ON CONFLICT DO NOTHING`), and the reopen is lost. The source-tag activation uses a key of its own (section 7.17 item 7). The replay enqueue of section 3.2 has the same weakness as written and should take the same key form when it is built.")

f.done()
