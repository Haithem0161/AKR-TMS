# Second re-check verdict (lead)

Second re-check round, 2026-10-10, on the user's word. It covers the 221 replacements of the first re-check
(`tools/agent/recheck-fixes.txt`). One Haiku agent per phase read `tools/agent/verify/applied2-NN.txt` (the 212
replacements made in phase files, grouped by file, each with the lead's verdict line) and the phase file, and wrote
`tools/agent/verify/recheck2-NN.md`. Prompt: `tools/agent/p-recheck2.txt`. The lead decides every finding against the
phase text. Format: `S-NN-k | CONFIRMED severity | what | fix` or `S-NN-k | REJECTED | reason`.

Totals: 32 findings raised by the agents in 19 of the 32 phases (13 phases had none): 27 confirmed and 5 rejected (S-09-2, S-14-1, S-24-1, S-25-1, S-29-1: three name text that stood before the replacement, two read a designed pattern as a conflict). 2 more found by the lead from the side notes of the phase 17 agent (L2-01-1, L2-17-1). By lead severity: 2 HIGH (S-27-2, S-30-2, both faults of the lead's own first-round fixes), 7 MEDIUM (S-01-1, S-03-1, S-03-2, S-03-4, S-06-1, S-17-1, S-27-1), 20 LOW. All 29 are fixed: 29 replacements (`tools/agent/recheck-fixes.txt`, ids `S-` and `L2-`; entries 222 to 250), applied by `tools/agent/recheck-fix.py`, which now keeps a ledger of applied ids (`tools/agent/recheck-fixes.done`) because a later entry may change text an earlier entry wrote. The three checks are green after the fixes. Not a clean pass: the plan is ready when a round finds 0 true gaps.

## Lead's own check: the 9 replacements outside the phase files
- R-13-1b (docs/DSL.md): `EXC_POOL_ACCOUNT` stands at line 374; no `EXCL_` scenario code is left (the two
  `MAP_EXCL_GL_INTERNAL` hits are a mapping code of another family). OK.
- R-19-1g, R-25-10d, L-04-1b, L-06-1, L-15-1d, L-18-1d, L-24-1d, L-99-1 (docs/plan/status.md): the Routes column was
  compared with the distinct route rows of each phase file by a script; the only differences are the designed ones
  (phase 00 infrastructure routes, the extended rows of phases 11, 16 and 24, the dev route of phase 17). The column
  sums to 581, the number of API.md rows (`tools/routecheck.py`). OK.


## Phase 11
- S-11-1 | CONFIRMED LOW (raised HIGH) | section 4 crate paragraph (the R-11-5b text): "none of the last three may link a driver" names akr-ingestd, akr-worker and akr-ctl, and akr-ingestd is the one binary that links a driver (the sentence before says so). A wrong word; the rule itself is stated right one sentence earlier | "and of these four only akr-ingestd links a driver"
- S-11-2 | CONFIRMED LOW (raised MEDIUM) | section 3.5 resume step 4 lists the payload of `source.halt_resolved` as `{decision, tables, check_id}`; block 7.13 (the R-11-1a text) adds `as_of_marker` for one decision and is headed "resume step 3", so it does not say it extends step 4. Stale base text under a 7.x block | step 4 points to 7.13
- S-11-3 | CONFIRMED LOW (raised MEDIUM) | block 7.20 item 4 (the R-11-7 text) takes the columns from the newest accepted manifest but still refuses only "a table with no receipt yet"; a table whose only receipt is a rejected batch has a receipt and no accepted manifest | the refusal names the same test: no accepted manifest yet

## Phase 27
- S-27-1 | CONFIRMED MEDIUM | section 3.16 still says "each batch is one transaction" for the sweep, and section 3.4 step 6 (the R-27-5 text) commits one transaction per chunk of a batch | section 3.16 names the chunk for the sweep and keeps the batch for the monitor
- S-27-2 | CONFIRMED HIGH | test l (the R-27-7e text) still expects 422 `E_PURPOSE_REQUIRED` for a team member with step-up and no purpose on a suspicion-tier alert, and the error row of section 3.11 says the same. Rule 5 of phase 17 admits no caller without a purpose code, so step 1 of the route (the R-27-7a text) answers the masked 404 "whatever header is missing", as phase 17 section 3.10 and phase 03 section 7.14 say; 422 is the answer on a restricted alert only. The stated check and the stated rule cannot both hold: the lead's own half-finished R-27-7 fix | test l expects the masked 404; the error row names the restricted tier only
- S-27-3 | CONFIRMED LOW | test l (the R-27-7e text) cites phase 03 section 7.14 right after the latency floor; the floor is phase 03 section 3.6 step 3, and 7.14 is the source of the answer | both are cited, each for its part

## Phase 01
- S-01-1 | CONFIRMED MEDIUM | section 7.9 step 1 (the R-01-1b text): a name is refused only while a principal that is not disabled holds it, so a disabled principal and a later active one can share a name, and the lookup of `rotate` and `disable` has no status condition and can match both rows | the lookup names `status = 'active'`

## Phase 14
- S-14-1 | REJECTED | `GoodGuyRuleInput` stands in the OLD text of the replacement as well: the replacement added only `GoodGuyRevoke`, so the name is outside this round. The parts of the rule object are named in prose in section 3.8 ("folded tokens that must all be present, and optional identifier conditions"), which the lead's scale accepts in place of a struct definition

## Phase 21
- S-21-1 | CONFIRMED LOW (raised HIGH) | the trailer (the R-21-2 text) says this phase "only implements the phase 20 port method `LetterRequester::delivered`"; sections 2 and 3.15 and section 4.5 say it implements the whole port, `request` too. A wrong word in the scope trailer; the body is right | the trailer names the port and both methods

## Phase 03
- S-03-1 | CONFIRMED MEDIUM (raised HIGH) | block 7.15 (the R-03-1 text) says the inventory of section 3.1 step 3 holds in a `dev-routes` build, but step 3 wants `@role` on every permit and the block names no annotation for the permits of `dev.cedar` | the permits carry `@role("*")`, which step 3 accepts for the `dev.*` actions as for the `self` group
- S-03-2 | CONFIRMED MEDIUM (raised HIGH) | the three actions are in `Action::ALL` whenever the feature is compiled in, but the fragment and `dev.cedar` were appended only when `AKR_ENV != production` too, so a `dev-routes` build started with the production value (the phase 00 check of section 6 does this) fails the inventory | the loader appends both whenever the feature is on; `AKR_ENV` governs only the mount (7.13)
- S-03-3 | CONFIRMED LOW (raised MEDIUM) | the schema test of section 2 compares `Action::ALL` with "the shipped `.cedarschema` text", which lacks the three actions under the feature; the block named only step 3 | the block says the test reads the shipped text plus the fragment under the feature
- S-03-4 | CONFIRMED MEDIUM | block 7.12 (the R-03-2 text) prints `roles` as the `RoleActions` list of 7.16 and cites phase 08 section 2, which still gives the map `{<role>: [<action>, ...]}`: two shapes for the one file `policy-matrix.json` that phase 08 reads | phase 08 section 2 gives the list
- S-03-5 | CONFIRMED LOW (raised MEDIUM) | block 7.17 (the R-03-4 text) says a miss and a refusal "cannot be told apart by an event"; section 3.2 step 8 writes the access event for a refusal and none for a miss. The sentence means the caller | "a caller cannot tell a miss from a refusal on them by the answer or by its timing"

## Phase 05
- S-05-1 | CONFIRMED LOW (raised MEDIUM) | section 3.3 still says the per-year counts show the curve "the backfill planner of phase 12 uses"; the trailer (the R-05-4 text) and DR-380 say no phase builds one. A stale aside with no building consequence | the aside names the object's volume curve only

## Phase 15
- S-15-1 | CONFIRMED LOW (raised MEDIUM) | the doc comment above `open_or_join` still says "returns (review, created)"; the signature (the R-15-7b text) and the next comment line give three parts | the stale clause is removed

## Phase 16
- S-16-1 | CONFIRMED LOW | block 7.15 (the R-16-2 text) puts the `capacity` exception under "DSL D.3 holds"; D.3's Minor row has no such exception, the rule is the `reason_code` row of DSL D.1 (S-39) | the lead-in names both

## Phase 22
- S-22-1 | CONFIRMED LOW (raised MEDIUM) | section 7 (the R-22-1 text) says an approver learns of an overdue task from `task.overdue`; phase 04 section 7.8 sends it on a `remind` timeout only, and most shipped approval steps have an `escalate` timeout alone. Descriptive text; phase 04 builds the rule | the clause is limited to a step with a `remind` timeout

## Phase 29
- S-29-1 | REJECTED | block 7.7 adds one kind to the table of section 3.17, as a 7.x block does, and section 4.5 item 2 (the R-29-3 text) counts exactly that: the 18 rows of the base table and `assess.rail_added` of section 7+. The two lines say the same thing; no developer is misled

## Phase 19
- S-19-1 | CONFIRMED LOW | section 5.5 (the R-19-4b text) words the exception as covering "party name, account or contact"; section 3.14 and section 2 allow only the parties of the case to be named. Phase 28 lets no account or contact reach the output, so this is wording | section 5.5 uses the wording of section 3.14

## Phase 04
- S-04-1 | CONFIRMED LOW (raised MEDIUM) | the OpenAPI bullet (the R-07-1b text) totals 32 operations but its tag counts sum to 31: the evidence route of section 7.2 is under no tag and in no router. The total is right | the bullet names the tag `change-control` and the router for it; the Routers bullet of section 2 says 9 with that route (two replacements)

## Phase 06
- S-06-1 | CONFIRMED MEDIUM (raised HIGH) | block 7.11 (the R-06-5 text) defines `AttributeKey` as "one canonical attribute reference"; section 7.3 gives catalogue keys the form `<namespace>.<name>` and admits the `feat.` and `rating.` namespaces, which are not canonical columns. Both have the same dotted form, so nothing fails to build, but the definition is too narrow | `AttributeKey` holds one key of the `AttributeCatalogue` of section 7.3
- S-06-2 | CONFIRMED LOW | the same sentence places `grade` in section 2; it is in section 3.2 | pointer corrected (same replacement as S-06-1)
- S-06-3 | CONFIRMED LOW | the same sentence cites DSL.md B.1 for `CanonicalRef`; it is defined in DSL.md section 1 | the citation leaves with the rewrite (same replacement as S-06-1)

## Phase 09
- S-09-1 | CONFIRMED LOW (raised MEDIUM) | the Keys paragraph of 7+ says each file of `backend/packs/reference/` carries the rows of one table; the calendar file (the R-09-4 text, same paragraph) carries the rows of two | the first sentence names the exception
- S-09-2 | REJECTED | `calendar_fixed` stands in the OLD text of the replacement as well: the replacement only removed two other codes, so the code is outside this round. Its check is step 4 of section 3.5 (`calendar_id` "is not a fixed clock")

## Phase 24
- S-24-1 | REJECTED | the two exceptions (G5 has no entry, G6 is never exported) stand in the OLD text of the replacement as well; the replacement changed only the reason given for G5. Section 7.23 states them as named exceptions to the general sentence of 7.12 and says what a screen with no entry answers (422): a rule with stated exceptions, not two rules

## Phase 25
- S-25-1 | REJECTED | the phase has 22 distinct routes, as the header, the Route count bullet and status.md say. The grep of the prompt counts 23 lines because the row of the rollout upload stands twice by design: in the section 2 table and in block 7.22, which says "the route table of section 2 gains one row" and shows it. The two rows agree

## Phase 30
- S-30-1 | CONFIRMED LOW | the recovery table of section 3.8 still cites "(section 3.11)" for the bundle on offline media, the pointer R-30-6 removed from 7.7; section 3.11 is upgrades and rollback | the row cites section 7.7 item 1
- S-30-2 | CONFIRMED HIGH | block 7.5 item 1 (the R-30-10a text) lets the allows select every pod of `akr-restore`; item 2 runs the known-bad probe of section 3.3 there, an unlabelled pod that must reach nothing, and its rows must pass before the restore starts. With every pod selected the probe reaches the repository, the object store and OpenBao, so the stated check fails: a fault of the lead's own fix | the allows select a membership label that only the Job and the scratch pod carry
- S-30-3 | CONFIRMED LOW (raised MEDIUM) | section 4.4 still says `zones.yaml` gains three keys that the renderer reads; `backup.repository` (the R-30-10b text) is a fourth | the bullet names four

## Phase 17
- S-17-1 | CONFIRMED MEDIUM (raised HIGH) | block 7.25 step 3 (the R-17-5 text) answers 422 `role_cannot_decide` to a second worker without `A2` or `M`; the check of section 6 expects 403 `E_SELF_APPROVAL` when `a1` (an A1) is named as L2 after being L1, and the order of the two refusals is not stated | the self-approval refusal of section 3.8 step 4 is checked first

## Lead's own finds (from the side notes of the phase 17 agent; outside the replacements, but true)
- L2-01-1 | CONFIRMED LOW (lead) | phase 01 section 3 says "a `record_open` step-up proves presence and is never consumed"; the steps of `consume` in the same section spend every assertion, the ledger is single-use (DR-29), and phases 17, 18, 19, 26, 27 and 28 call `consume` for a suspicion open and list a spent assertion among its refusals. One clause against the rest of the plan | the clause says that reading `fresh_until` spends nothing and that a `record_open` assertion is spent by `consume` like any other
- L2-17-1 | CONFIRMED LOW (lead) | phase 17 section 7.2 and phase 18 section 3.2 still say a reassignment revokes the previous assignee's grant without a condition; phase 17 section 7.25 (the R-17-6 text) revokes it only when its reason is `assignment` or `second_review` and keeps a wider grant | both sentences name the two reasons and point to 7.25
