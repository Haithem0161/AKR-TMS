# 12 — Vendors: AI-native detection (Quantexa, ThetaRay, Hawk, Feedzai, Featurespace/Visa, Tookitaki, Consilient) and published bank results from AI in AML

Status: done (2026-10-06)

Part 2 of 4 of the vendor research (AI-native and challenger AML vendors). Other file in this part: 13 (investigation
automation and challengers, with the comparison table covering both files). Requirement IDs use the prefix VB-xx and are
numbered across files 12 and 13. Incumbent suites are in files 08-11 (prefix VA); screening/data/KYC vendors in 14-16.

How to read the evidence grades used in this file:
- **Independent**: regulator, auditor, academic or customer-published figure with method disclosed.
- **Customer-reported**: the bank itself published the figure (still self-selected, but the bank bears reputational risk).
- **Vendor-reported**: figure comes from the vendor's own marketing, case study or press release.
- **Analyst-reported**: from Chartis, Gartner, Forrester, Celent, IDC and similar (their method is often paywalled).

## Contents

1. Quantexa (decision intelligence, entity resolution, graph)
2. ThetaRay (unsupervised AI for cross-border payments; Middle East clients)
3. Hawk (Hawk AI)
4. Feedzai
5. Featurespace (acquired by Visa; ARIC Risk Hub)
6. Tookitaki (FinCense; AFC Ecosystem federated typology sharing)
7. Consilient (federated learning)
8. Published bank results from AI in AML (HSBC and Google Cloud AML AI, Danske Bank, DBS, UOB, Standard Chartered, others)
9. What it means for AKR-TMS
10. Derived requirements (VB-xx, this file)
11. Sources

(Each vendor section covers: ownership/funding/acquisitions; product; detection approach and AI claims; published results
and credibility; deployment (SaaS vs on-prem/private cloud); explainability and model governance; Arabic and MENA presence;
pricing signals; customer reviews and criticisms; regulatory reception; what it does right; what it does wrong; lessons for
AKR-TMS; AKR-TMS hooks.)

---

## 1. Quantexa (decision intelligence, entity resolution, graph analytics)

### 1.1 Snapshot

| Item | Finding | Source and grade |
|------|---------|------------------|
| Ownership and funding | Private, London HQ. **Series F, 5 March 2025: USD 175m at USD 2.6bn post-money**, led by Teachers' Venture Growth (Ontario Teachers'); also British Patient Capital, Warburg Pincus, Dawn Capital, Evolution Equity Partners, AlbionVC and **HSBC** (HSBC is both investor and customer; press coverage lists HSBC and BNY on the board). Stated use of funds: platform, partnerships, North America, "selected M&A". | OTPP press page (vendor-published); pulse2 |
| Size | ARR above USD 100m ("Centaur"); licence revenue growth "nearly 40%" in 2024; 23 new customers in 2024; more than 800 staff; 16 offices. Companies House: **FY to March 2025 revenue GBP 126m (+49%), pre-tax loss GBP 25m (from GBP 48m), headcount 772**. Discontinued its News Intelligence Platform and folded the IP into the core platform. | OTPP page; tech.eu 8 Jan 2026 (from filing) |
| Product | **Decision Intelligence Platform** sold as "Platform Suites": Trusted Data Foundation (ingestion + entity resolution), Data Modernisation (Entity 360, data quality, master data hub), Contextual Data Foundation (graph generation, knowledge graph), Decisioning Suite (scoring, workflows, dynamic exploration), **Agentic AI Suite** (Agent Gateway, Q Assist). "Business Purpose Solutions" on top: AML (Contextual Monitoring, intelligence-led investigations), KYC/CDD, fraud, correspondent banking AML, trade AML, retail AML, government FIU. | Quantexa G-Cloud 15 pricing document (Jan 2026); quantexa.com AML page |
| Cloud AML | **Quantexa Cloud AML**, general availability 16 Sept 2025 at ACAMS Las Vegas: native SaaS on Microsoft Azure for **US mid-size and community banks (USD 5bn+ assets)**; includes entity resolution, contextual monitoring, external data enrichment, customer risk rating, Q Assist copilot, 314(a)/314(b), AI-augmented SAR/CTR reporting. No named customers in the release. | Quantexa press release 16 Sept 2025 |
| Deployment | Annual subscription with a multi-year initial term, no limit on users, data sources or volumes. Listed deployment: "offline, on-prem, cloud"; recommended on Kubernetes/cloud-native services. **Software prerequisites (service definition): an object store (HDFS/ADLSv2/S3/GCS), Elasticsearch or OpenSearch, an RDBMS (Oracle/PostgreSQL/MySQL), Apache Spark (Cloudera/EMR/Dataproc/Databricks) and a container platform (Kubernetes/OpenShift/AKS/EKS/GKE).** | UK Digital Marketplace G-Cloud 15 listing, 2026 |
| Support | 24x7 from Melbourne, London, Boston, Toronto; severity-1 response 1 hour. | Same listing |
| Arabic and MENA | Offices in DIFC Dubai and Doha. At QuanCon (6 March 2025) Quantexa announced "Advanced Language Parsers: Now supporting non-Latin scripts such as Japanese, with **Chinese and Arabic capabilities on the horizon**". No release of Arabic parsing found as of 2026-10-06. No named Gulf or Iraqi bank customer found in public sources. | GlobeNewswire 6 Mar 2025; quantexa.com contact page |
| Analyst standing | Chartis RiskTech Quadrant 2025: Category Leader in AML Transaction Monitoring and in KYC (press release 19 Nov 2025); Chartis FCC50 2025: 7th overall; Chartis 2026 AML TM Quadrant update: Category Leader again, and 18th in the RiskTech100 2027 ranking with Trade Finance and Compliance category awards (Quantexa press page, 2026). Celent 2022 Model Risk Manager Award for the ABN AMRO deployment (as reported by Quantexa). | Quantexa/GlobeNewswire and quantexa.com press pages (vendor-published summaries of analyst reports) |

### 1.2 Detection approach and AI claims

- **Entity resolution first.** Everything else sits on a resolved "entity" (person, company, address, phone) built from internal data and external data
  (company registries, news). Matching is exact plus fuzzy; the vendor says its "open architecture and no fixed data model" make integration easier.
- **Graph and network analytics.** "Identify risk beyond a single transaction"; network views for investigators. This is the Wolfsberg-endorsed direction (file 04, section 2a: entity resolution and graph
  networks "now central").
- **Contextual Monitoring.** Quantexa's term for scoring behaviour in the context of the entity's network; marketed as producing "fewer, more
  accurate alerts" and usable on top of, or instead of, rule-based alerts from a legacy TM system.
- **GenAI and agents (2025-2026).** Q Assist (embedded copilot, natural-language queries over entity and graph data; early access April 2025);
  Agent Gateway and a "FinCrime Agent" for L1 alert triage and continuous due diligence (2025-26 marketing).
- It is an **analytics and context layer**, not a rules-and-regulatory-reporting TM engine. Typical deployments (HSBC, Standard Chartered,
  ABN AMRO, Danske) place Quantexa beside the bank's existing TM and case management or replace parts of them.

### 1.3 Published results and credibility

| Claim | Where it appears | Grade | Caveat |
|-------|------------------|-------|--------|
| "75% reduction in false positives", "80% reduction in investigation time (at scale)", "99% match rate between customer and external data" | quantexa.com AML page; repeated in Cloud AML launch | Vendor-reported | Aggregate headline with no customer, baseline, period or method; the Cloud AML launch pairs "75% fewer false positives and 80% faster investigations" with hedged wording elsewhere in the same text ("up to 75%", investigative effort cut "by over 50%"). |
| HSBC: "60% reduction of case volumes", "39m customers", "GBP 4m potential savings" | quantexa.com customer-story tile | Vendor-reported (HSBC is also an investor) | "Potential" savings; investor-customer relationship; no baseline. |
| Standard Chartered: 1,500 investigators, 1 billion transactions a year | quantexa.com | Vendor-reported | Describes scale, not an outcome. |
| "20% proven de-duplication of records", "99% entity resolution accuracy", "60 billion records managed" | quantexa.com | Vendor-reported | No test set or error-rate definition (precision/recall) published. |
| Chartis "Category Leader" 2025 | Chartis report (paywalled), press release | Analyst-reported | Chartis scores vendor capability, not outcomes. |

Nothing found in this research is an independent, method-disclosed measurement of Quantexa's detection uplift. The nearest to independent
evidence is HSBC's own disclosures, which relate to Google Cloud AML AI rather than Quantexa (section 8).

### 1.4 Deployment, explainability, governance

- Technically deployable on-premises, but **the stack is a big-data stack** (Spark, Elasticsearch/OpenSearch, object store, Kubernetes). For an Iraqi
  bank with one data centre and a small IT team this is a heavy footprint; no Iraqi-scale reference found.
- Explainability is the network itself: an analyst can see why entities are linked and which records support a link. Alert scoring explanations
  and model documentation are not described in the public material reviewed; Cloud AML release says nothing on explainability or model governance.
- Audit: platform keeps user activity audit for "at least 12 months" (listing).

### 1.5 Reviews and criticisms

- **Gartner Peer Insights** (Decision Intelligence Platforms category): 4.4 of 5 from 25 ratings (search-result summary; the Gartner page returned HTTP 403,
  not read directly). Praised: entity resolution, relationship analytics, scalability. Criticised: **expensive licensing, complex setup, needs
  skilled resources and significant onboarding effort**. G2: too few reviews to rate.
- Pricing signal (public sector list prices, G-Cloud 15, Jan 2026, "annual subscription, multi-year initial term, excludes third-party data"): Trusted Data Foundation
  GBP 7.74m (Tier 1a) down to GBP 3.655m (Tier 3, charities); Contextual Data Foundation GBP 2.52m to GBP 1.19m; Decisioning Suite GBP 3.78m to
  GBP 1.785m; **Agentic AI Suite GBP 4.32m to GBP 2.04m**; Data Modernisation GBP 1.18m to GBP 0.56m. Operational pilots (up to 12 months):
  GBP 250-500k (Tier 1a), 150-300k, 75-150k, 25-75k. The table does not state the period of the platform prices; the same document says licences are annual
  subscriptions. These are UK government tiers, not bank quotes, but they show the order of magnitude: **typically millions of pounds a year per suite**, plus
  integration partners (PwC and Accenture are listed implementation partners).
- Regulatory reception: no regulator endorsement or criticism of Quantexa found; it is used inside large regulated banks (HSBC, Standard
  Chartered, ABN AMRO, Danske).

### 1.6 What it does right (with evidence)

1. **Puts entity resolution and the network in front of the analyst.** Wolfsberg Part I names entity resolution and graph as central to effective monitoring (file 04);
   FATF/BCBS ask for a consolidated view of the customer relationship. HSBC, Standard Chartered and ABN AMRO use it for this.
2. **Open data model.** "No fixed data model" lets it ingest whatever the bank has: the same trait AKR-TMS needs for CBS-agnostic mapping.
3. **Scales.** "60 billion records" (vendor claim) with documented Spark/Kubernetes scaling and Prometheus metrics endpoints.
4. **Moves from alerts to context for the analyst**: the unit of work is the entity and its network, not the isolated alert.

### 1.7 What it does wrong (with evidence)

1. **Cost and weight.** GBP 0.6-7.7m per suite per year list prices (public sector), Spark + Elasticsearch + Kubernetes prerequisites, integration partners needed;
   Gartner Peer Insights reviewers cite expense, complex set-up and skills. Out of reach for most Iraqi banks.
2. **No Arabic.** Arabic parsing is "on the horizon" (March 2025); a platform whose core value is name/entity matching across data is weakest on exactly Iraq's hardest
   data problem (file 22).
3. **Not a regulatory-workflow product.** No goAML-style reporting, CBI scenario catalogue or Iraqi list handling; those are all outside its scope.
4. **Unverifiable headline claims.** 75/80/99% numbers have no baseline or method; the HSBC saving is "potential" and HSBC is an investor.
5. **Cloud AML shows the direction of travel to SaaS** (Azure, US community banks); the on-premises route is described as available but is not what the company markets.

