# 07 — Vendor landscape overview: who sells AML and KYC systems, what they get right and wrong, and the gap for Iraqi banks

Status: done (lead, 2026-10-06). Synthesises files 08-18.

Evidence date: 6 October 2026. Every statement points to the vendor file where the evidence and links are. Evidence
grades follow file 13 section 11: **A** independent or regulator-compiled; **B** customer-reported with method; **C**
named customer on a vendor page; **D** vendor-only or anonymous. "Documented" means found in a vendor document, filing or
regulator text; "not found" means searched in public material and not found (it may exist behind a customer login).

## Contents

- Findings in brief
- 1. Scope and method
- 2. Market map: eight groups of vendors
- 3. What the market does right (table stakes for AKR-TMS)
- 4. What the market does wrong (recurring failures, with evidence)
- 5. Claims versus evidence
- 6. Fit for Iraqi banks
- 7. Legal and procurement screen for Iraq
- 8. The gap: what no vendor offers an Iraqi bank
- 9. What it means for AKR-TMS
- 10. Derived requirements (VO-01 to VO-14)
- 11. Sources

## Findings in brief

1. **One architecture, many wrappers.** Every serious vendor now sells the same hybrid: rules for known typologies and
   legal obligations, a model that prioritises or re-scores alerts, an anomaly or behavioural layer, entity resolution
   with a network view, an explanation per alert and, since 2025-26, a GenAI investigation assistant with a human in
   the loop (file 12 section 9.1; the Wolfsberg hybrid, file 04 section 2b). Vendors differ on packaging, evidence,
   deployment, language and price, not on detection theory.
2. **No vendor documents the Iraqi bundle**: on-premises delivery, a read-only link to the core banking database,
   Arabic-native matching of Iraqi names, goAML filing tested for the Iraqi FIU, the CBI scenario catalogue, Islamic
   products, an Iraqi presence and a price a small bank can pay (file 11 section 5d; file 13 section 11).
3. **Nearest by dimension, never on all.** Oracle FCCM is the only incumbent with a documented goAML STR application
   and a real-time hold path, but its on-premises name matcher states Arabic characters "will not match" (file 09).
   Napier AI and Silent Eight are the only challengers stating on-premises delivery and showing Arabic capability (file
   13). Flagright and Hummingbird treat the filing lifecycle as core; Hummingbird's "read in place, store only alerts" is
   the closest architecture to AKR-TMS (file 13). FinScan has the best public documentation of Arabic name handling
   (file 14). Quantexa leads on entity resolution and networks but needs a big-data stack (file 12).
4. **Legal screen.** Iraq's May 2022 law criminalising normalisation with Israel makes Israeli-headquartered vendors
   (NICE Actimize; ThetaRay, which also owns the screening engine Screena) legally and reputationally unusable for
   Iraqi banks; Iraqi counsel must confirm (file 12 section 2). US-owned vendors carry a different risk: their
   OFAC-driven policies can end support to a bank that is designated, as Al-Huda Bank was in 2024 (file 12 section 9.4;
   file 01 section 4).
5. **Cloud-first drift.** SymphonyAI, Nasdaq Verafin, NICE's Xceed and Essentials, ThetaRay, Lucinity and most
   challengers are SaaS; the newest AI features of Oracle (agents), SAS (Foundry copilot) and NICE are cloud services
   (files 08-13). Iraqi banks default to on-premises (file 01 section 8). This is a structural opening.
6. **Marketing numbers are not evidence.** False-positive reduction claims run from 30% to 99%; agent auto-resolution
   claims from 75% to 98%. The only regulator-compiled production figure is **7% of monitoring alerts auto-closed**
   (HKMA sample, Bank B); the only published alert-to-SAR rate is 27% at Level 2 in one NICE case; in an industry
   survey 70.8% of banks said 5% or fewer of alerts end in escalation or an STR (files 11, 12, 13).
7. **Owners change.** NICE is being sold (Brookfield reported in exclusive talks at about USD 2bn, 23 Sep 2026); FICO
   sold Siron (Dec 2022); BAE sold NetReveal to SymphonyAI (Oct 2022); UiPath bought WorkFusion (Feb 2026); Oracle
   licensed Lucinity's technology (Apr 2026). Vendor exit is a real risk for a bank that signs today (files 08, 11, 13).
8. **Regulators punished data, configuration, testing and operations, not algorithm brands.** TD Bank left 92% of
   volume unmonitored and added no scenario "since we first implemented SAS"; Metro Bank missed 60 million transactions
   through a bad feed; Starling screened 39 of 3,088 names for six years through a configuration error; RBS calibrated
   its screening once, in 2010 (files 11, 14, 20).
9. **Prices are hidden; the few signals are high.** Quantexa's UK public-sector list: GBP 0.56-7.7m per suite per year;
   NICE's average revenue per client about USD 0.45m a year and Verafin's about USD 0.12m (arithmetic from filings);
   screening contracts in public procurement are low six figures a year (files 11, 12, 14). Iraqi banks buy to comply
   and fund little tuning, so total cost of ownership is the competitive lever.
