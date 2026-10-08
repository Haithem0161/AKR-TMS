# 13 — Vendors: investigation automation and challengers (Lucinity, Silent Eight, WorkFusion, Napier AI, Unit21, Flagright, Salv, Sardine, Hummingbird, GenAI/agentic AML startups of 2025-2026) and the comparison table for files 12 and 13

Status: done (2026-10-06)

Part 2 of 4 of the vendor research (AI-native and challenger AML vendors). Other file in this part: 12 (AI-native detection
vendors and published bank results). Requirement IDs use the prefix VB-xx and are numbered across files 12 and 13.
Incumbent suites are in files 08-11 (prefix VA); screening/data/KYC vendors in 14-16.

Evidence grades used (same as file 12): Independent / Customer-reported / Vendor-reported / Analyst-reported.

## Contents

1. Lucinity (Luci copilot)
2. Silent Eight (AI adjudication of screening and TM alerts)
3. WorkFusion (AI agents for L1 alert review)
4. Napier AI
5. Unit21
6. Flagright
7. Salv (Salv Bridge data sharing)
8. Sardine
9. Hummingbird (case management)
10. GenAI and agentic AML investigation startups of 2025-2026
11. Comparison table: all vendors in files 12 and 13
12. What it means for AKR-TMS
13. Derived requirements (VB-xx, this file)
14. Sources

(Each vendor section covers: ownership/funding/acquisitions; product; detection approach and AI claims; published results
and credibility; deployment (SaaS vs on-prem/private cloud); explainability and model governance; Arabic and MENA presence;
pricing signals; customer reviews and criticisms; regulatory reception; what it does right; what it does wrong; lessons for
AKR-TMS; AKR-TMS hooks.)

---

## 1. Lucinity (Luci AI agent, case management, "Human AI Operations")

### 1.1 Snapshot

| Item | Finding | Source and grade |
|------|---------|------------------|
| Ownership and funding | Private, Reykjavik (Lucinity Group hf owns Lucinity ehf, Lucinity UK Ltd and Lucinity Inc). Founder Gudmundur Kristjansson became Executive Chairman; **Daniel Palmason is CEO (appointed with the December 2025 operations launch)**; new COO from Microsoft Nordic. **Funding history not verified** from reachable sources. | lucinity.com home, press and legal footer |
| Product | Case Manager; **Luci AI Agent** (generative copilot launched **6 June 2023** at Money20/20 Amsterdam, built on Microsoft Azure OpenAI; now multi-LLM through Azure); Luci plug-in; Customer 360; Regulatory Reporting (SAR); Transaction Monitoring ("scenario-based and AI-based"); Configuration (workflow automations, AI skills); open APIs. | lucinity.com; Luci press release 6 June 2023 |
| Business model change | **December 2025: "Human AI Powered FinCrime Operations"**: a managed service where Lucinity "takes on triage and investigation work for financial institutions under SLAs", with the institution keeping governance and decisions. | Lucinity press, 10 April 2026 |
| **Oracle deal** | **10 April 2026: "Oracle has secured rights to Lucinity's AI-native investigation and case management technology, embedding it into its Financial Crime and Compliance Management (FCCM) platform."** Lucinity continues mainly with Nordic institutions through the managed service. (Cross-reference file 09 on Oracle FCCM.) | Lucinity press, 10 April 2026 (vendor) |
| Deployment | SaaS on the Lucinity cloud (Azure); sold through Microsoft Azure Marketplace (consumption commitments); ISO 27001 and SOC 2; "Secure Lockbox" keeps encrypted PII on isolated infrastructure. **No on-premises option stated** (the Oracle route may bring one through FCCM). | lucinity.com security and Microsoft partnership pages |
| Customers named | Finshark (Sweden, open banking), Visa Cross-Border Solutions (formerly Currencycloud), Arion Bank (trial 2023), Pleo, Jyske Bank (event). Nordic focus. | lucinity.com case studies |
| BIS Aurora | Lucinity staff are listed as contributors to BIS Innovation Hub Project Aurora (file 12 section 8.3): the technology partner on a published independent PoC. | BIS report acknowledgements |
| Arabic and MENA | None found. Partner "Creditinfo" supplies localised PEP screening (region not verified). | lucinity.com |
| Pricing, reviews | None public; review sites not reached. | - |
| Regulatory reception | None found; contributor to a BIS Innovation Hub project (Aurora). | BIS report |

### 1.2 Detection approach and AI claims

Lucinity's thesis, in its own words: "Since day one, Lucinity has focused on the investigation layer of financial crime. While detection systems became more advanced, investigations remained manual, slow, and costly."
Detection is secondary (scenario plus AI monitoring); the product value is the case: automated evidence gathering, "consistent reasoning", summaries, narrative drafts, and a unified customer view.
Luci "converts complex AI findings into clear insights", "conducts thorough checks and provides crucial risk insights", "writes detailed regulatory reports consistently tailored to regulatory and enterprise guidelines".

### 1.3 Published results and credibility

| Claim | Source | Grade | Caveat |
|-------|--------|-------|--------|
| Lucinity and Microsoft "cut case investigation times from **3 hours to 30 minutes**" with Luci on Azure OpenAI | lucinity.com home | Vendor-reported, partner-co-marketed | One customer, no sample size or definition of "investigation"; time to prepare a case is not time to decide it. |
| Visa Cross-Border Solutions "saves **5 hours of analyst time daily**" with Customer Intelligence | lucinity.com case study | Vendor-reported, named customer | Per team or per analyst unclear. |
| Finshark: cases "easy to follow", "investigation is easily traceable" | MLRO quote | Customer quote, no numbers | Qualitative. |

No independent result for Luci was found.

### 1.4 Explainability and governance

"Human AI must be clear, explainable, and safe to supervise" (Ethical AI Pledge). The managed-service model keeps approvals of escalations, thresholds, policies and SARs with the institution. No model card, validation pack or hallucination testing is published.
The LLM is a third-party service (Azure OpenAI), so output quality, data handling and model version changes sit outside the bank's control.

### 1.5 What it does right

1. **Puts the investigator's case at the centre**: one prepared case with evidence, narrative and audit trail; the closest model to what the CBI asks for (internal report, acknowledgment, written decision, file 01 section 2d).
2. **Validated by an incumbent**: Oracle licensing its investigation layer shows large vendors expect LLM agents to arrive through OEM deals, not in-house builds.
3. **Privacy design for PII** (separate encrypted store) and a clear governance statement (bank keeps decisions).
4. **Open APIs and partner ecosystem** for screening, fraud and data quality.

### 1.6 What it does wrong

1. **Cloud-only plus a third-party LLM**: incompatible with an on-premises, read-only-replica deployment and with data-localisation caution (file 01 section 8).
2. **Pivot to outsourced operations**: "we run the FinCrime operations" conflicts with the CBI model in which the MLRO and branch liaison officers carry responsibility (file 01 section 2d).
3. **Thin evidence**: two vendor-hosted time-saving claims, one funding history not disclosed, no independent test.
4. **Dependence on one hyperscaler and now on Oracle**: the Oracle licence may shift product focus and customers.
5. **Detection is not the strength**; no CBI catalogue, no goAML, no Arabic.

### 1.7 Lessons for AKR-TMS

- **Copy:** case-first design; every AI statement tied to evidence; a copilot panel available wherever the analyst works; "Customer 360" assembled from CBS tables; open APIs.
- **Avoid:** a hosted LLM dependency; outsourcing decisions; time-saving claims without a definition.
- **Beat it:** a local model, Arabic narrative, the CBI internal-report flow, goAML export, and evidence-linked statements the MLRO can verify in one click.

### 1.8 AKR-TMS hooks

- **Case-screen copilot with cited facts**: every sentence in a summary or narrative draft links to the source records (transaction IDs, KYC fields, prior cases); prompts and outputs are logged; draft status is explicit (VB-30).
- **Decision gates stay human**: escalation, STR submission, freeze or block request and customer-exit decisions always require a named bank user; no AI or outsourced function can complete them (VB-31).

---

## 2. Silent Eight (Iris 7: AI adjudication of screening and transaction-monitoring alerts)

### 2.1 Snapshot

| Item | Finding | Source and grade |
|------|---------|------------------|
| Ownership and funding | Private, Singapore (Silent Eight Pte. Ltd.). Funding from the company's own history page: **Series A USD 6.2m (Nov 2019; OTB Ventures, Wavemaker, SC Ventures, the venture arm of Standard Chartered)**; total USD 15m after an OTB-led round (Oct 2020); **Series B USD 40m (March 2022, led by TYH Ventures with HSBC Ventures and existing investors)**. Later rounds not found. Two global banks are investors and customers. | silenteight.com history page (vendor) |
| Milestones | 2018: Standard Chartered first to deploy (sanctions alert resolution). Jan 2021: multi-year HSBC partnership, expanded to transaction screening in 2023-24. **2023-24: Emirates NBD, Mashreq, First Abu Dhabi Bank and Abu Dhabi Islamic Bank**. 2025: Iris 7 with "AI Agents". | History page |
| Product | **Iris 7**: policy-bound AI decisioning where decisions "must be executed with accountability, not merely recommended", with human oversight; agents for customer screening, payment screening, **transaction monitoring decisions (TMD Agent)**, adverse media, expert CDD, trade surveillance, horizon scanning, risk data manager, case manager; Customer Screening Suite and Payment Screening Suite. It sits **on top of the bank's existing screening engine and TM system**, "operating as a governed decision layer within existing workflows, rather than replacing core detection systems". | silenteight.com Iris 7 pages, Oct 2026 |
| Deployment | "**on-premises, in private or public cloud environments, in hybrid architectures, or as a managed service**"; on-premises means "within the institution's own data centres or controlled environments" with hosting and security managed by the bank, Silent Eight providing updates and support. | Iris 7 architecture and deployment pages |
| Arabic and MENA | Gulf customers named above (including an Islamic bank). A K2 Integrity case study describes "continuous, event-driven customer screening at country scale across a high-risk jurisdiction" with "complex Arabic and non-Latin naming environments" (jurisdiction not named). | History page; case study |
| Pricing | None public. A Forrester Consulting TEI (commissioned) reports NPV and ROI for a composite customer (below). | - |
| Reviews | Not reached (G2, Gartner blocked or not searched). | - |
| Regulatory reception | None published; FAQ says systems are "designed with auditability, transparency, and documentation features" to satisfy regulators; used by Tier 1 banks. | silenteight.com FAQ |

### 2.2 Detection (adjudication) approach and AI claims

- **Adjudicator, not detector.** The engine takes the alert produced by a screening or TM system, assembles a unified, decision-specific context (customer data, transaction history, network relationships, sanctions and AML sources, external intelligence), applies the institution's policy,
  and either closes the alert with a narrative and audit trail or escalates with the context attached. TMD Agent claim: "**Resolve up to 80% of monitoring cases without manual review**".
- **Policy-bound**: "policy boundaries, QA sampling, drift monitoring, version control, and complete audit traceability are applied across all AI Agent activity"; decisions are "produced within institution-defined approval limits" with "a complete reasoning path".
- "Learning-driven reasoning": adapts as typologies and policies change "without requiring rule rewrites, schema rebuilds, or ontology reconfiguration" (no technical detail published).
- Extras: SAR template pre-population from the investigation; Horizon Scanning Agent that reads regulatory and geopolitical developments against the bank's policies.
- Model bias and drift (FAQ): diverse training data, **"pooling anonymised datasets across institutions"**, retraining with investigator feedback, human-in-the-loop review.

### 2.3 Published results and credibility

