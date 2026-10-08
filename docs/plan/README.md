# Implementation Plan

Phase-file implementation planning for AKR-TMS, following the method in
[`.claude/rules/planning.md`](../../.claude/rules/planning.md) (ported from the digital-banking-system repository).
Requirements come from the system plan in [`docs/`](../README.md) and, behind it, the research set
([`research/`](../../research/), requirement IDs); these files translate them into copy-paste-ready implementation
specs.

| File | Purpose |
|-|-|
| [`roadmap.md`](roadmap.md) | Phase table (00-31), dependency graph, entities and services by phase, migration chain, gap log |
| [`research.md`](research.md) | Dated decisions log (DR-NN) and the implementation research queue |
| [`phase-template.md`](phase-template.md) | Template for new phase files |
| `phase-XX.md` | One spec per phase, written next, per the template |
| [`status.md`](status.md) | Living tracker: status, cumulative totals, jobs by phase, gap summary, blockers |
| `PHASES-X-Y-VERIFICATION.md` | Verification reports (created by verification passes) |

Workflow: write the phase files, then gap-analysis passes (coverage, then adversarial correctness on the decision,
deadline and CBS-connection paths), then a verification pass scoring 0 true gaps, then implement phase by phase,
updating `status.md` as phases move Planned, Spec ready, In Progress, Done.

Phases 00-08 are macro-phase P0 (the two early pieces, the goAML checker and the CBI coverage gap report, are phases
06 and 07 and come first in priority after the foundation they need). Phases 09-26 are P1, 27-31 are P2. P3 phases
are written when P2 nears its exit, as in the DBS.