10. **Positioning that follows.** AKR-TMS should be on-premises, read-only, Arabic-native, goAML-ready,
    CBI-scenario-ready, Islamic-aware, flat-priced and continuity-guaranteed, and should not promise "AI that removes
    90% of alerts". The credible promise is a measured reduction in noise on the bank's own data, shown in a replay
    pilot, with the coverage figure and tuning history visible every day (file 11 section 6.5).
11. **Iraq is a replacement market with a deadline.** Every Iraqi bank already runs an automated monitor on the CBI's
    standard scenarios and none had built its own when the MER was written (MER para 566). Banks' annual reports name
    the tools, often several per bank: Siron at Bank of Baghdad and, from 2024, National Bank of Iraq; Firco at Bank of
    Baghdad and Trans Iraq Bank; core vendors' AML modules at four banks (Azentio at two Islamic banks, Capital Banking
    Solutions at Trans Iraq Bank and Sumer); and Pio-Tech, a Jordanian supplier listing 29 banks in Iraq among its
    references. None of the largest global suites (Actimize, Oracle, SAS) has an Iraqi customer. The most common core
    system, ICS BANKS, has no AML engine, so the core vendor is mostly a data source, not a rival. Auditors at two banks
    still describe the CBI minimum as 25 scenarios. The CBI's banking reform now makes an external AML assessment by a
    CBI-approved auditor a condition of staying in business (files 17, 18).

## 1. Scope and method

- Files 08-18 profile 55 suppliers in eight groups (section 2), plus the integrators and consultancies that work in
  Iraq (file 18 section 9). Each vendor file records ownership, modules,
  deployment, detection, AI features, case management and reporting, data model, Arabic and MENA presence, price
  signals, rankings, what it does right and wrong with evidence, and derived requirements (prefixes VA, VB, VC, VD).
- Vendor pages were read on 2026-10-06. The session's web-search budget ran out part-way through the vendor work, so
  later facts come from direct page fetches. G2, Gartner Peer Insights, Business Wire and several vendor sites blocked
  automated access; Forrester and Chartis reports are paywalled, so quadrant positions are taken from vendor releases
  and marked as such (file 11 section 5).
- This file does not repeat vendor detail. It compares, ranks fit for Iraqi banks and turns the comparison into
  product requirements.

## 2. Market map: eight groups of vendors

