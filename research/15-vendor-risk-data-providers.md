# 15 — Vendors: risk data providers (LSEG World-Check, Dow Jones, Moody's, ComplyAdvantage, Sayari, Kharon, Castellum.AI, OpenSanctions) and sources of Arabic adverse media

Status: done (2026-10-06)

Part 3 of 4 of the vendor research (screening engines, risk data providers, KYC tools). Sister files: 14 screening engines,
16 KYC and client lifecycle management tools. Requirement IDs use the prefix VC-xx and are numbered across files 14, 15 and 16.
Cross-references: file 01 (section 2d: the CBI 2025 controls name World-Check; section 6: IQTFS national list; section 7: Iraqi ID
documents; section 9: Kurdistan), file 04 (sections 2i negative news, 2n PEPs, 4a UN feed), file 05 (section 1e OFAC 50 percent rule),
file 22 (Arabic name matching).

## Contents

1. What a bank needs from risk data (sanctions, PEP, adverse media, ownership) and what Iraq adds
2. LSEG World-Check (named in the CBI 2025 controls)
3. Dow Jones Risk and Compliance
4. Moody's (Bureau van Dijk Orbis, Grid, the RDC heritage; Passfort is in file 16)
5. ComplyAdvantage
6. Sayari
7. Kharon
8. Castellum.AI
9. OpenSanctions (data licensing)
10. Other providers worth knowing (LexisNexis data, Acuris, Ripjar, others) and official free sources
11. Sources of Arabic (and Kurdish) adverse media
12. Iraqi PEP coverage and Arabic name coverage: what to test before buying
13. Iraqi companies, Kurdistan and ownership data for the 50 percent rule
14. Data quality complaints, legal cases and licensing costs
15. Delivery formats: files, APIs, on-premises feeds
16. Comparison table
17. What it means for AKR-TMS
18. Derived requirements (VC-xx)
19. Sources

(Each vendor section covers: what it does, deployment and delivery, Arabic and MENA presence, pricing signals, what it does right,
what it does wrong, lessons for AKR-TMS, AKR-TMS hooks.)

## Findings in brief

1. **World-Check is a CBI-named requirement, and only its Data File keeps names inside the bank.** Cadence is "twice daily" (2019 fact sheet) or "daily" (2026 FAQ) against the CBI's 12-hour list refresh, so official UN, OFAC, UK, EU and IQTFS lists must be loaded directly.
2. **No supplier publishes Iraqi coverage** (PEP counts, Arabic or Kurdish sources, Iraqi companies). OpenSanctions is the only one with Iraq numbers (724 PEP tags; the Iraqi freeze list at 5,678 entities against 6,280 official rows). Section 12 gives a ten-test evaluation and an RFI.
3. **CBI PEP rules exceed any vendor model**: relatives to the second degree, status after office until a documented downgrade, daily local media; OpenSanctions' default retention is 5 years for most positions and it excludes people without birth dates. The bank must own its PEP register.
4. **Ownership is the largest gap.** CBI beneficial-owner threshold 20 percent versus OFAC's 50 percent aggregate; the Companies Registration Department publishes shareholder tables and BO declarations (about USD 4) but does not verify them; no commercial ownership dataset states Iraqi coverage.
5. **Adverse media for Iraq is partly buildable**: three Iraqi outlets publish RSS and five more sitemaps; Integrity Commission, judiciary, Interior and CBI pages publish no feeds.
6. **Public price signals**: BaFin EUR 277,380 for 25 Dow Jones seats; EUR 300,000 for LexisNexis WorldCompliance (European Commission) and for Bureau van Dijk screening (Norway); about EUR 375,000 for 5 Sayari seats (Germany); ComplyAdvantage USD 99 to 319 a month for 100 to 2,000 monitored entities; OpenSanctions EUR 0.03 to 0.10 per query.
7. **On-premises feasibility**: only World-Check Data File, Dow Jones feeds, Kharon files, OpenSanctions bulk and on-premise yente, and the official feeds run without sending names out; ComplyAdvantage is SaaS-only.
8. 25 derived requirements (VC-41 to VC-65).

---

## 1. What a bank needs from risk data, and what Iraq adds

**Method and limits of this file.** Vendor pages were read directly (curl and the fetch tool) on 2026-10-06; official lists and public
procurement records were read through their own APIs. The shared web-search budget ran out part-way through the vendor research, so
searches for independent reviews, analyst reports and Iraq-specific coverage statements could not be run; the review sites (Gartner Peer
Insights, G2, TrustRadius, Capterra) block automated access. Where a fact could not be verified it is marked. No vendor publishes
Iraq-specific coverage figures except OpenSanctions (section 9), which is why section 12 proposes a test before any purchase.

Four data families sit under the screening engines in file 14:

| Family | What it is | CBI and FATF basis | Main suppliers | Iraq-specific gap |
|---|---|---|---|---|
| Sanctions and watch lists | UN, OFAC, EU, UK, Iraqi national list; law-enforcement lists | CBI controls 2025 ch. 5 art. 5: UN, OFAC, EU and CFTF list, refresh at least every 12 hours, Arabic and English | Official feeds (free); every vendor repackages them | The Iraqi list is Arabic-only (6,280 rows, 3 aliases); UN has original-script names for only 379 of 1,010 entries (file 04 s.4a) |
| PEPs, relatives and close associates (RCA) | Office-holders and their families | CBI PEP circular 26 Aug 2026: relatives to second degree; update "immediately" when office-holders change; "do not rely only on the politician names inside commercial search systems"; status persists after office until a documented downgrade (file 01 s.2c) | LSEG World-Check, Dow Jones, LexisNexis, Moody's Grid, ComplyAdvantage, OpenSanctions | No vendor states Iraqi coverage; relatives to second degree exceed what vendors publish |
| Adverse media | Negative news by crime category and stage | CBI PEP circular: daily screening of "reliable local and regional news and media sources"; Wolfsberg negative news FAQ (file 04 s.2i) | Factiva (Dow Jones), LexisNexis, World-Check Media Check, Moody's Grid, ComplyAdvantage | Arabic and Kurdish Iraqi outlets: coverage unstated; open feeds are partly machine-readable (section 11) |
| Ownership and control | Who owns or controls a listed person's companies; state ownership | OFAC 50 percent rule (file 05 s.1e); CBI: identify beneficial owners; CDD instructions: 20 percent threshold | Dow Jones SCO, Moody's Orbis and Sanctions360, Sayari, Kharon, Castellum.AI, LSEG (via Dun & Bradstreet), the bank's own KYC | Iraq's Companies Registration Department "has no mechanism to verify" beneficial ownership information (MER, file 01 s.2); R.24 rated partially compliant |

Facts about Iraqi data that affect every vendor test (from files 01, 04, 22 and the public IQTFS API on 2026-10-06):
- Names are long chains: the national list has a median of 5 whitespace tokens and a maximum of 12; mother's name is present on 99.2 percent of
  rows and birth year on 95.5 percent.
- Iraqi ID documents carry father's and grandfather's names, mother's name, and a family number (file 01 s.7).
- The CBI names one commercial product (World-Check) and no other vendor (file 18 s.1).

---

## 2. LSEG World-Check (the product the CBI names)

### 2.1 Who and what

- **History.** World-Check was founded in 2000 (London; "first to the market in 2000 with our KYC solutions", LSEG fact sheet). Thomson
  Reuters bought it in 2011; after the Blackstone deal of October 2018 it sat in Refinitiv, which became LSEG in 2023 (Wikipedia
  summary; secondary source). LSEG Risk Intelligence now sells it.
- **Content.** "Sanctions, law and regulatory enforcement, politically exposed persons, relatives and close associates, state-owned or
  state-invested enterprises, and relevant adverse media" (LSEG page, 2026). Research claims: "more than 400 research analysts working in
  over 65 languages, covering 240 countries", "700+ sanction, watch, regulatory and law enforcement lists" and "100 percent sanctions
  coverage" (World-Check One brochure, Oct 2023); "hundreds of trained analysts speaking more than 70 languages" (About page, 2026). The
  Data File fact sheet (September 2019) says "more than 4 million highly structured records", "more than 30 risk categories" and "sanctions
  control and ownership information". Newer opt-in sets: Iran Economic Interest (IEI), UBO (powered by Dun & Bradstreet), vessels, sanctioned
  securities, "narrative and implicit sanctions", "sanction sets for payment screening", and Media Check (AI negative-media screening).
- **2026 positioning.** "Active Intelligence" is described as the AI-enabled data and technology capability behind World-Check, including
  **World-Check Resolve** (intelligent match resolution to cut false positives). LSEG won the Chartis FCC50 Watchlist Data Award 2025 and
  Financial Crime Data Management Award 2025 (LSEG page).

### 2.2 Delivery options (important for an on-premises bank)

| Option | What it is | Key facts (LSEG pages and fact sheets) |
|---|---|---|
| **World-Check One** | Cloud screening and case-management platform | Batch, single-name, ongoing screening, Watchlist Screening (upload your own lists), user interface in multiple languages; "World-Check data is updated daily" (2026 FAQ) |
| **World-Check One API** | HTTPS/JSON API into the platform | Synchronous or asynchronous; "supports screening of names in different languages and scripts (using the World-Check One matching engine)"; secondary identifiers; ongoing screening; opt-in sets (IEI, US SAM, Media Check); fact sheet dated September 2019 |
| **World-Check On Demand** | Structured data delivered by API into the customer's own screening platform | "Integrate data into internal screening platforms and tailor retrieval"; "fully structured, aggregated, and de-duplicated"; embedded by Eastnets (2 Jan 2026) and FinScan (22 Jul 2026) |
| **World-Check Verify** | Stateless, low-latency screening API for onboarding and payments | Built with AWS; "does not store customer PII", "in-region hosting options" |
| **Data File** | Bulk files for third-party or proprietary workflow systems doing CDD, customer screening and payment screening | "Unlimited downloads"; "daily, weekly and monthly updates as well as entire" files; "intelligence that is updated twice daily"; "in a choice of common file formats"; "automate your in-house screening for maximum privacy" (fact sheet RE1024695, September 2019) |
| **Zero Footprint** | Switch off tracking, ongoing screening, batch upload and audit trail in the screening API so "no permanent record is kept" | Fact sheet |
| Due diligence reports and Screening Resolution Service | Managed services | Brochure |