### 1.8 Lessons for AKR-TMS

- **Copy:** entity resolution as a first-class service with explainable links ("why are these two records the same person": shared national ID, phone, family number,
  mother's name); network view in the case screen; analyst unit of work = customer network.
- **Avoid:** a Spark/Elasticsearch prerequisite stack. A PostgreSQL-centred design (graph tables plus recursive queries or an in-process graph) fits a bank with one rack.
- **Beat it:** Arabic-first entity resolution on Iraqi identifiers (four-part names, tribal names, mother's name, family number, Kurdish spellings, file 01 section 7), a
  price a mid-size Iraqi bank can pay, and a published match-quality test set (precision/recall with the bank's own sample).

### 1.9 AKR-TMS hooks

- Entity-resolution service with a **match explanation record** (rules and features that linked each pair, confidence, reviewer override) and a measured
  precision/recall report per bank (VB-01).
- Network view built from the read-only CBS replica (shared identifiers, counterparties, beneficiaries) with no extra big-data stack (VB-02).
- Show analysts the **entity** (customer plus linked parties) with all open alerts on it, not alert-by-alert queues (VB-03).

---

## 2. ThetaRay (unsupervised "Cognitive AI" for cross-border payments, Israel)

### 2.1 Snapshot

| Item | Finding | Source and grade |
|------|---------|------------------|
| Ownership and funding | Private, Israeli (Hod HaSharon; offices New York, London, Madrid, Dubai DIFC). Co-founder and Chief Academic Officer: Prof. Amir Averbuch. CEO **Brad Levy** (former Symphony CEO; replaced Mark Gazit, the CEO at the 2022 Mashreq deal). Latest round found: **USD 57m on 5 Sept 2023**, led by Portage with JVP and OurCrowd (CB Insights summary; the Business Wire release returned HTTP 403). Reported lifetime funding differs by aggregator (USD 148m to USD 220m): **not verified**. | ThetaRay about page; CB Insights snippet; aggregators |
| Acquisition | **Screena** (Luxembourg, founded 2020; cloud sanctions-screening engine; partner since April 2022) acquired, announced July 2024, terms undisclosed. | AML Intelligence / Crowdfund Insider (July 2024) |
| Product | Transaction Monitoring ("Cognitive AI", formerly SONAR), Transaction and Customer Screening (Screena), Customer Risk Assessment, and **RAY**, an "agentic AI AML investigation suite" inside the Investigation Center (2026). Also a self-service **Rule Builder and Simulator**. | thetaray.com, May-Oct 2026 |
| Scale claims | "100+ institutional customers", "15 billion trusted transactions annually", "USD 20+ trillion total transactions monitored", "+30% more suspicious activity uncovered". | thetaray.com about page (vendor) |
| Customers named | Santander (retail, CIB, correspondent banking), Mashreq Bank (UAE, correspondent banking, announced 14 Feb 2022, "our first customer in the UAE"), ClearBank (UK), Payoneer, Shift4, Onafriq, PAPSS (Pan-African Payment and Settlement System, March 2024), Travelex, Novatti, Now Money (Dubai fintech), a Miami retail bank (unnamed). | Press releases; vendor case studies |
| Deployment | **SaaS, cloud-native; sold through Microsoft Azure** ("Microsoft Azure partners with ThetaRay to deliver SaaS monitoring solutions"). Company description: "Our SaaS solutions ...". No on-premises or private-cloud option found in any public source. | thetaray.com; PR Newswire 2022 |
| Arabic and MENA | DIFC Dubai office; Mashreq reference. No public evidence of Arabic UI, Arabic name matching in Screena, or any Iraqi customer. | thetaray.com |
| Pricing | No public pricing. Third-party aggregator comments "fair for value, not the cheapest" (low-quality source). | rfp.wiki aggregator |
| Reviews | G2 4.2 (10 reviews); Gartner Peer Insights 4.7 (2 reviews), both via an aggregator summary: **too few reviews to be informative**. Awards 2026: Datos Insights "Silver Medalist" (Fraud & AML Impact Awards, 10 Sept 2026), Finovate Best RegTech (17 Sept 2026), CNBC Top Fintech Companies (23 July 2026). | thetaray.com news page |
| Regulatory reception | The vendor says a central bank approved the approach at Santander and that it contributes as an expert to FATF discussions (both vendor claims); no regulator statement found. Israeli origin is a legal bar in Iraq (below). | ThetaRay case study; thetaray.com |

**Procurement blocker specific to Iraq.** On 26 May 2022 the Iraqi parliament passed the "Law Criminalizing Normalization and Establishment of Relations with the
Zionist Entity" (275 of 329 members voting in favour, per AP/Al-Monitor), banning "diplomatic, political, military, economic, cultural and any other types of
relations" with Israel for Iraqis, state bodies, private companies and foreign companies, with penalties up to life imprisonment or death. A bank buying and running
software from an Israeli-headquartered vendor (ThetaRay; also NICE Actimize, file 08) would face at least serious legal and reputational exposure. This is a
procurement fact, not a legal opinion; Iraqi counsel must confirm. It removes Israeli-headquartered vendors from the realistic competitor set in Iraq and is a
selling point for a local product.

### 2.2 Detection approach and AI claims

- **Unsupervised, label-free anomaly detection** ("Cognitive AI"): ThetaRay marketing (as quoted in a search-result summary) says its AI "does not require data labeling or long ML training times". The algorithm family
  is attributed to the co-founder's academic work on high-dimensional data analysis (general knowledge, not confirmed on the pages read); details are not published.
- Each alert carries **"30+ weighted risk indicators"** with visual contribution, so an analyst sees why the alert fired (Shift4 case study, May 2026).
- **"Pseudo customer"** concept: risk detection for third parties inside a transaction (for correspondent banking, merchants), and profiling by merchant type.
- Network analysis of money flow between entities; alert suppression and de-duplication; weekly tuning syncs with the vendor.
- 2025-26 additions: Rule Builder and Simulator (so rules complement the model: consistent with the Wolfsberg hybrid view, file 04 section 2b), RAY (L1 alert investigation
  report: gathers data, evaluates customer and counterparty behaviour, "checks KYC/profile alignment", assesses jurisdictions, finds "contradictions and anomalies", correlates historical
  alerts, recommends next steps; on-demand assistant that summarises documents and does "controlled open-source scanning". "Analysts remain in control.").

### 2.3 Published results and credibility

| Claim | Source | Grade | Caveat |
|-------|--------|-------|--------|
| **Shift4** (payments, Europe; ThetaRay live Q1 2024): "80% reduction in total alert volume", "70% increase in productive alerts", "**86% reduction in false positive rate**" (from an 89% false positive rate), investigation time "from hours to minutes", same-day SARs in Malta | ThetaRay case study PDF, May 2026 | Vendor-reported, named customer, MLRO quoted | "Productive" and "false positive" not defined; the old system's tuning state is unknown; "in 7 months"; no recall/missed-case measurement. |
| **Miami retail bank**: "45% total alert reduction", "70% reduction in false positives" | Case study PDF, 2026 | Vendor-reported, customer anonymous | Unnamed; no baseline. |
| **Santander** (~USD 1.9tn assets, 3,000+ branches): "in some countries ... alerts declined by over 500,000 per year"; found "unknown unknowns"; gained Central Bank approval | Case study PDF (2024) | Vendor-reported | No percentage, no baseline, no count of new true positives. "Central Bank approval" is the vendor's wording. |
| **Onafriq** (Pan-African payment network): deployment "in under 3 months"; "83% of sender alerts and 94% of receiver alerts ... were deemed detection-worthy" | thetaray.com TM page | Vendor-reported | A relevance judgement by the customer, not a confirmed-suspicious rate. |
| "Up to 99% reduction in false positives" and "up to 90%" | Marketing | Vendor-reported | "Up to"; no method. |

No independent (regulator, academic or auditor) evaluation of ThetaRay detection performance was found. Its numbers are consistent in direction with other vendors but not comparable.

### 2.4 Explainability and governance

Strength: weighted indicators per alert, "automatically documenting detection logic", "audit trails". Weakness: the model internals are proprietary; for a bank that must justify the model to its
regulator (BCBS: parameters must be bank-specific and testable; file 04 section 1b), the unsupervised score needs a typology-to-feature map that the bank can read. ThetaRay says it participates as an
expert contributor in FATF discussions (vendor claim).

### 2.5 What it does right

1. **No labels needed.** Fits a market with few STRs (Iraq: 451 bank STRs in 2024, file 01 section 5): there is no label set to train a supervised model.
2. **Alert explanation as a first-class output** (weighted indicators, visual contribution, "pseudo customer" for counterparties).
3. **Fast time-to-value on historical data** ("POV in 2 days" case study title; Onafriq under 3 months; Shift4 live within about a quarter).
4. **Rules plus AI**: it added a self-service rule builder and simulator rather than claiming rules are dead.

### 2.6 What it does wrong

1. **SaaS on Azure only**: incompatible with the default Iraqi deployment (bank data centre, read-only replica).
2. **Israeli headquarters**: legally and reputationally unusable for Iraqi banks (section 2.1).
3. **Unverifiable headline claims** (86%, 99%, "up to"), no definition of "productive alert", no missed-case (recall) data, no independent test.
4. **No Arabic and no CBI scenario catalogue**: an anomaly model alone does not cover the CBI's mandatory minimum scenario catalogue (file 27: 86 numbered scenarios plus product-specific indicators in the 2025 controls).
5. **Narrow origin**: cross-border payments, PSPs and correspondent banking; less evidence for domestic retail cash-heavy patterns typical of Iraq.

### 2.7 Lessons for AKR-TMS

- **Copy:** label-free anomaly detection as one layer; weighted indicators per alert; "pseudo customer" profiling for counterparties; rule simulator.
- **Avoid:** headline claims without definitions. Define "false positive", "productive alert" and "detection-worthy" in the product and publish them.
- **Beat it:** on-premises, Arabic-first, CBI scenario coverage built in, and a measurement report (alerts, escalations, STRs, missed cases) produced on the bank's own data.

### 2.8 AKR-TMS hooks

- Unsupervised anomaly layer that needs no labels, ships with a **typology-to-feature map** and reason codes per alert (VB-04).
- Counterparty ("pseudo customer") profiles for non-customer parties seen only in transactions (VB-05).
- Built-in definitions of alert outcome classes (false positive, productive, escalated, STR filed, missed) and a per-bank effectiveness report (VB-06).
- Rule simulator on historical data before any rule or threshold goes live (VB-07).

---

## 3. Hawk (Hawk AI, Munich)

### 3.1 Snapshot

| Item | Finding | Source and grade |
|------|---------|------------------|
| Ownership and funding | Private, Munich (CEO and co-founder Tobias Schweiger). **Series C, April 2025: USD 56m (EUR 52m)** led by One Peak with Macquarie Capital, Rabobank, BlackFin Capital Partners, Sands Capital, DN Capital, Picus and Coalition (earlier: Series A USD 10m 2021; Series B extension with Rabobank). Funds for product and US expansion. | Startbase, Munich Startup, IBS Intelligence (April 2025) |
| Customers | "More than 80" customers: Tier 1 to mid-size banks, payment firms, fintechs. Named: **Commerzbank (announced 10 March 2026)**, Ecobank, VakifBank International (Vienna), Worldline, Volt, Vodafone, VR Payment, Ebury, Alviere. | Hawk press pages; funding coverage |
| Product | Unified FinCrime platform: Transaction Monitoring, Customer Risk Rating, **AML AI Overlay**, **AML Investigative Agent (launched 25 March 2026)**, CDD agent, customer and payment screening (plus screening agents), fraud and FRAML, Production Sandbox, Analytics Studio (AI lifecycle management), unified case manager, SAR/CTR filing. | hawk.ai, Oct 2026 |
| Deployment | Home page: "Choose from **SaaS or private cloud** deployment". AI Overlay page: "**Virtual private cloud (VPC), Hawk cloud, and on-premise** deployment options". Core platform: multi-tenant SaaS on Kubernetes, Spring Boot/Cloud and Apache Kafka, multi-cloud via infrastructure as code (five AWS regions per its AWS Startups profile). | hawk.ai tech-stack and overlay pages |
| Arabic and MENA | No Arabic UI or Arabic name-matching evidence found. No Gulf or Iraqi bank customer found. Early (2021) coverage mentioned plans to expand to Singapore and the UAE. | Search; siliconcanals |
| Analyst standing | Recognised in a Forrester "Financial Crime Management Solutions Landscape" report; Datos Insights innovation award for the AI Overlay; Celent solution profile. | Hawk news pages (vendor-published) |
| Reviews | G2 and Capterra pages blocked (HTTP 403 / Cloudflare); an aggregator summary says review depth is too thin to be a trust signal and mentions integration complexity with legacy core banking, documentation gaps for custom models, and premium pricing. | rfp.wiki aggregator (low quality) |
| Pricing | No public pricing; "premium" per aggregator. | - |
| Regulatory reception | Commerzbank says its software validation now includes AI model governance (10 March 2026); no regulator statement found. | Hawk press release |

### 3.2 Detection approach and AI claims

- **"Fuse rules with AI."** Rules (self-serve templates for structuring, layering, smurfing) plus anomaly detection plus a false-positive-reduction model. Multi-layer: "known and unknown threats".
- **AI overlay**: the model analyses **all transactions, "not just those flagged by your underlying system"**, and returns two scores per case: *false-positive likelihood* and *suspicious-behaviour likelihood*,
  with an explanation "highlighting transaction elements that both deviate from and align with the entity's expected behavior". Model types named on the page: **XGBoost, CNNs, VAEs and "Big Bird" LLMs**; models are
  "trained for you by Hawk data scientists" on a feature library; retrain in under one day; 30,000 TPS.
- **Production Sandbox** and self-serve rule set-up: test a rule on production data and deploy "in hours".
- **AML Investigative Agent (March 2026)**: gathers data, summarises, identifies typologies, drafts SAR narratives; "human-in-the-loop controls that pause execution at defined steps"; "chain of thought
  visualization, confidence scores, citations and detailed action logs"; "model-agnostic, modular overlay" that works with an existing case manager.
- **Governance of agents**: Hawk and Oliver Wyman published (2026) a 34-page framework extending OCC, Wolfsberg and FATF principles to agentic systems: dual-lens materiality (inherent risk vs design-controlled
  factors such as autonomy and tool access), an "agent assurance layer" (register, test, observe, challenge, constrain, stop every material agent) and a "decision-rights ladder" mapping autonomy to human approval.

### 3.3 Published results and credibility

| Claim | Source | Grade | Caveat |
|-------|--------|-------|--------|
| "3-5x more threats identified", "70% less false alerts", "55% less payments wrongly blocked for sanctions", "62% reduction in AML investigation time", "30% more fraudulent customers caught" | hawk.ai home page | Vendor-reported | Aggregate headline; no baseline, customers or period. |
| **Tier 1 bank, AI Overlay, 3 months**: "+88% prediction accuracy", "-85% false positive reduction", "2x higher threat detection" | AI Overlay page | Vendor-reported, anonymous | "Prediction accuracy" is not defined; no recall data. |
| **Ebury** (case study dated 9 Sept 2026): **46% initial false-positive reduction, 60% after model retraining; actionable alerts 14% to 30%**; rule deployment from weeks to hours | Hawk case study; named SVP quoted | Vendor-reported, named customer | "Actionable" not defined; a payments/FX firm, not a retail bank. |
| **Alviere** (agent back-test on 3 months of historical alerts, four business lines): the agent "correctly escalated every confirmed true positive", "closed 98% of the alerts that human analysts had identified as false positives", and flagged additional alerts | Hawk case study | Vendor-reported, **back-test against analyst decisions** | Historical replay, not live; the "additional" alerts were not confirmed; analyst decisions are themselves the ground truth. |
| **Commerzbank** (10 March 2026): "increased the accuracy (targeting) of alerts and reduced false positives", "detected more novel cases", "expanded software validation to include robust AI model governance" | Hawk press release with bank quotes | Customer-quoted, **no numbers** | A Tier 1 bank gave qualitative statements only. |
| Chartis (Hawk-sponsored) survey: 85% expect to raise agentic AI investment in 2-3 years; 61% rank investigations as the top area; 21% already use agentic AI for investigations | Hawk press release | Analyst-reported, vendor-commissioned | Survey of intentions. |

### 3.4 Explainability and governance

Hawk's design is explicit about the Wolfsberg points (file 04 section 2b): per-alert explanations, "4-eye review and model approval processes", "automated model governance, including versioning, model QA, and validation",
performance monitoring and sampling for degradation, and an Analytics Studio for model lifecycle. Commerzbank's release says the explainable nature of the AI "plays a central role, particularly regarding regulatory approval".
Weakness: deep models (CNN, VAE) and LLMs enlarge the validation burden; the overlay needs labelled outcomes from the underlying system.

### 3.5 What it does right

1. **Overlay, not rip-and-replace.** Lowers adoption risk and fits the Wolfsberg "booster on top of rules" model (file 04, 2a).
2. **Rules stay in the bank's hands**: self-serve rules and a production sandbox (Ebury: weeks to hours) answer the complaint that vendor-controlled rule changes are slow.
3. **Two scores per case** (false-positive likelihood and suspicion likelihood) with human-readable explanation.
4. **Honest back-test design for the agent** (Alviere): replay on past alerts against analyst decisions, count missed true positives first.
5. **Agent governance published** with a recognised consultancy: decision-rights ladder, runtime assurance, kill switch.

### 3.6 What it does wrong

1. **Evidence is vendor-published and mostly anonymous**; even Commerzbank released no figures.
2. **Needs labels and an existing system.** The overlay is trained on the bank's alert outcomes. Iraq has few STRs (451 bank STRs in 2024) and weak legacy TM, so there are few labels to learn from.
3. **Vendor data scientists build the models** ("trained for you"), so the bank depends on the vendor for retraining and validation.
4. **Multi-tenant SaaS first**; on-premises exists for the overlay but the stack is Kubernetes + Kafka + Spring and not documented for air-gapped banks.
5. **No Arabic** evidence; no Iraqi/CBI scenario catalogue or goAML evidence.
6. **Enterprise pricing** (aggregator comment).

### 3.7 Lessons for AKR-TMS

- **Copy:** production sandbox with one-click "test this rule on last N months"; dual scoring; reason text per alert; versioned model governance with 4-eye approval; an agent decision-rights ladder.
- **Avoid:** designs that need a large labelled history or a vendor data-science team; claims like "prediction accuracy" without definitions.
- **Beat it:** label-light start (rules + unsupervised + peer-group deviation), then supervised models as the bank accumulates analyst outcomes; models the bank can retrain itself; on-premises and Arabic from day one.

### 3.8 AKR-TMS hooks

- **Production sandbox** replaying any rule or model change on the bank's own history, showing alert counts, overlap with current alerts, and missed known cases, before approval (extends VB-07).
- **Two-score alert**: priority = f(suspicion likelihood, false-positive likelihood) with textual reasons (VB-08).
- **Agent decision-rights ladder** in the product: each AI function (summary, triage, narrative draft, close) has a configured autonomy tier, a human-approval rule, a kill switch and full action logs (VB-09).
- **Back-test harness for any triage or investigation agent**: replay on past alerts, report missed true positives first, then false positives closed (VB-10).

---

## 4. Feedzai (RiskOps platform: fraud first, AML second)

### 4.1 Snapshot

| Item | Finding | Source and grade |
|------|---------|------------------|
| Ownership and funding | Private, Lisbon and San Mateo; co-founder and CEO Nuno Sebastiao. Rounds: Series B USD 17.5m (Oak HC/FT), Series C USD 50m, **USD 200m growth round led by KKR (2021)**, then **about USD 75m on 2 Oct 2025 at a valuation "of more than USD 2 billion"** (new: Lince Capital, Iberis Capital, Explorer Investments; renewed: Oxy Capital, Buenavista Equity Partners). Citi Ventures is an earlier strategic investor. | Feedzai press releases (vendor-published) |
| Acquisitions | Revelock (behavioural biometrics; the press release is in Feedzai's archive, year not verified); **Demyst** (data orchestration, April 2025, reported about USD 100m, Demyst's clients included ANZ, NAB and Standard Chartered; product became "Feedzai Orchestration"). | Feedzai press page; Australian press (search result) |
| Scale claims | "Safeguard more than one billion consumers and USD 9 trillion in payment volume a year"; "more than USD 2B in losses prevented and 20M+ analyst hours saved" (company-wide, mostly fraud). | Feedzai release 2 Oct 2025; Farol release (vendor) |
| AML product | **AML Transaction Monitoring** (20+ out-of-the-box scenarios, e.g. rapid movement of funds, large transactions, avoidance of reporting thresholds, dormant account activity, high-risk counterparties), ML **alert-prioritisation model**, Case Manager, **SAR Manager** (templates for US, Brazil, UK, France, Germany, Lithuania, Malaysia, South Africa), Watchlist Screening (with Neterium, a screening engine), "AML Suite" unifying TM, screening and case management. **No goAML template listed.** | feedzai.com AML pages, 2026 |
| Newer AI | **Farol** (24 Sept 2026): embedded agent in RiskOps Studio with skills Risk Strategy (finds rules that "generate noise without catching fraud"), Investigations, Knowledge, SAR Drafting; says it "operates within each financial institution's own environment". Feedzai IQ (network intelligence), TRUST framework for GenAI (2025). | Feedzai press release |
| Deployment | Marketing is cloud-first: "cloud-based, agile data-science environment"; Jack Henry builds a "cloud-native" platform on Feedzai's engine; **BTG Pactual (Brazil) "migrated from on-premises infrastructure to Feedzai on AWS"** (customer story), which shows on-premises delivery existed in the past. Whether on-premises is offered to new customers today **could not be verified**. | feedzai.com; Jack Henry release 25 Aug 2022; BTG Pactual story |
| Channel | **Embedded in a core-banking vendor's platform**: Jack Henry Financial Crimes Defender (announced 25 Aug 2022; Datos Impact Awards 2025 silver medal for best joint AML and fraud TM innovation). A dedicated "Core Banking Providers" page offers to "embed identity, fraud and AML protection". | Feedzai / Jack Henry |
| Arabic and MENA | Wio Bank (Abu Dhabi digital bank): **fraud** (transaction fraud, digital trust), not AML. Al Rajhi Bank Malaysia: fraud. Contact page lists London and Lisbon among offices; no Gulf office found. No Arabic UI or name-matching evidence. | Feedzai customer stories |
| Analyst standing | QKS SPARK Matrix 2023 Leader for AML; Liminal Link Index "Top 18 vendors for AML transaction monitoring"; Chartis RiskTech100 2025 best enterprise fraud solution. | Feedzai-published |
| Reviews, pricing | G2 and Gartner pages blocked (HTTP 403). No public pricing. | - |
| Regulatory reception | None found. A 2019 claim of SR 11-7 compliance is out of date (rescinded April 2026, file 05 section 1c). Analyst recognitions (QKS, Liminal, Chartis) are not regulatory. | Feedzai press releases |

### 4.2 Detection approach and AI claims

- **Rules plus a supervised prioritiser.** Scenarios generate alerts; an ML model "learning from past alert labels, case investigations, and SARs" scores and prioritises them.
- **Cold-start answer (2019 release):** "an iterative and supervised machine learning technique that leverages algorithms able to **query AML teams** to identify whether given transactions were
  associated with money laundering": active learning, to cope with "the lack of labeled data".
- **Segment-of-One profiles**: per-customer behavioural baselines instead of broad segments.
- **Genome link analysis** with "Genometries": automatic recognition of visual patterns such as mule accounts, triangle schemes and structuring (2019).
- **Risk Ledger (2018)**: a privacy-preserving, members-only consortium of pseudonymised, pre-aggregated profiles ("up to 5 months earlier" detection of repeat fraudsters: fraud-side claim).
- **Whitebox Explanations**: reason for each score, also for imported third-party models.

### 4.3 Published results and credibility

| Claim | Source | Grade | Caveat |
|-------|--------|-------|--------|
| "33% reduction in false alerts"; "48% lower TCO" versus "other leading solutions"; "5 clicks from alert view to SAR filing"; "3x faster time-to-market" | feedzai.com AML TM page | Vendor-reported | No baseline, customer or method; the TCO comparison is unexplained. The 33% is modest against other vendors' 70-90%, which is arguably more realistic. |
| Farol: "reduce alert handling times by 20%"; SAR drafting "up to 12x faster" | Farol release, 24 Sept 2026 | Vendor-reported | Fraud-side agent; one bank quote (SEB) is qualitative. |
| Lloyds Banking Group: ML "value ... well over GBP 200 million from a few years' investment" | RiskOps page quote | Customer-quoted | Fraud, not AML. |
| Novobanco (Portugal, 4th largest bank, 1.7m customers; 6 March 2026): multi-year programme merging KYC, AML and fraud teams on one platform; Neterium screening gave "higher quality alerts with fewer false positives" | Feedzai release | Vendor release with bank quote | **No numbers.** Same pattern as Commerzbank/Hawk. |
| "80% fraud and scam detection", "90% reduction in mule activity", "90% faster model deployment" | RiskOps page | Vendor-reported | Fraud claims. |

No independent measurement of Feedzai AML detection was found.

### 4.4 Explainability and governance

Feedzai markets Whitebox Explanations, audit trails, challenger testing and backtesting, and said in 2019 that the AML solution "meets model governance requirements under OCC 2011-12 and SR 11-7". Those US rules were
rescinded on 17 April 2026 (file 05 section 1c), so the statement is dated: today the relevant claim would be proportionate governance for real models only.

### 4.5 What it does right

1. **Faces the label problem openly** and designs for it (active learning that asks analysts to label sampled alerts), which is the Iraqi situation too.
2. **One platform for fraud and AML** and one workspace (RiskOps Studio); Novobanco merges KYC, AML and fraud teams on it. Fraud and AML share data and signals (mules).
3. **Distribution through core-banking vendors** (Jack Henry): the channel that reaches mid-size banks without a large integration project.
4. **Self-serve rules and models with challenger testing and backtesting.**
5. **Candid, modest AML claim (33%)** compared with vendors quoting 70-99%.

### 4.6 What it does wrong

1. **AML is a secondary line**: 20+ scenarios, eight SAR templates, no goAML, no CBI catalogue, no Arabic.
2. **Fraud-heavy evidence base**: most named results are fraud; the AML customer evidence (Novobanco, Jack Henry) has no figures.
3. **Opaque deployment and price**; no public statement on on-premises delivery; no public pricing.
4. **Stale governance claim** (SR 11-7) and no independent validation published.
5. **Consortium model (Risk Ledger) needs many participants**, and cross-bank data sharing is legally difficult in Iraq (no precedent found).

### 4.7 Lessons for AKR-TMS

- **Copy:** active-learning labelling loop (a daily queue of sampled alerts and unalerted transactions for analysts to label), automatic typology recognition on the link graph, fraud/AML signal sharing (mule accounts), embedding through a core-banking vendor channel.
- **Avoid:** TCO and accuracy claims without a method; consortium designs that depend on cross-bank sharing that the law does not yet allow.
- **Beat it:** goAML-native reporting, CBI scenario catalogue, Arabic, and an on-premises package priced for Iraqi banks.

### 4.8 AKR-TMS hooks

- **Active-learning label queue**: sampled alerts, near-threshold non-alerts and branch referrals presented to analysts to label; labels feed supervised models and the effectiveness report (VB-11).
- **Typology pattern recogniser on the link graph** (mule/funnel, circular flow, structuring fan-in/fan-out) producing explainable pattern alerts (VB-12).
- **Shared fraud/AML signal layer** (mule-account flags usable by both) without requiring a fraud product (VB-13).

---

## 5. Featurespace (now part of Visa; ARIC Risk Hub became "The Featurespace Platform")

### 5.1 Snapshot

| Item | Finding | Source and grade |
|------|---------|------------------|
| Ownership | Founded in Cambridge (UK) by Dave Excell and Bill Fitzgerald from University of Cambridge engineering research; Betfair invested early; TSYS partnership 2016; Atlanta office 2017. **Acquired by Visa**: the founder's June 2026 article refers to "the announcement of Visa's acquisition" and the website now uses Visa branding and fonts. The announcement date and price are **not verified here** (the Visa release could not be located after the web-search budget ran out; my recollection is an announcement in September 2024, terms undisclosed). | featurespace.com "journey" article (page dated 9 June 2026) |
| Product | One platform: card fraud, payment fraud, scams (**Visa A2A Protect**), check fraud, application fraud, merchant-acquiring fraud, multi-tenancy for issuers and acquirers, and an **AML solution** ("brings together fraud and financial crime detection": KYC data in a unified customer view, day-one rules and models, alert routing by specialist team, case management). | featurespace.com solutions, sitemap May-July 2026 |
| Models | **Adaptive Behavioral Analytics (ABA)**: per-customer ("segment of one") real-time profiles; **Automated Deep Behavioral Networks (ADBN, 2021)**: recurrent neural network on sequences of transactions; **TallierLTM**, a "large transaction model": self-supervised, pre-trained on billions of transactions, aimed at data scientists, "improvement in fraud detection of up to 71% against industry standard models". | featurespace.com |
| Deployment | **"Deployed on premise or via fully hosted cloud in over 180 countries"** (home page). | featurespace.com |
| Scale claims | "100bn+ events processed per year"; "500m consumers protected"; "75% reduction in false positive rates". | featurespace.com (vendor) |
| Arabic and MENA | Akbank (Turkey) case study in sitemap; no Arabic or Gulf evidence found. | sitemap |
| Pricing, reviews | None public; review pages not reachable. | - |
| Regulatory reception | None found. Queen's Award for Innovation 2018 for the first-generation technology (founder article); NatWest named it a "strong partner" in its 2019 results. | featurespace.com |

### 5.2 Published results and credibility

| Claim | Source | Grade | Caveat |
|-------|--------|-------|--------|
| NatWest (since 2019): "135% improved value of scams detected", "57% improved value of fraud detected", "75% reduced false positives for scams"; debit card false positives "from 11:1 to 8:1"; fraud per case three to two; NPS above 70% | Featurespace case study ("NatWest data, 2025"); NatWest 2019 results release named Featurespace a "strong partner" | Vendor case study using customer data; customer named and quoted | **Fraud, not AML**; measured against NatWest's own incumbent; metric definitions given ("value detected"). Better evidence than most because the numbers are tied to customer data and a named bank. |
| Worldpay: "reduced fraudulent card transactions by 56%"; TSYS: "fraud losses reduced by 39%"; Danske Bank: "fraud response time from 10 minutes to 24 seconds" | featurespace.com | Vendor-reported | Fraud. |
| Eika Gruppen (alliance of 46 Norwegian banks): live in **five weeks**; phishing losses "reduced by almost 90% in 2024" | Case study (Eika data 2023-2024) | Vendor case study | Fraud; shows a shared platform for many small banks. |
| HSBC (AML): "The Featurespace AML solution better equips HSBC to deal with future AML challenges where product, distribution and operational complexities will grow, and manual processes can no longer keep up." (Global Head of AML Technology) | featurespace.com AML page | Customer quote, **no numbers** | The only AML customer evidence on the site. |
| Founder statement: ABA cut fraud losses 40% and blocked 80% fewer genuine transactions (2018 generation); ADBN found 38% more fraud with the same customer impact | "journey" article | Vendor-reported | Historic, fraud. |

No independent evidence of Featurespace AML detection performance was found. Its AML offer appears thin: one page, one HSBC quote, no AML numbers, and a site otherwise organised around fraud.

### 5.3 What it does right

1. **Per-customer real-time behavioural profiling** with "adaptive" models, measured on live bank data and tied to named banks (NatWest).
2. **On-premises or hosted** is stated publicly, unlike most challengers.
3. **Fast deployment for small banks via a shared model** (Eika: 46 banks, five weeks).
4. **Self-supervised pre-training on transactions** (TallierLTM) is a credible route when labels are scarce (see file 21 for the technology comparison).
5. **Clear metric definitions** in the NatWest case (value detected, false-positive ratio, fraud per case).

### 5.4 What it does wrong

1. **Fraud first.** The AML offer is "configured to detect fraud and financial crime", with no AML-specific numbers, no AML regulatory reporting, no scenario catalogue.
2. **Owned by a card network.** Roadmap and neutrality follow Visa's priorities (A2A Protect, scams). For an Iraqi bank, a US-owned vendor also adds US-sanctions exposure (OFAC rules apply to the vendor; file 05 section 1e).
3. **Deep-network opacity.** RNN and large-transaction models are harder to explain to a regulator than rules or gradient-boosted trees, which Wolfsberg flags (file 04 section 2b).
4. **No Arabic, no Iraq references, no price transparency.**

### 5.5 Lessons for AKR-TMS

- **Copy:** per-customer behavioural baselines (for Phase 2 real-time scoring, response in milliseconds); case studies that define metrics and use the customer's own data; shared multi-bank model with per-bank calibration (Eika shows demand among many small banks).
- **Avoid:** deep sequence models as the primary AML detector in a market with no labels and no validators; the card-network ownership problem (AKR-TMS stays neutral).
- **Beat it:** AML-first scenario coverage, reporting and Arabic; behavioural baselines computed from the bank's own CBS history in days.

### 5.6 AKR-TMS hooks

- **Per-customer behavioural baseline store** (rolling profiles per customer, account, counterparty and product) usable offline (Phase 1 alerts) and online (Phase 2 scoring API within a latency budget) (VB-14).
- **Shared-model, per-bank calibration**: one validated model family, per-bank thresholds and calibration reports (links to Wolfsberg "validate once, then delta", file 04 section 2b) (VB-15).

---

## 6. Tookitaki (FinCense; AFC Ecosystem typology sharing)

### 6.1 Snapshot

| Item | Finding | Source and grade |
|------|---------|------------------|
| Ownership and funding | Private, Singapore (offices in Singapore, India and the US per its 2019 post). Founder and Executive Chairman Abhishek Chatterjee (ex-JPMorgan); **CEO Isabel Ridad** (ex-Standard Chartered, MUFG, Societe Generale, Maybank); co-founder and COO Jeeta Bandopadhyay. **Series A closed at USD 19.2m on 25 Nov 2019** (co-led by Viola Fintech and SIG Asia, with Nomura; earlier investors Illuminate Financial, Jungle Ventures, SEEDs Capital). Later rounds **not verified** here. | Tookitaki blog 25 Nov 2019; about page |
| Product | **FinCense**: Transaction Monitoring, Screening (name and payments), Onboarding Risk Suite, Customer Risk Scoring, **Alert Prioritization AI Agent**, Case Manager, fraud prevention; **AFC Ecosystem** (community typology library). "Trust Layer" positioning in 2026. | tookitaki.com, Oct 2026 |
| Scale claims | "100B+ transactions monitored", "200M+ customer accounts", "30+ APAC financial institution clients", "400+ typologies in the AFC Ecosystem", "3X annual scenario growth", "90%+ high alerts to STR cases". | tookitaki.com (vendor) |
| Customers named | UOB (Singapore), Maya (Philippines digital bank), GCash (event page only), Thunes alliance; four anonymised case studies (universal bank, digital bank, payment processor, e-wallet). APAC-centred: "intelligence ecosystem for APAC financial institutions". | tookitaki.com |
| Deployment | **Not stated on the public pages reviewed** (TM page lists no deployment options). SOC 2 certified. The company attends Cloudera hybrid-cloud and HPE events in Taiwan, so hybrid is plausible but **unconfirmed**. | tookitaki.com security page and events |
| Throughput | TM page: "real-time throughput of 50+ TPS with sub-second response" (a low figure next to Hawk's 30,000 TPS claim; adequate for a mid-size bank). | tookitaki.com TM page |
| Arabic and MENA | Many UAE and Saudi blog posts and ebooks ("Top AML Scenarios: Saudi Arabia", "Seamless Saudi Arabia 2022" event) but no Gulf customer named and no Arabic UI/matching evidence found. | sitemap |
| Analyst standing | "Tookitaki Named Category Leader by Chartis" (report not identified on the page). | tookitaki.com banner |
| Reviews, pricing | Not found (review sites blocked); no public pricing. | - |
| Regulatory reception | FATF's 2021 new-technologies report includes an unnamed "Smart Alert Management" case that matches the UOB description (contributed by the private sector, not verified); no regulator endorsement found. | FATF 2021 Annex D; Tookitaki |

### 6.2 Detection approach and AI claims

- **Hybrid detection**: "behavioral + typology-based" detection; **supervised plus unsupervised** models; automated model management that builds a model "trained on client data" (a bespoke model, not a generic one);
  **champion-challenger** framework; "glass-box" Explainable AI framework "under patent review" outputting human-readable reasons (UOB case study).
- **Scenario-as-content**: typologies from the AFC Ecosystem arrive as deployable scenarios ("deploy in hours, not weeks", "50% faster scenario deployment"); a **Typology/Scenario Design Studio** (no-code, drag-and-drop) and a **simulation engine** test them before go-live.
- **Alert Prioritization AI Agent**: ensemble ML that scores, triages and ranks alerts using "historical behavior, risk context, and real-world outcomes"; can run on top of an existing TM through "standardized data schema and adapters" (UOB).
- **AFC Ecosystem** (how it really works, from Tookitaki's own 2023 technical post): experts from banks, advisory firms, law firms and NGOs (the "AFC Network", free and invite-only) design a typology in the Typology Design Studio using "standardized programmatic tokens that
  encapsulate the modus operandi of financial crimes **without sharing any PII**"; once approved, a **Federated Insights Card** is generated and stored in a **Federated Knowledge Base**; cards can be downloaded and deployed by members. The text calls this
  "federated learning", but what is described is **expert-validated typology sharing, not training one model across banks' data**. The TM page separately lists a "Federated Learning Model" without technical detail.

### 6.3 Published results and credibility

| Claim | Source | Grade | Caveat |
|-------|--------|-------|--------|
| **UOB (Singapore), AMLS Smart Alert Management on TM and name screening**: baseline "90-95% false alert rate"; result "**reduced false positives by 50%-70%**", "**5% increase in true positives**", "**70% reduction in false positives for individual names**", risk classification L1-L3 "accuracy rate exceeding 85%" | Tookitaki case study page | Vendor-hosted case study naming the bank (UOB's own text not found) | Measured during COVID on data with "high defensive reporting"; accuracy of the bucket classification is against the bank's own past dispositions, so it inherits their errors; a 5% true-positive gain is modest. |
| Other case studies: Singapore bank (anonymous) "50% reduction in false positives", "~45% reduction in overall compliance cost"; payment services "90% reduction in false positives", "70% reduction in effort on threshold tuning"; digital bank "100% risk coverage"; e-wallet "90% accuracy in high-quality alerts" | tookitaki.com | Vendor-reported | The same product is credited with 50%, 70%, 80% and 90% false-positive reductions on different pages; **numbers vary by page**, suggesting marketing figures rather than one measured result. |
| Alert Prioritization: "80%+ reduction in false positives", "95%+ true cases identified with 20% effort", "lower compliance cost by 30%" | Agent page | Vendor-reported | "95% of true cases with 20% of effort" is a gains-chart claim; no data set, period or definition of "true case". |
| "90%+ high alerts to STR cases" | Home page | Vendor-reported | Undefined: could mean that 90% of "high" priority alerts become STR cases; if so it measures prioritisation against the bank's own filing behaviour (defensive filing risk, file 04 section 2a). |

No independent measurement was found. The UOB case is the best-documented because the bank is named and the baseline (90-95% false alerts) is stated.

### 6.4 Explainability and governance

Strengths: champion-challenger, automated model management, human-readable reasons, SOC 2. Gaps: "federated" claims have no public technical paper; no model card, validation pack or bias testing published; the typology community's quality control is described (expert approval) but not measured.

### 6.5 What it does right

1. **Scenarios as shareable, versioned content** with a design studio and simulator. This matches how regulators (the CBI's scenario catalogue, file 27) and Wolfsberg think: typology to rule to coverage. It speeds new-scenario rollout to hours for many banks.
2. **Works as an overlay on a legacy system**: adapters and a standard data schema (UOB), so a bank can start with alert prioritisation before replacing anything.
3. **Champion-challenger and per-client models** built into the platform.
4. **Candid about the false-alert baseline** (90-95% at UOB), which matches the Wolfsberg diagnosis.

### 6.6 What it does wrong

1. **Inconsistent, undefined headline numbers** (50/70/80/90% for the same effect; "90%+ high alerts to STR cases").
2. **"Federated learning" is a stretched label**: the shared object is an expert-written typology, not model parameters; no evidence that bank data improves other banks' models.
3. **APAC-centred typologies**: nothing on hawala, Iraqi cash-border flows, Islamic finance products, Iraqi PEPs or the CBI catalogue.
4. **Unspecified deployment**, low stated TPS, no Arabic, no price transparency.
5. **Typology community needs critical mass** and legal comfort; in Iraq banking secrecy, the absence of a personal-data law and tipping-off rules make an invite-only sharing network untested (file 01 sections 2d and 8).

### 6.7 Lessons for AKR-TMS

- **Copy:** scenario cards (PII-free, structured: typology, indicators, required data fields, parameters, test results, regulatory mapping); a design studio plus simulation; per-bank enable/disable and parameter overrides over a central library.
- **Avoid:** calling typology sharing "federated learning"; publishing percentages without a definition and baseline.
- **Beat it:** ship the **CBI scenario catalogue (file 27) and Iraqi/Islamic typologies** as the founding library, owned and curated locally (with the Association of Private Iraqi Banks or the FIU as a possible sponsor), validated by Iraqi compliance officers, and delivered on-premises with signed offline updates.

### 6.8 AKR-TMS hooks

- **Scenario library as signed, versioned "scenario cards"**: ID, typology, CBI article, indicators, data fields, default parameters, back-test report, owner, version, retire-or-keep review date; updates delivered as signed offline bundles (VB-16).
- **No-code scenario studio with simulation** on the bank's history before activation, with approval workflow and rollback (extends VB-07) (VB-17).
- **Alert re-scoring mode** that can ingest alerts from a legacy tool (CSV/DB) and prioritise them while the bank moves to AKR-TMS detection (VB-18).

---

## 7. Consilient (federated learning for AML)

### 7.1 Snapshot

| Item | Finding | Source and grade |
|------|---------|------------------|
| Ownership and funding | Private, US (Washington DC dateline). Founded through a partnership between **K2 Integrity** (Juan Zarate, chair) and **Giant Oak** (Gary Shiffman, CEO and co-founder at the Feb 2023 release); **USD 3m private seed** announced May 2022; CEO Ajit Tharakan by 2024. No later round found. Small team (leadership page lists a handful of staff; no open roles). | Consilient press release 28 Feb 2023; about page |
| Product | Federated-learning platform and a **model library** (18 models at 28 Feb 2023): Core AML/CFT model (risk score from 12 months of behaviour, XGBoost), Correspondent Banking model, High-Risk Typology models (e.g. MSBs, precious-metal dealers), High-Risk Jurisdictions model, KYC/AML Risk Rating model. | consilient.com products pages |
| How it works (vendor description) | "**Algorithms travel and insights are shared, but data never leaves its environment**": models are trained locally inside each institution, the updates are aggregated and validated into a "champion" model that is delivered back; ensembling combines local and global models. Trained on high-risk behaviours plus SARs because "many SARs are filed defensively". | Press releases 2023 and 2024 |
| Customers | **None named.** Partners: Secura/Isaac Group (advisory firm led by a former FDIC chairman, 2024; the published release still carries the placeholder date "June TK, 2024"), Sigma360 (integration for pKYC and transaction risk), Harex InfoTech (South Korea). The High-Risk Jurisdictions page says its model is built from "over 20 participating banks in high-risk jurisdictions" (not named). | Press pages |
| Deployment | Models run inside the bank's environment, so on-premises is natural; the federation needs a link to the aggregator. No public deployment guide. | Vendor text |
| Arabic and MENA | None found. | - |
| Pricing, reviews, analysts | None found. | - |
| Regulatory reception | A partner (Secura/Isaac Group) says the solution "has impressed regulators"; no regulator is named; none found otherwise. | Secura/Isaac release |

### 7.2 Published results and credibility

| Claim | Source | Grade | Caveat |
|-------|--------|-------|--------|
| Core model: "**88% reduction in false positive alerts**"; home page: "reduces false positive alerts by 75%" and "300% improved identification of high-risk customers" | consilient.com | Vendor-reported, no customer, no baseline | Two different headline numbers; "realized alongside standard (heuristic) transaction monitoring rules, with no need for additional investigations" suggests it was scored against existing alerts. |
| Correspondent banking model: "over a **76%** reduction in false positive alerts from beneficiary and counter-parties", "**244%** increase in detecting suspicious activities", "estimated to reduce annual review costs by **92%** in a recent pilot" | consilient.com | Vendor-reported pilot | "Estimated"; unnamed pilot. |
| High-Risk Jurisdictions model: built from "over 20 banks", "thousands of transactions", "40% manually reviewed" in one paragraph and "over 50%" in another | consilient.com | Vendor-reported | Internal inconsistency; very small data for "most comprehensive dataset available". |

**The only independent evidence on this architecture** is the BIS Innovation Hub's **Project Aurora** (May 2023), a proof of concept on a **synthetic** data set (see section 8.3): the
fully decentralised federated arrangement (CAL 4) "detected twice as many money launderers" and cut false positives by about 40% against rule-based monitoring, but "was not as effective as other CAL arrangements in detecting
the full money laundering network since the underlying transaction data are not shared"; the **centralised cross-border** arrangement detected three times more and cut false positives by 80%. The BIS also warns that federated learning "could be vulnerable to data leakage
attacks, where the original data could be reconstructed from the model updates". Consilient does not appear anywhere in the Aurora report (full text searched); the technology contributor named in its acknowledgements is Lucinity (file 13 section 1).

### 7.3 What it does right

1. **Names the label problem correctly**: SARs are a poor label because of defensive filing (matches Wolfsberg, file 04 section 2a), and models are built on behaviours and typologies.
2. **Keeps data local by design**, which is the right instinct where data sharing is legally blocked.
3. **Uses a simple, auditable model family** (XGBoost with feature importance) rather than deep nets, and states that explanations list the key factors behind a flag.
4. **Model library by typology** (MSBs, jurisdictions, correspondent banking) is easier to explain and map to regulation than one monolithic score.

### 7.4 What it does wrong

1. **No named customer and no independent test**; the headline numbers conflict (88% vs 75%; 40% vs 50% reviewed).
2. **Value depends on a network of participants** that does not exist in Iraq, and the legal basis for cross-bank model updates is untested (banking secrecy, no personal data law, file 01 sections 2d and 8).
3. **Federated is not the best collaborative design** according to the only independent test (Aurora): decentralised was the weakest of the collaborative options for network detection.
4. **Leakage and poisoning risk** of model updates is not addressed on the site (secure aggregation, differential privacy are not mentioned).
5. **Tiny vendor and thin product surface**: models, no case management, screening, reporting or Arabic.

### 7.5 Lessons for AKR-TMS

- **Copy:** train on behaviours and typologies, not only on STR labels; an explicit typology model library; local-first execution.
- **Avoid:** building the product around a cross-bank network. Start with strong single-bank analytics. Keep the design **federation-ready** (model-update export, secure aggregation, signed model packages) so a CBI-sponsored or association-sponsored pilot can be added later.
- **Beat it:** deliver the whole workflow, not just a model; document the leakage and poisoning controls.

### 7.6 AKR-TMS hooks

- **Federation-ready model packaging**: models exported and imported as signed packages with metadata (training window, features, metrics, data lineage), so a later cross-bank or FIU-hosted pilot can use secure aggregation without redesign (VB-19).
- **Label policy**: STR outcomes are one weak label; analyst dispositions, branch referrals, FIU and law-enforcement requests, and typology flags are separate label sources with their own reliability weights (VB-20).

---

## 8. Published bank results from AI in AML: the numbers, who reported them, and the caveats

Purpose: give the plan a defensible benchmark for what AI can and cannot deliver, and show how little independent evidence exists. Cross-reference: Wolfsberg Part I and II on metrics and
validation (file 04 section 2a-2b); technology comparison (file 21).

### 8.1 How to read these numbers

| Issue | Why it matters |
|-------|----------------|
| **"False positive" is not defined the same way twice.** | It can mean alerts closed without escalation, alerts that never became an STR, or alerts a reviewer called "not relevant". Defensive filing and over-tuned rules change the baseline (Wolfsberg: "no SAR left behind"). |
| **Baseline.** | Savings against an untuned legacy system (90-95% false alerts, a figure cited by BIS, UOB's case, Hawk's page) are larger than savings against a tuned one. |
| **Recall is rarely reported.** | A 90% alert cut with an unknown miss rate is not a result. Only HSBC, BIS Aurora and HKMA Bank C give a detection-side number. |
| **Same label = same bias.** | Models trained on past alert dispositions inherit analysts' errors and defensive filing; they can only be as good as past decisions. |
| **Who measured.** | Bank-reported with a stated method (HSBC) or regulator-compiled (HKMA, BIS) is better than vendor-reported; vendor-reported and anonymous is the weakest. |

### 8.2 HSBC with Google Cloud "AML AI" (customer-reported, method disclosed)

Source: Richard D. May (Group Head of Financial Crime, Global Banking & Markets and Commercial Banking, HSBC), Google Cloud blog, **30 November 2023**.

- Scale: HSBC screens "over 1.2 billion transactions" a month.
- "**AML AI identifies two to four times as much suspicious activity as the previous system, while reducing the number of alerts by 60%**": "twice as much identified financial crime in our commercial banking operations and almost four times as much across our retail banking".
- Time: suspicious accounts detected "now down to eight days after the first alert".
- Method (footnote): "a comparison of the 12 months following go-live vs. the preceding 12 months of rules-based monitoring, accounting for seasonality and removing the first month of data after go-live to avoid influencing the result due to detection backlogs".
- HSBC was named Celent Model Risk Manager of the Year 2023 for it; live in the UK and Hong Kong, "looking to distribute the technology across other markets".
- The model learns "suspicious activity on its own, without us telling it what to look for", trained on "our vast range of customer data".

Caveats: published on the vendor's blog; a before/after comparison, not a concurrent control; "suspicious activity" is not defined (confirmed, escalated or reported?); implied precision gain is large (two to four times more with 60% fewer alerts means roughly 5x to 10x higher yield per alert, my arithmetic); the label set (HSBC's investigation history) is far larger than any Iraqi bank's; Google AML AI is a **cloud-only** service on BigQuery, which cannot serve an Iraqi on-premises deployment (docs: 30 months of data, party-month scoring from Party, AccountPartyLink, Transaction and RiskCaseEvent tables, feature-family explanations, backtests, a governance-artifact pack including labeling and tuning methodology and an engine version support policy).
HSBC's earlier trade-finance deployment with Quantexa (25 Sept 2019: "over 50 different scenarios", 5.8 million trade transactions a year) gave no outcome numbers.

### 8.3 BIS Innovation Hub "Project Aurora" (independent, synthetic data, May 2023)

A proof of concept on a **synthetic** data set with embedded laundering patterns, comparing rule-based monitoring with five ML models (incl. logistic regression, isolation forest, graph neural network) under siloed, national and cross-border views, and four "collaborative analysis and learning" (CAL) arrangements.

| Finding | Number (from the report) |
|---------|--------------------------|
| Rule-based, siloed | detected "only up to 25% of the money launderers" |
| ML, siloed | "more than twice as many" |
| Graph neural network, cross-border view | "approximately 80% of the money launderers" |
| False positives vs siloed rule-based | about 40% fewer (ML, siloed); about 75% fewer with a national view |
| CAL 2 centralised cross-border | three times better detection and 80% fewer false positives vs rules with no CAL |
| CAL 3 hybrid | almost twice as many launderers, over 75% fewer false positives |
| CAL 4 fully decentralised (federated learning) | twice as many, 40% fewer false positives, but weaker on the full network because data are not shared |
| Supervised vs unsupervised | supervised models "better ... at reducing false positives"; labelling helps |

Caveats: synthetic data, a known set of embedded patterns, supervised models with complete labels (an advantage real banks lack); the BIS also warns that federated learning "could be vulnerable to data leakage attacks" and that encrypted data may only be pseudonymised, and asks for legal clarity before real use.

### 8.4 HKMA case studies (regulator-compiled, bank-reported, 2023)

Source: HKMA, "AML Regtech: Network Analytics" (May 2023) and "AML/CFT Regtech: Case Studies and Insights Volume 2" (September 2023); banks anonymised. **Bank letters are assigned separately in each paper**: "Bank A" and "Bank B" in the first two rows below come from the network-analytics paper; "Bank B", "Bank C" and "Bank D" in the ML rows come from Volume 2 (elsewhere in this research, "HKMA Bank B" means the Volume 2 ML-triage bank).

| Item | Number and detail |
|------|-------------------|
| Adoption | "Around 60% of retail banks in Hong Kong are now deploying network analytics, more than twice as many as three years ago" (2022). |
| Outcome of public-private sharing plus analytics | FMLIT member banks "increased their intelligence-led STRs filed by 319% compared to 2021, leading to an increase of 113% in the amount of criminal proceeds restrained or confiscated" (FMLIT: police, HKMA and 23 banks). |
| Bank A (in-house network tool) | "over 20 super STRs involving over 900 relationships" within six months of operation. |
| Bank B (in-house network tool) | disrupted a suspected laundering operation of over 20 individuals and over HK$2bn that rule-based TM would have struggled to find. |
| **Bank C (in-house ML alert prioritisation, PoC early 2023)** | "Investigators confirmed that the **80% of alerts scored as high-risk lead to an STR** being filed." |
| **Bank B (ML triage)** | "**7% of TM alerts are auto-closed**, subject to sample checking for quality assurance, and the daily workload for an investigator is reduced by 1 to 3 hours"; auto-discounted alerts stay "dormant" for 90 days and are monitored against pre-defined re-opening conditions; low re-opening rates are the confidence measure. |
| Bank D (vendor ML) | model trained on one year of historical TM alerts, six-month trial, long-term goal "auto-clear". |
| Data needs | all banks needed "6 months' to 2 years' of historical data comprising TM alerts, review results, and payment information". |
| Governance | Bank C runs **champion-challenger** in parallel; Bank B has a drift protocol (investigate, then retrain); explainability "built in from the start". |

Reading: **the regulator's own sample shows far smaller automation than vendor pages** (7% auto-closed, not 80-90%) and a precision-at-the-top measure (80% of top-scored alerts become STRs) that is simple, defensible and easy to reproduce. The largest measured gains come from **networks plus information sharing**, not from model sophistication.

### 8.5 UOB (Singapore) with Tookitaki (vendor-hosted)

Baseline "90-95% false alert rate across transaction monitoring and customer name screening"; result "false positives reduced by 50%-70%", "5% increase in true positives", "70% reduction in false positives for individual names" (details in section 6.3). FATF's 2021 new-technologies report reproduces this as an **unnamed** "Smart Alert Management" case in Annex D with benefits but no figures; the FATF case studies were contributed by the private sector and are not verified.

### 8.6 Other bank statements: qualitative only

| Bank (vendor) | What was published | Number? |
|---------------|--------------------|---------|
| Commerzbank (Hawk), 10 March 2026 | "increased the accuracy (targeting) of alerts and reduced false positives", "detected more novel cases", "expanded software validation to include robust AI model governance" | none |
| Novobanco (Feedzai), 6 March 2026 | merged KYC, AML and fraud teams on one platform; screening alerts of "higher quality ... fewer false positives" | none |
| Danske Bank (Quantexa), 17 Dec 2020 | pilot in 2018, then transaction monitoring on the markets trading business and financial-crime investigations; "high-quality alerts" | none (a Danske Bank card-fraud case with Featurespace states "fraud response time from 10 minutes to 24 seconds", fraud not AML) |
| Standard Chartered (Quantexa), 2019 | "billions of data points from more than 40 countries"; "consolidates information from multiple sources" | none (vendor tile: 1,500 investigators, 1bn transactions a year) |
| HSBC (Quantexa, trade finance), 25 Sept 2019 | "over 50 different scenarios", "more than double the average number of checks" | none |
| DBS | **no verifiable AML-AI result found** in the sources reachable for this research | not found |

Pattern: **large banks rarely publish numbers**; the figures with names attached come from vendor-hosted pages or from HSBC (the clear exception, with a stated method).
Not verified and therefore excluded: widely repeated figures for Danske Bank's deep-learning fraud model (reported by its vendor around 2018) and for DBS and JPMorgan AML analytics; the research session ran out of web-search budget before these could be traced to a primary source.

### 8.7 Collaboration reality check: Transaction Monitoring Netherlands (TMNL)

Five Dutch banks (ABN AMRO, ING, Rabobank, Triodos, de Volksbank) built TMNL to analyse a subset of information jointly "under current Dutch and EU law". On **1 July 2024** they announced it would **wind down its existing activities**: the new EU AMLR (applying mid-2027) "provides a broad legal basis for inter-bank cooperation", so TMNL would be redesigned around it. Lesson: even in a mature jurisdiction, cross-bank analytics needed a statute to scale; in Iraq (banking secrecy, no data-protection law) the same barrier is larger.

### 8.8 Synthesis: what to believe and what AKR-TMS should promise

| Evidence | Typical level | Use |
|----------|---------------|-----|
| Vendor pages | 70-99% fewer false positives, 3-5x more detection | Treat as upper bounds against untuned systems; do not copy. |
| Bank with stated method (HSBC) | alerts -60% with 2-4x more suspicious activity found | The most credible large-bank benchmark. |
| Independent PoC (BIS, synthetic) | ML finds 2x+ more than rules; false positives -40% siloed, -75% with a national view; supervised beats unsupervised | Direction, not a forecast. |
| Regulator sample (HKMA) | 7% auto-closed; 80% of top-scored alerts become STRs; network tools produce "super STRs" | Realistic operating targets for the first year. |
| Collaboration | large gains need legal basis and a network (Aurora CAL 2, FMLIT, TMNL) | Design federation-ready; do not depend on it. |

**Proposed AKR-TMS promise (to be validated on each bank's own data):** a back-tested alert volume and recall report before go-live; first-year targets such as 30-60% fewer alerts at equal or better recall on the bank's own labelled history, a defined precision-at-top-k (for example the share of the top-scored 10% of alerts that end as STR or confirmed suspicious), and a network view that produces investigation-ready "super STR" packages. Always report recall, baseline and definitions with any percentage.

### 8.9 AKR-TMS hooks (section 8)

- **Standard effectiveness metric set**, computed per scenario, per model and per bank: alerts, escalations, STRs, precision at top-k, auto-close volume with re-open rate, time to first action, recall on a labelled holdout, and baseline comparison with confidence intervals (VB-21).
- **Auto-close guardrails** modelled on HKMA Bank B: sample QA, a dormant period (default 90 days) with re-open conditions, and a drift protocol that stops auto-close if re-open rates rise (VB-22).
- **Champion-challenger in parallel** on live alerts without affecting operations (VB-23).
- **Pre-go-live back-test report** on the bank's own history (12-24 months), documented per Wolfsberg (no mandatory parallel run) and per HSBC's before/after method (VB-24).

---

## 9. What it means for AKR-TMS

1. **The market has converged on one architecture, so AKR-TMS should not bet on one technique.** Every vendor in this file, and HSBC's own system, combines (a) rules for known typologies and reporting obligations,
   (b) a model that prioritises or re-scores alerts, (c) an anomaly or behavioural layer for unknown patterns, (d) entity resolution and a network view, (e) explanations per alert, and (f) since 2025-26 an
   investigation agent with a human in the loop. This is exactly the Wolfsberg hybrid (file 04 section 2b). The differences are in packaging, evidence and deployment.
2. **Labels are the binding constraint in Iraq, and the vendors answer it in four ways**: unsupervised detection (ThetaRay), active learning that asks analysts to label (Feedzai), typology-based models and library content
   (Consilient, Tookitaki), and context that needs no labels (Quantexa). The only independent test (BIS Aurora) found supervised models better when labels exist. So AKR-TMS needs a **staged path**: CBI scenarios plus
   unsupervised and peer-deviation scores on day one; collect labels from the first week; add supervised models only after label-volume and quality gates are met (VB-27, VB-11, VB-20).
3. **Deployment is the clearest white space.** Of the seven: ThetaRay is SaaS on Azure; Quantexa can run on-premises but needs Spark, Elasticsearch and Kubernetes; Hawk offers on-premises only for the AI overlay;
   Featurespace states "on premise or fully hosted cloud"; Feedzai is cloud-first (a customer migrated from its on-premises version to Feedzai on AWS); Tookitaki and Consilient do not state it; Google's AML AI (HSBC's engine) is cloud-only. For the default Iraqi deployment (bank data centre,
   read-only replica, no personal-data law, cloud rules still anticipated, file 01 section 8) a product that runs on a PostgreSQL-centred stack in one rack is a real differentiator (VB-25).
4. **Legal and procurement screen for Iraq is not optional.** Iraq's May 2022 anti-normalisation law makes Israeli-headquartered vendors (ThetaRay here, NICE Actimize in file 08) legally and reputationally unusable;
   a US-owned or US-based vendor (Featurespace, owned by Visa; Feedzai, San Mateo) carries US-sanctions exposure: Feedzai's published policy "restricts transactions involving sanctioned countries or persons designated by OFAC" and screens clients and vendors against OFAC lists, so a bank hit by US measures (as Al-Huda Bank was, file 01 section 4) could lose vendor support. This shrinks the realistic competitor set to European and Asian vendors and to local builds (VB-28). Iraqi counsel must confirm.
5. **Arabic is unserved.** None of the seven shows Arabic UI or Arabic name matching; Quantexa says Arabic parsing is "on the horizon" (March 2025). Arabic-first entity resolution, name matching and LLM output are
   product-defining (links to file 22), and a test set with published precision and recall is a selling point (VB-01).
6. **Evidence quality is poor everywhere, and AKR-TMS can win on honesty.** Vendor pages claim 70-99% fewer false positives; HSBC (with method) reports 60% fewer alerts and 2-4x more suspicious activity; HKMA's sample shows 7%
   auto-closed and 80% of top-scored alerts becoming STRs; BIS (synthetic) shows about 40% fewer false positives for ML in silos. AKR-TMS should publish definitions, baselines and recall, run a back-test on each bank's own
   history before go-live, and promise ranges it can reproduce (VB-21, VB-24, VB-26).
7. **Networks plus information sharing, not model sophistication, produced the largest measured gains** (HKMA FMLIT: +319% intelligence-led STRs; Aurora CAL arrangements), but sharing needs a statute (TMNL wound down
   its current form pending the EU AMLR). Build strong single-bank graph analytics first; keep models and typology cards federation-ready for a later CBI- or FIU-sponsored pilot (VB-16, VB-19).
8. **Agents are arriving fast and need governance first.** Quantexa (FinCrime Agent, Q Assist), ThetaRay (RAY), Hawk (AML Investigative Agent, March 2026), Feedzai (Farol, 24 Sept 2026) all launched L1 or SAR-drafting
   agents; Hawk and Oliver Wyman published a governance framework; Hawk's Alviere test is a good validation pattern (replay on historical alerts, count missed true positives first). None documents a fully local model, which Iraq needs (VB-09, VB-10, VB-29).
9. **Price.** The only public price signal is Quantexa's UK public-sector list (GBP 0.6-7.7m per suite per year, annual subscription plus integrators), which no Iraqi bank can pay; everyone else hides prices. A transparent, per-bank
   tiered price with an on-premises licence and local support is a market position of its own.
10. **Regulatory reception.** No regulator endorses a named product. Evidence of acceptance is indirect: ThetaRay's Santander case says a central bank approved the approach; Commerzbank says its validation now includes AI
    model governance; Celent (a research firm) awarded HSBC and the Quantexa/ABN AMRO deployment for model risk management. Vendor statements of compliance with SR 11-7 are now out of date (file 05 section 1c).

### Limits of this research

- The web-search budget (200 calls shared by the whole research team) was exhausted midway through Feedzai, so later checks used direct page fetches only. **Not reached or not verified:** G2 and Capterra pages (Cloudflare 403),
  Gartner Peer Insights pages (403; Quantexa rating taken from a search summary), Business Wire releases (403), FinTech Futures (403), the Visa press release on Featurespace (date and price), Feedzai's and Tookitaki's
  deployment options, later funding rounds for Tookitaki and Consilient, and any DBS or Danske Bank AML-AI figures.
- Vendor sites change; all vendor pages were read on 2026-10-06.

---

## 10. Derived requirements (VB-xx, file 12)

| ID | Requirement | Source |
|----|-------------|--------|
| VB-01 | Entity-resolution service that stores, per matched pair, the features and rules that linked them, a confidence, and any reviewer override, and that reports precision and recall on a labelled sample for each bank (including Arabic and Kurdish name variants and Iraqi identifiers) | Quantexa (section 1); Arabic gap (sections 1, 9) |
| VB-02 | Network and link view built from the read-only CBS replica (shared identifiers, counterparties, beneficiaries, branches) on PostgreSQL, without a Spark or Elasticsearch prerequisite | Quantexa prerequisites (1.1); HKMA network analytics (8.4) |
| VB-03 | Entity-centric analyst workspace: customer, linked parties and all open alerts and cases in one screen | Quantexa (1.8) |
| VB-04 | Label-free anomaly layer shipped with a typology-to-feature map and per-alert reason codes (weighted indicators) | ThetaRay (2.2); Wolfsberg (file 04 section 2b) |
| VB-05 | Counterparty profiles for non-customers seen only in transactions ("pseudo customer") | ThetaRay (2.2) |
| VB-06 | Built-in definitions for alert outcome classes (false positive, productive, escalated, STR filed, missed) and a per-bank effectiveness report | ThetaRay (2.3); section 8.1 |
| VB-07 | Simulator that replays any rule, threshold or model change on the bank's history and reports alert counts, overlap with current alerts and known-case misses before activation | ThetaRay (2.2); Hawk (3.2); Tookitaki (6.2) |
| VB-08 | Two-score alert (suspicion likelihood and false-positive likelihood) with text reasons; queue priority derived from both | Hawk (3.2) |
| VB-09 | Decision-rights ladder for AI functions: autonomy tier per function, human-approval rule, kill switch, full action logs, no autonomous closing in version 1 | Hawk with Oliver Wyman (3.2); ThetaRay RAY (2.2) |
| VB-10 | Back-test harness for any triage or investigation agent: replay on past alerts, report missed true positives first, then false positives closed | Hawk, Alviere (3.3) |
| VB-11 | Active-learning label queue: sampled alerts, near-threshold non-alerts and branch referrals presented to analysts for labelling | Feedzai (4.2) |
| VB-12 | Pattern recogniser on the link graph (funnel/mule, circular flow, structuring fan-in/fan-out) producing explainable pattern alerts | Feedzai Genometries (4.2); HKMA (8.4) |
| VB-13 | Shared mule-account signal layer usable by AML and fraud teams | Feedzai (4.1, 4.5) |
| VB-14 | Per-customer behavioural baseline store usable offline (Phase 1) and online within a latency budget (Phase 2 scoring API) | Featurespace (5.1, 5.6) |
| VB-15 | One validated model family with per-bank calibration and a per-bank delta validation pack | Featurespace Eika case (5.2); Wolfsberg (file 04 section 2b) |
| VB-16 | Scenario library as signed, versioned, PII-free scenario cards (typology, CBI article, indicators, data fields, parameters, back-test, owner, review date), delivered as offline bundles | Tookitaki (6.2, 6.8) |
| VB-17 | No-code scenario studio with simulation, approval workflow and rollback | Tookitaki (6.2) |
| VB-18 | Alert re-scoring mode that ingests alerts from a legacy tool and prioritises them while the bank migrates | Tookitaki UOB case (6.3); HKMA banks C and D (8.4) |
| VB-19 | Model export and import as signed packages with lineage metadata so a later CBI- or FIU-sponsored federated or centralised pilot needs no redesign | Consilient (7.6); BIS Aurora (8.3) |
| VB-20 | Label policy with several label sources (STR outcome, analyst disposition, branch referral, FIU or law-enforcement request, typology flag) and reliability weights; STR is a weak label | Consilient (7.6); Wolfsberg (file 04 section 2a) |
| VB-21 | Standard metric set per scenario, model and bank: alerts, escalations, STRs, precision at top-k, auto-close volume and re-open rate, time to first action, recall on a labelled holdout, baseline comparison | Section 8 |
| VB-22 | Auto-close guardrails: sample quality assurance, a dormant period (default 90 days) with re-open conditions, and automatic suspension of auto-close when re-open rates rise | HKMA Bank B (8.4) |
| VB-23 | Champion-challenger running in parallel on live alerts without operational impact | HKMA Bank C (8.4); Tookitaki (6.2) |
| VB-24 | Pre-go-live back-test report on the bank's own history (12-24 months) with before/after method disclosed | HSBC (8.2); Wolfsberg (file 04 section 2b) |
| VB-25 | On-premises package with a minimal footprint (the project's PostgreSQL, Valkey, MinIO, OpenBao stack; file 32), published sizing, and no mandatory big-data cluster | Quantexa, Hawk, ThetaRay, Google AML AI deployment findings (sections 1.1, 2.1, 3.1, 8.2) |
| VB-26 | Evidence policy: every percentage in product, sales or plan material states definition, baseline, period, sample and recall | Sections 1.3, 2.3, 3.3, 4.3, 6.3, 7.2, 8.1 |
| VB-27 | Staged modelling path with gates: CBI scenarios plus unsupervised and peer-deviation scores first; supervised models only after label-volume and quality thresholds; hybrid output | BIS Aurora (8.3); Feedzai (4.2); ThetaRay (2.2); Consilient (7.5) |
| VB-28 | Documented vendor-origin and ownership screen for competitor analysis and for any component or data partner, with Iraqi legal advice on the 2022 anti-normalisation law and a US-sanctions-exposure review | ThetaRay (2.1); Featurespace (5.4) |
| VB-29 | Assistive LLM features (summaries, narrative drafts) must run on a local open-weight model inside the bank, with Arabic and English quality evaluation; output marked draft, logged, human-approved | Quantexa Cloud AML on Azure (1.1); Hawk, ThetaRay, Feedzai agents (sections 2-4) |

---

## 11. Sources

All URLs read on 2026-10-06 unless marked (search summary only) or (blocked).

**Quantexa**
- [Quantexa AML solution page](https://www.quantexa.com/solutions/anti-money-laundering/)
- [Quantexa, "A Connected View of Financial Crime" (impact claims, HSBC and Standard Chartered tiles)](https://www.quantexa.com/discover/financial-crime-a-connected-view/)
- [Series F release, 5 March 2025 (Ontario Teachers' page)](https://otpp.com/en-ca/about-us/news-and-insights/2025/quantexa-completes-175million-usd-investment-round-led-by-tvg/)
- [Series F coverage with board seats (search summary)](https://pulse2.com/quantexa-175-million-series-f-raised-at-2-6-billion-valuation-for-decision-intelligence-solutions/)
- [tech.eu, 8 Jan 2026: FY to March 2025 revenue GBP 126m, loss GBP 25m](https://tech.eu/2026/01/08/ai-unicorn-quantexa-reports-revenue-boost-as-losses-halve/)
- [UK Digital Marketplace G-Cloud 15 listing: deployment, prerequisites, support](https://www.applytosupply.digitalmarketplace.service.gov.uk/g-cloud/services/174689844830097)
- [Quantexa G-Cloud 15 pricing document, Jan 2026](https://assets.applytosupply.digitalmarketplace.service.gov.uk/g-cloud-15/documents/703515/174689844830097-pricing-document-2026-01-29-1253.pdf)
- [Quantexa Cloud AML launch, 16 Sept 2025](https://www.quantexa.com/press/quantexa-launches-cloud-aml-product-for-u-s-mid-size-and-community-banks/)
- [QuanCon25 announcements, 6 March 2025 (Arabic parsers "on the horizon")](https://timestech.in/quantexa-unleashes-wave-of-next-gen-ai-decision-intelligence-at-quancon25/) and [GlobeNewswire original](https://www.globenewswire.com/news-release/2025/03/06/3038527/0/en/Quantexa-Unleashes-Wave-of-Next-Gen-AI-Decision-Intelligence-at-QuanCon25.html) (search summary)
- [Chartis 2025 category leader release, 19 Nov 2025](https://www.globenewswire.com/news-release/2025/11/19/3190628/0/en/Quantexa-Named-as-a-Category-Leader-in-Chartis-Research-s-2025-AML-Transaction-Monitoring-and-KYC-Solutions-Reports.html) (search summary)
- [Gartner Peer Insights, Quantexa](https://www.gartner.com/reviews/market/decision-intelligence-platforms/vendor/quantexa/product/quantexa-decision-intelligence-platform) (blocked; rating from search summary)
- [Quantexa contact page: DIFC Dubai and Doha offices](https://www.quantexa.com/contact/) (search summary)
- [HSBC trade finance with Quantexa, 25 Sept 2019 (page also lists 2026 Chartis headlines)](https://www.quantexa.com/press/hsbc-introduces-financial-crime-detection-systems/)
- [Danske Bank with Quantexa, 17 Dec 2020](https://www.quantexa.com/press/danske-bank/)
- [ABN AMRO KYC case](https://www.quantexa.com/resources/abn-amro/)
- [Standard Chartered release on Quantexa](https://www.sc.com/en/press-release/joining-forces-with-quantexa-to-tackle-financial-crime-with-contextual-decision-intelligence/)

**ThetaRay**
- [thetaray.com home](https://www.thetaray.com/), [about](https://www.thetaray.com/about), [Transaction Monitoring](https://www.thetaray.com/transaction-monitoring), [news and media](https://www.thetaray.com/news-media), [partners](https://thetaray.com/partners/)
- [RAY agentic investigation suite](https://thetaray.com/ray-the-agentic-ai-aml-investigation-suite-that-transforms-case-resolution/)
- [Shift4 case study, May 2026](https://thetaray.com/wp-content/uploads/2026/05/THETARAY_CASESTUDY_SHIFT4_2026-3.pdf)
- [Miami retail bank case study, 2026](https://thetaray.com/wp-content/uploads/2026/05/Retail-Bank-Case-Study.pdf)
- [Santander case study](https://4639135.fs1.hubspotusercontent-na1.net/hubfs/4639135/2024%20Website/THETARAY_CASESTUDY_3_SANTANDER.pdf)
- [Mashreq Bank selects ThetaRay, 14 Feb 2022 (PR Newswire)](https://prnewswire.com/il/news-releases/mashreq-bank-chooses-thetaray-transaction-monitoring-solution-for-correspondent-banking-832548305.html)
- [ThetaRay USD 57m round, 5 Sept 2023 (Business Wire)](https://www.businesswire.com/news/home/20230905216874/en/ThetaRay-the-Leader-in-Next-Generation-AI-Powered-Secure-Global-Payments-Raises-$57-Million-to-Continue-Unlocking-Business-Opportunities-for-Banks-and-FinTechs) (blocked; search summary)
- [ThetaRay acquires Screena, July 2024 (AML Intelligence)](https://www.amlintelligence.com/2024/07/fast-growing-israeli-fincrime-detector-thetaray-acquires-screena/) (search summary)
- [Iraqi parliament passes bill criminalising ties with Israel, 26 May 2022 (AP via KSAT)](https://www.ksat.com/news/world/2022/05/26/iraqi-lawmakers-pass-bill-criminalizing-any-ties-with-israel/) (search summary); [Al-Monitor](https://al-monitor.com/originals/2022/05/iraqs-parliament-passes-anti-normalization-law) (search summary)
- [rfp.wiki aggregator on ThetaRay reviews](https://www.rfp.wiki/payments-fraud/fraud-detection/thetaray) (search summary, low quality)

**Hawk**
- [hawk.ai home](https://hawk.ai/), [AML AI Overlay](https://hawk.ai/solutions/aml/ai-overlay), [AML Transaction Monitoring](https://hawk.ai/solutions/aml/transaction-monitoring), [tech stack](https://hawk.ai/platform/tech-stack), [AML Investigative Agent](https://hawk.ai/solutions/aml/aml-investigative-agent)
- [AML Investigative Agent launch, 25 March 2026](https://hawk.ai/news-press/hawk-releases-aml-investigative-agent-drastically-reduce-costs-and-enhance-quality-anti)
- [Commerzbank and Hawk, 10 March 2026](https://hawk.ai/news-press/commerzbank-optimizes-anti-money-laundering-measures-through-ai-partnership-hawk)
- [Ebury case study, 9 Sept 2026](https://hawk.ai/case-studies/ebury-ai-aml-transaction-monitoring); [Alviere agent case study](https://hawk.ai/case-studies/alviere-aml-investigative-agent)
- [Hawk and Oliver Wyman, governing AFC agents](https://insights.hawk.ai/governing-afc-agents)
- [Series C coverage, April 2025 (Startbase)](https://www.startbase.com/news/hawk-sammelt-56-millionen-us-dollar-in-series-c-runde-ein/) and [Munich Startup](https://www.munich-startup.de/en/news/hawk-raises-52-million-euros) (search summaries)
- [Hawk profile on AWS Startups (five AWS Regions, four continents)](https://startups.aws.com/learn/hawk-fighting-financial-crime-with-ai-precision-) (search summary)

**Feedzai**
- [AML Transaction Monitoring](https://www.feedzai.com/solutions/aml-transaction-monitoring/), [Anti-Money Laundering](https://www.feedzai.com/anti-money-laundering/), [RiskOps platform](https://www.feedzai.com/riskops/), [Core Banking Providers](https://www.feedzai.com/industries/core-banking-providers/)
- Press: [Expanded AML capabilities, Oct 2019](https://www.feedzai.com/pressrelease/feedzai-unveils-expanded-aml-capabilities-underpinned-by-advanced-machine-learning-and-explainability/); [AI in Case Manager, Sept 2019](https://www.feedzai.com/pressrelease/feedzai-launches-ai-in-case-manager-powered-by-next-generation-ai-capabilities/); [Risk Ledger, Dec 2018](https://www.feedzai.com/pressrelease/announcing-feedzai-risk-ledger-leveraging-federated-data-to-find-and-prevent-financial-crime-at-scale/); [Jack Henry, 25 Aug 2022](https://www.feedzai.com/pressrelease/jack-henry-announces-plan-for-new-financial-crimes-platform/); [Novobanco, 6 March 2026](https://www.feedzai.com/pressrelease/novobanco-modernizes-fraud-aml-with-feedzai-ai/); [Farol, 24 Sept 2026](https://www.feedzai.com/pressrelease/as-banks-pivot-to-agentic-ai-feedzai-unveils-farol-to-transform-fraud-analysis-and-cut-investigation-times/)
- [Funding release, 2 Oct 2025 (AAP)](https://www.aap.com.au/aapreleases/cision20251002ae88885/)
- Customer stories: [Wio Bank](https://www.feedzai.com/customer-stories/wio-bank/), [Standard Chartered Bank](https://www.feedzai.com/customer-stories/standard-chartered-bank/)
- [Demyst acquisition coverage](https://www.feedzai.com/inthenews/australian-fintech-demyst-snapped-up-in-157m-acquisition/) (search summary)
- [BTG Pactual: migration from on-premises to Feedzai on AWS](https://www.feedzai.com/operational-efficiency-and-cloud-scale-btg-pactual/); [Feedzai Trade Sanctions and AML Policy](https://www.feedzai.com/legal/trade-sanctions-and-aml-policy/)

**Featurespace**
- [featurespace.com home](https://www.featurespace.com/), [solutions](https://www.featurespace.com/solutions), [AML solution](https://www.featurespace.com/solutions/anti-money-laundering), [TallierLTM](https://www.featurespace.com/solutions/tallier-ltm), [sitemap](https://www.featurespace.com/sitemap.xml)
- Case studies: [NatWest](https://www.featurespace.com/case-studies/natwest), [Danske Bank](https://www.featurespace.com/case-studies/danske-bank), [Eika Gruppen](https://www.featurespace.com/case-studies/eika-gruppen), [Permanent TSB](https://www.featurespace.com/case-studies/permanent-tsb)
- [Founder article on the journey and the Visa acquisition](https://www.featurespace.com/knowledge-hub/the-journey-of-featurespace)

**Tookitaki**
- [tookitaki.com home](https://www.tookitaki.com/), [Transaction Monitoring](https://www.tookitaki.com/product-aml-transaction-monitoring), [Alert Prioritization AI Agent](https://www.tookitaki.com/alert-prioritization-ai-agent), [about](https://www.tookitaki.com/about), [security](https://www.tookitaki.com/security), [sitemap](https://www.tookitaki.com/sitemap.xml)
- [UOB (Singapore) case study](https://www.tookitaki.com/case-studies/traditional-banks)
- [Series A, 25 Nov 2019](https://www.tookitaki.com/blog/tookitaki-gets-another-us11-7-million-in-series-a-closes-round-at-us19-2-million)
- [Technology behind the AFC Ecosystem, 24 Aug 2023](https://www.tookitaki.com/afc-thoughts/technology-behind-tookitaki-afc-ecosystem-deep-dive)

**Consilient**
- [consilient.com home](https://consilient.com/), [about](https://consilient.com/about-us), [Core AML/CFT model](https://consilient.com/our-products/core-aml-cft-model), [Correspondent Banking model](https://consilient.com/our-products/correspondent-banking), [High-Risk Jurisdictions model](https://consilient.com/our-products/high-risk-jurisdictions-model)
- [Next-generation federated learning solution, 28 Feb 2023](https://consilient.com/consilient-brings-to-market-its-next-generation-federated-learning-solution-for-financial-crime-detection); [Secura/Isaac Group alliance, 2024](https://consilient.com/secura-isaac-group-consilient-announce-strategic-alliance)

**Published results (section 8)**
- [HSBC, "Fighting money launderers with artificial intelligence", Google Cloud blog, 30 Nov 2023](https://cloud.google.com/blog/topics/financial-services/how-hsbc-fights-money-launderers-with-artificial-intelligence)
- Google Cloud AML AI documentation: [overview](https://cloud.google.com/financial-services/anti-money-laundering/docs/concepts/overview), [governance artifacts](https://cloud.google.com/financial-services/anti-money-laundering/docs/collect-model-risk-governance-artifacts), [data scope and duration](https://cloud.google.com/financial-services/anti-money-laundering/docs/understand-data-scope-duration)
- [BIS Innovation Hub, Project Aurora, May 2023 (page)](https://www.bis.org/publ/othp66.htm) and [PDF](https://www.bis.org/publications/project-aurora-power-data-technology-and-collaboration-combat-money-laundering-across-institutions-and-borders.pdf)
- HKMA: [AML/CFT Regtech page](https://www.hkma.gov.hk/eng/key-functions/banking/anti-money-laundering-and-counter-financing-of-terrorism/aml-cft-regtech/); [AML Regtech: Network Analytics, May 2023](https://www.hkma.gov.hk/media/eng/doc/key-functions/banking-stability/aml-cft/AML_Regtech-Network_Analytics.pdf); [AML/CFT Regtech: Case Studies and Insights Volume 2, Sept 2023](https://www.hkma.gov.hk/media/eng/doc/key-functions/banking-stability/aml-cft/AMLCFT_Regtech-Case_Studies_and_Insights_Volume_2.pdf)
- [TMNL announcement, 1 July 2024](https://tmnl.nl/en/article/transaction-monitoring-netherlands-adapts-its-working-method-to-new-european-legislation/) and [TMNL home](https://tmnl.nl/en/)
- [FATF, Opportunities and Challenges of New Technologies for AML/CFT, July 2021 (Bank of Russia mirror), Annex D case studies](https://www.cbr.ru/Content/Document/File/126302/OCNT.pdf)
