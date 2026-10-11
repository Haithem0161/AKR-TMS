# 35 — Vendors: MemberCheck and NameScan (Neurocom group, Australia)

Status: done (2026-10-11); checked and corrected the same day against research/_membercheck/90-verification.md

Vendor profile of the Australian screening brands MemberCheck and NameScan. It was prompted by a rival the owner heard of as "something in the lines of prototype member check", "its by some guy named mohammed" and "its a local iraqi system (i think im not really sure)" (the owner's words; typing slips corrected).
Cross-references: file 14 (screening engines; its section 1 holds the twelve-need yardstick (eight rated in file 35)), file 15 (risk data providers; World-Check and the data side), file 18 (MENA and Iraq market), file 22 (Arabic name matching), file 34 (Prototype for Information Technology, the Iraqi company; written after this file on 2026-10-11).
Source keys in square brackets are defined in section 13. Factual sentences and table rows end with one or more keys. Marks used: "the vendor says" means marketing or self-description with no outside check; "[INFERENCE]" means our own reading, not a stated fact.

Terms used in this file (explained once here, because the file is written for a reader who is not a compliance specialist):
- **AML, KYC:** anti-money-laundering rules and "know your customer" checks that a bank must run on its customers.
- **Sanctions list:** an official list of people, firms or countries the law forbids dealing with (for example the United Nations list,
  the US OFAC list, the EU list and the UK HM Treasury list, written HMT). The **Iraqi national list** is the list of freezing decisions of
  Iraq's Committee for Freezing Terrorist Funds (file 14, section 1). **FATF:** the Financial Action Task Force, which sets international AML standards.
- **PEP:** politically exposed person, someone with a senior public job (or a relative or close associate of one) who carries a higher
  corruption risk. **Adverse media:** negative news stories about a person or company. **Watchlist:** a loose word for all such lists.
- **Screening:** comparing a name with those lists. A **hit** is a possible same-person. A **false positive** is a hit that turns out
  to be someone else. **Fuzzy matching** finds names that are close but not identical. **Transliteration** writes a name from one
  alphabet in another, such as Arabic in Latin letters.
- **API:** a way for one computer system to call another automatically. **Sub-processor:** an outside company that handles a
  vendor's customer data on the vendor's behalf. **CBS:** a bank's core banking system, its main ledger. **FIU:** the government financial
  intelligence unit that receives suspicious-transaction reports. **goAML:** a filing system for those reports (file 23). **MLRO:** the bank's money
  laundering reporting officer.
- **Reseller and white label:** a reseller sells the maker's product under the maker's name. A white-label partner sells it under its own
  name, so the buyer may never see the maker's name.

## Contents

1. Who they are
2. What they sell
3. How it works, part one: the data
4. How it works, part two: matching and technology
5. Partner models: reseller, white label, licensing
6. Arabic, Iraq and the Middle East
7. Pricing, customers and reviews
8. What it does right
9. What it does wrong or leaves unproven
10. Fit against eight of the twelve needs of an Iraqi bank
11. What it means for AKR-TMS
12. Open questions and things not found
13. Sources

## Findings in brief

1. **Who, what, how, in three sentences.** MemberCheck is an Australian cloud service run by MEMBER CHECK PTY LIMITED of Chatswood, Sydney; the vendor says it
   sits in the Neurocom group (the business register shows no link between MEMBER CHECK PTY LIMITED and NEUROCOM PTY LTD; the logo image's alt text says
   "a Nexiant brand"), with the sister brand NameScan selling pay-per-search checks. [ABR-MC] [ABR-NC]
   [MC-ABOUT] [NS-ABOUT] It sells name screening against sanctions lists, politically exposed persons (PEPs) and bad news, plus identity checks,
   company checks, risk rating and transaction monitoring, delivered as a vendor-run cloud service as far as its pages say (no installed option is named
   for direct customers). [MC-PRODUCTS] [MC-FAQ] It works from bought-in list data
   whose supplier it does not name (Acuris is the only candidate on its own pages), fuzzy name matching with thresholds the customer sets, and an API
   whose reference it withholds. [MC-PRIV] [MC-SUBPROC] [MC-FAQ] [MC-API]
2. **Nothing names an Iraqi customer, partner or seller of MemberCheck or NameScan, and no vendor page uses the word "Prototype".** English and Arabic
   searches (working notes 06, 07 and 12) found no Iraqi user either. MemberCheck's Iraq page claims "Screening coverage for Iraq" and NameScan's 2024
   coverage table lists Iraq, but neither names an Iraqi list. One GetApp review (2024-10-28) by a "Mohammed", "CEO and Founder" of an IT-services firm,
   says his firm distributes and develops products provided by MemberCheck; no country is given. The only tie to Prototype runs the other way: the Baghdad
   company Prototype for Information Technology lists "Namescan" as a technology partner on its own site (file 34); that is Prototype's claim and no
   NameScan page confirms it. The owner's words match no MemberCheck or NameScan page read. [INFERENCE] "MemberCheck" is the likeliest sound-alike (file 34
   covers the Iraqi company). [MC-IRAQ] [MC-PARTNER] [GA-ARCH] [NOTE-IRAQ] [NOTE-NAMES] [NOTE-P12]
3. **Who owns it is not settled.** The About page says Neurocom Group (established 1992); the logo image's alt text says Nexiant; the register shows a separate NEXIANT AI
   company from 15 December 2023. The founding year is 2008 for the vendor and 21 December 2007 on the register. No owner, director or
   executive is named on any vendor page. [MC-ABOUT] [MC-HOME] [ABR-NX] [ABR-MC]
4. **The list supplier is unnamed and the clues conflict.** Acuris is on MemberCheck's sub-processor list, and NameScan says its Sapphire tier is powered by Acuris; a
   2012 press item says Dow Jones; a directory says LexisNexis WorldCompliance and may confuse the two brands. [MC-SUBPROC] [NS-FAQ] [GR-PRESS] [ZH-MC]
5. **Against eight of the twelve needs of an Iraqi bank (file 14, section 1) it meets none outright:** partly 3, does not meet 2, unknown 3. The stated list refresh is 24 hours
   against the Central Bank's 12; it is not the World-Check system the Central Bank names; no Iraqi list is shown; it is cloud only as far
   as its pages say. (Section 10.) [MC-PEP] [MC-CMP-WC] [MC-IRAQ] [MC-FAQ]
6. **Arabic is claimed on one page and absent from another.** The FAQ lists English, French and Japanese; the enterprise comparison page lists Arabic
   and says names are matched "in original scripts". Nothing describes the Iraqi name chain, and the listed input fields include no mother's name or tribe. [MC-FAQ] [MC-CMP-ENT] [MC-HOME]
7. **Its public numbers disagree with each other.** "13,000+" customers against 16 in production and one case study; freshness worded five ways
   (real-time to a few days); integration "<24h" against "three and four weeks"; "One platform" yet the monitoring page says it has its own credentials and links to a sign-up page on a Nexiant host;
   ISO 27001 "certified" (footer badge alt text; "holds this certification" in the FAQ) against "aligned" in a comparison table. [MC-HOME] [MC-CASES] [MC-FAQ] [MC-PEP] [MC-TM] [MC-CMP-WC]
8. **If a local firm sold it, white label is the form that would hide it.** The vendor offers resale, white label and licensing and names no
   partner. NameScan's API is published (a key is needed) and may be usable without a partnership; NameScan also runs a partner programme (referrals,
   distributors, application integrators), and its resale terms were not checked. No seller is confirmed by either vendor; the one Iraqi firm that names
   Namescan as a partner is covered in file 34 (its own claim). (Section 5.) [MC-PARTNER] [NS-API] [NS-PARTNERSHIP]
9. **For AKR-TMS it may be a rival on price, speed of start and bundle (vendor claims, none tested); its depth is unknown because the matching rules are
   unpublished.** The planned edge of AKR-TMS is what MemberCheck's pages do not show: operation on the bank's servers, the Iraqi list, a
   12-hour refresh, Iraqi name-chain matching, the bank-database link, goAML and a pre-release check. Worth copying: "cannot determine" as a
   result and a per-match evidence record. NameScan could be an optional online lookup. (Section 11.) [MC-ABOUT] [MC-PEP] [NS-API]
10. **Everything here comes from public pages and registers.** Nothing was tested; no demo was requested, no account made, no one contacted.
    Vendor claims are marked as such, and what is not known is in section 12.

---

## 1. Who they are

**In short.** MemberCheck is a cloud service for AML and KYC checks, run by an Australian private company, MEMBER CHECK PTY LIMITED, in
Chatswood, a suburb of Sydney. [ABR-MC] [MC-PRIV] NameScan is a sister brand that sells PEP and sanctions checks paid per search. [NC-HOME]
[NS-HOME] The vendor says both belong to the Neurocom group, "established in 1992". [MC-ABOUT] No source found names the owner of the group
or of either company. [ABR-MC] [OC-MC]

### 1.1 Legal entities and register facts

The Australian Business Register (ABR) is the government list of business numbers (ABN: business number; ACN: company number). It shows no
owners, shareholders or directors for any of the three entities below. [ABR-MC] [ABR-NC] [ABR-NX]

| Entity | What the register shows | Source |
|---|---|---|
| MEMBER CHECK PTY LIMITED | ABN 64 129 012 344, ACN 129 012 344, "Australian Private Company"; "Active from 21 Dec 2007"; main location "NSW 2067" (Chatswood); one business name, RISKGUARD, from 21 September 2021 (its use is not found); no business name "MemberCheck" is listed | [ABR-MC] |
| NEUROCOM PTY LTD | ABN 12 078 580 382, ACN 078 580 382, "Australian Private Company"; "Active from 19 Apr 2000"; "NSW 2067"; business names CREATIVE DIGITAL TECHNOLOGY (from 25 March 2010), IDSPHERE (from 18 March 2011) and "Name Scan" (from 11 October 2021), so NameScan trades under this company | [ABR-NC] |
| NEXIANT AI PTY LTD | ABN 90 673 693 019; "Active from 15 Dec 2023"; "NSW 2067"; business names NEXIANT AI (from 18 December 2023) and FRAUDSHIELD (from 28 August 2024) | [ABR-NX] |

