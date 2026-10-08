# 14 — Vendors: screening engines (LexisNexis Risk Solutions, Eastnets, FinScan, Dow Jones RiskCenter, Swift, Napier, Silent Eight, OpenSanctions yente)

Status: done (2026-10-06)

Part 3 of 4 of the vendor research (screening engines, risk data providers, KYC tools). Sister files: 15 risk data providers,
16 KYC and client lifecycle management tools. Requirement IDs use the prefix VC-xx and are numbered across files 14, 15 and 16.
Cross-references: file 01 (Iraq rules, sections 2d, 6, 7), file 04 (Wolfsberg sanctions screening 2d, negative news 2i, PEPs 2n,
UN feed 4a), file 05 (OFAC 50 percent rule 1e, NYDFS Part 504 1f), file 13 (Silent Eight), file 22 (Arabic name matching).

## Contents

1. What an Iraqi bank needs from a screening engine (baseline taken from files 01, 04, 05)
2. LexisNexis Risk Solutions (Bridger Insight XG, Firco Continuity, Firco Trust, Firco Compliance Link; FircoSoft and Accuity heritage)
3. Eastnets (SafeWatch Filtering, PaymentGuard, Screening cloud)
4. FinScan (Innovative Systems)
5. Dow Jones RiskCenter (screening platform; the data side is in file 15)
6. Swift (Sanctions Screening, Name Screening, Payment Controls, Sanctions Testing) and the ISO 20022 migration
7. Napier AI (client and transaction screening)
8. Silent Eight (cross-reference to file 13)
9. Open source: OpenSanctions yente and nomenklatura, and other open options
10. Cross-cutting: payment screening on Swift MT and ISO 20022 (what must be screened, truncation, structured data)
11. Cross-cutting: Arabic and Iraqi name matching in the engines
12. Cross-cutting: list management, good-guy lists, testing and tuning, false-positive rates, latency, deployment
13. Enforcement cases and regulator findings that turned on screening failures
14. Comparison table
15. What it means for AKR-TMS
16. Derived requirements (VC-xx)
17. Sources

(Each vendor section covers: what it does, deployment, Arabic and MENA presence, pricing signals, what it does right, what it does
wrong, lessons for AKR-TMS, AKR-TMS hooks.)

## Findings in brief

1. **No engine publicly solves Arabic or Iraqi name matching.** FinScan documents Arabic handling best (Al/Ibn/Bin, hamza forms, laqab, name order); LexisNexis describes native Arabic comparison as exact ("without applying any fuzzy matching logic"); Eastnets, Swift and Dow Jones are silent; OpenSanctions says cross-script matching for Arabic/Farsi "will produce increased error rates". None documents mother's name or chain-position alignment, although the Iraqi national list has 6,280 rows, mother's name on 99.2 percent, birth year on 95.5 percent and only 3 aliases.
2. **Regulators punish configuration, testing, data and operations, not algorithm brands.** RBS 2010 (calibrated once; under 5 percent and 27 percent test misses), Starling 2024 (screening matched 39 of 3,088 names for six years; GBP 28.96 million), the FCA's May 2026 review (90 percent exact, 75 percent variant detection; vendor list errors) and twelve OFAC cases 2019-2025.
3. **Post-event screening cannot interdict.** Napier's own text: screening after settlement "is checking a payment that has already gone". Phase 2 needs a pre-release API (target p99 under 250 ms, FinScan's published "most under 250 ms"), a degraded-mode policy, and complements Swift's in-network controls.
4. **ISO 20022 is a timely door-opener.** Coexistence ended 22 Nov 2025; unstructured addresses are rejected from 14 Nov 2026; Swift's own address model warns Arabic input "may yield unreliable or unexpected results". A read-only payment-address readiness report is deliverable now.
5. **Swift's hosted Transaction Screening and independent Sanctions Testing are the nearest analogue for a mid-size Iraqi bank** (Tadhamon Bank, Yemen: 300+ alerts a day, handling time from 6 hours to 1.5), and Sanctions Testing is a model for regulator-grade independent testing of any filter.
6. **Vendor percentages are not comparable** (one case study claims a "500 percent" reduction). AI that self-tunes or auto-clears alerts is a governance risk under the FCA and CBI expectations.
7. **Open source is usable but incomplete**: OpenSanctions software is MIT (including a Rust name-handling core), data is CC BY-NC (an OEM licence is needed to bundle it), and its copy of the Iraqi list (5,678 entities) differs from the official API (6,280 rows).
8. 40 derived requirements (VC-01 to VC-40).

---

## 1. What an Iraqi bank needs from a screening engine (baseline taken from files 01, 04, 05)

This section restates, in one table, what the rest of the research already established, so each vendor below can be judged against
the same yardstick. Details and sources are in the files named in the last column.

