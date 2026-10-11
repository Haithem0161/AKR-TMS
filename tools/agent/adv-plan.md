# Adversarial correctness pass: plan (2026-10-10)

Source rule: .claude/rules/planning.md "Adversarial correctness pass". Coverage passes confirmed requirements are
present (gap pass 1, DR-356 to DR-406); this pass attacks whether the logic survives concurrent and adversarial inputs.

Method (user choice 2026-10-10, "Haiku agents, lead checks"): one Haiku agent at a time drafts candidate flaws into
tools/agent/adv-runN.md only. The lead verifies each candidate independently against the phase files (the lock, the
CHECK, the unique index, the If-Match version, the guard trigger that would stop it), writes
tools/agent/adv-runN-verdict.md, then applies confirmed fixes with tools/agent/apply-adv-runN.py: a locked decision in
docs/plan/research.md and a section 7 block in the phase file. Each run is about 250k tokens.

Runs (each names the planning.md targets it attacks and the phases it reads):

| Run | Targets | Phases |
|-|-|-|
| 1 | Four-eyes races on case decisions and report approval; a change request approved by its own maker; self-approval through a second role, a delegate or a re-submission | 01 (step-up), 03 (roles), 04 (workflows, change requests), 14 (freeze-list decisions), 15 (overrides), 18 (case decisions), 19 (RFI release), 20 (freezes), 21 (report approval) |
| 2 | Claim races on alerts; alert fingerprint collisions; duplicate alerts on replayed batches; late data amending alerts already decided | 11 (ingestion, replays), 13 (features), 14 (screening alerts), 16 (scenario runs, detection keys), 17 (alert intake, claims, queues) |
| 3 | Watermark gaps after a CBS restore or failover; late or reordered batches; source pauses | 05 (sources), 10 (canon), 11 (watermarks, freshness), 12 (reconciliation) |
| 4 | Freeze, report, review and RFI clocks firing on a closed, merged or reopened case; timer re-arm and cancel races | 00 (timers), 15 (review clocks), 18 (case close and reopen), 19 (RFI clocks, breaches), 20 (freeze clocks), 21 (report clocks), 23 (holds) |
| 5 | Existence masking leaking through counts, search, exports, SSE events, notifications and error codes | 03 (masking), 13, 17, 18 (search), 19, 21, 22 (notifications, branch view), 24 (dashboards, packs) |
| 6 | Replayed idempotency keys; audit-chain gaps; outbox replay and handler idempotency | 00 (idempotency, outbox, jobs), 02 (audit chain, evidence), and the outbox consumers of 15, 19, 22 |

Status:
- Run 1: prompt p-adv-run1.txt, output adv-run1.md. APPLIED 2026-10-10 (apply-adv-run1.py; verdict adv-run1-verdict.md): 6 confirmed, 4 rejected, 9 guarded; DR-407.
  Lessons for later prompts: Haiku read a CHECK as outcome-limited when it applied to every decision (ADV-1-05); designed exceptions (emergency tightening, the job as CTR maker) came back as doubts; state them in the prompt.
- Run 2: prompt p-adv-run2.txt (adds the run 1 lessons), output adv-run2.md. APPLIED 2026-10-10 (apply-adv-run2.py; verdict adv-run2-verdict.md): 12 confirmed in 9 fixes, 1 left open, 14 guarded; DR-408.
  Lessons: fewer designed exceptions reported (the run 1 lessons worked); Haiku over-rated severity (CRITICAL where the item stayed visible to an analyst); it missed that `canon.touched` is per transaction, not per batch, until told by its own two findings.
- Run 3: prompt p-adv-run3.txt (adds the run 2 lessons), output adv-run3.md. APPLIED 2026-10-10 (apply-adv-run3.py; verdict adv-run3-verdict.md): 9 confirmed, 2 rejected, 11 guarded; DR-409.
  Lessons: severities were right this time; two proposed fixes were wrong in kind (a recon window that cannot reach past the read window; recording ordinary master updates as conflicts), so the lead writes fixes from the rule, not from the draft.
- Run 4: prompt p-adv-run4.txt (adds the run 3 lessons), output adv-run4.md. APPLIED 2026-10-10 (apply-adv-run4.py; verdict adv-run4-verdict.md): 10 confirmed in 9 fixes, 1 guarded on recheck, 17 guarded; DR-410.
  Lessons: Haiku missed a section-level preamble rule (phase 19 section 3 head) that guards several steps; tell run 5 to read section heads and preambles, not only the step.
- Run 5 (existence masking leaks): prompt p-adv-run5.txt (adds the preamble lesson and a masking severity scale), output adv-run5.md. APPLIED 2026-10-10 (apply-adv-run5.py; verdict adv-run5-verdict.md): 5 confirmed (HIGH 5), 4 LOW, 2 rejected as designed, 17 guarded; DR-411.
  Lessons: Haiku called every leak CRITICAL, even where the caller had to know the id already or was a designed reader (the A2 register, aggregate pack counts); it missed that the authorize layer runs before the handler. Tell run 6: check which layer runs first, and grep the tipping-off table for named readers.
- Run 6 (idempotency, audit chain, outbox replay): prompt p-adv-run6.txt, output adv-run6.md. APPLIED 2026-10-10 (apply-adv-run6.py; verdict adv-run6-verdict.md): 6 confirmed in 5 fixes (HIGH 2, MEDIUM 4), 1 LOW, 1 rejected as designed, 9 guarded; DR-412.
- ADVERSARIAL PASS COMPLETE (runs 1 to 6, DR-407 to DR-412). Next: summarise to the user and ask before the verification pass (planning.md).

