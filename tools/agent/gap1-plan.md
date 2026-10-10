# Gap pass 1 plan (lead, 2026-10-10)

Owner's choice (2026-10-10): Haiku agents draft, one at a time; the lead checks every finding against the phase files
before anything changes. Agents write ONLY their findings file; they never edit docs/ or research/.

Method (.claude/rules/planning.md "Pass 1"): compare requirements and cross-phase obligations against the phase specs;
each gap gets severity (CRITICAL/HIGH/MEDIUM/LOW), category, target phase, and the exact text of a Section 7.x addition.
The lead appends accepted additions to the target phase's "## 7+. Gap Additions" (replacing "None yet"), adds DRs where
a decision is made, and logs the pass in roadmap.md (gap log) and status.md (Gap Analysis Summary).

Mechanical inputs (re-run after changes):
- tools/coverage.py -> tools/agent/gap1-coverage.txt (1,208 baseline; 552 IDs named by no phase, 249 of them market
  VA/VB/VC/VD/VO; 112 named only on "Not carried" lines).
- tools/routecheck.py -> tools/agent/gap1-routes.txt (15 API.md rows with no phase route row; 5 phase-only rows: health,
  metrics, dev routes).
- tools/agent/gap-items.md (a)-(aa): cross-phase items collected in the batch reviews.
- Each phase 09-31 section 4.5 "Existing entity ... updates (extensions of earlier phases)" and "gap pass 1" notes in
  research.md DRs and status.md rows: obligations on earlier phases.
- DATA-MODEL vs CREATE TABLE: every DATA-MODEL table is created by a phase; `alert.model_scores` (phase 27) is missing
  from DATA-MODEL.

Runs (one Haiku agent each, in order; findings file tools/agent/gap1-runN.md):
1. Backward amendments, target phases 00, 01, 02.
2. Backward amendments, target phases 03, 04.
3. Backward amendments, target phases 05-16 and the 15 unowned API rows. Split (lead, 2026-10-10): 3a = targets 05-10;
   3b = targets 11-16 plus the 15 unowned API rows. Gap item (a) (Cedar form) was done by the lead inline
   (tools/agent/fix-g1-cedar.py).
4. Backward amendments, target phases 17-31, and the system docs (API.md, DSL.md, DATA-MODEL.md, DESIGN-SYSTEM.md,
   MODULES.md).
5. Coverage: core IDs named nowhere, prefixes SP, TS, C, S, RP, E, K (about 96, plus the K owners of run 3b G1-15-05: K-12, K-19, K-20, K-23, K-25, K-31, K-32, K-34, K-36, K-38 to K-41).
6. Coverage: prefixes L, T (about 106).
7. Coverage: prefixes UX, W, F, B, U (about 100).
8. Coverage: market IDs VA, VB, VC, VD, VO (248): classify only (carried in substance by a phase, out of scope, or gap).
Then the lead: apply, log, re-run the scripts, and ask the owner before the adversarial pass.

Run log: run 1 applied (DR-356..359); run 2 applied (DR-360..364); gap item (a) fixed inline; run 3a applied (DR-365..374; carried: phase 22 items 9 and 14 wording, phase 29 payment release status); run 3b applied (DR-375..383; carried: phase 22 "family observation", phase 30 Open doubts 23, phase 19 Open doubts 9, phase 20 Open doubts 22, phase 24 Open doubts 11 and 30, phase 09 typology "not here" line); run 4 split: 4a (17-24) applied (DR-384..388; carried to 4b: see the end of gap1-run4a-verdict.md), 4b (25-31 + system docs) applied (DR-389..392); run 5 split: 5a (C, E, RP, S, TS: 59 IDs; prompt p-g1-run5a.txt) applied (DR-393, DR-394), 5b (SP without SP-30, and the K owners: 51 IDs; prompt p-g1-run5b.txt) applied (DR-395, DR-396; plus the phase 15 header sentence fix). Run 6 split: 6a (L: 56 IDs; prompt p-g1-run6a.txt) applied (DR-397); 6b (T: 50 IDs; prompt p-g1-run6b.txt) applied (DR-398, DR-399;
counterparty resolver in phases 13 and 18; SP-30 placed on the phase 25 header); run 7 split: 7a (UX 31 + W 23 = 54
IDs, plus gap item (ab) X2 global search; prompt p-g1-run7a.txt) applied (DR-400; 8 gaps accepted); 7b (F 24 + B 13 +
U 9 = 46 IDs; prompt p-g1-run7b.txt) applied (DR-401; profile.breach topic in phase 17; gap items (ac), (ad) logged); run 8 split: 8a (VA 56 + VO 13 = 69; prompt p-g1-run8a.txt) applied (DR-402; parallel run in phase 24); 8b (VB 53 +
VD 40 = 93; prompt p-g1-run8b.txt) applied (DR-403; no gap); 8c (VC 86; prompt p-g1-run8c.txt) applied (DR-404; one-hour re-screen measure in phase 14). Gap pass 1 coverage complete: 940 carried, 268 Not carried, 0 named nowhere.