| # | Need | Basis | Where |
|---|---|---|---|
| 1 | Lists: UN consolidated list, OFAC (SDN and non-SDN), EU, UK, and the Iraqi national list (CFTF decisions on IQTFS) | CBI AML/CFT/CPF controls of 8 Jan 2025, ch. 5 art. 5 ("Second"): UN, OFAC, EU and the Committee for Freezing Terrorist Funds | file 01 s.2d, s.6 |
| 2 | Lists refreshed "at least every 12 hours" | Same article, item 3 | file 01 s.2d |
| 3 | Search in Arabic and English; native Arabic matching of the Iraqi name shape (given name, father, grandfather, tribe, mother's name, birth year) | Same article, item 4; the national list is Arabic-only, 6,280 entries on 2026-10-06, each with mother's name and birth year | file 01 s.6, s.7; file 22 |
| 4 | Screen all parties dealt with directly or indirectly, and identify beneficial owners | Same article, item 2 | file 01 s.2d |
| 5 | Named data source: "World-Check" | Same article, item 2 names the World-Check system explicitly | file 01 s.2d; file 15 s.2 |
| 6 | Direct link to the core banking system and to the screening platform ("Automatic Integration") | Chapter 5 art. 5 "First", item 3 | file 01 s.2d |
| 7 | Hold a transaction that matches a scenario until the MLRO has verified it | Chapter 5 art. 5 "First", item 6 | file 01 s.2d |
| 8 | Screen payments before release when USD or cross-border flows are involved (Wolfsberg: real-time for cross-border; domestic may not need it "unless the bank is subject to another regime", for example the US dollar rules) | Wolfsberg Sanctions Screening Guidance 2019; OFAC causing liability | file 04 s.2d; file 05 s.1e |
| 9 | Ownership-aware screening (OFAC 50 percent rule; EU and UK "more than 50 percent or control") | OFAC 2014 guidance | file 05 s.1e |
| 10 | Documented configuration, change management, pre- and post-implementation testing, model validation of the matching logic, data lineage and completeness | NYDFS Part 504 (3 NYCRR 504); FFIEC manual; OFAC 2019 framework; Wolfsberg 2019 | file 05 s.1d-1f; file 04 s.2d |
| 11 | AML tooling verified by a CBI-approved firm; three lines of defence | CBI standards handbook 2025 | file 01 s.2e |
| 12 | On-premises by default, read-only access to the CBS database, no write-back to the CBS | AKR-TMS brief; no explicit data-localisation rule found | file 01 s.8 |

**The limit of post-event screening (important for the product promise).** AKR-TMS phase 1 reads a read-only replica. A
transaction that has already been posted in the CBS can be screened afterwards, and a sanctions hit can be found, but the payment has
by then left the bank. Post-event screening therefore detects and triggers freeze tasks and reports; it does not interdict. CBI
controls item 7 ("stops any transaction that matches ... until the MLRO verifies") and OFAC "causing" liability both need a
pre-release check. That is the job of the phase 2 screening API called by the bank's payment or Swift gateway. Every vendor below is
judged on whether its engine can serve both modes: batch and delta customer screening, and a low-latency payment screening call.

**Two kinds of screening, two kinds of vendors.** (a) Customer (reference-data) screening: names, birth data, IDs, owners, against
sanctions, PEP and adverse-media data. (b) Transaction (payment) screening: message fields in Swift MT or ISO 20022, free text,
BICs, ports, vessels, against sanctions and embargo data. The suite vendors (LexisNexis, Eastnets, FinScan, Dow Jones) sell both;
Swift sells payment-side services; Napier and Silent Eight add AI layers; OpenSanctions supplies open data plus an open matching
engine.

---

## 2. LexisNexis Risk Solutions (Bridger Insight XG, Firco Continuity, Firco Insight/Trust, Firco Compliance Link; FircoSoft and Accuity heritage)

### 2.1 Who they are

- **Heritage.** FircoSoft is a Paris-based watch-list filtering company founded in 1990. Reed Elsevier announced the acquisition on
  3 September 2014 and combined it with Accuity (Bankers Almanac, payment routing data). At that date FircoSoft had "over 700
  customers including eight of the world's top 10 financial institutions" and "2,000 customer sites in more than 85 countries"
  (Reed Elsevier press release). Accuity and LexisNexis Risk Solutions announced their merger on **17 February 2021**; both are
  RELX companies. The Bridger Insight XG line (US, WorldCompliance data) was already inside LexisNexis Risk Solutions.
- **Positioning today.** LexisNexis markets "8 of the world's top 10 banks" for Firco Continuity (counted from Bankers Almanac
  data, January 2023), "nine of the top ten global banks" (press release of 7 July 2025, when Firco Continuity won a Risk
  Technology Awards 2025 product award), and "38/50 top banks use Fircosoft" (LexisNexis APAC deck, October 2025). Chartis ranked
  LexisNexis Risk Solutions fourth overall in its 2024 Financial Crime and Compliance 50.

### 2.2 Products (what each one is)

| Product | Role | Facts from vendor documents |
|---|---|---|
| **Firco Continuity** | Real-time **transaction (payment message) screening**, workflow, audit | "Available on cloud or on premise", "API-first"; "millions of transactions in an hour"; "a choice of more than 40 algorithms ... adjustable by geography, language requirements and risk appetite"; "15+ advanced and optional modules" (decision reapplication, detection of disguised payments / stripping); decision workflow with multi-user validation, "blind mode" and back-up workflow; keeps the full lifecycle of every transaction "including any transaction that has not raised any hit" (brochure NXR15974, 2023) |
| **Firco Continuity: Targeted Matching module** | Field-aware screening for ISO 20022 and MT | Screens grouped fields (name + city + country of the same party); "Comparison Mode" drops similar-name hits when nationality, place of birth or date of birth do not match; "leverage the ISO 20022 structure using standard configuration"; common field mapping through "Firco Unique Format"; fewer business rules to maintain (brochure NXR15384, 2022) |
| **Firco Insight** (customer-database screening; older name **Firco Trust**) | Customer, PEP and REP screening of the client base | Five parts: Entity Resolution Filter, List Management Services (downloads and validates sanctions, PEP and internal lists daily; archive for look-backs), Alert Workflow Interface (cloud option), Alert Review API, Set Up and Validation Service. I could not find a current "Firco Trust" product page; treat the name as legacy |
| **Firco Entity Resolution Filter (ERF)** | Secondary scoring filter that sits behind the name-matching engine | Feature-based **Strength Index** (name frequency NFQ, middle name or initial MNI, date of birth DOB, location LOC, data quality QAL, identity match IDM, burden of proof BP, additional evidence ADD) and an **Exposure Index** (how much it would hurt if the match were true). "Hand-tuned and validated", "fully explainable", unlike "black-box" AI. Classes: highly unlikely (false positive), review, escalate; auto-close of "highly unlikely" is configurable |
| **Firco Compliance Link** | One hub for customer, payment and trade screening (SaaS or hosted) | "115+ watchlists" and "70+ dual-use and controlled goods lists"; MT/MX API support; event-driven re-screening "whenever the watchlist, client or trade data is updated"; vessel and AIS data; "match rate below 4 percent" and "decrease manual review time by more than 80 percent" (customer points of view from 2017-2018) |
| **Bridger Insight XG** | Customer and payment screening, strongest in the US; uses LexisNexis **WorldCompliance Data Plus** | "180 global sanctions lists", "3.4 million+ PEP profiles", adverse media "across 59 languages", "8M+ risk profiles"; "single-tenant and multi-tenant cloud hosting options alongside on-premise installations"; payment screening for "ACH, SWIFT, ISO2022 and FedWire"; XML API; user interface in English, Spanish, French, German, Mandarin, Japanese, Portuguese (no Arabic) (2021 brochure) |

### 2.3 Arabic, Iraq and MENA

- **Native-script matching exists but is described as exact.** A LexisNexis deck (7 October 2025, an ABA webinar hosted at aba.org.tw) says
  the filter "supports any language or alphabet which can be filtered against itself (native vs native) without applying any fuzzy
  matching logic", and shows Arabic against Arabic. For **transliteration** ("matching a non-Latin alphabet against Latin with
  fuzzy matching applied") it lists Arabic among Cyrillic, Greek, Japanese and Chinese. The vendor's own slide therefore implies that
  Arabic-to-Arabic comparison is exact-match in the standard configuration. The Iraqi national list (Arabic only) is matched against
  bank customer names that were typed by clerks in Arabic with variable spelling of hamza, ta marbuta, ya and alef maqsura and
  with dropped or reordered chain elements; exact native matching would miss those. Whether a fuzzy native option is available needs
  a vendor test (file 22 sets out the test cases).
- **Dictionaries.** Internal dictionaries for abbreviations (LTD to LIMITED), first-name translation (Peter/Pierre/Pedro/Pietro),
  honorific titles ("Dr, Shri, Sheik"), country names and cities, ISO country codes inside BICs (XXXXIRXXXXX) and currency codes.
- **Arabic is a tuning job, not a switch.** A recruitment notice for a FircoSoft tuning specialist at a Saudi institution
  (Capitex, Riyadh) says "screening effectiveness in the Saudi market depends on how the filter handles Arabic names" and asks for
  "transliteration handling, romanisation variants, name particles and script-conversion behaviour", effectiveness testing in an
  "above-the-line/below-the-line" style, and Arabic and English fluency. The need for such a post shows that the out-of-the-box
  Arabic behaviour is not enough in the Gulf either.
- **Iraq.** I found no public evidence of an Iraqi bank using Firco or Bridger. *Update 2026-10-06 (file 18): two banks'
  audited annual reports name Firco: Bank of Baghdad (filtering of internal and external transfers before execution) and
  Trans Iraq Bank ("Firco Soft" supplied by a local group, the only AML system not linked to the banking system; 2022
  report p.70, checked by the lead).* LSEG, not LexisNexis, has the documented Iraqi
  channel for screening data (World-Check with a local agent, Iraqna IBTS, 10 March 2020; file 15 section 2). The CBI controls
  name World-Check, which makes a LexisNexis-only data choice a hard sell to Iraqi banks.

### 2.4 Pricing signals (no price list is published)

| Buyer | Product | Value | Date | Source |
|---|---|---|---|---|
| European Commission, DG NEAR | WorldCompliance software to screen beneficiaries (PEGASE direct financial support for Palestine; EUREP contractors) | EUR 300,000 (final value, duration not stated) | awarded 3 Dec 2024 | EU Publications Office procurement record |
| Financial Services Compensation Scheme (UK) | Sanctions screening service, batch against US, UK and UN lists | GBP 307,000 over 1 Mar 2019 to 28 Feb 2022 (about GBP 102,000 a year) | 2019 | TED notice 142139-2019 (contractor LexisNexis Risk Solution UK) |
| Met Office (UK) | Bridger Insight XG sanction lists screening tool | GBP 8,500 for one year | 1 May 2026 to 30 Apr 2027 | UK Contracts Finder |

These are small, low-volume buyers. A bank-scale Firco Continuity deployment (licences, modules, list data, professional services)
is quoted per deployment, and I found no public figure. Capterra and G2 list "contact us for pricing".

### 2.5 What it does right

1. **Depth in payment screening.** Firco Continuity is built for high-volume message screening, keeps a full audit trail of every
   message (even no-hit messages), supports multi-user validation and "blind mode", and has a disguised-payment (stripping) detection
   module. A vendor that "pioneered" stripping detection (its own words) is relevant to Iraqi banks that route USD through
   correspondents.
2. **Explainable, auditable second-stage scoring.** ERF scores named features, shows the result as a Strength Index class, and keeps
   the rule explainable. In a 2024 proof of value LexisNexis reports **93 percent fewer false positives and 73 percent fewer alerts
   needing manual review** (vendor figures, one proof of value, not independently verified). In the same deck, a large regional bank
   screening 1.8 million customer records against sanctions saw 396 "highly likely" hits (0.02 percent) and 17,109 "review" hits
   (0.95 percent); a global tier-1 bank screening 70 million records against PEP data saw about 4,500 highly likely hits (0.006
   percent) and about 50,000 review hits (0.07 percent). These are the only real hit-rate numbers I found for a customer base.
3. **Decision reapplication and good-guy handling** are first-class modules ("reapplying decisions", "automatically resolve
   recurring false positive hits across transactions based on historical qualification made by operators").
4. **ISO 20022 field awareness.** Targeted Matching screens each field with its own rules and uses party structure (name + city +
   country) instead of one concatenated string.
5. **Mature governance features**: documented audit trail, archive and statistics, configurable workflow, and regulator familiarity
   ("the most well-known solution by regulators globally", vendor deck).

### 2.6 What it does wrong (evidence)

1. **Arabic-to-Arabic described as exact matching** (above) and no Arabic user interface in Bridger Insight XG; tuning for Arabic
   names needs specialists (Saudi tuning post).
2. **Configuration is heavy.** More than 40 algorithms and 15+ optional modules, each adjustable; the vendor's own deck lists
   "Test and tuning" and "Impact assessment" as separate stages, and FCA testing (28 May 2026, section 13) found that firms often
   "had limited understanding of how vendor screening logic or configurations operate". A tool this configurable is easy to leave on
   defaults.
3. **Unverifiable efficiency claims.** "False-positive rate of less than one percent against the OFAC watchlists" came from an
   "internal test" on sample data against the SDN list only; the footnote says results "may vary". The ERF and Compliance Link figures
   come from single customer proofs of value. None is comparable across vendors.
4. **Cloud-first drift.** Bridger Insight XG "runs on LexisNexis Risk Solutions servers" (2021 brochure; on-premise has since been
   added); Compliance Link is SaaS/hosted. For an Iraqi bank whose payment data should stay on-premises, only Firco Continuity
   (cloud or on premise) is a clear fit.
5. **Cost and lock-in.** Per-deployment pricing, optional modules, and a proprietary field format ("Firco Unique Format") make
   switching engines expensive. Bridger is oriented to US lists, US data and English-language adverse media.
6. **Data and engine are bundled in marketing.** The list data (Firco Global WatchList, WorldCompliance) and the matching engine are
   sold together; CBI names World-Check, not LexisNexis, as the data source.

### 2.7 Lessons for AKR-TMS

- **Copy** the two-stage design: a fast, recall-oriented name matcher, then an explainable scorer with named features (name
  frequency, middle name, DOB, location, data quality, identity, burden of proof) and a Strength Index class, with auto-close only
  for "highly unlikely" and a full no-hit audit trail.
- **Copy** field-aware screening and Comparison Mode (suppress a name hit if nationality, place of birth or birth year clearly differ).
- **Beat** the Arabic problem by designing the matcher around the Iraqi name chain from day one: native Arabic fuzzy matching with
  letter normalisation, chain-position alignment, optional tribe token, **mother's name** and **birth year** as strong features,
  and Kurdish letter mapping (file 22).
- **Avoid** unexplained defaults: ship conservative defaults, show every setting with a plain-language effect and a test result,
  and require a sign-off to change it (FCA, NYDFS 504).
- **Avoid** per-module pricing that penalises compliance depth.

**AKR-TMS hooks**
- Explainable two-stage scorer with named features and a Strength-Index-style class; features include name frequency computed from
  the bank's own customer base (a very common Iraqi chain should need more supporting evidence than a rare one).
- "Comparison mode" rule: a name-only hit is auto-discounted only when a reliable secondary identifier (birth year, mother's name,
  national ID number) clearly conflicts; the discount rule is versioned and appears in the audit trail.
- Pair-keyed decision memory (customer record + list entry + list version) that is invalidated when either side changes.
- No-hit audit trail: every screened record and every screened payment is logged with list version, algorithm version and result,
  including the ones with no alert.

---

## 3. Eastnets (SafeWatch Screening, PaymentGuard, PaymentSafe, Swift hosting)

### 3.1 Who they are

- Dubai-headquartered compliance and payments vendor with 15 offices; the founder says the company began in **Amman, Jordan, in
  1984** with software that connected banks to Swift. It bought **SIDE International** (Belgium, AML software) in 2007 (Finextra),
  which is where the SafeWatch filtering heritage sits. It is also an accredited **Swift Service Bureau**: it hosts Swift
  connectivity for banks that do not want to run it themselves.
- Size claims (Eastnets, 2025-2026): "over 800 financial institutions" in "90 markets", "15 of the top 50 banks" and "22 of the
  world's central banks". In October 2021 the same boilerplate said 750 institutions and 11 of the top 50 banks, so the
  customer count is growing. SafeWatch Screening was "installed in over 360 banks" (2021 case study) and holds the **Swift
  Alliance Plug-in label** (integrates with Swift Alliance Access/Gateway message flows).
- **Chartis RiskTech Quadrant 2025** (published 28 August 2025, Watchlist and Adverse Media Monitoring Solutions): Eastnets named a
  category leader for name and transaction screening ("depth and scalability of its screening solutions, which serve a broad
  range of institutions"). Chartis' 2025 vendor list also includes ACTICO, FinScan, Sigma360 and others; in 2024 KYC360 and LSEG
  Risk Intelligence led.

### 3.2 Products

| Product | What it does | Facts |
|---|---|---|
| **SafeWatch Screening 5** (launched 2024; "almost complete roll out" to the customer base by November 2024) | Real-time (blocking) and batch screening of customer databases, payment messages and transaction files against sanctions, PEP, SIP, adverse media and internal lists | RESTful API, Kafka and MQ back ends, microservices; "unlimited" watch lists; four-eyes maker-checker; **Good Guys (white lists)**; **Decision Replay** (re-applies earlier decisions to similar detections); "Smart SWIFT Fields Interpretation"; **List Set Rank**; **Violation Filter** (analyses flagged instances to cut false positives); **AIDA**, an AI Decision Assistant that suggests an action per alert with a confidence level from historical data; version control, offline test environments and list comparison; **Real Time List Loading (RTLL)**; **ChainFeed**, which Eastnets says delivers "instant, secure watchlist refreshes" using "proprietary blockchain technology" |
| SafeWatch Screening integrations | "SWIFT, SEPA, XML and Fedwire" message screening; file scanner (delta and full files) plus a screening API (Prime Bank case) | ISO 20022 compliant configuration claimed |
| **LSEG World-Check On Demand** inside SafeWatch | Real-time risk data through an API, without extra integration layers | Announced 2 January 2026 |
| SafeWatch AML, SafeWatch KYC, PaymentSafe (payment hub), PaymentGuard, SafeTrade (trade-based money laundering), Swift hosting | Rest of the suite | SafeWatch Screening works with the PaymentSafe payment hub |

Deployment: Eastnets' pages do not list deployment models. The product is described as "cloud enabled micro-services"; Eastnets
moved its **Americas Swift and sanctions screening infrastructure to Microsoft Azure** (21 March 2023), so a hosted option exists.
Implementation time claimed: "8 weeks or less". No published throughput or latency figures (I looked for them).

### 3.3 Arabic, Iraq and MENA

- **MENA position is the strongest of any screening vendor in this file**: Dubai headquarters, Arab customers named in case studies
  (Al Jazeera Finance in Qatar, American Express KSA, Arab International Bank in Egypt, Commercial Bank International in the UAE,
  Capital Bank of Jordan) and a CBI (UAE) service-bureau deployment.
- **Iraq.** The SafeWatch Screening product page lists "QI Card" among notable customers. I read that as the International Smart
  Card Company (Qi Card), Iraq's largest card issuer and acquirer (established 2007 with Al-Rafidain Bank); the page does not say
  so and I could not confirm it from a second source. I found no named Iraqi bank customer.
- **Arabic matching is not documented publicly.** Eastnets' own blog acknowledges that "names can be written in a variety of ways
  ... even more pronounced when languages such as Arabic or Mandarin are transliterated into Latin-based characters", that
  "fuzzy matching is effective at text matching but not necessarily entity matching", and that rule-based scoring suffers
  "rigidity and fragility" because changing weights "can have major consequences on the final score". The proposed remedy is
  "contextual" AI that extracts gender, name origin and entity type from names (claimed reduction "upwards of 70 percent",
  legacy systems "upward of 95 percent" false positives). The product pages say nothing about native Arabic-script matching,
  the Arabic user interface, or Kurdish.

### 3.4 Pricing signals

None public. No tender award for Eastnets turned up. Eastnets quotes licences plus optional hosting; the "8 weeks or less" claim and
the Swift-hosting bundle suggest a services-heavy model.

### 3.5 What it does right

1. **Fits the payment world Iraqi banks live in**: Swift certified, Alliance plug-in label, a message hub and Swift hosting in the
   same vendor, so screening sits in the flow before release (the CBI "hold until the MLRO verifies" requirement).
2. **List agility as a product feature**: RTLL, list comparison, version control and offline test environments address the "list
   updated every 12 hours" and "test before you load" duties. Neutral on list suppliers: it integrates LSEG World-Check (named by the
   CBI), plus other feeds.
3. **MENA references and regional presence**: Gulf banks and a Jordanian bank; a vendor that already deals with Arabic customers
   and Arab regulators (SAMA, QCB, CBUAE).
4. **Good-guy and decision replay** are core, and **delta file scanning plus API** match the AKR-TMS phase 1 and phase 2 split.
5. **Chartis recognition** for depth and scalability (Chartis 2025).

### 3.6 What it does wrong (evidence)

1. **Marketing numbers that cannot be right.** The Al Jazeera Finance case study claims a "500% reduction in false positive alerts
   and a 170% decrease in false positive detections"; a reduction cannot exceed 100 percent. Other claims: "up to 90 percent"
   (American Express KSA), 60 percent (Prime Bank, Kenya; the case study names no baseline or volumes), "upwards of 70 percent"
   (blog). A buyer cannot compare these with anything.
2. **Arabic is asserted, not shown.** The only public Arabic statement is the problem statement in a blog post. For a Gulf-based
   vendor, that is a gap in public evidence; test before relying on it.
3. **Documentation behind a portal and no performance figures** (throughput, latency, hardware sizing) are published.
4. **Data dependence.** The engine does not own the premium data; it needs World-Check, Dow Jones or LexisNexis feeds, so the buyer
   pays for engine and data and inherits the data vendor's update lag. The FCA's May 2026 review found "errors or omissions in
   sanctions lists provided by third-party vendors" and "delays or failures in updating" lists (section 13).
5. **AI assistant without published validation.** AIDA gives "confidence levels informed by historical data and behavioural
   insights"; no performance or bias test is public, and CBI's handbook requires AML tools to be verified by a CBI-approved firm
   (file 01 s.2e). A black-box recommendation needs a validation pack.
6. **Hosted Swift plus screening concentrates risk and data** in the vendor's cloud (Azure in the Americas). For an Iraqi bank that
   wants payment data on-premises, the hosted route needs a data-protection decision (file 01 s.8).

### 3.7 Lessons for AKR-TMS

- **Copy**: "file scanner plus API" as the two integration modes (delta file or table scan for phase 1; low-latency API for phase 2);
  list set rank (which list wins when two lists give the same person); offline test environment with list comparison before a list
  goes live.
- **Copy**: neutral data supply: the engine should accept several data feeds and let the bank choose, including World-Check.
- **Beat**: publish plain, reproducible effectiveness numbers (alerts per 1,000 records on a named test set, detect rate on a
  published variant test) instead of marketing percentages.
- **Beat**: publish Arabic matching behaviour with test cases, not slides.
- **Avoid**: an AI recommendation that cannot be explained alert by alert; keep AI as a ranked suggestion with feature-level reasons
  and an off switch.

**AKR-TMS hooks**
- **List staging area**: every new list version loads to a staging copy, is compared with the previous version (added, changed,
  removed entries), is tested against a regression test set, and only then goes live; each step is logged. This serves the 12-hour
  CBI rule and the FCA finding on list errors.
- **List precedence ("list set rank")**: if several lists carry the same person, a rule decides which record and which action
  (block, review, inform) applies, and the alert shows all sources.
- **Two integration modes** as a first-class design: scan (delta or full, from the read-only replica) and call (API with a
  latency budget).
- **Effectiveness report in plain numbers** exported for the CBDDQ and for the CBI-approved verifier.

---

## 4. FinScan (Innovative Systems, Pittsburgh)

### 4.1 Who they are

- FinScan is the AML brand of **Innovative Systems**, a Pittsburgh data-quality and data-matching company founded in 1968 ("more than
  50 years" in data matching, linking and name screening, per FinScan). Unlike the suite vendors, FinScan grew out of a data
  matching engine, which shows in its design: a built-in data-quality step and field-by-field scoring.
- Recognition: Chartis RiskTech 2025 category leader in two quadrants; QKS SPARK Matrix leader; Datos Insights AML Impact Awards 2025
  silver medal for best sanction/watchlist screening innovation (for **FinScan Payments**, launched June 2023, enhanced March 2025,
  "6x faster response" and "95 percent reduction in false positive matches" per the award text, which are vendor-supplied). It states a
  customer retention rate of 93 percent (17 February 2026).
- **LSEG validation**: FinScan was validated for **LSEG World-Check On Demand** access on 22 July 2026 (the data flows into FinScan
  workflows through the integration). Eastnets announced the same kind of integration on 2 January 2026. Both matter for Iraq because the
  CBI controls name World-Check (file 15 section 2).

### 4.2 Product and technical facts

| Item | Fact (source: FinScan pages and press, 2026 unless stated) |
|---|---|
| Modules | Sanctions screening ("70+ global sanctions, PEP, and watchlists"), payment screening (SWIFT, SEPA, FedNow; IACH and Fedwire ISO 20022 native support added in the 17 Feb 2026 update), customer onboarding screening, adverse media, UBO verification, trade finance document screening ("3-8x false positive reduction" claimed) |
| Matching | "Unlike traditional fuzzy matching, our proprietary field-based algorithm scores each part of a record on its own"; fuzziness can be set "at the individual field level"; primary criteria (name, address) and secondary criteria (date of birth, passport number, ID number); different rules per risk level, jurisdiction and data set; every match shows "exactly why every match fired" |
| Data quality | Built-in engine that "standardizes, de-duplicates, and corrects your records" (FinScan Enhance, API-first) |
| Deployment | "SaaS, on-premise, or hybrid"; Windows on-premise listed on Software Advice; open API |
| Performance (vendor figures, 17 Feb 2026) | "beyond 100 million transactions daily"; response "sub-second, with most under 250 milliseconds"; "66 milliseconds per transaction in single-threaded environments"; "300B+ records annually" screened across customers |
| Certification | "SWIFT-certified", "ISO 20022 native", SOC 2 Type II |
| False-positive claims | "Up to 80 percent fewer false matches" (generic); customer cases: 62 percent across 15 countries (payment processor, "19,000+ hours" saved), 67 percent while screening 375 million records, 74 percent with improved fuzzy matching |
| Reviews | Software Advice: 4.3 out of 5 from 13 reviews, 80 percent would recommend; praise for documentation, support, value; complaints: "Reporting needs a lot of work", "Not really user friendly and too technical vocabulary", "Continually having to tick features when accessing search" |
| Pricing | Quote only. A third-party site (Spendbase) publishes a **modelled, unverified estimate**: about USD 500-1,000 a month for a starter package up to "often $100k-$300k+/year" for global enterprise; treat as a rough signal only |

### 4.3 Arabic, Iraq and MENA

- **FinScan documents how it handles Arabic names, which most vendors do not.** Its blog on culture-sensitive matching lists: prefixes
  "Al-", "Ibn", "Bin" that are "inconsistently handled as part of the first name, last name, or discarded completely"; transliteration
  variants of one Arabic name (Mohamed, Mohammad, Muhammed); inconsistent Arabic diacritics and letter forms (it shows the three
  spellings of Ahmad that differ only in the hamza: أحمد, إحمد, احمد); a tribal name (laqab) next to the last name; and reversed
  name order. It offers "native-to-native screening" where the list exists in the native script and "transliteration technology where
  native lists absent".
- Its MENA white paper says that "approximately 20 percent of banking data might be of poor quality" in the region, that names often
  carry father's and grandfather's names which "can lead to a high rate of false positives if not properly managed", and that the
  region should "prioritize language handling capabilities".
- **References in the region**: a Money20/20 Middle East exhibitor page lists Ahli Bank, Al Rayan Bank, a Qatari bank and Riyad
  Bank; FinScan's bank page shows the exchange houses Al Dahab Exchange, Al Mulla Exchange and Wall Street Exchange. Software Advice
  lists Arabic among supported languages. I found **no Iraqi customer**.

### 4.4 What it does right

1. **Field-level scoring and explainability.** Each part of a record is scored separately, with the reason for each match visible; this
   is the design the FCA and NYDFS want (documented intent, testable logic).
2. **Data quality is part of screening.** Standardising and correcting records before matching, with an API for real-time cleansing,
   answers the FCA finding on missing, incomplete or placeholder dates of birth and the NYDFS data-integrity attribute.
3. **Published latency numbers** (most under 250 ms, 66 ms single-threaded) give a benchmark for a phase 2 pre-release call.
4. **Best-documented Arabic handling** of the engines reviewed, including the tribal name and the hamza variants.
5. **Deployment flexibility** (SaaS, on-premise, hybrid) and list-agnostic design; World-Check On Demand integration validated.

### 4.5 What it does wrong (evidence)

1. **Weak reporting and usability** in user reviews (13 reviews, small sample); "too technical vocabulary".
2. **Marketing claims that are single-case or unverifiable** (80, 95, 62, 67, 74 percent), with no common test set; "6x faster" has no stated baseline.
3. **US-centric payment rails in the 2026 update** (IACH, Fedwire, FedNow, SEPA Instant). Nothing in the update concerns the Arab payment systems or Iraqi rails.
4. **No Iraqi references and no stated Kurdish or mother's-name handling.** Mother's name as a matching feature is the Iraqi national list's key
   identifier (file 01 s.6) and none of the engines in this file advertises it.
5. **Data poverty is not solved by matching.** FinScan itself says about 20 percent of regional data is poor. The engine cleans, but cannot
   invent a missing mother's name or birth year.
6. **Windows on-premise heritage** (Software Advice) and a US vendor: sanctions exposure for the vendor itself is low, but support in Iraq
   would depend on a partner I could not identify.

### 4.6 Lessons for AKR-TMS

- **Copy** field-level fuzziness, per-field scoring and the "why did this fire" panel; copy the idea of a data-quality pass in front of
  screening (in AKR-TMS this belongs in the CBS mapping layer, so the same cleaning serves screening, scenarios and risk rating).
- **Copy** the native-to-native versus transliteration split: an Iraqi bank has Arabic customer names and (a) the Arabic-only national list,
  (b) UN entries with original-script names (379 of 1,010 entries, file 04 s.4a), (c) Latin-script OFAC, EU and UK entries. All three
  comparisons are needed: Arabic to Arabic, Arabic to Latin by transliteration, and Latin to Latin.
- **Beat** it by adding Iraqi-specific features (mother's name, birth year, family number, tribe token, Kurdish letter mapping) and by
  publishing a test set.
- **Latency target**: FinScan's "most under 250 ms" is a realistic competitive target for the phase 2 API; set AKR-TMS target at p99
  under 250 ms for a single payment (with all lists in memory) and measure it.

**AKR-TMS hooks**
- Per-field fuzziness settings in the screening configuration (name, father, grandfather, tribe, mother's name, birth year, ID
  number), stored as a versioned policy object.
- "Why this fired" panel: field-by-field scores and which normalisations were applied (for example "hamza removed", "ibn/bin dropped",
  "chain position 2 and 3 swapped").
- Data-quality score per customer (missing birth year, placeholder dates, missing mother's name, one-word names), with a work
  queue for the branch, because poor data limits screening more than the algorithm does.
- Three comparison modes (native, transliterated, Latin) reported separately in test results.

---

## 5. Dow Jones RiskCenter (screening platform; the data side is covered in file 15 section 3)

### 5.1 What it is

Dow Jones Risk & Compliance sells **data** (sanctions, PEPs, adverse media, ownership) and **screening tools** built on that data. The
tool family (pages checked on 2026-10-06) is called RiskCenter:

| Product | What it does | Facts from Dow Jones pages |
|---|---|---|
| **RiskCenter Advanced Screening & Monitoring** | Batch and continuous screening of customers, third parties and suppliers; AI-ranked alerts | "Always-on monitoring"; "sub-second searches"; **address screening** (alert when a third party shares an address with a BIS Entity List address or any sanctioned profile "even if the identities do not match"); "AI Risk profiles" to cut analyst load; adverse media screening of unstructured Factiva content ("thousands of licensed global news sources") |
| **RiskCenter Financial Crime** | Web search tool and database for onboarding and EDD | "4 million consolidated risk profiles"; filters on "original script names, official identity numbers and company registration numbers, image URL, addresses and date of birth"; adjustable "fuzzy logic" and confidence; audit trail of every search; beneficial ownership through Dun & Bradstreet |
| RiskCenter Third-Party and Trade Compliance | Same data for corporates | Dual-use goods, vessels, ports and cities in sanctioned regions |
| **Data feeds and APIs** ("RiskFeeds") | Data delivery into the bank's own engine | Feeds by SFTP cloud storage or Pub/Sub; REST **Ad Hoc Search API** and **Screening & Monitoring API** ("bulk (10K+ names)"); custom subsets, filtering by list type and active status; "single file delivery" |
| **LPTS feed** (Lists for Payments and Transactions Screening) | Sanctions profiles formatted for high-speed payment engines | "updates up to six times a day"; consolidated at field level; jurisdictions can be customised |

Dow Jones also sells the data as feeds for other engines (the data feeds page and the developer platform say so); the engines in
this file are list-agnostic (FinScan says so; Eastnets and FinScan both embed LSEG World-Check On Demand, sections 3 and 4). I did not
find a statement of which engines load Dow Jones data. There is **no on-premise Dow Jones engine** in the public material: the
RiskCenter tools are hosted by Dow Jones; an on-premise bank would load the feeds into its own matcher.

### 5.2 Arabic, Iraq and MENA

- No Dow Jones product page checked mentions Arabic, Iraq or the Middle East. The language claims are "records are multi-lingual and
  contain original script names" with Cyrillic and Chinese used as the examples ("searches in Cyrillic and Chinese language
  characters, for example, will return results in English if desired"). Factiva is said to deliver "33,000+ sources in 33 languages
  across 200 countries"; whether Arabic and Kurdish Iraqi outlets are among them is not stated.
- The money-services-business data set covers Canada, Hong Kong, Malaysia, the Philippines, Singapore and the US only, so Iraqi and
  regional exchange houses and hawala-adjacent businesses are not in that set.
- The sanctions data "covers over 80 sanctioning authorities" on one page and "over 50" on another page; whether the Iraqi
  national list (IQTFS) is among them is not stated. This needs a direct vendor question (file 15 section 3).

### 5.3 Pricing signals

| Buyer | What | Value | Date | Source |
|---|---|---|---|---|
| BaFin (German financial supervisor) | "25 Lizenzen DowJones RiskCenter mit Factiva Pro" for checks on appointments in supervised entities, money laundering and other supervisory tasks | EUR 277,380 (contract duration not stated in the fields I could read; about EUR 11,100 per licence) | contract concluded 14 Feb 2024; notice 97959-2024 | EU TED API |

This is a seat-licence price for analyst use; a bank-wide batch or API licence is quoted separately and no figure is public.

### 5.4 What it does right

1. **Ownership-derived data is the clearest in the market**: "Sanctions Control and Ownership (SCO)": "more than 54,000 companies owned
   or controlled by individuals, entities or regions sanctioned by OFAC, the EU, the UK, and eight other jurisdictions" (RiskCenter
   page) or "over 56,000 entities ... enabling organizations to comply with the OFAC 50% rule and equivalent laws" (data feeds page).
   This is the practical way a bank applies the 50 percent rule without building the ownership graph itself.
2. **Payment-ready sanctions feed** (LPTS) with up to six updates a day, which is well inside the CBI's 12-hour minimum.
3. **Address screening** and state-owned company data (government ownership of 1 percent or more, voting control, board control).
4. **Delivery choices** (web tool, REST API, SFTP/Pub/Sub feeds) suit a bank that runs its own engine.
5. **Factiva** is the strongest licensed news archive for adverse media, and Dow Jones allows screening of unstructured content at scale.
6. **Used by supervisors**: BaFin buys 25 RiskCenter licences for its own work.

### 5.5 What it does wrong (evidence)

1. **Arabic and Iraq are silent in all public pages**; examples are Cyrillic and Chinese. No Arabic user interface is listed (the
   site languages are German, English, Spanish, French, Japanese, Korean, Portuguese, Chinese).
2. **Inconsistent numbers on Dow Jones' own pages**: 50 vs 80 sanctioning authorities; "more than 54,000" vs "over 56,000" SCO entities;
   Dun & Bradstreet ownership described as "270 million companies" on one page and "nearly 580 million records" on another. A buyer
   must ask for the dated data dictionary, not the web page.
3. **Beneficial ownership comes from a partner (Dun & Bradstreet)**, not from Dow Jones research; D&B's coverage of Iraqi companies
   and Kurdistan registrations is not stated and I expect it to be thin (unverified; test in file 15 section 13).
4. **Hosted tooling only.** No on-premise engine, so data must leave the bank for ad-hoc search, or the bank must run its own matcher
   on feeds.
5. **Pricing is per seat and per module**, so adverse media (Factiva), ownership (SCO/D&B) and payments feeds add up; there are no public
   bank-scale prices.

### 5.6 Lessons for AKR-TMS

- **Copy** the idea of "derived designation" data: a table of companies owned or controlled by sanctioned persons, with the ownership
  chain, percentage and source, refreshed on every list change.
- **Copy** the LPTS idea: a **payment-optimised view** of the unified watch list (consolidated at field level, small, in memory) separate
  from the research view. This is what makes the phase 2 API fast.
- **Copy** address matching, but design it for Iraqi addresses: governorate, district, neighbourhood (mahalla), street or alley,
  building, landmark; free-text "near" descriptions; Arabic and Latin forms.
- **Avoid** dependence on one ownership supplier; keep the bank's own KYC ownership data as the first source and add commercial data
  when licensed.

**AKR-TMS hooks**
- **Derived-designation table**: entity id, owner chain with percentages, aggregation rule (OFAC 50 percent aggregate; EU/UK more than 50
  percent or control), source and date, linked list entry; used by screening as extra watch-list records.
- **Payment view of the watch list**: a compact per-list index (names, aliases, identifiers, BICs, vessels, countries) rebuilt on each
  list version and held in memory by the phase 2 service.
- **Address screening** against list addresses, with an Iraqi address normaliser.
- **Dow Jones as one pluggable data source** (feed or API) among others, never hard-wired.

---

## 6. Swift (Transaction Screening, Name Screening, Sanctions Testing, Payment Controls) and the ISO 20022 migration

**Source caveat.** swift.com returns HTTP 403 to automated clients, so I could not read the current product pages. All Swift
product facts below come from dated Internet Archive snapshots of swift.com (dates given) and from Swift PDFs it served at the time.
The ISO 20022 pages (section 10) are snapshots from November 2025 to June 2026. Treat product details as "as of the snapshot date".

### 6.1 What Swift sells for screening

| Service | What it does | Facts (snapshot date) |
|---|---|---|
| **Sanctions Screening**, renamed **Transaction Screening** (2021) | Fully managed, Swift-hosted real-time screening of payment messages | Hosted "in SWIFT's operating centre, using private and secure cloud technology" (Aug 2020 FAQ); "no infrastructure required on your side"; two set-ups: **Copy** (FIN messages only) and **Connector** (all messages including SEPA, ISO 20022, Fedwire and non-Swift formats); alert-only or **block** until released; optional four-eyes; own private lists; more than 30 lists (2020) then "70+ sanctions lists collected directly from regulators" plus Sanctions Control and Ownership lists from Dow Jones (fact sheet, March 2021); matching threshold bands per list; routing rules by message type, currency and destination; exceptions and suppression rules; alerts segregated by business unit; **Smart Screening** that uses structured MT and ISO 20022 fields (for example, an originator or beneficiary name can alert only against sanctioned entities, not against addresses or countries); **Transaction Explorer** to look back 124 days "whether or not they generated an alert"; API connectivity. "Join over 900 banks and corporates already using our screening solutions" (Sept 2021; 700+ in 2020) |
| **Name Screening** | Hosted screening of customer databases and single names | Sanctions, PEP, relatives and close associates (RCA), sanctions ownership research and private lists; lists "sourced directly from regulators and through our partnership with Dow Jones"; "delta-delta" screening of new or modified names; configurable match-elimination criteria; priced "based on your level of usage" (Aug 2020 FAQ) |
| **Sanctions Testing** | Independent web tool to test, tune and validate **any** transaction, customer or PEP filter, in-house or third-party | Uses public data, "no customer data is used"; "more than 50 sanctions lists"; formats MT, MX, CHIPS, Fedwire and customer files; "a comprehensive library of derivations" for exact and fuzzy tests (misspellings, acronyms) with BICs, countries and cities; list alerts and change reports; optional **Peer Assessment** (aggregated baseline from peers); positioned as model validation (snapshots 2019 and 6 Feb 2023) |
| **Payment Controls** | Fraud and anomaly rules on outgoing payments (not sanctions) | "In-Swift network, out-of-band secondary control"; screens MT103, 202 and 202 COV and ISO equivalents; blocks or alerts "before they are released"; from August 2022 uses pseudonymised account information shared by the Swift community; daily independent reports; case study with Khan Bank (Mongolia) (snapshot 2022) |

### 6.2 An analogue for Iraq: Tadhamon Bank (Yemen)

Swift's July 2021 case study describes **Tadhamon International Islamic Bank (Yemen)**: established 1996, 37 branches, more than
700 staff, assets YR 497 billion (about USD 2.3 billion). It processes "over 300 alerts every day"; the same alert-handling task fell
from "6+ hours" to "1.5 hours"; it started migrating at the end of 2020 and went live in April 2021 "despite disruption from the
pandemic", without additional on-premise hardware; it can "upload our own lists that comply with internal policies" and says it
has "far fewer false positives" (no number given). This is the nearest published analogue to an Iraqi Islamic bank (a bank in a
fragile, sanctions-exposed country, sized like a mid-tier Iraqi bank) and shows that hosted Swift screening is a real option for them.

### 6.3 Arabic, Iraq and MENA

- No Arabic-specific claim for the matching engine in any snapshot. The engine is a hosted black box with "logical tests (to detect
  misspellings, abbreviations and phonetic similarities)". The Swift address-structuring model warns that "inputs in non-Latin scripts
  such as Arabic ... may yield unreliable or unexpected results" (section 10).
- Whether the Iraqi national list (IQTFS) is among Swift's 70+ lists is not stated; a bank could load it as a **private list**.
- A Yemeni Islamic bank is a named reference; no Iraqi bank is named in the material I could read.

### 6.4 Pricing signals

Name Screening is "based on your level of usage" (Swift account manager quotes). Transaction Screening pricing is not public; as a
hosted service there is no licence or hardware cost, which is the selling point to smaller banks. I found no tender notice with a price.

### 6.5 What it does right

1. **Sits in the network.** A message held in the Swift network cannot leave until released; it needs no change to the bank's
   core system or payment gateway beyond Swift connectivity. For a bank that cannot touch the CBS, this is the cleanest pre-release
   control for cross-border payments.
2. **Independent testing as a product.** Sanctions Testing separates the tester from the vendor of the filter, works on any filter,
   needs no customer data, and offers peer benchmarks; this is the model for a regulator-grade testing practice.
3. **Fit for small banks**: no hardware, quick set-up (Tadhamon: about 4 to 5 months from start to live).
4. **ISO 20022 ready** and aligned with the Swift standards process; the lists include ownership-derived entities (Dow Jones SCO).
5. **Look-back**: Transaction Explorer keeps 124 days of all screened transactions, alerting or not.

### 6.6 What it does wrong (evidence)

1. **Covers only what travels on Swift.** Domestic IQD transfers, RTGS or ACH flows that do not use Swift, card payments, cash
   deposits and branch transactions are outside it (the "Connector" option can take other formats but needs the bank to send them).
   The CBI wants all movements monitored.
2. **Opaque engine, no Arabic evidence** and a 124-day window; the bank cannot see or version the algorithm, and the "independent
   reviews" are available to users, not published.
3. **Customer data leaves the bank** for Name Screening (hosted by Swift, "protected by EU data privacy laws"), which Iraqi banks
   must weigh against their own secrecy rules (file 01 s.8).
4. **Dependence on one third party and the Swift connection.** If a bank's Swift access is restricted (as can happen to
   sanctioned or de-risked banks), the in-network control disappears at the worst moment.
5. **Lists beyond the big regimes** (the Iraqi list, regional exchange-house watchlists) are the bank's job to load and test.
6. **Vendor-stated metrics only** ("alerts doubling every four years", Tadhamon timings).

### 6.7 Lessons for AKR-TMS

- **Do not compete with Swift in-network control; plan to complement it.** For banks that already use Transaction Screening, AKR-TMS
  adds what Swift cannot: domestic flows, customer screening in Arabic, case management, evidence vault, and a cross-check that
  everything that was screened by Swift is also in the AKR-TMS record (reconciliation of messages sent vs screened).
- **Build a testing harness in the spirit of Sanctions Testing**: public data only, versioned test sets (exact, fuzzy, Arabic variants,
  chain reorderings), scheduled runs, tracked results; and accept Swift Sanctions Testing as an independent second opinion on the
  AKR-TMS filter (it tests any filter).
- **Copy Smart Screening**: field semantics decide which list types a field may match.
- **Copy Transaction Explorer**: keep every screened message with its result for the retention period (not 124 days; CBI requires
  five years for records and OFAC now expects ten for sanctions records, file 05 s.1e).

**AKR-TMS hooks**
- **Import of Swift screening results** (alerts and releases exported from the bank's Swift interface or Alliance logs) so one case
  system covers Swift-screened and domestic flows; reconcile counts daily.
- **Test-set library and scheduler**: regression and effectiveness tests (above-the-line, below-the-line) run before every list or
  configuration change, with results attached to the change record.
- **Field-semantics map** for messages and CBS tables (name of a party, name of a bank, address, free text, vessel, goods) so that a
  name field only matches name entries.

---

## 7. Napier AI (Client Screening, Transaction Screening; Continuum platform)

### 7.1 What it is

London-headquartered AML software company (offices: London, Belfast, Copenhagen, **Dubai**, Singapore, Kuala Lumpur, Sydney, New York),
"trusted by more than 150 financial institutions", ISO 27001:2022 certified, Chartis RiskTech 100 and RegTech 100 listed. Its
**Continuum** platform combines Client Screening (real-time and batch; sanctions, PEP, adverse media), Transaction Screening (every
payment rail), Transaction Monitoring (100+ inbuilt typologies, rules plus AI) and Perpetual Client Risk Assessment. Data comes from
partner providers ("we partner with leading providers of real-time data"); Napier does not publish which. Pages read 2026-10-06.

### 7.2 Product facts (Napier pages, 2026)

| Topic | Fact |
|---|---|
| Matching | "Fuzzy name matching spans more than 20 languages and scripts, including simplified Chinese and Arabic, handling transliteration, variant spellings, nicknames and partial matches"; "AI-driven cross-language and linguistic knowledge matching" |
| Two stages | (1) fuzzy matching creates the alert; (2) a second AI engine scores the alert on match quality and attributes (date of birth, ID, nationality) and gives a plain-language narrative; analyst decisions feed back and "tighten thresholds automatically over time" |
| Multi-configuration | Independent screening set-ups per customer type, risk level, list, payment rail or business unit (for example more sensitive for sanctions, tighter for PEP) |
| Sandbox | No-code rule builder and sandbox: test a change on real data with "a direct comparison against the current configuration" before it goes live; whitelist rules are time-boxed with an audit log |
| Entity extraction | Reads a payment message first, classifies persons, organisations, vessels, sender versus intermediary, isolates addresses from free text, validates IBAN, BIC, ISIN and LEI; resolves securities identifiers to issuers; screens vessel names, IMO and MMSI numbers |
| Rails | "SWIFT MT, ISO 20022, SEPA Instant, Fedwire and NACHA/ACH" parsed out of the box |
| Agents | "Responsible agents" built in an AI Studio that triage or clear known patterns with a configurable autonomy level, all logged |
| Deployment | "fully managed SaaS solution, in any private cloud, or on-premise" |
| Scale and claims | One brokerage went from 3,000 to 25,000 names per minute; one customer's alert volume fell from 12 percent to 2 percent of screened records; Banco do Brasil "reduces false positives by more than 90 percent" (foreign branches, replacing a legacy tool); Starling CTO: "deploy in just six weeks" |
| Philosophy statement | "Screening that runs after settlement, or in an overnight batch, is not checking the payment, it is checking a payment that has already gone" |

### 7.3 Arabic, Iraq and MENA

- Dubai office; Arabic is named in the language list. There is no Arabic user interface claim, no description of how Arabic names are
  parsed (father, grandfather, tribe, mother's name), no Kurdish, and no Iraqi or Gulf bank reference on the pages read.
- Napier's customer logos shown are mostly UK, Australian and global names. I could not confirm any MENA bank customer.

### 7.4 Pricing signals

None public. Positioning emphasises "lower total cost of ownership" through self-serve configuration and "six weeks" go-live.

### 7.5 A screening failure that led to Napier: Starling Bank (FCA Final Notice, 27 September 2024)

Starling selected Napier in July 2023 after its own review found that, since July 2017, its customer screening had compared customers
only with the 39 of 3,088 UK-listed Designated Persons who had UK citizenship or residence, because of a misconfiguration; screening
ran once every 14 days; not all cross-border payments were screened; payments were screened with a customer-screening tool. The
back-book review covered 3,988,143 payments and generated 795,712 alerts. The FCA fined Starling GBP 28,959,426 (after a 30 percent
discount). Details are in section 13. The relevance to this vendor is that the "build to buy" move, with third-party testing afterwards,
was the remedy the regulator recorded.

### 7.6 What it does right

1. **Sandbox first.** Test a rule or threshold on real data against the live configuration before it is released, with a
   comparison. That is the NYDFS 504 and FCA expectation turned into a workflow.
2. **Multi-configuration screening** avoids one global threshold; sanctions can be loose and PEP tight.
3. **Entity extraction before matching**, with identifier validation (IBAN, BIC, LEI, ISIN, IMO), reduces noise and closes gaps that
   a name-only screener has (securities issuers, vessels).
4. **Narrative explanations** for each recommendation, written for analysts and regulators.
5. **Flexible deployment** including on-premise; fast go-live record (Starling six weeks).
6. **Honest framing of real-time**: its own copy says post-settlement or overnight screening "is checking a payment that has already
   gone", which is the reason for the AKR-TMS phase 2 API.

### 7.7 What it does wrong (evidence)

1. **Self-tuning thresholds are a model-risk issue.** "Every analyst decision then feeds back ... tightening thresholds automatically
   over time" means the screening logic drifts with analyst behaviour; if analysts are under time pressure (the FCA's May 2026
   review records analysts bypassing escalation "under pressure to meet internal targets") the system learns to under-alert. No
   guard-rails (drift tests, floor thresholds, sanctions exclusion) are documented. CBI requires verification of AML tools by a
   CBI-approved firm (file 01 s.2e).
2. **Agents that clear alerts** (even with a human in charge) conflict with the CBI rule that a movement that matches a scenario is
   stopped "until the MLRO verifies it is sound" unless clearing rules are documented, approved and tested; the vendor's
   "responsible" label is not a regulatory status.
3. **Arabic described only by a language count**; no technical description, no test results.
4. **Vendor-only metrics** (more than 90 percent, 12 percent to 2 percent, sevenfold) with no baseline definitions.
5. **No Iraq or MENA bank reference**; data suppliers unnamed, so list coverage (including the Iraqi list) must be asked about.

### 7.8 Lessons for AKR-TMS

- **Copy** the sandbox-with-diff workflow (every change shows the alert delta on a real sample before release).
- **Copy** entity extraction for payment text (party roles, identifiers, vessels), but keep it deterministic and auditable first.
- **Avoid** auto-tuning from analyst decisions without guard-rails; if learning is used, it proposes a change that goes through
  maker-checker, with drift reports, and never lowers a sanctions threshold by itself.
- **Avoid** auto-clearing sanctions alerts; allow auto-discount only by documented rules with expiry, and sample the discounts
  for quality assurance.

**AKR-TMS hooks**
- **Change sandbox**: every proposed change to a threshold, list rule, whitelist entry or algorithm is replayed on a stored sample
  of history from the read-only replica, showing added and removed alerts and any missed known positives from the test set, before
  maker-checker approval.
- **Learning proposals, not learning actions**: analyst decision patterns produce suggestions (e.g. "this rule cleared 99.8 percent of
  its last 5,000 alerts") that need approval; each rule has an expiry date and a re-validation test.
- **Auto-discount register**: every automatic discount rule listed with its reason, owner, expiry, sample QA result.

---

## 8. Silent Eight (cross-reference to file 13, section 2)

Silent Eight is covered in full in **file 13 section 2** (Iris 7, an AI adjudication layer for screening and transaction-monitoring
alerts). This section records only what matters when it is paired with the engines in this file.

- **Role.** It does not match names. It takes the alert from a screening engine (the vendor says it "sits on top of the bank's
  existing screening engine and TM system"), assembles the customer, transaction and list context, applies the bank's policy and
  closes or escalates with a narrative. So the engines in sections 2 to 7 remain necessary; Silent Eight changes the cost of the alerts
  they produce, not their recall.
- **Evidence in file 13.** Gulf customers named by the vendor (Emirates NBD, Mashreq, First Abu Dhabi Bank, Abu Dhabi Islamic Bank);
  on-premises, private cloud, hybrid or managed deployment; vendor claims of "up to 90 percent" case resolution and "100 percent
  precision" are unverified; a commissioned Forrester TEI (June 2026) reports 184 percent three-year ROI for a composite customer; the
  first deployment bank and a major user are also investors.
- **What it implies for AKR-TMS (consistent with file 13).** Alert adjudication is the layer most worth building as a bounded assistant:
  deterministic policy checks first, a local language model only for text, full reasoning path, sampled QA of anything auto-discounted,
  and no autonomous closure of sanctions alerts. An Iraqi bank's MLRO must still make and sign the decision (CBI controls, file 01
  section 2d). The "executes decisions" positioning would need a documented CBI-acceptable operating model before it could be used in Iraq.
- **Pairing risk to watch.** A discount layer sitting on an engine whose Arabic recall is weak hides misses rather than reducing noise:
  the FCA's May 2026 review (section 13) found misses caused by engine configuration, not by alert volume. Test recall on the engine
  first (section 11), then judge the adjudicator.

---

## 9. Open source and open data: OpenSanctions (yente, nomenklatura, rigour, followthemoney) and other open options

### 9.1 What OpenSanctions is

OpenSanctions Datenbanken GmbH (Berlin) publishes a deduplicated global dataset of sanctions, PEPs, regulatory and enforcement lists, and
the open-source software around it. Figures on its licensing page on 2026-10-06:

| Item | Figure |
|---|---|
| People | 1,134,248 |
| Organizations | 297,233 |
| Securities | 230,011 |
| Crypto addresses | 14,049 |
| Vessels and aircraft | 10,512 |
| Enforcement notices | 3,483 |
| Sources | "460+"; "47mn+" facts; 30 entity types |
| Update frequency | "four times per day" (the on-premise service checks hourly) |
| Customers | "650+ clients worldwide" |

**Open-source software (all MIT licence, Python, active in the week of 2026-10-05; GitHub metadata)**:
- **yente**: the matching and search API ("entity search and bulk matching ... Supports Reconciliation API spec"). Ships as two Docker
  containers (application plus Elasticsearch); "8GB RAM, 1vCPU, 60GB hard drive" minimum, 16 GB and SSD advised.
- **nomenklatura**: framework to integrate and de-duplicate FollowTheMoney data streams from many sources.
- **followthemoney**: the data model (entities, properties, nested entities; "every stated value is linked to a public source").
- **rigour**: name, identifier and language utilities, **with a Rust core (`rigour-core` 2.5.0, MIT, edition 2024)** for tokenising and
  aligning names, person and organisation prefix stripping, a 4,000-line data file of organisation descriptors in many languages
  (including Arabic "شركة"), phonetic keys (Metaphone and Soundex via the `rphonetic` crate) and a residue-distance scorer whose
  documentation says a budget can be shifted "between strict (payment-screening) and permissive (KYC-onboarding) profiles".

### 9.2 Matching algorithms and their stated limits

| Algorithm | What it is |
|---|---|
| `logic-v2` ("best since 2025-12-01") | Deterministic rules: "fuzzy and culturally-aware dictionary-based name matching", rules for IMO, ISIN, LEI, OGRN, INN and other identifiers, and penalties when country, date of birth, gender or address diverge. Calibrated to threshold 0.7 |
| `ofac` (added 2026-06-26) | Name-only emulation of OFAC's Sanctions List Search tool, "reverse-engineered from OFAC's scoring FAQ": best score over every combination of query and candidate name (aliases included), whole name and tokens, **Jaro-Winkler**; optional mismatch penalties |
| `logic-v1`, `name-based`, `name-qualified` | Older; `name-based` and `name-qualified` (Jaro-Winkler plus Soundex) retire on 2027-01-18 |
| `regression-v1`, `regression-v2` | Logistic regression on many features; high scores only for multi-attribute matches |

Operation: a two-stage process (an index finds candidates for recall; then candidates are scored); threshold default 0.7 ("could be raised
to 0.8 or even 0.85" for sanctions with low false-positive tolerance); feature weights between 0 and 1 are adjustable per request; every
score is explained. Training data: 760,000 manual pair judgements (December 2025), shared as a "grouped pairs" file of about one million
rows, regenerated monthly (access with a delivery token; commercial use needs a licence).

**Limits stated by OpenSanctions in its own tuning documentation (quoted):**
- "Name matching is less precise when used in conjunction with writing systems that are not Western-style alphabets. In particular,
  the fuzzy comparison between different writing systems will produce increased error rates. This affects writing systems including
  **Arabic/Farsi**, Burmese, the systems used in China, Japan, Korea and many Indian languages."
- "Phonetic matching (Soundex, Metaphone) does not support any non-alphabet writing systems."
- "Name aliases and nicknames: ... Such dictionaries are not currently included in the OpenSanctions matching system."
- Company names are "particularly vulnerable to misspellings in the legal type parts".
- In the `rigour-core` source, built-in transliteration to Latin covers **only Latin, Cyrillic, Greek, Armenian, Georgian and Hangul**;
  Arabic, Han, Thai and Devanagari are left to a separate Python library ("rigour does not try to duplicate that surface").

### 9.3 Arabic, Iraq and MENA

- **Iraqi data.** The Iraq page lists exactly **one** source published by an Iraqi authority: "Iraq Terrorist Fund Freezing Lists"
  from the AML/CFT Office, updated daily, **5,678 entities**. The official public API on the same day (2026-10-06, my own count) showed
  **6,280 rows** (6,253 individuals and 27 entities; mother's name on 6,230 rows, birth year on 6,000, an alias on 3, no row marked
  as excluded). The 602-row difference is unexplained (de-duplication, lag or filtering); a bank must not assume the open
  copy equals the official list.
- The Iraq page also counts 1,262 people and 43 legal entities connected with Iraq (by country property, so it does not include every
  row of the freeze list), topic tags of **724 PEP** and 387 "politician" (the page does not say how the tags overlap), 343 sanctioned, 325 debarred and 264
  export-controlled, and five sanctions programmes that target Iraq (EU, UK, Switzerland, UN 1518, US). The page reads "724 people
  identified as political office-holders connected with Iraq". Quality of the PEP set is assessed in file 15.
- No Arabic-specific matching rules beyond the generic ones; no Iraqi name-chain logic; no mother's-name feature in the scorers.

### 9.4 Licensing and cost (primary source: OpenSanctions pages)

| Route | Terms |
|---|---|
| Data | CC BY-NC 4.0: free for non-commercial use; "Businesses must acquire a data license" |
| **Screening API** (hosted) | Prepaid credits, one credit per metered operation, valid one year; 2,000 free credits on sign-up; EUR 50 for 500 credits (EUR 0.10 per query), EUR 400 for 5,000 (0.08), EUR 2,000 for 30,000 (0.067), EUR 5,000 for 100,000 (0.05), EUR 15,000 for 500,000 (0.03); more than 1 million: quote; prices exclude VAT; an EU VAT number is required to sign up |
| **Screening licence** | "A flat-rate licence to use the full dataset inside your own organization", "monthly or annual flat rate for unlimited data usage and screening volumes", internal use only; price on quote; delivery by bulk or delta files, on-premise yente or API |
| **Reseller/OEM licence** | Needed "if you load our bulk data into a database, search index, matching engine, or yente deployment that serves your customers" (Commercial FAQ). The standard API terms allow screening through the hosted API for customers and showing the results, but not passing on the data collection or giving customers their own API access |
| Software | yente and nomenklatura are MIT, so "we won't charge you to run it"; only the data needs a licence |
| Scope | Licensed "as a complete collection", not by part; integrations can still select entity types and topics |

For AKR-TMS the licence route is a product decision (file 15 section 9): either each bank holds its own screening licence and AKR-TMS
only connects to it, or AKR-TMS takes an OEM licence and bundles the data. A hosted API is not acceptable for customer names leaving an
Iraqi bank, so the on-premise route (bulk or delta files) is the realistic one.

### 9.5 What it is not (the vendor's own words)

"OpenSanctions provides the database and matching technology used to build financial crime screening systems. We do not provide an
end-user compliance application with alert review, case management, and approval workflows." It also gives no service-level agreement for
the free website, no adjudication layer and no Arabic user interface.

### 9.6 Other open-source options (for completeness)

| Project | Licence and stack (GitHub, 2026-10-05) | Relevance |
|---|---|---|
| **Moov Watchman** | Apache-2.0; Go | An in-memory sanctions screening engine for OFAC (SDN, non-SDN, CSL, FinCEN 311), EU, UK, UN and OpenSanctions files; explainable field-level scores (`debug=true`); IDs such as passport, IMO or crypto address score 1.0; optional TF-IDF and cross-script embeddings. The project reports, on 472,477 labelled people, companies and vessels from OpenSanctions Pairs, Jaro-Winkler at `minMatch=0.80` precision **0.99**, and cross-script embeddings raising recall from **0.69 to 0.82** at precision 0.95 (project's own benchmark, not Arabic-specific) |
| **Splink** | MIT; Python, several SQL back ends | Probabilistic record linkage (Fellegi-Sunter) with term-frequency adjustments; a published method for the name-frequency effect and for de-duplicating the customer base |
| Jube | AGPL-3.0; C# | Open-source AML and fraud transaction monitoring; AGPL makes embedding in a sold product legally difficult |
| Tazama (Linux Foundation) | Apache-2.0 | Payment fraud and AML monitoring framework (not screening) |
| Marble | GitHub shows no licence asserted | Not assessed |

### 9.7 What it does right

1. **Open, auditable matching**: published weights and rules, an explanation for every score, and a pair-judgement training set; the
   only engine in this file whose logic a bank's model-validation team can read in full.
2. **Data provenance**: every value is linked to a source; deduplicated across lists.
3. **Fast list refresh** (four times a day; on-premise service polls hourly) and **delta files** (suits AKR-TMS delta re-screening).
4. **On-premise and cheap to evaluate**: Docker, standard terms, free evaluation before licensing.
5. **A national-list feed from Iraq's AML/CFT Office** (even if incomplete) and an `ofac` algorithm to benchmark against the regulator's own tool.
6. **Rust core for names** under MIT, matching AKR-TMS's stack.

### 9.8 What it does wrong (evidence)

1. **Arabic is a documented weak spot** (their words, section 9.2), with no Arabic transliteration in the Rust core and no nickname dictionaries.
2. **Not a compliance system**: no alert workflow, no case management, no adjudication, no testing service.
3. **Commercial licence required for banks**, with price on quote and complete-collection licensing; hosted API is the default route, which Iraqi banks cannot use for customer names.
4. **Elasticsearch dependency** in yente (two containers) and Python runtime; heavier to operate than a single Rust service.
5. **Incomplete mirror of the Iraqi list** (5,678 versus 6,280 rows) and the PEP data (724 people for a country with a large public sector) need checking before reliance.
6. **No warranty on matching quality**: the algorithm names change (`best` moved to `logic-v2` on 2025-12-01), so pinning is needed.

### 9.9 Lessons for AKR-TMS

- **Copy** the architecture: candidate retrieval for recall, deterministic scoring with explanations, identifier rules (IMO, ISIN, LEI, BIC)
  that confirm a match regardless of name, and published weights.
- **Copy** the statement-based provenance model (each fact tagged with source, first seen, last seen) for the unified watch list.
- **Use** `rigour-core` (MIT, Rust) as a starting point for the Latin side (tokenising, prefixes, organisation types), and write the Arabic
  and Iraqi layer ourselves (file 22).
- **Use** OpenSanctions as one optional data feed (bulk and delta files under a licence the bank or AKR-TMS holds), never as the only source.
- **Use** an `ofac`-style scorer as a benchmark: an alert that the OFAC Sanctions List Search tool would produce should never be missed.

**AKR-TMS hooks**
- **Statement-level provenance in the watch-list store**: list, record id, field, value, first seen, last seen, version; supports diff,
  delisting and look-back.
- **Benchmark scorers** (`ofac`-style Jaro-Winkler and a plain exact matcher) run alongside the production matcher in test mode; alerts the
  benchmark raises and production does not are listed for review.
- **Pair-judgement log**: every analyst decision (match, no match, unsure) is stored with both records, so the bank accumulates its own
  labelled pairs for evaluation (never used to auto-lower thresholds without approval).
- **Open data adapters**: OpenSanctions bulk and delta files, the IQTFS API, UN XML, OFAC, EU and UK files, all converted into one internal
  schema with a per-source freshness clock.

---

## 10. Cross-cutting: payment screening on Swift MT and ISO 20022

### 10.1 Two dates that change what a bank screens (Swift pages, Internet Archive snapshots of 22 Nov 2025 and 7 Jun 2026)

- **22 November 2025: end of coexistence.** "From 22 November 2025, all FI-to-FI payment instruction messages will be delivered only in
  ISO 20022 format." Cross-border payment screening in October 2026 therefore means pacs.008, pacs.009 and related messages, with
  MT only as a translation or as legacy fields in the bank's own systems.
- **14 November 2026: unstructured addresses are removed.** "After November 2026, only fully structured or hybrid postal addresses will
  be accepted." From 14 November 2026 "town and country information must be provided in designated fields, at a minimum, for all agents
  and parties in CBPR+ payment messages" (a BIC alone remains valid for an agent). "Payments containing unstructured addresses will be
  NAK'ed at network level"; "it is not possible for Swift to develop a contingency solution". Swift's own measurement: "approximately
  65% of payment messages still contain unstructured addresses", and about 65 market infrastructures "do not yet have plans to align",
  with interoperability problems "disproportionately affecting smaller banks and emerging-region markets". Swift ties the change to
  "more effective compliance screening" and to FATF Recommendation 16 (payment transparency, file 03).
- **Swift's open-source address-structuring model** (free, runs offline, conditional random field; Python and PyTorch; planned November 2025)
  infers **town and country only** from MT 103 fields 50 and 59 free text. Its limits are in Swift's own words: "The model performs
  optimally with inputs in English or transliterated formats. Inputs in non-Latin scripts such as **Arabic**, Chinese, Cyrillic, or
  Japanese (CJK) may yield unreliable or unexpected results." Iraqi bank addresses (governorate, district, neighbourhood, landmark)
  stored in Arabic are therefore not helped by it.

### 10.2 What must be screened (elements, MT field and ISO 20022 element)

Mapping below is from the MT and ISO 20022 message definitions as I know them; I could not re-verify against swift.com (blocked), so
check against the current CBPR+ usage guidelines before building adapters. The list of attributes to screen is from the Wolfsberg
guidance (file 04 s.2d).

| Element | MT 103 / 202 | ISO 20022 (pacs.008 / pacs.009) | Screening note |
|---|---|---|---|
| Ordering customer | 50A, 50F, 50K | Debtor: name, postal address, identifier; ultimate debtor | MT: lines of 35 characters; MX: name up to 140 characters and structured address; the bank's CBS may hold a longer or shorter field |
| Beneficiary | 59, 59A, 59F | Creditor; ultimate creditor | Same; beneficiary name and account alignment checks are coming with FATF R.16 (2025) |
| Ordering and account-with institutions | 52A/D, 57A/D | Debtor agent, creditor agent: BIC, name, address, LEI | BIC chars 5-6 give the country; a BIC missing from the tool caused a violation (section 13) |
| Intermediary and chain banks | 56A/D, 53, 54 | Intermediary agents 1-3, instructing and instructed agents | Cover payments: MT 202 COV or pacs.009 COV carry the underlying customer data again |
| Free text | 70, 72, 77B | Remittance information (unstructured and structured), instructions to next agent, purpose, regulatory reporting | Vessel names, IMO numbers, goods, ports, "Krimea"-type spellings |
| Trade documents | MT 700 series (applicant 50, beneficiary 59, ports 44E/F, goods 45A, documents 46A) | trade messages | Wolfsberg: all parties, vessels, ports, goods |
| Identifiers | within party fields | LEI, other IDs, BIC | OFAC cases turned on BICs and ownership |

### 10.3 Hard problems, each with evidence

1. **Truncation and field limits.** FCA May 2026: "Long names exceeded system character limits and failed without generating an alert or
   required manual intervention." An engine must flag truncated or over-length fields as a data defect, not silently pass them.
2. **Free text is where the signal hides.** OFAC: Amazon did not flag "Yalta, Krimea"; Uphold did not screen the free-text address field;
   FCA 2026: vessel names missed because filters required prefixes or fixed formats; Wolfsberg: screen field 70, banks, BICs, vessels, ports.
3. **Case, punctuation and abbreviations.** OFAC: Apple's tool failed to match "SIS DOO" with "SIS d.o.o."; FSA 2010: "ABC GmbH" versus
   "ABC" missed 27 percent of matches. Normalisation must be tested, not assumed.
4. **Identifiers beyond names.** OFAC: Deutsche Bank Trust Company Americas had not loaded a designated bank's BIC and its tool alerted
   only on exact entity matches (61 payments). Engines in section 2 to 9 treat BIC, IMO, LEI and ISIN as first-class (LexisNexis ISO
   country in BIC, Napier identifier validation, OpenSanctions and Watchman identifier rules).
5. **Language and script.** FIN MT messages use the Swift X character set (Latin). I could not verify whether CBPR+ ISO 20022 messages
   permit Arabic script; market practice is a **Latin transliteration chosen by the sending bank**. So an Iraqi bank's outgoing
   payment carries a Latin form of an Arabic name that staff or a CBS routine produced, while the CBS holds the Arabic form and the
   Iraqi national list holds only Arabic. Pre-release screening should compare the message form, the CBS Arabic form and the list
   forms, and should flag payments whose Latin form drifts from the stored transliteration of the same customer (an alignment check
   in the spirit of FATF R.16 2025).
6. **Structured addresses.** After 14 November 2026 town and country must be present for every party. A CBS that stores addresses as
   one Arabic text line cannot produce them without a conversion step, and Swift's own model does not handle Arabic. This is a
   data-quality problem the AKR-TMS read-only replica can measure now (share of outgoing payments whose party has a usable town
   and country).
7. **Outage and degraded mode.** FCA May 2026 case: a retail bank's external screening system "became unavailable"; with no contingency
   "thousands of payments" queued without being screened. A pre-release call needs a defined fail-closed or fail-safe behaviour agreed
   with the bank, health checks and a manual fallback, plus after-the-fact re-screening of anything released in degraded mode.
8. **Release errors.** OFAC cases (Interactive Brokers 2025: an alert wrongly dispositioned as a false positive; State Street 2019: a
   unit used its own filter and escalated within the business line instead of to compliance). Releases need four eyes and sampling.

### 10.4 Latency and capacity references (vendor statements, not measurements)

| Source | Statement |
|---|---|
| FinScan (17 Feb 2026) | "beyond 100 million transactions daily"; "sub-second, with most under 250 milliseconds"; "66 milliseconds per transaction in single-threaded environments" |
| LexisNexis Firco Continuity | "millions of transactions in an hour" (about 280 or more a second) |
| Napier | 3,000 to 25,000 names per minute (about 400 a second) in one customer deployment |
| Dow Jones | "sub-second searches" |
| Eastnets, Swift | no figure published |

### 10.5 What it means for AKR-TMS

- The phase 2 API (pre-release call from the bank's payment or Swift gateway) should accept a **canonical payment object** (parties,
  agents, free text, identifiers, raw message, source format), screen every element, return a decision with reasons, and log the full
  request and result. Target p99 below 250 ms with the payment view of the watch list in memory, measured on bank hardware.
- Phase 1 (read-only replica) screens the same object after the event and also measures **data readiness** (unstructured addresses,
  missing town or country, truncated names, missing BICs, blank field 52 equivalents).

**AKR-TMS hooks**
- **Canonical payment model** with adapters for MT 103, 202, 202 COV, 700 series and ISO 20022 pacs.008, pacs.009 (core and cover), pacs.004
  and camt messages, plus CBS payment tables; original text and normalised form both kept.
- **Truncation, length and character-set checks** on every field before matching, raising a data defect instead of a silent pass.
- **ISO 20022 readiness report** for the November 2026 change: share of outgoing payments per branch with structured or hybrid
  address, town and country present, by customer segment; target list for the data-clean-up.
- **Message-versus-record alignment**: Latin name in the message against the transliterations of the customer's Arabic name held in the
  CBS; drift raises an alert.
- **Degraded-mode policy object** (hold, release with post-screen, or reject), health probe and an "unscreened release" register
  re-screened within a set time.

---

## 11. Cross-cutting: Arabic and Iraqi name matching in the engines

### 11.1 What each engine documents (as of 2026-10-06)

| Engine | Arabic and name-structure evidence | Gaps |
|---|---|---|
| LexisNexis Firco | Native-to-native for any script "without applying any fuzzy matching logic"; Arabic to Latin by transliteration with fuzzy matching; dictionaries for honorifics ("Dr, Shri, Sheik"), first-name equivalents, abbreviations; Saudi tuning role asks for Arabic transliteration and particles expertise | Exact native matching described; no chain, mother's-name or tribe logic documented; Bridger has no Arabic UI |
| Eastnets SafeWatch | Acknowledges transliteration problem; "contextual" AI extracts gender, name origin and entity type | No Arabic method or test published |
| FinScan | Best documented: prefixes Al, Ibn, Bin; transliteration variants; diacritics and hamza forms; tribal name (laqab); name-order reversal; field-level fuzziness; native-to-native plus transliteration | No Iraqi-specific features (mother's name, Kurdish), no published test results |
| Dow Jones RiskCenter | Original-script names exist in records; examples are Cyrillic and Chinese | Nothing on Arabic or Kurdish |
| Swift Transaction Screening | Hosted "logical tests (misspellings, abbreviations, phonetic similarities)" | Nothing on Arabic; opaque |
| Napier | Fuzzy matching in "more than 20 languages and scripts, including simplified Chinese and Arabic" | Language count only |
| OpenSanctions yente | States that cross-script fuzzy comparison, including Arabic/Farsi, "will produce increased error rates"; no Arabic transliteration in the Rust core | Weak Arabic by its own account |
| Moov Watchman | Optional cross-script embeddings (recall 0.69 to 0.82 on a mixed benchmark, not Arabic-specific) | Not Arabic-specific |

None of the eight engines above documents a **mother's-name** feature, a **chain-position-aware** comparison of Iraqi patronymic names, or **Kurdish letter
folding**; those are exactly the features of the Iraqi national list (file 01 s.6, s.7; file 22 s.3 and s.4).

### 11.2 Facts about the Iraqi list that shape matching (computed 2026-10-06 from the public API, aggregates only)

- 6,280 entries; 6,253 individuals and 27 entities; **no row marked as excluded**.
- **Mother's name on 6,230 rows (99.2 percent), birth year on 6,000 (95.5 percent), aliases on only 3.**
- Name length in whitespace tokens: mean 5.8, median 5, range 3 to 12; **3,305 names (53 percent) have exactly five tokens**, 1,252 have six,
  741 have seven and 565 have eight.
- Decisions by year: 2017: 206; 2018: 38; 2019: 486; 2020: 1,428; 2021: 1,498; 2022: 1,000; 2023: 1,011; 2024: 26; 2025: 526; 2026: 61 (66
  distinct decisions).

Meaning: the list gives almost no aliases, so spelling variation sits on the bank's side and in Latin messages; a long five-token chain
carries more evidence than a Western two-token name, so token-level scoring with position awareness works better than whole-string
similarity; mother's name and birth year can resolve most name-only alerts.

### 11.3 A vendor bake-off protocol (minimum test for any engine, including AKR-TMS's own)

Run on the bank's own anonymised customer extract and on a synthetic Iraqi corpus (file 22 s.12 describes how to build one). Report alerts
per 1,000 records and detection rate for each test, per list (national, UN, OFAC, EU, UK).

1. **Exact native**: Arabic customer name equals list name (control).
2. **Orthographic variants**: hamza forms (أ إ ا), ya and alef maqsura (ي ى), ta marbuta and ha (ة ه), diacritics, tatweel, Arabic-Indic digits.
3. **Compound and particle forms**: عبد الله and عبدالله; أبو X; بن and ابن; "al" attached or detached.
4. **Chain variants**: dropped grandfather or tribe, reordered tokens, two-token short forms, mother's name added.
5. **Kurdish and Persian letters** (ک/ك, ی/ي, ە, ۆ, ێ) against Arabic equivalents.
6. **Cross-script**: Latin transliteration variants (Mohammed, Muhammad, Mohamed; Hussein, Husain, Hussain; Abd Al-Rahman, Abdulrahman) against Arabic list entries, and the reverse (Latin list entry against Arabic customer).
7. **Honorifics and titles** (الحاج، الشيخ، السيد، الدكتور) attached to names (FCA found honorifics lowered scores below thresholds).
8. **Very common names** (a chain of three very common tokens) to measure the false-positive burden (file 22 s.8 measures this on the UN list).
9. **Secondary identifier tests**: same name with different birth year, with and without mother's name; discount rules must not discount a true match.
10. **Length and structure**: names over 35 and over 70 characters, one-word names, names with digits (FCA found defaults that excluded them).

Pass criteria are set by the bank's MLRO and approved by the board, and the same suite is re-run after every list or configuration change
(Swift Sanctions Testing can independently test any filter, section 6).

**AKR-TMS hooks**
- **Iraqi name pipeline** as the default matcher: normalisation (letter folding with a reversible log), tokenisation into chain positions,
  particle handling, tribe and mother's-name features, token weights from the list and the bank's own frequency table, transliteration to
  several Latin variants in both directions (file 22).
- **List enrichment**: generate Latin variants for Arabic-only list entries (and Arabic forms for Latin entries where original script exists),
  stored as derived aliases with their method, so that payment messages in Latin can match the Arabic-only national list.
- **Bake-off suite** shipped with the product and re-run on every release; results stored per version.

---

## 12. Cross-cutting: list management, good-guy lists, testing and tuning, false-positive rates, latency and deployment

### 12.1 List management

| Topic | Evidence |
|---|---|
| Refresh frequency offered | Dow Jones LPTS feed: "up to six times a day"; OpenSanctions: four times a day (on-premise poll hourly); Swift: lists "collected directly from regulators" (no frequency stated in the snapshots); Eastnets RTLL and ChainFeed "instant, secure watchlist refreshes" (vendor wording); LexisNexis List Manager: "monitored for updates and validated each day". CBI floor: at least every 12 hours (file 01 s.2d) |
| What actually happens in banks | FCA 2026: about two-thirds implement list updates within one day of notification; "delays in updates to post-designation lists led to alert backlogs"; SST found "errors or omissions in sanctions lists provided by third-party vendors" |
| Lag and contract scope | OFAC MidFirst (2022): the vendor screened the entire existing customer base only monthly; the bank learned of two designations 14 days after; OFAC SkyGeek (2024): software "did not yet reflect that day's designation" |
| Regulator-grade expectation | Deutsche Bundesbank tender (OJ S 157/2026, 17 Aug 2026): extended sanctions lists "no later than 48 hours after their entry enters into force", machine-readable (CSV, XML or JSON), with "the highest possible coverage of implicitly sanctioned entities" |
| Ownership-derived entities | Dow Jones SCO (54,000 or 56,000 entities); Swift Transaction Screening includes SCO via Dow Jones; OFAC Microsoft (2023): screening "did not identify blocked parties ... owned 50 percent or more by SDNs" |
| Delta (re-)screening | Firco Compliance Link "automatically rescreen ... whenever the watchlist, client or trade data is updated"; Swift Name Screening "delta-delta"; OpenSanctions delta files; Wolfsberg PEP guidance: both directions |
| Private lists | Supported by all (Firco, Eastnets, Swift, Napier, Watchman via CSV); governance of inclusion and expiry is the bank's job (Wolfsberg 2019, file 04) |
| Versioning and comparison | Eastnets "version control, offline testing environments, and list comparison"; Swift Sanctions Testing list alerts and change reports; Firco archive for look-backs |

### 12.2 Good-guy lists (suppression of known false positives)

| Engine | Mechanism |
|---|---|
| Firco | Decision reapplication ("automatically resolve recurring false positive hits across transactions based on historical qualification made by operators"); Good Guy lists are part of the Fircosoft heritage |
| Eastnets | "Good Guys (White Lists)", Decision Replay, List Set Rank |
| Swift | "exceptions or suppression rules to manage and reduce common false positive alerts" |
| Napier | Whitelist with "time-boxed rules and a complete audit log"; repeat-hit and materiality logic ("suppresses hits that have already been cleared and haven't materially changed") |
| FinScan, Dow Jones | Not documented on the pages read |
| OpenSanctions | None: left to the application |

The risk is documented: Habib Bank New York (NYDFS 2017; file 20 s.3.4) used a "Good Guy" list that let "at least USD 250 million in transactions" pass "without any screening". Design rule from file 20: a good-guy entry is **pair-keyed** (this customer, this list entry, these fields), maker-checker approved, time-limited, **automatically reopened whenever either record or the list version changes**, and reported (count, age, approver, value passed, entries now matching a current designation, which must be zero). Napier's time-boxed rules and Firco's reapplication logic are the closest vendor implementations.

### 12.3 Testing and tuning

| Evidence | Point |
|---|---|
| FCA 28 May 2026 | 90 percent of alerts identified the sanctioned party for exact names, **75 percent for slightly different forms**; poor practice: "Relying on historic vendor settings without reassessment"; good practice: periodic calibration and QA testing, "engaging with vendors to retest systems following list updates or changes to matching logic", root-cause analysis after mismatches |
| FSA decision notice 2 Aug 2010 (RBS) | Fuzzy matching "calibrated only once" in 2006; first test in 2008 found failures (section 13) |
| FCA Starling notice 2024 | No testing or calibration after implementation; third-party testing in November 2023 and March 2024 confirmed the fix |
| Swift Sanctions Testing | Independent web tool for any filter, public data only, MT/MX/CHIPS/Fedwire, peer assessment |
| Napier sandbox, Eastnets offline test environment, Firco "Test and tuning" stage | Vendor-side test-before-release workflows |
| Open evaluation material | OpenSanctions pair judgements (760,000, Dec 2025); Watchman benchmark (472,477 labelled records: Jaro-Winkler at 0.80 precision 0.99; cross-script embeddings recall 0.69 to 0.82) |
| Practice vocabulary | The Fircosoft tuning job notice asks for "above-the-line/below-the-line style effectiveness testing": above the line tests that seeded variants alert; below the line samples near-threshold and sub-threshold results for missed true matches |

### 12.4 False-positive evidence (what is real and what is marketing)

| Figure | Source | Reliability |
|---|---|---|
| 95 percent of firms with automated screening reported no true client matches since 2022; 98 percent no true payment matches | FCA 2026 (REP-CRIM 2024-25) | Regulator data: almost every alert in most firms is a false positive |
| About 44 percent of firms close name-screening alerts within one working day; over a quarter take 3 to 5 days | FCA 2026 | Regulator data |
| 1.8 million customers screened: 0.95 percent "review" hits and 0.02 percent "highly likely"; 70 million PEP records: 0.07 percent review, 0.006 percent highly likely; APAC transaction screening hit rate 3.5 to 12 percent, name screening 2 percent | LexisNexis deck, Oct 2025 | Vendor deck, real customer numbers |
| 3,988,143 payments produced 795,712 alerts (20 percent) when Starling reviewed history with proper screening | FCA Starling notice | Regulator document; a one-off look-back at remediation settings, so not a steady-state rate |
| 26,000 alerts from 12 million credit-card customers | FSA 2010 | Regulator document, old engine |
| "Legacy systems can reach upward of 95 percent" false positives | Eastnets blog | Vendor |
| 93 percent (ERF), 90 percent (Banco do Brasil), 95 percent (FinScan Payments), 80 percent (FinScan generic), 60 to 90 percent (Eastnets cases), "500 percent" (Eastnets Al Jazeera) | Vendors | Not comparable; no common test; one is arithmetically impossible |

Regulators do not state an acceptable false-positive rate; they ask whether the bank can show that thresholds were set from tests and still detect (FCA, NYDFS 504, FFIEC; file 05). A bank should report **alerts per 1,000 records and detection rate on a standing test set**, not a percentage reduction.

### 12.5 Latency, capacity and deployment

| Engine | Deployment | Capacity or latency statement |
|---|---|---|
| LexisNexis Firco Continuity | Cloud or on premise; Bridger: single-tenant cloud, multi-tenant cloud, on-premise; Compliance Link hosted | "millions of transactions in an hour" |
| Eastnets SafeWatch Screening 5 | REST API, Kafka and MQ, microservices; hosted option (Azure in the Americas); Swift service bureau | Not published |
| FinScan | SaaS, on-premise (Windows listed), hybrid | "most under 250 milliseconds" |
| Dow Jones | Hosted tools and APIs; SFTP or Pub/Sub feeds for own engines | "sub-second searches" |
| Swift Transaction Screening | Hosted only, in the Swift network | Not published |
| Napier | SaaS, any private cloud, on-premise | 25,000 names per minute (one customer) |
| Silent Eight | On-premises, private or public cloud, hybrid or managed (file 13) | Not applicable (adjudication) |
| OpenSanctions yente | Docker (app plus Elasticsearch); 8 GB RAM, 1 vCPU, 60 GB disk minimum | "No query volume limitations" on own hardware |

**AKR-TMS hooks**
- **Screening configuration as a versioned object** with a test report attached (detection on the standing suite, alerts per 1,000 records); a change without a fresh report cannot be approved.
- **Good-guy register** with pair keys, expiry, automatic reopening, and a monthly board-ready report including "entries that now match a current designation".
- **List freshness monitor**: per source, time of publication, time loaded, time active in the matcher, and the lag; alarms at the CBI 12-hour limit (and an internal target of one hour for UN and the Iraqi list).
- **Standing test set and scheduler** (section 11.3), run nightly on the live configuration and after each change, with results trended.
- **Capacity plan**: published latency and throughput of the AKR-TMS matcher on reference hardware, so banks can size without guesswork.

---

## 13. Enforcement cases and regulator findings that turned on screening failures

File 20 treats the large cases (Standard Chartered, BNP Paribas, Commerzbank, Habib, Mashreq, National Bank of Pakistan, Swedbank Latvia, TD
Bank) in depth. This section records the cases and findings read from primary documents for this file, grouped by failure mode, and points
to file 20 where it adds more. Evidence grade A (primary document read) unless stated.

### 13.1 Regulator reviews and UK cases

| Case | What happened | Failure mode | Primary source |
|---|---|---|---|
| **RBS Group**, FSA Decision Notice, 2 Aug 2010, GBP 5.6m (8m before a 30 percent settlement discount) | Between Dec 2007 and Dec 2008, automated screening "failed to screen the majority of trade finance SWIFT messages"; directors and beneficial owners not consistently recorded or screened; the fuzzy matching was "calibrated only once, when the software was initially installed in 2006". The first test (2008) showed: "SMITH, John" rearranged to "J SMITH" triggered alerts for **less than 5 percent** of list terms; a one-character error in a name of six or fewer characters (SMITHE for SMYTHE) produced no alert; removing the company abbreviation ("ABC GmbH" to "ABC") missed **27 percent**; a 12-million-customer credit card database linked in 2008 produced 26,000 alerts, and the backlog was cleared only in August 2009 | One-time calibration, no review, scope gaps (trade messages, owners), alert backlog, software provider issues not resolved in time | fca.org.uk final notice rbs_group |
| **Starling Bank**, FCA Final Notice, 27 Sep 2024, GBP 28,959,426 | Since 20 July 2017 customer screening matched only the 39 of 3,088 Designated Persons with UK citizenship or residence (misconfiguration); no alerts for individuals from 1 Jul 2022 to 30 Jan 2023; at least one Designated Person opened an account; customers screened once every 14 days; not all cross-border payments screened; a customer-screening tool used for payments; no calibration or testing after implementation; second-line assurance only from Q1 2023; a historic review of 3,988,143 payments produced 795,712 alerts | Configuration error that silently narrowed list scope for six years; frequency; wrong tool; no testing | fca.org.uk final notice starling-bank-limited-2024 |
| **FCA thematic review**, 28 May 2026 (150+ firms) | 90 percent exact-name and 75 percent variant-name detection in tests; honorifics lowered scores below thresholds; one-word names and names with digits excluded by default; long names failed without alerts; reliance on vendor defaults; vendor list errors; stale vessel logic; analysts bypassing escalation under targets; unavailable screening system with no contingency; feed failure left transactions unscreened for an extended period | Matching logic, defaults, data feeds, resilience, alert handling | fca.org.uk good-and-poor-practice |

### 13.2 OFAC enforcement releases, 2019 to 2025 (120 releases scanned; those below name a screening-system cause)

| Date and party | Penalty | Screening failure in OFAC's words | Mode |
|---|---|---|---|
| 28 May 2019, State Street Bank (Finding of Violation) | none | A retiree-services unit "utilized their own sanctions screening filter instead of SSBT's centralized sanctions screening system"; the screening software alerted on each of 45 payments to Tehran but staff escalated within the business line | Parallel filter, wrong escalation |
| 25 Nov 2019, Apple | USD 466,912 | The tool failed "to match the upper case name 'SIS DOO' ... with the lower case name 'SIS d.o.o.'" | Normalisation (case, punctuation) |
| 8 Jul 2020, Amazon.com | USD 134,523 | Screening "failed to fully analyze all transaction and customer data"; did not flag "Yalta, Krimea"; "in several hundred instances ... failed to flag the correctly spelled names and addresses of persons on OFAC's SDN List" | Fields not screened, variants, exact names missed |
| 9 Sep 2020, Deutsche Bank Trust Company Americas | USD 583,100 (part for 61 payments, USD 425,600) | A designated bank's "SWIFT Business Identifier Code (BIC)" was not in the tool and the tool "was calibrated so that only an exact match to a designated entity would trigger further manual review" | BIC missing, exact-only calibration |
| 29 Apr 2021, MoneyGram | USD 34,328.78 | "screening, technology, and fuzzy logic failures"; earlier did not screen a service population at all | Fuzzy logic, population not screened |
| 23 Dec 2021, TD Bank N.A. | USD 115,005.04 | Profiles missing or misstating citizenship ("Korea" or a South Korea code), "the screening system did not flag" | Reference data quality |
| 21 Jul 2022, MidFirst Bank (Finding of Violation) | none | Vendor screened the full existing base "only once a month"; bank "misunderstood the scope of the contract"; vendor informed the bank of designations after 14 days; 98 percent of post-designation value moved within six hours | Vendor contract scope, list and rescreen lag |
| 31 Mar 2023, Uphold HQ | USD 72,230.32 | The free-text address field and identity documents "not screened" | Fields not screened |
| 6 Apr 2023, Microsoft | USD 2,980,265.86 | Screening "did not identify blocked parties ... owned 50 percent or more by SDNs, or SDNs' Cyrillic or Chinese names, even though many customers ... supplied order and customer information in their native scripts", "missing common variations of the restricted party names" | **Ownership, native script**, variants |
| 14 Nov 2024, American Life Insurance Co. | USD 178,421 | A Dubai trade licence "read in Arabic that the owner of the Entity was the Iranian embassy"; alerts cleared as false positives because the entity was not itself on the SDN List | **Arabic evidence, ownership**, clearing logic |
| 31 Dec 2024, SkyGeek Logistics | USD 22,172 | Software "did not yet reflect that day's designation"; re-screening before refunds also missed a designation | List lag |
| 15 Jul 2025, Interactive Brokers | USD 11,832,136 | An alert on a blocked person was "incorrectly dispositioned" as a false positive; IP geo-blocking bug | Alert disposition, geo-blocking |

Cases in file 20 add the Middle East dimension: Habib Bank (suppression list and 13,000 incomplete payments passed), Mashreq (sensitivity settings for USD screening, a vendor validation that only summarised rules, alerts waived without rationale), National Bank of Pakistan (no screening of incoming payments), BNP Paribas and Standard Chartered (cover payments and stripping).

### 13.3 What the cases say, by failure mode (and what each engine class must therefore prove)

| Failure mode | Cases | What a buyer must test or require |
|---|---|---|
| Matching logic defaults or one-time calibration | RBS 2010; DBTCA 2020 (exact-only); FCA 2026 | Standing test suite; re-run after each change (section 11.3, 12.3) |
| Silent narrowing of scope by configuration | Starling 2017-2023 | Config diff against the full list on every release; coverage reconciliation (every list entry screened) |
| Names in non-Latin script and variants | Microsoft 2023; FCA 2026; ALICO 2024 (Arabic document) | Native script tests; evidence documents read in Arabic |
| Ownership and control | Microsoft 2023; ALICO 2024; FCA 2026 (ownership gaps) | Derived-designation data; ownership fields collected in KYC |
| Fields and identifiers not screened | Amazon; Uphold; DBTCA (BIC); RBS (trade messages); FCA (vessels) | Field-level scope map; identifiers as first-class |
| Reference data quality | TD Bank (citizenship); FCA (placeholder birth dates) | Data-quality score and queues |
| Lists late or wrong | MidFirst; SkyGeek; FCA (vendor list errors) | Freshness clock per list; contract with frequency and escalation |
| Vendor contract or oversight gaps | MidFirst; FCA ("limited understanding of how vendor screening logic or configurations operate"); Mashreq vendor validation | Contract schedule of screening scope; independent testing |
| Alert disposition and parallel filters | Interactive Brokers; State Street; FCA analysts under targets | Four eyes, reason codes, sample re-review, no local filters |
| Resilience | FCA 2026 outage and feed-failure cases | Degraded-mode policy, heartbeat, unscreened-release register |

**AKR-TMS hooks** (collected into requirements in section 16)
- **Scope reconciliation**: for each configuration and list version, the system proves how many list entries and which fields were loaded into the matcher (the Starling gap), and compares with the source count; any shortfall is a blocking error.
- **Release controls**: screening configuration releases require a test report and a second approver; the live configuration hash is stored with each screening run.
- **Evidence documents in Arabic**: the case screen lets the analyst attach and annotate Arabic ownership or licence documents and records an "ownership and control" finding that feeds the derived-designation table.

---

## 14. Comparison table

**Table A: what each is, where it runs, what it screens, what it costs (public signals)**

| Vendor and product | Type | Deployment | Payment formats | Price signal (public) |
|---|---|---|---|---|
| LexisNexis Firco Continuity; Compliance Link; Insight (Trust); Bridger Insight XG | Payment and customer screening suite with its own data (Global WatchList, WorldCompliance) | Continuity: cloud or on premise; Bridger: single or multi-tenant cloud or on premise; Compliance Link: hosted | MT, ISO 20022 (Targeted Matching), Fedwire, ACH, SEPA | EC DG NEAR EUR 300,000 (2024); FSCS GBP 307,000 over 3 years (2019); Met Office GBP 8,500 a year (2026); bank-scale: quote |
| Eastnets SafeWatch Screening 5 (plus PaymentSafe, Swift hosting) | Screening suite and Swift service bureau | REST, Kafka, MQ; hosted option (Azure in the Americas); on-premise not stated | SWIFT, SEPA, XML, Fedwire; ISO 20022 | None public; "8 weeks or less" to deploy |
| FinScan (Innovative Systems) | Matching-first engine with data-quality step | SaaS, on-premise (Windows), hybrid | SWIFT, SEPA, FedNow, IACH, Fedwire; ISO 20022 | Quote; modelled third-party estimate only (unverified) |
| Dow Jones RiskCenter and risk data | Hosted screening tools plus data feeds and APIs | Hosted; SFTP, Pub/Sub feeds, REST for own engines | LPTS payment feed for engines | BaFin: 25 licences EUR 277,380 (2024) |
| Swift Transaction Screening, Name Screening, Sanctions Testing, Payment Controls | Managed network services | Swift-hosted only | MT, ISO 20022, proprietary via Connector | Usage-based; none public |
| Napier AI Client and Transaction Screening | Screening with AI layers and sandbox | SaaS, any private cloud, on-premise | MT, ISO 20022, SEPA Instant, Fedwire, ACH | None public |
| Silent Eight (file 13) | AI adjudication on top of any engine | On-premises, cloud, hybrid, managed | n/a | None public |
| OpenSanctions yente and data | Open data and open matcher (MIT software; CC BY-NC data) | Docker on-premise or hosted API | Payment schema, identifier rules | API EUR 0.10 to 0.03 per query; flat-rate licence on quote |

**Table B: evidence on Arabic and Iraq, strengths, caveats and the role for AKR-TMS**

| Vendor | Arabic evidence | Iraq and MENA evidence | Strongest point | Main caveat | Role for AKR-TMS |
|---|---|---|---|---|---|
| LexisNexis | Native-to-native "without ... fuzzy matching" (deck); transliteration with fuzzy; Saudi tuning role | None found for Iraq; Gulf via tuning role; no Arabic UI in Bridger | Explainable Strength Index; decision reapplication; audit of every message | Exact Arabic matching; heavy configuration; unverifiable claims | Learn (scoring features, no-hit trail); integrate data if bank licenses |
| Eastnets | Problem statement only | Dubai HQ; Qatar, Egypt, UAE, Jordan, KSA references; "QI Card" listed; World-Check On Demand (Jan 2026) | Swift-certified, hosted Swift, list tooling, MENA presence | Arabic undocumented; impossible FP percentages | Interoperate (import alerts); learn (file scanner plus API, list set rank) |
| FinScan | Best documented (prefixes, hamza, laqab, order) | Gulf banks and exchange houses; no Iraq | Field-level scoring; data-quality step; latency figures | No Iraqi features; US-centric rails update; weak reporting reviews | Learn (field-level fuzziness, 250 ms benchmark); benchmark |
| Dow Jones | Original script field; examples not Arabic | None stated | Ownership-derived data (SCO), payments feed, Factiva | Hosted only; inconsistent figures; D&B ownership partner | Data source for ownership and adverse media (file 15) |
| Swift | None; opaque | Yemeni Islamic bank case (Tadhamon) | In-network control; independent testing; look-back | Only Swift flows; no Arabic evidence; customer data to Swift for Name Screening | Complement; use Sanctions Testing as independent test; import results |
| Napier | Language count only | Dubai office; no MENA customer found | Sandbox with diff; entity extraction; multi-configuration | Self-tuning and auto-clearing governance risk | Learn (sandbox, extraction); avoid auto-tuning without approval |
| Silent Eight | See file 13 | Gulf banks (file 13) | Adjudication layer with policy bounds | "Executes decisions"; opaque reasoning | Learn (policy-bound assistant); do not copy autonomy |
| OpenSanctions | States weak Arabic; no Arabic transliteration in Rust core | Iraqi freeze-list source (5,678 vs 6,280); 724 PEP tags | Open, explained matching; data provenance; MIT Rust names core | Not a compliance app; Arabic weak; licence required | Optional data feed; reuse rigour-core ideas; benchmark scorer |

---

## 15. What it means for AKR-TMS

1. **No engine publicly solves Arabic or Iraqi name matching.** The best documentation is FinScan's (Al/Ibn/Bin, hamza forms, laqab, order). LexisNexis describes native Arabic comparison as exact; Eastnets, Swift and Dow Jones are silent; OpenSanctions says Arabic cross-script matching has "increased error rates". None documents mother's name, chain-position alignment or Kurdish folding, which are the identifiers of the Iraqi national list. An Iraqi-name-first matcher with published tests is the clearest differentiator, but only if the tests are public and re-run.
2. **The failures that regulators punished were configuration, testing, data and operations, not algorithm brand.** RBS (calibrated once, 2010), Starling (a configuration error that narrowed the list to 39 of 3,088 names for six years, 2024), the FCA review (90 percent exact and 75 percent variant detection, honorifics, defaults, vendor list errors, 2026) and twelve OFAC cases (missing BIC, case normalisation, free text not screened, vendor scope, ownership, native script) all say the same. So AKR-TMS earns its place through governance tooling: scope reconciliation, versioned configuration, a standing test suite, a change sandbox, list-freshness clocks and governed good-guy lists, built into the product.
3. **Phase 1 cannot interdict, and the market says so.** Napier's own text calls post-settlement screening "checking a payment that has already gone". Phase 1 should be described honestly as detection, freeze tasks and evidence; interdiction needs the phase 2 API (with a stated latency target, a degraded-mode policy and a heartbeat) or the bank's in-network Swift control. Swift's Transaction Screening and Payment Controls are complements, not competitors, and a bank that has them should be able to import their results.
4. **Data is separate from the engine, and the CBI has named one supplier.** Eastnets (2 Jan 2026) and FinScan (22 Jul 2026) both integrated LSEG World-Check On Demand. AKR-TMS must therefore offer a World-Check connector (file 15), stay data-agnostic (official lists, the IQTFS API, Dow Jones, LexisNexis, OpenSanctions as options), and never hard-wire one supplier. The Iraqi national list should be read from the official API and checked against any third-party copy (the OpenSanctions mirror shows 5,678 entities against 6,280 rows).
5. **Build the matcher, integrate the data, interoperate with installed engines.** Build the Iraqi name pipeline, the scorer and the governance layer in Rust (reusing the MIT `rigour-core` ideas for Latin names). Integrate paid data by feed or API. Where a bank already runs Fircosoft, Eastnets or Swift screening, AKR-TMS should import alerts and decisions, add Arabic and Iraqi coverage, case management and testing, and not demand a rip-and-replace.
6. **Ownership is the next gap.** OFAC's 50 percent rule and the Microsoft and American Life cases show that name screening misses controlled entities and evidence written in Arabic. The product needs a derived-designation table fed by the bank's own KYC ownership data (and licensed data if the bank has it) and an evidence vault that accepts Arabic documents.
7. **AI must arrive as proposals, not actions.** Auto-tuning from analyst decisions and auto-clearing agents conflict with the FCA's observation that analysts under pressure bypass controls, and with the CBI rule that the MLRO verifies. Ship AI as ranked suggestions with reasons, sampled QA, expiry dates and maker-checker.
8. **Readiness is a sales door.** Swift removes unstructured addresses on 14 November 2026, five weeks from the date of this file, and about 65 percent of messages were still unstructured. A read-only report of the bank's payment address quality is something AKR-TMS phase 1 can deliver now.
9. **Price signals** from public procurement are low six figures a year for modest-use buyers (EUR 300,000 for the European Commission's WorldCompliance screening; GBP 307,000 over three years for an FSCS batch service; EUR 277,380 for 25 Dow Jones seats at BaFin; GBP 8,500 a year for a small Bridger user). Enterprise bank deals are higher and not public. Iraqi banks buy to comply and fund little tuning (file 18), so total cost of ownership, not licence price, is the competitive lever.

---

## 16. Derived requirements (VC-xx, numbered across files 14, 15 and 16)

| ID | Requirement | Source |
|---|---|---|
| VC-01 | Unified watch-list store with statement-level provenance: list, record id, field, value, first seen, last seen, list version; supports diff, delisting and look-back | OpenSanctions statements model (s.9); Wolfsberg 2019 (file 04 s.2d); Firco archive (s.2) |
| VC-02 | Read the Iraqi national list from the IQTFS API at least hourly, diff every load, treat a delisting as a controlled event, and reconcile counts with any third-party copy (6,280 rows versus 5,678 entities) | File 01 s.6; s.9.3; IQTFS API aggregate count 2026-10-06 |
| VC-03 | List-freshness clock per source (published, loaded, active in matcher, lag) with alarm at the CBI 12-hour limit and an internal target of one hour for UN and Iraqi lists | CBI controls 2025 (file 01 s.2d); FCA 2026; OFAC MidFirst 2022 and SkyGeek 2024; Bundesbank 48-hour specification (s.12.1) |
| VC-04 | Every list version loads to staging, is compared with the previous version and passes a regression test set before it becomes active; the comparison is stored | Eastnets list comparison (s.3); Swift list alerts (s.6); FCA 2026 |
| VC-05 | Scope reconciliation for every screening run: number of list entries, aliases and fields loaded equals the source; any shortfall blocks activation and raises an incident | FCA Starling notice (s.13); FCA 2026 ("limited understanding of how lists were ingested") |
| VC-06 | Delta re-screening of the whole customer base and recent counterparties on each list change, and of changed customers against all lists, in both directions | Wolfsberg PEP guidance (file 04 s.2n); Firco Compliance Link; Swift delta-delta; CBI |
| VC-07 | Two-stage matcher: recall-oriented candidate retrieval, then explainable per-field scoring with a stored result for every pair (including no alert) | Firco ERF (s.2); FinScan (s.4); OpenSanctions logic-v2 (s.9) |
| VC-08 | Iraqi name pipeline as the default matcher: letter folding with a reversible log, chain positions, particles (Al, Ibn, Bin, Abd), tribe and laqab, mother's name, Kurdish letter folding, token weights from list and bank frequency, bidirectional transliteration to several Latin variants | File 22; FinScan Arabic handling (s.4); engine gap (s.11.1) |
| VC-09 | Derived aliases: generate and store Latin variants for Arabic-only list entries and Arabic forms for Latin entries with original script, labelled with method and version | IQTFS list is Arabic only with 3 aliases (s.11.2); Wolfsberg data enhancement |
| VC-10 | Native Arabic-to-Arabic comparison must be fuzzy (normalisation, edit tolerance, token alignment), not exact | LexisNexis deck "without applying any fuzzy matching logic" (s.2.3) |
| VC-11 | Secondary identifier rules (birth year with tolerance, mother's name, nationality, ID number): discount a name hit only on a reliable conflict, never on missing data; each rule versioned | LexisNexis Comparison Mode (s.2); Wolfsberg negative news and PEP (file 04); Napier (s.7) |
| VC-12 | Name-frequency feature computed from the bank's customer base and from the lists, so a very common chain needs more supporting evidence | Firco ERF NFQ (s.2); File 22 s.8; Splink (s.9.6) |
| VC-13 | Identifier-first rules: a matching BIC, IMO, LEI, ISIN, IBAN, passport or national ID number confirms a match whatever the name; a conflicting ID lowers it | OpenSanctions logic-v2; Watchman; OFAC DBTCA 2020 (s.13) |
| VC-14 | Field-scope map for every source (name, bank, BIC, address, free text, vessel, goods, port); no field silently skipped; free text, vessels, ports and addresses screened | Wolfsberg 2019 (file 04 s.2d); OFAC Amazon 2020, Uphold 2023; FCA 2026 |
| VC-15 | Length, truncation and character-set checks before matching; over-length or truncated fields and missing screening-critical fields are data defects that hold or flag the record | FCA 2026 (long names failed without alerts); Habib lesson (file 20 s.3.4) |
| VC-16 | Canonical payment model with adapters for MT 103, 202, 202 COV, 700 series and ISO 20022 pacs.008, pacs.009 (core and cover), pacs.004 and camt, plus CBS payment tables; raw and normalised text both kept | Swift ISO 20022 pages (s.10); Firco Targeted Matching (s.2) |
| VC-17 | Phase 2 screening API with a target of p99 under 250 ms per payment, payment view of the watch list in memory, decision with reasons, full request and result logging; publish measured capacity on reference hardware | FinScan latency (s.4, s.10.4); Napier (s.7); FCA 2026 |
| VC-18 | Degraded-mode policy object (hold, release with post-screen, reject), health probe, and an unscreened-release register re-screened within a set time | FCA 2026 outage and feed-failure cases (s.10.3) |
| VC-19 | ISO 20022 readiness report: per branch and segment, share of outgoing payments with structured or hybrid address, town and country present, truncated names, missing BICs | Swift unstructured address removal, 14 Nov 2026 (s.10.1) |
| VC-20 | Message-versus-record alignment: compare the Latin name in a payment with the transliterations of the customer's Arabic name in the CBS; drift raises an alert | FATF R.16 2025 (file 03); s.10.3 point 5 |
| VC-21 | Derived-designation table: owned or controlled entities with owner chain, percentages, aggregation rule per regime (OFAC 50 percent aggregate; EU and UK more than 50 percent or control), source and date; used as extra watch-list records | OFAC 50 percent rule (file 05 s.1e); Dow Jones SCO (s.5); OFAC Microsoft 2023, ALICO 2024 (s.13) |
| VC-22 | Address screening against list addresses with an Iraqi address normaliser (governorate, district, neighbourhood, street, building, landmark; Arabic and Latin) | Dow Jones address screening (s.5) |
| VC-23 | Good-guy register: pair-keyed, time-limited, maker-checker, automatically reopened when either record or the list version changes, with a monthly report including entries that now match a current designation | Habib (file 20 s.3.4); Napier time-boxed rules (s.7); Firco reapplication (s.2) |
| VC-24 | Auto-discount register (rule, reason, owner, expiry, QA sample result); no autonomous closure of sanctions alerts; learning produces proposals that need approval | Napier governance risk (s.7.7); FCA 2026 analysts under targets; CBI controls (file 01 s.2d) |
| VC-25 | Change sandbox: every proposed change to a threshold, rule, list rule or good-guy entry is replayed on stored history from the read-only replica, showing added and removed alerts and missed seeded positives, before approval | Napier sandbox (s.7); NYDFS 504 (file 05 s.1f) |
| VC-26 | Standing test suite (the ten bake-off tests of s.11.3) run nightly on the live configuration and after every change, reporting alerts per 1,000 records and detection rate per list, trended over time | FCA 2026 (90/75 percent); RBS 2010 (s.13); Swift Sanctions Testing (s.6) |
| VC-27 | Export test sets in formats an independent tester can use (MT, MX, customer files) and accept external testing of the AKR-TMS filter, including by Swift Sanctions Testing | Swift Sanctions Testing (s.6) |
| VC-28 | Benchmark scorers (an OFAC-style Jaro-Winkler scorer and an exact matcher) run beside the production matcher in test mode; alerts raised by the benchmark but not by production are listed for review | OpenSanctions `ofac` algorithm (s.9.2) |
| VC-29 | Screening configuration is a versioned object with rationale, approver and date; its hash is stored with every run and shown on every alert | Wolfsberg 2019 (file 04 s.2d); NYDFS 504 (file 05 s.1f); FCA 2026 |
| VC-30 | Alert release needs four eyes and a reason code; no local or parallel filters; monthly sample re-review of releases | OFAC State Street 2019, Interactive Brokers 2025; Mashreq (file 20 s.3.5) |
| VC-31 | Pair-judgement log: every analyst decision stored with both records, building bank-specific labelled pairs for evaluation (never used to lower thresholds without approval) | OpenSanctions pairs dataset (s.9.2) |
| VC-32 | Data-quality score and work queues per customer (missing birth year, placeholder dates, missing mother's name, wrong or blank nationality, one-word names) | OFAC TD Bank 2021; FCA 2026; FinScan MENA data note (s.4.3) |
| VC-33 | Retain every screened record and payment with result, including no-hit, with list, algorithm and configuration versions; retention per record class (five years CBI minimum; ten years option for sanctions records) | Firco no-hit trail (s.2); Swift Transaction Explorer (s.6); file 05 s.1e; file 01 |
| VC-34 | Import results from the bank's Swift screening (Transaction Screening, Payment Controls) and reconcile counts of messages sent, screened and released with AKR-TMS records | Swift services (s.6) |
| VC-35 | Pluggable data-source adapters with per-source licence flags: World-Check (named by the CBI), Dow Jones feeds and API, LexisNexis feeds, OpenSanctions bulk and delta files, official UN, OFAC, EU, UK and IQTFS sources; no hard dependency on any one | CBI controls (file 01 s.2d); Eastnets and FinScan World-Check On Demand (s.3, s.4); OpenSanctions (s.9) |
| VC-36 | Open-source hygiene: prefer MIT and Apache components (rigour-core, Watchman ideas), avoid AGPL components in the product, and obtain legal advice on whether an OpenSanctions OEM licence is needed | OpenSanctions commercial FAQ and GitHub licences (s.9) |
| VC-37 | Alert screen shows the customer record and the list record side by side with distinguishing attributes highlighted (birth year, mother's name, nationality, ID), one-click rationale picker; Arabic-first right-to-left | Wolfsberg 2019 (file 04 s.2d); Bridger without Arabic UI (s.2) |
| VC-38 | Alert-handling MI: alerts by list, branch, product and channel; ageing and closure-time distribution; spike detection after list loads | FCA 2026 (alert timeliness); Wolfsberg metrics |
| VC-39 | Evidence vault accepts and indexes Arabic ownership and licence documents with analyst annotations that feed an ownership-and-control finding | OFAC ALICO 2024 (s.13) |
| VC-40 | Vendor-agnostic screening-contract checklist for banks using third-party engines: screening scope, frequency of full-base re-screening, list update frequency and escalation, data-quality standards, test rights | OFAC MidFirst 2022; FCA 2026 good practice (clear contractual arrangements with vendors) |

---

## 17. Sources

All pages were read between 2026-10-06 (today) unless a snapshot date is given. Items marked (search result) were found in a search
result and not opened; the web search budget of the session was used up during this work, so some searches planned for later vendors
were not possible. swift.com and several review sites (Gartner Peer Insights, G2, TrustRadius, Capterra) block automated access.

**LexisNexis Risk Solutions (Fircosoft, Accuity, Bridger)**
- Firco Continuity brochure (2023): https://risk.lexisnexis.com/-/media/files/product%20pages/brochure/lnrs-firco-continuity-brochure2023-nxr15974-00-0423-en-us.pdf
- Firco Entity Resolution Filter brochure (April 2025): https://risk.lexisnexis.com/global/-/media/files/product%20pages/brochure/lnrs_entity_resolution_filter_brochure-nxr16930-00-0425-en-us.pdf
- Firco Continuity Targeted Matching module (Portuguese, 2022): https://risk.lexisnexis.com.br/-/media/files/product%20pages/brochure/lnrs_firco-continuity-optional-module-targeted-matching_nxr15384-00-0322-pt-la%20pdf.pdf
- Firco Compliance Link brochure (April 2025): https://risk.lexisnexis.co.uk/-/media/files/product%20pages/brochure/lnrs-firco_compliance_link_brochure_nxr16933-00-0425-en-us.pdf
- Firco Insight brochure (2021): https://risk.lexisnexis.com/-/media/files/product%20pages/brochure/lnrs_firco%20insight_brochure_nxr15004-00-0721-en-us.pdf
- Bridger Insight XG brochure (2021): https://risk.lexisnexis.com/global/-/media/files/product%20pages/brochure/lnrs_bridger_insight_xg-brochure2021_nxr01251-13-1021-en-us.pdf
- Bridger Insight XG product page: https://risk.lexisnexis.com/products/bridger-insight-xg
- LexisNexis deck "Bringing Certainty to Cross-Border Payments Screening", ABA webinar hosted at aba.org.tw, 7 Oct 2025: https://www.aba.org.tw/wp-content/uploads/2025/10/2025-1007-ABA-Webinar-Payment-Screening.pdf
- Press release 7 July 2025 (Firco Continuity award): https://risk.lexisnexis.com/global/de/about-us/press-room/press-release/20250707-firco-continuity
- Reed Elsevier news release 3 Sep 2014 (FircoSoft acquisition): https://www.relx.com/~/media/Files/R/RELX-Group/documents/press-releases/2014/reed-elsevier-announces-proposed-acquisition-of-fircosoft.pdf
- Accuity and LexisNexis Risk Solutions merger, 17 Feb 2021 (search result): https://risk.lexisnexis.com/about-us/press-room/press-release/20210217-merger
- Chartis FCC50 2024 sanctions screening award (search result): https://risk.lexisnexis.com/global/en/about-us/press-room/press-release/20240517-sanctions-screening-award
- FircoSoft tuning specialist job notice (Capitex, Riyadh): https://jobs.workable.com/jobs/1f6deed1-9db0-4293-84b5-21e754393658.md
- EU Publications Office, DG NEAR WorldCompliance contract (EUR 300,000, 3 Dec 2024): https://op.europa.eu/en/web/public-procurement/procurement-details/-/procurement/142bad45-7789-4f6d-8c73-8565cea0ab4e
- TED notice 142139-2019 (FSCS sanctions screening service, GBP 307,000): https://ted.europa.eu/en/notice/142139-2019/pdf
- UK Contracts Finder, Met Office Bridger Insight XG (GBP 8,500): https://www.contractsfinder.service.gov.uk/Published/Notice/releases/e8c5ccd3-f203-4094-a30d-eb68b0ddd4a2.json

**Eastnets**
- Chartis RiskTech 2025 leader announcement: https://www.eastnets.com/news/eastnets-named-leader-chartis-risktech-2025-watchlist-monitoring-screening
- SafeWatch Screening product page: https://www.eastnets.com/products/safewatch-screening
- Sanctions screening solution page: https://www.eastnets.com/solutions/sanctions-screening
- SafeWatch Screening 5 news (28 Nov 2024): https://www.eastnets.com/blog/news/safewatch-screening-eastnets-sanction-screening-solution-gains-global-momentum
- "Reducing false positives using contextual AI": https://www.eastnets.com/blog/blog/reducing-false-positives-using-contextual-ai
- Al Jazeera Finance case study: https://www.eastnets.com/spotlight-on-precision-how-al-jazeera-finance-transformed-compliance-with-eastnets
- Prime Bank (Kenya) case study: https://www.eastnets.com/enhancing-compliance-integrity-at-prime-bank-with-eastnets-safewatch-screening
- Commercial Bank International (UAE) case study, Oct 2021: https://www.eastnets.com/hubfs/Case%20Studies/CBI_Case_study%20-%20Oct%202021.pdf
- LSEG World-Check On Demand integration, 2 Jan 2026 (CFOtech): https://cfotech.co.uk/story/eastnets-deepens-lseg-tie-up-for-real-time-risk-checks
- Americas migration to Azure, 21 Mar 2023 (search result): https://www.eastnets.com/news/eastnets-successfully-migrates-all-its-americas-swift-and-sanctions-screening-infrastructure-to-microsoft-azure
- Eastnets acquires SIDE International (search result): https://www.finextra.com/newsarticle/16910/eastnets-acquires-side-international
- Founder interview on Amman origins (search result): https://jordannews.jo/Section-125/All/From-a-small-shop-in-Amman-to-a-global-software-powerhouse-12277
- Qi Card about page (search result): https://qi.iq/en/about-us
- Chartis watchlist monitoring page (2024, 2025 reports): https://www.chartis-research.com/financial-crime/watchlist-monitoring

**FinScan (Innovative Systems)**
- Product page: https://www.finscan.com/aml-solution
- Banks page: https://finscan.com/financial-services-banks
- 17 Feb 2026 update: https://www.finscan.com/post/finscan-advances-aml-compliance-by-strengthening-real-time-payments-data-quality-and-global-reach
- Datos Insights award 2025: https://www.finscan.com/post/finscan-recognized-for-best-sanction-watchlist-screening-innovation-in-the-2025-datos-insights-aml-i
- LSEG World-Check On Demand validation, 22 Jul 2026: https://fintech.global/2026/07/22/finscan-validated-for-lseg-world-check-on-demand-access/
- Culture-sensitive matching blog: https://www.finscan.com/post/the-role-of-culture-sensitive-matching-technology-in-aml-compliance
- MENA data and trade finance paper: https://www.finscan.com/post/from-data-woes-to-trade-finance-challenges-enhancing-aml-compliance-in-mena
- Software Advice listing and reviews: https://www.softwareadvice.co.uk/software/334552/finscan
- Spendbase modelled price estimate (unverified): https://test.spendbase.co/?p=37104
- Money20/20 Middle East exhibitor page (search result): https://app.money2020middleeast.com/event/money2020/exhibitor/RXhoaWJpdG9yXzIyNjU5NzI=

**Dow Jones Risk and Compliance**
- RiskCenter Advanced Screening and Monitoring: https://www.dowjones.com/business-intelligence/risk/products/advanced-screening-monitoring/
- Data feeds and APIs: https://www.dowjones.com/business-intelligence/risk/products/data-feeds-apis/
- RiskCenter Financial Crime: https://www.dowjones.com/business-intelligence/risk/products/financial-crime/
- Sanctions risk use case: https://www.dowjones.com/business-intelligence/risk/use-case/sanctions-risk/
- Developer platform, compliance datasets: https://developer.dowjones.com/datasets/datasets-details-compliance
- TED notice 97959-2024 (BaFin, 25 RiskCenter licences, EUR 277,380), read through the TED API: https://ted.europa.eu/en/notice/97959-2024/pdf

**Swift (all via Internet Archive snapshots; swift.com blocks automated access)**
- Sanctions Screening features (18 Nov 2019): http://web.archive.org/web/20191118194933/https://www.swift.com/our-solutions/compliance-and-shared-services/financial-crime-compliance/sanctions-solutions/sanctions-screening/features
- Sanctions Screening FAQ (15 Aug 2020): http://web.archive.org/web/20200815132511/https://www.swift.com/our-solutions/compliance-and-shared-services/financial-crime-compliance/sanctions-solutions/sanctions-screening/frequently-asked-questions-about-swifts-sanctions-screening
- Name Screening FAQ (6 Aug 2020): http://web.archive.org/web/20200806094347/https://www.swift.com/our-solutions/compliance-and-shared-services/financial-crime-compliance/sanctions-solutions/name-screening/name-screening-your-questions-answered
- Transaction Screening page (25 Sep 2021): http://web.archive.org/web/20210925034406/https://www.swift.com/our-solutions/compliance-and-shared-services/financial-crime-compliance/transaction-screening
- Transaction Screening fact sheet (March 2021): http://web.archive.org/web/2022id_/https://www.swift.com/swift-resource/251211/download
- Tadhamon Bank case study (July 2021): http://web.archive.org/web/2022id_/https://www.swift.com/swift-resource/251216/download
- Sanctions Testing features (22 Aug 2019) and FAQ (6 Feb 2023): http://web.archive.org/web/20190822114042/https://www.swift.com/our-solutions/compliance-and-shared-services/financial-crime-compliance/sanctions-solutions/sanctions-testing/features
- Payment Controls (2022): http://web.archive.org/web/2022/https://www.swift.com/our-solutions/compliance-and-shared-services/financial-crime-compliance/fraud-control/payment-controls
- ISO 20022 removal of unstructured address (7 Jun 2026): http://web.archive.org/web/20260607022627/https://www.swift.com/standards/iso-20022/iso-20022-faqs/iso-20022-removal-unstructured-address
- Swift AI address structuring model (11 Oct 2025): http://web.archive.org/web/20251011234915/https://www.swift.com/standards/iso-20022/iso-20022-faqs/swift-ai-address-structuring-model
- End of coexistence support (22 Nov 2025): http://web.archive.org/web/20251122204431/https://www.swift.com/standards/iso-20022/iso-20022-payments-financial-institutions/iso-20022-cpbr-end-coexistence-support

**Napier AI**
- Client Screening: https://www.napier.ai/client-screening
- Transaction Screening: https://www.napier.ai/transaction-screening
- About: https://www.napier.ai/about
- Banco do Brasil case study: https://www.napier.ai/case-study/banco-do-brasil-reduces-false-positives-with-nextgen-screening

**OpenSanctions and open-source projects**
- Licensing: https://www.opensanctions.org/licensing/ ; screening licence: https://www.opensanctions.org/licensing/screening/ ; API and prices: https://www.opensanctions.org/api/
- Tuning the matching algorithm: https://www.opensanctions.org/docs/api/tuning/ ; on-premise: https://www.opensanctions.org/docs/on-premise/ ; commercial FAQ: https://www.opensanctions.org/docs/commercial/faq/ ; matcher training data: https://www.opensanctions.org/docs/opensource/pairs/
- Iraq page: https://www.opensanctions.org/countries/iq/
- GitHub (metadata via API and raw files): https://github.com/opensanctions/yente ; https://github.com/opensanctions/nomenklatura ; https://github.com/opensanctions/rigour (rust/src/text/translit.rs, resources/names/symbols.yml, rigour/names/compare.py) ; https://github.com/opensanctions/followthemoney
- Moov Watchman: https://github.com/moov-io/watchman ; Splink: https://github.com/moj-analytical-services/splink ; Jube: https://github.com/jube-home/aml-fraud-transaction-monitoring ; Tazama: https://github.com/tazama-lf/docs

**Regulators, enforcement and official data**
- FSA Decision Notice, RBS Group, 2 Aug 2010: https://www.fca.org.uk/publication/final-notices/rbs_group.pdf
- FCA Final Notice, Starling Bank, 27 Sep 2024: https://www.fca.org.uk/publication/final-notices/starling-bank-limited-2024.pdf
- FCA, "Sanctions systems and controls in our firms: our findings", 28 May 2026: https://www.fca.org.uk/publications/good-and-poor-practice/sanctions-systems-and-controls-our-firms-our-findings
- OFAC enforcement information pages 2019 to 2025 (120 releases downloaded and scanned): https://ofac.treasury.gov/civil-penalties-and-enforcement-information/2025-enforcement-information (and the same path for 2019 to 2024); cases cited: Apple 25 Nov 2019, Amazon 8 Jul 2020, Deutsche Bank Trust Company Americas 9 Sep 2020, MoneyGram 29 Apr 2021, TD Bank 23 Dec 2021, MidFirst 21 Jul 2022, Uphold 31 Mar 2023, Microsoft 6 Apr 2023, American Life Insurance Company 14 Nov 2024, SkyGeek 31 Dec 2024, Interactive Brokers 15 Jul 2025, State Street 28 May 2019
- Deutsche Bundesbank tender, OJ S 157/2026, 17 Aug 2026 (screening software and extended sanctions lists): https://ted.europa.eu/en/notice/569063-2026/pdf
- Iraqi national list API (public JSON, 628 pages, aggregates computed 2026-10-06): https://api.aml.iq/api/localsanctionslist/getall/1
- LSEG and Iraqna IBTS, 10 Mar 2020: https://www.lseg.com/en/media-centre/press-releases/refinitiv/2020/march/refinitiv-partners-with-iraqna-ibts-to-enhance-regulatory-compliance-efficiency-in-iraq
- TED API (procurement fields): https://api.ted.europa.eu/v3/notices/search