- The privacy policy names "Member Check Pty Limited ACN 129 012 344" as "the owner and operator of the MemberCheck service". The legal name is
  spelled four ways across sources; the ACN does not change. [MC-PRIV] [ABR-MC] [MC-WAYBACK] The ASIC register (directors, owners,
  incorporation date) refused our tools. OpenCorporates (secondary) shows an incorporation filing dated 21 December 2007 and no officers. [ASIC-MC] [OC-MC]

### 1.2 The group: Neurocom, NameScan, GPayments, CDT and Nexiant

- **Neurocom.** The vendor says "MemberCheck operates under Neurocom Group, established in 1992, which also includes NameScan, GPayments and
  Creative Digital Technology." [MC-ABOUT] The group site says its first business, Creative Digital Technology (CDT), "is established to provide
  innovative information technology solutions to enterprises and the Government of Australia", and that "Neurocom Pty Ltd is registered – and uses
  the name of Creative Digital Technology, otherwise known as CDT". The 1992 date is the vendor's claim; no register record supports it. [NC-HOME] [ABR-NC]
- **GPayments and NameScan.** GPayments calls MemberCheck "a sister company" and says it "is part of the Neurocom group". [GP-HOME] [GP-ABOUT]
  NameScan's FAQ footer reads "a member of the Neurocom group", and its About page (re-read by the writer on 11 October 2026) names
  Creative Digital Technology, GPayments and Member Check as the other members. [NS-FAQ] [NS-ABOUT] That page lists Acuris Risk Intelligence
  under "Our Partners", not as a group member; one working note read it the other way. [NS-ABOUT]
- **Nexiant.** The group site says "Nexiant transforms Fraud Prevention and Risk Management by merging the strengths of leading solutions from
  GPayments, MemberCheck, FraudShield and NameScan". Nexiant's own site says it "is not related to other companies with similar names".
  [NC-HOME] [NX-ABOUT] The logo image on MemberCheck's pages carries the alt text (the hidden caption of an image) "MemberCheck, a Nexiant brand" (present on
  every page checked in the raw HTML; whether the drawn logo shows it was not checked). The lead's live read of the About page on 11 October 2026
  returned the same words. [MC-ABOUT] [MC-HOME]
- **Neurocom versus Nexiant is not settled.** The About page names Neurocom as the parent and the logo image's alt text names Nexiant. Monitoring has its
  own credentials and links to a sign-up page on a Nexiant address, and the register shows NEXIANT AI PTY LTD in the same postcode. [MC-TM] [FS-LOGIN] [ABR-NX] [INFERENCE] Nexiant looks like a
  newer brand of the same group, not a separate owner. A directory's "now part of Nexiant" has no primary backing. [ZH-MC] No source says who owns any of them. [NC-HOME]

### 1.3 People

The vendor names no founder, director, executive or staff member on its About page. [MC-ABOUT] The names below come from the group site and
secondary pages; none is tied to MemberCheck by a vendor page.

| Name | Role as shown | Source and strength |
|---|---|---|
| Bahram Boutorabi | "CEO/CTO & Founder of CDT"; a testimonial line reads "Bahram, Co-founder and CEO of Neurocom" (beside placeholder "Lorem Ipsum" text, so possibly a template leftover); CB Insights lists him as chief executive | [NC-HOME] [NC-WAYBACK] primary; [CBI-PEOPLE] secondary; current role not confirmed |
| Ross Edenborough | "Neurocom chief executive", in a press item of 6 November 2012 | [GR-PRESS] secondary; no later source |
| Harmen Sandhu, Shaun Thomas, Mei Chua | Sandhu "AML & Risk Management Specialist" and Thomas "Head of Corporate Development" (an undated event post); Mei Chua listed with no title | [LI-MC] secondary |

- Who founded MemberCheck itself is not stated in any source the research agents read (ASIC was not reachable). The only named founder is tied to CDT,
  the group's first business; no source read ties him to MemberCheck. [INFERENCE] [NC-HOME]
- **One person named Mohammed is tied to MemberCheck in a page the agents downloaded.** A GetApp review syndicated from Capterra (2024-10-28) is signed
  "mohammed habeeb", "CEO and Founder", industry "Information Technology and Services", company-size code "B", and says: "Our experience was very valuable
  as it increased financial revenues through distributing and developing products provided by Member check Company." The review gives no country and no
  company name, so nothing ties him to Iraq or to Prototype. [GA-ARCH] No other MemberCheck, group, GPayments, NameScan, LinkedIn or CB Insights page names a
  Mohammed; LinkedIn hides most of its staff list ("View 12 employees"), so this is not conclusive. [MC-ABOUT] [LI-MC]
- The same review lists as a drawback "It is preferable to add control over the number of searches at the organization or sub-organization level."
  [INFERENCE] That request reads like an account that runs sub-organisations for its own clients, which fits the reseller or white-label model of section 5
  (separate client organisations). It is our reading, not a stated fact. [GA-ARCH]
- The lead's follow-up: four web searches on 2026-10-11 (the name with Prototype's English name and Iraq; with CEO, Founder, Prototype, Baghdad and fintech
  terms, where the search tool reported that it dropped the name, so that one is weak; with MemberCheck and distributor; the Arabic form of the name with the
  Arabic title of Prototype's Facebook page) found no page that ties the reviewer to Prototype, to Iraq or to MemberCheck. The name is common, so a search
  cannot settle it. Nobody was contacted. [LEAD-SEARCH]
- To state it plainly: the review ties a Mohammed to MemberCheck distribution; no source ties him to Prototype or to Iraq. [GA-ARCH] [LEAD-SEARCH]

### 1.4 History

| Date | Event | Source |
|---|---|---|
| 1992 | Group "established" (vendor claim); first business was CDT | [MC-ABOUT] [NC-HOME] |
| 21 December 2007 | MEMBER CHECK PTY LIMITED number active; incorporation application filed (secondary) | [ABR-MC] [OC-MC] |
| 2008 | Vendor: "MemberCheck was established in 2008 to help organisations manage the risk in who they do business with" | [MC-ABOUT] |
| 6 November 2012 | Press: "Australian IT firm Neurocom offers a software program called Membercheck", with watchlists "compiled by Dow Jones featuring more than 600,000 names" | [GR-PRESS] |
| 2015 | Group site: "Since 2015, NameScan has provided a contractless, Pay-As-You-Go risk management solution"; an archived MemberCheck About page (copyright footer 2021; capture date unknown) says NameScan has been a PEP and sanctions check provider "since 2008" (not reconciled) | [NC-HOME] [MC-WAYBACK] |

- **Founding year conflict.** The vendor says 2008; CB Insights and LinkedIn say 2007, and the register date is 21 December 2007. [MC-ABOUT] [CBI-MC]
  [LI-MC] [ABR-MC] We take the company as existing from 21 December 2007 (a government record) and the service as starting in 2008 (the vendor's
  word). [INFERENCE] 2007 may be registration and 2008 the start of trading; this is not verified.

### 1.5 Offices, size, certifications, public trouble

- **Head office:** Chatswood, New South Wales, "Suite 213, 7 Railway St, Chatswood NSW 2067, Australia"; the privacy contact is "PO Box 160 Chatswood
  NSW 2057", also shown by NameScan. [NS-HOME] [CBI-MC] [LI-MC] [MC-PRIV] **Other offices:** "Our offices are in Sydney, Ankara, and Tokyo."
  [MC-CAREERS] The Japan page repeats Ankara; no third-party source confirms it. [MC-JAPAN] [NOTE-IRAQ] **Size:** LinkedIn shows "51-200 employees"; the vendor states no headcount. [LI-MC] [MC-ABOUT]
- **Certifications (vendor claims).** The footer shows the badges "ISO/IEC 27001:2022", "ISO/IEC 27701:2019", "GDPR" and "DORA" (the badge images carry the
  hidden alt text "ISO/IEC 27001:2022 certified" and "ISO/IEC 27701:2019 certified"), with no link, certifier, certificate number,
  scope or date. [MC-HOME] (ISO 27001 is an information-security standard; GDPR is the EU data-protection law; the PEP page glosses DORA as
  "Operational resilience for EU financial entities and their vendors".) [MC-FAQ] [MC-PEP] The group site says "MemberCheck achieves ISO27001:2013
  certification" (no certifier or date, and a different version); a comparison page says "ISO 27001-aligned, within an EMVCo-certified, GDPR and FATF-aligned group". [NC-HOME] [MC-CMP-WC] No GDPR
  certificate or assessor is shown, so that claim is the vendor's word; SOC 2 and PCI DSS are not claimed for MemberCheck. [NC-HOME]
- **Public trouble: none found, but the check is thin.** Web searches on lawsuits, breaches, outages and regulator action found nothing; the OAIC
  (Office of the Australian Information Commissioner) site search returned "0 search results for membercheck"; the vendor pages show no
  incident notice. [NOTE-CO] [OAIC] [MC-HOME] ASIC enforcement records and court judgments were not checked, so "none found" is weak evidence. [INFERENCE]

## 2. What they sell

The vendor says it sells "nine ways to screen, verify, and monitor risk" on "One platform", and that "Every product shares the same customer
record". [MC-PRODUCTS] Two of its own pages disagree: the FAQ names "four main products", and transaction monitoring "runs on its own platform, with
its own credentials, separate from the MemberCheck screening app". [MC-FAQ] [MC-TM] Everything below is what the vendor says; none of it was tested.

### 2.1 Modules