Meaning for AKR-TMS: **only the Data File route keeps customer names inside the bank** (the others send names to LSEG-operated cloud
services; Verify and Zero Footprint reduce retention but still transmit names). The Data File fact sheet is dated 2019, so current formats,
delta mechanics and update frequency must be confirmed in a contract. The 2019 sheet says "updated twice daily"; the 2026 World-Check One
FAQ says "daily"; the CBI requires lists "every 12 hours at least". World-Check alone may therefore not meet the CBI's 12-hour rule for UN,
OFAC, EU and national lists, and the bank should take official lists directly (VC-03).

### 2.3 Arabic, Iraq and MENA

- **Iraq channel.** LSEG (as Refinitiv) announced a partnership with **Iraqna IBTS**, an Iraqi training and compliance consultancy and
  exclusive local agent for ACAMS exams, on 10 March 2020, to support Iraqi institutions' KYC and AML in line with FATF and MENAFATF work.
  The access method is not described. The CBI's January 2025 controls then named World-Check explicitly (file 01 s.2d).
- **Arabic matching.** The API fact sheet says names in different languages and scripts are supported, with no method, no Arabic example and
  no test result. LSEG says researchers work in "over 65" or "more than 70" languages. How Arabic-script names are stored (original script,
  transliterations) is not stated on the pages read.
- **Iraqi coverage numbers**: none published (PEPs, companies, media sources).
- **Data-subject route.** "Am I on LSEG World-Check?" lets a person request a copy, correction, restriction or removal; LSEG aims to
  respond "within 10 working days". This is the formal channel for disputing a record and a bank should know it.

### 2.4 Pricing signals

None public for World-Check. No tender notice naming World-Check with a readable price was found (the TED full-text search for "World-Check"
returned two notices from a Polish state bank, BGK, for "electronic information services" of PLN 23.4 million, whose contractor and scope I could not
read; they are not a reliable World-Check price). The CBI's naming of the product means Iraqi banks buy it anyway, which
gives LSEG pricing power; ask for a bank-wide Data File quote and compare it with the alternatives in sections 3 to 9.

### 2.5 What it does right

1. **Established regulatory recognition**: named in the CBI controls; "100 percent sanctions coverage" claimed; researched, structured and de-duplicated records with source references.
2. **Several delivery routes**, including bulk files for in-house screening (the on-premises case) and an API for engines.
3. **Ecosystem integration**: Eastnets and FinScan embed World-Check On Demand; Swift's Name Screening and many engines load World-Check.
4. **Ownership additions** (UBO via Dun & Bradstreet, "narrative and implicit sanctions", IEI) and Media Check.
5. **A published route for data subjects** to correct or remove records.

### 2.6 What it does wrong (evidence)

