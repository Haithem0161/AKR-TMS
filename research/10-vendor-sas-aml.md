# 10 — Vendor: SAS Anti-Money Laundering, Visual Investigator, Viya and Customer Due Diligence

Status: done (2026-10-06)

Part 1 of 4 of the vendor research (incumbent enterprise AML suites). Other files: 08 NICE Actimize, 09 Oracle FCCM, 11 other
incumbents and the comparison table. Requirement IDs use the prefix VA-xx and are numbered across all four files.

## Contents

1. Ownership and history
2. Product modules
3. Architecture and deployment
4. Detection approach
5. AI and GenAI features to 2026
6. Case management and regulatory reporting (goAML?)
7. Data model and core banking integration
8. Arabic/RTL support and MENA/Iraq presence
9. Implementation time, cost and licensing signals
10. Analyst rankings
11. What it does right (with evidence)
12. What it does wrong (with evidence)
13. Lessons for AKR-TMS (copy, avoid, beat)
14. What it means for AKR-TMS
15. Derived requirements (VA-xx)
16. Sources

---

## 1. Ownership and history

| Date | Event | Source |
|---|---|---|
| 1 Jul 1976 | SAS Institute incorporated in North Carolina with four employees; first product Base SAS, "roughly 300,000 lines of code" (started in 1960s agricultural-research software at NC State) | SAS 2025 annual report |
| 1998 | HSBC becomes a SAS customer; relationship "extended" on **5 Oct 2026** (HSBC "to pilot its Viya platform") | SAS press release 5 Oct 2026 |
| 2000s | Acquisitions listed in the annual report: DataFlux, Marketmax, IDeaS, Teragram (DataFlux brings data quality and Teragram text analytics; presumably the origin of the AML suite's data-quality and name-resolution components, which SAS does not state) | SAS annual report |
| 2006 | Mashreq (UAE) deploys SAS AML across the UAE and overseas branches; a "major overhaul of the SAS AML 4.2 system" later | SAS customer story |
| Versions | AML 5.1, 5.2, 6.1, 6.2, 6.3 (with "High-Performance AML" and "Peer Group Analysis"), 7.1, 7.2, 7.2M1, 8.1, 8.2, 8.3, then SAS AML on **SAS Viya 4** (help center version 11.1); **SAS Customer Due Diligence is now included in the AML application** | SAS support page |
| Nov 2024 | Acquires the principal assets of Hazy (synthetic data) | Forrester Wave Q2 2025 via SAS release |
| Apr 2025 | Forrester Wave AML Q2 2025: Leader, current offering 4.40/5 ("second highest") | SAS release 8 Apr 2025 |
| 28 Apr 2026 | Viya Copilot (built on Microsoft Foundry), Viya MCP Server, Agentic AI Accelerator; fincrime copilot "planned" for 2026 | SAS release |
| 24 Aug 2026 | Forrester Wave: **Financial Crime Management Solutions, Q3 2026**: SAS highest current-offering score (4.70/5) | SAS release |
| 24 Sep 2026 | Chartis RiskTech100 2027: SAS ranked No. 3 overall; 12 category awards | SAS release |

**Ownership and scale.** SAS is **privately held and debt-free** ("one of the world's largest privately held software
companies", SAS release 1 Jul 2026), with "more than $3 billion in annual sales", an "uninterrupted streak of
profitability" and more than half of sales outside the US (2025 annual report). It reports 50,000+ customers in
150+ countries and 2,700+ financial-services customers. It does not disclose AML revenue or AML customer counts. The
Forrester inclusion rule for AML vendors (at least USD 10m AML revenue) shows SAS exceeds that.

**Contrast with NICE and Oracle.** No sale process, no listed parent to please (compare NICE's pending sale to Brookfield,
file 08), and a founder-led, long-horizon owner. SAS's weakness is not ownership risk; it is platform dependence
(everything is SAS code, SAS licences and SAS skills).

### AKR-TMS hooks (section 1)

- Offer the vendor-stability argument locally: a small Iraqi vendor must replace "founder-led, debt-free" with
  escrow, open formats and a bank-held code licence (VA-01, VA-02).
- Avoid proprietary languages: scenarios and models are exported as readable definitions (SQL/JSON) and Python
  notebooks, not a vendor-specific language (VA-35).

## 2. Product modules

| Module | What SAS says it does | Notes |
|---|---|---|
| **SAS Anti-Money Laundering** (on Viya) | Transaction monitoring with rules plus ML, peer-group anomaly detection, alert aggregation at entity level, auto-prioritisation, workflow builder, entity resolution and network generation, case creation and regulatory reporting, low-code scenario authoring, CDD/EDD included | "Combine proven, out-of-the-box scenarios with industry data"; a "scenario catalogue" (Mashreq picked 8 new scenarios from it) |
| **SAS Customer Due Diligence** | Customer risk rating from risk policies, bands (high, medium, low), event-based continuous rating, EDD workflows with dynamic checklists, "open-box (transparent) customer risk rating models", beneficial-owner view through networks | Now bundled into the AML application |
| **SAS Visual Investigator (VI)** | Investigation management and intelligence analysis on Viya: alert triage, search, entity resolution, network analytics with centrality metrics, configurable case workflows, investigative workspaces; **SAS Mobile Investigator** for field staff; **Viya Copilot for VI** (natural-language search, summaries) | Used across AML, fraud, public sector |
| **SAS Real-Time Watchlist Screening** | Cloud-native sanctions/watchlist screening via REST, composite scoring, four-eyes review, allow and deny lists, rules authoring; AI predicts the cultural origin of names; offered jointly with Neterium (Orange Bank case) | "As-a-service offering"; isolated tenants |
| **SAS Financial Crimes Analytics** | Overlay for an existing monitoring platform: dynamic segmentation (k-means), above/below-the-line threshold tuning, hibernation of low-risk events, models operationalised "in batch and real time" | "No need to replace your current platform" |
| SAS Fraud Decisioning, Enterprise Fraud and Financial Crimes | Real-time fraud (cards, payments, application, mule detection), consortium-trained models; FRAML | Not in scope for AKR-TMS |
| Platform | SAS Viya, Model Studio, Visual Analytics, Data Hub, Data Management, Intelligent Decisioning, Visual Text Analytics, AI Navigator (governance, Oct 2026) | The data-science and governance layer |
| Services | SAS Managed Cloud Services (hosted on Azure, Azure Government or AWS, or "Remote Managed Services" where SAS manages software on the customer's own infrastructure); SAS consulting and implementation | |

### AKR-TMS hooks (section 2)

- SAS's overlay product (Financial Crimes Analytics) shows that many banks want better detection **without** replacing
  their current engine; AKR-TMS should be installable as an overlay that imports alerts and transactions from the
  bank's existing monitoring tool during migration (VA-13 in file 08).
- "Transparent customer risk rating" is a stated differentiator: show every factor, weight and band boundary of the
  CRR model, version it, and let the bank re-run history under new weights (file 26) (VA-36).

## 3. Architecture and deployment

- **Platform.** The current AML release runs on **SAS Viya 4**, SAS's "cloud-native data and AI platform", deployable
  on AWS, Azure, GCP or on premises ("flexible deployment options across cloud, hybrid and on-premises"). (Viya 4 is
  generally described as container/Kubernetes-based; that detail was **not verified** in the pages fetched.)
  Earlier versions (5.x to 8.x) had separate installation, configuration and administration guides; customers describe
  upgrades and installs as substantial projects (Mashreq's "major overhaul" of AML 4.2; TD Bank's "various issues with
  the install", below). Platform details of the older versions were not verified.
- **Data.** "A financial-services-specific data model: maps transaction records to support transaction, account,
  customer and **household** dimensions; includes the core schema **for preparing data for nightly batch analysis**";
  a knowledge-centre schema for retention and investigation; data orchestration for multiple data types (non-monetary
  events, geography, risk lists, third-party data, associates). Data integration, data quality (DataFlux heritage) and
  entity resolution are inside the product.
- **Real time vs batch.** The core AML detection is a **nightly batch** on the SAS data model; real-time elements are the
  watchlist screening service, fraud decisioning, and "real-time network and entity generation" ("without the need to
  execute long overnight batch jobs", SAS marketing). The Financial Crimes Analytics page says models can be
  operationalised "in batch and real time".
- **Hosting.** SAS Managed Cloud Services offers Hosted (SAS runs it on Azure, Azure Government or AWS) and **Remote
  Managed** (SAS manages the SAS software "while your organization retains its data and infrastructure within its own
  on-premises or private cloud environment"). That is the on-premises-friendly route. Real-Time Watchlist Screening is
  described as a fully cloud-native as-a-service product; an on-premises variant was not found.
- **Documentation is closed.** "The documentation for SAS Anti-Money Laundering is intended for use by existing customers
  and requires an access key" (SAS support page); older versions use "a secure site that requires a user ID and
  password". Compare Oracle (public) and NICE (customer portal).
- **Regions in the Gulf.** SAS has offices in Dubai (SAS Middle East FZ-LLC, Dubai Media City), Riyadh (Saudi branch) and
  Doha; no Iraq office (section 8).
- **What failure looks like (regulator).** The US Department of Justice's statement of facts for TD Bank (plea of
  10 Oct 2024, attachment A, para 44) quotes the bank's own AML technologist chat of Oct 2021: "Old scenarios ; old CRR ;
  tech agility is poor to react to changers ... **we have not had a single new scenario added since we first implemented
  SAS due to various issues with the install**." The same document (paras 29-33) says no new scenarios or substantive
  parameter changes were made from at least 2014 to late 2022, consultants found "technology barriers to developing new
  scenarios or adding new parameters", and "between January 1, 2018, and April 12, 2024, TDBNA's automated AML monitoring
  failed to monitor **92% of transaction volume and 74% of transaction value**" (14.6 billion transactions, USD 18.3
  trillion) because domestic ACH, most check activity and internal transfers were excluded after about 2011. Two
  readings: (1) the vendor's tool is not the main cause (the bank chose and kept the exclusions); (2) an install that
  makes scenario changes impractical becomes a control failure. Both lessons apply to AKR-TMS.

### AKR-TMS hooks (section 3)

- Make "add a new scenario" a one-day activity inside the product (draft, test on history, approve, deploy) with no
  vendor install work; measure and publish the time (VA-37).
- Offer a managed-remote option for banks that cannot use a vendor cloud: AKR-TMS staff manage the software while the
  bank keeps data and servers on premises (VA-38).
- Put the coverage register and reconciliation in the first screen (VA-15, VA-16 in file 08)
  so a TD-style exclusion cannot persist unnoticed.

## 4. Detection approach

**Hybrid analytics.** "Traditional rules combined with AI and machine learning strategies" (AML solution brief);
"Combination of proven rules with advanced analytic techniques including machine learning, deep learning and artificial
intelligence", "peer-group anomaly detection" that compares an entity with its own history and its peers.

| Layer | What SAS describes | Notes |
|---|---|---|
| Scenarios | Out-of-the-box scenarios and a catalogue (including correspondent-banking scenarios); point-and-click scenario and risk-factor authoring; customised routing rules; scenarios, business rules and APIs can all generate alerts | "Supercharged scenario tuning and what-if analysis" reducing analysis "from hours to just minutes" |
| Segmentation | "Dynamic segmentation": ML models that identify customer segments and adapt thresholds; k-means clustering, scatter plots, summary statistics for segment evaluation | Financial Crimes Analytics page |
| Tuning | "Above-the-line/below-the-line testing"; scenario threshold tuning; model registration, validation, tracking, monitoring and retraining framework | |
| ML | Predictive models, supervised and unsupervised, "hundreds of financial crime features" via model-based scenarios; "predictive alert analytics" to score and hibernate low-risk events "until they are deemed worthy of investigation" | Forrester Wave Q3 2026 (as quoted by SAS): "a wide range of customizable SAS IP algorithms for ML-based risk scoring" |
| Entity resolution and network | Entity resolution (deduplication), network generation, centrality metrics (closeness, betweenness, influence), transaction network visualisation, link expansion | Forrester Q2 2025: "[SAS'] name resolution module offers a built-in **cultural affinity AI model**" |
| Alert aggregation | Alerts aggregated **at the entity level** "significantly enhancing efficiency compared with transaction-level reviews" | A practical false-positive reducer |
| Explainability | "Transparency and explainability ... visibility into the decisions made by AI/ML models so AML teams can understand why activities were flagged"; "white-box" model governance; open-box CRR models; white paper "Explainable AI for AML" | SAS says regulatory demands for transparency are "in part" why ML adoption in AML has been slow |

**Outcomes SAS publishes.**

| Claim | Where | Reading |
|---|---|---|
| "Reduce false positives by up to 80%", "increase efficiency by 25% through automation", "improve regulatory report conversion rates by 3-5x" | AML solution brief (2025-26) | "Up to"; no baseline |
| "Reduce false positives by up to 76%" | Enterprise Fraud & Financial Crimes page | Older figure |
| "90% reduction in false positives": Landsbankinn (Iceland) went from about **1,000 false positives a day to about 100** "within months" after replacing a legacy screening system and "tuning exercises" | Customer story | Large drop, but the baseline was a poor previous system and the unit is daily flagged transactions |
| "we've helped our customers reduce their false positives by as much as 90%" | SAS senior vice president quote, Apr 2025 | Marketing |

Compare the Actimize published cases (31%, 33%): both vendors' best claims differ by an order of magnitude because
baselines differ; neither is evidence for a bank whose baseline is unknown.

**Peers and scenarios in the MENA reference.** Mashreq "handpicked and implemented 8 new scenarios from the SAS scenario
catalogue" and targeted correspondent banking relationships, and runs "perpetual screening ... against the World Check
Premium file with specific consideration given to the updates for fields identified as Critical". Note the three
elements Iraq's CBI also asks for: scenario catalogue, correspondent focus, World-Check screening.

### AKR-TMS hooks (section 4)

- Aggregate alerts **by entity and period** by default (one case per customer per review window, with the triggering
  scenarios as evidence), which is the cheapest false-positive reduction available (VA-39).
- Treat "above-the-line / below-the-line" testing as a standard tuning report for every threshold change
  (VA-07 in file 08).
- Build peer-group comparison (segment-relative behaviour) into the Iraqi segmentation (government salary accounts,
  cash-heavy traders, money-transfer and exchange companies, NPOs) so that thresholds adapt to peers, not to a single
  national number (files 27 and 26).

## 5. AI and GenAI features to October 2026

| Feature | What SAS says | Date / status | Evidence quality |
|---|---|---|---|
| Classical ML in AML | Dynamic segmentation, anomaly detection, predictive alert scoring, hibernation, network analytics, model governance (register, validate, track, monitor, retrain) | Shipping on Viya | Vendor descriptions; Forrester scores |
| **SAS Viya Copilot** | A "governed, conversational AI assistant embedded directly into the SAS Viya platform"; "integrates **Microsoft Foundry**"; capabilities today include "**Visual investigation with AI-assisted search and AI-powered case and alert narratives**", code generation, model-pipeline guidance, dashboards | Announced 28 Apr 2026 (SAS Innovate); "SAS Viya Copilot wins SiliconANGLE TechForward award" 15 Sep 2026 | Vendor |
| Viya Copilot for SAS Visual Investigator | Natural-language search, "visual evidence interpretation" and case summarisation; product-documentation Q&A | VI page, Oct 2026 | Vendor |
| **Financial-crime Copilot** | "SAS plans to expand Viya Copilot for additional industries in 2026, **including financial crime prevention in banking**" - i.e., not yet shipped as of 28 Apr 2026 (ALM Copilot and a health-care copilot are the industry copilots "available today") | Planned | Vendor roadmap |
| **AI agents for sanctions screening** | "AI agents automate compliance checks against sanctioned entities using machine learning, rules and contextual intelligence. They perform initial triage and generate audit-ready documentation" | Agentic AI page | Vendor claim; no customer data |
| Agent infrastructure | **Viya MCP Server** (exposes SAS analytics, models and decisioning to external agents "from the LLM interface of their choice, such as Claude"), **Agentic AI Accelerator** (framework to build, govern, deploy agents), **Retrieval Agent Manager** (RAG; standalone product for now) | 28 Apr 2026 | Vendor |
| Schema-extraction agent | Forrester Q3 2026 notes a "powerful AI agent for extracting schemas from data sources" and "broad productized data source integrations" | Wave, Aug 2026 | Analyst (via SAS release) |
| Synthetic data | **SAS Data Maker** (from the Hazy assets bought Nov 2024), sold on Azure marketplaces; Forrester expects it to accelerate "large data models and other GenAI techniques in SAS' Financial Crime Compliance portfolio" | 2025-26 | Vendor/analyst |
| Roadmap (Forrester Q2 2025) | "GenAI inclusion into productized copilots", "model-driven customer risk rating, increased depth of cash activity risk scoring, and better scenario tuning and optimization processes" | Apr 2025 | Analyst |
| Governance | "White-box" model governance, "Trustworthy AI", SAS AI Navigator (announced 28 Apr 2026, available 5 Oct 2026); Chartis names SAS a leader in AI governance (12 Mar 2026); SAS press release "Only 11% of banks have cracked the code on trustworthy AI" (24 Feb 2026) | 2026 | Vendor/analyst |

**Reading.** SAS is, as of October 2026, **behind NICE and Oracle in shipped GenAI for AML investigations**: AI-powered
case and alert narratives exist inside Visual Investigator through the Copilot, but the AML-specific financial-crime
copilot was still "planned" in April, and the AML page and brochures do not name an STR/SAR narrative generator. Its
strength is the governed model platform around the AI (model management, explainability, synthetic data). The LLM
runs through Microsoft Foundry, which implies an Azure dependency; whether Copilot works for a "Remote Managed"
on-premises customer is not stated.

### AKR-TMS hooks (section 5)

- SAS's best AI idea for a bank like those in Iraq is **synthetic data for testing and model development** (data
  sharing is restricted in practice): ship a generator that produces Iraqi-shaped synthetic customers, accounts and
  laundering patterns for demos, tuning rehearsal and the typology harness (VA-40).
- A schema-extraction assistant for mapping core banking tables into the canonical model (Forrester praises SAS's)
  is a high-value, low-risk use of AI because humans approve every mapping (VA-41).
- Do not depend on a hosted LLM: SAS's Foundry dependency is the counter-example (VA-10 in file 08).

## 6. Case management and regulatory reporting

**Case management.** Two places: the AML application's own alert management and triage (alert aggregation at entity
level, auto-prioritisation, disposition options, workflow builder) and **SAS Visual Investigator** (alerts organised "by
domain, strategy and queue", configurable workflows with routing, escalation and task assignment, investigative
workspaces with timelines, maps and network views, audit log of alert activity, mobile app for field staff). Datos Matrix
2025 (Fraud and AML Case Management) lists SAS as a leader (SAS accolades list).

**Regulatory reporting.** The AML features list says users can "create cases from alerts and file regulatory reports to
government authorities"; Forrester gave SAS the top score for "regulatory and operational reporting" (Q3 2026) and said
SAS "provides more types of built-in reports than do many other vendors" (Q2 2025). SAS claims "3-5x" better "regulatory
report conversion rates". **goAML is not named** on any SAS page, brief or release fetched; the jurisdictions and report
formats supported are not listed publicly; documentation is behind a customer key. For an Iraqi bank the answer to "does
it file goAML STRs?" must be tested with a sample XML against the Iraqi schema (file 23).

### AKR-TMS hooks (section 6)

- Mobile access to a case is a niche SAS feature (Mobile Investigator); for Iraq the equivalent is a **branch liaison
  officer window** that works on a phone or a branch PC for the monthly report and confidential escalation (file 01,
  section 2d) (VA-42).
- Publish the list of report formats and the schema versions supported, with test XML samples (VA-30 in file 09).

## 7. Data model and core banking integration

- **Data model:** SAS's financial-services data model organises records by transaction, account, customer and household
  dimensions with a core schema for nightly batch analysis and a knowledge-centre schema for retention and
  investigation. Households are a first-class dimension; this fits Iraq's "family number" idea (file 01 section 7) better
  than the other incumbents, though SAS does not mention it.
- **Integration:** data integration, quality and entity resolution run inside the platform (data orchestration "through
  flexible compute environments"); "broad productized data source integrations" and an AI schema-extraction agent
  (Forrester Q3 2026). No named core-banking adapters; customers describe integrating "data from their core banking
  systems and third-party sources" into an enterprise data warehouse first (Landsbankinn: "a powerful enterprise
  data warehouse" plus the SAS tuning exercise).
- **Multi-bank and multi-country patterns that work (customer stories):**
  - **SDC** (IT service provider for 120 Nordic banks) runs "a communal service based on SAS Anti-Money Laundering":
    one governance and oversight system, scenarios changed once for several banks, "individual tailoring for each
    client's specific needs", so small banks get large-bank-quality surveillance.
  - **Bangkok Bank** built a "**SAS AML Global Image**" (a standard set of capabilities and analyses) piloted in Hong Kong,
    then rolled out to remaining locations; local requirements are either local specifications or added to the Global
    Image. Governance model, roles and process flows were defined alongside the deployment.
- **Failure pattern (TD Bank):** transaction types excluded by design for years (ACH, most checks, internal transfers) and a
  system difficult to change (para 33 and para 44, section 3). A data model is only as good as what is fed into it.
- **Migration burden:** SAS 9-based AML customers face a platform change: "SAS Viya is more than just an upgrade - it's a
  transformation" (SAS "Move to SAS Viya" page).

### AKR-TMS hooks (section 7)

- **Multi-bank operating model.** AKR-TMS is sold to many Iraqi banks: adopt the SDC and Bangkok Bank patterns - a
  **standard product image plus a local overlay** per bank (scenarios, thresholds, lists) with central release
  management, and an optional **shared-service** edition for small private banks or exchange companies run by a trusted
  operator, with strict tenant isolation (VA-43, VA-44).
- Include a **household / family dimension** in the canonical model using the Iraqi family number and mother's name
  (file 01) as link keys for relatives and PEP associates (VA-45).

## 8. Arabic/RTL support and presence in MENA and Iraq

| Question | Finding | Confidence |
|---|---|---|
| Arabic or RTL UI | Not stated anywhere in the SAS public pages fetched (31 pages). SAS Viya is a multi-language product but Arabic/RTL support for AML and Visual Investigator was **not verified** | Ask in RFP |
| Arabic names | Real-Time Watchlist Screening: "advanced fuzzy matching algorithms handle common aliases, **transliterations** and misspellings" and "AI to **predict the cultural origin of names across languages and alphabets**, enabling culturally aware matching"; name resolution in the AML suite has "a built-in cultural affinity AI model" (Forrester Q2 2025). SAS Viya Copilot and AML pages do not name Arabic | Medium (documented capability, no Arabic or Iraqi accuracy data) |
| Islamic banking | No Islamic product, scenario or CDD feature mentioned | High that it is not marketed |
| Offices and partners | **SAS Middle East FZ-LLC (Dubai Media City)**, Saudi branch (Riyadh), Qatar (Doha); contact lines for Bahrain, Egypt, Jordan, Kuwait, Lebanon, Oman, Pakistan, Qatar, Saudi Arabia, UAE and Yemen. **No Iraq office or contact** (HTML checked) | High |
| Customers (MENA) | **Mashreq (UAE)**: SAS AML since 2006 across UAE and overseas branches; AML 4.2 overhaul; World-Check Premium perpetual screening; correspondent banking scenarios; 8 scenarios from the catalogue. Other MENA AML references were not found in public pages | Medium |
| Iraq | No public Iraqi reference | Medium |

**Reading.** SAS is the incumbent with the strongest explicit claim on **name matching across scripts**, and a real
Gulf presence and reference (Mashreq). It has no Iraq footprint, no Islamic content, and no published goAML or Arabic UI
statement.

### AKR-TMS hooks (section 8)

- Publish an Arabic name-matching benchmark: precision and recall on a labelled set of Iraqi names with transliteration
  variants and Kurdish spellings, plus the "cultural origin" classifier as a visible feature (VA-32
  in file 09).
- Use Mashreq's pattern - scenario catalogue + correspondent-banking scenarios + World-Check perpetual screening - as the
  reference bundle Iraqi banks will be compared with by their USD correspondents.

## 9. Implementation time, cost and licensing signals

SAS publishes no AML prices. Licensing routes: "direct from SAS, through our trusted partners or via cloud
marketplaces" (SAS Viya "How to buy"); "Request pricing" for Viya; AML is quoted on request. Signals:

| Signal | Source | Reliability |
|---|---|---|
| "Landsbankinn ... Within months, we got that number [false positives] down to about 100 per day" after "tuning exercises with SAS" and "a powerful enterprise data warehouse" | SAS customer story | Medium (vendor-published, bank quote) |
| Treezor: "helped us implement a state-of-the-art solution in a **tight time frame**" (BaaS, 2023-24) | SAS customer story | Medium |
| Bangkok Bank: pilot in Hong Kong, "built up our internal capability to manage the SAS AML solution", then rolled out the Global Image "very quickly"; SAS provided "domain knowledge, best practices, project management, technical expertise" | SAS customer story | Medium |
| Mashreq: first deployment 2006; later "a major overhaul of the SAS AML 4.2 system" | SAS customer story | Medium |
| TD Bank: "we have not had a single new scenario added since we first implemented SAS due to various issues with the install" (Oct 2021) | DOJ statement of facts para 44 | **High** for the quote; the cause is the bank's characterisation |
| SAS 9 -> Viya is "more than just an upgrade - it's a transformation" | SAS Viya migration page | High: a platform migration for existing AML customers |
| Managed services: hosted on Azure/AWS or **Remote Managed** on the customer's infrastructure, to "alleviate compliance concerns" | SAS Managed Cloud Services page | High |
| No practitioner reviews: PeerSpot shows **0 reviews** for SAS AML; Gartner Peer Insights, G2 and TrustRadius could not be read | PeerSpot, access blocks | - |

**Estimate.** From the public record a first bank deployment is a multi-month integration with SAS professional services or
partners; the large-bank references (Bangkok Bank global rollout, Mashreq overhaul) are multi-year programmes. No
price data was found; treat any figure as unverified.

### AKR-TMS hooks (section 9)

- Publish what the bank does in each week of a deployment and what AKR-TMS does (no "implementation by consultants"
  for configuration that compliance staff can do) (VA-33 in file 09).
- Build the "image plus overlay" so that bank number two costs a fraction of bank number one (VA-43).

## 10. Analyst rankings

| Firm / report | Date | Result for SAS | Note |
|---|---|---|---|
| **Forrester Wave: Financial Crime Management Solutions** | Q3 2026 (SAS release 24 Aug 2026) | **Leader; highest current-offering score 4.70/5; top marks in 9 of 11 criteria "more than any other vendor evaluated"** (data integration, administrator management, watchlist management and screening, AI/ML risk scoring, transaction types, queue definitions and alert routing, case management, third-party integrations, regulatory and operational reporting); "no below-par scores ... the only solution provider" to achieve that; above-average customer feedback (customers like ML model building and testing); 15 firms evaluated | The most recent independent-style ranking in this research; Forrester: "SAS offers one of the strongest solutions in terms of technical capabilities"; the full report was not read |
| Forrester Wave: Anti-Money Laundering Solutions | Q2 2025 (8 Apr 2025) | Leader; current offering 4.40/5, "second highest"; top marks in 10 of 18 criteria; "SAS is the only AML vendor that is also recognised as a leader in AI and ML platforms" | NICE had top scores in all 10 current-offering criteria |
| Chartis RiskTech100 | 2027 (24 Sep 2026) | **No. 3 overall; the only vendor in the Top 10 in every edition since 2005**; 12 category wins (none of them AML, KYC or screening) | Chartis: "SAS' ongoing innovation, diversity of offerings and long-term commitment" |
| Chartis RiskTech Quadrant | 2025 | Leader listings for AML Transaction Monitoring (Nov 2025), Watchlist Monitoring, Adverse Media Monitoring, Name and Transaction Screening (all Aug 2025), Enterprise and Payment Fraud (Jun 2025); Chartis July 2026 update: "best-in-class in every category" in fraud platforms | SAS accolades pages |
| Datos Insights Matrix | Jul 2025 | Fraud and AML Case Management leader | SAS list |
| Celent | Sep 2025 | Anti-Fraud Solutionscape leader | SAS list |
| Gartner | Feb 2026 | Leader, Magic Quadrant for Decision Intelligence Platforms (not AML) | Context |
| PeerSpot | Oct 2026 | No reviews; AML mindshare 3.9%, down from 8.0% a year earlier | |
| Chartis FCC50 | 2025, 2026 | SAS is not among the vendor tags on the FCC50 2025 and 2026 pages (NICE Actimize, Oracle, SymphonyAI, Nasdaq Verafin, ACTICO and others are); Chartis lists a separate "Vendor Analysis: SAS - AML Transaction Monitoring Solutions, 2025" dated 10 Aug 2026 | Ranks paywalled; only vendor tags are visible |

## 11. What it does right (with evidence)

1. **Strongest independent score in the 2026 Wave** (4.70/5 current offering, no below-par criterion).
2. **Entity-level alert aggregation and household dimension** in the data model: fewer, richer alerts per customer.
3. **Governance and explainability as a product theme**: "white-box" governance, open-box CRR models, SAS model risk
   management leadership (Chartis MRM-Credit winner four years running), AI Navigator for AI governance.
4. **Proven multi-bank and multi-country operating models**: SDC (120 Nordic banks in a communal service) and Bangkok Bank
   (Global Image, Hong Kong pilot, 14 economies).
5. **Low-code scenario and workflow authoring and what-if tuning with above/below-the-line testing**.
6. **Screening across scripts**: cultural-origin AI, transliteration-aware fuzzy matching, four-eyes review, REST API.
7. **Overlay option** (Financial Crimes Analytics) and **Remote Managed Services** (SAS runs its software on the bank's
   own infrastructure).
8. **Synthetic data** (Data Maker) and a model lifecycle (register, validate, monitor, retrain).
9. **Open agent interface**: Viya MCP Server exposes models and decisioning to the bank's own agents.
10. **Corporate durability**: private, debt-free, 50 years, more than USD 3bn in sales.
11. **A published MENA reference** (Mashreq since 2006, correspondent-banking scenarios, World-Check Premium perpetual
    screening) and a Gulf presence (Dubai, Riyadh, Doha).

## 12. What it does wrong (with evidence)

| # | Weakness | Evidence |
|---|---|---|
| 1 | **Install and change are hard** | TD Bank: "various issues with the install" and no new scenario since first implementation (Oct 2021 chat; no new scenarios 2014-2022 per the statement of facts); Mashreq needed a "major overhaul" of AML 4.2; SAS 9 -> Viya "is a transformation" |
| 2 | **Closed documentation** | AML guides "require an access key" obtainable from a SAS consultant or support; Oracle and NICE publish more |
| 3 | **Nightly-batch core** | "core schema for preparing data for nightly batch analysis"; real-time pieces are screening and fraud |
| 4 | **GenAI for AML investigations not shipped by Apr 2026** | Financial-crime Copilot "planned" for 2026; no STR/SAR narrative generator named; Copilot depends on Microsoft Foundry |
| 5 | **Proprietary platform and skills** | Everything runs on SAS Viya; SAS code and skills; migration between SAS generations is a project |
| 6 | **No goAML, Arabic UI, Islamic or Iraq evidence** | No public statement on any of them |
| 7 | **No independent practitioner evidence** | PeerSpot: 0 reviews; mindshare 3.9% (from 8.0%); Gartner, G2, TrustRadius not readable |
| 8 | **Marketing figures without baselines** | "up to 80%", "up to 76%", "as much as 90%"; Landsbankinn's 90% came from a poor previous screening system |
| 9 | **Cost opacity** | No public price; multiple licence routes; managed-services add-on |
| 10 | **Cloud-first screening** | Real-Time Watchlist Screening is "fully cloud-native ... as-a-service"; an on-premises variant not found |
| 11 | **Regulator risk through the install** | TD Bank's SAS system excluded 92% of volume and 74% of value; the bank, not SAS, made exclusion decisions, but nothing in the product surfaced them as a coverage gap in a way that forced action |

## 13. Lessons for AKR-TMS

**Copy**

- Entity-level alerts and a household/relatives dimension (VA-39, VA-45).
- The "image plus overlay" and shared-service patterns for many banks (VA-43, VA-44).
- Synthetic data generation for testing and training (VA-40).
- Remote-managed on-premises operation (VA-38).
- An agent-friendly API so a bank can plug its own assistant into AKR-TMS safely (with the same access control and audit
  as a human user) (VA-46).
- Public documentation that a prospective bank can read without a customer key (VA-47).

**Avoid**

- An installation that needs vendor consultants for every scenario change (VA-37).
- "Up to X%" claims; replace them with a before/after on the bank's data.
- A hosted-LLM dependency for core features.
- Releasing an engine that can silently omit products or channels (the TD pattern) (VA-15 in file 08).

**Beat it on**

| Dimension | SAS | AKR-TMS target |
|---|---|---|
| Time to change a scenario | Weeks to years (TD) | One day, in the UI, with history replay |
| Documentation | Customer key | Public, bilingual |
| Arabic and Islamic | Not documented | First-class |
| Reporting | Not documented for goAML | goAML-native with the Iraqi chain |
| Multi-bank | Proven (SDC) but heavy | Productised image + overlay and shared service |
| Price | Opaque | Published tiers |
| On-prem AI | Foundry-dependent | Local model |

## 14. What it means for AKR-TMS

1. SAS is the **best-scoring technology** in 2026 and has the best-documented multi-bank patterns; AKR-TMS cannot win on
   breadth but can win on speed of change, Arabic and Iraqi fit, openness and price.
2. The TD Bank case is the one regulator-grade story that names a vendor product and teaches the right lesson: the risk
   is a system that is hard to change and a coverage gap nobody sees.
3. SDC's shared service for small banks is a model for AKR-TMS's route to the many small Iraqi private banks and exchange
   companies; the Bangkok Bank "global image" is the route to many banks with one product.
4. SAS's GenAI gap (shipping late, hosted in Azure) leaves room for a local, governed assistant.

## 15. Derived requirements (VA-35 to VA-47; the SAS-specific ones. Requirements VA-01 to VA-34 in files 08 and 09 also apply)

| ID | Requirement | Source |
|---|---|---|
| VA-35 | Scenarios, thresholds, models and workflows are stored as readable, versioned definitions (SQL/JSON/YAML, Python notebooks) that the bank can read, diff and export; no proprietary language or runtime is required to understand or move them | File 10 s.1, s.12 #5 (SAS code, skills and platform generations create lock-in) |
| VA-36 | Customer risk rating is an open-box model: every factor, weight, band boundary and override is visible and versioned, history can be re-run under new weights, and the model is exportable for CBI and correspondent review | File 10 s.2 (SAS CDD 'open-box CRR models'); file 26; CBI controls (risk rating 3-5 levels) |
| VA-37 | Adding or changing a scenario is a same-day, in-product workflow (draft, replay on history, approve, deploy) with no vendor install work; time-to-change per scenario is measured and shown to the MLRO | File 10 s.3, s.12 #1 (TD Bank: no new scenario since the SAS install; DOJ statement of facts paras 29-33, 44) |
| VA-38 | Remote-managed operation option: AKR-TMS staff operate and patch the software under the bank's access controls while data and servers stay inside the bank's data centre, with full session logging and no standing outbound data flow | File 10 s.3, s.11 #7 (SAS Remote Managed Services) |
| VA-39 | Alerts and cases are aggregated by entity and review period by default (one case per customer or counterparty with triggering scenarios as evidence), with the ability to drill to transaction-level alerts | File 10 s.4 (SAS entity-level aggregation) |
| VA-40 | Built-in synthetic data generator that produces Iraqi-shaped customers, accounts, transactions and laundering patterns for demos, training, tuning rehearsal and the typology harness, with no real customer data leaving the bank | File 10 s.5 (SAS Data Maker from Hazy assets) |
| VA-41 | AI-assisted mapping of core banking tables and columns to the canonical model that proposes mappings and data-type conversions with evidence (sample values, null rates), where a human approves every mapping and the decision is logged | File 10 s.5, s.7 (Forrester Q3 2026: SAS schema-extraction agent); file 25 |
| VA-42 | Branch liaison officer window usable on a branch PC or phone: raise an internal report, see acknowledgement and decision status, receive the monthly report prompt (by the 10th) and send confidential reports directly to the MLRO | File 10 s.6 (SAS Mobile Investigator idea); file 01 s.2d (CBI branch liaison officers) |
| VA-43 | Product image plus per-bank overlay: one tested standard configuration (scenarios, lists, workflows, reports) released centrally and a bank-specific overlay for thresholds, segments, lists and workflow changes that survives upgrades | File 10 s.7 (SAS Global Image at Bangkok Bank; SDC communal service) |
| VA-44 | Optional shared-service edition for small banks and exchange companies run by a trusted operator, with strict tenant isolation, per-tenant configuration and keys, per-tenant audit export and exit/data-return terms | File 10 s.7 (SDC provides SAS AML as a communal service to 120 Nordic banks) |
| VA-45 | Canonical model includes a household/family dimension (Iraqi family number, mother's name, address, guardianship) used to link relatives and PEP associates and to aggregate activity across a family | File 10 s.7 (SAS household dimension); file 01 s.7 (family number, mother's name) |
| VA-46 | Agent-safe API: a bank can connect its own assistants or agents to AKR-TMS through a documented interface that enforces the same roles, data scopes, rate limits and audit trail as a human user, with every call logged and reviewable | File 10 s.5 (SAS Viya MCP Server); file 08 s.5 |
| VA-47 | Complete product, data-model, scenario, API and operations documentation published openly in Arabic and English so a bank can evaluate it before signing; no customer key required for the technical documents | File 10 s.3, s.12 #2 (SAS docs behind an access key); file 09 s.12 #7 |

## 16. Sources

All fetched 2026-10-06 unless marked "(search result only)". Vendor pages are marketing and are labelled as such.

**SAS (vendor)**
- SAS Anti-Money Laundering page and feature list: https://www.sas.com/en_us/software/anti-money-laundering.html ; https://www.sas.com/en_us/software/anti-money-laundering/features-list.html
- AML solution brief (80% / 25% / 3-5x claims): https://www.sas.com/en/solution-briefs/anti-money-laundering-114010.html
- SAS Customer Due Diligence: https://www.sas.com/en_us/software/customer-due-diligence.html ; Visual Investigator: https://www.sas.com/en_us/software/intelligence-analytics-visual-investigator.html ; Real-Time Watchlist Screening: https://www.sas.com/en_us/software/real-time-watchlist-screening.html ; Financial Crimes Analytics: https://www.sas.com/en_us/software/financial-crimes-analytics.html ; Enterprise Fraud and Financial Crimes: https://www.sas.com/en_us/software/enterprise-fraud-financial-crimes.html ; Banking fraud and compliance: https://www.sas.com/en_us/industry/banking/solution/fraud-financial-crimes-compliance.html ; Fraud and compliance solutions: https://www.sas.com/en_us/solutions/fraud-and-compliance.html
- SAS Viya, how to buy, move to Viya: https://www.sas.com/en_us/software/viya.html ; https://www.sas.com/en_us/software/viya/how-to-buy-sas-viya.html ; https://www.sas.com/en_us/software/viya/move-to-sas-viya.html ; Agentic AI: https://www.sas.com/en_us/solutions/ai/agentic-ai.html ; Managed Cloud Services: https://www.sas.com/en_us/solutions/cloud/sas-managed-cloud-services.html
- AML support and documentation access page (versions, access key): https://support.sas.com/en/software/sas-anti-money-laundering-support.html
- White-paper and brief landing pages: https://www.sas.com/en/whitepapers/ai-machine-learning-redefining-aml-110762.html ; https://www.sas.com/en/whitepapers/explainable-artificial-intelligence-for-anti-money-laundering.html ; https://www.sas.com/en/solution-briefs/real-time-watchlist-screening-114128.html
- Customer stories: Treezor https://www.sas.com/en_us/customers/treezor.html ; SDC https://www.sas.com/en_us/customers/sdc-dk.html ; Bangkok Bank https://www.sas.com/en_us/customers/bangkok-bank.html ; Landsbankinn https://www.sas.com/en_us/customers/landsbankinn.html ; Mashreq https://sas.com/en_sa/customers/mashreq.html
- Company: https://www.sas.com/en_us/company-information.html ; 2025 annual report page: https://www.sas.com/en_us/company-information/annual-report.html ; contact pages: https://www.sas.com/en_us/contact.html ; https://www.sas.com/en_ae/contact.html ; https://www.sas.com/en_sa/contact.html
- Press releases (SAS newsroom, https://www.sas.com/en_us/news/press-releases.html): Forrester Wave Financial Crime Management Q3 2026 (24 Aug 2026) .../2026/august/forrester-financial-crime-solutions.html ; RiskTech100 2027 (24 Sep 2026) .../2026/september/risktech100-record-awards.html ; HSBC (5 Oct 2026) .../2026/october/hsbc-extends-strategic-relationship.html ; Viya AI assistants and agentic AI (28 Apr 2026) .../2026/april/innovate-viya-ai-assistants-agentic.html ; industry accelerators (28 Apr 2026) .../2026/april/industry-accelerators.html ; analyst accolades 2025 (30 Jan 2026) .../2026/january/analyst-accolades-2025.html ; 50th anniversary .../2026/july/50th-anniversary-july.html and .../2026/april/50th-anniversary.html ; Forrester Wave AML Q2 2025 (8 Apr 2025): https://sas.com/en_in/news/press-releases/2025/april/forrester-wave-aml-solutions-leader.html

**Regulator and court documents**
- US DOJ, TD Bank US Holding Company plea, Attachment A statement of facts, 10 Oct 2024 (paras 29-34, 44): https://www.justice.gov/media/1373151/dl ; Information (same text): https://www.justice.gov/media/1373341/dl

**Analysts and practitioners**
- Chartis rankings and FCC50 pages (vendor tags, "Vendor Analysis: SAS - AML Transaction Monitoring Solutions, 2025", 10 Aug 2026): https://chartis-research.com/financial-crime/7947472/financial-crime-and-compliance50-fcc50-2026-ranking-and-award-winners ; https://chartis-research.com/financial-crime/7947401/financial-crime-and-compliance50-2025-ranking-and-award-winners
- PeerSpot, SAS Anti-Money Laundering (no reviews): https://www.peerspot.com/products/sas-anti-money-laundering-reviews
- NICE Actimize and SymphonyAI statements on the Forrester Wave Q2 2025 (files 08 and 11).

**Not reached:** SAS AML documentation (access key), the Forrester Wave reports (paywalled), Chartis ranking tables, Gartner Peer Insights, G2, TrustRadius (blocked), SAS price lists, any Iraq reference (none public), and any SAS statement on goAML or Arabic user-interface support.
