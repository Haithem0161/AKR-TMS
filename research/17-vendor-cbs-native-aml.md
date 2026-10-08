# 17 — Vendors: AML modules bundled with core banking systems (CBS-native AML) and how each CBS stores data

Status: done (2026-10-06)

Part 4a of 4 of the vendor research. Companion file: 18-vendor-mena-iraq-market.md (what Iraqi banks actually run, tenders,
regional peers, market map, competitor table). Requirement IDs use the prefix VD-xx and are numbered across both files
(VD-01 onward; this file takes the first block).

Why this file exists: AKR-TMS reads each bank's core banking system (CBS) database through a read-only connection and maps
the CBS schema to a canonical model. The CBS vendor therefore matters twice: (1) the CBS often ships its own AML or
screening module, which AKR-TMS will meet as an incumbent inside the bank; (2) the CBS database design (engine, table
naming, how balances and history are stored) decides how hard the mapping layer is. Database-level integration patterns
(CDC, replicas, read-only roles) are covered in file 25; this file only records vendor facts.

## Contents

1. Method, evidence grading and what counts as "CBS-native AML"
2. Temenos (Transact, Financial Crime Mitigation, Payments Hub, Data Hub)
3. Path Solutions, now Azentio (iMAL Islamic core banking; Financial Crime Management and AMLOCK)
4. ICS Financial Systems (ICS BANKS and ICS BANKS Islamic)
5. Oracle FLEXCUBE Universal Banking (and Oracle FCCM)
6. Infosys Finacle
7. Finastra (Misys Equation, Fusionbanking, Essence; Fusion Total Screening; Compliance as a Service)
8. Intellect Design Arena and other CBS vendors present in the region
9. BML Istisharat (ICBS), Capital Banking Solutions (CapitalBanker, CapitalCompliance) and other core systems seen in Iraq
10. Typical weaknesses of CBS-native AML compared with specialist systems
11. Database comparison table: engine, schema quirks, what AKR-TMS must handle
12. What it means for AKR-TMS
13. Derived requirements (VD-01 to VD-15; the market side VD-16 to VD-40 is in file 18)
14. Sources

---

## 1. Method, evidence grading and what counts as "CBS-native AML"

- Search budget note: the shared WebSearch budget of the session ran out partway through this work (200 of 200 calls). The remaining
  research used direct reads of vendor sites (sitemaps and press pages), vendor documentation, the Wayback Machine for pages that block
  automated access (FinTech Futures, Iraq Business News) and the existing research files. Where a fact could only be seen as a
  search-result summary, it is marked [B-].
- Grades: **[A]** vendor or bank primary source opened and read; **[B]** reputable trade press opened and read (often a restated vendor release);
  **[B-]** seen only as a search summary or a restated claim; **[C]** weak (vendor sales claim with no number or customer, directory page);
  **[K]** practitioner knowledge not confirmed in a document this session (to confirm in each bank's discovery phase; the same convention
  as file 25).
- "CBS-native AML" here means three different things, and the difference matters for what AKR-TMS replaces:
  1. **Own engine in the same product family** (Temenos Financial Crime Mitigation, Azentio AMLOCK next to iMAL, Oracle FCCM next to FLEXCUBE): the AML
     product is sold by the same vendor, usually with a pre-built feed from that vendor's CBS.
  2. **Marketplace or partner AML** (Infosys Finacle App Center, Finastra Fusion Total Screening plus partners, Temenos marketplace): the CBS vendor
     lists someone else's AML product and certifies an interface.
  3. **Thin built-in controls**: customer risk class fields, blacklists, a screening hook, regulatory reports (ICS BANKS "Regulatory Reports", BML ICBS "local authorities requirements").
- Iraq fact that frames everything: banks run a CBS first and an AML tool second. Iraqi banks' CBS choice is dominated by regional vendors
  (ICS Financial Systems, BML Istisharat, Capital Banking Solutions, Path Solutions now Azentio) plus Misys/Finastra and Oracle at the state banks and Temenos at a few
  (file 18 sections 3 to 7). The AML tool is a separate purchase, so AKR-TMS meets the CBS vendor as a data source more often than as an AML competitor.

## 2. Temenos (Transact, Financial Crime Mitigation, Payments Hub, Data Hub)

### 2.1 What the bundled AML does

| Item | Fact | Grade, source |
|---|---|---|
| Product family | "Temenos Financial Crime Mitigation (FCM)" covers "suspicious activity detection", "anti-money laundering" (customer monitoring, real-time detection, peer-based risk comparison), "watchlist screening" and "know your customer plus" (standardised, dynamic KYC risk calculation). The press releases add PEP matching, AML transaction monitoring, payment fraud mitigation and KYC customer risk assessment. | [A] TEM-FCM; TEM-FCM-SAAS |
| Detection method | "Based on key metrics such as income, spending patterns, and transaction types, a unique financial fingerprint or 'DNA' is created for each customer. Combined with pre-built typologies and self-learning algorithms, it enables financial institutions to distinguish between usual and unusual behavior." The vendor calls its decision engine "patented Explainable AI (XAI)". | [A] TEM-FCM; TEM-FCM-SAAS |
| Footprint | "Used by over 300 banks from global tier ones such as UBS to regional banks, neobanks and digital-only banks" (12 Oct 2022). | [A] vendor claim, TEM-FCM-SAAS |
| Deployment | On-premise, cloud or SaaS ("FCM-as-a-Service" on Temenos Banking Cloud since 12 Oct 2022). "pre-integrated with Temenos core banking platform or with an API catalogue for fast connection to any other third-party core." | [A] TEM-FCM-SAAS |
| AI layer | FCM AI Agent, generally available 21 May 2025: evaluates screening alerts automatically; the vendor claims "less than 2% false positives rates" against an "industry average ... for customer screening [of] 5-8%"; co-created with "a European tier-1 bank"; "deployment agnostic" (on premises, public or private cloud). A later vendor blog claims "up to 40% fewer false positives" for banks using FCM. | [A] vendor claims, TEM-FCM-AI; TEM-FCM-BLOG |
| Marketplace | Temenos lists third-party AML on its solution-provider marketplace, including LexisNexis InstantID/TrueID and "6 Sigma Financial Consultancy Ltd - Anti Money Laundering (AML)", described as "designed for banks and financial institutions operating across the MENA, Levant, Africa, Europe, and Turkey regions" (its own listing counts seven behavioural transaction rules). | [A] TEM-6SIGMA |
| Arabic | No Arabic-script name matching or Arabic-language statement appears on the FCM pages, press releases or blog read (the word "Arabic" does not occur; absence is not proof). | [A] grep of the pages above |

### 2.2 Temenos in Iraq (what is evidenced)

| Bank | What | Date | Grade, source |
|---|---|---|---|
| National Bank of Iraq (NBI; majority-owned by Capital Bank of Jordan) | Temenos Transact core banking plus Temenos Payments Hub replace "legacy systems"; migration "in under 12 months"; delivery partner ITSS; "over 100,000 domestic and international payment transactions in the first 15 days with a 99% straight-through processing rate". Iraq Business News (11 Jan 2024, citing the Capital Bank of Jordan) adds that trade finance runs on software from Finastra. The release names no AML, screening or FCM component. | Press release 25 Apr 2024 | [A] TEM-NBI; [B] IBN-NBI |
| ZainCash (wallet operator, March Holding) | Temenos core banking plus Temenos Payments Hub live "in March 2024" with Prema Consulting as implementation partner; "the Temenos core banking platform connected to the middleware TIBCO"; migrated "data and balances from 700,000 wallets"; 100,000+ transactions a day; CEO: a focus area is "compliance, risk, financial crime and management of AML all from within the Temenos platform" (an aim, not a deployment). Integration with the CBI is named in the release. | Press release 23 May 2024 | [A] TEM-ZC; TEM-ZC-SS |
| Banque Libano-Francaise (Iraq branch network) | T24 upgrade to R17 in July 2018, said to be the first in the Middle East to complete R17; the bank opened its first Iraqi branch in October 2012. | July 2018 | [B-] FF-BLF |

No Temenos FCM customer in Iraq was found in any Temenos or other source. The NBI and ZainCash releases are about core banking and payments.
NBI's previous system was BML Istisharat's ICBS (section 9): a 2016 FinTech Futures article lists NBI among ICBS users, and NBI's own 2023 annual report names "ICBS" as its activated banking system and lists an "ICBS System Renewal 2023" contract with Istisharat [A, NBI-AR-2023]. NBI's AML tool is not Temenos FCM: its 2023 and 2024 reports name "AML Sirion" (Siron by name) launched in early 2024 next to the Temenos core [A, NBI-AR-2023; NBI-AR-2024].

### 2.3 How Transact stores data (what AKR-TMS meets)

- Database engines: the Temenos marketplace pages for Cintra describe "Oracle database workloads that underpin Temenos" and a "roadmap to lower
  cost temenos certified Postgres database platforms" ("refactor to Postgres"). So Oracle is the common engine today and PostgreSQL is a certified target
  [A, TEM-CINTRA]. File 25 section 11.1 records, from community posts, the older jBASE multi-value store, Db2, and the Oracle layout in which each
  Transact "file" is a table with a key column (`RECID`, a reserved word in Oracle) and the whole record as one XML column [S, file 25]. Both facts
  matter for a read-only mapping layer: the data is not relational columns; the XML must be parsed outside the CBS (file 25 section 5.3).
- Event streaming and a ready-made read store: "Temenos Data Hub" gives "real-time event streaming, data engineering, and purpose-built data stores", with an
  "operational data store" and a "snapshot data store" outside the core, "so the Core can thrive" [A, TEM-TDH]. For a bank that already pays for it, the
  Data Hub ODS is the natural read-only source for AKR-TMS (file 25 pattern F), instead of the XML tables.
- Documentation: Temenos product documentation (docs.temenos.com) is behind a customer or partner login, so the Transact data dictionary could not
  be read for this research. Treat Transact table details as [K] until a bank's DBA supplies the dictionary [A, login page].

### 2.4 Weaknesses compared with a specialist (evidence and reasoning)

- The integration is deepest with the Temenos core: other cores connect through an "API catalogue", while AKR-TMS reads the database (file 25). For a Temenos bank
  that already owns FCM, the switching cost argument is licence and process re-work, not technology.
- Behaviour model is the vendor's "DNA" plus typologies, tuned by the vendor's design partners (European tier-1 bank for the AI agent). Nothing public
  shows Iraqi typologies (hawala pass-through, import-invoice abuse, salary-localisation) or the CBI's 56 scenarios as configuration.
- The headline numbers are vendor claims from vendor-run programmes ("Temenos CEO Navigator"). No independent validation was found. Treat all of them as [C].
- Price and delivery: the vendor's own blog says traditional solutions bring "Lengthy, resource-intensive implementations; High operational and licensing costs; Ongoing dependence on IT and specialist teams". The SaaS
  option is the vendor's answer to cost, but a SaaS offering on a public cloud runs against the on-premises default in Iraq (file 01 section 8).