| Claim | Source | Grade | Caveat |
|-------|--------|-------|--------|
| "Up to 90% case resolution", "100% precision", "no decision-quality errors reported by clients to date" | silenteight.com home page (the two figures sit in animated counters; values and labels were recovered from the page markup) | Vendor-reported | "Up to"; "reported by clients" is not an audit; precision at 100% implies no escalations were wrong, which cannot be true of any model without recall data. |
| TMD Agent: "resolve up to 80% of monitoring cases without manual review" | TM agent page | Vendor-reported | Compare HKMA Bank B (7% auto-closed, file 12 section 8.4): regulator-sampled auto-close is far lower. |
| **Forrester Consulting TEI (June 2026, commissioned by Silent Eight)** for the Customer Screening Suite: **three-year ROI 184%, NPV USD 2.6m, payback 9 months, "effective match rate" about 4%** | Silent Eight case page (the study itself not read) | Commissioned analyst study of a composite organisation | TEI studies model a composite from a few interviewed customers; commissioned by the vendor; assumptions not reviewed here. |
| K2 Integrity case: "15% match rates across Arabic and non-Latin naming conventions" reduced so that "**less than 1 in 1,000 screenings require manual review**" ("sub 4%" and "76%" appear as headline figures; their labels are not readable on the page) | Case page | Vendor-hosted | Jurisdiction and bank not named; the zero-tolerance claim (no missed true matches) has no stated test. |
| "Over 70% solve rate" (name screening case) | Case page | Vendor-hosted | Anonymous. |

No independent evaluation was found. Both the first deployment bank (Standard Chartered) and a major user (HSBC) are also investors.

### 2.4 Explainability and governance

Strong on paper and in process: decision narrative, evidence, reasoning path, retained decision record, QA sampling, drift monitoring, version control. Weak point: the underlying reasoning technology is undisclosed (it "does not rely on rigid rules or predefined schemas"), so a bank's model-risk team cannot reproduce a decision without the vendor.
Whether LLMs are used, and where they run in an on-premises deployment, is not stated.

### 2.5 What it does right

1. **Targets the highest-volume cost directly**: alert adjudication after detection, in screening and TM, without replacing the detection stack: the lowest-risk way to adopt AI (matches HKMA banks C and D).
2. **Explicit accountability model**: "AI agent-led decisioning with human oversight and accountability", institution-defined approval limits, QA sampling, auto-close drift control.
3. **On-premises is offered**, and the Gulf customer list (Emirates NBD, Mashreq, FAB, ADIB) shows it works with Arabic names and Islamic banking context.
4. **Honest about the build-versus-buy and scope** (it does not claim to detect, only to decide).

### 2.6 What it does wrong

1. **"Executes decisions" is a regulatory red flag in Iraq**: the CBI model needs a named MLRO decision, internal acknowledgments and written decisions (file 01 section 2d); autonomous closure of alerts must be tightly bounded and sampled.
2. **Opaque reasoning engine and unpublished validation**; the 80%/90%/100% claims are far above regulator-sampled results.
3. **Investor-customer ties** (Standard Chartered, HSBC) complicate the independence of the main references.
4. **No detection layer, no case management of its own to speak of, no goAML, no CBI catalogue**: it depends on another vendor's alerts.
5. **Price and integration effort unknown**; likely out of reach for smaller Iraqi banks.

### 2.7 Lessons for AKR-TMS

- **Copy:** the "decision layer on top of detection" idea with policy-bound approval limits, QA sampling and drift monitoring; decision narratives with a retained record; sampling of auto-closed alerts with re-open monitoring.
- **Avoid:** headline claims of 80-100%; unexplained reasoning engines; any default of autonomous closure.
- **Beat it:** an adjudication assistant that is deterministic where possible (policy rules and evidence checks), uses a local model only for language tasks, shows every policy clause applied, and is validated by a published back-test; Arabic names handled by a measured matcher (file 22).

### 2.8 AKR-TMS hooks

- **Policy-bound adjudication assistant**: bank-defined policies (as structured checklists, not free text) drive a recommended disposition; each recommendation lists the policy clauses and evidence used; auto-close only under a bank-signed policy, a risk ceiling, daily sampling and a re-open monitor (VB-32).
- **Retained decision record** per alert: context snapshot, policy version, model version, reasoning path, reviewer, timestamps; exportable for CBI inspection (VB-33).

---

## 3. WorkFusion (AI agents for Level 1 alert review; now part of UiPath)

### 3.1 Snapshot

| Item | Finding | Source and grade |
|------|---------|------------------|
| Ownership | Headquartered in New York with offices in Europe and Asia. **Acquired by UiPath: announced about 6 Feb 2026; the deal closed in UiPath's fiscal 2027 first quarter; terms not disclosed.** Earlier investors listed by the company: Georgian, Greycroft, iNovia, Mohr Davidow, Hawk Equity, NGP Capital (round sizes not shown). CEO Adam Famularo. | The Next Web (Feb 2026); WorkFusion press and investors pages |
| Product | "AI Agents" with names and faces: **Evelyn** (name/PEP/sanctions screening alert review), **Tara** (transaction screening), **Evan** (adverse media), **Isaac** (AML transaction-monitoring investigator and fraud alert review), **Kayla** (KYC), Edward (EDD/high-risk reviews); agents "review and resolve" alerts "like an analyst would" at Level 1 and some Level 2 work. Built on the "Work.AI" platform; also customer-service email triage. | workfusion.com |
| Position | "**Not a screening tool**": integrates with the bank's existing screening and monitoring systems and disposes alerts. | Evelyn page |
| Claims | "Trusted by 10 of the top 20 banks"; "4 out of 5 top U.S. banks"; "four top-10 EU banks"; "70% less labor"; "start saving 50% of costs within 3 months"; savings "exceeded USD 100 million" for some enterprise customers; onboarding "typically within 4-6 weeks". | company page (vendor) |
| Deployment | **Not stated on the pages reviewed** (the platform came out of the RPA world; hosting options unverified). | - |
| Arabic and MENA | **Evelyn lists "English and Spanish language support"**; no Arabic. Customers are mostly North American and European; Standard Bank (South Africa) and Bank of Asia named. | Evelyn page; customer stories |
| Analyst standing | Ranked 5th in Everest Group's Top 50 financial crime and compliance technology providers (WorkFusion report page). | workfusion.com reports |
| Reviews, pricing | Not reached; no public pricing. | - |
| Regulatory reception | None found; the vendor says explanations are "regulator-ready" and "aligned with MRM". | workfusion.com |

### 3.2 Detection (adjudication) approach and AI claims

- **Alert adjudication by agents**: sanctions and name-screening hits (Evelyn "can resolve more than 80% of false positives"), transaction-screening hits, adverse-media articles, TM alerts (Isaac), KYC and EDD reviews.
- **TM framing**: "Banks manually review millions of alerts monthly, with 80-85% being false positives. Yet up to 10-15% escalate to formal investigations, each requiring hours of manual data gathering". Isaac "executes core investigative tasks including transaction and counterparty analysis, adverse media research, and geographic risk review", outputs "human-readable justification and a confidence threshold" and "a case file of each decision".
- **"We're not reducing false positives. Our AI Agents are eliminating them"**: every alert is reviewed at L1 in seconds so volume becomes irrelevant; only escalations reach analysts.
- **Learning**: agents "constantly learn from the WorkFusion Network" (cross-customer learning, mechanism unspecified) and in a human-in-the-loop with analysts who "dial in" a confidence threshold.
- "Explainable AI aligned with MRM"; narrative generation "beyond current template-based narratives" with "up to 95% STP" on document work.

### 3.3 Published results and credibility

| Claim | Source | Grade | Caveat |
|-------|--------|-------|--------|
| **Scotiabank** (adverse media): "95% reduction in false positives", "50% increase in media search coverage" (30 articles per name vs 20), "105 FTEs freed up", "cost savings of USD 4.2M within 6 months" | WorkFusion case study | Vendor-hosted, named bank | "False positive" here means irrelevant article hits; FTE and USD figures are modelled; no recall check on articles missed. |
| Carter Bank and Trust "saves USD 3M annually through AML automation"; Deutsche Bank "saves tens of thousands of hours annually"; Valley Bank, Raymond James | Customer-story titles | Vendor-hosted | Titles only were reviewed. |
| Standard Bank "automates 1 million transactions monthly", "60% cut in vehicle and asset finance verification time" | Case study | Vendor-hosted | Older RPA-era automation, not alert adjudication by LLM agents. |
| Evelyn: "more than 80% of false positives" resolved | Agent page | Vendor-reported | No definition of the false-positive baseline or the miss rate. |

No independent evaluation was found. Customer concentration is in North America; there is no evidence from a market with Arabic names.

### 3.4 Explainability and governance

Per-decision case file with human-readable justification and a confidence threshold; configurable thresholds; human-in-the-loop learning. Absent: published validation method, model card, drift metrics, bias testing. After the UiPath acquisition, roadmap and hosting decisions sit inside a larger automation vendor.

### 3.5 What it does right

1. **A clear unit of work: the alert**, with a named agent per alert type, which maps one-to-one onto the analyst's job description and makes ROI easy to measure (FTE freed).
2. **Confidence thresholds the bank controls** and a per-decision case file for examiners.
3. **Fast start** (pre-built agents, 4-6 weeks claimed) and a large reference base among US banks, which is why UiPath bought it.

### 3.6 What it does wrong

1. **No Arabic** (English and Spanish only), which rules it out for name-screening adjudication in Iraq.
2. **"Eliminating false positives" without recall evidence**: the quality of what the agent closes is not published; the bank bears the miss risk.
3. **Needs upstream alerts**: it adds nothing to detection, and an Iraqi bank without a good alert source has nothing to adjudicate.
4. **Ownership change** (UiPath, Feb 2026) creates roadmap and support uncertainty for customers outside its core markets.
5. **Hosting and data location undisclosed**; no on-premises statement.

### 3.7 Lessons for AKR-TMS

- **Copy:** the agent-per-alert-type framing and the per-decision case file with confidence and justification; a visible "capacity freed" metric for management.
- **Avoid:** promising to "eliminate" false positives; closing alerts without publishing the miss-rate method.
- **Beat it:** Arabic screening adjudication with a measured matcher; bank-controlled thresholds; a mandatory sampled-miss audit on auto-closed alerts.

### 3.8 AKR-TMS hooks

- **Alert-type playbooks**: for each alert type (screening hit, TM scenario, adverse media, EDD refresh) a structured checklist of evidence steps, with an assistant that pre-fills the evidence and proposes a disposition; per-type confidence thresholds set by the MLRO (VB-34).
- **Sampled-miss audit**: a daily random sample of auto-closed or low-priority alerts goes to a second reviewer; the miss rate is a visible KPI that can switch automation off (VB-35).

---

## 4. Napier AI (Napier Continuum: screening, transaction monitoring, perpetual client risk assessment)

### 4.1 Snapshot

