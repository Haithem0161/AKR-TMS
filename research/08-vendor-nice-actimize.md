# 08 — Vendor: NICE Actimize (SAM, X-Sight, ActOne, CDD/KYC, WL-X, Xceed, IFM)

Status: done (2026-10-06)

Part 1 of 4 of the vendor research (incumbent enterprise AML suites). Other files: 09 Oracle FCCM, 10 SAS AML, 11 other
incumbents (SymphonyAI NetReveal, FICO TONBELLER Siron, Nasdaq Verafin, Fiserv AML Risk Manager) and the comparison table.
Requirement IDs use the prefix VA-xx and are numbered across all four files.

## Contents

1. Ownership and history
2. Product modules (what NICE Actimize sells)
3. Architecture and deployment
4. Detection approach (scenarios, segmentation, ML, entity resolution, graph)
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
| 1999 | Actimize founded in Israel ("Silicon Wadi") by ex-army colleagues in business intelligence and data warehousing; first products were trading compliance, AML and fraud packages | Innovation Israel profile; Actimize history |
| 2 Jul / 30 Aug 2007 | NICE Systems agrees (2 Jul) and completes (30 Aug) the purchase of Actimize Ltd. Price about **USD 280 million**, about 80% cash and 20% in 1,501,933 NICE shares | SEC 6-K / 424B5 (2007) |
| 2009 | Actimize becomes a NICE division and is rebranded "NICE Actimize" | Calcalist (Sep 2026) |
| 17 Oct 2018 | X-Sight launched as a "Platform-as-a-Service" that "augments trusted on-premise deployment and works seamlessly toward cloud adoption" (marketplace, ML, open services-based architecture) | NICE press release, 17 Oct 2018 |
| 4 Jun 2020 / 19 Aug 2020 | Acquires Guardian Analytics (behavioural fraud analytics for banks and credit unions); on 19 Aug 2020 launches **Xceed**, a cloud platform for "financial institutions of all sizes, including community and regional banks" that combines Guardian Analytics and Actimize capabilities (AML plus fraud) | Business Wire 4 Jun and 19 Aug 2020 |
| Apr 2025 | Xceed AI FRAML and Xceed AI agents announced (7 Apr 2025). Forrester Wave AML Solutions Q2 2025 names Actimize a Leader (8 Apr 2025) | NICE press releases |
| Jan 2026 | Actimize Insights Network launched (shared counterparty-risk signals across the Actimize client base) | NICE press release, 27 Jan 2026 |
| 2 Jun 2026 | DNB (Norway's largest bank) selects X-Sight Enterprise (SAM, IFM, ActOne, WL-X, CDD, ActimizeWatch, DataIQ), with Infosys as integrator | NICE press release |
| Nov 2025 -> Sep 2026 | **Actimize put up for sale** (Goldman Sachs and J.P. Morgan advising; asking price reported USD 1.5-2bn; May 2026 five bidders advanced with offers around USD 2.5bn; **23 Sep 2026: Brookfield's financial-infrastructure arm reported in exclusive talks at about USD 2bn**). Not yet confirmed or signed as of 6 Oct 2026 | Calcalist/CTech 23 Sep 2026; A-Team 15 May 2026 |

Scale (parent figures): NICE (renamed "NiCE" in 2025) reported FY2025 revenue USD 2,945.4m (SEC 6-K, 19 Feb 2026).
CTech puts **Actimize at USD 453.5m revenue in 2024 (16.6% of NICE) and USD 158.3m operating profit (almost 29% of
group operating profit)**. In Q1 2026 the financial-crime-and-compliance segment had USD 133m revenue (17% of NICE),
+23% year on year, "supported by premise-based term renewals with top-tier financial institutions" (NICE CFO,
quoted by A-Team). NICE's own claims: "over 1,000 organizations across more than 70 countries"; "5 billion+
transactions monitored daily"; the 2022 AML brochure said "400+ customers" for the AML suite.

NICE's CEO told analysts in May 2026 that the company had been "working with advisers over the past several months"
on a process for "non-CX assets", which he identified as financial crime and compliance and public safety (A-Team,
15 May 2026). Management now frames NICE around CX (CXone, Cognigy); Actimize is "increasingly non-core".

**Why this matters for AKR-TMS.** (1) The market leader is changing owner in 2026-27; clients will ask about roadmap
continuity (A-Team: "roadmap continuity, support for current deployments, cloud migration"). A bank buying an
incumbent buys that risk; a local product with a bank-held source-escrow and an Iraqi owner does not. (2) A private-
equity carve-out usually means margin discipline and price pressure on renewals: the exact moment banks look at
alternatives. (3) The "premise-based term renewals" line confirms that the biggest banks still run Actimize
on-premises: on-premises is not obsolete, even for top-tier institutions.

### AKR-TMS hooks (section 1)

- Position AKR-TMS as vendor-stable and locally supported; offer escrow of source, schema and configuration with
  every bank licence (VA-01).
- Keep every deployment's detection logic, tuning history and data model exportable in open formats so a bank is
  never locked to the product (VA-02).

## 2. Product modules (what NICE Actimize sells)

NICE's current AML portfolio (niceactimize.com, Oct 2026) lists: **Suspicious Activity Monitoring (SAM)**, **KYC**
(CDD/EDD/customer lifecycle), **Sanctions Screening (WL-X, payments and party screening)**, **X-Sight Entity Risk**
(one entity profile and trust score across fraud, AML and KYC), **Suspicious Transaction Activity Reporting (STAR)**,
**Currency Transaction Reporting (CTR)**, **AML Essentials** (pre-packaged AML for smaller banks), and **Xceed**
(integrated FRAML for credit unions and community banks). Around them:

| Layer | Product | What it does | Notes |
|---|---|---|---|
| Detection | SAM (Suspicious Activity Monitoring) | Rules library plus ML: unsupervised segmentation, tuning/optimisation, anomaly detection, predictive alert scoring, graph "community analytics" | "Retain Rules - The Critical Safety Net": an "extensive and industry-proven library of rules" stays |
| Detection | IFM (Integrated Fraud Management) | Fraud scoring and mule detection | Shares data and network analytics with SAM |
| Screening | WL-X (watch list filtering), payments screening | Party and payment screening, "watchlist agnostic", predictive hit scoring | Cultural-match and transliteration agents added 2026 |
| KYC | KYC / CDD (customer due diligence), X-Sight Onboard | Risk-rating engine, event triggers, perpetual KYC (pKYC), outreach portal | Celent 2026 Luminary; IDC MarketScape CLM 2026 Leader |
| Case | ActOne (Enterprise Case Manager) | Workflow, alert/case management across AML, fraud, KYC and surveillance; "Productivity Studio"; quality assurance; "Extend" | Datos Matrix: market leader in fraud and AML case management |
| Reporting | STAR, CTR | OOTB SAR/STR forms for "many global jurisdictions", e-filing utility, acknowledgements, FinCEN 90-day continuing activity, GenAI narrative ("NarrateAI") | goAML: not named on any NICE page (see section 6) |
| Data | X-Sight DataIQ / Data Intelligence, Unified Data Model (UDM) | Ingestion, mapping, enrichment, identity resolution; "pre-integrated access to over 100 global data sources" | Marketplace of 75+ partner providers |
| Analytics | X-Sight Studio, ActimizeWatch, Bring Your Own, Collective Intelligence | DIY model build; fully managed ML analytics-as-a-service; own models; peer performance data without PII | |
| Network | Actimize Insights Network (Jan 2026) | Shared counterparty risk signals, millisecond response for instant payments | Needs a network of banks; irrelevant to a first Iraqi deployment |
| Mid-market | Xceed AI FRAML / AML / Fraud / Investigations; Xceed AI agents | SaaS suite for credit unions and community banks (from Guardian Analytics heritage) | The nearest competitor in packaging for a small bank, but cloud-only |
| Markets | SURVEIL-X | Trade and communications surveillance | Out of scope for AKR-TMS |

Sources: niceactimize.com AML, X-Sight, Xceed, case-management and KYC pages; AML Solutions Portfolio brochure
(Aug 2025); press releases (Apr 2025, Jan 2026, Jun 2026).

## 3. Architecture and deployment

**Deployment options (vendor statements).**

| Option | Evidence |
|---|---|
| On-premises, "containerized" | SAM-10 brochure (2025): "1. Containerized on-premise deployment ... 2. SaaS cloud deployment ... a versionless solution". Older generations (SAM 9 and earlier) are classic multi-server installs (see below) |
| Private or public cloud, SaaS (X-Sight Enterprise) | Celent 2026 (KYC): "can be deployed on-premise, in private or public cloud environments, or as a SaaS model. It supports a microservices architecture and offers extensive API integration capabilities" |
| SaaS only (Xceed) | Xceed AI FRAML is a "seamless SaaS platform" for credit unions and community banks |
| Forrester summary (Q2 2025) | "a great fit for enterprises looking for a combined AML and fraud management (FRAML) solution on-premises or in the cloud" |
| Hosting regions | No public statement found of an X-Sight region in the Gulf. NICE announced EU sovereign cloud, UK and Australia regions (CX product, Jul 2026). **Not verified** for Iraq/MENA data-residency; for Iraq the on-premises option is the relevant one |

**What a real on-premises deployment looks like.** The best public evidence is an engineering runbook published on
Read the Docs ("NICE Actimize ETL 3.0.0", versions dated Dec 2022 - Aug 2024) for a bank whose environment names
(KEN_PRD) and core system (Temenos T24) point to a Kenyan bank. It documents the data feed for SAM, WL-X (WLF) and CDD:

- Source: an **Oracle enterprise data warehouse** (parties, accounts, loans, transactions), daily exchange-rate flat
  files, World-Check delta and deletion files, an employee repository.
- Three **SQL Server staging databases** in sequence (`EDW_Staging` -> `StagingKE` -> `UDM_STG`), loaded by SSIS
  packages and `.bat` files, scheduled by **BMC Control-M**; two application servers (one for WLF, one for SAM/UDM),
  each with a Control-M agent and the "Actimize Server Monitor".
- **Daily batch**: "for each skipped day, a separate batch is run"; a backlog is cleared by hand-setting four
  Control-M variables (`IS_PROCESSING_BACKLOG`, start, end and current date) and re-running day by day. A
  `Batch_Sanity_Check` table with per-table checksums tells operators whether a load completed.
- Fragility is operational, not conceptual: when the **Windows service account's password expires** the Oracle linked
  server, Control-M run-as, SSIS proxy and credentials all have to be re-mapped; World-Check files could not be read
  from Control-M ("access denied") so a separate SQL Agent job was added; exception workbooks above 15,000 rows failed
  to load under the job proxy.
- Write-back to the core is a **file drop**: "a batch script extracts current CDD scoring ... delta entries are
  persisted onto a file stored on a network folder. T24 will then pick this delta file and update the customer
  scores" (described as "near-real-time CDD scoring").

This matches the AKR-TMS constraint (read-only database access, nothing written to the CBS), but it shows the cost: a
bank team has to build and operate a warehouse, three staging stores, a scheduler and file hand-offs around the product.

**Practitioner statements (PeerSpot, "NICE Actimize Fraud & Authentication Management" page, which covers SAM, CDD and
WLF reviews; 18 reviews, 4.1/5, 94% would recommend; mindshare 2.8% in Oct 2026, down from 6.0%).**

- "Actimize uses many databases, and everything on Actimize has been deployed to the database" (fraud strategist,
  10,001+ employee firm, Feb 2023) and "Processes don't function when the front end is down" (summary of cons).
- Modes: a technology-planning director at an IT services firm describes a "batch mode and real-time mode" and says that
  in the version deployed "when we were running the test during the deployment, we encountered some technical issues".
- "It is complex in terms of daily maintenance. Other detection platforms run on a 15-day or one-month window, whereas
  this particular platform runs daily."
- A reviewer who runs Actimize, Oracle FCCM and a BAE Detica/Norkom system side by side: "Actimize has a very flexible
  UI ... Actimize doesn't do error handling very well, and it will be difficult for the developers to support it every
  day. Otherwise, to implement and deploy, NICE Actimize has many automatic solutions that are better. Oracle has many
  manual deployments and implementations."
- Upgrades: "Sometimes when we move from one version to another, a few things don't work as expected" (Actimize
  developer at Infosys, Mar 2023); Gartner Peer Insights summary (4.0/5 from 6 ratings; Service and Support 3.4/5):
  dislikes include "lack of innovation and new features, and challenges with integration and upgrading" (from a
  search summary; the Gartner page itself sits behind a bot check and was not read directly).

**Real-time vs batch.** Historically SAM is a **daily batch** engine fed by UDM; **WL-X payment screening is real
time** ("as little as 200 milliseconds", ISO 20022 MX parser, "over 85 million records a day"); IFM is real-time
fraud scoring. For AKR-TMS this is the key architectural contrast: the CBI controls require "instant, real-time
follow-up" of account movements (file 01, section 2d), which a daily-batch UDM design does not satisfy without extra
components. NICE does not claim a streaming AML engine for SAM in the materials read.

### AKR-TMS hooks (section 3)

- Ship the data pipeline with the product (read-only extract -> canonical model -> detection), with checksums, a
  per-batch sanity table, automatic backlog catch-up and no operator-set variables (VA-03).
- Do not depend on a general-purpose scheduler or Windows service accounts: one runtime (Rust services, PostgreSQL)
  with built-in scheduling, secrets in OpenBao (the stack's secrets service, file 32), and alerts when a load is late or incomplete (VA-04).
- Because CBS write-back is forbidden, the bank-operations tasks (freeze, block, score change) must be a first-class
  task queue with read-only verification, not a file drop that someone has to pick up (VA-05).

## 4. Detection approach

NICE describes SAM as "three layers of defense" (SAM-10 brochure, 2025):

| Layer | Content | Vendor claim | Comment |
|---|---|---|---|
| 1. Rules | "250+ expertly crafted out-of-the-box rules"; "over 20 years of knowledge built into our rules"; "comprehensive out-of-the-box coverage of all FATF money laundering typologies"; self-service rule building | "Rules ensure your organization is protected and always catches known money laundering threats, even if the advanced analytics don't" | The vendor itself keeps rules as "the critical safety net" and as a benchmark against ML bias. Verticals: retail, corporate, correspondent, wholesale, private banking, investment banking, asset/wealth, MSBs, merchants/acquirers, PSPs, gaming, insurance, trade finance, capital markets, crypto. **No Islamic-banking vertical is listed** |
| 2. AI analytics | Unsupervised **segmentation** (customer groups from attribute, relationship and behavioural data); ML **optimisation** that recommends thresholds per segment and runs "full tuning cycles and simulation in under 2 hours"; **anomaly detection** (unsupervised multivariate, against history and all peer segments); **predictive scoring** (supervised on past dispositions, hibernates low-risk alerts, below-the-line sampling of hibernated alerts, re-escalation if new activity arrives); **Collective Intelligence** (proven thresholds, auto-generated rule suggestions and pre-built models from peer institutions, without PII); build-your-own models (X-Sight Studio, ActimizeWatch managed ML, bring-your-own) | "Reduce alert volumes by over 40%"; "over 35% false positive reduction" (segmentation); predictive scoring "up to 40% with 100% accuracy"; "61% reduction in alert volumes, 98% accuracy" (2022 brochure); "up to 85%" (infographic) | Numbers are vendor marketing and mix definitions (alerts vs false positives; accuracy on historical samples) |
| 3. Network analytics | Identity resolution + graph + **community detection** to find mule networks and corporate structures, pre- and post-alert | Global tier-1 bank "uncovers 21 new mule rings"; tier-one North American bank "1,800+ mule accounts" | Needs entity-resolved data first (see section 7) |

**Entity resolution.** "single, deduplicated, and accurate view of each entity" - "Entity Resolution can be deployed at
scale quickly, with implementation taking only a few days" (SAM brochure). Identity resolution is described as
intrinsic to the suite. Whether it handles Arabic transliteration and patronymic chains is not documented publicly.

**What the case studies actually show.**

| Case study | Result | Reading |
|---|---|---|
| Large US full-service bank (2025) | False positives -31%, "100% true positive recall", 1,817 investigation hours saved per quarter, 3.5 FTE freed; predictive scoring replaced outsourced level-1 triage | A one-third cut after adding a supervised scoring layer to an existing SAM |
| Large US retail bank (2025) | FP -33%; unsupervised segmentation into **13 groups across four regions**, new thresholds under model governance; 46,033 alerts = 514 high + 30,194 medium + 15,325 hibernated (~33%); 5 FTE, 11,769 hours a year saved | The bank's problems were "existing thresholds and lack of segmentation" |
| Multi-national bank (2023, SAM 9) | Starting FP rate **96%**; 30% of the lowest-risk alerts hibernated "at a precision of 99.9%"; 27% of escalated higher-risk alerts led to a SAR; average investigation time -14% | Even after ML, 73% of the alerts routed to level 2 were not SARs |

So the leading incumbent's own evidence is a **30-35% reduction** in noise after a heavy programme, not the 70-90%
that is often quoted. This is the benchmark AKR-TMS must beat and also the number it should promise honestly.

**Tuning governance.** NICE sells "automated model tuning ... with full model governance"; "recommendations with full
governance documentation for organizations to make a tuning determination" (human approval). The SAM-10 brochure
admits what the market has complained about for years: transaction monitoring "has suffered from persistent
problems - slow and lengthy tuning cycles, limitations in typology coverage, and technology constraints".

### AKR-TMS hooks (section 4)

- Keep a transparent rules library as layer 1 (the CBI scenario catalogue is rule-shaped; see file 27) and add
  segmentation, threshold recommendation and alert scoring as layers 2-3; every ML output must show the rule/feature
  behind it and be switchable off (VA-06).
- Make tuning a product feature with a what-if simulator over the bank's own history, an approval step and an audit
  record, and publish the simulation time (target: minutes, not weeks) (VA-07).
- Hibernation must always carry below-the-line sampling and automatic re-escalation, as Actimize does, and must be
  approved by the MLRO per scenario (VA-08).
- Do not market a false-positive reduction above what can be shown on the bank's own data; report alert-to-STR
  conversion per scenario instead (VA-09).
- Add Islamic-banking products as a first-class segment/vertical (none of the incumbents list it; see file 24)
  (VA-26).

## 5. AI and GenAI features to October 2026

| Feature | What it does (vendor description) | Date / status | Evidence quality |
|---|---|---|---|
| Predictive scoring, segmentation, anomaly detection, community analytics | Classical ML (supervised and unsupervised); see section 4 | In production at banks since SAM 9 (2023 case study) | Customer case studies, vendor-published |
| **InvestigateAI** | Summarises alert details, surfaces prior cases and related activity, recommends next steps aligned with the bank's procedures; embedded in ActOne. Marketing claim: "50% reduction in investigation time" | 2024 (GenAI); "agentic" version 2025-26 | Vendor claim |
| **NarrateAI** (inside STAR) | Drafts the SAR/STR narrative from case data; analyst reviews, edits and approves before filing. Claims: "70% reduction in SAR filing time" | 2024 | Vendor claim |
| Xceed AI agents | Alert triage, backlog categorisation, high-risk case summarisation, clustering, "conversational adaptability" for the Xceed SaaS suite | 7 Apr 2025 | Press release |
| X-Sight ActOne with agentic AI | Applies NLP to the bank's own policies "to autonomously craft investigation plans", reasons over case context, injects real-time data, learns, hands over to humans when needed | Apr 2025 | Press release |
| Embedded agentic AI for AML (5 tasks) | (1) data enrichment for customer risk rating, (2) screening hit analysis incl. a **"Cultural Match" step resolving "linguistic and transliteration variations"**, (3) transaction context, (4) prior-case insights and evidence, (5) SAR narrative. Each flow is plan -> fetch data -> summarise -> insights -> **human review -> human decision** | Brochure Jun 2026 | Brochure; no customer data published |
| Adverse-media agent | Agentic adverse media screening; Celent 2026 "Luminary" for adverse media (27 Jul 2026) | Jun-Jul 2026 | Analyst plus brochure |
| GenAI for data onboarding | "Schema setup and mapping both use genAI and are intuitive" | Forrester Q2 2025 | Analyst |
| Insights Network | Shared, anonymised counterparty-risk signals across the Actimize client base, millisecond response for instant payments | 27 Jan 2026 | Press release; value depends on network size |
| Collective Intelligence | Peer-proven thresholds, auto-generated rule suggestions and pre-built models without exposing PII | 2024-25 | Vendor |

**Governance statements worth copying.** NICE's GenAI privacy sheet: data "is never shared or exposed externally",
encryption in transit and at rest, "No Training" (customer data not used to train foundation models), retrieval-
augmented generation over bank knowledge bases. The Jul 2026 article "Agentic AI in AML" tells buyers to demand
"a governance model built for action" and says "Every agent contributing to a compliance decision is a model under
SR 11-7, EU AMLR and ISO/IEC 42001" (note: the US agencies rescinded SR 11-7 on 17 Apr 2026, see file 05, section 1c;
vendor material lags). The same article concedes: "A well-tuned rule, a coded heuristic or a supervised model often
resolves a problem faster, cheaper and with less oversight than an agent will. Sanctions name screening on a clean
entity does not need a reasoning loop." A Chartis/NICE article stresses that SAR narratives must be "auditable and
traced back to source information to avoid the risk of hallucinations".

**Unknowns that matter for Iraq.** NICE does not say which LLMs are used or where they run. The materials describe
hosted foundation models; for an on-premises bank with no outbound data path the GenAI and agent features may be
unavailable or need a NICE-managed endpoint. **Not verified**; ask any Actimize bidder for an air-gapped GenAI
deployment option and its cost.

### AKR-TMS hooks (section 5)

- The agentic pattern (plan -> fetch -> summarise -> insight -> human decision) with evidence links per statement is the
  right one; AKR-TMS should ship it using a **locally hosted model** and make every sentence in a draft STR link to
  the transaction, KYC field or document it came from (VA-10).
- A "Cultural Match" assistant for Arabic names is the first Iraq-specific GenAI feature to build (see file 22), with
  the matching score and the reason shown, never a bare score (VA-11).
- Treat every AI output as a model under the bank's governance (inventory, version, validation set, drift check),
  independent of whether the US guidance is rescinded (BCBS and Wolfsberg still require it, files 04 and 29)
  (VA-12).

## 6. Case management and regulatory reporting

**ActOne (Enterprise Case Manager).** One environment for alerts, cases and investigation activity across AML, fraud,
KYC and (separately) markets surveillance. Features named in NICE material: dynamic workflows driven by alert type and
severity; low-code configuration "without relying on IT"; queues and routing by risk and policy; entity-centric view
with point-in-time history ("tracing your entities' activity, relationship networks, and profile information back to
any point in time"); graph exploration; alert auto-sampling, analyst scoring and surveying (quality assurance);
investigator-performance dashboards; "Productivity Studio"; "Extend" for third-party tools. Notably ActOne is
"agnostic to where alerts originate, whether from internal systems, NICE Actimize solutions or third-party tools".
Analyst recognition: Aite Matrix 2020 "Best in Class"; Datos Insights Matrix 2025 "Market Leader in Fraud & AML Case
Management"; Forrester Q2 2025 top score for case management.

**Regulatory reporting (STAR, CTR).** Auto-populates mandatory and optional fields, "validated forms", e-file utility,
"form status updated" and acknowledgements ("approved or rejected"), post-filing change history, **continuing-activity
triggers (the FinCEN 90-day review)**, dashboards; "OOTB regulatory forms, available for many global jurisdictions".
**goAML**: not named on any NICE page, brochure or press release read (searched the STAR page and brochure, the AML
portfolio, SAM-10 and agentic brochures). The product claims "integrated with Financial Intelligence Unit reporting
systems". Several FIUs use goAML (including Iraq's AML/CFT Office, file 01), so a goAML template is plausible but
**unverified**; any Iraqi evaluation must test the XML against the Iraqi goAML schema (see file 23).

**What it means.** The reporting module is built around the US SAR workflow (continuing activity, 90 days) and then
localised per jurisdiction. Iraq's flow is different (internal report -> written acknowledgement to the branch -> MLRO
written decision -> goAML; file 01, section 2d), so the generic "case -> SAR" model needs an extra layer for branch
liaison officers and internal reports.

### AKR-TMS hooks (section 6)

- ActOne's good ideas to copy: alert-source-agnostic intake (bank's legacy alerts can be imported on day one),
  low-code workflow configuration by compliance staff, point-in-time entity history, built-in QA sampling and analyst
  scoring, and acknowledgement/status tracking for every filing (VA-13).
- Model the Iraqi chain explicitly: branch internal report -> acknowledgement -> MLRO decision -> goAML STR, with the
  FIU "poor/acceptable/good" quality check before upload (VA-14).

## 7. Data model and integration with the core banking system

- **Canonical model.** The "Unified Data Model (UDM)" with Party, Account, Balance, Transaction, Alert and related
  entities, loaded by "Actimize Unified Data Manager"; "X-Sight DataIQ" for third-party data (registries, UBO,
  sanctions, PEP, adverse media); "pre-integrated access to over 100 global data sources" (2022). The schema is
  proprietary and not publicly documented.
- **Integration style.** The bank builds an extract (to an enterprise warehouse or staging area) and maps it to the UDM
  (the Kenya runbook above: Oracle EDW -> three SQL Server stages -> UDM). With X-Sight, NICE offers "smart mapping" and
  GenAI-assisted schema setup (Forrester). No ready-made adapters for specific core banking systems appear in public
  material (a Temenos T24 deployment is hand-built in the runbook).
- **Minimum data to run alerts** (runbook): party, account and balance must be present or the batch stops; a data
  quality step checks duplicate entities for CDD; balances are derived from transactions when days are missed.
- **Feedback to the core.** A scoring file for the core to read (no API write in the described deployment).
- **Lesson from enforcement.** The data feed is the weak point of every monitoring system. The FCA's Metro Bank notice
  (12 Nov 2024, GBP 16.7m) found an automated system fed from a near-real-time copy of the core database through
  batch files **failed to monitor over 60 million transactions (6.0% of volume), worth over GBP 51 billion (7.6% of
  value)**, because accounts opened and used on the same day were missing from the feed; nobody reconciled the feed for
  about three years (6 Jun 2016 to 17 Apr 2019) and the lookback finished in 2022 (153 extra SARs, 43 account closures).
  The FCA does not name the vendor. The architecture is exactly AKR-TMS's: a read-only replica feeding the engine.
  TD Bank's plea (DOJ, 10 Oct 2024) shows the second failure mode: transaction types left out of monitoring by design
  (domestic ACH, most checks, internal transfers) meant **92% of volume and 74% of value** was never monitored
  between 2018 and 2024 (statement of facts, para 33).

### AKR-TMS hooks (section 7)

- **Completeness reconciliation is a core feature**: control totals per source table, per day and per product (count and
  value in the CBS vs rows accepted by the engine), a visible "coverage" figure on the main dashboard, and an alarm on
  any gap; same-day-open-and-transact accounts, late-arriving rows and rejected records get explicit tests
  (VA-15).
- **Coverage register**: every product, channel and transaction type of the bank is listed with its monitoring status
  and a named reason if not monitored; unmonitored items expire and must be re-approved (VA-16).
- Provide ready-made read-only adapters for the core banking systems used in Iraq (file 25) rather than a generic
  mapping workshop (VA-17).

## 8. Arabic/RTL support and presence in MENA and Iraq

| Question | Finding | Confidence |
|---|---|---|
| Arabic or RTL user interface | **No NICE Actimize document read mentions Arabic or right-to-left** (51 NICE web pages, brochures, datasheets and press releases searched for "Arabic": no hit). Press pages exist in English, French, Japanese and Chinese only (site language switcher) | Absence of evidence; ask in any RFP |
| Arabic name matching | WL-X: "37 tunable engine matching options per culture ... with 23 different cultures available"; fuzzy matching "transliterates original script, classifies names according to region and culture, normalizes name forms"; edit distance "accounts for over 80% of common mistakes"; 2021 launch release: "the industry's most advanced culture/name matching technology" and "7th generation fuzzy matching". The list of the 23 cultures is not published. 2026 agent: "Cultural Match ... resolve linguistic and transliteration variations" | Medium for the capability, low for Arabic quality (patronymic chains, tribe names, Kurdish spellings; file 22) |
| Islamic banking | No Islamic-banking vertical, product (murabaha, ijara, mudaraba, wakala, Sukuk) or scenario is mentioned in any NICE material read; the SAM verticals list has none | High that it is not marketed |
| Iraq | **No public customer, partner or office in Iraq found.** No NICE statement on CBI scenarios. Iraq's AML/CFT Office uses goAML; a local goAML vendor (Pio-Tech, Bank-BI goAML) announced a go-live at Bank Al Etihad-Iraq (found via search, not read) | Medium |
| MENA | One public AML reference: **UAE Exchange** (Abu Dhabi remittance and exchange house) selected Actimize AML, SAM, CDD and WLF plus IFM, announced Sep 2015, "rolled out across operations around the world". PeerSpot lists a UAE Exchange sanctions/AML manager as a reviewer. Mashreq (UAE) chose **SAS**, not Actimize. No Gulf data-centre region for X-Sight found | Medium |
| Partners | Global SIs (Infosys, Mphasis since 2014 as reseller, Cognizant and others, from job postings); X-Sight Marketplace with 75+ providers. No Iraqi or Gulf partner named on the partner page | Medium |

**Reading.** Actimize is a Western-bank product with global reach; nothing public shows it fitted to Iraqi data (family
number, mother's name, Arabic-only national list, Kurdish spellings), to the CBI scenario catalogue, or to Islamic
products. It would be installed by an integrator, in English, on a SQL Server/Oracle stack, at a cost that few Iraqi
private banks can bear (section 9). This is AKR-TMS's opening.

### AKR-TMS hooks (section 8)

- Make Arabic and RTL a first-class property of every screen, report, STR narrative and export, not a translation layer
  (files 31 and 22) (VA-18).
- Ship the CBI scenario catalogue (86+ scenarios, file 27) and Iraqi list formats on day one, and publish a coverage
  matrix against them (VA-19).

## 9. Implementation time, cost and licensing signals

None of this is officially published; NICE sells through direct negotiation. Signals:

| Signal | Source | Reliability |
|---|---|---|
| "Expensive product"; "Licensing costs are high compared to other products in the market" (several reviewers) | PeerSpot | Medium (anonymous reviews) |
| One-time licence and implementation fee plus **annual maintenance**; changes beyond the standard contract follow a **change-order process** with extra charges | UAE Exchange manager and a compliance AVP, PeerSpot | Medium |
| "We need a separate license for each of the packages, such as the core package, self-development package, and customization package. Additionally, a separate license for using Designer. We must upgrade our license every 200 or 365 days" | Actimize developer at Infosys, PeerSpot | Medium |
| Pricing depends on users and modules (AML, activity monitoring, custom, plug-ins priced separately) | Actimize implementor, PeerSpot | Medium |
| "In Nigeria, the phase one deployment would cost 165,000,000 Naira which includes CDD, RCM, WNS ... implementation alone" with "a lot of middlemen" | Technology planning director at an IT services firm, PeerSpot (undated; not convertible without a date) | Low |
| Long contracts "kill our flexibility ... large banks value consistency over flexibility" | Compliance AVP at a 10,001+ employee firm | Medium |
| Revenue per client: USD 453.5m (2024) over "over 1,000 organizations" is about **USD 0.45m per client per year on average** (fraud, AML, surveillance, all sizes; large banks pay multiples of this) | Calcalist + NICE | Arithmetic from published figures |
| EUR 200,000-1,000,000+ licence and implementation; "at least EUR 1 million" for a mid-size bank with 2-3 modules and 12 months; "no self-service or public price model" | A German AI-tool directory citing "industry experience and public RFP documents" | **Low**; shown only as a rough order of magnitude |
| Deployment "ranges from a few days to several months, depending on the number of solutions and the expertise of the teams" | PeerSpot summary | Medium |
| Forrester Q2 2025 gave NICE the top score for "pricing flexibility and transparency" and praised "free health checks and model/library updates" | Forrester via NICE release | Analyst view; at odds with practitioners |
| Vendor claims: entity resolution "can be deployed at scale quickly, with implementation taking only a few days"; tuning cycles "in under 2 hours" | NICE brochures | Marketing |

**Deployment footprint (runbook).** Per environment: WLF batch server, WLF real-time server, SAM/UDM batch server, RCM
server, database server, Control-M client, ETL application server plus ETL database, in both UAT and production:
about ten virtual machines before high availability.

### AKR-TMS hooks (section 9)

- Price transparently and in a form an Iraqi bank can approve: per-bank subscription with all modules, no per-package
  licence, no change-order fees for tuning or new scenarios, and a published implementation plan with weeks, not
  months (VA-20).
- Run on far fewer servers: target a three-node production footprint (application, database, object store) with a
  documented sizing table by transaction volume (VA-21).

## 10. Analyst rankings

| Firm / report | Year | Result for NICE Actimize | Note |
|---|---|---|---|
| Forrester Wave: Anti-Money Laundering Solutions | Q2 2025 (8 Apr 2025) | **Leader**; highest possible scores in all 10 "current offering" criteria and in vision, adoption, pricing flexibility and transparency, community; 15 vendors evaluated (qualifying: standalone AML product with at least USD 10m AML revenue) | Forrester: "a great fit for enterprises looking for a combined AML and fraud management (FRAML) solution on-premises or in the cloud"; SAS Leader (4.40 current offering, "second highest"), SymphonyAI Leader, DataVisor Leader, Hawk Strong Performer. The full report is not public; caution text not read |
| Forrester Wave: Financial Crime Management Solutions | Q3 2026 (SAS release 24 Aug 2026) | **No NICE announcement found** on the NICE newsroom or accolades page (snapshot 6 Oct 2026). SAS reports the highest current-offering score (4.70/5; top marks in 9 of 11 criteria; no below-par score) among 15 firms evaluated | The newer Wave replaces the AML-only view with a converged FRAML view; NICE's placement is unknown and should not be assumed (see file 10, section 10) |
| Chartis Financial Crime and Compliance50 (FCC50) | 2024 (30 Apr 2024) | **#1 overall** in the inaugural report; category awards AML/TM, case management, global banking | FCC50 2025 (19 Sep 2025): "second consecutive year" as overall leader (from a search summary of a NICE release; release not opened); FCC50 2026 (9 Mar 2026) lists NICE Actimize but the rank is paywalled |
| Chartis RiskTech100 | 2024 | #15 overall (2024 edition; from a search summary, not opened) | RT100 2026 (23 Oct 2025) and 2027 (24 Sep 2026) ranks paywalled |
| Celent | 2024-2026 | "Luminary" in Watchlist Screening (2024), Anti-Fraud Solutionscape (2025, published Jan 2026), KYC CDD/CLM Solutionscape (Mar 2026), KYC Adverse Media Screening (Jul 2026) | Celent's KYC report profiled 30 providers |
| IDC MarketScape | 2024; 2026 | Leader, Worldwide Enterprise Fraud Solutions 2024; Leader, Worldwide Corporate Client Life-Cycle Management for Financial Services 2026 | |
| QKS Group SPARK Matrix | 2023; Q2 2026 | Overall Leader, Watchlist and Sanctions Monitoring 2023; Leader KYC Solutions Q2 2026; "Industry Pioneer" in the 2026 AI Maturity Matrix for KYC/CLM | |
| Datos Insights Matrix | 2025 | Market Leader, Fraud and AML Case Management | |
| Gartner Peer Insights (AML market) | read 2026 | 4.0/5 from only 6 ratings (Evaluation and contracting 3.8; Integration and deployment 3.8; Service and support 3.4; Product capabilities 3.8) | Tiny sample; page not directly readable (bot check) |
| PeerSpot | Oct 2026 | 4.1/5 from 18 reviews (94% would recommend); AML-category mindshare 6.1%, down from 15.3% a year earlier | Mindshare is page-view based |

Every analyst result above is from NICE or from the analyst's public listing; nearly all vendor-published. None
measured detection effectiveness on a bank's own data.

## 11. What it does right (with evidence)

1. **Layered detection with the rules kept.** Rules (250+ OOTB, built over 20+ years) stay as "the critical safety net";
   ML is added in layers (segmentation, tuning, anomaly detection, predictive scoring, graph). This is the same
   hybrid the Wolfsberg Group describes (file 04, sections 2a-2b) and it lets a bank explain the rule behind an alert.
   Forrester: rules-based transaction risk scoring "is powerful and ahead of the competition"; data scientists get
   "a catalog of built-in, productized AI and ML risk-scoring models".
2. **Safe use of alert hibernation.** Predictive scoring hibernates low-risk alerts only with **below-the-line
   sampling**, automatic re-linking and escalation if new activity arrives, and dip-sample advice on thresholds. In
   the published case, 30% of the lowest-risk alerts were hibernated "at a precision of 99.9%" (multi-national bank, SAM 9).
3. **Case management that regulators and peers recognise.** Datos Matrix 2025 market leader; Aite 2020 "best in
   class"; PeerSpot reviewers: "It's a very good case management system" (AVP compliance, 2023), "The case management
   tool is user friendly" (VP compliance, 2020).
4. **Regulatory comfort.** "It helps when we go to the regulators and can show them we're using a solution that is a
   market leader, and it gives us comfort and gives the regulators comfort that our transaction monitoring software
   is up to scratch" (PeerSpot reviewer). Brand is part of the product for a compliance head. AKR-TMS must replace
   brand comfort with evidence packs (validation, tuning logs, coverage matrix).
5. **Proven scale and stability.** "We don't have problems with stability and scalability"; "the core engine ... able
   to process large amounts of data"; WL-X claims 200 ms payment screening and "over 85 million records a day";
   largest banks run it on-premises on term renewals (NICE Q1 2026 call).
6. **Screening engineering.** Versioned and archived lists with point-in-time restore, per-line-of-business list
   combinations, ISO 20022 parsing with configurable fields, hit consolidation into one alert, hit suppression instead
   of allow lists, predictive hit scoring with hibernation and dip sampling.
7. **Entity-centric design and shared risk signals.** One entity profile feeding KYC, screening, monitoring and case
   management; community detection for mule rings; peer thresholds (Collective Intelligence) without PII.
8. **Human-in-the-loop GenAI.** Draft narratives are editable, drafts link to case data, final decisions stay with the
   analyst; a clear separation of what ML, GenAI and agents may do.
9. **Packaging for different sizes.** Xceed and AML Essentials target smaller banks, with "versionless" SaaS.

## 12. What it does wrong (with evidence)

| # | Weakness | Evidence |
|---|---|---|
| 1 | **Complexity and many versions** | "It has become too complex for its own good with a lot of versions ... Instead of keeping it ... traditional and keeping the core search engine as a standalone and having analytical bolt-ons, they have decided to jam it all into the same product" (director, 2021); "unnecessarily complex" (VP compliance, 2020); "complex in terms of daily maintenance" |
| 2 | **Cost and licence structure** | "an expensive product"; separate licences for core, self-development, customisation packages and Designer; periodic licence renewals; change orders; very long contracts "kill our flexibility" |
| 3 | **Heavy, brittle data plumbing** | The runbook: Oracle EDW + three SQL Server stages + SSIS + Control-M + about ten VMs per environment; manual backlog procedure; expired service-account password stops the chain; "Processes don't function when the front end is down" |
| 4 | **Daily batch, not real time, for SAM** | Batch ETL into UDM, alerts per daily run; the CBI requires "instant, real-time follow-up" and holding transactions until the MLRO clears them (file 01, 2d). Real-time exists for screening (WL-X) and fraud (IFM) |
| 5 | **Upgrades hurt** | "Sometimes when we move from one version to another, a few things don't work as expected"; Gartner Peer Insights dislikes: "challenges with integration and upgrading"; PeerSpot: "frequent version upgrades sometimes result in issues"; "one listener process that keeps failing ... after crossing 500 transactions ... no solution" from the vendor (reviewer, 2023) |
| 6 | **Slow tuning, admitted by the vendor** | SAM-10 brochure: transaction monitoring "has suffered from persistent problems - slow and lengthy tuning cycles, limitations in typology coverage". The fix (2-hour tuning, collective intelligence) is recent |
| 7 | **Modest measured noise reduction** | Own case studies: 31%, 33%, 30% hibernated; a 96% false-positive start rate; 27% of escalated alerts led to a SAR. Marketing says "up to 85%" and "61%" |
| 8 | **Innovation and support perception** | "Actimize is not being moved forward by Nice" (AVP compliance, Nov 2023); Gartner Peer Insights: "lack of innovation and new features"; Service and Support 3.4/5; PeerSpot: response times "slow", "bureaucratic processes" |
| 9 | **Ownership uncertainty** | Put up for sale Nov 2025; Brookfield in exclusive talks at about USD 2bn (23 Sep 2026); NICE calls financial crime "non-CX". Roadmap, price and support after a private-equity carve-out are unknown |
| 10 | **Lock-in** | Proprietary UDM, workflows and models; a directory page says exit means migrating "all historical case data, audit trails and calibrations" and that banks stay "10-15 years" (low-reliability source, but consistent with long term-renewals) |
| 11 | **No Arabic, Islamic or Iraq fit documented** | Sections 4 and 8 |
| 12 | **Reporting and UI depth** | "The reporting feature and dashboards could be better"; "interface speed" and "customer transaction searches" listed as improvement areas (PeerSpot summary) |
| 13 | **GenAI features not shown to run on-premises** | Section 5 |
| 14 | **Marketing outruns evidence** | "Comprehensive out-of-the-box coverage of all FATF money laundering typologies" is not backed by a published coverage matrix; tests such as 100% recall are on historical samples |

No regulator enforcement document read names Actimize as a cause of a failure. (Enforcement texts rarely name vendors:
the FCA's Metro Bank notice and the CSSF's Rakuten Europe Bank decision describe the same failure types - unmonitored
data, unmaintained tools - without naming any.)

## 13. Lessons for AKR-TMS

**Copy**

- The three-layer idea: transparent rules first, ML to tune and prioritise, graph for networks; every ML layer can be
  switched off and explains itself.
- Hibernation with below-the-line sampling and re-escalation (but approved per scenario by the MLRO).
- Alert-source-agnostic case management, low-code workflow, QA sampling and point-in-time entity history.
- List versioning with point-in-time restore for the screening lists (needed for the CBI's 12-hour refresh proof), plus
  hit suppression for repeat hits instead of permanent allow-lists (VA-22).
- Honest, named per-customer case studies with before/after numbers, and a published tuning simulation; a per-bank
  evidence pack that answers the correspondents' questionnaires from system facts (VA-23).

**Avoid**

- A product that needs a warehouse, three staging databases, a scheduler and ten servers before the first alert.
- Per-package licences, change-order fees and multi-year lock-in contracts.
- A batch-only engine when the regulator asks for real-time interdiction: build near-real-time monitoring with a
  hold-until-cleared path (VA-24).
- Painful upgrades: every release must ship with tested migrations, a rollback, and no re-implementation of the bank's
  configuration (VA-25).
- "AI" claims with no baseline: always give alert-to-STR conversion and a bank-specific replay test.
- Letting the vendor define the schema in secret: publish the canonical model and the mapping layer.

**Beat it on**

| Dimension | Actimize | AKR-TMS target |
|---|---|---|
| Time to first alert on a bank's real data | Months (ETL, UDM, mapping) | Weeks: read-only replica, ready adapters, canonical model in the box |
| Footprint | About 10 VMs per environment | Small, containerised, one scheduler, three nodes |
| Real time | Screening only | Near-real-time monitoring plus the optional pre-release screening API |
| Iraq fit | None documented | CBI scenario pack, Arabic-first UI, Iraqi names, goAML, branch liaison workflow, Islamic products |
| Price | Enterprise negotiation; separate packages | Published tiers for private and state banks |
| Data completeness | Bank's problem | Built-in coverage reconciliation and register (VA coverage items) |
| Governance | "Model governance" claims | Evidence pack per bank (CBDDQ answers, tuning log, validation) |

## 14. What it means for AKR-TMS

1. NICE Actimize is the benchmark for **capability breadth** and the reference an Iraqi compliance head will name when
   asked "what is the best system". The sale to Brookfield (if signed) removes the "stable parent" argument but not
   the capability argument.
2. Its weaknesses are exactly where a local, on-premises product can win: price and licence structure, the data
   pipeline, real-time monitoring, Arabic and Islamic fit, and speed of tuning.
3. Its claims are a ceiling for marketing: a one-third reduction in false positives after a programme is what the
   leader publishes. AKR-TMS should promise measurable, bank-specific improvement and show the method.
4. The enforcement record (TD Bank, Metro Bank) says the decisive risk is not the algorithm but **coverage and data
   completeness**; AKR-TMS can make that visible in a way no incumbent does.

## 15. Derived requirements (VA-01 to VA-26)

| ID | Requirement | Source |
|---|---|---|
| VA-01 | Escrow of source code, database schema and configuration for every bank licence, releasable if the vendor ceases support; vendor ownership change must not interrupt support or price | File 08 s.1 (Actimize sale to Brookfield reported 23 Sep 2026; A-Team 15 May 2026) |
| VA-02 | All detection logic, scenario parameters, tuning history, cases, audit trails and the canonical data model exportable in documented open formats (JSON/CSV/SQL) so a bank can leave without losing history | File 08 s.1 and s.12 #10 (lock-in; proprietary UDM) |
| VA-03 | Built-in read-only extract pipeline from the CBS database: control totals and checksums per table and batch, automatic backlog catch-up (no operator-set variables), per-batch sanity report, exceptions queue | File 08 s.3 (Actimize ETL runbook: Control-M variables, Batch_Sanity_Check) |
| VA-04 | One runtime with built-in scheduling and secrets (OpenBao); no dependence on a general-purpose scheduler or Windows service accounts; alarms when a load is late, partial or failed | File 08 s.3 (service-account password expiry stops the chain) |
| VA-05 | Freeze, block, score-change and similar actions are a first-class task queue for bank operations with SLA, ownership and read-only verification that the CBS state changed; never a file drop that someone must pick up | File 08 s.3 (T24 score delta file); project brief (no CBS write) |
| VA-06 | Three detection layers: transparent rule library (CBI scenarios first), ML for segmentation, threshold recommendation and alert scoring, graph for networks; every ML output shows the feature or rule behind it and can be switched off per scenario | File 08 s.4, s.11 #1 (SAM three layers; Wolfsberg effective-monitoring statements) |
| VA-07 | Tuning workbench: what-if simulation over the bank's own history (target minutes), side-by-side alert counts and alert-to-STR conversion, approval step, immutable audit record, and a published simulation time | File 08 s.4 (tuning in 'as little as 2 hours'; 'slow and lengthy tuning cycles' admitted in SAM-10 brochure) |
| VA-08 | Alert hibernation only with MLRO approval per scenario, below-the-line sampling, automatic re-escalation when related activity arrives, and a monthly sampling report | File 08 s.4, s.11 #2 (predictive scoring with dip sampling) |
| VA-09 | Report effectiveness as alert-to-STR conversion per scenario and segment and through replay tests on the bank's data; no headline false-positive reduction claim without the bank's own baseline | File 08 s.4 (case studies: 31%, 33%, 30% hibernated, 27% SAR rate) |
| VA-10 | GenAI and agent features run on a locally hosted model with no outbound data path; every sentence in a draft STR or case summary links to the transaction, KYC field or document it came from; analyst approves before use | File 08 s.5 (NarrateAI/InvestigateAI; hosted-model unknown for on-prem; Chartis/NICE traceability point) |
| VA-11 | Arabic name 'Cultural Match' assistant for screening and entity resolution (patronymic chains, tribe names, Kurdish spellings, transliteration variants) that returns a score and a human-readable reason | File 08 s.5, s.8 (Actimize 'Cultural Match'; 23 cultures, Arabic quality undocumented); file 22 |
| VA-12 | Inventory of every model and AI component (purpose, version, training data, validation set, owner, drift check, last review) exposed as a report for the MLRO, auditors and the CBI | File 08 s.5 (agents are models); BCBS/Wolfsberg (files 04, 29) |
| VA-13 | Case management core: intake of alerts from any source (including the bank's legacy tool), low-code workflow configured by compliance staff, point-in-time entity history, QA sampling and analyst scoring, and filing status/acknowledgement tracking | File 08 s.6 (ActOne: alert-source-agnostic, low-code, QA) |
| VA-14 | Model the Iraqi reporting chain: branch liaison/internal report -> written acknowledgement -> MLRO written decision -> goAML STR, with a pre-submission STR quality check that mirrors the FIU grading | File 08 s.6; file 01 s.2d and MER paras 563-582 |
| VA-15 | Coverage reconciliation: control totals (count and value) per source system, product and day versus rows accepted by the engine; a coverage percentage on the main dashboard; alarm on any gap; explicit tests for same-day account open-and-transact, late-arriving and rejected rows | File 08 s.7 (FCA Metro Bank Final Notice, 12 Nov 2024: 60m transactions, GBP 51bn unmonitored for ~3 years) |
| VA-16 | Coverage register: every product, channel and transaction type listed with monitoring status; any exclusion needs a named reason, an owner and an expiry date and must be re-approved | File 08 s.7 (TD Bank statement of facts para 33: 92% of volume, 74% of value unmonitored) |
| VA-17 | Ready-made read-only adapters and canonical mappings for the core banking systems in use in Iraq, tested with sample data, instead of a generic mapping workshop | File 08 s.7 (hand-built T24 integration); file 25 |
| VA-18 | Arabic and RTL as a first-class property of every screen, report, STR narrative and export, with English as the second language | File 08 s.8 (no Arabic/RTL documented by Actimize); files 22 and 31 |
| VA-19 | Ship the CBI scenario catalogue (86+ scenarios), Iraqi list formats and a coverage matrix against them on day one | File 08 s.8; file 01 s.2d; file 27 |
| VA-20 | Published per-bank subscription with all modules included; no per-package licences, no change-order fees for tuning or new scenarios; implementation plan measured in weeks | File 08 s.9 (PeerSpot licence and change-order complaints) |
| VA-21 | Production footprint of three nodes (application, database, object store) with a sizing table by transaction volume; no more than one scheduler and one runtime | File 08 s.9 (about ten VMs per environment in the Actimize runbook) |
| VA-22 | Screening lists versioned and archived with point-in-time restore, per-line-of-business list combinations, hit suppression for repeat hits instead of permanent allow-lists, and proof of list age (12-hour refresh) | File 08 s.11 #6 (WL-X list versioning); file 01 s.2d |
| VA-23 | Per-bank evidence pack generated from system facts: tool and version, last calibration, MLRO-approved tuning log, screening test dates, list-update lag, review metrics, to answer CBDDQ/FCCQ and CBI inspections | File 08 s.13; file 04 s.2c (CBDDQ) |
| VA-24 | Near-real-time monitoring of CBS movements plus a hold-until-cleared path for transactions matching high-risk scenarios (via the Phase 2 API), alongside daily batch checks | File 08 s.3, s.12 #4 (SAM is daily batch); file 01 s.2d (CBI 'instant, real-time follow-up' and hold until MLRO clears) |
| VA-25 | Every release ships with tested migrations, an automatic rollback and preserved bank configuration (scenarios, thresholds, workflows); no re-implementation after upgrade | File 08 s.12 #5 (PeerSpot and Gartner upgrade complaints) |
| VA-26 | Islamic-banking products (murabaha, ijara, mudaraba, musharaka, wakala, sukuk, profit distribution) as first-class segments and scenario inputs | File 08 s.4, s.8 (no incumbent Islamic vertical); file 24 |

## 16. Sources

All fetched 2026-10-06 unless marked "(search result only)", meaning the page was seen in a search result but not
opened. Vendor-published pages are marketing and are labelled as such.

**NICE Actimize (vendor)**
- Press-release index (latest items to 27 Jul 2026): https://www.niceactimize.com/press-releases/
- Xceed AI agents release, 7 Apr 2025 (PDF): https://www.niceactimize.com/Lists/PressReleases/Attachments/477/nice-actimize-introduces-xceed-ai-agents-for-faster-smarter-fraud-and-fincrime-prevention.pdf
- DNB selects X-Sight Enterprise, 2 Jun 2026: https://www.niceactimize.com/press-releases/nice-actimize-x-sight-enterprise-cloud-solutions-selected-by-dnb-norway
- Insights Network, 27 Jan 2026: https://www.niceactimize.com/press-releases/nice-actimize-unveils-insights-network-the-unified-force
- Celent 2026 KYC Luminary, 17 Mar 2026: https://www.niceactimize.com/press-releases/nice-actimize-positioned-as-a-leading-luminary-in-celents-2026
- Forrester Wave AML Q2 2025 release, 8 Apr 2025 (PDF): https://www.niceactimize.com/Lists/PressReleases/Attachments/478/nice-actimize-recognized-as-a-leader-for-aml-4-8-2025.pdf
- X-Sight launch, 17 Oct 2018 (PDF): https://niceactimize.com/Lists/PressReleases/Attachments/243/X_Sight_Announcement_10_17_2018.pdf
- WL-X launch, 11 Feb 2021 (PDF): https://www.niceactimize.com/Lists/PressReleases/Attachments/349/actimize_wlf_launch_2_11_2021.pdf
- ENGAGE 2026 announcement, 15 Apr 2026: https://www.niceactimize.com/press-releases/nice-actimize-announces-engage-2026
- Product pages: https://www.niceactimize.com/anti-money-laundering ; /xsight ; /xceed ; /enterprise-risk-case-management ; /kyc-and-client-lifecycle-management ; /data-intelligence ; /anti-money-laundering/suspicious-activity-monitoring ; /anti-money-laundering/sanctions-screening ; /anti-money-laundering/suspicious-transaction-activity-reporting ; /anti-money-laundering/customer-due-diligence ; /partners ; /customer-success ; /industry-accolades ; /get-in-touch (all under https://www.niceactimize.com)
- AML Solutions Portfolio brochure (Aug 2025): https://resources.niceactimize.com/wp-content/uploads/2025/08/Anti-Money-Laundering-Solutions-Portfolio-Brochure.pdf
- Embedded AI in AML brochure (Sep 2025): https://resources.niceactimize.com/wp-content/uploads/2025/09/Unleash-the-Power-of-Embedded-AI-in-AML-Brochure.pdf
- Embedded Agentic AI for AML brochure (Jun 2026): https://resources.niceactimize.com/wp-content/uploads/2026/06/Embedded-Agentic-AI-for-AML_Brochure.pdf
- "Agentic AI in AML" insights article (Jul 2026): https://resources.niceactimize.com/wp-content/uploads/2026/07/Agentic-AI-in-AML_Insights-Article.pdf
- SAM-10 brochure: https://resources.niceactimize.com/wp-content/uploads/2025/08/SAM10_Brochure.pdf ; SAM brochure: https://resources.niceactimize.com/wp-content/uploads/2025/09/aml_brochure_sam.pdf
- "Silencing the Noise" brochure: https://resources.niceactimize.com/wp-content/uploads/2025/09/aml-reducing-false-positives-in-transaction-monitoring-brochure.pdf
- Case studies: bank reduces false positives 31%: https://resources.niceactimize.com/wp-content/uploads/2025/09/aml_case_study_bank_reduces_false_positives_31_percent.pdf ; 33%: https://resources.niceactimize.com/wp-content/uploads/2025/09/aml_case_study_bank_reduces_false_positives_33_percent.pdf ; "From investigating the older to the riskier alerts" (2023): https://niceactimize.com/Lists/infographics/from-investigating-the-older-to-investigating-the-riskier-alerts.pdf
- STAR brochure (Mar 2024): https://niceactimize.com/Lists/Brochures/AML_STAR-Brochure_4Mar24.pdf
- AML suite brochure (2022): https://info.nice.com/rs/338-EJP-431/images/NICE%20Actimize_AML%20Suite%20Brochure_2022.pdf ; AML Essentials: https://www.niceactimize.com/Lists/Brochures/aml_essentials.pdf
- Payment screening datasheet (WL-X): https://resources.niceactimize.com/wp-content/uploads/2025/09/aml_payments_screening_datasheet_2024.pdf
- GenAI privacy infographic: https://resources.niceactimize.com/wp-content/uploads/2025/09/generative-ai-solutions-prioritize-data-security-infographic.pdf ; InvestigateAI brochure: https://resources.niceactimize.com/wp-content/uploads/2025/09/Accelerate-Investigations-with-Embedded-Agentic-AI-Brochure.pdf ; NarrateAI brochure: https://resources.niceactimize.com/wp-content/uploads/2025/09/Accelerate-SAR-Filing-with-AI-Brochure.pdf ; ActOne 2026 brochure: https://resources.niceactimize.com/wp-content/uploads/2026/05/ActOne-Centralized-Risk-FinCrime-Management-Brochure.pdf
- Chartis/NICE article on case management: https://resources.niceactimize.com/wp-content/uploads/2025/09/Chartis-NICEActimize-Next-Gen-Case-Management.pdf ; Data intelligence brochure: https://resources.niceactimize.com/wp-content/uploads/2025/09/AML_Data_intelligence_Brochure.pdf
- Actimize watch list filtering sheet hosted by IBM: https://public.dhe.ibm.com/software/data/information-agenda/B-Actimize_Watch_list_Filtering_Deep_Embed.pdf
- UAE Exchange implementation, Sep 2015 (search result only): https://www.niceactimize.com/press-releases/nice-actimize-to-support-uae-exchange-with-financial-crime-solutions-addressing-antimoney-laundering-and-integrated-fraud-management-135
- Mphasis delivery partnership: https://mondovisione.com/news/nice-actimize-confirms-mphasis-as-certified-delivery-partner-to-expand-reach-o-201666/
- Xceed launch, 19 Aug 2020 (search result only): https://www.businesswire.com/news/home/20200819005285/en/NICE-Actimize-Unveils-Xceed-a-Next-Generation-AI-Powered-Financial-Crime-Risk-Management-Cloud-Platform ; Guardian Analytics acquisition, 4 Jun 2020 (search result only): https://www.businesswire.com/news/home/20200604005343/en/

**Ownership, sale and financials**
- Calcalist/CTech, "Brookfield closes in on $2 billion deal for Nice's Actimize", 23 Sep 2026: https://www.calcalistech.com/ctechnews/article/iazqvedg2
- A-Team Insight, "NICE Earnings Call Puts Reported Actimize Sale Update in Context", 15 May 2026: https://a-teaminsight.com/blog/nice-earnings-call-puts-reported-actimize-sale-update-in-context/?brand=rti
- Calcalist, Nov 2025 sale report (search result only): https://www.calcalistech.com/ctechnews/article/syc7hf11zwx ; AML Intelligence (search result only): https://www.amlintelligence.com/2025/11/latest-aml-giant-actimize-up-for-sale-expected-price-of-1-5bn-report/
- NICE Ltd 6-K (Q4 and FY2025 results, 19 Feb 2026), SEC: https://www.sec.gov/Archives/edgar/data/1003935/000117891326000567/exhibit_99-1.htm
- NICE/Actimize 2007 acquisition, SEC 424B5 (search result only): https://www.sec.gov/Archives/edgar/data/0001003935/000095013607006535/file1.htm

**Analysts**
- Chartis rankings and awards index: https://chartis-research.com/rankings-awards ; FCC50 2026 listing: https://chartis-research.com/financial-crime/7947472/financial-crime-and-compliance50-fcc50-2026-ranking-and-award-winners ; RiskTech100 2027 listing: https://chartis-research.com/market-analysis/7947547/risktech100r-2027-ranking-and-award-winners (rankings paywalled)
- SAS release on Forrester Wave AML Q2 2025: https://sas.com/en_in/news/press-releases/2025/april/forrester-wave-aml-solutions-leader.html ; SymphonyAI page on the same Wave (inclusion criteria): https://www.symphonyai.com/resources/leader-2025-forrester-wave-anti-money-laundering
- Gartner Peer Insights NICE Actimize (rating summary from a search result; page behind a bot check): https://www.gartner.com/reviews/product/nice-actimize

**Practitioner evidence**
- PeerSpot reviews, pros and cons, pricing (Fraud and Authentication Management page covering SAM, CDD, WLF): https://www.peerspot.com/products/nice-actimize-fraud-authentication-management-reviews ; .../nice-actimize-fraud-authentication-management-pros-and-cons ; .../nice-actimize-fraud-authentication-management-pricing
- PeerSpot comparison Actimize AML vs Oracle FCCM Cloud (includes the three-product reviewer): https://www.peerspot.com/products/comparisons/nice-actimize-anti-money-laundering_vs_oracle-financial-crime-and-compliance-management-cloud-service
- "NICE Actimize ETL 3.0.0" runbook of a bank deployment (Read the Docs): https://nice-actimize-etl.readthedocs.io/en/latest/deployment-setup.html (and /troubleshoot-guide.html, /processing-backlog-steps.html, /cdd-feedback-loop.html, /server-catalog.html)
- KI-Syndikat directory page (low reliability, price range only): https://www.ki-syndikat.de/tools/nice-actimize/

**Regulators and courts (failure modes)**
- FCA Final Notice, Metro Bank plc, 12 Nov 2024: https://www.fca.org.uk/publication/final-notices/metro-bank-plc-2024.pdf
- US DOJ, TD Bank US Holding Company plea, Attachment A statement of facts, 10 Oct 2024: https://www.justice.gov/media/1373151/dl ; Information: https://www.justice.gov/media/1373341/dl
- CSSF decision on Rakuten Europe Bank, 9 Oct 2025: https://www.cssf.lu/wp-content/uploads/S_51_B_Rakuten-Europe-Bank-S.A._091025_en.pdf

**Not reached:** G2 and TrustRadius (HTTP 403), Gartner Peer Insights pages (bot check), Business Wire (HTTP 403), Forrester Wave full report and Chartis ranking tables (paywalled), NICE support portal and product manuals (customer login), any Iraq-specific Actimize reference (none public).
