# 09 — Vendor: Oracle Financial Services Crime and Compliance Management (FCCM / OFS AML)

Status: done (2026-10-06)

Part 1 of 4 of the vendor research (incumbent enterprise AML suites). Other files: 08 NICE Actimize, 10 SAS AML, 11 other
incumbents and the comparison table. Requirement IDs use the prefix VA-xx and are numbered across all four files.

## Contents

1. Ownership and history (i-flex/Mantas heritage)
2. Product modules
3. Architecture and deployment
4. Detection approach
5. AI and GenAI features to 2026 (Compliance Studio, agentic)
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

## 1. Ownership and history (i-flex and Mantas heritage)

| Date | Event | Source |
|---|---|---|
| 1990-92 | i-flex Solutions grows out of Citicorp's software arm (Flexcube core banking) | Wikipedia (secondary) |
| Aug 2005 | Oracle buys Citigroup's 41% of i-flex for USD 593m; raises stake in 2006-07 to about 83% | Wikipedia citing 2005-07 press |
| Aug 2006 | **i-flex acquires Mantas**, a US anti-money-laundering and compliance software company, for **USD 122.6m** (part-funded by a share allotment to Oracle). Mantas is the origin of Oracle's behavior-detection engine | Wikipedia (secondary) |
| Apr 2008 | i-flex renamed Oracle Financial Services Software (OFSS); today a listed Indian subsidiary of Oracle that builds the banking and analytical applications; FY2024 revenue INR 6,373 crore (about USD 660m), 8,754 staff (whole company, not FCCM) | Wikipedia |
| 8 Jun 2021 | Oracle stops offering Trading Compliance, Broker Compliance (and Enterprise Editions) and Trade Blotter; unsupported from release 8.1.1.1 | FCCM 8.1.2.12 release notes |
| From release 8.1.1 | Enterprise Case Management becomes mandatory for alert review: "the dispositioning of alerts through Alert Management (AM) is no longer supported"; a restricted-use ECM licence comes with the BDF licence (release year not stated in the notes) | Release notes |
| 2024-26 | **FCCM Cloud Service** (modular SaaS on OCI; What's New books exist from release 24.2.1), Compliance Agent (datasheet 2024), FCCM Studio cloud, AI recommendations in cases (25.11.1, Nov 2025), AI Investigator features (26.2.1-26.5.1), Nexus (page live Oct 2026; launch date not found) | Oracle docs "What's New" |
| 13 Jul 2026 | On-premises Behavior Detection release 8.1.2.12.0 (semi-annual cadence: 8.1.2.6 Oct 2023, .7 Feb 2024, .8 Aug 2024, .9 Feb 2025, .10 Aug 2025, .11 Feb 2026, .12 Jul 2026) | Oracle docs |
| 12 Aug 2026 | Cloud service release 26.8.1 (quarterly-style cadence: 24.2, 24.3, 24.5, 24.6, 24.8, 24.11, 25.1, 25.3, 25.5, 25.8, 25.11, 26.2, 26.5, 26.8) | Oracle docs |

Heritage is visible in the code: the 2026 installer still ships `mantas8.1.2.12.0_delta.cfg`, a `mantas_schema`,
`KDD_*` batch tables (from Mantas's "knowledge discovery in databases" origin) and a `mantas.war`.

**Scale (Oracle claims).** "25+ years of fighting financial crime"; "working with 180+ global financial institutions"
(2025 datasheets; 130 production AML implementations in the 2020 datasheet; "150+" in the 2021 screening datasheet).
For comparison NICE claims 1,000+ organisations (all products, all sizes). Oracle does not disclose FCCM revenue; it is
inside Oracle's Industries and OFSS figures.

**Ownership stability.** Oracle is not selling FCCM; the product line has outlived Mantas's founders by twenty years and
Oracle has retired products (Trading Compliance, Alert Management, Scenario Wizard) without selling them off. That
stability is a selling point against NICE Actimize in 2026-27, but it comes with Oracle's habit of forcing migrations
(section 12).

### AKR-TMS hooks (section 1)

- A bank should never have to accept a forced product migration: AKR-TMS commits to long-lived LTS releases and
  published deprecation periods with a migration tool (VA-34; upgrade safety is VA-25 in file 08).
- Document the origin of every detection method; for AKR-TMS the lineage is CBI scenario text -> rule definition ->
  threshold set (VA-27).

## 2. Product modules

Oracle sells "Financial Crime and Compliance Management (FCCM)" as an on-premises suite (OFSAA-based) and as separately
subscribed cloud services. Names as of Oct 2026 (Oracle product and documentation pages):

| Area | On-premises product (OFSAA) | Cloud service (OCI) | Notes |
|---|---|---|---|
| Transaction monitoring | Anti Money Laundering (Behavior Detection, scenario library, Inline Processing Engine for real time); Trade-Based AML; Real-Time Fraud; Currency Transaction Reporting | FCCM Transaction Monitoring Cloud Service (scenarios run as "pipelines" on the Data Transformation Platform) | Event Scoring (ML scoring of events/alerts) |
| KYC / CDD | Know Your Customer (risk assessment, perpetual KYC, real-time risk API) | FCCM Know Your Customer Cloud Service | "Straight-through processing" for low-risk customers, CDD/EDD routing for others |
| Screening | Customer Screening (on Oracle Enterprise Data Quality), Universal Customer Screening, Transaction Filtering | Customer Screening CS, Transaction Filtering CS (real-time SWIFT MT and ISO 20022; keyword extraction) | 450+ match rules; list connectors (OFAC, HMT, EU, UN, Factiva, Thomson Reuters/World-Check, Dow Jones) and private lists |
| Case management | Enterprise Case Management, Investigation Toolkit | **Investigation Hub** (iHub) cloud; **Oracle Nexus** = Case Flow (cloud case manager) + Reach (browser extension that adds AI to Oracle, non-Oracle and on-prem case tools) | Nexus announced 2026 |
| Reporting | Compliance Regulatory Reporting, FATCA, CRS, CTR | Regulatory Reporting CS: US SAR, CTR, AUSTRAC, Philippines CTR/STR, **goAML STR application** | See section 6 |
| Analytics | Compliance Studio (graph, ML, scenario authoring; Spark, Zeppelin, R, Python) | FCCM Studio CS; Automated Scenario Calibration CS; **Compliance Agent CS** (simulation) | |
| Dashboards | OFSAA analytics (OAS 25.0) | FCCM Monitor CS (persona dashboards, e.g. CAMLO) | |
| Infrastructure | OFSAAI, Model Management and Governance, Data Foundation (FSDF CSA) | Data Foundation Cloud Service (DFCS), Data Transformation Platform | |

Oracle's own pitch contrasts with NICE: "While most solutions rely on extensive data ingestion - pulling in every
available database and watchlist - we focus on intelligent data usage that's both cost-effective and secure" (2025
Cloud Service datasheet), and it attacks "post-sale customizations ... a complex, customized project that delays your
actual go-live date and inflates costs" with a "truly SaaS", "no-code/low-code" message.

## 3. Architecture and deployment

**On-premises (the version most relevant to Iraq).** From the Behavior Detection Administration Guide and installation
guide, release 8.1.2.12.0 (13 Jul 2026):

- Tiers: OFSAA infrastructure (OFSAAI: security, batch scheduler, metadata), the Behavior Detection application pack
  (AML, fraud, CTR and KYC solution sets) and the BD platform with three subsystems: **Data Management** (Ingestion
  Manager; the "Data Interface Specification" (DIS) flat-file adapters; "datamaps" that derive and aggregate data;
  fuzzy name matcher against the watch list), **Behavior Detection** (pattern engine; scenarios grouped in scenario
  classes) and **Alert Viewer / Enterprise Case Management** (web tier).
- Data load: either the OFSAA **Common Staging Area (CSA)** or DIS flat files into the Financial Services Data Model
  (FSDM). "The Oracle client implements Ingestion Manager by setting up a batch process ... typically ... AAI Batch
  Scheduler" (the bank builds the batch).
- Cadence: "OFSBD routinely generates alerts ... typically nightly, weekly, monthly, and quarterly".
- Real time: the **Inline Processing Engine** "evaluates and assesses scores on incoming data in real-time, near real-time
  or batch to make quick decisions and to hold or release transactions" (2020 AML datasheet). The BD REST API in the
  2026 guide only loads "Trusted Pair" and "Alert Suppression" data; no public REST scoring API for the AML engine
  appears in the on-premises documentation (the cloud KYC service has a real-time risk-assessment API, 26.5.1).
- Technology stack (qualification matrix, 8.1.2.12): Linux 8 or 9 (Solaris dropped), **Oracle Database 19c or "23.26" (as written in the matrix)**,
  Java 8, 11 or 17, WebLogic 12.x / 14.1.x / 15.1, Tomcat 9/10; OFSAA 8.1.2.6.0; Oracle Analytic Server 25.0 for the
  analytics dashboards. The notes contradict themselves in two places: the matrix lists WebSphere 9 but a note says
  WebSphere is not supported with 8.1.2.12.0, and the notes say BD supports HTTP 1.1 and not HTTP/2 while the matrix
  shows HTTP2 for some stacks. Components can run on one machine
  or be distributed ("horizontally and vertically scalable").
- Scenario authoring: Scenario Manager; sequence-scenario design tips in the 2026 notes ("select an unusual
  occurrence to serve as the initial event ... reduces the number of match states"; "apply logic to filter [records]
  out up-front"; sort datasets) show that scenario performance is hand-engineered by the bank's analysts.

**Cloud.** FCCM Cloud Service runs on **Oracle Cloud Infrastructure**: the customer creates a tenancy and chooses a
**home region** that "cannot be modified" after provisioning, plus a DR region (Getting Started guide, 26.08.01). Data
loading in the cloud uses the Data Foundation Cloud Service / Data Transformation Platform pipelines. OCI has no Iraq
region; cloud use by an Iraqi bank is a data-residency decision under the CBI's rules (file 01 section 8).

**Practitioner statements.** A reviewer who runs three products: "Oracle's performance isn't good, and it's very slow.
They have issues. They have to optimize it. ... Oracle's engine is excellent and very powerful ... Oracle has many
manual deployments and implementations" (PeerSpot, senior project lead at a 10,001+ employee firm). The one PeerSpot
review of the cloud service: "The UI's performance is very slow, and the processing time is very high when handling a
huge amount of data."

### AKR-TMS hooks (section 3)

- The single biggest Oracle-style burden is that "the bank builds the batch" (DIS files, CSA staging, schedulers).
  AKR-TMS must own the read-only extract and the canonical mapping (VA-03 in file 08).
- Keep the stack small and open: PostgreSQL rather than a licensed database, one runtime, no application-server
  zoo; publish the minimum hardware per bank size (VA-21 in file 08).
- Provide both a daily full run and a streaming path so alerts exist within minutes of the transaction (VA-24
  in file 08).

## 4. Detection approach

**Scenario library (rules).** AML scenarios are "pre-configured, out-of-box, industry-recognized" with "documentation for
each scenario for transparency into behavior detection logic"; "proven scenarios currently in production globally over
15 years that can be easily defined as various rules to enable business user driven management without IT support"
(2020 datasheet). Scenarios run as "patterns" over datasets (the 2026 notes describe sequence patterns with "match states" and
aggregate bindings: Max, Min, Sum, Avg, CountDistinct, Count) with a **focus** (for example customer or account,
with "focus clustering"), thresholds held in versioned **threshold sets** per segment, and **scenario classes**. But **the Anti-Money Laundering Technical Scenario Description and the data model are only on My
Oracle Support "due to confidentiality reasons"** (docs library note), so a prospective bank cannot read the logic
without a support login.

**Segmentation and thresholds.** Customer segments and threshold sets per segment; Scenario Manager for editing;
**Automated Scenario Calibration** (cloud service; launch date not found): structured below-the-line / above-the-line analysis, alert-volume
comparison "by run date, by segment, by jurisdiction", versioned threshold sets; claim "reduces manual tuning effort
by 80%". **Alert Analyzer** (26.8.1, Aug 2026): "fast, targeted evaluation of how a scenario pipeline behaves for a
specific entity, without the need to run a full batch ... pinpoints the configuration responsible".

**Behavioral models and event scoring (ML).** "Behavioral models are supervised machine learning models that detect
suspicious activity by learning patterns from historical labeled outcomes (for example, investigator dispositions)";
needs a modelling dataset, a label field and a stable entity identifier; evaluated by AUC and out-of-time samples;
lifecycle train -> publish -> approve -> deploy with model IDs. Claims: "advanced behavioral models replace traditional
rules, cutting false positives by over 60%"; "event scoring ... reducing alerts by up to 50%"; "more than 60% fewer
unnecessary alerts, 2X ROI" (Oracle AML page). No customer-level evidence was published for these figures in the
documents read; they should be treated as marketing.

**Graph and entity resolution.** Oracle Parallel Graph Analytics (PGX) with 30+ algorithms and a SQL-like graph query
language, Link Analysis network definitions, "shared entity intelligence" in Nexus, "advanced entity resolution" in the
cloud suite; customer screening runs on Oracle Enterprise Data Quality.

**Simulation, not just tuning: Compliance Agent.** A distinct Oracle idea (2024 datasheet): deploy "an AI money launderer
which moves money through a simulated environment where a financial institution's products, customer segments and
controls have been recreated", then measure which scenarios catch it, find gaps and overlaps, estimate alert volumes of
a change "without running expensive simulations on historical data", and "assess the effectiveness of controls against
high-risk typologies, including human trafficking". Oracle frames scenarios as "Tier I models" that model-risk teams
must challenge, with "empirical evidence" for each threshold.

**Governance claims (Oracle AML page, 2026).** "Versioning, approvals, rollback, explainability, drift monitoring, lineage,
role-based access control, attribute-based access control, and operational reporting".

### AKR-TMS hooks (section 4)

- Publish the scenario logic to the bank in readable form (Arabic and English) with parameter definitions, data inputs
  and worked examples; never put scenario descriptions behind a support login (VA-27).
- Build **entity-level scenario testing** (Alert Analyzer idea): "why did or didn't this customer alert?" with the
  configuration shown, without a full batch (VA-28).
- Build a **typology replay harness**: inject synthetic laundering patterns drawn from the CBI scenario catalogue into a
  copy of the bank's profile, measure which scenarios fire, and keep the results as validation evidence
  (VA-29).
- Version every threshold set; compare alert volumes by segment and date before approval (VA-07 in file 08).

## 5. AI and GenAI features to October 2026 (Compliance Studio, agentic)

| Feature | Oracle description | Date and status | Evidence quality |
|---|---|---|---|
| Behavioral models (Compliance Studio / FCCM Studio) | Supervised ML on labelled outcomes (investigator dispositions); AUC and out-of-time validation; train -> publish -> approve -> deploy | Studio brief 2021; FAQ 2026 | Vendor |
| Event scoring | ML scores on alerts/events to prioritise; "reducing alerts by up to 50%" | 2025 datasheets | Vendor claim |
| Automated Scenario Calibration | Below/above-the-line analysis, alert-volume comparison by segment, jurisdiction and run date, versioned threshold sets; "reduces manual tuning effort by 80%" | Cloud service (docs list it; launch date not found) | Vendor claim |
| Alert Analyzer | Entity-level test of scenario pipelines without a full batch | 26.8.1, Aug 2026 | Release note |
| **Compliance Agent** | "AI money launderer" moved through a simulated environment of the bank's products, segments and controls; what-if analysis; gap/overlap analysis; alert-volume estimates without historical simulations; SaaS, metered by "number of experiments run" | Datasheet v2.1, 2024 | Vendor |
| **AI Investigator** (Investigation Hub) | Multi-agent system: forms a hypothesis "using the 5 W's framework", searches and collects evidence across sources, reviews prior history, recommends decisions, writes the narrative; humans agree or disagree, add inputs, edit, and "set up auto decisioning (auto-close/auto-escalate) for similar investigations". Claims: "over 80% reduction in investigation time", "Gen AI initiatives improve investigator efficiency by 60%" | AI recommendations in AML and screening cases from 25.11.1 (Nov 2025); customer bio, prior-case summary, green flags, reading red-flag/typology mapping from object store (26.2.1, Feb 2026); customer profile memory, configurable case types, agent decision-tree UI, chatbot UI inside iHub (26.5.1, May 2026) | Vendor claim; PDF carries a "Confidential - Oracle Internal" footer |
| AI Assistant Cloud Service (AIACS) | AI-generated case narratives; **licensed per "number of cases having AI generated narrative"** | Cloud metering page, 26.05.01 | Oracle documentation |
| GenAI in name matching | Sanctions screening extracts the full name with GenAI and matches it as one entity rather than keyword by keyword ("David Mark Smith" vs separate tokens); "significantly decreases the likelihood of incorrect matches" | Feb 2026 release 26.2.1 | Release note; no numbers |
| Oracle Nexus | Case Flow (SaaS case manager) and **Reach** (a browser-based extension that brings AI-assisted investigation into existing Oracle, non-Oracle and on-premises case systems without replacing them) | Page live Oct 2026 | Vendor FAQ |
| Public Domain Search, adverse media improvements | Open-source/adverse media lookup inside cases | 25.11.1-26.2.1 | Release note |

**Governance statements.** The Nexus FAQ says investigators "remain accountable", can "review, edit, accept, reject, or
override" outputs, that case records "preserve source references, time stamps, investigator edits, approvals, overrides,
and the final disposition", and lists monitoring measures (recommendation acceptance and override rates, narrative editing
rates, exception volumes, QA rework). On training data it is noncommittal: "The treatment of institutional and customer
data depends on the applicable Oracle service, deployment model, security architecture, and contractual terms"
(contrast NICE's flat "No Training" claim, file 08). Automated closure or escalation is allowed "where certain actions
may be automated or recommended based on approved policy", with the institution defining case types, review rules and
override rights.

**What is not documented.** Which LLMs run the agents, where, and whether any of this works on-premises. The agent
features sit in cloud services (Investigation Hub, AI Assistant, Compliance Agent is "a SaaS application"); the on-premises
8.1.2.12 release (July 2026) lists as its only new AML features "scenario bindings enhancements" for three scenarios.
**Reading: an on-premises Oracle bank gets the engine, not the AI.** Nexus Reach (browser extension) is the only route
described that adds AI to on-premises systems, and it still needs a connection to Oracle's service.

### AKR-TMS hooks (section 5)

- The simulation idea is the most original thing in this file: ship a **controls testing harness** (synthetic launderers
  built from CBI and FATF typologies) that produces coverage evidence for the MLRO, the auditor and the CBI
  (VA-29).
- Offer **AI assist as an overlay**: Oracle's Nexus Reach shows that banks want AI without replacing case management.
  AKR-TMS can offer its assistant on top of the bank's existing alert source (via alert import, VA-13 in file 08).
- Meter nothing per AI narrative: AI features must be part of the licence, otherwise cost rises with use (VA-20
  in file 08).

## 6. Case management and regulatory reporting (goAML)

**Case management.** On-premises: Enterprise Case Management (mandatory since release 8.1.1; restricted-use ECM licence
bundled with Behavior Detection) and the Investigation Toolkit. Cloud: **Investigation Hub** (queues with attributes,
case merging for AML cases, RFIs with participant support, dossier generation, graph visualisation with rule-based styling,
correspondent-banking entity views and "Institutional Money Laundering" support, real-time customer-screening events
with pre-case disposition, workflow and case designers, substitution variables for narratives and RFI templates) and
**Nexus Case Flow**. Licensed by pooled named user.

**Regulatory reporting (the strongest differentiator against NICE in this research).**

| Item | Detail |
|---|---|
| Report packs in the cloud service | US SAR, US CTR, AUSTRAC (Australia), Philippines CTR and STR, and a **goAML STR application** |
| goAML application | Created from a case with the action "Generate goAML STR"; analyst opens, analyses, requests approval, approver approves or rejects; rework; due date; "Generating E-File"; "Submitting the Reports" (user guide dated 14 Aug 2026) |
| Admin UI (26.5.1, May 2026) | "configure and manage jurisdiction-specific goAML XSD templates aligned with the appropriate schema and process versions"; assign the goAML process version per country - "**Currently, the application supports only v5.0.2**"; associate each template with an e-file XSD; "automatically generate and configure the UI when an XSD is uploaded" |
| Workflow (26.8.1, Aug 2026) | automatic e-file generation, Activity tab, supervisor **reject workflow**, internal-party search and add, **report preview**; central **Validation Errors** tab (26.5.1) with in-place correction |
| Data capture (26.8.1) | means of transport; related entity, related account, related person; network devices (IP, device) on accounts, entities and persons; employment history; the guide's person form has "Mother's Name" and "Full Foreign Name ... in a foreign language or script" |
| Older datasheet (2021) | goAML "widely used in Middle Eastern, Asian, and European countries"; "each country regulator may have specific requirements that warrant customizations to the template ... utilizing consulting services. This product is country specific" |
| Licensing | Regulatory Reporting CS is metered by "number of templates configured of a country or jurisdiction for filing to a regulator" |

**For an Iraqi bank.** Oracle is the only incumbent in this research whose public documents show a working,
XSD-driven goAML STR workflow with maker-checker, validation and e-file generation. Limits: version 5.0.2 only (the
Iraqi FIU's version must be checked, file 23), the guide default locale is US English, and each jurisdiction template
is a separately licensed unit. The report is built from a case, not from the Iraqi internal-report chain (branch liaison
officer, acknowledgement, MLRO decision), which the product does not model.

### AKR-TMS hooks (section 6)

- Match Oracle's goAML functions and exceed them: schema-version-aware templates loaded from the XSD, a validation
  error list that links to the field, preview, e-file, status and rejection from the FIU, and Iraq's extra
  attributes; support several goAML versions at once (VA-30).
- Do not meter or license per jurisdiction template; one price covers all reports a bank must file
  (VA-20 in file 08).

## 7. Data model and core banking integration

**Models.** OFSAA's Financial Services Data Model (FSDM) and Financial Crimes Data Model; a Common Staging Area (CSA)
under the Financial Services Data Foundation (FSDF 8.1.2.4 or 8.0.8 supported); in the cloud, the Data Foundation
Cloud Service and the **Data Transformation Platform** (pipelines with "persist", "create event", "business filter" widgets
and audit history). The data model and scenario technical descriptions are available "only on My Oracle Support".

**How data gets in.**
- On premises: DIS flat files through Ingestion Manager, or CSA tables through BD datamaps; the bank implements the batch
  (AAI Batch Scheduler); datamaps derive balances and aggregates; Hive-to-Table load with "over 3,000 data quality
  checks" for big-data deployments (2020 datasheet).
- Cloud: the bank's ETL exports **daily CSV files** to "specific templates", uploads them via a pre-authenticated URL to
  Oracle Object Storage (optionally **AES-256-CBC encrypted**; split above 100 MiB; the standard bucket keeps data for
  7 days, the archive bucket for one year), then runs the staging batch. The Aug 2026 loading guide lists **67 staging
  templates**: 26 account tables (one per product family: current/savings, cards, loans, annuities, leases, merchants,
  retirement, swaps, term deposits, trusts, money market, trading, overdraft, correspondent, repo ...), 18 customer/party
  tables, 17 transaction tables (again one per product family), 4 insurance and 2 watch-list tables.
  **No Islamic product (murabaha, ijara, mudaraba, sukuk) appears in the account, contract or transaction templates**
  (searched for the terms; no hit).
- **Unified Data Loading (26.2.1, Feb 2026)** exists because the old way was heavy: it is "to help on-premise customers who
  currently use flat file ingestion to migrate to a more structured ETL system ... Currently, this transition is complex
  and requires effort to map multiple product Processor tables. DFCS now accepts a single Account table and a single
  Transaction table".
- Delta loads need "seeding table" maintenance (e.g., changes to the phone master need matching changes in the
  account-phone and party-phone maps).
- **No core banking adapters** are named in the documentation read (the words FLEXCUBE, Temenos, T24, Finacle do not
  occur in the Behavior Detection admin, install or API guides). Oracle FLEXCUBE is Oracle's own core banking product
  (in the same documentation library, including a FLEXCUBE Islamic Loan Origination guide) but no FLEXCUBE-to-FCCM
  mapping is documented publicly. Integration is a data-warehouse-style project run by the bank or an integrator.
- The "zero costly data migrations" claim in the 2025 datasheets means FCCM can sit on the bank's existing data
  architecture; the guides show that the bank still prepares product-specific extracts in Oracle's templates.

### AKR-TMS hooks (section 7)

- A **product-agnostic canonical model**: one `account`, one `transaction`, one `party`, with `product_family` and
  typed extension tables (including Islamic contract types and profit-rate events), never one table per product
  (VA-31).
- Publish the canonical schema and the mapping rules to the bank (open documentation), as the opposite of "available
  only on My Oracle Support" (VA-27).
- Mapping packs per core banking system with a validation report (row counts, null rates, reference-data coverage)
  produced automatically at the first load (VA-17 and VA-15 in file 08).

## 8. Arabic/RTL support and presence in MENA and Iraq

| Question | Finding | Confidence |
|---|---|---|
| Arabic UI or RTL | No Arabic or RTL statement in any FCCM datasheet, release note, or the BD admin, install, user, API and cloud guides read (38 Oracle documents, about 2.8 million characters, searched for "Arabic", "right-to-left", "transliteration"; the only product hit is the name-matcher limit in the next row). The cloud service has a user "Locale" preference ("By default, it is US English") and 26.5.1 added "Expanded Localization Support: additional interface text is now available for localization" without naming languages. Oracle's wider platform (OJET) supports bidirectional text, but nothing FCCM-specific was found | Absence of evidence; ask in any RFP |
| Arabic name matching | **The on-premises Behavior Detection "Fuzzy Name Matcher Utility" (release 8.1.2.12.0, 13 Jul 2026, Administration Guide appendix E.5.1) "supports matching on ASCII, extended ASCII, AND the first 128 encoded characters of the UTF-8 character set ... Any UTF-8 characters beyond this (such as Chinese, Arabic, and so on) will be ignored (will not cause the engine to crash, but such names will not match)".** It is the utility used "during BD Datamap processing ... to match names of individuals and corporations (candidates) against a list of names (targets)". **The newer Customer Screening Cloud Service is different**: its matching guide (26 Aug 2026) documents synonyms ("Mohamed" = "Muhammad"), token normalisation (hyphens, apostrophes) and **transliteration between non-Latin scripts such as Arabic or Cyrillic and Latin**, switched on per attribute through a `translateFields` array in the index JSON; candidate selection keeps "the top 1000 candidate records per watchlist"; scoring uses character and word edit distance, match percentages and Metaphone (a phonetic code designed for English). Older datasheet: Oracle Enterprise Data Quality with "over 450 standard match rules" (2021). No Arabic test results or Iraqi/Kurdish name handling are published. The goAML form has "Full Foreign Name ... in a foreign language or script" and "Mother's Name" | High for the BD matcher limit (quoted); medium for cloud screening (documented capability, no accuracy data) |
| Islamic banking | No Islamic account/contract/transaction template, scenario or case type found | High that it is not provided out of the box |
| Presence in MENA | Oracle's "global contacts" page lists local Arabic/English lines for **Bahrain, Egypt, Jordan, Kuwait, Lebanon, Oman, Qatar, Saudi Arabia, UAE and Yemen. No Iraq entry**. OCI regions in the region: Saudi Arabia (Jeddah, Riyadh) and UAE (Dubai, Abu Dhabi) | High |
| Customers | The AML page shows three logos: Crédit Agricole, Ailos (a Brazilian cooperative system) and "Aman Bank" (the page does not say which; Libya's Aman Bank is the likely match, **unverified**). The 2014-era Oracle datasheet quotes JPMorgan Chase's CEO on using OFSAA for "all U.S. dollar payment transactions" | Low |
| Iraq | No public Iraqi reference, partner or office found for FCCM | Medium |
| Courses and community | Oracle University learning path for OFSAA; FCCM customer community; Oracle partner network (Accenture, Deloitte and Indian SIs are the usual implementers; not verified here) | Low |

**Reading.** Oracle is a plausible choice for a large Iraqi bank that already runs Oracle FLEXCUBE or Oracle databases
and can fund a multi-partner project, and it is the only incumbent documenting goAML. But the on-premises Behavior Detection
name matcher cannot match Arabic-script names (stated in Oracle's July 2026 guide), the Arabic-aware transliteration
lives in the cloud screening service, nothing shows Arabic-first operation, Iraqi data or Islamic products, and the cloud
route needs a Gulf region for data hosting. Iraq's national list is Arabic-only (file 01, section 6), so a bank on the
on-premises engine would have to transliterate every party name to Latin characters in the data feed first, which is
exactly where Iraqi transliteration variance creates false negatives (file 22).

### AKR-TMS hooks (section 8)

- Offer what Oracle does not document: a published Arabic and RTL UI with Arabic STR narratives, Arabic name matching
  evidence (precision and recall on Iraqi test names), and Islamic product scenarios (VA-18 and
  VA-26 in file 08).
- Give an integration path for banks that run Oracle FLEXCUBE (read-only DB access to its tables) (VA-17 in
  file 08).

## 9. Implementation time, cost and licensing signals

Oracle does not publish prices for on-premises FCCM or for the SaaS list. What is public:

| Signal | Source | Reliability |
|---|---|---|
| **Cloud licence metering** (Oracle documentation, release 26.05.01): Transaction Monitoring = "number of transactions per month"; Regulatory Reporting = "number of templates configured of a country or jurisdiction for filing to a regulator"; KYC = "count of KYC checks created" (onboarding and continuous monitoring); Customer Screening = customers screened by batch per day plus real time over a month; Transaction Filtering = real-time messages screened per month; Investigation Hub, Monitor, iHub real-time screening = **pooled named users**; Studio and Automated Scenario Calibration = transactions (per month); Compliance Agent = "number of experiments run"; **AI Assistant = "number of cases having AI generated narrative"** | Oracle docs | High: a usage-based, multi-meter licence. A bank pays per transaction, per screening, per user, per jurisdiction template and per AI narrative |
| Cloud account model: tenancy with a fixed home region plus a DR region; Object Storage buckets; daily CSV load | Oracle docs | High |
| On-premises stack: Oracle Database 19c or 23.26, WebLogic or Tomcat, Java, Oracle Analytic Server 25.0, OFSAAI; scenario work done by the bank's analysts with Scenario Manager | Release notes and guides | High for components; licence terms not found |
| "I think Oracle is very expensive" (cloud service reviewer, PeerSpot) | PeerSpot | Low (1 review) |
| "Oracle has many manual deployments and implementations" (reviewer who runs Actimize, Oracle and Norkom) | PeerSpot | Medium |
| "has been deployed in over 130 distinct production implementations globally" (2020) and "180+ global FIs" (2025) with "predictable deployment with predefined data sourcing and scenario functionality" | Oracle datasheets | Marketing |
| Oracle's own marketing: SaaS avoids "post-sale customizations ... a complex, customized project that delays your actual go-live date and inflates costs" | Cloud datasheet 2025 | Marketing, but an admission of what on-premises projects look like |
| Unified Data Loading (Feb 2026) exists because "this transition is complex and requires effort to map multiple product Processor tables" | Release 26.2.1 | High |

**No public implementation duration or price was found for Oracle.** Reasonable inference from the 67-template data
load, the bank-built batch and the partner-led delivery model: this is a multi-quarter integration project before the
first production alert, comparable to or heavier than Actimize; treat as an estimate to be confirmed in RFPs.

**Implementation partners.** Not named by Oracle in the documents read. Oracle University and a customer community are
listed; Oracle Consulting and the Oracle Partner Network are named in the release notes as the route for training on
the mandatory ECM migration.

### AKR-TMS hooks (section 9)

- A usage-metered licence (transactions, screens, users, templates, AI narratives) makes cost unpredictable and
  punishes growth; Iraqi state banks with large low-value volumes would pay the most. Use a flat annual price per
  bank tier with every module (VA-20 in file 08).
- Show the first-alert timeline honestly: data profiling in week 1-2, mapped data in week 3-4, CBI scenario pack
  running in week 5-6, tuning from week 7 (targets to be validated in the first deployment) (VA-33).

## 10. Analyst rankings

| Firm / report | Year | Result for Oracle FCCM | Note |
|---|---|---|---|
| Chartis RiskTech100 | 2020-2024 editions | "Category Leader for AML" in each of 2020, 2021, 2022, 2023 and 2024 | Oracle datasheet claim (copyright 2025); no 2025-2027 category claim found. A search summary placed Oracle among the top-ranked vendors in RiskTech100 2026 (Oct 2025) but the rank is paywalled |
| Chartis Entity Management and Analytics Solutions | 2022 | Category Leader | Oracle claim |
| Chartis KYC report | 2020 | Leader | Oracle claim (screening datasheet) |
| Chartis Financial Crime and Compliance50 (FCC50) | 2025, 2026 | Oracle appears in the published vendor list of both editions (published 19 Sep 2025 and 9 Mar 2026) | Rank paywalled; NICE claims first place in the 2024 edition and again in the next |
| QKS Group (Quadrant) SPARK Matrix | 2021, 2022 | "Technology Leader for AML" (2022, 2021); Leader for KYC/CDD (2021) | Oracle claim |
| Forrester Wave: AML Solutions | Q2 2025 | **Not found**: the release pages of NICE, SAS, SymphonyAI, DataVisor and Hawk name no Oracle result; whether Oracle was among the 15 evaluated vendors is unverified | Qualifying rule: USD 10m AML revenue, standalone SKU |
| Forrester Wave: Financial Crime Management Solutions | Q3 2026 | **Not found**: no Oracle announcement located; SAS reports 4.70/5 (highest current offering) among 15 firms (SAS release, 24 Aug 2026) | Oracle's placement unknown |
| PeerSpot | Oct 2026 | Oracle FCCM Cloud: 4.0/5 from 1 review; "#2" ranking and AML mindshare 4.7% (from 10.3% a year earlier) | Too few reviews to mean anything |
| Gartner Peer Insights, G2, TrustRadius | - | Not accessible (bot checks / HTTP 403) | |

## 11. What it does right (with evidence)

1. **Real-time interdiction in the core engine.** The Inline Processing Engine scores incoming data "in real-time, near
   real-time or batch ... to hold or release transactions" (2020 datasheet). This matches the CBI requirement to stop
   a transaction until the MLRO clears it (file 01, 2d); NICE's SAM is batch.
2. **A documented goAML STR application** (2026) with per-country XSD templates, validation list, preview, maker-checker,
   e-file generation, reject workflow, plus AUSTRAC, Philippine and US forms (section 6).
3. **Transparent, business-editable scenario model** at its best: threshold sets, versioned, documented patterns,
   Scenario Manager, and (cloud) Alert Analyzer and Automated Scenario Calibration with below/above-the-line analysis.
4. **Control-testing innovation**: Compliance Agent (AI money launderer) gives model-risk teams "empirical evidence" for
   thresholds without historical simulations; Oracle frames scenarios as Tier I models.
5. **Data-quality discipline at ingestion**: "over 3,000 data quality checks" (Hive load, 2020 datasheet), a data-quality
   rule library, audit history on Data Transformation Platform pipelines (25.8.1, 26.5.1).
6. **Open analytics tooling**: Apache Spark, Zeppelin, R, Python, polyglot scenario authoring, in-memory graph (PGX, 30+
   algorithms), model lifecycle with champion/challenger by AUC.
7. **Governance claims that include rollback and drift**: "Versioning, approvals, rollback, explainability, drift
   monitoring, lineage, RBAC, ABAC".
8. **Cloud-native cadence and visible release notes**: quarterly cloud releases (24.2 to 26.8), semi-annual on-premises
   releases, public documentation for most guides.
9. **Corporate stability and ecosystem**: Oracle itself, OFSS, the Oracle FLEXCUBE installed base, global SIs.
10. **Phased modernisation without rip-and-replace**: "AML transformation can be completed in phases"; Nexus Reach adds AI to
    existing case tools.
11. **Practitioner view of the engine**: "Oracle's engine is excellent and very powerful" (PeerSpot reviewer, same review
    that criticises its speed).

## 12. What it does wrong (with evidence)

| # | Weakness | Evidence |
|---|---|---|
| 1 | **Heavy stack and bank-built batch** | OFSAAI + Oracle DB + WebLogic/Tomcat + OAS; Ingestion Manager and DIS files; "The Oracle client implements Ingestion Manager by setting up a batch process"; alerts "nightly, weekly, monthly, and quarterly" |
| 2 | **Rigid, product-specific data model** | 67 CSV staging templates, one account and one transaction table per product family; Oracle's own 2026 admission: "complex and requires effort to map multiple product Processor tables"; no Islamic products |
| 3 | **Name matcher blind to Arabic (on-premises BD)** | Administration Guide E.5.1: Arabic characters "will be ignored ... such names will not match" |
| 4 | **Slow performance reported** | "Oracle's performance isn't good, and it's very slow"; "UI's performance is very slow, and the processing time is very high when handling a huge amount of data" (PeerSpot) |
| 5 | **Cost and meters** | "very expensive"; multi-meter licence (transactions, screens, users, templates, AI narratives); Oracle Database dependency on premises |
| 6 | **Forced migrations and retired features** | Alert Management no longer allowed for alert disposition from 8.1.1 ("customers will be out of compliance with their support contract" if they keep using it); mandatory ECM (restricted licence); Trading/Broker Compliance withdrawn 8 Jun 2021; Scenario Wizard decommissioned in 8.1.2.6.0 |
| 7 | **Opaque documentation** | "Due to confidentiality reasons, the Data Model and Technical Scenario Description documents are available only on My Oracle Support" |
| 8 | **Two-speed product** | Compliance Agent is "a SaaS application"; AI Investigator, AI narratives and Nexus are cloud services; on-premises 8.1.2.12 (Jul 2026) lists only three scenario-binding enhancements under AML; docs contradict themselves (WebSphere, HTTP/2) |
| 9 | **Manual deployments** | "Oracle has many manual deployments and implementations" (PeerSpot) |
| 10 | **AI claims without evidence; unclear data use** | ">60%", "80%", "over 80%" figures without customer data; Nexus FAQ on training data: "depends on the applicable Oracle service ... contractual terms"; the AI Investigator PDF is marked "Confidential - Oracle Internal" |
| 11 | **Per-country goAML limits** | goAML application "supports only v5.0.2"; each jurisdiction template is a metered, separately configured unit; customisation through consulting |
| 12 | **Thin independent evidence and no local presence** | One PeerSpot review of the cloud service; no Iraq office or partner; Gartner/G2/TrustRadius not readable |
| 13 | **Data residency** | Cloud home region fixed at provisioning; nearest OCI regions Saudi Arabia and UAE; nothing in Iraq |

No enforcement document read names Oracle as the cause of a failure. (TD Bank's case names SAS; the FCA and CSSF cases
name no vendor; see files 08 and 10.)

## 13. Lessons for AKR-TMS

**Copy**

- Real-time hold path next to batch monitoring, with per-scenario control over what may hold a transaction (VA-24 in file 08).
- Versioned threshold sets with below/above-the-line analysis and an entity-level "why did it (not) alert" tester.
- A **controls-testing harness** with synthetic launderers (Compliance Agent idea).
- A goAML application: XSD-driven templates, central validation list, preview, e-file, reject workflow, maker-checker.
- Phased adoption: let a bank start with alert import and AI assist, and replace components later.
- Pipeline audit history and data-quality checks at ingestion.

**Avoid**

- Product-specific table sprawl (67 templates) and daily CSV hand-offs; use one canonical model with extensions.
- A scenario catalogue behind a support portal; a data model behind a login.
- Usage meters (per transaction, per screen, per AI narrative, per template).
- Forced migrations (retiring a feature that banks use) without a multi-year overlap and a migration tool (VA-34).
- An Arabic-blind component anywhere in the name path; test every matching component with Arabic and transliterated names
  before go-live (VA-32).
- AI that exists only in a cloud the bank cannot use.

**Beat it on**

| Dimension | Oracle FCCM | AKR-TMS target |
|---|---|---|
| Time to first alert | Multi-quarter project, 67 templates | Weeks, one canonical model and ready adapters |
| Arabic | BD matcher ignores Arabic; cloud screening transliterates (config) | Arabic-native matching engine with published precision/recall on Iraqi names |
| Islamic products | None | First-class contract types and scenarios |
| goAML | Cloud-only, v5.0.2, per-template licence | Multi-version, included, with the Iraqi chain and STR quality check |
| Price | Multi-meter usage licence | Flat tier per bank |
| Transparency | Scenario docs on a support portal | Scenario cards in the UI, exportable |
| Footprint | OFSAAI + DB + app server + OAS | Small containerised stack on PostgreSQL |
| On-premises AI | Not offered in the documents read | Local-model assistant (VA-10 in file 08) |

## 14. What it means for AKR-TMS

1. Oracle FCCM is the main **technical** alternative to NICE for a large Iraqi bank: it has the real-time hold path and
   the only documented goAML STR workflow. It is also the heaviest to integrate and the most expensive to meter.
2. The CBI controls ask for a system that is real time, linked directly to the CBS and screens in Arabic and English;
   Oracle's on-premises engine fails the Arabic part in its own documentation, and its cloud route raises residency
   questions. These are verifiable points to use in comparison tables (file 33).
3. Oracle's best ideas for AKR-TMS are not its AI claims but its engineering habits: versioned thresholds, entity-level
   alert testing, simulation of controls, XSD-driven reporting.
4. Oracle's marketing copy ("zero data migrations", "truly SaaS", "no post-sale customization") identifies the pain that
   buyers feel: use the same words, but prove them on a bank's real extract.

## 15. Derived requirements (VA-27 to VA-34; the Oracle-specific ones. Requirements from file 08, VA-01 to VA-26, also apply)

| ID | Requirement | Source |
|---|---|---|
| VA-27 | Every scenario, rule and threshold is documented in the product itself (Arabic and English): purpose, data inputs, logic in plain words, parameters, worked example, change history; exportable to the bank, never behind a support login | File 09 s.4, s.7, s.12 #7 (Oracle data model and scenario descriptions only on My Oracle Support); file 08 s.12 |
| VA-28 | Entity-level scenario tester: for any customer, account or period, show which scenarios and thresholds fired or did not fire, and which configuration decided it, without a full batch run | File 09 s.4, s.11 (Oracle Alert Analyzer, 26.8.1, Aug 2026) |
| VA-29 | Controls-testing harness: inject synthetic laundering patterns built from CBI and FATF typologies into a copy of the bank's profile data, measure which scenarios catch them, report gaps and overlaps, and keep results as validation evidence | File 09 s.4, s.5 (Oracle Compliance Agent: AI money launderer in a simulated environment); file 27 |
| VA-30 | goAML reporting that loads templates from the regulator's XSD, supports several schema versions at once, lists validation errors with links to the field, previews the report, generates the e-file, tracks status and FIU rejections, runs maker-checker, and carries Iraq-specific attributes (mother's name, foreign-script names) | File 09 s.6 (Oracle goAML app: XSD templates, v5.0.2 only, validation tab, e-file, reject workflow); file 23 |
| VA-31 | A product-agnostic canonical model (one account, one transaction, one party) with product_family and typed extension tables, including Islamic contract types and profit events; no per-product table sprawl; schema and mapping rules published to the bank | File 09 s.7, s.12 #2 (67 Oracle staging templates, one account and transaction table per product, none Islamic) |
| VA-32 | Every name-matching path (screening, entity resolution, scenario watch-list flags) handles Arabic script natively with transliteration variants, Kurdish spellings and patronymic chains; acceptance test with Arabic and transliterated names before go-live and published precision/recall; no component may silently ignore non-ASCII characters | File 09 s.8, s.12 #3 (Oracle BD matcher ignores Arabic: Administration Guide E.5.1); file 22 |
| VA-33 | Published implementation timeline from first data profiling to first production alert measured in weeks, with milestones (profile, mapped data, CBI scenario pack running, first tuning) and an exit criterion for each | File 09 s.9 (Oracle multi-template data load; no published timeline) |
| VA-34 | Long-term-support releases and a published deprecation policy: a feature in use is never retired without a multi-year overlap, a migration tool and a licence that does not force a different product | File 09 s.1, s.12 #6 (Oracle Alert Management retired for disposition, Trading Compliance withdrawn 2021, Scenario Wizard decommissioned) |

## 16. Sources

All fetched 2026-10-06 unless marked "(search result only)". Vendor pages are marketing and are labelled as such.

**Oracle product pages and datasheets (vendor)**
- FCCM AML landing page (modules, behavioural models and calibration FAQ, Nexus link, customer logos): https://www.oracle.com/financial-services/aml-financial-crime-compliance/
- Transaction monitoring and AML transformation FAQ: https://www.oracle.com/financial-services/aml-financial-crime-compliance/transaction-monitoring/
- Oracle Nexus for financial crime investigations (Case Flow, Reach, FAQ): https://www.oracle.com/financial-services/aml-financial-crime-compliance/oracle-nexus/
- FCCM on-premises datasheet (2025): https://www.oracle.com/a/ocom/docs/industries/financial-services/fs-crime-compliance-management-fccm-ds.pdf
- FCCM Cloud Service datasheet (2025): https://www.oracle.com/a/ocom/docs/industries/financial-services/fs-crime-compliance-management-cs-br.pdf
- Anti Money Laundering datasheet (2020): https://www.oracle.com/a/ocom/docs/industries/financial-services/ds-fs-aml.pdf
- Compliance Agent datasheet (v2.1, 2024): https://www.oracle.com/a/ocom/docs/industries/financial-services/fs-compliance-agent-ds.pdf
- Crime and Compliance Studio brief (2021): https://www.oracle.com/a/ocom/docs/industries/financial-services/fs-crime-compliance-studio-br.pdf
- AI Investigator sheet: https://www.oracle.com/a/ocom/docs/industries/financial-services/fs-fccm-ai-investigator.pdf
- Regulatory Reporting datasheet (2021, goAML template): https://www.oracle.com/a/ocom/docs/industries/financial-services/ofs-compliance-regulatory-report.pdf
- Customer Screening datasheet (2021): https://www.oracle.com/a/ocom/docs/industries/financial-services/oracle-fs-customer-screening-ds.pdf ; KYC datasheet: .../ofs-know-your-customer-ds.pdf ; Transaction Filtering: .../fs-transaction-filtering-cloud-services-ds.pdf ; Monitor: .../fs-compliance-monitor-cloud-service-ds.pdf
- Oracle global contacts: https://www.oracle.com/corporate/contact/global.html ; OCI public regions: https://www.oracle.com/cloud/public-cloud-regions/

**Oracle documentation (docs.oracle.com)**
- Behavior Detection library (all releases, note on confidential data model and scenario descriptions): https://docs.oracle.com/cd/E91253_01/homepage.htm ; reference guides: https://docs.oracle.com/cd/E91253_01/BD_reference_guides.htm
- Release Notes 8.1.2.12.0 (13 Jul 2026): https://docs.oracle.com/en/industries/financial-services/ofs-analytical-applications/behavior-detection/8.1.2.12.0/bdrln/release-notes-oracle-financial-services-behavior-detection-application-pack.pdf
- Installation Guide 8.1.2.12.0: .../behavior-detection/8.1.2.12.0/ingbd/installation-guide.pdf
- Administration and Configuration Guide 8.1.2.12.0 (architecture, Ingestion Manager, Fuzzy Name Matcher appendix E.5.1): .../behavior-detection/8.1.2.12.0/bdacg/behavior-detection-admin-guide.pdf
- Behavior Detection User Guide 8.1.2.12.0: .../behavior-detection/8.1.2.12.0/bduge/user-guide.pdf ; API Services Guide: .../behavior-detection/8.1.2.12.0/bdapi/behavior-detection-api-services-guide.pdf
- FCCM on-premises and cloud documentation indexes: https://docs.oracle.com/en/industries/financial-services/ofs-analytical-applications/fccm-onprem.html ; .../crime-compliance-cloud/index.html ; Transaction Monitoring Cloud docs: .../transaction-monitor-cloud/
- Cloud What's New books (readiness): https://docs.oracle.com/en/cloud/saas/readiness/financial-services-all.html ; releases 25.8.1, 25.11.1, 26.2.1, 26.5.1, 26.8.1 under https://docs.oracle.com/en/cloud/saas/readiness/financial-services/ (e.g. 2026/fccm-268/, 2026/fccm-265/, 2026/fccm-262/, 2025/fccm-2511/)
- FCCM Cloud Services Data Loading Guide 26.08.01 (67 templates): https://docs.oracle.com/en/industries/financial-services/ofs-analytical-applications/transaction-monitor-cloud/26.08.01/tmcsd/loading-data-fccm-cloud-services.pdf
- Metering page 26.05.01 (licence meters): https://docs.oracle.com/en/industries/financial-services/ofs-analytical-applications/crime-compliance-cloud/26.05.01/fcadm/metering.html
- Getting Started with FCCM Cloud (home region, DR region): https://docs.oracle.com/en/industries/financial-services/ofs-analytical-applications/infrastructure-common/26.08.01/fccgs/getting-started-oracle-fccm.pdf
- goAML User Guide, cloud (14 Aug 2026): https://docs.oracle.com/en/industries/financial-services/ofs-analytical-applications/regulatory-reporting-cloud/26.08.01/goamluserguide/goaml-user-guide-cs.pdf ; Regulatory Reporting books: .../regulatory-reporting-cloud/26.08.01/books.html
- Customer Screening Matching Guide (26 Aug 2026): https://docs.oracle.com/en/industries/financial-services/ofs-analytical-applications/customer-screening-cloud/26.08.01/csmat/matching-guide.pdf

**History, analysts and practitioners**
- Wikipedia, Oracle Financial Services Software (i-flex, Mantas acquisition 2006, Oracle stakes): https://en.wikipedia.org/wiki/Oracle_Financial_Services_Software
- Chartis FCC50 2025 and 2026 listings (rank paywalled): https://chartis-research.com/financial-crime/7947401/financial-crime-and-compliance50-2025-ranking-and-award-winners ; https://chartis-research.com/financial-crime/7947472/financial-crime-and-compliance50-fcc50-2026-ranking-and-award-winners ; RiskTech100 2026: https://chartis-research.com/market-analysis/7947415/risktech100r-2026-ranking-and-award-winners
- PeerSpot, Oracle FCCM Cloud Service reviews: https://www.peerspot.com/products/oracle-financial-crime-and-compliance-management-cloud-service-reviews ; Actimize AML vs Oracle comparison with the three-product reviewer: https://www.peerspot.com/products/comparisons/nice-actimize-anti-money-laundering_vs_oracle-financial-crime-and-compliance-management-cloud-service ; AML category page: https://www.peerspot.com/categories/anti-money-laundering-aml
- Forrester Wave AML Q2 2025 vendor releases (checked for Oracle; none found): SAS https://sas.com/en_in/news/press-releases/2025/april/forrester-wave-aml-solutions-leader.html ; SymphonyAI https://www.symphonyai.com/resources/leader-2025-forrester-wave-anti-money-laundering ; NICE (file 08)
- Regulators: FCA Final Notice Metro Bank (no vendor named) https://www.fca.org.uk/publication/final-notices/metro-bank-plc-2024.pdf ; DOJ TD Bank statement of facts (SAS named, file 10) https://www.justice.gov/media/1373151/dl

**Not reached:** My Oracle Support (data model and scenario descriptions), Oracle blogs (HTTP 403), Oracle customer stories (404), Gartner Peer Insights, G2, TrustRadius (blocked), the Forrester Wave report, Chartis ranking tables (paywalled), Oracle price lists, any Iraq-specific Oracle FCCM reference (none public).
