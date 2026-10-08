# AKR-TMS plan PDF: working outline

Status: draft (2026-10-06). Refined as research files land. Audience: bank leadership (CEO, board members, CRO) and
compliance heads (MLRO, head of compliance). English. Target 20-25 pages, 6-8 mockups. Plain language; no deep
technical detail (no code, no schemas, no stack jargon beyond one short "under the hood" box).

## Narrative in one line

Iraqi banks now have to prove their controls to foreign correspondents, not just have them; AKR-TMS is the first AML
and KYC system designed in Iraq, for Iraqi data and Iraqi rules, that produces that proof by default, without touching
the core banking system.

## Sections (page budget)

| # | Section | Pages | Content | Research |
|---|---|---|---|---|
| 0 | Cover | 1 | Name, one-line promise, hero mockup | identity |
| 1 | Executive summary | 1 | Why now, what it is, what is different, phases | all |
| 2 | The situation Iraqi banks are in | 2 | Grey list 19 June 2026; dollar bans 2023-2025; end of the dollar platform; CBI 2025 controls; correspondents as second supervisors; STR statistics; rules in place but execution weak (MER IO4) | 01, 02, 05 |
| 3 | What the rules require | 1.5 | CBI hard requirements (condensed); FATF and UN; foreign regimes converge on one specification | 01, 03, 04, 05 |
| 4 | What the market offers and where it falls short | 2 | Vendor groups; right/wrong matrix; the gaps for Iraq (Arabic, cost, batch-only, black box, lock-in, no Iraqi lists or CBI catalogue) | 07-18 |
| 5 | Lessons from enforcement | 1 | Seven failure patterns and the design answer to each | 20 |
| 6 | Design principles | 1 | Ten principles | 33 |
| 7 | The system, module by module | 7-8 | Command centre; customer risk and KYC; screening; monitoring and scenario studio; investigation and network view; reporting (goAML); data connection and quality; governance and evidence packs; Islamic banking | 21-31 |
| 8 | How it connects to the bank | 1.5 | Read-only replica; CBS-agnostic mapping; freezes as tasks with verification; phase-2 pre-release API; on-premises, security | 25, 30, 32 |
| 9 | AI, used responsibly | 1 | Three tracks (rules, models, GenAI drafts); explainability; human decides; validation pack | 21, 29, 05 |
| 10 | One product, many Iraqi banks | 1 | Conventional and Islamic; private and state; regime packs; Kurdistan; per-bank deployment | 18, 24, 05 |
| 11 | Roadmap | 1 | Phase 0 discovery and mapping; phase 1 core; phase 2 pre-release API and advanced analytics; phase 3 | all |
| 12 | What success looks like | 0.5 | KPIs | 29, 05 |
| A | Appendix | 1-2 | Glossary (English-Arabic); research index; requirement counts by source | 06, all |

## Mockups (English UI, Arabic-first product; 6-8)

1. **Command centre** (MLRO dashboard): coverage reconciled, list freshness, alert backlog ageing, STRs this month,
   regulatory deadlines, risk distribution, correspondent evidence status.
2. **Investigation workspace**: alert consolidated per customer, reason codes, transaction timeline, counterparties,
   AI-drafted summary marked "draft, requires review", four-eyes decision panel.
3. **Screening match review**: customer name in Arabic script and transliteration next to the list entry, per-name-part
   match explanation, list source and version, decision buttons, pair memory.
4. **Customer risk profile**: risk score with factor breakdown and reason codes, KYC review status and due date, EDD
   checklist, evidence vault documents, ownership tree.
5. **Scenario studio**: a CBI scenario with parameters, plain-language description generated from configuration,
   above- and below-the-line test results, change request awaiting approval.
6. **Network view**: graph of linked accounts (shared counterparties, family number, phones), mule ring highlighted.
7. **Regulatory reporting**: goAML STR builder with validation checks and narrative draft.
8. **Data connection health**: CBS replica feed status, reconciliation of counts and totals per table, data-quality
   issues, mapping coverage.

## Open questions to settle before writing

- Phase timeline lengths (indicative ranges only).
- Whether to include an identity page (logo and palette) in the PDF: yes, one page near the end or inside the cover
  spread.

## Mockup log

- 01-command-centre.png v1 (2026-10-06, Codex via ChatGPT, 1536x1024): layout, palette and text correct. Defects to fix
  in the final batch: backlog bars invented by the model total 2,350 while the KPI says 1,284 open alerts and "212 older
  than 30 days" (bars show 1,030); donut says 10,000 customers (too small for a bank). Fix: give every chart value
  explicitly. Consistent set: Cash 160/150/40/12 (362), Transfers 140/170/45/20 (375), Trade finance 40/50/18/9 (117),
  Screening 150/120/22/8 (300), Staff accounts 50/42/30/8 (130) = 1,284 total, 212 older than 30 days; customers
  486,200 = High 29,172 (6%), Medium 102,102 (21%), Low 354,926 (73%). The logo in the rail reads as a zigzag arrow;
  draw the real logo as SVG for the PDF and describe it more precisely in prompts.
- Use v1 as --ref for every other mockup so the set looks like one product.