| Module | What the vendor says it does | Limits or unknowns |
|---|---|---|
| PEP and sanctions screening | Screens customers, entities and counterparties against "global sanctions, PEP, watchlist, and enforcement data, with configurable matching and ongoing monitoring"; covers individuals, entities, beneficial owners, directors and the existing customer base [MC-PEP] | The 1,000+ sources are not named; no Iraqi list; aliases not addressed (section 3) [MC-PEP] [MC-FAQ] |
| Customer identity verification | Checks name, birth date and document details "against government and official sources", with face match and liveness. Six outcomes: verified, not verified, review required, pending, incomplete, not requested. Coverage "60+ countries, expanding to 120+" [MC-ID] | No country list, data source or biometric provider named. The Iraq page lists ID checks but no Iraqi document or registry [MC-ID] [MC-IRAQ] |
| Transaction monitoring | Monitors transactions fed in "through the API" against configurable rules and scenarios; alerts; "support suspicious matter reporting" [MC-TM] | Separate platform and sign-in, linked to a FraudShield host on a Nexiant domain. [INFERENCE] Probably sold under another brand. No ready-made scenario set stated; filing not described [MC-TM] [FS-LOGIN] |
| Adverse media | Screens names against "negative news and adverse media from thousands of sources"; results scored and grouped by theme [MC-ADVERSE] | "Thousands" not counted or named; method is keyword search; media languages, including Arabic, not stated [MC-ADVERSE] [MC-FAQ] |
| Know your business (KYB) | Checks a company against registry data in "130+ countries and states" from "300+ data sources", maps ownership, finds ultimate beneficial owners (UBOs) above a customer-set threshold, screens the company and its owners [MC-KYB] | No registry or country named; OFAC 50 percent rule not mentioned; company checks are not on the Iraq page's list [MC-KYB] [MC-IRAQ] |
| Enhanced due diligence (EDD) | Deeper review of high-risk customers and PEPs (source of funds, ownership, written rationale, review schedule), triggered by the customer's own rules [MC-EDD] | Only an "approval decision" is recorded; no multi-step approval is described [MC-EDD] |
| Customer risk assessment | Scores customers with "configurable factors across geography, product, channel, and customer type"; the rating sets the level of checks and monitoring [MC-RISK] | Risk-level names and the model are not shown; it is the customer's model "rather than a fixed template" [MC-RISK] |
| Jurisdiction risk | Rates countries from FATF lists, sanctions regimes and corruption indices and applies the rating to customer and transaction risk [MC-JURIS] | Indices not named; update time not stated [MC-JURIS] |
| Partner programme | "White-label MemberCheck's risk screening platform under your own brand" [MC-COMPLIANCE] | See section 5 |

### 2.2 The path of one check, from input to decision

1. **Set up and screen.** The user sets "risk rules, matching thresholds, and customer context", then screens "in real time". A home-page picture
   (labelled "Schematic, not a screenshot") shows secondary identifiers as "Date of birth, country". [MC-HOME]
2. **Candidates, then score.** Every name that touches a list becomes a candidate "before any scoring is applied"; "Weak coincidental matches
   drop out below your threshold." [MC-PEP]
3. **Review.** Matches go to an analyst queue with "Evidence, assignee and a decision history a regulator can inspect". [MC-PEP] "Cannot
   determine" is "a first-class outcome" beside match and no match. [MC-ABOUT] [MC-API] The picture shows "Escalated for review" and "Recorded";
   the full list of decision and risk-level names is not stated. [MC-HOME] [MC-FAQ]
4. **Record and monitor.** Each match keeps its source list, score, identifiers used, assignee and closing decision; a cleared match keeps its
   evidence "so a tuning decision is defensible later". The customer is then re-screened "automatically as sanctions and PEP lists change". [MC-PEP]

### 2.3 Batch, monitoring, users, reports, languages

- **Batch.** "Screen a single customer at onboarding through the API, or run your entire book in batch. Both paths use the same lists and matching
  logic." File formats, size limits and schedules are not stated. The integration guide warns that a batch can finish only part of the book
  unnoticed and asks for "A completion check, not just a job that ran". [MC-PEP] [MC-COMPLIANCE] [MC-API]
- **Ongoing monitoring.** Daily re-screening with email alerts; the page wordings differ (section 3.4). Dashboard or API-callback alerts are not stated. [MC-FAQ] [MC-FEATURES]
- **Users and reports.** "Unlimited multi-user access, with simple and advanced permission settings and pre-defined user roles"; one account can
  manage several organisations; multi-factor authentication on every account. Single sign-on, branches for direct customers and maker-checker approval
  are not stated. A production account is set up "within 24 to 48 business hours" of signing. Reports download as PDF, Word and Excel; templates,
  scheduled reports and regulator formats are not stated. [MC-FAQ] [MC-FEATURES] [MC-COMPLIANCE]
- **Languages: the vendor's pages disagree.** The FAQ says "The software is currently available in English, French and Japanese, and its
  documentation in English and French, with Spanish to follow". [MC-FAQ] The enterprise comparison page (updated 27 August 2026; re-read by the writer
  on 11 October 2026) says "The MemberCheck interface is available in English, French, Arabic, Chinese, Japanese and Spanish". [MC-CMP-ENT] A
  directory also says yes to Arabic. [ZH-OV] Neither claim was tested (section 6.2). A Japanese name is kept in an "Original Script Name" field;
  no page says Arabic gets the same treatment. [MC-JAPAN]

### 2.4 Not offered, or not stated

- **Payment screening before release and Swift messages:** not stated. The API guide names "a payment about to be released" as a moment to call
  the API, but no page says the platform reads payment messages or holds a payment; the FAQ only says "SWIFT doesn't maintain its own sanctions lists". [MC-API] [MC-FAQ]
- **goAML filing:** not stated. The Iraq page lists "File suspicious transaction reports with the FIU" as a duty of regulated firms, not as a
  MemberCheck feature. [MC-IRAQ] [MC-TM]
- **Installed in the customer's own data centre:** not stated for direct customers; client data sits on "our cloud servers" (section 4.4). [MC-FAQ] [MC-COMPLIANCE]
- **Ready-made core-banking connector, public documentation:** none named or found; no help centre, user guide, release notes, API reference or demo
  video; demos are on request. [MC-API] [MC-FEATURES] [MC-CONTACT]

## 3. How it works, part one: the data

### 3.1 Who supplies the watchlist and PEP data

**MemberCheck does not say.** Thirteen of its pages were searched for "powered by", "sourced from", "licensed from" and for the names Dow Jones,
LexisNexis, Refinitiv, World-Check, Accuity and others; none matched, and the About page names no research team. [MC-ABOUT] [MC-PEP] [MC-FAQ]
[MC-SUBPROC] [MC-API] [NOTE-DATA] What the vendor does say points to bought-in data:
- The privacy policy says database data comes "from a commercial supplier of AML/CTF watchlists", unnamed. [MC-PRIV] Comparison pages say it
  "aggregates multiple premium risk sources and screens against more than 1,000 global watchlists, then adds its own monitoring, matching and
  workflow layer"; the FAQ calls the data "our database". [MC-CMP-CA] [MC-FAQ]
