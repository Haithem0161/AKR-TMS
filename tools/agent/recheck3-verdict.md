# Third re-check verdict (lead)

Third re-check round, 2026-10-10. It covers the 29 replacements of the second re-check (ids `S-` and `L2-` in
`tools/agent/recheck-fixes.txt`), all of them in phase files. One Haiku agent per touched phase file (01, 03, 04, 05,
06, 08, 09, 11, 15, 16, 17, 18, 19, 21, 22, 27, 30) read `tools/agent/verify/applied3-NN.txt` and the phase file, and
wrote `tools/agent/verify/recheck3-NN.md`. Prompt: `tools/agent/p-recheck3.txt`. The lead decides every finding
against the phase text. Format: `T-NN-k | CONFIRMED severity | what | fix` or `T-NN-k | REJECTED | reason`.

Totals: 7 findings raised by the agents in 7 of the 17 phase files (10 had none): 3 confirmed (T-01-1, T-04-1, T-18-1) and 4 rejected (T-03-1, T-06-1, T-22-1, T-30-1: a section 7 block that extends base text, a column comment with no effect, a general rule that no mapping line repeats, and two lines that do not disagree). 2 more found by the lead while checking T-30-1 (L3-00-1, L3-30-1). By lead severity: 0 HIGH, 1 MEDIUM (T-01-1, a fault of the S-01-1 fix), 4 LOW. All 5 are fixed: 6 replacements (T-01-1 took two), entries 251 to 256 of `tools/agent/recheck-fixes.txt`, applied; the three checks are green after them. Not a clean pass.

## Phases with no finding
05, 08, 09, 11, 15, 16, 17, 19, 21: every entry `ok`. The phase 17 agent left one side note (phase 18 section 4.5
item 4), which the phase 18 agent raised as T-18-1.

## Phase 01
- T-01-1 | CONFIRMED MEDIUM (raised HIGH) | a fault of the S-01-1 fix: step 1 of section 7.9 now lets `disable` find the principal among active ones only, while step 3 commits `status = 'disabled'` before the Hydra delete and says the command can be run again after a failed Hydra call. The rerun finds no active principal, so the Hydra client could never be removed by the command. No stated check fails | step 3 says what the rerun does (no active principal of the name: it repeats only the Hydra delete for the `hydra_client_id` of each disabled service principal that carries it; no row changes and no second audit event), and step 1 names that one other use of the name (two replacements)

## Phase 03
- T-03-1 | REJECTED | block 7.15 is a section 7 block and extends step 3 of section 3.1: it states the rule in full (the permits of `dev.cedar` carry `@role("*")`, the loader accepts it for the `dev.*` actions, the effect is no `actor_role`). A 7.x block wins over base text, so the base wording of step 3, which names the `self` group only, is not a second rule. A builder has nothing to guess

## Phase 04
- T-04-1 | CONFIRMED LOW (raised MEDIUM) | the OpenAPI bullet (the S-04-1 text) calls the evidence route "a ninth operation of the tag `change-control`" while the same bullet still counts that tag as 8, so its tag counts still sum to 31 against the stated 32 | the tag count reads 9 and says of what

## Phase 06
- T-06-1 | REJECTED | the line is a column comment of the migration, outside the replacement and unchanged since gap pass 1 added the `feat.` and `rating.` namespaces (section 7.3); the agent itself says it has no build effect. "Canonical attribute keys" there names the keys of the catalogue file `canonical-attributes.v1.json`, to which the two namespaces are registered; nothing checks the column against the canonical model

## Phase 18
- T-18-1 | CONFIRMED LOW (raised MEDIUM) | section 4.5 item 4 still says `AssignmentService` "revokes the previous assignee's grant on reassignment" with no condition: the third place of the sentence that L2-17-1 corrected in two (phase 17 section 7.2, phase 18 section 3.2). The lead's sweep for the second re-check missed it | the item names the two reasons and points to section 3.2

## Phase 22
- T-22-1 | REJECTED | the sentence of 7.11 names the template that phase 04 puts in the payload, as every row of the mapping table of section 3.8 does; the swap of step 5 of the dispatcher (a suspicion-tier resource gives `task.waiting`, DR-208) is one general rule over every mapping and is stated in step 5, in the tipping-off table and in the check of section 6. No mapping statement repeats it. The clause also stood in the OLD text of the replacement, as the agent says, so it is outside this round

## Phase 27
No finding: every entry `ok`.

## Phase 30
- T-30-1 | REJECTED | section 3.1 lists what `deploy/zones.yaml` holds when the renderer reads it, and its list already includes a key this phase adds (the payment hub CIDRs, `gateway.cidrs`); "(phase 00)" names where the file comes from, not who wrote each entry. Section 4.4 says which keys this phase adds. The two lines do not disagree. The agent's grep did show a true gap elsewhere: L3-00-1 below

## Lead's own finds (outside the replacements, but true)
- L3-00-1 | CONFIRMED LOW (lead) | phase 00 section 7.7 lists the keys that phase 30 section 4.4 adds to `deploy/zones.yaml` and names three; that section names four since S-30-3 (`backup.repository`, which R-30-10b brought in). A mirror list out of step, found while checking T-30-1 | the phase 00 list names `backup.repository` (phase 30 sections 4.4 and 7.5)
- L3-30-1 | CONFIRMED LOW (lead) | phase 30 section 3.1 says "Inputs (both in `deploy/`, the first new in this phase)", but the first bullet is `zones.yaml` (phase 00) and the second is `catalogue.yaml` (new). A wrong ordinal in base text, seen while checking T-30-1; each bullet labels itself, so nothing is built wrong | "the second new in this phase"
