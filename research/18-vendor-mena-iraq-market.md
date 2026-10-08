# 18 — Vendors: the real Iraqi and MENA market (who runs what, tenders, programmes, pricing, competitors)

Status: done (2026-10-06)

Part 4b of 4 of the vendor research. Companion file: 17-vendor-cbs-native-aml.md (CBS-native AML modules and CBS database
facts). Requirement IDs use the prefix VD-xx and are numbered across both files.

Evidence grading used in this file: [A] vendor or bank primary source (press release, annual report, case study, tender
notice); [B] reputable secondary source (trade press, regulator, rating agency); [C] weak (job advert, directory page,
single unverified mention, forum); [N] not found or not verified. Claims without a grade are quoted from the existing research files.

## Contents

1. What the MENAFATF 2024 MER and the CBI documents say about systems in use
2. Structure of the Iraqi banking sector (who the buyers are)
3. State banks: Rafidain, Rasheed, Trade Bank of Iraq (and the other state banks)
4. Leading private conventional banks
5. Islamic banks
6. Kurdistan Region banks
7. Foreign banks and branches operating in Iraq
8. AML, screening and data vendors present in Iraq (Eastnets, Temenos, Path, ICS, World-Check, others)
9. Local integrators in Baghdad and Erbil; Jordanian and Lebanese integrators
10. Public tenders, vendor press releases and job adverts naming systems (2001-2026)
11. Rafidain and Rasheed modernisation programmes
12. CBI programmes: payment infrastructure (national switch, ACH, RTGS, ISO 20022, wallets), the 2025 Banking Reform Initiative and external remittance control
13. Pricing and procurement realities
14. Regional peers: Jordan, UAE, Saudi Arabia, Egypt, Lebanon (who dominates)
15. What it means for AKR-TMS
16. Derived requirements (VD-xx)
17. Market map (bank | CBS | AML or screening system | source)
18. Competitor table (vendor | strengths in Iraq | weaknesses | likely price band)
19. Sources

## 1. What the MENAFATF 2024 MER and the CBI documents say about systems in use

Read in full from the MER text (iraq-mer-2024-en.txt, paragraph numbers as in the report) and from file 01 (CBI controls 2025).
The key finding for vendor strategy: **the MER and the CBI AML controls name only one commercial product, World-Check, and name no
AML, monitoring or core banking vendor at all.** The CBI's 2020-2025 achievements booklet adds two vendors for the CBI's own remittance
platform (Refinitiv and FinScan, section 12.3), and the reform FAQ names Gartner and Forrester as the test for core banking vendors (section 12.2),
but nothing tells a bank which AML tool to buy. Almost everything about vendors in this file therefore comes from vendor and bank sources, not from the regulator. A grep of the MER for Temenos, T24, Flexcube, Oracle, Actimize, Eastnets, Dow Jones, LexisNexis,
software and vendor returns nothing relevant.

| What the documents say | Source | Meaning for a vendor |
|---|---|---|
| "FIs supervised by the CBI (banks, exchange companies, and e-payment companies) have automated systems to monitor financial transactions ... based on various scenarios" and "All banks, exchange companies, and e-payment companies use a set of standardized scenarios developed and disseminated by the CBI. The number of these scenarios has increased to 56 for banks ... Previously, the number of scenarios was 34." | MER para 566 | Every bank already runs some automated monitor. The market is a replacement and upgrade market, not a greenfield one. The detection logic is the CBI's scenario list, so the incumbent tool is mostly a scenario runner. |
| "None of the institutions that were interviewed have developed additional scenarios ... which indicates that these institutions do not have sufficient expertise to develop scenarios" | MER para 566 | Banks cannot tune or extend scenarios themselves; a tool that makes scenario building and back-testing easy for a bank team is a differentiator. |
| "Each branch ... has a designated liaising officer who has been trained to use the automated systems for monitoring transactions. They initially examine the alerts ... then the employees of the AML/CFT central departments ... review" | MER para 567 | Existing tools are used in a two-tier branch plus central workflow. |
| "All banks in Iraq use commercial electronic systems that are automatically linked to international sanctions lists, allowing them to verify on a daily basis whether the designated name belongs to any of their clients." | MER para 442 (also para 509 for PF TFS) | Screening is commercial software, run daily on the customer base; a CBS-native or third-party filter, not a manual process. |
| "Most of the financial fines of 100,000 IQD were applied for each working day until the violating bank would acquire or activate an automated system to combat ML/TF." Warnings 85 and fines of about USD 1.5 million (2018-2022) included "failure to acquire a specialized automated system to monitor transactions that may involve ML/TF". | MER paras 661-662 | Buying a system was enforced by daily fines. Banks bought to satisfy the regulator; many installed the cheapest compliant tool. |
| "the effectiveness of these systems and the process of continuous monitoring, in general, do not seem to be sufficiently effective. This is evidenced by the significant lack of STRs" (said about non-bank FIs; for banks the MER blames low STR counts on a few banks being "keen not to file defensive reports") | MER paras 552, 567 | Detection effectiveness is not proven for banks either: 628 ML STRs from 74 banks in 2018-2022 (file 01 section 5). |
| CBI controls 2025, ch. 5 art. 5: the AML system must have "Direct link to the core banking system" and "Automatic Integration" with the screening platform; "Uses AI and machine-learning"; screening "through the World-Check system" with lists refreshed at least every 12 hours and Arabic plus English search | File 01 section 2d | The regulator has already written the read-only-CBS-link architecture into its rules. World-Check is named in the rules, so any screening product must either integrate it or show equivalence. |
| NBI's 2023 and 2024 annual reports, under "Additional Disclosures for Securities Commission Requirements", item 26 "Anti-Money Laundering and Counter-Terrorist Financing Regulations": "The bank is committed to implementing the electronic system, which includes the minimum required scenarios"; to "periodically and automatically updating the blacklists specified within the electronic system"; to "classifying customers according to their risk level within the electronic system"; and to "implementing a system for monitoring financial transactions within customer accounts based on the established scenarios" | NBI-AR-2023 (PDF p.179); NBI-AR-2024 (PDF p.177) | A four-point AML statement is part of a listed bank's annual disclosure to the Securities Commission, and the external auditors repeat the points in their reports (Ameen Al-Iraq and Trans Iraq Bank repeat all four; Bank of Baghdad's auditor describes its systems in similar terms). Evidence that the product produces automatically (scenarios running, list updates, risk classes, monitoring) is evidence the bank needs once a year. |
| CBI standards handbook 2025: AML tools are verified by a CBI-approved firm | File 01 section 2e | A vendor that can pass such a verification and supply the evidence pack has an advantage. |
| The FIU runs goAML (UNODC); the Office "has an automated system tied to the databases of banks, allowing it to identify the bank accounts belonging to individuals under investigation and their balances" | MER paras 179, 259, 261 and footnote 25 to para 291 | A CBI/FIU-side link into bank CBS databases already exists. Banks are used to the idea that an outside system reads CBS data. |
| E-payment companies, exchange companies and securities brokers "monitor transactions using electronic systems to track and prioritize alerts based on risk assessment" | MER paras 542-543 | Small non-bank FIs are a second buyer segment with simpler needs (560 exchange companies, 17 e-payment companies, file 02). |

Why this matters for AKR-TMS: the evidence says Iraqi banks buy AML software to comply, tune nothing, and file few STRs. A product that is
cheap to adopt, ships the CBI scenarios pre-configured, proves effectiveness with metrics, and reads the CBS directly addresses the
regulator's own criticism (MER paras 566-567) better than a generic enterprise suite.

AKR-TMS hooks:
- Ship the CBI 56-scenario pack (file 27) as a certified baseline and show alert, STR and closure metrics per scenario from day one.
- Provide a scenario builder with back-testing on the bank's own history so banks can add scenarios (the MER's named gap).
- Produce the "verification evidence pack" for a CBI-approved reviewer (rules, parameters, tuning log, test results).
- Position as the system "directly linked to the core banking system" in the exact words of CBI controls 2025 ch. 5 art. 5.

## 2. Structure of the Iraqi banking sector (who the buyers are)

| Fact | Value | Grade, source |
|---|---|---|
| Licensed institutions, CBI slide "Licensed financial institutions in Iraq" (booklet 2020-2025, figures as of late 2025) | 7 state banks; 49 private ("ahli") banks; 11 foreign bank branches; digital banks "under establishment"; 18 e-payment companies; 3 microfinance companies; exchange companies 118 (class A), 60 (B) and 280 (C) after 1,500 licences were merged (75% fewer companies) | [A] CBI-BOOKLET |
| Earlier counts for comparison | MER 2022: 76 banks, assets about USD 128bn, 7 state, rest private and foreign branches (file 02); a Temenos success story in 2024 says "There are 70 banks in Iraq" | [A] MER; TEM-ZC-SS |
| Concentration | Rafidain: "approximately 45% of Iraq's banking assets" and 146 domestic branches (2017 figure); about 17% of all bank branches are in the Kurdistan Region (MER); in external remittances "75% concentrated in 4 main banks" | [B] WIKI-RAFIDAIN; [A] MER; CBI-BOOKLET |
| Banks able to send external remittances | Fell from 32 (2024) to 20 (2025) "because a number of banks were deprived" (the dollar bans) | [A] CBI-BOOKLET |
| Consolidation under way | CBI Banking Reform Initiative 2025: every private and foreign bank must choose continue, merge or liquidate; minimum paid-in capital IQD 300bn rising to 400bn for continuing private banks; annual non-refundable reform fee IQD 3.168bn (about USD 2.4m) per bank; first assessment deadline 30 Sep 2026 (section 12.2). Al-Taif Islamic Bank was placed in receivership on 3 Sep 2026 (Banking Law 94/2004 art. 59, 18-month custodian). | [A] CBI-REFORM-CIRC; [B] IBN-TAIF |
| Non-bank buyers | 18 licensed e-payment companies including wallets; NBI bought 51% of Iraq Electronic Gate Company (licensed e-payment provider) and 51% of Digital Future Company in 2025 | [A] CBI-BOOKLET; NBI-AR-2025 |

Reading for AKR-TMS: the number of bank buyers will fall from about 67 banks (7 + 49 + 11) toward perhaps 25 to 40 over 2026-2028 (an estimate, not a source figure: the paid-in capital and fee hurdles
are the filter), while the survivors are forced to buy or prove AML and core-banking compliance. The sales target is the continuing private banks, the seven state banks (which follow the Ministry of
Finance and EY, not the private-bank track), the foreign branches, the digital banks and the 18 e-payment companies.

## 3. State banks: Rafidain, Rasheed, Trade Bank of Iraq (and the other state banks)