- The sub-processor page (11 entries on the lead's live read of 11 October 2026) says its entries "assist with the processing of data submitted
  to our platform". Amazon Web Services Inc and Google LLC are cloud hosts in the United States. Kyckr Ireland Ltd, Realyou Limited (New Zealand),
  Acuris Risk Intelligence Limited (England), the Australian Securities & Investments Commission and Bing Search Engine (Microsoft) are listed
  under "Data analysis". Hotjar Ltd (Malta) is listed for website analytics; Suite CRM (Australia), ActiveCampaign LLC and Calendly LLC (both United
  States) are listed for sales and marketing. The page does not say what any "Data analysis" entry supplies, and a
  sub-processor handles customer data for the vendor without necessarily supplying lists. [MC-SUBPROC]

| Candidate supplier | Evidence | Weight |
|---|---|---|
| Acuris Risk Intelligence | On MemberCheck's sub-processor list. Sister brand NameScan says its Sapphire tier is "powered by the Acuris Risk Intelligence database" [MC-SUBPROC] [NS-FAQ] | The only current vendor-published link. That Acuris feeds MemberCheck is [INFERENCE]; no MemberCheck page says it |
| Dow Jones | A press item of 6 November 2012: watchlists "compiled by Dow Jones featuring more than 600,000 names". A search summary also said NameScan uses Dow Jones, but NameScan's page names no provider [GR-PRESS] [NS-HOME] | Weak: secondary, from 2012, current pages silent |
| LexisNexis WorldCompliance | A directory says the datasets include "LexisNexis WorldCompliance" and a "proprietary Emerald dataset"; Emerald is NameScan's name, so the listing may mix up the brands [ZH-MC] | Weakest: secondary, undated, unverified |
| NameScan's own database | NameScan says Emerald is its "Proprietary Database" built from lists "collected from governments and other official lists that are publicly available on the internet" [NS-ABOUT] | Describes NameScan only; no MemberCheck page mentions Emerald |

The sub-processor list is the strongest of these, because it is the vendor's own current page, yet it proves only that Acuris handles data on the
platform. Who supplies MemberCheck's lists is **not known**.

### 3.2 Sanctions lists

- **Named:** "Global sanctions lists (OFAC, UN, EU, HMT)", "alongside national lists and watchlists" (none named); "over 1,000 sanctions, PEP, and
  watchlist sources, refreshed on a 24-hour cycle, covering 195 countries". The FAQ counts "more than 1,000 sanctions lists, PEP lists, and
  wanted/watched lists from 195+ countries". Sources and lists are different units, and neither is itemised. [MC-PEP] [MC-FAQ]
- **Inferences.** For Singapore's MAS lists the FAQ says they are "covered in MemberCheck, but under UN sanctions rather than MAS sanctions", so a
  national list with no UN twin may not be loaded separately. The FAQ says "SWIFT doesn't maintain its own sanctions lists" and points to OFAC, so
  Swift bank listings are probably covered only through OFAC. Four regional services exist and the first covers "Australia and regions not listed
  below"; Iraq is named in none, so it would sit there. [INFERENCE] [MC-FAQ] [MC-PEP] [MC-HOME]
- **Not found:** Australian (DFAT) sanctions by name, the OFAC 50 percent ownership rule, enforcement or law-enforcement lists by name. [MC-PEP] [MC-FAQ] [MC-KYB]

### 3.3 PEPs, adverse media, names and aliases

- **PEPs.** The vendor uses the FATF Recommendation 12 groups (foreign, domestic, international organisation) and screens relatives and close
  associates "as part of the same check"; "The database goes back more than 20 years." No profile count, tiers, state-owned-enterprise treatment or
  rule for former PEPs is given. [MC-PEP] [MC-FAQ] (NameScan's Acuris tier quotes 1.4M+ PEP profiles; that is its figure. [NS-FAQ])
- **Adverse media.** "Negative news and adverse media from thousands of sources", found by "keyword search" on "reputable news and government sites"
  that are not named; results are grouped by theme and the source article, link and date are stored. No media language is stated. [MC-ADVERSE]
  [MC-FAQ]
- **Names and aliases.** "Fuzzy matching for transliteration, phonetic variants and non-Latin scripts". [MC-PEP] Aliases are not addressed on any page read.
  One sentence says names are matched in their own script: "the screening engine matches names in original scripts such as Arabic, Chinese, Cyrillic
  and Japanese" (enterprise comparison page). No page says whether the lists hold Arabic-script names. [MC-PEP] [MC-CMP-ENT]

### 3.4 Freshness: one claim, many wordings

| Where | Wording |
|---|---|
| About page | Data "is refreshed daily to keep pace with regulatory change" [MC-ABOUT] |
| PEP page | "refreshed on a 24-hour cycle"; re-screen "the moment a new listing affects them" [MC-PEP] |
| FAQ | Sanctions changes "In most cases within 24 hours of the information becoming public"; adverse news "generally added overnight"; PEP changes "usually made within a few days"; customers hear "through automatic daily screening" [MC-FAQ] |
| Features page | "Real-time detection of changes to your customers' status through continuous activity monitoring" [MC-FEATURES] |

At least five wordings, from "daily" to "real-time"; no page gives a measured or audited lag from an official listing to the data. [MC-FAQ] [MC-PEP]
[INFERENCE] Plan on a daily cycle, treat "real-time" as marketing, and expect PEP changes to arrive slower than sanctions. The About page promises that
an unverified figure "is either marked or removed", yet the home-page and product-page figures (60M scans, 13,000+ customers, 75 percent fewer false
positives, 1,000+ lists, a 24-hour cycle) carry no mark. [MC-ABOUT] [MC-HOME] [MC-PEP]

### 3.5 Private lists, identity and company data

- **Private lists.** "Custom watchlists and blacklists", to "Build targeted lists to support your own AML/CTF compliance approach"; no upload is
  described. The FAQ describes whitelists only (names cleared as false matches). [MC-FEATURES] [MC-FAQ]
- **Identity and company checks.** Each identity request "is routed automatically to the most appropriate source"; none is named, and only Australian
  and New Zealand clients can choose a source. Company checks use "300+ data sources", none named. [MC-ID] [MC-KYB] [INFERENCE] Kyckr, which describes
  itself as a company-registry connector, is probably the registry source. [KYCKR] [MC-SUBPROC]

### 3.6 Is any Iraqi list covered?

**Not stated, and not shown.** The Iraq page (updated 17 August 2026; re-read by the writer on 11 October 2026) lists five items under "Screening coverage for
Iraq": sanctions and PEP screening, adverse media, identity verification, jurisdiction risk and ongoing monitoring. It names the Central Bank of Iraq
as supervisor and the Office of Combating Money Laundering and Terrorist Financing as the financial intelligence unit, and names no Iraqi list, no
freezing list and no United Nations list; its screening is "against global sanctions, PEP, and adverse-media data". [MC-IRAQ] The words "Iraqi" and
"freez" appear on none of the vendor pages searched. [MC-COUNTRY] [NOTE-DATA] One working note counted four items and left out sanctions and PEP; the live
page has five. NameScan names "OFAC, UN, EU, UK, Australian and other major sanctions regimes". [NS-HOME] NameScan's 2024 Emerald coverage document, hosted
on docs.membercheck.com, lists "Iraq" in its country table and shows no separate Iraqi list (working note 12, lines 65 and 137). A coverage table is not a
customer list. [NOTE-P12]

### 3.7 NameScan's two data tiers (the sister brand, not MemberCheck)

- **Sapphire:** "powered by the Acuris Risk Intelligence database" with "1.4M+ PEP profiles", "1.2M+ sanctions profiles" and "400,000+ adverse
  media links", updated daily. **Emerald:** "our proprietary global database of PEP, sanctions, and law enforcement watchlists". Users can choose which
  database to screen. [NS-FAQ] [NS-ABOUT]
- The FAQ's two headings appear swapped, and NameScan's home page and FAQ disagree on which tier is the premium one; we match each description to the tier it names. [NS-FAQ] [NS-HOME]

## 4. How it works, part two: matching and technology

### 4.1 Matching methods, thresholds and secondary identifiers

- **Methods.** The only method named is "fuzzy matching and phonetic algorithms": "Our intelligent filters use fuzzy matching and phonetic algorithms
  to filter out noise in the database and cut down false positives." [MC-FAQ] The PEP page adds "Thresholds, fuzzy logic and secondary identifiers, set
  to your risk appetite" and "Set match thresholds per customer segment, not one setting for the whole book". [MC-PEP]
- **Not published:** match types (exact, similarity percentage, initials, name order), the phonetic method, any default, the score scale and a
  recommended threshold; "Match score" appears only as a label in a home-page picture. [MC-FEATURES] [MC-ABOUT] [MC-HOME] The user sets
  "matching thresholds"; the integration guide says "Matching thresholds are a control" and asks who may change one. [MC-HOME] [MC-API]
  "75%" "Fewer false positives" on the home page has no method, base or test. [MC-HOME]
- **Secondary identifiers.** The home page lists input fields "Full Name", "Date of Birth", "Nationality", "Government ID", "Address", "Business No."
  and "+10 more fields", and its picture shows "Date of birth, country". The PEP page says they "narrow a name-only hit"; the integration guide tells banks
  "Name similarity without secondary identifiers is the single largest source of false positives". [MC-HOME] [MC-PEP] [MC-API]
- **Not stated:** how a year-only birth date, nationality, gender, ID number or address changes a result. A mother's name, father's and grandfather's
  names, tribe and chain position are not among the listed fields (ten more are unnamed). [MC-FEATURES] [MC-HOME] NameScan's sample request sends a
  year-only birth date ("dob": "1974"); that is NameScan's rule, not MemberCheck's. [NS-HOME]

### 4.2 False-positive handling

- **Whitelists.** "You can create whitelists from due diligence decisions marked as a 'false match' during the screening process, then apply them to
  future scans." Not stated: who approves an entry, whether it expires, whether it is re-checked when the list record changes, whether changes are
  logged. [MC-FAQ] Cleared matches keep their evidence. [MC-PEP]
- **Advice to banks:** "A match is not a decision" and "Never auto-reject on a name match alone". The outcomes are match, no match and "cannot
  determine". [MC-API] [MC-ABOUT]

### 4.3 API and integration

- **The reference is withheld.** "The endpoint-level reference is not published here — base URLs, authentication, request and response shapes,
  error codes and rate limits"; the vendor "will get you the current reference along with a sandbox" on request. Callbacks or webhooks, SDKs, rate
  limits, version numbers and an endpoint list are not on any page. [MC-API] The hosts api.membercheck.com/docs and demo.api.membercheck.com/docs
  answered HTTP 401 and asked for an "api-key" header; docs.membercheck.com and membercheck.com/api answered 403. No key was requested.
  [API-DOCS] [API-DEMO] [MC-DOCS] [MC-APIPATH]
- **Two modes.** A call "at the moment" for "One subject" (for "Onboarding, a payment about to be released, any decision a person is waiting on")
  and "Batch" for "The whole portfolio". The PEP page claims "a sub-second response" for a single call. [MC-API] [MC-PEP]
- **No ready-made connectors.** CRM, core bank, webhook, callback and SDK appear on no page. The integration sentences are generic: "Connect MemberCheck
  to your existing systems for seamless, real-time screening" (features page) and "Embed screening directly into onboarding and core systems through the
  MemberCheck API, so checks run inside your product" (PEP page). [INFERENCE] A bank's core system would call the API from its own side. [MC-API] [MC-FEATURES] [MC-PEP]
- **Time to integrate: two answers.** A stat block shows "<24h" under the label "Average integration time"; the FAQ says "the average implementation takes between
  three and four weeks". [MC-PEP] [MC-FAQ] [INFERENCE] The longer figure is the one to trust.
- **NameScan is more open.** Its Developer Centre says "The NameScan API is organised around REST", with a test key and a production key, but lists no
  endpoints, fields or rate limits; its reference answered 401. [NS-API] [NS-APIDOC]

### 4.4 Deployment and hosting

- **Cloud, in the vendor's words.** "Clients retain full ownership of the data stored on our cloud servers"; "MemberCheck databases are held and stored by
  MemberCheck or a cloud services provider". Amazon Web Services Inc and Google LLC are listed as United States cloud services; which one holds the
  screening data, and in which region, is not stated. [MC-FAQ] [MC-PRIV] [MC-SUBPROC]
- **Where the data sits is left to the buyer:** "Where is the data processed and stored? A jurisdictional question with contractual consequences;
  settle it before integration rather than during procurement." Accounts belong to one of four sign-in regions ("Oceania & Rest of World", "Europe",
  "Southeast Asia", "Middle East (GCC)"); the data location of each is not stated. [MC-API] [MC-HOME]
- **No installed or offline option is named** for direct customers; [INFERENCE] the offer looks like cloud software only. For partners, "Local
  deployment for data sovereignty" and a "Licensing" model appear with no detail (section 5).
  [MC-FAQ] [MC-API] [MC-COMPLIANCE] [MC-PARTNER]

### 4.5 Security and operations (all vendor claims)

- **Claimed:** "MemberCheck holds" ISO 27001 certification; GDPR compliant; multi-factor authentication on every account (method not given);
  pre-defined user roles; deletion of client data "when required"; "secure servers" with "firewall security systems"; "encrypted PII" (partner
  page); "a decision records who made it". [MC-FAQ] [MC-FEATURES] [MC-PRIV] [MC-COMPLIANCE] [MC-ABOUT]
- **Not published:** certificate body, number and scope; encryption method; single sign-on; IP allow-listing; an uptime commitment or service-level agreement (SLA); a status
  page; backup and disaster recovery; breach notification; penetration testing; a retention period in numbers. [MC-FAQ] [MC-PRIV] [MC-FEATURES]
  "99.98%" under "System uptime" and "<1s" under "Average screening response" appear in stat blocks with no method or period. [MC-PEP] [MC-IRAQ]
- **Retention.** Personal information is kept "only for the duration of the client's subscription and for a period of time afterwards where we have an
  ongoing business need", then deleted or anonymised; no period is given. [MC-PRIV]

## 5. Partner models: reseller, white label, licensing

**No Iraqi partner is named on either vendor's pages.** MemberCheck's partnership page names no partner of any kind. English searches found no reseller, distributor, integrator or
person for MemberCheck in Iraq, Jordan, the UAE, Oman, Lebanon or Saudi Arabia, and an Arabic search found no Iraqi user. [MC-PARTNER] [NOTE-IRAQ] The vendor lists "Ten
partnership models, from a referral through to a fully white-labelled platform", and says "there is no application form and no
partner tier to qualify for". [MC-PARTNER] The right-hand column is our reading, not a finding.

| Model | What the vendor says | If an Iraqi company used it [INFERENCE] |
|---|---|---|
| Referral, affiliate | "Refer clients, resell the platform, or white-label screening under your own brand through our partner programme." [MC-HOME] | The Iraqi firm introduces the bank, and the bank signs with MemberCheck and uses its cloud product directly |
| Reseller | "A reseller sells MemberCheck; the customer knows they are buying MemberCheck, and the platform says so." Also "Resell MemberCheck's AML/CTF risk management solution globally." [MC-PARTNER] | The bank sees the MemberCheck name. The local firm could add support, training and Arabic documentation on top |
| White label | "A white-label partner sells their own service, and the customer may never encounter our name." The package includes "Rebranding, custom domains, and a multilingual interface", separate client organisations with their own users and records, and "Local deployment for data sovereignty" [MC-PARTNER] [MC-COMPLIANCE] | The bank sees a local product name and web address and may never learn that MemberCheck is the engine. This is the only model under which a "local" product could run on MemberCheck unseen |
| Technology partner, licensing | Technology partners: "we run the platform, you call it". "Licensing is for cases where that arrangement will not do, usually because of a data-residency or procurement constraint on your side." [MC-PARTNER] | A licence might let a local firm run the software itself, but the page does not say so. Whether hosting in Iraq is allowed is unknown |
| Distribution, strategic alliance, joint venture, co-marketing | Named only [MC-PARTNER] | Unknown |

- **Deployments.** "MemberCheck is deployed in Australia, Germany, Oman and Indonesia, with expansion planned into the Americas and South
  Africa." Iraq is not named, and no Omani client or partner is named either. [MC-PARTNER]
- **The NameScan route.** NameScan's partnership page offers three routes: referrals, distributors and application integrators ("Integrate with our API to
  streamline your clients onboarding process"). It names eight partners in Sweden, the UK, Georgia, South Africa, Hong Kong, Estonia, Latin America and
  France, none in Iraq or the Middle East, and does not mention white label or Prototype. [NS-PARTNERSHIP] [INFERENCE] A local firm could reach NameScan as a
  distributor or integrator, or use the API directly if its terms allow; those terms were not checked.
- **Pointer.** The Iraqi company that lists NameScan as a screening partner is covered in file 34. This file does not assess that company.

## 6. Arabic, Iraq and the Middle East

**No MemberCheck or NameScan page names an Iraqi customer, partner or seller, or uses the word "Prototype". One GetApp review by a "Mohammed" says his
firm distributes and develops MemberCheck products (no country given). The one tie found to an Iraqi company runs from Prototype to NameScan (file 34;
Prototype's own claim).** [GA-ARCH]
- No Iraqi bank, exchange company, payment company, reseller or person is named as a user or seller of MemberCheck or NameScan, in English or
  Arabic searches (working notes 06, 07 and 12), on the vendor's pages, or in a web search limited to the Iraq Stock Exchange website, which returned no links.
  [MC-IRAQ] [MC-PARTNER] [MC-CASES] [NOTE-IRAQ] [NOTE-NAMES] [NOTE-P12]
- "Prototype", "Mohammed" (any spelling), "Erbil", "Baghdad" and "Iraqi" appear on none of the 11 vendor pages checked for them; the "Mohammed" review above is on a third-party site (section 1.3). The one "prototype"
  hit is JavaScript code on NameScan's pricing page, not a name. [NOTE-IRAQ] [NS-PRICING]
- The owner's words match no MemberCheck or NameScan page read. [INFERENCE] "MemberCheck" is the likeliest sound-alike (file 34 covers the Iraqi company).

### 6.1 What the vendor says about the Middle East and Iraq

- **Regions and presence.** The sign-in region "Middle East (GCC)" is described as "Gulf Cooperation Council states". Iraq is named in none of the four regions,
  so by the page's own wording it would fall in "Australia and regions not listed below". [INFERENCE] [MC-HOME] [MC-PEP] The vendor says it is deployed in Oman
  (no client named); the one Middle East event it names is "Transform Finance MENA" in Dubai, dated January 2023 on the events page (one note read December 2022; we use the note that quotes the page);
  it says it has an office in Ankara, Turkey (vendor claim; Turkey is not in the vendor's own Middle East country group). [MC-PARTNER] [MC-EVENTS] [MC-CAREERS] [MC-COUNTRY]
- **The Iraq page** ("AML/CTF Compliance in Iraq", updated 17 August 2026, re-read by the writer on 11 October 2026) names the Central Bank of Iraq and the
  financial intelligence unit (FIU), lists five covered capabilities (section 3.6) and shows "1,000+" watchlists, "24h" refresh and "<1s" response. It names no
  Iraqi list, customer, partner, local office or Arabic. Its legal citations ("Law No. 39 of 2015", "Instructions No. 1 of 2023") are the vendor's own and
  were not checked against Iraqi sources. [MC-IRAQ]
- **A gap the vendor does not explain.** The Iraq page says "Screening coverage for Iraq", yet the "deployed" list has no Iraq; Iraq in the contact form's drop-down is
  not a customer. [MC-IRAQ] [MC-PARTNER] [MC-CONTACT] The country guide counts 205 guides against "195 Countries screened", also unexplained. [MC-COUNTRY]

### 6.2 Arabic

- **Interface.** The FAQ lists English, French and Japanese; the enterprise comparison page lists English, French, Arabic, Chinese, Japanese and Spanish.
  [MC-FAQ] [MC-CMP-ENT] They cannot both be right. The FAQ (last updated 5 August 2026) is the more specific; the comparison page (last updated 27 August 2026) is the later of the two. We cannot tell which holds. [INFERENCE]
- **Matching.** The word "Arabic" is on the PEP page once, in an answer about transliteration: "Names transliterated from Arabic, Cyrillic, Chinese or Japanese
  scripts rarely match character for character"; "Fuzzy matching handles spelling variants, phonetic similarity and alternative transliterations". [MC-PEP] The
  enterprise page goes further: "the screening engine matches names in original scripts such as Arabic, Chinese, Cyrillic and Japanese". [MC-CMP-ENT] No page
  read describes Arabic name parts (Al, Bin, Abu, Abd), letter variants or the Iraqi name chain, and the listed input fields include no mother's name or tribe.
  [MC-FAQ] [MC-IRAQ] [MC-HOME]
- **NameScan** has one line of Arabic marketing copy on its home page; it is site copy, not a matching feature. [NS-HOME]

## 7. Pricing, customers and reviews

### 7.1 Pricing

- **MemberCheck publishes no price.** Its pricing page is a demo request form ("Request a demo for your compliance team") that asks for "Anticipated
  Annual Scan Volume" in three bands: "Less than 10,000 Scans", "10,000 to 50,000 Scans" and "More than 50,000 Scans". A comparison table says "Based
  on screening volume and the features you use. Request a quote" and "Unlimited users, with no per-seat fee". A directory also shows "No pricing info".
  [MC-PRICING] [MC-CMP-WC] [GA-ARCH]
- **NameScan is pay as you go.** "Pay As You Go, per search"; "no annual contracts, no minimum monthly commitment and no setup fees"; its FAQ says "Pay only for what you use, no contracts, no minimums, no hidden
  fees" and "Start with 10 free scans"; packages "Valid 12
  months from purchase"; 12 currencies, among them the Emirati dirham and the US dollar, not the Iraqi dinar. [NS-PRICING] [NS-FAQ] The home page says "From
  $2.00 USD per scan" and "$1.40 per scan at 1,000"; the pricing page loads its amounts in the browser, so they could not be read there. [NS-HOME] [NS-PRICING]

### 7.2 Customers

- **How many?** The home page says "13,000+" customers worldwide; the case-studies page says "16" "organisations screening in production" and "1"
  "written case study published — more in progress". No page reconciles the two. [MC-HOME] [MC-CASES]
- **Who?** The 16 logos, "live customers, not pilots": AMP, BancABC, BMG, BTC, CMC Markets, Coop, CSC, EFICA, EML, Groupama, MEO, Riskified, Securiport,
  Tanzania Commercial Bank, Tradeview Markets and Zappit, with no sector or country given. [INFERENCE] Two names contain the word bank; none is confirmed as a
  bank. The vendor describes its clients as "Banks, payment processors, brokers, insurers and telecommunications operators across Australia, Africa, Europe
  and the Caribbean". [MC-CASES]
- **Case studies.** Titan Corporate Services, a trust and company service provider in Mauritius with "roughly 500 clients"; its figures ("95%" approval
  rate, "4 min" median processing time) are unlabelled and the page does not say who measured them. [MC-TITAN] The group's other cases are NameScan's (Aegis
  Reins, UPayCard) and one for GPayments (card authentication). None is a bank, and the banking page ("purpose-built for banks") names none.
  [NX-CASES] [NX-AEGIS] [NX-UPAY] [NX-EGT] [NX-BANKING] The Nexiant site says "1,900+" watchlist sources against "1,000+" on membercheck.com; no page explains. [NX-ABOUT] [MC-HOME]

### 7.3 Reviews and rankings

- GetApp (archive copy of 12 June 2026): 4.8 out of 5 from 15 verified reviews (12 five-star, 3 four-star); the reviewers' top listed industries are
  Information Technology and Services, then Financial Services (no counts shown). [GA-ARCH] (secondary) The copy shows six review texts (21 August 2023 to
  4 November 2024, syndicated from Capterra). Praise: ease of use, support and turn-around of 24 hours. Complaints: "Screening hits are limited", batch checks
  need an Excel file filled in correctly, no search-count control at sub-organisation level (the review described in section 1.3), single checks not priced
  apart from daily monitoring, and "Ltd service capacity in Africa". [GA-ARCH] The Capterra, G2, Software Advice, SaaSworthy and TrustRadius pages themselves
  returned HTTP 403. [NOTE-MKT] A search summary said a November 2022 Capterra review lists no connection to locally hosted, offline servers as a drawback;
  that review is not among the six texts and its page could not be read, so this is **not used as a finding**. [CAPTERRA-MC]
- No analyst ranking or award was found. The vendor's comparison pages (World-Check, ComplyAdvantage, LexisNexis, First AML) use the rivals' own published
  figures, and one tells readers when to choose the rival: "Choose World-Check if your firm is contractually required to hold the World-Check dataset itself,
  or you are a very large bank, government body or intelligence agency that has standardised on it." [NOTE-MKT]
  [MC-CMP] [MC-CMP-WC]
- For Iraq, the rivals on record are other vendors: LSEG's World-Check with the local firm Iraqna IBTS (10 March 2020) and Capital Banking Solutions at UR
  Islamic Bank (25 November 2021, "over 22 installations in Iraq"). [LSEG-IQ] [FINTECH]

## 8. What it does right

Credit is given for what the vendor's own pages show; a vendor claim is marked as such.

1. **It treats doubt as a result.** "Cannot determine" is "a first-class outcome", and the integration guide tells banks "A match is not a decision" and
   "Never auto-reject on a name match alone". [MC-ABOUT] [MC-API]
2. **It keeps the evidence behind each decision** (the vendor says): source list, score, identifiers used, assignee and closing decision, and the evidence
   of each cleared match "so a tuning decision is defensible later". [MC-PEP]
3. **The customer sets the knobs.** Thresholds can be set "per customer segment", and the risk rating follows the customer's own model "rather than a fixed
   template". [MC-PEP] [MC-RISK]
4. **It is open about integration pitfalls.** The guide asks for "A completion check, not just a job that ran" and raises the data-location question
   ("settle it before integration rather than during procurement"). [MC-API]
5. **One suite, a no-per-seat licence, and quick account set-up** (vendor claims): screening, identity, company checks, EDD, risk rating and monitoring in one
   family (monitoring is the exception, section 2); "Unlimited users, with no per-seat fee"; an account set up "within 24 to 48 business hours" of signing,
   though the FAQ gives three to four weeks for integration; NameScan offers "10 free scans". [MC-PRODUCTS] [MC-CMP-WC] [MC-FAQ] [NS-FAQ]
6. **It names its rivals and says when to choose one** (its own comparison pages, not checked against the rivals' own pages): it says when a rival is the
   better choice, namely a firm contractually required to hold the World-Check dataset, or a very large bank that has standardised on it. [MC-CMP-WC]
7. **It has an Iraq page** naming the Central Bank of Iraq and the FIU (vendor content, unchecked), and it acknowledges data-residency limits through "Local
   deployment for data sovereignty" and a licensing model for partners, without detail. [MC-IRAQ] [MC-COMPLIANCE] [MC-PARTNER]
8. **Early user ratings are good but thin:** 4.8 out of 5 from 15 verified GetApp reviews. [GA-ARCH] (secondary)

## 9. What it does wrong or leaves unproven

Each point gives the evidence. "Unproven" means the vendor says it, or is silent, and no outside check exists.

1. **The data supplier is unnamed and the hints conflict:** Acuris on the sub-processor list, Dow Jones in 2012 press, LexisNexis WorldCompliance in a directory.
   A bank cannot assess list quality or the supplier's terms. [MC-SUBPROC] [GR-PRESS] [ZH-MC] (Section 3.1.)
2. **No Iraqi list is named anywhere.** The Iraq page claims "Screening coverage for Iraq" yet names no Iraqi or freezing list, and Iraq is not in the "deployed" list.
   [MC-IRAQ] [MC-PARTNER]
3. **Freshness is claimed five ways and never measured:** "real-time", "overnight", "within 24 hours", "daily", "a few days" for PEP changes; the headline
   figure is a 24-hour cycle. [MC-FEATURES] [MC-FAQ] [MC-PEP]
4. **Arabic is contradicted by its own pages:** the FAQ lists no Arabic; the enterprise page lists Arabic and "original scripts"; nothing describes Arabic name
   parts, letter variants or the Iraqi name chain. [MC-FAQ] [MC-CMP-ENT]
5. **Matching is a black box:** no match types, defaults, score scale or recommended threshold, so the public pages give a bank nothing to validate or
   document the logic against (a sandbox and the reference are offered on request, not tested). [MC-FAQ] [MC-FEATURES] [MC-ABOUT] [MC-API]
6. **Headline figures carry no method, against its own policy:** "75% Fewer false positives", "60M Scans performed", "99.98%" uptime, "<1s" response and "<24h"
   integration are unmarked although unverified figures are supposed to be "either marked or removed"; the FAQ gives "three and four weeks". [MC-HOME] [MC-PEP]
   [MC-ABOUT] [MC-FAQ]
7. **Customer evidence is thin and inconsistent:** "13,000+" customers against "16" in production and one written case study; two logos carry bank names (Tanzania Commercial Bank, BancABC) but the
   page gives no sector for any logo and no case study is a bank. [MC-HOME] [MC-CASES]
8. **Cloud only, as far as stated:** client data is "on our cloud servers", United States hosts are listed, the data region is left to the buyer, no installed
   option is named for direct customers, and watchlist personal data "may be disclosed to overseas recipients". [MC-FAQ] [MC-SUBPROC] [MC-API] [MC-PRIV]
9. **Transaction monitoring is a separate platform** with its own credentials, reached through a sign-up link on a Nexiant host, despite "One platform"; payment screening before release and goAML
   filing are not stated. [MC-TM] [FS-LOGIN] [MC-HOME] [MC-API]
10. **Documentation and assurance are withheld:** no public API reference, help centre, release notes, SLA or status page; no certificate details; "certified" (footer badge alt text; "holds this certification" in the FAQ)
    against "aligned" in a comparison table. [MC-API] [MC-HOME] [MC-CMP-WC]
11. **Identity is muddy:** Neurocom versus Nexiant as parent, 2007 versus 2008, no named executive, and a sub-processor page dated both "Last updated 14 September
    2026" and "Modified on: 25.07.2023" with a column headed "Data Controller" above processors. [MC-ABOUT] [MC-SUBPROC] [ABR-MC]
12. **Few independent checks:** 15 verified reviews on one directory, the larger review sites blocked to our tools, no analyst ranking, no public incident or audit
    record either way. [GA-ARCH] [NOTE-MKT] [OAIC]

## 10. Fit against eight of the twelve needs of an Iraqi bank

The needs come from the twelve-row yardstick table in file 14, section 1 (which draws on the Central Bank of Iraq's AML/CFT controls of 8 January 2025 and
file 01). Eight rows are rated here. Rows 1, 2 and 5-8 are file 14's needs with the same numbers; rows 3 and 4 split file 14's need 3 (search in Arabic and
English; the Iraqi name shape). File 14's needs 4 (screen all parties, identify beneficial owners), 9 (ownership-aware screening), 10 (documented
configuration and model validation) and 11 (tooling verified by a CBI-approved firm) are not rated here. File 14's need 12 (on-premises, read-only) is below
the table. [F14] The verdicts judge the vendor's public pages only; nothing was tested. "Does not meet" means the vendor's own stated figure or position fails the need.

| # | Need | MemberCheck and NameScan | Evidence | Verdict |
|---|---|---|---|---|
| 1 | Lists named by the Central Bank: UN, OFAC, EU, UK and the Iraqi national list | MemberCheck claims OFAC, UN, EU and HMT "alongside national lists and watchlists"; its Iraq page names no Iraqi list. NameScan names OFAC, UN, EU, UK and Australian regimes | [MC-PEP] [MC-IRAQ] [NS-HOME] | Partly (international lists by claim; Iraqi list not shown) |
| 2 | Lists refreshed at least every 12 hours | Headline "24-hour cycle"; "within 24 hours"; "daily"; one page says "real-time"; no measured lag. NameScan: "updated daily" | [MC-PEP] [MC-FAQ] [MC-FEATURES] [NS-HOME] | Does not meet (24 hours as stated; "real-time" unproven) |
| 3 | Search in Arabic and English | The vendor's pages disagree on an Arabic interface; one sentence claims matching "in original scripts such as Arabic"; the PEP page describes transliteration only | [MC-FAQ] [MC-CMP-ENT] [MC-PEP] | Unknown |
| 4 | The Iraqi name shape: given name, father, grandfather, tribe, mother's name, birth year | No page covers Arabic name parts or the name chain. Listed input fields include birth date and nationality, not mother's name or tribe; ten more fields are unnamed | [MC-HOME] [MC-FAQ] | Unknown |
| 5 | World-Check, the system the Central Bank names | MemberCheck is a rival to World-Check, not a feed of it; no page names World-Check as its data source; it tells firms contractually required to hold the World-Check dataset, and very large banks that have standardised on it, to choose World-Check | [MC-CMP-WC] [MC-PEP] | Does not meet as written |
| 6 | Direct link between the core banking system and the screening platform | An API exists; no core-banking connector is named; the reference is withheld; integration is "<24h" on one page and "three and four weeks" in the FAQ | [MC-API] [MC-FEATURES] [MC-FAQ] [MC-PEP] | Partly |
| 7 | Hold a transaction until the compliance officer (MLRO) has verified it | Transaction monitoring alerts and escalates for review; no page says it can hold or release a payment | [MC-TM] [MC-API] | Unknown |
| 8 | Screen payments before release (US dollar or cross-border) | The API guide names "a payment about to be released" as a moment to call, and claims "a sub-second response" (no figure for the slowest 1 percent of calls, the "p99"); payment-message (Swift) screening is not stated; the data region of the call is not stated (United States cloud hosts are listed) | [MC-API] [MC-PEP] [MC-FAQ] [MC-SUBPROC] | Partly |

Count: meets 0; partly 3 (rows 1, 6, 8); does not meet 2 (rows 2, 5); unknown 3 (rows 3, 4, 7). Not among the eight rows but decisive for AKR-TMS: file 14's
need 12 asks for on-premises operation with read-only access to the bank's database, and MemberCheck is cloud only as far as its pages say
(section 4.4). [F14] [MC-FAQ]

## 11. What it means for AKR-TMS

AKR-TMS is designed to read each bank's core banking database through a read-only connection and to run on the bank's own servers (file 30 concluded that
Central Bank of Iraq data-classification rules block a vendor-hosted multi-tenant model in the first release). It is planned to do transaction monitoring,
customer risk rating, name screening with Arabic and Iraqi name-chain matching and goAML reporting (the build is at phase 00 of 32 on 2026-10-11,
docs/plan/status.md), with an optional screening API that a payment gateway can call before releasing a payment. [F30] [F14]

**As a rival, if resold or white-labelled locally**
- No local seller is confirmed by either vendor (section 5; file 34 covers the one Iraqi firm that names Namescan as a partner). If one existed, the offer
  [INFERENCE] would be quick to set up (an account "within 24 to 48 business hours" of signing, though the FAQ gives three to four weeks for integration),
  bundled (screening, identity, company checks, monitoring), cheap to try (NameScan: "From $2.00 USD per scan", 10 free scans) and possibly wrapped in
  Arabic. [MC-FAQ] [NS-HOME] [NS-FAQ]
- **Where AKR-TMS is designed to have the edge, set against what MemberCheck's pages show:** it is designed to run on the bank's servers, while MemberCheck is
  cloud only as stated, on hosts whose region is not stated (AWS and Google are both listed as United States companies); it is planned to commit to the Central
  Bank's 12-hour refresh against a stated 24, to load the Iraqi national list, which MemberCheck never names, and to match the Iraqi name chain (mother's name,
  birth year), which MemberCheck's pages never describe; and it is planned to add a read-only link to the bank's database, goAML filing and a pre-release
  check. [MC-FAQ] [MC-SUBPROC] [MC-PEP] [MC-IRAQ] [MC-HOME]
- **Where it should not assume it wins:** price and speed of start, the breadth of bought-in list data (1,000+ sources claimed), and ID and company checks that
  AKR-TMS is not planned to offer. A hosted tool may simply tick the bank's "screening" box. [INFERENCE] [MC-PEP] [MC-ID] [MC-KYB]
- [INFERENCE] If a local product ran on MemberCheck, it would inherit the same gaps (hosting, a 24-hour cycle, no Iraqi list shown) unless the local seller added
  fixes of its own.

**As a possible data source**
- MemberCheck is not offered as data: its supplier is unnamed, its API reference is withheld and it is cloud only. [MC-PRIV] [MC-API] [MC-FAQ]
- NameScan has a published REST API, a test key, pay-as-you-go use and a tier the vendor says is powered by Acuris ("1.4M+ PEP profiles", "1.2M+ sanctions
  profiles", "400,000+ adverse media links", vendor figures). [NS-API] [NS-FAQ] [INFERENCE] It could serve as an optional online PEP and adverse-media lookup that
  a bank switches on, if the bank's data rules allow names to leave Iraq. Redistribution and embedding terms are unknown, it names no Iraqi list, and it is not
  the World-Check system the Central Bank names (file 15). The UN, OFAC, EU and UK lists need no vendor; file 14 covers open options. [F14]

**Lessons**
- **Copy:** "cannot determine" as a result; a per-match evidence record; per-segment thresholds; plain guidance to reviewers ("A match is not a decision"); a
  completion check on batch runs; a public Iraq page. [MC-ABOUT] [MC-PEP] [MC-API] [MC-IRAQ]
- **Avoid:** unmarked headline numbers; one fact worded five ways (freshness); an unnamed data supplier; a withheld API reference; "one platform" when monitoring
  has its own credentials elsewhere; contradictory language claims. [MC-HOME] [MC-TM] [MC-FAQ] [MC-CMP-ENT]
- **Beat:** publish the matching rules, defaults and test results; state the refresh time in hours and show the list version on every decision; load the Iraqi
  national list and show its date; match the Iraqi name chain in Arabic with mother's name and birth year; run on the bank's own servers; offer a pre-release
  payment check.

## 12. Open questions and things not found

**The company.** Who owns MEMBER CHECK PTY LIMITED, NEUROCOM PTY LTD and NEXIANT AI PTY LTD, who their directors are, and the incorporation date (the ASIC register was
not reachable). Whether Nexiant owns MemberCheck, replaced the Neurocom name or is a group brand. Who founded MemberCheck itself and who runs it now. What the
RISKGUARD business name is for. The ISO 27001 certifier, certificate number, scope and date. ASIC enforcement records and court judgments (not checked).
[ASIC-MC] [NX-ABOUT] [ABR-MC] [MC-HOME] [OAIC]

**The data.** Who supplies the lists, and whether it is Acuris. The list catalogue, aliases, Arabic-script names, PEP profile count and tiers, and adverse-media
sources and languages. Whether the Iraqi national list (the freezing committee's decisions) is loaded, and how fast it refreshes. Whether MemberCheck and NameScan share a
database (a shared PO Box does not show it). The measured delay from an official listing to the data. [MC-SUBPROC] [MC-IRAQ] [MC-PRIV] [NS-HOME] [MC-FAQ]

**The technology.** Match types, defaults, score scale and threshold advice; how a year-only birth date or a mother's name is used. The API reference, webhooks, rate
limits, a 99th-percentile response time, and batch file formats and size limits. Which cloud and region holds each sign-in region's data, and what "Local deployment
for data sovereignty" and "Licensing" mean. Single sign-on, encryption, service levels, backup, breach notification and penetration testing. Whether monitoring can
hold a payment, which scenarios ship, and whether it can file in goAML. [MC-FAQ] [MC-API] [MC-SUBPROC] [MC-COMPLIANCE] [MC-PARTNER] [MC-TM]

**The market.** Any Iraqi customer, partner or reseller beyond the one Iraqi firm that names Namescan as a partner (file 34; its own claim). What "13,000+" counts, and which
sectors and countries the 16 logos stand for. MemberCheck's prices, and NameScan's per-search amounts on its pricing page. The text of the reviews on the Capterra, G2 and
Software Advice pages themselves (six Capterra reviews are on the GetApp copy), including the "offline servers" complaint.
[NOTE-IRAQ] [MC-CASES] [NS-PRICING] [NOTE-MKT] [GA-ARCH]

**The owner's clue.** Where the owner heard "something in the lines of prototype member check", "its by some guy named mohammed" and "its a local iraqi system (i think im not
really sure)". Who the reviewer "mohammed habeeb" is, and where his firm is. The single most useful detail would be a screenshot of the page or document where the name
appeared, with the web address and logo visible. [NOTE-NAMES] [GA-ARCH] [LEAD-SEARCH]

**Questions the owner could put to the vendor** (this file's author contacted no one): who supplies your lists, and is the Iraqi national list loaded? How often are lists
refreshed, in hours? Does the engine match Arabic script natively, and how are mother's name and birth year used? Can the service run on a bank's own servers in Iraq,
and what does "Licensing" cover? Which Iraqi firms, if any, sell or white-label it? Can you show a test on Iraqi names with the results?

## 13. Sources

All pages were read on 2026-10-11 by the research agents unless a date is given. Vendor pages are the vendor's own words (marketing, not
audited). Items marked "(secondary)" are third-party pages. "(not read)" means the page refused our tools or was seen only in a search
result; it is listed because a key points to it. Search results are not addresses, so negative searches are cited through the working notes, or, for the
lead's own searches, through the entry LEAD-SEARCH below.

**Marks**
- [INFERENCE] Not a source. It marks our own reading of the evidence, not a stated fact.

**Registers and company records**
- [ABR-MC] Australian Business Register, MEMBER CHECK PTY LIMITED: https://abr.business.gov.au/ABN/View?abn=64129012344
- [ABR-NC] Australian Business Register, NEUROCOM PTY LTD: https://abr.business.gov.au/ABN/View?abn=12078580382
- [ABR-NX] Australian Business Register, NEXIANT AI PTY LTD: https://abr.business.gov.au/ABN/View?abn=90673693019
- [ASIC-MC] ASIC company search for ACN 129 012 344 (not read; HTTP 403): https://connectonline.asic.gov.au/RegistrySearch/faces/landing/panelSearch.jsfx?searchText=129012344&searchType=ACN
- [OC-MC] OpenCorporates record (secondary): https://opencorporates.com/companies/au/129012344
- [OAIC] Australian Information Commissioner site search for "membercheck", 0 results: https://www.oaic.gov.au/search?query=membercheck

**MemberCheck, the vendor's own pages**
- [MC-HOME] Home page: https://membercheck.com/
- [MC-ABOUT] About page: https://membercheck.com/about-us/
- [MC-PRIV] Privacy policy: https://membercheck.com/privacy-policy/
- [MC-SUBPROC] Sub-processor list: https://membercheck.com/sub-processors/
- [MC-PARTNER] Partnership page: https://membercheck.com/partnership/
- [MC-COMPLIANCE] Partner programme, "Compliance as a Business": https://membercheck.com/compliance-as-a-business/
- [MC-PRODUCTS] Products: https://membercheck.com/products/
- [MC-PEP] PEP and sanctions screening: https://membercheck.com/pep-sanction-screening/
- [MC-ID] Customer identity verification: https://membercheck.com/id-verification/
- [MC-TM] Transaction monitoring: https://membercheck.com/transaction-monitoring/
- [MC-ADVERSE] Adverse media: https://membercheck.com/adverse-media-check/
- [MC-KYB] Know your business: https://membercheck.com/knowyourbusiness/
- [MC-EDD] Enhanced due diligence: https://membercheck.com/enhanced-due-diligence/
- [MC-RISK] Customer and entity risk assessment: https://membercheck.com/product/aml-risk-assessment/
- [MC-JURIS] Jurisdiction risk: https://membercheck.com/jurisdiction-risk/
- [MC-API] API and integration guide: https://membercheck.com/api-integration/
- [MC-FEATURES] Features: https://membercheck.com/features-sets/
- [MC-FAQ] FAQ: https://membercheck.com/faq/
- [MC-COUNTRY] Country coverage guide: https://membercheck.com/country-coverage/
- [MC-IRAQ] Iraq page, "AML/CTF Compliance in Iraq" (updated 17 August 2026; re-read by the writer): https://membercheck.com/aml-cft-legislation-in-iraq/
- [MC-JAPAN] Japan page: https://membercheck.com/japan/
- [MC-CASES] Case studies and customer logos: https://membercheck.com/case-studies/
- [MC-TITAN] Titan Corporate Services case study: https://membercheck.com/case-studies/titan-corporate-services/
- [MC-PRICING] Pricing (demo form): https://membercheck.com/pricing/
- [MC-CMP] Comparison index: https://membercheck.com/compare/
- [MC-CMP-WC] Comparison with World-Check: https://membercheck.com/compare/membercheck-vs-world-check/
- [MC-CMP-CA] Comparison with ComplyAdvantage: https://membercheck.com/compare/membercheck-vs-complyadvantage/
- [MC-CMP-ENT] Enterprise AML platforms comparison (updated 27 August 2026; re-read by the writer): https://membercheck.com/compare/enterprise-aml-platforms/
- [MC-CAREERS] Careers: https://membercheck.com/careers/
- [MC-EVENTS] Events: https://membercheck.com/events/
- [MC-CONTACT] Contact page (read only; no form used): https://membercheck.com/contact-us/
- [MC-WAYBACK] Archived About page, request of 2019, footer dated 2021: https://web.archive.org/web/2019/https://www.membercheck.com/about-us/
- [API-DOCS] API docs host, HTTP 401 (not read): https://api.membercheck.com/docs
- [API-DEMO] Demo API docs host, HTTP 401 (not read): https://demo.api.membercheck.com/docs
- [MC-DOCS] Docs host, HTTP 403 (not read): https://docs.membercheck.com/
- [MC-APIPATH] API path, HTTP 403 (not read): https://membercheck.com/api
- [FS-LOGIN] Monitoring platform sign-up link on the monitoring page, form not opened: https://eu.fraudshield.nexiant.ai/register?src=membercheck

**Neurocom group, GPayments and Nexiant, their own pages**
- [NC-HOME] Neurocom group site: https://neurocom.com.au/
- [NC-WAYBACK] Neurocom group site, copy of 11 August 2022: https://web.archive.org/web/20220811030531/https://neurocom.com.au/
- [GP-HOME] GPayments home: https://www.gpayments.com/
- [GP-ABOUT] GPayments about: https://www.gpayments.com/about/
- [NX-ABOUT] Nexiant about: https://nexiant.ai/about/
- [NX-CASES] Nexiant case-study index: https://nexiant.ai/resources/case-studies/
- [NX-AEGIS] Aegis Reins (NameScan) case: https://nexiant.ai/resources/case-studies/namescan/aegis-reins/
- [NX-UPAY] UPayCard (NameScan) case: https://nexiant.ai/resources/case-studies/namescan/upaycard/
- [NX-EGT] EGT Digital (GPayments) case: https://nexiant.ai/resources/case-studies/gpayments/egt-digital/
- [NX-BANKING] Nexiant banking use case: https://nexiant.ai/resources/use-case/banking/

**NameScan, the sister brand's own pages**
- [NS-HOME] Home page: https://namescan.io/
- [NS-ABOUT] About page (re-read by the writer): https://namescan.io/about
- [NS-FAQ] FAQ: https://namescan.io/insights/faq/
- [NS-PRICING] Pricing: https://namescan.io/pricing
- [NS-API] Developer Centre: https://namescan.io/pepandsanctionapi
- [NS-PARTNERSHIP] Partnership page (read by the lead on 2026-10-11; also by the verifier): https://namescan.io/partnership
- [NS-APIDOC] API reference, HTTP 401 (not read): https://api.namescan.io/doc/

**Directories, press and review sites (all secondary)**
- [CBI-MC] CB Insights profile: https://www.cbinsights.com/company/membercheck/
- [CBI-PEOPLE] CB Insights people: https://www.cbinsights.com/company/membercheck/people
- [LI-MC] LinkedIn company page: https://www.linkedin.com/company/membercheck/
- [ZH-MC] Zoftware Hub product details: https://zoftwarehub.com/en-ae/products/membercheck/product-details
- [ZH-OV] Zoftware Hub overview: https://zoftwarehub.com/en-ae/products/membercheck/overview
- [GR-PRESS] Goodreturns press item, 6 November 2012: https://test.goodreturns.co.nz/article/new-software-to-weed-out-dodgy-clients (same article: https://www.goodreturns.co.nz/article/976500223/new-software-to-weed-out-dodgy-clients.html)
- [GA-ARCH] GetApp, archive copy captured 12 June 2026: https://web.archive.org/web/2026/https://www.getapp.com/finance-accounting-software/a/membercheck/
- [CAPTERRA-MC] Capterra MemberCheck reviews (not read; HTTP 403): https://www.capterra.com/reviews/154148/MemberCheck
- [KYCKR] Kyckr, about: https://kyckr.com/about-us
- [LSEG-IQ] LSEG press release, 10 March 2020, World-Check and Iraqna IBTS: https://www.lseg.com/en/media-centre/press-releases/refinitiv/2020/march/refinitiv-partners-with-iraqna-ibts-to-enhance-regulatory-compliance-efficiency-in-iraq
- [FINTECH] Fintech Global, 25 November 2021, UR Islamic Bank and Capital Banking Solutions: https://fintech.global/?p=66393

**Project files (local)**
- [F14] Vendors: screening engines, section 1 (the twelve-need yardstick (eight rated in file 35)): /home/haithem/Projects/AKR-TMS/research/14-vendor-screening-engines.md
- [F30] Security and privacy research; its data-classification conclusion is quoted as given in the project brief, the file itself was not opened: /home/haithem/Projects/AKR-TMS/research/30-security-privacy.md
- [NOTE-CO] Working notes 01, company: /home/haithem/Projects/AKR-TMS/research/_membercheck/01-company.md
- [NOTE-DATA] Working notes 03, data sources: /home/haithem/Projects/AKR-TMS/research/_membercheck/03-data-sources.md
- [NOTE-MKT] Working notes 05, market and pricing: /home/haithem/Projects/AKR-TMS/research/_membercheck/05-market-pricing.md
- [NOTE-IRAQ] Working notes 06, Iraq and the Middle East: /home/haithem/Projects/AKR-TMS/research/_membercheck/06-iraq-mena.md
- [NOTE-NAMES] Working notes 07, name resolution: /home/haithem/Projects/AKR-TMS/research/_membercheck/07-name-resolution.md
- [NOTE-P12] Working notes 12, independent evidence at Iraqi banks and NameScan's pages (lines 65 and 137 describe the NameScan Emerald coverage document, https://docs.membercheck.com/docs/NameScan_Emerald_Countries_and_Watchlists_List.pdf?v=2024_1): /home/haithem/Projects/AKR-TMS/research/_membercheck/12-prototype-third-party.md

**Searches by the lead (no page address)**
- [LEAD-SEARCH] Four web searches run by the lead on 2026-10-11 about the reviewer named in section 1.3: the name with Prototype's English name and Iraq; the name with CEO, Founder, Prototype, Baghdad and fintech terms (the search tool reported that it dropped the name, so this one is weak); the name with MemberCheck and distributor; the Arabic form of the name with the Arabic title of Prototype's Facebook page. None found a page that ties the reviewer to Prototype, to Iraq or to MemberCheck. The name is common, so a search cannot settle it. Nobody was contacted.

**Fetches made by the writer on 2026-10-11 (5 of 5 allowed; each returns a model-written summary of the page, so quotes come from that summary)**
1. NS-ABOUT, to settle whether Acuris is a Neurocom group member: it is listed under "Our Partners", separate from the group.
2. MC-IRAQ, to settle how many items the Iraq coverage block has: five, including "Sanctions & PEP screening"; no Iraqi list; no Arabic.
3. MC-PEP, keyword quotes: "Arabic" appears once, in an FAQ answer on transliteration; "Iraq" does not appear.
4. MC-PEP, second read: "<24h" under "Average integration time", "99.98%" under "System uptime", "<1s" under "Average screening response", and "a sub-second response".
5. MC-CMP-ENT, to settle the Arabic claim: interface "English, French, Arabic, Chinese, Japanese and Spanish" and matching "in original scripts".