AKR-TMS hooks:
- Mapping profile "Temenos Transact (Oracle/XML)": fetch XML text, parse in Rust outside the CBS; support PostgreSQL-hosted Transact; support "Temenos Data Hub ODS" as an alternative source adapter.
- Keep a documented data dictionary per bank (the bank's DBA exports it; Temenos docs are login-only).
- Compete with FCM by shipping the CBI scenarios and Arabic matching as visible configuration, not by out-detecting a vendor on generic typologies.

## 3. Path Solutions, now Azentio (iMAL Islamic core banking; Financial Crime Management and AMLOCK)

### 3.1 Who owns it now

Path Solutions (Kuwait; Islamic core banking vendor) is now part of Azentio Software, headquartered in Singapore and "owned by funds advised by Apax Partners"
[A, AZ-KIB-IMAL]. Evidence of the merger: Path Solutions' own site already carried "Azentio" news items on 21 Dec 2021 and described Path as "serving 150+
Islamic financial institutions across 40 countries" with Iraq in its country map [A, PATH-2021]; by 10 May 2023 a Wayback snapshot of path-solutions.com
resolves to azentio.com, where iMAL sits under "ONEBanking" [A, PATH-2023]. In Iraqi tenders and bank statements the product is still called "iMAL" and
the vendor "Path Solutions", so AKR-TMS should treat "Path" and "Azentio" as one counterparty. The path-solutions.com domain does not answer from the research sandbox
(connection refused), so older Path pages were read through the Wayback Machine.

### 3.2 What the bundled AML does

iMAL itself is a core banking and Islamic finance product: core, financing, profit calculation (PCS), treasury, investment, trade finance, microfinance and
digital channels; "iMAL is the first and only software certified by [AAOIFI]" (vendor claim) [A, AZ-IMAL; AZ-TADHAMON]. The 2017 review lists "risk management and FATCA"
modules for a greenfield Iraqi bank (Al-Rajih) and "Compliance with Basel II/III & AML" as a feature line [B, FF-ALRAJIH; FIT-IMAL] but no AML engine inside iMAL.
The AML engine is a separate Azentio product sold beside it:

| Item | Fact | Grade, source |
|---|---|---|
| Product names | "Azentio Financial Crime Management & Anti-Fraud System" (FCM suite; brochure) and "AMLOCK" (the name used in 2025 releases). Modules: prospect onboarding due diligence, KYC, risk categorisation, transaction monitoring, case management, compliance reporting, remittance monitoring, anti-fraud, analytics. Editions: "FCM Lite" for "small and medium institutions" (CTR and STR filing to the FIU), "FCM Enterprise", "FCM Anti-Fraud", "FCM Analytics". | [A] AZ-FCM-PDF |
| Scale claims | "100+ implementations", "Extensive implementation experience with 20+ core banking systems", "18+ years of AML implementation", "Compliant with AML regulations in 35+ countries", largest site "80+ million daily transactions" and "550+ million customer accounts", largest multi-country roll-out "25+ countries". | [A] vendor claims, AZ-FCM-PDF |
| Detection | "Comprehensive library of 150+ scenarios" (brochure) and "over 400 ready-to-use rules for banking, lending, insurance, and fintech" (May 2025); profile-deviation engine; flexible rule builder; "Rules simulation to provide 'What-If' analysis"; graphical link analysis; AI models for false-positive reduction and "alert closure probability". | [A] AZ-FCM-PDF; AZ-AMLOCK-2025 |
| Screening | "Sanctions screening against third-party lists (World-Check, Dow Jones, Accuity World Compliance)"; vector-based sanctions matching; multilingual investigative narratives in "Arabic and English" (Saudi page); SWIFT message library "with local format support". | [A] AZ-FCM-PDF; AZ-SAUDI; AZ-AMLOCK-2025 |
| Claims about effect | "reducing false positives by up to 40%", "slash compliance costs by at least 20%", "every institution using AMLOCK has maintained a flawless record of zero AML-related regulatory fines" (a marketing claim; the Saudi page says "up to 30%" and "up to 10%"). | [C] AZ-AMLOCK-2025; AZ-SAUDI |
| Analyst standing | Celent "Noteworthy Solution" in "Financial Crime Compliance Technology: AML Transaction Monitoring Edition 2023" (the platform "used by over 100 financial institutions"; Celent: "price performance, machine learning-enabled detection, a large rules library, network analysis, and regulatory reporting capabilities across many jurisdictions"); QKS Group SPARK Matrix AML "Leader" (vendor statement). | [B] AZ-CELENT; [C] AZ-SAUDI |
| Integration | "Integration with any data source"; "Microservices architecture"; the vendor's CBS-neutral claim is "20+ core banking systems", which makes it the one CBS-vendor AML in this file that is also a stand-alone product. | [A] AZ-FCM-PDF |

### 3.3 Path/Azentio in Iraq (what is evidenced)

| Bank | What | Date | Grade, source |
|---|---|---|---|
| Cihan Bank (Erbil, Kurdistan Region) | Path's first Iraqi customer: "signed in 2009, with two more signings the following year and a succession of deals since then" | 2009 | [B] FF-AMEEN |
| International Development Bank for Investment and Finance (IDB Iraq) | iMAL R14 upgrade, go-live 30 Sep 2018; 15 branches; the release speaks of "compliance and risk management functionalities", no AML module | 2018 | [B] FIT-IDB |
| Zain Iraq Islamic Bank for Investment and Finance | Selected iMAL; evaluated "Temenos, Misys, Oracle FS, ICS Financial Systems and Capital Banking Solutions"; modules include mobile/internet banking, trade finance, treasury, BI, risk management, FATCA | 27 Oct 2017 | [A] FFNEWS-ZAIN |
| Al-Rajih Islamic Bank for Investment and Financing | Greenfield bank (licence spring 2017); chose iMAL "ahead of ... ICS Financial Systems"; modules core, clearing and settlement, credit facilities, risk management and FATCA, trade finance, payroll, Islamic invest, profit calculation, e-cheque clearing, channels, "central bank reporting", ATM management | 13 Dec 2017 | [B] FF-ALRAJIH |
| Ameen Al-Iraq Islamic Bank for Investment and Finance (Baghdad) | Signed iMAL (Baghdad), 18 Apr 2019 [B]. **Also runs the iMAL vendor's AML engine, shown in four consecutive annual reports.** The bank's 2021 to 2024 reports list "IMAL" as the "electronic banking system" and an "AML" system among its electronic systems. In each report the external auditor's paragraph on the AML and terrorism-financing reporting department says the bank owns electronic systems specialised in AML: an "AML system supplied by path solutions" (2021, 2022, 2023) or "supplied by Azento Software Middle East FZ LLC" (2024; sic, Azentio), and a "Word check" system "supplied by Refinitiv". It adds that "all the above systems are linked to the banking system", that the electronic system "includes the minimum scenarios required by the Central Bank of Iraq, amounting to 25 scenarios", that blacklists are updated "periodically and automatically", that customers are risk-classified by the AML system, and that some operations are flagged by the AML system, the daily movements are reviewed by the AML reporting department and alerts are closed "according to the justifications relating to the customers". Quotes are the researcher's translation from Arabic. | 2019 signing; AML in the 2021 report and in each later report | [B] FF-AMEEN; [A] AMEEN-AR-2021; AMEEN-AR-2022; AMEEN-AR-2023; AMEEN-AR-2024; AMEEN-REPORTS |
| Kurdistan International Islamic Bank for Investment and Development (KIB/KIIB) | iMAL adopted (release 25 Jan 2024; signing in Erbil, the caption reads "December 14, 2024", probably a typo for 2023), with "Azentio's Financial Crime Management and Anti-Money Laundering system ... integrated with iMAL ... KYC, sanction screening, holistic risk scoring and comprehensive case management". On 28 Feb 2025 Azentio announced "successful deployment of our AMLOCK" at KIIB, "fully integrated across all of Kurdistan International Islamic Bank's branches". KIB was one of the eight banks barred from dollar auctions on 4 Feb 2024 (file 01 section 4). | 2024-2025 | [A] AZ-KIB-IMAL; AZ-KIIB-AML |

Two Iraqi banks are therefore evidenced as running the AML engine of their iMAL vendor: **Ameen Al-Iraq** (in place by the 2021 report, about two and a half years after its iMAL signing) and **KIIB** (FCM named in the Jan 2024 iMAL release, AMLOCK "deployed" in Feb 2025). These are the **only Azentio/Path AML deployments confirmed in production at Iraqi banks found during this research** (the other iMAL wins above are core banking only). The only other CBS-vendor AML engine found inside Iraqi banks is Capital Banking Solutions' CapitalCompliance (section 9.2); the other named Iraqi AML tools are
from non-CBS vendors (file 18 section 8). Two adoption patterns appear: an AML engine bought with the core (KIIB, a bank cut off from dollars in Feb 2024 that announced AMLOCK within about a year) and an engine added after the core was live (Ameen). In both cases the CBS vendor is already inside the bank and can pre-integrate: a pattern AKR-TMS should expect.

One caution. Two banks audited by different firms, Ameen Al-Iraq (reports 2021 to 2024) and Trans Iraq Bank (reports 2022 to 2024, file 18 section 4), carry the same sentence: the system "includes the minimum scenarios required by the Central Bank of Iraq, amounting to 25 scenarios". The MENAFATF report says that the CBI's standardised set grew for banks "to 56" after the CBI's sectoral assessment, and that it was "34" before [A, MER para 566]. The wording follows a standard four-point AML statement that listed banks make in the annual report under the Securities Commission's disclosure requirements (item 26 in National Bank of Iraq's reports: the bank is committed to "implementing the electronic system, which includes the minimum required scenarios", automatic blacklist updates, customer risk classification and transaction monitoring "based on the established scenarios" [A, NBI-AR-2024]); the two auditors add the number. How the 25 "minimum" scenarios relate to the 56 is not explained in any source found: the 25 may be a minimum inside the larger set or an older list. Trans Iraq Bank goes beyond the minimum: its auditor writes that "a contract was concluded with PIO-TECH to implement (81) scenarios inside the system" (2023 report) and "(96)" (2024). So "we carry the CBI scenarios" is not a differentiator against Azentio at Ameen; proven coverage of the current pack, with the version and the circular it came from, is.

### 3.4 How iMAL stores data

- 2017 review: "multi-tier architecture using the latest JAVA technology with either **Oracle or SAP Sybase** as core database servers" [B, FIT-IMAL; 2017, so confirm per bank whether Sybase
  ASE is still in use]. Release R14.5 is "cloud-enabled, fully scalable with an n-tier architecture" and "API-first and cloud-ready" [A, AZ-TADHAMON; AZ-IMAL]. Old iMAL digital releases 12.x and 13.x are end of life [A, AZ-IMAL].
- A Sybase ASE source is a real possibility for older Iraqi iMAL banks (Cihan and IDB went live before 2018). The Rust mapping layer needs a TDS-capable driver or a bank-side replica;
  file 25 section 3.5 covers other engines in general. Table and column names for iMAL were not found in any public document: [K].

### 3.5 Weaknesses compared with a specialist

- The strongest evidence is positive for Azentio: the brochure shows a mature rule library, link analysis and screening against the same lists the CBI names (World-Check). The gaps are Iraq-specific, not general.
- No Iraqi regulatory pack is shown in the vendor's own material: the brochure and Saudi page name SAFIU reporting for Saudi Arabia and "CTR, STR, etc." generically, but nothing public from the vendor mentions goAML for the Iraqi FIU, the CBI 56 scenarios or the CBI behavioural indicators. The vendor-side silence is partly contradicted by Ameen's auditor, who says the bank's AML system carries the CBI minimum scenarios (stated as 25, not 56; section 3.3). goAML packaging is still not evidenced for Azentio in Iraq.
- Arabic is present as narrative language ("supports Arabic and English investigative narratives", Saudi page) rather than as documented Arabic name matching with patronymic chains (file 22).
- Footprint in Iraq is small for AML (two named banks, Ameen Al-Iraq and KIIB) and in Iraqi core banking it competes with a larger regional vendor (ICS, section 4): in 2017 ICS BANKS won the greenfield Islamic banks Al-Arabiya, Al-Thiqa and International Islamic Bank, while iMAL won Zain Iraq Islamic Bank and Al-Rajih (file 18 section 5).