| Bank | Core banking | AML or screening system | Programme and status | Grade, source |
|---|---|---|---|---|
| Rafidain Bank (largest; 146 domestic branches) | Misys Bankmaster (legacy) then Fusionbanking from 2012; deal "signed" in 2018 to move to Oracle FLEXCUBE Universal Banking, Oracle Banking Digital Experience and Oracle Banking Payments (unnamed Iraqi bank in Oracle's Q2 FY2019 results); **go-live not confirmed anywhere** | None found. A professional partnership with the US firm K2 Integrity was signed in Washington (reported 18 Aug 2025): "anti-money laundering and counter-terrorist financing measures, implementation of compliance systems aligned with international standards, and strengthening regulatory infrastructure across the Iraqi banking sector" under Cabinet Resolution No. 23274 of 2023 | EY restructuring contract approved by Cabinet 3 Jan 2024; to be re-formed as "First Rafidain Bank" with the state holding "less than 24 per cent"; plan due end-2025; EY and the Finance Minister reviewed progress on 26 Jan 2026 including "upgrades to Iraq's financial information infrastructure" and "automation of financial systems and digital transformation" | [B] FF-RAFIDAIN; RBI-2008; IBN-K2; IBN-REFORM-JUN25; IBN-EY-2026 |
| Rasheed Bank | Its website says it "acquired the comprehensive banking system (Core Bank System)" to become a universal bank with one customer view across branches, plus a data centre and a DR centre "approved by reputable international bodies"; **vendor not named** | None found | EY restructuring (same contracts); PM review with EY (reported 25 Mar 2025); removed from the UN and UK sanctions lists in May 2022; lending frozen by liquidity shortage (Feb 2026 report) | [A] RASHEED-ABOUT; [B] IBN-PM-MAR25; IBN-RASHEED-SANC |
| Trade Bank of Iraq (TBI) | Misys Equation with Trade Innovation "TI Plus 2.0" and Equation Branch Automation at head office (2008, 6 branches); "two major projects with TBI in 2003 and 2009"; upgrade to FusionBanking Equation and Teller (contract signed in Baghdad on 14 Oct, probably 2015). Finastra announced on 19 Jun 2026 the sale of its Universal Banking core business (file 17 section 7); the release names Essence, so whether Equation customers move with it is not stated. | None found. TBI appears as a Pio-Tech reference (product not stated) | Engaged K2 Integrity and KPMG separately (June 2025) to restructure "internal and external operations"; MoU with ESDS (India) on data centre, cloud and fintech (June 2019); one of four banks with USD correspondent balances in the 2023 CBI mechanism | [A] GC-TBI-2008; TMI-TBI-2016; PIO-REF; CBI-BOOKLET; [B] IBN-REFORM-JUN25; IBN-TBI-ESDS |
| Industrial Bank, Real Estate Bank, Agricultural bank | Not found | Not found | EY contract (Jan 2024) to study merging the three into one bank | [B] IBN-EY-JAN24 |
| Central Bank of Iraq itself | BML Istisharat ICBS (CBI listed as a user in 2016) | Refinitiv and FinScan on the external remittance platform; goAML (UNODC) for the FIU | See sections 8 and 12 | [B] FF-ICBS-2016; [A] CBI-BOOKLET |

Lessons for AKR-TMS (state banks):
- The state banks are not on the private-bank reform track; their modernisation is driven by the Cabinet and EY, with K2 Integrity (US) and KPMG as compliance and restructuring advisers. A product that will be assessed by "a company approved by the CBI" is likely to be assessed
  against K2 Integrity-style frameworks, not against a local checklist.
- The CBS question is open at both big banks (Rafidain: Fusionbanking or FLEXCUBE; Rasheed: unknown). Any commitment to either bank needs a discovery phase on the production CBS before a price.
- A state bank will procure through a government tender or direct award by Cabinet resolution. No public tender notice for AML systems at the state banks was found (section 10).

## 4. Leading private conventional banks

| Bank | Core banking | AML, screening and related systems named | Notes | Grade, source |
|---|---|---|---|---|
| Bank of Baghdad (Burgan Bank group) | "the banking system (BANKS)" updated "according to the latest version approved by the processing company". The name matches ICS BANKS (inference: ICSFS listed the bank as a customer in 2017 and parent Burgan Bank went live on ICS BANKS in May 2016) | **SIRON**: "monitors daily banking operations according to scenarios developed to monitor suspicious and unusual operations, which are based on scenarios approved by the Central Bank of Iraq" (inference: the FICO TONBELLER Siron AML product; the report writes only "SIRON"); **FIRCO**: "a system linked to the screens of internal and external transfers ... checks the name of the transferor and the beneficiary when making the transfer before execution ... The transfer is stopped and the official is alerted" (FircoSoft filtering). Also a "Nucleus" risk-assessment system, a KYC and FATCA system "linked ... with the banking system", Laserfiche archiving, Kaspersky, a credit bureau link, ECL (IFRS 9) automation, a move of card processing to a local switch provider in Jan 2025, hot and cold DR sites in Iraq and Jordan, RTGS and ACH link to the CBI | Annual reports 2022, 2023 (auditor's AML section), 2024, 2025. A Pio-Tech reference. | [A] BOB-AR-2022; BOB-AR-2023; BOB-AR-2025; ICS-BURGAN; [B] FF-IIB-OCT17; [A] PIO-REF |
| National Bank of Iraq (NBI; Capital Bank of Jordan owns the majority) | **BML Istisharat ICBS until the end of 2023**: the 2023 auditor's AML paragraph calls the activated banking system "ICBS" (it holds the customer risk-rating field linked to the AML profiling system) and the 2023 contracts table lists an "ICBS System Renewal 2023" with Istisharat (IQD 64.7m a year, about USD 49k) plus an "Escrow Agreement Renewal 2023" with the same vendor (IQD 4.585m). **Temenos Transact (T24)** live from early 2024 ("T24 Banking System" in the 2024 and 2025 reports); Temenos Payments Hub; Finastra trade finance | "OFAC Search Engine" (local and international watchlists, screens new clients before account opening, existing accounts daily or monthly, and incoming and outgoing transfers; vendor not named). **"AML Profiling Electronic System"** ("monitors transactions based on predefined scenarios to generate daily/monthly alerts. The scenarios required by the Central Bank are included"): the 2023 contracts table lists a "Profiling system renewal 2023" with **Infrasoft** (IQD 4.585m, about USD 3.5k a year; presumably this system, the table does not say). A new system, written **"AML Sirion"**, was prepared in 2023 and "launched" in early 2024 (chairman's letters 2023 and 2024; by name FICO TONBELLER Siron, as at Bank of Baghdad: inference). The auditor's paragraph still says "AML Profiling" in 2024 and 2025, so whether the two are one system after 2024 is not stated. **goAML** went live in 2023 "according to the instructions of the Central Bank of Iraq". New SWIFT MX standard and IFRS 9 automation in 2025 | A Pio-Tech reference. Group Temenos platform; first in the group on Temenos Payments. The contracts table also shows "Mule Soft Integration" with ITSS Solutions (IQD 88.8m) | [A] TEM-NBI; NBI-AR-2023; NBI-AR-2024; NBI-AR-2025; PIO-REF; [B] IBN-NBI |
| Mansour Bank for Investment (QNB owns 51%) | ICS BANKS (replacement of "core banking and IT systems", signed Nov 2015) | Not found | A Pio-Tech reference | [A] ICS-MANSOUR; PIO-REF |
| Ashur International Bank | ICS since 2007; IFRS 9 module on ICS BANKS in 2023 | Pio-Tech "Risk-Based Approach (RBA) solution" (Jan 2024) | On the CBI's Feb 2024 dollar-ban list | [A] ICS-SUMMIT23; PIO-ASHUR |
| Hammurabi Commercial Bank | ICS BANKS Universal (Dec 2018) | Not found; a Pio-Tech reference | On the Feb 2024 dollar-ban list | [A] ICS-HAMMURABI; PIO-REF |
| First Iraqi Bank (digital bank) | ICS BANKS Digital (RFP, selected Mar 2021, live Aug 2021) | Pio-Tech "credit bureau" reporting solution (Jan 2024) | First fully digital Iraqi bank | [A] ICS-FIB-SEL; ICS-FIB-LIVE; PIO-FIB |
| Banque Libano-Francaise (Iraq) | Temenos T24 R17 (July 2018) | Not found | Lebanese group bank | [B-] FF-BLF |
| Trans Iraq Bank (TIB) | "the banks system": the 2023 annual report says the bank's systems (the customer-account registration systems and the goAML system) were linked "per the requirements of the Central Bank of Iraq" with the "banks" system, and that "auditing" was enabled on the banking-system database. Probably ICS BANKS (inference from the product name, as at Bank of Baghdad) | The auditor's AML paragraph in the 2022, 2023 and 2024 reports names four tools in 2022 and five in 2023 and 2024: an **AML system "supplied by capital banking"** (Capital Banking Solutions, whose AML product is CapitalCompliance, section 8); **Firco Soft** "supplied by" "United Engineers Group" (an Iraqi supplier, not identified further; the 2022 report says all systems were linked to the banking system "except the Firco Soft system"); **World Check** from Refinitiv; a **Black List** system "supplied by Banks"; and, from the 2023 report, **goAML**. On scenarios the system "includes the minimum scenarios required by the Central Bank of Iraq, amounting to 25 scenarios" (all three years) and "a contract was concluded with PIO-TECH to implement (81) scenarios inside the system" (2023), then "(96)" (2024). Customers "are classified according to risk" in 2022 and 2023 but "were not classified according to risk" in 2024 | The only bank found whose auditor lists five separate AML-side tools and a named scenario implementer (Pio-Tech). The sentence does not say which listed system hosts the 81 and 96 scenarios. Quotes translated from Arabic by the researcher. A Pio-Tech reference | [A] TIB-AR-2022; TIB-AR-2023; TIB-AR-2024; PIO-REF |
| Gulf Commercial Bank (GCB) | "BANKS": the 2024 auditor's report says the bank relies on electronic systems to record transactions and keeps all accounting records "according to the electronic banking system (BANKS)". Probably ICS BANKS (inference from the product name) | None named in the auditor's report | A Pio-Tech reference | [A, inference] GCB-AR-2024; PIO-REF |
| Region Trade Bank (ex-Emerald), Investment Bank of Iraq, Mosul Bank, Bank al Etihad Iraq | Not found | Listed as Pio-Tech references (products not stated) | CBS unknown | [A] PIO-REF |
| Iraqi Middle East Investment Bank (IMEIB) | **Capital Banking Solutions core since 2006**: the bank's chairman says IMEIB "implemented CBS's first Core banking system in Iraq" in 2006; CapitalDigital went live in Jan 2022 (vendor release; the product line of the 2006 core is not stated) | Not found | Chairman: "18 other Iraqi banks chose CBS products" | [A, vendor release] CBSOL-IMEIB |
| Sumer Commercial Bank | **Capital Banking Solutions**: partnership "since 2015"; the new CapitalBanker core and CapitalDigital were selected in Aug 2022 | **CapitalCompliance**, "extended ... to include GoAML and CRS to cover the compliance and tax evasion requirements as per the Central Bank of Iraq regulation" (Aug 2022) | The vendor adds that it "is currently servicing more than 25 Iraqi retail, commercial and islamic banks" | [A, vendor release] CBSOL-SUMER |
| Credit Bank of Iraq, Commercial Bank of Iraq, Union Bank of Iraq, Warka, North Bank, Dar Es Salaam Investment Bank | Not found in any source reachable without a search engine | Not found | Needs bank-by-bank discovery | - |

What the Bank of Baghdad, NBI, Trans Iraq Bank and Ameen Al-Iraq statements prove about the market:
1. The four private banks whose audited reports name their tools run **separate AML tools next to the CBS**, and often several: Siron for monitoring and Firco for transfers at Bank of Baghdad; a separate "AML profiling system", an "OFAC search engine" and, from 2024, "AML Sirion" at NBI; an AML system from one vendor, FircoSoft through a local supplier, World-Check and goAML at Trans Iraq Bank; the iMAL vendor's AML plus World-Check at Ameen Al-Iraq. This is exactly the pattern the CBI's 2025 controls describe (AML system "directly linked" to the CBS) and it means the typical Iraqi bank holds two to five tools for one compliance job.
2. The Siron at Bank of Baghdad is configured with "scenarios approved by the Central Bank of Iraq", and the 2025 report says the bank "continued to develop and activate some special scenarios in the SIRON system" to "increase the effective control over the suspicious indicators": this is the MER's finding of tool use without self-built scenarios, seen from inside.
3. Pre-release screening exists in practice (Firco at Bank of Baghdad stops a transfer before execution and alerts an official), so AKR-TMS phase 2 must integrate with or replace an existing hook, not introduce a new idea.
4. The Siron product is FICO TONBELLER; FICO sells it as a licence; a Siron bank has a licence, annual maintenance and a vendor upgrade path to pay for. File 11 covers the product.

AKR-TMS hooks (section 4):
- Build a "displacement" story for Siron and Firco banks: import their scenario configuration and alert history, run in parallel read-only, show side-by-side alert quality (file 29 parallel-run policy).
- Do not assume a bank has no AML tool: ask for the named products in the first meeting; Bank of Baghdad, NBI, Trans Iraq Bank and Ameen Al-Iraq all state them in their audited annual reports, which any CBI-approved assessor will read. Read each bank's last three annual reports before the first meeting: the auditor's AML paragraph lists the tools, their suppliers and the scenario count.

## 5. Islamic banks

Islamic banks are the largest group of newly licensed Iraqi banks since 2016 and the group most exposed to the dollar bans (file 01 section 4: most of the 13 banks barred in 2024-2025 are Islamic). Their CBS choices
are therefore a direct map of AKR-TMS's first adapters.

| Bank | Core banking (vendor) | Date | AML or reporting tool seen | Grade, source |
|---|---|---|---|---|
| Al-Arabiya Islamic Bank (probably the "Arabia Islamic Bank" on the Feb 2024 ban list) | ICS BANKS Islamic | Feb 2017 | Pio-Tech reference | [A] ICS-ARABIYA; PIO-REF |
| Al Janoob Islamic Bank (on the Feb 2024 ban list), Taawen Islamic Bank, Noor Al Iraq Islamic Bank | ICS BANKS | 2017 | Noor Islamic Bank is a Pio-Tech reference | [B] FF-IIB-MAR17; [A] PIO-REF |
| International Islamic Bank (IMTB) | ICS BANKS Islamic (signed Mar 2017, live Oct 2017) | 2017 | **Pio-Tech goAML** system live (news 15 Apr 2024): "goAML reports ... require automation and preparation of data that is either uncollected or resides in multiple business areas" | [B] FF-IIB-MAR17; [A] PIO-IMTB |
| Al-Thiqa Islamic Bank | ICS BANKS Islamic (signed Sep 2017, live Dec 2017) | 2017 | Pio-Tech reference | [A] ICS-THIQA; PIO-REF |
| Al Mustashar Islamic Bank | ICS BANKS Islamic (live Jan 2019) | 2019 | Pio-Tech reference | [A] ICS-MUSTASHAR; PIO-REF |
| UR Islamic Bank | ICS BANKS Islamic (replaced a legacy system; live Feb 2022; HQ plus 5 branches) | 2021-22 | - | [A] ICS-UR-SEL; ICS-UR-LIVE |
| Al Sanam Islamic Bank (on the Feb 2025 ban list) | ICS BANKS Islamic with digital banking and ERP | Jul 2022 | Pio-Tech reference | [A] ICS-SANAM; PIO-REF |
| Al-Ataa Islamic Bank (8 branches) | ICS BANKS Islamic | Aug 2022 | Pio-Tech reference | [A] ICS-ATAA; PIO-REF |
| Zain Iraq Islamic Bank | Path iMAL (competitors named in the selection: Temenos, Misys, Oracle FS, ICS, Capital Banking Solutions) | Oct 2017 | - | [A] FFNEWS-ZAIN |
| Al-Rajih Islamic Bank | Path iMAL (chosen over ICS) | Dec 2017 | Pio-Tech reference (Rajih Islamic Bank) | [B] FF-ALRAJIH; [A] PIO-REF |
| Ameen Al-Iraq Islamic Bank (probably the "Amin Iraq for Islamic Investment and Finance" on the Feb 2025 ban list) | Path iMAL: signed Apr 2019 [B]; "IMAL" is listed as the "electronic banking system" in the bank's 2021 to 2024 annual reports [A] | Apr 2019; in use 2021-2024 | **Azentio (formerly Path Solutions) AML system plus Refinitiv "Word check"**, "linked to the banking system"; the auditor writes that it "includes the minimum scenarios required by the Central Bank of Iraq, amounting to 25 scenarios" (same paragraph in each report; supplier named "path solutions" in 2021 to 2023 and "Azento Software Middle East FZ LLC" in 2024) | [B] FF-AMEEN; [A] AMEEN-AR-2021; AMEEN-AR-2022; AMEEN-AR-2023; AMEEN-AR-2024; AMEEN-REPORTS |
| Kurdistan International Islamic Bank (KIIB/KIB; on the Feb 2024 ban list) | Path iMAL (2024) plus Azentio AMLOCK (2025) | 2024-25 | **Azentio AMLOCK** (all branches) | [A] AZ-KIB-IMAL; AZ-KIIB-AML |
| Al Qurtas Islamic Bank (Erbil) | BML Istisharat ICBS | Nov 2017 | - | [B] FF-QURTAS |
| Iraqi Islamic Bank for Investment and Development (HQ Baghdad, 14 branches) | Not found | - | **Pio-Tech goAML** (news 25 Feb 2024) | [A] PIO-IIB |
| Elaf Islamic, Nasik Islamic, Asia Iraq Islamic, Taif (in receivership Sep 2026) | Not found | - | Pio-Tech references | [A] PIO-REF; [B] IBN-TAIF |

Observations:
1. By named count in this research, **ICS BANKS Islamic is in 10 Iraqi Islamic banks, Path iMAL in 5, BML ICBS in 1, Temenos in none, Oracle FLEXCUBE in none** (the first four are vendor or trade-press statements; the count is a floor, not a census).
2. The pattern matches vendors' strategies: ICS wins greenfield banks with 2-3 month "record" implementations; Path wins with AAOIFI certification and Islamic profit-calculation depth.
3. Five of the thirteen banks barred from dollars in 2024-2025 run ICS BANKS (Ashur, Al Janoob, Hammurabi, Al-Sanam and, by name match, Arabia Islamic) and two run iMAL (KIIB and, by name match, Ameen). The CBI says its reform standards "address the root causes underlying the international restrictions on foreign currencies on many banks in Iraq" (section 12.2); a CBS change at these banks is possible within the 2028 window.
4. Of the six iMAL banks named in this research (Cihan, KIIB, Zain Iraq, Al-Rajih, Ameen, IDB), two run the iMAL vendor's own AML engine (Ameen and KIIB); Pio-Tech lists Cihan and Al-Rajih as references, so an iMAL bank is not automatically an Azentio AML bank. Ameen's AML system appears in its annual report from 2021, well before the Feb 2025 ban list on which its name probably appears, so the dollar ban did not trigger that purchase.

## 6. Kurdistan Region banks

About 17% of bank branches are in the Kurdistan Region, which is supervised by the CBI (Erbil branch) under the same AML law (file 01 section 9). CBS evidence:

| Bank | Core banking | AML or reporting | Source |
|---|---|---|---|
| Cihan Bank (Erbil; Islamic window) | Path iMAL: Path's first Iraqi customer, "signed in 2009" | Pio-Tech reference | [B] FF-AMEEN; [A] PIO-REF |
| Kurdistan International Islamic Bank | Path iMAL (2024) | Azentio AMLOCK (2025) | [A] AZ-KIB-IMAL; AZ-KIIB-AML |
| Erbil Bank, Minare Bank | BML Istisharat ICBS (2016 user list) | not found | [B] FF-ICBS-2016 |
| Al Qurtas Islamic Bank (Erbil) | BML Istisharat ICBS (live Nov 2017) | not found | [B] FF-QURTAS |
| Kurdistan International Bank, Erbil-based branches of Baghdad banks | Not found | not found | - |

Kurdistan banks therefore sit on **different CBS vendors** (Path, BML) from the Baghdad-based majority (ICS), with names that need Kurdish-Sorani spelling support (file 22) and local salary-localisation flows (file 01 section 9).
Azentio's AML engine is confirmed at two Iraqi banks: KIIB in Erbil (2024-25) and Ameen Al-Iraq in Baghdad (in its annual reports since 2021, section 5), so buying the iMAL vendor's AML is not a Kurdistan-only pattern.

## 7. Foreign banks and branches operating in Iraq

| Bank | Evidence | Source |
|---|---|---|
| Banque Libano-Francaise (Iraq) | Temenos T24 R17 upgrade (July 2018), first branch opened Oct 2012 | [B-] FF-BLF |
| Bank of Baghdad (Burgan Bank, Kuwait, owns the majority) | ICS BANKS (section 4); Burgan itself is on ICS BANKS since May 2016 and a Pio-Tech reference | [A] ICS-BURGAN; PIO-REF |
| National Bank of Iraq (Capital Bank of Jordan) | Group core on Temenos; Finastra trade finance | [A] TEM-NBI; [B] IBN-NBI |
| Mansour Bank (QNB 51%) | ICS BANKS | [A] ICS-MANSOUR |
| Ziraat Bank, VakifBank (Turkish state banks) | Listed as Pio-Tech references in Iraq (CBS not stated; presumably group systems) | [A] PIO-REF |
| Housing Bank for Trade and Finance (Jordan) | A Finacle client in Jordan (client story and press release on the Finacle site) and a Pio-Tech reference; its Iraqi CBS not found | [A] FINACLE-SITEMAP; PIO-REF |
| Standard Chartered, Arab Bank, QNB Iraq, other branches | No Iraqi-specific CBS or AML evidence found | - |

Foreign branches face the same reform standards on a two-path model (continue or liquidate), with an annual reform fee of USD 600,000 and paid-in capital of USD 80m equivalent, a legal undertaking due by 30 Nov 2025 and capital proof by 31 Dec 2025
[A] CBI-FOREIGN-CIRC. Group-owned branches usually run the parent's CBS and AML tools, so for them AKR-TMS is a local overlay (CBI scenario pack, Arabic names, goAML) on top of group systems, not a replacement.

## 8. AML, screening and data vendors present in Iraq

Evidence ladder: bank statements in audited annual reports (strongest), vendor press releases naming a bank, vendor reference pages, CBI statements. Absence of a vendor below means none was found in the sources reachable without a search engine.

| Vendor and product | What the evidence shows in Iraq | Grade, source |
|---|---|---|
| **FICO TONBELLER Siron** (AML monitoring) | Bank of Baghdad runs "SIRON", daily monitoring "according to scenarios ... approved by the Central Bank of Iraq" (annual reports 2022 and 2023; scenarios extended in 2024 and 2025). National Bank of Iraq: "AML Sirion", prepared in 2023 and launched in early 2024 as "a new anti-money laundering management system" next to its Temenos migration | [A, inference on the product name] BOB-AR-2023; BOB-AR-2025; NBI-AR-2023; NBI-AR-2024 |
| **FircoSoft Firco** (transfer and name filtering; today part of LexisNexis Risk Solutions, file 14) | Bank of Baghdad: "FIRCO ... system linked to the screens of internal and external transfers ... checks the name of the transferor and the beneficiary ... before execution". Trans Iraq Bank: a "Firco Soft" system "supplied by" "United Engineers Group", an Iraqi supplier (2022 to 2024 reports; in 2022 it was the one system not linked to the banking system). Two banks, one of them through a local reseller | [A, inference] BOB-AR-2022; BOB-AR-2023; TIB-AR-2022; TIB-AR-2024 |
| **Azentio AMLOCK / FCM** (with Path iMAL; Azentio was formerly Path Solutions) | **Ameen Al-Iraq (Baghdad)**: the auditor's paragraph in the 2021 to 2024 annual reports names an "AML system supplied by path solutions" (2021 to 2023) or by "Azento Software Middle East FZ LLC" (2024), "linked to the banking system", carrying "the minimum scenarios required by the Central Bank of Iraq, amounting to 25 scenarios". **KIIB (Erbil)**: iMAL plus AMLOCK, "fully integrated across all ... branches" (Feb 2025). Vendor says FCM works with "20+ core banking systems" | [A] AMEEN-AR-2021; AMEEN-AR-2022; AMEEN-AR-2023; AMEEN-AR-2024; AZ-KIB-IMAL; AZ-KIIB-AML; AZ-FCM-PDF |
| **Capital Banking Solutions CapitalCompliance** (Lebanese-linked vendor, releases datelined Paris; also a core banking vendor, section 9) | **Trans Iraq Bank**: the auditor names an "AML system supplied by capital banking" in the 2022, 2023 and 2024 annual reports (the bank's core is a different product, probably ICS BANKS). **Sumer Commercial Bank**: vendor release of Aug 2022, CapitalCompliance "extended ... to include GoAML and CRS" as per the CBI. Platform: AML transaction monitoring, name screening, FATCA, CRS and regulatory reporting, on-cloud or on-premises, "used by more than 50 Banks" worldwide (vendor claim). Vendor also claims "more than 25 Iraqi retail, commercial and islamic banks" as clients of its products | [A] TIB-AR-2022; TIB-AR-2023; TIB-AR-2024; [A, vendor] CBSOL-SUMER; CBSOL-COMPLIANCE; CBSOL-IMEIB |
| **Pio-Tech Bank-BI AML, goAML management system, RBA (risk-based assessment), FATCA, credit bureau** (Jordan) | 29 banks operating in Iraq (27 Iraqi banks plus the Ziraat and VakifBank branches) and 6 Iraqi non-bank institutions (Asia Pay, FastPay, Iraq Stock Exchange, Tabadul, ZainCash, CSC Iraq) are listed as references (product not stated; count re-checked by the lead from the page's embedded data, 2026-10-06), including Bank of Baghdad, NBI, Ashur, Mansour, Hammurabi, First Iraqi Bank, Iraqi Islamic Bank, Rajih, Cihan, TBI, plus FastPay, Asia Pay, Tabadul and the Iraq Stock Exchange. Dated releases in 2024: Ashur RBA (10 Jan), FastPay RBA (17 Jan), First Iraqi Bank credit bureau (28 Jan), Iraqi Islamic Bank goAML (25 Feb), International Islamic Bank goAML live (15 Apr). Its goAML page: "Dynamic Scenario-Builder and Rules Engine", "Utilities to Adjust, Monitor, and Validate Reports". Its AML page: transaction monitoring, user-defined scenarios, sanctions screening, case management and API integration. 2026: "AI-Powered AML" with a "GenAI Scenario Assistant", and a "Names Bilingual Translator" for Arabic names. At Trans Iraq Bank the auditor reports a contract with PIO-TECH "to implement (81) scenarios inside the system" (2023 report) and "(96)" (2024), on top of the 25 CBI minimum scenarios, so Pio-Tech also works as a scenario implementer inside another supplier's AML system. Pio-Tech's data model is the "Bank-BI" data warehouse ("a unified internationally recognized banking data model"), the same design as AKR-TMS's canonical model. | [A] PIO-REF; PIO-ASHUR; PIO-FASTPAY; PIO-FIB; PIO-IIB; PIO-IMTB; PIO-AML; PIO-GOAML; PIO-AI; PIO-NAMES; TIB-AR-2023; TIB-AR-2024 |
| **LSEG / Refinitiv World-Check** (risk data) | Named in the CBI AML controls 2025 (file 01); partnership with Iraqna IBTS "to provide Iraqi financial institutions with risk screening and compliance services" (10 Mar 2020; "The lack of accurate data and reliable sources pose significant challenges when it comes to identifying high risk individuals and organizations in Iraq"); the CBI itself contracted Refinitiv in Jan 2023 to monitor outgoing electronic remittances. Two banks name it in their annual reports: Ameen Al-Iraq (a "Word check system supplied by Refinitiv", 2021 to 2024) and Trans Iraq Bank (the same words, 2022 to 2024), each next to its AML system | [A] LSEG-IRAQNA; CBI-BOOKLET; AMEEN-AR-2021; AMEEN-AR-2024; TIB-AR-2022; TIB-AR-2024 |
| **FinScan** (Innovative Systems; name screening) | The CBI's external-remittance platform screens "names daily/monthly/annually for each company ... electronically via FinScan" | [A] CBI-BOOKLET |
| **UNODC goAML** (FIU platform) | Platform launched in Iraq in 2022; the CBI booklet counts 415 entities connected and 242,884 cash transaction reports received in 2023-2025 | [A] CBI-BOOKLET |
| **Eastnets SafeWatch** | No Iraqi bank case study; Eastnets' sitemap has no Iraq page; file 14 saw "QI Card" among SafeWatch customer logos (not confirmed: [C]) | [C] file 14 |
| **Temenos FCM** | No Iraqi deployment; ZainCash says it wants AML "from within the Temenos platform" | [A] TEM-ZC-SS |
| Oracle FCCM, SAS, NICE Actimize, Quantexa, ThetaRay, Feedzai-class vendors | No Iraqi customer, partner or office found (files 08, 09, 10, 12); Israeli-headquartered vendors (NICE Actimize, ThetaRay) face the 26 May 2022 law criminalising relations with Israel (file 12 section 2.1) | [A] files 08-12 |
| Consultancies that shape the buying criteria | K2 Integrity (Rafidain, CBI remittance review, CBI training), EY (state banks, CBI non-USD remittance review), KPMG (TBI), Oliver Wyman (CBI private-bank reform, 2-3 years); none sells software | [B] IBN-K2; IBN-REFORM-JUN25; [A] CBI-BOOKLET |
| Iraqna IBTS (Baghdad) | "exclusive and super-agent in Iraq" for ACAMS, ACFCS, Kryterion, Laserfiche and a "TRABBIT Financial Crimes Solution" (not identified further), plus OPES FATCA software; "PMO for 8 leading banks"; rating advisor to 13 banks | [A] LSEG-IRAQNA |

What this says about competition:
1. **The Iraqi AML market is already served by a regional vendor (Pio-Tech, Jordan) and by a few enterprise tools at the largest private banks (Siron at Bank of Baghdad and NBI, Firco at Bank of Baghdad and Trans Iraq Bank).** AKR-TMS will not meet an empty field. Its closest comparable is Pio-Tech: it reads data from the bank through a data warehouse, ships goAML and RBA, and is already referenced at 29 Iraqi banks.
2. The CBS vendors that sell AML have four Iraqi AML deployments between them: Azentio next to iMAL at Ameen Al-Iraq (since at least 2021) and KIIB (2024-25), and Capital Banking Solutions at Trans Iraq Bank (annual reports 2022 to 2024) and, in its own release, at Sumer Commercial Bank (goAML and CRS added in 2022). Temenos has none in Iraq.
3. The global suites (Actimize, FCCM, SAS) have no Iraqi presence at all.
4. The CBI's own platform runs Refinitiv plus FinScan with K2 Integrity and EY as human reviewers for external remittances: the regulator has set a reference standard for name screening quality and for human review.

AKR-TMS hooks (section 8):
- goAML export, RBA/customer-risk-rating and credit-bureau reporting are table stakes in Iraq because a Jordanian vendor already sells them as products: ship goAML with STR quality grading and CBI scenario coverage in the first release.
- Integrate World-Check (the CBI names it) and FinScan-class matching quality as the reference for Arabic name matching tests (file 22).
- Treat Pio-Tech as a possible partner as well as a rival: 29 references show it has bank-by-bank access to CBS data that AKR-TMS needs too. Do not assume a bank's data warehouse exists; ask.
- Listed Iraqi banks make a standard four-point AML statement in the annual report (Securities Commission disclosure item 26 at NBI: scenarios, automatic list updates, customer risk classes, monitoring) and the external auditor adds a paragraph naming the systems, their suppliers, the link to the banking system and, at some banks, the number of CBI minimum scenarios (Bank of Baghdad 2022-2023, NBI 2023-2025, Ameen Al-Iraq 2021-2024 and Trans Iraq Bank 2022-2024 all do this). Ameen's and Trans Iraq Bank's auditors, who are different firms, both write 25 where the MER counts 56. Generate that evidence automatically (system inventory, scenario list with CBI pack version, last list update, share of customers risk-rated, alert review statistics) so the auditor's paragraph is true by construction and the bank can see its own gap against the current CBI pack.

## 9. Local integrators in Baghdad and Erbil; Jordanian and Lebanese integrators

| Firm | Base | What it does in Iraq | Evidence |
|---|---|---|---|
| Pio-Tech | Amman, Jordan (also Pio-Tech East Africa) | Bank-BI data warehouse, AML, goAML, RBA, credit bureau, collections; "a System Integrator & one of the leading Business Solutions Providers across the Levant, GCC, North, and East Africa"; "Trusted by 100+ Clients Worldwide" | [A] PIO-HOME; PIO-REF |
| ICS Financial Systems | Amman (centre of excellence); Iraq office | ICS BANKS; user-group summits in Baghdad (Jun 2022 with AL-AWAEL, Fortinet and Huawei; May 2023 with Oracle) | [A] ICS-SUMMIT22; ICS-SUMMIT23 |
| AL-AWAEL | Jordan and Iraq (new Baghdad office, Al-Qadisiyah, 2022) | "one of the biggest IT solution provider companies in the region, with more than 30 years of experience in integrated business, cyber security solutions, and cloud technology in Iraq, Jordan and the Middle East" | [A] ICS-SUMMIT22 |
| ITSS | Geneva headquarters, 18+ offices, "Temenos Regional Partner of 2019" | Temenos delivery partner at NBI (2024); also delivered ADCB Egypt's Temenos Payments Hub | [A] TEM-NBI; TEM-ITSS; TEM-ADCB-EG |
| Prema Consulting | MENA (established 2015) | Temenos implementation partner at ZainCash (2024); certified for Transact, Payments Hub and **Temenos Financial Crime Mitigation**, so a likely channel for FCM at Temenos-based Iraqi banks | [A] TEM-ZC-SS; TEM-PREMA |
| BML Istisharat | Lebanon | ICBS core banking at the CBI, Erbil Bank, Minare Bank, NBI (to the end of 2023), Al Qurtas | [B] FF-ICBS-2016; [A] NBI-AR-2023 |
| Capital Banking Solutions ("CBS") | Lebanese-linked, Paris-dated releases; "Since 1997"; regional sales director for the Middle East | CapitalBanker core (Oracle) and PowerBanker (IBM Power); CapitalDigital; CapitalCompliance AML with goAML and CRS; IMEIB (core since 2006), Sumer (core and compliance, 2022), Trans Iraq Bank (AML); claims "more than 25 Iraqi" banks | [A, vendor] CBSOL-HOME; CBSOL-CORE; CBSOL-SUMER; CBSOL-IMEIB; [A] TIB-AR-2024 |
| Iraqna IBTS | Baghdad | Compliance training and technology consultancy; ACAMS/Laserfiche/FATCA tool agent; LSEG World-Check channel | [A] LSEG-IRAQNA |
| Azentio | Singapore; Gulf sales team | iMAL and AMLOCK; Erbil signing ceremony | [A] AZ-KIB-IMAL |
| ESDS | India | MoU with TBI on data centre, cloud, fintech (2019) | [B] IBN-TBI-ESDS |
| ProgressSoft | Amman, Jordan | PS-GatePay RTGS, ACH and cheque gateways at nine Iraqi banks (2019); electronic cheque clearing at Standard Chartered Iraq; Iraq Finance Expo 2020 | [A] PS-GATEPAY |

No Erbil-based integrator was identified by name in any source. Integrators are mostly Amman-based, which matters for support (Jordan is the logistics hub for Iraq: SWIFT user group meetings for Iraqi banks are held in Amman, ICSFS signs Iraqi banks in Amman and Beirut) and for visas, sanctions and payment constraints (a Jordanian vendor can invoice in USD or JOD; an Iraqi vendor invoices in IQD).

AKR-TMS hooks:
- Staff the first deployments from Baghdad and Erbil; plan a Jordan-based partner role for 24-hour support only if a bank asks.
- A "partner programme" with local integrators (ITSS, Prema, AL-AWAEL-class firms) lets them implement while AKR-TMS keeps the product: copy the Temenos and ICSFS pattern.

## 10. Public tenders, vendor press releases and job adverts that name systems (2001-2026)

**Public tenders: none found.** No tender notice for an AML, screening or core banking system at an Iraqi bank was found in sources reachable here (the web-search budget ran out). The CBI, Rasheed, Ashur, Bank of Baghdad, NBI and First Iraqi Bank websites were checked and show no tender pages for these systems; the Rafidain and TBI sites did not answer, and Iraqi government procurement portals could not be searched.
First Iraqi Bank ran "an international RFP" (2021) and Zain Iraq Islamic Bank an evaluation of five vendors (2017); both are known only from the winning vendor's release.
Treat tenders as private invitations to named vendors.

Dated vendor and bank statements (all in this file's source list):

| Date | Event | Source |
|---|---|---|
| Mar 2001 | ICSFS says it went live in Iraq | [A] ICS-MANSOUR |
| 2003-2009 | TBI: Misys projects | [A] TMI-TBI-2016 |
| 2006 | IMEIB implements Capital Banking Solutions' "first Core banking system in Iraq" (vendor release of 2022) | [A, vendor] CBSOL-IMEIB |
| 2007 | Ashur starts on ICS; Rafidain/B-Plan/Misys turnkey contract | [A] ICS-SUMMIT23; [B] RBI-2008 |
| 2009 | Cihan Bank signs Path iMAL | [B] FF-AMEEN |
| Nov 2015 | Mansour Bank selects ICS BANKS; TBI contract with Misys (14 Oct, probably 2015) | [A] ICS-MANSOUR; TMI-TBI-2016 |
| Oct 2016 | BML ICBS user list incl. CBI | [B] FF-ICBS-2016 |
| 2017 | A wave of greenfield Islamic banks: ICS (Al-Arabiya Feb, IIB Mar, Al-Thiqa Sep); Path (Zain Oct, Al-Rajih Dec); BML (Al Qurtas Nov) | [A][B] sections 5 |
| Jul 2018 | BLF Iraq T24 R17 | [B-] FF-BLF |
| Dec 2018 / Jan 2019 | Hammurabi (ICS), Al Mustashar (ICS) | [A] ICS-HAMMURABI; ICS-MUSTASHAR |
| Mar 2019 | Rafidain-Oracle report | [B] FF-RAFIDAIN |
| Apr 2019 / 2021-2024 | Ameen Al-Iraq: iMAL signed (Apr 2019); annual reports 2021 to 2024 name an AML system from Path Solutions (Azentio from 2024) plus Refinitiv Word Check | [B] FF-AMEEN; [A] AMEEN-AR-2021; AMEEN-AR-2024 |
| Mar 2020 | LSEG and Iraqna IBTS partnership | [A] LSEG-IRAQNA |
| 2021-2022 | First Iraqi Bank, UR, Al Sanam, Al-Ataa choose ICS; Jun 2022 ICS user-group summit | [A] section 5 |
| Jan-Aug 2022 | IMEIB CapitalDigital live (Jan); Sumer selects CapitalBanker core and extends CapitalCompliance with goAML and CRS (Aug) | [A, vendor] CBSOL-IMEIB; CBSOL-SUMER |
| Jan 2023 onwards | CBI: Refinitiv and FinScan on external remittances; K2 Integrity (Aug 2023, Sep 2024); EY (Sep 2024) | [A] CBI-BOOKLET |
| Apr 2024, May 2024 | NBI and ZainCash go live on Temenos | [A] TEM-NBI; TEM-ZC |
| Jan-Apr 2024 | Pio-Tech releases (Ashur, FastPay, FIB, Iraqi Islamic Bank, IMTB) | [A] section 8 |
| Jan 2024 / Feb 2025 | KIIB: iMAL then AMLOCK | [A] AZ-KIB-IMAL; AZ-KIIB-AML |
| Sep 2025 / Mar 2026 | CBI reform initiative; Phase 1 guides | [A] CBI-REFORM-CIRC; CBI-PHASE1 |

Job adverts naming systems (weak evidence, [C]): ZainCash advertises an on-site "Temenos T24 Developer" in Baghdad (Temenos Design Studio, Java or Python; "Fluency in English and Arabic (Kurdish is a plus)") [C, ZC-JOB]; Bank of Baghdad's 2025 report lists staff with "ORACLE DBA" qualifications [A, BOB-AR-2025]. Job boards (Bayt, LinkedIn) block automated reading, so a broader job-advert survey was not possible.

## 11. Rafidain and Rasheed modernisation programmes

| Programme element | Facts | Source |
|---|---|---|
| Advisers | EY (state banks; Cabinet approved EY contracts on 3 Jan 2024: study of Rafidain and Rasheed restructuring with recommendations "within six months"; merger of Industrial, Real Estate and Agricultural banks; merger of two insurers); K2 Integrity for Rafidain (AML/CFT and compliance systems, Aug 2025); K2 Integrity and KPMG for TBI (Jun 2025); Oliver Wyman for the CBI's private-bank reform | [B] IBN-EY-JAN24; IBN-K2; IBN-REFORM-JUN25 |
| Target model | Rafidain becomes "First Rafidain Bank" ("Rafidain Al-Awal"), "designed to implement advanced banking technologies and international best practices", with government participation "less than 24 per cent"; the rest offered to private banks and international investors; plan expected "by the end of the year" (2025). On 26 Jan 2026 the Finance Minister and EY reviewed "preparations for the establishment of the First Rafidain Bank and upgrades to Iraq's financial information infrastructure", IFRS adoption, "Automation of financial systems and digital transformation" | [B] IBN-PM-MAR25; IBN-REFORM-JUN25; IBN-EY-2026 |
| Prior system projects | Rafidain: Misys turnkey (2007), Fusionbanking (2012), Oracle FLEXCUBE reported (2018-19). Rasheed: a core banking system acquired (vendor not named), data and DR centres. TBI: Misys Equation upgrade (2015) and ESDS MoU (2019). | [B] RBI-2008; FF-RAFIDAIN; [A] RASHEED-ABOUT; TMI-TBI-2016 |
| Conditions in 2026 | State-bank lending frozen by "declining liquidity" (Feb 2026 report); a new Prime Minister's "anti-corruption drive"; a new CBI Governor (Aug 2026); the state banks are not subject to the private-bank reform fee and capital rules | [B] IBN-AGBI; IBN-CMC |
| Timing | Plan approval was expected at the end of 2025 and was still "in line with the consultant's recommendations" in Jan 2026: a new CBS or AML tender for First Rafidain Bank would follow the restructuring decision, probably 2027 or later (an estimate) | [B] IBN-EY-2026 |

Implications for AKR-TMS:
- The two state banks hold the largest account bases (Rafidain: about 45% of banking assets in 2017) and the largest AML risk (salary localisation, government payments, large cash). They are strategically the biggest prospects and procure slowest.
- A new "First Rafidain Bank" legal entity means a **new CBS instance and a migration of the customer base**: AKR-TMS must handle a bank whose CBS changes during the contract (canonical keys independent of CBS ids; replay of history).
- K2 Integrity is the likely author of Rafidain's AML target model; it will assess tools against global practice (Wolfsberg, FATF), so AKR-TMS needs the same evidence pack as for a US correspondent (files 03, 04, 29).

## 12. CBI programmes: payment infrastructure, banking reform and remittance control

### 12.1 Payment infrastructure (what a bank's CBS must connect to)

| System or programme | Facts | Grade, source |
|---|---|---|
| Iraqi Payment System | RTGS ("updated"), C-ACH (electronic clearing), IBSC (internal clearing system), RPSI (retail payment system infrastructure), CSD (central securities depository), TS (trading system) | [A] CBI-BOOKLET |
| CBI banking systems | ICI (credit information), BSRS (banking supervisory reporting system), ITRS (international transaction reporting), FITR (financial institutions transaction reports), digital platforms for letters of guarantee and for financial-adequacy books | [A] CBI-BOOKLET |
| Integrations banks must support | The Detailed Assessment Guide (standard B2) lists RTGS, ACH, **National Switch (NS)**, AML systems, CBI reporting interfaces (CBR, CMS), interbank transfers, SWIFT MX, ICI, e-payment company platforms and gateways, card networks and card management, BSRS, the **instant payment system (IPS)**, the **unified government payment gateway (UGPG)**, the national card scheme, "and any new systems imposed by the CBI"; payment messages must follow **ISO 20022**, APIs must follow **OpenAPI**, data must use IBAN, BIC and ISIC, and reporting must comply with BCBS 239 | [A] CBI-GUIDE |
| ISO 20022 and SWIFT MX | The CBI requires ISO 20022 for payment messages "in line with global best practice and the CBI's modernisation plan"; NBI reports implementing "the new SWIFT MX standard" in 2025; SWIFT's coexistence period for cross-border instructions ended on 22 Nov 2025 (file 25 section 12); Bank of Baghdad reports SWIFT CSP standards work | [A] CBI-GUIDE; NBI-AR-2025; BOB-AR-2025 |
| National Switch, national payments company, local card scheme | Listed among the CBI's strategic initiatives: "building the National Switch"; "project to establish the National Payments Company and the Local Scheme Card"; Bank of Baghdad moved card processing from an external provider to a local switch in Jan 2025 | [A] CBI-BOOKLET; BOB-AR-2025 |
| Volumes (2025) | 500+ trillion IQD of electronic settlements a year; 20 million electronic cards issued; 179,654 POS and 3,015 ATMs; 37,636 cash-withdrawal points; 18 e-payment companies (5 wallets); about 15 trillion IQD of taxes and bills collected electronically in 2025; all government payments electronic from June 2025; financial inclusion above 40% | [A] CBI-BOOKLET; [B] IBN-REFORM-JUN25 |
| Wallets and super-apps | CBI launched inter-wallet transfers between verified wallets and to merchant wallets (4 Sep 2025); SuperQi app from Qi with Ant International (Dec 2024); Qi has a 36-month AML/CFT partnership with K2 Integrity and is building a compliance back office in Jordan (Jun 2025); CBI told e-payment firms on 22 Jul 2026 that international-standard compliance is "a condition for expanding card usage limits" | [B] IBN-WALLET; IBN-SUPERQI; IBN-QI-K2; IBN-EPAY-JUL26 |
| Cooperation with the Communications and Media Commission | Talks on digital payment regulation, AML/CFT cooperation, licensing of global platforms (6 Aug 2026) | [B] IBN-CMC |
| Correspondent banking | JP Morgan began opening correspondent accounts for Iraqi banks in Jan 2025; Citi agreed to settle card payments; TBI won JP Morgan's "STP Excellence" award (99.68% MT103 success, 14 May 2025) | [A] CBI-BOOKLET; [B] IBN-JPM-TBI |
| Vendors of the national platforms | The CBI's own RTGS, ACH and switch builders are not named in the sources read. On the bank side, **ProgressSoft (Jordan) PS-GatePay**, an RTGS, ACH and cheque gateway "integrating with the banks' Core Banking Systems", was chosen after an Iraqi bank procurement "against other vendors" by National Bank of Iraq, Mosul Bank, RT Bank, BLOM Bank and Economy Bank, joining Standard Chartered, Ashur, Bank of Baghdad and Credit Bank of Iraq (nine banks, Jan 2019); the gateway database is a structured source of RTGS and ACH data (file 25 section 12) | [A] PS-GATEPAY |

AKR-TMS hooks (12.1):
- The canonical model needs first-class fields for RTGS, ACH, National Switch, IPS and card transaction types and for ISO 20022 party and purpose data (files 25 section 12 and 03 payment transparency).
- Interbank wallet transfers and the national switch create flows that do not pass through the bank's CBS in the same way as account transactions: ask each bank where those flows are recorded.

### 12.2 The Banking Reform Initiative 2025: the demand driver and the CBS filter

Source documents (all read in full or in the relevant parts; the PDFs are on cbi.iq page 172): the Banking Reform Initiative circular for private commercial and Islamic banks, the **CBI Standards Booklet 2025**, the **Detailed Assessment Guide 2025**, the Phase 1 requirements guide (March 2026), the merger plan guide (March 2026), the foreign-branch circular, and the August 2025 and February 2026 FAQs [A] CBI-REFORM-PAGE.

What it is: "a binding multi-year banking reform initiative" that came into force in September 2025 for all private commercial and Islamic banks (and foreign branches), with three paths: continue, merge or liquidate. The standards "were designed to address the root causes underlying the international restrictions on foreign currencies on many banks in Iraq, and were developed in coordination with the international bodies responsible for these measures". Full compliance, "as confirmed by reputable external organisations", "will enable removal of the current foreign-currency restrictions", except where other restrictions arise "for reasons outside the scope of these standards (such as OFAC designation)". Access to foreign currency "will remain linked" to full compliance and to "direct relationships with foreign correspondent banks". Non-compliance can lead to sanctions or licence withdrawal under Banking Law 94/2004 articles 56 and 13 [A] CBI-STANDARDS.

| Element | Facts |
|---|---|
| Continue path, Phase 1 gates | Board letter by 11 Sep 2025; legal undertaking and a non-refundable annual reform fee of **IQD 3,168,000,000 (about USD 2.4 million)** by 30 Sep 2025; paid-in capital of IQD 300bn by 31 Dec 2025 with a plan to IQD 400bn; liquidity ratios; ownership disclosure |
| Foreign branches | Continue or liquidate; USD 600,000 fee; USD 80m capital |
| Assessment phases | Phase 1 30 Sep 2026; Phase 2 March 2027; Phase 3 30 Sep 2027; Phase 4 March 2028; Phase 5 30 Sep 2028; final check end 2028 (the circular text prints two of the dates inconsistently; the schedule table shows these) |
| Phase 1 submission window | 1 April to 30 September 2026. For **AML/CFT/sanctions (standard D2)** and internal controls (D4) the bank must submit "an assessment from an independent external operational auditor approved by the CBI that includes the test results together with a detailed remediation plan to reach full compliance", done "within 12 months of submission" by the same third party. CBI decisions: "compliance" or "remediation" (resubmit within six months); banks that fail to remediate "may be moved to the merger path or the liquidation path" |
| External firms | "Wherever the phrase 'company approved by the CBI' appears, it means an international company specialised in its field with experience acceptable to the CBI", to be defined in a list "published later". A firm that helped implement a fix cannot later do the formal assessment for the same bank; banks pay the firms themselves; all documents in Arabic and English |
| Technical audit (core banking B2, online banking B3, infrastructure and data B7, payment systems B8) | The first test and remediation plan comes after Phase 1; the February 2026 FAQ says "the audit by a technical auditor at end-2026 with a two-year grace period until the end of 2028" for full compliance |
| **B2 core banking system** | "Each bank must use a comprehensive core banking system ... supplied by reputable and internationally recognised service providers". The FAQ: "The universal core banking system ... must be supplied by vendors classified within **Tier 1** in terms of global reputation and operational reliability, based on internationally recognised reports and databases (such as Forrester and Gartner)", with "proven experience in running core banking systems for a large number of commercial or digital banks in advanced or supervised markets, and evidence of strong international presence" and compliance with international standards on information security, business continuity and technology governance. Banks must **notify the CBI before any upgrade or major replacement** (14 days before major changes) and report security incidents within 48 hours. Availability 99.5% per month; ISO 27001, 22301, 20000 and 25011; annual maintenance contract with each CBS vendor with SLAs (P1 response 30 minutes, fix 4 hours); knowledge transfer; "The system must be hosted in data centres inside Iraq and be subject to the CBI's data sovereignty requirements" |
| **D2 AML/CFT/sanctions** | Governance (committee, independent MLRO reporting to the board committee), risk-based procedures (customer risk rating with geography, product, transaction behaviour and shareholder structure; KYC with CDD, ID&V and PEP identification; EDD), "specialised electronic monitoring systems to detect patterns of suspicious activity", the system "must support dynamic rule creation, alert generation and investigation workflow, with periodic calibration and quality assurance controls", "real-time sanctions screening tools covering customers, counterparties and transactions", and enablers: "high-quality data for all AML-relevant operations for **7 years**" and "integrated systems supporting KYC, transaction monitoring, sanctions screening and reporting ... able to interact with regulators ... integrate with the core banking system", with audit logs and access controls. The assessor tests "rule logic, alert generation, communication and reporting paths, the review and improvement mechanism", "configuration files, alert thresholds, user access permissions, audit trail integrity" and "integration across KYC, monitoring and sanctions modules", and whether the data used for detection and reporting "can be traced" |
| B8 payment systems | Banks must integrate with RTGS, CSD, ACH and the mobile switch; as sender, payer fields must "match information verified through the bank's AML procedures"; as receiver, beneficiary fields must match verified information and payer fields must not contain "missing, incomplete or meaningless data" |
| Merger path | The plan must identify "any products, services, systems (such as core banking systems) or processes ... to be shut down", with dates, and ensure "secure ... phased" technology integration and data migration |
| General answers | Banks barred from the OFAC list are not exempt from the reform; "effective implementation ... is the only way to address the 'black list' issue"; CBI will declare a bank compliant at any time, not only at the end |

Why this is the single most important market fact for AKR-TMS:
1. **Every private and foreign bank must pass an external AML assessment** with remediation plans, over 2026-2028. This turns the AML system from a CBI-fine avoidance item into a licence-survival item, with an assessor reading the tool's rules, thresholds, logs, integration and data retention. Banks failing remediation are merged or liquidated.
2. The **external assessors are international firms from a CBI list not yet published**; they will favour tools they can test, document and audit: explainable rules, logs, immutable audit trails, configuration export, threshold registers. AKR-TMS should ship a ready "assessor pack" (D2 test script mapped to product evidence) and publish its own test results.
3. **The Tier 1 core banking rule threatens the regional CBS vendors** if the CBI applies "Tier 1 by Gartner or Forrester" strictly. ICS BANKS (about 30 Iraqi customers), BML Istisharat ICBS and Path/Azentio iMAL are not described as Gartner or Forrester core banking vendors in their own materials (ICSFS cites Celent, IBS Intelligence and World Finance; the Gartner mention on Azentio's site is for insurance core systems; no Gartner or Forrester core banking listing was found for any of the three; Capital Banking Solutions, whose CapitalBanker core runs at IMEIB and Sumer, says it is a "Solid Player" in Forrester's Global Banking Platform Vendor Pyramid 2020 [C, CBSOL-FORRESTER], so it is the one regional vendor that may pass a Forrester-based test), while Temenos, Oracle FLEXCUBE, Infosys Finacle and Finastra Essence are the usual Gartner-listed candidates (Finastra's page cites a 2025 Gartner Magic Quadrant for Retail Core Banking Systems, Europe). How strictly the CBI will apply the test is unknown. If it does, some banks will replace their CBS in 2026-2028 and mergers will retire one CBS of each pair. AKR-TMS's CBS-agnostic mapping layer is then a strategic asset **and** a continuous cost: mapping profiles for the new CBS families (file 17) must come first.
4. The data-in-Iraq rule ("systems hosted in data centres inside Iraq") settles the hosting debate: on-premises or Iraqi data centre only; no foreign SaaS. This also excludes vendors that only offer SaaS (several challengers, file 12) and favours AKR-TMS's default deployment.
5. A **7-year retention** for AML data (CBI guide) is longer than the 5 years in Law 39/2015 and the CBI controls (file 01); AKR-TMS should default to 7 years and make it configurable.
6. The annual reform fee (about USD 2.4m per bank), capital of IQD 300-400bn and assessor costs mean surviving banks have budgets for compliance but also strong incentives to buy the cheapest passable tool; price discipline and a fast, assessor-friendly deployment win.

AKR-TMS hooks (12.2):
- Product feature set mapped one-to-one to the D2 sub-clauses (governance, customer risk rating, KYC/PEP, EDD, investigation protocols with audit trail, specialised monitoring with dynamic rules and periodic calibration, real-time sanctions screening of customers, counterparties and transactions, 7-year data, integration with CBS, user access and audit logs).
- "Assessor mode": a read-only role for the CBI-approved auditor with exports of rules, thresholds, logs, test cases, alert and STR data, and a record of what was shown to whom.
- Remediation-plan generator: from an assessor's finding to an AKR-TMS configuration task list and evidence.
- Migration-proofing: stable internal identifiers; a mapping profile library keyed to CBS versions; a procedure to point AKR-TMS at a replacement CBS or merged entity without losing history (file 25).
- Hosting: Iraqi data centre or bank premises only; no vendor-operated cloud.

### 12.3 External remittance control (the CBI's own pre-release screening)

The CBI runs an electronic platform through which banks and payment companies submit foreign remittance requests; the platform screens names, and international advisers review samples. Evolution:

| Date | Step |
|---|---|
| Jan 2023 | Contract with Refinitiv "to monitor electronic outgoing remittances and screen them via FinScan" |
| May 2023 | Balances strengthened only at correspondents for NBI, Bank of Baghdad, TBI and ADIB (USD) |
| Aug 2023 | K2 Integrity reviews all USD remittance requests |
| Feb 2024 | Transfers start in other currencies (UAE dirham, yuan, Jordanian dinar, Saudi riyal) |
| Sep 2024 | K2 reviews 20% of USD remittances and 100% of dirham remittances; EY reviews all non-USD outgoing remittances |
| Jan 2025 | JP Morgan starts opening correspondent accounts for Iraqi banks |
| Result | 207,638 outgoing remittances to Nov 2025 at an average of USD 360,233 (2022: 12,000 remittances averaging USD 3.3 million; 90% via two correspondents); the number of banks sending external remittances fell from 32 to 20; 75% of volume at 4 banks |
| Control pyramid | Laws, CBI instructions, banks' and payment companies' KYC, due diligence and EDD, electronic platforms, "control companies EY + K2 Integrity", foreign correspondent banks, final beneficiary |

All from [A] CBI-BOOKLET (slides 40 to 46). So a bank's own pre-release screening (phase 2 of AKR-TMS) runs *before* the request enters the CBI platform: a bank that screens well sees fewer rejections (the booklet tracks rejected remittances), and a bank that screens badly loses dollar access.
AKR-TMS hooks: a "CBI platform pre-check" mode that applies the same name lists and beneficiary rules as the CBI's FinScan screening, reports predicted rejection risk and keeps the evidence package K2 Integrity and EY will ask for (end-beneficiary, invoice, customs data).

## 13. Pricing and procurement realities

### 13.1 What Iraqi banks actually spend (annual reports)

| Bank | Software and systems spend | Reading | Source |
|---|---|---|---|
| National Bank of Iraq | Additions to "Computer Software & Systems": IQD 25.995bn in 2024 (the Temenos core and payments migration year; about USD 19.8m at about IQD 1,310 per USD), IQD 3.113bn in 2025; amortisation IQD 8.8-9.0bn a year; projects in progress IQD 10.5bn at end-2025 | Software additions of roughly **USD 20m in the year of one core banking and payments replacement** (the Temenos migration); the note does not split the projects | [A] NBI-AR-2025 |
| National Bank of Iraq (third-party contracts table, 2023 report) | The 2023 report lists every third-party contract with its yearly amount (Arabic table with English product names). Examples: "ICBS System Renewal 2023" (Istisharat) IQD 64.714m, about USD 49k; "Profiling system renewal 2023" (Infrasoft) IQD 4.585m, about USD 3.5k; "Swift Reporter Renewal 2022" (AEG) IQD 34.3m, about USD 26k; "Mule Soft Integration" (ITSS Solutions) IQD 88.8m, about USD 68k; "PAM renewal and Purchasing Licenses" IQD 134.5m, about USD 103k; "Genesys 2023 renewal" IQD 98.3m, about USD 75k (all at about IQD 1,310 per USD) | **The yearly renewal of the legacy AML profiling system was about USD 3.5k and of the legacy core about USD 49k.** The price anchor in a large private bank's mind for AML maintenance is in the low thousands of dollars, not the hundreds of thousands. The migration-year spend above is a one-off, and the table does not show the new Temenos or Siron licence terms | [A] NBI-AR-2023 |
| Bank of Baghdad | Software additions IQD 3.384bn in 2025 (about USD 2.6m) and IQD 0.474bn in 2024 (about USD 0.4m); 20% annual amortisation; software net book value IQD 4.157bn | A large private bank's yearly software budget is in the **low single-digit USD millions** | [A] BOB-AR-2025 |
| CBI reform fee | IQD 3.168bn a year (about USD 2.4m) per private bank; USD 600,000 per foreign branch | A new compulsory yearly cost that competes with software budgets | [A] CBI-REFORM-CIRC; CBI-FOREIGN-CIRC |
| CBI fines for missing an AML system | IQD 100,000 per working day "until the violating bank would acquire or activate an automated system" (about USD 76 a day); average fine per bank about USD 18,314; total about USD 1.5m in 2018-2022 | Fines were far below the price of a tool, so banks bought minimum viable systems; the reform's licence-level consequence is a different incentive | [A] MER paras 661-662 |

### 13.2 Public price signals for the competitors (from files 08-16; all weak)

| Product or vendor | Signal | Reliability and source |
|---|---|---|
| NICE Actimize | About USD 0.45m revenue per client per year (arithmetic from NICE revenue and customer count); "EUR 200,000-1,000,000+ licence and implementation" from a directory | Low to medium; file 08 s.9 |
| Verafin (Nasdaq) | About USD 120,000 per institution per year (arithmetic), community-bank base | Medium; file 11 |
| Quantexa | UK public-sector list GBP 0.6-7.7m per suite per year | Medium; file 12 |
| Oracle FCCM | No list price; cloud metering per transaction, per template, per KYC check | Medium; file 09 |
| FinScan (the CBI's remittance screen) | A third-party model: USD 500-1,000 a month for a starter package up to "often $100k-$300k+/year" for global enterprise | Low; file 14 |
| Dow Jones RiskCenter | EUR 277,380 for 25 licences at BaFin (2024) | Medium; file 14/15 |
| ComplyAdvantage | Starter plan USD 99-319 a month for 100-2,000 monitored entities | Medium; file 15 |
| Azentio AMLOCK, Pio-Tech, Temenos FCM, Siron, Firco | No public price found; Azentio's buyer's guide says total cost of ownership "usually outweighs the license fee" and gives no figure; Temenos FCM SaaS is "pay-as-you-go" | Weak; AZ-BUYERS; TEM-FCM-SAAS |

### 13.3 Procurement behaviour (what the evidence supports)

- **Private banks** buy through invitation to named vendors; vendor releases speak of "an international RFP" (First Iraqi Bank) or "carefully evaluating alternative systems" but no public tender notices exist (section 10). Selection is influenced by the regional integrator (Amman) and by group parents (Burgan, QNB, Capital Bank of Jordan).
- **State banks** are driven by Cabinet decisions (Resolution 23274 of 2023 for the K2 Integrity partnership; Cabinet-approved EY contracts), which means a direct award or government tender and slow timelines.
- **Under the reform**, banks pay the assessment firms themselves, must notify the CBI of core banking changes, and need a CBI-approved firm to certify AML: vendors that bring evidence packs reduce the bank's assessor cost.
- **Payment barriers**: banks barred from dollar transactions cannot easily pay foreign vendors in USD through correspondents (an inference from the bans: file 01 section 4); a vendor that invoices in IQD from an Iraqi entity, or through a regional agent, removes the problem. No source confirms vendors invoicing in IQD.
- **Hosting**: all production systems in Iraqi data centres (CBI FAQ Feb 2026); this excludes SaaS-only vendors and cloud-metered pricing for core AML functions.
- **Support**: the CBI guide requires an annual maintenance contract with SLAs (P1 response 30 minutes, fix within 4 hours) with every core banking vendor; AML vendors should expect the same standard from assessors.

Working hypothesis for AKR-TMS pricing (not evidence, to test with the first customers): a flat annual licence per bank with bands by size, in the low six figures for a mid-size private bank (the same order as Verafin's average and well below Actimize's), a smaller package for small banks and exchange companies, and an implementation fee that is fixed and small because the data mapping is the main effort. A bank spending USD 0.4-2.6m a year on all software (Bank of Baghdad) cannot pay Actimize-class prices for AML, and the one maintenance renewal found for an incumbent AML profiling system (National Bank of Iraq, about USD 3.5k) shows how low the reference price in a bank's mind can be (an inference from one contract line, to test before quoting).

## 14. Regional peers: Jordan, UAE, Saudi Arabia, Egypt, Lebanon (who dominates)

Pattern across the region: **the CBS and the AML tool are bought from different vendors**, with a regional compliance integrator or a global AML vendor on top. Evidence of dominance is thin and mostly from vendor references; treat the "dominant" label as a reading of the references, not market share.

| Country | CBS seen | AML, screening or reporting seen | Regulator driver | Source |
|---|---|---|---|---|
| **Jordan** | Temenos (Capital Bank of Jordan since 2022; Jordan Ahli; Cairo Amman), Infosys Finacle (Housing Bank for Trade and Finance), ICS BANKS (Al Rajhi Bank Jordan, Arab Jordan Investment Bank's leasing arm) | **Temenos FCM at Cairo Amman Bank**: "completed its upgrade of the Temenos Financial Crime Mitigation (FCM) suite in December 2024, in line with a mandate from the Central Bank of Jordan ... uses the full suite of FCM modules, including screening, profiling, and KYC+"; false positives "dropped by 40-50%" with the VBOT model (vendor story). **Pio-Tech** lists 14 Jordanian banks including Arab Bank, Capital Bank, Housing Bank, Jordan Kuwait Bank and Jordan Islamic Bank. Eastnets: Arab Jordan Investment Bank | Central Bank of Jordan mandated the FCM upgrade | [A] TEM-CBJ; TEM-CAB; FINACLE-SITEMAP; ICS-RAJHI-JO; PIO-REF; EAST-AJIB |
| **UAE** | Infosys Finacle (Emirates NBD, RAKBANK), Oracle FLEXCUBE (Mashreq since 2010, replacing 40+ legacy systems), Azentio iMAL (National Bonds), Temenos (Reem Finance, Invest Bank on Temenos Banking Cloud) | SAS AML (Mashreq since 2006), ThetaRay (Mashreq, Feb 2022), Silent Eight (Emirates NBD, Mashreq, FAB, ADIB), NICE Actimize (UAE Exchange, 2015), Eastnets (Commercial Bank International, Oct 2021) | CBUAE; high-sophistication market with several specialist AML vendors per bank | [A] RBI-MASHREQ; FINACLE-SITEMAP; EAST-CBI; files 08, 10, 12, 13 |
| **Saudi Arabia** | Temenos (Bank Albilad, Saudi EXIM, FAB's Saudi expansion), Finastra Essence (Agricultural Development Fund), ICS BANKS (microfinance entrants), Azentio digital lending (Tamam Financing, SAHB Finance) | Azentio AMLOCK marketed to Saudi banks with SAFIU reporting; Eastnets SafeWatch at American Express KSA; Pio-Tech lists Alinma and SAIB; SAS has a Riyadh branch | SAMA and SAFIU | [A] TEM-SITEMAP; FIN-SITEMAP; AZ-SAUDI; PIO-REF; file 10 |
| **Egypt** | Temenos (ADCB Egypt with ITSS, Midbank), Infosys Finacle (Bank Alexandria), Intellect (Faisal Islamic Bank of Egypt) | **Pio-Tech** lists at least 6 Egyptian banks incl. National Bank of Egypt, Banque du Caire, Midbank and Industrial Development Bank (AML and RBA implemented Sep 2025); Eastnets: Arab International Bank (SafeWatch Screening 5, March 2023) | CBE and the Egyptian FIU | [A] TEM-ADCB-EG; FINACLE-SITEMAP; INT-SITEMAP; PIO-REF; PIO-IDB-EG; EAST-AIB |
| **Lebanon** | BML Istisharat ICBS is Lebanese (Iraq users); Pio-Tech lists Al Mawarid Bank; other Lebanese banks' systems were not found | Not found | BDL and the Special Investigation Commission; a bank sector in crisis since 2019 [K] | [B] FF-ICBS-2016; [A] PIO-REF; [N] otherwise |

What the peers teach:
1. **Jordan is the closest peer and the main supplier of Iraqi compliance tools**: Pio-Tech, ICSFS, AL-AWAEL and Capital Bank (NBI's parent) are all Jordanian. A central-bank mandate (CBJ) drove FCM upgrades; the CBI's reform standards do the same job in Iraq.
2. **Gulf banks run specialist AML on top of any CBS** (SAS at Mashreq, Silent Eight, ThetaRay): Iraq's largest private banks follow that pattern (Siron plus Firco at Bank of Baghdad).
3. **Egypt and Jordan show a compliance-reporting niche** (RBA, goAML-type, FATCA) served by a regional integrator (Pio-Tech) for 14 Jordanian and at least 6 Egyptian banks; this is the niche AKR-TMS will meet first in Iraq.
4. No single global AML vendor dominates the region; where Temenos owns the CBS it sells FCM (Jordan), elsewhere AML is a separate tender.

## 15. What it means for AKR-TMS

1. **The window is now and it is regulatory.** The CBI Banking Reform Initiative makes an external, CBI-approved AML assessment (standard D2) a gate on every private and foreign bank's licence: first submissions were due 30 Sep 2026 with a remediation plan and a six-month resubmission if gaps remain, final compliance by the end of 2028. A bank that fails remediation is merged or liquidated. This replaces the old fines-for-no-system incentive (USD 76 a day) with a licence-survival incentive.
2. **Buyers are fewer and poorer than the 74-76 banks suggest**: 49 private banks (shrinking), 7 state banks, 11 foreign branches, 18 e-payment companies, digital banks and about 460 exchange companies. A large private bank spends USD 0.4-2.6m a year on all software; the new reform fee is about USD 2.4m a year per private bank.
3. **The core banking landscape is fragmented and about to move**: ICS BANKS leads by named count (30+ customers claimed, most Baghdad private and Islamic banks), Capital Banking Solutions claims "more than 25 Iraqi" banks (core, digital and compliance; IMEIB and Sumer named), Path/Azentio and BML Istisharat in Kurdistan and Islamic niches, Temenos at three newcomers (NBI, ZainCash, BLF), Finastra/Misys and Oracle at the state banks. The CBI's "Tier 1" rule and the merger path will force CBS changes by 2028.
4. **AML tools are separate purchases and few are named**: Siron and Firco at Bank of Baghdad, Siron ("AML Sirion") at NBI from 2024 after a low-cost "profiling system", Azentio (formerly Path Solutions) AML at Ameen Al-Iraq since at least 2021 and at KIIB since 2024-25, a Capital Banking AML system plus FircoSoft through a local supplier at Trans Iraq Bank, Pio-Tech's compliance modules at many banks (29 references) and as scenario implementer at Trans Iraq Bank (81 then 96 scenarios), and the CBI itself on Refinitiv plus FinScan. Ameen and Trans Iraq Bank both name a Refinitiv World-Check system, the only bank-level World-Check customers found. No global suite (Actimize, FCCM, SAS) has an Iraqi footprint.
5. **The closest competitor is Pio-Tech**, a Jordanian integrator with a data-warehouse-based AML, goAML and RBA offer that is already referenced at 29 Iraqi banks. AKR-TMS should compete on detection quality, Arabic matching, assessor-ready evidence and a fixed price, and treat Pio-Tech as a possible channel for banks that already run its warehouse.
6. **The regulator wrote the product spec**: D2 sub-clauses, the CBI scenario catalogue, 7-year data, real-time sanctions screening, integration with the CBS, hosting in Iraq, and an assessor who tests rule logic, thresholds, logs and integration.
7. **Pre-release screening is not new in Iraq** (Firco at Bank of Baghdad; the CBI's remittance platform): the phase-2 API must coexist with and then replace these hooks, and a CBI platform pre-check mode is a differentiator.
8. **State banks are the largest prospects and the slowest**; they follow the Ministry of Finance, EY and K2 Integrity.
9. **Distribution runs through Amman and through the CBS vendors' communities**: ICSFS summits (90-120 attendees from 40+ banks), integrators (ITSS, Prema, AL-AWAEL), and the future CBI-approved assessors list.
10. **Evidence gaps to close in discovery**: the CBS of Rasheed, Credit Bank of Iraq, Iraqi Islamic Bank and about 20 other banks (Gulf Commercial Bank, Trans Iraq Bank, IMEIB and Sumer are now known or inferred); the product scope behind each vendor's claimed Iraqi customer count (ICSFS "30+", Capital Banking Solutions "more than 25"); the real status of Rafidain's FLEXCUBE; tender practice; the CBI approved-firms list (not yet published); the build vendors of the RTGS, ACH and National Switch.

## 16. Derived requirements (VD-16 to VD-40, market side; the CBS side is VD-01 to VD-15 in file 17)

| ID | Requirement | Source |
|---|---|---|
| VD-16 | Ship the CBI scenario catalogue (56 bank scenarios plus later additions, file 27) as the default configuration, with a coverage matrix, a scenario builder, back-testing on the bank's own history and per-scenario alert-to-STR metrics: the MER's named gap that no bank builds its own scenarios | MER paras 566-567; CBI-GUIDE (D2) |
| VD-17 | Produce goAML STR, SAR/TFR and CTR output natively, validated against the Iraqi FIU schema, with an STR quality grade before submission; goAML and RBA reporting are sold as products by Pio-Tech and are table stakes | PIO-GOAML; PIO-IIB; PIO-IMTB; MER |
| VD-18 | Provide an "assessor pack" mapped clause by clause to standard D2 (governance, customer risk rating, KYC and PEP, EDD, investigation with audit trail, specialised monitoring with dynamic rules and periodic calibration, real-time sanctions screening of customers, counterparties and transactions, data, integration, logs, user permissions), plus a read-only assessor role and export of rules, thresholds, test cases, logs and configuration | CBI-GUIDE (D2); CBI-PHASE1 |
| VD-19 | Default AML data retention to 7 years (CBI Detailed Assessment Guide), configurable upward, with a note that other CBI texts say 5 years | CBI-GUIDE (D2 enablers); file 01 |
| VD-20 | Deploy only on bank premises or in Iraqi data centres; no vendor-operated cloud for production; offline-capable list and model updates | CBI-FAQ-FEB26 (B7); CBI-GUIDE (B2) |
| VD-21 | Support CBS change management: stable canonical identifiers independent of the CBS, versioned mapping profiles, support for a bank running two CBS during a migration or merger, replay of history after re-pointing, and a notification pack for the bank's required 14-day CBI notice | CBI-GUIDE (B2); CBI-MERGER; CBI-FAQ-FEB26 |
| VD-22 | Co-exist with and displace the tools Iraqi banks already run (Siron, Firco and FircoSoft through a local reseller, Capital Banking AML, Azentio AML, Pio-Tech scenario sets): import scenario configuration and alert history, run read-only in parallel with side-by-side quality metrics, then take over; keep Firco-style pre-execution stop semantics for the phase-2 API; support banks that run two to five tools for one compliance job | BOB-AR-2022; BOB-AR-2023; BOB-AR-2025; TIB-AR-2022; TIB-AR-2024; AMEEN-AR-2024 |
| VD-23 | Add a "CBI platform pre-check" for external remittances: apply the same name lists and beneficiary rules as the CBI's Refinitiv/FinScan screening, estimate rejection risk, and assemble the evidence K2 Integrity and EY ask for (end beneficiary, invoice, customs data) | CBI-BOOKLET (slides 40-46) |
| VD-24 | Check payment message quality against standard B8: payer and beneficiary fields consistent with AML-verified data; no missing, incomplete or meaningless payer fields; ISO 20022, IBAN and BIC present | CBI-GUIDE (B8) |
| VD-25 | Model Iraqi payment channels in the canonical model (RTGS, C-ACH, IBSC, RPSI, National Switch, IPS, UGPG, wallet and inter-wallet transfers, card networks) and ISO 20022 party and purpose fields | CBI-BOOKLET; CBI-GUIDE (B2); IBN-WALLET |
| VD-26 | Add an e-payment-company profile (wallets, agent networks, inter-wallet transfers, salary and government payment flows), because 18 licensed e-payment companies are buyers and ZainCash, FastPay and Qi are building AML capability | CBI-BOOKLET; TEM-ZC-SS; PIO-FASTPAY; IBN-QI-K2 |
| VD-27 | Make Kurdistan a first-class market: Kurdish-Sorani name forms, iMAL and BML ICBS adapters, KRG salary localisation flows | AZ-KIB-IMAL; FF-ICBS-2016; file 01 section 9 |
| VD-28 | Offer a small-institution package (exchange companies, e-payment companies, small private banks): pre-mapped adapter, default CBI scenarios, minimal administration, fixed annual price, remote-managed option | Section 13; file 11 (VA-51) |
| VD-29 | Price as a flat annual licence per bank in bands, quoted in IQD or through an Iraqi entity so banks without dollar access can pay; no per-transaction metering for core AML functions | Section 13 |
| VD-30 | Run a partner programme for integrators (ITSS, Prema, AL-AWAEL-class firms) and a presence at the ICSFS user-group and CBI assessor communities; certify partner implementers and keep the assessor and implementer roles apart | ICS-SUMMIT22; ICS-SUMMIT23; TEM-NBI; CBI-FAQ-FEB26 |
| VD-31 | Prepare a state-bank tender kit: bilingual documents, a Wolfsberg and FATF evidence pack, references, and a Cabinet-procurement-friendly commercial structure; align with the K2 Integrity and EY frameworks used at Rafidain, Rasheed and TBI | IBN-K2; IBN-REFORM-JUN25; IBN-EY-2026 |
| VD-32 | Provide an Arabic name-matching benchmark against the references Iraqi banks already trust (World-Check data, FinScan, Firco) and publish precision and recall on a labelled Iraqi test set | LSEG-IRAQNA; CBI-BOOKLET; BOB-AR-2023 |
| VD-33 | Run a bank discovery checklist before any quote: CBS, database engine and version, character set, AML, screening, reporting, archiving and DWH tools (for example Pio-Tech Bank-BI, Laserfiche), goAML status, hosting, and the status of the bank's reform path | Sections 3-8, 12 |
| VD-34 | Offer Pio-Tech-compatible data export or import where a bank's warehouse holds the unified data model, so AKR-TMS can read a Bank-BI warehouse as a source instead of the CBS if the bank prefers | PIO-AML; PIO-REF |
| VD-35 | Track the CBI approved assessor list when published, and the CBI's list of Tier 1 CBS vendors if any, and update adapter priorities within a quarter | CBI-STANDARDS (footnote); CBI-FAQ-FEB26 |
| VD-36 | Include a customer-risk-rating (RBA) module that matches the CBI risk elements, because RBA is sold as a product to Iraqi banks and e-payment companies, is an assessor test item and is one of the four points a listed bank states in its annual report (one bank's auditor wrote in 2024 that customers "were not classified according to risk") | PIO-ASHUR; PIO-FASTPAY; CBI-GUIDE (D2); NBI-AR-2024; TIB-AR-2024 |
| VD-37 | Keep an evidence-grade register for market claims (A, B, C, N) and refresh the Iraqi market map every six months; the reform and CBS changes will outdate it | This file |
| VD-38 | Provide a documented path for banks barred from dollar transactions to buy and be supported by local invoicing and local support, and avoid dependence on foreign SaaS licence servers | Section 13; CBI-FAQ-FEB26 |
| VD-39 | Keep the CBI scenario pack as a versioned register (scenario number, name, source circular, effective date; 34 then 56 per the MER) and produce a coverage report per bank that maps the scenarios its current system runs to the current pack and lists the gaps, including bank-specific extension scenarios (two banks with different auditors say their system carries the 25 CBI minimum scenarios; Trans Iraq Bank adds a Pio-Tech set of 81, then 96), so that "we carry the CBI scenarios" is shown, not claimed | MER para 566; AMEEN-AR-2021; AMEEN-AR-2024; TIB-AR-2023; TIB-AR-2024; section 5 |
| VD-40 | Generate the annual evidence behind a listed Iraqi bank's AML statements: the four points of the Securities Commission disclosure (scenarios running, automatic periodic blacklist updates, customer risk classification, monitoring of account transactions) and the facts the external auditor writes next to them (systems and suppliers, link to the banking system, number of active CBI scenarios and pack version, share of customers risk-classified, alert review and closure statistics); export it as a signed yearly pack | NBI-AR-2023; NBI-AR-2024; BOB-AR-2023; AMEEN-AR-2021; AMEEN-AR-2024; TIB-AR-2024 |

## 17. Market map (bank | CBS | AML or screening system | source)

Grades: [A] bank or vendor primary statement, [B] trade press, [C] weak, [N] not found. "Pio-Tech ref" means the bank is listed on Pio-Tech's references page without a stated product. "Ban" marks the banks the CBI barred from dollar transactions in Feb 2024 or Feb 2025 (file 01 section 4). Counts are floors, not a census.

| Bank | CBS (vendor, product) | AML or screening system | Source |
|---|---|---|---|
| **State banks** | | | |
| Rafidain Bank | Misys Bankmaster, then Fusionbanking (2012); Oracle FLEXCUBE reported from 2018-19, go-live unconfirmed [B] | None found; K2 Integrity AML/CFT partnership (Aug 2025) | FF-RAFIDAIN; IBN-K2 |
| Rasheed Bank | Core banking system acquired, vendor not named [A] | None found | RASHEED-ABOUT |
| Trade Bank of Iraq | Misys Equation, Trade Innovation, FusionBanking Equation upgrade [A] | None found (Pio-Tech ref); K2i and KPMG advisers | GC-TBI-2008; TMI-TBI-2016; PIO-REF |
| Industrial, Real Estate, Agricultural banks | [N] | [N] (EY merger study) | IBN-EY-JAN24 |
| Central Bank of Iraq (regulator) | BML Istisharat ICBS [B] | Refinitiv and FinScan (remittance platform); goAML (UNODC) | FF-ICBS-2016; CBI-BOOKLET |
| **Private conventional banks** | | | |
| Bank of Baghdad | ICS BANKS ("BANKS") [A, inference] | FICO Siron (monitoring), FircoSoft Firco (transfer filtering) [A, inference]; KYC and FATCA system; Pio-Tech ref | BOB-AR-2022; BOB-AR-2023; BOB-AR-2025 |
| National Bank of Iraq | BML Istisharat ICBS to end-2023 [A]; Temenos Transact (T24) and Payments Hub from early 2024; Finastra trade finance [A] | "AML Sirion" (Siron by name, launched early 2024) [A, inference]; earlier "AML profiling system" renewed with Infrasoft (2023); "OFAC search engine" (vendor not named); goAML live 2023; Pio-Tech ref | TEM-NBI; NBI-AR-2023; NBI-AR-2024; NBI-AR-2025; PIO-REF |
| Mansour Bank (QNB 51%) | ICS BANKS [A] | None found; Pio-Tech ref | ICS-MANSOUR; PIO-REF |
| Ashur International Bank (Ban 2024) | ICS BANKS since 2007 [A] | Pio-Tech RBA solution (Jan 2024) | ICS-SUMMIT23; PIO-ASHUR |
| Hammurabi Commercial Bank (Ban 2024) | ICS BANKS [A] | None found; Pio-Tech ref | ICS-HAMMURABI; PIO-REF |
| First Iraqi Bank (digital) | ICS BANKS Digital [A] | Pio-Tech credit bureau solution | ICS-FIB-LIVE; PIO-FIB |
| Banque Libano-Francaise (Iraq) | Temenos T24 R17 [B-] | [N] | FF-BLF |
| Cihan Bank (Erbil) | Path iMAL since 2009 [B] | Pio-Tech ref | FF-AMEEN; PIO-REF |
| Trans Iraq Bank | "banks system" (probably ICS BANKS) [A, inference] | **Capital Banking Solutions AML** (CapitalCompliance); **FircoSoft** (via United Engineers Group); **Refinitiv World-Check**; goAML; Black List "supplied by Banks"; **Pio-Tech** contracted for 81 (2023) then 96 (2024) scenarios on top of the 25 CBI minimum [A]; Pio-Tech ref | TIB-AR-2022; TIB-AR-2023; TIB-AR-2024; PIO-REF |
| Gulf Commercial Bank | "BANKS" (probably ICS BANKS) [A, inference] | [N]; Pio-Tech ref | GCB-AR-2024; PIO-REF |
| Region Trade Bank; Investment Bank of Iraq (Ban 2024); Mosul Bank; Bank al Etihad Iraq | [N] | Pio-Tech refs | PIO-REF |
| Iraqi Middle East Investment Bank | **Capital Banking Solutions** core since 2006; CapitalDigital live Jan 2022 [A, vendor release] | [N] | CBSOL-IMEIB |
| Sumer Commercial Bank | **Capital Banking Solutions** CapitalBanker core and CapitalDigital (selected Aug 2022; partner since 2015) [A, vendor release] | **CapitalCompliance** with goAML and CRS [A, vendor release] | CBSOL-SUMER |
| Credit Bank of Iraq; Commercial Bank of Iraq; Union Bank of Iraq (Ban 2024); Warka; North Bank; Dar Es Salaam; Al-Huda (Ban 2024) | [N] | [N] | - |
| **Islamic banks** | | | |
| Al-Arabiya Islamic Bank (probably "Arabia Islamic Bank", Ban 2024) | ICS BANKS Islamic [A] | Pio-Tech ref | ICS-ARABIYA; PIO-REF |
| Al Janoob Islamic Bank (Ban 2024) | ICS BANKS [B] | [N] | FF-IIB-MAR17 |
| Taawen Islamic Bank | ICS BANKS [B] | [N] | FF-IIB-MAR17 |
| Noor Al Iraq Islamic Bank | ICS BANKS [B] | Pio-Tech ref | FF-IIB-MAR17; PIO-REF |
| International Islamic Bank (IMTB) | ICS BANKS Islamic [B] | **Pio-Tech goAML** (live Apr 2024) | FF-IIB-OCT17; PIO-IMTB |
| Al-Thiqa Islamic Bank | ICS BANKS Islamic [A] | Pio-Tech ref | ICS-THIQA; PIO-REF |
| Al Mustashar Islamic Bank | ICS BANKS Islamic [A] | Pio-Tech ref | ICS-MUSTASHAR; PIO-REF |
| UR Islamic Bank | ICS BANKS Islamic [A] | [N] | ICS-UR-LIVE |
| Al Sanam Islamic Bank (Ban 2025) | ICS BANKS Islamic [A] | Pio-Tech ref | ICS-SANAM; PIO-REF |
| Al-Ataa Islamic Bank | ICS BANKS Islamic [A] | Pio-Tech ref | ICS-ATAA; PIO-REF |
| Zain Iraq Islamic Bank | Path iMAL [A] | [N] | FFNEWS-ZAIN |
| Al-Rajih Islamic Bank | Path iMAL [B] | Pio-Tech ref | FF-ALRAJIH; PIO-REF |
| Ameen Al-Iraq Islamic Bank (probably Ban 2025) | Path iMAL (signed Apr 2019 [B]; "IMAL" in annual reports 2021-2024 [A]) | **Azentio (ex-Path Solutions) AML** and **Refinitiv World-Check** ("25 scenarios"; annual reports 2021-2024) [A] | FF-AMEEN; AMEEN-AR-2021; AMEEN-AR-2022; AMEEN-AR-2023; AMEEN-AR-2024 |
| Kurdistan International Islamic Bank (Ban 2024) | Path iMAL [A] | **Azentio AMLOCK** (all branches, Feb 2025) | AZ-KIB-IMAL; AZ-KIIB-AML |
| International Development Bank (IDB Iraq) | Path iMAL R14 [B] | [N] | FIT-IDB |
| Iraqi Islamic Bank for Investment and Development | [N] | **Pio-Tech goAML** (Feb 2024) | PIO-IIB |
| Elaf Islamic; Nasik Islamic; Asia Iraq Islamic; Taif Islamic (receivership Sep 2026) | [N] | Pio-Tech refs | PIO-REF; IBN-TAIF |
| **Kurdistan banks** | | | |
| Erbil Bank; Minare Bank | BML Istisharat ICBS [B] | [N] | FF-ICBS-2016 |
| Al Qurtas Islamic Bank (Erbil) | BML Istisharat ICBS [B] | [N] | FF-QURTAS |
| **E-payment and fintech** | | | |
| ZainCash | Temenos Transact and Payments Hub (Mar 2024) [A] | AML intended inside Temenos; Pio-Tech ref | TEM-ZC-SS; PIO-REF |
| FastPay | [N] | Pio-Tech RBA solution (Jan 2024) | PIO-FASTPAY |
| Qi (International Smart Card) | [N] | K2 Integrity 36-month AML/CFT programme; "QI Card" on an Eastnets logo wall [C] | IBN-QI-K2; file 14 |
| Asia Pay; Tabadul Electronic Payment Solutions; Iraq Stock Exchange | [N] | Pio-Tech refs | PIO-REF |
| **Foreign branches** | | | |
| Ziraat Bank; VakifBank | Group systems [N] | Pio-Tech refs | PIO-REF |

Counts from this map (named in a source): ICS BANKS 17 Iraqi institutions (Bank of Baghdad, Mansour, Ashur, Hammurabi, First Iraqi Bank, 10 Islamic banks, and Trans Iraq Bank and Gulf Commercial Bank by the product name "BANKS" in their annual reports); Path iMAL 6 (Cihan, KIIB, Zain Iraq, Al-Rajih, Ameen, IDB); BML ICBS 5 (CBI, Erbil Bank, Minare, Al Qurtas and NBI, which ran it to the end of 2023 [A]); Temenos 3 (NBI, ZainCash, BLF); Finastra/Misys 2 (TBI, Rafidain before 2019) plus NBI trade finance; Capital Banking Solutions core 2 named (IMEIB, Sumer; the vendor claims "more than 25 Iraqi" banks for its products); Oracle FLEXCUBE 1 reported (Rafidain). AML-side tools named at banks: Azentio/Path AML 2 (Ameen, KIIB), Capital Banking Solutions AML 2 (Trans Iraq Bank by its auditor; Sumer by the vendor's release), FICO Siron 2 (Bank of Baghdad; National Bank of Iraq as "AML Sirion"), FircoSoft Firco 2 (Bank of Baghdad, Trans Iraq Bank), Pio-Tech goAML or RBA releases at 4 institutions (IMTB, Iraqi Islamic Bank, Ashur, FastPay) plus a scenario contract at Trans Iraq Bank, with 29 bank references; Refinitiv World-Check named by 2 banks (Ameen, Trans Iraq Bank). The census would show far more ICS customers (the vendor claims 30+).

## 18. Competitor table (vendor | strengths in Iraq | weaknesses | likely price band)

Price bands are estimates built from the signals in section 13 (not quotes): **Entry** below about USD 100k a year per bank; **Mid** about USD 100k to 400k; **High** above about USD 400k. All are hypotheses to test.

| Vendor and product | Strengths in Iraq (evidence) | Weaknesses | Likely price band |
|---|---|---|---|
| **Pio-Tech** (Jordan; Bank-BI AML, goAML, RBA, FATCA, credit bureau) | Most Iraqi references of any AML-side vendor (29 banks plus 6 non-bank institutions listed; 5 dated 2024 releases); goAML and RBA already live at IMTB, Iraqi Islamic Bank, Ashur, FastPay; scenario implementer at Trans Iraq Bank (81 scenarios contracted in 2023, 96 in 2024, per the bank's auditor); Arabic names translator; Amman-based, regional channel; same data-warehouse design as AKR-TMS | Reference page does not say which product each bank uses; detection depth unclear (the AML product lists generic transaction monitoring and scenarios; its AI product launched in 2026 with no public results); no published accuracy; vendor claims only | Entry to Mid (estimate) |
| **Azentio** (AMLOCK and FCM; iMAL) | Two confirmed Iraqi AML deployments, both integrated with iMAL: Ameen Al-Iraq (in its annual reports since 2021, with the CBI minimum scenarios counted as 25) and KIIB (2024-25); large rule library (150+ scenarios; 400+ rules); Celent "noteworthy"; 20+ core systems; FCM Lite for small institutions; Saudi page shows Arabic and regional reporting | Iraq footprint small; no goAML shown in Iraq; Ameen's 25 scenarios against the CBI's 56 (MER) suggests a partial or old pack; claims unvalidated; Path-era Iraqi base is small versus ICS | Mid (estimate; Celent cites "price performance") |
| **Capital Banking Solutions** (CapitalCompliance AML; CapitalBanker core) | AML in production at Trans Iraq Bank (auditor, 2022 to 2024); goAML and CRS added at Sumer (vendor release, Aug 2022); a core banking and digital base claimed at "more than 25 Iraqi" banks, IMEIB since 2006; one platform for transaction monitoring, name screening, FATCA, CRS and reporting; on-cloud or on-premises; cites a Forrester vendor listing | No public AML accuracy figures or Arabic name-matching statement; Trans Iraq Bank added a Pio-Tech scenario set (81 then 96) beyond the 25 CBI minimum and the sources do not say why; all counts are vendor statements; its current website was not reachable | Not known (Entry to Mid, estimate) |
| **FICO TONBELLER Siron** (AML) | In production at Bank of Baghdad with CBI scenarios and chosen by National Bank of Iraq in 2023-24 ("AML Sirion", next to its Temenos migration); established enterprise product (file 11) | Daily batch; scenario extension appears to be a vendor-assisted task (Bank of Baghdad "developed and activated some special scenarios"); licence and maintenance cost; Arabic not shown | Mid to High (estimate) |
| **FircoSoft Firco** (filtering; LexisNexis Risk Solutions) | In the payment flow at Bank of Baghdad (pre-execution stop) and named at Trans Iraq Bank, where a local supplier (United Engineers Group) provides it; strong in SWIFT filtering (file 14) | Screening only; no monitoring; file 14 found no Iraqi reference, so the two annual-report statements are the only Iraqi evidence; at Trans Iraq Bank it was not linked to the banking system in 2022 | Mid (estimate) |
| **LSEG World-Check** (data) | Named by the CBI controls; local channel (Iraqna IBTS, 2020); used by the CBI itself (Jan 2023) | Data only, not a system; per-seat or per-screen licences | Entry to Mid (data licence) |
| **FinScan** (Innovative Systems) | Used by the CBI's remittance platform for name screening | No Iraqi bank customer found; US vendor | Entry to Mid (a modelled USD 100-300k+ a year for enterprise, low reliability) |
| **Temenos FCM** (with Transact) | 300+ banks globally; pre-integrated with Temenos CBS (NBI, ZainCash, BLF); FCM used in Jordan (Cairo Amman Bank) under a CBJ mandate; SaaS and AI agent | No Iraqi FCM customer; NBI and ZainCash releases mention core and payments only; ZainCash only intends AML on Temenos; SaaS clashes with hosting in Iraq; Arabic matching not stated | Mid (SaaS, pay as you go) to High (on-premises) |
| **Oracle FCCM** (with FLEXCUBE via OFSAA) | Rafidain's reported FLEXCUBE; OFSAA staging gives a documented read-only pattern; strongest enterprise functionality (file 09) | No Iraqi presence; daily EOFI feed; matcher ignores Arabic characters (file 09); heavy licence and database cost | High |
| **NICE Actimize, SAS, Quantexa, ThetaRay and similar suites** | Global references (Mashreq on SAS; UAE Exchange on Actimize); strong analytics | No Iraqi customer, partner or office; USD 0.45m a year average at Actimize; NICE and ThetaRay are Israeli-headquartered (legal exposure under Iraq's 2022 law, file 12); SaaS-only offers conflict with hosting in Iraq | High |
| **Eastnets SafeWatch** | SWIFT service bureau and screening in the region; screening cases in the UAE, Jordan, Egypt; SWIFT user-group ties | No Iraqi bank case study (logo "QI Card" only [C]); screening and messaging, not CBI scenarios | Mid (estimate) |
| **ICS Financial Systems** (CBS, no AML) | 30+ Iraqi customers; 15+ named; user-group summits; Oracle base; partners Huawei, Fortinet, Oracle; possible channel | No AML engine; "Tier 1" risk; a gatekeeper to its customers' CBS data | Not an AML competitor; channel or gatekeeper |
| **Consultancies (K2 Integrity, EY, KPMG, Oliver Wyman)** | Define the target models at Rafidain, TBI, Qi and the CBI; EY and K2 run the CBI's remittance review | Not software vendors; may recommend vendors and shape RFP criteria | n/a (influencers) |
| **Regional small AML on Temenos marketplace (6 Sigma)** | MENA, Levant, Africa focus; Temenos listing | Seven behavioural rules listed; no Iraq reference found | Entry (estimate) |
| **UNODC goAML** (free, FIU side) | Mandatory platform for STRs; 415 entities connected | A reporting channel, not detection; banks must prepare the XML | Free (but integration cost) |
| **In-house builds at banks** | Banks already use "AML profiling" systems and scenarios on CBI lists; DBAs hold the CBS | The MER says banks lack expertise to build scenarios | Internal cost |

AKR-TMS positioning from the table: against Pio-Tech, win on detection quality, Arabic matching, evidence for the CBI assessor and a published accuracy test; against Azentio, Capital Banking Solutions and Temenos, win on proven coverage of the full current CBI scenario pack (Azentio's customer Ameen is credited with 25 against the CBI's 56), goAML, Iraqi data and price; against Siron and Firco, win on parallel-run quality and near-real-time reads; against the global suites, be the Iraqi-fit, on-premises, fixed-price alternative.

## 19. Sources

Every URL used, by the label used in the text. Wayback Machine URLs are copies of pages whose original site blocks automated access (FinTech Futures, Iraq Business News, parts of Finacle) or no longer answers (Path Solutions, Capital Banking Solutions). Arabic-language bank annual reports (Ameen Al-Iraq, Trans Iraq Bank, Gulf Commercial Bank) were read as page images because their text layer is missing or incomplete, and the National Bank of Iraq 2023 report through its text layer after Unicode normalisation of Arabic presentation forms; quoted phrases from them are the researcher's translation, and page numbers are PDF page numbers. Evidence grades are explained at the top of file 17 section 1 (A, B, B-, C, K) and in the header of file 18 (A, B, C, N). Pages read in October 2026.

- **MER**: MENAFATF Mutual Evaluation Report of Iraq, May 2024 (read from the local text copy iraq-mer-2024-en.txt). https://www.menafatf.org/sites/default/files/Newsletter/IRAQ%20MER_En_0.pdf
- **TEM-NBI**: Temenos press release, 25 Apr 2024: National Bank of Iraq goes live (core banking and payments; delivery partner ITSS). https://www.temenos.com/press_release/national-bank-of-iraq-goes-live-with-temenos-core-banking-and-payments/
- **TEM-ZC**: Temenos press release, 23 May 2024: ZainCash goes live. https://www.temenos.com/press_release/iraqi-fintech-zaincash-goes-live-with-temenos-core-banking-and-payments-to-scale-digital-payment-services/
- **TEM-ZC-SS**: Temenos success story: ZainCash (Temenos core + Payments Hub, Prema Consulting, TIBCO middleware, 700,000 wallets migrated). https://www.temenos.com/success-story/zain-cash-success-story/
- **TEM-FCM-SAAS**: Temenos press release, 12 Oct 2022: FCM-as-a-Service (over 300 banks). https://www.temenos.com/press_release/temenos-launches-self-service-financial-crime-mitigation-on-temenos-banking-cloud/
- **TEM-SITEMAP**: Temenos sitemap index (used to enumerate press releases and success stories). https://www.temenos.com/sitemap_index.xml
- **FF-BLF**: FinTech Futures: Banque Libano-Francaise (Iraq) upgrades Temenos T24 (read only as a search-result summary; the site blocks automated access). https://www.fintechfutures.com/core-banking-technology/banque-libano-fran-aise-in-iraq-upgrades-temenos-t24-core-banking-system
- **IBN-NBI**: Iraq Business News, 11 Jan 2024: UK firms introduce new banking systems at National Bank of Iraq (Wayback copy). https://web.archive.org/web/20240118022256/https://www.iraq-businessnews.com/2024/01/11/uk-firms-introduce-new-banking-systems-at-national-bank-of-iraq/
- **AZ-KIB-IMAL**: Azentio release, 25 Jan 2024: Kurdistan International Islamic Bank adopts iMAL, with Azentio FCM/AML integrated. https://www.azentio.com/insights/news-and-announcements/kiib-digital-transformation-journey-successfully-kicked-off-by-azentio
- **AZ-KIIB-AML**: Azentio release, 28 Feb 2025: AMLOCK deployed at Kurdistan International Islamic Bank. https://www.azentio.com/insights/news-and-announcements/azentio-partners-with-kurdistan-international-islamic-bank-to-strengthen-anti-money-laundering-compliance-with-azentio-amlock
- **AZ-FCM-PDF**: Azentio Financial Crime Management and Anti-Fraud System brochure (100+ implementations, 20+ core systems, 150+ scenarios, FCM Lite). https://content.azentio.com/hubfs/file/azentio-fin-crime-anti-fraud-system.pdf
- **AZ-SAUDI**: Azentio AMLOCK page for Saudi Arabia (feature list, SPARK Matrix leader claim). https://www.azentio.com/aml-software-saudi-arabia
- **AZ-BUYERS**: Azentio AML software buyer's guide (no price figures). https://www.azentio.com/thought-leadership/aml-software-buyers-guide
- **FIT-IDB**: Financial IT: International Development Bank (Iraq) iMAL R14 upgrade, go-live 30 Sep 2018. https://financialit.net/news/core-banking/iraq-idbs-successful-upgrade-imal-r14
- **FFNEWS-ZAIN**: FF News, 27 Oct 2017: Zain Iraq Islamic Bank selects iMAL (Path Solutions); competing vendors named. https://ffnews.com/news/zain-bank-chooses-imal
- **FF-AMEEN**: FinTech Futures, 18 Apr 2019: Ameen Al-Iraq signs iMAL; Path's first Iraq customer Cihan Bank in 2009 (Wayback copy). https://web.archive.org/web/20200522194839/https://www.fintechfutures.com/2019/04/ameen-al-iraq-latest-addition-to-paths-iraq-user-base/
- **FF-ALRAJIH**: FinTech Futures, 13 Dec 2017: Al-Rajih Islamic Bank signs iMAL ahead of ICS Banks (Wayback copy). https://web.archive.org/web/20210305042753/https://www.fintechfutures.com/2017/12/path-solutions-beats-ics-financial-systems-to-core-banking-system-deal-in-iraq/
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
- **FF-IIB-MAR17**: FinTech Futures, 27 Mar 2017: International Islamic Bank (Iraq) signs ICS Banks; ICSFS has "around 20 users" (Wayback copy). https://web.archive.org/web/20210305042753/https://www.fintechfutures.com/2017/03/international-islamic-bank-picks-core-banking-system-and-vendor/
- **FF-IIB-OCT17**: FinTech Futures, 23 Oct 2017: International Islamic Bank live; ICSFS "over 20 customers in Iraq. Among them are Al-Arabiya Islamic Bank, Al Janoob Islamic Bank, Bank of Baghdad, Taawen Islamic Bank and Noor Al Iraq Islamic Bank" (Wayback copy). https://web.archive.org/web/20210305042753/https://www.fintechfutures.com/2017/10/international-islamic-bank-live-with-new-core-banking-system/
- **FF-RAFIDAIN**: FinTech Futures, 13 Mar 2019: Rafidain Bank is understood to be implementing Oracle FLEXCUBE Universal Banking, Digital Experience and Payments (Wayback copy). https://web.archive.org/web/20200621034622/https://www.fintechfutures.com/2019/03/iraqs-rafidain-bank-revamps-tech-with-oracle/
- **RBI-2008**: Retail Banker International, 29 Feb 2008: Rafidain Bank turnkey contract with Misys and B-Plan; Misys competed against Temenos and Systems Access. https://www.retailbankerinternational.com/news/iraqs-banks-look-to-the-future/
- **WIKI-RAFIDAIN**: Wikipedia: Rafidain Bank (2007 B-Plan and Misys turnkey contract; 146 domestic branches). https://en.wikipedia.org/wiki/Rafidain_Bank
- **GC-TBI-2008**: Global Custodian, 10 Nov 2008: Trade Bank of Iraq selects Misys Equation, Trade Innovation TI Plus 2.0 and Equation Branch Automation. https://www.globalcustodian.com/?p=27781
- **TMI-TBI-2016**: Treasury Management International, 8 Feb 2016: TBI selects Misys FusionBanking Equation and Teller (projects in 2003 and 2009; contract signed in Baghdad on 14 October). https://treasury-management.com/news/trade-bank-of-iraq-tbi-selects-misys-to-provide-world-class-banking-solutions
- **FIN-SITEMAP**: Finastra sitemap (enumerated product, press and customer pages; no Iraq pages). https://www.finastra.com/sitemap.xml
- **FINACLE-SITEMAP**: Infosys Finacle sitemap, Wayback snapshot 3 Jan 2026 (solution pages, client stories, App Center partners). https://web.archive.org/web/20260103080948/https://www.finacle.com/sitemap.xml
- **FF-QURTAS**: FinTech Futures, 7 Nov 2017: Al Qurtas Islamic Bank live on BML Istisharat ICBS (Wayback copy). https://web.archive.org/web/20210305042753/https://www.fintechfutures.com/2017/11/al-qurtas-islamic-bank-live-with-new-core-banking-system-bml-istisharats-icbs/
- **FF-ICBS-2016**: FinTech Futures, 24 Oct 2016: Al Qurtas selects ICBS; other Iraqi users CBI, Erbil Bank, Minare Bank, NBI (Wayback copy). https://web.archive.org/web/20210305042753/https://www.fintechfutures.com/2016/10/new-bank-in-iraq-selects-bml-istisharats-icbs-core-banking-system/
- **INT-SITEMAP**: Intellect Design Arena sitemap (no AML product page, no Iraq page). https://www.intellectdesign.com/sitemap_index.xml
- **PIO-HOME**: Pio-Tech (Jordan) home page: system integrator for Levant, GCC, North and East Africa; Bank-BI, Bank-BPM platforms. https://www.pio-tech.com/
- **PIO-REF**: Pio-Tech references page: client logos by country, 29 Iraqi banks and 5-6 Iraqi non-bank institutions (read from the page's embedded JSON, Oct 2026). https://www.pio-tech.com/reference
- **PIO-AML**: Pio-Tech AML system (part of Bank-BI): transaction monitoring, screening, case management, API integration. https://www.pio-tech.com/products/anti-money-laundering-system-aml
- **PIO-GOAML**: Pio-Tech goAML management system (Bank-BI goAML). https://www.pio-tech.com/products/goaml-management-system
- **PIO-AI**: Pio-Tech AI-Powered AML (behavioural intelligence engine, GenAI scenario assistant). https://www.pio-tech.com/the-next-evolution-of-your-aml-framework
- **PIO-ASHUR**: Pio-Tech news, 10 Jan 2024: Ashur International Bank deploys the RBA solution. https://www.pio-tech.com/news/ashur-bank-partners-with-pio-tech-to-execute-rba-solution
- **PIO-IIB**: Pio-Tech news, 25 Feb 2024: Iraqi Islamic Bank integrates goAML (14 branches). https://www.pio-tech.com/news/iraqi-islamic-bank-integrates-go-aml-solution
- **PIO-IMTB**: Pio-Tech news, 15 Apr 2024: International Islamic Bank (IMTB) goAML go-live. https://www.pio-tech.com/news/imtb-announces-successful-go-live-of-go-aml-system-from-pio-tech
- **PIO-FIB**: Pio-Tech news, 28 Jan 2024: First Iraqi Bank deploys credit bureau solution. https://www.pio-tech.com/news/first-iraqi-bank-deploys-credit-bureau-solution
- **PIO-FASTPAY**: Pio-Tech news, 17 Jan 2024: FastPay implements the RBA solution. https://www.pio-tech.com/news/fast-pay-implements-risk-based-approach-solution-from-pio-tech
- **PIO-IDB-EG**: Pio-Tech news, 28 Sep 2025: Industrial Development Bank (Egypt) implements AML and RBA. https://www.pio-tech.com/news/idb-and-pio-tech
- **PIO-NAMES**: Pio-Tech Names Bilingual Translator (Arabic names, machine learning). https://www.pio-tech.com/products/names-bilingual-translator
- **LSEG-IRAQNA**: LSEG/Refinitiv press release, 10 Mar 2020: World-Check partnership with Iraqna IBTS. https://www.lseg.com/en/media-centre/press-releases/refinitiv/2020/march/refinitiv-partners-with-iraqna-ibts-to-enhance-regulatory-compliance-efficiency-in-iraq
- **IBN-K2**: Iraq Business News, 18 Aug 2025: Rafidain Bank partnership with K2 Integrity (Wayback copy). https://web.archive.org/web/20260208161557/https://www.iraq-businessnews.com/2025/08/18/us-firm-to-strengthen-financial-compliance-in-iraq/
- **IBN-REFORM-JUN25**: Iraq Business News, 7 Jun 2025: banking sector overhaul (EY for state banks, K2i and KPMG for TBI, Oliver Wyman for private banks). https://web.archive.org/web/20260208161557/https://www.iraq-businessnews.com/2025/06/07/iraq-announces-major-banking-sector-overhaul/
- **IBN-EY-2026**: Iraq Business News, 26 Jan 2026: EY and Ministry of Finance on Rafidain and Rasheed restructuring (First Rafidain Bank). https://web.archive.org/web/20260208161557/https://www.iraq-businessnews.com/2026/01/26/ey-assists-restructuring-of-iraqi-state-banks-and-insurance/
- **IBN-PM-MAR25**: Iraq Business News, 25 Mar 2025: PM reviews reforms at Rafidain and Rasheed with EY; Rafidain First Bank. https://web.archive.org/web/20260208161557/https://www.iraq-businessnews.com/2025/03/25/pm-reviews-reforms-at-rafidain-and-rasheed-banks/
- **IBN-EY-JAN24**: Iraq Business News, 3 Jan 2024: Cabinet approves EY contracts for Rafidain/Rasheed, Industrial/Real Estate/Agricultural bank merger, insurance merger. https://web.archive.org/web/20260128130203/https://www.iraq-businessnews.com/2024/01/03/ey-to-advise-iraq-on-banking-and-insurance-reforms/
- **IBN-RASHEED-SANC**: Iraq Business News, 17 May 2022: Al-Rasheed Bank removed from UN and UK sanctions lists. https://web.archive.org/web/20260128130203/https://www.iraq-businessnews.com/2022/05/17/al-rasheed-bank-removed-from-sanctions-lists/
- **IBN-TBI-ESDS**: Iraq Business News, 20 Jun 2019: TBI memorandum with ESDS Software Solution (cloud, data centre, fintech). https://web.archive.org/web/20191130040521/http://www.iraq-businessnews.com/2019/06/20/tbi-partners-with-indian-firm-for-fintech-development/
- **CBI-REFORM-CIRC**: CBI Banking Reform Initiative 2025 circular for private commercial and Islamic banks (paths: continue, merge, liquidate; timeline; reform fee). https://cbi.iq/static/uploads/up/file-177615183531332.pdf
- **CBI-STANDARDS**: CBI Standards Booklet 2025 for private commercial and Islamic banks (standards B2, B7, B8, D2, D4). https://cbi.iq/static/uploads/up/file-17761583642351.pdf
- **CBI-GUIDE**: CBI Detailed Assessment Guide 2025 (core banking system, payment systems and AML/CFT/sanctions assessment criteria). https://cbi.iq/static/uploads/up/file-177615833416828.pdf
- **CBI-PHASE1**: CBI Phase 1 requirements assessment guide for continuing banks, March 2026 (submission 1 Apr to 30 Sep 2026; AML standard D2 needs a CBI-approved operational auditor). https://cbi.iq/static/uploads/up/file-177615176498571.pdf
- **CBI-MERGER**: CBI merger plan guide, March 2026 (core banking systems to be decommissioned are part of the plan). https://cbi.iq/static/uploads/up/file-177615197128794.pdf
- **CBI-FAQ-FEB26**: CBI banking reform FAQ, February 2026 (scanned; read as images: core banking Tier 1 vendors, hosting inside Iraq, external service providers). https://cbi.iq/static/uploads/up/file-177234428394860.pdf
- **CBI-BOOKLET**: CBI strategic achievements booklet 2020-2025 (licensed institutions, payment systems, remittance monitoring with Refinitiv/FinScan, K2 Integrity, EY; goAML figures). https://cbi.iq/static/uploads/up/file-177935854666319.pdf
- **CBI-REFORM-PAGE**: CBI web page "Banking Reform Project" listing the 2025-2026 reform documents. https://cbi.iq/page/172
- **BOB-AR-2025**: Bank of Baghdad annual report 2025 (banking system "BANKS", SIRON scenarios, KYC and FATCA system, Laserfiche, Nucleus, RTGS and ACH linkage). https://bankofbaghdad.com.iq/Resources/8/Annual_Report_2025_Eng.pdf
- **BOB-AR-2023**: Bank of Baghdad annual report 2023 (auditor's AML section: SIRON daily monitoring on CBI scenarios; FIRCO pre-execution screening of transfers). https://bankofbaghdad.com.iq/Resources/8/Annual_Report_2023_Eng.pdf
- **BOB-AR-2022**: Bank of Baghdad annual report 2022 (same SIRON and FIRCO statements). https://bankofbaghdad.com.iq/Resources/8/Annual_Report_2022_Eng.pdf
- **NBI-AR-2025**: National Bank of Iraq annual report 2025 (T24 banking system, OFAC search engine, AML profiling electronic system on CBI scenarios). https://www.nbi.iq/-/media/pdfs/fin---reports-2025/annual-report-2025-final-mini/annual-report-english-version-2025-final-mini_compressed.ashx
- **RASHEED-ABOUT**: Rasheed Bank website, about us: acquired a comprehensive core banking system (vendor not named); data centre and DR centre; Nakheel cards. https://rasheedbank.gov.iq/about-us/
- **ICS-BURGAN**: ICSFS release, Jul 2016: Burgan Bank (Kuwait; parent of Bank of Baghdad) live on ICS BANKS. https://icsfs.com/newsroom/burgan-bank-goes-live-on-ics-banks-from-ics-financial-systems/
- **IBN-TAIF**: Iraq Business News, 3 Sep 2026: Al-Taif Islamic Bank placed in receivership (Banking Law 94/2004 art. 59; 18-month custodian). https://web.archive.org/web/20260922030011/https://www.iraq-businessnews.com/2026/09/03/al-taif-islamic-bank-put-into-receivership/
- **IBN-AGBI**: Iraq Business News/AGBI, 13 Aug 2026: bank reforms could open the sector to regional investors; seven restricted banks reported to be reintegrating into dollar system. https://web.archive.org/web/20260922030011/https://www.iraq-businessnews.com/2026/08/13/iraqi-bank-reforms-could-open-up-sector-for-regional-investors/
- **IBN-CMC**: Iraq Business News, 6 Aug 2026: CBI and Communications and Media Commission on digital payments and AML. https://web.archive.org/web/20260922030011/https://www.iraq-businessnews.com/2026/08/06/cbi-and-cmc-in-talks-on-digital-payments-framework/
- **CBI-FOREIGN-CIRC**: CBI Banking Reform Initiative 2025 circular for foreign bank branches (continue or liquidate; USD 600,000 annual fee; USD 80m capital). https://cbi.iq/static/uploads/up/file-177615207684937.pdf
- **ZC-JOB**: Workable job advert: ZainCash Temenos T24 Developer, Baghdad. https://jobs.workable.com/jobs/979fa9a9-44c3-4a39-88ff-abbe3848fa7b.md
- **IBN-QI-K2**: Iraq Business News, 24 Jun 2025: Qi (International Smart Card) and K2 Integrity 36-month AML/CFT partnership. https://web.archive.org/web/20260416024524/https://www.iraq-businessnews.com/2025/06/24/qi-and-k2-integrity-to-elevate-iraqs-financial-standards/
- **IBN-WALLET**: Iraq Business News, 4 Sep 2025: CBI launches inter-wallet transfers. https://web.archive.org/web/20260723113514/https://www.iraq-businessnews.com/2025/09/04/cbi-launches-inter-wallet-money-transfers/
- **IBN-JPM-TBI**: Iraq Business News, 14 May 2025: JP Morgan STP award to TBI (99.68% MT103 success). https://web.archive.org/web/20260723113514/https://www.iraq-businessnews.com/2025/05/14/dollar-transfers-from-iraq-jp-morgan-awards-tbi/
- **IBN-SUPERQI**: Iraq Business News, 27 Dec 2024: SuperQi app with Ant International. https://web.archive.org/web/20260723113514/https://www.iraq-businessnews.com/2024/12/27/chinese-payment-firm-enables-iraqs-new-superqi-app/
- **IBN-EPAY-JUL26**: Iraq Business News, 22 Jul 2026: CBI meets licensed e-payment companies. https://web.archive.org/web/20260723113514/https://www.iraq-businessnews.com/2026/07/22/cbi-pushes-e-payment-firms-on-digital-expansion/
- **RBI-MASHREQ**: Retail Banker International: Mashreq Bank on Oracle FLEXCUBE Universal and Direct Banking (selected 2010; replaced 40+ legacy systems) (Wayback copy). https://web.archive.org/web/2024/https://www.retailbankerinternational.com/news/mashreq-bank-moves-towards-relationship-pricing-with-oracles-flexcube/
- **TEM-CAB**: Temenos success story: Cairo Amman Bank (Jordan), FCM suite upgrade Dec 2024 per Central Bank of Jordan mandate; false positives down 40-50%. https://www.temenos.com/success-story/cairo-amman-bank-success-story/
- **TEM-CBJ**: Temenos success story: Capital Bank of Jordan (core and Payments Hub from 2022; NBI migrated in 12 months; Riyadh branch). https://www.temenos.com/success-story/capital-bank-of-jordan-success-story/
- **TEM-ADCB-EG**: Temenos success story: ADCB Egypt (Temenos Payments Hub; ITSS partner). https://www.temenos.com/success-story/adcb-egypt/
- **ICS-RAJHI-JO**: ICSFS release: Al Rajhi Bank Jordan live on ICS BANKS Islamic and Digital. https://icsfs.com/newsroom/alrajhi-golive/
- **EAST-AIB**: Eastnets case study: Arab International Bank (Egypt), SafeWatch Screening 5 live March 2023. https://www.eastnets.com/case-studies/how-aib-cut-manual-processes-and-enhanced-security-with-safewatch-screening-5
- **EAST-CBI**: Eastnets case study: Commercial Bank International (UAE), SafeWatch Screening, PaymentSafe, SWIFT Hosting (Oct 2021). https://www.eastnets.com/case-studies/cbi-partnered-with-eastnets-to-strengthen-sanctions-screening
- **EAST-AJIB**: Eastnets case study: Arab Jordan Investment Bank (Jordan). https://www.eastnets.com/case-studies/eastnets-partners-with-arab-jordan-investment-bank-ajib
- **PS-GATEPAY**: ProgressSoft, 22 Jan 2019: PS-GatePay (ACH, RTGS, cheque gateway integrating with the banks' CBS) selected by five more Iraqi banks after a procurement, nine in total. https://www.progressoft.com/news/banks-in-iraq-automate-their-clearing-and-settlement-processes-using-ps-gatepay
- **TEM-ITSS**: Temenos partner page: ITSS (Geneva headquarters, 18+ offices, Temenos Regional Partner of 2019). https://www.temenos.com/partner/itss/
- **TEM-PREMA**: Temenos partner page: Prema Consulting (MENA; Transact, Infinity, Financial Crime Mitigation, Payments Hub; ZainCash story). https://www.temenos.com/partner/prema-consulting/
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
- Local research files cross-referenced (not URLs): 01-iraq-regulatory-framework.md, 02-iraq-ml-tf-risk-landscape.md, 08 to 16 vendor files, 17-vendor-cbs-native-aml.md, 22-arabic-name-matching.md, 25-core-banking-integration.md, 27-transaction-monitoring-scenarios.md.