| Item | Finding | Source and grade |
|------|---------|------------------|
| Ownership and funding | Private, London (founded 2015; CEO Greg Watson; Chief Data Scientist Dr Janet Bastiman). **3 Feb 2025: majority growth investment from Marlin Equity Partners** (US private-equity firm, over USD 9bn of commitments). Earlier: **GBP 45m backing from Crestline Investors (Feb 2024)**. Centres of excellence in Belfast and Kuala Lumpur; expansion into the Americas and APAC (April 2025). | Napier press releases |
| Customers | "Trusted by more than 150 financial institutions" (Oct 2026; the Feb 2025 release said "over 100", naming HSBC, State Street, Mizuho Trust and Banking, SS&C, Starling Bank, ClearBank, WTW). Others: St James's Place, Banco do Brasil, Australia Post, Freemarket, Salt Bank (Romania), noqodi and the Game LLC (UAE), **DIFC Authority** (client screening). | napier.ai |
| Product | **Napier AI Continuum**: Client Screening (batch and real-time, sanctions, PEP, adverse media), Transaction Screening, **Transaction Monitoring ("over 100+ inbuilt AML typologies")**, Perpetual Client Risk Assessment, case management, **sandbox** for testing rules on production data, no-code rules in "plain language", "Insights AI" (March 2026) giving natural-language explanations of behaviour, **responsible agents** built in a sandbox with a chosen level of autonomy (flag, or clear automatically) and a full audit trail. | napier.ai product pages, Oct 2026 |
| Deployment | "**Fully managed SaaS, in any private cloud, or on-premise**"; "deployable on any leading cloud or on-premises"; ISO 27001:2022 and SOC 2 Type 2; available in the Microsoft Marketplace (16 Sept 2026). | napier.ai client screening and Continuum pages |
| **Arabic and MENA** | AI fuzzy matching "across more than **20 languages, including simplified Chinese and Arabic**", with transliteration, phonetics and nicknames. **Dubai office at DIFC since 2021**; "provides anti-financial crime solutions to many organisations across the MEASA region, including central banks, monetary agencies and crypto exchanges" (unnamed). | napier.ai; DIFC release |
| Analyst standing | Celent Model Risk Manager awards linked to Napier deployments (Banco do Brasil, year not shown; Australia Post 2025); Celent AML Watchlist Screening Report 2024 "Technology Standout"; Chartis RiskTech100; Datos Insights honourable mention (10 Sept 2026) for the Agentic Test Simulator. | napier.ai newsroom (vendor-published) |
| Pricing, reviews | None public; review sites not reached. | - |
| Regulatory reception | Participant in the FCA Supercharged Sandbox (2025-26); says its typology library was developed "in collaboration with regulators"; DIFC Authority is a client. | Napier press releases |

### 4.2 Detection approach and AI claims

- **Governed rules plus AI**: "combines governed rules with AI to detect complex behaviour that rules alone can miss"; typology library "developed in collaboration with regulators"; rules authored without code; **every change previewed in the sandbox on real production behaviour** and promoted "only when the numbers hold", with audit trail and version history.
- **Three-layer false-positive reduction in screening**: multi-configuration screening (independent set-ups per customer type, risk or list), AI scoring and explaining the remaining alerts, then responsible agents that triage or clear known patterns.
- **FCA Supercharged Sandbox "Project Theseus"** (2025-26): novel frequency-based algorithms ("pattern mining" and "fluid dynamics": detecting "ripples" in transaction flow where money was injected) tested on **large-scale synthetic financial data** to detect typologies "more effectively than traditional rules-based systems, and with significantly reduced compute power".
- **Agentic Test Simulator for AML Effectiveness** (closed beta; Datos Insights 2026 honourable mention): each **synthetic customer is an autonomous agent**; complex typologies (mule networks, smurfing, circular payments) "emerge dynamically"; because the simulation is controlled end to end, "each illicit event is known and labelled", so an institution can **measure recall and typology coverage** of its existing rules and models and get a gap report. Napier says early tests covered up to 30 typologies and generated "hundreds of millions of transactions within hours", and states that the findings "are indicative of its potential rather than validated".

### 4.3 Published results and credibility

| Claim | Source | Grade | Caveat |
|-------|--------|-------|--------|
| "More than 90% false positive reduction", "65%+ improvement in case investigation time", "handles billions of transactions a year" | Continuum page | Vendor-reported | No customer, baseline or method. |
| "135% increase in unusual activity detection" (a corporate case, with a law-enforcement strike force); "77% faster transaction approval times" (Freemarket) | Product pages | Vendor-reported | Selected cases; metric definitions absent. |
| Insights AI, Project Theseus: detection "more effectively than traditional rules-based systems" | Press release 12 March 2026 | Vendor-reported; tested in the FCA sandbox on synthetic data | No numbers published; FCA tested the environment, it did not certify the result. |
| Agentic Test Simulator: up to 30 typologies, hundreds of millions of synthetic transactions in hours | Press release 10 Sept 2026 | Vendor-reported; self-labelled as indicative | Closed beta; synthetic realism unvalidated. |

No independent measurement of production detection was found. The synthetic-data approach is the most useful idea in the evidence, not the headline numbers.

### 4.4 Explainability and governance

Plain-language explanations per alert and per task, sandbox before go-live, full audit trail of every user and system action, automatic version history of rules, autonomy levels per agent with a human decision maker in charge. FCA sandbox participation signals regulator-facing design. No model card or validation pack is published.

### 4.5 What it does right

1. **Sandbox-first change control**: every rule or screening configuration is tested on production data before it goes live; this is the practice Wolfsberg and CBI-style audits want (file 04 section 2d on screening configuration governance).
2. **Deploy-anywhere**: managed SaaS, private cloud or on-premises, with Arabic matching among 20+ languages and a Gulf presence (DIFC), the closest fit to Iraq among the challengers in this file.
3. **Measures recall without real labels** through synthetic, agent-generated typologies, a direct answer to label scarcity (and an idea AKR-TMS should adopt).
4. **Regulator engagement** (FCA sandbox, typologies developed with regulators).
5. **Candid framing of its own results** in the simulator announcement ("indicative ... rather than validated").

### 4.6 What it does wrong

1. **Private-equity ownership** (Marlin, Feb 2025) typically means price discipline and an exit within a few years; roadmap and support priorities may shift.
2. **Headline claims without method** (90%, 65%, 135%) are the same marketing pattern as everyone else.
3. **Breadth over depth for Iraq**: no CBI scenario catalogue, no goAML, no Islamic-banking typologies found; Arabic matching claimed but unpublished in accuracy terms.
4. **Synthetic-data realism is unproven**: agent-generated typologies may not reflect Iraqi cash-heavy and hawala-like behaviour.
5. **UK-centred reference base**; only one named Gulf public body (DIFC) and UAE fintech/payment firms.

### 4.7 Lessons for AKR-TMS

- **Copy:** sandbox for every rule, threshold and screening configuration; synthetic labelled typology injection to measure recall and coverage; multi-configuration screening; agents with configurable autonomy levels in a sandbox.
- **Avoid:** headline percentages; relying on synthetic data without checking it against the bank's own history.
- **Beat it:** CBI-catalogue coverage matrix built in, Iraqi typologies in the synthetic generator (cash structuring across governorates, hawala-like flows, salary-localisation accounts, Islamic product patterns), published Arabic matching accuracy, price for Iraqi banks.

### 4.8 AKR-TMS hooks

- **Typology injection test harness**: a generator creates synthetic customers and labelled illicit patterns (structuring, mule networks, circular flows, rapid movement, third-party deposits, trade-based patterns) and injects them into a copy of the bank's data; reports recall per CBI scenario and per model; scheduled before every release and after every threshold change (VB-36).
- **Multi-configuration screening**: independent screening set-ups per customer type, risk rating and list, each tested in the sandbox with a recorded lookback (VB-37).
- **Regulator sandbox pack**: a CBI-facing test pack (synthetic data, typology coverage report, explanation samples) so the CBI and the bank can evaluate AKR-TMS without real customer data (VB-38).

---

## 5. Unit21 (agentic fraud and AML platform, US)

### 5.1 Snapshot

| Item | Finding | Source and grade |
|------|---------|------------------|
| Ownership and funding | Private, US; co-founder Trisha Kothari appears in the newsroom as a keynote speaker (CEO title and headquarters from general knowledge, not re-verified on the pages read). **Funding history not verified** from reachable sources (an earlier Series C is widely reported; not re-checked). | unit21.ai newsroom |
| Product | "AI Risk Infrastructure for fraud and AML": AML Transaction Monitoring (configurable rules, behavioural models, adaptive risk scoring), no-code rules, case management with alert-to-case-to-SAR workflow, graph analysis, payment and sanction screening, customer risk rating, regulatory filing, fraud (device intelligence, sub-250 ms decisions), **Fraud Consortium** ("shared intelligence network covering 80M+ adults in the U.S."), and **AI agents**: a **Detection Agent**, an **Investigation Agent** (summaries, narratives, linked entities), custom agents through an **Agentic Task Builder** ("bespoke investigation logic in plain English"). | unit21.ai home and newsroom, Oct 2026 |
| Customers | Intuit, Green Dot, Rippling, MoonPay, Uphold, Nexo, DriveWealth, Sallie Mae, Patriot Bank, Sunrise Banks, Cogent Bank, credit unions. US fintech, BaaS, crypto and community banks. | unit21.ai customer pages |
| Deployment | **SaaS** (no on-premises or private-cloud statement found). | unit21.ai |
| Arabic and MENA | None found. US-centred (BSA, FinCEN, Nacha, MiCA resources). | unit21.ai |
| Analyst standing | "Leader" in the Liminal Index for AML Transaction Monitoring 2026 (vendor-reported); a **Chartis-authored report, "The Agentic Maturity Spectrum in Financial Crime"** published on Unit21's site (sponsored). | unit21.ai |
| Scale claim | "Unit21's agents have completed **over 1.5 million alert reviews**" (blog, 28 May 2026). | Unit21 blog (vendor) |
| Pricing, reviews | None public; review sites not reached. | - |
| Regulatory reception | None found; says the AI is "designed with the requirements of the EU AI Act in mind" (vendor). | Unit21 blog |

### 5.2 Detection and AI approach: how Unit21 says it controls LLM errors

From the blog "4 ways we've engineered around the AI hallucination problem" (Chief Product Officer, 28 May 2026), the clearest public description of production controls for LLM agents in this research:

1. **Eval sets**: structured test suites of real cases with known-correct answers; the agent is iterated "until it hits 95% or above" before deployment. Ground truth comes from "seven to eight years of human analysts performing real investigations inside our platform", with a QA layer so that evals are "benchmarked against the best, not the average" analysts.
2. **Deterministic code generation**: the LLM writes SQL or Python; deterministic code retrieves exact values; the model then summarises those values. "The LLM decides what to look for. The code retrieves it precisely."
3. **Context engineering**: give each task only the data it needs; an investigation is split across **specialised parallel workers** (data gathering, behavioural analysis, sanctions) writing to a shared store, and a separate decision layer aggregates them over multiple passes.
4. **Temperature zero** for repeatability.
Plus: an **LLM-as-judge monitor** scoring every intermediate step and detecting drift; **human escalation** when confidence is low or the decision carries regulatory weight; a **multi-model architecture** (about 11-12 models routed per task, swapped only if they beat the eval sets); design "with the requirements of the EU AI Act in mind".
The vendor says the aim is to automate "roughly the 60 to 80 percent of alert volume that experienced analysts close in minutes" and escalate the rest.