AKR-TMS hooks:
- Mapping profile "iMAL (Oracle or Sybase ASE)"; plan a Sybase-capable adapter or insist on a bank-supplied Oracle/PostgreSQL replica.
- Position against Azentio at KIIB- and Ameen-type banks (iMAL plus the vendor's AML) as the tool that carries the full current CBI scenario pack with a coverage report, goAML packaging and Arabic matching, and that can read iMAL read-only without a vendor project. Do not claim "the CBI scenarios" as unique: Ameen's auditor says its Azentio system already carries the CBI minimum set.
- Add a scenario-coverage report to the pilot: list the scenarios the bank runs today (Ameen's auditor says 25; the MER says the CBI pack is 56), map them to the CBI pack by number and version, and show the gaps.
- Ship goAML export and CBI scenarios as named features so a CBS vendor's "integrated" AML does not look more compliant on the day of a CBI review.

## 4. ICS Financial Systems (ICS BANKS and ICS BANKS Islamic)

### 4.1 Why it is the most important CBS vendor for AKR-TMS in Iraq

ICSFS (Amman centre of excellence; offices in the UK, Jordan, UAE, Iraq, Saudi Arabia, Palestine, Sudan, Nigeria and Lebanon [A, ICS-MKT]) is the incumbent core banking vendor with the largest
Iraqi customer count in any source found:

| Date | Claim by ICSFS | Source |
|---|---|---|
| Nov 2015 | "We have been live and fully operated in the Iraqi Market since March 2001"; "most of the Iraqi and international banks in Iraq are already banking on ICS BANKS" | [A] ICS-MANSOUR |
| Mar 2017 | "around 20 users" in Iraq (FinTech Futures); "over 20 customers" including Al-Arabiya Islamic Bank, Al Janoob Islamic Bank, Bank of Baghdad, Taawen Islamic Bank, Noor Al Iraq Islamic Bank (Oct 2017) | [B] FF-IIB-MAR17; FF-IIB-OCT17 |
| 3 Oct 2018 | "the Iraqi Banking Software Market, where ICSFS is dominant, with 23 banks that are long-term users" (SWIFT user group meeting for Iraqi banks, Amman) | [A] ICS-SWIFT-UG |
| Dec 2018 | "in production and live in Iraq for over 15 years, with over 25 banks of Iraq's local and foreign banks using ICS BANKS" | [A] ICS-HAMMURABI |
| 21 Jun 2022 | "over 30 active customers"; user-group summit in Baghdad with 120+ attendees "representing more than 40 banks and financial institutions" | [A] ICS-SUMMIT22 |
| 21 May 2023 | "over 30 active customers"; Open Day with Oracle at the Babylon Rotana, "over 90 guests representing most of the banks and financial institutions in Iraq" | [A] ICS-SUMMIT23 |

These are vendor claims (the count of "banks" includes small and non-bank institutions; the CBI counts 67 banks in late 2025 and the MER 74-76 in 2022, files 02 and 18), but they are consistent over seven years and with the named customers below.

### 4.2 Named Iraqi customers (all vendor or trade-press statements)

| Bank | What | Date | Source |
|---|---|---|---|
| Ashur International Bank for Investment | A customer "In 2007, we embarked on ... a successful journey with ICSFS"; 2023 IFRS 9 system on ICS BANKS (quote from the bank's head of credit facilities) | 2007; May 2023 | [A] ICS-SUMMIT23 |
| Bank of Baghdad | Listed among ICSFS Iraqi customers (FinTech Futures list repeating the vendor, Oct 2017). The bank's own annual reports 2022-2025 call its core system "the banking system (BANKS)" updated "according to the latest version approved by the processing company"; parent Burgan Bank went live on ICS BANKS in May 2016 | 2016-2025 | [B] FF-IIB-OCT17; [A, inference] BOB-AR-2025; ICS-BURGAN |
| Mansour Bank for Investment (QNB owns 51%) | Replaces "its core banking and IT systems" with ICS BANKS; signing in Beirut | Nov 2015 | [A] ICS-MANSOUR |
| Al-Arabiya Islamic Bank (Baghdad; probably the "Arabia Islamic Bank" on the CBI's 2024 dollar-ban list, not confirmed) | ICS BANKS Islamic with core, credit facilities, trade finance, remittances, delivery channels, internet banking, cards and ATM management; planned go-live in two months | Feb 2017 | [A] ICS-ARABIYA |
| Al Janoob Islamic Bank, Taawen Islamic Bank, Noor Al Iraq Islamic Bank | Named as customers | 2017 | [B] FF-IIB-MAR17 |
| International Islamic Bank | Signed Mar 2017, live Oct 2017 ("five months") | 2017 | [B] FF-IIB-MAR17; FF-IIB-OCT17 |
| Al-Thiqa Islamic Bank | Selected Sep 2017, live Dec 2017 ("two months") | 2017 | [A] ICS-THIQA |
| Hammurabi Commercial Bank | Selected ICS BANKS Universal | Dec 2018 | [A] ICS-HAMMURABI |
| Al Mustashar Islamic Bank | CBI approval Q2 2018; live in "only 3 months" | Jan 2019 | [A] ICS-MUSTASHAR |
| First Iraqi Bank (digital bank) | International RFP "where many international vendors were evaluated"; ICS BANKS Digital selected Mar 2021, live Aug 2021 in "less than four months" | 2021 | [A] ICS-FIB-SEL; ICS-FIB-LIVE |
| UR Islamic Bank | Replaced a legacy system; selected Dec 2021, live Feb 2022 with HQ and five branches | 2021-22 | [A] ICS-UR-SEL; ICS-UR-LIVE |
| Al Sanam Islamic Bank | Selected ICS BANKS Islamic and digital | 21 Jul 2022 | [A] ICS-SANAM |
| Al-Ataa Islamic Bank | Signed; eight branches | Aug 2022 | [A] ICS-ATAA |
| Trans Iraq Bank; Gulf Commercial Bank (inference from the product name, not vendor-confirmed) | Trans Iraq Bank's 2023 annual report says its systems (customer-account registration and goAML) were linked with "the banks system", and its AML auditor paragraph lists a "Black List system supplied by Banks"; Gulf Commercial Bank's 2024 auditor says all accounting records are kept "according to the electronic banking system (BANKS)". Neither bank appears in the ICSFS press releases read | 2022-2024 | [A, inference] TIB-AR-2023; TIB-AR-2024; GCB-AR-2024 |

Several of these banks are among the banks the CBI barred from dollar transactions in February 2024 and February 2025 (file 01 section 4): Ashur International Bank, Al-Janoob Islamic Bank, Hammurabi Commercial Bank,
Arabia Islamic Bank (probably the Al-Arabiya Islamic Bank above) and Al-Sanam Islamic Bank are on those lists. This is a correlation (ICSFS has 30+ Iraqi customers), not evidence that the CBS contributed to any failure, but it means the banks that most
urgently need AML credibility disproportionately run ICS BANKS.

### 4.3 What the bundled AML does

Nothing is evidenced. The ICS BANKS product list is: Core Banking, IFRS9 ECL Module, Credit Facilities & Risk Groups, Lending, Trade Finance, Remittances, Investment & Treasury, ERP Suite, Dynamic Application Builder,
Dynamic Advice Builder, Regulatory Reports, Document Management System, plus classification and provisioning, nostro reconciliation, notifications and others [A, ICS-PRODUCTS]. No product page, release or
Iraq summit programme mentions anti-money laundering, transaction monitoring or screening; the 2022 summit covered "customer on-boarding, E-KYC", "Customer Classification and BASEL requirement ... covers the
Central Bank of Iraq's requirements", IFRS 9 and credit scoring [A, ICS-SUMMIT22]. The core banking page lists "RegTech Solutions" and "Customer Analytics" as headings only [A, ICS-CORE].
Conclusion: the dominant Iraqi CBS vendor does not sell an AML engine, so the AML and screening tools at ICS banks are bought separately (file 18 section 8). That is the biggest opening for AKR-TMS.
The absence is of public evidence, not proof; ask each ICS bank whether it licenses any "ICS" watch-list or customer-classification module. One bank statement supports the thin-controls reading: Trans Iraq Bank's auditor lists a "Black List system supplied by Banks" next to a separate AML system from another supplier, FircoSoft through a local reseller and World-Check [A, TIB-AR-2022; TIB-AR-2024] (the bank's core is probably ICS BANKS, file 18 section 4; the identification rests on the product name "BANKS"). Gulf Commercial Bank's 2024 auditor likewise names "the electronic banking system (BANKS)" and no AML tool [A, GCB-AR-2024].

### 4.4 How ICS BANKS stores data

- **Oracle Database.** ICSFS is "a Gold level member of Oracle PartnerNetwork", received Oracle's "ISV Partner of the Year" for Levant and Egypt on 18 Dec 2019 for "Oracle-based Financial solutions" and was "an
  early adopter of Oracle Autonomous Transaction Processing (ATP), Oracle Data Safe, Oracle JET, Visual Builder and Oracle 19c" [A, ICS-ORACLE-ISV]; its benchmark ran "ICS BANKS Universal Banking solution running on Oracle Exadata" and
  speaks of customers who "consolidate their databases" [A, ICS-EXADATA]. The application is "deployed in a multi-tiered setup that runs on a web thin client, J2EE environment" with an embedded service-oriented architecture [A, ICS-MANSOUR].
  File 25 flags that Arabic-region Oracle databases are often not AL32UTF8 and that Finacle and FLEXCUBE estates often sit on 11g or 12c [K]; the same checks apply here.
- Deployment: "on-premises, hybrid, or cloud" [A, ICS-SUMMIT23]. Iraqi banks will be on-premises (file 01 section 8).
- Table and column names for ICS BANKS were not found in any public document: [K]. The ICSFS "Dynamic Application Builder" suggests user-defined entities and fields, which in a bank's database usually means extra tables and
  flex columns that only that bank's DBA can explain.
- Integration points the vendor itself documents: "data interfaces", "ICS BANKS Remittances" and "Trade Finance" modules updated for each SWIFT MT standards release, including UETR in field 121 of MT 103, 202 and 202 COV [A, ICS-SWIFT-UG].
  So ICS banks keep SWIFT message data in the CBS remittance tables; AKR-TMS can use these for transaction screening context but still needs the SWIFT interface log for the raw message (file 25 section 12).

### 4.5 Weaknesses compared with a specialist

- No AML engine: a bank that wants monitoring must add a second vendor; the CBS vendor offers only "Customer Classification" and regulatory reports.
- Heavy reliance on one regional vendor with one database platform (Oracle): good for AKR-TMS's mapping effort (one engine, one vendor logic across 30 banks), but a change at ICS affects most of the market.
- The vendor markets implementation speed ("record-breaking time of only 3 months", "two months"), which suggests configuration packages where the AML-relevant data (expected activity, source of funds, beneficial owners, PEP flag) may be only
  partly captured in the CBS (to verify bank by bank; file 25 data-quality profile).

AKR-TMS hooks:
- Mapping profile "ICS BANKS (Oracle)" is priority one: one adapter family, per-bank customisation tables, data-quality profile for expected-activity and KYC fields.
- Sales motion for ICS banks: AKR-TMS is the AML layer ICS does not sell; message "reads your ICS BANKS database read-only, no change to the CBS vendor contract".
- Treat ICSFS as a possible partner: a user-group summit slot (June 2022 and May 2023 events with 90-120 attendees) is the cheapest way to reach 30+ banks at once.

## 5. Oracle FLEXCUBE Universal Banking (and Oracle FCCM)

Oracle's own AML product (Oracle Financial Crime and Compliance Management on OFSAA) is covered in file 09. This section covers FLEXCUBE as a CBS, how it hands data to OFSAA, and what that means for a read-only reader.

### 5.1 FLEXCUBE in Iraq

| Bank | What | Date | Grade, source |
|---|---|---|---|
| Rafidain Bank (largest state bank) | "understood to be implementing a suite of new Oracle software": FLEXCUBE Universal Banking (core), Oracle Banking Digital Experience (channels) and Oracle Banking Payments; "the deal was signed last year" (2018); it "will replace Finastra's Fusionbanking core banking system". Oracle's own Q2 FY2019 results said "a leading Iraqi bank" without a name. Rafidain's history: 2007 turnkey contract with B-Plan and Misys (Misys beat Temenos and Systems Access), legacy Misys Bankmaster, upgrade to Fusionbanking in 2012. No go-live announcement, Oracle customer reference or later press confirmation was found. | 13 Mar 2019 | [B] FF-RAFIDAIN (exclusive, unnamed sources); [B] RBI-2008; [B] WIKI-RAFIDAIN |

So the Rafidain FLEXCUBE project is **[B-] unconfirmed in operation**. The State bank restructuring programme (file 18 section 11, "First Rafidain Bank" and "upgrades to Iraq's financial information infrastructure") makes the CBS question live again. AKR-TMS must plan for Rafidain on any of three states: Fusionbanking on IBM i (old), FLEXCUBE (new), or a hybrid during migration.

### 5.2 What the bundled AML does

FLEXCUBE has no AML engine of its own on Oracle's FLEXCUBE page (it lists universal, retail, corporate, Islamic and microfinance banking, plus Digital Experience, Branch, Payments and Revenue Management) [A, ORA-FLEXCUBE].
AML is the separate OFSAA product line. The documented bridge is the FLEXCUBE-to-OFSAA integration guide (release 14.9, August 2026):

- "This integration is achieved by handing off FCUBS core banking data via staging tables to OFSAA using DIH connector" (Data Integration Hub) [A, ORA-OFSAA].
- "OFSAA user will have read-only access to this application. The access is provided only to particular extraction tables." [A, ORA-OFSAA]. This is the same security model AKR-TMS wants to use.
- "FLEXCUBE has pre-defined staging tables required for OFSAA extracts. During EOFI batch, the module wise data is extracted to staging tables in FCUBS. FCUBS provides a control table to indicate successful data extraction." Batch `EMXTRACT`
  is maintained in "Mandatory Batch Program Maintenance" (EIDMANPE); routines are listed in the table `ESTM_DEST_TABLES`; parameters are in `CSTB_EIS_PARAM`; the job status per routine is in `ESTB_JOB_CONTROL` (W in progress, F failed, S done)
  [A, ORA-OFSAA].
- "It is recommended that the extraction of data from FLEXCUBE UBS is done from the reporting environment and not the production environment." [A, ORA-OFSAA].
- Transfer rules: master tables incremental between two extraction dates; maintenance and contract tables in full each extraction; transaction table only the transactions created on the extraction date [A, ORA-OFSAA].
- The hand-off modules include customer entities, current and savings accounts, consumer lending, term deposit, limits and collateral, general ledger (including interbank correspondent transactions), corporate deposit, collections and leasing.

Consequences: (a) the OFSAA route is an **end-of-day** hand-off ("EOFI batch"), not near-real-time; a bank that wants intraday monitoring must read the transaction tables directly (AKR-TMS's approach) or buy
Oracle's real-time interfaces; (b) a FLEXCUBE bank that already runs `EMXTRACT` has a ready-made read-only staging area AKR-TMS could use as a source for daily reports and KYC/CDD data (file 25 pattern F, "vendor feed the bank
chooses to switch on"); (c) the control table gives a free completeness check.

### 5.2.1 Typical weaknesses of the FLEXCUBE plus OFSAA route compared with a specialist

- Two products, two upgrade cycles, one licence relationship with Oracle; evidence from the documentation itself: separate prerequisites in OFSAA ("FCUBS Connectors User Guide", "Data Integration Hub User Guide"), manual maintenance of the extraction routines in `ESTM_DEST_TABLES`
  "with the following values along with other details", manual maintenance of interest-rate data in `AATB_ICTM_INTEREST_RATE` and manual correction of extraction errors ("These errors are rectified manually and the batch is run again for the failed data") [A, ORA-OFSAA].
- Daily latency by design.
- Cost: no price was found in any public source; the Temenos marketplace pages on Oracle licence optimisation (TEM-CINTRA) show that Oracle database licences are a significant cost driver for the platform they underpin, which also applies to OFSAA's Oracle Database requirement [C].

### 5.3 How FLEXCUBE stores data

- **Oracle Database only.** Oracle's data model guide tells developers to "create the Oracle FLEXCUBE Universal Banking database tables by running all the DDL scripts" in the install tree (folders `MAIN\TABLE`, `ROFC\MAIN\TABLE`, `SMS\MAIN\TABLE`),
  then foreign keys from `ROFC\MAIN\FKR` and column comments from `...\CMT` [A, ORA-DATAMODEL]. The foreign keys are in a separate script, so a running schema may not enforce them: relationships come from the column comments and the vendor's
  DDL, not from the catalogue (verify on each bank).
- **No packaged ER diagram.** Oracle: "As the complete ER diagram of FLEXCUBE UBS application would be huge, the business application developers need to re-engineer with required filtered portion of FLEXCUBE UBS to get specific portion of data model"
  [A, ORA-DATAMODEL]. The documented method is to load the DDL into a scratch schema and reverse-engineer with SQL Developer Data Modeler. AKR-TMS's mapping team should do exactly that per release (a "DDL-to-canonical" assistant, section 12 hook).
- Table naming (from the tables named in Oracle's own guide): a two-letter module code, then `TB` or `TM`, then an underscore, for example `AATB_ICTM_INTEREST_RATE`, `ESTM_DEST_TABLES`, `ESTB_JOB_CONTROL`, `CSTB_EIS_PARAM` [A, ORA-OFSAA].
  The `TB` versus `TM` split is usually read as transactional versus maintenance tables [K]. Other names often cited in practitioner material (`STTM_CUSTOMER`, `ACTB_HISTORY`) are [K] until a bank's schema confirms them.
- Releases: documentation libraries run from 11.x through 14.9; a bank can be on any of 11.x, 12.x, 14.x [A, ORA-UBS149]. File 25 notes that FLEXCUBE estates frequently sit on Oracle 11g or 12c [K].
- Authorisation and record-status flags sit on master records; the business date is branch-relative in at least some modules (file 25 section 6, [K]).

AKR-TMS hooks:
- Mapping profile "FLEXCUBE UBS (Oracle)": generate the canonical mapping from the bank's DDL plus column comments, then confirm with sample queries; do not hard-code table names across releases.
- Support a second source adapter: the OFSAA staging tables (`EMXTRACT` outputs) for banks that have them, read from the bank's reporting database; use `ESTB_JOB_CONTROL` as a completeness gate.
- For Rafidain: build and test against both Fusionbanking (IBM i, section 7) and FLEXCUBE; ask the bank which is in production before pricing the integration.

## 6. Infosys Finacle

| Item | Fact | Grade, source |
|---|---|---|
| Own AML engine | None in the solution catalogue. The Finacle site (Wayback snapshot of the sitemap, 3 Jan 2026) lists some 60 solution pages (core banking, deposits, payments, trade finance, treasury, wealth, Islamic banking, digital onboarding, data and AI suite, customer data hub and more) but no AML, transaction monitoring or screening solution. "Finacle Alerts" is customer notification (SMS and push about account activity), and "Finacle Case Management" is for service requests, disputes and exceptions. | [A] FINACLE-SITEMAP; the two solution pages (alerts, case management) read via Wayback |
| Partner AML | The Finacle App Center lists partner products: "CrossFraud AML solution" ("Compliance reports compatible with FIUs"; "Integrated with KYC screening & Customer Risk Rating"; real-time, near-real-time and batch; configurable scoring), NetGuardians (fraud: "Plugging directly into core banking systems via pre-defined connectors"), Regula (identity verification), reg360 and others. | [A] FINACLE-CROSSFRAUD; FINACLE-NG; FINACLE-SITEMAP |
| Iraq | No Iraqi bank appears among Finacle's more than 100 published client-story pages. Regional clients shown: Housing Bank for Trade and Finance (Jordan; its client story and press release are on the site), Bank Alexandria (Egypt), Emirates NBD, RAKBANK, Al Ahli Bank Kuwait, Albaraka, Bank Muscat, Bank Dhofar and Bank Sohar. "Iraq" occurs on the Islamic banking page only inside the country drop-down. Finacle-based Iraqi banks were not found in any other source either. | [A] FINACLE-SITEMAP; [A] page check |
| Architecture | "cloud-native, cloud-agnostic, componentized, layered, and open APIs-led microservices architecture" (current Finacle); no database engine is named on the pages read. File 25 records Finacle estates on Oracle 11g or 12c [K]. | [A] FINACLE-CORE |

Weaknesses compared with a specialist: no native AML at all, so a Finacle bank always buys AML elsewhere (partner or specialist); the partner list is thin for AML (one AML vendor and one fraud vendor). Finacle is irrelevant to the Iraqi
mapping effort unless a bank is found to run it, but it matters in Jordan, Egypt and the Gulf (file 18 section 14).

AKR-TMS hooks: keep a "Finacle (Oracle)" mapping profile as a low-priority regional adapter; do not build it before a signed Iraqi or Kurdistan bank needs it.

## 7. Finastra (Fusion Equation and Bankmaster, Fusionbanking, Essence; Fusion Total Screening; Compliance as a Service)

### 7.1 Finastra in Iraq

| Bank | What | Date | Grade, source |
|---|---|---|---|
| Trade Bank of Iraq (TBI; state-owned) | Misys Equation core, Trade Innovation "TI Plus 2.0" and Equation Branch Automation at head office (6 branches) in 2008; "two major projects with TBI in 2003 and 2009"; an upgrade to "the latest version of Misys FusionBanking Equation" plus FusionBanking Teller, contract signed in Baghdad on 14 October (release republished 8 Feb 2016, so the signing was probably 14 Oct 2015) | 2008, 2015 | [A] GC-TBI-2008; TMI-TBI-2016 |
| Rafidain Bank | Misys Bankmaster legacy; 2012 upgrade to Fusionbanking; reported to be replaced by Oracle FLEXCUBE (section 5) | 2007-2019 | [B] RBI-2008; FF-RAFIDAIN |
| National Bank of Iraq | Trade finance runs on software from Finastra (Capital Bank of Jordan statement), core on Temenos since 2024 | Jan 2024 | [B] IBN-NBI |

### 7.2 What the bundled AML does

- Finastra's product pages and press releases show **no native transaction-monitoring engine**. It sells sanctions screening for payments ("Fusion Total Screening automatically screens and filters payments behind the scenes in one centralized system"; brochure 4 Jun 2021 [A, FIN-TOTAL]) and, since 14 Sep 2023, "Finastra Compliance as a Service" on Azure:
  "Fincom's real-time AML (Anti-Money Laundering) transaction screening and ThetaRay's AI-powered transaction monitoring as a pre-integrated packaged solution with Finastra Payments To Go" aimed at FedNow and TIPS [A, FIN-CAAS]. Both are partner technologies.
- Ownership change: on 19 Jun 2026 Finastra announced that Pollen Street Capital will acquire "Universal Banking ... Finastra's global core banking software business", "over 150 customers worldwide" in "more than 100 countries", centred on **Essence**; Finastra keeps payments and lending ("sharpen our focus on payments and lending"); subject to regulatory approvals; the buyer will run UB "as an independent business"
  [A, FIN-POLLEN]. The release does not name Equation, Fusionbanking or Bankmaster. Iraqi state banks on those products face a vendor-transition risk, and Trade Innovation (trade finance) stays with Finastra's lending business (the Finastra site lists it under lending).
- Comparison with the regional pattern: the CBS vendor does not own AML, the bank buys screening and monitoring from a specialist.

### 7.3 How Finastra cores store data

- Misys Equation (later Fusion Equation) is an IBM i (AS/400) application with the Db2 for i database [K]; I could not open a Finastra document that states this, because the Finastra site no longer carries Equation pages (the sitemap has none) and the Equation documentation is customer-only.
  File 25 section 3.4 covers Db2 in general. Bankmaster, the older Misys product, is also an IBM midrange system [K].
- Equation keeps records in fixed-length, short-named files with packed or zoned decimals and EBCDIC-era conventions [K]. For a Rust reader this means a Db2 for i or ODBC driver, a CCSID check for Arabic text, and bank-supplied logical-file definitions.
  Essence, the newer core, is described as "cloud-first, open banking" with APIs [A, FIN-POLLEN]; its database is not named in the pages read.
- Practical consequence for state banks: if Rafidain or TBI still run Equation or Fusionbanking in production, the CBI-mandated "direct link to the core banking system" is a Db2 for i job, not an Oracle one. Confirm in discovery.

AKR-TMS hooks:
- Mapping profile "Misys/Finastra Equation (Db2 for i)" for the two state banks: keep it as an option behind a bank-supplied extract (file 25 R4) if no ODBC read-only role can be created on the IBM i.
- Do not build a Finastra Essence adapter before an Iraqi customer needs it.
- Track the Pollen Street transaction (closing and any change of support terms) for the state-bank reference.

## 8. Intellect Design Arena and other CBS vendors present in the region

- **Intellect Design Arena** (Intellect Global Consumer Banking, Global Transaction Banking, eMACH.ai; iTurmeric for insurance): the sitemap (987 URLs) has no AML, screening or financial-crime product page and no Iraq page [A, INT-SITEMAP].
  Regional references on the site: Abu Dhabi Islamic Bank (transaction banking), Boubyan Bank, Faisal Islamic Bank of Egypt, a Saudi bank's Basel III project. No Iraqi bank was found.
  Relevance to AKR-TMS: low in Iraq; a possible data source in Egypt or the Gulf only.
- **Capital Banking Solutions** (CapitalBanker core, CapitalCompliance AML; Lebanese-linked, press releases dated Paris): present in Iraq with core banking since 2006 and with an AML platform at Trans Iraq Bank and Sumer Commercial Bank; see section 9.2.
- **Others found in Iraqi press**: BML Istisharat (section 9). The Iraq market also has e-payment and card processors (Qi Card, Switch, ZainCash, FastPay, AsiaHawala) whose own platforms are not core banking systems (file 18 section 12).
- **Vendors that evaluated Iraqi greenfield banks**: Zain Iraq Islamic Bank's selection named Temenos, Misys, Oracle FS, ICS Financial Systems and Capital Banking Solutions as the field in 2017 [A, FFNEWS-ZAIN]; First Iraqi Bank ran "an international RFP ... where many international vendors were evaluated" in 2021 [A, ICS-FIB-SEL].
  Greenfield banks therefore compare several vendors, and the CBS decision is repeated whenever a new bank is licensed (the CBI is licensing digital banks in 2026, file 01 section 2e).

## 9. BML Istisharat (ICBS), Capital Banking Solutions (CapitalBanker, CapitalCompliance) and other core systems seen in Iraq

### 9.1 BML Istisharat ICBS

BML Istisharat is a Lebanon-based banking software vendor; its ICBS core is one of the regional CBS families with Iraqi references (with ICS BANKS, iMAL and CapitalBanker), and it is the one with the **central bank itself** as a user:

| Item | Fact | Grade, source |
|---|---|---|
| Users | When Al Qurtas Islamic Bank (Erbil) selected ICBS on 24 Oct 2016 it "will become the tenth user of ICBS in the country. Other uses include Central Bank of Iraq, Erbil Bank, Minare Bank and National Bank of Iraq." Go-live was announced on 7 Nov 2017 ("core, branch, Islamic financing and international banking modules"). | [B] FF-ICBS-2016; FF-QURTAS |
| Modules at Al Qurtas | Core (parameters and engines, customers and accounts, batch); Islamic deposits, profit calculation and distribution, "local authorities requirements"; branches (cheque books, clearing, banker's cheques); Islamic financing (murabaha, ijarah, istisna, tawarruq, qard hasan); international (inbound and outbound transfers, letters of guarantee); interfaces for ATM, CDM, POS, card management and "payments solutions (RTGS and ACH)". | [B] FF-ICBS-2016 |
| AML | No AML module or partner was found for ICBS. The company's own website could not be read (a bare frameset), and the two Wayback dates tried (2019, 2021) have no copy. | [B-] |
| Data | Not found: [K]. |  |

Implications:
- National Bank of Iraq's pre-2024 "legacy systems" were ICBS: the bank's 2023 annual report calls ICBS its activated banking system (customer risk-rating field linked to the AML profiling system) and lists an "ICBS System Renewal 2023" contract with Istisharat at IQD 64.7m a year (about USD 49k) [A, NBI-AR-2023]; its Temenos migration in early 2024 shows ICBS banks do move.
- The CBI running a regional core (ICBS) matters for the "CBS-system linked with banks" statement in the MER (Box 3.1, para 291 footnote): a CBI-side system already reads bank data, so banks accept an outside read link.
- The interface to "RTGS and ACH" means payment-system data enters the CBS: AKR-TMS can see CBI payment flows through the CBS ledger without a separate CBI feed (file 25 section 12).

### 9.2 Capital Banking Solutions (CapitalBanker, PowerBanker, CapitalCompliance): a CBS vendor with an AML platform and a large claimed Iraqi base

Capital Banking Solutions ("CBS" in its own releases, not to be confused with "core banking system") describes itself as active "Since 1997" in "Europe, Africa, Central Asia, Middle East and America" and as "Partner of 200+ clients" [A, CBSOL-HOME]. Its releases are datelined Paris and carry a Lebanese contact number [A, CBSOL-FORRESTER]. The vendor's site was read through Wayback copies (Oct 2022 to Jan 2023) because the current domain was not reachable from the research sandbox.

| Item | Fact | Grade, source |
|---|---|---|
| Core banking products | "CapitalBanker, running on Oracle technologies" and "PowerBanker, running on IBM Power platforms"; CapitalBanker is described as a "universal cloud-ready core banking system" for retail, corporate, private and Islamic banking, "Compliant with 60+ country localizations"; companion products CapitalDigital (digital banking) and CapitalConnect (open banking APIs) | [A] CBSOL-CORE; CBSOL-SUMER |
| AML product | **CapitalCompliance**: "AML Transaction Monitoring, Name Screening, FATCA, CRS, and Regulatory Reporting", configurable alert and case management with "Intelligent Learning", dashboards and a report generator, "open standard and dedicated APIs", multilingual, deployable "On-Cloud or On-Premises"; "developed in collaboration with Intelleval"; "used by more than 50 Banks and financial institutions worldwide" (vendor claim) | [A] CBSOL-COMPLIANCE; [C] for the usage claim |
| Iraq claims | IMEIB's chairman recalls the 2006 start: IMEIB "implemented CBS's first Core banking system in Iraq" and "18 other Iraqi banks chose CBS products" (release of 12 Jan 2022); in Aug 2022 the vendor says it "is currently servicing more than 25 Iraqi retail, commercial and islamic banks" (product scope per bank not stated) | [A] CBSOL-IMEIB; CBSOL-SUMER (vendor statements) |
| Named Iraqi banks | Iraqi Middle East Investment Bank (core since 2006; CapitalDigital live Jan 2022); Sumer Commercial Bank (partnership "since 2015"; Aug 2022 selection of the new CapitalBanker core and CapitalDigital; CapitalCompliance "extended ... to include GoAML and CRS to cover the compliance and tax evasion requirements as per the Central Bank of Iraq regulation"); Trans Iraq Bank (its auditor names an "AML system supplied by capital banking" in the 2022, 2023 and 2024 reports, with a different core, probably ICS BANKS) | [A] CBSOL-IMEIB; CBSOL-SUMER; TIB-AR-2022; TIB-AR-2023; TIB-AR-2024 |
| Analyst standing | Vendor says it was named for the fourth year running "a Base and Regional Player for new deals and a Solid Player in Forrester's Global Banking Platform Vendor Pyramid 2020 with 25 combined deals"; the Forrester report was not read | [C] CBSOL-FORRESTER |
| Zain Iraq evaluation | Capital Banking Solutions was one of five vendors Zain Iraq Islamic Bank evaluated in 2017 | [A] FFNEWS-ZAIN |

What this means:
- It is the second CBS vendor (after Azentio) whose AML product is evidenced inside Iraqi banks, and the only one whose AML runs next to a **different** vendor's core (Trans Iraq Bank). It is also the only CBS-type vendor found that advertises goAML for Iraq, in the Sumer release.
- Its claimed Iraqi base ("more than 25", plus the "18 other Iraqi banks" in 2022) overlaps in size with ICSFS's claim of "30+" Iraqi customers; with 49 private banks and 11 foreign branches (file 18 section 2) and iMAL (6 named banks), ICBS (5) and Temenos (3) also present, the claims of ICSFS ("30+") and Capital Banking Solutions ("more than 25") cannot all describe full core banking, so the product scope per bank (core, digital, compliance) must be asked in every discovery. The claims are vendor statements: [C] for counts.
- The Forrester statement matters for the CBI's "Tier 1" core banking test (file 18 section 12.2): Capital Banking Solutions is the only regional core banking vendor found that cites a Forrester vendor listing, which is a vendor claim, not a verified Tier 1 status.
- Database facts: CapitalBanker "running on Oracle technologies" (Oracle read-only role or replica, as for ICS BANKS); PowerBanker on "IBM Power platforms" (probably IBM i with Db2 for i, as for Misys Equation: [K], not stated). Table and column names are not public: [K].
- CBS-vendor AML comparison for AKR-TMS: CapitalCompliance is pre-integrated with CapitalBanker, carries goAML and CRS for Iraq at Sumer, and is cheap to add for a bank already on the vendor's stack; it is not evidenced to carry the CBI scenario pack, Arabic name matching or published accuracy figures.

Payment-gateway databases sit beside the CBS: ProgressSoft's PS-GatePay (RTGS, ACH and cheque gateway, "integrating with the banks' Core Banking Systems") runs at nine Iraqi banks including National Bank of Iraq, Ashur, Bank of Baghdad, Credit Bank of Iraq, Mosul Bank, RT Bank, BLOM and Standard Chartered [A, PS-GATEPAY; file 25 sections 11.6 and 12]. Its database is a second structured source for domestic payments.

AKR-TMS hooks:
- Add "BML ICBS" and "Capital Banking CapitalBanker" to the adapter backlog behind ICS BANKS and iMAL; ask each Kurdistan bank (Erbil Bank, Minare Bank, Al Qurtas) which core they run, and ask IMEIB and Sumer for their CapitalBanker database details.
- Keep the mapping layer schema-driven; four regional vendors with unknown schemas (ICS, Path/Azentio, BML Istisharat, Capital Banking Solutions) cover most Iraqi private banks, so the first customer in each family funds the adapter for the rest.

## 10. Typical weaknesses of CBS-native AML compared with specialist systems

Drawn from the evidence in sections 2 to 9; "specialist" here means a stand-alone AML product such as AKR-TMS aims to be.

| # | Weakness | Evidence | Who is affected |
|---|---|---|---|
| 1 | **No AML engine at all**: the CBS vendor sells core banking and leaves AML to the bank | ICS BANKS product list has no AML or monitoring product (section 4.3); Finacle's catalogue has none and lists partners (section 6); Finastra has screening for payments and partner monitoring (section 7); Intellect has none (section 8); BML ICBS: none found (section 9) | ICS, Finacle, Finastra, Intellect, BML (and iMAL itself, whose AML is a separate Azentio product) |
| 2 | **Deepest integration only with the vendor's own CBS**; other cores connect through an API catalogue or "pre-defined connectors" | Temenos FCM "pre-integrated with Temenos core ... or with an API catalogue ... to any other third-party core" (section 2.1); Azentio FCM claims "20+ core banking systems" (section 3.2); Finacle partner NetGuardians "Plugging directly into core banking systems via pre-defined connectors" (section 6) | Temenos, Azentio, Finacle partners |
| 3 | **Latency by design**: daily or end-of-day feeds | FLEXCUBE to OFSAA is an EOFI batch (section 5.2); Siron at Bank of Baghdad "monitors daily banking operations" (file 18 section 4) | Oracle route, Siron-type tools |
| 4 | **Arabic and Iraqi data not shown**: no public statement of Arabic-script name matching or Iraqi identifiers | Temenos pages: no mention (section 2.1); Azentio: Arabic only for narratives (section 3.5); Oracle's Behavior Detection fuzzy matcher ignores characters beyond the first 128 UTF-8 code points, "such as Chinese, Arabic" (file 09 section 8) | Temenos, Azentio, Oracle |
| 5 | **No Iraqi regulatory pack** in vendor materials: no CBI scenario catalogue, no goAML for the Iraqi FIU, no CBI behavioural indicators. Partly contradicted for Azentio at Ameen, whose auditor says the AML system "includes the minimum scenarios required by the Central Bank of Iraq" (stated as 25; the MER says the CBI pack is 56) | Azentio names SAFIU (Saudi), generic CTR and STR; Temenos names regulators by region; nothing for the CBI in vendor material; Ameen annual reports 2021 to 2024 (section 3.3) | All CBS-bound vendors; Azentio only in part |
| 6 | **Unvalidated headline numbers**: false-positive claims from vendor programmes | "less than 2%" and "up to 40% fewer" (Temenos); "up to 40%" and "at least 20%" cost reduction, "zero fines" (Azentio); no independent validation or Iraqi data | Temenos, Azentio |
| 7 | **Licence and platform cost tied to a database vendor** | Temenos marketplace pages: "Oracle database workloads that underpin Temenos" with a roadmap to "temenos certified Postgres" to cut licence cost (section 2.3); FLEXCUBE and OFSAA need Oracle Database (section 5.3) | Temenos, Oracle FLEXCUBE |
| 8 | **Ownership churn and roadmap risk** | Path Solutions absorbed into Apax-owned Azentio (section 3.1); Finastra selling Universal Banking to Pollen Street, 19 Jun 2026 (section 7.2); Actimize on sale (file 08) | Azentio, Finastra |
| 9 | **The CBI's "Tier 1" core banking rule can retire the CBS and its bundled modules** | CBI FAQ Feb 2026 (file 18 section 12.2); regional CBS vendors (ICS, BML, iMAL) are not described as Gartner or Forrester core banking vendors | ICS, BML, iMAL |
| 10 | **Conflict of roles in assessment**: a firm that implemented a fix cannot do the formal compliance assessment for the same bank | CBI FAQ Feb 2026 (file 18 section 12.2) | CBS vendors that also sell AML consulting |

What CBS-native AML does better than a specialist (to be fair to the incumbents):
- Zero or low data integration effort for the vendor's own customers: Temenos FCM and Azentio at iMAL banks are pre-wired (KIIB: "integrated with iMAL"; Ameen Al-Iraq: "all the above systems are linked to the banking system", four annual reports running).
- One contract, one support line and one upgrade cycle; the bank can tell an assessor "the vendor certifies it".
- Real-time hooks inside the CBS: a native hook can stop a transaction before posting; a read-only reader cannot (file 25 section 15, phase 2).

AKR-TMS hooks (section 10): answer weaknesses 3 to 6 with visible features (near-real-time reads, Arabic matching, CBI scenarios, goAML) and answer the "native hook" advantage honestly through the phase-2 API.

## 11. Database comparison table (what AKR-TMS must handle per CBS)

Evidence marks as in section 1; "[A]" means read in a vendor document in this research. File 25 section 11 (written by another agent) holds the integration detail; if the two disagree, treat file 25 and the bank's DBA as authoritative.

| CBS | Database engine(s) | Notable schema facts | Read-only approach | Verified by |
|---|---|---|---|---|
| Temenos Transact (T24) | Oracle (common), PostgreSQL ("certified"), older jBASE, Db2 and SQL Server reported | Each Transact "file" is a table; the record is one XML text in RDBMS mode; key column `RECID` (reserved word in Oracle); unauthorised and history copies live in separate files (`$NAU`, `$HIS`) | Fetch the XML text and parse in Rust; or read the Temenos Data Hub operational data store | [A] TEM-CINTRA; TEM-TDH; [S, K] file 25 |
| Oracle FLEXCUBE Universal Banking | Oracle Database only | Two-letter module code plus `TB` or `TM` table names; no packaged ER diagram (reverse-engineer from DDL scripts); foreign keys in a separate script; EOD staging tables for OFSAA with a job-control table | Read the staging tables on a reporting database (Oracle's own advice), or live tables on a replica | [A] ORA-DATAMODEL; ORA-OFSAA |
| ICS BANKS and BANKS Islamic | Oracle Database (Oracle 19c, Exadata benchmarks) | J2EE multi-tier application; "Dynamic Application Builder" implies bank-specific tables and flex fields; remittance and trade finance modules carry SWIFT MT data | Oracle read-only role or replica; per-bank data dictionary from the DBA | [A] ICS-ORACLE-ISV; ICS-EXADATA; ICS-MANSOUR; ICS-SWIFT-UG |
| Path/Azentio iMAL | Oracle or SAP Sybase ASE (2017 review); R14.5 "n-tier, cloud-enabled" | Islamic profit-calculation (PCS) and pool tables; table names not public | Oracle: as above; Sybase ASE: TDS-capable driver or bank replica | [B] FIT-IMAL; [A] AZ-TADHAMON |
| BML Istisharat ICBS | Not found | Interfaces for RTGS and ACH, ATM, POS, cards | Bank-delivered extract until the engine is known | [B] FF-ICBS-2016 |
| Capital Banking Solutions CapitalBanker | "running on Oracle technologies" | "cloud-ready" universal core; table names not public | Oracle read-only role or replica, as for ICS BANKS; per-bank dictionary from the DBA | [A] CBSOL-CORE |
| Capital Banking Solutions PowerBanker | "running on IBM Power platforms" (probably IBM i with Db2 for i [K]) | Not public | Db2 for i via ODBC or a bank-delivered extract, as for Equation | [A] CBSOL-CORE; [K] for the engine |
| Misys/Finastra Equation, Fusionbanking, Bankmaster | IBM i with Db2 for i (Equation) [K] | Short fixed-length file and field names, packed decimals, EBCDIC-era conventions [K] | Db2 for i via ODBC or a bank-delivered extract | [K]; Finastra documents not public |
| Infosys Finacle | Oracle commonly [K]; the vendor pages name no engine | Cloud-native, microservices in current releases | Oracle read role or replica [K] | [A] FINACLE-CORE (architecture only) |
| Intellect (iGCB, eMACH.ai) | Not found | Not found | Not found | [A] INT-SITEMAP |

## 12. What it means for AKR-TMS

1. **The CBS vendor is a data source first and a competitor second.** In Iraq the CBS-type vendors that sell AML are Azentio next to iMAL (two banks: Ameen Al-Iraq in its annual reports since 2021 and KIIB in 2024-25), Temenos in principle (no Iraqi customer) and Capital Banking Solutions (CapitalCompliance: Trans Iraq Bank by its auditor, Sumer by the vendor's release; section 9.2). The dominant Iraqi CBS (ICS BANKS) has none. The largest private banks run separate AML tools from other vendors: FICO Siron and FircoSoft Firco at Bank of Baghdad, Siron (written "AML Sirion") at NBI from 2024 (file 18 section 4), Capital Banking AML and FircoSoft at Trans Iraq Bank, and a Jordanian regional integrator, Pio-Tech, sells goAML and risk-based-assessment modules to many banks (file 18 section 8).
2. **Adapter priority follows the Iraqi evidence**: ICS BANKS (Oracle) first, then Temenos Transact (NBI, ZainCash, BLF, and likely replacements under the "Tier 1" rule), then iMAL (Kurdistan and several Islamic banks), then Capital Banking CapitalBanker (Oracle; IMEIB and Sumer by vendor releases, a claimed 25+ Iraqi banks), then BML ICBS and Misys/Finastra Equation for the state banks and Kurdistan, and Oracle FLEXCUBE once Rafidain's production state is known.
3. **Vendor documentation is mostly closed** (Temenos docs behind login, iMAL and ICS schemas not public). The product must not depend on vendor documents: it must derive the mapping from each bank's own dictionary and data profile (Oracle itself recommends reverse engineering from DDL).
4. **Use the vendor feeds that already exist** (Temenos Data Hub ODS, FLEXCUBE OFSAA staging) as optional read-only sources; they cost the bank a licence or a batch but give a clean interface and a completeness check.
5. **Expect CBS churn** (CBI Tier 1 rule, mergers, First Rafidain Bank, Finastra's divestment): the canonical model must be independent of any CBS and the mapping library versioned.
6. **Position against CBS-bundled AML on the five visible gaps** (latency, Arabic, CBI scenarios, goAML, transparent price) and concede the native-hook advantage through the phase-2 API. The "CBI scenarios" gap is about the full, current, versioned pack and proof of coverage: at Ameen the auditor already says the Azentio system carries the CBI minimum set, counted as 25 against the CBI's 56 in the MER.

## 13. Derived requirements (VD-xx, CBS side; the market side continues in file 18)

| ID | Requirement | Source |
|---|---|---|
| VD-01 | Ship CBS adapter families in this order, each with a data-quality profile and a version register: ICS BANKS on Oracle; Temenos Transact (Oracle XML, PostgreSQL, Data Hub ODS); Path/Azentio iMAL (Oracle or Sybase ASE); Capital Banking CapitalBanker (Oracle) and PowerBanker (IBM Power); BML ICBS and Misys/Finastra Equation (Db2 for i) for state and Kurdistan banks; Oracle FLEXCUBE (Oracle) | Sections 2 to 9; file 18 sections 4 to 7 |
| VD-02 | Derive each bank's mapping from the bank's own DDL, column comments and a sample extract (reverse-engineer into a scratch schema, as Oracle documents), never from hard-coded vendor table names; keep the dictionary under change control | ORA-DATAMODEL; TEM-DOCS (login-only) |
| VD-03 | Support optional vendor read stores as sources: Temenos Data Hub operational and snapshot stores; FLEXCUBE OFSAA staging tables on a reporting database; use their job-control or reconciliation tables (`ESTB_JOB_CONTROL`) as completeness gates | TEM-TDH; ORA-OFSAA |
| VD-04 | Parse Temenos Transact XML records outside the CBS database in a streaming parser; handle the `RECID` reserved word, unauthorised (`$NAU`) and history (`$HIS`) files and multi-company prefixes | File 25 section 11.1 [S]; section 2.3 |
| VD-05 | Provide a Sybase ASE-capable path (driver or bank-supplied Oracle/PostgreSQL replica) for older iMAL banks, and an IBM i Db2 path (ODBC or bank extract) for Equation and Fusionbanking banks | FIT-IMAL (2017); section 7.3 [K] |
| VD-06 | Run a character-set round-trip test (Arabic letters, tashkeel, Kurdish letters, Persian digits) on every adapter at install and on every CBS upgrade; Oracle, Db2 and Sybase character sets differ | File 25 section 3.1 |
| VD-07 | Capture in the first discovery meeting the bank's named AML, screening, reporting and archiving tools (for example Siron, Firco, an "AML profiling system", Pio-Tech modules, AMLOCK, CapitalCompliance) and the CBS vendor's AML offer, and decide per bank between co-existence and replacement | Sections 3.3, 4.3; file 18 sections 4 and 8 |
| VD-08 | Keep a CBS-bundled-AML comparison sheet (Temenos FCM, Azentio FCM/AMLOCK, Capital Banking CapitalCompliance, Oracle FCCM via OFSAA, Finacle and Finastra partner AML) with claims, evidence grades and the five visible gaps (latency, Arabic, full current CBI scenario pack with proof of coverage, goAML, price) | Sections 2, 3, 5, 6, 7, 10 |
| VD-09 | Treat every vendor performance number (false-positive reductions, "zero fines") as a claim: AKR-TMS must publish its own measured precision and recall on the bank's data and never reuse a vendor's figure in sales material | Sections 2.4, 3.2 |
| VD-10 | Data dictionary and mapping artefacts must be exportable for a CBI-approved assessor and for the bank's DBA, with the lineage from CBS column to canonical field | File 18 section 12.2; ORA-DATAMODEL |
| VD-11 | Plan for PostgreSQL-hosted CBS databases (Temenos certified Postgres roadmap) alongside Oracle; do not assume Oracle | TEM-CINTRA |
| VD-12 | For ICS BANKS banks, model user-defined fields and bank-specific tables (Dynamic Application Builder) as per-bank extension mappings; profile completeness of expected activity, source of funds, beneficial owner and PEP fields | Section 4.4 |
| VD-13 | Read SWIFT-related fields from CBS remittance and trade finance tables where present (MT 103, MT 202, UETR in field 121) and mark messages without raw SWIFT logs as lower-fidelity | ICS-SWIFT-UG |
| VD-14 | Treat the vendor's pre-integration with its own AML as a strength to answer, not ignore: offer a one-day "read-only trial" on a replica showing alerts on the bank's last 90 days beside the bundled tool | Section 10 |
| VD-15 | Track vendor ownership events (Finastra Universal Banking sale to Pollen Street, Azentio under Apax) in the adapter support policy and promise adapter support for each CBS version for the contract term | Sections 3.1, 7.2 |

## 14. Sources

Every URL used, by the label used in the text. Wayback Machine URLs are copies of pages whose original site blocks automated access (FinTech Futures, Iraq Business News, parts of Finacle) or no longer answers (Path Solutions, Capital Banking Solutions). Arabic-language bank annual reports (Ameen Al-Iraq, Trans Iraq Bank, Gulf Commercial Bank) were read as page images because their text layer is missing or incomplete, and the National Bank of Iraq 2023 report through its text layer after Unicode normalisation of Arabic presentation forms; quoted phrases from them are the researcher's translation, and page numbers are PDF page numbers. Evidence grades are explained at the top of file 17 section 1 (A, B, B-, C, K) and in the header of file 18 (A, B, C, N). Pages read in October 2026.

- **MER**: MENAFATF Mutual Evaluation Report of Iraq, May 2024 (read from the local text copy iraq-mer-2024-en.txt). https://www.menafatf.org/sites/default/files/Newsletter/IRAQ%20MER_En_0.pdf
- **TEM-NBI**: Temenos press release, 25 Apr 2024: National Bank of Iraq goes live (core banking and payments; delivery partner ITSS). https://www.temenos.com/press_release/national-bank-of-iraq-goes-live-with-temenos-core-banking-and-payments/
- **TEM-ZC**: Temenos press release, 23 May 2024: ZainCash goes live. https://www.temenos.com/press_release/iraqi-fintech-zaincash-goes-live-with-temenos-core-banking-and-payments-to-scale-digital-payment-services/
- **TEM-ZC-SS**: Temenos success story: ZainCash (Temenos core + Payments Hub, Prema Consulting, TIBCO middleware, 700,000 wallets migrated). https://www.temenos.com/success-story/zain-cash-success-story/
- **TEM-FCM**: Temenos Financial Crime Mitigation product page. https://www.temenos.com/products/core-banking/regulatory-compliance/financial-crime-mitigation/
- **TEM-FCM-SAAS**: Temenos press release, 12 Oct 2022: FCM-as-a-Service (over 300 banks). https://www.temenos.com/press_release/temenos-launches-self-service-financial-crime-mitigation-on-temenos-banking-cloud/
- **TEM-FCM-AI**: Temenos press release, 21 May 2025: FCM AI Agent. https://www.temenos.com/press_release/temenos-launches-ai-agent-to-transform-financial-crime-mitigation/
- **TEM-FCM-BLOG**: Temenos blog: why regional and community banks need a smarter approach to financial crime compliance. https://www.temenos.com/blog/why-regional-and-community-banks-need-a-smarter-approach-to-financial-crime-compliance/
- **TEM-TDH**: Temenos Data Hub product page. https://www.temenos.com/products/core-banking/data-hub/
- **TEM-6SIGMA**: Temenos marketplace: 6 Sigma AML platform. https://www.temenos.com/solution-provider/anti-money-laudring-aml-6-sigma/
- **TEM-CINTRA**: Temenos marketplace: Cintra Oracle optimisation and licence assessment (Temenos on Oracle; certified Postgres roadmap). https://www.temenos.com/solution-provider/temenos-oracle-optimization-and-licence-assessment-cintra-ltd/
- **FF-BLF**: FinTech Futures: Banque Libano-Francaise (Iraq) upgrades Temenos T24 (read only as a search-result summary; the site blocks automated access). https://www.fintechfutures.com/core-banking-technology/banque-libano-fran-aise-in-iraq-upgrades-temenos-t24-core-banking-system
- **IBN-NBI**: Iraq Business News, 11 Jan 2024: UK firms introduce new banking systems at National Bank of Iraq (Wayback copy). https://web.archive.org/web/20240118022256/https://www.iraq-businessnews.com/2024/01/11/uk-firms-introduce-new-banking-systems-at-national-bank-of-iraq/
- **TEM-DOCS**: Temenos product documentation portal (customer and partner login only). https://docs.temenos.com/
- **AZ-KIB-IMAL**: Azentio release, 25 Jan 2024: Kurdistan International Islamic Bank adopts iMAL, with Azentio FCM/AML integrated. https://www.azentio.com/insights/news-and-announcements/kiib-digital-transformation-journey-successfully-kicked-off-by-azentio
- **AZ-KIIB-AML**: Azentio release, 28 Feb 2025: AMLOCK deployed at Kurdistan International Islamic Bank. https://www.azentio.com/insights/news-and-announcements/azentio-partners-with-kurdistan-international-islamic-bank-to-strengthen-anti-money-laundering-compliance-with-azentio-amlock
- **AZ-AMLOCK-2025**: Azentio release, 22 May 2025: next-generation AMLOCK (400+ rules, 40% fewer false positives claim). https://www.azentio.com/insights/news-and-announcements/ai-powered-amlock-aml-solution-for-compliance
- **AZ-FCM-PDF**: Azentio Financial Crime Management and Anti-Fraud System brochure (100+ implementations, 20+ core systems, 150+ scenarios, FCM Lite). https://content.azentio.com/hubfs/file/azentio-fin-crime-anti-fraud-system.pdf
- **AZ-CELENT**: Azentio release, 26 Jul 2023: Celent noteworthy solution, platform used by 100+ institutions. https://www.azentio.com/insights/news-and-announcements/azentio-aml-transaction-monitoring-recognised-by-celent
- **AZ-SAUDI**: Azentio AMLOCK page for Saudi Arabia (feature list, SPARK Matrix leader claim). https://www.azentio.com/aml-software-saudi-arabia
- **AZ-IMAL**: Azentio iMAL Islamic banking product page. https://www.azentio.com/banking/imal-islamic-banking
- **AZ-TADHAMON**: Azentio release: Tadhamon Bank upgrade to iMAL R14.5 (cloud-enabled, n-tier). https://www.azentio.com/insights/news-and-announcements/tadhamon-bank-embraces-open-banking-with-imal-r14-5
- **FIT-IMAL**: Financial IT product review of iMAL, 27 Feb 2017 (Oracle or SAP Sybase database). https://financialit.net/publications/product-reviews/imal-powering-islamic-banking-infrastructure
- **FIT-IDB**: Financial IT: International Development Bank (Iraq) iMAL R14 upgrade, go-live 30 Sep 2018. https://financialit.net/news/core-banking/iraq-idbs-successful-upgrade-imal-r14
- **FFNEWS-ZAIN**: FF News, 27 Oct 2017: Zain Iraq Islamic Bank selects iMAL (Path Solutions); competing vendors named. https://ffnews.com/news/zain-bank-chooses-imal
- **FF-AMEEN**: FinTech Futures, 18 Apr 2019: Ameen Al-Iraq signs iMAL; Path's first Iraq customer Cihan Bank in 2009 (Wayback copy). https://web.archive.org/web/20200522194839/https://www.fintechfutures.com/2019/04/ameen-al-iraq-latest-addition-to-paths-iraq-user-base/
- **FF-ALRAJIH**: FinTech Futures, 13 Dec 2017: Al-Rajih Islamic Bank signs iMAL ahead of ICS Banks (Wayback copy). https://web.archive.org/web/20210305042753/https://www.fintechfutures.com/2017/12/path-solutions-beats-ics-financial-systems-to-core-banking-system-deal-in-iraq/
- **PATH-2021**: Wayback snapshot of path-solutions.com, 30 Dec 2021 (Azentio news items; Iraq in country list; 150+ Islamic institutions in 40 countries). https://web.archive.org/web/20211230160250/https://www.path-solutions.com/
- **PATH-2023**: Wayback snapshot, 10 May 2023: path-solutions.com request resolves to azentio.com (iMAL listed under ONEBanking). https://web.archive.org/web/20230510133037/https://www.azentio.com/
- **ICS-MANSOUR**: ICSFS release, Nov 2015: Mansour Bank for Investment (QNB owns 51%) replaces its core banking and IT systems with ICS BANKS; "live and fully operated in the Iraqi Market since March 2001". https://icsfs.com/newsroom/mansour-bank-for-investment-opts-for-ics-banks-from-icsfs/
- **ICS-HAMMURABI**: ICSFS release, Dec 2018: Hammurabi Commercial Bank selects ICS BANKS; "over 25 banks" in Iraq. https://icsfs.com/newsroom/hammurabi-bank-selection/
- **ICS-ARABIYA**: ICSFS release, Feb 2017: Al-Arabiya Islamic Bank selects ICS BANKS Islamic. https://icsfs.com/newsroom/al-arabiya-islamic-bank-selects-ics-banks-islamic/
- **ICS-THIQA**: ICSFS release, Dec 2017: Al-Thiqa Islamic Bank live in two months. https://icsfs.com/newsroom/al-thiqa-islamic-bank-deploys-ics-banks-islamic-system-from-icsfs/
- **ICS-MUSTASHAR**: ICSFS release, Jan 2019: Al Mustashar Islamic Bank live in three months. https://icsfs.com/newsroom/al-mustashar-islamic-bank-goes-live-in-record-time-with-ics-banks-islamic-from-icsfs/
- **ICS-FIB-SEL**: ICSFS release, Mar 2021: First Iraqi Bank selects ICS BANKS Digital after an international RFP. https://icsfs.com/newsroom/new-digital-only-bank-to-select-ics-banks-from-ics-financial-systems-icsfs/
- **ICS-FIB-LIVE**: ICSFS release, Aug 2021: First Iraqi Bank live in under four months. https://icsfs.com/newsroom/the-first-digital-bank-in-iraq-to-successfully-go-live-on-ics-banks-from-icsfs/
- **ICS-UR-SEL**: ICSFS release, Dec 2021: UR Islamic Bank replaces its legacy system. https://icsfs.com/newsroom/ur-islamic-bank-selects-ics-banks-islamic-banking-solution-from-icsfs/
- **ICS-UR-LIVE**: ICSFS release, Feb 2022: UR Islamic Bank live (HQ and five branches). https://icsfs.com/newsroom/ur-islamic-bank-goes-live-on-ics-banks-islamic-banking-solution-from-icsfs/
- **ICS-SANAM**: ICSFS release, 21 Jul 2022: Al Sanam Islamic Bank selects ICS BANKS Islamic. https://icsfs.com/newsroom/al-sanam-islamic-bank-selects-ics-banks-islamic-banking-solution-from-icsfs/
- **ICS-ATAA**: ICSFS release, Aug 2022: Al-Ataa Islamic Bank signs (eight branches). https://icsfs.com/newsroom/al-ataa-islamic-bank-signs-for-ics-banks-islamic-platform-from-icsfs/
- **ICS-SUMMIT22**: ICSFS release: Iraq User Group Summit, Baghdad, 21 Jun 2022 (30+ active customers; AL-AWAEL, Fortinet, Huawei partners). https://icsfs.com/newsroom/iraq-summit-22/
- **ICS-SUMMIT23**: ICSFS event page: ICS BANKS Open Day Summit, Baghdad, 21 May 2023, with Oracle; Ashur International Bank testimonial. https://icsfs.com/events/icsbanks-iraq-summit/
- **ICS-SWIFT-UG**: ICSFS event page: SWIFT user group meeting for Iraqi banks, Amman, 3 Oct 2018 ("dominant, with 23 banks"). https://icsfs.com/events/swift-user-group-meeting-for-iraqi-banks/
- **ICS-ORACLE-ISV**: ICSFS: Oracle ISV Partner of the Year, Levant and Egypt, 18 Dec 2019 (Oracle 19c, Exadata, ATP). https://icsfs.com/recognitions/icsfs-wins-oracle-isv-partner-of-the-year/
- **ICS-EXADATA**: ICSFS: ICS BANKS benchmark on Oracle Exadata. https://icsfs.com/insights/success-stories/oracle-exadata/
- **ICS-MKT**: ICSFS release, Mar 2018: ICS BANKS in the Oracle Cloud Marketplace; offices in UK, Jordan, UAE, Iraq, Saudi Arabia, Palestine, Sudan, Nigeria, Lebanon. https://icsfs.com/newsroom/ics-banks-universal-banking-application-from-icsfs-available-in-oracle-cloud-marketplace/
- **ICS-PRODUCTS**: ICS BANKS product list (no AML or transaction-monitoring product listed). https://icsfs.com/products/
- **ICS-CORE**: ICS BANKS Core Banking page ("RegTech Solutions", "Customer Analytics"). https://icsfs.com/products/core-banking/
- **FF-IIB-MAR17**: FinTech Futures, 27 Mar 2017: International Islamic Bank (Iraq) signs ICS Banks; ICSFS has "around 20 users" (Wayback copy). https://web.archive.org/web/20210305042753/https://www.fintechfutures.com/2017/03/international-islamic-bank-picks-core-banking-system-and-vendor/
- **FF-IIB-OCT17**: FinTech Futures, 23 Oct 2017: International Islamic Bank live; ICSFS "over 20 customers in Iraq. Among them are Al-Arabiya Islamic Bank, Al Janoob Islamic Bank, Bank of Baghdad, Taawen Islamic Bank and Noor Al Iraq Islamic Bank" (Wayback copy). https://web.archive.org/web/20210305042753/https://www.fintechfutures.com/2017/10/international-islamic-bank-live-with-new-core-banking-system/
- **FF-RAFIDAIN**: FinTech Futures, 13 Mar 2019: Rafidain Bank is understood to be implementing Oracle FLEXCUBE Universal Banking, Digital Experience and Payments (Wayback copy). https://web.archive.org/web/20200621034622/https://www.fintechfutures.com/2019/03/iraqs-rafidain-bank-revamps-tech-with-oracle/
- **RBI-2008**: Retail Banker International, 29 Feb 2008: Rafidain Bank turnkey contract with Misys and B-Plan; Misys competed against Temenos and Systems Access. https://www.retailbankerinternational.com/news/iraqs-banks-look-to-the-future/
- **WIKI-RAFIDAIN**: Wikipedia: Rafidain Bank (2007 B-Plan and Misys turnkey contract; 146 domestic branches). https://en.wikipedia.org/wiki/Rafidain_Bank
- **ORA-FLEXCUBE**: Oracle FLEXCUBE product page. https://www.oracle.com/financial-services/banking/flexcube/
- **ORA-UBS149**: Oracle FLEXCUBE Universal Banking 14.9.0.0.0 documentation library (document dates August 2026). https://docs.oracle.com/cd/G60251_01/index.html
- **ORA-OFSAA**: Oracle FLEXCUBE UBS - OFSAA Integration User Guide, release 14.9 (staging tables, EMXTRACT, read-only extraction tables). https://docs.oracle.com/cd/G60251_01/PDF/IF/IF_UBS-OFSAA.pdf
- **ORA-DATAMODEL**: Oracle FLEXCUBE UBS Data Model - Getting Started (reverse-engineering the schema; no packaged ER diagram). https://docs.oracle.com/cd/G60251_01/PDF/Extensibility/FCUBS-FD08-01-01-Data%20Model%20getting%20started.pdf
- **GC-TBI-2008**: Global Custodian, 10 Nov 2008: Trade Bank of Iraq selects Misys Equation, Trade Innovation TI Plus 2.0 and Equation Branch Automation. https://www.globalcustodian.com/?p=27781
- **TMI-TBI-2016**: Treasury Management International, 8 Feb 2016: TBI selects Misys FusionBanking Equation and Teller (projects in 2003 and 2009; contract signed in Baghdad on 14 October). https://treasury-management.com/news/trade-bank-of-iraq-tbi-selects-misys-to-provide-world-class-banking-solutions
- **FIN-POLLEN**: Finastra, 19 Jun 2026: Pollen Street Capital to acquire Universal Banking (core banking: Essence). https://www.finastra.com/press-media/pollen-street-capital-acquire-universal-banking
- **FIN-CAAS**: Finastra, 14 Sep 2023: Compliance as a Service (Fincom screening and ThetaRay monitoring, pre-integrated with Payments To Go). https://www.finastra.com/press-media/finastra-launches-compliance-service-help-banks-combat-financial-crime-instant-payments
- **FIN-TOTAL**: Finastra, 4 Jun 2021: Fusion Total Screening sanctions screening brochure. https://www.finastra.com/viewpoints/brochure/fusion-total-screening-sanctions-screening-solution-meet-embargo-and-anti-money
- **FINACLE-SITEMAP**: Infosys Finacle sitemap, Wayback snapshot 3 Jan 2026 (solution pages, client stories, App Center partners). https://web.archive.org/web/20260103080948/https://www.finacle.com/sitemap.xml
- **FINACLE-CROSSFRAUD**: Finacle App Center: CrossFraud AML solution (partner product). https://web.archive.org/web/2025/https://www.finacle.com/app-center-listing/app-center-detail/crossfraud-aml-solution/
- **FINACLE-NG**: Finacle App Center: NetGuardians fraud solution (plugs into core banking systems via connectors). https://web.archive.org/web/2025/https://www.finacle.com/app-center-listing/app-center-detail/beat-fraud-with-smarter-ai/
- **FINACLE-CORE**: Finacle Core Banking Solution page (cloud-native, microservices; no database named). https://web.archive.org/web/2025/https://www.finacle.com/solution/core-banking-solution/
- **FF-QURTAS**: FinTech Futures, 7 Nov 2017: Al Qurtas Islamic Bank live on BML Istisharat ICBS (Wayback copy). https://web.archive.org/web/20210305042753/https://www.fintechfutures.com/2017/11/al-qurtas-islamic-bank-live-with-new-core-banking-system-bml-istisharats-icbs/
- **FF-ICBS-2016**: FinTech Futures, 24 Oct 2016: Al Qurtas selects ICBS; other Iraqi users CBI, Erbil Bank, Minare Bank, NBI (Wayback copy). https://web.archive.org/web/20210305042753/https://www.fintechfutures.com/2016/10/new-bank-in-iraq-selects-bml-istisharats-icbs-core-banking-system/
- **INT-SITEMAP**: Intellect Design Arena sitemap (no AML product page, no Iraq page). https://www.intellectdesign.com/sitemap_index.xml
- **BOB-AR-2025**: Bank of Baghdad annual report 2025 (banking system "BANKS", SIRON scenarios, KYC and FATCA system, Laserfiche, Nucleus, RTGS and ACH linkage). https://bankofbaghdad.com.iq/Resources/8/Annual_Report_2025_Eng.pdf
- **ICS-BURGAN**: ICSFS release, Jul 2016: Burgan Bank (Kuwait; parent of Bank of Baghdad) live on ICS BANKS. https://icsfs.com/newsroom/burgan-bank-goes-live-on-ics-banks-from-ics-financial-systems/
- **PS-GATEPAY**: ProgressSoft, 22 Jan 2019: PS-GatePay (ACH, RTGS, cheque gateway integrating with the banks' CBS) selected by five more Iraqi banks after a procurement, nine in total. https://www.progressoft.com/news/banks-in-iraq-automate-their-clearing-and-settlement-processes-using-ps-gatepay
- **AMEEN-AR-2021**: Ameen Al-Iraq Islamic Bank annual report 2021 (Arabic, partial text layer): PDF p.32 systems table lists "PATH SOLUTION IMAL" and an AML system; PDF p.129 external auditor's AML paragraph (AML system supplied by path solutions, Word Check supplied by Refinitiv, CBI minimum scenarios "25"). https://alaminbankstorage.blob.core.windows.net/root/root/images/916122025_data%202021_compressed.pdf
- **AMEEN-AR-2022**: Ameen Al-Iraq Islamic Bank annual report 2022 (Arabic): PDF p.33 systems table (IMAL, AML); PDF p.50 auditor's AML paragraph (same wording as 2021). https://alaminbankstorage.blob.core.windows.net/root/root/images/916122025_data%202022_compressed.pdf
- **AMEEN-AR-2023**: Ameen Al-Iraq Islamic Bank annual report 2023 (Arabic, image-only scan): PDF p.43 systems table (IMAL, Western Union, AML); PDF p.60 auditor's AML paragraph (same wording). https://alaminbankstorage.blob.core.windows.net/root/root/images/916122025_data%202023_compressed.pdf
- **AMEEN-AR-2024**: Ameen Al-Iraq Islamic Bank annual report 2024 (Arabic, partial text layer): PDF p.46 systems table (IMAL, AML); PDF p.64 auditor's AML paragraph (supplier now written "Azento Software Middle East FZ LLC"). https://alaminbankstorage.blob.core.windows.net/root/root/images/1016122025_data%202024_compressed.pdf
- **AMEEN-REPORTS**: Ameen Al-Iraq Islamic Bank website, annual report downloads (2020 link returns BlobNotFound; 2025 link is a template file). https://aib.iq/about-AnnualReports
- **TIB-AR-2022**: Trans Iraq Bank annual report 2022 (Arabic): PDF p.70 auditor's AML paragraph (World Check from Refinitiv, AML system from capital banking, Firco Soft from a local group, Black List from Banks; 25 CBI scenarios). https://www.tib.iq/uploads/files/reports/annual-report-2022.pdf
- **TIB-AR-2023**: Trans Iraq Bank annual report 2023 (Arabic): PDF p.69 (IT section: systems linked to the "banks" system and goAML) and PDF pp.69-70 (auditor's AML paragraph; Pio-Tech contract for 81 scenarios). https://www.tib.iq/uploads/files/reports/annual-report-2023.pdf
- **TIB-AR-2024**: Trans Iraq Bank annual report 2024 (Arabic scan; audit letter dated 7 Apr 2025): PDF pp.47-48 auditor's AML paragraph (Pio-Tech contract for 96 scenarios; customers not risk-classified). https://www.tib.iq/uploads/files/reports/2024/annual-report-2024.pdf
- **GCB-AR-2024**: Gulf Commercial Bank annual report 2024 (Arabic, image-only): PDF p.108 auditor's item 4, accounting records kept per the electronic banking system (BANKS). https://www.gcb.iq/wp-content/uploads/2025/07/annual-report-2024-final.pdf
- **NBI-AR-2023**: National Bank of Iraq annual report 2023 (Arabic and English; PDF p.12 and p.38: T24 launch prepared and "AML Sirion"; p.39: goAML live; p.54: third-party contracts table incl. ICBS and profiling-system renewals; p.66: auditor's AML paragraph naming ICBS; p.179: Securities Commission disclosure item 26 on AML). https://www.nbi.iq/-/media/pdfs/investor-relations/annual-reports/english/national-bank-annual-report-2023.ashx
- **NBI-AR-2024**: National Bank of Iraq annual report 2024 (English; PDF p.13: T24 launched early 2024 and "a new anti-money laundering management system (AML Sirion)"; p.59: auditor's AML paragraph; p.177: Securities Commission disclosure item 26 on AML). https://www.nbi.iq/-/media/pdfs/investor-relations/annual-reports/english/english-report-soft-copy_compressed.ashx
- **CBSOL-HOME**: Capital Banking Solutions homepage (Wayback copy, 2 Jan 2023): "Since 1997" in Europe, Africa, Central Asia, Middle East and America; "Partner of 200+ clients"; Sumer, BCI and EBNA releases. https://web.archive.org/web/20230102130820/https://www.capital-banking.com/en/
- **CBSOL-COMPLIANCE**: Capital Banking Solutions, CapitalCompliance platform page (Wayback copy, 9 Dec 2022): AML transaction monitoring, name screening, FATCA, CRS, regulatory reporting; "more than 50 Banks"; on-cloud or on-premises; developed with Intelleval. https://web.archive.org/web/20221209131756/https://www.capital-banking.com/en/compliance/
- **CBSOL-CORE**: Capital Banking Solutions, core banking page (Wayback copy, 1 Oct 2022): CapitalBanker "running on Oracle technologies" and PowerBanker "running on IBM Power platforms". https://web.archive.org/web/20221001011931/https://www.capital-banking.com/en/core-banking/
- **CBSOL-SUMER**: Capital Banking Solutions release, 26 Aug 2022 (Wayback copy): Sumer Commercial Bank selects CapitalBanker core and CapitalDigital, extends CapitalCompliance with goAML and CRS; "more than 25 Iraqi retail, commercial and islamic banks". https://web.archive.org/web/20221001015216/https://www.capital-banking.com/en/sumer-commercial-bank-selects-capital-banking-solutions-for-its-core-and-digital-banking-solutions/
- **CBSOL-IMEIB**: Capital Banking Solutions release, 1 Feb 2022 (Wayback copy): IMEIB goes live with CapitalDigital; IMEIB implemented the vendor's "first Core banking system in Iraq" in 2006; "18 other Iraqi banks". https://web.archive.org/web/20221001022236/https://www.capital-banking.com/en/iraqi-middle-east-investment-bank-imeib-goes-live-with-capitaldigital-from-capital-banking-solutions/
- **CBSOL-FORRESTER**: Capital Banking Solutions release, 24 Sep 2021 (Wayback copy): vendor says it is a "Solid Player" in Forrester's Global Banking Platform Vendor Pyramid 2020. https://web.archive.org/web/20221001015648/https://www.capital-banking.com/en/capital-banking-solutions-a-solid-and-regional-player-in-forresters-global-banking-platform-deals-survey-2021/
- Local research files cross-referenced (not URLs): 01-iraq-regulatory-framework.md, 02-iraq-ml-tf-risk-landscape.md, 09-vendor-oracle-fccm.md, 11-vendor-other-incumbents.md, 12-vendor-ai-native-detection.md, 14-vendor-screening-engines.md, 25-core-banking-integration.md, 18-vendor-mena-iraq-market.md.