## Verification pass (after the adversarial pass)
- LAUNCHED 2026-10-10: 32 Haiku agents in parallel (user allowed up to 50), one per phase; prompt tools/agent/p-verify.txt; outputs tools/agent/verify/phase-NN.md. The lead checks every GAP line, then writes docs/plan/PHASES-00-31-VERIFICATION.md.
- APPLIED 2026-10-10: three rounds (auditor, refuter, fix drafter; outputs tools/agent/verify/), lead verdict tools/agent/verify-verdict.md, fixes tools/agent/verify/fixes/NN.txt applied once by tools/agent/apply-verify.py (218 blocks, 549 replacements), ten hand edits (fixes/MANUAL.txt), DR-413. 641 confirmed (39 HIGH, 289 MEDIUM, 313 LOW). Report docs/plan/PHASES-00-31-VERIFICATION.md (gaps_found, 94/384).
- RE-CHECK DONE 2026-10-10 (prompt tools/agent/p-recheck.txt, outputs tools/agent/verify/recheck-NN.md, lead verdict tools/agent/recheck-verdict.md): 153 raised, all confirmed, 8 lead finds; 161 gaps (7 HIGH, 43 MEDIUM, 111 LOW) fixed by 221 replacements (tools/agent/recheck-fixes.txt, applied by tools/agent/recheck-fix.py, safe to re-run; backup tools/agent/verify/pre-recheck-backup.tgz). Checks green.
- SECOND RE-CHECK DONE 2026-10-10 on the user's word (prompt tools/agent/p-recheck2.txt, inputs tools/agent/verify/applied2-NN.txt, outputs tools/agent/verify/recheck2-NN.md, lead verdict tools/agent/recheck2-verdict.md): 32 raised, 27 confirmed, 5 rejected, 2 lead finds; 29 gaps (2 HIGH, 7 MEDIUM, 20 LOW) fixed by 29 replacements (ids S- and L2-). The applier keeps a ledger now (tools/agent/recheck-fixes.done).
- THIRD RE-CHECK DONE 2026-10-10: 17 Haiku agents on the 29 new replacements only (tools/agent/mk-recheck3.py, tools/agent/p-recheck3.txt, tools/agent/verify/applied3-NN.txt, outputs tools/agent/verify/recheck3-NN.md, lead verdict tools/agent/recheck3-verdict.md): 7 raised, 3 confirmed, 4 rejected, 2 lead finds; 5 gaps (0 HIGH, 1 MEDIUM, 4 LOW) fixed by 6 replacements (ids T- and L3-, entries 251 to 256). Three checks green.
- FOURTH RE-CHECK DONE 2026-10-10: 5 Haiku agents (phase files 00, 01, 04, 18, 30) on the 6 new replacements only (tools/agent/mk-recheck4.py, tools/agent/p-recheck4.txt, tools/agent/verify/applied4-NN.txt, outputs tools/agent/verify/recheck4-NN.md, lead verdict tools/agent/recheck4-verdict.md): 0 raised; 1 lead find (L4-27-1 HIGH: phase 27 check 6 compared the visibility predicate with role-only Cedar permits), fixed by 1 replacement (entry 257). Three checks green.
- FIFTH RE-CHECK DONE 2026-10-10: 1 Haiku agent (phase 27) on the 1 new replacement (tools/agent/mk-recheck5.py, tools/agent/p-recheck5.txt, tools/agent/verify/applied5-27.txt, output tools/agent/verify/recheck5-27.md, lead verdict tools/agent/recheck5-verdict.md): 0 raised, no lead find, 0 gaps. CLEAN ROUND.
- VERIFICATION PASS CLOSED 2026-10-10: report status complete (641 + 161 + 29 + 5 + 1 = 837 gaps fixed, 49 HIGH; 257 entries in tools/agent/recheck-fixes.txt, all in the ledger). Caveat written in the report: the first pass was a sample of 384 items and the later rounds read only changed text; 5 gaps were found by the lead in text no round was reading. The nine scope decisions of the report wait for the user's word.
- OWNER DECISIONS 2026-10-10: the owner answered the nine scope decisions (seven overturn the plan, DR-414 to DR-420; two confirm it, DR-421). The changes are section 7+ blocks in ten phase files (block sources tools/agent/owner/).
- OWNER-DECISION CHECK CLOSED 2026-10-11, four rounds of Haiku agents with lead verdicts (tools/agent/owner-verdict.md; prompts tools/agent/p-owner.txt, p-owner2.txt, p-owner3.txt, p-owner4.txt; inputs written by tools/agent/mk-owner2.py; fixes applied by tools/agent/ofix.py scripts and logged in tools/agent/owner-fixes.txt, tags R1 to R3): round 1 = 96 raised, 95 confirmed, 7 lead finds, 208 replacements, DR-422; round 2 = 29 raised, 29 confirmed, 4 lead finds, 46 replacements, DR-423 (phase 13 section 7.18, phase 04 section 7.21); round 3 = 5 raised, 5 confirmed, 1 lead find, 4 replacements; round 4 = 0 raised, CLEAN ROUND. True gaps 101, 31, 6, 0 (12 HIGH). Three checks green. Report section "Owner decisions (2026-10-10)" in docs/plan/PHASES-00-31-VERIFICATION.md.
