# 11 — Vendors: SymphonyAI NetReveal, FICO TONBELLER Siron AML, Nasdaq Verafin, Fiserv AML Risk Manager, plus the incumbent comparison table

Status: done (2026-10-06)

Part 1 of 4 of the vendor research (incumbent enterprise AML suites). Other files: 08 NICE Actimize, 09 Oracle FCCM, 10 SAS AML.
Requirement IDs use the prefix VA-xx and are numbered across all four files.

## Contents

1. SymphonyAI NetReveal (ex-Norkom, BAE Systems Detica)
2. FICO TONBELLER Siron AML
3. Nasdaq Verafin (consortium analytics)
4. Fiserv AML Risk Manager
5. Comparison table of all incumbents (NICE Actimize, Oracle FCCM, SAS, NetReveal, Siron, Verafin, Fiserv)
6. What it means for AKR-TMS
7. Derived requirements (VA-xx)
8. Sources

(Each vendor section covers: ownership/history, modules, architecture/deployment, detection approach, AI/GenAI, case
management and reporting, data model and CBS integration, Arabic and MENA/Iraq presence, implementation/cost, analyst
rankings, what it does right, what it does wrong, lessons for AKR-TMS, AKR-TMS hooks.)

---

## 1. SymphonyAI NetReveal (ex-Norkom, ex-BAE Systems Detica; now "Symphony Risk Intelligence")

### 1.1 Ownership and history

| Date | Event | Source |
|---|---|---|
| 1990s | Norkom Technologies (Irish financial-crime software maker) | Wikipedia (BAE Systems Applied Intelligence) |
| 2011 | BAE Systems acquires Norkom; the product becomes **Detica NetReveal**, later BAE Systems NetReveal | Wikipedia |
| 12 Jul 2022 / 31 Oct 2022 | BAE announces and completes the sale of its financial-services business, "which includes its flagship NetReveal financial crime detection solution", to **SymphonyAI** | Wikipedia citing BAE/SymphonyAI releases |
| Dec 2024 | Bank of Bahrain and Kuwait (BBK) selects SymphonyAI AML transaction monitoring and CDD (SaaS) | SymphonyAI press release 3 Dec 2024 |
| 19 Dec 2025 | Metro Bank case study: migration to SymphonyAI v9 SaaS (cut-over in two tranches of about 4 hours and under 24 hours) | SymphonyAI case study |
| 27 May 2026 | Chartis names SymphonyAI Category Leader in Name and Transaction Screening and Adverse Media (70+ vendors assessed) | SymphonyAI release |
| (Sep) 2026 | Forrester Wave: Financial Crime Management Solutions, Q3 2026: Leader, highest Strategy score and highest combined Strategy and Offering score among 15 vendors | SymphonyAI page and release |
| 24 Sep 2026 | Launch of **Symphony Risk Intelligence (SRI)**, "agent-native" platform; "SymphonyAI's transaction monitoring is the next generation of NetReveal" | SymphonyAI release |
| May 2026 | Reported as one of five bidders for NICE Actimize (CTech, unnamed sources) | A-Team 15 May 2026 (file 08) |

**Owner.** SymphonyAI is a SAIGroup company ("backed by a $1 billion commitment" from Romesh Wadhwani; Palo Alto; "3,000
talented leaders, data scientists and other professionals" in Dec 2024; "2,000+ customers in 40 countries across retail,
financial services, industrial and IT"). It is a private multi-vertical AI company; financial services is one of
five verticals, led by the president of SymphonyAI Financial Services. The NetReveal line has had three owners in 15
years (Norkom, BAE, SymphonyAI) and four names (Norkom, Detica NetReveal, BAE NetReveal, Sensa-NetReveal/SRI).

### 1.2 Modules

NetReveal Transaction Monitoring (AML) and "AI Overlay for Transaction Monitoring" (adds ML and agents on top of a bank's
existing engine), NetReveal Customer Due Diligence (KYC/CDD) and customer risk assessment, NetReveal Name Screening and
Transaction (payment) Screening, Payment Fraud, SRI Investigation (case management with copilot), **SRI Agents**
(Summary, Narrative, Web Research, Watchlist Enrichment, Name Match Analysis, Transaction Match Analysis), the SRI
platform with a **FinCrime ontology**, a **Model Store** and MLOps, and "Always-on Compliance" (continuous KYC and control
updates).

### 1.3 Architecture and deployment

SymphonyAI's 2026 messaging is **cloud-native, SaaS, "evergreen"** (BBK and Metro Bank are SaaS), with "hybrid-cloud
architecture" for CIOs and a "modular, data-agnostic" design that integrates with "legacy and modern systems".
Chartis (2026): "Cloud-based deployments are now standard". **No public statement of an on-premises deployment option
was found** in the SymphonyAI pages read; older NetReveal installations were on-premises. For an Iraqi bank that must run
inside its own data centre this is the key question to put to SymphonyAI. Screening claims: "350+ global watchlists across
60+ languages", payments "in as little as 40 milliseconds", a certified SWIFT Alliance Access plug-in; fraud
scoring under 50 ms.

### 1.4 Detection approach

- **Library:** "2,500+ detection scenarios available out-of-the-box ... spanning retail banking, capital markets,
  insurance and correspondent banking" (SRI page); the largest scenario count of any vendor in this research (NICE:
  250+ rules). A count says nothing about quality or relevance; it matters that scenarios are mapped to typologies.
- **Rules plus ML:** "scenarios combined with supervised and unsupervised ML models reinforce each other on every
  transaction"; pre-trained models from a Model Store; customers can "build, bring, or pick" models; MLOps.
- **Entity-level alerts:** "alerts are now rolled up to the customer level" (Metro Bank: 20% fewer alerts after go-live).
- **Triage agent:** assesses every alert, prioritises, and "auto-hibernat[es] known false positives without an analyst
  ever touching them"; every triage decision logs risk signals, scores and reasoning.
- **Decision re-application:** compares incoming matches with history to dispose of repeat patterns (screening).
- **Overlay path:** "AI Overlay for Transaction Monitoring" claims "Reduce false positives by 80%".

### 1.5 AI and GenAI to October 2026

Agentic design in four stages: **ingest** (map any source to the ontology), **detect** (AI plus rules), **triage** (triage
agent), **investigate** (a *manager agent* orchestrates *worker agents* for subject profiling, transaction analysis,
web research, narrative drafting; results on one screen/"evidence board"; "auto-populate SAR submissions, including all
relevant evidence attachments, ready for human review and approval"). Governance statements: "strict policy guardrails
dictate every agent action", "human oversight configurable from semi- to fully autonomous", "continuous drift and bias
monitoring", automatic model risk management documentation "including compliance with the EU AI Act and ISO 42001".
Claims: "90% false positive reduction, 6x improvement in investigator productivity, 70% faster case resolution, 30% more
SAR-worthy risk detected, >90% agent/investigator adjudication alignment" (tier-1 institutions; the sanctions agent PoC:
"99% false positive reduction achieved autonomously", ">98% agent-investigator adjudication agreement").
**Inconsistent marketing numbers** appear on SymphonyAI's own pages: "reduce false positives by 30%" (banking page),
"40% reduction in false alerts for sanctions", "up to 70% fewer false positives" (screening release), "77%" (Absa), "up to
80%" (SRI release), "85%" (FS page), "90%" and "99%". The sensible reading: 30-40% is the production baseline for a
re-tuned engine; the high figures come from overlays or proofs of concept on specific populations.