1. **Documented false-labelling incidents.** In 2017 World-Check admitted that the Palestine Solidarity Campaign should never have been on the database nor associated with terrorism, and agreed to pay compensation and apologise to Finsbury Park Mosque; a Vice News investigation (published 4 February 2016 and 18 October 2017) reported a person wrongly listed as a terrorist, who was removed and compensated (Wikipedia summary of those reports; secondary). The lesson is that a record is a lead, not a finding.
2. **Numbers and update claims differ across LSEG documents**: 65 versus 70 languages; "twice daily" versus "daily"; a 2019 record count of "more than 4 million" with no current figure on 2026 pages.
3. **No Iraq or Arabic evidence** on any page; no sample extract is public; the CBI names the product without publishing a coverage test.
4. **Cloud-first delivery.** Four of five options send customer names to LSEG services; a bank wanting names to stay inside needs the Data File and its own matcher (so the matching quality becomes the bank's responsibility).
5. **UBO via a partner** (Dun & Bradstreet), the same dependence as Dow Jones.
6. **Pricing opaque**, with a named-product requirement that reduces bargaining power.

### 2.7 Lessons for AKR-TMS

- **Support World-Check from day one** through the Data File (files, delta) and On Demand API connectors, and treat it as one source among several.
- **Do not rely on World-Check alone for the CBI's 12-hour rule**; load official lists directly.
- **Store the profile snapshot** (record id, date, content hash) with each alert decision so that later removals or corrections do not erase the basis of an old decision.
- **Provide a dispute log**: when a customer disputes a listing, keep the record, the request and the vendor's answer.

**AKR-TMS hooks**
- **World-Check adapter**: Data File ingestion (full and incremental) into the unified watch list with provenance; On Demand API connector with caching and rate limits for ad-hoc and EDD lookups; both optional by licence flag.
- **Profile snapshot** stored in the evidence vault with each screening decision (record id, retrieval time, hash, source).
- **Dispute and correction register** linked to customer and vendor record.
- **Vendor cadence monitor**: record the actual publication time of each vendor file and compare with the CBI 12-hour limit.

---

## 3. Dow Jones Risk and Compliance (data side; the screening tool side is file 14 section 5)

### 3.1 What it sells (Dow Jones pages, 2026)

| Content set | Facts |
|---|---|
| Sanctions | In-house research team; "over 50" (one page) or "over 80" (another page) sanctioning authorities; Sanctions Control and Ownership (SCO): "more than 54,000 companies" (RiskCenter page) or "over 56,000 entities" (data feeds page) owned or controlled by sanctioned persons, "enabling organizations to comply with the OFAC 50% rule and equivalent laws" |
| PEPs, relatives, close associates | "Senior public officials ... relatives, and close associates"; roles in "22 job categories" (RiskCenter) with filtering by function; a dedicated category to include or exclude national-definition PEPs; positioned for the EU AMLR (July 2027) |
| Adverse media | Structured Adverse Media Entities (companies) and Special Interest Persons (individuals in legal processes for corruption, fraud, trafficking, terror and other crimes); unstructured Factiva: "33,000+ sources in 33 languages across 200 countries" |
| State-owned companies | Government ownership of 1 percent or more, voting control, board control |
| Ownership | Beneficial ownership through Dun & Bradstreet: "270 million companies" (RiskCenter page) or "nearly 580 million records" (feeds page); direct and indirect, with "degree of separation" |
| Other | Other Official Lists (BIS and "over 400 other regulatory or enforcement bodies"), vessels, ports and cities, dual-use goods, financial instruments, money services businesses (Canada, Hong Kong, Malaysia, the Philippines, Singapore, US) |
| Payments | LPTS feed "updates up to six times a day" |

Delivery: REST APIs (Ad Hoc Search, bulk Screening and Monitoring for "10K+ names"); feeds by SFTP cloud storage or Pub/Sub, customised by content set, list type and active status (developer platform page). Dow Jones customises the feeds; on-premise engines can ingest the files.

### 3.2 Arabic, Iraq, price

- No Arabic, Iraq or Middle East statement on any Dow Jones risk page. Records "contain original script names" (examples in Cyrillic and Chinese). Factiva's 33 languages may include Arabic but the language list was not read.
- **Price signal**: BaFin bought 25 RiskCenter licences with Factiva Pro for EUR 277,380 (contract concluded 14 Feb 2024; about EUR 11,100 per seat; duration not stated).

### 3.3 What it does right and wrong

| Right | Wrong (evidence) |
|---|---|
| SCO is the market's clearest packaged answer to the OFAC 50 percent rule; LPTS gives a payment-optimised list | Inconsistent figures on Dow Jones' own pages (50 versus 80 authorities; 54,000 versus 56,000; 270 million versus 580 million) |
| Factiva is a strong licensed news archive; feeds and APIs suit on-premise engines | Iraq and Arabic silent; MSB data excludes the Middle East; UBO depends on Dun & Bradstreet |
| Used by supervisors (BaFin) and embedded in Swift Name Screening and Transaction Screening lists | Per-seat and per-module pricing; no bank-scale public price |

### 3.4 Lessons and hooks for AKR-TMS

- Use SCO-type data as an **optional enrichment feed** into the derived-designation table (file 14 VC-21), never as the only ownership source.
- Ask in procurement: Iraqi PEP counts by tier and by the CBI relative definition; Arabic-script fields; whether the Iraqi national list is in the "sanctioning authorities"; how Iraqi companies appear in SCO.
- **Hook**: a feed adapter for Dow Jones files (SFTP/Pub/Sub) mapping SCO, SOC, PEP and AME records into the unified model with provenance.

---

## 4. Moody's (Bureau van Dijk Orbis, Grid, Sanctions360, Entity Verification; Passfort is in file 16)

### 4.1 What it is (Moody's pages, July 2026 figures)

| Product | Facts |
|---|---|
| **Orbis** (Bureau van Dijk) | "More than 635 million companies"; "blends data from more than 170 different sources"; "corporate and beneficial ownership structures", corporate trees; country-level products (AIDA, FAME, DIANE, MARKUS, ASTREE, all European) |
| **Grid** (risk database; the line that came from Regulatory DataCorp, acquisition date not verified here) | "30M+ risk profiles"; "coverage across 200+ countries and jurisdictions"; adverse media "50+ risk categories", PEP coverage "across more than 200 jurisdictions" with family and close associates; state-owned and state-invested entities; sanctions and watch lists from official authorities |
| **Sanctions360** | "Data on more than 600 million entities ... and millions of curated risk profiles"; OFAC (SDN, SSI, non-SDN), EU and UK lists (including Article 5r of EU Regulation 833); sanctions by ownership, subsidiaries and leadership; network graphs; user interface or API. The page lists US, EU and UK lists only (no UN or national lists named) |
| **Entity Verification API** (the kompany heritage) | One API into "commercial registries, financial authorities, and tax offices in more than 200 countries and jurisdictions", with original documents |
| **Moody's for Compliance and Maxsight** | Orchestration and AI agents for onboarding, periodic review and pKYC (Passfort lineage; file 16) |
| UBO | "Calculated ownership derived from multiple sources"; Moody's itself notes "public access to beneficial ownership registries becoming more restricted" (May 2026) |

### 4.2 Arabic, Iraq, pricing

- None of the Moody's KYC pages read mentions Iraq, Arabic or the Middle East. Coverage of Iraqi companies in Orbis and of Iraqi PEPs in Grid is unstated; both must be tested (section 12).
- **Pricing signals (public procurement)**:
  - Eksportfinansiering Norge (Norway's export credit bank) awarded a screening system for KYC ("ownership, listing on sanctions lists, negative media reviews and criminal conditions") to Bureau van Dijk Electronic Publishing AB, final value **EUR 300,000** (notice 499165-2024, 20 Aug 2024).
  - National Wealth Fund (UK): "Sanctions-related due diligence in customers, counterparties, and supply chain" awarded to Moody's Analytics UK Limited, **GBP 59,994 to GBP 99,990** for 30 Jan 2025 to 11 Sep 2026 with an optional year (UK Contracts Finder).
  - UK Contracts Finder, 30 Mar 2022: GBP 43,000 for additional screening due to Russia and Ukraine sanctions awarded to Moody's Analytics UK Ltd (search-result summary; not opened).

### 4.3 What it does right and wrong

| Right | Wrong (evidence) |
|---|---|
| Largest company-ownership database in the market (635 million entities) and registry-document access; network graphs that expose ownership chains | No Iraq or Arabic statement; ownership outside Europe is "calculated" and depends on what registries publish, which Moody's says is shrinking |
| Combines ownership (Orbis), risk data (Grid) and a workflow layer (Passfort) in one vendor | Sanctions360 names US, EU and UK lists only; UN and Iraqi lists must come from elsewhere |
| Priced in the open for public buyers: USD 60k to 100k a year class for modest use | Per-seat and per-module pricing; enterprise price not public |

### 4.4 Lessons and hooks

- Orbis-type data is the likely commercial source for **derived designations** outside Iraq (Iraqi companies owned through UAE, Turkish, Jordanian or Lebanese holding companies); the Iraqi layer must come from the bank's own KYC and the Companies Registration Department data.
- **Hook**: an ownership-data adapter with source tagging (bank KYC, registry document, commercial calculated) and confidence; the 50 percent engine uses only edges with a source and a date.

---

## 5. ComplyAdvantage (Mesh)

### 5.1 What it is (pages read 2026-10-06)

SaaS-only financial crime platform: "ComplyAdvantage Mesh is an industry-leading and trusted SaaS-based risk intelligence platform". Data: sanctions from OFAC, HMT, the EU "and over 60 additional jurisdictions", including entities owned or controlled by sanctioned parties "aligning with the OFAC 50 per cent rule"; PEP and RCA data with "daily updates" and election changes; adverse media "classified into 34 distinct financial crime categories" from "millions of global sources"; a knowledge graph of "24 million+ entities"; payment screening (sender, receiver, intermediary banks, BICs, reference text; "critical sanctions lists are refreshed within minutes of publication"; every configuration change recorded with user, timestamp and prior version); transaction monitoring; "agentic AI to resolve up to 85% of routine alerts". Claims: "82% reduction in false positives". Data feedback loop: "improves with every remediation decision" across "thousands of financial crime teams".

### 5.2 Arabic, Iraq, price

- "Linguistic identity matching, expertly handling the intricacies of naming conventions, aliases / AKAs, non-Latin names" (PEP page); no Arabic or Iraq statement.
- **Public price list (rare):** Starter Plan from **USD 99 a month** (annual billing) for 100 monitored entities up to **USD 319 a month** for 2,000 (USD 119 to USD 383 monthly billing); a higher tier with agentic AI USD 149 to USD 479 (annual); overage at 1.5 times the unit rate; above 2,000 entities, custom pricing. That is about USD 0.16 per monitored entity per month at 2,000 entities; a bank with a million customers is outside the public plans.

### 5.3 What it does right and wrong

| Right | Wrong (evidence) |
|---|---|
| Transparent entry pricing; fast list refresh claimed; configuration change log for screening settings; one platform for screening, monitoring and payments | **SaaS only**: customer names leave the bank; incompatible with an on-premises default and with Iraqi banks' caution on data (file 01 s.8) |
| Strong adverse-media taxonomy (34 categories) and PEP/RCA focus | Cross-customer "feedback loop" raises confidentiality questions (what is learnt from one bank's decisions and shared); claims of 82 percent and 85 percent are vendor-only |
| API-first | No Iraq or Arabic evidence; no published latency or throughput |

### 5.4 Lessons and hooks

- Copy the **per-source and per-attribute sensitivity settings** and the **configuration change log** (VC-29).
- Avoid cross-bank learning from analyst decisions unless contractually and technically isolated; in Iraq, tipping-off and secrecy rules make shared decision data a risk.
- **Hook**: published price per screened entity per month as a comparator when AKR-TMS prices its own screening module.

---

## 6. Sayari

### 6.1 What it is (pages read 2026-10-06)

Washington-based corporate-ownership and trade-data company (founded 2015; USD 40 million Series C in 2021; USD 8.5 million IDIQ contract with the US Treasury Department; "used by more than 3,000 frontline analysts in 35 countries"). Products: **Sayari Graph** (entity resolution and ownership graph; "500M+ companies", "3B+ ownership links", "12.4B+ records", "250+ jurisdictions"), **Sayari Signal** (continuous monitoring), **Sayari API** ("40+ sanctions lists, with 50% rule ownership attribution applied automatically"; batch up to 10,000 entities; 4 billion-plus trade transactions). KYC page: independently resolves beneficial-ownership chains "across 250+ jurisdictions" and flags discrepancies with customer declarations; "10+ degrees of corporate ownership" in one query; claims 65 percent less manual ownership research time.

### 6.2 Arabic, Iraq, price

- No Iraq, Gulf or Arabic statement on the pages read. Evidence on regional interest: Germany's Federal Office for Foreign Affairs bought Sayari for "ownership structure/owners of companies in mainland China, Hong Kong, the United Arab Emirates, Saudi Arabia, Qatar, Russia" (TED notices 547615-2025 and 628822-2025): **five named users, 1 Oct 2025 to 30 Sep 2026, with two 12-month options, estimated value EUR 375,000** (the notice does not say whether the value includes the options). Iraq is not in that list.
- US Treasury IDIQ USD 8.5 million (company announcement; date not stated on the page).

### 6.3 What it does right and wrong

| Right | Wrong (evidence) |
|---|---|
| Ownership resolution to natural persons with source documents; automatic 50 percent attribution; batch and monitoring APIs | No Iraq coverage statement; "250+ jurisdictions" is a count, not a quality measure; the GCC is a stated buyer interest, Iraq is not |
| Government-grade references (Treasury) | Cloud platform and API; no on-premise route; names of customers and counterparties leave the bank |
| Trade (customs) data adds a second signal | Price per named user (German tender: EUR 375,000 for five users is about EUR 75,000 per user-year if the value covers one year, or about EUR 25,000 if it covers the full three years; the notice is ambiguous) is a government research-seat price, not a bank-wide screening price |

### 6.4 Lessons and hooks

- Sayari-style ownership resolution is a **specialist add-on**, not a baseline; AKR-TMS should accept its output as an enrichment source and log which chain edges came from it.
- **Hook**: "ownership chain evidence" object (nodes, edges, percentage, source, date, vendor) attached to cases and to derived-designation alerts.

---

## 7. Kharon

### 7.1 What it is (pages read 2026-10-06)

Sanctions and trade-risk intelligence company with analyst-led research. Datasets: **50-Plus** ("entities majority owned, directly, indirectly, or in the aggregate, by sanctioned actors"; Kharon states that "more than 30% of sanctioned-by-law companies are three or more degrees removed from the sanctioned parent"), **Control** (entities controlled without ownership: affiliates of sanctioned entities and state-owned enterprises, companies "controlled or founded by family members of sanctioned individuals"), plus Russia, Belarus, Venezuela, crypto, maritime, military end use, forced labor and sanctioned securities sets. Products: **Kharon Core** (research database), **ClearView** (investigation interface), **GraphCast** (targeted data), **API** (Dynamic Exposure Scores), CoreStream. "Used operationally by U.S. Department of the Treasury, Department of Homeland Security, Bureau of Industry and Security, and dozens of other international government partners." Banking page: "data files that integrate directly into existing customer and transaction screening software — no rip-and-replace"; partners include NICE Actimize. Recognition: Risk Technology Awards 2026 third-party risk product of the year; AIFinTech100 2026.

### 7.2 Arabic, Iraq, price

- No Iraq or Arabic page; Iran, IRGC or Iraqi militia datasets are not listed among the 12 named data pages. The banking page's client strip includes "Largest Middle Eastern & African Banks" (names not shown). No public price.

### 7.3 What it does right and wrong

| Right | Wrong (evidence) |
|---|---|
| Analyst-sourced 50-Plus and Control sets address exactly the OFAC 50 percent and "control" exposures name matching misses; evidence trail ("ownership linkages are sourced, documented, and traceable") | Control data is open-source research by Kharon; a bank cannot reproduce it, so an alert based on it needs the underlying evidence |
| Files can feed existing engines without a new platform | No Iraq-specific dataset advertised even though OFAC's 2025-2026 actions against Iraqi militia-linked companies and bankers are the live Iraq exposure (file 05 s.1e); price opaque |
| Government use is a credibility signal | Limited public evidence on Arabic transliteration of aliases |

### 7.4 Lessons and hooks

- Ask Kharon whether the 50-Plus and Control sets include the entities named in OFAC's Iraq-nexus actions (Kata'ib Hizballah-linked companies, Iran-oil smuggling networks, bankers) and whether aliases are held in Arabic.
- **Hook**: an "evidence-linked derived designation" type that stores the control relationship (ownership, family, affiliate) with its source and review date, so that a Kharon or Dow Jones record can be re-checked.

---

## 8. Castellum.AI

### 8.1 What it is (pages read 2026-10-06)

US company selling sanctions, PEP and adverse media data with its own screening API and AI agents (investors "representing 100+ banks and credit unions"; Lead Bank as a named customer). Facts: "1,700+ watchlists" (screening page) or "1,000+" (home page), "200,000+ sources globally", updated "every 5 minutes (legacy systems take 24 hours)", "data sourced directly from issuers — often before public press release", a "patented Jgram search algorithm", "language-agnostic algorithm accurately matches names across 100+ languages, including Arabic, Chinese, Farsi, Korean, and Russian", ">50M" records, "94% fewer false positives", agents that "instantly resolve 83% of L1 and L2 alerts", "passed a fincrime exam". A free static sample file of sanctions designations (OFAC, UK OFSI, Argentina, India, Indonesia, Singapore MAS, Saudi Arabia Presidency of State Security, South Africa FIC) is available for testing; purchased data "includes passport numbers, identity numbers, crypto wallet addresses, locations, places of birth, dates of birth" and is refreshed every five minutes. The pricing page shows tiers without figures in the page text.

### 8.2 Arabic, Iraq, price

- Arabic and Farsi appear in a language list only. No Iraq data page. The free sample's Saudi list shows GCC interest, not Iraq. No public price.

### 8.3 What it does right and wrong

| Right | Wrong (evidence) |
|---|---|
| Very fast list refresh claimed (5 minutes) from issuers; a free sample file for test-before-buy; own data feeds its own agents (no third-party SLA chain) | US community-bank and credit-union base; no MENA or Iraq reference; matching method (Jgram) is proprietary with no Arabic test published |
| Explainable agent narratives and policy training | "94 percent fewer false positives" and "83 percent of alerts" are vendor-only; agent clearing needs the same governance as in file 14 s.7 |
| API and files | Hosted data and agents; no on-premise statement on the pages read |

### 8.4 Lessons and hooks

- **Test before buy using a free sample**: Castellum's free file, OpenSanctions' free evaluation and Swift Sanctions Testing's public-data approach show a pattern AKR-TMS should follow: publish a sample test pack so banks can try the matcher with their own names.
- **Hook**: a "list refresh target" metric in minutes for UN and Iraqi lists; five-minute polling is feasible for sources that publish quickly (the UK Sanctions List XML had a last-modified time of 10:00 GMT on 2026-10-06).

---

## 9. OpenSanctions as a data supplier (the matcher and software are in file 14 section 9)

### 9.1 Coverage and cadence (OpenSanctions pages, 2026-10-06)

- Entities: 1,134,248 people; 297,233 organizations; 230,011 securities; 14,049 crypto addresses; 10,512 vessels and aircraft; 3,483
  enforcement notices; "460+ sources"; "47mn+ facts". The hosted API serves "1.9mn+" entities and 460+ sources.
- Updates "four times per day"; delta patches (ADD, MOD, DEL, in FollowTheMoney JSON only) for most datasets; the dataset index lists the last
  100 versions, so a consumer more than 100 versions behind must reload fully. A DEL carries only the id, not the removed data, and a merge of
  two entities produces two DELs and one ADD with a new id (documentation). Formats: FollowTheMoney JSON, simplified CSV, plain-text names,
  Senzing.
- Provenance: every stated value links to a public source; the model is statement-based.

### 9.2 PEP data: the position-based model and what it means for Iraq

- **Model.** OpenSanctions models offices ("Position"), links people to them ("Occupancy"), and tags the person as PEP; topics by role and level
  (head of state, executive, legislature, supreme judiciary, security, central bank, ambassadors, state-owned enterprise boards, party leadership,
  religious leadership, regional and municipal levels). "We prioritize national-level coverage; subnational and local positions are included where
  we have reliable, maintained sources."
- **Retention windows after leaving office**: head of state or government 50 years; other national-level positions, intergovernmental and
  diplomatic 20 years; "all other positions" **5 years**. The CBI requires that PEP status "continues after the PEP leaves office" until a
  documented risk assessment ends it (file 01 s.2c), so any vendor window must be overridden by the bank's own record.
- **Sources and gaps.** "PEP data is source-dependent: we include what is documented in publicly available sources"; upstream sources include
  Wikidata and government-published lists; "Manual PEP research and profile creation is not part of our commercial service". **Relatives and close
  associates** are included only where documented, "primarily Wikidata and official declarations", and OpenSanctions does "not independently
  research family or business networks". The CBI requires relatives to the second degree.
- **Exclusions.** "Individuals with no birth date, death date, or position dates are excluded from the database"; people over 110 or dead for more
  than five years are excluded; open occupancies older than 40 years are not current. Many Iraqi office-holders have incomplete public biographical
  data, so the birth-date rule can drop real PEPs.
- **Iraq numbers.** The Iraq page shows topic tags of 724 PEP and 387 politician for 1,262 people connected with Iraq, and the sentence "724 people
  identified as political office-holders connected with Iraq". The only Iraqi authority source is the Terrorist Fund Freezing Lists (5,678
  entities against 6,280 rows in the official API).

### 9.3 Licensing and price (full terms in file 14 s.9.4)

CC BY-NC 4.0 data; Screening API credits EUR 0.10 down to EUR 0.03 per query (EUR 50 for 500 credits to EUR 15,000 for 500,000; more than 1 million
on quote; EU VAT number required); Screening licence (flat rate, internal use, quote); Reseller/OEM licence needed when bulk data is loaded into a
database, search index or matching engine that serves customers; licensed only as a complete collection; self-hosted yente is MIT software.
**For AKR-TMS the decision is whether each bank licenses the data directly or AKR-TMS takes an OEM licence**; a hosted API call per customer
name is not acceptable for names leaving an Iraqi bank.

### 9.4 What it does right and wrong

| Right | Wrong (evidence) |
|---|---|
| Open, inspectable data with provenance; delta files; hourly polling; free evaluation; modest, published prices | PEP data limited to what public sources document; relatives not researched; no Arabic-script or Iraqi-specific research; birth-date exclusion; 5-year default retention |
| Carries the Iraqi AML/CFT Office's list as a source (daily) | Count differs from the official API (5,678 versus 6,280); the cause is unknown |
| Position-based PEP model gives "which office makes them a PEP" | "Not an end-user compliance application"; no service warranty on the free website |

### 9.5 Lessons and hooks

- **Hook**: a **data-reconciliation job** for every external copy of an official list (row counts, identifiers, last-changed), reporting differences to the MLRO; apply it first to OpenSanctions' copy of the Iraqi list.
- **Hook**: PEP register in AKR-TMS keeps its own lifecycle states (active, former, downgrade proposed, downgraded) independent of any vendor's retention window.
- **Hook**: delta-file consumer with its own snapshot store, because a DEL carries no data and merges change ids.

---

## 10. Other providers worth knowing, and the official free sources

### 10.1 LexisNexis Risk Solutions (data; engine details are in file 14 s.2)

WorldCompliance Data Plus behind Bridger Insight XG: "180 global sanctions lists", "3.4 million+ PEP profiles", adverse media "across 59 languages", "8M+ risk profiles", "250 countries and territories"; Firco Compliance Link: "115+ watchlists and 70+ controlled goods lists"; Firco Global WatchList (Accuity heritage). **Price signals**: European Commission DG NEAR EUR 300,000 for WorldCompliance screening of beneficiaries (3 Dec 2024); Portugal's tax authority EUR 327,270 for a WorldCompliance notice (TED 710751-2022; notice type "software package and information systems", so the attribution to a screening service is unverified). No Arabic or Iraq statement.

### 10.2 Dun and Bradstreet (as a sub-supplier)

Both LSEG (UBO Check) and Dow Jones (beneficial ownership) use Dun & Bradstreet UBO data. D&B coverage of Iraqi and Kurdistan companies was not found in any page read; this single upstream dependency should be tested once (section 13).

### 10.3 Other names (not researched in depth because the search budget ended)

KYC360 and LSEG Risk Intelligence were the 2024 Chartis category leaders for name and transaction screening and adverse media (Chartis page); ACTICO, FinScan, Eastnets and Sigma360 were named in 2025. Ripjar, Acuris, Sigma360 and others sell adverse-media AI; none was read.

### 10.4 Official free sources (these are the first sources a bank should load directly)

| Source | Facts observed 2026-10-06 |
|---|---|
| UN Security Council consolidated list | XML; 736 individuals and 274 entities; 379 with original-script names; amended on 14 dates in 12 months (file 04 s.4a) |
| OFAC Sanctions List Service | SDN and Consolidated (non-SDN) lists for immediate download; customised datasets; "Archived Published Delta Files"; Advanced Sanctions Data Model (XML); e-mail notifications; Sanctions List Search tool with a published scoring FAQ (OFAC page) |
| UK Sanctions List (FCDO) | One XML file of 21.9 MB; `Last-Modified: Tue, 06 Oct 2026 10:00:19 GMT`; the old OFSI ConList.csv no longer exists (404) |
| EU consolidated financial sanctions list | The public files endpoint answered 403 without a token; the EU publishes it through the FSF service (token) and data.europa.eu; not downloaded |
| Iraq national list (AML/CFT Office IQTFS) | Public JSON API, 628 pages of 10; 6,280 rows; fields: name, type, alias, birth year, mother's name, decision number and year, excluded flag; company lookup across registries needs authentication (HTTP 401 for anonymous calls) |

---

## 11. Sources of Arabic (and Kurdish) adverse media

### 11.1 What the rules ask for

- CBI PEP circular (26 Aug 2026): do "not rely only on the politician names inside commercial search systems"; search "reliable local and regional news and media sources as part of daily screening" (file 01 s.2c).
- Wolfsberg negative-news FAQ (file 04 s.2i): screen in the local script where sources exist; document why sources are included or rejected; rate source credibility, consider press freedom and state control; disinformation is a real risk; keep article snapshots because articles are taken down.

### 11.2 Licensed aggregators: language and source statements

| Supplier | Statement (vendor pages, 2026) | Arabic or Kurdish stated? |
|---|---|---|
| Dow Jones Factiva | "33,000+ sources in 33 languages across 200 countries"; structured Adverse Media Entities and Special Interest Persons | Not stated on the pages read |
| LexisNexis | "thousands of reputable and reliable news media sources across 59 languages" | Not enumerated |
| LSEG World-Check | Researchers "working in over 65 languages" (brochure) or "more than 70 languages" (About page); Media Check AI | Not stated |
| Moody's Grid | "global publications and broadcast sources", "50+ risk categories" | Not stated |
| ComplyAdvantage | "millions of global sources", 34 categories | Not stated |
| Castellum.AI | "200,000+ sources globally" | Arabic in a language list only |

None of the six states Arabic or Kurdish Iraqi outlets by name. Before purchase, each supplier should provide the list of Iraqi and Kurdish sources it ingests, with language, frequency and retention.

### 11.3 Open Iraqi and regional sources: what is machine-readable (probe on 2026-10-06; HTTP status, feeds in the page header, sitemap file present)

| Source | Result |
|---|---|
| Iraqi News Agency (ina.iq) | Homepage 200; **RSS feeds** `rss.xml` and `rssturbo.xml`; sitemap.xml 200 |
| Baghdad Today | 200; **RSS** `rss.xml`; sitemap 200 |
| 964 Media | 200; RSS feed link in page; sitemap 200 |
| Shafaq News (Arabic) | 200; no feed link in page; sitemap 200 |
| Rudaw (Arabic edition) | 200; no feed link; sitemap 200 |
| Kurdistan24 (Arabic) | 200; no feed link; sitemap 200 |
| Alsumaria | 200; no feed link; sitemap 200 |
| Al-Mada | 200; no feed link; sitemap 200 |
| Alsabaah | no response in the probe |
| Integrity Commission (Nazaha), Supreme Judicial Council, Ministry of Interior, Central Bank of Iraq, Kurdistan Regional Government | Homepages respond (200); **no feed and no sitemap.xml** found; Council of Representatives site did not respond |

Meaning: three outlets offer RSS and five more offer sitemaps (which list article URLs and often dates), so a polite crawler can follow them. The official sources that matter most for corruption and enforcement news (Nazaha announcements, court notices, Ministry of Interior wanted lists, CBI circulars) publish no feed, so they need page monitoring with change detection. I did not read any site's terms of use; copyright and robots rules must be checked before ingestion. A test of GDELT (an open global news index) for Arabic Iraqi coverage was blocked by rate limiting and is not evaluated.

### 11.4 Design implications

Build a **source registry** (outlet, language, script, country, feed or sitemap, credibility class, state-control flag, licence status, crawl frequency) and an ingestion path (feed, sitemap, page diff) into an Arabic entity-extraction step; match extracted persons and companies against customers, owners and signatories with the Iraqi name pipeline; present results to analysts by crime category and stage (Wolfsberg: allegation, investigation, charge, conviction); store the article snapshot and hash in the evidence vault; keep a disinformation flag and a "reputational but not financial-crime" tag.

**AKR-TMS hooks**
- **Adverse-media source registry** with per-source credibility, state-control and licence fields and a crawl log; at least INA, Baghdad Today and 964 Media feeds and the sitemap-bearing outlets in the pilot set.
- **Page-monitor jobs** for Nazaha, judiciary, Ministry of Interior and CBI pages with change detection and snapshot storage.
- **Daily adverse-media run** over customers, owners, signatories and PEPs with a delta report (new hits only), as the CBI circular requires "daily screening".
- **Vendor media feeds** accepted through the same pipeline when a bank licenses one, with the vendor's source list stored.

---

## 12. Iraqi PEP coverage and Arabic name coverage: what to test before buying

No vendor publishes Iraqi coverage figures (OpenSanctions excepted), and I could not obtain extracts. The test below should be run by the bank (or by AKR-TMS in a pilot) on a trial extract, and its results kept as procurement evidence. It also serves as the CBI-facing justification for the data choice.

| # | Test | How | Pass signal |
|---|---|---|---|
| 1 | **Roster completeness** | Compile from official sources the Iraqi office-holders in each tier of the CBI PEP definition (Presidency, Council of Ministers and deputies, Council of Representatives members, Federal Supreme Court and Court of Cassation judges, governors and provincial councils, Kurdistan Region offices, independent commissions, CBI governor and board, senior army, police and security commanders, heads and boards of state-owned enterprises and state banks, party leaders); check each vendor for presence | Share found per tier; report separately for current and former holders |
| 2 | **Timeliness** | Take office changes of the last 12 months and measure days from appointment to vendor record | Days; the CBI says update "immediately" |
| 3 | **Arabic script** | For found records, check whether an Arabic-script name is stored and whether the Iraqi chain (given name, father, grandfather, tribe) is split in fields | Share with original script and with chain parts |
| 4 | **Relatives and close associates** | Take PEP families from public declarations or official biographies and check second-degree relatives | Share found; compare with the CBI requirement of second degree |
| 5 | **Retention after office** | Check whether former office-holders disappear after the vendor window | Share still present after 5 and 10 years |
| 6 | **Identifiers** | Birth year or date, mother's name, position and dates | Share with each field |
| 7 | **Record stability** | Stable record ids across updates; delisting events visible | Yes or no; DEL handling |
| 8 | **Sanctions-list extras** | Does the vendor carry the Iraqi national list and the UN 1518 Iraq list, with Arabic names? | Yes or no; row counts versus the official source (6,280 on 2026-10-06) |
| 9 | **Iraqi companies** | Take 100 Iraqi companies from the bank's own KYC (including Kurdistan) and check ownership data from Orbis, D&B and Sayari-type sources | Share with owners resolved to natural persons |
| 10 | **Adverse media** | Seed 50 known Iraqi corruption, fraud and smuggling cases from court and Integrity Commission announcements; check vendor recall and the source list | Recall by language of source |

**The CBI's own roster of positions.** The CBI's AML controls for e-payment service providers (20 April 2025, document page 13, read from the scanned Arabic and translated here) list the "high-risk senior position holders" to whom enhanced due diligence applies, "whether local or foreign" and including "their family members and persons related to them": the President of the Republic, deputies and advisers "and those of equivalent rank"; the Prime Minister, ministers and Council of Ministers members and advisers; heads of political parties; the Speaker and members of the Council of Representatives; the head and members of the Supreme Judicial Council; heads of independent bodies; ministry undersecretaries and inspectors; ambassadors, plenipotentiaries and diplomatic advisers; directors general; judges of all courts; senior security commanders and officers; and heads, managers and board members of charities and non-governmental organisations. This is wider than the "national-level positions" that vendors emphasise (it reaches directors general, inspectors, all judges and NGO boards, each "and those of equivalent rank"), so test 1 should use this roster as the sampling frame.

A vendor questionnaire (RFI) should ask in writing for: Iraqi PEP counts by tier and as of date; Arabic-script and Kurdish fields; source list for Iraq; update promise for office changes; relatives to second degree; retention rules; delisting handling; the Iraqi national list; Iraqi company and ownership coverage; and a free trial extract.

**AKR-TMS hooks**
- **Vendor evaluation workspace**: the ten tests above scripted, with an anonymised sample from the bank and result tables stored with the procurement record.
- **Own PEP register** (file 04 s.2n) as the authoritative layer on top of any vendor feed: the bank's confirmed PEP flags and relatives survive a vendor change.

---

## 13. Iraqi companies, Kurdistan and ownership data for the OFAC 50 percent rule

### 13.1 What exists in Iraq (from the MER and CBI documents in file 01; vendor pages checked)

- The Companies Registration Department (CRD) "has no mechanism to verify the accuracy" of beneficial-ownership information filed by legal persons; banks rely mainly on the customer's statement; FATF R.24 is rated **partially compliant**; BO information quality is "good among banks" (MER, file 01 s.2).
- **What the CRD does publish** (file 26 s.8b, from MER paras 685-730): since August 2023 its Baghdad portal shows company name, capital, legal form, **shareholder names and nationalities, number of shares each holds**, the authorised director and address; **beneficial-owner declarations are available to reporting entities for about USD 4 within one to three days**; CRD Controls No. 1 of 2023 (July 2023) require companies to declare beneficial owners; the CBI's May 2026 deck lists a beneficial-owner platform with 37,286 statements received. The CRD "does not include cases where ownership or control is through a chain ... indirectly through a group of persons or front persons". The first commercial ownership source for Iraqi companies is therefore the CRD itself, not Orbis or Sayari.
- The CBI's CDD instructions set the beneficial-owner threshold at **20 percent** (file 01 s.2b). **OFAC's rule aggregates all blocked owners to 50 percent**, so a bank that only records owners at 20 percent or more can miss an entity that is, for example, 30 percent owned by each of two blocked persons. The KYC model must hold the full shareholder table with percentages, not only owners above 20 percent.
- The AML/CFT Office's IQTFS platform offers a **company lookup across registries**, but it needs authentication (anonymous calls return HTTP 401) and I could not see which registries it covers.
- **Online registration**: the Ministry of Trade runs a "one-window system for registering Iraqi companies" (business.mot.gov.iq: apply online, pay fees online, collect certificates at the Companies Registration Department; the older address tasjeel.mot.gov.iq redirects to "BSR Iraq"). The portal home page shows no public company-search function; the CRD data used for ownership checks is the portal extract described above (file 26 s.8b).
- Kurdistan: the same AML law applies and the CBI supervises Kurdistan banks (file 01 s.9); company registration in the Region is understood to be handled by Kurdistan authorities, a separate source from Baghdad's registry (not verified: the Kurdistan Ministry of Trade and Industry site reset the connection in my probe); Kurdish-script names appear in documents (file 22 s.4).

### 13.2 What commercial suppliers offer for derived designations

| Supplier | Offer | Iraq statement |
|---|---|---|
| Dow Jones SCO | 54,000 to 56,000 entities owned or controlled by sanctioned parties | None |
| Kharon 50-Plus and Control | Majority owned in the aggregate; control without ownership; claims "more than 30%" are three or more degrees removed | None |
| Sayari | Automatic 50 percent attribution; 250+ jurisdictions | None; GCC interest shown by buyers |
| Moody's Orbis and Sanctions360 | 635 million entities; ownership trees; US, EU and UK lists | None |
| ComplyAdvantage | Entities owned or controlled by sanctioned persons | None |
| LSEG | UBO (Dun & Bradstreet), "narrative and implicit sanctions" | None |

Every supplier's ownership coverage is weakest where registries are opaque or closed, which is the situation in Iraq. The bank's own KYC data is therefore the first-line source for Iraqi entities, and commercial data helps mainly with the foreign holding companies in the chain (UAE, Turkey, Jordan, Lebanon, Cyprus and others).

### 13.3 The live exposure

OFAC's 2025-2026 actions with an Iraq nexus (file 05 s.1e) name a state-owned Iraqi company said to be controlled by Kata'ib Hizballah, Iranian oil disguised as Iraqi crude, and three Iraqi bankers. These are exactly the cases where **control** (not ownership) and **state ownership** matter, and where supplier datasets were not shown to cover Iraqi targets.

**AKR-TMS hooks**
- **Full shareholder table in KYC** for corporate customers (all holders with percentages, source, date), feeding an aggregation engine for OFAC (50 percent aggregate), EU and UK (more than 50 percent or control) rules.
- **Iraqi derived-designation set** maintained by the bank's compliance team: companies identified from OFAC press releases, IQTFS decisions and internal investigations, with the evidence stored (so the bank does not depend on a vendor for the Iraqi layer).
- **Registry evidence attachments** (Companies Registration Department portal extracts and BO declarations, Kurdistan registry extracts, chamber of commerce documents) kept in the vault and tied to ownership edges; a CRD lookup step in corporate onboarding and review (about USD 4 and one to three days per declaration, file 26 s.8b).
- **Foreign-chain enrichment** through an optional commercial ownership adapter (Orbis, Sayari, D&B, Kharon), clearly labelled as such.

---

## 14. Data quality complaints, legal cases and licensing costs

### 14.1 Data quality and legal evidence by supplier

| Supplier | Evidence of data-quality or legal problems | Grade |
|---|---|---|
| LSEG World-Check | 2017: admission that the Palestine Solidarity Campaign should never have been listed or associated with terrorism; compensation and apology to Finsbury Park Mosque; Vice News investigations (4 Feb 2016, 18 Oct 2017) of people wrongly listed as terrorists, one removed and compensated | Secondary (Wikipedia summary of press reports) |
| LSEG World-Check | Inconsistent figures across its own documents (65 versus 70 languages; "twice daily" 2019 versus "daily" 2026; no current record count) | Primary (LSEG pages) |
| Dow Jones | Inconsistent figures on its own pages (50 versus 80 authorities; 54,000 versus 56,000 SCO entities; 270 million versus 580 million ownership records) | Primary |
| Vendors in general (UK) | FCA 28 May 2026: SST "found errors or omissions in sanctions lists provided by third-party vendors, because of poor quality data and the transfer of data between systems, as well as delays or failures in updating"; firms had "limited understanding of how lists were ingested" | Primary (regulator) |
| Screening vendor (US) | OFAC MidFirst 2022: vendor notified the bank of designations after 14 days; full-base screening only monthly | Primary (regulator) |
| OpenSanctions | Mirror of the Iraqi freeze list shows 5,678 entities against 6,280 official rows; PEP set excludes people without birth or position dates | Primary (own count; their documentation) |
| Castellum.AI | "1,700+ watchlists" and "1,000+ watchlists" on different pages | Primary |
| Marketing claims | 93, 90, 95, 80, 82, 94 percent false-positive reductions (LexisNexis, Napier, FinScan, ComplyAdvantage, Castellum), "500 percent" (Eastnets): no common baseline | Primary (vendor pages) |

I found no independent customer review text for any supplier (review sites block automated access) and no regulator finding that names a specific data supplier.

### 14.2 Public price signals gathered in this file and in file 14

| Supplier | Signal | Value | Date and source |
|---|---|---|---|
| LexisNexis WorldCompliance | European Commission DG NEAR screening of beneficiaries | EUR 300,000 (final value) | 3 Dec 2024; EU Publications Office |
| LexisNexis (sanctions screening service) | FSCS batch service, 3 years | GBP 307,000 | 2019; TED 142139-2019 |
| LexisNexis Bridger Insight XG | Met Office, one year | GBP 8,500 | 1 May 2026 to 30 Apr 2027; Contracts Finder |
| Dow Jones RiskCenter with Factiva Pro | BaFin, 25 licences | EUR 277,380 | 14 Feb 2024; TED 97959-2024 |
| Bureau van Dijk (Moody's) | Eksportfinansiering Norge KYC screening system | EUR 300,000 | 20 Aug 2024; TED 499165-2024 |
| Moody's Analytics UK | National Wealth Fund sanctions due diligence, about 20 months | GBP 59,994 to GBP 99,990 | 29 Jan 2025; Contracts Finder |
| Sayari | German Federal Office for Foreign Affairs, 5 named users, 1 year plus two options | EUR 375,000 (estimated; basis unclear) | Aug and Sep 2025; TED 547615-2025, 628822-2025 |
| ComplyAdvantage | Public Starter Plan | USD 99 to USD 319 a month for 100 to 2,000 monitored entities (annual billing); overage 1.5 times | 2026 price page |
| OpenSanctions | API credits | EUR 0.10 down to EUR 0.03 per query; free 2,000 credits; flat-rate licence on quote | 2026 API page |
| FinScan | Third-party modelled estimate (unverified) | USD 500 to 8,000 a month; enterprise "often USD 100k-300k+ a year" | Spendbase |
| Swift Name Screening and Transaction Screening | Usage-based | not public | Swift FAQ (archive) |
| LSEG World-Check, Eastnets, Kharon, Castellum.AI | None public | not public | pages |

None of these figures covers integration, tuning, hardware or analyst time, and none is a bank-wide, many-million-record enterprise price. They show that public-sector and small-buyer prices sit in the tens to low hundreds of thousands of euros a year, and that ownership data (Moody's, Sayari) and news archives (Factiva) are priced per seat.

### 14.3 Licence terms to check in every contract

Internal use only versus use in a product sold to others (OpenSanctions "reseller/OEM"); permitted copies and on-premise storage; **retention and deletion of data after termination**; audit rights; update frequency and a service level for list changes (FCA good practice: "clear contractual and operational arrangements with vendors, setting out update frequencies, data quality standards, and escalation processes"); scope of full-base re-screening (OFAC MidFirst); data residency and where customer names are sent; whether decisions or queries are used to train shared models; restrictions on use for credit decisions (US FCRA disclaimers appear on LexisNexis, Moody's and ComplyAdvantage pages); and warranties.

**AKR-TMS hooks**
- **Data-licence register** in the product: for each external dataset, who holds the licence (bank or AKR-TMS), permitted uses, copies, retention and deletion rules, expiry date, and the residency of any hosted query.
- **Connector residency flag**: each data connector states whether it transmits names off-premises; hosted connectors are off by default in the on-premises profile.

---

## 15. Delivery formats: files, APIs, on-premises feeds

| Supplier | Files | API | On-premises feasible | Update cadence stated |
|---|---|---|---|---|
| LSEG World-Check | Data File (daily, weekly, monthly, full; "common file formats"; fact sheet 2019) | World-Check One API, On Demand API, Verify (stateless, AWS), Zero Footprint | **Yes through Data File** | "twice daily" (2019 sheet); "daily" (2026 FAQ) |
| Dow Jones | SFTP cloud storage, Pub/Sub feeds; custom content sets; single-file delivery | REST: Ad Hoc Search, Screening and Monitoring (10K+ names) | Yes through feeds | LPTS up to six times a day |
| LexisNexis | Firco List Management Services (downloads and validates lists daily) with the engine | Bridger XML and REST APIs; Compliance Link RESTful APIs | Engine on premise (Continuity, Bridger option) | "monitored for updates and validated each day" |
| Moody's | Not stated on pages read | Entity Verification API; Sanctions360 API; Orbis "available via a wide range of data integration options and applications" | Not stated | Not stated |
| ComplyAdvantage | None stated | Mesh API (SaaS) | **No** (SaaS-based) | Sanctions "within minutes of publication"; PEP "daily" |
| Sayari | None stated | API (batch up to 10,000 entities), Signal monitoring | Not stated | "continuously updating" |
| Kharon | Data files into existing screening software | API, GraphCast | Files yes | "regularly updated" |
| Castellum.AI | Files "updated every five minutes"; free static xlsx sample | API | Files likely | Every 5 minutes (claim) |
| OpenSanctions | FollowTheMoney JSON, CSV, plain names, Senzing; delta patches; hosted dataset index | Screening API; on-premise yente | **Yes** | Four times a day |
| UN | XML, HTML, PDF; e-mail subscription | none | Yes | Irregular; file regenerated often |
| OFAC | SDN, Consolidated, Advanced XML, CSV, custom datasets, delta archive; e-mail notices | Sanctions List Search tool | Yes | On each action |
| UK Sanctions List | One XML (21.9 MB; modified 2026-10-06 10:00 GMT) | none | Yes | Frequent |
| EU | XML via FSF (token) and data.europa.eu | none | Yes | On each act |
| Iraq (IQTFS) | none | Public JSON API (10 rows a page, 628 pages) | Yes (pull) | New decisions as they are issued |

**AKR-TMS hooks**
- **Connector contract** for every feed: full load, delta load, schema version, publication timestamp, hash, row count, and an alarm on schema change (the UN XML is already served under a "legacy" label, file 04 s.4a).
- **Pull schedule** with jitter and back-off for open APIs (the IQTFS list takes 628 requests; use incremental detection by decision id and `createdAt`, and a polite rate).

---

## 16. Comparison table

| Supplier | Data strengths | Iraq and Arabic evidence | On-premises route | Ownership and 50 percent rule | Public price signal | Main caveat | Role for AKR-TMS |
|---|---|---|---|---|---|---|---|
| **LSEG World-Check** | Researched sanctions, PEP, RCA, SOE, adverse media; 700+ lists; opt-in IEI, UBO, vessels | Named by the CBI; Iraqna IBTS agent (2020); no coverage figures; Arabic matching undescribed | **Data File** | "Narrative and implicit sanctions"; UBO via Dun & Bradstreet | None | Cloud-heavy; record count and cadence unclear; 2016-2017 false-labelling cases | **Must-support connector**; one source among several |
| **Dow Jones** | SCO (54-56k entities), PEP (22 categories), AME/SIP, Factiva (33,000+ sources, 33 languages), LPTS payment feed | None stated | Feeds (SFTP, Pub/Sub) | **SCO**; SOC (1 percent government ownership) | EUR 277,380 for 25 seats (BaFin) | Inconsistent figures; UBO via D&B; hosted tools | Ownership and media enrichment feed |
| **Moody's** | Orbis 635M entities; Grid 30M+ profiles, 200+ countries; Entity Verification API | None stated | Not stated | Orbis ownership trees; Sanctions360 (US, EU, UK lists) | EUR 300,000 (Norway); GBP 60-100k (UK NWF) | Registry access shrinking; no UN or national list named in Sanctions360 | Foreign-chain ownership enrichment |
| **ComplyAdvantage** | 60+ sanctions jurisdictions, daily PEP, 34 media categories, 24M+ entities | Non-Latin matching claimed; no Iraq | **No (SaaS)** | Owned or controlled entities (50 percent rule) | USD 99-319 per month for 100-2,000 entities | SaaS only; cross-customer learning loop | Pricing comparator; avoid for on-prem |
| **Sayari** | 500M+ companies, 3B+ links, 250+ jurisdictions, trade data | None; buyers ask for GCC | Not stated | **Automatic 50 percent attribution**; UBO resolution | EUR 375,000 estimated (5 seats, Germany) | Cloud; per-seat price; no Iraq | Specialist ownership add-on |
| **Kharon** | 50-Plus, Control, evasion networks, analyst research | None; ME banks as clients | Files | **50-Plus and Control** | None | No Iraq dataset advertised; OSINT control claims | Source of control relationships |
| **Castellum.AI** | 1,000-1,700 watchlists, 5-minute refresh, free sample | Arabic in language list | Files likely | Data on owned entities (not detailed) | None | US base; vendor-only claims | Free-sample idea; fast-refresh benchmark |
| **OpenSanctions** | 460+ sources; open; delta files; PEP position model | Iraqi freeze list (5,678 vs 6,280); 724 PEP tags | **Yes** | Limited (nested entities) | EUR 0.03-0.10 per query; flat rate on quote | PEP set thin; relatives not researched; Arabic weak | Optional data feed and benchmark |
| **LexisNexis** | 180 lists, 3.4M PEP profiles, 59-language media | None | Engine on premise | Firco ownership modules (not detailed) | EUR 300,000 (EC); GBP 8,500 a year (Met Office) | Opaque pricing; Arabic exact matching | Optional feed |
| **Official sources** (UN, OFAC, UK, EU, IQTFS) | Free, authoritative | Iraqi list is the only Arabic-only source | **Yes** | OFAC 50 percent guidance | Free | Parsing and refresh are the bank's job | **Load directly; first source** |

---

## 17. What it means for AKR-TMS

1. **Load official lists directly; use vendors for what official sources do not provide.** UN, OFAC, UK, EU and the IQTFS API are free and machine-readable; vendor data adds PEPs, relatives, adverse media and ownership. The CBI's 12-hour rule is best met by direct feeds with a freshness clock, because vendor documents show "twice daily" (2019) and "daily" (2026) cadences and the FCA found vendor list errors and delays.
2. **World-Check is a compliance requirement, not just a data choice.** AKR-TMS must connect to the World-Check Data File (the only route that keeps names in the bank) and the On Demand API, while remaining supplier-agnostic. The same adapter pattern covers Dow Jones feeds, LexisNexis and OpenSanctions.
3. **Nobody publishes Iraqi coverage.** Not one supplier states Iraqi PEP counts, Arabic or Kurdish source lists, or Iraqi company coverage; only OpenSanctions shows Iraq-linked numbers, and its copy of the national list differs from the official one. The ten-test evaluation (section 12) and the RFI are therefore part of the product: they turn a vendor claim into evidence the MLRO can show the CBI.
4. **The bank-owned PEP register is the authority.** CBI rules (relatives to the second degree, status after office until a documented downgrade, daily local media) exceed what any vendor window or relative list provides, and vendor retention rules (OpenSanctions 5 years for most positions) can contradict the CBI. AKR-TMS keeps its own PEP lifecycle and uses vendors as candidates.
5. **Ownership data is the biggest gap.** The CBI threshold is 20 percent while OFAC aggregates to 50 percent, Iraq's registry does not verify beneficial ownership, and every commercial ownership dataset is weakest in opaque registries. The KYC model must hold full shareholder tables, the bank must keep an Iraqi derived-designation set with evidence, and commercial data should be a labelled enrichment for foreign holding chains.
6. **Adverse media for Iraq is partly buildable.** Three Iraqi outlets publish RSS and five more publish sitemaps; the official sources that matter (Integrity Commission, judiciary, Interior, CBI) publish no feeds, so page monitoring with snapshots is needed. A source registry with credibility and state-control flags meets the CBI's daily local media requirement and Wolfsberg's source-credibility guidance, and complements any licensed feed.
7. **On-premises and data residency decide which products are usable.** Of the suppliers read, only World-Check (Data File), Dow Jones (feeds), Kharon (files), OpenSanctions (bulk and on-premise yente) and the official feeds can run without sending names out; ComplyAdvantage is SaaS-only, Sayari and Castellum are API-first, Moody's did not state. Connector residency flags and a data-licence register make this visible to bank leadership.
8. **Price transparency is a differentiator.** Only ComplyAdvantage (USD 99 to USD 319 a month for up to 2,000 entities) and OpenSanctions (EUR 0.03 to 0.10 per query) publish prices; public-sector buyers pay EUR 277,000 to EUR 375,000 for a few seats or one screening system. AKR-TMS should publish a clear price per monitored entity and a total-cost view including tuning and testing.

---

## 18. Derived requirements (VC-xx, continuing from file 14, which ends at VC-40)

| ID | Requirement | Source |
|---|---|---|
| VC-41 | World-Check connectors behind a licence flag: Data File (full and incremental) into the unified watch list, and On Demand API for ad-hoc and EDD lookups; both store record id, retrieval time and content hash | CBI controls name World-Check (file 01 s.2d); LSEG fact sheets (s.2) |
| VC-42 | Vendor cadence monitor: for every external file or API, record publication time, load time and activation time and compare with the CBI 12-hour limit; official lists are always loaded directly in addition to vendor copies | LSEG "twice daily" vs "daily" (s.2); FCA 2026 vendor list errors (s.14) |
| VC-43 | External-copy reconciliation: for each third-party copy of an official list (row counts, identifiers, last-changed), report differences to the MLRO; apply first to the OpenSanctions copy of the Iraqi list (5,678 versus 6,280) | OpenSanctions Iraq page versus IQTFS API, 2026-10-06 (s.9) |
| VC-44 | Delta consumer with its own snapshot store: handle ADD, MOD and DEL, id changes after merges, a full reload when more than 100 versions behind, and keep the content of deleted records | OpenSanctions delta documentation (s.9.1) |
| VC-45 | Profile snapshot per decision stored in the evidence vault: vendor, record id, retrieval time, source link, hash, so later vendor corrections or removals do not erase the basis of a decision | World-Check false-labelling cases (s.2.6, s.14); Wolfsberg negative news (file 04 s.2i) |
| VC-46 | Listing dispute and correction register linked to customer and vendor record: request, vendor response, outcome, dates | LSEG "Am I on World-Check?" process (s.2.3) |
| VC-47 | Bank-owned PEP register with lifecycle states (active, former, downgrade proposed, downgraded, deceased), relatives to the second degree and the CBI KPI report (a to g), independent of any vendor retention window | CBI PEP circular (file 01 s.2c); OpenSanctions retention windows (s.9.2); Wolfsberg PEP guidance (file 04 s.2n) |
| VC-48 | Candidate relative detection from family number, father, grandfather and tribe fields, with human confirmation, to extend vendor RCA data to the second degree | CBI second-degree rule; OpenSanctions "relatives only where documented" (s.9.2) |
| VC-49 | Vendor evaluation workspace scripting the ten Iraqi coverage tests (roster completeness, timeliness, Arabic script, relatives, retention, identifiers, record stability, Iraqi lists, Iraqi companies, adverse media) with results stored as procurement evidence | s.12 |
| VC-50 | Standard RFI for risk-data suppliers: Iraqi PEP counts by tier and date, Arabic and Kurdish fields, Iraq source list, update promise for office changes, relatives to second degree, retention, delisting, Iraqi lists, Iraqi company coverage, free trial extract | s.12 |
| VC-51 | Adverse-media source registry: outlet, language, script, country, feed or sitemap, credibility class, state-control flag, licence status, crawl frequency and log | CBI PEP circular; Wolfsberg negative news (s.11.1) |
| VC-52 | Ingestion of RSS (Iraqi News Agency, Baghdad Today, 964 Media), sitemaps (Shafaq, Rudaw, Kurdistan24, Alsumaria, Al-Mada) and page monitors with change detection for Nazaha, judiciary, Ministry of Interior and CBI pages; snapshot and hash for every article; terms-of-use and robots check before enabling a source | Probe results 2026-10-06 (s.11.3) |
| VC-53 | Daily adverse-media delta run over customers, owners, signatories and PEPs; analyst triage by crime category and stage; disinformation flag; "reputational only" tag; a hit is a trigger event for review | CBI PEP circular "daily screening"; Wolfsberg negative news (s.11) |
| VC-54 | Arabic entity extraction (persons and companies) for news text, feeding the Iraqi name pipeline for matching | s.11.4; file 22 |
| VC-55 | Corporate KYC holds the full shareholder table (all holders with percentages, source, date), not only owners at 20 percent or more, to support aggregation | CDD instructions 20 percent (file 01 s.2b); OFAC 50 percent aggregate rule (file 05 s.1e) |
| VC-56 | Iraqi derived-designation set maintained by the bank's compliance team with evidence (OFAC releases, IQTFS decisions, investigations), independent of vendor datasets | OFAC Iraq-nexus actions (file 05 s.1e); no vendor Iraq statement (s.13) |
| VC-57 | Ownership-edge provenance: each edge tagged with source class (bank KYC, registry document, commercial calculated, vendor research), confidence and date; the 50 percent engine reports which classes it used | Moody's "calculated ownership" caveat (s.4); Kharon Control (s.7); MER on unverified BO (s.13) |
| VC-58 | Pluggable ownership-data adapters (Orbis, Dun & Bradstreet, Sayari, Kharon, Dow Jones SCO) with no hard dependency and clear labelling of vendor-derived edges | s.4, s.6, s.7, s.3 |
| VC-59 | Registry evidence attachments (Companies Registration Department and Kurdistan registry extracts, chamber of commerce documents, Arabic licences) tied to ownership edges in the evidence vault | MER (file 01 s.2); OFAC ALICO 2024 (file 14 s.13) |
| VC-60 | Data-licence register: holder (bank or AKR-TMS), permitted uses, copies, retention and deletion on termination, audit rights, expiry, residency; flag the OpenSanctions OEM decision | s.14.3; OpenSanctions commercial FAQ (file 14 s.9.4) |
| VC-61 | Connector residency flag: each data connector states whether names leave the bank; hosted connectors (World-Check One and Verify, ComplyAdvantage, Sayari, Castellum, hosted OpenSanctions API) are off by default in the on-premises profile | s.2.2, s.5, s.6, s.8; file 01 s.8 |
| VC-62 | No cross-tenant learning from analyst decisions unless isolated by design and contract; document any vendor feedback loop | ComplyAdvantage "improves with every remediation decision" (s.5) |
| VC-63 | Test-before-buy pack: a synthetic Iraqi test set and a sample extract so banks can evaluate AKR-TMS and any supplier, mirroring free samples from Castellum.AI and OpenSanctions | s.8, s.9 |
| VC-64 | Published price per monitored entity per month and a total-cost view (licence, data, hosting, tuning, testing) for bank procurement, with the public price signals as comparators | s.14.2 |
| VC-65 | One-off check of Iraqi and Kurdistan company coverage in Dun & Bradstreet-based ownership data, because LSEG UBO and Dow Jones ownership both depend on it; result stored in procurement evidence | s.10.2 |

---

## 19. Sources

Pages read 2026-10-06 unless a date is given. Items marked (search result) were found in a search result and not opened. Review sites (Gartner Peer
Insights, G2, TrustRadius, Capterra) and swift.com block automated access; the web-search budget ran out during the work.

**LSEG World-Check**
- World-Check KYC screening overview: https://www.lseg.com/en/risk-intelligence/screening-solutions/world-check-kyc-screening
- World-Check On Demand: https://www.lseg.com/en/risk-intelligence/screening-solutions/world-check-kyc-screening/world-check-on-demand
- World-Check Verify: https://www.lseg.com/en/risk-intelligence/screening-solutions/world-check-verify
- World-Check Resolve: https://www.lseg.com/en/risk-intelligence/screening-solutions/world-check-kyc-screening/world-check-resolve
- World-Check One (FAQ, Data File, API, Zero Footprint links): https://www.lseg.com/en/risk-intelligence/screening-solutions/world-check-kyc-screening/one-kyc-verification
- About World-Check: https://www.lseg.com/en/risk-intelligence/screening-solutions/world-check-kyc-screening/about
- Am I on LSEG World-Check?: https://www.lseg.com/en/risk-intelligence/screening-solutions/world-check-kyc-screening/about/am-i-listed-on-world-check
- Data File fact sheet (Sept 2019): https://www.lseg.com/content/dam/risk-intelligence/en_us/documents/fact-sheets/world-check-data-file.pdf
- World-Check One API fact sheet (Sept 2019): https://www.lseg.com/content/dam/risk-intelligence/en_us/documents/fact-sheets/world-check-one-api.pdf
- Zero Footprint fact sheet: https://www.lseg.com/content/dam/risk-intelligence/en_us/documents/fact-sheets/wc1-zero-footprint-screening-api.pdf
- World-Check One brochure (Oct 2023): https://www.lseg.com/content/dam/risk-intelligence/en_us/documents/brochures/world-check-one-brochure.pdf
- LSEG and Iraqna IBTS, 10 Mar 2020: https://www.lseg.com/en/media-centre/press-releases/refinitiv/2020/march/refinitiv-partners-with-iraqna-ibts-to-enhance-regulatory-compliance-efficiency-in-iraq
- Wikipedia, World-Check (history and 2017 controversies; secondary): https://en.wikipedia.org/wiki/World-Check
- Eastnets and LSEG integration, 2 Jan 2026: https://cfotech.co.uk/story/eastnets-deepens-lseg-tie-up-for-real-time-risk-checks
- FinScan validated for World-Check On Demand, 22 Jul 2026: https://fintech.global/2026/07/22/finscan-validated-for-lseg-world-check-on-demand-access/

**Dow Jones**
- RiskCenter Advanced Screening and Monitoring: https://www.dowjones.com/business-intelligence/risk/products/advanced-screening-monitoring/
- Data feeds and APIs: https://www.dowjones.com/business-intelligence/risk/products/data-feeds-apis/
- RiskCenter Financial Crime: https://www.dowjones.com/business-intelligence/risk/products/financial-crime/
- Developer platform compliance datasets: https://developer.dowjones.com/datasets/datasets-details-compliance
- TED notice 97959-2024 (BaFin): https://ted.europa.eu/en/notice/97959-2024/pdf

**Moody's**
- KYC and compliance overview: https://www.moodys.com/web/en/us/kyc.html
- Grid: https://www.moodys.com/web/en/us/kyc/products/grid.html
- Orbis for Compliance: https://www.moodys.com/web/en/us/kyc/products/orbis-compliance.html and https://www.moodys.com/web/en/us/capabilities/company-reference-data/orbis.html
- Sanctions360: https://www.moodys.com/web/en/us/kyc/products/sanctions360.html
- Ultimate beneficial owners: https://www.moodys.com/web/en/us/kyc/solutions/ultimate-beneficial-owners.html
- Entity ownership and verification: https://www.moodys.com/web/en/us/kyc/solutions/entity-ownership.html
- TED notice 499165-2024 (Eksportfinansiering Norge, Bureau van Dijk): https://ted.europa.eu/en/notice/499165-2024/pdf
- UK Contracts Finder, National Wealth Fund, Moody's Analytics UK: https://www.contractsfinder.service.gov.uk/Published/Notice/releases/32d7ec6e-d27f-4ebf-9dda-b6986281fb54.json

**ComplyAdvantage**
- Home: https://complyadvantage.com/ ; risk intelligence: https://complyadvantage.com/fincrime-risk-intelligence/ ; PEP: https://complyadvantage.com/fincrime-risk-intelligence/politically-exposed-persons-screening/ ; sanctions: https://complyadvantage.com/fincrime-risk-intelligence/sanctions-watchlists-screening/ ; payment screening: https://complyadvantage.com/mesh/payment-screening/ ; pricing: https://complyadvantage.com/pricing/ and https://complyadvantage.com/starter-plan/

**Sayari**
- Home: https://sayari.com/ ; KYC and beneficial ownership: https://sayari.com/enterprise/kyc-beneficial-ownership/ ; API: https://sayari.com/platform/api/ ; US Treasury contract: https://sayari.com/sayari-awarded-8-5-million-contract-with-u-s-treasury-department-for-financial-crime-analytics/
- TED notices 547615-2025 and 628822-2025 (German Federal Office for Foreign Affairs), read through the TED API: https://ted.europa.eu/en/notice/628822-2025/pdf

**Kharon**
- Home: https://www.kharon.com/ ; 50-Plus: https://www.kharon.com/data/50-plus ; Control: https://www.kharon.com/data/control ; banking: https://www.kharon.com/solutions/industries/banking ; API: https://www.kharon.com/products/api ; Core: https://www.kharon.com/products/kharon-core ; partnerships: https://www.kharon.com/partnerships

**Castellum.AI**
- Home: https://www.castellum.ai/ ; Sanctions and PEPs screening: https://www.castellum.ai/products/sanctions-and-peps-screening ; free sanctions data: https://www.castellum.ai/free-sanctions-data ; pricing: https://www.castellum.ai/pricing ; API: https://www.castellum.ai/api-integration

**OpenSanctions**
- Licensing: https://www.opensanctions.org/licensing/ ; screening licence: https://www.opensanctions.org/licensing/screening/ ; API and prices: https://www.opensanctions.org/api/
- PEP coverage: https://www.opensanctions.org/docs/coverage/pep/ ; PEP methodology: https://www.opensanctions.org/docs/coverage/pep/methodology/ ; delta files: https://www.opensanctions.org/docs/bulk/delta/ ; Iraq: https://www.opensanctions.org/countries/iq/ ; commercial FAQ: https://www.opensanctions.org/docs/commercial/faq/

**LexisNexis**
- Bridger Insight XG: https://risk.lexisnexis.com/products/bridger-insight-xg ; Firco Compliance Link brochure and Firco Insight brochure (see file 14 sources); EU Publications Office DG NEAR record: https://op.europa.eu/en/web/public-procurement/procurement-details/-/procurement/142bad45-7789-4f6d-8c73-8565cea0ab4e ; TED 710751-2022 (Portugal, attribution unverified): https://ted.europa.eu/en/notice/710751-2022/pdf

**Official lists and Iraqi sources**
- OFAC Sanctions List Service: https://ofac.treasury.gov/sanctions-list-service
- UK Sanctions List XML: https://sanctionslist.fcdo.gov.uk/docs/UK-Sanctions-List.xml
- IQTFS public API: https://api.aml.iq/api/localsanctionslist/getall/1 ; company lookup (401 for anonymous): https://api.aml.iq/api/companylookup/registries
- Iraqi media probe (homepages, feeds, sitemaps): https://ina.iq/ ; https://baghdadtoday.news/ ; https://964media.com/ ; https://shafaq.com/ar ; https://www.rudaw.net/arabic ; https://www.kurdistan24.net/ar ; https://www.alsumaria.tv/ ; https://almadapaper.net/ ; https://nazaha.iq/ ; https://sjc.iq/ ; https://moi.gov.iq/ ; https://cbi.iq/ ; https://gov.krd/
- Ministry of Trade one-window company registration: https://business.mot.gov.iq ; Ministry of Trade home: https://www.mot.gov.iq/
- FCA, "Sanctions systems and controls in our firms: our findings", 28 May 2026: https://www.fca.org.uk/publications/good-and-poor-practice/sanctions-systems-and-controls-our-firms-our-findings
- OFAC enforcement release, MidFirst Bank, 21 Jul 2022: https://ofac.treasury.gov/civil-penalties-and-enforcement-information/2022-enforcement-information
- Chartis watchlist monitoring reports page: https://www.chartis-research.com/financial-crime/watchlist-monitoring
- TED API: https://api.ted.europa.eu/v3/notices/search

