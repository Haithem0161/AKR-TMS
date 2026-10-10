# Re-check, phase 26 (Haiku draft for lead review)

### R-26-1  LOW   (fix F6)
- Problem: The block says the two constants are declared "with phase 01's closed classes (section 4.5 gains this phase 01 item)", but section 4.5 has no phase 01 item and the block never defines one. Phase 01 needs no change, because both constants reuse the existing classes `record_open` and `decision`. The pointer leads to nothing.
- Proof: docs/plan/phase-26.md:904 "with phase 01's closed classes (section 4.5 gains this phase 01 item)"; docs/plan/phase-26.md:750 "### 4.5 Existing entity, schema, route and code updates" (items 1 to 11 name no phase 01 item).
- Smallest fix: On line 904, replace "(section 4.5 gains this phase 01 item)" with "(no change to phase 01)".

### R-26-2  LOW   (fix F12)
- Problem: The old text began at `duplicate_exists`, so the list separator `;` in front of it stayed behind. The new sentence now follows a stray semicolon, so the bullet reads as a list item running into a sentence.
- Proof: docs/plan/phase-26.md:268 "`decide(tx, id, expected_version, decision) -> PossessionEvidence`; `list_for_deal(deal_id)`;" and docs/plan/phase-26.md:269 "No duplicate method lives here: the tile method `islamic_duplicate_refs` of section 7+ counts the shared references when G1 is read."
- Smallest fix: On line 268, replace the final `;` after `list_for_deal(deal_id)` with a full stop.

### R-26-3  MEDIUM   (fix F15)
- Problem: The new text says the duplicate tile is "absent" for `a2@`. A2 does not hold `dashboard.read`, so the G1 route refuses `a2@` with 403 `E_SCOPE` and never returns the tile. The same scope rule gives 403 for `a1@` in test n. The M-only rule for the duplicate tile is therefore not tested for CH or IA, the two dashboard roles that read G1 and must not see it.
- Proof: docs/plan/phase-26.md:846 "and as `a2@` the tile is absent."; docs/plan/phase-26.md:795 "A2 holds no such scope, lead review"; docs/plan/phase-26.md:671 "dashboard.read admits M, CH and IA".
- Smallest fix: On line 846, replace "and as `a2@` the tile is absent" with "and as `ch@` and `ia@` the tile is absent (`a2@` gets 403 `E_SCOPE` on the route, having no `dashboard.read`)".

### R-26-4  MEDIUM   (fix F16)
- Problem: The `content` signature has no step-up header, but its step 1 runs steps 1 to 6 of 3.4, and step 6 consumes `DEAL_OPEN` with `header` (as fix F6 says). The `verify` signature in the same list takes `header`, so the omission is specific to `content`. Without it a suspicion-tier content read cannot take the `record_open` step-up that the route row requires, and no section 6 check tests that step-up for content. The base signature of `IslamicDealService::get` in 3.4 has the same gap; the lead may want the same parameter there.
- Proof: docs/plan/phase-26.md:928 "`content(ctx, deal_id, evidence_id, download: bool) -> VerifiedReader`"; docs/plan/phase-26.md:409 "1. Steps 1 to 6 of 3.4 (flag, deal read, tier, Cedar, purpose, step-up `record_open` on a suspicion-tier deal)"; docs/plan/phase-26.md:367 "`StepUpService::consume(tx, ctx, header, DEAL_OPEN, Some("islamic_deal:<id>"))`".
- Smallest fix: On line 928, change the signature to "`content(ctx, deal_id, evidence_id, download: bool, header) -> VerifiedReader`", as line 927 does for `verify`.

### R-26-5  MEDIUM   (fix F21)
- Problem: The trailer says phase 20 changes only the task grouping of section 3.8. The new block says section 3.8 gains a state rule that changes phase 20's `on_level` (a freeze moves to `frozen` only when every group task has a level). That is a second change to phase 20's freeze behaviour, so the trailer's "only" is now false. The exception for 7.x blocks does not apply, because the block does not say it replaces this trailer line.
- Proof: docs/plan/phase-26.md:1019 "No change to phase 20's freeze trigger, the screening freeze, the freeze release workflow or the neutral texts; only the task grouping of section 3.8."; docs/plan/phase-26.md:944 "Section 3.8 gains the state rule."
- Smallest fix: On line 1019, replace "only the task grouping of section 3.8" with "only the task grouping of section 3.8 and the group state rule of section 7.10".

### R-26-6  LOW   (fix F22)
- Problem: `FreezeService::release_freeze` is not a method. Phase 20 defines `release_freeze` as a terminal effect in the effects registry (docs/plan/phase-20.md:700 "**`release_freeze`** (terminal effect, new in phase 20;"). The `FreezeService` methods that phase 20 names are `start`, `on_level`, `on_task_completed`, `refer` and `propose_release`. The pointer to section 3.5 step 5 is right, but the qualifier names a method that does not exist.
- Proof: docs/plan/phase-26.md:527 "Release (`FreezeService::release_freeze`, phase 20 section 3.5 step 5, on approval)".
- Smallest fix: On line 527, replace "`FreezeService::release_freeze`" with "the terminal effect `release_freeze`".

### R-26-7  LOW   (fix F29)
- Problem: The fix points the "Checks of the file" paragraph of 3.13 at check 10. Check 10 names only the Cedar bundle validation and the generated-principal list-predicate test. The word-for-word forbid comparison and the role-reach check that the paragraph lists are in no section 6 check, and the route-scope check is covered only by some curl cases of check 6.
- Proof: docs/plan/phase-26.md:670 "Checks of the file (section 6, check 10): every route scope admits every role the file permits for that route"; docs/plan/phase-26.md:863 "10. The Cedar bundle validates (`akr-ctl policy validate` prints the new bundle hash with `88_islamic.cedar`)".
- Smallest fix: At the end of check 10 (line 863 to 864), add "; the route scopes, the role reach and the forbid text of section 3.13 are compared with `88_islamic.cedar` as section 3.13 states".

## Summary
- Fixes checked: 36 (F1 to F36, from tools/agent/verify/applied-26.txt).
- Findings: HIGH 0, MEDIUM 3, LOW 4 (7 in total).
- CHECKED: F1 ok, F2 ok, F3 ok, F4 ok, F5 ok, F6 R-26-1, F7 ok, F8 ok, F9 ok, F10 ok, F11 ok, F12 R-26-2, F13 ok, F14 ok, F15 R-26-3, F16 R-26-4, F17 ok, F18 ok, F19 ok, F20 ok, F21 R-26-5, F22 R-26-6, F23 ok, F24 ok, F25 ok, F26 ok, F27 ok, F28 ok, F29 R-26-7, F30 ok, F31 ok, F32 ok, F33 ok, F34 ok, F35 ok, F36 ok