### 1.6 Case management and regulatory reporting

SRI Investigation: subject-centric single view across AML, KYC/CDD, sanctions and payment fraud, copilot, Narrative Agent
("create high-quality, consistent SAR narratives in seconds that align to your policies and procedures and **FIU
requirements**"), "fully integrated reporting". **goAML is not named** on any SymphonyAI page read; formats and
jurisdictions are not listed. (Metro Bank's case study adds "legal orders" and "court orders" to the platform.)

### 1.7 Data model and integration

A "FinCrime ontology" maps "disparate data sources to a shared structure"; "golden record for every customer and entity".
"Connect any source and map every record ... in real time." Integration is described as API-based and data-agnostic;
no core banking adapters named. Metro Bank: one platform for transaction monitoring, payment screening, name screening,
investigations, CRA, legal and court orders, ongoing due diligence.

### 1.8 Arabic/RTL and MENA/Iraq presence

| Question | Finding |
|---|---|
| Arabic UI/RTL | Not stated in the pages read |
| Arabic names | "350+ global watchlists across 60+ languages"; "AI-powered intelligent matching algorithms"; no Arabic-specific accuracy data |
| Islamic banking | Not mentioned |
| MENA customers | **Bank of Bahrain and Kuwait** (BBK; Bahrain, Kuwait, India, Turkey, UAE), Dec 2024, SaaS AML TM and CDD; Absa (South Africa) |
| Iraq | None found |

### 1.9 Implementation time, cost and licensing

No prices. Claims: "Modular applications deploy in weeks - not months"; "New models can be deployed in weeks"; Metro Bank's
cut-over "within 24 hours" (the project to reach that point took longer; the case study gives no programme duration).
A SymphonyAI-sponsored survey (FinCrime Frontier 2026-27, 200+ leaders, with AML Intelligence) says **70.8% of
respondents report that 5% or fewer of the alerts they investigate lead to an escalation or SAR/STR**, 76.3% review
alerts manually or with partial automation, and only 4.7% continuously update controls as risk changes.

### 1.10 Analyst rankings

Forrester Wave AML Q2 2025: Leader (top scores 5/5 on admin management, data integration, AI/ML risk scoring, case
management, third-party integrations, queue definitions and reporting; a "good fit for companies that require broad,
out-of-the-box support for ACH and wire transactions"). Forrester Wave Financial Crime Management Q3 2026: Leader,
highest Strategy score ("SymphonyAI's vision excels, with a strong focus on compliance effectiveness, regulatory-ready AI,
and modularity"; "watchlist integration and use is outstanding"; "well suited for large financial services enterprises
that require defensible risk scoring"). Chartis RiskTech100 2027: top third for the second year, category award
"Agentic Platforms - Compliance". Chartis Watchlist and Adverse Media Monitoring 2026: Category Leader in both quadrants,
Workflow score 4.4 "joint highest". PeerSpot: no reviews; 1.0% mindshare in the fraud category.

### 1.11 What it does right

1. The largest out-of-the-box scenario library (2,500+) and pre-trained model store.
2. A **phased, overlay-first adoption path**: add AI on top of an existing engine, then replace later.
3. Agent design with **auditable triage and auto-hibernation**, human-in-the-loop configurable by case type.
4. **Customer-level alert roll-up** (20% fewer alerts at Metro Bank) and a single subject-centric view.
5. Screening engineering: 350+ lists, 60+ languages, 40 ms, SWIFT Alliance Access plug-in, decision re-application.
6. Strongest Forrester Strategy score in the Q3 2026 Wave; Chartis screening leadership.
7. Evergreen SaaS upgrades and fast cut-over (hours) once data is mapped.
8. A MENA reference (BBK, multi-country).

### 1.12 What it does wrong

| # | Weakness | Evidence |
|---|---|---|
| 1 | **Cloud/SaaS-first; on-premises not offered in current messaging** | SymphonyAI pages; BBK and Metro Bank are SaaS |
| 2 | **Inconsistent and inflated numbers** | 30% to 99% across its own pages; PoC figures presented as outcomes |
| 3 | **Churning owners and names** | Norkom, BAE, SymphonyAI; Detica NetReveal, NetReveal, Sensa-NetReveal, SRI: roadmap risk, rebrand cost |
| 4 | **Migration burden between versions** (as a prior state) | Metro Bank: evergreen SaaS replaces "the major, resource-intensive programme required for the migration from previous versions" and "difficulty in introducing enhancements ... without significant overheads" |
| 5 | **Practitioner view of the Detica heritage** | PeerSpot reviewer: "BAE Detica System is also not that great in terms of the UI ... Oracle has many manual deployments and implementations, and it's the same with Detica Systems" |
| 6 | **No goAML, Arabic, Islamic, Iraq evidence** | Section 1.8 |
| 7 | **Thin independent reviews** | PeerSpot: no reviews |
| 8 | **Ownership and consolidation risk** | Private; reported Actimize bidder (a combination of the two would reshape the market) |

### 1.13 Lessons for AKR-TMS

- Copy: overlay-first adoption; triage agent with auditable hibernation and per-decision reasoning; entity-level roll-up;
  decision re-application for repeat screening hits; an ontology for the canonical model.
- Avoid: a SaaS-only story; a scenario count as a headline (publish coverage against typologies instead); mixed
  efficacy numbers.
- Beat: on-premises, Arabic and goAML, and a transparent evidence page for every claim.

### AKR-TMS hooks (SymphonyAI)

- Offer an **overlay mode** from day one: import alerts and transactions from the bank's current tool, add AKR-TMS
  triage and investigation, then retire the old engine on the bank's schedule (VA-13 in file 08).
- Build **decision re-application** for repeat screening and alert patterns, with expiry, sampling and MLRO approval
  (VA-48).
- Publish an **evidence page per marketing claim** (what population, what baseline, production or proof of concept)
  (VA-49).

---

## 2. FICO TONBELLER Siron AML (divested by FICO in December 2022)

**Evidence warning.** FICO's website is behind a bot-protection wall (Incapsula; HTTP block page also in a real browser),
FICO's investor pages return "Access Denied", and the Wayback Machine rate-limited the request. The product pages could
not be read. What follows is limited to (a) FICO's SEC filings, (b) PeerSpot and Wikipedia, (c) ACTICO's public
pages as a possible comparator. Treat product detail as **unverified** and ask any bidder for current documentation.

### 2.1 Ownership and history

| Date | Event | Source |
|---|---|---|
| 1990s | TONBELLER AG (Bensheim, Germany) builds the **Siron** compliance products (AML monitoring, KYC, embargo/sanctions filtering, FIU reporting) | Industry knowledge; not verified in sources read |
| 2015 | **FICO acquires TONBELLER AG** (listed among FICO acquisitions "TONBELLER AG 2015") | Wikipedia, FICO |
| **Dec 2022** | **FICO sells "certain assets related to our Siron compliance business"**; gain USD 1.9m; fiscal 2023 | FICO Form 10-K FY2023 (MD&A: "sale of certain assets related to our Siron compliance business in December 2022") and FY2025 note 2 |
| Since | The 10-K does **not name the buyer**; FICO's metrics exclude Siron "for all periods presented" | FICO 10-K |

So "FICO TONBELLER Siron AML" is **no longer a FICO product line**. PeerSpot still lists "FICO Siron AML" under FICO
(a stale vendor tag); the current owner, brand and roadmap must be established directly. ACTICO (Germany, founded
1997, Compliance Solutions including AML monitoring with "FIU reporting ... upload the XML file to the FIU via
goAML", customer and payment screening, an Avaloq core adapter, SaaS or customer-hosted deployment, GenAI drafts for
SARs, MROS for Switzerland) serves the same DACH customer base; **no source read links it to the Siron sale**, so it
is shown only as a comparator.

### 2.2 What is known from practitioners

PeerSpot ("FICO Siron AML", 1 review, 3.5/5, #4 in the AML category, mindshare 4.6% down from 5.8%): reviewer is a
programme manager and Oracle consultant at Orange (telecom), user for about six years, May 2022:
- "I am using FICO Siron AML for real-time transaction screening. We perform screening in batch and real-time mode.
  Real-time screening is the most important feature."
- "The scalability of FICO Siron AML is not as good as if it was in the cloud. However, we are working with personal data
  and we do not use the cloud because of regulatory laws." (on-premises by regulatory choice)
- "I rate the price ... a two out of five. The price could be less expensive." Licence "is based on how many customers
  you have." Reporting and first-level support need improvement; "stable".
- PeerSpot lists customers 4finance, Worldpay, State of Maryland, SA Taxi, Mastercard.

### 2.3 Architecture, detection, AI, reporting, Arabic, MENA

Not verifiable (see warning). Visible facts: on-premises operation with batch and real-time screening (review);
licence by number of customers (review); an earlier search result showed a FICO investor release about a bank in
Azerbaijan using Siron (not readable). **No evidence of GenAI features, Arabic UI, Islamic content or an Iraq presence.**
*Update 2026-10-06 (file 18):* Iraqi annual reports do show Siron in production: Bank of Baghdad runs "SIRON" for daily
monitoring on CBI scenarios (reports 2022-2025), and National Bank of Iraq launched "AML Sirion" in early 2024 next to its
Temenos migration (report 2024). The vendor publishes neither reference.

### 2.4 Analyst rankings and ratings

No FICO Siron result found in the Forrester AML Wave Q2 2025 materials; Chartis FCC50 2025/2026 vendor tags do not
include FICO (they include ACTICO); PeerSpot as above.

### 2.5 What it does right / wrong (from the evidence available)

| Right | Wrong |
|---|---|
| Real-time screening valued by a long-term user; on-premises operation suited to strict data-protection regimes; stable | A parent that exited the product line in 2022; unknown owner and roadmap; weak first-level support and reporting (one reviewer); licence per customer count and "price could be less expensive"; cloud scalability gap; no AI evidence |

### 2.6 Lessons for AKR-TMS

- **A product can be sold off**: for a bank, vendor exit is a real risk (FICO exited Siron; BAE exited NetReveal; NICE is
  selling Actimize). Source escrow and open formats are not luxuries (VA-01 and VA-02 in file 08).
- The Orange reviewer chose on-premises "because of regulatory laws": exactly the Iraqi pattern; it validates an
  on-premises-first design.
- Licence-by-customer-count creates a perverse incentive to prune the customer base; price on bank tier and modules.

### AKR-TMS hooks (Siron)

- Provide **customer-count-independent pricing** and an explicit vendor-exit clause set (escrow release, 24-month support
  bridge, data and configuration export) in every contract (VA-50).

---

## 3. Nasdaq Verafin (consortium analytics, SaaS only)

### 3.1 Ownership and history

| Date | Event | Source |
|---|---|---|
| 1999 | Verafin founded in St. John's, Newfoundland, Canada (cloud-first from the start) | Industry knowledge; not verified in sources read |
| **Feb 2021** | **Nasdaq completes the acquisition of Verafin** "for an aggregate purchase price of $2.75 billion" (including USD 102m cash subject to employment conditions over three years); 776 Verafin employees at 31 Dec 2021 | Nasdaq FY2021 Form 10-K |
| 2023-2026 | Financial Crime Management Technology revenue **USD 223m (2023), USD 273m (2024), USD 331m (2025)**; growth 22.2% and 21.5%; ARR and quarterly annualised SaaS revenue **USD 226m, 278m, 329m** at year-ends; **USD 359m at 30 Jun 2026** (USD 308m a year earlier) | Nasdaq 10-K FY2025 and 10-Q Q2 2026 |
| 2025-26 | "more than 2,750 North American financial institutions, including G-SIBs" (10-K); "more than 2,800 financial institutions" (10-Q, Jun 2026) | Nasdaq filings |

Arithmetic: USD 331m of 2025 revenue over about 2,750 institutions is roughly **USD 120,000 per institution a year** on
average, far below Actimize's average (about USD 450,000 per client) because Verafin's base is mostly community banks and
credit unions.

### 3.2 Modules, architecture and deployment

"A cloud-based solution to financial institutions for fraud detection and management, anti-money laundering and
countering the financing of terrorism compliance and management, high-risk customer management, sanctions screening and
management, and **information sharing**." **Software-as-a-service only**: "Nasdaq Verafin is offered as a cloud service
whereby the software is hosted and managed for customers", "subscription agreements are generally **three years** in term,
payable in advance, with the option of automatic renewal for some products" (10-K revenue note). The solution is
"designed to support a range of client needs, from smaller financial institutions using integrated applications to larger
institutions accessing specific capabilities through APIs". North America only in the filings ("North American
financial institutions"). **No on-premises option.**

### 3.3 Detection approach and AI

"Nasdaq Verafin has leveraged AI for more than 20 years ... combining deep domain and technical expertise with
**consortium data**." Its "AI-based **Targeted Typology Analytics** solution examines a range of behavioral,
transactional, third-party, and consortium insights for more effective detection of crimes with fewer false positives".
The 2026 10-K adds: "We continue to enhance the platform with additional automation and AI-based capabilities,
including agentic AI". Consortium means models and risk signals learn across institutions (cross-institution
network view, for example shared counterparties and mule patterns), which no single-institution engine can replicate.

### 3.4 Case management, reporting, data and CBS integration

Built-in alert and case management and e-filing for **US (FinCEN BSA E-Filing) and Canadian (FINTRAC) reporting**
(industry knowledge; the filings do not detail it); no goAML. Integration: Verafin extracts data from the bank's core
through established adapters with US core processors (not verified here); the filings only say "integrated applications"
for smaller institutions and "APIs" for larger ones.

### 3.5 Arabic/RTL, MENA/Iraq

None. The platform is English-language, US/Canadian-regulation-shaped and SaaS-only; nothing in the filings suggests
Gulf or Iraqi availability. **Not an option for an Iraqi bank** under on-premises-default conditions.

### 3.6 Implementation, cost, analyst ranking

- Cost signals: three-year subscription paid in advance; implied average about USD 120,000 per institution per year;
  Nasdaq describes its software professional services (segment-wide) as "primarily ... up-front non-complex
  implementations" (a sign that the product is meant to be implemented quickly, not a figure for Verafin alone).
- Analyst: appears in Chartis FCC50 2025 and 2026 vendor lists (rank paywalled); no Forrester Wave AML Q2 2025 result found
  in the vendor releases read; Gartner, G2, TrustRadius not readable. Nasdaq reports 2,800+ clients and 21-22% growth.

### 3.7 What it does right / wrong

| Right | Wrong |
|---|---|
| **Consortium intelligence** and a single platform for fraud, AML, sanctions and information sharing; **low average price point** (about USD 120k per institution a year) proving that a standardised SaaS can serve small FIs; fast organic growth (21-22% a year); AI used for 20 years; tight integration with the US ecosystem | **SaaS only, North America only**; three-year prepaid contracts (lock-in, cash outlay); no data-residency control; no goAML or Arabic; limited public technical documentation; a platform tuned to US regulation (SAR/CTR) |

### 3.8 Lessons for AKR-TMS

- **Copy the product packaging for small institutions**: a standard, pre-integrated configuration with minimal
  administration, and a modest annual price; Iraq has many small private banks and exchange companies (VA-51).
- **Copy the idea of shared intelligence**, but in a form Iraqi law and bank culture allow: CBI/FIU-issued typology updates
  and, with a legal basis, hashed identifiers of confirmed suspects shared among participating banks through a
  neutral operator (VA-52).
- **Avoid** prepayment and three-year lock-ins; offer annual terms with a free exit.

---

## 4. Fiserv AML Risk Manager (core-vendor-bundled AML)

### 4.1 Ownership and history

Fiserv Inc. (US; "the largest fintech at the intersection of banking and commerce", Fiserv homepage Oct 2026; "serves
clients in more than 100 countries"; regional sites include Europe, **Middle East** (English) and Africa). The history of
AML Risk Manager (acquired or built, and when) was **not found** in the pages read. Marketed for "financial
institutions, insurance companies, and investment firms".

### 4.2 Modules and capabilities

AML Risk Manager: AI-driven risk scoring of transaction patterns and customer behaviour; pre-built regulatory reports and
customisable dashboards; KYC scoring (continuously updated); watch-list filtering ("OFAC, PEP, EU, UN and FATF lists",
real-time automated updates); profiling of customers, transactions, policies and relationships; scenario-based monitoring
(structuring, layering, rapid movement of funds); e-filing inside the platform; versions for FIs, insurers (early policy
cash-outs, third-party beneficiary risk) and investment firms (SEC and FINRA). Companion product: Payment Fraud Manager.

### 4.3 Architecture, deployment, data and integration

Not documented in the pages read (hosting model, database, batch or real time). Case studies show bank and insurer
customers using it for transaction monitoring, watch-list filtering and customer risk rating; a custom scenario was
built by Fiserv for a third-party-payer risk at Empire Life.

### 4.4 Case studies and timelines

- **Cadence Bank** (US, 98 branches, USD 18.8bn assets, 2021): replaced an "archaic", "inflexible" system with
  high false positives and manual SAR filing; "more user-friendly, provides very distinct alerts and allows us to do
  e-filing within the platform"; the bank sets its own risk thresholds.
- **Empire Life** (Canadian insurer): decided to streamline in **2016**; **transaction monitoring launched 2020**,
  **watch-list filtering 2021**: a four-to-five-year path from decision to full scope; the objective risk ranking
  "decreased the number of clients identified as high-risk".

### 4.5 AI, GenAI, reporting, Arabic, MENA

"AI-driven risk scoring" and machine learning to rank alerts (white paper 2022); no GenAI or agentic features named on the
pages read (Fiserv announces an OpenAI collaboration and "AgentOS" at corporate level). Reporting is US-centric (SAR
e-filing, CTR) plus Canadian references (Empire Life). **No goAML, Arabic, Islamic or Iraq evidence.**

### 4.6 Cost, analyst rankings

No prices; no analyst ranking found (not in the Forrester Wave Q2 2025 releases read; not in Chartis FCC50 vendor tags;
PeerSpot has no page). Fiserv's own AML offering is sold mostly alongside core banking, cards and payments.

### 4.7 What it does right / wrong

| Right | Wrong |
|---|---|
| Embedded in a core-processing vendor's ecosystem (fast data access for its own cores; one contract); in-platform e-filing and "very distinct alerts" cited by a bank; insurer and investment-firm variants | Opaque documentation; no published detection depth, scenario count or tuning evidence; slow (four-to-five-year) enterprise rollout at one insurer; no on-premises or goAML evidence; tied to a US vendor's priorities; no independent rating |

### 4.8 Lessons for AKR-TMS

- A **core-bundled AML module** is the competitor in many Iraqi banks (the core vendor offers its own); AKR-TMS must be
  CBS-agnostic, read-only and clearly better on the CBI scenario pack, Arabic and cost, and must make switching from
  the bundled module a one-month project (VA-53).
- In-platform e-filing and "very distinct alerts" are what bank staff praise: keep alert explanations plain and e-filing
  one click (VA-14 in file 08).

---

## 5. Comparison of all incumbents (files 08, 09, 10 and this file)

Evidence date: 6 Oct 2026. "Documented" means found in a primary vendor document or a regulator/filing; "not found" means
searched in the vendor's public material and not found (it may exist behind a customer login). Cells that rest on vendor
marketing say so.

### 5a. Corporate position and delivery

| | NICE Actimize | Oracle FCCM | SAS AML | SymphonyAI NetReveal | FICO TONBELLER Siron | Nasdaq Verafin | Fiserv AML Risk Manager |
|---|---|---|---|---|---|---|---|
| Owner and status (Oct 2026) | NICE Ltd; **being sold**: Brookfield reported in exclusive talks at about USD 2bn (23 Sep 2026) | Oracle; stable; product retirements (Alert Management, Trading Compliance, Scenario Wizard) | SAS Institute: private, debt-free, over USD 3bn sales | SymphonyAI (SAIGroup, private); NetReveal bought from BAE in Oct 2022; reported Actimize bidder | **Sold by FICO in Dec 2022**; buyer not named in FICO's 10-K | Nasdaq (acquired Feb 2021, USD 2.75bn) | Fiserv (public); AML line history not found |
| Scale signals | 1,000+ organisations (all products); 2024 revenue USD 453.5m | 180+ FIs (Oracle claim) | 2,700+ financial-services customers (all products) | 2,000+ customers (all verticals); "33% of the largest FIs" | 1 review (Orange) | 2,800+ FIs (Jun 2026); ARR USD 359m | not found |
| Average revenue per client | about USD 0.45m a year (arithmetic) | not published | not published | not published | not published | about USD 0.12m a year (arithmetic) | not published |
| Deployment | Containerised on-premises (SAM-10), private/public cloud, SaaS (X-Sight); Xceed and AML Essentials are SaaS-only | On-premises (OFSAA, Oracle DB, WebLogic/Tomcat) and **OCI cloud service** (modular) | Viya 4: cloud, hybrid, on-premises; **Remote Managed** on customer infrastructure; screening service cloud-native | **SaaS / "hybrid-cloud"**; on-premises not offered in current messaging | On-premises (per the one review) | **SaaS only** | not documented |
| Core detection run | **SAM: daily batch** (UDM ETL); screening real time (200 ms) | **Batch (nightly to quarterly)** plus Inline Processing Engine for real-time hold/release | **Nightly batch** core schema; real-time screening and fraud | "Always-on" and continuous-coverage marketing; fraud scoring under 50 ms; AML processing mode not detailed | batch and real-time screening (review) | SaaS; processing mode not detailed in the filings | not documented |
| Documentation openness | customer portal; many public brochures | **Most public** (docs.oracle.com), except data model and scenario descriptions (MOS) | **Closed** (access key) | marketing only | n/a | marketing only | marketing only |
| Licence model signals | one-time/term licence, maintenance, change orders, separate packages; "expensive" | **multi-meter usage** (transactions, screens, users, templates, AI narratives); "very expensive" | not published | not published; SaaS | per customer count (review) | 3-year prepaid subscription | not published |

### 5b. Detection, AI and governance

| | NICE Actimize | Oracle FCCM | SAS AML | SymphonyAI NetReveal | FICO Siron | Nasdaq Verafin | Fiserv |
|---|---|---|---|---|---|---|---|
| Scenario library | 250+ rules, "all FATF typologies" (claim) | library not public (MOS); scenario bindings, sequence patterns | catalogue (Mashreq picked 8 new scenarios); correspondent scenarios | **2,500+** out of the box | not verified | typology analytics with consortium data | scenario-based (structuring, layering, rapid movement) |
| ML approach | segmentation, 2-hour tuning, anomaly, predictive scoring and hibernation, community graph | behavioural models, event scoring, calibration, Compliance Agent simulation, PGX graph | dynamic segmentation, ATL/BTL tuning, predictive alert analytics, entity-level aggregation | pre-trained Model Store, MLOps, triage agent with auto-hibernation, overlay | not verified | consortium-trained models | "AI-driven risk scoring" |
| Shipped GenAI/agents (Oct 2026) | InvestigateAI, NarrateAI, embedded agents (Jun 2026) | AI Investigator, AI narratives (cloud services), chatbot UI | Viya Copilot (Foundry) incl. VI alert/case narratives; **fincrime copilot "planned"** | **most agentic**: manager/worker agents, SRI Agents, Copilot | none found | "agentic AI" mentioned in 10-K | none found |
| Alert-to-STR evidence | multi-national bank: 27% of L2 alerts led to SAR | none published | none published | industry survey: 70.8% say <=5% of alerts end in escalation/STR | none | none | none |
| Published FP-reduction claims | 31%, 33%, up to 85% | ">60%", "up to 50%" | "up to 80%", Landsbankinn "90%" | 30% to 99% across pages | none | "fewer false positives" | none |
| Entity-level alerts | entity-centric, graph | cases by focus | **yes (documented)** | **yes (Metro Bank -20%)** | not verified | not verified | not verified |

### 5c. Fit for Iraqi banks

| | NICE Actimize | Oracle FCCM | SAS AML | SymphonyAI NetReveal | FICO Siron | Nasdaq Verafin | Fiserv |
|---|---|---|---|---|---|---|---|
| **goAML STR** | not found | **Documented** (cloud; XSD templates; v5.0.2; validation, e-file, reject workflow, Aug 2026) | not found | not found | not found for the current owner | no | no |
| Arabic matching / UI | 23 "cultures" (list not published), "Cultural Match" agent; UI not documented | **On-prem BD matcher ignores Arabic (admin guide)**; cloud screening has Arabic-Latin transliteration; UI not documented | cultural-origin AI + transliteration in screening; UI not documented | "60+ languages" lists; no Arabic detail | not found | no | no |
| Islamic products | no | no (no Islamic template in 67 staging tables) | no | no | no | no | no |
| MENA references | UAE Exchange (2015) | logos incl. "Aman Bank" (unverified); contacts in 10 Arab countries, not Iraq | **Mashreq (UAE, since 2006)**; offices Dubai, Riyadh, Doha | **BBK (Dec 2024)** | none found | none | Middle East regional site only |
| Iraq presence | none | none (no Iraq contact line) | none (no Iraq contact line) | none | none | none | none |
| On-prem under CBI defaults | yes (containerised) | yes | yes (Viya or Remote Managed) | **no (not offered)** | yes (legacy) | **no** | unknown |
| Forrester Wave AML Q2 2025 | **Leader** (top 10/10 current-offering criteria) | not found | **Leader** (4.40) | **Leader** | not found | not found | not found |
| Forrester Wave Financial Crime Mgmt Q3 2026 | no NICE announcement found | not found | **Leader, highest current offering 4.70** | **Leader, highest Strategy and combined score** | n/a | not found | n/a |
| Chartis | FCC50 #1 (2024); RT100 2024 #15 | AML category leader (Oracle claim, 2020-24); FCC50 listed | RT100 2027 **#3 overall** | RT100 2027 top third; screening Category Leader 2026 | not in FCC50 | FCC50 listed | not listed |
| Practitioner evidence | PeerSpot 4.1/5 (18 reviews): complex, expensive | PeerSpot 1 review: slow, expensive; reviewer prefers its engine | PeerSpot: none | PeerSpot: none | PeerSpot 3.5/5 (1 review) | none read | none read |
| **Verdict for a large Iraqi private/state bank** | Capable, costly, ownership in flux; Arabic and goAML unproven | Capable and heavy; the only documented goAML; Arabic issue on-prem | Capable, heavy, closed docs; Arabic-aware screening; goAML unknown | Modern but **cloud-first**: only if an on-premises or sovereign-cloud option is confirmed | Legacy; ask the new owner | **Not available** (SaaS, North America) | **Not applicable** unless the bank's core is Fiserv |
| **Verdict for a small Iraqi bank** | Too expensive | Too heavy | Too heavy | Cloud-first | n/a | n/a | n/a |

### 5d. What the table says in four lines

1. **No incumbent documents the full Iraqi bundle**: real-time hold, direct read-only CBS link, Arabic-native matching,
   goAML, CBI scenario catalogue, Islamic products, on-premises, at a price a small bank can pay.
2. **Oracle** is closest on reporting (goAML) and real-time hold; **SAS and SymphonyAI** lead on analyst scores and
   agentic AI; **NICE** leads on case-management reputation and rule depth, but is changing owner.
3. **The cloud-first drift** (SymphonyAI, Verafin, NICE Xceed/Essentials, Oracle's newest features, SAS screening) is
   a structural opening for an on-premises-first product.
4. **Marketing numbers are not comparable**: 30% to 99% false-positive reductions; the only published alert-to-SAR
   rates are 27% (NICE case) and the survey's "5% or fewer" for most banks.

**Paywalled reports that would settle the ranking questions** (positions were not readable; vendors' newsrooms read do not
publish all results): Chartis "AML Transaction Monitoring Solutions, 2026: Quadrant Update" (9 Sep 2026; 2025 edition
6 Nov 2025; SAS vendor analysis 10 Aug 2026), Forrester Wave: Financial Crime Management Solutions, Q3 2026 (15 vendors;
SAS highest current offering at 4.70/5, SymphonyAI highest Strategy and combined score), Chartis RiskTech100 2027
(24 Sep 2026; SAS No. 3), Chartis FCC50 2026 (9 Mar 2026). If the plan PDF will cite quadrant positions, buy or
request those four reports rather than relying on vendor releases.

---

## 6. What it means for AKR-TMS

**1. The market is consolidating, relocating to the cloud and changing owners.** NICE is being sold (exclusive talks at
about USD 2bn, 23 Sep 2026) after a sale process that began in Nov 2025; BAE sold NetReveal to SymphonyAI in 2022;
FICO sold Siron in Dec 2022; Oracle has retired products inside the suite; Verafin is SaaS-only; SymphonyAI and the
newest NICE, Oracle and SAS features are cloud services. For an Iraqi bank the practical consequences are vendor-exit risk,
roadmap uncertainty and data-residency friction. **Continuity (escrow, open formats, LTS releases) is a feature
AKR-TMS can sell and the incumbents cannot easily match.**

**2. No incumbent documents the Iraqi bundle.** Oracle is the only one documenting goAML and a real-time hold path; its
on-premises name matcher ignores Arabic. SAS and SymphonyAI document cross-script screening but not goAML; NICE
documents neither. None documents Islamic products. None has an Iraqi office or reference. The gap is not detection
theory; it is **fit, language, reporting, price and operating model**.

**3. Table stakes AKR-TMS must match** (every incumbent offers these): a rule library mapped to typologies plus ML for
segmentation and prioritisation; entity-level cases (customer roll-up); alert triage with hibernation and sampling;
case management with QA and audit; screening at payment speed; STR drafting assistance with human approval; model
governance documents; a published, regulator-readable explanation of every alert.

**4. Design lessons that recur across vendors.**

| Lesson | Evidence | AKR-TMS rule |
|---|---|---|
| Data completeness decides everything | TD Bank (92% of volume unmonitored), Metro Bank (60m transactions missed for three years) | Coverage reconciliation and register on the front page |
| Changing a scenario must be easy | TD Bank (SAS: "no new scenario since the install"); Actimize "slow and lengthy tuning cycles" (its own brochure) | Same-day scenario change with history replay |
| Heavy ETL and product-specific data models | Actimize runbook (three staging databases, about ten VMs), Oracle (67 CSV templates; 2026 "Unified Data Loading") | One canonical, product-agnostic, open model and ready mapping packs |
| Licensing by meters punishes growth | Oracle (transactions, screens, users, templates, AI narratives); Actimize (separate packages, change orders); Verafin (3-year prepaid) | Flat annual price per bank tier, everything included |
| Marketing numbers are not evidence | 30% to 99% false-positive claims; 27% alert-to-SAR; survey "<=5%" | Claims ledger and a replay pilot on the bank's own history |
| Cloud-only AI excludes on-premises banks | Oracle (agents are cloud services), SAS (Foundry), SymphonyAI (SaaS), NICE (hosted GenAI undocumented) | Local-model assistant and no mandatory outbound path |
| Multi-bank operation needs a standard image plus overlay | SAS (Bangkok Bank Global Image; SDC for 120 banks) | Image plus overlay, optional shared-service edition |
| Small institutions need a packaged product | Verafin (USD 0.12m average), Actimize Xceed/Essentials | A "small bank pack": pre-mapped, minimal admin, fixed price |
| Regulators and peers take comfort from brand | PeerSpot: "gives regulators comfort that our monitoring software is up to scratch" | An evidence pack per bank (validation, tuning log, coverage, CBDDQ answers) |
| Vendor exit is real | FICO, BAE, NICE | Escrow, 24-month support bridge, open export |

**5. Positioning.** AKR-TMS should be described to bank leadership as: *on-premises, read-only, Arabic-native, goAML-ready,
CBI-scenario-ready, Islamic-aware, flat-priced, continuity-guaranteed*, and explicitly **not** "AI that removes 90% of
alerts". The credible promise is: **a measured reduction in noise on the bank's own data, shown in a pilot, with the
tuning history and the coverage figure visible every day.**

### AKR-TMS hooks (cross-vendor)

- **Overlay and parallel-run mode**: run next to the bank's existing engine for 30-90 days, compare alerts, measure
  alert-to-STR conversion on both, then migrate (VA-54).
- **Pilot replay protocol**: before commitment, replay 12 months of the bank's history through AKR-TMS and report alert
  volumes by scenario, overlap with the incumbent engine, and the STRs that would and would not have been caught
  (VA-55).
- **No mandatory outbound dependency**: every product feature (including the assistant, screening lists and updates)
  works with no internet path, with signed offline update bundles for lists, scenarios and models
  (VA-56).

## 7. Derived requirements (VA-48 to VA-56 from this file; VA-01 to VA-47 are in files 08, 09 and 10)

| ID | Requirement | Source |
|---|---|---|
| VA-48 | Decision re-application: when a new alert or screening hit matches a previously adjudicated pattern (same parties, same attributes), propose the earlier disposition with an expiry, below-the-line sampling and MLRO approval; never auto-close without a policy that names the pattern | File 11 s.1 (SymphonyAI Decision reApplication; triage agent auto-hibernation); file 08 s.4 (hibernation controls) |
| VA-49 | A published claims ledger: every performance figure shown to a bank (false-positive reduction, speed, STR conversion) names its population, baseline, period and whether it is production or proof of concept; no figure without a baseline | File 11 s.1.5 (SymphonyAI claims range 30% to 99%); file 08 s.4; file 10 s.4 |
| VA-50 | Standard vendor-exit terms in every contract: escrow release triggers, 24-month support bridge on vendor sale or insolvency, change-of-control notice and termination right, full export of configuration, history and audit trail, no pricing by customer count | File 11 s.2 (FICO sold Siron Dec 2022; licence by customer count); s.5 (NICE sale, BAE exit) |
| VA-51 | A packaged configuration for banks and exchange companies with a one-to-three person compliance team: pre-mapped adapter, default CBI scenario pack and thresholds, minimal administration, guided tuning, fixed annual price, remote-managed option | File 11 s.3.7-3.8 (Verafin average about USD 0.12m per institution a year; integrated applications for small FIs) |
| VA-52 | Optional, legally grounded cross-bank signal sharing through a neutral operator: CBI/FIU-issued typology updates and, where law permits, hashed identifiers of confirmed suspects or mule accounts shared only among participating banks, with no raw customer data, full audit and opt-out | File 11 s.3 (Verafin consortium data); file 04 s.3 (Egmont); file 01 (AML law information sharing) |
| VA-53 | A migration kit from core-vendor-bundled AML modules (alert and case export, scenario and threshold mapping, screening list and decision history import) that lets a bank move to AKR-TMS in about one month with parallel run | File 11 s.4 (Fiserv AML Risk Manager as core-bundled AML); files 12-18 (CBS-native AML) |
| VA-54 | Overlay and parallel-run mode: AKR-TMS runs beside the bank's existing monitoring engine for 30-90 days, ingests its alerts and decisions, compares outcomes and alert-to-STR conversion, and supports a staged cut-over by scenario or product | File 11 s.1.11 and s.6 (SymphonyAI overlay path; SAS Financial Crimes Analytics overlay, file 10 s.2) |
| VA-55 | Pilot replay protocol: replay 12 months of the bank's history through AKR-TMS before commitment and report alert volume by scenario, overlap with the incumbent engine, and the STRs that would and would not have been caught, signed off by the MLRO | File 11 s.6 (non-comparable vendor claims); file 09 s.5 (Compliance Agent evidence idea) |
| VA-56 | Every product feature, including the assistant, screening list updates, scenario and model updates, works with no internet path; updates arrive as signed offline bundles with versioned release notes and rollback | File 11 s.6 (cloud-only AI in Oracle, SAS, SymphonyAI); file 01 s.8 (on-premises default); file 08 s.5 |

### Index of all vendor-research requirements (VA-01 to VA-56, files 08-11)

| ID | Requirement (short) | Full text in file |
|---|---|---|
| VA-01 | Source, schema and configuration escrow; ownership-change protection | 08-vendor-nice-actimize.md |
| VA-02 | Open-format export of logic, tuning history, cases and data model | 08-vendor-nice-actimize.md |
| VA-03 | Built-in read-only extract pipeline: checksums, backlog catch-up, sanity report | 08-vendor-nice-actimize.md |
| VA-04 | Single runtime with built-in scheduling and secrets; late-load alarms | 08-vendor-nice-actimize.md |
| VA-05 | CBS-action task queue with read-only verification | 08-vendor-nice-actimize.md |
| VA-06 | Rules + ML + graph layers, each explainable and switchable | 08-vendor-nice-actimize.md |
| VA-07 | Tuning workbench: replay simulation, approval, audit | 08-vendor-nice-actimize.md |
| VA-08 | Hibernation with MLRO approval, sampling, re-escalation | 08-vendor-nice-actimize.md |
| VA-09 | Alert-to-STR conversion reporting; no baseline-free claims | 08-vendor-nice-actimize.md |
| VA-10 | Local-model GenAI with evidence links for every sentence | 08-vendor-nice-actimize.md |
| VA-11 | Arabic "Cultural Match" name assistant with reasons | 08-vendor-nice-actimize.md |
| VA-12 | Inventory of every model and AI component | 08-vendor-nice-actimize.md |
| VA-13 | Alert-agnostic intake, low-code workflow, QA, filing tracking | 08-vendor-nice-actimize.md |
| VA-14 | Branch report, acknowledgement, MLRO decision, goAML chain with STR quality check | 08-vendor-nice-actimize.md |
| VA-15 | Coverage reconciliation: control totals per source, product and day | 08-vendor-nice-actimize.md |
| VA-16 | Register of monitored and unmonitored products with owner and expiry | 08-vendor-nice-actimize.md |
| VA-17 | Ready read-only CBS adapters and mapping packs | 08-vendor-nice-actimize.md |
| VA-18 | Arabic and RTL as first-class in every screen, report and export | 08-vendor-nice-actimize.md |
| VA-19 | CBI scenario catalogue and Iraqi list formats on day one | 08-vendor-nice-actimize.md |
| VA-20 | Published flat subscription; no per-package or change-order fees | 08-vendor-nice-actimize.md |
| VA-21 | Three-node production footprint with sizing table | 08-vendor-nice-actimize.md |
| VA-22 | Versioned screening lists, point-in-time restore, hit suppression | 08-vendor-nice-actimize.md |
| VA-23 | Per-bank evidence pack for CBDDQ and CBI inspections | 08-vendor-nice-actimize.md |
| VA-24 | Near-real-time monitoring plus hold-until-cleared path | 08-vendor-nice-actimize.md |
| VA-25 | Tested migrations, rollback, preserved configuration | 08-vendor-nice-actimize.md |
| VA-26 | Islamic products as first-class segments and scenario inputs | 08-vendor-nice-actimize.md |
| VA-27 | In-product bilingual documentation of every scenario and threshold | 09-vendor-oracle-fccm.md |
| VA-28 | Entity-level "why did / did not it alert" tester | 09-vendor-oracle-fccm.md |
| VA-29 | Synthetic-launderer controls-testing harness | 09-vendor-oracle-fccm.md |
| VA-30 | XSD-driven, multi-version goAML with validation, e-file, Iraqi attributes | 09-vendor-oracle-fccm.md |
| VA-31 | Product-agnostic canonical model with Islamic extension tables | 09-vendor-oracle-fccm.md |
| VA-32 | Arabic-native matching on every name path with acceptance test | 09-vendor-oracle-fccm.md |
| VA-33 | Published weeks-based implementation timeline | 09-vendor-oracle-fccm.md |
| VA-34 | LTS releases and multi-year deprecation overlap | 09-vendor-oracle-fccm.md |
| VA-35 | Readable, versioned scenario, model and workflow definitions | 10-vendor-sas-aml.md |
| VA-36 | Open-box customer risk rating, re-runnable on history | 10-vendor-sas-aml.md |
| VA-37 | Same-day scenario change in the product, measured | 10-vendor-sas-aml.md |
| VA-38 | Remote-managed on-premises operation option | 10-vendor-sas-aml.md |
| VA-39 | Entity-level alert aggregation by default | 10-vendor-sas-aml.md |
| VA-40 | Iraqi-shaped synthetic data generator | 10-vendor-sas-aml.md |
| VA-41 | AI-assisted CBS-to-canonical mapping, human-approved | 10-vendor-sas-aml.md |
| VA-42 | Branch liaison officer window (PC and phone) | 10-vendor-sas-aml.md |
| VA-43 | Product image plus per-bank overlay | 10-vendor-sas-aml.md |
| VA-44 | Optional shared-service edition for small banks | 10-vendor-sas-aml.md |
| VA-45 | Household and family dimension (family number, mother's name) | 10-vendor-sas-aml.md |
| VA-46 | Agent-safe API with human-equivalent access control and audit | 10-vendor-sas-aml.md |
| VA-47 | Open bilingual product and technical documentation | 10-vendor-sas-aml.md |
| VA-48 | Decision re-application with expiry, sampling, MLRO approval | 11-vendor-other-incumbents.md |
| VA-49 | Claims ledger: population, baseline, production versus pilot | 11-vendor-other-incumbents.md |
| VA-50 | Vendor-exit contract terms | 11-vendor-other-incumbents.md |
| VA-51 | Packaged configuration for small banks and exchange companies | 11-vendor-other-incumbents.md |
| VA-52 | Optional legally grounded cross-bank signal sharing | 11-vendor-other-incumbents.md |
| VA-53 | Migration kit from core-bundled AML modules | 11-vendor-other-incumbents.md |
| VA-54 | Overlay and parallel-run mode | 11-vendor-other-incumbents.md |
| VA-55 | Pilot replay protocol on the bank's own history | 11-vendor-other-incumbents.md |
| VA-56 | No mandatory outbound dependency; signed offline updates | 11-vendor-other-incumbents.md |

## 8. Sources

All fetched 2026-10-06 unless marked "(search result only)". Vendor pages are marketing and are labelled as such.

**SymphonyAI / NetReveal (vendor and history)**
- Financial services home: https://www.symphonyai.com/financial-services/ ; platform (SRI): https://www.symphonyai.com/financial-services/platform/ ; transaction monitoring (FAQ "Is this the same as NetReveal"): https://www.symphonyai.com/financial-services/transaction-monitoring ; agentic AI: https://www.symphonyai.com/financial-services/agentic-ai/ ; investigation: https://www.symphonyai.com/financial-services/sensa-investigation-hub ; sanctions screening: https://www.symphonyai.com/financial-services/sanctions-screening/ ; banking: https://www.symphonyai.com/financial-services/banking/ ; about: https://www.symphonyai.com/about/ ; news list: https://www.symphonyai.com/news/?_sft_portfolio_companies=financial-services
- Forrester Wave FCM Q3 2026 page: https://www.symphonyai.com/resources/forrester-wave-tm-financial-crime-management-solutions-q3-2026?portfolio=financial-services ; Forrester Wave AML Q2 2025 page: https://www.symphonyai.com/resources/leader-2025-forrester-wave-anti-money-laundering
- Press releases: Introducing Symphony Risk Intelligence (24 Sep 2026): https://www.symphonyai.com/news/introducing-symphony-risk-intelligence-sri ; FinCrime Frontier 2026-27 (24 Sep 2026): https://www.symphonyai.com/news/fincrime-frontier-report-2026-27 ; Chartis Watchlist and Adverse Media (27 May 2026): https://www.symphonyai.com/news/chartis-watchlist-adverse-media-category-leader-2026 ; BBK (3 Dec 2024): https://www.symphonyai.com/news/financial-services/bank-bahrain-kuwait-financial-crime-aml
- Metro Bank case study (19 Dec 2025): https://www.symphonyai.com/resources/case-study/financial-services/metro-bank/
- Wikipedia, BAE Systems Applied Intelligence (Norkom 2011, NetReveal sale to SymphonyAI 2022): https://en.wikipedia.org/wiki/BAE_Systems_Applied_Intelligence
- PeerSpot, BAE Systems NetReveal: https://www.peerspot.com/products/bae-systems-netreveal-reviews ; three-product reviewer: https://www.peerspot.com/products/comparisons/nice-actimize-anti-money-laundering_vs_oracle-financial-crime-and-compliance-management-cloud-service

**FICO Siron and ACTICO**
- FICO Form 10-K FY2023 (Siron sale, Dec 2022; gain USD 1.9m): https://www.sec.gov/Archives/edgar/data/814547/000081454723000022/fico-20230930.htm ; FY2025: https://www.sec.gov/Archives/edgar/data/814547/000081454725000030/fico-20250930.htm ; FY2022: https://www.sec.gov/Archives/edgar/data/814547/000081454722000016/fico-20220930.htm
- Wikipedia, FICO (TONBELLER AG 2015): https://en.wikipedia.org/wiki/FICO
- PeerSpot, FICO Siron AML: https://www.peerspot.com/products/fico-siron-aml-reviews
- ACTICO: https://www.actico.com/ ; AML software: https://www.actico.com/solutions/compliance-fraud-management/aml-software/ ; compliance: https://www.actico.com/solutions/compliance-fraud-management/ ; about: https://www.actico.com/company/about-us/
- FICO product pages and investor pages (blocked by bot protection, not read): https://www.fico.com/en/products/fico-siron-aml ; https://investors.fico.com/ (search result only)

**Nasdaq Verafin**
- Nasdaq Form 10-K FY2025: https://www.sec.gov/Archives/edgar/data/1120193/000162828026007703/ndaq-20251231.htm ; Form 10-Q Q2 2026: https://www.sec.gov/Archives/edgar/data/1120193/000112019326000013/ndaq-20260630.htm ; Form 10-K FY2021 (Verafin acquisition, USD 2.75bn): https://www.sec.gov/Archives/edgar/data/1120193/000112019322000007/ndaq-20211231.htm
- Verafin's own site (bot protection, not read): https://verafin.com/

**Fiserv AML Risk Manager**
- Financial crime page: https://www.fiserv.com/en/solutions/financial-performance-risk/financial-crime.html ; AML Risk Manager: https://www.fiserv.com/en/solutions/risk-compliance/aml-risk-manager.html ; white paper (2022): https://www.fiserv.com/content/dam/fiserv-ent/final-files/marketing-collateral/white-papers/smarter-approach-to-anti-money-laundering-white-paper-0322.pdf
- Case studies: Cadence Bank https://www.fiserv.com/en/about-fiserv/resource-center/case-studies/advanced-money-laundering-detection-keeps-pace-with-banks-rapid-growth.html ; Empire Life https://www.fiserv.com/en/about-fiserv/resource-center/case-studies/holistic-aml-risk-monitoring-streamlines-compliance-insurance-investment-company.html
- Fiserv home and country selector: https://www.fiserv.com/ ; https://www.fiserv.com/en/country-selector.html

**Cross-vendor**
- Forrester Wave FCM Q3 2026 release by SAS (24 Aug 2026): https://www.sas.com/en_us/news/press-releases/2026/august/forrester-financial-crime-solutions.html ; Forrester Wave AML Q2 2025 releases (SAS, NICE): see files 10 and 08
- Chartis FCC50 2025 and 2026 listings (vendor tags): https://chartis-research.com/financial-crime/7947401/financial-crime-and-compliance50-2025-ranking-and-award-winners ; https://chartis-research.com/financial-crime/7947472/financial-crime-and-compliance50-fcc50-2026-ranking-and-award-winners
- A-Team Insight, NICE earnings call and reported Actimize sale (15 May 2026): https://a-teaminsight.com/blog/nice-earnings-call-puts-reported-actimize-sale-update-in-context/?brand=rti ; Calcalist (23 Sep 2026): https://www.calcalistech.com/ctechnews/article/iazqvedg2
- FCA Final Notice, Metro Bank (12 Nov 2024): https://www.fca.org.uk/publication/final-notices/metro-bank-plc-2024.pdf ; US DOJ TD Bank statement of facts (10 Oct 2024): https://www.justice.gov/media/1373151/dl

**Not reached:** FICO and Verafin websites (bot protection), Wayback Machine (HTTP 429), G2, TrustRadius, Gartner Peer Insights (blocked), any Iraq-specific reference for the four vendors in this file (none public), the Forrester Wave reports and Chartis ranking tables (paywalled), and architecture and licensing documents for Siron and Fiserv.