**Chartis four-level agentic maturity spectrum** (as published on Unit21's site): (1) rule-based automation; (2) AI-assisted copilot, where "humans perform every step and make every decision" ("where most vendors claiming agentic AI actually are"); (3) agentic with human oversight ("production frontier in 2026"); (4) autonomous agentic ("emerging"). "Agent-washing" red flags: cannot explain context scoping per task; no evaluation datasets from real human decisions; the "agent" is a chatbot over a data lake; no configurable autonomy ladder; audit trails without reasoning chains; no deterministic code generation for data queries.

### 5.3 Published results and credibility

| Claim | Source | Grade | Caveat |
|-------|--------|-------|--------|
| "50% reduction in false positives, improvement in alert quality, and catching 2x the fraud" | unit21.ai home (fraud section) | Vendor-reported | Fraud, no customer or method. |
| Agents "above 95% accuracy on well-defined compliance tasks"; 1.5 million alert reviews completed | Blog | Vendor-reported | Accuracy is against Unit21's own eval sets; no external audit. |
| **Sunrise Banks**: 7,000-8,000 active alerts, team of 11; **migrated about 60 of 70+ rules**; saved about six hours every Monday; target to cut resolution "from 60-day ... down to 30 days" with agents | Customer story | Customer-quoted, named; the 30-day target is an aspiration | Operational, not detection, results. |
| Patriot Bank (BaaS): up and ingesting data "within days" of a 12-week plan; duplicate-alert reduction | Customer story | Customer-quoted | No numbers. |

No independent measurement of detection or agent accuracy was found.

### 5.4 What it does right

1. **Publishes the real engineering of trust**: eval sets, deterministic retrieval, context scoping, drift monitoring, human escalation. This is the best description of LLM controls found.
2. **One data model for fraud, AML, EDD and sanctions** (entity networks with device and behavioural signals), and one workspace, a pattern shared with Feedzai.
3. **Fast rule migration** (60 of 70 rules moved directly, the rest ingested as standard alerts) lowers switching cost; flexible no-code rules per BaaS program.
4. **Honest scope for automation**: aims at clear false positives, not 100% automation.

### 5.5 What it does wrong

1. **SaaS-only and US-centred**: BSA-oriented filings, no goAML, no Arabic, no on-premises route.
2. **Eval ground truth is its own analysts' history**, which embeds defensive filing and US conventions; a market without such history (Iraq) cannot copy the method directly.
3. **A consortium of "80M+ US adults"** has no equivalent in Iraq.
4. **Claims**: "agents ... 95%+" and "catching 2x the fraud" are unaudited.
5. **Dependence on external LLMs** (a multi-model estate) is the opposite of an on-premises constraint.

### 5.6 Lessons for AKR-TMS

- **Copy:** eval sets as a deployment gate; deterministic query generation against the read-only data; per-task context scoping; LLM-as-judge and drift monitoring; low-confidence human escalation; a "configurable autonomy ladder" (matches Hawk's decision-rights ladder).
- **Avoid:** SaaS or hosted-LLM dependence; unaudited accuracy claims.
- **Beat it:** build the eval sets from the bank's own MLRO-approved dispositions plus synthetic typologies (Napier idea), keep the model local, restrict the agent's data tools to a read-only, allow-listed analytic schema, and log every query for audit.

### 5.7 AKR-TMS hooks

- **Eval-set gate**: every AI task (summary, triage, narrative) has a versioned eval set built from MLRO-approved past cases; a task can only be enabled when it passes a bank-set accuracy threshold and is re-tested on every model or prompt change (VB-39).
- **Deterministic retrieval for agents**: the assistant never reads raw tables in its prompt; it writes parameterised read-only queries against a whitelisted analytic schema (the CBS replica remains read-only), a query validator runs them, and the answer is summarised from returned values; all queries are logged (VB-40).
- **Per-task context scoping and judge monitor**: small, task-specific context bundles; a second check scores every output and raises a drift alert; low confidence routes to a human (VB-41).

---

## 6. Flagright (AI operating system for financial-crime compliance; goAML-ready filing)

### 6.1 Snapshot

| Item | Finding | Source and grade |
|------|---------|------------------|
| Ownership and funding | Private. Co-founder and CEO Baran Ozkan, CTO and co-founder Madhu G. Nadig ("after 18 months evaluating transaction monitoring vendors"). Investors named on its about page include **Frontline** (a partner quoted on the "initial investment") and angels such as Rubin Ritter (Zalando co-founder) and Charlie Delingpole (ComplyAdvantage founder). **Round sizes and dates not stated; not verified.** | flagright.com about page |
| Product | Rule engine (nested no-code logic), **Rule Simulation** and **Shadow Rules**, **AI Rule Builder** (natural language), dynamic risk scoring, case management with a Workflow Builder, network "Ontology", Quality Assurance, screening (configurable matching), **regulatory filing**, and **AI Forensics (AIF)** agents for screening, monitoring, governance and QA ("**turns your SOPs into AI agents in under 20 minutes**"). | flagright.com, Oct 2026 |
| Regulatory filing | "Automate SAR filing to FinCEN and **70+ goAML jurisdictions**" (other pages say 30+ and 35+); 7-stage filing workflow (escalation, drafting, approval, filing, **regulator acknowledgment**, archive); SAR/STR/CTR/TTR; **goAML-ready workflows**; role-based approval (MLRO, supervisor, reviewer); no SAR can be filed without supervisor approval; final versions locked, side-by-side comparison of versions. | flagright.com regulatory-filing page |
| Deployment | **SaaS** with regional data residency (**US, EU, UK, APAC**), isolated ephemeral compute, 99.99% uptime SLA; SOC 2 Type II and ISO 27001:2022; GDPR. No on-premises or self-hosted option stated; **no Middle East region**. | flagright.com security and why-choose-us pages |
| Performance claims | "1.4B+ transactions processed monthly", "200 ms p99 API latency", "1200 RPS", "production-ready in 2 weeks". | flagright.com (vendor) |
| Customers | Payment processors, digital banks, neobanks, credit unions, crypto, remittance, gaming; named stories include SciOPay (7-day integration) and B4B Payments. Customers in "35+ countries". | flagright.com |
| Arabic and MENA | None stated. | - |
| Pricing, reviews | None public. A "reviews" page exists on the site (not independently checked). | - |
| Regulatory reception | None found. | - |

### 6.2 Detection and AI approach

- Rules first (no-code, simulation, shadow mode), plus risk scoring, network analysis and agents.
- **AI Forensics**: an uploaded SOP is converted into an investigation agent; for screening it investigates every hit across sanctions, PEP, adverse media and enforcement lists with "citable evidence"; for monitoring it investigates alerts "end-to-end using SOPs", including pattern recognition (structuring, smurfing, velocity) and multi-hop network analysis; QA and governance agents check analysts' work and policy changes.

### 6.3 Published results and credibility

| Claim | Source | Grade | Caveat |
|-------|--------|-------|--------|
| "Up to **93%** reduction in false positives"; "**93%** of screening hits cleared with evidence", "**99%** analyst-aligned decisions", "30 seconds per hit" | why-choose-us and AIF screening pages | Vendor-reported | "Analyst-aligned" = agrees with a human decision; no recall; baseline unspecified. |
| "Close **77%** of TM alerts automatically with high confidence"; "10x faster"; "4 minutes per alert with AIF (**94% analyst agreement**)" vs "38-minute manual workflows" | AIF monitoring page, why-choose-us | Vendor-reported | The 38-minute baseline is a generic figure; compare HKMA Bank B, 7% auto-closed (file 12 section 8.4). |
| AI-assisted SAR drafting "completed in 18 minutes"; "90 minutes saved per day on narrative creation" (customer quote) | filing page | Vendor / customer-quoted | Anecdotal. |
| Payment processor customer: "50% fewer resources for flagged transaction reviews" | home page | Vendor-reported | Anonymous. |

Claim set is consistent with other vendors but is not independently verified. The same product is credited with "30+", "35+" and "70+" goAML jurisdictions on different pages, so the number of filing schemas actually in production is unclear.

### 6.4 Explainability and governance

Explainability is framed as "native, explainable, and auditable" with one-click evidence exports "with full lineage", supervisor approval before filing, locked final reports, and QA agents. No model card, eval-set method or drift reporting is published (compare Unit21's detailed description).

### 6.5 What it does right

1. **goAML-ready filing workflow in the product** (SAR, STR, CTR, TTR with approval, acknowledgment and archive), the closest to Iraq's FIU channel among the challengers (Iraq uses goAML; file 01, file 23).
2. **SOP-to-agent**: the bank's own written procedure becomes the investigation logic, which is the right unit of customisation for a regulator-driven market.
3. **Simulation and shadow mode** for rules, **no-code** changes in minutes, and **fast integration** (two weeks claimed, API-first).
4. **Supervisor-approval and version control around filings**: a sound control design.

### 6.6 What it does wrong

1. **SaaS with no Middle East region and no on-premises option** (regional silos are US, EU, UK, APAC).
2. **Unclear coverage claims** (30+, 35+, 70+ goAML jurisdictions): goAML schemas differ by country (Iraq's FIU has its own required fields and manual).
3. **Claims without method**; "analyst-aligned" measures agreement, not correctness.
4. **No Arabic**, no CBI catalogue, no Islamic-finance content.
5. **Very young vendor** (founded 2021-22; round sizes undisclosed), thin evidence for large banks.

### 6.7 Lessons for AKR-TMS

- **Copy:** the 7-stage filing lifecycle with regulator acknowledgment logs and locked versions; SOP-to-playbook; shadow rules; side-by-side report versions.
- **Avoid:** counting jurisdictions as a selling point without per-country schema tests; "analyst-aligned" as the only quality measure.
- **Beat it:** a goAML implementation tested against the Iraqi FIU's actual schema and manual, in Arabic and English, with CBI internal-report flow (acknowledgment, written decision) linked to the filing.

### 6.8 AKR-TMS hooks

- **SOP-to-playbook compiler**: the bank loads its Arabic or English procedures; the system proposes a structured playbook (steps, evidence, decision rules); the MLRO edits and approves it; only approved playbooks drive assistant checklists (extends VB-34) (VB-42).
- **Filing lifecycle**: escalation, drafting, approval (MLRO and supervisor roles), submission, regulator acknowledgment log, archive; locked final version; side-by-side comparison of drafts; goAML schema tests for each report type against the Iraqi FIU specification (details in file 23) (VB-43).

---

## 7. Salv (Estonia; Salv Bridge intelligence sharing; screening, monitoring, risk scoring)

### 7.1 Snapshot

| Item | Finding | Source and grade |
|------|---------|------------------|
| Ownership and funding | Private, Tallinn; founded **2018** by former Skype and Wise AML staff (Taavi Tamkivi, CEO; Jeff McClelland; Sergei Rumjantsev). **Total funding EUR 12.1m by January 2024**, including an earlier EUR 1.5m raise to take Bridge beyond Estonia, a EUR 4m round for Bridge expansion, and an additional EUR 3.9m for UK expansion announced 18 Jan 2024 (which the release says brought capital raised that year to EUR 7.9m); investors include Passion Capital, G+D Ventures, ff Venture Capital. Offices in Estonia, UK, Lithuania, Latvia, Poland. | Salv news, 18 Jan and 6 Feb 2024 |
| Customers | "Trusted by **100+ financial institutions across Europe**" (Oct 2026); LHV, Swedbank (Estonia), SEB, Luminor, Coop Pank, Paysera, NEO Finance, Kroo Bank, Solidgate, Tuum/Skaleet/Crassula (core-banking vendors that embed Salv). In Jan 2024: 53 institutions in 12 countries. | salv.com |
| Product | **Salv Screening** (sanctions, PEP/RCA, adverse media), **Salv Monitoring** (real-time and post-event; no-code and AI rule generator planned Q2 2024; **Collaborative Scenario Library** launched 28 Nov 2024), **Salv Risk Scoring**, **Salv Bridge** (collaborative investigations and intelligence sharing between institutions). | salv.com; press |
| Deployment | **SaaS** ("modular SaaS platform"); ISO/IEC 27001:2022 and SOC 2 Type 2; regulated partner and licensed KYC data processor. No on-premises option stated. | salv.com |
| Regulatory licences | **Latvia: "Closed Shared KYC Utility" licence (6 Feb 2024)** from the State Data Inspectorate, the first regulator-awarded licence of its kind (standards set in 2019). | Salv news, 6 Feb 2024 |
| Analyst standing | Chartis category leader awards (2025, including watchlist monitoring and fraud platforms); first Chartis Intelligence Sharing Award; FinCrimeTech50 2025 and 2026. | salv.com news (vendor-published) |
| Arabic and MENA | None. | - |
| Pricing, reviews | None public. | - |
| Regulatory reception | The Estonian FSA, Data Protection Inspectorate and FIU supervised the pilot (Data Protection Inspectorate Director General quoted); the Latvian State Data Inspectorate issued the Closed Shared KYC Utility licence (6 Feb 2024). | Salv blog and news |

### 7.2 What Bridge is (and is not)

Bridge is **a secured request-and-response network between named investigators at member institutions**, not a shared data lake or a federated model. Each exchange follows a **use-case template** (fraud recall, AML clarification, sanctions clarification) that fixes which fields may be shared and why ("data minimisation is hard-wired ... limiting information exchange ... to only that which is necessary to identify the customer or transaction in question"), is logged, and is available only to credentialled members.

### 7.3 Published results and credibility

| Claim | Source | Grade | Caveat |
|-------|--------|-------|--------|
| **Estonia pilot, July 2021 to March 2022**: ten banks "representing 99% of the domestic market" ran **1,400+ collaborative investigations** under the supervision of the Estonian FSA, Data Protection Inspectorate and FIU; "estimated to have prevented **EUR 3m**" of customer money reaching criminal accounts; **one in four urgent requests answered in under five minutes**, **75% within an hour** (against 24-48 hours typical); one bank: "at least 70 such cases" stopped in three months | Salv blog (CEO), with a data-protection regulator quote | Vendor-reported; supervised by three authorities | The EUR 3m is an estimate; the FIU benefit (fewer back-and-forth requests on about 11,000 STRs a year) is asserted. |
| "Up to **80%** recovery of funds lost to fraud using Salv Bridge" vs a UK baseline "of 5%" | Salv release, 18 Jan 2024 | Vendor-reported | "Up to"; selected cases. |
| **RUSI/FFIS global survey (21 May 2026)**: 37 platforms on six continents; **Salv Bridge one of only two classified as sharing personal data across borders** (with NFCERT in the Nordics); "**60,000+ intelligence exchanges** across 100+ financial institutions in the EU and UK since 2021"; "around 50% of reported cases result in at least partial financial loss recovery"; one large Nordic bank went from **10% to 80%** recovery | Salv summary of the RUSI report (report itself not read) | **Independent survey**, figures as reported by the vendor about itself | Secondary reading; the survey also found the median time between a criminal's offboarding at one bank and onboarding at another is **9 days**, and that over 60% of platforms have limited international cooperation. |
| "Customers have reduced false positives and repetitive tasks by 80%" | salv.com | Vendor-reported | Generic. |

### 7.4 What it does right

1. **The only collaboration model with a measured, supervised national pilot** and a regulator-issued licence, which also shows how the legal route works: banking-secrecy law with a purpose-limited AML exception (Estonia's AML law section 16), GDPR legitimate-interest basis, regulator supervision, and (from July 2027) AMLR Article 75 in the EU.
2. **Speed**: minutes instead of days for urgent fraud and sanctions clarifications; value measured in recovered funds, not alert counts.
3. **Data minimisation by template**, with no PII pooled; cross-border operation (UK and the Baltics) proven with personal data.
4. **A core-banking-vendor channel** (Tuum, Skaleet, Crassula) that embeds screening and monitoring into banks' cores, similar to Jack Henry with Feedzai.
5. **Collaborative Scenario Library** of scenarios with high true-positive rates, a lighter and legally simpler form of collaboration than data sharing.

### 7.5 What it does wrong

1. **Needs a network and a legal basis**; neither exists in Iraq today (banking secrecy under Banking Law 94/2004 art. 49 yields to AML controls, but there is no personal-data law and no inter-bank sharing framework, file 01 sections 2d and 8).
2. **SaaS-only; European**: no on-premises route, no Arabic, no Middle East reference.
3. **Fraud and APP-scam focus** in much of the messaging (UK reimbursement rules), less AML depth than the incumbents; no goAML evidence.
4. **Small company** (EUR 12.1m raised by early 2024); evidence of results is largely self-published.
5. **Value of Bridge scales with participants**; a single bank gets only the monitoring product.

### 7.6 Lessons for AKR-TMS

- **Copy:** the request template with minimal fields, purpose and legal-basis tag; logged, credentialled exchanges; measuring value in time to respond and funds frozen or recovered; a scenario library with true-positive statistics.
- **Avoid:** building the business on a network that needs law that does not exist; hiding the legal basis.
- **Beat it:** an Iraqi-law-aware request layer (to the FIU, to the CBI, to correspondent banks per the Wolfsberg RFI guidance in file 04 section 2p, later to other Iraqi banks if the CBI creates a channel), on-premises, Arabic.

### 7.7 AKR-TMS hooks

- **Template-based request channel**: every outgoing or incoming request (FIU request, correspondent RFI, fraud recall, sanctions clarification) follows a template with minimal fields, purpose, legal-basis tag, expiry, four-eye release and full logging; the channel can later be pointed at a CBI-approved inter-bank relay (VB-44).
- **Scenario cards with performance statistics**: each scenario card (VB-16) records alerts, true-positive rate and typology coverage per bank; aggregate statistics may be shared across banks only if the CBI or the banks agree (VB-45).

---

## 8. Sardine (agentic fraud, AML and KYC platform with device intelligence; US)

### 8.1 Snapshot

| Item | Finding | Source and grade |
|------|---------|------------------|
| Ownership and funding | Private, US (headquarters not verified); founded 2020 by former risk and compliance leaders from Coinbase, Revolut, Uber and PayPal (CEO Soups Ranjan, co-founder Aditya Goel). **Series C USD 70m on 11 Feb 2025 led by Activant Capital, total raised USD 145m then; its about page now says "USD 170 million raised"**. In the year to Feb 2025: customers doubled, ARR +130%, "300+ enterprises across 70 countries", 2.2bn devices profiled. | Sardine blog, 11 Feb 2025; about page, Oct 2026 |
| Product | Device intelligence and behaviour biometrics ("True Piercing"), Global KYC/KYB, identity and document verification, fraud (payment, bank transactions, card issuing, merchant monitoring, refund and policy abuse), cyber (account takeover, bots), **AML**: Transaction Monitoring, Customer Risk Rating, Sanctions Screening, Case Management, Sponsor Banking; **Agentic AML Ops** and Agentic Fraud Ops: **OSINT Search, Data Analyst, Rule Assistant, Transaction Monitoring, Business Due Diligence, Doc KYC, PEP Screening, Graph Analyst, Sanctions Screening and SAR Generation agents**; Sardine Flow (workflow orchestration); **Sonar** consortium (anonymised cross-customer intelligence); Connections Graph. | sardine.ai, Oct 2026 |
| Scale | "441M consumers, 3.4M businesses, 6.58bn transactions, USD 1.8tn transaction volume", "6.5B+ devices profiled" (home page). | sardine.ai (vendor) |
| Deployment | **SaaS**; no on-premises option stated. | sardine.ai |
| Arabic and MENA | None found; rule assistant "describe rules in any language". | sardine.ai AML page |
| Analyst standing | "Named a **Leader in The Forrester Wave: Financial Crime Management Solutions, Q3 2026**" (vendor banner); Datos Insights Gold winner for FinCrime Convergence; "4.9 out of 5 on G2" (vendor banner; G2 not reachable). | sardine.ai home page |
| Pricing | None public. | - |
| Regulatory reception | None found. | - |

### 8.2 Detection and AI approach

Fraud-first signals (device, behaviour, consortium) feed rules, workflows and ML ("trained on billions of data points"); AML agents sit on top: triage KYC, sanctions, PEP and adverse-media alerts ("name mismatches, transliteration errors, flipped DOB formats, weak name matches"), complete EDD and business due diligence from open sources, trace networks, **write monitoring rules from a description "in any language"** and tune thresholds, summarise cases and **draft SAR narratives "aligned with your team's SOPs"**.

### 8.3 Published results and credibility

| Claim | Source | Grade | Caveat |
|-------|--------|-------|--------|
| "Agents that make your risk team **2-5X more efficient**"; "**90%** faster entity research"; "**75%** auto-resolution rate"; "reduce average time to resolution from days to minutes" | Agentic AI for AML page | Vendor-reported | No customer, method, or error rate; the same "75%" is compared with the 7% auto-close in the HKMA sample (file 12 section 8.4). |
| Raise (fintech): "98% reduction in manual reviews"; Prezzee: "approval rate increase up to 70%"; "alert to resolution in under 20 minutes" (a bot-driven account takeover case at a large Canadian bank) | Customer stories | Vendor-hosted, named in part | Fraud use cases, not AML. |
| "Leading US neobank takes down large money laundering network" | Customer story | Vendor-hosted, anonymous | No numbers shown. |

No independent evaluation was found; the Forrester Wave Q3 2026 placement is analyst-sourced but only a vendor banner was read.

### 8.4 What it does right

1. **Device and behavioural signals plus consortium data** give early mule and account-takeover detection that transaction-only systems lack; the Wolfsberg view lists device ID and IP as inputs to effective monitoring (file 04 section 2a).
2. **One agent per task** with a named role (OSINT, data analyst, graph analyst, rule assistant, SAR drafter) and an orchestration layer (Flow) that mixes data, rules, agents and human review.
3. **Rule creation from a plain-language description** and threshold tuning help non-engineers keep pace with typologies.
4. **Fast growth and a funded roadmap** (USD 145-170m raised).

### 8.5 What it does wrong

1. **SaaS-only, fraud-first, US-centred**: consortium (441M consumers) is mostly Western; no goAML, no Arabic, no Iraqi data sources.
2. **Auto-resolution at 75% claimed with no method**, far above regulator-sampled results; transliteration "errors" are handled by an LLM agent, not a published matching method.
3. **Device and behaviour signals need the vendor's SDK on the bank's digital channels**, impossible in a read-only-database deployment.
4. **Broad scope** (KYC, KYB, credit underwriting, cyber): depth in AML typologies is unproven.
5. **Price and data-sharing terms opaque.**

### 8.6 Lessons for AKR-TMS

- **Copy:** the agent roster and the Flow idea (data, rules, agents, human review in one configurable workflow); rule creation from natural language with generated tests; entity research with sources cited.
- **Avoid:** an SDK-dependent device layer for a read-only deployment; consortium dependence.
- **Beat it:** read-only integration, Arabic rule drafting and matching, goAML filing, and a published recall method for any auto-resolution rate.

### 8.7 AKR-TMS hooks

- **Natural-language rule drafting** in Arabic and English: the assistant proposes a rule in the product's rule DSL with an explanation, test data and back-test on history; a human approves; no rule goes live without the simulator (extends VB-07, VB-17) (VB-46).
- **Optional digital-channel signals**: where the bank's digital channel database is available read-only (login IP, device ID, SIM or phone change events), ingest them as features and link-graph edges; where not, mark device features as unavailable instead of faking them (VB-47).

---

## 9. Hummingbird (case management, warehouse-native monitoring, regulatory reporting; US)

### 9.1 Snapshot

| Item | Finding | Source and grade |
|------|---------|------------------|
| Ownership and funding | Private (US); co-founder and CEO Joe Robinson; "founded by technologists and former regulators". **Funding history not verified** from reachable sources. | hummingbird.co about page |
| Product | Case management and investigations with **360 customer profiles**, no-code **automations**, **Customer Screening**, **Transaction and Risk Monitoring** that runs "**directly on top of your cloud data warehouse**", **Regulatory Reporting** (patented report automation), a marketplace of apps and data sources, and an **AI Agent Orchestration Platform**: Research Agent, Review Agent, Narrative agent, SAR filing agent, AI Assistant; "our agents, your agents, and specialist third-party agents" in one workspace. | hummingbird.co, Oct 2026 |
| Monitoring architecture | Connects to the customer's warehouse (**Snowflake, Databricks, BigQuery, Starburst, Postgres, AWS Athena**); rules are **SQL queries** defining data views plus a no-code builder ("If you can express it in SQL, you can monitor it in Hummingbird"); "**we only read, never store, the data used for monitoring and only retain the data associated with your alerts**". The team "must have some proficiency with SQL". | Transaction and Risk Monitoring page |
| Regulatory reporting | E-filing directly with **FinCEN (US) and FINTRAC (Canada)**; prepared reports for FIUs in Ireland, Germany, Sweden, the Netherlands, Luxembourg, Bermuda, Ghana and Nigeria (goAML countries); "thousands of validation tests before filing, ensuring they're never rejected"; complete record from internal approval to receipt acknowledgment. | Regulatory Reporting page |
| Deployment | SaaS querying a customer-controlled cloud warehouse; no on-premises option stated. | hummingbird.co |
| Arabic and MENA | None found. | - |
| Customers | Banks, fintechs, crypto firms, sponsor banks; webinar speakers from Kraken and CIBC; Datos Insights report "Best Financial Crime RegTech Innovation". | hummingbird.co |
| Pricing, reviews | None public. | - |
| Regulatory reception | None found; the company says it was founded partly by former regulators and supports filing with FinCEN, FINTRAC and goAML FIUs. | hummingbird.co |

### 9.2 AI approach and governance

- **Autonomy dial**: "Start with AI recommendations that your team approves. As confidence in the agent's performance grows, expand to fully automated casework."
- **Oversight in the workflow**: "See the agent's full reasoning at each step, with the ability to accept, reject, or override before moving forward"; built-in QA/QC workflows; every action "explainable, documented, and audit-ready".
- **SAR path**: AI prepares the report; "once approved, let AI handle the rest from populating the form to electronic submission to the FIU".
- **Delivery modes**: inside Hummingbird or over an API (agents callable from other systems).

### 9.3 Published results and credibility

No quantified result of substance was found on the pages read ("cases at 10x speed", "file SARs 10x faster" are generic). Hummingbird publishes fewer numbers than other vendors, which is a credibility plus but gives nothing to benchmark.

### 9.4 What it does right

1. **Warehouse-native monitoring that reads in place and does not store the data** is the closest public architecture to AKR-TMS's read-only CBS replica approach; it avoids copying the bank's data into the vendor's platform.
2. **Rules as SQL over governed views** with a no-code builder on top: flexible, testable, version-controllable, and honest about needing SQL skills.
3. **Regulatory reporting treated as a core product**: validation before filing, approvals, acknowledgments, goAML countries.
4. **A clear autonomy dial and step-by-step reasoning visibility** with accept/reject/override.
5. **Open orchestration**: bring your own agents.

### 9.5 What it does wrong

1. **Requires a cloud data warehouse and SQL-literate staff**: Iraqi banks run operational databases, not Snowflake or Databricks, and have few SQL analysts in compliance.
2. **SaaS**: the monitoring service must reach the bank's warehouse from outside; for an Iraqi data centre that connectivity would not be accepted.
3. **goAML coverage lists no Middle East FIU**; Iraq's schema is unverified; no Arabic.
4. **Little published evidence** and no funding information to assess viability.
5. **No ML detection** is described: monitoring is rules in SQL, with AI mainly for the investigation layer.

### 9.6 Lessons for AKR-TMS

- **Copy:** governed SQL views as the substrate for rules, with a visual builder, tests and approvals; "read, never store" data handling (only alert-related data is retained); autonomy dial; agent step visibility; pre-filing validation.
- **Avoid:** requiring analysts to know SQL; requiring a cloud warehouse.
- **Beat it:** ship the canonical views and a library of CBI scenarios as ready-made SQL, a visual rule builder for compliance officers, and an on-premises runtime that never leaves the bank; Arabic-first.

### 9.7 AKR-TMS hooks

- **Canonical analytic views plus governed rules**: rules are versioned SQL (or visual) definitions over canonical views in a read-only schema; each rule has an owner, tests, a back-test, and an approval; the runtime stores only alert-related data (VB-48).
- **Pre-submission validator** for goAML XML: schema validation plus Iraqi FIU business-rule checks, run on every draft report; rejects are blocked before submission (extends VB-43) (VB-49).

---

## 10. GenAI and agentic AML investigation vendors of 2025-2026

### 10.1 What happened in the market (dated events found in this research)

| Date | Event | Source |
|------|-------|--------|
| 11 Feb 2025 | Sardine USD 70m Series C (Activant); agents for fraud and AML | Sardine blog |
| 6 Mar 2025 | Quantexa announces Q Assist (early access April 2025) and Cloud AML | GlobeNewswire (file 12 section 1) |
| Apr 2025 | NICE Actimize Xceed AI agents (file 08) | file 08 |
| Dec 2025 | Lucinity launches "Human AI Powered FinCrime Operations" (managed service) | Lucinity press |
| ~6 Feb 2026 | **UiPath acquires WorkFusion** (closed in UiPath fiscal 2027 Q1; terms undisclosed) | The Next Web |
| 10 Feb 2026 | **Greenlite AI rebrands as Bretton AI and raises USD 75m Series B** (Sapphire Ventures; Greylock, Thomson Reuters Ventures, Canvas, Y Combinator, TIAA Ventures) | FinSMEs; Bretton newsroom |
| 10 Apr 2026 | **Oracle secures rights to Lucinity's investigation and case-management technology** for FCCM | Lucinity press |
| 25 Mar 2026 | Hawk launches the AML Investigative Agent | Hawk press |
| 2026 | Silent Eight Iris 7; ThetaRay RAY; Unit21 Agentic Task Builder; Flagright AI Forensics; Hummingbird agent orchestration | vendor sites |
| June 2026 | Norm Ai (agentic law and compliance for institutions) raises USD 120m Series C at USD 1.2bn valuation (not AML-specific) | norm.ai |
| 10-24 Sept 2026 | Napier Agentic Test Simulator (Datos honourable mention, 10 Sept); Feedzai Farol (24 Sept) | vendor press |
| Q3 2026 | Sardine banner: "Leader in The Forrester Wave: Financial Crime Management Solutions, Q3 2026" | sardine.ai |
| Undated (2025) | **Parcha announces it retired its original compliance product line and became Grep AI**, a general "high-stakes knowledge work" agent platform | parcha.ai home |

Signals: (1) agent features are now table stakes at every vendor in files 12 and 13; (2) **several pure-play GenAI compliance startups are moving toward managed services or other markets** (Lucinity to operations and Oracle licensing, Bretton to "BPO transformation", Parcha to Grep AI); (3) incumbents buy or license rather than build (UiPath-WorkFusion, Oracle-Lucinity); (4) consolidation and PE ownership continue (Napier-Marlin, Visa-Featurespace).

### 10.2 Short profiles

| Vendor | What it is | Evidence and claims | Deployment, Arabic | Credibility notes |
|--------|------------|---------------------|--------------------|-------------------|
| **Bretton AI (fka Greenlite AI)**, San Francisco, CEO Will Lawrence | "AI-native operations for the financial back office": agent platform for KYC, AML, sanctions, ongoing monitoring; 30+ skill templates, 180+ data sources, natural-language agent builder, workbench; managed services; "Trust Infrastructure": every decision logged with reasoning, evidence and model version; **production traces become eval sets**; agent-as-a-judge QA. Series A USD 15m (2025) then **Series B USD 75m (Feb 2026)** | "Compliance review time reduced by **87%**" (FDIC-insured bank), "EDD queue completion time reduced by **70%**" (OCC bank), "USD 5.35M first-year savings" (global exchange) | US-regulated banks; SaaS/managed; no Arabic | Anonymous vendor-hosted results; positioned as a BPO replacement, i.e. it sells outcomes, not a product |
| **Arva AI** (HQ and funding not verified; cites FCA, OCC, FDIC and SEC frameworks) | AgentStudio and AgentCore (analyst feedback to controlled improvements; "every change is tested, versioned and validated"); agents for Screening, **Investigations (fiat and on-chain monitoring)**, Onboarding, EDD, Lending; proprietary "Arva Intel" entity-enrichment model | "Up to 50% increase in accuracy compared to human teams and in-house builds", "1M+ reviews processed every month", "+21% straight-through processing", "**92% alert resolution** across production deployment"; **SOC 2 Type II, ISO/IEC 27001 and ISO/IEC 42001** (AI management system) certified; "built for FCA, OCC, FDIC and SEC frameworks" | SaaS; no Arabic | Vendor-reported; funding not verified; ISO 42001 is a useful governance reference |
| **Parcha, now Grep AI** (YC; founders ran compliance and infrastructure at Brex) | Compliance and KYB agents for banks and fintechs, "persistent memory, narrow specialists, every claim traced to a source"; **product line retired**; Grep AI is a general agent harness (250+ skills, 100+ data integrations, "99.7% accuracy", "5x output per dollar", "agents reach data through an isolated proxy ... every call scoped, logged and revocable") | Vendor claims | Cloud | The pivot is evidence that selling GenAI agents to banks is hard; its engineering description (scoped proxy, narrow specialists, source tracing) is useful |
| **Quantifind** (US; Graphyte) | Pure-SaaS risk screening and adverse-media/OSINT platform with entity resolution, case-manager integrations; partners Oracle FCCM, Dow Jones, Matrix-IFS | "2x productivity", "10-100x fewer false positives", "entity resolution with 90% accuracy", "40% productivity gains"; Celent study "up to USD 177.9M annual savings" for a Tier 1 bank (vendor-commissioned) | SaaS only; no Arabic found | Older company (decade of R&D); overlaps with screening vendors (files 14-16) |
| **Norm Ai** (New York) | "Agentic law": compliance and legal AI agents for institutions (policy-to-agent), affiliated AI-native law firm; USD 120m Series C at USD 1.2bn (June 2026), backers include Blackstone, Citi, Vanguard, TIAA | "Trusted by institutions managing over USD 35T in combined assets" | Not an AML monitoring vendor | Relevant only as a sign that policy-to-agent is attracting very large capital |
| **FluxForce** | "Agentic OS for regulated industries", 28 prebuilt agents, "kill switch", configurable autonomy; sells itself as an "alternative to NICE Actimize and Quantexa" | "89% detection accuracy and 0.1% false positives" (fraud agent) | Not stated | **Low credibility**: marketing-only evidence, no named customers or independent data found |

(Sphinx AI, checked as a possible AML agent vendor, is a data-trust layer for AI and not an AML product.)

### 10.3 What the agent claims add up to

| Source | Auto-resolution or time claim | Grade |
|--------|-------------------------------|-------|
| Silent Eight TMD Agent | up to 80% of monitoring cases without manual review | vendor |
| WorkFusion Evelyn | more than 80% of false positives resolved | vendor |
| Flagright AIF | 77% of TM alerts auto-cleared; 93% of screening hits cleared | vendor |
| Sardine | 75% auto-resolution | vendor |
| Arva | 92% alert resolution across production | vendor |
| Hawk, Alviere back-test | every confirmed true positive escalated, 98% of analyst-identified false positives closed (replay of 3 months) | vendor, back-test method |
| Unit21 | aims at the "60 to 80 percent" of alerts experienced analysts close in minutes; 1.5 million alert reviews completed | vendor |
| **HKMA sample (Bank B, in production)** | **7% of TM alerts auto-closed** with sampling, dormant period and re-open monitoring; 1-3 hours less work per investigator per day | **regulator-compiled** |
| **HSBC (scoring, not agents)** | 60% fewer alerts with 2-4x more suspicious activity found | customer-reported, method disclosed |

Reading: vendor claims cluster at 75-98% because they count different things (screening noise, name mismatches, low-risk TM alerts under a risk appetite the bank has not necessarily chosen) and rarely report recall; the one regulator-compiled production figure is an order of magnitude lower. **Treat 7% to 30% auto-closure as a realistic first-year target for TM and screening alerts under sampling, and make the miss rate, not the closure rate, the headline KPI** (links to VB-22, VB-35).

### 10.4 Buyer-side checklist adopted from Chartis and Unit21 (use as AKR-TMS design tests)

An "agent" is credible if the vendor can show: context scoped per task; evaluation datasets from real human decisions; a configurable autonomy ladder; audit trails with reasoning chains; deterministic data retrieval (queries) instead of raw tables in the prompt; drift monitoring; a kill switch (Chartis "agent washing" red flags as published on Unit21's site).

### 10.5 AKR-TMS hooks

- **AI-system card and management system** per assistive function (purpose, data, model, eval results, known limits, human oversight, change log), aligned to ISO/IEC 42001 so a bank's auditors and correspondents can read it; every function is registered in an agent register with owner and autonomy tier (extends VB-09, VB-39) (VB-50).
- **Claims register inside the product**: the effectiveness report (VB-21) states for every automation feature the closure rate, sampled miss rate, re-open rate and the date and sample of the last audit (VB-51).

---

## 11. Comparison tables: all vendors in files 12 and 13

Evidence grade of the vendor's best public result: **A** independent or regulator-compiled; **B** customer-reported with method; **C** named customer, vendor-hosted; **D** vendor-only or anonymous. "Iraq fit" combines legal screen, deployment, Arabic and cost for a bank running on-premises on a read-only CBS replica.

### 11.1 Facts

| Vendor (file) | Type and origin | Ownership and latest funding event | Deployment, as publicly stated | Arabic and MENA | Public price signal | Best evidence (grade) | Iraq fit |
|---------------|-----------------|-----------------------------------|--------------------------------|-----------------|--------------------|-----------------------|----------|
| Quantexa (12) | Entity resolution and decision intelligence; UK | Series F USD 175m at USD 2.6bn, 5 Mar 2025; FY Mar-2025 revenue GBP 126m | Annual subscription; on-prem listed but needs Spark, Elasticsearch, Kubernetes; Cloud AML is SaaS on Azure | Arabic parsing "on the horizon" (Mar 2025); DIFC and Doha offices | UK public-sector list GBP 0.56-7.7m per suite per year | Vendor tiles (HSBC "60% fewer case volumes", investor-customer) (D) | Medium on features; low on cost and stack |
| ThetaRay (12) | Label-free anomaly detection, cross-border payments; **Israel** | USD 57m, Sept 2023; Screena acquired July 2024 | SaaS on Azure only | Mashreq (2022), Now Money; no Arabic evidence | None | Shift4 case: 86% fewer false positives (C) | **Blocked** (Israeli HQ vs Iraq's 2022 law; SaaS-only) |
| Hawk (12) | AI-native TM, screening, fraud; Germany | USD 56m Series C, April 2025 | SaaS or private cloud; on-prem for AI overlay | None found | "Premium" (aggregator) | Ebury actionable alerts 14% to 30%; Alviere agent back-test (C) | Low-medium |
| Feedzai (12) | Fraud-first RiskOps with AML; Portugal/US | About USD 75m at >USD 2bn, 2 Oct 2025; Demyst acquired Apr 2025 | Cloud-first (AWS); an on-premises version existed (BTG Pactual migrated off it) | Wio Bank (fraud); none for AML | None | NatWest-type results absent for AML; Novobanco no numbers (D) | Low |
| Featurespace (12) | Behavioural fraud with AML module; UK, **Visa-owned** | Acquired by Visa (date and price not verified) | "On premise or fully hosted cloud" | Akbank (Turkey); none | None | NatWest scams +135% value detected (fraud) (C) | Low-medium (fraud-first, US-owned) |
| Tookitaki (12) | FinCense and AFC Ecosystem typology sharing; Singapore | Series A USD 19.2m, Nov 2019; later rounds not verified | Not stated | Saudi content; no Arabic evidence | None | UOB: 50-70% fewer false positives, +5% true positives (C) | Low-medium |
| Consilient (12) | Federated-learning models; US | USD 3m seed, May 2022 | Models run locally; no deployment guide | None | None | No named customer; BIS Aurora tested the architecture on synthetic data (A for architecture) | Low (network-dependent) |
| Lucinity (13) | Investigation and case layer, Luci agent; Iceland | Oracle licensed its technology (Apr 2026); funding not verified | SaaS on Azure with Azure OpenAI | None | None | 3 hours to 30 minutes with Microsoft (C) | Low |
| Silent Eight (13) | AI alert adjudication; Singapore | USD 40m Series B, Mar 2022 (HSBC Ventures, SC Ventures earlier) | On-premises, private or public cloud, hybrid, managed | **Emirates NBD, Mashreq, FAB, ADIB**; Arabic names case | None (Forrester TEI: NPV USD 2.6m composite) | Commissioned Forrester TEI, ROI 184% (C) | **Medium** (needs upstream alerts; decision-layer risk) |
| WorkFusion (13) | L1 alert agents; New York, now **UiPath** | Acquired by UiPath, Feb 2026 | Not stated | **English and Spanish only** | None | Scotiabank adverse media: 95% fewer false positives, USD 4.2m (C) | Low |
| Napier AI (13) | Screening, TM, case, sandbox; UK | Marlin Equity majority investment, Feb 2025; GBP 45m Crestline, Feb 2024 | **Managed SaaS, private cloud or on-premises** | **Arabic among 20+ languages; DIFC office since 2021; DIFC Authority client** | None | 90% claims (D); synthetic simulator "indicative" | **Medium-high among vendors** |
| Unit21 (13) | Agentic fraud and AML; US | Funding not verified | SaaS | None | None | Sunrise Banks operational results (C) | Low |
| Flagright (13) | TM, goAML-ready filing, AI Forensics | Undisclosed rounds (Frontline and angels) | SaaS; US, EU, UK, APAC residency | None | None | Claims only (D) | Low-medium (goAML) |
| Salv (13) | Screening, monitoring, Bridge intelligence sharing; Estonia | EUR 12.1m total, Jan 2024 | SaaS | None | None | Estonia pilot supervised by FSA, DPI and FIU; RUSI survey (B/A, self-reported) | Low (needs network and law) |
| Sardine (13) | Device intelligence plus fraud and AML agents; US | USD 70m Series C, Feb 2025; USD 170m total | SaaS | None | None | Claims only (D) | Low |
| Hummingbird (13) | Case management, warehouse-native monitoring, filing; US | Funding not verified | SaaS querying the customer's cloud warehouse (Postgres supported) | None | None | Few claims (n/a) | Low-medium (architecture alignment) |
| Bretton AI, fka Greenlite (13) | Agent operations, managed services; US | USD 75m Series B, Feb 2026 | SaaS and managed | None | None | Anonymous 87% and 70% (D) | Low |
| Arva AI (13) | Banking agents; HQ not verified | Not verified | SaaS | None | None | "92% alert resolution" (D); ISO 42001 | Low |

### 11.2 Capability matrix

Y = described as a product capability; p = partial or via partner; - = not offered or not found; ? = not stated.

| Vendor | Transaction monitoring | Screening | Entity and network analytics | Case management | AI agents or copilot | Regulatory filing | Rule sandbox or simulator | Cross-bank sharing |
|--------|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| Quantexa | Y | - | Y | p | Y | p | ? | - |
| ThetaRay | Y | Y | p | p | Y | ? | Y | - |
| Hawk | Y | Y | p | Y | Y | Y | Y | - |
| Feedzai | Y | Y | Y | Y | Y | Y (8 countries, no goAML) | p | p (Risk Ledger) |
| Featurespace | p | - | p | Y | ? | ? | p | - |
| Tookitaki | Y | Y | p | Y | Y | ? | Y | p (typology cards) |
| Consilient | models only | - | - | - | - | - | - | Y (federated) |
| Lucinity | Y | p (partner) | p | Y | Y | Y | ? | - |
| Silent Eight | adjudication only | adjudication | p | p | Y | p (SAR pre-fill) | - | - |
| WorkFusion | adjudication only | adjudication | - | - | Y | - | - | p (network learning) |
| Napier AI | Y | Y | p | Y | p | p | Y | - |
| Unit21 | Y | Y | Y | Y | Y | Y | p | p (US consortium) |
| Flagright | Y | Y | Y | Y | Y | Y (goAML-ready) | Y | - |
| Salv | Y | Y | - | p | ? | ? | ? | Y (Bridge) |
| Sardine | Y | Y | Y | Y | Y | p | p | Y (Sonar) |
| Hummingbird | Y (SQL on warehouse) | Y | p | Y | Y | Y (FinCEN, FINTRAC, goAML list) | p | - |
| Bretton | - | agents | - | Y (workbench) | Y | ? | - | - |
| Arva | - | agents | - | ? | Y | ? | - | - |

Reading across both files: **no vendor combines on-premises delivery, Arabic matching, goAML filing tested for Iraq, the CBI scenario catalogue and a public price**. Napier AI and Silent Eight are the nearest on deployment and Arabic; Flagright and Hummingbird are the nearest on filing and architecture; Quantexa is nearest on entity and network analytics; Salv is nearest on lawful collaboration. That gap is the product opportunity.

---

## 12. What it means for AKR-TMS (file 13)

1. **Investigation automation is the visible battleground, and the evidence is weaker than the marketing.** Vendor auto-resolution claims run at 75-98%; the only regulator-compiled production figure is 7% auto-closed with sampling (HKMA, file 12 section 8.4); HSBC's scoring approach cut alerts by 60% while finding 2-4x more. AKR-TMS should plan for **7-30% auto-closure in the first year**, treat miss rate as the headline KPI, and make every automation switchable and sampled (VB-22, VB-35, VB-51).
2. **Copy the control design, not the claims.** The most useful engineering found: Unit21's eval sets, deterministic query generation, context scoping, judge monitoring and human escalation; Hawk and Oliver Wyman's decision-rights ladder; Silent Eight's policy-bound decisions with sampling and drift control; Flagright's SOP-to-agent; Chartis's "agent washing" red flags. Together they define an assistant that fits a **read-only data connection** (the model writes queries, never touches raw tables) (VB-39, VB-40, VB-41, VB-9, VB-32, VB-42).
3. **Keep decisions human by design in Iraq.** CBI rules require MLRO decisions, acknowledgments and written reasons (file 01 section 2d). Managed-service and "executes decisions" models (Lucinity, Bretton, Silent Eight wording) conflict with that. The product should make recommendations, pre-fill evidence and drafts, and gate every decision with a named bank user (VB-31).
4. **Deployment and language define who can compete in Iraq.** Only Silent Eight and Napier AI state on-premises delivery and show Arabic capability; WorkFusion supports English and Spanish only; the rest are SaaS or silent. Napier's sandbox-first change control and synthetic typology testing are the most transferable ideas; Hummingbird's "read in place, store only alerts" is the closest architecture match (VB-36, VB-37, VB-48).
5. **Reporting is a differentiator Iraq actually needs.** Flagright and Hummingbird treat the filing lifecycle (approval, acknowledgment, archive, validation) as core, and both list goAML countries; neither lists Iraq or any Middle East FIU. An AKR-TMS filing module tested against the Iraqi FIU's schema, with a pre-submission validator, is a concrete advantage (VB-43, VB-49).
6. **Collaboration needs law first.** Salv's Estonian pilot worked because of a statutory AML exception to banking secrecy, three supervising authorities and a licensing regime in Latvia; TMNL wound down pending the EU AMLR (file 12 section 8.7). Build a **template-based request channel** now (to the FIU, to correspondent banks, later to other Iraqi banks if the CBI creates a relay) and scenario cards with performance statistics; leave cross-bank data pooling to a regulator-led phase (VB-44, VB-45).
7. **Market signals favour a local, on-premises, transparent product.** GenAI-agent startups are pivoting (Parcha to Grep AI), outsourcing (Lucinity, Bretton), or being acquired (WorkFusion by UiPath, Lucinity's technology licensed by Oracle). A bank that buys a startup today buys an exit risk. Source escrow, exportable configuration and local support (file 08's VA-20 and VA-21) matter more with startups than with incumbents.
8. **Price.** None of the challengers publish prices. Quantexa's public-sector list shows that large platforms cost millions of pounds a year. AKR-TMS should publish bands by bank size, include the on-premises runtime and offline updates, and not require a managed service or hosted model (VB-52).
9. **Say how mature each AI feature is.** Classify each feature on Chartis's four levels (rule automation, copilot, agentic with human oversight, autonomous) and state the level in the product documentation, rather than calling everything an "agent" (VB-53).

### Limits of this research (file 13)

- Web-search budget exhausted before this file started; all facts come from direct page fetches. **Not reached:** G2, Capterra, Gartner Peer Insights (blocked), Business Wire (blocked), FinTech Futures (blocked), the Forrester TEI and Wave documents themselves, the RUSI/FFIS report itself, funding for Lucinity, Unit21, Hummingbird and Arva, deployment options for WorkFusion, and Silent Eight's pricing.
- Vendor sites change; pages were read on 2026-10-06.
- Some numbers on animated vendor pages (Silent Eight) were recovered from page metadata; labels for two figures on the K2 case page were not readable and are reported as such.

---

## 13. Derived requirements (VB-xx, file 13)

VB-01 to VB-29 are in file 12 section 10.

| ID | Requirement | Source |
|----|-------------|--------|
| VB-30 | Case-screen copilot whose every sentence links to source records; prompts and outputs logged; drafts clearly marked | Lucinity (section 1) |
| VB-31 | Decision gates stay human: escalation, STR submission, freeze or block requests and customer exit always need a named bank user; no AI or outsourced function can complete them | Lucinity (1); CBI MLRO rules (file 01 section 2d) |
| VB-32 | Policy-bound adjudication assistant: bank policies as structured checklists; recommendations list the clauses and evidence used; auto-close only under a bank-signed policy with risk ceiling, daily sampling and a re-open monitor | Silent Eight (2); HKMA (file 12 section 8.4) |
| VB-33 | Retained decision record per alert: context snapshot, policy and model versions, reasoning path, reviewer, timestamps; exportable for CBI inspection | Silent Eight (2) |
| VB-34 | Alert-type playbooks (screening hit, TM scenario, adverse media, EDD refresh) with evidence checklists and per-type confidence thresholds set by the MLRO | WorkFusion (3) |
| VB-35 | Sampled-miss audit: daily sample of auto-closed or low-priority alerts to a second reviewer; miss rate is a visible KPI that can switch automation off (extends VB-22) | WorkFusion (3); HKMA |
| VB-36 | Typology injection test harness: synthetic customers and labelled illicit patterns injected into a copy of bank data; recall per CBI scenario and per model before each release and after threshold changes | Napier (4); BIS Aurora (file 12 section 8.3) |
| VB-37 | Multi-configuration screening: independent set-ups per customer type, risk rating and list, each tested in the sandbox with a recorded lookback | Napier (4); Wolfsberg (file 04 section 2d) |
| VB-38 | Regulator sandbox pack for the CBI: synthetic data, scenario coverage report and explanation samples to evaluate the product without real customer data | Napier FCA sandbox (4) |
| VB-39 | Eval-set gate for every AI task, built from MLRO-approved past cases; enabled only above a bank-set accuracy threshold; re-tested on every model or prompt change | Unit21 (5) |
| VB-40 | Deterministic retrieval for assistants: parameterised read-only queries against a whitelisted analytic schema, validated and logged; answers summarised from returned values, never raw tables in the prompt | Unit21 (5); Hummingbird (9) |
| VB-41 | Per-task context scoping, an independent output check (judge) with drift alerts, and routing of low-confidence cases to humans | Unit21 (5); Bretton (10) |
| VB-42 | SOP-to-playbook compiler: Arabic or English procedures become a structured playbook that the MLRO edits and approves before it drives any checklist | Flagright (6) |
| VB-43 | Filing lifecycle: escalation, draft, MLRO and supervisor approval, submission, regulator acknowledgment log, archive; locked final versions; side-by-side draft comparison; schema tests for every report type against the Iraqi FIU specification | Flagright (6); Hummingbird (9); file 23 |
| VB-44 | Template-based request channel (FIU requests, correspondent RFIs, fraud recall, sanctions clarification) with minimal fields, purpose, legal-basis tag, expiry, four-eye release and logging; relay-ready for a CBI-approved inter-bank channel | Salv (7); Wolfsberg RFI guidance (file 04 section 2p) |
| VB-45 | Scenario cards carry per-bank performance statistics; cross-bank aggregation only if the CBI or the banks agree | Salv (7) |
| VB-46 | Natural-language rule drafting in Arabic and English producing rule definitions with tests and a back-test; human approval; never live without the simulator | Sardine (8) |
| VB-47 | Optional digital-channel signals (login IP, device ID, SIM or phone change) read from the digital channel database as features and graph edges; features marked unavailable otherwise | Sardine (8); Wolfsberg (file 04 section 2a) |
| VB-48 | Canonical analytic views and rules as governed, versioned SQL or visual definitions in a read-only schema; each rule has owner, tests, back-test and approval; the runtime stores only alert-related data | Hummingbird (9) |
| VB-49 | Pre-submission goAML validator: XML schema validation plus Iraqi FIU business rules on every draft; failures block submission | Hummingbird (9) |
| VB-50 | AI-system card and agent register aligned to ISO/IEC 42001 for every assistive function: purpose, data, model, eval results, limits, oversight, change log, owner, autonomy tier | Arva (10); Hawk/Oliver Wyman (file 12 section 3) |
| VB-51 | In-product claims register: for every automation feature, closure rate, sampled miss rate, re-open rate, date and sample of the last audit | Section 10.3 |
| VB-52 | Packaging: transparent licence bands by bank size, including the on-premises runtime, offline updates and local support; no mandatory managed service or hosted model | Sections 11, 12 (price signals, Quantexa list, Lucinity/Bretton pivots) |
| VB-53 | Each AI feature is classified on the Chartis four-level maturity spectrum and the level is stated in product documentation | Unit21/Chartis (5, 10.4) |

---

## 14. Sources

All URLs read on 2026-10-06 unless marked (search summary) or (blocked).

**Lucinity**
- [lucinity.com home](https://lucinity.com/), [press](https://lucinity.com/press), [case studies](https://lucinity.com/case-studies), [security](https://lucinity.com/security), [Microsoft partnership](https://lucinity.com/partnerships-microsoft)
- [Luci launch, 6 June 2023](https://lucinity.com/luci-press-release)
- [Oracle brings Lucinity's AI agent to its financial crime platform, 10 April 2026](https://lucinity.com/blog/oracle-brings-lucinitys-ai-agent-driven-capabilities-to-its-financial-crime-platform-as-lucinity-expands-into-human-ai-operations)
- [BIS Project Aurora report (Lucinity listed in acknowledgements)](https://www.bis.org/publications/project-aurora-power-data-technology-and-collaboration-combat-money-laundering-across-institutions-and-borders.pdf)

**Silent Eight**
- [silenteight.com home](https://www.silenteight.com/), [history](https://www.silenteight.com/history), [Iris 7 deployment](https://www.silenteight.com/iris-7-deployment), [Iris 7 architecture](https://www.silenteight.com/iris-7-architecture), [TM agent](https://www.silenteight.com/ai-agents-by-capability/transaction-monitoring), [FAQ](https://www.silenteight.com/faq)
- Case studies: [Forrester TEI](https://www.silenteight.com/case-studies/the-total-economic-impact), [K2 Integrity](https://www.silenteight.com/case-studies/k2-integrity-achieves-continuous-event-driven-customer-screening-at-country-scale), [70% solve rate](https://www.silenteight.com/case-studies/solving-screening-complexity-over-70-solve-rate-with-ai)

**WorkFusion**
- [workfusion.com home](https://www.workfusion.com/), [press](https://www.workfusion.com/press/), [investors](https://www.workfusion.com/investors/), [company](https://www.workfusion.com/company/)
- [Evelyn](https://www.workfusion.com/ai-agents/evelyn/), [Isaac](https://www.workfusion.com/ai-agents/isaac-ai-transaction-monitoring-investigator/); [Scotiabank](https://www.workfusion.com/customer-stories/scotiabank/), [Standard Bank](https://www.workfusion.com/customer-stories/standard-bank/)
- [The Next Web: UiPath acquires WorkFusion](https://thenextweb.com/news/uipath-pushes-deeper-into-financial-services-with-workfusion-acquisition)

**Napier AI**
- [napier.ai home](https://www.napier.ai/), [Client Screening](https://www.napier.ai/client-screening), [Transaction Monitoring](https://www.napier.ai/transaction-monitoring), [Continuum](https://www.napier.ai/continuum), [newsroom](https://www.napier.ai/newsroom)
- [Marlin investment, 3 Feb 2025](https://www.napier.ai/post/napier-investment-marlin); [FCA Supercharged Sandbox, 12 March 2026](https://www.napier.ai/post/napier-ai-tested-in-fca-supercharged-sandbox); [Agentic Test Simulator, 10 Sept 2026](https://www.napier.ai/post/napier-ai-datos-insights-agentic-aml-testing)
- [DIFC client screening](https://www.napier.ai/post/difc-client-screening-partnership); [Dubai office](https://www.napier.ai/post/aml-middle-east)

**Unit21**
- [unit21.ai home](https://www.unit21.ai/), [newsroom](https://www.unit21.ai/company/newsroom), [Chartis agentic maturity spectrum](https://www.unit21.ai/agentic-ai-maturity)
- [Blog: 4 ways we've engineered around the AI hallucination problem, 28 May 2026](https://www.unit21.ai/blog/4-ways-weve-engineered-around-the-ai-hallucination-problem-in-financial-crime-compliance)
- [Sunrise Banks](https://www.unit21.ai/customers/sunrise-banks), [Patriot Bank](https://www.unit21.ai/customers/patriot-bank)

**Flagright**
- [flagright.com home](https://www.flagright.com/), [regulatory filing](https://www.flagright.com/regulatory-filing), [security](https://www.flagright.com/security-compliance), [why Flagright](https://www.flagright.com/why-choose-us), [about](https://www.flagright.com/about-us), [AIF screening](https://www.flagright.com/ai-forensics-for-screening), [AIF monitoring](https://www.flagright.com/ai-forensics-for-monitoring)

**Salv**
- [salv.com home](https://salv.com/); [Bridge Estonia pilot outcomes](https://salv.com/blog/salv-bridge-estonia-pilot-outcomes/); [five legal myths of intelligence sharing](https://salv.com/blog/five-legal-myths-fincrime-intelligence-sharing/)
- News: [funding, 18 Jan 2024](https://salv.com/news/salv-secures-funding-to-combat-app-fraud/); [RUSI/FFIS survey, 21 May 2026](https://salv.com/news/salv-bridge-cross-border-intelligence-sharing/); [Collaborative Scenario Library, 28 Nov 2024](https://salv.com/news/collaborative-scenario-library-transaction-monitoring/); [Latvia Closed Shared KYC Utility licence, 6 Feb 2024](https://salv.com/news/salv-obtains-first-closed-shared-kyc-utility-licence/); [news index](https://salv.com/news/)

**Sardine**
- [sardine.ai home](https://www.sardine.ai/), [Agentic AI for AML](https://www.sardine.ai/agentic-ai-for-aml), [Series C, 11 Feb 2025](https://www.sardine.ai/blog/series-c-announcement), [about](https://www.sardine.ai/about), [customers](https://www.sardine.ai/customers)

**Hummingbird**
- [hummingbird.co home](https://www.hummingbird.co/), [Transaction and Risk Monitoring](https://www.hummingbird.co/product/transaction-risk-monitoring), [Regulatory Reporting](https://www.hummingbird.co/product/regulatory-reporting), [AI platform](https://www.hummingbird.co/platform/ai), [about](https://www.hummingbird.co/about)

**GenAI and agentic startups**
- [Bretton AI (fka Greenlite) site](https://www.greenlite.ai/) and [newsroom](https://www.greenlite.ai/newsroom); [FinSMEs, Series B, Feb 2026](https://www.finsmes.com/2026/02/bretton-ai-raises-75m-in-series-b-funding.html)
- [Parcha (now Grep AI)](https://www.parcha.ai/); [Arva AI](https://www.arva.ai/); [Norm Ai](https://www.norm.ai/); [FluxForce](https://www.fluxforce.ai/); [Quantifind](https://www.quantifind.com/); [Sphinx AI](https://www.sphinx.ai/)

Cross-references to published-results sources (HKMA, BIS Aurora, HSBC/Google Cloud, TMNL, FATF 2021) are listed in file 12 section 11.