| Group | Vendors profiled | What they sell | Typical buyer | File |
|---|---|---|---|---|
| 1. Incumbent enterprise suites | NICE Actimize, Oracle FCCM, SAS AML, SymphonyAI NetReveal, FICO TONBELLER Siron (divested), Nasdaq Verafin, Fiserv AML Risk Manager | Full AML suites: monitoring, screening, customer risk, case management, reporting | Large and global banks; Verafin and Fiserv serve North American community banks | 08-11 |
| 2. AI-native detection | Quantexa, ThetaRay, Hawk, Feedzai, Featurespace (Visa), Tookitaki, Consilient | Models, anomaly detection, entity resolution and networks, often as an overlay on an incumbent | Tier-1 banks, payment firms, fintechs | 12 |
| 3. Investigation automation and challengers | Lucinity, Silent Eight, WorkFusion (UiPath), Napier AI, Unit21, Flagright, Salv, Sardine, Hummingbird, Bretton AI, Arva AI | AI adjudication of alerts, investigation agents, modern case management, cloud-native suites | Banks with large alert backlogs; fintechs | 13 |
| 4. Screening engines | LexisNexis (Firco, Bridger), Eastnets, FinScan, Dow Jones RiskCenter, Swift screening services, Napier, OpenSanctions (open data and matcher) | Name and payment screening against sanctions, PEP and adverse-media lists | Every bank; payment screening is often a separate purchase | 14 |
| 5. Risk data providers | LSEG World-Check (named in the CBI's 2025 controls), Dow Jones, Moody's (Orbis, Grid), ComplyAdvantage, Sayari, Kharon, Castellum.AI, OpenSanctions; official free feeds (UN, OFAC, UK, EU, Iraq's IQTFS API) | Sanctions, PEP, relatives, adverse-media, ownership and company data | Every bank, through the screening engine | 15 |
| 6. KYC and client lifecycle (CLM) | Fenergo, Pega, Moody's Passfort and Maxsight, Encompass; identity proofing: Uqudo, Entrust (Onfido), Sumsub, IDnow, Jumio | Onboarding, periodic review, customer risk rating, document workflows, identity checks | Corporate and private banks; digital onboarding | 16 |
| 7. Core-banking-native AML | Temenos Financial Crime Mitigation, Azentio AMLOCK and FCM (with iMAL, formerly Path Solutions), Capital Banking Solutions CapitalCompliance (with CapitalBanker and PowerBanker), Oracle FCCM (with FLEXCUBE through OFSAA); partner AML on Infosys Finacle and Finastra; no AML engine at ICS BANKS, BML ICBS or Intellect | AML modules sold with, or certified for, the core banking system | Banks that buy from their core vendor; in Iraq four banks (two Azentio, two Capital Banking) | 17, 18 |
| 8. MENA and Iraq players | Pio-Tech (Jordan: AML, goAML and risk-based-assessment modules on its Bank-BI warehouse), 6 Sigma (Temenos marketplace); integrators ITSS, Prema, AL-AWAEL, Iraqna IBTS; consultancies that shape buying (K2 Integrity, EY, KPMG, Oliver Wyman) | Compliance and reporting modules, implementation, training, data channels | Iraqi and regional banks, e-payment companies | 18 |

## 3. What the market does right (table stakes for AKR-TMS)

These are capabilities every serious buyer will expect. AKR-TMS must match them before any differentiator counts.

| Practice | Who shows it best | Evidence | AKR-TMS answer |
|---|---|---|---|
| Hybrid detection: rules plus prioritisation model plus anomaly layer | All groups 1-3 | File 12 section 9.1 | Rules first (CBI catalogue), unsupervised and peer scores day one, supervised models after label gates (T-xx, file 21) |
| Segmentation and above/below-the-line tuning | SAS, NICE | Files 08, 10 | Scenario studio with segment parameters and BTL sampling (mockup 05) |
| Entity-level alerting (customer roll-up) | SAS (documented), SymphonyAI (Metro Bank: 20% fewer alerts) | Files 10, 11 | Alerts grouped per customer and per network cluster |
| Explainable screening scores and decision reuse | LexisNexis (Strength Index, decision reapplication), FinScan (field-level scoring) | File 14 | Field-by-field "why this fired" panel; remembered decisions per pair with expiry |
| Change sandbox with a diff and synthetic typology tests | Napier AI | Files 13, 14 | Every scenario or list change replayed on history before approval |
| Policy-bound AI adjudication with sampling and drift control | Silent Eight | File 13 | Assistant recommends within written policy; random QA samples of closed alerts |
| LLM control design | Unit21 (eval sets from real decisions, deterministic query generation, context scoping, judge monitoring, escalation); Hawk and Oliver Wyman (decision-rights ladder); Chartis "agent washing" red flags | Files 12, 13 | The assistant writes queries against the canonical model, never raw tables; autonomy tier per function |
| Filing lifecycle as a core module | Oracle (goAML STR app: XSD templates, validation, e-file, reject workflow), Flagright, Hummingbird; in Iraq, Pio-Tech's goAML management system (live at two Islamic banks, 2024) | Files 09, 13, 18 | goAML builder validated against the Iraqi schema before submission (mockup 07) |
| Read data in place, store only what is needed | Hummingbird (queries the customer's warehouse, Postgres supported) | File 13 | Read-only replica link, canonical model, no write path (file 25) |
| One standard image plus a per-bank overlay | SAS (Bangkok Bank Global Image; SDC running 120 Nordic banks) | File 10 | One product image, bank-owned mapping and parameters as versioned configuration |
| Typology and intelligence sharing under a legal basis | Verafin (consortium), Tookitaki (AFC Ecosystem cards), Salv (Bridge, under Estonian statute) | Files 11, 12, 13 | Scenario cards with performance statistics; template-based requests; pooling only in a regulator-led phase |
| Packaged product for small institutions | Verafin, NICE Xceed and Essentials | File 11 | "Small bank pack": pre-mapped, minimal administration, fixed price |

## 4. What the market does wrong (recurring failures, with evidence)

| Failure | Evidence | Consequence for the bank | AKR-TMS rule |
|---|---|---|---|
| Heavy, product-specific data pipelines | A bank's Actimize runbook: three staging databases and about ten servers; Oracle: 67 staging templates, none for Islamic products | Months of ETL work; products the model does not know go unmonitored | One canonical, product-agnostic model with mapping packs per core system (file 25) |
| Scenario change is slow | TD Bank staff: no scenario added "since we first implemented SAS"; NICE's own brochure cites "slow and lengthy tuning cycles" | Scenarios fossilise; regulators find gaps | Same-day scenario change with replay, approval and history (mockup 05) |
| Batch-only cores | NICE SAM daily batch, Oracle nightly to quarterly, SAS nightly core schema | Detection hours or days after the money moved | Polling every few minutes on the replica; pre-release screening API in phase 2 |
| Licence meters that punish growth | Oracle: transactions, screens, users, templates and AI narratives metered; NICE: separate packages and change orders; Verafin: 3-year prepaid | Costs rise with volume; banks switch scenarios off to save money | Flat annual price per bank tier, all modules included |
| Closed documentation | SAS behind an access key; Oracle's data model and scenario descriptions behind its support portal | Banks cannot explain the system to supervisors | Every scenario, model and parameter documented in plain language inside the product |
| Claims that cannot be compared | 30% to 99% false-positive reductions; recall rarely reported | Purchases on promises that are never measured | Claims ledger; replay pilot on the bank's own data (section 5) |
| Cloud-only AI | Oracle agents, SAS Foundry, SymphonyAI, NICE hosted GenAI; no vendor documents a fully local model | On-premises banks get none of the new features | Local models only; no mandatory outbound path |
| Arabic treated as an edge case | Oracle on-premises matcher: Arabic "will not match"; LexisNexis native Arabic comparison exact; OpenSanctions admits weak Arabic; none handles mother's name, chain position or Kurdish letters | Missed hits on Iraqi lists, or floods of false hits | Iraqi-name pipeline with a public test set (file 22) |
| Autonomy beyond what Iraqi rules allow | "Executes decisions" wording (Silent Eight); managed services that close alerts (Lucinity, Bretton); self-tuning from analyst decisions (Napier) | Conflicts with CBI rules that the MLRO decides and records reasons | AI drafts and ranks; a named bank user decides; tuning only through approved change requests |
| No Islamic products | No incumbent or challenger documents murabaha, ijara, musharaka, mudaraba or sukuk data | Islamic banks' main products are monitored as generic loans or not at all | Islamic contract model and scenario pack (file 24) |
| No local presence | No global incumbent has an Iraqi office or contact line; the only Iraqi references are in banks' annual reports (Siron at Bank of Baghdad and NBI, Firco at Bank of Baghdad and Trans Iraq Bank, Azentio at Ameen Al-Iraq, Capital Banking at Trans Iraq Bank) and on regional vendors' pages (Azentio, Capital Banking, Pio-Tech) | Slow support, no Arabic training, no knowledge of CBI practice | Local team and Arabic documentation |

## 5. Claims versus evidence

| Claim type | Range claimed | Best independent or method-disclosed figure | Source |
|---|---|---|---|
| False-positive reduction (detection) | 30% to 99% (vendor pages) | HSBC with Google AML AI: 60% fewer alerts and 2-4 times more suspicious activity found (customer-reported, method disclosed, B); BIS Aurora: about 40% fewer false positives for ML in silos (synthetic data, A for architecture) | Files 11, 12 |
| Agent auto-resolution | 75% (Sardine), 77% (Flagright), 80% (Silent Eight, WorkFusion), 92% (Arva), 98% (Hawk back-test) | HKMA sample, Bank B in production: 7% of monitoring alerts auto-closed, with sampling, a dormant period and re-open monitoring (regulator-compiled, A) | File 13 section 10.3 |
| Alerts that lead to an STR | rarely published | 27% of Level 2 alerts at one NICE customer; survey: 70.8% of banks say 5% or fewer end in escalation or an STR; HKMA: 80% of top-scored alerts became STRs | Files 11, 12 |
| Rankings | Forrester and Chartis "Leader" in releases | Paywalled; SAS highest current offering (4.70) and SymphonyAI highest strategy score in Forrester FCM Q3 2026 per vendor releases | File 11 section 5c |

Reading: vendor claims count different things (screening noise, name mismatches, low-risk monitoring alerts under a risk
appetite the bank never chose) and seldom report recall. AKR-TMS should plan for **7-30% auto-closure in the first
year** under sampling, make the **miss rate** the headline measure, and publish definitions, baselines and samples for
every number (VB-21, VB-22, VB-35, VB-51; VO-01).

## 6. Fit for Iraqi banks

Fit is judged for the default Iraqi deployment: the bank's own data centre, a read-only replica of the core banking
database, Arabic users, the CBI's rules, reporting to the AML/CFT Office through goAML, and a budget far below a global
bank's. Cells summarise files 08-18; "?" means not stated publicly.

| Vendor | On-premises | Arabic matching | goAML | Islamic products | MENA references | Iraq presence | Legal screen | Fit |
|---|---|---|---|---|---|---|---|---|
| NICE Actimize | Yes (containerised SAM-10) | 23 "cultures", list unpublished | Not found | No | UAE Exchange (2015) | None | **Israeli HQ: blocked pending counsel** | Low |
| Oracle FCCM | Yes (OFSAA) | **On-premises matcher ignores Arabic**; cloud transliterates | **Documented** (cloud) | No | Contacts in 10 Arab countries, not Iraq | None | US; OFAC support risk | Medium for large banks |
| SAS AML | Yes (Viya, Remote Managed) | Cultural-origin AI and transliteration in screening | Not found | No | Mashreq since 2006; Dubai, Riyadh, Doha | None | US; OFAC support risk | Medium for large banks |
| SymphonyAI NetReveal | **No** (SaaS, hybrid cloud) | "60+ languages" lists | Not found | No | BBK Bahrain (Dec 2024) | None | US | Low unless a sovereign option appears |
| Nasdaq Verafin | **No** (SaaS, North America) | No | No | No | None | None | US | Not available |
| Quantexa | Yes, but needs Spark, Elasticsearch, Kubernetes | Arabic parsing "on the horizon" (Mar 2025) | Filing partial; goAML not stated | No | DIFC, Doha | None | UK | Medium on features, low on cost |
| ThetaRay | **No** (SaaS on Azure) | No evidence | ? | No | Mashreq (2022) | None | **Israeli HQ: blocked pending counsel** | Blocked |
| Hawk | Overlay only | None found | Filing module; goAML not stated | No | None | None | Germany | Low-medium |
| Silent Eight | **Yes** | **Arabic names case; Emirates NBD, Mashreq, FAB, ADIB** | SAR pre-fill | No | Yes (Gulf banks) | None | Singapore | Medium (adjudication only, needs upstream alerts) |
| Napier AI | **Yes** | **Arabic among 20+ languages** | Filing partial; goAML not stated | No | DIFC office since 2021 | None | UK (Marlin Equity) | **Medium-high among vendors** |
| Flagright | No (SaaS; no Middle East region) | None | **"70+ goAML jurisdictions"** (vendor; Iraq not named) | No | None | None | Private; HQ not verified | Low-medium |
| Hummingbird | No (SaaS on the customer's warehouse) | None | goAML among listed filings | No | None | None | US | Low-medium |
| LexisNexis Firco and Bridger | Yes | Native Arabic compared exactly; fuzzy after transliteration | n/a | n/a | Saudi tuning role | None found | UK/US (RELX) | Medium for screening |
| Eastnets SafeWatch | Not stated (hosted option) | Undocumented | n/a | n/a | Dubai HQ; Qatar, Egypt, UAE, Jordan, KSA; lists "QI Card" | Not confirmed (no Iraqi bank case study; "QI Card" logo only, file 18) | UAE | Medium for screening |
| FinScan | Yes (Windows) | **Best documented** (prefixes, hamza, laqab, order) | n/a | n/a | Gulf banks and exchange houses | None | US | Medium for screening |
| OpenSanctions | Yes (Docker) | States weak Arabic | n/a | n/a | Mirrors the Iraqi freeze list (5,678 of 6,280 rows) | n/a | Germany | Data source and benchmark |
| Temenos FCM | Yes, also cloud and SaaS | Not stated (no Arabic on the FCM pages) | Not found | ? | Cairo Amman Bank (Jordan, under a central-bank mandate) | Temenos core at NBI, ZainCash, BLF; no FCM customer | Switzerland | Medium for Temenos-core banks |
| Azentio AMLOCK and FCM | Deployed in Iraq (hosting not stated) | Arabic for narratives only | Not shown for Iraq (Saudi SAFIU named) | Sold next to iMAL, an Islamic core | Saudi Arabia, Gulf | **Ameen Al-Iraq (since 2021), KIIB (2024-25)** | Singapore (Apax-owned) | Medium for iMAL banks |
| Capital Banking Solutions | On-premises or cloud (vendor page) | ? | **goAML added at Sumer (vendor release, 2022)** | Claims Islamic banks among its Iraqi customers | Lebanon-linked; releases datelined Paris | **AML at Trans Iraq Bank (auditor, 2022-24) and Sumer; core at IMEIB since 2006; claims "more than 25 Iraqi" banks** | France or Lebanon (not verified) | Medium for its own core customers |
| Pio-Tech | ? | "Names Bilingual Translator" (2026) | **goAML module live at two Iraqi Islamic banks (2024)** | ? | 14 Jordanian, 6 Egyptian banks listed | **29 banks in Iraq listed as references** (products not stated); 81, then 96 scenarios added at Trans Iraq Bank | Jordan | Closest competitor in Iraq; no published accuracy |

**Groups 5 and 6 (files 15 and 16).**

- *Risk data.* No supplier publishes Iraqi PEP counts, Arabic or Kurdish source lists or Iraqi company coverage; only
  OpenSanctions shows Iraq-linked numbers, and its copy of the Iraqi national list (5,678 entities) differs from the
  official API (6,280 rows) (file 15 section 12). World-Check is the source the CBI's 2025 controls name; its Data File
  is the only World-Check route that keeps customer names inside the bank, and its documented update cadence ("twice
  daily" in 2019, "daily" in 2026) does not by itself meet the CBI's 12-hour refresh rule, so official lists must be
  loaded directly with a freshness clock (file 15 sections 2 and 17). Only World-Check (Data File), Dow Jones (feeds),
  Kharon (files), OpenSanctions (bulk and on-premises yente) and the official feeds run without sending names out;
  ComplyAdvantage is SaaS-only. Only ComplyAdvantage and OpenSanctions publish prices. Ownership is the largest gap:
  the CBI's beneficial-ownership threshold is 20%, OFAC aggregates to 50%, and Iraq's registry does not verify
  declarations, so the bank's own shareholder tables are the primary source (file 15 section 13; file 26 section 8).
- *KYC and CLM.* Fenergo and Moody's are SaaS-only; the others are cloud-first or unverified; CLM systems expect to be
  the system of record with write access. None shows an Iraqi regulatory profile, the Iraqi ID model or family-number
  relationship detection. Only Uqudo documents the Iraqi national card (mother's and grandfathers' names, family
  number, Kurdish-to-Arabic standardised names, MRZ check) and offers on-premises or in-country hosting. No public
  bank-facing check against the Iraqi civil registry was found; this is a question for the CBI (file 16 sections 6,
  11 and 16). For AKR-TMS these vendors are design references (Fenergo's policy engine and materiality filter, Pega's
  questionnaires, Encompass's attribute lineage, Uqudo's ID and name model) and optional inputs, not competitors: onboarding
  stays in the core banking system and AKR-TMS is the read-only risk, review and evidence layer around it.

**Groups 7 and 8 (files 17 and 18).**

- *Core-banking-native AML.* "Native" covers three things: the core vendor's own engine (Temenos FCM, Azentio AMLOCK
  and FCM next to iMAL, Oracle FCCM fed from FLEXCUBE), a partner product certified on the vendor's marketplace
  (Finacle, Finastra, Temenos), and thin built-in controls such as risk-class fields, blacklists and report packs (ICS
  BANKS, BML ICBS). The core system most Iraqi banks run, ICS BANKS (30+ Iraqi customers claimed, 15 named), has no AML
  engine; nor have BML ICBS, Finacle or Intellect, and Finastra sells payment screening plus partners. Four Iraqi AML
  deployments by a core vendor are evidenced: Azentio next to iMAL (Ameen Al-Iraq, in its annual reports since 2021;
  Kurdistan International Islamic Bank, 2024-25) and Capital Banking Solutions' CapitalCompliance (Trans Iraq Bank,
  auditor 2022-24, where the bank's core is a different product; Sumer Commercial Bank, vendor release 2022, with goAML
  added); no Iraqi bank runs Temenos FCM. The weaknesses repeat: deepest integration only with the vendor's own core,
  end-of-day feeds (FLEXCUBE to OFSAA is a batch), no published Arabic name matching, no current Iraqi pack (auditors at
  Ameen and Trans Iraq Bank count 25 CBI scenarios, against 56 in the pack the MER describes and 86 in the 2025
  controls), database lock-in and ownership churn (Finastra selling Universal Banking to Pollen Street, June 2026). What they do better: no integration work for their own customers, one contract, and a native hook that can
  stop a transaction before posting, which a read-only reader cannot; AKR-TMS answers that with the phase-2 API. For
  AKR-TMS the core vendor is a data source first and a competitor second; adapter order follows the Iraqi evidence: ICS
  BANKS on Oracle, Temenos Transact, iMAL, BML ICBS and Finastra Equation, FLEXCUBE (file 17 sections 10-12).
- *Iraq and the region.* The MER and the CBI name no AML or core banking vendor; the only commercial product in the
  CBI's controls is World-Check (file 18 section 1). Every bank already runs an automated monitor on the CBI's standard
  scenarios and none had built its own at the time of the MER (para 566), so the market is a replacement and upgrade
  market. Banks' audited annual reports name the tools, often two to five per bank: Siron and Firco at Bank of Baghdad;
  a low-cost "AML profiling system", then Siron ("AML Sirion", 2024) at NBI; Capital Banking AML, Firco through a local
  reseller, World-Check, goAML and 96 Pio-Tech scenarios at Trans Iraq Bank, whose 2024 auditor also notes customers
  were not risk-classified; Azentio and World-Check at Ameen Al-Iraq; and Refinitiv plus FinScan on the CBI's own
  remittance platform. The closest competitor is Pio-Tech (Amman): its
  references page lists 29 banks in Iraq and 6 Iraqi non-bank institutions, it sells goAML, risk-based assessment and
  AML modules on a banking data warehouse (the same read-and-model design as AKR-TMS), and it publishes no accuracy
  figures. No global suite has an Iraqi customer, partner or office. Integrators are mostly in Amman (ITSS, Prema,
  AL-AWAEL, ICSFS); K2 Integrity, EY, KPMG and Oliver Wyman shape buying at the state banks and the CBI. The demand
  driver is the CBI's Banking Reform Initiative: every continuing private bank had to submit, by 30 September 2026, an
  assessment of its AML controls (standard D2) by an independent external auditor approved by the CBI; gaps must be
  fixed and resubmitted within six months, and a bank that cannot fix them may be moved to the merger or liquidation
  path (CBI Phase 1 guide, March 2026, checked by the lead). Budgets are small: Bank of Baghdad added USD 0.4m of
  software in 2024 and USD 2.6m in 2025, and NBI's 2023 contracts table shows about USD 3.5k a year to renew its AML
  profiling system and about USD 49k for its core system (file 18 sections 8, 12.2, 13 and 15).

**Verdict by bank size (incumbents, file 11 section 5c):** a large Iraqi private or state bank could run Oracle or SAS
at high cost, with Arabic and goAML gaps to close; NICE is blocked by the legal screen; SymphonyAI and Verafin are
cloud-only. For a small Iraqi bank every incumbent is too heavy or too expensive.

## 7. Legal and procurement screen for Iraq

1. **Israeli-headquartered vendors.** On 26 May 2022 Iraq's parliament passed the law criminalising normalisation and
   relations with Israel (275 of 329 members in favour). It bans "diplomatic, political, military, economic, cultural
   and any other types of relations" for Iraqis, state bodies, private companies and foreign companies operating in
   Iraq, with penalties up to life imprisonment or death (file 12 section 2, AP and Al-Monitor reports). NICE Ltd
   (NICE Actimize's parent) is headquartered in Ra'anana, Israel; ThetaRay in Hod HaSharon, and it bought the screening
   engine Screena in July 2024. A sale of NICE to a non-Israeli owner would not by itself remove the question, because
   development may stay in Israel. **This is a procurement fact, not a legal opinion; Iraqi counsel must confirm.**
2. **US-owned vendors and OFAC exposure.** US vendors screen their own customers against OFAC lists (Feedzai publishes
   such a policy). An Iraqi bank hit by US measures, as Al-Huda Bank was under FinCEN section 311 in 2024, could lose
   licences, updates and support exactly when it needs them (file 12 section 9.4; file 01 section 4). A bank should ask
   every vendor what happens to its licence and support if the bank, or Iraq, is subject to US measures.
3. **Data residency and outbound links.** SaaS-only vendors send customer data abroad; Iraq has no personal-data law yet
   but cloud and hosting rules are anticipated (file 01 section 8), and correspondent banks ask where data sits.
   Swift's Name Screening sends customer data to Swift (file 14).
4. **The AKR-TMS supply chain gets the same screen.** The planned stack (file 32) uses open-source components. Most
   come from foundations or companies outside the scope of the 2022 law (PostgreSQL, Kubernetes and K3s, React, Rust
   crates, Prometheus and Grafana). **Redis** is developed by Redis Ltd., founded in Tel Aviv. Its open-source licence
   does not create a commercial relationship, but the cheapest way to remove the question is **Valkey**, the Linux
   Foundation's drop-in fork of Redis; AKR-TMS should use Valkey and keep a software bill of materials with each
   component's maintainer and country, reviewed by counsel (VO-07).

## 8. The gap: what no vendor offers an Iraqi bank

| Need of an Iraqi bank | Who comes closest | Gap |
|---|---|---|
| Read-only database link to the core system, with proof of completeness | Hummingbird reads in place (SaaS, US) | No vendor documents a read-only replica link with reconciliation and coverage certificates (file 25) |
| CBI scenario catalogue (86 numbered scenarios plus articles 4-14) | Azentio at Ameen Al-Iraq and Capital Banking at Trans Iraq Bank (auditors: "the minimum scenarios required by the CBI, amounting to 25"; Trans Iraq Bank adds 96 Pio-Tech scenarios); Siron at Bank of Baghdad (CBI-approved scenarios) | No vendor publishes coverage of the current catalogue or a versioned mapping to it; 25 compares with 56 in the MER and 86 in the 2025 controls (file 01 section 2d; file 18) |
| goAML filing tested for the Iraqi FIU | Pio-Tech (goAML module live at two Iraqi Islamic banks, 2024); Capital Banking (goAML added at Sumer, 2022); Oracle (documented goAML app, cloud), Flagright and Hummingbird (goAML listed) | Vendor releases only: no validation results or STR quality checks published; the global vendors list no Middle East FIU (file 13 section 12.5; file 18) |
| Arabic and Iraqi name matching (chain names, mother's name, tribe, Kurdish letters) | FinScan (best documentation), Silent Eight (Gulf cases), Napier (language support) | None documents Iraqi naming; none publishes a test set (file 14 section 15.1; file 22) |
| Islamic contracts as contracts | Nobody | No data model or scenarios for murabaha, ijara, musharaka, mudaraba (file 24) |
| On-premises with local AI | Silent Eight, Napier, Oracle, SAS (without their cloud AI) | No vendor documents a fully local LLM (file 12 section 9.8) |
| Arabic interface, right to left, Hijri dates | Not documented by any incumbent | Undocumented everywhere (files 08-14) |
| Evidence for correspondent banks and the CBI | Brand comfort only ("gives regulators comfort", PeerSpot review of NICE) | No product generates an evidence pack per audience (file 05 section 5) |
| Price an Iraqi bank can pay, with local support | Regional suppliers (Pio-Tech, Azentio, Capital Banking) with Iraqi customers; support mostly from Amman | No vendor publishes prices; global vendors meter licences and have no Iraqi office |
| Continuity if the vendor is sold or the bank is sanctioned | Nobody | Escrow, open export and support bridges are not standard |

## 9. What it means for AKR-TMS

1. **Build, integrate, interoperate.** Build what nobody does well for Iraq: the read-only data layer, the CBI scenario
   catalogue, the Iraqi-name matcher and scorer, the Islamic model, the goAML builder, the governance and evidence
   layer. Integrate paid data by feed or API (World-Check, Dow Jones, LexisNexis, OpenSanctions as options; files 14,
   15). Interoperate with engines banks already run (Firco, Eastnets, Swift services): import their alerts and
   decisions rather than demand a rip-and-replace (file 14 section 15.5).
2. **Sell the evidence, not the algorithm.** Regulators punished missing data, narrow configuration and untested
   changes. Coverage certificates, versioned configuration, a standing test suite, a change sandbox and list-freshness
   clocks are the product's core, and they are what correspondents ask for (files 14, 20, 25).
3. **Prove it on the bank's own data before contract.** Replay 12 months of history, compare with the incumbent engine
   in a parallel run, report alert volumes, overlap and the reports that would and would not have been made (VA-54,
   VA-55).
4. **Keep humans deciding.** AI ranks, drafts and explains; a named user decides and records reasons, as CBI rules
   require (file 01 section 2d; file 29). State each AI feature's maturity level (Chartis's four levels; VB-53).
5. **Make continuity a feature.** Source escrow, open export of configuration, cases and evidence, long-term-support
   releases and a support bridge if the vendor changes hands (VA-20, VA-21). Signed offline update bundles for lists,
   scenarios and models, so nothing depends on an outbound link (VA-56).
6. **Publish the price.** Flat annual bands by bank size, everything included, and a small-bank pack. Total cost of
   ownership is the lever in a market that buys to comply: Bank of Baghdad added USD 0.4-2.6m of software a year in
   2024-25 (file 14 section 15.9; file 18 section 13).
7. **Match the table stakes first** (section 3). A bank comparing AKR-TMS with Oracle or SAS will check the same list:
   rule library, segmentation and tuning, entity-level cases, triage with sampling, case management with QA and audit,
   screening at payment speed, STR drafting with human approval, model governance documents, an explanation for every
   alert (file 11 section 6.3).

## 10. Derived requirements (VO-01 to VO-14)

Cross-vendor requirements. Where a vendor file already holds the detailed requirement, its ID is given.

| ID | Requirement | Why (evidence) |
|---|---|---|
| VO-01 | Every effectiveness number in product reports and sales material states its definition, baseline, sample, date and miss rate; the in-product claims register lists each automation feature's closure, sampled miss and re-open rates (VB-21, VB-51) | Claims of 30-99% and 75-98% are not comparable; HKMA's production figure is 7% (section 5) |
| VO-02 | Replay pilot: before contract, run 12 months of the bank's history and report alerts by scenario, overlap with the incumbent engine and the STR cases caught and missed (VA-55) | Banks buy on promises that are never measured (section 4) |
| VO-03 | Parallel-run mode next to an incumbent engine for 30-90 days with side-by-side alert and STR conversion statistics (VA-54) | Lowers switching risk; regulators expect parallel runs before cut-over |
| VO-04 | Import alerts and decisions from installed screening engines (Firco, Eastnets, Swift services) and add Arabic coverage, case management and testing on top (VC-xx, file 14) | Banks will not rip out working payment screening |
| VO-05 | Continuity package: source-code escrow, open export of configuration, cases and evidence in documented formats, long-term-support releases and a 24-month support bridge on change of control (VA-20, VA-21) | NICE, FICO, BAE, WorkFusion ownership changes (finding 7) |
| VO-06 | Published, flat annual price bands by bank size, all modules and offline updates included, plus a small-bank pack | Hidden and metered prices (section 4); small Iraqi banks cannot pay incumbent prices |
| VO-07 | Supply-chain legal screen: software bill of materials with maintainer and country for every component, reviewed by Iraqi counsel; use Valkey instead of Redis; no component, data feed, model or subcontractor from a party barred under Iraqi law | Iraq's 2022 anti-normalisation law (section 7) |
| VO-08 | No mandatory outbound dependency: every feature works offline, with signed update bundles for lists, scenarios and models (VA-56) | Cloud-only AI excludes on-premises banks (section 4) |
| VO-09 | One standard product image plus a per-bank overlay of mapping, parameters and branding, released and versioned separately | SAS Global Image, SDC 120 banks (section 3) |
| VO-10 | A public Arabic and Iraqi name-matching test set with precision and recall, re-run on every release (VB-01; file 22) | No engine documents Iraqi naming (section 8) |
| VO-11 | goAML builder validated against the schema the Iraqi AML/CFT Office uses, with a pre-submission validator and the rejection workflow (VB-43, VB-49; file 23) | Only Oracle documents a goAML application, and not for Iraq (section 8) |
| VO-12 | Each AI feature labelled with its maturity level (rule automation, copilot, agentic with human oversight, autonomous) and its autonomy tier in an agent register (VB-50, VB-53) | "Agent washing" (Chartis) and autonomy that conflicts with CBI rules |
| VO-13 | The table stakes in section 3 are a release gate for version 1 | Buyers compare against Oracle and SAS feature lists |
| VO-14 | Vendor-exit and sanctions clause in the AKR-TMS contract: licences and updates continue if the bank or Iraq becomes subject to foreign measures, to the extent law allows | US vendors' OFAC policies can cut support (section 7) |

## 11. Sources

The evidence and links for every statement are in the vendor files: 08 NICE Actimize, 09 Oracle FCCM, 10 SAS AML, 11
other incumbents and the incumbent comparison (section 5), 12 AI-native detection (section 8 for published bank results),
13 investigation automation and challengers (section 11 for the comparison tables), 14 screening engines (section 14 for
the comparison table), 15 risk data providers, 16 KYC and CLM, 17 core-banking-native AML, 18 MENA and Iraq market. Key
external sources used directly here:

- [Iraqi parliament passes bill criminalising ties with Israel, 26 May 2022 (AP via KSAT)](https://www.ksat.com/news/world/2022/05/26/iraqi-lawmakers-pass-bill-criminalizing-any-ties-with-israel/)
- [Al-Monitor, Iraq's parliament passes anti-normalisation law (May 2022)](https://al-monitor.com/originals/2022/05/iraqs-parliament-passes-anti-normalization-law)
- [FinCEN, final measure against Al-Huda Bank (2024)](https://www.fincen.gov/news/news-releases/fincen-finalizes-financial-measure-against-iraq-based-al-huda-bank-combat)
- [Valkey project (Linux Foundation)](https://valkey.io/)
