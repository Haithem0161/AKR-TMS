# Fourth re-check verdict (lead)

Fourth re-check round, 2026-10-10. It covers the 6 replacements of the third re-check (ids `T-` and `L3-` in
`tools/agent/recheck-fixes.txt`), all of them in phase files. One Haiku agent per touched phase file (00, 01, 04, 18,
30) read `tools/agent/verify/applied4-NN.txt` and the phase file, and wrote `tools/agent/verify/recheck4-NN.md`.
Prompt: `tools/agent/p-recheck4.txt`. The lead decides every finding against the phase text. Format:
`U-NN-k | CONFIRMED severity | what | fix` or `U-NN-k | REJECTED | reason`.

Totals: 0 findings raised by the agents in the 5 phase files; all 6 replacements `ok`, and no side note. 1 found by the lead (L4-27-1, HIGH): an open point the lead had noted in an earlier round and left aside, judged before the round was closed. So 1 true gap, fixed by 1 replacement (entry 257 of `tools/agent/recheck-fixes.txt`), applied; the three checks are green after it. Not a clean pass, because of the lead find.

## Phases 00, 01, 04, 18, 30
No finding: every entry `ok` (F1 of 00, F1 and F2 of 01, F1 of 04, F1 of 18, F1 of 30). The phase 04 agent counted the
routes again: 32, with tag counts 9, 10, 8, 3, 1 and 1.

## Lead's own find (outside the replacements, but true)
- L4-27-1 | CONFIRMED HIGH (lead) | phase 27 check 6 of section 6 says the predicate of `services::visibility::build` and "the Cedar permit of `alert.model_score.read`" agree on every row. The three permits of that action are by role only (section 3.12, with the comment that visibility is applied in the handler), no forbid of phase 17 section 3.17 names the action, and phase 03 has no tier forbid over every action. So the Cedar decision for the action allows every A1, A2 and M on every alert, the predicate does not, and the check as written cannot pass. The design is the handler's (section 3.4 step 1); the check was worded for the phase 17 pattern, where the forbids carry the rule | the check pins the handler: over the generated set of phase 17 check 6, route 16 answers the masked 404 on exactly the rows on which phase 17 `GET /alerts/{id}` answers it; the text says the permits are by role only
